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
    pub type font_desc;
    pub type tfb_soundsample;
    pub type FlashContext;
    fn SetContext(Context: CONTEXT) -> CONTEXT;
    fn SetContextForeGroundColor(Color: Color) -> Color;
    fn GetContextClipRect(pRect: *mut RECT) -> BOOLEAN;
    fn DrawStamp(pStamp: *mut STAMP);
    fn DrawFilledRectangle(pRect: *mut RECT);
    fn font_DrawText(pText: *mut TEXT);
    fn font_DrawShadowedText(
        pText: *mut TEXT,
        direction: BYTE,
        text_color: Color,
        shadow_color: Color,
    );
    fn BatchGraphics();
    fn UnbatchGraphics();
    fn SetContextFont(Font: FONT) -> FONT;
    fn GetContextFontLeading(pheight: *mut SIZE) -> BOOLEAN;
    fn SetAbsFrameIndex(Frame: FRAME, FrameIndex: COUNT) -> FRAME;
    fn SetRelFrameIndex(Frame: FRAME, FrameOffs: SIZE) -> FRAME;
    fn snprintf(
        _: *mut ::core::ffi::c_char,
        _: size_t,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn SetAbsStringTableIndex(String: STRING, StringTableIndex: COUNT) -> STRING;
    fn GetStringAddress(String: STRING) -> STRINGPTR;
    fn utf8StringCopy(
        dst: *mut ::core::ffi::c_char,
        size: size_t,
        src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_uchar;
    fn AlignText(str: *const UNICODE, loc_x: *mut sint16) -> *mut UNICODE;
    fn strlen(_: *const ::core::ffi::c_char) -> size_t;
    static mut PulsedInputState: CONTROLLER_INPUT_STATE;
    static mut optWhichMenu: ::core::ffi::c_int;
    static mut optWhichFonts: ::core::ffi::c_int;
    static mut optSmoothScroll: ::core::ffi::c_int;
    static mut resolutionFactor: ::core::ffi::c_uint;
    static mut optSubmenu: OPT_ENABLABLE;
    static mut optCustomBorder: OPT_ENABLABLE;
    static mut optWholeFuel: OPT_ENABLABLE;
    static mut optWindowType: DWORD;
    static mut CanvasWidth: ::core::ffi::c_int;
    static mut CanvasHeight: ::core::ffi::c_int;
    fn PreUpdateFlashRect();
    fn PostUpdateFlashRect();
    fn DeltaSISGauges(
        crew_delta: SIZE,
        fuel_delta: SDWORD,
        resunit_delta: ::core::ffi::c_int,
    );
    fn GetStorageBayCapacity() -> COUNT;
    static mut GlobData: GLOBDATA;
    fn isPC(optWhich: ::core::ffi::c_int) -> OPT_CONSOLETYPE;
    static mut MiscDataFrame: FRAME;
    static mut BorderFrame: FRAME;
    static mut HDBorderFrame: FRAME;
    static mut CustBevelFrame: FRAME;
    static mut ScreenContext: CONTEXT;
    static mut StatusContext: CONTEXT;
    static mut StarConFont: FONT;
    static mut TinyFont: FONT;
    static mut TinyFontBold: FONT;
    static mut PlayMenuFont: FONT;
    static mut usingSpeech: BOOLEAN;
    static mut GameStrings: STRING;
    fn log_add(_: log_Level, fmt: *const ::core::ffi::c_char, ...);
    fn DrawStarConBox(
        pRect: *mut RECT,
        BorderWidth: SIZE,
        TopLeftColor: Color,
        BottomRightColor: Color,
        FillInterior: BOOLEAN,
        InteriorColor: Color,
        CreateCorners: BOOLEAN,
        CornerColor: Color,
    );
    fn DrawRenderedBox(
        r: *mut RECT,
        filled: BOOLEAN,
        fill_color: Color,
        r#type: ::core::ffi::c_int,
        custom: ::core::ffi::c_int,
    );
    fn DrawFlagStatDisplay(str: *mut UNICODE);
    fn WholeFuelValue() -> *mut UNICODE;
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
    pub flags: BYTE,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed {
    pub const NORTH_SHADOW: Self = Self(0);
    pub const NORTH_EAST_SHADOW: Self = Self(1);
    pub const EAST_SHADOW: Self = Self(2);
    pub const SOUTH_EAST_SHADOW: Self = Self(3);
    pub const SOUTH_SHADOW: Self = Self(4);
    pub const SOUTH_WEST_SHADOW: Self = Self(5);
    pub const WEST_SHADOW: Self = Self(6);
    pub const NORTH_WEST_SHADOW: Self = Self(7);
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
    pub flash_rect2: RECT,
    pub flash_frame0: FRAME,
    pub flash_frame1: FRAME,
    pub flashContext: *mut FlashContext,
    pub hMusic: MUSIC_REF,
    pub privData: *mut ::core::ffi::c_void,
}
pub type MENU_STATE = menu_state;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_0 {
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
    pub const PM_READ_VERY_SLOW: Self = Self(36);
    pub const PM_READ_SLOW: Self = Self(37);
    pub const PM_READ_MODERATE: Self = Self(38);
    pub const PM_READ_FAST: Self = Self(39);
    pub const PM_READ_VERY_FAST: Self = Self(40);
    pub const PM_CHANGE_CAPTAIN: Self = Self(41);
    pub const PM_CHANGE_SHIP: Self = Self(42);
    pub const PM_EXIT_SETTINGS: Self = Self(43);
    pub const PM_ALT_SCAN: Self = Self(44);
    pub const PM_ALT_STARMAP: Self = Self(45);
    pub const PM_ALT_MANIFEST: Self = Self(46);
    pub const PM_ALT_GAME_MENU: Self = Self(47);
    pub const PM_ALT_NAVIGATE: Self = Self(48);
    pub const PM_ALT_CARGO: Self = Self(49);
    pub const PM_ALT_DEVICES: Self = Self(50);
    pub const PM_ALT_ROSTER: Self = Self(51);
    pub const PM_ALT_EXIT_MANIFEST: Self = Self(52);
    pub const PM_ALT_MSCAN: Self = Self(53);
    pub const PM_ALT_ESCAN: Self = Self(54);
    pub const PM_ALT_BSCAN: Self = Self(55);
    pub const PM_ALT_ASCAN: Self = Self(56);
    pub const PM_ALT_DISPATCH: Self = Self(57);
    pub const PM_ALT_EXIT_SCAN: Self = Self(58);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_1(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_1 {
    pub const GAMESTR_GAME_MENU: Self = Self(5);
    pub const GAMESTR_EXIT_MENU: Self = Self(10);
    pub const GAMESTR_CONVERSE: Self = Self(17);
    pub const GAMESTR_ATTACK: Self = Self(18);
    pub const GAMESTR_FUEL: Self = Self(19);
    pub const GAMESTR_MODULE: Self = Self(20);
    pub const GAMESTR_CREW: Self = Self(21);
    pub const GAMESTR_SND_ON: Self = Self(22);
    pub const GAMESTR_SND_OFF: Self = Self(23);
    pub const GAMESTR_MUS_ON: Self = Self(24);
    pub const GAMESTR_MUS_OFF: Self = Self(25);
    pub const GAMESTR_COMBAT: Self = Self(26);
    pub const GAMESTR_READING: Self = Self(27);
    pub const GAMESTR_CHANGE_CAP: Self = Self(28);
    pub const GAMESTR_CHANGE_SIS: Self = Self(29);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct OPT_ENABLABLE(pub ::core::ffi::c_uint);
impl OPT_ENABLABLE {
    pub const OPTVAL_DISABLED: Self = Self(0);
    pub const OPTVAL_ENABLED: Self = Self(1);
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
    pub stowed_ship_q: QUEUE,
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
    pub ShipName: [UNICODE; 32],
    pub CommanderName: [UNICODE; 32],
    pub PlanetName: [UNICODE; 32],
    pub Difficulty: BYTE,
    pub Extended: BYTE,
    pub Nomad: BYTE,
    pub ShipSeed: BYTE,
    pub Seed: SDWORD,
    pub SaveVersion: BYTE,
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
    pub key: [[::core::ffi::c_int; 8]; 2],
    pub menu: [::core::ffi::c_int; 31],
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_2(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_2 {
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
    pub const KEY_MENU_TOGGLEMAP: Self = Self(23);
    pub const KEY_SCREENSHOT: Self = Self(24);
    pub const KEY_QUICKSAVE: Self = Self(25);
    pub const KEY_QUICKLOAD: Self = Self(26);
    pub const KEY_DEBUG_2: Self = Self(27);
    pub const KEY_DEBUG_3: Self = Self(28);
    pub const KEY_DEBUG_4: Self = Self(29);
    pub const KEY_MENU_ANY: Self = Self(30);
    pub const NUM_MENU_KEYS: Self = Self(31);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_3(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_3 {
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
pub struct C2Rust_Unnamed_4(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_4 {
    pub const SIS_MAIN_FRAME: Self = Self(0);
    pub const SIS_STAT_FRAME: Self = Self(1);
    pub const DEVICE_CARGO_FRAME: Self = Self(2);
    pub const SIS_STAT_REPAIR_FRAME: Self = Self(3);
    pub const SIS_STAT_SUMM_FRAME: Self = Self(4);
    pub const SIS_MSG_FRAME: Self = Self(5);
    pub const SIS_TITLE_FRAME: Self = Self(6);
    pub const CAPTAIN_FRAME: Self = Self(7);
    pub const CAP_NAME_FRAME: Self = Self(8);
    pub const STAT_MSG_FRAME: Self = Self(9);
    pub const SIS_RADAR_FRAME: Self = Self(10);
    pub const TEXT_LABEL_FRAME: Self = Self(11);
    pub const SIS_REPAIR_FRAME: Self = Self(12);
    pub const LANDER_3DO_FRAME: Self = Self(13);
    pub const LANDER_DOS_FRAME: Self = Self(14);
    pub const SIS_COMM_FRAME: Self = Self(15);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_5(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_5 {
    pub const DOS_MENU_HILITE: Self = Self(0);
    pub const SPECIAL_BEVEL: Self = Self(1);
    pub const THIN_INNER_BEVEL: Self = Self(9);
    pub const THICK_INNER_BEVEL: Self = Self(17);
    pub const THIN_OUTER_BEVEL: Self = Self(25);
    pub const THICK_OUTER_BEVEL: Self = Self(33);
    pub const THICK_OUT_INV_BEVEL: Self = Self(41);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_6(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_6 {
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
pub struct C2Rust_Unnamed_7(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_7 {
    pub const STAR_STRING_BASE: Self = Self(0);
    pub const DEVICE_STRING_BASE: Self = Self(149);
    pub const CARGO_STRING_BASE: Self = Self(178);
    pub const ELEMENTS_STRING_BASE: Self = Self(192);
    pub const SCAN_STRING_BASE: Self = Self(325);
    pub const STAR_NUMBER_BASE: Self = Self(391);
    pub const PLANET_NUMBER_BASE: Self = Self(405);
    pub const MONTHS_STRING_BASE: Self = Self(448);
    pub const FEEDBACK_STRING_BASE: Self = Self(460);
    pub const STARBASE_STRING_BASE: Self = Self(468);
    pub const ENCOUNTER_STRING_BASE: Self = Self(510);
    pub const NAVIGATION_STRING_BASE: Self = Self(518);
    pub const NAMING_STRING_BASE: Self = Self(527);
    pub const MELEE_STRING_BASE: Self = Self(535);
    pub const SAVEGAME_STRING_BASE: Self = Self(560);
    pub const OPTION_STRING_BASE: Self = Self(570);
    pub const QUITMENU_STRING_BASE: Self = Self(580);
    pub const STATUS_STRING_BASE: Self = Self(585);
    pub const FLAGSHIP_STRING_BASE: Self = Self(607);
    pub const ORBITSCAN_STRING_BASE: Self = Self(620);
    pub const MAINMENU_STRING_BASE: Self = Self(639);
    pub const NETMELEE_STRING_BASE: Self = Self(713);
    pub const BIOLOGICAL_STRING_BASE: Self = Self(747);
    pub const PLAYMENU_STRING_BASE: Self = Self(773);
    pub const LABEL_STRING_BASE: Self = Self(803);
    pub const GAMESTR_COUNT: Self = Self(829);
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
unsafe extern "C" fn buildColorRgba(
    mut r: BYTE,
    mut g: BYTE,
    mut b: BYTE,
    mut a: BYTE,
) -> Color {
    let mut c: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    c.r = r;
    c.g = g;
    c.b = b;
    c.a = a;
    return c;
}
unsafe extern "C" fn DrawPCMenuFrame(mut r: *mut RECT) {
    if if resolutionFactor != 2 as ::core::ffi::c_uint {
        BOOLEAN::FALSE.0 as ::core::ffi::c_int
    } else {
        BOOLEAN::TRUE.0 as ::core::ffi::c_int
    } != 0 || optCustomBorder.0 != 0
    {
        uqm_lua_checked!(DrawRenderedBox(
            r,
            BOOLEAN::TRUE,
            colorFromRgb15(0 as BYTE, 0 as BYTE, 0x15 as BYTE),
            if optCustomBorder.0 != 0 {
                C2Rust_Unnamed_5::SPECIAL_BEVEL.0 as ::core::ffi::c_int
            } else {
                C2Rust_Unnamed_5::THIN_INNER_BEVEL.0 as ::core::ffi::c_int
            },
            optCustomBorder.0 as ::core::ffi::c_int,
        ), ());
    } else {
        uqm_lua_checked!(DrawStarConBox(
            r,
            ((1 as ::core::ffi::c_int) << resolutionFactor) as SIZE,
            colorFromRgb15(0xf as BYTE, 0xf as BYTE, 0xf as BYTE),
            colorFromRgb15(0x6 as BYTE, 0x6 as BYTE, 0x6 as BYTE),
            BOOLEAN::TRUE,
            colorFromRgb15(0 as BYTE, 0 as BYTE, 0x15 as BYTE),
            BOOLEAN::FALSE,
            buildColorRgba(0 as BYTE, 0 as BYTE, 0 as BYTE, 0 as BYTE),
        ), ());
    }
    (*r).corner.x = ((*r).corner.x as ::core::ffi::c_int
        + ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    (*r).corner.y = ((*r).corner.y as ::core::ffi::c_int
        + ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    (*r).extent.height = ((*r).extent.height as ::core::ffi::c_int
        - ((2 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    (*r).extent.width = ((*r).extent.width as ::core::ffi::c_int
        - ((2 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
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
    let mut j: COUNT = 0;
    static mut rd: COUNT = 0 as COUNT;
    let mut num_items: ::core::ffi::c_int = 0;
    let mut OldFont: FONT = ::core::ptr::null_mut::<FONT_DESC>();
    let mut t: TEXT = TEXT {
        baseline: POINT { x: 0, y: 0 },
        pStr: ::core::ptr::null::<UNICODE>(),
        align: TEXT_ALIGN::ALIGN_LEFT,
        CharCount: 0,
    };
    let mut buf: [UNICODE; 256] = [0; 256];
    let mut rt: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    pos = (beg_index as ::core::ffi::c_int + NewState as ::core::ffi::c_int) as BYTE;
    num_items = 1 as ::core::ffi::c_int + end_index as ::core::ffi::c_int
        - beg_index as ::core::ffi::c_int;
    (*r).extent.width = ((*r).extent.width as ::core::ffi::c_int
        - ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    rt = *r;
    rt.corner.x = (rt.corner.x as ::core::ffi::c_int
        - ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    rt.corner.y = (rt.corner.y as ::core::ffi::c_int
        + (((8 as ::core::ffi::c_int) << resolutionFactor)
            - (if if optWindowType == 0 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                3 as ::core::ffi::c_int
            } else {
                12 as ::core::ffi::c_int
            } << resolutionFactor))) as COORD;
    rt.extent.width = (rt.extent.width as ::core::ffi::c_int
        + ((2 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    rt.extent.height = (rt.extent.height as ::core::ffi::c_int
        + (if if optWindowType == 0 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            -6 as ::core::ffi::c_int
        } else {
            2 as ::core::ffi::c_int
        } << resolutionFactor)) as COORD;
    if optCustomBorder.0 == 0 {
        uqm_lua_checked!(DrawFilledRectangle(&raw mut rt), ());
    }
    uqm_lua_checked!(DrawBorder(C2Rust_Unnamed_4::SIS_RADAR_FRAME.0 as ::core::ffi::c_int as BYTE), ());
    if num_items * ((8 as ::core::ffi::c_int) << resolutionFactor)
        > (*r).extent.height as ::core::ffi::c_int
    {
        log_add(
            log_Level::log_Error,
            b"Warning, no room for all menu items!\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    } else {
        (*r).corner.y = ((*r).corner.y as ::core::ffi::c_int
            + ((*r).extent.height as ::core::ffi::c_int
                - num_items * ((8 as ::core::ffi::c_int) << resolutionFactor))
                / 2 as ::core::ffi::c_int) as COORD;
    }
    (*r).extent.height = (num_items * ((8 as ::core::ffi::c_int) << resolutionFactor)
        + ((3 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    (*r).corner.y = ((*r).corner.y as ::core::ffi::c_int
        + (if if optWindowType == 0 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        } << resolutionFactor)) as COORD;
    (*r).corner.y = ((*r).corner.y as ::core::ffi::c_int
        - (if if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            4 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        } << resolutionFactor)) as COORD;
    uqm_lua_checked!(DrawPCMenuFrame(r), ());
    OldFont = SetContextFont(StarConFont);
    t.align = TEXT_ALIGN::ALIGN_LEFT;
    t.baseline.x = ((*r).corner.x as ::core::ffi::c_int
        + ((2 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    t.baseline.y = ((*r).corner.y as ::core::ffi::c_int
        + ((8 as ::core::ffi::c_int) << resolutionFactor)
        - ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    t.pStr = &raw mut buf as *mut UNICODE;
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    (*r).corner.x = ((*r).corner.x as ::core::ffi::c_int
        + ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    (*r).extent.width = ((*r).extent.width as ::core::ffi::c_int
        - ((2 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    i = beg_index as COUNT;
    while i as ::core::ffi::c_int <= end_index as ::core::ffi::c_int {
        j = (if i as ::core::ffi::c_int
            > C2Rust_Unnamed_0::PM_EXIT_SETTINGS.0 as ::core::ffi::c_int
        {
            i as ::core::ffi::c_int
                - (C2Rust_Unnamed_0::PM_CHANGE_CAPTAIN.0 as ::core::ffi::c_int
                    - C2Rust_Unnamed_0::PM_READ_VERY_SLOW.0 as ::core::ffi::c_int)
        } else {
            i as ::core::ffi::c_int
        }) as COUNT;
        utf8StringCopy(
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[UNICODE; 256]>(),
            if i as ::core::ffi::c_int
                == C2Rust_Unnamed_0::PM_FUEL.0 as ::core::ffi::c_int
            {
                &raw mut pm_fuel_str as *mut UNICODE
            } else if i as ::core::ffi::c_int
                == C2Rust_Unnamed_0::PM_CREW.0 as ::core::ffi::c_int
            {
                &raw mut pm_crew_str as *mut UNICODE
            } else {
                GetStringAddress(
                    SetAbsStringTableIndex(
                        GameStrings,
                        (C2Rust_Unnamed_7::MAINMENU_STRING_BASE.0 as ::core::ffi::c_int
                            + j as ::core::ffi::c_int) as COUNT,
                    ),
                )
            },
        );
        if hilite as ::core::ffi::c_int != 0
            && pos as ::core::ffi::c_int == i as ::core::ffi::c_int
        {
            if pos as ::core::ffi::c_int != rd as ::core::ffi::c_int {
                SetContextForeGroundColor(
                    colorFromRgb15(0xa as BYTE, 0xa as BYTE, 0x1f as BYTE),
                );
                (*r).corner.y = (t.baseline.y as ::core::ffi::c_int
                    - ((8 as ::core::ffi::c_int) << resolutionFactor)
                    + ((2 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
                (*r).extent.height = (((8 as ::core::ffi::c_int) << resolutionFactor)
                    - ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
                if optCustomBorder.0 != 0 {
                    let mut s: STAMP = STAMP {
                        origin: POINT { x: 0, y: 0 },
                        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
                    };
                    s.origin = (*r).corner;
                    s.frame = SetAbsFrameIndex(
                        CustBevelFrame,
                        C2Rust_Unnamed_5::DOS_MENU_HILITE.0 as ::core::ffi::c_int
                            as COUNT,
                    );
                    uqm_lua_checked!(DrawStamp(&raw mut s), ());
                } else {
                    uqm_lua_checked!(DrawFilledRectangle(r), ());
                }
                rd = pos as COUNT;
            } else {
                rd = 0 as COUNT;
            }
            SetContextForeGroundColor(
                colorFromRgb15(0xa as BYTE, 0x1f as BYTE, 0x1f as BYTE),
            );
            uqm_lua_checked!(font_DrawText(&raw mut t), ());
        } else {
            SetContextForeGroundColor(
                colorFromRgb15(0 as BYTE, 0x15 as BYTE, 0x15 as BYTE),
            );
            uqm_lua_checked!(font_DrawText(&raw mut t), ());
        }
        t.baseline.y = (t.baseline.y as ::core::ffi::c_int
            + ((8 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        i = i.wrapping_add(1);
    }
    SetContextFont(OldFont);
}
unsafe extern "C" fn GetEndMenuState(mut BaseState: BYTE) -> BYTE {
    match BaseState as ::core::ffi::c_int {
        0 | 1 => return C2Rust_Unnamed_0::PM_NAVIGATE.0 as ::core::ffi::c_int as BYTE,
        7 => return C2Rust_Unnamed_0::PM_LAUNCH_LANDER.0 as ::core::ffi::c_int as BYTE,
        13 => return C2Rust_Unnamed_0::PM_EXIT_GAME_MENU.0 as ::core::ffi::c_int as BYTE,
        18 => {
            return C2Rust_Unnamed_0::PM_ENCOUNTER_GAME_MENU.0 as ::core::ffi::c_int
                as BYTE;
        }
        21 => return C2Rust_Unnamed_0::PM_EXIT_OUTFIT.0 as ::core::ffi::c_int as BYTE,
        25 => return C2Rust_Unnamed_0::PM_EXIT_SHIPYARD.0 as ::core::ffi::c_int as BYTE,
        28 => return C2Rust_Unnamed_0::PM_EXIT_SETTINGS.0 as ::core::ffi::c_int as BYTE,
        44 | 45 => {
            return C2Rust_Unnamed_0::PM_ALT_NAVIGATE.0 as ::core::ffi::c_int as BYTE;
        }
        49 => {
            return C2Rust_Unnamed_0::PM_ALT_EXIT_MANIFEST.0 as ::core::ffi::c_int as BYTE;
        }
        53 => return C2Rust_Unnamed_0::PM_ALT_EXIT_SCAN.0 as ::core::ffi::c_int as BYTE,
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
                return C2Rust_Unnamed_0::PM_SOUND_OFF.0 as ::core::ffi::c_int as BYTE
            } else {
                return C2Rust_Unnamed_0::PM_SOUND_ON.0 as ::core::ffi::c_int as BYTE
            }
        }
        30 => {
            if GlobData.Game_state.glob_flags as ::core::ffi::c_int
                & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int != 0
            {
                return C2Rust_Unnamed_0::PM_MUSIC_OFF.0 as ::core::ffi::c_int as BYTE
            } else {
                return C2Rust_Unnamed_0::PM_MUSIC_ON.0 as ::core::ffi::c_int as BYTE
            }
        }
        32 => {
            return (C2Rust_Unnamed_0::PM_CYBORG_OFF.0 as ::core::ffi::c_int
                + ((GlobData.Game_state.glob_flags as ::core::ffi::c_int
                    & (((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as BYTE
                    as ::core::ffi::c_int >> 6 as ::core::ffi::c_int)) as BYTE;
        }
        36 => {
            return (C2Rust_Unnamed_0::PM_READ_VERY_SLOW.0 as ::core::ffi::c_int
                + (GlobData.Game_state.glob_flags as ::core::ffi::c_int
                    & ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int) as BYTE as ::core::ffi::c_int) as BYTE;
        }
        _ => {}
    }
    return BadState;
}
unsafe extern "C" fn NextMenuState(mut BaseState: BYTE, mut CurState: BYTE) -> BYTE {
    let mut NextState: BYTE = 0;
    let mut AdjBase: BYTE = BaseState;
    if BaseState as ::core::ffi::c_int
        == C2Rust_Unnamed_0::PM_STARMAP.0 as ::core::ffi::c_int
    {
        AdjBase = AdjBase.wrapping_sub(1);
    }
    match AdjBase as ::core::ffi::c_int + CurState as ::core::ffi::c_int {
        28 | 29 => {
            NextState = C2Rust_Unnamed_0::PM_MUSIC_ON.0 as ::core::ffi::c_int as BYTE;
        }
        30 | 31 => {
            NextState = C2Rust_Unnamed_0::PM_CYBORG_OFF.0 as ::core::ffi::c_int as BYTE;
        }
        32 | 33 | 34 | 35 => {
            if isPC(optSmoothScroll).0 != 0 && usingSpeech.0 == 0
                || isPC(optWhichMenu).0 != 0
            {
                NextState = C2Rust_Unnamed_0::PM_READ_VERY_SLOW.0 as ::core::ffi::c_int
                    as BYTE;
            } else {
                NextState = C2Rust_Unnamed_0::PM_CHANGE_CAPTAIN.0 as ::core::ffi::c_int
                    as BYTE;
            }
        }
        36 | 37 | 38 | 39 | 40 => {
            NextState = C2Rust_Unnamed_0::PM_CHANGE_CAPTAIN.0 as ::core::ffi::c_int
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
        == C2Rust_Unnamed_0::PM_STARMAP.0 as ::core::ffi::c_int
    {
        AdjBase = AdjBase.wrapping_sub(1);
    }
    match AdjBase as ::core::ffi::c_int + CurState as ::core::ffi::c_int {
        29 => {
            NextState = C2Rust_Unnamed_0::PM_EXIT_SETTINGS.0 as ::core::ffi::c_int
                as SWORD;
        }
        30 | 31 => {
            NextState = C2Rust_Unnamed_0::PM_SOUND_ON.0 as ::core::ffi::c_int as SWORD;
        }
        32 | 33 | 34 | 35 => {
            NextState = C2Rust_Unnamed_0::PM_MUSIC_ON.0 as ::core::ffi::c_int as SWORD;
        }
        36 | 37 | 38 | 39 | 40 => {
            NextState = C2Rust_Unnamed_0::PM_CYBORG_OFF.0 as ::core::ffi::c_int as SWORD;
        }
        41 => {
            if isPC(optSmoothScroll).0 != 0 && usingSpeech.0 == 0
                || isPC(optWhichMenu).0 != 0
            {
                NextState = C2Rust_Unnamed_0::PM_READ_VERY_SLOW.0 as ::core::ffi::c_int
                    as SWORD;
            } else {
                NextState = C2Rust_Unnamed_0::PM_CYBORG_OFF.0 as ::core::ffi::c_int
                    as SWORD;
            }
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
        == C2Rust_Unnamed_0::PM_STARMAP.0 as ::core::ffi::c_int
    {
        AdjBase = AdjBase.wrapping_sub(1);
        adj = 1 as BYTE;
    }
    if *CurState as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int != 0 {
        match *CurState as ::core::ffi::c_int {
            0x80 => {
                *BaseState = (C2Rust_Unnamed_0::PM_ALT_SCAN.0 as ::core::ffi::c_int
                    + adj as ::core::ffi::c_int) as BYTE;
                *CurState = (C2Rust_Unnamed_0::PM_ALT_MANIFEST.0 as ::core::ffi::c_int
                    - C2Rust_Unnamed_0::PM_ALT_SCAN.0 as ::core::ffi::c_int
                    - adj as ::core::ffi::c_int) as BYTE;
                return BOOLEAN::TRUE;
            }
            0x81 => {
                *BaseState = C2Rust_Unnamed_0::PM_ALT_CARGO.0 as ::core::ffi::c_int
                    as BYTE;
                *CurState = (C2Rust_Unnamed_0::PM_ALT_EXIT_MANIFEST.0
                    as ::core::ffi::c_int
                    - C2Rust_Unnamed_0::PM_ALT_CARGO.0 as ::core::ffi::c_int) as BYTE;
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
                *BaseState = C2Rust_Unnamed_0::PM_ALT_SCAN.0 as ::core::ffi::c_int
                    as BYTE;
                *CurState = (C2Rust_Unnamed_0::PM_ALT_SCAN.0 as ::core::ffi::c_int
                    - C2Rust_Unnamed_0::PM_ALT_SCAN.0 as ::core::ffi::c_int) as BYTE;
                return BOOLEAN::TRUE;
            }
            1 => {
                *BaseState = (C2Rust_Unnamed_0::PM_ALT_SCAN.0 as ::core::ffi::c_int
                    + adj as ::core::ffi::c_int) as BYTE;
                *CurState = (C2Rust_Unnamed_0::PM_ALT_STARMAP.0 as ::core::ffi::c_int
                    - C2Rust_Unnamed_0::PM_ALT_SCAN.0 as ::core::ffi::c_int
                    - adj as ::core::ffi::c_int) as BYTE;
                return BOOLEAN::TRUE;
            }
            2 => {
                *BaseState = C2Rust_Unnamed_0::PM_ALT_CARGO.0 as ::core::ffi::c_int
                    as BYTE;
                *CurState = (C2Rust_Unnamed_0::PM_ALT_DEVICES.0 as ::core::ffi::c_int
                    - C2Rust_Unnamed_0::PM_ALT_CARGO.0 as ::core::ffi::c_int) as BYTE;
                return BOOLEAN::TRUE;
            }
            3 => {
                *BaseState = C2Rust_Unnamed_0::PM_ALT_CARGO.0 as ::core::ffi::c_int
                    as BYTE;
                *CurState = (C2Rust_Unnamed_0::PM_ALT_CARGO.0 as ::core::ffi::c_int
                    - C2Rust_Unnamed_0::PM_ALT_CARGO.0 as ::core::ffi::c_int) as BYTE;
                return BOOLEAN::TRUE;
            }
            4 => {
                *BaseState = C2Rust_Unnamed_0::PM_ALT_CARGO.0 as ::core::ffi::c_int
                    as BYTE;
                *CurState = (C2Rust_Unnamed_0::PM_ALT_ROSTER.0 as ::core::ffi::c_int
                    - C2Rust_Unnamed_0::PM_ALT_CARGO.0 as ::core::ffi::c_int) as BYTE;
                return BOOLEAN::TRUE;
            }
            5 => {
                *BaseState = (C2Rust_Unnamed_0::PM_ALT_SCAN.0 as ::core::ffi::c_int
                    + adj as ::core::ffi::c_int) as BYTE;
                *CurState = (C2Rust_Unnamed_0::PM_ALT_GAME_MENU.0 as ::core::ffi::c_int
                    - C2Rust_Unnamed_0::PM_ALT_SCAN.0 as ::core::ffi::c_int
                    - adj as ::core::ffi::c_int) as BYTE;
                return BOOLEAN::TRUE;
            }
            6 => {
                *BaseState = (C2Rust_Unnamed_0::PM_ALT_SCAN.0 as ::core::ffi::c_int
                    + adj as ::core::ffi::c_int) as BYTE;
                *CurState = (C2Rust_Unnamed_0::PM_ALT_NAVIGATE.0 as ::core::ffi::c_int
                    - C2Rust_Unnamed_0::PM_ALT_SCAN.0 as ::core::ffi::c_int
                    - adj as ::core::ffi::c_int) as BYTE;
                return BOOLEAN::TRUE;
            }
            7 => {
                *BaseState = C2Rust_Unnamed_0::PM_ALT_MSCAN.0 as ::core::ffi::c_int
                    as BYTE;
                *CurState = (C2Rust_Unnamed_0::PM_ALT_MSCAN.0 as ::core::ffi::c_int
                    - C2Rust_Unnamed_0::PM_ALT_MSCAN.0 as ::core::ffi::c_int) as BYTE;
                return BOOLEAN::TRUE;
            }
            8 => {
                *BaseState = C2Rust_Unnamed_0::PM_ALT_MSCAN.0 as ::core::ffi::c_int
                    as BYTE;
                *CurState = (C2Rust_Unnamed_0::PM_ALT_ESCAN.0 as ::core::ffi::c_int
                    - C2Rust_Unnamed_0::PM_ALT_MSCAN.0 as ::core::ffi::c_int) as BYTE;
                return BOOLEAN::TRUE;
            }
            9 => {
                *BaseState = C2Rust_Unnamed_0::PM_ALT_MSCAN.0 as ::core::ffi::c_int
                    as BYTE;
                *CurState = (C2Rust_Unnamed_0::PM_ALT_BSCAN.0 as ::core::ffi::c_int
                    - C2Rust_Unnamed_0::PM_ALT_MSCAN.0 as ::core::ffi::c_int) as BYTE;
                return BOOLEAN::TRUE;
            }
            10 => {
                *BaseState = C2Rust_Unnamed_0::PM_ALT_MSCAN.0 as ::core::ffi::c_int
                    as BYTE;
                *CurState = (C2Rust_Unnamed_0::PM_ALT_EXIT_SCAN.0 as ::core::ffi::c_int
                    - C2Rust_Unnamed_0::PM_ALT_MSCAN.0 as ::core::ffi::c_int) as BYTE;
                return BOOLEAN::TRUE;
            }
            11 => {
                *BaseState = C2Rust_Unnamed_0::PM_ALT_MSCAN.0 as ::core::ffi::c_int
                    as BYTE;
                *CurState = (C2Rust_Unnamed_0::PM_ALT_ASCAN.0 as ::core::ffi::c_int
                    - C2Rust_Unnamed_0::PM_ALT_MSCAN.0 as ::core::ffi::c_int) as BYTE;
                return BOOLEAN::TRUE;
            }
            12 => {
                *BaseState = C2Rust_Unnamed_0::PM_ALT_MSCAN.0 as ::core::ffi::c_int
                    as BYTE;
                *CurState = (C2Rust_Unnamed_0::PM_ALT_DISPATCH.0 as ::core::ffi::c_int
                    - C2Rust_Unnamed_0::PM_ALT_MSCAN.0 as ::core::ffi::c_int) as BYTE;
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
        44 => {
            return (C2Rust_Unnamed_0::PM_SCAN.0 as ::core::ffi::c_int
                - C2Rust_Unnamed_0::PM_SCAN.0 as ::core::ffi::c_int) as BYTE;
        }
        45 => {
            return (C2Rust_Unnamed_0::PM_STARMAP.0 as ::core::ffi::c_int
                - C2Rust_Unnamed_0::PM_SCAN.0 as ::core::ffi::c_int) as BYTE;
        }
        46 => return 0x80 as BYTE,
        47 => {
            return (C2Rust_Unnamed_0::PM_GAME_MENU.0 as ::core::ffi::c_int
                - C2Rust_Unnamed_0::PM_SCAN.0 as ::core::ffi::c_int) as BYTE;
        }
        48 => {
            return (C2Rust_Unnamed_0::PM_NAVIGATE.0 as ::core::ffi::c_int
                - C2Rust_Unnamed_0::PM_SCAN.0 as ::core::ffi::c_int) as BYTE;
        }
        49 => {
            return (C2Rust_Unnamed_0::PM_CARGO.0 as ::core::ffi::c_int
                - C2Rust_Unnamed_0::PM_SCAN.0 as ::core::ffi::c_int) as BYTE;
        }
        50 => {
            return (C2Rust_Unnamed_0::PM_DEVICES.0 as ::core::ffi::c_int
                - C2Rust_Unnamed_0::PM_SCAN.0 as ::core::ffi::c_int) as BYTE;
        }
        51 => {
            return (C2Rust_Unnamed_0::PM_ROSTER.0 as ::core::ffi::c_int
                - C2Rust_Unnamed_0::PM_SCAN.0 as ::core::ffi::c_int) as BYTE;
        }
        52 => return 0x81 as BYTE,
        53 => {
            return (C2Rust_Unnamed_0::PM_MIN_SCAN.0 as ::core::ffi::c_int
                - C2Rust_Unnamed_0::PM_MIN_SCAN.0 as ::core::ffi::c_int) as BYTE;
        }
        54 => {
            return (C2Rust_Unnamed_0::PM_ENE_SCAN.0 as ::core::ffi::c_int
                - C2Rust_Unnamed_0::PM_MIN_SCAN.0 as ::core::ffi::c_int) as BYTE;
        }
        55 => {
            return (C2Rust_Unnamed_0::PM_BIO_SCAN.0 as ::core::ffi::c_int
                - C2Rust_Unnamed_0::PM_MIN_SCAN.0 as ::core::ffi::c_int) as BYTE;
        }
        56 => {
            return (C2Rust_Unnamed_0::PM_AUTO_SCAN.0 as ::core::ffi::c_int
                - C2Rust_Unnamed_0::PM_MIN_SCAN.0 as ::core::ffi::c_int) as BYTE;
        }
        57 => {
            return (C2Rust_Unnamed_0::PM_LAUNCH_LANDER.0 as ::core::ffi::c_int
                - C2Rust_Unnamed_0::PM_MIN_SCAN.0 as ::core::ffi::c_int) as BYTE;
        }
        58 => {
            return (C2Rust_Unnamed_0::PM_EXIT_SCAN.0 as ::core::ffi::c_int
                - C2Rust_Unnamed_0::PM_MIN_SCAN.0 as ::core::ffi::c_int) as BYTE;
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
        .menu[C2Rust_Unnamed_2::KEY_MENU_LEFT.0 as ::core::ffi::c_int as usize] != 0
        || PulsedInputState
            .menu[C2Rust_Unnamed_2::KEY_MENU_UP.0 as ::core::ffi::c_int as usize] != 0
    {
        NewState = PreviousMenuState(BaseState, NewState);
    } else if PulsedInputState
        .menu[C2Rust_Unnamed_2::KEY_MENU_RIGHT.0 as ::core::ffi::c_int as usize] != 0
        || PulsedInputState
            .menu[C2Rust_Unnamed_2::KEY_MENU_DOWN.0 as ::core::ffi::c_int as usize] != 0
    {
        NewState = NextMenuState(BaseState, NewState);
    } else if useAltMenu.0 != 0
        && PulsedInputState
            .menu[C2Rust_Unnamed_2::KEY_MENU_SELECT.0 as ::core::ffi::c_int as usize]
            != 0
    {
        NewState = ConvertAlternateMenu(BaseState, NewState);
        if NewState as ::core::ffi::c_int == 0x80 as ::core::ffi::c_int {
            uqm_lua_checked!(DrawMenuStateStrings(
                C2Rust_Unnamed_0::PM_ALT_CARGO.0 as ::core::ffi::c_int as BYTE,
                0 as SWORD,
            ), BOOLEAN(0));
            (*pMS).CurState = (C2Rust_Unnamed_0::PM_CARGO.0 as ::core::ffi::c_int
                - C2Rust_Unnamed_0::PM_SCAN.0 as ::core::ffi::c_int) as BYTE;
            return BOOLEAN::TRUE;
        }
        if NewState as ::core::ffi::c_int == 0x81 as ::core::ffi::c_int {
            if OrigBase as ::core::ffi::c_int
                == C2Rust_Unnamed_0::PM_SCAN.0 as ::core::ffi::c_int
            {
                uqm_lua_checked!(DrawMenuStateStrings(
                    C2Rust_Unnamed_0::PM_ALT_SCAN.0 as ::core::ffi::c_int as BYTE,
                    (C2Rust_Unnamed_0::PM_ALT_MANIFEST.0 as ::core::ffi::c_int
                        - C2Rust_Unnamed_0::PM_ALT_SCAN.0 as ::core::ffi::c_int) as SWORD,
                ), BOOLEAN(0));
            } else {
                uqm_lua_checked!(DrawMenuStateStrings(
                    C2Rust_Unnamed_0::PM_ALT_STARMAP.0 as ::core::ffi::c_int as BYTE,
                    (C2Rust_Unnamed_0::PM_ALT_MANIFEST.0 as ::core::ffi::c_int
                        - C2Rust_Unnamed_0::PM_ALT_STARMAP.0 as ::core::ffi::c_int)
                        as SWORD,
                ), BOOLEAN(0));
            }
            (*pMS).CurState = 0x80 as BYTE;
            return BOOLEAN::TRUE;
        }
        return BOOLEAN::FALSE;
    } else if optWhichMenu == 0x2 as ::core::ffi::c_int
        && PulsedInputState
            .menu[C2Rust_Unnamed_2::KEY_MENU_CANCEL.0 as ::core::ffi::c_int as usize]
            != 0
        && BaseState as ::core::ffi::c_int
            == C2Rust_Unnamed_0::PM_ALT_CARGO.0 as ::core::ffi::c_int
    {
        if OrigBase as ::core::ffi::c_int
            == C2Rust_Unnamed_0::PM_SCAN.0 as ::core::ffi::c_int
        {
            uqm_lua_checked!(DrawMenuStateStrings(
                C2Rust_Unnamed_0::PM_ALT_SCAN.0 as ::core::ffi::c_int as BYTE,
                (C2Rust_Unnamed_0::PM_ALT_MANIFEST.0 as ::core::ffi::c_int
                    - C2Rust_Unnamed_0::PM_ALT_SCAN.0 as ::core::ffi::c_int) as SWORD,
            ), BOOLEAN(0));
        } else {
            uqm_lua_checked!(DrawMenuStateStrings(
                C2Rust_Unnamed_0::PM_ALT_STARMAP.0 as ::core::ffi::c_int as BYTE,
                (C2Rust_Unnamed_0::PM_ALT_MANIFEST.0 as ::core::ffi::c_int
                    - C2Rust_Unnamed_0::PM_ALT_STARMAP.0 as ::core::ffi::c_int) as SWORD,
            ), BOOLEAN(0));
        }
        (*pMS).CurState = 0x80 as BYTE;
        return BOOLEAN::TRUE;
    } else {
        return BOOLEAN::FALSE
    }
    uqm_lua_checked!(DrawMenuStateStrings(BaseState, NewState as SWORD), BOOLEAN(0));
    if useAltMenu.0 != 0 {
        NewState = ConvertAlternateMenu(BaseState, NewState);
    }
    (*pMS).CurState = NewState;
    return BOOLEAN::TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn IndexToText(mut Index: ::core::ffi::c_int) -> *const UNICODE {
    let mut i: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    if Index < C2Rust_Unnamed_0::PM_EXIT_GAME_MENU.0 as ::core::ffi::c_int {
        i = Index;
    }
    match Index {
        20 | 23 | 26 => {
            i = C2Rust_Unnamed_1::GAMESTR_GAME_MENU.0 as ::core::ffi::c_int;
        }
        17 | 24 | 27 | 43 => {
            i = C2Rust_Unnamed_1::GAMESTR_EXIT_MENU.0 as ::core::ffi::c_int;
        }
        18 => {
            i = C2Rust_Unnamed_1::GAMESTR_CONVERSE.0 as ::core::ffi::c_int;
        }
        19 => {
            i = C2Rust_Unnamed_1::GAMESTR_ATTACK.0 as ::core::ffi::c_int;
        }
        21 => {
            i = C2Rust_Unnamed_1::GAMESTR_FUEL.0 as ::core::ffi::c_int;
        }
        22 => {
            i = C2Rust_Unnamed_1::GAMESTR_MODULE.0 as ::core::ffi::c_int;
        }
        25 => {
            i = C2Rust_Unnamed_1::GAMESTR_CREW.0 as ::core::ffi::c_int;
        }
        28 => {
            i = C2Rust_Unnamed_1::GAMESTR_SND_ON.0 as ::core::ffi::c_int;
        }
        29 => {
            i = C2Rust_Unnamed_1::GAMESTR_SND_OFF.0 as ::core::ffi::c_int;
        }
        30 => {
            i = C2Rust_Unnamed_1::GAMESTR_MUS_ON.0 as ::core::ffi::c_int;
        }
        31 => {
            i = C2Rust_Unnamed_1::GAMESTR_MUS_OFF.0 as ::core::ffi::c_int;
        }
        32 | 33 | 34 | 35 => {
            i = C2Rust_Unnamed_1::GAMESTR_COMBAT.0 as ::core::ffi::c_int;
        }
        36 | 37 | 38 | 39 | 40 => {
            i = C2Rust_Unnamed_1::GAMESTR_READING.0 as ::core::ffi::c_int;
        }
        41 => {
            i = C2Rust_Unnamed_1::GAMESTR_CHANGE_CAP.0 as ::core::ffi::c_int;
        }
        42 => {
            i = C2Rust_Unnamed_1::GAMESTR_CHANGE_SIS.0 as ::core::ffi::c_int;
        }
        _ => {}
    }
    if i == -1 as ::core::ffi::c_int
        || strlen(
            GetStringAddress(
                SetAbsStringTableIndex(
                    GameStrings,
                    (C2Rust_Unnamed_7::PLAYMENU_STRING_BASE.0 as ::core::ffi::c_int + i)
                        as COUNT,
                ),
            ),
        ) == 0
    {
        return ::core::ptr::null::<UNICODE>();
    }
    return GetStringAddress(
        SetAbsStringTableIndex(
            GameStrings,
            (C2Rust_Unnamed_7::PLAYMENU_STRING_BASE.0 as ::core::ffi::c_int + i) as COUNT,
        ),
    );
}
unsafe extern "C" fn Draw3DOMenuText(mut r: *mut RECT, mut Index: ::core::ffi::c_int) {
    let mut text: TEXT = TEXT {
        baseline: POINT { x: 0, y: 0 },
        pStr: ::core::ptr::null::<UNICODE>(),
        align: TEXT_ALIGN::ALIGN_LEFT,
        CharCount: 0,
    };
    let mut leading: SIZE = 0;
    let mut block: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut OldFont: FONT = ::core::ptr::null_mut::<FONT_DESC>();
    let mut OldColor: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    if IndexToText(Index).is_null() {
        return;
    }
    OldFont = SetContextFont(PlayMenuFont);
    OldColor = SetContextForeGroundColor(
        colorFromRgb15(0xa as BYTE, 0xa as BYTE, 0xa as BYTE),
    );
    GetContextFontLeading(&raw mut leading);
    if optCustomBorder.0 == 0 {
        block = *r;
        block.corner.y = (block.corner.y as ::core::ffi::c_int
            + (if if optWindowType == 0 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                2 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } << resolutionFactor)) as COORD;
        block.extent.height = leading as COORD;
        uqm_lua_checked!(DrawFilledRectangle(&raw mut block), ());
    } else {
        uqm_lua_checked!(DrawBorder(C2Rust_Unnamed_4::TEXT_LABEL_FRAME.0 as ::core::ffi::c_int as BYTE), ());
    }
    text.align = TEXT_ALIGN::ALIGN_CENTER;
    text.baseline.x = ((*r).corner.x as ::core::ffi::c_int
        + ((*r).extent.width as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)) as COORD;
    text.baseline.y = ((*r).corner.y as ::core::ffi::c_int
        + leading as ::core::ffi::c_int
        - (if if optWindowType == 0 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            0 as ::core::ffi::c_int
        } else {
            2 as ::core::ffi::c_int
        } << resolutionFactor)) as COORD;
    text.pStr = AlignText(IndexToText(Index), &raw mut text.baseline.x);
    text.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    uqm_lua_checked!(font_DrawShadowedText(
        &raw mut text,
        C2Rust_Unnamed::NORTH_WEST_SHADOW.0 as ::core::ffi::c_int as BYTE,
        buildColorRgba(0x74 as BYTE, 0x74 as BYTE, 0x74 as BYTE, 0xff as BYTE),
        buildColorRgba(0x31 as BYTE, 0x31 as BYTE, 0x31 as BYTE, 0xff as BYTE),
    ), ());
    SetContextFont(OldFont);
    SetContextForeGroundColor(OldColor);
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
    if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
        & (C2Rust_Unnamed_6::CHECK_ABORT.0 as ::core::ffi::c_int
            | C2Rust_Unnamed_6::CHECK_LOAD.0 as ::core::ffi::c_int) != 0
    {
        return;
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
        == C2Rust_Unnamed_0::PM_STARMAP.0 as ::core::ffi::c_int
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
    uqm_lua_checked!(PreUpdateFlashRect(), ());
    OldContext = SetContext(StatusContext);
    GetContextClipRect(&raw mut r);
    s.origin.x = (((4 as ::core::ffi::c_int) << resolutionFactor)
        + (CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
            - (if if optWindowType == 1 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                16 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } << resolutionFactor) * 2 as ::core::ffi::c_int
            + (if if optWindowType == 1 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                16 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } << resolutionFactor)) - r.corner.x as ::core::ffi::c_int) as COORD;
    s.origin.y = (((9 as ::core::ffi::c_int) << resolutionFactor)
        + if if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            (if if optWindowType == 1 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                16 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } << resolutionFactor) + ((1 as ::core::ffi::c_int) << resolutionFactor)
        } else {
            0 as ::core::ffi::c_int
        }
        + (CanvasHeight
            - (if if optWindowType == 1 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                16 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } << resolutionFactor) * 2 as ::core::ffi::c_int
            - ((13 as ::core::ffi::c_int) << resolutionFactor))
        - ((53 as ::core::ffi::c_int) << resolutionFactor)
        - r.corner.y as ::core::ffi::c_int) as COORD;
    r.corner.x = (s.origin.x as ::core::ffi::c_int
        - ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    if optWhichMenu == 0x2 as ::core::ffi::c_int {
        r.corner.y = (s.origin.y as ::core::ffi::c_int
            - ((8 as ::core::ffi::c_int) << resolutionFactor)
            - if if resolutionFactor != 2 as ::core::ffi::c_uint {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } == 0
            {
                0 as ::core::ffi::c_int
            } else {
                2 as ::core::ffi::c_int
            }) as COORD;
    } else {
        r.corner.y = (s.origin.y as ::core::ffi::c_int
            - ((11 as ::core::ffi::c_int) << resolutionFactor)
            + (if if optWindowType == 0 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                3 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } << resolutionFactor)) as COORD;
    }
    r.extent.width = (((64 as ::core::ffi::c_int) << resolutionFactor)
        - ((8 as ::core::ffi::c_int) << resolutionFactor)
        + ((3 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    BatchGraphics();
    SetContextForeGroundColor(colorFromRgb15(0xa as BYTE, 0xa as BYTE, 0xa as BYTE));
    if !s.frame.is_null() && optWhichMenu == 0x2 as ::core::ffi::c_int {
        if beg_index as ::core::ffi::c_int
            == C2Rust_Unnamed_0::PM_CREW.0 as ::core::ffi::c_int
        {
            snprintf(
                &raw mut pm_crew_str as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[UNICODE; 128]>(),
                b"%s(%d)\0".as_ptr() as *const ::core::ffi::c_char,
                GetStringAddress(
                    SetAbsStringTableIndex(
                        GameStrings,
                        (C2Rust_Unnamed_7::MAINMENU_STRING_BASE.0 as ::core::ffi::c_int
                            + C2Rust_Unnamed_0::PM_CREW.0 as ::core::ffi::c_int) as COUNT,
                    ),
                ),
                GlobData.Game_state.CrewCost as ::core::ffi::c_int,
            );
        }
        if beg_index as ::core::ffi::c_int
            == C2Rust_Unnamed_0::PM_FUEL.0 as ::core::ffi::c_int
        {
            snprintf(
                &raw mut pm_fuel_str as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[UNICODE; 128]>(),
                b"%s(%d)\0".as_ptr() as *const ::core::ffi::c_char,
                GetStringAddress(
                    SetAbsStringTableIndex(
                        GameStrings,
                        (C2Rust_Unnamed_7::MAINMENU_STRING_BASE.0 as ::core::ffi::c_int
                            + C2Rust_Unnamed_0::PM_FUEL.0 as ::core::ffi::c_int) as COUNT,
                    ),
                ),
                GlobData.Game_state.FuelCost as ::core::ffi::c_int,
            );
        }
        if beg_index as ::core::ffi::c_int
            == C2Rust_Unnamed_0::PM_SOUND_ON.0 as ::core::ffi::c_int
        {
            end_index = (beg_index as ::core::ffi::c_int + 6 as ::core::ffi::c_int)
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
                36 | 37 | 38 | 39 | 40 => {
                    NewState = 3 as SWORD;
                }
                41 => {
                    NewState = 4 as SWORD;
                }
                42 => {
                    NewState = 5 as SWORD;
                }
                43 => {
                    NewState = 6 as SWORD;
                }
                _ => {}
            }
        }
        r.extent.height = (((53 as ::core::ffi::c_int) << resolutionFactor)
            + ((12 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        uqm_lua_checked!(DrawPCMenu(beg_index, end_index, NewState as BYTE, hilite, &raw mut r), ());
        s.frame = ::core::ptr::null_mut::<FRAME_DESC>();
        if optSubmenu.0 != 0 {
            uqm_lua_checked!(FunkyMenu(
                (beg_index as ::core::ffi::c_int
                    + NewState as BYTE as ::core::ffi::c_int) as BYTE,
                s,
            ), ());
        }
    } else {
        r.corner.x = (r.corner.x as ::core::ffi::c_int
            - ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        r.corner.y = (r.corner.y as ::core::ffi::c_int
            + (if if optWindowType == 0 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                5 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } << resolutionFactor)) as COORD;
        r.extent.width = (r.extent.width as ::core::ffi::c_int
            + ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        r.extent.height = (((53 as ::core::ffi::c_int) << resolutionFactor)
            + (if isPC(optWhichMenu).0 != 0 {
                9 as ::core::ffi::c_int
            } else {
                12 as ::core::ffi::c_int
            } << resolutionFactor)) as COORD;
        r.extent.height = (r.extent.height as ::core::ffi::c_int
            - (if if optWindowType == 0 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                6 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } << resolutionFactor)) as COORD;
        if optCustomBorder.0 == 0 {
            uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
        } else {
            uqm_lua_checked!(DrawBorder(
                C2Rust_Unnamed_4::SIS_RADAR_FRAME.0 as ::core::ffi::c_int as BYTE,
            ), ());
        }
    }
    if !s.frame.is_null() {
        if optSubmenu.0 != 0 {
            uqm_lua_checked!(FunkyMenu(
                (beg_index as ::core::ffi::c_int
                    + NewState as BYTE as ::core::ffi::c_int) as BYTE,
                s,
            ), ());
        } else {
            uqm_lua_checked!(DrawStamp(&raw mut s), ());
        }
        uqm_lua_checked!(Draw3DOMenuText(
            &raw mut r,
            beg_index as ::core::ffi::c_int + NewState as ::core::ffi::c_int,
        ), ());
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
                    + (((64 as ::core::ffi::c_int) << resolutionFactor)
                        - ((8 as ::core::ffi::c_int) << resolutionFactor))
                    - ((2 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
                t.baseline.y = (s.origin.y as ::core::ffi::c_int
                    + ((53 as ::core::ffi::c_int) << resolutionFactor)
                    - ((2 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
                t.align = TEXT_ALIGN::ALIGN_RIGHT;
                t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
                t.pStr = &raw mut buf as *mut UNICODE;
                snprintf(
                    &raw mut buf as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[UNICODE; 20]>(),
                    b"%u\0".as_ptr() as *const ::core::ffi::c_char,
                    GlobData.Game_state.CrewCost as ::core::ffi::c_int,
                );
                if isPC(optWhichFonts).0 != 0 {
                    SetContextFont(TinyFont);
                } else {
                    SetContextFont(TinyFontBold);
                }
                SetContextForeGroundColor(
                    colorFromRgb15(0 as BYTE, 0x1f as BYTE, 0 as BYTE),
                );
                uqm_lua_checked!(font_DrawText(&raw mut t), ());
            }
            21 => {
                t.baseline.x = (s.origin.x as ::core::ffi::c_int
                    + (((64 as ::core::ffi::c_int) << resolutionFactor)
                        - ((8 as ::core::ffi::c_int) << resolutionFactor))
                    - ((2 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
                t.baseline.y = (s.origin.y as ::core::ffi::c_int
                    + ((53 as ::core::ffi::c_int) << resolutionFactor)
                    - ((2 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
                t.align = TEXT_ALIGN::ALIGN_RIGHT;
                t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
                t.pStr = &raw mut buf as *mut UNICODE;
                snprintf(
                    &raw mut buf as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[UNICODE; 20]>(),
                    b"%u\0".as_ptr() as *const ::core::ffi::c_char,
                    GlobData.Game_state.FuelCost as ::core::ffi::c_int,
                );
                if optWhichFonts == 0x1 as ::core::ffi::c_int && optWholeFuel.0 == 0 {
                    SetContextFont(TinyFontBold);
                } else {
                    SetContextFont(TinyFont);
                }
                SetContextForeGroundColor(
                    colorFromRgb15(0 as BYTE, 0x1f as BYTE, 0 as BYTE),
                );
                uqm_lua_checked!(font_DrawText(&raw mut t), ());
            }
            _ => {}
        }
    }
    UnbatchGraphics();
    SetContext(OldContext);
    uqm_lua_checked!(PostUpdateFlashRect(), ());
}
#[no_mangle]
pub unsafe extern "C" fn DrawMineralHelpers() {
    let mut OldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    let mut i: COUNT = 0;
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
    let mut buf: [UNICODE; 40] = [0; 40];
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut leading: SIZE = 0;
    if optSubmenu.0 == 0 {
        return;
    }
    OldContext = SetContext(StatusContext);
    BatchGraphics();
    uqm_lua_checked!(DrawFlagStatDisplay(b"\0".as_ptr() as *const ::core::ffi::c_char as *mut UNICODE), ());
    SetContextFont(TinyFont);
    GetContextFontLeading(&raw mut leading);
    leading = (leading as ::core::ffi::c_int
        - ((1 as ::core::ffi::c_int) << resolutionFactor)) as SIZE;
    s.frame = SetAbsFrameIndex(
        MiscDataFrame,
        (((4 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
            + 3 as ::core::ffi::c_int) as COUNT,
    );
    s.origin.x = ((10 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    s.origin.y = ((24 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    t.baseline.x = ((32 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    t.baseline.y = (((24 as ::core::ffi::c_int) << resolutionFactor)
        + ((3 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    t.align = TEXT_ALIGN::ALIGN_RIGHT;
    t.pStr = &raw mut buf as *mut UNICODE;
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    r.extent = (*s.frame).Bounds;
    r.corner.x = (((10 as ::core::ffi::c_int) << resolutionFactor)
        - ((3 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    i = 0 as COUNT;
    while (i as ::core::ffi::c_int)
        < C2Rust_Unnamed_3::NUM_ELEMENT_CATEGORIES.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(DrawStamp(&raw mut s), ());
        s.frame = SetRelFrameIndex(s.frame, 5 as SIZE);
        s.origin.y = (s.origin.y as ::core::ffi::c_int
            + ((9 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        SetContextForeGroundColor(
            buildColorRgba(0x55 as BYTE, 0x55 as BYTE, 0xff as BYTE, 0xff as BYTE),
        );
        snprintf(
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[UNICODE; 40]>(),
            b"%u\0".as_ptr() as *const ::core::ffi::c_char,
            GlobData.Game_state.ElementWorth[i as usize] as ::core::ffi::c_int,
        );
        uqm_lua_checked!(font_DrawText(&raw mut t), ());
        snprintf(
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[UNICODE; 40]>(),
            b"x\0".as_ptr() as *const ::core::ffi::c_char,
        );
        t.baseline.x = (t.baseline.x as ::core::ffi::c_int
            - ((11 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        uqm_lua_checked!(font_DrawText(&raw mut t), ());
        SetContextForeGroundColor(
            buildColorRgba(0 as BYTE, 0xaa as BYTE, 0xaa as BYTE, 0xff as BYTE),
        );
        t.baseline.x = ((58 as ::core::ffi::c_int) << resolutionFactor) as COORD;
        snprintf(
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[UNICODE; 40]>(),
            b"%u\0".as_ptr() as *const ::core::ffi::c_char,
            GlobData.SIS_state.ElementAmounts[i as usize] as ::core::ffi::c_int,
        );
        uqm_lua_checked!(font_DrawText(&raw mut t), ());
        t.baseline.x = ((32 as ::core::ffi::c_int) << resolutionFactor) as COORD;
        t.baseline.y = (t.baseline.y as ::core::ffi::c_int
            + ((9 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        i = i.wrapping_add(1);
    }
    SetContextForeGroundColor(colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE));
    r.corner.y = (s.origin.y as ::core::ffi::c_int
        - ((3 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
    s.frame = SetAbsFrameIndex(s.frame, 68 as COUNT);
    s.origin.x = (r.corner.x as ::core::ffi::c_int
        + ((r.extent.height as ::core::ffi::c_int >> resolutionFactor
            >> 1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    s.origin.y = (r.corner.y as ::core::ffi::c_int
        + ((r.extent.height as ::core::ffi::c_int >> resolutionFactor
            >> 1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    uqm_lua_checked!(DrawStamp(&raw mut s), ());
    SetContextForeGroundColor(
        buildColorRgba(0 as BYTE, 0xaa as BYTE, 0xaa as BYTE, 0xff as BYTE),
    );
    t.baseline.x = ((58 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    snprintf(
        &raw mut buf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[UNICODE; 40]>(),
        b"%u\0".as_ptr() as *const ::core::ffi::c_char,
        GlobData.SIS_state.TotalBioMass as ::core::ffi::c_int,
    );
    uqm_lua_checked!(font_DrawText(&raw mut t), ());
    t.baseline.y = (t.baseline.y as ::core::ffi::c_int
        + ((9 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    r.corner.x = ((4 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    r.corner.y = (t.baseline.y as ::core::ffi::c_int
        - ((7 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    r.extent.width = (((64 as ::core::ffi::c_int) << resolutionFactor)
        - ((5 as ::core::ffi::c_int) << resolutionFactor)
        - ((3 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    r.extent.height = ((1 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    SetContextForeGroundColor(colorFromRgb15(0xa as BYTE, 0xa as BYTE, 0x1f as BYTE));
    uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
    SetContextForeGroundColor(
        buildColorRgba(0 as BYTE, 0xaa as BYTE, 0xaa as BYTE, 0xff as BYTE),
    );
    t.align = TEXT_ALIGN::ALIGN_LEFT;
    t.baseline.x = (r.corner.x as ::core::ffi::c_int
        + ((2 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    t.baseline.y = (r.corner.y as ::core::ffi::c_int + leading as ::core::ffi::c_int
        + ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    t.pStr = GetStringAddress(
        SetAbsStringTableIndex(
            GameStrings,
            (C2Rust_Unnamed_7::STATUS_STRING_BASE.0 as ::core::ffi::c_int
                + 8 as ::core::ffi::c_int) as COUNT,
        ),
    );
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    uqm_lua_checked!(font_DrawText(&raw mut t), ());
    t.baseline.y = (t.baseline.y as ::core::ffi::c_int + leading as ::core::ffi::c_int)
        as COORD;
    t.pStr = GetStringAddress(
        SetAbsStringTableIndex(
            GameStrings,
            (C2Rust_Unnamed_7::STATUS_STRING_BASE.0 as ::core::ffi::c_int
                + 9 as ::core::ffi::c_int) as COUNT,
        ),
    );
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    uqm_lua_checked!(font_DrawText(&raw mut t), ());
    t.baseline.y = (t.baseline.y as ::core::ffi::c_int + leading as ::core::ffi::c_int)
        as COORD;
    t.pStr = GetStringAddress(
        SetAbsStringTableIndex(
            GameStrings,
            (C2Rust_Unnamed_7::STATUS_STRING_BASE.0 as ::core::ffi::c_int
                + 10 as ::core::ffi::c_int) as COUNT,
        ),
    );
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    uqm_lua_checked!(font_DrawText(&raw mut t), ());
    t.baseline.y = (t.baseline.y as ::core::ffi::c_int + leading as ::core::ffi::c_int)
        as COORD;
    t.pStr = GetStringAddress(
        SetAbsStringTableIndex(
            GameStrings,
            (C2Rust_Unnamed_7::STATUS_STRING_BASE.0 as ::core::ffi::c_int
                + 11 as ::core::ffi::c_int) as COUNT,
        ),
    );
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    uqm_lua_checked!(font_DrawText(&raw mut t), ());
    SetContextForeGroundColor(
        buildColorRgba(0x55 as BYTE, 0x55 as BYTE, 0xff as BYTE, 0xff as BYTE),
    );
    t.align = TEXT_ALIGN::ALIGN_RIGHT;
    t.baseline.x = (r.extent.width as ::core::ffi::c_int
        + ((2 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    t.baseline.y = (r.corner.y as ::core::ffi::c_int + leading as ::core::ffi::c_int
        + ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    t.pStr = WholeFuelValue();
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    uqm_lua_checked!(font_DrawText(&raw mut t), ());
    t.baseline.y = (t.baseline.y as ::core::ffi::c_int + leading as ::core::ffi::c_int)
        as COORD;
    snprintf(
        &raw mut buf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[UNICODE; 40]>(),
        b"%u\0".as_ptr() as *const ::core::ffi::c_char,
        GlobData.SIS_state.CrewEnlisted as ::core::ffi::c_int,
    );
    t.pStr = &raw mut buf as *mut UNICODE;
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    uqm_lua_checked!(font_DrawText(&raw mut t), ());
    t.baseline.y = (t.baseline.y as ::core::ffi::c_int + leading as ::core::ffi::c_int)
        as COORD;
    snprintf(
        &raw mut buf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[UNICODE; 40]>(),
        b"%u\0".as_ptr() as *const ::core::ffi::c_char,
        GlobData.SIS_state.NumLanders as ::core::ffi::c_int,
    );
    t.pStr = &raw mut buf as *mut UNICODE;
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    uqm_lua_checked!(font_DrawText(&raw mut t), ());
    t.baseline.y = (t.baseline.y as ::core::ffi::c_int + leading as ::core::ffi::c_int)
        as COORD;
    if GetStorageBayCapacity() != 0 {
        let mut totalCapacity: ::core::ffi::c_float = GlobData.SIS_state.TotalElementMass
            as ::core::ffi::c_float / GetStorageBayCapacity() as ::core::ffi::c_float
            * 100 as ::core::ffi::c_int as ::core::ffi::c_float;
        if totalCapacity == 100 as ::core::ffi::c_int as ::core::ffi::c_float {
            snprintf(
                &raw mut buf as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[UNICODE; 40]>(),
                b"%.0f%%\0".as_ptr() as *const ::core::ffi::c_char,
                totalCapacity as ::core::ffi::c_double,
            );
        } else if totalCapacity > 9 as ::core::ffi::c_int as ::core::ffi::c_float {
            snprintf(
                &raw mut buf as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[UNICODE; 40]>(),
                b"%.1f%%\0".as_ptr() as *const ::core::ffi::c_char,
                totalCapacity as ::core::ffi::c_double,
            );
        } else {
            snprintf(
                &raw mut buf as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[UNICODE; 40]>(),
                b"%.2f%%\0".as_ptr() as *const ::core::ffi::c_char,
                totalCapacity as ::core::ffi::c_double,
            );
        }
    } else {
        snprintf(
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[UNICODE; 40]>(),
            b"---\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    t.pStr = &raw mut buf as *mut UNICODE;
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    uqm_lua_checked!(font_DrawText(&raw mut t), ());
    UnbatchGraphics();
    SetContext(OldContext);
}
#[no_mangle]
pub unsafe extern "C" fn DrawBorder(mut Visible: BYTE) {
    let mut s: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    let mut OldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    OldContext = SetContext(ScreenContext);
    s.origin.x = 0 as COORD;
    s.origin.y = 0 as COORD;
    if optCustomBorder.0 != 0 {
        s.frame = SetAbsFrameIndex(BorderFrame, Visible as COUNT);
        uqm_lua_checked!(DrawStamp(&raw mut s), ());
    } else if if resolutionFactor != 2 as ::core::ffi::c_uint {
        BOOLEAN::FALSE.0 as ::core::ffi::c_int
    } else {
        BOOLEAN::TRUE.0 as ::core::ffi::c_int
    } != 0
    {
        s.frame = SetAbsFrameIndex(HDBorderFrame, Visible as COUNT);
        uqm_lua_checked!(DrawStamp(&raw mut s), ());
    }
    SetContext(OldContext);
}
#[no_mangle]
pub unsafe extern "C" fn FunkyMenu(mut m: BYTE, mut stmp: STAMP) {
    static mut subMenuFlag: BOOLEAN = BOOLEAN::FALSE;
    if stmp.frame.is_null()
        && m as ::core::ffi::c_int
            >= C2Rust_Unnamed_0::PM_ALT_MSCAN.0 as ::core::ffi::c_int
        && m as ::core::ffi::c_int
            <= C2Rust_Unnamed_0::PM_ALT_EXIT_SCAN.0 as ::core::ffi::c_int
        || m as ::core::ffi::c_int
            == C2Rust_Unnamed_0::PM_LAUNCH_LANDER.0 as ::core::ffi::c_int
    {
        if !stmp.frame.is_null() {
            uqm_lua_checked!(DrawStamp(&raw mut stmp), ());
        }
        uqm_lua_checked!(DrawMineralHelpers(), ());
        subMenuFlag = BOOLEAN::TRUE;
        if !stmp.frame.is_null() {
            return;
        }
    } else if subMenuFlag.0 == BOOLEAN::TRUE.0 {
        uqm_lua_checked!(DeltaSISGauges(0x7fff as SIZE, 0x7fff as SDWORD, 0x7fff as ::core::ffi::c_int), ());
        subMenuFlag = BOOLEAN::FALSE;
    }
    if !stmp.frame.is_null() {
        uqm_lua_checked!(DrawStamp(&raw mut stmp), ());
    }
}
