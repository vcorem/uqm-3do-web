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

// This file handles loading of teams, but the UI and the actual loading.

// Rust translation of src/uqm/supermelee/loadmele.c.
extern "C" {
    pub type uio_DirHandle;
    pub type uio_Stream;
    pub type string_table_entry;
    pub type string_table;
    pub type context_desc;
    pub type frame_desc;
    pub type font_desc;
    pub type drawable_desc;
    pub type tfb_soundsample;
    pub type RandomContext;
    fn SetContext(Context: CONTEXT) -> CONTEXT;
    fn SetContextForeGroundColor(Color: Color) -> Color;
    fn DrawStamp(pStamp: *mut STAMP);
    fn font_DrawText(pText: *mut TEXT);
    fn BatchGraphics();
    fn UnbatchGraphics();
    fn DestroyDrawable(Drawable: DRAWABLE) -> BOOLEAN;
    fn GetFrameRect(Frame: FRAME, pRect: *mut RECT) -> BOOLEAN;
    fn SetContextFont(Font: FONT) -> FONT;
    fn SetAbsFrameIndex(Frame: FRAME, FrameIndex: COUNT) -> FRAME;
    fn ReleaseDrawable(Frame: FRAME) -> DRAWABLE;
    fn HMalloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn HFree(p: *mut ::core::ffi::c_void);
    fn __assert_fail(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    ) -> !;
    fn uio_unlink(
        dirHandle: *mut uio_DirHandle,
        path: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn uio_fopen(
        dir: *mut uio_DirHandle,
        path: *const ::core::ffi::c_char,
        mode: *const ::core::ffi::c_char,
    ) -> *mut uio_Stream;
    fn uio_fclose(stream: *mut uio_Stream) -> ::core::ffi::c_int;
    fn sprintf(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn snprintf(
        _: *mut ::core::ffi::c_char,
        _: size_t,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn LoadDirEntryTable(
        dirHandle: *mut uio_DirHandle,
        path: *const ::core::ffi::c_char,
        pattern: *const ::core::ffi::c_char,
        matchType: match_MatchType,
    ) -> DIRENTRY_REF;
    fn DestroyStringTable(StringTable: STRING_TABLE) -> BOOLEAN;
    fn CaptureStringTable(StringTable: STRING_TABLE) -> STRING;
    fn ReleaseStringTable(String: STRING) -> STRING_TABLE;
    fn GetStringTableCount(String: STRING) -> COUNT;
    fn SetAbsStringTableIndex(String: STRING, StringTableIndex: COUNT) -> STRING;
    fn GetStringAddress(String: STRING) -> STRINGPTR;
    fn GetTimeCounter() -> TimeCount;
    fn memmove(
        _: *mut ::core::ffi::c_void,
        _: *const ::core::ffi::c_void,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strcasecmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    static mut PulsedInputState: CONTROLLER_INPUT_STATE;
    static mut GamePaused: BOOLEAN;
    static mut resolutionFactor: ::core::ffi::c_uint;
    static mut optWindowType: DWORD;
    static mut meleeDir: *mut uio_DirHandle;
    fn SleepThreadUntil(wakeTime: TimeCount);
    static mut GlobData: GLOBDATA;
    fn DoMelee(pMS: *mut MELEE_STATE) -> BOOLEAN;
    fn DrawMeleeIcon(which_icon: COUNT, HiLite: BOOLEAN);
    fn RepairMeleeFrame(pRect: *const RECT);
    static mut MeleeFrame: FRAME;
    fn QuickRepair(whichFrame: COUNT, pRect: *mut RECT);
    fn Melee_LocalChange_team(
        pMS: *mut MELEE_STATE,
        teamNr: size_t,
        team: *const MeleeTeam,
    ) -> bool;
    fn MeleeTeam_new() -> *mut MeleeTeam;
    fn MeleeTeam_delete(team: *mut MeleeTeam);
    fn MeleeTeam_serialize(
        team: *const MeleeTeam,
        stream: *mut uio_Stream,
    ) -> ::core::ffi::c_int;
    fn MeleeTeam_deserialize(
        team: *mut MeleeTeam,
        stream: *mut uio_Stream,
    ) -> ::core::ffi::c_int;
    fn MeleeTeam_getValue(team: *const MeleeTeam) -> COUNT;
    fn MeleeTeam_setShip(team: *mut MeleeTeam, slotNr: FleetShipIndex, ship: MeleeShip);
    fn MeleeTeam_getTeamName(team: *const MeleeTeam) -> *const ::core::ffi::c_char;
    fn MeleeTeam_setName(team: *mut MeleeTeam, name: *const ::core::ffi::c_char);
    fn MeleeTeam_copy(copy: *mut MeleeTeam, original: *const MeleeTeam);
    fn MeleeSetup_getTeamName(
        setup: *const MeleeSetup,
        teamNr: size_t,
    ) -> *const ::core::ffi::c_char;
    static mut GameStrings: STRING;
    fn ConfirmSaveLoad(MsgStamp: *mut STAMP);
    fn GetShipIconsFromIndex(Index: ::core::ffi::c_uint) -> FRAME;
    static mut ScreenContext: CONTEXT;
    static mut SpaceContext: CONTEXT;
    static mut MicroFont: FONT;
    fn SetMenuSounds(sound_0: MENU_SOUND_FLAGS, sound_1: MENU_SOUND_FLAGS);
    fn SaveProblem();
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct match_MatchType(pub ::core::ffi::c_uint);
impl match_MatchType {
    pub const match_MATCH_LITERAL: Self = Self(0);
    pub const match_MATCH_PREFIX: Self = Self(1);
    pub const match_MATCH_SUFFIX: Self = Self(2);
    pub const match_MATCH_SUBSTRING: Self = Self(3);
    pub const match_MATCH_REGEX: Self = Self(4);
}
pub type STRING_TABLE_ENTRY_DESC = string_table_entry;
pub type STRING_TABLE_DESC = string_table;
pub type STRING_TABLE = *mut STRING_TABLE_DESC;
pub type STRING = *mut STRING_TABLE_ENTRY_DESC;
pub type STRINGPTR = *mut ::core::ffi::c_char;
pub type DIRENTRY_REF = STRING_TABLE;
pub type DIRENTRY = STRING;
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
pub type TimeCount = DWORD;
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
pub type QUEUE_HANDLE = *mut ::core::ffi::c_void;
pub type HLINK = QUEUE_HANDLE;
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
pub type TFB_SoundSample = tfb_soundsample;
pub type MUSIC_REF = *mut *mut TFB_SoundSample;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct velocity_desc {
    pub TravelAngle: COUNT,
    pub vector: EXTENT,
    pub fract: EXTENT,
    pub error: EXTENT,
    pub incr: EXTENT,
}
pub type VELOCITY_DESC = velocity_desc;
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
pub struct GLOBDATA {
    pub SIS_state: SIS_STATE,
    pub Game_state: GAME_STATE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct melee_state {
    pub InputFunc: Option<unsafe extern "C" fn(*mut melee_state) -> BOOLEAN>,
    pub Initialized: BOOLEAN,
    pub meleeStarted: BOOLEAN,
    pub MeleeOption: MELEE_OPTIONS,
    pub side: COUNT,
    pub row: COUNT,
    pub col: COUNT,
    pub meleeSetup: *mut MeleeSetup,
    pub load: melee_load_state,
    pub currentShip: MeleeShip,
    pub CurIndex: COUNT,
    pub buildPickConfirmed: BOOLEAN,
    pub randomContext: *mut RandomContext,
    pub LastInputTime: TimeCount,
    pub hMusic: MUSIC_REF,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct MeleeShip(pub ::core::ffi::c_uint);
impl MeleeShip {
    pub const MELEE_ANDROSYNTH: Self = Self(0);
    pub const MELEE_ARILOU: Self = Self(1);
    pub const MELEE_CHENJESU: Self = Self(2);
    pub const MELEE_CHMMR: Self = Self(3);
    pub const MELEE_DRUUGE: Self = Self(4);
    pub const MELEE_EARTHLING: Self = Self(5);
    pub const MELEE_ILWRATH: Self = Self(6);
    pub const MELEE_KOHR_AH: Self = Self(7);
    pub const MELEE_MELNORME: Self = Self(8);
    pub const MELEE_MMRNMHRM: Self = Self(9);
    pub const MELEE_MYCON: Self = Self(10);
    pub const MELEE_ORZ: Self = Self(11);
    pub const MELEE_PKUNK: Self = Self(12);
    pub const MELEE_SHOFIXTI: Self = Self(13);
    pub const MELEE_SLYLANDRO: Self = Self(14);
    pub const MELEE_SPATHI: Self = Self(15);
    pub const MELEE_SUPOX: Self = Self(16);
    pub const MELEE_SYREEN: Self = Self(17);
    pub const MELEE_THRADDASH: Self = Self(18);
    pub const MELEE_UMGAH: Self = Self(19);
    pub const MELEE_URQUAN: Self = Self(20);
    pub const MELEE_UTWIG: Self = Self(21);
    pub const MELEE_VUX: Self = Self(22);
    pub const MELEE_YEHAT: Self = Self(23);
    pub const MELEE_ZOQFOTPIK: Self = Self(24);
    pub const MELEE_UNSET: Self = Self(254);
    pub const MELEE_NONE: Self = Self(255);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct melee_load_state {
    pub preBuiltList: *mut *mut MeleeTeam,
    pub preBuiltCount: COUNT,
    pub dirEntries: DIRENTRY,
    pub entryIndices: *mut COUNT,
    pub numIndices: COUNT,
    pub view: [*mut MeleeTeam; 5],
    pub top: COUNT,
    pub bot: COUNT,
    pub cur: COUNT,
    pub viewSize: COUNT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MeleeTeam {
    pub ships: [MeleeShip; 14],
    pub name: [::core::ffi::c_char; 55],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MeleeSetup {
    pub teams: [MeleeTeam; 2],
    pub fleetValue: [COUNT; 2],
}
pub type MELEE_OPTIONS = BYTE;
pub type MELEE_STATE = melee_state;
pub type FleetShipIndex = COUNT;
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
pub type MENU_SOUND_FLAGS = UWORD;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_1(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_1 {
    pub const MELEE_BACKGROUND: Self = Self(0);
    pub const HUMAN_CON_TOP: Self = Self(1);
    pub const WEAK_CYBORG_TOP: Self = Self(2);
    pub const GOOD_CYBORG_TOP: Self = Self(3);
    pub const AWES_CYBORG_TOP: Self = Self(4);
    pub const HUMAN_CON_TOP_HL: Self = Self(5);
    pub const WEAK_CYBORG_TOP_HL: Self = Self(6);
    pub const GOOD_CYBORG_TOP_HL: Self = Self(7);
    pub const AWES_CYBORG_TOP_HL: Self = Self(8);
    pub const HUMAN_CON_BOTT: Self = Self(9);
    pub const WEAK_CYBORG_BOTT: Self = Self(10);
    pub const GOOD_CYBORG_BOTT: Self = Self(11);
    pub const AWES_CYBORG_BOTT: Self = Self(12);
    pub const HUMAN_CON_BOTT_HL: Self = Self(13);
    pub const WEAK_CYBORG_BOTT_HL: Self = Self(14);
    pub const GOOD_CYBORG_BOTT_HL: Self = Self(15);
    pub const AWES_CYBORG_BOTT_HL: Self = Self(16);
    pub const LOAD_BUTTON_TOP: Self = Self(17);
    pub const SAVE_BUTTON_TOP: Self = Self(18);
    pub const LOAD_BUTTON_TOP_HL: Self = Self(19);
    pub const SAVE_BUTTON_TOP_HL: Self = Self(20);
    pub const SAVE_BUTTON_BOTT: Self = Self(21);
    pub const LOAD_BUTTON_BOTT: Self = Self(22);
    pub const SAVE_BUTTON_BOTT_HL: Self = Self(23);
    pub const LOAD_BUTTON_BOTT_HL: Self = Self(24);
    pub const BATTLE_BUTTON: Self = Self(25);
    pub const BATTLE_BUTTON_HL: Self = Self(26);
    pub const SHIP_PICKER: Self = Self(27);
    pub const TEAM_PICKER: Self = Self(28);
    pub const QUIT_BUTTON: Self = Self(29);
    pub const QUIT_BUTTON_HL: Self = Self(30);
    pub const NETWORK_CON_TOP: Self = Self(31);
    pub const NETWORK_CON_TOP_HL: Self = Self(32);
    pub const NETWORK_CON_BOTT: Self = Self(33);
    pub const NETWORK_CON_BOTT_HL: Self = Self(34);
    pub const NET_BUTTON_TOP: Self = Self(35);
    pub const NET_BUTTON_TOP_HL: Self = Self(36);
    pub const NET_BUTTON_BOTT: Self = Self(37);
    pub const NET_BUTTON_BOTT_HL: Self = Self(38);
    pub const SHIP_DESC: Self = Self(39);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_2(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_2 {
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
unsafe extern "C" fn LoadTeamImage(
    mut DirEntry: DIRENTRY,
    mut team: *mut MeleeTeam,
) -> bool {
    let mut fileName: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    let mut stream: *mut uio_Stream = ::core::ptr::null_mut::<uio_Stream>();
    fileName = GetStringAddress(DirEntry) as *const ::core::ffi::c_char;
    stream = uqm_lua_checked!(uio_fopen(
        meleeDir,
        fileName,
        b"rb\0".as_ptr() as *const ::core::ffi::c_char,
    ), false);
    if stream.is_null() {
        return false;
    }
    if uqm_lua_checked!(MeleeTeam_deserialize(team, stream), false) == -1 as ::core::ffi::c_int {
        return false;
    }
    uqm_lua_checked!(uio_fclose(stream), false);
    return true;
}
unsafe extern "C" fn UnindexFleets(
    mut pMS: *mut MELEE_STATE,
    mut index: COUNT,
    mut count: COUNT,
) {
    (index as ::core::ffi::c_int + count as ::core::ffi::c_int
        <= (*pMS).load.numIndices as ::core::ffi::c_int
        || {
            __assert_fail(
                b"index + count <= pMS->load.numIndices\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/uqm/supermelee/loadmele.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                91 as ::core::ffi::c_int,
                b"UnindexFleets\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    (*pMS).load.numIndices = ((*pMS).load.numIndices as ::core::ffi::c_int
        - count as ::core::ffi::c_int) as COUNT;
    memmove(
        (*pMS).load.entryIndices.offset(index as isize) as *mut ::core::ffi::c_void,
        (*pMS)
            .load
            .entryIndices
            .offset((index as ::core::ffi::c_int + count as ::core::ffi::c_int) as isize)
            as *const ::core::ffi::c_void,
        (((*pMS).load.numIndices as ::core::ffi::c_int - index as ::core::ffi::c_int)
            as size_t)
            .wrapping_mul(::core::mem::size_of::<COUNT>()),
    );
}
unsafe extern "C" fn GetFleetByIndex(
    mut pMS: *mut MELEE_STATE,
    mut index: COUNT,
    mut result: *mut MeleeTeam,
) -> bool {
    let mut firstIndex: COUNT = 0;
    if (index as ::core::ffi::c_int) < (*pMS).load.preBuiltCount as ::core::ffi::c_int {
        MeleeTeam_copy(result, *(*pMS).load.preBuiltList.offset(index as isize));
        return true;
    }
    index = (index as ::core::ffi::c_int
        - (*pMS).load.preBuiltCount as ::core::ffi::c_int) as COUNT;
    firstIndex = index;
    while (index as ::core::ffi::c_int) < (*pMS).load.numIndices as ::core::ffi::c_int {
        let mut entry: DIRENTRY = SetAbsStringTableIndex(
            (*pMS).load.dirEntries,
            *(*pMS).load.entryIndices.offset(index as isize),
        );
        if uqm_lua_checked!(LoadTeamImage(entry, result), false) {
            break;
        }
        let mut fileName: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        fileName = GetStringAddress(entry) as *const ::core::ffi::c_char;
        log_add(
            log_Level::log_Warning,
            b"Warning: File '%s' is not a valid SuperMelee team.\0".as_ptr()
                as *const ::core::ffi::c_char,
            fileName,
        );
        index = index.wrapping_add(1);
    }
    if index as ::core::ffi::c_int != firstIndex as ::core::ffi::c_int {
        UnindexFleets(
            pMS,
            firstIndex,
            (index as ::core::ffi::c_int - firstIndex as ::core::ffi::c_int) as COUNT,
        );
    }
    return (index as ::core::ffi::c_int) < (*pMS).load.numIndices as ::core::ffi::c_int;
}
unsafe extern "C" fn GetFleetIndexByFileName(
    mut pMS: *mut MELEE_STATE,
    mut fileName: *const ::core::ffi::c_char,
) -> COUNT {
    let mut index: COUNT = 0;
    index = 0 as COUNT;
    while (index as ::core::ffi::c_int) < (*pMS).load.numIndices as ::core::ffi::c_int {
        let mut entry: DIRENTRY = SetAbsStringTableIndex(
            (*pMS).load.dirEntries,
            *(*pMS).load.entryIndices.offset(index as isize),
        );
        let mut entryName: *const ::core::ffi::c_char = GetStringAddress(entry)
            as *const ::core::ffi::c_char;
        if strcasecmp(entryName, fileName) == 0 as ::core::ffi::c_int {
            return ((*pMS).load.preBuiltCount as ::core::ffi::c_int
                + index as ::core::ffi::c_int) as COUNT;
        }
        index = index.wrapping_add(1);
    }
    return -1 as ::core::ffi::c_int as COUNT;
}
unsafe extern "C" fn DrawFileString(
    mut team: *const MeleeTeam,
    mut origin: *const POINT,
    mut drawShips: BOOLEAN,
    mut highlite: BOOLEAN,
) {
    if if resolutionFactor != 2 as ::core::ffi::c_uint {
        BOOLEAN::FALSE.0 as ::core::ffi::c_int
    } else {
        BOOLEAN::TRUE.0 as ::core::ffi::c_int
    } != 0
    {
        let mut r: RECT = RECT {
            corner: POINT { x: 0, y: 0 },
            extent: EXTENT { width: 0, height: 0 },
        };
        r.corner.x = (*origin).x;
        r.corner.y = ((*origin).y as ::core::ffi::c_int
            - ((10 as ::core::ffi::c_int) << resolutionFactor)
            + ((3 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        r.extent.width = ((244 as ::core::ffi::c_int) << resolutionFactor) as COORD;
        r.extent.height = ((10 as ::core::ffi::c_int) << resolutionFactor) as COORD;
        uqm_lua_checked!(QuickRepair(
            C2Rust_Unnamed_1::TEAM_PICKER.0 as ::core::ffi::c_int as COUNT,
            &raw mut r,
        ), ());
    }
    SetContextForeGroundColor(
        if highlite.0 != 0 {
            colorFromRgb15(0x17 as BYTE, 0x18 as BYTE, 0x1d as BYTE)
        } else {
            colorFromRgb15(0xf as BYTE, 0x10 as BYTE, 0x1b as BYTE)
        },
    );
    let mut Text: TEXT = TEXT {
        baseline: POINT { x: 0, y: 0 },
        pStr: ::core::ptr::null::<UNICODE>(),
        align: TEXT_ALIGN::ALIGN_LEFT,
        CharCount: 0,
    };
    Text.baseline = *origin;
    Text.align = TEXT_ALIGN::ALIGN_LEFT;
    Text.pStr = MeleeTeam_getTeamName(team) as *const UNICODE;
    Text.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    uqm_lua_checked!(font_DrawText(&raw mut Text), ());
    let mut Text_0: TEXT = TEXT {
        baseline: POINT { x: 0, y: 0 },
        pStr: ::core::ptr::null::<UNICODE>(),
        align: TEXT_ALIGN::ALIGN_LEFT,
        CharCount: 0,
    };
    let mut buf: [UNICODE; 60] = [0; 60];
    sprintf(
        &raw mut buf as *mut ::core::ffi::c_char,
        b"%u\0".as_ptr() as *const ::core::ffi::c_char,
        MeleeTeam_getValue(team) as ::core::ffi::c_int,
    );
    Text_0.baseline = *origin;
    Text_0.baseline.x = (Text_0.baseline.x as ::core::ffi::c_int
        + (if if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            6 as ::core::ffi::c_int
        } else {
            7 as ::core::ffi::c_int
        }
            * (((34 as ::core::ffi::c_int) << resolutionFactor)
                + ((1 as ::core::ffi::c_int) << resolutionFactor))
            - ((1 as ::core::ffi::c_int) << resolutionFactor))) as COORD;
    Text_0.align = TEXT_ALIGN::ALIGN_RIGHT;
    Text_0.pStr = &raw mut buf as *mut UNICODE;
    Text_0.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    uqm_lua_checked!(font_DrawText(&raw mut Text_0), ());
    if drawShips.0 != 0 {
        let mut s: STAMP = STAMP {
            origin: POINT { x: 0, y: 0 },
            frame: ::core::ptr::null_mut::<FRAME_DESC>(),
        };
        let mut slotI: FleetShipIndex = 0;
        s.origin.x = ((*origin).x as ::core::ffi::c_int
            + ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        s.origin.y = ((*origin).y as ::core::ffi::c_int
            + ((4 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        slotI = 0 as FleetShipIndex;
        while (slotI as ::core::ffi::c_int) < 14 as ::core::ffi::c_int {
            let mut StarShip: BYTE = 0;
            if slotI as ::core::ffi::c_int
                == 2 as ::core::ffi::c_int
                    * if if optWindowType == 1 as DWORD {
                        BOOLEAN::TRUE.0 as ::core::ffi::c_int
                    } else {
                        BOOLEAN::FALSE.0 as ::core::ffi::c_int
                    } != 0
                    {
                        6 as ::core::ffi::c_int
                    } else {
                        7 as ::core::ffi::c_int
                    }
            {
                break;
            }
            StarShip = (*team).ships[slotI as usize].0 as BYTE;
            if StarShip as ::core::ffi::c_int
                != MeleeShip::MELEE_NONE.0 as ::core::ffi::c_int
            {
                s.frame = GetShipIconsFromIndex(StarShip as ::core::ffi::c_uint);
                uqm_lua_checked!(DrawStamp(&raw mut s), ());
                s.origin.x = (s.origin.x as ::core::ffi::c_int
                    + ((17 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
            }
            slotI = slotI.wrapping_add(1);
        }
    }
}
unsafe extern "C" fn FillFileView(mut pMS: *mut MELEE_STATE) -> bool {
    let mut viewI: COUNT = 0;
    viewI = 0 as COUNT;
    while (viewI as ::core::ffi::c_int) < 5 as ::core::ffi::c_int {
        let mut success: bool = uqm_lua_checked!(GetFleetByIndex(
            pMS,
            ((*pMS).load.top as ::core::ffi::c_int + viewI as ::core::ffi::c_int)
                as COUNT,
            (*pMS).load.view[viewI as usize],
        ), false);
        if !success {
            break;
        }
        viewI = viewI.wrapping_add(1);
    }
    if viewI as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        return false;
    }
    (*pMS).load.bot = ((*pMS).load.top as ::core::ffi::c_int
        + viewI as ::core::ffi::c_int) as COUNT;
    return true;
}
unsafe extern "C" fn SelectFileString(mut pMS: *mut MELEE_STATE, mut hilite: bool) {
    let mut OldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    let mut origin: POINT = POINT { x: 0, y: 0 };
    let mut viewI: COUNT = 0;
    viewI = ((*pMS).load.cur as ::core::ffi::c_int
        - (*pMS).load.top as ::core::ffi::c_int) as COUNT;
    OldContext = SetContext(SpaceContext);
    SetContextFont(MicroFont);
    BatchGraphics();
    origin.x = (5 as ::core::ffi::c_int
        + if if optWindowType == 0 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            2 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        } << resolutionFactor) as COORD;
    origin.y = ((32 as ::core::ffi::c_int
        - if if optWindowType == 0 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        } << resolutionFactor)
        + viewI as ::core::ffi::c_int * ((32 as ::core::ffi::c_int) << resolutionFactor))
        as COORD;
    uqm_lua_checked!(DrawFileString(
        (*pMS).load.view[viewI as usize],
        &raw mut origin,
        BOOLEAN::FALSE,
        BOOLEAN(hilite as ::core::ffi::c_uint),
    ), ());
    UnbatchGraphics();
    SetContext(OldContext);
}
unsafe extern "C" fn DrawFileStrings(mut pMS: *mut MELEE_STATE) {
    let mut origin: POINT = POINT { x: 0, y: 0 };
    let mut OldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    origin.x = (5 as ::core::ffi::c_int
        + if if optWindowType == 0 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            2 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        } << resolutionFactor) as COORD;
    origin.y = (32 as ::core::ffi::c_int
        - if if optWindowType == 0 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        } << resolutionFactor) as COORD;
    OldContext = SetContext(SpaceContext);
    SetContextFont(MicroFont);
    BatchGraphics();
    uqm_lua_checked!(DrawMeleeIcon(
        C2Rust_Unnamed_1::TEAM_PICKER.0 as ::core::ffi::c_int as COUNT,
        BOOLEAN::FALSE,
    ), ());
    if uqm_lua_checked!(FillFileView(pMS), ()) {
        let mut i: COUNT = 0;
        i = (*pMS).load.top;
        while (i as ::core::ffi::c_int) < (*pMS).load.bot as ::core::ffi::c_int {
            uqm_lua_checked!(DrawFileString(
                (*pMS)
                    .load
                    .view[(i as ::core::ffi::c_int
                    - (*pMS).load.top as ::core::ffi::c_int) as usize],
                &raw mut origin,
                BOOLEAN::TRUE,
                BOOLEAN::FALSE,
            ), ());
            origin.y = (origin.y as ::core::ffi::c_int
                + ((32 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
            i = i.wrapping_add(1);
        }
    }
    UnbatchGraphics();
    SetContext(OldContext);
}
unsafe extern "C" fn RefocusView(mut pMS: *mut MELEE_STATE, mut index: COUNT) {
    ((index as ::core::ffi::c_int)
        < (*pMS).load.preBuiltCount as ::core::ffi::c_int
            + (*pMS).load.numIndices as ::core::ffi::c_int
        || {
            __assert_fail(
                b"index < pMS->load.preBuiltCount + pMS->load.numIndices\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/uqm/supermelee/loadmele.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                309 as ::core::ffi::c_int,
                b"RefocusView\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    (*pMS).load.cur = index;
    if index as ::core::ffi::c_int <= 5 as ::core::ffi::c_int / 2 as ::core::ffi::c_int {
        (*pMS).load.top = 0 as COUNT;
    } else {
        (*pMS).load.top = (index as ::core::ffi::c_int
            - 5 as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as COUNT;
    };
}
unsafe extern "C" fn flashSelectedTeam(mut pMS: *mut MELEE_STATE) {
    static mut NextTime: TimeCount = 0 as TimeCount;
    static mut hilite: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut Now: TimeCount = GetTimeCounter();
    if Now >= NextTime {
        let mut OldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
        NextTime = Now
            .wrapping_add(
                (840 as ::core::ffi::c_int / 9 as ::core::ffi::c_int) as TimeCount,
            );
        hilite ^= 1 as ::core::ffi::c_int;
        OldContext = SetContext(SpaceContext);
        uqm_lua_checked!(SelectFileString(pMS, hilite != 0), ());
        SetContext(OldContext);
    }
}
#[no_mangle]
pub unsafe extern "C" fn DoLoadTeam(mut pMS: *mut MELEE_STATE) -> BOOLEAN {
    let mut TimeIn: DWORD = GetTimeCounter();
    ::core::ptr::write_volatile(&raw mut GamePaused, BOOLEAN::FALSE);
    if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
        & C2Rust_Unnamed_0::CHECK_ABORT.0 as ::core::ffi::c_int != 0
    {
        return BOOLEAN::FALSE;
    }
    SetMenuSounds(
        (((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as MENU_SOUND_FLAGS
            as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as MENU_SOUND_FLAGS
                as ::core::ffi::c_int
            | (((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as MENU_SOUND_FLAGS
                as ::core::ffi::c_int
                | ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
                    as MENU_SOUND_FLAGS as ::core::ffi::c_int)) as MENU_SOUND_FLAGS,
        ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as MENU_SOUND_FLAGS,
    );
    if (*pMS).Initialized.0 == 0 {
        uqm_lua_checked!(DrawFileStrings(pMS), BOOLEAN(0));
        uqm_lua_checked!(SelectFileString(pMS, true), BOOLEAN(0));
        (*pMS).Initialized = BOOLEAN::TRUE;
        (*pMS).InputFunc = Some(
            DoLoadTeam as unsafe extern "C" fn(*mut MELEE_STATE) -> BOOLEAN,
        ) as Option<unsafe extern "C" fn(*mut melee_state) -> BOOLEAN>;
        return BOOLEAN::TRUE;
    }
    if PulsedInputState
        .menu[C2Rust_Unnamed::KEY_MENU_SELECT.0 as ::core::ffi::c_int as usize] != 0
        || PulsedInputState
            .menu[C2Rust_Unnamed::KEY_MENU_CANCEL.0 as ::core::ffi::c_int as usize] != 0
    {
        if PulsedInputState
            .menu[C2Rust_Unnamed::KEY_MENU_SELECT.0 as ::core::ffi::c_int as usize] != 0
        {
            uqm_lua_checked!(Melee_LocalChange_team(
                pMS,
                (*pMS).side as size_t,
                (*pMS)
                    .load
                    .view[((*pMS).load.cur as ::core::ffi::c_int
                    - (*pMS).load.top as ::core::ffi::c_int) as usize],
            ), BOOLEAN(0));
        }
        (*pMS).InputFunc = Some(
            DoMelee as unsafe extern "C" fn(*mut MELEE_STATE) -> BOOLEAN,
        ) as Option<unsafe extern "C" fn(*mut melee_state) -> BOOLEAN>;
        (*pMS).LastInputTime = GetTimeCounter();
        let mut r: RECT = RECT {
            corner: POINT { x: 0, y: 0 },
            extent: EXTENT { width: 0, height: 0 },
        };
        GetFrameRect(
            SetAbsFrameIndex(
                MeleeFrame,
                C2Rust_Unnamed_1::TEAM_PICKER.0 as ::core::ffi::c_int as COUNT,
            ),
            &raw mut r,
        );
        uqm_lua_checked!(RepairMeleeFrame(&raw mut r), BOOLEAN(0));
        return BOOLEAN::TRUE;
    }
    let mut newTop: COUNT = (*pMS).load.top;
    let mut newIndex: COUNT = (*pMS).load.cur;
    if PulsedInputState
        .menu[C2Rust_Unnamed::KEY_MENU_UP.0 as ::core::ffi::c_int as usize] != 0
    {
        if newIndex as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
            newIndex = newIndex.wrapping_sub(1);
            if (newIndex as ::core::ffi::c_int) < newTop as ::core::ffi::c_int {
                newTop = (if (newTop as ::core::ffi::c_int) < 5 as ::core::ffi::c_int {
                    0 as ::core::ffi::c_int
                } else {
                    newTop as ::core::ffi::c_int - 5 as ::core::ffi::c_int
                }) as COUNT;
            }
        }
    } else if PulsedInputState
        .menu[C2Rust_Unnamed::KEY_MENU_DOWN.0 as ::core::ffi::c_int as usize] != 0
    {
        let mut numEntries: COUNT = ((*pMS).load.numIndices as ::core::ffi::c_int
            + (*pMS).load.preBuiltCount as ::core::ffi::c_int) as COUNT;
        if (newIndex as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
            < numEntries as ::core::ffi::c_int
        {
            newIndex = newIndex.wrapping_add(1);
            if newIndex as ::core::ffi::c_int >= (*pMS).load.bot as ::core::ffi::c_int {
                newTop = (*pMS).load.bot;
            }
        }
    } else if PulsedInputState
        .menu[C2Rust_Unnamed::KEY_MENU_PAGE_UP.0 as ::core::ffi::c_int as usize] != 0
    {
        newIndex = (if (newIndex as ::core::ffi::c_int) < 5 as ::core::ffi::c_int {
            0 as ::core::ffi::c_int
        } else {
            newIndex as ::core::ffi::c_int - 5 as ::core::ffi::c_int
        }) as COUNT;
        newTop = (if (newTop as ::core::ffi::c_int) < 5 as ::core::ffi::c_int {
            0 as ::core::ffi::c_int
        } else {
            newTop as ::core::ffi::c_int - 5 as ::core::ffi::c_int
        }) as COUNT;
    } else if PulsedInputState
        .menu[C2Rust_Unnamed::KEY_MENU_PAGE_DOWN.0 as ::core::ffi::c_int as usize] != 0
    {
        let mut numEntries_0: COUNT = ((*pMS).load.numIndices as ::core::ffi::c_int
            + (*pMS).load.preBuiltCount as ::core::ffi::c_int) as COUNT;
        if (newIndex as ::core::ffi::c_int + 5 as ::core::ffi::c_int)
            < numEntries_0 as ::core::ffi::c_int
        {
            newIndex = (newIndex as ::core::ffi::c_int + 5 as ::core::ffi::c_int)
                as COUNT;
            newTop = (newTop as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as COUNT;
        } else {
            newIndex = (numEntries_0 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                as COUNT;
            if (newTop as ::core::ffi::c_int + 5 as ::core::ffi::c_int)
                < numEntries_0 as ::core::ffi::c_int
                && numEntries_0 as ::core::ffi::c_int > 5 as ::core::ffi::c_int
            {
                newTop = (numEntries_0 as ::core::ffi::c_int - 5 as ::core::ffi::c_int)
                    as COUNT;
            }
        }
    }
    if newIndex as ::core::ffi::c_int != (*pMS).load.cur as ::core::ffi::c_int {
        if newTop as ::core::ffi::c_int == (*pMS).load.top as ::core::ffi::c_int {
            uqm_lua_checked!(SelectFileString(pMS, false), BOOLEAN(0));
        } else {
            (*pMS).load.top = newTop;
            uqm_lua_checked!(DrawFileStrings(pMS), BOOLEAN(0));
        }
        (*pMS).load.cur = newIndex;
    }
    uqm_lua_checked!(flashSelectedTeam(pMS), BOOLEAN(0));
    uqm_lua_checked!(SleepThreadUntil(
        TimeIn
            .wrapping_add(
                (840 as ::core::ffi::c_int / 30 as ::core::ffi::c_int) as TimeCount,
            ),
    ), BOOLEAN(0));
    return BOOLEAN::TRUE;
}
unsafe extern "C" fn SelectTeamByFileName(
    mut pMS: *mut MELEE_STATE,
    mut fileName: *const ::core::ffi::c_char,
) {
    let mut index: COUNT = GetFleetIndexByFileName(pMS, fileName);
    if index as ::core::ffi::c_int
        == -1 as ::core::ffi::c_int as COUNT as ::core::ffi::c_int
    {
        return;
    }
    RefocusView(pMS, index);
}
#[no_mangle]
pub unsafe extern "C" fn LoadTeamList(mut pMS: *mut MELEE_STATE) {
    let mut i: COUNT = 0;
    DestroyStringTable(ReleaseStringTable((*pMS).load.dirEntries));
    (*pMS).load.dirEntries = CaptureStringTable(
        uqm_lua_checked!(LoadDirEntryTable(
            meleeDir,
            b"\0".as_ptr() as *const ::core::ffi::c_char,
            b".mle\0".as_ptr() as *const ::core::ffi::c_char,
            match_MatchType::match_MATCH_SUFFIX,
        ), ()),
    ) as DIRENTRY;
    if !(*pMS).load.entryIndices.is_null() {
        HFree((*pMS).load.entryIndices as *mut ::core::ffi::c_void);
    }
    (*pMS).load.numIndices = GetStringTableCount((*pMS).load.dirEntries);
    (*pMS).load.entryIndices = HMalloc(
        ((*pMS).load.numIndices as size_t).wrapping_mul(::core::mem::size_of::<COUNT>()),
    ) as *mut COUNT;
    i = 0 as COUNT;
    while (i as ::core::ffi::c_int) < (*pMS).load.numIndices as ::core::ffi::c_int {
        *(*pMS).load.entryIndices.offset(i as isize) = i;
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn DoSaveTeam(mut pMS: *mut MELEE_STATE) -> BOOLEAN {
    let mut MsgStamp: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    let mut file: [::core::ffi::c_char; 255] = [0; 255];
    let mut stream: *mut uio_Stream = ::core::ptr::null_mut::<uio_Stream>();
    let mut OldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    let mut saveOk: bool = false;
    snprintf(
        &raw mut file as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 255]>(),
        b"%s.mle\0".as_ptr() as *const ::core::ffi::c_char,
        MeleeSetup_getTeamName((*pMS).meleeSetup, (*pMS).side as size_t),
    );
    OldContext = SetContext(ScreenContext);
    uqm_lua_checked!(ConfirmSaveLoad(&raw mut MsgStamp), BOOLEAN(0));
    stream = uqm_lua_checked!(uio_fopen(
        meleeDir,
        &raw mut file as *mut ::core::ffi::c_char,
        b"wb\0".as_ptr() as *const ::core::ffi::c_char,
    ), BOOLEAN(0));
    if !stream.is_null() {
        saveOk = uqm_lua_checked!(MeleeTeam_serialize(
            (&raw mut (*(*pMS).meleeSetup).teams as *mut MeleeTeam)
                .offset((*pMS).side as isize),
            stream,
        ), BOOLEAN(0)) == 0 as ::core::ffi::c_int;
        uqm_lua_checked!(uio_fclose(stream), BOOLEAN(0));
        if !saveOk {
            uqm_lua_checked!(uio_unlink(meleeDir, &raw mut file as *mut ::core::ffi::c_char), BOOLEAN(0));
        }
    }
    (*pMS).load.top = 0 as COUNT;
    (*pMS).load.cur = 0 as COUNT;
    uqm_lua_checked!(DrawStamp(&raw mut MsgStamp), BOOLEAN(0));
    DestroyDrawable(ReleaseDrawable(MsgStamp.frame));
    SetContext(OldContext);
    if !saveOk {
        uqm_lua_checked!(SaveProblem(), BOOLEAN(0));
    }
    uqm_lua_checked!(LoadTeamList(pMS), BOOLEAN(0));
    SelectTeamByFileName(pMS, &raw mut file as *mut ::core::ffi::c_char);
    return BOOLEAN(!stream.is_null() as ::core::ffi::c_int as ::core::ffi::c_uint);
}
unsafe extern "C" fn InitPreBuilt(mut pMS: *mut MELEE_STATE) {
    let mut list: *mut *mut MeleeTeam = ::core::ptr::null_mut::<*mut MeleeTeam>();
    (*pMS).load.preBuiltList = HMalloc(
        (15 as size_t).wrapping_mul(::core::mem::size_of::<*mut MeleeTeam>()),
    ) as *mut *mut MeleeTeam;
    (*pMS).load.preBuiltCount = 15 as COUNT;
    let mut fleetI: size_t = 0;
    fleetI = 0 as size_t;
    while fleetI < (*pMS).load.preBuiltCount as size_t {
        *(*pMS).load.preBuiltList.offset(fleetI as isize) = MeleeTeam_new();
        fleetI = fleetI.wrapping_add(1);
    }
    list = (*pMS).load.preBuiltList;
    let mut i: FleetShipIndex = 0 as FleetShipIndex;
    MeleeTeam_setName(
        *list,
        GetStringAddress(
            SetAbsStringTableIndex(
                GameStrings,
                (C2Rust_Unnamed_2::MELEE_STRING_BASE.0 as ::core::ffi::c_int
                    + 4 as ::core::ffi::c_int) as COUNT,
            ),
        ),
    );
    let c2rust_fresh0 = i;
    i = i.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh0, MeleeShip::MELEE_ANDROSYNTH);
    let c2rust_fresh1 = i;
    i = i.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh1, MeleeShip::MELEE_CHMMR);
    let c2rust_fresh2 = i;
    i = i.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh2, MeleeShip::MELEE_DRUUGE);
    let c2rust_fresh3 = i;
    i = i.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh3, MeleeShip::MELEE_URQUAN);
    let c2rust_fresh4 = i;
    i = i.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh4, MeleeShip::MELEE_MELNORME);
    let c2rust_fresh5 = i;
    i = i.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh5, MeleeShip::MELEE_ORZ);
    let c2rust_fresh6 = i;
    i = i.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh6, MeleeShip::MELEE_SPATHI);
    let c2rust_fresh7 = i;
    i = i.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh7, MeleeShip::MELEE_SYREEN);
    let c2rust_fresh8 = i;
    i = i.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh8, MeleeShip::MELEE_UTWIG);
    list = list.offset(1);
    let mut i_0: FleetShipIndex = 0 as FleetShipIndex;
    MeleeTeam_setName(
        *list,
        GetStringAddress(
            SetAbsStringTableIndex(
                GameStrings,
                (C2Rust_Unnamed_2::MELEE_STRING_BASE.0 as ::core::ffi::c_int
                    + 5 as ::core::ffi::c_int) as COUNT,
            ),
        ),
    );
    let c2rust_fresh9 = i_0;
    i_0 = i_0.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh9, MeleeShip::MELEE_ARILOU);
    let c2rust_fresh10 = i_0;
    i_0 = i_0.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh10, MeleeShip::MELEE_CHENJESU);
    let c2rust_fresh11 = i_0;
    i_0 = i_0.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh11, MeleeShip::MELEE_EARTHLING);
    let c2rust_fresh12 = i_0;
    i_0 = i_0.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh12, MeleeShip::MELEE_KOHR_AH);
    let c2rust_fresh13 = i_0;
    i_0 = i_0.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh13, MeleeShip::MELEE_MYCON);
    let c2rust_fresh14 = i_0;
    i_0 = i_0.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh14, MeleeShip::MELEE_YEHAT);
    let c2rust_fresh15 = i_0;
    i_0 = i_0.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh15, MeleeShip::MELEE_PKUNK);
    let c2rust_fresh16 = i_0;
    i_0 = i_0.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh16, MeleeShip::MELEE_SUPOX);
    let c2rust_fresh17 = i_0;
    i_0 = i_0.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh17, MeleeShip::MELEE_THRADDASH);
    let c2rust_fresh18 = i_0;
    i_0 = i_0.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh18, MeleeShip::MELEE_ZOQFOTPIK);
    let c2rust_fresh19 = i_0;
    i_0 = i_0.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh19, MeleeShip::MELEE_SHOFIXTI);
    list = list.offset(1);
    let mut i_1: FleetShipIndex = 0 as FleetShipIndex;
    MeleeTeam_setName(
        *list,
        GetStringAddress(
            SetAbsStringTableIndex(
                GameStrings,
                (C2Rust_Unnamed_2::MELEE_STRING_BASE.0 as ::core::ffi::c_int
                    + 6 as ::core::ffi::c_int) as COUNT,
            ),
        ),
    );
    let c2rust_fresh20 = i_1;
    i_1 = i_1.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh20, MeleeShip::MELEE_ANDROSYNTH);
    let c2rust_fresh21 = i_1;
    i_1 = i_1.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh21, MeleeShip::MELEE_CHMMR);
    let c2rust_fresh22 = i_1;
    i_1 = i_1.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh22, MeleeShip::MELEE_DRUUGE);
    let c2rust_fresh23 = i_1;
    i_1 = i_1.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh23, MeleeShip::MELEE_MELNORME);
    let c2rust_fresh24 = i_1;
    i_1 = i_1.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh24, MeleeShip::MELEE_EARTHLING);
    let c2rust_fresh25 = i_1;
    i_1 = i_1.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh25, MeleeShip::MELEE_KOHR_AH);
    let c2rust_fresh26 = i_1;
    i_1 = i_1.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh26, MeleeShip::MELEE_SUPOX);
    let c2rust_fresh27 = i_1;
    i_1 = i_1.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh27, MeleeShip::MELEE_ORZ);
    let c2rust_fresh28 = i_1;
    i_1 = i_1.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh28, MeleeShip::MELEE_SPATHI);
    let c2rust_fresh29 = i_1;
    i_1 = i_1.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh29, MeleeShip::MELEE_ILWRATH);
    let c2rust_fresh30 = i_1;
    i_1 = i_1.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh30, MeleeShip::MELEE_VUX);
    list = list.offset(1);
    let mut i_2: FleetShipIndex = 0 as FleetShipIndex;
    MeleeTeam_setName(
        *list,
        GetStringAddress(
            SetAbsStringTableIndex(
                GameStrings,
                (C2Rust_Unnamed_2::MELEE_STRING_BASE.0 as ::core::ffi::c_int
                    + 7 as ::core::ffi::c_int) as COUNT,
            ),
        ),
    );
    let c2rust_fresh31 = i_2;
    i_2 = i_2.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh31, MeleeShip::MELEE_CHENJESU);
    let c2rust_fresh32 = i_2;
    i_2 = i_2.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh32, MeleeShip::MELEE_CHENJESU);
    let c2rust_fresh33 = i_2;
    i_2 = i_2.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh33, MeleeShip::MELEE_CHMMR);
    let c2rust_fresh34 = i_2;
    i_2 = i_2.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh34, MeleeShip::MELEE_CHMMR);
    let c2rust_fresh35 = i_2;
    i_2 = i_2.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh35, MeleeShip::MELEE_KOHR_AH);
    let c2rust_fresh36 = i_2;
    i_2 = i_2.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh36, MeleeShip::MELEE_KOHR_AH);
    let c2rust_fresh37 = i_2;
    i_2 = i_2.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh37, MeleeShip::MELEE_URQUAN);
    let c2rust_fresh38 = i_2;
    i_2 = i_2.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh38, MeleeShip::MELEE_URQUAN);
    let c2rust_fresh39 = i_2;
    i_2 = i_2.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh39, MeleeShip::MELEE_UTWIG);
    let c2rust_fresh40 = i_2;
    i_2 = i_2.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh40, MeleeShip::MELEE_UTWIG);
    list = list.offset(1);
    let mut i_3: FleetShipIndex = 0 as FleetShipIndex;
    MeleeTeam_setName(
        *list,
        GetStringAddress(
            SetAbsStringTableIndex(
                GameStrings,
                (C2Rust_Unnamed_2::MELEE_STRING_BASE.0 as ::core::ffi::c_int
                    + 8 as ::core::ffi::c_int) as COUNT,
            ),
        ),
    );
    let c2rust_fresh41 = i_3;
    i_3 = i_3.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh41, MeleeShip::MELEE_URQUAN);
    let c2rust_fresh42 = i_3;
    i_3 = i_3.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh42, MeleeShip::MELEE_CHENJESU);
    let c2rust_fresh43 = i_3;
    i_3 = i_3.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh43, MeleeShip::MELEE_MYCON);
    let c2rust_fresh44 = i_3;
    i_3 = i_3.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh44, MeleeShip::MELEE_SYREEN);
    let c2rust_fresh45 = i_3;
    i_3 = i_3.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh45, MeleeShip::MELEE_ZOQFOTPIK);
    let c2rust_fresh46 = i_3;
    i_3 = i_3.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh46, MeleeShip::MELEE_SHOFIXTI);
    let c2rust_fresh47 = i_3;
    i_3 = i_3.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh47, MeleeShip::MELEE_EARTHLING);
    let c2rust_fresh48 = i_3;
    i_3 = i_3.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh48, MeleeShip::MELEE_KOHR_AH);
    let c2rust_fresh49 = i_3;
    i_3 = i_3.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh49, MeleeShip::MELEE_MELNORME);
    let c2rust_fresh50 = i_3;
    i_3 = i_3.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh50, MeleeShip::MELEE_DRUUGE);
    let c2rust_fresh51 = i_3;
    i_3 = i_3.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh51, MeleeShip::MELEE_PKUNK);
    let c2rust_fresh52 = i_3;
    i_3 = i_3.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh52, MeleeShip::MELEE_ORZ);
    list = list.offset(1);
    let mut i_4: FleetShipIndex = 0 as FleetShipIndex;
    MeleeTeam_setName(
        *list,
        b"Ford's Fighters\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let c2rust_fresh53 = i_4;
    i_4 = i_4.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh53, MeleeShip::MELEE_CHMMR);
    let c2rust_fresh54 = i_4;
    i_4 = i_4.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh54, MeleeShip::MELEE_ZOQFOTPIK);
    let c2rust_fresh55 = i_4;
    i_4 = i_4.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh55, MeleeShip::MELEE_MELNORME);
    let c2rust_fresh56 = i_4;
    i_4 = i_4.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh56, MeleeShip::MELEE_SUPOX);
    let c2rust_fresh57 = i_4;
    i_4 = i_4.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh57, MeleeShip::MELEE_UTWIG);
    let c2rust_fresh58 = i_4;
    i_4 = i_4.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh58, MeleeShip::MELEE_UMGAH);
    list = list.offset(1);
    let mut i_5: FleetShipIndex = 0 as FleetShipIndex;
    MeleeTeam_setName(
        *list,
        b"Leyland's Lashers\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let c2rust_fresh59 = i_5;
    i_5 = i_5.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh59, MeleeShip::MELEE_ANDROSYNTH);
    let c2rust_fresh60 = i_5;
    i_5 = i_5.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh60, MeleeShip::MELEE_EARTHLING);
    let c2rust_fresh61 = i_5;
    i_5 = i_5.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh61, MeleeShip::MELEE_MYCON);
    let c2rust_fresh62 = i_5;
    i_5 = i_5.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh62, MeleeShip::MELEE_ORZ);
    let c2rust_fresh63 = i_5;
    i_5 = i_5.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh63, MeleeShip::MELEE_URQUAN);
    list = list.offset(1);
    let mut i_6: FleetShipIndex = 0 as FleetShipIndex;
    MeleeTeam_setName(
        *list,
        b"The Gregorizers 200\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let c2rust_fresh64 = i_6;
    i_6 = i_6.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh64, MeleeShip::MELEE_ANDROSYNTH);
    let c2rust_fresh65 = i_6;
    i_6 = i_6.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh65, MeleeShip::MELEE_CHMMR);
    let c2rust_fresh66 = i_6;
    i_6 = i_6.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh66, MeleeShip::MELEE_DRUUGE);
    let c2rust_fresh67 = i_6;
    i_6 = i_6.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh67, MeleeShip::MELEE_MELNORME);
    let c2rust_fresh68 = i_6;
    i_6 = i_6.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh68, MeleeShip::MELEE_EARTHLING);
    let c2rust_fresh69 = i_6;
    i_6 = i_6.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh69, MeleeShip::MELEE_KOHR_AH);
    let c2rust_fresh70 = i_6;
    i_6 = i_6.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh70, MeleeShip::MELEE_SUPOX);
    let c2rust_fresh71 = i_6;
    i_6 = i_6.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh71, MeleeShip::MELEE_ORZ);
    let c2rust_fresh72 = i_6;
    i_6 = i_6.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh72, MeleeShip::MELEE_PKUNK);
    let c2rust_fresh73 = i_6;
    i_6 = i_6.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh73, MeleeShip::MELEE_SPATHI);
    list = list.offset(1);
    let mut i_7: FleetShipIndex = 0 as FleetShipIndex;
    MeleeTeam_setName(
        *list,
        b"300 point Armada!\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let c2rust_fresh74 = i_7;
    i_7 = i_7.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh74, MeleeShip::MELEE_ANDROSYNTH);
    let c2rust_fresh75 = i_7;
    i_7 = i_7.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh75, MeleeShip::MELEE_CHMMR);
    let c2rust_fresh76 = i_7;
    i_7 = i_7.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh76, MeleeShip::MELEE_CHENJESU);
    let c2rust_fresh77 = i_7;
    i_7 = i_7.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh77, MeleeShip::MELEE_DRUUGE);
    let c2rust_fresh78 = i_7;
    i_7 = i_7.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh78, MeleeShip::MELEE_EARTHLING);
    let c2rust_fresh79 = i_7;
    i_7 = i_7.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh79, MeleeShip::MELEE_KOHR_AH);
    let c2rust_fresh80 = i_7;
    i_7 = i_7.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh80, MeleeShip::MELEE_MELNORME);
    let c2rust_fresh81 = i_7;
    i_7 = i_7.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh81, MeleeShip::MELEE_MYCON);
    let c2rust_fresh82 = i_7;
    i_7 = i_7.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh82, MeleeShip::MELEE_ORZ);
    let c2rust_fresh83 = i_7;
    i_7 = i_7.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh83, MeleeShip::MELEE_PKUNK);
    let c2rust_fresh84 = i_7;
    i_7 = i_7.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh84, MeleeShip::MELEE_SPATHI);
    let c2rust_fresh85 = i_7;
    i_7 = i_7.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh85, MeleeShip::MELEE_SUPOX);
    let c2rust_fresh86 = i_7;
    i_7 = i_7.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh86, MeleeShip::MELEE_URQUAN);
    let c2rust_fresh87 = i_7;
    i_7 = i_7.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh87, MeleeShip::MELEE_YEHAT);
    list = list.offset(1);
    let mut i_8: FleetShipIndex = 0 as FleetShipIndex;
    MeleeTeam_setName(
        *list,
        b"Little Dudes with Attitudes\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let c2rust_fresh88 = i_8;
    i_8 = i_8.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh88, MeleeShip::MELEE_UMGAH);
    let c2rust_fresh89 = i_8;
    i_8 = i_8.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh89, MeleeShip::MELEE_THRADDASH);
    let c2rust_fresh90 = i_8;
    i_8 = i_8.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh90, MeleeShip::MELEE_SHOFIXTI);
    let c2rust_fresh91 = i_8;
    i_8 = i_8.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh91, MeleeShip::MELEE_EARTHLING);
    let c2rust_fresh92 = i_8;
    i_8 = i_8.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh92, MeleeShip::MELEE_VUX);
    let c2rust_fresh93 = i_8;
    i_8 = i_8.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh93, MeleeShip::MELEE_ZOQFOTPIK);
    list = list.offset(1);
    let mut i_9: FleetShipIndex = 0 as FleetShipIndex;
    MeleeTeam_setName(
        *list,
        b"New Alliance Ships\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let c2rust_fresh94 = i_9;
    i_9 = i_9.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh94, MeleeShip::MELEE_ARILOU);
    let c2rust_fresh95 = i_9;
    i_9 = i_9.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh95, MeleeShip::MELEE_CHMMR);
    let c2rust_fresh96 = i_9;
    i_9 = i_9.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh96, MeleeShip::MELEE_EARTHLING);
    let c2rust_fresh97 = i_9;
    i_9 = i_9.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh97, MeleeShip::MELEE_ORZ);
    let c2rust_fresh98 = i_9;
    i_9 = i_9.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh98, MeleeShip::MELEE_PKUNK);
    let c2rust_fresh99 = i_9;
    i_9 = i_9.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh99, MeleeShip::MELEE_SHOFIXTI);
    let c2rust_fresh100 = i_9;
    i_9 = i_9.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh100, MeleeShip::MELEE_SUPOX);
    let c2rust_fresh101 = i_9;
    i_9 = i_9.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh101, MeleeShip::MELEE_SYREEN);
    let c2rust_fresh102 = i_9;
    i_9 = i_9.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh102, MeleeShip::MELEE_UTWIG);
    let c2rust_fresh103 = i_9;
    i_9 = i_9.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh103, MeleeShip::MELEE_ZOQFOTPIK);
    let c2rust_fresh104 = i_9;
    i_9 = i_9.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh104, MeleeShip::MELEE_YEHAT);
    let c2rust_fresh105 = i_9;
    i_9 = i_9.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh105, MeleeShip::MELEE_DRUUGE);
    let c2rust_fresh106 = i_9;
    i_9 = i_9.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh106, MeleeShip::MELEE_THRADDASH);
    let c2rust_fresh107 = i_9;
    i_9 = i_9.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh107, MeleeShip::MELEE_SPATHI);
    list = list.offset(1);
    let mut i_10: FleetShipIndex = 0 as FleetShipIndex;
    MeleeTeam_setName(
        *list,
        b"Old Alliance Ships\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let c2rust_fresh108 = i_10;
    i_10 = i_10.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh108, MeleeShip::MELEE_ARILOU);
    let c2rust_fresh109 = i_10;
    i_10 = i_10.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh109, MeleeShip::MELEE_CHENJESU);
    let c2rust_fresh110 = i_10;
    i_10 = i_10.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh110, MeleeShip::MELEE_EARTHLING);
    let c2rust_fresh111 = i_10;
    i_10 = i_10.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh111, MeleeShip::MELEE_MMRNMHRM);
    let c2rust_fresh112 = i_10;
    i_10 = i_10.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh112, MeleeShip::MELEE_SHOFIXTI);
    let c2rust_fresh113 = i_10;
    i_10 = i_10.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh113, MeleeShip::MELEE_SYREEN);
    let c2rust_fresh114 = i_10;
    i_10 = i_10.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh114, MeleeShip::MELEE_YEHAT);
    list = list.offset(1);
    let mut i_11: FleetShipIndex = 0 as FleetShipIndex;
    MeleeTeam_setName(
        *list,
        b"Old Hierarchy Ships\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let c2rust_fresh115 = i_11;
    i_11 = i_11.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh115, MeleeShip::MELEE_ANDROSYNTH);
    let c2rust_fresh116 = i_11;
    i_11 = i_11.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh116, MeleeShip::MELEE_ILWRATH);
    let c2rust_fresh117 = i_11;
    i_11 = i_11.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh117, MeleeShip::MELEE_MYCON);
    let c2rust_fresh118 = i_11;
    i_11 = i_11.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh118, MeleeShip::MELEE_SPATHI);
    let c2rust_fresh119 = i_11;
    i_11 = i_11.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh119, MeleeShip::MELEE_UMGAH);
    let c2rust_fresh120 = i_11;
    i_11 = i_11.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh120, MeleeShip::MELEE_URQUAN);
    let c2rust_fresh121 = i_11;
    i_11 = i_11.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh121, MeleeShip::MELEE_VUX);
    list = list.offset(1);
    let mut i_12: FleetShipIndex = 0 as FleetShipIndex;
    MeleeTeam_setName(*list, b"Star Control 1\0".as_ptr() as *const ::core::ffi::c_char);
    let c2rust_fresh122 = i_12;
    i_12 = i_12.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh122, MeleeShip::MELEE_ANDROSYNTH);
    let c2rust_fresh123 = i_12;
    i_12 = i_12.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh123, MeleeShip::MELEE_ARILOU);
    let c2rust_fresh124 = i_12;
    i_12 = i_12.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh124, MeleeShip::MELEE_CHENJESU);
    let c2rust_fresh125 = i_12;
    i_12 = i_12.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh125, MeleeShip::MELEE_EARTHLING);
    let c2rust_fresh126 = i_12;
    i_12 = i_12.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh126, MeleeShip::MELEE_ILWRATH);
    let c2rust_fresh127 = i_12;
    i_12 = i_12.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh127, MeleeShip::MELEE_MMRNMHRM);
    let c2rust_fresh128 = i_12;
    i_12 = i_12.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh128, MeleeShip::MELEE_MYCON);
    let c2rust_fresh129 = i_12;
    i_12 = i_12.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh129, MeleeShip::MELEE_SHOFIXTI);
    let c2rust_fresh130 = i_12;
    i_12 = i_12.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh130, MeleeShip::MELEE_SPATHI);
    let c2rust_fresh131 = i_12;
    i_12 = i_12.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh131, MeleeShip::MELEE_SYREEN);
    let c2rust_fresh132 = i_12;
    i_12 = i_12.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh132, MeleeShip::MELEE_UMGAH);
    let c2rust_fresh133 = i_12;
    i_12 = i_12.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh133, MeleeShip::MELEE_URQUAN);
    let c2rust_fresh134 = i_12;
    i_12 = i_12.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh134, MeleeShip::MELEE_VUX);
    let c2rust_fresh135 = i_12;
    i_12 = i_12.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh135, MeleeShip::MELEE_YEHAT);
    list = list.offset(1);
    let mut i_13: FleetShipIndex = 0 as FleetShipIndex;
    MeleeTeam_setName(*list, b"Star Control 2\0".as_ptr() as *const ::core::ffi::c_char);
    let c2rust_fresh136 = i_13;
    i_13 = i_13.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh136, MeleeShip::MELEE_CHMMR);
    let c2rust_fresh137 = i_13;
    i_13 = i_13.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh137, MeleeShip::MELEE_DRUUGE);
    let c2rust_fresh138 = i_13;
    i_13 = i_13.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh138, MeleeShip::MELEE_KOHR_AH);
    let c2rust_fresh139 = i_13;
    i_13 = i_13.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh139, MeleeShip::MELEE_MELNORME);
    let c2rust_fresh140 = i_13;
    i_13 = i_13.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh140, MeleeShip::MELEE_ORZ);
    let c2rust_fresh141 = i_13;
    i_13 = i_13.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh141, MeleeShip::MELEE_PKUNK);
    let c2rust_fresh142 = i_13;
    i_13 = i_13.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh142, MeleeShip::MELEE_SLYLANDRO);
    let c2rust_fresh143 = i_13;
    i_13 = i_13.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh143, MeleeShip::MELEE_SUPOX);
    let c2rust_fresh144 = i_13;
    i_13 = i_13.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh144, MeleeShip::MELEE_THRADDASH);
    let c2rust_fresh145 = i_13;
    i_13 = i_13.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh145, MeleeShip::MELEE_UTWIG);
    let c2rust_fresh146 = i_13;
    i_13 = i_13.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh146, MeleeShip::MELEE_ZOQFOTPIK);
    let c2rust_fresh147 = i_13;
    i_13 = i_13.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh147, MeleeShip::MELEE_ZOQFOTPIK);
    let c2rust_fresh148 = i_13;
    i_13 = i_13.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh148, MeleeShip::MELEE_ZOQFOTPIK);
    let c2rust_fresh149 = i_13;
    i_13 = i_13.wrapping_add(1);
    MeleeTeam_setShip(*list, c2rust_fresh149, MeleeShip::MELEE_ZOQFOTPIK);
    list = list.offset(1);
    (list
        == (*pMS)
            .load
            .preBuiltList
            .offset((*pMS).load.preBuiltCount as ::core::ffi::c_int as isize)
        || {
            __assert_fail(
                b"list == pMS->load.preBuiltList + pMS->load.preBuiltCount\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/uqm/supermelee/loadmele.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                795 as ::core::ffi::c_int,
                b"InitPreBuilt\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
}
unsafe extern "C" fn UninitPreBuilt(mut pMS: *mut MELEE_STATE) {
    let mut fleetI: size_t = 0;
    fleetI = 0 as size_t;
    while fleetI < (*pMS).load.preBuiltCount as size_t {
        MeleeTeam_delete(*(*pMS).load.preBuiltList.offset(fleetI as isize));
        fleetI = fleetI.wrapping_add(1);
    }
    HFree((*pMS).load.preBuiltList as *mut ::core::ffi::c_void);
    (*pMS).load.preBuiltCount = 0 as COUNT;
}
unsafe extern "C" fn InitLoadView(mut pMS: *mut MELEE_STATE) {
    let mut viewI: size_t = 0;
    let mut view: *mut *mut MeleeTeam = &raw mut (*pMS).load.view as *mut *mut MeleeTeam;
    viewI = 0 as size_t;
    while viewI < 5 as size_t {
        *view.offset(viewI as isize) = MeleeTeam_new();
        viewI = viewI.wrapping_add(1);
    }
}
unsafe extern "C" fn UninitLoadView(mut pMS: *mut MELEE_STATE) {
    let mut viewI: size_t = 0;
    let mut view: *mut *mut MeleeTeam = &raw mut (*pMS).load.view as *mut *mut MeleeTeam;
    viewI = 0 as size_t;
    while viewI < 5 as size_t {
        MeleeTeam_delete(*view.offset(viewI as isize));
        viewI = viewI.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn InitMeleeLoadState(mut pMS: *mut MELEE_STATE) {
    (*pMS).load.entryIndices = ::core::ptr::null_mut::<COUNT>();
    InitPreBuilt(pMS);
    InitLoadView(pMS);
}
#[no_mangle]
pub unsafe extern "C" fn UninitMeleeLoadState(mut pMS: *mut MELEE_STATE) {
    UninitLoadView(pMS);
    UninitPreBuilt(pMS);
    if !(*pMS).load.entryIndices.is_null() {
        HFree((*pMS).load.entryIndices as *mut ::core::ffi::c_void);
    }
}
