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

// Rust translation of src/uqm/save.c.
extern "C" {
    pub type uio_DirHandle;
    pub type uio_Stream;
    pub type string_table_entry;
    pub type context_desc;
    pub type frame_desc;
    pub type font_desc;
    pub type drawable_desc;
    pub type GAME_STATE_FILE;
    fn __assert_fail(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    ) -> !;
    fn BoxUnion(pr1: *mut RECT, pr2: *mut RECT, punion: *mut RECT);
    fn SetContext(Context: CONTEXT) -> CONTEXT;
    fn SetContextForeGroundColor(Color: Color) -> Color;
    fn DrawStamp(pStamp: *mut STAMP);
    fn font_DrawText(pText: *mut TEXT);
    fn BatchGraphics();
    fn UnbatchGraphics();
    fn FlushGraphics();
    fn DestroyDrawable(Drawable: DRAWABLE) -> BOOLEAN;
    fn SetContextFont(Font: FONT) -> FONT;
    fn TextRect(pText: *mut TEXT, pRect: *mut RECT, pdelta: *mut BYTE) -> BOOLEAN;
    fn ReleaseDrawable(Frame: FRAME) -> DRAWABLE;
    fn HFree(p: *mut ::core::ffi::c_void);
    fn sprintf(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn res_OpenResFile(
        dir: *mut uio_DirHandle,
        filename: *const ::core::ffi::c_char,
        mode: *const ::core::ffi::c_char,
    ) -> *mut uio_Stream;
    fn WriteResFile(
        lpBuf: *const ::core::ffi::c_void,
        size: size_t,
        count: size_t,
        fp: *mut uio_Stream,
    ) -> size_t;
    fn res_CloseResFile(fp: *mut uio_Stream) -> BOOLEAN;
    fn DeleteResFile(
        dir: *mut uio_DirHandle,
        filename: *const ::core::ffi::c_char,
    ) -> BOOLEAN;
    fn utf8StringCopy(
        dst: *mut ::core::ffi::c_char,
        size: size_t,
        src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_uchar;
    fn SetAbsStringTableIndex(String: STRING, StringTableIndex: COUNT) -> STRING;
    fn GetStringAddress(String: STRING) -> STRINGPTR;
    fn memset(
        _: *mut ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strncpy(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: size_t,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(_: *const ::core::ffi::c_char) -> size_t;
    fn WaitForAnyButton(
        newButton: BOOLEAN,
        duration: TimePeriod,
        resetInput: BOOLEAN,
    ) -> BOOLEAN;
    static mut optWhichFonts: ::core::ffi::c_int;
    static mut resolutionFactor: ::core::ffi::c_uint;
    static mut optWindowType: DWORD;
    static mut saveDir: *mut uio_DirHandle;
    static mut CanvasWidth: ::core::ffi::c_int;
    static mut CanvasHeight: ::core::ffi::c_int;
    fn CountLinks(pq: *mut QUEUE) -> COUNT;
    fn getGameStateUint(name: *const ::core::ffi::c_char) -> DWORD;
    fn serialiseGameState(
        bm: *const GameStateBitMap,
        buf: *mut *mut BYTE,
        numBytes: *mut size_t,
    ) -> BOOLEAN;
    static mut GlobData: GLOBDATA;
    fn inHQSpace() -> BOOLEAN;
    fn inQuasiSpace() -> BOOLEAN;
    fn isPC(optWhich: ::core::ffi::c_int) -> OPT_CONSOLETYPE;
    fn playerInSolarSystem() -> bool;
    fn playerInPlanetOrbit() -> bool;
    fn GetPlanetOrMoonName(buf: *mut UNICODE, bufsize: COUNT);
    fn SaveSolarSysLocation();
    fn GetIndexFromStarShip(pShipQ: *mut QUEUE, hStarShip: HLINK) -> COUNT;
    fn InventoryDevices(pDeviceMap: *mut BYTE, Size: COUNT) -> SIZE;
    static mut GameStrings: STRING;
    static mut SpaceContext: CONTEXT;
    static mut StarConFont: FONT;
    fn PutGroupInfo(offset: DWORD, which_group: BYTE) -> DWORD;
    fn OpenStateFile(
        stateFile: ::core::ffi::c_int,
        mode: *const ::core::ffi::c_char,
    ) -> *mut GAME_STATE_FILE;
    fn CloseStateFile(fp: *mut GAME_STATE_FILE);
    fn LengthStateFile(fp: *mut GAME_STATE_FILE) -> DWORD;
    fn ReadStateFile(
        lpBuf: *mut ::core::ffi::c_void,
        size: COUNT,
        count: COUNT,
        fp: *mut GAME_STATE_FILE,
    ) -> ::core::ffi::c_int;
    fn SeekStateFile(
        fp: *mut GAME_STATE_FILE,
        offset: ::core::ffi::c_long,
        whence: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn ReadGroupHeader(fp: *mut GAME_STATE_FILE, pGH: *mut GROUP_HEADER);
    fn ReadShipFragment(fp: *mut GAME_STATE_FILE, FragPtr: *mut SHIP_FRAGMENT);
    fn ReadIpGroup(fp: *mut GAME_STATE_FILE, GroupPtr: *mut IP_GROUP);
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
    fn SaveContextFrame(saveRect: *const RECT) -> STAMP;
    fn SaveSisHyperState();
    fn log_add(_: log_Level, fmt: *const ::core::ffi::c_char, ...);
    static mut CurStarDescPtr: *mut STAR_DESC;
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
pub struct C2Rust_Unnamed(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed {
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
pub type TimePeriod = DWORD;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct OPT_CONSOLETYPE(pub ::core::ffi::c_uint);
impl OPT_CONSOLETYPE {
    pub const OPTVAL_PC: Self = Self(0);
    pub const OPTVAL_3DO: Self = Self(1);
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
pub type HEVENT = HLINK;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event {
    pub pred: HEVENT,
    pub succ: HEVENT,
    pub day_index: BYTE,
    pub month_index: BYTE,
    pub year_index: COUNT,
    pub func_index: BYTE,
}
pub type EVENT = event;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GameStateBitMap {
    pub name: *const ::core::ffi::c_char,
    pub numBits: BYTE,
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
pub struct SUMMARY_DESC {
    pub SS: SIS_STATE,
    pub Activity: BYTE,
    pub Flags: BYTE,
    pub day_index: BYTE,
    pub month_index: BYTE,
    pub year_index: COUNT,
    pub MCreditLo: BYTE,
    pub MCreditHi: BYTE,
    pub NumShips: BYTE,
    pub NumDevices: BYTE,
    pub ShipList: [BYTE; 12],
    pub DeviceList: [BYTE; 16],
    pub SaveName: [UNICODE; 64],
    pub SaveNameChecker: [UNICODE; 32],
    pub LegacySaveName: [UNICODE; 32],
    pub res_factor: BYTE,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GROUP_HEADER {
    pub NumGroups: BYTE,
    pub day_index: BYTE,
    pub month_index: BYTE,
    pub star_index: COUNT,
    pub year_index: COUNT,
    pub GroupOffset: [DWORD; 65],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IP_GROUP {
    pub pred: HIPGROUP,
    pub succ: HIPGROUP,
    pub group_counter: UWORD,
    pub race_id: BYTE,
    pub sys_loc: BYTE,
    pub task: BYTE,
    pub in_system: BYTE,
    pub dest_loc: BYTE,
    pub orbit_pos: BYTE,
    pub group_id: BYTE,
    pub loc: POINT,
    pub melee_icon: FRAME,
    pub flags: IP_FLAGS,
}
pub type IP_FLAGS = BYTE;
pub type HIPGROUP = HLINK;
pub type HENCOUNTER = HLINK;
pub type ENCOUNTER = encounter;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct encounter {
    pub pred: HENCOUNTER,
    pub succ: HENCOUNTER,
    pub hElement: HELEMENT,
    pub transition_state: SIZE,
    pub origin: POINT,
    pub radius: COUNT,
    pub race_id: BYTE,
    pub num_ships: BYTE,
    pub flags: BYTE,
    pub loc_pt: POINT,
    pub ShipList: [BRIEF_SHIP_INFO; 7],
    pub log_x: SDWORD,
    pub log_y: SDWORD,
}
pub type BRIEF_SHIP_INFO = brief_ship_info;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct brief_ship_info {
    pub race_id: BYTE,
    pub crew_level: COUNT,
    pub max_crew: COUNT,
    pub max_energy: BYTE,
}
pub type HELEMENT = HLINK;
pub type HSHIPFRAG = HLINK;
pub type HFLEETINFO = HLINK;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FLEET_INFO {
    pub pred: HFLEETINFO,
    pub succ: HFLEETINFO,
    pub SpeciesID: SPECIES_ID,
    pub shipIdStr: *const ::core::ffi::c_char,
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
    pub can_build: BOOLEAN,
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
static mut io_ok: BOOLEAN = BOOLEAN::TRUE;
#[no_mangle]
pub static mut gameStateBitMap: [GameStateBitMap; 459] = [
    GameStateBitMap {
        name: b"SHOFIXTI_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"SHOFIXTI_STACK1\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"SHOFIXTI_STACK2\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"SHOFIXTI_STACK3\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"SHOFIXTI_KIA\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"SHOFIXTI_BRO_KIA\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"SHOFIXTI_RECRUITED\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"SHOFIXTI_MAIDENS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"MAIDENS_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"BATTLE_SEGUE\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"PLANETARY_LANDING\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"PLANETARY_CHANGE\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"SPATHI_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"SPATHI_HOME_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"FOUND_PLUTO_SPATHI\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"SPATHI_SHIELDED_SELVES\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"SPATHI_CREATURES_EXAMINED\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"SPATHI_CREATURES_ELIMINATED\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"UMGAH_BROADCASTERS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"SPATHI_MANNER\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"SPATHI_QUEST\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"LIED_ABOUT_CREATURES\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"SPATHI_PARTY\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"KNOW_SPATHI_PASSWORD\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"ILWRATH_HOME_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"ILWRATH_CHMMR_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"ARILOU_SPACE\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"ARILOU_SPACE_SIDE\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"ARILOU_SPACE_COUNTER\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 4 as BYTE,
    },
    GameStateBitMap {
        name: b"LANDER_SHIELDS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 4 as BYTE,
    },
    GameStateBitMap {
        name: b"MET_MELNORME\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"MELNORME_RESCUE_REFUSED\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"MELNORME_RESCUE_COUNT\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"TRADED_WITH_MELNORME\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"WHY_MELNORME_PURPLE\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"MELNORME_CREDIT0\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 8 as BYTE,
    },
    GameStateBitMap {
        name: b"MELNORME_CREDIT1\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 8 as BYTE,
    },
    GameStateBitMap {
        name: b"MELNORME_BUSINESS_COUNT\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"MELNORME_YACK_STACK0\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"MELNORME_YACK_STACK1\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"MELNORME_YACK_STACK2\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 4 as BYTE,
    },
    GameStateBitMap {
        name: b"MELNORME_YACK_STACK3\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"MELNORME_YACK_STACK4\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"WHY_MELNORME_BLUE\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"MELNORME_ANGER\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"MELNORME_MIFFED_COUNT\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"MELNORME_PISSED_COUNT\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"MELNORME_HATE_COUNT\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"PROBE_MESSAGE_DELIVERED\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"PROBE_ILWRATH_ENCOUNTER\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"STARBASE_AVAILABLE\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"STARBASE_VISITED\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"RADIOACTIVES_PROVIDED\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"LANDERS_LOST\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"GIVEN_FUEL_BEFORE\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"AWARE_OF_SAMATRA\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"YEHAT_CAVALRY_ARRIVED\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"URQUAN_MESSED_UP\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"MOONBASE_DESTROYED\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"WILL_DESTROY_BASE\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"WIMBLIS_TRIDENT_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"GLOWING_ROD_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"KOHR_AH_KILLED_ALL\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"STARBASE_YACK_STACK1\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"DISCUSSED_PORTAL_SPAWNER\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"DISCUSSED_TALKING_PET\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"DISCUSSED_UTWIG_BOMB\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"DISCUSSED_SUN_EFFICIENCY\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"DISCUSSED_ROSY_SPHERE\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"DISCUSSED_AQUA_HELIX\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"DISCUSSED_CLEAR_SPINDLE\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"DISCUSSED_ULTRON\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"DISCUSSED_MAIDENS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"DISCUSSED_UMGAH_HYPERWAVE\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"DISCUSSED_BURVIX_HYPERWAVE\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"SYREEN_WANT_PROOF\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"PLAYER_HAVING_SEX\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"MET_ARILOU\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"DISCUSSED_TAALO_PROTECTOR\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"DISCUSSED_EGG_CASING0\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"DISCUSSED_EGG_CASING1\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"DISCUSSED_EGG_CASING2\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"DISCUSSED_SYREEN_SHUTTLE\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"DISCUSSED_VUX_BEAST\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"DISCUSSED_DESTRUCT_CODE\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"DISCUSSED_URQUAN_WARP\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"DISCUSSED_WIMBLIS_TRIDENT\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"DISCUSSED_GLOWING_ROD\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"ATTACKED_DRUUGE\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"NEW_ALLIANCE_NAME\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"PORTAL_COUNTER\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 4 as BYTE,
    },
    GameStateBitMap {
        name: b"BURVIXESE_BROADCASTERS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"BURV_BROADCASTERS_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"UTWIG_BOMB\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"UTWIG_BOMB_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"AQUA_HELIX\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"AQUA_HELIX_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"SUN_DEVICE\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"SUN_DEVICE_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"TAALO_PROTECTOR\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"TAALO_PROTECTOR_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"SHIP_VAULT_UNLOCKED\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"SYREEN_SHUTTLE\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"PORTAL_KEY\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"PORTAL_KEY_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"VUX_BEAST\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"VUX_BEAST_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"TALKING_PET\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"TALKING_PET_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"MOONBASE_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"KOHR_AH_FRENZY\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"KOHR_AH_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"KOHR_AH_BYES\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"SLYLANDRO_HOME_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"DESTRUCT_CODE_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"ILWRATH_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"ILWRATH_DECEIVED\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"FLAGSHIP_CLOAKED\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"MYCON_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"MYCON_HOME_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"MYCON_AMBUSH\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"MYCON_FELL_FOR_AMBUSH\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"GLOBAL_FLAGS_AND_DATA\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 8 as BYTE,
    },
    GameStateBitMap {
        name: b"ORZ_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"TAALO_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"ORZ_MANNER\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"PROBE_EXHIBITED_BUG\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"CLEAR_SPINDLE_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"URQUAN_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"PLAYER_HYPNOTIZED\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"VUX_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"VUX_HOME_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"ZEX_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"ZEX_IS_DEAD\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"KNOW_ZEX_WANTS_MONSTER\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"UTWIG_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"UTWIG_HOME_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"BOMB_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"ULTRON_CONDITION\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"UTWIG_HAVE_ULTRON\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"BOMB_UNPROTECTED\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"TAALO_UNPROTECTED\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"TALKING_PET_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"TALKING_PET_HOME_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"UMGAH_ZOMBIE_BLOBBIES\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"KNOW_UMGAH_ZOMBIES\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"ARILOU_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"ARILOU_HOME_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"KNOW_ARILOU_WANT_WRECK\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"ARILOU_CHECKED_UMGAH\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"PORTAL_SPAWNER\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"PORTAL_SPAWNER_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"UMGAH_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"UMGAH_HOME_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"MET_NORMAL_UMGAH\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"SYREEN_HOME_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"SYREEN_SHUTTLE_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"KNOW_SYREEN_VAULT\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"EGG_CASE0_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"SUN_DEVICE_UNGUARDED\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"ROSY_SPHERE_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"CHMMR_HOME_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"CHMMR_EMERGING\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"CHMMR_UNLEASHED\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"CHMMR_BOMB_STATE\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"DRUUGE_DISCLAIMER\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"YEHAT_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"YEHAT_REBEL_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"YEHAT_HOME_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"YEHAT_CIVIL_WAR\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"YEHAT_ABSORBED_PKUNK\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"YEHAT_SHIP_MONTH\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 4 as BYTE,
    },
    GameStateBitMap {
        name: b"YEHAT_SHIP_DAY\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 5 as BYTE,
    },
    GameStateBitMap {
        name: b"YEHAT_SHIP_YEAR\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 5 as BYTE,
    },
    GameStateBitMap {
        name: b"CLEAR_SPINDLE\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"PKUNK_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"PKUNK_HOME_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"PKUNK_SHIP_MONTH\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 4 as BYTE,
    },
    GameStateBitMap {
        name: b"PKUNK_SHIP_DAY\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 5 as BYTE,
    },
    GameStateBitMap {
        name: b"PKUNK_SHIP_YEAR\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 5 as BYTE,
    },
    GameStateBitMap {
        name: b"PKUNK_MISSION\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"SUPOX_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"SUPOX_HOME_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"THRADD_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"THRADD_HOME_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"HELIX_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"HELIX_UNPROTECTED\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"THRADD_CULTURE\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"THRADD_MISSION\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"DRUUGE_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"DRUUGE_HOME_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"ROSY_SPHERE\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"SCANNED_MAIDENS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"SCANNED_FRAGMENTS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"SCANNED_CASTER\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"SCANNED_SPAWNER\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"SCANNED_ULTRON\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"ZOQFOT_INFO\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"ZOQFOT_HOSTILE\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"ZOQFOT_HOME_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"MET_ZOQFOT\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"ZOQFOT_DISTRESS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"EGG_CASE1_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"EGG_CASE2_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"MYCON_SUN_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"ORZ_HOME_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"MELNORME_FUEL_PROCEDURE\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"MELNORME_TECH_PROCEDURE\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"MELNORME_INFO_PROCEDURE\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"MELNORME_TECH_STACK\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 4 as BYTE,
    },
    GameStateBitMap {
        name: b"MELNORME_EVENTS_INFO_STACK\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 5 as BYTE,
    },
    GameStateBitMap {
        name: b"MELNORME_ALIEN_INFO_STACK\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 5 as BYTE,
    },
    GameStateBitMap {
        name: b"MELNORME_HISTORY_INFO_STACK\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 5 as BYTE,
    },
    GameStateBitMap {
        name: b"RAINBOW_WORLD0\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 8 as BYTE,
    },
    GameStateBitMap {
        name: b"RAINBOW_WORLD1\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"MELNORME_RAINBOW_COUNT\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 4 as BYTE,
    },
    GameStateBitMap {
        name: b"USED_BROADCASTER\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"BROADCASTER_RESPONSE\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"IMPROVED_LANDER_SPEED\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"IMPROVED_LANDER_CARGO\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"IMPROVED_LANDER_SHOT\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"MET_ORZ_BEFORE\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"YEHAT_REBEL_TOLD_PKUNK\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"PLAYER_HAD_SEX\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"UMGAH_BROADCASTERS_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"LIGHT_MINERAL_LOAD\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"MEDIUM_MINERAL_LOAD\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"HEAVY_MINERAL_LOAD\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"STARBASE_BULLETS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 32 as BYTE,
    },
    GameStateBitMap {
        name: b"STARBASE_MONTH\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 4 as BYTE,
    },
    GameStateBitMap {
        name: b"STARBASE_DAY\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 5 as BYTE,
    },
    GameStateBitMap {
        name: b"CREW_SOLD_TO_DRUUGE0\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 8 as BYTE,
    },
    GameStateBitMap {
        name: b"CREW_PURCHASED0\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 8 as BYTE,
    },
    GameStateBitMap {
        name: b"CREW_PURCHASED1\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 8 as BYTE,
    },
    GameStateBitMap {
        name: b"URQUAN_PROTECTING_SAMATRA\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"THRADDASH_BODY_COUNT\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 5 as BYTE,
    },
    GameStateBitMap {
        name: b"UTWIG_SUPOX_MISSION\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"SPATHI_INFO\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"ILWRATH_INFO\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"ILWRATH_GODS_SPOKEN\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 4 as BYTE,
    },
    GameStateBitMap {
        name: b"ILWRATH_WORSHIP\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"ILWRATH_FIGHT_THRADDASH\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"READY_TO_CONFUSE_URQUAN\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"URQUAN_HYPNO_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"MENTIONED_PET_COMPULSION\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"URQUAN_INFO\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"KNOW_URQUAN_STORY\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"MYCON_INFO\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 4 as BYTE,
    },
    GameStateBitMap {
        name: b"MYCON_RAMBLE\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 5 as BYTE,
    },
    GameStateBitMap {
        name: b"KNOW_ABOUT_SHATTERED\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"MYCON_INSULTS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"MYCON_KNOW_AMBUSH\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"SYREEN_INFO\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"KNOW_SYREEN_WORLD_SHATTERED\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"SYREEN_KNOW_ABOUT_MYCON\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"TALKING_PET_INFO\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"TALKING_PET_SUGGESTIONS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"LEARNED_TALKING_PET\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"DNYARRI_LIED\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"SHIP_TO_COMPEL\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"ORZ_GENERAL_INFO\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"ORZ_PERSONAL_INFO\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"ORZ_ANDRO_STATE\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"REFUSED_ORZ_ALLIANCE\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"PKUNK_MANNER\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"PKUNK_ON_THE_MOVE\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"PKUNK_FLEET\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"PKUNK_MIGRATE\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"PKUNK_RETURN\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"PKUNK_WORRY\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"PKUNK_INFO\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"PKUNK_WAR\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"PKUNK_FORTUNE\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"PKUNK_MIGRATE_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"PKUNK_REASONS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 4 as BYTE,
    },
    GameStateBitMap {
        name: b"PKUNK_SWITCH\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"PKUNK_SENSE_VICTOR\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"KOHR_AH_REASONS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"KOHR_AH_PLEAD\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"KOHR_AH_INFO\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"KNOW_KOHR_AH_STORY\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"KOHR_AH_SENSES_EVIL\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"URQUAN_SENSES_EVIL\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"SLYLANDRO_PROBE_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"SLYLANDRO_PROBE_THREAT\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"SLYLANDRO_PROBE_WRONG\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"SLYLANDRO_PROBE_ID\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"SLYLANDRO_PROBE_INFO\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"SLYLANDRO_PROBE_EXIT\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"UMGAH_HOSTILE\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"UMGAH_EVIL_BLOBBIES\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"UMGAH_MENTIONED_TRICKS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"BOMB_CARRIER\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"THRADD_MANNER\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"THRADD_INTRO\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"THRADD_DEMEANOR\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"THRADD_INFO\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"THRADD_BODY_LEVEL\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"THRADD_MISSION_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"THRADD_STACK_1\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"THRADD_HOSTILE_STACK_2\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"THRADD_HOSTILE_STACK_3\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"THRADD_HOSTILE_STACK_4\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"THRADD_HOSTILE_STACK_5\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"CHMMR_STACK\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"ARILOU_MANNER\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"NO_PORTAL_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"ARILOU_STACK_1\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"ARILOU_STACK_2\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"ARILOU_STACK_3\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"ARILOU_STACK_4\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"ARILOU_STACK_5\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"ARILOU_INFO\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"ARILOU_HINTS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"DRUUGE_MANNER\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"DRUUGE_SPACE_INFO\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"DRUUGE_HOME_INFO\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"DRUUGE_SALVAGE\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"KNOW_DRUUGE_SLAVERS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"FRAGMENTS_BOUGHT\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"ZEX_STACK_1\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"ZEX_STACK_2\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"ZEX_STACK_3\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"VUX_INFO\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"VUX_STACK_1\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 4 as BYTE,
    },
    GameStateBitMap {
        name: b"VUX_STACK_2\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"VUX_STACK_3\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"VUX_STACK_4\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"SHOFIXTI_STACK4\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"YEHAT_REBEL_INFO\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"YEHAT_ROYALIST_INFO\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"YEHAT_ROYALIST_TOLD_PKUNK\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"NO_YEHAT_ALLY_HOME\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"NO_YEHAT_HELP_HOME\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"NO_YEHAT_INFO\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"NO_YEHAT_ALLY_SPACE\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"NO_YEHAT_HELP_SPACE\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"ZOQFOT_KNOW_MASK\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 4 as BYTE,
    },
    GameStateBitMap {
        name: b"SUPOX_HOSTILE\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"SUPOX_INFO\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"SUPOX_WAR_NEWS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"SUPOX_ULTRON_HELP\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"SUPOX_STACK1\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"SUPOX_STACK2\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"UTWIG_HOSTILE\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"UTWIG_INFO\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"UTWIG_WAR_NEWS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"UTWIG_STACK1\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"UTWIG_STACK2\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"BOMB_INFO\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"BOMB_STACK1\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"BOMB_STACK2\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"SLYLANDRO_KNOW_BROKEN\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"PLAYER_KNOWS_PROBE\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"PLAYER_KNOWS_PROGRAM\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"PLAYER_KNOWS_EFFECTS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"PLAYER_KNOWS_PRIORITY\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"SLYLANDRO_STACK1\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"SLYLANDRO_STACK2\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"SLYLANDRO_STACK3\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"SLYLANDRO_STACK4\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"SLYLANDRO_STACK5\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"SLYLANDRO_STACK6\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"SLYLANDRO_STACK7\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"SLYLANDRO_STACK8\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"SLYLANDRO_STACK9\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"SLYLANDRO_KNOW_EARTH\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"SLYLANDRO_KNOW_EXPLORE\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"SLYLANDRO_KNOW_GATHER\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"SLYLANDRO_KNOW_URQUAN\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"RECALL_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"SLYLANDRO_MULTIPLIER\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"KNOW_SPATHI_QUEST\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"KNOW_SPATHI_EVIL\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"BATTLE_PLANET\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 8 as BYTE,
    },
    GameStateBitMap {
        name: b"ESCAPE_COUNTER\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 8 as BYTE,
    },
    GameStateBitMap {
        name: b"CREW_SOLD_TO_DRUUGE1\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 8 as BYTE,
    },
    GameStateBitMap {
        name: b"PKUNK_DONE_WAR\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"SYREEN_STACK0\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"SYREEN_STACK1\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"SYREEN_STACK2\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"REFUSED_ULTRON_AT_BOMB\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"NO_TRICK_AT_SUN\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"SPATHI_STACK0\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"SPATHI_STACK1\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"SPATHI_STACK2\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"ORZ_STACK0\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"ORZ_STACK1\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"SHOFIXTI_GRPOFFS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 32 as BYTE,
    },
    GameStateBitMap {
        name: b"ZOQFOT_GRPOFFS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 32 as BYTE,
    },
    GameStateBitMap {
        name: b"MELNORME0_GRPOFFS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 32 as BYTE,
    },
    GameStateBitMap {
        name: b"MELNORME1_GRPOFFS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 32 as BYTE,
    },
    GameStateBitMap {
        name: b"MELNORME2_GRPOFFS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 32 as BYTE,
    },
    GameStateBitMap {
        name: b"MELNORME3_GRPOFFS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 32 as BYTE,
    },
    GameStateBitMap {
        name: b"MELNORME4_GRPOFFS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 32 as BYTE,
    },
    GameStateBitMap {
        name: b"MELNORME5_GRPOFFS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 32 as BYTE,
    },
    GameStateBitMap {
        name: b"MELNORME6_GRPOFFS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 32 as BYTE,
    },
    GameStateBitMap {
        name: b"MELNORME7_GRPOFFS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 32 as BYTE,
    },
    GameStateBitMap {
        name: b"MELNORME8_GRPOFFS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 32 as BYTE,
    },
    GameStateBitMap {
        name: b"URQUAN_PROBE_GRPOFFS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 32 as BYTE,
    },
    GameStateBitMap {
        name: b"COLONY_GRPOFFS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 32 as BYTE,
    },
    GameStateBitMap {
        name: b"SAMATRA_GRPOFFS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 32 as BYTE,
    },
    GameStateBitMap {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"AUTOPILOT_OK\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: b"KNOW_QS_PORTAL\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 16 as BYTE,
    },
    GameStateBitMap {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"SYS_VISITED_00\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 32 as BYTE,
    },
    GameStateBitMap {
        name: b"SYS_VISITED_01\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 32 as BYTE,
    },
    GameStateBitMap {
        name: b"SYS_VISITED_02\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 32 as BYTE,
    },
    GameStateBitMap {
        name: b"SYS_VISITED_03\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 32 as BYTE,
    },
    GameStateBitMap {
        name: b"SYS_VISITED_04\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 32 as BYTE,
    },
    GameStateBitMap {
        name: b"SYS_VISITED_05\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 32 as BYTE,
    },
    GameStateBitMap {
        name: b"SYS_VISITED_06\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 32 as BYTE,
    },
    GameStateBitMap {
        name: b"SYS_VISITED_07\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 32 as BYTE,
    },
    GameStateBitMap {
        name: b"SYS_VISITED_08\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 32 as BYTE,
    },
    GameStateBitMap {
        name: b"SYS_VISITED_09\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 32 as BYTE,
    },
    GameStateBitMap {
        name: b"SYS_VISITED_10\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 32 as BYTE,
    },
    GameStateBitMap {
        name: b"SYS_VISITED_11\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 32 as BYTE,
    },
    GameStateBitMap {
        name: b"SYS_VISITED_12\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 32 as BYTE,
    },
    GameStateBitMap {
        name: b"SYS_VISITED_13\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 32 as BYTE,
    },
    GameStateBitMap {
        name: b"SYS_VISITED_14\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 32 as BYTE,
    },
    GameStateBitMap {
        name: b"SYS_VISITED_15\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 32 as BYTE,
    },
    GameStateBitMap {
        name: b"KNOW_HOMEWORLD\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 18 as BYTE,
    },
    GameStateBitMap {
        name: b"HM_ENCOUNTERS\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 9 as BYTE,
    },
    GameStateBitMap {
        name: b"RESERVED\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 32 as BYTE,
    },
    GameStateBitMap {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        numBits: 3 as BYTE,
    },
    GameStateBitMap {
        name: b"SYS_PLYR_MARKER_00\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 32 as BYTE,
    },
    GameStateBitMap {
        name: b"SYS_PLYR_MARKER_01\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 32 as BYTE,
    },
    GameStateBitMap {
        name: b"SYS_PLYR_MARKER_02\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 32 as BYTE,
    },
    GameStateBitMap {
        name: b"SYS_PLYR_MARKER_03\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 32 as BYTE,
    },
    GameStateBitMap {
        name: b"SYS_PLYR_MARKER_04\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 32 as BYTE,
    },
    GameStateBitMap {
        name: b"SYS_PLYR_MARKER_05\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 32 as BYTE,
    },
    GameStateBitMap {
        name: b"SYS_PLYR_MARKER_06\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 32 as BYTE,
    },
    GameStateBitMap {
        name: b"SYS_PLYR_MARKER_07\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 32 as BYTE,
    },
    GameStateBitMap {
        name: b"SYS_PLYR_MARKER_08\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 32 as BYTE,
    },
    GameStateBitMap {
        name: b"SYS_PLYR_MARKER_09\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 32 as BYTE,
    },
    GameStateBitMap {
        name: b"SYS_PLYR_MARKER_10\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 32 as BYTE,
    },
    GameStateBitMap {
        name: b"SYS_PLYR_MARKER_11\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 32 as BYTE,
    },
    GameStateBitMap {
        name: b"SYS_PLYR_MARKER_12\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 32 as BYTE,
    },
    GameStateBitMap {
        name: b"SYS_PLYR_MARKER_13\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 32 as BYTE,
    },
    GameStateBitMap {
        name: b"SYS_PLYR_MARKER_14\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 32 as BYTE,
    },
    GameStateBitMap {
        name: b"SYS_PLYR_MARKER_15\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 32 as BYTE,
    },
    GameStateBitMap {
        name: b"LAST_LOCATION_X\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 16 as BYTE,
    },
    GameStateBitMap {
        name: b"LAST_LOCATION_Y\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 16 as BYTE,
    },
    GameStateBitMap {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        numBits: 4 as BYTE,
    },
    GameStateBitMap {
        name: b"ADV_AUTOPILOT_SAVE_X\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 16 as BYTE,
    },
    GameStateBitMap {
        name: b"ADV_AUTOPILOT_SAVE_Y\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 16 as BYTE,
    },
    GameStateBitMap {
        name: b"ADV_AUTOPILOT_QUASI_X\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 16 as BYTE,
    },
    GameStateBitMap {
        name: b"ADV_AUTOPILOT_QUASI_Y\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 16 as BYTE,
    },
    GameStateBitMap {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        numBits: 5 as BYTE,
    },
    GameStateBitMap {
        name: b"SEED_TYPE\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 2 as BYTE,
    },
    GameStateBitMap {
        name: b"SUPOX_SHIP_MONTH\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 4 as BYTE,
    },
    GameStateBitMap {
        name: b"SUPOX_SHIP_DAY\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 5 as BYTE,
    },
    GameStateBitMap {
        name: b"SUPOX_SHIP_YEAR\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 5 as BYTE,
    },
    GameStateBitMap {
        name: b"UTWIG_SHIP_MONTH\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 4 as BYTE,
    },
    GameStateBitMap {
        name: b"UTWIG_SHIP_DAY\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 5 as BYTE,
    },
    GameStateBitMap {
        name: b"UTWIG_SHIP_YEAR\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 5 as BYTE,
    },
    GameStateBitMap {
        name: b"REV_5_PAD\0".as_ptr() as *const ::core::ffi::c_char,
        numBits: 1 as BYTE,
    },
    GameStateBitMap {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        numBits: 0 as BYTE,
    },
];
#[no_mangle]
pub static mut gameStateBitMapRevTag: [*const ::core::ffi::c_char; 6] = [
    b"Core UQM v0.8.0\0".as_ptr() as *const ::core::ffi::c_char,
    b"MegaMod v0.8.0.85\0".as_ptr() as *const ::core::ffi::c_char,
    b"MegaMod v0.8.1\0".as_ptr() as *const ::core::ffi::c_char,
    b"MegaMod v0.8.2\0".as_ptr() as *const ::core::ffi::c_char,
    b"MegaMod v0.8.3\0".as_ptr() as *const ::core::ffi::c_char,
    b"MegaMod v0.8.4\0".as_ptr() as *const ::core::ffi::c_char,
];
#[inline]
unsafe extern "C" fn write_8(mut fp: *mut ::core::ffi::c_void, mut v: BYTE) {
    if io_ok.0 != 0 {
        if uqm_lua_checked!(WriteResFile(
            &raw mut v as *const ::core::ffi::c_void,
            1 as size_t,
            1 as size_t,
            fp as *mut uio_Stream,
        ), ()) != 1 as size_t
        {
            io_ok = BOOLEAN::FALSE;
        }
    }
}
#[inline]
unsafe extern "C" fn write_16(mut fp: *mut ::core::ffi::c_void, mut v: UWORD) {
    uqm_lua_checked!(write_8(fp, (v as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as BYTE), ());
    uqm_lua_checked!(write_8(
        fp,
        (v as ::core::ffi::c_int >> 8 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int)
            as BYTE,
    ), ());
}
#[inline]
unsafe extern "C" fn write_32(mut fp: *mut ::core::ffi::c_void, mut v: DWORD) {
    uqm_lua_checked!(write_8(fp, (v & 0xff as DWORD) as BYTE), ());
    uqm_lua_checked!(write_8(fp, (v >> 8 as ::core::ffi::c_int & 0xff as DWORD) as BYTE), ());
    uqm_lua_checked!(write_8(fp, (v >> 16 as ::core::ffi::c_int & 0xff as DWORD) as BYTE), ());
    uqm_lua_checked!(write_8(fp, (v >> 24 as ::core::ffi::c_int & 0xff as DWORD) as BYTE), ());
}
#[inline]
unsafe extern "C" fn write_a8(
    mut fp: *mut ::core::ffi::c_void,
    mut ar: *const BYTE,
    mut count: COUNT,
) {
    if io_ok.0 != 0 {
        if uqm_lua_checked!(WriteResFile(
            ar as *const ::core::ffi::c_void,
            1 as size_t,
            count as size_t,
            fp as *mut uio_Stream,
        ), ()) != count as size_t
        {
            io_ok = BOOLEAN::FALSE;
        }
    }
}
#[inline]
unsafe extern "C" fn write_str(
    mut fp: *mut ::core::ffi::c_void,
    mut str: *const ::core::ffi::c_char,
    mut count: COUNT,
) {
    uqm_lua_checked!(write_a8(fp, str as *const BYTE, count), ());
}
#[inline]
unsafe extern "C" fn write_a16(
    mut fp: *mut ::core::ffi::c_void,
    mut ar: *const UWORD,
    mut count: COUNT,
) {
    while count as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
        if io_ok.0 == 0 {
            break;
        }
        uqm_lua_checked!(write_16(fp, *ar), ());
        count = count.wrapping_sub(1);
        ar = ar.offset(1);
    }
}
unsafe extern "C" fn SaveShipQueue(
    mut fh: *mut uio_Stream,
    mut pQueue: *mut QUEUE,
    mut tag: DWORD,
) {
    let mut num_links: COUNT = 0;
    let mut hStarShip: HSHIPFRAG = ::core::ptr::null_mut::<::core::ffi::c_void>();
    num_links = CountLinks(pQueue);
    if num_links as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        return;
    }
    uqm_lua_checked!(write_32(fh as *mut ::core::ffi::c_void, tag), ());
    uqm_lua_checked!(write_32(
        fh as *mut ::core::ffi::c_void,
        (num_links as ::core::ffi::c_int * 11 as ::core::ffi::c_int) as DWORD,
    ), ());
    hStarShip = (*pQueue).head as HSHIPFRAG;
    loop {
        let c2rust_fresh2 = num_links;
        num_links = num_links.wrapping_sub(1);
        if c2rust_fresh2 == 0 {
            break;
        }
        let mut hNextShip: HSHIPFRAG = ::core::ptr::null_mut::<::core::ffi::c_void>();
        let mut FragPtr: *mut SHIP_FRAGMENT = ::core::ptr::null_mut::<SHIP_FRAGMENT>();
        let mut Index: COUNT = 0;
        FragPtr = LockShipFrag(pQueue, hStarShip);
        hNextShip = (*FragPtr).succ as HSHIPFRAG;
        Index = (*FragPtr).race_id as COUNT;
        uqm_lua_checked!(write_16(fh as *mut ::core::ffi::c_void, Index), ());
        uqm_lua_checked!(write_8(fh as *mut ::core::ffi::c_void, (*FragPtr).captains_name_index), ());
        uqm_lua_checked!(write_8(fh as *mut ::core::ffi::c_void, (*FragPtr).race_id), ());
        uqm_lua_checked!(write_8(fh as *mut ::core::ffi::c_void, (*FragPtr).index), ());
        uqm_lua_checked!(write_16(fh as *mut ::core::ffi::c_void, (*FragPtr).crew_level), ());
        uqm_lua_checked!(write_16(fh as *mut ::core::ffi::c_void, (*FragPtr).max_crew), ());
        uqm_lua_checked!(write_8(fh as *mut ::core::ffi::c_void, (*FragPtr).energy_level), ());
        uqm_lua_checked!(write_8(fh as *mut ::core::ffi::c_void, (*FragPtr).max_energy), ());
        UnlockLink(pQueue, hStarShip);
        hStarShip = hNextShip;
    };
}
unsafe extern "C" fn SaveRaceQueue(mut fh: *mut uio_Stream, mut pQueue: *mut QUEUE) {
    let mut num_links: COUNT = 0;
    let mut hFleet: HFLEETINFO = ::core::ptr::null_mut::<::core::ffi::c_void>();
    num_links = CountLinks(pQueue);
    if num_links as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        return;
    }
    uqm_lua_checked!(write_32(fh as *mut ::core::ffi::c_void, 0x51636152 as ::core::ffi::c_int as DWORD), ());
    uqm_lua_checked!(write_32(
        fh as *mut ::core::ffi::c_void,
        (num_links as ::core::ffi::c_int * 30 as ::core::ffi::c_int) as DWORD,
    ), ());
    hFleet = (*pQueue).head as HFLEETINFO;
    loop {
        let c2rust_fresh3 = num_links;
        num_links = num_links.wrapping_sub(1);
        if c2rust_fresh3 == 0 {
            break;
        }
        let mut hNextFleet: HFLEETINFO = ::core::ptr::null_mut::<::core::ffi::c_void>();
        let mut FleetPtr: *mut FLEET_INFO = ::core::ptr::null_mut::<FLEET_INFO>();
        let mut Index: COUNT = 0;
        FleetPtr = LockFleetInfo(pQueue, hFleet);
        hNextFleet = (*FleetPtr).succ;
        Index = GetIndexFromStarShip(pQueue, hFleet);
        uqm_lua_checked!(write_16(fh as *mut ::core::ffi::c_void, Index), ());
        uqm_lua_checked!(write_16(fh as *mut ::core::ffi::c_void, (*FleetPtr).allied_state), ());
        uqm_lua_checked!(write_8(fh as *mut ::core::ffi::c_void, (*FleetPtr).days_left), ());
        uqm_lua_checked!(write_8(fh as *mut ::core::ffi::c_void, (*FleetPtr).growth_fract), ());
        uqm_lua_checked!(write_16(fh as *mut ::core::ffi::c_void, (*FleetPtr).crew_level), ());
        uqm_lua_checked!(write_16(fh as *mut ::core::ffi::c_void, (*FleetPtr).max_crew), ());
        uqm_lua_checked!(write_8(fh as *mut ::core::ffi::c_void, (*FleetPtr).growth), ());
        uqm_lua_checked!(write_8(fh as *mut ::core::ffi::c_void, (*FleetPtr).max_energy), ());
        uqm_lua_checked!(write_16(fh as *mut ::core::ffi::c_void, (*FleetPtr).loc.x as UWORD), ());
        uqm_lua_checked!(write_16(fh as *mut ::core::ffi::c_void, (*FleetPtr).loc.y as UWORD), ());
        uqm_lua_checked!(write_16(fh as *mut ::core::ffi::c_void, (*FleetPtr).actual_strength), ());
        uqm_lua_checked!(write_16(fh as *mut ::core::ffi::c_void, (*FleetPtr).known_strength), ());
        uqm_lua_checked!(write_16(fh as *mut ::core::ffi::c_void, (*FleetPtr).known_loc.x as UWORD), ());
        uqm_lua_checked!(write_16(fh as *mut ::core::ffi::c_void, (*FleetPtr).known_loc.y as UWORD), ());
        uqm_lua_checked!(write_8(fh as *mut ::core::ffi::c_void, (*FleetPtr).growth_err_term), ());
        uqm_lua_checked!(write_8(fh as *mut ::core::ffi::c_void, (*FleetPtr).func_index), ());
        uqm_lua_checked!(write_16(fh as *mut ::core::ffi::c_void, (*FleetPtr).dest_loc.x as UWORD), ());
        uqm_lua_checked!(write_16(fh as *mut ::core::ffi::c_void, (*FleetPtr).dest_loc.y as UWORD), ());
        UnlockLink(pQueue, hFleet);
        hFleet = hNextFleet;
    };
}
unsafe extern "C" fn SaveGroupQueue(mut fh: *mut uio_Stream, mut pQueue: *mut QUEUE) {
    let mut hGroup: HIPGROUP = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut hNextGroup: HIPGROUP = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut num_links: COUNT = 0;
    num_links = CountLinks(pQueue);
    if num_links as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        return;
    }
    uqm_lua_checked!(write_32(fh as *mut ::core::ffi::c_void, 0x51704749 as ::core::ffi::c_int as DWORD), ());
    uqm_lua_checked!(write_32(
        fh as *mut ::core::ffi::c_void,
        (num_links as ::core::ffi::c_int * 13 as ::core::ffi::c_int) as DWORD,
    ), ());
    hGroup = (*pQueue).head as HIPGROUP;
    while !hGroup.is_null() {
        let mut GroupPtr: *mut IP_GROUP = ::core::ptr::null_mut::<IP_GROUP>();
        GroupPtr = LockIpGroup(pQueue, hGroup);
        hNextGroup = (*GroupPtr).succ;
        uqm_lua_checked!(write_16(fh as *mut ::core::ffi::c_void, (*GroupPtr).group_counter), ());
        uqm_lua_checked!(write_8(fh as *mut ::core::ffi::c_void, (*GroupPtr).race_id), ());
        uqm_lua_checked!(write_8(fh as *mut ::core::ffi::c_void, (*GroupPtr).sys_loc), ());
        uqm_lua_checked!(write_8(fh as *mut ::core::ffi::c_void, (*GroupPtr).task), ());
        uqm_lua_checked!(write_8(fh as *mut ::core::ffi::c_void, (*GroupPtr).in_system), ());
        uqm_lua_checked!(write_8(fh as *mut ::core::ffi::c_void, (*GroupPtr).dest_loc), ());
        uqm_lua_checked!(write_8(fh as *mut ::core::ffi::c_void, (*GroupPtr).orbit_pos), ());
        uqm_lua_checked!(write_8(fh as *mut ::core::ffi::c_void, (*GroupPtr).group_id), ());
        uqm_lua_checked!(write_16(fh as *mut ::core::ffi::c_void, (*GroupPtr).loc.x as UWORD), ());
        uqm_lua_checked!(write_16(fh as *mut ::core::ffi::c_void, (*GroupPtr).loc.y as UWORD), ());
        UnlockLink(pQueue, hGroup);
        hGroup = hNextGroup;
    }
}
unsafe extern "C" fn SaveEncounters(mut fh: *mut uio_Stream) {
    let mut num_links: COUNT = 0;
    let mut hEncounter: HENCOUNTER = ::core::ptr::null_mut::<::core::ffi::c_void>();
    num_links = CountLinks(&raw mut GlobData.Game_state.encounter_q);
    if num_links as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        return;
    }
    uqm_lua_checked!(write_32(fh as *mut ::core::ffi::c_void, 0x74636e45 as ::core::ffi::c_int as DWORD), ());
    uqm_lua_checked!(write_32(
        fh as *mut ::core::ffi::c_void,
        (65 as ::core::ffi::c_int * num_links as ::core::ffi::c_int) as DWORD,
    ), ());
    hEncounter = GlobData.Game_state.encounter_q.head as HENCOUNTER;
    loop {
        let c2rust_fresh0 = num_links;
        num_links = num_links.wrapping_sub(1);
        if c2rust_fresh0 == 0 {
            break;
        }
        let mut hNextEncounter: HENCOUNTER = ::core::ptr::null_mut::<
            ::core::ffi::c_void,
        >();
        let mut EncounterPtr: *mut ENCOUNTER = ::core::ptr::null_mut::<ENCOUNTER>();
        let mut i: COUNT = 0;
        EncounterPtr = LockLink(&raw mut GlobData.Game_state.encounter_q, hEncounter)
            as *mut ENCOUNTER;
        hNextEncounter = (*EncounterPtr).succ;
        uqm_lua_checked!(write_16(
            fh as *mut ::core::ffi::c_void,
            (*EncounterPtr).transition_state as UWORD,
        ), ());
        uqm_lua_checked!(write_16(fh as *mut ::core::ffi::c_void, (*EncounterPtr).origin.x as UWORD), ());
        uqm_lua_checked!(write_16(fh as *mut ::core::ffi::c_void, (*EncounterPtr).origin.y as UWORD), ());
        uqm_lua_checked!(write_16(fh as *mut ::core::ffi::c_void, (*EncounterPtr).radius), ());
        uqm_lua_checked!(write_16(fh as *mut ::core::ffi::c_void, (*EncounterPtr).loc_pt.x as UWORD), ());
        uqm_lua_checked!(write_16(fh as *mut ::core::ffi::c_void, (*EncounterPtr).loc_pt.y as UWORD), ());
        uqm_lua_checked!(write_8(fh as *mut ::core::ffi::c_void, (*EncounterPtr).race_id), ());
        uqm_lua_checked!(write_8(fh as *mut ::core::ffi::c_void, (*EncounterPtr).num_ships), ());
        uqm_lua_checked!(write_8(fh as *mut ::core::ffi::c_void, (*EncounterPtr).flags), ());
        i = 0 as COUNT;
        while (i as ::core::ffi::c_int) < 7 as ::core::ffi::c_int {
            let mut ShipInfo: *const BRIEF_SHIP_INFO = (&raw mut (*EncounterPtr).ShipList
                as *mut BRIEF_SHIP_INFO)
                .offset(i as isize);
            uqm_lua_checked!(write_8(fh as *mut ::core::ffi::c_void, (*ShipInfo).race_id), ());
            uqm_lua_checked!(write_16(fh as *mut ::core::ffi::c_void, (*ShipInfo).crew_level), ());
            uqm_lua_checked!(write_16(fh as *mut ::core::ffi::c_void, (*ShipInfo).max_crew), ());
            uqm_lua_checked!(write_8(fh as *mut ::core::ffi::c_void, (*ShipInfo).max_energy), ());
            i = i.wrapping_add(1);
        }
        uqm_lua_checked!(write_32(
            fh as *mut ::core::ffi::c_void,
            ((*EncounterPtr).log_x >> resolutionFactor) as DWORD,
        ), ());
        uqm_lua_checked!(write_32(
            fh as *mut ::core::ffi::c_void,
            ((*EncounterPtr).log_y >> resolutionFactor) as DWORD,
        ), ());
        UnlockLink(&raw mut GlobData.Game_state.encounter_q, hEncounter);
        hEncounter = hNextEncounter;
    };
}
unsafe extern "C" fn SaveEvents(mut fh: *mut uio_Stream) {
    let mut num_links: COUNT = 0;
    let mut hEvent: HEVENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
    num_links = CountLinks(&raw mut GlobData.Game_state.GameClock.event_q);
    if num_links as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        return;
    }
    uqm_lua_checked!(write_32(fh as *mut ::core::ffi::c_void, 0x73747645 as ::core::ffi::c_int as DWORD), ());
    uqm_lua_checked!(write_32(
        fh as *mut ::core::ffi::c_void,
        (num_links as ::core::ffi::c_int * 5 as ::core::ffi::c_int) as DWORD,
    ), ());
    hEvent = GlobData.Game_state.GameClock.event_q.head as HEVENT;
    loop {
        let c2rust_fresh1 = num_links;
        num_links = num_links.wrapping_sub(1);
        if c2rust_fresh1 == 0 {
            break;
        }
        let mut hNextEvent: HEVENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
        let mut EventPtr: *mut EVENT = ::core::ptr::null_mut::<EVENT>();
        EventPtr = LockLink(&raw mut GlobData.Game_state.GameClock.event_q, hEvent)
            as *mut EVENT;
        hNextEvent = (*EventPtr).succ;
        uqm_lua_checked!(write_8(fh as *mut ::core::ffi::c_void, (*EventPtr).day_index), ());
        uqm_lua_checked!(write_8(fh as *mut ::core::ffi::c_void, (*EventPtr).month_index), ());
        uqm_lua_checked!(write_16(fh as *mut ::core::ffi::c_void, (*EventPtr).year_index), ());
        uqm_lua_checked!(write_8(fh as *mut ::core::ffi::c_void, (*EventPtr).func_index), ());
        UnlockLink(&raw mut GlobData.Game_state.GameClock.event_q, hEvent);
        hEvent = hNextEvent;
    };
}
unsafe extern "C" fn SaveClockState(
    mut ClockPtr: *const CLOCK_STATE,
    mut fh: *mut uio_Stream,
) {
    uqm_lua_checked!(write_8(fh as *mut ::core::ffi::c_void, (*ClockPtr).day_index), ());
    uqm_lua_checked!(write_8(fh as *mut ::core::ffi::c_void, (*ClockPtr).month_index), ());
    uqm_lua_checked!(write_16(fh as *mut ::core::ffi::c_void, (*ClockPtr).year_index), ());
    uqm_lua_checked!(write_16(fh as *mut ::core::ffi::c_void, (*ClockPtr).tick_count as UWORD), ());
    uqm_lua_checked!(write_16(fh as *mut ::core::ffi::c_void, (*ClockPtr).day_in_ticks as UWORD), ());
}
unsafe extern "C" fn SaveGameState(
    mut GSPtr: *const GAME_STATE,
    mut fh: *mut uio_Stream,
) -> BOOLEAN {
    let mut res_scale: BYTE = 0;
    if (*GSPtr).CurrentActivity as BYTE as ::core::ffi::c_int
        != C2Rust_Unnamed_1::IN_INTERPLANETARY.0 as ::core::ffi::c_int
    {
        res_scale = resolutionFactor as BYTE;
    } else {
        res_scale = 0 as BYTE;
    }
    uqm_lua_checked!(write_32(fh as *mut ::core::ffi::c_void, 0x74536c47 as ::core::ffi::c_int as DWORD), BOOLEAN(0));
    uqm_lua_checked!(write_32(fh as *mut ::core::ffi::c_void, 75 as DWORD), BOOLEAN(0));
    uqm_lua_checked!(write_8(fh as *mut ::core::ffi::c_void, (*GSPtr).glob_flags), BOOLEAN(0));
    uqm_lua_checked!(write_8(fh as *mut ::core::ffi::c_void, (*GSPtr).CrewCost), BOOLEAN(0));
    uqm_lua_checked!(write_8(fh as *mut ::core::ffi::c_void, (*GSPtr).FuelCost), BOOLEAN(0));
    uqm_lua_checked!(write_a8(
        fh as *mut ::core::ffi::c_void,
        &raw const (*GSPtr).ModuleCost as *const BYTE,
        C2Rust_Unnamed_0::NUM_MODULES.0 as ::core::ffi::c_int as COUNT,
    ), BOOLEAN(0));
    uqm_lua_checked!(write_a8(
        fh as *mut ::core::ffi::c_void,
        &raw const (*GSPtr).ElementWorth as *const BYTE,
        C2Rust_Unnamed::NUM_ELEMENT_CATEGORIES.0 as ::core::ffi::c_int as COUNT,
    ), BOOLEAN(0));
    uqm_lua_checked!(write_16(fh as *mut ::core::ffi::c_void, (*GSPtr).CurrentActivity), BOOLEAN(0));
    uqm_lua_checked!(SaveClockState(&raw const (*GSPtr).GameClock, fh), BOOLEAN(0));
    uqm_lua_checked!(write_16(fh as *mut ::core::ffi::c_void, (*GSPtr).autopilot.x as UWORD), BOOLEAN(0));
    uqm_lua_checked!(write_16(fh as *mut ::core::ffi::c_void, (*GSPtr).autopilot.y as UWORD), BOOLEAN(0));
    uqm_lua_checked!(write_16(fh as *mut ::core::ffi::c_void, (*GSPtr).ip_location.x as UWORD), BOOLEAN(0));
    uqm_lua_checked!(write_16(fh as *mut ::core::ffi::c_void, (*GSPtr).ip_location.y as UWORD), BOOLEAN(0));
    uqm_lua_checked!(write_16(
        fh as *mut ::core::ffi::c_void,
        ((*GSPtr).ShipStamp.origin.x as ::core::ffi::c_int >> resolutionFactor) as UWORD,
    ), BOOLEAN(0));
    uqm_lua_checked!(write_16(
        fh as *mut ::core::ffi::c_void,
        ((*GSPtr).ShipStamp.origin.y as ::core::ffi::c_int >> resolutionFactor) as UWORD,
    ), BOOLEAN(0));
    uqm_lua_checked!(write_16(fh as *mut ::core::ffi::c_void, (*GSPtr).ShipFacing), BOOLEAN(0));
    uqm_lua_checked!(write_8(fh as *mut ::core::ffi::c_void, (*GSPtr).ip_planet), BOOLEAN(0));
    uqm_lua_checked!(write_8(fh as *mut ::core::ffi::c_void, (*GSPtr).in_orbit), BOOLEAN(0));
    uqm_lua_checked!(write_16(
        fh as *mut ::core::ffi::c_void,
        ((*GSPtr).velocity.TravelAngle as ::core::ffi::c_int
            >> res_scale as ::core::ffi::c_int) as UWORD,
    ), BOOLEAN(0));
    uqm_lua_checked!(write_16(
        fh as *mut ::core::ffi::c_void,
        ((*GSPtr).velocity.vector.width as ::core::ffi::c_int
            >> res_scale as ::core::ffi::c_int) as UWORD,
    ), BOOLEAN(0));
    uqm_lua_checked!(write_16(
        fh as *mut ::core::ffi::c_void,
        ((*GSPtr).velocity.vector.height as ::core::ffi::c_int
            >> res_scale as ::core::ffi::c_int) as UWORD,
    ), BOOLEAN(0));
    uqm_lua_checked!(write_16(
        fh as *mut ::core::ffi::c_void,
        ((*GSPtr).velocity.fract.width as ::core::ffi::c_int
            >> res_scale as ::core::ffi::c_int) as UWORD,
    ), BOOLEAN(0));
    uqm_lua_checked!(write_16(
        fh as *mut ::core::ffi::c_void,
        ((*GSPtr).velocity.fract.height as ::core::ffi::c_int
            >> res_scale as ::core::ffi::c_int) as UWORD,
    ), BOOLEAN(0));
    uqm_lua_checked!(write_16(
        fh as *mut ::core::ffi::c_void,
        ((*GSPtr).velocity.error.width as ::core::ffi::c_int
            >> res_scale as ::core::ffi::c_int) as UWORD,
    ), BOOLEAN(0));
    uqm_lua_checked!(write_16(
        fh as *mut ::core::ffi::c_void,
        ((*GSPtr).velocity.error.height as ::core::ffi::c_int
            >> res_scale as ::core::ffi::c_int) as UWORD,
    ), BOOLEAN(0));
    uqm_lua_checked!(write_16(
        fh as *mut ::core::ffi::c_void,
        ((*GSPtr).velocity.incr.width as ::core::ffi::c_int
            >> res_scale as ::core::ffi::c_int) as UWORD,
    ), BOOLEAN(0));
    uqm_lua_checked!(write_16(
        fh as *mut ::core::ffi::c_void,
        ((*GSPtr).velocity.incr.height as ::core::ffi::c_int
            >> res_scale as ::core::ffi::c_int) as UWORD,
    ), BOOLEAN(0));
    uqm_lua_checked!(write_32(fh as *mut ::core::ffi::c_void, 0x74536d47 as ::core::ffi::c_int as DWORD), BOOLEAN(0));
    let mut buf: *mut uint8 = ::core::ptr::null_mut::<uint8>();
    let mut bufSize: size_t = 0;
    if uqm_lua_checked!(serialiseGameState(
            &raw const gameStateBitMap as *const GameStateBitMap,
            &raw mut buf,
            &raw mut bufSize,
        ), BOOLEAN(0))
        .0 != 0
    {
        uqm_lua_checked!(write_32(fh as *mut ::core::ffi::c_void, bufSize as DWORD), BOOLEAN(0));
        uqm_lua_checked!(write_a8(fh as *mut ::core::ffi::c_void, buf, bufSize as COUNT), BOOLEAN(0));
        HFree(buf as *mut ::core::ffi::c_void);
    } else {
        return BOOLEAN::FALSE
    }
    return BOOLEAN::TRUE;
}
unsafe extern "C" fn SaveSisState(
    mut SSPtr: *const SIS_STATE,
    mut fp: *mut ::core::ffi::c_void,
) {
    uqm_lua_checked!(write_32(fp, ((*SSPtr).log_x >> resolutionFactor) as DWORD), ());
    uqm_lua_checked!(write_32(fp, ((*SSPtr).log_y >> resolutionFactor) as DWORD), ());
    uqm_lua_checked!(write_32(fp, (*SSPtr).ResUnits), ());
    uqm_lua_checked!(write_32(fp, (*SSPtr).FuelOnBoard), ());
    uqm_lua_checked!(write_16(fp, (*SSPtr).CrewEnlisted), ());
    uqm_lua_checked!(write_16(fp, (*SSPtr).TotalElementMass), ());
    uqm_lua_checked!(write_16(fp, (*SSPtr).TotalBioMass), ());
    uqm_lua_checked!(write_a8(fp, &raw const (*SSPtr).ModuleSlots as *const BYTE, 16 as COUNT), ());
    uqm_lua_checked!(write_a8(fp, &raw const (*SSPtr).DriveSlots as *const BYTE, 11 as COUNT), ());
    uqm_lua_checked!(write_a8(fp, &raw const (*SSPtr).JetSlots as *const BYTE, 8 as COUNT), ());
    uqm_lua_checked!(write_8(fp, (*SSPtr).NumLanders), ());
    uqm_lua_checked!(write_a16(
        fp,
        &raw const (*SSPtr).ElementAmounts as *const UWORD,
        C2Rust_Unnamed::NUM_ELEMENT_CATEGORIES.0 as ::core::ffi::c_int as COUNT,
    ), ());
    uqm_lua_checked!(write_str(
        fp,
        &raw const (*SSPtr).ShipName as *const ::core::ffi::c_char,
        32 as COUNT,
    ), ());
    uqm_lua_checked!(write_str(
        fp,
        &raw const (*SSPtr).CommanderName as *const ::core::ffi::c_char,
        32 as COUNT,
    ), ());
    uqm_lua_checked!(write_str(
        fp,
        &raw const (*SSPtr).PlanetName as *const ::core::ffi::c_char,
        32 as COUNT,
    ), ());
    uqm_lua_checked!(write_8(fp, (*SSPtr).Difficulty), ());
    uqm_lua_checked!(write_8(fp, (*SSPtr).Extended), ());
    uqm_lua_checked!(write_8(fp, (*SSPtr).Nomad), ());
    uqm_lua_checked!(write_32(fp, (*SSPtr).Seed as DWORD), ());
    uqm_lua_checked!(write_8(fp, (*SSPtr).ShipSeed), ());
}
unsafe extern "C" fn SaveSummary(
    mut SummPtr: *const SUMMARY_DESC,
    mut fp: *mut ::core::ffi::c_void,
) {
    uqm_lua_checked!(write_32(fp, 0x6d6d7553 as ::core::ffi::c_int as DWORD), ());
    uqm_lua_checked!(write_32(
        fp,
        (160 as size_t)
            .wrapping_add(
                strlen(&raw const (*SummPtr).SaveName as *const ::core::ffi::c_char),
            ) as DWORD,
    ), ());
    uqm_lua_checked!(SaveSisState(&raw const (*SummPtr).SS, fp), ());
    uqm_lua_checked!(write_8(fp, (*SummPtr).Activity), ());
    uqm_lua_checked!(write_8(fp, (*SummPtr).Flags), ());
    uqm_lua_checked!(write_8(fp, (*SummPtr).day_index), ());
    uqm_lua_checked!(write_8(fp, (*SummPtr).month_index), ());
    uqm_lua_checked!(write_16(fp, (*SummPtr).year_index), ());
    uqm_lua_checked!(write_8(fp, (*SummPtr).MCreditLo), ());
    uqm_lua_checked!(write_8(fp, (*SummPtr).MCreditHi), ());
    uqm_lua_checked!(write_8(fp, (*SummPtr).NumShips), ());
    uqm_lua_checked!(write_8(fp, (*SummPtr).NumDevices), ());
    uqm_lua_checked!(write_a8(fp, &raw const (*SummPtr).ShipList as *const BYTE, 12 as COUNT), ());
    uqm_lua_checked!(write_a8(fp, &raw const (*SummPtr).DeviceList as *const BYTE, 16 as COUNT), ());
    uqm_lua_checked!(write_8(fp, (*SummPtr).res_factor), ());
    uqm_lua_checked!(write_a8(
        fp,
        &raw const (*SummPtr).SaveName as *const UNICODE as *mut BYTE,
        strlen(&raw const (*SummPtr).SaveName as *const ::core::ffi::c_char) as COUNT,
    ), ());
}
unsafe extern "C" fn SaveStarDesc(mut SDPtr: *const STAR_DESC, mut fh: *mut uio_Stream) {
    uqm_lua_checked!(write_32(fh as *mut ::core::ffi::c_void, 0x72617453 as ::core::ffi::c_int as DWORD), ());
    uqm_lua_checked!(write_32(fh as *mut ::core::ffi::c_void, 8 as DWORD), ());
    uqm_lua_checked!(write_16(fh as *mut ::core::ffi::c_void, (*SDPtr).star_pt.x as UWORD), ());
    uqm_lua_checked!(write_16(fh as *mut ::core::ffi::c_void, (*SDPtr).star_pt.y as UWORD), ());
    uqm_lua_checked!(write_8(fh as *mut ::core::ffi::c_void, (*SDPtr).Type), ());
    uqm_lua_checked!(write_8(fh as *mut ::core::ffi::c_void, (*SDPtr).Index), ());
    uqm_lua_checked!(write_8(fh as *mut ::core::ffi::c_void, (*SDPtr).Prefix), ());
    uqm_lua_checked!(write_8(fh as *mut ::core::ffi::c_void, (*SDPtr).Postfix), ());
}
unsafe extern "C" fn PrepareSummary(
    mut SummPtr: *mut SUMMARY_DESC,
    mut name: *const ::core::ffi::c_char,
) {
    (*SummPtr).SS = GlobData.SIS_state;
    (*SummPtr).Activity = GlobData.Game_state.CurrentActivity as BYTE;
    match (*SummPtr).Activity as ::core::ffi::c_int {
        3 => {
            if uqm_lua_checked!(inQuasiSpace(), ()).0 != 0 {
                (*SummPtr).Activity = C2Rust_Unnamed_1::IN_QUASISPACE.0
                    as ::core::ffi::c_int as BYTE;
            }
        }
        4 => {
            uqm_lua_checked!(GetPlanetOrMoonName(
                &raw mut (*SummPtr).SS.PlanetName as *mut UNICODE,
                ::core::mem::size_of::<[UNICODE; 32]>() as COUNT,
            ), ());
            if uqm_lua_checked!(getGameStateUint(
                b"GLOBAL_FLAGS_AND_DATA\0".as_ptr() as *const ::core::ffi::c_char,
            ), ()) == !(0 as ::core::ffi::c_int) as BYTE as DWORD
            {
                (*SummPtr).Activity = C2Rust_Unnamed_1::IN_STARBASE.0
                    as ::core::ffi::c_int as BYTE;
            } else if playerInPlanetOrbit() {
                (*SummPtr).Activity = C2Rust_Unnamed_1::IN_PLANET_ORBIT.0
                    as ::core::ffi::c_int as BYTE;
            }
        }
        1 => {
            utf8StringCopy(
                &raw mut (*SummPtr).SS.PlanetName as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[UNICODE; 32]>(),
                GetStringAddress(
                    SetAbsStringTableIndex(
                        GameStrings,
                        (C2Rust_Unnamed_2::PLANET_NUMBER_BASE.0 as ::core::ffi::c_int
                            + 32 as ::core::ffi::c_int) as COUNT,
                    ),
                ),
            );
        }
        _ => {}
    }
    (*SummPtr).MCreditLo = uqm_lua_checked!(getGameStateUint(
        b"MELNORME_CREDIT0\0".as_ptr() as *const ::core::ffi::c_char,
    ), ()) as BYTE;
    (*SummPtr).MCreditHi = uqm_lua_checked!(getGameStateUint(
        b"MELNORME_CREDIT1\0".as_ptr() as *const ::core::ffi::c_char,
    ), ()) as BYTE;
    let mut hStarShip: HSHIPFRAG = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut hNextShip: HSHIPFRAG = ::core::ptr::null_mut::<::core::ffi::c_void>();
    hStarShip = GlobData.Game_state.built_ship_q.head as HSHIPFRAG;
    (*SummPtr).NumShips = 0 as BYTE;
    while !hStarShip.is_null() {
        let mut StarShipPtr: *mut SHIP_FRAGMENT = ::core::ptr::null_mut::<
            SHIP_FRAGMENT,
        >();
        StarShipPtr = LockShipFrag(&raw mut GlobData.Game_state.built_ship_q, hStarShip);
        hNextShip = (*StarShipPtr).succ as HSHIPFRAG;
        (*SummPtr).ShipList[(*SummPtr).NumShips as usize] = (*StarShipPtr).race_id;
        UnlockLink(&raw mut GlobData.Game_state.built_ship_q, hStarShip);
        hStarShip = hNextShip;
        (*SummPtr).NumShips = (*SummPtr).NumShips.wrapping_add(1);
    }
    (*SummPtr).NumDevices = uqm_lua_checked!(InventoryDevices(
        &raw mut (*SummPtr).DeviceList as *mut BYTE,
        16 as COUNT,
    ), ()) as BYTE;
    (*SummPtr).Flags = (uqm_lua_checked!(getGameStateUint(
        b"LANDER_SHIELDS\0".as_ptr() as *const ::core::ffi::c_char,
    ), ())
        | uqm_lua_checked!(getGameStateUint(
            b"IMPROVED_LANDER_SPEED\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) << 4 as ::core::ffi::c_int + 0 as ::core::ffi::c_int
        | uqm_lua_checked!(getGameStateUint(
            b"IMPROVED_LANDER_CARGO\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) << 4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
        | uqm_lua_checked!(getGameStateUint(
            b"IMPROVED_LANDER_SHOT\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) << 4 as ::core::ffi::c_int + 2 as ::core::ffi::c_int
        | (if uqm_lua_checked!(getGameStateUint(
            b"CHMMR_BOMB_STATE\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) < 2 as DWORD
        {
            0 as ::core::ffi::c_int
        } else {
            1 as ::core::ffi::c_int
        } << 4 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as DWORD) as BYTE;
    (*SummPtr).day_index = GlobData.Game_state.GameClock.day_index;
    (*SummPtr).month_index = GlobData.Game_state.GameClock.month_index;
    (*SummPtr).year_index = GlobData.Game_state.GameClock.year_index;
    (*SummPtr).SaveName[(64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as usize] = 0
        as UNICODE;
    strncpy(
        &raw mut (*SummPtr).SaveName as *mut ::core::ffi::c_char,
        name,
        (64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as size_t,
    );
    (*SummPtr).res_factor = resolutionFactor as BYTE;
}
unsafe extern "C" fn SaveProblemMessage(mut MsgStamp: *mut STAMP) {
    let mut r: RECT = rect {
        corner: point {
            x: 0 as COORD,
            y: 0 as COORD,
        },
        extent: extent {
            width: 0 as COORD,
            height: 0 as COORD,
        },
    };
    let mut i: COUNT = 0;
    let mut t: TEXT = TEXT {
        baseline: POINT { x: 0, y: 0 },
        pStr: ::core::ptr::null::<UNICODE>(),
        align: TEXT_ALIGN::ALIGN_LEFT,
        CharCount: 0,
    };
    let mut ppStr: [*mut UNICODE; 1] = [::core::ptr::null_mut::<UNICODE>(); 1];
    ppStr[0usize] = GetStringAddress(
        SetAbsStringTableIndex(
            GameStrings,
            (C2Rust_Unnamed_2::SAVEGAME_STRING_BASE.0 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int) as COUNT,
        ),
    );
    SetContextFont(StarConFont);
    t.baseline.y = 0 as COORD;
    t.baseline.x = t.baseline.y;
    t.align = TEXT_ALIGN::ALIGN_CENTER;
    i = 0 as COUNT;
    while (i as ::core::ffi::c_int) < 1 as ::core::ffi::c_int {
        let mut tr: RECT = RECT {
            corner: POINT { x: 0, y: 0 },
            extent: EXTENT { width: 0, height: 0 },
        };
        t.pStr = ppStr[i as usize];
        if *t.pStr as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
            break;
        }
        t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
        TextRect(&raw mut t, &raw mut tr, ::core::ptr::null_mut::<BYTE>());
        if i as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            r = tr;
        } else {
            BoxUnion(&raw mut tr, &raw mut r, &raw mut r);
        }
        t.baseline.y = (t.baseline.y as ::core::ffi::c_int + 11 as ::core::ffi::c_int)
            as COORD;
        i = i.wrapping_add(1);
    }
    t.baseline.x = (((CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
        << resolutionFactor)
        - (r.extent.width as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
        - r.corner.x as ::core::ffi::c_int) as COORD;
    t.baseline.y = (((CanvasHeight
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
        - (r.extent.height as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
        - r.corner.y as ::core::ffi::c_int) as COORD;
    r.corner.x = (r.corner.x as ::core::ffi::c_int
        + (t.baseline.x as ::core::ffi::c_int
            - ((4 as ::core::ffi::c_int) << resolutionFactor))) as COORD;
    r.corner.y = (r.corner.y as ::core::ffi::c_int
        + (t.baseline.y as ::core::ffi::c_int
            - ((4 as ::core::ffi::c_int) << resolutionFactor))) as COORD;
    r.extent.width = (r.extent.width as ::core::ffi::c_int
        + ((8 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    r.extent.height = (r.extent.height as ::core::ffi::c_int
        + ((8 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    *MsgStamp = SaveContextFrame(&raw mut r);
    BatchGraphics();
    uqm_lua_checked!(DrawStarConBox(
        &raw mut r,
        ((2 as ::core::ffi::c_int) << resolutionFactor) as SIZE,
        colorFromRgb15(0x10 as BYTE, 0x10 as BYTE, 0x10 as BYTE),
        colorFromRgb15(0x8 as BYTE, 0x8 as BYTE, 0x8 as BYTE),
        BOOLEAN::TRUE,
        colorFromRgb15(0xa as BYTE, 0xa as BYTE, 0xa as BYTE),
        BOOLEAN::TRUE,
        buildColorRgba(0x50 as BYTE, 0x50 as BYTE, 0x50 as BYTE, 0 as BYTE),
    ), ());
    SetContextForeGroundColor(
        if isPC(optWhichFonts).0 != 0 {
            colorFromRgb15(0x1f as BYTE, 0x1f as BYTE, 0x1f as BYTE)
        } else {
            colorFromRgb15(0x14 as BYTE, 0x14 as BYTE, 0x14 as BYTE)
        },
    );
    i = 0 as COUNT;
    while (i as ::core::ffi::c_int) < 1 as ::core::ffi::c_int {
        t.pStr = ppStr[i as usize];
        if *t.pStr as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
            break;
        }
        t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
        uqm_lua_checked!(font_DrawText(&raw mut t), ());
        t.baseline.y = (t.baseline.y as ::core::ffi::c_int + 11 as ::core::ffi::c_int)
            as COORD;
        i = i.wrapping_add(1);
    }
    UnbatchGraphics();
}
#[no_mangle]
pub unsafe extern "C" fn SaveProblem() {
    let mut s: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    let mut OldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    OldContext = SetContext(SpaceContext);
    uqm_lua_checked!(SaveProblemMessage(&raw mut s), ());
    FlushGraphics();
    uqm_lua_checked!(WaitForAnyButton(
        BOOLEAN::TRUE,
        -1 as ::core::ffi::c_int as TimePeriod,
        BOOLEAN::FALSE,
    ), ());
    uqm_lua_checked!(DrawStamp(&raw mut s), ());
    SetContext(OldContext);
    DestroyDrawable(ReleaseDrawable(s.frame));
}
unsafe extern "C" fn SaveFlagshipState() {
    if inHQSpace().0 != 0 {
        SaveSisHyperState();
    } else if playerInSolarSystem() {
        uqm_lua_checked!(SaveSolarSysLocation(), ());
    }
}
unsafe extern "C" fn SaveStarInfo(mut fh: *mut uio_Stream) {
    let mut fp: *mut GAME_STATE_FILE = ::core::ptr::null_mut::<GAME_STATE_FILE>();
    fp = OpenStateFile(
        0 as ::core::ffi::c_int,
        b"rb\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !fp.is_null() {
        let mut flen: DWORD = LengthStateFile(fp);
        if flen.wrapping_rem(4 as DWORD) != 0 {
            log_add(
                log_Level::log_Warning,
                b"Unexpected Star Info length! Expected an integral number of DWORDS.\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        } else {
            uqm_lua_checked!(write_32(
                fh as *mut ::core::ffi::c_void,
                0x6e616353 as ::core::ffi::c_int as DWORD,
            ), ());
            uqm_lua_checked!(write_32(fh as *mut ::core::ffi::c_void, flen), ());
            while flen != 0 {
                let mut val: DWORD = 0;
                sread_32(fp, &raw mut val);
                uqm_lua_checked!(write_32(fh as *mut ::core::ffi::c_void, val), ());
                flen = flen.wrapping_sub(4 as DWORD);
            }
        }
        CloseStateFile(fp);
    }
}
unsafe extern "C" fn SaveBattleGroup(
    mut fp: *mut GAME_STATE_FILE,
    mut encounter_id: DWORD,
    mut grpoffs: DWORD,
    mut fh: *mut uio_Stream,
) {
    let mut h: GROUP_HEADER = GROUP_HEADER {
        NumGroups: 0,
        day_index: 0,
        month_index: 0,
        star_index: 0,
        year_index: 0,
        GroupOffset: [0; 65],
    };
    let mut size: DWORD = 12 as DWORD;
    let mut i: ::core::ffi::c_int = 0;
    SeekStateFile(fp, grpoffs as ::core::ffi::c_long, 0 as ::core::ffi::c_int);
    ReadGroupHeader(fp, &raw mut h);
    i = 1 as ::core::ffi::c_int;
    while i <= h.NumGroups as ::core::ffi::c_int {
        let mut NumShips: BYTE = 0;
        SeekStateFile(
            fp,
            h.GroupOffset[i as usize] as ::core::ffi::c_long,
            0 as ::core::ffi::c_int,
        );
        sread_8(fp, ::core::ptr::null_mut::<BYTE>());
        sread_8(fp, &raw mut NumShips);
        size = size
            .wrapping_add(
                (2 as ::core::ffi::c_int
                    + 10 as ::core::ffi::c_int * NumShips as ::core::ffi::c_int) as DWORD,
            );
        i += 1;
    }
    uqm_lua_checked!(write_32(fh as *mut ::core::ffi::c_void, 0x70477442 as ::core::ffi::c_int as DWORD), ());
    uqm_lua_checked!(write_32(fh as *mut ::core::ffi::c_void, size), ());
    uqm_lua_checked!(write_32(fh as *mut ::core::ffi::c_void, encounter_id), ());
    uqm_lua_checked!(write_8(
        fh as *mut ::core::ffi::c_void,
        (if grpoffs != 0 && GlobData.Game_state.BattleGroupRef == grpoffs {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as BYTE,
    ), ());
    uqm_lua_checked!(write_16(fh as *mut ::core::ffi::c_void, h.star_index), ());
    uqm_lua_checked!(write_8(fh as *mut ::core::ffi::c_void, h.day_index), ());
    uqm_lua_checked!(write_8(fh as *mut ::core::ffi::c_void, h.month_index), ());
    uqm_lua_checked!(write_16(fh as *mut ::core::ffi::c_void, h.year_index), ());
    uqm_lua_checked!(write_8(fh as *mut ::core::ffi::c_void, h.NumGroups), ());
    i = 1 as ::core::ffi::c_int;
    while i <= h.NumGroups as ::core::ffi::c_int {
        let mut j: ::core::ffi::c_int = 0;
        let mut b: BYTE = 0;
        SeekStateFile(
            fp,
            h.GroupOffset[i as usize] as ::core::ffi::c_long,
            0 as ::core::ffi::c_int,
        );
        sread_8(fp, &raw mut b);
        uqm_lua_checked!(write_8(fh as *mut ::core::ffi::c_void, b), ());
        sread_8(fp, &raw mut b);
        uqm_lua_checked!(write_8(fh as *mut ::core::ffi::c_void, b), ());
        j = 0 as ::core::ffi::c_int;
        while j < b as ::core::ffi::c_int {
            let mut race_outer: BYTE = 0;
            let mut sf: SHIP_FRAGMENT = SHIP_FRAGMENT {
                pred: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                succ: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                SpeciesID: SPECIES_ID::NO_ID,
                captains_name_index: 0,
                race_id: 0,
                index: 0,
                crew_level: 0,
                max_crew: 0,
                energy_level: 0,
                max_energy: 0,
                race_strings: ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>(),
                icons: ::core::ptr::null_mut::<FRAME_DESC>(),
                melee_icon: ::core::ptr::null_mut::<FRAME_DESC>(),
            };
            sread_8(fp, &raw mut race_outer);
            ReadShipFragment(fp, &raw mut sf);
            uqm_lua_checked!(write_8(fh as *mut ::core::ffi::c_void, race_outer), ());
            uqm_lua_checked!(write_8(fh as *mut ::core::ffi::c_void, sf.captains_name_index), ());
            uqm_lua_checked!(write_8(fh as *mut ::core::ffi::c_void, sf.race_id), ());
            uqm_lua_checked!(write_8(fh as *mut ::core::ffi::c_void, sf.index), ());
            uqm_lua_checked!(write_16(fh as *mut ::core::ffi::c_void, sf.crew_level), ());
            uqm_lua_checked!(write_16(fh as *mut ::core::ffi::c_void, sf.max_crew), ());
            uqm_lua_checked!(write_8(fh as *mut ::core::ffi::c_void, sf.energy_level), ());
            uqm_lua_checked!(write_8(fh as *mut ::core::ffi::c_void, sf.max_energy), ());
            j += 1;
        }
        i += 1;
    }
}
unsafe extern "C" fn GetBattleGroupOffset(
    mut encounterIndex: ::core::ffi::c_int,
) -> DWORD {
    match encounterIndex {
        1 => {
            return uqm_lua_checked!(getGameStateUint(
                b"SHOFIXTI_GRPOFFS\0".as_ptr() as *const ::core::ffi::c_char,
            ), 0);
        }
        2 => {
            return uqm_lua_checked!(getGameStateUint(
                b"ZOQFOT_GRPOFFS\0".as_ptr() as *const ::core::ffi::c_char,
            ), 0);
        }
        3 => {
            return uqm_lua_checked!(getGameStateUint(
                b"MELNORME0_GRPOFFS\0".as_ptr() as *const ::core::ffi::c_char,
            ), 0);
        }
        4 => {
            return uqm_lua_checked!(getGameStateUint(
                b"MELNORME1_GRPOFFS\0".as_ptr() as *const ::core::ffi::c_char,
            ), 0);
        }
        5 => {
            return uqm_lua_checked!(getGameStateUint(
                b"MELNORME2_GRPOFFS\0".as_ptr() as *const ::core::ffi::c_char,
            ), 0);
        }
        6 => {
            return uqm_lua_checked!(getGameStateUint(
                b"MELNORME3_GRPOFFS\0".as_ptr() as *const ::core::ffi::c_char,
            ), 0);
        }
        7 => {
            return uqm_lua_checked!(getGameStateUint(
                b"MELNORME4_GRPOFFS\0".as_ptr() as *const ::core::ffi::c_char,
            ), 0);
        }
        8 => {
            return uqm_lua_checked!(getGameStateUint(
                b"MELNORME5_GRPOFFS\0".as_ptr() as *const ::core::ffi::c_char,
            ), 0);
        }
        9 => {
            return uqm_lua_checked!(getGameStateUint(
                b"MELNORME6_GRPOFFS\0".as_ptr() as *const ::core::ffi::c_char,
            ), 0);
        }
        10 => {
            return uqm_lua_checked!(getGameStateUint(
                b"MELNORME7_GRPOFFS\0".as_ptr() as *const ::core::ffi::c_char,
            ), 0);
        }
        11 => {
            return uqm_lua_checked!(getGameStateUint(
                b"MELNORME8_GRPOFFS\0".as_ptr() as *const ::core::ffi::c_char,
            ), 0);
        }
        12 => {
            return uqm_lua_checked!(getGameStateUint(
                b"URQUAN_PROBE_GRPOFFS\0".as_ptr() as *const ::core::ffi::c_char,
            ), 0);
        }
        13 => {
            return uqm_lua_checked!(getGameStateUint(
                b"COLONY_GRPOFFS\0".as_ptr() as *const ::core::ffi::c_char,
            ), 0);
        }
        14 => {
            return uqm_lua_checked!(getGameStateUint(
                b"SAMATRA_GRPOFFS\0".as_ptr() as *const ::core::ffi::c_char,
            ), 0);
        }
        _ => {
            log_add(
                log_Level::log_Warning,
                b"SetBattleGroupOffset: invalid encounter index.\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return 0 as DWORD;
        }
    };
}
unsafe extern "C" fn SaveGroups(mut fh: *mut uio_Stream) {
    let mut fp: *mut GAME_STATE_FILE = ::core::ptr::null_mut::<GAME_STATE_FILE>();
    fp = OpenStateFile(
        1 as ::core::ffi::c_int,
        b"rb\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !fp.is_null() && LengthStateFile(fp) > 0 as DWORD {
        let mut h: GROUP_HEADER = GROUP_HEADER {
            NumGroups: 0,
            day_index: 0,
            month_index: 0,
            star_index: 0,
            year_index: 0,
            GroupOffset: [0; 65],
        };
        let mut lastenc: BYTE = 0;
        let mut count: BYTE = 0;
        let mut i: ::core::ffi::c_int = 0;
        ReadGroupHeader(fp, &raw mut h);
        SeekStateFile(
            fp,
            h.GroupOffset[0usize] as ::core::ffi::c_long,
            0 as ::core::ffi::c_int,
        );
        sread_8(fp, &raw mut lastenc);
        sread_8(fp, &raw mut count);
        uqm_lua_checked!(write_32(
            fh as *mut ::core::ffi::c_void,
            0x73707247 as ::core::ffi::c_int as DWORD,
        ), ());
        uqm_lua_checked!(write_32(
            fh as *mut ::core::ffi::c_void,
            (1 as ::core::ffi::c_int
                + 14 as ::core::ffi::c_int * count as ::core::ffi::c_int) as DWORD,
        ), ());
        uqm_lua_checked!(write_8(fh as *mut ::core::ffi::c_void, lastenc), ());
        i = 0 as ::core::ffi::c_int;
        while i < count as ::core::ffi::c_int {
            let mut race_outer: BYTE = 0;
            let mut ip: IP_GROUP = IP_GROUP {
                pred: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                succ: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                group_counter: 0,
                race_id: 0,
                sys_loc: 0,
                task: 0,
                in_system: 0,
                dest_loc: 0,
                orbit_pos: 0,
                group_id: 0,
                loc: POINT { x: 0, y: 0 },
                melee_icon: ::core::ptr::null_mut::<FRAME_DESC>(),
                flags: 0,
            };
            sread_8(fp, &raw mut race_outer);
            ReadIpGroup(fp, &raw mut ip);
            uqm_lua_checked!(write_8(fh as *mut ::core::ffi::c_void, race_outer), ());
            uqm_lua_checked!(write_16(fh as *mut ::core::ffi::c_void, ip.group_counter), ());
            uqm_lua_checked!(write_8(fh as *mut ::core::ffi::c_void, ip.race_id), ());
            uqm_lua_checked!(write_8(fh as *mut ::core::ffi::c_void, ip.sys_loc), ());
            uqm_lua_checked!(write_8(fh as *mut ::core::ffi::c_void, ip.task), ());
            uqm_lua_checked!(write_8(fh as *mut ::core::ffi::c_void, ip.in_system), ());
            uqm_lua_checked!(write_8(fh as *mut ::core::ffi::c_void, ip.dest_loc), ());
            uqm_lua_checked!(write_8(fh as *mut ::core::ffi::c_void, ip.orbit_pos), ());
            uqm_lua_checked!(write_8(fh as *mut ::core::ffi::c_void, ip.group_id), ());
            uqm_lua_checked!(write_16(fh as *mut ::core::ffi::c_void, ip.loc.x as UWORD), ());
            uqm_lua_checked!(write_16(fh as *mut ::core::ffi::c_void, ip.loc.y as UWORD), ());
            i += 1;
        }
        uqm_lua_checked!(SaveBattleGroup(fp, 0 as DWORD, 0 as DWORD, fh), ());
        CloseStateFile(fp);
    }
    fp = OpenStateFile(
        2 as ::core::ffi::c_int,
        b"rb\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !fp.is_null() && LengthStateFile(fp) > 0 as DWORD {
        let mut encounter_index: ::core::ffi::c_int = 0;
        encounter_index = 1 as ::core::ffi::c_int;
        while encounter_index < 15 as ::core::ffi::c_int {
            let mut grpoffs: DWORD = uqm_lua_checked!(GetBattleGroupOffset(encounter_index), ());
            if grpoffs != 0 {
                uqm_lua_checked!(SaveBattleGroup(fp, encounter_index as DWORD, grpoffs, fh), ());
            }
            encounter_index += 1;
        }
        CloseStateFile(fp);
    }
}
#[no_mangle]
pub unsafe extern "C" fn SaveGame(
    mut which_game: COUNT,
    mut SummPtr: *mut SUMMARY_DESC,
    mut name: *const ::core::ffi::c_char,
) -> BOOLEAN {
    let mut out_fp: *mut uio_Stream = ::core::ptr::null_mut::<uio_Stream>();
    let mut pt: POINT = POINT { x: 0, y: 0 };
    let mut SD: STAR_DESC = STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    };
    let mut file: [::core::ffi::c_char; 4096] = [0; 4096];
    if !CurStarDescPtr.is_null() {
        SD = *CurStarDescPtr;
    } else {
        memset(
            &raw mut SD as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<STAR_DESC>(),
        );
    }
    pt = GlobData.Game_state.ip_location;
    uqm_lua_checked!(SaveFlagshipState(), BOOLEAN(0));
    if GlobData.Game_state.CurrentActivity as BYTE as ::core::ffi::c_int
        == C2Rust_Unnamed_1::IN_INTERPLANETARY.0 as ::core::ffi::c_int
        && GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
            & (C2Rust_Unnamed_1::START_ENCOUNTER.0 as ::core::ffi::c_int
                | C2Rust_Unnamed_1::START_INTERPLANETARY.0 as ::core::ffi::c_int) == 0
    {
        PutGroupInfo(
            0 as ::core::ffi::c_long as DWORD,
            !(0 as ::core::ffi::c_int) as BYTE,
        );
    }
    sprintf(
        &raw mut file as *mut ::core::ffi::c_char,
        b"uqmsave.%02u\0".as_ptr() as *const ::core::ffi::c_char,
        which_game as ::core::ffi::c_int,
    );
    out_fp = uqm_lua_checked!(res_OpenResFile(
        saveDir,
        &raw mut file as *mut ::core::ffi::c_char,
        b"wb\0".as_ptr() as *const ::core::ffi::c_char,
    ), BOOLEAN(0));
    if !out_fp.is_null() {
        io_ok = BOOLEAN::TRUE;
        uqm_lua_checked!(write_32(
            out_fp as *mut ::core::ffi::c_void,
            0x34564d4d as ::core::ffi::c_int as DWORD,
        ), BOOLEAN(0));
        uqm_lua_checked!(PrepareSummary(SummPtr, name), BOOLEAN(0));
        uqm_lua_checked!(SaveSummary(SummPtr, out_fp as *mut ::core::ffi::c_void), BOOLEAN(0));
        if uqm_lua_checked!(SaveGameState(&raw mut GlobData.Game_state, out_fp), BOOLEAN(0)).0 == 0 {
            io_ok = BOOLEAN::FALSE;
        }
        GlobData.Game_state.ip_location = pt;
        GlobData.Game_state.in_orbit = 0 as BYTE;
        uqm_lua_checked!(SaveRaceQueue(out_fp, &raw mut GlobData.Game_state.avail_race_q), BOOLEAN(0));
        if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
            & C2Rust_Unnamed_1::START_INTERPLANETARY.0 as ::core::ffi::c_int == 0
        {
            if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
                & C2Rust_Unnamed_1::START_ENCOUNTER.0 as ::core::ffi::c_int != 0
            {
                uqm_lua_checked!(SaveShipQueue(
                    out_fp,
                    &raw mut GlobData.Game_state.npc_built_ship_q,
                    0x5163704e as ::core::ffi::c_int as DWORD,
                ), BOOLEAN(0));
            } else if GlobData.Game_state.CurrentActivity as BYTE as ::core::ffi::c_int
                == C2Rust_Unnamed_1::IN_INTERPLANETARY.0 as ::core::ffi::c_int
            {
                uqm_lua_checked!(SaveGroupQueue(out_fp, &raw mut GlobData.Game_state.ip_group_q), BOOLEAN(0));
            }
        }
        uqm_lua_checked!(SaveShipQueue(
            out_fp,
            &raw mut GlobData.Game_state.built_ship_q,
            0x51706853 as ::core::ffi::c_int as DWORD,
        ), BOOLEAN(0));
        uqm_lua_checked!(SaveShipQueue(
            out_fp,
            &raw mut GlobData.Game_state.stowed_ship_q,
            0x51777453 as ::core::ffi::c_int as DWORD,
        ), BOOLEAN(0));
        uqm_lua_checked!(SaveEvents(out_fp), BOOLEAN(0));
        uqm_lua_checked!(SaveEncounters(out_fp), BOOLEAN(0));
        uqm_lua_checked!(SaveStarInfo(out_fp), BOOLEAN(0));
        uqm_lua_checked!(SaveGroups(out_fp), BOOLEAN(0));
        uqm_lua_checked!(SaveStarDesc(&raw mut SD, out_fp), BOOLEAN(0));
        uqm_lua_checked!(res_CloseResFile(out_fp), BOOLEAN(0));
        if io_ok.0 == 0 {
            uqm_lua_checked!(DeleteResFile(saveDir, &raw mut file as *mut ::core::ffi::c_char), BOOLEAN(0));
            return BOOLEAN::FALSE;
        }
    } else {
        return BOOLEAN::FALSE
    }
    return BOOLEAN::TRUE;
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
                b"src/uqm/races.h\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                410 as ::core::ffi::c_int,
                b"LockFleetInfo\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    return LockLink(pq, h) as *mut FLEET_INFO;
}
#[inline]
unsafe extern "C" fn sread_8(mut fp: *mut GAME_STATE_FILE, mut v: *mut BYTE) -> COUNT {
    let mut t: BYTE = 0;
    if v.is_null() {
        v = &raw mut t;
    }
    return ReadStateFile(v as *mut ::core::ffi::c_void, 1 as COUNT, 1 as COUNT, fp)
        as COUNT;
}
#[inline]
unsafe extern "C" fn sread_32(mut fp: *mut GAME_STATE_FILE, mut v: *mut DWORD) -> COUNT {
    let mut t: DWORD = 0;
    if v.is_null() {
        v = &raw mut t;
    }
    return ReadStateFile(v as *mut ::core::ffi::c_void, 4 as COUNT, 1 as COUNT, fp)
        as COUNT;
}
#[inline]
unsafe extern "C" fn LockIpGroup(
    mut pq: *const QUEUE,
    mut h: HIPGROUP,
) -> *mut IP_GROUP {
    ((*pq).object_size as usize == ::core::mem::size_of::<IP_GROUP>()
        || {
            __assert_fail(
                b"GetLinkSize (pq) == sizeof (IP_GROUP)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/uqm/grpinfo.h\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                89 as ::core::ffi::c_int,
                b"LockIpGroup\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    return LockLink(pq, h) as *mut IP_GROUP;
}
