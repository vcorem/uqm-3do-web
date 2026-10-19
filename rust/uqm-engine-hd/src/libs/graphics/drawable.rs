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

// Rust translation of src/libs/graphics/drawable.c.
extern "C" {
    pub type SDL_BlitMap;
    fn atan2(
        _: ::core::ffi::c_double,
        _: ::core::ffi::c_double,
    ) -> ::core::ffi::c_double;
    fn cos(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn sin(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn sqrt(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn MAKE_HOT_SPOT(_: COORD, _: COORD) -> HOT_SPOT;
    fn CaptureDrawable(Drawable: DRAWABLE) -> FRAME;
    fn ReleaseDrawable(Frame: FRAME) -> DRAWABLE;
    fn GetFrameParentDrawable(Frame: FRAME) -> DRAWABLE;
    fn HMalloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn HFree(p: *mut ::core::ffi::c_void);
    fn HCalloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn __assert_fail(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    ) -> !;
    static mut _pCurContext: CONTEXT;
    static mut _GraphicsStatusFlags: GRAPHICS_STATUS;
    fn _ReleaseCelData(handle: *mut ::core::ffi::c_void) -> BOOLEAN;
    fn TFB_DrawImage_New(canvas: TFB_Canvas) -> *mut TFB_Image;
    fn TFB_DrawImage_New_Rotated(
        img: *mut TFB_Image,
        angle: ::core::ffi::c_int,
    ) -> *mut TFB_Image;
    fn TFB_DrawImage_CopyRect(
        source: *mut TFB_Image,
        srcRect: *const RECT,
        target: *mut TFB_Image,
        dstPt: POINT,
    );
    fn TFB_DrawCanvas_New_TrueColor(
        w: ::core::ffi::c_int,
        h: ::core::ffi::c_int,
        hasalpha: BOOLEAN,
    ) -> TFB_Canvas;
    fn TFB_DrawCanvas_New_Paletted(
        w: ::core::ffi::c_int,
        h: ::core::ffi::c_int,
        palette: *mut Color,
        transparent_index: ::core::ffi::c_int,
    ) -> TFB_Canvas;
    fn TFB_DrawCanvas_IsPaletted(canvas: TFB_Canvas) -> BOOLEAN;
    fn TFB_DrawCanvas_Rescale_Nearest(
        src: TFB_Canvas,
        dst: TFB_Canvas,
        scale: ::core::ffi::c_int,
        src_hs: *mut HOT_SPOT,
        size: *mut EXTENT,
        dst_hs: *mut HOT_SPOT,
    );
    fn TFB_DrawCanvas_Rescale_Bilinear(
        src: TFB_Canvas,
        dst: TFB_Canvas,
        scale: ::core::ffi::c_int,
        src_hs: *mut HOT_SPOT,
        size: *mut EXTENT,
        dst_hs: *mut HOT_SPOT,
    );
    fn TFB_DrawCanvas_ExtractPalette(canvas: TFB_Canvas) -> *mut Color;
    fn TFB_DrawCanvas_SetTransparentColor(
        canvas: TFB_Canvas,
        color: Color,
        rleaccel: BOOLEAN,
    );
    fn TFB_DrawCanvas_GetPixel(
        canvas: TFB_Canvas,
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
    ) -> Color;
    fn TFB_DrawCanvas_GetPixelColors(
        _: TFB_Canvas,
        pixels: *mut Color,
        width: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
    ) -> BOOLEAN;
    fn TFB_DrawCanvas_SetPixelColors(
        _: TFB_Canvas,
        pixels: *const Color,
        width: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
    ) -> BOOLEAN;
    fn TFB_DrawCanvas_GetPixelIndexes(
        _: TFB_Canvas,
        data: *mut BYTE,
        width: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
    ) -> BOOLEAN;
    fn TFB_DrawCanvas_SetPixelIndexes(
        _: TFB_Canvas,
        data: *const BYTE,
        width: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
    ) -> BOOLEAN;
    fn LockMutex(sem: Mutex);
    fn UnlockMutex(sem: Mutex);
    static mut CanvasWidth: ::core::ffi::c_int;
    static mut CanvasHeight: ::core::ffi::c_int;
}
pub type size_t = usize;
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
pub type uint8 = ::core::ffi::c_uchar;
pub type sint8 = ::core::ffi::c_schar;
pub type uint16 = ::core::ffi::c_ushort;
pub type sint16 = ::core::ffi::c_short;
pub type uint32 = ::core::ffi::c_uint;
pub type BYTE = uint8;
pub type SBYTE = sint8;
pub type UWORD = uint16;
pub type SWORD = sint16;
pub type DWORD = uint32;
pub type COUNT = UWORD;
pub type SIZE = SWORD;
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
pub type UniChar = uint32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct context_desc {
    pub Flags: UWORD,
    pub ForeGroundColor: Color,
    pub BackGroundColor: Color,
    pub Mode: DrawMode,
    pub ForeGroundFrame: FRAME,
    pub Font: FONT,
    pub ClipRect: RECT,
    pub FontEffect: FRAME,
    pub FontBacking: *mut TFB_Image,
    pub BackingFlags: FBK_FLAGS,
    pub name: *const ::core::ffi::c_char,
    pub next: CONTEXT,
}
pub type CONTEXT = *mut CONTEXT_DESC;
pub type CONTEXT_DESC = context_desc;
pub type FBK_FLAGS = UWORD;
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
pub type FRAME = *mut FRAME_DESC;
pub type FRAME_DESC = frame_desc;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct frame_desc {
    pub Type: DRAWABLE_TYPE,
    pub Index: UWORD,
    pub HotSpot: HOT_SPOT,
    pub Bounds: EXTENT,
    pub image: *mut TFB_Image,
    pub parent: *mut drawable_desc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct drawable_desc {
    pub Flags: CREATE_FLAGS,
    pub MaxIndex: UWORD,
    pub Frame: *mut FRAME_DESC,
}
pub type CREATE_FLAGS = BYTE;
pub type DRAWABLE_TYPE = UWORD;
pub type RECT = rect;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rect {
    pub corner: POINT,
    pub extent: EXTENT,
}
pub type FONT = *mut FONT_DESC;
pub type FONT_DESC = font_desc;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct font_desc {
    pub Leading: BYTE,
    pub fontPages: *mut FONT_PAGE,
    pub disp: EXTENT,
    pub filename: [::core::ffi::c_char; 4096],
    pub CharSpace: BYTE,
    pub KernAmount: BYTE,
    pub KernTab: [BYTE; 65535],
    pub VertAlign: SBYTE,
    pub HaveFntData: BOOLEAN,
}
pub type FONT_PAGE = FontPage;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FontPage {
    pub next: *mut FontPage,
    pub pageStart: UniChar,
    pub firstChar: UniChar,
    pub numChars: size_t,
    pub charDesc: *mut TFB_Char,
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
pub type DRAWABLE_DESC = drawable_desc;
pub type DRAWABLE = *mut DRAWABLE_DESC;
pub type GRAPHICS_STATUS = BYTE;
#[inline]
unsafe extern "C" fn buildColorRgba(
    mut r: BYTE,
    mut g: BYTE,
    mut b: BYTE,
    mut a: BYTE,
) -> Color {
    let mut c: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    c.r = r;
    c.g = g;
    c.b = b;
    c.a = a;
    return c;
}
#[inline]
unsafe extern "C" fn MAKE_POINT(mut x: COORD, mut y: COORD) -> POINT {
    let mut pt: POINT = point { x: x, y: y };
    return pt;
}
#[no_mangle]
pub static mut _CurFramePtr: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
#[no_mangle]
pub unsafe extern "C" fn SetContextFGFrame(mut Frame: FRAME) -> FRAME {
    let mut LastFrame: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
    LastFrame = _CurFramePtr;
    if Frame != LastFrame {
        if !LastFrame.is_null() {
            _GraphicsStatusFlags = (_GraphicsStatusFlags as ::core::ffi::c_int
                & !(((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                    as GRAPHICS_STATUS as ::core::ffi::c_int)) as GRAPHICS_STATUS;
        }
        _CurFramePtr = Frame;
        if !_CurFramePtr.is_null() {
            _GraphicsStatusFlags = (_GraphicsStatusFlags as ::core::ffi::c_int
                | ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                    as GRAPHICS_STATUS as ::core::ffi::c_int) as GRAPHICS_STATUS;
        }
        if _GraphicsStatusFlags as ::core::ffi::c_int
            & ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GRAPHICS_STATUS
                as ::core::ffi::c_int != 0
        {
            (*_pCurContext).ForeGroundFrame = Frame;
        }
    }
    return LastFrame;
}
#[no_mangle]
pub unsafe extern "C" fn GetContextFGFrame() -> FRAME {
    return _CurFramePtr;
}
unsafe extern "C" fn request_drawable(
    mut NumFrames: COUNT,
    mut DrawableType: DRAWABLE_TYPE,
    mut flags: CREATE_FLAGS,
    mut width: SIZE,
    mut height: SIZE,
) -> DRAWABLE {
    let mut Drawable: DRAWABLE = ::core::ptr::null_mut::<DRAWABLE_DESC>();
    let mut i: COUNT = 0;
    Drawable = AllocDrawable(NumFrames);
    if Drawable.is_null() {
        return ::core::ptr::null_mut::<DRAWABLE_DESC>();
    }
    (*Drawable).Flags = flags;
    (*Drawable).MaxIndex = (NumFrames as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
        as UWORD;
    i = 0 as COUNT;
    while (i as ::core::ffi::c_int) < NumFrames as ::core::ffi::c_int {
        let mut FramePtr: FRAME = (*Drawable).Frame.offset(i as isize);
        if DrawableType as ::core::ffi::c_int == 1 as ::core::ffi::c_int
            && width as ::core::ffi::c_int > 0 as ::core::ffi::c_int
            && height as ::core::ffi::c_int > 0 as ::core::ffi::c_int
        {
            (*FramePtr).image = TFB_DrawImage_New(
                TFB_DrawCanvas_New_TrueColor(
                    width as ::core::ffi::c_int,
                    height as ::core::ffi::c_int,
                    BOOLEAN(
                        (if flags as ::core::ffi::c_int
                            & ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                                as CREATE_FLAGS as ::core::ffi::c_int != 0
                        {
                            BOOLEAN::TRUE.0 as ::core::ffi::c_int
                        } else {
                            BOOLEAN::FALSE.0 as ::core::ffi::c_int
                        }) as ::core::ffi::c_uint,
                    ),
                ),
            );
        }
        (*FramePtr).Type = DrawableType;
        (*FramePtr).Index = i as UWORD;
        (*FramePtr).Bounds.width = width as COORD;
        (*FramePtr).Bounds.height = height as COORD;
        i = i.wrapping_add(1);
    }
    return Drawable;
}
unsafe extern "C" fn request_indexed_drawable(
    mut palette: *mut Color,
    mut width: SIZE,
    mut height: SIZE,
) -> DRAWABLE {
    let mut Drawable: DRAWABLE = ::core::ptr::null_mut::<DRAWABLE_DESC>();
    Drawable = AllocDrawable(1 as COUNT);
    if Drawable.is_null() {
        return ::core::ptr::null_mut::<DRAWABLE_DESC>();
    }
    (*Drawable).Flags = ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
        as CREATE_FLAGS;
    (*Drawable).MaxIndex = 0 as UWORD;
    let mut FramePtr: FRAME = (*Drawable).Frame.offset(0isize);
    if width as ::core::ffi::c_int > 0 as ::core::ffi::c_int
        && height as ::core::ffi::c_int > 0 as ::core::ffi::c_int
    {
        (*FramePtr).image = TFB_DrawImage_New(
            TFB_DrawCanvas_New_Paletted(
                width as ::core::ffi::c_int,
                height as ::core::ffi::c_int,
                palette,
                -1 as ::core::ffi::c_int,
            ),
        );
    } else {
        return ::core::ptr::null_mut::<DRAWABLE_DESC>()
    }
    (*FramePtr).Type = 1 as DRAWABLE_TYPE;
    (*FramePtr).Index = 0 as UWORD;
    (*FramePtr).Bounds.width = width as COORD;
    (*FramePtr).Bounds.height = height as COORD;
    return Drawable;
}
#[no_mangle]
pub unsafe extern "C" fn CreateDisplay(
    mut CreateFlags: CREATE_FLAGS,
    mut pwidth: *mut SIZE,
    mut pheight: *mut SIZE,
) -> DRAWABLE {
    let mut Drawable: DRAWABLE = ::core::ptr::null_mut::<DRAWABLE_DESC>();
    Drawable = request_drawable(
        1 as COUNT,
        2 as DRAWABLE_TYPE,
        (CreateFlags as ::core::ffi::c_int
            & (((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as CREATE_FLAGS
                as ::core::ffi::c_int
                | ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as CREATE_FLAGS
                    as ::core::ffi::c_int)) as CREATE_FLAGS,
        CanvasWidth as SIZE,
        CanvasHeight as SIZE,
    );
    if !Drawable.is_null() {
        let mut F: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
        F = CaptureDrawable(Drawable);
        if F.is_null() {
            DestroyDrawable(Drawable);
        } else {
            *pwidth = (*F).Bounds.width as SIZE;
            *pheight = (*F).Bounds.height as SIZE;
            ReleaseDrawable(F);
            return Drawable;
        }
    }
    *pheight = 0 as SIZE;
    *pwidth = *pheight;
    return ::core::ptr::null_mut::<DRAWABLE_DESC>();
}
#[no_mangle]
pub unsafe extern "C" fn AllocDrawable(mut n: COUNT) -> DRAWABLE {
    let mut Drawable: DRAWABLE = ::core::ptr::null_mut::<DRAWABLE_DESC>();
    Drawable = HCalloc(::core::mem::size_of::<DRAWABLE_DESC>()) as DRAWABLE;
    if !Drawable.is_null() {
        let mut i: ::core::ffi::c_int = 0;
        (*Drawable).Frame = HMalloc(
            ::core::mem::size_of::<FRAME_DESC>().wrapping_mul(n as size_t),
        ) as FRAME as *mut FRAME_DESC;
        if (*Drawable).Frame.is_null() {
            HFree(Drawable as *mut ::core::ffi::c_void);
            return ::core::ptr::null_mut::<DRAWABLE_DESC>();
        }
        i = 0 as ::core::ffi::c_int;
        while i < n as ::core::ffi::c_int {
            let mut F: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
            F = (*Drawable).Frame.offset(i as isize) as FRAME;
            (*F).parent = Drawable as *mut drawable_desc;
            (*F).Type = 0 as DRAWABLE_TYPE;
            (*F).Index = 0 as UWORD;
            (*F).image = ::core::ptr::null_mut::<TFB_Image>();
            (*F).Bounds.width = 0 as COORD;
            (*F).Bounds.height = 0 as COORD;
            (*F).HotSpot.x = 0 as COORD;
            (*F).HotSpot.y = 0 as COORD;
            i += 1;
        }
    }
    return Drawable;
}
#[no_mangle]
pub unsafe extern "C" fn CreateDrawable(
    mut CreateFlags: CREATE_FLAGS,
    mut width: SIZE,
    mut height: SIZE,
    mut num_frames: COUNT,
) -> DRAWABLE {
    let mut Drawable: DRAWABLE = ::core::ptr::null_mut::<DRAWABLE_DESC>();
    Drawable = request_drawable(
        num_frames,
        1 as DRAWABLE_TYPE,
        (CreateFlags as ::core::ffi::c_int
            & (((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as CREATE_FLAGS
                as ::core::ffi::c_int
                | ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as CREATE_FLAGS
                    as ::core::ffi::c_int
                | ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as CREATE_FLAGS
                    as ::core::ffi::c_int
                | ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as CREATE_FLAGS
                    as ::core::ffi::c_int)) as CREATE_FLAGS,
        width,
        height,
    );
    if !Drawable.is_null() {
        let mut F: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
        F = CaptureDrawable(Drawable);
        if !F.is_null() {
            ReleaseDrawable(F);
            return Drawable;
        }
    }
    return ::core::ptr::null_mut::<DRAWABLE_DESC>();
}
#[no_mangle]
pub unsafe extern "C" fn CreateIndexedDrawable(
    mut palette: *mut Color,
    mut width: SIZE,
    mut height: SIZE,
) -> DRAWABLE {
    let mut Drawable: DRAWABLE = ::core::ptr::null_mut::<DRAWABLE_DESC>();
    Drawable = request_indexed_drawable(palette, width, height);
    if !Drawable.is_null() {
        let mut F: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
        F = CaptureDrawable(Drawable);
        if !F.is_null() {
            ReleaseDrawable(F);
            return Drawable;
        }
    }
    return ::core::ptr::null_mut::<DRAWABLE_DESC>();
}
#[no_mangle]
pub unsafe extern "C" fn DestroyDrawable(mut Drawable: DRAWABLE) -> BOOLEAN {
    if !_CurFramePtr.is_null() && Drawable == (*_CurFramePtr).parent {
        SetContextFGFrame(::core::ptr::null_mut::<::core::ffi::c_void>() as FRAME);
    }
    if !Drawable.is_null() {
        _ReleaseCelData(Drawable as *mut ::core::ffi::c_void);
        return BOOLEAN::TRUE;
    }
    return BOOLEAN::FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn GetFrameRect(
    mut FramePtr: FRAME,
    mut pRect: *mut RECT,
) -> BOOLEAN {
    if !FramePtr.is_null() {
        (*pRect).corner.x = -((*FramePtr).HotSpot.x as ::core::ffi::c_int) as COORD;
        (*pRect).corner.y = -((*FramePtr).HotSpot.y as ::core::ffi::c_int) as COORD;
        (*pRect).extent = (*FramePtr).Bounds;
        return BOOLEAN::TRUE;
    }
    return BOOLEAN::FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn SetFrameHot(
    mut FramePtr: FRAME,
    mut HotSpot: HOT_SPOT,
) -> HOT_SPOT {
    if !FramePtr.is_null() {
        let mut OldHot: HOT_SPOT = POINT { x: 0, y: 0 };
        OldHot = (*FramePtr).HotSpot;
        (*FramePtr).HotSpot = HotSpot;
        return OldHot;
    }
    return MAKE_HOT_SPOT(0 as COORD, 0 as COORD);
}
#[no_mangle]
pub unsafe extern "C" fn GetFrameHot(mut FramePtr: FRAME) -> HOT_SPOT {
    if !FramePtr.is_null() {
        return (*FramePtr).HotSpot;
    }
    return MAKE_HOT_SPOT(0 as COORD, 0 as COORD);
}
#[no_mangle]
pub unsafe extern "C" fn RotateFrame(
    mut Frame: FRAME,
    mut angle_deg: ::core::ffi::c_int,
) -> DRAWABLE {
    let mut Drawable: DRAWABLE = ::core::ptr::null_mut::<DRAWABLE_DESC>();
    let mut RotFramePtr: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
    let mut dx: ::core::ffi::c_double = 0.;
    let mut dy: ::core::ffi::c_double = 0.;
    let mut d: ::core::ffi::c_double = 0.;
    let mut angle: ::core::ffi::c_double = angle_deg as ::core::ffi::c_double
        * 3.14159265358979323846f64 / 180 as ::core::ffi::c_int as ::core::ffi::c_double;
    if Frame.is_null() {
        return ::core::ptr::null_mut::<DRAWABLE_DESC>();
    }
    ((*Frame).Type as ::core::ffi::c_int != 2 as ::core::ffi::c_int
        || {
            __assert_fail(
                b"Frame->Type != SCREEN_DRAWABLE\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/libs/graphics/drawable.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                316 as ::core::ffi::c_int,
                b"RotateFrame\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    Drawable = request_drawable(
        1 as COUNT,
        1 as DRAWABLE_TYPE,
        ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as CREATE_FLAGS,
        0 as SIZE,
        0 as SIZE,
    );
    if Drawable.is_null() {
        return ::core::ptr::null_mut::<DRAWABLE_DESC>();
    }
    RotFramePtr = CaptureDrawable(Drawable);
    if RotFramePtr.is_null() {
        _ReleaseCelData(Drawable as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<DRAWABLE_DESC>();
    }
    (*RotFramePtr).image = TFB_DrawImage_New_Rotated((*Frame).image, angle_deg);
    (*RotFramePtr).Bounds.width = (*(*RotFramePtr).image).extent.width;
    (*RotFramePtr).Bounds.height = (*(*RotFramePtr).image).extent.height;
    dx = ((*Frame).HotSpot.x as ::core::ffi::c_int
        - (*Frame).Bounds.width as ::core::ffi::c_int / 2 as ::core::ffi::c_int)
        as ::core::ffi::c_double;
    dy = ((*Frame).HotSpot.y as ::core::ffi::c_int
        - (*Frame).Bounds.height as ::core::ffi::c_int / 2 as ::core::ffi::c_int)
        as ::core::ffi::c_double;
    d = sqrt(dx * dx + dy * dy);
    if d as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
        let mut organg: ::core::ffi::c_double = atan2(-dy, dx);
        dx = cos(organg + angle) * d;
        dy = -sin(organg + angle) * d;
    }
    (*RotFramePtr).HotSpot.x = ((*RotFramePtr).Bounds.width as ::core::ffi::c_int
        / 2 as ::core::ffi::c_int + dx as ::core::ffi::c_int) as COORD;
    (*RotFramePtr).HotSpot.y = ((*RotFramePtr).Bounds.height as ::core::ffi::c_int
        / 2 as ::core::ffi::c_int + dy as ::core::ffi::c_int) as COORD;
    ReleaseDrawable(RotFramePtr);
    return Drawable;
}
#[no_mangle]
pub unsafe extern "C" fn SetFrameTransparentColor(mut frame: FRAME, mut color: Color) {
    let mut img: *mut TFB_Image = ::core::ptr::null_mut::<TFB_Image>();
    if frame.is_null() {
        return;
    }
    ((*frame).Type as ::core::ffi::c_int != 2 as ::core::ffi::c_int
        || {
            __assert_fail(
                b"frame->Type != SCREEN_DRAWABLE\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/libs/graphics/drawable.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                360 as ::core::ffi::c_int,
                b"SetFrameTransparentColor\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    img = (*frame).image;
    LockMutex((*img).mutex);
    TFB_DrawCanvas_SetTransparentColor((*img).NormalImg, color, BOOLEAN::FALSE);
    UnlockMutex((*img).mutex);
}
#[no_mangle]
pub unsafe extern "C" fn GetFramePixel(mut frame: FRAME, mut pixelPt: POINT) -> Color {
    let mut img: *mut TFB_Image = ::core::ptr::null_mut::<TFB_Image>();
    let mut ret: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    if frame.is_null() {
        return buildColorRgba(0 as BYTE, 0 as BYTE, 0 as BYTE, 0 as BYTE);
    }
    ((*frame).Type as ::core::ffi::c_int != 2 as ::core::ffi::c_int
        || {
            __assert_fail(
                b"frame->Type != SCREEN_DRAWABLE\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/libs/graphics/drawable.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                380 as ::core::ffi::c_int,
                b"GetFramePixel\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    img = (*frame).image;
    LockMutex((*img).mutex);
    ret = uqm_lua_checked!(TFB_DrawCanvas_GetPixel(
        (*img).NormalImg,
        pixelPt.x as ::core::ffi::c_int,
        pixelPt.y as ::core::ffi::c_int,
    ), Color { r: 0, g: 0, b: 0, a: 0 });
    UnlockMutex((*img).mutex);
    return ret;
}
unsafe extern "C" fn makeMatchingFrame(
    mut frame: FRAME,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
) -> FRAME {
    let mut drawable: DRAWABLE = ::core::ptr::null_mut::<DRAWABLE_DESC>();
    let mut newFrame: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
    let mut flags: CREATE_FLAGS = 0;
    let dst_has_alpha: bool = (*(*((*(*frame).image).NormalImg as *mut SDL_Surface))
        .format)
        .Amask != 0 as Uint32;
    flags = (if dst_has_alpha as ::core::ffi::c_int != 0 {
        ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as CREATE_FLAGS
            as ::core::ffi::c_int
    } else {
        (*GetFrameParentDrawable(frame)).Flags as ::core::ffi::c_int
    }) as CREATE_FLAGS;
    drawable = CreateDrawable(flags, width as SIZE, height as SIZE, 1 as COUNT);
    if drawable.is_null() {
        return ::core::ptr::null_mut::<FRAME_DESC>();
    }
    newFrame = CaptureDrawable(drawable);
    if newFrame.is_null() {
        _ReleaseCelData(drawable as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<FRAME_DESC>();
    }
    return newFrame;
}
unsafe extern "C" fn makeMatchingIndexedFrame(
    mut frame: FRAME,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
) -> FRAME {
    let mut drawable: DRAWABLE = ::core::ptr::null_mut::<DRAWABLE_DESC>();
    let mut newFrame: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
    drawable = CreateIndexedDrawable(
        TFB_DrawCanvas_ExtractPalette((*(*frame).image).NormalImg),
        width as SIZE,
        height as SIZE,
    );
    if drawable.is_null() {
        return ::core::ptr::null_mut::<FRAME_DESC>();
    }
    newFrame = CaptureDrawable(drawable);
    if newFrame.is_null() {
        _ReleaseCelData(drawable as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<FRAME_DESC>();
    }
    return newFrame;
}
#[no_mangle]
pub unsafe extern "C" fn CopyFrameRect(
    mut frame: FRAME,
    mut area: *const RECT,
) -> DRAWABLE {
    let mut newFrame: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
    let mut nullPt: POINT = MAKE_POINT(0 as COORD, 0 as COORD);
    if frame.is_null() {
        return ::core::ptr::null_mut::<DRAWABLE_DESC>();
    }
    ((*frame).Type as ::core::ffi::c_int != 2 as ::core::ffi::c_int
        || {
            __assert_fail(
                b"frame->Type != SCREEN_DRAWABLE\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/libs/graphics/drawable.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                451 as ::core::ffi::c_int,
                b"CopyFrameRect\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    newFrame = makeMatchingFrame(
        frame,
        (*area).extent.width as ::core::ffi::c_int,
        (*area).extent.height as ::core::ffi::c_int,
    );
    if newFrame.is_null() {
        return ::core::ptr::null_mut::<DRAWABLE_DESC>();
    }
    TFB_DrawImage_CopyRect((*frame).image, area, (*newFrame).image, nullPt);
    return ReleaseDrawable(newFrame);
}
#[no_mangle]
pub unsafe extern "C" fn CloneFrame(mut frame: FRAME) -> DRAWABLE {
    let mut newFrame: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    if frame.is_null() {
        return ::core::ptr::null_mut::<DRAWABLE_DESC>();
    }
    ((*frame).Type as ::core::ffi::c_int != 2 as ::core::ffi::c_int
        || {
            __assert_fail(
                b"frame->Type != SCREEN_DRAWABLE\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/libs/graphics/drawable.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                474 as ::core::ffi::c_int,
                b"CloneFrame\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    GetFrameRect(frame, &raw mut r);
    r.corner.x = 0 as COORD;
    r.corner.y = 0 as COORD;
    newFrame = CaptureDrawable(CopyFrameRect(frame, &raw mut r));
    if newFrame.is_null() {
        return ::core::ptr::null_mut::<DRAWABLE_DESC>();
    }
    (*newFrame).HotSpot = (*frame).HotSpot;
    return ReleaseDrawable(newFrame);
}
#[no_mangle]
pub unsafe extern "C" fn RescaleFrame(
    mut frame: FRAME,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
) -> DRAWABLE {
    let mut newFrame: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
    let mut img: *mut TFB_Image = ::core::ptr::null_mut::<TFB_Image>();
    let mut src: TFB_Canvas = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut dst: TFB_Canvas = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if frame.is_null() {
        return ::core::ptr::null_mut::<DRAWABLE_DESC>();
    }
    ((*frame).Type as ::core::ffi::c_int != 2 as ::core::ffi::c_int
        || {
            __assert_fail(
                b"frame->Type != SCREEN_DRAWABLE\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/libs/graphics/drawable.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                502 as ::core::ffi::c_int,
                b"RescaleFrame\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    if TFB_DrawCanvas_IsPaletted((*(*frame).image).NormalImg).0 != 0 {
        newFrame = makeMatchingIndexedFrame(frame, width, height);
    } else {
        newFrame = makeMatchingFrame(frame, width, height);
    }
    if newFrame.is_null() {
        return ::core::ptr::null_mut::<DRAWABLE_DESC>();
    }
    (*newFrame).HotSpot.x = ((*frame).HotSpot.x as ::core::ffi::c_int * width
        / (*frame).Bounds.width as ::core::ffi::c_int) as COORD;
    (*newFrame).HotSpot.y = ((*frame).HotSpot.y as ::core::ffi::c_int * height
        / (*frame).Bounds.height as ::core::ffi::c_int) as COORD;
    img = (*frame).image;
    LockMutex((*img).mutex);
    src = (*img).NormalImg;
    dst = (*(*newFrame).image).NormalImg;
    TFB_DrawCanvas_Rescale_Nearest(
        src,
        dst,
        -1 as ::core::ffi::c_int,
        ::core::ptr::null_mut::<HOT_SPOT>(),
        ::core::ptr::null_mut::<EXTENT>(),
        ::core::ptr::null_mut::<HOT_SPOT>(),
    );
    UnlockMutex((*img).mutex);
    return ReleaseDrawable(newFrame);
}
#[no_mangle]
pub unsafe extern "C" fn RescalePercentage(
    mut frame: FRAME,
    mut percentage: ::core::ffi::c_float,
) -> DRAWABLE {
    let mut newFrame: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
    let mut img: *mut TFB_Image = ::core::ptr::null_mut::<TFB_Image>();
    let mut src: TFB_Canvas = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut dst: TFB_Canvas = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if frame.is_null()
        || (*frame).Bounds.height as ::core::ffi::c_int == 1 as ::core::ffi::c_int
            && (*frame).Bounds.width as ::core::ffi::c_int == 1 as ::core::ffi::c_int
    {
        return ::core::ptr::null_mut::<DRAWABLE_DESC>();
    }
    ((*frame).Type as ::core::ffi::c_int != 2 as ::core::ffi::c_int
        || {
            __assert_fail(
                b"frame->Type != SCREEN_DRAWABLE\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/libs/graphics/drawable.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                546 as ::core::ffi::c_int,
                b"RescalePercentage\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    percentage = percentage / 100 as ::core::ffi::c_int as ::core::ffi::c_float;
    newFrame = makeMatchingFrame(
        frame,
        ((*frame).Bounds.width as ::core::ffi::c_int as ::core::ffi::c_float
            * percentage) as ::core::ffi::c_int,
        ((*frame).Bounds.height as ::core::ffi::c_int as ::core::ffi::c_float
            * percentage) as ::core::ffi::c_int,
    );
    if newFrame.is_null() {
        return ::core::ptr::null_mut::<DRAWABLE_DESC>();
    }
    (*newFrame).HotSpot.x = ((*frame).HotSpot.x as ::core::ffi::c_int
        as ::core::ffi::c_float * percentage) as COORD;
    (*newFrame).HotSpot.y = ((*frame).HotSpot.y as ::core::ffi::c_int
        as ::core::ffi::c_float * percentage) as COORD;
    img = (*frame).image;
    LockMutex((*img).mutex);
    src = (*img).NormalImg;
    dst = (*(*newFrame).image).NormalImg;
    TFB_DrawCanvas_Rescale_Bilinear(
        src,
        dst,
        -1 as ::core::ffi::c_int,
        ::core::ptr::null_mut::<HOT_SPOT>(),
        ::core::ptr::null_mut::<EXTENT>(),
        ::core::ptr::null_mut::<HOT_SPOT>(),
    );
    UnlockMutex((*img).mutex);
    return ReleaseDrawable(newFrame);
}
#[no_mangle]
pub unsafe extern "C" fn ReadFramePixelColors(
    mut frame: FRAME,
    mut pixels: *mut Color,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
) -> BOOLEAN {
    let mut img: *mut TFB_Image = ::core::ptr::null_mut::<TFB_Image>();
    if frame.is_null() {
        return BOOLEAN::FALSE;
    }
    ((*frame).Type as ::core::ffi::c_int != 2 as ::core::ffi::c_int
        || {
            __assert_fail(
                b"frame->Type != SCREEN_DRAWABLE\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/libs/graphics/drawable.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                583 as ::core::ffi::c_int,
                b"ReadFramePixelColors\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    img = (*frame).image;
    return uqm_lua_checked!(TFB_DrawCanvas_GetPixelColors((*img).NormalImg, pixels, width, height), BOOLEAN(0));
}
#[no_mangle]
pub unsafe extern "C" fn WriteFramePixelColors(
    mut frame: FRAME,
    mut pixels: *const Color,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
) -> BOOLEAN {
    let mut img: *mut TFB_Image = ::core::ptr::null_mut::<TFB_Image>();
    if frame.is_null() {
        return BOOLEAN::FALSE;
    }
    ((*frame).Type as ::core::ffi::c_int != 2 as ::core::ffi::c_int
        || {
            __assert_fail(
                b"frame->Type != SCREEN_DRAWABLE\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/libs/graphics/drawable.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                600 as ::core::ffi::c_int,
                b"WriteFramePixelColors\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    img = (*frame).image;
    return uqm_lua_checked!(TFB_DrawCanvas_SetPixelColors((*img).NormalImg, pixels, width, height), BOOLEAN(0));
}
#[no_mangle]
pub unsafe extern "C" fn ReadFramePixelIndexes(
    mut frame: FRAME,
    mut pixels: *mut BYTE,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut paletted: BOOLEAN,
) -> BOOLEAN {
    let mut img: *mut TFB_Image = ::core::ptr::null_mut::<TFB_Image>();
    if frame.is_null() {
        return BOOLEAN::FALSE;
    }
    ((*frame).Type as ::core::ffi::c_int != 2 as ::core::ffi::c_int
        || {
            __assert_fail(
                b"frame->Type != SCREEN_DRAWABLE\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/libs/graphics/drawable.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                616 as ::core::ffi::c_int,
                b"ReadFramePixelIndexes\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    img = (*frame).image;
    if paletted.0 != 0 {
        return TFB_DrawCanvas_GetPixelIndexes((*img).NormalImg, pixels, width, height)
    } else {
        return BOOLEAN::FALSE
    };
}
#[no_mangle]
pub unsafe extern "C" fn WriteFramePixelIndexes(
    mut frame: FRAME,
    mut pixels: *const BYTE,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
) -> BOOLEAN {
    let mut img: *mut TFB_Image = ::core::ptr::null_mut::<TFB_Image>();
    if frame.is_null() {
        return BOOLEAN::FALSE;
    }
    ((*frame).Type as ::core::ffi::c_int != 2 as ::core::ffi::c_int
        || {
            __assert_fail(
                b"frame->Type != SCREEN_DRAWABLE\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/libs/graphics/drawable.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                638 as ::core::ffi::c_int,
                b"WriteFramePixelIndexes\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    img = (*frame).image;
    return TFB_DrawCanvas_SetPixelIndexes((*img).NormalImg, pixels, width, height);
}
