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

// Rust translation of src/uqm/menu.c.
extern "C" {
    pub type string_table_entry;
    pub type context_desc;
    pub type frame_desc;
    pub type font_desc;
    pub type tfb_soundsample;
    pub type FlashContext;
    fn SetContext(Context: CONTEXT) -> CONTEXT;
    fn SetContextForeGroundColor(Color: Color) -> Color;
    fn GetContextClipRect(pRect: *mut RECT) -> BOOLEAN;
    fn DrawStamp(pStamp: *mut STAMP);
    fn DrawRectangle(pRect: *mut RECT);
    fn DrawFilledRectangle(pRect: *mut RECT);
    fn font_DrawText(pText: *mut TEXT);
    fn BatchGraphics();
    fn UnbatchGraphics();
    fn SetContextFont(Font: FONT) -> FONT;
    fn SetAbsFrameIndex(Frame: FRAME, FrameIndex: COUNT) -> FRAME;
    fn snprintf(
        _: *mut ::core::ffi::c_char,
        _: size_t,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn utf8StringCopy(
        dst: *mut ::core::ffi::c_char,
        size: size_t,
        src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_uchar;
    fn SetAbsStringTableIndex(String: STRING, StringTableIndex: COUNT) -> STRING;
    fn GetStringAddress(String: STRING) -> STRINGPTR;
    static mut PulsedInputState: CONTROLLER_INPUT_STATE;
    static mut ScreenWidth: ::core::ffi::c_int;
    static mut ScreenHeight: ::core::ffi::c_int;
    static mut optWhichMenu: ::core::ffi::c_int;
    fn SleepThread(timePeriod: TimePeriod);
    fn PreUpdateFlashRect();
    fn PostUpdateFlashRect();
    static mut GlobData: GLOBDATA;
    static mut StatusContext: CONTEXT;
    static mut StarConFont: FONT;
    static mut TinyFont: FONT;
    static mut GameStrings: STRING;
    fn log_add(_: log_Level, fmt: *const ::core::ffi::c_char, ...);
}
pub type size_t = usize;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Color {
    pub r: BYTE,
    pub g: BYTE,
    pub b: BYTE,
    pub a: BYTE,
}
pub type STRING_TABLE_ENTRY_DESC = string_table_entry;
pub type STRING = *mut STRING_TABLE_ENTRY_DESC;
pub type STRINGPTR = *mut ::core::ffi::c_char;
pub type CONTEXT_DESC = context_desc;
pub type FRAME_DESC = frame_desc;
pub type FONT_DESC = font_desc;
pub type CONTEXT = *mut CONTEXT_DESC;
pub type FRAME = *mut FRAME_DESC;
pub type FONT = *mut FONT_DESC;
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
pub type TFB_SoundSample = tfb_soundsample;
pub type MUSIC_REF = *mut *mut TFB_SoundSample;
pub type TimePeriod = DWORD;
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed {
    pub const PM_SCAN: Self = Self(0);
    pub const PM_STARMAP: Self = Self(1);
    pub const PM_DEVICES: Self = Self(2);
    pub const PM_CARGO: Self = Self(3);
    pub const PM_ROSTER: Self = Self(4);
    pub const PM_GAME_MENU: Self = Self(5);
    pub const PM_NAVIGATE: Self = Self(6);
    pub const PM_MIN_SCAN: Self = Self(7);
    pub const PM_ENE_SCAN: Self = Self(8);
    pub const PM_BIO_SCAN: Self = Self(9);
    pub const PM_EXIT_SCAN: Self = Self(10);
    pub const PM_AUTO_SCAN: Self = Self(11);
    pub const PM_LAUNCH_LANDER: Self = Self(12);
    pub const PM_SAVE_GAME: Self = Self(13);
    pub const PM_LOAD_GAME: Self = Self(14);
    pub const PM_QUIT_GAME: Self = Self(15);
    pub const PM_CHANGE_SETTINGS: Self = Self(16);
    pub const PM_EXIT_GAME_MENU: Self = Self(17);
    pub const PM_CONVERSE: Self = Self(18);
    pub const PM_ATTACK: Self = Self(19);
    pub const PM_ENCOUNTER_GAME_MENU: Self = Self(20);
    pub const PM_FUEL: Self = Self(21);
    pub const PM_MODULE: Self = Self(22);
    pub const PM_OUTFIT_GAME_MENU: Self = Self(23);
    pub const PM_EXIT_OUTFIT: Self = Self(24);
    pub const PM_CREW: Self = Self(25);
    pub const PM_SHIPYARD_GAME_MENU: Self = Self(26);
    pub const PM_EXIT_SHIPYARD: Self = Self(27);
    pub const PM_SOUND_ON: Self = Self(28);
    pub const PM_SOUND_OFF: Self = Self(29);
    pub const PM_MUSIC_ON: Self = Self(30);
    pub const PM_MUSIC_OFF: Self = Self(31);
    pub const PM_CYBORG_OFF: Self = Self(32);
    pub const PM_CYBORG_NORMAL: Self = Self(33);
    pub const PM_CYBORG_DOUBLE: Self = Self(34);
    pub const PM_CYBORG_SUPER: Self = Self(35);
    pub const PM_CHANGE_CAPTAIN: Self = Self(36);
    pub const PM_CHANGE_SHIP: Self = Self(37);
    pub const PM_EXIT_SETTINGS: Self = Self(38);
    pub const PM_ALT_SCAN: Self = Self(39);
    pub const PM_ALT_STARMAP: Self = Self(40);
    pub const PM_ALT_MANIFEST: Self = Self(41);
    pub const PM_ALT_GAME_MENU: Self = Self(42);
    pub const PM_ALT_NAVIGATE: Self = Self(43);
    pub const PM_ALT_CARGO: Self = Self(44);
    pub const PM_ALT_DEVICES: Self = Self(45);
    pub const PM_ALT_ROSTER: Self = Self(46);
    pub const PM_ALT_EXIT_MANIFEST: Self = Self(47);
    pub const PM_ALT_MSCAN: Self = Self(48);
    pub const PM_ALT_ESCAN: Self = Self(49);
    pub const PM_ALT_BSCAN: Self = Self(50);
    pub const PM_ALT_ASCAN: Self = Self(51);
    pub const PM_ALT_DISPATCH: Self = Self(52);
    pub const PM_ALT_EXIT_SCAN: Self = Self(53);
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
pub type CONTROLLER_INPUT_STATE = _controller_input_state;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _controller_input_state {
    pub key: [[::core::ffi::c_int; 7]; 6],
    pub menu: [::core::ffi::c_int; 24],
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
unsafe extern "C" fn DrawPCMenuFrame(mut r: *mut RECT) {
    SetContextForeGroundColor(colorFromRgb15(0xf as BYTE, 0xf as BYTE, 0xf as BYTE));
    DrawRectangle(r);
    (*r).corner.x += 1;
    (*r).corner.y += 1;
    (*r).extent.height -= 1;
    (*r).extent.width -= 1;
    SetContextForeGroundColor(colorFromRgb15(0x6 as BYTE, 0x6 as BYTE, 0x6 as BYTE));
    DrawRectangle(r);
    (*r).extent.height -= 1;
    (*r).extent.width -= 1;
    SetContextForeGroundColor(colorFromRgb15(0 as BYTE, 0 as BYTE, 0x15 as BYTE));
    DrawFilledRectangle(r);
}
static mut pm_crew_str: [UNICODE; 128] = [0; 128];
static mut pm_fuel_str: [UNICODE; 128] = [0; 128];
unsafe extern "C" fn DrawPCMenu(
    mut beg_index: BYTE,
    mut end_index: BYTE,
    mut NewState: BYTE,
    mut hilite: BYTE,
    mut r: *mut RECT,
) {
    let mut pos: BYTE = 0;
    let mut i: COUNT = 0;
    let mut num_items: ::core::ffi::c_int = 0;
    let mut OldFont: FONT = ::core::ptr::null_mut::<FONT_DESC>();
    let mut t: TEXT = TEXT {
        baseline: POINT { x: 0, y: 0 },
        pStr: ::core::ptr::null::<UNICODE>(),
        align: TEXT_ALIGN::ALIGN_LEFT,
        CharCount: 0,
    };
    let mut buf: [UNICODE; 256] = [0; 256];
    pos = (beg_index as ::core::ffi::c_int + NewState as ::core::ffi::c_int) as BYTE;
    num_items = 1 as ::core::ffi::c_int + end_index as ::core::ffi::c_int
        - beg_index as ::core::ffi::c_int;
    (*r).corner.x = ((*r).corner.x as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
        as COORD;
    (*r).extent.width = ((*r).extent.width as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as COORD;
    DrawFilledRectangle(r);
    if num_items * 8 as ::core::ffi::c_int > (*r).extent.height as ::core::ffi::c_int {
        log_add(
            log_Level::log_Error,
            b"Warning, no room for all menu items!\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    } else {
        (*r).corner.y = ((*r).corner.y as ::core::ffi::c_int
            + ((*r).extent.height as ::core::ffi::c_int
                - num_items * 8 as ::core::ffi::c_int) / 2 as ::core::ffi::c_int)
            as COORD;
    }
    (*r).extent.height = (num_items * 8 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
        as COORD;
    DrawPCMenuFrame(r);
    OldFont = SetContextFont(StarConFont);
    t.align = TEXT_ALIGN::ALIGN_LEFT;
    t.baseline.x = ((*r).corner.x as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
        as COORD;
    t.baseline.y = ((*r).corner.y as ::core::ffi::c_int + 8 as ::core::ffi::c_int
        - 1 as ::core::ffi::c_int) as COORD;
    t.pStr = &raw mut buf as *mut UNICODE;
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    (*r).corner.x += 1;
    (*r).extent.width = ((*r).extent.width as ::core::ffi::c_int
        - 2 as ::core::ffi::c_int) as COORD;
    i = beg_index as COUNT;
    while i as ::core::ffi::c_int <= end_index as ::core::ffi::c_int {
        utf8StringCopy(
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[UNICODE; 256]>(),
            if i as ::core::ffi::c_int == C2Rust_Unnamed::PM_FUEL.0 as ::core::ffi::c_int
            {
                &raw mut pm_fuel_str as *mut UNICODE
            } else if i as ::core::ffi::c_int
                == C2Rust_Unnamed::PM_CREW.0 as ::core::ffi::c_int
            {
                &raw mut pm_crew_str as *mut UNICODE
            } else {
                GetStringAddress(
                    SetAbsStringTableIndex(
                        GameStrings,
                        (C2Rust_Unnamed_1::MAINMENU_STRING_BASE.0 as ::core::ffi::c_int
                            + i as ::core::ffi::c_int) as COUNT,
                    ),
                )
            },
        );
        if hilite as ::core::ffi::c_int != 0
            && pos as ::core::ffi::c_int == i as ::core::ffi::c_int
        {
            SetContextForeGroundColor(
                colorFromRgb15(0xa as BYTE, 0xa as BYTE, 0x1f as BYTE),
            );
            (*r).corner.y = (t.baseline.y as ::core::ffi::c_int - 8 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int) as COORD;
            (*r).extent.height = (8 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                as COORD;
            DrawFilledRectangle(r);
            SetContextForeGroundColor(
                colorFromRgb15(0xa as BYTE, 0x1f as BYTE, 0x1f as BYTE),
            );
            font_DrawText(&raw mut t);
        } else {
            SetContextForeGroundColor(
                colorFromRgb15(0 as BYTE, 0x15 as BYTE, 0x15 as BYTE),
            );
            font_DrawText(&raw mut t);
        }
        t.baseline.y = (t.baseline.y as ::core::ffi::c_int + 8 as ::core::ffi::c_int)
            as COORD;
        i = i.wrapping_add(1);
    }
    SetContextFont(OldFont);
}
unsafe extern "C" fn GetEndMenuState(mut BaseState: BYTE) -> BYTE {
    match BaseState as ::core::ffi::c_int {
        0 | 1 => return C2Rust_Unnamed::PM_NAVIGATE.0 as ::core::ffi::c_int as BYTE,
        7 => return C2Rust_Unnamed::PM_LAUNCH_LANDER.0 as ::core::ffi::c_int as BYTE,
        13 => return C2Rust_Unnamed::PM_EXIT_GAME_MENU.0 as ::core::ffi::c_int as BYTE,
        18 => {
            return C2Rust_Unnamed::PM_ENCOUNTER_GAME_MENU.0 as ::core::ffi::c_int as BYTE;
        }
        21 => return C2Rust_Unnamed::PM_EXIT_OUTFIT.0 as ::core::ffi::c_int as BYTE,
        25 => return C2Rust_Unnamed::PM_EXIT_SHIPYARD.0 as ::core::ffi::c_int as BYTE,
        28 => return C2Rust_Unnamed::PM_EXIT_SETTINGS.0 as ::core::ffi::c_int as BYTE,
        39 | 40 => return C2Rust_Unnamed::PM_ALT_NAVIGATE.0 as ::core::ffi::c_int as BYTE,
        44 => return C2Rust_Unnamed::PM_ALT_EXIT_MANIFEST.0 as ::core::ffi::c_int as BYTE,
        48 => return C2Rust_Unnamed::PM_ALT_EXIT_SCAN.0 as ::core::ffi::c_int as BYTE,
        _ => {}
    }
    return BaseState;
}
unsafe extern "C" fn GetBeginMenuState(mut BaseState: BYTE) -> BYTE {
    return BaseState;
}
unsafe extern "C" fn FixMenuState(mut BadState: BYTE) -> BYTE {
    match BadState as ::core::ffi::c_int {
        28 => {
            if GlobData.Game_state.glob_flags as ::core::ffi::c_int
                & (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int != 0
            {
                return C2Rust_Unnamed::PM_SOUND_OFF.0 as ::core::ffi::c_int as BYTE
            } else {
                return C2Rust_Unnamed::PM_SOUND_ON.0 as ::core::ffi::c_int as BYTE
            }
        }
        30 => {
            if GlobData.Game_state.glob_flags as ::core::ffi::c_int
                & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int != 0
            {
                return C2Rust_Unnamed::PM_MUSIC_OFF.0 as ::core::ffi::c_int as BYTE
            } else {
                return C2Rust_Unnamed::PM_MUSIC_ON.0 as ::core::ffi::c_int as BYTE
            }
        }
        32 => {
            return (C2Rust_Unnamed::PM_CYBORG_OFF.0 as ::core::ffi::c_int
                + ((GlobData.Game_state.glob_flags as ::core::ffi::c_int
                    & (((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as BYTE
                    as ::core::ffi::c_int >> 6 as ::core::ffi::c_int)) as BYTE;
        }
        _ => {}
    }
    return BadState;
}
unsafe extern "C" fn NextMenuState(mut BaseState: BYTE, mut CurState: BYTE) -> BYTE {
    let mut NextState: BYTE = 0;
    let mut AdjBase: BYTE = BaseState;
    if BaseState as ::core::ffi::c_int
        == C2Rust_Unnamed::PM_STARMAP.0 as ::core::ffi::c_int
    {
        AdjBase = AdjBase.wrapping_sub(1);
    }
    match AdjBase as ::core::ffi::c_int + CurState as ::core::ffi::c_int {
        28 | 29 => {
            NextState = C2Rust_Unnamed::PM_MUSIC_ON.0 as ::core::ffi::c_int as BYTE;
        }
        30 | 31 => {
            NextState = C2Rust_Unnamed::PM_CYBORG_OFF.0 as ::core::ffi::c_int as BYTE;
        }
        32 | 33 | 34 | 35 => {
            NextState = C2Rust_Unnamed::PM_CHANGE_CAPTAIN.0 as ::core::ffi::c_int
                as BYTE;
        }
        _ => {
            NextState = (AdjBase as ::core::ffi::c_int + CurState as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int) as BYTE;
        }
    }
    if NextState as ::core::ffi::c_int > GetEndMenuState(BaseState) as ::core::ffi::c_int
    {
        NextState = GetBeginMenuState(BaseState);
    }
    return (FixMenuState(NextState) as ::core::ffi::c_int
        - AdjBase as ::core::ffi::c_int) as BYTE;
}
unsafe extern "C" fn PreviousMenuState(mut BaseState: BYTE, mut CurState: BYTE) -> BYTE {
    let mut NextState: SWORD = 0;
    let mut AdjBase: BYTE = BaseState;
    if BaseState as ::core::ffi::c_int
        == C2Rust_Unnamed::PM_STARMAP.0 as ::core::ffi::c_int
    {
        AdjBase = AdjBase.wrapping_sub(1);
    }
    match AdjBase as ::core::ffi::c_int + CurState as ::core::ffi::c_int {
        29 => {
            NextState = C2Rust_Unnamed::PM_EXIT_SETTINGS.0 as ::core::ffi::c_int
                as SWORD;
        }
        30 | 31 => {
            NextState = C2Rust_Unnamed::PM_SOUND_ON.0 as ::core::ffi::c_int as SWORD;
        }
        32 | 33 | 34 | 35 => {
            NextState = C2Rust_Unnamed::PM_MUSIC_ON.0 as ::core::ffi::c_int as SWORD;
        }
        36 => {
            NextState = C2Rust_Unnamed::PM_CYBORG_OFF.0 as ::core::ffi::c_int as SWORD;
        }
        _ => {
            NextState = (AdjBase as ::core::ffi::c_int + CurState as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as SWORD;
        }
    }
    if (NextState as ::core::ffi::c_int)
        < GetBeginMenuState(BaseState) as ::core::ffi::c_int
    {
        NextState = GetEndMenuState(BaseState) as SWORD;
    }
    return (FixMenuState(NextState as BYTE) as ::core::ffi::c_int
        - AdjBase as ::core::ffi::c_int) as BYTE;
}
unsafe extern "C" fn GetAlternateMenu(
    mut BaseState: *mut BYTE,
    mut CurState: *mut BYTE,
) -> BOOLEAN {
    let mut AdjBase: BYTE = *BaseState;
    let mut adj: BYTE = 0 as BYTE;
    if *BaseState as ::core::ffi::c_int
        == C2Rust_Unnamed::PM_STARMAP.0 as ::core::ffi::c_int
    {
        AdjBase = AdjBase.wrapping_sub(1);
        adj = 1 as BYTE;
    }
    if *CurState as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int != 0 {
        match *CurState as ::core::ffi::c_int {
            0x80 => {
                *BaseState = (C2Rust_Unnamed::PM_ALT_SCAN.0 as ::core::ffi::c_int
                    + adj as ::core::ffi::c_int) as BYTE;
                *CurState = (C2Rust_Unnamed::PM_ALT_MANIFEST.0 as ::core::ffi::c_int
                    - C2Rust_Unnamed::PM_ALT_SCAN.0 as ::core::ffi::c_int
                    - adj as ::core::ffi::c_int) as BYTE;
                return BOOLEAN::TRUE;
            }
            0x81 => {
                *BaseState = C2Rust_Unnamed::PM_ALT_CARGO.0 as ::core::ffi::c_int
                    as BYTE;
                *CurState = (C2Rust_Unnamed::PM_ALT_EXIT_MANIFEST.0 as ::core::ffi::c_int
                    - C2Rust_Unnamed::PM_ALT_CARGO.0 as ::core::ffi::c_int) as BYTE;
                return BOOLEAN::TRUE;
            }
            _ => {}
        }
        log_add(
            log_Level::log_Error,
            b"Unknown state combination: %d, %d\0".as_ptr()
                as *const ::core::ffi::c_char,
            *BaseState as ::core::ffi::c_int,
            *CurState as ::core::ffi::c_int,
        );
        return BOOLEAN::FALSE;
    } else {
        match AdjBase as ::core::ffi::c_int + *CurState as ::core::ffi::c_int {
            0 => {
                *BaseState = C2Rust_Unnamed::PM_ALT_SCAN.0 as ::core::ffi::c_int as BYTE;
                *CurState = (C2Rust_Unnamed::PM_ALT_SCAN.0 as ::core::ffi::c_int
                    - C2Rust_Unnamed::PM_ALT_SCAN.0 as ::core::ffi::c_int) as BYTE;
                return BOOLEAN::TRUE;
            }
            1 => {
                *BaseState = (C2Rust_Unnamed::PM_ALT_SCAN.0 as ::core::ffi::c_int
                    + adj as ::core::ffi::c_int) as BYTE;
                *CurState = (C2Rust_Unnamed::PM_ALT_STARMAP.0 as ::core::ffi::c_int
                    - C2Rust_Unnamed::PM_ALT_SCAN.0 as ::core::ffi::c_int
                    - adj as ::core::ffi::c_int) as BYTE;
                return BOOLEAN::TRUE;
            }
            2 => {
                *BaseState = C2Rust_Unnamed::PM_ALT_CARGO.0 as ::core::ffi::c_int
                    as BYTE;
                *CurState = (C2Rust_Unnamed::PM_ALT_DEVICES.0 as ::core::ffi::c_int
                    - C2Rust_Unnamed::PM_ALT_CARGO.0 as ::core::ffi::c_int) as BYTE;
                return BOOLEAN::TRUE;
            }
            3 => {
                *BaseState = C2Rust_Unnamed::PM_ALT_CARGO.0 as ::core::ffi::c_int
                    as BYTE;
                *CurState = (C2Rust_Unnamed::PM_ALT_CARGO.0 as ::core::ffi::c_int
                    - C2Rust_Unnamed::PM_ALT_CARGO.0 as ::core::ffi::c_int) as BYTE;
                return BOOLEAN::TRUE;
            }
            4 => {
                *BaseState = C2Rust_Unnamed::PM_ALT_CARGO.0 as ::core::ffi::c_int
                    as BYTE;
                *CurState = (C2Rust_Unnamed::PM_ALT_ROSTER.0 as ::core::ffi::c_int
                    - C2Rust_Unnamed::PM_ALT_CARGO.0 as ::core::ffi::c_int) as BYTE;
                return BOOLEAN::TRUE;
            }
            5 => {
                *BaseState = (C2Rust_Unnamed::PM_ALT_SCAN.0 as ::core::ffi::c_int
                    + adj as ::core::ffi::c_int) as BYTE;
                *CurState = (C2Rust_Unnamed::PM_ALT_GAME_MENU.0 as ::core::ffi::c_int
                    - C2Rust_Unnamed::PM_ALT_SCAN.0 as ::core::ffi::c_int
                    - adj as ::core::ffi::c_int) as BYTE;
                return BOOLEAN::TRUE;
            }
            6 => {
                *BaseState = (C2Rust_Unnamed::PM_ALT_SCAN.0 as ::core::ffi::c_int
                    + adj as ::core::ffi::c_int) as BYTE;
                *CurState = (C2Rust_Unnamed::PM_ALT_NAVIGATE.0 as ::core::ffi::c_int
                    - C2Rust_Unnamed::PM_ALT_SCAN.0 as ::core::ffi::c_int
                    - adj as ::core::ffi::c_int) as BYTE;
                return BOOLEAN::TRUE;
            }
            7 => {
                *BaseState = C2Rust_Unnamed::PM_ALT_MSCAN.0 as ::core::ffi::c_int
                    as BYTE;
                *CurState = (C2Rust_Unnamed::PM_ALT_MSCAN.0 as ::core::ffi::c_int
                    - C2Rust_Unnamed::PM_ALT_MSCAN.0 as ::core::ffi::c_int) as BYTE;
                return BOOLEAN::TRUE;
            }
            8 => {
                *BaseState = C2Rust_Unnamed::PM_ALT_MSCAN.0 as ::core::ffi::c_int
                    as BYTE;
                *CurState = (C2Rust_Unnamed::PM_ALT_ESCAN.0 as ::core::ffi::c_int
                    - C2Rust_Unnamed::PM_ALT_MSCAN.0 as ::core::ffi::c_int) as BYTE;
                return BOOLEAN::TRUE;
            }
            9 => {
                *BaseState = C2Rust_Unnamed::PM_ALT_MSCAN.0 as ::core::ffi::c_int
                    as BYTE;
                *CurState = (C2Rust_Unnamed::PM_ALT_BSCAN.0 as ::core::ffi::c_int
                    - C2Rust_Unnamed::PM_ALT_MSCAN.0 as ::core::ffi::c_int) as BYTE;
                return BOOLEAN::TRUE;
            }
            10 => {
                *BaseState = C2Rust_Unnamed::PM_ALT_MSCAN.0 as ::core::ffi::c_int
                    as BYTE;
                *CurState = (C2Rust_Unnamed::PM_ALT_EXIT_SCAN.0 as ::core::ffi::c_int
                    - C2Rust_Unnamed::PM_ALT_MSCAN.0 as ::core::ffi::c_int) as BYTE;
                return BOOLEAN::TRUE;
            }
            11 => {
                *BaseState = C2Rust_Unnamed::PM_ALT_MSCAN.0 as ::core::ffi::c_int
                    as BYTE;
                *CurState = (C2Rust_Unnamed::PM_ALT_ASCAN.0 as ::core::ffi::c_int
                    - C2Rust_Unnamed::PM_ALT_MSCAN.0 as ::core::ffi::c_int) as BYTE;
                return BOOLEAN::TRUE;
            }
            12 => {
                *BaseState = C2Rust_Unnamed::PM_ALT_MSCAN.0 as ::core::ffi::c_int
                    as BYTE;
                *CurState = (C2Rust_Unnamed::PM_ALT_DISPATCH.0 as ::core::ffi::c_int
                    - C2Rust_Unnamed::PM_ALT_MSCAN.0 as ::core::ffi::c_int) as BYTE;
                return BOOLEAN::TRUE;
            }
            _ => {}
        }
        return BOOLEAN::FALSE;
    };
}
unsafe extern "C" fn ConvertAlternateMenu(
    mut BaseState: BYTE,
    mut NewState: BYTE,
) -> BYTE {
    match BaseState as ::core::ffi::c_int + NewState as ::core::ffi::c_int {
        39 => {
            return (C2Rust_Unnamed::PM_SCAN.0 as ::core::ffi::c_int
                - C2Rust_Unnamed::PM_SCAN.0 as ::core::ffi::c_int) as BYTE;
        }
        40 => {
            return (C2Rust_Unnamed::PM_STARMAP.0 as ::core::ffi::c_int
                - C2Rust_Unnamed::PM_SCAN.0 as ::core::ffi::c_int) as BYTE;
        }
        41 => return 0x80 as BYTE,
        42 => {
            return (C2Rust_Unnamed::PM_GAME_MENU.0 as ::core::ffi::c_int
                - C2Rust_Unnamed::PM_SCAN.0 as ::core::ffi::c_int) as BYTE;
        }
        43 => {
            return (C2Rust_Unnamed::PM_NAVIGATE.0 as ::core::ffi::c_int
                - C2Rust_Unnamed::PM_SCAN.0 as ::core::ffi::c_int) as BYTE;
        }
        44 => {
            return (C2Rust_Unnamed::PM_CARGO.0 as ::core::ffi::c_int
                - C2Rust_Unnamed::PM_SCAN.0 as ::core::ffi::c_int) as BYTE;
        }
        45 => {
            return (C2Rust_Unnamed::PM_DEVICES.0 as ::core::ffi::c_int
                - C2Rust_Unnamed::PM_SCAN.0 as ::core::ffi::c_int) as BYTE;
        }
        46 => {
            return (C2Rust_Unnamed::PM_ROSTER.0 as ::core::ffi::c_int
                - C2Rust_Unnamed::PM_SCAN.0 as ::core::ffi::c_int) as BYTE;
        }
        47 => return 0x81 as BYTE,
        48 => {
            return (C2Rust_Unnamed::PM_MIN_SCAN.0 as ::core::ffi::c_int
                - C2Rust_Unnamed::PM_MIN_SCAN.0 as ::core::ffi::c_int) as BYTE;
        }
        49 => {
            return (C2Rust_Unnamed::PM_ENE_SCAN.0 as ::core::ffi::c_int
                - C2Rust_Unnamed::PM_MIN_SCAN.0 as ::core::ffi::c_int) as BYTE;
        }
        50 => {
            return (C2Rust_Unnamed::PM_BIO_SCAN.0 as ::core::ffi::c_int
                - C2Rust_Unnamed::PM_MIN_SCAN.0 as ::core::ffi::c_int) as BYTE;
        }
        51 => {
            return (C2Rust_Unnamed::PM_AUTO_SCAN.0 as ::core::ffi::c_int
                - C2Rust_Unnamed::PM_MIN_SCAN.0 as ::core::ffi::c_int) as BYTE;
        }
        52 => {
            return (C2Rust_Unnamed::PM_LAUNCH_LANDER.0 as ::core::ffi::c_int
                - C2Rust_Unnamed::PM_MIN_SCAN.0 as ::core::ffi::c_int) as BYTE;
        }
        53 => {
            return (C2Rust_Unnamed::PM_EXIT_SCAN.0 as ::core::ffi::c_int
                - C2Rust_Unnamed::PM_MIN_SCAN.0 as ::core::ffi::c_int) as BYTE;
        }
        _ => {}
    }
    return NewState;
}
#[no_mangle]
pub unsafe extern "C" fn DoMenuChooser(
    mut pMS: *mut MENU_STATE,
    mut BaseState: BYTE,
) -> BOOLEAN {
    let mut NewState: BYTE = (*pMS).CurState;
    let mut OrigBase: BYTE = BaseState;
    let mut useAltMenu: BOOLEAN = BOOLEAN::FALSE;
    if optWhichMenu == 0x2 as ::core::ffi::c_int {
        useAltMenu = GetAlternateMenu(&raw mut BaseState, &raw mut NewState);
    }
    if PulsedInputState
        .menu[C2Rust_Unnamed_0::KEY_MENU_LEFT.0 as ::core::ffi::c_int as usize] != 0
        || PulsedInputState
            .menu[C2Rust_Unnamed_0::KEY_MENU_UP.0 as ::core::ffi::c_int as usize] != 0
    {
        NewState = PreviousMenuState(BaseState, NewState);
    } else if PulsedInputState
        .menu[C2Rust_Unnamed_0::KEY_MENU_RIGHT.0 as ::core::ffi::c_int as usize] != 0
        || PulsedInputState
            .menu[C2Rust_Unnamed_0::KEY_MENU_DOWN.0 as ::core::ffi::c_int as usize] != 0
    {
        NewState = NextMenuState(BaseState, NewState);
    } else if useAltMenu.0 != 0
        && PulsedInputState
            .menu[C2Rust_Unnamed_0::KEY_MENU_SELECT.0 as ::core::ffi::c_int as usize]
            != 0
    {
        NewState = ConvertAlternateMenu(BaseState, NewState);
        if NewState as ::core::ffi::c_int == 0x80 as ::core::ffi::c_int {
            DrawMenuStateStrings(
                C2Rust_Unnamed::PM_ALT_CARGO.0 as ::core::ffi::c_int as BYTE,
                0 as SWORD,
            );
            (*pMS).CurState = (C2Rust_Unnamed::PM_CARGO.0 as ::core::ffi::c_int
                - C2Rust_Unnamed::PM_SCAN.0 as ::core::ffi::c_int) as BYTE;
            return BOOLEAN::TRUE;
        }
        if NewState as ::core::ffi::c_int == 0x81 as ::core::ffi::c_int {
            if OrigBase as ::core::ffi::c_int
                == C2Rust_Unnamed::PM_SCAN.0 as ::core::ffi::c_int
            {
                DrawMenuStateStrings(
                    C2Rust_Unnamed::PM_ALT_SCAN.0 as ::core::ffi::c_int as BYTE,
                    (C2Rust_Unnamed::PM_ALT_MANIFEST.0 as ::core::ffi::c_int
                        - C2Rust_Unnamed::PM_ALT_SCAN.0 as ::core::ffi::c_int) as SWORD,
                );
            } else {
                DrawMenuStateStrings(
                    C2Rust_Unnamed::PM_ALT_STARMAP.0 as ::core::ffi::c_int as BYTE,
                    (C2Rust_Unnamed::PM_ALT_MANIFEST.0 as ::core::ffi::c_int
                        - C2Rust_Unnamed::PM_ALT_STARMAP.0 as ::core::ffi::c_int)
                        as SWORD,
                );
            }
            (*pMS).CurState = 0x80 as BYTE;
            return BOOLEAN::TRUE;
        }
        return BOOLEAN::FALSE;
    } else if optWhichMenu == 0x2 as ::core::ffi::c_int
        && PulsedInputState
            .menu[C2Rust_Unnamed_0::KEY_MENU_CANCEL.0 as ::core::ffi::c_int as usize]
            != 0
        && BaseState as ::core::ffi::c_int
            == C2Rust_Unnamed::PM_ALT_CARGO.0 as ::core::ffi::c_int
    {
        if OrigBase as ::core::ffi::c_int
            == C2Rust_Unnamed::PM_SCAN.0 as ::core::ffi::c_int
        {
            DrawMenuStateStrings(
                C2Rust_Unnamed::PM_ALT_SCAN.0 as ::core::ffi::c_int as BYTE,
                (C2Rust_Unnamed::PM_ALT_MANIFEST.0 as ::core::ffi::c_int
                    - C2Rust_Unnamed::PM_ALT_SCAN.0 as ::core::ffi::c_int) as SWORD,
            );
        } else {
            DrawMenuStateStrings(
                C2Rust_Unnamed::PM_ALT_STARMAP.0 as ::core::ffi::c_int as BYTE,
                (C2Rust_Unnamed::PM_ALT_MANIFEST.0 as ::core::ffi::c_int
                    - C2Rust_Unnamed::PM_ALT_STARMAP.0 as ::core::ffi::c_int) as SWORD,
            );
        }
        (*pMS).CurState = 0x80 as BYTE;
        return BOOLEAN::TRUE;
    } else {
        return BOOLEAN::FALSE
    }
    DrawMenuStateStrings(BaseState, NewState as SWORD);
    if useAltMenu.0 != 0 {
        NewState = ConvertAlternateMenu(BaseState, NewState);
    }
    (*pMS).CurState = NewState;
    SleepThread((840 as ::core::ffi::c_int / 20 as ::core::ffi::c_int) as TimePeriod);
    return BOOLEAN::TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn DrawMenuStateStrings(mut beg_index: BYTE, mut NewState: SWORD) {
    let mut end_index: BYTE = 0;
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut s: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    let mut OldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    let mut hilite: BYTE = 1 as BYTE;
    extern "C" {
        static mut PlayFrame: FRAME;
    }
    if (NewState as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
        NewState = -(NewState as ::core::ffi::c_int) as SWORD;
        hilite = 0 as BYTE;
    }
    if optWhichMenu == 0x2 as ::core::ffi::c_int {
        let mut tmpState: BYTE = NewState as BYTE;
        GetAlternateMenu(&raw mut beg_index, &raw mut tmpState);
        NewState = tmpState as SWORD;
    }
    if beg_index as ::core::ffi::c_int
        == C2Rust_Unnamed::PM_STARMAP.0 as ::core::ffi::c_int
    {
        NewState -= 1;
    }
    end_index = GetEndMenuState(beg_index);
    s.frame = ::core::ptr::null_mut::<FRAME_DESC>();
    if NewState as ::core::ffi::c_int
        <= end_index as ::core::ffi::c_int - beg_index as ::core::ffi::c_int
    {
        s.frame = SetAbsFrameIndex(
            PlayFrame,
            (beg_index as ::core::ffi::c_int + NewState as ::core::ffi::c_int) as COUNT,
        );
    }
    PreUpdateFlashRect();
    OldContext = SetContext(StatusContext);
    GetContextClipRect(&raw mut r);
    s.origin.x = (4 as ::core::ffi::c_int
        + (ScreenWidth - 64 as ::core::ffi::c_int - 0 as ::core::ffi::c_int)
        - r.corner.x as ::core::ffi::c_int) as COORD;
    s.origin.y = (10 as ::core::ffi::c_int + 0 as ::core::ffi::c_int
        + (ScreenHeight - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
            - 13 as ::core::ffi::c_int) - 53 as ::core::ffi::c_int
        - r.corner.y as ::core::ffi::c_int) as COORD;
    r.corner.x = (s.origin.x as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as COORD;
    r.corner.y = (s.origin.y as ::core::ffi::c_int - 11 as ::core::ffi::c_int) as COORD;
    r.extent.width = (64 as ::core::ffi::c_int - 8 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as COORD;
    BatchGraphics();
    SetContextForeGroundColor(colorFromRgb15(0xa as BYTE, 0xa as BYTE, 0xa as BYTE));
    if !s.frame.is_null() && optWhichMenu == 0x2 as ::core::ffi::c_int {
        if beg_index as ::core::ffi::c_int
            == C2Rust_Unnamed::PM_CREW.0 as ::core::ffi::c_int
        {
            snprintf(
                &raw mut pm_crew_str as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[UNICODE; 128]>(),
                b"%s(%d)\0".as_ptr() as *const ::core::ffi::c_char,
                GetStringAddress(
                    SetAbsStringTableIndex(
                        GameStrings,
                        (C2Rust_Unnamed_1::MAINMENU_STRING_BASE.0 as ::core::ffi::c_int
                            + C2Rust_Unnamed::PM_CREW.0 as ::core::ffi::c_int) as COUNT,
                    ),
                ),
                GlobData.Game_state.CrewCost as ::core::ffi::c_int,
            );
        }
        if beg_index as ::core::ffi::c_int
            == C2Rust_Unnamed::PM_FUEL.0 as ::core::ffi::c_int
        {
            snprintf(
                &raw mut pm_fuel_str as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[UNICODE; 128]>(),
                b"%s(%d)\0".as_ptr() as *const ::core::ffi::c_char,
                GetStringAddress(
                    SetAbsStringTableIndex(
                        GameStrings,
                        (C2Rust_Unnamed_1::MAINMENU_STRING_BASE.0 as ::core::ffi::c_int
                            + C2Rust_Unnamed::PM_FUEL.0 as ::core::ffi::c_int) as COUNT,
                    ),
                ),
                GlobData.Game_state.FuelCost as ::core::ffi::c_int,
            );
        }
        if beg_index as ::core::ffi::c_int
            == C2Rust_Unnamed::PM_SOUND_ON.0 as ::core::ffi::c_int
        {
            end_index = (beg_index as ::core::ffi::c_int + 5 as ::core::ffi::c_int)
                as BYTE;
            match beg_index as ::core::ffi::c_int + NewState as ::core::ffi::c_int {
                28 | 29 => {
                    NewState = 0 as SWORD;
                }
                30 | 31 => {
                    NewState = 1 as SWORD;
                }
                32 | 33 | 34 | 35 => {
                    NewState = 2 as SWORD;
                }
                36 => {
                    NewState = 3 as SWORD;
                }
                37 => {
                    NewState = 4 as SWORD;
                }
                38 => {
                    NewState = 5 as SWORD;
                }
                _ => {}
            }
        }
        r.extent.height = (53 as ::core::ffi::c_int + 11 as ::core::ffi::c_int) as COORD;
        DrawPCMenu(beg_index, end_index, NewState as BYTE, hilite, &raw mut r);
        s.frame = ::core::ptr::null_mut::<FRAME_DESC>();
    } else {
        if optWhichMenu == 0x2 as ::core::ffi::c_int {
            r.corner.x = (r.corner.x as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                as COORD;
            r.extent.width = (r.extent.width as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int) as COORD;
            r.extent.height = (53 as ::core::ffi::c_int + 11 as ::core::ffi::c_int)
                as COORD;
        } else {
            r.extent.height = 11 as COORD;
        }
        DrawFilledRectangle(&raw mut r);
    }
    if !s.frame.is_null() {
        DrawStamp(&raw mut s);
        let mut t: TEXT = TEXT {
            baseline: POINT { x: 0, y: 0 },
            pStr: ::core::ptr::null::<UNICODE>(),
            align: TEXT_ALIGN::ALIGN_LEFT,
            CharCount: 0,
        };
        let mut buf: [UNICODE; 20] = [0; 20];
        match beg_index as ::core::ffi::c_int + NewState as ::core::ffi::c_int {
            25 => {
                t.baseline.x = (s.origin.x as ::core::ffi::c_int
                    + (64 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)
                    - 2 as ::core::ffi::c_int) as COORD;
                t.baseline.y = (s.origin.y as ::core::ffi::c_int
                    + 53 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as COORD;
                t.align = TEXT_ALIGN::ALIGN_RIGHT;
                t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
                t.pStr = &raw mut buf as *mut UNICODE;
                snprintf(
                    &raw mut buf as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[UNICODE; 20]>(),
                    b"%u\0".as_ptr() as *const ::core::ffi::c_char,
                    GlobData.Game_state.CrewCost as ::core::ffi::c_int,
                );
                SetContextFont(TinyFont);
                SetContextForeGroundColor(
                    colorFromRgb15(0 as BYTE, 0x1f as BYTE, 0 as BYTE),
                );
                font_DrawText(&raw mut t);
            }
            21 => {
                t.baseline.x = (s.origin.x as ::core::ffi::c_int
                    + (64 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)
                    - 2 as ::core::ffi::c_int) as COORD;
                t.baseline.y = (s.origin.y as ::core::ffi::c_int
                    + 53 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as COORD;
                t.align = TEXT_ALIGN::ALIGN_RIGHT;
                t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
                t.pStr = &raw mut buf as *mut UNICODE;
                snprintf(
                    &raw mut buf as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[UNICODE; 20]>(),
                    b"%u\0".as_ptr() as *const ::core::ffi::c_char,
                    GlobData.Game_state.FuelCost as ::core::ffi::c_int,
                );
                SetContextFont(TinyFont);
                SetContextForeGroundColor(
                    colorFromRgb15(0 as BYTE, 0x1f as BYTE, 0 as BYTE),
                );
                font_DrawText(&raw mut t);
            }
            _ => {}
        }
    }
    UnbatchGraphics();
    SetContext(OldContext);
    PostUpdateFlashRect();
}
