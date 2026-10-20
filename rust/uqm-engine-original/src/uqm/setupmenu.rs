// Copyright Michael Martin, 2004.

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

// Rust translation of src/uqm/setupmenu.c.
extern "C" {
    pub type uio_DirHandle;
    pub type string_table_entry;
    pub type string_table;
    pub type frame_desc;
    pub type font_desc;
    pub type drawable_desc;
    pub type joy_char;
    fn snprintf(
        _: *mut ::core::ffi::c_char,
        _: size_t,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn LoadStringTableInstance(res: RESOURCE) -> STRING_TABLE;
    fn DestroyStringTable(StringTable: STRING_TABLE) -> BOOLEAN;
    fn CaptureStringTable(StringTable: STRING_TABLE) -> STRING;
    fn ReleaseStringTable(String: STRING) -> STRING_TABLE;
    fn GetStringTableCount(String: STRING) -> COUNT;
    fn SetAbsStringTableIndex(String: STRING, StringTableIndex: COUNT) -> STRING;
    fn GetStringAddress(String: STRING) -> STRINGPTR;
    fn HMalloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn HFree(p: *mut ::core::ffi::c_void);
    fn utf8StringCount(start: *const ::core::ffi::c_char) -> size_t;
    fn utf8StringCopy(
        dst: *mut ::core::ffi::c_char,
        size: size_t,
        src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_uchar;
    fn SaveResourceIndex(
        dir: *mut uio_DirHandle,
        rmpfile: *const ::core::ffi::c_char,
        root: *const ::core::ffi::c_char,
        strip_root: BOOLEAN,
    );
    fn res_PutString(key: *const ::core::ffi::c_char, value: *const ::core::ffi::c_char);
    fn res_PutInteger(key: *const ::core::ffi::c_char, value: ::core::ffi::c_int);
    fn res_IsBoolean(key: *const ::core::ffi::c_char) -> BOOLEAN;
    fn res_GetBoolean(key: *const ::core::ffi::c_char) -> BOOLEAN;
    fn res_PutBoolean(key: *const ::core::ffi::c_char, value: BOOLEAN);
    fn font_DrawText(pText: *mut TEXT);
    fn BatchGraphics();
    fn UnbatchGraphics();
    fn FlushGraphics();
    fn DestroyDrawable(Drawable: DRAWABLE) -> BOOLEAN;
    fn LoadGraphicInstance(res: RESOURCE) -> *mut ::core::ffi::c_void;
    fn SetAbsFrameIndex(Frame: FRAME, FrameIndex: COUNT) -> FRAME;
    fn CaptureDrawable(Drawable: DRAWABLE) -> FRAME;
    fn ReleaseDrawable(Frame: FRAME) -> DRAWABLE;
    fn GetTimeCounter() -> TimeCount;
    static mut optWhichCoarseScan: ::core::ffi::c_int;
    static mut optWhichMenu: ::core::ffi::c_int;
    static mut optWhichFonts: ::core::ffi::c_int;
    static mut optWhichIntro: ::core::ffi::c_int;
    static mut optWhichShield: ::core::ffi::c_int;
    static mut optSmoothScroll: ::core::ffi::c_int;
    static mut optMeleeScale: ::core::ffi::c_int;
    static mut opt3doMusic: BOOLEAN;
    static mut optRemixMusic: BOOLEAN;
    static mut optSpeech: BOOLEAN;
    static mut optSubtitles: BOOLEAN;
    static mut optStereoSFX: BOOLEAN;
    static mut optKeepAspectRatio: BOOLEAN;
    static mut optGamma: ::core::ffi::c_float;
    static mut configDir: *mut uio_DirHandle;
    static mut input_templates: [INPUT_TEMPLATE; 6];
    fn setGammaCorrection(gamma: ::core::ffi::c_float) -> bool;
    static mut PulsedInputState: CONTROLLER_INPUT_STATE;
    static mut PlayerControls: [CONTROL_TEMPLATE; 0];
    fn FlushInput();
    fn SetDefaultMenuRepeatDelay();
    fn DoInput(pInputState: *mut ::core::ffi::c_void, resetInput: BOOLEAN);
    static mut GamePaused: BOOLEAN;
    fn DoTextEntry(pTES: *mut TEXTENTRY_STATE) -> BOOLEAN;
    fn SleepThreadUntil(wakeTime: TimeCount);
    fn strncpy(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: size_t,
    ) -> *mut ::core::ffi::c_char;
    static mut GlobData: GLOBDATA;
    static mut StarConFont: FONT;
    fn SetMenuSounds(sound_0: MENU_SOUND_FLAGS, sound_1: MENU_SOUND_FLAGS);
    fn exit(_: ::core::ffi::c_int) -> !;
    static mut GfxFlags: ::core::ffi::c_int;
    fn TFB_SetGamma(gamma: ::core::ffi::c_float) -> bool;
    fn TFB_SupportsHardwareScaling() -> ::core::ffi::c_int;
    fn SetTransitionSource(pRect: *const RECT);
    fn ScreenTransition(transition: ::core::ffi::c_int, pRect: *const RECT);
    static mut ScreenWidthActual: ::core::ffi::c_int;
    static mut ScreenHeightActual: ::core::ffi::c_int;
    static mut GraphicsDriver: ::core::ffi::c_int;
    fn DrawLabelAsWindow(label: *mut WIDGET_LABEL, windowRect: *mut RECT);
    fn Widget_SetWindowColors(bg: Color, dark: Color, medium: Color);
    fn Widget_SetFont(newFont: FONT) -> FONT;
    fn Widget_Event(event: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn Widget_ReceiveFocusMenuScreen(
        _self: *mut WIDGET,
        event: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn Widget_ReceiveFocusChoice(
        _self: *mut WIDGET,
        event: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn Widget_ReceiveFocusSimple(
        _self: *mut WIDGET,
        event: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn Widget_ReceiveFocusControlEntry(
        _self: *mut WIDGET,
        event: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn Widget_ReceiveFocusRefuseFocus(
        _self: *mut WIDGET,
        event: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn Widget_HandleEventMenuScreen(
        _self: *mut WIDGET,
        event: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn Widget_HandleEventChoice(
        _self: *mut WIDGET,
        event: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn Widget_HandleEventSlider(
        _self: *mut WIDGET,
        event: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn Widget_HandleEventTextEntry(
        _self: *mut WIDGET,
        event: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn Widget_HandleEventControlEntry(
        _self: *mut WIDGET,
        event: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn Widget_HandleEventIgnoreAll(
        _self: *mut WIDGET,
        event: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn Widget_HeightChoice(_self: *mut WIDGET) -> ::core::ffi::c_int;
    fn Widget_HeightFullScreen(_self: *mut WIDGET) -> ::core::ffi::c_int;
    fn Widget_HeightOneLine(_self: *mut WIDGET) -> ::core::ffi::c_int;
    fn Widget_HeightLabel(_self: *mut WIDGET) -> ::core::ffi::c_int;
    fn Widget_WidthFullScreen(_self: *mut WIDGET) -> ::core::ffi::c_int;
    fn Widget_DrawMenuScreen(
        _self: *mut WIDGET,
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
    );
    fn Widget_DrawChoice(
        _self: *mut WIDGET,
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
    );
    fn Widget_DrawButton(
        _self: *mut WIDGET,
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
    );
    fn Widget_DrawLabel(
        _self: *mut WIDGET,
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
    );
    fn Widget_DrawSlider(
        _self: *mut WIDGET,
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
    );
    fn Widget_DrawTextEntry(
        _self: *mut WIDGET,
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
    );
    fn Widget_DrawControlEntry(
        _self: *mut WIDGET,
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
    );
    fn Widget_Slider_DrawValue(
        self_0: *mut WIDGET_SLIDER,
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
    );
    fn TFB_DrawScreen_ReinitVideo(
        driver: ::core::ffi::c_int,
        flags: ::core::ffi::c_int,
        width: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
    );
    fn InterrogateInputState(
        templat: ::core::ffi::c_int,
        control: ::core::ffi::c_int,
        index: ::core::ffi::c_int,
        buffer: *mut ::core::ffi::c_char,
        maxlen: ::core::ffi::c_int,
    );
    fn RemoveInputState(
        templat: ::core::ffi::c_int,
        control: ::core::ffi::c_int,
        index: ::core::ffi::c_int,
    );
    fn RebindInputState(
        templat: ::core::ffi::c_int,
        control: ::core::ffi::c_int,
        index: ::core::ffi::c_int,
    );
    fn SaveKeyConfiguration(path: *mut uio_DirHandle, fname: *const ::core::ffi::c_char);
    static mut snddriver: ::core::ffi::c_int;
    static mut soundflags: ::core::ffi::c_int;
    fn InitVideoPlayer(UseCDROM: BOOLEAN) -> BOOLEAN;
    fn UninitVideoPlayer();
    static mut musicVolume: ::core::ffi::c_int;
    static mut musicVolumeScale: ::core::ffi::c_float;
    static mut sfxVolumeScale: ::core::ffi::c_float;
    static mut speechVolumeScale: ::core::ffi::c_float;
    fn SetSpeechVolume(volume: ::core::ffi::c_float);
    fn log_add(_: log_Level, fmt: *const ::core::ffi::c_char, ...);
    fn StringBank_Create() -> *mut stringbank;
    fn StringBank_Free(bank_0: *mut stringbank);
    fn StringBank_AddOrFindString(
        bank_0: *mut stringbank,
        str: *const ::core::ffi::c_char,
    ) -> *const ::core::ffi::c_char;
    fn SplitString(
        s: *const ::core::ffi::c_char,
        splitchar: ::core::ffi::c_char,
        n: ::core::ffi::c_int,
        result: *mut *const ::core::ffi::c_char,
        bank_0: *mut stringbank,
    ) -> ::core::ffi::c_int;
    fn exp(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn fabs(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn log(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn SetMusicVolume(Volume: COUNT);
}
pub type uint8 = ::core::ffi::c_uchar;
pub type uint16 = ::core::ffi::c_ushort;
pub type sint16 = ::core::ffi::c_short;
pub type uint32 = ::core::ffi::c_uint;
pub type sint32 = ::core::ffi::c_int;
pub type BYTE = uint8;
pub type UWORD = uint16;
pub type SWORD = sint16;
pub type DWORD = uint32;
pub type SDWORD = sint32;
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
pub type size_t = usize;
pub type STRING_TABLE_ENTRY_DESC = string_table_entry;
pub type STRING_TABLE_DESC = string_table;
pub type STRING_TABLE = *mut STRING_TABLE_DESC;
pub type STRING = *mut STRING_TABLE_ENTRY_DESC;
pub type STRINGPTR = *mut ::core::ffi::c_char;
pub type RESOURCE = *const ::core::ffi::c_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Color {
    pub r: BYTE,
    pub g: BYTE,
    pub b: BYTE,
    pub a: BYTE,
}
pub type FRAME_DESC = frame_desc;
pub type FONT_DESC = font_desc;
pub type DRAWABLE_DESC = drawable_desc;
pub type FRAME = *mut FRAME_DESC;
pub type FONT = *mut FONT_DESC;
pub type DRAWABLE = *mut DRAWABLE_DESC;
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
pub type GRAPHICS_PRIM = BYTE;
#[derive(Copy, Clone)]
#[repr(C)]
pub union PRIM_DESC {
    pub Point: POINT,
    pub Stamp: STAMP,
    pub Line: LINE,
    pub Text: TEXT,
    pub Rect: RECT,
}
pub type PRIM_LINKS = DWORD;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PRIMITIVE {
    pub Links: PRIM_LINKS,
    pub Type: GRAPHICS_PRIM,
    pub color: Color,
    pub Object: PRIM_DESC,
}
pub type TimeCount = DWORD;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed {
    pub const KEY_UP: Self = Self(0);
    pub const KEY_DOWN: Self = Self(1);
    pub const KEY_LEFT: Self = Self(2);
    pub const KEY_RIGHT: Self = Self(3);
    pub const KEY_WEAPON: Self = Self(4);
    pub const KEY_SPECIAL: Self = Self(5);
    pub const KEY_ESCAPE: Self = Self(6);
    pub const NUM_KEYS: Self = Self(7);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_0 {
    pub const KEY_PAUSE: Self = Self(0);
    pub const KEY_EXIT: Self = Self(1);
    pub const KEY_ABORT: Self = Self(2);
    pub const KEY_DEBUG: Self = Self(3);
    pub const KEY_FULLSCREEN: Self = Self(4);
    pub const KEY_MENU_UP: Self = Self(5);
    pub const KEY_MENU_DOWN: Self = Self(6);
    pub const KEY_MENU_LEFT: Self = Self(7);
    pub const KEY_MENU_RIGHT: Self = Self(8);
    pub const KEY_MENU_SELECT: Self = Self(9);
    pub const KEY_MENU_CANCEL: Self = Self(10);
    pub const KEY_MENU_SPECIAL: Self = Self(11);
    pub const KEY_MENU_PAGE_UP: Self = Self(12);
    pub const KEY_MENU_PAGE_DOWN: Self = Self(13);
    pub const KEY_MENU_HOME: Self = Self(14);
    pub const KEY_MENU_END: Self = Self(15);
    pub const KEY_MENU_ZOOM_IN: Self = Self(16);
    pub const KEY_MENU_ZOOM_OUT: Self = Self(17);
    pub const KEY_MENU_DELETE: Self = Self(18);
    pub const KEY_MENU_BACKSPACE: Self = Self(19);
    pub const KEY_MENU_EDIT_CANCEL: Self = Self(20);
    pub const KEY_MENU_SEARCH: Self = Self(21);
    pub const KEY_MENU_NEXT: Self = Self(22);
    pub const KEY_MENU_ANY: Self = Self(23);
    pub const NUM_MENU_KEYS: Self = Self(24);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct CONTROL_TEMPLATE(pub ::core::ffi::c_uint);
impl CONTROL_TEMPLATE {
    pub const CONTROL_TEMPLATE_KB_1: Self = Self(0);
    pub const CONTROL_TEMPLATE_KB_2: Self = Self(1);
    pub const CONTROL_TEMPLATE_KB_3: Self = Self(2);
    pub const CONTROL_TEMPLATE_JOY_1: Self = Self(3);
    pub const CONTROL_TEMPLATE_JOY_2: Self = Self(4);
    pub const CONTROL_TEMPLATE_JOY_3: Self = Self(5);
    pub const NUM_TEMPLATES: Self = Self(6);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _controller_input_state {
    pub key: [[::core::ffi::c_int; 7]; 6],
    pub menu: [::core::ffi::c_int; 24],
}
pub type CONTROLLER_INPUT_STATE = _controller_input_state;
pub type joy_char_t = joy_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct textentry_state {
    pub InputFunc: Option<unsafe extern "C" fn(*mut textentry_state) -> BOOLEAN>,
    pub Initialized: BOOLEAN,
    pub NextTime: DWORD,
    pub Success: BOOLEAN,
    pub CacheStr: *mut UNICODE,
    pub JoyAlphaString: STRING,
    pub JoystickMode: BOOLEAN,
    pub UpperRegister: BOOLEAN,
    pub JoyAlpha: *mut joy_char_t,
    pub JoyAlphaLength: ::core::ffi::c_int,
    pub JoyUpper: *mut joy_char_t,
    pub JoyLower: *mut joy_char_t,
    pub JoyRegLength: ::core::ffi::c_int,
    pub InsPt: *mut UNICODE,
    pub BaseStr: *mut UNICODE,
    pub CursorPos: ::core::ffi::c_int,
    pub MaxSize: ::core::ffi::c_int,
    pub ChangeCallback: Option<unsafe extern "C" fn(*mut textentry_state) -> BOOLEAN>,
    pub FrameCallback: Option<unsafe extern "C" fn(*mut textentry_state) -> BOOLEAN>,
    pub CbParam: *mut ::core::ffi::c_void,
}
pub type TEXTENTRY_STATE = textentry_state;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct OPT_ENABLABLE(pub ::core::ffi::c_uint);
impl OPT_ENABLABLE {
    pub const OPTVAL_DISABLED: Self = Self(0);
    pub const OPTVAL_ENABLED: Self = Self(1);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct OPT_CONSOLETYPE(pub ::core::ffi::c_uint);
impl OPT_CONSOLETYPE {
    pub const OPTVAL_PC: Self = Self(0);
    pub const OPTVAL_3DO: Self = Self(1);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct OPT_SCALETYPE(pub ::core::ffi::c_uint);
impl OPT_SCALETYPE {
    pub const OPTVAL_NO_SCALE: Self = Self(0);
    pub const OPTVAL_BILINEAR_SCALE: Self = Self(1);
    pub const OPTVAL_BIADAPT_SCALE: Self = Self(2);
    pub const OPTVAL_BIADV_SCALE: Self = Self(3);
    pub const OPTVAL_TRISCAN_SCALE: Self = Self(4);
    pub const OPTVAL_HQXX_SCALE: Self = Self(5);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct OPT_RESTYPE(pub ::core::ffi::c_uint);
impl OPT_RESTYPE {
    pub const OPTVAL_320_240: Self = Self(0);
    pub const OPTVAL_640_480: Self = Self(1);
    pub const OPTVAL_800_600: Self = Self(2);
    pub const OPTVAL_1024_768: Self = Self(3);
    pub const OPTVAL_1280_960: Self = Self(4);
    pub const OPTVAL_CUSTOM: Self = Self(5);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct OPT_DRIVERTYPE(pub ::core::ffi::c_uint);
impl OPT_DRIVERTYPE {
    pub const OPTVAL_PURE_IF_POSSIBLE: Self = Self(0);
    pub const OPTVAL_ALWAYS_GL: Self = Self(1);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct OPT_ADRIVERTYPE(pub ::core::ffi::c_uint);
impl OPT_ADRIVERTYPE {
    pub const OPTVAL_SILENCE: Self = Self(0);
    pub const OPTVAL_MIXSDL: Self = Self(1);
    pub const OPTVAL_OPENAL: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct OPT_AQUALITYTYPE(pub ::core::ffi::c_uint);
impl OPT_AQUALITYTYPE {
    pub const OPTVAL_LOW: Self = Self(0);
    pub const OPTVAL_MEDIUM: Self = Self(1);
    pub const OPTVAL_HIGH: Self = Self(2);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct globalopts_struct {
    pub scaler: OPT_SCALETYPE,
    pub res: OPT_RESTYPE,
    pub driver: OPT_DRIVERTYPE,
    pub adriver: OPT_ADRIVERTYPE,
    pub aquality: OPT_AQUALITYTYPE,
    pub fullscreen: OPT_ENABLABLE,
    pub subtitles: OPT_ENABLABLE,
    pub scanlines: OPT_ENABLABLE,
    pub fps: OPT_ENABLABLE,
    pub stereo: OPT_ENABLABLE,
    pub music3do: OPT_ENABLABLE,
    pub musicremix: OPT_ENABLABLE,
    pub speech: OPT_ENABLABLE,
    pub keepaspect: OPT_ENABLABLE,
    pub menu: OPT_CONSOLETYPE,
    pub text: OPT_CONSOLETYPE,
    pub cscan: OPT_CONSOLETYPE,
    pub scroll: OPT_CONSOLETYPE,
    pub intro: OPT_CONSOLETYPE,
    pub meleezoom: OPT_CONSOLETYPE,
    pub shield: OPT_CONSOLETYPE,
    pub player1: CONTROL_TEMPLATE,
    pub player2: CONTROL_TEMPLATE,
    pub speechvol: ::core::ffi::c_int,
    pub musicvol: ::core::ffi::c_int,
    pub sfxvol: ::core::ffi::c_int,
    pub gamma: ::core::ffi::c_int,
}
pub type GLOBALOPTS = globalopts_struct;
pub type stringbank = _stringbank_chunk;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _stringbank_chunk {
    pub data: [::core::ffi::c_char; 1016],
    pub len: ::core::ffi::c_int,
    pub next: *mut _stringbank_chunk,
}
pub type WIDGET_LABEL = _widget_label;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _widget_label {
    pub tag: WIDGET_TYPE,
    pub parent: *mut _widget,
    pub handleEvent: Option<
        unsafe extern "C" fn(*mut _widget, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub receiveFocus: Option<
        unsafe extern "C" fn(*mut _widget, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub draw: Option<
        unsafe extern "C" fn(*mut _widget, ::core::ffi::c_int, ::core::ffi::c_int) -> (),
    >,
    pub height: Option<unsafe extern "C" fn(*mut _widget) -> ::core::ffi::c_int>,
    pub width: Option<unsafe extern "C" fn(*mut _widget) -> ::core::ffi::c_int>,
    pub line_count: ::core::ffi::c_int,
    pub lines: *mut *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _widget {
    pub tag: WIDGET_TYPE,
    pub parent: *mut _widget,
    pub handleEvent: Option<
        unsafe extern "C" fn(*mut _widget, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub receiveFocus: Option<
        unsafe extern "C" fn(*mut _widget, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub draw: Option<
        unsafe extern "C" fn(*mut _widget, ::core::ffi::c_int, ::core::ffi::c_int) -> (),
    >,
    pub height: Option<unsafe extern "C" fn(*mut _widget) -> ::core::ffi::c_int>,
    pub width: Option<unsafe extern "C" fn(*mut _widget) -> ::core::ffi::c_int>,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct WIDGET_TYPE(pub ::core::ffi::c_uint);
impl WIDGET_TYPE {
    pub const WIDGET_TYPE_MENU_SCREEN: Self = Self(0);
    pub const WIDGET_TYPE_CHOICE: Self = Self(1);
    pub const WIDGET_TYPE_BUTTON: Self = Self(2);
    pub const WIDGET_TYPE_LABEL: Self = Self(3);
    pub const WIDGET_TYPE_SLIDER: Self = Self(4);
    pub const WIDGET_TYPE_TEXTENTRY: Self = Self(5);
    pub const WIDGET_TYPE_CONTROLENTRY: Self = Self(6);
    pub const NUM_WIDGET_TYPES: Self = Self(7);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CHOICE_OPTION {
    pub optname: *const ::core::ffi::c_char,
    pub tooltip: [*const ::core::ffi::c_char; 3],
}
pub type WIDGET_CHOICE = _widget_choice;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _widget_choice {
    pub tag: WIDGET_TYPE,
    pub parent: *mut _widget,
    pub handleEvent: Option<
        unsafe extern "C" fn(*mut _widget, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub receiveFocus: Option<
        unsafe extern "C" fn(*mut _widget, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub draw: Option<
        unsafe extern "C" fn(*mut _widget, ::core::ffi::c_int, ::core::ffi::c_int) -> (),
    >,
    pub height: Option<unsafe extern "C" fn(*mut _widget) -> ::core::ffi::c_int>,
    pub width: Option<unsafe extern "C" fn(*mut _widget) -> ::core::ffi::c_int>,
    pub category: *const ::core::ffi::c_char,
    pub numopts: ::core::ffi::c_int,
    pub maxcolumns: ::core::ffi::c_int,
    pub options: *mut CHOICE_OPTION,
    pub selected: ::core::ffi::c_int,
    pub highlighted: ::core::ffi::c_int,
    pub onChange: Option<
        unsafe extern "C" fn(*mut _widget_choice, ::core::ffi::c_int) -> (),
    >,
}
pub type INPUT_TEMPLATE = _input_template;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _input_template {
    pub name: [::core::ffi::c_char; 30],
}
pub type WIDGET_SLIDER = _widget_slider;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _widget_slider {
    pub tag: WIDGET_TYPE,
    pub parent: *mut _widget,
    pub handleEvent: Option<
        unsafe extern "C" fn(*mut _widget, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub receiveFocus: Option<
        unsafe extern "C" fn(*mut _widget, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub draw: Option<
        unsafe extern "C" fn(*mut _widget, ::core::ffi::c_int, ::core::ffi::c_int) -> (),
    >,
    pub height: Option<unsafe extern "C" fn(*mut _widget) -> ::core::ffi::c_int>,
    pub width: Option<unsafe extern "C" fn(*mut _widget) -> ::core::ffi::c_int>,
    pub draw_value: Option<
        unsafe extern "C" fn(
            *mut _widget_slider,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub min: ::core::ffi::c_int,
    pub max: ::core::ffi::c_int,
    pub step: ::core::ffi::c_int,
    pub value: ::core::ffi::c_int,
    pub category: *const ::core::ffi::c_char,
    pub tooltip: [*const ::core::ffi::c_char; 3],
}
pub type ACTIVITY = UWORD;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GAME_STATE {
    pub glob_flags: BYTE,
    pub CrewCost: BYTE,
    pub FuelCost: BYTE,
    pub ModuleCost: [BYTE; 20],
    pub ElementWorth: [BYTE; 8],
    pub DisplayArray: *mut PRIMITIVE,
    pub CurrentActivity: ACTIVITY,
    pub GameClock: CLOCK_STATE,
    pub autopilot: POINT,
    pub ip_location: POINT,
    pub ShipStamp: STAMP,
    pub ShipFacing: UWORD,
    pub ip_planet: BYTE,
    pub in_orbit: BYTE,
    pub velocity: VELOCITY_DESC,
    pub BattleGroupRef: DWORD,
    pub avail_race_q: QUEUE,
    pub npc_built_ship_q: QUEUE,
    pub ip_group_q: QUEUE,
    pub encounter_q: QUEUE,
    pub built_ship_q: QUEUE,
    pub GameState: [BYTE; 155],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct QUEUE {
    pub head: HLINK,
    pub tail: HLINK,
    pub pq_tab: *mut BYTE,
    pub free_list: HLINK,
    pub object_size: COUNT,
    pub num_objects: BYTE,
}
pub type HLINK = QUEUE_HANDLE;
pub type QUEUE_HANDLE = *mut ::core::ffi::c_void;
pub type VELOCITY_DESC = velocity_desc;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct velocity_desc {
    pub TravelAngle: COUNT,
    pub vector: EXTENT,
    pub fract: EXTENT,
    pub error: EXTENT,
    pub incr: EXTENT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CLOCK_STATE {
    pub day_index: BYTE,
    pub month_index: BYTE,
    pub year_index: COUNT,
    pub tick_count: SIZE,
    pub day_in_ticks: SIZE,
    pub event_q: QUEUE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GLOBDATA {
    pub SIS_state: SIS_STATE,
    pub Game_state: GAME_STATE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SIS_STATE {
    pub log_x: SDWORD,
    pub log_y: SDWORD,
    pub ResUnits: DWORD,
    pub FuelOnBoard: DWORD,
    pub CrewEnlisted: COUNT,
    pub TotalElementMass: COUNT,
    pub TotalBioMass: COUNT,
    pub ModuleSlots: [BYTE; 16],
    pub DriveSlots: [BYTE; 11],
    pub JetSlots: [BYTE; 8],
    pub NumLanders: BYTE,
    pub ElementAmounts: [COUNT; 8],
    pub ShipName: [UNICODE; 16],
    pub CommanderName: [UNICODE; 16],
    pub PlanetName: [UNICODE; 16],
}
pub type SETUP_MENU_STATE = setup_menu_state;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct setup_menu_state {
    pub InputFunc: Option<unsafe extern "C" fn(*mut setup_menu_state) -> BOOLEAN>,
    pub initialized: BOOLEAN,
    pub anim_frame_count: ::core::ffi::c_int,
    pub NextTime: DWORD,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _widget_controlentry {
    pub tag: WIDGET_TYPE,
    pub parent: *mut _widget,
    pub handleEvent: Option<
        unsafe extern "C" fn(*mut _widget, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub receiveFocus: Option<
        unsafe extern "C" fn(*mut _widget, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub draw: Option<
        unsafe extern "C" fn(*mut _widget, ::core::ffi::c_int, ::core::ffi::c_int) -> (),
    >,
    pub height: Option<unsafe extern "C" fn(*mut _widget) -> ::core::ffi::c_int>,
    pub width: Option<unsafe extern "C" fn(*mut _widget) -> ::core::ffi::c_int>,
    pub onChange: Option<unsafe extern "C" fn(*mut _widget_controlentry) -> ()>,
    pub onDelete: Option<unsafe extern "C" fn(*mut _widget_controlentry) -> ()>,
    pub category: *const ::core::ffi::c_char,
    pub controlindex: ::core::ffi::c_int,
    pub highlighted: ::core::ffi::c_int,
    pub controlname: [[::core::ffi::c_char; 16]; 2],
}
pub type WIDGET_CONTROLENTRY = _widget_controlentry;
pub type WIDGET_TEXTENTRY = _widget_textentry;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _widget_textentry {
    pub tag: WIDGET_TYPE,
    pub parent: *mut _widget,
    pub handleEvent: Option<
        unsafe extern "C" fn(*mut _widget, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub receiveFocus: Option<
        unsafe extern "C" fn(*mut _widget, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub draw: Option<
        unsafe extern "C" fn(*mut _widget, ::core::ffi::c_int, ::core::ffi::c_int) -> (),
    >,
    pub height: Option<unsafe extern "C" fn(*mut _widget) -> ::core::ffi::c_int>,
    pub width: Option<unsafe extern "C" fn(*mut _widget) -> ::core::ffi::c_int>,
    pub handleEventSelect: Option<
        unsafe extern "C" fn(*mut _widget_textentry) -> ::core::ffi::c_int,
    >,
    pub onChange: Option<unsafe extern "C" fn(*mut _widget_textentry) -> ()>,
    pub category: *const ::core::ffi::c_char,
    pub value: [::core::ffi::c_char; 50],
    pub maxlen: ::core::ffi::c_int,
    pub state: WIDGET_TEXTENTRY_STATE,
    pub cursor_pos: ::core::ffi::c_int,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct WIDGET_TEXTENTRY_STATE(pub ::core::ffi::c_uint);
impl WIDGET_TEXTENTRY_STATE {
    pub const WTE_NORMAL: Self = Self(0);
    pub const WTE_EDITING: Self = Self(1);
    pub const WTE_BLOCKCUR: Self = Self(2);
}
pub type WIDGET = _widget;
pub type MENU_SOUND_FLAGS = UWORD;
pub type WIDGET_BUTTON = _widget_button;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _widget_button {
    pub tag: WIDGET_TYPE,
    pub parent: *mut _widget,
    pub handleEvent: Option<
        unsafe extern "C" fn(*mut _widget, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub receiveFocus: Option<
        unsafe extern "C" fn(*mut _widget, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub draw: Option<
        unsafe extern "C" fn(*mut _widget, ::core::ffi::c_int, ::core::ffi::c_int) -> (),
    >,
    pub height: Option<unsafe extern "C" fn(*mut _widget) -> ::core::ffi::c_int>,
    pub width: Option<unsafe extern "C" fn(*mut _widget) -> ::core::ffi::c_int>,
    pub name: *const ::core::ffi::c_char,
    pub tooltip: [*const ::core::ffi::c_char; 3],
}
pub type HANDLER = Option<
    unsafe extern "C" fn(*mut WIDGET, ::core::ffi::c_int) -> ::core::ffi::c_int,
>;
pub type WIDGET_MENU_SCREEN = _widget_menu_screen;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _widget_menu_screen {
    pub tag: WIDGET_TYPE,
    pub parent: *mut _widget,
    pub handleEvent: Option<
        unsafe extern "C" fn(*mut _widget, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub receiveFocus: Option<
        unsafe extern "C" fn(*mut _widget, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub draw: Option<
        unsafe extern "C" fn(*mut _widget, ::core::ffi::c_int, ::core::ffi::c_int) -> (),
    >,
    pub height: Option<unsafe extern "C" fn(*mut _widget) -> ::core::ffi::c_int>,
    pub width: Option<unsafe extern "C" fn(*mut _widget) -> ::core::ffi::c_int>,
    pub title: *const ::core::ffi::c_char,
    pub subtitle: *const ::core::ffi::c_char,
    pub bgStamp: STAMP,
    pub num_children: ::core::ffi::c_int,
    pub child: *mut *mut _widget,
    pub highlighted: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_1 {
    pub widgets: *mut *mut WIDGET,
    pub bgIndex: ::core::ffi::c_int,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_2(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_2 {
    pub const SUPER_MELEE: Self = Self(0);
    pub const IN_LAST_BATTLE: Self = Self(1);
    pub const IN_ENCOUNTER: Self = Self(2);
    pub const IN_HYPERSPACE: Self = Self(3);
    pub const IN_INTERPLANETARY: Self = Self(4);
    pub const WON_LAST_BATTLE: Self = Self(5);
    pub const IN_QUASISPACE: Self = Self(6);
    pub const IN_PLANET_ORBIT: Self = Self(7);
    pub const IN_STARBASE: Self = Self(8);
    pub const CHECK_PAUSE: Self = Self(256);
    pub const IN_BATTLE: Self = Self(512);
    pub const START_ENCOUNTER: Self = Self(1024);
    pub const START_INTERPLANETARY: Self = Self(2048);
    pub const CHECK_LOAD: Self = Self(4096);
    pub const CHECK_RESTART: Self = Self(8192);
    pub const CHECK_ABORT: Self = Self(16384);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_3(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_3 {
    pub const TFB_GFXDRIVER_SDL_OPENGL: Self = Self(0);
    pub const TFB_GFXDRIVER_SDL_PURE: Self = Self(1);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_4(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_4 {
    pub const TFB_SCALE_STEP: Self = Self(0);
    pub const TFB_SCALE_NEAREST: Self = Self(1);
    pub const TFB_SCALE_BILINEAR: Self = Self(2);
    pub const TFB_SCALE_TRILINEAR: Self = Self(3);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_5(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_5 {
    pub const WIDGET_EVENT_UP: Self = Self(0);
    pub const WIDGET_EVENT_DOWN: Self = Self(1);
    pub const WIDGET_EVENT_LEFT: Self = Self(2);
    pub const WIDGET_EVENT_RIGHT: Self = Self(3);
    pub const WIDGET_EVENT_SELECT: Self = Self(4);
    pub const WIDGET_EVENT_CANCEL: Self = Self(5);
    pub const WIDGET_EVENT_DELETE: Self = Self(6);
    pub const NUM_WIDGET_EVENTS: Self = Self(7);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_6(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_6 {
    pub const audio_DRIVER_MIXSDL: Self = Self(0);
    pub const audio_DRIVER_NOSOUND: Self = Self(1);
    pub const audio_DRIVER_OPENAL: Self = Self(2);
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
static mut SetupTab: STRING = ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>();
static mut done: BOOLEAN = BOOLEAN::FALSE;
static mut next: *mut WIDGET = ::core::ptr::null_mut::<WIDGET>();
static mut current: *mut WIDGET = ::core::ptr::null_mut::<WIDGET>();
static mut menus: [WIDGET_MENU_SCREEN; 8] = [WIDGET_MENU_SCREEN {
    tag: WIDGET_TYPE::WIDGET_TYPE_MENU_SCREEN,
    parent: ::core::ptr::null_mut::<_widget>(),
    handleEvent: None,
    receiveFocus: None,
    draw: None,
    height: None,
    width: None,
    title: ::core::ptr::null::<::core::ffi::c_char>(),
    subtitle: ::core::ptr::null::<::core::ffi::c_char>(),
    bgStamp: STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    },
    num_children: 0,
    child: ::core::ptr::null_mut::<*mut _widget>(),
    highlighted: 0,
}; 8];
static mut choices: [WIDGET_CHOICE; 24] = [WIDGET_CHOICE {
    tag: WIDGET_TYPE::WIDGET_TYPE_MENU_SCREEN,
    parent: ::core::ptr::null_mut::<_widget>(),
    handleEvent: None,
    receiveFocus: None,
    draw: None,
    height: None,
    width: None,
    category: ::core::ptr::null::<::core::ffi::c_char>(),
    numopts: 0,
    maxcolumns: 0,
    options: ::core::ptr::null_mut::<CHOICE_OPTION>(),
    selected: 0,
    highlighted: 0,
    onChange: None,
}; 24];
static mut sliders: [WIDGET_SLIDER; 4] = [WIDGET_SLIDER {
    tag: WIDGET_TYPE::WIDGET_TYPE_MENU_SCREEN,
    parent: ::core::ptr::null_mut::<_widget>(),
    handleEvent: None,
    receiveFocus: None,
    draw: None,
    height: None,
    width: None,
    draw_value: None,
    min: 0,
    max: 0,
    step: 0,
    value: 0,
    category: ::core::ptr::null::<::core::ffi::c_char>(),
    tooltip: [::core::ptr::null::<::core::ffi::c_char>(); 3],
}; 4];
static mut buttons: [WIDGET_BUTTON; 10] = [WIDGET_BUTTON {
    tag: WIDGET_TYPE::WIDGET_TYPE_MENU_SCREEN,
    parent: ::core::ptr::null_mut::<_widget>(),
    handleEvent: None,
    receiveFocus: None,
    draw: None,
    height: None,
    width: None,
    name: ::core::ptr::null::<::core::ffi::c_char>(),
    tooltip: [::core::ptr::null::<::core::ffi::c_char>(); 3],
}; 10];
static mut labels: [WIDGET_LABEL; 4] = [WIDGET_LABEL {
    tag: WIDGET_TYPE::WIDGET_TYPE_MENU_SCREEN,
    parent: ::core::ptr::null_mut::<_widget>(),
    handleEvent: None,
    receiveFocus: None,
    draw: None,
    height: None,
    width: None,
    line_count: 0,
    lines: ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
}; 4];
static mut textentries: [WIDGET_TEXTENTRY; 1] = [WIDGET_TEXTENTRY {
    tag: WIDGET_TYPE::WIDGET_TYPE_MENU_SCREEN,
    parent: ::core::ptr::null_mut::<_widget>(),
    handleEvent: None,
    receiveFocus: None,
    draw: None,
    height: None,
    width: None,
    handleEventSelect: None,
    onChange: None,
    category: ::core::ptr::null::<::core::ffi::c_char>(),
    value: [0; 50],
    maxlen: 0,
    state: WIDGET_TEXTENTRY_STATE::WTE_NORMAL,
    cursor_pos: 0,
}; 1];
static mut controlentries: [WIDGET_CONTROLENTRY; 7] = [WIDGET_CONTROLENTRY {
    tag: WIDGET_TYPE::WIDGET_TYPE_MENU_SCREEN,
    parent: ::core::ptr::null_mut::<_widget>(),
    handleEvent: None,
    receiveFocus: None,
    draw: None,
    height: None,
    width: None,
    onChange: None,
    onDelete: None,
    category: ::core::ptr::null::<::core::ffi::c_char>(),
    controlindex: 0,
    highlighted: 0,
    controlname: [[0; 16]; 2],
}; 7];
static mut choice_widths: [::core::ffi::c_int; 24] = [
    3 as ::core::ffi::c_int,
    2 as ::core::ffi::c_int,
    3 as ::core::ffi::c_int,
    3 as ::core::ffi::c_int,
    2 as ::core::ffi::c_int,
    2 as ::core::ffi::c_int,
    2 as ::core::ffi::c_int,
    2 as ::core::ffi::c_int,
    2 as ::core::ffi::c_int,
    2 as ::core::ffi::c_int,
    2 as ::core::ffi::c_int,
    2 as ::core::ffi::c_int,
    3 as ::core::ffi::c_int,
    2 as ::core::ffi::c_int,
    2 as ::core::ffi::c_int,
    3 as ::core::ffi::c_int,
    3 as ::core::ffi::c_int,
    2 as ::core::ffi::c_int,
    3 as ::core::ffi::c_int,
    3 as ::core::ffi::c_int,
    3 as ::core::ffi::c_int,
    2 as ::core::ffi::c_int,
    2 as ::core::ffi::c_int,
    2 as ::core::ffi::c_int,
];
static mut button_handlers: [HANDLER; 10] = [
    Some(
        quit_main_menu
            as unsafe extern "C" fn(
                *mut WIDGET,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
    ),
    Some(
        quit_sub_menu
            as unsafe extern "C" fn(
                *mut WIDGET,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
    ),
    Some(
        do_graphics
            as unsafe extern "C" fn(
                *mut WIDGET,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
    ),
    Some(
        do_engine
            as unsafe extern "C" fn(
                *mut WIDGET,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
    ),
    Some(
        do_audio
            as unsafe extern "C" fn(
                *mut WIDGET,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
    ),
    Some(
        do_resources
            as unsafe extern "C" fn(
                *mut WIDGET,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
    ),
    Some(
        do_keyconfig
            as unsafe extern "C" fn(
                *mut WIDGET,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
    ),
    Some(
        do_advanced
            as unsafe extern "C" fn(
                *mut WIDGET,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
    ),
    Some(
        do_editkeys
            as unsafe extern "C" fn(
                *mut WIDGET,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
    ),
    Some(
        do_keyconfig
            as unsafe extern "C" fn(
                *mut WIDGET,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
    ),
];
static mut main_widgets: [*mut WIDGET; 8] = [::core::ptr::null_mut::<WIDGET>(); 8];
static mut graphics_widgets: [*mut WIDGET; 8] = [::core::ptr::null_mut::<WIDGET>(); 8];
static mut audio_widgets: [*mut WIDGET; 9] = [::core::ptr::null_mut::<WIDGET>(); 9];
static mut engine_widgets: [*mut WIDGET; 10] = [::core::ptr::null_mut::<WIDGET>(); 10];
static mut advanced_widgets: [*mut WIDGET; 5] = [::core::ptr::null_mut::<WIDGET>(); 5];
static mut keyconfig_widgets: [*mut WIDGET; 6] = [::core::ptr::null_mut::<WIDGET>(); 6];
static mut editkeys_widgets: [*mut WIDGET; 12] = [::core::ptr::null_mut::<WIDGET>(); 12];
static mut incomplete_widgets: [*mut WIDGET; 3] = [::core::ptr::null_mut::<WIDGET>(); 3];
static mut menu_defs: [C2Rust_Unnamed_1; 9] = unsafe {
    [
        C2Rust_Unnamed_1 {
            widgets: &raw const main_widgets as *mut *mut WIDGET,
            bgIndex: 0 as ::core::ffi::c_int,
        },
        C2Rust_Unnamed_1 {
            widgets: &raw const graphics_widgets as *mut *mut WIDGET,
            bgIndex: 1 as ::core::ffi::c_int,
        },
        C2Rust_Unnamed_1 {
            widgets: &raw const audio_widgets as *mut *mut WIDGET,
            bgIndex: 1 as ::core::ffi::c_int,
        },
        C2Rust_Unnamed_1 {
            widgets: &raw const engine_widgets as *mut *mut WIDGET,
            bgIndex: 2 as ::core::ffi::c_int,
        },
        C2Rust_Unnamed_1 {
            widgets: &raw const incomplete_widgets as *mut *mut WIDGET,
            bgIndex: 3 as ::core::ffi::c_int,
        },
        C2Rust_Unnamed_1 {
            widgets: &raw const keyconfig_widgets as *mut *mut WIDGET,
            bgIndex: 1 as ::core::ffi::c_int,
        },
        C2Rust_Unnamed_1 {
            widgets: &raw const advanced_widgets as *mut *mut WIDGET,
            bgIndex: 2 as ::core::ffi::c_int,
        },
        C2Rust_Unnamed_1 {
            widgets: &raw const editkeys_widgets as *mut *mut WIDGET,
            bgIndex: 1 as ::core::ffi::c_int,
        },
        C2Rust_Unnamed_1 {
            widgets: ::core::ptr::null_mut::<*mut WIDGET>(),
            bgIndex: 0 as ::core::ffi::c_int,
        },
    ]
};
static mut minGamma: ::core::ffi::c_float = 0.4f32;
static mut maxGamma: ::core::ffi::c_float = 2.5f32;
static mut minGammaX: ::core::ffi::c_float = 0.;
static mut maxGammaX: ::core::ffi::c_float = 0.;
unsafe extern "C" fn number_res_options() -> ::core::ffi::c_int {
    if TFB_SupportsHardwareScaling() != 0 {
        return 5 as ::core::ffi::c_int
    } else {
        return 2 as ::core::ffi::c_int
    };
}
unsafe extern "C" fn quit_main_menu(
    mut self_0: *mut WIDGET,
    mut event: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if event == C2Rust_Unnamed_5::WIDGET_EVENT_SELECT.0 as ::core::ffi::c_int {
        next = ::core::ptr::null_mut::<WIDGET>();
        return BOOLEAN::TRUE.0 as ::core::ffi::c_int;
    }
    return BOOLEAN::FALSE.0 as ::core::ffi::c_int;
}
unsafe extern "C" fn quit_sub_menu(
    mut self_0: *mut WIDGET,
    mut event: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if event == C2Rust_Unnamed_5::WIDGET_EVENT_SELECT.0 as ::core::ffi::c_int {
        next = (&raw mut menus as *mut WIDGET_MENU_SCREEN).offset(0isize) as *mut WIDGET;
        Some((*next).receiveFocus.expect("non-null function pointer"))
            .expect(
                "non-null function pointer",
            )(next, C2Rust_Unnamed_5::WIDGET_EVENT_SELECT.0 as ::core::ffi::c_int);
        return BOOLEAN::TRUE.0 as ::core::ffi::c_int;
    }
    return BOOLEAN::FALSE.0 as ::core::ffi::c_int;
}
unsafe extern "C" fn do_graphics(
    mut self_0: *mut WIDGET,
    mut event: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if event == C2Rust_Unnamed_5::WIDGET_EVENT_SELECT.0 as ::core::ffi::c_int {
        next = (&raw mut menus as *mut WIDGET_MENU_SCREEN).offset(1isize) as *mut WIDGET;
        Some((*next).receiveFocus.expect("non-null function pointer"))
            .expect(
                "non-null function pointer",
            )(next, C2Rust_Unnamed_5::WIDGET_EVENT_DOWN.0 as ::core::ffi::c_int);
        return BOOLEAN::TRUE.0 as ::core::ffi::c_int;
    }
    return BOOLEAN::FALSE.0 as ::core::ffi::c_int;
}
unsafe extern "C" fn do_audio(
    mut self_0: *mut WIDGET,
    mut event: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if event == C2Rust_Unnamed_5::WIDGET_EVENT_SELECT.0 as ::core::ffi::c_int {
        next = (&raw mut menus as *mut WIDGET_MENU_SCREEN).offset(2isize) as *mut WIDGET;
        Some((*next).receiveFocus.expect("non-null function pointer"))
            .expect(
                "non-null function pointer",
            )(next, C2Rust_Unnamed_5::WIDGET_EVENT_DOWN.0 as ::core::ffi::c_int);
        return BOOLEAN::TRUE.0 as ::core::ffi::c_int;
    }
    return BOOLEAN::FALSE.0 as ::core::ffi::c_int;
}
unsafe extern "C" fn do_engine(
    mut self_0: *mut WIDGET,
    mut event: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if event == C2Rust_Unnamed_5::WIDGET_EVENT_SELECT.0 as ::core::ffi::c_int {
        next = (&raw mut menus as *mut WIDGET_MENU_SCREEN).offset(3isize) as *mut WIDGET;
        Some((*next).receiveFocus.expect("non-null function pointer"))
            .expect(
                "non-null function pointer",
            )(next, C2Rust_Unnamed_5::WIDGET_EVENT_DOWN.0 as ::core::ffi::c_int);
        return BOOLEAN::TRUE.0 as ::core::ffi::c_int;
    }
    return BOOLEAN::FALSE.0 as ::core::ffi::c_int;
}
unsafe extern "C" fn do_resources(
    mut self_0: *mut WIDGET,
    mut event: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if event == C2Rust_Unnamed_5::WIDGET_EVENT_SELECT.0 as ::core::ffi::c_int {
        next = (&raw mut menus as *mut WIDGET_MENU_SCREEN).offset(4isize) as *mut WIDGET;
        Some((*next).receiveFocus.expect("non-null function pointer"))
            .expect(
                "non-null function pointer",
            )(next, C2Rust_Unnamed_5::WIDGET_EVENT_DOWN.0 as ::core::ffi::c_int);
        return BOOLEAN::TRUE.0 as ::core::ffi::c_int;
    }
    return BOOLEAN::FALSE.0 as ::core::ffi::c_int;
}
unsafe extern "C" fn do_keyconfig(
    mut self_0: *mut WIDGET,
    mut event: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if event == C2Rust_Unnamed_5::WIDGET_EVENT_SELECT.0 as ::core::ffi::c_int {
        next = (&raw mut menus as *mut WIDGET_MENU_SCREEN).offset(5isize) as *mut WIDGET;
        Some((*next).receiveFocus.expect("non-null function pointer"))
            .expect(
                "non-null function pointer",
            )(next, C2Rust_Unnamed_5::WIDGET_EVENT_DOWN.0 as ::core::ffi::c_int);
        return BOOLEAN::TRUE.0 as ::core::ffi::c_int;
    }
    return BOOLEAN::FALSE.0 as ::core::ffi::c_int;
}
unsafe extern "C" fn do_advanced(
    mut self_0: *mut WIDGET,
    mut event: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if event == C2Rust_Unnamed_5::WIDGET_EVENT_SELECT.0 as ::core::ffi::c_int {
        next = (&raw mut menus as *mut WIDGET_MENU_SCREEN).offset(6isize) as *mut WIDGET;
        Some((*next).receiveFocus.expect("non-null function pointer"))
            .expect(
                "non-null function pointer",
            )(next, C2Rust_Unnamed_5::WIDGET_EVENT_DOWN.0 as ::core::ffi::c_int);
        return BOOLEAN::TRUE.0 as ::core::ffi::c_int;
    }
    return BOOLEAN::FALSE.0 as ::core::ffi::c_int;
}
unsafe extern "C" fn populate_editkeys(mut templat: ::core::ffi::c_int) {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    strncpy(
        &raw mut (*(&raw mut textentries as *mut WIDGET_TEXTENTRY).offset(0isize)).value
            as *mut ::core::ffi::c_char,
        &raw mut (*(&raw mut input_templates as *mut INPUT_TEMPLATE)
            .offset(templat as isize))
            .name as *mut ::core::ffi::c_char,
        textentries[0usize].maxlen as size_t,
    );
    textentries[0usize]
        .value[(textentries[0usize].maxlen - 1 as ::core::ffi::c_int) as usize] = 0
        as ::core::ffi::c_char;
    i = 0 as ::core::ffi::c_int;
    while i < C2Rust_Unnamed::NUM_KEYS.0 as ::core::ffi::c_int {
        j = 0 as ::core::ffi::c_int;
        while j < 2 as ::core::ffi::c_int {
            InterrogateInputState(
                templat,
                i,
                j,
                &raw mut *(&raw mut (*(&raw mut controlentries
                    as *mut WIDGET_CONTROLENTRY)
                    .offset(i as isize))
                    .controlname as *mut [::core::ffi::c_char; 16])
                    .offset(j as isize) as *mut ::core::ffi::c_char,
                16 as ::core::ffi::c_int,
            );
            j += 1;
        }
        i += 1;
    }
}
unsafe extern "C" fn do_editkeys(
    mut self_0: *mut WIDGET,
    mut event: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if event == C2Rust_Unnamed_5::WIDGET_EVENT_SELECT.0 as ::core::ffi::c_int {
        next = (&raw mut menus as *mut WIDGET_MENU_SCREEN).offset(7isize) as *mut WIDGET;
        choices[20usize].selected = 0 as ::core::ffi::c_int;
        populate_editkeys(0 as ::core::ffi::c_int);
        Some((*next).receiveFocus.expect("non-null function pointer"))
            .expect(
                "non-null function pointer",
            )(next, C2Rust_Unnamed_5::WIDGET_EVENT_DOWN.0 as ::core::ffi::c_int);
        return BOOLEAN::TRUE.0 as ::core::ffi::c_int;
    }
    return BOOLEAN::FALSE.0 as ::core::ffi::c_int;
}
unsafe extern "C" fn change_template(
    mut self_0: *mut WIDGET_CHOICE,
    mut oldval: ::core::ffi::c_int,
) {
    populate_editkeys((*self_0).selected);
}
unsafe extern "C" fn rename_template(mut self_0: *mut WIDGET_TEXTENTRY) {
    strncpy(
        &raw mut (*(&raw mut input_templates as *mut INPUT_TEMPLATE)
            .offset(choices[20usize].selected as isize))
            .name as *mut ::core::ffi::c_char,
        &raw mut (*self_0).value as *mut ::core::ffi::c_char,
        30 as size_t,
    );
    input_templates[choices[20usize].selected as usize].name[29usize] = 0
        as ::core::ffi::c_char;
}
unsafe extern "C" fn SetDefaults() {
    let mut opts: GLOBALOPTS = GLOBALOPTS {
        scaler: OPT_SCALETYPE::OPTVAL_NO_SCALE,
        res: OPT_RESTYPE::OPTVAL_320_240,
        driver: OPT_DRIVERTYPE::OPTVAL_PURE_IF_POSSIBLE,
        adriver: OPT_ADRIVERTYPE::OPTVAL_SILENCE,
        aquality: OPT_AQUALITYTYPE::OPTVAL_LOW,
        fullscreen: OPT_ENABLABLE::OPTVAL_DISABLED,
        subtitles: OPT_ENABLABLE::OPTVAL_DISABLED,
        scanlines: OPT_ENABLABLE::OPTVAL_DISABLED,
        fps: OPT_ENABLABLE::OPTVAL_DISABLED,
        stereo: OPT_ENABLABLE::OPTVAL_DISABLED,
        music3do: OPT_ENABLABLE::OPTVAL_DISABLED,
        musicremix: OPT_ENABLABLE::OPTVAL_DISABLED,
        speech: OPT_ENABLABLE::OPTVAL_DISABLED,
        keepaspect: OPT_ENABLABLE::OPTVAL_DISABLED,
        menu: OPT_CONSOLETYPE::OPTVAL_PC,
        text: OPT_CONSOLETYPE::OPTVAL_PC,
        cscan: OPT_CONSOLETYPE::OPTVAL_PC,
        scroll: OPT_CONSOLETYPE::OPTVAL_PC,
        intro: OPT_CONSOLETYPE::OPTVAL_PC,
        meleezoom: OPT_CONSOLETYPE::OPTVAL_PC,
        shield: OPT_CONSOLETYPE::OPTVAL_PC,
        player1: CONTROL_TEMPLATE::CONTROL_TEMPLATE_KB_1,
        player2: CONTROL_TEMPLATE::CONTROL_TEMPLATE_KB_1,
        speechvol: 0,
        musicvol: 0,
        sfxvol: 0,
        gamma: 0,
    };
    GetGlobalOptions(&raw mut opts);
    if opts.res.0 == OPT_RESTYPE::OPTVAL_CUSTOM.0 {
        choices[0usize].numopts = number_res_options() + 1 as ::core::ffi::c_int;
    } else {
        choices[0usize].numopts = number_res_options();
    }
    choices[0usize].selected = opts.res.0 as ::core::ffi::c_int;
    choices[1usize].selected = opts.driver.0 as ::core::ffi::c_int;
    choices[2usize].selected = opts.scaler.0 as ::core::ffi::c_int;
    choices[3usize].selected = opts.scanlines.0 as ::core::ffi::c_int;
    choices[4usize].selected = opts.menu.0 as ::core::ffi::c_int;
    choices[5usize].selected = opts.text.0 as ::core::ffi::c_int;
    choices[6usize].selected = opts.cscan.0 as ::core::ffi::c_int;
    choices[7usize].selected = opts.scroll.0 as ::core::ffi::c_int;
    choices[8usize].selected = opts.subtitles.0 as ::core::ffi::c_int;
    choices[9usize].selected = opts.music3do.0 as ::core::ffi::c_int;
    choices[10usize].selected = opts.fullscreen.0 as ::core::ffi::c_int;
    choices[11usize].selected = opts.intro.0 as ::core::ffi::c_int;
    choices[12usize].selected = opts.fps.0 as ::core::ffi::c_int;
    choices[13usize].selected = opts.meleezoom.0 as ::core::ffi::c_int;
    choices[14usize].selected = opts.stereo.0 as ::core::ffi::c_int;
    choices[15usize].selected = opts.adriver.0 as ::core::ffi::c_int;
    choices[16usize].selected = opts.aquality.0 as ::core::ffi::c_int;
    choices[17usize].selected = opts.shield.0 as ::core::ffi::c_int;
    choices[18usize].selected = opts.player1.0 as ::core::ffi::c_int;
    choices[19usize].selected = opts.player2.0 as ::core::ffi::c_int;
    choices[20usize].selected = 0 as ::core::ffi::c_int;
    choices[21usize].selected = opts.musicremix.0 as ::core::ffi::c_int;
    choices[22usize].selected = opts.speech.0 as ::core::ffi::c_int;
    choices[23usize].selected = opts.keepaspect.0 as ::core::ffi::c_int;
    sliders[0usize].value = opts.musicvol;
    sliders[1usize].value = opts.sfxvol;
    sliders[2usize].value = opts.speechvol;
    sliders[3usize].value = opts.gamma;
}
unsafe extern "C" fn PropagateResults() {
    let mut opts: GLOBALOPTS = GLOBALOPTS {
        scaler: OPT_SCALETYPE::OPTVAL_NO_SCALE,
        res: OPT_RESTYPE::OPTVAL_320_240,
        driver: OPT_DRIVERTYPE::OPTVAL_PURE_IF_POSSIBLE,
        adriver: OPT_ADRIVERTYPE::OPTVAL_SILENCE,
        aquality: OPT_AQUALITYTYPE::OPTVAL_LOW,
        fullscreen: OPT_ENABLABLE::OPTVAL_DISABLED,
        subtitles: OPT_ENABLABLE::OPTVAL_DISABLED,
        scanlines: OPT_ENABLABLE::OPTVAL_DISABLED,
        fps: OPT_ENABLABLE::OPTVAL_DISABLED,
        stereo: OPT_ENABLABLE::OPTVAL_DISABLED,
        music3do: OPT_ENABLABLE::OPTVAL_DISABLED,
        musicremix: OPT_ENABLABLE::OPTVAL_DISABLED,
        speech: OPT_ENABLABLE::OPTVAL_DISABLED,
        keepaspect: OPT_ENABLABLE::OPTVAL_DISABLED,
        menu: OPT_CONSOLETYPE::OPTVAL_PC,
        text: OPT_CONSOLETYPE::OPTVAL_PC,
        cscan: OPT_CONSOLETYPE::OPTVAL_PC,
        scroll: OPT_CONSOLETYPE::OPTVAL_PC,
        intro: OPT_CONSOLETYPE::OPTVAL_PC,
        meleezoom: OPT_CONSOLETYPE::OPTVAL_PC,
        shield: OPT_CONSOLETYPE::OPTVAL_PC,
        player1: CONTROL_TEMPLATE::CONTROL_TEMPLATE_KB_1,
        player2: CONTROL_TEMPLATE::CONTROL_TEMPLATE_KB_1,
        speechvol: 0,
        musicvol: 0,
        sfxvol: 0,
        gamma: 0,
    };
    opts.res = OPT_RESTYPE(choices[0usize].selected as ::core::ffi::c_uint);
    opts.driver = OPT_DRIVERTYPE(choices[1usize].selected as ::core::ffi::c_uint);
    opts.scaler = OPT_SCALETYPE(choices[2usize].selected as ::core::ffi::c_uint);
    opts.scanlines = OPT_ENABLABLE(choices[3usize].selected as ::core::ffi::c_uint);
    opts.menu = OPT_CONSOLETYPE(choices[4usize].selected as ::core::ffi::c_uint);
    opts.text = OPT_CONSOLETYPE(choices[5usize].selected as ::core::ffi::c_uint);
    opts.cscan = OPT_CONSOLETYPE(choices[6usize].selected as ::core::ffi::c_uint);
    opts.scroll = OPT_CONSOLETYPE(choices[7usize].selected as ::core::ffi::c_uint);
    opts.subtitles = OPT_ENABLABLE(choices[8usize].selected as ::core::ffi::c_uint);
    opts.music3do = OPT_ENABLABLE(choices[9usize].selected as ::core::ffi::c_uint);
    opts.fullscreen = OPT_ENABLABLE(choices[10usize].selected as ::core::ffi::c_uint);
    opts.intro = OPT_CONSOLETYPE(choices[11usize].selected as ::core::ffi::c_uint);
    opts.fps = OPT_ENABLABLE(choices[12usize].selected as ::core::ffi::c_uint);
    opts.meleezoom = OPT_CONSOLETYPE(choices[13usize].selected as ::core::ffi::c_uint);
    opts.stereo = OPT_ENABLABLE(choices[14usize].selected as ::core::ffi::c_uint);
    opts.adriver = OPT_ADRIVERTYPE(choices[15usize].selected as ::core::ffi::c_uint);
    opts.aquality = OPT_AQUALITYTYPE(choices[16usize].selected as ::core::ffi::c_uint);
    opts.shield = OPT_CONSOLETYPE(choices[17usize].selected as ::core::ffi::c_uint);
    opts.player1 = CONTROL_TEMPLATE(choices[18usize].selected as ::core::ffi::c_uint);
    opts.player2 = CONTROL_TEMPLATE(choices[19usize].selected as ::core::ffi::c_uint);
    opts.musicremix = OPT_ENABLABLE(choices[21usize].selected as ::core::ffi::c_uint);
    opts.speech = OPT_ENABLABLE(choices[22usize].selected as ::core::ffi::c_uint);
    opts.keepaspect = OPT_ENABLABLE(choices[23usize].selected as ::core::ffi::c_uint);
    opts.musicvol = sliders[0usize].value;
    opts.sfxvol = sliders[1usize].value;
    opts.speechvol = sliders[2usize].value;
    opts.gamma = sliders[3usize].value;
    SetGlobalOptions(&raw mut opts);
}
unsafe extern "C" fn DoSetupMenu(mut pInputState: *mut SETUP_MENU_STATE) -> BOOLEAN {
    ::core::ptr::write_volatile(&raw mut GamePaused, BOOLEAN::FALSE);
    if (*pInputState).initialized.0 == 0 {
        SetDefaultMenuRepeatDelay();
        (*pInputState).NextTime = GetTimeCounter() as DWORD;
        SetDefaults();
        Widget_SetFont(StarConFont);
        Widget_SetWindowColors(
            colorFromRgb15(0x14 as BYTE, 0x14 as BYTE, 0x14 as BYTE),
            colorFromRgb15(0x8 as BYTE, 0x8 as BYTE, 0x8 as BYTE),
            colorFromRgb15(0x10 as BYTE, 0x10 as BYTE, 0x10 as BYTE),
        );
        current = ::core::ptr::null_mut::<WIDGET>();
        next = (&raw mut menus as *mut WIDGET_MENU_SCREEN).offset(0isize) as *mut WIDGET;
        Some((*next).receiveFocus.expect("non-null function pointer"))
            .expect(
                "non-null function pointer",
            )(next, C2Rust_Unnamed_5::WIDGET_EVENT_DOWN.0 as ::core::ffi::c_int);
        (*pInputState).initialized = BOOLEAN::TRUE;
    }
    if current != next {
        SetTransitionSource(::core::ptr::null::<RECT>());
    }
    BatchGraphics();
    Some((*next).draw.expect("non-null function pointer"))
        .expect(
            "non-null function pointer",
        )(next, 0 as ::core::ffi::c_int, 0 as ::core::ffi::c_int);
    if current != next {
        ScreenTransition(3 as ::core::ffi::c_int, ::core::ptr::null::<RECT>());
        current = next;
    }
    UnbatchGraphics();
    if PulsedInputState
        .menu[C2Rust_Unnamed_0::KEY_MENU_UP.0 as ::core::ffi::c_int as usize] != 0
    {
        Widget_Event(C2Rust_Unnamed_5::WIDGET_EVENT_UP.0 as ::core::ffi::c_int);
    } else if PulsedInputState
        .menu[C2Rust_Unnamed_0::KEY_MENU_DOWN.0 as ::core::ffi::c_int as usize] != 0
    {
        Widget_Event(C2Rust_Unnamed_5::WIDGET_EVENT_DOWN.0 as ::core::ffi::c_int);
    } else if PulsedInputState
        .menu[C2Rust_Unnamed_0::KEY_MENU_LEFT.0 as ::core::ffi::c_int as usize] != 0
    {
        Widget_Event(C2Rust_Unnamed_5::WIDGET_EVENT_LEFT.0 as ::core::ffi::c_int);
    } else if PulsedInputState
        .menu[C2Rust_Unnamed_0::KEY_MENU_RIGHT.0 as ::core::ffi::c_int as usize] != 0
    {
        Widget_Event(C2Rust_Unnamed_5::WIDGET_EVENT_RIGHT.0 as ::core::ffi::c_int);
    }
    if PulsedInputState
        .menu[C2Rust_Unnamed_0::KEY_MENU_SELECT.0 as ::core::ffi::c_int as usize] != 0
    {
        Widget_Event(C2Rust_Unnamed_5::WIDGET_EVENT_SELECT.0 as ::core::ffi::c_int);
    }
    if PulsedInputState
        .menu[C2Rust_Unnamed_0::KEY_MENU_CANCEL.0 as ::core::ffi::c_int as usize] != 0
    {
        Widget_Event(C2Rust_Unnamed_5::WIDGET_EVENT_CANCEL.0 as ::core::ffi::c_int);
    }
    if PulsedInputState
        .menu[C2Rust_Unnamed_0::KEY_MENU_DELETE.0 as ::core::ffi::c_int as usize] != 0
    {
        Widget_Event(C2Rust_Unnamed_5::WIDGET_EVENT_DELETE.0 as ::core::ffi::c_int);
    }
    SleepThreadUntil(
        (*pInputState)
            .NextTime
            .wrapping_add(
                (840 as ::core::ffi::c_int / 20 as ::core::ffi::c_int) as TimeCount,
            ),
    );
    (*pInputState).NextTime = GetTimeCounter() as DWORD;
    return BOOLEAN(
        !(GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
            & C2Rust_Unnamed_2::CHECK_ABORT.0 as ::core::ffi::c_int != 0
            || next.is_null()) as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
}
unsafe extern "C" fn redraw_menu() {
    BatchGraphics();
    Some((*next).draw.expect("non-null function pointer"))
        .expect(
            "non-null function pointer",
        )(next, 0 as ::core::ffi::c_int, 0 as ::core::ffi::c_int);
    UnbatchGraphics();
}
unsafe extern "C" fn OnTextEntryChange(mut pTES: *mut TEXTENTRY_STATE) -> BOOLEAN {
    let mut widget: *mut WIDGET_TEXTENTRY = (*pTES).CbParam as *mut WIDGET_TEXTENTRY;
    (*widget).cursor_pos = (*pTES).CursorPos;
    if (*pTES).JoystickMode.0 != 0 {
        (*widget).state = WIDGET_TEXTENTRY_STATE(
            (*widget).state.0 | WIDGET_TEXTENTRY_STATE::WTE_BLOCKCUR.0,
        );
    } else {
        (*widget).state = WIDGET_TEXTENTRY_STATE(
            (*widget).state.0
                & !(WIDGET_TEXTENTRY_STATE::WTE_BLOCKCUR.0 as ::core::ffi::c_int)
                    as ::core::ffi::c_uint,
        );
    }
    return BOOLEAN::TRUE;
}
unsafe extern "C" fn OnTextEntryFrame(mut pTES: *mut TEXTENTRY_STATE) -> BOOLEAN {
    redraw_menu();
    SleepThreadUntil((*pTES).NextTime);
    (*pTES).NextTime = GetTimeCounter()
        .wrapping_add(
            (840 as ::core::ffi::c_int / 20 as ::core::ffi::c_int) as TimeCount,
        ) as DWORD;
    return BOOLEAN::TRUE;
}
unsafe extern "C" fn OnTextEntryEvent(
    mut widget: *mut WIDGET_TEXTENTRY,
) -> ::core::ffi::c_int {
    let mut tes: TEXTENTRY_STATE = TEXTENTRY_STATE {
        InputFunc: None,
        Initialized: BOOLEAN::FALSE,
        NextTime: 0,
        Success: BOOLEAN::FALSE,
        CacheStr: ::core::ptr::null_mut::<UNICODE>(),
        JoyAlphaString: ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>(),
        JoystickMode: BOOLEAN::FALSE,
        UpperRegister: BOOLEAN::FALSE,
        JoyAlpha: ::core::ptr::null_mut::<joy_char_t>(),
        JoyAlphaLength: 0,
        JoyUpper: ::core::ptr::null_mut::<joy_char_t>(),
        JoyLower: ::core::ptr::null_mut::<joy_char_t>(),
        JoyRegLength: 0,
        InsPt: ::core::ptr::null_mut::<UNICODE>(),
        BaseStr: ::core::ptr::null_mut::<UNICODE>(),
        CursorPos: 0,
        MaxSize: 0,
        ChangeCallback: None,
        FrameCallback: None,
        CbParam: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    let mut revert_buf: [UNICODE; 256] = [0; 256];
    (*widget).cursor_pos = utf8StringCount(
        &raw mut (*widget).value as *mut ::core::ffi::c_char,
    ) as ::core::ffi::c_int;
    (*widget).state = WIDGET_TEXTENTRY_STATE::WTE_EDITING;
    redraw_menu();
    utf8StringCopy(
        &raw mut revert_buf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[UNICODE; 256]>(),
        &raw mut (*widget).value as *mut ::core::ffi::c_char,
    );
    tes.Initialized = BOOLEAN::FALSE;
    tes.NextTime = GetTimeCounter()
        .wrapping_add(
            (840 as ::core::ffi::c_int / 20 as ::core::ffi::c_int) as TimeCount,
        ) as DWORD;
    tes.BaseStr = &raw mut (*widget).value as *mut ::core::ffi::c_char as *mut UNICODE;
    tes.MaxSize = (*widget).maxlen;
    tes.CursorPos = (*widget).cursor_pos;
    tes.CbParam = widget as *mut ::core::ffi::c_void;
    tes.ChangeCallback = Some(
        OnTextEntryChange as unsafe extern "C" fn(*mut TEXTENTRY_STATE) -> BOOLEAN,
    ) as Option<unsafe extern "C" fn(*mut textentry_state) -> BOOLEAN>;
    tes.FrameCallback = Some(
        OnTextEntryFrame as unsafe extern "C" fn(*mut TEXTENTRY_STATE) -> BOOLEAN,
    ) as Option<unsafe extern "C" fn(*mut textentry_state) -> BOOLEAN>;
    SetMenuSounds(
        0 as ::core::ffi::c_int as MENU_SOUND_FLAGS,
        ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as MENU_SOUND_FLAGS,
    );
    if DoTextEntry(&raw mut tes).0 == 0 {
        utf8StringCopy(
            &raw mut (*widget).value as *mut ::core::ffi::c_char,
            (*widget).maxlen as size_t,
            &raw mut revert_buf as *mut UNICODE,
        );
    } else if (*widget).onChange.is_some() {
        Some((*widget).onChange.expect("non-null function pointer"))
            .expect("non-null function pointer")(widget);
    }
    SetMenuSounds(
        (((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as MENU_SOUND_FLAGS
            as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as MENU_SOUND_FLAGS
                as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as MENU_SOUND_FLAGS
                as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as MENU_SOUND_FLAGS
                as ::core::ffi::c_int) as MENU_SOUND_FLAGS,
        ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as MENU_SOUND_FLAGS,
    );
    (*widget).state = WIDGET_TEXTENTRY_STATE::WTE_NORMAL;
    redraw_menu();
    return BOOLEAN::TRUE.0 as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn gammaCurve(mut x: ::core::ffi::c_float) -> ::core::ffi::c_float {
    return exp(
        ((x - 1 as ::core::ffi::c_int as ::core::ffi::c_float) * 2.1972f32)
            as ::core::ffi::c_double,
    ) as ::core::ffi::c_float;
}
#[inline]
unsafe extern "C" fn solveGammaCurve(
    mut y: ::core::ffi::c_float,
) -> ::core::ffi::c_float {
    return (log(y as ::core::ffi::c_double) / 2.1972f64
        + 1 as ::core::ffi::c_int as ::core::ffi::c_double) as ::core::ffi::c_float;
}
unsafe extern "C" fn gammaToSlider(
    mut gamma: ::core::ffi::c_float,
) -> ::core::ffi::c_int {
    let x: ::core::ffi::c_float = solveGammaCurve(gamma);
    let step: ::core::ffi::c_float = (maxGammaX - minGammaX)
        / 100 as ::core::ffi::c_int as ::core::ffi::c_float;
    return (((x - minGammaX) / step) as ::core::ffi::c_double + 0.5f64)
        as ::core::ffi::c_int;
}
unsafe extern "C" fn sliderToGamma(
    mut value: ::core::ffi::c_int,
) -> ::core::ffi::c_float {
    let step: ::core::ffi::c_float = (maxGammaX - minGammaX)
        / 100 as ::core::ffi::c_int as ::core::ffi::c_float;
    let x: ::core::ffi::c_float = minGammaX + step * value as ::core::ffi::c_float;
    let g: ::core::ffi::c_float = gammaCurve(x);
    return if fabs((g - 1.0f32) as ::core::ffi::c_double) < 0.001f64 {
        1.0f32
    } else {
        g
    };
}
unsafe extern "C" fn updateGammaBounds(mut useUpper: bool) {
    let mut g: ::core::ffi::c_float = 0.;
    let mut x: ::core::ffi::c_float = 0.;
    let mut slider: ::core::ffi::c_int = 0;
    minGammaX = solveGammaCurve(minGamma);
    maxGammaX = solveGammaCurve(maxGamma);
    slider = gammaToSlider(1.0f32);
    g = sliderToGamma(slider);
    if g == 1.0f32 {
        return;
    }
    x = solveGammaCurve(g);
    if useUpper {
        let d: ::core::ffi::c_float = (x - 1.0f32)
            * 100 as ::core::ffi::c_int as ::core::ffi::c_float
            / slider as ::core::ffi::c_float;
        maxGammaX -= d;
        maxGamma = gammaCurve(maxGammaX);
    } else {
        let d_0: ::core::ffi::c_float = (x - 1.0f32)
            * 100 as ::core::ffi::c_int as ::core::ffi::c_float
            / (100 as ::core::ffi::c_int - slider) as ::core::ffi::c_float;
        minGammaX -= d_0;
        minGamma = gammaCurve(minGammaX);
    };
}
unsafe extern "C" fn gamma_HandleEventSlider(
    mut _self: *mut WIDGET,
    mut event: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut self_0: *mut WIDGET_SLIDER = _self as *mut WIDGET_SLIDER;
    let mut prevValue: ::core::ffi::c_int = (*self_0).value;
    let mut gamma: ::core::ffi::c_float = 0.;
    let mut set: bool = false;
    match event {
        2 => {
            (*self_0).value -= (*self_0).step;
        }
        3 => {
            (*self_0).value += (*self_0).step;
        }
        _ => return BOOLEAN::FALSE.0 as ::core::ffi::c_int,
    }
    gamma = sliderToGamma((*self_0).value);
    set = TFB_SetGamma(gamma);
    if !set {
        (*self_0).value = prevValue;
        gamma = sliderToGamma((*self_0).value);
    }
    if gamma < minGamma
        || !set && event == C2Rust_Unnamed_5::WIDGET_EVENT_LEFT.0 as ::core::ffi::c_int
    {
        minGamma = gamma;
        updateGammaBounds(true);
        (*self_0).value = 0 as ::core::ffi::c_int;
    } else if gamma > maxGamma
        || !set && event == C2Rust_Unnamed_5::WIDGET_EVENT_RIGHT.0 as ::core::ffi::c_int
    {
        maxGamma = gamma;
        updateGammaBounds(false);
        (*self_0).value = 100 as ::core::ffi::c_int;
    }
    return BOOLEAN::TRUE.0 as ::core::ffi::c_int;
}
unsafe extern "C" fn gamma_DrawValue(
    mut self_0: *mut WIDGET_SLIDER,
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
) {
    let mut t: TEXT = TEXT {
        baseline: POINT { x: 0, y: 0 },
        pStr: ::core::ptr::null::<UNICODE>(),
        align: TEXT_ALIGN::ALIGN_LEFT,
        CharCount: 0,
    };
    let mut buf: [::core::ffi::c_char; 16] = [0; 16];
    let mut gamma: ::core::ffi::c_float = sliderToGamma((*self_0).value);
    snprintf(
        &raw mut buf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
        b"%.4f\0".as_ptr() as *const ::core::ffi::c_char,
        gamma as ::core::ffi::c_double,
    );
    t.baseline.x = x as COORD;
    t.baseline.y = y as COORD;
    t.align = TEXT_ALIGN::ALIGN_CENTER;
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    t.pStr = &raw mut buf as *mut ::core::ffi::c_char;
    font_DrawText(&raw mut t);
}
unsafe extern "C" fn rebind_control(mut widget: *mut WIDGET_CONTROLENTRY) {
    let mut templat: ::core::ffi::c_int = choices[20usize].selected;
    let mut control: ::core::ffi::c_int = (*widget).controlindex;
    let mut index: ::core::ffi::c_int = (*widget).highlighted;
    FlushInput();
    DrawLabelAsWindow(
        (&raw mut labels as *mut WIDGET_LABEL).offset(3isize),
        ::core::ptr::null_mut::<RECT>(),
    );
    RebindInputState(templat, control, index);
    populate_editkeys(templat);
    FlushInput();
}
unsafe extern "C" fn clear_control(mut widget: *mut WIDGET_CONTROLENTRY) {
    let mut templat: ::core::ffi::c_int = choices[20usize].selected;
    let mut control: ::core::ffi::c_int = (*widget).controlindex;
    let mut index: ::core::ffi::c_int = (*widget).highlighted;
    RemoveInputState(templat, control, index);
    populate_editkeys(templat);
}
unsafe extern "C" fn count_widgets(mut widgets: *mut *mut WIDGET) -> ::core::ffi::c_int {
    let mut count: ::core::ffi::c_int = 0;
    count = 0 as ::core::ffi::c_int;
    while !(*widgets).is_null() {
        widgets = widgets.offset(1);
        count += 1;
    }
    return count;
}
static mut bank: *mut stringbank = ::core::ptr::null_mut::<stringbank>();
static mut setup_frame: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
unsafe extern "C" fn init_widgets() {
    let mut buffer: [*const ::core::ffi::c_char; 100] = [::core::ptr::null::<
        ::core::ffi::c_char,
    >(); 100];
    let mut str: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut title: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    let mut count: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut index: ::core::ffi::c_int = 0;
    if bank.is_null() {
        bank = StringBank_Create();
    }
    if setup_frame.is_null() {
        setup_frame = CaptureDrawable(
            LoadGraphicInstance(b"graphics.setupmenu\0".as_ptr() as RESOURCE) as DRAWABLE,
        );
    }
    count = GetStringTableCount(SetupTab) as ::core::ffi::c_int;
    if count < 3 as ::core::ffi::c_int {
        log_add(
            log_Level::log_Fatal,
            b"PANIC: Setup string table too short to even hold all indices!\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        exit(1 as ::core::ffi::c_int);
    }
    title = StringBank_AddOrFindString(
        bank,
        GetStringAddress(SetAbsStringTableIndex(SetupTab, 0 as COUNT))
            as *const ::core::ffi::c_char,
    );
    if SplitString(
        GetStringAddress(SetAbsStringTableIndex(SetupTab, 1 as COUNT))
            as *const ::core::ffi::c_char,
        '\n' as ::core::ffi::c_char,
        100 as ::core::ffi::c_int,
        &raw mut buffer as *mut *const ::core::ffi::c_char,
        bank,
    ) != 8 as ::core::ffi::c_int
    {
        log_add(
            log_Level::log_Fatal,
            b"PANIC: Incorrect number of Menu Subtitles\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        exit(1 as ::core::ffi::c_int);
    }
    i = 0 as ::core::ffi::c_int;
    while i < 8 as ::core::ffi::c_int {
        menus[i as usize].tag = WIDGET_TYPE::WIDGET_TYPE_MENU_SCREEN;
        menus[i as usize].parent = ::core::ptr::null_mut::<_widget>();
        menus[i as usize].handleEvent = Some(
            Widget_HandleEventMenuScreen
                as unsafe extern "C" fn(
                    *mut WIDGET,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut _widget,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
            >;
        menus[i as usize].receiveFocus = Some(
            Widget_ReceiveFocusMenuScreen
                as unsafe extern "C" fn(
                    *mut WIDGET,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut _widget,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
            >;
        menus[i as usize].draw = Some(
            Widget_DrawMenuScreen
                as unsafe extern "C" fn(
                    *mut WIDGET,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C" fn(
                    *mut _widget,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> (),
            >;
        menus[i as usize].height = Some(
            Widget_HeightFullScreen
                as unsafe extern "C" fn(*mut WIDGET) -> ::core::ffi::c_int,
        ) as Option<unsafe extern "C" fn(*mut _widget) -> ::core::ffi::c_int>;
        menus[i as usize].width = Some(
            Widget_WidthFullScreen
                as unsafe extern "C" fn(*mut WIDGET) -> ::core::ffi::c_int,
        ) as Option<unsafe extern "C" fn(*mut _widget) -> ::core::ffi::c_int>;
        menus[i as usize].title = title;
        menus[i as usize].subtitle = buffer[i as usize];
        menus[i as usize].bgStamp.origin.x = 0 as COORD;
        menus[i as usize].bgStamp.origin.y = 0 as COORD;
        menus[i as usize].bgStamp.frame = SetAbsFrameIndex(
            setup_frame,
            menu_defs[i as usize].bgIndex as COUNT,
        );
        menus[i as usize].num_children = count_widgets(menu_defs[i as usize].widgets);
        menus[i as usize].child = menu_defs[i as usize].widgets as *mut *mut _widget;
        menus[i as usize].highlighted = 0 as ::core::ffi::c_int;
        i += 1;
    }
    if !menu_defs[i as usize].widgets.is_null() {
        log_add(
            log_Level::log_Error,
            b"Menu definition array has more items!\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if SplitString(
        GetStringAddress(SetAbsStringTableIndex(SetupTab, 2 as COUNT))
            as *const ::core::ffi::c_char,
        '\n' as ::core::ffi::c_char,
        100 as ::core::ffi::c_int,
        &raw mut buffer as *mut *const ::core::ffi::c_char,
        bank,
    ) != 24 as ::core::ffi::c_int
    {
        log_add(
            log_Level::log_Fatal,
            b"PANIC: Incorrect number of Choice Options\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        exit(1 as ::core::ffi::c_int);
    }
    i = 0 as ::core::ffi::c_int;
    while i < 24 as ::core::ffi::c_int {
        choices[i as usize].tag = WIDGET_TYPE::WIDGET_TYPE_CHOICE;
        choices[i as usize].parent = ::core::ptr::null_mut::<_widget>();
        choices[i as usize].handleEvent = Some(
            Widget_HandleEventChoice
                as unsafe extern "C" fn(
                    *mut WIDGET,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut _widget,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
            >;
        choices[i as usize].receiveFocus = Some(
            Widget_ReceiveFocusChoice
                as unsafe extern "C" fn(
                    *mut WIDGET,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut _widget,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
            >;
        choices[i as usize].draw = Some(
            Widget_DrawChoice
                as unsafe extern "C" fn(
                    *mut WIDGET,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C" fn(
                    *mut _widget,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> (),
            >;
        choices[i as usize].height = Some(
            Widget_HeightChoice
                as unsafe extern "C" fn(*mut WIDGET) -> ::core::ffi::c_int,
        ) as Option<unsafe extern "C" fn(*mut _widget) -> ::core::ffi::c_int>;
        choices[i as usize].width = Some(
            Widget_WidthFullScreen
                as unsafe extern "C" fn(*mut WIDGET) -> ::core::ffi::c_int,
        ) as Option<unsafe extern "C" fn(*mut _widget) -> ::core::ffi::c_int>;
        choices[i as usize].category = buffer[i as usize];
        choices[i as usize].numopts = 0 as ::core::ffi::c_int;
        choices[i as usize].options = ::core::ptr::null_mut::<CHOICE_OPTION>();
        choices[i as usize].selected = 0 as ::core::ffi::c_int;
        choices[i as usize].highlighted = 0 as ::core::ffi::c_int;
        choices[i as usize].maxcolumns = choice_widths[i as usize];
        choices[i as usize].onChange = None;
        i += 1;
    }
    index = 3 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < 24 as ::core::ffi::c_int {
        let mut j: ::core::ffi::c_int = 0;
        let mut optcount: ::core::ffi::c_int = 0;
        if index >= count {
            log_add(
                log_Level::log_Fatal,
                b"PANIC: String table cut short while reading choices\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            exit(1 as ::core::ffi::c_int);
        }
        let c2rust_fresh0 = index;
        index += 1;
        str = GetStringAddress(SetAbsStringTableIndex(SetupTab, c2rust_fresh0 as COUNT))
            as *const ::core::ffi::c_char;
        optcount = SplitString(
            str,
            '\n' as ::core::ffi::c_char,
            100 as ::core::ffi::c_int,
            &raw mut buffer as *mut *const ::core::ffi::c_char,
            bank,
        );
        choices[i as usize].numopts = optcount;
        choices[i as usize].options = HMalloc(
            (optcount as size_t).wrapping_mul(::core::mem::size_of::<CHOICE_OPTION>()),
        ) as *mut CHOICE_OPTION;
        j = 0 as ::core::ffi::c_int;
        while j < optcount {
            (*choices[i as usize].options.offset(j as isize)).optname = buffer[j
                as usize];
            (*choices[i as usize].options.offset(j as isize)).tooltip[0usize] = b"\0"
                .as_ptr() as *const ::core::ffi::c_char;
            (*choices[i as usize].options.offset(j as isize)).tooltip[1usize] = b"\0"
                .as_ptr() as *const ::core::ffi::c_char;
            (*choices[i as usize].options.offset(j as isize)).tooltip[2usize] = b"\0"
                .as_ptr() as *const ::core::ffi::c_char;
            j += 1;
        }
        j = 0 as ::core::ffi::c_int;
        while j < optcount {
            let mut k: ::core::ffi::c_int = 0;
            let mut tipcount: ::core::ffi::c_int = 0;
            if index >= count {
                log_add(
                    log_Level::log_Fatal,
                    b"PANIC: String table cut short while reading choices\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                exit(1 as ::core::ffi::c_int);
            }
            let c2rust_fresh1 = index;
            index += 1;
            str = GetStringAddress(
                SetAbsStringTableIndex(SetupTab, c2rust_fresh1 as COUNT),
            ) as *const ::core::ffi::c_char;
            tipcount = SplitString(
                str,
                '\n' as ::core::ffi::c_char,
                100 as ::core::ffi::c_int,
                &raw mut buffer as *mut *const ::core::ffi::c_char,
                bank,
            );
            if tipcount > 3 as ::core::ffi::c_int {
                tipcount = 3 as ::core::ffi::c_int;
            }
            k = 0 as ::core::ffi::c_int;
            while k < tipcount {
                (*choices[i as usize].options.offset(j as isize)).tooltip[k as usize] = buffer[k
                    as usize];
                k += 1;
            }
            j += 1;
        }
        i += 1;
    }
    choices[0usize].numopts = number_res_options();
    i = 0 as ::core::ffi::c_int;
    while i < 6 as ::core::ffi::c_int {
        (*choices[18usize].options.offset(i as isize)).optname = &raw mut (*(&raw mut input_templates
            as *mut INPUT_TEMPLATE)
            .offset(i as isize))
            .name as *mut ::core::ffi::c_char;
        (*choices[19usize].options.offset(i as isize)).optname = &raw mut (*(&raw mut input_templates
            as *mut INPUT_TEMPLATE)
            .offset(i as isize))
            .name as *mut ::core::ffi::c_char;
        (*choices[20usize].options.offset(i as isize)).optname = &raw mut (*(&raw mut input_templates
            as *mut INPUT_TEMPLATE)
            .offset(i as isize))
            .name as *mut ::core::ffi::c_char;
        i += 1;
    }
    choices[20usize].onChange = Some(
        change_template
            as unsafe extern "C" fn(*mut WIDGET_CHOICE, ::core::ffi::c_int) -> (),
    ) as Option<unsafe extern "C" fn(*mut _widget_choice, ::core::ffi::c_int) -> ()>;
    if index >= count {
        log_add(
            log_Level::log_Fatal,
            b"PANIC: String table cut short while reading sliders\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        exit(1 as ::core::ffi::c_int);
    }
    let c2rust_fresh2 = index;
    index += 1;
    if SplitString(
        GetStringAddress(SetAbsStringTableIndex(SetupTab, c2rust_fresh2 as COUNT))
            as *const ::core::ffi::c_char,
        '\n' as ::core::ffi::c_char,
        100 as ::core::ffi::c_int,
        &raw mut buffer as *mut *const ::core::ffi::c_char,
        bank,
    ) != 4 as ::core::ffi::c_int
    {
        log_add(
            log_Level::log_Fatal,
            b"PANIC: Incorrect number of Slider Options\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        exit(1 as ::core::ffi::c_int);
    }
    i = 0 as ::core::ffi::c_int;
    while i < 4 as ::core::ffi::c_int {
        sliders[i as usize].tag = WIDGET_TYPE::WIDGET_TYPE_SLIDER;
        sliders[i as usize].parent = ::core::ptr::null_mut::<_widget>();
        sliders[i as usize].handleEvent = Some(
            Widget_HandleEventSlider
                as unsafe extern "C" fn(
                    *mut WIDGET,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut _widget,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
            >;
        sliders[i as usize].receiveFocus = Some(
            Widget_ReceiveFocusSimple
                as unsafe extern "C" fn(
                    *mut WIDGET,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut _widget,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
            >;
        sliders[i as usize].draw = Some(
            Widget_DrawSlider
                as unsafe extern "C" fn(
                    *mut WIDGET,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C" fn(
                    *mut _widget,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> (),
            >;
        sliders[i as usize].height = Some(
            Widget_HeightOneLine
                as unsafe extern "C" fn(*mut WIDGET) -> ::core::ffi::c_int,
        ) as Option<unsafe extern "C" fn(*mut _widget) -> ::core::ffi::c_int>;
        sliders[i as usize].width = Some(
            Widget_WidthFullScreen
                as unsafe extern "C" fn(*mut WIDGET) -> ::core::ffi::c_int,
        ) as Option<unsafe extern "C" fn(*mut _widget) -> ::core::ffi::c_int>;
        sliders[i as usize].draw_value = Some(
            Widget_Slider_DrawValue
                as unsafe extern "C" fn(
                    *mut WIDGET_SLIDER,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C" fn(
                    *mut _widget_slider,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> (),
            >;
        sliders[i as usize].min = 0 as ::core::ffi::c_int;
        sliders[i as usize].max = 100 as ::core::ffi::c_int;
        sliders[i as usize].step = 5 as ::core::ffi::c_int;
        sliders[i as usize].value = 75 as ::core::ffi::c_int;
        sliders[i as usize].category = buffer[i as usize];
        sliders[i as usize].tooltip[0usize] = b"\0".as_ptr()
            as *const ::core::ffi::c_char;
        sliders[i as usize].tooltip[1usize] = b"\0".as_ptr()
            as *const ::core::ffi::c_char;
        sliders[i as usize].tooltip[2usize] = b"\0".as_ptr()
            as *const ::core::ffi::c_char;
        i += 1;
    }
    sliders[3usize].step = 1 as ::core::ffi::c_int;
    sliders[3usize].handleEvent = Some(
        gamma_HandleEventSlider
            as unsafe extern "C" fn(
                *mut WIDGET,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
    )
        as Option<
            unsafe extern "C" fn(*mut _widget, ::core::ffi::c_int) -> ::core::ffi::c_int,
        >;
    sliders[3usize].draw_value = Some(
        gamma_DrawValue
            as unsafe extern "C" fn(
                *mut WIDGET_SLIDER,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *mut _widget_slider,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
            ) -> (),
        >;
    i = 0 as ::core::ffi::c_int;
    while i < 4 as ::core::ffi::c_int {
        let mut j_0: ::core::ffi::c_int = 0;
        let mut tipcount_0: ::core::ffi::c_int = 0;
        if index >= count {
            log_add(
                log_Level::log_Fatal,
                b"PANIC: String table cut short while reading sliders\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            exit(1 as ::core::ffi::c_int);
        }
        let c2rust_fresh3 = index;
        index += 1;
        str = GetStringAddress(SetAbsStringTableIndex(SetupTab, c2rust_fresh3 as COUNT))
            as *const ::core::ffi::c_char;
        tipcount_0 = SplitString(
            str,
            '\n' as ::core::ffi::c_char,
            100 as ::core::ffi::c_int,
            &raw mut buffer as *mut *const ::core::ffi::c_char,
            bank,
        );
        if tipcount_0 > 3 as ::core::ffi::c_int {
            tipcount_0 = 3 as ::core::ffi::c_int;
        }
        j_0 = 0 as ::core::ffi::c_int;
        while j_0 < tipcount_0 {
            sliders[i as usize].tooltip[j_0 as usize] = buffer[j_0 as usize];
            j_0 += 1;
        }
        i += 1;
    }
    if index >= count {
        log_add(
            log_Level::log_Fatal,
            b"PANIC: String table cut short while reading buttons\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        exit(1 as ::core::ffi::c_int);
    }
    let c2rust_fresh4 = index;
    index += 1;
    if SplitString(
        GetStringAddress(SetAbsStringTableIndex(SetupTab, c2rust_fresh4 as COUNT))
            as *const ::core::ffi::c_char,
        '\n' as ::core::ffi::c_char,
        100 as ::core::ffi::c_int,
        &raw mut buffer as *mut *const ::core::ffi::c_char,
        bank,
    ) != 10 as ::core::ffi::c_int
    {
        log_add(
            log_Level::log_Fatal,
            b"PANIC: Incorrect number of Button Options\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        exit(1 as ::core::ffi::c_int);
    }
    i = 0 as ::core::ffi::c_int;
    while i < 10 as ::core::ffi::c_int {
        buttons[i as usize].tag = WIDGET_TYPE::WIDGET_TYPE_BUTTON;
        buttons[i as usize].parent = ::core::ptr::null_mut::<_widget>();
        buttons[i as usize].handleEvent = button_handlers[i as usize]
            as Option<
                unsafe extern "C" fn(
                    *mut _widget,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
            >;
        buttons[i as usize].receiveFocus = Some(
            Widget_ReceiveFocusSimple
                as unsafe extern "C" fn(
                    *mut WIDGET,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut _widget,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
            >;
        buttons[i as usize].draw = Some(
            Widget_DrawButton
                as unsafe extern "C" fn(
                    *mut WIDGET,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C" fn(
                    *mut _widget,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> (),
            >;
        buttons[i as usize].height = Some(
            Widget_HeightOneLine
                as unsafe extern "C" fn(*mut WIDGET) -> ::core::ffi::c_int,
        ) as Option<unsafe extern "C" fn(*mut _widget) -> ::core::ffi::c_int>;
        buttons[i as usize].width = Some(
            Widget_WidthFullScreen
                as unsafe extern "C" fn(*mut WIDGET) -> ::core::ffi::c_int,
        ) as Option<unsafe extern "C" fn(*mut _widget) -> ::core::ffi::c_int>;
        buttons[i as usize].name = buffer[i as usize];
        buttons[i as usize].tooltip[0usize] = b"\0".as_ptr()
            as *const ::core::ffi::c_char;
        buttons[i as usize].tooltip[1usize] = b"\0".as_ptr()
            as *const ::core::ffi::c_char;
        buttons[i as usize].tooltip[2usize] = b"\0".as_ptr()
            as *const ::core::ffi::c_char;
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < 10 as ::core::ffi::c_int {
        let mut j_1: ::core::ffi::c_int = 0;
        let mut tipcount_1: ::core::ffi::c_int = 0;
        if index >= count {
            log_add(
                log_Level::log_Fatal,
                b"PANIC: String table cut short while reading buttons\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            exit(1 as ::core::ffi::c_int);
        }
        let c2rust_fresh5 = index;
        index += 1;
        str = GetStringAddress(SetAbsStringTableIndex(SetupTab, c2rust_fresh5 as COUNT))
            as *const ::core::ffi::c_char;
        tipcount_1 = SplitString(
            str,
            '\n' as ::core::ffi::c_char,
            100 as ::core::ffi::c_int,
            &raw mut buffer as *mut *const ::core::ffi::c_char,
            bank,
        );
        if tipcount_1 > 3 as ::core::ffi::c_int {
            tipcount_1 = 3 as ::core::ffi::c_int;
        }
        j_1 = 0 as ::core::ffi::c_int;
        while j_1 < tipcount_1 {
            buttons[i as usize].tooltip[j_1 as usize] = buffer[j_1 as usize];
            j_1 += 1;
        }
        i += 1;
    }
    if index >= count {
        log_add(
            log_Level::log_Fatal,
            b"PANIC: String table cut short while reading labels\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        exit(1 as ::core::ffi::c_int);
    }
    let c2rust_fresh6 = index;
    index += 1;
    if SplitString(
        GetStringAddress(SetAbsStringTableIndex(SetupTab, c2rust_fresh6 as COUNT))
            as *const ::core::ffi::c_char,
        '\n' as ::core::ffi::c_char,
        100 as ::core::ffi::c_int,
        &raw mut buffer as *mut *const ::core::ffi::c_char,
        bank,
    ) != 4 as ::core::ffi::c_int
    {
        log_add(
            log_Level::log_Fatal,
            b"PANIC: Incorrect number of Label Options\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        exit(1 as ::core::ffi::c_int);
    }
    i = 0 as ::core::ffi::c_int;
    while i < 4 as ::core::ffi::c_int {
        labels[i as usize].tag = WIDGET_TYPE::WIDGET_TYPE_LABEL;
        labels[i as usize].parent = ::core::ptr::null_mut::<_widget>();
        labels[i as usize].handleEvent = Some(
            Widget_HandleEventIgnoreAll
                as unsafe extern "C" fn(
                    *mut WIDGET,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut _widget,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
            >;
        labels[i as usize].receiveFocus = Some(
            Widget_ReceiveFocusRefuseFocus
                as unsafe extern "C" fn(
                    *mut WIDGET,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut _widget,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
            >;
        labels[i as usize].draw = Some(
            Widget_DrawLabel
                as unsafe extern "C" fn(
                    *mut WIDGET,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C" fn(
                    *mut _widget,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> (),
            >;
        labels[i as usize].height = Some(
            Widget_HeightLabel as unsafe extern "C" fn(*mut WIDGET) -> ::core::ffi::c_int,
        ) as Option<unsafe extern "C" fn(*mut _widget) -> ::core::ffi::c_int>;
        labels[i as usize].width = Some(
            Widget_WidthFullScreen
                as unsafe extern "C" fn(*mut WIDGET) -> ::core::ffi::c_int,
        ) as Option<unsafe extern "C" fn(*mut _widget) -> ::core::ffi::c_int>;
        labels[i as usize].line_count = 0 as ::core::ffi::c_int;
        labels[i as usize].lines = ::core::ptr::null_mut::<*const ::core::ffi::c_char>();
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < 4 as ::core::ffi::c_int {
        let mut j_2: ::core::ffi::c_int = 0;
        let mut linecount: ::core::ffi::c_int = 0;
        if index >= count {
            log_add(
                log_Level::log_Fatal,
                b"PANIC: String table cut short while reading labels\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            exit(1 as ::core::ffi::c_int);
        }
        let c2rust_fresh7 = index;
        index += 1;
        str = GetStringAddress(SetAbsStringTableIndex(SetupTab, c2rust_fresh7 as COUNT))
            as *const ::core::ffi::c_char;
        linecount = SplitString(
            str,
            '\n' as ::core::ffi::c_char,
            100 as ::core::ffi::c_int,
            &raw mut buffer as *mut *const ::core::ffi::c_char,
            bank,
        );
        labels[i as usize].line_count = linecount;
        labels[i as usize].lines = HMalloc(
            (linecount as size_t)
                .wrapping_mul(::core::mem::size_of::<*const ::core::ffi::c_char>()),
        ) as *mut *const ::core::ffi::c_char;
        j_2 = 0 as ::core::ffi::c_int;
        while j_2 < linecount {
            *labels[i as usize].lines.offset(j_2 as isize) = buffer[j_2 as usize];
            j_2 += 1;
        }
        i += 1;
    }
    if index >= count {
        log_add(
            log_Level::log_Fatal,
            b"PANIC: String table cut short while reading text entries\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        exit(1 as ::core::ffi::c_int);
    }
    let c2rust_fresh8 = index;
    index += 1;
    if SplitString(
        GetStringAddress(SetAbsStringTableIndex(SetupTab, c2rust_fresh8 as COUNT))
            as *const ::core::ffi::c_char,
        '\n' as ::core::ffi::c_char,
        100 as ::core::ffi::c_int,
        &raw mut buffer as *mut *const ::core::ffi::c_char,
        bank,
    ) != 1 as ::core::ffi::c_int
    {
        log_add(
            log_Level::log_Fatal,
            b"PANIC: Incorrect number of Text Entries\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        exit(1 as ::core::ffi::c_int);
    }
    i = 0 as ::core::ffi::c_int;
    while i < 1 as ::core::ffi::c_int {
        textentries[i as usize].tag = WIDGET_TYPE::WIDGET_TYPE_TEXTENTRY;
        textentries[i as usize].parent = ::core::ptr::null_mut::<_widget>();
        textentries[i as usize].handleEvent = Some(
            Widget_HandleEventTextEntry
                as unsafe extern "C" fn(
                    *mut WIDGET,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut _widget,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
            >;
        textentries[i as usize].receiveFocus = Some(
            Widget_ReceiveFocusSimple
                as unsafe extern "C" fn(
                    *mut WIDGET,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut _widget,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
            >;
        textentries[i as usize].draw = Some(
            Widget_DrawTextEntry
                as unsafe extern "C" fn(
                    *mut WIDGET,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C" fn(
                    *mut _widget,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> (),
            >;
        textentries[i as usize].height = Some(
            Widget_HeightOneLine
                as unsafe extern "C" fn(*mut WIDGET) -> ::core::ffi::c_int,
        ) as Option<unsafe extern "C" fn(*mut _widget) -> ::core::ffi::c_int>;
        textentries[i as usize].width = Some(
            Widget_WidthFullScreen
                as unsafe extern "C" fn(*mut WIDGET) -> ::core::ffi::c_int,
        ) as Option<unsafe extern "C" fn(*mut _widget) -> ::core::ffi::c_int>;
        textentries[i as usize].handleEventSelect = Some(
            OnTextEntryEvent
                as unsafe extern "C" fn(*mut WIDGET_TEXTENTRY) -> ::core::ffi::c_int,
        ) as Option<unsafe extern "C" fn(*mut _widget_textentry) -> ::core::ffi::c_int>;
        textentries[i as usize].category = buffer[i as usize];
        textentries[i as usize].value[0usize] = 0 as ::core::ffi::c_char;
        textentries[i as usize].maxlen = 50 as ::core::ffi::c_int
            - 1 as ::core::ffi::c_int;
        textentries[i as usize].state = WIDGET_TEXTENTRY_STATE::WTE_NORMAL;
        textentries[i as usize].cursor_pos = 0 as ::core::ffi::c_int;
        i += 1;
    }
    if index >= count {
        log_add(
            log_Level::log_Fatal,
            b"PANIC: String table cut short while reading text entries\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        exit(1 as ::core::ffi::c_int);
    }
    let c2rust_fresh9 = index;
    index += 1;
    if SplitString(
        GetStringAddress(SetAbsStringTableIndex(SetupTab, c2rust_fresh9 as COUNT))
            as *const ::core::ffi::c_char,
        '\n' as ::core::ffi::c_char,
        100 as ::core::ffi::c_int,
        &raw mut buffer as *mut *const ::core::ffi::c_char,
        bank,
    ) != 1 as ::core::ffi::c_int
    {
        log_add(
            log_Level::log_Fatal,
            b"PANIC: Incorrect number of Text Entries\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        exit(1 as ::core::ffi::c_int);
    }
    i = 0 as ::core::ffi::c_int;
    while i < 1 as ::core::ffi::c_int {
        strncpy(
            &raw mut (*(&raw mut textentries as *mut WIDGET_TEXTENTRY)
                .offset(i as isize))
                .value as *mut ::core::ffi::c_char,
            buffer[i as usize],
            textentries[i as usize].maxlen as size_t,
        );
        textentries[i as usize].value[textentries[i as usize].maxlen as usize] = 0
            as ::core::ffi::c_char;
        i += 1;
    }
    textentries[0usize].onChange = Some(
        rename_template as unsafe extern "C" fn(*mut WIDGET_TEXTENTRY) -> (),
    ) as Option<unsafe extern "C" fn(*mut _widget_textentry) -> ()>;
    if index >= count {
        log_add(
            log_Level::log_Fatal,
            b"PANIC: String table cut short while reading control entries\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        exit(1 as ::core::ffi::c_int);
    }
    let c2rust_fresh10 = index;
    index += 1;
    if SplitString(
        GetStringAddress(SetAbsStringTableIndex(SetupTab, c2rust_fresh10 as COUNT))
            as *const ::core::ffi::c_char,
        '\n' as ::core::ffi::c_char,
        100 as ::core::ffi::c_int,
        &raw mut buffer as *mut *const ::core::ffi::c_char,
        bank,
    ) != 7 as ::core::ffi::c_int
    {
        log_add(
            log_Level::log_Fatal,
            b"PANIC: Incorrect number of Control Entries\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        exit(1 as ::core::ffi::c_int);
    }
    i = 0 as ::core::ffi::c_int;
    while i < 7 as ::core::ffi::c_int {
        controlentries[i as usize].tag = WIDGET_TYPE::WIDGET_TYPE_CONTROLENTRY;
        controlentries[i as usize].parent = ::core::ptr::null_mut::<_widget>();
        controlentries[i as usize].handleEvent = Some(
            Widget_HandleEventControlEntry
                as unsafe extern "C" fn(
                    *mut WIDGET,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut _widget,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
            >;
        controlentries[i as usize].receiveFocus = Some(
            Widget_ReceiveFocusControlEntry
                as unsafe extern "C" fn(
                    *mut WIDGET,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut _widget,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
            >;
        controlentries[i as usize].draw = Some(
            Widget_DrawControlEntry
                as unsafe extern "C" fn(
                    *mut WIDGET,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C" fn(
                    *mut _widget,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> (),
            >;
        controlentries[i as usize].height = Some(
            Widget_HeightOneLine
                as unsafe extern "C" fn(*mut WIDGET) -> ::core::ffi::c_int,
        ) as Option<unsafe extern "C" fn(*mut _widget) -> ::core::ffi::c_int>;
        controlentries[i as usize].width = Some(
            Widget_WidthFullScreen
                as unsafe extern "C" fn(*mut WIDGET) -> ::core::ffi::c_int,
        ) as Option<unsafe extern "C" fn(*mut _widget) -> ::core::ffi::c_int>;
        controlentries[i as usize].category = buffer[i as usize];
        controlentries[i as usize].highlighted = 0 as ::core::ffi::c_int;
        controlentries[i as usize].controlname[0usize][0usize] = 0
            as ::core::ffi::c_char;
        controlentries[i as usize].controlname[1usize][0usize] = 0
            as ::core::ffi::c_char;
        controlentries[i as usize].controlindex = i;
        controlentries[i as usize].onChange = Some(
            rebind_control as unsafe extern "C" fn(*mut WIDGET_CONTROLENTRY) -> (),
        ) as Option<unsafe extern "C" fn(*mut _widget_controlentry) -> ()>;
        controlentries[i as usize].onDelete = Some(
            clear_control as unsafe extern "C" fn(*mut WIDGET_CONTROLENTRY) -> (),
        ) as Option<unsafe extern "C" fn(*mut _widget_controlentry) -> ()>;
        i += 1;
    }
    if index < count {
        log_add(
            log_Level::log_Warning,
            b"WARNING: Setup strings had %d garbage entries at the end.\0".as_ptr()
                as *const ::core::ffi::c_char,
            count - index,
        );
    }
}
unsafe extern "C" fn clean_up_widgets() {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < 24 as ::core::ffi::c_int {
        if !choices[i as usize].options.is_null() {
            HFree(choices[i as usize].options as *mut ::core::ffi::c_void);
        }
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < 4 as ::core::ffi::c_int {
        if !labels[i as usize].lines.is_null() {
            HFree(labels[i as usize].lines as *mut ::core::ffi::c_void);
        }
        i += 1;
    }
    if !SetupTab.is_null() {
        DestroyStringTable(ReleaseStringTable(SetupTab));
        SetupTab = ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>();
    }
    if !bank.is_null() {
        StringBank_Free(bank);
        bank = ::core::ptr::null_mut::<stringbank>();
    }
    if !setup_frame.is_null() {
        DestroyDrawable(ReleaseDrawable(setup_frame));
        setup_frame = ::core::ptr::null_mut::<FRAME_DESC>();
    }
}
#[no_mangle]
pub unsafe extern "C" fn SetupMenu() {
    let mut s: SETUP_MENU_STATE = SETUP_MENU_STATE {
        InputFunc: None,
        initialized: BOOLEAN::FALSE,
        anim_frame_count: 0,
        NextTime: 0,
    };
    s.InputFunc = Some(
        DoSetupMenu as unsafe extern "C" fn(*mut SETUP_MENU_STATE) -> BOOLEAN,
    ) as Option<unsafe extern "C" fn(*mut setup_menu_state) -> BOOLEAN>;
    s.initialized = BOOLEAN::FALSE;
    SetMenuSounds(
        (((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as MENU_SOUND_FLAGS
            as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as MENU_SOUND_FLAGS
                as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as MENU_SOUND_FLAGS
                as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as MENU_SOUND_FLAGS
                as ::core::ffi::c_int) as MENU_SOUND_FLAGS,
        ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as MENU_SOUND_FLAGS,
    );
    SetupTab = CaptureStringTable(
        LoadStringTableInstance(b"text.setupmenu\0".as_ptr() as RESOURCE),
    );
    if !SetupTab.is_null() {
        init_widgets();
    } else {
        log_add(
            log_Level::log_Fatal,
            b"PANIC: Could not find strings for the setup menu!\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        exit(1 as ::core::ffi::c_int);
    }
    done = BOOLEAN::FALSE;
    DoInput(&raw mut s as *mut ::core::ffi::c_void, BOOLEAN::TRUE);
    GlobData.Game_state.CurrentActivity = (GlobData.Game_state.CurrentActivity
        as ::core::ffi::c_int & !(C2Rust_Unnamed_2::CHECK_ABORT.0 as ::core::ffi::c_int))
        as ACTIVITY;
    PropagateResults();
    if !SetupTab.is_null() {
        clean_up_widgets();
    }
}
#[no_mangle]
pub unsafe extern "C" fn GetGlobalOptions(mut opts: *mut GLOBALOPTS) {
    let mut whichBound: bool = false;
    if GfxFlags & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int != 0 {
        (*opts).scaler = OPT_SCALETYPE::OPTVAL_BILINEAR_SCALE;
    } else if GfxFlags & (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int != 0 {
        (*opts).scaler = OPT_SCALETYPE::OPTVAL_BIADAPT_SCALE;
    } else if GfxFlags & (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int != 0 {
        (*opts).scaler = OPT_SCALETYPE::OPTVAL_BIADV_SCALE;
    } else if GfxFlags & (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int != 0 {
        (*opts).scaler = OPT_SCALETYPE::OPTVAL_TRISCAN_SCALE;
    } else if GfxFlags & (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int != 0 {
        (*opts).scaler = OPT_SCALETYPE::OPTVAL_HQXX_SCALE;
    } else {
        (*opts).scaler = OPT_SCALETYPE::OPTVAL_NO_SCALE;
    }
    (*opts).fullscreen = OPT_ENABLABLE(
        (if GfxFlags & (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int != 0 {
            OPT_ENABLABLE::OPTVAL_ENABLED.0 as ::core::ffi::c_int
        } else {
            OPT_ENABLABLE::OPTVAL_DISABLED.0 as ::core::ffi::c_int
        }) as ::core::ffi::c_uint,
    );
    (*opts).subtitles = OPT_ENABLABLE(
        (if optSubtitles.0 != 0 {
            OPT_ENABLABLE::OPTVAL_ENABLED.0 as ::core::ffi::c_int
        } else {
            OPT_ENABLABLE::OPTVAL_DISABLED.0 as ::core::ffi::c_int
        }) as ::core::ffi::c_uint,
    );
    (*opts).scanlines = OPT_ENABLABLE(
        (if GfxFlags & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int != 0 {
            OPT_ENABLABLE::OPTVAL_ENABLED.0 as ::core::ffi::c_int
        } else {
            OPT_ENABLABLE::OPTVAL_DISABLED.0 as ::core::ffi::c_int
        }) as ::core::ffi::c_uint,
    );
    (*opts).menu = OPT_CONSOLETYPE(
        (if optWhichMenu == 0x1 as ::core::ffi::c_int {
            OPT_CONSOLETYPE::OPTVAL_3DO.0 as ::core::ffi::c_int
        } else {
            OPT_CONSOLETYPE::OPTVAL_PC.0 as ::core::ffi::c_int
        }) as ::core::ffi::c_uint,
    );
    (*opts).text = OPT_CONSOLETYPE(
        (if optWhichFonts == 0x1 as ::core::ffi::c_int {
            OPT_CONSOLETYPE::OPTVAL_3DO.0 as ::core::ffi::c_int
        } else {
            OPT_CONSOLETYPE::OPTVAL_PC.0 as ::core::ffi::c_int
        }) as ::core::ffi::c_uint,
    );
    (*opts).cscan = OPT_CONSOLETYPE(
        (if optWhichCoarseScan == 0x1 as ::core::ffi::c_int {
            OPT_CONSOLETYPE::OPTVAL_3DO.0 as ::core::ffi::c_int
        } else {
            OPT_CONSOLETYPE::OPTVAL_PC.0 as ::core::ffi::c_int
        }) as ::core::ffi::c_uint,
    );
    (*opts).scroll = OPT_CONSOLETYPE(
        (if optSmoothScroll == 0x1 as ::core::ffi::c_int {
            OPT_CONSOLETYPE::OPTVAL_3DO.0 as ::core::ffi::c_int
        } else {
            OPT_CONSOLETYPE::OPTVAL_PC.0 as ::core::ffi::c_int
        }) as ::core::ffi::c_uint,
    );
    (*opts).intro = OPT_CONSOLETYPE(
        (if optWhichIntro == 0x1 as ::core::ffi::c_int {
            OPT_CONSOLETYPE::OPTVAL_3DO.0 as ::core::ffi::c_int
        } else {
            OPT_CONSOLETYPE::OPTVAL_PC.0 as ::core::ffi::c_int
        }) as ::core::ffi::c_uint,
    );
    (*opts).shield = OPT_CONSOLETYPE(
        (if optWhichShield == 0x1 as ::core::ffi::c_int {
            OPT_CONSOLETYPE::OPTVAL_3DO.0 as ::core::ffi::c_int
        } else {
            OPT_CONSOLETYPE::OPTVAL_PC.0 as ::core::ffi::c_int
        }) as ::core::ffi::c_uint,
    );
    (*opts).fps = OPT_ENABLABLE(
        (if GfxFlags & (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int != 0 {
            OPT_ENABLABLE::OPTVAL_ENABLED.0 as ::core::ffi::c_int
        } else {
            OPT_ENABLABLE::OPTVAL_DISABLED.0 as ::core::ffi::c_int
        }) as ::core::ffi::c_uint,
    );
    (*opts).meleezoom = OPT_CONSOLETYPE(
        (if optMeleeScale == C2Rust_Unnamed_4::TFB_SCALE_STEP.0 as ::core::ffi::c_int {
            OPT_CONSOLETYPE::OPTVAL_PC.0 as ::core::ffi::c_int
        } else {
            OPT_CONSOLETYPE::OPTVAL_3DO.0 as ::core::ffi::c_int
        }) as ::core::ffi::c_uint,
    );
    (*opts).stereo = OPT_ENABLABLE(
        (if optStereoSFX.0 != 0 {
            OPT_ENABLABLE::OPTVAL_ENABLED.0 as ::core::ffi::c_int
        } else {
            OPT_ENABLABLE::OPTVAL_DISABLED.0 as ::core::ffi::c_int
        }) as ::core::ffi::c_uint,
    );
    (*opts).music3do = OPT_ENABLABLE(
        (if opt3doMusic.0 != 0 {
            OPT_ENABLABLE::OPTVAL_ENABLED.0 as ::core::ffi::c_int
        } else {
            OPT_ENABLABLE::OPTVAL_DISABLED.0 as ::core::ffi::c_int
        }) as ::core::ffi::c_uint,
    );
    (*opts).musicremix = OPT_ENABLABLE(
        (if optRemixMusic.0 != 0 {
            OPT_ENABLABLE::OPTVAL_ENABLED.0 as ::core::ffi::c_int
        } else {
            OPT_ENABLABLE::OPTVAL_DISABLED.0 as ::core::ffi::c_int
        }) as ::core::ffi::c_uint,
    );
    (*opts).speech = OPT_ENABLABLE(
        (if optSpeech.0 != 0 {
            OPT_ENABLABLE::OPTVAL_ENABLED.0 as ::core::ffi::c_int
        } else {
            OPT_ENABLABLE::OPTVAL_DISABLED.0 as ::core::ffi::c_int
        }) as ::core::ffi::c_uint,
    );
    (*opts).keepaspect = OPT_ENABLABLE(
        (if optKeepAspectRatio.0 != 0 {
            OPT_ENABLABLE::OPTVAL_ENABLED.0 as ::core::ffi::c_int
        } else {
            OPT_ENABLABLE::OPTVAL_DISABLED.0 as ::core::ffi::c_int
        }) as ::core::ffi::c_uint,
    );
    match snddriver {
        2 => {
            (*opts).adriver = OPT_ADRIVERTYPE::OPTVAL_OPENAL;
        }
        0 => {
            (*opts).adriver = OPT_ADRIVERTYPE::OPTVAL_MIXSDL;
        }
        _ => {
            (*opts).adriver = OPT_ADRIVERTYPE::OPTVAL_SILENCE;
        }
    }
    if soundflags & (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int != 0 {
        (*opts).aquality = OPT_AQUALITYTYPE::OPTVAL_HIGH;
    } else if soundflags & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int != 0 {
        (*opts).aquality = OPT_AQUALITYTYPE::OPTVAL_LOW;
    } else {
        (*opts).aquality = OPT_AQUALITYTYPE::OPTVAL_MEDIUM;
    }
    (*opts).driver = OPT_DRIVERTYPE::OPTVAL_PURE_IF_POSSIBLE;
    match ScreenWidthActual {
        320 => {
            if GraphicsDriver
                == C2Rust_Unnamed_3::TFB_GFXDRIVER_SDL_PURE.0 as ::core::ffi::c_int
            {
                (*opts).res = OPT_RESTYPE::OPTVAL_320_240;
            } else if ScreenHeightActual != 240 as ::core::ffi::c_int {
                (*opts).res = OPT_RESTYPE::OPTVAL_CUSTOM;
            } else {
                (*opts).res = OPT_RESTYPE::OPTVAL_320_240;
                (*opts).driver = OPT_DRIVERTYPE::OPTVAL_ALWAYS_GL;
            }
        }
        640 => {
            if GraphicsDriver
                == C2Rust_Unnamed_3::TFB_GFXDRIVER_SDL_PURE.0 as ::core::ffi::c_int
            {
                (*opts).res = OPT_RESTYPE::OPTVAL_640_480;
            } else if ScreenHeightActual != 480 as ::core::ffi::c_int {
                (*opts).res = OPT_RESTYPE::OPTVAL_CUSTOM;
            } else {
                (*opts).res = OPT_RESTYPE::OPTVAL_640_480;
                (*opts).driver = OPT_DRIVERTYPE::OPTVAL_ALWAYS_GL;
            }
        }
        800 => {
            if ScreenHeightActual != 600 as ::core::ffi::c_int {
                (*opts).res = OPT_RESTYPE::OPTVAL_CUSTOM;
            } else {
                (*opts).res = OPT_RESTYPE::OPTVAL_800_600;
            }
        }
        1024 => {
            if ScreenHeightActual != 768 as ::core::ffi::c_int {
                (*opts).res = OPT_RESTYPE::OPTVAL_CUSTOM;
            } else {
                (*opts).res = OPT_RESTYPE::OPTVAL_1024_768;
            }
        }
        1280 => {
            if ScreenHeightActual != 960 as ::core::ffi::c_int {
                (*opts).res = OPT_RESTYPE::OPTVAL_CUSTOM;
            } else {
                (*opts).res = OPT_RESTYPE::OPTVAL_1280_960;
            }
        }
        _ => {
            (*opts).res = OPT_RESTYPE::OPTVAL_CUSTOM;
        }
    }
    if res_IsBoolean(b"config.alwaysgl\0".as_ptr() as *const ::core::ffi::c_char).0 != 0
    {
        if res_GetBoolean(b"config.alwaysgl\0".as_ptr() as *const ::core::ffi::c_char).0
            != 0
        {
            (*opts).driver = OPT_DRIVERTYPE::OPTVAL_ALWAYS_GL;
        } else {
            (*opts).driver = OPT_DRIVERTYPE::OPTVAL_PURE_IF_POSSIBLE;
        }
    }
    whichBound = optGamma < maxGamma;
    if optGamma <= minGamma {
        minGamma = optGamma - 0.03f32;
    } else if optGamma >= maxGamma {
        maxGamma = optGamma + 0.3f32;
    }
    updateGammaBounds(whichBound);
    (*opts).gamma = gammaToSlider(optGamma);
    (*opts).player1 = *(&raw mut PlayerControls as *mut CONTROL_TEMPLATE).offset(0isize);
    (*opts).player2 = *(&raw mut PlayerControls as *mut CONTROL_TEMPLATE).offset(1isize);
    (*opts).musicvol = ((musicVolumeScale * 100.0f32) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) / 5 as ::core::ffi::c_int * 5 as ::core::ffi::c_int;
    (*opts).sfxvol = ((sfxVolumeScale * 100.0f32) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) / 5 as ::core::ffi::c_int * 5 as ::core::ffi::c_int;
    (*opts).speechvol = ((speechVolumeScale * 100.0f32) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) / 5 as ::core::ffi::c_int * 5 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn SetGlobalOptions(mut opts: *mut GLOBALOPTS) {
    let mut NewGfxFlags: ::core::ffi::c_int = GfxFlags;
    let mut NewWidth: ::core::ffi::c_int = ScreenWidthActual;
    let mut NewHeight: ::core::ffi::c_int = ScreenHeightActual;
    let mut NewDriver: ::core::ffi::c_int = GraphicsDriver;
    NewGfxFlags
        &= !((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int);
    match (*opts).res {
        OPT_RESTYPE::OPTVAL_320_240 => {
            NewWidth = 320 as ::core::ffi::c_int;
            NewHeight = 240 as ::core::ffi::c_int;
            NewDriver = C2Rust_Unnamed_3::TFB_GFXDRIVER_SDL_PURE.0 as ::core::ffi::c_int;
        }
        OPT_RESTYPE::OPTVAL_640_480 => {
            NewWidth = 640 as ::core::ffi::c_int;
            NewHeight = 480 as ::core::ffi::c_int;
            NewDriver = C2Rust_Unnamed_3::TFB_GFXDRIVER_SDL_PURE.0 as ::core::ffi::c_int;
        }
        OPT_RESTYPE::OPTVAL_800_600 => {
            NewWidth = 800 as ::core::ffi::c_int;
            NewHeight = 600 as ::core::ffi::c_int;
            NewDriver = C2Rust_Unnamed_3::TFB_GFXDRIVER_SDL_OPENGL.0
                as ::core::ffi::c_int;
        }
        OPT_RESTYPE::OPTVAL_1024_768 => {
            NewWidth = 1024 as ::core::ffi::c_int;
            NewHeight = 768 as ::core::ffi::c_int;
            NewDriver = C2Rust_Unnamed_3::TFB_GFXDRIVER_SDL_OPENGL.0
                as ::core::ffi::c_int;
        }
        OPT_RESTYPE::OPTVAL_1280_960 => {
            NewWidth = 1280 as ::core::ffi::c_int;
            NewHeight = 960 as ::core::ffi::c_int;
            NewDriver = C2Rust_Unnamed_3::TFB_GFXDRIVER_SDL_OPENGL.0
                as ::core::ffi::c_int;
        }
        _ => {}
    }
    res_PutInteger(
        b"config.reswidth\0".as_ptr() as *const ::core::ffi::c_char,
        NewWidth,
    );
    res_PutInteger(
        b"config.resheight\0".as_ptr() as *const ::core::ffi::c_char,
        NewHeight,
    );
    res_PutBoolean(
        b"config.alwaysgl\0".as_ptr() as *const ::core::ffi::c_char,
        BOOLEAN(
            ((*opts).driver.0 == OPT_DRIVERTYPE::OPTVAL_ALWAYS_GL.0)
                as ::core::ffi::c_int as ::core::ffi::c_uint,
        ),
    );
    res_PutBoolean(
        b"config.usegl\0".as_ptr() as *const ::core::ffi::c_char,
        BOOLEAN(
            (NewDriver
                == C2Rust_Unnamed_3::TFB_GFXDRIVER_SDL_OPENGL.0 as ::core::ffi::c_int)
                as ::core::ffi::c_int as ::core::ffi::c_uint,
        ),
    );
    match (*opts).scaler {
        OPT_SCALETYPE::OPTVAL_BILINEAR_SCALE => {
            NewGfxFlags |= (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
            res_PutString(
                b"config.scaler\0".as_ptr() as *const ::core::ffi::c_char,
                b"bilinear\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        OPT_SCALETYPE::OPTVAL_BIADAPT_SCALE => {
            NewGfxFlags |= (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int;
            res_PutString(
                b"config.scaler\0".as_ptr() as *const ::core::ffi::c_char,
                b"biadapt\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        OPT_SCALETYPE::OPTVAL_BIADV_SCALE => {
            NewGfxFlags |= (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int;
            res_PutString(
                b"config.scaler\0".as_ptr() as *const ::core::ffi::c_char,
                b"biadv\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        OPT_SCALETYPE::OPTVAL_TRISCAN_SCALE => {
            NewGfxFlags |= (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int;
            res_PutString(
                b"config.scaler\0".as_ptr() as *const ::core::ffi::c_char,
                b"triscan\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        OPT_SCALETYPE::OPTVAL_HQXX_SCALE => {
            NewGfxFlags |= (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int;
            res_PutString(
                b"config.scaler\0".as_ptr() as *const ::core::ffi::c_char,
                b"hq\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        _ => {
            res_PutString(
                b"config.scaler\0".as_ptr() as *const ::core::ffi::c_char,
                b"no\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    }
    if (*opts).scanlines.0 != 0 {
        NewGfxFlags |= (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int;
    } else {
        NewGfxFlags &= !((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int);
    }
    if (*opts).fullscreen.0 != 0 {
        NewGfxFlags |= (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int;
    } else {
        NewGfxFlags &= !((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int);
    }
    res_PutBoolean(
        b"config.scanlines\0".as_ptr() as *const ::core::ffi::c_char,
        BOOLEAN((*opts).scanlines.0),
    );
    res_PutBoolean(
        b"config.fullscreen\0".as_ptr() as *const ::core::ffi::c_char,
        BOOLEAN((*opts).fullscreen.0),
    );
    if NewWidth != ScreenWidthActual || NewHeight != ScreenHeightActual
        || NewDriver != GraphicsDriver || NewGfxFlags != GfxFlags
    {
        FlushGraphics();
        UninitVideoPlayer();
        TFB_DrawScreen_ReinitVideo(NewDriver, NewGfxFlags, NewWidth, NewHeight);
        FlushGraphics();
        InitVideoPlayer(BOOLEAN::TRUE);
    }
    if optGamma != 1.0f32 || sliderToGamma((*opts).gamma) != 1.0f32 {
        optGamma = sliderToGamma((*opts).gamma);
        setGammaCorrection(optGamma);
    }
    optSubtitles = BOOLEAN(
        (if (*opts).subtitles.0 == OPT_ENABLABLE::OPTVAL_ENABLED.0 {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        }) as ::core::ffi::c_uint,
    );
    optWhichMenu = if (*opts).menu.0 == OPT_CONSOLETYPE::OPTVAL_3DO.0 {
        0x1 as ::core::ffi::c_int
    } else {
        0x2 as ::core::ffi::c_int
    };
    optWhichFonts = if (*opts).text.0 == OPT_CONSOLETYPE::OPTVAL_3DO.0 {
        0x1 as ::core::ffi::c_int
    } else {
        0x2 as ::core::ffi::c_int
    };
    optWhichCoarseScan = if (*opts).cscan.0 == OPT_CONSOLETYPE::OPTVAL_3DO.0 {
        0x1 as ::core::ffi::c_int
    } else {
        0x2 as ::core::ffi::c_int
    };
    optSmoothScroll = if (*opts).scroll.0 == OPT_CONSOLETYPE::OPTVAL_3DO.0 {
        0x1 as ::core::ffi::c_int
    } else {
        0x2 as ::core::ffi::c_int
    };
    optWhichShield = if (*opts).shield.0 == OPT_CONSOLETYPE::OPTVAL_3DO.0 {
        0x1 as ::core::ffi::c_int
    } else {
        0x2 as ::core::ffi::c_int
    };
    optMeleeScale = if (*opts).meleezoom.0 == OPT_CONSOLETYPE::OPTVAL_3DO.0 {
        C2Rust_Unnamed_4::TFB_SCALE_TRILINEAR.0 as ::core::ffi::c_int
    } else {
        C2Rust_Unnamed_4::TFB_SCALE_STEP.0 as ::core::ffi::c_int
    };
    opt3doMusic = BOOLEAN(
        ((*opts).music3do.0 == OPT_ENABLABLE::OPTVAL_ENABLED.0) as ::core::ffi::c_int
            as ::core::ffi::c_uint,
    );
    optRemixMusic = BOOLEAN(
        ((*opts).musicremix.0 == OPT_ENABLABLE::OPTVAL_ENABLED.0) as ::core::ffi::c_int
            as ::core::ffi::c_uint,
    );
    optSpeech = BOOLEAN(
        ((*opts).speech.0 == OPT_ENABLABLE::OPTVAL_ENABLED.0) as ::core::ffi::c_int
            as ::core::ffi::c_uint,
    );
    optWhichIntro = if (*opts).intro.0 == OPT_CONSOLETYPE::OPTVAL_3DO.0 {
        0x1 as ::core::ffi::c_int
    } else {
        0x2 as ::core::ffi::c_int
    };
    optStereoSFX = BOOLEAN(
        ((*opts).stereo.0 == OPT_ENABLABLE::OPTVAL_ENABLED.0) as ::core::ffi::c_int
            as ::core::ffi::c_uint,
    );
    optKeepAspectRatio = BOOLEAN(
        ((*opts).keepaspect.0 == OPT_ENABLABLE::OPTVAL_ENABLED.0) as ::core::ffi::c_int
            as ::core::ffi::c_uint,
    );
    *(&raw mut PlayerControls as *mut CONTROL_TEMPLATE).offset(0isize) = (*opts).player1;
    *(&raw mut PlayerControls as *mut CONTROL_TEMPLATE).offset(1isize) = (*opts).player2;
    res_PutBoolean(
        b"config.subtitles\0".as_ptr() as *const ::core::ffi::c_char,
        BOOLEAN(
            ((*opts).subtitles.0 == OPT_ENABLABLE::OPTVAL_ENABLED.0)
                as ::core::ffi::c_int as ::core::ffi::c_uint,
        ),
    );
    res_PutBoolean(
        b"config.textmenu\0".as_ptr() as *const ::core::ffi::c_char,
        BOOLEAN(
            ((*opts).menu.0 == OPT_CONSOLETYPE::OPTVAL_PC.0) as ::core::ffi::c_int
                as ::core::ffi::c_uint,
        ),
    );
    res_PutBoolean(
        b"config.textgradients\0".as_ptr() as *const ::core::ffi::c_char,
        BOOLEAN(
            ((*opts).text.0 == OPT_CONSOLETYPE::OPTVAL_PC.0) as ::core::ffi::c_int
                as ::core::ffi::c_uint,
        ),
    );
    res_PutBoolean(
        b"config.iconicscan\0".as_ptr() as *const ::core::ffi::c_char,
        BOOLEAN(
            ((*opts).cscan.0 == OPT_CONSOLETYPE::OPTVAL_3DO.0) as ::core::ffi::c_int
                as ::core::ffi::c_uint,
        ),
    );
    res_PutBoolean(
        b"config.smoothscroll\0".as_ptr() as *const ::core::ffi::c_char,
        BOOLEAN(
            ((*opts).scroll.0 == OPT_CONSOLETYPE::OPTVAL_3DO.0) as ::core::ffi::c_int
                as ::core::ffi::c_uint,
        ),
    );
    res_PutBoolean(
        b"config.3domusic\0".as_ptr() as *const ::core::ffi::c_char,
        BOOLEAN(
            ((*opts).music3do.0 == OPT_ENABLABLE::OPTVAL_ENABLED.0) as ::core::ffi::c_int
                as ::core::ffi::c_uint,
        ),
    );
    res_PutBoolean(
        b"config.remixmusic\0".as_ptr() as *const ::core::ffi::c_char,
        BOOLEAN(
            ((*opts).musicremix.0 == OPT_ENABLABLE::OPTVAL_ENABLED.0)
                as ::core::ffi::c_int as ::core::ffi::c_uint,
        ),
    );
    res_PutBoolean(
        b"config.speech\0".as_ptr() as *const ::core::ffi::c_char,
        BOOLEAN(
            ((*opts).speech.0 == OPT_ENABLABLE::OPTVAL_ENABLED.0) as ::core::ffi::c_int
                as ::core::ffi::c_uint,
        ),
    );
    res_PutBoolean(
        b"config.3domovies\0".as_ptr() as *const ::core::ffi::c_char,
        BOOLEAN(
            ((*opts).intro.0 == OPT_CONSOLETYPE::OPTVAL_3DO.0) as ::core::ffi::c_int
                as ::core::ffi::c_uint,
        ),
    );
    res_PutBoolean(
        b"config.showfps\0".as_ptr() as *const ::core::ffi::c_char,
        BOOLEAN(
            ((*opts).fps.0 == OPT_ENABLABLE::OPTVAL_ENABLED.0) as ::core::ffi::c_int
                as ::core::ffi::c_uint,
        ),
    );
    res_PutBoolean(
        b"config.smoothmelee\0".as_ptr() as *const ::core::ffi::c_char,
        BOOLEAN(
            ((*opts).meleezoom.0 == OPT_CONSOLETYPE::OPTVAL_3DO.0) as ::core::ffi::c_int
                as ::core::ffi::c_uint,
        ),
    );
    res_PutBoolean(
        b"config.positionalsfx\0".as_ptr() as *const ::core::ffi::c_char,
        BOOLEAN(
            ((*opts).stereo.0 == OPT_ENABLABLE::OPTVAL_ENABLED.0) as ::core::ffi::c_int
                as ::core::ffi::c_uint,
        ),
    );
    res_PutBoolean(
        b"config.pulseshield\0".as_ptr() as *const ::core::ffi::c_char,
        BOOLEAN(
            ((*opts).shield.0 == OPT_CONSOLETYPE::OPTVAL_3DO.0) as ::core::ffi::c_int
                as ::core::ffi::c_uint,
        ),
    );
    res_PutBoolean(
        b"config.keepaspectratio\0".as_ptr() as *const ::core::ffi::c_char,
        BOOLEAN(
            ((*opts).keepaspect.0 == OPT_ENABLABLE::OPTVAL_ENABLED.0)
                as ::core::ffi::c_int as ::core::ffi::c_uint,
        ),
    );
    res_PutInteger(
        b"config.gamma\0".as_ptr() as *const ::core::ffi::c_char,
        ((optGamma * 1000 as ::core::ffi::c_int as ::core::ffi::c_float)
            as ::core::ffi::c_double + 0.5f64) as ::core::ffi::c_int,
    );
    res_PutInteger(
        b"config.player1control\0".as_ptr() as *const ::core::ffi::c_char,
        (*opts).player1.0 as ::core::ffi::c_int,
    );
    res_PutInteger(
        b"config.player2control\0".as_ptr() as *const ::core::ffi::c_char,
        (*opts).player2.0 as ::core::ffi::c_int,
    );
    match (*opts).adriver {
        OPT_ADRIVERTYPE::OPTVAL_SILENCE => {
            res_PutString(
                b"config.audiodriver\0".as_ptr() as *const ::core::ffi::c_char,
                b"none\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        OPT_ADRIVERTYPE::OPTVAL_MIXSDL => {
            res_PutString(
                b"config.audiodriver\0".as_ptr() as *const ::core::ffi::c_char,
                b"mixsdl\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        OPT_ADRIVERTYPE::OPTVAL_OPENAL => {
            res_PutString(
                b"config.audiodriver\0".as_ptr() as *const ::core::ffi::c_char,
                b"openal\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        _ => {}
    }
    match (*opts).aquality {
        OPT_AQUALITYTYPE::OPTVAL_LOW => {
            res_PutString(
                b"config.audioquality\0".as_ptr() as *const ::core::ffi::c_char,
                b"low\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        OPT_AQUALITYTYPE::OPTVAL_MEDIUM => {
            res_PutString(
                b"config.audioquality\0".as_ptr() as *const ::core::ffi::c_char,
                b"medium\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        OPT_AQUALITYTYPE::OPTVAL_HIGH => {
            res_PutString(
                b"config.audioquality\0".as_ptr() as *const ::core::ffi::c_char,
                b"high\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        _ => {}
    }
    res_PutInteger(
        b"config.musicvol\0".as_ptr() as *const ::core::ffi::c_char,
        (*opts).musicvol,
    );
    res_PutInteger(
        b"config.sfxvol\0".as_ptr() as *const ::core::ffi::c_char,
        (*opts).sfxvol,
    );
    res_PutInteger(
        b"config.speechvol\0".as_ptr() as *const ::core::ffi::c_char,
        (*opts).speechvol,
    );
    musicVolumeScale = (*opts).musicvol as ::core::ffi::c_float / 100.0f32;
    sfxVolumeScale = (*opts).sfxvol as ::core::ffi::c_float / 100.0f32;
    speechVolumeScale = (*opts).speechvol as ::core::ffi::c_float / 100.0f32;
    SetMusicVolume(musicVolume as COUNT);
    SetSpeechVolume(speechVolumeScale);
    res_PutString(
        b"keys.1.name\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*(&raw mut input_templates as *mut INPUT_TEMPLATE).offset(0isize)).name
            as *mut ::core::ffi::c_char,
    );
    res_PutString(
        b"keys.2.name\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*(&raw mut input_templates as *mut INPUT_TEMPLATE).offset(1isize)).name
            as *mut ::core::ffi::c_char,
    );
    res_PutString(
        b"keys.3.name\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*(&raw mut input_templates as *mut INPUT_TEMPLATE).offset(2isize)).name
            as *mut ::core::ffi::c_char,
    );
    res_PutString(
        b"keys.4.name\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*(&raw mut input_templates as *mut INPUT_TEMPLATE).offset(3isize)).name
            as *mut ::core::ffi::c_char,
    );
    res_PutString(
        b"keys.5.name\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*(&raw mut input_templates as *mut INPUT_TEMPLATE).offset(4isize)).name
            as *mut ::core::ffi::c_char,
    );
    res_PutString(
        b"keys.6.name\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*(&raw mut input_templates as *mut INPUT_TEMPLATE).offset(5isize)).name
            as *mut ::core::ffi::c_char,
    );
    SaveResourceIndex(
        configDir,
        b"uqm.cfg\0".as_ptr() as *const ::core::ffi::c_char,
        b"config.\0".as_ptr() as *const ::core::ffi::c_char,
        BOOLEAN::TRUE,
    );
    SaveKeyConfiguration(
        configDir,
        b"flight.cfg\0".as_ptr() as *const ::core::ffi::c_char,
    );
}
#[export_name = "uqm_c2rust_init_ba407804bbae8b28"]
unsafe extern "C" fn c2rust_run_static_initializers() {
    editkeys_widgets = [
        (&raw mut choices as *mut WIDGET_CHOICE).offset(20isize) as *mut WIDGET,
        (&raw mut labels as *mut WIDGET_LABEL).offset(2isize) as *mut WIDGET,
        (&raw mut textentries as *mut WIDGET_TEXTENTRY).offset(0isize) as *mut WIDGET,
        (&raw mut controlentries as *mut WIDGET_CONTROLENTRY).offset(0isize)
            as *mut WIDGET,
        (&raw mut controlentries as *mut WIDGET_CONTROLENTRY).offset(1isize)
            as *mut WIDGET,
        (&raw mut controlentries as *mut WIDGET_CONTROLENTRY).offset(2isize)
            as *mut WIDGET,
        (&raw mut controlentries as *mut WIDGET_CONTROLENTRY).offset(3isize)
            as *mut WIDGET,
        (&raw mut controlentries as *mut WIDGET_CONTROLENTRY).offset(4isize)
            as *mut WIDGET,
        (&raw mut controlentries as *mut WIDGET_CONTROLENTRY).offset(5isize)
            as *mut WIDGET,
        (&raw mut controlentries as *mut WIDGET_CONTROLENTRY).offset(6isize)
            as *mut WIDGET,
        (&raw mut buttons as *mut WIDGET_BUTTON).offset(9isize) as *mut WIDGET,
        ::core::ptr::null_mut::<WIDGET>(),
    ];
    advanced_widgets = [
        (&raw mut choices as *mut WIDGET_CHOICE).offset(12isize) as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE).offset(15isize) as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE).offset(16isize) as *mut WIDGET,
        (&raw mut buttons as *mut WIDGET_BUTTON).offset(1isize) as *mut WIDGET,
        ::core::ptr::null_mut::<WIDGET>(),
    ];
    keyconfig_widgets = [
        (&raw mut choices as *mut WIDGET_CHOICE).offset(18isize) as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE).offset(19isize) as *mut WIDGET,
        (&raw mut labels as *mut WIDGET_LABEL).offset(1isize) as *mut WIDGET,
        (&raw mut buttons as *mut WIDGET_BUTTON).offset(8isize) as *mut WIDGET,
        (&raw mut buttons as *mut WIDGET_BUTTON).offset(1isize) as *mut WIDGET,
        ::core::ptr::null_mut::<WIDGET>(),
    ];
    incomplete_widgets = [
        (&raw mut labels as *mut WIDGET_LABEL).offset(0isize) as *mut WIDGET,
        (&raw mut buttons as *mut WIDGET_BUTTON).offset(1isize) as *mut WIDGET,
        ::core::ptr::null_mut::<WIDGET>(),
    ];
    engine_widgets = [
        (&raw mut choices as *mut WIDGET_CHOICE).offset(4isize) as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE).offset(5isize) as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE).offset(6isize) as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE).offset(7isize) as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE).offset(8isize) as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE).offset(13isize) as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE).offset(11isize) as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE).offset(17isize) as *mut WIDGET,
        (&raw mut buttons as *mut WIDGET_BUTTON).offset(1isize) as *mut WIDGET,
        ::core::ptr::null_mut::<WIDGET>(),
    ];
    audio_widgets = [
        (&raw mut sliders as *mut WIDGET_SLIDER).offset(0isize) as *mut WIDGET,
        (&raw mut sliders as *mut WIDGET_SLIDER).offset(1isize) as *mut WIDGET,
        (&raw mut sliders as *mut WIDGET_SLIDER).offset(2isize) as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE).offset(14isize) as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE).offset(9isize) as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE).offset(21isize) as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE).offset(22isize) as *mut WIDGET,
        (&raw mut buttons as *mut WIDGET_BUTTON).offset(1isize) as *mut WIDGET,
        ::core::ptr::null_mut::<WIDGET>(),
    ];
    graphics_widgets = [
        (&raw mut choices as *mut WIDGET_CHOICE).offset(0isize) as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE).offset(23isize) as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE).offset(10isize) as *mut WIDGET,
        (&raw mut sliders as *mut WIDGET_SLIDER).offset(3isize) as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE).offset(2isize) as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE).offset(3isize) as *mut WIDGET,
        (&raw mut buttons as *mut WIDGET_BUTTON).offset(1isize) as *mut WIDGET,
        ::core::ptr::null_mut::<WIDGET>(),
    ];
    main_widgets = [
        (&raw mut buttons as *mut WIDGET_BUTTON).offset(2isize) as *mut WIDGET,
        (&raw mut buttons as *mut WIDGET_BUTTON).offset(3isize) as *mut WIDGET,
        (&raw mut buttons as *mut WIDGET_BUTTON).offset(4isize) as *mut WIDGET,
        (&raw mut buttons as *mut WIDGET_BUTTON).offset(5isize) as *mut WIDGET,
        (&raw mut buttons as *mut WIDGET_BUTTON).offset(6isize) as *mut WIDGET,
        (&raw mut buttons as *mut WIDGET_BUTTON).offset(7isize) as *mut WIDGET,
        (&raw mut buttons as *mut WIDGET_BUTTON).offset(0isize) as *mut WIDGET,
        ::core::ptr::null_mut::<WIDGET>(),
    ];
}
// Explicit engine initialization replaces platform constructor tables.
