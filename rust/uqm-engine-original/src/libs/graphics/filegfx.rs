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

// Rust translation of src/libs/graphics/filegfx.c.
extern "C" {
    pub type uio_DirHandle;
    pub type uio_Stream;
    static mut _cur_resfile_name: *const ::core::ffi::c_char;
    fn res_OpenResFile(
        dir: *mut uio_DirHandle,
        filename: *const ::core::ffi::c_char,
        mode: *const ::core::ffi::c_char,
    ) -> *mut uio_Stream;
    fn LengthResFile(fp: *mut uio_Stream) -> size_t;
    fn res_CloseResFile(fp: *mut uio_Stream) -> BOOLEAN;
    fn _GetCelData(fp: *mut uio_Stream, length: DWORD) -> *mut ::core::ffi::c_void;
    fn _GetFontData(fp: *mut uio_Stream, length: DWORD) -> *mut ::core::ffi::c_void;
    static mut contentDir: *mut uio_DirHandle;
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
pub type DRAWABLE_DESC = drawable_desc;
pub type DRAWABLE = *mut DRAWABLE_DESC;
#[no_mangle]
pub unsafe extern "C" fn LoadGraphicFile(
    mut pStr: *const ::core::ffi::c_char,
) -> DRAWABLE {
    let mut fp: *mut uio_Stream = ::core::ptr::null_mut::<uio_Stream>();
    if !_cur_resfile_name.is_null() {
        return ::core::ptr::null_mut::<DRAWABLE_DESC>();
    }
    fp = res_OpenResFile(
        contentDir,
        pStr,
        b"rb\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !fp.is_null() {
        let mut hData: DRAWABLE = ::core::ptr::null_mut::<DRAWABLE_DESC>();
        _cur_resfile_name = pStr;
        hData = _GetCelData(fp, LengthResFile(fp) as DWORD) as DRAWABLE;
        _cur_resfile_name = ::core::ptr::null::<::core::ffi::c_char>();
        res_CloseResFile(fp);
        return hData;
    }
    return ::core::ptr::null_mut::<DRAWABLE_DESC>();
}
#[no_mangle]
pub unsafe extern "C" fn LoadFontFile(mut pStr: *const ::core::ffi::c_char) -> FONT {
    let mut fp: *mut uio_Stream = ::core::ptr::null_mut::<uio_Stream>();
    if !_cur_resfile_name.is_null() {
        return ::core::ptr::null_mut::<FONT_DESC>();
    }
    fp = res_OpenResFile(
        contentDir,
        pStr,
        b"rb\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !fp.is_null() {
        let mut hData: FONT = ::core::ptr::null_mut::<FONT_DESC>();
        _cur_resfile_name = pStr;
        hData = _GetFontData(fp, LengthResFile(fp) as DWORD) as FONT;
        _cur_resfile_name = ::core::ptr::null::<::core::ffi::c_char>();
        res_CloseResFile(fp);
        return hData;
    }
    return ::core::ptr::null_mut::<FONT_DESC>();
}
