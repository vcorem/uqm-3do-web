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

// Rust translation of src/uqm/planets/cargo.c.
extern "C" {
    pub type string_table_entry;
    pub type context_desc;
    pub type font_desc;
    pub type tfb_soundsample;
    pub type FlashContext;
    fn snprintf(
        _: *mut ::core::ffi::c_char,
        _: size_t,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn SetAbsStringTableIndex(String: STRING, StringTableIndex: COUNT) -> STRING;
    fn GetStringAddress(String: STRING) -> STRINGPTR;
    fn SetContext(Context: CONTEXT) -> CONTEXT;
    fn SetContextForeGroundColor(Color: Color) -> Color;
    fn DrawStamp(pStamp: *mut STAMP);
    fn DrawFilledRectangle(pRect: *mut RECT);
    fn font_DrawText(pText: *mut TEXT);
    fn BatchGraphics();
    fn UnbatchGraphics();
    fn SetContextFont(Font: FONT) -> FONT;
    fn SetAbsFrameIndex(Frame: FRAME, FrameIndex: COUNT) -> FRAME;
    fn SetRelFrameIndex(Frame: FRAME, FrameOffs: SIZE) -> FRAME;
    static mut PulsedInputState: CONTROLLER_INPUT_STATE;
    fn DoInput(pInputState: *mut ::core::ffi::c_void, resetInput: BOOLEAN);
    fn SleepThread(timePeriod: TimePeriod);
    fn memset(
        _: *mut ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    static mut GlobData: GLOBDATA;
    static mut MiscDataFrame: FRAME;
    static mut StatusContext: CONTEXT;
    static mut StarConFont: FONT;
    static mut TinyFont: FONT;
    fn DrawStarConBox(
        pRect: *mut RECT,
        BorderWidth: SIZE,
        TopLeftColor: Color,
        BottomRightColor: Color,
        FillInterior: BOOLEAN,
        InteriorColor: Color,
    );
    fn SetMenuSounds(sound_0: MENU_SOUND_FLAGS, sound_1: MENU_SOUND_FLAGS);
    fn PlayMenuSound(S: MENU_SOUND_EFFECT);
    static mut GameStrings: STRING;
    fn ClearSISRect(ClearFlags: BYTE);
    fn DrawStatusMessage(pStr: *const UNICODE);
    fn GetStorageBayCapacity() -> COUNT;
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
pub type FONT_DESC = font_desc;
pub type CONTEXT = *mut CONTEXT_DESC;
pub type FRAME = *mut FRAME_DESC;
pub type FONT = *mut FONT_DESC;
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
pub type TFB_SoundSample = tfb_soundsample;
pub type MUSIC_REF = *mut *mut TFB_SoundSample;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct menu_state {
    pub InputFunc: Option<unsafe extern "C" fn(*mut menu_state) -> BOOLEAN>,
    pub Initialized: SIZE,
    pub CurState: BYTE,
    pub CurFrame: FRAME,
    pub CurString: STRING,
    pub first_item: POINT,
    pub delta_item: SIZE,
    pub ModuleFrame: FRAME,
    pub flash_rect0: RECT,
    pub flash_rect1: RECT,
    pub flash_frame0: FRAME,
    pub flash_frame1: FRAME,
    pub flashContext: *mut FlashContext,
    pub hMusic: MUSIC_REF,
    pub privData: *mut ::core::ffi::c_void,
}
pub type MENU_STATE = menu_state;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GLOBDATA {
    pub SIS_state: SIS_STATE,
    pub Game_state: GAME_STATE,
}
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
pub type ACTIVITY = UWORD;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct MENU_SOUND_EFFECT(pub ::core::ffi::c_uint);
impl MENU_SOUND_EFFECT {
    pub const MENU_SOUND_MOVE: Self = Self(0);
    pub const MENU_SOUND_SUCCESS: Self = Self(1);
    pub const MENU_SOUND_FAILURE: Self = Self(2);
    pub const MENU_SOUND_INVOKED: Self = Self(3);
}
pub type MENU_SOUND_FLAGS = UWORD;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_1(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_1 {
    pub const COMMON: Self = Self(0);
    pub const CORROSIVE: Self = Self(1);
    pub const BASE_METAL: Self = Self(2);
    pub const NOBLE: Self = Self(3);
    pub const RARE_EARTH: Self = Self(4);
    pub const PRECIOUS: Self = Self(5);
    pub const RADIOACTIVE: Self = Self(6);
    pub const EXOTIC: Self = Self(7);
    pub const NUM_ELEMENT_CATEGORIES: Self = Self(8);
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
pub unsafe extern "C" fn ShowRemainingCapacity() {
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
    let mut OldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    let mut buf: [UNICODE; 40] = [0; 40];
    OldContext = SetContext(StatusContext);
    SetContextFont(TinyFont);
    r.corner.x = 40 as COORD;
    r.corner.y = (35 as ::core::ffi::c_int
        + C2Rust_Unnamed_1::NUM_ELEMENT_CATEGORIES.0 as ::core::ffi::c_int
            * 9 as ::core::ffi::c_int) as COORD;
    snprintf(
        &raw mut buf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[UNICODE; 40]>(),
        b"%u\0".as_ptr() as *const ::core::ffi::c_char,
        GetStorageBayCapacity() as ::core::ffi::c_int
            - GlobData.SIS_state.TotalElementMass as ::core::ffi::c_int,
    );
    t.baseline.x = (58 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as COORD;
    t.baseline.y = (r.corner.y as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as COORD;
    t.align = TEXT_ALIGN::ALIGN_RIGHT;
    t.pStr = &raw mut buf as *mut UNICODE;
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    r.extent.width = (t.baseline.x as ::core::ffi::c_int
        - r.corner.x as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as COORD;
    r.extent.height = (9 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as COORD;
    BatchGraphics();
    SetContextForeGroundColor(colorFromRgb15(0 as BYTE, 0 as BYTE, 0x14 as BYTE));
    DrawFilledRectangle(&raw mut r);
    SetContextForeGroundColor(colorFromRgb15(0xa as BYTE, 0xa as BYTE, 0x1f as BYTE));
    font_DrawText(&raw mut t);
    UnbatchGraphics();
    SetContext(OldContext);
}
unsafe extern "C" fn DrawElementAmount(mut element: COUNT, mut selected: bool) {
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
    let mut buf: [UNICODE; 40] = [0; 40];
    r.corner.x = (7 as ::core::ffi::c_int + 7 as ::core::ffi::c_int
        + 5 as ::core::ffi::c_int) as COORD;
    r.extent.width = (58 as ::core::ffi::c_int
        - (7 as ::core::ffi::c_int + 7 as ::core::ffi::c_int + 5 as ::core::ffi::c_int)
        + 1 as ::core::ffi::c_int) as COORD;
    r.extent.height = (9 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as COORD;
    if element as ::core::ffi::c_int
        == C2Rust_Unnamed_1::NUM_ELEMENT_CATEGORIES.0 as ::core::ffi::c_int
    {
        r.corner.y = 119 as COORD;
    } else {
        r.corner.y = (35 as ::core::ffi::c_int
            + element as ::core::ffi::c_int * 9 as ::core::ffi::c_int) as COORD;
    }
    SetContextForeGroundColor(
        if selected as ::core::ffi::c_int != 0 {
            colorFromRgb15(0xa as BYTE, 0xa as BYTE, 0x1f as BYTE)
        } else {
            colorFromRgb15(0 as BYTE, 0 as BYTE, 0x14 as BYTE)
        },
    );
    DrawFilledRectangle(&raw mut r);
    t.align = TEXT_ALIGN::ALIGN_RIGHT;
    t.pStr = &raw mut buf as *mut UNICODE;
    t.baseline.y = (r.corner.y as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as COORD;
    if element as ::core::ffi::c_int
        == C2Rust_Unnamed_1::NUM_ELEMENT_CATEGORIES.0 as ::core::ffi::c_int
    {
        snprintf(
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[UNICODE; 40]>(),
            b"%u\0".as_ptr() as *const ::core::ffi::c_char,
            GlobData.SIS_state.TotalBioMass as ::core::ffi::c_int,
        );
    } else {
        SetContextForeGroundColor(
            if selected as ::core::ffi::c_int != 0 {
                colorFromRgb15(0 as BYTE, 0x14 as BYTE, 0x14 as BYTE)
            } else {
                colorFromRgb15(0xa as BYTE, 0xa as BYTE, 0x1f as BYTE)
            },
        );
        t.baseline.x = 32 as COORD;
        snprintf(
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[UNICODE; 40]>(),
            b"%u\0".as_ptr() as *const ::core::ffi::c_char,
            GlobData.Game_state.ElementWorth[element as usize] as ::core::ffi::c_int,
        );
        t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
        font_DrawText(&raw mut t);
        snprintf(
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[UNICODE; 40]>(),
            b"%u\0".as_ptr() as *const ::core::ffi::c_char,
            GlobData.SIS_state.ElementAmounts[element as usize] as ::core::ffi::c_int,
        );
    }
    SetContextForeGroundColor(
        if selected as ::core::ffi::c_int != 0 {
            colorFromRgb15(0xa as BYTE, 0x1f as BYTE, 0x1f as BYTE)
        } else {
            colorFromRgb15(0 as BYTE, 0x14 as BYTE, 0x14 as BYTE)
        },
    );
    t.baseline.x = 58 as COORD;
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    font_DrawText(&raw mut t);
}
unsafe extern "C" fn DrawCargoDisplay() {
    let mut s: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
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
    let mut cy: COORD = 0;
    let mut i: COUNT = 0;
    r.corner.x = 2 as COORD;
    r.extent.width = (64 as ::core::ffi::c_int - 5 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as COORD;
    r.corner.y = 20 as COORD;
    r.extent.height = (129 as ::core::ffi::c_int - r.corner.y as ::core::ffi::c_int)
        as COORD;
    DrawStarConBox(
        &raw mut r,
        1 as SIZE,
        colorFromRgb15(0x10 as BYTE, 0x10 as BYTE, 0x10 as BYTE),
        colorFromRgb15(0x8 as BYTE, 0x8 as BYTE, 0x8 as BYTE),
        BOOLEAN::TRUE,
        colorFromRgb15(0 as BYTE, 0 as BYTE, 0x14 as BYTE),
    );
    SetContextFont(StarConFont);
    t.baseline.x = ((64 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
        - 1 as ::core::ffi::c_int) as COORD;
    t.baseline.y = 27 as COORD;
    t.align = TEXT_ALIGN::ALIGN_CENTER;
    t.pStr = GetStringAddress(
        SetAbsStringTableIndex(
            GameStrings,
            C2Rust_Unnamed_0::CARGO_STRING_BASE.0 as ::core::ffi::c_int as COUNT,
        ),
    );
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    SetContextForeGroundColor(colorFromRgb15(0xa as BYTE, 0x1f as BYTE, 0x1f as BYTE));
    font_DrawText(&raw mut t);
    SetContextFont(TinyFont);
    s.frame = SetAbsFrameIndex(
        MiscDataFrame,
        (4 as ::core::ffi::c_int * 2 as ::core::ffi::c_int + 3 as ::core::ffi::c_int)
            as COUNT,
    );
    r.corner.x = 7 as COORD;
    r.extent = (*s.frame).Bounds;
    s.origin.x = (r.corner.x as ::core::ffi::c_int
        + (r.extent.width as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)) as COORD;
    cy = 35 as COORD;
    t.align = TEXT_ALIGN::ALIGN_RIGHT;
    t.baseline.y = (cy as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as COORD;
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    SetContextForeGroundColor(colorFromRgb15(0xa as BYTE, 0xa as BYTE, 0x1f as BYTE));
    t.baseline.x = 32 as COORD;
    t.pStr = b"$\0".as_ptr() as *const ::core::ffi::c_char as *const UNICODE;
    font_DrawText(&raw mut t);
    t.baseline.x = 58 as COORD;
    t.pStr = b"#\0".as_ptr() as *const ::core::ffi::c_char as *const UNICODE;
    font_DrawText(&raw mut t);
    i = 0 as COUNT;
    while (i as ::core::ffi::c_int)
        < C2Rust_Unnamed_1::NUM_ELEMENT_CATEGORIES.0 as ::core::ffi::c_int
    {
        SetContextForeGroundColor(colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE));
        r.corner.y = cy;
        DrawFilledRectangle(&raw mut r);
        s.origin.y = (r.corner.y as ::core::ffi::c_int
            + (r.extent.height as ::core::ffi::c_int >> 1 as ::core::ffi::c_int))
            as COORD;
        DrawStamp(&raw mut s);
        s.frame = SetRelFrameIndex(s.frame, 5 as SIZE);
        DrawElementAmount(i, false);
        i = i.wrapping_add(1);
        cy = (cy as ::core::ffi::c_int + 9 as ::core::ffi::c_int) as COORD;
    }
    SetContextForeGroundColor(colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE));
    r.corner.y = 119 as COORD;
    DrawFilledRectangle(&raw mut r);
    s.origin.y = (r.corner.y as ::core::ffi::c_int
        + (r.extent.height as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)) as COORD;
    s.frame = SetAbsFrameIndex(s.frame, 68 as COUNT);
    DrawStamp(&raw mut s);
    DrawElementAmount(
        C2Rust_Unnamed_1::NUM_ELEMENT_CATEGORIES.0 as ::core::ffi::c_int as COUNT,
        false,
    );
    r.corner.x = 4 as COORD;
    r.corner.y = (119 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as COORD;
    r.extent.width = (64 as ::core::ffi::c_int - 5 as ::core::ffi::c_int
        - 3 as ::core::ffi::c_int) as COORD;
    r.extent.height = 1 as COORD;
    SetContextForeGroundColor(colorFromRgb15(0xa as BYTE, 0xa as BYTE, 0x1f as BYTE));
    DrawFilledRectangle(&raw mut r);
    t.baseline.x = 5 as COORD;
    t.baseline.y = (35 as ::core::ffi::c_int
        + C2Rust_Unnamed_1::NUM_ELEMENT_CATEGORIES.0 as ::core::ffi::c_int
            * 9 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as COORD;
    t.align = TEXT_ALIGN::ALIGN_LEFT;
    t.pStr = GetStringAddress(
        SetAbsStringTableIndex(
            GameStrings,
            (C2Rust_Unnamed_0::CARGO_STRING_BASE.0 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int) as COUNT,
        ),
    );
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    font_DrawText(&raw mut t);
    ShowRemainingCapacity();
}
#[no_mangle]
pub unsafe extern "C" fn DrawCargoStrings(mut OldElement: BYTE, mut NewElement: BYTE) {
    let mut OldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    OldContext = SetContext(StatusContext);
    SetContextFont(TinyFont);
    BatchGraphics();
    if OldElement as ::core::ffi::c_int
        > C2Rust_Unnamed_1::NUM_ELEMENT_CATEGORIES.0 as ::core::ffi::c_int
    {
        DrawCargoDisplay();
        OldElement = NewElement;
    }
    if OldElement as ::core::ffi::c_int != NewElement as ::core::ffi::c_int {
        DrawElementAmount(OldElement as COUNT, false);
    }
    if NewElement as ::core::ffi::c_int
        != !(0 as ::core::ffi::c_int) as BYTE as ::core::ffi::c_int
    {
        DrawElementAmount(NewElement as COUNT, true);
    }
    UnbatchGraphics();
    SetContext(OldContext);
}
unsafe extern "C" fn DrawElementDescription(mut element: COUNT) {
    DrawStatusMessage(
        GetStringAddress(
            SetAbsStringTableIndex(
                GameStrings,
                (element as ::core::ffi::c_int
                    + (C2Rust_Unnamed_0::CARGO_STRING_BASE.0 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int)) as COUNT,
            ),
        ),
    );
}
unsafe extern "C" fn DoDiscardCargo(mut pMS: *mut MENU_STATE) -> BOOLEAN {
    let mut NewState: BYTE = 0;
    let mut select: BOOLEAN = BOOLEAN::FALSE;
    let mut cancel: BOOLEAN = BOOLEAN::FALSE;
    let mut back: BOOLEAN = BOOLEAN::FALSE;
    let mut forward: BOOLEAN = BOOLEAN::FALSE;
    select = BOOLEAN(
        PulsedInputState
            .menu[C2Rust_Unnamed::KEY_MENU_SELECT.0 as ::core::ffi::c_int as usize]
            as ::core::ffi::c_uint,
    );
    cancel = BOOLEAN(
        PulsedInputState
            .menu[C2Rust_Unnamed::KEY_MENU_CANCEL.0 as ::core::ffi::c_int as usize]
            as ::core::ffi::c_uint,
    );
    back = BOOLEAN(
        (PulsedInputState
            .menu[C2Rust_Unnamed::KEY_MENU_UP.0 as ::core::ffi::c_int as usize] != 0
            || PulsedInputState
                .menu[C2Rust_Unnamed::KEY_MENU_LEFT.0 as ::core::ffi::c_int as usize]
                != 0) as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
    forward = BOOLEAN(
        (PulsedInputState
            .menu[C2Rust_Unnamed::KEY_MENU_DOWN.0 as ::core::ffi::c_int as usize] != 0
            || PulsedInputState
                .menu[C2Rust_Unnamed::KEY_MENU_RIGHT.0 as ::core::ffi::c_int as usize]
                != 0) as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
    if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
        & C2Rust_Unnamed_2::CHECK_ABORT.0 as ::core::ffi::c_int != 0
    {
        return BOOLEAN::FALSE;
    }
    if cancel.0 != 0 {
        return BOOLEAN::FALSE
    } else if select.0 != 0 {
        if GlobData.SIS_state.ElementAmounts[(*pMS).CurState as usize] != 0 {
            GlobData.SIS_state.ElementAmounts[(*pMS).CurState as usize] = GlobData
                .SIS_state
                .ElementAmounts[(*pMS).CurState as usize]
                .wrapping_sub(1);
            DrawCargoStrings((*pMS).CurState, (*pMS).CurState);
            GlobData.SIS_state.TotalElementMass = GlobData
                .SIS_state
                .TotalElementMass
                .wrapping_sub(1);
            ShowRemainingCapacity();
        } else {
            PlayMenuSound(MENU_SOUND_EFFECT::MENU_SOUND_FAILURE);
        }
    } else {
        NewState = (*pMS).CurState;
        if back.0 != 0 {
            if NewState as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                NewState = (NewState as ::core::ffi::c_int
                    + C2Rust_Unnamed_1::NUM_ELEMENT_CATEGORIES.0 as ::core::ffi::c_int)
                    as BYTE;
            }
            NewState = NewState.wrapping_sub(1);
        } else if forward.0 != 0 {
            NewState = NewState.wrapping_add(1);
            if NewState as ::core::ffi::c_int
                == C2Rust_Unnamed_1::NUM_ELEMENT_CATEGORIES.0 as ::core::ffi::c_int
            {
                NewState = 0 as BYTE;
            }
        }
        if NewState as ::core::ffi::c_int != (*pMS).CurState as ::core::ffi::c_int {
            DrawCargoStrings((*pMS).CurState, NewState);
            DrawElementDescription(NewState as COUNT);
            (*pMS).CurState = NewState;
        }
    }
    SleepThread((840 as ::core::ffi::c_int / 30 as ::core::ffi::c_int) as TimePeriod);
    return BOOLEAN::TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn CargoMenu() {
    let mut MenuState: MENU_STATE = MENU_STATE {
        InputFunc: None,
        Initialized: 0,
        CurState: 0,
        CurFrame: ::core::ptr::null_mut::<FRAME_DESC>(),
        CurString: ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>(),
        first_item: POINT { x: 0, y: 0 },
        delta_item: 0,
        ModuleFrame: ::core::ptr::null_mut::<FRAME_DESC>(),
        flash_rect0: RECT {
            corner: POINT { x: 0, y: 0 },
            extent: EXTENT { width: 0, height: 0 },
        },
        flash_rect1: RECT {
            corner: POINT { x: 0, y: 0 },
            extent: EXTENT { width: 0, height: 0 },
        },
        flash_frame0: ::core::ptr::null_mut::<FRAME_DESC>(),
        flash_frame1: ::core::ptr::null_mut::<FRAME_DESC>(),
        flashContext: ::core::ptr::null_mut::<FlashContext>(),
        hMusic: ::core::ptr::null_mut::<*mut TFB_SoundSample>(),
        privData: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    memset(
        &raw mut MenuState as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<MENU_STATE>(),
    );
    DrawCargoStrings(!(0 as ::core::ffi::c_int) as BYTE, MenuState.CurState);
    DrawElementDescription(MenuState.CurState as COUNT);
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
    MenuState.InputFunc = Some(
        DoDiscardCargo as unsafe extern "C" fn(*mut MENU_STATE) -> BOOLEAN,
    ) as Option<unsafe extern "C" fn(*mut menu_state) -> BOOLEAN>;
    DoInput(&raw mut MenuState as *mut ::core::ffi::c_void, BOOLEAN::TRUE);
    ClearSISRect(((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as BYTE);
}
