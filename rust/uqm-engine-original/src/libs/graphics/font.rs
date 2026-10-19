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

// Rust translation of src/libs/graphics/font.c.
extern "C" {
    fn BoxIntersect(
        pr1: *mut RECT,
        pr2: *mut RECT,
        printer: *mut RECT,
    ) -> INTERSECT_CODE;
    fn SetContextForeGroundColor(Color: Color) -> Color;
    static mut _pCurContext: CONTEXT;
    static mut _GraphicsStatusFlags: GRAPHICS_STATUS;
    fn GetContextValidRect(pValidRect: *mut RECT, origin: *mut POINT) -> BOOLEAN;
    fn FixContextFontEffect();
    fn getCharFromString(ptr: *mut *const ::core::ffi::c_char) -> UniChar;
    static mut _CurFontPtr: FONT;
    fn _ReleaseFontData(handle: *mut ::core::ffi::c_void) -> BOOLEAN;
    fn TFB_Prim_FontChar(
        charOrigin: POINT,
        fontChar: *mut TFB_Char,
        backing: *mut TFB_Image,
        _: DrawMode,
        ctxOrigin: POINT,
    );
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
pub type UNICODE = ::core::ffi::c_char;
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
    pub Leading: UWORD,
    pub LeadingWidth: UWORD,
    pub fontPages: *mut FONT_PAGE,
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct TEXT_ALIGN(pub ::core::ffi::c_uint);
impl TEXT_ALIGN {
    pub const ALIGN_LEFT: Self = Self(0);
    pub const ALIGN_CENTER: Self = Self(1);
    pub const ALIGN_RIGHT: Self = Self(2);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct text {
    pub baseline: POINT,
    pub pStr: *const UNICODE,
    pub align: TEXT_ALIGN,
    pub CharCount: COUNT,
}
pub type TEXT = text;
pub type INTERSECT_CODE = BYTE;
pub type GRAPHICS_STATUS = BYTE;
#[no_mangle]
pub unsafe extern "C" fn SetContextFont(mut Font: FONT) -> FONT {
    let mut LastFont: FONT = ::core::ptr::null_mut::<FONT_DESC>();
    LastFont = _CurFontPtr;
    _CurFontPtr = Font;
    if _GraphicsStatusFlags as ::core::ffi::c_int
        & ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GRAPHICS_STATUS
            as ::core::ffi::c_int != 0
    {
        (*_pCurContext).Font = Font;
        (*_pCurContext).BackingFlags = ((*_pCurContext).BackingFlags
            as ::core::ffi::c_int | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int)
            as FBK_FLAGS;
    }
    return LastFont;
}
#[no_mangle]
pub unsafe extern "C" fn DestroyFont(mut FontRef: FONT) -> BOOLEAN {
    if FontRef.is_null() {
        return BOOLEAN::FALSE;
    }
    if !_CurFontPtr.is_null() && _CurFontPtr == FontRef {
        SetContextFont(::core::ptr::null_mut::<::core::ffi::c_void>() as FONT);
    }
    return _ReleaseFontData(FontRef as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn font_DrawText(mut lpText: *mut TEXT) {
    let mut ClipRect: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut origin: POINT = POINT { x: 0, y: 0 };
    let mut text: TEXT = TEXT {
        baseline: POINT { x: 0, y: 0 },
        pStr: ::core::ptr::null::<UNICODE>(),
        align: TEXT_ALIGN::ALIGN_LEFT,
        CharCount: 0,
    };
    FixContextFontEffect();
    if !(_GraphicsStatusFlags as ::core::ffi::c_int
        & (((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GRAPHICS_STATUS
            as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as GRAPHICS_STATUS
                as ::core::ffi::c_int) as GRAPHICS_STATUS as ::core::ffi::c_int
        == (((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GRAPHICS_STATUS
            as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as GRAPHICS_STATUS
                as ::core::ffi::c_int) as GRAPHICS_STATUS as ::core::ffi::c_int)
        || GetContextValidRect(::core::ptr::null_mut::<RECT>(), &raw mut origin).0 == 0
    {
        return;
    }
    text = *lpText;
    if TextRect(&raw mut text, &raw mut ClipRect, ::core::ptr::null_mut::<BYTE>()).0 == 0
    {
        return;
    }
    _text_blt(&raw mut ClipRect, &raw mut text, origin);
}
#[no_mangle]
pub unsafe extern "C" fn font_DrawTracedText(
    mut pText: *mut TEXT,
    mut text: Color,
    mut trace: Color,
) {
    let mut oldfg: Color = SetContextForeGroundColor(trace);
    (*pText).baseline.x -= 1;
    font_DrawText(pText);
    (*pText).baseline.x = ((*pText).baseline.x as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as COORD;
    font_DrawText(pText);
    (*pText).baseline.x -= 1;
    (*pText).baseline.y -= 1;
    font_DrawText(pText);
    (*pText).baseline.y = ((*pText).baseline.y as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as COORD;
    font_DrawText(pText);
    (*pText).baseline.y -= 1;
    SetContextForeGroundColor(text);
    font_DrawText(pText);
    SetContextForeGroundColor(oldfg);
}
#[no_mangle]
pub unsafe extern "C" fn GetContextFontLeading(mut pheight: *mut SIZE) -> BOOLEAN {
    if !_CurFontPtr.is_null() {
        *pheight = (*_CurFontPtr).Leading as SIZE;
        return BOOLEAN::TRUE;
    }
    *pheight = 0 as SIZE;
    return BOOLEAN::FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn GetContextFontLeadingWidth(mut pwidth: *mut SIZE) -> BOOLEAN {
    if !_CurFontPtr.is_null() {
        *pwidth = (*_CurFontPtr).LeadingWidth as SIZE;
        return BOOLEAN::TRUE;
    }
    *pwidth = 0 as SIZE;
    return BOOLEAN::FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn TextRect(
    mut lpText: *mut TEXT,
    mut pRect: *mut RECT,
    mut pdelta: *mut BYTE,
) -> BOOLEAN {
    let mut char_delta_array: [BYTE; 100] = [0; 100];
    let mut FontPtr: FONT = ::core::ptr::null_mut::<FONT_DESC>();
    FontPtr = _CurFontPtr;
    if !FontPtr.is_null()
        && (*lpText).CharCount as ::core::ffi::c_int != 0 as ::core::ffi::c_int
    {
        let mut top_y: COORD = 0;
        let mut bot_y: COORD = 0;
        let mut width: SIZE = 0;
        let mut next_ch: UniChar = 0 as UniChar;
        let mut pStr: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut num_chars: COUNT = 0;
        num_chars = (*lpText).CharCount;
        if pdelta.is_null() {
            pdelta = &raw mut char_delta_array as *mut BYTE;
            if num_chars as ::core::ffi::c_int > 100 as ::core::ffi::c_int {
                num_chars = 100 as COUNT;
                (*lpText).CharCount = 100 as COUNT;
            }
        }
        top_y = 0 as COORD;
        bot_y = 0 as COORD;
        width = 0 as SIZE;
        pStr = (*lpText).pStr as *const ::core::ffi::c_char;
        if num_chars as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
            next_ch = getCharFromString(&raw mut pStr);
            if next_ch == '\0' as UniChar {
                num_chars = 0 as COUNT;
            }
        }
        loop {
            let c2rust_fresh1 = num_chars;
            num_chars = num_chars.wrapping_sub(1);
            if c2rust_fresh1 == 0 {
                break;
            }
            let mut ch: UniChar = 0;
            let mut last_width: SIZE = 0;
            let mut charFrame: *mut TFB_Char = ::core::ptr::null_mut::<TFB_Char>();
            last_width = width;
            ch = next_ch;
            if num_chars as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                next_ch = getCharFromString(&raw mut pStr);
                if next_ch == '\0' as UniChar {
                    (*lpText).CharCount = ((*lpText).CharCount as ::core::ffi::c_int
                        - num_chars as ::core::ffi::c_int) as COUNT;
                    num_chars = 0 as COUNT;
                }
            }
            charFrame = getCharFrame(FontPtr, ch);
            if !charFrame.is_null() && (*charFrame).disp.width as ::core::ffi::c_int != 0
            {
                let mut y: COORD = 0;
                y = -((*charFrame).HotSpot.y as ::core::ffi::c_int) as COORD;
                if (y as ::core::ffi::c_int) < top_y as ::core::ffi::c_int {
                    top_y = y;
                }
                y = (y as ::core::ffi::c_int
                    + (*charFrame).disp.height as ::core::ffi::c_int) as COORD;
                if y as ::core::ffi::c_int > bot_y as ::core::ffi::c_int {
                    bot_y = y;
                }
                width = (width as ::core::ffi::c_int
                    + (*charFrame).disp.width as ::core::ffi::c_int) as SIZE;
            }
            let c2rust_fresh2 = pdelta;
            pdelta = pdelta.offset(1);
            *c2rust_fresh2 = (width as ::core::ffi::c_int
                - last_width as ::core::ffi::c_int) as BYTE;
        }
        if width as ::core::ffi::c_int > 0 as ::core::ffi::c_int
            && {
                bot_y = (bot_y as ::core::ffi::c_int - top_y as ::core::ffi::c_int)
                    as COORD;
                bot_y as ::core::ffi::c_int > 0 as ::core::ffi::c_int
            }
        {
            if *pdelta.offset(-1isize) as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                *pdelta.offset(-1isize) = (*pdelta.offset(-1isize)).wrapping_sub(1);
                width -= 1;
            }
            if (*lpText).align.0 == TEXT_ALIGN::ALIGN_LEFT.0 {
                (*pRect).corner.x = 0 as COORD;
            } else if (*lpText).align.0 == TEXT_ALIGN::ALIGN_CENTER.0 {
                (*pRect).corner.x = -(width as ::core::ffi::c_int
                    >> 1 as ::core::ffi::c_int) as COORD;
            } else {
                (*pRect).corner.x = -(width as ::core::ffi::c_int) as COORD;
            }
            (*pRect).corner.y = top_y;
            (*pRect).extent.width = width as COORD;
            (*pRect).extent.height = bot_y;
            (*pRect).corner.x = ((*pRect).corner.x as ::core::ffi::c_int
                + (*lpText).baseline.x as ::core::ffi::c_int) as COORD;
            (*pRect).corner.y = ((*pRect).corner.y as ::core::ffi::c_int
                + (*lpText).baseline.y as ::core::ffi::c_int) as COORD;
            return BOOLEAN::TRUE;
        }
    }
    (*pRect).corner = (*lpText).baseline;
    (*pRect).extent.width = 0 as COORD;
    (*pRect).extent.height = 0 as COORD;
    return BOOLEAN::FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn _text_blt(
    mut pClipRect: *mut RECT,
    mut TextPtr: *mut TEXT,
    mut ctxOrigin: POINT,
) {
    let mut FontPtr: FONT = ::core::ptr::null_mut::<FONT_DESC>();
    let mut num_chars: COUNT = 0;
    let mut next_ch: UniChar = 0;
    let mut pStr: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    let mut origin: POINT = POINT { x: 0, y: 0 };
    let mut backing: *mut TFB_Image = ::core::ptr::null_mut::<TFB_Image>();
    let mut mode: DrawMode = (*_pCurContext).Mode;
    FontPtr = _CurFontPtr;
    if FontPtr.is_null() {
        return;
    }
    backing = (*_pCurContext).FontBacking;
    if backing.is_null() {
        return;
    }
    origin.x = (*pClipRect).corner.x;
    origin.y = (*TextPtr).baseline.y;
    num_chars = (*TextPtr).CharCount;
    if num_chars as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        return;
    }
    pStr = (*TextPtr).pStr as *const ::core::ffi::c_char;
    next_ch = getCharFromString(&raw mut pStr);
    if next_ch == '\0' as UniChar {
        num_chars = 0 as COUNT;
    }
    loop {
        let c2rust_fresh0 = num_chars;
        num_chars = num_chars.wrapping_sub(1);
        if c2rust_fresh0 == 0 {
            break;
        }
        let mut ch: UniChar = 0;
        let mut fontChar: *mut TFB_Char = ::core::ptr::null_mut::<TFB_Char>();
        ch = next_ch;
        if num_chars as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
            next_ch = getCharFromString(&raw mut pStr);
            if next_ch == '\0' as UniChar {
                num_chars = 0 as COUNT;
            }
        }
        fontChar = getCharFrame(FontPtr, ch);
        if !fontChar.is_null() && (*fontChar).disp.width as ::core::ffi::c_int != 0 {
            let mut r: RECT = RECT {
                corner: POINT { x: 0, y: 0 },
                extent: EXTENT { width: 0, height: 0 },
            };
            r.corner.x = (origin.x as ::core::ffi::c_int
                - (*fontChar).HotSpot.x as ::core::ffi::c_int) as COORD;
            r.corner.y = (origin.y as ::core::ffi::c_int
                - (*fontChar).HotSpot.y as ::core::ffi::c_int) as COORD;
            r.extent.width = (*fontChar).disp.width;
            r.extent.height = (*fontChar).disp.height;
            if BoxIntersect(&raw mut r, pClipRect, &raw mut r) != 0 {
                TFB_Prim_FontChar(origin, fontChar, backing, mode, ctxOrigin);
            }
            origin.x = (origin.x as ::core::ffi::c_int
                + (*fontChar).disp.width as ::core::ffi::c_int) as COORD;
        }
    };
}
#[inline]
unsafe extern "C" fn getCharFrame(
    mut fontPtr: *mut FONT_DESC,
    mut ch: UniChar,
) -> *mut TFB_Char {
    let mut pageStart: UniChar = ch & 0xfffff800 as UniChar;
    let mut charIndex: size_t = 0;
    let mut page: *mut FONT_PAGE = (*fontPtr).fontPages;
    loop {
        if page.is_null() {
            return ::core::ptr::null_mut::<TFB_Char>();
        }
        if (*page).pageStart == pageStart {
            break;
        }
        page = (*page).next as *mut FONT_PAGE;
    }
    charIndex = ch.wrapping_sub((*page).firstChar) as size_t;
    if ch >= (*page).firstChar && charIndex < (*page).numChars
        && !(*(*page).charDesc.offset(charIndex as isize)).data.is_null()
    {
        return (*page).charDesc.offset(charIndex as isize)
    } else {
        return ::core::ptr::null_mut::<TFB_Char>()
    };
}
