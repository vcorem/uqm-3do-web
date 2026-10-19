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

// Rust translation of src/uqm/confirm.c.
extern "C" {
    pub type string_table_entry;
    pub type context_desc;
    pub type frame_desc;
    pub type font_desc;
    pub type drawable_desc;
    fn SetAbsStringTableIndex(String: STRING, StringTableIndex: COUNT) -> STRING;
    fn GetStringAddress(String: STRING) -> STRINGPTR;
    fn SetContext(Context: CONTEXT) -> CONTEXT;
    fn SetContextForeGroundColor(Color: Color) -> Color;
    fn SetContextClipRect(pRect: *mut RECT) -> BOOLEAN;
    fn GetContextClipRect(pRect: *mut RECT) -> BOOLEAN;
    fn DrawStamp(pStamp: *mut STAMP);
    fn font_DrawText(pText: *mut TEXT);
    fn BatchGraphics();
    fn UnbatchGraphics();
    fn FlushGraphics();
    fn DestroyDrawable(Drawable: DRAWABLE) -> BOOLEAN;
    fn SetContextFontEffect(EffectFrame: FRAME) -> FRAME;
    fn SetContextFont(Font: FONT) -> FONT;
    fn ReleaseDrawable(Frame: FRAME) -> DRAWABLE;
    fn SetSystemRect(pRect: *const RECT);
    fn ClearSystemRect();
    static mut PulsedInputState: CONTROLLER_INPUT_STATE;
    fn UpdateInputState();
    fn FlushInput();
    fn SetInputCallback(_: Option<InputFrameCallback>) -> Option<InputFrameCallback>;
    fn DoInput(pInputState: *mut ::core::ffi::c_void, resetInput: BOOLEAN);
    static mut GamePaused: BOOLEAN;
    static mut ExitRequested: BOOLEAN;
    fn SleepThread(timePeriod: TimePeriod);
    fn PauseFlash();
    fn ContinueFlash();
    static mut GlobData: GLOBDATA;
    static mut ScreenContext: CONTEXT;
    static mut StarConFont: FONT;
    static mut GameStrings: STRING;
    fn SetMenuSounds(sound_0: MENU_SOUND_FLAGS, sound_1: MENU_SOUND_FLAGS);
    fn GetMenuSounds(sound_0: *mut MENU_SOUND_FLAGS, sound_1: *mut MENU_SOUND_FLAGS);
    fn PlayMenuSound(S: MENU_SOUND_EFFECT);
    static mut ScreenWidth: ::core::ffi::c_int;
    static mut ScreenHeight: ::core::ffi::c_int;
    fn SaveContextFrame(saveRect: *const RECT) -> STAMP;
    fn DrawShadowedBox(r: *mut RECT, bg: Color, dark: Color, medium: Color);
    fn DrawLabelAsWindow(label: *mut WIDGET_LABEL, windowRect: *mut RECT);
    fn Widget_SetWindowColors(bg: Color, dark: Color, medium: Color);
    fn Widget_SetFont(newFont: FONT) -> FONT;
    fn Widget_ReceiveFocusRefuseFocus(
        _self: *mut WIDGET,
        event: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn Widget_HandleEventIgnoreAll(
        _self: *mut WIDGET,
        event: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn Widget_HeightLabel(_self: *mut WIDGET) -> ::core::ffi::c_int;
    fn Widget_WidthFullScreen(_self: *mut WIDGET) -> ::core::ffi::c_int;
    fn Widget_DrawLabel(
        _self: *mut WIDGET,
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
    );
    fn PauseTrack();
    fn ResumeTrack();
    fn PlayingTrack() -> COUNT;
    fn log_add(_: log_Level, fmt: *const ::core::ffi::c_char, ...);
    fn StringBank_Create() -> *mut stringbank;
    fn StringBank_Free(bank: *mut stringbank);
    fn SplitString(
        s: *const ::core::ffi::c_char,
        splitchar: ::core::ffi::c_char,
        n: ::core::ffi::c_int,
        result: *mut *const ::core::ffi::c_char,
        bank: *mut stringbank,
    ) -> ::core::ffi::c_int;
    fn exit(_: ::core::ffi::c_int) -> !;
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
pub type STRING_TABLE_ENTRY_DESC = string_table_entry;
pub type STRING = *mut STRING_TABLE_ENTRY_DESC;
pub type STRINGPTR = *mut ::core::ffi::c_char;
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
pub type FONT_DESC = font_desc;
pub type DRAWABLE_DESC = drawable_desc;
pub type CONTEXT = *mut CONTEXT_DESC;
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
pub type TimePeriod = DWORD;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed {
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _controller_input_state {
    pub key: [[::core::ffi::c_int; 7]; 6],
    pub menu: [::core::ffi::c_int; 24],
}
pub type CONTROLLER_INPUT_STATE = _controller_input_state;
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct MENU_SOUND_EFFECT(pub ::core::ffi::c_uint);
impl MENU_SOUND_EFFECT {
    pub const MENU_SOUND_MOVE: Self = Self(0);
    pub const MENU_SOUND_SUCCESS: Self = Self(1);
    pub const MENU_SOUND_FAILURE: Self = Self(2);
    pub const MENU_SOUND_INVOKED: Self = Self(3);
}
pub type stringbank = _stringbank_chunk;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _stringbank_chunk {
    pub data: [::core::ffi::c_char; 1016],
    pub len: ::core::ffi::c_int,
    pub next: *mut _stringbank_chunk,
}
pub type MENU_SOUND_FLAGS = UWORD;
pub type InputFrameCallback = unsafe extern "C" fn() -> ();
pub type POPUP_STATE = popup_state;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct popup_state {
    pub InputFunc: Option<unsafe extern "C" fn(*mut popup_state) -> BOOLEAN>,
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
pub type WIDGET = _widget;
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_0 {
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
pub struct C2Rust_Unnamed_1(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_1 {
    pub const STAR_STRING_BASE: Self = Self(0);
    pub const DEVICE_STRING_BASE: Self = Self(133);
    pub const CARGO_STRING_BASE: Self = Self(162);
    pub const ELEMENTS_STRING_BASE: Self = Self(172);
    pub const SCAN_STRING_BASE: Self = Self(305);
    pub const STAR_NUMBER_BASE: Self = Self(361);
    pub const PLANET_NUMBER_BASE: Self = Self(375);
    pub const MONTHS_STRING_BASE: Self = Self(408);
    pub const FEEDBACK_STRING_BASE: Self = Self(420);
    pub const STARBASE_STRING_BASE: Self = Self(422);
    pub const ENCOUNTER_STRING_BASE: Self = Self(427);
    pub const NAVIGATION_STRING_BASE: Self = Self(435);
    pub const NAMING_STRING_BASE: Self = Self(441);
    pub const MELEE_STRING_BASE: Self = Self(445);
    pub const SAVEGAME_STRING_BASE: Self = Self(454);
    pub const OPTION_STRING_BASE: Self = Self(459);
    pub const QUITMENU_STRING_BASE: Self = Self(464);
    pub const STATUS_STRING_BASE: Self = Self(467);
    pub const FLAGSHIP_STRING_BASE: Self = Self(473);
    pub const ORBITSCAN_STRING_BASE: Self = Self(486);
    pub const MAINMENU_STRING_BASE: Self = Self(505);
    pub const NETMELEE_STRING_BASE: Self = Self(560);
    pub const GAMESTR_COUNT: Self = Self(579);
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
unsafe extern "C" fn DrawConfirmationWindow(mut answer: BOOLEAN) {
    let mut oldfg: Color = SetContextForeGroundColor(
        colorFromRgb15(0x6 as BYTE, 0x6 as BYTE, 0x6 as BYTE),
    );
    let mut oldfont: FONT = SetContextFont(StarConFont);
    let mut oldFontEffect: FRAME = SetContextFontEffect(
        ::core::ptr::null_mut::<FRAME_DESC>(),
    );
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut t: TEXT = TEXT {
        baseline: POINT { x: 0, y: 0 },
        pStr: ::core::ptr::null::<UNICODE>(),
        align: TEXT_ALIGN::ALIGN_LEFT,
        CharCount: 0,
    };
    BatchGraphics();
    r.corner.x = (ScreenWidth - 80 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
        as COORD;
    r.corner.y = (ScreenHeight - 22 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
        as COORD;
    r.extent.width = 80 as COORD;
    r.extent.height = 22 as COORD;
    DrawShadowedBox(
        &raw mut r,
        colorFromRgb15(0x14 as BYTE, 0x14 as BYTE, 0x14 as BYTE),
        colorFromRgb15(0x8 as BYTE, 0x8 as BYTE, 0x8 as BYTE),
        colorFromRgb15(0x10 as BYTE, 0x10 as BYTE, 0x10 as BYTE),
    );
    t.baseline.x = (r.corner.x as ::core::ffi::c_int
        + (r.extent.width as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)) as COORD;
    t.baseline.y = (r.corner.y as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as COORD;
    t.pStr = GetStringAddress(
        SetAbsStringTableIndex(
            GameStrings,
            C2Rust_Unnamed_1::QUITMENU_STRING_BASE.0 as ::core::ffi::c_int as COUNT,
        ),
    );
    t.align = TEXT_ALIGN::ALIGN_CENTER;
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    font_DrawText(&raw mut t);
    t.baseline.y = (t.baseline.y as ::core::ffi::c_int + 10 as ::core::ffi::c_int)
        as COORD;
    t.baseline.x = (r.corner.x as ::core::ffi::c_int
        + (r.extent.width as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)) as COORD;
    t.pStr = GetStringAddress(
        SetAbsStringTableIndex(
            GameStrings,
            (C2Rust_Unnamed_1::QUITMENU_STRING_BASE.0 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int) as COUNT,
        ),
    );
    SetContextForeGroundColor(
        if answer.0 != 0 {
            colorFromRgb15(0x1f as BYTE, 0x1f as BYTE, 0 as BYTE)
        } else {
            colorFromRgb15(0x6 as BYTE, 0x6 as BYTE, 0x6 as BYTE)
        },
    );
    font_DrawText(&raw mut t);
    t.baseline.x = (t.baseline.x as ::core::ffi::c_int
        + (r.extent.width as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)) as COORD;
    t.pStr = GetStringAddress(
        SetAbsStringTableIndex(
            GameStrings,
            (C2Rust_Unnamed_1::QUITMENU_STRING_BASE.0 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int) as COUNT,
        ),
    );
    SetContextForeGroundColor(
        if answer.0 != 0 {
            colorFromRgb15(0x6 as BYTE, 0x6 as BYTE, 0x6 as BYTE)
        } else {
            colorFromRgb15(0x1f as BYTE, 0x1f as BYTE, 0 as BYTE)
        },
    );
    font_DrawText(&raw mut t);
    UnbatchGraphics();
    SetContextFontEffect(oldFontEffect);
    SetContextFont(oldfont);
    SetContextForeGroundColor(oldfg);
}
#[no_mangle]
pub unsafe extern "C" fn DoConfirmExit() -> BOOLEAN {
    let mut result: BOOLEAN = BOOLEAN::FALSE;
    if PlayingTrack() != 0 {
        PauseTrack();
    }
    PauseFlash();
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut s: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    let mut ctxRect: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut oldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    let mut oldRect: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut response: BOOLEAN = BOOLEAN::FALSE;
    let mut done: BOOLEAN = BOOLEAN::FALSE;
    oldContext = SetContext(ScreenContext);
    GetContextClipRect(&raw mut oldRect);
    SetContextClipRect(::core::ptr::null_mut::<RECT>());
    GetContextClipRect(&raw mut ctxRect);
    r.extent.width = (80 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as COORD;
    r.extent.height = (22 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as COORD;
    r.corner.x = (ctxRect.extent.width as ::core::ffi::c_int
        - r.extent.width as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as COORD;
    r.corner.y = (ctxRect.extent.height as ::core::ffi::c_int
        - r.extent.height as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as COORD;
    s = SaveContextFrame(&raw mut r);
    SetSystemRect(&raw mut r);
    DrawConfirmationWindow(response);
    FlushGraphics();
    FlushInput();
    done = BOOLEAN::FALSE;
    loop {
        ::core::ptr::write_volatile(&raw mut ExitRequested, BOOLEAN::FALSE);
        ::core::ptr::write_volatile(&raw mut GamePaused, BOOLEAN::FALSE);
        UpdateInputState();
        if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
            & C2Rust_Unnamed_0::CHECK_ABORT.0 as ::core::ffi::c_int != 0
        {
            done = BOOLEAN::TRUE;
            response = BOOLEAN::TRUE;
        } else if PulsedInputState
            .menu[C2Rust_Unnamed::KEY_MENU_SELECT.0 as ::core::ffi::c_int as usize] != 0
        {
            done = BOOLEAN::TRUE;
            PlayMenuSound(MENU_SOUND_EFFECT::MENU_SOUND_SUCCESS);
        } else if PulsedInputState
            .menu[C2Rust_Unnamed::KEY_MENU_CANCEL.0 as ::core::ffi::c_int as usize] != 0
        {
            done = BOOLEAN::TRUE;
            response = BOOLEAN::FALSE;
        } else if PulsedInputState
            .menu[C2Rust_Unnamed::KEY_MENU_LEFT.0 as ::core::ffi::c_int as usize] != 0
            || PulsedInputState
                .menu[C2Rust_Unnamed::KEY_MENU_RIGHT.0 as ::core::ffi::c_int as usize]
                != 0
        {
            response = BOOLEAN(
                (response.0 == 0) as ::core::ffi::c_int as ::core::ffi::c_uint,
            );
            DrawConfirmationWindow(response);
            PlayMenuSound(MENU_SOUND_EFFECT::MENU_SOUND_MOVE);
        }
        SleepThread(
            (840 as ::core::ffi::c_int / 30 as ::core::ffi::c_int) as TimePeriod,
        );
        if done.0 != 0 {
            break;
        }
    }
    DrawStamp(&raw mut s);
    DestroyDrawable(ReleaseDrawable(s.frame));
    ClearSystemRect();
    if response.0 != 0
        || GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
            & C2Rust_Unnamed_0::CHECK_ABORT.0 as ::core::ffi::c_int != 0
    {
        result = BOOLEAN::TRUE;
        GlobData.Game_state.CurrentActivity = (GlobData.Game_state.CurrentActivity
            as ::core::ffi::c_int
            | C2Rust_Unnamed_0::CHECK_ABORT.0 as ::core::ffi::c_int) as ACTIVITY;
    } else {
        result = BOOLEAN::FALSE;
    }
    ::core::ptr::write_volatile(&raw mut ExitRequested, BOOLEAN::FALSE);
    ::core::ptr::write_volatile(&raw mut GamePaused, BOOLEAN::FALSE);
    FlushInput();
    SetContextClipRect(&raw mut oldRect);
    SetContext(oldContext);
    ContinueFlash();
    if PlayingTrack() != 0 {
        ResumeTrack();
    }
    return result;
}
unsafe extern "C" fn DoPopup(mut self_0: *mut popup_state) -> BOOLEAN {
    SleepThread((840 as ::core::ffi::c_int / 20 as ::core::ffi::c_int) as TimePeriod);
    return BOOLEAN(
        !(PulsedInputState
            .menu[C2Rust_Unnamed::KEY_MENU_SELECT.0 as ::core::ffi::c_int as usize] != 0
            || PulsedInputState
                .menu[C2Rust_Unnamed::KEY_MENU_CANCEL.0 as ::core::ffi::c_int as usize]
                != 0
            || GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
                & C2Rust_Unnamed_0::CHECK_ABORT.0 as ::core::ffi::c_int != 0)
            as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
}
#[no_mangle]
pub unsafe extern "C" fn DoPopupWindow(mut msg: *const ::core::ffi::c_char) {
    let mut bank: *mut stringbank = StringBank_Create();
    let mut lines: [*const ::core::ffi::c_char; 30] = [::core::ptr::null::<
        ::core::ffi::c_char,
    >(); 30];
    let mut label: WIDGET_LABEL = WIDGET_LABEL {
        tag: WIDGET_TYPE::WIDGET_TYPE_MENU_SCREEN,
        parent: ::core::ptr::null_mut::<_widget>(),
        handleEvent: None,
        receiveFocus: None,
        draw: None,
        height: None,
        width: None,
        line_count: 0,
        lines: ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    };
    let mut s: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    let mut oldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    let mut oldRect: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut windowRect: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut state: POPUP_STATE = POPUP_STATE { InputFunc: None };
    let mut s0: MENU_SOUND_FLAGS = 0;
    let mut s1: MENU_SOUND_FLAGS = 0;
    let mut oldCallback: Option<InputFrameCallback> = None;
    if bank.is_null() {
        log_add(
            log_Level::log_Fatal,
            b"FATAL: Memory exhaustion when preparing popup window\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        exit(1 as ::core::ffi::c_int);
    }
    label.tag = WIDGET_TYPE::WIDGET_TYPE_LABEL;
    label.parent = ::core::ptr::null_mut::<_widget>();
    label.handleEvent = Some(
        Widget_HandleEventIgnoreAll
            as unsafe extern "C" fn(
                *mut WIDGET,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
    )
        as Option<
            unsafe extern "C" fn(*mut _widget, ::core::ffi::c_int) -> ::core::ffi::c_int,
        >;
    label.receiveFocus = Some(
        Widget_ReceiveFocusRefuseFocus
            as unsafe extern "C" fn(
                *mut WIDGET,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
    )
        as Option<
            unsafe extern "C" fn(*mut _widget, ::core::ffi::c_int) -> ::core::ffi::c_int,
        >;
    label.draw = Some(
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
    label.height = Some(
        Widget_HeightLabel as unsafe extern "C" fn(*mut WIDGET) -> ::core::ffi::c_int,
    ) as Option<unsafe extern "C" fn(*mut _widget) -> ::core::ffi::c_int>;
    label.width = Some(
        Widget_WidthFullScreen as unsafe extern "C" fn(*mut WIDGET) -> ::core::ffi::c_int,
    ) as Option<unsafe extern "C" fn(*mut _widget) -> ::core::ffi::c_int>;
    label.line_count = SplitString(
        msg,
        '\n' as ::core::ffi::c_char,
        30 as ::core::ffi::c_int,
        &raw mut lines as *mut *const ::core::ffi::c_char,
        bank,
    );
    label.lines = &raw mut lines as *mut *const ::core::ffi::c_char;
    PauseFlash();
    oldContext = SetContext(ScreenContext);
    GetContextClipRect(&raw mut oldRect);
    SetContextClipRect(::core::ptr::null_mut::<RECT>());
    s = SaveContextFrame(::core::ptr::null::<RECT>());
    Widget_SetFont(StarConFont);
    Widget_SetWindowColors(
        colorFromRgb15(0x14 as BYTE, 0x14 as BYTE, 0x14 as BYTE),
        colorFromRgb15(0x8 as BYTE, 0x8 as BYTE, 0x8 as BYTE),
        colorFromRgb15(0x10 as BYTE, 0x10 as BYTE, 0x10 as BYTE),
    );
    DrawLabelAsWindow(&raw mut label, &raw mut windowRect);
    SetSystemRect(&raw mut windowRect);
    GetMenuSounds(&raw mut s0, &raw mut s1);
    SetMenuSounds(
        0 as ::core::ffi::c_int as MENU_SOUND_FLAGS,
        0 as ::core::ffi::c_int as MENU_SOUND_FLAGS,
    );
    oldCallback = SetInputCallback(None);
    state.InputFunc = Some(DoPopup as unsafe extern "C" fn(*mut popup_state) -> BOOLEAN)
        as Option<unsafe extern "C" fn(*mut popup_state) -> BOOLEAN>;
    DoInput(&raw mut state as *mut ::core::ffi::c_void, BOOLEAN::TRUE);
    SetInputCallback(oldCallback);
    ClearSystemRect();
    DrawStamp(&raw mut s);
    DestroyDrawable(ReleaseDrawable(s.frame));
    SetContextClipRect(&raw mut oldRect);
    SetContext(oldContext);
    ContinueFlash();
    SetMenuSounds(s0, s1);
    StringBank_Free(bank);
}
