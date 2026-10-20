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

// Rust translation of src/uqm/starbase.c.
extern "C" {
    pub type string_table_entry;
    pub type context_desc;
    pub type frame_desc;
    pub type font_desc;
    pub type drawable_desc;
    pub type tfb_soundsample;
    pub type FlashContext;
    fn cos(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn sin(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn SetContext(Context: CONTEXT) -> CONTEXT;
    fn SetContextForeGroundColor(Color: Color) -> Color;
    fn SetContextBackGroundColor(Color: Color) -> Color;
    fn DrawStamp(pStamp: *mut STAMP);
    fn DrawFilledStamp(pStamp: *mut STAMP);
    fn DrawFilledRectangle(pRect: *mut RECT);
    fn font_DrawText(pText: *mut TEXT);
    fn BatchGraphics();
    fn UnbatchGraphics();
    fn ClearDrawable();
    fn DestroyDrawable(Drawable: DRAWABLE) -> BOOLEAN;
    fn LoadGraphicInstance(res: RESOURCE) -> *mut ::core::ffi::c_void;
    fn SetContextFont(Font: FONT) -> FONT;
    fn TextRect(pText: *mut TEXT, pRect: *mut RECT, pdelta: *mut BYTE) -> BOOLEAN;
    fn GetFrameIndex(Frame: FRAME) -> COUNT;
    fn SetAbsFrameIndex(Frame: FRAME, FrameIndex: COUNT) -> FRAME;
    fn SetRelFrameIndex(Frame: FRAME, FrameOffs: SIZE) -> FRAME;
    fn IncFrameIndex(Frame: FRAME) -> FRAME;
    fn CaptureDrawable(Drawable: DRAWABLE) -> FRAME;
    fn ReleaseDrawable(Frame: FRAME) -> DRAWABLE;
    fn FadeScreen(fadeType: ScreenFadeType, TimeInterval: SIZE) -> DWORD;
    fn __assert_fail(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    ) -> !;
    fn SetAbsStringTableIndex(String: STRING, StringTableIndex: COUNT) -> STRING;
    fn GetStringAddress(String: STRING) -> STRINGPTR;
    fn GetTimeCounter() -> TimeCount;
    fn memset(
        _: *mut ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    static mut PulsedInputState: CONTROLLER_INPUT_STATE;
    fn FlushInput();
    fn DoInput(pInputState: *mut ::core::ffi::c_void, resetInput: BOOLEAN);
    static mut resolutionFactor: ::core::ffi::c_uint;
    static mut optOrbitingPlanets: OPT_ENABLABLE;
    static mut optScrTrans: ::core::ffi::c_int;
    static mut optWindowType: DWORD;
    static mut CanvasWidth: ::core::ffi::c_int;
    static mut CanvasHeight: ::core::ffi::c_int;
    fn LoadMusicInstance(res: RESOURCE) -> MUSIC_REF;
    fn DestroyMusic(MusicRef: MUSIC_REF) -> BOOLEAN;
    fn SetMusicPosition();
    fn SleepThread(timePeriod: TimePeriod);
    fn SleepThreadUntil(wakeTime: TimeCount);
    fn daysElapsed() -> ::core::ffi::c_float;
    fn MoveGameClockDays(days: COUNT);
    fn SetFlashRect(pRect: *const RECT, purple: BOOLEAN);
    fn SetStatusMessageMode(_: StatMsgMode) -> StatMsgMode;
    static mut hdFuelFrame: FRAME;
    fn setGameStateUint(name: *const ::core::ffi::c_char, val: DWORD);
    fn getGameStateUint(name: *const ::core::ffi::c_char) -> DWORD;
    static mut GlobData: GLOBDATA;
    static mut CurStarDescPtr: *mut STAR_DESC;
    fn CloneShipFragment(
        shipIndex: RACE_ID,
        pDstQueue: *mut QUEUE,
        crew_level: COUNT,
    ) -> HSHIPFRAG;
    static mut NextActivity: ACTIVITY;
    static mut GameStrings: STRING;
    fn InitCommunication(which_comm: CONVERSATION) -> COUNT;
    fn SetCommIntroMode(_: CommIntroMode, howLong: TimeCount);
    fn DoOutfit(pMS: *mut MENU_STATE) -> BOOLEAN;
    fn DoShipyard(pMS: *mut MENU_STATE) -> BOOLEAN;
    fn StopMusic();
    fn PlayMusicResume(hMusic: MUSIC_REF, Volume: BYTE);
    static mut ScreenContext: CONTEXT;
    static mut StarConFont: FONT;
    static mut LastActivity: ACTIVITY;
    fn SetMenuSounds(sound_0: MENU_SOUND_FLAGS, sound_1: MENU_SOUND_FLAGS);
    fn SetTransitionSource(pRect: *const RECT);
    fn ScreenTransition(transition: ::core::ffi::c_int, pRect: *const RECT);
    fn DrawBorderPadding(videoWidth: DWORD);
}
pub type uint8 = ::core::ffi::c_uchar;
pub type sint8 = ::core::ffi::c_schar;
pub type uint16 = ::core::ffi::c_ushort;
pub type sint16 = ::core::ffi::c_short;
pub type uint32 = ::core::ffi::c_uint;
pub type sint32 = ::core::ffi::c_int;
pub type BYTE = uint8;
pub type SBYTE = sint8;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Color {
    pub r: BYTE,
    pub g: BYTE,
    pub b: BYTE,
    pub a: BYTE,
}
pub type RESOURCE = *const ::core::ffi::c_char;
pub type STRING_TABLE_ENTRY_DESC = string_table_entry;
pub type STRING = *mut STRING_TABLE_ENTRY_DESC;
pub type STRINGPTR = *mut ::core::ffi::c_char;
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
pub struct ScreenFadeType(pub ::core::ffi::c_uint);
impl ScreenFadeType {
    pub const FadeAllToWhite: Self = Self(250);
    pub const FadeSomeToWhite: Self = Self(251);
    pub const FadeAllToBlack: Self = Self(252);
    pub const FadeAllToColor: Self = Self(253);
    pub const FadeSomeToBlack: Self = Self(254);
    pub const FadeSomeToColor: Self = Self(255);
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct OPT_ENABLABLE(pub ::core::ffi::c_uint);
impl OPT_ENABLABLE {
    pub const OPTVAL_DISABLED: Self = Self(0);
    pub const OPTVAL_ENABLED: Self = Self(1);
}
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
pub type MUSIC_REF = *mut *mut TFB_SoundSample;
pub type TFB_SoundSample = tfb_soundsample;
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
pub type HSHIPFRAG = HLINK;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SHIP_FRAGMENT {
    pub pred: HLINK,
    pub succ: HLINK,
    pub SpeciesID: SPECIES_ID,
    pub captains_name_index: BYTE,
    pub race_id: BYTE,
    pub index: BYTE,
    pub crew_level: COUNT,
    pub max_crew: COUNT,
    pub energy_level: BYTE,
    pub max_energy: BYTE,
    pub race_strings: STRING,
    pub icons: FRAME,
    pub melee_icon: FRAME,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct RACE_ID(pub ::core::ffi::c_uint);
impl RACE_ID {
    pub const ARILOU_SHIP: Self = Self(0);
    pub const CHMMR_SHIP: Self = Self(1);
    pub const HUMAN_SHIP: Self = Self(2);
    pub const ORZ_SHIP: Self = Self(3);
    pub const PKUNK_SHIP: Self = Self(4);
    pub const SHOFIXTI_SHIP: Self = Self(5);
    pub const SPATHI_SHIP: Self = Self(6);
    pub const SUPOX_SHIP: Self = Self(7);
    pub const THRADDASH_SHIP: Self = Self(8);
    pub const UTWIG_SHIP: Self = Self(9);
    pub const VUX_SHIP: Self = Self(10);
    pub const YEHAT_SHIP: Self = Self(11);
    pub const MELNORME_SHIP: Self = Self(12);
    pub const DRUUGE_SHIP: Self = Self(13);
    pub const ILWRATH_SHIP: Self = Self(14);
    pub const MYCON_SHIP: Self = Self(15);
    pub const SLYLANDRO_SHIP: Self = Self(16);
    pub const UMGAH_SHIP: Self = Self(17);
    pub const URQUAN_SHIP: Self = Self(18);
    pub const ZOQFOTPIK_SHIP: Self = Self(19);
    pub const SYREEN_SHIP: Self = Self(20);
    pub const BLACK_URQUAN_SHIP: Self = Self(21);
    pub const ANDROSYNTH_SHIP: Self = Self(22);
    pub const CHENJESU_SHIP: Self = Self(23);
    pub const MMRNMHRM_SHIP: Self = Self(24);
    pub const NUM_BUILDABLE_SHIPS: Self = Self(25);
    pub const YEHAT_REBEL_SHIP: Self = Self(25);
    pub const URQUAN_DRONE_SHIP: Self = Self(26);
    pub const SAMATRA_SHIP: Self = Self(26);
    pub const NUM_AVAILABLE_RACES: Self = Self(27);
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_0 {
    pub const PLANET_LANDER: Self = Self(0);
    pub const FUSION_THRUSTER: Self = Self(1);
    pub const TURNING_JETS: Self = Self(2);
    pub const CREW_POD: Self = Self(3);
    pub const STORAGE_BAY: Self = Self(4);
    pub const FUEL_TANK: Self = Self(5);
    pub const HIGHEFF_FUELSYS: Self = Self(6);
    pub const DYNAMO_UNIT: Self = Self(7);
    pub const SHIVA_FURNACE: Self = Self(8);
    pub const GUN_WEAPON: Self = Self(9);
    pub const BLASTER_WEAPON: Self = Self(10);
    pub const CANNON_WEAPON: Self = Self(11);
    pub const TRACKING_SYSTEM: Self = Self(12);
    pub const ANTIMISSILE_DEFENSE: Self = Self(13);
    pub const NUM_PURCHASE_MODULES: Self = Self(14);
    pub const BOMB_MODULE_0: Self = Self(14);
    pub const BOMB_MODULE_1: Self = Self(15);
    pub const BOMB_MODULE_2: Self = Self(16);
    pub const BOMB_MODULE_3: Self = Self(17);
    pub const BOMB_MODULE_4: Self = Self(18);
    pub const BOMB_MODULE_5: Self = Self(19);
    pub const NUM_MODULES: Self = Self(20);
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
pub struct StatMsgMode(pub ::core::ffi::c_uint);
impl StatMsgMode {
    pub const SMM_UNDEFINED: Self = Self(0);
    pub const SMM_DATE: Self = Self(1);
    pub const SMM_RES_UNITS: Self = Self(2);
    pub const SMM_CREDITS: Self = Self(3);
    pub const SMM_WARNING: Self = Self(4);
    pub const SMM_ALERT: Self = Self(5);
    pub const SMM_DEFAULT: Self = Self(1);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct CONVERSATION(pub ::core::ffi::c_uint);
impl CONVERSATION {
    pub const ARILOU_CONVERSATION: Self = Self(0);
    pub const CHMMR_CONVERSATION: Self = Self(1);
    pub const COMMANDER_CONVERSATION: Self = Self(2);
    pub const ORZ_CONVERSATION: Self = Self(3);
    pub const PKUNK_CONVERSATION: Self = Self(4);
    pub const SHOFIXTI_CONVERSATION: Self = Self(5);
    pub const SPATHI_CONVERSATION: Self = Self(6);
    pub const SUPOX_CONVERSATION: Self = Self(7);
    pub const THRADD_CONVERSATION: Self = Self(8);
    pub const UTWIG_CONVERSATION: Self = Self(9);
    pub const VUX_CONVERSATION: Self = Self(10);
    pub const YEHAT_CONVERSATION: Self = Self(11);
    pub const MELNORME_CONVERSATION: Self = Self(12);
    pub const DRUUGE_CONVERSATION: Self = Self(13);
    pub const ILWRATH_CONVERSATION: Self = Self(14);
    pub const MYCON_CONVERSATION: Self = Self(15);
    pub const SLYLANDRO_CONVERSATION: Self = Self(16);
    pub const UMGAH_CONVERSATION: Self = Self(17);
    pub const URQUAN_CONVERSATION: Self = Self(18);
    pub const ZOQFOTPIK_CONVERSATION: Self = Self(19);
    pub const SYREEN_CONVERSATION: Self = Self(20);
    pub const BLACKURQ_CONVERSATION: Self = Self(21);
    pub const TALKING_PET_CONVERSATION: Self = Self(22);
    pub const SLYLANDRO_HOME_CONVERSATION: Self = Self(23);
    pub const URQUAN_DRONE_CONVERSATION: Self = Self(24);
    pub const YEHAT_REBEL_CONVERSATION: Self = Self(25);
    pub const INVALID_CONVERSATION: Self = Self(26);
    pub const NUM_CONVERSATIONS: Self = Self(27);
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
pub struct star_desc {
    pub star_pt: POINT,
    pub Type: BYTE,
    pub Index: BYTE,
    pub Prefix: BYTE,
    pub Postfix: BYTE,
}
pub type STAR_DESC = star_desc;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct CommIntroMode(pub ::core::ffi::c_uint);
impl CommIntroMode {
    pub const CIM_CROSSFADE_SPACE: Self = Self(0);
    pub const CIM_CROSSFADE_WINDOW: Self = Self(1);
    pub const CIM_CROSSFADE_SCREEN: Self = Self(2);
    pub const CIM_FADE_IN_SCREEN: Self = Self(3);
    pub const CIM_DEFAULT: Self = Self(0);
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
pub struct C2Rust_Unnamed_3(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_3 {
    pub const TALK_COMMANDER: Self = Self(0);
    pub const OUTFIT_STARSHIP: Self = Self(1);
    pub const SHIPYARD: Self = Self(2);
    pub const DEPART_BASE: Self = Self(3);
}
pub type STARBASE_STATE = BYTE;
pub type MENU_SOUND_FLAGS = UWORD;
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
                    b"src/uqm/displist.h\0"
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
                    b"src/uqm/displist.h\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    88 as ::core::ffi::c_int,
                    b"UnlockLink\0".as_ptr() as *const ::core::ffi::c_char,
                );
                0 as ::core::ffi::c_int != 0
            }) as ::core::ffi::c_int;
    }
}
#[inline]
unsafe extern "C" fn LockShipFrag(
    mut pq: *const QUEUE,
    mut h: HSHIPFRAG,
) -> *mut SHIP_FRAGMENT {
    ((*pq).object_size as usize == ::core::mem::size_of::<SHIP_FRAGMENT>()
        || {
            __assert_fail(
                b"GetLinkSize (pq) == sizeof (SHIP_FRAGMENT)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/uqm/races.h\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                340 as ::core::ffi::c_int,
                b"LockShipFrag\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    return LockLink(pq, h) as *mut SHIP_FRAGMENT;
}
unsafe extern "C" fn DrawBaseStateStrings(
    mut OldState: STARBASE_STATE,
    mut NewState: STARBASE_STATE,
) {
    let mut t: TEXT = TEXT {
        baseline: POINT { x: 0, y: 0 },
        pStr: ::core::ptr::null::<UNICODE>(),
        align: TEXT_ALIGN::ALIGN_LEFT,
        CharCount: 0,
    };
    let mut text_base_y: COUNT = ((106 as ::core::ffi::c_int + 28 as ::core::ffi::c_int)
        << resolutionFactor) as COUNT;
    let mut text_spacing_y: COUNT = ((23 as ::core::ffi::c_int - 4 as ::core::ffi::c_int)
        << resolutionFactor) as COUNT;
    SetContext(ScreenContext);
    SetContextFont(StarConFont);
    SetContextForeGroundColor(colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE));
    t.baseline.x = (((73 as ::core::ffi::c_int - 4 as ::core::ffi::c_int)
        << resolutionFactor)
        + (if if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            16 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        } << resolutionFactor)) as COORD;
    t.align = TEXT_ALIGN::ALIGN_CENTER;
    if OldState as ::core::ffi::c_int
        == !(0 as ::core::ffi::c_int) as STARBASE_STATE as ::core::ffi::c_int
    {
        t.baseline.y = (text_base_y as ::core::ffi::c_int
            + (if if optWindowType == 1 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                16 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } << resolutionFactor)) as COORD;
        OldState = C2Rust_Unnamed_3::TALK_COMMANDER.0 as ::core::ffi::c_int
            as STARBASE_STATE;
        while (OldState as ::core::ffi::c_int)
            < C2Rust_Unnamed_3::DEPART_BASE.0 as ::core::ffi::c_int
        {
            if OldState as ::core::ffi::c_int != NewState as ::core::ffi::c_int {
                t.pStr = GetStringAddress(
                    SetAbsStringTableIndex(
                        GameStrings,
                        (C2Rust_Unnamed_2::STARBASE_STRING_BASE.0 as ::core::ffi::c_int
                            + 1 as ::core::ffi::c_int + OldState as ::core::ffi::c_int)
                            as COUNT,
                    ),
                );
                t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
                uqm_lua_checked!(font_DrawText(&raw mut t), ());
            }
            t.baseline.y = (t.baseline.y as ::core::ffi::c_int
                + text_spacing_y as ::core::ffi::c_int) as COORD;
            OldState = OldState.wrapping_add(1);
        }
    }
    t.baseline.y = (text_base_y as ::core::ffi::c_int
        + (if if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            16 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        } << resolutionFactor)
        + text_spacing_y as ::core::ffi::c_int * OldState as ::core::ffi::c_int)
        as COORD;
    t.pStr = GetStringAddress(
        SetAbsStringTableIndex(
            GameStrings,
            (C2Rust_Unnamed_2::STARBASE_STRING_BASE.0 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int + OldState as ::core::ffi::c_int) as COUNT,
        ),
    );
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    uqm_lua_checked!(font_DrawText(&raw mut t), ());
    SetContextForeGroundColor(colorFromRgb15(0x1f as BYTE, 0x1f as BYTE, 0xa as BYTE));
    t.baseline.y = (text_base_y as ::core::ffi::c_int
        + (if if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            16 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        } << resolutionFactor)
        + text_spacing_y as ::core::ffi::c_int * NewState as ::core::ffi::c_int)
        as COORD;
    t.pStr = GetStringAddress(
        SetAbsStringTableIndex(
            GameStrings,
            (C2Rust_Unnamed_2::STARBASE_STRING_BASE.0 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int + NewState as ::core::ffi::c_int) as COUNT,
        ),
    );
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    uqm_lua_checked!(font_DrawText(&raw mut t), ());
}
#[no_mangle]
pub unsafe extern "C" fn DrawShipPiece(
    mut ModuleFrame: FRAME,
    mut which_piece: COUNT,
    mut which_slot: COUNT,
    mut DrawBluePrint: BOOLEAN,
) {
    let mut OldColor: Color = colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE);
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut Side: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    let mut Top: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    let mut RepairSlot: SBYTE = 0;
    RepairSlot = 0 as SBYTE;
    match which_piece as ::core::ffi::c_int {
        1 | 20 => {
            Side.origin.x = (((31 as ::core::ffi::c_int) << resolutionFactor)
                - if if optWindowType == 1 as DWORD {
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
                    } << resolutionFactor)
                        - ((3 as ::core::ffi::c_int) << resolutionFactor)
                } else {
                    0 as ::core::ffi::c_int
                }) as COORD;
            Side.origin.y = ((56 as ::core::ffi::c_int) << resolutionFactor) as COORD;
            Top.origin.x = (((33 as ::core::ffi::c_int) << resolutionFactor)
                - if if optWindowType == 1 as DWORD {
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
                    } << resolutionFactor)
                        - ((3 as ::core::ffi::c_int) << resolutionFactor)
                } else {
                    0 as ::core::ffi::c_int
                }) as COORD;
            Top.origin.y = (((86 as ::core::ffi::c_int) << resolutionFactor)
                - (if if optWindowType == 0 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    21 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                } << resolutionFactor)) as COORD;
        }
        2 | 21 => {
            Side.origin.x = (((71 as ::core::ffi::c_int) << resolutionFactor)
                - if if optWindowType == 1 as DWORD {
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
                    } << resolutionFactor)
                        - ((3 as ::core::ffi::c_int) << resolutionFactor)
                } else {
                    0 as ::core::ffi::c_int
                }) as COORD;
            Side.origin.y = ((48 as ::core::ffi::c_int) << resolutionFactor) as COORD;
            Top.origin.x = (((70 as ::core::ffi::c_int) << resolutionFactor)
                - if if optWindowType == 1 as DWORD {
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
                    } << resolutionFactor)
                        - ((3 as ::core::ffi::c_int) << resolutionFactor)
                } else {
                    0 as ::core::ffi::c_int
                }) as COORD;
            Top.origin.y = (((94 as ::core::ffi::c_int) << resolutionFactor)
                - (if if optWindowType == 0 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    21 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                } << resolutionFactor)) as COORD;
        }
        _ => {
            if (which_piece as ::core::ffi::c_int)
                < C2Rust_Unnamed_0::NUM_MODULES.0 as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int
            {
                RepairSlot = 1 as SBYTE;
                if (which_piece as ::core::ffi::c_int)
                    < C2Rust_Unnamed_0::NUM_MODULES.0 as ::core::ffi::c_int
                    && (which_slot as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                        || (GlobData
                            .SIS_state
                            .ModuleSlots[(which_slot as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int) as usize] as ::core::ffi::c_int)
                            < C2Rust_Unnamed_0::NUM_MODULES.0 as ::core::ffi::c_int)
                {
                    RepairSlot += 1;
                }
            } else if DrawBluePrint.0 == 0 {
                if which_slot as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                    || which_slot as ::core::ffi::c_int
                        >= 16 as ::core::ffi::c_int - 3 as ::core::ffi::c_int
                {
                    which_piece = which_piece.wrapping_add(1);
                }
                if (which_slot as ::core::ffi::c_int)
                    < 16 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                    && (GlobData
                        .SIS_state
                        .ModuleSlots[(which_slot as ::core::ffi::c_int
                        + 1 as ::core::ffi::c_int) as usize] as ::core::ffi::c_int)
                        < C2Rust_Unnamed_0::NUM_MODULES.0 as ::core::ffi::c_int
                {
                    RepairSlot = -1 as SBYTE;
                    if which_piece as ::core::ffi::c_int
                        == C2Rust_Unnamed_0::NUM_MODULES.0 as ::core::ffi::c_int
                            + 3 as ::core::ffi::c_int
                        || which_slot as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                            == 16 as ::core::ffi::c_int - 3 as ::core::ffi::c_int
                    {
                        RepairSlot -= 1;
                    }
                }
            }
            Side.origin.x = (((17 as ::core::ffi::c_int) << resolutionFactor)
                - if if optWindowType == 1 as DWORD {
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
                    } << resolutionFactor)
                        - ((3 as ::core::ffi::c_int) << resolutionFactor)
                } else {
                    0 as ::core::ffi::c_int
                }) as COORD;
            Side.origin.y = ((14 as ::core::ffi::c_int) << resolutionFactor) as COORD;
            Top.origin.x = (((17 as ::core::ffi::c_int) << resolutionFactor)
                - if if optWindowType == 1 as DWORD {
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
                    } << resolutionFactor)
                        - ((3 as ::core::ffi::c_int) << resolutionFactor)
                } else {
                    0 as ::core::ffi::c_int
                }) as COORD;
            Top.origin.y = (((117 as ::core::ffi::c_int) << resolutionFactor)
                - (if if optWindowType == 0 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    21 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                } << resolutionFactor)) as COORD;
        }
    }
    Side.origin.x = (Side.origin.x as ::core::ffi::c_int
        + which_slot as ::core::ffi::c_int
            * ((12 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    Side.frame = ::core::ptr::null_mut::<FRAME_DESC>();
    if (RepairSlot as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
        Side.frame = SetAbsFrameIndex(
            ModuleFrame,
            (C2Rust_Unnamed_0::NUM_MODULES.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int
                + (6 as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
                + (C2Rust_Unnamed_0::NUM_MODULES.0 as ::core::ffi::c_int
                    + 6 as ::core::ffi::c_int)
                - (RepairSlot as ::core::ffi::c_int + 1 as ::core::ffi::c_int)) as COUNT,
        );
        uqm_lua_checked!(DrawStamp(&raw mut Side), ());
    } else if RepairSlot != 0 {
        r.corner = Side.origin;
        r.extent.width = ((12 as ::core::ffi::c_int) << resolutionFactor) as COORD;
        r.extent.height = ((1 as ::core::ffi::c_int) << resolutionFactor) as COORD;
        OldColor = SetContextForeGroundColor(
            colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE),
        );
        if if resolutionFactor != 2 as ::core::ffi::c_uint {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } == 0
        {
            uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
        }
        r.corner.y = (r.corner.y as ::core::ffi::c_int
            + ((23 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) << resolutionFactor))
            as COORD;
        if if resolutionFactor != 2 as ::core::ffi::c_uint {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } == 0
        {
            uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
        }
        r.extent.width = ((1 as ::core::ffi::c_int) << resolutionFactor) as COORD;
        r.extent.height = ((8 as ::core::ffi::c_int) << resolutionFactor) as COORD;
        if RepairSlot as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            r.corner = Side.origin;
            if if resolutionFactor != 2 as ::core::ffi::c_uint {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } == 0
            {
                uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
            }
            r.corner.y = (r.corner.y as ::core::ffi::c_int
                + ((15 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
            if if resolutionFactor != 2 as ::core::ffi::c_uint {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } == 0
            {
                uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
            }
        }
        if (which_slot as ::core::ffi::c_int)
            < 16 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
        {
            r.corner = Side.origin;
            r.corner.x = (r.corner.x as ::core::ffi::c_int
                + ((12 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
            if if resolutionFactor != 2 as ::core::ffi::c_uint {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } == 0
            {
                uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
            }
            r.corner.y = (r.corner.y as ::core::ffi::c_int
                + ((15 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
            if if resolutionFactor != 2 as ::core::ffi::c_uint {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } == 0
            {
                uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
            }
        }
    }
    if DrawBluePrint.0 != 0 {
        if RepairSlot != 0 {
            SetContextForeGroundColor(OldColor);
        }
        Side.frame = SetAbsFrameIndex(
            ModuleFrame,
            (which_piece as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as COUNT,
        );
        uqm_lua_checked!(DrawFilledStamp(&raw mut Side), ());
    } else {
        Top.origin.x = (Top.origin.x as ::core::ffi::c_int
            + which_slot as ::core::ffi::c_int
                * ((12 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        if (RepairSlot as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
            Top.frame = SetRelFrameIndex(
                Side.frame,
                -(C2Rust_Unnamed_0::NUM_MODULES.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as SIZE,
            );
            uqm_lua_checked!(DrawStamp(&raw mut Top), ());
        } else if RepairSlot != 0 {
            r.corner = Top.origin;
            r.extent.width = ((12 as ::core::ffi::c_int) << resolutionFactor) as COORD;
            r.extent.height = ((1 as ::core::ffi::c_int) << resolutionFactor) as COORD;
            if if resolutionFactor != 2 as ::core::ffi::c_uint {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } == 0
            {
                uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
            }
            r.corner.y = (r.corner.y as ::core::ffi::c_int
                + ((32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                    << resolutionFactor)) as COORD;
            if if resolutionFactor != 2 as ::core::ffi::c_uint {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } == 0
            {
                uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
            }
            r.extent.width = ((1 as ::core::ffi::c_int) << resolutionFactor) as COORD;
            r.extent.height = ((12 as ::core::ffi::c_int) << resolutionFactor) as COORD;
            if RepairSlot as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
                r.corner = Top.origin;
                if if resolutionFactor != 2 as ::core::ffi::c_uint {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } == 0
                {
                    uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
                }
                r.corner.y = (r.corner.y as ::core::ffi::c_int
                    + ((20 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
                if if resolutionFactor != 2 as ::core::ffi::c_uint {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } == 0
                {
                    uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
                }
            }
            RepairSlot = ((which_slot as ::core::ffi::c_int)
                < 16 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_int as SBYTE;
            if RepairSlot != 0 {
                r.corner = Top.origin;
                r.corner.x = (r.corner.x as ::core::ffi::c_int
                    + ((12 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
                if if resolutionFactor != 2 as ::core::ffi::c_uint {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } == 0
                {
                    uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
                }
                r.corner.y = (r.corner.y as ::core::ffi::c_int
                    + ((20 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
                if if resolutionFactor != 2 as ::core::ffi::c_uint {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } == 0
                {
                    uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
                }
            }
        }
        Top.frame = SetAbsFrameIndex(ModuleFrame, which_piece);
        uqm_lua_checked!(DrawStamp(&raw mut Top), ());
        Side.frame = SetRelFrameIndex(
            Top.frame,
            (C2Rust_Unnamed_0::NUM_MODULES.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as SIZE,
        );
        uqm_lua_checked!(DrawStamp(&raw mut Side), ());
        if which_slot as ::core::ffi::c_int == 1 as ::core::ffi::c_int
            && which_piece as ::core::ffi::c_int
                == C2Rust_Unnamed_0::NUM_MODULES.0 as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int
        {
            let mut s: STAMP = STAMP {
                origin: POINT { x: 0, y: 0 },
                frame: ::core::ptr::null_mut::<FRAME_DESC>(),
            };
            s.origin = Top.origin;
            s.origin.x = (s.origin.x as ::core::ffi::c_int
                - ((12 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
            s.frame = SetAbsFrameIndex(
                ModuleFrame,
                (C2Rust_Unnamed_0::NUM_MODULES.0 as ::core::ffi::c_int
                    + 5 as ::core::ffi::c_int) as COUNT,
            );
            uqm_lua_checked!(DrawStamp(&raw mut s), ());
            s.origin = Side.origin;
            s.origin.x = (s.origin.x as ::core::ffi::c_int
                - ((12 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
            s.frame = SetRelFrameIndex(
                s.frame,
                (C2Rust_Unnamed_0::NUM_MODULES.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as SIZE,
            );
            uqm_lua_checked!(DrawStamp(&raw mut s), ());
        }
        if RepairSlot != 0 {
            Top.origin.x = (Top.origin.x as ::core::ffi::c_int
                + ((12 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
            Side.origin.x = (Side.origin.x as ::core::ffi::c_int
                + ((12 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
            which_slot = which_slot.wrapping_add(1);
            which_piece = GlobData.SIS_state.ModuleSlots[which_slot as usize] as COUNT;
            if which_piece as ::core::ffi::c_int
                == C2Rust_Unnamed_0::NUM_MODULES.0 as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int
                && which_slot as ::core::ffi::c_int
                    >= 16 as ::core::ffi::c_int - 3 as ::core::ffi::c_int
            {
                which_piece = which_piece.wrapping_add(1);
            }
            Top.frame = SetAbsFrameIndex(ModuleFrame, which_piece);
            uqm_lua_checked!(DrawStamp(&raw mut Top), ());
            Side.frame = SetRelFrameIndex(
                Top.frame,
                (C2Rust_Unnamed_0::NUM_MODULES.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as SIZE,
            );
            uqm_lua_checked!(DrawStamp(&raw mut Side), ());
        }
    };
}
unsafe extern "C" fn rotateStarbase(mut pMS: *mut MENU_STATE, mut AniFrame: FRAME) {
    static mut NextTime: TimeCount = 0 as TimeCount;
    let mut Now: TimeCount = GetTimeCounter();
    if !AniFrame.is_null() {
        (*pMS).flash_frame0 = AniFrame;
        (*pMS).flash_rect0.corner.x = (if if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            16 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        } << resolutionFactor) as COORD;
        (*pMS).flash_rect0.corner.y = (if if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            16 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        } << resolutionFactor) as COORD;
    }
    if Now >= NextTime || !AniFrame.is_null() {
        let mut s: STAMP = STAMP {
            origin: POINT { x: 0, y: 0 },
            frame: ::core::ptr::null_mut::<FRAME_DESC>(),
        };
        NextTime = Now
            .wrapping_add(
                (840 as ::core::ffi::c_int / 20 as ::core::ffi::c_int) as TimeCount,
            );
        s.origin = (*pMS).flash_rect0.corner;
        s.frame = (*pMS).flash_frame0;
        uqm_lua_checked!(DrawStamp(&raw mut s), ());
        s.frame = IncFrameIndex(s.frame);
        if GetFrameIndex(s.frame) as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            s.frame = IncFrameIndex(s.frame);
        }
        (*pMS).flash_frame0 = s.frame;
    }
}
#[no_mangle]
pub unsafe extern "C" fn DoStarBase(mut pMS: *mut MENU_STATE) -> BOOLEAN {
    's_313: {
        if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
            & (C2Rust_Unnamed_1::CHECK_ABORT.0 as ::core::ffi::c_int
                | C2Rust_Unnamed_1::CHECK_LOAD.0 as ::core::ffi::c_int) != 0
        {
            (*pMS).CurState = C2Rust_Unnamed_3::DEPART_BASE.0 as ::core::ffi::c_int
                as BYTE;
        } else {
            SetMenuSounds(
                (((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int)
                    as MENU_SOUND_FLAGS as ::core::ffi::c_int
                    | ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
                        as MENU_SOUND_FLAGS as ::core::ffi::c_int
                    | ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                        as MENU_SOUND_FLAGS as ::core::ffi::c_int
                    | ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                        as MENU_SOUND_FLAGS as ::core::ffi::c_int) as MENU_SOUND_FLAGS,
                ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                    as MENU_SOUND_FLAGS,
            );
            if (*pMS).Initialized == 0 {
                LastActivity = (LastActivity as ::core::ffi::c_int
                    & !(C2Rust_Unnamed_1::CHECK_LOAD.0 as ::core::ffi::c_int))
                    as ACTIVITY;
                (*pMS).InputFunc = Some(
                    DoStarBase as unsafe extern "C" fn(*mut MENU_STATE) -> BOOLEAN,
                ) as Option<unsafe extern "C" fn(*mut menu_state) -> BOOLEAN>;
                uqm_lua_checked!(SetFlashRect(::core::ptr::null::<RECT>(), BOOLEAN::FALSE), BOOLEAN(0));
                if !(*pMS).hMusic.is_null() {
                    uqm_lua_checked!(StopMusic(), BOOLEAN(0));
                    uqm_lua_checked!(DestroyMusic((*pMS).hMusic), BOOLEAN(0));
                    (*pMS).hMusic = ::core::ptr::null_mut::<*mut TFB_SoundSample>();
                }
                (*pMS).Initialized = BOOLEAN::TRUE.0 as ::core::ffi::c_int as SIZE;
                (*pMS).CurFrame = CaptureDrawable(
                    uqm_lua_checked!(LoadGraphicInstance(b"graphics.starbase\0".as_ptr() as RESOURCE), BOOLEAN(0))
                        as DRAWABLE,
                );
                (*pMS).hMusic = uqm_lua_checked!(LoadMusicInstance(
                    b"music.starbase\0".as_ptr() as RESOURCE,
                ), BOOLEAN(0));
                SetContext(ScreenContext);
                SetTransitionSource(::core::ptr::null::<RECT>());
                BatchGraphics();
                SetContextBackGroundColor(
                    colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE),
                );
                uqm_lua_checked!(ClearDrawable(), BOOLEAN(0));
                uqm_lua_checked!(rotateStarbase(pMS, (*pMS).CurFrame), BOOLEAN(0));
                uqm_lua_checked!(DrawBaseStateStrings(
                    !(0 as ::core::ffi::c_int) as STARBASE_STATE,
                    (*pMS).CurState,
                ), BOOLEAN(0));
                uqm_lua_checked!(ScreenTransition(optScrTrans, ::core::ptr::null::<RECT>()), BOOLEAN(0));
                uqm_lua_checked!(PlayMusicResume((*pMS).hMusic, 160 as BYTE), BOOLEAN(0));
                uqm_lua_checked!(DrawBorderPadding(0 as DWORD), BOOLEAN(0));
                UnbatchGraphics();
                break 's_313;
            } else if PulsedInputState
                .menu[C2Rust_Unnamed::KEY_MENU_SELECT.0 as ::core::ffi::c_int as usize]
                == 0
            {
                let mut NewState: STARBASE_STATE = 0;
                NewState = (*pMS).CurState as STARBASE_STATE;
                if PulsedInputState
                    .menu[C2Rust_Unnamed::KEY_MENU_LEFT.0 as ::core::ffi::c_int as usize]
                    != 0
                    || PulsedInputState
                        .menu[C2Rust_Unnamed::KEY_MENU_UP.0 as ::core::ffi::c_int
                        as usize] != 0
                {
                    let c2rust_fresh0 = NewState;
                    NewState = NewState.wrapping_sub(1);
                    if c2rust_fresh0 as ::core::ffi::c_int
                        == C2Rust_Unnamed_3::TALK_COMMANDER.0 as ::core::ffi::c_int
                    {
                        NewState = C2Rust_Unnamed_3::DEPART_BASE.0 as ::core::ffi::c_int
                            as STARBASE_STATE;
                    }
                } else if PulsedInputState
                    .menu[C2Rust_Unnamed::KEY_MENU_RIGHT.0 as ::core::ffi::c_int
                    as usize] != 0
                    || PulsedInputState
                        .menu[C2Rust_Unnamed::KEY_MENU_DOWN.0 as ::core::ffi::c_int
                        as usize] != 0
                {
                    let c2rust_fresh1 = NewState;
                    NewState = NewState.wrapping_add(1);
                    if c2rust_fresh1 as ::core::ffi::c_int
                        == C2Rust_Unnamed_3::DEPART_BASE.0 as ::core::ffi::c_int
                    {
                        NewState = C2Rust_Unnamed_3::TALK_COMMANDER.0
                            as ::core::ffi::c_int as STARBASE_STATE;
                    }
                }
                BatchGraphics();
                SetContext(ScreenContext);
                if NewState as ::core::ffi::c_int
                    != (*pMS).CurState as ::core::ffi::c_int
                {
                    uqm_lua_checked!(DrawBaseStateStrings((*pMS).CurState, NewState), BOOLEAN(0));
                    (*pMS).CurState = NewState as BYTE;
                }
                uqm_lua_checked!(rotateStarbase(pMS, ::core::ptr::null_mut::<FRAME_DESC>()), BOOLEAN(0));
                uqm_lua_checked!(DrawBorderPadding(0 as DWORD), BOOLEAN(0));
                UnbatchGraphics();
                uqm_lua_checked!(SleepThread(
                    (840 as ::core::ffi::c_int / 30 as ::core::ffi::c_int) as TimePeriod,
                ), BOOLEAN(0));
                break 's_313;
            }
        }
        DestroyDrawable(ReleaseDrawable((*pMS).CurFrame));
        (*pMS).CurFrame = ::core::ptr::null_mut::<FRAME_DESC>();
        uqm_lua_checked!(SetMusicPosition(), BOOLEAN(0));
        uqm_lua_checked!(StopMusic(), BOOLEAN(0));
        if !(*pMS).hMusic.is_null() {
            uqm_lua_checked!(DestroyMusic((*pMS).hMusic), BOOLEAN(0));
            (*pMS).hMusic = ::core::ptr::null_mut::<*mut TFB_SoundSample>();
        }
        if (*pMS).CurState as ::core::ffi::c_int
            == C2Rust_Unnamed_3::DEPART_BASE.0 as ::core::ffi::c_int
        {
            if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
                & (C2Rust_Unnamed_1::CHECK_ABORT.0 as ::core::ffi::c_int
                    | C2Rust_Unnamed_1::CHECK_LOAD.0 as ::core::ffi::c_int) == 0
            {
                uqm_lua_checked!(setGameStateUint(
                    b"STARBASE_VISITED\0".as_ptr() as *const ::core::ffi::c_char,
                    0 as DWORD,
                ), BOOLEAN(0));
            }
            return BOOLEAN::FALSE;
        }
        (*pMS).Initialized = BOOLEAN::FALSE.0 as ::core::ffi::c_int as SIZE;
        if (*pMS).CurState as ::core::ffi::c_int
            == C2Rust_Unnamed_3::TALK_COMMANDER.0 as ::core::ffi::c_int
        {
            FlushInput();
            uqm_lua_checked!(InitCommunication(CONVERSATION::COMMANDER_CONVERSATION), BOOLEAN(0));
            uqm_lua_checked!(setGameStateUint(
                b"GLOBAL_FLAGS_AND_DATA\0".as_ptr() as *const ::core::ffi::c_char,
                !(0 as ::core::ffi::c_int) as BYTE as DWORD,
            ), BOOLEAN(0));
        } else {
            let mut OldState: BYTE = 0;
            if if resolutionFactor != 2 as ::core::ffi::c_uint {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } != 0 && hdFuelFrame.is_null()
            {
                hdFuelFrame = CaptureDrawable(
                    uqm_lua_checked!(LoadGraphicInstance(b"graphics.fuel\0".as_ptr() as RESOURCE), BOOLEAN(0))
                        as DRAWABLE,
                );
            }
            OldState = (*pMS).CurState;
            match OldState as ::core::ffi::c_int {
                1 => {
                    (*pMS).InputFunc = Some(
                        DoOutfit as unsafe extern "C" fn(*mut MENU_STATE) -> BOOLEAN,
                    ) as Option<unsafe extern "C" fn(*mut menu_state) -> BOOLEAN>;
                }
                2 => {
                    (*pMS).InputFunc = Some(
                        DoShipyard as unsafe extern "C" fn(*mut MENU_STATE) -> BOOLEAN,
                    ) as Option<unsafe extern "C" fn(*mut menu_state) -> BOOLEAN>;
                }
                _ => {}
            }
            SetMenuSounds(
                (((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int)
                    as MENU_SOUND_FLAGS as ::core::ffi::c_int
                    | ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
                        as MENU_SOUND_FLAGS as ::core::ffi::c_int
                    | ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                        as MENU_SOUND_FLAGS as ::core::ffi::c_int
                    | ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                        as MENU_SOUND_FLAGS as ::core::ffi::c_int) as MENU_SOUND_FLAGS,
                ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                    as MENU_SOUND_FLAGS,
            );
            uqm_lua_checked!(DoInput(pMS as *mut ::core::ffi::c_void, BOOLEAN::TRUE), BOOLEAN(0));
            if if resolutionFactor != 2 as ::core::ffi::c_uint {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } != 0
            {
                DestroyDrawable(ReleaseDrawable(hdFuelFrame));
                hdFuelFrame = ::core::ptr::null_mut::<FRAME_DESC>();
            }
            (*pMS).Initialized = BOOLEAN::FALSE.0 as ::core::ffi::c_int as SIZE;
            (*pMS).CurState = OldState;
            (*pMS).InputFunc = Some(
                DoStarBase as unsafe extern "C" fn(*mut MENU_STATE) -> BOOLEAN,
            ) as Option<unsafe extern "C" fn(*mut menu_state) -> BOOLEAN>;
        }
    }
    return BOOLEAN::TRUE;
}
unsafe extern "C" fn DoTimePassage() {
    uqm_lua_checked!(SleepThreadUntil(
        FadeScreen(
            ScreenFadeType::FadeAllToBlack,
            (840 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as SIZE,
        ),
    ), ());
    uqm_lua_checked!(MoveGameClockDays(14 as COUNT), ());
    if optOrbitingPlanets.0 != 0 {
        let mut newAngle: ::core::ffi::c_double = 0.;
        let mut starbase_coords: POINT = POINT { x: 0, y: 0 };
        newAngle = (10 as ::core::ffi::c_int as ::core::ffi::c_double
            + daysElapsed() as ::core::ffi::c_double
                * (((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                    as ::core::ffi::c_double / 11.46f64)) * 3.14159265358979323846f64
            / 32 as ::core::ffi::c_int as ::core::ffi::c_double
            - 3.14159265358979323846f64
                / 2 as ::core::ffi::c_int as ::core::ffi::c_double;
        starbase_coords.x = (cos(newAngle)
            * ((35 as ::core::ffi::c_int) << resolutionFactor) as ::core::ffi::c_double)
            as COORD;
        starbase_coords.y = (sin(newAngle)
            * ((35 as ::core::ffi::c_int) << resolutionFactor) as ::core::ffi::c_double)
            as COORD;
        GlobData.Game_state.ShipStamp.origin.x = (((CanvasWidth
            - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
            - (((13 as ::core::ffi::c_int) << resolutionFactor)
                + (if if optWindowType == 1 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    1 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                } << resolutionFactor)) >> resolutionFactor >> 1 as ::core::ffi::c_int)
            << resolutionFactor) + starbase_coords.x as ::core::ffi::c_int) as COORD;
        GlobData.Game_state.ShipStamp.origin.y = (((CanvasHeight
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
            - ((13 as ::core::ffi::c_int) << resolutionFactor) >> resolutionFactor
            >> 1 as ::core::ffi::c_int) << resolutionFactor)
            + (starbase_coords.y as ::core::ffi::c_int >> 1 as ::core::ffi::c_int))
            as COORD;
    }
}
#[no_mangle]
pub unsafe extern "C" fn VisitStarBase() {
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
        flash_rect2: RECT {
            corner: POINT { x: 0, y: 0 },
            extent: EXTENT { width: 0, height: 0 },
        },
        flash_frame0: ::core::ptr::null_mut::<FRAME_DESC>(),
        flash_frame1: ::core::ptr::null_mut::<FRAME_DESC>(),
        flashContext: ::core::ptr::null_mut::<FlashContext>(),
        hMusic: ::core::ptr::null_mut::<*mut TFB_SoundSample>(),
        privData: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    let mut OldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    let mut prevMsgMode: StatMsgMode = StatMsgMode::SMM_UNDEFINED;
    if uqm_lua_checked!(getGameStateUint(b"CHMMR_BOMB_STATE\0".as_ptr() as *const ::core::ffi::c_char), ())
        == 2 as DWORD
    {
        CurStarDescPtr = ::core::ptr::null_mut::<STAR_DESC>();
        uqm_lua_checked!(setGameStateUint(
            b"GLOBAL_FLAGS_AND_DATA\0".as_ptr() as *const ::core::ffi::c_char,
            !(0 as ::core::ffi::c_int) as BYTE as DWORD,
        ), ());
    }
    if uqm_lua_checked!(getGameStateUint(b"STARBASE_AVAILABLE\0".as_ptr() as *const ::core::ffi::c_char), ())
        == 0
    {
        let mut hStarShip: HSHIPFRAG = ::core::ptr::null_mut::<::core::ffi::c_void>();
        let mut FragPtr: *mut SHIP_FRAGMENT = ::core::ptr::null_mut::<SHIP_FRAGMENT>();
        SetCommIntroMode(CommIntroMode::CIM_CROSSFADE_SCREEN, 0 as TimeCount);
        uqm_lua_checked!(InitCommunication(CONVERSATION::COMMANDER_CONVERSATION), ());
        if uqm_lua_checked!(getGameStateUint(
            b"PROBE_ILWRATH_ENCOUNTER\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) == 0
            || GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
                & C2Rust_Unnamed_1::CHECK_ABORT.0 as ::core::ffi::c_int != 0
        {
            uqm_lua_checked!(CleanupAfterStarBase(), ());
            return;
        }
        hStarShip = CloneShipFragment(
            RACE_ID::ILWRATH_SHIP,
            &raw mut GlobData.Game_state.npc_built_ship_q,
            7 as COUNT,
        );
        FragPtr = LockShipFrag(&raw mut GlobData.Game_state.npc_built_ship_q, hStarShip);
        (*FragPtr).race_id = !(0 as ::core::ffi::c_int) as BYTE;
        UnlockLink(&raw mut GlobData.Game_state.npc_built_ship_q, hStarShip);
        uqm_lua_checked!(InitCommunication(CONVERSATION::ILWRATH_CONVERSATION), ());
        if GlobData.SIS_state.CrewEnlisted as ::core::ffi::c_int
            == !(0 as ::core::ffi::c_int) as COUNT as ::core::ffi::c_int
            || GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
                & C2Rust_Unnamed_1::CHECK_ABORT.0 as ::core::ffi::c_int != 0
        {
            return;
        }
        SetCommIntroMode(CommIntroMode::CIM_CROSSFADE_SCREEN, 0 as TimeCount);
        uqm_lua_checked!(InitCommunication(CONVERSATION::COMMANDER_CONVERSATION), ());
        if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
            & C2Rust_Unnamed_1::CHECK_ABORT.0 as ::core::ffi::c_int != 0
        {
            return;
        }
        uqm_lua_checked!(setGameStateUint(
            b"GLOBAL_FLAGS_AND_DATA\0".as_ptr() as *const ::core::ffi::c_char,
            !(0 as ::core::ffi::c_int) as BYTE as DWORD,
        ), ());
    }
    prevMsgMode = SetStatusMessageMode(StatMsgMode::SMM_RES_UNITS);
    if uqm_lua_checked!(getGameStateUint(b"MOONBASE_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char), ())
        != 0
        || uqm_lua_checked!(getGameStateUint(b"CHMMR_BOMB_STATE\0".as_ptr() as *const ::core::ffi::c_char), ())
            == 2 as DWORD
    {
        uqm_lua_checked!(DoTimePassage(), ());
        if GlobData.SIS_state.CrewEnlisted as ::core::ffi::c_int
            == !(0 as ::core::ffi::c_int) as COUNT as ::core::ffi::c_int
        {
            return;
        }
        SetCommIntroMode(
            CommIntroMode::CIM_FADE_IN_SCREEN,
            (840 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as TimeCount,
        );
        uqm_lua_checked!(InitCommunication(CONVERSATION::COMMANDER_CONVERSATION), ());
        if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
            & C2Rust_Unnamed_1::CHECK_ABORT.0 as ::core::ffi::c_int != 0
        {
            return;
        }
        uqm_lua_checked!(setGameStateUint(
            b"GLOBAL_FLAGS_AND_DATA\0".as_ptr() as *const ::core::ffi::c_char,
            !(0 as ::core::ffi::c_int) as BYTE as DWORD,
        ), ());
    }
    memset(
        &raw mut MenuState as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<MENU_STATE>(),
    );
    MenuState.InputFunc = Some(
        DoStarBase as unsafe extern "C" fn(*mut MENU_STATE) -> BOOLEAN,
    ) as Option<unsafe extern "C" fn(*mut menu_state) -> BOOLEAN>;
    OldContext = SetContext(ScreenContext);
    uqm_lua_checked!(DoInput(&raw mut MenuState as *mut ::core::ffi::c_void, BOOLEAN::TRUE), ());
    SetContext(OldContext);
    SetStatusMessageMode(prevMsgMode);
    uqm_lua_checked!(CleanupAfterStarBase(), ());
}
unsafe extern "C" fn CleanupAfterStarBase() {
    if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
        & (C2Rust_Unnamed_1::CHECK_LOAD.0 as ::core::ffi::c_int
            | C2Rust_Unnamed_1::CHECK_ABORT.0 as ::core::ffi::c_int) == 0
    {
        uqm_lua_checked!(setGameStateUint(
            b"GLOBAL_FLAGS_AND_DATA\0".as_ptr() as *const ::core::ffi::c_char,
            0 as DWORD,
        ), ());
        GlobData.Game_state.CurrentActivity = C2Rust_Unnamed_1::CHECK_LOAD.0
            as ::core::ffi::c_int as ACTIVITY;
        NextActivity = (((0 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int)
            << 8 as ::core::ffi::c_int) as UWORD as ::core::ffi::c_int
            | C2Rust_Unnamed_1::IN_INTERPLANETARY.0 as ::core::ffi::c_int as BYTE
                as ::core::ffi::c_int
            | C2Rust_Unnamed_1::START_INTERPLANETARY.0 as ::core::ffi::c_int)
            as ACTIVITY;
    }
}
#[no_mangle]
pub unsafe extern "C" fn InstallBombAtEarth() {
    uqm_lua_checked!(DoTimePassage(), ());
    GlobData.Game_state.ShipStamp.origin.x = ((CanvasWidth
        - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
        - (((13 as ::core::ffi::c_int) << resolutionFactor)
            + (if if optWindowType == 1 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } << resolutionFactor)) >> resolutionFactor >> 1 as ::core::ffi::c_int)
        << resolutionFactor) as COORD;
    GlobData.Game_state.ShipStamp.origin.y = ((CanvasHeight
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
        - ((13 as ::core::ffi::c_int) << resolutionFactor) >> resolutionFactor
        >> 1 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    SetContext(ScreenContext);
    SetTransitionSource(::core::ptr::null::<RECT>());
    SetContextBackGroundColor(colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE));
    uqm_lua_checked!(ClearDrawable(), ());
    uqm_lua_checked!(SleepThreadUntil(FadeScreen(ScreenFadeType::FadeAllToColor, 0 as SIZE)), ());
    uqm_lua_checked!(setGameStateUint(
        b"CHMMR_BOMB_STATE\0".as_ptr() as *const ::core::ffi::c_char,
        3 as DWORD,
    ), ());
    GlobData.Game_state.CurrentActivity = C2Rust_Unnamed_1::CHECK_LOAD.0
        as ::core::ffi::c_int as ACTIVITY;
    NextActivity = (((0 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int)
        << 8 as ::core::ffi::c_int) as UWORD as ::core::ffi::c_int
        | C2Rust_Unnamed_1::IN_INTERPLANETARY.0 as ::core::ffi::c_int as BYTE
            as ::core::ffi::c_int
        | C2Rust_Unnamed_1::START_INTERPLANETARY.0 as ::core::ffi::c_int) as ACTIVITY;
    CurStarDescPtr = ::core::ptr::null_mut::<STAR_DESC>();
}
#[no_mangle]
pub unsafe extern "C" fn WrapText(
    mut pStr: *const UNICODE,
    mut len: COUNT,
    mut tarray: *mut TEXT,
    mut field_width: SIZE,
) -> COUNT {
    let mut num_lines: COUNT = 0;
    num_lines = 0 as COUNT;
    loop {
        let mut r: RECT = RECT {
            corner: POINT { x: 0, y: 0 },
            extent: EXTENT { width: 0, height: 0 },
        };
        let mut OldCount: COUNT = 0;
        (*tarray).align = TEXT_ALIGN::ALIGN_LEFT;
        (*tarray).pStr = pStr;
        (*tarray).CharCount = 1 as COUNT;
        num_lines = num_lines.wrapping_add(1);
        loop {
            OldCount = (*tarray).CharCount;
            loop {
                pStr = pStr.offset(1);
                if !(*pStr as ::core::ffi::c_int != ' ' as ::core::ffi::c_int
                    && (pStr.offset_from((*tarray).pStr) as COUNT as ::core::ffi::c_int)
                        < len as ::core::ffi::c_int)
                {
                    break;
                }
            }
            (*tarray).CharCount = pStr.offset_from((*tarray).pStr) as COUNT;
            TextRect(tarray, &raw mut r, ::core::ptr::null_mut::<BYTE>());
            if !(((*tarray).CharCount as ::core::ffi::c_int) < len as ::core::ffi::c_int
                && (r.extent.width as ::core::ffi::c_int)
                    < field_width as ::core::ffi::c_int)
            {
                break;
            }
        }
        if r.extent.width as ::core::ffi::c_int >= field_width as ::core::ffi::c_int {
            (*tarray).CharCount = OldCount;
            if (*tarray).CharCount as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
                loop {
                    (*tarray).CharCount = (*tarray).CharCount.wrapping_add(1);
                    TextRect(tarray, &raw mut r, ::core::ptr::null_mut::<BYTE>());
                    if (r.extent.width as ::core::ffi::c_int)
                        >= field_width as ::core::ffi::c_int
                    {
                        break;
                    }
                }
                (*tarray).CharCount = (*tarray).CharCount.wrapping_sub(1);
            }
        }
        pStr = (*tarray).pStr.offset((*tarray).CharCount as ::core::ffi::c_int as isize);
        len = (len as ::core::ffi::c_int - (*tarray).CharCount as ::core::ffi::c_int)
            as COUNT;
        tarray = tarray.offset(1);
        if len as ::core::ffi::c_int != 0
            && ((r.extent.width as ::core::ffi::c_int)
                < field_width as ::core::ffi::c_int
                || OldCount as ::core::ffi::c_int > 1 as ::core::ffi::c_int)
        {
            pStr = pStr.offset(1);
            len = len.wrapping_sub(1);
        }
        if len == 0 {
            break;
        }
    }
    return num_lines;
}
