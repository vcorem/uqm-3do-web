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

// Rust translation of src/libs/graphics/sdl/canvas.c.
extern "C" {
    pub type SDL_BlitMap;
    pub type uio_DirHandle;
    fn memcpy(
        _: *mut ::core::ffi::c_void,
        _: *const ::core::ffi::c_void,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn exit(_: ::core::ffi::c_int) -> !;
    fn SDL_SetError(fmt: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn SDL_GetError() -> *const ::core::ffi::c_char;
    fn SDL_MapRGBA(
        format: *const SDL_PixelFormat,
        r: Uint8,
        g: Uint8,
        b: Uint8,
        a: Uint8,
    ) -> Uint32;
    fn SDL_GetRGB(
        pixel: Uint32,
        format: *const SDL_PixelFormat,
        r: *mut Uint8,
        g: *mut Uint8,
        b: *mut Uint8,
    );
    fn SDL_GetRGBA(
        pixel: Uint32,
        format: *const SDL_PixelFormat,
        r: *mut Uint8,
        g: *mut Uint8,
        b: *mut Uint8,
        a: *mut Uint8,
    );
    fn SDL_CreateRGBSurface(
        flags: Uint32,
        width: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
        depth: ::core::ffi::c_int,
        Rmask: Uint32,
        Gmask: Uint32,
        Bmask: Uint32,
        Amask: Uint32,
    ) -> *mut SDL_Surface;
    fn SDL_FreeSurface(surface: *mut SDL_Surface);
    fn SDL_LockSurface(surface: *mut SDL_Surface) -> ::core::ffi::c_int;
    fn SDL_UnlockSurface(surface: *mut SDL_Surface);
    fn SDL_SetClipRect(surface: *mut SDL_Surface, rect: *const SDL_Rect) -> SDL_bool;
    fn SDL_FillRect(
        dst: *mut SDL_Surface,
        rect: *const SDL_Rect,
        color: Uint32,
    ) -> ::core::ffi::c_int;
    fn SDL_UpperBlit(
        src: *mut SDL_Surface,
        srcrect: *const SDL_Rect,
        dst: *mut SDL_Surface,
        dstrect: *mut SDL_Rect,
    ) -> ::core::ffi::c_int;
    fn HCalloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn __assert_fail(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    );
    fn TFB_DrawImage_FixScaling(
        image: *mut TFB_Image,
        target: ::core::ffi::c_int,
        r#type: ::core::ffi::c_int,
    );
    fn LockMutex(sem: Mutex);
    fn UnlockMutex(sem: Mutex);
    fn sdluio_loadImage(
        dir: *mut uio_DirHandle,
        fileName: *const ::core::ffi::c_char,
    ) -> *mut SDL_Surface;
    fn rotateSurface(
        src: *mut SDL_Surface,
        dst: *mut SDL_Surface,
        angle: ::core::ffi::c_double,
        smooth: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn rotozoomSurfaceSize(
        width: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
        angle: ::core::ffi::c_double,
        zoom: ::core::ffi::c_double,
        dstwidth: *mut ::core::ffi::c_int,
        dstheight: *mut ::core::ffi::c_int,
    );
    fn log_add(_: log_Level, fmt: *const ::core::ffi::c_char, ...);
    static mut SDL_Screen: *mut SDL_Surface;
    static mut format_conv_surf: *mut SDL_Surface;
    fn TFB_DisplayFormatAlpha(surface: *mut SDL_Surface) -> *mut SDL_Surface;
    fn TFB_HasSurfaceAlphaMod(surface: *mut SDL_Surface) -> ::core::ffi::c_int;
    fn TFB_SetSurfaceAlphaMod(
        surface: *mut SDL_Surface,
        alpha: Uint8,
    ) -> ::core::ffi::c_int;
    fn TFB_DisableSurfaceAlphaMod(surface: *mut SDL_Surface) -> ::core::ffi::c_int;
    fn TFB_GetColorKey(
        surface: *mut SDL_Surface,
        key: *mut Uint32,
    ) -> ::core::ffi::c_int;
    fn TFB_SetColorKey(
        surface: *mut SDL_Surface,
        key: Uint32,
        rleaccel: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn TFB_DisableColorKey(surface: *mut SDL_Surface) -> ::core::ffi::c_int;
    fn TFB_SetColors(
        surface: *mut SDL_Surface,
        colors: *mut SDL_Color,
        firstcolor: ::core::ffi::c_int,
        ncolors: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn TFB_ReturnColorMap(map: *mut TFB_ColorMap);
    fn getpixel_for(surface: *mut SDL_Surface) -> GetPixelFn;
    fn putpixel_for(surface: *mut SDL_Surface) -> PutPixelFn;
    fn renderpixel_for(surface: *mut SDL_Surface, _: RenderKind) -> RenderPixelFn;
    fn line_prim(
        x1: ::core::ffi::c_int,
        y1: ::core::ffi::c_int,
        x2: ::core::ffi::c_int,
        y2: ::core::ffi::c_int,
        color: Uint32,
        plot: RenderPixelFn,
        factor: ::core::ffi::c_int,
        dst: *mut SDL_Surface,
    );
    fn fillrect_prim(
        r: SDL_Rect,
        color: Uint32,
        plot: RenderPixelFn,
        factor: ::core::ffi::c_int,
        dst: *mut SDL_Surface,
    );
    fn blt_prim(
        src: *mut SDL_Surface,
        src_r: SDL_Rect,
        plot: RenderPixelFn,
        factor: ::core::ffi::c_int,
        dst: *mut SDL_Surface,
        dst_r: SDL_Rect,
    );
}
pub type size_t = usize;
pub type uint8_t = ::core::ffi::c_uchar;
pub type uint32_t = ::core::ffi::c_uint;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct SDL_bool(pub ::core::ffi::c_uint);
impl SDL_bool {
    pub const SDL_FALSE: Self = Self(0);
    pub const SDL_TRUE: Self = Self(1);
}
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
pub type uint8 = ::core::ffi::c_uchar;
pub type sint16 = ::core::ffi::c_short;
pub type uint32 = ::core::ffi::c_uint;
pub type sint32 = ::core::ffi::c_int;
pub type BYTE = uint8;
pub type SWORD = sint16;
pub type DWORD = uint32;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct BOOLEAN(pub ::core::ffi::c_uint);
impl BOOLEAN {
    pub const FALSE: Self = Self(0);
    pub const TRUE: Self = Self(1);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Color {
    pub r: BYTE,
    pub g: BYTE,
    pub b: BYTE,
    pub a: BYTE,
}
pub type TFB_Image = tfb_image;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tfb_image {
    pub NormalImg: TFB_Canvas,
    pub ScaledImg: TFB_Canvas,
    pub MipmapImg: TFB_Canvas,
    pub FilledImg: TFB_Canvas,
    pub colormap_index: ::core::ffi::c_int,
    pub colormap_version: ::core::ffi::c_int,
    pub NormalHs: HOT_SPOT,
    pub MipmapHs: HOT_SPOT,
    pub last_scale_hs: HOT_SPOT,
    pub last_scale: ::core::ffi::c_int,
    pub last_scale_type: ::core::ffi::c_int,
    pub last_fill: Color,
    pub extent: EXTENT,
    pub mutex: Mutex,
    pub dirty: BOOLEAN,
}
pub type Mutex = *mut ::core::ffi::c_void;
pub type EXTENT = extent;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct extent {
    pub width: COORD,
    pub height: COORD,
}
pub type COORD = SWORD;
pub type HOT_SPOT = POINT;
pub type POINT = point;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct point {
    pub x: COORD,
    pub y: COORD,
}
pub type TFB_Canvas = *mut ::core::ffi::c_void;
pub type RECT = rect;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rect {
    pub corner: POINT,
    pub extent: EXTENT,
}
pub type TFB_Char = tfb_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tfb_char {
    pub extent: EXTENT,
    pub disp: EXTENT,
    pub HotSpot: HOT_SPOT,
    pub data: *mut BYTE,
    pub pitch: DWORD,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DrawMode {
    pub kind: BYTE,
    pub factor: SWORD,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed {
    pub const DRAW_REPLACE: Self = Self(0);
    pub const DRAW_ADDITIVE: Self = Self(1);
    pub const DRAW_ALPHA: Self = Self(2);
    pub const DRAW_DEFAULT: Self = Self(0);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_0 {
    pub const TFB_SCALE_STEP: Self = Self(0);
    pub const TFB_SCALE_NEAREST: Self = Self(1);
    pub const TFB_SCALE_BILINEAR: Self = Self(2);
    pub const TFB_SCALE_TRILINEAR: Self = Self(3);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NativePalette {
    pub colors: [SDL_Color; 256],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tfb_colormap {
    pub index: ::core::ffi::c_int,
    pub version: ::core::ffi::c_int,
    pub refcount: ::core::ffi::c_int,
    pub next: *mut tfb_colormap,
    pub palette: *mut NativePalette,
}
pub type TFB_ColorMap = tfb_colormap;
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
pub type TFB_PixelFormat = tfb_pixelformat;
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
pub struct C2Rust_Unnamed_1 {
    pub r: Uint8,
    pub g: Uint8,
    pub b: Uint8,
    pub a: Uint8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pixel_t {
    pub value: Uint32,
    pub chan: [Uint8; 4],
    pub c: C2Rust_Unnamed_1,
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
pub struct RenderKind(pub ::core::ffi::c_uint);
impl RenderKind {
    pub const renderReplace: Self = Self(0);
    pub const renderAdditive: Self = Self(1);
    pub const renderAlpha: Self = Self(2);
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
#[inline]
unsafe extern "C" fn sameColor(mut c1: Color, mut c2: Color) -> bool {
    return c1.r as ::core::ffi::c_int == c2.r as ::core::ffi::c_int
        && c1.g as ::core::ffi::c_int == c2.g as ::core::ffi::c_int
        && c1.b as ::core::ffi::c_int == c2.b as ::core::ffi::c_int
        && c1.a as ::core::ffi::c_int == c2.a as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn NativeToColor(mut native: SDL_Color) -> Color {
    let mut color: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    color.r = native.r as BYTE;
    color.g = native.g as BYTE;
    color.b = native.b as BYTE;
    color.a = 0xff as BYTE;
    return color;
}
#[inline]
unsafe extern "C" fn ColorToNative(mut color: Color) -> SDL_Color {
    let mut native: SDL_Color = SDL_Color {
        r: 0,
        g: 0,
        b: 0,
        a: 0,
    };
    native.r = color.r as Uint8;
    native.g = color.g as Uint8;
    native.b = color.b as Uint8;
    native.a = color.a as Uint8;
    return native;
}
static mut btable: [[Uint8; 256]; 256] = [[0; 256]; 256];
#[no_mangle]
pub unsafe extern "C" fn TFB_DrawCanvas_Initialize() {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < 256 as ::core::ffi::c_int {
        j = 0 as ::core::ffi::c_int;
        while j < 256 as ::core::ffi::c_int {
            btable[j as usize][i as usize] = (j * i + 0x80 as ::core::ffi::c_int
                >> 8 as ::core::ffi::c_int) as Uint8;
            j += 1;
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn TFB_DrawCanvas_GetError() -> *const ::core::ffi::c_char {
    let mut err: *const ::core::ffi::c_char = SDL_GetError();
    return err;
}
unsafe extern "C" fn checkPrimitiveMode(
    mut surf: *mut SDL_Surface,
    mut color: *mut Color,
    mut mode: *mut DrawMode,
) {
    let mut fmt: *const SDL_PixelFormat = (*surf).format;
    if (*mode).kind as ::core::ffi::c_int
        == C2Rust_Unnamed::DRAW_REPLACE.0 as ::core::ffi::c_int
        && (*fmt).Amask == 0 as Uint32
        && (*color).a as ::core::ffi::c_int != 0xff as ::core::ffi::c_int
    {
        (*mode).kind = C2Rust_Unnamed::DRAW_ALPHA.0 as ::core::ffi::c_int as BYTE;
        (*mode).factor = (*color).a as SWORD;
        (*color).a = 0xff as BYTE;
    }
}
#[no_mangle]
pub unsafe extern "C" fn TFB_DrawCanvas_Line(
    mut x1: ::core::ffi::c_int,
    mut y1: ::core::ffi::c_int,
    mut x2: ::core::ffi::c_int,
    mut y2: ::core::ffi::c_int,
    mut color: Color,
    mut mode: DrawMode,
    mut target: TFB_Canvas,
) {
    let mut dst: *mut SDL_Surface = target as *mut SDL_Surface;
    let mut fmt: *mut SDL_PixelFormat = (*dst).format;
    let mut sdlColor: Uint32 = 0;
    let mut plotFn: RenderPixelFn = None;
    checkPrimitiveMode(dst, &raw mut color, &raw mut mode);
    sdlColor = SDL_MapRGBA(fmt, color.r, color.g, color.b, color.a);
    plotFn = renderpixel_for(
        target as *mut SDL_Surface,
        RenderKind(mode.kind as ::core::ffi::c_uint),
    );
    if plotFn.is_none() {
        log_add(
            log_Level::log_Warning,
            b"ERROR: TFB_DrawCanvas_Line unsupported draw mode (%d)\0".as_ptr()
                as *const ::core::ffi::c_char,
            mode.kind as ::core::ffi::c_int,
        );
        return;
    }
    SDL_LockSurface(dst);
    line_prim(x1, y1, x2, y2, sdlColor, plotFn, mode.factor as ::core::ffi::c_int, dst);
    SDL_UnlockSurface(dst);
}
#[no_mangle]
pub unsafe extern "C" fn TFB_DrawCanvas_Rect(
    mut rect: *mut RECT,
    mut color: Color,
    mut mode: DrawMode,
    mut target: TFB_Canvas,
) {
    let mut dst: *mut SDL_Surface = target as *mut SDL_Surface;
    let mut fmt: *mut SDL_PixelFormat = (*dst).format;
    let mut sdlColor: Uint32 = 0;
    let mut sr: SDL_Rect = SDL_Rect { x: 0, y: 0, w: 0, h: 0 };
    sr.x = (*rect).corner.x as ::core::ffi::c_int;
    sr.y = (*rect).corner.y as ::core::ffi::c_int;
    sr.w = (*rect).extent.width as ::core::ffi::c_int;
    sr.h = (*rect).extent.height as ::core::ffi::c_int;
    checkPrimitiveMode(dst, &raw mut color, &raw mut mode);
    sdlColor = SDL_MapRGBA(fmt, color.r, color.g, color.b, color.a);
    if mode.kind as ::core::ffi::c_int
        == C2Rust_Unnamed::DRAW_REPLACE.0 as ::core::ffi::c_int
    {
        let mut colorkey: Uint32 = 0;
        if (*fmt).Amask != 0
            && TFB_GetColorKey(dst, &raw mut colorkey) == 0 as ::core::ffi::c_int
        {
            if sdlColor & !(*fmt).Amask == colorkey & !(*fmt).Amask {
                sdlColor &= !(*fmt).Amask;
            }
        }
        SDL_FillRect(dst, &raw mut sr, sdlColor);
    } else {
        let mut plotFn: RenderPixelFn = renderpixel_for(
            target as *mut SDL_Surface,
            RenderKind(mode.kind as ::core::ffi::c_uint),
        );
        if plotFn.is_none() {
            log_add(
                log_Level::log_Warning,
                b"ERROR: TFB_DrawCanvas_Rect unsupported draw mode (%d)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                mode.kind as ::core::ffi::c_int,
            );
            return;
        }
        SDL_LockSurface(dst);
        fillrect_prim(sr, sdlColor, plotFn, mode.factor as ::core::ffi::c_int, dst);
        SDL_UnlockSurface(dst);
    };
}
unsafe extern "C" fn TFB_DrawCanvas_Blit(
    mut src: *mut SDL_Surface,
    mut src_r: *mut SDL_Rect,
    mut dst: *mut SDL_Surface,
    mut dst_r: *mut SDL_Rect,
    mut mode: DrawMode,
) {
    let mut srcfmt: *mut SDL_PixelFormat = (*src).format;
    if mode.kind as ::core::ffi::c_int
        == C2Rust_Unnamed::DRAW_REPLACE.0 as ::core::ffi::c_int
    {
        SDL_UpperBlit(src, src_r, dst, dst_r);
    } else if mode.kind as ::core::ffi::c_int
        == C2Rust_Unnamed::DRAW_ALPHA.0 as ::core::ffi::c_int
        && (*srcfmt).Amask == 0 as Uint32
    {
        let mut hasAlpha: ::core::ffi::c_int = TFB_HasSurfaceAlphaMod(src);
        (hasAlpha == 0
            || {
                __assert_fail(
                    b"!hasAlpha\0".as_ptr() as *const ::core::ffi::c_char,
                    b"src/libs/graphics/sdl/canvas.c\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    155 as ::core::ffi::c_int,
                    b"TFB_DrawCanvas_Blit\0".as_ptr() as *const ::core::ffi::c_char,
                );
                0 as ::core::ffi::c_int != 0
            }) as ::core::ffi::c_int;
        TFB_SetSurfaceAlphaMod(src, mode.factor as Uint8);
        SDL_UpperBlit(src, src_r, dst, dst_r);
        TFB_DisableSurfaceAlphaMod(src);
    } else {
        let mut loc_src_r: SDL_Rect = SDL_Rect { x: 0, y: 0, w: 0, h: 0 };
        let mut loc_dst_r: SDL_Rect = SDL_Rect { x: 0, y: 0, w: 0, h: 0 };
        let mut plotFn: RenderPixelFn = renderpixel_for(
            dst,
            RenderKind(mode.kind as ::core::ffi::c_uint),
        );
        if plotFn.is_none() {
            log_add(
                log_Level::log_Warning,
                b"ERROR: TFB_DrawCanvas_Blit unsupported draw mode (%d)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                mode.kind as ::core::ffi::c_int,
            );
            return;
        }
        if src_r.is_null() {
            loc_src_r.x = 0 as ::core::ffi::c_int;
            loc_src_r.y = 0 as ::core::ffi::c_int;
            loc_src_r.w = (*src).w;
            loc_src_r.h = (*src).h;
            src_r = &raw mut loc_src_r;
        }
        if dst_r.is_null() {
            loc_dst_r.x = 0 as ::core::ffi::c_int;
            loc_dst_r.y = 0 as ::core::ffi::c_int;
            loc_dst_r.w = (*dst).w;
            loc_dst_r.h = (*dst).h;
            dst_r = &raw mut loc_dst_r;
        }
        SDL_LockSurface(dst);
        blt_prim(src, *src_r, plotFn, mode.factor as ::core::ffi::c_int, dst, *dst_r);
        SDL_UnlockSurface(dst);
    };
}
#[no_mangle]
pub unsafe extern "C" fn TFB_DrawCanvas_Image(
    mut img: *mut TFB_Image,
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
    mut scale: ::core::ffi::c_int,
    mut scaleMode: ::core::ffi::c_int,
    mut cmap: *mut TFB_ColorMap,
    mut mode: DrawMode,
    mut target: TFB_Canvas,
) {
    let mut srcRect: SDL_Rect = SDL_Rect { x: 0, y: 0, w: 0, h: 0 };
    let mut targetRect: SDL_Rect = SDL_Rect { x: 0, y: 0, w: 0, h: 0 };
    let mut pSrcRect: *mut SDL_Rect = ::core::ptr::null_mut::<SDL_Rect>();
    let mut surf: *mut SDL_Surface = ::core::ptr::null_mut::<SDL_Surface>();
    let mut NormalPal: *mut SDL_Palette = ::core::ptr::null_mut::<SDL_Palette>();
    if img.is_null() {
        log_add(
            log_Level::log_Warning,
            b"ERROR: TFB_DrawCanvas_Image passed null image ptr\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    LockMutex((*img).mutex);
    NormalPal = (*(*((*img).NormalImg as *mut SDL_Surface)).format).palette;
    if !NormalPal.is_null() && !cmap.is_null()
        && (*img).colormap_version != (*cmap).version
    {
        TFB_SetColors(
            (*img).NormalImg as *mut SDL_Surface,
            &raw mut (*(*cmap).palette).colors as *mut SDL_Color,
            0 as ::core::ffi::c_int,
            256 as ::core::ffi::c_int,
        );
    }
    if scale != 0 as ::core::ffi::c_int && scale != 256 as ::core::ffi::c_int {
        if scaleMode == C2Rust_Unnamed_0::TFB_SCALE_TRILINEAR.0 as ::core::ffi::c_int
            && !(*img).MipmapImg.is_null()
        {
            if TFB_DrawCanvas_IsPaletted((*img).MipmapImg).0 != 0 && !cmap.is_null()
                && (*img).colormap_version != (*cmap).version
            {
                TFB_SetColors(
                    (*img).MipmapImg as *mut SDL_Surface,
                    &raw mut (*(*cmap).palette).colors as *mut SDL_Color,
                    0 as ::core::ffi::c_int,
                    256 as ::core::ffi::c_int,
                );
            }
        } else if scaleMode
            == C2Rust_Unnamed_0::TFB_SCALE_TRILINEAR.0 as ::core::ffi::c_int
            && (*img).MipmapImg.is_null()
        {
            scaleMode = C2Rust_Unnamed_0::TFB_SCALE_BILINEAR.0 as ::core::ffi::c_int;
        }
        TFB_DrawImage_FixScaling(img, scale, scaleMode);
        surf = (*img).ScaledImg as *mut SDL_Surface;
        if TFB_DrawCanvas_IsPaletted(surf as TFB_Canvas).0 != 0 {
            (!NormalPal.is_null() && !(*NormalPal).colors.is_null()
                || {
                    __assert_fail(
                        b"NormalPal && NormalPal->colors\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"src/libs/graphics/sdl/canvas.c\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        240 as ::core::ffi::c_int,
                        b"TFB_DrawCanvas_Image\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    0 as ::core::ffi::c_int != 0
                }) as ::core::ffi::c_int;
            TFB_SetColors(
                surf,
                (*NormalPal).colors,
                0 as ::core::ffi::c_int,
                (*NormalPal).ncolors,
            );
        }
        srcRect.x = 0 as ::core::ffi::c_int;
        srcRect.y = 0 as ::core::ffi::c_int;
        srcRect.w = (*img).extent.width as ::core::ffi::c_int;
        srcRect.h = (*img).extent.height as ::core::ffi::c_int;
        pSrcRect = &raw mut srcRect;
        targetRect.x = x - (*img).last_scale_hs.x as ::core::ffi::c_int;
        targetRect.y = y - (*img).last_scale_hs.y as ::core::ffi::c_int;
    } else {
        surf = (*img).NormalImg as *mut SDL_Surface;
        pSrcRect = ::core::ptr::null_mut::<SDL_Rect>();
        targetRect.x = x - (*img).NormalHs.x as ::core::ffi::c_int;
        targetRect.y = y - (*img).NormalHs.y as ::core::ffi::c_int;
    }
    if !cmap.is_null() {
        (*img).colormap_version = (*cmap).version;
        TFB_ReturnColorMap(cmap);
    }
    TFB_DrawCanvas_Blit(
        surf,
        pSrcRect,
        target as *mut SDL_Surface,
        &raw mut targetRect,
        mode,
    );
    UnlockMutex((*img).mutex);
}
unsafe extern "C" fn TFB_DrawCanvas_Fill(
    mut src: *mut SDL_Surface,
    mut fillcolor: Uint32,
    mut dst: *mut SDL_Surface,
) {
    let mut srcfmt: *const SDL_PixelFormat = (*src).format;
    let mut dstfmt: *mut SDL_PixelFormat = (*dst).format;
    let width: ::core::ffi::c_int = (*src).w;
    let height: ::core::ffi::c_int = (*src).h;
    let bpp: ::core::ffi::c_int = (*dstfmt).BytesPerPixel as ::core::ffi::c_int;
    let sp: ::core::ffi::c_int = (*src).pitch;
    let dp: ::core::ffi::c_int = (*dst).pitch;
    let slen: ::core::ffi::c_int = sp / bpp;
    let dlen: ::core::ffi::c_int = dp / bpp;
    let dsrc: ::core::ffi::c_int = slen - width;
    let ddst: ::core::ffi::c_int = dlen - width;
    let mut src_p: *mut Uint32 = ::core::ptr::null_mut::<Uint32>();
    let mut dst_p: *mut Uint32 = ::core::ptr::null_mut::<Uint32>();
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut srckey: Uint32 = 0 as Uint32;
    let mut dstkey: Uint32 = 0 as Uint32;
    let mut amask: Uint32 = (*srcfmt).Amask;
    let mut alpha: ::core::ffi::c_int = ((fillcolor & amask)
        >> (*srcfmt).Ashift as ::core::ffi::c_int) as ::core::ffi::c_int;
    if (*srcfmt).BytesPerPixel as ::core::ffi::c_int != 4 as ::core::ffi::c_int
        || (*dstfmt).BytesPerPixel as ::core::ffi::c_int != 4 as ::core::ffi::c_int
    {
        log_add(
            log_Level::log_Warning,
            b"TFB_DrawCanvas_Fill: Unsupported surface formats: %d bytes/pixel source, %d bytes/pixel destination\0"
                .as_ptr() as *const ::core::ffi::c_char,
            (*srcfmt).BytesPerPixel as ::core::ffi::c_int,
            (*dstfmt).BytesPerPixel as ::core::ffi::c_int,
        );
        return;
    }
    fillcolor &= !amask;
    SDL_LockSurface(src);
    SDL_LockSurface(dst);
    src_p = (*src).pixels as *mut Uint32;
    dst_p = (*dst).pixels as *mut Uint32;
    if dstkey == fillcolor {
        dstkey = SDL_MapRGBA(
            dstfmt,
            127 as Uint8,
            127 as Uint8,
            127 as Uint8,
            0 as Uint8,
        );
    }
    if (*srcfmt).Amask != 0 {
        y = 0 as ::core::ffi::c_int;
        while y < height {
            x = 0 as ::core::ffi::c_int;
            while x < width {
                let mut p: Uint32 = *src_p & amask;
                if p == 0 as Uint32 {
                    *dst_p = dstkey;
                } else if alpha == 0xff as ::core::ffi::c_int {
                    *dst_p = p | fillcolor;
                } else {
                    p >>= (*srcfmt).Ashift as ::core::ffi::c_int;
                    p = p.wrapping_mul(alpha as Uint32) >> 8 as ::core::ffi::c_int;
                    p <<= (*srcfmt).Ashift as ::core::ffi::c_int;
                    *dst_p = p | fillcolor;
                }
                x += 1;
                src_p = src_p.offset(1);
                dst_p = dst_p.offset(1);
            }
            y += 1;
            dst_p = dst_p.offset(ddst as isize);
            src_p = src_p.offset(dsrc as isize);
        }
    } else if TFB_GetColorKey(src, &raw mut srckey) == 0 as ::core::ffi::c_int {
        y = 0 as ::core::ffi::c_int;
        while y < height {
            x = 0 as ::core::ffi::c_int;
            while x < width {
                let mut p_0: Uint32 = *src_p;
                *dst_p = if p_0 == srckey { dstkey } else { fillcolor };
                x += 1;
                src_p = src_p.offset(1);
                dst_p = dst_p.offset(1);
            }
            y += 1;
            dst_p = dst_p.offset(ddst as isize);
            src_p = src_p.offset(dsrc as isize);
        }
    } else {
        log_add(
            log_Level::log_Warning,
            b"TFB_DrawCanvas_Fill: Unsupported sourcesurface format\n\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    SDL_UnlockSurface(dst);
    SDL_UnlockSurface(src);
    TFB_SetColorKey(dst, dstkey, 0 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn TFB_DrawCanvas_FilledImage(
    mut img: *mut TFB_Image,
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
    mut scale: ::core::ffi::c_int,
    mut scaleMode: ::core::ffi::c_int,
    mut color: Color,
    mut mode: DrawMode,
    mut target: TFB_Canvas,
) {
    let mut dst: *mut SDL_Surface = target as *mut SDL_Surface;
    let mut srcRect: SDL_Rect = SDL_Rect { x: 0, y: 0, w: 0, h: 0 };
    let mut targetRect: SDL_Rect = SDL_Rect { x: 0, y: 0, w: 0, h: 0 };
    let mut pSrcRect: *mut SDL_Rect = ::core::ptr::null_mut::<SDL_Rect>();
    let mut surf: *mut SDL_Surface = ::core::ptr::null_mut::<SDL_Surface>();
    let mut palette: *mut SDL_Palette = ::core::ptr::null_mut::<SDL_Palette>();
    let mut i: ::core::ffi::c_int = 0;
    let mut force_fill: bool = false;
    if img.is_null() {
        log_add(
            log_Level::log_Warning,
            b"ERROR: TFB_DrawCanvas_FilledImage passed null image ptr\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    checkPrimitiveMode(dst, &raw mut color, &raw mut mode);
    LockMutex((*img).mutex);
    if scale != 0 as ::core::ffi::c_int && scale != 256 as ::core::ffi::c_int {
        if scaleMode == C2Rust_Unnamed_0::TFB_SCALE_TRILINEAR.0 as ::core::ffi::c_int {
            scaleMode = C2Rust_Unnamed_0::TFB_SCALE_BILINEAR.0 as ::core::ffi::c_int;
        }
        if scale != (*img).last_scale || scaleMode != (*img).last_scale_type {
            force_fill = true;
        }
        TFB_DrawImage_FixScaling(img, scale, scaleMode);
        surf = (*img).ScaledImg as *mut SDL_Surface;
        srcRect.x = 0 as ::core::ffi::c_int;
        srcRect.y = 0 as ::core::ffi::c_int;
        srcRect.w = (*img).extent.width as ::core::ffi::c_int;
        srcRect.h = (*img).extent.height as ::core::ffi::c_int;
        pSrcRect = &raw mut srcRect;
        targetRect.x = x - (*img).last_scale_hs.x as ::core::ffi::c_int;
        targetRect.y = y - (*img).last_scale_hs.y as ::core::ffi::c_int;
    } else {
        if (*img).last_scale != 0 as ::core::ffi::c_int {
            force_fill = true;
            (*img).last_scale = 0 as ::core::ffi::c_int;
        }
        surf = (*img).NormalImg as *mut SDL_Surface;
        pSrcRect = ::core::ptr::null_mut::<SDL_Rect>();
        targetRect.x = x - (*img).NormalHs.x as ::core::ffi::c_int;
        targetRect.y = y - (*img).NormalHs.y as ::core::ffi::c_int;
    }
    palette = (*(*surf).format).palette;
    if !palette.is_null() {
        let mut colors: [SDL_Color; 256] = [SDL_Color {
            r: 0,
            g: 0,
            b: 0,
            a: 0,
        }; 256];
        colors[0usize] = ColorToNative(color);
        i = 1 as ::core::ffi::c_int;
        while i < (*palette).ncolors {
            colors[i as usize] = colors[0usize];
            i += 1;
        }
        TFB_SetColors(
            surf,
            &raw mut colors as *mut SDL_Color,
            0 as ::core::ffi::c_int,
            (*palette).ncolors,
        );
        (*img).colormap_version -= 1;
    } else {
        let mut newfill: *mut SDL_Surface = (*img).FilledImg as *mut SDL_Surface;
        let mut fillfmt: *mut SDL_PixelFormat = ::core::ptr::null_mut::<
            SDL_PixelFormat,
        >();
        if !newfill.is_null() && ((*newfill).w < (*surf).w || (*newfill).h < (*surf).h) {
            TFB_DrawCanvas_Delete(newfill as TFB_Canvas);
            newfill = ::core::ptr::null_mut::<SDL_Surface>();
        }
        if newfill.is_null() {
            newfill = SDL_CreateRGBSurface(
                0 as Uint32,
                (*surf).w,
                (*surf).h,
                (*(*surf).format).BitsPerPixel as ::core::ffi::c_int,
                (*(*surf).format).Rmask,
                (*(*surf).format).Gmask,
                (*(*surf).format).Bmask,
                (*(*surf).format).Amask,
            );
            force_fill = true;
        }
        fillfmt = (*newfill).format;
        if force_fill as ::core::ffi::c_int != 0 || !sameColor((*img).last_fill, color) {
            let mut fillColor: Uint32 = 0;
            if mode.kind as ::core::ffi::c_int
                == C2Rust_Unnamed::DRAW_ALPHA.0 as ::core::ffi::c_int
                && (*fillfmt).Amask != 0
            {
                color.a = mode.factor as BYTE;
                mode.kind = C2Rust_Unnamed::DRAW_REPLACE.0 as ::core::ffi::c_int as BYTE;
            } else {
                color.a = 0xff as BYTE;
            }
            fillColor = SDL_MapRGBA(
                (*newfill).format,
                color.r,
                color.g,
                color.b,
                color.a,
            );
            TFB_DrawCanvas_Fill(surf, fillColor, newfill);
            (*img).last_fill = color;
        }
        (*img).FilledImg = newfill as TFB_Canvas;
        surf = newfill;
    }
    TFB_DrawCanvas_Blit(surf, pSrcRect, dst, &raw mut targetRect, mode);
    UnlockMutex((*img).mutex);
}
#[no_mangle]
pub unsafe extern "C" fn TFB_DrawCanvas_FontChar(
    mut fontChar: *mut TFB_Char,
    mut backing: *mut TFB_Image,
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
    mut mode: DrawMode,
    mut target: TFB_Canvas,
) {
    let mut dst: *mut SDL_Surface = target as *mut SDL_Surface;
    let mut srcRect: SDL_Rect = SDL_Rect { x: 0, y: 0, w: 0, h: 0 };
    let mut targetRect: SDL_Rect = SDL_Rect { x: 0, y: 0, w: 0, h: 0 };
    let mut surf: *mut SDL_Surface = ::core::ptr::null_mut::<SDL_Surface>();
    let mut w: ::core::ffi::c_int = 0;
    let mut h: ::core::ffi::c_int = 0;
    if fontChar.is_null() {
        log_add(
            log_Level::log_Warning,
            b"ERROR: TFB_DrawCanvas_FontChar passed null char ptr\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if backing.is_null() {
        log_add(
            log_Level::log_Warning,
            b"ERROR: TFB_DrawCanvas_FontChar passed null backing ptr\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    w = (*fontChar).extent.width as ::core::ffi::c_int;
    h = (*fontChar).extent.height as ::core::ffi::c_int;
    LockMutex((*backing).mutex);
    surf = (*backing).NormalImg as *mut SDL_Surface;
    if (*(*surf).format).BytesPerPixel as ::core::ffi::c_int != 4 as ::core::ffi::c_int
        || (*surf).w < w || (*surf).h < h
    {
        log_add(
            log_Level::log_Warning,
            b"ERROR: TFB_DrawCanvas_FontChar bad backing surface: %dx%dx%d; char: %dx%d\0"
                .as_ptr() as *const ::core::ffi::c_char,
            (*surf).w,
            (*surf).h,
            (*(*surf).format).BytesPerPixel as ::core::ffi::c_int,
            w,
            h,
        );
        UnlockMutex((*backing).mutex);
        return;
    }
    srcRect.x = 0 as ::core::ffi::c_int;
    srcRect.y = 0 as ::core::ffi::c_int;
    srcRect.w = (*fontChar).extent.width as ::core::ffi::c_int;
    srcRect.h = (*fontChar).extent.height as ::core::ffi::c_int;
    targetRect.x = x - (*fontChar).HotSpot.x as ::core::ffi::c_int;
    targetRect.y = y - (*fontChar).HotSpot.y as ::core::ffi::c_int;
    SDL_LockSurface(surf);
    let mut x_0: ::core::ffi::c_int = 0;
    let mut y_0: ::core::ffi::c_int = 0;
    let sskip: ::core::ffi::c_int = (*fontChar).pitch.wrapping_sub(w as DWORD)
        as ::core::ffi::c_int;
    let dskip: ::core::ffi::c_int = (*surf).pitch / 4 as ::core::ffi::c_int - w;
    let dmask: Uint32 = !(*(*surf).format).Amask;
    let ashift: ::core::ffi::c_int = (*(*surf).format).Ashift as ::core::ffi::c_int;
    let mut src_p: *mut Uint8 = (*fontChar).data;
    let mut dst_p: *mut Uint32 = (*surf).pixels as *mut Uint32;
    if mode.kind as ::core::ffi::c_int
        == C2Rust_Unnamed::DRAW_ALPHA.0 as ::core::ffi::c_int
    {
        let mut alpha: ::core::ffi::c_int = mode.factor as ::core::ffi::c_int;
        mode.kind = C2Rust_Unnamed::DRAW_REPLACE.0 as ::core::ffi::c_int as BYTE;
        y_0 = 0 as ::core::ffi::c_int;
        while y_0 < h {
            x_0 = 0 as ::core::ffi::c_int;
            while x_0 < w {
                let mut p: Uint32 = *dst_p & dmask;
                let mut a: Uint32 = *src_p as Uint32;
                if alpha != 0xff as ::core::ffi::c_int {
                    a = a.wrapping_mul(alpha as Uint32) >> 8 as ::core::ffi::c_int;
                }
                *dst_p = p | a << ashift;
                x_0 += 1;
                src_p = src_p.offset(1);
                dst_p = dst_p.offset(1);
            }
            y_0 += 1;
            src_p = src_p.offset(sskip as isize);
            dst_p = dst_p.offset(dskip as isize);
        }
    } else {
        y_0 = 0 as ::core::ffi::c_int;
        while y_0 < h {
            x_0 = 0 as ::core::ffi::c_int;
            while x_0 < w {
                *dst_p = *dst_p & dmask | (*src_p as Uint32) << ashift;
                x_0 += 1;
                src_p = src_p.offset(1);
                dst_p = dst_p.offset(1);
            }
            y_0 += 1;
            src_p = src_p.offset(sskip as isize);
            dst_p = dst_p.offset(dskip as isize);
        }
    }
    SDL_UnlockSurface(surf);
    TFB_DrawCanvas_Blit(surf, &raw mut srcRect, dst, &raw mut targetRect, mode);
    UnlockMutex((*backing).mutex);
}
#[no_mangle]
pub unsafe extern "C" fn TFB_DrawCanvas_New_TrueColor(
    mut w: ::core::ffi::c_int,
    mut h: ::core::ffi::c_int,
    mut hasalpha: BOOLEAN,
) -> TFB_Canvas {
    let mut new_surf: *mut SDL_Surface = ::core::ptr::null_mut::<SDL_Surface>();
    let mut fmt: *mut SDL_PixelFormat = (*format_conv_surf).format;
    new_surf = SDL_CreateRGBSurface(
        0 as Uint32,
        w,
        h,
        (*fmt).BitsPerPixel as ::core::ffi::c_int,
        (*fmt).Rmask,
        (*fmt).Gmask,
        (*fmt).Bmask,
        if hasalpha.0 != 0 { (*fmt).Amask } else { 0 as Uint32 },
    );
    if new_surf.is_null() {
        log_add(
            log_Level::log_Fatal,
            b"INTERNAL PANIC: Failed to create TFB_Canvas: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            SDL_GetError(),
        );
        exit(1 as ::core::ffi::c_int);
    }
    return new_surf as TFB_Canvas;
}
#[no_mangle]
pub unsafe extern "C" fn TFB_DrawCanvas_New_ForScreen(
    mut w: ::core::ffi::c_int,
    mut h: ::core::ffi::c_int,
    mut withalpha: BOOLEAN,
) -> TFB_Canvas {
    let mut new_surf: *mut SDL_Surface = ::core::ptr::null_mut::<SDL_Surface>();
    let mut fmt: *mut SDL_PixelFormat = (*SDL_Screen).format;
    if !(*fmt).palette.is_null() {
        log_add(
            log_Level::log_Warning,
            b"TFB_DrawCanvas_New_ForScreen() WARNING:Paletted display format will be slow\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        new_surf = TFB_DrawCanvas_New_TrueColor(w, h, withalpha) as *mut SDL_Surface;
    } else {
        if withalpha.0 != 0 && (*fmt).Amask == 0 as Uint32 {
            fmt = (*format_conv_surf).format;
        }
        new_surf = SDL_CreateRGBSurface(
            0 as Uint32,
            w,
            h,
            (*fmt).BitsPerPixel as ::core::ffi::c_int,
            (*fmt).Rmask,
            (*fmt).Gmask,
            (*fmt).Bmask,
            if withalpha.0 != 0 { (*fmt).Amask } else { 0 as Uint32 },
        );
    }
    if new_surf.is_null() {
        log_add(
            log_Level::log_Fatal,
            b"TFB_DrawCanvas_New_ForScreen() INTERNAL PANIC:Failed to create TFB_Canvas: %s\0"
                .as_ptr() as *const ::core::ffi::c_char,
            SDL_GetError(),
        );
        exit(1 as ::core::ffi::c_int);
    }
    return new_surf as TFB_Canvas;
}
#[no_mangle]
pub unsafe extern "C" fn TFB_DrawCanvas_New_Paletted(
    mut w: ::core::ffi::c_int,
    mut h: ::core::ffi::c_int,
    mut palette: *mut Color,
    mut transparent_index: ::core::ffi::c_int,
) -> TFB_Canvas {
    let mut new_surf: *mut SDL_Surface = ::core::ptr::null_mut::<SDL_Surface>();
    new_surf = SDL_CreateRGBSurface(
        0 as Uint32,
        w,
        h,
        8 as ::core::ffi::c_int,
        0 as Uint32,
        0 as Uint32,
        0 as Uint32,
        0 as Uint32,
    );
    if new_surf.is_null() {
        log_add(
            log_Level::log_Fatal,
            b"INTERNAL PANIC: Failed to create TFB_Canvas: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            SDL_GetError(),
        );
        exit(1 as ::core::ffi::c_int);
    }
    if !palette.is_null() {
        TFB_DrawCanvas_SetPalette(new_surf as TFB_Canvas, palette);
    }
    if transparent_index >= 0 as ::core::ffi::c_int {
        TFB_SetColorKey(new_surf, transparent_index as Uint32, 0 as ::core::ffi::c_int);
    } else {
        TFB_DisableColorKey(new_surf);
    }
    return new_surf as TFB_Canvas;
}
#[no_mangle]
pub unsafe extern "C" fn TFB_DrawCanvas_New_ScaleTarget(
    mut canvas: TFB_Canvas,
    mut oldcanvas: TFB_Canvas,
    mut r#type: ::core::ffi::c_int,
    mut last_type: ::core::ffi::c_int,
) -> TFB_Canvas {
    let mut src: *mut SDL_Surface = canvas as *mut SDL_Surface;
    let mut old: *mut SDL_Surface = oldcanvas as *mut SDL_Surface;
    let mut newsurf: *mut SDL_Surface = ::core::ptr::null_mut::<SDL_Surface>();
    if r#type == C2Rust_Unnamed_0::TFB_SCALE_TRILINEAR.0 as ::core::ffi::c_int {
        r#type = C2Rust_Unnamed_0::TFB_SCALE_BILINEAR.0 as ::core::ffi::c_int;
    }
    if last_type == C2Rust_Unnamed_0::TFB_SCALE_TRILINEAR.0 as ::core::ffi::c_int {
        last_type = C2Rust_Unnamed_0::TFB_SCALE_BILINEAR.0 as ::core::ffi::c_int;
    }
    if !old.is_null() && r#type != last_type {
        TFB_DrawCanvas_Delete(old as TFB_Canvas);
        old = ::core::ptr::null_mut::<SDL_Surface>();
    }
    if !old.is_null() {
        return old as TFB_Canvas;
    }
    if r#type == C2Rust_Unnamed_0::TFB_SCALE_NEAREST.0 as ::core::ffi::c_int {
        newsurf = SDL_CreateRGBSurface(
            0 as Uint32,
            (*src).w,
            (*src).h,
            (*(*src).format).BitsPerPixel as ::core::ffi::c_int,
            (*(*src).format).Rmask,
            (*(*src).format).Gmask,
            (*(*src).format).Bmask,
            (*(*src).format).Amask,
        );
        TFB_DrawCanvas_CopyTransparencyInfo(src as TFB_Canvas, newsurf as TFB_Canvas);
    } else if (*(*SDL_Screen).format).BitsPerPixel as ::core::ffi::c_int
        == 32 as ::core::ffi::c_int
    {
        newsurf = TFB_DrawCanvas_New_ForScreen(
            (*src).w + 1 as ::core::ffi::c_int,
            (*src).h + 1 as ::core::ffi::c_int,
            BOOLEAN::TRUE,
        ) as *mut SDL_Surface;
    } else {
        newsurf = TFB_DrawCanvas_New_TrueColor(
            (*src).w + 1 as ::core::ffi::c_int,
            (*src).h + 1 as ::core::ffi::c_int,
            BOOLEAN::TRUE,
        ) as *mut SDL_Surface;
    }
    return newsurf as TFB_Canvas;
}
#[no_mangle]
pub unsafe extern "C" fn TFB_DrawCanvas_New_RotationTarget(
    mut src_canvas: TFB_Canvas,
    mut angle: ::core::ffi::c_int,
) -> TFB_Canvas {
    let mut src: *mut SDL_Surface = src_canvas as *mut SDL_Surface;
    let mut newsurf: *mut SDL_Surface = ::core::ptr::null_mut::<SDL_Surface>();
    let mut size: EXTENT = EXTENT { width: 0, height: 0 };
    TFB_DrawCanvas_GetRotatedExtent(src_canvas, angle, &raw mut size);
    newsurf = SDL_CreateRGBSurface(
        0 as Uint32,
        size.width as ::core::ffi::c_int,
        size.height as ::core::ffi::c_int,
        (*(*src).format).BitsPerPixel as ::core::ffi::c_int,
        (*(*src).format).Rmask,
        (*(*src).format).Gmask,
        (*(*src).format).Bmask,
        (*(*src).format).Amask,
    );
    if newsurf.is_null() {
        log_add(
            log_Level::log_Fatal,
            b"TFB_DrawCanvas_New_RotationTarget() INTERNAL PANIC: Failed to create TFB_Canvas: %s\0"
                .as_ptr() as *const ::core::ffi::c_char,
            SDL_GetError(),
        );
        exit(1 as ::core::ffi::c_int);
    }
    TFB_DrawCanvas_CopyTransparencyInfo(src as TFB_Canvas, newsurf as TFB_Canvas);
    return newsurf as TFB_Canvas;
}
#[no_mangle]
pub unsafe extern "C" fn TFB_DrawCanvas_LoadFromFile(
    mut dir: *mut ::core::ffi::c_void,
    mut fileName: *const ::core::ffi::c_char,
) -> TFB_Canvas {
    let mut surf: *mut SDL_Surface = sdluio_loadImage(
        dir as *mut uio_DirHandle,
        fileName,
    );
    if surf.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    if ((*(*surf).format).BitsPerPixel as ::core::ffi::c_int) < 8 as ::core::ffi::c_int {
        SDL_SetError(
            b"unsupported image format (min 8bpp)\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        SDL_FreeSurface(surf);
        surf = ::core::ptr::null_mut::<SDL_Surface>();
    }
    return surf as TFB_Canvas;
}
#[no_mangle]
pub unsafe extern "C" fn TFB_DrawCanvas_Delete(mut canvas: TFB_Canvas) {
    if canvas.is_null() {
        log_add(
            log_Level::log_Warning,
            b"INTERNAL PANIC: Attempted to delete a NULL canvas!\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    } else {
        SDL_FreeSurface(canvas as *mut SDL_Surface);
    };
}
#[no_mangle]
pub unsafe extern "C" fn TFB_DrawCanvas_GetFontCharData(
    mut canvas: TFB_Canvas,
    mut outData: *mut BYTE,
    mut dataPitch: ::core::ffi::c_uint,
) -> BOOLEAN {
    let mut surf: *mut SDL_Surface = canvas as *mut SDL_Surface;
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut r: Uint8 = 0;
    let mut g: Uint8 = 0;
    let mut b: Uint8 = 0;
    let mut a: Uint8 = 0;
    let mut p: Uint32 = 0;
    let mut fmt: *mut SDL_PixelFormat = (*surf).format;
    let mut getpix: GetPixelFn = None;
    if surf.is_null() || outData.is_null() {
        return BOOLEAN::FALSE;
    }
    SDL_LockSurface(surf);
    getpix = getpixel_for(surf);
    y = 0 as ::core::ffi::c_int;
    while y < (*surf).h {
        let mut dst: *mut BYTE = outData
            .offset(dataPitch.wrapping_mul(y as ::core::ffi::c_uint) as isize);
        x = 0 as ::core::ffi::c_int;
        while x < (*surf).w {
            p = getpix.expect("non-null function pointer")(surf, x, y);
            SDL_GetRGBA(p, fmt, &raw mut r, &raw mut g, &raw mut b, &raw mut a);
            if (*fmt).Amask == 0 {
                a = ((r as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    + g as ::core::ffi::c_int * 4 as ::core::ffi::c_int
                    + b as ::core::ffi::c_int) / 7 as ::core::ffi::c_int) as Uint8;
            }
            *dst = a as BYTE;
            x += 1;
            dst = dst.offset(1);
        }
        y += 1;
    }
    SDL_UnlockSurface(surf);
    return BOOLEAN::TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn TFB_DrawCanvas_ExtractPalette(
    mut canvas: TFB_Canvas,
) -> *mut Color {
    let mut i: ::core::ffi::c_int = 0;
    let mut result: *mut Color = ::core::ptr::null_mut::<Color>();
    let mut surf: *mut SDL_Surface = canvas as *mut SDL_Surface;
    let mut palette: *mut SDL_Palette = (*(*surf).format).palette;
    if palette.is_null() {
        return ::core::ptr::null_mut::<Color>();
    }
    result = HCalloc(::core::mem::size_of::<Color>().wrapping_mul(256 as size_t))
        as *mut Color;
    ((*palette).ncolors <= 256 as ::core::ffi::c_int
        || {
            __assert_fail(
                b"palette->ncolors <= 256\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/libs/graphics/sdl/canvas.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                847 as ::core::ffi::c_int,
                b"TFB_DrawCanvas_ExtractPalette\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < (*palette).ncolors {
        *result.offset(i as isize) = NativeToColor(
            *(*palette).colors.offset(i as isize),
        );
        i += 1;
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn TFB_DrawCanvas_ToScreenFormat(
    mut canvas: TFB_Canvas,
) -> TFB_Canvas {
    let mut result: *mut SDL_Surface = TFB_DisplayFormatAlpha(
        canvas as *mut SDL_Surface,
    );
    if result.is_null() {
        log_add(
            log_Level::log_Debug,
            b"WARNING: Could not convert sprite-canvas to display format.\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return canvas;
    } else if result == canvas as *mut SDL_Surface {
        return canvas
    } else {
        TFB_DrawCanvas_Delete(canvas);
        return result as TFB_Canvas;
    };
}
#[no_mangle]
pub unsafe extern "C" fn TFB_DrawCanvas_IsPaletted(mut canvas: TFB_Canvas) -> BOOLEAN {
    return BOOLEAN(
        !(*(*(canvas as *mut SDL_Surface)).format).palette.is_null()
            as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
}
#[no_mangle]
pub unsafe extern "C" fn TFB_DrawCanvas_SetPalette(
    mut target: TFB_Canvas,
    mut palette: *mut Color,
) {
    let mut colors: [SDL_Color; 256] = [SDL_Color {
        r: 0,
        g: 0,
        b: 0,
        a: 0,
    }; 256];
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < 256 as ::core::ffi::c_int {
        colors[i as usize] = ColorToNative(*palette.offset(i as isize));
        i += 1;
    }
    TFB_SetColors(
        target as *mut SDL_Surface,
        &raw mut colors as *mut SDL_Color,
        0 as ::core::ffi::c_int,
        256 as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn TFB_DrawCanvas_GetTransparentIndex(
    mut canvas: TFB_Canvas,
) -> ::core::ffi::c_int {
    let mut colorkey: Uint32 = 0;
    if TFB_GetColorKey(canvas as *mut SDL_Surface, &raw mut colorkey) != 0 {
        return colorkey as ::core::ffi::c_int;
    }
    return -1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn TFB_DrawCanvas_SetTransparentIndex(
    mut canvas: TFB_Canvas,
    mut index: ::core::ffi::c_int,
    mut rleaccel: BOOLEAN,
) {
    if index >= 0 as ::core::ffi::c_int {
        TFB_SetColorKey(
            canvas as *mut SDL_Surface,
            index as Uint32,
            rleaccel.0 as ::core::ffi::c_int,
        );
        if TFB_DrawCanvas_IsPaletted(canvas).0 == 0 {
            TFB_DisableSurfaceAlphaMod(canvas as *mut SDL_Surface);
        }
    } else {
        TFB_DisableColorKey(canvas as *mut SDL_Surface);
    };
}
#[no_mangle]
pub unsafe extern "C" fn TFB_DrawCanvas_CopyTransparencyInfo(
    mut src_canvas: TFB_Canvas,
    mut dst_canvas: TFB_Canvas,
) {
    let mut src: *mut SDL_Surface = src_canvas as *mut SDL_Surface;
    if !(*(*src).format).palette.is_null() {
        let mut index: ::core::ffi::c_int = 0;
        index = TFB_DrawCanvas_GetTransparentIndex(src_canvas);
        TFB_DrawCanvas_SetTransparentIndex(dst_canvas, index, BOOLEAN::FALSE);
    } else {
        let mut color: Color = Color { r: 0, g: 0, b: 0, a: 0 };
        if TFB_DrawCanvas_GetTransparentColor(src_canvas, &raw mut color).0 != 0 {
            TFB_DrawCanvas_SetTransparentColor(dst_canvas, color, BOOLEAN::FALSE);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn TFB_DrawCanvas_GetTransparentColor(
    mut canvas: TFB_Canvas,
    mut color: *mut Color,
) -> BOOLEAN {
    let mut colorkey: Uint32 = 0;
    if TFB_DrawCanvas_IsPaletted(canvas).0 == 0
        && TFB_GetColorKey(canvas as *mut SDL_Surface, &raw mut colorkey)
            == 0 as ::core::ffi::c_int
    {
        let mut ur: Uint8 = 0;
        let mut ug: Uint8 = 0;
        let mut ub: Uint8 = 0;
        SDL_GetRGB(
            colorkey,
            (*(canvas as *mut SDL_Surface)).format,
            &raw mut ur,
            &raw mut ug,
            &raw mut ub,
        );
        (*color).r = ur as BYTE;
        (*color).g = ug as BYTE;
        (*color).b = ub as BYTE;
        (*color).a = 0xff as BYTE;
        return BOOLEAN::TRUE;
    }
    return BOOLEAN::FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn TFB_DrawCanvas_SetTransparentColor(
    mut canvas: TFB_Canvas,
    mut color: Color,
    mut rleaccel: BOOLEAN,
) {
    let mut sdlColor: Uint32 = 0;
    sdlColor = SDL_MapRGBA(
        (*(canvas as *mut SDL_Surface)).format,
        color.r,
        color.g,
        color.b,
        0 as Uint8,
    );
    TFB_SetColorKey(
        canvas as *mut SDL_Surface,
        sdlColor,
        rleaccel.0 as ::core::ffi::c_int,
    );
    if TFB_DrawCanvas_IsPaletted(canvas).0 == 0 {
        TFB_DisableSurfaceAlphaMod(canvas as *mut SDL_Surface);
    }
}
#[no_mangle]
pub unsafe extern "C" fn TFB_DrawCanvas_GetScaledExtent(
    mut src_canvas: TFB_Canvas,
    mut src_hs: *mut HOT_SPOT,
    mut src_mipmap: TFB_Canvas,
    mut mm_hs: *mut HOT_SPOT,
    mut scale: ::core::ffi::c_int,
    mut r#type: ::core::ffi::c_int,
    mut size: *mut EXTENT,
    mut hs: *mut HOT_SPOT,
) {
    let mut src: *mut SDL_Surface = src_canvas as *mut SDL_Surface;
    let mut x: sint32 = 0;
    let mut y: sint32 = 0;
    let mut w: sint32 = 0;
    let mut h: sint32 = 0;
    let mut frac: ::core::ffi::c_int = 0;
    if src_mipmap.is_null() {
        w = ((*src).w * scale) as sint32;
        h = ((*src).h * scale) as sint32;
        x = ((*src_hs).x as ::core::ffi::c_int * scale) as sint32;
        y = ((*src_hs).y as ::core::ffi::c_int * scale) as sint32;
    } else {
        let mut mipmap: *mut SDL_Surface = src_mipmap as *mut SDL_Surface;
        let mut ratio: ::core::ffi::c_int = scale * 2 as ::core::ffi::c_int
            - 256 as ::core::ffi::c_int;
        (scale >= 256 as ::core::ffi::c_int / 2 as ::core::ffi::c_int
            || {
                __assert_fail(
                    b"scale >= GSCALE_IDENTITY / 2\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"src/libs/graphics/sdl/canvas.c\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    1002 as ::core::ffi::c_int,
                    b"TFB_DrawCanvas_GetScaledExtent\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                0 as ::core::ffi::c_int != 0
            }) as ::core::ffi::c_int;
        w = ((*mipmap).w * 256 as ::core::ffi::c_int + ((*src).w - (*mipmap).w) * ratio)
            as sint32;
        h = ((*mipmap).h * 256 as ::core::ffi::c_int + ((*src).h - (*mipmap).h) * ratio)
            as sint32;
        x = ((*mm_hs).x as ::core::ffi::c_int * 256 as ::core::ffi::c_int
            + ((*src_hs).x as ::core::ffi::c_int - (*mm_hs).x as ::core::ffi::c_int)
                * ratio) as sint32;
        y = ((*mm_hs).y as ::core::ffi::c_int * 256 as ::core::ffi::c_int
            + ((*src_hs).y as ::core::ffi::c_int - (*mm_hs).y as ::core::ffi::c_int)
                * ratio) as sint32;
    }
    if r#type != C2Rust_Unnamed_0::TFB_SCALE_NEAREST.0 as ::core::ffi::c_int {
        if x & 256 as ::core::ffi::c_int - 1 as ::core::ffi::c_int != 0 {
            frac = 256 as ::core::ffi::c_int
                - (x & 256 as ::core::ffi::c_int - 1 as ::core::ffi::c_int);
            x += frac;
            w += frac;
        }
        if y & 256 as ::core::ffi::c_int - 1 as ::core::ffi::c_int != 0 {
            frac = 256 as ::core::ffi::c_int
                - (y & 256 as ::core::ffi::c_int - 1 as ::core::ffi::c_int);
            y += frac;
            h += frac;
        }
        w += 256 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
        h += 256 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    }
    (*size).width = (w / 256 as ::core::ffi::c_int) as COORD;
    (*size).height = (h / 256 as ::core::ffi::c_int) as COORD;
    (*hs).x = (x / 256 as ::core::ffi::c_int) as COORD;
    (*hs).y = (y / 256 as ::core::ffi::c_int) as COORD;
    ((*size).width as ::core::ffi::c_int <= (*src).w + 1 as ::core::ffi::c_int
        && (*size).height as ::core::ffi::c_int <= (*src).h + 1 as ::core::ffi::c_int
        || {
            __assert_fail(
                b"size->width <= src->w + 1 && size->height <= src->h + 1\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/libs/graphics/sdl/canvas.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                1040 as ::core::ffi::c_int,
                b"TFB_DrawCanvas_GetScaledExtent\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    if (*size).width == 0 && (*src).w != 0 {
        (*size).width = 1 as COORD;
    }
    if (*size).height == 0 && (*src).h != 0 {
        (*size).height = 1 as COORD;
    }
}
#[no_mangle]
pub unsafe extern "C" fn TFB_DrawCanvas_GetExtent(
    mut canvas: TFB_Canvas,
    mut size: *mut EXTENT,
) {
    let mut src: *mut SDL_Surface = canvas as *mut SDL_Surface;
    (*size).width = (*src).w as COORD;
    (*size).height = (*src).h as COORD;
}
#[no_mangle]
pub unsafe extern "C" fn TFB_DrawCanvas_Rescale_Nearest(
    mut src_canvas: TFB_Canvas,
    mut dst_canvas: TFB_Canvas,
    mut scale: ::core::ffi::c_int,
    mut src_hs: *mut HOT_SPOT,
    mut size: *mut EXTENT,
    mut dst_hs: *mut HOT_SPOT,
) {
    let mut src: *mut SDL_Surface = src_canvas as *mut SDL_Surface;
    let mut dst: *mut SDL_Surface = dst_canvas as *mut SDL_Surface;
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut fsx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut fsy: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut ssx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut ssy: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut w: ::core::ffi::c_int = 0;
    let mut h: ::core::ffi::c_int = 0;
    if scale > 0 as ::core::ffi::c_int {
        TFB_DrawCanvas_GetScaledExtent(
            src as TFB_Canvas,
            src_hs,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            ::core::ptr::null_mut::<HOT_SPOT>(),
            scale,
            C2Rust_Unnamed_0::TFB_SCALE_NEAREST.0 as ::core::ffi::c_int,
            size,
            dst_hs,
        );
        w = (*size).width as ::core::ffi::c_int;
        h = (*size).height as ::core::ffi::c_int;
    } else {
        w = (*dst).w;
        h = (*dst).h;
    }
    if w > (*dst).w || h > (*dst).h {
        log_add(
            log_Level::log_Warning,
            b"TFB_DrawCanvas_Rescale_Nearest: Tried to scale image to size %d %d when dest_canvas has only dimensions of %d %d! Failing.\0"
                .as_ptr() as *const ::core::ffi::c_char,
            w,
            h,
            (*dst).w,
            (*dst).h,
        );
        return;
    }
    if w > 1 as ::core::ffi::c_int {
        fsx = (((*src).w - 1 as ::core::ffi::c_int) << 16 as ::core::ffi::c_int)
            / (w - 1 as ::core::ffi::c_int);
    }
    if h > 1 as ::core::ffi::c_int {
        fsy = (((*src).h - 1 as ::core::ffi::c_int) << 16 as ::core::ffi::c_int)
            / (h - 1 as ::core::ffi::c_int);
    }
    ssx = 0x6000 as ::core::ffi::c_int;
    ssy = 0x6000 as ::core::ffi::c_int;
    SDL_LockSurface(src);
    SDL_LockSurface(dst);
    if (*(*src).format).BytesPerPixel as ::core::ffi::c_int == 1 as ::core::ffi::c_int
        && (*(*dst).format).BytesPerPixel as ::core::ffi::c_int
            == 1 as ::core::ffi::c_int
    {
        let mut sp: *mut Uint8 = ::core::ptr::null_mut::<Uint8>();
        let mut csp: *mut Uint8 = ::core::ptr::null_mut::<Uint8>();
        let mut dp: *mut Uint8 = ::core::ptr::null_mut::<Uint8>();
        let mut cdp: *mut Uint8 = ::core::ptr::null_mut::<Uint8>();
        let mut sx: ::core::ffi::c_int = 0;
        let mut sy: ::core::ffi::c_int = 0;
        csp = (*src).pixels as *mut Uint8;
        sp = csp;
        cdp = (*dst).pixels as *mut Uint8;
        dp = cdp;
        y = 0 as ::core::ffi::c_int;
        sy = ssy;
        while y < h {
            csp = csp.offset(((sy >> 16 as ::core::ffi::c_int) * (*src).pitch) as isize);
            sp = csp;
            dp = cdp;
            x = 0 as ::core::ffi::c_int;
            sx = ssx;
            while x < w {
                sp = sp.offset((sx >> 16 as ::core::ffi::c_int) as isize);
                *dp = *sp;
                sx &= 0xffff as ::core::ffi::c_int;
                sx += fsx;
                dp = dp.offset(1);
                x += 1;
            }
            sy &= 0xffff as ::core::ffi::c_int;
            sy += fsy;
            cdp = cdp.offset((*dst).pitch as isize);
            y += 1;
        }
    } else if (*(*src).format).BytesPerPixel as ::core::ffi::c_int
        == 4 as ::core::ffi::c_int
        && (*(*dst).format).BytesPerPixel as ::core::ffi::c_int
            == 4 as ::core::ffi::c_int
    {
        let mut sp_0: *mut Uint32 = ::core::ptr::null_mut::<Uint32>();
        let mut csp_0: *mut Uint32 = ::core::ptr::null_mut::<Uint32>();
        let mut dp_0: *mut Uint32 = ::core::ptr::null_mut::<Uint32>();
        let mut cdp_0: *mut Uint32 = ::core::ptr::null_mut::<Uint32>();
        let mut sx_0: ::core::ffi::c_int = 0;
        let mut sy_0: ::core::ffi::c_int = 0;
        let mut sgap: ::core::ffi::c_int = 0;
        let mut dgap: ::core::ffi::c_int = 0;
        sgap = (*src).pitch >> 2 as ::core::ffi::c_int;
        dgap = (*dst).pitch >> 2 as ::core::ffi::c_int;
        csp_0 = (*src).pixels as *mut Uint32;
        sp_0 = csp_0;
        cdp_0 = (*dst).pixels as *mut Uint32;
        dp_0 = cdp_0;
        y = 0 as ::core::ffi::c_int;
        sy_0 = ssy;
        while y < h {
            csp_0 = csp_0.offset(((sy_0 >> 16 as ::core::ffi::c_int) * sgap) as isize);
            sp_0 = csp_0;
            dp_0 = cdp_0;
            x = 0 as ::core::ffi::c_int;
            sx_0 = ssx;
            while x < w {
                sp_0 = sp_0.offset((sx_0 >> 16 as ::core::ffi::c_int) as isize);
                *dp_0 = *sp_0;
                sx_0 &= 0xffff as ::core::ffi::c_int;
                sx_0 += fsx;
                dp_0 = dp_0.offset(1);
                x += 1;
            }
            sy_0 &= 0xffff as ::core::ffi::c_int;
            sy_0 += fsy;
            cdp_0 = cdp_0.offset(dgap as isize);
            y += 1;
        }
    } else {
        log_add(
            log_Level::log_Warning,
            b"Tried to deal with unknown BPP: %d -> %d\0".as_ptr()
                as *const ::core::ffi::c_char,
            (*(*src).format).BitsPerPixel as ::core::ffi::c_int,
            (*(*dst).format).BitsPerPixel as ::core::ffi::c_int,
        );
    }
    SDL_UnlockSurface(dst);
    SDL_UnlockSurface(src);
}
#[inline]
unsafe extern "C" fn dot_product_8_4(
    mut p: *mut pixel_t,
    mut c: ::core::ffi::c_int,
    mut v: *mut Uint8,
) -> Uint8 {
    return (btable[(*p.offset(0isize)).chan[c as usize]
        as usize][*v.offset(0isize) as usize] as ::core::ffi::c_int
        + btable[(*p.offset(1isize)).chan[c as usize]
            as usize][*v.offset(1isize) as usize] as ::core::ffi::c_int
        + btable[(*p.offset(2isize)).chan[c as usize]
            as usize][*v.offset(2isize) as usize] as ::core::ffi::c_int
        + btable[(*p.offset(3isize)).chan[c as usize]
            as usize][*v.offset(3isize) as usize] as ::core::ffi::c_int) as Uint8;
}
#[inline]
unsafe extern "C" fn weight_product_8_4(
    mut p: *mut pixel_t,
    mut c: ::core::ffi::c_int,
    mut w: *mut Uint8,
) -> Uint8 {
    return ((*p.offset(0isize)).chan[c as usize] as Uint32)
        .wrapping_mul(*w.offset(0isize) as Uint32)
        .wrapping_add(
            ((*p.offset(1isize)).chan[c as usize] as Uint32)
                .wrapping_mul(*w.offset(1isize) as Uint32),
        )
        .wrapping_add(
            ((*p.offset(2isize)).chan[c as usize] as Uint32)
                .wrapping_mul(*w.offset(2isize) as Uint32),
        )
        .wrapping_add(
            ((*p.offset(3isize)).chan[c as usize] as Uint32)
                .wrapping_mul(*w.offset(3isize) as Uint32),
        )
        .wrapping_div(
            (*w.offset(0isize) as ::core::ffi::c_int
                + *w.offset(1isize) as ::core::ffi::c_int
                + *w.offset(2isize) as ::core::ffi::c_int
                + *w.offset(3isize) as ::core::ffi::c_int) as Uint32,
        ) as Uint8;
}
#[inline]
unsafe extern "C" fn blend_ratio_2(
    mut c1: Uint8,
    mut c2: Uint8,
    mut ratio: ::core::ffi::c_int,
) -> Uint8 {
    return (((c1 as ::core::ffi::c_int - c2 as ::core::ffi::c_int) * ratio
        >> 8 as ::core::ffi::c_int) + c2 as ::core::ffi::c_int) as Uint8;
}
#[inline]
unsafe extern "C" fn scale_read_pixel(
    mut ppix: *mut ::core::ffi::c_void,
    mut fmt: *mut SDL_PixelFormat,
    mut pal: *mut SDL_Color,
    mut mask: Uint32,
    mut key: Uint32,
) -> Uint32 {
    let mut p: pixel_t = pixel_t { value: 0 };
    p.value = 0 as Uint32;
    if !pal.is_null() {
        let mut c: Uint32 = *(ppix as *mut Uint8) as Uint32;
        if c != key {
            p.c.r = (*pal.offset(c as isize)).r;
            p.c.g = (*pal.offset(c as isize)).g;
            p.c.b = (*pal.offset(c as isize)).b;
            p.c.a = 255 as Uint8;
        }
    } else {
        let mut c_0: Uint32 = *(ppix as *mut Uint32);
        if c_0 & mask != key {
            p.c.r = (c_0 >> (*fmt).Rshift as ::core::ffi::c_int & 0xff as Uint32)
                as Uint8;
            p.c.g = (c_0 >> (*fmt).Gshift as ::core::ffi::c_int & 0xff as Uint32)
                as Uint8;
            p.c.b = (c_0 >> (*fmt).Bshift as ::core::ffi::c_int & 0xff as Uint32)
                as Uint8;
            if (*fmt).Amask != 0 {
                p.c.a = (c_0 >> (*fmt).Ashift as ::core::ffi::c_int & 0xff as Uint32)
                    as Uint8;
            } else {
                p.c.a = 255 as Uint8;
            }
        }
    }
    return p.value;
}
#[inline]
unsafe extern "C" fn scale_get_pixel(
    mut src: *mut SDL_Surface,
    mut mask: Uint32,
    mut key: Uint32,
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
) -> Uint32 {
    let mut pal: *mut SDL_Color = if !(*(*src).format).palette.is_null() {
        (*(*(*src).format).palette).colors
    } else {
        ::core::ptr::null_mut::<SDL_Color>()
    };
    if x < 0 as ::core::ffi::c_int || x >= (*src).w || y < 0 as ::core::ffi::c_int
        || y >= (*src).h
    {
        return 0 as Uint32;
    }
    return scale_read_pixel(
        ((*src).pixels as *mut Uint8)
            .offset((y * (*src).pitch) as isize)
            .offset((x * (*(*src).format).BytesPerPixel as ::core::ffi::c_int) as isize)
            as *mut ::core::ffi::c_void,
        (*src).format,
        pal,
        mask,
        key,
    );
}
#[no_mangle]
pub unsafe extern "C" fn TFB_DrawCanvas_Rescale_Trilinear(
    mut src_canvas: TFB_Canvas,
    mut src_mipmap: TFB_Canvas,
    mut dst_canvas: TFB_Canvas,
    mut scale: ::core::ffi::c_int,
    mut src_hs: *mut HOT_SPOT,
    mut mm_hs: *mut HOT_SPOT,
    mut size: *mut EXTENT,
    mut dst_hs: *mut HOT_SPOT,
) {
    let mut src: *mut SDL_Surface = src_canvas as *mut SDL_Surface;
    let mut dst: *mut SDL_Surface = dst_canvas as *mut SDL_Surface;
    let mut mm: *mut SDL_Surface = src_mipmap as *mut SDL_Surface;
    let mut srcfmt: *mut SDL_PixelFormat = (*src).format;
    let mut mmfmt: *mut SDL_PixelFormat = (*mm).format;
    let mut dstfmt: *mut SDL_PixelFormat = (*dst).format;
    let mut srcpal: *mut SDL_Color = if !(*srcfmt).palette.is_null() {
        (*(*srcfmt).palette).colors
    } else {
        ::core::ptr::null_mut::<SDL_Color>()
    };
    let sbpp: ::core::ffi::c_int = (*srcfmt).BytesPerPixel as ::core::ffi::c_int;
    let mmbpp: ::core::ffi::c_int = (*mmfmt).BytesPerPixel as ::core::ffi::c_int;
    let slen: ::core::ffi::c_int = (*src).pitch;
    let mmlen: ::core::ffi::c_int = (*mm).pitch;
    let dst_has_alpha: ::core::ffi::c_int = ((*dstfmt).Amask != 0 as Uint32)
        as ::core::ffi::c_int;
    let mut transparent: Uint32 = 0 as Uint32;
    let alpha_threshold: ::core::ffi::c_int = if dst_has_alpha != 0 {
        0 as ::core::ffi::c_int
    } else {
        127 as ::core::ffi::c_int
    };
    let mut ratio: ::core::ffi::c_int = scale * 2 as ::core::ffi::c_int
        - 256 as ::core::ffi::c_int;
    let mut mk0: Uint32 = 0 as Uint32;
    let mut ck0: Uint32 = !(0 as ::core::ffi::c_int) as Uint32;
    let mut mk1: Uint32 = 0 as Uint32;
    let mut ck1: Uint32 = !(0 as ::core::ffi::c_int) as Uint32;
    let mut sx0: ::core::ffi::c_int = 0;
    let mut sy0: ::core::ffi::c_int = 0;
    let mut sx1: ::core::ffi::c_int = 0;
    let mut sy1: ::core::ffi::c_int = 0;
    let mut fsx0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut fsy0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut fsx1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut fsy1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut ssx0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut ssy0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut ssx1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut ssy1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut w: ::core::ffi::c_int = 0;
    let mut h: ::core::ffi::c_int = 0;
    TFB_GetColorKey(dst, &raw mut transparent);
    if !(*mmfmt).palette.is_null() && srcpal.is_null() {
        log_add(
            log_Level::log_Warning,
            b"TFB_DrawCanvas_Rescale_Trilinear: Mipmap is paletted, but source is not! Failing.\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    if scale > 0 as ::core::ffi::c_int {
        let mut fw: ::core::ffi::c_int = 0;
        let mut fh: ::core::ffi::c_int = 0;
        TFB_DrawCanvas_GetScaledExtent(
            src as TFB_Canvas,
            src_hs,
            mm as TFB_Canvas,
            mm_hs,
            scale,
            C2Rust_Unnamed_0::TFB_SCALE_TRILINEAR.0 as ::core::ffi::c_int,
            size,
            dst_hs,
        );
        w = (*size).width as ::core::ffi::c_int;
        h = (*size).height as ::core::ffi::c_int;
        fw = (*mm).w * 256 as ::core::ffi::c_int + ((*src).w - (*mm).w) * ratio;
        fh = (*mm).h * 256 as ::core::ffi::c_int + ((*src).h - (*mm).h) * ratio;
        fsx0 = ((*src).w << 20 as ::core::ffi::c_int) / fw;
        fsx0 <<= 4 as ::core::ffi::c_int;
        fsy0 = ((*src).h << 20 as ::core::ffi::c_int) / fh;
        fsy0 <<= 4 as ::core::ffi::c_int;
        fsx1 = ((*mm).w << 20 as ::core::ffi::c_int) / fw;
        fsx1 <<= 4 as ::core::ffi::c_int;
        fsy1 = ((*mm).h << 20 as ::core::ffi::c_int) / fh;
        fsy1 <<= 4 as ::core::ffi::c_int;
        ssx0 = (((*src_hs).x as ::core::ffi::c_int) << 16 as ::core::ffi::c_int)
            - fsx0 * (*dst_hs).x as ::core::ffi::c_int;
        ssy0 = (((*src_hs).y as ::core::ffi::c_int) << 16 as ::core::ffi::c_int)
            - fsy0 * (*dst_hs).y as ::core::ffi::c_int;
        ssx1 = (((*mm_hs).x as ::core::ffi::c_int) << 16 as ::core::ffi::c_int)
            - fsx1 * (*dst_hs).x as ::core::ffi::c_int;
        ssy1 = (((*mm_hs).y as ::core::ffi::c_int) << 16 as ::core::ffi::c_int)
            - fsy1 * (*dst_hs).y as ::core::ffi::c_int;
    } else {
        w = (*dst).w;
        h = (*dst).h;
        fsx0 = ((*src).w << 16 as ::core::ffi::c_int) / w;
        fsy0 = ((*src).h << 16 as ::core::ffi::c_int) / h;
        fsx1 = ((*mm).w << 16 as ::core::ffi::c_int) / w;
        fsy1 = ((*mm).h << 16 as ::core::ffi::c_int) / h;
        ssx0 = (((*src).w - 1 as ::core::ffi::c_int) << 16 as ::core::ffi::c_int)
            - fsx0 * (w - 1 as ::core::ffi::c_int) >> 1 as ::core::ffi::c_int;
        ssy0 = (((*src).h - 1 as ::core::ffi::c_int) << 16 as ::core::ffi::c_int)
            - fsy0 * (h - 1 as ::core::ffi::c_int) >> 1 as ::core::ffi::c_int;
        ssx1 = (((*mm).w - 1 as ::core::ffi::c_int) << 16 as ::core::ffi::c_int)
            - fsx1 * (w - 1 as ::core::ffi::c_int) >> 1 as ::core::ffi::c_int;
        ssy1 = (((*mm).h - 1 as ::core::ffi::c_int) << 16 as ::core::ffi::c_int)
            - fsy1 * (h - 1 as ::core::ffi::c_int) >> 1 as ::core::ffi::c_int;
    }
    if w > (*dst).w || h > (*dst).h {
        log_add(
            log_Level::log_Warning,
            b"TFB_DrawCanvas_Rescale_Trilinear: Tried to scale image to size %d %d when dest_canvas has only dimensions of %d %d! Failing.\0"
                .as_ptr() as *const ::core::ffi::c_char,
            w,
            h,
            (*dst).w,
            (*dst).h,
        );
        return;
    }
    if (*srcfmt).BytesPerPixel as ::core::ffi::c_int != 1 as ::core::ffi::c_int
        && (*srcfmt).BytesPerPixel as ::core::ffi::c_int != 4 as ::core::ffi::c_int
        || (*mmfmt).BytesPerPixel as ::core::ffi::c_int != 1 as ::core::ffi::c_int
            && (*mmfmt).BytesPerPixel as ::core::ffi::c_int != 4 as ::core::ffi::c_int
        || (*(*dst).format).BytesPerPixel as ::core::ffi::c_int
            != 4 as ::core::ffi::c_int
    {
        log_add(
            log_Level::log_Warning,
            b"TFB_DrawCanvas_Rescale_Trilinear: Tried to deal with unknown BPP: %d -> %d, mipmap %d\0"
                .as_ptr() as *const ::core::ffi::c_char,
            (*srcfmt).BitsPerPixel as ::core::ffi::c_int,
            (*(*dst).format).BitsPerPixel as ::core::ffi::c_int,
            (*mmfmt).BitsPerPixel as ::core::ffi::c_int,
        );
        return;
    }
    if (*srcfmt).Amask != 0 {
        mk0 = (*srcfmt).Amask;
        ck0 = 0 as Uint32;
    } else if TFB_GetColorKey(src, &raw mut ck0) == 0 as ::core::ffi::c_int {
        mk0 = !(*srcfmt).Amask;
        ck0 &= mk0;
    }
    if (*mmfmt).Amask != 0 {
        mk1 = (*mmfmt).Amask;
        ck1 = 0 as Uint32;
    } else if TFB_GetColorKey(mm, &raw mut ck1) == 0 as ::core::ffi::c_int {
        mk1 = !(*mmfmt).Amask;
        ck1 &= mk1;
    }
    SDL_LockSurface(src);
    SDL_LockSurface(dst);
    SDL_LockSurface(mm);
    y = 0 as ::core::ffi::c_int;
    sy0 = ssy0;
    sy1 = ssy1;
    while y < h {
        let mut dst_p: *mut Uint32 = ((*dst).pixels as *mut Uint8)
            .offset((y * (*dst).pitch) as isize) as *mut Uint32;
        let py0: ::core::ffi::c_int = sy0 >> 16 as ::core::ffi::c_int;
        let py1: ::core::ffi::c_int = sy1 >> 16 as ::core::ffi::c_int;
        let mut src_a0: *mut Uint8 = ((*src).pixels as *mut Uint8)
            .offset((py0 * slen) as isize);
        let mut src_a1: *mut Uint8 = ((*mm).pixels as *mut Uint8)
            .offset((py1 * mmlen) as isize);
        let v0: Uint8 = (sy0 >> 8 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int)
            as Uint8;
        let v1: Uint8 = (sy1 >> 8 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int)
            as Uint8;
        let mut w0: [Uint8; 4] = [0; 4];
        let mut w1: [Uint8; 4] = [0; 4];
        x = 0 as ::core::ffi::c_int;
        sx0 = ssx0;
        sx1 = ssx1;
        while x < w {
            let px0: ::core::ffi::c_int = sx0 >> 16 as ::core::ffi::c_int;
            let px1: ::core::ffi::c_int = sx1 >> 16 as ::core::ffi::c_int;
            let u0: Uint8 = (sx0 >> 8 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int)
                as Uint8;
            let u1: Uint8 = (sx1 >> 8 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int)
                as Uint8;
            let mut p0: [pixel_t; 5] = [pixel_t { value: 0 }; 5];
            let mut p1: [pixel_t; 5] = [pixel_t { value: 0 }; 5];
            let mut res_a: Uint8 = 0;
            w0[0usize] = btable[(255 as ::core::ffi::c_int - u0 as ::core::ffi::c_int)
                as usize][(255 as ::core::ffi::c_int - v0 as ::core::ffi::c_int)
                as usize];
            w0[1usize] = btable[u0
                as usize][(255 as ::core::ffi::c_int - v0 as ::core::ffi::c_int)
                as usize];
            w0[2usize] = btable[(255 as ::core::ffi::c_int - u0 as ::core::ffi::c_int)
                as usize][v0 as usize];
            w0[3usize] = btable[u0 as usize][v0 as usize];
            w1[0usize] = btable[(255 as ::core::ffi::c_int - u1 as ::core::ffi::c_int)
                as usize][(255 as ::core::ffi::c_int - v1 as ::core::ffi::c_int)
                as usize];
            w1[1usize] = btable[u1
                as usize][(255 as ::core::ffi::c_int - v1 as ::core::ffi::c_int)
                as usize];
            w1[2usize] = btable[(255 as ::core::ffi::c_int - u1 as ::core::ffi::c_int)
                as usize][v1 as usize];
            w1[3usize] = btable[u1 as usize][v1 as usize];
            if px0 >= 0 as ::core::ffi::c_int
                && (px0 + 1 as ::core::ffi::c_int) < (*src).w
                && py0 >= 0 as ::core::ffi::c_int
                && (py0 + 1 as ::core::ffi::c_int) < (*src).h
            {
                let mut src_p: *mut Uint8 = src_a0.offset((px0 * sbpp) as isize);
                p0[0usize].value = scale_read_pixel(
                    src_p as *mut ::core::ffi::c_void,
                    srcfmt,
                    srcpal,
                    mk0,
                    ck0,
                );
                p0[1usize].value = scale_read_pixel(
                    src_p.offset(sbpp as isize) as *mut ::core::ffi::c_void,
                    srcfmt,
                    srcpal,
                    mk0,
                    ck0,
                );
                p0[2usize].value = scale_read_pixel(
                    src_p.offset(slen as isize) as *mut ::core::ffi::c_void,
                    srcfmt,
                    srcpal,
                    mk0,
                    ck0,
                );
                p0[3usize].value = scale_read_pixel(
                    src_p.offset(sbpp as isize).offset(slen as isize)
                        as *mut ::core::ffi::c_void,
                    srcfmt,
                    srcpal,
                    mk0,
                    ck0,
                );
            } else {
                p0[0usize].value = scale_get_pixel(src, mk0, ck0, px0, py0);
                p0[1usize].value = scale_get_pixel(
                    src,
                    mk0,
                    ck0,
                    px0 + 1 as ::core::ffi::c_int,
                    py0,
                );
                p0[2usize].value = scale_get_pixel(
                    src,
                    mk0,
                    ck0,
                    px0,
                    py0 + 1 as ::core::ffi::c_int,
                );
                p0[3usize].value = scale_get_pixel(
                    src,
                    mk0,
                    ck0,
                    px0 + 1 as ::core::ffi::c_int,
                    py0 + 1 as ::core::ffi::c_int,
                );
            }
            if px1 >= 0 as ::core::ffi::c_int
                && (px1 + 1 as ::core::ffi::c_int) < (*mm).w
                && py1 >= 0 as ::core::ffi::c_int
                && (py1 + 1 as ::core::ffi::c_int) < (*mm).h
            {
                let mut mm_p: *mut Uint8 = src_a1.offset((px1 * mmbpp) as isize);
                p1[0usize].value = scale_read_pixel(
                    mm_p as *mut ::core::ffi::c_void,
                    mmfmt,
                    srcpal,
                    mk1,
                    ck1,
                );
                p1[1usize].value = scale_read_pixel(
                    mm_p.offset(mmbpp as isize) as *mut ::core::ffi::c_void,
                    mmfmt,
                    srcpal,
                    mk1,
                    ck1,
                );
                p1[2usize].value = scale_read_pixel(
                    mm_p.offset(mmlen as isize) as *mut ::core::ffi::c_void,
                    mmfmt,
                    srcpal,
                    mk1,
                    ck1,
                );
                p1[3usize].value = scale_read_pixel(
                    mm_p.offset(mmbpp as isize).offset(mmlen as isize)
                        as *mut ::core::ffi::c_void,
                    mmfmt,
                    srcpal,
                    mk1,
                    ck1,
                );
            } else {
                p1[0usize].value = scale_get_pixel(mm, mk1, ck1, px1, py1);
                p1[1usize].value = scale_get_pixel(
                    mm,
                    mk1,
                    ck1,
                    px1 + 1 as ::core::ffi::c_int,
                    py1,
                );
                p1[2usize].value = scale_get_pixel(
                    mm,
                    mk1,
                    ck1,
                    px1,
                    py1 + 1 as ::core::ffi::c_int,
                );
                p1[3usize].value = scale_get_pixel(
                    mm,
                    mk1,
                    ck1,
                    px1 + 1 as ::core::ffi::c_int,
                    py1 + 1 as ::core::ffi::c_int,
                );
            }
            p0[4usize].c.a = dot_product_8_4(
                &raw mut p0 as *mut pixel_t,
                3 as ::core::ffi::c_int,
                &raw mut w0 as *mut Uint8,
            );
            p1[4usize].c.a = dot_product_8_4(
                &raw mut p1 as *mut pixel_t,
                3 as ::core::ffi::c_int,
                &raw mut w1 as *mut Uint8,
            );
            res_a = blend_ratio_2(p0[4usize].c.a, p1[4usize].c.a, ratio);
            if res_a as ::core::ffi::c_int <= alpha_threshold {
                *dst_p = transparent;
            } else if dst_has_alpha == 0 {
                p0[4usize].c.r = dot_product_8_4(
                    &raw mut p0 as *mut pixel_t,
                    0 as ::core::ffi::c_int,
                    &raw mut w0 as *mut Uint8,
                );
                p0[4usize].c.g = dot_product_8_4(
                    &raw mut p0 as *mut pixel_t,
                    1 as ::core::ffi::c_int,
                    &raw mut w0 as *mut Uint8,
                );
                p0[4usize].c.b = dot_product_8_4(
                    &raw mut p0 as *mut pixel_t,
                    2 as ::core::ffi::c_int,
                    &raw mut w0 as *mut Uint8,
                );
                p1[4usize].c.r = dot_product_8_4(
                    &raw mut p1 as *mut pixel_t,
                    0 as ::core::ffi::c_int,
                    &raw mut w1 as *mut Uint8,
                );
                p1[4usize].c.g = dot_product_8_4(
                    &raw mut p1 as *mut pixel_t,
                    1 as ::core::ffi::c_int,
                    &raw mut w1 as *mut Uint8,
                );
                p1[4usize].c.b = dot_product_8_4(
                    &raw mut p1 as *mut pixel_t,
                    2 as ::core::ffi::c_int,
                    &raw mut w1 as *mut Uint8,
                );
                p0[4usize].c.r = blend_ratio_2(p0[4usize].c.r, p1[4usize].c.r, ratio);
                p0[4usize].c.g = blend_ratio_2(p0[4usize].c.g, p1[4usize].c.g, ratio);
                p0[4usize].c.b = blend_ratio_2(p0[4usize].c.b, p1[4usize].c.b, ratio);
                *dst_p = ((p0[4usize].c.r as ::core::ffi::c_int)
                    << (*dstfmt).Rshift as ::core::ffi::c_int
                    | (p0[4usize].c.g as ::core::ffi::c_int)
                        << (*dstfmt).Gshift as ::core::ffi::c_int
                    | (p0[4usize].c.b as ::core::ffi::c_int)
                        << (*dstfmt).Bshift as ::core::ffi::c_int) as Uint32;
            } else {
                if p0[4usize].c.a as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                    let mut i: ::core::ffi::c_int = 0;
                    i = 0 as ::core::ffi::c_int;
                    while i < 4 as ::core::ffi::c_int {
                        if p0[i as usize].c.a as ::core::ffi::c_int
                            == 0 as ::core::ffi::c_int
                        {
                            w0[i as usize] = 0 as Uint8;
                        }
                        i += 1;
                    }
                    p0[4usize].c.r = weight_product_8_4(
                        &raw mut p0 as *mut pixel_t,
                        0 as ::core::ffi::c_int,
                        &raw mut w0 as *mut Uint8,
                    );
                    p0[4usize].c.g = weight_product_8_4(
                        &raw mut p0 as *mut pixel_t,
                        1 as ::core::ffi::c_int,
                        &raw mut w0 as *mut Uint8,
                    );
                    p0[4usize].c.b = weight_product_8_4(
                        &raw mut p0 as *mut pixel_t,
                        2 as ::core::ffi::c_int,
                        &raw mut w0 as *mut Uint8,
                    );
                }
                if p1[4usize].c.a as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                    let mut i_0: ::core::ffi::c_int = 0;
                    i_0 = 0 as ::core::ffi::c_int;
                    while i_0 < 4 as ::core::ffi::c_int {
                        if p1[i_0 as usize].c.a as ::core::ffi::c_int
                            == 0 as ::core::ffi::c_int
                        {
                            w1[i_0 as usize] = 0 as Uint8;
                        }
                        i_0 += 1;
                    }
                    p1[4usize].c.r = weight_product_8_4(
                        &raw mut p1 as *mut pixel_t,
                        0 as ::core::ffi::c_int,
                        &raw mut w1 as *mut Uint8,
                    );
                    p1[4usize].c.g = weight_product_8_4(
                        &raw mut p1 as *mut pixel_t,
                        1 as ::core::ffi::c_int,
                        &raw mut w1 as *mut Uint8,
                    );
                    p1[4usize].c.b = weight_product_8_4(
                        &raw mut p1 as *mut pixel_t,
                        2 as ::core::ffi::c_int,
                        &raw mut w1 as *mut Uint8,
                    );
                }
                if p0[4usize].c.a as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                    && p1[4usize].c.a as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                {
                    p0[4usize].c.r = blend_ratio_2(
                        p0[4usize].c.r,
                        p1[4usize].c.r,
                        ratio,
                    );
                    p0[4usize].c.g = blend_ratio_2(
                        p0[4usize].c.g,
                        p1[4usize].c.g,
                        ratio,
                    );
                    p0[4usize].c.b = blend_ratio_2(
                        p0[4usize].c.b,
                        p1[4usize].c.b,
                        ratio,
                    );
                } else if p1[4usize].c.a as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                {
                    p0[4usize].value = p1[4usize].value;
                }
                if res_a as ::core::ffi::c_int > 0xf8 as ::core::ffi::c_int {
                    res_a = 0xff as Uint8;
                }
                *dst_p = ((p0[4usize].c.r as ::core::ffi::c_int)
                    << (*dstfmt).Rshift as ::core::ffi::c_int
                    | (p0[4usize].c.g as ::core::ffi::c_int)
                        << (*dstfmt).Gshift as ::core::ffi::c_int
                    | (p0[4usize].c.b as ::core::ffi::c_int)
                        << (*dstfmt).Bshift as ::core::ffi::c_int
                    | (res_a as ::core::ffi::c_int)
                        << (*dstfmt).Ashift as ::core::ffi::c_int) as Uint32;
            }
            x += 1;
            dst_p = dst_p.offset(1);
            sx0 += fsx0;
            sx1 += fsx1;
        }
        y += 1;
        sy0 += fsy0;
        sy1 += fsy1;
    }
    SDL_UnlockSurface(mm);
    SDL_UnlockSurface(dst);
    SDL_UnlockSurface(src);
}
#[no_mangle]
pub unsafe extern "C" fn TFB_DrawCanvas_Rescale_Bilinear(
    mut src_canvas: TFB_Canvas,
    mut dst_canvas: TFB_Canvas,
    mut scale: ::core::ffi::c_int,
    mut src_hs: *mut HOT_SPOT,
    mut size: *mut EXTENT,
    mut dst_hs: *mut HOT_SPOT,
) {
    let mut src: *mut SDL_Surface = src_canvas as *mut SDL_Surface;
    let mut dst: *mut SDL_Surface = dst_canvas as *mut SDL_Surface;
    let mut srcfmt: *mut SDL_PixelFormat = (*src).format;
    let mut dstfmt: *mut SDL_PixelFormat = (*dst).format;
    let mut srcpal: *mut SDL_Color = if !(*srcfmt).palette.is_null() {
        (*(*srcfmt).palette).colors
    } else {
        ::core::ptr::null_mut::<SDL_Color>()
    };
    let sbpp: ::core::ffi::c_int = (*srcfmt).BytesPerPixel as ::core::ffi::c_int;
    let slen: ::core::ffi::c_int = (*src).pitch;
    let dst_has_alpha: ::core::ffi::c_int = ((*dstfmt).Amask != 0 as Uint32)
        as ::core::ffi::c_int;
    let mut srckey: Uint32 = 0 as Uint32;
    let mut transparent: Uint32 = 0 as Uint32;
    let alpha_threshold: ::core::ffi::c_int = if dst_has_alpha != 0 {
        0 as ::core::ffi::c_int
    } else {
        127 as ::core::ffi::c_int
    };
    let mut mk: Uint32 = 0 as Uint32;
    let mut ck: Uint32 = !(0 as ::core::ffi::c_int) as Uint32;
    let mut sx: ::core::ffi::c_int = 0;
    let mut sy: ::core::ffi::c_int = 0;
    let mut fsx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut fsy: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut ssx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut ssy: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut w: ::core::ffi::c_int = 0;
    let mut h: ::core::ffi::c_int = 0;
    TFB_GetColorKey(dst, &raw mut transparent);
    if scale > 0 as ::core::ffi::c_int {
        TFB_DrawCanvas_GetScaledExtent(
            src as TFB_Canvas,
            src_hs,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            ::core::ptr::null_mut::<HOT_SPOT>(),
            scale,
            C2Rust_Unnamed_0::TFB_SCALE_BILINEAR.0 as ::core::ffi::c_int,
            size,
            dst_hs,
        );
        w = (*size).width as ::core::ffi::c_int;
        h = (*size).height as ::core::ffi::c_int;
        fsx = ((256 as ::core::ffi::c_int) << 16 as ::core::ffi::c_int) / scale;
        fsy = ((256 as ::core::ffi::c_int) << 16 as ::core::ffi::c_int) / scale;
        ssx = (((*src_hs).x as ::core::ffi::c_int) << 16 as ::core::ffi::c_int)
            - fsx * (*dst_hs).x as ::core::ffi::c_int;
        ssy = (((*src_hs).y as ::core::ffi::c_int) << 16 as ::core::ffi::c_int)
            - fsy * (*dst_hs).y as ::core::ffi::c_int;
    } else {
        w = (*dst).w;
        h = (*dst).h;
        fsx = ((*src).w << 16 as ::core::ffi::c_int) / w;
        fsy = ((*src).h << 16 as ::core::ffi::c_int) / h;
        ssx = (((*src).w - 1 as ::core::ffi::c_int) << 16 as ::core::ffi::c_int)
            - fsx * (w - 1 as ::core::ffi::c_int) >> 1 as ::core::ffi::c_int;
        ssy = (((*src).h - 1 as ::core::ffi::c_int) << 16 as ::core::ffi::c_int)
            - fsy * (h - 1 as ::core::ffi::c_int) >> 1 as ::core::ffi::c_int;
    }
    if w > (*dst).w || h > (*dst).h {
        log_add(
            log_Level::log_Warning,
            b"TFB_DrawCanvas_Rescale_Bilinear: Tried to scale image to size %d %d when dest_canvas has only dimensions of %d %d! Failing.\0"
                .as_ptr() as *const ::core::ffi::c_char,
            w,
            h,
            (*dst).w,
            (*dst).h,
        );
        return;
    }
    if (*srcfmt).BytesPerPixel as ::core::ffi::c_int != 1 as ::core::ffi::c_int
        && (*srcfmt).BytesPerPixel as ::core::ffi::c_int != 4 as ::core::ffi::c_int
        || (*(*dst).format).BytesPerPixel as ::core::ffi::c_int
            != 4 as ::core::ffi::c_int
    {
        log_add(
            log_Level::log_Warning,
            b"TFB_DrawCanvas_Rescale_Bilinear: Tried to deal with unknown BPP: %d -> %d\0"
                .as_ptr() as *const ::core::ffi::c_char,
            (*srcfmt).BitsPerPixel as ::core::ffi::c_int,
            (*(*dst).format).BitsPerPixel as ::core::ffi::c_int,
        );
        return;
    }
    if (*srcfmt).Amask != 0 {
        mk = (*srcfmt).Amask;
        ck = 0 as Uint32;
    } else if TFB_GetColorKey(src, &raw mut srckey) == 0 as ::core::ffi::c_int {
        mk = !(*srcfmt).Amask;
        ck = srckey & mk;
    }
    SDL_LockSurface(src);
    SDL_LockSurface(dst);
    y = 0 as ::core::ffi::c_int;
    sy = ssy;
    while y < h {
        let mut dst_p: *mut Uint32 = ((*dst).pixels as *mut Uint8)
            .offset((y * (*dst).pitch) as isize) as *mut Uint32;
        let py: ::core::ffi::c_int = sy >> 16 as ::core::ffi::c_int;
        let mut src_a: *mut Uint8 = ((*src).pixels as *mut Uint8)
            .offset((py * slen) as isize);
        let v: Uint8 = (sy >> 8 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int)
            as Uint8;
        let mut weight: [Uint8; 4] = [0; 4];
        x = 0 as ::core::ffi::c_int;
        sx = ssx;
        while x < w {
            let px: ::core::ffi::c_int = sx >> 16 as ::core::ffi::c_int;
            let u: Uint8 = (sx >> 8 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int)
                as Uint8;
            let mut p: [pixel_t; 5] = [pixel_t { value: 0 }; 5];
            weight[0usize] = btable[(255 as ::core::ffi::c_int - u as ::core::ffi::c_int)
                as usize][(255 as ::core::ffi::c_int - v as ::core::ffi::c_int)
                as usize];
            weight[1usize] = btable[u
                as usize][(255 as ::core::ffi::c_int - v as ::core::ffi::c_int)
                as usize];
            weight[2usize] = btable[(255 as ::core::ffi::c_int - u as ::core::ffi::c_int)
                as usize][v as usize];
            weight[3usize] = btable[u as usize][v as usize];
            if px >= 0 as ::core::ffi::c_int && (px + 1 as ::core::ffi::c_int) < (*src).w
                && py >= 0 as ::core::ffi::c_int
                && (py + 1 as ::core::ffi::c_int) < (*src).h
            {
                let mut src_p: *mut Uint8 = src_a.offset((px * sbpp) as isize);
                p[0usize].value = scale_read_pixel(
                    src_p as *mut ::core::ffi::c_void,
                    srcfmt,
                    srcpal,
                    mk,
                    ck,
                );
                p[1usize].value = scale_read_pixel(
                    src_p.offset(sbpp as isize) as *mut ::core::ffi::c_void,
                    srcfmt,
                    srcpal,
                    mk,
                    ck,
                );
                p[2usize].value = scale_read_pixel(
                    src_p.offset(slen as isize) as *mut ::core::ffi::c_void,
                    srcfmt,
                    srcpal,
                    mk,
                    ck,
                );
                p[3usize].value = scale_read_pixel(
                    src_p.offset(sbpp as isize).offset(slen as isize)
                        as *mut ::core::ffi::c_void,
                    srcfmt,
                    srcpal,
                    mk,
                    ck,
                );
            } else {
                p[0usize].value = scale_get_pixel(src, mk, ck, px, py);
                p[1usize].value = scale_get_pixel(
                    src,
                    mk,
                    ck,
                    px + 1 as ::core::ffi::c_int,
                    py,
                );
                p[2usize].value = scale_get_pixel(
                    src,
                    mk,
                    ck,
                    px,
                    py + 1 as ::core::ffi::c_int,
                );
                p[3usize].value = scale_get_pixel(
                    src,
                    mk,
                    ck,
                    px + 1 as ::core::ffi::c_int,
                    py + 1 as ::core::ffi::c_int,
                );
            }
            p[4usize].c.a = dot_product_8_4(
                &raw mut p as *mut pixel_t,
                3 as ::core::ffi::c_int,
                &raw mut weight as *mut Uint8,
            );
            if p[4usize].c.a as ::core::ffi::c_int <= alpha_threshold {
                *dst_p = transparent;
            } else if dst_has_alpha == 0 {
                p[4usize].c.r = dot_product_8_4(
                    &raw mut p as *mut pixel_t,
                    0 as ::core::ffi::c_int,
                    &raw mut weight as *mut Uint8,
                );
                p[4usize].c.g = dot_product_8_4(
                    &raw mut p as *mut pixel_t,
                    1 as ::core::ffi::c_int,
                    &raw mut weight as *mut Uint8,
                );
                p[4usize].c.b = dot_product_8_4(
                    &raw mut p as *mut pixel_t,
                    2 as ::core::ffi::c_int,
                    &raw mut weight as *mut Uint8,
                );
                *dst_p = ((p[4usize].c.r as ::core::ffi::c_int)
                    << (*dstfmt).Rshift as ::core::ffi::c_int
                    | (p[4usize].c.g as ::core::ffi::c_int)
                        << (*dstfmt).Gshift as ::core::ffi::c_int
                    | (p[4usize].c.b as ::core::ffi::c_int)
                        << (*dstfmt).Bshift as ::core::ffi::c_int) as Uint32;
            } else {
                let mut i: ::core::ffi::c_int = 0;
                i = 0 as ::core::ffi::c_int;
                while i < 4 as ::core::ffi::c_int {
                    if p[i as usize].c.a as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                    {
                        weight[i as usize] = 0 as Uint8;
                    }
                    i += 1;
                }
                p[4usize].c.r = weight_product_8_4(
                    &raw mut p as *mut pixel_t,
                    0 as ::core::ffi::c_int,
                    &raw mut weight as *mut Uint8,
                );
                p[4usize].c.g = weight_product_8_4(
                    &raw mut p as *mut pixel_t,
                    1 as ::core::ffi::c_int,
                    &raw mut weight as *mut Uint8,
                );
                p[4usize].c.b = weight_product_8_4(
                    &raw mut p as *mut pixel_t,
                    2 as ::core::ffi::c_int,
                    &raw mut weight as *mut Uint8,
                );
                if p[4usize].c.a as ::core::ffi::c_int > 0xf8 as ::core::ffi::c_int {
                    p[4usize].c.a = 0xff as Uint8;
                }
                *dst_p = ((p[4usize].c.r as ::core::ffi::c_int)
                    << (*dstfmt).Rshift as ::core::ffi::c_int
                    | (p[4usize].c.g as ::core::ffi::c_int)
                        << (*dstfmt).Gshift as ::core::ffi::c_int
                    | (p[4usize].c.b as ::core::ffi::c_int)
                        << (*dstfmt).Bshift as ::core::ffi::c_int
                    | (p[4usize].c.a as ::core::ffi::c_int)
                        << (*dstfmt).Ashift as ::core::ffi::c_int) as Uint32;
            }
            x += 1;
            dst_p = dst_p.offset(1);
            sx += fsx;
        }
        y += 1;
        sy += fsy;
    }
    SDL_UnlockSurface(dst);
    SDL_UnlockSurface(src);
}
#[no_mangle]
pub unsafe extern "C" fn TFB_DrawCanvas_Lock(mut canvas: TFB_Canvas) {
    let mut surf: *mut SDL_Surface = canvas as *mut SDL_Surface;
    SDL_LockSurface(surf);
}
#[no_mangle]
pub unsafe extern "C" fn TFB_DrawCanvas_Unlock(mut canvas: TFB_Canvas) {
    let mut surf: *mut SDL_Surface = canvas as *mut SDL_Surface;
    SDL_UnlockSurface(surf);
}
#[no_mangle]
pub unsafe extern "C" fn TFB_DrawCanvas_GetScreenFormat(mut fmt: *mut TFB_PixelFormat) {
    let mut sdl: *mut SDL_PixelFormat = (*SDL_Screen).format;
    if !(*sdl).palette.is_null() {
        log_add(
            log_Level::log_Warning,
            b"TFB_DrawCanvas_GetScreenFormat() WARNING:Paletted display format will be slow\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        (*fmt).BitsPerPixel = 32 as ::core::ffi::c_int;
        (*fmt).Rmask = 0xff as DWORD;
        (*fmt).Gmask = 0xff00 as DWORD;
        (*fmt).Bmask = 0xff0000 as ::core::ffi::c_int as DWORD;
        (*fmt).Amask = 0xff000000 as ::core::ffi::c_uint as DWORD;
    } else {
        (*fmt).BitsPerPixel = (*sdl).BitsPerPixel as ::core::ffi::c_int;
        (*fmt).Rmask = (*sdl).Rmask as DWORD;
        (*fmt).Gmask = (*sdl).Gmask as DWORD;
        (*fmt).Bmask = (*sdl).Bmask as DWORD;
        (*fmt).Amask = (*sdl).Amask as DWORD;
    };
}
#[no_mangle]
pub unsafe extern "C" fn TFB_DrawCanvas_GetStride(
    mut canvas: TFB_Canvas,
) -> ::core::ffi::c_int {
    let mut surf: *mut SDL_Surface = canvas as *mut SDL_Surface;
    return (*surf).pitch;
}
#[no_mangle]
pub unsafe extern "C" fn TFB_DrawCanvas_GetLine(
    mut canvas: TFB_Canvas,
    mut line: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_void {
    let mut surf: *mut SDL_Surface = canvas as *mut SDL_Surface;
    return ((*surf).pixels as *mut uint8).offset(((*surf).pitch * line) as isize)
        as *mut ::core::ffi::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn TFB_DrawCanvas_GetPixel(
    mut canvas: TFB_Canvas,
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
) -> Color {
    let mut surf: *mut SDL_Surface = canvas as *mut SDL_Surface;
    let mut pixel: Uint32 = 0;
    let mut getpixel: GetPixelFn = None;
    let mut c: Color = Color {
        r: 0 as BYTE,
        g: 0 as BYTE,
        b: 0 as BYTE,
        a: 0 as BYTE,
    };
    if x < 0 as ::core::ffi::c_int || x >= (*surf).w || y < 0 as ::core::ffi::c_int
        || y >= (*surf).h
    {
        return c;
    }
    SDL_LockSurface(surf);
    getpixel = getpixel_for(surf);
    pixel = Some(getpixel.expect("non-null function pointer"))
        .expect("non-null function pointer")(surf, x, y);
    SDL_GetRGBA(
        pixel,
        (*surf).format,
        &raw mut c.r,
        &raw mut c.g,
        &raw mut c.b,
        &raw mut c.a,
    );
    SDL_UnlockSurface(surf);
    return c;
}
#[no_mangle]
pub unsafe extern "C" fn TFB_DrawCanvas_Rotate(
    mut src_canvas: TFB_Canvas,
    mut dst_canvas: TFB_Canvas,
    mut angle: ::core::ffi::c_int,
    mut size: EXTENT,
) {
    let mut src: *mut SDL_Surface = src_canvas as *mut SDL_Surface;
    let mut dst: *mut SDL_Surface = dst_canvas as *mut SDL_Surface;
    let mut ret: ::core::ffi::c_int = 0;
    let mut color: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    if size.width as ::core::ffi::c_int > (*dst).w
        || size.height as ::core::ffi::c_int > (*dst).h
    {
        log_add(
            log_Level::log_Warning,
            b"TFB_DrawCanvas_Rotate: Tried to rotate image to size %d %d when dst_canvas has only dimensions of %d %d! Failing.\0"
                .as_ptr() as *const ::core::ffi::c_char,
            size.width as ::core::ffi::c_int,
            size.height as ::core::ffi::c_int,
            (*dst).w,
            (*dst).h,
        );
        return;
    }
    if TFB_DrawCanvas_GetTransparentColor(src as TFB_Canvas, &raw mut color).0 != 0 {
        TFB_DrawCanvas_SetTransparentColor(dst as TFB_Canvas, color, BOOLEAN::FALSE);
        SDL_FillRect(
            dst,
            ::core::ptr::null::<SDL_Rect>(),
            SDL_MapRGBA((*dst).format, color.r, color.g, color.b, 0 as Uint8),
        );
    }
    ret = rotateSurface(
        src,
        dst,
        angle as ::core::ffi::c_double,
        0 as ::core::ffi::c_int,
    );
    if ret != 0 as ::core::ffi::c_int {
        log_add(
            log_Level::log_Warning,
            b"TFB_DrawCanvas_Rotate: WARNING: actual rotation func returned failure\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn TFB_DrawCanvas_GetRotatedExtent(
    mut src_canvas: TFB_Canvas,
    mut angle: ::core::ffi::c_int,
    mut size: *mut EXTENT,
) {
    let mut dstw: ::core::ffi::c_int = 0;
    let mut dsth: ::core::ffi::c_int = 0;
    let mut src: *mut SDL_Surface = src_canvas as *mut SDL_Surface;
    rotozoomSurfaceSize(
        (*src).w,
        (*src).h,
        angle as ::core::ffi::c_double,
        1 as ::core::ffi::c_int as ::core::ffi::c_double,
        &raw mut dstw,
        &raw mut dsth,
    );
    (*size).height = dsth as COORD;
    (*size).width = dstw as COORD;
}
#[no_mangle]
pub unsafe extern "C" fn TFB_DrawCanvas_CopyRect(
    mut source: TFB_Canvas,
    mut srcRect: *const RECT,
    mut target: TFB_Canvas,
    mut dstPt: POINT,
) {
    let mut sourceRect: SDL_Rect = SDL_Rect { x: 0, y: 0, w: 0, h: 0 };
    let mut targetRect: SDL_Rect = SDL_Rect { x: 0, y: 0, w: 0, h: 0 };
    if source.is_null() || target.is_null() {
        log_add(
            log_Level::log_Warning,
            b"ERROR: TFB_DrawCanvas_CopyRect passed null canvas ptr\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    sourceRect.x = (*srcRect).corner.x as ::core::ffi::c_int;
    sourceRect.y = (*srcRect).corner.y as ::core::ffi::c_int;
    sourceRect.w = (*srcRect).extent.width as ::core::ffi::c_int;
    sourceRect.h = (*srcRect).extent.height as ::core::ffi::c_int;
    targetRect.x = dstPt.x as ::core::ffi::c_int;
    targetRect.y = dstPt.y as ::core::ffi::c_int;
    targetRect.w = (*srcRect).extent.width as ::core::ffi::c_int;
    targetRect.h = (*srcRect).extent.height as ::core::ffi::c_int;
    SDL_UpperBlit(
        source as *mut SDL_Surface,
        &raw mut sourceRect,
        target as *mut SDL_Surface,
        &raw mut targetRect,
    );
}
#[no_mangle]
pub unsafe extern "C" fn TFB_DrawCanvas_SetClipRect(
    mut canvas: TFB_Canvas,
    mut clipRect: *const RECT,
) {
    if canvas.is_null() {
        log_add(
            log_Level::log_Warning,
            b"ERROR: TFB_DrawCanvas_SetClipRect passed null canvas ptr\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if clipRect.is_null() {
        SDL_SetClipRect(canvas as *mut SDL_Surface, ::core::ptr::null::<SDL_Rect>());
    } else {
        let mut r: SDL_Rect = SDL_Rect { x: 0, y: 0, w: 0, h: 0 };
        r.x = (*clipRect).corner.x as ::core::ffi::c_int;
        r.y = (*clipRect).corner.y as ::core::ffi::c_int;
        r.w = (*clipRect).extent.width as ::core::ffi::c_int;
        r.h = (*clipRect).extent.height as ::core::ffi::c_int;
        SDL_SetClipRect(canvas as *mut SDL_Surface, &raw mut r);
    };
}
#[no_mangle]
pub unsafe extern "C" fn TFB_DrawCanvas_Intersect(
    mut canvas1: TFB_Canvas,
    mut c1org: POINT,
    mut canvas2: TFB_Canvas,
    mut c2org: POINT,
    mut interRect: *const RECT,
) -> BOOLEAN {
    let mut ret: BOOLEAN = BOOLEAN::FALSE;
    let mut surf1: *mut SDL_Surface = canvas1 as *mut SDL_Surface;
    let mut surf2: *mut SDL_Surface = canvas2 as *mut SDL_Surface;
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut s1key: Uint32 = 0;
    let mut s2key: Uint32 = 0;
    let mut s1mask: Uint32 = 0;
    let mut s2mask: Uint32 = 0;
    let mut getpixel1: GetPixelFn = None;
    let mut getpixel2: GetPixelFn = None;
    SDL_LockSurface(surf1);
    SDL_LockSurface(surf2);
    getpixel1 = getpixel_for(surf1);
    getpixel2 = getpixel_for(surf2);
    if (*(*surf1).format).Amask != 0 {
        s1mask = (*(*surf1).format).Amask;
        s1key = 0 as Uint32;
    } else {
        let mut colorkey: Uint32 = 0 as Uint32;
        TFB_GetColorKey(surf1, &raw mut colorkey);
        s1mask = !(*(*surf1).format).Amask;
        s1key = colorkey & s1mask;
    }
    if (*(*surf2).format).Amask != 0 {
        s2mask = (*(*surf2).format).Amask;
        s2key = 0 as Uint32;
    } else {
        let mut colorkey_0: Uint32 = 0 as Uint32;
        TFB_GetColorKey(surf2, &raw mut colorkey_0);
        s2mask = !(*(*surf2).format).Amask;
        s2key = colorkey_0 & s2mask;
    }
    c1org.x = ((*interRect).corner.x as ::core::ffi::c_int
        - c1org.x as ::core::ffi::c_int) as COORD;
    c1org.y = ((*interRect).corner.y as ::core::ffi::c_int
        - c1org.y as ::core::ffi::c_int) as COORD;
    c2org.x = ((*interRect).corner.x as ::core::ffi::c_int
        - c2org.x as ::core::ffi::c_int) as COORD;
    c2org.y = ((*interRect).corner.y as ::core::ffi::c_int
        - c2org.y as ::core::ffi::c_int) as COORD;
    y = 0 as ::core::ffi::c_int;
    while y < (*interRect).extent.height as ::core::ffi::c_int {
        x = 0 as ::core::ffi::c_int;
        while x < (*interRect).extent.width as ::core::ffi::c_int {
            let mut p1: Uint32 = getpixel1
                .expect(
                    "non-null function pointer",
                )(
                surf1,
                x + c1org.x as ::core::ffi::c_int,
                y + c1org.y as ::core::ffi::c_int,
            ) & s1mask;
            let mut p2: Uint32 = getpixel2
                .expect(
                    "non-null function pointer",
                )(
                surf2,
                x + c2org.x as ::core::ffi::c_int,
                y + c2org.y as ::core::ffi::c_int,
            ) & s2mask;
            if p1 != s1key && p2 != s2key {
                ret = BOOLEAN::TRUE;
                break;
            } else {
                x += 1;
            }
        }
        y += 1;
    }
    SDL_UnlockSurface(surf2);
    SDL_UnlockSurface(surf1);
    return ret;
}
unsafe extern "C" fn TFB_DrawCanvas_TransferColors(
    mut canvas: TFB_Canvas,
    mut write: BOOLEAN,
    mut pixels: *mut Color,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
) -> BOOLEAN {
    let mut surf: *mut SDL_Surface = canvas as *mut SDL_Surface;
    let mut fmt: *mut SDL_PixelFormat = ::core::ptr::null_mut::<SDL_PixelFormat>();
    let mut getpix: GetPixelFn = None;
    let mut putpix: PutPixelFn = None;
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut w: ::core::ffi::c_int = 0;
    let mut h: ::core::ffi::c_int = 0;
    if canvas.is_null() {
        log_add(
            log_Level::log_Warning,
            b"ERROR: TFB_DrawCanvas_TransferColors passed null canvas\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return BOOLEAN::FALSE;
    }
    fmt = (*surf).format;
    getpix = getpixel_for(surf);
    putpix = putpixel_for(surf);
    w = if width < (*surf).w { width } else { (*surf).w };
    h = if height < (*surf).h { height } else { (*surf).h };
    SDL_LockSurface(surf);
    y = 0 as ::core::ffi::c_int;
    while y < h {
        let mut c: *mut Color = pixels.offset((y * width) as isize);
        x = 0 as ::core::ffi::c_int;
        while x < w {
            if write.0 != 0 {
                let mut p: Uint32 = SDL_MapRGBA(fmt, (*c).r, (*c).g, (*c).b, (*c).a);
                putpix.expect("non-null function pointer")(surf, x, y, p);
            } else {
                let mut p_0: Uint32 = getpix
                    .expect("non-null function pointer")(surf, x, y);
                SDL_GetRGBA(
                    p_0,
                    fmt,
                    &raw mut (*c).r,
                    &raw mut (*c).g,
                    &raw mut (*c).b,
                    &raw mut (*c).a,
                );
            }
            x += 1;
            c = c.offset(1);
        }
        y += 1;
    }
    SDL_UnlockSurface(surf);
    return BOOLEAN::TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn TFB_DrawCanvas_GetPixelColors(
    mut canvas: TFB_Canvas,
    mut pixels: *mut Color,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
) -> BOOLEAN {
    return TFB_DrawCanvas_TransferColors(canvas, BOOLEAN::FALSE, pixels, width, height);
}
#[no_mangle]
pub unsafe extern "C" fn TFB_DrawCanvas_SetPixelColors(
    mut canvas: TFB_Canvas,
    mut pixels: *const Color,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
) -> BOOLEAN {
    return TFB_DrawCanvas_TransferColors(
        canvas,
        BOOLEAN::TRUE,
        pixels as *mut Color,
        width,
        height,
    );
}
unsafe extern "C" fn TFB_DrawCanvas_TransferIndexes(
    mut canvas: TFB_Canvas,
    mut write: BOOLEAN,
    mut data: *mut BYTE,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
) -> BOOLEAN {
    let mut surf: *mut SDL_Surface = canvas as *mut SDL_Surface;
    let mut fmt: *const SDL_PixelFormat = ::core::ptr::null::<SDL_PixelFormat>();
    let mut y: ::core::ffi::c_int = 0;
    let mut w: ::core::ffi::c_int = 0;
    let mut h: ::core::ffi::c_int = 0;
    if canvas.is_null() {
        log_add(
            log_Level::log_Warning,
            b"ERROR: TFB_DrawCanvas_TransferIndexes passed null canvas\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return BOOLEAN::FALSE;
    }
    fmt = (*surf).format;
    if TFB_DrawCanvas_IsPaletted(canvas).0 == 0
        || (*fmt).BitsPerPixel as ::core::ffi::c_int != 8 as ::core::ffi::c_int
    {
        log_add(
            log_Level::log_Warning,
            b"ERROR: TFB_DrawCanvas_TransferIndexes unimplemeted function: not an 8bpp indexed canvas\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        return BOOLEAN::FALSE;
    }
    w = if width < (*surf).w { width } else { (*surf).w };
    h = if height < (*surf).h { height } else { (*surf).h };
    SDL_LockSurface(surf);
    y = 0 as ::core::ffi::c_int;
    while y < h {
        let mut surf_p: *mut Uint8 = ((*surf).pixels as *mut Uint8)
            .offset((y * (*surf).pitch) as isize);
        let mut data_p: *mut BYTE = data.offset((y * width) as isize);
        if write.0 != 0 {
            memcpy(
                surf_p as *mut ::core::ffi::c_void,
                data_p as *const ::core::ffi::c_void,
                (w as size_t).wrapping_mul(::core::mem::size_of::<BYTE>()),
            );
        } else {
            memcpy(
                data_p as *mut ::core::ffi::c_void,
                surf_p as *const ::core::ffi::c_void,
                (w as size_t).wrapping_mul(::core::mem::size_of::<BYTE>()),
            );
        }
        y += 1;
    }
    SDL_UnlockSurface(surf);
    return BOOLEAN::TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn TFB_DrawCanvas_GetPixelIndexes(
    mut canvas: TFB_Canvas,
    mut data: *mut BYTE,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
) -> BOOLEAN {
    return TFB_DrawCanvas_TransferIndexes(canvas, BOOLEAN::FALSE, data, width, height);
}
#[no_mangle]
pub unsafe extern "C" fn TFB_DrawCanvas_SetPixelIndexes(
    mut canvas: TFB_Canvas,
    mut data: *const BYTE,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
) -> BOOLEAN {
    return TFB_DrawCanvas_TransferIndexes(
        canvas,
        BOOLEAN::TRUE,
        data as *mut BYTE,
        width,
        height,
    );
}
