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

// NOTE: A lot of this code is untested. Only highlite and overlay flash
//       areas, drawing directly to the screen, using a cache, are
//       currently in use.

// NOTE:
// - If you change the properties of the original CONTEXT, specifically the
//   dimensions and origin, you'll need to call Flash_preUpdate() before and
//   Flash_postUpdate() after that change. Note that this may change which
//   part of the screen is flashing.

// TODO:
// - During a few frames during the sequence, the frame to be displayed
//   is equal to a frame which was supplied as a parameter to the flash
//   sequence (instead of generated during the sequence). It is not
//   necessary to make a copy in this case. Instead, the original can be
//   used. I had code to do this, but this doesn't work anymore with the
//   addition of startNumer, endNumer, and denom. This code is still
//   present, but disabled with BEGIN_AND_END_FRAME_EXCEPTIONS.

// Rust translation of src/uqm/flash.c.
extern "C" {
    pub type context_desc;
    pub type frame_desc;
    pub type drawable_desc;
    fn SetContext(Context: CONTEXT) -> CONTEXT;
    fn SetContextBackGroundColor(Color: Color) -> Color;
    fn SetContextFGFrame(Frame: FRAME) -> FRAME;
    fn SetContextOrigin(orgOffset: POINT) -> POINT;
    fn SetContextDrawMode(_: DrawMode) -> DrawMode;
    fn CopyContextRect(area: *const RECT) -> DRAWABLE;
    fn DrawStamp(pStamp: *mut STAMP);
    fn FlushGraphics();
    fn ClearDrawable();
    fn CreateContextAux(name: *const ::core::ffi::c_char) -> CONTEXT;
    fn CreateDrawable(
        CreateFlags: CREATE_FLAGS,
        width: SIZE,
        height: SIZE,
        num_frames: COUNT,
    ) -> DRAWABLE;
    fn DestroyDrawable(Drawable: DRAWABLE) -> BOOLEAN;
    fn GetFrameRect(Frame: FRAME, pRect: *mut RECT) -> BOOLEAN;
    fn GetFrameHot(Frame: FRAME) -> HOT_SPOT;
    fn CaptureDrawable(Drawable: DRAWABLE) -> FRAME;
    fn ReleaseDrawable(Frame: FRAME) -> DRAWABLE;
    fn HMalloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn HFree(p: *mut ::core::ffi::c_void);
    fn __assert_fail(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    );
    fn GetTimeCounter() -> TimeCount;
    fn log_add(_: log_Level, fmt: *const ::core::ffi::c_char, ...);
}
pub type size_t = usize;
pub type uint8 = ::core::ffi::c_uchar;
pub type uint16 = ::core::ffi::c_ushort;
pub type sint16 = ::core::ffi::c_short;
pub type uint32 = ::core::ffi::c_uint;
pub type BYTE = uint8;
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
pub type CONTEXT_DESC = context_desc;
pub type FRAME_DESC = frame_desc;
pub type DRAWABLE_DESC = drawable_desc;
pub type CONTEXT = *mut CONTEXT_DESC;
pub type FRAME = *mut FRAME_DESC;
pub type DRAWABLE = *mut DRAWABLE_DESC;
pub type COORD = SWORD;
pub type CREATE_FLAGS = BYTE;
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
pub struct stamp {
    pub origin: POINT,
    pub frame: FRAME,
}
pub type STAMP = stamp;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rect {
    pub corner: POINT,
    pub extent: EXTENT,
}
pub type RECT = rect;
pub type HOT_SPOT = POINT;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct DrawKind(pub ::core::ffi::c_uint);
impl DrawKind {
    pub const DRAW_REPLACE: Self = Self(0);
    pub const DRAW_ADDITIVE: Self = Self(1);
    pub const DRAW_ALPHA: Self = Self(2);
    pub const DRAW_DEFAULT: Self = Self(0);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DrawMode {
    pub kind: BYTE,
    pub factor: SWORD,
}
pub type TimeCount = DWORD;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct FlashState(pub ::core::ffi::c_uint);
impl FlashState {
    pub const FlashState_fadeIn: Self = Self(0);
    pub const FlashState_on: Self = Self(1);
    pub const FlashState_fadeOut: Self = Self(2);
    pub const FlashState_off: Self = Self(3);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FlashContext {
    pub gfxContext: CONTEXT,
    pub rect: RECT,
    pub original: FRAME,
    pub r#type: FlashType,
    pub u: C2Rust_Unnamed,
    pub startNumer: ::core::ffi::c_int,
    pub endNumer: ::core::ffi::c_int,
    pub denom: ::core::ffi::c_int,
    pub fadeInTime: TimeCount,
    pub onTime: TimeCount,
    pub fadeOutTime: TimeCount,
    pub offTime: TimeCount,
    pub frameTime: TimeCount,
    pub state: FlashState,
    pub lastStateTime: TimeCount,
    pub lastFrameTime: TimeCount,
    pub started: BOOLEAN,
    pub paused: BOOLEAN,
    pub cache: *mut FRAME,
    pub cacheSize: COUNT,
    pub lastFrameIndex: COUNT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed {
    pub transition: C2Rust_Unnamed_1,
    pub overlay: C2Rust_Unnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_0 {
    pub frame: FRAME,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_1 {
    pub first: FRAME,
    pub r#final: FRAME,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct FlashType(pub ::core::ffi::c_uint);
impl FlashType {
    pub const FlashType_highlight: Self = Self(0);
    pub const FlashType_transition: Self = Self(1);
    pub const FlashType_overlay: Self = Self(2);
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
unsafe extern "C" fn MAKE_DRAW_MODE(mut kind: DrawKind, mut factor: SWORD) -> DrawMode {
    let mut mode: DrawMode = DrawMode { kind: 0, factor: 0 };
    mode.kind = kind.0 as BYTE;
    mode.factor = factor;
    return mode;
}
static mut workGfxContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
unsafe extern "C" fn Flash_create(mut gfxContext: CONTEXT) -> *mut FlashContext {
    let mut context: *mut FlashContext = HMalloc(::core::mem::size_of::<FlashContext>())
        as *mut FlashContext;
    (*context).gfxContext = gfxContext;
    (*context).original = ::core::ptr::null_mut::<FRAME_DESC>();
    (*context).startNumer = 0 as ::core::ffi::c_int;
    (*context).endNumer = 1 as ::core::ffi::c_int;
    (*context).denom = 1 as ::core::ffi::c_int;
    (*context).fadeInTime = 0 as TimeCount;
    (*context).onTime = (840 as ::core::ffi::c_int / 8 as ::core::ffi::c_int)
        as TimeCount;
    (*context).fadeOutTime = 0 as TimeCount;
    (*context).offTime = (840 as ::core::ffi::c_int / 8 as ::core::ffi::c_int)
        as TimeCount;
    (*context).frameTime = 0 as TimeCount;
    (*context).state = FlashState::FlashState_off;
    (*context).lastStateTime = 0 as TimeCount;
    (*context).lastFrameTime = 0 as TimeCount;
    (*context).started = BOOLEAN(0 as ::core::ffi::c_int as ::core::ffi::c_uint);
    (*context).paused = BOOLEAN(0 as ::core::ffi::c_int as ::core::ffi::c_uint);
    (*context).cache = ::core::ptr::null_mut::<FRAME>();
    (*context).cacheSize = 9 as COUNT;
    (*context).lastFrameIndex = -1 as ::core::ffi::c_int as COUNT;
    if workGfxContext.is_null() {
        workGfxContext = CreateContextAux(
            b"Flash.workGfxContext\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    return context;
}
#[no_mangle]
pub unsafe extern "C" fn Flash_createHighlight(
    mut gfxContext: CONTEXT,
    mut rect: *const RECT,
) -> *mut FlashContext {
    let mut context: *mut FlashContext = Flash_create(gfxContext);
    if rect.is_null() {
        (*context).rect.corner.x = 0 as COORD;
        (*context).rect.corner.y = 0 as COORD;
        (*context).rect.extent.width = 0 as COORD;
        (*context).rect.extent.height = 0 as COORD;
    } else {
        (*context).rect = *rect;
    }
    (*context).r#type = FlashType::FlashType_highlight;
    return context;
}
#[no_mangle]
pub unsafe extern "C" fn Flash_createTransition(
    mut gfxContext: CONTEXT,
    mut origin: *const POINT,
    mut first: FRAME,
    mut r#final: FRAME,
) -> *mut FlashContext {
    let mut context: *mut FlashContext = Flash_create(gfxContext);
    (*context).r#type = FlashType::FlashType_transition;
    (*context).u.transition.first = first;
    (*context).u.transition.r#final = r#final;
    GetFrameRect(r#final, &raw mut (*context).rect);
    (*context).rect.corner = *origin;
    return context;
}
#[no_mangle]
pub unsafe extern "C" fn Flash_createOverlay(
    mut gfxContext: CONTEXT,
    mut origin: *const POINT,
    mut overlay: FRAME,
) -> *mut FlashContext {
    let mut context: *mut FlashContext = Flash_create(gfxContext);
    (*context).r#type = FlashType::FlashType_overlay;
    if origin.is_null() || overlay.is_null() {
        (*context).u.overlay.frame = ::core::ptr::null_mut::<FRAME_DESC>();
        (*context).rect.corner.x = 0 as COORD;
        (*context).rect.corner.y = 0 as COORD;
        (*context).rect.extent.width = 0 as COORD;
        (*context).rect.extent.height = 0 as COORD;
    } else {
        Flash_setOverlay(context, origin, overlay);
    }
    return context;
}
#[no_mangle]
pub unsafe extern "C" fn Flash_setState(
    mut context: *mut FlashContext,
    mut state: FlashState,
    mut timeSpentInState: TimeCount,
) {
    let mut now: TimeCount = 0;
    now = GetTimeCounter();
    (*context).state = state;
    Flash_fixState(context);
    (*context).lastStateTime = now.wrapping_sub(timeSpentInState);
    (*context).lastFrameTime = now;
    if (*context).started.0 != 0 {
        Flash_drawCurrentFrame(context);
    }
}
#[no_mangle]
pub unsafe extern "C" fn Flash_start(mut context: *mut FlashContext) {
    if (*context).started.0 != 0 {
        log_add(
            log_Level::log_Warning,
            b"Flash_start() called on already started FlashContext.\n\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    Flash_initCache(context);
    (*context).started = BOOLEAN(1 as ::core::ffi::c_int as ::core::ffi::c_uint);
    (*context).paused = BOOLEAN(0 as ::core::ffi::c_int as ::core::ffi::c_uint);
    Flash_grabOriginal(context);
    (*context).lastFrameIndex = 0 as COUNT;
    Flash_fixState(context);
    Flash_drawCurrentFrame(context);
}
#[no_mangle]
pub unsafe extern "C" fn Flash_terminate(mut context: *mut FlashContext) {
    if (*context).started.0 != 0 {
        Flash_drawFrame(context, (*context).original);
        Flash_clearCache(context);
        HFree((*context).cache as *mut ::core::ffi::c_void);
        DestroyDrawable(ReleaseDrawable((*context).original));
    }
    HFree(context as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn Flash_pause(mut context: *mut FlashContext) {
    (*context).paused = BOOLEAN(1 as ::core::ffi::c_int as ::core::ffi::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn Flash_continue(mut context: *mut FlashContext) {
    (*context).paused = BOOLEAN(0 as ::core::ffi::c_int as ::core::ffi::c_uint);
}
unsafe extern "C" fn Flash_fixState(mut context: *mut FlashContext) {
    let mut stateTime: TimeCount = 0 as TimeCount;
    loop {
        match (*context).state {
            FlashState::FlashState_fadeIn => {
                stateTime = (*context).fadeInTime;
            }
            FlashState::FlashState_on => {
                stateTime = (*context).onTime;
            }
            FlashState::FlashState_fadeOut => {
                stateTime = (*context).fadeOutTime;
            }
            FlashState::FlashState_off => {
                stateTime = (*context).offTime;
            }
            _ => {}
        }
        if stateTime != 0 as TimeCount {
            break;
        }
        (*context).state = FlashState(
            (*context).state.0.wrapping_add(1 as ::core::ffi::c_uint)
                & 0x3 as ::core::ffi::c_uint,
        );
    };
}
unsafe extern "C" fn Flash_nextState(mut context: *mut FlashContext) {
    (*context).state = FlashState(
        (*context).state.0.wrapping_add(1 as ::core::ffi::c_uint)
            & 0x3 as ::core::ffi::c_uint,
    );
    Flash_fixState(context);
}
#[no_mangle]
pub unsafe extern "C" fn Flash_process(mut context: *mut FlashContext) {
    let mut now: TimeCount = 0;
    if (*context).started.0 == 0 || (*context).paused.0 != 0 {
        return;
    }
    now = GetTimeCounter();
    match (*context).state {
        FlashState::FlashState_fadeIn => {
            if now >= (*context).lastStateTime.wrapping_add((*context).fadeInTime) {
                Flash_nextState(context);
                (*context).lastStateTime = now;
            }
            (*context).lastFrameTime = now;
        }
        FlashState::FlashState_on => {
            if now < (*context).lastStateTime.wrapping_add((*context).onTime) {
                return;
            }
            Flash_nextState(context);
            (*context).lastStateTime = now;
        }
        FlashState::FlashState_fadeOut => {
            if now >= (*context).lastStateTime.wrapping_add((*context).fadeOutTime) {
                Flash_nextState(context);
                (*context).lastStateTime = now;
            }
            (*context).lastFrameTime = now;
        }
        FlashState::FlashState_off => {
            if now < (*context).lastStateTime.wrapping_add((*context).offTime) {
                return;
            }
            Flash_nextState(context);
            (*context).lastStateTime = now;
        }
        _ => {}
    }
    Flash_drawCurrentFrame(context);
}
#[no_mangle]
pub unsafe extern "C" fn Flash_setSpeed(
    mut context: *mut FlashContext,
    mut fadeInTime: TimeCount,
    mut onTime: TimeCount,
    mut fadeOutTime: TimeCount,
    mut offTime: TimeCount,
) {
    (*context).fadeInTime = fadeInTime;
    (*context).onTime = onTime;
    (*context).fadeOutTime = fadeOutTime;
    (*context).offTime = offTime;
}
#[no_mangle]
pub unsafe extern "C" fn Flash_setMergeFactors(
    mut context: *mut FlashContext,
    mut startNumer: ::core::ffi::c_int,
    mut endNumer: ::core::ffi::c_int,
    mut denom: ::core::ffi::c_int,
) {
    if (*context).started.0 != 0 {
        Flash_drawFrame(context, (*context).original);
        Flash_clearCache(context);
    }
    (*context).startNumer = startNumer;
    (*context).endNumer = endNumer;
    (*context).denom = denom;
    if (*context).started.0 != 0 {
        Flash_grabOriginal(context);
        Flash_drawCurrentFrame(context);
    }
}
#[no_mangle]
pub unsafe extern "C" fn Flash_setFrameTime(
    mut context: *mut FlashContext,
    mut frameTime: TimeCount,
) {
    (*context).frameTime = frameTime;
}
#[no_mangle]
pub unsafe extern "C" fn Flash_nextTime(mut context: *mut FlashContext) -> TimeCount {
    if (*context).started.0 == 0 || (*context).paused.0 != 0 {
        return -1 as ::core::ffi::c_int as TimeCount;
    }
    if (*context).state.0 == FlashState::FlashState_fadeIn.0
        || (*context).state.0 == FlashState::FlashState_fadeOut.0
    {
        return (*context).lastFrameTime.wrapping_add((*context).frameTime)
    } else if (*context).state.0 == FlashState::FlashState_on.0 {
        return (*context).lastStateTime.wrapping_add((*context).onTime)
    } else {
        return (*context).lastStateTime.wrapping_add((*context).offTime)
    };
}
unsafe extern "C" fn Flash_clearCache(mut context: *mut FlashContext) {
    let mut i: COUNT = 0;
    i = 0 as COUNT;
    while (i as ::core::ffi::c_int) < (*context).cacheSize as ::core::ffi::c_int {
        if !(*(*context).cache.offset(i as isize)).is_null() {
            DestroyDrawable(ReleaseDrawable(*(*context).cache.offset(i as isize)));
            *(*context).cache.offset(i as isize) = ::core::ptr::null_mut::<FRAME_DESC>();
        }
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn Flash_setRect(
    mut context: *mut FlashContext,
    mut rect: *const RECT,
) {
    ((*context).r#type.0 == FlashType::FlashType_highlight.0
        || {
            __assert_fail(
                b"context->type == FlashType_highlight\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/uqm/flash.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                430 as ::core::ffi::c_int,
                b"Flash_setRect\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    if (*context).started.0 != 0 {
        Flash_drawFrame(context, (*context).original);
        Flash_clearCache(context);
    }
    (*context).rect = *rect;
    (*context).lastFrameIndex = -1 as ::core::ffi::c_int as COUNT;
    if (*context).started.0 != 0 {
        Flash_grabOriginal(context);
        Flash_drawCurrentFrame(context);
    }
}
#[no_mangle]
pub unsafe extern "C" fn Flash_getRect(
    mut context: *mut FlashContext,
    mut rect: *mut RECT,
) {
    (((*context).r#type.0 == 0) as ::core::ffi::c_int
        == FlashType::FlashType_highlight.0 as ::core::ffi::c_int
        || {
            __assert_fail(
                b"!context->type == FlashType_highlight\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/uqm/flash.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                451 as ::core::ffi::c_int,
                b"Flash_getRect\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    *rect = (*context).rect;
}
#[no_mangle]
pub unsafe extern "C" fn Flash_setOverlay(
    mut context: *mut FlashContext,
    mut origin: *const POINT,
    mut overlay: FRAME,
) {
    ((*context).r#type.0 == FlashType::FlashType_overlay.0
        || {
            __assert_fail(
                b"context->type == FlashType_overlay\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/uqm/flash.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                459 as ::core::ffi::c_int,
                b"Flash_setOverlay\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    if (*context).started.0 != 0 {
        Flash_drawFrame(context, (*context).original);
        Flash_clearCache(context);
    }
    (*context).u.overlay.frame = overlay;
    GetFrameRect(overlay, &raw mut (*context).rect);
    (*context).rect.corner.x = ((*context).rect.corner.x as ::core::ffi::c_int
        + (*origin).x as ::core::ffi::c_int) as COORD;
    (*context).rect.corner.y = ((*context).rect.corner.y as ::core::ffi::c_int
        + (*origin).y as ::core::ffi::c_int) as COORD;
    if (*context).started.0 != 0 {
        Flash_grabOriginal(context);
        Flash_drawCurrentFrame(context);
    }
}
#[no_mangle]
pub unsafe extern "C" fn Flash_preUpdate(mut context: *mut FlashContext) {
    if (*context).started.0 != 0 {
        Flash_drawFrame(context, (*context).original);
        Flash_clearCache(context);
    }
}
#[no_mangle]
pub unsafe extern "C" fn Flash_postUpdate(mut context: *mut FlashContext) {
    if (*context).started.0 != 0 {
        Flash_grabOriginal(context);
        Flash_drawCurrentFrame(context);
    }
}
unsafe extern "C" fn Flash_initCache(mut context: *mut FlashContext) {
    let mut i: COUNT = 0;
    (*context).cache = HMalloc(
        ((*context).cacheSize as size_t).wrapping_mul(::core::mem::size_of::<FRAME>()),
    ) as *mut FRAME;
    i = 0 as COUNT;
    while (i as ::core::ffi::c_int) < (*context).cacheSize as ::core::ffi::c_int {
        *(*context).cache.offset(i as isize) = ::core::ptr::null_mut::<FRAME_DESC>();
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn Flash_setCacheSize(
    mut context: *mut FlashContext,
    mut size: COUNT,
) {
    (size as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        || size as ::core::ffi::c_int >= 2 as ::core::ffi::c_int
        || {
            __assert_fail(
                b"size == 0 || size >= 2\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/uqm/flash.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                517 as ::core::ffi::c_int,
                b"Flash_setCacheSize\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    if !(*context).cache.is_null() {
        Flash_clearCache(context);
        HFree((*context).cache as *mut ::core::ffi::c_void);
        (*context).cache = ::core::ptr::null_mut::<FRAME>();
    }
    (*context).cacheSize = size;
    if size as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
        Flash_initCache(context);
    }
}
#[no_mangle]
pub unsafe extern "C" fn Flash_getCacheSize(mut context: *const FlashContext) -> COUNT {
    return (*context).cacheSize;
}
unsafe extern "C" fn Flash_grabOriginal(mut context: *mut FlashContext) {
    let mut oldGfxContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    if !(*context).original.is_null() {
        DestroyDrawable(ReleaseDrawable((*context).original));
    }
    oldGfxContext = SetContext((*context).gfxContext);
    (*context).original = CaptureDrawable(CopyContextRect(&raw mut (*context).rect));
    SetContext(oldGfxContext);
    FlushGraphics();
}
#[inline]
unsafe extern "C" fn Flash_blendFraction(
    mut context: *mut FlashContext,
    mut numer: ::core::ffi::c_int,
    mut denom: ::core::ffi::c_int,
    mut resNumer: *mut ::core::ffi::c_int,
    mut resDenom: *mut ::core::ffi::c_int,
) {
    (numer >= 0 as ::core::ffi::c_int && numer <= denom
        || {
            __assert_fail(
                b"numer >= 0 && numer <= denom\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/uqm/flash.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                568 as ::core::ffi::c_int,
                b"Flash_blendFraction\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    *resNumer = numer * (*context).endNumer + (denom - numer) * (*context).startNumer;
    *resDenom = denom * (*context).denom;
}
unsafe extern "C" fn Flash_makeFrame(
    mut context: *mut FlashContext,
    mut dest: FRAME,
    mut numer: ::core::ffi::c_int,
    mut denom: ::core::ffi::c_int,
) {
    let mut oldGfxContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    let mut s: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    let mut blendedNumer: ::core::ffi::c_int = 0;
    let mut blendedDenom: ::core::ffi::c_int = 0;
    s.origin.x = 0 as COORD;
    s.origin.y = 0 as COORD;
    Flash_blendFraction(
        context,
        numer,
        denom,
        &raw mut blendedNumer,
        &raw mut blendedDenom,
    );
    oldGfxContext = SetContext(workGfxContext);
    SetContextFGFrame(dest);
    match (*context).r#type {
        FlashType::FlashType_highlight => {
            SetContextBackGroundColor(
                buildColorRgba(0 as BYTE, 0 as BYTE, 0 as BYTE, 0 as BYTE),
            );
            ClearDrawable();
            SetContextDrawMode(
                MAKE_DRAW_MODE(
                    DrawKind::DRAW_ADDITIVE,
                    (0xff as ::core::ffi::c_int * blendedNumer / blendedDenom) as SWORD,
                ),
            );
            s.frame = (*context).original;
            DrawStamp(&raw mut s);
        }
        FlashType::FlashType_transition => {
            let mut first: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
            let mut r#final: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
            first = (*context).u.transition.first;
            if first.is_null() {
                first = (*context).original;
            }
            r#final = (*context).u.transition.r#final;
            if r#final.is_null() {
                r#final = (*context).original;
            }
            SetContextDrawMode(MAKE_DRAW_MODE(DrawKind::DRAW_REPLACE, 0 as SWORD));
            s.frame = first;
            DrawStamp(&raw mut s);
            SetContextDrawMode(
                MAKE_DRAW_MODE(
                    DrawKind::DRAW_ALPHA,
                    (0xff as ::core::ffi::c_int * blendedNumer / blendedDenom) as SWORD,
                ),
            );
            s.frame = r#final;
            DrawStamp(&raw mut s);
        }
        FlashType::FlashType_overlay => {
            let mut oldOrigin: POINT = POINT { x: 0, y: 0 };
            SetContextDrawMode(MAKE_DRAW_MODE(DrawKind::DRAW_REPLACE, 0 as SWORD));
            s.frame = (*context).original;
            DrawStamp(&raw mut s);
            SetContextDrawMode(
                MAKE_DRAW_MODE(
                    DrawKind::DRAW_ADDITIVE,
                    (0xff as ::core::ffi::c_int * blendedNumer / blendedDenom) as SWORD,
                ),
            );
            s.frame = (*context).u.overlay.frame;
            oldOrigin = SetContextOrigin(GetFrameHot(s.frame));
            DrawStamp(&raw mut s);
            SetContextOrigin(oldOrigin);
        }
        _ => {}
    }
    SetContext(oldGfxContext);
}
#[inline]
unsafe extern "C" fn Flash_prepareCacheFrame(
    mut context: *mut FlashContext,
    mut index: COUNT,
) {
    if !(*(*context).cache.offset(index as isize)).is_null() {
        return;
    }
    *(*context).cache.offset(index as isize) = CaptureDrawable(
        CreateDrawable(
            ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as CREATE_FLAGS,
            (*context).rect.extent.width,
            (*context).rect.extent.height,
            1 as COUNT,
        ),
    );
    Flash_makeFrame(
        context,
        *(*context).cache.offset(index as isize),
        index as ::core::ffi::c_int,
        (*context).cacheSize as ::core::ffi::c_int - 1 as ::core::ffi::c_int,
    );
}
unsafe extern "C" fn Flash_drawFrame(mut context: *mut FlashContext, mut frame: FRAME) {
    let mut oldGfxContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    let mut stamp: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    oldGfxContext = SetContext((*context).gfxContext);
    stamp.origin = (*context).rect.corner;
    stamp.frame = frame;
    DrawStamp(&raw mut stamp);
    SetContext(oldGfxContext);
}
unsafe extern "C" fn Flash_drawCacheFrame(
    mut context: *mut FlashContext,
    mut index: COUNT,
) {
    let mut frame: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
    if (*context).lastFrameIndex as ::core::ffi::c_int == index as ::core::ffi::c_int {
        return;
    }
    frame = *(*context).cache.offset(index as isize);
    Flash_drawFrame(context, frame);
    (*context).lastFrameIndex = index;
}
#[inline]
unsafe extern "C" fn Flash_drawUncachedFrame(
    mut context: *mut FlashContext,
    mut numer: ::core::ffi::c_int,
    mut denom: ::core::ffi::c_int,
) {
    let mut work: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
    work = CaptureDrawable(
        CreateDrawable(
            ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as CREATE_FLAGS,
            (*context).rect.extent.width,
            (*context).rect.extent.height,
            1 as COUNT,
        ),
    );
    Flash_makeFrame(context, work, numer, denom);
    Flash_drawFrame(context, work);
    DestroyDrawable(ReleaseDrawable(work));
}
#[inline]
unsafe extern "C" fn Flash_drawCachedFrame(
    mut context: *mut FlashContext,
    mut numer: ::core::ffi::c_int,
    mut denom: ::core::ffi::c_int,
) {
    let mut cachePos: COUNT = 0;
    cachePos = ((((*context).cacheSize as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
        * numer + denom / 2 as ::core::ffi::c_int) / denom) as COUNT;
    Flash_prepareCacheFrame(context, cachePos);
    Flash_drawCacheFrame(context, cachePos);
}
unsafe extern "C" fn Flash_drawCurrentFrame(mut context: *mut FlashContext) {
    let mut numer: ::core::ffi::c_int = 0;
    let mut denom: ::core::ffi::c_int = 0;
    if (*context).state.0 == FlashState::FlashState_off.0 {
        numer = 0 as ::core::ffi::c_int;
        denom = 1 as ::core::ffi::c_int;
    } else if (*context).state.0 == FlashState::FlashState_on.0 {
        numer = 1 as ::core::ffi::c_int;
        denom = 1 as ::core::ffi::c_int;
    } else {
        let mut now: TimeCount = GetTimeCounter();
        if (*context).state.0 == FlashState::FlashState_fadeIn.0 {
            denom = (*context).fadeInTime as ::core::ffi::c_int;
        } else {
            denom = (*context).fadeOutTime as ::core::ffi::c_int;
        }
        numer = now.wrapping_sub((*context).lastStateTime) as ::core::ffi::c_int;
        if numer > denom {
            numer = denom;
        }
        if (*context).state.0 == FlashState::FlashState_fadeOut.0 {
            numer = (*context).fadeOutTime as ::core::ffi::c_int - numer;
        }
    }
    if (*context).cacheSize as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        Flash_drawUncachedFrame(context, numer, denom);
    } else {
        Flash_drawCachedFrame(context, numer, denom);
    };
}
