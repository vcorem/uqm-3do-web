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

// Rust translation of src/libs/graphics/sdl/sdl2_pure.c.
extern "C" {
    pub type SDL_BlitMap;
    pub type SDL_Window;
    pub type SDL_Renderer;
    pub type SDL_Texture;
    fn exit(_: ::core::ffi::c_int) -> !;
    fn sprintf(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn strcmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn SDL_GetError() -> *const ::core::ffi::c_char;
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
    fn SDL_CreateWindow(
        title: *const ::core::ffi::c_char,
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
        w: ::core::ffi::c_int,
        h: ::core::ffi::c_int,
        flags: Uint32,
    ) -> *mut SDL_Window;
    fn SDL_SetWindowSize(
        window_0: *mut SDL_Window,
        w: ::core::ffi::c_int,
        h: ::core::ffi::c_int,
    );
    fn SDL_SetWindowFullscreen(
        window_0: *mut SDL_Window,
        flags: Uint32,
    ) -> ::core::ffi::c_int;
    fn SDL_DestroyWindow(window_0: *mut SDL_Window);
    fn SDL_SetHint(
        name: *const ::core::ffi::c_char,
        value: *const ::core::ffi::c_char,
    ) -> SDL_bool;
    fn SDL_GetNumRenderDrivers() -> ::core::ffi::c_int;
    fn SDL_GetRenderDriverInfo(
        index: ::core::ffi::c_int,
        info: *mut SDL_RendererInfo,
    ) -> ::core::ffi::c_int;
    fn SDL_CreateRenderer(
        window_0: *mut SDL_Window,
        index: ::core::ffi::c_int,
        flags: Uint32,
    ) -> *mut SDL_Renderer;
    fn SDL_GetRendererInfo(
        renderer_0: *mut SDL_Renderer,
        info: *mut SDL_RendererInfo,
    ) -> ::core::ffi::c_int;
    fn SDL_CreateTexture(
        renderer_0: *mut SDL_Renderer,
        format: Uint32,
        access: ::core::ffi::c_int,
        w: ::core::ffi::c_int,
        h: ::core::ffi::c_int,
    ) -> *mut SDL_Texture;
    fn SDL_SetTextureAlphaMod(
        texture: *mut SDL_Texture,
        alpha: Uint8,
    ) -> ::core::ffi::c_int;
    fn SDL_SetTextureBlendMode(
        texture: *mut SDL_Texture,
        blendMode: SDL_BlendMode,
    ) -> ::core::ffi::c_int;
    fn SDL_UpdateTexture(
        texture: *mut SDL_Texture,
        rect: *const SDL_Rect,
        pixels: *const ::core::ffi::c_void,
        pitch: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn SDL_RenderSetLogicalSize(
        renderer_0: *mut SDL_Renderer,
        w: ::core::ffi::c_int,
        h: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn SDL_SetRenderDrawColor(
        renderer_0: *mut SDL_Renderer,
        r: Uint8,
        g: Uint8,
        b: Uint8,
        a: Uint8,
    ) -> ::core::ffi::c_int;
    fn SDL_SetRenderDrawBlendMode(
        renderer_0: *mut SDL_Renderer,
        blendMode: SDL_BlendMode,
    ) -> ::core::ffi::c_int;
    fn SDL_RenderClear(renderer_0: *mut SDL_Renderer) -> ::core::ffi::c_int;
    fn SDL_RenderDrawLine(
        renderer_0: *mut SDL_Renderer,
        x1: ::core::ffi::c_int,
        y1: ::core::ffi::c_int,
        x2: ::core::ffi::c_int,
        y2: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn SDL_RenderFillRect(
        renderer_0: *mut SDL_Renderer,
        rect: *const SDL_Rect,
    ) -> ::core::ffi::c_int;
    fn SDL_RenderCopy(
        renderer_0: *mut SDL_Renderer,
        texture: *mut SDL_Texture,
        srcrect: *const SDL_Rect,
        dstrect: *const SDL_Rect,
    ) -> ::core::ffi::c_int;
    fn SDL_RenderPresent(renderer_0: *mut SDL_Renderer);
    fn SDL_DestroyTexture(texture: *mut SDL_Texture);
    fn SDL_DestroyRenderer(renderer_0: *mut SDL_Renderer);
    static mut GfxFlags: ::core::ffi::c_int;
    static mut ScreenWidth: ::core::ffi::c_int;
    static mut ScreenHeight: ::core::ffi::c_int;
    static mut ScreenWidthActual: ::core::ffi::c_int;
    static mut ScreenHeightActual: ::core::ffi::c_int;
    static mut GraphicsDriver: ::core::ffi::c_int;
    fn Scale_Init();
    fn Scale_PrepPlatform(
        flags: ::core::ffi::c_int,
        fmt: *const SDL_PixelFormat,
    ) -> TFB_ScaleFunc;
    static mut TFB_BBox: TFB_BoundingBox;
    static mut graphics_backend: *mut TFB_GRAPHICS_BACKEND;
    static mut SDL_Screen: *mut SDL_Surface;
    static mut TransitionScreen: *mut SDL_Surface;
    static mut SDL_Screens: [*mut SDL_Surface; 3];
    static mut format_conv_surf: *mut SDL_Surface;
    fn log_add(_: log_Level, fmt: *const ::core::ffi::c_char, ...);
}
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed {
    pub const SDL_PIXELFORMAT_UNKNOWN: Self = Self(0);
    pub const SDL_PIXELFORMAT_INDEX1LSB: Self = Self(286261504);
    pub const SDL_PIXELFORMAT_INDEX1MSB: Self = Self(287310080);
    pub const SDL_PIXELFORMAT_INDEX4LSB: Self = Self(303039488);
    pub const SDL_PIXELFORMAT_INDEX4MSB: Self = Self(304088064);
    pub const SDL_PIXELFORMAT_INDEX8: Self = Self(318769153);
    pub const SDL_PIXELFORMAT_RGB332: Self = Self(336660481);
    pub const SDL_PIXELFORMAT_XRGB4444: Self = Self(353504258);
    pub const SDL_PIXELFORMAT_RGB444: Self = Self(353504258);
    pub const SDL_PIXELFORMAT_XBGR4444: Self = Self(357698562);
    pub const SDL_PIXELFORMAT_BGR444: Self = Self(357698562);
    pub const SDL_PIXELFORMAT_XRGB1555: Self = Self(353570562);
    pub const SDL_PIXELFORMAT_RGB555: Self = Self(353570562);
    pub const SDL_PIXELFORMAT_XBGR1555: Self = Self(357764866);
    pub const SDL_PIXELFORMAT_BGR555: Self = Self(357764866);
    pub const SDL_PIXELFORMAT_ARGB4444: Self = Self(355602434);
    pub const SDL_PIXELFORMAT_RGBA4444: Self = Self(356651010);
    pub const SDL_PIXELFORMAT_ABGR4444: Self = Self(359796738);
    pub const SDL_PIXELFORMAT_BGRA4444: Self = Self(360845314);
    pub const SDL_PIXELFORMAT_ARGB1555: Self = Self(355667970);
    pub const SDL_PIXELFORMAT_RGBA5551: Self = Self(356782082);
    pub const SDL_PIXELFORMAT_ABGR1555: Self = Self(359862274);
    pub const SDL_PIXELFORMAT_BGRA5551: Self = Self(360976386);
    pub const SDL_PIXELFORMAT_RGB565: Self = Self(353701890);
    pub const SDL_PIXELFORMAT_BGR565: Self = Self(357896194);
    pub const SDL_PIXELFORMAT_RGB24: Self = Self(386930691);
    pub const SDL_PIXELFORMAT_BGR24: Self = Self(390076419);
    pub const SDL_PIXELFORMAT_XRGB8888: Self = Self(370546692);
    pub const SDL_PIXELFORMAT_RGB888: Self = Self(370546692);
    pub const SDL_PIXELFORMAT_RGBX8888: Self = Self(371595268);
    pub const SDL_PIXELFORMAT_XBGR8888: Self = Self(374740996);
    pub const SDL_PIXELFORMAT_BGR888: Self = Self(374740996);
    pub const SDL_PIXELFORMAT_BGRX8888: Self = Self(375789572);
    pub const SDL_PIXELFORMAT_ARGB8888: Self = Self(372645892);
    pub const SDL_PIXELFORMAT_RGBA8888: Self = Self(373694468);
    pub const SDL_PIXELFORMAT_ABGR8888: Self = Self(376840196);
    pub const SDL_PIXELFORMAT_BGRA8888: Self = Self(377888772);
    pub const SDL_PIXELFORMAT_ARGB2101010: Self = Self(372711428);
    pub const SDL_PIXELFORMAT_RGBA32: Self = Self(376840196);
    pub const SDL_PIXELFORMAT_ARGB32: Self = Self(377888772);
    pub const SDL_PIXELFORMAT_BGRA32: Self = Self(372645892);
    pub const SDL_PIXELFORMAT_ABGR32: Self = Self(373694468);
    pub const SDL_PIXELFORMAT_YV12: Self = Self(842094169);
    pub const SDL_PIXELFORMAT_IYUV: Self = Self(1448433993);
    pub const SDL_PIXELFORMAT_YUY2: Self = Self(844715353);
    pub const SDL_PIXELFORMAT_UYVY: Self = Self(1498831189);
    pub const SDL_PIXELFORMAT_YVYU: Self = Self(1431918169);
    pub const SDL_PIXELFORMAT_NV12: Self = Self(842094158);
    pub const SDL_PIXELFORMAT_NV21: Self = Self(825382478);
    pub const SDL_PIXELFORMAT_EXTERNAL_OES: Self = Self(542328143);
}
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_0 {
    pub const SDL_WINDOW_FULLSCREEN: Self = Self(1);
    pub const SDL_WINDOW_OPENGL: Self = Self(2);
    pub const SDL_WINDOW_SHOWN: Self = Self(4);
    pub const SDL_WINDOW_HIDDEN: Self = Self(8);
    pub const SDL_WINDOW_BORDERLESS: Self = Self(16);
    pub const SDL_WINDOW_RESIZABLE: Self = Self(32);
    pub const SDL_WINDOW_MINIMIZED: Self = Self(64);
    pub const SDL_WINDOW_MAXIMIZED: Self = Self(128);
    pub const SDL_WINDOW_MOUSE_GRABBED: Self = Self(256);
    pub const SDL_WINDOW_INPUT_FOCUS: Self = Self(512);
    pub const SDL_WINDOW_MOUSE_FOCUS: Self = Self(1024);
    pub const SDL_WINDOW_FULLSCREEN_DESKTOP: Self = Self(4097);
    pub const SDL_WINDOW_FOREIGN: Self = Self(2048);
    pub const SDL_WINDOW_ALLOW_HIGHDPI: Self = Self(8192);
    pub const SDL_WINDOW_MOUSE_CAPTURE: Self = Self(16384);
    pub const SDL_WINDOW_ALWAYS_ON_TOP: Self = Self(32768);
    pub const SDL_WINDOW_SKIP_TASKBAR: Self = Self(65536);
    pub const SDL_WINDOW_UTILITY: Self = Self(131072);
    pub const SDL_WINDOW_TOOLTIP: Self = Self(262144);
    pub const SDL_WINDOW_POPUP_MENU: Self = Self(524288);
    pub const SDL_WINDOW_KEYBOARD_GRABBED: Self = Self(1048576);
    pub const SDL_WINDOW_VULKAN: Self = Self(268435456);
    pub const SDL_WINDOW_METAL: Self = Self(536870912);
    pub const SDL_WINDOW_INPUT_GRABBED: Self = Self(256);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_RendererInfo {
    pub name: *const ::core::ffi::c_char,
    pub flags: Uint32,
    pub num_texture_formats: Uint32,
    pub texture_formats: [Uint32; 16],
    pub max_texture_width: ::core::ffi::c_int,
    pub max_texture_height: ::core::ffi::c_int,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_1(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_1 {
    pub const SDL_TEXTUREACCESS_STATIC: Self = Self(0);
    pub const SDL_TEXTUREACCESS_STREAMING: Self = Self(1);
    pub const SDL_TEXTUREACCESS_TARGET: Self = Self(2);
}
pub type sint16 = ::core::ffi::c_short;
pub type SWORD = sint16;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct BOOLEAN(pub ::core::ffi::c_uint);
impl BOOLEAN {
    pub const FALSE: Self = Self(0);
    pub const TRUE: Self = Self(1);
}
pub type EXTENT = extent;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct extent {
    pub width: COORD,
    pub height: COORD,
}
pub type COORD = SWORD;
pub type POINT = point;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct point {
    pub x: COORD,
    pub y: COORD,
}
pub type RECT = rect;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rect {
    pub corner: POINT,
    pub extent: EXTENT,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct SCREEN(pub ::core::ffi::c_uint);
impl SCREEN {
    pub const TFB_SCREEN_MAIN: Self = Self(0);
    pub const TFB_SCREEN_EXTRA: Self = Self(1);
    pub const TFB_SCREEN_TRANSITION: Self = Self(2);
    pub const TFB_GFX_NUMSCREENS: Self = Self(3);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_2(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_2 {
    pub const TFB_REDRAW_NO: Self = Self(0);
    pub const TFB_REDRAW_FADING: Self = Self(1);
    pub const TFB_REDRAW_EXPOSE: Self = Self(2);
    pub const TFB_REDRAW_YES: Self = Self(3);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _tfb_graphics_backend {
    pub preprocess: Option<
        unsafe extern "C" fn(
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub postprocess: Option<unsafe extern "C" fn() -> ()>,
    pub uploadTransitionScreen: Option<unsafe extern "C" fn() -> ()>,
    pub screen: Option<unsafe extern "C" fn(SCREEN, Uint8, *mut SDL_Rect) -> ()>,
    pub color: Option<
        unsafe extern "C" fn(Uint8, Uint8, Uint8, Uint8, *mut SDL_Rect) -> (),
    >,
}
pub type TFB_GRAPHICS_BACKEND = _tfb_graphics_backend;
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
pub type TFB_SDL2_SCREENINFO = tfb_sdl2_screeninfo_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tfb_sdl2_screeninfo_s {
    pub scaled: *mut SDL_Surface,
    pub texture: *mut SDL_Texture,
    pub dirty: BOOLEAN,
    pub active: BOOLEAN,
    pub updated: SDL_Rect,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TFB_BoundingBox {
    pub valid: ::core::ffi::c_int,
    pub region: RECT,
    pub clip: RECT,
}
pub type TFB_ScaleFunc = Option<
    unsafe extern "C" fn(*mut SDL_Surface, *mut SDL_Surface, *mut SDL_Rect) -> (),
>;
static mut SDL2_Screens: [TFB_SDL2_SCREENINFO; 3] = [TFB_SDL2_SCREENINFO {
    scaled: ::core::ptr::null_mut::<SDL_Surface>(),
    texture: ::core::ptr::null_mut::<SDL_Texture>(),
    dirty: BOOLEAN::FALSE,
    active: BOOLEAN::FALSE,
    updated: SDL_Rect { x: 0, y: 0, w: 0, h: 0 },
}; 3];
static mut window: *mut SDL_Window = ::core::ptr::null_mut::<SDL_Window>();
static mut renderer: *mut SDL_Renderer = ::core::ptr::null_mut::<SDL_Renderer>();
static mut rendererBackend: *const ::core::ffi::c_char = ::core::ptr::null::<
    ::core::ffi::c_char,
>();
static mut scaler: TFB_ScaleFunc = None;
static mut sdl2_scaled_backend: TFB_GRAPHICS_BACKEND = _tfb_graphics_backend {
    preprocess: Some(
        TFB_SDL2_Preprocess
            as unsafe extern "C" fn(
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
            ) -> (),
    ),
    postprocess: Some(TFB_SDL2_Postprocess as unsafe extern "C" fn() -> ()),
    uploadTransitionScreen: Some(
        TFB_SDL2_UploadTransitionScreen as unsafe extern "C" fn() -> (),
    ),
    screen: Some(
        TFB_SDL2_Scaled_ScreenLayer
            as unsafe extern "C" fn(SCREEN, Uint8, *mut SDL_Rect) -> (),
    ),
    color: Some(
        TFB_SDL2_ColorLayer
            as unsafe extern "C" fn(Uint8, Uint8, Uint8, Uint8, *mut SDL_Rect) -> (),
    ),
};
static mut sdl2_unscaled_backend: TFB_GRAPHICS_BACKEND = _tfb_graphics_backend {
    preprocess: Some(
        TFB_SDL2_Preprocess
            as unsafe extern "C" fn(
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
            ) -> (),
    ),
    postprocess: Some(TFB_SDL2_Postprocess as unsafe extern "C" fn() -> ()),
    uploadTransitionScreen: Some(
        TFB_SDL2_UploadTransitionScreen as unsafe extern "C" fn() -> (),
    ),
    screen: Some(
        TFB_SDL2_Unscaled_ScreenLayer
            as unsafe extern "C" fn(SCREEN, Uint8, *mut SDL_Rect) -> (),
    ),
    color: Some(
        TFB_SDL2_ColorLayer
            as unsafe extern "C" fn(Uint8, Uint8, Uint8, Uint8, *mut SDL_Rect) -> (),
    ),
};
unsafe extern "C" fn Create_Screen(
    mut w: ::core::ffi::c_int,
    mut h: ::core::ffi::c_int,
) -> *mut SDL_Surface {
    let mut newsurf: *mut SDL_Surface = SDL_CreateRGBSurface(
        0 as Uint32,
        w,
        h,
        32 as ::core::ffi::c_int,
        0xff000000 as Uint32,
        0xff0000 as ::core::ffi::c_int as Uint32,
        0xff00 as Uint32,
        0 as Uint32,
    );
    if newsurf.is_null() {
        log_add(
            log_Level::log_Error,
            b"Couldn't create screen buffers: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            SDL_GetError(),
        );
    }
    return newsurf;
}
unsafe extern "C" fn ReInit_Screen(
    mut screen: *mut *mut SDL_Surface,
    mut w: ::core::ffi::c_int,
    mut h: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if !(*screen).is_null() {
        SDL_FreeSurface(*screen);
    }
    *screen = Create_Screen(w, h);
    return if (*screen).is_null() {
        -1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    };
}
unsafe extern "C" fn FindBestRenderDriver() -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    if rendererBackend.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    n = SDL_GetNumRenderDrivers();
    log_add(
        log_Level::log_Info,
        b"Searching for render driver \"%s\".\0".as_ptr() as *const ::core::ffi::c_char,
        rendererBackend,
    );
    i = 0 as ::core::ffi::c_int;
    while i < n {
        let mut info: SDL_RendererInfo = SDL_RendererInfo {
            name: ::core::ptr::null::<::core::ffi::c_char>(),
            flags: 0,
            num_texture_formats: 0,
            texture_formats: [0; 16],
            max_texture_width: 0,
            max_texture_height: 0,
        };
        if SDL_GetRenderDriverInfo(i, &raw mut info) >= 0 as ::core::ffi::c_int {
            if strcmp(info.name, rendererBackend) == 0 {
                return i;
            }
            log_add(
                log_Level::log_Info,
                b"Skipping render driver \"%s\"\0".as_ptr()
                    as *const ::core::ffi::c_char,
                info.name,
            );
        }
        i += 1;
    }
    log_add(
        log_Level::log_Info,
        b"Render driver \"%s\" not available, using system default\0".as_ptr()
            as *const ::core::ffi::c_char,
        rendererBackend,
    );
    return -1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn TFB_Pure_ConfigureVideo(
    mut driver: ::core::ffi::c_int,
    mut flags: ::core::ffi::c_int,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut togglefullscreen: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    GraphicsDriver = driver;
    if window.is_null() {
        let mut info: SDL_RendererInfo = SDL_RendererInfo {
            name: ::core::ptr::null::<::core::ffi::c_char>(),
            flags: 0,
            num_texture_formats: 0,
            texture_formats: [0; 16],
            max_texture_width: 0,
            max_texture_height: 0,
        };
        let mut caption: [::core::ffi::c_char; 200] = [0; 200];
        sprintf(
            &raw mut caption as *mut ::core::ffi::c_char,
            b"The Ur-Quan Masters v%d.%d.%d%s\0".as_ptr() as *const ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
            8 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            b"\0".as_ptr() as *const ::core::ffi::c_char,
        );
        window = SDL_CreateWindow(
            &raw mut caption as *mut ::core::ffi::c_char,
            (0x2fff0000 as ::core::ffi::c_uint | 0 as ::core::ffi::c_uint)
                as ::core::ffi::c_int,
            (0x2fff0000 as ::core::ffi::c_uint | 0 as ::core::ffi::c_uint)
                as ::core::ffi::c_int,
            width,
            height,
            0 as Uint32,
        );
        if flags & (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int != 0 {
            SDL_SetWindowFullscreen(
                window,
                C2Rust_Unnamed_0::SDL_WINDOW_FULLSCREEN_DESKTOP.0 as ::core::ffi::c_int
                    as Uint32,
            );
        }
        if window.is_null() {
            return -1 as ::core::ffi::c_int;
        }
        renderer = SDL_CreateRenderer(window, FindBestRenderDriver(), 0 as Uint32);
        if renderer.is_null() {
            return -1 as ::core::ffi::c_int;
        }
        if SDL_GetRendererInfo(renderer, &raw mut info) == 0 as ::core::ffi::c_int {
            log_add(
                log_Level::log_Info,
                b"SDL2 renderer '%s' selected.\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                info.name,
            );
        } else {
            log_add(
                log_Level::log_Info,
                b"SDL2 renderer had no name.\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        SDL_RenderSetLogicalSize(renderer, ScreenWidth, ScreenHeight);
        i = 0 as ::core::ffi::c_int;
        while i < SCREEN::TFB_GFX_NUMSCREENS.0 as ::core::ffi::c_int {
            SDL2_Screens[i as usize].scaled = ::core::ptr::null_mut::<SDL_Surface>();
            SDL2_Screens[i as usize].texture = ::core::ptr::null_mut::<SDL_Texture>();
            SDL2_Screens[i as usize].dirty = BOOLEAN::TRUE;
            SDL2_Screens[i as usize].active = BOOLEAN::TRUE;
            if 0 as ::core::ffi::c_int
                != ReInit_Screen(
                    (&raw mut SDL_Screens as *mut *mut SDL_Surface).offset(i as isize),
                    ScreenWidth,
                    ScreenHeight,
                )
            {
                return -1 as ::core::ffi::c_int;
            }
            i += 1;
        }
        SDL2_Screens[1usize].active = BOOLEAN::FALSE;
        SDL_Screen = SDL_Screens[0usize];
        TransitionScreen = SDL_Screens[2usize];
        format_conv_surf = SDL_CreateRGBSurface(
            0 as Uint32,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            32 as ::core::ffi::c_int,
            0xff000000 as Uint32,
            0xff0000 as ::core::ffi::c_int as Uint32,
            0xff00 as Uint32,
            0xff as Uint32,
        );
        if format_conv_surf.is_null() {
            return -1 as ::core::ffi::c_int;
        }
    } else if flags & (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int != 0 {
        SDL_SetWindowFullscreen(
            window,
            C2Rust_Unnamed_0::SDL_WINDOW_FULLSCREEN_DESKTOP.0 as ::core::ffi::c_int
                as Uint32,
        );
    } else {
        SDL_SetWindowFullscreen(window, 0 as Uint32);
        SDL_SetWindowSize(window, width, height);
    }
    if GfxFlags
        & ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) != 0
    {
        SDL_SetHint(
            b"SDL_RENDER_SCALE_QUALITY\0".as_ptr() as *const ::core::ffi::c_char,
            b"1\0".as_ptr() as *const ::core::ffi::c_char,
        );
    } else {
        SDL_SetHint(
            b"SDL_RENDER_SCALE_QUALITY\0".as_ptr() as *const ::core::ffi::c_char,
            b"0\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if GfxFlags
        & (((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int)
            & !((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)) != 0
    {
        i = 0 as ::core::ffi::c_int;
        while i < SCREEN::TFB_GFX_NUMSCREENS.0 as ::core::ffi::c_int {
            if SDL2_Screens[i as usize].active.0 != 0 {
                if 0 as ::core::ffi::c_int
                    != ReInit_Screen(
                        &raw mut (*(&raw mut SDL2_Screens as *mut TFB_SDL2_SCREENINFO)
                            .offset(i as isize))
                            .scaled,
                        ScreenWidth * 2 as ::core::ffi::c_int,
                        ScreenHeight * 2 as ::core::ffi::c_int,
                    )
                {
                    return -1 as ::core::ffi::c_int;
                }
                if !SDL2_Screens[i as usize].texture.is_null() {
                    SDL_DestroyTexture(SDL2_Screens[i as usize].texture);
                    SDL2_Screens[i as usize].texture = ::core::ptr::null_mut::<
                        SDL_Texture,
                    >();
                }
                SDL2_Screens[i as usize].texture = SDL_CreateTexture(
                    renderer,
                    C2Rust_Unnamed::SDL_PIXELFORMAT_RGBX8888.0 as ::core::ffi::c_int
                        as Uint32,
                    C2Rust_Unnamed_1::SDL_TEXTUREACCESS_STREAMING.0
                        as ::core::ffi::c_int,
                    ScreenWidth * 2 as ::core::ffi::c_int,
                    ScreenHeight * 2 as ::core::ffi::c_int,
                );
                SDL_LockSurface(SDL2_Screens[i as usize].scaled);
                SDL_UpdateTexture(
                    SDL2_Screens[i as usize].texture,
                    ::core::ptr::null::<SDL_Rect>(),
                    (*SDL2_Screens[i as usize].scaled).pixels,
                    (*SDL2_Screens[i as usize].scaled).pitch,
                );
                SDL_UnlockSurface(SDL2_Screens[i as usize].scaled);
            }
            i += 1;
        }
        scaler = Scale_PrepPlatform(flags, (*SDL2_Screens[0usize].scaled).format);
        graphics_backend = &raw mut sdl2_scaled_backend;
    } else {
        i = 0 as ::core::ffi::c_int;
        while i < SCREEN::TFB_GFX_NUMSCREENS.0 as ::core::ffi::c_int {
            if !SDL2_Screens[i as usize].scaled.is_null() {
                SDL_FreeSurface(SDL2_Screens[i as usize].scaled);
                SDL2_Screens[i as usize].scaled = ::core::ptr::null_mut::<SDL_Surface>();
            }
            if !SDL2_Screens[i as usize].texture.is_null() {
                SDL_DestroyTexture(SDL2_Screens[i as usize].texture);
                SDL2_Screens[i as usize].texture = ::core::ptr::null_mut::<
                    SDL_Texture,
                >();
            }
            SDL2_Screens[i as usize].texture = SDL_CreateTexture(
                renderer,
                C2Rust_Unnamed::SDL_PIXELFORMAT_RGBX8888.0 as ::core::ffi::c_int
                    as Uint32,
                C2Rust_Unnamed_1::SDL_TEXTUREACCESS_STREAMING.0 as ::core::ffi::c_int,
                ScreenWidth,
                ScreenHeight,
            );
            SDL_LockSurface(SDL_Screens[i as usize]);
            SDL_UpdateTexture(
                SDL2_Screens[i as usize].texture,
                ::core::ptr::null::<SDL_Rect>(),
                (*SDL_Screens[i as usize]).pixels,
                (*SDL_Screens[i as usize]).pitch,
            );
            SDL_UnlockSurface(SDL_Screens[i as usize]);
            i += 1;
        }
        scaler = None;
        graphics_backend = &raw mut sdl2_unscaled_backend;
    }
    ScreenWidthActual = width;
    ScreenHeightActual = height;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn TFB_Pure_InitGraphics(
    mut driver: ::core::ffi::c_int,
    mut flags: ::core::ffi::c_int,
    mut renderer_0: *const ::core::ffi::c_char,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    log_add(
        log_Level::log_Info,
        b"Initializing SDL.\0".as_ptr() as *const ::core::ffi::c_char,
    );
    log_add(
        log_Level::log_Info,
        b"SDL initialized.\0".as_ptr() as *const ::core::ffi::c_char,
    );
    log_add(
        log_Level::log_Info,
        b"Initializing Screen.\0".as_ptr() as *const ::core::ffi::c_char,
    );
    ScreenWidth = 320 as ::core::ffi::c_int;
    ScreenHeight = 240 as ::core::ffi::c_int;
    rendererBackend = renderer_0;
    if TFB_Pure_ConfigureVideo(driver, flags, width, height, 0 as ::core::ffi::c_int)
        != 0
    {
        log_add(
            log_Level::log_Fatal,
            b"Could not initialize video: %s\0".as_ptr() as *const ::core::ffi::c_char,
            SDL_GetError(),
        );
        exit(1 as ::core::ffi::c_int);
    }
    Scale_Init();
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn TFB_Pure_UninitGraphics() {
    if !renderer.is_null() {
        SDL_DestroyRenderer(renderer);
    }
    if !window.is_null() {
        SDL_DestroyWindow(window);
    }
}
unsafe extern "C" fn TFB_SDL2_UploadTransitionScreen() {
    SDL2_Screens[SCREEN::TFB_SCREEN_TRANSITION.0 as ::core::ffi::c_int as usize]
        .updated
        .x = 0 as ::core::ffi::c_int;
    SDL2_Screens[SCREEN::TFB_SCREEN_TRANSITION.0 as ::core::ffi::c_int as usize]
        .updated
        .y = 0 as ::core::ffi::c_int;
    SDL2_Screens[SCREEN::TFB_SCREEN_TRANSITION.0 as ::core::ffi::c_int as usize]
        .updated
        .w = ScreenWidth;
    SDL2_Screens[SCREEN::TFB_SCREEN_TRANSITION.0 as ::core::ffi::c_int as usize]
        .updated
        .h = ScreenHeight;
    SDL2_Screens[SCREEN::TFB_SCREEN_TRANSITION.0 as ::core::ffi::c_int as usize].dirty = BOOLEAN::TRUE;
}
unsafe extern "C" fn TFB_SDL2_UpdateTexture(
    mut dest: *mut SDL_Texture,
    mut src: *mut SDL_Surface,
    mut rect: *mut SDL_Rect,
) {
    let mut srcBytes: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    SDL_LockSurface(src);
    srcBytes = (*src).pixels as *mut ::core::ffi::c_char;
    if !rect.is_null() {
        srcBytes = srcBytes
            .offset(
                ((*src).pitch * (*rect).y + (*rect).x * 4 as ::core::ffi::c_int) as isize,
            );
    }
    SDL_UpdateTexture(dest, rect, srcBytes as *const ::core::ffi::c_void, (*src).pitch);
    SDL_UnlockSurface(src);
}
unsafe extern "C" fn TFB_SDL2_ScanLines() {
    let mut y: ::core::ffi::c_int = 0;
    SDL_SetRenderDrawColor(renderer, 0 as Uint8, 0 as Uint8, 0 as Uint8, 64 as Uint8);
    SDL_SetRenderDrawBlendMode(renderer, SDL_BlendMode::SDL_BLENDMODE_BLEND);
    SDL_RenderSetLogicalSize(
        renderer,
        ScreenWidth * 2 as ::core::ffi::c_int,
        ScreenHeight * 2 as ::core::ffi::c_int,
    );
    y = 0 as ::core::ffi::c_int;
    while y < ScreenHeight * 2 as ::core::ffi::c_int {
        SDL_RenderDrawLine(
            renderer,
            0 as ::core::ffi::c_int,
            y,
            ScreenWidth * 2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int,
            y,
        );
        y += 2 as ::core::ffi::c_int;
    }
    SDL_RenderSetLogicalSize(renderer, ScreenWidth, ScreenHeight);
}
unsafe extern "C" fn TFB_SDL2_Preprocess(
    mut force_full_redraw: ::core::ffi::c_int,
    mut transition_amount: ::core::ffi::c_int,
    mut fade_amount: ::core::ffi::c_int,
) {
    if force_full_redraw == C2Rust_Unnamed_2::TFB_REDRAW_YES.0 as ::core::ffi::c_int {
        SDL2_Screens[SCREEN::TFB_SCREEN_MAIN.0 as ::core::ffi::c_int as usize]
            .updated
            .x = 0 as ::core::ffi::c_int;
        SDL2_Screens[SCREEN::TFB_SCREEN_MAIN.0 as ::core::ffi::c_int as usize]
            .updated
            .y = 0 as ::core::ffi::c_int;
        SDL2_Screens[SCREEN::TFB_SCREEN_MAIN.0 as ::core::ffi::c_int as usize]
            .updated
            .w = ScreenWidth;
        SDL2_Screens[SCREEN::TFB_SCREEN_MAIN.0 as ::core::ffi::c_int as usize]
            .updated
            .h = ScreenHeight;
        SDL2_Screens[SCREEN::TFB_SCREEN_MAIN.0 as ::core::ffi::c_int as usize].dirty = BOOLEAN::TRUE;
    } else if TFB_BBox.valid != 0 {
        SDL2_Screens[SCREEN::TFB_SCREEN_MAIN.0 as ::core::ffi::c_int as usize]
            .updated
            .x = TFB_BBox.region.corner.x as ::core::ffi::c_int;
        SDL2_Screens[SCREEN::TFB_SCREEN_MAIN.0 as ::core::ffi::c_int as usize]
            .updated
            .y = TFB_BBox.region.corner.y as ::core::ffi::c_int;
        SDL2_Screens[SCREEN::TFB_SCREEN_MAIN.0 as ::core::ffi::c_int as usize]
            .updated
            .w = TFB_BBox.region.extent.width as ::core::ffi::c_int;
        SDL2_Screens[SCREEN::TFB_SCREEN_MAIN.0 as ::core::ffi::c_int as usize]
            .updated
            .h = TFB_BBox.region.extent.height as ::core::ffi::c_int;
        SDL2_Screens[SCREEN::TFB_SCREEN_MAIN.0 as ::core::ffi::c_int as usize].dirty = BOOLEAN::TRUE;
    }
    SDL_SetRenderDrawBlendMode(renderer, SDL_BlendMode::SDL_BLENDMODE_NONE);
    SDL_SetRenderDrawColor(renderer, 0 as Uint8, 0 as Uint8, 0 as Uint8, 255 as Uint8);
    SDL_RenderClear(renderer);
}
unsafe extern "C" fn TFB_SDL2_Unscaled_ScreenLayer(
    mut screen: SCREEN,
    mut a: Uint8,
    mut rect: *mut SDL_Rect,
) {
    let mut texture: *mut SDL_Texture = SDL2_Screens[screen.0 as usize].texture;
    if SDL2_Screens[screen.0 as usize].dirty.0 != 0 {
        TFB_SDL2_UpdateTexture(
            texture,
            SDL_Screens[screen.0 as usize],
            &raw mut (*(&raw mut SDL2_Screens as *mut TFB_SDL2_SCREENINFO)
                .offset(screen.0 as isize))
                .updated,
        );
    }
    if a as ::core::ffi::c_int == 255 as ::core::ffi::c_int {
        SDL_SetTextureBlendMode(texture, SDL_BlendMode::SDL_BLENDMODE_NONE);
    } else {
        SDL_SetTextureBlendMode(texture, SDL_BlendMode::SDL_BLENDMODE_BLEND);
        SDL_SetTextureAlphaMod(texture, a);
    }
    SDL_RenderCopy(renderer, texture, rect, rect);
}
unsafe extern "C" fn TFB_SDL2_Scaled_ScreenLayer(
    mut screen: SCREEN,
    mut a: Uint8,
    mut rect: *mut SDL_Rect,
) {
    let mut texture: *mut SDL_Texture = SDL2_Screens[screen.0 as usize].texture;
    let mut srcRect: SDL_Rect = SDL_Rect { x: 0, y: 0, w: 0, h: 0 };
    let mut pSrcRect: *mut SDL_Rect = ::core::ptr::null_mut::<SDL_Rect>();
    if SDL2_Screens[screen.0 as usize].dirty.0 != 0 {
        let mut src: *mut SDL_Surface = SDL2_Screens[screen.0 as usize].scaled;
        let mut scaled_update: SDL_Rect = SDL2_Screens[screen.0 as usize].updated;
        scaler
            .expect(
                "non-null function pointer",
            )(
            SDL_Screens[screen.0 as usize],
            src,
            &raw mut (*(&raw mut SDL2_Screens as *mut TFB_SDL2_SCREENINFO)
                .offset(screen.0 as isize))
                .updated,
        );
        scaled_update.x *= 2 as ::core::ffi::c_int;
        scaled_update.y *= 2 as ::core::ffi::c_int;
        scaled_update.w *= 2 as ::core::ffi::c_int;
        scaled_update.h *= 2 as ::core::ffi::c_int;
        TFB_SDL2_UpdateTexture(texture, src, &raw mut scaled_update);
    }
    if a as ::core::ffi::c_int == 255 as ::core::ffi::c_int {
        SDL_SetTextureBlendMode(texture, SDL_BlendMode::SDL_BLENDMODE_NONE);
    } else {
        SDL_SetTextureBlendMode(texture, SDL_BlendMode::SDL_BLENDMODE_BLEND);
        SDL_SetTextureAlphaMod(texture, a);
    }
    if !rect.is_null() {
        srcRect = *rect;
        srcRect.x *= 2 as ::core::ffi::c_int;
        srcRect.y *= 2 as ::core::ffi::c_int;
        srcRect.w *= 2 as ::core::ffi::c_int;
        srcRect.h *= 2 as ::core::ffi::c_int;
        pSrcRect = &raw mut srcRect;
    }
    SDL_RenderCopy(renderer, texture, pSrcRect, rect);
}
unsafe extern "C" fn TFB_SDL2_ColorLayer(
    mut r: Uint8,
    mut g: Uint8,
    mut b: Uint8,
    mut a: Uint8,
    mut rect: *mut SDL_Rect,
) {
    SDL_SetRenderDrawBlendMode(
        renderer,
        SDL_BlendMode(
            (if a as ::core::ffi::c_int == 255 as ::core::ffi::c_int {
                SDL_BlendMode::SDL_BLENDMODE_NONE.0 as ::core::ffi::c_int
            } else {
                SDL_BlendMode::SDL_BLENDMODE_BLEND.0 as ::core::ffi::c_int
            }) as ::core::ffi::c_uint,
        ),
    );
    SDL_SetRenderDrawColor(renderer, r, g, b, a);
    SDL_RenderFillRect(renderer, rect);
}
unsafe extern "C" fn TFB_SDL2_Postprocess() {
    if GfxFlags & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int != 0 {
        TFB_SDL2_ScanLines();
    }
    SDL_RenderPresent(renderer);
}
