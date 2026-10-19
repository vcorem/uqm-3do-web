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

// Rust translation of src/libs/graphics/sdl/sdl2_common.c.
extern "C" {
    pub type SDL_BlitMap;
    fn atexit(_: Option<unsafe extern "C" fn() -> ()>) -> ::core::ffi::c_int;
    fn exit(_: ::core::ffi::c_int) -> !;
    fn SDL_GetError() -> *const ::core::ffi::c_char;
    fn SDL_SetPaletteColors(
        palette: *mut SDL_Palette,
        colors: *const SDL_Color,
        firstcolor: ::core::ffi::c_int,
        ncolors: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn SDL_SetSurfaceRLE(
        surface: *mut SDL_Surface,
        flag: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn SDL_SetColorKey(
        surface: *mut SDL_Surface,
        flag: ::core::ffi::c_int,
        key: Uint32,
    ) -> ::core::ffi::c_int;
    fn SDL_GetColorKey(
        surface: *mut SDL_Surface,
        key: *mut Uint32,
    ) -> ::core::ffi::c_int;
    fn SDL_SetSurfaceAlphaMod(
        surface: *mut SDL_Surface,
        alpha: Uint8,
    ) -> ::core::ffi::c_int;
    fn SDL_GetSurfaceAlphaMod(
        surface: *mut SDL_Surface,
        alpha: *mut Uint8,
    ) -> ::core::ffi::c_int;
    fn SDL_SetSurfaceBlendMode(
        surface: *mut SDL_Surface,
        blendMode: SDL_BlendMode,
    ) -> ::core::ffi::c_int;
    fn SDL_GetSurfaceBlendMode(
        surface: *mut SDL_Surface,
        blendMode: *mut SDL_BlendMode,
    ) -> ::core::ffi::c_int;
    fn SDL_ShowCursor(toggle: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn SDL_GetVersion(ver: *mut SDL_version);
    fn SDL_Init(flags: Uint32) -> ::core::ffi::c_int;
    fn SDL_Quit();
    static mut GfxFlags: ::core::ffi::c_int;
    static mut ScreenWidthActual: ::core::ffi::c_int;
    static mut ScreenHeightActual: ::core::ffi::c_int;
    static mut GraphicsDriver: ::core::ffi::c_int;
    fn log_add(_: log_Level, fmt: *const ::core::ffi::c_char, ...);
    fn TFB_Pure_ConfigureVideo(
        driver: ::core::ffi::c_int,
        flags: ::core::ffi::c_int,
        width: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
        togglefullscreen: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}
pub type uint8_t = ::core::ffi::c_uchar;
pub type uint32_t = ::core::ffi::c_uint;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed {
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct SDL_BlendMode(pub ::core::ffi::c_uint);
impl SDL_BlendMode {
    pub const SDL_BLENDMODE_NONE: Self = Self(0);
    pub const SDL_BLENDMODE_BLEND: Self = Self(1);
    pub const SDL_BLENDMODE_ADD: Self = Self(2);
    pub const SDL_BLENDMODE_MOD: Self = Self(4);
    pub const SDL_BLENDMODE_MUL: Self = Self(8);
    pub const SDL_BLENDMODE_INVALID: Self = Self(2147483647);
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_version {
    pub major: Uint8,
    pub minor: Uint8,
    pub patch: Uint8,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_0 {
    pub const TFB_GFXDRIVER_SDL_OPENGL: Self = Self(0);
    pub const TFB_GFXDRIVER_SDL_PURE: Self = Self(1);
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
#[no_mangle]
pub unsafe extern "C" fn TFB_PreInit() {
    let mut compiled: SDL_version = SDL_version {
        major: 0,
        minor: 0,
        patch: 0,
    };
    let mut linked: SDL_version = SDL_version {
        major: 0,
        minor: 0,
        patch: 0,
    };
    compiled.major = 2 as Uint8;
    compiled.minor = 24 as Uint8;
    compiled.patch = 2 as Uint8;
    SDL_GetVersion(&raw mut linked);
    log_add(
        log_Level::log_Info,
        b"Initializing base SDL functionality.\0".as_ptr() as *const ::core::ffi::c_char,
    );
    log_add(
        log_Level::log_Info,
        b"Using SDL version %d.%d.%d (compiled with %d.%d.%d)\0".as_ptr()
            as *const ::core::ffi::c_char,
        linked.major as ::core::ffi::c_int,
        linked.minor as ::core::ffi::c_int,
        linked.patch as ::core::ffi::c_int,
        compiled.major as ::core::ffi::c_int,
        compiled.minor as ::core::ffi::c_int,
        compiled.patch as ::core::ffi::c_int,
    );
    if SDL_Init(0x20 as Uint32) == -1 as ::core::ffi::c_int {
        log_add(
            log_Level::log_Fatal,
            b"Could not initialize SDL: %s.\0".as_ptr() as *const ::core::ffi::c_char,
            SDL_GetError(),
        );
        exit(1 as ::core::ffi::c_int);
    }
    atexit(Some(TFB_PreQuit as unsafe extern "C" fn() -> ()));
}
unsafe extern "C" fn TFB_PreQuit() {
    SDL_Quit();
}
#[no_mangle]
pub unsafe extern "C" fn TFB_ReInitGraphics(
    mut driver: ::core::ffi::c_int,
    mut flags: ::core::ffi::c_int,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut result: ::core::ffi::c_int = 0;
    let mut togglefullscreen: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if GfxFlags == flags ^ (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
        && driver == GraphicsDriver && width == ScreenWidthActual
        && height == ScreenHeightActual
    {
        togglefullscreen = 1 as ::core::ffi::c_int;
    }
    GfxFlags = flags;
    result = TFB_Pure_ConfigureVideo(
        C2Rust_Unnamed_0::TFB_GFXDRIVER_SDL_PURE.0 as ::core::ffi::c_int,
        flags,
        width,
        height,
        togglefullscreen,
    );
    if flags & (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int != 0 {
        SDL_ShowCursor(0 as ::core::ffi::c_int);
    } else {
        SDL_ShowCursor(1 as ::core::ffi::c_int);
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn TFB_SetGamma(mut gamma: ::core::ffi::c_float) -> bool {
    log_add(
        log_Level::log_Warning,
        b"Custom gamma correction is not available in the SDL2 engine.\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    return false;
}
#[no_mangle]
pub unsafe extern "C" fn TFB_HasSurfaceAlphaMod(
    mut surface: *mut SDL_Surface,
) -> ::core::ffi::c_int {
    let mut blend_mode: SDL_BlendMode = SDL_BlendMode::SDL_BLENDMODE_NONE;
    if surface.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if SDL_GetSurfaceBlendMode(surface, &raw mut blend_mode) != 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    return (blend_mode.0 == SDL_BlendMode::SDL_BLENDMODE_BLEND.0) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn TFB_GetSurfaceAlphaMod(
    mut surface: *mut SDL_Surface,
    mut alpha: *mut Uint8,
) -> ::core::ffi::c_int {
    let mut blend_mode: SDL_BlendMode = SDL_BlendMode::SDL_BLENDMODE_NONE;
    if surface.is_null() || alpha.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    if SDL_GetSurfaceBlendMode(surface, &raw mut blend_mode) == 0 as ::core::ffi::c_int {
        if blend_mode.0 == SDL_BlendMode::SDL_BLENDMODE_BLEND.0 {
            return SDL_GetSurfaceAlphaMod(surface, alpha);
        }
    }
    *alpha = 255 as Uint8;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn TFB_SetSurfaceAlphaMod(
    mut surface: *mut SDL_Surface,
    mut alpha: Uint8,
) -> ::core::ffi::c_int {
    let mut result: ::core::ffi::c_int = 0;
    if surface.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    result = SDL_SetSurfaceBlendMode(surface, SDL_BlendMode::SDL_BLENDMODE_BLEND);
    if result == 0 as ::core::ffi::c_int {
        result = SDL_SetSurfaceAlphaMod(surface, alpha);
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn TFB_DisableSurfaceAlphaMod(
    mut surface: *mut SDL_Surface,
) -> ::core::ffi::c_int {
    if surface.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    SDL_SetSurfaceAlphaMod(surface, 255 as Uint8);
    return SDL_SetSurfaceBlendMode(surface, SDL_BlendMode::SDL_BLENDMODE_NONE);
}
#[no_mangle]
pub unsafe extern "C" fn TFB_GetColorKey(
    mut surface: *mut SDL_Surface,
    mut key: *mut Uint32,
) -> ::core::ffi::c_int {
    if surface.is_null() || key.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    return SDL_GetColorKey(surface, key);
}
#[no_mangle]
pub unsafe extern "C" fn TFB_SetColorKey(
    mut surface: *mut SDL_Surface,
    mut key: Uint32,
    mut rleaccel: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if surface.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    SDL_SetSurfaceRLE(surface, rleaccel);
    return SDL_SetColorKey(
        surface,
        C2Rust_Unnamed::SDL_TRUE.0 as ::core::ffi::c_int,
        key,
    );
}
#[no_mangle]
pub unsafe extern "C" fn TFB_DisableColorKey(
    mut surface: *mut SDL_Surface,
) -> ::core::ffi::c_int {
    if surface.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    return SDL_SetColorKey(
        surface,
        C2Rust_Unnamed::SDL_FALSE.0 as ::core::ffi::c_int,
        0 as Uint32,
    );
}
#[no_mangle]
pub unsafe extern "C" fn TFB_SetColors(
    mut surface: *mut SDL_Surface,
    mut colors: *mut SDL_Color,
    mut firstcolor: ::core::ffi::c_int,
    mut ncolors: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if surface.is_null() || colors.is_null() || (*surface).format.is_null()
        || (*(*surface).format).palette.is_null()
    {
        return 0 as ::core::ffi::c_int;
    }
    if SDL_SetPaletteColors((*(*surface).format).palette, colors, firstcolor, ncolors)
        == 0 as ::core::ffi::c_int
    {
        return 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn TFB_SupportsHardwareScaling() -> ::core::ffi::c_int {
    return 1 as ::core::ffi::c_int;
}
