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

// Rust translation of src/uqm/util.c.
extern "C" {
    pub type string_table_entry;
    pub type context_desc;
    pub type frame_desc;
    pub type font_desc;
    pub type drawable_desc;
    fn snprintf(
        _: *mut ::core::ffi::c_char,
        _: size_t,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn SetAbsStringTableIndex(String: STRING, StringTableIndex: COUNT) -> STRING;
    fn GetStringAddress(String: STRING) -> STRINGPTR;
    fn AlignText(str: *const UNICODE, loc_x: *mut sint16) -> *mut UNICODE;
    fn SetContext(Context: CONTEXT) -> CONTEXT;
    fn SetContextForeGroundColor(Color: Color) -> Color;
    fn SetContextClipRect(pRect: *mut RECT) -> BOOLEAN;
    fn GetContextClipRect(pRect: *mut RECT) -> BOOLEAN;
    fn SetContextOrigin(orgOffset: POINT) -> POINT;
    fn SetContextDrawMode(_: DrawMode) -> DrawMode;
    fn CopyContextRect(area: *const RECT) -> DRAWABLE;
    fn DrawStamp(pStamp: *mut STAMP);
    fn DrawFilledRectangle(pRect: *mut RECT);
    fn font_DrawText(pText: *mut TEXT);
    fn BatchGraphics();
    fn UnbatchGraphics();
    fn FlushGraphics();
    fn DestroyDrawable(Drawable: DRAWABLE) -> BOOLEAN;
    fn GetFrameRect(Frame: FRAME, pRect: *mut RECT) -> BOOLEAN;
    fn SetContextFont(Font: FONT) -> FONT;
    fn SetAbsFrameIndex(Frame: FRAME, FrameIndex: COUNT) -> FRAME;
    fn CaptureDrawable(Drawable: DRAWABLE) -> FRAME;
    fn ReleaseDrawable(Frame: FRAME) -> DRAWABLE;
    fn ClearSystemRect();
    fn strlen(_: *const ::core::ffi::c_char) -> size_t;
    fn GetTimeCounter() -> TimeCount;
    static mut ImmediateInputState: CONTROLLER_INPUT_STATE;
    fn FlushInput();
    static mut GamePaused: BOOLEAN;
    fn SleepThread(timePeriod: TimePeriod);
    fn TaskSwitch();
    fn FadeMusic(end_vol: BYTE, TimeInterval: SIZE) -> DWORD;
    fn GetCurrMusicVol() -> BYTE;
    static mut CanvasWidth: ::core::ffi::c_int;
    static mut CanvasHeight: ::core::ffi::c_int;
    fn getGameStateUint(name: *const ::core::ffi::c_char) -> DWORD;
    static mut GlobData: GLOBDATA;
    static mut ActivityFrame: FRAME;
    static mut CustBevelFrame: FRAME;
    static mut DefBevelFrame: FRAME;
    static mut ScreenContext: CONTEXT;
    static mut StatusContext: CONTEXT;
    static mut StarConFont: FONT;
    static mut LabelFont: FONT;
    static mut LastActivity: ACTIVITY;
    fn ResumeMusic();
    fn PauseMusic();
    fn AnyButtonPress(DetectSpecial: BOOLEAN) -> BOOLEAN;
    fn ActKeysPress() -> BOOLEAN;
    static mut QuitPosted: ::core::ffi::c_int;
    static mut GameActive: ::core::ffi::c_int;
    fn BeginInputFrame();
    fn PauseTrack();
    fn ResumeTrack();
    fn PlayingTrack() -> COUNT;
    fn TFB_SeedRandom(seed: DWORD) -> DWORD;
    fn log_add(_: log_Level, fmt: *const ::core::ffi::c_char, ...);
    fn square_root(value: DWORD) -> COUNT;
    fn DrawBorder(Visible: BYTE);
    static mut GameStrings: STRING;
    fn DeltaLastTime(diff: TimeCount);
    static mut plot_map: [PLOT_LOCATION; 70];
    static mut resolutionFactor: ::core::ffi::c_uint;
    static mut optInfiniteFuel: OPT_ENABLABLE;
    static mut optCustomBorder: OPT_ENABLABLE;
    static mut optWholeFuel: OPT_ENABLABLE;
    static mut optWindowType: DWORD;
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
    pub flags: BYTE,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct DrawKind(pub ::core::ffi::c_uint);
impl DrawKind {
    pub const DRAW_REPLACE: Self = Self(0);
    pub const DRAW_ADDITIVE: Self = Self(1);
    pub const DRAW_ALPHA: Self = Self(2);
    pub const DRAW_MULTIPLY: Self = Self(3);
    pub const DRAW_OVERLAY: Self = Self(4);
    pub const DRAW_SCREEN: Self = Self(5);
    pub const DRAW_GRAYSCALE: Self = Self(6);
    pub const DRAW_LINEARBURN: Self = Self(7);
    pub const DRAW_HYPTOQUAS: Self = Self(8);
    pub const DRAW_DESATURATE: Self = Self(9);
    pub const DRAW_DEFAULT: Self = Self(0);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DrawMode {
    pub kind: BYTE,
    pub factor: SWORD,
}
pub type TimeCount = DWORD;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _controller_input_state {
    pub key: [[::core::ffi::c_int; 8]; 2],
    pub menu: [::core::ffi::c_int; 31],
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
pub struct OPT_ENABLABLE(pub ::core::ffi::c_uint);
impl OPT_ENABLABLE {
    pub const OPTVAL_DISABLED: Self = Self(0);
    pub const OPTVAL_ENABLED: Self = Self(1);
}
pub type PLOT_LOCATION = plot_location;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct plot_location {
    pub star_pt: POINT,
    pub star: *mut STAR_DESC,
    pub dist_sq: [DWORD; 70],
}
pub type STAR_DESC = star_desc;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct star_desc {
    pub star_pt: POINT,
    pub Type: BYTE,
    pub Index: BYTE,
    pub Prefix: BYTE,
    pub Postfix: BYTE,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_0 {
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
pub struct C2Rust_Unnamed_1(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_1 {
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_4(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_4 {
    pub const ARILOU_DEFINED: Self = Self(0);
    pub const SOL_DEFINED: Self = Self(1);
    pub const SHOFIXTI_DEFINED: Self = Self(2);
    pub const MAIDENS_DEFINED: Self = Self(3);
    pub const START_COLONY_DEFINED: Self = Self(4);
    pub const SPATHI_DEFINED: Self = Self(5);
    pub const ZOQFOT_DEFINED: Self = Self(6);
    pub const MELNORME0_DEFINED: Self = Self(7);
    pub const MELNORME1_DEFINED: Self = Self(8);
    pub const MELNORME2_DEFINED: Self = Self(9);
    pub const MELNORME3_DEFINED: Self = Self(10);
    pub const MELNORME4_DEFINED: Self = Self(11);
    pub const MELNORME5_DEFINED: Self = Self(12);
    pub const MELNORME6_DEFINED: Self = Self(13);
    pub const MELNORME7_DEFINED: Self = Self(14);
    pub const MELNORME8_DEFINED: Self = Self(15);
    pub const TALKING_PET_DEFINED: Self = Self(16);
    pub const CHMMR_DEFINED: Self = Self(17);
    pub const SYREEN_DEFINED: Self = Self(18);
    pub const BURVIXESE_DEFINED: Self = Self(19);
    pub const SLYLANDRO_DEFINED: Self = Self(20);
    pub const DRUUGE_DEFINED: Self = Self(21);
    pub const BOMB_DEFINED: Self = Self(22);
    pub const AQUA_HELIX_DEFINED: Self = Self(23);
    pub const SUN_DEVICE_DEFINED: Self = Self(24);
    pub const TAALO_PROTECTOR_DEFINED: Self = Self(25);
    pub const SHIP_VAULT_DEFINED: Self = Self(26);
    pub const URQUAN_WRECK_DEFINED: Self = Self(27);
    pub const VUX_BEAST_DEFINED: Self = Self(28);
    pub const SAMATRA_DEFINED: Self = Self(29);
    pub const ZOQ_SCOUT_DEFINED: Self = Self(30);
    pub const MYCON_DEFINED: Self = Self(31);
    pub const EGG_CASE0_DEFINED: Self = Self(32);
    pub const EGG_CASE1_DEFINED: Self = Self(33);
    pub const EGG_CASE2_DEFINED: Self = Self(34);
    pub const PKUNK_DEFINED: Self = Self(35);
    pub const UTWIG_DEFINED: Self = Self(36);
    pub const SUPOX_DEFINED: Self = Self(37);
    pub const YEHAT_DEFINED: Self = Self(38);
    pub const VUX_DEFINED: Self = Self(39);
    pub const ORZ_DEFINED: Self = Self(40);
    pub const THRADD_DEFINED: Self = Self(41);
    pub const RAINBOW0_DEFINED: Self = Self(42);
    pub const RAINBOW1_DEFINED: Self = Self(43);
    pub const RAINBOW2_DEFINED: Self = Self(44);
    pub const RAINBOW3_DEFINED: Self = Self(45);
    pub const RAINBOW4_DEFINED: Self = Self(46);
    pub const RAINBOW5_DEFINED: Self = Self(47);
    pub const RAINBOW6_DEFINED: Self = Self(48);
    pub const RAINBOW7_DEFINED: Self = Self(49);
    pub const RAINBOW8_DEFINED: Self = Self(50);
    pub const RAINBOW9_DEFINED: Self = Self(51);
    pub const ILWRATH_DEFINED: Self = Self(52);
    pub const ANDROSYNTH_DEFINED: Self = Self(53);
    pub const MYCON_TRAP_DEFINED: Self = Self(54);
    pub const URQUAN0_DEFINED: Self = Self(55);
    pub const URQUAN1_DEFINED: Self = Self(56);
    pub const URQUAN2_DEFINED: Self = Self(57);
    pub const KOHRAH0_DEFINED: Self = Self(58);
    pub const KOHRAH1_DEFINED: Self = Self(59);
    pub const KOHRAH2_DEFINED: Self = Self(60);
    pub const DESTROYED_STARBASE_DEFINED: Self = Self(61);
    pub const MOTHER_ARK_DEFINED: Self = Self(62);
    pub const ZOQ_COLONY0_DEFINED: Self = Self(63);
    pub const ZOQ_COLONY1_DEFINED: Self = Self(64);
    pub const ZOQ_COLONY2_DEFINED: Self = Self(65);
    pub const ZOQ_COLONY3_DEFINED: Self = Self(66);
    pub const ALGOLITES_DEFINED: Self = Self(67);
    pub const SPATHI_MONUMENT_DEFINED: Self = Self(68);
    pub const EXCAVATION_SITE_DEFINED: Self = Self(69);
    pub const NUM_PLOTS: Self = Self(70);
    pub const WAR_ERA: Self = Self(70);
    pub const SUCCESS: Self = Self(70);
    pub const HOME: Self = Self(71);
}
#[inline]
unsafe extern "C" fn sameColor(mut c1: Color, mut c2: Color) -> bool {
    return c1.r as ::core::ffi::c_int == c2.r as ::core::ffi::c_int
        && c1.g as ::core::ffi::c_int == c2.g as ::core::ffi::c_int
        && c1.b as ::core::ffi::c_int == c2.b as ::core::ffi::c_int
        && c1.a as ::core::ffi::c_int == c2.a as ::core::ffi::c_int;
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
#[inline]
unsafe extern "C" fn AreTheyShades(
    mut first_color: Color,
    mut second_color: Color,
) -> BOOLEAN {
    return BOOLEAN(
        (first_color.r as ::core::ffi::c_int == first_color.g as ::core::ffi::c_int
            && first_color.g as ::core::ffi::c_int == first_color.b as ::core::ffi::c_int
            && (second_color.r as ::core::ffi::c_int
                == second_color.g as ::core::ffi::c_int
                && second_color.g as ::core::ffi::c_int
                    == second_color.b as ::core::ffi::c_int)) as ::core::ffi::c_int
            as ::core::ffi::c_uint,
    );
}
#[inline]
unsafe extern "C" fn CreateAvgShade(
    mut first_color: Color,
    mut second_color: Color,
) -> Color {
    let mut temp: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    temp = buildColorRgba(0 as BYTE, 0 as BYTE, 0 as BYTE, 0 as BYTE);
    if first_color.r as ::core::ffi::c_int > second_color.r as ::core::ffi::c_int {
        temp.r = (first_color.r as ::core::ffi::c_int
            - second_color.r as ::core::ffi::c_int) as BYTE;
        temp.g = temp.r;
        temp.b = temp.r;
    }
    if (first_color.r as ::core::ffi::c_int) < second_color.r as ::core::ffi::c_int {
        temp.r = (second_color.r as ::core::ffi::c_int
            - first_color.r as ::core::ffi::c_int) as BYTE;
        temp.g = temp.r;
        temp.b = temp.r;
    }
    if sameColor(first_color, second_color) {
        return first_color;
    }
    if temp.r as ::core::ffi::c_int == first_color.r as ::core::ffi::c_int
        || temp.r as ::core::ffi::c_int == second_color.r as ::core::ffi::c_int
    {
        temp.r = (first_color.r as ::core::ffi::c_int
            + second_color.r as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as BYTE;
        temp.g = temp.r;
        temp.b = temp.r;
    }
    if temp.r as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
        temp.a = 255 as BYTE;
    }
    return temp;
}
#[inline]
unsafe extern "C" fn MAKE_POINT(mut x: COORD, mut y: COORD) -> POINT {
    let mut pt: POINT = point { x: x, y: y };
    return pt;
}
#[inline]
unsafe extern "C" fn MAKE_DRAW_MODE(mut kind: DrawKind, mut factor: SWORD) -> DrawMode {
    let mut mode: DrawMode = DrawMode { kind: 0, factor: 0 };
    mode.kind = kind.0 as BYTE;
    mode.factor = factor;
    return mode;
}
#[no_mangle]
pub unsafe extern "C" fn DrawStarConBox(
    mut pRect: *mut RECT,
    mut BorderWidth: SIZE,
    mut TopLeftColor: Color,
    mut BottomRightColor: Color,
    mut FillInterior: BOOLEAN,
    mut InteriorColor: Color,
    mut CreateCorners: BOOLEAN,
    mut CornerColor: Color,
) {
    let mut locRect: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut oldcolor: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    BatchGraphics();
    if FillInterior.0 != 0 {
        oldcolor = SetContextForeGroundColor(InteriorColor);
        uqm_lua_checked!(DrawFilledRectangle(pRect), ());
        SetContextForeGroundColor(TopLeftColor);
    } else {
        oldcolor = SetContextForeGroundColor(TopLeftColor);
    }
    if BorderWidth as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        BorderWidth = ((2 as ::core::ffi::c_int) << resolutionFactor) as SIZE;
    }
    locRect.corner = (*pRect).corner;
    locRect.extent.width = (*pRect).extent.width;
    locRect.extent.height = ((1 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    uqm_lua_checked!(DrawFilledRectangle(&raw mut locRect), ());
    if BorderWidth as ::core::ffi::c_int == (2 as ::core::ffi::c_int) << resolutionFactor
    {
        locRect.corner.x = (locRect.corner.x as ::core::ffi::c_int
            + ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        locRect.corner.y = (locRect.corner.y as ::core::ffi::c_int
            + ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        locRect.extent.width = (locRect.extent.width as ::core::ffi::c_int
            - ((2 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        uqm_lua_checked!(DrawFilledRectangle(&raw mut locRect), ());
    }
    locRect.corner = (*pRect).corner;
    locRect.extent.width = ((1 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    locRect.extent.height = (*pRect).extent.height;
    uqm_lua_checked!(DrawFilledRectangle(&raw mut locRect), ());
    if BorderWidth as ::core::ffi::c_int == (2 as ::core::ffi::c_int) << resolutionFactor
    {
        locRect.corner.x = (locRect.corner.x as ::core::ffi::c_int
            + ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        locRect.corner.y = (locRect.corner.y as ::core::ffi::c_int
            + ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        locRect.extent.height = (locRect.extent.height as ::core::ffi::c_int
            - ((2 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        uqm_lua_checked!(DrawFilledRectangle(&raw mut locRect), ());
    }
    SetContextForeGroundColor(BottomRightColor);
    locRect.corner.x = ((*pRect).corner.x as ::core::ffi::c_int
        + (*pRect).extent.width as ::core::ffi::c_int
        - ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    locRect.corner.y = ((*pRect).corner.y as ::core::ffi::c_int
        + ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    locRect.extent.height = ((*pRect).extent.height as ::core::ffi::c_int
        - ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    uqm_lua_checked!(DrawFilledRectangle(&raw mut locRect), ());
    if BorderWidth as ::core::ffi::c_int == (2 as ::core::ffi::c_int) << resolutionFactor
    {
        locRect.corner.x = (locRect.corner.x as ::core::ffi::c_int
            - ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        locRect.corner.y = (locRect.corner.y as ::core::ffi::c_int
            + ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        locRect.extent.height = (locRect.extent.height as ::core::ffi::c_int
            - ((2 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        uqm_lua_checked!(DrawFilledRectangle(&raw mut locRect), ());
    }
    locRect.corner.x = (*pRect).corner.x;
    locRect.extent.width = (*pRect).extent.width;
    locRect.corner.y = ((*pRect).corner.y as ::core::ffi::c_int
        + (*pRect).extent.height as ::core::ffi::c_int
        - ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    locRect.extent.height = ((1 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    uqm_lua_checked!(DrawFilledRectangle(&raw mut locRect), ());
    if BorderWidth as ::core::ffi::c_int == (2 as ::core::ffi::c_int) << resolutionFactor
    {
        locRect.corner.x = (locRect.corner.x as ::core::ffi::c_int
            + ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        locRect.corner.y = (locRect.corner.y as ::core::ffi::c_int
            - ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        locRect.extent.width = (locRect.extent.width as ::core::ffi::c_int
            - ((2 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        uqm_lua_checked!(DrawFilledRectangle(&raw mut locRect), ());
    }
    if CreateCorners.0 != 0 {
        if sameColor(
            buildColorRgba(0x50 as BYTE, 0x50 as BYTE, 0x50 as BYTE, 0 as BYTE),
            CornerColor,
        ) as ::core::ffi::c_int != 0
            && AreTheyShades(TopLeftColor, BottomRightColor).0 != 0
        {
            SetContextForeGroundColor(CreateAvgShade(TopLeftColor, BottomRightColor));
        } else {
            SetContextForeGroundColor(CornerColor);
        }
        locRect.corner.x = (*pRect).corner.x;
        locRect.corner.y = ((*pRect).corner.y as ::core::ffi::c_int
            + (*pRect).extent.height as ::core::ffi::c_int
            - ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        locRect.extent.width = ((1 as ::core::ffi::c_int) << resolutionFactor) as COORD;
        locRect.extent.height = ((1 as ::core::ffi::c_int) << resolutionFactor) as COORD;
        uqm_lua_checked!(DrawFilledRectangle(&raw mut locRect), ());
        locRect.corner.x = ((*pRect).corner.x as ::core::ffi::c_int
            + (*pRect).extent.width as ::core::ffi::c_int
            - ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        locRect.corner.y = (*pRect).corner.y;
        uqm_lua_checked!(DrawFilledRectangle(&raw mut locRect), ());
        if BorderWidth as ::core::ffi::c_int
            == (2 as ::core::ffi::c_int) << resolutionFactor
        {
            locRect.corner.x = (locRect.corner.x as ::core::ffi::c_int
                - ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
            locRect.corner.y = (locRect.corner.y as ::core::ffi::c_int
                + ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
            uqm_lua_checked!(DrawFilledRectangle(&raw mut locRect), ());
            locRect.corner.x = ((*pRect).corner.x as ::core::ffi::c_int
                + ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
            locRect.corner.y = ((*pRect).corner.y as ::core::ffi::c_int
                + (*pRect).extent.height as ::core::ffi::c_int
                - ((2 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
            uqm_lua_checked!(DrawFilledRectangle(&raw mut locRect), ());
        }
    }
    SetContextForeGroundColor(oldcolor);
    UnbatchGraphics();
}
#[no_mangle]
pub unsafe extern "C" fn DrawRenderedBox(
    mut r: *mut RECT,
    mut filled: BOOLEAN,
    mut fill_color: Color,
    mut r#type: ::core::ffi::c_int,
    mut custom: ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut stamp: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    let mut columns: COORD = (*r).extent.width;
    let mut rows: COORD = (*r).extent.height;
    if (*r).extent.width == 0 || (*r).extent.height == 0 {
        return;
    }
    BatchGraphics();
    if filled.0 != 0 {
        let mut OldColor: Color = SetContextForeGroundColor(fill_color);
        if r#type == C2Rust_Unnamed_1::SPECIAL_BEVEL.0 as ::core::ffi::c_int {
            (*r).corner.x = ((*r).corner.x as ::core::ffi::c_int
                + ((4 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
            (*r).corner.y = ((*r).corner.y as ::core::ffi::c_int
                + ((4 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
            (*r).extent.width = ((*r).extent.width as ::core::ffi::c_int
                - ((8 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
            (*r).extent.height = ((*r).extent.height as ::core::ffi::c_int
                - ((8 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        }
        uqm_lua_checked!(DrawFilledRectangle(r), ());
        if r#type == C2Rust_Unnamed_1::SPECIAL_BEVEL.0 as ::core::ffi::c_int {
            (*r).corner.x = ((*r).corner.x as ::core::ffi::c_int
                - ((4 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
            (*r).corner.y = ((*r).corner.y as ::core::ffi::c_int
                - ((4 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
            (*r).extent.width = ((*r).extent.width as ::core::ffi::c_int
                + ((8 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
            (*r).extent.height = ((*r).extent.height as ::core::ffi::c_int
                + ((8 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        }
        SetContextForeGroundColor(OldColor);
    }
    stamp.frame = if custom != 0 { CustBevelFrame } else { DefBevelFrame };
    if r#type == C2Rust_Unnamed_1::SPECIAL_BEVEL.0 as ::core::ffi::c_int {
        columns = (columns as ::core::ffi::c_int
            - ((8 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        rows = (rows as ::core::ffi::c_int
            - ((8 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    }
    i = 0 as ::core::ffi::c_int;
    while i < columns as ::core::ffi::c_int {
        stamp.frame = SetAbsFrameIndex(
            stamp.frame,
            (r#type + 4 as ::core::ffi::c_int) as COUNT,
        );
        stamp.origin.x = ((*r).corner.x as ::core::ffi::c_int + i) as COORD;
        stamp.origin.y = (*r).corner.y;
        uqm_lua_checked!(DrawStamp(&raw mut stamp), ());
        stamp.frame = SetAbsFrameIndex(
            stamp.frame,
            (r#type + 6 as ::core::ffi::c_int) as COUNT,
        );
        stamp.origin.y = ((*r).corner.y as ::core::ffi::c_int
            + (*r).extent.height as ::core::ffi::c_int) as COORD;
        uqm_lua_checked!(DrawStamp(&raw mut stamp), ());
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < rows as ::core::ffi::c_int {
        stamp.frame = SetAbsFrameIndex(
            stamp.frame,
            (r#type + 5 as ::core::ffi::c_int) as COUNT,
        );
        stamp.origin.x = ((*r).corner.x as ::core::ffi::c_int
            + (*r).extent.width as ::core::ffi::c_int) as COORD;
        stamp.origin.y = ((*r).corner.y as ::core::ffi::c_int + i) as COORD;
        uqm_lua_checked!(DrawStamp(&raw mut stamp), ());
        stamp.frame = SetAbsFrameIndex(
            stamp.frame,
            (r#type + 7 as ::core::ffi::c_int) as COUNT,
        );
        stamp.origin.x = (*r).corner.x;
        uqm_lua_checked!(DrawStamp(&raw mut stamp), ());
        i += 1;
    }
    stamp.frame = SetAbsFrameIndex(stamp.frame, r#type as COUNT);
    stamp.origin.x = (*r).corner.x;
    stamp.origin.y = (*r).corner.y;
    uqm_lua_checked!(DrawStamp(&raw mut stamp), ());
    stamp.frame = SetAbsFrameIndex(
        stamp.frame,
        (r#type + 1 as ::core::ffi::c_int) as COUNT,
    );
    stamp.origin.x = ((*r).corner.x as ::core::ffi::c_int
        + (*r).extent.width as ::core::ffi::c_int) as COORD;
    uqm_lua_checked!(DrawStamp(&raw mut stamp), ());
    stamp.frame = SetAbsFrameIndex(
        stamp.frame,
        (r#type + 2 as ::core::ffi::c_int) as COUNT,
    );
    stamp.origin.y = ((*r).corner.y as ::core::ffi::c_int
        + (*r).extent.height as ::core::ffi::c_int) as COORD;
    uqm_lua_checked!(DrawStamp(&raw mut stamp), ());
    stamp.frame = SetAbsFrameIndex(
        stamp.frame,
        (r#type + 3 as ::core::ffi::c_int) as COUNT,
    );
    stamp.origin.x = (*r).corner.x;
    uqm_lua_checked!(DrawStamp(&raw mut stamp), ());
    UnbatchGraphics();
}
#[no_mangle]
pub unsafe extern "C" fn DrawBorderPadding(mut videoWidth: DWORD) {
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut OldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    let mut safe_x: UWORD = (if videoWidth != 0 && videoWidth < 280 as DWORD {
        (if if optWindowType == 1 as DWORD {
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
            } << resolutionFactor) - ((4 as ::core::ffi::c_int) << resolutionFactor)
        } else {
            0 as ::core::ffi::c_int
        }) * 2 as ::core::ffi::c_int
    } else {
        (if if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            16 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) << resolutionFactor
    }) as UWORD;
    if safe_x == 0 {
        return;
    }
    OldContext = SetContext(ScreenContext);
    if videoWidth != 0 {
        SetContextForeGroundColor(
            buildColorRgba(0xc as BYTE, 0xc as BYTE, 0xc as BYTE, 0xff as BYTE),
        );
    } else {
        SetContextForeGroundColor(colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE));
    }
    r.corner = MAKE_POINT(0 as COORD, 0 as COORD);
    r.extent.width = CanvasWidth as COORD;
    r.extent.height = (if if optWindowType == 1 as DWORD {
        BOOLEAN::TRUE.0 as ::core::ffi::c_int
    } else {
        BOOLEAN::FALSE.0 as ::core::ffi::c_int
    } != 0
    {
        16 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    } << resolutionFactor) as COORD;
    uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
    r.corner.x = (r.extent.width as ::core::ffi::c_int - safe_x as ::core::ffi::c_int)
        as COORD;
    r.extent.width = safe_x as COORD;
    r.extent.height = CanvasHeight as COORD;
    uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
    r.corner.x = 0 as COORD;
    r.corner.y = (CanvasHeight
        - (if if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            16 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        } << resolutionFactor)) as COORD;
    r.extent.width = CanvasWidth as COORD;
    r.extent.height = (if if optWindowType == 1 as DWORD {
        BOOLEAN::TRUE.0 as ::core::ffi::c_int
    } else {
        BOOLEAN::FALSE.0 as ::core::ffi::c_int
    } != 0
    {
        16 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    } << resolutionFactor) as COORD;
    uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
    r.corner = MAKE_POINT(0 as COORD, 0 as COORD);
    r.extent.width = safe_x as COORD;
    r.extent.height = CanvasHeight as COORD;
    uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
    SetContext(OldContext);
}
#[no_mangle]
pub unsafe extern "C" fn DrawRadarBorder() {
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut OldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    if if optWindowType == 1 as DWORD {
        BOOLEAN::TRUE.0 as ::core::ffi::c_int
    } else {
        BOOLEAN::FALSE.0 as ::core::ffi::c_int
    } != 0
    {
        return;
    }
    OldContext = SetContext(StatusContext);
    r.corner.x = (((4 as ::core::ffi::c_int) << resolutionFactor)
        - ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    r.corner.y = (((9 as ::core::ffi::c_int) << resolutionFactor)
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
        - ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    r.extent.width = (((64 as ::core::ffi::c_int) << resolutionFactor)
        - ((8 as ::core::ffi::c_int) << resolutionFactor)
        + ((2 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    r.extent.height = (((53 as ::core::ffi::c_int) << resolutionFactor)
        + ((2 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    if if resolutionFactor != 2 as ::core::ffi::c_uint {
        BOOLEAN::FALSE.0 as ::core::ffi::c_int
    } else {
        BOOLEAN::TRUE.0 as ::core::ffi::c_int
    } != 0 || optCustomBorder.0 != 0
    {
        uqm_lua_checked!(DrawRenderedBox(
            &raw mut r,
            BOOLEAN::FALSE,
            buildColorRgba(0 as BYTE, 0 as BYTE, 0 as BYTE, 0 as BYTE),
            C2Rust_Unnamed_1::THIN_INNER_BEVEL.0 as ::core::ffi::c_int,
            optCustomBorder.0 as ::core::ffi::c_int,
        ), ());
    } else {
        uqm_lua_checked!(DrawStarConBox(
            &raw mut r,
            ((1 as ::core::ffi::c_int) << resolutionFactor) as SIZE,
            colorFromRgb15(0xe as BYTE, 0xe as BYTE, 0xe as BYTE),
            colorFromRgb15(0x6 as BYTE, 0x6 as BYTE, 0x6 as BYTE),
            BOOLEAN::FALSE,
            buildColorRgba(0x50 as BYTE, 0x50 as BYTE, 0x50 as BYTE, 0 as BYTE),
            BOOLEAN::FALSE,
            buildColorRgba(0x50 as BYTE, 0x50 as BYTE, 0x50 as BYTE, 0 as BYTE),
        ), ());
    }
    SetContext(OldContext);
}
#[no_mangle]
pub unsafe extern "C" fn SeedRandomNumbers() -> DWORD {
    let mut cur_time: DWORD = 0;
    cur_time = GetTimeCounter() as DWORD;
    TFB_SeedRandom(cur_time);
    return cur_time;
}
#[no_mangle]
pub unsafe extern "C" fn SaveContextFrame(mut saveRect: *const RECT) -> STAMP {
    let mut s: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    if !saveRect.is_null() {
        s.origin = (*saveRect).corner;
    } else {
        s.origin.x = 0 as COORD;
        s.origin.y = 0 as COORD;
    }
    s.frame = CaptureDrawable(CopyContextRect(saveRect));
    return s;
}
unsafe extern "C" fn DrawPauseText(mut rect: *mut RECT) {
    let mut text: TEXT = TEXT {
        baseline: POINT { x: 0, y: 0 },
        pStr: ::core::ptr::null::<UNICODE>(),
        align: TEXT_ALIGN::ALIGN_LEFT,
        CharCount: 0,
    };
    let mut OldFont: FONT = ::core::ptr::null_mut::<FONT_DESC>();
    let mut OldColor: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    let mut block: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    if strlen(
        GetStringAddress(
            SetAbsStringTableIndex(
                GameStrings,
                (C2Rust_Unnamed_3::QUITMENU_STRING_BASE.0 as ::core::ffi::c_int
                    + 4 as ::core::ffi::c_int) as COUNT,
            ),
        ),
    ) == 0
    {
        return;
    }
    OldFont = SetContextFont(
        if if optWindowType == 0 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            StarConFont
        } else {
            LabelFont
        },
    );
    OldColor = SetContextForeGroundColor(
        buildColorRgba(
            (if if optWindowType == 0 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                0x6b as ::core::ffi::c_int
            } else {
                0x74 as ::core::ffi::c_int
            }) as BYTE,
            (if if optWindowType == 0 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                0x6b as ::core::ffi::c_int
            } else {
                0x74 as ::core::ffi::c_int
            }) as BYTE,
            (if if optWindowType == 0 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                0x6b as ::core::ffi::c_int
            } else {
                0x74 as ::core::ffi::c_int
            }) as BYTE,
            0xff as BYTE,
        ),
    );
    block = *rect;
    block.extent.width = (block.extent.width as ::core::ffi::c_int
        - ((4 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    block.extent.height = (block.extent.height as ::core::ffi::c_int
        - ((4 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    block.corner.x = (block.corner.x as ::core::ffi::c_int
        + ((2 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    block.corner.y = (block.corner.y as ::core::ffi::c_int
        + ((2 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    uqm_lua_checked!(DrawFilledRectangle(&raw mut block), ());
    SetContextForeGroundColor(
        if if optWindowType == 0 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            colorFromRgb15(0x1f as BYTE, 0x1f as BYTE, 0x1f as BYTE)
        } else {
            colorFromRgb15(0x8 as BYTE, 0x8 as BYTE, 0x8 as BYTE)
        },
    );
    text.baseline = (*rect).corner;
    text.baseline.x = (text.baseline.x as ::core::ffi::c_int
        + ((*rect).extent.width as ::core::ffi::c_int >> 1 as ::core::ffi::c_int))
        as COORD;
    text.baseline.y = (text.baseline.y as ::core::ffi::c_int
        + ((10 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    text.align = TEXT_ALIGN::ALIGN_CENTER;
    text.pStr = AlignText(
        GetStringAddress(
            SetAbsStringTableIndex(
                GameStrings,
                (C2Rust_Unnamed_3::QUITMENU_STRING_BASE.0 as ::core::ffi::c_int
                    + 4 as ::core::ffi::c_int) as COUNT,
            ),
        ) as *const UNICODE,
        &raw mut text.baseline.x,
    );
    text.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    uqm_lua_checked!(font_DrawText(&raw mut text), ());
    SetContextFont(OldFont);
    SetContextForeGroundColor(OldColor);
}
#[no_mangle]
pub unsafe extern "C" fn PauseGame() -> BOOLEAN {
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut s: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    let mut OldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    let mut saveStamp: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    let mut ctxRect: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut OldOrigin: POINT = POINT { x: 0, y: 0 };
    let mut OldRect: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut OldColor: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    let mut mode: DrawMode = DrawMode { kind: 0, factor: 0 };
    let mut oldMode: DrawMode = DrawMode { kind: 0, factor: 0 };
    let mut oldVolume: BYTE = 0;
    let mut deltaT: TimeCount = 0;
    if ActivityFrame.is_null()
        || GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
            & (C2Rust_Unnamed_2::CHECK_ABORT.0 as ::core::ffi::c_int
                | C2Rust_Unnamed_2::CHECK_PAUSE.0 as ::core::ffi::c_int) != 0
        || LastActivity as ::core::ffi::c_int
            & (C2Rust_Unnamed_2::CHECK_LOAD.0 as ::core::ffi::c_int
                | C2Rust_Unnamed_2::CHECK_RESTART.0 as ::core::ffi::c_int) != 0
    {
        return BOOLEAN::FALSE;
    }
    GlobData.Game_state.CurrentActivity = (GlobData.Game_state.CurrentActivity
        as ::core::ffi::c_int | C2Rust_Unnamed_2::CHECK_PAUSE.0 as ::core::ffi::c_int)
        as ACTIVITY;
    if PlayingTrack() != 0 {
        uqm_lua_checked!(PauseTrack(), BOOLEAN(0));
    }
    deltaT = GetTimeCounter();
    OldContext = SetContext(ScreenContext);
    OldOrigin = SetContextOrigin(MAKE_POINT(0 as COORD, 0 as COORD));
    GetContextClipRect(&raw mut OldRect);
    SetContextClipRect(::core::ptr::null_mut::<RECT>());
    GetContextClipRect(&raw mut ctxRect);
    GetFrameRect(ActivityFrame, &raw mut r);
    r.corner.x = (ctxRect.extent.width as ::core::ffi::c_int
        - r.extent.width as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as COORD;
    r.corner.y = (ctxRect.extent.height as ::core::ffi::c_int
        - r.extent.height as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as COORD;
    saveStamp = SaveContextFrame(&raw mut ctxRect);
    mode = MAKE_DRAW_MODE(DrawKind::DRAW_DESATURATE, 0xbe as SWORD);
    oldMode = SetContextDrawMode(mode);
    uqm_lua_checked!(DrawFilledRectangle(&raw mut ctxRect), BOOLEAN(0));
    SetContextDrawMode(oldMode);
    OldColor = SetContextForeGroundColor(
        buildColorRgba(0 as BYTE, 0 as BYTE, 0 as BYTE, 0x30 as BYTE),
    );
    uqm_lua_checked!(DrawFilledRectangle(&raw mut ctxRect), BOOLEAN(0));
    SetContextForeGroundColor(OldColor);
    s.origin = r.corner;
    s.frame = ActivityFrame;
    uqm_lua_checked!(DrawStamp(&raw mut s), BOOLEAN(0));
    uqm_lua_checked!(DrawPauseText(&raw mut r), BOOLEAN(0));
    oldVolume = GetCurrMusicVol();
    uqm_lua_checked!(FadeMusic(60 as BYTE, (840 as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as SIZE), BOOLEAN(0));
    FlushGraphics();
    while ::core::ptr::read_volatile::<
        ::core::ffi::c_int,
    >(
        &raw const ImmediateInputState
            .menu[C2Rust_Unnamed::KEY_PAUSE.0 as ::core::ffi::c_int as usize],
    ) != 0 && ::core::ptr::read_volatile::<BOOLEAN>(&raw const GamePaused).0 != 0
    {
        BeginInputFrame();
        TaskSwitch();
    }
    while ::core::ptr::read_volatile::<
        ::core::ffi::c_int,
    >(
        &raw const ImmediateInputState
            .menu[C2Rust_Unnamed::KEY_PAUSE.0 as ::core::ffi::c_int as usize],
    ) == 0 && ::core::ptr::read_volatile::<BOOLEAN>(&raw const GamePaused).0 != 0
    {
        BeginInputFrame();
        TaskSwitch();
    }
    while ::core::ptr::read_volatile::<
        ::core::ffi::c_int,
    >(
        &raw const ImmediateInputState
            .menu[C2Rust_Unnamed::KEY_PAUSE.0 as ::core::ffi::c_int as usize],
    ) != 0 && ::core::ptr::read_volatile::<BOOLEAN>(&raw const GamePaused).0 != 0
    {
        BeginInputFrame();
        TaskSwitch();
    }
    ::core::ptr::write_volatile(&raw mut GamePaused, BOOLEAN::FALSE);
    uqm_lua_checked!(DrawStamp(&raw mut saveStamp), BOOLEAN(0));
    DestroyDrawable(ReleaseDrawable(saveStamp.frame));
    ClearSystemRect();
    uqm_lua_checked!(FadeMusic(oldVolume, (840 as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as SIZE), BOOLEAN(0));
    SetContextClipRect(&raw mut OldRect);
    SetContextOrigin(OldOrigin);
    SetContext(OldContext);
    uqm_lua_checked!(WaitForNoInput(
        (840 as ::core::ffi::c_int / 4 as ::core::ffi::c_int) as TimePeriod,
        BOOLEAN::TRUE,
    ), BOOLEAN(0));
    DeltaLastTime(GetTimeCounter().wrapping_sub(deltaT));
    if PlayingTrack() != 0 {
        uqm_lua_checked!(ResumeTrack(), BOOLEAN(0));
    }
    TaskSwitch();
    GlobData.Game_state.CurrentActivity = (GlobData.Game_state.CurrentActivity
        as ::core::ffi::c_int & !(C2Rust_Unnamed_2::CHECK_PAUSE.0 as ::core::ffi::c_int))
        as ACTIVITY;
    return BOOLEAN::TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn WaitForAnyButtonUntil(
    mut newButton: BOOLEAN,
    mut timeOut: TimeCount,
    mut resetInput: BOOLEAN,
) -> BOOLEAN {
    let mut buttonPressed: BOOLEAN = BOOLEAN::FALSE;
    if newButton.0 != 0 && uqm_lua_checked!(WaitForNoInputUntil(timeOut, BOOLEAN::FALSE), BOOLEAN(0)).0 == 0 {
        return BOOLEAN::FALSE;
    }
    buttonPressed = uqm_lua_checked!(AnyButtonPress(BOOLEAN::TRUE), BOOLEAN(0));
    while buttonPressed.0 == 0
        && (timeOut == -1 as ::core::ffi::c_int as TimePeriod
            || GetTimeCounter() < timeOut)
        && GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
            & C2Rust_Unnamed_2::CHECK_ABORT.0 as ::core::ffi::c_int == 0
        && ::core::ptr::read_volatile::<::core::ffi::c_int>(&raw const QuitPosted) == 0
    {
        uqm_lua_checked!(SleepThread(
            (840 as ::core::ffi::c_int / 40 as ::core::ffi::c_int) as TimePeriod,
        ), BOOLEAN(0));
        buttonPressed = uqm_lua_checked!(AnyButtonPress(BOOLEAN::TRUE), BOOLEAN(0));
    }
    if resetInput.0 != 0 {
        FlushInput();
    }
    return buttonPressed;
}
#[no_mangle]
pub unsafe extern "C" fn WaitForActButtonUntil(
    mut newButton: BOOLEAN,
    mut timeOut: TimeCount,
    mut resetInput: BOOLEAN,
) -> BOOLEAN {
    let mut buttonPressed: BOOLEAN = BOOLEAN::FALSE;
    if newButton.0 != 0 && uqm_lua_checked!(WaitForNoInputUntil(timeOut, BOOLEAN::FALSE), BOOLEAN(0)).0 == 0 {
        return BOOLEAN::FALSE;
    }
    buttonPressed = uqm_lua_checked!(ActKeysPress(), BOOLEAN(0));
    while buttonPressed.0 == 0
        && (timeOut == -1 as ::core::ffi::c_int as TimePeriod
            || GetTimeCounter() < timeOut)
        && GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
            & C2Rust_Unnamed_2::CHECK_ABORT.0 as ::core::ffi::c_int == 0
        && ::core::ptr::read_volatile::<::core::ffi::c_int>(&raw const QuitPosted) == 0
    {
        uqm_lua_checked!(SleepThread(
            (840 as ::core::ffi::c_int / 40 as ::core::ffi::c_int) as TimePeriod,
        ), BOOLEAN(0));
        buttonPressed = uqm_lua_checked!(ActKeysPress(), BOOLEAN(0));
    }
    if resetInput.0 != 0 {
        FlushInput();
    }
    return buttonPressed;
}
#[no_mangle]
pub unsafe extern "C" fn WaitForAnyButton(
    mut newButton: BOOLEAN,
    mut duration: TimePeriod,
    mut resetInput: BOOLEAN,
) -> BOOLEAN {
    let mut timeOut: TimeCount = duration;
    if duration != -1 as ::core::ffi::c_int as TimePeriod {
        timeOut = timeOut.wrapping_add(GetTimeCounter());
    }
    return uqm_lua_checked!(WaitForAnyButtonUntil(newButton, timeOut, resetInput), BOOLEAN(0));
}
#[no_mangle]
pub unsafe extern "C" fn WaitForActButton(
    mut newButton: BOOLEAN,
    mut duration: TimePeriod,
    mut resetInput: BOOLEAN,
) -> BOOLEAN {
    let mut timeOut: TimeCount = duration;
    if duration != -1 as ::core::ffi::c_int as TimePeriod {
        timeOut = timeOut.wrapping_add(GetTimeCounter());
    }
    return uqm_lua_checked!(WaitForActButtonUntil(newButton, timeOut, resetInput), BOOLEAN(0));
}
#[no_mangle]
pub unsafe extern "C" fn WaitForNoInputUntil(
    mut timeOut: TimeCount,
    mut resetInput: BOOLEAN,
) -> BOOLEAN {
    let mut buttonPressed: BOOLEAN = BOOLEAN::FALSE;
    buttonPressed = uqm_lua_checked!(AnyButtonPress(BOOLEAN::TRUE), BOOLEAN(0));
    while buttonPressed.0 != 0
        && (timeOut == -1 as ::core::ffi::c_int as TimePeriod
            || GetTimeCounter() < timeOut)
        && GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
            & C2Rust_Unnamed_2::CHECK_ABORT.0 as ::core::ffi::c_int == 0
        && ::core::ptr::read_volatile::<::core::ffi::c_int>(&raw const QuitPosted) == 0
    {
        uqm_lua_checked!(SleepThread(
            (840 as ::core::ffi::c_int / 40 as ::core::ffi::c_int) as TimePeriod,
        ), BOOLEAN(0));
        buttonPressed = uqm_lua_checked!(AnyButtonPress(BOOLEAN::TRUE), BOOLEAN(0));
    }
    if resetInput.0 != 0 {
        FlushInput();
    }
    return BOOLEAN((buttonPressed.0 == 0) as ::core::ffi::c_int as ::core::ffi::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn WaitForNoInput(
    mut duration: TimePeriod,
    mut resetInput: BOOLEAN,
) -> BOOLEAN {
    let mut timeOut: TimeCount = duration;
    if duration != -1 as ::core::ffi::c_int as TimePeriod {
        timeOut = timeOut.wrapping_add(GetTimeCounter());
    }
    return uqm_lua_checked!(WaitForNoInputUntil(timeOut, resetInput), BOOLEAN(0));
}
#[no_mangle]
pub unsafe extern "C" fn SleepGame() {
    if ::core::ptr::read_volatile::<::core::ffi::c_int>(&raw const QuitPosted) != 0 {
        return;
    }
    log_add(
        log_Level::log_Debug,
        b"Game is going to sleep\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if PlayingTrack() != 0 {
        uqm_lua_checked!(PauseTrack(), ());
    }
    uqm_lua_checked!(PauseMusic(), ());
    while ::core::ptr::read_volatile::<::core::ffi::c_int>(&raw const GameActive) == 0
        && ::core::ptr::read_volatile::<::core::ffi::c_int>(&raw const QuitPosted) == 0
    {
        uqm_lua_checked!(SleepThread((840 as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as TimePeriod), ());
    }
    log_add(
        log_Level::log_Debug,
        b"Game is waking up\0".as_ptr() as *const ::core::ffi::c_char,
    );
    uqm_lua_checked!(WaitForNoInput(
        (840 as ::core::ffi::c_int / 10 as ::core::ffi::c_int) as TimePeriod,
        BOOLEAN::TRUE,
    ), ());
    uqm_lua_checked!(ResumeMusic(), ());
    if PlayingTrack() != 0 {
        uqm_lua_checked!(ResumeTrack(), ());
    }
    TaskSwitch();
}
#[no_mangle]
pub unsafe extern "C" fn get_fuel_to_sol() -> DWORD {
    let mut pt: POINT = POINT { x: 0, y: 0 };
    let mut f: DWORD = 0;
    pt.x = logxToUniverse(GlobData.SIS_state.log_x) as COORD;
    pt.y = logyToUniverse(GlobData.SIS_state.log_y) as COORD;
    pt.x = (pt.x as ::core::ffi::c_int
        - plot_map[C2Rust_Unnamed_4::SOL_DEFINED.0 as ::core::ffi::c_int as usize]
            .star_pt
            .x as ::core::ffi::c_int) as COORD;
    pt.y = (pt.y as ::core::ffi::c_int
        - plot_map[C2Rust_Unnamed_4::SOL_DEFINED.0 as ::core::ffi::c_int as usize]
            .star_pt
            .y as ::core::ffi::c_int) as COORD;
    f = (pt.x as ::core::ffi::c_long * pt.x as ::core::ffi::c_long
        + pt.y as ::core::ffi::c_long * pt.y as ::core::ffi::c_long) as DWORD;
    if f == 0 as DWORD
        || uqm_lua_checked!(getGameStateUint(
            b"ARILOU_SPACE_SIDE\0".as_ptr() as *const ::core::ffi::c_char,
        ), 0) > 1 as DWORD
    {
        return 0 as DWORD
    } else {
        return (square_root(f) as ::core::ffi::c_int
            + 100 as ::core::ffi::c_int / 20 as ::core::ffi::c_int) as DWORD
    };
}
#[no_mangle]
pub unsafe extern "C" fn DrawFlagStatDisplay(mut str: *mut UNICODE) {
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
    r.corner.x = ((2 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    r.corner.y = ((20 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    r.extent.width = (((64 as ::core::ffi::c_int) << resolutionFactor)
        - ((5 as ::core::ffi::c_int) << resolutionFactor)
        + ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    r.extent.height = (((129 as ::core::ffi::c_int) << resolutionFactor)
        - r.corner.y as ::core::ffi::c_int) as COORD;
    if optCustomBorder.0 == 0
        && if resolutionFactor != 2 as ::core::ffi::c_uint {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } == 0
    {
        uqm_lua_checked!(DrawStarConBox(
            &raw mut r,
            ((1 as ::core::ffi::c_int) << resolutionFactor) as SIZE,
            colorFromRgb15(0x10 as BYTE, 0x10 as BYTE, 0x10 as BYTE),
            colorFromRgb15(0x8 as BYTE, 0x8 as BYTE, 0x8 as BYTE),
            BOOLEAN::TRUE,
            buildColorRgba(0 as BYTE, 0 as BYTE, 0xaa as BYTE, 0xff as BYTE),
            BOOLEAN::FALSE,
            buildColorRgba(0x50 as BYTE, 0x50 as BYTE, 0x50 as BYTE, 0 as BYTE),
        ), ());
    } else {
        uqm_lua_checked!(DrawBorder(C2Rust_Unnamed_0::DEVICE_CARGO_FRAME.0 as ::core::ffi::c_int as BYTE), ());
    }
    SetContextFont(StarConFont);
    t.baseline.x = (((64 as ::core::ffi::c_int) << resolutionFactor
        >> 1 as ::core::ffi::c_int) - ((1 as ::core::ffi::c_int) << resolutionFactor))
        as COORD;
    t.baseline.y = (r.corner.y as ::core::ffi::c_int
        + ((7 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    t.align = TEXT_ALIGN::ALIGN_CENTER;
    t.pStr = str;
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    SetContextForeGroundColor(
        buildColorRgba(0x55 as BYTE, 0xff as BYTE, 0xff as BYTE, 0xff as BYTE),
    );
    uqm_lua_checked!(font_DrawText(&raw mut t), ());
}
#[no_mangle]
pub unsafe extern "C" fn WholeFuelValue() -> *mut UNICODE {
    static mut buf: [UNICODE; 7] = [0; 7];
    let mut CoarseFuel: DWORD = GlobData.SIS_state.FuelOnBoard;
    let mut dblFuelOnBoard: ::core::ffi::c_double = CoarseFuel as ::core::ffi::c_double
        / 100 as ::core::ffi::c_int as ::core::ffi::c_double;
    if optInfiniteFuel.0 == 0 {
        if optWholeFuel.0 == 0 {
            snprintf(
                &raw mut buf as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[UNICODE; 7]>(),
                b"%u\0".as_ptr() as *const ::core::ffi::c_char,
                CoarseFuel,
            );
        } else if dblFuelOnBoard > 999.99f64 {
            snprintf(
                &raw mut buf as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[UNICODE; 7]>(),
                b"%.1f\0".as_ptr() as *const ::core::ffi::c_char,
                dblFuelOnBoard,
            );
        } else {
            snprintf(
                &raw mut buf as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[UNICODE; 7]>(),
                b"%.2f\0".as_ptr() as *const ::core::ffi::c_char,
                dblFuelOnBoard,
            );
        }
    } else {
        snprintf(
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[UNICODE; 7]>(),
            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
            b"\xE2\x88\x9E\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    return &raw mut buf as *mut UNICODE;
}
#[inline]
unsafe extern "C" fn logxToUniverse(mut lx: SDWORD) -> SDWORD {
    return (lx * (9999 as SDWORD + 1 as SDWORD >> 4 as ::core::ffi::c_int)
        + ((9999 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
            >> 4 as ::core::ffi::c_int)
            * ((16 as ::core::ffi::c_int) << resolutionFactor)
            >> 1 as ::core::ffi::c_int))
        / ((9999 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
            >> 4 as ::core::ffi::c_int)
            * ((16 as ::core::ffi::c_int) << resolutionFactor));
}
#[inline]
unsafe extern "C" fn logyToUniverse(mut ly: SDWORD) -> SDWORD {
    return 9999 as SDWORD
        - (ly * (9999 as SDWORD + 1 as SDWORD >> 4 as ::core::ffi::c_int)
            + ((9999 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                >> 4 as ::core::ffi::c_int)
                * ((16 as ::core::ffi::c_int) << resolutionFactor)
                >> 1 as ::core::ffi::c_int))
            / ((9999 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                >> 4 as ::core::ffi::c_int)
                * ((16 as ::core::ffi::c_int) << resolutionFactor));
}
