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

// Rust translation of src/uqm/oscill.c.
extern "C" {
    pub type context_desc;
    fn SetContext(Context: CONTEXT) -> CONTEXT;
    fn SetContextForeGroundColor(Color: Color) -> Color;
    fn SetContextFGFrame(Frame: FRAME) -> FRAME;
    fn SetContextClipRect(pRect: *mut RECT) -> BOOLEAN;
    fn DrawStamp(pStamp: *mut STAMP);
    fn DrawLine(pLine: *mut LINE);
    fn BatchGraphics();
    fn UnbatchGraphics();
    fn CreateDrawable(
        CreateFlags: CREATE_FLAGS,
        width: SIZE,
        height: SIZE,
        num_frames: COUNT,
    ) -> DRAWABLE;
    fn DestroyDrawable(Drawable: DRAWABLE) -> BOOLEAN;
    fn GetFramePixel(_: FRAME, pixelPt: POINT) -> Color;
    fn CaptureDrawable(Drawable: DRAWABLE) -> FRAME;
    fn ReleaseDrawable(Frame: FRAME) -> DRAWABLE;
    fn __assert_fail(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    );
    static mut OffScreenContext: CONTEXT;
    static mut usingSpeech: BOOLEAN;
    fn GetTrackPosition(in_units: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn GraphForegroundStream(
        data: *mut uint8,
        width: sint32,
        height: sint32,
        wantSpeech: bool,
    ) -> ::core::ffi::c_int;
}
pub type uint8 = ::core::ffi::c_uchar;
pub type uint16 = ::core::ffi::c_ushort;
pub type sint16 = ::core::ffi::c_short;
pub type sint32 = ::core::ffi::c_int;
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
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Color {
    pub r: BYTE,
    pub g: BYTE,
    pub b: BYTE,
    pub a: BYTE,
}
pub type CONTEXT_DESC = context_desc;
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
pub type FRAME_DESC = frame_desc;
pub type CREATE_FLAGS = BYTE;
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
pub type DRAWABLE_TYPE = UWORD;
pub type DRAWABLE_DESC = drawable_desc;
pub type CONTEXT = *mut CONTEXT_DESC;
pub type FRAME = *mut FRAME_DESC;
pub type DRAWABLE = *mut DRAWABLE_DESC;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct line {
    pub first: POINT,
    pub second: POINT,
}
pub type LINE = line;
static mut scope_frame: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
static mut scope_init: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut scopeWork: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
static mut scopeColor: Color = Color { r: 0, g: 0, b: 0, a: 0 };
static mut scopeSize: EXTENT = EXTENT { width: 0, height: 0 };
#[no_mangle]
pub static mut oscillDisabled: BOOLEAN = BOOLEAN::FALSE;
#[no_mangle]
pub unsafe extern "C" fn InitOscilloscope(mut scopeBg: FRAME) {
    scope_frame = scopeBg;
    if scope_init == 0 {
        let mut size: EXTENT = (*scope_frame).Bounds;
        let mut midPt: POINT = point {
            x: (size.width as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as COORD,
            y: (size.height as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as COORD,
        };
        scopeColor = GetFramePixel(scope_frame, midPt);
        scopeColor.a = 0xff as BYTE;
        scopeWork = CaptureDrawable(
            CreateDrawable(
                (((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as CREATE_FLAGS
                    as ::core::ffi::c_int
                    | ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                        as CREATE_FLAGS as ::core::ffi::c_int) as CREATE_FLAGS,
                size.width,
                size.height,
                1 as COUNT,
            ),
        );
        scopeSize.width = (size.width as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
            as COORD;
        scopeSize.height = (size.height as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
            as COORD;
        scope_init = 1 as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn UninitOscilloscope() {
    DestroyDrawable(ReleaseDrawable(scopeWork));
    scopeWork = ::core::ptr::null_mut::<FRAME_DESC>();
    scope_init = 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn DrawOscilloscope() {
    let mut s: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    let mut scope_data: [BYTE; 128] = [0; 128];
    if oscillDisabled.0 != 0 {
        return;
    }
    (scopeSize.width as size_t <= ::core::mem::size_of::<[BYTE; 128]>()
        || {
            __assert_fail(
                b"(size_t)scopeSize.width <= sizeof scope_data\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/uqm/oscill.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                81 as ::core::ffi::c_int,
                b"DrawOscilloscope\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    ((scopeSize.height as ::core::ffi::c_int) < 256 as ::core::ffi::c_int
        || {
            __assert_fail(
                b"scopeSize.height < 256\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/uqm/oscill.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                82 as ::core::ffi::c_int,
                b"DrawOscilloscope\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    if GraphForegroundStream(
        &raw mut scope_data as *mut uint8,
        scopeSize.width as sint32,
        scopeSize.height as sint32,
        usingSpeech.0 != 0,
    ) != 0
    {
        let mut i: ::core::ffi::c_int = 0;
        let mut oldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
        oldContext = SetContext(OffScreenContext);
        SetContextFGFrame(scopeWork);
        SetContextClipRect(::core::ptr::null_mut::<RECT>());
        s.origin.x = 0 as COORD;
        s.origin.y = 0 as COORD;
        s.frame = scope_frame;
        DrawStamp(&raw mut s);
        SetContextForeGroundColor(scopeColor);
        i = 0 as ::core::ffi::c_int;
        while i < scopeSize.width as ::core::ffi::c_int - 1 as ::core::ffi::c_int {
            let mut line: LINE = LINE {
                first: POINT { x: 0, y: 0 },
                second: POINT { x: 0, y: 0 },
            };
            line.first.x = (i + 1 as ::core::ffi::c_int) as COORD;
            line.first.y = (scope_data[i as usize] as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int) as COORD;
            line.second.x = (i + 2 as ::core::ffi::c_int) as COORD;
            line.second.y = (scope_data[(i + 1 as ::core::ffi::c_int) as usize]
                as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as COORD;
            DrawLine(&raw mut line);
            i += 1;
        }
        SetContext(oldContext);
        s.frame = scopeWork;
    } else {
        s.frame = scope_frame;
    }
    s.origin.x = 0 as COORD;
    s.origin.y = 0 as COORD;
    DrawStamp(&raw mut s);
}
static mut sliderStamp: STAMP = STAMP {
    origin: POINT { x: 0, y: 0 },
    frame: ::core::ptr::null_mut::<FRAME_DESC>(),
};
static mut buttonStamp: STAMP = STAMP {
    origin: POINT { x: 0, y: 0 },
    frame: ::core::ptr::null_mut::<FRAME_DESC>(),
};
static mut sliderChanged: BOOLEAN = BOOLEAN::FALSE;
#[no_mangle]
pub static mut sliderSpace: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut sliderDisabled: BOOLEAN = BOOLEAN::FALSE;
#[no_mangle]
pub unsafe extern "C" fn InitSlider(
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
    mut width: ::core::ffi::c_int,
    mut sliderFrame: FRAME,
    mut buttonFrame: FRAME,
) {
    let mut sliderSize: EXTENT = (*sliderFrame).Bounds;
    let mut buttonSize: EXTENT = (*buttonFrame).Bounds;
    sliderStamp.origin.x = x as COORD;
    sliderStamp.origin.y = y as COORD;
    sliderStamp.frame = sliderFrame;
    buttonStamp.origin.x = x as COORD;
    buttonStamp.origin.y = (y
        - (buttonSize.height as ::core::ffi::c_int
            - sliderSize.height as ::core::ffi::c_int) / 2 as ::core::ffi::c_int)
        as COORD;
    buttonStamp.frame = buttonFrame;
    sliderSpace = width - buttonSize.width as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn SetSliderImage(mut f: FRAME) {
    sliderChanged = BOOLEAN::TRUE;
    buttonStamp.frame = f;
}
#[no_mangle]
pub unsafe extern "C" fn DrawSlider() {
    let mut offs: ::core::ffi::c_int = 0;
    static mut last_offs: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    if sliderDisabled.0 != 0 {
        return;
    }
    offs = GetTrackPosition(sliderSpace);
    if offs != last_offs || sliderChanged.0 != 0 {
        sliderChanged = BOOLEAN::FALSE;
        last_offs = offs;
        buttonStamp.origin.x = (sliderStamp.origin.x as ::core::ffi::c_int + offs)
            as COORD;
        BatchGraphics();
        DrawStamp(&raw mut sliderStamp);
        DrawStamp(&raw mut buttonStamp);
        UnbatchGraphics();
    }
}
