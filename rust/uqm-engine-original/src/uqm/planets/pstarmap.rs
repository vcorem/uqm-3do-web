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

// Rust translation of src/uqm/planets/pstarmap.c.
extern "C" {
    pub type string_table_entry;
    pub type context_desc;
    pub type frame_desc;
    pub type font_desc;
    pub type tfb_soundsample;
    pub type FlashContext;
    pub type joy_char;
    fn HMalloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn HFree(p: *mut ::core::ffi::c_void);
    fn __assert_fail(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    );
    fn sprintf(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn getCharFromString(ptr: *mut *const ::core::ffi::c_char) -> UniChar;
    fn utf8StringCount(start: *const ::core::ffi::c_char) -> size_t;
    fn utf8StringCountN(
        start: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
    ) -> size_t;
    fn utf8StringCopy(
        dst: *mut ::core::ffi::c_char,
        size: size_t,
        src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_uchar;
    fn utf8StringCompare(
        str1: *const ::core::ffi::c_char,
        str2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn skipUTF8Chars(
        ptr: *const ::core::ffi::c_char,
        num: size_t,
    ) -> *mut ::core::ffi::c_char;
    fn UniChar_toUpper(ch: UniChar) -> UniChar;
    fn SetAbsStringTableIndex(String: STRING, StringTableIndex: COUNT) -> STRING;
    fn GetStringLength(String: STRING) -> COUNT;
    fn GetStringAddress(String: STRING) -> STRINGPTR;
    fn square_root(value: DWORD) -> COUNT;
    fn DrawOval(pRect: *mut RECT, num_off_pixels: BYTE);
    fn DrawFilledOval(pRect: *mut RECT);
    fn BoxUnion(pr1: *mut RECT, pr2: *mut RECT, punion: *mut RECT);
    fn SetContext(Context: CONTEXT) -> CONTEXT;
    fn SetContextForeGroundColor(Color: Color) -> Color;
    fn SetContextBackGroundColor(Color: Color) -> Color;
    fn SetContextClipRect(pRect: *mut RECT) -> BOOLEAN;
    fn GetContextClipRect(pRect: *mut RECT) -> BOOLEAN;
    fn SetContextOrigin(orgOffset: POINT) -> POINT;
    fn DrawStamp(pStamp: *mut STAMP);
    fn DrawFilledStamp(pStamp: *mut STAMP);
    fn DrawPoint(pPoint: *mut POINT);
    fn DrawFilledRectangle(pRect: *mut RECT);
    fn font_DrawText(pText: *mut TEXT);
    fn BatchGraphics();
    fn UnbatchGraphics();
    fn ClearDrawable();
    fn GetFrameRect(Frame: FRAME, pRect: *mut RECT) -> BOOLEAN;
    fn SetContextFont(Font: FONT) -> FONT;
    fn TextRect(pText: *mut TEXT, pRect: *mut RECT, pdelta: *mut BYTE) -> BOOLEAN;
    fn SetAbsFrameIndex(Frame: FRAME, FrameIndex: COUNT) -> FRAME;
    fn SetRelFrameIndex(Frame: FRAME, FrameOffs: SIZE) -> FRAME;
    fn IncFrameIndex(Frame: FRAME) -> FRAME;
    fn GetTimeCounter() -> TimeCount;
    static mut PulsedInputState: CONTROLLER_INPUT_STATE;
    fn FlushInput();
    fn SetMenuRepeatDelay(min: DWORD, max: DWORD, step: DWORD, gestalt: BOOLEAN);
    fn SetDefaultMenuRepeatDelay();
    fn DoInput(pInputState: *mut ::core::ffi::c_void, resetInput: BOOLEAN);
    fn DoTextEntry(pTES: *mut TEXTENTRY_STATE) -> BOOLEAN;
    static mut CurStarDescPtr: *mut STAR_DESC;
    static mut star_array: *mut STAR_DESC;
    fn FindStar(
        pLastStar: *mut STAR_DESC,
        puniverse: *mut POINT,
        xbounds: SIZE,
        ybounds: SIZE,
    ) -> *mut STAR_DESC;
    fn GetClusterName(pSD: *const STAR_DESC, buf: *mut UNICODE);
    static mut ScreenWidth: ::core::ffi::c_int;
    static mut ScreenHeight: ::core::ffi::c_int;
    fn RepairBackRect(pRect: *mut RECT);
    fn memset(
        _: *mut ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strcpy(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strcmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn DrawHyperCoords(puniverse: POINT);
    fn DrawSISMessageEx(
        pStr: *const UNICODE,
        CurPos: SIZE,
        ExPos: SIZE,
        flags: COUNT,
    ) -> BOOLEAN;
    fn DrawSISMessage(pStr: *const UNICODE);
    fn DrawStatusMessage(pStr: *const UNICODE);
    static mut GlobData: GLOBDATA;
    fn getGameState(
        state: *mut BYTE,
        startBit: ::core::ffi::c_int,
        endBit: ::core::ffi::c_int,
    ) -> BYTE;
    fn inHQSpace() -> BOOLEAN;
    fn SleepThread(timePeriod: TimePeriod);
    fn SleepThreadUntil(wakeTime: TimeCount);
    static mut GameStrings: STRING;
    static mut MiscDataFrame: FRAME;
    static mut SpaceContext: CONTEXT;
    static mut TinyFont: FONT;
    fn SetMenuSounds(sound_0: MENU_SOUND_FLAGS, sound_1: MENU_SOUND_FLAGS);
    fn PlayMenuSound(S: MENU_SOUND_EFFECT);
    static mut debugHook: Option<unsafe extern "C" fn() -> ()>;
    fn doInstantMove();
    static mut instantMove: BOOLEAN;
    fn qsort(
        _: *mut ::core::ffi::c_void,
        _: size_t,
        _: size_t,
        _: Option<
            unsafe extern "C" fn(
                *const ::core::ffi::c_void,
                *const ::core::ffi::c_void,
            ) -> ::core::ffi::c_int,
        >,
    );
    fn LoadIntoExtraScreen(r: *mut RECT);
    fn SetTransitionSource(pRect: *const RECT);
    fn ScreenTransition(transition: ::core::ffi::c_int, pRect: *const RECT);
    fn AnyButtonPress(DetectSpecial: BOOLEAN) -> BOOLEAN;
}
pub type COUNT = UWORD;
pub type UWORD = uint16;
pub type uint16 = ::core::ffi::c_ushort;
pub type POINT = point;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct point {
    pub x: COORD,
    pub y: COORD,
}
pub type COORD = SWORD;
pub type SWORD = sint16;
pub type sint16 = ::core::ffi::c_short;
pub type uint8 = ::core::ffi::c_uchar;
pub type sint8 = ::core::ffi::c_schar;
pub type uint32 = ::core::ffi::c_uint;
pub type sint32 = ::core::ffi::c_int;
pub type BYTE = uint8;
pub type SBYTE = sint8;
pub type DWORD = uint32;
pub type SDWORD = sint32;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Color {
    pub r: BYTE,
    pub g: BYTE,
    pub b: BYTE,
    pub a: BYTE,
}
pub type UniChar = uint32;
pub type STRING_TABLE_ENTRY_DESC = string_table_entry;
pub type STRING = *mut STRING_TABLE_ENTRY_DESC;
pub type STRINGPTR = *mut ::core::ffi::c_char;
pub type CONTEXT_DESC = context_desc;
pub type FRAME_DESC = frame_desc;
pub type FONT_DESC = font_desc;
pub type CONTEXT = *mut CONTEXT_DESC;
pub type FRAME = *mut FRAME_DESC;
pub type FONT = *mut FONT_DESC;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct extent {
    pub width: COORD,
    pub height: COORD,
}
pub type EXTENT = extent;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct star_desc {
    pub star_pt: POINT,
    pub Type: BYTE,
    pub Index: BYTE,
    pub Prefix: BYTE,
    pub Postfix: BYTE,
}
pub type STAR_DESC = star_desc;
pub type TFB_SoundSample = tfb_soundsample;
pub type MUSIC_REF = *mut *mut TFB_SoundSample;
pub type TimeCount = DWORD;
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
    pub const DWARF_STAR: Self = Self(0);
    pub const GIANT_STAR: Self = Self(1);
    pub const SUPER_GIANT_STAR: Self = Self(2);
    pub const NUM_STAR_TYPES: Self = Self(3);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_0 {
    pub const BLUE_BODY: Self = Self(0);
    pub const GREEN_BODY: Self = Self(1);
    pub const ORANGE_BODY: Self = Self(2);
    pub const RED_BODY: Self = Self(3);
    pub const WHITE_BODY: Self = Self(4);
    pub const GRAY_BODY: Self = Self(4);
    pub const YELLOW_BODY: Self = Self(5);
    pub const NUM_STAR_COLORS: Self = Self(6);
    pub const CYAN_BODY: Self = Self(6);
    pub const PURPLE_BODY: Self = Self(7);
    pub const VIOLET_BODY: Self = Self(8);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_1(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_1 {
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
pub type QUEUE_HANDLE = *mut ::core::ffi::c_void;
pub type HLINK = QUEUE_HANDLE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct link {
    pub pred: HLINK,
    pub succ: HLINK,
}
pub type LINK = link;
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct SPECIES_ID(pub ::core::ffi::c_uint);
impl SPECIES_ID {
    pub const NO_ID: Self = Self(0);
    pub const ARILOU_ID: Self = Self(1);
    pub const CHMMR_ID: Self = Self(2);
    pub const EARTHLING_ID: Self = Self(3);
    pub const ORZ_ID: Self = Self(4);
    pub const PKUNK_ID: Self = Self(5);
    pub const SHOFIXTI_ID: Self = Self(6);
    pub const SPATHI_ID: Self = Self(7);
    pub const SUPOX_ID: Self = Self(8);
    pub const THRADDASH_ID: Self = Self(9);
    pub const UTWIG_ID: Self = Self(10);
    pub const VUX_ID: Self = Self(11);
    pub const YEHAT_ID: Self = Self(12);
    pub const MELNORME_ID: Self = Self(13);
    pub const DRUUGE_ID: Self = Self(14);
    pub const ILWRATH_ID: Self = Self(15);
    pub const MYCON_ID: Self = Self(16);
    pub const SLYLANDRO_ID: Self = Self(17);
    pub const UMGAH_ID: Self = Self(18);
    pub const UR_QUAN_ID: Self = Self(19);
    pub const ZOQFOTPIK_ID: Self = Self(20);
    pub const SYREEN_ID: Self = Self(21);
    pub const KOHR_AH_ID: Self = Self(22);
    pub const ANDROSYNTH_ID: Self = Self(23);
    pub const CHENJESU_ID: Self = Self(24);
    pub const MMRNMHRM_ID: Self = Self(25);
    pub const LAST_MELEE_ID: Self = Self(25);
    pub const SIS_SHIP_ID: Self = Self(26);
    pub const SA_MATRA_ID: Self = Self(27);
    pub const UR_QUAN_PROBE_ID: Self = Self(28);
    pub const NUM_SPECIES_ID: Self = Self(29);
}
pub type HFLEETINFO = HLINK;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FLEET_INFO {
    pub pred: HFLEETINFO,
    pub succ: HFLEETINFO,
    pub SpeciesID: SPECIES_ID,
    pub allied_state: UWORD,
    pub days_left: BYTE,
    pub growth_fract: BYTE,
    pub crew_level: COUNT,
    pub max_crew: COUNT,
    pub growth: BYTE,
    pub max_energy: BYTE,
    pub loc: POINT,
    pub race_strings: STRING,
    pub icons: FRAME,
    pub melee_icon: FRAME,
    pub actual_strength: COUNT,
    pub known_strength: COUNT,
    pub known_loc: POINT,
    pub growth_err_term: BYTE,
    pub func_index: BYTE,
    pub dest_loc: POINT,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_2(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_2 {
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
pub struct C2Rust_Unnamed_3(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_3 {
    pub const SHOFIXTI_VISITS: Self = Self(0);
    pub const END_SHOFIXTI_VISITS: Self = Self(2);
    pub const SHOFIXTI_STACK1: Self = Self(3);
    pub const END_SHOFIXTI_STACK1: Self = Self(4);
    pub const SHOFIXTI_STACK2: Self = Self(5);
    pub const END_SHOFIXTI_STACK2: Self = Self(7);
    pub const SHOFIXTI_STACK3: Self = Self(8);
    pub const END_SHOFIXTI_STACK3: Self = Self(9);
    pub const SHOFIXTI_KIA: Self = Self(10);
    pub const END_SHOFIXTI_KIA: Self = Self(10);
    pub const SHOFIXTI_BRO_KIA: Self = Self(11);
    pub const END_SHOFIXTI_BRO_KIA: Self = Self(11);
    pub const SHOFIXTI_RECRUITED: Self = Self(12);
    pub const END_SHOFIXTI_RECRUITED: Self = Self(12);
    pub const SHOFIXTI_MAIDENS: Self = Self(13);
    pub const END_SHOFIXTI_MAIDENS: Self = Self(13);
    pub const MAIDENS_ON_SHIP: Self = Self(14);
    pub const END_MAIDENS_ON_SHIP: Self = Self(14);
    pub const BATTLE_SEGUE: Self = Self(15);
    pub const END_BATTLE_SEGUE: Self = Self(15);
    pub const PLANETARY_LANDING: Self = Self(16);
    pub const END_PLANETARY_LANDING: Self = Self(16);
    pub const PLANETARY_CHANGE: Self = Self(17);
    pub const END_PLANETARY_CHANGE: Self = Self(17);
    pub const SPATHI_VISITS: Self = Self(18);
    pub const END_SPATHI_VISITS: Self = Self(20);
    pub const SPATHI_HOME_VISITS: Self = Self(21);
    pub const END_SPATHI_HOME_VISITS: Self = Self(23);
    pub const FOUND_PLUTO_SPATHI: Self = Self(24);
    pub const END_FOUND_PLUTO_SPATHI: Self = Self(25);
    pub const SPATHI_SHIELDED_SELVES: Self = Self(26);
    pub const END_SPATHI_SHIELDED_SELVES: Self = Self(26);
    pub const SPATHI_CREATURES_EXAMINED: Self = Self(27);
    pub const END_SPATHI_CREATURES_EXAMINED: Self = Self(27);
    pub const SPATHI_CREATURES_ELIMINATED: Self = Self(28);
    pub const END_SPATHI_CREATURES_ELIMINATED: Self = Self(28);
    pub const UMGAH_BROADCASTERS: Self = Self(29);
    pub const END_UMGAH_BROADCASTERS: Self = Self(29);
    pub const SPATHI_MANNER: Self = Self(30);
    pub const END_SPATHI_MANNER: Self = Self(31);
    pub const SPATHI_QUEST: Self = Self(32);
    pub const END_SPATHI_QUEST: Self = Self(32);
    pub const LIED_ABOUT_CREATURES: Self = Self(33);
    pub const END_LIED_ABOUT_CREATURES: Self = Self(34);
    pub const SPATHI_PARTY: Self = Self(35);
    pub const END_SPATHI_PARTY: Self = Self(35);
    pub const KNOW_SPATHI_PASSWORD: Self = Self(36);
    pub const END_KNOW_SPATHI_PASSWORD: Self = Self(36);
    pub const ILWRATH_HOME_VISITS: Self = Self(37);
    pub const END_ILWRATH_HOME_VISITS: Self = Self(39);
    pub const ILWRATH_CHMMR_VISITS: Self = Self(40);
    pub const END_ILWRATH_CHMMR_VISITS: Self = Self(40);
    pub const ARILOU_SPACE: Self = Self(41);
    pub const END_ARILOU_SPACE: Self = Self(41);
    pub const ARILOU_SPACE_SIDE: Self = Self(42);
    pub const END_ARILOU_SPACE_SIDE: Self = Self(43);
    pub const ARILOU_SPACE_COUNTER: Self = Self(44);
    pub const END_ARILOU_SPACE_COUNTER: Self = Self(47);
    pub const LANDER_SHIELDS: Self = Self(48);
    pub const END_LANDER_SHIELDS: Self = Self(51);
    pub const MET_MELNORME: Self = Self(52);
    pub const END_MET_MELNORME: Self = Self(52);
    pub const MELNORME_RESCUE_REFUSED: Self = Self(53);
    pub const END_MELNORME_RESCUE_REFUSED: Self = Self(53);
    pub const MELNORME_RESCUE_COUNT: Self = Self(54);
    pub const END_MELNORME_RESCUE_COUNT: Self = Self(56);
    pub const TRADED_WITH_MELNORME: Self = Self(57);
    pub const END_TRADED_WITH_MELNORME: Self = Self(57);
    pub const WHY_MELNORME_PURPLE: Self = Self(58);
    pub const END_WHY_MELNORME_PURPLE: Self = Self(58);
    pub const MELNORME_CREDIT0: Self = Self(59);
    pub const END_MELNORME_CREDIT0: Self = Self(66);
    pub const MELNORME_CREDIT1: Self = Self(67);
    pub const END_MELNORME_CREDIT1: Self = Self(74);
    pub const MELNORME_BUSINESS_COUNT: Self = Self(75);
    pub const END_MELNORME_BUSINESS_COUNT: Self = Self(76);
    pub const MELNORME_YACK_STACK0: Self = Self(77);
    pub const END_MELNORME_YACK_STACK0: Self = Self(78);
    pub const MELNORME_YACK_STACK1: Self = Self(79);
    pub const END_MELNORME_YACK_STACK1: Self = Self(80);
    pub const MELNORME_YACK_STACK2: Self = Self(81);
    pub const END_MELNORME_YACK_STACK2: Self = Self(84);
    pub const MELNORME_YACK_STACK3: Self = Self(85);
    pub const END_MELNORME_YACK_STACK3: Self = Self(87);
    pub const MELNORME_YACK_STACK4: Self = Self(88);
    pub const END_MELNORME_YACK_STACK4: Self = Self(89);
    pub const WHY_MELNORME_BLUE: Self = Self(90);
    pub const END_WHY_MELNORME_BLUE: Self = Self(90);
    pub const MELNORME_ANGER: Self = Self(91);
    pub const END_MELNORME_ANGER: Self = Self(92);
    pub const MELNORME_MIFFED_COUNT: Self = Self(93);
    pub const END_MELNORME_MIFFED_COUNT: Self = Self(94);
    pub const MELNORME_PISSED_COUNT: Self = Self(95);
    pub const END_MELNORME_PISSED_COUNT: Self = Self(96);
    pub const MELNORME_HATE_COUNT: Self = Self(97);
    pub const END_MELNORME_HATE_COUNT: Self = Self(98);
    pub const PROBE_MESSAGE_DELIVERED: Self = Self(99);
    pub const END_PROBE_MESSAGE_DELIVERED: Self = Self(99);
    pub const PROBE_ILWRATH_ENCOUNTER: Self = Self(100);
    pub const END_PROBE_ILWRATH_ENCOUNTER: Self = Self(100);
    pub const STARBASE_AVAILABLE: Self = Self(101);
    pub const END_STARBASE_AVAILABLE: Self = Self(101);
    pub const STARBASE_VISITED: Self = Self(102);
    pub const END_STARBASE_VISITED: Self = Self(102);
    pub const RADIOACTIVES_PROVIDED: Self = Self(103);
    pub const END_RADIOACTIVES_PROVIDED: Self = Self(103);
    pub const LANDERS_LOST: Self = Self(104);
    pub const END_LANDERS_LOST: Self = Self(104);
    pub const GIVEN_FUEL_BEFORE: Self = Self(105);
    pub const END_GIVEN_FUEL_BEFORE: Self = Self(105);
    pub const AWARE_OF_SAMATRA: Self = Self(106);
    pub const END_AWARE_OF_SAMATRA: Self = Self(106);
    pub const YEHAT_CAVALRY_ARRIVED: Self = Self(107);
    pub const END_YEHAT_CAVALRY_ARRIVED: Self = Self(107);
    pub const URQUAN_MESSED_UP: Self = Self(108);
    pub const END_URQUAN_MESSED_UP: Self = Self(108);
    pub const MOONBASE_DESTROYED: Self = Self(109);
    pub const END_MOONBASE_DESTROYED: Self = Self(109);
    pub const WILL_DESTROY_BASE: Self = Self(110);
    pub const END_WILL_DESTROY_BASE: Self = Self(110);
    pub const ARTIFACT_2_ON_SHIP: Self = Self(111);
    pub const END_ARTIFACT_2_ON_SHIP: Self = Self(111);
    pub const ARTIFACT_3_ON_SHIP: Self = Self(112);
    pub const END_ARTIFACT_3_ON_SHIP: Self = Self(112);
    pub const KOHR_AH_KILLED_ALL: Self = Self(113);
    pub const END_KOHR_AH_KILLED_ALL: Self = Self(113);
    pub const STARBASE_YACK_STACK1: Self = Self(114);
    pub const END_STARBASE_YACK_STACK1: Self = Self(114);
    pub const DISCUSSED_PORTAL_SPAWNER: Self = Self(115);
    pub const END_DISCUSSED_PORTAL_SPAWNER: Self = Self(115);
    pub const DISCUSSED_TALKING_PET: Self = Self(116);
    pub const END_DISCUSSED_TALKING_PET: Self = Self(116);
    pub const DISCUSSED_UTWIG_BOMB: Self = Self(117);
    pub const END_DISCUSSED_UTWIG_BOMB: Self = Self(117);
    pub const DISCUSSED_SUN_EFFICIENCY: Self = Self(118);
    pub const END_DISCUSSED_SUN_EFFICIENCY: Self = Self(118);
    pub const DISCUSSED_ROSY_SPHERE: Self = Self(119);
    pub const END_DISCUSSED_ROSY_SPHERE: Self = Self(119);
    pub const DISCUSSED_AQUA_HELIX: Self = Self(120);
    pub const END_DISCUSSED_AQUA_HELIX: Self = Self(120);
    pub const DISCUSSED_CLEAR_SPINDLE: Self = Self(121);
    pub const END_DISCUSSED_CLEAR_SPINDLE: Self = Self(121);
    pub const DISCUSSED_ULTRON: Self = Self(122);
    pub const END_DISCUSSED_ULTRON: Self = Self(122);
    pub const DISCUSSED_MAIDENS: Self = Self(123);
    pub const END_DISCUSSED_MAIDENS: Self = Self(123);
    pub const DISCUSSED_UMGAH_HYPERWAVE: Self = Self(124);
    pub const END_DISCUSSED_UMGAH_HYPERWAVE: Self = Self(124);
    pub const DISCUSSED_BURVIX_HYPERWAVE: Self = Self(125);
    pub const END_DISCUSSED_BURVIX_HYPERWAVE: Self = Self(125);
    pub const SYREEN_WANT_PROOF: Self = Self(126);
    pub const END_SYREEN_WANT_PROOF: Self = Self(126);
    pub const PLAYER_HAVING_SEX: Self = Self(127);
    pub const END_PLAYER_HAVING_SEX: Self = Self(127);
    pub const MET_ARILOU: Self = Self(128);
    pub const END_MET_ARILOU: Self = Self(128);
    pub const DISCUSSED_TAALO_PROTECTOR: Self = Self(129);
    pub const END_DISCUSSED_TAALO_PROTECTOR: Self = Self(129);
    pub const DISCUSSED_EGG_CASING0: Self = Self(130);
    pub const END_DISCUSSED_EGG_CASING0: Self = Self(130);
    pub const DISCUSSED_EGG_CASING1: Self = Self(131);
    pub const END_DISCUSSED_EGG_CASING1: Self = Self(131);
    pub const DISCUSSED_EGG_CASING2: Self = Self(132);
    pub const END_DISCUSSED_EGG_CASING2: Self = Self(132);
    pub const DISCUSSED_SYREEN_SHUTTLE: Self = Self(133);
    pub const END_DISCUSSED_SYREEN_SHUTTLE: Self = Self(133);
    pub const DISCUSSED_VUX_BEAST: Self = Self(134);
    pub const END_DISCUSSED_VUX_BEAST: Self = Self(134);
    pub const DISCUSSED_DESTRUCT_CODE: Self = Self(135);
    pub const END_DISCUSSED_DESTRUCT_CODE: Self = Self(135);
    pub const DISCUSSED_URQUAN_WARP: Self = Self(136);
    pub const END_DISCUSSED_URQUAN_WARP: Self = Self(136);
    pub const DISCUSSED_ARTIFACT_2: Self = Self(137);
    pub const END_DISCUSSED_ARTIFACT_2: Self = Self(137);
    pub const DISCUSSED_ARTIFACT_3: Self = Self(138);
    pub const END_DISCUSSED_ARTIFACT_3: Self = Self(138);
    pub const ATTACKED_DRUUGE: Self = Self(139);
    pub const END_ATTACKED_DRUUGE: Self = Self(139);
    pub const NEW_ALLIANCE_NAME: Self = Self(140);
    pub const END_NEW_ALLIANCE_NAME: Self = Self(141);
    pub const PORTAL_COUNTER: Self = Self(142);
    pub const END_PORTAL_COUNTER: Self = Self(145);
    pub const BURVIXESE_BROADCASTERS: Self = Self(146);
    pub const END_BURVIXESE_BROADCASTERS: Self = Self(146);
    pub const BURV_BROADCASTERS_ON_SHIP: Self = Self(147);
    pub const END_BURV_BROADCASTERS_ON_SHIP: Self = Self(147);
    pub const UTWIG_BOMB: Self = Self(148);
    pub const END_UTWIG_BOMB: Self = Self(148);
    pub const UTWIG_BOMB_ON_SHIP: Self = Self(149);
    pub const END_UTWIG_BOMB_ON_SHIP: Self = Self(149);
    pub const AQUA_HELIX: Self = Self(150);
    pub const END_AQUA_HELIX: Self = Self(150);
    pub const AQUA_HELIX_ON_SHIP: Self = Self(151);
    pub const END_AQUA_HELIX_ON_SHIP: Self = Self(151);
    pub const SUN_DEVICE: Self = Self(152);
    pub const END_SUN_DEVICE: Self = Self(152);
    pub const SUN_DEVICE_ON_SHIP: Self = Self(153);
    pub const END_SUN_DEVICE_ON_SHIP: Self = Self(153);
    pub const TAALO_PROTECTOR: Self = Self(154);
    pub const END_TAALO_PROTECTOR: Self = Self(154);
    pub const TAALO_PROTECTOR_ON_SHIP: Self = Self(155);
    pub const END_TAALO_PROTECTOR_ON_SHIP: Self = Self(155);
    pub const SHIP_VAULT_UNLOCKED: Self = Self(156);
    pub const END_SHIP_VAULT_UNLOCKED: Self = Self(156);
    pub const SYREEN_SHUTTLE: Self = Self(157);
    pub const END_SYREEN_SHUTTLE: Self = Self(157);
    pub const PORTAL_KEY: Self = Self(158);
    pub const END_PORTAL_KEY: Self = Self(158);
    pub const PORTAL_KEY_ON_SHIP: Self = Self(159);
    pub const END_PORTAL_KEY_ON_SHIP: Self = Self(159);
    pub const VUX_BEAST: Self = Self(160);
    pub const END_VUX_BEAST: Self = Self(160);
    pub const VUX_BEAST_ON_SHIP: Self = Self(161);
    pub const END_VUX_BEAST_ON_SHIP: Self = Self(161);
    pub const TALKING_PET: Self = Self(162);
    pub const END_TALKING_PET: Self = Self(162);
    pub const TALKING_PET_ON_SHIP: Self = Self(163);
    pub const END_TALKING_PET_ON_SHIP: Self = Self(163);
    pub const MOONBASE_ON_SHIP: Self = Self(164);
    pub const END_MOONBASE_ON_SHIP: Self = Self(164);
    pub const KOHR_AH_FRENZY: Self = Self(165);
    pub const END_KOHR_AH_FRENZY: Self = Self(165);
    pub const KOHR_AH_VISITS: Self = Self(166);
    pub const END_KOHR_AH_VISITS: Self = Self(167);
    pub const KOHR_AH_BYES: Self = Self(168);
    pub const END_KOHR_AH_BYES: Self = Self(168);
    pub const SLYLANDRO_HOME_VISITS: Self = Self(169);
    pub const END_SLYLANDRO_HOME_VISITS: Self = Self(171);
    pub const DESTRUCT_CODE_ON_SHIP: Self = Self(172);
    pub const END_DESTRUCT_CODE_ON_SHIP: Self = Self(172);
    pub const ILWRATH_VISITS: Self = Self(173);
    pub const END_ILWRATH_VISITS: Self = Self(175);
    pub const ILWRATH_DECEIVED: Self = Self(176);
    pub const END_ILWRATH_DECEIVED: Self = Self(176);
    pub const FLAGSHIP_CLOAKED: Self = Self(177);
    pub const END_FLAGSHIP_CLOAKED: Self = Self(177);
    pub const MYCON_VISITS: Self = Self(178);
    pub const END_MYCON_VISITS: Self = Self(180);
    pub const MYCON_HOME_VISITS: Self = Self(181);
    pub const END_MYCON_HOME_VISITS: Self = Self(183);
    pub const MYCON_AMBUSH: Self = Self(184);
    pub const END_MYCON_AMBUSH: Self = Self(184);
    pub const MYCON_FELL_FOR_AMBUSH: Self = Self(185);
    pub const END_MYCON_FELL_FOR_AMBUSH: Self = Self(185);
    pub const GLOBAL_FLAGS_AND_DATA: Self = Self(186);
    pub const END_GLOBAL_FLAGS_AND_DATA: Self = Self(193);
    pub const ORZ_VISITS: Self = Self(194);
    pub const END_ORZ_VISITS: Self = Self(196);
    pub const TAALO_VISITS: Self = Self(197);
    pub const END_TAALO_VISITS: Self = Self(199);
    pub const ORZ_MANNER: Self = Self(200);
    pub const END_ORZ_MANNER: Self = Self(201);
    pub const PROBE_EXHIBITED_BUG: Self = Self(202);
    pub const END_PROBE_EXHIBITED_BUG: Self = Self(202);
    pub const CLEAR_SPINDLE_ON_SHIP: Self = Self(203);
    pub const END_CLEAR_SPINDLE_ON_SHIP: Self = Self(203);
    pub const URQUAN_VISITS: Self = Self(204);
    pub const END_URQUAN_VISITS: Self = Self(206);
    pub const PLAYER_HYPNOTIZED: Self = Self(207);
    pub const END_PLAYER_HYPNOTIZED: Self = Self(207);
    pub const VUX_VISITS: Self = Self(208);
    pub const END_VUX_VISITS: Self = Self(210);
    pub const VUX_HOME_VISITS: Self = Self(211);
    pub const END_VUX_HOME_VISITS: Self = Self(213);
    pub const ZEX_VISITS: Self = Self(214);
    pub const END_ZEX_VISITS: Self = Self(216);
    pub const ZEX_IS_DEAD: Self = Self(217);
    pub const END_ZEX_IS_DEAD: Self = Self(217);
    pub const KNOW_ZEX_WANTS_MONSTER: Self = Self(218);
    pub const END_KNOW_ZEX_WANTS_MONSTER: Self = Self(218);
    pub const UTWIG_VISITS: Self = Self(219);
    pub const END_UTWIG_VISITS: Self = Self(221);
    pub const UTWIG_HOME_VISITS: Self = Self(222);
    pub const END_UTWIG_HOME_VISITS: Self = Self(224);
    pub const BOMB_VISITS: Self = Self(225);
    pub const END_BOMB_VISITS: Self = Self(227);
    pub const ULTRON_CONDITION: Self = Self(228);
    pub const END_ULTRON_CONDITION: Self = Self(230);
    pub const UTWIG_HAVE_ULTRON: Self = Self(231);
    pub const END_UTWIG_HAVE_ULTRON: Self = Self(231);
    pub const BOMB_UNPROTECTED: Self = Self(232);
    pub const END_BOMB_UNPROTECTED: Self = Self(232);
    pub const TAALO_UNPROTECTED: Self = Self(233);
    pub const END_TAALO_UNPROTECTED: Self = Self(233);
    pub const TALKING_PET_VISITS: Self = Self(234);
    pub const END_TALKING_PET_VISITS: Self = Self(236);
    pub const TALKING_PET_HOME_VISITS: Self = Self(237);
    pub const END_TALKING_PET_HOME_VISITS: Self = Self(239);
    pub const UMGAH_ZOMBIE_BLOBBIES: Self = Self(240);
    pub const END_UMGAH_ZOMBIE_BLOBBIES: Self = Self(240);
    pub const KNOW_UMGAH_ZOMBIES: Self = Self(241);
    pub const END_KNOW_UMGAH_ZOMBIES: Self = Self(241);
    pub const ARILOU_VISITS: Self = Self(242);
    pub const END_ARILOU_VISITS: Self = Self(244);
    pub const ARILOU_HOME_VISITS: Self = Self(245);
    pub const END_ARILOU_HOME_VISITS: Self = Self(247);
    pub const KNOW_ARILOU_WANT_WRECK: Self = Self(248);
    pub const END_KNOW_ARILOU_WANT_WRECK: Self = Self(248);
    pub const ARILOU_CHECKED_UMGAH: Self = Self(249);
    pub const END_ARILOU_CHECKED_UMGAH: Self = Self(250);
    pub const PORTAL_SPAWNER: Self = Self(251);
    pub const END_PORTAL_SPAWNER: Self = Self(251);
    pub const PORTAL_SPAWNER_ON_SHIP: Self = Self(252);
    pub const END_PORTAL_SPAWNER_ON_SHIP: Self = Self(252);
    pub const UMGAH_VISITS: Self = Self(253);
    pub const END_UMGAH_VISITS: Self = Self(255);
    pub const UMGAH_HOME_VISITS: Self = Self(256);
    pub const END_UMGAH_HOME_VISITS: Self = Self(258);
    pub const MET_NORMAL_UMGAH: Self = Self(259);
    pub const END_MET_NORMAL_UMGAH: Self = Self(259);
    pub const SYREEN_HOME_VISITS: Self = Self(260);
    pub const END_SYREEN_HOME_VISITS: Self = Self(262);
    pub const SYREEN_SHUTTLE_ON_SHIP: Self = Self(263);
    pub const END_SYREEN_SHUTTLE_ON_SHIP: Self = Self(263);
    pub const KNOW_SYREEN_VAULT: Self = Self(264);
    pub const END_KNOW_SYREEN_VAULT: Self = Self(264);
    pub const EGG_CASE0_ON_SHIP: Self = Self(265);
    pub const END_EGG_CASE0_ON_SHIP: Self = Self(265);
    pub const SUN_DEVICE_UNGUARDED: Self = Self(266);
    pub const END_SUN_DEVICE_UNGUARDED: Self = Self(266);
    pub const ROSY_SPHERE_ON_SHIP: Self = Self(267);
    pub const END_ROSY_SPHERE_ON_SHIP: Self = Self(267);
    pub const CHMMR_HOME_VISITS: Self = Self(268);
    pub const END_CHMMR_HOME_VISITS: Self = Self(270);
    pub const CHMMR_EMERGING: Self = Self(271);
    pub const END_CHMMR_EMERGING: Self = Self(271);
    pub const CHMMR_UNLEASHED: Self = Self(272);
    pub const END_CHMMR_UNLEASHED: Self = Self(272);
    pub const CHMMR_BOMB_STATE: Self = Self(273);
    pub const END_CHMMR_BOMB_STATE: Self = Self(274);
    pub const DRUUGE_DISCLAIMER: Self = Self(275);
    pub const END_DRUUGE_DISCLAIMER: Self = Self(275);
    pub const YEHAT_VISITS: Self = Self(276);
    pub const END_YEHAT_VISITS: Self = Self(278);
    pub const YEHAT_REBEL_VISITS: Self = Self(279);
    pub const END_YEHAT_REBEL_VISITS: Self = Self(281);
    pub const YEHAT_HOME_VISITS: Self = Self(282);
    pub const END_YEHAT_HOME_VISITS: Self = Self(284);
    pub const YEHAT_CIVIL_WAR: Self = Self(285);
    pub const END_YEHAT_CIVIL_WAR: Self = Self(285);
    pub const YEHAT_ABSORBED_PKUNK: Self = Self(286);
    pub const END_YEHAT_ABSORBED_PKUNK: Self = Self(286);
    pub const YEHAT_SHIP_MONTH: Self = Self(287);
    pub const END_YEHAT_SHIP_MONTH: Self = Self(290);
    pub const YEHAT_SHIP_DAY: Self = Self(291);
    pub const END_YEHAT_SHIP_DAY: Self = Self(295);
    pub const YEHAT_SHIP_YEAR: Self = Self(296);
    pub const END_YEHAT_SHIP_YEAR: Self = Self(300);
    pub const CLEAR_SPINDLE: Self = Self(301);
    pub const END_CLEAR_SPINDLE: Self = Self(301);
    pub const PKUNK_VISITS: Self = Self(302);
    pub const END_PKUNK_VISITS: Self = Self(304);
    pub const PKUNK_HOME_VISITS: Self = Self(305);
    pub const END_PKUNK_HOME_VISITS: Self = Self(307);
    pub const PKUNK_SHIP_MONTH: Self = Self(308);
    pub const END_PKUNK_SHIP_MONTH: Self = Self(311);
    pub const PKUNK_SHIP_DAY: Self = Self(312);
    pub const END_PKUNK_SHIP_DAY: Self = Self(316);
    pub const PKUNK_SHIP_YEAR: Self = Self(317);
    pub const END_PKUNK_SHIP_YEAR: Self = Self(321);
    pub const PKUNK_MISSION: Self = Self(322);
    pub const END_PKUNK_MISSION: Self = Self(324);
    pub const SUPOX_VISITS: Self = Self(325);
    pub const END_SUPOX_VISITS: Self = Self(327);
    pub const SUPOX_HOME_VISITS: Self = Self(328);
    pub const END_SUPOX_HOME_VISITS: Self = Self(330);
    pub const THRADD_VISITS: Self = Self(331);
    pub const END_THRADD_VISITS: Self = Self(333);
    pub const THRADD_HOME_VISITS: Self = Self(334);
    pub const END_THRADD_HOME_VISITS: Self = Self(336);
    pub const HELIX_VISITS: Self = Self(337);
    pub const END_HELIX_VISITS: Self = Self(339);
    pub const HELIX_UNPROTECTED: Self = Self(340);
    pub const END_HELIX_UNPROTECTED: Self = Self(340);
    pub const THRADD_CULTURE: Self = Self(341);
    pub const END_THRADD_CULTURE: Self = Self(342);
    pub const THRADD_MISSION: Self = Self(343);
    pub const END_THRADD_MISSION: Self = Self(345);
    pub const DRUUGE_VISITS: Self = Self(346);
    pub const END_DRUUGE_VISITS: Self = Self(348);
    pub const DRUUGE_HOME_VISITS: Self = Self(349);
    pub const END_DRUUGE_HOME_VISITS: Self = Self(351);
    pub const ROSY_SPHERE: Self = Self(352);
    pub const END_ROSY_SPHERE: Self = Self(352);
    pub const SCANNED_MAIDENS: Self = Self(353);
    pub const END_SCANNED_MAIDENS: Self = Self(353);
    pub const SCANNED_FRAGMENTS: Self = Self(354);
    pub const END_SCANNED_FRAGMENTS: Self = Self(354);
    pub const SCANNED_CASTER: Self = Self(355);
    pub const END_SCANNED_CASTER: Self = Self(355);
    pub const SCANNED_SPAWNER: Self = Self(356);
    pub const END_SCANNED_SPAWNER: Self = Self(356);
    pub const SCANNED_ULTRON: Self = Self(357);
    pub const END_SCANNED_ULTRON: Self = Self(357);
    pub const ZOQFOT_INFO: Self = Self(358);
    pub const END_ZOQFOT_INFO: Self = Self(359);
    pub const ZOQFOT_HOSTILE: Self = Self(360);
    pub const END_ZOQFOT_HOSTILE: Self = Self(360);
    pub const ZOQFOT_HOME_VISITS: Self = Self(361);
    pub const END_ZOQFOT_HOME_VISITS: Self = Self(363);
    pub const MET_ZOQFOT: Self = Self(364);
    pub const END_MET_ZOQFOT: Self = Self(364);
    pub const ZOQFOT_DISTRESS: Self = Self(365);
    pub const END_ZOQFOT_DISTRESS: Self = Self(366);
    pub const EGG_CASE1_ON_SHIP: Self = Self(367);
    pub const END_EGG_CASE1_ON_SHIP: Self = Self(367);
    pub const EGG_CASE2_ON_SHIP: Self = Self(368);
    pub const END_EGG_CASE2_ON_SHIP: Self = Self(368);
    pub const MYCON_SUN_VISITS: Self = Self(369);
    pub const END_MYCON_SUN_VISITS: Self = Self(371);
    pub const ORZ_HOME_VISITS: Self = Self(372);
    pub const END_ORZ_HOME_VISITS: Self = Self(374);
    pub const MELNORME_FUEL_PROCEDURE: Self = Self(375);
    pub const END_MELNORME_FUEL_PROCEDURE: Self = Self(375);
    pub const MELNORME_TECH_PROCEDURE: Self = Self(376);
    pub const END_MELNORME_TECH_PROCEDURE: Self = Self(376);
    pub const MELNORME_INFO_PROCEDURE: Self = Self(377);
    pub const END_MELNORME_INFO_PROCEDURE: Self = Self(377);
    pub const MELNORME_TECH_STACK: Self = Self(378);
    pub const END_MELNORME_TECH_STACK: Self = Self(381);
    pub const MELNORME_EVENTS_INFO_STACK: Self = Self(382);
    pub const END_MELNORME_EVENTS_INFO_STACK: Self = Self(386);
    pub const MELNORME_ALIEN_INFO_STACK: Self = Self(387);
    pub const END_MELNORME_ALIEN_INFO_STACK: Self = Self(391);
    pub const MELNORME_HISTORY_INFO_STACK: Self = Self(392);
    pub const END_MELNORME_HISTORY_INFO_STACK: Self = Self(396);
    pub const RAINBOW_WORLD0: Self = Self(397);
    pub const END_RAINBOW_WORLD0: Self = Self(404);
    pub const RAINBOW_WORLD1: Self = Self(405);
    pub const END_RAINBOW_WORLD1: Self = Self(406);
    pub const MELNORME_RAINBOW_COUNT: Self = Self(407);
    pub const END_MELNORME_RAINBOW_COUNT: Self = Self(410);
    pub const USED_BROADCASTER: Self = Self(411);
    pub const END_USED_BROADCASTER: Self = Self(411);
    pub const BROADCASTER_RESPONSE: Self = Self(412);
    pub const END_BROADCASTER_RESPONSE: Self = Self(412);
    pub const IMPROVED_LANDER_SPEED: Self = Self(413);
    pub const END_IMPROVED_LANDER_SPEED: Self = Self(413);
    pub const IMPROVED_LANDER_CARGO: Self = Self(414);
    pub const END_IMPROVED_LANDER_CARGO: Self = Self(414);
    pub const IMPROVED_LANDER_SHOT: Self = Self(415);
    pub const END_IMPROVED_LANDER_SHOT: Self = Self(415);
    pub const MET_ORZ_BEFORE: Self = Self(416);
    pub const END_MET_ORZ_BEFORE: Self = Self(416);
    pub const YEHAT_REBEL_TOLD_PKUNK: Self = Self(417);
    pub const END_YEHAT_REBEL_TOLD_PKUNK: Self = Self(417);
    pub const PLAYER_HAD_SEX: Self = Self(418);
    pub const END_PLAYER_HAD_SEX: Self = Self(418);
    pub const UMGAH_BROADCASTERS_ON_SHIP: Self = Self(419);
    pub const END_UMGAH_BROADCASTERS_ON_SHIP: Self = Self(419);
    pub const LIGHT_MINERAL_LOAD: Self = Self(420);
    pub const END_LIGHT_MINERAL_LOAD: Self = Self(422);
    pub const MEDIUM_MINERAL_LOAD: Self = Self(423);
    pub const END_MEDIUM_MINERAL_LOAD: Self = Self(425);
    pub const HEAVY_MINERAL_LOAD: Self = Self(426);
    pub const END_HEAVY_MINERAL_LOAD: Self = Self(428);
    pub const STARBASE_BULLETS0: Self = Self(429);
    pub const END_STARBASE_BULLETS0: Self = Self(436);
    pub const STARBASE_BULLETS1: Self = Self(437);
    pub const END_STARBASE_BULLETS1: Self = Self(444);
    pub const STARBASE_BULLETS2: Self = Self(445);
    pub const END_STARBASE_BULLETS2: Self = Self(452);
    pub const STARBASE_BULLETS3: Self = Self(453);
    pub const END_STARBASE_BULLETS3: Self = Self(460);
    pub const STARBASE_MONTH: Self = Self(461);
    pub const END_STARBASE_MONTH: Self = Self(464);
    pub const STARBASE_DAY: Self = Self(465);
    pub const END_STARBASE_DAY: Self = Self(469);
    pub const CREW_SOLD_TO_DRUUGE0: Self = Self(470);
    pub const END_CREW_SOLD_TO_DRUUGE0: Self = Self(477);
    pub const CREW_PURCHASED0: Self = Self(478);
    pub const END_CREW_PURCHASED0: Self = Self(485);
    pub const CREW_PURCHASED1: Self = Self(486);
    pub const END_CREW_PURCHASED1: Self = Self(493);
    pub const URQUAN_PROTECTING_SAMATRA: Self = Self(494);
    pub const END_URQUAN_PROTECTING_SAMATRA: Self = Self(494);
    pub const THRADDASH_BODY_COUNT: Self = Self(495);
    pub const END_THRADDASH_BODY_COUNT: Self = Self(499);
    pub const UTWIG_SUPOX_MISSION: Self = Self(500);
    pub const END_UTWIG_SUPOX_MISSION: Self = Self(502);
    pub const SPATHI_INFO: Self = Self(503);
    pub const END_SPATHI_INFO: Self = Self(505);
    pub const ILWRATH_INFO: Self = Self(506);
    pub const END_ILWRATH_INFO: Self = Self(507);
    pub const ILWRATH_GODS_SPOKEN: Self = Self(508);
    pub const END_ILWRATH_GODS_SPOKEN: Self = Self(511);
    pub const ILWRATH_WORSHIP: Self = Self(512);
    pub const END_ILWRATH_WORSHIP: Self = Self(513);
    pub const ILWRATH_FIGHT_THRADDASH: Self = Self(514);
    pub const END_ILWRATH_FIGHT_THRADDASH: Self = Self(514);
    pub const READY_TO_CONFUSE_URQUAN: Self = Self(515);
    pub const END_READY_TO_CONFUSE_URQUAN: Self = Self(515);
    pub const URQUAN_HYPNO_VISITS: Self = Self(516);
    pub const END_URQUAN_HYPNO_VISITS: Self = Self(516);
    pub const MENTIONED_PET_COMPULSION: Self = Self(517);
    pub const END_MENTIONED_PET_COMPULSION: Self = Self(517);
    pub const URQUAN_INFO: Self = Self(518);
    pub const END_URQUAN_INFO: Self = Self(519);
    pub const KNOW_URQUAN_STORY: Self = Self(520);
    pub const END_KNOW_URQUAN_STORY: Self = Self(521);
    pub const MYCON_INFO: Self = Self(522);
    pub const END_MYCON_INFO: Self = Self(525);
    pub const MYCON_RAMBLE: Self = Self(526);
    pub const END_MYCON_RAMBLE: Self = Self(530);
    pub const KNOW_ABOUT_SHATTERED: Self = Self(531);
    pub const END_KNOW_ABOUT_SHATTERED: Self = Self(532);
    pub const MYCON_INSULTS: Self = Self(533);
    pub const END_MYCON_INSULTS: Self = Self(535);
    pub const MYCON_KNOW_AMBUSH: Self = Self(536);
    pub const END_MYCON_KNOW_AMBUSH: Self = Self(536);
    pub const SYREEN_INFO: Self = Self(537);
    pub const END_SYREEN_INFO: Self = Self(538);
    pub const KNOW_SYREEN_WORLD_SHATTERED: Self = Self(539);
    pub const END_KNOW_SYREEN_WORLD_SHATTERED: Self = Self(539);
    pub const SYREEN_KNOW_ABOUT_MYCON: Self = Self(540);
    pub const END_SYREEN_KNOW_ABOUT_MYCON: Self = Self(540);
    pub const TALKING_PET_INFO: Self = Self(541);
    pub const END_TALKING_PET_INFO: Self = Self(543);
    pub const TALKING_PET_SUGGESTIONS: Self = Self(544);
    pub const END_TALKING_PET_SUGGESTIONS: Self = Self(546);
    pub const LEARNED_TALKING_PET: Self = Self(547);
    pub const END_LEARNED_TALKING_PET: Self = Self(547);
    pub const DNYARRI_LIED: Self = Self(548);
    pub const END_DNYARRI_LIED: Self = Self(548);
    pub const SHIP_TO_COMPEL: Self = Self(549);
    pub const END_SHIP_TO_COMPEL: Self = Self(549);
    pub const ORZ_GENERAL_INFO: Self = Self(550);
    pub const END_ORZ_GENERAL_INFO: Self = Self(551);
    pub const ORZ_PERSONAL_INFO: Self = Self(552);
    pub const END_ORZ_PERSONAL_INFO: Self = Self(554);
    pub const ORZ_ANDRO_STATE: Self = Self(555);
    pub const END_ORZ_ANDRO_STATE: Self = Self(556);
    pub const REFUSED_ORZ_ALLIANCE: Self = Self(557);
    pub const END_REFUSED_ORZ_ALLIANCE: Self = Self(557);
    pub const PKUNK_MANNER: Self = Self(558);
    pub const END_PKUNK_MANNER: Self = Self(559);
    pub const PKUNK_ON_THE_MOVE: Self = Self(560);
    pub const END_PKUNK_ON_THE_MOVE: Self = Self(560);
    pub const PKUNK_FLEET: Self = Self(561);
    pub const END_PKUNK_FLEET: Self = Self(562);
    pub const PKUNK_MIGRATE: Self = Self(563);
    pub const END_PKUNK_MIGRATE: Self = Self(564);
    pub const PKUNK_RETURN: Self = Self(565);
    pub const END_PKUNK_RETURN: Self = Self(565);
    pub const PKUNK_WORRY: Self = Self(566);
    pub const END_PKUNK_WORRY: Self = Self(567);
    pub const PKUNK_INFO: Self = Self(568);
    pub const END_PKUNK_INFO: Self = Self(570);
    pub const PKUNK_WAR: Self = Self(571);
    pub const END_PKUNK_WAR: Self = Self(572);
    pub const PKUNK_FORTUNE: Self = Self(573);
    pub const END_PKUNK_FORTUNE: Self = Self(575);
    pub const PKUNK_MIGRATE_VISITS: Self = Self(576);
    pub const END_PKUNK_MIGRATE_VISITS: Self = Self(578);
    pub const PKUNK_REASONS: Self = Self(579);
    pub const END_PKUNK_REASONS: Self = Self(582);
    pub const PKUNK_SWITCH: Self = Self(583);
    pub const END_PKUNK_SWITCH: Self = Self(583);
    pub const PKUNK_SENSE_VICTOR: Self = Self(584);
    pub const END_PKUNK_SENSE_VICTOR: Self = Self(584);
    pub const KOHR_AH_REASONS: Self = Self(585);
    pub const END_KOHR_AH_REASONS: Self = Self(586);
    pub const KOHR_AH_PLEAD: Self = Self(587);
    pub const END_KOHR_AH_PLEAD: Self = Self(588);
    pub const KOHR_AH_INFO: Self = Self(589);
    pub const END_KOHR_AH_INFO: Self = Self(590);
    pub const KNOW_KOHR_AH_STORY: Self = Self(591);
    pub const END_KNOW_KOHR_AH_STORY: Self = Self(592);
    pub const KOHR_AH_SENSES_EVIL: Self = Self(593);
    pub const END_KOHR_AH_SENSES_EVIL: Self = Self(593);
    pub const URQUAN_SENSES_EVIL: Self = Self(594);
    pub const END_URQUAN_SENSES_EVIL: Self = Self(594);
    pub const SLYLANDRO_PROBE_VISITS: Self = Self(595);
    pub const END_SLYLANDRO_PROBE_VISITS: Self = Self(597);
    pub const SLYLANDRO_PROBE_THREAT: Self = Self(598);
    pub const END_SLYLANDRO_PROBE_THREAT: Self = Self(599);
    pub const SLYLANDRO_PROBE_WRONG: Self = Self(600);
    pub const END_SLYLANDRO_PROBE_WRONG: Self = Self(601);
    pub const SLYLANDRO_PROBE_ID: Self = Self(602);
    pub const END_SLYLANDRO_PROBE_ID: Self = Self(603);
    pub const SLYLANDRO_PROBE_INFO: Self = Self(604);
    pub const END_SLYLANDRO_PROBE_INFO: Self = Self(605);
    pub const SLYLANDRO_PROBE_EXIT: Self = Self(606);
    pub const END_SLYLANDRO_PROBE_EXIT: Self = Self(607);
    pub const UMGAH_HOSTILE: Self = Self(608);
    pub const END_UMGAH_HOSTILE: Self = Self(608);
    pub const UMGAH_EVIL_BLOBBIES: Self = Self(609);
    pub const END_UMGAH_EVIL_BLOBBIES: Self = Self(609);
    pub const UMGAH_MENTIONED_TRICKS: Self = Self(610);
    pub const END_UMGAH_MENTIONED_TRICKS: Self = Self(611);
    pub const BOMB_CARRIER: Self = Self(612);
    pub const END_BOMB_CARRIER: Self = Self(612);
    pub const THRADD_MANNER: Self = Self(613);
    pub const END_THRADD_MANNER: Self = Self(613);
    pub const THRADD_INTRO: Self = Self(614);
    pub const END_THRADD_INTRO: Self = Self(615);
    pub const THRADD_DEMEANOR: Self = Self(616);
    pub const END_THRADD_DEMEANOR: Self = Self(618);
    pub const THRADD_INFO: Self = Self(619);
    pub const END_THRADD_INFO: Self = Self(620);
    pub const THRADD_BODY_LEVEL: Self = Self(621);
    pub const END_THRADD_BODY_LEVEL: Self = Self(622);
    pub const THRADD_MISSION_VISITS: Self = Self(623);
    pub const END_THRADD_MISSION_VISITS: Self = Self(623);
    pub const THRADD_STACK_1: Self = Self(624);
    pub const END_THRADD_STACK_1: Self = Self(626);
    pub const THRADD_HOSTILE_STACK_2: Self = Self(627);
    pub const END_THRADD_HOSTILE_STACK_2: Self = Self(627);
    pub const THRADD_HOSTILE_STACK_3: Self = Self(628);
    pub const END_THRADD_HOSTILE_STACK_3: Self = Self(628);
    pub const THRADD_HOSTILE_STACK_4: Self = Self(629);
    pub const END_THRADD_HOSTILE_STACK_4: Self = Self(629);
    pub const THRADD_HOSTILE_STACK_5: Self = Self(630);
    pub const END_THRADD_HOSTILE_STACK_5: Self = Self(630);
    pub const CHMMR_STACK: Self = Self(631);
    pub const END_CHMMR_STACK: Self = Self(632);
    pub const ARILOU_MANNER: Self = Self(633);
    pub const END_ARILOU_MANNER: Self = Self(634);
    pub const NO_PORTAL_VISITS: Self = Self(635);
    pub const END_NO_PORTAL_VISITS: Self = Self(635);
    pub const ARILOU_STACK_1: Self = Self(636);
    pub const END_ARILOU_STACK_1: Self = Self(637);
    pub const ARILOU_STACK_2: Self = Self(638);
    pub const END_ARILOU_STACK_2: Self = Self(638);
    pub const ARILOU_STACK_3: Self = Self(639);
    pub const END_ARILOU_STACK_3: Self = Self(640);
    pub const ARILOU_STACK_4: Self = Self(641);
    pub const END_ARILOU_STACK_4: Self = Self(641);
    pub const ARILOU_STACK_5: Self = Self(642);
    pub const END_ARILOU_STACK_5: Self = Self(643);
    pub const ARILOU_INFO: Self = Self(644);
    pub const END_ARILOU_INFO: Self = Self(645);
    pub const ARILOU_HINTS: Self = Self(646);
    pub const END_ARILOU_HINTS: Self = Self(647);
    pub const DRUUGE_MANNER: Self = Self(648);
    pub const END_DRUUGE_MANNER: Self = Self(648);
    pub const DRUUGE_SPACE_INFO: Self = Self(649);
    pub const END_DRUUGE_SPACE_INFO: Self = Self(650);
    pub const DRUUGE_HOME_INFO: Self = Self(651);
    pub const END_DRUUGE_HOME_INFO: Self = Self(652);
    pub const DRUUGE_SALVAGE: Self = Self(653);
    pub const END_DRUUGE_SALVAGE: Self = Self(653);
    pub const KNOW_DRUUGE_SLAVERS: Self = Self(654);
    pub const END_KNOW_DRUUGE_SLAVERS: Self = Self(655);
    pub const FRAGMENTS_BOUGHT: Self = Self(656);
    pub const END_FRAGMENTS_BOUGHT: Self = Self(657);
    pub const ZEX_STACK_1: Self = Self(658);
    pub const END_ZEX_STACK_1: Self = Self(659);
    pub const ZEX_STACK_2: Self = Self(660);
    pub const END_ZEX_STACK_2: Self = Self(661);
    pub const ZEX_STACK_3: Self = Self(662);
    pub const END_ZEX_STACK_3: Self = Self(663);
    pub const VUX_INFO: Self = Self(664);
    pub const END_VUX_INFO: Self = Self(665);
    pub const VUX_STACK_1: Self = Self(666);
    pub const END_VUX_STACK_1: Self = Self(669);
    pub const VUX_STACK_2: Self = Self(670);
    pub const END_VUX_STACK_2: Self = Self(671);
    pub const VUX_STACK_3: Self = Self(672);
    pub const END_VUX_STACK_3: Self = Self(673);
    pub const VUX_STACK_4: Self = Self(674);
    pub const END_VUX_STACK_4: Self = Self(675);
    pub const SHOFIXTI_STACK4: Self = Self(676);
    pub const END_SHOFIXTI_STACK4: Self = Self(677);
    pub const YEHAT_REBEL_INFO: Self = Self(678);
    pub const END_YEHAT_REBEL_INFO: Self = Self(680);
    pub const YEHAT_ROYALIST_INFO: Self = Self(681);
    pub const END_YEHAT_ROYALIST_INFO: Self = Self(681);
    pub const YEHAT_ROYALIST_TOLD_PKUNK: Self = Self(682);
    pub const END_YEHAT_ROYALIST_TOLD_PKUNK: Self = Self(682);
    pub const NO_YEHAT_ALLY_HOME: Self = Self(683);
    pub const END_NO_YEHAT_ALLY_HOME: Self = Self(683);
    pub const NO_YEHAT_HELP_HOME: Self = Self(684);
    pub const END_NO_YEHAT_HELP_HOME: Self = Self(684);
    pub const NO_YEHAT_INFO: Self = Self(685);
    pub const END_NO_YEHAT_INFO: Self = Self(685);
    pub const NO_YEHAT_ALLY_SPACE: Self = Self(686);
    pub const END_NO_YEHAT_ALLY_SPACE: Self = Self(687);
    pub const NO_YEHAT_HELP_SPACE: Self = Self(688);
    pub const END_NO_YEHAT_HELP_SPACE: Self = Self(689);
    pub const ZOQFOT_KNOW_MASK: Self = Self(690);
    pub const END_ZOQFOT_KNOW_MASK: Self = Self(693);
    pub const SUPOX_HOSTILE: Self = Self(694);
    pub const END_SUPOX_HOSTILE: Self = Self(694);
    pub const SUPOX_INFO: Self = Self(695);
    pub const END_SUPOX_INFO: Self = Self(695);
    pub const SUPOX_WAR_NEWS: Self = Self(696);
    pub const END_SUPOX_WAR_NEWS: Self = Self(697);
    pub const SUPOX_ULTRON_HELP: Self = Self(698);
    pub const END_SUPOX_ULTRON_HELP: Self = Self(698);
    pub const SUPOX_STACK1: Self = Self(699);
    pub const END_SUPOX_STACK1: Self = Self(701);
    pub const SUPOX_STACK2: Self = Self(702);
    pub const END_SUPOX_STACK2: Self = Self(703);
    pub const UTWIG_HOSTILE: Self = Self(704);
    pub const END_UTWIG_HOSTILE: Self = Self(704);
    pub const UTWIG_INFO: Self = Self(705);
    pub const END_UTWIG_INFO: Self = Self(705);
    pub const UTWIG_WAR_NEWS: Self = Self(706);
    pub const END_UTWIG_WAR_NEWS: Self = Self(707);
    pub const UTWIG_STACK1: Self = Self(708);
    pub const END_UTWIG_STACK1: Self = Self(710);
    pub const UTWIG_STACK2: Self = Self(711);
    pub const END_UTWIG_STACK2: Self = Self(712);
    pub const BOMB_INFO: Self = Self(713);
    pub const END_BOMB_INFO: Self = Self(713);
    pub const BOMB_STACK1: Self = Self(714);
    pub const END_BOMB_STACK1: Self = Self(715);
    pub const BOMB_STACK2: Self = Self(716);
    pub const END_BOMB_STACK2: Self = Self(717);
    pub const SLYLANDRO_KNOW_BROKEN: Self = Self(718);
    pub const END_SLYLANDRO_KNOW_BROKEN: Self = Self(718);
    pub const PLAYER_KNOWS_PROBE: Self = Self(719);
    pub const END_PLAYER_KNOWS_PROBE: Self = Self(719);
    pub const PLAYER_KNOWS_PROGRAM: Self = Self(720);
    pub const END_PLAYER_KNOWS_PROGRAM: Self = Self(720);
    pub const PLAYER_KNOWS_EFFECTS: Self = Self(721);
    pub const END_PLAYER_KNOWS_EFFECTS: Self = Self(721);
    pub const PLAYER_KNOWS_PRIORITY: Self = Self(722);
    pub const END_PLAYER_KNOWS_PRIORITY: Self = Self(722);
    pub const SLYLANDRO_STACK1: Self = Self(723);
    pub const END_SLYLANDRO_STACK1: Self = Self(725);
    pub const SLYLANDRO_STACK2: Self = Self(726);
    pub const END_SLYLANDRO_STACK2: Self = Self(726);
    pub const SLYLANDRO_STACK3: Self = Self(727);
    pub const END_SLYLANDRO_STACK3: Self = Self(728);
    pub const SLYLANDRO_STACK4: Self = Self(729);
    pub const END_SLYLANDRO_STACK4: Self = Self(730);
    pub const SLYLANDRO_STACK5: Self = Self(731);
    pub const END_SLYLANDRO_STACK5: Self = Self(731);
    pub const SLYLANDRO_STACK6: Self = Self(732);
    pub const END_SLYLANDRO_STACK6: Self = Self(732);
    pub const SLYLANDRO_STACK7: Self = Self(733);
    pub const END_SLYLANDRO_STACK7: Self = Self(734);
    pub const SLYLANDRO_STACK8: Self = Self(735);
    pub const END_SLYLANDRO_STACK8: Self = Self(736);
    pub const SLYLANDRO_STACK9: Self = Self(737);
    pub const END_SLYLANDRO_STACK9: Self = Self(738);
    pub const SLYLANDRO_KNOW_EARTH: Self = Self(739);
    pub const END_SLYLANDRO_KNOW_EARTH: Self = Self(739);
    pub const SLYLANDRO_KNOW_EXPLORE: Self = Self(740);
    pub const END_SLYLANDRO_KNOW_EXPLORE: Self = Self(740);
    pub const SLYLANDRO_KNOW_GATHER: Self = Self(741);
    pub const END_SLYLANDRO_KNOW_GATHER: Self = Self(741);
    pub const SLYLANDRO_KNOW_URQUAN: Self = Self(742);
    pub const END_SLYLANDRO_KNOW_URQUAN: Self = Self(743);
    pub const RECALL_VISITS: Self = Self(744);
    pub const END_RECALL_VISITS: Self = Self(745);
    pub const SLYLANDRO_MULTIPLIER: Self = Self(746);
    pub const END_SLYLANDRO_MULTIPLIER: Self = Self(748);
    pub const KNOW_SPATHI_QUEST: Self = Self(749);
    pub const END_KNOW_SPATHI_QUEST: Self = Self(749);
    pub const KNOW_SPATHI_EVIL: Self = Self(750);
    pub const END_KNOW_SPATHI_EVIL: Self = Self(750);
    pub const BATTLE_PLANET: Self = Self(751);
    pub const END_BATTLE_PLANET: Self = Self(758);
    pub const ESCAPE_COUNTER: Self = Self(759);
    pub const END_ESCAPE_COUNTER: Self = Self(766);
    pub const CREW_SOLD_TO_DRUUGE1: Self = Self(767);
    pub const END_CREW_SOLD_TO_DRUUGE1: Self = Self(774);
    pub const PKUNK_DONE_WAR: Self = Self(775);
    pub const END_PKUNK_DONE_WAR: Self = Self(775);
    pub const SYREEN_STACK0: Self = Self(776);
    pub const END_SYREEN_STACK0: Self = Self(777);
    pub const SYREEN_STACK1: Self = Self(778);
    pub const END_SYREEN_STACK1: Self = Self(779);
    pub const SYREEN_STACK2: Self = Self(780);
    pub const END_SYREEN_STACK2: Self = Self(781);
    pub const REFUSED_ULTRON_AT_BOMB: Self = Self(782);
    pub const END_REFUSED_ULTRON_AT_BOMB: Self = Self(782);
    pub const NO_TRICK_AT_SUN: Self = Self(783);
    pub const END_NO_TRICK_AT_SUN: Self = Self(783);
    pub const SPATHI_STACK0: Self = Self(784);
    pub const END_SPATHI_STACK0: Self = Self(785);
    pub const SPATHI_STACK1: Self = Self(786);
    pub const END_SPATHI_STACK1: Self = Self(786);
    pub const SPATHI_STACK2: Self = Self(787);
    pub const END_SPATHI_STACK2: Self = Self(787);
    pub const ORZ_STACK0: Self = Self(788);
    pub const END_ORZ_STACK0: Self = Self(788);
    pub const ORZ_STACK1: Self = Self(789);
    pub const END_ORZ_STACK1: Self = Self(789);
    pub const SHOFIXTI_GRPOFFS0: Self = Self(790);
    pub const END_SHOFIXTI_GRPOFFS0: Self = Self(797);
    pub const SHOFIXTI_GRPOFFS1: Self = Self(798);
    pub const END_SHOFIXTI_GRPOFFS1: Self = Self(805);
    pub const SHOFIXTI_GRPOFFS2: Self = Self(806);
    pub const END_SHOFIXTI_GRPOFFS2: Self = Self(813);
    pub const SHOFIXTI_GRPOFFS3: Self = Self(814);
    pub const END_SHOFIXTI_GRPOFFS3: Self = Self(821);
    pub const ZOQFOT_GRPOFFS0: Self = Self(822);
    pub const END_ZOQFOT_GRPOFFS0: Self = Self(829);
    pub const ZOQFOT_GRPOFFS1: Self = Self(830);
    pub const END_ZOQFOT_GRPOFFS1: Self = Self(837);
    pub const ZOQFOT_GRPOFFS2: Self = Self(838);
    pub const END_ZOQFOT_GRPOFFS2: Self = Self(845);
    pub const ZOQFOT_GRPOFFS3: Self = Self(846);
    pub const END_ZOQFOT_GRPOFFS3: Self = Self(853);
    pub const MELNORME0_GRPOFFS0: Self = Self(854);
    pub const END_MELNORME0_GRPOFFS0: Self = Self(861);
    pub const MELNORME0_GRPOFFS1: Self = Self(862);
    pub const END_MELNORME0_GRPOFFS1: Self = Self(869);
    pub const MELNORME0_GRPOFFS2: Self = Self(870);
    pub const END_MELNORME0_GRPOFFS2: Self = Self(877);
    pub const MELNORME0_GRPOFFS3: Self = Self(878);
    pub const END_MELNORME0_GRPOFFS3: Self = Self(885);
    pub const MELNORME1_GRPOFFS0: Self = Self(886);
    pub const END_MELNORME1_GRPOFFS0: Self = Self(893);
    pub const MELNORME1_GRPOFFS1: Self = Self(894);
    pub const END_MELNORME1_GRPOFFS1: Self = Self(901);
    pub const MELNORME1_GRPOFFS2: Self = Self(902);
    pub const END_MELNORME1_GRPOFFS2: Self = Self(909);
    pub const MELNORME1_GRPOFFS3: Self = Self(910);
    pub const END_MELNORME1_GRPOFFS3: Self = Self(917);
    pub const MELNORME2_GRPOFFS0: Self = Self(918);
    pub const END_MELNORME2_GRPOFFS0: Self = Self(925);
    pub const MELNORME2_GRPOFFS1: Self = Self(926);
    pub const END_MELNORME2_GRPOFFS1: Self = Self(933);
    pub const MELNORME2_GRPOFFS2: Self = Self(934);
    pub const END_MELNORME2_GRPOFFS2: Self = Self(941);
    pub const MELNORME2_GRPOFFS3: Self = Self(942);
    pub const END_MELNORME2_GRPOFFS3: Self = Self(949);
    pub const MELNORME3_GRPOFFS0: Self = Self(950);
    pub const END_MELNORME3_GRPOFFS0: Self = Self(957);
    pub const MELNORME3_GRPOFFS1: Self = Self(958);
    pub const END_MELNORME3_GRPOFFS1: Self = Self(965);
    pub const MELNORME3_GRPOFFS2: Self = Self(966);
    pub const END_MELNORME3_GRPOFFS2: Self = Self(973);
    pub const MELNORME3_GRPOFFS3: Self = Self(974);
    pub const END_MELNORME3_GRPOFFS3: Self = Self(981);
    pub const MELNORME4_GRPOFFS0: Self = Self(982);
    pub const END_MELNORME4_GRPOFFS0: Self = Self(989);
    pub const MELNORME4_GRPOFFS1: Self = Self(990);
    pub const END_MELNORME4_GRPOFFS1: Self = Self(997);
    pub const MELNORME4_GRPOFFS2: Self = Self(998);
    pub const END_MELNORME4_GRPOFFS2: Self = Self(1005);
    pub const MELNORME4_GRPOFFS3: Self = Self(1006);
    pub const END_MELNORME4_GRPOFFS3: Self = Self(1013);
    pub const MELNORME5_GRPOFFS0: Self = Self(1014);
    pub const END_MELNORME5_GRPOFFS0: Self = Self(1021);
    pub const MELNORME5_GRPOFFS1: Self = Self(1022);
    pub const END_MELNORME5_GRPOFFS1: Self = Self(1029);
    pub const MELNORME5_GRPOFFS2: Self = Self(1030);
    pub const END_MELNORME5_GRPOFFS2: Self = Self(1037);
    pub const MELNORME5_GRPOFFS3: Self = Self(1038);
    pub const END_MELNORME5_GRPOFFS3: Self = Self(1045);
    pub const MELNORME6_GRPOFFS0: Self = Self(1046);
    pub const END_MELNORME6_GRPOFFS0: Self = Self(1053);
    pub const MELNORME6_GRPOFFS1: Self = Self(1054);
    pub const END_MELNORME6_GRPOFFS1: Self = Self(1061);
    pub const MELNORME6_GRPOFFS2: Self = Self(1062);
    pub const END_MELNORME6_GRPOFFS2: Self = Self(1069);
    pub const MELNORME6_GRPOFFS3: Self = Self(1070);
    pub const END_MELNORME6_GRPOFFS3: Self = Self(1077);
    pub const MELNORME7_GRPOFFS0: Self = Self(1078);
    pub const END_MELNORME7_GRPOFFS0: Self = Self(1085);
    pub const MELNORME7_GRPOFFS1: Self = Self(1086);
    pub const END_MELNORME7_GRPOFFS1: Self = Self(1093);
    pub const MELNORME7_GRPOFFS2: Self = Self(1094);
    pub const END_MELNORME7_GRPOFFS2: Self = Self(1101);
    pub const MELNORME7_GRPOFFS3: Self = Self(1102);
    pub const END_MELNORME7_GRPOFFS3: Self = Self(1109);
    pub const MELNORME8_GRPOFFS0: Self = Self(1110);
    pub const END_MELNORME8_GRPOFFS0: Self = Self(1117);
    pub const MELNORME8_GRPOFFS1: Self = Self(1118);
    pub const END_MELNORME8_GRPOFFS1: Self = Self(1125);
    pub const MELNORME8_GRPOFFS2: Self = Self(1126);
    pub const END_MELNORME8_GRPOFFS2: Self = Self(1133);
    pub const MELNORME8_GRPOFFS3: Self = Self(1134);
    pub const END_MELNORME8_GRPOFFS3: Self = Self(1141);
    pub const URQUAN_PROBE_GRPOFFS0: Self = Self(1142);
    pub const END_URQUAN_PROBE_GRPOFFS0: Self = Self(1149);
    pub const URQUAN_PROBE_GRPOFFS1: Self = Self(1150);
    pub const END_URQUAN_PROBE_GRPOFFS1: Self = Self(1157);
    pub const URQUAN_PROBE_GRPOFFS2: Self = Self(1158);
    pub const END_URQUAN_PROBE_GRPOFFS2: Self = Self(1165);
    pub const URQUAN_PROBE_GRPOFFS3: Self = Self(1166);
    pub const END_URQUAN_PROBE_GRPOFFS3: Self = Self(1173);
    pub const COLONY_GRPOFFS0: Self = Self(1174);
    pub const END_COLONY_GRPOFFS0: Self = Self(1181);
    pub const COLONY_GRPOFFS1: Self = Self(1182);
    pub const END_COLONY_GRPOFFS1: Self = Self(1189);
    pub const COLONY_GRPOFFS2: Self = Self(1190);
    pub const END_COLONY_GRPOFFS2: Self = Self(1197);
    pub const COLONY_GRPOFFS3: Self = Self(1198);
    pub const END_COLONY_GRPOFFS3: Self = Self(1205);
    pub const SAMATRA_GRPOFFS0: Self = Self(1206);
    pub const END_SAMATRA_GRPOFFS0: Self = Self(1213);
    pub const SAMATRA_GRPOFFS1: Self = Self(1214);
    pub const END_SAMATRA_GRPOFFS1: Self = Self(1221);
    pub const SAMATRA_GRPOFFS2: Self = Self(1222);
    pub const END_SAMATRA_GRPOFFS2: Self = Self(1229);
    pub const SAMATRA_GRPOFFS3: Self = Self(1230);
    pub const END_SAMATRA_GRPOFFS3: Self = Self(1237);
    pub const NUM_GAME_STATE_BITS: Self = Self(1238);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_4(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_4 {
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
pub struct GLOBDATA {
    pub SIS_state: SIS_STATE,
    pub Game_state: GAME_STATE,
}
pub type MENU_SOUND_FLAGS = UWORD;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct MENU_SOUND_EFFECT(pub ::core::ffi::c_uint);
impl MENU_SOUND_EFFECT {
    pub const MENU_SOUND_MOVE: Self = Self(0);
    pub const MENU_SOUND_SUCCESS: Self = Self(1);
    pub const MENU_SOUND_FAILURE: Self = Self(2);
    pub const MENU_SOUND_INVOKED: Self = Self(3);
}
pub type STAR_SEARCH_STATE = starsearch_state;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct starsearch_state {
    pub pMS: *mut MENU_STATE,
    pub Text: [UNICODE; 256],
    pub LastText: [UNICODE; 256],
    pub LastChangeTime: DWORD,
    pub FirstIndex: ::core::ffi::c_int,
    pub CurIndex: ::core::ffi::c_int,
    pub LastIndex: ::core::ffi::c_int,
    pub SingleClust: BOOLEAN,
    pub SingleMatch: BOOLEAN,
    pub Buffer: [UNICODE; 256],
    pub Prefix: *const UNICODE,
    pub Cluster: *const UNICODE,
    pub PrefixLen: ::core::ffi::c_int,
    pub ClusterLen: ::core::ffi::c_int,
    pub ClusterPos: ::core::ffi::c_int,
    pub SortedStars: [::core::ffi::c_int; 502],
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
unsafe extern "C" fn MAKE_POINT(mut x: COORD, mut y: COORD) -> POINT {
    let mut pt: POINT = point { x: x, y: y };
    return pt;
}
#[inline]
unsafe extern "C" fn LockLink(mut pq: *const QUEUE, mut h: HLINK) -> *mut LINK {
    if !h.is_null() {
        (!(*pq).pq_tab.is_null() && h as *mut BYTE >= (*pq).pq_tab
            && (h as *mut BYTE)
                < (*pq)
                    .pq_tab
                    .offset(
                        ((*pq).object_size as ::core::ffi::c_int
                            * (*pq).num_objects as ::core::ffi::c_int) as isize,
                    )
            || {
                __assert_fail(
                    b"pq->pq_tab && (BYTE*)h >= pq->pq_tab && (BYTE*)h < pq->pq_tab + pq->object_size * pq->num_objects\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    b"src/uqm/planets/../displist.h\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    77 as ::core::ffi::c_int,
                    b"LockLink\0".as_ptr() as *const ::core::ffi::c_char,
                );
                0 as ::core::ffi::c_int != 0
            }) as ::core::ffi::c_int;
    }
    return h as *mut LINK;
}
#[inline]
unsafe extern "C" fn UnlockLink(mut pq: *const QUEUE, mut h: HLINK) {
    if !h.is_null() {
        (!(*pq).pq_tab.is_null() && h as *mut BYTE >= (*pq).pq_tab
            && (h as *mut BYTE)
                < (*pq)
                    .pq_tab
                    .offset(
                        ((*pq).object_size as ::core::ffi::c_int
                            * (*pq).num_objects as ::core::ffi::c_int) as isize,
                    )
            || {
                __assert_fail(
                    b"pq->pq_tab && (BYTE*)h >= pq->pq_tab && (BYTE*)h < pq->pq_tab + pq->object_size * pq->num_objects\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    b"src/uqm/planets/../displist.h\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    88 as ::core::ffi::c_int,
                    b"UnlockLink\0".as_ptr() as *const ::core::ffi::c_char,
                );
                0 as ::core::ffi::c_int != 0
            }) as ::core::ffi::c_int;
    }
}
#[inline]
unsafe extern "C" fn LockFleetInfo(
    mut pq: *const QUEUE,
    mut h: HFLEETINFO,
) -> *mut FLEET_INFO {
    ((*pq).object_size as usize == ::core::mem::size_of::<FLEET_INFO>()
        || {
            __assert_fail(
                b"GetLinkSize (pq) == sizeof (FLEET_INFO)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/uqm/planets/../races.h\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                395 as ::core::ffi::c_int,
                b"LockFleetInfo\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    return LockLink(pq, h) as *mut FLEET_INFO;
}
#[inline]
unsafe extern "C" fn logxToUniverse(mut lx: SDWORD) -> COORD {
    return ((lx
        * ((9999 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
            >> 4 as ::core::ffi::c_int) * 10 as ::core::ffi::c_int)
        + ((((ScreenWidth - 64 as ::core::ffi::c_int
            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
            << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
            >> 4 as ::core::ffi::c_int) * 195 as ::core::ffi::c_int
            >> 1 as ::core::ffi::c_int))
        / ((((ScreenWidth - 64 as ::core::ffi::c_int
            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
            << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
            >> 4 as ::core::ffi::c_int) * 195 as ::core::ffi::c_int)) as COORD;
}
#[inline]
unsafe extern "C" fn logyToUniverse(mut ly: SDWORD) -> COORD {
    return (9999 as ::core::ffi::c_int
        - (ly
            * (9999 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                >> 4 as ::core::ffi::c_int)
            + ((((ScreenHeight - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                >> 4 as ::core::ffi::c_int) * 25 as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int))
            / ((((ScreenHeight - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                >> 4 as ::core::ffi::c_int) * 25 as ::core::ffi::c_int)) as COORD;
}
static mut cursorLoc: POINT = POINT { x: 0, y: 0 };
static mut mapOrigin: POINT = POINT { x: 0, y: 0 };
static mut zoomLevel: ::core::ffi::c_int = 0;
static mut StarMapFrame: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
#[inline]
unsafe extern "C" fn signedDivWithError(
    mut val: ::core::ffi::c_long,
    mut divisor: ::core::ffi::c_long,
) -> ::core::ffi::c_long {
    let mut invert: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if val < 0 as ::core::ffi::c_long {
        invert = 1 as ::core::ffi::c_int;
        val = -val;
    }
    val = (val + (divisor >> 1 as ::core::ffi::c_int)) / divisor;
    return if invert != 0 { -val } else { val };
}
#[inline]
unsafe extern "C" fn universeToDispx(mut ux: ::core::ffi::c_long) -> COORD {
    return (signedDivWithError(
        ((ux - mapOrigin.x as ::core::ffi::c_long) << zoomLevel)
            * (ScreenWidth - 64 as ::core::ffi::c_int
                - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                - 14 as ::core::ffi::c_int) as ::core::ffi::c_long,
        (9999 as ::core::ffi::c_int
            + ((9999 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                / (ScreenWidth - 64 as ::core::ffi::c_int
                    - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    - 14 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int))
            as ::core::ffi::c_long,
    )
        + (ScreenWidth - 64 as ::core::ffi::c_int
            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
            - 14 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
            >> 1 as ::core::ffi::c_int) as ::core::ffi::c_long) as COORD;
}
#[inline]
unsafe extern "C" fn universeToDispy(mut uy: ::core::ffi::c_long) -> COORD {
    return (signedDivWithError(
        (mapOrigin.y as ::core::ffi::c_long - uy << zoomLevel)
            * (ScreenHeight - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                - 13 as ::core::ffi::c_int) as ::core::ffi::c_long,
        (9999 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as ::core::ffi::c_long,
    )
        + (ScreenHeight - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
            - 13 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
            >> 1 as ::core::ffi::c_int) as ::core::ffi::c_long) as COORD;
}
#[inline]
unsafe extern "C" fn dispxToUniverse(mut dx: COORD) -> COORD {
    return (((dx as ::core::ffi::c_int
        - (ScreenWidth - 64 as ::core::ffi::c_int
            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
            - 14 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
            >> 1 as ::core::ffi::c_int)) as ::core::ffi::c_long
        * (9999 as ::core::ffi::c_int
            + ((9999 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                / (ScreenWidth - 64 as ::core::ffi::c_int
                    - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    - 14 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int))
            as ::core::ffi::c_long >> zoomLevel)
        / (ScreenWidth - 64 as ::core::ffi::c_int
            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
            - 14 as ::core::ffi::c_int) as ::core::ffi::c_long
        + mapOrigin.x as ::core::ffi::c_long) as COORD;
}
#[inline]
unsafe extern "C" fn dispyToUniverse(mut dy: COORD) -> COORD {
    return ((((ScreenHeight - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
        - 13 as ::core::ffi::c_int - 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
        - dy as ::core::ffi::c_int) as ::core::ffi::c_long
        * (9999 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as ::core::ffi::c_long
        >> zoomLevel)
        / (ScreenHeight - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
            - 13 as ::core::ffi::c_int) as ::core::ffi::c_long
        + mapOrigin.y as ::core::ffi::c_long) as COORD;
}
static mut transition_pending: BOOLEAN = BOOLEAN::FALSE;
unsafe extern "C" fn flashCurrentLocation(mut r#where: *mut POINT) {
    static mut c: BYTE = 0 as BYTE;
    static mut val: ::core::ffi::c_int = -2 as ::core::ffi::c_int;
    static mut universe: POINT = POINT { x: 0, y: 0 };
    static mut NextTime: TimeCount = 0 as TimeCount;
    if !r#where.is_null() {
        universe = *r#where;
    }
    if GetTimeCounter() >= NextTime {
        let mut OldColor: Color = Color { r: 0, g: 0, b: 0, a: 0 };
        let mut OldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
        let mut s: STAMP = STAMP {
            origin: POINT { x: 0, y: 0 },
            frame: ::core::ptr::null_mut::<FRAME_DESC>(),
        };
        NextTime = GetTimeCounter()
            .wrapping_add(
                (840 as ::core::ffi::c_int / 16 as ::core::ffi::c_int) as TimeCount,
            );
        OldContext = SetContext(SpaceContext);
        if c as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            || c as ::core::ffi::c_int == 0x1a as ::core::ffi::c_int
        {
            val = -val;
        }
        c = (c as ::core::ffi::c_int + val) as BYTE;
        OldColor = SetContextForeGroundColor(colorFromRgb15(c, c, c));
        s.origin.x = universeToDispx(universe.x as ::core::ffi::c_long);
        s.origin.y = universeToDispy(universe.y as ::core::ffi::c_long);
        s.frame = IncFrameIndex(StarMapFrame);
        DrawFilledStamp(&raw mut s);
        SetContextForeGroundColor(OldColor);
        SetContext(OldContext);
    }
}
unsafe extern "C" fn DrawCursor(mut curs_x: COORD, mut curs_y: COORD) {
    let mut s: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    s.origin.x = curs_x;
    s.origin.y = curs_y;
    s.frame = StarMapFrame;
    DrawStamp(&raw mut s);
}
unsafe extern "C" fn DrawAutoPilot(mut pDstPt: *mut POINT) {
    let mut dx: SIZE = 0;
    let mut dy: SIZE = 0;
    let mut xincr: SIZE = 0;
    let mut yincr: SIZE = 0;
    let mut xerror: SIZE = 0;
    let mut yerror: SIZE = 0;
    let mut cycle: SIZE = 0;
    let mut delta: SIZE = 0;
    let mut pt: POINT = POINT { x: 0, y: 0 };
    if inHQSpace().0 == 0 {
        pt = (*CurStarDescPtr).star_pt;
    } else {
        pt.x = logxToUniverse(GlobData.SIS_state.log_x);
        pt.y = logyToUniverse(GlobData.SIS_state.log_y);
    }
    pt.x = universeToDispx(pt.x as ::core::ffi::c_long);
    pt.y = universeToDispy(pt.y as ::core::ffi::c_long);
    dx = (universeToDispx((*pDstPt).x as ::core::ffi::c_long) as ::core::ffi::c_int
        - pt.x as ::core::ffi::c_int) as SIZE;
    if dx as ::core::ffi::c_int >= 0 as ::core::ffi::c_int {
        xincr = 1 as SIZE;
    } else {
        xincr = -1 as SIZE;
        dx = -(dx as ::core::ffi::c_int) as SIZE;
    }
    dx = ((dx as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as SIZE;
    dy = (universeToDispy((*pDstPt).y as ::core::ffi::c_long) as ::core::ffi::c_int
        - pt.y as ::core::ffi::c_int) as SIZE;
    if dy as ::core::ffi::c_int >= 0 as ::core::ffi::c_int {
        yincr = 1 as SIZE;
    } else {
        yincr = -1 as SIZE;
        dy = -(dy as ::core::ffi::c_int) as SIZE;
    }
    dy = ((dy as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as SIZE;
    if dx as ::core::ffi::c_int >= dy as ::core::ffi::c_int {
        cycle = dx;
    } else {
        cycle = dy;
    }
    yerror = (cycle as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as SIZE;
    xerror = yerror;
    delta = xerror;
    SetContextForeGroundColor(colorFromRgb15(0x4 as BYTE, 0x4 as BYTE, 0x1f as BYTE));
    delta = (delta as ::core::ffi::c_int & !(1 as ::core::ffi::c_int)) as SIZE;
    loop {
        let c2rust_fresh0 = delta;
        delta -= 1;
        if c2rust_fresh0 == 0 {
            break;
        }
        if delta as ::core::ffi::c_int & 1 as ::core::ffi::c_int == 0 {
            DrawPoint(&raw mut pt);
        }
        xerror = (xerror as ::core::ffi::c_int - dx as ::core::ffi::c_int) as SIZE;
        if xerror as ::core::ffi::c_int <= 0 as ::core::ffi::c_int {
            pt.x = (pt.x as ::core::ffi::c_int + xincr as ::core::ffi::c_int) as COORD;
            xerror = (xerror as ::core::ffi::c_int + cycle as ::core::ffi::c_int)
                as SIZE;
        }
        yerror = (yerror as ::core::ffi::c_int - dy as ::core::ffi::c_int) as SIZE;
        if yerror as ::core::ffi::c_int <= 0 as ::core::ffi::c_int {
            pt.y = (pt.y as ::core::ffi::c_int + yincr as ::core::ffi::c_int) as COORD;
            yerror = (yerror as ::core::ffi::c_int + cycle as ::core::ffi::c_int)
                as SIZE;
        }
    };
}
unsafe extern "C" fn GetSphereRect(
    mut FleetPtr: *mut FLEET_INFO,
    mut pRect: *mut RECT,
    mut pRepairRect: *mut RECT,
) {
    let mut diameter: ::core::ffi::c_long = 0;
    diameter = ((*FleetPtr).known_strength as ::core::ffi::c_int
        * 11 as ::core::ffi::c_int) as ::core::ffi::c_long;
    (*pRect).extent.width = (universeToDispx(diameter) as ::core::ffi::c_int
        - universeToDispx(0 as ::core::ffi::c_long) as ::core::ffi::c_int) as COORD;
    if ((*pRect).extent.width as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
        (*pRect).extent.width = -((*pRect).extent.width as ::core::ffi::c_int) as COORD;
    } else if (*pRect).extent.width as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        (*pRect).extent.width = 1 as COORD;
    }
    (*pRect).extent.height = (universeToDispy(diameter) as ::core::ffi::c_int
        - universeToDispy(0 as ::core::ffi::c_long) as ::core::ffi::c_int) as COORD;
    if ((*pRect).extent.height as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
        (*pRect).extent.height = -((*pRect).extent.height as ::core::ffi::c_int)
            as COORD;
    } else if (*pRect).extent.height as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        (*pRect).extent.height = 1 as COORD;
    }
    (*pRect).corner.x = universeToDispx((*FleetPtr).known_loc.x as ::core::ffi::c_long);
    (*pRect).corner.y = universeToDispy((*FleetPtr).known_loc.y as ::core::ffi::c_long);
    (*pRect).corner.x = ((*pRect).corner.x as ::core::ffi::c_int
        - ((*pRect).extent.width as ::core::ffi::c_int >> 1 as ::core::ffi::c_int))
        as COORD;
    (*pRect).corner.y = ((*pRect).corner.y as ::core::ffi::c_int
        - ((*pRect).extent.height as ::core::ffi::c_int >> 1 as ::core::ffi::c_int))
        as COORD;
    let mut t: TEXT = TEXT {
        baseline: POINT { x: 0, y: 0 },
        pStr: ::core::ptr::null::<UNICODE>(),
        align: TEXT_ALIGN::ALIGN_LEFT,
        CharCount: 0,
    };
    let mut locString: STRING = ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>();
    SetContextFont(TinyFont);
    t.baseline.x = ((*pRect).corner.x as ::core::ffi::c_int
        + ((*pRect).extent.width as ::core::ffi::c_int >> 1 as ::core::ffi::c_int))
        as COORD;
    t.baseline.y = ((*pRect).corner.y as ::core::ffi::c_int
        + ((*pRect).extent.height as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
        - 1 as ::core::ffi::c_int) as COORD;
    t.align = TEXT_ALIGN::ALIGN_CENTER;
    locString = SetAbsStringTableIndex((*FleetPtr).race_strings, 1 as COUNT);
    t.CharCount = GetStringLength(locString);
    t.pStr = GetStringAddress(locString);
    TextRect(&raw mut t, pRepairRect, ::core::ptr::null_mut::<BYTE>());
    if (*pRepairRect).corner.x as ::core::ffi::c_int <= 0 as ::core::ffi::c_int {
        (*pRepairRect).corner.x = 1 as COORD;
    } else if (*pRepairRect).corner.x as ::core::ffi::c_int
        + (*pRepairRect).extent.width as ::core::ffi::c_int
        >= ScreenWidth - 64 as ::core::ffi::c_int
            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
            - 14 as ::core::ffi::c_int
    {
        (*pRepairRect).corner.x = (ScreenWidth - 64 as ::core::ffi::c_int
            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
            - 14 as ::core::ffi::c_int
            - (*pRepairRect).extent.width as ::core::ffi::c_int
            - 1 as ::core::ffi::c_int) as COORD;
    }
    if (*pRepairRect).corner.y as ::core::ffi::c_int <= 0 as ::core::ffi::c_int {
        (*pRepairRect).corner.y = 1 as COORD;
    } else if (*pRepairRect).corner.y as ::core::ffi::c_int
        + (*pRepairRect).extent.height as ::core::ffi::c_int
        >= ScreenHeight - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
            - 13 as ::core::ffi::c_int
    {
        (*pRepairRect).corner.y = (ScreenHeight
            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
            - 13 as ::core::ffi::c_int
            - (*pRepairRect).extent.height as ::core::ffi::c_int
            - 1 as ::core::ffi::c_int) as COORD;
    }
    BoxUnion(pRepairRect, pRect, pRepairRect);
    (*pRepairRect).extent.width += 1;
    (*pRepairRect).extent.height += 1;
}
unsafe extern "C" fn DrawStarMap(mut race_update: COUNT, mut pClipRect: *mut RECT) {
    let mut i: SIZE = 0;
    let mut which_space: COUNT = 0;
    let mut diameter: ::core::ffi::c_long = 0;
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut old_r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut oldOrigin: POINT = point {
        x: 0 as COORD,
        y: 0 as COORD,
    };
    let mut s: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    let mut star_frame: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
    let mut SDPtr: *mut STAR_DESC = ::core::ptr::null_mut::<STAR_DESC>();
    let mut draw_cursor: BOOLEAN = BOOLEAN::FALSE;
    if pClipRect
        == ::core::ptr::with_exposed_provenance_mut::<RECT>(-1 as ::core::ffi::c_int as usize)
    {
        pClipRect = ::core::ptr::null_mut::<RECT>();
        draw_cursor = BOOLEAN::FALSE;
    } else {
        draw_cursor = BOOLEAN::TRUE;
    }
    SetContext(SpaceContext);
    if !pClipRect.is_null() {
        GetContextClipRect(&raw mut old_r);
        (*pClipRect).corner.x = ((*pClipRect).corner.x as ::core::ffi::c_int
            + old_r.corner.x as ::core::ffi::c_int) as COORD;
        (*pClipRect).corner.y = ((*pClipRect).corner.y as ::core::ffi::c_int
            + old_r.corner.y as ::core::ffi::c_int) as COORD;
        SetContextClipRect(pClipRect);
        (*pClipRect).corner.x = ((*pClipRect).corner.x as ::core::ffi::c_int
            - old_r.corner.x as ::core::ffi::c_int) as COORD;
        (*pClipRect).corner.y = ((*pClipRect).corner.y as ::core::ffi::c_int
            - old_r.corner.y as ::core::ffi::c_int) as COORD;
        oldOrigin = SetContextOrigin(
            MAKE_POINT(
                -((*pClipRect).corner.x as ::core::ffi::c_int) as COORD,
                -((*pClipRect).corner.y as ::core::ffi::c_int) as COORD,
            ),
        );
    }
    if transition_pending.0 != 0 {
        SetTransitionSource(::core::ptr::null::<RECT>());
    }
    BatchGraphics();
    which_space = getGameState(
        &raw mut GlobData.Game_state.GameState as *mut BYTE,
        C2Rust_Unnamed_3::ARILOU_SPACE_SIDE.0 as ::core::ffi::c_int,
        C2Rust_Unnamed_3::END_ARILOU_SPACE_SIDE.0 as ::core::ffi::c_int,
    ) as COUNT;
    if which_space as ::core::ffi::c_int <= 1 as ::core::ffi::c_int {
        SDPtr = star_array.offset(0isize);
        SetContextForeGroundColor(colorFromRgb15(0 as BYTE, 0 as BYTE, 0x7 as BYTE));
        SetContextBackGroundColor(colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE));
    } else {
        SDPtr = star_array
            .offset((502 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize);
        SetContextForeGroundColor(colorFromRgb15(0 as BYTE, 0xb as BYTE, 0 as BYTE));
        SetContextBackGroundColor(colorFromRgb15(0 as BYTE, 0x8 as BYTE, 0 as BYTE));
    }
    ClearDrawable();
    if race_update as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        && (which_space as ::core::ffi::c_int) < 2 as ::core::ffi::c_int
        && {
            diameter = (GlobData.SIS_state.FuelOnBoard as ::core::ffi::c_long)
                << 1 as ::core::ffi::c_int;
            diameter != 0
        }
    {
        let mut OldColor: Color = Color { r: 0, g: 0, b: 0, a: 0 };
        if inHQSpace().0 == 0 {
            r.corner = (*CurStarDescPtr).star_pt;
        } else {
            r.corner.x = logxToUniverse(GlobData.SIS_state.log_x);
            r.corner.y = logyToUniverse(GlobData.SIS_state.log_y);
        }
        if diameter
            > (9999 as ::core::ffi::c_int * 4 as ::core::ffi::c_int)
                as ::core::ffi::c_long
        {
            diameter = (9999 as ::core::ffi::c_int * 4 as ::core::ffi::c_int)
                as ::core::ffi::c_long;
        }
        r.extent.width = (universeToDispx(diameter) as ::core::ffi::c_int
            - universeToDispx(0 as ::core::ffi::c_long) as ::core::ffi::c_int) as COORD;
        if (r.extent.width as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
            r.extent.width = -(r.extent.width as ::core::ffi::c_int) as COORD;
        }
        r.extent.height = (universeToDispy(diameter) as ::core::ffi::c_int
            - universeToDispy(0 as ::core::ffi::c_long) as ::core::ffi::c_int) as COORD;
        if (r.extent.height as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
            r.extent.height = -(r.extent.height as ::core::ffi::c_int) as COORD;
        }
        r.corner.x = (universeToDispx(r.corner.x as ::core::ffi::c_long)
            as ::core::ffi::c_int
            - (r.extent.width as ::core::ffi::c_int >> 1 as ::core::ffi::c_int))
            as COORD;
        r.corner.y = (universeToDispy(r.corner.y as ::core::ffi::c_long)
            as ::core::ffi::c_int
            - (r.extent.height as ::core::ffi::c_int >> 1 as ::core::ffi::c_int))
            as COORD;
        OldColor = SetContextForeGroundColor(
            colorFromRgb15(0x3 as BYTE, 0x3 as BYTE, 0x3 as BYTE),
        );
        DrawFilledOval(&raw mut r);
        SetContextForeGroundColor(OldColor);
    }
    i = (9999 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as SIZE;
    while (i as ::core::ffi::c_int) >= 0 as ::core::ffi::c_int {
        let mut j: SIZE = 0;
        r.corner.x = universeToDispx(0 as ::core::ffi::c_long);
        r.corner.y = universeToDispy(i as ::core::ffi::c_long);
        r.extent.width = ((ScreenWidth - 64 as ::core::ffi::c_int
            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
            - 14 as ::core::ffi::c_int) << zoomLevel) as COORD;
        r.extent.height = 1 as COORD;
        DrawFilledRectangle(&raw mut r);
        r.corner.y = universeToDispy(
            (9999 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as ::core::ffi::c_long,
        );
        r.extent.width = 1 as COORD;
        r.extent.height = ((ScreenHeight
            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
            - 13 as ::core::ffi::c_int) << zoomLevel) as COORD;
        j = (9999 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as SIZE;
        while (j as ::core::ffi::c_int) >= 0 as ::core::ffi::c_int {
            r.corner.x = universeToDispx(j as ::core::ffi::c_long);
            DrawFilledRectangle(&raw mut r);
            j = (j as ::core::ffi::c_int - 500 as ::core::ffi::c_int) as SIZE;
        }
        i = (i as ::core::ffi::c_int - 500 as ::core::ffi::c_int) as SIZE;
    }
    star_frame = SetRelFrameIndex(StarMapFrame, 2 as SIZE);
    if which_space as ::core::ffi::c_int <= 1 as ::core::ffi::c_int {
        let mut index: COUNT = 0;
        let mut hStarShip: HFLEETINFO = ::core::ptr::null_mut::<::core::ffi::c_void>();
        let mut hNextShip: HFLEETINFO = ::core::ptr::null_mut::<::core::ffi::c_void>();
        static mut race_colors: [Color; 23] = [
            Color {
                r: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                g: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                b: ((0x10 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x10 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                a: 0xff as BYTE,
            },
            Color {
                r: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                g: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                a: 0xff as BYTE,
            },
            Color {
                r: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                g: ((0x1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x1 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                b: ((0x1f as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x1f as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                a: 0xff as BYTE,
            },
            Color {
                r: ((0xe as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0xe as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                g: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                b: ((0xe as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0xe as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                a: 0xff as BYTE,
            },
            Color {
                r: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                g: ((0x6 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x6 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                b: ((0x8 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x8 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                a: 0xff as BYTE,
            },
            Color {
                r: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                g: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                a: 0xff as BYTE,
            },
            Color {
                r: ((0xc as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0xc as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                g: ((0x5 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x5 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                a: 0xff as BYTE,
            },
            Color {
                r: ((0xc as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0xc as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                g: ((0x5 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x5 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                a: 0xff as BYTE,
            },
            Color {
                r: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                g: ((0x6 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x6 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                b: ((0x8 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x8 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                a: 0xff as BYTE,
            },
            Color {
                r: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                g: ((0x6 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x6 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                b: ((0x8 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x8 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                a: 0xff as BYTE,
            },
            Color {
                r: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                g: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                b: ((0x10 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x10 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                a: 0xff as BYTE,
            },
            Color {
                r: ((0xa as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0xa as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                g: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                b: ((0x11 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x11 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                a: 0xff as BYTE,
            },
            Color {
                r: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                g: ((0x6 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x6 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                b: ((0x8 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x8 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                a: 0xff as BYTE,
            },
            Color {
                r: ((0xf as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0xf as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                g: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                a: 0xff as BYTE,
            },
            Color {
                r: ((0xe as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0xe as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                g: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                b: ((0xe as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0xe as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                a: 0xff as BYTE,
            },
            Color {
                r: ((0xe as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0xe as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                g: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                b: ((0xe as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0xe as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                a: 0xff as BYTE,
            },
            Color {
                r: ((0xc as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0xc as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                g: ((0x5 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x5 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                a: 0xff as BYTE,
            },
            Color {
                r: ((0xa as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0xa as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                g: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                b: ((0x11 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x11 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                a: 0xff as BYTE,
            },
            Color {
                r: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                g: ((0x8 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x8 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                a: 0xff as BYTE,
            },
            Color {
                r: ((0xf as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0xf as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                g: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                a: 0xff as BYTE,
            },
            Color {
                r: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                g: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                a: 0xff as BYTE,
            },
            Color {
                r: ((0x6 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x6 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                g: ((0x6 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x6 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                b: ((0x6 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x6 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                a: 0xff as BYTE,
            },
            Color {
                r: ((0x14 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x14 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                g: ((0x7 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x7 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                b: ((0x1f as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x1f as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                a: 0xff as BYTE,
            },
        ];
        index = 0 as COUNT;
        hStarShip = GlobData.Game_state.avail_race_q.head as HFLEETINFO;
        while !hStarShip.is_null() {
            let mut FleetPtr: *mut FLEET_INFO = ::core::ptr::null_mut::<FLEET_INFO>();
            FleetPtr = LockFleetInfo(
                &raw mut GlobData.Game_state.avail_race_q,
                hStarShip,
            );
            hNextShip = (*FleetPtr).succ;
            if (*FleetPtr).known_strength != 0 {
                let mut repair_r: RECT = RECT {
                    corner: POINT { x: 0, y: 0 },
                    extent: EXTENT { width: 0, height: 0 },
                };
                GetSphereRect(FleetPtr, &raw mut r, &raw mut repair_r);
                if (r.corner.x as ::core::ffi::c_int)
                    < ScreenWidth - 64 as ::core::ffi::c_int
                        - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                        - 14 as ::core::ffi::c_int
                    && (r.corner.y as ::core::ffi::c_int)
                        < ScreenHeight
                            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                            - 13 as ::core::ffi::c_int
                    && r.corner.x as ::core::ffi::c_int
                        + r.extent.width as ::core::ffi::c_int > 0 as ::core::ffi::c_int
                    && r.corner.y as ::core::ffi::c_int
                        + r.extent.height as ::core::ffi::c_int > 0 as ::core::ffi::c_int
                    && (pClipRect.is_null()
                        || (repair_r.corner.x as ::core::ffi::c_int)
                            < (*pClipRect).corner.x as ::core::ffi::c_int
                                + (*pClipRect).extent.width as ::core::ffi::c_int
                            && (repair_r.corner.y as ::core::ffi::c_int)
                                < (*pClipRect).corner.y as ::core::ffi::c_int
                                    + (*pClipRect).extent.height as ::core::ffi::c_int
                            && repair_r.corner.x as ::core::ffi::c_int
                                + repair_r.extent.width as ::core::ffi::c_int
                                > (*pClipRect).corner.x as ::core::ffi::c_int
                            && repair_r.corner.y as ::core::ffi::c_int
                                + repair_r.extent.height as ::core::ffi::c_int
                                > (*pClipRect).corner.y as ::core::ffi::c_int)
                {
                    let mut c: Color = Color { r: 0, g: 0, b: 0, a: 0 };
                    let mut t: TEXT = TEXT {
                        baseline: POINT { x: 0, y: 0 },
                        pStr: ::core::ptr::null::<UNICODE>(),
                        align: TEXT_ALIGN::ALIGN_LEFT,
                        CharCount: 0,
                    };
                    let mut locString: STRING = ::core::ptr::null_mut::<
                        STRING_TABLE_ENTRY_DESC,
                    >();
                    c = race_colors[index as usize];
                    if index as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                        == race_update as ::core::ffi::c_int
                    {
                        SetContextForeGroundColor(
                            colorFromRgb15(0x1f as BYTE, 0x1f as BYTE, 0x1f as BYTE),
                        );
                    } else {
                        SetContextForeGroundColor(c);
                    }
                    DrawOval(&raw mut r, 0 as BYTE);
                    SetContextFont(TinyFont);
                    t.baseline.x = (r.corner.x as ::core::ffi::c_int
                        + (r.extent.width as ::core::ffi::c_int
                            >> 1 as ::core::ffi::c_int)) as COORD;
                    t.baseline.y = (r.corner.y as ::core::ffi::c_int
                        + (r.extent.height as ::core::ffi::c_int
                            >> 1 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                        as COORD;
                    t.align = TEXT_ALIGN::ALIGN_CENTER;
                    locString = SetAbsStringTableIndex(
                        (*FleetPtr).race_strings,
                        1 as COUNT,
                    );
                    t.CharCount = GetStringLength(locString);
                    t.pStr = GetStringAddress(locString);
                    TextRect(&raw mut t, &raw mut r, ::core::ptr::null_mut::<BYTE>());
                    if r.corner.x as ::core::ffi::c_int <= 0 as ::core::ffi::c_int {
                        t.baseline.x = (t.baseline.x as ::core::ffi::c_int
                            - (r.corner.x as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int)) as COORD;
                    } else if r.corner.x as ::core::ffi::c_int
                        + r.extent.width as ::core::ffi::c_int
                        >= ScreenWidth - 64 as ::core::ffi::c_int
                            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                            - 14 as ::core::ffi::c_int
                    {
                        t.baseline.x = (t.baseline.x as ::core::ffi::c_int
                            - (r.corner.x as ::core::ffi::c_int
                                + r.extent.width as ::core::ffi::c_int
                                - (ScreenWidth - 64 as ::core::ffi::c_int
                                    - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                                    - 14 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int))
                            as COORD;
                    }
                    if r.corner.y as ::core::ffi::c_int <= 0 as ::core::ffi::c_int {
                        t.baseline.y = (t.baseline.y as ::core::ffi::c_int
                            - (r.corner.y as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int)) as COORD;
                    } else if r.corner.y as ::core::ffi::c_int
                        + r.extent.height as ::core::ffi::c_int
                        >= ScreenHeight
                            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                            - 13 as ::core::ffi::c_int
                    {
                        t.baseline.y = (t.baseline.y as ::core::ffi::c_int
                            - (r.corner.y as ::core::ffi::c_int
                                + r.extent.height as ::core::ffi::c_int
                                - (ScreenHeight
                                    - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                                    - 13 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int))
                            as COORD;
                    }
                    c.r = (if c.r as ::core::ffi::c_int
                        >= 0xff as ::core::ffi::c_int
                            - ((0x3 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                | 0x3 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
                    {
                        0xff as ::core::ffi::c_int
                    } else {
                        c.r as ::core::ffi::c_int
                            + ((0x3 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                | 0x3 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
                    }) as BYTE;
                    c.g = (if c.g as ::core::ffi::c_int
                        >= 0xff as ::core::ffi::c_int
                            - ((0x3 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                | 0x3 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
                    {
                        0xff as ::core::ffi::c_int
                    } else {
                        c.g as ::core::ffi::c_int
                            + ((0x3 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                | 0x3 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
                    }) as BYTE;
                    c.b = (if c.b as ::core::ffi::c_int
                        >= 0xff as ::core::ffi::c_int
                            - ((0x3 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                | 0x3 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
                    {
                        0xff as ::core::ffi::c_int
                    } else {
                        c.b as ::core::ffi::c_int
                            + ((0x3 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                | 0x3 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
                    }) as BYTE;
                    SetContextForeGroundColor(c);
                    font_DrawText(&raw mut t);
                }
            }
            UnlockLink(&raw mut GlobData.Game_state.avail_race_q, hStarShip);
            index = index.wrapping_add(1);
            hStarShip = hNextShip;
        }
    }
    loop {
        let mut star_type: BYTE = 0;
        star_type = (*SDPtr).Type;
        s.origin.x = universeToDispx((*SDPtr).star_pt.x as ::core::ffi::c_long);
        s.origin.y = universeToDispy((*SDPtr).star_pt.y as ::core::ffi::c_long);
        if which_space as ::core::ffi::c_int <= 1 as ::core::ffi::c_int {
            s.frame = SetRelFrameIndex(
                star_frame,
                (((star_type as ::core::ffi::c_int
                    & ((0xff as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                        & !(((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int)
                            as BYTE as ::core::ffi::c_int)) as BYTE
                        as ::core::ffi::c_int) >> 3 as ::core::ffi::c_int) as BYTE
                    as ::core::ffi::c_int
                    * C2Rust_Unnamed_0::NUM_STAR_COLORS.0 as ::core::ffi::c_int
                    + ((star_type as ::core::ffi::c_int
                        & ((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int)
                            as BYTE as ::core::ffi::c_int) >> 5 as ::core::ffi::c_int)
                        as BYTE as ::core::ffi::c_int) as SIZE,
            );
        } else if (*SDPtr).star_pt.x as ::core::ffi::c_int
            == 5000 as ::core::ffi::c_int
                + (63 as ::core::ffi::c_int * 12 as ::core::ffi::c_int
                    >> 1 as ::core::ffi::c_int) * 3 as ::core::ffi::c_int
            && (*SDPtr).star_pt.y as ::core::ffi::c_int
                == 5000 as ::core::ffi::c_int
                    + (50 as ::core::ffi::c_int * 12 as ::core::ffi::c_int
                        >> 1 as ::core::ffi::c_int) * 3 as ::core::ffi::c_int
        {
            s.frame = SetRelFrameIndex(
                star_frame,
                (C2Rust_Unnamed::SUPER_GIANT_STAR.0 as ::core::ffi::c_int
                    * C2Rust_Unnamed_0::NUM_STAR_COLORS.0 as ::core::ffi::c_int
                    + C2Rust_Unnamed_0::GREEN_BODY.0 as ::core::ffi::c_int) as SIZE,
            );
        } else {
            s.frame = SetRelFrameIndex(
                star_frame,
                (C2Rust_Unnamed::GIANT_STAR.0 as ::core::ffi::c_int
                    * C2Rust_Unnamed_0::NUM_STAR_COLORS.0 as ::core::ffi::c_int
                    + C2Rust_Unnamed_0::GREEN_BODY.0 as ::core::ffi::c_int) as SIZE,
            );
        }
        DrawStamp(&raw mut s);
        SDPtr = SDPtr.offset(1);
        if !((*SDPtr).star_pt.x as ::core::ffi::c_int <= 9999 as ::core::ffi::c_int
            && (*SDPtr).star_pt.y as ::core::ffi::c_int <= 9999 as ::core::ffi::c_int)
        {
            break;
        }
    }
    if getGameState(
        &raw mut GlobData.Game_state.GameState as *mut BYTE,
        C2Rust_Unnamed_3::ARILOU_SPACE.0 as ::core::ffi::c_int,
        C2Rust_Unnamed_3::END_ARILOU_SPACE.0 as ::core::ffi::c_int,
    ) != 0
    {
        if which_space as ::core::ffi::c_int <= 1 as ::core::ffi::c_int {
            s.origin.x = universeToDispx(438 as ::core::ffi::c_long);
            s.origin.y = universeToDispy(6372 as ::core::ffi::c_long);
        } else {
            s.origin.x = universeToDispx(5000 as ::core::ffi::c_long);
            s.origin.y = universeToDispy(5000 as ::core::ffi::c_long);
        }
        s.frame = SetRelFrameIndex(
            star_frame,
            (C2Rust_Unnamed::GIANT_STAR.0 as ::core::ffi::c_int
                * C2Rust_Unnamed_0::NUM_STAR_COLORS.0 as ::core::ffi::c_int
                + C2Rust_Unnamed_0::GREEN_BODY.0 as ::core::ffi::c_int) as SIZE,
        );
        DrawStamp(&raw mut s);
    }
    if race_update as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        && GlobData.Game_state.autopilot.x as ::core::ffi::c_int
            != !(0 as ::core::ffi::c_int)
        && GlobData.Game_state.autopilot.y as ::core::ffi::c_int
            != !(0 as ::core::ffi::c_int)
    {
        DrawAutoPilot(&raw mut GlobData.Game_state.autopilot);
    }
    if transition_pending.0 != 0 {
        GetContextClipRect(&raw mut r);
        ScreenTransition(3 as ::core::ffi::c_int, &raw mut r);
        transition_pending = BOOLEAN::FALSE;
    }
    UnbatchGraphics();
    if !pClipRect.is_null() {
        SetContextClipRect(&raw mut old_r);
        SetContextOrigin(oldOrigin);
    }
    if race_update as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        if draw_cursor.0 != 0 {
            GetContextClipRect(&raw mut r);
            LoadIntoExtraScreen(&raw mut r);
            DrawCursor(
                universeToDispx(cursorLoc.x as ::core::ffi::c_long),
                universeToDispy(cursorLoc.y as ::core::ffi::c_long),
            );
        }
    }
}
unsafe extern "C" fn EraseCursor(mut curs_x: COORD, mut curs_y: COORD) {
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    GetFrameRect(StarMapFrame, &raw mut r);
    r.corner.x = (r.corner.x as ::core::ffi::c_int + curs_x as ::core::ffi::c_int)
        as COORD;
    if (r.corner.x as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
        r.extent.width = (r.extent.width as ::core::ffi::c_int
            + r.corner.x as ::core::ffi::c_int) as COORD;
        r.corner.x = 0 as COORD;
    } else if r.corner.x as ::core::ffi::c_int + r.extent.width as ::core::ffi::c_int
        >= ScreenWidth - 64 as ::core::ffi::c_int
            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
            - 14 as ::core::ffi::c_int
    {
        r.extent.width = (ScreenWidth - 64 as ::core::ffi::c_int
            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
            - 14 as ::core::ffi::c_int - r.corner.x as ::core::ffi::c_int) as COORD;
    }
    r.corner.y = (r.corner.y as ::core::ffi::c_int + curs_y as ::core::ffi::c_int)
        as COORD;
    if (r.corner.y as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
        r.extent.height = (r.extent.height as ::core::ffi::c_int
            + r.corner.y as ::core::ffi::c_int) as COORD;
        r.corner.y = 0 as COORD;
    } else if r.corner.y as ::core::ffi::c_int + r.extent.height as ::core::ffi::c_int
        >= ScreenHeight - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
            - 13 as ::core::ffi::c_int
    {
        r.extent.height = (ScreenHeight
            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
            - 13 as ::core::ffi::c_int - r.corner.y as ::core::ffi::c_int) as COORD;
    }
    RepairBackRect(&raw mut r);
}
unsafe extern "C" fn ZoomStarMap(mut dir: SIZE) {
    if dir as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
        if zoomLevel < 4 as ::core::ffi::c_int {
            zoomLevel += 1;
            mapOrigin = cursorLoc;
            DrawStarMap(0 as COUNT, ::core::ptr::null_mut::<RECT>());
            SleepThread(
                (840 as ::core::ffi::c_int / 8 as ::core::ffi::c_int) as TimePeriod,
            );
        }
    } else if (dir as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
        if zoomLevel > 0 as ::core::ffi::c_int {
            if zoomLevel > 1 as ::core::ffi::c_int {
                mapOrigin = cursorLoc;
            } else {
                mapOrigin.x = (9999 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
                    as COORD;
                mapOrigin.y = (9999 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
                    as COORD;
            }
            zoomLevel -= 1;
            DrawStarMap(0 as COUNT, ::core::ptr::null_mut::<RECT>());
            SleepThread(
                (840 as ::core::ffi::c_int / 8 as ::core::ffi::c_int) as TimePeriod,
            );
        }
    }
}
unsafe extern "C" fn UpdateCursorLocation(
    mut sx: ::core::ffi::c_int,
    mut sy: ::core::ffi::c_int,
    mut newpt: *const POINT,
) {
    let mut s: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    let mut pt: POINT = POINT { x: 0, y: 0 };
    pt.x = universeToDispx(cursorLoc.x as ::core::ffi::c_long);
    pt.y = universeToDispy(cursorLoc.y as ::core::ffi::c_long);
    if !newpt.is_null() {
        sy = 0 as ::core::ffi::c_int;
        sx = sy;
        s.origin.x = universeToDispx((*newpt).x as ::core::ffi::c_long);
        s.origin.y = universeToDispy((*newpt).y as ::core::ffi::c_long);
        cursorLoc = *newpt;
    } else {
        s.origin.x = (pt.x as ::core::ffi::c_int + sx) as COORD;
        s.origin.y = (pt.y as ::core::ffi::c_int + sy) as COORD;
    }
    if sx != 0 {
        cursorLoc.x = (dispxToUniverse(s.origin.x) as ::core::ffi::c_int - sx) as COORD;
        while universeToDispx(cursorLoc.x as ::core::ffi::c_long) as ::core::ffi::c_int
            == pt.x as ::core::ffi::c_int
        {
            cursorLoc.x = (cursorLoc.x as ::core::ffi::c_int + sx) as COORD;
        }
        if (cursorLoc.x as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
            cursorLoc.x = 0 as COORD;
        } else if cursorLoc.x as ::core::ffi::c_int > 9999 as ::core::ffi::c_int {
            cursorLoc.x = 9999 as COORD;
        }
        s.origin.x = universeToDispx(cursorLoc.x as ::core::ffi::c_long);
    }
    if sy != 0 {
        cursorLoc.y = (dispyToUniverse(s.origin.y) as ::core::ffi::c_int + sy) as COORD;
        while universeToDispy(cursorLoc.y as ::core::ffi::c_long) as ::core::ffi::c_int
            == pt.y as ::core::ffi::c_int
        {
            cursorLoc.y = (cursorLoc.y as ::core::ffi::c_int - sy) as COORD;
        }
        if (cursorLoc.y as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
            cursorLoc.y = 0 as COORD;
        } else if cursorLoc.y as ::core::ffi::c_int > 9999 as ::core::ffi::c_int {
            cursorLoc.y = 9999 as COORD;
        }
        s.origin.y = universeToDispy(cursorLoc.y as ::core::ffi::c_long);
    }
    if (s.origin.x as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
        || (s.origin.y as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
        || s.origin.x as ::core::ffi::c_int
            >= ScreenWidth - 64 as ::core::ffi::c_int
                - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                - 14 as ::core::ffi::c_int
        || s.origin.y as ::core::ffi::c_int
            >= ScreenHeight - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                - 13 as ::core::ffi::c_int
    {
        mapOrigin = cursorLoc;
        DrawStarMap(0 as COUNT, ::core::ptr::null_mut::<RECT>());
        s.origin.x = universeToDispx(cursorLoc.x as ::core::ffi::c_long);
        s.origin.y = universeToDispy(cursorLoc.y as ::core::ffi::c_long);
    } else {
        EraseCursor(pt.x, pt.y);
        DrawCursor(s.origin.x, s.origin.y);
    };
}
unsafe extern "C" fn UpdateCursorInfo(mut prevbuf: *mut UNICODE) {
    let mut buf: [UNICODE; 256] = ::core::mem::transmute::<
        [u8; 256],
        [UNICODE; 256],
    >(
        *b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    );
    let mut pt: POINT = POINT { x: 0, y: 0 };
    let mut SDPtr: *mut STAR_DESC = ::core::ptr::null_mut::<STAR_DESC>();
    let mut BestSDPtr: *mut STAR_DESC = ::core::ptr::null_mut::<STAR_DESC>();
    pt.x = universeToDispx(cursorLoc.x as ::core::ffi::c_long);
    pt.y = universeToDispy(cursorLoc.y as ::core::ffi::c_long);
    BestSDPtr = ::core::ptr::null_mut::<STAR_DESC>();
    SDPtr = BestSDPtr;
    loop {
        SDPtr = FindStar(SDPtr, &raw mut cursorLoc, 75 as SIZE, 75 as SIZE);
        if SDPtr.is_null() {
            break;
        }
        if universeToDispx((*SDPtr).star_pt.x as ::core::ffi::c_long)
            as ::core::ffi::c_int == pt.x as ::core::ffi::c_int
            && universeToDispy((*SDPtr).star_pt.y as ::core::ffi::c_long)
                as ::core::ffi::c_int == pt.y as ::core::ffi::c_int
            && (BestSDPtr.is_null()
                || (((*SDPtr).Type as ::core::ffi::c_int
                    & ((0xff as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                        & !(((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int)
                            as BYTE as ::core::ffi::c_int)) as BYTE
                        as ::core::ffi::c_int) >> 3 as ::core::ffi::c_int) as BYTE
                    as ::core::ffi::c_int
                    >= (((*BestSDPtr).Type as ::core::ffi::c_int
                        & ((0xff as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                            & !(((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int)
                                as BYTE as ::core::ffi::c_int)) as BYTE
                            as ::core::ffi::c_int) >> 3 as ::core::ffi::c_int) as BYTE
                        as ::core::ffi::c_int)
        {
            BestSDPtr = SDPtr;
        }
    }
    if !BestSDPtr.is_null() {
        cursorLoc = (*BestSDPtr).star_pt;
        GetClusterName(BestSDPtr, &raw mut buf as *mut UNICODE);
    } else {
        cursorLoc.x = dispxToUniverse(pt.x);
        if (cursorLoc.x as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
            cursorLoc.x = 0 as COORD;
        } else if cursorLoc.x as ::core::ffi::c_int > 9999 as ::core::ffi::c_int {
            cursorLoc.x = 9999 as COORD;
        }
        cursorLoc.y = dispyToUniverse(pt.y);
        if (cursorLoc.y as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
            cursorLoc.y = 0 as COORD;
        } else if cursorLoc.y as ::core::ffi::c_int > 9999 as ::core::ffi::c_int {
            cursorLoc.y = 9999 as COORD;
        }
    }
    if getGameState(
        &raw mut GlobData.Game_state.GameState as *mut BYTE,
        C2Rust_Unnamed_3::ARILOU_SPACE.0 as ::core::ffi::c_int,
        C2Rust_Unnamed_3::END_ARILOU_SPACE.0 as ::core::ffi::c_int,
    ) != 0
    {
        let mut ari_pt: POINT = POINT { x: 0, y: 0 };
        if getGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_3::ARILOU_SPACE_SIDE.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_3::END_ARILOU_SPACE_SIDE.0 as ::core::ffi::c_int,
        ) as ::core::ffi::c_int <= 1 as ::core::ffi::c_int
        {
            ari_pt.x = 438 as COORD;
            ari_pt.y = 6372 as COORD;
        } else {
            ari_pt.x = 5000 as COORD;
            ari_pt.y = 5000 as COORD;
        }
        if universeToDispx(ari_pt.x as ::core::ffi::c_long) as ::core::ffi::c_int
            == pt.x as ::core::ffi::c_int
            && universeToDispy(ari_pt.y as ::core::ffi::c_long) as ::core::ffi::c_int
                == pt.y as ::core::ffi::c_int
        {
            cursorLoc = ari_pt;
            utf8StringCopy(
                &raw mut buf as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[UNICODE; 256]>(),
                GetStringAddress(
                    SetAbsStringTableIndex(
                        GameStrings,
                        (C2Rust_Unnamed_2::STAR_STRING_BASE.0 as ::core::ffi::c_int
                            + 132 as ::core::ffi::c_int) as COUNT,
                    ),
                ),
            );
        }
    }
    DrawHyperCoords(cursorLoc);
    if strcmp(&raw mut buf as *mut UNICODE, prevbuf) != 0 as ::core::ffi::c_int {
        strcpy(prevbuf, &raw mut buf as *mut UNICODE);
        DrawSISMessage(&raw mut buf as *mut UNICODE);
    }
}
unsafe extern "C" fn UpdateFuelRequirement() {
    let mut buf: [UNICODE; 80] = [0; 80];
    let mut fuel_required: COUNT = 0;
    let mut f: DWORD = 0;
    let mut pt: POINT = POINT { x: 0, y: 0 };
    if inHQSpace().0 == 0 {
        pt = (*CurStarDescPtr).star_pt;
    } else {
        pt.x = logxToUniverse(GlobData.SIS_state.log_x);
        pt.y = logyToUniverse(GlobData.SIS_state.log_y);
    }
    pt.x = (pt.x as ::core::ffi::c_int - cursorLoc.x as ::core::ffi::c_int) as COORD;
    pt.y = (pt.y as ::core::ffi::c_int - cursorLoc.y as ::core::ffi::c_int) as COORD;
    f = (pt.x as ::core::ffi::c_long * pt.x as ::core::ffi::c_long
        + pt.y as ::core::ffi::c_long * pt.y as ::core::ffi::c_long) as DWORD;
    if f == 0 as DWORD
        || getGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_3::ARILOU_SPACE_SIDE.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_3::END_ARILOU_SPACE_SIDE.0 as ::core::ffi::c_int,
        ) as ::core::ffi::c_int > 1 as ::core::ffi::c_int
    {
        fuel_required = 0 as COUNT;
    } else {
        fuel_required = (square_root(f) as ::core::ffi::c_int
            + 100 as ::core::ffi::c_int / 20 as ::core::ffi::c_int) as COUNT;
    }
    sprintf(
        &raw mut buf as *mut ::core::ffi::c_char,
        b"%s %u.%u\0".as_ptr() as *const ::core::ffi::c_char,
        GetStringAddress(
            SetAbsStringTableIndex(
                GameStrings,
                (C2Rust_Unnamed_2::NAVIGATION_STRING_BASE.0 as ::core::ffi::c_int
                    + 4 as ::core::ffi::c_int) as COUNT,
            ),
        ),
        fuel_required as ::core::ffi::c_int / 100 as ::core::ffi::c_int,
        fuel_required as ::core::ffi::c_int % 100 as ::core::ffi::c_int
            / 10 as ::core::ffi::c_int,
    );
    DrawStatusMessage(&raw mut buf as *mut UNICODE);
}
unsafe extern "C" fn compStarName(
    mut ptr1: *const ::core::ffi::c_void,
    mut ptr2: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut index1: ::core::ffi::c_int = 0;
    let mut index2: ::core::ffi::c_int = 0;
    index1 = *(ptr1 as *const ::core::ffi::c_int);
    index2 = *(ptr2 as *const ::core::ffi::c_int);
    if (*star_array.offset(index1 as isize)).Postfix as ::core::ffi::c_int
        != (*star_array.offset(index2 as isize)).Postfix as ::core::ffi::c_int
    {
        return utf8StringCompare(
            GetStringAddress(
                SetAbsStringTableIndex(
                    GameStrings,
                    (*star_array.offset(index1 as isize)).Postfix as COUNT,
                ),
            ),
            GetStringAddress(
                SetAbsStringTableIndex(
                    GameStrings,
                    (*star_array.offset(index2 as isize)).Postfix as COUNT,
                ),
            ),
        );
    }
    if ((*star_array.offset(index1 as isize)).Prefix as ::core::ffi::c_int)
        < (*star_array.offset(index2 as isize)).Prefix as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    if (*star_array.offset(index1 as isize)).Prefix as ::core::ffi::c_int
        > (*star_array.offset(index2 as isize)).Prefix as ::core::ffi::c_int
    {
        return 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn SortStarsOnName(mut pSS: *mut STAR_SEARCH_STATE) {
    let mut i: ::core::ffi::c_int = 0;
    let mut sorted: *mut ::core::ffi::c_int = &raw mut (*pSS).SortedStars
        as *mut ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < 502 as ::core::ffi::c_int {
        *sorted.offset(i as isize) = i;
        i += 1;
    }
    qsort(
        sorted as *mut ::core::ffi::c_void,
        502 as size_t,
        ::core::mem::size_of::<::core::ffi::c_int>(),
        Some(
            compStarName
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_void,
                    *const ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
    );
}
unsafe extern "C" fn SplitStarName(mut pSS: *mut STAR_SEARCH_STATE) {
    let mut buf: *mut UNICODE = &raw mut (*pSS).Buffer as *mut UNICODE;
    let mut next: *mut UNICODE = ::core::ptr::null_mut::<UNICODE>();
    let mut sep: *mut UNICODE = ::core::ptr::null_mut::<UNICODE>();
    (*pSS).Prefix = ::core::ptr::null::<UNICODE>();
    (*pSS).PrefixLen = 0 as ::core::ffi::c_int;
    (*pSS).Cluster = ::core::ptr::null::<UNICODE>();
    (*pSS).ClusterLen = 0 as ::core::ffi::c_int;
    (*pSS).ClusterPos = 0 as ::core::ffi::c_int;
    next = buf;
    while *next as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        && getCharFromString(&raw mut next as *mut *const ::core::ffi::c_char)
            == ' ' as UniChar
    {
        buf = next;
    }
    if *buf as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
        return;
    }
    (*pSS).Prefix = buf;
    buf = next;
    while *next as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        && getCharFromString(&raw mut next as *mut *const ::core::ffi::c_char)
            != ' ' as UniChar
    {
        buf = next;
    }
    if *buf as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        sep = buf;
        buf = next;
        while *next as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
            && getCharFromString(&raw mut next as *mut *const ::core::ffi::c_char)
                == ' ' as UniChar
        {
            buf = next;
        }
    }
    if *buf as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
        (*pSS).Cluster = (*pSS).Prefix;
        (*pSS).ClusterLen = utf8StringCount((*pSS).Cluster) as ::core::ffi::c_int;
        (*pSS).ClusterPos = utf8StringCountN(
            &raw mut (*pSS).Buffer as *mut UNICODE,
            (*pSS).Cluster,
        ) as ::core::ffi::c_int;
        (*pSS).Prefix = ::core::ptr::null::<UNICODE>();
        return;
    }
    (*pSS).Cluster = buf;
    (*pSS).ClusterLen = utf8StringCount((*pSS).Cluster) as ::core::ffi::c_int;
    (*pSS).ClusterPos = utf8StringCountN(
        &raw mut (*pSS).Buffer as *mut UNICODE,
        (*pSS).Cluster,
    ) as ::core::ffi::c_int;
    *sep = '\0' as UNICODE;
    (*pSS).PrefixLen = utf8StringCount((*pSS).Prefix) as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn SkipStarCluster(
    mut sortedStars: *mut ::core::ffi::c_int,
    mut istar: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut Postfix: ::core::ffi::c_int = (*star_array
        .offset(*sortedStars.offset(istar as isize) as isize))
        .Postfix as ::core::ffi::c_int;
    istar += 1;
    while istar < 502 as ::core::ffi::c_int
        && (*star_array.offset(*sortedStars.offset(istar as isize) as isize)).Postfix
            as ::core::ffi::c_int == Postfix
    {
        istar += 1;
    }
    return istar;
}
unsafe extern "C" fn FindNextStarIndex(
    mut pSS: *mut STAR_SEARCH_STATE,
    mut from: ::core::ffi::c_int,
    mut WithinClust: BOOLEAN,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    if (*pSS).Cluster.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    i = from;
    while i < 502 as ::core::ffi::c_int {
        let mut SDPtr: *mut STAR_DESC = star_array
            .offset((*pSS).SortedStars[i as usize] as isize);
        let mut FullName: [UNICODE; 256] = [0; 256];
        let mut ClusterName: *mut UNICODE = GetStringAddress(
            SetAbsStringTableIndex(GameStrings, (*SDPtr).Postfix as COUNT),
        );
        let mut sptr: *const UNICODE = ::core::ptr::null::<UNICODE>();
        let mut dptr: *const UNICODE = ::core::ptr::null::<UNICODE>();
        let mut dlen: ::core::ffi::c_int = 0;
        let mut c: ::core::ffi::c_int = 0;
        dlen = utf8StringCount(ClusterName) as ::core::ffi::c_int;
        if (*pSS).ClusterLen > dlen {
            i = SkipStarCluster(
                &raw mut (*pSS).SortedStars as *mut ::core::ffi::c_int,
                i,
            ) - 1 as ::core::ffi::c_int;
        } else {
            c = 0 as ::core::ffi::c_int;
            sptr = (*pSS).Cluster;
            dptr = ClusterName;
            while c < (*pSS).ClusterLen {
                let mut sc: UniChar = getCharFromString(&raw mut sptr);
                let mut dc: UniChar = getCharFromString(&raw mut dptr);
                if UniChar_toUpper(sc) != UniChar_toUpper(dc) {
                    break;
                }
                c += 1;
            }
            if c < (*pSS).ClusterLen {
                i = SkipStarCluster(
                    &raw mut (*pSS).SortedStars as *mut ::core::ffi::c_int,
                    i,
                ) - 1 as ::core::ffi::c_int;
            } else if !(!(*pSS).Prefix.is_null() && (*SDPtr).Prefix == 0) {
                if WithinClust.0 != 0 {
                    break;
                }
                if (*pSS).Prefix.is_null() {
                    if i == 0 as ::core::ffi::c_int
                        || (*SDPtr).Postfix as ::core::ffi::c_int
                            != (*star_array
                                .offset(
                                    (*pSS).SortedStars[(i - 1 as ::core::ffi::c_int) as usize]
                                        as isize,
                                ))
                                .Postfix as ::core::ffi::c_int
                    {
                        break;
                    }
                    i = SkipStarCluster(
                        &raw mut (*pSS).SortedStars as *mut ::core::ffi::c_int,
                        i,
                    ) - 1 as ::core::ffi::c_int;
                } else {
                    GetClusterName(SDPtr, &raw mut FullName as *mut UNICODE);
                    dlen = utf8StringCount(&raw mut FullName as *mut UNICODE)
                        as ::core::ffi::c_int;
                    if (*pSS).PrefixLen <= dlen {
                        c = 0 as ::core::ffi::c_int;
                        sptr = (*pSS).Prefix;
                        dptr = &raw mut FullName as *mut UNICODE;
                        while c < (*pSS).PrefixLen {
                            let mut sc_0: UniChar = getCharFromString(&raw mut sptr);
                            let mut dc_0: UniChar = getCharFromString(&raw mut dptr);
                            if UniChar_toUpper(sc_0) != UniChar_toUpper(dc_0) {
                                break;
                            }
                            c += 1;
                        }
                        if c >= (*pSS).PrefixLen {
                            break;
                        }
                    }
                }
            }
        }
        i += 1;
    }
    return if i < 502 as ::core::ffi::c_int { i } else { -1 as ::core::ffi::c_int };
}
unsafe extern "C" fn DrawMatchedStarName(mut pTES: *mut TEXTENTRY_STATE) {
    let mut pSS: *mut STAR_SEARCH_STATE = (*pTES).CbParam as *mut STAR_SEARCH_STATE;
    let mut buf: [UNICODE; 256] = ::core::mem::transmute::<
        [u8; 256],
        [UNICODE; 256],
    >(
        *b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    );
    let mut ExPos: SIZE = 0 as SIZE;
    let mut CurPos: SIZE = -1 as SIZE;
    let mut SDPtr: *mut STAR_DESC = star_array
        .offset((*pSS).SortedStars[(*pSS).CurIndex as usize] as isize);
    let mut flags: COUNT = 0;
    if (*pSS).SingleClust.0 != 0 || (*pSS).SingleMatch.0 != 0 {
        GetClusterName(SDPtr, &raw mut buf as *mut UNICODE);
        ExPos = -1 as SIZE;
        flags = ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as COUNT;
    } else {
        let mut pstr: *mut UNICODE = &raw mut buf as *mut UNICODE;
        strcpy(pstr, &raw mut (*pSS).Text as *mut UNICODE);
        ExPos = (*pSS).ClusterPos as SIZE;
        pstr = skipUTF8Chars(pstr, (*pSS).ClusterPos as size_t) as *mut UNICODE;
        strcpy(
            pstr,
            GetStringAddress(
                SetAbsStringTableIndex(GameStrings, (*SDPtr).Postfix as COUNT),
            ),
        );
        ExPos = (ExPos as ::core::ffi::c_int + (*pSS).ClusterLen) as SIZE;
        CurPos = (*pTES).CursorPos as SIZE;
        flags = ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as COUNT;
        if (*pTES).JoystickMode.0 != 0 {
            flags = (flags as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as COUNT;
        }
    }
    DrawSISMessageEx(&raw mut buf as *mut UNICODE, CurPos, ExPos, flags);
    DrawHyperCoords(cursorLoc);
}
unsafe extern "C" fn MatchNextStar(mut pSS: *mut STAR_SEARCH_STATE, mut Reset: BOOLEAN) {
    if Reset.0 != 0 {
        (*pSS).FirstIndex = -1 as ::core::ffi::c_int;
    }
    if (*pSS).FirstIndex < 0 as ::core::ffi::c_int {
        (*pSS).CurIndex = -1 as ::core::ffi::c_int;
        (*pSS).LastIndex = -1 as ::core::ffi::c_int;
        (*pSS).SingleClust = BOOLEAN::FALSE;
        (*pSS).SingleMatch = BOOLEAN::FALSE;
        strcpy(
            &raw mut (*pSS).Buffer as *mut ::core::ffi::c_char,
            &raw mut (*pSS).Text as *mut UNICODE,
        );
        SplitStarName(pSS);
    }
    (*pSS).CurIndex = FindNextStarIndex(
        pSS,
        (*pSS).CurIndex + 1 as ::core::ffi::c_int,
        (*pSS).SingleClust,
    );
    if (*pSS).FirstIndex < 0 as ::core::ffi::c_int {
        (*pSS).FirstIndex = (*pSS).CurIndex;
    }
    if (*pSS).CurIndex >= 0 as ::core::ffi::c_int {
        (*pSS).LastIndex = (*pSS).CurIndex;
    } else {
        (*pSS).CurIndex = (*pSS).FirstIndex;
        if (*pSS).FirstIndex == (*pSS).LastIndex
            && (*pSS).FirstIndex != -1 as ::core::ffi::c_int
        {
            if (*pSS).Prefix.is_null() {
                (*pSS).SingleClust = BOOLEAN::TRUE;
            } else {
                (*pSS).SingleMatch = BOOLEAN::TRUE;
            }
        }
    };
}
unsafe extern "C" fn OnStarNameChange(mut pTES: *mut TEXTENTRY_STATE) -> BOOLEAN {
    let mut pSS: *mut STAR_SEARCH_STATE = (*pTES).CbParam as *mut STAR_SEARCH_STATE;
    let mut flags: COUNT = 0;
    let mut ret: BOOLEAN = BOOLEAN::TRUE;
    if strcmp(
        &raw mut (*pSS).Text as *mut UNICODE,
        &raw mut (*pSS).LastText as *mut UNICODE,
    ) != 0 as ::core::ffi::c_int
    {
        (*pSS).LastChangeTime = GetTimeCounter() as DWORD;
        strcpy(
            &raw mut (*pSS).LastText as *mut ::core::ffi::c_char,
            &raw mut (*pSS).Text as *mut UNICODE,
        );
        MatchNextStar(pSS, BOOLEAN::TRUE);
    }
    if (*pSS).CurIndex < 0 as ::core::ffi::c_int {
        if (*pSS).Text[0usize] as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
            flags = ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as COUNT;
        } else {
            flags = ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as COUNT;
        }
        if (*pTES).JoystickMode.0 != 0 {
            flags = (flags as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as COUNT;
        }
        ret = DrawSISMessageEx(
            &raw mut (*pSS).Text as *mut UNICODE,
            (*pTES).CursorPos as SIZE,
            -1 as SIZE,
            flags,
        );
    } else {
        let mut SDPtr: *mut STAR_DESC = ::core::ptr::null_mut::<STAR_DESC>();
        SDPtr = star_array.offset((*pSS).SortedStars[(*pSS).CurIndex as usize] as isize);
        UpdateCursorLocation(
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            &raw mut (*SDPtr).star_pt,
        );
        DrawMatchedStarName(pTES);
        UpdateFuelRequirement();
    }
    return ret;
}
unsafe extern "C" fn OnStarNameFrame(mut pTES: *mut TEXTENTRY_STATE) -> BOOLEAN {
    let mut pSS: *mut STAR_SEARCH_STATE = (*pTES).CbParam as *mut STAR_SEARCH_STATE;
    if PulsedInputState
        .menu[C2Rust_Unnamed_1::KEY_MENU_NEXT.0 as ::core::ffi::c_int as usize] != 0
    {
        let mut SDPtr: *mut STAR_DESC = ::core::ptr::null_mut::<STAR_DESC>();
        MatchNextStar(pSS, BOOLEAN::FALSE);
        if (*pSS).CurIndex < 0 as ::core::ffi::c_int {
            if PulsedInputState
                .menu[C2Rust_Unnamed_1::KEY_MENU_NEXT.0 as ::core::ffi::c_int as usize]
                != 0
            {
                PlayMenuSound(MENU_SOUND_EFFECT::MENU_SOUND_FAILURE);
            }
            return BOOLEAN::TRUE;
        }
        SDPtr = star_array.offset((*pSS).SortedStars[(*pSS).CurIndex as usize] as isize);
        UpdateCursorLocation(
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            &raw mut (*SDPtr).star_pt,
        );
        DrawMatchedStarName(pTES);
        UpdateFuelRequirement();
    }
    flashCurrentLocation(::core::ptr::null_mut::<POINT>());
    SleepThread((840 as ::core::ffi::c_int / 30 as ::core::ffi::c_int) as TimePeriod);
    return BOOLEAN::TRUE;
}
unsafe extern "C" fn DoStarSearch(mut pMS: *mut MENU_STATE) -> BOOLEAN {
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
    let mut pss: *mut STAR_SEARCH_STATE = ::core::ptr::null_mut::<STAR_SEARCH_STATE>();
    let mut success: BOOLEAN = BOOLEAN::FALSE;
    pss = HMalloc(::core::mem::size_of::<STAR_SEARCH_STATE>()) as *mut STAR_SEARCH_STATE;
    if pss.is_null() {
        return BOOLEAN::FALSE;
    }
    DrawSISMessageEx(
        b"\0".as_ptr() as *const UNICODE,
        0 as SIZE,
        0 as SIZE,
        ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as COUNT,
    );
    (*pss).pMS = pMS;
    (*pss).LastChangeTime = 0 as DWORD;
    (*pss).Text[0usize] = '\0' as UNICODE;
    (*pss).LastText[0usize] = '\0' as UNICODE;
    (*pss).FirstIndex = -1 as ::core::ffi::c_int;
    SortStarsOnName(pss);
    tes.Initialized = BOOLEAN::FALSE;
    tes.BaseStr = &raw mut (*pss).Text as *mut UNICODE;
    tes.MaxSize = ::core::mem::size_of::<[UNICODE; 256]>() as ::core::ffi::c_int;
    tes.CursorPos = 0 as ::core::ffi::c_int;
    tes.CbParam = pss as *mut ::core::ffi::c_void;
    tes.ChangeCallback = Some(
        OnStarNameChange as unsafe extern "C" fn(*mut TEXTENTRY_STATE) -> BOOLEAN,
    ) as Option<unsafe extern "C" fn(*mut textentry_state) -> BOOLEAN>;
    tes.FrameCallback = Some(
        OnStarNameFrame as unsafe extern "C" fn(*mut TEXTENTRY_STATE) -> BOOLEAN,
    ) as Option<unsafe extern "C" fn(*mut textentry_state) -> BOOLEAN>;
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
    SetDefaultMenuRepeatDelay();
    success = DoTextEntry(&raw mut tes);
    DrawSISMessageEx(
        &raw mut (*pss).Text as *mut UNICODE,
        -1 as SIZE,
        -1 as SIZE,
        ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as COUNT,
    );
    HFree(pss as *mut ::core::ffi::c_void);
    return success;
}
unsafe extern "C" fn DoMoveCursor(mut pMS: *mut MENU_STATE) -> BOOLEAN {
    static mut last_buf: [UNICODE; 256] = [0; 256];
    let mut TimeIn: DWORD = GetTimeCounter();
    if (*pMS).Initialized == 0 {
        let mut universe: POINT = POINT { x: 0, y: 0 };
        (*pMS).Initialized = BOOLEAN::TRUE.0 as ::core::ffi::c_int as SIZE;
        (*pMS).InputFunc = Some(
            DoMoveCursor as unsafe extern "C" fn(*mut MENU_STATE) -> BOOLEAN,
        ) as Option<unsafe extern "C" fn(*mut menu_state) -> BOOLEAN>;
        if inHQSpace().0 == 0 {
            universe = (*CurStarDescPtr).star_pt;
        } else {
            universe.x = logxToUniverse(GlobData.SIS_state.log_x);
            universe.y = logyToUniverse(GlobData.SIS_state.log_y);
        }
        flashCurrentLocation(&raw mut universe);
        last_buf[0usize] = '\0' as UNICODE;
        UpdateCursorInfo(&raw mut last_buf as *mut UNICODE);
        UpdateFuelRequirement();
        return BOOLEAN::TRUE;
    } else if PulsedInputState
        .menu[C2Rust_Unnamed_1::KEY_MENU_CANCEL.0 as ::core::ffi::c_int as usize] != 0
    {
        return BOOLEAN::FALSE
    } else if PulsedInputState
        .menu[C2Rust_Unnamed_1::KEY_MENU_SELECT.0 as ::core::ffi::c_int as usize] != 0
    {
        GlobData.Game_state.autopilot = cursorLoc;
        if instantMove.0 != 0 {
            PlayMenuSound(MENU_SOUND_EFFECT::MENU_SOUND_INVOKED);
            if inHQSpace().0 != 0 {
                doInstantMove();
            } else if GlobData.Game_state.CurrentActivity as BYTE as ::core::ffi::c_int
                == C2Rust_Unnamed_4::IN_INTERPLANETARY.0 as ::core::ffi::c_int
            {
                GlobData.Game_state.CurrentActivity = (GlobData
                    .Game_state
                    .CurrentActivity as ::core::ffi::c_int
                    | C2Rust_Unnamed_4::START_INTERPLANETARY.0 as ::core::ffi::c_int)
                    as ACTIVITY;
                ::core::ptr::write_volatile(
                    &raw mut debugHook,
                    Some(doInstantMove as unsafe extern "C" fn() -> ())
                        as Option<unsafe extern "C" fn() -> ()>,
                );
            }
            return BOOLEAN::FALSE;
        }
        DrawStarMap(0 as COUNT, ::core::ptr::null_mut::<RECT>());
    } else if PulsedInputState
        .menu[C2Rust_Unnamed_1::KEY_MENU_SEARCH.0 as ::core::ffi::c_int as usize] != 0
    {
        if getGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_3::ARILOU_SPACE_SIDE.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_3::END_ARILOU_SPACE_SIDE.0 as ::core::ffi::c_int,
        ) as ::core::ffi::c_int <= 1 as ::core::ffi::c_int
        {
            let mut oldpt: POINT = cursorLoc;
            if DoStarSearch(pMS).0 == 0 {
                UpdateCursorLocation(
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    &raw mut oldpt,
                );
            }
            strcpy(
                &raw mut last_buf as *mut ::core::ffi::c_char,
                b"  <random garbage>  \0".as_ptr() as *const ::core::ffi::c_char,
            );
            UpdateCursorInfo(&raw mut last_buf as *mut UNICODE);
            UpdateFuelRequirement();
            SetMenuRepeatDelay(
                (840 as ::core::ffi::c_int / 60 as ::core::ffi::c_int) as DWORD,
                (840 as ::core::ffi::c_int / 8 as ::core::ffi::c_int) as DWORD,
                (840 as ::core::ffi::c_int / 120 as ::core::ffi::c_int) as DWORD,
                BOOLEAN::TRUE,
            );
            SetMenuSounds(
                0 as ::core::ffi::c_int as MENU_SOUND_FLAGS,
                0 as ::core::ffi::c_int as MENU_SOUND_FLAGS,
            );
        } else {
            PlayMenuSound(MENU_SOUND_EFFECT::MENU_SOUND_FAILURE);
        }
    } else {
        let mut sx: SBYTE = 0;
        let mut sy: SBYTE = 0;
        let mut ZoomIn: SIZE = 0;
        let mut ZoomOut: SIZE = 0;
        ZoomOut = 0 as SIZE;
        ZoomIn = ZoomOut;
        if PulsedInputState
            .menu[C2Rust_Unnamed_1::KEY_MENU_ZOOM_IN.0 as ::core::ffi::c_int as usize]
            != 0
        {
            ZoomIn = 1 as SIZE;
        } else if PulsedInputState
            .menu[C2Rust_Unnamed_1::KEY_MENU_ZOOM_OUT.0 as ::core::ffi::c_int as usize]
            != 0
        {
            ZoomOut = 1 as SIZE;
        }
        ZoomStarMap(
            (ZoomIn as ::core::ffi::c_int - ZoomOut as ::core::ffi::c_int) as SIZE,
        );
        sy = 0 as SBYTE;
        sx = sy;
        if PulsedInputState
            .menu[C2Rust_Unnamed_1::KEY_MENU_LEFT.0 as ::core::ffi::c_int as usize] != 0
        {
            sx = -1 as SBYTE;
        }
        if PulsedInputState
            .menu[C2Rust_Unnamed_1::KEY_MENU_RIGHT.0 as ::core::ffi::c_int as usize] != 0
        {
            sx = 1 as SBYTE;
        }
        if PulsedInputState
            .menu[C2Rust_Unnamed_1::KEY_MENU_UP.0 as ::core::ffi::c_int as usize] != 0
        {
            sy = -1 as SBYTE;
        }
        if PulsedInputState
            .menu[C2Rust_Unnamed_1::KEY_MENU_DOWN.0 as ::core::ffi::c_int as usize] != 0
        {
            sy = 1 as SBYTE;
        }
        if sx as ::core::ffi::c_int != 0 as ::core::ffi::c_int
            || sy as ::core::ffi::c_int != 0 as ::core::ffi::c_int
        {
            UpdateCursorLocation(
                sx as ::core::ffi::c_int,
                sy as ::core::ffi::c_int,
                ::core::ptr::null::<POINT>(),
            );
            UpdateCursorInfo(&raw mut last_buf as *mut UNICODE);
            UpdateFuelRequirement();
        }
        SleepThreadUntil(
            TimeIn
                .wrapping_add(
                    (840 as ::core::ffi::c_int / 60 as ::core::ffi::c_int) as TimeCount,
                ),
        );
    }
    flashCurrentLocation(::core::ptr::null_mut::<POINT>());
    return BOOLEAN(
        (GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
            & C2Rust_Unnamed_4::CHECK_ABORT.0 as ::core::ffi::c_int == 0)
            as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
}
unsafe extern "C" fn RepairMap(
    mut update_race: COUNT,
    mut pLastRect: *mut RECT,
    mut pNextRect: *mut RECT,
) {
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    r.extent.width = 50 as COORD;
    r.corner.x = ((*pNextRect).corner.x as ::core::ffi::c_int
        + ((*pNextRect).extent.width as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
        - (r.extent.width as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)) as COORD;
    if (r.corner.x as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
        r.corner.x = 0 as COORD;
    } else if r.corner.x as ::core::ffi::c_int + r.extent.width as ::core::ffi::c_int
        >= ScreenWidth - 64 as ::core::ffi::c_int
            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
            - 14 as ::core::ffi::c_int
    {
        r.corner.x = (ScreenWidth - 64 as ::core::ffi::c_int
            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
            - 14 as ::core::ffi::c_int - r.extent.width as ::core::ffi::c_int) as COORD;
    }
    r.extent.height = 9 as COORD;
    r.corner.y = ((*pNextRect).corner.y as ::core::ffi::c_int
        + ((*pNextRect).extent.height as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
        - r.extent.height as ::core::ffi::c_int) as COORD;
    if (r.corner.y as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
        r.corner.y = 0 as COORD;
    } else if r.corner.y as ::core::ffi::c_int + r.extent.height as ::core::ffi::c_int
        >= ScreenHeight - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
            - 13 as ::core::ffi::c_int
    {
        r.corner.y = (ScreenHeight - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
            - 13 as ::core::ffi::c_int - r.extent.height as ::core::ffi::c_int) as COORD;
    }
    BoxUnion(pLastRect, &raw mut r, &raw mut r);
    BoxUnion(pNextRect, &raw mut r, &raw mut r);
    *pLastRect = *pNextRect;
    if (r.corner.x as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
        r.extent.width = (r.extent.width as ::core::ffi::c_int
            + r.corner.x as ::core::ffi::c_int) as COORD;
        r.corner.x = 0 as COORD;
    }
    if r.corner.x as ::core::ffi::c_int + r.extent.width as ::core::ffi::c_int
        > ScreenWidth - 64 as ::core::ffi::c_int
            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
            - 14 as ::core::ffi::c_int
    {
        r.extent.width = (ScreenWidth - 64 as ::core::ffi::c_int
            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
            - 14 as ::core::ffi::c_int - r.corner.x as ::core::ffi::c_int) as COORD;
    }
    if (r.corner.y as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
        r.extent.height = (r.extent.height as ::core::ffi::c_int
            + r.corner.y as ::core::ffi::c_int) as COORD;
        r.corner.y = 0 as COORD;
    }
    if r.corner.y as ::core::ffi::c_int + r.extent.height as ::core::ffi::c_int
        > ScreenHeight - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
            - 13 as ::core::ffi::c_int
    {
        r.extent.height = (ScreenHeight
            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
            - 13 as ::core::ffi::c_int - r.corner.y as ::core::ffi::c_int) as COORD;
    }
    r.extent.height = (r.extent.height as ::core::ffi::c_int
        + (r.corner.y as ::core::ffi::c_int & 1 as ::core::ffi::c_int)) as COORD;
    r.corner.y = (r.corner.y as ::core::ffi::c_int & !(1 as ::core::ffi::c_int))
        as COORD;
    DrawStarMap(update_race, &raw mut r);
}
unsafe extern "C" fn UpdateMap() {
    let mut ButtonState: BYTE = 0;
    let mut VisibleChange: BYTE = 0;
    let mut MapDrawn: BOOLEAN = BOOLEAN::FALSE;
    let mut Interrupted: BOOLEAN = BOOLEAN::FALSE;
    let mut index: COUNT = 0;
    let mut hStarShip: HFLEETINFO = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut hNextShip: HFLEETINFO = ::core::ptr::null_mut::<::core::ffi::c_void>();
    FlushInput();
    ButtonState = 1 as BYTE;
    Interrupted = BOOLEAN::FALSE;
    MapDrawn = Interrupted;
    index = 1 as COUNT;
    hStarShip = GlobData.Game_state.avail_race_q.head as HFLEETINFO;
    while !hStarShip.is_null() {
        let mut FleetPtr: *mut FLEET_INFO = ::core::ptr::null_mut::<FLEET_INFO>();
        FleetPtr = LockFleetInfo(&raw mut GlobData.Game_state.avail_race_q, hStarShip);
        hNextShip = (*FleetPtr).succ;
        if ButtonState != 0 {
            if AnyButtonPress(BOOLEAN::TRUE).0 == 0 {
                ButtonState = 0 as BYTE;
            }
        } else {
            Interrupted = BOOLEAN(
                (Interrupted.0 != 0 || AnyButtonPress(BOOLEAN::TRUE).0 != 0)
                    as ::core::ffi::c_int as ::core::ffi::c_uint,
            );
            if Interrupted.0 != 0 {
                MapDrawn = BOOLEAN::TRUE;
            }
        }
        if (*FleetPtr).known_strength != 0 {
            let mut dx: SIZE = 0;
            let mut dy: SIZE = 0;
            let mut delta: SIZE = 0;
            let mut r: RECT = RECT {
                corner: POINT { x: 0, y: 0 },
                extent: EXTENT { width: 0, height: 0 },
            };
            let mut last_r: RECT = RECT {
                corner: POINT { x: 0, y: 0 },
                extent: EXTENT { width: 0, height: 0 },
            };
            let mut temp_r0: RECT = RECT {
                corner: POINT { x: 0, y: 0 },
                extent: EXTENT { width: 0, height: 0 },
            };
            let mut temp_r1: RECT = RECT {
                corner: POINT { x: 0, y: 0 },
                extent: EXTENT { width: 0, height: 0 },
            };
            dx = ((*FleetPtr).loc.x as ::core::ffi::c_int
                - (*FleetPtr).known_loc.x as ::core::ffi::c_int) as SIZE;
            dy = ((*FleetPtr).loc.y as ::core::ffi::c_int
                - (*FleetPtr).known_loc.y as ::core::ffi::c_int) as SIZE;
            if dx as ::core::ffi::c_int != 0 || dy as ::core::ffi::c_int != 0 {
                let mut xincr: SIZE = 0;
                let mut yincr: SIZE = 0;
                let mut xerror: SIZE = 0;
                let mut yerror: SIZE = 0;
                let mut cycle: SIZE = 0;
                if dx as ::core::ffi::c_int >= 0 as ::core::ffi::c_int {
                    xincr = 1 as SIZE;
                } else {
                    xincr = -1 as SIZE;
                    dx = -(dx as ::core::ffi::c_int) as SIZE;
                }
                dx = ((dx as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as SIZE;
                if dy as ::core::ffi::c_int >= 0 as ::core::ffi::c_int {
                    yincr = 1 as SIZE;
                } else {
                    yincr = -1 as SIZE;
                    dy = -(dy as ::core::ffi::c_int) as SIZE;
                }
                dy = ((dy as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as SIZE;
                if dx as ::core::ffi::c_int >= dy as ::core::ffi::c_int {
                    cycle = dx;
                } else {
                    cycle = dy;
                }
                yerror = (cycle as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
                    as SIZE;
                xerror = yerror;
                delta = xerror;
                if MapDrawn.0 == 0 {
                    DrawStarMap(
                        !(0 as ::core::ffi::c_int) as COUNT,
                        ::core::ptr::null_mut::<RECT>(),
                    );
                    MapDrawn = BOOLEAN::TRUE;
                }
                GetSphereRect(FleetPtr, &raw mut temp_r0, &raw mut last_r);
                last_r.extent.width += 1;
                last_r.extent.height += 1;
                VisibleChange = BOOLEAN::FALSE.0 as ::core::ffi::c_int as BYTE;
                '_DoneSphereMove: {
                    loop {
                        loop {
                            xerror = (xerror as ::core::ffi::c_int
                                - dx as ::core::ffi::c_int) as SIZE;
                            if xerror as ::core::ffi::c_int <= 0 as ::core::ffi::c_int {
                                (*FleetPtr).known_loc.x = ((*FleetPtr).known_loc.x
                                    as ::core::ffi::c_int + xincr as ::core::ffi::c_int)
                                    as COORD;
                                xerror = (xerror as ::core::ffi::c_int
                                    + cycle as ::core::ffi::c_int) as SIZE;
                            }
                            yerror = (yerror as ::core::ffi::c_int
                                - dy as ::core::ffi::c_int) as SIZE;
                            if yerror as ::core::ffi::c_int <= 0 as ::core::ffi::c_int {
                                (*FleetPtr).known_loc.y = ((*FleetPtr).known_loc.y
                                    as ::core::ffi::c_int + yincr as ::core::ffi::c_int)
                                    as COORD;
                                yerror = (yerror as ::core::ffi::c_int
                                    + cycle as ::core::ffi::c_int) as SIZE;
                            }
                            GetSphereRect(FleetPtr, &raw mut temp_r1, &raw mut r);
                            let c2rust_fresh1 = delta;
                            delta -= 1;
                            if !(c2rust_fresh1 as ::core::ffi::c_int != 0
                                && (delta as ::core::ffi::c_int & 0x1f as ::core::ffi::c_int
                                    != 0
                                    || temp_r0.corner.x as ::core::ffi::c_int
                                        == temp_r1.corner.x as ::core::ffi::c_int
                                        && temp_r0.corner.y as ::core::ffi::c_int
                                            == temp_r1.corner.y as ::core::ffi::c_int))
                            {
                                break;
                            }
                        }
                        if ButtonState != 0 {
                            if AnyButtonPress(BOOLEAN::TRUE).0 == 0 {
                                ButtonState = 0 as BYTE;
                            }
                        } else {
                            Interrupted = BOOLEAN(
                                (Interrupted.0 != 0 || AnyButtonPress(BOOLEAN::TRUE).0 != 0)
                                    as ::core::ffi::c_int as ::core::ffi::c_uint,
                            );
                            if Interrupted.0 != 0 {
                                MapDrawn = BOOLEAN::TRUE;
                                break '_DoneSphereMove;
                            }
                        }
                        r.extent.width += 1;
                        r.extent.height += 1;
                        if temp_r0.corner.x as ::core::ffi::c_int
                            != temp_r1.corner.x as ::core::ffi::c_int
                            || temp_r0.corner.y as ::core::ffi::c_int
                                != temp_r1.corner.y as ::core::ffi::c_int
                        {
                            VisibleChange = BOOLEAN::TRUE.0 as ::core::ffi::c_int
                                as BYTE;
                            RepairMap(index, &raw mut last_r, &raw mut r);
                        }
                        if (delta as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                            break;
                        }
                    }
                    if VisibleChange != 0 {
                        RepairMap(
                            !(0 as ::core::ffi::c_int) as COUNT,
                            &raw mut last_r,
                            &raw mut r,
                        );
                    }
                }
                (*FleetPtr).known_loc = (*FleetPtr).loc;
            }
            delta = ((*FleetPtr).actual_strength as ::core::ffi::c_int
                - (*FleetPtr).known_strength as ::core::ffi::c_int) as SIZE;
            if delta != 0 {
                if MapDrawn.0 == 0 {
                    DrawStarMap(
                        !(0 as ::core::ffi::c_int) as COUNT,
                        ::core::ptr::null_mut::<RECT>(),
                    );
                    MapDrawn = BOOLEAN::TRUE;
                }
                if delta as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                    dx = 1 as SIZE;
                } else {
                    delta = -(delta as ::core::ffi::c_int) as SIZE;
                    dx = -1 as SIZE;
                }
                delta -= 1;
                GetSphereRect(FleetPtr, &raw mut temp_r0, &raw mut last_r);
                last_r.extent.width += 1;
                last_r.extent.height += 1;
                VisibleChange = BOOLEAN::FALSE.0 as ::core::ffi::c_int as BYTE;
                '_DoneSphereGrowth: {
                    loop {
                        loop {
                            (*FleetPtr).known_strength = ((*FleetPtr).known_strength
                                as ::core::ffi::c_int + dx as ::core::ffi::c_int) as COUNT;
                            GetSphereRect(FleetPtr, &raw mut temp_r1, &raw mut r);
                            let c2rust_fresh2 = delta;
                            delta -= 1;
                            if !(c2rust_fresh2 as ::core::ffi::c_int != 0
                                && (delta as ::core::ffi::c_int & 0xf as ::core::ffi::c_int
                                    != 0
                                    || temp_r0.extent.height as ::core::ffi::c_int
                                        == temp_r1.extent.height as ::core::ffi::c_int))
                            {
                                break;
                            }
                        }
                        if ButtonState != 0 {
                            if AnyButtonPress(BOOLEAN::TRUE).0 == 0 {
                                ButtonState = 0 as BYTE;
                            }
                        } else {
                            Interrupted = BOOLEAN(
                                (Interrupted.0 != 0 || AnyButtonPress(BOOLEAN::TRUE).0 != 0)
                                    as ::core::ffi::c_int as ::core::ffi::c_uint,
                            );
                            if Interrupted.0 != 0 {
                                MapDrawn = BOOLEAN::TRUE;
                                break '_DoneSphereGrowth;
                            }
                        }
                        r.extent.width += 1;
                        r.extent.height += 1;
                        if temp_r0.extent.height as ::core::ffi::c_int
                            != temp_r1.extent.height as ::core::ffi::c_int
                        {
                            VisibleChange = BOOLEAN::TRUE.0 as ::core::ffi::c_int
                                as BYTE;
                            RepairMap(index, &raw mut last_r, &raw mut r);
                        }
                        if (delta as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                            break;
                        }
                    }
                    if VisibleChange as ::core::ffi::c_int != 0
                        || temp_r0.extent.width as ::core::ffi::c_int
                            != temp_r1.extent.width as ::core::ffi::c_int
                    {
                        RepairMap(
                            !(0 as ::core::ffi::c_int) as COUNT,
                            &raw mut last_r,
                            &raw mut r,
                        );
                    }
                }
                (*FleetPtr).known_strength = (*FleetPtr).actual_strength;
            }
        }
        UnlockLink(&raw mut GlobData.Game_state.avail_race_q, hStarShip);
        index = index.wrapping_add(1);
        hStarShip = hNextShip;
    }
}
#[no_mangle]
pub unsafe extern "C" fn StarMap() -> BOOLEAN {
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
    let mut universe: POINT = POINT { x: 0, y: 0 };
    let mut clip_r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut OldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    memset(
        &raw mut MenuState as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<MENU_STATE>(),
    );
    zoomLevel = 0 as ::core::ffi::c_int;
    mapOrigin.x = (9999 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as COORD;
    mapOrigin.y = (9999 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as COORD;
    StarMapFrame = SetAbsFrameIndex(MiscDataFrame, 48 as COUNT);
    if inHQSpace().0 == 0 {
        universe = (*CurStarDescPtr).star_pt;
    } else {
        universe.x = logxToUniverse(GlobData.SIS_state.log_x);
        universe.y = logyToUniverse(GlobData.SIS_state.log_y);
    }
    cursorLoc = GlobData.Game_state.autopilot;
    if cursorLoc.x as ::core::ffi::c_int == !(0 as ::core::ffi::c_int)
        && cursorLoc.y as ::core::ffi::c_int == !(0 as ::core::ffi::c_int)
    {
        cursorLoc = universe;
    }
    MenuState.InputFunc = Some(
        DoMoveCursor as unsafe extern "C" fn(*mut MENU_STATE) -> BOOLEAN,
    ) as Option<unsafe extern "C" fn(*mut menu_state) -> BOOLEAN>;
    MenuState.Initialized = BOOLEAN::FALSE.0 as ::core::ffi::c_int as SIZE;
    transition_pending = BOOLEAN::TRUE;
    if getGameState(
        &raw mut GlobData.Game_state.GameState as *mut BYTE,
        C2Rust_Unnamed_3::ARILOU_SPACE_SIDE.0 as ::core::ffi::c_int,
        C2Rust_Unnamed_3::END_ARILOU_SPACE_SIDE.0 as ::core::ffi::c_int,
    ) as ::core::ffi::c_int <= 1 as ::core::ffi::c_int
    {
        UpdateMap();
    }
    DrawStarMap(
        0 as COUNT,
        ::core::ptr::with_exposed_provenance_mut::<RECT>(-1 as ::core::ffi::c_int as usize),
    );
    transition_pending = BOOLEAN::FALSE;
    BatchGraphics();
    OldContext = SetContext(SpaceContext);
    GetContextClipRect(&raw mut clip_r);
    SetContext(OldContext);
    LoadIntoExtraScreen(&raw mut clip_r);
    DrawCursor(
        universeToDispx(cursorLoc.x as ::core::ffi::c_long),
        universeToDispy(cursorLoc.y as ::core::ffi::c_long),
    );
    UnbatchGraphics();
    SetMenuSounds(
        0 as ::core::ffi::c_int as MENU_SOUND_FLAGS,
        0 as ::core::ffi::c_int as MENU_SOUND_FLAGS,
    );
    SetMenuRepeatDelay(
        (840 as ::core::ffi::c_int / 60 as ::core::ffi::c_int) as DWORD,
        (840 as ::core::ffi::c_int / 8 as ::core::ffi::c_int) as DWORD,
        (840 as ::core::ffi::c_int / 120 as ::core::ffi::c_int) as DWORD,
        BOOLEAN::TRUE,
    );
    DoInput(&raw mut MenuState as *mut ::core::ffi::c_void, BOOLEAN::FALSE);
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
    SetDefaultMenuRepeatDelay();
    DrawHyperCoords(universe);
    DrawSISMessage(::core::ptr::null::<UNICODE>());
    DrawStatusMessage(::core::ptr::null::<UNICODE>());
    if GlobData.Game_state.autopilot.x as ::core::ffi::c_int
        == universe.x as ::core::ffi::c_int
        && GlobData.Game_state.autopilot.y as ::core::ffi::c_int
            == universe.y as ::core::ffi::c_int
    {
        GlobData.Game_state.autopilot.y = !(0 as ::core::ffi::c_int) as COORD;
        GlobData.Game_state.autopilot.x = GlobData.Game_state.autopilot.y;
    }
    return BOOLEAN(
        (GlobData.Game_state.autopilot.x as ::core::ffi::c_int
            != !(0 as ::core::ffi::c_int)
            && GlobData.Game_state.autopilot.y as ::core::ffi::c_int
                != !(0 as ::core::ffi::c_int)) as ::core::ffi::c_int
            as ::core::ffi::c_uint,
    );
}
