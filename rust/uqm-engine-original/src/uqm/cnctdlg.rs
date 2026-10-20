//Copyright Michael Martin, 2006

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

// Rust translation of src/uqm/cnctdlg.c.
extern "C" {
    pub type font_desc;
    pub type frame_desc;
    pub type string_table_entry;
    pub type joy_char;
    fn SetAbsStringTableIndex(String: STRING, StringTableIndex: COUNT) -> STRING;
    fn GetStringAddress(String: STRING) -> STRINGPTR;
    fn utf8StringCount(start: *const ::core::ffi::c_char) -> size_t;
    fn utf8StringCopy(
        dst: *mut ::core::ffi::c_char,
        size: size_t,
        src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_uchar;
    fn SetContextForeGroundColor(Color: Color) -> Color;
    fn DrawFilledRectangle(pRect: *mut RECT);
    fn font_DrawText(pText: *mut TEXT);
    fn BatchGraphics();
    fn UnbatchGraphics();
    fn LoadGraphicInstance(res: RESOURCE) -> *mut ::core::ffi::c_void;
    fn SetContextFontEffect(EffectFrame: FRAME) -> FRAME;
    fn SetContextFont(Font: FONT) -> FONT;
    fn DestroyFont(FontRef: FONT) -> BOOLEAN;
    fn TextRect(pText: *mut TEXT, pRect: *mut RECT, pdelta: *mut BYTE) -> BOOLEAN;
    fn GetContextFontLeading(pheight: *mut SIZE) -> BOOLEAN;
    fn GetTimeCounter() -> TimeCount;
    static mut GameStrings: STRING;
    fn SleepThreadUntil(wakeTime: TimeCount);
    static mut PulsedInputState: CONTROLLER_INPUT_STATE;
    fn SetDefaultMenuRepeatDelay();
    fn DoInput(pInputState: *mut ::core::ffi::c_void, resetInput: BOOLEAN);
    static mut GamePaused: BOOLEAN;
    fn DoTextEntry(pTES: *mut TEXTENTRY_STATE) -> BOOLEAN;
    fn strncpy(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: size_t,
    ) -> *mut ::core::ffi::c_char;
    static mut GlobData: GLOBDATA;
    static mut StarConFont: FONT;
    fn DrawShadowedBox(r: *mut RECT, bg: Color, dark: Color, medium: Color);
    fn Widget_Event(event: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn Widget_ReceiveFocusMenuScreen(
        _self: *mut WIDGET,
        event: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn Widget_ReceiveFocusSimple(
        _self: *mut WIDGET,
        event: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn Widget_HandleEventMenuScreen(
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
    fn Widget_HeightFullScreen(_self: *mut WIDGET) -> ::core::ffi::c_int;
    fn Widget_HeightOneLine(_self: *mut WIDGET) -> ::core::ffi::c_int;
    fn Widget_WidthFullScreen(_self: *mut WIDGET) -> ::core::ffi::c_int;
    fn Widget_Slider_DrawValue(
        self_0: *mut WIDGET_SLIDER,
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
    );
    static mut widget_focus: *mut WIDGET;
    static mut netplayOptions: NetplayOptions;
    static mut ScreenWidth: ::core::ffi::c_int;
    static mut ScreenHeight: ::core::ffi::c_int;
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
pub type CONNECT_DIALOG_STATE = connect_dialog_state;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct connect_dialog_state {
    pub InputFunc: Option<unsafe extern "C" fn(*mut connect_dialog_state) -> BOOLEAN>,
    pub NextTime: DWORD,
    pub Initialized: BOOLEAN,
    pub which_side: ::core::ffi::c_int,
    pub confirmed: ::core::ffi::c_int,
}
pub type FONT = *mut FONT_DESC;
pub type FONT_DESC = font_desc;
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
pub type EXTENT = extent;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct extent {
    pub width: COORD,
    pub height: COORD,
}
pub type COORD = SWORD;
pub type STAMP = stamp;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stamp {
    pub origin: POINT,
    pub frame: FRAME,
}
pub type FRAME = *mut FRAME_DESC;
pub type FRAME_DESC = frame_desc;
pub type POINT = point;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct point {
    pub x: COORD,
    pub y: COORD,
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
pub struct PRIMITIVE {
    pub Links: PRIM_LINKS,
    pub Type: GRAPHICS_PRIM,
    pub color: Color,
    pub Object: PRIM_DESC,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union PRIM_DESC {
    pub Point: POINT,
    pub Stamp: STAMP,
    pub Line: LINE,
    pub Text: TEXT,
    pub Rect: RECT,
}
pub type RECT = rect;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rect {
    pub corner: POINT,
    pub extent: EXTENT,
}
pub type TEXT = text;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct text {
    pub baseline: POINT,
    pub pStr: *const UNICODE,
    pub align: TEXT_ALIGN,
    pub CharCount: COUNT,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct TEXT_ALIGN(pub ::core::ffi::c_uint);
impl TEXT_ALIGN {
    pub const ALIGN_LEFT: Self = Self(0);
    pub const ALIGN_CENTER: Self = Self(1);
    pub const ALIGN_RIGHT: Self = Self(2);
}
pub type LINE = line;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct line {
    pub first: POINT,
    pub second: POINT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Color {
    pub r: BYTE,
    pub g: BYTE,
    pub b: BYTE,
    pub a: BYTE,
}
pub type GRAPHICS_PRIM = BYTE;
pub type PRIM_LINKS = DWORD;
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
pub type TimeCount = DWORD;
pub type WIDGET = _widget;
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
pub type CONTROLLER_INPUT_STATE = _controller_input_state;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _controller_input_state {
    pub key: [[::core::ffi::c_int; 7]; 6],
    pub menu: [::core::ffi::c_int; 24],
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NetplayPeerOptions {
    pub isServer: bool,
    pub host: *const ::core::ffi::c_char,
    pub port: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NetplayOptions {
    pub metaServer: *const ::core::ffi::c_char,
    pub metaPort: *const ::core::ffi::c_char,
    pub peer: [NetplayPeerOptions; 2],
    pub inputDelay: size_t,
}
pub type size_t = usize;
pub type STRINGPTR = *mut ::core::ffi::c_char;
pub type STRING = *mut STRING_TABLE_ENTRY_DESC;
pub type STRING_TABLE_ENTRY_DESC = string_table_entry;
pub type TEXTENTRY_STATE = textentry_state;
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
pub type joy_char_t = joy_char;
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
pub type RESOURCE = *const ::core::ffi::c_char;
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_0 {
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_1(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_1 {
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
pub struct C2Rust_Unnamed_2(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_2 {
    pub const WIDGET_EVENT_UP: Self = Self(0);
    pub const WIDGET_EVENT_DOWN: Self = Self(1);
    pub const WIDGET_EVENT_LEFT: Self = Self(2);
    pub const WIDGET_EVENT_RIGHT: Self = Self(3);
    pub const WIDGET_EVENT_SELECT: Self = Self(4);
    pub const WIDGET_EVENT_CANCEL: Self = Self(5);
    pub const WIDGET_EVENT_DELETE: Self = Self(6);
    pub const NUM_WIDGET_EVENTS: Self = Self(7);
}
static mut menu: WIDGET_MENU_SCREEN = WIDGET_MENU_SCREEN {
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
};
static mut buttons: [WIDGET_BUTTON; 3] = [WIDGET_BUTTON {
    tag: WIDGET_TYPE::WIDGET_TYPE_MENU_SCREEN,
    parent: ::core::ptr::null_mut::<_widget>(),
    handleEvent: None,
    receiveFocus: None,
    draw: None,
    height: None,
    width: None,
    name: ::core::ptr::null::<::core::ffi::c_char>(),
    tooltip: [::core::ptr::null::<::core::ffi::c_char>(); 3],
}; 3];
static mut slider: WIDGET_SLIDER = WIDGET_SLIDER {
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
};
static mut texts: [WIDGET_TEXTENTRY; 2] = [WIDGET_TEXTENTRY {
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
}; 2];
static mut menu_widgets: [*mut WIDGET; 6] = [::core::ptr::null_mut::<WIDGET>(); 6];
static mut done: BOOLEAN = BOOLEAN::FALSE;
static mut current_state: *mut CONNECT_DIALOG_STATE = ::core::ptr::null_mut::<
    CONNECT_DIALOG_STATE,
>();
static mut PlayerFont: FONT = ::core::ptr::null_mut::<FONT_DESC>();
unsafe extern "C" fn MCD_DrawMenuScreen(
    mut _self: *mut WIDGET,
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
) {
    let mut widget_index: ::core::ffi::c_int = 0;
    let mut widget_y: ::core::ffi::c_int = 0;
    let mut self_0: *mut WIDGET_MENU_SCREEN = _self as *mut WIDGET_MENU_SCREEN;
    widget_y = y + 8 as ::core::ffi::c_int;
    widget_index = 0 as ::core::ffi::c_int;
    while widget_index < (*self_0).num_children {
        let mut c: *mut WIDGET = *(*self_0).child.offset(widget_index as isize);
        Some((*c).draw.expect("non-null function pointer"))
            .expect("non-null function pointer")(c, x, widget_y);
        widget_y
            += Some((*c).height.expect("non-null function pointer"))
                .expect("non-null function pointer")(c) + 8 as ::core::ffi::c_int;
        widget_index += 1;
    }
}
unsafe extern "C" fn MCD_DrawButton(
    mut _self: *mut WIDGET,
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
) {
    let mut self_0: *mut WIDGET_BUTTON = _self as *mut WIDGET_BUTTON;
    let mut oldtext: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    let mut inactive: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    let mut selected: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    let mut oldfont: FONT = SetContextFont(StarConFont);
    let mut oldFontEffect: FRAME = SetContextFontEffect(
        ::core::ptr::null_mut::<FRAME_DESC>(),
    );
    let mut t: TEXT = TEXT {
        baseline: POINT { x: 0, y: 0 },
        pStr: ::core::ptr::null::<UNICODE>(),
        align: TEXT_ALIGN::ALIGN_LEFT,
        CharCount: 0,
    };
    selected = colorFromRgb15(0x1f as BYTE, 0x1f as BYTE, 0 as BYTE);
    inactive = colorFromRgb15(0x6 as BYTE, 0x6 as BYTE, 0x6 as BYTE);
    t.baseline.x = 160 as COORD;
    t.baseline.y = y as COORD;
    t.align = TEXT_ALIGN::ALIGN_CENTER;
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    t.pStr = (*self_0).name as *const UNICODE;
    if widget_focus == _self {
        oldtext = SetContextForeGroundColor(selected);
    } else {
        oldtext = SetContextForeGroundColor(inactive);
    }
    font_DrawText(&raw mut t);
    SetContextFontEffect(oldFontEffect);
    SetContextFont(oldfont);
    SetContextForeGroundColor(oldtext);
}
unsafe extern "C" fn MCD_DrawSlider(
    mut _self: *mut WIDGET,
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
) {
    let mut self_0: *mut WIDGET_SLIDER = _self as *mut WIDGET_SLIDER;
    let mut oldtext: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    let mut default_color: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    let mut selected: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    let mut oldfont: FONT = SetContextFont(PlayerFont);
    let mut oldFontEffect: FRAME = SetContextFontEffect(
        ::core::ptr::null_mut::<FRAME_DESC>(),
    );
    let mut t: TEXT = TEXT {
        baseline: POINT { x: 0, y: 0 },
        pStr: ::core::ptr::null::<UNICODE>(),
        align: TEXT_ALIGN::ALIGN_LEFT,
        CharCount: 0,
    };
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut tick: ::core::ffi::c_int = 260 as ::core::ffi::c_int
        / 8 as ::core::ffi::c_int;
    default_color = colorFromRgb15(0x6 as BYTE, 0x6 as BYTE, 0x6 as BYTE);
    selected = colorFromRgb15(0x1f as BYTE, 0x1f as BYTE, 0 as BYTE);
    t.baseline.x = x as COORD;
    t.baseline.y = y as COORD;
    t.align = TEXT_ALIGN::ALIGN_LEFT;
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    t.pStr = (*self_0).category as *const UNICODE;
    if widget_focus == _self {
        oldtext = SetContextForeGroundColor(selected);
    } else {
        oldtext = SetContextForeGroundColor(default_color);
    }
    font_DrawText(&raw mut t);
    r.corner.x = (t.baseline.x as ::core::ffi::c_int + 3 as ::core::ffi::c_int * tick)
        as COORD;
    r.corner.y = (t.baseline.y as ::core::ffi::c_int - 4 as ::core::ffi::c_int) as COORD;
    r.extent.height = 2 as COORD;
    r.extent.width = (3 as ::core::ffi::c_int * tick) as COORD;
    DrawFilledRectangle(&raw mut r);
    r.extent.width = 3 as COORD;
    r.extent.height = 8 as COORD;
    r.corner.y = (t.baseline.y as ::core::ffi::c_int - 7 as ::core::ffi::c_int) as COORD;
    r.corner.x = (t.baseline.x as ::core::ffi::c_int + 3 as ::core::ffi::c_int * tick
        + 3 as ::core::ffi::c_int * tick * ((*self_0).value - (*self_0).min)
            / ((*self_0).max - (*self_0).min) - 1 as ::core::ffi::c_int) as COORD;
    DrawFilledRectangle(&raw mut r);
    Some((*self_0).draw_value.expect("non-null function pointer"))
        .expect(
            "non-null function pointer",
        )(
        self_0,
        t.baseline.x as ::core::ffi::c_int + 7 as ::core::ffi::c_int * tick,
        t.baseline.y as ::core::ffi::c_int,
    );
    SetContextFontEffect(oldFontEffect);
    SetContextFont(oldfont);
    SetContextForeGroundColor(oldtext);
}
unsafe extern "C" fn MCD_DrawTextEntry(
    mut _self: *mut WIDGET,
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
) {
    let mut self_0: *mut WIDGET_TEXTENTRY = _self as *mut WIDGET_TEXTENTRY;
    let mut oldtext: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    let mut inactive: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    let mut default_color: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    let mut selected: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    let mut oldfont: FONT = SetContextFont(PlayerFont);
    let mut oldFontEffect: FRAME = SetContextFontEffect(
        ::core::ptr::null_mut::<FRAME_DESC>(),
    );
    let mut t: TEXT = TEXT {
        baseline: POINT { x: 0, y: 0 },
        pStr: ::core::ptr::null::<UNICODE>(),
        align: TEXT_ALIGN::ALIGN_LEFT,
        CharCount: 0,
    };
    default_color = colorFromRgb15(0x6 as BYTE, 0x6 as BYTE, 0x6 as BYTE);
    selected = colorFromRgb15(0x1f as BYTE, 0x1f as BYTE, 0 as BYTE);
    inactive = colorFromRgb15(0x6 as BYTE, 0x6 as BYTE, 0x6 as BYTE);
    BatchGraphics();
    t.baseline.x = x as COORD;
    t.baseline.y = y as COORD;
    t.align = TEXT_ALIGN::ALIGN_LEFT;
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    t.pStr = (*self_0).category as *const UNICODE;
    if widget_focus == _self {
        oldtext = SetContextForeGroundColor(selected);
    } else {
        oldtext = SetContextForeGroundColor(default_color);
    }
    font_DrawText(&raw mut t);
    (*self_0).value[(50 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as usize] = 0
        as ::core::ffi::c_char;
    t.baseline.y = y as COORD;
    t.CharCount = utf8StringCount(&raw mut (*self_0).value as *mut ::core::ffi::c_char)
        as COUNT;
    t.pStr = &raw mut (*self_0).value as *mut ::core::ffi::c_char;
    if (*self_0).state.0 & WIDGET_TEXTENTRY_STATE::WTE_EDITING.0 == 0 {
        t.baseline.x = 160 as COORD;
        t.align = TEXT_ALIGN::ALIGN_CENTER;
        if widget_focus == _self {
            oldtext = SetContextForeGroundColor(selected);
        } else {
            oldtext = SetContextForeGroundColor(inactive);
        }
        font_DrawText(&raw mut t);
    } else {
        let mut i: COUNT = 0;
        let mut text_r: RECT = RECT {
            corner: POINT { x: 0, y: 0 },
            extent: EXTENT { width: 0, height: 0 },
        };
        let mut char_deltas: [BYTE; 50] = [0; 50];
        let mut pchar_deltas: *mut BYTE = ::core::ptr::null_mut::<BYTE>();
        let mut r: RECT = RECT {
            corner: POINT { x: 0, y: 0 },
            extent: EXTENT { width: 0, height: 0 },
        };
        let mut leading: SIZE = 0;
        t.baseline.x = (x + 90 as ::core::ffi::c_int) as COORD;
        t.align = TEXT_ALIGN::ALIGN_LEFT;
        GetContextFontLeading(&raw mut leading);
        r.corner.x = (t.baseline.x as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
            as COORD;
        r.corner.y = (t.baseline.y as ::core::ffi::c_int - leading as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int) as COORD;
        r.extent.width = (260 as ::core::ffi::c_int - r.corner.x as ::core::ffi::c_int
            - 10 as ::core::ffi::c_int) as COORD;
        r.extent.height = (leading as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
            as COORD;
        TextRect(&raw mut t, &raw mut text_r, &raw mut char_deltas as *mut BYTE);
        oldtext = SetContextForeGroundColor(selected);
        DrawFilledRectangle(&raw mut r);
        pchar_deltas = &raw mut char_deltas as *mut BYTE;
        i = (*self_0).cursor_pos as COUNT;
        while i as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
            let c2rust_fresh0 = pchar_deltas;
            pchar_deltas = pchar_deltas.offset(1);
            r.corner.x = (r.corner.x as ::core::ffi::c_int
                + *c2rust_fresh0 as SIZE as ::core::ffi::c_int) as COORD;
            i = i.wrapping_sub(1);
        }
        if (*self_0).cursor_pos < t.CharCount as ::core::ffi::c_int {
            r.corner.x -= 1;
        }
        if (*self_0).state.0 & WIDGET_TEXTENTRY_STATE::WTE_BLOCKCUR.0 != 0 {
            if (*self_0).cursor_pos == t.CharCount as ::core::ffi::c_int {
                r.extent.width = 1 as COORD;
            } else if (*self_0).cursor_pos + 1 as ::core::ffi::c_int
                == t.CharCount as ::core::ffi::c_int
            {
                r.extent.width = (*pchar_deltas as SIZE as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int) as COORD;
            } else {
                r.extent.width = (*pchar_deltas as SIZE as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int) as COORD;
            }
        } else {
            r.extent.width = 1 as COORD;
        }
        r.corner.x += 1;
        r.corner.y += 1;
        r.extent.height = (r.extent.height as ::core::ffi::c_int
            - 2 as ::core::ffi::c_int) as COORD;
        SetContextForeGroundColor(
            colorFromRgb15(0x1f as BYTE, 0x1f as BYTE, 0x1f as BYTE),
        );
        DrawFilledRectangle(&raw mut r);
        SetContextForeGroundColor(inactive);
        font_DrawText(&raw mut t);
    }
    UnbatchGraphics();
    SetContextFontEffect(oldFontEffect);
    SetContextFont(oldfont);
    SetContextForeGroundColor(oldtext);
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
    DrawConnectDialog();
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
    DrawConnectDialog();
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
    (*widget).state = WIDGET_TEXTENTRY_STATE::WTE_NORMAL;
    DrawConnectDialog();
    return BOOLEAN::TRUE.0 as ::core::ffi::c_int;
}
unsafe extern "C" fn do_connect(
    mut self_0: *mut WIDGET,
    mut event: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if event == C2Rust_Unnamed_2::WIDGET_EVENT_SELECT.0 as ::core::ffi::c_int {
        netplayOptions.peer[(*current_state).which_side as usize].host = &raw mut (*(&raw mut texts
            as *mut WIDGET_TEXTENTRY)
            .offset(0isize))
            .value as *mut ::core::ffi::c_char;
        netplayOptions.peer[(*current_state).which_side as usize].port = &raw mut (*(&raw mut texts
            as *mut WIDGET_TEXTENTRY)
            .offset(1isize))
            .value as *mut ::core::ffi::c_char;
        netplayOptions.peer[(*current_state).which_side as usize].isServer = BOOLEAN::FALSE
            .0 as ::core::ffi::c_int != 0;
        (*current_state).confirmed = BOOLEAN::TRUE.0 as ::core::ffi::c_int;
        netplayOptions.inputDelay = slider.value as size_t;
        done = BOOLEAN::TRUE;
    }
    return BOOLEAN::FALSE.0 as ::core::ffi::c_int;
}
unsafe extern "C" fn do_listen(
    mut self_0: *mut WIDGET,
    mut event: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if event == C2Rust_Unnamed_2::WIDGET_EVENT_SELECT.0 as ::core::ffi::c_int {
        netplayOptions.peer[(*current_state).which_side as usize].port = &raw mut (*(&raw mut texts
            as *mut WIDGET_TEXTENTRY)
            .offset(1isize))
            .value as *mut ::core::ffi::c_char;
        netplayOptions.peer[(*current_state).which_side as usize].isServer = BOOLEAN::TRUE
            .0 as ::core::ffi::c_int != 0;
        netplayOptions.inputDelay = slider.value as size_t;
        (*current_state).confirmed = BOOLEAN::TRUE.0 as ::core::ffi::c_int;
        done = BOOLEAN::TRUE;
    }
    return BOOLEAN::FALSE.0 as ::core::ffi::c_int;
}
unsafe extern "C" fn do_cancel(
    mut self_0: *mut WIDGET,
    mut event: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if event == C2Rust_Unnamed_2::WIDGET_EVENT_SELECT.0 as ::core::ffi::c_int {
        (*current_state).confirmed = BOOLEAN::FALSE.0 as ::core::ffi::c_int;
        done = BOOLEAN::TRUE;
    }
    return BOOLEAN::FALSE.0 as ::core::ffi::c_int;
}
unsafe extern "C" fn CreateWidgets() {
    let mut i: ::core::ffi::c_int = 0;
    done = BOOLEAN(0 as ::core::ffi::c_int as ::core::ffi::c_uint);
    i = 0 as ::core::ffi::c_int;
    while i < 3 as ::core::ffi::c_int {
        buttons[i as usize].tag = WIDGET_TYPE::WIDGET_TYPE_BUTTON;
        buttons[i as usize].parent = ::core::ptr::null_mut::<_widget>();
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
            MCD_DrawButton
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
        i += 1;
    }
    buttons[0usize].name = GetStringAddress(
        SetAbsStringTableIndex(
            GameStrings,
            (C2Rust_Unnamed_0::NETMELEE_STRING_BASE.0 as ::core::ffi::c_int
                + 19 as ::core::ffi::c_int) as COUNT,
        ),
    );
    buttons[1usize].name = GetStringAddress(
        SetAbsStringTableIndex(
            GameStrings,
            (C2Rust_Unnamed_0::NETMELEE_STRING_BASE.0 as ::core::ffi::c_int
                + 20 as ::core::ffi::c_int) as COUNT,
        ),
    );
    buttons[2usize].name = GetStringAddress(
        SetAbsStringTableIndex(
            GameStrings,
            (C2Rust_Unnamed_0::NETMELEE_STRING_BASE.0 as ::core::ffi::c_int
                + 21 as ::core::ffi::c_int) as COUNT,
        ),
    );
    buttons[0usize].handleEvent = Some(
        do_connect
            as unsafe extern "C" fn(
                *mut WIDGET,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
    )
        as Option<
            unsafe extern "C" fn(*mut _widget, ::core::ffi::c_int) -> ::core::ffi::c_int,
        >;
    buttons[1usize].handleEvent = Some(
        do_listen
            as unsafe extern "C" fn(
                *mut WIDGET,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
    )
        as Option<
            unsafe extern "C" fn(*mut _widget, ::core::ffi::c_int) -> ::core::ffi::c_int,
        >;
    buttons[2usize].handleEvent = Some(
        do_cancel
            as unsafe extern "C" fn(
                *mut WIDGET,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
    )
        as Option<
            unsafe extern "C" fn(*mut _widget, ::core::ffi::c_int) -> ::core::ffi::c_int,
        >;
    menu.tag = WIDGET_TYPE::WIDGET_TYPE_MENU_SCREEN;
    menu.parent = ::core::ptr::null_mut::<_widget>();
    menu.receiveFocus = Some(
        Widget_ReceiveFocusMenuScreen
            as unsafe extern "C" fn(
                *mut WIDGET,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
    )
        as Option<
            unsafe extern "C" fn(*mut _widget, ::core::ffi::c_int) -> ::core::ffi::c_int,
        >;
    menu.draw = Some(
        MCD_DrawMenuScreen
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
    menu.height = Some(
        Widget_HeightFullScreen
            as unsafe extern "C" fn(*mut WIDGET) -> ::core::ffi::c_int,
    ) as Option<unsafe extern "C" fn(*mut _widget) -> ::core::ffi::c_int>;
    menu.width = Some(
        Widget_WidthFullScreen as unsafe extern "C" fn(*mut WIDGET) -> ::core::ffi::c_int,
    ) as Option<unsafe extern "C" fn(*mut _widget) -> ::core::ffi::c_int>;
    menu.num_children = 6 as ::core::ffi::c_int;
    menu.child = &raw mut menu_widgets as *mut *mut WIDGET as *mut *mut _widget;
    menu.handleEvent = Some(
        Widget_HandleEventMenuScreen
            as unsafe extern "C" fn(
                *mut WIDGET,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
    )
        as Option<
            unsafe extern "C" fn(*mut _widget, ::core::ffi::c_int) -> ::core::ffi::c_int,
        >;
    slider.tag = WIDGET_TYPE::WIDGET_TYPE_SLIDER;
    slider.parent = ::core::ptr::null_mut::<_widget>();
    slider.handleEvent = Some(
        Widget_HandleEventSlider
            as unsafe extern "C" fn(
                *mut WIDGET,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
    )
        as Option<
            unsafe extern "C" fn(*mut _widget, ::core::ffi::c_int) -> ::core::ffi::c_int,
        >;
    slider.receiveFocus = Some(
        Widget_ReceiveFocusSimple
            as unsafe extern "C" fn(
                *mut WIDGET,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
    )
        as Option<
            unsafe extern "C" fn(*mut _widget, ::core::ffi::c_int) -> ::core::ffi::c_int,
        >;
    slider.draw = Some(
        MCD_DrawSlider
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
    slider.height = Some(
        Widget_HeightOneLine as unsafe extern "C" fn(*mut WIDGET) -> ::core::ffi::c_int,
    ) as Option<unsafe extern "C" fn(*mut _widget) -> ::core::ffi::c_int>;
    slider.width = Some(
        Widget_WidthFullScreen as unsafe extern "C" fn(*mut WIDGET) -> ::core::ffi::c_int,
    ) as Option<unsafe extern "C" fn(*mut _widget) -> ::core::ffi::c_int>;
    slider.draw_value = Some(
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
    slider.min = 0 as ::core::ffi::c_int;
    slider.max = 9 as ::core::ffi::c_int;
    slider.step = 1 as ::core::ffi::c_int;
    slider.value = netplayOptions.inputDelay as ::core::ffi::c_int;
    slider.category = GetStringAddress(
        SetAbsStringTableIndex(
            GameStrings,
            (C2Rust_Unnamed_0::NETMELEE_STRING_BASE.0 as ::core::ffi::c_int
                + 24 as ::core::ffi::c_int) as COUNT,
        ),
    );
    i = 0 as ::core::ffi::c_int;
    while i < 2 as ::core::ffi::c_int {
        texts[i as usize].tag = WIDGET_TYPE::WIDGET_TYPE_TEXTENTRY;
        texts[i as usize].parent = ::core::ptr::null_mut::<_widget>();
        texts[i as usize].handleEvent = Some(
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
        texts[i as usize].receiveFocus = Some(
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
        texts[i as usize].draw = Some(
            MCD_DrawTextEntry
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
        texts[i as usize].height = Some(
            Widget_HeightOneLine
                as unsafe extern "C" fn(*mut WIDGET) -> ::core::ffi::c_int,
        ) as Option<unsafe extern "C" fn(*mut _widget) -> ::core::ffi::c_int>;
        texts[i as usize].width = Some(
            Widget_WidthFullScreen
                as unsafe extern "C" fn(*mut WIDGET) -> ::core::ffi::c_int,
        ) as Option<unsafe extern "C" fn(*mut _widget) -> ::core::ffi::c_int>;
        texts[i as usize].handleEventSelect = Some(
            OnTextEntryEvent
                as unsafe extern "C" fn(*mut WIDGET_TEXTENTRY) -> ::core::ffi::c_int,
        ) as Option<unsafe extern "C" fn(*mut _widget_textentry) -> ::core::ffi::c_int>;
        texts[i as usize].maxlen = 50 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
        texts[i as usize].state = WIDGET_TEXTENTRY_STATE::WTE_NORMAL;
        texts[i as usize].cursor_pos = 0 as ::core::ffi::c_int;
        i += 1;
    }
    texts[0usize].category = GetStringAddress(
        SetAbsStringTableIndex(
            GameStrings,
            (C2Rust_Unnamed_0::NETMELEE_STRING_BASE.0 as ::core::ffi::c_int
                + 22 as ::core::ffi::c_int) as COUNT,
        ),
    );
    texts[1usize].category = GetStringAddress(
        SetAbsStringTableIndex(
            GameStrings,
            (C2Rust_Unnamed_0::NETMELEE_STRING_BASE.0 as ::core::ffi::c_int
                + 23 as ::core::ffi::c_int) as COUNT,
        ),
    );
    if &raw mut (*(&raw mut texts as *mut WIDGET_TEXTENTRY).offset(0isize)).value
        as *mut ::core::ffi::c_char
        != netplayOptions.peer[(*current_state).which_side as usize].host
            as *mut ::core::ffi::c_char
    {
        strncpy(
            &raw mut (*(&raw mut texts as *mut WIDGET_TEXTENTRY).offset(0isize)).value
                as *mut ::core::ffi::c_char,
            netplayOptions.peer[(*current_state).which_side as usize].host,
            texts[0usize].maxlen as size_t,
        );
    }
    if &raw mut (*(&raw mut texts as *mut WIDGET_TEXTENTRY).offset(1isize)).value
        as *mut ::core::ffi::c_char
        != netplayOptions.peer[(*current_state).which_side as usize].port
            as *mut ::core::ffi::c_char
    {
        strncpy(
            &raw mut (*(&raw mut texts as *mut WIDGET_TEXTENTRY).offset(1isize)).value
                as *mut ::core::ffi::c_char,
            netplayOptions.peer[(*current_state).which_side as usize].port,
            texts[1usize].maxlen as size_t,
        );
    }
    texts[0usize].value[texts[0usize].maxlen as usize] = 0 as ::core::ffi::c_char;
    texts[1usize].value[texts[1usize].maxlen as usize] = 0 as ::core::ffi::c_char;
    menu
        .receiveFocus
        .expect(
            "non-null function pointer",
        )(
        &raw mut menu as *mut _widget,
        C2Rust_Unnamed_2::WIDGET_EVENT_DOWN.0 as ::core::ffi::c_int,
    );
}
unsafe extern "C" fn DrawConnectDialog() {
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    r.extent.width = 260 as COORD;
    r.extent.height = 110 as COORD;
    r.corner.x = (ScreenWidth - r.extent.width as ::core::ffi::c_int
        >> 1 as ::core::ffi::c_int) as COORD;
    r.corner.y = (ScreenHeight - r.extent.height as ::core::ffi::c_int
        >> 1 as ::core::ffi::c_int) as COORD;
    DrawShadowedBox(
        &raw mut r,
        colorFromRgb15(0x14 as BYTE, 0x14 as BYTE, 0x14 as BYTE),
        colorFromRgb15(0x8 as BYTE, 0x8 as BYTE, 0x8 as BYTE),
        colorFromRgb15(0x10 as BYTE, 0x10 as BYTE, 0x10 as BYTE),
    );
    menu
        .draw
        .expect(
            "non-null function pointer",
        )(
        &raw mut menu as *mut _widget,
        r.corner.x as ::core::ffi::c_int + 10 as ::core::ffi::c_int,
        r.corner.y as ::core::ffi::c_int + 10 as ::core::ffi::c_int,
    );
}
unsafe extern "C" fn DoMeleeConnectDialog(
    mut state: *mut CONNECT_DIALOG_STATE,
) -> BOOLEAN {
    let mut changed: BOOLEAN = BOOLEAN::FALSE;
    ::core::ptr::write_volatile(&raw mut GamePaused, BOOLEAN::FALSE);
    if (*state).Initialized.0 == 0 {
        (*state).Initialized = BOOLEAN::TRUE;
        SetDefaultMenuRepeatDelay();
        (*state).NextTime = GetTimeCounter() as DWORD;
        CreateWidgets();
        DrawConnectDialog();
    }
    changed = BOOLEAN::TRUE;
    if PulsedInputState
        .menu[C2Rust_Unnamed::KEY_MENU_UP.0 as ::core::ffi::c_int as usize] != 0
    {
        Widget_Event(C2Rust_Unnamed_2::WIDGET_EVENT_UP.0 as ::core::ffi::c_int);
    } else if PulsedInputState
        .menu[C2Rust_Unnamed::KEY_MENU_DOWN.0 as ::core::ffi::c_int as usize] != 0
    {
        Widget_Event(C2Rust_Unnamed_2::WIDGET_EVENT_DOWN.0 as ::core::ffi::c_int);
    } else if PulsedInputState
        .menu[C2Rust_Unnamed::KEY_MENU_LEFT.0 as ::core::ffi::c_int as usize] != 0
    {
        Widget_Event(C2Rust_Unnamed_2::WIDGET_EVENT_LEFT.0 as ::core::ffi::c_int);
    } else if PulsedInputState
        .menu[C2Rust_Unnamed::KEY_MENU_RIGHT.0 as ::core::ffi::c_int as usize] != 0
    {
        Widget_Event(C2Rust_Unnamed_2::WIDGET_EVENT_RIGHT.0 as ::core::ffi::c_int);
    } else if PulsedInputState
        .menu[C2Rust_Unnamed::KEY_MENU_SELECT.0 as ::core::ffi::c_int as usize] != 0
    {
        Widget_Event(C2Rust_Unnamed_2::WIDGET_EVENT_SELECT.0 as ::core::ffi::c_int);
    } else if PulsedInputState
        .menu[C2Rust_Unnamed::KEY_MENU_CANCEL.0 as ::core::ffi::c_int as usize] != 0
    {
        Widget_Event(C2Rust_Unnamed_2::WIDGET_EVENT_CANCEL.0 as ::core::ffi::c_int);
    } else if PulsedInputState
        .menu[C2Rust_Unnamed::KEY_MENU_DELETE.0 as ::core::ffi::c_int as usize] != 0
    {
        Widget_Event(C2Rust_Unnamed_2::WIDGET_EVENT_DELETE.0 as ::core::ffi::c_int);
    } else {
        changed = BOOLEAN::FALSE;
    }
    if changed.0 != 0 {
        DrawConnectDialog();
    }
    SleepThreadUntil(
        (*state)
            .NextTime
            .wrapping_add(
                (840 as ::core::ffi::c_int / 20 as ::core::ffi::c_int) as TimeCount,
            ),
    );
    (*state).NextTime = GetTimeCounter() as DWORD;
    return BOOLEAN(
        !(GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
            & C2Rust_Unnamed_1::CHECK_ABORT.0 as ::core::ffi::c_int != 0 || done.0 != 0)
            as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
}
#[no_mangle]
pub unsafe extern "C" fn MeleeConnectDialog(mut side: ::core::ffi::c_int) -> BOOLEAN {
    let mut state: CONNECT_DIALOG_STATE = CONNECT_DIALOG_STATE {
        InputFunc: None,
        NextTime: 0,
        Initialized: BOOLEAN::FALSE,
        which_side: 0,
        confirmed: 0,
    };
    PlayerFont = LoadGraphicInstance(b"font.player\0".as_ptr() as RESOURCE) as FONT;
    state.Initialized = BOOLEAN::FALSE;
    state.which_side = side;
    state.InputFunc = Some(
        DoMeleeConnectDialog
            as unsafe extern "C" fn(*mut CONNECT_DIALOG_STATE) -> BOOLEAN,
    ) as Option<unsafe extern "C" fn(*mut connect_dialog_state) -> BOOLEAN>;
    state.confirmed = BOOLEAN::TRUE.0 as ::core::ffi::c_int;
    current_state = &raw mut state;
    DoInput(&raw mut state as *mut ::core::ffi::c_void, BOOLEAN::TRUE);
    current_state = ::core::ptr::null_mut::<CONNECT_DIALOG_STATE>();
    DestroyFont(PlayerFont);
    return BOOLEAN(state.confirmed as ::core::ffi::c_uint);
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
#[export_name = "uqm_c2rust_init_d335382ca1ccd2fd"]
unsafe extern "C" fn c2rust_run_static_initializers() {
    menu_widgets = [
        (&raw mut buttons as *mut WIDGET_BUTTON).offset(1isize) as *mut WIDGET,
        (&raw mut texts as *mut WIDGET_TEXTENTRY).offset(0isize) as *mut WIDGET,
        (&raw mut buttons as *mut WIDGET_BUTTON).offset(0isize) as *mut WIDGET,
        &raw mut slider as *mut WIDGET,
        (&raw mut texts as *mut WIDGET_TEXTENTRY).offset(1isize) as *mut WIDGET,
        (&raw mut buttons as *mut WIDGET_BUTTON).offset(2isize) as *mut WIDGET,
    ];
}
// Explicit engine initialization replaces platform constructor tables.
