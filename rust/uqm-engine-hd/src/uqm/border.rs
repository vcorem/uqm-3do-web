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
    static mut resolutionFactor: ::core::ffi::c_uint;
    static mut optWindowType: DWORD;
    static mut CanvasWidth: ::core::ffi::c_int;
    static mut CanvasHeight: ::core::ffi::c_int;
    fn ClearSISRect(ClearFlags: BYTE);
    static mut Screen: FRAME;
    static mut ScreenContext: CONTEXT;
    static mut SpaceContext: CONTEXT;
    static mut StatusContext: CONTEXT;
    fn DrawBorder(Visible: BYTE);
    fn DrawStarConBox(
        pRect: *mut RECT,
        BorderWidth: SIZE,
        TopLeftColor: Color,
        BottomRightColor: Color,
        FillInterior: BOOLEAN,
        InteriorColor: Color,
        CreateCorners: BOOLEAN,
        CornerColor: Color,
    );
}
pub type uint8 = ::core::ffi::c_uchar;
pub type sint16 = ::core::ffi::c_short;
pub type uint32 = ::core::ffi::c_uint;
pub type BYTE = uint8;
pub type SWORD = sint16;
pub type DWORD = uint32;
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed {
    pub const SIS_MAIN_FRAME: Self = Self(0);
    pub const SIS_STAT_FRAME: Self = Self(1);
    pub const DEVICE_CARGO_FRAME: Self = Self(2);
    pub const SIS_STAT_REPAIR_FRAME: Self = Self(3);
    pub const SIS_STAT_SUMM_FRAME: Self = Self(4);
    pub const SIS_MSG_FRAME: Self = Self(5);
    pub const SIS_TITLE_FRAME: Self = Self(6);
    pub const CAPTAIN_FRAME: Self = Self(7);
    pub const CAP_NAME_FRAME: Self = Self(8);
    pub const STAT_MSG_FRAME: Self = Self(9);
    pub const SIS_RADAR_FRAME: Self = Self(10);
    pub const TEXT_LABEL_FRAME: Self = Self(11);
    pub const SIS_REPAIR_FRAME: Self = Self(12);
    pub const LANDER_3DO_FRAME: Self = Self(13);
    pub const LANDER_DOS_FRAME: Self = Self(14);
    pub const SIS_COMM_FRAME: Self = Self(15);
}
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
#[no_mangle]
pub unsafe extern "C" fn InitSISContexts() {
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    SetContext(StatusContext);
    SetContext(SpaceContext);
    SetContextFGFrame(Screen);
    r.corner.x = (((6 as ::core::ffi::c_int) << resolutionFactor)
        + if if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            (if if optWindowType == 1 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                16 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } << resolutionFactor) + ((1 as ::core::ffi::c_int) << resolutionFactor)
        } else {
            0 as ::core::ffi::c_int
        }) as COORD;
    r.corner.y = (((9 as ::core::ffi::c_int) << resolutionFactor)
        + if if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            (if if optWindowType == 1 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                16 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } << resolutionFactor) + ((1 as ::core::ffi::c_int) << resolutionFactor)
        } else {
            0 as ::core::ffi::c_int
        }) as COORD;
    r.extent.width = (CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
        - (if if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            16 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        } << resolutionFactor) * 2 as ::core::ffi::c_int
        - (((13 as ::core::ffi::c_int) << resolutionFactor)
            + (if if optWindowType == 1 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } << resolutionFactor))) as COORD;
    r.extent.height = (CanvasHeight
        - (if if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            16 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        } << resolutionFactor) * 2 as ::core::ffi::c_int
        - ((13 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
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
    if if resolutionFactor != 2 as ::core::ffi::c_uint {
        BOOLEAN::FALSE.0 as ::core::ffi::c_int
    } else {
        BOOLEAN::TRUE.0 as ::core::ffi::c_int
    } == 0
    {
        SetContextForeGroundColor(colorFromRgb15(0xa as BYTE, 0xa as BYTE, 0xa as BYTE));
        r.corner.x = 0 as COORD;
        r.corner.y = 0 as COORD;
        r.extent.width = (((6 as ::core::ffi::c_int) << resolutionFactor)
            + if if optWindowType == 1 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                (if if optWindowType == 1 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    16 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                } << resolutionFactor) + ((1 as ::core::ffi::c_int) << resolutionFactor)
            } else {
                0 as ::core::ffi::c_int
            }
            + (CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
                - (if if optWindowType == 1 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    16 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                } << resolutionFactor) * 2 as ::core::ffi::c_int
                - (((13 as ::core::ffi::c_int) << resolutionFactor)
                    + (if if optWindowType == 1 as DWORD {
                        BOOLEAN::TRUE.0 as ::core::ffi::c_int
                    } else {
                        BOOLEAN::FALSE.0 as ::core::ffi::c_int
                    } != 0
                    {
                        1 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    } << resolutionFactor))) + 1 as ::core::ffi::c_int) as COORD;
        r.extent.height = (((9 as ::core::ffi::c_int) << resolutionFactor)
            + if if optWindowType == 1 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                (if if optWindowType == 1 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    16 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                } << resolutionFactor) + ((1 as ::core::ffi::c_int) << resolutionFactor)
            } else {
                0 as ::core::ffi::c_int
            } - 1 as ::core::ffi::c_int) as COORD;
        uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
        r.corner.x = 0 as COORD;
        r.corner.y = 0 as COORD;
        r.extent.width = (((6 as ::core::ffi::c_int) << resolutionFactor)
            + if if optWindowType == 1 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                (if if optWindowType == 1 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    16 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                } << resolutionFactor) + ((1 as ::core::ffi::c_int) << resolutionFactor)
            } else {
                0 as ::core::ffi::c_int
            } - 1 as ::core::ffi::c_int) as COORD;
        r.extent.height = (((9 as ::core::ffi::c_int) << resolutionFactor)
            + if if optWindowType == 1 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                (if if optWindowType == 1 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    16 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                } << resolutionFactor) + ((1 as ::core::ffi::c_int) << resolutionFactor)
            } else {
                0 as ::core::ffi::c_int
            }
            + (CanvasHeight
                - (if if optWindowType == 1 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    16 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                } << resolutionFactor) * 2 as ::core::ffi::c_int
                - ((13 as ::core::ffi::c_int) << resolutionFactor))
            + 1 as ::core::ffi::c_int) as COORD;
        uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
        r.corner.x = 0 as COORD;
        r.corner.y = r.extent.height;
        r.extent.width = (((6 as ::core::ffi::c_int) << resolutionFactor)
            + if if optWindowType == 1 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                (if if optWindowType == 1 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    16 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                } << resolutionFactor) + ((1 as ::core::ffi::c_int) << resolutionFactor)
            } else {
                0 as ::core::ffi::c_int
            }
            + (CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
                - (if if optWindowType == 1 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    16 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                } << resolutionFactor) * 2 as ::core::ffi::c_int
                - (((13 as ::core::ffi::c_int) << resolutionFactor)
                    + (if if optWindowType == 1 as DWORD {
                        BOOLEAN::TRUE.0 as ::core::ffi::c_int
                    } else {
                        BOOLEAN::FALSE.0 as ::core::ffi::c_int
                    } != 0
                    {
                        1 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    } << resolutionFactor))) + 1 as ::core::ffi::c_int) as COORD;
        r.extent.height = (CanvasHeight
            - (((9 as ::core::ffi::c_int) << resolutionFactor)
                + if if optWindowType == 1 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    (if if optWindowType == 1 as DWORD {
                        BOOLEAN::TRUE.0 as ::core::ffi::c_int
                    } else {
                        BOOLEAN::FALSE.0 as ::core::ffi::c_int
                    } != 0
                    {
                        16 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    } << resolutionFactor)
                        + ((1 as ::core::ffi::c_int) << resolutionFactor)
                } else {
                    0 as ::core::ffi::c_int
                })
            + (CanvasHeight
                - (if if optWindowType == 1 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    16 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                } << resolutionFactor) * 2 as ::core::ffi::c_int
                - ((13 as ::core::ffi::c_int) << resolutionFactor))) as COORD;
        uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
        r.corner.x = (((6 as ::core::ffi::c_int) << resolutionFactor)
            + if if optWindowType == 1 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                (if if optWindowType == 1 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    16 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                } << resolutionFactor) + ((1 as ::core::ffi::c_int) << resolutionFactor)
            } else {
                0 as ::core::ffi::c_int
            }
            + (CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
                - (if if optWindowType == 1 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    16 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                } << resolutionFactor) * 2 as ::core::ffi::c_int
                - (((13 as ::core::ffi::c_int) << resolutionFactor)
                    + (if if optWindowType == 1 as DWORD {
                        BOOLEAN::TRUE.0 as ::core::ffi::c_int
                    } else {
                        BOOLEAN::FALSE.0 as ::core::ffi::c_int
                    } != 0
                    {
                        1 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    } << resolutionFactor))) + 1 as ::core::ffi::c_int) as COORD;
        r.corner.y = 0 as COORD;
        r.extent.width = (CanvasWidth - r.corner.x as ::core::ffi::c_int) as COORD;
        r.extent.height = CanvasHeight as COORD;
        uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
        r.corner.x = (((6 as ::core::ffi::c_int) << resolutionFactor)
            + if if optWindowType == 1 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                (if if optWindowType == 1 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    16 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                } << resolutionFactor) + ((1 as ::core::ffi::c_int) << resolutionFactor)
            } else {
                0 as ::core::ffi::c_int
            } - 1 as ::core::ffi::c_int) as COORD;
        r.corner.y = (((9 as ::core::ffi::c_int) << resolutionFactor)
            + if if optWindowType == 1 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                (if if optWindowType == 1 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    16 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                } << resolutionFactor) + ((1 as ::core::ffi::c_int) << resolutionFactor)
            } else {
                0 as ::core::ffi::c_int
            } - 1 as ::core::ffi::c_int) as COORD;
        r.extent.width = (CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
            - (if if optWindowType == 1 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                16 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } << resolutionFactor) * 2 as ::core::ffi::c_int
            - (((13 as ::core::ffi::c_int) << resolutionFactor)
                + (if if optWindowType == 1 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    1 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                } << resolutionFactor)) + 2 as ::core::ffi::c_int) as COORD;
        r.extent.height = (CanvasHeight
            - (if if optWindowType == 1 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                16 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } << resolutionFactor) * 2 as ::core::ffi::c_int
            - ((13 as ::core::ffi::c_int) << resolutionFactor) + 2 as ::core::ffi::c_int)
            as COORD;
        uqm_lua_checked!(DrawStarConBox(
            &raw mut r,
            1 as SIZE,
            colorFromRgb15(0x10 as BYTE, 0x10 as BYTE, 0x10 as BYTE),
            colorFromRgb15(0x8 as BYTE, 0x8 as BYTE, 0x8 as BYTE),
            BOOLEAN::TRUE,
            colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE),
            BOOLEAN::FALSE,
            buildColorRgba(0x50 as BYTE, 0x50 as BYTE, 0x50 as BYTE, 0 as BYTE),
        ), ());
        r.corner.y = 0 as COORD;
        r.extent.height = (((9 as ::core::ffi::c_int) << resolutionFactor)
            + if if optWindowType == 1 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                (if if optWindowType == 1 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    16 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                } << resolutionFactor) + ((1 as ::core::ffi::c_int) << resolutionFactor)
            } else {
                0 as ::core::ffi::c_int
            }) as COORD;
        r.corner.x = (((6 as ::core::ffi::c_int) << resolutionFactor)
            + if if optWindowType == 1 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                (if if optWindowType == 1 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    16 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                } << resolutionFactor) + ((1 as ::core::ffi::c_int) << resolutionFactor)
            } else {
                0 as ::core::ffi::c_int
            }) as COORD;
        r.extent.width = (CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
            - (if if optWindowType == 1 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                16 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } << resolutionFactor) * 2 as ::core::ffi::c_int
            - (((13 as ::core::ffi::c_int) << resolutionFactor)
                + (if if optWindowType == 1 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    1 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                } << resolutionFactor))
            - ((57 as ::core::ffi::c_int) << resolutionFactor)
            - ((12 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        uqm_lua_checked!(DrawStarConBox(
            &raw mut r,
            1 as SIZE,
            colorFromRgb15(0 as BYTE, 0 as BYTE, 0xe as BYTE),
            colorFromRgb15(0 as BYTE, 0x1 as BYTE, 0x1c as BYTE),
            BOOLEAN::TRUE,
            colorFromRgb15(0 as BYTE, 0 as BYTE, 0x14 as BYTE),
            BOOLEAN::FALSE,
            buildColorRgba(0x50 as BYTE, 0x50 as BYTE, 0x50 as BYTE, 0 as BYTE),
        ), ());
        r.extent.width = ((57 as ::core::ffi::c_int) << resolutionFactor) as COORD;
        r.corner.x = (((6 as ::core::ffi::c_int) << resolutionFactor)
            + if if optWindowType == 1 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                (if if optWindowType == 1 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    16 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                } << resolutionFactor) + ((1 as ::core::ffi::c_int) << resolutionFactor)
            } else {
                0 as ::core::ffi::c_int
            }
            + (CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
                - (if if optWindowType == 1 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    16 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                } << resolutionFactor) * 2 as ::core::ffi::c_int
                - (((13 as ::core::ffi::c_int) << resolutionFactor)
                    + (if if optWindowType == 1 as DWORD {
                        BOOLEAN::TRUE.0 as ::core::ffi::c_int
                    } else {
                        BOOLEAN::FALSE.0 as ::core::ffi::c_int
                    } != 0
                    {
                        1 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    } << resolutionFactor)))
            - ((57 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        uqm_lua_checked!(DrawStarConBox(
            &raw mut r,
            1 as SIZE,
            colorFromRgb15(0 as BYTE, 0 as BYTE, 0xe as BYTE),
            colorFromRgb15(0 as BYTE, 0x1 as BYTE, 0x1c as BYTE),
            BOOLEAN::TRUE,
            colorFromRgb15(0 as BYTE, 0 as BYTE, 0x14 as BYTE),
            BOOLEAN::FALSE,
            buildColorRgba(0x50 as BYTE, 0x50 as BYTE, 0x50 as BYTE, 0 as BYTE),
        ), ());
        SetContextForeGroundColor(colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE));
        r.corner.x = ((if if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            16 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        } << resolutionFactor)
            + (CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
                - (if if optWindowType == 1 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    16 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                } << resolutionFactor) * 2 as ::core::ffi::c_int)
            - 1 as ::core::ffi::c_int) as COORD;
        r.corner.y = 0 as COORD;
        r.extent.width = 1 as COORD;
        r.extent.height = CanvasHeight as COORD;
        uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
        r.corner.x = ((if if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            16 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        } << resolutionFactor)
            + (CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
                - (if if optWindowType == 1 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    16 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                } << resolutionFactor) * 2 as ::core::ffi::c_int)) as COORD;
        r.corner.y = ((if if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            16 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        } << resolutionFactor) + 139 as ::core::ffi::c_int) as COORD;
        uqm_lua_checked!(DrawPoint(&raw mut r.corner), ());
        r.corner.x = (CanvasWidth - 1 as ::core::ffi::c_int) as COORD;
        uqm_lua_checked!(DrawPoint(&raw mut r.corner), ());
        SetContextForeGroundColor(
            colorFromRgb15(0x10 as BYTE, 0x10 as BYTE, 0x10 as BYTE),
        );
        r.corner.y = 1 as COORD;
        r.extent.width = 1 as COORD;
        r.extent.height = ((if if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            16 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        } << resolutionFactor) + ((8 as ::core::ffi::c_int) << resolutionFactor))
            as COORD;
        r.corner.x = (((6 as ::core::ffi::c_int) << resolutionFactor)
            + if if optWindowType == 1 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                (if if optWindowType == 1 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    16 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                } << resolutionFactor) + ((1 as ::core::ffi::c_int) << resolutionFactor)
            } else {
                0 as ::core::ffi::c_int
            } - 1 as ::core::ffi::c_int) as COORD;
        uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
        r.corner.x = (((6 as ::core::ffi::c_int) << resolutionFactor)
            + if if optWindowType == 1 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                (if if optWindowType == 1 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    16 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                } << resolutionFactor) + ((1 as ::core::ffi::c_int) << resolutionFactor)
            } else {
                0 as ::core::ffi::c_int
            }
            + (CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
                - (if if optWindowType == 1 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    16 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                } << resolutionFactor) * 2 as ::core::ffi::c_int
                - (((13 as ::core::ffi::c_int) << resolutionFactor)
                    + (if if optWindowType == 1 as DWORD {
                        BOOLEAN::TRUE.0 as ::core::ffi::c_int
                    } else {
                        BOOLEAN::FALSE.0 as ::core::ffi::c_int
                    } != 0
                    {
                        1 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    } << resolutionFactor)))
            - ((57 as ::core::ffi::c_int) << resolutionFactor) - 1 as ::core::ffi::c_int)
            as COORD;
        uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
        r.corner.x = 0 as COORD;
        r.corner.y = (CanvasHeight - 1 as ::core::ffi::c_int) as COORD;
        r.extent.width = ((if if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            16 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        } << resolutionFactor)
            + (CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
                - (if if optWindowType == 1 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    16 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                } << resolutionFactor) * 2 as ::core::ffi::c_int)
            - 1 as ::core::ffi::c_int) as COORD;
        r.extent.height = 1 as COORD;
        uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
        r.corner.x = ((if if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            16 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        } << resolutionFactor)
            + (CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
                - (if if optWindowType == 1 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    16 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                } << resolutionFactor) * 2 as ::core::ffi::c_int)
            - 2 as ::core::ffi::c_int) as COORD;
        r.corner.y = 0 as COORD;
        r.extent.width = 1 as COORD;
        r.extent.height = (CanvasHeight - 1 as ::core::ffi::c_int) as COORD;
        uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
        r.corner.x = (CanvasWidth - 1 as ::core::ffi::c_int) as COORD;
        r.corner.y = 0 as COORD;
        r.extent.width = 1 as COORD;
        r.extent.height = ((if if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            16 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        } << resolutionFactor) + 139 as ::core::ffi::c_int) as COORD;
        uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
        r.corner.x = (CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
            - (if if optWindowType == 1 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                16 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } << resolutionFactor) * 2 as ::core::ffi::c_int
            + (if if optWindowType == 1 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                16 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } << resolutionFactor)) as COORD;
        r.corner.y = (CanvasHeight - 1 as ::core::ffi::c_int) as COORD;
        r.extent.width = (((64 as ::core::ffi::c_int) << resolutionFactor)
            + (if if optWindowType == 1 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                16 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } << resolutionFactor)) as COORD;
        r.extent.height = 1 as COORD;
        uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
        r.corner.x = (CanvasWidth - 1 as ::core::ffi::c_int) as COORD;
        r.corner.y = ((if if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            16 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        } << resolutionFactor) + 140 as ::core::ffi::c_int) as COORD;
        r.extent.width = 1 as COORD;
        r.extent.height = (CanvasHeight - 1 as ::core::ffi::c_int
            - r.corner.y as ::core::ffi::c_int) as COORD;
        uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
        SetContextForeGroundColor(colorFromRgb15(0x8 as BYTE, 0x8 as BYTE, 0x8 as BYTE));
        r.corner.y = 1 as COORD;
        r.extent.width = 1 as COORD;
        r.extent.height = ((if if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            16 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        } << resolutionFactor) + ((8 as ::core::ffi::c_int) << resolutionFactor)
            - if if optWindowType == 0 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as COORD;
        r.corner.x = (((6 as ::core::ffi::c_int) << resolutionFactor)
            + if if optWindowType == 1 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                (if if optWindowType == 1 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    16 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                } << resolutionFactor) + ((1 as ::core::ffi::c_int) << resolutionFactor)
            } else {
                0 as ::core::ffi::c_int
            }
            + (CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
                - (if if optWindowType == 1 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    16 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                } << resolutionFactor) * 2 as ::core::ffi::c_int
                - (((13 as ::core::ffi::c_int) << resolutionFactor)
                    + (if if optWindowType == 1 as DWORD {
                        BOOLEAN::TRUE.0 as ::core::ffi::c_int
                    } else {
                        BOOLEAN::FALSE.0 as ::core::ffi::c_int
                    } != 0
                    {
                        1 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    } << resolutionFactor))
                - ((57 as ::core::ffi::c_int) << resolutionFactor)
                - ((12 as ::core::ffi::c_int) << resolutionFactor))) as COORD;
        uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
        r.corner.x = (((6 as ::core::ffi::c_int) << resolutionFactor)
            + if if optWindowType == 1 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                (if if optWindowType == 1 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    16 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                } << resolutionFactor) + ((1 as ::core::ffi::c_int) << resolutionFactor)
            } else {
                0 as ::core::ffi::c_int
            }
            + (CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
                - (if if optWindowType == 1 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    16 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                } << resolutionFactor) * 2 as ::core::ffi::c_int
                - (((13 as ::core::ffi::c_int) << resolutionFactor)
                    + (if if optWindowType == 1 as DWORD {
                        BOOLEAN::TRUE.0 as ::core::ffi::c_int
                    } else {
                        BOOLEAN::FALSE.0 as ::core::ffi::c_int
                    } != 0
                    {
                        1 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    } << resolutionFactor)))) as COORD;
        r.extent.height += 1;
        uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
        r.corner.y = 0 as COORD;
        r.extent.width = ((if if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            16 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        } << resolutionFactor)
            + (CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
                - (if if optWindowType == 1 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    16 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                } << resolutionFactor) * 2 as ::core::ffi::c_int)
            - 2 as ::core::ffi::c_int - r.corner.x as ::core::ffi::c_int) as COORD;
        r.extent.height = 1 as COORD;
        uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
        r.corner.x = 0 as COORD;
        r.extent.width = (((6 as ::core::ffi::c_int) << resolutionFactor)
            + if if optWindowType == 1 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                (if if optWindowType == 1 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    16 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                } << resolutionFactor) + ((1 as ::core::ffi::c_int) << resolutionFactor)
            } else {
                0 as ::core::ffi::c_int
            } - r.corner.x as ::core::ffi::c_int) as COORD;
        uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
        r.corner.x = (((6 as ::core::ffi::c_int) << resolutionFactor)
            + if if optWindowType == 1 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                (if if optWindowType == 1 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    16 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                } << resolutionFactor) + ((1 as ::core::ffi::c_int) << resolutionFactor)
            } else {
                0 as ::core::ffi::c_int
            }
            + (CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
                - (if if optWindowType == 1 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    16 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                } << resolutionFactor) * 2 as ::core::ffi::c_int
                - (((13 as ::core::ffi::c_int) << resolutionFactor)
                    + (if if optWindowType == 1 as DWORD {
                        BOOLEAN::TRUE.0 as ::core::ffi::c_int
                    } else {
                        BOOLEAN::FALSE.0 as ::core::ffi::c_int
                    } != 0
                    {
                        1 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    } << resolutionFactor))
                - ((57 as ::core::ffi::c_int) << resolutionFactor)
                - ((12 as ::core::ffi::c_int) << resolutionFactor))) as COORD;
        r.extent.width = ((12 as ::core::ffi::c_int) << resolutionFactor) as COORD;
        uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
        r.corner.x = 0 as COORD;
        r.corner.y = 1 as COORD;
        r.extent.width = 1 as COORD;
        r.extent.height = (CanvasHeight - 1 as ::core::ffi::c_int
            - r.corner.y as ::core::ffi::c_int) as COORD;
        uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
        r.corner.x = ((if if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            16 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        } << resolutionFactor)
            + (CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
                - (if if optWindowType == 1 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    16 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                } << resolutionFactor) * 2 as ::core::ffi::c_int)) as COORD;
        r.corner.y = 0 as COORD;
        r.extent.width = 1 as COORD;
        r.extent.height = ((if if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            16 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        } << resolutionFactor) + 139 as ::core::ffi::c_int) as COORD;
        uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
        r.corner.x = ((if if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            16 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        } << resolutionFactor)
            + (CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
                - (if if optWindowType == 1 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    16 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                } << resolutionFactor) * 2 as ::core::ffi::c_int)
            + 1 as ::core::ffi::c_int) as COORD;
        r.corner.y = ((if if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            16 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        } << resolutionFactor) + 139 as ::core::ffi::c_int) as COORD;
        r.extent.width = (((64 as ::core::ffi::c_int) << resolutionFactor)
            - 2 as ::core::ffi::c_int) as COORD;
        r.extent.height = 1 as COORD;
        uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
        r.corner.x = ((if if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            16 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        } << resolutionFactor)
            + (CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
                - (if if optWindowType == 1 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    16 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                } << resolutionFactor) * 2 as ::core::ffi::c_int)) as COORD;
        r.corner.y = ((if if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            16 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        } << resolutionFactor) + 140 as ::core::ffi::c_int) as COORD;
        r.extent.width = 1 as COORD;
        r.extent.height = (CanvasHeight - r.corner.y as ::core::ffi::c_int) as COORD;
        uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
    } else {
        SetContextForeGroundColor(colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE));
        r.corner.x = (((6 as ::core::ffi::c_int) << resolutionFactor)
            + if if optWindowType == 1 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                (if if optWindowType == 1 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    16 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                } << resolutionFactor) + ((1 as ::core::ffi::c_int) << resolutionFactor)
            } else {
                0 as ::core::ffi::c_int
            } - ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        r.corner.y = (((9 as ::core::ffi::c_int) << resolutionFactor)
            + if if optWindowType == 1 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                (if if optWindowType == 1 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    16 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                } << resolutionFactor) + ((1 as ::core::ffi::c_int) << resolutionFactor)
            } else {
                0 as ::core::ffi::c_int
            } - ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        r.extent.width = (CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
            - (if if optWindowType == 1 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                16 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } << resolutionFactor) * 2 as ::core::ffi::c_int
            - (((13 as ::core::ffi::c_int) << resolutionFactor)
                + (if if optWindowType == 1 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    1 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                } << resolutionFactor))
            + ((2 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        r.extent.height = (CanvasHeight
            - (if if optWindowType == 1 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                16 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } << resolutionFactor) * 2 as ::core::ffi::c_int
            - ((13 as ::core::ffi::c_int) << resolutionFactor)
            + ((2 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
    }
    uqm_lua_checked!(DrawBorder(C2Rust_Unnamed::SIS_MAIN_FRAME.0 as ::core::ffi::c_int as BYTE), ());
    InitSISContexts();
    uqm_lua_checked!(ClearSISRect(((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as BYTE), ());
    UnbatchGraphics();
}
