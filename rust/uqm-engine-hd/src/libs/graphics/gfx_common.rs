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
    ) -> !;
    fn TFB_DrawScreen_Copy(r: *const RECT, src: SCREEN, dest: SCREEN);
    fn TFB_DrawScreen_WaitForSignal();
    fn SleepThread(timePeriod: TimePeriod);
    fn GetTimeCounter() -> TimeCount;
    fn TFB_BatchGraphics();
    fn TFB_UnbatchGraphics();
    fn TFB_UploadTransitionScreen(pRect: *mut RECT);
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
pub static mut fs_height: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub static mut fs_width: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub static mut CanvasWidth: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut CanvasHeight: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut WindowWidth: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut WindowHeight: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut SavedWidth: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut SavedHeight: ::core::ffi::c_int = 0;
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
                87 as ::core::ffi::c_int,
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
#[no_mangle]
pub unsafe extern "C" fn SetTransitionSource(mut pRect: *const RECT) {
    TFB_DrawScreen_Copy(
        ::core::ptr::null::<RECT>(),
        SCREEN::TFB_SCREEN_MAIN,
        SCREEN::TFB_SCREEN_TRANSITION,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ScreenTransition(
    mut TransType: ::core::ffi::c_int,
    mut pRect: *const RECT,
) {
    let DURATION: TimePeriod = (840 as ::core::ffi::c_int * 31 as ::core::ffi::c_int
        / 60 as ::core::ffi::c_int) as TimePeriod;
    let mut startTime: TimeCount = 0;
    if TransType == 0x2 as ::core::ffi::c_int {
        return;
    }
    uqm_lua_checked!(TFB_UploadTransitionScreen(pRect as *mut RECT), ());
    ::core::ptr::write_volatile(&raw mut TransitionAmount, 0 as ::core::ffi::c_int);
    FlushGraphics();
    startTime = GetTimeCounter();
    while ::core::ptr::read_volatile::<::core::ffi::c_int>(&raw const TransitionAmount)
        < 255 as ::core::ffi::c_int
    {
        let mut deltaT: TimePeriod = 0;
        let mut newAmount: ::core::ffi::c_int = 0;
        uqm_lua_checked!(SleepThread(
            (840 as ::core::ffi::c_int / 100 as ::core::ffi::c_int) as TimePeriod,
        ), ());
        deltaT = GetTimeCounter().wrapping_sub(startTime) as TimePeriod;
        newAmount = deltaT.wrapping_mul(255 as TimePeriod).wrapping_div(DURATION)
            as ::core::ffi::c_int;
        if newAmount > 255 as ::core::ffi::c_int {
            newAmount = 255 as ::core::ffi::c_int;
        }
        ::core::ptr::write_volatile(&raw mut TransitionAmount, newAmount);
    }
}
