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

// Rust translation of src/uqm/border.c.
extern "C" {
    pub type context_desc;
    pub type frame_desc;
    fn SetContext(Context: CONTEXT) -> CONTEXT;
    fn SetContextForeGroundColor(Color: Color) -> Color;
    fn SetContextFGFrame(Frame: FRAME) -> FRAME;
    fn SetContextClipRect(pRect: *mut RECT) -> BOOLEAN;
    fn DrawPoint(pPoint: *mut POINT);
    fn DrawFilledRectangle(pRect: *mut RECT);
    fn BatchGraphics();
    fn UnbatchGraphics();
    fn ClearSISRect(ClearFlags: BYTE);
    fn DrawStarConBox(
        pRect: *mut RECT,
        BorderWidth: SIZE,
        TopLeftColor: Color,
        BottomRightColor: Color,
        FillInterior: BOOLEAN,
        InteriorColor: Color,
    );
    static mut ScreenWidth: ::core::ffi::c_int;
    static mut ScreenHeight: ::core::ffi::c_int;
    static mut Screen: FRAME;
    static mut ScreenContext: CONTEXT;
    static mut SpaceContext: CONTEXT;
    static mut StatusContext: CONTEXT;
}
pub type uint8 = ::core::ffi::c_uchar;
pub type sint16 = ::core::ffi::c_short;
pub type BYTE = uint8;
pub type SWORD = sint16;
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
pub type CONTEXT = *mut CONTEXT_DESC;
pub type FRAME = *mut FRAME_DESC;
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
#[inline]
unsafe extern "C" fn colorFromRgb15(mut r: BYTE, mut g: BYTE, mut b: BYTE) -> Color {
    let mut c: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    c.r = ((r as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
        | r as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE;
    c.g = ((g as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
        | g as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE;
    c.b = ((b as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
        | b as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE;
    c.a = 0xff as BYTE;
    return c;
}
#[no_mangle]
pub unsafe extern "C" fn InitSISContexts() {
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    SetContext(StatusContext);
    SetContext(SpaceContext);
    SetContextFGFrame(Screen);
    r.corner.x = (7 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as COORD;
    r.corner.y = (10 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as COORD;
    r.extent.width = (ScreenWidth - 64 as ::core::ffi::c_int
        - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int - 14 as ::core::ffi::c_int)
        as COORD;
    r.extent.height = (ScreenHeight - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
        - 13 as ::core::ffi::c_int) as COORD;
    SetContextClipRect(&raw mut r);
}
#[no_mangle]
pub unsafe extern "C" fn DrawSISFrame() {
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    SetContext(ScreenContext);
    BatchGraphics();
    SetContextForeGroundColor(colorFromRgb15(0xa as BYTE, 0xa as BYTE, 0xa as BYTE));
    r.corner.x = 0 as COORD;
    r.corner.y = 0 as COORD;
    r.extent.width = (7 as ::core::ffi::c_int + 0 as ::core::ffi::c_int
        + (ScreenWidth - 64 as ::core::ffi::c_int
            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
            - 14 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int) as COORD;
    r.extent.height = (10 as ::core::ffi::c_int + 0 as ::core::ffi::c_int
        - 1 as ::core::ffi::c_int) as COORD;
    DrawFilledRectangle(&raw mut r);
    r.corner.x = 0 as COORD;
    r.corner.y = 0 as COORD;
    r.extent.width = (7 as ::core::ffi::c_int + 0 as ::core::ffi::c_int
        - 1 as ::core::ffi::c_int) as COORD;
    r.extent.height = (10 as ::core::ffi::c_int + 0 as ::core::ffi::c_int
        + (ScreenHeight - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
            - 13 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int) as COORD;
    DrawFilledRectangle(&raw mut r);
    r.corner.x = 0 as COORD;
    r.corner.y = r.extent.height;
    r.extent.width = (7 as ::core::ffi::c_int + 0 as ::core::ffi::c_int
        + (ScreenWidth - 64 as ::core::ffi::c_int
            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
            - 14 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int) as COORD;
    r.extent.height = (ScreenHeight
        - (10 as ::core::ffi::c_int + 0 as ::core::ffi::c_int)
        + (ScreenHeight - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
            - 13 as ::core::ffi::c_int)) as COORD;
    DrawFilledRectangle(&raw mut r);
    r.corner.x = (7 as ::core::ffi::c_int + 0 as ::core::ffi::c_int
        + (ScreenWidth - 64 as ::core::ffi::c_int
            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
            - 14 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int) as COORD;
    r.corner.y = 0 as COORD;
    r.extent.width = (ScreenWidth - r.corner.x as ::core::ffi::c_int) as COORD;
    r.extent.height = ScreenHeight as COORD;
    DrawFilledRectangle(&raw mut r);
    r.corner.x = (7 as ::core::ffi::c_int + 0 as ::core::ffi::c_int
        - 1 as ::core::ffi::c_int) as COORD;
    r.corner.y = (10 as ::core::ffi::c_int + 0 as ::core::ffi::c_int
        - 1 as ::core::ffi::c_int) as COORD;
    r.extent.width = (ScreenWidth - 64 as ::core::ffi::c_int
        - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int - 14 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as COORD;
    r.extent.height = (ScreenHeight - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
        - 13 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as COORD;
    DrawStarConBox(
        &raw mut r,
        1 as SIZE,
        colorFromRgb15(0x10 as BYTE, 0x10 as BYTE, 0x10 as BYTE),
        colorFromRgb15(0x8 as BYTE, 0x8 as BYTE, 0x8 as BYTE),
        BOOLEAN::TRUE,
        colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE),
    );
    r.corner.y = 0 as COORD;
    r.extent.height = (10 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as COORD;
    r.corner.x = (7 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as COORD;
    r.extent.width = (ScreenWidth - 64 as ::core::ffi::c_int
        - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int - 14 as ::core::ffi::c_int
        - 57 as ::core::ffi::c_int - 12 as ::core::ffi::c_int) as COORD;
    DrawStarConBox(
        &raw mut r,
        1 as SIZE,
        colorFromRgb15(0 as BYTE, 0 as BYTE, 0xe as BYTE),
        colorFromRgb15(0 as BYTE, 0x1 as BYTE, 0x1c as BYTE),
        BOOLEAN::TRUE,
        colorFromRgb15(0 as BYTE, 0 as BYTE, 0x14 as BYTE),
    );
    r.extent.width = 57 as COORD;
    r.corner.x = (7 as ::core::ffi::c_int + 0 as ::core::ffi::c_int
        + (ScreenWidth - 64 as ::core::ffi::c_int
            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
            - 14 as ::core::ffi::c_int) - 57 as ::core::ffi::c_int) as COORD;
    DrawStarConBox(
        &raw mut r,
        1 as SIZE,
        colorFromRgb15(0 as BYTE, 0 as BYTE, 0xe as BYTE),
        colorFromRgb15(0 as BYTE, 0x1 as BYTE, 0x1c as BYTE),
        BOOLEAN::TRUE,
        colorFromRgb15(0 as BYTE, 0 as BYTE, 0x14 as BYTE),
    );
    SetContextForeGroundColor(colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE));
    r.corner.x = (0 as ::core::ffi::c_int
        + (ScreenWidth - 64 as ::core::ffi::c_int
            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
        - 1 as ::core::ffi::c_int) as COORD;
    r.corner.y = 0 as COORD;
    r.extent.width = 1 as COORD;
    r.extent.height = ScreenHeight as COORD;
    DrawFilledRectangle(&raw mut r);
    r.corner.x = (0 as ::core::ffi::c_int
        + (ScreenWidth - 64 as ::core::ffi::c_int
            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)) as COORD;
    r.corner.y = (0 as ::core::ffi::c_int + 139 as ::core::ffi::c_int) as COORD;
    DrawPoint(&raw mut r.corner);
    r.corner.x = (ScreenWidth - 1 as ::core::ffi::c_int) as COORD;
    DrawPoint(&raw mut r.corner);
    SetContextForeGroundColor(colorFromRgb15(0x10 as BYTE, 0x10 as BYTE, 0x10 as BYTE));
    r.corner.y = 1 as COORD;
    r.extent.width = 1 as COORD;
    r.extent.height = (0 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as COORD;
    r.corner.x = (7 as ::core::ffi::c_int + 0 as ::core::ffi::c_int
        - 1 as ::core::ffi::c_int) as COORD;
    DrawFilledRectangle(&raw mut r);
    r.corner.x = (7 as ::core::ffi::c_int + 0 as ::core::ffi::c_int
        + (ScreenWidth - 64 as ::core::ffi::c_int
            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
            - 14 as ::core::ffi::c_int) - 57 as ::core::ffi::c_int
        - 1 as ::core::ffi::c_int) as COORD;
    DrawFilledRectangle(&raw mut r);
    r.corner.x = 0 as COORD;
    r.corner.y = (ScreenHeight - 1 as ::core::ffi::c_int) as COORD;
    r.extent.width = (0 as ::core::ffi::c_int
        + (ScreenWidth - 64 as ::core::ffi::c_int
            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
        - 1 as ::core::ffi::c_int) as COORD;
    r.extent.height = 1 as COORD;
    DrawFilledRectangle(&raw mut r);
    r.corner.x = (0 as ::core::ffi::c_int
        + (ScreenWidth - 64 as ::core::ffi::c_int
            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
        - 2 as ::core::ffi::c_int) as COORD;
    r.corner.y = 0 as COORD;
    r.extent.width = 1 as COORD;
    r.extent.height = (ScreenHeight - 1 as ::core::ffi::c_int) as COORD;
    DrawFilledRectangle(&raw mut r);
    r.corner.x = (ScreenWidth - 1 as ::core::ffi::c_int) as COORD;
    r.corner.y = 0 as COORD;
    r.extent.width = 1 as COORD;
    r.extent.height = (0 as ::core::ffi::c_int + 139 as ::core::ffi::c_int) as COORD;
    DrawFilledRectangle(&raw mut r);
    r.corner.x = (0 as ::core::ffi::c_int
        + (ScreenWidth - 64 as ::core::ffi::c_int
            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)) as COORD;
    r.corner.y = (ScreenHeight - 1 as ::core::ffi::c_int) as COORD;
    r.extent.width = (ScreenWidth - r.corner.x as ::core::ffi::c_int) as COORD;
    r.extent.height = 1 as COORD;
    DrawFilledRectangle(&raw mut r);
    r.corner.x = (ScreenWidth - 1 as ::core::ffi::c_int) as COORD;
    r.corner.y = (0 as ::core::ffi::c_int + 140 as ::core::ffi::c_int) as COORD;
    r.extent.width = 1 as COORD;
    r.extent.height = (ScreenHeight - 1 as ::core::ffi::c_int
        - r.corner.y as ::core::ffi::c_int) as COORD;
    DrawFilledRectangle(&raw mut r);
    SetContextForeGroundColor(colorFromRgb15(0x8 as BYTE, 0x8 as BYTE, 0x8 as BYTE));
    r.corner.y = 1 as COORD;
    r.extent.width = 1 as COORD;
    r.extent.height = (0 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as COORD;
    r.corner.x = (7 as ::core::ffi::c_int + 0 as ::core::ffi::c_int
        + (ScreenWidth - 64 as ::core::ffi::c_int
            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
            - 14 as ::core::ffi::c_int - 57 as ::core::ffi::c_int
            - 12 as ::core::ffi::c_int)) as COORD;
    DrawFilledRectangle(&raw mut r);
    r.corner.x = (7 as ::core::ffi::c_int + 0 as ::core::ffi::c_int
        + (ScreenWidth - 64 as ::core::ffi::c_int
            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
            - 14 as ::core::ffi::c_int)) as COORD;
    r.extent.height += 1;
    DrawFilledRectangle(&raw mut r);
    r.corner.y = 0 as COORD;
    r.extent.width = (0 as ::core::ffi::c_int
        + (ScreenWidth - 64 as ::core::ffi::c_int
            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
        - 2 as ::core::ffi::c_int - r.corner.x as ::core::ffi::c_int) as COORD;
    r.extent.height = 1 as COORD;
    DrawFilledRectangle(&raw mut r);
    r.corner.x = 0 as COORD;
    r.extent.width = (7 as ::core::ffi::c_int + 0 as ::core::ffi::c_int
        - r.corner.x as ::core::ffi::c_int) as COORD;
    DrawFilledRectangle(&raw mut r);
    r.corner.x = (7 as ::core::ffi::c_int + 0 as ::core::ffi::c_int
        + (ScreenWidth - 64 as ::core::ffi::c_int
            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
            - 14 as ::core::ffi::c_int - 57 as ::core::ffi::c_int
            - 12 as ::core::ffi::c_int)) as COORD;
    r.extent.width = 12 as COORD;
    DrawFilledRectangle(&raw mut r);
    r.corner.x = 0 as COORD;
    r.corner.y = 1 as COORD;
    r.extent.width = 1 as COORD;
    r.extent.height = (ScreenHeight - 1 as ::core::ffi::c_int
        - r.corner.y as ::core::ffi::c_int) as COORD;
    DrawFilledRectangle(&raw mut r);
    r.corner.x = (0 as ::core::ffi::c_int
        + (ScreenWidth - 64 as ::core::ffi::c_int
            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)) as COORD;
    r.corner.y = 0 as COORD;
    r.extent.width = 1 as COORD;
    r.extent.height = (0 as ::core::ffi::c_int + 139 as ::core::ffi::c_int) as COORD;
    DrawFilledRectangle(&raw mut r);
    r.corner.x = (0 as ::core::ffi::c_int
        + (ScreenWidth - 64 as ::core::ffi::c_int
            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
        + 1 as ::core::ffi::c_int) as COORD;
    r.corner.y = (0 as ::core::ffi::c_int + 139 as ::core::ffi::c_int) as COORD;
    r.extent.width = (64 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as COORD;
    r.extent.height = 1 as COORD;
    DrawFilledRectangle(&raw mut r);
    r.corner.x = (0 as ::core::ffi::c_int
        + (ScreenWidth - 64 as ::core::ffi::c_int
            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)) as COORD;
    r.corner.y = (0 as ::core::ffi::c_int + 140 as ::core::ffi::c_int) as COORD;
    r.extent.width = 1 as COORD;
    r.extent.height = (ScreenHeight - r.corner.y as ::core::ffi::c_int) as COORD;
    DrawFilledRectangle(&raw mut r);
    InitSISContexts();
    ClearSISRect(((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as BYTE);
    UnbatchGraphics();
}
