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
//#define DEBUG_STARSEED
//#define DEBUG_STARSEED_TRACE
//#define DEBUG_STARSEED_TRACE_W
//#define DEBUG_STARSEED_TRACE_X
//#define DEBUG_STARSEED_TRACE_Y
//#define DEBUG_STARSEED_TRACE_Z

// Rust translation of src/uqm/starmap.c.
extern "C" {
    pub type _IO_FILE;
    pub type string_table_entry;
    pub type frame_desc;
    pub type RandomContext;
    fn sqrt(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    static stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn SetAbsStringTableIndex(String: STRING, StringTableIndex: COUNT) -> STRING;
    fn GetStringAddress(String: STRING) -> STRINGPTR;
    fn bsearch(
        _: *const ::core::ffi::c_void,
        _: *const ::core::ffi::c_void,
        _: size_t,
        _: size_t,
        _: Option<
            unsafe extern "C" fn(
                *const ::core::ffi::c_void,
                *const ::core::ffi::c_void,
            ) -> ::core::ffi::c_int,
        >,
    ) -> *mut ::core::ffi::c_void;
    fn strcmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn clock() -> clock_t;
    static mut optCustomSeed: ::core::ffi::c_int;
    fn getGameStateUint(name: *const ::core::ffi::c_char) -> DWORD;
    static mut GlobData: GLOBDATA;
    fn RandomContext_New() -> *mut RandomContext;
    fn RandomContext_Random(context: *mut RandomContext) -> DWORD;
    fn RandomContext_SeedRandom(context: *mut RandomContext, new_seed: DWORD) -> DWORD;
    fn GetRandomSeedForStar(star: *const STAR_DESC) -> DWORD;
    static mut GameStrings: STRING;
    static starmap_array: [STAR_DESC; 0];
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
pub type size_t = ::core::ffi::c_ulong;
pub type clock_t = ::core::ffi::c_int;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Color {
    pub r: BYTE,
    pub g: BYTE,
    pub b: BYTE,
    pub a: BYTE,
}
pub type FILE = _IO_FILE;
pub type STRING_TABLE_ENTRY_DESC = string_table_entry;
pub type STRING = *mut STRING_TABLE_ENTRY_DESC;
pub type STRINGPTR = *mut ::core::ffi::c_char;
pub type FRAME_DESC = frame_desc;
pub type FRAME = *mut FRAME_DESC;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct plot_location {
    pub star_pt: POINT,
    pub star: *mut STAR_DESC,
    pub dist_sq: [DWORD; 70],
}
pub type PLOT_LOCATION = plot_location;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct portal_location {
    pub star_pt: POINT,
    pub quasi_pt: POINT,
    pub nearest_star: *mut STAR_DESC,
}
pub type PORTAL_LOCATION = portal_location;
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
    pub const NUM_PLANET_COLORS: Self = Self(9);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_1(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_1 {
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PlotIdMap {
    pub idStr: *const ::core::ffi::c_char,
    pub id: COUNT,
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
#[no_mangle]
pub static mut star_array: [STAR_DESC; 520] = [
    star_desc {
        star_pt: point { x: 0 as COORD, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
    STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    },
];
#[no_mangle]
pub static mut CurStarDescPtr: *mut STAR_DESC = ::core::ptr::null_mut::<STAR_DESC>();
#[no_mangle]
pub static mut constel_array: *mut POINT = ::core::ptr::null_mut::<POINT>();
#[no_mangle]
pub static mut StarGenRNG: *mut RandomContext = ::core::ptr::null_mut::<RandomContext>();
#[no_mangle]
pub static mut portal_map: [PORTAL_LOCATION; 16] = [
    portal_location {
        star_pt: point { x: 0 as COORD, y: 0 },
        quasi_pt: POINT { x: 0, y: 0 },
        nearest_star: ::core::ptr::null_mut::<STAR_DESC>(),
    },
    PORTAL_LOCATION {
        star_pt: POINT { x: 0, y: 0 },
        quasi_pt: POINT { x: 0, y: 0 },
        nearest_star: ::core::ptr::null_mut::<STAR_DESC>(),
    },
    PORTAL_LOCATION {
        star_pt: POINT { x: 0, y: 0 },
        quasi_pt: POINT { x: 0, y: 0 },
        nearest_star: ::core::ptr::null_mut::<STAR_DESC>(),
    },
    PORTAL_LOCATION {
        star_pt: POINT { x: 0, y: 0 },
        quasi_pt: POINT { x: 0, y: 0 },
        nearest_star: ::core::ptr::null_mut::<STAR_DESC>(),
    },
    PORTAL_LOCATION {
        star_pt: POINT { x: 0, y: 0 },
        quasi_pt: POINT { x: 0, y: 0 },
        nearest_star: ::core::ptr::null_mut::<STAR_DESC>(),
    },
    PORTAL_LOCATION {
        star_pt: POINT { x: 0, y: 0 },
        quasi_pt: POINT { x: 0, y: 0 },
        nearest_star: ::core::ptr::null_mut::<STAR_DESC>(),
    },
    PORTAL_LOCATION {
        star_pt: POINT { x: 0, y: 0 },
        quasi_pt: POINT { x: 0, y: 0 },
        nearest_star: ::core::ptr::null_mut::<STAR_DESC>(),
    },
    PORTAL_LOCATION {
        star_pt: POINT { x: 0, y: 0 },
        quasi_pt: POINT { x: 0, y: 0 },
        nearest_star: ::core::ptr::null_mut::<STAR_DESC>(),
    },
    PORTAL_LOCATION {
        star_pt: POINT { x: 0, y: 0 },
        quasi_pt: POINT { x: 0, y: 0 },
        nearest_star: ::core::ptr::null_mut::<STAR_DESC>(),
    },
    PORTAL_LOCATION {
        star_pt: POINT { x: 0, y: 0 },
        quasi_pt: POINT { x: 0, y: 0 },
        nearest_star: ::core::ptr::null_mut::<STAR_DESC>(),
    },
    PORTAL_LOCATION {
        star_pt: POINT { x: 0, y: 0 },
        quasi_pt: POINT { x: 0, y: 0 },
        nearest_star: ::core::ptr::null_mut::<STAR_DESC>(),
    },
    PORTAL_LOCATION {
        star_pt: POINT { x: 0, y: 0 },
        quasi_pt: POINT { x: 0, y: 0 },
        nearest_star: ::core::ptr::null_mut::<STAR_DESC>(),
    },
    PORTAL_LOCATION {
        star_pt: POINT { x: 0, y: 0 },
        quasi_pt: POINT { x: 0, y: 0 },
        nearest_star: ::core::ptr::null_mut::<STAR_DESC>(),
    },
    PORTAL_LOCATION {
        star_pt: POINT { x: 0, y: 0 },
        quasi_pt: POINT { x: 0, y: 0 },
        nearest_star: ::core::ptr::null_mut::<STAR_DESC>(),
    },
    PORTAL_LOCATION {
        star_pt: POINT { x: 0, y: 0 },
        quasi_pt: POINT { x: 0, y: 0 },
        nearest_star: ::core::ptr::null_mut::<STAR_DESC>(),
    },
    PORTAL_LOCATION {
        star_pt: POINT { x: 0, y: 0 },
        quasi_pt: POINT { x: 0, y: 0 },
        nearest_star: ::core::ptr::null_mut::<STAR_DESC>(),
    },
];
#[no_mangle]
pub unsafe extern "C" fn FindStar(
    mut LastSDPtr: *mut STAR_DESC,
    mut puniverse: *mut POINT,
    mut xbounds: SIZE,
    mut ybounds: SIZE,
) -> *mut STAR_DESC {
    let mut min_y: COORD = 0;
    let mut max_y: COORD = 0;
    let mut lo: SIZE = 0;
    let mut hi: SIZE = 0;
    let mut BaseSDPtr: *mut STAR_DESC = ::core::ptr::null_mut::<STAR_DESC>();
    if uqm_lua_checked!(getGameStateUint(b"ARILOU_SPACE_SIDE\0".as_ptr() as *const ::core::ffi::c_char), ::core::ptr::null_mut())
        <= 1 as DWORD
    {
        BaseSDPtr = &raw mut star_array as *mut STAR_DESC;
        hi = (502 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as SIZE;
    } else {
        BaseSDPtr = (&raw mut star_array as *mut STAR_DESC)
            .offset((502 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize);
        hi = (15 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
            - 1 as ::core::ffi::c_int) as SIZE;
    }
    if LastSDPtr.is_null() {
        lo = 0 as SIZE;
    } else {
        lo = (LastSDPtr.offset_from(BaseSDPtr) + 1isize) as SIZE;
        if lo as ::core::ffi::c_int > hi as ::core::ffi::c_int {
            return ::core::ptr::null_mut::<STAR_DESC>()
        } else {
            hi = lo;
        }
    }
    if ybounds as ::core::ffi::c_int <= 0 as ::core::ffi::c_int {
        max_y = (*puniverse).y;
        min_y = max_y;
    } else {
        min_y = ((*puniverse).y as ::core::ffi::c_int - ybounds as ::core::ffi::c_int)
            as COORD;
        max_y = ((*puniverse).y as ::core::ffi::c_int + ybounds as ::core::ffi::c_int)
            as COORD;
    }
    while (lo as ::core::ffi::c_int) < hi as ::core::ffi::c_int {
        let mut mid: SIZE = 0;
        mid = (lo as ::core::ffi::c_int + hi as ::core::ffi::c_int
            >> 1 as ::core::ffi::c_int) as SIZE;
        if (*BaseSDPtr.offset(mid as isize)).star_pt.y as ::core::ffi::c_int
            >= min_y as ::core::ffi::c_int
        {
            hi = (mid as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as SIZE;
        } else {
            lo = (mid as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as SIZE;
        }
    }
    LastSDPtr = BaseSDPtr.offset(lo as isize);
    if (ybounds as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
        || (*LastSDPtr).star_pt.y as ::core::ffi::c_int <= max_y as ::core::ffi::c_int
    {
        let mut min_x: COORD = 0;
        let mut max_x: COORD = 0;
        if xbounds as ::core::ffi::c_int <= 0 as ::core::ffi::c_int {
            max_x = (*puniverse).x;
            min_x = max_x;
        } else {
            min_x = ((*puniverse).x as ::core::ffi::c_int
                - xbounds as ::core::ffi::c_int) as COORD;
            max_x = ((*puniverse).x as ::core::ffi::c_int
                + xbounds as ::core::ffi::c_int) as COORD;
        }
        loop {
            if ((ybounds as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
                || (*LastSDPtr).star_pt.y as ::core::ffi::c_int
                    >= min_y as ::core::ffi::c_int)
                && ((xbounds as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
                    || (*LastSDPtr).star_pt.x as ::core::ffi::c_int
                        >= min_x as ::core::ffi::c_int
                        && (*LastSDPtr).star_pt.x as ::core::ffi::c_int
                            <= max_x as ::core::ffi::c_int)
            {
                return LastSDPtr;
            }
            LastSDPtr = LastSDPtr.offset(1);
            if (*LastSDPtr).star_pt.y as ::core::ffi::c_int > max_y as ::core::ffi::c_int
            {
                break;
            }
        }
    }
    return ::core::ptr::null_mut::<STAR_DESC>();
}
#[no_mangle]
pub unsafe extern "C" fn GetClusterName(
    mut pSD: *const STAR_DESC,
    mut buf: *mut UNICODE,
) {
    let mut pBuf: *mut UNICODE = ::core::ptr::null_mut::<UNICODE>();
    let mut pStr: *mut UNICODE = ::core::ptr::null_mut::<UNICODE>();
    pBuf = buf as *mut UNICODE;
    if (*pSD).Prefix != 0 {
        pStr = GetStringAddress(
            SetAbsStringTableIndex(
                GameStrings,
                (C2Rust_Unnamed_2::STAR_NUMBER_BASE.0 as ::core::ffi::c_int
                    + (*pSD).Prefix as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                    as COUNT,
            ),
        );
        if !pStr.is_null() {
            loop {
                let c2rust_fresh0 = pStr;
                pStr = pStr.offset(1);
                let c2rust_fresh1 = pBuf;
                pBuf = pBuf.offset(1);
                let c2rust_lvalue_ptr = &raw mut *c2rust_fresh1;
                *c2rust_lvalue_ptr = *c2rust_fresh0;
                if *c2rust_lvalue_ptr == 0 {
                    break;
                }
            }
            *pBuf.offset(-1isize) = ' ' as UNICODE;
        }
    }
    pStr = GetStringAddress(
        SetAbsStringTableIndex(GameStrings, (*pSD).Postfix as COUNT),
    );
    if pStr.is_null() {
        *pBuf = '\0' as UNICODE;
    } else {
        loop {
            let c2rust_fresh2 = pStr;
            pStr = pStr.offset(1);
            let c2rust_fresh3 = pBuf;
            pBuf = pBuf.offset(1);
            let c2rust_lvalue_ptr_0 = &raw mut *c2rust_fresh3;
            *c2rust_lvalue_ptr_0 = *c2rust_fresh2;
            if *c2rust_lvalue_ptr_0 == 0 {
                break;
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn FindNearest(
    mut starmap: *mut STAR_DESC,
    mut p: POINT,
    mut constellation: BOOLEAN,
) -> *mut STAR_DESC {
    if starmap.is_null() || p.x as ::core::ffi::c_int == !(0 as ::core::ffi::c_int)
        || p.y as ::core::ffi::c_int == !(0 as ::core::ffi::c_int)
    {
        return ::core::ptr::null_mut::<STAR_DESC>();
    }
    let mut index: COUNT = 0;
    let mut star_id: COUNT = 0 as COUNT;
    let mut dist: DWORD = 0;
    let mut min_dist: DWORD = (9999 as ::core::ffi::c_int * 9999 as ::core::ffi::c_int)
        as DWORD;
    index = 0 as COUNT;
    while (index as ::core::ffi::c_int) < 502 as ::core::ffi::c_int {
        dist = (((*starmap.offset(index as isize)).star_pt.x as ::core::ffi::c_int
            - p.x as ::core::ffi::c_int)
            * ((*starmap.offset(index as isize)).star_pt.x as ::core::ffi::c_int
                - p.x as ::core::ffi::c_int)
            + ((*starmap.offset(index as isize)).star_pt.y as ::core::ffi::c_int
                - p.y as ::core::ffi::c_int)
                * ((*starmap.offset(index as isize)).star_pt.y as ::core::ffi::c_int
                    - p.y as ::core::ffi::c_int)) as DWORD;
        if dist < min_dist
            && ((*starmap.offset(index as isize)).Prefix as ::core::ffi::c_int
                > 0 as ::core::ffi::c_int || constellation.0 == 0)
        {
            min_dist = dist;
            star_id = index;
        }
        index = index.wrapping_add(1);
    }
    return starmap.offset(star_id as isize);
}
#[no_mangle]
pub unsafe extern "C" fn FindNearestStar(
    mut starmap: *mut STAR_DESC,
    mut p: POINT,
) -> *mut STAR_DESC {
    return if !starmap.is_null() {
        FindNearest(starmap, p, BOOLEAN::FALSE)
    } else {
        ::core::ptr::null_mut::<STAR_DESC>()
    };
}
#[no_mangle]
pub unsafe extern "C" fn FindNearestConstellation(
    mut starmap: *mut STAR_DESC,
    mut p: POINT,
) -> *mut STAR_DESC {
    return if !starmap.is_null() {
        FindNearest(starmap, p, BOOLEAN::TRUE)
    } else {
        ::core::ptr::null_mut::<STAR_DESC>()
    };
}
#[no_mangle]
pub static mut plot_map: [PLOT_LOCATION; 70] = [
    plot_location {
        star_pt: point { x: 0 as COORD, y: 0 },
        star: ::core::ptr::null_mut::<STAR_DESC>(),
        dist_sq: [0; 70],
    },
    PLOT_LOCATION {
        star_pt: POINT { x: 0, y: 0 },
        star: ::core::ptr::null_mut::<STAR_DESC>(),
        dist_sq: [0; 70],
    },
    PLOT_LOCATION {
        star_pt: POINT { x: 0, y: 0 },
        star: ::core::ptr::null_mut::<STAR_DESC>(),
        dist_sq: [0; 70],
    },
    PLOT_LOCATION {
        star_pt: POINT { x: 0, y: 0 },
        star: ::core::ptr::null_mut::<STAR_DESC>(),
        dist_sq: [0; 70],
    },
    PLOT_LOCATION {
        star_pt: POINT { x: 0, y: 0 },
        star: ::core::ptr::null_mut::<STAR_DESC>(),
        dist_sq: [0; 70],
    },
    PLOT_LOCATION {
        star_pt: POINT { x: 0, y: 0 },
        star: ::core::ptr::null_mut::<STAR_DESC>(),
        dist_sq: [0; 70],
    },
    PLOT_LOCATION {
        star_pt: POINT { x: 0, y: 0 },
        star: ::core::ptr::null_mut::<STAR_DESC>(),
        dist_sq: [0; 70],
    },
    PLOT_LOCATION {
        star_pt: POINT { x: 0, y: 0 },
        star: ::core::ptr::null_mut::<STAR_DESC>(),
        dist_sq: [0; 70],
    },
    PLOT_LOCATION {
        star_pt: POINT { x: 0, y: 0 },
        star: ::core::ptr::null_mut::<STAR_DESC>(),
        dist_sq: [0; 70],
    },
    PLOT_LOCATION {
        star_pt: POINT { x: 0, y: 0 },
        star: ::core::ptr::null_mut::<STAR_DESC>(),
        dist_sq: [0; 70],
    },
    PLOT_LOCATION {
        star_pt: POINT { x: 0, y: 0 },
        star: ::core::ptr::null_mut::<STAR_DESC>(),
        dist_sq: [0; 70],
    },
    PLOT_LOCATION {
        star_pt: POINT { x: 0, y: 0 },
        star: ::core::ptr::null_mut::<STAR_DESC>(),
        dist_sq: [0; 70],
    },
    PLOT_LOCATION {
        star_pt: POINT { x: 0, y: 0 },
        star: ::core::ptr::null_mut::<STAR_DESC>(),
        dist_sq: [0; 70],
    },
    PLOT_LOCATION {
        star_pt: POINT { x: 0, y: 0 },
        star: ::core::ptr::null_mut::<STAR_DESC>(),
        dist_sq: [0; 70],
    },
    PLOT_LOCATION {
        star_pt: POINT { x: 0, y: 0 },
        star: ::core::ptr::null_mut::<STAR_DESC>(),
        dist_sq: [0; 70],
    },
    PLOT_LOCATION {
        star_pt: POINT { x: 0, y: 0 },
        star: ::core::ptr::null_mut::<STAR_DESC>(),
        dist_sq: [0; 70],
    },
    PLOT_LOCATION {
        star_pt: POINT { x: 0, y: 0 },
        star: ::core::ptr::null_mut::<STAR_DESC>(),
        dist_sq: [0; 70],
    },
    PLOT_LOCATION {
        star_pt: POINT { x: 0, y: 0 },
        star: ::core::ptr::null_mut::<STAR_DESC>(),
        dist_sq: [0; 70],
    },
    PLOT_LOCATION {
        star_pt: POINT { x: 0, y: 0 },
        star: ::core::ptr::null_mut::<STAR_DESC>(),
        dist_sq: [0; 70],
    },
    PLOT_LOCATION {
        star_pt: POINT { x: 0, y: 0 },
        star: ::core::ptr::null_mut::<STAR_DESC>(),
        dist_sq: [0; 70],
    },
    PLOT_LOCATION {
        star_pt: POINT { x: 0, y: 0 },
        star: ::core::ptr::null_mut::<STAR_DESC>(),
        dist_sq: [0; 70],
    },
    PLOT_LOCATION {
        star_pt: POINT { x: 0, y: 0 },
        star: ::core::ptr::null_mut::<STAR_DESC>(),
        dist_sq: [0; 70],
    },
    PLOT_LOCATION {
        star_pt: POINT { x: 0, y: 0 },
        star: ::core::ptr::null_mut::<STAR_DESC>(),
        dist_sq: [0; 70],
    },
    PLOT_LOCATION {
        star_pt: POINT { x: 0, y: 0 },
        star: ::core::ptr::null_mut::<STAR_DESC>(),
        dist_sq: [0; 70],
    },
    PLOT_LOCATION {
        star_pt: POINT { x: 0, y: 0 },
        star: ::core::ptr::null_mut::<STAR_DESC>(),
        dist_sq: [0; 70],
    },
    PLOT_LOCATION {
        star_pt: POINT { x: 0, y: 0 },
        star: ::core::ptr::null_mut::<STAR_DESC>(),
        dist_sq: [0; 70],
    },
    PLOT_LOCATION {
        star_pt: POINT { x: 0, y: 0 },
        star: ::core::ptr::null_mut::<STAR_DESC>(),
        dist_sq: [0; 70],
    },
    PLOT_LOCATION {
        star_pt: POINT { x: 0, y: 0 },
        star: ::core::ptr::null_mut::<STAR_DESC>(),
        dist_sq: [0; 70],
    },
    PLOT_LOCATION {
        star_pt: POINT { x: 0, y: 0 },
        star: ::core::ptr::null_mut::<STAR_DESC>(),
        dist_sq: [0; 70],
    },
    PLOT_LOCATION {
        star_pt: POINT { x: 0, y: 0 },
        star: ::core::ptr::null_mut::<STAR_DESC>(),
        dist_sq: [0; 70],
    },
    PLOT_LOCATION {
        star_pt: POINT { x: 0, y: 0 },
        star: ::core::ptr::null_mut::<STAR_DESC>(),
        dist_sq: [0; 70],
    },
    PLOT_LOCATION {
        star_pt: POINT { x: 0, y: 0 },
        star: ::core::ptr::null_mut::<STAR_DESC>(),
        dist_sq: [0; 70],
    },
    PLOT_LOCATION {
        star_pt: POINT { x: 0, y: 0 },
        star: ::core::ptr::null_mut::<STAR_DESC>(),
        dist_sq: [0; 70],
    },
    PLOT_LOCATION {
        star_pt: POINT { x: 0, y: 0 },
        star: ::core::ptr::null_mut::<STAR_DESC>(),
        dist_sq: [0; 70],
    },
    PLOT_LOCATION {
        star_pt: POINT { x: 0, y: 0 },
        star: ::core::ptr::null_mut::<STAR_DESC>(),
        dist_sq: [0; 70],
    },
    PLOT_LOCATION {
        star_pt: POINT { x: 0, y: 0 },
        star: ::core::ptr::null_mut::<STAR_DESC>(),
        dist_sq: [0; 70],
    },
    PLOT_LOCATION {
        star_pt: POINT { x: 0, y: 0 },
        star: ::core::ptr::null_mut::<STAR_DESC>(),
        dist_sq: [0; 70],
    },
    PLOT_LOCATION {
        star_pt: POINT { x: 0, y: 0 },
        star: ::core::ptr::null_mut::<STAR_DESC>(),
        dist_sq: [0; 70],
    },
    PLOT_LOCATION {
        star_pt: POINT { x: 0, y: 0 },
        star: ::core::ptr::null_mut::<STAR_DESC>(),
        dist_sq: [0; 70],
    },
    PLOT_LOCATION {
        star_pt: POINT { x: 0, y: 0 },
        star: ::core::ptr::null_mut::<STAR_DESC>(),
        dist_sq: [0; 70],
    },
    PLOT_LOCATION {
        star_pt: POINT { x: 0, y: 0 },
        star: ::core::ptr::null_mut::<STAR_DESC>(),
        dist_sq: [0; 70],
    },
    PLOT_LOCATION {
        star_pt: POINT { x: 0, y: 0 },
        star: ::core::ptr::null_mut::<STAR_DESC>(),
        dist_sq: [0; 70],
    },
    PLOT_LOCATION {
        star_pt: POINT { x: 0, y: 0 },
        star: ::core::ptr::null_mut::<STAR_DESC>(),
        dist_sq: [0; 70],
    },
    PLOT_LOCATION {
        star_pt: POINT { x: 0, y: 0 },
        star: ::core::ptr::null_mut::<STAR_DESC>(),
        dist_sq: [0; 70],
    },
    PLOT_LOCATION {
        star_pt: POINT { x: 0, y: 0 },
        star: ::core::ptr::null_mut::<STAR_DESC>(),
        dist_sq: [0; 70],
    },
    PLOT_LOCATION {
        star_pt: POINT { x: 0, y: 0 },
        star: ::core::ptr::null_mut::<STAR_DESC>(),
        dist_sq: [0; 70],
    },
    PLOT_LOCATION {
        star_pt: POINT { x: 0, y: 0 },
        star: ::core::ptr::null_mut::<STAR_DESC>(),
        dist_sq: [0; 70],
    },
    PLOT_LOCATION {
        star_pt: POINT { x: 0, y: 0 },
        star: ::core::ptr::null_mut::<STAR_DESC>(),
        dist_sq: [0; 70],
    },
    PLOT_LOCATION {
        star_pt: POINT { x: 0, y: 0 },
        star: ::core::ptr::null_mut::<STAR_DESC>(),
        dist_sq: [0; 70],
    },
    PLOT_LOCATION {
        star_pt: POINT { x: 0, y: 0 },
        star: ::core::ptr::null_mut::<STAR_DESC>(),
        dist_sq: [0; 70],
    },
    PLOT_LOCATION {
        star_pt: POINT { x: 0, y: 0 },
        star: ::core::ptr::null_mut::<STAR_DESC>(),
        dist_sq: [0; 70],
    },
    PLOT_LOCATION {
        star_pt: POINT { x: 0, y: 0 },
        star: ::core::ptr::null_mut::<STAR_DESC>(),
        dist_sq: [0; 70],
    },
    PLOT_LOCATION {
        star_pt: POINT { x: 0, y: 0 },
        star: ::core::ptr::null_mut::<STAR_DESC>(),
        dist_sq: [0; 70],
    },
    PLOT_LOCATION {
        star_pt: POINT { x: 0, y: 0 },
        star: ::core::ptr::null_mut::<STAR_DESC>(),
        dist_sq: [0; 70],
    },
    PLOT_LOCATION {
        star_pt: POINT { x: 0, y: 0 },
        star: ::core::ptr::null_mut::<STAR_DESC>(),
        dist_sq: [0; 70],
    },
    PLOT_LOCATION {
        star_pt: POINT { x: 0, y: 0 },
        star: ::core::ptr::null_mut::<STAR_DESC>(),
        dist_sq: [0; 70],
    },
    PLOT_LOCATION {
        star_pt: POINT { x: 0, y: 0 },
        star: ::core::ptr::null_mut::<STAR_DESC>(),
        dist_sq: [0; 70],
    },
    PLOT_LOCATION {
        star_pt: POINT { x: 0, y: 0 },
        star: ::core::ptr::null_mut::<STAR_DESC>(),
        dist_sq: [0; 70],
    },
    PLOT_LOCATION {
        star_pt: POINT { x: 0, y: 0 },
        star: ::core::ptr::null_mut::<STAR_DESC>(),
        dist_sq: [0; 70],
    },
    PLOT_LOCATION {
        star_pt: POINT { x: 0, y: 0 },
        star: ::core::ptr::null_mut::<STAR_DESC>(),
        dist_sq: [0; 70],
    },
    PLOT_LOCATION {
        star_pt: POINT { x: 0, y: 0 },
        star: ::core::ptr::null_mut::<STAR_DESC>(),
        dist_sq: [0; 70],
    },
    PLOT_LOCATION {
        star_pt: POINT { x: 0, y: 0 },
        star: ::core::ptr::null_mut::<STAR_DESC>(),
        dist_sq: [0; 70],
    },
    PLOT_LOCATION {
        star_pt: POINT { x: 0, y: 0 },
        star: ::core::ptr::null_mut::<STAR_DESC>(),
        dist_sq: [0; 70],
    },
    PLOT_LOCATION {
        star_pt: POINT { x: 0, y: 0 },
        star: ::core::ptr::null_mut::<STAR_DESC>(),
        dist_sq: [0; 70],
    },
    PLOT_LOCATION {
        star_pt: POINT { x: 0, y: 0 },
        star: ::core::ptr::null_mut::<STAR_DESC>(),
        dist_sq: [0; 70],
    },
    PLOT_LOCATION {
        star_pt: POINT { x: 0, y: 0 },
        star: ::core::ptr::null_mut::<STAR_DESC>(),
        dist_sq: [0; 70],
    },
    PLOT_LOCATION {
        star_pt: POINT { x: 0, y: 0 },
        star: ::core::ptr::null_mut::<STAR_DESC>(),
        dist_sq: [0; 70],
    },
    PLOT_LOCATION {
        star_pt: POINT { x: 0, y: 0 },
        star: ::core::ptr::null_mut::<STAR_DESC>(),
        dist_sq: [0; 70],
    },
    PLOT_LOCATION {
        star_pt: POINT { x: 0, y: 0 },
        star: ::core::ptr::null_mut::<STAR_DESC>(),
        dist_sq: [0; 70],
    },
    PLOT_LOCATION {
        star_pt: POINT { x: 0, y: 0 },
        star: ::core::ptr::null_mut::<STAR_DESC>(),
        dist_sq: [0; 70],
    },
];
#[no_mangle]
pub static mut portalmap_array: [PORTAL_LOCATION; 16] = [
    portal_location {
        star_pt: point {
            x: 4091 as COORD,
            y: 7748 as COORD,
        },
        quasi_pt: point {
            x: (-12 as ::core::ffi::c_int * 20 as ::core::ffi::c_int
                + 5000 as ::core::ffi::c_int) as COORD,
            y: (-21 as ::core::ffi::c_int * 20 as ::core::ffi::c_int
                + 5000 as ::core::ffi::c_int) as COORD,
        },
        nearest_star: ::core::ptr::null_mut::<STAR_DESC>(),
    },
    portal_location {
        star_pt: point {
            x: 3184 as COORD,
            y: 4906 as COORD,
        },
        quasi_pt: point {
            x: (1 as ::core::ffi::c_int * 20 as ::core::ffi::c_int
                + 5000 as ::core::ffi::c_int) as COORD,
            y: (-20 as ::core::ffi::c_int * 20 as ::core::ffi::c_int
                + 5000 as ::core::ffi::c_int) as COORD,
        },
        nearest_star: ::core::ptr::null_mut::<STAR_DESC>(),
    },
    portal_location {
        star_pt: point {
            x: 9211 as COORD,
            y: 6104 as COORD,
        },
        quasi_pt: point {
            x: (-16 as ::core::ffi::c_int * 20 as ::core::ffi::c_int
                + 5000 as ::core::ffi::c_int) as COORD,
            y: (-18 as ::core::ffi::c_int * 20 as ::core::ffi::c_int
                + 5000 as ::core::ffi::c_int) as COORD,
        },
        nearest_star: ::core::ptr::null_mut::<STAR_DESC>(),
    },
    portal_location {
        star_pt: point {
            x: 5673 as COORD,
            y: 1207 as COORD,
        },
        quasi_pt: point {
            x: (8 as ::core::ffi::c_int * 20 as ::core::ffi::c_int
                + 5000 as ::core::ffi::c_int) as COORD,
            y: (-17 as ::core::ffi::c_int * 20 as ::core::ffi::c_int
                + 5000 as ::core::ffi::c_int) as COORD,
        },
        nearest_star: ::core::ptr::null_mut::<STAR_DESC>(),
    },
    portal_location {
        star_pt: point {
            x: 1910 as COORD,
            y: 926 as COORD,
        },
        quasi_pt: point {
            x: (3 as ::core::ffi::c_int * 20 as ::core::ffi::c_int
                + 5000 as ::core::ffi::c_int) as COORD,
            y: (-13 as ::core::ffi::c_int * 20 as ::core::ffi::c_int
                + 5000 as ::core::ffi::c_int) as COORD,
        },
        nearest_star: ::core::ptr::null_mut::<STAR_DESC>(),
    },
    portal_location {
        star_pt: point {
            x: 8607 as COORD,
            y: 151 as COORD,
        },
        quasi_pt: point {
            x: (-21 as ::core::ffi::c_int * 20 as ::core::ffi::c_int
                + 5000 as ::core::ffi::c_int) as COORD,
            y: (-4 as ::core::ffi::c_int * 20 as ::core::ffi::c_int
                + 5000 as ::core::ffi::c_int) as COORD,
        },
        nearest_star: ::core::ptr::null_mut::<STAR_DESC>(),
    },
    portal_location {
        star_pt: point {
            x: 50 as COORD,
            y: 1647 as COORD,
        },
        quasi_pt: point {
            x: (-4 as ::core::ffi::c_int * 20 as ::core::ffi::c_int
                + 5000 as ::core::ffi::c_int) as COORD,
            y: (-4 as ::core::ffi::c_int * 20 as ::core::ffi::c_int
                + 5000 as ::core::ffi::c_int) as COORD,
        },
        nearest_star: ::core::ptr::null_mut::<STAR_DESC>(),
    },
    portal_location {
        star_pt: point {
            x: 6117 as COORD,
            y: 4131 as COORD,
        },
        quasi_pt: point {
            x: (-12 as ::core::ffi::c_int * 20 as ::core::ffi::c_int
                + 5000 as ::core::ffi::c_int) as COORD,
            y: (-2 as ::core::ffi::c_int * 20 as ::core::ffi::c_int
                + 5000 as ::core::ffi::c_int) as COORD,
        },
        nearest_star: ::core::ptr::null_mut::<STAR_DESC>(),
    },
    portal_location {
        star_pt: point {
            x: 5658 as COORD,
            y: 9712 as COORD,
        },
        quasi_pt: point {
            x: (-26 as ::core::ffi::c_int * 20 as ::core::ffi::c_int
                + 5000 as ::core::ffi::c_int) as COORD,
            y: (2 as ::core::ffi::c_int * 20 as ::core::ffi::c_int
                + 5000 as ::core::ffi::c_int) as COORD,
        },
        nearest_star: ::core::ptr::null_mut::<STAR_DESC>(),
    },
    portal_location {
        star_pt: point {
            x: 2302 as COORD,
            y: 3988 as COORD,
        },
        quasi_pt: point {
            x: (-17 as ::core::ffi::c_int * 20 as ::core::ffi::c_int
                + 5000 as ::core::ffi::c_int) as COORD,
            y: (7 as ::core::ffi::c_int * 20 as ::core::ffi::c_int
                + 5000 as ::core::ffi::c_int) as COORD,
        },
        nearest_star: ::core::ptr::null_mut::<STAR_DESC>(),
    },
    portal_location {
        star_pt: point {
            x: 112 as COORD,
            y: 9409 as COORD,
        },
        quasi_pt: point {
            x: (10 as ::core::ffi::c_int * 20 as ::core::ffi::c_int
                + 5000 as ::core::ffi::c_int) as COORD,
            y: (7 as ::core::ffi::c_int * 20 as ::core::ffi::c_int
                + 5000 as ::core::ffi::c_int) as COORD,
        },
        nearest_star: ::core::ptr::null_mut::<STAR_DESC>(),
    },
    portal_location {
        star_pt: point {
            x: 7752 as COORD,
            y: 8906 as COORD,
        },
        quasi_pt: point {
            x: (15 as ::core::ffi::c_int * 20 as ::core::ffi::c_int
                + 5000 as ::core::ffi::c_int) as COORD,
            y: (14 as ::core::ffi::c_int * 20 as ::core::ffi::c_int
                + 5000 as ::core::ffi::c_int) as COORD,
        },
        nearest_star: ::core::ptr::null_mut::<STAR_DESC>(),
    },
    portal_location {
        star_pt: point {
            x: 368 as COORD,
            y: 6332 as COORD,
        },
        quasi_pt: point {
            x: (22 as ::core::ffi::c_int * 20 as ::core::ffi::c_int
                + 5000 as ::core::ffi::c_int) as COORD,
            y: (16 as ::core::ffi::c_int * 20 as ::core::ffi::c_int
                + 5000 as ::core::ffi::c_int) as COORD,
        },
        nearest_star: ::core::ptr::null_mut::<STAR_DESC>(),
    },
    portal_location {
        star_pt: point {
            x: 9735 as COORD,
            y: 3153 as COORD,
        },
        quasi_pt: point {
            x: (-6 as ::core::ffi::c_int * 20 as ::core::ffi::c_int
                + 5000 as ::core::ffi::c_int) as COORD,
            y: (19 as ::core::ffi::c_int * 20 as ::core::ffi::c_int
                + 5000 as ::core::ffi::c_int) as COORD,
        },
        nearest_star: ::core::ptr::null_mut::<STAR_DESC>(),
    },
    portal_location {
        star_pt: point {
            x: 5850 as COORD,
            y: 6213 as COORD,
        },
        quasi_pt: point {
            x: (10 as ::core::ffi::c_int * 20 as ::core::ffi::c_int
                + 5000 as ::core::ffi::c_int) as COORD,
            y: (20 as ::core::ffi::c_int * 20 as ::core::ffi::c_int
                + 5000 as ::core::ffi::c_int) as COORD,
        },
        nearest_star: ::core::ptr::null_mut::<STAR_DESC>(),
    },
    portal_location {
        star_pt: point {
            x: 0 as COORD,
            y: 0 as COORD,
        },
        quasi_pt: point {
            x: 6134 as COORD,
            y: 5900 as COORD,
        },
        nearest_star: ::core::ptr::null_mut::<STAR_DESC>(),
    },
];
#[no_mangle]
pub unsafe extern "C" fn DefaultStarmap(mut starmap: *mut STAR_DESC) {
    if starmap.is_null() {
        fprintf(
            stderr,
            b"DefaaultStarmap called with NULL starmap PTR.\n\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut i: COUNT = 0;
    extern "C" {
        #[link_name = "starmap_array"]
        static starmap_array_0: [STAR_DESC; 0];
    }
    i = 0 as COUNT;
    while (i as ::core::ffi::c_int)
        < 502 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 15 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
    {
        *starmap.offset(i as isize) = *(&raw const starmap_array as *const STAR_DESC)
            .offset(i as isize);
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn SeedStarmap(mut starmap: *mut STAR_DESC) {
    if starmap.is_null() {
        fprintf(
            stderr,
            b"SeedStarmap called with NULL starmap PTR.\n\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut i: COUNT = 0;
    let mut rand_val: UWORD = 0;
    if StarGenRNG.is_null() {
        fprintf(
            stderr,
            b"****SeedStarmap creating a STAR GEN RNG****\n\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        StarGenRNG = RandomContext_New();
    }
    RandomContext_SeedRandom(StarGenRNG, optCustomSeed as DWORD);
    i = 0 as COUNT;
    while (i as ::core::ffi::c_int) < 502 as ::core::ffi::c_int {
        rand_val = RandomContext_Random(StarGenRNG) as UWORD;
        (*starmap.offset(i as isize)).Type = (((if rand_val as BYTE as ::core::ffi::c_int
            % (469 as ::core::ffi::c_int + 24 as ::core::ffi::c_int)
            >= 24 as ::core::ffi::c_int
        {
            C2Rust_Unnamed::DWARF_STAR.0 as ::core::ffi::c_int
        } else {
            C2Rust_Unnamed::GIANT_STAR.0 as ::core::ffi::c_int
        }) as BYTE as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
            & ((0xff as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                & !(((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as BYTE
                    as ::core::ffi::c_int)) as BYTE as ::core::ffi::c_int
            | (((rand_val as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as BYTE
                as ::core::ffi::c_int
                % C2Rust_Unnamed_0::NUM_STAR_COLORS.0 as ::core::ffi::c_int) as BYTE
                as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
                & ((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as BYTE
                    as ::core::ffi::c_int
            | (-1 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int)
                << 0 as ::core::ffi::c_int
                & ((0xff as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                    & !(((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as BYTE
                        as ::core::ffi::c_int
                        | ((0xff as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                            & !(((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int)
                                as BYTE as ::core::ffi::c_int)) as BYTE
                            as ::core::ffi::c_int)) as BYTE as ::core::ffi::c_int)
            as BYTE;
        (*starmap.offset(i as isize)).Index = 0 as BYTE;
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn print_plot_id(mut plot_id: COUNT) {
    static mut plot_name: [*const ::core::ffi::c_char; 70] = [
        b"ARILOU_DEFINED\0".as_ptr() as *const ::core::ffi::c_char,
        b"SOL_DEFINED\0".as_ptr() as *const ::core::ffi::c_char,
        b"SHOFIXTI_DEFINED\0".as_ptr() as *const ::core::ffi::c_char,
        b"MAIDENS_DEFINED\0".as_ptr() as *const ::core::ffi::c_char,
        b"START_COLONY_DEFINED\0".as_ptr() as *const ::core::ffi::c_char,
        b"SPATHI_DEFINED\0".as_ptr() as *const ::core::ffi::c_char,
        b"ZOQFOT_DEFINED\0".as_ptr() as *const ::core::ffi::c_char,
        b"MELNORME0_DEFINED\0".as_ptr() as *const ::core::ffi::c_char,
        b"MELNORME1_DEFINED\0".as_ptr() as *const ::core::ffi::c_char,
        b"MELNORME2_DEFINED\0".as_ptr() as *const ::core::ffi::c_char,
        b"MELNORME3_DEFINED\0".as_ptr() as *const ::core::ffi::c_char,
        b"MELNORME4_DEFINED\0".as_ptr() as *const ::core::ffi::c_char,
        b"MELNORME5_DEFINED\0".as_ptr() as *const ::core::ffi::c_char,
        b"MELNORME6_DEFINED\0".as_ptr() as *const ::core::ffi::c_char,
        b"MELNORME7_DEFINED\0".as_ptr() as *const ::core::ffi::c_char,
        b"MELNORME8_DEFINED\0".as_ptr() as *const ::core::ffi::c_char,
        b"TALKING_PET_DEFINED\0".as_ptr() as *const ::core::ffi::c_char,
        b"CHMMR_DEFINED\0".as_ptr() as *const ::core::ffi::c_char,
        b"SYREEN_DEFINED\0".as_ptr() as *const ::core::ffi::c_char,
        b"BURVIXESE_DEFINED\0".as_ptr() as *const ::core::ffi::c_char,
        b"SLYLANDRO_DEFINED\0".as_ptr() as *const ::core::ffi::c_char,
        b"DRUUGE_DEFINED\0".as_ptr() as *const ::core::ffi::c_char,
        b"BOMB_DEFINED\0".as_ptr() as *const ::core::ffi::c_char,
        b"AQUA_HELIX_DEFINED\0".as_ptr() as *const ::core::ffi::c_char,
        b"SUN_DEVICE_DEFINED\0".as_ptr() as *const ::core::ffi::c_char,
        b"TAALO_PROTECTOR_DEFINED\0".as_ptr() as *const ::core::ffi::c_char,
        b"SHIP_VAULT_DEFINED\0".as_ptr() as *const ::core::ffi::c_char,
        b"URQUAN_WRECK_DEFINED\0".as_ptr() as *const ::core::ffi::c_char,
        b"VUX_BEAST_DEFINED\0".as_ptr() as *const ::core::ffi::c_char,
        b"SAMATRA_DEFINED\0".as_ptr() as *const ::core::ffi::c_char,
        b"ZOQ_SCOUT_DEFINED\0".as_ptr() as *const ::core::ffi::c_char,
        b"MYCON_DEFINED\0".as_ptr() as *const ::core::ffi::c_char,
        b"EGG_CASE0_DEFINED\0".as_ptr() as *const ::core::ffi::c_char,
        b"EGG_CASE1_DEFINED\0".as_ptr() as *const ::core::ffi::c_char,
        b"EGG_CASE2_DEFINED\0".as_ptr() as *const ::core::ffi::c_char,
        b"PKUNK_DEFINED\0".as_ptr() as *const ::core::ffi::c_char,
        b"UTWIG_DEFINED\0".as_ptr() as *const ::core::ffi::c_char,
        b"SUPOX_DEFINED\0".as_ptr() as *const ::core::ffi::c_char,
        b"YEHAT_DEFINED\0".as_ptr() as *const ::core::ffi::c_char,
        b"VUX_DEFINED\0".as_ptr() as *const ::core::ffi::c_char,
        b"ORZ_DEFINED\0".as_ptr() as *const ::core::ffi::c_char,
        b"THRADD_DEFINED\0".as_ptr() as *const ::core::ffi::c_char,
        b"RAINBOW0_DEFINED\0".as_ptr() as *const ::core::ffi::c_char,
        b"RAINBOW1_DEFINED\0".as_ptr() as *const ::core::ffi::c_char,
        b"RAINBOW2_DEFINED\0".as_ptr() as *const ::core::ffi::c_char,
        b"RAINBOW3_DEFINED\0".as_ptr() as *const ::core::ffi::c_char,
        b"RAINBOW4_DEFINED\0".as_ptr() as *const ::core::ffi::c_char,
        b"RAINBOW5_DEFINED\0".as_ptr() as *const ::core::ffi::c_char,
        b"RAINBOW6_DEFINED\0".as_ptr() as *const ::core::ffi::c_char,
        b"RAINBOW7_DEFINED\0".as_ptr() as *const ::core::ffi::c_char,
        b"RAINBOW8_DEFINED\0".as_ptr() as *const ::core::ffi::c_char,
        b"RAINBOW9_DEFINED\0".as_ptr() as *const ::core::ffi::c_char,
        b"ILWRATH_DEFINED\0".as_ptr() as *const ::core::ffi::c_char,
        b"ANDROSYNTH_DEFINED\0".as_ptr() as *const ::core::ffi::c_char,
        b"MYCON_TRAP_DEFINED\0".as_ptr() as *const ::core::ffi::c_char,
        b"URQUAN0_DEFINED\0".as_ptr() as *const ::core::ffi::c_char,
        b"URQUAN1_DEFINED\0".as_ptr() as *const ::core::ffi::c_char,
        b"URQUAN2_DEFINED\0".as_ptr() as *const ::core::ffi::c_char,
        b"KOHRAH0_DEFINED\0".as_ptr() as *const ::core::ffi::c_char,
        b"KOHRAH1_DEFINED\0".as_ptr() as *const ::core::ffi::c_char,
        b"KOHRAH2_DEFINED\0".as_ptr() as *const ::core::ffi::c_char,
        b"DESTROYED_STARBASE_DEFINED\0".as_ptr() as *const ::core::ffi::c_char,
        b"MOTHER_ARK_DEFINED\0".as_ptr() as *const ::core::ffi::c_char,
        b"ZOQ_COLONY0_DEFINED\0".as_ptr() as *const ::core::ffi::c_char,
        b"ZOQ_COLONY1_DEFINED\0".as_ptr() as *const ::core::ffi::c_char,
        b"ZOQ_COLONY2_DEFINED\0".as_ptr() as *const ::core::ffi::c_char,
        b"ZOQ_COLONY3_DEFINED\0".as_ptr() as *const ::core::ffi::c_char,
        b"ALGOLITES_DEFINED\0".as_ptr() as *const ::core::ffi::c_char,
        b"SPATHI_MONUMENT_DEFINED\0".as_ptr() as *const ::core::ffi::c_char,
        b"EXCAVATION_SITE_DEFINED\0".as_ptr() as *const ::core::ffi::c_char,
    ];
    fprintf(
        stderr,
        b"%s (%d)\0".as_ptr() as *const ::core::ffi::c_char,
        plot_name[plot_id as usize],
        plot_id as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn SetPlotLength(
    mut plot: *mut PLOT_LOCATION,
    mut plotA: COUNT,
    mut plotB: COUNT,
    mut p_min: COUNT,
    mut p_max: COUNT,
) {
    let mut min: COUNT = p_min;
    let mut max: COUNT = p_max;
    if plot.is_null()
        || plotA as ::core::ffi::c_int
            >= C2Rust_Unnamed_1::NUM_PLOTS.0 as ::core::ffi::c_int
        || plotB as ::core::ffi::c_int
            >= C2Rust_Unnamed_1::NUM_PLOTS.0 as ::core::ffi::c_int
        || p_min as ::core::ffi::c_int
            > p_max as ::core::ffi::c_int - 100 as ::core::ffi::c_int
    {
        fprintf(
            stderr,
            b"%s called with bad data (PTR %d %d %d %d)\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"SetPlotLength (plotmap, plot, plot, min, max)\0".as_ptr()
                as *const ::core::ffi::c_char,
            plotA as ::core::ffi::c_int,
            plotB as ::core::ffi::c_int,
            p_min as ::core::ffi::c_int,
            p_max as ::core::ffi::c_int,
        );
        return;
    }
    if max as ::core::ffi::c_int > 10000 as ::core::ffi::c_int
        || max as ::core::ffi::c_int == 0 as ::core::ffi::c_int
    {
        max = 10000 as COUNT;
    }
    if (max as ::core::ffi::c_int) < 100 as ::core::ffi::c_int {
        max = 100 as COUNT;
    }
    if min as ::core::ffi::c_int > 10000 as ::core::ffi::c_int {
        min = 10000 as COUNT;
    }
    if if (plotA as ::core::ffi::c_int) < plotB as ::core::ffi::c_int {
        (*plot.offset(plotA as isize)).dist_sq[plotB as usize]
    } else {
        (*plot.offset(plotB as isize)).dist_sq[plotA as usize]
    } > 0 as DWORD
    {
        (*plot.offset(plotA as isize)).dist_sq[plotA as usize] = (*plot
            .offset(plotA as isize))
            .dist_sq[plotA as usize]
            .wrapping_sub(
                sqrt(
                    (if (plotA as ::core::ffi::c_int) < plotB as ::core::ffi::c_int {
                        (*plot.offset(plotA as isize)).dist_sq[plotB as usize]
                    } else {
                        (*plot.offset(plotB as isize)).dist_sq[plotA as usize]
                    }) as ::core::ffi::c_double,
                ) as UWORD as DWORD,
            );
        (*plot.offset(plotB as isize)).dist_sq[plotB as usize] = (*plot
            .offset(plotB as isize))
            .dist_sq[plotB as usize]
            .wrapping_sub(
                sqrt(
                    (if (plotA as ::core::ffi::c_int) < plotB as ::core::ffi::c_int {
                        (*plot.offset(plotA as isize)).dist_sq[plotB as usize]
                    } else {
                        (*plot.offset(plotB as isize)).dist_sq[plotA as usize]
                    }) as ::core::ffi::c_double,
                ) as UWORD as DWORD,
            );
    }
    if if plotA as ::core::ffi::c_int > plotB as ::core::ffi::c_int {
        (*plot.offset(plotA as isize)).dist_sq[plotB as usize]
    } else {
        (*plot.offset(plotB as isize)).dist_sq[plotA as usize]
    } != 0 as DWORD
        && if plotA as ::core::ffi::c_int > plotB as ::core::ffi::c_int {
            (*plot.offset(plotA as isize)).dist_sq[plotB as usize]
        } else {
            (*plot.offset(plotB as isize)).dist_sq[plotA as usize]
        } < (10000 as ::core::ffi::c_int * 10000 as ::core::ffi::c_int) as DWORD
    {
        (*plot.offset(plotA as isize)).dist_sq[plotA as usize] = (*plot
            .offset(plotA as isize))
            .dist_sq[plotA as usize]
            .wrapping_add(
                (sqrt(
                    (if plotA as ::core::ffi::c_int > plotB as ::core::ffi::c_int {
                        (*plot.offset(plotA as isize)).dist_sq[plotB as usize]
                    } else {
                        (*plot.offset(plotB as isize)).dist_sq[plotA as usize]
                    }) as ::core::ffi::c_double,
                ) as UWORD as ::core::ffi::c_int - 10000 as ::core::ffi::c_int) as DWORD,
            );
        (*plot.offset(plotB as isize)).dist_sq[plotB as usize] = (*plot
            .offset(plotB as isize))
            .dist_sq[plotB as usize]
            .wrapping_add(
                (sqrt(
                    (if plotA as ::core::ffi::c_int > plotB as ::core::ffi::c_int {
                        (*plot.offset(plotA as isize)).dist_sq[plotB as usize]
                    } else {
                        (*plot.offset(plotB as isize)).dist_sq[plotA as usize]
                    }) as ::core::ffi::c_double,
                ) as UWORD as ::core::ffi::c_int - 10000 as ::core::ffi::c_int) as DWORD,
            );
    }
    if (plotA as ::core::ffi::c_int) < plotB as ::core::ffi::c_int {
        (*plot.offset(plotA as isize)).dist_sq[plotB as usize] = (min
            as ::core::ffi::c_int * min as ::core::ffi::c_int) as DWORD;
    } else {
        (*plot.offset(plotB as isize)).dist_sq[plotA as usize] = (min
            as ::core::ffi::c_int * min as ::core::ffi::c_int) as DWORD;
    }
    if plotA as ::core::ffi::c_int > plotB as ::core::ffi::c_int {
        (*plot.offset(plotA as isize)).dist_sq[plotB as usize] = (max
            as ::core::ffi::c_int * max as ::core::ffi::c_int) as DWORD;
    } else {
        (*plot.offset(plotB as isize)).dist_sq[plotA as usize] = (max
            as ::core::ffi::c_int * max as ::core::ffi::c_int) as DWORD;
    }
    (*plot.offset(plotA as isize)).dist_sq[plotA as usize] = (*plot
        .offset(plotA as isize))
        .dist_sq[plotA as usize]
        .wrapping_add(
            (min as ::core::ffi::c_int + 10000 as ::core::ffi::c_int
                - max as ::core::ffi::c_int) as DWORD,
        );
    (*plot.offset(plotB as isize)).dist_sq[plotB as usize] = (*plot
        .offset(plotB as isize))
        .dist_sq[plotB as usize]
        .wrapping_add(
            (min as ::core::ffi::c_int + 10000 as ::core::ffi::c_int
                - max as ::core::ffi::c_int) as DWORD,
        );
}
#[no_mangle]
pub unsafe extern "C" fn ConnectedPlot(
    mut plot: *mut PLOT_LOCATION,
    mut plotA: COUNT,
    mut plotB: COUNT,
) -> DWORD {
    if !(!plot.is_null()
        && (plotA as ::core::ffi::c_int)
            < C2Rust_Unnamed_1::NUM_PLOTS.0 as ::core::ffi::c_int
        && (plotB as ::core::ffi::c_int)
            < C2Rust_Unnamed_1::NUM_PLOTS.0 as ::core::ffi::c_int)
    {
        fprintf(
            stderr,
            b"%s called with bad data PTR %d %d\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"ConnectedPlot (plotmap, plot_id, plot_id)\0".as_ptr()
                as *const ::core::ffi::c_char,
            plotA as ::core::ffi::c_int,
            plotB as ::core::ffi::c_int,
        );
        return 0 as DWORD;
    }
    return if if (plotA as ::core::ffi::c_int) < plotB as ::core::ffi::c_int {
        (*plot.offset(plotA as isize)).dist_sq[plotB as usize]
    } else {
        (*plot.offset(plotB as isize)).dist_sq[plotA as usize]
    } > 100000000 as ::core::ffi::c_int as DWORD
    {
        100000000 as ::core::ffi::c_int as DWORD
    } else if (plotA as ::core::ffi::c_int) < plotB as ::core::ffi::c_int {
        (*plot.offset(plotA as isize)).dist_sq[plotB as usize]
    } else {
        (*plot.offset(plotB as isize)).dist_sq[plotA as usize]
    }
        .wrapping_add(
            if if plotA as ::core::ffi::c_int > plotB as ::core::ffi::c_int {
                (*plot.offset(plotA as isize)).dist_sq[plotB as usize]
            } else {
                (*plot.offset(plotB as isize)).dist_sq[plotA as usize]
            } == 0 as DWORD
                || if plotA as ::core::ffi::c_int > plotB as ::core::ffi::c_int {
                    (*plot.offset(plotA as isize)).dist_sq[plotB as usize]
                } else {
                    (*plot.offset(plotB as isize)).dist_sq[plotA as usize]
                } > 100000000 as ::core::ffi::c_int as DWORD
            {
                0 as DWORD
            } else {
                (100000000 as ::core::ffi::c_int as DWORD)
                    .wrapping_sub(
                        if plotA as ::core::ffi::c_int > plotB as ::core::ffi::c_int {
                            (*plot.offset(plotA as isize)).dist_sq[plotB as usize]
                        } else {
                            (*plot.offset(plotB as isize)).dist_sq[plotA as usize]
                        },
                    )
            },
        );
}
static mut next_plot: COUNT = !(0 as ::core::ffi::c_int) as COUNT;
#[no_mangle]
pub unsafe extern "C" fn GetNextPlot(mut plot: *mut PLOT_LOCATION) -> COUNT {
    let mut plot_id: COUNT = 0 as COUNT;
    let mut top_weight: DWORD = 0 as DWORD;
    let mut i: COUNT = 0;
    if plot.is_null() {
        fprintf(
            stderr,
            b"GetNextPlot (plotmap) called with bad data PTR\n\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return 0 as COUNT;
    }
    if (next_plot as ::core::ffi::c_int)
        < C2Rust_Unnamed_1::NUM_PLOTS.0 as ::core::ffi::c_int
        && !(!(*plot.offset(next_plot as isize)).star.is_null()
            && (*plot.offset(next_plot as isize)).star_pt.x as ::core::ffi::c_int
                != !(0 as ::core::ffi::c_int)
            && (*plot.offset(next_plot as isize)).star_pt.y as ::core::ffi::c_int
                != !(0 as ::core::ffi::c_int))
    {
        return next_plot;
    }
    i = 0 as COUNT;
    while (i as ::core::ffi::c_int) < C2Rust_Unnamed_1::NUM_PLOTS.0 as ::core::ffi::c_int
    {
        if (*plot.offset(i as isize)).star.is_null()
            && (*plot.offset(i as isize)).star_pt.x as ::core::ffi::c_int
                != !(0 as ::core::ffi::c_int)
            && (*plot.offset(i as isize)).star_pt.y as ::core::ffi::c_int
                != !(0 as ::core::ffi::c_int)
        {
            next_plot = i;
            return next_plot;
        }
        i = i.wrapping_add(1);
    }
    i = 0 as COUNT;
    while (i as ::core::ffi::c_int) < C2Rust_Unnamed_1::NUM_PLOTS.0 as ::core::ffi::c_int
    {
        if (*plot.offset(i as isize)).dist_sq[i as usize] > top_weight
            && !(!(*plot.offset(i as isize)).star.is_null()
                && (*plot.offset(i as isize)).star_pt.x as ::core::ffi::c_int
                    != !(0 as ::core::ffi::c_int)
                && (*plot.offset(i as isize)).star_pt.y as ::core::ffi::c_int
                    != !(0 as ::core::ffi::c_int))
        {
            plot_id = i;
            top_weight = (*plot.offset(i as isize)).dist_sq[i as usize];
        }
        i = i.wrapping_add(1);
    }
    if top_weight == 0 as DWORD {
        while (plot_id as ::core::ffi::c_int)
            < C2Rust_Unnamed_1::NUM_PLOTS.0 as ::core::ffi::c_int
            && (!(*plot.offset(plot_id as isize)).star.is_null()
                && (*plot.offset(plot_id as isize)).star_pt.x as ::core::ffi::c_int
                    != !(0 as ::core::ffi::c_int)
                && (*plot.offset(plot_id as isize)).star_pt.y as ::core::ffi::c_int
                    != !(0 as ::core::ffi::c_int))
        {
            plot_id = plot_id.wrapping_add(1);
        }
    }
    next_plot = plot_id;
    return next_plot;
}
#[no_mangle]
pub unsafe extern "C" fn ResetPlot(mut plot: *mut PLOT_LOCATION) {
    if plot.is_null() {
        fprintf(
            stderr,
            b"ResetPlot (plotmap) called with bad data PTR.\n\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut i: COUNT = 0;
    let mut j: COUNT = 0;
    i = 0 as COUNT;
    while (i as ::core::ffi::c_int) < C2Rust_Unnamed_1::NUM_PLOTS.0 as ::core::ffi::c_int
    {
        j = 0 as COUNT;
        while (j as ::core::ffi::c_int)
            < C2Rust_Unnamed_1::NUM_PLOTS.0 as ::core::ffi::c_int
        {
            (*plot.offset(i as isize)).dist_sq[j as usize] = 0 as DWORD;
            j = j.wrapping_add(1);
        }
        (*plot.offset(i as isize)).star_pt = point {
            x: !(0 as ::core::ffi::c_int) as COORD,
            y: !(0 as ::core::ffi::c_int) as COORD,
        };
        (*plot.offset(i as isize)).star = ::core::ptr::null_mut::<STAR_DESC>();
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn DefaultPlot(
    mut plot: *mut PLOT_LOCATION,
    mut starmap: *mut STAR_DESC,
) {
    let mut i: COUNT = 0;
    if plot.is_null() || starmap.is_null() {
        fprintf(
            stderr,
            b"DefaultPlot (plotmap, starmap) called %s\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"with bad data PTR.\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    ResetPlot(plot);
    (*plot.offset(0isize)).star_pt = point {
        x: 438 as COORD,
        y: 6372 as COORD,
    };
    (*plot.offset(0isize)).star = starmap
        .offset(
            (502 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                + 15 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
        );
    i = 0 as COUNT;
    while (i as ::core::ffi::c_int) < 502 as ::core::ffi::c_int {
        if (*starmap.offset(i as isize)).Index as ::core::ffi::c_int
            > 0 as ::core::ffi::c_int
        {
            if ((*starmap.offset(i as isize)).Index as ::core::ffi::c_int)
                < C2Rust_Unnamed_1::NUM_PLOTS.0 as ::core::ffi::c_int
            {
                (*plot.offset((*starmap.offset(i as isize)).Index as isize)).star_pt = (*starmap
                    .offset(i as isize))
                    .star_pt;
                (*plot.offset((*starmap.offset(i as isize)).Index as isize)).star = starmap
                    .offset(i as isize);
            }
        }
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn InitPlot(mut plotmap: *mut PLOT_LOCATION) {
    if plotmap.is_null() {
        fprintf(
            stderr,
            b"InitPlot (plotmap) called with bad data PTR\n\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut i: COUNT = 0;
    let mut j: COUNT = 0;
    ResetPlot(plotmap);
    let mut home_map: [[COUNT; 2]; 23] = [
        [
            C2Rust_Unnamed_1::ARILOU_DEFINED.0 as ::core::ffi::c_int as COUNT,
            300 as COUNT,
        ],
        [C2Rust_Unnamed_1::SOL_DEFINED.0 as ::core::ffi::c_int as COUNT, 700 as COUNT],
        [C2Rust_Unnamed_1::YEHAT_DEFINED.0 as ::core::ffi::c_int as COUNT, 700 as COUNT],
        [
            C2Rust_Unnamed_1::SHOFIXTI_DEFINED.0 as ::core::ffi::c_int as COUNT,
            700 as COUNT,
        ],
        [C2Rust_Unnamed_1::CHMMR_DEFINED.0 as ::core::ffi::c_int as COUNT, 700 as COUNT],
        [
            C2Rust_Unnamed_1::SYREEN_DEFINED.0 as ::core::ffi::c_int as COUNT,
            700 as COUNT,
        ],
        [
            C2Rust_Unnamed_1::EGG_CASE0_DEFINED.0 as ::core::ffi::c_int as COUNT,
            700 as COUNT,
        ],
        [
            C2Rust_Unnamed_1::MOTHER_ARK_DEFINED.0 as ::core::ffi::c_int as COUNT,
            700 as COUNT,
        ],
        [
            C2Rust_Unnamed_1::DRUUGE_DEFINED.0 as ::core::ffi::c_int as COUNT,
            1300 as COUNT,
        ],
        [C2Rust_Unnamed_1::PKUNK_DEFINED.0 as ::core::ffi::c_int as COUNT, 700 as COUNT],
        [C2Rust_Unnamed_1::UTWIG_DEFINED.0 as ::core::ffi::c_int as COUNT, 700 as COUNT],
        [C2Rust_Unnamed_1::SUPOX_DEFINED.0 as ::core::ffi::c_int as COUNT, 300 as COUNT],
        [
            C2Rust_Unnamed_1::ZOQFOT_DEFINED.0 as ::core::ffi::c_int as COUNT,
            700 as COUNT,
        ],
        [
            C2Rust_Unnamed_1::BURVIXESE_DEFINED.0 as ::core::ffi::c_int as COUNT,
            300 as COUNT,
        ],
        [
            C2Rust_Unnamed_1::START_COLONY_DEFINED.0 as ::core::ffi::c_int as COUNT,
            300 as COUNT,
        ],
        [
            C2Rust_Unnamed_1::SAMATRA_DEFINED.0 as ::core::ffi::c_int as COUNT,
            3000 as COUNT,
        ],
        [
            C2Rust_Unnamed_1::ANDROSYNTH_DEFINED.0 as ::core::ffi::c_int as COUNT,
            1000 as COUNT,
        ],
        [
            C2Rust_Unnamed_1::ILWRATH_DEFINED.0 as ::core::ffi::c_int as COUNT,
            1000 as COUNT,
        ],
        [
            C2Rust_Unnamed_1::MYCON_DEFINED.0 as ::core::ffi::c_int as COUNT,
            1000 as COUNT,
        ],
        [
            C2Rust_Unnamed_1::SPATHI_DEFINED.0 as ::core::ffi::c_int as COUNT,
            1000 as COUNT,
        ],
        [
            C2Rust_Unnamed_1::THRADD_DEFINED.0 as ::core::ffi::c_int as COUNT,
            1000 as COUNT,
        ],
        [C2Rust_Unnamed_1::VUX_DEFINED.0 as ::core::ffi::c_int as COUNT, 1000 as COUNT],
        [
            C2Rust_Unnamed_1::TALKING_PET_DEFINED.0 as ::core::ffi::c_int as COUNT,
            1000 as COUNT,
        ],
    ];
    i = 0 as COUNT;
    while (i as ::core::ffi::c_int) < 22 as ::core::ffi::c_int {
        j = (i as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as COUNT;
        while (j as ::core::ffi::c_int) < 23 as ::core::ffi::c_int {
            SetPlotLength(
                plotmap,
                home_map[i as usize][0usize],
                home_map[j as usize][0usize],
                (if home_map[i as usize][1usize] as ::core::ffi::c_int
                    > home_map[j as usize][1usize] as ::core::ffi::c_int
                {
                    home_map[i as usize][1usize] as ::core::ffi::c_int
                } else {
                    home_map[j as usize][1usize] as ::core::ffi::c_int
                }) as COUNT,
                10000 as COUNT,
            );
            j = j.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    InitMelnormeRainbow(plotmap);
    SetPlotLength(
        plotmap,
        C2Rust_Unnamed_1::SOL_DEFINED.0 as ::core::ffi::c_int as COUNT,
        C2Rust_Unnamed_1::SAMATRA_DEFINED.0 as ::core::ffi::c_int as COUNT,
        5000 as COUNT,
        10000 as COUNT,
    );
    SetPlotLength(
        plotmap,
        C2Rust_Unnamed_1::SOL_DEFINED.0 as ::core::ffi::c_int as COUNT,
        C2Rust_Unnamed_1::START_COLONY_DEFINED.0 as ::core::ffi::c_int as COUNT,
        1500 as COUNT,
        5000 as COUNT,
    );
    SetPlotLength(
        plotmap,
        C2Rust_Unnamed_1::ARILOU_DEFINED.0 as ::core::ffi::c_int as COUNT,
        C2Rust_Unnamed_1::ORZ_DEFINED.0 as ::core::ffi::c_int as COUNT,
        4000 as COUNT,
        10000 as COUNT,
    );
    SetPlotLength(
        plotmap,
        C2Rust_Unnamed_1::ARILOU_DEFINED.0 as ::core::ffi::c_int as COUNT,
        C2Rust_Unnamed_1::SOL_DEFINED.0 as ::core::ffi::c_int as COUNT,
        3000 as COUNT,
        7000 as COUNT,
    );
    SetPlotLength(
        plotmap,
        C2Rust_Unnamed_1::SLYLANDRO_DEFINED.0 as ::core::ffi::c_int as COUNT,
        C2Rust_Unnamed_1::SOL_DEFINED.0 as ::core::ffi::c_int as COUNT,
        6000 as COUNT,
        10000 as COUNT,
    );
    SetPlotLength(
        plotmap,
        C2Rust_Unnamed_1::DRUUGE_DEFINED.0 as ::core::ffi::c_int as COUNT,
        C2Rust_Unnamed_1::BURVIXESE_DEFINED.0 as ::core::ffi::c_int as COUNT,
        2000 as COUNT,
        4000 as COUNT,
    );
    SetPlotLength(
        plotmap,
        C2Rust_Unnamed_1::DRUUGE_DEFINED.0 as ::core::ffi::c_int as COUNT,
        C2Rust_Unnamed_1::ALGOLITES_DEFINED.0 as ::core::ffi::c_int as COUNT,
        0 as COUNT,
        1250 as COUNT,
    );
    SetPlotLength(
        plotmap,
        C2Rust_Unnamed_1::SPATHI_DEFINED.0 as ::core::ffi::c_int as COUNT,
        C2Rust_Unnamed_1::ALGOLITES_DEFINED.0 as ::core::ffi::c_int as COUNT,
        5000 as COUNT,
        10000 as COUNT,
    );
    SetPlotLength(
        plotmap,
        C2Rust_Unnamed_1::CHMMR_DEFINED.0 as ::core::ffi::c_int as COUNT,
        C2Rust_Unnamed_1::ILWRATH_DEFINED.0 as ::core::ffi::c_int as COUNT,
        1000 as COUNT,
        3000 as COUNT,
    );
    SetPlotLength(
        plotmap,
        C2Rust_Unnamed_1::CHMMR_DEFINED.0 as ::core::ffi::c_int as COUNT,
        C2Rust_Unnamed_1::PKUNK_DEFINED.0 as ::core::ffi::c_int as COUNT,
        700 as COUNT,
        3500 as COUNT,
    );
    SetPlotLength(
        plotmap,
        C2Rust_Unnamed_1::ILWRATH_DEFINED.0 as ::core::ffi::c_int as COUNT,
        C2Rust_Unnamed_1::PKUNK_DEFINED.0 as ::core::ffi::c_int as COUNT,
        3000 as COUNT,
        6000 as COUNT,
    );
    SetPlotLength(
        plotmap,
        C2Rust_Unnamed_1::MYCON_DEFINED.0 as ::core::ffi::c_int as COUNT,
        C2Rust_Unnamed_1::SUN_DEVICE_DEFINED.0 as ::core::ffi::c_int as COUNT,
        0 as COUNT,
        500 as COUNT,
    );
    SetPlotLength(
        plotmap,
        C2Rust_Unnamed_1::MYCON_DEFINED.0 as ::core::ffi::c_int as COUNT,
        C2Rust_Unnamed_1::EGG_CASE0_DEFINED.0 as ::core::ffi::c_int as COUNT,
        300 as COUNT,
        1250 as COUNT,
    );
    SetPlotLength(
        plotmap,
        C2Rust_Unnamed_1::MYCON_DEFINED.0 as ::core::ffi::c_int as COUNT,
        C2Rust_Unnamed_1::EGG_CASE1_DEFINED.0 as ::core::ffi::c_int as COUNT,
        0 as COUNT,
        500 as COUNT,
    );
    SetPlotLength(
        plotmap,
        C2Rust_Unnamed_1::MYCON_DEFINED.0 as ::core::ffi::c_int as COUNT,
        C2Rust_Unnamed_1::EGG_CASE2_DEFINED.0 as ::core::ffi::c_int as COUNT,
        0 as COUNT,
        800 as COUNT,
    );
    SetPlotLength(
        plotmap,
        C2Rust_Unnamed_1::MYCON_DEFINED.0 as ::core::ffi::c_int as COUNT,
        C2Rust_Unnamed_1::MYCON_TRAP_DEFINED.0 as ::core::ffi::c_int as COUNT,
        1200 as COUNT,
        3500 as COUNT,
    );
    SetPlotLength(
        plotmap,
        C2Rust_Unnamed_1::ORZ_DEFINED.0 as ::core::ffi::c_int as COUNT,
        C2Rust_Unnamed_1::SAMATRA_DEFINED.0 as ::core::ffi::c_int as COUNT,
        3000 as COUNT,
        10000 as COUNT,
    );
    SetPlotLength(
        plotmap,
        C2Rust_Unnamed_1::ANDROSYNTH_DEFINED.0 as ::core::ffi::c_int as COUNT,
        C2Rust_Unnamed_1::ORZ_DEFINED.0 as ::core::ffi::c_int as COUNT,
        0 as COUNT,
        350 as COUNT,
    );
    SetPlotLength(
        plotmap,
        C2Rust_Unnamed_1::ORZ_DEFINED.0 as ::core::ffi::c_int as COUNT,
        C2Rust_Unnamed_1::TAALO_PROTECTOR_DEFINED.0 as ::core::ffi::c_int as COUNT,
        0 as COUNT,
        350 as COUNT,
    );
    SetPlotLength(
        plotmap,
        C2Rust_Unnamed_1::ANDROSYNTH_DEFINED.0 as ::core::ffi::c_int as COUNT,
        C2Rust_Unnamed_1::EXCAVATION_SITE_DEFINED.0 as ::core::ffi::c_int as COUNT,
        0 as COUNT,
        350 as COUNT,
    );
    SetPlotLength(
        plotmap,
        C2Rust_Unnamed_1::ANDROSYNTH_DEFINED.0 as ::core::ffi::c_int as COUNT,
        C2Rust_Unnamed_1::SOL_DEFINED.0 as ::core::ffi::c_int as COUNT,
        2000 as COUNT,
        6000 as COUNT,
    );
    SetPlotLength(
        plotmap,
        C2Rust_Unnamed_1::ANDROSYNTH_DEFINED.0 as ::core::ffi::c_int as COUNT,
        C2Rust_Unnamed_1::START_COLONY_DEFINED.0 as ::core::ffi::c_int as COUNT,
        350 as COUNT,
        1000 as COUNT,
    );
    SetPlotLength(
        plotmap,
        C2Rust_Unnamed_1::PKUNK_DEFINED.0 as ::core::ffi::c_int as COUNT,
        C2Rust_Unnamed_1::YEHAT_DEFINED.0 as ::core::ffi::c_int as COUNT,
        3000 as COUNT,
        6000 as COUNT,
    );
    SetPlotLength(
        plotmap,
        C2Rust_Unnamed_1::SPATHI_DEFINED.0 as ::core::ffi::c_int as COUNT,
        C2Rust_Unnamed_1::SPATHI_MONUMENT_DEFINED.0 as ::core::ffi::c_int as COUNT,
        300 as COUNT,
        1000 as COUNT,
    );
    SetPlotLength(
        plotmap,
        C2Rust_Unnamed_1::MOTHER_ARK_DEFINED.0 as ::core::ffi::c_int as COUNT,
        C2Rust_Unnamed_1::SPATHI_MONUMENT_DEFINED.0 as ::core::ffi::c_int as COUNT,
        300 as COUNT,
        1000 as COUNT,
    );
    SetPlotLength(
        plotmap,
        C2Rust_Unnamed_1::MOTHER_ARK_DEFINED.0 as ::core::ffi::c_int as COUNT,
        C2Rust_Unnamed_1::SPATHI_DEFINED.0 as ::core::ffi::c_int as COUNT,
        1000 as COUNT,
        2000 as COUNT,
    );
    SetPlotLength(
        plotmap,
        C2Rust_Unnamed_1::SUPOX_DEFINED.0 as ::core::ffi::c_int as COUNT,
        C2Rust_Unnamed_1::UTWIG_DEFINED.0 as ::core::ffi::c_int as COUNT,
        300 as COUNT,
        3000 as COUNT,
    );
    SetPlotLength(
        plotmap,
        C2Rust_Unnamed_1::THRADD_DEFINED.0 as ::core::ffi::c_int as COUNT,
        C2Rust_Unnamed_1::AQUA_HELIX_DEFINED.0 as ::core::ffi::c_int as COUNT,
        0 as COUNT,
        700 as COUNT,
    );
    SetPlotLength(
        plotmap,
        C2Rust_Unnamed_1::CHMMR_DEFINED.0 as ::core::ffi::c_int as COUNT,
        C2Rust_Unnamed_1::THRADD_DEFINED.0 as ::core::ffi::c_int as COUNT,
        3000 as COUNT,
        10000 as COUNT,
    );
    SetPlotLength(
        plotmap,
        C2Rust_Unnamed_1::ILWRATH_DEFINED.0 as ::core::ffi::c_int as COUNT,
        C2Rust_Unnamed_1::THRADD_DEFINED.0 as ::core::ffi::c_int as COUNT,
        3000 as COUNT,
        10000 as COUNT,
    );
    SetPlotLength(
        plotmap,
        C2Rust_Unnamed_1::PKUNK_DEFINED.0 as ::core::ffi::c_int as COUNT,
        C2Rust_Unnamed_1::THRADD_DEFINED.0 as ::core::ffi::c_int as COUNT,
        3000 as COUNT,
        10000 as COUNT,
    );
    SetPlotLength(
        plotmap,
        C2Rust_Unnamed_1::SAMATRA_DEFINED.0 as ::core::ffi::c_int as COUNT,
        C2Rust_Unnamed_1::SHIP_VAULT_DEFINED.0 as ::core::ffi::c_int as COUNT,
        0 as COUNT,
        3000 as COUNT,
    );
    SetPlotLength(
        plotmap,
        C2Rust_Unnamed_1::SAMATRA_DEFINED.0 as ::core::ffi::c_int as COUNT,
        C2Rust_Unnamed_1::URQUAN_WRECK_DEFINED.0 as ::core::ffi::c_int as COUNT,
        4000 as COUNT,
        10000 as COUNT,
    );
    SetPlotLength(
        plotmap,
        C2Rust_Unnamed_1::SYREEN_DEFINED.0 as ::core::ffi::c_int as COUNT,
        C2Rust_Unnamed_1::SHIP_VAULT_DEFINED.0 as ::core::ffi::c_int as COUNT,
        1700 as COUNT,
        2100 as COUNT,
    );
    SetPlotLength(
        plotmap,
        C2Rust_Unnamed_1::SAMATRA_DEFINED.0 as ::core::ffi::c_int as COUNT,
        C2Rust_Unnamed_1::URQUAN0_DEFINED.0 as ::core::ffi::c_int as COUNT,
        0 as COUNT,
        2500 as COUNT,
    );
    SetPlotLength(
        plotmap,
        C2Rust_Unnamed_1::SAMATRA_DEFINED.0 as ::core::ffi::c_int as COUNT,
        C2Rust_Unnamed_1::URQUAN1_DEFINED.0 as ::core::ffi::c_int as COUNT,
        0 as COUNT,
        2500 as COUNT,
    );
    SetPlotLength(
        plotmap,
        C2Rust_Unnamed_1::SAMATRA_DEFINED.0 as ::core::ffi::c_int as COUNT,
        C2Rust_Unnamed_1::URQUAN2_DEFINED.0 as ::core::ffi::c_int as COUNT,
        0 as COUNT,
        3000 as COUNT,
    );
    SetPlotLength(
        plotmap,
        C2Rust_Unnamed_1::SAMATRA_DEFINED.0 as ::core::ffi::c_int as COUNT,
        C2Rust_Unnamed_1::KOHRAH0_DEFINED.0 as ::core::ffi::c_int as COUNT,
        0 as COUNT,
        2500 as COUNT,
    );
    SetPlotLength(
        plotmap,
        C2Rust_Unnamed_1::SAMATRA_DEFINED.0 as ::core::ffi::c_int as COUNT,
        C2Rust_Unnamed_1::KOHRAH1_DEFINED.0 as ::core::ffi::c_int as COUNT,
        0 as COUNT,
        2500 as COUNT,
    );
    SetPlotLength(
        plotmap,
        C2Rust_Unnamed_1::SAMATRA_DEFINED.0 as ::core::ffi::c_int as COUNT,
        C2Rust_Unnamed_1::KOHRAH2_DEFINED.0 as ::core::ffi::c_int as COUNT,
        0 as COUNT,
        3000 as COUNT,
    );
    SetPlotLength(
        plotmap,
        C2Rust_Unnamed_1::SAMATRA_DEFINED.0 as ::core::ffi::c_int as COUNT,
        C2Rust_Unnamed_1::DESTROYED_STARBASE_DEFINED.0 as ::core::ffi::c_int as COUNT,
        3000 as COUNT,
        10000 as COUNT,
    );
    SetPlotLength(
        plotmap,
        C2Rust_Unnamed_1::UTWIG_DEFINED.0 as ::core::ffi::c_int as COUNT,
        C2Rust_Unnamed_1::BOMB_DEFINED.0 as ::core::ffi::c_int as COUNT,
        0 as COUNT,
        750 as COUNT,
    );
    SetPlotLength(
        plotmap,
        C2Rust_Unnamed_1::UTWIG_DEFINED.0 as ::core::ffi::c_int as COUNT,
        C2Rust_Unnamed_1::BURVIXESE_DEFINED.0 as ::core::ffi::c_int as COUNT,
        1000 as COUNT,
        4000 as COUNT,
    );
    SetPlotLength(
        plotmap,
        C2Rust_Unnamed_1::VUX_DEFINED.0 as ::core::ffi::c_int as COUNT,
        C2Rust_Unnamed_1::MAIDENS_DEFINED.0 as ::core::ffi::c_int as COUNT,
        0 as COUNT,
        1000 as COUNT,
    );
    SetPlotLength(
        plotmap,
        C2Rust_Unnamed_1::VUX_BEAST_DEFINED.0 as ::core::ffi::c_int as COUNT,
        C2Rust_Unnamed_1::MAIDENS_DEFINED.0 as ::core::ffi::c_int as COUNT,
        5000 as COUNT,
        10000 as COUNT,
    );
    SetPlotLength(
        plotmap,
        C2Rust_Unnamed_1::SHOFIXTI_DEFINED.0 as ::core::ffi::c_int as COUNT,
        C2Rust_Unnamed_1::YEHAT_DEFINED.0 as ::core::ffi::c_int as COUNT,
        700 as COUNT,
        3000 as COUNT,
    );
    SetPlotLength(
        plotmap,
        C2Rust_Unnamed_1::ZOQFOT_DEFINED.0 as ::core::ffi::c_int as COUNT,
        C2Rust_Unnamed_1::SAMATRA_DEFINED.0 as ::core::ffi::c_int as COUNT,
        1500 as COUNT,
        2500 as COUNT,
    );
    SetPlotLength(
        plotmap,
        C2Rust_Unnamed_1::ZOQFOT_DEFINED.0 as ::core::ffi::c_int as COUNT,
        C2Rust_Unnamed_1::ZOQ_SCOUT_DEFINED.0 as ::core::ffi::c_int as COUNT,
        2000 as COUNT,
        5000 as COUNT,
    );
    SetPlotLength(
        plotmap,
        C2Rust_Unnamed_1::ZOQ_SCOUT_DEFINED.0 as ::core::ffi::c_int as COUNT,
        C2Rust_Unnamed_1::SOL_DEFINED.0 as ::core::ffi::c_int as COUNT,
        0 as COUNT,
        1500 as COUNT,
    );
    SetPlotLength(
        plotmap,
        C2Rust_Unnamed_1::ZOQFOT_DEFINED.0 as ::core::ffi::c_int as COUNT,
        C2Rust_Unnamed_1::ZOQ_COLONY0_DEFINED.0 as ::core::ffi::c_int as COUNT,
        0 as COUNT,
        1250 as COUNT,
    );
    SetPlotLength(
        plotmap,
        C2Rust_Unnamed_1::ZOQFOT_DEFINED.0 as ::core::ffi::c_int as COUNT,
        C2Rust_Unnamed_1::ZOQ_COLONY1_DEFINED.0 as ::core::ffi::c_int as COUNT,
        0 as COUNT,
        1000 as COUNT,
    );
    SetPlotLength(
        plotmap,
        C2Rust_Unnamed_1::ZOQFOT_DEFINED.0 as ::core::ffi::c_int as COUNT,
        C2Rust_Unnamed_1::ZOQ_COLONY2_DEFINED.0 as ::core::ffi::c_int as COUNT,
        0 as COUNT,
        1000 as COUNT,
    );
    SetPlotLength(
        plotmap,
        C2Rust_Unnamed_1::ZOQFOT_DEFINED.0 as ::core::ffi::c_int as COUNT,
        C2Rust_Unnamed_1::ZOQ_COLONY3_DEFINED.0 as ::core::ffi::c_int as COUNT,
        0 as COUNT,
        750 as COUNT,
    );
    SetPlotLength(
        plotmap,
        C2Rust_Unnamed_1::SAMATRA_DEFINED.0 as ::core::ffi::c_int as COUNT,
        C2Rust_Unnamed_1::ZOQ_COLONY0_DEFINED.0 as ::core::ffi::c_int as COUNT,
        0 as COUNT,
        2500 as COUNT,
    );
    SetPlotLength(
        plotmap,
        C2Rust_Unnamed_1::SAMATRA_DEFINED.0 as ::core::ffi::c_int as COUNT,
        C2Rust_Unnamed_1::ZOQ_COLONY1_DEFINED.0 as ::core::ffi::c_int as COUNT,
        0 as COUNT,
        2500 as COUNT,
    );
    SetPlotLength(
        plotmap,
        C2Rust_Unnamed_1::SAMATRA_DEFINED.0 as ::core::ffi::c_int as COUNT,
        C2Rust_Unnamed_1::ZOQ_COLONY2_DEFINED.0 as ::core::ffi::c_int as COUNT,
        0 as COUNT,
        3000 as COUNT,
    );
    SetPlotLength(
        plotmap,
        C2Rust_Unnamed_1::SAMATRA_DEFINED.0 as ::core::ffi::c_int as COUNT,
        C2Rust_Unnamed_1::ZOQ_COLONY3_DEFINED.0 as ::core::ffi::c_int as COUNT,
        0 as COUNT,
        3000 as COUNT,
    );
}
#[no_mangle]
pub unsafe extern "C" fn InitMelnormeRainbow(mut plotmap: *mut PLOT_LOCATION) {
    let mut i: COUNT = 0;
    let mut j: COUNT = 0;
    i = C2Rust_Unnamed_1::RAINBOW0_DEFINED.0 as ::core::ffi::c_int as COUNT;
    while i as ::core::ffi::c_int
        <= C2Rust_Unnamed_1::RAINBOW9_DEFINED.0 as ::core::ffi::c_int
    {
        (*plotmap.offset(i as isize)).star = ::core::ptr::null_mut::<STAR_DESC>();
        (*plotmap.offset(i as isize)).star_pt = point {
            x: !(0 as ::core::ffi::c_int) as COORD,
            y: !(0 as ::core::ffi::c_int) as COORD,
        };
        j = C2Rust_Unnamed_1::RAINBOW0_DEFINED.0 as ::core::ffi::c_int as COUNT;
        while j as ::core::ffi::c_int
            <= C2Rust_Unnamed_1::RAINBOW9_DEFINED.0 as ::core::ffi::c_int
        {
            if i as ::core::ffi::c_int != j as ::core::ffi::c_int {
                SetPlotLength(plotmap, i, j, 2500 as COUNT, 10000 as COUNT);
            }
            j = j.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    i = C2Rust_Unnamed_1::MELNORME0_DEFINED.0 as ::core::ffi::c_int as COUNT;
    while i as ::core::ffi::c_int
        <= C2Rust_Unnamed_1::MELNORME8_DEFINED.0 as ::core::ffi::c_int
    {
        (*plotmap.offset(i as isize)).star = ::core::ptr::null_mut::<STAR_DESC>();
        (*plotmap.offset(i as isize)).star_pt = point {
            x: !(0 as ::core::ffi::c_int) as COORD,
            y: !(0 as ::core::ffi::c_int) as COORD,
        };
        j = C2Rust_Unnamed_1::MELNORME0_DEFINED.0 as ::core::ffi::c_int as COUNT;
        while j as ::core::ffi::c_int
            <= C2Rust_Unnamed_1::MELNORME8_DEFINED.0 as ::core::ffi::c_int
        {
            if i as ::core::ffi::c_int != j as ::core::ffi::c_int {
                SetPlotLength(plotmap, i, j, 2750 as COUNT, 10000 as COUNT);
            }
            j = j.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    i = C2Rust_Unnamed_1::RAINBOW0_DEFINED.0 as ::core::ffi::c_int as COUNT;
    while i as ::core::ffi::c_int
        <= C2Rust_Unnamed_1::RAINBOW9_DEFINED.0 as ::core::ffi::c_int
    {
        (*plotmap.offset(i as isize)).dist_sq[i as usize] = (*plotmap.offset(i as isize))
            .dist_sq[i as usize]
            .wrapping_mul(0 as DWORD);
        i = i.wrapping_add(1);
    }
    i = C2Rust_Unnamed_1::MELNORME0_DEFINED.0 as ::core::ffi::c_int as COUNT;
    while i as ::core::ffi::c_int
        <= C2Rust_Unnamed_1::MELNORME8_DEFINED.0 as ::core::ffi::c_int
    {
        (*plotmap.offset(i as isize)).dist_sq[i as usize] = (*plotmap.offset(i as isize))
            .dist_sq[i as usize]
            .wrapping_mul(0 as DWORD);
        i = i.wrapping_add(1);
    }
    SetPlotLength(
        plotmap,
        C2Rust_Unnamed_1::YEHAT_DEFINED.0 as ::core::ffi::c_int as COUNT,
        C2Rust_Unnamed_1::RAINBOW0_DEFINED.0 as ::core::ffi::c_int as COUNT,
        0 as COUNT,
        1000 as COUNT,
    );
    SetPlotLength(
        plotmap,
        C2Rust_Unnamed_1::THRADD_DEFINED.0 as ::core::ffi::c_int as COUNT,
        C2Rust_Unnamed_1::RAINBOW5_DEFINED.0 as ::core::ffi::c_int as COUNT,
        0 as COUNT,
        1000 as COUNT,
    );
    SetPlotLength(
        plotmap,
        C2Rust_Unnamed_1::SLYLANDRO_DEFINED.0 as ::core::ffi::c_int as COUNT,
        C2Rust_Unnamed_1::RAINBOW4_DEFINED.0 as ::core::ffi::c_int as COUNT,
        1500 as COUNT,
        3500 as COUNT,
    );
    SetPlotLength(
        plotmap,
        C2Rust_Unnamed_1::SUPOX_DEFINED.0 as ::core::ffi::c_int as COUNT,
        C2Rust_Unnamed_1::RAINBOW7_DEFINED.0 as ::core::ffi::c_int as COUNT,
        0 as COUNT,
        1000 as COUNT,
    );
    SetPlotLength(
        plotmap,
        C2Rust_Unnamed_1::SOL_DEFINED.0 as ::core::ffi::c_int as COUNT,
        C2Rust_Unnamed_1::MELNORME1_DEFINED.0 as ::core::ffi::c_int as COUNT,
        0 as COUNT,
        1000 as COUNT,
    );
    SetPlotLength(
        plotmap,
        C2Rust_Unnamed_1::THRADD_DEFINED.0 as ::core::ffi::c_int as COUNT,
        C2Rust_Unnamed_1::MELNORME7_DEFINED.0 as ::core::ffi::c_int as COUNT,
        0 as COUNT,
        1000 as COUNT,
    );
}
#[no_mangle]
pub unsafe extern "C" fn CheckValid(
    mut plot: *mut PLOT_LOCATION,
    mut plot_id: COUNT,
) -> BOOLEAN {
    let mut i: COUNT = 0;
    let mut distance_sq: DWORD = 0;
    if plot.is_null()
        || plot_id as ::core::ffi::c_int
            >= C2Rust_Unnamed_1::NUM_PLOTS.0 as ::core::ffi::c_int
    {
        fprintf(
            stderr,
            b"CheckValid (plotmap, plot_id) called %d.\nwith bad data or NULL: PTR\0"
                .as_ptr() as *const ::core::ffi::c_char,
            plot_id as ::core::ffi::c_int,
        );
        return BOOLEAN::FALSE;
    }
    if !(!(*plot.offset(plot_id as isize)).star.is_null()
        && (*plot.offset(plot_id as isize)).star_pt.x as ::core::ffi::c_int
            != !(0 as ::core::ffi::c_int)
        && (*plot.offset(plot_id as isize)).star_pt.y as ::core::ffi::c_int
            != !(0 as ::core::ffi::c_int))
    {
        fprintf(
            stderr,
            b"CheckValid (plotmap, plot_id) called %d.\nwith un-set plot:\0".as_ptr()
                as *const ::core::ffi::c_char,
            plot_id as ::core::ffi::c_int,
        );
        return BOOLEAN::FALSE;
    }
    i = 0 as COUNT;
    while (i as ::core::ffi::c_int) < C2Rust_Unnamed_1::NUM_PLOTS.0 as ::core::ffi::c_int
    {
        if i as ::core::ffi::c_int != plot_id as ::core::ffi::c_int {
            if !(*plot.offset(i as isize)).star.is_null()
                && (*plot.offset(i as isize)).star_pt.x as ::core::ffi::c_int
                    != !(0 as ::core::ffi::c_int)
                && (*plot.offset(i as isize)).star_pt.y as ::core::ffi::c_int
                    != !(0 as ::core::ffi::c_int)
            {
                if !(if (plot_id as ::core::ffi::c_int) < i as ::core::ffi::c_int {
                    (*plot.offset(plot_id as isize)).dist_sq[i as usize]
                } else {
                    (*plot.offset(i as isize)).dist_sq[plot_id as usize]
                } == 0 as DWORD
                    && if plot_id as ::core::ffi::c_int > i as ::core::ffi::c_int {
                        (*plot.offset(plot_id as isize)).dist_sq[i as usize]
                    } else {
                        (*plot.offset(i as isize)).dist_sq[plot_id as usize]
                    } == 100000000 as ::core::ffi::c_int as DWORD)
                {
                    distance_sq = (((*plot.offset(plot_id as isize)).star_pt.x
                        as ::core::ffi::c_int
                        - (*plot.offset(i as isize)).star_pt.x as ::core::ffi::c_int)
                        * ((*plot.offset(plot_id as isize)).star_pt.x
                            as ::core::ffi::c_int
                            - (*plot.offset(i as isize)).star_pt.x as ::core::ffi::c_int)
                        + ((*plot.offset(plot_id as isize)).star_pt.y
                            as ::core::ffi::c_int
                            - (*plot.offset(i as isize)).star_pt.y as ::core::ffi::c_int)
                            * ((*plot.offset(plot_id as isize)).star_pt.y
                                as ::core::ffi::c_int
                                - (*plot.offset(i as isize)).star_pt.y
                                    as ::core::ffi::c_int)) as DWORD;
                    if distance_sq
                        < if (plot_id as ::core::ffi::c_int) < i as ::core::ffi::c_int {
                            (*plot.offset(plot_id as isize)).dist_sq[i as usize]
                        } else {
                            (*plot.offset(i as isize)).dist_sq[plot_id as usize]
                        }
                        || distance_sq
                            > if plot_id as ::core::ffi::c_int > i as ::core::ffi::c_int
                            {
                                (*plot.offset(plot_id as isize)).dist_sq[i as usize]
                            } else {
                                (*plot.offset(i as isize)).dist_sq[plot_id as usize]
                            }
                            && if plot_id as ::core::ffi::c_int > i as ::core::ffi::c_int
                            {
                                (*plot.offset(plot_id as isize)).dist_sq[i as usize]
                            } else {
                                (*plot.offset(i as isize)).dist_sq[plot_id as usize]
                            } > 0 as DWORD
                    {
                        return BOOLEAN::FALSE;
                    }
                }
            }
        }
        i = i.wrapping_add(1);
    }
    return BOOLEAN::TRUE;
}
unsafe extern "C" fn NeedsHabitable(mut sIndex: BYTE) -> BOOLEAN {
    match sIndex as ::core::ffi::c_int {
        2 | 4 | 6 | 16 | 18 | 19 | 21 | 24 | 28 | 31 | 32 | 33 | 34 | 35 | 36 | 38 | 39
        | 40 | 41 | 53 | 54 | 63 | 64 | 65 | 66 | 67 => return BOOLEAN::TRUE,
        _ => return BOOLEAN::FALSE,
    };
}
#[no_mangle]
pub unsafe extern "C" fn Plotify(mut starmap: *mut STAR_DESC, mut star: *mut STAR_DESC) {
    if starmap.is_null() || star.is_null() {
        fprintf(
            stderr,
            b"Plotify (starmap, star) called with NULL PTR.\n\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    extern "C" {
        #[link_name = "starmap_array"]
        static starmap_array_0: [STAR_DESC; 0];
    }
    let mut i: COUNT = 0 as COUNT;
    while (*star).Index as ::core::ffi::c_int
        != (*(&raw const starmap_array as *const STAR_DESC).offset(i as isize)).Index
            as ::core::ffi::c_int
    {
        i = i.wrapping_add(1);
        if i as ::core::ffi::c_int >= 502 as ::core::ffi::c_int {
            return;
        }
    }
    if (((*(&raw const starmap_array as *const STAR_DESC).offset(i as isize)).Type
        as ::core::ffi::c_int
        & ((0xff as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
            & !(((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as BYTE
                as ::core::ffi::c_int)) as BYTE as ::core::ffi::c_int)
        >> 3 as ::core::ffi::c_int) as BYTE as ::core::ffi::c_int
        == C2Rust_Unnamed::SUPER_GIANT_STAR.0 as ::core::ffi::c_int
    {
        (*star).Type = ((C2Rust_Unnamed::SUPER_GIANT_STAR.0 as ::core::ffi::c_int as BYTE
            as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
            & ((0xff as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                & !(((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as BYTE
                    as ::core::ffi::c_int)) as BYTE as ::core::ffi::c_int
            | ((((*star).Type as ::core::ffi::c_int
                & ((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as BYTE
                    as ::core::ffi::c_int) >> 5 as ::core::ffi::c_int) as BYTE
                as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
                & ((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as BYTE
                    as ::core::ffi::c_int
            | ((((*star).Type as ::core::ffi::c_int
                & ((0xff as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                    & !(((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as BYTE
                        as ::core::ffi::c_int
                        | ((0xff as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                            & !(((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int)
                                as BYTE as ::core::ffi::c_int)) as BYTE
                            as ::core::ffi::c_int)) as BYTE as ::core::ffi::c_int)
                >> 0 as ::core::ffi::c_int) as BYTE as ::core::ffi::c_int)
                << 0 as ::core::ffi::c_int
                & ((0xff as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                    & !(((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as BYTE
                        as ::core::ffi::c_int
                        | ((0xff as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                            & !(((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int)
                                as BYTE as ::core::ffi::c_int)) as BYTE
                            as ::core::ffi::c_int)) as BYTE as ::core::ffi::c_int)
            as BYTE;
    }
    if (((*(&raw const starmap_array as *const STAR_DESC).offset(i as isize)).Type
        as ::core::ffi::c_int
        & ((0xff as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
            & !(((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as BYTE
                as ::core::ffi::c_int)) as BYTE as ::core::ffi::c_int)
        >> 3 as ::core::ffi::c_int) as BYTE as ::core::ffi::c_int
        == C2Rust_Unnamed::GIANT_STAR.0 as ::core::ffi::c_int
    {
        (*star).Type = ((C2Rust_Unnamed::GIANT_STAR.0 as ::core::ffi::c_int as BYTE
            as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
            & ((0xff as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                & !(((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as BYTE
                    as ::core::ffi::c_int)) as BYTE as ::core::ffi::c_int
            | ((((*star).Type as ::core::ffi::c_int
                & ((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as BYTE
                    as ::core::ffi::c_int) >> 5 as ::core::ffi::c_int) as BYTE
                as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
                & ((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as BYTE
                    as ::core::ffi::c_int
            | ((((*star).Type as ::core::ffi::c_int
                & ((0xff as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                    & !(((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as BYTE
                        as ::core::ffi::c_int
                        | ((0xff as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                            & !(((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int)
                                as BYTE as ::core::ffi::c_int)) as BYTE
                            as ::core::ffi::c_int)) as BYTE as ::core::ffi::c_int)
                >> 0 as ::core::ffi::c_int) as BYTE as ::core::ffi::c_int)
                << 0 as ::core::ffi::c_int
                & ((0xff as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                    & !(((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as BYTE
                        as ::core::ffi::c_int
                        | ((0xff as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                            & !(((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int)
                                as BYTE as ::core::ffi::c_int)) as BYTE
                            as ::core::ffi::c_int)) as BYTE as ::core::ffi::c_int)
            as BYTE;
    }
    if (((*(&raw const starmap_array as *const STAR_DESC).offset(i as isize)).Type
        as ::core::ffi::c_int
        & ((0xff as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
            & !(((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as BYTE
                as ::core::ffi::c_int)) as BYTE as ::core::ffi::c_int)
        >> 3 as ::core::ffi::c_int) as BYTE as ::core::ffi::c_int
        == C2Rust_Unnamed::DWARF_STAR.0 as ::core::ffi::c_int
    {
        (*star).Type = ((C2Rust_Unnamed::DWARF_STAR.0 as ::core::ffi::c_int as BYTE
            as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
            & ((0xff as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                & !(((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as BYTE
                    as ::core::ffi::c_int)) as BYTE as ::core::ffi::c_int
            | ((((*star).Type as ::core::ffi::c_int
                & ((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as BYTE
                    as ::core::ffi::c_int) >> 5 as ::core::ffi::c_int) as BYTE
                as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
                & ((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as BYTE
                    as ::core::ffi::c_int
            | ((((*star).Type as ::core::ffi::c_int
                & ((0xff as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                    & !(((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as BYTE
                        as ::core::ffi::c_int
                        | ((0xff as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                            & !(((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int)
                                as BYTE as ::core::ffi::c_int)) as BYTE
                            as ::core::ffi::c_int)) as BYTE as ::core::ffi::c_int)
                >> 0 as ::core::ffi::c_int) as BYTE as ::core::ffi::c_int)
                << 0 as ::core::ffi::c_int
                & ((0xff as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                    & !(((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as BYTE
                        as ::core::ffi::c_int
                        | ((0xff as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                            & !(((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int)
                                as BYTE as ::core::ffi::c_int)) as BYTE
                            as ::core::ffi::c_int)) as BYTE as ::core::ffi::c_int)
            as BYTE;
    }
    if NeedsHabitable((*star).Index).0 != 0 {
        let mut star_color: BYTE = (((*star).Type as ::core::ffi::c_int
            & ((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as BYTE
                as ::core::ffi::c_int) >> 5 as ::core::ffi::c_int) as BYTE;
        let mut rand_val: DWORD = GetRandomSeedForStar(star);
        if (((*star).Type as ::core::ffi::c_int
            & ((0xff as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                & !(((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as BYTE
                    as ::core::ffi::c_int)) as BYTE as ::core::ffi::c_int)
            >> 3 as ::core::ffi::c_int) as BYTE as ::core::ffi::c_int
            == C2Rust_Unnamed::DWARF_STAR.0 as ::core::ffi::c_int
            && star_color as ::core::ffi::c_int
                == C2Rust_Unnamed_0::RED_BODY.0 as ::core::ffi::c_int
        {
            let mut color_array: [BYTE; 5] = [
                C2Rust_Unnamed_0::BLUE_BODY.0 as ::core::ffi::c_int as BYTE,
                C2Rust_Unnamed_0::GREEN_BODY.0 as ::core::ffi::c_int as BYTE,
                C2Rust_Unnamed_0::ORANGE_BODY.0 as ::core::ffi::c_int as BYTE,
                C2Rust_Unnamed_0::WHITE_BODY.0 as ::core::ffi::c_int as BYTE,
                C2Rust_Unnamed_0::YELLOW_BODY.0 as ::core::ffi::c_int as BYTE,
            ];
            (*star).Type = (((((*star).Type as ::core::ffi::c_int
                & ((0xff as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    & !(((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as BYTE
                        as ::core::ffi::c_int)) as BYTE as ::core::ffi::c_int)
                >> 3 as ::core::ffi::c_int) as BYTE as ::core::ffi::c_int)
                << 3 as ::core::ffi::c_int
                & ((0xff as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    & !(((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as BYTE
                        as ::core::ffi::c_int)) as BYTE as ::core::ffi::c_int
                | (color_array[rand_val.wrapping_rem(5 as DWORD) as usize]
                    as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
                    & ((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as BYTE
                        as ::core::ffi::c_int
                | ((((*star).Type as ::core::ffi::c_int
                    & ((0xff as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                        & !(((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int)
                            as BYTE as ::core::ffi::c_int
                            | ((0xff as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                & !(((0xff as ::core::ffi::c_int)
                                    << 5 as ::core::ffi::c_int) as BYTE as ::core::ffi::c_int))
                                as BYTE as ::core::ffi::c_int)) as BYTE
                        as ::core::ffi::c_int) >> 0 as ::core::ffi::c_int) as BYTE
                    as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                    & ((0xff as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                        & !(((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int)
                            as BYTE as ::core::ffi::c_int
                            | ((0xff as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                & !(((0xff as ::core::ffi::c_int)
                                    << 5 as ::core::ffi::c_int) as BYTE as ::core::ffi::c_int))
                                as BYTE as ::core::ffi::c_int)) as BYTE
                        as ::core::ffi::c_int) as BYTE;
        } else if (((*star).Type as ::core::ffi::c_int
            & ((0xff as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                & !(((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as BYTE
                    as ::core::ffi::c_int)) as BYTE as ::core::ffi::c_int)
            >> 3 as ::core::ffi::c_int) as BYTE as ::core::ffi::c_int
            == C2Rust_Unnamed::GIANT_STAR.0 as ::core::ffi::c_int
            && (star_color as ::core::ffi::c_int
                == C2Rust_Unnamed_0::BLUE_BODY.0 as ::core::ffi::c_int
                || star_color as ::core::ffi::c_int
                    == C2Rust_Unnamed_0::GREEN_BODY.0 as ::core::ffi::c_int
                || star_color as ::core::ffi::c_int
                    == C2Rust_Unnamed_0::WHITE_BODY.0 as ::core::ffi::c_int)
        {
            let mut color_array_0: [BYTE; 3] = [
                C2Rust_Unnamed_0::ORANGE_BODY.0 as ::core::ffi::c_int as BYTE,
                C2Rust_Unnamed_0::RED_BODY.0 as ::core::ffi::c_int as BYTE,
                C2Rust_Unnamed_0::YELLOW_BODY.0 as ::core::ffi::c_int as BYTE,
            ];
            (*star).Type = (((((*star).Type as ::core::ffi::c_int
                & ((0xff as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    & !(((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as BYTE
                        as ::core::ffi::c_int)) as BYTE as ::core::ffi::c_int)
                >> 3 as ::core::ffi::c_int) as BYTE as ::core::ffi::c_int)
                << 3 as ::core::ffi::c_int
                & ((0xff as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    & !(((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as BYTE
                        as ::core::ffi::c_int)) as BYTE as ::core::ffi::c_int
                | (color_array_0[rand_val.wrapping_rem(3 as DWORD) as usize]
                    as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
                    & ((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as BYTE
                        as ::core::ffi::c_int
                | ((((*star).Type as ::core::ffi::c_int
                    & ((0xff as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                        & !(((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int)
                            as BYTE as ::core::ffi::c_int
                            | ((0xff as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                & !(((0xff as ::core::ffi::c_int)
                                    << 5 as ::core::ffi::c_int) as BYTE as ::core::ffi::c_int))
                                as BYTE as ::core::ffi::c_int)) as BYTE
                        as ::core::ffi::c_int) >> 0 as ::core::ffi::c_int) as BYTE
                    as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                    & ((0xff as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                        & !(((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int)
                            as BYTE as ::core::ffi::c_int
                            | ((0xff as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                & !(((0xff as ::core::ffi::c_int)
                                    << 5 as ::core::ffi::c_int) as BYTE as ::core::ffi::c_int))
                                as BYTE as ::core::ffi::c_int)) as BYTE
                        as ::core::ffi::c_int) as BYTE;
        }
    }
    if (*star).Index as ::core::ffi::c_int
        == C2Rust_Unnamed_1::SOL_DEFINED.0 as ::core::ffi::c_int
    {
        (*star).Type = (*(&raw const starmap_array as *const STAR_DESC)
            .offset(i as isize))
            .Type;
    }
    if (*star).Index as ::core::ffi::c_int
        == C2Rust_Unnamed_1::ILWRATH_DEFINED.0 as ::core::ffi::c_int
    {
        let mut newcolor: COUNT = (((((*star).Type as ::core::ffi::c_int
            & ((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as BYTE
                as ::core::ffi::c_int) >> 5 as ::core::ffi::c_int) as BYTE
            as ::core::ffi::c_int + optCustomSeed) % 5 as ::core::ffi::c_int) as COUNT;
        if optCustomSeed % 44 as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            || optCustomSeed % 100 as ::core::ffi::c_int == 44 as ::core::ffi::c_int
        {
            (*star).Type = (*(&raw const starmap_array as *const STAR_DESC)
                .offset(i as isize))
                .Type;
        } else {
            (*star).Type = (((((*star).Type as ::core::ffi::c_int
                & ((0xff as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    & !(((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as BYTE
                        as ::core::ffi::c_int)) as BYTE as ::core::ffi::c_int)
                >> 3 as ::core::ffi::c_int) as BYTE as ::core::ffi::c_int)
                << 3 as ::core::ffi::c_int
                & ((0xff as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    & !(((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as BYTE
                        as ::core::ffi::c_int)) as BYTE as ::core::ffi::c_int
                | ((if newcolor as ::core::ffi::c_int
                    >= C2Rust_Unnamed_0::GREEN_BODY.0 as ::core::ffi::c_int
                {
                    newcolor as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                } else {
                    newcolor as ::core::ffi::c_int
                }) as BYTE as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
                    & ((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as BYTE
                        as ::core::ffi::c_int
                | ((((*star).Type as ::core::ffi::c_int
                    & ((0xff as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                        & !(((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int)
                            as BYTE as ::core::ffi::c_int
                            | ((0xff as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                & !(((0xff as ::core::ffi::c_int)
                                    << 5 as ::core::ffi::c_int) as BYTE as ::core::ffi::c_int))
                                as BYTE as ::core::ffi::c_int)) as BYTE
                        as ::core::ffi::c_int) >> 0 as ::core::ffi::c_int) as BYTE
                    as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                    & ((0xff as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                        & !(((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int)
                            as BYTE as ::core::ffi::c_int
                            | ((0xff as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                & !(((0xff as ::core::ffi::c_int)
                                    << 5 as ::core::ffi::c_int) as BYTE as ::core::ffi::c_int))
                                as BYTE as ::core::ffi::c_int)) as BYTE
                        as ::core::ffi::c_int) as BYTE;
        }
    }
    if (*star).Index as ::core::ffi::c_int
        == C2Rust_Unnamed_1::SPATHI_DEFINED.0 as ::core::ffi::c_int
    {
        if (((*star).Type as ::core::ffi::c_int
            & ((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as BYTE
                as ::core::ffi::c_int) >> 5 as ::core::ffi::c_int) as BYTE
            as ::core::ffi::c_int == C2Rust_Unnamed_0::WHITE_BODY.0 as ::core::ffi::c_int
        {
            (*star).Type = (((((*star).Type as ::core::ffi::c_int
                & ((0xff as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    & !(((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as BYTE
                        as ::core::ffi::c_int)) as BYTE as ::core::ffi::c_int)
                >> 3 as ::core::ffi::c_int) as BYTE as ::core::ffi::c_int)
                << 3 as ::core::ffi::c_int
                & ((0xff as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    & !(((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as BYTE
                        as ::core::ffi::c_int)) as BYTE as ::core::ffi::c_int
                | (C2Rust_Unnamed_0::YELLOW_BODY.0 as ::core::ffi::c_int as BYTE
                    as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
                    & ((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as BYTE
                        as ::core::ffi::c_int
                | ((((*star).Type as ::core::ffi::c_int
                    & ((0xff as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                        & !(((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int)
                            as BYTE as ::core::ffi::c_int
                            | ((0xff as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                & !(((0xff as ::core::ffi::c_int)
                                    << 5 as ::core::ffi::c_int) as BYTE as ::core::ffi::c_int))
                                as BYTE as ::core::ffi::c_int)) as BYTE
                        as ::core::ffi::c_int) >> 0 as ::core::ffi::c_int) as BYTE
                    as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                    & ((0xff as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                        & !(((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int)
                            as BYTE as ::core::ffi::c_int
                            | ((0xff as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                & !(((0xff as ::core::ffi::c_int)
                                    << 5 as ::core::ffi::c_int) as BYTE as ::core::ffi::c_int))
                                as BYTE as ::core::ffi::c_int)) as BYTE
                        as ::core::ffi::c_int) as BYTE;
        }
        if (((*star).Type as ::core::ffi::c_int
            & ((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as BYTE
                as ::core::ffi::c_int) >> 5 as ::core::ffi::c_int) as BYTE
            as ::core::ffi::c_int == C2Rust_Unnamed_0::BLUE_BODY.0 as ::core::ffi::c_int
        {
            (*star).Type = (((((*star).Type as ::core::ffi::c_int
                & ((0xff as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    & !(((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as BYTE
                        as ::core::ffi::c_int)) as BYTE as ::core::ffi::c_int)
                >> 3 as ::core::ffi::c_int) as BYTE as ::core::ffi::c_int)
                << 3 as ::core::ffi::c_int
                & ((0xff as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    & !(((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as BYTE
                        as ::core::ffi::c_int)) as BYTE as ::core::ffi::c_int
                | (C2Rust_Unnamed_0::ORANGE_BODY.0 as ::core::ffi::c_int as BYTE
                    as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
                    & ((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as BYTE
                        as ::core::ffi::c_int
                | ((((*star).Type as ::core::ffi::c_int
                    & ((0xff as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                        & !(((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int)
                            as BYTE as ::core::ffi::c_int
                            | ((0xff as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                & !(((0xff as ::core::ffi::c_int)
                                    << 5 as ::core::ffi::c_int) as BYTE as ::core::ffi::c_int))
                                as BYTE as ::core::ffi::c_int)) as BYTE
                        as ::core::ffi::c_int) >> 0 as ::core::ffi::c_int) as BYTE
                    as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                    & ((0xff as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                        & !(((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int)
                            as BYTE as ::core::ffi::c_int
                            | ((0xff as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                & !(((0xff as ::core::ffi::c_int)
                                    << 5 as ::core::ffi::c_int) as BYTE as ::core::ffi::c_int))
                                as BYTE as ::core::ffi::c_int)) as BYTE
                        as ::core::ffi::c_int) as BYTE;
        }
        if (((*star).Type as ::core::ffi::c_int
            & ((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as BYTE
                as ::core::ffi::c_int) >> 5 as ::core::ffi::c_int) as BYTE
            as ::core::ffi::c_int == C2Rust_Unnamed_0::GREEN_BODY.0 as ::core::ffi::c_int
        {
            (*star).Type = (((((*star).Type as ::core::ffi::c_int
                & ((0xff as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    & !(((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as BYTE
                        as ::core::ffi::c_int)) as BYTE as ::core::ffi::c_int)
                >> 3 as ::core::ffi::c_int) as BYTE as ::core::ffi::c_int)
                << 3 as ::core::ffi::c_int
                & ((0xff as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    & !(((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as BYTE
                        as ::core::ffi::c_int)) as BYTE as ::core::ffi::c_int
                | (C2Rust_Unnamed_0::RED_BODY.0 as ::core::ffi::c_int as BYTE
                    as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
                    & ((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as BYTE
                        as ::core::ffi::c_int
                | ((((*star).Type as ::core::ffi::c_int
                    & ((0xff as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                        & !(((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int)
                            as BYTE as ::core::ffi::c_int
                            | ((0xff as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                & !(((0xff as ::core::ffi::c_int)
                                    << 5 as ::core::ffi::c_int) as BYTE as ::core::ffi::c_int))
                                as BYTE as ::core::ffi::c_int)) as BYTE
                        as ::core::ffi::c_int) >> 0 as ::core::ffi::c_int) as BYTE
                    as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                    & ((0xff as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                        & !(((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int)
                            as BYTE as ::core::ffi::c_int
                            | ((0xff as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                & !(((0xff as ::core::ffi::c_int)
                                    << 5 as ::core::ffi::c_int) as BYTE as ::core::ffi::c_int))
                                as BYTE as ::core::ffi::c_int)) as BYTE
                        as ::core::ffi::c_int) as BYTE;
        }
    }
    if (*star).Index as ::core::ffi::c_int
        == C2Rust_Unnamed_1::SUPOX_DEFINED.0 as ::core::ffi::c_int
    {
        let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while C2Rust_Unnamed_1::RAINBOW0_DEFINED.0 as ::core::ffi::c_int
            != (*starmap.offset(j as isize)).Index as ::core::ffi::c_int
        {
            j += 1;
            if j >= 502 as ::core::ffi::c_int {
                return;
            }
        }
        (*star).Type = ((C2Rust_Unnamed::DWARF_STAR.0 as ::core::ffi::c_int as BYTE
            as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
            & ((0xff as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                & !(((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as BYTE
                    as ::core::ffi::c_int)) as BYTE as ::core::ffi::c_int
            | ((((((*starmap.offset(j as isize)).Type as ::core::ffi::c_int
                & ((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as BYTE
                    as ::core::ffi::c_int) >> 5 as ::core::ffi::c_int) as BYTE
                as ::core::ffi::c_int + 6 as ::core::ffi::c_int
                + (((*starmap.offset(j as isize)).Type as ::core::ffi::c_int
                    & ((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as BYTE
                        as ::core::ffi::c_int) >> 5 as ::core::ffi::c_int) as BYTE
                    as ::core::ffi::c_int % 2 as ::core::ffi::c_int
                - ((((*starmap.offset(j as isize)).Type as ::core::ffi::c_int
                    & ((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as BYTE
                        as ::core::ffi::c_int) >> 5 as ::core::ffi::c_int) as BYTE
                    as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                    % 2 as ::core::ffi::c_int) % 6 as ::core::ffi::c_int) as BYTE
                as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
                & ((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as BYTE
                    as ::core::ffi::c_int
            | ((((*star).Type as ::core::ffi::c_int
                & ((0xff as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                    & !(((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as BYTE
                        as ::core::ffi::c_int
                        | ((0xff as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                            & !(((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int)
                                as BYTE as ::core::ffi::c_int)) as BYTE
                            as ::core::ffi::c_int)) as BYTE as ::core::ffi::c_int)
                >> 0 as ::core::ffi::c_int) as BYTE as ::core::ffi::c_int)
                << 0 as ::core::ffi::c_int
                & ((0xff as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                    & !(((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as BYTE
                        as ::core::ffi::c_int
                        | ((0xff as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                            & !(((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int)
                                as BYTE as ::core::ffi::c_int)) as BYTE
                            as ::core::ffi::c_int)) as BYTE as ::core::ffi::c_int)
            as BYTE;
    }
    if (*star).Index as ::core::ffi::c_int
        > C2Rust_Unnamed_1::RAINBOW0_DEFINED.0 as ::core::ffi::c_int
        && (*star).Index as ::core::ffi::c_int
            <= C2Rust_Unnamed_1::RAINBOW9_DEFINED.0 as ::core::ffi::c_int
    {
        let mut j_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while C2Rust_Unnamed_1::RAINBOW0_DEFINED.0 as ::core::ffi::c_int
            != (*starmap.offset(j_0 as isize)).Index as ::core::ffi::c_int
        {
            j_0 += 1;
            if j_0 >= 502 as ::core::ffi::c_int {
                return;
            }
        }
        (*star).Type = (((((*star).Type as ::core::ffi::c_int
            & ((0xff as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                & !(((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as BYTE
                    as ::core::ffi::c_int)) as BYTE as ::core::ffi::c_int)
            >> 3 as ::core::ffi::c_int) as BYTE as ::core::ffi::c_int)
            << 3 as ::core::ffi::c_int
            & ((0xff as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                & !(((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as BYTE
                    as ::core::ffi::c_int)) as BYTE as ::core::ffi::c_int
            | ((((*starmap.offset(j_0 as isize)).Type as ::core::ffi::c_int
                & ((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as BYTE
                    as ::core::ffi::c_int) >> 5 as ::core::ffi::c_int) as BYTE
                as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
                & ((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as BYTE
                    as ::core::ffi::c_int
            | ((((*star).Type as ::core::ffi::c_int
                & ((0xff as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                    & !(((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as BYTE
                        as ::core::ffi::c_int
                        | ((0xff as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                            & !(((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int)
                                as BYTE as ::core::ffi::c_int)) as BYTE
                            as ::core::ffi::c_int)) as BYTE as ::core::ffi::c_int)
                >> 0 as ::core::ffi::c_int) as BYTE as ::core::ffi::c_int)
                << 0 as ::core::ffi::c_int
                & ((0xff as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                    & !(((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as BYTE
                        as ::core::ffi::c_int
                        | ((0xff as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                            & !(((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int)
                                as BYTE as ::core::ffi::c_int)) as BYTE
                            as ::core::ffi::c_int)) as BYTE as ::core::ffi::c_int)
            as BYTE;
    }
    if (*star).Index as ::core::ffi::c_int
        >= C2Rust_Unnamed_1::MELNORME0_DEFINED.0 as ::core::ffi::c_int
        && (*star).Index as ::core::ffi::c_int
            <= C2Rust_Unnamed_1::MELNORME8_DEFINED.0 as ::core::ffi::c_int
        && (*star).Prefix as ::core::ffi::c_int > 1 as ::core::ffi::c_int
    {
        let mut j_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while (*starmap.offset(j_1 as isize)).Postfix as ::core::ffi::c_int
            != (*star).Postfix as ::core::ffi::c_int
            || (*starmap.offset(j_1 as isize)).Prefix as ::core::ffi::c_int
                != 1 as ::core::ffi::c_int
        {
            j_1 += 1;
            if j_1 >= 502 as ::core::ffi::c_int {
                return;
            }
        }
        (*starmap.offset(j_1 as isize)).Prefix = (*star).Prefix;
        (*star).Prefix = 1 as BYTE;
    }
    if (*star).Index as ::core::ffi::c_int
        == C2Rust_Unnamed_1::SOL_DEFINED.0 as ::core::ffi::c_int
    {
        let mut j_2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while (*starmap.offset(j_2 as isize)).Postfix as ::core::ffi::c_int
            != 129 as ::core::ffi::c_int
        {
            j_2 += 1;
            if j_2 >= 502 as ::core::ffi::c_int {
                return;
            }
        }
        if (*star).Prefix as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (*starmap.offset(j_2 as isize)).Postfix = (*star).Postfix;
            (*star).Postfix = 129 as BYTE;
        } else {
            let mut prefix: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut k: ::core::ffi::c_int = 0;
            k = 0 as ::core::ffi::c_int;
            while k < 502 as ::core::ffi::c_int {
                if (*starmap.offset(k as isize)).Postfix as ::core::ffi::c_int
                    == (*star).Postfix as ::core::ffi::c_int
                    && (*starmap.offset(k as isize)).Prefix as ::core::ffi::c_int
                        > (*star).Prefix as ::core::ffi::c_int
                {
                    (*starmap.offset(k as isize)).Prefix = (*starmap.offset(k as isize))
                        .Prefix
                        .wrapping_sub(1);
                }
                k += 1;
            }
            (*star).Prefix = 0 as BYTE;
            (*star).Postfix = 129 as BYTE;
            let mut pStr: *mut STAR_DESC = FindNearestConstellation(
                starmap,
                (*starmap.offset(j_2 as isize)).star_pt,
            );
            k = 0 as ::core::ffi::c_int;
            while k < 502 as ::core::ffi::c_int {
                if (*starmap.offset(k as isize)).Postfix as ::core::ffi::c_int
                    == (*pStr).Postfix as ::core::ffi::c_int
                    && (*starmap.offset(k as isize)).Prefix as ::core::ffi::c_int
                        > prefix
                {
                    prefix = (*starmap.offset(k as isize)).Prefix as ::core::ffi::c_int;
                }
                k += 1;
            }
            (*starmap.offset(j_2 as isize)).Postfix = (*pStr).Postfix;
            (*starmap.offset(j_2 as isize)).Prefix = (if prefix
                >= 14 as ::core::ffi::c_int
            {
                14 as ::core::ffi::c_int
            } else {
                prefix + 1 as ::core::ffi::c_int
            }) as BYTE;
            Plotify(starmap, starmap.offset(j_2 as isize));
        }
    }
    if (*star).Index as ::core::ffi::c_int
        == C2Rust_Unnamed_1::ALGOLITES_DEFINED.0 as ::core::ffi::c_int
    {
        let mut j_3: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        j_3 = 0 as ::core::ffi::c_int;
        while j_3 < 502 as ::core::ffi::c_int {
            if starmap.offset(j_3 as isize) != star {
                if (*starmap.offset(j_3 as isize)).Postfix as ::core::ffi::c_int
                    == (*star).Postfix as ::core::ffi::c_int
                {
                    (*starmap.offset(j_3 as isize)).Postfix = 113 as BYTE;
                } else if (*starmap.offset(j_3 as isize)).Postfix as ::core::ffi::c_int
                    == 113 as ::core::ffi::c_int
                {
                    (*starmap.offset(j_3 as isize)).Postfix = (*star).Postfix;
                }
            }
            j_3 += 1;
        }
        (*star).Postfix = 113 as BYTE;
    }
}
#[no_mangle]
pub unsafe extern "C" fn SeedPlot(
    mut plotmap: *mut PLOT_LOCATION,
    mut starmap: *mut STAR_DESC,
) -> COUNT {
    static mut timer_running: BOOLEAN = BOOLEAN::FALSE;
    static mut timer: clock_t = 0;
    if plotmap.is_null() || starmap.is_null() {
        fprintf(
            stderr,
            b"SeedPlot (plotmap, starmap) called with NULL PTR.\n\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return 0 as COUNT;
    }
    let mut my_clock: BOOLEAN = BOOLEAN::FALSE;
    let mut rand_val: UWORD = 0;
    let mut plot_id: COUNT = 0;
    let mut star_id: COUNT = 0;
    let mut i: COUNT = 0;
    let mut return_id: COUNT = 0;
    let mut timelimit: COUNT = (if GlobData.Game_state.CurrentActivity
        as ::core::ffi::c_int != 0
    {
        600 as ::core::ffi::c_int
    } else {
        20 as ::core::ffi::c_int
    }) as COUNT;
    if StarGenRNG.is_null() {
        fprintf(
            stderr,
            b"****SeedPlot creating a STAR GEN RNG****\n\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        StarGenRNG = RandomContext_New();
    }
    if timer_running.0 == 0 {
        timer_running = BOOLEAN::TRUE;
        next_plot = !(0 as ::core::ffi::c_int) as COUNT;
        my_clock = BOOLEAN::TRUE;
        timer = clock();
        RandomContext_SeedRandom(StarGenRNG, optCustomSeed as DWORD);
        i = 1 as COUNT;
        while (i as ::core::ffi::c_int)
            < C2Rust_Unnamed_1::NUM_PLOTS.0 as ::core::ffi::c_int
        {
            (*plotmap.offset(i as isize)).star = ::core::ptr::null_mut::<STAR_DESC>();
            i = i.wrapping_add(1);
        }
        if (*plotmap
            .offset(C2Rust_Unnamed_1::ARILOU_DEFINED.0 as ::core::ffi::c_int as isize))
            .star_pt
            .x as ::core::ffi::c_int != !(0 as ::core::ffi::c_int)
            && (*plotmap
                .offset(
                    C2Rust_Unnamed_1::ARILOU_DEFINED.0 as ::core::ffi::c_int as isize,
                ))
                .star_pt
                .y as ::core::ffi::c_int != !(0 as ::core::ffi::c_int)
        {
            (*plotmap
                .offset(
                    C2Rust_Unnamed_1::ARILOU_DEFINED.0 as ::core::ffi::c_int as isize,
                ))
                .star = starmap
                .offset(
                    (502 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                        + 15 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                );
        }
    } else if (clock() - timer) / 100000 as ::core::ffi::c_int
        > timelimit as ::core::ffi::c_int
    {
        fprintf(
            stderr,
            b"TIME'S UP!  Giving up on seed %d.\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            optCustomSeed,
        );
        timer_running = BOOLEAN::FALSE;
        return (C2Rust_Unnamed_1::NUM_PLOTS.0 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int) as COUNT;
    }
    rand_val = RandomContext_Random(StarGenRNG) as UWORD;
    plot_id = GetNextPlot(plotmap);
    if plot_id as ::core::ffi::c_int
        == C2Rust_Unnamed_1::NUM_PLOTS.0 as ::core::ffi::c_int
    {
        timer_running = BOOLEAN::FALSE;
        i = 1 as COUNT;
        while (i as ::core::ffi::c_int)
            < C2Rust_Unnamed_1::NUM_PLOTS.0 as ::core::ffi::c_int
        {
            if (*plotmap.offset(i as isize)).star.is_null()
                || (*plotmap.offset(i as isize)).star
                    != FindNearestStar(starmap, (*plotmap.offset(i as isize)).star_pt)
            {
                print_plot_id(i);
                fprintf(
                    stderr,
                    b" star ptr was unassigned, corrected.\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                (*plotmap.offset(i as isize)).star = FindNearestStar(
                    starmap,
                    (*plotmap.offset(i as isize)).star_pt,
                );
            }
            if (*plotmap.offset(i as isize)).star_pt.x as ::core::ffi::c_int
                != (*(*plotmap.offset(i as isize)).star).star_pt.x as ::core::ffi::c_int
                || (*plotmap.offset(i as isize)).star_pt.y as ::core::ffi::c_int
                    != (*(*plotmap.offset(i as isize)).star).star_pt.y
                        as ::core::ffi::c_int
            {
                print_plot_id(i);
                fprintf(
                    stderr,
                    b" coords don't match star ptr, corrected.\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                (*plotmap.offset(i as isize)).star_pt = (*(*plotmap.offset(i as isize))
                    .star)
                    .star_pt;
            }
            if (*(*plotmap.offset(i as isize)).star).Index as ::core::ffi::c_int
                > 0 as ::core::ffi::c_int
                && (*(*plotmap.offset(i as isize)).star).Index as ::core::ffi::c_int
                    != i as ::core::ffi::c_int
            {
                print_plot_id(i);
                fprintf(
                    stderr,
                    b" plot wants to be at star %05.1f : %05.1f but plot ID \0".as_ptr()
                        as *const ::core::ffi::c_char,
                    ((*plotmap.offset(i as isize)).star_pt.x as ::core::ffi::c_float
                        / 10 as ::core::ffi::c_int as ::core::ffi::c_float)
                        as ::core::ffi::c_double,
                    ((*plotmap.offset(i as isize)).star_pt.y as ::core::ffi::c_float
                        / 10 as ::core::ffi::c_int as ::core::ffi::c_float)
                        as ::core::ffi::c_double,
                );
                print_plot_id((*(*plotmap.offset(i as isize)).star).Index as COUNT);
                fprintf(
                    stderr,
                    b" is there, ABORT!!!!\n\0".as_ptr() as *const ::core::ffi::c_char,
                );
                return i;
            }
            if (*(*plotmap.offset(i as isize)).star).Index as ::core::ffi::c_int
                != i as ::core::ffi::c_int
            {
                print_plot_id(i);
                fprintf(
                    stderr,
                    b" plot location was missing from star, corrected.\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                (*(*plotmap.offset(i as isize)).star).Index = i as BYTE;
                Plotify(starmap, (*plotmap.offset(i as isize)).star);
            }
            i = i.wrapping_add(1);
        }
        return C2Rust_Unnamed_1::NUM_PLOTS.0 as ::core::ffi::c_int as COUNT;
    }
    if (*plotmap.offset(plot_id as isize)).star_pt.x as ::core::ffi::c_int
        != !(0 as ::core::ffi::c_int)
        && (*plotmap.offset(plot_id as isize)).star_pt.y as ::core::ffi::c_int
            != !(0 as ::core::ffi::c_int)
    {
        (*plotmap.offset(plot_id as isize)).star = FindNearestStar(
            starmap,
            (*plotmap.offset(plot_id as isize)).star_pt,
        );
        if (*(*plotmap.offset(plot_id as isize)).star).Index as ::core::ffi::c_int != 0
            && (*(*plotmap.offset(plot_id as isize)).star).Index as ::core::ffi::c_int
                != plot_id as ::core::ffi::c_int
        {
            print_plot_id(plot_id);
            fprintf(
                stderr,
                b" cannot be placed in location %05.1f : %05.1f due to existing plot \0"
                    .as_ptr() as *const ::core::ffi::c_char,
                ((*(*plotmap.offset(plot_id as isize)).star).star_pt.x
                    as ::core::ffi::c_float
                    / 10 as ::core::ffi::c_int as ::core::ffi::c_float)
                    as ::core::ffi::c_double,
                ((*(*plotmap.offset(plot_id as isize)).star).star_pt.y
                    as ::core::ffi::c_float
                    / 10 as ::core::ffi::c_int as ::core::ffi::c_float)
                    as ::core::ffi::c_double,
            );
            print_plot_id((*(*plotmap.offset(plot_id as isize)).star).Index as COUNT);
            fprintf(stderr, b", aborting...\n\0".as_ptr() as *const ::core::ffi::c_char);
            return plot_id;
        }
        (*plotmap.offset(plot_id as isize)).star_pt = (*(*plotmap
            .offset(plot_id as isize))
            .star)
            .star_pt;
        (*(*plotmap.offset(plot_id as isize)).star).Index = plot_id as BYTE;
        return_id = SeedPlot(plotmap, starmap);
        if return_id as ::core::ffi::c_int
            == C2Rust_Unnamed_1::NUM_PLOTS.0 as ::core::ffi::c_int
        {
            timer_running = BOOLEAN::FALSE;
            Plotify(starmap, (*plotmap.offset(plot_id as isize)).star);
            return return_id;
        }
        (*(*plotmap.offset(plot_id as isize)).star).Index = 0 as BYTE;
        (*plotmap.offset(plot_id as isize)).star = ::core::ptr::null_mut::<STAR_DESC>();
        if my_clock.0 != 0 {
            fprintf(
                stderr,
                b"Complete failure, stopping clock.\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            timer_running = BOOLEAN::FALSE;
        }
        return return_id;
    }
    i = 0 as COUNT;
    while (i as ::core::ffi::c_int) < 502 as ::core::ffi::c_int {
        's_300: {
            if plot_id as ::core::ffi::c_int
                == C2Rust_Unnamed_1::ARILOU_DEFINED.0 as ::core::ffi::c_int
            {
                star_id = ((rand_val as ::core::ffi::c_int
                    + i as ::core::ffi::c_int * 89 as ::core::ffi::c_int)
                    % 100 as ::core::ffi::c_int) as COUNT;
                (*plotmap
                    .offset(
                        C2Rust_Unnamed_1::ARILOU_DEFINED.0 as ::core::ffi::c_int as isize,
                    ))
                    .star_pt = point {
                    x: (star_id as ::core::ffi::c_int / 10 as ::core::ffi::c_int
                        * 1000 as ::core::ffi::c_int
                        + rand_val as BYTE as ::core::ffi::c_int
                            * 100 as ::core::ffi::c_int / 256 as ::core::ffi::c_int)
                        as COORD,
                    y: (star_id as ::core::ffi::c_int % 10 as ::core::ffi::c_int
                        * 1000 as ::core::ffi::c_int
                        + (rand_val as ::core::ffi::c_int >> 8 as ::core::ffi::c_int)
                            as BYTE as ::core::ffi::c_int * 100 as ::core::ffi::c_int
                            / 256 as ::core::ffi::c_int) as COORD,
                };
                (*plotmap
                    .offset(
                        C2Rust_Unnamed_1::ARILOU_DEFINED.0 as ::core::ffi::c_int as isize,
                    ))
                    .star = FindNearestStar(
                    starmap,
                    (*plotmap
                        .offset(
                            C2Rust_Unnamed_1::ARILOU_DEFINED.0 as ::core::ffi::c_int
                                as isize,
                        ))
                        .star_pt,
                );
                if !(*plotmap
                    .offset(
                        C2Rust_Unnamed_1::ARILOU_DEFINED.0 as ::core::ffi::c_int as isize,
                    ))
                    .star
                    .is_null()
                    && ((*plotmap
                        .offset(
                            C2Rust_Unnamed_1::ARILOU_DEFINED.0 as ::core::ffi::c_int
                                as isize,
                        ))
                        .star_pt
                        .x as ::core::ffi::c_int
                        - (*(*plotmap
                            .offset(
                                C2Rust_Unnamed_1::ARILOU_DEFINED.0 as ::core::ffi::c_int
                                    as isize,
                            ))
                            .star)
                            .star_pt
                            .x as ::core::ffi::c_int)
                        * ((*plotmap
                            .offset(
                                C2Rust_Unnamed_1::ARILOU_DEFINED.0 as ::core::ffi::c_int
                                    as isize,
                            ))
                            .star_pt
                            .x as ::core::ffi::c_int
                            - (*(*plotmap
                                .offset(
                                    C2Rust_Unnamed_1::ARILOU_DEFINED.0 as ::core::ffi::c_int
                                        as isize,
                                ))
                                .star)
                                .star_pt
                                .x as ::core::ffi::c_int)
                        + ((*plotmap
                            .offset(
                                C2Rust_Unnamed_1::ARILOU_DEFINED.0 as ::core::ffi::c_int
                                    as isize,
                            ))
                            .star_pt
                            .y as ::core::ffi::c_int
                            - (*(*plotmap
                                .offset(
                                    C2Rust_Unnamed_1::ARILOU_DEFINED.0 as ::core::ffi::c_int
                                        as isize,
                                ))
                                .star)
                                .star_pt
                                .y as ::core::ffi::c_int)
                            * ((*plotmap
                                .offset(
                                    C2Rust_Unnamed_1::ARILOU_DEFINED.0 as ::core::ffi::c_int
                                        as isize,
                                ))
                                .star_pt
                                .y as ::core::ffi::c_int
                                - (*(*plotmap
                                    .offset(
                                        C2Rust_Unnamed_1::ARILOU_DEFINED.0 as ::core::ffi::c_int
                                            as isize,
                                    ))
                                    .star)
                                    .star_pt
                                    .y as ::core::ffi::c_int) < 400 as ::core::ffi::c_int
                {
                    break 's_300;
                } else {
                    star_id = (502 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                        + 15 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as COUNT;
                    (*plotmap.offset(plot_id as isize)).star = starmap
                        .offset(star_id as isize);
                }
            } else {
                star_id = ((rand_val as ::core::ffi::c_int
                    + i as ::core::ffi::c_int * 89 as ::core::ffi::c_int)
                    % 502 as ::core::ffi::c_int) as COUNT;
                if (*starmap.offset(star_id as isize)).Index as ::core::ffi::c_int
                    != 0 as ::core::ffi::c_int
                {
                    break 's_300;
                } else {
                    (*starmap.offset(star_id as isize)).Index = plot_id as BYTE;
                    (*plotmap.offset(plot_id as isize)).star_pt = (*starmap
                        .offset(star_id as isize))
                        .star_pt;
                    (*plotmap.offset(plot_id as isize)).star = starmap
                        .offset(star_id as isize);
                }
            }
            if CheckValid(plotmap, plot_id).0 != 0 {
                return_id = SeedPlot(plotmap, starmap);
                if return_id as ::core::ffi::c_int
                    == C2Rust_Unnamed_1::NUM_PLOTS.0 as ::core::ffi::c_int
                {
                    timer_running = BOOLEAN::FALSE;
                    if plot_id as ::core::ffi::c_int
                        != C2Rust_Unnamed_1::ARILOU_DEFINED.0 as ::core::ffi::c_int
                    {
                        Plotify(starmap, (*plotmap.offset(plot_id as isize)).star);
                    }
                    return return_id;
                }
                if return_id as ::core::ffi::c_int
                    > C2Rust_Unnamed_1::NUM_PLOTS.0 as ::core::ffi::c_int
                    || ConnectedPlot(plotmap, plot_id, return_id)
                        <= 2000000 as ::core::ffi::c_int as DWORD
                {
                    if plot_id as ::core::ffi::c_int
                        != C2Rust_Unnamed_1::ARILOU_DEFINED.0 as ::core::ffi::c_int
                    {
                        (*starmap.offset(star_id as isize)).Index = 0 as BYTE;
                    }
                    (*plotmap.offset(plot_id as isize)).star_pt = point {
                        x: !(0 as ::core::ffi::c_int) as COORD,
                        y: !(0 as ::core::ffi::c_int) as COORD,
                    };
                    (*plotmap.offset(plot_id as isize)).star = ::core::ptr::null_mut::<
                        STAR_DESC,
                    >();
                    if my_clock.0 != 0 {
                        fprintf(
                            stderr,
                            b"Complete failure, stopping clock.\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                        timer_running = BOOLEAN::FALSE;
                    }
                    return return_id;
                }
            }
            if plot_id as ::core::ffi::c_int
                != C2Rust_Unnamed_1::ARILOU_DEFINED.0 as ::core::ffi::c_int
            {
                (*starmap.offset(star_id as isize)).Index = 0 as BYTE;
            }
            (*plotmap.offset(plot_id as isize)).star_pt = point {
                x: !(0 as ::core::ffi::c_int) as COORD,
                y: !(0 as ::core::ffi::c_int) as COORD,
            };
            (*plotmap.offset(plot_id as isize)).star = ::core::ptr::null_mut::<
                STAR_DESC,
            >();
        }
        i = i.wrapping_add(1);
    }
    if my_clock.0 != 0 {
        fprintf(
            stderr,
            b"Complete failure, stopping clock.\n\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        timer_running = BOOLEAN::FALSE;
    }
    return plot_id;
}
#[no_mangle]
pub unsafe extern "C" fn DefaultQuasispace(mut portalmap: *mut PORTAL_LOCATION) {
    if portalmap.is_null() {
        fprintf(
            stderr,
            b"DefaultQuasispace (portalmap) calledwith NULL PTR.\n\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    extern "C" {
        #[link_name = "starmap_array"]
        static starmap_array_0: [STAR_DESC; 0];
    }
    let mut i: COUNT = 0;
    i = 0 as COUNT;
    while (i as ::core::ffi::c_int) < 15 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
    {
        (*portalmap.offset(i as isize)).star_pt = portalmap_array[i as usize].star_pt;
        (*portalmap.offset(i as isize)).quasi_pt = portalmap_array[i as usize].quasi_pt;
        (*portalmap.offset(i as isize)).nearest_star = FindNearestConstellation(
            &raw const starmap_array as *const STAR_DESC as *mut ::core::ffi::c_void
                as *mut STAR_DESC,
            (*portalmap.offset(i as isize)).star_pt,
        );
        if (*portalmap.offset(i as isize)).nearest_star.is_null() {
            fprintf(
                stderr,
                b"BAD Quasi Portal %c at %05.1f : %05.1f, %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                'A' as ::core::ffi::c_int + i as ::core::ffi::c_int,
                ((*portalmap.offset(i as isize)).star_pt.x as ::core::ffi::c_float
                    / 10 as ::core::ffi::c_int as ::core::ffi::c_float)
                    as ::core::ffi::c_double,
                ((*portalmap.offset(i as isize)).star_pt.y as ::core::ffi::c_float
                    / 10 as ::core::ffi::c_int as ::core::ffi::c_float)
                    as ::core::ffi::c_double,
                b"but no star found with FindNearestStar.\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn SeedQuasispace(
    mut portalmap: *mut PORTAL_LOCATION,
    mut plotmap: *mut PLOT_LOCATION,
    mut starmap: *mut STAR_DESC,
) -> BOOLEAN {
    let mut swap: PORTAL_LOCATION = PORTAL_LOCATION {
        star_pt: POINT { x: 0, y: 0 },
        quasi_pt: POINT { x: 0, y: 0 },
        nearest_star: ::core::ptr::null_mut::<STAR_DESC>(),
    };
    let mut rand_val: UWORD = 0;
    let mut i: COUNT = 0;
    let mut j: COUNT = 0;
    let mut valid: BOOLEAN = BOOLEAN::FALSE;
    if portalmap.is_null() || plotmap.is_null() || starmap.is_null() {
        fprintf(
            stderr,
            b"Seed Quasispace called with NULL pointer(s).\n\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return BOOLEAN::FALSE;
    }
    if StarGenRNG.is_null() {
        fprintf(
            stderr,
            b"****SeedQuasispace creating a STAR GEN RNG****\n\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        StarGenRNG = RandomContext_New();
    }
    RandomContext_SeedRandom(StarGenRNG, optCustomSeed as DWORD);
    i = 0 as COUNT;
    while (i as ::core::ffi::c_int) < 15 as ::core::ffi::c_int {
        loop {
            valid = BOOLEAN::TRUE;
            's_50: {
                if i as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    rand_val = RandomContext_Random(StarGenRNG) as UWORD;
                    (*portalmap.offset(i as isize)).star_pt = point {
                        x: (rand_val as BYTE as COORD as ::core::ffi::c_int
                            * 9 as ::core::ffi::c_int
                            + (*plotmap
                                .offset(
                                    C2Rust_Unnamed_1::SOL_DEFINED.0 as ::core::ffi::c_int
                                        as isize,
                                ))
                                .star_pt
                                .x as ::core::ffi::c_int - 1147 as ::core::ffi::c_int)
                            as COORD,
                        y: ((rand_val as ::core::ffi::c_int >> 8 as ::core::ffi::c_int)
                            as BYTE as COORD as ::core::ffi::c_int
                            * 9 as ::core::ffi::c_int
                            + (*plotmap
                                .offset(
                                    C2Rust_Unnamed_1::SOL_DEFINED.0 as ::core::ffi::c_int
                                        as isize,
                                ))
                                .star_pt
                                .y as ::core::ffi::c_int - 1147 as ::core::ffi::c_int)
                            as COORD,
                    };
                    if ((*portalmap.offset(i as isize)).star_pt.x as ::core::ffi::c_int)
                        < 189 as ::core::ffi::c_int
                        || (*portalmap.offset(i as isize)).star_pt.x
                            as ::core::ffi::c_int
                            > 9999 as ::core::ffi::c_int - 189 as ::core::ffi::c_int
                        || ((*portalmap.offset(i as isize)).star_pt.y
                            as ::core::ffi::c_int) < 189 as ::core::ffi::c_int
                        || (*portalmap.offset(i as isize)).star_pt.y
                            as ::core::ffi::c_int
                            > 9999 as ::core::ffi::c_int - 189 as ::core::ffi::c_int
                        || ((*portalmap.offset(i as isize)).star_pt.x
                            as ::core::ffi::c_int
                            - (*plotmap
                                .offset(
                                    C2Rust_Unnamed_1::SOL_DEFINED.0 as ::core::ffi::c_int
                                        as isize,
                                ))
                                .star_pt
                                .x as ::core::ffi::c_int)
                            * ((*portalmap.offset(i as isize)).star_pt.x
                                as ::core::ffi::c_int
                                - (*plotmap
                                    .offset(
                                        C2Rust_Unnamed_1::SOL_DEFINED.0 as ::core::ffi::c_int
                                            as isize,
                                    ))
                                    .star_pt
                                    .x as ::core::ffi::c_int)
                            + ((*portalmap.offset(i as isize)).star_pt.y
                                as ::core::ffi::c_int
                                - (*plotmap
                                    .offset(
                                        C2Rust_Unnamed_1::SOL_DEFINED.0 as ::core::ffi::c_int
                                            as isize,
                                    ))
                                    .star_pt
                                    .y as ::core::ffi::c_int)
                                * ((*portalmap.offset(i as isize)).star_pt.y
                                    as ::core::ffi::c_int
                                    - (*plotmap
                                        .offset(
                                            C2Rust_Unnamed_1::SOL_DEFINED.0 as ::core::ffi::c_int
                                                as isize,
                                        ))
                                        .star_pt
                                        .y as ::core::ffi::c_int) < 250000 as ::core::ffi::c_int
                        || ((*portalmap.offset(i as isize)).star_pt.x
                            as ::core::ffi::c_int
                            - (*plotmap
                                .offset(
                                    C2Rust_Unnamed_1::SOL_DEFINED.0 as ::core::ffi::c_int
                                        as isize,
                                ))
                                .star_pt
                                .x as ::core::ffi::c_int)
                            * ((*portalmap.offset(i as isize)).star_pt.x
                                as ::core::ffi::c_int
                                - (*plotmap
                                    .offset(
                                        C2Rust_Unnamed_1::SOL_DEFINED.0 as ::core::ffi::c_int
                                            as isize,
                                    ))
                                    .star_pt
                                    .x as ::core::ffi::c_int)
                            + ((*portalmap.offset(i as isize)).star_pt.y
                                as ::core::ffi::c_int
                                - (*plotmap
                                    .offset(
                                        C2Rust_Unnamed_1::SOL_DEFINED.0 as ::core::ffi::c_int
                                            as isize,
                                    ))
                                    .star_pt
                                    .y as ::core::ffi::c_int)
                                * ((*portalmap.offset(i as isize)).star_pt.y
                                    as ::core::ffi::c_int
                                    - (*plotmap
                                        .offset(
                                            C2Rust_Unnamed_1::SOL_DEFINED.0 as ::core::ffi::c_int
                                                as isize,
                                        ))
                                        .star_pt
                                        .y as ::core::ffi::c_int) > 1000000 as ::core::ffi::c_int
                    {
                        valid = BOOLEAN::FALSE;
                        break 's_50;
                    }
                } else {
                    rand_val = RandomContext_Random(StarGenRNG) as UWORD;
                    (*portalmap.offset(i as isize)).star_pt = point {
                        x: (rand_val as BYTE as ::core::ffi::c_int
                            * 9999 as ::core::ffi::c_int / 265 as ::core::ffi::c_int
                            + 189 as ::core::ffi::c_int) as COORD,
                        y: ((rand_val as ::core::ffi::c_int >> 8 as ::core::ffi::c_int)
                            as BYTE as ::core::ffi::c_int * 9999 as ::core::ffi::c_int
                            / 265 as ::core::ffi::c_int + 189 as ::core::ffi::c_int)
                            as COORD,
                    };
                    j = 0 as COUNT;
                    while (j as ::core::ffi::c_int) < i as ::core::ffi::c_int {
                        if ((*portalmap.offset(i as isize)).star_pt.x
                            as ::core::ffi::c_int
                            - (*portalmap.offset(j as isize)).star_pt.x
                                as ::core::ffi::c_int)
                            * ((*portalmap.offset(i as isize)).star_pt.x
                                as ::core::ffi::c_int
                                - (*portalmap.offset(j as isize)).star_pt.x
                                    as ::core::ffi::c_int)
                            + ((*portalmap.offset(i as isize)).star_pt.y
                                as ::core::ffi::c_int
                                - (*portalmap.offset(j as isize)).star_pt.y
                                    as ::core::ffi::c_int)
                                * ((*portalmap.offset(i as isize)).star_pt.y
                                    as ::core::ffi::c_int
                                    - (*portalmap.offset(j as isize)).star_pt.y
                                        as ::core::ffi::c_int)
                            < 2000 as ::core::ffi::c_int * 2000 as ::core::ffi::c_int
                        {
                            valid = BOOLEAN::FALSE;
                        }
                        j = j.wrapping_add(1);
                    }
                    if valid.0 == 0 {
                        break 's_50;
                    }
                }
                if (*portalmap.offset(i as isize)).star_pt.x as ::core::ffi::c_int
                    <= 0 as ::core::ffi::c_int
                    || (*portalmap.offset(i as isize)).star_pt.x as ::core::ffi::c_int
                        >= 9999 as ::core::ffi::c_int
                    || (*portalmap.offset(i as isize)).star_pt.y as ::core::ffi::c_int
                        <= 0 as ::core::ffi::c_int
                    || (*portalmap.offset(i as isize)).star_pt.y as ::core::ffi::c_int
                        >= 9999 as ::core::ffi::c_int
                {
                    fprintf(
                        stderr,
                        b"BAD Quasi Portal %c at %05.1f : %05.1f, %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        'A' as ::core::ffi::c_int + i as ::core::ffi::c_int,
                        ((*portalmap.offset(i as isize)).star_pt.x
                            as ::core::ffi::c_float
                            / 10 as ::core::ffi::c_int as ::core::ffi::c_float)
                            as ::core::ffi::c_double,
                        ((*portalmap.offset(i as isize)).star_pt.y
                            as ::core::ffi::c_float
                            / 10 as ::core::ffi::c_int as ::core::ffi::c_float)
                            as ::core::ffi::c_double,
                        b"but no star found with FindNearestStar.\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                    valid = BOOLEAN::FALSE;
                } else {
                    (*portalmap.offset(i as isize)).nearest_star = FindNearestConstellation(
                        &raw mut star_array as *mut STAR_DESC,
                        (*portalmap.offset(i as isize)).star_pt,
                    );
                    if (*portalmap.offset(i as isize)).nearest_star.is_null() {
                        fprintf(
                            stderr,
                            b"BAD Quasi Portal %c at %05.1f : %05.1f, %s\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            'A' as ::core::ffi::c_int + i as ::core::ffi::c_int,
                            ((*portalmap.offset(i as isize)).star_pt.x
                                as ::core::ffi::c_float
                                / 10 as ::core::ffi::c_int as ::core::ffi::c_float)
                                as ::core::ffi::c_double,
                            ((*portalmap.offset(i as isize)).star_pt.y
                                as ::core::ffi::c_float
                                / 10 as ::core::ffi::c_int as ::core::ffi::c_float)
                                as ::core::ffi::c_double,
                            b"but no star found with FindNearestStar.\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                        valid = BOOLEAN::FALSE;
                    } else if ((*(*portalmap.offset(i as isize)).nearest_star).star_pt.x
                        as ::core::ffi::c_int
                        - (*portalmap.offset(i as isize)).star_pt.x
                            as ::core::ffi::c_int)
                        * ((*(*portalmap.offset(i as isize)).nearest_star).star_pt.x
                            as ::core::ffi::c_int
                            - (*portalmap.offset(i as isize)).star_pt.x
                                as ::core::ffi::c_int)
                        + ((*(*portalmap.offset(i as isize)).nearest_star).star_pt.y
                            as ::core::ffi::c_int
                            - (*portalmap.offset(i as isize)).star_pt.y
                                as ::core::ffi::c_int)
                            * ((*(*portalmap.offset(i as isize)).nearest_star).star_pt.y
                                as ::core::ffi::c_int
                                - (*portalmap.offset(i as isize)).star_pt.y
                                    as ::core::ffi::c_int) < 400 as ::core::ffi::c_int
                    {
                        valid = BOOLEAN::FALSE;
                    }
                }
            }
            if valid.0 != 0 {
                break;
            }
        }
        valid = BOOLEAN::FALSE;
        while valid.0 == 0 {
            valid = BOOLEAN::TRUE;
            rand_val = RandomContext_Random(StarGenRNG) as UWORD;
            (*portalmap.offset(i as isize)).quasi_pt = point {
                x: ((rand_val as BYTE as ::core::ffi::c_int * 51 as ::core::ffi::c_int
                    / 256 as ::core::ffi::c_int - 25 as ::core::ffi::c_int)
                    * 20 as ::core::ffi::c_int + 5000 as ::core::ffi::c_int) as COORD,
                y: (((rand_val as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as BYTE
                    as ::core::ffi::c_int * 51 as ::core::ffi::c_int
                    / 256 as ::core::ffi::c_int - 25 as ::core::ffi::c_int)
                    * 20 as ::core::ffi::c_int + 5000 as ::core::ffi::c_int) as COORD,
            };
            j = 0 as COUNT;
            while (j as ::core::ffi::c_int) < i as ::core::ffi::c_int {
                if ((*portalmap.offset(i as isize)).quasi_pt.x as ::core::ffi::c_int
                    - (*portalmap.offset(j as isize)).quasi_pt.x as ::core::ffi::c_int)
                    * ((*portalmap.offset(i as isize)).quasi_pt.x as ::core::ffi::c_int
                        - (*portalmap.offset(j as isize)).quasi_pt.x
                            as ::core::ffi::c_int)
                    + ((*portalmap.offset(i as isize)).quasi_pt.y as ::core::ffi::c_int
                        - (*portalmap.offset(j as isize)).quasi_pt.y
                            as ::core::ffi::c_int)
                        * ((*portalmap.offset(i as isize)).quasi_pt.y
                            as ::core::ffi::c_int
                            - (*portalmap.offset(j as isize)).quasi_pt.y
                                as ::core::ffi::c_int)
                    < 150 as ::core::ffi::c_int * 150 as ::core::ffi::c_int
                {
                    valid = BOOLEAN::FALSE;
                }
                j = j.wrapping_add(1);
            }
            if ((*portalmap.offset(i as isize)).quasi_pt.x as ::core::ffi::c_int
                - 5000 as ::core::ffi::c_int)
                * ((*portalmap.offset(i as isize)).quasi_pt.x as ::core::ffi::c_int
                    - 5000 as ::core::ffi::c_int)
                + ((*portalmap.offset(i as isize)).quasi_pt.y as ::core::ffi::c_int
                    - 5000 as ::core::ffi::c_int)
                    * ((*portalmap.offset(i as isize)).quasi_pt.y as ::core::ffi::c_int
                        - 5000 as ::core::ffi::c_int)
                < 150 as ::core::ffi::c_int * 150 as ::core::ffi::c_int
            {
                valid = BOOLEAN::FALSE;
            }
        }
        i = i.wrapping_add(1);
    }
    valid = BOOLEAN::FALSE;
    while valid.0 == 0 {
        valid = BOOLEAN::TRUE;
        i = 0 as COUNT;
        while (i as ::core::ffi::c_int)
            < 15 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
        {
            j = (i as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as COUNT;
            while (j as ::core::ffi::c_int) < 15 as ::core::ffi::c_int {
                if (*portalmap.offset(i as isize)).quasi_pt.y as ::core::ffi::c_int
                    > (*portalmap.offset(j as isize)).quasi_pt.y as ::core::ffi::c_int
                    || (*portalmap.offset(i as isize)).quasi_pt.y as ::core::ffi::c_int
                        == (*portalmap.offset(j as isize)).quasi_pt.y
                            as ::core::ffi::c_int
                        && (*portalmap.offset(i as isize)).quasi_pt.x
                            as ::core::ffi::c_int
                            > (*portalmap.offset(j as isize)).quasi_pt.x
                                as ::core::ffi::c_int
                {
                    valid = BOOLEAN::FALSE;
                    swap = *portalmap.offset(i as isize);
                    *portalmap.offset(i as isize) = *portalmap.offset(j as isize);
                    *portalmap.offset(j as isize) = swap;
                }
                j = j.wrapping_add(1);
            }
            i = i.wrapping_add(1);
        }
    }
    i = 0 as COUNT;
    j = (502 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as COUNT;
    while (i as ::core::ffi::c_int) < 15 as ::core::ffi::c_int {
        (*starmap.offset(j as isize)).star_pt = (*portalmap.offset(i as isize)).quasi_pt;
        i = i.wrapping_add(1);
        j = j.wrapping_add(1);
    }
    return BOOLEAN::TRUE;
}
static mut plotIdMap: [PlotIdMap; 70] = [
    PlotIdMap {
        idStr: b"algolites\0".as_ptr() as *const ::core::ffi::c_char,
        id: C2Rust_Unnamed_1::ALGOLITES_DEFINED.0 as ::core::ffi::c_int as COUNT,
    },
    PlotIdMap {
        idStr: b"androsynth\0".as_ptr() as *const ::core::ffi::c_char,
        id: C2Rust_Unnamed_1::ANDROSYNTH_DEFINED.0 as ::core::ffi::c_int as COUNT,
    },
    PlotIdMap {
        idStr: b"aqua helix\0".as_ptr() as *const ::core::ffi::c_char,
        id: C2Rust_Unnamed_1::AQUA_HELIX_DEFINED.0 as ::core::ffi::c_int as COUNT,
    },
    PlotIdMap {
        idStr: b"arilou\0".as_ptr() as *const ::core::ffi::c_char,
        id: C2Rust_Unnamed_1::ARILOU_DEFINED.0 as ::core::ffi::c_int as COUNT,
    },
    PlotIdMap {
        idStr: b"bomb\0".as_ptr() as *const ::core::ffi::c_char,
        id: C2Rust_Unnamed_1::BOMB_DEFINED.0 as ::core::ffi::c_int as COUNT,
    },
    PlotIdMap {
        idStr: b"burvixese\0".as_ptr() as *const ::core::ffi::c_char,
        id: C2Rust_Unnamed_1::BURVIXESE_DEFINED.0 as ::core::ffi::c_int as COUNT,
    },
    PlotIdMap {
        idStr: b"chmmr\0".as_ptr() as *const ::core::ffi::c_char,
        id: C2Rust_Unnamed_1::CHMMR_DEFINED.0 as ::core::ffi::c_int as COUNT,
    },
    PlotIdMap {
        idStr: b"destroyed starbase\0".as_ptr() as *const ::core::ffi::c_char,
        id: C2Rust_Unnamed_1::DESTROYED_STARBASE_DEFINED.0 as ::core::ffi::c_int as COUNT,
    },
    PlotIdMap {
        idStr: b"druuge\0".as_ptr() as *const ::core::ffi::c_char,
        id: C2Rust_Unnamed_1::DRUUGE_DEFINED.0 as ::core::ffi::c_int as COUNT,
    },
    PlotIdMap {
        idStr: b"egg case 0\0".as_ptr() as *const ::core::ffi::c_char,
        id: C2Rust_Unnamed_1::EGG_CASE0_DEFINED.0 as ::core::ffi::c_int as COUNT,
    },
    PlotIdMap {
        idStr: b"egg case 1\0".as_ptr() as *const ::core::ffi::c_char,
        id: C2Rust_Unnamed_1::EGG_CASE1_DEFINED.0 as ::core::ffi::c_int as COUNT,
    },
    PlotIdMap {
        idStr: b"egg case 2\0".as_ptr() as *const ::core::ffi::c_char,
        id: C2Rust_Unnamed_1::EGG_CASE2_DEFINED.0 as ::core::ffi::c_int as COUNT,
    },
    PlotIdMap {
        idStr: b"excavation site\0".as_ptr() as *const ::core::ffi::c_char,
        id: C2Rust_Unnamed_1::EXCAVATION_SITE_DEFINED.0 as ::core::ffi::c_int as COUNT,
    },
    PlotIdMap {
        idStr: b"ilwrath\0".as_ptr() as *const ::core::ffi::c_char,
        id: C2Rust_Unnamed_1::ILWRATH_DEFINED.0 as ::core::ffi::c_int as COUNT,
    },
    PlotIdMap {
        idStr: b"kohrah 0\0".as_ptr() as *const ::core::ffi::c_char,
        id: C2Rust_Unnamed_1::KOHRAH0_DEFINED.0 as ::core::ffi::c_int as COUNT,
    },
    PlotIdMap {
        idStr: b"kohrah 1\0".as_ptr() as *const ::core::ffi::c_char,
        id: C2Rust_Unnamed_1::KOHRAH1_DEFINED.0 as ::core::ffi::c_int as COUNT,
    },
    PlotIdMap {
        idStr: b"kohrah 2\0".as_ptr() as *const ::core::ffi::c_char,
        id: C2Rust_Unnamed_1::KOHRAH2_DEFINED.0 as ::core::ffi::c_int as COUNT,
    },
    PlotIdMap {
        idStr: b"maidens\0".as_ptr() as *const ::core::ffi::c_char,
        id: C2Rust_Unnamed_1::MAIDENS_DEFINED.0 as ::core::ffi::c_int as COUNT,
    },
    PlotIdMap {
        idStr: b"melnorme 0\0".as_ptr() as *const ::core::ffi::c_char,
        id: C2Rust_Unnamed_1::MELNORME0_DEFINED.0 as ::core::ffi::c_int as COUNT,
    },
    PlotIdMap {
        idStr: b"melnorme 1\0".as_ptr() as *const ::core::ffi::c_char,
        id: C2Rust_Unnamed_1::MELNORME1_DEFINED.0 as ::core::ffi::c_int as COUNT,
    },
    PlotIdMap {
        idStr: b"melnorme 2\0".as_ptr() as *const ::core::ffi::c_char,
        id: C2Rust_Unnamed_1::MELNORME2_DEFINED.0 as ::core::ffi::c_int as COUNT,
    },
    PlotIdMap {
        idStr: b"melnorme 3\0".as_ptr() as *const ::core::ffi::c_char,
        id: C2Rust_Unnamed_1::MELNORME3_DEFINED.0 as ::core::ffi::c_int as COUNT,
    },
    PlotIdMap {
        idStr: b"melnorme 4\0".as_ptr() as *const ::core::ffi::c_char,
        id: C2Rust_Unnamed_1::MELNORME4_DEFINED.0 as ::core::ffi::c_int as COUNT,
    },
    PlotIdMap {
        idStr: b"melnorme 5\0".as_ptr() as *const ::core::ffi::c_char,
        id: C2Rust_Unnamed_1::MELNORME5_DEFINED.0 as ::core::ffi::c_int as COUNT,
    },
    PlotIdMap {
        idStr: b"melnorme 6\0".as_ptr() as *const ::core::ffi::c_char,
        id: C2Rust_Unnamed_1::MELNORME6_DEFINED.0 as ::core::ffi::c_int as COUNT,
    },
    PlotIdMap {
        idStr: b"melnorme 7\0".as_ptr() as *const ::core::ffi::c_char,
        id: C2Rust_Unnamed_1::MELNORME7_DEFINED.0 as ::core::ffi::c_int as COUNT,
    },
    PlotIdMap {
        idStr: b"melnorme 8\0".as_ptr() as *const ::core::ffi::c_char,
        id: C2Rust_Unnamed_1::MELNORME8_DEFINED.0 as ::core::ffi::c_int as COUNT,
    },
    PlotIdMap {
        idStr: b"mother ark\0".as_ptr() as *const ::core::ffi::c_char,
        id: C2Rust_Unnamed_1::MOTHER_ARK_DEFINED.0 as ::core::ffi::c_int as COUNT,
    },
    PlotIdMap {
        idStr: b"mycon\0".as_ptr() as *const ::core::ffi::c_char,
        id: C2Rust_Unnamed_1::MYCON_DEFINED.0 as ::core::ffi::c_int as COUNT,
    },
    PlotIdMap {
        idStr: b"mycon trap\0".as_ptr() as *const ::core::ffi::c_char,
        id: C2Rust_Unnamed_1::MYCON_TRAP_DEFINED.0 as ::core::ffi::c_int as COUNT,
    },
    PlotIdMap {
        idStr: b"orz\0".as_ptr() as *const ::core::ffi::c_char,
        id: C2Rust_Unnamed_1::ORZ_DEFINED.0 as ::core::ffi::c_int as COUNT,
    },
    PlotIdMap {
        idStr: b"pkunk\0".as_ptr() as *const ::core::ffi::c_char,
        id: C2Rust_Unnamed_1::PKUNK_DEFINED.0 as ::core::ffi::c_int as COUNT,
    },
    PlotIdMap {
        idStr: b"rainbow 0\0".as_ptr() as *const ::core::ffi::c_char,
        id: C2Rust_Unnamed_1::RAINBOW0_DEFINED.0 as ::core::ffi::c_int as COUNT,
    },
    PlotIdMap {
        idStr: b"rainbow 1\0".as_ptr() as *const ::core::ffi::c_char,
        id: C2Rust_Unnamed_1::RAINBOW1_DEFINED.0 as ::core::ffi::c_int as COUNT,
    },
    PlotIdMap {
        idStr: b"rainbow 2\0".as_ptr() as *const ::core::ffi::c_char,
        id: C2Rust_Unnamed_1::RAINBOW2_DEFINED.0 as ::core::ffi::c_int as COUNT,
    },
    PlotIdMap {
        idStr: b"rainbow 3\0".as_ptr() as *const ::core::ffi::c_char,
        id: C2Rust_Unnamed_1::RAINBOW3_DEFINED.0 as ::core::ffi::c_int as COUNT,
    },
    PlotIdMap {
        idStr: b"rainbow 4\0".as_ptr() as *const ::core::ffi::c_char,
        id: C2Rust_Unnamed_1::RAINBOW4_DEFINED.0 as ::core::ffi::c_int as COUNT,
    },
    PlotIdMap {
        idStr: b"rainbow 5\0".as_ptr() as *const ::core::ffi::c_char,
        id: C2Rust_Unnamed_1::RAINBOW5_DEFINED.0 as ::core::ffi::c_int as COUNT,
    },
    PlotIdMap {
        idStr: b"rainbow 6\0".as_ptr() as *const ::core::ffi::c_char,
        id: C2Rust_Unnamed_1::RAINBOW6_DEFINED.0 as ::core::ffi::c_int as COUNT,
    },
    PlotIdMap {
        idStr: b"rainbow 7\0".as_ptr() as *const ::core::ffi::c_char,
        id: C2Rust_Unnamed_1::RAINBOW7_DEFINED.0 as ::core::ffi::c_int as COUNT,
    },
    PlotIdMap {
        idStr: b"rainbow 8\0".as_ptr() as *const ::core::ffi::c_char,
        id: C2Rust_Unnamed_1::RAINBOW8_DEFINED.0 as ::core::ffi::c_int as COUNT,
    },
    PlotIdMap {
        idStr: b"rainbow 9\0".as_ptr() as *const ::core::ffi::c_char,
        id: C2Rust_Unnamed_1::RAINBOW9_DEFINED.0 as ::core::ffi::c_int as COUNT,
    },
    PlotIdMap {
        idStr: b"samatra\0".as_ptr() as *const ::core::ffi::c_char,
        id: C2Rust_Unnamed_1::SAMATRA_DEFINED.0 as ::core::ffi::c_int as COUNT,
    },
    PlotIdMap {
        idStr: b"ship vault\0".as_ptr() as *const ::core::ffi::c_char,
        id: C2Rust_Unnamed_1::SHIP_VAULT_DEFINED.0 as ::core::ffi::c_int as COUNT,
    },
    PlotIdMap {
        idStr: b"shofixti\0".as_ptr() as *const ::core::ffi::c_char,
        id: C2Rust_Unnamed_1::SHOFIXTI_DEFINED.0 as ::core::ffi::c_int as COUNT,
    },
    PlotIdMap {
        idStr: b"slylandro\0".as_ptr() as *const ::core::ffi::c_char,
        id: C2Rust_Unnamed_1::SLYLANDRO_DEFINED.0 as ::core::ffi::c_int as COUNT,
    },
    PlotIdMap {
        idStr: b"sol\0".as_ptr() as *const ::core::ffi::c_char,
        id: C2Rust_Unnamed_1::SOL_DEFINED.0 as ::core::ffi::c_int as COUNT,
    },
    PlotIdMap {
        idStr: b"spathi\0".as_ptr() as *const ::core::ffi::c_char,
        id: C2Rust_Unnamed_1::SPATHI_DEFINED.0 as ::core::ffi::c_int as COUNT,
    },
    PlotIdMap {
        idStr: b"spathi monument\0".as_ptr() as *const ::core::ffi::c_char,
        id: C2Rust_Unnamed_1::SPATHI_MONUMENT_DEFINED.0 as ::core::ffi::c_int as COUNT,
    },
    PlotIdMap {
        idStr: b"start colony\0".as_ptr() as *const ::core::ffi::c_char,
        id: C2Rust_Unnamed_1::START_COLONY_DEFINED.0 as ::core::ffi::c_int as COUNT,
    },
    PlotIdMap {
        idStr: b"sun device\0".as_ptr() as *const ::core::ffi::c_char,
        id: C2Rust_Unnamed_1::SUN_DEVICE_DEFINED.0 as ::core::ffi::c_int as COUNT,
    },
    PlotIdMap {
        idStr: b"supox\0".as_ptr() as *const ::core::ffi::c_char,
        id: C2Rust_Unnamed_1::SUPOX_DEFINED.0 as ::core::ffi::c_int as COUNT,
    },
    PlotIdMap {
        idStr: b"syreen\0".as_ptr() as *const ::core::ffi::c_char,
        id: C2Rust_Unnamed_1::SYREEN_DEFINED.0 as ::core::ffi::c_int as COUNT,
    },
    PlotIdMap {
        idStr: b"taalo protector\0".as_ptr() as *const ::core::ffi::c_char,
        id: C2Rust_Unnamed_1::TAALO_PROTECTOR_DEFINED.0 as ::core::ffi::c_int as COUNT,
    },
    PlotIdMap {
        idStr: b"talking pet\0".as_ptr() as *const ::core::ffi::c_char,
        id: C2Rust_Unnamed_1::TALKING_PET_DEFINED.0 as ::core::ffi::c_int as COUNT,
    },
    PlotIdMap {
        idStr: b"thraddash\0".as_ptr() as *const ::core::ffi::c_char,
        id: C2Rust_Unnamed_1::THRADD_DEFINED.0 as ::core::ffi::c_int as COUNT,
    },
    PlotIdMap {
        idStr: b"urquan 0\0".as_ptr() as *const ::core::ffi::c_char,
        id: C2Rust_Unnamed_1::URQUAN0_DEFINED.0 as ::core::ffi::c_int as COUNT,
    },
    PlotIdMap {
        idStr: b"urquan 1\0".as_ptr() as *const ::core::ffi::c_char,
        id: C2Rust_Unnamed_1::URQUAN1_DEFINED.0 as ::core::ffi::c_int as COUNT,
    },
    PlotIdMap {
        idStr: b"urquan 2\0".as_ptr() as *const ::core::ffi::c_char,
        id: C2Rust_Unnamed_1::URQUAN2_DEFINED.0 as ::core::ffi::c_int as COUNT,
    },
    PlotIdMap {
        idStr: b"urquan wreck\0".as_ptr() as *const ::core::ffi::c_char,
        id: C2Rust_Unnamed_1::URQUAN_WRECK_DEFINED.0 as ::core::ffi::c_int as COUNT,
    },
    PlotIdMap {
        idStr: b"utwig\0".as_ptr() as *const ::core::ffi::c_char,
        id: C2Rust_Unnamed_1::UTWIG_DEFINED.0 as ::core::ffi::c_int as COUNT,
    },
    PlotIdMap {
        idStr: b"vux\0".as_ptr() as *const ::core::ffi::c_char,
        id: C2Rust_Unnamed_1::VUX_DEFINED.0 as ::core::ffi::c_int as COUNT,
    },
    PlotIdMap {
        idStr: b"vux beast\0".as_ptr() as *const ::core::ffi::c_char,
        id: C2Rust_Unnamed_1::VUX_BEAST_DEFINED.0 as ::core::ffi::c_int as COUNT,
    },
    PlotIdMap {
        idStr: b"yehat\0".as_ptr() as *const ::core::ffi::c_char,
        id: C2Rust_Unnamed_1::YEHAT_DEFINED.0 as ::core::ffi::c_int as COUNT,
    },
    PlotIdMap {
        idStr: b"zoq colony 0\0".as_ptr() as *const ::core::ffi::c_char,
        id: C2Rust_Unnamed_1::ZOQ_COLONY0_DEFINED.0 as ::core::ffi::c_int as COUNT,
    },
    PlotIdMap {
        idStr: b"zoq colony 1\0".as_ptr() as *const ::core::ffi::c_char,
        id: C2Rust_Unnamed_1::ZOQ_COLONY1_DEFINED.0 as ::core::ffi::c_int as COUNT,
    },
    PlotIdMap {
        idStr: b"zoq colony 2\0".as_ptr() as *const ::core::ffi::c_char,
        id: C2Rust_Unnamed_1::ZOQ_COLONY2_DEFINED.0 as ::core::ffi::c_int as COUNT,
    },
    PlotIdMap {
        idStr: b"zoq colony 3\0".as_ptr() as *const ::core::ffi::c_char,
        id: C2Rust_Unnamed_1::ZOQ_COLONY3_DEFINED.0 as ::core::ffi::c_int as COUNT,
    },
    PlotIdMap {
        idStr: b"zoq scout\0".as_ptr() as *const ::core::ffi::c_char,
        id: C2Rust_Unnamed_1::ZOQ_SCOUT_DEFINED.0 as ::core::ffi::c_int as COUNT,
    },
    PlotIdMap {
        idStr: b"zoqfot\0".as_ptr() as *const ::core::ffi::c_char,
        id: C2Rust_Unnamed_1::ZOQFOT_DEFINED.0 as ::core::ffi::c_int as COUNT,
    },
];
unsafe extern "C" fn PlotIdCompare(
    mut id1: *const ::core::ffi::c_void,
    mut id2: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return strcmp((*(id1 as *mut PlotIdMap)).idStr, (*(id2 as *mut PlotIdMap)).idStr);
}
#[no_mangle]
pub unsafe extern "C" fn PlotIdStrToIndex(
    mut plotIdStr: *const ::core::ffi::c_char,
) -> COUNT {
    let mut key: PlotIdMap = PlotIdMap {
        idStr: plotIdStr,
        id: !(0 as ::core::ffi::c_int) as COUNT,
    };
    let mut found: *mut PlotIdMap = bsearch(
        &raw mut key as *const ::core::ffi::c_void,
        &raw mut plotIdMap as *mut PlotIdMap as *const ::core::ffi::c_void,
        (::core::mem::size_of::<[PlotIdMap; 70]>() as size_t)
            .wrapping_div(::core::mem::size_of::<PlotIdMap>() as size_t),
        ::core::mem::size_of::<PlotIdMap>() as size_t,
        Some(
            PlotIdCompare
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_void,
                    *const ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
    ) as *mut PlotIdMap;
    if found.is_null() {
        return (C2Rust_Unnamed_1::NUM_PLOTS.0 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int) as COUNT;
    }
    return (*found).id;
}
