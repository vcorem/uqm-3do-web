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

// Rust translation of src/libs/graphics/pixmap.c.
extern "C" {
    fn TFB_DrawCanvas_IsPaletted(canvas: TFB_Canvas) -> BOOLEAN;
    fn log_add(_: log_Level, fmt: *const ::core::ffi::c_char, ...);
}
pub type uint8 = ::core::ffi::c_uchar;
pub type uint16 = ::core::ffi::c_ushort;
pub type sint16 = ::core::ffi::c_short;
pub type BYTE = uint8;
pub type UWORD = uint16;
pub type SWORD = sint16;
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
pub type DRAWABLE_DESC = drawable_desc;
pub type DRAWABLE = *mut DRAWABLE_DESC;
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
pub unsafe extern "C" fn GetFrameParentDrawable(mut f: FRAME) -> DRAWABLE {
    if !f.is_null() {
        return (*f).parent;
    }
    return ::core::ptr::null_mut::<DRAWABLE_DESC>();
}
#[no_mangle]
pub unsafe extern "C" fn CaptureDrawable(mut DrawablePtr: DRAWABLE) -> FRAME {
    if !DrawablePtr.is_null() {
        return (*DrawablePtr).Frame.offset(0isize);
    }
    return ::core::ptr::null_mut::<FRAME_DESC>();
}
#[no_mangle]
pub unsafe extern "C" fn ReleaseDrawable(mut FramePtr: FRAME) -> DRAWABLE {
    if !FramePtr.is_null() {
        let mut Drawable: DRAWABLE = ::core::ptr::null_mut::<DRAWABLE_DESC>();
        Drawable = GetFrameParentDrawable(FramePtr);
        return Drawable;
    }
    return ::core::ptr::null_mut::<DRAWABLE_DESC>();
}
#[no_mangle]
pub unsafe extern "C" fn GetFrameCount(mut FramePtr: FRAME) -> COUNT {
    let mut DrawablePtr: *mut DRAWABLE_DESC = ::core::ptr::null_mut::<DRAWABLE_DESC>();
    if FramePtr.is_null() {
        return 0 as COUNT;
    }
    DrawablePtr = GetFrameParentDrawable(FramePtr) as *mut DRAWABLE_DESC;
    return ((*DrawablePtr).MaxIndex as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
        as COUNT;
}
#[no_mangle]
pub unsafe extern "C" fn GetFrameIndex(mut FramePtr: FRAME) -> COUNT {
    if FramePtr.is_null() {
        return 0 as COUNT;
    }
    return (*FramePtr).Index;
}
#[no_mangle]
pub unsafe extern "C" fn SetAbsFrameIndex(
    mut FramePtr: FRAME,
    mut FrameIndex: COUNT,
) -> FRAME {
    if !FramePtr.is_null() {
        let mut DrawablePtr: *mut DRAWABLE_DESC = ::core::ptr::null_mut::<
            DRAWABLE_DESC,
        >();
        DrawablePtr = GetFrameParentDrawable(FramePtr) as *mut DRAWABLE_DESC;
        FrameIndex = (FrameIndex as ::core::ffi::c_int
            % ((*DrawablePtr).MaxIndex as ::core::ffi::c_int + 1 as ::core::ffi::c_int))
            as COUNT;
        FramePtr = (*DrawablePtr).Frame.offset(FrameIndex as isize) as FRAME;
    }
    return FramePtr;
}
#[no_mangle]
pub unsafe extern "C" fn SetRelFrameIndex(
    mut FramePtr: FRAME,
    mut FrameOffs: SIZE,
) -> FRAME {
    if !FramePtr.is_null() {
        let mut num_frames: COUNT = 0;
        let mut DrawablePtr: *mut DRAWABLE_DESC = ::core::ptr::null_mut::<
            DRAWABLE_DESC,
        >();
        DrawablePtr = GetFrameParentDrawable(FramePtr) as *mut DRAWABLE_DESC;
        num_frames = ((*DrawablePtr).MaxIndex as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int) as COUNT;
        if (FrameOffs as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
            loop {
                FrameOffs = (FrameOffs as ::core::ffi::c_int
                    + num_frames as ::core::ffi::c_int) as SIZE;
                if (FrameOffs as ::core::ffi::c_int) >= 0 as ::core::ffi::c_int {
                    break;
                }
            }
        }
        FrameOffs = (((*FramePtr).Index as SWORD as ::core::ffi::c_int
            + FrameOffs as ::core::ffi::c_int) % num_frames as ::core::ffi::c_int)
            as SIZE;
        FramePtr = (*DrawablePtr).Frame.offset(FrameOffs as isize) as FRAME;
    }
    return FramePtr;
}
#[no_mangle]
pub unsafe extern "C" fn SetEquFrameIndex(
    mut DstFramePtr: FRAME,
    mut SrcFramePtr: FRAME,
) -> FRAME {
    let mut Index: COUNT = 0;
    if DstFramePtr.is_null() || SrcFramePtr.is_null() {
        return ::core::ptr::null_mut::<FRAME_DESC>();
    }
    Index = GetFrameIndex(SrcFramePtr);
    let mut DrawablePtr: *mut DRAWABLE_DESC = GetFrameParentDrawable(DstFramePtr);
    if Index as ::core::ffi::c_int > (*DrawablePtr).MaxIndex as ::core::ffi::c_int {
        log_add(
            log_Level::log_Debug,
            b"SetEquFrameIndex: source index (%d) beyond destination range (%d)\0"
                .as_ptr() as *const ::core::ffi::c_char,
            Index as ::core::ffi::c_int,
            (*DrawablePtr).MaxIndex as ::core::ffi::c_int,
        );
    }
    return SetAbsFrameIndex(DstFramePtr, Index);
}
#[no_mangle]
pub unsafe extern "C" fn IncFrameIndex(mut FramePtr: FRAME) -> FRAME {
    let mut DrawablePtr: *mut DRAWABLE_DESC = ::core::ptr::null_mut::<DRAWABLE_DESC>();
    if FramePtr.is_null() {
        return ::core::ptr::null_mut::<FRAME_DESC>();
    }
    DrawablePtr = GetFrameParentDrawable(FramePtr) as *mut DRAWABLE_DESC;
    if ((*FramePtr).Index as ::core::ffi::c_int)
        < (*DrawablePtr).MaxIndex as ::core::ffi::c_int
    {
        FramePtr = FramePtr.offset(1);
        return FramePtr;
    } else {
        return (*DrawablePtr).Frame
    };
}
#[no_mangle]
pub unsafe extern "C" fn DecFrameIndex(mut FramePtr: FRAME) -> FRAME {
    if FramePtr.is_null() {
        return ::core::ptr::null_mut::<FRAME_DESC>();
    }
    if (*FramePtr).Index as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
        FramePtr = FramePtr.offset(-1);
        return FramePtr;
    } else {
        let mut DrawablePtr: *mut DRAWABLE_DESC = ::core::ptr::null_mut::<
            DRAWABLE_DESC,
        >();
        DrawablePtr = GetFrameParentDrawable(FramePtr) as *mut DRAWABLE_DESC;
        return (*DrawablePtr).Frame.offset((*DrawablePtr).MaxIndex as isize);
    };
}
#[no_mangle]
pub unsafe extern "C" fn IsFrameIndexed(mut FramePtr: FRAME) -> BOOLEAN {
    return TFB_DrawCanvas_IsPaletted((*(*FramePtr).image).NormalImg);
}
