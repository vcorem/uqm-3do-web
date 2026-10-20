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

// Rust translation of src/libs/graphics/gfx_common.c.
extern "C" {
    fn __assert_fail(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    );
    fn TFB_DrawScreen_Copy(r: *const RECT, src: SCREEN, dest: SCREEN);
    fn TFB_DrawScreen_WaitForSignal();
    fn SleepThread(timePeriod: TimePeriod);
    fn GetTimeCounter() -> TimeCount;
    fn TFB_UploadTransitionScreen();
    fn TFB_BatchGraphics();
    fn TFB_UnbatchGraphics();
}
pub type sint16 = ::core::ffi::c_short;
pub type uint32 = ::core::ffi::c_uint;
pub type SWORD = sint16;
pub type DWORD = uint32;
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
pub type TimeCount = DWORD;
pub type TimePeriod = DWORD;
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
pub struct C2Rust_Unnamed(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed {
    pub const TFB_SCALE_STEP: Self = Self(0);
    pub const TFB_SCALE_NEAREST: Self = Self(1);
    pub const TFB_SCALE_BILINEAR: Self = Self(2);
    pub const TFB_SCALE_TRILINEAR: Self = Self(3);
}
#[no_mangle]
pub static mut ScreenWidth: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut ScreenHeight: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut ScreenWidthActual: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut ScreenHeightActual: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut ScreenColorDepth: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut GraphicsDriver: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut TFB_DEBUG_HALT: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub static mut TransitionAmount: ::core::ffi::c_int = 255 as ::core::ffi::c_int;
#[no_mangle]
pub static mut TransitionClipRect: RECT = RECT {
    corner: POINT { x: 0, y: 0 },
    extent: EXTENT { width: 0, height: 0 },
};
static mut gscale: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
static mut gscale_mode: ::core::ffi::c_int = C2Rust_Unnamed::TFB_SCALE_NEAREST.0
    as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn DrawFromExtraScreen(mut r: *mut RECT) {
    TFB_DrawScreen_Copy(r, SCREEN::TFB_SCREEN_EXTRA, SCREEN::TFB_SCREEN_MAIN);
}
#[no_mangle]
pub unsafe extern "C" fn LoadIntoExtraScreen(mut r: *mut RECT) {
    TFB_DrawScreen_Copy(r, SCREEN::TFB_SCREEN_MAIN, SCREEN::TFB_SCREEN_EXTRA);
}
#[no_mangle]
pub unsafe extern "C" fn SetGraphicScale(
    mut scale: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut old_scale: ::core::ffi::c_int = gscale;
    gscale = if scale != 0 { scale } else { 256 as ::core::ffi::c_int };
    return old_scale;
}
#[no_mangle]
pub unsafe extern "C" fn GetGraphicScale() -> ::core::ffi::c_int {
    return gscale;
}
#[no_mangle]
pub unsafe extern "C" fn SetGraphicScaleMode(
    mut mode: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut old_mode: ::core::ffi::c_int = gscale_mode;
    (mode >= C2Rust_Unnamed::TFB_SCALE_NEAREST.0 as ::core::ffi::c_int
        && mode <= C2Rust_Unnamed::TFB_SCALE_TRILINEAR.0 as ::core::ffi::c_int
        || {
            __assert_fail(
                b"mode >= TFB_SCALE_NEAREST && mode <= TFB_SCALE_TRILINEAR\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/libs/graphics/gfx_common.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                71 as ::core::ffi::c_int,
                b"SetGraphicScaleMode\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    gscale_mode = mode;
    return old_mode;
}
#[no_mangle]
pub unsafe extern "C" fn GetGraphicScaleMode() -> ::core::ffi::c_int {
    return gscale_mode;
}
#[no_mangle]
pub unsafe extern "C" fn BatchGraphics() {
    TFB_BatchGraphics();
}
#[no_mangle]
pub unsafe extern "C" fn UnbatchGraphics() {
    TFB_UnbatchGraphics();
}
#[no_mangle]
pub unsafe extern "C" fn FlushGraphics() {
    TFB_DrawScreen_WaitForSignal();
}
unsafe extern "C" fn ExpandRect(mut rect: *mut RECT, mut expansion: ::core::ffi::c_int) {
    if (*rect).corner.x as ::core::ffi::c_int - expansion >= 0 as ::core::ffi::c_int {
        (*rect).extent.width = ((*rect).extent.width as ::core::ffi::c_int + expansion)
            as COORD;
        (*rect).corner.x = ((*rect).corner.x as ::core::ffi::c_int - expansion) as COORD;
    } else {
        (*rect).extent.width = ((*rect).extent.width as ::core::ffi::c_int
            + (*rect).corner.x as ::core::ffi::c_int) as COORD;
        (*rect).corner.x = 0 as COORD;
    }
    if (*rect).corner.y as ::core::ffi::c_int - expansion >= 0 as ::core::ffi::c_int {
        (*rect).extent.height = ((*rect).extent.height as ::core::ffi::c_int + expansion)
            as COORD;
        (*rect).corner.y = ((*rect).corner.y as ::core::ffi::c_int - expansion) as COORD;
    } else {
        (*rect).extent.height = ((*rect).extent.height as ::core::ffi::c_int
            + (*rect).corner.y as ::core::ffi::c_int) as COORD;
        (*rect).corner.y = 0 as COORD;
    }
    if (*rect).corner.x as ::core::ffi::c_int
        + (*rect).extent.width as ::core::ffi::c_int + expansion <= ScreenWidth
    {
        (*rect).extent.width = ((*rect).extent.width as ::core::ffi::c_int + expansion)
            as COORD;
    } else {
        (*rect).extent.width = (ScreenWidth - (*rect).corner.x as ::core::ffi::c_int)
            as COORD;
    }
    if (*rect).corner.y as ::core::ffi::c_int
        + (*rect).extent.height as ::core::ffi::c_int + expansion <= ScreenHeight
    {
        (*rect).extent.height = ((*rect).extent.height as ::core::ffi::c_int + expansion)
            as COORD;
    } else {
        (*rect).extent.height = (ScreenHeight - (*rect).corner.y as ::core::ffi::c_int)
            as COORD;
    };
}
#[no_mangle]
pub unsafe extern "C" fn SetTransitionSource(mut pRect: *const RECT) {
    let mut ActualRect: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    if !pRect.is_null() {
        ActualRect = *pRect;
        pRect = &raw mut ActualRect;
        ExpandRect(&raw mut ActualRect, 2 as ::core::ffi::c_int);
    }
    TFB_DrawScreen_Copy(pRect, SCREEN::TFB_SCREEN_MAIN, SCREEN::TFB_SCREEN_TRANSITION);
}
#[no_mangle]
pub unsafe extern "C" fn ScreenTransition(
    mut TransType: ::core::ffi::c_int,
    mut pRect: *const RECT,
) {
    let DURATION: TimePeriod = (840 as ::core::ffi::c_int * 31 as ::core::ffi::c_int
        / 60 as ::core::ffi::c_int) as TimePeriod;
    let mut startTime: TimeCount = 0;
    if !pRect.is_null() {
        TransitionClipRect = *pRect;
    } else {
        TransitionClipRect.corner.x = 0 as COORD;
        TransitionClipRect.corner.y = 0 as COORD;
        TransitionClipRect.extent.width = ScreenWidth as COORD;
        TransitionClipRect.extent.height = ScreenHeight as COORD;
    }
    TFB_UploadTransitionScreen();
    ::core::ptr::write_volatile(&raw mut TransitionAmount, 0 as ::core::ffi::c_int);
    FlushGraphics();
    startTime = GetTimeCounter();
    while ::core::ptr::read_volatile::<::core::ffi::c_int>(&raw const TransitionAmount)
        < 255 as ::core::ffi::c_int
    {
        let mut deltaT: TimePeriod = 0;
        let mut newAmount: ::core::ffi::c_int = 0;
        SleepThread(
            (840 as ::core::ffi::c_int / 100 as ::core::ffi::c_int) as TimePeriod,
        );
        deltaT = GetTimeCounter().wrapping_sub(startTime) as TimePeriod;
        newAmount = deltaT.wrapping_mul(255 as TimePeriod).wrapping_div(DURATION)
            as ::core::ffi::c_int;
        if newAmount > 255 as ::core::ffi::c_int {
            newAmount = 255 as ::core::ffi::c_int;
        }
        ::core::ptr::write_volatile(&raw mut TransitionAmount, newAmount);
    }
}
