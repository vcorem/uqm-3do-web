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

// Rust translation of src/libs/graphics/dcqueue.c.
extern "C" {
    pub type font_desc;
    pub type NativePalette;
    fn GetTimeCounter() -> TimeCount;
    fn CreateRecursiveMutex_Core(
        name: *const ::core::ffi::c_char,
        syncClass: DWORD,
    ) -> RecursiveMutex;
    fn CreateCondVar_Core(name: *const ::core::ffi::c_char, syncClass: DWORD) -> CondVar;
    fn TaskSwitch();
    fn ClearSemaphore(sem: Semaphore);
    fn LockMutex(sem: Mutex);
    fn UnlockMutex(sem: Mutex);
    fn DestroyRecursiveMutex(m: RecursiveMutex);
    fn LockRecursiveMutex(m: RecursiveMutex);
    fn UnlockRecursiveMutex(m: RecursiveMutex);
    fn GetRecursiveMutexDepth(m: RecursiveMutex) -> ::core::ffi::c_int;
    fn DestroyCondVar(_: CondVar);
    fn WaitCondVar(_: CondVar);
    fn BroadcastCondVar(_: CondVar);
    fn TFB_DrawImage_SetMipmap(
        img: *mut TFB_Image,
        mmimg: *mut TFB_Image,
        hotx: ::core::ffi::c_int,
        hoty: ::core::ffi::c_int,
    );
    fn TFB_DrawImage_Delete(image: *mut TFB_Image);
    fn TFB_DrawCanvas_SetClipRect(canvas: TFB_Canvas, clipRect: *const RECT);
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
    fn TFB_GetScreenCanvas(screen: SCREEN) -> TFB_Canvas;
    fn exit(_: ::core::ffi::c_int) -> !;
    fn HFree(p: *mut ::core::ffi::c_void);
    static mut _pCurContext: CONTEXT;
    static mut _CurFramePtr: FRAME;
    fn GetFadeAmount() -> ::core::ffi::c_int;
    fn TFB_ReturnColorMap(map: *mut TFB_ColorMap);
    static mut GfxFlags: ::core::ffi::c_int;
    fn TFB_ReInitGraphics(
        driver: ::core::ffi::c_int,
        flags: ::core::ffi::c_int,
        width: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn TFB_SwapBuffers(force_full_redraw: ::core::ffi::c_int);
    static mut TransitionAmount: ::core::ffi::c_int;
    static mut ScreenWidth: ::core::ffi::c_int;
    static mut ScreenHeight: ::core::ffi::c_int;
    static mut ScreenWidthActual: ::core::ffi::c_int;
    static mut ScreenHeightActual: ::core::ffi::c_int;
    static mut GraphicsDriver: ::core::ffi::c_int;
    fn log_add(_: log_Level, fmt: *const ::core::ffi::c_char, ...);
    static mut TFB_DEBUG_HALT: ::core::ffi::c_int;
    fn TFB_BBox_RegisterPoint(x: ::core::ffi::c_int, y: ::core::ffi::c_int);
    fn TFB_BBox_RegisterRect(r: *const RECT);
    fn TFB_BBox_RegisterCanvas(
        c: TFB_Canvas,
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
    );
    fn TFB_BBox_Init(width: ::core::ffi::c_int, height: ::core::ffi::c_int);
    fn TFB_BBox_Reset();
    fn TFB_BBox_SetClipRect(r: *const RECT);
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
pub type TimeCount = DWORD;
pub type TimePeriod = DWORD;
pub type Mutex = *mut ::core::ffi::c_void;
pub type Semaphore = *mut ::core::ffi::c_void;
pub type RecursiveMutex = *mut ::core::ffi::c_void;
pub type CondVar = *mut ::core::ffi::c_void;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed {
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
pub struct Color {
    pub r: BYTE,
    pub g: BYTE,
    pub b: BYTE,
    pub a: BYTE,
}
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
pub struct DrawMode {
    pub kind: BYTE,
    pub factor: SWORD,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_0 {
    pub const TFB_REDRAW_NO: Self = Self(0);
    pub const TFB_REDRAW_FADING: Self = Self(1);
    pub const TFB_REDRAW_EXPOSE: Self = Self(2);
    pub const TFB_REDRAW_YES: Self = Self(3);
}
pub type TFB_DrawCommand_Callback = tfb_dc_callback;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tfb_dc_callback {
    pub callback: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    pub arg: *mut ::core::ffi::c_void,
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
pub type TFB_ColorMap = tfb_colormap;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tfb_colormap {
    pub index: ::core::ffi::c_int,
    pub version: ::core::ffi::c_int,
    pub refcount: ::core::ffi::c_int,
    pub next: *mut tfb_colormap,
    pub palette: *mut NativePalette,
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
pub type TFB_DrawCommand = tfb_drawcommand;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tfb_drawcommand {
    pub Type: ::core::ffi::c_int,
    pub data: C2Rust_Unnamed_1,
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
pub type TFB_DrawCommandQueue = tfb_drawcommandqueue;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tfb_drawcommandqueue {
    pub Front: ::core::ffi::c_int,
    pub Back: ::core::ffi::c_int,
    pub InsertionPoint: ::core::ffi::c_int,
    pub Batching: ::core::ffi::c_int,
    pub FullSize: ::core::ffi::c_int,
    pub Size: ::core::ffi::c_int,
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
static mut DCQ_Mutex: RecursiveMutex = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[no_mangle]
pub static mut RenderingCond: CondVar = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[no_mangle]
pub static mut DCQ: [TFB_DrawCommand; 16384] = [TFB_DrawCommand {
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
}; 16384];
#[no_mangle]
pub static mut DrawCommandQueue: TFB_DrawCommandQueue = TFB_DrawCommandQueue {
    Front: 0,
    Back: 0,
    InsertionPoint: 0,
    Batching: 0,
    FullSize: 0,
    Size: 0,
};
#[no_mangle]
pub static mut RenderedFrames: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
unsafe extern "C" fn TFB_WaitForSpace(mut requested_slots: ::core::ffi::c_int) {
    let mut old_depth: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    log_add(
        log_Level::log_Debug,
        b"DCQ overload (Size = %d, FullSize = %d, Requested = %d).  Sleeping until renderer is done.\0"
            .as_ptr() as *const ::core::ffi::c_char,
        ::core::ptr::read_volatile::<
            ::core::ffi::c_int,
        >(&raw const DrawCommandQueue.Size),
        ::core::ptr::read_volatile::<
            ::core::ffi::c_int,
        >(&raw const DrawCommandQueue.FullSize),
        requested_slots,
    );
    TFB_BatchReset();
    old_depth = GetRecursiveMutexDepth(DCQ_Mutex);
    i = 0 as ::core::ffi::c_int;
    while i < old_depth {
        UnlockRecursiveMutex(DCQ_Mutex);
        i += 1;
    }
    WaitCondVar(RenderingCond);
    i = 0 as ::core::ffi::c_int;
    while i < old_depth {
        LockRecursiveMutex(DCQ_Mutex);
        i += 1;
    }
    log_add(
        log_Level::log_Debug,
        b"DCQ clear (Size = %d, FullSize = %d).  Continuing.\0".as_ptr()
            as *const ::core::ffi::c_char,
        ::core::ptr::read_volatile::<
            ::core::ffi::c_int,
        >(&raw const DrawCommandQueue.Size),
        ::core::ptr::read_volatile::<
            ::core::ffi::c_int,
        >(&raw const DrawCommandQueue.FullSize),
    );
}
#[no_mangle]
pub unsafe extern "C" fn Lock_DCQ(mut slots: ::core::ffi::c_int) {
    LockRecursiveMutex(DCQ_Mutex);
    while ::core::ptr::read_volatile::<
        ::core::ffi::c_int,
    >(&raw const DrawCommandQueue.FullSize) >= 16384 as ::core::ffi::c_int - slots
    {
        TFB_WaitForSpace(slots);
    }
}
#[no_mangle]
pub unsafe extern "C" fn Unlock_DCQ() {
    UnlockRecursiveMutex(DCQ_Mutex);
}
unsafe extern "C" fn Synchronize_DCQ() {
    if DrawCommandQueue.Batching == 0 {
        let mut front: ::core::ffi::c_int = DrawCommandQueue.Front;
        let mut back: ::core::ffi::c_int = DrawCommandQueue.InsertionPoint;
        DrawCommandQueue.Back = DrawCommandQueue.InsertionPoint;
        if front <= back {
            ::core::ptr::write_volatile(&raw mut DrawCommandQueue.Size, back - front);
        } else {
            ::core::ptr::write_volatile(
                &raw mut DrawCommandQueue.Size,
                back + 16384 as ::core::ffi::c_int - front,
            );
        }
        ::core::ptr::write_volatile(
            &raw mut DrawCommandQueue.FullSize,
            ::core::ptr::read_volatile::<
                ::core::ffi::c_int,
            >(&raw const DrawCommandQueue.Size),
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn TFB_BatchGraphics() {
    LockRecursiveMutex(DCQ_Mutex);
    DrawCommandQueue.Batching += 1;
    UnlockRecursiveMutex(DCQ_Mutex);
}
#[no_mangle]
pub unsafe extern "C" fn TFB_UnbatchGraphics() {
    LockRecursiveMutex(DCQ_Mutex);
    if DrawCommandQueue.Batching != 0 {
        DrawCommandQueue.Batching -= 1;
    }
    Synchronize_DCQ();
    UnlockRecursiveMutex(DCQ_Mutex);
}
#[no_mangle]
pub unsafe extern "C" fn TFB_BatchReset() {
    LockRecursiveMutex(DCQ_Mutex);
    DrawCommandQueue.Batching = 0 as ::core::ffi::c_int;
    Synchronize_DCQ();
    UnlockRecursiveMutex(DCQ_Mutex);
}
#[no_mangle]
pub unsafe extern "C" fn Init_DrawCommandQueue() {
    DrawCommandQueue.Back = 0 as ::core::ffi::c_int;
    DrawCommandQueue.Front = 0 as ::core::ffi::c_int;
    DrawCommandQueue.InsertionPoint = 0 as ::core::ffi::c_int;
    DrawCommandQueue.Batching = 0 as ::core::ffi::c_int;
    ::core::ptr::write_volatile(
        &raw mut DrawCommandQueue.FullSize,
        0 as ::core::ffi::c_int,
    );
    ::core::ptr::write_volatile(&raw mut DrawCommandQueue.Size, 0 as ::core::ffi::c_int);
    TFB_BBox_Init(ScreenWidth, ScreenHeight);
    DCQ_Mutex = CreateRecursiveMutex_Core(
        b"DCQ\0".as_ptr() as *const ::core::ffi::c_char,
        (C2Rust_Unnamed::SYNC_CLASS_TOPLEVEL.0 as ::core::ffi::c_int
            | C2Rust_Unnamed::SYNC_CLASS_VIDEO.0 as ::core::ffi::c_int) as DWORD,
    );
    RenderingCond = CreateCondVar_Core(
        b"DCQ empty\0".as_ptr() as *const ::core::ffi::c_char,
        (C2Rust_Unnamed::SYNC_CLASS_TOPLEVEL.0 as ::core::ffi::c_int
            | C2Rust_Unnamed::SYNC_CLASS_VIDEO.0 as ::core::ffi::c_int) as DWORD,
    );
}
#[no_mangle]
pub unsafe extern "C" fn Uninit_DrawCommandQueue() {
    if !RenderingCond.is_null() {
        DestroyCondVar(RenderingCond);
        RenderingCond = ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    if !DCQ_Mutex.is_null() {
        DestroyRecursiveMutex(DCQ_Mutex);
        DCQ_Mutex = ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
}
#[no_mangle]
pub unsafe extern "C" fn TFB_DrawCommandQueue_Push(mut Command: *mut TFB_DrawCommand) {
    Lock_DCQ(1 as ::core::ffi::c_int);
    DCQ[DrawCommandQueue.InsertionPoint as usize] = *Command;
    DrawCommandQueue.InsertionPoint = (DrawCommandQueue.InsertionPoint
        + 1 as ::core::ffi::c_int) % 16384 as ::core::ffi::c_int;
    ::core::ptr::write_volatile(
        &raw mut DrawCommandQueue.FullSize,
        ::core::ptr::read_volatile::<
            ::core::ffi::c_int,
        >(&raw const DrawCommandQueue.FullSize) + 1,
    );
    Synchronize_DCQ();
    Unlock_DCQ();
}
#[no_mangle]
pub unsafe extern "C" fn TFB_DrawCommandQueue_Pop(
    mut target: *mut TFB_DrawCommand,
) -> ::core::ffi::c_int {
    LockRecursiveMutex(DCQ_Mutex);
    if ::core::ptr::read_volatile::<::core::ffi::c_int>(&raw const DrawCommandQueue.Size)
        == 0 as ::core::ffi::c_int
    {
        Unlock_DCQ();
        return 0 as ::core::ffi::c_int;
    }
    if DrawCommandQueue.Front == DrawCommandQueue.Back
        && ::core::ptr::read_volatile::<
            ::core::ffi::c_int,
        >(&raw const DrawCommandQueue.Size) != 16384 as ::core::ffi::c_int
    {
        log_add(
            log_Level::log_Debug,
            b"Augh!  Assertion failure in DCQ!  Front == Back, Size != DCQ_MAX\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        ::core::ptr::write_volatile(
            &raw mut DrawCommandQueue.Size,
            0 as ::core::ffi::c_int,
        );
        Unlock_DCQ();
        return 0 as ::core::ffi::c_int;
    }
    *target = DCQ[DrawCommandQueue.Front as usize];
    DrawCommandQueue.Front = (DrawCommandQueue.Front + 1 as ::core::ffi::c_int)
        % 16384 as ::core::ffi::c_int;
    ::core::ptr::write_volatile(
        &raw mut DrawCommandQueue.Size,
        ::core::ptr::read_volatile::<
            ::core::ffi::c_int,
        >(&raw const DrawCommandQueue.Size) - 1,
    );
    ::core::ptr::write_volatile(
        &raw mut DrawCommandQueue.FullSize,
        ::core::ptr::read_volatile::<
            ::core::ffi::c_int,
        >(&raw const DrawCommandQueue.FullSize) - 1,
    );
    UnlockRecursiveMutex(DCQ_Mutex);
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn TFB_DrawCommandQueue_Clear() {
    LockRecursiveMutex(DCQ_Mutex);
    ::core::ptr::write_volatile(&raw mut DrawCommandQueue.Size, 0 as ::core::ffi::c_int);
    DrawCommandQueue.Front = 0 as ::core::ffi::c_int;
    DrawCommandQueue.Back = 0 as ::core::ffi::c_int;
    DrawCommandQueue.Batching = 0 as ::core::ffi::c_int;
    ::core::ptr::write_volatile(
        &raw mut DrawCommandQueue.FullSize,
        0 as ::core::ffi::c_int,
    );
    DrawCommandQueue.InsertionPoint = 0 as ::core::ffi::c_int;
    UnlockRecursiveMutex(DCQ_Mutex);
}
unsafe extern "C" fn checkExclusiveThread(mut DrawCommand: *mut TFB_DrawCommand) {}
#[no_mangle]
pub unsafe extern "C" fn TFB_EnqueueDrawCommand(mut DrawCommand: *mut TFB_DrawCommand) {
    if TFB_DEBUG_HALT != 0 {
        return;
    }
    checkExclusiveThread(DrawCommand);
    if (*DrawCommand).Type
        <= C2Rust_Unnamed_2::TFB_DRAWCOMMANDTYPE_COPYTOIMAGE.0 as ::core::ffi::c_int
        && (*_CurFramePtr).Type as ::core::ffi::c_int == 2 as ::core::ffi::c_int
    {
        static mut scissor_rect: RECT = RECT {
            corner: POINT { x: 0, y: 0 },
            extent: EXTENT { width: 0, height: 0 },
        };
        if !_pCurContext.is_null() && !rectsEqual(scissor_rect, (*_pCurContext).ClipRect)
            || _pCurContext.is_null()
                && scissor_rect.extent.width as ::core::ffi::c_int
                    != 0 as ::core::ffi::c_int
        {
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
            if !_pCurContext.is_null() {
                scissor_rect = (*_pCurContext).ClipRect;
            } else {
                scissor_rect.extent.width = 0 as COORD;
            }
            if scissor_rect.extent.width != 0 {
                DC.Type = C2Rust_Unnamed_2::TFB_DRAWCOMMANDTYPE_SCISSORENABLE.0
                    as ::core::ffi::c_int;
                DC.data.scissor.rect = scissor_rect;
            } else {
                DC.Type = C2Rust_Unnamed_2::TFB_DRAWCOMMANDTYPE_SCISSORDISABLE.0
                    as ::core::ffi::c_int;
            }
            TFB_EnqueueDrawCommand(&raw mut DC);
        }
    }
    TFB_DrawCommandQueue_Push(DrawCommand);
}
unsafe extern "C" fn computeFPS() {
    static mut last_time: TimeCount = 0;
    static mut fps_counter: TimePeriod = 0;
    let mut current_time: TimeCount = 0;
    let mut delta_time: TimePeriod = 0;
    current_time = GetTimeCounter();
    delta_time = current_time.wrapping_sub(last_time) as TimePeriod;
    last_time = current_time;
    fps_counter = fps_counter.wrapping_add(delta_time);
    if fps_counter
        > (840 as ::core::ffi::c_int / 100 as ::core::ffi::c_int) as TimePeriod
    {
        log_add(
            log_Level::log_User,
            b"fps %.2f, effective %.2f\0".as_ptr() as *const ::core::ffi::c_char,
            (840 as ::core::ffi::c_int as ::core::ffi::c_float
                / delta_time as ::core::ffi::c_float) as ::core::ffi::c_double,
            (840 as ::core::ffi::c_int as ::core::ffi::c_float
                * RenderedFrames as ::core::ffi::c_float
                / fps_counter as ::core::ffi::c_float) as ::core::ffi::c_double,
        );
        fps_counter = 0 as TimePeriod;
        RenderedFrames = 0 as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn TFB_FlushGraphics() {
    let mut commands_handled: ::core::ffi::c_int = 0;
    let mut livelock_deterrence: BOOLEAN = BOOLEAN::FALSE;
    if ::core::ptr::read_volatile::<::core::ffi::c_int>(&raw const DrawCommandQueue.Size)
        == 0 as ::core::ffi::c_int
    {
        static mut last_fade: ::core::ffi::c_int = 255 as ::core::ffi::c_int;
        static mut last_transition: ::core::ffi::c_int = 255 as ::core::ffi::c_int;
        let mut current_fade: ::core::ffi::c_int = GetFadeAmount();
        let mut current_transition: ::core::ffi::c_int = ::core::ptr::read_volatile::<
            ::core::ffi::c_int,
        >(&raw const TransitionAmount);
        if current_fade != 255 as ::core::ffi::c_int && current_fade != last_fade
            || current_transition != 255 as ::core::ffi::c_int
                && current_transition != last_transition
            || current_fade == 255 as ::core::ffi::c_int
                && last_fade != 255 as ::core::ffi::c_int
            || current_transition == 255 as ::core::ffi::c_int
                && last_transition != 255 as ::core::ffi::c_int
        {
            TFB_SwapBuffers(C2Rust_Unnamed_0::TFB_REDRAW_FADING.0 as ::core::ffi::c_int);
        } else {
            TaskSwitch();
        }
        last_fade = current_fade;
        last_transition = current_transition;
        BroadcastCondVar(RenderingCond);
        return;
    }
    if GfxFlags & (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int != 0 {
        computeFPS();
    }
    commands_handled = 0 as ::core::ffi::c_int;
    livelock_deterrence = BOOLEAN::FALSE;
    if ::core::ptr::read_volatile::<
        ::core::ffi::c_int,
    >(&raw const DrawCommandQueue.FullSize) > 16384 as ::core::ffi::c_int
    {
        TFB_BatchReset();
    }
    if ::core::ptr::read_volatile::<::core::ffi::c_int>(&raw const DrawCommandQueue.Size)
        > 4096 as ::core::ffi::c_int
    {
        Lock_DCQ(-1 as ::core::ffi::c_int);
        livelock_deterrence = BOOLEAN::TRUE;
    }
    TFB_BBox_Reset();
    loop {
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
        if TFB_DrawCommandQueue_Pop(&raw mut DC) == 0 {
            break;
        }
        commands_handled += 1;
        if livelock_deterrence.0 == 0
            && commands_handled
                + ::core::ptr::read_volatile::<
                    ::core::ffi::c_int,
                >(&raw const DrawCommandQueue.Size) > 4096 as ::core::ffi::c_int
        {
            livelock_deterrence = BOOLEAN::TRUE;
            Lock_DCQ(-1 as ::core::ffi::c_int);
        }
        match DC.Type {
            9 => {
                let mut cmd: *mut TFB_DrawCommand_SetMipmap = &raw mut DC.data.setmipmap;
                TFB_DrawImage_SetMipmap(
                    (*cmd).image,
                    (*cmd).mipmap,
                    (*cmd).hotx,
                    (*cmd).hoty,
                );
            }
            2 => {
                let mut cmd_0: *mut TFB_DrawCommand_Image = &raw mut DC.data.image;
                let mut DC_image: *mut TFB_Image = (*cmd_0).image;
                let x: ::core::ffi::c_int = (*cmd_0).x;
                let y: ::core::ffi::c_int = (*cmd_0).y;
                TFB_DrawCanvas_Image(
                    DC_image,
                    x,
                    y,
                    (*cmd_0).scale,
                    (*cmd_0).scaleMode,
                    (*cmd_0).colormap,
                    (*cmd_0).drawMode,
                    TFB_GetScreenCanvas((*cmd_0).destBuffer),
                );
                if (*cmd_0).destBuffer.0 == SCREEN::TFB_SCREEN_MAIN.0 {
                    LockMutex((*DC_image).mutex);
                    if (*cmd_0).scale != 0 {
                        TFB_BBox_RegisterCanvas(
                            (*DC_image).ScaledImg,
                            x - (*DC_image).last_scale_hs.x as ::core::ffi::c_int,
                            y - (*DC_image).last_scale_hs.y as ::core::ffi::c_int,
                        );
                    } else {
                        TFB_BBox_RegisterCanvas(
                            (*DC_image).NormalImg,
                            x - (*DC_image).NormalHs.x as ::core::ffi::c_int,
                            y - (*DC_image).NormalHs.y as ::core::ffi::c_int,
                        );
                    }
                    UnlockMutex((*DC_image).mutex);
                }
            }
            3 => {
                let mut cmd_1: *mut TFB_DrawCommand_FilledImage = &raw mut DC
                    .data
                    .filledimage;
                let mut DC_image_0: *mut TFB_Image = (*cmd_1).image;
                let x_0: ::core::ffi::c_int = (*cmd_1).x;
                let y_0: ::core::ffi::c_int = (*cmd_1).y;
                TFB_DrawCanvas_FilledImage(
                    DC_image_0,
                    x_0,
                    y_0,
                    (*cmd_1).scale,
                    (*cmd_1).scaleMode,
                    (*cmd_1).color,
                    (*cmd_1).drawMode,
                    TFB_GetScreenCanvas((*cmd_1).destBuffer),
                );
                if (*cmd_1).destBuffer.0 == SCREEN::TFB_SCREEN_MAIN.0 {
                    LockMutex((*DC_image_0).mutex);
                    if (*cmd_1).scale != 0 {
                        TFB_BBox_RegisterCanvas(
                            (*DC_image_0).ScaledImg,
                            x_0 - (*DC_image_0).last_scale_hs.x as ::core::ffi::c_int,
                            y_0 - (*DC_image_0).last_scale_hs.y as ::core::ffi::c_int,
                        );
                    } else {
                        TFB_BBox_RegisterCanvas(
                            (*DC_image_0).NormalImg,
                            x_0 - (*DC_image_0).NormalHs.x as ::core::ffi::c_int,
                            y_0 - (*DC_image_0).NormalHs.y as ::core::ffi::c_int,
                        );
                    }
                    UnlockMutex((*DC_image_0).mutex);
                }
            }
            4 => {
                let mut cmd_2: *mut TFB_DrawCommand_FontChar = &raw mut DC.data.fontchar;
                let mut DC_char: *mut TFB_Char = (*cmd_2).fontchar;
                let x_1: ::core::ffi::c_int = (*cmd_2).x;
                let y_1: ::core::ffi::c_int = (*cmd_2).y;
                TFB_DrawCanvas_FontChar(
                    DC_char,
                    (*cmd_2).backing,
                    x_1,
                    y_1,
                    (*cmd_2).drawMode,
                    TFB_GetScreenCanvas((*cmd_2).destBuffer),
                );
                if (*cmd_2).destBuffer.0 == SCREEN::TFB_SCREEN_MAIN.0 {
                    let mut r: RECT = RECT {
                        corner: POINT { x: 0, y: 0 },
                        extent: EXTENT { width: 0, height: 0 },
                    };
                    r.corner.x = (x_1 - (*DC_char).HotSpot.x as ::core::ffi::c_int)
                        as COORD;
                    r.corner.y = (y_1 - (*DC_char).HotSpot.y as ::core::ffi::c_int)
                        as COORD;
                    r.extent.width = (*DC_char).extent.width;
                    r.extent.height = (*DC_char).extent.height;
                    TFB_BBox_RegisterRect(&raw mut r);
                }
            }
            0 => {
                let mut cmd_3: *mut TFB_DrawCommand_Line = &raw mut DC.data.line;
                if (*cmd_3).destBuffer.0 == SCREEN::TFB_SCREEN_MAIN.0 {
                    TFB_BBox_RegisterPoint((*cmd_3).x1, (*cmd_3).y1);
                    TFB_BBox_RegisterPoint((*cmd_3).x2, (*cmd_3).y2);
                }
                TFB_DrawCanvas_Line(
                    (*cmd_3).x1,
                    (*cmd_3).y1,
                    (*cmd_3).x2,
                    (*cmd_3).y2,
                    (*cmd_3).color,
                    (*cmd_3).drawMode,
                    TFB_GetScreenCanvas((*cmd_3).destBuffer),
                );
            }
            1 => {
                let mut cmd_4: *mut TFB_DrawCommand_Rect = &raw mut DC.data.rect;
                if (*cmd_4).destBuffer.0 == SCREEN::TFB_SCREEN_MAIN.0 {
                    TFB_BBox_RegisterRect(&raw mut (*cmd_4).rect);
                }
                TFB_DrawCanvas_Rect(
                    &raw mut (*cmd_4).rect,
                    (*cmd_4).color,
                    (*cmd_4).drawMode,
                    TFB_GetScreenCanvas((*cmd_4).destBuffer),
                );
            }
            7 => {
                let mut cmd_5: *mut TFB_DrawCommand_Scissor = &raw mut DC.data.scissor;
                TFB_DrawCanvas_SetClipRect(
                    TFB_GetScreenCanvas(SCREEN::TFB_SCREEN_MAIN),
                    &raw mut (*cmd_5).rect,
                );
                TFB_BBox_SetClipRect(&raw mut DC.data.scissor.rect);
            }
            8 => {
                TFB_DrawCanvas_SetClipRect(
                    TFB_GetScreenCanvas(SCREEN::TFB_SCREEN_MAIN),
                    ::core::ptr::null::<RECT>(),
                );
                TFB_BBox_SetClipRect(::core::ptr::null::<RECT>());
            }
            6 => {
                let mut cmd_6: *mut TFB_DrawCommand_CopyToImage = &raw mut DC
                    .data
                    .copytoimage;
                let mut DC_image_1: *mut TFB_Image = (*cmd_6).image;
                let dstPt: POINT = point {
                    x: 0 as COORD,
                    y: 0 as COORD,
                };
                if DC_image_1.is_null() {
                    log_add(
                        log_Level::log_Debug,
                        b"DCQ ERROR: COPYTOIMAGE passed null image ptr\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                } else {
                    LockMutex((*DC_image_1).mutex);
                    TFB_DrawCanvas_CopyRect(
                        TFB_GetScreenCanvas((*cmd_6).srcBuffer),
                        &raw mut (*cmd_6).rect,
                        (*DC_image_1).NormalImg,
                        dstPt,
                    );
                    UnlockMutex((*DC_image_1).mutex);
                }
            }
            5 => {
                let mut cmd_7: *mut TFB_DrawCommand_Copy = &raw mut DC.data.copy;
                let r_0: RECT = (*cmd_7).rect;
                if (*cmd_7).destBuffer.0 == SCREEN::TFB_SCREEN_MAIN.0 {
                    TFB_BBox_RegisterRect(&raw mut (*cmd_7).rect);
                }
                TFB_DrawCanvas_CopyRect(
                    TFB_GetScreenCanvas((*cmd_7).srcBuffer),
                    &raw const r_0,
                    TFB_GetScreenCanvas((*cmd_7).destBuffer),
                    r_0.corner,
                );
            }
            10 => {
                let mut DC_image_2: *mut TFB_Image = DC.data.deleteimage.image;
                TFB_DrawImage_Delete(DC_image_2);
            }
            11 => {
                let mut data: *mut ::core::ffi::c_void = DC.data.deletedata.data;
                HFree(data);
            }
            12 => {
                ClearSemaphore(DC.data.sendsignal.sem);
            }
            13 => {
                let mut cmd_8: *mut TFB_DrawCommand_ReinitVideo = &raw mut DC
                    .data
                    .reinitvideo;
                let mut oldDriver: ::core::ffi::c_int = GraphicsDriver;
                let mut oldFlags: ::core::ffi::c_int = GfxFlags;
                let mut oldWidth: ::core::ffi::c_int = ScreenWidthActual;
                let mut oldHeight: ::core::ffi::c_int = ScreenHeightActual;
                if TFB_ReInitGraphics(
                    (*cmd_8).driver,
                    (*cmd_8).flags,
                    (*cmd_8).width,
                    (*cmd_8).height,
                ) != 0
                {
                    log_add(
                        log_Level::log_Error,
                        b"Could not provide requested mode: reverting to last known driver.\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                    if TFB_ReInitGraphics(oldDriver, oldFlags, oldWidth, oldHeight) != 0
                    {
                        log_add(
                            log_Level::log_Fatal,
                            b"Couldn't reinit at that point either. Your video has been somehow tied in knots.\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                        );
                        exit(1 as ::core::ffi::c_int);
                    }
                }
                TFB_SwapBuffers(
                    C2Rust_Unnamed_0::TFB_REDRAW_YES.0 as ::core::ffi::c_int,
                );
            }
            14 => {
                DC
                    .data
                    .callback
                    .callback
                    .expect("non-null function pointer")(DC.data.callback.arg);
            }
            _ => {}
        }
    }
    if livelock_deterrence.0 != 0 {
        Unlock_DCQ();
    }
    TFB_SwapBuffers(C2Rust_Unnamed_0::TFB_REDRAW_NO.0 as ::core::ffi::c_int);
    RenderedFrames += 1;
    BroadcastCondVar(RenderingCond);
}
#[no_mangle]
pub unsafe extern "C" fn TFB_PurgeDanglingGraphics() {
    Lock_DCQ(-1 as ::core::ffi::c_int);
    loop {
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
        if TFB_DrawCommandQueue_Pop(&raw mut DC) == 0 {
            break;
        }
        match DC.Type {
            10 => {
                let mut DC_image: *mut TFB_Image = DC.data.deleteimage.image;
                TFB_DrawImage_Delete(DC_image);
            }
            11 => {
                let mut data: *mut ::core::ffi::c_void = DC.data.deletedata.data;
                HFree(data);
            }
            2 => {
                let mut cmap: *mut TFB_ColorMap = DC.data.image.colormap;
                if !cmap.is_null() {
                    TFB_ReturnColorMap(cmap);
                }
            }
            12 => {
                ClearSemaphore(DC.data.sendsignal.sem);
            }
            _ => {}
        }
    }
    Unlock_DCQ();
}
#[inline]
unsafe extern "C" fn pointsEqual(mut p1: POINT, mut p2: POINT) -> bool {
    return p1.x as ::core::ffi::c_int == p2.x as ::core::ffi::c_int
        && p1.y as ::core::ffi::c_int == p2.y as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn extentsEqual(mut e1: EXTENT, mut e2: EXTENT) -> bool {
    return e1.width as ::core::ffi::c_int == e2.width as ::core::ffi::c_int
        && e1.height as ::core::ffi::c_int == e2.height as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn rectsEqual(mut r1: RECT, mut r2: RECT) -> bool {
    return pointsEqual(r1.corner, r2.corner) as ::core::ffi::c_int != 0
        && extentsEqual(r1.extent, r2.extent) as ::core::ffi::c_int != 0;
}
