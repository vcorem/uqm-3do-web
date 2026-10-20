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
    ) -> !;
    static mut optCheatMode: OPT_ENABLABLE;
    static mut optDeCleansing: OPT_ENABLABLE;
    fn AddEvent(
        r#type: EVENT_TYPE,
        month_index: COUNT,
        day_index: COUNT,
        year_index: COUNT,
        func_index: BYTE,
    ) -> HEVENT;
    fn setGameStateUint(name: *const ::core::ffi::c_char, val: DWORD);
    fn getGameStateUint(name: *const ::core::ffi::c_char) -> DWORD;
    static mut GlobData: GLOBDATA;
    fn inHyperSpace() -> BOOLEAN;
    fn square_root(value: DWORD) -> COUNT;
    static mut CurStarDescPtr: *mut STAR_DESC;
    static mut plot_map: [PLOT_LOCATION; 70];
    fn GetStarShipFromIndex(pShipQ: *mut QUEUE, Index: COUNT) -> HLINK;
    fn SetRaceAllied(race: RACE_ID, flag: BOOLEAN) -> BOOLEAN;
    fn StartSphereTracking(race: RACE_ID) -> COUNT;
    fn RemoveEscortShips(race: RACE_ID) -> COUNT;
    fn SeedFleetLocation(
        FleetPtr: *mut FLEET_INFO,
        plotmap: *mut PLOT_LOCATION,
        visit: COUNT,
    ) -> POINT;
    fn check_hyperspace_encounter();
    fn log_add(_: log_Level, fmt: *const ::core::ffi::c_char, ...);
    fn luaUqm_event_init(
        customFuncs: *const luaUqm_custom_Function,
        scriptRes: RESOURCE,
    ) -> BOOLEAN;
    fn luaUqm_event_uninit();
    fn luaUqm_event_callEvent(eventIdStr: *const ::core::ffi::c_char);
    fn strcmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct luaUqm_custom_Function {
    pub name: *const ::core::ffi::c_char,
    pub fun: Option<unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int>,
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
pub type HFLEETINFO = HLINK;
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
static mut eventNames: [*const ::core::ffi::c_char; 18] = [
    b"ARILOU_ENTRANCE_EVENT\0".as_ptr() as *const ::core::ffi::c_char,
    b"ARILOU_EXIT_EVENT\0".as_ptr() as *const ::core::ffi::c_char,
    b"HYPERSPACE_ENCOUNTER_EVENT\0".as_ptr() as *const ::core::ffi::c_char,
    b"KOHR_AH_VICTORIOUS_EVENT\0".as_ptr() as *const ::core::ffi::c_char,
    b"ADVANCE_PKUNK_MISSION\0".as_ptr() as *const ::core::ffi::c_char,
    b"ADVANCE_THRADD_MISSION\0".as_ptr() as *const ::core::ffi::c_char,
    b"ZOQFOT_DISTRESS_EVENT\0".as_ptr() as *const ::core::ffi::c_char,
    b"ZOQFOT_DEATH_EVENT\0".as_ptr() as *const ::core::ffi::c_char,
    b"SHOFIXTI_RETURN_EVENT\0".as_ptr() as *const ::core::ffi::c_char,
    b"ADVANCE_UTWIG_SUPOX_MISSION\0".as_ptr() as *const ::core::ffi::c_char,
    b"KOHR_AH_GENOCIDE_EVENT\0".as_ptr() as *const ::core::ffi::c_char,
    b"SPATHI_SHIELD_EVENT\0".as_ptr() as *const ::core::ffi::c_char,
    b"ADVANCE_ILWRATH_MISSION\0".as_ptr() as *const ::core::ffi::c_char,
    b"ADVANCE_MYCON_MISSION\0".as_ptr() as *const ::core::ffi::c_char,
    b"ARILOU_UMGAH_CHECK\0".as_ptr() as *const ::core::ffi::c_char,
    b"YEHAT_REBEL_EVENT\0".as_ptr() as *const ::core::ffi::c_char,
    b"SLYLANDRO_RAMP_UP\0".as_ptr() as *const ::core::ffi::c_char,
    b"SLYLANDRO_RAMP_DOWN\0".as_ptr() as *const ::core::ffi::c_char,
];
#[no_mangle]
pub unsafe extern "C" fn initEventSystem() {
    static mut eventFuncs: [luaUqm_custom_Function; 19] = [
        luaUqm_custom_Function {
            name: b"arilou_entrance_event\0".as_ptr() as *const ::core::ffi::c_char,
            fun: Some(
                arilou_entrance_event
                    as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        luaUqm_custom_Function {
            name: b"arilou_exit_event\0".as_ptr() as *const ::core::ffi::c_char,
            fun: Some(
                arilou_exit_event
                    as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        luaUqm_custom_Function {
            name: b"hyperspace_encounter_event\0".as_ptr() as *const ::core::ffi::c_char,
            fun: Some(
                hyperspace_encounter_event
                    as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        luaUqm_custom_Function {
            name: b"kohr_ah_victorious_event\0".as_ptr() as *const ::core::ffi::c_char,
            fun: Some(
                kohr_ah_victorious_event
                    as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        luaUqm_custom_Function {
            name: b"advance_pkunk_mission\0".as_ptr() as *const ::core::ffi::c_char,
            fun: Some(
                advance_pkunk_mission
                    as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        luaUqm_custom_Function {
            name: b"advance_thradd_mission\0".as_ptr() as *const ::core::ffi::c_char,
            fun: Some(
                advance_thradd_mission
                    as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        luaUqm_custom_Function {
            name: b"zoqfot_distress_event\0".as_ptr() as *const ::core::ffi::c_char,
            fun: Some(
                zoqfot_distress_event
                    as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        luaUqm_custom_Function {
            name: b"zoqfot_death_event\0".as_ptr() as *const ::core::ffi::c_char,
            fun: Some(
                zoqfot_death_event
                    as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        luaUqm_custom_Function {
            name: b"shofixti_return_event\0".as_ptr() as *const ::core::ffi::c_char,
            fun: Some(
                shofixti_return_event
                    as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        luaUqm_custom_Function {
            name: b"advance_utwig_supox_mission\0".as_ptr()
                as *const ::core::ffi::c_char,
            fun: Some(
                advance_utwig_supox_mission
                    as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        luaUqm_custom_Function {
            name: b"kohr_ah_genocide_event\0".as_ptr() as *const ::core::ffi::c_char,
            fun: Some(
                kohr_ah_genocide_event
                    as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        luaUqm_custom_Function {
            name: b"spathi_shield_event\0".as_ptr() as *const ::core::ffi::c_char,
            fun: Some(
                spathi_shield_event
                    as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        luaUqm_custom_Function {
            name: b"advance_ilwrath_mission\0".as_ptr() as *const ::core::ffi::c_char,
            fun: Some(
                advance_ilwrath_mission
                    as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        luaUqm_custom_Function {
            name: b"advance_mycon_mission\0".as_ptr() as *const ::core::ffi::c_char,
            fun: Some(
                advance_mycon_mission
                    as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        luaUqm_custom_Function {
            name: b"arilou_umgah_check\0".as_ptr() as *const ::core::ffi::c_char,
            fun: Some(
                arilou_umgah_check
                    as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        luaUqm_custom_Function {
            name: b"yehat_rebel_event\0".as_ptr() as *const ::core::ffi::c_char,
            fun: Some(
                yehat_rebel_event
                    as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        luaUqm_custom_Function {
            name: b"slylandro_ramp_up\0".as_ptr() as *const ::core::ffi::c_char,
            fun: Some(
                slylandro_ramp_up
                    as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        luaUqm_custom_Function {
            name: b"slylandro_ramp_down\0".as_ptr() as *const ::core::ffi::c_char,
            fun: Some(
                slylandro_ramp_down
                    as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        luaUqm_custom_Function {
            name: ::core::ptr::null::<::core::ffi::c_char>(),
            fun: None,
        },
    ];
    uqm_lua_checked!(luaUqm_event_init(
        &raw const eventFuncs as *const luaUqm_custom_Function,
        b"event.script\0".as_ptr() as RESOURCE,
    ), ());
}
#[no_mangle]
pub unsafe extern "C" fn uninitEventSystem() {
    luaUqm_event_uninit();
}
#[no_mangle]
pub unsafe extern "C" fn eventIdStrToNum(
    mut eventIdStr: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut eventCount: size_t = ::core::mem::size_of::<
        [*const ::core::ffi::c_char; 18],
    >()
        .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>());
    let mut eventI: size_t = 0;
    eventI = 0 as size_t;
    while eventI < eventCount {
        if strcmp(eventIdStr, eventNames[eventI]) == 0 as ::core::ffi::c_int {
            return eventI as ::core::ffi::c_int;
        }
        eventI = eventI.wrapping_add(1);
    }
    return -1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn eventIdNumToStr(
    mut eventNum: ::core::ffi::c_int,
) -> *const ::core::ffi::c_char {
    let mut eventCount: size_t = ::core::mem::size_of::<
        [*const ::core::ffi::c_char; 18],
    >()
        .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>());
    if eventNum < 0 as ::core::ffi::c_int || eventNum as size_t >= eventCount {
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    return eventNames[eventNum as usize];
}
#[no_mangle]
pub unsafe extern "C" fn AddInitialGameEvents() {
    uqm_lua_checked!(AddEvent(
        EVENT_TYPE::RELATIVE_EVENT,
        0 as COUNT,
        1 as COUNT,
        0 as COUNT,
        C2Rust_Unnamed::HYPERSPACE_ENCOUNTER_EVENT.0 as ::core::ffi::c_int as BYTE,
    ), ());
    uqm_lua_checked!(AddEvent(
        EVENT_TYPE::ABSOLUTE_EVENT,
        3 as COUNT,
        17 as COUNT,
        2155 as COUNT,
        C2Rust_Unnamed::ARILOU_ENTRANCE_EVENT.0 as ::core::ffi::c_int as BYTE,
    ), ());
    uqm_lua_checked!(AddEvent(
        EVENT_TYPE::RELATIVE_EVENT,
        0 as COUNT,
        0 as COUNT,
        (if optDeCleansing.0 != 0 {
            100 as ::core::ffi::c_int
        } else {
            4 as ::core::ffi::c_int
        }
            + if if GlobData.SIS_state.Difficulty as ::core::ffi::c_int != 0 {
                GlobData.SIS_state.Difficulty as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } == 0 as ::core::ffi::c_int
            {
                0 as ::core::ffi::c_int
            } else if if GlobData.SIS_state.Difficulty as ::core::ffi::c_int != 0 {
                GlobData.SIS_state.Difficulty as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } == 1 as ::core::ffi::c_int
            {
                2 as ::core::ffi::c_int
            } else {
                -1 as ::core::ffi::c_int
            }) as COUNT,
        C2Rust_Unnamed::KOHR_AH_VICTORIOUS_EVENT.0 as ::core::ffi::c_int as BYTE,
    ), ());
    uqm_lua_checked!(AddEvent(
        EVENT_TYPE::RELATIVE_EVENT,
        0 as COUNT,
        0 as COUNT,
        0 as COUNT,
        C2Rust_Unnamed::SLYLANDRO_RAMP_UP.0 as ::core::ffi::c_int as BYTE,
    ), ());
}
#[no_mangle]
pub unsafe extern "C" fn EventHandler(mut selector: BYTE) {
    let mut eventIdStr: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    eventIdStr = eventIdNumToStr(selector as ::core::ffi::c_int);
    if eventIdStr.is_null() {
        log_add(
            log_Level::log_Warning,
            b"Warning: EventHandler(): Event %d is unknown.\0".as_ptr()
                as *const ::core::ffi::c_char,
            selector as ::core::ffi::c_int,
        );
        return;
    }
    uqm_lua_checked!(luaUqm_event_callEvent(eventIdStr), ());
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
unsafe extern "C" fn arilou_entrance_event(
    mut arg: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    uqm_lua_checked!(setGameStateUint(
        b"ARILOU_SPACE\0".as_ptr() as *const ::core::ffi::c_char,
        C2Rust_Unnamed_0::OPENING.0 as ::core::ffi::c_int as DWORD,
    ), 0);
    uqm_lua_checked!(AddEvent(
        EVENT_TYPE::RELATIVE_EVENT,
        0 as COUNT,
        3 as COUNT,
        0 as COUNT,
        C2Rust_Unnamed::ARILOU_EXIT_EVENT.0 as ::core::ffi::c_int as BYTE,
    ), 0);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn arilou_exit_event(
    mut arg: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut month_index: COUNT = 0;
    let mut year_index: COUNT = 0;
    year_index = GlobData.Game_state.GameClock.year_index;
    month_index = (GlobData.Game_state.GameClock.month_index as ::core::ffi::c_int
        % 12 as ::core::ffi::c_int) as COUNT;
    if month_index as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        year_index = year_index.wrapping_add(1);
    }
    month_index = month_index.wrapping_add(1);
    uqm_lua_checked!(setGameStateUint(
        b"ARILOU_SPACE\0".as_ptr() as *const ::core::ffi::c_char,
        C2Rust_Unnamed_0::CLOSING.0 as ::core::ffi::c_int as DWORD,
    ), 0);
    uqm_lua_checked!(AddEvent(
        EVENT_TYPE::ABSOLUTE_EVENT,
        month_index,
        17 as COUNT,
        year_index,
        C2Rust_Unnamed::ARILOU_ENTRANCE_EVENT.0 as ::core::ffi::c_int as BYTE,
    ), 0);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn check_race_growth() {
    let mut Index: BYTE = 0;
    let mut hStarShip: HFLEETINFO = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut hNextShip: HFLEETINFO = ::core::ptr::null_mut::<::core::ffi::c_void>();
    Index = 0 as BYTE;
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
                if !(Index as ::core::ffi::c_int
                    == RACE_ID::BLACK_URQUAN_SHIP.0 as ::core::ffi::c_int
                    && optCheatMode.0 != 0)
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
                    if (*FleetPtr).days_left as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                        && (*FleetPtr).func_index as ::core::ffi::c_int
                            != !(0 as ::core::ffi::c_int) as BYTE as ::core::ffi::c_int
                    {
                        uqm_lua_checked!(EventHandler((*FleetPtr).func_index), ());
                    }
                }
            }
        }
        UnlockLink(&raw mut GlobData.Game_state.avail_race_q, hStarShip);
        Index = Index.wrapping_add(1);
        hStarShip = hNextShip;
    }
}
unsafe extern "C" fn hyperspace_encounter_event(
    mut arg: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    uqm_lua_checked!(check_race_growth(), 0);
    if uqm_lua_checked!(inHyperSpace(), 0).0 != 0 {
        uqm_lua_checked!(check_hyperspace_encounter(), 0);
    }
    uqm_lua_checked!(AddEvent(
        EVENT_TYPE::RELATIVE_EVENT,
        0 as COUNT,
        1 as COUNT,
        0 as COUNT,
        C2Rust_Unnamed::HYPERSPACE_ENCOUNTER_EVENT.0 as ::core::ffi::c_int as BYTE,
    ), 0);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn kohr_ah_victorious_event(
    mut arg: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if uqm_lua_checked!(getGameStateUint(b"UTWIG_SUPOX_MISSION\0".as_ptr() as *const ::core::ffi::c_char), 0)
        != 0
    {
        uqm_lua_checked!(AddEvent(
            EVENT_TYPE::RELATIVE_EVENT,
            0 as COUNT,
            0 as COUNT,
            1 as COUNT,
            C2Rust_Unnamed::KOHR_AH_GENOCIDE_EVENT.0 as ::core::ffi::c_int as BYTE,
        ), 0);
        return 0 as ::core::ffi::c_int;
    }
    return uqm_lua_checked!(kohr_ah_genocide_event(arg), 0);
}
unsafe extern "C" fn advance_pkunk_mission(
    mut arg: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut hPkunk: HFLEETINFO = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut PkunkPtr: *mut FLEET_INFO = ::core::ptr::null_mut::<FLEET_INFO>();
    hPkunk = GetStarShipFromIndex(
        &raw mut GlobData.Game_state.avail_race_q,
        RACE_ID::PKUNK_SHIP.0 as ::core::ffi::c_int as COUNT,
    ) as HFLEETINFO;
    PkunkPtr = LockFleetInfo(&raw mut GlobData.Game_state.avail_race_q, hPkunk);
    let mut yehat: POINT = SeedFleetLocation(
        PkunkPtr,
        &raw mut plot_map as *mut PLOT_LOCATION,
        C2Rust_Unnamed_3::YEHAT_DEFINED.0 as ::core::ffi::c_int as COUNT,
    );
    let mut pkunk: POINT = SeedFleetLocation(
        PkunkPtr,
        &raw mut plot_map as *mut PLOT_LOCATION,
        0 as COUNT,
    );
    if (*PkunkPtr).actual_strength != 0 {
        let mut MissionState: BYTE = 0;
        MissionState = uqm_lua_checked!(getGameStateUint(
            b"PKUNK_MISSION\0".as_ptr() as *const ::core::ffi::c_char,
        ), 0) as BYTE;
        if (*PkunkPtr).days_left as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            && MissionState as ::core::ffi::c_int != 0
        {
            if MissionState as ::core::ffi::c_int & 1 as ::core::ffi::c_int != 0
                || (*PkunkPtr).loc.x as ::core::ffi::c_int
                    == yehat.x as ::core::ffi::c_int
                    && (*PkunkPtr).loc.y as ::core::ffi::c_int
                        == yehat.y as ::core::ffi::c_int
            {
                (*PkunkPtr).actual_strength = 0 as COUNT;
            } else if (*PkunkPtr).loc.x as ::core::ffi::c_int
                == pkunk.x as ::core::ffi::c_int
                && (*PkunkPtr).loc.y as ::core::ffi::c_int
                    == pkunk.y as ::core::ffi::c_int
                && uqm_lua_checked!(getGameStateUint(
                    b"PKUNK_ON_THE_MOVE\0".as_ptr() as *const ::core::ffi::c_char,
                ), 0) != 0
            {
                uqm_lua_checked!(setGameStateUint(
                    b"PKUNK_ON_THE_MOVE\0".as_ptr() as *const ::core::ffi::c_char,
                    0 as DWORD,
                ), 0);
                uqm_lua_checked!(AddEvent(
                    EVENT_TYPE::RELATIVE_EVENT,
                    3 as COUNT,
                    0 as COUNT,
                    0 as COUNT,
                    C2Rust_Unnamed::ADVANCE_PKUNK_MISSION.0 as ::core::ffi::c_int as BYTE,
                ), 0);
                UnlockLink(&raw mut GlobData.Game_state.avail_race_q, hPkunk);
                return 0 as ::core::ffi::c_int;
            }
        }
        if (*PkunkPtr).actual_strength as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            uqm_lua_checked!(setGameStateUint(
                b"YEHAT_ABSORBED_PKUNK\0".as_ptr() as *const ::core::ffi::c_char,
                1 as DWORD,
            ), 0);
            (*PkunkPtr).allied_state = C2Rust_Unnamed_1::DEAD_GUY.0 as ::core::ffi::c_int
                as UWORD;
            StartSphereTracking(RACE_ID::YEHAT_SHIP);
        } else {
            let mut loc: POINT = POINT { x: 0, y: 0 };
            if MissionState as ::core::ffi::c_int & 1 as ::core::ffi::c_int == 0 {
                loc = yehat;
            } else {
                loc = pkunk;
            }
            uqm_lua_checked!(setGameStateUint(
                b"PKUNK_ON_THE_MOVE\0".as_ptr() as *const ::core::ffi::c_char,
                1 as DWORD,
            ), 0);
            uqm_lua_checked!(setGameStateUint(
                b"PKUNK_SWITCH\0".as_ptr() as *const ::core::ffi::c_char,
                0 as DWORD,
            ), 0);
            SetRaceDest(
                RACE_ID::PKUNK_SHIP.0 as ::core::ffi::c_int as BYTE,
                loc.x,
                loc.y,
                ((365 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
                    - (*PkunkPtr).days_left as ::core::ffi::c_int) as BYTE,
                C2Rust_Unnamed::ADVANCE_PKUNK_MISSION.0 as ::core::ffi::c_int as BYTE,
            );
        }
        uqm_lua_checked!(setGameStateUint(
            b"PKUNK_MISSION\0".as_ptr() as *const ::core::ffi::c_char,
            (MissionState as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as DWORD,
        ), 0);
    }
    UnlockLink(&raw mut GlobData.Game_state.avail_race_q, hPkunk);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn advance_thradd_mission(
    mut arg: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut MissionState: BYTE = 0;
    let mut hThradd: HFLEETINFO = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut ThraddPtr: *mut FLEET_INFO = ::core::ptr::null_mut::<FLEET_INFO>();
    hThradd = GetStarShipFromIndex(
        &raw mut GlobData.Game_state.avail_race_q,
        RACE_ID::THRADDASH_SHIP.0 as ::core::ffi::c_int as COUNT,
    ) as HFLEETINFO;
    ThraddPtr = LockFleetInfo(&raw mut GlobData.Game_state.avail_race_q, hThradd);
    MissionState = uqm_lua_checked!(getGameStateUint(
        b"THRADD_MISSION\0".as_ptr() as *const ::core::ffi::c_char,
    ), 0) as BYTE;
    if (*ThraddPtr).actual_strength as ::core::ffi::c_int != 0
        && (MissionState as ::core::ffi::c_int) < 3 as ::core::ffi::c_int
    {
        let mut loc: POINT = POINT { x: 0, y: 0 };
        if (MissionState as ::core::ffi::c_int) < 2 as ::core::ffi::c_int {
            loc = SeedFleetLocation(
                ThraddPtr,
                &raw mut plot_map as *mut PLOT_LOCATION,
                C2Rust_Unnamed_3::SAMATRA_DEFINED.0 as ::core::ffi::c_int as COUNT,
            );
        } else {
            loc = SeedFleetLocation(
                ThraddPtr,
                &raw mut plot_map as *mut PLOT_LOCATION,
                0 as COUNT,
            );
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
            RACE_ID::THRADDASH_SHIP.0 as ::core::ffi::c_int as BYTE,
            loc.x,
            loc.y,
            14 as BYTE,
            C2Rust_Unnamed::ADVANCE_THRADD_MISSION.0 as ::core::ffi::c_int as BYTE,
        );
    }
    MissionState = MissionState.wrapping_add(1);
    uqm_lua_checked!(setGameStateUint(
        b"THRADD_MISSION\0".as_ptr() as *const ::core::ffi::c_char,
        MissionState as DWORD,
    ), 0);
    if MissionState as ::core::ffi::c_int == 4 as ::core::ffi::c_int
        && uqm_lua_checked!(getGameStateUint(
            b"ILWRATH_FIGHT_THRADDASH\0".as_ptr() as *const ::core::ffi::c_char,
        ), 0) != 0
    {
        uqm_lua_checked!(AddEvent(
            EVENT_TYPE::RELATIVE_EVENT,
            0 as COUNT,
            0 as COUNT,
            0 as COUNT,
            C2Rust_Unnamed::ADVANCE_ILWRATH_MISSION.0 as ::core::ffi::c_int as BYTE,
        ), 0);
    }
    UnlockLink(&raw mut GlobData.Game_state.avail_race_q, hThradd);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn zoqfot_distress_event(
    mut arg: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if GlobData.Game_state.CurrentActivity as BYTE as ::core::ffi::c_int
        == C2Rust_Unnamed_2::IN_INTERPLANETARY.0 as ::core::ffi::c_int
        && !CurStarDescPtr.is_null()
        && (*CurStarDescPtr).Index as ::core::ffi::c_int
            == C2Rust_Unnamed_3::ZOQFOT_DEFINED.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(AddEvent(
            EVENT_TYPE::RELATIVE_EVENT,
            0 as COUNT,
            7 as COUNT,
            0 as COUNT,
            C2Rust_Unnamed::ZOQFOT_DISTRESS_EVENT.0 as ::core::ffi::c_int as BYTE,
        ), 0);
    } else {
        uqm_lua_checked!(setGameStateUint(
            b"ZOQFOT_DISTRESS\0".as_ptr() as *const ::core::ffi::c_char,
            1 as DWORD,
        ), 0);
        uqm_lua_checked!(AddEvent(
            EVENT_TYPE::RELATIVE_EVENT,
            6 as COUNT,
            0 as COUNT,
            0 as COUNT,
            C2Rust_Unnamed::ZOQFOT_DEATH_EVENT.0 as ::core::ffi::c_int as BYTE,
        ), 0);
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn zoqfot_death_event(
    mut arg: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if GlobData.Game_state.CurrentActivity as BYTE as ::core::ffi::c_int
        == C2Rust_Unnamed_2::IN_INTERPLANETARY.0 as ::core::ffi::c_int
        && !CurStarDescPtr.is_null()
        && (*CurStarDescPtr).Index as ::core::ffi::c_int
            == C2Rust_Unnamed_3::ZOQFOT_DEFINED.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(AddEvent(
            EVENT_TYPE::RELATIVE_EVENT,
            0 as COUNT,
            7 as COUNT,
            0 as COUNT,
            C2Rust_Unnamed::ZOQFOT_DEATH_EVENT.0 as ::core::ffi::c_int as BYTE,
        ), 0);
    } else if uqm_lua_checked!(getGameStateUint(
        b"ZOQFOT_DISTRESS\0".as_ptr() as *const ::core::ffi::c_char,
    ), 0) != 0
    {
        let mut hZoqFot: HFLEETINFO = ::core::ptr::null_mut::<::core::ffi::c_void>();
        let mut ZoqFotPtr: *mut FLEET_INFO = ::core::ptr::null_mut::<FLEET_INFO>();
        hZoqFot = GetStarShipFromIndex(
            &raw mut GlobData.Game_state.avail_race_q,
            RACE_ID::ZOQFOTPIK_SHIP.0 as ::core::ffi::c_int as COUNT,
        ) as HFLEETINFO;
        ZoqFotPtr = LockFleetInfo(&raw mut GlobData.Game_state.avail_race_q, hZoqFot);
        (*ZoqFotPtr).actual_strength = 0 as COUNT;
        (*ZoqFotPtr).allied_state = C2Rust_Unnamed_1::DEAD_GUY.0 as ::core::ffi::c_int
            as UWORD;
        UnlockLink(&raw mut GlobData.Game_state.avail_race_q, hZoqFot);
        uqm_lua_checked!(setGameStateUint(
            b"ZOQFOT_DISTRESS\0".as_ptr() as *const ::core::ffi::c_char,
            2 as DWORD,
        ), 0);
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn shofixti_return_event(
    mut arg: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut hShofixti: HFLEETINFO = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut ShofixtiPtr: *mut FLEET_INFO = ::core::ptr::null_mut::<FLEET_INFO>();
    hShofixti = GetStarShipFromIndex(
        &raw mut GlobData.Game_state.avail_race_q,
        RACE_ID::SHOFIXTI_SHIP.0 as ::core::ffi::c_int as COUNT,
    ) as HFLEETINFO;
    ShofixtiPtr = LockFleetInfo(&raw mut GlobData.Game_state.avail_race_q, hShofixti);
    if (*ShofixtiPtr).actual_strength != 0 {
        (*ShofixtiPtr).growth = 0 as BYTE;
        (*ShofixtiPtr).growth_fract = 0 as BYTE;
        return 0 as ::core::ffi::c_int;
    }
    if if GlobData.SIS_state.Extended as ::core::ffi::c_int != 0 {
        BOOLEAN::TRUE.0 as ::core::ffi::c_int
    } else {
        BOOLEAN::FALSE.0 as ::core::ffi::c_int
    } != 0
    {
        (*ShofixtiPtr).actual_strength = (150 as ::core::ffi::c_int
            / 11 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as COUNT;
        (*ShofixtiPtr).loc = SeedFleetLocation(
            ShofixtiPtr,
            &raw mut plot_map as *mut PLOT_LOCATION,
            C2Rust_Unnamed_3::HOME.0 as ::core::ffi::c_int as COUNT,
        );
        StartSphereTracking(RACE_ID::SHOFIXTI_SHIP);
        (*ShofixtiPtr).growth = 1 as BYTE;
        (*ShofixtiPtr).growth_fract = 0 as BYTE;
        uqm_lua_checked!(AddEvent(
            EVENT_TYPE::RELATIVE_EVENT,
            3 as COUNT,
            0 as COUNT,
            0 as COUNT,
            C2Rust_Unnamed::SHOFIXTI_RETURN_EVENT.0 as ::core::ffi::c_int as BYTE,
        ), 0);
    }
    SetRaceAllied(RACE_ID::SHOFIXTI_SHIP, BOOLEAN::TRUE);
    GlobData.Game_state.CrewCost = (GlobData.Game_state.CrewCost as ::core::ffi::c_int
        - if if if GlobData.SIS_state.Difficulty as ::core::ffi::c_int != 0 {
            GlobData.SIS_state.Difficulty as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        } == 2 as ::core::ffi::c_int
        {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        } == 0
        {
            2 as ::core::ffi::c_int
        } else {
            1 as ::core::ffi::c_int
        }) as BYTE;
    uqm_lua_checked!(setGameStateUint(
        b"CREW_PURCHASED0\0".as_ptr() as *const ::core::ffi::c_char,
        0 as DWORD,
    ), 0);
    uqm_lua_checked!(setGameStateUint(
        b"CREW_PURCHASED1\0".as_ptr() as *const ::core::ffi::c_char,
        0 as DWORD,
    ), 0);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn advance_utwig_supox_mission(
    mut arg: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut MissionState: BYTE = 0;
    let mut hUtwig: HFLEETINFO = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut hSupox: HFLEETINFO = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut hKohrAh: HFLEETINFO = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut UtwigPtr: *mut FLEET_INFO = ::core::ptr::null_mut::<FLEET_INFO>();
    let mut SupoxPtr: *mut FLEET_INFO = ::core::ptr::null_mut::<FLEET_INFO>();
    let mut KohrAhPtr: *mut FLEET_INFO = ::core::ptr::null_mut::<FLEET_INFO>();
    hUtwig = GetStarShipFromIndex(
        &raw mut GlobData.Game_state.avail_race_q,
        RACE_ID::UTWIG_SHIP.0 as ::core::ffi::c_int as COUNT,
    ) as HFLEETINFO;
    UtwigPtr = LockFleetInfo(&raw mut GlobData.Game_state.avail_race_q, hUtwig);
    hSupox = GetStarShipFromIndex(
        &raw mut GlobData.Game_state.avail_race_q,
        RACE_ID::SUPOX_SHIP.0 as ::core::ffi::c_int as COUNT,
    ) as HFLEETINFO;
    SupoxPtr = LockFleetInfo(&raw mut GlobData.Game_state.avail_race_q, hSupox);
    hKohrAh = GetStarShipFromIndex(
        &raw mut GlobData.Game_state.avail_race_q,
        RACE_ID::BLACK_URQUAN_SHIP.0 as ::core::ffi::c_int as COUNT,
    ) as HFLEETINFO;
    KohrAhPtr = LockFleetInfo(&raw mut GlobData.Game_state.avail_race_q, hKohrAh);
    MissionState = uqm_lua_checked!(getGameStateUint(
        b"UTWIG_SUPOX_MISSION\0".as_ptr() as *const ::core::ffi::c_char,
    ), 0) as BYTE;
    if (*UtwigPtr).actual_strength as ::core::ffi::c_int != 0
        && (MissionState as ::core::ffi::c_int) < 5 as ::core::ffi::c_int
    {
        if MissionState as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            let mut strength_loss: SIZE = 0;
            uqm_lua_checked!(AddEvent(
                EVENT_TYPE::RELATIVE_EVENT,
                0 as COUNT,
                (160 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as COUNT,
                0 as COUNT,
                C2Rust_Unnamed::ADVANCE_UTWIG_SUPOX_MISSION.0 as ::core::ffi::c_int
                    as BYTE,
            ), 0);
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
            if if GlobData.SIS_state.Extended as ::core::ffi::c_int != 0 {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                strength_loss = ((*KohrAhPtr).actual_strength as ::core::ffi::c_int
                    >> 5 as ::core::ffi::c_int) as SIZE;
                (*KohrAhPtr).growth = (-(strength_loss as ::core::ffi::c_int)
                    / 160 as ::core::ffi::c_int) as BYTE;
                (*KohrAhPtr).growth_fract = (((strength_loss as ::core::ffi::c_int
                    % 160 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
                    / 160 as ::core::ffi::c_int) as BYTE;
                (*KohrAhPtr).growth_err_term = (255 as ::core::ffi::c_int
                    >> 1 as ::core::ffi::c_int) as BYTE;
            }
            uqm_lua_checked!(setGameStateUint(
                b"UTWIG_WAR_NEWS\0".as_ptr() as *const ::core::ffi::c_char,
                0 as DWORD,
            ), 0);
            uqm_lua_checked!(setGameStateUint(
                b"SUPOX_WAR_NEWS\0".as_ptr() as *const ::core::ffi::c_char,
                0 as DWORD,
            ), 0);
        } else if MissionState as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            uqm_lua_checked!(AddEvent(
                EVENT_TYPE::RELATIVE_EVENT,
                0 as COUNT,
                (160 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as COUNT,
                0 as COUNT,
                C2Rust_Unnamed::ADVANCE_UTWIG_SUPOX_MISSION.0 as ::core::ffi::c_int
                    as BYTE,
            ), 0);
            MissionState = MissionState.wrapping_add(1);
        } else {
            let mut utwig: POINT = POINT { x: 0, y: 0 };
            let mut supox: POINT = POINT { x: 0, y: 0 };
            if MissionState as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                utwig = SeedFleetLocation(
                    UtwigPtr,
                    &raw mut plot_map as *mut PLOT_LOCATION,
                    C2Rust_Unnamed_3::SAMATRA_DEFINED.0 as ::core::ffi::c_int as COUNT,
                );
                supox = SeedFleetLocation(
                    SupoxPtr,
                    &raw mut plot_map as *mut PLOT_LOCATION,
                    C2Rust_Unnamed_3::SAMATRA_DEFINED.0 as ::core::ffi::c_int as COUNT,
                );
            } else {
                utwig = SeedFleetLocation(
                    UtwigPtr,
                    &raw mut plot_map as *mut PLOT_LOCATION,
                    0 as COUNT,
                );
                supox = SeedFleetLocation(
                    SupoxPtr,
                    &raw mut plot_map as *mut PLOT_LOCATION,
                    0 as COUNT,
                );
                (*UtwigPtr).growth = 0 as BYTE;
                (*UtwigPtr).growth_fract = 0 as BYTE;
                (*SupoxPtr).growth = 0 as BYTE;
                (*SupoxPtr).growth_fract = 0 as BYTE;
                if if GlobData.SIS_state.Extended as ::core::ffi::c_int != 0 {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    (*KohrAhPtr).growth = 0 as BYTE;
                    (*KohrAhPtr).growth_fract = 0 as BYTE;
                }
                uqm_lua_checked!(setGameStateUint(
                    b"UTWIG_WAR_NEWS\0".as_ptr() as *const ::core::ffi::c_char,
                    0 as DWORD,
                ), 0);
                uqm_lua_checked!(setGameStateUint(
                    b"SUPOX_WAR_NEWS\0".as_ptr() as *const ::core::ffi::c_char,
                    0 as DWORD,
                ), 0);
            }
            uqm_lua_checked!(setGameStateUint(
                b"UTWIG_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
                0 as DWORD,
            ), 0);
            uqm_lua_checked!(setGameStateUint(
                b"UTWIG_INFO\0".as_ptr() as *const ::core::ffi::c_char,
                0 as DWORD,
            ), 0);
            uqm_lua_checked!(setGameStateUint(
                b"SUPOX_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
                0 as DWORD,
            ), 0);
            uqm_lua_checked!(setGameStateUint(
                b"SUPOX_INFO\0".as_ptr() as *const ::core::ffi::c_char,
                0 as DWORD,
            ), 0);
            SetRaceDest(
                RACE_ID::UTWIG_SHIP.0 as ::core::ffi::c_int as BYTE,
                utwig.x,
                utwig.y,
                21 as BYTE,
                C2Rust_Unnamed::ADVANCE_UTWIG_SUPOX_MISSION.0 as ::core::ffi::c_int
                    as BYTE,
            );
            SetRaceDest(
                RACE_ID::SUPOX_SHIP.0 as ::core::ffi::c_int as BYTE,
                supox.x,
                supox.y,
                21 as BYTE,
                !(0 as ::core::ffi::c_int) as BYTE,
            );
        }
    }
    uqm_lua_checked!(setGameStateUint(
        b"UTWIG_SUPOX_MISSION\0".as_ptr() as *const ::core::ffi::c_char,
        (MissionState as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as DWORD,
    ), 0);
    UnlockLink(&raw mut GlobData.Game_state.avail_race_q, hSupox);
    UnlockLink(&raw mut GlobData.Game_state.avail_race_q, hUtwig);
    UnlockLink(&raw mut GlobData.Game_state.avail_race_q, hKohrAh);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn kohr_ah_genocide_event(
    mut arg: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut Index: BYTE = 0;
    let mut best_dist: ::core::ffi::c_long = 0;
    let mut best_dx: SIZE = 0;
    let mut best_dy: SIZE = 0;
    let mut hStarShip: HFLEETINFO = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut hNextShip: HFLEETINFO = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut hBlackUrquan: HFLEETINFO = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut BlackUrquanPtr: *mut FLEET_INFO = ::core::ptr::null_mut::<FLEET_INFO>();
    if uqm_lua_checked!(getGameStateUint(b"KOHR_AH_FRENZY\0".as_ptr() as *const ::core::ffi::c_char), 0) == 0
        && GlobData.Game_state.CurrentActivity as BYTE as ::core::ffi::c_int
            == C2Rust_Unnamed_2::IN_INTERPLANETARY.0 as ::core::ffi::c_int
        && !CurStarDescPtr.is_null()
        && (*CurStarDescPtr).Index as ::core::ffi::c_int
            == C2Rust_Unnamed_3::SAMATRA_DEFINED.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(AddEvent(
            EVENT_TYPE::RELATIVE_EVENT,
            0 as COUNT,
            7 as COUNT,
            0 as COUNT,
            C2Rust_Unnamed::KOHR_AH_GENOCIDE_EVENT.0 as ::core::ffi::c_int as BYTE,
        ), 0);
        return 0 as ::core::ffi::c_int;
    }
    hBlackUrquan = GetStarShipFromIndex(
        &raw mut GlobData.Game_state.avail_race_q,
        RACE_ID::BLACK_URQUAN_SHIP.0 as ::core::ffi::c_int as COUNT,
    ) as HFLEETINFO;
    BlackUrquanPtr = LockFleetInfo(
        &raw mut GlobData.Game_state.avail_race_q,
        hBlackUrquan,
    );
    best_dist = -1 as ::core::ffi::c_long;
    best_dx = (plot_map[C2Rust_Unnamed_3::SOL_DEFINED.0 as ::core::ffi::c_int as usize]
        .star_pt
        .x as ::core::ffi::c_int - (*BlackUrquanPtr).loc.x as ::core::ffi::c_int)
        as SIZE;
    best_dy = (plot_map[C2Rust_Unnamed_3::SOL_DEFINED.0 as ::core::ffi::c_int as usize]
        .star_pt
        .y as ::core::ffi::c_int - (*BlackUrquanPtr).loc.y as ::core::ffi::c_int)
        as SIZE;
    Index = 0 as BYTE;
    hStarShip = GlobData.Game_state.avail_race_q.head as HFLEETINFO;
    while !hStarShip.is_null() {
        let mut FleetPtr: *mut FLEET_INFO = ::core::ptr::null_mut::<FLEET_INFO>();
        FleetPtr = LockFleetInfo(&raw mut GlobData.Game_state.avail_race_q, hStarShip);
        hNextShip = (*FleetPtr).succ;
        if Index as ::core::ffi::c_int
            != RACE_ID::BLACK_URQUAN_SHIP.0 as ::core::ffi::c_int
            && Index as ::core::ffi::c_int
                != RACE_ID::URQUAN_SHIP.0 as ::core::ffi::c_int
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
                        == RACE_ID::DRUUGE_SHIP.0 as ::core::ffi::c_int
                {
                    best_dist = dist;
                    best_dx = dx;
                    best_dy = dy;
                    if Index as ::core::ffi::c_int
                        == RACE_ID::DRUUGE_SHIP.0 as ::core::ffi::c_int
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
            & !(C2Rust_Unnamed_2::IN_BATTLE.0 as ::core::ffi::c_int)) as ACTIVITY;
        GlobData.SIS_state.CrewEnlisted = !(0 as ::core::ffi::c_int) as COUNT;
        uqm_lua_checked!(setGameStateUint(
            b"KOHR_AH_KILLED_ALL\0".as_ptr() as *const ::core::ffi::c_char,
            1 as DWORD,
        ), 0);
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
        uqm_lua_checked!(setGameStateUint(
            b"KOHR_AH_FRENZY\0".as_ptr() as *const ::core::ffi::c_char,
            1 as DWORD,
        ), 0);
        uqm_lua_checked!(setGameStateUint(
            b"KOHR_AH_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
            0 as DWORD,
        ), 0);
        uqm_lua_checked!(setGameStateUint(
            b"KOHR_AH_REASONS\0".as_ptr() as *const ::core::ffi::c_char,
            0 as DWORD,
        ), 0);
        uqm_lua_checked!(setGameStateUint(
            b"KOHR_AH_PLEAD\0".as_ptr() as *const ::core::ffi::c_char,
            0 as DWORD,
        ), 0);
        uqm_lua_checked!(setGameStateUint(
            b"KOHR_AH_INFO\0".as_ptr() as *const ::core::ffi::c_char,
            0 as DWORD,
        ), 0);
        uqm_lua_checked!(setGameStateUint(
            b"URQUAN_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
            0 as DWORD,
        ), 0);
        SetRaceDest(
            RACE_ID::BLACK_URQUAN_SHIP.0 as ::core::ffi::c_int as BYTE,
            ((*BlackUrquanPtr).loc.x as ::core::ffi::c_int
                + best_dx as ::core::ffi::c_int) as COORD,
            ((*BlackUrquanPtr).loc.y as ::core::ffi::c_int
                + best_dy as ::core::ffi::c_int) as COORD,
            speed as BYTE,
            C2Rust_Unnamed::KOHR_AH_GENOCIDE_EVENT.0 as ::core::ffi::c_int as BYTE,
        );
    }
    UnlockLink(&raw mut GlobData.Game_state.avail_race_q, hBlackUrquan);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn spathi_shield_event(
    mut arg: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if GlobData.Game_state.CurrentActivity as BYTE as ::core::ffi::c_int
        == C2Rust_Unnamed_2::IN_INTERPLANETARY.0 as ::core::ffi::c_int
        && !CurStarDescPtr.is_null()
        && (*CurStarDescPtr).Index as ::core::ffi::c_int
            == C2Rust_Unnamed_3::SPATHI_DEFINED.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(AddEvent(
            EVENT_TYPE::RELATIVE_EVENT,
            0 as COUNT,
            7 as COUNT,
            0 as COUNT,
            C2Rust_Unnamed::SPATHI_SHIELD_EVENT.0 as ::core::ffi::c_int as BYTE,
        ), 0);
    } else {
        let mut hSpathi: HFLEETINFO = ::core::ptr::null_mut::<::core::ffi::c_void>();
        let mut SpathiPtr: *mut FLEET_INFO = ::core::ptr::null_mut::<FLEET_INFO>();
        hSpathi = GetStarShipFromIndex(
            &raw mut GlobData.Game_state.avail_race_q,
            RACE_ID::SPATHI_SHIP.0 as ::core::ffi::c_int as COUNT,
        ) as HFLEETINFO;
        SpathiPtr = LockFleetInfo(&raw mut GlobData.Game_state.avail_race_q, hSpathi);
        if (*SpathiPtr).actual_strength != 0 {
            SetRaceAllied(RACE_ID::SPATHI_SHIP, BOOLEAN::FALSE);
            if if if GlobData.SIS_state.Difficulty as ::core::ffi::c_int != 0 {
                GlobData.SIS_state.Difficulty as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } == 2 as ::core::ffi::c_int
            {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } != 0
            {
                uqm_lua_checked!(RemoveEscortShips(RACE_ID::SPATHI_SHIP), 0);
            }
            uqm_lua_checked!(setGameStateUint(
                b"SPATHI_SHIELDED_SELVES\0".as_ptr() as *const ::core::ffi::c_char,
                1 as DWORD,
            ), 0);
            (*SpathiPtr).actual_strength = 0 as COUNT;
        }
        UnlockLink(&raw mut GlobData.Game_state.avail_race_q, hSpathi);
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn advance_ilwrath_mission(
    mut arg: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ThraddState: BYTE = 0;
    let mut hIlwrath: HFLEETINFO = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut hThradd: HFLEETINFO = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut IlwrathPtr: *mut FLEET_INFO = ::core::ptr::null_mut::<FLEET_INFO>();
    let mut ThraddPtr: *mut FLEET_INFO = ::core::ptr::null_mut::<FLEET_INFO>();
    hIlwrath = GetStarShipFromIndex(
        &raw mut GlobData.Game_state.avail_race_q,
        RACE_ID::ILWRATH_SHIP.0 as ::core::ffi::c_int as COUNT,
    ) as HFLEETINFO;
    IlwrathPtr = LockFleetInfo(&raw mut GlobData.Game_state.avail_race_q, hIlwrath);
    hThradd = GetStarShipFromIndex(
        &raw mut GlobData.Game_state.avail_race_q,
        RACE_ID::THRADDASH_SHIP.0 as ::core::ffi::c_int as COUNT,
    ) as HFLEETINFO;
    ThraddPtr = LockFleetInfo(&raw mut GlobData.Game_state.avail_race_q, hThradd);
    let mut thradd_home: POINT = SeedFleetLocation(
        ThraddPtr,
        &raw mut plot_map as *mut PLOT_LOCATION,
        0 as COUNT,
    );
    let mut conflict: POINT = SeedFleetLocation(
        IlwrathPtr,
        &raw mut plot_map as *mut PLOT_LOCATION,
        C2Rust_Unnamed_3::THRADD_DEFINED.0 as ::core::ffi::c_int as COUNT,
    );
    if uqm_lua_checked!(getGameStateUint(
        b"ILWRATH_FIGHT_THRADDASH\0".as_ptr() as *const ::core::ffi::c_char,
    ), 0) != 0
    {
        (*IlwrathPtr).actual_strength = 0 as COUNT;
        (*IlwrathPtr).allied_state = C2Rust_Unnamed_1::DEAD_GUY.0 as ::core::ffi::c_int
            as UWORD;
        if if GlobData.SIS_state.Extended as ::core::ffi::c_int != 0 {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
            && (*ThraddPtr).allied_state as ::core::ffi::c_int
                != C2Rust_Unnamed_1::GOOD_GUY.0 as ::core::ffi::c_int
            || if GlobData.SIS_state.Extended as ::core::ffi::c_int != 0 {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } == 0
        {
            (*ThraddPtr).actual_strength = 0 as COUNT;
            (*ThraddPtr).allied_state = C2Rust_Unnamed_1::DEAD_GUY.0
                as ::core::ffi::c_int as UWORD;
        }
    } else if (*IlwrathPtr).actual_strength != 0 {
        if (*IlwrathPtr).dest_loc.x as ::core::ffi::c_int
            != conflict.x as ::core::ffi::c_int
            || (*IlwrathPtr).dest_loc.y as ::core::ffi::c_int
                != conflict.y as ::core::ffi::c_int
        {
            SetRaceDest(
                RACE_ID::ILWRATH_SHIP.0 as ::core::ffi::c_int as BYTE,
                conflict.x,
                conflict.y,
                90 as BYTE,
                C2Rust_Unnamed::ADVANCE_ILWRATH_MISSION.0 as ::core::ffi::c_int as BYTE,
            );
        } else {
            let mut strength_loss: SIZE = 0;
            if (*IlwrathPtr).days_left as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                uqm_lua_checked!(setGameStateUint(
                    b"ILWRATH_FIGHT_THRADDASH\0".as_ptr() as *const ::core::ffi::c_char,
                    1 as DWORD,
                ), 0);
                uqm_lua_checked!(setGameStateUint(
                    b"HELIX_UNPROTECTED\0".as_ptr() as *const ::core::ffi::c_char,
                    1 as DWORD,
                ), 0);
                strength_loss = (*IlwrathPtr).actual_strength as SIZE;
                (*IlwrathPtr).growth = (-(strength_loss as ::core::ffi::c_int)
                    / 128 as ::core::ffi::c_int) as BYTE;
                (*IlwrathPtr).growth_fract = (((strength_loss as ::core::ffi::c_int
                    % 128 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
                    / 128 as ::core::ffi::c_int) as BYTE;
                SetRaceDest(
                    RACE_ID::ILWRATH_SHIP.0 as ::core::ffi::c_int as BYTE,
                    ((conflict.x as ::core::ffi::c_int
                        + thradd_home.x as ::core::ffi::c_int) / 2 as ::core::ffi::c_int)
                        as COORD,
                    ((conflict.y as ::core::ffi::c_int
                        + thradd_home.y as ::core::ffi::c_int) / 2 as ::core::ffi::c_int)
                        as COORD,
                    (128 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as BYTE,
                    C2Rust_Unnamed::ADVANCE_ILWRATH_MISSION.0 as ::core::ffi::c_int
                        as BYTE,
                );
                if if GlobData.SIS_state.Extended as ::core::ffi::c_int != 0 {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                    && (*ThraddPtr).allied_state as ::core::ffi::c_int
                        == C2Rust_Unnamed_1::GOOD_GUY.0 as ::core::ffi::c_int
                {
                    strength_loss = ((*ThraddPtr).actual_strength as ::core::ffi::c_int
                        as ::core::ffi::c_double * 0.25f64) as SIZE;
                } else {
                    strength_loss = (*ThraddPtr).actual_strength as SIZE;
                }
                (*ThraddPtr).growth = (-(strength_loss as ::core::ffi::c_int)
                    / 128 as ::core::ffi::c_int) as BYTE;
                (*ThraddPtr).growth_fract = (((strength_loss as ::core::ffi::c_int
                    % 128 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
                    / 128 as ::core::ffi::c_int) as BYTE;
                if if GlobData.SIS_state.Extended as ::core::ffi::c_int != 0 {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                    && (*ThraddPtr).allied_state as ::core::ffi::c_int
                        != C2Rust_Unnamed_1::GOOD_GUY.0 as ::core::ffi::c_int
                    || if GlobData.SIS_state.Extended as ::core::ffi::c_int != 0 {
                        BOOLEAN::TRUE.0 as ::core::ffi::c_int
                    } else {
                        BOOLEAN::FALSE.0 as ::core::ffi::c_int
                    } == 0
                {
                    uqm_lua_checked!(setGameStateUint(
                        b"THRADD_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
                        0 as DWORD,
                    ), 0);
                    if (*ThraddPtr).allied_state as ::core::ffi::c_int
                        == C2Rust_Unnamed_1::GOOD_GUY.0 as ::core::ffi::c_int
                    {
                        SetRaceAllied(RACE_ID::THRADDASH_SHIP, BOOLEAN::FALSE);
                    }
                }
            }
            ThraddState = uqm_lua_checked!(getGameStateUint(
                b"THRADD_MISSION\0".as_ptr() as *const ::core::ffi::c_char,
            ), 0) as BYTE;
            if ThraddState as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                || ThraddState as ::core::ffi::c_int > 3 as ::core::ffi::c_int
            {
                SetRaceDest(
                    RACE_ID::THRADDASH_SHIP.0 as ::core::ffi::c_int as BYTE,
                    ((conflict.x as ::core::ffi::c_int
                        + thradd_home.x as ::core::ffi::c_int) / 2 as ::core::ffi::c_int)
                        as COORD,
                    ((conflict.y as ::core::ffi::c_int
                        + thradd_home.y as ::core::ffi::c_int) / 2 as ::core::ffi::c_int)
                        as COORD,
                    ((*IlwrathPtr).days_left as ::core::ffi::c_int
                        + 1 as ::core::ffi::c_int) as BYTE,
                    !(0 as ::core::ffi::c_int) as BYTE,
                );
            } else if (ThraddState as ::core::ffi::c_int) < 3 as ::core::ffi::c_int {
                SetRaceDest(
                    RACE_ID::THRADDASH_SHIP.0 as ::core::ffi::c_int as BYTE,
                    thradd_home.x,
                    thradd_home.y,
                    10 as BYTE,
                    C2Rust_Unnamed::ADVANCE_THRADD_MISSION.0 as ::core::ffi::c_int
                        as BYTE,
                );
                uqm_lua_checked!(setGameStateUint(
                    b"THRADD_MISSION\0".as_ptr() as *const ::core::ffi::c_char,
                    3 as DWORD,
                ), 0);
            }
        }
    }
    if if GlobData.SIS_state.Extended as ::core::ffi::c_int != 0 {
        BOOLEAN::TRUE.0 as ::core::ffi::c_int
    } else {
        BOOLEAN::FALSE.0 as ::core::ffi::c_int
    } != 0
        && (*ThraddPtr).allied_state as ::core::ffi::c_int
            == C2Rust_Unnamed_1::GOOD_GUY.0 as ::core::ffi::c_int
        && (*IlwrathPtr).actual_strength == 0
    {
        (*ThraddPtr).growth = 0 as BYTE;
        (*ThraddPtr).growth_fract = 0 as BYTE;
        uqm_lua_checked!(setGameStateUint(
            b"ILWRATH_FIGHT_THRADDASH\0".as_ptr() as *const ::core::ffi::c_char,
            0 as DWORD,
        ), 0);
        SetRaceDest(
            RACE_ID::THRADDASH_SHIP.0 as ::core::ffi::c_int as BYTE,
            thradd_home.x,
            thradd_home.y,
            3 as BYTE,
            !(0 as ::core::ffi::c_int) as BYTE,
        );
        if uqm_lua_checked!(getGameStateUint(b"AQUA_HELIX\0".as_ptr() as *const ::core::ffi::c_char), 0) == 0
        {
            uqm_lua_checked!(setGameStateUint(
                b"HELIX_UNPROTECTED\0".as_ptr() as *const ::core::ffi::c_char,
                0 as DWORD,
            ), 0);
        }
    }
    UnlockLink(&raw mut GlobData.Game_state.avail_race_q, hThradd);
    UnlockLink(&raw mut GlobData.Game_state.avail_race_q, hIlwrath);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn advance_mycon_mission(
    mut arg: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut hMycon: HFLEETINFO = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut MyconPtr: *mut FLEET_INFO = ::core::ptr::null_mut::<FLEET_INFO>();
    hMycon = GetStarShipFromIndex(
        &raw mut GlobData.Game_state.avail_race_q,
        RACE_ID::MYCON_SHIP.0 as ::core::ffi::c_int as COUNT,
    ) as HFLEETINFO;
    MyconPtr = LockFleetInfo(&raw mut GlobData.Game_state.avail_race_q, hMycon);
    let mut home: POINT = SeedFleetLocation(
        MyconPtr,
        &raw mut plot_map as *mut PLOT_LOCATION,
        0 as COUNT,
    );
    let mut trap: POINT = SeedFleetLocation(
        MyconPtr,
        &raw mut plot_map as *mut PLOT_LOCATION,
        C2Rust_Unnamed_3::MYCON_TRAP_DEFINED.0 as ::core::ffi::c_int as COUNT,
    );
    if (*MyconPtr).actual_strength != 0 {
        if (*MyconPtr).growth != 0 {
            uqm_lua_checked!(setGameStateUint(
                b"MYCON_KNOW_AMBUSH\0".as_ptr() as *const ::core::ffi::c_char,
                1 as DWORD,
            ), 0);
            SetRaceDest(
                RACE_ID::MYCON_SHIP.0 as ::core::ffi::c_int as BYTE,
                home.x,
                home.y,
                30 as BYTE,
                !(0 as ::core::ffi::c_int) as BYTE,
            );
            if if GlobData.SIS_state.Extended as ::core::ffi::c_int != 0 {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                let mut hSyreen: HFLEETINFO = ::core::ptr::null_mut::<
                    ::core::ffi::c_void,
                >();
                let mut SyreenPtr: *mut FLEET_INFO = ::core::ptr::null_mut::<
                    FLEET_INFO,
                >();
                hSyreen = GetStarShipFromIndex(
                    &raw mut GlobData.Game_state.avail_race_q,
                    RACE_ID::SYREEN_SHIP.0 as ::core::ffi::c_int as COUNT,
                ) as HFLEETINFO;
                SyreenPtr = LockFleetInfo(
                    &raw mut GlobData.Game_state.avail_race_q,
                    hSyreen,
                );
                if !SyreenPtr.is_null() {
                    let mut syra: POINT = SeedFleetLocation(
                        SyreenPtr,
                        &raw mut plot_map as *mut PLOT_LOCATION,
                        0 as COUNT,
                    );
                    SetRaceDest(
                        RACE_ID::SYREEN_SHIP.0 as ::core::ffi::c_int as BYTE,
                        syra.x,
                        syra.y,
                        30 as BYTE,
                        !(0 as ::core::ffi::c_int) as BYTE,
                    );
                    (*SyreenPtr).growth = 0 as BYTE;
                    (*SyreenPtr).growth_fract = 0 as BYTE;
                }
            }
            (*MyconPtr).growth = 0 as BYTE;
            (*MyconPtr).growth_fract = 0 as BYTE;
        } else if (*MyconPtr).loc.x as ::core::ffi::c_int != trap.x as ::core::ffi::c_int
            || (*MyconPtr).loc.y as ::core::ffi::c_int != trap.y as ::core::ffi::c_int
        {
            SetRaceDest(
                RACE_ID::MYCON_SHIP.0 as ::core::ffi::c_int as BYTE,
                trap.x,
                trap.y,
                30 as BYTE,
                C2Rust_Unnamed::ADVANCE_MYCON_MISSION.0 as ::core::ffi::c_int as BYTE,
            );
        } else {
            let mut strength_loss: SIZE = 0;
            uqm_lua_checked!(AddEvent(
                EVENT_TYPE::RELATIVE_EVENT,
                0 as COUNT,
                14 as COUNT,
                0 as COUNT,
                C2Rust_Unnamed::ADVANCE_MYCON_MISSION.0 as ::core::ffi::c_int as BYTE,
            ), 0);
            strength_loss = ((*MyconPtr).actual_strength as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int) as SIZE;
            (*MyconPtr).growth = (-(strength_loss as ::core::ffi::c_int)
                / 14 as ::core::ffi::c_int) as BYTE;
            (*MyconPtr).growth_fract = (((strength_loss as ::core::ffi::c_int
                % 14 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
                / 14 as ::core::ffi::c_int) as BYTE;
            (*MyconPtr).growth_err_term = (255 as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int) as BYTE;
            if if GlobData.SIS_state.Extended as ::core::ffi::c_int != 0 {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                let mut hSyreen_0: HFLEETINFO = ::core::ptr::null_mut::<
                    ::core::ffi::c_void,
                >();
                let mut SyreenPtr_0: *mut FLEET_INFO = ::core::ptr::null_mut::<
                    FLEET_INFO,
                >();
                hSyreen_0 = GetStarShipFromIndex(
                    &raw mut GlobData.Game_state.avail_race_q,
                    RACE_ID::SYREEN_SHIP.0 as ::core::ffi::c_int as COUNT,
                ) as HFLEETINFO;
                SyreenPtr_0 = LockFleetInfo(
                    &raw mut GlobData.Game_state.avail_race_q,
                    hSyreen_0,
                );
                if !SyreenPtr_0.is_null() {
                    (*SyreenPtr_0).growth = (strength_loss as ::core::ffi::c_int
                        / 28 as ::core::ffi::c_int) as BYTE;
                    (*SyreenPtr_0).growth_fract = (((strength_loss as ::core::ffi::c_int
                        % 28 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
                        / 28 as ::core::ffi::c_int) as BYTE;
                }
            }
        }
    }
    UnlockLink(&raw mut GlobData.Game_state.avail_race_q, hMycon);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn arilou_umgah_check(
    mut arg: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    uqm_lua_checked!(setGameStateUint(
        b"ARILOU_CHECKED_UMGAH\0".as_ptr() as *const ::core::ffi::c_char,
        2 as DWORD,
    ), 0);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn yehat_rebel_event(
    mut arg: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut hRebel: HFLEETINFO = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut hRoyalist: HFLEETINFO = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut RebelPtr: *mut FLEET_INFO = ::core::ptr::null_mut::<FLEET_INFO>();
    let mut RoyalistPtr: *mut FLEET_INFO = ::core::ptr::null_mut::<FLEET_INFO>();
    hRebel = GetStarShipFromIndex(
        &raw mut GlobData.Game_state.avail_race_q,
        RACE_ID::YEHAT_REBEL_SHIP.0 as ::core::ffi::c_int as COUNT,
    ) as HFLEETINFO;
    RebelPtr = LockFleetInfo(&raw mut GlobData.Game_state.avail_race_q, hRebel);
    hRoyalist = GetStarShipFromIndex(
        &raw mut GlobData.Game_state.avail_race_q,
        RACE_ID::YEHAT_SHIP.0 as ::core::ffi::c_int as COUNT,
    ) as HFLEETINFO;
    RoyalistPtr = LockFleetInfo(&raw mut GlobData.Game_state.avail_race_q, hRoyalist);
    (*RoyalistPtr).actual_strength = ((*RoyalistPtr).actual_strength
        as ::core::ffi::c_int * 2 as ::core::ffi::c_int / 3 as ::core::ffi::c_int)
        as COUNT;
    (*RebelPtr).actual_strength = (*RoyalistPtr).actual_strength;
    (*RebelPtr).loc = SeedFleetLocation(
        RebelPtr,
        &raw mut plot_map as *mut PLOT_LOCATION,
        C2Rust_Unnamed_3::YEHAT_DEFINED.0 as ::core::ffi::c_int as COUNT,
    );
    UnlockLink(&raw mut GlobData.Game_state.avail_race_q, hRoyalist);
    UnlockLink(&raw mut GlobData.Game_state.avail_race_q, hRebel);
    StartSphereTracking(RACE_ID::YEHAT_REBEL_SHIP);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn slylandro_ramp_up(
    mut arg: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if uqm_lua_checked!(getGameStateUint(
        b"DESTRUCT_CODE_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
    ), 0) == 0
    {
        let mut ramp_factor: BYTE = 0;
        ramp_factor = uqm_lua_checked!(getGameStateUint(
            b"SLYLANDRO_MULTIPLIER\0".as_ptr() as *const ::core::ffi::c_char,
        ), 0) as BYTE;
        ramp_factor = ramp_factor.wrapping_add(1);
        if ramp_factor as ::core::ffi::c_int <= 4 as ::core::ffi::c_int {
            uqm_lua_checked!(setGameStateUint(
                b"SLYLANDRO_MULTIPLIER\0".as_ptr() as *const ::core::ffi::c_char,
                ramp_factor as DWORD,
            ), 0);
            uqm_lua_checked!(AddEvent(
                EVENT_TYPE::RELATIVE_EVENT,
                0 as COUNT,
                182 as COUNT,
                0 as COUNT,
                C2Rust_Unnamed::SLYLANDRO_RAMP_UP.0 as ::core::ffi::c_int as BYTE,
            ), 0);
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn slylandro_ramp_down(
    mut arg: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ramp_factor: BYTE = 0;
    ramp_factor = uqm_lua_checked!(getGameStateUint(
        b"SLYLANDRO_MULTIPLIER\0".as_ptr() as *const ::core::ffi::c_char,
    ), 0) as BYTE;
    ramp_factor = ramp_factor.wrapping_sub(1);
    if ramp_factor != 0 {
        uqm_lua_checked!(AddEvent(
            EVENT_TYPE::RELATIVE_EVENT,
            0 as COUNT,
            23 as COUNT,
            0 as COUNT,
            C2Rust_Unnamed::SLYLANDRO_RAMP_DOWN.0 as ::core::ffi::c_int as BYTE,
        ), 0);
    }
    uqm_lua_checked!(setGameStateUint(
        b"SLYLANDRO_MULTIPLIER\0".as_ptr() as *const ::core::ffi::c_char,
        ramp_factor as DWORD,
    ), 0);
    return 0 as ::core::ffi::c_int;
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
                410 as ::core::ffi::c_int,
                b"LockFleetInfo\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    return LockLink(pq, h) as *mut FLEET_INFO;
}
