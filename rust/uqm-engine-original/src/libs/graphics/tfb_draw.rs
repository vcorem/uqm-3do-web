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
 *  Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA  02111-1307  USA
 */

// Rust translation of src/libs/graphics/tfb_draw.c.
extern "C" {
    pub type NativePalette;
    fn HMalloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn HFree(p: *mut ::core::ffi::c_void);
    static mut ScreenWidth: ::core::ffi::c_int;
    static mut ScreenHeight: ::core::ffi::c_int;
    fn TFB_DrawCanvas_New_ForScreen(
        w: ::core::ffi::c_int,
        h: ::core::ffi::c_int,
        withalpha: BOOLEAN,
    ) -> TFB_Canvas;
    fn TFB_DrawCanvas_New_ScaleTarget(
        canvas: TFB_Canvas,
        oldcanvas: TFB_Canvas,
        r#type: ::core::ffi::c_int,
        last_type: ::core::ffi::c_int,
    ) -> TFB_Canvas;
    fn TFB_DrawCanvas_New_RotationTarget(
        src: TFB_Canvas,
        angle: ::core::ffi::c_int,
    ) -> TFB_Canvas;
    fn TFB_DrawCanvas_ToScreenFormat(canvas: TFB_Canvas) -> TFB_Canvas;
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
    fn TFB_DrawCanvas_Rescale_Trilinear(
        src: TFB_Canvas,
        mipmap: TFB_Canvas,
        dst: TFB_Canvas,
        scale: ::core::ffi::c_int,
        src_hs: *mut HOT_SPOT,
        mm_hs: *mut HOT_SPOT,
        size: *mut EXTENT,
        dst_hs: *mut HOT_SPOT,
    );
    fn TFB_DrawCanvas_Rotate(
        src: TFB_Canvas,
        dst: TFB_Canvas,
        angle: ::core::ffi::c_int,
        size: EXTENT,
    );
    fn TFB_DrawCanvas_GetRotatedExtent(
        src: TFB_Canvas,
        angle: ::core::ffi::c_int,
        size: *mut EXTENT,
    );
    fn TFB_DrawCanvas_GetExtent(canvas: TFB_Canvas, size: *mut EXTENT);
    fn TFB_DrawCanvas_Delete(canvas: TFB_Canvas);
    fn TFB_DrawCanvas_Line(
        x1: ::core::ffi::c_int,
        y1: ::core::ffi::c_int,
        x2: ::core::ffi::c_int,
        y2: ::core::ffi::c_int,
        color: Color,
        _: DrawMode,
        target: TFB_Canvas,
    );
    fn TFB_DrawCanvas_Rect(rect: *mut RECT, _: Color, _: DrawMode, target: TFB_Canvas);
    fn TFB_DrawCanvas_Image(
        img: *mut TFB_Image,
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
        scale: ::core::ffi::c_int,
        scaleMode: ::core::ffi::c_int,
        _: *mut TFB_ColorMap,
        _: DrawMode,
        target: TFB_Canvas,
    );
    fn TFB_DrawCanvas_FilledImage(
        img: *mut TFB_Image,
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
        scale: ::core::ffi::c_int,
        scaleMode: ::core::ffi::c_int,
        _: Color,
        _: DrawMode,
        target: TFB_Canvas,
    );
    fn TFB_DrawCanvas_FontChar(
        _: *mut TFB_Char,
        backing: *mut TFB_Image,
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
        _: DrawMode,
        target: TFB_Canvas,
    );
    fn TFB_DrawCanvas_CopyRect(
        source: TFB_Canvas,
        srcRect: *const RECT,
        target: TFB_Canvas,
        dstPt: POINT,
    );
    fn TFB_DrawCanvas_Intersect(
        canvas1: TFB_Canvas,
        c1org: POINT,
        canvas2: TFB_Canvas,
        c2org: POINT,
        interRect: *const RECT,
    ) -> BOOLEAN;
    fn CreateMutex_Core(name: *const ::core::ffi::c_char, syncClass: DWORD) -> Mutex;
    fn GetMyThreadLocal() -> *mut ThreadLocal;
    fn SetSemaphore(sem: Semaphore);
    fn DestroyMutex(sem: Mutex);
    fn LockMutex(sem: Mutex);
    fn UnlockMutex(sem: Mutex);
    fn TFB_BatchReset();
    fn TFB_EnqueueDrawCommand(DrawCommand: *mut TFB_DrawCommand);
    fn Lock_DCQ(slots: ::core::ffi::c_int);
    fn Unlock_DCQ();
    fn log_add(_: log_Level, fmt: *const ::core::ffi::c_char, ...);
}
pub type size_t = usize;
pub type uint8 = ::core::ffi::c_uchar;
pub type sint16 = ::core::ffi::c_short;
pub type uint32 = ::core::ffi::c_uint;
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
pub type COORD = SWORD;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct extent {
    pub width: COORD,
    pub height: COORD,
}
pub type EXTENT = extent;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct point {
    pub x: COORD,
    pub y: COORD,
}
pub type POINT = point;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rect {
    pub corner: POINT,
    pub extent: EXTENT,
}
pub type RECT = rect;
pub type HOT_SPOT = POINT;
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
    pub const TFB_SCALE_STEP: Self = Self(0);
    pub const TFB_SCALE_NEAREST: Self = Self(1);
    pub const TFB_SCALE_BILINEAR: Self = Self(2);
    pub const TFB_SCALE_TRILINEAR: Self = Self(3);
}
pub type Mutex = *mut ::core::ffi::c_void;
pub type Semaphore = *mut ::core::ffi::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _threadLocal {
    pub flushSem: Semaphore,
}
pub type ThreadLocal = _threadLocal;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_0 {
    pub const SYNC_CLASS_TOPLEVEL: Self = Self(1);
    pub const SYNC_CLASS_AUDIO: Self = Self(2);
    pub const SYNC_CLASS_VIDEO: Self = Self(4);
    pub const SYNC_CLASS_RESOURCE: Self = Self(8);
}
pub type TFB_Canvas = *mut ::core::ffi::c_void;
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
pub type TFB_Image = tfb_image;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tfb_char {
    pub extent: EXTENT,
    pub disp: EXTENT,
    pub HotSpot: HOT_SPOT,
    pub data: *mut BYTE,
    pub pitch: DWORD,
}
pub type TFB_Char = tfb_char;
pub type TFB_DrawCommand = tfb_drawcommand;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tfb_drawcommand {
    pub Type: ::core::ffi::c_int,
    pub data: C2Rust_Unnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_1 {
    pub line: TFB_DrawCommand_Line,
    pub rect: TFB_DrawCommand_Rect,
    pub image: TFB_DrawCommand_Image,
    pub filledimage: TFB_DrawCommand_FilledImage,
    pub fontchar: TFB_DrawCommand_FontChar,
    pub copy: TFB_DrawCommand_Copy,
    pub copytoimage: TFB_DrawCommand_CopyToImage,
    pub scissor: TFB_DrawCommand_Scissor,
    pub setmipmap: TFB_DrawCommand_SetMipmap,
    pub deleteimage: TFB_DrawCommand_DeleteImage,
    pub deletedata: TFB_DrawCommand_DeleteData,
    pub sendsignal: TFB_DrawCommand_SendSignal,
    pub reinitvideo: TFB_DrawCommand_ReinitVideo,
    pub callback: TFB_DrawCommand_Callback,
}
pub type TFB_DrawCommand_Callback = tfb_dc_callback;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tfb_dc_callback {
    pub callback: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    pub arg: *mut ::core::ffi::c_void,
}
pub type TFB_DrawCommand_ReinitVideo = tfb_dc_reinit_video;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tfb_dc_reinit_video {
    pub driver: ::core::ffi::c_int,
    pub flags: ::core::ffi::c_int,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
}
pub type TFB_DrawCommand_SendSignal = tfb_dc_signal;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tfb_dc_signal {
    pub sem: Semaphore,
}
pub type TFB_DrawCommand_DeleteData = tfb_dc_deldata;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tfb_dc_deldata {
    pub data: *mut ::core::ffi::c_void,
}
pub type TFB_DrawCommand_DeleteImage = tfb_dc_delimg;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tfb_dc_delimg {
    pub image: *mut TFB_Image,
}
pub type TFB_DrawCommand_SetMipmap = tfb_dc_setmip;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tfb_dc_setmip {
    pub image: *mut TFB_Image,
    pub mipmap: *mut TFB_Image,
    pub hotx: ::core::ffi::c_int,
    pub hoty: ::core::ffi::c_int,
}
pub type TFB_DrawCommand_Scissor = tfb_dc_scissor;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tfb_dc_scissor {
    pub rect: RECT,
}
pub type TFB_DrawCommand_CopyToImage = tfb_dc_copyimg;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tfb_dc_copyimg {
    pub image: *mut TFB_Image,
    pub rect: RECT,
    pub srcBuffer: SCREEN,
}
pub type TFB_DrawCommand_Copy = tfb_dc_copy;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tfb_dc_copy {
    pub rect: RECT,
    pub srcBuffer: SCREEN,
    pub destBuffer: SCREEN,
}
pub type TFB_DrawCommand_FontChar = tfb_dc_fontchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tfb_dc_fontchar {
    pub fontchar: *mut TFB_Char,
    pub backing: *mut TFB_Image,
    pub x: ::core::ffi::c_int,
    pub y: ::core::ffi::c_int,
    pub drawMode: DrawMode,
    pub destBuffer: SCREEN,
}
pub type TFB_DrawCommand_FilledImage = tfb_dc_filledimg;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tfb_dc_filledimg {
    pub image: *mut TFB_Image,
    pub x: ::core::ffi::c_int,
    pub y: ::core::ffi::c_int,
    pub color: Color,
    pub destBuffer: SCREEN,
    pub drawMode: DrawMode,
    pub scale: ::core::ffi::c_int,
    pub scaleMode: ::core::ffi::c_int,
}
pub type TFB_DrawCommand_Image = tfb_dc_img;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tfb_dc_img {
    pub image: *mut TFB_Image,
    pub x: ::core::ffi::c_int,
    pub y: ::core::ffi::c_int,
    pub destBuffer: SCREEN,
    pub colormap: *mut TFB_ColorMap,
    pub drawMode: DrawMode,
    pub scale: ::core::ffi::c_int,
    pub scaleMode: ::core::ffi::c_int,
}
pub type TFB_DrawCommand_Rect = tfb_dc_rect;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tfb_dc_rect {
    pub rect: RECT,
    pub color: Color,
    pub drawMode: DrawMode,
    pub destBuffer: SCREEN,
}
pub type TFB_DrawCommand_Line = tfb_dc_line;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tfb_dc_line {
    pub x1: ::core::ffi::c_int,
    pub y1: ::core::ffi::c_int,
    pub x2: ::core::ffi::c_int,
    pub y2: ::core::ffi::c_int,
    pub color: Color,
    pub drawMode: DrawMode,
    pub destBuffer: SCREEN,
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_2(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_2 {
    pub const TFB_DRAWCOMMANDTYPE_LINE: Self = Self(0);
    pub const TFB_DRAWCOMMANDTYPE_RECTANGLE: Self = Self(1);
    pub const TFB_DRAWCOMMANDTYPE_IMAGE: Self = Self(2);
    pub const TFB_DRAWCOMMANDTYPE_FILLEDIMAGE: Self = Self(3);
    pub const TFB_DRAWCOMMANDTYPE_FONTCHAR: Self = Self(4);
    pub const TFB_DRAWCOMMANDTYPE_COPY: Self = Self(5);
    pub const TFB_DRAWCOMMANDTYPE_COPYTOIMAGE: Self = Self(6);
    pub const TFB_DRAWCOMMANDTYPE_SCISSORENABLE: Self = Self(7);
    pub const TFB_DRAWCOMMANDTYPE_SCISSORDISABLE: Self = Self(8);
    pub const TFB_DRAWCOMMANDTYPE_SETMIPMAP: Self = Self(9);
    pub const TFB_DRAWCOMMANDTYPE_DELETEIMAGE: Self = Self(10);
    pub const TFB_DRAWCOMMANDTYPE_DELETEDATA: Self = Self(11);
    pub const TFB_DRAWCOMMANDTYPE_SENDSIGNAL: Self = Self(12);
    pub const TFB_DRAWCOMMANDTYPE_REINITVIDEO: Self = Self(13);
    pub const TFB_DRAWCOMMANDTYPE_CALLBACK: Self = Self(14);
}
static mut NullHs: HOT_SPOT = point {
    x: 0 as COORD,
    y: 0 as COORD,
};
#[no_mangle]
pub unsafe extern "C" fn TFB_DrawScreen_Line(
    mut x1: ::core::ffi::c_int,
    mut y1: ::core::ffi::c_int,
    mut x2: ::core::ffi::c_int,
    mut y2: ::core::ffi::c_int,
    mut color: Color,
    mut mode: DrawMode,
    mut dest: SCREEN,
) {
    let mut DC: TFB_DrawCommand = TFB_DrawCommand {
        Type: 0,
        data: C2Rust_Unnamed_1 {
            line: TFB_DrawCommand_Line {
                x1: 0,
                y1: 0,
                x2: 0,
                y2: 0,
                color: Color { r: 0, g: 0, b: 0, a: 0 },
                drawMode: DrawMode { kind: 0, factor: 0 },
                destBuffer: SCREEN::TFB_SCREEN_MAIN,
            },
        },
    };
    DC.Type = C2Rust_Unnamed_2::TFB_DRAWCOMMANDTYPE_LINE.0 as ::core::ffi::c_int;
    DC.data.line.x1 = x1;
    DC.data.line.y1 = y1;
    DC.data.line.x2 = x2;
    DC.data.line.y2 = y2;
    DC.data.line.color = color;
    DC.data.line.drawMode = mode;
    DC.data.line.destBuffer = dest;
    TFB_EnqueueDrawCommand(&raw mut DC);
}
#[no_mangle]
pub unsafe extern "C" fn TFB_DrawScreen_Rect(
    mut rect: *mut RECT,
    mut color: Color,
    mut mode: DrawMode,
    mut dest: SCREEN,
) {
    let mut locRect: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut DC: TFB_DrawCommand = TFB_DrawCommand {
        Type: 0,
        data: C2Rust_Unnamed_1 {
            line: TFB_DrawCommand_Line {
                x1: 0,
                y1: 0,
                x2: 0,
                y2: 0,
                color: Color { r: 0, g: 0, b: 0, a: 0 },
                drawMode: DrawMode { kind: 0, factor: 0 },
                destBuffer: SCREEN::TFB_SCREEN_MAIN,
            },
        },
    };
    if rect.is_null() {
        locRect.corner.y = 0 as COORD;
        locRect.corner.x = locRect.corner.y;
        locRect.extent.width = ScreenWidth as COORD;
        locRect.extent.height = ScreenHeight as COORD;
        rect = &raw mut locRect;
    }
    DC.Type = C2Rust_Unnamed_2::TFB_DRAWCOMMANDTYPE_RECTANGLE.0 as ::core::ffi::c_int;
    DC.data.rect.rect = *rect;
    DC.data.rect.color = color;
    DC.data.rect.drawMode = mode;
    DC.data.rect.destBuffer = dest;
    TFB_EnqueueDrawCommand(&raw mut DC);
}
#[no_mangle]
pub unsafe extern "C" fn TFB_DrawScreen_Image(
    mut img: *mut TFB_Image,
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
    mut scale: ::core::ffi::c_int,
    mut scaleMode: ::core::ffi::c_int,
    mut cmap: *mut TFB_ColorMap,
    mut mode: DrawMode,
    mut dest: SCREEN,
) {
    let mut DC: TFB_DrawCommand = TFB_DrawCommand {
        Type: 0,
        data: C2Rust_Unnamed_1 {
            line: TFB_DrawCommand_Line {
                x1: 0,
                y1: 0,
                x2: 0,
                y2: 0,
                color: Color { r: 0, g: 0, b: 0, a: 0 },
                drawMode: DrawMode { kind: 0, factor: 0 },
                destBuffer: SCREEN::TFB_SCREEN_MAIN,
            },
        },
    };
    DC.Type = C2Rust_Unnamed_2::TFB_DRAWCOMMANDTYPE_IMAGE.0 as ::core::ffi::c_int;
    DC.data.image.image = img;
    DC.data.image.colormap = cmap;
    DC.data.image.x = x;
    DC.data.image.y = y;
    DC.data.image.scale = if scale == 256 as ::core::ffi::c_int {
        0 as ::core::ffi::c_int
    } else {
        scale
    };
    DC.data.image.scaleMode = scaleMode;
    DC.data.image.drawMode = mode;
    DC.data.image.destBuffer = dest;
    TFB_EnqueueDrawCommand(&raw mut DC);
}
#[no_mangle]
pub unsafe extern "C" fn TFB_DrawScreen_FilledImage(
    mut img: *mut TFB_Image,
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
    mut scale: ::core::ffi::c_int,
    mut scaleMode: ::core::ffi::c_int,
    mut color: Color,
    mut mode: DrawMode,
    mut dest: SCREEN,
) {
    let mut DC: TFB_DrawCommand = TFB_DrawCommand {
        Type: 0,
        data: C2Rust_Unnamed_1 {
            line: TFB_DrawCommand_Line {
                x1: 0,
                y1: 0,
                x2: 0,
                y2: 0,
                color: Color { r: 0, g: 0, b: 0, a: 0 },
                drawMode: DrawMode { kind: 0, factor: 0 },
                destBuffer: SCREEN::TFB_SCREEN_MAIN,
            },
        },
    };
    DC.Type = C2Rust_Unnamed_2::TFB_DRAWCOMMANDTYPE_FILLEDIMAGE.0 as ::core::ffi::c_int;
    DC.data.filledimage.image = img;
    DC.data.filledimage.x = x;
    DC.data.filledimage.y = y;
    DC.data.filledimage.scale = if scale == 256 as ::core::ffi::c_int {
        0 as ::core::ffi::c_int
    } else {
        scale
    };
    DC.data.filledimage.scaleMode = scaleMode;
    DC.data.filledimage.color = color;
    DC.data.filledimage.drawMode = mode;
    DC.data.filledimage.destBuffer = dest;
    TFB_EnqueueDrawCommand(&raw mut DC);
}
#[no_mangle]
pub unsafe extern "C" fn TFB_DrawScreen_FontChar(
    mut fontChar: *mut TFB_Char,
    mut backing: *mut TFB_Image,
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
    mut mode: DrawMode,
    mut dest: SCREEN,
) {
    let mut DC: TFB_DrawCommand = TFB_DrawCommand {
        Type: 0,
        data: C2Rust_Unnamed_1 {
            line: TFB_DrawCommand_Line {
                x1: 0,
                y1: 0,
                x2: 0,
                y2: 0,
                color: Color { r: 0, g: 0, b: 0, a: 0 },
                drawMode: DrawMode { kind: 0, factor: 0 },
                destBuffer: SCREEN::TFB_SCREEN_MAIN,
            },
        },
    };
    DC.Type = C2Rust_Unnamed_2::TFB_DRAWCOMMANDTYPE_FONTCHAR.0 as ::core::ffi::c_int;
    DC.data.fontchar.fontchar = fontChar;
    DC.data.fontchar.backing = backing;
    DC.data.fontchar.x = x;
    DC.data.fontchar.y = y;
    DC.data.fontchar.drawMode = mode;
    DC.data.fontchar.destBuffer = dest;
    TFB_EnqueueDrawCommand(&raw mut DC);
}
#[no_mangle]
pub unsafe extern "C" fn TFB_DrawScreen_CopyToImage(
    mut img: *mut TFB_Image,
    mut r: *const RECT,
    mut src: SCREEN,
) {
    let mut DC: TFB_DrawCommand = TFB_DrawCommand {
        Type: 0,
        data: C2Rust_Unnamed_1 {
            line: TFB_DrawCommand_Line {
                x1: 0,
                y1: 0,
                x2: 0,
                y2: 0,
                color: Color { r: 0, g: 0, b: 0, a: 0 },
                drawMode: DrawMode { kind: 0, factor: 0 },
                destBuffer: SCREEN::TFB_SCREEN_MAIN,
            },
        },
    };
    DC.Type = C2Rust_Unnamed_2::TFB_DRAWCOMMANDTYPE_COPYTOIMAGE.0 as ::core::ffi::c_int;
    DC.data.copytoimage.rect = *r;
    DC.data.copytoimage.image = img;
    DC.data.copytoimage.srcBuffer = src;
    TFB_EnqueueDrawCommand(&raw mut DC);
}
#[no_mangle]
pub unsafe extern "C" fn TFB_DrawScreen_Copy(
    mut r: *const RECT,
    mut src: SCREEN,
    mut dest: SCREEN,
) {
    let mut locRect: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut DC: TFB_DrawCommand = TFB_DrawCommand {
        Type: 0,
        data: C2Rust_Unnamed_1 {
            line: TFB_DrawCommand_Line {
                x1: 0,
                y1: 0,
                x2: 0,
                y2: 0,
                color: Color { r: 0, g: 0, b: 0, a: 0 },
                drawMode: DrawMode { kind: 0, factor: 0 },
                destBuffer: SCREEN::TFB_SCREEN_MAIN,
            },
        },
    };
    if r.is_null() {
        locRect.corner.y = 0 as COORD;
        locRect.corner.x = locRect.corner.y;
        locRect.extent.width = ScreenWidth as COORD;
        locRect.extent.height = ScreenHeight as COORD;
        r = &raw mut locRect;
    }
    DC.Type = C2Rust_Unnamed_2::TFB_DRAWCOMMANDTYPE_COPY.0 as ::core::ffi::c_int;
    DC.data.copy.rect = *r;
    DC.data.copy.srcBuffer = src;
    DC.data.copy.destBuffer = dest;
    TFB_EnqueueDrawCommand(&raw mut DC);
}
#[no_mangle]
pub unsafe extern "C" fn TFB_DrawScreen_SetMipmap(
    mut img: *mut TFB_Image,
    mut mmimg: *mut TFB_Image,
    mut hotx: ::core::ffi::c_int,
    mut hoty: ::core::ffi::c_int,
) {
    let mut DC: TFB_DrawCommand = TFB_DrawCommand {
        Type: 0,
        data: C2Rust_Unnamed_1 {
            line: TFB_DrawCommand_Line {
                x1: 0,
                y1: 0,
                x2: 0,
                y2: 0,
                color: Color { r: 0, g: 0, b: 0, a: 0 },
                drawMode: DrawMode { kind: 0, factor: 0 },
                destBuffer: SCREEN::TFB_SCREEN_MAIN,
            },
        },
    };
    DC.Type = C2Rust_Unnamed_2::TFB_DRAWCOMMANDTYPE_SETMIPMAP.0 as ::core::ffi::c_int;
    DC.data.setmipmap.image = img;
    DC.data.setmipmap.mipmap = mmimg;
    DC.data.setmipmap.hotx = hotx;
    DC.data.setmipmap.hoty = hoty;
    TFB_EnqueueDrawCommand(&raw mut DC);
}
#[no_mangle]
pub unsafe extern "C" fn TFB_DrawScreen_DeleteImage(mut img: *mut TFB_Image) {
    if !img.is_null() {
        let mut DC: TFB_DrawCommand = TFB_DrawCommand {
            Type: 0,
            data: C2Rust_Unnamed_1 {
                line: TFB_DrawCommand_Line {
                    x1: 0,
                    y1: 0,
                    x2: 0,
                    y2: 0,
                    color: Color { r: 0, g: 0, b: 0, a: 0 },
                    drawMode: DrawMode { kind: 0, factor: 0 },
                    destBuffer: SCREEN::TFB_SCREEN_MAIN,
                },
            },
        };
        DC.Type = C2Rust_Unnamed_2::TFB_DRAWCOMMANDTYPE_DELETEIMAGE.0
            as ::core::ffi::c_int;
        DC.data.deleteimage.image = img;
        TFB_EnqueueDrawCommand(&raw mut DC);
    }
}
#[no_mangle]
pub unsafe extern "C" fn TFB_DrawScreen_DeleteData(mut data: *mut ::core::ffi::c_void) {
    if !data.is_null() {
        let mut DC: TFB_DrawCommand = TFB_DrawCommand {
            Type: 0,
            data: C2Rust_Unnamed_1 {
                line: TFB_DrawCommand_Line {
                    x1: 0,
                    y1: 0,
                    x2: 0,
                    y2: 0,
                    color: Color { r: 0, g: 0, b: 0, a: 0 },
                    drawMode: DrawMode { kind: 0, factor: 0 },
                    destBuffer: SCREEN::TFB_SCREEN_MAIN,
                },
            },
        };
        DC.Type = C2Rust_Unnamed_2::TFB_DRAWCOMMANDTYPE_DELETEDATA.0
            as ::core::ffi::c_int;
        DC.data.deletedata.data = data;
        TFB_EnqueueDrawCommand(&raw mut DC);
    }
}
#[no_mangle]
pub unsafe extern "C" fn TFB_DrawScreen_WaitForSignal() {
    let mut DrawCommand: TFB_DrawCommand = TFB_DrawCommand {
        Type: 0,
        data: C2Rust_Unnamed_1 {
            line: TFB_DrawCommand_Line {
                x1: 0,
                y1: 0,
                x2: 0,
                y2: 0,
                color: Color { r: 0, g: 0, b: 0, a: 0 },
                drawMode: DrawMode { kind: 0, factor: 0 },
                destBuffer: SCREEN::TFB_SCREEN_MAIN,
            },
        },
    };
    let mut s: Semaphore = ::core::ptr::null_mut::<::core::ffi::c_void>();
    s = (*GetMyThreadLocal()).flushSem;
    DrawCommand.Type = C2Rust_Unnamed_2::TFB_DRAWCOMMANDTYPE_SENDSIGNAL.0
        as ::core::ffi::c_int;
    DrawCommand.data.sendsignal.sem = s;
    Lock_DCQ(1 as ::core::ffi::c_int);
    TFB_BatchReset();
    TFB_EnqueueDrawCommand(&raw mut DrawCommand);
    Unlock_DCQ();
    SetSemaphore(s);
}
#[no_mangle]
pub unsafe extern "C" fn TFB_DrawScreen_ReinitVideo(
    mut driver: ::core::ffi::c_int,
    mut flags: ::core::ffi::c_int,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
) {
    let mut DrawCommand: TFB_DrawCommand = TFB_DrawCommand {
        Type: 0,
        data: C2Rust_Unnamed_1 {
            line: TFB_DrawCommand_Line {
                x1: 0,
                y1: 0,
                x2: 0,
                y2: 0,
                color: Color { r: 0, g: 0, b: 0, a: 0 },
                drawMode: DrawMode { kind: 0, factor: 0 },
                destBuffer: SCREEN::TFB_SCREEN_MAIN,
            },
        },
    };
    DrawCommand.Type = C2Rust_Unnamed_2::TFB_DRAWCOMMANDTYPE_REINITVIDEO.0
        as ::core::ffi::c_int;
    DrawCommand.data.reinitvideo.driver = driver;
    DrawCommand.data.reinitvideo.flags = flags;
    DrawCommand.data.reinitvideo.width = width;
    DrawCommand.data.reinitvideo.height = height;
    TFB_EnqueueDrawCommand(&raw mut DrawCommand);
}
#[no_mangle]
pub unsafe extern "C" fn TFB_DrawScreen_Callback(
    mut callback: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    mut arg: *mut ::core::ffi::c_void,
) {
    let mut DrawCommand: TFB_DrawCommand = TFB_DrawCommand {
        Type: 0,
        data: C2Rust_Unnamed_1 {
            line: TFB_DrawCommand_Line {
                x1: 0,
                y1: 0,
                x2: 0,
                y2: 0,
                color: Color { r: 0, g: 0, b: 0, a: 0 },
                drawMode: DrawMode { kind: 0, factor: 0 },
                destBuffer: SCREEN::TFB_SCREEN_MAIN,
            },
        },
    };
    DrawCommand.Type = C2Rust_Unnamed_2::TFB_DRAWCOMMANDTYPE_CALLBACK.0
        as ::core::ffi::c_int;
    DrawCommand.data.callback.callback = callback;
    DrawCommand.data.callback.arg = arg;
    TFB_EnqueueDrawCommand(&raw mut DrawCommand);
}
#[no_mangle]
pub unsafe extern "C" fn TFB_DrawImage_Line(
    mut x1: ::core::ffi::c_int,
    mut y1: ::core::ffi::c_int,
    mut x2: ::core::ffi::c_int,
    mut y2: ::core::ffi::c_int,
    mut color: Color,
    mut mode: DrawMode,
    mut target: *mut TFB_Image,
) {
    LockMutex((*target).mutex);
    TFB_DrawCanvas_Line(x1, y1, x2, y2, color, mode, (*target).NormalImg);
    (*target).dirty = BOOLEAN::TRUE;
    UnlockMutex((*target).mutex);
}
#[no_mangle]
pub unsafe extern "C" fn TFB_DrawImage_Rect(
    mut rect: *mut RECT,
    mut color: Color,
    mut mode: DrawMode,
    mut target: *mut TFB_Image,
) {
    LockMutex((*target).mutex);
    TFB_DrawCanvas_Rect(rect, color, mode, (*target).NormalImg);
    (*target).dirty = BOOLEAN::TRUE;
    UnlockMutex((*target).mutex);
}
#[no_mangle]
pub unsafe extern "C" fn TFB_DrawImage_Image(
    mut img: *mut TFB_Image,
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
    mut scale: ::core::ffi::c_int,
    mut scaleMode: ::core::ffi::c_int,
    mut cmap: *mut TFB_ColorMap,
    mut mode: DrawMode,
    mut target: *mut TFB_Image,
) {
    LockMutex((*target).mutex);
    TFB_DrawCanvas_Image(img, x, y, scale, scaleMode, cmap, mode, (*target).NormalImg);
    (*target).dirty = BOOLEAN::TRUE;
    UnlockMutex((*target).mutex);
}
#[no_mangle]
pub unsafe extern "C" fn TFB_DrawImage_FilledImage(
    mut img: *mut TFB_Image,
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
    mut scale: ::core::ffi::c_int,
    mut scaleMode: ::core::ffi::c_int,
    mut color: Color,
    mut mode: DrawMode,
    mut target: *mut TFB_Image,
) {
    LockMutex((*target).mutex);
    TFB_DrawCanvas_FilledImage(
        img,
        x,
        y,
        scale,
        scaleMode,
        color,
        mode,
        (*target).NormalImg,
    );
    (*target).dirty = BOOLEAN::TRUE;
    UnlockMutex((*target).mutex);
}
#[no_mangle]
pub unsafe extern "C" fn TFB_DrawImage_FontChar(
    mut fontChar: *mut TFB_Char,
    mut backing: *mut TFB_Image,
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
    mut mode: DrawMode,
    mut target: *mut TFB_Image,
) {
    LockMutex((*target).mutex);
    TFB_DrawCanvas_FontChar(fontChar, backing, x, y, mode, (*target).NormalImg);
    (*target).dirty = BOOLEAN::TRUE;
    UnlockMutex((*target).mutex);
}
#[no_mangle]
pub unsafe extern "C" fn TFB_DrawImage_New(mut canvas: TFB_Canvas) -> *mut TFB_Image {
    let mut img: *mut TFB_Image = HMalloc(::core::mem::size_of::<TFB_Image>())
        as *mut TFB_Image;
    (*img).mutex = CreateMutex_Core(
        b"image lock\0".as_ptr() as *const ::core::ffi::c_char,
        C2Rust_Unnamed_0::SYNC_CLASS_VIDEO.0 as ::core::ffi::c_int as DWORD,
    );
    (*img).ScaledImg = ::core::ptr::null_mut::<::core::ffi::c_void>() as TFB_Canvas;
    (*img).MipmapImg = ::core::ptr::null_mut::<::core::ffi::c_void>() as TFB_Canvas;
    (*img).FilledImg = ::core::ptr::null_mut::<::core::ffi::c_void>() as TFB_Canvas;
    (*img).colormap_index = -1 as ::core::ffi::c_int;
    (*img).colormap_version = 0 as ::core::ffi::c_int;
    (*img).NormalHs = NullHs;
    (*img).MipmapHs = NullHs;
    (*img).last_scale_hs = NullHs;
    (*img).last_scale_type = -1 as ::core::ffi::c_int;
    (*img).last_scale = 0 as ::core::ffi::c_int;
    (*img).dirty = BOOLEAN::FALSE;
    TFB_DrawCanvas_GetExtent(canvas, &raw mut (*img).extent);
    if TFB_DrawCanvas_IsPaletted(canvas).0 != 0 {
        (*img).NormalImg = canvas;
    } else {
        (*img).NormalImg = TFB_DrawCanvas_ToScreenFormat(canvas);
    }
    return img;
}
#[no_mangle]
pub unsafe extern "C" fn TFB_DrawImage_CreateForScreen(
    mut w: ::core::ffi::c_int,
    mut h: ::core::ffi::c_int,
    mut withalpha: BOOLEAN,
) -> *mut TFB_Image {
    let mut img: *mut TFB_Image = HMalloc(::core::mem::size_of::<TFB_Image>())
        as *mut TFB_Image;
    (*img).mutex = CreateMutex_Core(
        b"image lock\0".as_ptr() as *const ::core::ffi::c_char,
        C2Rust_Unnamed_0::SYNC_CLASS_VIDEO.0 as ::core::ffi::c_int as DWORD,
    );
    (*img).ScaledImg = ::core::ptr::null_mut::<::core::ffi::c_void>() as TFB_Canvas;
    (*img).MipmapImg = ::core::ptr::null_mut::<::core::ffi::c_void>() as TFB_Canvas;
    (*img).FilledImg = ::core::ptr::null_mut::<::core::ffi::c_void>() as TFB_Canvas;
    (*img).colormap_index = -1 as ::core::ffi::c_int;
    (*img).colormap_version = 0 as ::core::ffi::c_int;
    (*img).NormalHs = NullHs;
    (*img).MipmapHs = NullHs;
    (*img).last_scale_hs = NullHs;
    (*img).last_scale_type = -1 as ::core::ffi::c_int;
    (*img).last_scale = 0 as ::core::ffi::c_int;
    (*img).extent.width = w as COORD;
    (*img).extent.height = h as COORD;
    (*img).NormalImg = TFB_DrawCanvas_New_ForScreen(w, h, withalpha);
    return img;
}
#[no_mangle]
pub unsafe extern "C" fn TFB_DrawImage_New_Rotated(
    mut img: *mut TFB_Image,
    mut angle: ::core::ffi::c_int,
) -> *mut TFB_Image {
    let mut dst: TFB_Canvas = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut size: EXTENT = EXTENT { width: 0, height: 0 };
    let mut newimg: *mut TFB_Image = ::core::ptr::null_mut::<TFB_Image>();
    if (*img).NormalImg.is_null() {
        log_add(
            log_Level::log_Warning,
            b"TFB_DrawImage_New_Rotated: source canvas is NULL! Failing.\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<TFB_Image>();
    }
    TFB_DrawCanvas_GetRotatedExtent((*img).NormalImg, angle, &raw mut size);
    dst = TFB_DrawCanvas_New_RotationTarget((*img).NormalImg, angle);
    if dst.is_null() {
        log_add(
            log_Level::log_Warning,
            b"TFB_DrawImage_New_Rotated: rotation target canvas not created! Failing.\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<TFB_Image>();
    }
    TFB_DrawCanvas_Rotate((*img).NormalImg, dst, angle, size);
    newimg = TFB_DrawImage_New(dst);
    return newimg;
}
#[no_mangle]
pub unsafe extern "C" fn TFB_DrawImage_SetMipmap(
    mut img: *mut TFB_Image,
    mut mmimg: *mut TFB_Image,
    mut hotx: ::core::ffi::c_int,
    mut hoty: ::core::ffi::c_int,
) {
    let mut imgpal: bool = false;
    let mut mmpal: bool = false;
    if img.is_null() || mmimg.is_null() {
        return;
    }
    LockMutex((*img).mutex);
    LockMutex((*mmimg).mutex);
    imgpal = TFB_DrawCanvas_IsPaletted((*img).NormalImg).0 != 0;
    mmpal = TFB_DrawCanvas_IsPaletted((*mmimg).NormalImg).0 != 0;
    if !mmpal
        || mmpal as ::core::ffi::c_int != 0 && imgpal as ::core::ffi::c_int != 0
            && (*img).colormap_index == (*mmimg).colormap_index
    {
        (*img).MipmapImg = (*mmimg).NormalImg;
        (*img).MipmapHs.x = hotx as COORD;
        (*img).MipmapHs.y = hoty as COORD;
    } else {
        (*img).MipmapImg = ::core::ptr::null_mut::<::core::ffi::c_void>() as TFB_Canvas;
    }
    UnlockMutex((*mmimg).mutex);
    UnlockMutex((*img).mutex);
}
#[no_mangle]
pub unsafe extern "C" fn TFB_DrawImage_Delete(mut image: *mut TFB_Image) {
    if image.is_null() {
        log_add(
            log_Level::log_Warning,
            b"INTERNAL ERROR: Tried to delete a null image!\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    LockMutex((*image).mutex);
    TFB_DrawCanvas_Delete((*image).NormalImg);
    if !(*image).ScaledImg.is_null() {
        TFB_DrawCanvas_Delete((*image).ScaledImg);
        (*image).ScaledImg = ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    if !(*image).FilledImg.is_null() {
        TFB_DrawCanvas_Delete((*image).FilledImg);
        (*image).FilledImg = ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    UnlockMutex((*image).mutex);
    DestroyMutex((*image).mutex);
    HFree(image as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn TFB_DrawImage_FixScaling(
    mut image: *mut TFB_Image,
    mut target: ::core::ffi::c_int,
    mut r#type: ::core::ffi::c_int,
) {
    if (*image).dirty.0 != 0 || (*image).ScaledImg.is_null()
        || target != (*image).last_scale || r#type != (*image).last_scale_type
    {
        (*image).dirty = BOOLEAN::FALSE;
        (*image).ScaledImg = TFB_DrawCanvas_New_ScaleTarget(
            (*image).NormalImg,
            (*image).ScaledImg,
            r#type,
            (*image).last_scale_type,
        );
        if r#type == C2Rust_Unnamed::TFB_SCALE_NEAREST.0 as ::core::ffi::c_int {
            TFB_DrawCanvas_Rescale_Nearest(
                (*image).NormalImg,
                (*image).ScaledImg,
                target,
                &raw mut (*image).NormalHs,
                &raw mut (*image).extent,
                &raw mut (*image).last_scale_hs,
            );
        } else if r#type == C2Rust_Unnamed::TFB_SCALE_BILINEAR.0 as ::core::ffi::c_int {
            TFB_DrawCanvas_Rescale_Bilinear(
                (*image).NormalImg,
                (*image).ScaledImg,
                target,
                &raw mut (*image).NormalHs,
                &raw mut (*image).extent,
                &raw mut (*image).last_scale_hs,
            );
        } else {
            TFB_DrawCanvas_Rescale_Trilinear(
                (*image).NormalImg,
                (*image).MipmapImg,
                (*image).ScaledImg,
                target,
                &raw mut (*image).NormalHs,
                &raw mut (*image).MipmapHs,
                &raw mut (*image).extent,
                &raw mut (*image).last_scale_hs,
            );
        }
        (*image).last_scale_type = r#type;
        (*image).last_scale = target;
    }
}
#[no_mangle]
pub unsafe extern "C" fn TFB_DrawImage_Intersect(
    mut img1: *mut TFB_Image,
    mut img1org: POINT,
    mut img2: *mut TFB_Image,
    mut img2org: POINT,
    mut interRect: *const RECT,
) -> BOOLEAN {
    let mut ret: BOOLEAN = BOOLEAN::FALSE;
    LockMutex((*img1).mutex);
    LockMutex((*img2).mutex);
    ret = TFB_DrawCanvas_Intersect(
        (*img1).NormalImg,
        img1org,
        (*img2).NormalImg,
        img2org,
        interRect,
    );
    UnlockMutex((*img2).mutex);
    UnlockMutex((*img1).mutex);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn TFB_DrawImage_CopyRect(
    mut source: *mut TFB_Image,
    mut srcRect: *const RECT,
    mut target: *mut TFB_Image,
    mut dstPt: POINT,
) {
    LockMutex((*source).mutex);
    LockMutex((*target).mutex);
    TFB_DrawCanvas_CopyRect((*source).NormalImg, srcRect, (*target).NormalImg, dstPt);
    (*target).dirty = BOOLEAN::TRUE;
    UnlockMutex((*target).mutex);
    UnlockMutex((*source).mutex);
}
