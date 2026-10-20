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

// Rust translation of src/uqm/load_legacy.c.
extern "C" {
    pub type uio_DirHandle;
    pub type uio_Stream;
    pub type string_table_entry;
    pub type frame_desc;
    pub type _LZHCODE_DESC;
    pub type GAME_STATE_FILE;
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
    fn res_OpenResFile(
        dir: *mut uio_DirHandle,
        filename: *const ::core::ffi::c_char,
        mode: *const ::core::ffi::c_char,
    ) -> *mut uio_Stream;
    fn ReadResFile(
        lpBuf: *mut ::core::ffi::c_void,
        size: size_t,
        count: size_t,
        fp: *mut uio_Stream,
    ) -> size_t;
    fn res_CloseResFile(fp: *mut uio_Stream) -> BOOLEAN;
    fn AllocLink(pq: *mut QUEUE) -> HLINK;
    fn ReinitQueue(pq: *mut QUEUE);
    fn PutQueue(pq: *mut QUEUE, hLink: HLINK);
    fn memcpy(
        _: *mut ::core::ffi::c_void,
        _: *const ::core::ffi::c_void,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        _: *mut ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn CloneShipFragment(
        shipIndex: COUNT,
        pDstQueue: *mut QUEUE,
        crew_level: COUNT,
    ) -> HSHIPFRAG;
    fn GetStarShipFromIndex(pShipQ: *mut QUEUE, Index: COUNT) -> HLINK;
    static mut EncounterRace: SIZE;
    static mut EncounterGroup: BYTE;
    fn copen(
        InStream: *mut ::core::ffi::c_void,
        SType: STREAM_TYPE,
        SMode: STREAM_MODE,
    ) -> DECODE_REF;
    fn cclose(DecodeRef: DECODE_REF) -> DWORD;
    fn cread(
        pStr: *mut ::core::ffi::c_void,
        size: COUNT,
        count: COUNT,
        DecodeRef: DECODE_REF,
    ) -> COUNT;
    static mut GlobData: GLOBDATA;
    fn copyGameState(
        dest: *mut BYTE,
        target: DWORD,
        src: *mut BYTE,
        begin: DWORD,
        end: DWORD,
    );
    fn getGameState32(state: *mut BYTE, startBit: ::core::ffi::c_int) -> DWORD;
    fn setGameState32(state: *mut BYTE, startBit: ::core::ffi::c_int, val: DWORD);
    static mut CurStarDescPtr: *mut STAR_DESC;
    fn FindStar(
        pLastStar: *mut STAR_DESC,
        puniverse: *mut POINT,
        xbounds: SIZE,
        ybounds: SIZE,
    ) -> *mut STAR_DESC;
    static mut race_q: [QUEUE; 0];
    fn OpenStateFile(
        stateFile: ::core::ffi::c_int,
        mode: *const ::core::ffi::c_char,
    ) -> *mut GAME_STATE_FILE;
    fn CloseStateFile(fp: *mut GAME_STATE_FILE);
    fn WriteStateFile(
        lpBuf: *const ::core::ffi::c_void,
        size: COUNT,
        count: COUNT,
        fp: *mut GAME_STATE_FILE,
    ) -> ::core::ffi::c_int;
    fn BuildGroup(pDstQueue: *mut QUEUE, race_id: BYTE) -> HIPGROUP;
    fn log_add(_: log_Level, fmt: *const ::core::ffi::c_char, ...);
    static mut NextActivity: ACTIVITY;
    static mut saveDir: *mut uio_DirHandle;
    fn abort() -> !;
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
pub type HELEMENT = HLINK;
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
pub type DECODE_REF = *mut _LZHCODE_DESC;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed {
    pub const FILE_STREAM: Self = Self(0);
    pub const MEMORY_STREAM: Self = Self(1);
}
pub type STREAM_TYPE = BYTE;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_0 {
    pub const STREAM_READ: Self = Self(0);
    pub const STREAM_WRITE: Self = Self(1);
}
pub type STREAM_MODE = BYTE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct brief_ship_info {
    pub race_id: BYTE,
    pub crew_level: COUNT,
    pub max_crew: COUNT,
    pub max_energy: BYTE,
}
pub type BRIEF_SHIP_INFO = brief_ship_info;
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
pub type HENCOUNTER = HLINK;
pub type ENCOUNTER = encounter;
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_5(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_5 {
    pub const DEFGRP_NONE: Self = Self(0);
    pub const DEFGRP_SHOFIXTI: Self = Self(1);
    pub const DEFGRP_ZOQFOT: Self = Self(2);
    pub const DEFGRP_MELNORME0: Self = Self(3);
    pub const DEFGRP_MELNORME1: Self = Self(4);
    pub const DEFGRP_MELNORME2: Self = Self(5);
    pub const DEFGRP_MELNORME3: Self = Self(6);
    pub const DEFGRP_MELNORME4: Self = Self(7);
    pub const DEFGRP_MELNORME5: Self = Self(8);
    pub const DEFGRP_MELNORME6: Self = Self(9);
    pub const DEFGRP_MELNORME7: Self = Self(10);
    pub const DEFGRP_MELNORME8: Self = Self(11);
    pub const DEFGRP_URQUAN_PROBE: Self = Self(12);
    pub const DEFGRP_COLONY: Self = Self(13);
    pub const DEFGRP_SAMATRA: Self = Self(14);
    pub const NUM_DEFGRPS: Self = Self(15);
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
pub type HIPGROUP = HLINK;
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
}
pub type GAMESTATE_TRANSPOSE = struct_GAMESTATE_TRANSPOSE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct struct_GAMESTATE_TRANSPOSE {
    pub start: ::core::ffi::c_int,
    pub end: ::core::ffi::c_int,
    pub target: ::core::ffi::c_int,
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
                329 as ::core::ffi::c_int,
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
                395 as ::core::ffi::c_int,
                b"LockFleetInfo\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    return LockLink(pq, h) as *mut FLEET_INFO;
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
                80 as ::core::ffi::c_int,
                b"LockIpGroup\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    return LockLink(pq, h) as *mut IP_GROUP;
}
#[inline]
unsafe extern "C" fn cread_8(mut fh: DECODE_REF, mut v: *mut BYTE) -> COUNT {
    let mut t: BYTE = 0;
    if v.is_null() {
        v = &raw mut t;
    }
    return cread(v as *mut ::core::ffi::c_void, 1 as COUNT, 1 as COUNT, fh);
}
#[inline]
unsafe extern "C" fn cread_16(mut fh: DECODE_REF, mut v: *mut UWORD) -> COUNT {
    let mut t: UWORD = 0;
    if v.is_null() {
        v = &raw mut t;
    }
    return cread(v as *mut ::core::ffi::c_void, 2 as COUNT, 1 as COUNT, fh);
}
#[inline]
unsafe extern "C" fn cread_16s(mut fh: DECODE_REF, mut v: *mut SWORD) -> COUNT {
    let mut t: UWORD = 0;
    let mut ret: COUNT = 0;
    ret = cread_16(fh, &raw mut t);
    if !v.is_null() {
        *v = t as SWORD;
    }
    return ret;
}
#[inline]
unsafe extern "C" fn cread_32(mut fh: DECODE_REF, mut v: *mut DWORD) -> COUNT {
    let mut t: DWORD = 0;
    if v.is_null() {
        v = &raw mut t;
    }
    return cread(v as *mut ::core::ffi::c_void, 4 as COUNT, 1 as COUNT, fh);
}
#[inline]
unsafe extern "C" fn cread_32s(mut fh: DECODE_REF, mut v: *mut SDWORD) -> COUNT {
    let mut t: DWORD = 0;
    let mut ret: COUNT = 0;
    ret = cread_32(fh, &raw mut t);
    if !v.is_null() {
        *v = t as SDWORD;
    }
    return ret;
}
#[inline]
unsafe extern "C" fn cread_ptr(mut fh: DECODE_REF) -> COUNT {
    let mut t: DWORD = 0;
    return cread_32(fh, &raw mut t);
}
#[inline]
unsafe extern "C" fn cread_a8(
    mut fh: DECODE_REF,
    mut ar: *mut BYTE,
    mut count: COUNT,
) -> COUNT {
    (!ar.is_null()
        || {
            __assert_fail(
                b"ar != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/uqm/load_legacy.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                103 as ::core::ffi::c_int,
                b"cread_a8\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    return (cread(ar as *mut ::core::ffi::c_void, 1 as COUNT, count, fh)
        as ::core::ffi::c_int == count as ::core::ffi::c_int) as ::core::ffi::c_int
        as COUNT;
}
#[inline]
unsafe extern "C" fn read_8(
    mut fp: *mut ::core::ffi::c_void,
    mut v: *mut BYTE,
) -> size_t {
    let mut t: BYTE = 0;
    if v.is_null() {
        v = &raw mut t;
    }
    return ReadResFile(
        v as *mut ::core::ffi::c_void,
        1 as size_t,
        1 as size_t,
        fp as *mut uio_Stream,
    );
}
#[inline]
unsafe extern "C" fn read_16(
    mut fp: *mut ::core::ffi::c_void,
    mut v: *mut UWORD,
) -> size_t {
    let mut t: UWORD = 0;
    if v.is_null() {
        v = &raw mut t;
    }
    return ReadResFile(
        v as *mut ::core::ffi::c_void,
        2 as size_t,
        1 as size_t,
        fp as *mut uio_Stream,
    );
}
#[inline]
unsafe extern "C" fn read_32(
    mut fp: *mut ::core::ffi::c_void,
    mut v: *mut DWORD,
) -> size_t {
    let mut t: DWORD = 0;
    if v.is_null() {
        v = &raw mut t;
    }
    return ReadResFile(
        v as *mut ::core::ffi::c_void,
        4 as size_t,
        1 as size_t,
        fp as *mut uio_Stream,
    );
}
#[inline]
unsafe extern "C" fn read_32s(
    mut fp: *mut ::core::ffi::c_void,
    mut v: *mut SDWORD,
) -> size_t {
    let mut t: DWORD = 0;
    let mut ret: COUNT = 0;
    ret = read_32(fp, &raw mut t) as COUNT;
    if !v.is_null() {
        *v = t as SDWORD;
    }
    return ret as size_t;
}
#[inline]
unsafe extern "C" fn read_a8(
    mut fp: *mut ::core::ffi::c_void,
    mut ar: *mut BYTE,
    mut count: COUNT,
) -> size_t {
    (!ar.is_null()
        || {
            __assert_fail(
                b"ar != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/uqm/load_legacy.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                157 as ::core::ffi::c_int,
                b"read_a8\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    return (ReadResFile(
        ar as *mut ::core::ffi::c_void,
        1 as size_t,
        count as size_t,
        fp as *mut uio_Stream,
    ) == count as size_t) as ::core::ffi::c_int as size_t;
}
#[inline]
unsafe extern "C" fn read_str(
    mut fp: *mut ::core::ffi::c_void,
    mut str: *mut ::core::ffi::c_char,
    mut count: COUNT,
) -> size_t {
    return read_a8(fp, str as *mut BYTE, count);
}
#[inline]
unsafe extern "C" fn read_a16(
    mut fp: *mut ::core::ffi::c_void,
    mut ar: *mut UWORD,
    mut count: COUNT,
) -> size_t {
    (!ar.is_null()
        || {
            __assert_fail(
                b"ar != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/uqm/load_legacy.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                171 as ::core::ffi::c_int,
                b"read_a16\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    while count as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
        if read_16(fp, ar) != 1 as size_t {
            return 0 as size_t;
        }
        count = count.wrapping_sub(1);
        ar = ar.offset(1);
    }
    return 1 as size_t;
}
static mut transpose: [GAMESTATE_TRANSPOSE; 6] = [
    struct_GAMESTATE_TRANSPOSE {
        start: 0 as ::core::ffi::c_int,
        end: 51 as ::core::ffi::c_int,
        target: 0 as ::core::ffi::c_int,
    },
    struct_GAMESTATE_TRANSPOSE {
        start: 404 as ::core::ffi::c_int,
        end: 450 as ::core::ffi::c_int,
        target: 52 as ::core::ffi::c_int,
    },
    struct_GAMESTATE_TRANSPOSE {
        start: 483 as ::core::ffi::c_int,
        end: 878 as ::core::ffi::c_int,
        target: 99 as ::core::ffi::c_int,
    },
    struct_GAMESTATE_TRANSPOSE {
        start: 911 as ::core::ffi::c_int,
        end: 930 as ::core::ffi::c_int,
        target: 495 as ::core::ffi::c_int,
    },
    struct_GAMESTATE_TRANSPOSE {
        start: 963 as ::core::ffi::c_int,
        end: 1237 as ::core::ffi::c_int,
        target: 515 as ::core::ffi::c_int,
    },
    struct_GAMESTATE_TRANSPOSE {
        start: -1 as ::core::ffi::c_int,
        end: -1 as ::core::ffi::c_int,
        target: -1 as ::core::ffi::c_int,
    },
];
static mut old_defgrp_offsets: [DWORD; 15] = [
    0 as DWORD,
    52 as DWORD,
    84 as DWORD,
    116 as DWORD,
    148 as DWORD,
    180 as DWORD,
    212 as DWORD,
    244 as DWORD,
    276 as DWORD,
    308 as DWORD,
    340 as DWORD,
    372 as DWORD,
    451 as DWORD,
    879 as DWORD,
    931 as DWORD,
];
static mut new_defgrp_offsets: [DWORD; 15] = [
    0 as DWORD,
    C2Rust_Unnamed_3::SHOFIXTI_GRPOFFS0.0 as ::core::ffi::c_int as DWORD,
    C2Rust_Unnamed_3::ZOQFOT_GRPOFFS0.0 as ::core::ffi::c_int as DWORD,
    C2Rust_Unnamed_3::MELNORME0_GRPOFFS0.0 as ::core::ffi::c_int as DWORD,
    C2Rust_Unnamed_3::MELNORME1_GRPOFFS0.0 as ::core::ffi::c_int as DWORD,
    C2Rust_Unnamed_3::MELNORME2_GRPOFFS0.0 as ::core::ffi::c_int as DWORD,
    C2Rust_Unnamed_3::MELNORME3_GRPOFFS0.0 as ::core::ffi::c_int as DWORD,
    C2Rust_Unnamed_3::MELNORME4_GRPOFFS0.0 as ::core::ffi::c_int as DWORD,
    C2Rust_Unnamed_3::MELNORME5_GRPOFFS0.0 as ::core::ffi::c_int as DWORD,
    C2Rust_Unnamed_3::MELNORME6_GRPOFFS0.0 as ::core::ffi::c_int as DWORD,
    C2Rust_Unnamed_3::MELNORME7_GRPOFFS0.0 as ::core::ffi::c_int as DWORD,
    C2Rust_Unnamed_3::MELNORME8_GRPOFFS0.0 as ::core::ffi::c_int as DWORD,
    C2Rust_Unnamed_3::URQUAN_PROBE_GRPOFFS0.0 as ::core::ffi::c_int as DWORD,
    C2Rust_Unnamed_3::COLONY_GRPOFFS0.0 as ::core::ffi::c_int as DWORD,
    C2Rust_Unnamed_3::SAMATRA_GRPOFFS0.0 as ::core::ffi::c_int as DWORD,
];
unsafe extern "C" fn InterpretLegacyGameState(
    mut result: *mut BYTE,
    mut legacy: *mut BYTE,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut grpoffs: [DWORD; 15] = [0; 15];
    let mut t: *mut GAMESTATE_TRANSPOSE = (&raw mut transpose
        as *mut GAMESTATE_TRANSPOSE)
        .offset(0isize);
    grpoffs[0usize] = 0 as DWORD;
    i = 1 as ::core::ffi::c_int;
    while i < C2Rust_Unnamed_5::NUM_DEFGRPS.0 as ::core::ffi::c_int {
        grpoffs[i as usize] = getGameState32(
            legacy,
            old_defgrp_offsets[i as usize] as ::core::ffi::c_int,
        );
        i += 1;
    }
    while (*t).start >= 0 as ::core::ffi::c_int {
        copyGameState(
            result,
            (*t).target as DWORD,
            legacy,
            (*t).start as DWORD,
            (*t).end as DWORD,
        );
        t = t.offset(1);
    }
    i = 1 as ::core::ffi::c_int;
    while i < C2Rust_Unnamed_5::NUM_DEFGRPS.0 as ::core::ffi::c_int {
        setGameState32(
            result,
            new_defgrp_offsets[i as usize] as ::core::ffi::c_int,
            grpoffs[i as usize],
        );
        i += 1;
    }
}
unsafe extern "C" fn LoadEmptyQueue(mut fh: DECODE_REF) {
    let mut num_links: COUNT = 0;
    cread_16(fh, &raw mut num_links);
    if num_links != 0 {
        log_add(
            log_Level::log_Error,
            b"LoadEmptyQueue(): BUG: the queue is not empty!\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        explode();
    }
}
unsafe extern "C" fn LoadShipQueue(mut fh: DECODE_REF, mut pQueue: *mut QUEUE) {
    let mut num_links: COUNT = 0;
    cread_16(fh, &raw mut num_links);
    loop {
        let c2rust_fresh2 = num_links;
        num_links = num_links.wrapping_sub(1);
        if c2rust_fresh2 == 0 {
            break;
        }
        let mut hStarShip: HSHIPFRAG = ::core::ptr::null_mut::<::core::ffi::c_void>();
        let mut FragPtr: *mut SHIP_FRAGMENT = ::core::ptr::null_mut::<SHIP_FRAGMENT>();
        let mut Index: COUNT = 0;
        let mut tmpb: BYTE = 0;
        cread_16(fh, &raw mut Index);
        hStarShip = CloneShipFragment(Index, pQueue, 0 as COUNT);
        FragPtr = LockShipFrag(pQueue, hStarShip);
        cread_16(fh, ::core::ptr::null_mut::<UWORD>());
        cread_8(fh, &raw mut (*FragPtr).captains_name_index);
        cread_8(fh, ::core::ptr::null_mut::<BYTE>());
        cread_16(fh, ::core::ptr::null_mut::<UWORD>());
        cread_8(fh, &raw mut (*FragPtr).race_id);
        cread_8(fh, &raw mut (*FragPtr).index);
        cread_8(fh, &raw mut tmpb);
        (*FragPtr).crew_level = tmpb as COUNT;
        cread_8(fh, &raw mut tmpb);
        (*FragPtr).max_crew = tmpb as COUNT;
        cread_8(fh, &raw mut (*FragPtr).energy_level);
        cread_8(fh, &raw mut (*FragPtr).max_energy);
        cread_16(fh, ::core::ptr::null_mut::<UWORD>());
        cread_16(fh, ::core::ptr::null_mut::<UWORD>());
        UnlockLink(pQueue, hStarShip);
    };
}
unsafe extern "C" fn LoadRaceQueue(mut fh: DECODE_REF, mut pQueue: *mut QUEUE) {
    let mut num_links: COUNT = 0;
    cread_16(fh, &raw mut num_links);
    loop {
        let c2rust_fresh4 = num_links;
        num_links = num_links.wrapping_sub(1);
        if c2rust_fresh4 == 0 {
            break;
        }
        let mut hStarShip: HFLEETINFO = ::core::ptr::null_mut::<::core::ffi::c_void>();
        let mut FleetPtr: *mut FLEET_INFO = ::core::ptr::null_mut::<FLEET_INFO>();
        let mut Index: COUNT = 0;
        let mut tmpb: BYTE = 0;
        cread_16(fh, &raw mut Index);
        hStarShip = GetStarShipFromIndex(pQueue, Index) as HFLEETINFO;
        FleetPtr = LockFleetInfo(pQueue, hStarShip);
        cread_16(fh, &raw mut (*FleetPtr).allied_state);
        cread_8(fh, &raw mut (*FleetPtr).days_left);
        cread_8(fh, &raw mut (*FleetPtr).growth_fract);
        cread_8(fh, &raw mut tmpb);
        (*FleetPtr).crew_level = tmpb as COUNT;
        cread_8(fh, &raw mut tmpb);
        (*FleetPtr).max_crew = tmpb as COUNT;
        cread_8(fh, &raw mut (*FleetPtr).growth);
        cread_8(fh, &raw mut (*FleetPtr).max_energy);
        cread_16s(fh, &raw mut (*FleetPtr).loc.x);
        cread_16s(fh, &raw mut (*FleetPtr).loc.y);
        cread_16(fh, &raw mut (*FleetPtr).actual_strength);
        cread_16(fh, &raw mut (*FleetPtr).known_strength);
        cread_16s(fh, &raw mut (*FleetPtr).known_loc.x);
        cread_16s(fh, &raw mut (*FleetPtr).known_loc.y);
        cread_8(fh, &raw mut (*FleetPtr).growth_err_term);
        cread_8(fh, &raw mut (*FleetPtr).func_index);
        cread_16s(fh, &raw mut (*FleetPtr).dest_loc.x);
        cread_16s(fh, &raw mut (*FleetPtr).dest_loc.y);
        cread_16(fh, ::core::ptr::null_mut::<UWORD>());
        UnlockLink(pQueue, hStarShip);
    };
}
unsafe extern "C" fn LoadGroupQueue(mut fh: DECODE_REF, mut pQueue: *mut QUEUE) {
    let mut num_links: COUNT = 0;
    cread_16(fh, &raw mut num_links);
    loop {
        let c2rust_fresh3 = num_links;
        num_links = num_links.wrapping_sub(1);
        if c2rust_fresh3 == 0 {
            break;
        }
        let mut hGroup: HIPGROUP = ::core::ptr::null_mut::<::core::ffi::c_void>();
        let mut GroupPtr: *mut IP_GROUP = ::core::ptr::null_mut::<IP_GROUP>();
        let mut tmpb: BYTE = 0;
        cread_16(fh, ::core::ptr::null_mut::<UWORD>());
        hGroup = BuildGroup(pQueue, 0 as BYTE);
        GroupPtr = LockIpGroup(pQueue, hGroup);
        cread_16(fh, ::core::ptr::null_mut::<UWORD>());
        cread_8(fh, ::core::ptr::null_mut::<BYTE>());
        cread_8(fh, ::core::ptr::null_mut::<BYTE>());
        cread_16(fh, &raw mut (*GroupPtr).group_counter);
        cread_8(fh, &raw mut (*GroupPtr).race_id);
        cread_8(fh, &raw mut tmpb);
        (*GroupPtr).sys_loc = (tmpb as ::core::ffi::c_int
            & 0xf as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE;
        (*GroupPtr).task = (tmpb as ::core::ffi::c_int
            >> 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE;
        cread_8(fh, &raw mut (*GroupPtr).in_system);
        cread_8(fh, ::core::ptr::null_mut::<BYTE>());
        cread_8(fh, &raw mut tmpb);
        (*GroupPtr).dest_loc = (tmpb as ::core::ffi::c_int
            & 0xf as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE;
        (*GroupPtr).orbit_pos = (tmpb as ::core::ffi::c_int
            >> 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE;
        cread_8(fh, &raw mut (*GroupPtr).group_id);
        cread_16s(fh, &raw mut (*GroupPtr).loc.x);
        cread_16s(fh, &raw mut (*GroupPtr).loc.y);
        UnlockLink(pQueue, hGroup);
    };
}
unsafe extern "C" fn LoadEncounter(
    mut EncounterPtr: *mut ENCOUNTER,
    mut fh: DECODE_REF,
) {
    let mut i: COUNT = 0;
    let mut tmpb: BYTE = 0;
    cread_ptr(fh);
    (*EncounterPtr).pred = ::core::ptr::null_mut::<::core::ffi::c_void>();
    cread_ptr(fh);
    (*EncounterPtr).succ = ::core::ptr::null_mut::<::core::ffi::c_void>();
    cread_ptr(fh);
    (*EncounterPtr).hElement = ::core::ptr::null_mut::<::core::ffi::c_void>();
    cread_16s(fh, &raw mut (*EncounterPtr).transition_state);
    cread_16s(fh, &raw mut (*EncounterPtr).origin.x);
    cread_16s(fh, &raw mut (*EncounterPtr).origin.y);
    cread_16(fh, &raw mut (*EncounterPtr).radius);
    cread_16s(fh, &raw mut (*EncounterPtr).loc_pt.x);
    cread_16s(fh, &raw mut (*EncounterPtr).loc_pt.y);
    cread_8(fh, &raw mut (*EncounterPtr).race_id);
    cread_8(fh, &raw mut tmpb);
    (*EncounterPtr).num_ships = (tmpb as ::core::ffi::c_int & 0xf as ::core::ffi::c_int)
        as BYTE;
    (*EncounterPtr).flags = (tmpb as ::core::ffi::c_int & 0xf0 as ::core::ffi::c_int)
        as BYTE;
    cread_16(fh, ::core::ptr::null_mut::<UWORD>());
    i = 0 as COUNT;
    while (i as ::core::ffi::c_int) < 7 as ::core::ffi::c_int {
        let mut ShipInfo: *mut BRIEF_SHIP_INFO = (&raw mut (*EncounterPtr).ShipList
            as *mut BRIEF_SHIP_INFO)
            .offset(i as isize);
        cread_16(fh, ::core::ptr::null_mut::<UWORD>());
        cread_8(fh, &raw mut (*ShipInfo).race_id);
        cread_8(fh, ::core::ptr::null_mut::<BYTE>());
        cread_8(fh, &raw mut tmpb);
        (*ShipInfo).crew_level = tmpb as COUNT;
        cread_8(fh, &raw mut tmpb);
        (*ShipInfo).max_crew = tmpb as COUNT;
        cread_8(fh, ::core::ptr::null_mut::<BYTE>());
        cread_8(fh, &raw mut (*ShipInfo).max_energy);
        cread_16(fh, ::core::ptr::null_mut::<UWORD>());
        cread_16(fh, ::core::ptr::null_mut::<UWORD>());
        cread_32(fh, ::core::ptr::null_mut::<DWORD>());
        cread_ptr(fh);
        cread_ptr(fh);
        i = i.wrapping_add(1);
    }
    cread_32s(fh, &raw mut (*EncounterPtr).log_x);
    cread_32s(fh, &raw mut (*EncounterPtr).log_y);
}
unsafe extern "C" fn LoadEvent(mut EventPtr: *mut EVENT, mut fh: DECODE_REF) {
    cread_ptr(fh);
    (*EventPtr).pred = ::core::ptr::null_mut::<::core::ffi::c_void>();
    cread_ptr(fh);
    (*EventPtr).succ = ::core::ptr::null_mut::<::core::ffi::c_void>();
    cread_8(fh, &raw mut (*EventPtr).day_index);
    cread_8(fh, &raw mut (*EventPtr).month_index);
    cread_16(fh, &raw mut (*EventPtr).year_index);
    cread_8(fh, &raw mut (*EventPtr).func_index);
    cread_8(fh, ::core::ptr::null_mut::<BYTE>());
    cread_16(fh, ::core::ptr::null_mut::<UWORD>());
}
unsafe extern "C" fn DummyLoadQueue(mut QueuePtr: *mut QUEUE, mut fh: DECODE_REF) {
    cread_ptr(fh);
    cread_ptr(fh);
    cread_ptr(fh);
    cread_ptr(fh);
    cread_16(fh, ::core::ptr::null_mut::<UWORD>());
    cread_16(fh, ::core::ptr::null_mut::<UWORD>());
    cread_8(fh, ::core::ptr::null_mut::<BYTE>());
    cread_8(fh, ::core::ptr::null_mut::<BYTE>());
    cread_16(fh, ::core::ptr::null_mut::<UWORD>());
}
unsafe extern "C" fn LoadClockState(mut ClockPtr: *mut CLOCK_STATE, mut fh: DECODE_REF) {
    cread_8(fh, &raw mut (*ClockPtr).day_index);
    cread_8(fh, &raw mut (*ClockPtr).month_index);
    cread_16(fh, &raw mut (*ClockPtr).year_index);
    cread_16s(fh, &raw mut (*ClockPtr).tick_count);
    cread_16s(fh, &raw mut (*ClockPtr).day_in_ticks);
    cread_ptr(fh);
    cread_ptr(fh);
    cread_32(fh, ::core::ptr::null_mut::<DWORD>());
    DummyLoadQueue(&raw mut (*ClockPtr).event_q, fh);
}
unsafe extern "C" fn LoadGameState(mut GSPtr: *mut GAME_STATE, mut fh: DECODE_REF) {
    let mut dummy8: BYTE = 0;
    let mut oldstate: [BYTE; 155] = [0; 155];
    cread_8(fh, &raw mut dummy8);
    cread_8(fh, &raw mut (*GSPtr).glob_flags);
    cread_8(fh, &raw mut (*GSPtr).CrewCost);
    cread_8(fh, &raw mut (*GSPtr).FuelCost);
    cread_a8(
        fh,
        &raw mut (*GSPtr).ModuleCost as *mut BYTE,
        C2Rust_Unnamed_2::NUM_MODULES.0 as ::core::ffi::c_int as COUNT,
    );
    cread_a8(
        fh,
        &raw mut (*GSPtr).ElementWorth as *mut BYTE,
        C2Rust_Unnamed_1::NUM_ELEMENT_CATEGORIES.0 as ::core::ffi::c_int as COUNT,
    );
    cread_ptr(fh);
    cread_16(fh, &raw mut (*GSPtr).CurrentActivity);
    cread_16(fh, ::core::ptr::null_mut::<UWORD>());
    LoadClockState(&raw mut (*GSPtr).GameClock, fh);
    cread_16s(fh, &raw mut (*GSPtr).autopilot.x);
    cread_16s(fh, &raw mut (*GSPtr).autopilot.y);
    cread_16s(fh, &raw mut (*GSPtr).ip_location.x);
    cread_16s(fh, &raw mut (*GSPtr).ip_location.y);
    cread_16s(fh, &raw mut (*GSPtr).ShipStamp.origin.x);
    cread_16s(fh, &raw mut (*GSPtr).ShipStamp.origin.y);
    cread_16(fh, &raw mut (*GSPtr).ShipFacing);
    cread_8(fh, &raw mut (*GSPtr).ip_planet);
    cread_8(fh, &raw mut (*GSPtr).in_orbit);
    cread_16(fh, &raw mut (*GSPtr).velocity.TravelAngle);
    cread_16s(fh, &raw mut (*GSPtr).velocity.vector.width);
    cread_16s(fh, &raw mut (*GSPtr).velocity.vector.height);
    cread_16s(fh, &raw mut (*GSPtr).velocity.fract.width);
    cread_16s(fh, &raw mut (*GSPtr).velocity.fract.height);
    cread_16s(fh, &raw mut (*GSPtr).velocity.error.width);
    cread_16s(fh, &raw mut (*GSPtr).velocity.error.height);
    cread_16s(fh, &raw mut (*GSPtr).velocity.incr.width);
    cread_16s(fh, &raw mut (*GSPtr).velocity.incr.height);
    cread_16(fh, ::core::ptr::null_mut::<UWORD>());
    cread_32(fh, &raw mut (*GSPtr).BattleGroupRef);
    DummyLoadQueue(&raw mut (*GSPtr).avail_race_q, fh);
    DummyLoadQueue(&raw mut (*GSPtr).npc_built_ship_q, fh);
    DummyLoadQueue(&raw mut (*GSPtr).encounter_q, fh);
    DummyLoadQueue(&raw mut (*GSPtr).built_ship_q, fh);
    cread_a8(fh, &raw mut oldstate as *mut BYTE, 155 as COUNT);
    InterpretLegacyGameState(
        &raw mut (*GSPtr).GameState as *mut BYTE,
        &raw mut oldstate as *mut BYTE,
    );
    cread_8(fh, ::core::ptr::null_mut::<BYTE>());
}
unsafe extern "C" fn LoadSisState(
    mut SSPtr: *mut SIS_STATE,
    mut fp: *mut ::core::ffi::c_void,
) -> BOOLEAN {
    if read_32s(fp, &raw mut (*SSPtr).log_x) != 1 as size_t
        || read_32s(fp, &raw mut (*SSPtr).log_y) != 1 as size_t
        || read_32(fp, &raw mut (*SSPtr).ResUnits) != 1 as size_t
        || read_32(fp, &raw mut (*SSPtr).FuelOnBoard) != 1 as size_t
        || read_16(fp, &raw mut (*SSPtr).CrewEnlisted) != 1 as size_t
        || read_16(fp, &raw mut (*SSPtr).TotalElementMass) != 1 as size_t
        || read_16(fp, &raw mut (*SSPtr).TotalBioMass) != 1 as size_t
        || read_a8(fp, &raw mut (*SSPtr).ModuleSlots as *mut BYTE, 16 as COUNT)
            != 1 as size_t
        || read_a8(fp, &raw mut (*SSPtr).DriveSlots as *mut BYTE, 11 as COUNT)
            != 1 as size_t
        || read_a8(fp, &raw mut (*SSPtr).JetSlots as *mut BYTE, 8 as COUNT)
            != 1 as size_t || read_8(fp, &raw mut (*SSPtr).NumLanders) != 1 as size_t
        || read_a16(
            fp,
            &raw mut (*SSPtr).ElementAmounts as *mut UWORD,
            C2Rust_Unnamed_1::NUM_ELEMENT_CATEGORIES.0 as ::core::ffi::c_int as COUNT,
        ) != 1 as size_t
        || read_str(
            fp,
            &raw mut (*SSPtr).ShipName as *mut ::core::ffi::c_char,
            16 as COUNT,
        ) != 1 as size_t
        || read_str(
            fp,
            &raw mut (*SSPtr).CommanderName as *mut ::core::ffi::c_char,
            16 as COUNT,
        ) != 1 as size_t
        || read_str(
            fp,
            &raw mut (*SSPtr).PlanetName as *mut ::core::ffi::c_char,
            16 as COUNT,
        ) != 1 as size_t || read_16(fp, ::core::ptr::null_mut::<UWORD>()) != 1 as size_t
    {
        return BOOLEAN::FALSE
    } else {
        return BOOLEAN::TRUE
    };
}
unsafe extern "C" fn LoadSummary(
    mut SummPtr: *mut SUMMARY_DESC,
    mut fp: *mut ::core::ffi::c_void,
) -> BOOLEAN {
    if LoadSisState(&raw mut (*SummPtr).SS, fp).0 == 0 {
        return BOOLEAN::FALSE;
    }
    if read_8(fp, &raw mut (*SummPtr).Activity) != 1 as size_t
        || read_8(fp, &raw mut (*SummPtr).Flags) != 1 as size_t
        || read_8(fp, &raw mut (*SummPtr).day_index) != 1 as size_t
        || read_8(fp, &raw mut (*SummPtr).month_index) != 1 as size_t
        || read_16(fp, &raw mut (*SummPtr).year_index) != 1 as size_t
        || read_8(fp, &raw mut (*SummPtr).MCreditLo) != 1 as size_t
        || read_8(fp, &raw mut (*SummPtr).MCreditHi) != 1 as size_t
        || read_8(fp, &raw mut (*SummPtr).NumShips) != 1 as size_t
        || read_8(fp, &raw mut (*SummPtr).NumDevices) != 1 as size_t
        || read_a8(fp, &raw mut (*SummPtr).ShipList as *mut BYTE, 12 as COUNT)
            != 1 as size_t
        || read_a8(fp, &raw mut (*SummPtr).DeviceList as *mut BYTE, 16 as COUNT)
            != 1 as size_t
        || read_16(fp, ::core::ptr::null_mut::<UWORD>()) != 1 as size_t
    {
        return BOOLEAN::FALSE
    } else {
        return BOOLEAN::TRUE
    };
}
unsafe extern "C" fn LoadStarDesc(mut SDPtr: *mut STAR_DESC, mut fh: DECODE_REF) {
    cread_16s(fh, &raw mut (*SDPtr).star_pt.x);
    cread_16s(fh, &raw mut (*SDPtr).star_pt.y);
    cread_8(fh, &raw mut (*SDPtr).Type);
    cread_8(fh, &raw mut (*SDPtr).Index);
    cread_8(fh, &raw mut (*SDPtr).Prefix);
    cread_8(fh, &raw mut (*SDPtr).Postfix);
}
#[no_mangle]
pub unsafe extern "C" fn LoadLegacyGame(
    mut which_game: COUNT,
    mut SummPtr: *mut SUMMARY_DESC,
) -> BOOLEAN {
    let mut in_fp: *mut uio_Stream = ::core::ptr::null_mut::<uio_Stream>();
    let mut file: [::core::ffi::c_char; 4096] = [0; 4096];
    let mut buf: [::core::ffi::c_char; 256] = [0; 256];
    let mut loc_sd: SUMMARY_DESC = SUMMARY_DESC {
        SS: SIS_STATE {
            log_x: 0,
            log_y: 0,
            ResUnits: 0,
            FuelOnBoard: 0,
            CrewEnlisted: 0,
            TotalElementMass: 0,
            TotalBioMass: 0,
            ModuleSlots: [0; 16],
            DriveSlots: [0; 11],
            JetSlots: [0; 8],
            NumLanders: 0,
            ElementAmounts: [0; 8],
            ShipName: [0; 16],
            CommanderName: [0; 16],
            PlanetName: [0; 16],
        },
        Activity: 0,
        Flags: 0,
        day_index: 0,
        month_index: 0,
        year_index: 0,
        MCreditLo: 0,
        MCreditHi: 0,
        NumShips: 0,
        NumDevices: 0,
        ShipList: [0; 12],
        DeviceList: [0; 16],
        SaveName: [0; 64],
    };
    let mut fp: *mut GAME_STATE_FILE = ::core::ptr::null_mut::<GAME_STATE_FILE>();
    let mut fh: DECODE_REF = ::core::ptr::null_mut::<_LZHCODE_DESC>();
    let mut num_links: COUNT = 0;
    let mut SD: STAR_DESC = STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    };
    let mut Activity: ACTIVITY = 0;
    sprintf(
        &raw mut file as *mut ::core::ffi::c_char,
        b"starcon2.%02u\0".as_ptr() as *const ::core::ffi::c_char,
        which_game as ::core::ffi::c_int,
    );
    in_fp = res_OpenResFile(
        saveDir,
        &raw mut file as *mut ::core::ffi::c_char,
        b"rb\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if in_fp.is_null() {
        return BOOLEAN::FALSE;
    }
    loc_sd.SaveName[0usize] = '\0' as UNICODE;
    if LoadSummary(&raw mut loc_sd, in_fp as *mut ::core::ffi::c_void).0 == 0 {
        log_add(
            log_Level::log_Error,
            b"Warning: Savegame is corrupt\0".as_ptr() as *const ::core::ffi::c_char,
        );
        res_CloseResFile(in_fp);
        return BOOLEAN::FALSE;
    }
    if SummPtr.is_null() {
        SummPtr = &raw mut loc_sd;
    } else {
        memcpy(
            SummPtr as *mut ::core::ffi::c_void,
            &raw mut loc_sd as *const ::core::ffi::c_void,
            ::core::mem::size_of::<SUMMARY_DESC>(),
        );
        res_CloseResFile(in_fp);
        return BOOLEAN::TRUE;
    }
    if ((*SummPtr).year_index as ::core::ffi::c_int) < 2155 as ::core::ffi::c_int
        || (*SummPtr).year_index as ::core::ffi::c_int
            >= 2155 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                + 25 as ::core::ffi::c_int
    {
        log_add(
            log_Level::log_Error,
            b"Warning: Savegame corrupt or from an incompatible platform.\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        res_CloseResFile(in_fp);
        return BOOLEAN::FALSE;
    }
    GlobData.SIS_state = (*SummPtr).SS;
    fh = copen(
        in_fp as *mut ::core::ffi::c_void,
        C2Rust_Unnamed::FILE_STREAM.0 as ::core::ffi::c_int as STREAM_TYPE,
        C2Rust_Unnamed_0::STREAM_READ.0 as ::core::ffi::c_int as STREAM_MODE,
    );
    if fh.is_null() {
        res_CloseResFile(in_fp);
        return BOOLEAN::FALSE;
    }
    ReinitQueue(&raw mut GlobData.Game_state.GameClock.event_q);
    ReinitQueue(&raw mut GlobData.Game_state.encounter_q);
    ReinitQueue(&raw mut GlobData.Game_state.ip_group_q);
    ReinitQueue(&raw mut GlobData.Game_state.npc_built_ship_q);
    ReinitQueue(&raw mut GlobData.Game_state.built_ship_q);
    memset(
        (&raw mut GlobData.Game_state.GameState as *mut BYTE).offset(0isize)
            as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[BYTE; 155]>(),
    );
    Activity = GlobData.Game_state.CurrentActivity;
    LoadGameState(&raw mut GlobData.Game_state, fh);
    NextActivity = GlobData.Game_state.CurrentActivity;
    GlobData.Game_state.CurrentActivity = Activity;
    LoadRaceQueue(fh, &raw mut GlobData.Game_state.avail_race_q);
    if NextActivity as ::core::ffi::c_int
        & C2Rust_Unnamed_4::START_INTERPLANETARY.0 as ::core::ffi::c_int == 0
    {
        if NextActivity as ::core::ffi::c_int
            & C2Rust_Unnamed_4::START_ENCOUNTER.0 as ::core::ffi::c_int != 0
        {
            LoadShipQueue(fh, &raw mut GlobData.Game_state.npc_built_ship_q);
        } else if NextActivity as BYTE as ::core::ffi::c_int
            == C2Rust_Unnamed_4::IN_INTERPLANETARY.0 as ::core::ffi::c_int
        {
            LoadGroupQueue(fh, &raw mut GlobData.Game_state.ip_group_q);
        } else {
            LoadEmptyQueue(fh);
        }
    }
    LoadShipQueue(fh, &raw mut GlobData.Game_state.built_ship_q);
    cread_16(fh, &raw mut num_links);
    loop {
        let c2rust_fresh0 = num_links;
        num_links = num_links.wrapping_sub(1);
        if c2rust_fresh0 == 0 {
            break;
        }
        let mut hEvent: HEVENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
        let mut EventPtr: *mut EVENT = ::core::ptr::null_mut::<EVENT>();
        hEvent = AllocLink(&raw mut GlobData.Game_state.GameClock.event_q) as HEVENT;
        EventPtr = LockLink(&raw mut GlobData.Game_state.GameClock.event_q, hEvent)
            as *mut EVENT;
        LoadEvent(EventPtr, fh);
        UnlockLink(&raw mut GlobData.Game_state.GameClock.event_q, hEvent);
        PutQueue(&raw mut GlobData.Game_state.GameClock.event_q, hEvent);
    }
    cread_16(fh, &raw mut num_links);
    loop {
        let c2rust_fresh1 = num_links;
        num_links = num_links.wrapping_sub(1);
        if c2rust_fresh1 == 0 {
            break;
        }
        let mut hEncounter: HENCOUNTER = ::core::ptr::null_mut::<::core::ffi::c_void>();
        let mut EncounterPtr: *mut ENCOUNTER = ::core::ptr::null_mut::<ENCOUNTER>();
        hEncounter = AllocLink(&raw mut GlobData.Game_state.encounter_q) as HENCOUNTER;
        EncounterPtr = LockLink(&raw mut GlobData.Game_state.encounter_q, hEncounter)
            as *mut ENCOUNTER;
        LoadEncounter(EncounterPtr, fh);
        UnlockLink(&raw mut GlobData.Game_state.encounter_q, hEncounter);
        PutQueue(&raw mut GlobData.Game_state.encounter_q, hEncounter);
    }
    fp = OpenStateFile(
        0 as ::core::ffi::c_int,
        b"wb\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !fp.is_null() {
        let mut flen: DWORD = 0;
        cread_32(fh, &raw mut flen);
        while flen != 0 {
            let mut num_bytes: COUNT = 0;
            num_bytes = (if flen as usize
                >= ::core::mem::size_of::<[::core::ffi::c_char; 256]>()
            {
                ::core::mem::size_of::<[::core::ffi::c_char; 256]>()
            } else {
                flen as COUNT as usize
            }) as COUNT;
            cread(
                &raw mut buf as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
                num_bytes,
                1 as COUNT,
                fh,
            );
            WriteStateFile(
                &raw mut buf as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
                num_bytes,
                1 as COUNT,
                fp,
            );
            flen = flen.wrapping_sub(num_bytes as DWORD);
        }
        CloseStateFile(fp);
    }
    fp = OpenStateFile(
        2 as ::core::ffi::c_int,
        b"wb\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !fp.is_null() {
        let mut flen_0: DWORD = 0;
        cread_32(fh, &raw mut flen_0);
        while flen_0 != 0 {
            let mut num_bytes_0: COUNT = 0;
            num_bytes_0 = (if flen_0 as usize
                >= ::core::mem::size_of::<[::core::ffi::c_char; 256]>()
            {
                ::core::mem::size_of::<[::core::ffi::c_char; 256]>()
            } else {
                flen_0 as COUNT as usize
            }) as COUNT;
            cread(
                &raw mut buf as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
                num_bytes_0,
                1 as COUNT,
                fh,
            );
            WriteStateFile(
                &raw mut buf as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
                num_bytes_0,
                1 as COUNT,
                fp,
            );
            flen_0 = flen_0.wrapping_sub(num_bytes_0 as DWORD);
        }
        CloseStateFile(fp);
    }
    fp = OpenStateFile(
        1 as ::core::ffi::c_int,
        b"wb\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !fp.is_null() {
        let mut flen_1: DWORD = 0;
        cread_32(fh, &raw mut flen_1);
        while flen_1 != 0 {
            let mut num_bytes_1: COUNT = 0;
            num_bytes_1 = (if flen_1 as usize
                >= ::core::mem::size_of::<[::core::ffi::c_char; 256]>()
            {
                ::core::mem::size_of::<[::core::ffi::c_char; 256]>()
            } else {
                flen_1 as COUNT as usize
            }) as COUNT;
            cread(
                &raw mut buf as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
                num_bytes_1,
                1 as COUNT,
                fh,
            );
            WriteStateFile(
                &raw mut buf as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
                num_bytes_1,
                1 as COUNT,
                fp,
            );
            flen_1 = flen_1.wrapping_sub(num_bytes_1 as DWORD);
        }
        CloseStateFile(fp);
    }
    LoadStarDesc(&raw mut SD, fh);
    cclose(fh);
    res_CloseResFile(in_fp);
    EncounterGroup = 0 as BYTE;
    EncounterRace = -1 as SIZE;
    ReinitQueue((&raw mut race_q as *mut QUEUE).offset(0isize));
    ReinitQueue((&raw mut race_q as *mut QUEUE).offset(1isize));
    CurStarDescPtr = FindStar(
        ::core::ptr::null_mut::<STAR_DESC>(),
        &raw mut SD.star_pt,
        0 as SIZE,
        0 as SIZE,
    );
    if NextActivity as ::core::ffi::c_int
        & C2Rust_Unnamed_4::START_ENCOUNTER.0 as ::core::ffi::c_int == 0
        && NextActivity as BYTE as ::core::ffi::c_int
            == C2Rust_Unnamed_4::IN_INTERPLANETARY.0 as ::core::ffi::c_int
    {
        NextActivity = (NextActivity as ::core::ffi::c_int
            | C2Rust_Unnamed_4::START_INTERPLANETARY.0 as ::core::ffi::c_int)
            as ACTIVITY;
    }
    return BOOLEAN::TRUE;
}
#[inline]
unsafe extern "C" fn explode() -> ! {
    abort();
}
