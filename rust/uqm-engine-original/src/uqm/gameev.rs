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
 *  Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA  02111-1307  USA
 */

// Rust translation of src/uqm/gameev.c.
extern "C" {
    pub type string_table_entry;
    pub type frame_desc;
    fn __assert_fail(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    );
    fn square_root(value: DWORD) -> COUNT;
    fn AddEvent(
        r#type: EVENT_TYPE,
        month_index: COUNT,
        day_index: COUNT,
        year_index: COUNT,
        func_index: BYTE,
    ) -> HEVENT;
    fn GetStarShipFromIndex(pShipQ: *mut QUEUE, Index: COUNT) -> HLINK;
    fn SetRaceAllied(race: COUNT, flag: BOOLEAN) -> COUNT;
    fn StartSphereTracking(race: COUNT) -> COUNT;
    static mut GlobData: GLOBDATA;
    fn getGameState(
        state: *mut BYTE,
        startBit: ::core::ffi::c_int,
        endBit: ::core::ffi::c_int,
    ) -> BYTE;
    fn setGameState(
        state: *mut BYTE,
        startBit: ::core::ffi::c_int,
        endBit: ::core::ffi::c_int,
        val: BYTE,
    );
    fn inHyperSpace() -> BOOLEAN;
    fn check_hyperspace_encounter();
    static mut CurStarDescPtr: *mut STAR_DESC;
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed {
    pub const ARILOU_ENTRANCE_EVENT: Self = Self(0);
    pub const ARILOU_EXIT_EVENT: Self = Self(1);
    pub const HYPERSPACE_ENCOUNTER_EVENT: Self = Self(2);
    pub const KOHR_AH_VICTORIOUS_EVENT: Self = Self(3);
    pub const ADVANCE_PKUNK_MISSION: Self = Self(4);
    pub const ADVANCE_THRADD_MISSION: Self = Self(5);
    pub const ZOQFOT_DISTRESS_EVENT: Self = Self(6);
    pub const ZOQFOT_DEATH_EVENT: Self = Self(7);
    pub const SHOFIXTI_RETURN_EVENT: Self = Self(8);
    pub const ADVANCE_UTWIG_SUPOX_MISSION: Self = Self(9);
    pub const KOHR_AH_GENOCIDE_EVENT: Self = Self(10);
    pub const SPATHI_SHIELD_EVENT: Self = Self(11);
    pub const ADVANCE_ILWRATH_MISSION: Self = Self(12);
    pub const ADVANCE_MYCON_MISSION: Self = Self(13);
    pub const ARILOU_UMGAH_CHECK: Self = Self(14);
    pub const YEHAT_REBEL_EVENT: Self = Self(15);
    pub const SLYLANDRO_RAMP_UP: Self = Self(16);
    pub const SLYLANDRO_RAMP_DOWN: Self = Self(17);
    pub const NUM_EVENTS: Self = Self(18);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_0 {
    pub const CLOSING: Self = Self(0);
    pub const OPENING: Self = Self(1);
}
pub type HEVENT = HLINK;
pub type HLINK = QUEUE_HANDLE;
pub type QUEUE_HANDLE = *mut ::core::ffi::c_void;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct EVENT_TYPE(pub ::core::ffi::c_uint);
impl EVENT_TYPE {
    pub const ABSOLUTE_EVENT: Self = Self(0);
    pub const RELATIVE_EVENT: Self = Self(1);
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
pub type LINK = link;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct link {
    pub pred: HLINK,
    pub succ: HLINK,
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
pub struct C2Rust_Unnamed_1(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_1 {
    pub const DEAD_GUY: Self = Self(0);
    pub const GOOD_GUY: Self = Self(1);
    pub const BAD_GUY: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_2(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_2 {
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
    pub const YEHAT_REBEL_SHIP: Self = Self(22);
    pub const URQUAN_DRONE_SHIP: Self = Self(23);
    pub const SAMATRA_SHIP: Self = Self(23);
    pub const NUM_AVAILABLE_RACES: Self = Self(24);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_3(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_3 {
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
    pub const RAINBOW_DEFINED: Self = Self(42);
    pub const ILWRATH_DEFINED: Self = Self(43);
    pub const ANDROSYNTH_DEFINED: Self = Self(44);
    pub const MYCON_TRAP_DEFINED: Self = Self(45);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_4(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_4 {
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
pub struct C2Rust_Unnamed_5(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_5 {
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
#[no_mangle]
pub unsafe extern "C" fn AddInitialGameEvents() {
    AddEvent(
        EVENT_TYPE::RELATIVE_EVENT,
        0 as COUNT,
        1 as COUNT,
        0 as COUNT,
        C2Rust_Unnamed::HYPERSPACE_ENCOUNTER_EVENT.0 as ::core::ffi::c_int as BYTE,
    );
    AddEvent(
        EVENT_TYPE::ABSOLUTE_EVENT,
        3 as COUNT,
        17 as COUNT,
        2155 as COUNT,
        C2Rust_Unnamed::ARILOU_ENTRANCE_EVENT.0 as ::core::ffi::c_int as BYTE,
    );
    AddEvent(
        EVENT_TYPE::RELATIVE_EVENT,
        0 as COUNT,
        0 as COUNT,
        4 as COUNT,
        C2Rust_Unnamed::KOHR_AH_VICTORIOUS_EVENT.0 as ::core::ffi::c_int as BYTE,
    );
    AddEvent(
        EVENT_TYPE::RELATIVE_EVENT,
        0 as COUNT,
        0 as COUNT,
        0 as COUNT,
        C2Rust_Unnamed::SLYLANDRO_RAMP_UP.0 as ::core::ffi::c_int as BYTE,
    );
}
#[no_mangle]
pub unsafe extern "C" fn EventHandler(mut selector: BYTE) {
    's_335: {
        match selector as ::core::ffi::c_int {
            0 => {
                arilou_entrance_event();
                break 's_335;
            }
            1 => {
                arilou_exit_event();
                break 's_335;
            }
            2 => {
                check_race_growth();
                if inHyperSpace().0 != 0 {
                    check_hyperspace_encounter();
                }
                AddEvent(
                    EVENT_TYPE::RELATIVE_EVENT,
                    0 as COUNT,
                    1 as COUNT,
                    0 as COUNT,
                    C2Rust_Unnamed::HYPERSPACE_ENCOUNTER_EVENT.0 as ::core::ffi::c_int
                        as BYTE,
                );
                break 's_335;
            }
            3 => {
                if getGameState(
                    &raw mut GlobData.Game_state.GameState as *mut BYTE,
                    C2Rust_Unnamed_4::UTWIG_SUPOX_MISSION.0 as ::core::ffi::c_int,
                    C2Rust_Unnamed_4::END_UTWIG_SUPOX_MISSION.0 as ::core::ffi::c_int,
                ) != 0
                {
                    AddEvent(
                        EVENT_TYPE::RELATIVE_EVENT,
                        0 as COUNT,
                        0 as COUNT,
                        1 as COUNT,
                        C2Rust_Unnamed::KOHR_AH_GENOCIDE_EVENT.0 as ::core::ffi::c_int
                            as BYTE,
                    );
                    break 's_335;
                }
            }
            10 => {}
            4 => {
                pkunk_mission();
                break 's_335;
            }
            5 => {
                thradd_mission();
                break 's_335;
            }
            6 => {
                if GlobData.Game_state.CurrentActivity as BYTE as ::core::ffi::c_int
                    == C2Rust_Unnamed_5::IN_INTERPLANETARY.0 as ::core::ffi::c_int
                    && !CurStarDescPtr.is_null()
                    && (*CurStarDescPtr).Index as ::core::ffi::c_int
                        == C2Rust_Unnamed_3::ZOQFOT_DEFINED.0 as ::core::ffi::c_int
                {
                    AddEvent(
                        EVENT_TYPE::RELATIVE_EVENT,
                        0 as COUNT,
                        7 as COUNT,
                        0 as COUNT,
                        C2Rust_Unnamed::ZOQFOT_DISTRESS_EVENT.0 as ::core::ffi::c_int
                            as BYTE,
                    );
                } else {
                    setGameState(
                        &raw mut GlobData.Game_state.GameState as *mut BYTE,
                        C2Rust_Unnamed_4::ZOQFOT_DISTRESS.0 as ::core::ffi::c_int,
                        C2Rust_Unnamed_4::END_ZOQFOT_DISTRESS.0 as ::core::ffi::c_int,
                        1 as BYTE,
                    );
                    AddEvent(
                        EVENT_TYPE::RELATIVE_EVENT,
                        6 as COUNT,
                        0 as COUNT,
                        0 as COUNT,
                        C2Rust_Unnamed::ZOQFOT_DEATH_EVENT.0 as ::core::ffi::c_int
                            as BYTE,
                    );
                }
                break 's_335;
            }
            7 => {
                if GlobData.Game_state.CurrentActivity as BYTE as ::core::ffi::c_int
                    == C2Rust_Unnamed_5::IN_INTERPLANETARY.0 as ::core::ffi::c_int
                    && !CurStarDescPtr.is_null()
                    && (*CurStarDescPtr).Index as ::core::ffi::c_int
                        == C2Rust_Unnamed_3::ZOQFOT_DEFINED.0 as ::core::ffi::c_int
                {
                    AddEvent(
                        EVENT_TYPE::RELATIVE_EVENT,
                        0 as COUNT,
                        7 as COUNT,
                        0 as COUNT,
                        C2Rust_Unnamed::ZOQFOT_DEATH_EVENT.0 as ::core::ffi::c_int
                            as BYTE,
                    );
                } else if getGameState(
                    &raw mut GlobData.Game_state.GameState as *mut BYTE,
                    C2Rust_Unnamed_4::ZOQFOT_DISTRESS.0 as ::core::ffi::c_int,
                    C2Rust_Unnamed_4::END_ZOQFOT_DISTRESS.0 as ::core::ffi::c_int,
                ) != 0
                {
                    let mut hZoqFot: HFLEETINFO = ::core::ptr::null_mut::<
                        ::core::ffi::c_void,
                    >();
                    let mut ZoqFotPtr: *mut FLEET_INFO = ::core::ptr::null_mut::<
                        FLEET_INFO,
                    >();
                    hZoqFot = GetStarShipFromIndex(
                        &raw mut GlobData.Game_state.avail_race_q,
                        C2Rust_Unnamed_2::ZOQFOTPIK_SHIP.0 as ::core::ffi::c_int as COUNT,
                    ) as HFLEETINFO;
                    ZoqFotPtr = LockFleetInfo(
                        &raw mut GlobData.Game_state.avail_race_q,
                        hZoqFot,
                    );
                    (*ZoqFotPtr).actual_strength = 0 as COUNT;
                    (*ZoqFotPtr).allied_state = C2Rust_Unnamed_1::DEAD_GUY.0
                        as ::core::ffi::c_int as UWORD;
                    UnlockLink(&raw mut GlobData.Game_state.avail_race_q, hZoqFot);
                    setGameState(
                        &raw mut GlobData.Game_state.GameState as *mut BYTE,
                        C2Rust_Unnamed_4::ZOQFOT_DISTRESS.0 as ::core::ffi::c_int,
                        C2Rust_Unnamed_4::END_ZOQFOT_DISTRESS.0 as ::core::ffi::c_int,
                        2 as BYTE,
                    );
                }
                break 's_335;
            }
            8 => {
                SetRaceAllied(
                    C2Rust_Unnamed_2::SHOFIXTI_SHIP.0 as ::core::ffi::c_int as COUNT,
                    BOOLEAN::TRUE,
                );
                GlobData.Game_state.CrewCost = (GlobData.Game_state.CrewCost
                    as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as BYTE;
                setGameState(
                    &raw mut GlobData.Game_state.GameState as *mut BYTE,
                    C2Rust_Unnamed_4::CREW_PURCHASED0.0 as ::core::ffi::c_int,
                    C2Rust_Unnamed_4::END_CREW_PURCHASED0.0 as ::core::ffi::c_int,
                    0 as BYTE,
                );
                setGameState(
                    &raw mut GlobData.Game_state.GameState as *mut BYTE,
                    C2Rust_Unnamed_4::CREW_PURCHASED1.0 as ::core::ffi::c_int,
                    C2Rust_Unnamed_4::END_CREW_PURCHASED1.0 as ::core::ffi::c_int,
                    0 as BYTE,
                );
                break 's_335;
            }
            9 => {
                utwig_supox_mission();
                break 's_335;
            }
            11 => {
                if GlobData.Game_state.CurrentActivity as BYTE as ::core::ffi::c_int
                    == C2Rust_Unnamed_5::IN_INTERPLANETARY.0 as ::core::ffi::c_int
                    && !CurStarDescPtr.is_null()
                    && (*CurStarDescPtr).Index as ::core::ffi::c_int
                        == C2Rust_Unnamed_3::SPATHI_DEFINED.0 as ::core::ffi::c_int
                {
                    AddEvent(
                        EVENT_TYPE::RELATIVE_EVENT,
                        0 as COUNT,
                        7 as COUNT,
                        0 as COUNT,
                        C2Rust_Unnamed::SPATHI_SHIELD_EVENT.0 as ::core::ffi::c_int
                            as BYTE,
                    );
                } else {
                    let mut hSpathi: HFLEETINFO = ::core::ptr::null_mut::<
                        ::core::ffi::c_void,
                    >();
                    let mut SpathiPtr: *mut FLEET_INFO = ::core::ptr::null_mut::<
                        FLEET_INFO,
                    >();
                    hSpathi = GetStarShipFromIndex(
                        &raw mut GlobData.Game_state.avail_race_q,
                        C2Rust_Unnamed_2::SPATHI_SHIP.0 as ::core::ffi::c_int as COUNT,
                    ) as HFLEETINFO;
                    SpathiPtr = LockFleetInfo(
                        &raw mut GlobData.Game_state.avail_race_q,
                        hSpathi,
                    );
                    if (*SpathiPtr).actual_strength != 0 {
                        SetRaceAllied(
                            C2Rust_Unnamed_2::SPATHI_SHIP.0 as ::core::ffi::c_int
                                as COUNT,
                            BOOLEAN::FALSE,
                        );
                        setGameState(
                            &raw mut GlobData.Game_state.GameState as *mut BYTE,
                            C2Rust_Unnamed_4::SPATHI_SHIELDED_SELVES.0
                                as ::core::ffi::c_int,
                            C2Rust_Unnamed_4::END_SPATHI_SHIELDED_SELVES.0
                                as ::core::ffi::c_int,
                            1 as BYTE,
                        );
                        (*SpathiPtr).actual_strength = 0 as COUNT;
                    }
                    UnlockLink(&raw mut GlobData.Game_state.avail_race_q, hSpathi);
                }
                break 's_335;
            }
            12 => {
                ilwrath_mission();
                break 's_335;
            }
            13 => {
                mycon_mission();
                break 's_335;
            }
            14 => {
                setGameState(
                    &raw mut GlobData.Game_state.GameState as *mut BYTE,
                    C2Rust_Unnamed_4::ARILOU_CHECKED_UMGAH.0 as ::core::ffi::c_int,
                    C2Rust_Unnamed_4::END_ARILOU_CHECKED_UMGAH.0 as ::core::ffi::c_int,
                    2 as BYTE,
                );
                break 's_335;
            }
            15 => {
                let mut hRebel: HFLEETINFO = ::core::ptr::null_mut::<
                    ::core::ffi::c_void,
                >();
                let mut hRoyalist: HFLEETINFO = ::core::ptr::null_mut::<
                    ::core::ffi::c_void,
                >();
                let mut RebelPtr: *mut FLEET_INFO = ::core::ptr::null_mut::<
                    FLEET_INFO,
                >();
                let mut RoyalistPtr: *mut FLEET_INFO = ::core::ptr::null_mut::<
                    FLEET_INFO,
                >();
                hRebel = GetStarShipFromIndex(
                    &raw mut GlobData.Game_state.avail_race_q,
                    C2Rust_Unnamed_2::YEHAT_REBEL_SHIP.0 as ::core::ffi::c_int as COUNT,
                ) as HFLEETINFO;
                RebelPtr = LockFleetInfo(
                    &raw mut GlobData.Game_state.avail_race_q,
                    hRebel,
                );
                hRoyalist = GetStarShipFromIndex(
                    &raw mut GlobData.Game_state.avail_race_q,
                    C2Rust_Unnamed_2::YEHAT_SHIP.0 as ::core::ffi::c_int as COUNT,
                ) as HFLEETINFO;
                RoyalistPtr = LockFleetInfo(
                    &raw mut GlobData.Game_state.avail_race_q,
                    hRoyalist,
                );
                (*RoyalistPtr).actual_strength = ((*RoyalistPtr).actual_strength
                    as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    / 3 as ::core::ffi::c_int) as COUNT;
                (*RebelPtr).actual_strength = (*RoyalistPtr).actual_strength;
                (*RebelPtr).loc.x = 5150 as COORD;
                (*RebelPtr).loc.y = 0 as COORD;
                UnlockLink(&raw mut GlobData.Game_state.avail_race_q, hRoyalist);
                UnlockLink(&raw mut GlobData.Game_state.avail_race_q, hRebel);
                StartSphereTracking(
                    C2Rust_Unnamed_2::YEHAT_REBEL_SHIP.0 as ::core::ffi::c_int as COUNT,
                );
                break 's_335;
            }
            16 => {
                if getGameState(
                    &raw mut GlobData.Game_state.GameState as *mut BYTE,
                    C2Rust_Unnamed_4::DESTRUCT_CODE_ON_SHIP.0 as ::core::ffi::c_int,
                    C2Rust_Unnamed_4::END_DESTRUCT_CODE_ON_SHIP.0 as ::core::ffi::c_int,
                ) == 0
                {
                    let mut ramp_factor: BYTE = 0;
                    ramp_factor = getGameState(
                        &raw mut GlobData.Game_state.GameState as *mut BYTE,
                        C2Rust_Unnamed_4::SLYLANDRO_MULTIPLIER.0 as ::core::ffi::c_int,
                        C2Rust_Unnamed_4::END_SLYLANDRO_MULTIPLIER.0
                            as ::core::ffi::c_int,
                    );
                    ramp_factor = ramp_factor.wrapping_add(1);
                    if ramp_factor as ::core::ffi::c_int <= 4 as ::core::ffi::c_int {
                        setGameState(
                            &raw mut GlobData.Game_state.GameState as *mut BYTE,
                            C2Rust_Unnamed_4::SLYLANDRO_MULTIPLIER.0
                                as ::core::ffi::c_int,
                            C2Rust_Unnamed_4::END_SLYLANDRO_MULTIPLIER.0
                                as ::core::ffi::c_int,
                            ramp_factor,
                        );
                        AddEvent(
                            EVENT_TYPE::RELATIVE_EVENT,
                            0 as COUNT,
                            182 as COUNT,
                            0 as COUNT,
                            C2Rust_Unnamed::SLYLANDRO_RAMP_UP.0 as ::core::ffi::c_int
                                as BYTE,
                        );
                    }
                }
                break 's_335;
            }
            17 => {
                let mut ramp_factor_0: BYTE = 0;
                ramp_factor_0 = getGameState(
                    &raw mut GlobData.Game_state.GameState as *mut BYTE,
                    C2Rust_Unnamed_4::SLYLANDRO_MULTIPLIER.0 as ::core::ffi::c_int,
                    C2Rust_Unnamed_4::END_SLYLANDRO_MULTIPLIER.0 as ::core::ffi::c_int,
                );
                ramp_factor_0 = ramp_factor_0.wrapping_sub(1);
                if ramp_factor_0 != 0 {
                    AddEvent(
                        EVENT_TYPE::RELATIVE_EVENT,
                        0 as COUNT,
                        23 as COUNT,
                        0 as COUNT,
                        C2Rust_Unnamed::SLYLANDRO_RAMP_DOWN.0 as ::core::ffi::c_int
                            as BYTE,
                    );
                }
                setGameState(
                    &raw mut GlobData.Game_state.GameState as *mut BYTE,
                    C2Rust_Unnamed_4::SLYLANDRO_MULTIPLIER.0 as ::core::ffi::c_int,
                    C2Rust_Unnamed_4::END_SLYLANDRO_MULTIPLIER.0 as ::core::ffi::c_int,
                    ramp_factor_0,
                );
                break 's_335;
            }
            _ => {
                break 's_335;
            }
        }
        if getGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_4::KOHR_AH_FRENZY.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_4::END_KOHR_AH_FRENZY.0 as ::core::ffi::c_int,
        ) == 0
            && GlobData.Game_state.CurrentActivity as BYTE as ::core::ffi::c_int
                == C2Rust_Unnamed_5::IN_INTERPLANETARY.0 as ::core::ffi::c_int
            && !CurStarDescPtr.is_null()
            && (*CurStarDescPtr).Index as ::core::ffi::c_int
                == C2Rust_Unnamed_3::SAMATRA_DEFINED.0 as ::core::ffi::c_int
        {
            AddEvent(
                EVENT_TYPE::RELATIVE_EVENT,
                0 as COUNT,
                7 as COUNT,
                0 as COUNT,
                C2Rust_Unnamed::KOHR_AH_GENOCIDE_EVENT.0 as ::core::ffi::c_int as BYTE,
            );
        } else {
            black_urquan_genocide();
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn SetRaceDest(
    mut which_race: BYTE,
    mut x: COORD,
    mut y: COORD,
    mut days_left: BYTE,
    mut func_index: BYTE,
) {
    let mut hFleet: HFLEETINFO = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut FleetPtr: *mut FLEET_INFO = ::core::ptr::null_mut::<FLEET_INFO>();
    hFleet = GetStarShipFromIndex(
        &raw mut GlobData.Game_state.avail_race_q,
        which_race as COUNT,
    ) as HFLEETINFO;
    FleetPtr = LockFleetInfo(&raw mut GlobData.Game_state.avail_race_q, hFleet);
    (*FleetPtr).dest_loc.x = x;
    (*FleetPtr).dest_loc.y = y;
    (*FleetPtr).days_left = days_left;
    (*FleetPtr).func_index = func_index;
    UnlockLink(&raw mut GlobData.Game_state.avail_race_q, hFleet);
}
unsafe extern "C" fn arilou_entrance_event() {
    setGameState(
        &raw mut GlobData.Game_state.GameState as *mut BYTE,
        C2Rust_Unnamed_4::ARILOU_SPACE.0 as ::core::ffi::c_int,
        C2Rust_Unnamed_4::END_ARILOU_SPACE.0 as ::core::ffi::c_int,
        C2Rust_Unnamed_0::OPENING.0 as ::core::ffi::c_int as BYTE,
    );
    AddEvent(
        EVENT_TYPE::RELATIVE_EVENT,
        0 as COUNT,
        3 as COUNT,
        0 as COUNT,
        C2Rust_Unnamed::ARILOU_EXIT_EVENT.0 as ::core::ffi::c_int as BYTE,
    );
}
unsafe extern "C" fn arilou_exit_event() {
    let mut month_index: COUNT = 0;
    let mut year_index: COUNT = 0;
    year_index = GlobData.Game_state.GameClock.year_index;
    month_index = (GlobData.Game_state.GameClock.month_index as ::core::ffi::c_int
        % 12 as ::core::ffi::c_int) as COUNT;
    if month_index as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        year_index = year_index.wrapping_add(1);
    }
    month_index = month_index.wrapping_add(1);
    setGameState(
        &raw mut GlobData.Game_state.GameState as *mut BYTE,
        C2Rust_Unnamed_4::ARILOU_SPACE.0 as ::core::ffi::c_int,
        C2Rust_Unnamed_4::END_ARILOU_SPACE.0 as ::core::ffi::c_int,
        C2Rust_Unnamed_0::CLOSING.0 as ::core::ffi::c_int as BYTE,
    );
    AddEvent(
        EVENT_TYPE::ABSOLUTE_EVENT,
        month_index,
        17 as COUNT,
        year_index,
        C2Rust_Unnamed::ARILOU_ENTRANCE_EVENT.0 as ::core::ffi::c_int as BYTE,
    );
}
unsafe extern "C" fn check_race_growth() {
    let mut hStarShip: HFLEETINFO = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut hNextShip: HFLEETINFO = ::core::ptr::null_mut::<::core::ffi::c_void>();
    hStarShip = GlobData.Game_state.avail_race_q.head as HFLEETINFO;
    while !hStarShip.is_null() {
        let mut FleetPtr: *mut FLEET_INFO = ::core::ptr::null_mut::<FLEET_INFO>();
        FleetPtr = LockFleetInfo(&raw mut GlobData.Game_state.avail_race_q, hStarShip);
        hNextShip = (*FleetPtr).succ;
        if (*FleetPtr).actual_strength as ::core::ffi::c_int != 0
            && (*FleetPtr).actual_strength as ::core::ffi::c_int
                != !(0 as ::core::ffi::c_int) as COUNT as ::core::ffi::c_int
        {
            let mut delta_strength: SIZE = 0;
            delta_strength = (*FleetPtr).growth as SBYTE as SIZE;
            if (*FleetPtr).growth_err_term as ::core::ffi::c_int
                <= (*FleetPtr).growth_fract as ::core::ffi::c_int
            {
                if delta_strength as ::core::ffi::c_int <= 0 as ::core::ffi::c_int {
                    delta_strength -= 1;
                } else {
                    delta_strength += 1;
                }
            }
            (*FleetPtr).growth_err_term = ((*FleetPtr).growth_err_term
                as ::core::ffi::c_int - (*FleetPtr).growth_fract as ::core::ffi::c_int)
                as BYTE;
            delta_strength = (delta_strength as ::core::ffi::c_int
                + (*FleetPtr).actual_strength as ::core::ffi::c_int) as SIZE;
            if delta_strength as ::core::ffi::c_int <= 0 as ::core::ffi::c_int {
                delta_strength = 0 as SIZE;
                (*FleetPtr).allied_state = C2Rust_Unnamed_1::DEAD_GUY.0
                    as ::core::ffi::c_int as UWORD;
            } else if delta_strength as ::core::ffi::c_int
                > 254 as ::core::ffi::c_int * 11 as ::core::ffi::c_int
            {
                delta_strength = (254 as ::core::ffi::c_int * 11 as ::core::ffi::c_int)
                    as SIZE;
            }
            (*FleetPtr).actual_strength = delta_strength as COUNT;
            if (*FleetPtr).actual_strength as ::core::ffi::c_int != 0
                && (*FleetPtr).days_left as ::core::ffi::c_int != 0
            {
                (*FleetPtr).loc.x = ((*FleetPtr).loc.x as ::core::ffi::c_int
                    + ((*FleetPtr).dest_loc.x as ::core::ffi::c_int
                        - (*FleetPtr).loc.x as ::core::ffi::c_int)
                        / (*FleetPtr).days_left as ::core::ffi::c_int) as COORD;
                (*FleetPtr).loc.y = ((*FleetPtr).loc.y as ::core::ffi::c_int
                    + ((*FleetPtr).dest_loc.y as ::core::ffi::c_int
                        - (*FleetPtr).loc.y as ::core::ffi::c_int)
                        / (*FleetPtr).days_left as ::core::ffi::c_int) as COORD;
                (*FleetPtr).days_left = (*FleetPtr).days_left.wrapping_sub(1);
                if (*FleetPtr).days_left as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                    && (*FleetPtr).func_index as ::core::ffi::c_int
                        != !(0 as ::core::ffi::c_int) as BYTE as ::core::ffi::c_int
                {
                    EventHandler((*FleetPtr).func_index);
                }
            }
        }
        UnlockLink(&raw mut GlobData.Game_state.avail_race_q, hStarShip);
        hStarShip = hNextShip;
    }
}
unsafe extern "C" fn black_urquan_genocide() {
    let mut Index: BYTE = 0;
    let mut best_dist: ::core::ffi::c_long = 0;
    let mut best_dx: SIZE = 0;
    let mut best_dy: SIZE = 0;
    let mut hStarShip: HFLEETINFO = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut hNextShip: HFLEETINFO = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut hBlackUrquan: HFLEETINFO = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut BlackUrquanPtr: *mut FLEET_INFO = ::core::ptr::null_mut::<FLEET_INFO>();
    hBlackUrquan = GetStarShipFromIndex(
        &raw mut GlobData.Game_state.avail_race_q,
        C2Rust_Unnamed_2::BLACK_URQUAN_SHIP.0 as ::core::ffi::c_int as COUNT,
    ) as HFLEETINFO;
    BlackUrquanPtr = LockFleetInfo(
        &raw mut GlobData.Game_state.avail_race_q,
        hBlackUrquan,
    );
    best_dist = -1 as ::core::ffi::c_long;
    best_dx = (1752 as ::core::ffi::c_int
        - (*BlackUrquanPtr).loc.x as ::core::ffi::c_int) as SIZE;
    best_dy = (1450 as ::core::ffi::c_int
        - (*BlackUrquanPtr).loc.y as ::core::ffi::c_int) as SIZE;
    Index = 0 as BYTE;
    hStarShip = GlobData.Game_state.avail_race_q.head as HFLEETINFO;
    while !hStarShip.is_null() {
        let mut FleetPtr: *mut FLEET_INFO = ::core::ptr::null_mut::<FLEET_INFO>();
        FleetPtr = LockFleetInfo(&raw mut GlobData.Game_state.avail_race_q, hStarShip);
        hNextShip = (*FleetPtr).succ;
        if Index as ::core::ffi::c_int
            != C2Rust_Unnamed_2::BLACK_URQUAN_SHIP.0 as ::core::ffi::c_int
            && Index as ::core::ffi::c_int
                != C2Rust_Unnamed_2::URQUAN_SHIP.0 as ::core::ffi::c_int
            && (*FleetPtr).actual_strength as ::core::ffi::c_int
                != !(0 as ::core::ffi::c_int) as COUNT as ::core::ffi::c_int
        {
            let mut dx: SIZE = 0;
            let mut dy: SIZE = 0;
            dx = ((*FleetPtr).loc.x as ::core::ffi::c_int
                - (*BlackUrquanPtr).loc.x as ::core::ffi::c_int) as SIZE;
            dy = ((*FleetPtr).loc.y as ::core::ffi::c_int
                - (*BlackUrquanPtr).loc.y as ::core::ffi::c_int) as SIZE;
            if dx as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                && dy as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            {
                (*FleetPtr).allied_state = C2Rust_Unnamed_1::DEAD_GUY.0
                    as ::core::ffi::c_int as UWORD;
                (*FleetPtr).actual_strength = 0 as COUNT;
            } else if (*FleetPtr).actual_strength != 0 {
                let mut dist: ::core::ffi::c_long = 0;
                dist = dx as ::core::ffi::c_long * dx as ::core::ffi::c_long
                    + dy as ::core::ffi::c_long * dy as ::core::ffi::c_long;
                if best_dist < 0 as ::core::ffi::c_long || dist < best_dist
                    || Index as ::core::ffi::c_int
                        == C2Rust_Unnamed_2::DRUUGE_SHIP.0 as ::core::ffi::c_int
                {
                    best_dist = dist;
                    best_dx = dx;
                    best_dy = dy;
                    if Index as ::core::ffi::c_int
                        == C2Rust_Unnamed_2::DRUUGE_SHIP.0 as ::core::ffi::c_int
                    {
                        hNextShip = ::core::ptr::null_mut::<::core::ffi::c_void>();
                    }
                }
            }
        }
        UnlockLink(&raw mut GlobData.Game_state.avail_race_q, hStarShip);
        Index = Index.wrapping_add(1);
        hStarShip = hNextShip;
    }
    if best_dist < 0 as ::core::ffi::c_long
        && best_dx as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        && best_dy as ::core::ffi::c_int == 0 as ::core::ffi::c_int
    {
        GlobData.Game_state.CurrentActivity = (GlobData.Game_state.CurrentActivity
            as ::core::ffi::c_int
            & !(C2Rust_Unnamed_5::IN_BATTLE.0 as ::core::ffi::c_int)) as ACTIVITY;
        GlobData.SIS_state.CrewEnlisted = !(0 as ::core::ffi::c_int) as COUNT;
        setGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_4::KOHR_AH_KILLED_ALL.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_4::END_KOHR_AH_KILLED_ALL.0 as ::core::ffi::c_int,
            1 as BYTE,
        );
    } else {
        let mut speed: COUNT = 0;
        if best_dist < 0 as ::core::ffi::c_long {
            best_dist = best_dx as ::core::ffi::c_long * best_dx as ::core::ffi::c_long
                + best_dy as ::core::ffi::c_long * best_dy as ::core::ffi::c_long;
        }
        speed = (square_root(best_dist as DWORD) as ::core::ffi::c_int
            / 158 as ::core::ffi::c_int) as COUNT;
        if speed as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            speed = 1 as COUNT;
        } else if speed as ::core::ffi::c_int > 255 as ::core::ffi::c_int {
            speed = 255 as COUNT;
        }
        setGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_4::KOHR_AH_FRENZY.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_4::END_KOHR_AH_FRENZY.0 as ::core::ffi::c_int,
            1 as BYTE,
        );
        setGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_4::KOHR_AH_VISITS.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_4::END_KOHR_AH_VISITS.0 as ::core::ffi::c_int,
            0 as BYTE,
        );
        setGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_4::KOHR_AH_REASONS.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_4::END_KOHR_AH_REASONS.0 as ::core::ffi::c_int,
            0 as BYTE,
        );
        setGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_4::KOHR_AH_PLEAD.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_4::END_KOHR_AH_PLEAD.0 as ::core::ffi::c_int,
            0 as BYTE,
        );
        setGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_4::KOHR_AH_INFO.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_4::END_KOHR_AH_INFO.0 as ::core::ffi::c_int,
            0 as BYTE,
        );
        setGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_4::URQUAN_VISITS.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_4::END_URQUAN_VISITS.0 as ::core::ffi::c_int,
            0 as BYTE,
        );
        SetRaceDest(
            C2Rust_Unnamed_2::BLACK_URQUAN_SHIP.0 as ::core::ffi::c_int as BYTE,
            ((*BlackUrquanPtr).loc.x as ::core::ffi::c_int
                + best_dx as ::core::ffi::c_int) as COORD,
            ((*BlackUrquanPtr).loc.y as ::core::ffi::c_int
                + best_dy as ::core::ffi::c_int) as COORD,
            speed as BYTE,
            C2Rust_Unnamed::KOHR_AH_GENOCIDE_EVENT.0 as ::core::ffi::c_int as BYTE,
        );
    }
    UnlockLink(&raw mut GlobData.Game_state.avail_race_q, hBlackUrquan);
}
unsafe extern "C" fn pkunk_mission() {
    let mut hPkunk: HFLEETINFO = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut PkunkPtr: *mut FLEET_INFO = ::core::ptr::null_mut::<FLEET_INFO>();
    hPkunk = GetStarShipFromIndex(
        &raw mut GlobData.Game_state.avail_race_q,
        C2Rust_Unnamed_2::PKUNK_SHIP.0 as ::core::ffi::c_int as COUNT,
    ) as HFLEETINFO;
    PkunkPtr = LockFleetInfo(&raw mut GlobData.Game_state.avail_race_q, hPkunk);
    if (*PkunkPtr).actual_strength != 0 {
        let mut MissionState: BYTE = 0;
        MissionState = getGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_4::PKUNK_MISSION.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_4::END_PKUNK_MISSION.0 as ::core::ffi::c_int,
        );
        if (*PkunkPtr).days_left as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            && MissionState as ::core::ffi::c_int != 0
        {
            if MissionState as ::core::ffi::c_int & 1 as ::core::ffi::c_int != 0
                || (*PkunkPtr).loc.x as ::core::ffi::c_int == 4970 as ::core::ffi::c_int
                    && (*PkunkPtr).loc.y as ::core::ffi::c_int
                        == 400 as ::core::ffi::c_int
            {
                (*PkunkPtr).actual_strength = 0 as COUNT;
            } else if (*PkunkPtr).loc.x as ::core::ffi::c_int
                == 502 as ::core::ffi::c_int
                && (*PkunkPtr).loc.y as ::core::ffi::c_int == 401 as ::core::ffi::c_int
                && getGameState(
                    &raw mut GlobData.Game_state.GameState as *mut BYTE,
                    C2Rust_Unnamed_4::PKUNK_ON_THE_MOVE.0 as ::core::ffi::c_int,
                    C2Rust_Unnamed_4::END_PKUNK_ON_THE_MOVE.0 as ::core::ffi::c_int,
                ) as ::core::ffi::c_int != 0
            {
                setGameState(
                    &raw mut GlobData.Game_state.GameState as *mut BYTE,
                    C2Rust_Unnamed_4::PKUNK_ON_THE_MOVE.0 as ::core::ffi::c_int,
                    C2Rust_Unnamed_4::END_PKUNK_ON_THE_MOVE.0 as ::core::ffi::c_int,
                    0 as BYTE,
                );
                AddEvent(
                    EVENT_TYPE::RELATIVE_EVENT,
                    3 as COUNT,
                    0 as COUNT,
                    0 as COUNT,
                    C2Rust_Unnamed::ADVANCE_PKUNK_MISSION.0 as ::core::ffi::c_int as BYTE,
                );
                UnlockLink(&raw mut GlobData.Game_state.avail_race_q, hPkunk);
                return;
            }
        }
        if (*PkunkPtr).actual_strength as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            setGameState(
                &raw mut GlobData.Game_state.GameState as *mut BYTE,
                C2Rust_Unnamed_4::YEHAT_ABSORBED_PKUNK.0 as ::core::ffi::c_int,
                C2Rust_Unnamed_4::END_YEHAT_ABSORBED_PKUNK.0 as ::core::ffi::c_int,
                1 as BYTE,
            );
            (*PkunkPtr).allied_state = C2Rust_Unnamed_1::DEAD_GUY.0 as ::core::ffi::c_int
                as UWORD;
            StartSphereTracking(
                C2Rust_Unnamed_2::YEHAT_SHIP.0 as ::core::ffi::c_int as COUNT,
            );
        } else {
            let mut x: COORD = 0;
            let mut y: COORD = 0;
            if MissionState as ::core::ffi::c_int & 1 as ::core::ffi::c_int == 0 {
                x = 4970 as COORD;
                y = 400 as COORD;
            } else {
                x = 502 as COORD;
                y = 401 as COORD;
            }
            setGameState(
                &raw mut GlobData.Game_state.GameState as *mut BYTE,
                C2Rust_Unnamed_4::PKUNK_ON_THE_MOVE.0 as ::core::ffi::c_int,
                C2Rust_Unnamed_4::END_PKUNK_ON_THE_MOVE.0 as ::core::ffi::c_int,
                1 as BYTE,
            );
            setGameState(
                &raw mut GlobData.Game_state.GameState as *mut BYTE,
                C2Rust_Unnamed_4::PKUNK_SWITCH.0 as ::core::ffi::c_int,
                C2Rust_Unnamed_4::END_PKUNK_SWITCH.0 as ::core::ffi::c_int,
                0 as BYTE,
            );
            SetRaceDest(
                C2Rust_Unnamed_2::PKUNK_SHIP.0 as ::core::ffi::c_int as BYTE,
                x,
                y,
                ((365 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
                    - (*PkunkPtr).days_left as ::core::ffi::c_int) as BYTE,
                C2Rust_Unnamed::ADVANCE_PKUNK_MISSION.0 as ::core::ffi::c_int as BYTE,
            );
        }
        setGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_4::PKUNK_MISSION.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_4::END_PKUNK_MISSION.0 as ::core::ffi::c_int,
            (MissionState as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as BYTE,
        );
    }
    UnlockLink(&raw mut GlobData.Game_state.avail_race_q, hPkunk);
}
unsafe extern "C" fn thradd_mission() {
    let mut MissionState: BYTE = 0;
    let mut hThradd: HFLEETINFO = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut ThraddPtr: *mut FLEET_INFO = ::core::ptr::null_mut::<FLEET_INFO>();
    hThradd = GetStarShipFromIndex(
        &raw mut GlobData.Game_state.avail_race_q,
        C2Rust_Unnamed_2::THRADDASH_SHIP.0 as ::core::ffi::c_int as COUNT,
    ) as HFLEETINFO;
    ThraddPtr = LockFleetInfo(&raw mut GlobData.Game_state.avail_race_q, hThradd);
    MissionState = getGameState(
        &raw mut GlobData.Game_state.GameState as *mut BYTE,
        C2Rust_Unnamed_4::THRADD_MISSION.0 as ::core::ffi::c_int,
        C2Rust_Unnamed_4::END_THRADD_MISSION.0 as ::core::ffi::c_int,
    );
    if (*ThraddPtr).actual_strength as ::core::ffi::c_int != 0
        && (MissionState as ::core::ffi::c_int) < 3 as ::core::ffi::c_int
    {
        let mut x: COORD = 0;
        let mut y: COORD = 0;
        if (MissionState as ::core::ffi::c_int) < 2 as ::core::ffi::c_int {
            x = 4879 as COORD;
            y = 7201 as COORD;
        } else {
            x = 2535 as COORD;
            y = 8358 as COORD;
        }
        if MissionState as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            let mut strength_loss: SIZE = 0;
            strength_loss = ((*ThraddPtr).actual_strength as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int) as SIZE;
            (*ThraddPtr).growth = (-(strength_loss as ::core::ffi::c_int)
                / 14 as ::core::ffi::c_int) as BYTE;
            (*ThraddPtr).growth_fract = (((strength_loss as ::core::ffi::c_int
                % 14 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
                / 14 as ::core::ffi::c_int) as BYTE;
            (*ThraddPtr).growth_err_term = (255 as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int) as BYTE;
        } else if MissionState as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            (*ThraddPtr).growth = 0 as BYTE;
            (*ThraddPtr).growth_fract = 0 as BYTE;
        }
        SetRaceDest(
            C2Rust_Unnamed_2::THRADDASH_SHIP.0 as ::core::ffi::c_int as BYTE,
            x,
            y,
            14 as BYTE,
            C2Rust_Unnamed::ADVANCE_THRADD_MISSION.0 as ::core::ffi::c_int as BYTE,
        );
    }
    MissionState = MissionState.wrapping_add(1);
    setGameState(
        &raw mut GlobData.Game_state.GameState as *mut BYTE,
        C2Rust_Unnamed_4::THRADD_MISSION.0 as ::core::ffi::c_int,
        C2Rust_Unnamed_4::END_THRADD_MISSION.0 as ::core::ffi::c_int,
        MissionState,
    );
    if MissionState as ::core::ffi::c_int == 4 as ::core::ffi::c_int
        && getGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_4::ILWRATH_FIGHT_THRADDASH.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_4::END_ILWRATH_FIGHT_THRADDASH.0 as ::core::ffi::c_int,
        ) as ::core::ffi::c_int != 0
    {
        AddEvent(
            EVENT_TYPE::RELATIVE_EVENT,
            0 as COUNT,
            0 as COUNT,
            0 as COUNT,
            C2Rust_Unnamed::ADVANCE_ILWRATH_MISSION.0 as ::core::ffi::c_int as BYTE,
        );
    }
    UnlockLink(&raw mut GlobData.Game_state.avail_race_q, hThradd);
}
unsafe extern "C" fn ilwrath_mission() {
    let mut ThraddState: BYTE = 0;
    let mut hIlwrath: HFLEETINFO = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut hThradd: HFLEETINFO = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut IlwrathPtr: *mut FLEET_INFO = ::core::ptr::null_mut::<FLEET_INFO>();
    let mut ThraddPtr: *mut FLEET_INFO = ::core::ptr::null_mut::<FLEET_INFO>();
    hIlwrath = GetStarShipFromIndex(
        &raw mut GlobData.Game_state.avail_race_q,
        C2Rust_Unnamed_2::ILWRATH_SHIP.0 as ::core::ffi::c_int as COUNT,
    ) as HFLEETINFO;
    IlwrathPtr = LockFleetInfo(&raw mut GlobData.Game_state.avail_race_q, hIlwrath);
    hThradd = GetStarShipFromIndex(
        &raw mut GlobData.Game_state.avail_race_q,
        C2Rust_Unnamed_2::THRADDASH_SHIP.0 as ::core::ffi::c_int as COUNT,
    ) as HFLEETINFO;
    ThraddPtr = LockFleetInfo(&raw mut GlobData.Game_state.avail_race_q, hThradd);
    if (*IlwrathPtr).loc.x as ::core::ffi::c_int
        == 2500 as ::core::ffi::c_int + 2535 as ::core::ffi::c_int
            >> 1 as ::core::ffi::c_int
        && (*IlwrathPtr).loc.y as ::core::ffi::c_int
            == 8070 as ::core::ffi::c_int + 8358 as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int
    {
        (*IlwrathPtr).actual_strength = 0 as COUNT;
        (*ThraddPtr).actual_strength = 0 as COUNT;
        (*IlwrathPtr).allied_state = C2Rust_Unnamed_1::DEAD_GUY.0 as ::core::ffi::c_int
            as UWORD;
        (*ThraddPtr).allied_state = C2Rust_Unnamed_1::DEAD_GUY.0 as ::core::ffi::c_int
            as UWORD;
    } else if (*IlwrathPtr).actual_strength != 0 {
        if getGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_4::ILWRATH_FIGHT_THRADDASH.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_4::END_ILWRATH_FIGHT_THRADDASH.0 as ::core::ffi::c_int,
        ) == 0
            && ((*IlwrathPtr).dest_loc.x as ::core::ffi::c_int
                != 2500 as ::core::ffi::c_int
                || (*IlwrathPtr).dest_loc.y as ::core::ffi::c_int
                    != 8070 as ::core::ffi::c_int)
        {
            SetRaceDest(
                C2Rust_Unnamed_2::ILWRATH_SHIP.0 as ::core::ffi::c_int as BYTE,
                2500 as COORD,
                8070 as COORD,
                90 as BYTE,
                C2Rust_Unnamed::ADVANCE_ILWRATH_MISSION.0 as ::core::ffi::c_int as BYTE,
            );
        } else {
            let mut strength_loss: SIZE = 0;
            if (*IlwrathPtr).days_left as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                setGameState(
                    &raw mut GlobData.Game_state.GameState as *mut BYTE,
                    C2Rust_Unnamed_4::ILWRATH_FIGHT_THRADDASH.0 as ::core::ffi::c_int,
                    C2Rust_Unnamed_4::END_ILWRATH_FIGHT_THRADDASH.0
                        as ::core::ffi::c_int,
                    1 as BYTE,
                );
                setGameState(
                    &raw mut GlobData.Game_state.GameState as *mut BYTE,
                    C2Rust_Unnamed_4::HELIX_UNPROTECTED.0 as ::core::ffi::c_int,
                    C2Rust_Unnamed_4::END_HELIX_UNPROTECTED.0 as ::core::ffi::c_int,
                    1 as BYTE,
                );
                strength_loss = (*IlwrathPtr).actual_strength as SIZE;
                (*IlwrathPtr).growth = (-(strength_loss as ::core::ffi::c_int)
                    / 128 as ::core::ffi::c_int) as BYTE;
                (*IlwrathPtr).growth_fract = (((strength_loss as ::core::ffi::c_int
                    % 128 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
                    / 128 as ::core::ffi::c_int) as BYTE;
                SetRaceDest(
                    C2Rust_Unnamed_2::ILWRATH_SHIP.0 as ::core::ffi::c_int as BYTE,
                    (2500 as ::core::ffi::c_int + 2535 as ::core::ffi::c_int
                        >> 1 as ::core::ffi::c_int) as COORD,
                    (8070 as ::core::ffi::c_int + 8358 as ::core::ffi::c_int
                        >> 1 as ::core::ffi::c_int) as COORD,
                    (128 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as BYTE,
                    C2Rust_Unnamed::ADVANCE_ILWRATH_MISSION.0 as ::core::ffi::c_int
                        as BYTE,
                );
                strength_loss = (*ThraddPtr).actual_strength as SIZE;
                (*ThraddPtr).growth = (-(strength_loss as ::core::ffi::c_int)
                    / 128 as ::core::ffi::c_int) as BYTE;
                (*ThraddPtr).growth_fract = (((strength_loss as ::core::ffi::c_int
                    % 128 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
                    / 128 as ::core::ffi::c_int) as BYTE;
                setGameState(
                    &raw mut GlobData.Game_state.GameState as *mut BYTE,
                    C2Rust_Unnamed_4::THRADD_VISITS.0 as ::core::ffi::c_int,
                    C2Rust_Unnamed_4::END_THRADD_VISITS.0 as ::core::ffi::c_int,
                    0 as BYTE,
                );
                if (*ThraddPtr).allied_state as ::core::ffi::c_int
                    == C2Rust_Unnamed_1::GOOD_GUY.0 as ::core::ffi::c_int
                {
                    SetRaceAllied(
                        C2Rust_Unnamed_2::THRADDASH_SHIP.0 as ::core::ffi::c_int
                            as COUNT,
                        BOOLEAN::FALSE,
                    );
                }
            }
            ThraddState = getGameState(
                &raw mut GlobData.Game_state.GameState as *mut BYTE,
                C2Rust_Unnamed_4::THRADD_MISSION.0 as ::core::ffi::c_int,
                C2Rust_Unnamed_4::END_THRADD_MISSION.0 as ::core::ffi::c_int,
            );
            if ThraddState as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                || ThraddState as ::core::ffi::c_int > 3 as ::core::ffi::c_int
            {
                SetRaceDest(
                    C2Rust_Unnamed_2::THRADDASH_SHIP.0 as ::core::ffi::c_int as BYTE,
                    (2500 as ::core::ffi::c_int + 2535 as ::core::ffi::c_int
                        >> 1 as ::core::ffi::c_int) as COORD,
                    (8070 as ::core::ffi::c_int + 8358 as ::core::ffi::c_int
                        >> 1 as ::core::ffi::c_int) as COORD,
                    ((*IlwrathPtr).days_left as ::core::ffi::c_int
                        + 1 as ::core::ffi::c_int) as BYTE,
                    !(0 as ::core::ffi::c_int) as BYTE,
                );
            } else if (ThraddState as ::core::ffi::c_int) < 3 as ::core::ffi::c_int {
                SetRaceDest(
                    C2Rust_Unnamed_2::THRADDASH_SHIP.0 as ::core::ffi::c_int as BYTE,
                    2535 as COORD,
                    8358 as COORD,
                    10 as BYTE,
                    C2Rust_Unnamed::ADVANCE_THRADD_MISSION.0 as ::core::ffi::c_int
                        as BYTE,
                );
                setGameState(
                    &raw mut GlobData.Game_state.GameState as *mut BYTE,
                    C2Rust_Unnamed_4::THRADD_MISSION.0 as ::core::ffi::c_int,
                    C2Rust_Unnamed_4::END_THRADD_MISSION.0 as ::core::ffi::c_int,
                    3 as BYTE,
                );
            }
        }
    }
    UnlockLink(&raw mut GlobData.Game_state.avail_race_q, hThradd);
    UnlockLink(&raw mut GlobData.Game_state.avail_race_q, hIlwrath);
}
unsafe extern "C" fn utwig_supox_mission() {
    let mut MissionState: BYTE = 0;
    let mut hUtwig: HFLEETINFO = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut hSupox: HFLEETINFO = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut UtwigPtr: *mut FLEET_INFO = ::core::ptr::null_mut::<FLEET_INFO>();
    let mut SupoxPtr: *mut FLEET_INFO = ::core::ptr::null_mut::<FLEET_INFO>();
    hUtwig = GetStarShipFromIndex(
        &raw mut GlobData.Game_state.avail_race_q,
        C2Rust_Unnamed_2::UTWIG_SHIP.0 as ::core::ffi::c_int as COUNT,
    ) as HFLEETINFO;
    UtwigPtr = LockFleetInfo(&raw mut GlobData.Game_state.avail_race_q, hUtwig);
    hSupox = GetStarShipFromIndex(
        &raw mut GlobData.Game_state.avail_race_q,
        C2Rust_Unnamed_2::SUPOX_SHIP.0 as ::core::ffi::c_int as COUNT,
    ) as HFLEETINFO;
    SupoxPtr = LockFleetInfo(&raw mut GlobData.Game_state.avail_race_q, hSupox);
    MissionState = getGameState(
        &raw mut GlobData.Game_state.GameState as *mut BYTE,
        C2Rust_Unnamed_4::UTWIG_SUPOX_MISSION.0 as ::core::ffi::c_int,
        C2Rust_Unnamed_4::END_UTWIG_SUPOX_MISSION.0 as ::core::ffi::c_int,
    );
    if (*UtwigPtr).actual_strength as ::core::ffi::c_int != 0
        && (MissionState as ::core::ffi::c_int) < 5 as ::core::ffi::c_int
    {
        if MissionState as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            let mut strength_loss: SIZE = 0;
            AddEvent(
                EVENT_TYPE::RELATIVE_EVENT,
                0 as COUNT,
                (160 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as COUNT,
                0 as COUNT,
                C2Rust_Unnamed::ADVANCE_UTWIG_SUPOX_MISSION.0 as ::core::ffi::c_int
                    as BYTE,
            );
            strength_loss = ((*UtwigPtr).actual_strength as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int) as SIZE;
            (*UtwigPtr).growth = (-(strength_loss as ::core::ffi::c_int)
                / 160 as ::core::ffi::c_int) as BYTE;
            (*UtwigPtr).growth_fract = (((strength_loss as ::core::ffi::c_int
                % 160 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
                / 160 as ::core::ffi::c_int) as BYTE;
            (*UtwigPtr).growth_err_term = (255 as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int) as BYTE;
            strength_loss = ((*SupoxPtr).actual_strength as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int) as SIZE;
            if strength_loss != 0 {
                (*SupoxPtr).growth = (-(strength_loss as ::core::ffi::c_int)
                    / 160 as ::core::ffi::c_int) as BYTE;
                (*SupoxPtr).growth_fract = (((strength_loss as ::core::ffi::c_int
                    % 160 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
                    / 160 as ::core::ffi::c_int) as BYTE;
                (*SupoxPtr).growth_err_term = (255 as ::core::ffi::c_int
                    >> 1 as ::core::ffi::c_int) as BYTE;
            }
            setGameState(
                &raw mut GlobData.Game_state.GameState as *mut BYTE,
                C2Rust_Unnamed_4::UTWIG_WAR_NEWS.0 as ::core::ffi::c_int,
                C2Rust_Unnamed_4::END_UTWIG_WAR_NEWS.0 as ::core::ffi::c_int,
                0 as BYTE,
            );
            setGameState(
                &raw mut GlobData.Game_state.GameState as *mut BYTE,
                C2Rust_Unnamed_4::SUPOX_WAR_NEWS.0 as ::core::ffi::c_int,
                C2Rust_Unnamed_4::END_SUPOX_WAR_NEWS.0 as ::core::ffi::c_int,
                0 as BYTE,
            );
        } else if MissionState as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            AddEvent(
                EVENT_TYPE::RELATIVE_EVENT,
                0 as COUNT,
                (160 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as COUNT,
                0 as COUNT,
                C2Rust_Unnamed::ADVANCE_UTWIG_SUPOX_MISSION.0 as ::core::ffi::c_int
                    as BYTE,
            );
            MissionState = MissionState.wrapping_add(1);
        } else {
            let mut ux: COORD = 0;
            let mut uy: COORD = 0;
            let mut sx: COORD = 0;
            let mut sy: COORD = 0;
            if MissionState as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                ux = 7208 as COORD;
                uy = 7000 as COORD;
                sx = 6479 as COORD;
                sy = 7541 as COORD;
            } else {
                ux = 8534 as COORD;
                uy = 8797 as COORD;
                sx = 7468 as COORD;
                sy = 9246 as COORD;
                (*UtwigPtr).growth = 0 as BYTE;
                (*UtwigPtr).growth_fract = 0 as BYTE;
                (*SupoxPtr).growth = 0 as BYTE;
                (*SupoxPtr).growth_fract = 0 as BYTE;
                setGameState(
                    &raw mut GlobData.Game_state.GameState as *mut BYTE,
                    C2Rust_Unnamed_4::UTWIG_WAR_NEWS.0 as ::core::ffi::c_int,
                    C2Rust_Unnamed_4::END_UTWIG_WAR_NEWS.0 as ::core::ffi::c_int,
                    0 as BYTE,
                );
                setGameState(
                    &raw mut GlobData.Game_state.GameState as *mut BYTE,
                    C2Rust_Unnamed_4::SUPOX_WAR_NEWS.0 as ::core::ffi::c_int,
                    C2Rust_Unnamed_4::END_SUPOX_WAR_NEWS.0 as ::core::ffi::c_int,
                    0 as BYTE,
                );
            }
            setGameState(
                &raw mut GlobData.Game_state.GameState as *mut BYTE,
                C2Rust_Unnamed_4::UTWIG_VISITS.0 as ::core::ffi::c_int,
                C2Rust_Unnamed_4::END_UTWIG_VISITS.0 as ::core::ffi::c_int,
                0 as BYTE,
            );
            setGameState(
                &raw mut GlobData.Game_state.GameState as *mut BYTE,
                C2Rust_Unnamed_4::UTWIG_INFO.0 as ::core::ffi::c_int,
                C2Rust_Unnamed_4::END_UTWIG_INFO.0 as ::core::ffi::c_int,
                0 as BYTE,
            );
            setGameState(
                &raw mut GlobData.Game_state.GameState as *mut BYTE,
                C2Rust_Unnamed_4::SUPOX_VISITS.0 as ::core::ffi::c_int,
                C2Rust_Unnamed_4::END_SUPOX_VISITS.0 as ::core::ffi::c_int,
                0 as BYTE,
            );
            setGameState(
                &raw mut GlobData.Game_state.GameState as *mut BYTE,
                C2Rust_Unnamed_4::SUPOX_INFO.0 as ::core::ffi::c_int,
                C2Rust_Unnamed_4::END_SUPOX_INFO.0 as ::core::ffi::c_int,
                0 as BYTE,
            );
            SetRaceDest(
                C2Rust_Unnamed_2::UTWIG_SHIP.0 as ::core::ffi::c_int as BYTE,
                ux,
                uy,
                21 as BYTE,
                C2Rust_Unnamed::ADVANCE_UTWIG_SUPOX_MISSION.0 as ::core::ffi::c_int
                    as BYTE,
            );
            SetRaceDest(
                C2Rust_Unnamed_2::SUPOX_SHIP.0 as ::core::ffi::c_int as BYTE,
                sx,
                sy,
                21 as BYTE,
                !(0 as ::core::ffi::c_int) as BYTE,
            );
        }
    }
    setGameState(
        &raw mut GlobData.Game_state.GameState as *mut BYTE,
        C2Rust_Unnamed_4::UTWIG_SUPOX_MISSION.0 as ::core::ffi::c_int,
        C2Rust_Unnamed_4::END_UTWIG_SUPOX_MISSION.0 as ::core::ffi::c_int,
        (MissionState as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as BYTE,
    );
    UnlockLink(&raw mut GlobData.Game_state.avail_race_q, hSupox);
    UnlockLink(&raw mut GlobData.Game_state.avail_race_q, hUtwig);
}
unsafe extern "C" fn mycon_mission() {
    let mut hMycon: HFLEETINFO = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut MyconPtr: *mut FLEET_INFO = ::core::ptr::null_mut::<FLEET_INFO>();
    hMycon = GetStarShipFromIndex(
        &raw mut GlobData.Game_state.avail_race_q,
        C2Rust_Unnamed_2::MYCON_SHIP.0 as ::core::ffi::c_int as COUNT,
    ) as HFLEETINFO;
    MyconPtr = LockFleetInfo(&raw mut GlobData.Game_state.avail_race_q, hMycon);
    if (*MyconPtr).actual_strength != 0 {
        if (*MyconPtr).growth != 0 {
            setGameState(
                &raw mut GlobData.Game_state.GameState as *mut BYTE,
                C2Rust_Unnamed_4::MYCON_KNOW_AMBUSH.0 as ::core::ffi::c_int,
                C2Rust_Unnamed_4::END_MYCON_KNOW_AMBUSH.0 as ::core::ffi::c_int,
                1 as BYTE,
            );
            SetRaceDest(
                C2Rust_Unnamed_2::MYCON_SHIP.0 as ::core::ffi::c_int as BYTE,
                6392 as COORD,
                2200 as COORD,
                30 as BYTE,
                !(0 as ::core::ffi::c_int) as BYTE,
            );
            (*MyconPtr).growth = 0 as BYTE;
            (*MyconPtr).growth_fract = 0 as BYTE;
        } else if (*MyconPtr).loc.x as ::core::ffi::c_int != 6858 as ::core::ffi::c_int
            || (*MyconPtr).loc.y as ::core::ffi::c_int != 577 as ::core::ffi::c_int
        {
            SetRaceDest(
                C2Rust_Unnamed_2::MYCON_SHIP.0 as ::core::ffi::c_int as BYTE,
                6858 as COORD,
                577 as COORD,
                30 as BYTE,
                C2Rust_Unnamed::ADVANCE_MYCON_MISSION.0 as ::core::ffi::c_int as BYTE,
            );
        } else {
            let mut strength_loss: SIZE = 0;
            AddEvent(
                EVENT_TYPE::RELATIVE_EVENT,
                0 as COUNT,
                14 as COUNT,
                0 as COUNT,
                C2Rust_Unnamed::ADVANCE_MYCON_MISSION.0 as ::core::ffi::c_int as BYTE,
            );
            strength_loss = ((*MyconPtr).actual_strength as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int) as SIZE;
            (*MyconPtr).growth = (-(strength_loss as ::core::ffi::c_int)
                / 14 as ::core::ffi::c_int) as BYTE;
            (*MyconPtr).growth_fract = (((strength_loss as ::core::ffi::c_int
                % 14 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
                / 14 as ::core::ffi::c_int) as BYTE;
            (*MyconPtr).growth_err_term = (255 as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int) as BYTE;
        }
    }
    UnlockLink(&raw mut GlobData.Game_state.avail_race_q, hMycon);
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
