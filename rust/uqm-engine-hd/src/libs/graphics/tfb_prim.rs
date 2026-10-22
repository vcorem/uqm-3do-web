// Copyright Michael Martin, 2003

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

/* The original Primitive routines do various elaborate checks to
 * ensure we're within bounds for the clipping.  Since clipping is
 * handled by the underlying TFB_Canvas implementation, we need not
 * worry about this. */

// Rust translation of src/libs/graphics/tfb_prim.c.
extern "C" {
    pub type NativePalette;
    static mut _CurFramePtr: FRAME;
    fn TFB_DrawScreen_Line(
        x1: ::core::ffi::c_int,
        y1: ::core::ffi::c_int,
        x2: ::core::ffi::c_int,
        y2: ::core::ffi::c_int,
        color: Color,
        _: DrawMode,
        dest: SCREEN,
        thickness: BYTE,
    );
    fn TFB_DrawScreen_Rect(rect: *mut RECT, _: Color, _: DrawMode, dest: SCREEN);
    fn TFB_DrawScreen_Image(
        img: *mut TFB_Image,
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
        scale: ::core::ffi::c_int,
        scaleMode: ::core::ffi::c_int,
        _: *mut TFB_ColorMap,
        _: DrawMode,
        dest: SCREEN,
    );
    fn TFB_DrawScreen_FilledImage(
        img: *mut TFB_Image,
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
        scale: ::core::ffi::c_int,
        scaleMode: ::core::ffi::c_int,
        _: Color,
        _: DrawMode,
        dest: SCREEN,
    );
    fn TFB_DrawScreen_FontChar(
        _: *mut TFB_Char,
        backing: *mut TFB_Image,
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
        _: DrawMode,
        dest: SCREEN,
    );
    fn TFB_DrawImage_Line(
        x1: ::core::ffi::c_int,
        y1: ::core::ffi::c_int,
        x2: ::core::ffi::c_int,
        y2: ::core::ffi::c_int,
        color: Color,
        _: DrawMode,
        target: *mut TFB_Image,
        thickness: BYTE,
    );
    fn TFB_DrawImage_Rect(
        rect: *mut RECT,
        _: Color,
        _: DrawMode,
        target: *mut TFB_Image,
    );
    fn TFB_DrawImage_Image(
        img: *mut TFB_Image,
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
        scale: ::core::ffi::c_int,
        scaleMode: ::core::ffi::c_int,
        _: *mut TFB_ColorMap,
        _: DrawMode,
        target: *mut TFB_Image,
    );
    fn TFB_DrawImage_FilledImage(
        img: *mut TFB_Image,
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
        scale: ::core::ffi::c_int,
        scaleMode: ::core::ffi::c_int,
        _: Color,
        _: DrawMode,
        target: *mut TFB_Image,
    );
    fn TFB_DrawImage_FontChar(
        _: *mut TFB_Char,
        backing: *mut TFB_Image,
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
        _: DrawMode,
        target: *mut TFB_Image,
    );
    fn TFB_DrawImage_MaskImage(
        img: *mut TFB_Image,
        mode: DrawMode,
        target: *mut TFB_Image,
        fill: *mut Color,
    );
    fn TFB_DrawCanvas_IsPaletted(canvas: TFB_Canvas) -> BOOLEAN;
    fn LockMutex(sem: Mutex);
    fn UnlockMutex(sem: Mutex);
    fn log_add(_: log_Level, fmt: *const ::core::ffi::c_char, ...);
    fn TFB_GetColorMap(index: ::core::ffi::c_int) -> *mut TFB_ColorMap;
    fn GetGraphicScale() -> ::core::ffi::c_int;
    fn GetGraphicScaleMode() -> ::core::ffi::c_int;
}
pub type uint8 = ::core::ffi::c_uchar;
pub type uint16 = ::core::ffi::c_ushort;
pub type sint16 = ::core::ffi::c_short;
pub type uint32 = ::core::ffi::c_uint;
pub type BYTE = uint8;
pub type UWORD = uint16;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stamp {
    pub origin: POINT,
    pub frame: FRAME,
}
pub type STAMP = stamp;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct line {
    pub first: POINT,
    pub second: POINT,
}
pub type LINE = line;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct SCREEN(pub ::core::ffi::c_uint);
impl SCREEN {
    pub const TFB_SCREEN_MAIN: Self = Self(0);
    pub const TFB_SCREEN_EXTRA: Self = Self(1);
    pub const TFB_SCREEN_TRANSITION: Self = Self(2);
    pub const TFB_GFX_NUMSCREENS: Self = Self(3);
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
pub unsafe extern "C" fn TFB_Prim_Point(
    mut p: *mut POINT,
    mut color: Color,
    mut mode: DrawMode,
    mut ctxOrigin: POINT,
    mut scaled: BOOLEAN,
) {
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    r.corner.x = ((*p).x as ::core::ffi::c_int + ctxOrigin.x as ::core::ffi::c_int)
        as COORD;
    r.corner.y = ((*p).y as ::core::ffi::c_int + ctxOrigin.y as ::core::ffi::c_int)
        as COORD;
    r.extent.height = (if scaled.0 == 0 {
        1 as ::core::ffi::c_int
    } else {
        3 as ::core::ffi::c_int
    }) as COORD;
    r.extent.width = r.extent.height;
    if (*_CurFramePtr).Type as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
        TFB_DrawScreen_Rect(&raw mut r, color, mode, SCREEN::TFB_SCREEN_MAIN);
    } else {
        uqm_lua_checked!(TFB_DrawImage_Rect(&raw mut r, color, mode, (*_CurFramePtr).image), ());
    };
}
#[no_mangle]
pub unsafe extern "C" fn TFB_Prim_Rect(
    mut r: *mut RECT,
    mut color: Color,
    mut mode: DrawMode,
    mut ctxOrigin: POINT,
    mut scaled: BOOLEAN,
) {
    let mut arm: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut gscale: ::core::ffi::c_int = 0;
    let mut scale: BYTE = (if scaled.0 == 0 {
        1 as ::core::ffi::c_int
    } else {
        4 as ::core::ffi::c_int
    }) as BYTE;
    gscale = GetGraphicScale();
    arm = *r;
    arm.extent.width = (*r).extent.width;
    arm.extent.height = scale as COORD;
    uqm_lua_checked!(TFB_Prim_FillRect(&raw mut arm, color, mode, ctxOrigin), ());
    arm.extent.height = (*r).extent.height;
    arm.extent.width = scale as COORD;
    uqm_lua_checked!(TFB_Prim_FillRect(&raw mut arm, color, mode, ctxOrigin), ());
    arm.corner.x = (arm.corner.x as ::core::ffi::c_int
        + (((*r).extent.width as ::core::ffi::c_int * gscale
            + (256 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int))
            / 256 as ::core::ffi::c_int - scale as ::core::ffi::c_int)) as COORD;
    uqm_lua_checked!(TFB_Prim_FillRect(&raw mut arm, color, mode, ctxOrigin), ());
    arm.corner.x = (*r).corner.x;
    arm.corner.y = (arm.corner.y as ::core::ffi::c_int
        + (((*r).extent.height as ::core::ffi::c_int * gscale
            + (256 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int))
            / 256 as ::core::ffi::c_int - scale as ::core::ffi::c_int)) as COORD;
    arm.extent.width = (*r).extent.width;
    arm.extent.height = scale as COORD;
    uqm_lua_checked!(TFB_Prim_FillRect(&raw mut arm, color, mode, ctxOrigin), ());
}
#[no_mangle]
pub unsafe extern "C" fn TFB_Prim_FillRect(
    mut r: *mut RECT,
    mut color: Color,
    mut mode: DrawMode,
    mut ctxOrigin: POINT,
) {
    let mut rect: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut gscale: ::core::ffi::c_int = 0;
    rect.corner.x = ((*r).corner.x as ::core::ffi::c_int
        + ctxOrigin.x as ::core::ffi::c_int) as COORD;
    rect.corner.y = ((*r).corner.y as ::core::ffi::c_int
        + ctxOrigin.y as ::core::ffi::c_int) as COORD;
    rect.extent.width = (*r).extent.width;
    rect.extent.height = (*r).extent.height;
    gscale = GetGraphicScale();
    if gscale != 256 as ::core::ffi::c_int {
        rect.extent.width = ((rect.extent.width as ::core::ffi::c_int * gscale
            + (256 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int))
            / 256 as ::core::ffi::c_int) as COORD;
        rect.extent.height = ((rect.extent.height as ::core::ffi::c_int * gscale
            + (256 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int))
            / 256 as ::core::ffi::c_int) as COORD;
    }
    if (*_CurFramePtr).Type as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
        TFB_DrawScreen_Rect(&raw mut rect, color, mode, SCREEN::TFB_SCREEN_MAIN);
    } else {
        uqm_lua_checked!(TFB_DrawImage_Rect(&raw mut rect, color, mode, (*_CurFramePtr).image), ());
    };
}
#[no_mangle]
pub unsafe extern "C" fn TFB_Prim_Line(
    mut line: *mut LINE,
    mut color: Color,
    mut mode: DrawMode,
    mut ctxOrigin: POINT,
    mut thickness: BYTE,
) {
    let mut x1: ::core::ffi::c_int = 0;
    let mut y1: ::core::ffi::c_int = 0;
    let mut x2: ::core::ffi::c_int = 0;
    let mut y2: ::core::ffi::c_int = 0;
    x1 = (*line).first.x as ::core::ffi::c_int + ctxOrigin.x as ::core::ffi::c_int;
    y1 = (*line).first.y as ::core::ffi::c_int + ctxOrigin.y as ::core::ffi::c_int;
    x2 = (*line).second.x as ::core::ffi::c_int + ctxOrigin.x as ::core::ffi::c_int;
    y2 = (*line).second.y as ::core::ffi::c_int + ctxOrigin.y as ::core::ffi::c_int;
    if (*_CurFramePtr).Type as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
        TFB_DrawScreen_Line(
            x1,
            y1,
            x2,
            y2,
            color,
            mode,
            SCREEN::TFB_SCREEN_MAIN,
            thickness,
        );
    } else {
        uqm_lua_checked!(TFB_DrawImage_Line(
            x1,
            y1,
            x2,
            y2,
            color,
            mode,
            (*_CurFramePtr).image,
            thickness,
        ), ());
    };
}
#[no_mangle]
pub unsafe extern "C" fn TFB_Prim_Stamp(
    mut stmp: *mut STAMP,
    mut mode: DrawMode,
    mut ctxOrigin: POINT,
    mut unscaled: BOOLEAN,
) {
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut SrcFramePtr: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
    let mut img: *mut TFB_Image = ::core::ptr::null_mut::<TFB_Image>();
    let mut cmap: *mut TFB_ColorMap = ::core::ptr::null_mut::<TFB_ColorMap>();
    let mut scaleMode: ::core::ffi::c_int = 0;
    SrcFramePtr = (*stmp).frame;
    if SrcFramePtr.is_null() {
        log_add(
            log_Level::log_Warning,
            b"TFB_Prim_Stamp: Tried to draw a NULL frame (Stamp address = %p)\0".as_ptr()
                as *const ::core::ffi::c_char,
            stmp as *mut ::core::ffi::c_void,
        );
        return;
    }
    img = crate::uqm_melee_presentation::UQM_MeleePresentationImage((*SrcFramePtr).image.cast()).cast();
    if img.is_null() {
        log_add(
            log_Level::log_Warning,
            b"Non-existent image to TFB_Prim_Stamp()\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    LockMutex((*img).mutex);
    (*img).NormalHs = (*SrcFramePtr).HotSpot;
    x = (*stmp).origin.x as ::core::ffi::c_int + ctxOrigin.x as ::core::ffi::c_int;
    y = (*stmp).origin.y as ::core::ffi::c_int + ctxOrigin.y as ::core::ffi::c_int;
    if TFB_DrawCanvas_IsPaletted((*img).NormalImg).0 != 0
        && (*img).colormap_index != -1 as ::core::ffi::c_int
    {
        cmap = TFB_GetColorMap((*img).colormap_index);
    }
    UnlockMutex((*img).mutex);
    if unscaled.0 != 0 {
        scaleMode = 256 as ::core::ffi::c_int;
    } else {
        scaleMode = GetGraphicScale();
    }
    if (*_CurFramePtr).Type as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
        TFB_DrawScreen_Image(
            img,
            x,
            y,
            scaleMode,
            GetGraphicScaleMode(),
            cmap,
            mode,
            SCREEN::TFB_SCREEN_MAIN,
        );
    } else {
        uqm_lua_checked!(TFB_DrawImage_Image(
            img,
            x,
            y,
            scaleMode,
            GetGraphicScaleMode(),
            cmap,
            mode,
            (*_CurFramePtr).image,
        ), ());
    };
}
#[no_mangle]
pub unsafe extern "C" fn TFB_Prim_StampFill(
    mut stmp: *mut STAMP,
    mut color: Color,
    mut mode: DrawMode,
    mut ctxOrigin: POINT,
    mut unscaled: BOOLEAN,
) {
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut SrcFramePtr: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
    let mut img: *mut TFB_Image = ::core::ptr::null_mut::<TFB_Image>();
    let mut scaleMode: ::core::ffi::c_int = 0;
    SrcFramePtr = (*stmp).frame;
    if SrcFramePtr.is_null() {
        log_add(
            log_Level::log_Warning,
            b"TFB_Prim_StampFill: Tried to draw a NULL frame (Stamp address = %p)\0"
                .as_ptr() as *const ::core::ffi::c_char,
            stmp as *mut ::core::ffi::c_void,
        );
        return;
    }
    img = (*SrcFramePtr).image;
    if crate::uqm_melee_presentation::UQM_MeleePresentationHiddenCloak(img.cast(), color.r, color.g, color.b) {
        return;
    }
    if img.is_null() {
        log_add(
            log_Level::log_Warning,
            b"Non-existent image to TFB_Prim_StampFill()\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    LockMutex((*img).mutex);
    (*img).NormalHs = (*SrcFramePtr).HotSpot;
    x = (*stmp).origin.x as ::core::ffi::c_int + ctxOrigin.x as ::core::ffi::c_int;
    y = (*stmp).origin.y as ::core::ffi::c_int + ctxOrigin.y as ::core::ffi::c_int;
    UnlockMutex((*img).mutex);
    if unscaled.0 != 0 {
        scaleMode = 256 as ::core::ffi::c_int;
    } else {
        scaleMode = GetGraphicScale();
    }
    if (*_CurFramePtr).Type as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
        TFB_DrawScreen_FilledImage(
            img,
            x,
            y,
            scaleMode,
            GetGraphicScaleMode(),
            color,
            mode,
            SCREEN::TFB_SCREEN_MAIN,
        );
    } else {
        uqm_lua_checked!(TFB_DrawImage_FilledImage(
            img,
            x,
            y,
            scaleMode,
            GetGraphicScaleMode(),
            color,
            mode,
            (*_CurFramePtr).image,
        ), ());
    };
}
#[no_mangle]
pub unsafe extern "C" fn TFB_Prim_FontChar(
    mut charOrigin: POINT,
    mut fontChar: *mut TFB_Char,
    mut backing: *mut TFB_Image,
    mut mode: DrawMode,
    mut ctxOrigin: POINT,
) {
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    x = charOrigin.x as ::core::ffi::c_int + ctxOrigin.x as ::core::ffi::c_int;
    y = charOrigin.y as ::core::ffi::c_int + ctxOrigin.y as ::core::ffi::c_int;
    if (*_CurFramePtr).Type as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
        TFB_DrawScreen_FontChar(fontChar, backing, x, y, mode, SCREEN::TFB_SCREEN_MAIN);
    } else {
        uqm_lua_checked!(TFB_DrawImage_FontChar(fontChar, backing, x, y, mode, (*_CurFramePtr).image), ());
    };
}
#[no_mangle]
pub unsafe extern "C" fn TFB_Prim_MaskFrame(
    mut layer: FRAME,
    mut base: FRAME,
    mut mode: DrawMode,
    mut fill: *mut Color,
) {
    let mut img: *mut TFB_Image = ::core::ptr::null_mut::<TFB_Image>();
    let mut bas: *mut TFB_Image = ::core::ptr::null_mut::<TFB_Image>();
    if base.is_null() {
        log_add(
            log_Level::log_Warning,
            b"TFB_Prim_MaskFrame: NULL frame passed\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if layer.is_null() {
        img = ::core::ptr::null_mut::<TFB_Image>();
    } else {
        img = (*layer).image;
        if img.is_null() {
            log_add(
                log_Level::log_Warning,
                b"Non-existent layer image to TFB_Prim_MaskFrame()\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return;
        }
    }
    bas = (*base).image;
    if bas.is_null() {
        log_add(
            log_Level::log_Warning,
            b"Non-existent base image to TFB_Prim_MaskFrame()\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    uqm_lua_checked!(TFB_DrawImage_MaskImage(img, mode, bas, fill), ());
}
