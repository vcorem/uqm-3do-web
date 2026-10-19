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

// Rust translation of src/uqm/build.c.
extern "C" {
    pub type _IO_FILE;
    pub type string_table_entry;
    pub type frame_desc;
    pub type tfb_soundsample;
    pub type RandomContext;
    fn sqrt(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn __assert_fail(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    ) -> !;
    static stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn snprintf(
        _: *mut ::core::ffi::c_char,
        _: size_t,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
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
    fn memset(
        _: *mut ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strcmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    static mut optInfiniteRU: OPT_ENABLABLE;
    static mut oldRU: DWORD;
    static mut optInfiniteFuel: OPT_ENABLABLE;
    static mut loadFuel: DWORD;
    static mut optSeedType: ::core::ffi::c_int;
    static mut optCustomSeed: ::core::ffi::c_int;
    static mut optShipSeed: OPT_ENABLABLE;
    static mut optFleetPointSys: OPT_ENABLABLE;
    static mut optShipStore: OPT_ENABLABLE;
    static mut optDeviceArray: [OPT_ADD_REMOVE; 28];
    static mut optUpgradeArray: [OPT_ADD_REMOVE; 13];
    fn AllocLink(pq: *mut QUEUE) -> HLINK;
    fn FreeLink(pq: *mut QUEUE, hLink: HLINK);
    fn PutQueue(pq: *mut QUEUE, hLink: HLINK);
    fn InsertQueue(pq: *mut QUEUE, hLink: HLINK, hRefLink: HLINK);
    fn RemoveQueue(pq: *mut QUEUE, hLink: HLINK);
    fn CountLinks(pq: *mut QUEUE) -> COUNT;
    fn DeltaSISGauges(
        crew_delta: SIZE,
        fuel_delta: SDWORD,
        resunit_delta: ::core::ffi::c_int,
    );
    fn GetFuelTankCapacity() -> DWORD;
    fn setGameStateUint(name: *const ::core::ffi::c_char, val: DWORD);
    fn getGameStateUint(name: *const ::core::ffi::c_char) -> DWORD;
    static mut GlobData: GLOBDATA;
    fn TFB_Random() -> DWORD;
    fn RandomContext_New() -> *mut RandomContext;
    fn RandomContext_Delete(context: *mut RandomContext);
    fn RandomContext_Random(context: *mut RandomContext) -> DWORD;
    fn RandomContext_SeedRandom(context: *mut RandomContext, new_seed: DWORD) -> DWORD;
    fn GetRandomSeedForStar(star: *const STAR_DESC) -> DWORD;
    static mut plot_map: [PLOT_LOCATION; 70];
    static mut StarGenRNG: *mut RandomContext;
    fn CanBuyPoints(hFleet: HFLEETINFO) -> BOOLEAN;
    fn SeedShip(SpeciesID: SPECIES_ID, loadWindow: BOOLEAN) -> SPECIES_ID;
    static mut master_q: QUEUE;
    fn ShipCost(race_id: BYTE) -> COUNT;
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
pub type FILE = _IO_FILE;
pub type RESOURCE = *const ::core::ffi::c_char;
pub type STRING_TABLE_ENTRY_DESC = string_table_entry;
pub type STRING = *mut STRING_TABLE_ENTRY_DESC;
pub type FRAME_DESC = frame_desc;
pub type FRAME = *mut FRAME_DESC;
pub type TIME_VALUE = UWORD;
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
pub struct INTERSECT_CONTROL {
    pub last_time_val: TIME_VALUE,
    pub EndPoint: POINT,
    pub IntersectStamp: STAMP,
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
pub struct C2Rust_Unnamed(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed {
    pub const OPTVAL_PRIME: Self = Self(0);
    pub const OPTVAL_PLANET: Self = Self(1);
    pub const OPTVAL_MRQ: Self = Self(2);
    pub const OPTVAL_STAR: Self = Self(3);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct OPT_ADD_REMOVE(pub ::core::ffi::c_uint);
impl OPT_ADD_REMOVE {
    pub const OPTVAL_DO_NOTHING: Self = Self(0);
    pub const OPTVAL_REM_THING: Self = Self(1);
    pub const OPTVAL_ADD_THING: Self = Self(2);
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
pub struct STARSHIP {
    pub pred: HLINK,
    pub succ: HLINK,
    pub SpeciesID: SPECIES_ID,
    pub captains_name_index: BYTE,
    pub RaceDescPtr: *mut RACE_DESC,
    pub crew_level: COUNT,
    pub max_crew: COUNT,
    pub ship_cost: BYTE,
    pub index: COUNT,
    pub race_strings: STRING,
    pub icons: FRAME,
    pub weapon_counter: BYTE,
    pub special_counter: BYTE,
    pub energy_counter: BYTE,
    pub ship_input_state: BYTE,
    pub cur_status_flags: STATUS_FLAGS,
    pub old_status_flags: STATUS_FLAGS,
    pub hShip: HELEMENT,
    pub ShipFacing: COUNT,
    pub playerNr: SIZE,
    pub control: BYTE,
}
pub type HELEMENT = HLINK;
pub type STATUS_FLAGS = UWORD;
pub type RACE_DESC = race_desc;
#[derive(Copy, Clone)]
#[repr(C, align(16))]
pub struct race_desc {
    pub ship_info: SHIP_INFO,
    pub c2rust_padding: [u8; 8],
    pub fleet: FLEET_STUFF,
    pub c2rust_padding_0: [u8; 10],
    pub characteristics: CHARACTERISTIC_STUFF,
    pub c2rust_padding_1: [u8; 2],
    pub ship_data: DATA_STUFF,
    pub c2rust_padding_2: [u8; 8],
    pub cyborg_control: INTEL_STUFF,
    pub uninit_func: Option<UNINIT_FUNC>,
    pub preprocess_func: Option<PREPROCESS_FUNC>,
    pub postprocess_func: Option<POSTPROCESS_FUNC>,
    pub init_weapon_func: Option<INIT_WEAPON_FUNC>,
    pub data: *mut ::core::ffi::c_void,
    pub CodeRef: *mut ::core::ffi::c_void,
}
pub type INIT_WEAPON_FUNC = unsafe extern "C" fn(*mut ELEMENT, *mut HELEMENT) -> COUNT;
pub type ELEMENT = element;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct element {
    pub pred: HELEMENT,
    pub succ: HELEMENT,
    pub preprocess_func: Option<ElementProcessFunc>,
    pub postprocess_func: Option<ElementProcessFunc>,
    pub collision_func: Option<ElementCollisionFunc>,
    pub death_func: Option<ElementProcessFunc>,
    pub playerNr: SIZE,
    pub state_flags: ELEMENT_FLAGS,
    pub c2rust_unnamed: C2Rust_Unnamed_5,
    pub c2rust_unnamed_0: C2Rust_Unnamed_4,
    pub c2rust_unnamed_1: C2Rust_Unnamed_3,
    pub c2rust_unnamed_2: C2Rust_Unnamed_2,
    pub c2rust_unnamed_3: C2Rust_Unnamed_1,
    pub colorCycleIndex: BYTE,
    pub velocity: VELOCITY_DESC,
    pub IntersectControl: INTERSECT_CONTROL,
    pub PrimIndex: COUNT,
    pub current: STATE,
    pub next: STATE,
    pub pParent: *mut ::core::ffi::c_void,
    pub hTarget: HELEMENT,
}
pub type STATE = state;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct state {
    pub location: POINT,
    pub image: C2Rust_Unnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_0 {
    pub frame: FRAME,
    pub farray: *mut FRAME,
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
pub union C2Rust_Unnamed_1 {
    pub thrust_wait: BYTE,
    pub blast_offset: BYTE,
    pub next_turn: BYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_2 {
    pub turn_wait: BYTE,
    pub sys_loc: BYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_3 {
    pub mass_points: BYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_4 {
    pub crew_level: COUNT,
    pub hit_points: COUNT,
    pub facing: COUNT,
    pub cycle: COUNT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_5 {
    pub life_span: COUNT,
    pub scan_node: COUNT,
}
pub type ELEMENT_FLAGS = UWORD;
pub type ElementProcessFunc = unsafe extern "C" fn(*mut ELEMENT) -> ();
pub type ElementCollisionFunc = unsafe extern "C" fn(
    *mut ELEMENT,
    *mut POINT,
    *mut ELEMENT,
    *mut POINT,
) -> ();
pub type POSTPROCESS_FUNC = unsafe extern "C" fn(*mut ELEMENT) -> ();
pub type PREPROCESS_FUNC = unsafe extern "C" fn(*mut ELEMENT) -> ();
pub type UNINIT_FUNC = unsafe extern "C" fn(*mut RACE_DESC) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct INTEL_STUFF {
    pub ManeuverabilityIndex: COUNT,
    pub WeaponRange: COUNT,
    pub intelligence_func: Option<IntelligenceFunc>,
}
pub type IntelligenceFunc = unsafe extern "C" fn(
    *mut ELEMENT,
    *mut EVALUATE_DESC,
    COUNT,
) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EVALUATE_DESC {
    pub ObjectPtr: *mut ELEMENT,
    pub facing: COUNT,
    pub which_turn: COUNT,
    pub MoveState: MOVEMENT_STATE,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct MOVEMENT_STATE(pub ::core::ffi::c_uint);
impl MOVEMENT_STATE {
    pub const PURSUE: Self = Self(0);
    pub const AVOID: Self = Self(1);
    pub const ENTICE: Self = Self(2);
    pub const NO_MOVEMENT: Self = Self(3);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DATA_STUFF {
    pub ship_rsc: [RESOURCE; 3],
    pub weapon_rsc: [RESOURCE; 3],
    pub special_rsc: [RESOURCE; 3],
    pub captain_control: CAPTAIN_STUFF,
    pub victory_ditty_rsc: RESOURCE,
    pub ship_sounds_rsc: RESOURCE,
    pub ship: [FRAME; 3],
    pub weapon: [FRAME; 3],
    pub special: [FRAME; 3],
    pub victory_ditty: MUSIC_REF,
    pub ship_sounds: SOUND,
}
pub type SOUND = STRING;
pub type MUSIC_REF = *mut *mut TFB_SoundSample;
pub type TFB_SoundSample = tfb_soundsample;
pub type CAPTAIN_STUFF = captain_stuff;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct captain_stuff {
    pub captain_rsc: RESOURCE,
    pub background: FRAME,
    pub turn: FRAME,
    pub thrust: FRAME,
    pub weapon: FRAME,
    pub special: FRAME,
    pub tl_offset: BYTE,
    pub tr_offset: BYTE,
    pub thrust_offset: BYTE,
    pub weapon_offset: BYTE,
    pub special_offset: BYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CHARACTERISTIC_STUFF {
    pub max_thrust: COUNT,
    pub thrust_increment: COUNT,
    pub energy_regeneration: BYTE,
    pub weapon_energy_cost: BYTE,
    pub special_energy_cost: BYTE,
    pub energy_wait: BYTE,
    pub turn_wait: BYTE,
    pub thrust_wait: BYTE,
    pub weapon_wait: BYTE,
    pub special_wait: BYTE,
    pub ship_mass: BYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FLEET_STUFF {
    pub strength: COUNT,
    pub known_loc: POINT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SHIP_INFO {
    pub idStr: *const ::core::ffi::c_char,
    pub ship_flags: UWORD,
    pub ship_cost: BYTE,
    pub crew_level: COUNT,
    pub max_crew: COUNT,
    pub energy_level: BYTE,
    pub max_energy: BYTE,
    pub race_strings_rsc: RESOURCE,
    pub icons_rsc: RESOURCE,
    pub melee_icon_rsc: RESOURCE,
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
pub struct SHIP_BASE {
    pub pred: HLINK,
    pub succ: HLINK,
    pub SpeciesID: SPECIES_ID,
    pub captains_name_index: BYTE,
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
pub struct C2Rust_Unnamed_6(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_6 {
    pub const DEAD_GUY: Self = Self(0);
    pub const GOOD_GUY: Self = Self(1);
    pub const BAD_GUY: Self = Self(2);
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
pub struct C2Rust_Unnamed_7(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_7 {
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_8(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_8 {
    pub const BIOLOGICAL_DISASTER: Self = Self(0);
    pub const EARTHQUAKE_DISASTER: Self = Self(1);
    pub const LIGHTNING_DISASTER: Self = Self(2);
    pub const LAVASPOT_DISASTER: Self = Self(3);
    pub const LANDER_INJURED: Self = Self(4);
    pub const LANDER_SHOOTS: Self = Self(5);
    pub const LANDER_HITS: Self = Self(6);
    pub const LIFEFORM_CANNED: Self = Self(7);
    pub const LANDER_PICKUP: Self = Self(8);
    pub const LANDER_FULL: Self = Self(9);
    pub const LANDER_DEPARTS: Self = Self(10);
    pub const LANDER_RETURNS: Self = Self(11);
    pub const LANDER_DESTROYED: Self = Self(12);
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_9(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_9 {
    pub const DWARF_STAR: Self = Self(0);
    pub const GIANT_STAR: Self = Self(1);
    pub const SUPER_GIANT_STAR: Self = Self(2);
    pub const NUM_STAR_TYPES: Self = Self(3);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_10(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_10 {
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
pub struct C2Rust_Unnamed_11(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_11 {
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
pub type HMASTERSHIP = HLINK;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MASTER_SHIP_INFO {
    pub pred: HMASTERSHIP,
    pub succ: HMASTERSHIP,
    pub SpeciesID: SPECIES_ID,
    pub ShipInfo: SHIP_INFO,
    pub Fleet: FLEET_STUFF,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RaceIdMap {
    pub idStr: *const ::core::ffi::c_char,
    pub id: RACE_ID,
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
#[no_mangle]
pub unsafe extern "C" fn Build(
    mut pQueue: *mut QUEUE,
    mut SpeciesID: SPECIES_ID,
) -> HLINK {
    let mut hNewShip: HLINK = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut ShipPtr: *mut SHIP_BASE = ::core::ptr::null_mut::<SHIP_BASE>();
    ((*pQueue).object_size as usize == ::core::mem::size_of::<STARSHIP>()
        || (*pQueue).object_size as usize == ::core::mem::size_of::<SHIP_FRAGMENT>()
        || {
            __assert_fail(
                b"GetLinkSize (pQueue) == sizeof (STARSHIP) || GetLinkSize (pQueue) == sizeof (SHIP_FRAGMENT)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"src/uqm/build.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                43 as ::core::ffi::c_int,
                b"Build\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    hNewShip = AllocLink(pQueue);
    if hNewShip.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    ShipPtr = LockLink(pQueue, hNewShip) as *mut SHIP_BASE;
    memset(
        ShipPtr as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (*pQueue).object_size as size_t,
    );
    (*ShipPtr).SpeciesID = SpeciesID;
    UnlockLink(pQueue, hNewShip);
    PutQueue(pQueue, hNewShip);
    return hNewShip;
}
#[no_mangle]
pub unsafe extern "C" fn GetStarShipFromIndex(
    mut pShipQ: *mut QUEUE,
    mut Index: COUNT,
) -> HLINK {
    let mut hStarShip: HLINK = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut hNextShip: HLINK = ::core::ptr::null_mut::<::core::ffi::c_void>();
    hStarShip = (*pShipQ).head;
    while Index as ::core::ffi::c_int > 0 as ::core::ffi::c_int && !hStarShip.is_null() {
        let mut StarShipPtr: *mut LINK = ::core::ptr::null_mut::<LINK>();
        StarShipPtr = LockLink(pShipQ, hStarShip);
        hNextShip = (*StarShipPtr).succ;
        UnlockLink(pShipQ, hStarShip);
        hStarShip = hNextShip;
        Index = Index.wrapping_sub(1);
    }
    return hStarShip;
}
#[no_mangle]
pub unsafe extern "C" fn GetFleetFromSpecies(mut id: SPECIES_ID) -> HLINK {
    let mut hFleet: HLINK = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut hNextFleet: HLINK = ::core::ptr::null_mut::<::core::ffi::c_void>();
    hFleet = GlobData.Game_state.avail_race_q.head;
    while !hFleet.is_null() {
        let mut FleetPtr: *mut FLEET_INFO = ::core::ptr::null_mut::<FLEET_INFO>();
        FleetPtr = LockFleetInfo(&raw mut GlobData.Game_state.avail_race_q, hFleet);
        if (*FleetPtr).SpeciesID.0 == id.0 {
            UnlockLink(&raw mut GlobData.Game_state.avail_race_q, hFleet);
            return hFleet;
        }
        hNextFleet = (*FleetPtr).succ as HLINK;
        UnlockLink(&raw mut GlobData.Game_state.avail_race_q, hFleet);
        hFleet = hNextFleet;
    }
    return hFleet;
}
#[no_mangle]
pub unsafe extern "C" fn GetSeededFleetFromIndex(mut Index: COUNT) -> HFLEETINFO {
    let mut TemplatePtr: *mut FLEET_INFO = ::core::ptr::null_mut::<FLEET_INFO>();
    let mut hFleet: HFLEETINFO = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut ship: SPECIES_ID = SPECIES_ID::NO_ID;
    let mut loadWindow: BOOLEAN = BOOLEAN(
        (optShipSeed.0 != 0
            && GlobData.SIS_state.ShipSeed as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
            || optShipSeed.0 == 0
                && GlobData.SIS_state.ShipSeed as ::core::ffi::c_int
                    != 0 as ::core::ffi::c_int
            || optCustomSeed != GlobData.SIS_state.Seed) as ::core::ffi::c_int
            as ::core::ffi::c_uint,
    );
    hFleet = GetStarShipFromIndex(&raw mut GlobData.Game_state.avail_race_q, Index)
        as HFLEETINFO;
    if hFleet.is_null() {
        return hFleet;
    }
    TemplatePtr = LockFleetInfo(&raw mut GlobData.Game_state.avail_race_q, hFleet);
    if TemplatePtr.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    ship = SeedShip((*TemplatePtr).SpeciesID, loadWindow);
    UnlockLink(&raw mut GlobData.Game_state.avail_race_q, hFleet);
    hFleet = GetFleetFromSpecies(ship) as HFLEETINFO;
    return hFleet;
}
#[no_mangle]
pub unsafe extern "C" fn GetEscortByStarShipIndex(mut index: COUNT) -> HSHIPFRAG {
    let mut hStarShip: HSHIPFRAG = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut hNextShip: HSHIPFRAG = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut StarShipPtr: *mut SHIP_FRAGMENT = ::core::ptr::null_mut::<SHIP_FRAGMENT>();
    hStarShip = GlobData.Game_state.built_ship_q.head as HSHIPFRAG;
    while !hStarShip.is_null() {
        StarShipPtr = LockShipFrag(&raw mut GlobData.Game_state.built_ship_q, hStarShip);
        if (*StarShipPtr).index as ::core::ffi::c_int == index as ::core::ffi::c_int {
            UnlockLink(&raw mut GlobData.Game_state.built_ship_q, hStarShip);
            break;
        } else {
            hNextShip = (*StarShipPtr).succ as HSHIPFRAG;
            UnlockLink(&raw mut GlobData.Game_state.built_ship_q, hStarShip);
            hStarShip = hNextShip;
        }
    }
    return hStarShip;
}
#[no_mangle]
pub unsafe extern "C" fn ShipIdStrToIndex(
    mut shipIdStr: *const ::core::ffi::c_char,
) -> SPECIES_ID {
    let mut hStarShip: HMASTERSHIP = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut hNextShip: HMASTERSHIP = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut result: SPECIES_ID = SPECIES_ID::NO_ID;
    hStarShip = master_q.head as HMASTERSHIP;
    while !hStarShip.is_null() {
        let mut MasterPtr: *mut MASTER_SHIP_INFO = ::core::ptr::null_mut::<
            MASTER_SHIP_INFO,
        >();
        MasterPtr = LockMasterShip(&raw mut master_q, hStarShip);
        hNextShip = (*MasterPtr).succ;
        if strcmp(shipIdStr, (*MasterPtr).ShipInfo.idStr) == 0 as ::core::ffi::c_int {
            result = (*MasterPtr).SpeciesID;
            UnlockLink(&raw mut master_q, hStarShip);
            break;
        } else {
            UnlockLink(&raw mut master_q, hStarShip);
            hStarShip = hNextShip;
        }
    }
    return result;
}
static mut raceIdMap: [RaceIdMap; 28] = [
    RaceIdMap {
        idStr: b"androsynth\0".as_ptr() as *const ::core::ffi::c_char,
        id: RACE_ID::ANDROSYNTH_SHIP,
    },
    RaceIdMap {
        idStr: b"arilou\0".as_ptr() as *const ::core::ffi::c_char,
        id: RACE_ID::ARILOU_SHIP,
    },
    RaceIdMap {
        idStr: b"chenjesu\0".as_ptr() as *const ::core::ffi::c_char,
        id: RACE_ID::CHENJESU_SHIP,
    },
    RaceIdMap {
        idStr: b"chmmr\0".as_ptr() as *const ::core::ffi::c_char,
        id: RACE_ID::CHMMR_SHIP,
    },
    RaceIdMap {
        idStr: b"druuge\0".as_ptr() as *const ::core::ffi::c_char,
        id: RACE_ID::DRUUGE_SHIP,
    },
    RaceIdMap {
        idStr: b"human\0".as_ptr() as *const ::core::ffi::c_char,
        id: RACE_ID::HUMAN_SHIP,
    },
    RaceIdMap {
        idStr: b"ilwrath\0".as_ptr() as *const ::core::ffi::c_char,
        id: RACE_ID::ILWRATH_SHIP,
    },
    RaceIdMap {
        idStr: b"kohrah\0".as_ptr() as *const ::core::ffi::c_char,
        id: RACE_ID::BLACK_URQUAN_SHIP,
    },
    RaceIdMap {
        idStr: b"melnorme\0".as_ptr() as *const ::core::ffi::c_char,
        id: RACE_ID::MELNORME_SHIP,
    },
    RaceIdMap {
        idStr: b"mmrnmhrm\0".as_ptr() as *const ::core::ffi::c_char,
        id: RACE_ID::MMRNMHRM_SHIP,
    },
    RaceIdMap {
        idStr: b"mycon\0".as_ptr() as *const ::core::ffi::c_char,
        id: RACE_ID::MYCON_SHIP,
    },
    RaceIdMap {
        idStr: b"orz\0".as_ptr() as *const ::core::ffi::c_char,
        id: RACE_ID::ORZ_SHIP,
    },
    RaceIdMap {
        idStr: b"pkunk\0".as_ptr() as *const ::core::ffi::c_char,
        id: RACE_ID::PKUNK_SHIP,
    },
    RaceIdMap {
        idStr: b"samatra\0".as_ptr() as *const ::core::ffi::c_char,
        id: RACE_ID::SAMATRA_SHIP,
    },
    RaceIdMap {
        idStr: b"shofixti\0".as_ptr() as *const ::core::ffi::c_char,
        id: RACE_ID::SHOFIXTI_SHIP,
    },
    RaceIdMap {
        idStr: b"slylandro\0".as_ptr() as *const ::core::ffi::c_char,
        id: RACE_ID::SLYLANDRO_SHIP,
    },
    RaceIdMap {
        idStr: b"spathi\0".as_ptr() as *const ::core::ffi::c_char,
        id: RACE_ID::SPATHI_SHIP,
    },
    RaceIdMap {
        idStr: b"supox\0".as_ptr() as *const ::core::ffi::c_char,
        id: RACE_ID::SUPOX_SHIP,
    },
    RaceIdMap {
        idStr: b"syreen\0".as_ptr() as *const ::core::ffi::c_char,
        id: RACE_ID::SYREEN_SHIP,
    },
    RaceIdMap {
        idStr: b"thraddash\0".as_ptr() as *const ::core::ffi::c_char,
        id: RACE_ID::THRADDASH_SHIP,
    },
    RaceIdMap {
        idStr: b"umgah\0".as_ptr() as *const ::core::ffi::c_char,
        id: RACE_ID::UMGAH_SHIP,
    },
    RaceIdMap {
        idStr: b"urquandrone\0".as_ptr() as *const ::core::ffi::c_char,
        id: RACE_ID::URQUAN_DRONE_SHIP,
    },
    RaceIdMap {
        idStr: b"urquan\0".as_ptr() as *const ::core::ffi::c_char,
        id: RACE_ID::URQUAN_SHIP,
    },
    RaceIdMap {
        idStr: b"utwig\0".as_ptr() as *const ::core::ffi::c_char,
        id: RACE_ID::UTWIG_SHIP,
    },
    RaceIdMap {
        idStr: b"vux\0".as_ptr() as *const ::core::ffi::c_char,
        id: RACE_ID::VUX_SHIP,
    },
    RaceIdMap {
        idStr: b"yehat\0".as_ptr() as *const ::core::ffi::c_char,
        id: RACE_ID::YEHAT_SHIP,
    },
    RaceIdMap {
        idStr: b"yehatrebel\0".as_ptr() as *const ::core::ffi::c_char,
        id: RACE_ID::YEHAT_REBEL_SHIP,
    },
    RaceIdMap {
        idStr: b"zoqfotpik\0".as_ptr() as *const ::core::ffi::c_char,
        id: RACE_ID::ZOQFOTPIK_SHIP,
    },
];
unsafe extern "C" fn RaceIdCompare(
    mut id1: *const ::core::ffi::c_void,
    mut id2: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return strcmp((*(id1 as *mut RaceIdMap)).idStr, (*(id2 as *mut RaceIdMap)).idStr);
}
#[no_mangle]
pub unsafe extern "C" fn RaceIdStrToIndex(
    mut raceIdStr: *const ::core::ffi::c_char,
) -> RACE_ID {
    let mut key: RaceIdMap = RaceIdMap {
        idStr: raceIdStr,
        id: RACE_ID(-1 as ::core::ffi::c_int as ::core::ffi::c_uint),
    };
    let mut found: *mut RaceIdMap = bsearch(
        &raw mut key as *const ::core::ffi::c_void,
        &raw mut raceIdMap as *mut RaceIdMap as *const ::core::ffi::c_void,
        ::core::mem::size_of::<[RaceIdMap; 28]>()
            .wrapping_div(::core::mem::size_of::<RaceIdMap>()),
        ::core::mem::size_of::<RaceIdMap>(),
        Some(
            RaceIdCompare
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_void,
                    *const ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
    ) as *mut RaceIdMap;
    if found.is_null() {
        return RACE_ID(-1 as ::core::ffi::c_int as ::core::ffi::c_uint);
    }
    return (*found).id;
}
#[no_mangle]
pub unsafe extern "C" fn AddEscortShips(mut race: RACE_ID, mut count: SIZE) -> COUNT {
    let mut hFleet: HFLEETINFO = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut which_window: BYTE = 0;
    let mut i: COUNT = 0;
    hFleet = GetStarShipFromIndex(
        &raw mut GlobData.Game_state.avail_race_q,
        race.0 as COUNT,
    ) as HFLEETINFO;
    if hFleet.is_null() || count as ::core::ffi::c_int <= 0 as ::core::ffi::c_int {
        return 0 as COUNT;
    }
    which_window = 0 as BYTE;
    i = 0 as COUNT;
    's_102: while (i as ::core::ffi::c_int) < count as COUNT as ::core::ffi::c_int {
        let mut hStarShip: HSHIPFRAG = ::core::ptr::null_mut::<::core::ffi::c_void>();
        let mut hOldShip: HSHIPFRAG = ::core::ptr::null_mut::<::core::ffi::c_void>();
        let mut StarShipPtr: *mut SHIP_FRAGMENT = ::core::ptr::null_mut::<
            SHIP_FRAGMENT,
        >();
        's_23: {
            if !(optShipStore.0 != 0
                || if if GlobData.SIS_state.Difficulty as ::core::ffi::c_int != 0 {
                    GlobData.SIS_state.Difficulty as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                } == 2 as ::core::ffi::c_int
                {
                    1 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                } != 0 || optFleetPointSys.0 != 0)
            {
                hStarShip = CloneShipFragment(
                    race,
                    &raw mut GlobData.Game_state.built_ship_q,
                    0 as COUNT,
                );
            } else if uqm_lua_checked!(CanBuyPoints(hFleet), 0).0 == 0
                || {
                    hStarShip = CloneShipFragment(
                        race,
                        &raw mut GlobData.Game_state.built_ship_q,
                        0 as COUNT,
                    );
                    hStarShip.is_null()
                }
            {
                hStarShip = CloneShipFragment(
                    race,
                    &raw mut GlobData.Game_state.stowed_ship_q,
                    0 as COUNT,
                );
                if !hStarShip.is_null() {
                    break 's_23;
                }
            }
            if hStarShip.is_null() {
                break 's_102;
            }
            RemoveQueue(&raw mut GlobData.Game_state.built_ship_q, hStarShip);
            loop {
                let c2rust_fresh0 = which_window;
                which_window = which_window.wrapping_add(1);
                hOldShip = GetStarShipFromIndex(
                    &raw mut GlobData.Game_state.built_ship_q,
                    c2rust_fresh0 as COUNT,
                ) as HSHIPFRAG;
                if hOldShip.is_null() {
                    break;
                }
                let mut win_loc: BYTE = 0;
                StarShipPtr = LockShipFrag(
                    &raw mut GlobData.Game_state.built_ship_q,
                    hOldShip,
                );
                win_loc = (*StarShipPtr).index;
                UnlockLink(&raw mut GlobData.Game_state.built_ship_q, hOldShip);
                if which_window as ::core::ffi::c_int <= win_loc as ::core::ffi::c_int {
                    break;
                }
            }
            StarShipPtr = LockShipFrag(
                &raw mut GlobData.Game_state.built_ship_q,
                hStarShip,
            );
            (*StarShipPtr).index = (which_window as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as BYTE;
            UnlockLink(&raw mut GlobData.Game_state.built_ship_q, hStarShip);
            InsertQueue(&raw mut GlobData.Game_state.built_ship_q, hStarShip, hOldShip);
        }
        i = i.wrapping_add(1);
    }
    uqm_lua_checked!(DeltaSISGauges(0x7fff as SIZE, 0x7fff as SDWORD, 0x7fff as ::core::ffi::c_int), 0);
    return i;
}
#[no_mangle]
pub unsafe extern "C" fn CalculateEscortsWorth() -> COUNT {
    let mut total: COUNT = 0 as COUNT;
    let mut hStarShip: HSHIPFRAG = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut hNextShip: HSHIPFRAG = ::core::ptr::null_mut::<::core::ffi::c_void>();
    hStarShip = GlobData.Game_state.built_ship_q.head as HSHIPFRAG;
    while !hStarShip.is_null() {
        let mut StarShipPtr: *mut SHIP_FRAGMENT = ::core::ptr::null_mut::<
            SHIP_FRAGMENT,
        >();
        StarShipPtr = LockShipFrag(&raw mut GlobData.Game_state.built_ship_q, hStarShip);
        hNextShip = (*StarShipPtr).succ as HSHIPFRAG;
        total = (total as ::core::ffi::c_int
            + ShipCost((*StarShipPtr).race_id) as ::core::ffi::c_int) as COUNT;
        UnlockLink(&raw mut GlobData.Game_state.built_ship_q, hStarShip);
        hStarShip = hNextShip;
    }
    return total;
}
#[no_mangle]
pub unsafe extern "C" fn ShipsReady(mut race: RACE_ID) -> BOOLEAN {
    let mut i: SIZE = 0;
    let mut year: COUNT = 0;
    let mut month: COUNT = 0;
    let mut day: COUNT = 0 as COUNT;
    match race {
        RACE_ID::PKUNK_SHIP => {
            year = uqm_lua_checked!(getGameStateUint(
                b"PKUNK_SHIP_YEAR\0".as_ptr() as *const ::core::ffi::c_char,
            ), BOOLEAN(0)) as COUNT;
            month = uqm_lua_checked!(getGameStateUint(
                b"PKUNK_SHIP_MONTH\0".as_ptr() as *const ::core::ffi::c_char,
            ), BOOLEAN(0)) as COUNT;
            day = uqm_lua_checked!(getGameStateUint(
                b"PKUNK_SHIP_DAY\0".as_ptr() as *const ::core::ffi::c_char,
            ), BOOLEAN(0)) as COUNT;
        }
        RACE_ID::SUPOX_SHIP => {
            year = uqm_lua_checked!(getGameStateUint(
                b"SUPOX_SHIP_YEAR\0".as_ptr() as *const ::core::ffi::c_char,
            ), BOOLEAN(0)) as COUNT;
            month = uqm_lua_checked!(getGameStateUint(
                b"SUPOX_SHIP_MONTH\0".as_ptr() as *const ::core::ffi::c_char,
            ), BOOLEAN(0)) as COUNT;
            day = uqm_lua_checked!(getGameStateUint(
                b"SUPOX_SHIP_DAY\0".as_ptr() as *const ::core::ffi::c_char,
            ), BOOLEAN(0)) as COUNT;
        }
        RACE_ID::UTWIG_SHIP => {
            year = uqm_lua_checked!(getGameStateUint(
                b"UTWIG_SHIP_YEAR\0".as_ptr() as *const ::core::ffi::c_char,
            ), BOOLEAN(0)) as COUNT;
            month = uqm_lua_checked!(getGameStateUint(
                b"UTWIG_SHIP_MONTH\0".as_ptr() as *const ::core::ffi::c_char,
            ), BOOLEAN(0)) as COUNT;
            day = uqm_lua_checked!(getGameStateUint(
                b"UTWIG_SHIP_DAY\0".as_ptr() as *const ::core::ffi::c_char,
            ), BOOLEAN(0)) as COUNT;
        }
        RACE_ID::YEHAT_SHIP | RACE_ID::YEHAT_REBEL_SHIP => {
            year = uqm_lua_checked!(getGameStateUint(
                b"YEHAT_SHIP_YEAR\0".as_ptr() as *const ::core::ffi::c_char,
            ), BOOLEAN(0)) as COUNT;
            month = uqm_lua_checked!(getGameStateUint(
                b"YEHAT_SHIP_MONTH\0".as_ptr() as *const ::core::ffi::c_char,
            ), BOOLEAN(0)) as COUNT;
            day = uqm_lua_checked!(getGameStateUint(
                b"YEHAT_SHIP_DAY\0".as_ptr() as *const ::core::ffi::c_char,
            ), BOOLEAN(0)) as COUNT;
        }
        _ => return BOOLEAN(0 as ::core::ffi::c_int as ::core::ffi::c_uint),
    }
    i = (GlobData.Game_state.GameClock.year_index as ::core::ffi::c_int
        - 2155 as ::core::ffi::c_int - year as ::core::ffi::c_int) as SIZE;
    return BOOLEAN(
        (i as ::core::ffi::c_int > 0 as ::core::ffi::c_int
            || i as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                && {
                    i = (GlobData.Game_state.GameClock.month_index as ::core::ffi::c_int
                        - month as ::core::ffi::c_int) as SIZE;
                    i as ::core::ffi::c_int > 0 as ::core::ffi::c_int
                        || i as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                            && GlobData.Game_state.GameClock.day_index
                                as ::core::ffi::c_int > day as ::core::ffi::c_int
                }) as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
}
#[no_mangle]
pub unsafe extern "C" fn PrepareShip(mut race: RACE_ID) {
    let mut mi: BYTE = 0;
    let mut di: BYTE = 0;
    let mut yi: BYTE = 0;
    mi = GlobData.Game_state.GameClock.month_index;
    di = GlobData.Game_state.GameClock.day_index;
    if di as ::core::ffi::c_int > 28 as ::core::ffi::c_int {
        di = 28 as BYTE;
    }
    yi = ((GlobData.Game_state.GameClock.year_index as ::core::ffi::c_int
        - 2155 as ::core::ffi::c_int) as BYTE as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as BYTE;
    match race {
        RACE_ID::PKUNK_SHIP => {
            uqm_lua_checked!(setGameStateUint(
                b"PKUNK_SHIP_YEAR\0".as_ptr() as *const ::core::ffi::c_char,
                yi as DWORD,
            ), ());
            uqm_lua_checked!(setGameStateUint(
                b"PKUNK_SHIP_MONTH\0".as_ptr() as *const ::core::ffi::c_char,
                mi as DWORD,
            ), ());
            uqm_lua_checked!(setGameStateUint(
                b"PKUNK_SHIP_DAY\0".as_ptr() as *const ::core::ffi::c_char,
                di as DWORD,
            ), ());
        }
        RACE_ID::SUPOX_SHIP => {
            uqm_lua_checked!(setGameStateUint(
                b"SUPOX_SHIP_YEAR\0".as_ptr() as *const ::core::ffi::c_char,
                yi as DWORD,
            ), ());
            uqm_lua_checked!(setGameStateUint(
                b"SUPOX_SHIP_MONTH\0".as_ptr() as *const ::core::ffi::c_char,
                mi as DWORD,
            ), ());
            uqm_lua_checked!(setGameStateUint(
                b"SUPOX_SHIP_DAY\0".as_ptr() as *const ::core::ffi::c_char,
                di as DWORD,
            ), ());
        }
        RACE_ID::UTWIG_SHIP => {
            uqm_lua_checked!(setGameStateUint(
                b"UTWIG_SHIP_YEAR\0".as_ptr() as *const ::core::ffi::c_char,
                yi as DWORD,
            ), ());
            uqm_lua_checked!(setGameStateUint(
                b"UTWIG_SHIP_MONTH\0".as_ptr() as *const ::core::ffi::c_char,
                mi as DWORD,
            ), ());
            uqm_lua_checked!(setGameStateUint(
                b"UTWIG_SHIP_DAY\0".as_ptr() as *const ::core::ffi::c_char,
                di as DWORD,
            ), ());
        }
        RACE_ID::YEHAT_SHIP | RACE_ID::YEHAT_REBEL_SHIP => {
            uqm_lua_checked!(setGameStateUint(
                b"YEHAT_SHIP_YEAR\0".as_ptr() as *const ::core::ffi::c_char,
                yi as DWORD,
            ), ());
            uqm_lua_checked!(setGameStateUint(
                b"YEHAT_SHIP_MONTH\0".as_ptr() as *const ::core::ffi::c_char,
                mi as DWORD,
            ), ());
            uqm_lua_checked!(setGameStateUint(
                b"YEHAT_SHIP_DAY\0".as_ptr() as *const ::core::ffi::c_char,
                di as DWORD,
            ), ());
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn SetRaceAllied(mut race: RACE_ID, mut flag: BOOLEAN) -> BOOLEAN {
    let mut hFleet: HFLEETINFO = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut FleetPtr: *mut FLEET_INFO = ::core::ptr::null_mut::<FLEET_INFO>();
    hFleet = GetStarShipFromIndex(
        &raw mut GlobData.Game_state.avail_race_q,
        race.0 as COUNT,
    ) as HFLEETINFO;
    if hFleet.is_null() {
        return BOOLEAN::FALSE;
    }
    FleetPtr = LockFleetInfo(&raw mut GlobData.Game_state.avail_race_q, hFleet);
    if (*FleetPtr).allied_state as ::core::ffi::c_int
        != C2Rust_Unnamed_6::DEAD_GUY.0 as ::core::ffi::c_int
    {
        (*FleetPtr).allied_state = (if flag.0 != 0 {
            C2Rust_Unnamed_6::GOOD_GUY.0 as ::core::ffi::c_int
        } else {
            C2Rust_Unnamed_6::BAD_GUY.0 as ::core::ffi::c_int
        }) as UWORD;
    }
    UnlockLink(&raw mut GlobData.Game_state.avail_race_q, hFleet);
    return BOOLEAN::TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn StartSphereTracking(mut race: RACE_ID) -> COUNT {
    let mut hFleet: HFLEETINFO = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut FleetPtr: *mut FLEET_INFO = ::core::ptr::null_mut::<FLEET_INFO>();
    hFleet = GetStarShipFromIndex(
        &raw mut GlobData.Game_state.avail_race_q,
        race.0 as COUNT,
    ) as HFLEETINFO;
    if hFleet.is_null() {
        return 0 as COUNT;
    }
    FleetPtr = LockFleetInfo(&raw mut GlobData.Game_state.avail_race_q, hFleet);
    if (*FleetPtr).actual_strength as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        if (*FleetPtr).allied_state as ::core::ffi::c_int
            == C2Rust_Unnamed_6::DEAD_GUY.0 as ::core::ffi::c_int
        {
            UnlockLink(&raw mut GlobData.Game_state.avail_race_q, hFleet);
            return 0 as COUNT;
        }
    } else if (*FleetPtr).known_strength as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        && (*FleetPtr).actual_strength as ::core::ffi::c_int
            != !(0 as ::core::ffi::c_int) as COUNT as ::core::ffi::c_int
    {
        (*FleetPtr).known_strength = 1 as COUNT;
        (*FleetPtr).known_loc = (*FleetPtr).loc;
    }
    UnlockLink(&raw mut GlobData.Game_state.avail_race_q, hFleet);
    return race.0 as COUNT;
}
#[no_mangle]
pub unsafe extern "C" fn CheckSphereTracking(mut race: RACE_ID) -> BOOLEAN {
    let mut hFleet: HFLEETINFO = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut FleetPtr: *mut FLEET_INFO = ::core::ptr::null_mut::<FLEET_INFO>();
    let mut result: COUNT = 0;
    hFleet = GetStarShipFromIndex(
        &raw mut GlobData.Game_state.avail_race_q,
        race.0 as COUNT,
    ) as HFLEETINFO;
    if hFleet.is_null() {
        return BOOLEAN::FALSE;
    }
    FleetPtr = LockFleetInfo(&raw mut GlobData.Game_state.avail_race_q, hFleet);
    if (*FleetPtr).actual_strength as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        result = BOOLEAN::FALSE.0 as ::core::ffi::c_int as COUNT;
    } else {
        result = ((*FleetPtr).known_strength as ::core::ffi::c_int
            > 0 as ::core::ffi::c_int) as ::core::ffi::c_int as COUNT;
    }
    UnlockLink(&raw mut GlobData.Game_state.avail_race_q, hFleet);
    return BOOLEAN(result as ::core::ffi::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn KillRace(mut race: RACE_ID) -> BOOLEAN {
    let mut hFleet: HFLEETINFO = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut FleetPtr: *mut FLEET_INFO = ::core::ptr::null_mut::<FLEET_INFO>();
    hFleet = GetStarShipFromIndex(
        &raw mut GlobData.Game_state.avail_race_q,
        race.0 as COUNT,
    ) as HFLEETINFO;
    if hFleet.is_null() {
        return BOOLEAN::FALSE;
    }
    FleetPtr = LockFleetInfo(&raw mut GlobData.Game_state.avail_race_q, hFleet);
    (*FleetPtr).allied_state = C2Rust_Unnamed_6::DEAD_GUY.0 as ::core::ffi::c_int
        as UWORD;
    (*FleetPtr).actual_strength = 0 as COUNT;
    UnlockLink(&raw mut GlobData.Game_state.avail_race_q, hFleet);
    return BOOLEAN::TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn CountEscortShips(mut race: RACE_ID) -> COUNT {
    let mut hFleet: HFLEETINFO = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut hStarShip: HSHIPFRAG = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut hNextShip: HSHIPFRAG = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut result: COUNT = 0 as COUNT;
    hFleet = GetStarShipFromIndex(
        &raw mut GlobData.Game_state.avail_race_q,
        race.0 as COUNT,
    ) as HFLEETINFO;
    if hFleet.is_null() {
        return 0 as COUNT;
    }
    hStarShip = GlobData.Game_state.built_ship_q.head as HSHIPFRAG;
    while !hStarShip.is_null() {
        let mut ship_type: BYTE = 0;
        let mut StarShipPtr: *mut SHIP_FRAGMENT = ::core::ptr::null_mut::<
            SHIP_FRAGMENT,
        >();
        StarShipPtr = LockShipFrag(&raw mut GlobData.Game_state.built_ship_q, hStarShip);
        hNextShip = (*StarShipPtr).succ as HSHIPFRAG;
        ship_type = (*StarShipPtr).race_id;
        UnlockLink(&raw mut GlobData.Game_state.built_ship_q, hStarShip);
        if ship_type as ::core::ffi::c_uint == race.0 {
            result = result.wrapping_add(1);
        }
        hStarShip = hNextShip;
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn HaveEscortShip(mut race: RACE_ID) -> BOOLEAN {
    return BOOLEAN(
        (CountEscortShips(race) as ::core::ffi::c_int > 0 as ::core::ffi::c_int)
            as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
}
#[no_mangle]
pub unsafe extern "C" fn EscortFeasibilityStudy(mut race: RACE_ID) -> COUNT {
    let mut hFleet: HFLEETINFO = ::core::ptr::null_mut::<::core::ffi::c_void>();
    hFleet = GetStarShipFromIndex(
        &raw mut GlobData.Game_state.avail_race_q,
        race.0 as COUNT,
    ) as HFLEETINFO;
    if hFleet.is_null() {
        return 0 as COUNT;
    }
    return (12 as ::core::ffi::c_int
        - CountLinks(&raw mut GlobData.Game_state.built_ship_q) as ::core::ffi::c_int
        + if optShipStore.0 != 0
            || if if GlobData.SIS_state.Difficulty as ::core::ffi::c_int != 0 {
                GlobData.SIS_state.Difficulty as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } == 2 as ::core::ffi::c_int
            {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } != 0 || optFleetPointSys.0 != 0
        {
            200 as ::core::ffi::c_int
                - CountLinks(&raw mut GlobData.Game_state.stowed_ship_q)
                    as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as COUNT;
}
#[no_mangle]
pub unsafe extern "C" fn CheckAlliance(mut race: RACE_ID) -> COUNT {
    let mut hFleet: HFLEETINFO = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut flags: UWORD = 0;
    let mut FleetPtr: *mut FLEET_INFO = ::core::ptr::null_mut::<FLEET_INFO>();
    hFleet = GetStarShipFromIndex(
        &raw mut GlobData.Game_state.avail_race_q,
        race.0 as COUNT,
    ) as HFLEETINFO;
    if hFleet.is_null() {
        return 0 as COUNT;
    }
    FleetPtr = LockFleetInfo(&raw mut GlobData.Game_state.avail_race_q, hFleet);
    flags = (*FleetPtr).allied_state;
    UnlockLink(&raw mut GlobData.Game_state.avail_race_q, hFleet);
    return flags;
}
#[no_mangle]
pub unsafe extern "C" fn RaceDead(mut race: RACE_ID) -> BOOLEAN {
    return BOOLEAN(
        (CheckAlliance(race) as ::core::ffi::c_int
            == C2Rust_Unnamed_6::DEAD_GUY.0 as ::core::ffi::c_int) as ::core::ffi::c_int
            as ::core::ffi::c_uint,
    );
}
#[no_mangle]
pub unsafe extern "C" fn RemoveSomeEscortShips(
    mut race: RACE_ID,
    mut count: COUNT,
) -> COUNT {
    let mut ship_q: *mut QUEUE = &raw mut GlobData.Game_state.built_ship_q;
    let mut hStarShip: HSHIPFRAG = (*ship_q).head;
    let mut hNextShip: HSHIPFRAG = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if count as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        return 0 as COUNT;
    }
    if hStarShip.is_null() && count as ::core::ffi::c_int > 12 as ::core::ffi::c_int {
        ship_q = &raw mut GlobData.Game_state.stowed_ship_q;
        hStarShip = (*ship_q).head as HSHIPFRAG;
    }
    hStarShip = (*ship_q).head as HSHIPFRAG;
    while !hStarShip.is_null() {
        let mut RemoveShip: BOOLEAN = BOOLEAN::FALSE;
        let mut StarShipPtr: *mut SHIP_FRAGMENT = ::core::ptr::null_mut::<
            SHIP_FRAGMENT,
        >();
        StarShipPtr = LockShipFrag(ship_q, hStarShip);
        hNextShip = (*StarShipPtr).succ as HSHIPFRAG;
        RemoveShip = BOOLEAN(
            ((*StarShipPtr).race_id as ::core::ffi::c_uint == race.0)
                as ::core::ffi::c_int as ::core::ffi::c_uint,
        );
        UnlockLink(ship_q, hStarShip);
        if RemoveShip.0 != 0 {
            RemoveQueue(ship_q, hStarShip);
            FreeLink(ship_q, hStarShip);
            count = count.wrapping_sub(1);
            if count as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                break;
            }
        }
        if count as ::core::ffi::c_int > 12 as ::core::ffi::c_int && hNextShip.is_null()
            && ship_q == &raw mut GlobData.Game_state.built_ship_q
        {
            ship_q = &raw mut GlobData.Game_state.stowed_ship_q;
            hNextShip = (*ship_q).head as HSHIPFRAG;
        }
        hStarShip = hNextShip;
    }
    uqm_lua_checked!(DeltaSISGauges(0x7fff as SIZE, 0x7fff as SDWORD, 0x7fff as ::core::ffi::c_int), 0);
    return count;
}
#[no_mangle]
pub unsafe extern "C" fn RemoveEscortShips(mut race: RACE_ID) -> COUNT {
    return uqm_lua_checked!(RemoveSomeEscortShips(race, -1 as ::core::ffi::c_int as COUNT), 0);
}
#[no_mangle]
pub unsafe extern "C" fn GetIndexFromStarShip(
    mut pShipQ: *mut QUEUE,
    mut hStarShip: HLINK,
) -> COUNT {
    let mut Index: COUNT = 0;
    Index = 0 as COUNT;
    while hStarShip != (*pShipQ).head {
        let mut hNextShip: HLINK = ::core::ptr::null_mut::<::core::ffi::c_void>();
        let mut StarShipPtr: *mut LINK = ::core::ptr::null_mut::<LINK>();
        StarShipPtr = LockLink(pShipQ, hStarShip);
        hNextShip = (*StarShipPtr).pred;
        UnlockLink(pShipQ, hStarShip);
        hStarShip = hNextShip;
        Index = Index.wrapping_add(1);
    }
    return Index;
}
#[no_mangle]
pub unsafe extern "C" fn NameCaptain(
    mut pQueue: *mut QUEUE,
    mut SpeciesID: SPECIES_ID,
) -> BYTE {
    let mut name_index: BYTE = 0;
    let mut hStarShip: HLINK = ::core::ptr::null_mut::<::core::ffi::c_void>();
    ((*pQueue).object_size as usize == ::core::mem::size_of::<STARSHIP>()
        || (*pQueue).object_size as usize == ::core::mem::size_of::<SHIP_FRAGMENT>()
        || {
            __assert_fail(
                b"GetLinkSize (pQueue) == sizeof (STARSHIP) || GetLinkSize (pQueue) == sizeof (SHIP_FRAGMENT)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"src/uqm/build.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                723 as ::core::ffi::c_int,
                b"NameCaptain\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    loop {
        let mut hNextShip: HLINK = ::core::ptr::null_mut::<::core::ffi::c_void>();
        name_index = ((TFB_Random() as COUNT as ::core::ffi::c_int
            & 16 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
            + (3 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)) as BYTE;
        hStarShip = (*pQueue).head;
        while !hStarShip.is_null() {
            let mut ShipPtr: *mut SHIP_BASE = ::core::ptr::null_mut::<SHIP_BASE>();
            let mut test_name_index: BYTE = -1 as ::core::ffi::c_int as BYTE;
            ShipPtr = LockLink(pQueue, hStarShip) as *mut SHIP_BASE;
            hNextShip = (*ShipPtr).succ;
            if (*ShipPtr).SpeciesID.0 == SpeciesID.0 {
                test_name_index = (*ShipPtr).captains_name_index;
            }
            UnlockLink(pQueue, hStarShip);
            if name_index as ::core::ffi::c_int == test_name_index as ::core::ffi::c_int
            {
                break;
            }
            hStarShip = hNextShip;
        }
        if hStarShip.is_null() {
            break;
        }
    }
    return name_index;
}
#[no_mangle]
pub unsafe extern "C" fn CloneShipFragment(
    mut shipIndex: RACE_ID,
    mut pDstQueue: *mut QUEUE,
    mut crew_level: COUNT,
) -> HSHIPFRAG {
    let mut hFleet: HFLEETINFO = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut hBuiltShip: HSHIPFRAG = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut TemplatePtr: *mut FLEET_INFO = ::core::ptr::null_mut::<FLEET_INFO>();
    let mut captains_name_index: BYTE = 0;
    ((*pDstQueue).object_size as usize == ::core::mem::size_of::<SHIP_FRAGMENT>()
        || {
            __assert_fail(
                b"GetLinkSize (pDstQueue) == sizeof (SHIP_FRAGMENT)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/uqm/build.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                760 as ::core::ffi::c_int,
                b"CloneShipFragment\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    if optShipSeed.0 != 0
        && GlobData.SIS_state.ShipSeed as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        || optShipSeed.0 == 0
            && GlobData.SIS_state.ShipSeed as ::core::ffi::c_int
                != 0 as ::core::ffi::c_int || optCustomSeed != GlobData.SIS_state.Seed
    {
        hFleet = GetSeededFleetFromIndex(shipIndex.0 as COUNT);
    } else {
        hFleet = GetStarShipFromIndex(
            &raw mut GlobData.Game_state.avail_race_q,
            shipIndex.0 as COUNT,
        ) as HFLEETINFO;
    }
    if hFleet.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    TemplatePtr = LockFleetInfo(&raw mut GlobData.Game_state.avail_race_q, hFleet);
    if shipIndex.0 == RACE_ID::SAMATRA_SHIP.0 {
        captains_name_index = 0 as BYTE;
    } else {
        captains_name_index = NameCaptain(pDstQueue, (*TemplatePtr).SpeciesID);
    }
    hBuiltShip = Build(pDstQueue, (*TemplatePtr).SpeciesID) as HSHIPFRAG;
    if !hBuiltShip.is_null() {
        let mut ShipFragPtr: *mut SHIP_FRAGMENT = ::core::ptr::null_mut::<
            SHIP_FRAGMENT,
        >();
        ShipFragPtr = LockShipFrag(pDstQueue, hBuiltShip);
        (*ShipFragPtr).captains_name_index = captains_name_index;
        (*ShipFragPtr).race_strings = (*TemplatePtr).race_strings;
        (*ShipFragPtr).icons = (*TemplatePtr).icons;
        (*ShipFragPtr).melee_icon = (*TemplatePtr).melee_icon;
        if crew_level != 0 {
            (*ShipFragPtr).crew_level = crew_level;
        } else {
            (*ShipFragPtr).crew_level = (*TemplatePtr).crew_level;
        }
        (*ShipFragPtr).max_crew = (*TemplatePtr).max_crew;
        (*ShipFragPtr).energy_level = 0 as BYTE;
        (*ShipFragPtr).max_energy = (*TemplatePtr).max_energy;
        (*ShipFragPtr).race_id = shipIndex.0 as BYTE;
        (*ShipFragPtr).index = 0 as BYTE;
        UnlockLink(pDstQueue, hBuiltShip);
    }
    UnlockLink(&raw mut GlobData.Game_state.avail_race_q, hFleet);
    return hBuiltShip;
}
#[no_mangle]
pub unsafe extern "C" fn SetEscortCrewComplement(
    mut which_ship: RACE_ID,
    mut crew_level: COUNT,
    mut captain: BYTE,
) -> ::core::ffi::c_int {
    let mut hFleet: HFLEETINFO = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut TemplatePtr: *mut FLEET_INFO = ::core::ptr::null_mut::<FLEET_INFO>();
    let mut hStarShip: HSHIPFRAG = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut hNextShip: HSHIPFRAG = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut StarShipPtr: *mut SHIP_FRAGMENT = ::core::ptr::null_mut::<SHIP_FRAGMENT>();
    let mut Index: ::core::ffi::c_int = 0;
    hFleet = GetStarShipFromIndex(
        &raw mut GlobData.Game_state.avail_race_q,
        which_ship.0 as COUNT,
    ) as HFLEETINFO;
    if hFleet.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    TemplatePtr = LockFleetInfo(&raw mut GlobData.Game_state.avail_race_q, hFleet);
    hStarShip = GlobData.Game_state.built_ship_q.head as HSHIPFRAG;
    Index = 0 as ::core::ffi::c_int;
    while !hStarShip.is_null() {
        StarShipPtr = LockShipFrag(&raw mut GlobData.Game_state.built_ship_q, hStarShip);
        hNextShip = (*StarShipPtr).succ as HSHIPFRAG;
        if which_ship.0 == (*StarShipPtr).race_id as ::core::ffi::c_uint
            && (*StarShipPtr).crew_level as ::core::ffi::c_int
                == (*TemplatePtr).crew_level as ::core::ffi::c_int
        {
            break;
        }
        UnlockLink(&raw mut GlobData.Game_state.built_ship_q, hStarShip);
        hStarShip = hNextShip;
        Index += 1;
    }
    if !hStarShip.is_null() {
        (*StarShipPtr).crew_level = (if crew_level as ::core::ffi::c_int
            > (*StarShipPtr).max_crew as ::core::ffi::c_int
        {
            (*StarShipPtr).max_crew as ::core::ffi::c_int
        } else {
            crew_level as ::core::ffi::c_int
        }) as COUNT;
        (*StarShipPtr).captains_name_index = captain;
        UnlockLink(&raw mut GlobData.Game_state.built_ship_q, hStarShip);
    } else {
        Index = -1 as ::core::ffi::c_int;
    }
    UnlockLink(&raw mut GlobData.Game_state.avail_race_q, hFleet);
    return Index;
}
unsafe extern "C" fn deviceSwitch(
    mut device: ::core::ffi::c_int,
    mut val: ::core::ffi::c_int,
) {
    let mut var: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if val != 0 {
        var = val - 1 as ::core::ffi::c_int;
    }
    match device {
        0 => {
            uqm_lua_checked!(setGameStateUint(
                b"PORTAL_SPAWNER_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
                var as DWORD,
            ), ());
        }
        1 => {
            uqm_lua_checked!(setGameStateUint(
                b"TALKING_PET_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
                var as DWORD,
            ), ());
        }
        2 => {
            uqm_lua_checked!(setGameStateUint(
                b"UTWIG_BOMB_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
                var as DWORD,
            ), ());
        }
        3 => {
            uqm_lua_checked!(setGameStateUint(
                b"SUN_DEVICE_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
                var as DWORD,
            ), ());
        }
        4 => {
            uqm_lua_checked!(setGameStateUint(
                b"ROSY_SPHERE_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
                var as DWORD,
            ), ());
        }
        5 => {
            uqm_lua_checked!(setGameStateUint(
                b"AQUA_HELIX_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
                var as DWORD,
            ), ());
        }
        6 => {
            uqm_lua_checked!(setGameStateUint(
                b"CLEAR_SPINDLE_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
                var as DWORD,
            ), ());
        }
        7 => {
            uqm_lua_checked!(setGameStateUint(
                b"ULTRON_CONDITION\0".as_ptr() as *const ::core::ffi::c_char,
                var as DWORD,
            ), ());
        }
        8 => {
            uqm_lua_checked!(setGameStateUint(
                b"ULTRON_CONDITION\0".as_ptr() as *const ::core::ffi::c_char,
                (if var != 0 {
                    2 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                }) as DWORD,
            ), ());
        }
        9 => {
            uqm_lua_checked!(setGameStateUint(
                b"ULTRON_CONDITION\0".as_ptr() as *const ::core::ffi::c_char,
                (if var != 0 {
                    3 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                }) as DWORD,
            ), ());
        }
        10 => {
            uqm_lua_checked!(setGameStateUint(
                b"ULTRON_CONDITION\0".as_ptr() as *const ::core::ffi::c_char,
                (if var != 0 {
                    4 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                }) as DWORD,
            ), ());
        }
        11 => {
            uqm_lua_checked!(setGameStateUint(
                b"MAIDENS_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
                var as DWORD,
            ), ());
        }
        12 => {
            uqm_lua_checked!(setGameStateUint(
                b"UMGAH_BROADCASTERS_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
                var as DWORD,
            ), ());
        }
        13 => {
            uqm_lua_checked!(setGameStateUint(
                b"BURV_BROADCASTERS_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
                var as DWORD,
            ), ());
        }
        14 => {
            uqm_lua_checked!(setGameStateUint(
                b"TAALO_PROTECTOR_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
                var as DWORD,
            ), ());
        }
        15 | 16 | 17 => {
            if uqm_lua_checked!(getGameStateUint(
                b"KNOW_ABOUT_SHATTERED\0".as_ptr() as *const ::core::ffi::c_char,
            ), ()) < 2 as DWORD && var != 0
            {
                uqm_lua_checked!(setGameStateUint(
                    b"KNOW_ABOUT_SHATTERED\0".as_ptr() as *const ::core::ffi::c_char,
                    2 as DWORD,
                ), ());
            }
            uqm_lua_checked!(setGameStateUint(
                b"KNOW_SYREEN_WORLD_SHATTERED\0".as_ptr() as *const ::core::ffi::c_char,
                var as DWORD,
            ), ());
            if device == 15 as ::core::ffi::c_int {
                uqm_lua_checked!(setGameStateUint(
                    b"EGG_CASE0_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
                    var as DWORD,
                ), ());
            } else if device == 16 as ::core::ffi::c_int {
                uqm_lua_checked!(setGameStateUint(
                    b"EGG_CASE1_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
                    var as DWORD,
                ), ());
            } else {
                uqm_lua_checked!(setGameStateUint(
                    b"EGG_CASE2_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
                    var as DWORD,
                ), ());
            }
        }
        18 => {
            uqm_lua_checked!(setGameStateUint(
                b"SYREEN_SHUTTLE_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
                var as DWORD,
            ), ());
        }
        19 => {
            uqm_lua_checked!(setGameStateUint(
                b"VUX_BEAST_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
                var as DWORD,
            ), ());
        }
        20 => {
            uqm_lua_checked!(setGameStateUint(
                b"DESTRUCT_CODE_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
                var as DWORD,
            ), ());
        }
        21 => {
            uqm_lua_checked!(setGameStateUint(
                b"PORTAL_KEY_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
                var as DWORD,
            ), ());
        }
        22 => {
            uqm_lua_checked!(setGameStateUint(
                b"WIMBLIS_TRIDENT_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
                var as DWORD,
            ), ());
        }
        23 => {
            uqm_lua_checked!(setGameStateUint(
                b"GLOWING_ROD_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
                var as DWORD,
            ), ());
        }
        24 => {
            uqm_lua_checked!(setGameStateUint(
                b"MOONBASE_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
                var as DWORD,
            ), ());
        }
        _ => {}
    };
}
unsafe extern "C" fn upgradeSwitch(
    mut upgrade: ::core::ffi::c_int,
    mut val: ::core::ffi::c_int,
) {
    let mut var: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut LanderShields: BYTE = 0;
    let mut ModuleCost: ::core::ffi::c_int = 0;
    if val != 0 {
        var = val - 1 as ::core::ffi::c_int;
    }
    if upgrade > 2 as ::core::ffi::c_int || upgrade < 7 as ::core::ffi::c_int {
        LanderShields = uqm_lua_checked!(getGameStateUint(
            b"LANDER_SHIELDS\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) as BYTE;
    }
    match upgrade {
        0 => {
            uqm_lua_checked!(setGameStateUint(
                b"IMPROVED_LANDER_SPEED\0".as_ptr() as *const ::core::ffi::c_char,
                var as DWORD,
            ), ());
        }
        1 => {
            uqm_lua_checked!(setGameStateUint(
                b"IMPROVED_LANDER_CARGO\0".as_ptr() as *const ::core::ffi::c_char,
                var as DWORD,
            ), ());
        }
        2 => {
            uqm_lua_checked!(setGameStateUint(
                b"IMPROVED_LANDER_SHOT\0".as_ptr() as *const ::core::ffi::c_char,
                var as DWORD,
            ), ());
        }
        3 => {
            if var != 0 {
                LanderShields = (LanderShields as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int)
                        << C2Rust_Unnamed_8::BIOLOGICAL_DISASTER.0 as ::core::ffi::c_int)
                    as BYTE;
            } else {
                LanderShields = (LanderShields as ::core::ffi::c_int
                    & !((1 as ::core::ffi::c_int)
                        << C2Rust_Unnamed_8::BIOLOGICAL_DISASTER.0
                            as ::core::ffi::c_int)) as BYTE;
            }
            uqm_lua_checked!(setGameStateUint(
                b"LANDER_SHIELDS\0".as_ptr() as *const ::core::ffi::c_char,
                LanderShields as DWORD,
            ), ());
        }
        4 => {
            if var != 0 {
                LanderShields = (LanderShields as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int)
                        << C2Rust_Unnamed_8::EARTHQUAKE_DISASTER.0 as ::core::ffi::c_int)
                    as BYTE;
            } else {
                LanderShields = (LanderShields as ::core::ffi::c_int
                    & !((1 as ::core::ffi::c_int)
                        << C2Rust_Unnamed_8::EARTHQUAKE_DISASTER.0
                            as ::core::ffi::c_int)) as BYTE;
            }
            uqm_lua_checked!(setGameStateUint(
                b"LANDER_SHIELDS\0".as_ptr() as *const ::core::ffi::c_char,
                LanderShields as DWORD,
            ), ());
        }
        5 => {
            if var != 0 {
                LanderShields = (LanderShields as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int)
                        << C2Rust_Unnamed_8::LIGHTNING_DISASTER.0 as ::core::ffi::c_int)
                    as BYTE;
            } else {
                LanderShields = (LanderShields as ::core::ffi::c_int
                    & !((1 as ::core::ffi::c_int)
                        << C2Rust_Unnamed_8::LIGHTNING_DISASTER.0 as ::core::ffi::c_int))
                    as BYTE;
            }
            uqm_lua_checked!(setGameStateUint(
                b"LANDER_SHIELDS\0".as_ptr() as *const ::core::ffi::c_char,
                LanderShields as DWORD,
            ), ());
        }
        6 => {
            if var != 0 {
                LanderShields = (LanderShields as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int)
                        << C2Rust_Unnamed_8::LAVASPOT_DISASTER.0 as ::core::ffi::c_int)
                    as BYTE;
            } else {
                LanderShields = (LanderShields as ::core::ffi::c_int
                    & !((1 as ::core::ffi::c_int)
                        << C2Rust_Unnamed_8::LAVASPOT_DISASTER.0 as ::core::ffi::c_int))
                    as BYTE;
            }
            uqm_lua_checked!(setGameStateUint(
                b"LANDER_SHIELDS\0".as_ptr() as *const ::core::ffi::c_char,
                LanderShields as DWORD,
            ), ());
        }
        7 => {
            ModuleCost = if var != 0 {
                4000 as ::core::ffi::c_int / 50 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            };
            GlobData
                .Game_state
                .ModuleCost[C2Rust_Unnamed_7::ANTIMISSILE_DEFENSE.0 as ::core::ffi::c_int
                as usize] = ModuleCost as BYTE;
        }
        8 => {
            ModuleCost = if var != 0 {
                4000 as ::core::ffi::c_int / 50 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            };
            GlobData
                .Game_state
                .ModuleCost[C2Rust_Unnamed_7::BLASTER_WEAPON.0 as ::core::ffi::c_int
                as usize] = ModuleCost as BYTE;
        }
        9 => {
            ModuleCost = if var != 0 {
                1000 as ::core::ffi::c_int / 50 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            };
            GlobData
                .Game_state
                .ModuleCost[C2Rust_Unnamed_7::HIGHEFF_FUELSYS.0 as ::core::ffi::c_int
                as usize] = ModuleCost as BYTE;
        }
        10 => {
            ModuleCost = if var != 0 {
                5000 as ::core::ffi::c_int / 50 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            };
            GlobData
                .Game_state
                .ModuleCost[C2Rust_Unnamed_7::TRACKING_SYSTEM.0 as ::core::ffi::c_int
                as usize] = ModuleCost as BYTE;
        }
        11 => {
            ModuleCost = if var != 0 {
                6000 as ::core::ffi::c_int / 50 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            };
            GlobData
                .Game_state
                .ModuleCost[C2Rust_Unnamed_7::CANNON_WEAPON.0 as ::core::ffi::c_int
                as usize] = ModuleCost as BYTE;
        }
        12 => {
            ModuleCost = if var != 0 {
                4000 as ::core::ffi::c_int / 50 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            };
            GlobData
                .Game_state
                .ModuleCost[C2Rust_Unnamed_7::SHIVA_FURNACE.0 as ::core::ffi::c_int
                as usize] = ModuleCost as BYTE;
        }
        _ => {}
    };
}
unsafe extern "C" fn cheatAddRemoveDevices() {
    let mut i: BYTE = 0;
    i = 0 as BYTE;
    while (i as usize)
        < ::core::mem::size_of::<[OPT_ADD_REMOVE; 28]>()
            .wrapping_div(::core::mem::size_of::<OPT_ADD_REMOVE>())
    {
        if optDeviceArray[i as usize].0 != 0 {
            uqm_lua_checked!(deviceSwitch(
                i as ::core::ffi::c_int,
                optDeviceArray[i as usize].0 as ::core::ffi::c_int,
            ), ());
        }
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn cheatAddRemoveUpgrades() {
    let mut i: BYTE = 0;
    i = 0 as BYTE;
    while (i as ::core::ffi::c_int) < 13 as ::core::ffi::c_int {
        if optUpgradeArray[i as usize].0 != 0 {
            uqm_lua_checked!(upgradeSwitch(
                i as ::core::ffi::c_int,
                optUpgradeArray[i as usize].0 as ::core::ffi::c_int,
            ), ());
        }
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn loadGameCheats() {
    if if GlobData.SIS_state.Extended as ::core::ffi::c_int != 0 {
        BOOLEAN::TRUE.0 as ::core::ffi::c_int
    } else {
        BOOLEAN::FALSE.0 as ::core::ffi::c_int
    } != 0 && !(optSeedType > C2Rust_Unnamed::OPTVAL_PLANET.0 as ::core::ffi::c_int)
    {
        (*plot_map[C2Rust_Unnamed_11::MELNORME0_DEFINED.0 as ::core::ffi::c_int as usize]
            .star)
            .Type = ((C2Rust_Unnamed_9::SUPER_GIANT_STAR.0 as ::core::ffi::c_int as BYTE
            as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
            & ((0xff as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                & !(((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as BYTE
                    as ::core::ffi::c_int)) as BYTE as ::core::ffi::c_int
            | (C2Rust_Unnamed_10::ORANGE_BODY.0 as ::core::ffi::c_int as BYTE
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
    }
    if optInfiniteRU.0 != 0 {
        oldRU = GlobData.SIS_state.ResUnits;
    } else {
        oldRU = 0 as DWORD;
    }
    if optInfiniteFuel.0 != 0 {
        loadFuel = GlobData.SIS_state.FuelOnBoard;
        GlobData.SIS_state.FuelOnBoard = GetFuelTankCapacity();
    } else {
        loadFuel = 0 as DWORD;
    }
    uqm_lua_checked!(cheatAddRemoveDevices(), ());
    uqm_lua_checked!(cheatAddRemoveUpgrades(), ());
}
#[no_mangle]
pub unsafe extern "C" fn Jitter(mut str: COUNT, mut rand_val: UWORD) -> COUNT {
    return (str as ::core::ffi::c_int
        * (11 as ::core::ffi::c_int / 2 as ::core::ffi::c_int)
        * (255 as ::core::ffi::c_int
            - sqrt(rand_val as ::core::ffi::c_double) as COUNT as ::core::ffi::c_int)
        / 256 as ::core::ffi::c_int * 2 as ::core::ffi::c_int / 3 as ::core::ffi::c_int)
        as COUNT;
}
#[no_mangle]
pub unsafe extern "C" fn JitDebug(
    mut FleetPtr: *mut FLEET_INFO,
    mut rand_val_x: UWORD,
    mut rand_val_y: UWORD,
) {
    static mut fleet_name: [*const ::core::ffi::c_char; 26] = [
        b"ERROR\0".as_ptr() as *const ::core::ffi::c_char,
        b"Arilou\0".as_ptr() as *const ::core::ffi::c_char,
        b"CHMMR\0".as_ptr() as *const ::core::ffi::c_char,
        b"Earthling\0".as_ptr() as *const ::core::ffi::c_char,
        b"Orz\0".as_ptr() as *const ::core::ffi::c_char,
        b"Pkunk\0".as_ptr() as *const ::core::ffi::c_char,
        b"Shofixti\0".as_ptr() as *const ::core::ffi::c_char,
        b"Spathi\0".as_ptr() as *const ::core::ffi::c_char,
        b"Supox\0".as_ptr() as *const ::core::ffi::c_char,
        b"Thraddash\0".as_ptr() as *const ::core::ffi::c_char,
        b"Utwig\0".as_ptr() as *const ::core::ffi::c_char,
        b"VUX\0".as_ptr() as *const ::core::ffi::c_char,
        b"Yehat\0".as_ptr() as *const ::core::ffi::c_char,
        b"Melnorme\0".as_ptr() as *const ::core::ffi::c_char,
        b"Druuge\0".as_ptr() as *const ::core::ffi::c_char,
        b"Ilwrath\0".as_ptr() as *const ::core::ffi::c_char,
        b"Mycon\0".as_ptr() as *const ::core::ffi::c_char,
        b"Slylandro\0".as_ptr() as *const ::core::ffi::c_char,
        b"Umgah\0".as_ptr() as *const ::core::ffi::c_char,
        b"Ur Quan\0".as_ptr() as *const ::core::ffi::c_char,
        b"ZoqFotPik\0".as_ptr() as *const ::core::ffi::c_char,
        b"Syreen\0".as_ptr() as *const ::core::ffi::c_char,
        b"Kohr Ah\0".as_ptr() as *const ::core::ffi::c_char,
        b"Androsynth\0".as_ptr() as *const ::core::ffi::c_char,
        b"Chenjesu\0".as_ptr() as *const ::core::ffi::c_char,
        b"Mmrnmhrm\0".as_ptr() as *const ::core::ffi::c_char,
    ];
    let mut str: COUNT = (if (*FleetPtr).actual_strength as ::core::ffi::c_int
        > 0 as ::core::ffi::c_int
    {
        (*FleetPtr).actual_strength as ::core::ffi::c_int
    } else {
        WarEraStrength((*FleetPtr).SpeciesID) as ::core::ffi::c_int
    }) as COUNT;
    fprintf(
        stderr,
        b"%s Fleet %d: Actual Str %d; WarEra Str %d; \0".as_ptr()
            as *const ::core::ffi::c_char,
        fleet_name[(if (*FleetPtr).SpeciesID.0 > 25 as ::core::ffi::c_uint {
            0 as ::core::ffi::c_uint
        } else {
            (*FleetPtr).SpeciesID.0
        }) as usize],
        (*FleetPtr).SpeciesID.0,
        (*FleetPtr).actual_strength as ::core::ffi::c_int * 11 as ::core::ffi::c_int
            / 2 as ::core::ffi::c_int,
        WarEraStrength((*FleetPtr).SpeciesID) as ::core::ffi::c_int
            * 11 as ::core::ffi::c_int / 2 as ::core::ffi::c_int,
    );
    fprintf(
        stderr,
        b"Rand X %d (%d); Rand Y %d (%d)\n\0".as_ptr() as *const ::core::ffi::c_char,
        rand_val_x as ::core::ffi::c_int,
        255 as ::core::ffi::c_int
            - sqrt(rand_val_x as ::core::ffi::c_double) as COUNT as ::core::ffi::c_int,
        rand_val_y as ::core::ffi::c_int,
        255 as ::core::ffi::c_int
            - sqrt(rand_val_y as ::core::ffi::c_double) as COUNT as ::core::ffi::c_int,
    );
    fprintf(
        stderr,
        b"\t\tJit X %d%% Jit Y %d%%; (Jit X %d Jit Y %d)  \0".as_ptr()
            as *const ::core::ffi::c_char,
        (255 as ::core::ffi::c_int
            - sqrt(rand_val_x as ::core::ffi::c_double) as COUNT as ::core::ffi::c_int)
            * 67 as ::core::ffi::c_int / 256 as ::core::ffi::c_int,
        (255 as ::core::ffi::c_int
            - sqrt(rand_val_y as ::core::ffi::c_double) as COUNT as ::core::ffi::c_int)
            * 67 as ::core::ffi::c_int / 256 as ::core::ffi::c_int,
        Jitter(str, rand_val_x) as ::core::ffi::c_int,
        Jitter(str, rand_val_y) as ::core::ffi::c_int,
    );
    fprintf(
        stderr,
        b"%05.1f : %05.1f  units.\n\0".as_ptr() as *const ::core::ffi::c_char,
        (Jitter(str, rand_val_x) as ::core::ffi::c_float
            / 10 as ::core::ffi::c_int as ::core::ffi::c_float) as ::core::ffi::c_double,
        (Jitter(str, rand_val_y) as ::core::ffi::c_float
            / 10 as ::core::ffi::c_int as ::core::ffi::c_float) as ::core::ffi::c_double,
    );
}
#[no_mangle]
pub unsafe extern "C" fn DefaultFleetLocation(
    mut SpeciesID: SPECIES_ID,
    mut visit: COUNT,
) -> POINT {
    match SpeciesID {
        SPECIES_ID::CHMMR_ID => {
            return point {
                x: 577 as COORD,
                y: 2509 as COORD,
            };
        }
        SPECIES_ID::EARTHLING_ID => {
            return point {
                x: 1806 as COORD,
                y: 1476 as COORD,
            };
        }
        SPECIES_ID::PKUNK_ID => {
            if visit as ::core::ffi::c_int
                == C2Rust_Unnamed_11::YEHAT_DEFINED.0 as ::core::ffi::c_int
            {
                return point {
                    x: 4970 as COORD,
                    y: 400 as COORD,
                }
            } else if visit as ::core::ffi::c_int
                == C2Rust_Unnamed_11::WAR_ERA.0 as ::core::ffi::c_int
            {
                return point {
                    x: 577 as COORD,
                    y: 463 as COORD,
                }
            } else {
                return point {
                    x: 502 as COORD,
                    y: 401 as COORD,
                }
            }
        }
        SPECIES_ID::SHOFIXTI_ID => {
            return point {
                x: 2852 as COORD,
                y: 242 as COORD,
            };
        }
        SPECIES_ID::SPATHI_ID => {
            return point {
                x: 2416 as COORD,
                y: 3687 as COORD,
            };
        }
        SPECIES_ID::SUPOX_ID => {
            if visit as ::core::ffi::c_int
                == C2Rust_Unnamed_11::SAMATRA_DEFINED.0 as ::core::ffi::c_int
            {
                return point {
                    x: 6479 as COORD,
                    y: 7541 as COORD,
                }
            } else {
                return point {
                    x: 7468 as COORD,
                    y: 9246 as COORD,
                }
            }
        }
        SPECIES_ID::THRADDASH_ID => {
            if visit as ::core::ffi::c_int
                == C2Rust_Unnamed_11::SAMATRA_DEFINED.0 as ::core::ffi::c_int
            {
                return point {
                    x: 4879 as COORD,
                    y: 7201 as COORD,
                }
            } else if visit as ::core::ffi::c_int
                == C2Rust_Unnamed_11::WAR_ERA.0 as ::core::ffi::c_int
            {
                return point {
                    x: 2808 as COORD,
                    y: 8522 as COORD,
                }
            } else {
                return point {
                    x: 2535 as COORD,
                    y: 8358 as COORD,
                }
            }
        }
        SPECIES_ID::UTWIG_ID => {
            if visit as ::core::ffi::c_int
                == C2Rust_Unnamed_11::SAMATRA_DEFINED.0 as ::core::ffi::c_int
            {
                return point {
                    x: 7208 as COORD,
                    y: 7000 as COORD,
                }
            } else {
                return point {
                    x: 8534 as COORD,
                    y: 8797 as COORD,
                }
            }
        }
        SPECIES_ID::VUX_ID => {
            return point {
                x: 4333 as COORD,
                y: 1520 as COORD,
            };
        }
        SPECIES_ID::YEHAT_ID => {
            if visit as ::core::ffi::c_int
                == C2Rust_Unnamed_11::YEHAT_DEFINED.0 as ::core::ffi::c_int
            {
                return point {
                    x: 5150 as COORD,
                    y: 0 as COORD,
                }
            } else if visit as ::core::ffi::c_int
                == C2Rust_Unnamed_11::WAR_ERA.0 as ::core::ffi::c_int
            {
                return point {
                    x: 4969 as COORD,
                    y: 75 as COORD,
                }
            } else {
                return point {
                    x: 4970 as COORD,
                    y: 40 as COORD,
                }
            }
        }
        SPECIES_ID::DRUUGE_ID => {
            return point {
                x: 9421 as COORD,
                y: 2754 as COORD,
            };
        }
        SPECIES_ID::ILWRATH_ID => {
            if visit as ::core::ffi::c_int
                == C2Rust_Unnamed_11::THRADD_DEFINED.0 as ::core::ffi::c_int
            {
                return point {
                    x: 2500 as COORD,
                    y: 8070 as COORD,
                }
            } else if visit as ::core::ffi::c_int
                == C2Rust_Unnamed_11::PKUNK_DEFINED.0 as ::core::ffi::c_int
            {
                return point {
                    x: 48 as COORD,
                    y: 1700 as COORD,
                }
            } else if visit as ::core::ffi::c_int
                == C2Rust_Unnamed_11::WAR_ERA.0 as ::core::ffi::c_int
            {
                return point {
                    x: 0 as COORD,
                    y: 3589 as COORD,
                }
            } else {
                return point {
                    x: 215 as COORD,
                    y: 3630 as COORD,
                }
            }
        }
        SPECIES_ID::MYCON_ID => {
            if visit as ::core::ffi::c_int
                == C2Rust_Unnamed_11::MYCON_TRAP_DEFINED.0 as ::core::ffi::c_int
            {
                return point {
                    x: 6858 as COORD,
                    y: 577 as COORD,
                }
            } else if visit as ::core::ffi::c_int
                == C2Rust_Unnamed_11::WAR_ERA.0 as ::core::ffi::c_int
            {
                return point {
                    x: 6278 as COORD,
                    y: 2399 as COORD,
                }
            } else {
                return point {
                    x: 6392 as COORD,
                    y: 2200 as COORD,
                }
            }
        }
        SPECIES_ID::UMGAH_ID => {
            if visit as ::core::ffi::c_int
                == C2Rust_Unnamed_11::SAMATRA_DEFINED.0 as ::core::ffi::c_int
            {
                return point {
                    x: 5288 as COORD,
                    y: 4892 as COORD,
                }
            } else {
                return point {
                    x: 1860 as COORD,
                    y: 6099 as COORD,
                }
            }
        }
        SPECIES_ID::SYREEN_ID => {
            if visit as ::core::ffi::c_int
                == C2Rust_Unnamed_11::MYCON_TRAP_DEFINED.0 as ::core::ffi::c_int
            {
                return point {
                    x: 6858 as COORD,
                    y: 577 as COORD,
                }
            } else {
                return point {
                    x: 4125 as COORD,
                    y: 3770 as COORD,
                }
            }
        }
        SPECIES_ID::ANDROSYNTH_ID => {
            return point {
                x: 3676 as COORD,
                y: 2619 as COORD,
            };
        }
        SPECIES_ID::CHENJESU_ID => {
            return point {
                x: 701 as COORD,
                y: 2137 as COORD,
            };
        }
        SPECIES_ID::MMRNMHRM_ID => {
            return point {
                x: 672 as COORD,
                y: 2930 as COORD,
            };
        }
        _ => {
            fprintf(
                stderr,
                b"%s %d / plot %d combo.\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"DefaultFleetLocation called with bad fleet\0".as_ptr()
                    as *const ::core::ffi::c_char,
                SpeciesID.0,
                visit as ::core::ffi::c_int,
            );
            return point {
                x: 0 as COORD,
                y: 0 as COORD,
            };
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn WarEraStrength(mut SpeciesID: SPECIES_ID) -> COUNT {
    match SpeciesID {
        SPECIES_ID::EARTHLING_ID => {
            return (460 as ::core::ffi::c_int / 11 as ::core::ffi::c_int
                * 2 as ::core::ffi::c_int) as COUNT;
        }
        SPECIES_ID::PKUNK_ID => {
            return (350 as ::core::ffi::c_int / 11 as ::core::ffi::c_int
                * 2 as ::core::ffi::c_int) as COUNT;
        }
        SPECIES_ID::SHOFIXTI_ID => {
            return (329 as ::core::ffi::c_int / 11 as ::core::ffi::c_int
                * 2 as ::core::ffi::c_int) as COUNT;
        }
        SPECIES_ID::SPATHI_ID => {
            return (1007 as ::core::ffi::c_int / 11 as ::core::ffi::c_int
                * 2 as ::core::ffi::c_int) as COUNT;
        }
        SPECIES_ID::THRADDASH_ID => {
            return (723 as ::core::ffi::c_int / 11 as ::core::ffi::c_int
                * 2 as ::core::ffi::c_int) as COUNT;
        }
        SPECIES_ID::VUX_ID => {
            return (572 as ::core::ffi::c_int / 11 as ::core::ffi::c_int
                * 2 as ::core::ffi::c_int) as COUNT;
        }
        SPECIES_ID::YEHAT_ID => {
            return (1029 as ::core::ffi::c_int / 11 as ::core::ffi::c_int
                * 2 as ::core::ffi::c_int) as COUNT;
        }
        SPECIES_ID::DRUUGE_ID => {
            return (745 as ::core::ffi::c_int / 11 as ::core::ffi::c_int
                * 2 as ::core::ffi::c_int) as COUNT;
        }
        SPECIES_ID::ILWRATH_ID => {
            return (470 as ::core::ffi::c_int / 11 as ::core::ffi::c_int
                * 2 as ::core::ffi::c_int) as COUNT;
        }
        SPECIES_ID::MYCON_ID => {
            return (591 as ::core::ffi::c_int / 11 as ::core::ffi::c_int
                * 2 as ::core::ffi::c_int) as COUNT;
        }
        SPECIES_ID::UMGAH_ID => {
            return (506 as ::core::ffi::c_int / 11 as ::core::ffi::c_int
                * 2 as ::core::ffi::c_int) as COUNT;
        }
        SPECIES_ID::ANDROSYNTH_ID => {
            return (504 as ::core::ffi::c_int / 11 as ::core::ffi::c_int
                * 2 as ::core::ffi::c_int) as COUNT;
        }
        SPECIES_ID::CHENJESU_ID => {
            return (657 as ::core::ffi::c_int / 11 as ::core::ffi::c_int
                * 2 as ::core::ffi::c_int) as COUNT;
        }
        SPECIES_ID::MMRNMHRM_ID => {
            return (482 as ::core::ffi::c_int / 11 as ::core::ffi::c_int
                * 2 as ::core::ffi::c_int) as COUNT;
        }
        _ => return 0 as COUNT,
    };
}
#[no_mangle]
pub unsafe extern "C" fn Homeworld(mut SpeciesID: SPECIES_ID) -> COUNT {
    match SpeciesID {
        SPECIES_ID::ARILOU_ID => {
            return C2Rust_Unnamed_11::ARILOU_DEFINED.0 as ::core::ffi::c_int as COUNT;
        }
        SPECIES_ID::CHMMR_ID => {
            return C2Rust_Unnamed_11::CHMMR_DEFINED.0 as ::core::ffi::c_int as COUNT;
        }
        SPECIES_ID::EARTHLING_ID => {
            return C2Rust_Unnamed_11::SOL_DEFINED.0 as ::core::ffi::c_int as COUNT;
        }
        SPECIES_ID::ORZ_ID => {
            return C2Rust_Unnamed_11::ORZ_DEFINED.0 as ::core::ffi::c_int as COUNT;
        }
        SPECIES_ID::PKUNK_ID => {
            return C2Rust_Unnamed_11::PKUNK_DEFINED.0 as ::core::ffi::c_int as COUNT;
        }
        SPECIES_ID::SHOFIXTI_ID => {
            return C2Rust_Unnamed_11::SHOFIXTI_DEFINED.0 as ::core::ffi::c_int as COUNT;
        }
        SPECIES_ID::SPATHI_ID => {
            return C2Rust_Unnamed_11::SPATHI_DEFINED.0 as ::core::ffi::c_int as COUNT;
        }
        SPECIES_ID::SUPOX_ID => {
            return C2Rust_Unnamed_11::SUPOX_DEFINED.0 as ::core::ffi::c_int as COUNT;
        }
        SPECIES_ID::THRADDASH_ID => {
            return C2Rust_Unnamed_11::THRADD_DEFINED.0 as ::core::ffi::c_int as COUNT;
        }
        SPECIES_ID::UTWIG_ID => {
            return C2Rust_Unnamed_11::UTWIG_DEFINED.0 as ::core::ffi::c_int as COUNT;
        }
        SPECIES_ID::VUX_ID => {
            return C2Rust_Unnamed_11::VUX_DEFINED.0 as ::core::ffi::c_int as COUNT;
        }
        SPECIES_ID::YEHAT_ID => {
            return C2Rust_Unnamed_11::YEHAT_DEFINED.0 as ::core::ffi::c_int as COUNT;
        }
        SPECIES_ID::MELNORME_ID => {
            return C2Rust_Unnamed_11::MELNORME1_DEFINED.0 as ::core::ffi::c_int as COUNT;
        }
        SPECIES_ID::DRUUGE_ID => {
            return C2Rust_Unnamed_11::DRUUGE_DEFINED.0 as ::core::ffi::c_int as COUNT;
        }
        SPECIES_ID::ILWRATH_ID => {
            return C2Rust_Unnamed_11::ILWRATH_DEFINED.0 as ::core::ffi::c_int as COUNT;
        }
        SPECIES_ID::MYCON_ID => {
            return C2Rust_Unnamed_11::MYCON_DEFINED.0 as ::core::ffi::c_int as COUNT;
        }
        SPECIES_ID::SLYLANDRO_ID => {
            return C2Rust_Unnamed_11::SLYLANDRO_DEFINED.0 as ::core::ffi::c_int as COUNT;
        }
        SPECIES_ID::UMGAH_ID => {
            return C2Rust_Unnamed_11::TALKING_PET_DEFINED.0 as ::core::ffi::c_int
                as COUNT;
        }
        SPECIES_ID::UR_QUAN_ID => {
            return C2Rust_Unnamed_11::SAMATRA_DEFINED.0 as ::core::ffi::c_int as COUNT;
        }
        SPECIES_ID::ZOQFOTPIK_ID => {
            return C2Rust_Unnamed_11::ZOQFOT_DEFINED.0 as ::core::ffi::c_int as COUNT;
        }
        SPECIES_ID::SYREEN_ID => {
            return C2Rust_Unnamed_11::SYREEN_DEFINED.0 as ::core::ffi::c_int as COUNT;
        }
        SPECIES_ID::KOHR_AH_ID => {
            return C2Rust_Unnamed_11::SAMATRA_DEFINED.0 as ::core::ffi::c_int as COUNT;
        }
        SPECIES_ID::ANDROSYNTH_ID => {
            return C2Rust_Unnamed_11::ANDROSYNTH_DEFINED.0 as ::core::ffi::c_int as COUNT;
        }
        SPECIES_ID::CHENJESU_ID => {
            return C2Rust_Unnamed_11::CHMMR_DEFINED.0 as ::core::ffi::c_int as COUNT;
        }
        SPECIES_ID::MMRNMHRM_ID => {
            return C2Rust_Unnamed_11::MOTHER_ARK_DEFINED.0 as ::core::ffi::c_int as COUNT;
        }
        _ => {
            fprintf(
                stderr,
                b"Homeworld %s %d.\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"(Fleet ID) called with bad fleet ID\0".as_ptr()
                    as *const ::core::ffi::c_char,
                SpeciesID.0,
            );
            return C2Rust_Unnamed_11::NUM_PLOTS.0 as ::core::ffi::c_int as COUNT;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn SeedFleet(
    mut FleetPtr: *mut FLEET_INFO,
    mut plotmap: *mut PLOT_LOCATION,
) {
    if !(optSeedType > C2Rust_Unnamed::OPTVAL_PLANET.0 as ::core::ffi::c_int)
        || optSeedType == C2Rust_Unnamed::OPTVAL_MRQ.0 as ::core::ffi::c_int
    {
        return;
    }
    if FleetPtr.is_null() || plotmap.is_null() {
        fprintf(
            stderr,
            b"SeedFleet called with NULL PTR(s).\n\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    (*FleetPtr).known_loc = SeedFleetLocation(
        FleetPtr,
        plotmap,
        (if (*FleetPtr).SpeciesID.0 == SPECIES_ID::ILWRATH_ID.0 {
            C2Rust_Unnamed_11::PKUNK_DEFINED.0 as ::core::ffi::c_int
        } else {
            C2Rust_Unnamed_11::HOME.0 as ::core::ffi::c_int
        }) as COUNT,
    );
}
#[no_mangle]
pub unsafe extern "C" fn SeedFleetLocation(
    mut FleetPtr: *mut FLEET_INFO,
    mut plotmap: *mut PLOT_LOCATION,
    mut visit: COUNT,
) -> POINT {
    let mut rand_val_x: UWORD = 0;
    let mut rand_val_y: UWORD = 0;
    let mut home: COUNT = 0;
    let mut strength: COUNT = 0;
    let mut warpoint: POINT = POINT { x: 0, y: 0 };
    let mut location: POINT = point {
        x: 0 as COORD,
        y: 0 as COORD,
    };
    let mut myRNG: BOOLEAN = BOOLEAN(0 as ::core::ffi::c_int as ::core::ffi::c_uint);
    let mut buf: [UNICODE; 256] = ::core::mem::transmute::<
        [u8; 256],
        [UNICODE; 256],
    >(
        *b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    );
    if FleetPtr.is_null() || plotmap.is_null() {
        fprintf(
            stderr,
            b"SeedFleetLocation called with NULL PTR(s).\n\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return point {
            x: !(0 as ::core::ffi::c_int) as COORD,
            y: !(0 as ::core::ffi::c_int) as COORD,
        };
    }
    if !(optSeedType > C2Rust_Unnamed::OPTVAL_PLANET.0 as ::core::ffi::c_int)
        || optSeedType == C2Rust_Unnamed::OPTVAL_MRQ.0 as ::core::ffi::c_int
    {
        return DefaultFleetLocation((*FleetPtr).SpeciesID, visit);
    }
    home = Homeworld((*FleetPtr).SpeciesID);
    if home as ::core::ffi::c_int >= C2Rust_Unnamed_11::NUM_PLOTS.0 as ::core::ffi::c_int
    {
        return point {
            x: !(0 as ::core::ffi::c_int) as COORD,
            y: !(0 as ::core::ffi::c_int) as COORD,
        };
    }
    if visit as ::core::ffi::c_int
        >= C2Rust_Unnamed_11::NUM_PLOTS.0 as ::core::ffi::c_int
        && visit as ::core::ffi::c_int
            != C2Rust_Unnamed_11::WAR_ERA.0 as ::core::ffi::c_int
        && visit as ::core::ffi::c_int != C2Rust_Unnamed_11::HOME.0 as ::core::ffi::c_int
    {
        fprintf(
            stderr,
            b"%s %d\n\0".as_ptr() as *const ::core::ffi::c_char,
            b"SeedFleet called with invalid away plot ID\0".as_ptr()
                as *const ::core::ffi::c_char,
            visit as ::core::ffi::c_int,
        );
        return point {
            x: !(0 as ::core::ffi::c_int) as COORD,
            y: !(0 as ::core::ffi::c_int) as COORD,
        };
    }
    if (visit as ::core::ffi::c_int)
        < C2Rust_Unnamed_11::NUM_PLOTS.0 as ::core::ffi::c_int
        && (*plotmap.offset(visit as isize)).star.is_null()
    {
        fprintf(
            stderr,
            b"%s %d has a NULL star pointer.\n\0".as_ptr() as *const ::core::ffi::c_char,
            b"SeedFleet called, but away plot ID\0".as_ptr()
                as *const ::core::ffi::c_char,
            visit as ::core::ffi::c_int,
        );
        return point {
            x: !(0 as ::core::ffi::c_int) as COORD,
            y: !(0 as ::core::ffi::c_int) as COORD,
        };
    }
    if (*plotmap.offset(home as isize)).star.is_null() {
        fprintf(
            stderr,
            b"%s %d has a NULL star pointer.\n\0".as_ptr() as *const ::core::ffi::c_char,
            b"SeedFleet called, but home plot ID\0".as_ptr()
                as *const ::core::ffi::c_char,
            home as ::core::ffi::c_int,
        );
        return point {
            x: !(0 as ::core::ffi::c_int) as COORD,
            y: !(0 as ::core::ffi::c_int) as COORD,
        };
    }
    if StarGenRNG.is_null() {
        StarGenRNG = RandomContext_New();
        myRNG = BOOLEAN(1 as ::core::ffi::c_int as ::core::ffi::c_uint);
    }
    RandomContext_SeedRandom(
        StarGenRNG,
        GetRandomSeedForStar((*plotmap.offset(home as isize)).star),
    );
    rand_val_x = RandomContext_Random(StarGenRNG) as UWORD;
    rand_val_y = RandomContext_Random(StarGenRNG) as UWORD;
    if visit as ::core::ffi::c_int > 0 as ::core::ffi::c_int
        && visit as ::core::ffi::c_int
            != C2Rust_Unnamed_11::SAMATRA_DEFINED.0 as ::core::ffi::c_int
        && (visit as ::core::ffi::c_int)
            < C2Rust_Unnamed_11::NUM_PLOTS.0 as ::core::ffi::c_int
    {
        RandomContext_SeedRandom(
            StarGenRNG,
            GetRandomSeedForStar((*plotmap.offset(visit as isize)).star),
        );
        rand_val_x = (rand_val_x as ::core::ffi::c_ulong)
            .wrapping_add(
                (RandomContext_Random(StarGenRNG) as usize)
                    .wrapping_rem(::core::mem::size_of::<UWORD>())
                    as ::core::ffi::c_ulong,
            ) as UWORD;
        rand_val_y = (rand_val_y as ::core::ffi::c_ulong)
            .wrapping_add(
                (RandomContext_Random(StarGenRNG) as usize)
                    .wrapping_rem(::core::mem::size_of::<UWORD>())
                    as ::core::ffi::c_ulong,
            ) as UWORD;
        warpoint = (*plotmap.offset(visit as isize)).star_pt;
    } else if visit as ::core::ffi::c_int
        == C2Rust_Unnamed_11::SAMATRA_DEFINED.0 as ::core::ffi::c_int
    {
        RandomContext_SeedRandom(
            StarGenRNG,
            GetRandomSeedForStar((*plotmap.offset(visit as isize)).star),
        );
        let mut rand_val_s: UWORD = RandomContext_Random(StarGenRNG) as UWORD;
        warpoint = point {
            x: ((*plotmap.offset(visit as isize)).star_pt.x as ::core::ffi::c_int
                + rand_val_s as BYTE as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    * if (*plotmap.offset(home as isize)).star_pt.x as ::core::ffi::c_int
                        > (*plotmap.offset(visit as isize)).star_pt.x
                            as ::core::ffi::c_int
                    {
                        1 as ::core::ffi::c_int
                    } else {
                        -1 as ::core::ffi::c_int
                    }) as COORD,
            y: ((*plotmap.offset(visit as isize)).star_pt.y as ::core::ffi::c_int
                + (rand_val_s as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as BYTE
                    as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    * if (*plotmap.offset(home as isize)).star_pt.y as ::core::ffi::c_int
                        > (*plotmap.offset(visit as isize)).star_pt.y
                            as ::core::ffi::c_int
                    {
                        1 as ::core::ffi::c_int
                    } else {
                        -1 as ::core::ffi::c_int
                    }) as COORD,
        };
        snprintf(
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[UNICODE; 256]>(),
            b"'Sa-Matra' center %05.1f : %05.1f\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            (warpoint.x as ::core::ffi::c_float
                / 10 as ::core::ffi::c_int as ::core::ffi::c_float)
                as ::core::ffi::c_double,
            (warpoint.y as ::core::ffi::c_float
                / 10 as ::core::ffi::c_int as ::core::ffi::c_float)
                as ::core::ffi::c_double,
        );
    } else {
        warpoint = (*plotmap.offset(home as isize)).star_pt;
    }
    strength = (if visit as ::core::ffi::c_int
        == C2Rust_Unnamed_11::WAR_ERA.0 as ::core::ffi::c_int
    {
        WarEraStrength((*FleetPtr).SpeciesID) as ::core::ffi::c_int
    } else {
        (*FleetPtr).actual_strength as ::core::ffi::c_int
    }) as COUNT;
    match (*FleetPtr).SpeciesID {
        SPECIES_ID::ARILOU_ID => {
            location = (*plotmap.offset(home as isize)).star_pt;
        }
        SPECIES_ID::CHMMR_ID => {
            location = point {
                x: ((*plotmap.offset(home as isize)).star_pt.x as ::core::ffi::c_int
                    + if ((*plotmap.offset(home as isize)).star_pt.x
                        as ::core::ffi::c_int)
                        < (*plotmap
                            .offset(
                                C2Rust_Unnamed_11::MOTHER_ARK_DEFINED.0
                                    as ::core::ffi::c_int as isize,
                            ))
                            .star_pt
                            .x as ::core::ffi::c_int
                    {
                        1 as ::core::ffi::c_int
                    } else {
                        -1 as ::core::ffi::c_int
                    }
                        * if rand_val_x as ::core::ffi::c_int
                            * rand_val_y as ::core::ffi::c_int % 10 as ::core::ffi::c_int
                            != 0
                        {
                            1 as ::core::ffi::c_int
                        } else {
                            -1 as ::core::ffi::c_int
                        } * Jitter(strength, rand_val_x) as ::core::ffi::c_int) as COORD,
                y: ((*plotmap.offset(home as isize)).star_pt.y as ::core::ffi::c_int
                    + if ((*plotmap.offset(home as isize)).star_pt.y
                        as ::core::ffi::c_int)
                        < (*plotmap
                            .offset(
                                C2Rust_Unnamed_11::MOTHER_ARK_DEFINED.0
                                    as ::core::ffi::c_int as isize,
                            ))
                            .star_pt
                            .y as ::core::ffi::c_int
                    {
                        1 as ::core::ffi::c_int
                    } else {
                        -1 as ::core::ffi::c_int
                    }
                        * if (rand_val_x as ::core::ffi::c_int
                            + rand_val_y as ::core::ffi::c_int) % 5 as ::core::ffi::c_int
                            != 0
                        {
                            1 as ::core::ffi::c_int
                        } else {
                            -1 as ::core::ffi::c_int
                        } * Jitter(strength, rand_val_y) as ::core::ffi::c_int) as COORD,
            };
        }
        SPECIES_ID::EARTHLING_ID => {
            location = point {
                x: ((*plotmap.offset(home as isize)).star_pt.x as ::core::ffi::c_int
                    + if rand_val_x as ::core::ffi::c_int % 2 as ::core::ffi::c_int != 0
                    {
                        1 as ::core::ffi::c_int
                    } else {
                        -1 as ::core::ffi::c_int
                    } * Jitter(strength, rand_val_x) as ::core::ffi::c_int) as COORD,
                y: ((*plotmap.offset(home as isize)).star_pt.y as ::core::ffi::c_int
                    + if rand_val_y as ::core::ffi::c_int % 2 as ::core::ffi::c_int != 0
                    {
                        1 as ::core::ffi::c_int
                    } else {
                        -1 as ::core::ffi::c_int
                    } * Jitter(strength, rand_val_y) as ::core::ffi::c_int) as COORD,
            };
        }
        SPECIES_ID::ORZ_ID => {
            location = point {
                x: ((*plotmap.offset(home as isize)).star_pt.x as ::core::ffi::c_int
                    + if ((*plotmap.offset(home as isize)).star_pt.x
                        as ::core::ffi::c_int)
                        < (*plotmap
                            .offset(
                                C2Rust_Unnamed_11::TAALO_PROTECTOR_DEFINED.0
                                    as ::core::ffi::c_int as isize,
                            ))
                            .star_pt
                            .x as ::core::ffi::c_int
                    {
                        1 as ::core::ffi::c_int
                    } else {
                        -1 as ::core::ffi::c_int
                    }
                        * if rand_val_x as ::core::ffi::c_int
                            * rand_val_y as ::core::ffi::c_int % 10 as ::core::ffi::c_int
                            != 0
                        {
                            1 as ::core::ffi::c_int
                        } else {
                            -1 as ::core::ffi::c_int
                        } * Jitter(strength, rand_val_x) as ::core::ffi::c_int) as COORD,
                y: ((*plotmap.offset(home as isize)).star_pt.y as ::core::ffi::c_int
                    + if ((*plotmap.offset(home as isize)).star_pt.y
                        as ::core::ffi::c_int)
                        < (*plotmap
                            .offset(
                                C2Rust_Unnamed_11::TAALO_PROTECTOR_DEFINED.0
                                    as ::core::ffi::c_int as isize,
                            ))
                            .star_pt
                            .y as ::core::ffi::c_int
                    {
                        1 as ::core::ffi::c_int
                    } else {
                        -1 as ::core::ffi::c_int
                    }
                        * if (rand_val_x as ::core::ffi::c_int
                            + rand_val_y as ::core::ffi::c_int) % 5 as ::core::ffi::c_int
                            != 0
                        {
                            1 as ::core::ffi::c_int
                        } else {
                            -1 as ::core::ffi::c_int
                        } * Jitter(strength, rand_val_y) as ::core::ffi::c_int) as COORD,
            };
        }
        SPECIES_ID::PKUNK_ID => {
            if visit as ::core::ffi::c_int
                == C2Rust_Unnamed_11::YEHAT_DEFINED.0 as ::core::ffi::c_int
            {
                location = point {
                    x: ((*plotmap.offset(visit as isize)).star_pt.x as ::core::ffi::c_int
                        + if (*plotmap.offset(home as isize)).star_pt.x
                            as ::core::ffi::c_int
                            > (*plotmap.offset(visit as isize)).star_pt.x
                                as ::core::ffi::c_int
                        {
                            1 as ::core::ffi::c_int
                        } else {
                            -1 as ::core::ffi::c_int
                        }
                            * if rand_val_x as ::core::ffi::c_int
                                * rand_val_y as ::core::ffi::c_int
                                % 10 as ::core::ffi::c_int != 0
                            {
                                1 as ::core::ffi::c_int
                            } else {
                                -1 as ::core::ffi::c_int
                            } * Jitter(strength, rand_val_x) as ::core::ffi::c_int)
                        as COORD,
                    y: ((*plotmap.offset(visit as isize)).star_pt.y as ::core::ffi::c_int
                        + if (*plotmap.offset(home as isize)).star_pt.y
                            as ::core::ffi::c_int
                            > (*plotmap.offset(visit as isize)).star_pt.y
                                as ::core::ffi::c_int
                        {
                            1 as ::core::ffi::c_int
                        } else {
                            -1 as ::core::ffi::c_int
                        }
                            * if (rand_val_x as ::core::ffi::c_int
                                + rand_val_y as ::core::ffi::c_int)
                                % 5 as ::core::ffi::c_int != 0
                            {
                                1 as ::core::ffi::c_int
                            } else {
                                -1 as ::core::ffi::c_int
                            } * Jitter(strength, rand_val_y) as ::core::ffi::c_int)
                        as COORD,
                };
            } else {
                location = point {
                    x: ((*plotmap.offset(home as isize)).star_pt.x as ::core::ffi::c_int
                        + if (*plotmap.offset(home as isize)).star_pt.x
                            as ::core::ffi::c_int
                            > (*plotmap
                                .offset(
                                    C2Rust_Unnamed_11::ILWRATH_DEFINED.0 as ::core::ffi::c_int
                                        as isize,
                                ))
                                .star_pt
                                .x as ::core::ffi::c_int
                        {
                            1 as ::core::ffi::c_int
                        } else {
                            -1 as ::core::ffi::c_int
                        }
                            * if rand_val_x as ::core::ffi::c_int
                                * rand_val_y as ::core::ffi::c_int
                                % 10 as ::core::ffi::c_int != 0
                            {
                                1 as ::core::ffi::c_int
                            } else {
                                -1 as ::core::ffi::c_int
                            } * Jitter(strength, rand_val_x) as ::core::ffi::c_int)
                        as COORD,
                    y: ((*plotmap.offset(home as isize)).star_pt.y as ::core::ffi::c_int
                        + if (*plotmap.offset(home as isize)).star_pt.y
                            as ::core::ffi::c_int
                            > (*plotmap
                                .offset(
                                    C2Rust_Unnamed_11::ILWRATH_DEFINED.0 as ::core::ffi::c_int
                                        as isize,
                                ))
                                .star_pt
                                .y as ::core::ffi::c_int
                        {
                            1 as ::core::ffi::c_int
                        } else {
                            -1 as ::core::ffi::c_int
                        }
                            * if (rand_val_x as ::core::ffi::c_int
                                + rand_val_y as ::core::ffi::c_int)
                                % 5 as ::core::ffi::c_int != 0
                            {
                                1 as ::core::ffi::c_int
                            } else {
                                -1 as ::core::ffi::c_int
                            } * Jitter(strength, rand_val_y) as ::core::ffi::c_int)
                        as COORD,
                };
            }
        }
        SPECIES_ID::SHOFIXTI_ID => {
            location = point {
                x: ((*plotmap.offset(home as isize)).star_pt.x as ::core::ffi::c_int
                    + if ((*plotmap.offset(home as isize)).star_pt.x
                        as ::core::ffi::c_int)
                        < (*plotmap
                            .offset(
                                C2Rust_Unnamed_11::YEHAT_DEFINED.0 as ::core::ffi::c_int
                                    as isize,
                            ))
                            .star_pt
                            .x as ::core::ffi::c_int
                    {
                        1 as ::core::ffi::c_int
                    } else {
                        -1 as ::core::ffi::c_int
                    }
                        * if rand_val_x as ::core::ffi::c_int
                            * rand_val_y as ::core::ffi::c_int % 10 as ::core::ffi::c_int
                            != 0
                        {
                            1 as ::core::ffi::c_int
                        } else {
                            -1 as ::core::ffi::c_int
                        } * Jitter(strength, rand_val_x) as ::core::ffi::c_int) as COORD,
                y: ((*plotmap.offset(home as isize)).star_pt.y as ::core::ffi::c_int
                    + if ((*plotmap.offset(home as isize)).star_pt.y
                        as ::core::ffi::c_int)
                        < (*plotmap
                            .offset(
                                C2Rust_Unnamed_11::YEHAT_DEFINED.0 as ::core::ffi::c_int
                                    as isize,
                            ))
                            .star_pt
                            .y as ::core::ffi::c_int
                    {
                        1 as ::core::ffi::c_int
                    } else {
                        -1 as ::core::ffi::c_int
                    }
                        * if (rand_val_x as ::core::ffi::c_int
                            + rand_val_y as ::core::ffi::c_int) % 5 as ::core::ffi::c_int
                            != 0
                        {
                            1 as ::core::ffi::c_int
                        } else {
                            -1 as ::core::ffi::c_int
                        } * Jitter(strength, rand_val_y) as ::core::ffi::c_int) as COORD,
            };
        }
        SPECIES_ID::SPATHI_ID => {
            location = point {
                x: ((*plotmap.offset(home as isize)).star_pt.x as ::core::ffi::c_int
                    + if rand_val_x as ::core::ffi::c_int % 2 as ::core::ffi::c_int != 0
                    {
                        1 as ::core::ffi::c_int
                    } else {
                        -1 as ::core::ffi::c_int
                    } * Jitter(strength, rand_val_x) as ::core::ffi::c_int) as COORD,
                y: ((*plotmap.offset(home as isize)).star_pt.y as ::core::ffi::c_int
                    + if rand_val_y as ::core::ffi::c_int % 2 as ::core::ffi::c_int != 0
                    {
                        1 as ::core::ffi::c_int
                    } else {
                        -1 as ::core::ffi::c_int
                    } * Jitter(strength, rand_val_y) as ::core::ffi::c_int) as COORD,
            };
        }
        SPECIES_ID::SUPOX_ID => {
            if visit as ::core::ffi::c_int
                == C2Rust_Unnamed_11::SAMATRA_DEFINED.0 as ::core::ffi::c_int
            {
                location = point {
                    x: (warpoint.x as ::core::ffi::c_int as ::core::ffi::c_double
                        + (1200 as ::core::ffi::c_int
                            * ((*plotmap.offset(home as isize)).star_pt.x
                                as ::core::ffi::c_int - warpoint.x as ::core::ffi::c_int))
                            as ::core::ffi::c_double
                            / sqrt(
                                (((*plotmap.offset(home as isize)).star_pt.x
                                    as ::core::ffi::c_int - warpoint.x as ::core::ffi::c_int)
                                    * ((*plotmap.offset(home as isize)).star_pt.x
                                        as ::core::ffi::c_int - warpoint.x as ::core::ffi::c_int)
                                    + ((*plotmap.offset(home as isize)).star_pt.y
                                        as ::core::ffi::c_int - warpoint.y as ::core::ffi::c_int)
                                        * ((*plotmap.offset(home as isize)).star_pt.y
                                            as ::core::ffi::c_int - warpoint.y as ::core::ffi::c_int))
                                    as ::core::ffi::c_double,
                            )
                        + (if (*plotmap.offset(home as isize)).star_pt.x
                            as ::core::ffi::c_int
                            > (*plotmap
                                .offset(
                                    C2Rust_Unnamed_11::UTWIG_DEFINED.0 as ::core::ffi::c_int
                                        as isize,
                                ))
                                .star_pt
                                .x as ::core::ffi::c_int
                        {
                            1 as ::core::ffi::c_int
                        } else {
                            -1 as ::core::ffi::c_int
                        } * Jitter(strength, rand_val_x) as ::core::ffi::c_int)
                            as ::core::ffi::c_double) as COORD,
                    y: (warpoint.y as ::core::ffi::c_int as ::core::ffi::c_double
                        + (1200 as ::core::ffi::c_int
                            * ((*plotmap.offset(home as isize)).star_pt.y
                                as ::core::ffi::c_int - warpoint.y as ::core::ffi::c_int))
                            as ::core::ffi::c_double
                            / sqrt(
                                (((*plotmap.offset(home as isize)).star_pt.x
                                    as ::core::ffi::c_int - warpoint.x as ::core::ffi::c_int)
                                    * ((*plotmap.offset(home as isize)).star_pt.x
                                        as ::core::ffi::c_int - warpoint.x as ::core::ffi::c_int)
                                    + ((*plotmap.offset(home as isize)).star_pt.y
                                        as ::core::ffi::c_int - warpoint.y as ::core::ffi::c_int)
                                        * ((*plotmap.offset(home as isize)).star_pt.y
                                            as ::core::ffi::c_int - warpoint.y as ::core::ffi::c_int))
                                    as ::core::ffi::c_double,
                            )
                        + (if (*plotmap.offset(home as isize)).star_pt.y
                            as ::core::ffi::c_int
                            > (*plotmap
                                .offset(
                                    C2Rust_Unnamed_11::UTWIG_DEFINED.0 as ::core::ffi::c_int
                                        as isize,
                                ))
                                .star_pt
                                .y as ::core::ffi::c_int
                        {
                            1 as ::core::ffi::c_int
                        } else {
                            -1 as ::core::ffi::c_int
                        } * Jitter(strength, rand_val_y) as ::core::ffi::c_int)
                            as ::core::ffi::c_double) as COORD,
                };
            } else {
                location = point {
                    x: ((*plotmap.offset(home as isize)).star_pt.x as ::core::ffi::c_int
                        + if ((*plotmap.offset(home as isize)).star_pt.x
                            as ::core::ffi::c_int)
                            < (*plotmap
                                .offset(
                                    C2Rust_Unnamed_11::UTWIG_DEFINED.0 as ::core::ffi::c_int
                                        as isize,
                                ))
                                .star_pt
                                .x as ::core::ffi::c_int
                        {
                            1 as ::core::ffi::c_int
                        } else {
                            -1 as ::core::ffi::c_int
                        }
                            * if rand_val_x as ::core::ffi::c_int
                                * rand_val_y as ::core::ffi::c_int
                                % 10 as ::core::ffi::c_int != 0
                            {
                                1 as ::core::ffi::c_int
                            } else {
                                -1 as ::core::ffi::c_int
                            } * Jitter(strength, rand_val_x) as ::core::ffi::c_int)
                        as COORD,
                    y: ((*plotmap.offset(home as isize)).star_pt.y as ::core::ffi::c_int
                        + if ((*plotmap.offset(home as isize)).star_pt.y
                            as ::core::ffi::c_int)
                            < (*plotmap
                                .offset(
                                    C2Rust_Unnamed_11::UTWIG_DEFINED.0 as ::core::ffi::c_int
                                        as isize,
                                ))
                                .star_pt
                                .y as ::core::ffi::c_int
                        {
                            1 as ::core::ffi::c_int
                        } else {
                            -1 as ::core::ffi::c_int
                        }
                            * if (rand_val_x as ::core::ffi::c_int
                                + rand_val_y as ::core::ffi::c_int)
                                % 5 as ::core::ffi::c_int != 0
                            {
                                1 as ::core::ffi::c_int
                            } else {
                                -1 as ::core::ffi::c_int
                            } * Jitter(strength, rand_val_y) as ::core::ffi::c_int)
                        as COORD,
                };
            }
        }
        SPECIES_ID::THRADDASH_ID => {
            if visit as ::core::ffi::c_int
                == C2Rust_Unnamed_11::SAMATRA_DEFINED.0 as ::core::ffi::c_int
            {
                location = point {
                    x: (warpoint.x as ::core::ffi::c_int as ::core::ffi::c_double
                        + (1200 as ::core::ffi::c_int
                            * ((*plotmap.offset(home as isize)).star_pt.x
                                as ::core::ffi::c_int - warpoint.x as ::core::ffi::c_int))
                            as ::core::ffi::c_double
                            / sqrt(
                                (((*plotmap.offset(home as isize)).star_pt.x
                                    as ::core::ffi::c_int - warpoint.x as ::core::ffi::c_int)
                                    * ((*plotmap.offset(home as isize)).star_pt.x
                                        as ::core::ffi::c_int - warpoint.x as ::core::ffi::c_int)
                                    + ((*plotmap.offset(home as isize)).star_pt.y
                                        as ::core::ffi::c_int - warpoint.y as ::core::ffi::c_int)
                                        * ((*plotmap.offset(home as isize)).star_pt.y
                                            as ::core::ffi::c_int - warpoint.y as ::core::ffi::c_int))
                                    as ::core::ffi::c_double,
                            )
                        + (if (*plotmap.offset(home as isize)).star_pt.x
                            as ::core::ffi::c_int
                            > (*plotmap
                                .offset(
                                    C2Rust_Unnamed_11::AQUA_HELIX_DEFINED.0
                                        as ::core::ffi::c_int as isize,
                                ))
                                .star_pt
                                .x as ::core::ffi::c_int
                        {
                            1 as ::core::ffi::c_int
                        } else {
                            -1 as ::core::ffi::c_int
                        } * Jitter(strength, rand_val_x) as ::core::ffi::c_int)
                            as ::core::ffi::c_double) as COORD,
                    y: (warpoint.y as ::core::ffi::c_int as ::core::ffi::c_double
                        + (1200 as ::core::ffi::c_int
                            * ((*plotmap.offset(home as isize)).star_pt.y
                                as ::core::ffi::c_int - warpoint.y as ::core::ffi::c_int))
                            as ::core::ffi::c_double
                            / sqrt(
                                (((*plotmap.offset(home as isize)).star_pt.x
                                    as ::core::ffi::c_int - warpoint.x as ::core::ffi::c_int)
                                    * ((*plotmap.offset(home as isize)).star_pt.x
                                        as ::core::ffi::c_int - warpoint.x as ::core::ffi::c_int)
                                    + ((*plotmap.offset(home as isize)).star_pt.y
                                        as ::core::ffi::c_int - warpoint.y as ::core::ffi::c_int)
                                        * ((*plotmap.offset(home as isize)).star_pt.y
                                            as ::core::ffi::c_int - warpoint.y as ::core::ffi::c_int))
                                    as ::core::ffi::c_double,
                            )
                        + (if (*plotmap.offset(home as isize)).star_pt.y
                            as ::core::ffi::c_int
                            > (*plotmap
                                .offset(
                                    C2Rust_Unnamed_11::AQUA_HELIX_DEFINED.0
                                        as ::core::ffi::c_int as isize,
                                ))
                                .star_pt
                                .y as ::core::ffi::c_int
                        {
                            1 as ::core::ffi::c_int
                        } else {
                            -1 as ::core::ffi::c_int
                        } * Jitter(strength, rand_val_y) as ::core::ffi::c_int)
                            as ::core::ffi::c_double) as COORD,
                };
            } else {
                location = point {
                    x: ((*plotmap.offset(home as isize)).star_pt.x as ::core::ffi::c_int
                        + if ((*plotmap.offset(home as isize)).star_pt.x
                            as ::core::ffi::c_int)
                            < (*plotmap
                                .offset(
                                    C2Rust_Unnamed_11::AQUA_HELIX_DEFINED.0
                                        as ::core::ffi::c_int as isize,
                                ))
                                .star_pt
                                .x as ::core::ffi::c_int
                        {
                            1 as ::core::ffi::c_int
                        } else {
                            -1 as ::core::ffi::c_int
                        }
                            * if rand_val_x as ::core::ffi::c_int
                                * rand_val_y as ::core::ffi::c_int
                                % 10 as ::core::ffi::c_int != 0
                            {
                                1 as ::core::ffi::c_int
                            } else {
                                -1 as ::core::ffi::c_int
                            } * Jitter(strength, rand_val_x) as ::core::ffi::c_int)
                        as COORD,
                    y: ((*plotmap.offset(home as isize)).star_pt.y as ::core::ffi::c_int
                        + if ((*plotmap.offset(home as isize)).star_pt.y
                            as ::core::ffi::c_int)
                            < (*plotmap
                                .offset(
                                    C2Rust_Unnamed_11::AQUA_HELIX_DEFINED.0
                                        as ::core::ffi::c_int as isize,
                                ))
                                .star_pt
                                .y as ::core::ffi::c_int
                        {
                            1 as ::core::ffi::c_int
                        } else {
                            -1 as ::core::ffi::c_int
                        }
                            * if (rand_val_x as ::core::ffi::c_int
                                + rand_val_y as ::core::ffi::c_int)
                                % 5 as ::core::ffi::c_int != 0
                            {
                                1 as ::core::ffi::c_int
                            } else {
                                -1 as ::core::ffi::c_int
                            } * Jitter(strength, rand_val_y) as ::core::ffi::c_int)
                        as COORD,
                };
            }
        }
        SPECIES_ID::UTWIG_ID => {
            if visit as ::core::ffi::c_int
                == C2Rust_Unnamed_11::SAMATRA_DEFINED.0 as ::core::ffi::c_int
            {
                location = point {
                    x: (warpoint.x as ::core::ffi::c_int as ::core::ffi::c_double
                        + (1200 as ::core::ffi::c_int
                            * ((*plotmap.offset(home as isize)).star_pt.x
                                as ::core::ffi::c_int - warpoint.x as ::core::ffi::c_int))
                            as ::core::ffi::c_double
                            / sqrt(
                                (((*plotmap.offset(home as isize)).star_pt.x
                                    as ::core::ffi::c_int - warpoint.x as ::core::ffi::c_int)
                                    * ((*plotmap.offset(home as isize)).star_pt.x
                                        as ::core::ffi::c_int - warpoint.x as ::core::ffi::c_int)
                                    + ((*plotmap.offset(home as isize)).star_pt.y
                                        as ::core::ffi::c_int - warpoint.y as ::core::ffi::c_int)
                                        * ((*plotmap.offset(home as isize)).star_pt.y
                                            as ::core::ffi::c_int - warpoint.y as ::core::ffi::c_int))
                                    as ::core::ffi::c_double,
                            )
                        + (if (*plotmap.offset(home as isize)).star_pt.x
                            as ::core::ffi::c_int
                            > (*plotmap
                                .offset(
                                    C2Rust_Unnamed_11::SUPOX_DEFINED.0 as ::core::ffi::c_int
                                        as isize,
                                ))
                                .star_pt
                                .x as ::core::ffi::c_int
                        {
                            1 as ::core::ffi::c_int
                        } else {
                            -1 as ::core::ffi::c_int
                        } * Jitter(strength, rand_val_x) as ::core::ffi::c_int)
                            as ::core::ffi::c_double) as COORD,
                    y: (warpoint.y as ::core::ffi::c_int as ::core::ffi::c_double
                        + (1200 as ::core::ffi::c_int
                            * ((*plotmap.offset(home as isize)).star_pt.y
                                as ::core::ffi::c_int - warpoint.y as ::core::ffi::c_int))
                            as ::core::ffi::c_double
                            / sqrt(
                                (((*plotmap.offset(home as isize)).star_pt.x
                                    as ::core::ffi::c_int - warpoint.x as ::core::ffi::c_int)
                                    * ((*plotmap.offset(home as isize)).star_pt.x
                                        as ::core::ffi::c_int - warpoint.x as ::core::ffi::c_int)
                                    + ((*plotmap.offset(home as isize)).star_pt.y
                                        as ::core::ffi::c_int - warpoint.y as ::core::ffi::c_int)
                                        * ((*plotmap.offset(home as isize)).star_pt.y
                                            as ::core::ffi::c_int - warpoint.y as ::core::ffi::c_int))
                                    as ::core::ffi::c_double,
                            )
                        + (if (*plotmap.offset(home as isize)).star_pt.y
                            as ::core::ffi::c_int
                            > (*plotmap
                                .offset(
                                    C2Rust_Unnamed_11::SUPOX_DEFINED.0 as ::core::ffi::c_int
                                        as isize,
                                ))
                                .star_pt
                                .y as ::core::ffi::c_int
                        {
                            1 as ::core::ffi::c_int
                        } else {
                            -1 as ::core::ffi::c_int
                        } * Jitter(strength, rand_val_y) as ::core::ffi::c_int)
                            as ::core::ffi::c_double) as COORD,
                };
            } else {
                location = point {
                    x: ((*plotmap.offset(home as isize)).star_pt.x as ::core::ffi::c_int
                        + if ((*plotmap.offset(home as isize)).star_pt.x
                            as ::core::ffi::c_int)
                            < (*plotmap
                                .offset(
                                    C2Rust_Unnamed_11::SUPOX_DEFINED.0 as ::core::ffi::c_int
                                        as isize,
                                ))
                                .star_pt
                                .x as ::core::ffi::c_int
                        {
                            1 as ::core::ffi::c_int
                        } else {
                            -1 as ::core::ffi::c_int
                        }
                            * if rand_val_x as ::core::ffi::c_int
                                * rand_val_y as ::core::ffi::c_int
                                % 10 as ::core::ffi::c_int != 0
                            {
                                1 as ::core::ffi::c_int
                            } else {
                                -1 as ::core::ffi::c_int
                            } * Jitter(strength, rand_val_x) as ::core::ffi::c_int)
                        as COORD,
                    y: ((*plotmap.offset(home as isize)).star_pt.y as ::core::ffi::c_int
                        + if ((*plotmap.offset(home as isize)).star_pt.y
                            as ::core::ffi::c_int)
                            < (*plotmap
                                .offset(
                                    C2Rust_Unnamed_11::SUPOX_DEFINED.0 as ::core::ffi::c_int
                                        as isize,
                                ))
                                .star_pt
                                .y as ::core::ffi::c_int
                        {
                            1 as ::core::ffi::c_int
                        } else {
                            -1 as ::core::ffi::c_int
                        }
                            * if (rand_val_x as ::core::ffi::c_int
                                + rand_val_y as ::core::ffi::c_int)
                                % 5 as ::core::ffi::c_int != 0
                            {
                                1 as ::core::ffi::c_int
                            } else {
                                -1 as ::core::ffi::c_int
                            } * Jitter(strength, rand_val_y) as ::core::ffi::c_int)
                        as COORD,
                };
            }
        }
        SPECIES_ID::VUX_ID => {
            location = point {
                x: ((*plotmap.offset(home as isize)).star_pt.x as ::core::ffi::c_int
                    + if rand_val_x as ::core::ffi::c_int % 2 as ::core::ffi::c_int != 0
                    {
                        1 as ::core::ffi::c_int
                    } else {
                        -1 as ::core::ffi::c_int
                    } * Jitter(strength, rand_val_x) as ::core::ffi::c_int) as COORD,
                y: ((*plotmap.offset(home as isize)).star_pt.y as ::core::ffi::c_int
                    + if rand_val_y as ::core::ffi::c_int % 2 as ::core::ffi::c_int != 0
                    {
                        1 as ::core::ffi::c_int
                    } else {
                        -1 as ::core::ffi::c_int
                    } * Jitter(strength, rand_val_y) as ::core::ffi::c_int) as COORD,
            };
        }
        SPECIES_ID::YEHAT_ID => {
            if visit as ::core::ffi::c_int
                == C2Rust_Unnamed_11::YEHAT_DEFINED.0 as ::core::ffi::c_int
            {
                location = point {
                    x: ((*plotmap.offset(home as isize)).star_pt.x as ::core::ffi::c_int
                        + if (*plotmap.offset(home as isize)).star_pt.x
                            as ::core::ffi::c_int
                            > (*plotmap
                                .offset(
                                    C2Rust_Unnamed_11::SHOFIXTI_DEFINED.0 as ::core::ffi::c_int
                                        as isize,
                                ))
                                .star_pt
                                .x as ::core::ffi::c_int
                        {
                            1 as ::core::ffi::c_int
                        } else {
                            -1 as ::core::ffi::c_int
                        }
                            * if rand_val_x as ::core::ffi::c_int
                                * rand_val_y as ::core::ffi::c_int
                                % 10 as ::core::ffi::c_int != 0
                            {
                                1 as ::core::ffi::c_int
                            } else {
                                -1 as ::core::ffi::c_int
                            } * Jitter(strength, rand_val_x) as ::core::ffi::c_int)
                        as COORD,
                    y: ((*plotmap.offset(home as isize)).star_pt.y as ::core::ffi::c_int
                        + if (*plotmap.offset(home as isize)).star_pt.y
                            as ::core::ffi::c_int
                            > (*plotmap
                                .offset(
                                    C2Rust_Unnamed_11::SHOFIXTI_DEFINED.0 as ::core::ffi::c_int
                                        as isize,
                                ))
                                .star_pt
                                .y as ::core::ffi::c_int
                        {
                            1 as ::core::ffi::c_int
                        } else {
                            -1 as ::core::ffi::c_int
                        }
                            * if (rand_val_x as ::core::ffi::c_int
                                + rand_val_y as ::core::ffi::c_int)
                                % 5 as ::core::ffi::c_int != 0
                            {
                                1 as ::core::ffi::c_int
                            } else {
                                -1 as ::core::ffi::c_int
                            } * Jitter(strength, rand_val_y) as ::core::ffi::c_int)
                        as COORD,
                };
            } else {
                location = point {
                    x: ((*plotmap.offset(home as isize)).star_pt.x as ::core::ffi::c_int
                        + if ((*plotmap.offset(home as isize)).star_pt.x
                            as ::core::ffi::c_int)
                            < (*plotmap
                                .offset(
                                    C2Rust_Unnamed_11::SHOFIXTI_DEFINED.0 as ::core::ffi::c_int
                                        as isize,
                                ))
                                .star_pt
                                .x as ::core::ffi::c_int
                        {
                            1 as ::core::ffi::c_int
                        } else {
                            -1 as ::core::ffi::c_int
                        }
                            * if rand_val_x as ::core::ffi::c_int
                                * rand_val_y as ::core::ffi::c_int
                                % 10 as ::core::ffi::c_int != 0
                            {
                                1 as ::core::ffi::c_int
                            } else {
                                -1 as ::core::ffi::c_int
                            } * Jitter(strength, rand_val_x) as ::core::ffi::c_int)
                        as COORD,
                    y: ((*plotmap.offset(home as isize)).star_pt.y as ::core::ffi::c_int
                        + if ((*plotmap.offset(home as isize)).star_pt.y
                            as ::core::ffi::c_int)
                            < (*plotmap
                                .offset(
                                    C2Rust_Unnamed_11::SHOFIXTI_DEFINED.0 as ::core::ffi::c_int
                                        as isize,
                                ))
                                .star_pt
                                .y as ::core::ffi::c_int
                        {
                            1 as ::core::ffi::c_int
                        } else {
                            -1 as ::core::ffi::c_int
                        }
                            * if (rand_val_x as ::core::ffi::c_int
                                + rand_val_y as ::core::ffi::c_int)
                                % 5 as ::core::ffi::c_int != 0
                            {
                                1 as ::core::ffi::c_int
                            } else {
                                -1 as ::core::ffi::c_int
                            } * Jitter(strength, rand_val_y) as ::core::ffi::c_int)
                        as COORD,
                };
            }
        }
        SPECIES_ID::DRUUGE_ID => {
            location = point {
                x: ((*plotmap.offset(home as isize)).star_pt.x as ::core::ffi::c_int
                    + if rand_val_x as ::core::ffi::c_int % 2 as ::core::ffi::c_int != 0
                    {
                        1 as ::core::ffi::c_int
                    } else {
                        -1 as ::core::ffi::c_int
                    } * Jitter(strength, rand_val_x) as ::core::ffi::c_int) as COORD,
                y: ((*plotmap.offset(home as isize)).star_pt.y as ::core::ffi::c_int
                    + if rand_val_y as ::core::ffi::c_int % 2 as ::core::ffi::c_int != 0
                    {
                        1 as ::core::ffi::c_int
                    } else {
                        -1 as ::core::ffi::c_int
                    } * Jitter(strength, rand_val_y) as ::core::ffi::c_int) as COORD,
            };
        }
        SPECIES_ID::ILWRATH_ID => {
            if visit as ::core::ffi::c_int
                == C2Rust_Unnamed_11::THRADD_DEFINED.0 as ::core::ffi::c_int
            {
                location = point {
                    x: (warpoint.x as ::core::ffi::c_int as ::core::ffi::c_double
                        + (500 as ::core::ffi::c_int
                            * ((*plotmap.offset(home as isize)).star_pt.x
                                as ::core::ffi::c_int - warpoint.x as ::core::ffi::c_int))
                            as ::core::ffi::c_double
                            / sqrt(
                                (((*plotmap.offset(home as isize)).star_pt.x
                                    as ::core::ffi::c_int - warpoint.x as ::core::ffi::c_int)
                                    * ((*plotmap.offset(home as isize)).star_pt.x
                                        as ::core::ffi::c_int - warpoint.x as ::core::ffi::c_int)
                                    + ((*plotmap.offset(home as isize)).star_pt.y
                                        as ::core::ffi::c_int - warpoint.y as ::core::ffi::c_int)
                                        * ((*plotmap.offset(home as isize)).star_pt.y
                                            as ::core::ffi::c_int - warpoint.y as ::core::ffi::c_int))
                                    as ::core::ffi::c_double,
                            )
                        + (if ((*plotmap.offset(home as isize)).star_pt.x
                            as ::core::ffi::c_int)
                            < (*plotmap.offset(visit as isize)).star_pt.x
                                as ::core::ffi::c_int
                        {
                            1 as ::core::ffi::c_int
                        } else {
                            -1 as ::core::ffi::c_int
                        } * Jitter(strength, rand_val_x) as ::core::ffi::c_int)
                            as ::core::ffi::c_double) as COORD,
                    y: (warpoint.y as ::core::ffi::c_int as ::core::ffi::c_double
                        + (500 as ::core::ffi::c_int
                            * ((*plotmap.offset(home as isize)).star_pt.y
                                as ::core::ffi::c_int - warpoint.y as ::core::ffi::c_int))
                            as ::core::ffi::c_double
                            / sqrt(
                                (((*plotmap.offset(home as isize)).star_pt.x
                                    as ::core::ffi::c_int - warpoint.x as ::core::ffi::c_int)
                                    * ((*plotmap.offset(home as isize)).star_pt.x
                                        as ::core::ffi::c_int - warpoint.x as ::core::ffi::c_int)
                                    + ((*plotmap.offset(home as isize)).star_pt.y
                                        as ::core::ffi::c_int - warpoint.y as ::core::ffi::c_int)
                                        * ((*plotmap.offset(home as isize)).star_pt.y
                                            as ::core::ffi::c_int - warpoint.y as ::core::ffi::c_int))
                                    as ::core::ffi::c_double,
                            )
                        + (if ((*plotmap.offset(home as isize)).star_pt.y
                            as ::core::ffi::c_int)
                            < (*plotmap.offset(visit as isize)).star_pt.y
                                as ::core::ffi::c_int
                        {
                            1 as ::core::ffi::c_int
                        } else {
                            -1 as ::core::ffi::c_int
                        } * Jitter(strength, rand_val_y) as ::core::ffi::c_int)
                            as ::core::ffi::c_double) as COORD,
                };
                snprintf(
                    &raw mut buf as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[UNICODE; 256]>(),
                    b"%s %05.1f : %05.1f\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"ILWRATH x THRADDASH\0".as_ptr() as *const ::core::ffi::c_char,
                    (location.x as ::core::ffi::c_float
                        / 10 as ::core::ffi::c_int as ::core::ffi::c_float)
                        as ::core::ffi::c_double,
                    (location.y as ::core::ffi::c_float
                        / 10 as ::core::ffi::c_int as ::core::ffi::c_float)
                        as ::core::ffi::c_double,
                );
            } else if visit as ::core::ffi::c_int
                == C2Rust_Unnamed_11::PKUNK_DEFINED.0 as ::core::ffi::c_int
            {
                warpoint = (*plotmap.offset(visit as isize)).star_pt;
                location = point {
                    x: (warpoint.x as ::core::ffi::c_int as ::core::ffi::c_double
                        + (1400 as ::core::ffi::c_int
                            * ((*plotmap.offset(home as isize)).star_pt.x
                                as ::core::ffi::c_int - warpoint.x as ::core::ffi::c_int))
                            as ::core::ffi::c_double
                            / sqrt(
                                (((*plotmap.offset(home as isize)).star_pt.x
                                    as ::core::ffi::c_int - warpoint.x as ::core::ffi::c_int)
                                    * ((*plotmap.offset(home as isize)).star_pt.x
                                        as ::core::ffi::c_int - warpoint.x as ::core::ffi::c_int)
                                    + ((*plotmap.offset(home as isize)).star_pt.y
                                        as ::core::ffi::c_int - warpoint.y as ::core::ffi::c_int)
                                        * ((*plotmap.offset(home as isize)).star_pt.y
                                            as ::core::ffi::c_int - warpoint.y as ::core::ffi::c_int))
                                    as ::core::ffi::c_double,
                            )
                        + (if ((*plotmap.offset(home as isize)).star_pt.x
                            as ::core::ffi::c_int)
                            < (*plotmap.offset(visit as isize)).star_pt.x
                                as ::core::ffi::c_int
                        {
                            1 as ::core::ffi::c_int
                        } else {
                            -1 as ::core::ffi::c_int
                        } * Jitter(strength, rand_val_x) as ::core::ffi::c_int)
                            as ::core::ffi::c_double) as COORD,
                    y: (warpoint.y as ::core::ffi::c_int as ::core::ffi::c_double
                        + (1400 as ::core::ffi::c_int
                            * ((*plotmap.offset(home as isize)).star_pt.y
                                as ::core::ffi::c_int - warpoint.y as ::core::ffi::c_int))
                            as ::core::ffi::c_double
                            / sqrt(
                                (((*plotmap.offset(home as isize)).star_pt.x
                                    as ::core::ffi::c_int - warpoint.x as ::core::ffi::c_int)
                                    * ((*plotmap.offset(home as isize)).star_pt.x
                                        as ::core::ffi::c_int - warpoint.x as ::core::ffi::c_int)
                                    + ((*plotmap.offset(home as isize)).star_pt.y
                                        as ::core::ffi::c_int - warpoint.y as ::core::ffi::c_int)
                                        * ((*plotmap.offset(home as isize)).star_pt.y
                                            as ::core::ffi::c_int - warpoint.y as ::core::ffi::c_int))
                                    as ::core::ffi::c_double,
                            )
                        + (if ((*plotmap.offset(home as isize)).star_pt.y
                            as ::core::ffi::c_int)
                            < (*plotmap.offset(visit as isize)).star_pt.y
                                as ::core::ffi::c_int
                        {
                            1 as ::core::ffi::c_int
                        } else {
                            -1 as ::core::ffi::c_int
                        } * Jitter(strength, rand_val_y) as ::core::ffi::c_int)
                            as ::core::ffi::c_double) as COORD,
                };
                snprintf(
                    &raw mut buf as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[UNICODE; 256]>(),
                    b"%s %05.1f : %05.1f\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"ILWRATH x PKUNK\0".as_ptr() as *const ::core::ffi::c_char,
                    (location.x as ::core::ffi::c_float
                        / 10 as ::core::ffi::c_int as ::core::ffi::c_float)
                        as ::core::ffi::c_double,
                    (location.y as ::core::ffi::c_float
                        / 10 as ::core::ffi::c_int as ::core::ffi::c_float)
                        as ::core::ffi::c_double,
                );
            } else {
                location = point {
                    x: ((*plotmap.offset(home as isize)).star_pt.x as ::core::ffi::c_int
                        + if rand_val_x as ::core::ffi::c_int % 2 as ::core::ffi::c_int
                            != 0
                        {
                            1 as ::core::ffi::c_int
                        } else {
                            -1 as ::core::ffi::c_int
                        } * Jitter(strength, rand_val_x) as ::core::ffi::c_int) as COORD,
                    y: ((*plotmap.offset(home as isize)).star_pt.y as ::core::ffi::c_int
                        + if rand_val_y as ::core::ffi::c_int % 2 as ::core::ffi::c_int
                            != 0
                        {
                            1 as ::core::ffi::c_int
                        } else {
                            -1 as ::core::ffi::c_int
                        } * Jitter(strength, rand_val_y) as ::core::ffi::c_int) as COORD,
                };
            }
        }
        SPECIES_ID::MYCON_ID => {
            if visit as ::core::ffi::c_int
                == C2Rust_Unnamed_11::MYCON_TRAP_DEFINED.0 as ::core::ffi::c_int
            {
                location = point {
                    x: ((*plotmap.offset(visit as isize)).star_pt.x as ::core::ffi::c_int
                        + if (*plotmap.offset(home as isize)).star_pt.x
                            as ::core::ffi::c_int
                            > (*plotmap.offset(visit as isize)).star_pt.x
                                as ::core::ffi::c_int
                        {
                            1 as ::core::ffi::c_int
                        } else {
                            -1 as ::core::ffi::c_int
                        } * Jitter(strength, rand_val_x) as ::core::ffi::c_int) as COORD,
                    y: ((*plotmap.offset(visit as isize)).star_pt.y as ::core::ffi::c_int
                        + if (*plotmap.offset(home as isize)).star_pt.y
                            as ::core::ffi::c_int
                            > (*plotmap.offset(visit as isize)).star_pt.y
                                as ::core::ffi::c_int
                        {
                            1 as ::core::ffi::c_int
                        } else {
                            -1 as ::core::ffi::c_int
                        } * Jitter(strength, rand_val_y) as ::core::ffi::c_int) as COORD,
                };
            } else {
                location = point {
                    x: ((*plotmap.offset(home as isize)).star_pt.x as ::core::ffi::c_int
                        + if ((*plotmap.offset(home as isize)).star_pt.x
                            as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                            < (*plotmap
                                .offset(
                                    C2Rust_Unnamed_11::SUN_DEVICE_DEFINED.0
                                        as ::core::ffi::c_int as isize,
                                ))
                                .star_pt
                                .x as ::core::ffi::c_int
                                + (*plotmap
                                    .offset(
                                        C2Rust_Unnamed_11::EGG_CASE2_DEFINED.0 as ::core::ffi::c_int
                                            as isize,
                                    ))
                                    .star_pt
                                    .x as ::core::ffi::c_int / 2 as ::core::ffi::c_int
                                + (*plotmap
                                    .offset(
                                        C2Rust_Unnamed_11::EGG_CASE1_DEFINED.0 as ::core::ffi::c_int
                                            as isize,
                                    ))
                                    .star_pt
                                    .x as ::core::ffi::c_int / 3 as ::core::ffi::c_int
                                + (*plotmap
                                    .offset(
                                        C2Rust_Unnamed_11::EGG_CASE0_DEFINED.0 as ::core::ffi::c_int
                                            as isize,
                                    ))
                                    .star_pt
                                    .x as ::core::ffi::c_int / 6 as ::core::ffi::c_int
                        {
                            1 as ::core::ffi::c_int
                        } else {
                            -1 as ::core::ffi::c_int
                        } * Jitter(strength, rand_val_x) as ::core::ffi::c_int) as COORD,
                    y: ((*plotmap.offset(home as isize)).star_pt.y as ::core::ffi::c_int
                        + if ((*plotmap.offset(home as isize)).star_pt.y
                            as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                            < (*plotmap
                                .offset(
                                    C2Rust_Unnamed_11::SUN_DEVICE_DEFINED.0
                                        as ::core::ffi::c_int as isize,
                                ))
                                .star_pt
                                .y as ::core::ffi::c_int
                                + (*plotmap
                                    .offset(
                                        C2Rust_Unnamed_11::EGG_CASE2_DEFINED.0 as ::core::ffi::c_int
                                            as isize,
                                    ))
                                    .star_pt
                                    .y as ::core::ffi::c_int / 2 as ::core::ffi::c_int
                                + (*plotmap
                                    .offset(
                                        C2Rust_Unnamed_11::EGG_CASE1_DEFINED.0 as ::core::ffi::c_int
                                            as isize,
                                    ))
                                    .star_pt
                                    .y as ::core::ffi::c_int / 3 as ::core::ffi::c_int
                                + (*plotmap
                                    .offset(
                                        C2Rust_Unnamed_11::EGG_CASE0_DEFINED.0 as ::core::ffi::c_int
                                            as isize,
                                    ))
                                    .star_pt
                                    .y as ::core::ffi::c_int / 6 as ::core::ffi::c_int
                        {
                            1 as ::core::ffi::c_int
                        } else {
                            -1 as ::core::ffi::c_int
                        } * Jitter(strength, rand_val_y) as ::core::ffi::c_int) as COORD,
                };
            }
        }
        SPECIES_ID::UMGAH_ID => {
            if visit as ::core::ffi::c_int
                == C2Rust_Unnamed_11::SAMATRA_DEFINED.0 as ::core::ffi::c_int
            {
                location = point {
                    x: (warpoint.x as ::core::ffi::c_int as ::core::ffi::c_double
                        + (2000 as ::core::ffi::c_int
                            * ((*plotmap.offset(home as isize)).star_pt.x
                                as ::core::ffi::c_int - warpoint.x as ::core::ffi::c_int))
                            as ::core::ffi::c_double
                            / sqrt(
                                (((*plotmap.offset(home as isize)).star_pt.x
                                    as ::core::ffi::c_int - warpoint.x as ::core::ffi::c_int)
                                    * ((*plotmap.offset(home as isize)).star_pt.x
                                        as ::core::ffi::c_int - warpoint.x as ::core::ffi::c_int)
                                    + ((*plotmap.offset(home as isize)).star_pt.y
                                        as ::core::ffi::c_int - warpoint.y as ::core::ffi::c_int)
                                        * ((*plotmap.offset(home as isize)).star_pt.y
                                            as ::core::ffi::c_int - warpoint.y as ::core::ffi::c_int))
                                    as ::core::ffi::c_double,
                            )
                        + (if (*plotmap.offset(home as isize)).star_pt.x
                            as ::core::ffi::c_int
                            > (*plotmap
                                .offset(
                                    C2Rust_Unnamed_11::TALKING_PET_DEFINED.0
                                        as ::core::ffi::c_int as isize,
                                ))
                                .star_pt
                                .x as ::core::ffi::c_int
                        {
                            1 as ::core::ffi::c_int
                        } else {
                            -1 as ::core::ffi::c_int
                        } * Jitter(strength, rand_val_x) as ::core::ffi::c_int)
                            as ::core::ffi::c_double) as COORD,
                    y: (warpoint.y as ::core::ffi::c_int as ::core::ffi::c_double
                        + (2000 as ::core::ffi::c_int
                            * ((*plotmap.offset(home as isize)).star_pt.y
                                as ::core::ffi::c_int - warpoint.y as ::core::ffi::c_int))
                            as ::core::ffi::c_double
                            / sqrt(
                                (((*plotmap.offset(home as isize)).star_pt.x
                                    as ::core::ffi::c_int - warpoint.x as ::core::ffi::c_int)
                                    * ((*plotmap.offset(home as isize)).star_pt.x
                                        as ::core::ffi::c_int - warpoint.x as ::core::ffi::c_int)
                                    + ((*plotmap.offset(home as isize)).star_pt.y
                                        as ::core::ffi::c_int - warpoint.y as ::core::ffi::c_int)
                                        * ((*plotmap.offset(home as isize)).star_pt.y
                                            as ::core::ffi::c_int - warpoint.y as ::core::ffi::c_int))
                                    as ::core::ffi::c_double,
                            )
                        + (if (*plotmap.offset(home as isize)).star_pt.y
                            as ::core::ffi::c_int
                            > (*plotmap
                                .offset(
                                    C2Rust_Unnamed_11::TALKING_PET_DEFINED.0
                                        as ::core::ffi::c_int as isize,
                                ))
                                .star_pt
                                .y as ::core::ffi::c_int
                        {
                            1 as ::core::ffi::c_int
                        } else {
                            -1 as ::core::ffi::c_int
                        } * Jitter(strength, rand_val_y) as ::core::ffi::c_int)
                            as ::core::ffi::c_double) as COORD,
                };
            } else {
                location = point {
                    x: ((*plotmap.offset(home as isize)).star_pt.x as ::core::ffi::c_int
                        + if rand_val_x as ::core::ffi::c_int % 2 as ::core::ffi::c_int
                            != 0
                        {
                            1 as ::core::ffi::c_int
                        } else {
                            -1 as ::core::ffi::c_int
                        } * Jitter(strength, rand_val_x) as ::core::ffi::c_int) as COORD,
                    y: ((*plotmap.offset(home as isize)).star_pt.y as ::core::ffi::c_int
                        + if rand_val_y as ::core::ffi::c_int % 2 as ::core::ffi::c_int
                            != 0
                        {
                            1 as ::core::ffi::c_int
                        } else {
                            -1 as ::core::ffi::c_int
                        } * Jitter(strength, rand_val_y) as ::core::ffi::c_int) as COORD,
                };
            }
        }
        SPECIES_ID::UR_QUAN_ID => {
            location = point {
                x: ((*plotmap.offset(home as isize)).star_pt.x as ::core::ffi::c_int
                    + if rand_val_y as ::core::ffi::c_int % 2 as ::core::ffi::c_int != 0
                    {
                        1 as ::core::ffi::c_int
                    } else {
                        -1 as ::core::ffi::c_int
                    } * Jitter(strength, rand_val_x) as ::core::ffi::c_int
                        / 2 as ::core::ffi::c_int) as COORD,
                y: ((*plotmap.offset(home as isize)).star_pt.y as ::core::ffi::c_int
                    + if rand_val_x as ::core::ffi::c_int % 2 as ::core::ffi::c_int != 0
                    {
                        1 as ::core::ffi::c_int
                    } else {
                        -1 as ::core::ffi::c_int
                    }
                    + Jitter(strength, rand_val_y) as ::core::ffi::c_int
                        / 2 as ::core::ffi::c_int) as COORD,
            };
        }
        SPECIES_ID::KOHR_AH_ID => {
            location = point {
                x: ((*plotmap.offset(home as isize)).star_pt.x as ::core::ffi::c_int
                    - if rand_val_x as ::core::ffi::c_int % 2 as ::core::ffi::c_int != 0
                    {
                        1 as ::core::ffi::c_int
                    } else {
                        -1 as ::core::ffi::c_int
                    } * Jitter(strength, rand_val_y) as ::core::ffi::c_int
                        / 2 as ::core::ffi::c_int) as COORD,
                y: ((*plotmap.offset(home as isize)).star_pt.y as ::core::ffi::c_int
                    + if rand_val_y as ::core::ffi::c_int % 2 as ::core::ffi::c_int != 0
                    {
                        1 as ::core::ffi::c_int
                    } else {
                        -1 as ::core::ffi::c_int
                    }
                    + Jitter(strength, rand_val_x) as ::core::ffi::c_int
                        / 2 as ::core::ffi::c_int) as COORD,
            };
        }
        SPECIES_ID::ZOQFOTPIK_ID => {
            location = point {
                x: ((*plotmap.offset(home as isize)).star_pt.x as ::core::ffi::c_int
                    + if (*plotmap.offset(home as isize)).star_pt.x as ::core::ffi::c_int
                        > (*plotmap
                            .offset(
                                C2Rust_Unnamed_11::SAMATRA_DEFINED.0 as ::core::ffi::c_int
                                    as isize,
                            ))
                            .star_pt
                            .x as ::core::ffi::c_int
                    {
                        1 as ::core::ffi::c_int
                    } else {
                        -1 as ::core::ffi::c_int
                    }
                        * ((*FleetPtr).actual_strength as ::core::ffi::c_int
                            * 11 as ::core::ffi::c_int / 2 as ::core::ffi::c_int
                            - Jitter(strength, rand_val_x) as ::core::ffi::c_int)
                        / 2 as ::core::ffi::c_int) as COORD,
                y: ((*plotmap.offset(home as isize)).star_pt.y as ::core::ffi::c_int
                    + if (*plotmap.offset(home as isize)).star_pt.y as ::core::ffi::c_int
                        > (*plotmap
                            .offset(
                                C2Rust_Unnamed_11::SAMATRA_DEFINED.0 as ::core::ffi::c_int
                                    as isize,
                            ))
                            .star_pt
                            .y as ::core::ffi::c_int
                    {
                        1 as ::core::ffi::c_int
                    } else {
                        -1 as ::core::ffi::c_int
                    }
                        * ((*FleetPtr).actual_strength as ::core::ffi::c_int
                            * 11 as ::core::ffi::c_int / 2 as ::core::ffi::c_int
                            - Jitter(strength, rand_val_y) as ::core::ffi::c_int)
                        / 2 as ::core::ffi::c_int) as COORD,
            };
        }
        SPECIES_ID::SYREEN_ID => {
            if visit as ::core::ffi::c_int
                == C2Rust_Unnamed_11::MYCON_TRAP_DEFINED.0 as ::core::ffi::c_int
            {
                location = point {
                    x: ((*plotmap.offset(visit as isize)).star_pt.x as ::core::ffi::c_int
                        + if rand_val_x as ::core::ffi::c_int % 2 as ::core::ffi::c_int
                            != 0
                        {
                            1 as ::core::ffi::c_int
                        } else {
                            -1 as ::core::ffi::c_int
                        } * Jitter(strength, rand_val_x) as ::core::ffi::c_int) as COORD,
                    y: ((*plotmap.offset(visit as isize)).star_pt.y as ::core::ffi::c_int
                        + if rand_val_y as ::core::ffi::c_int % 2 as ::core::ffi::c_int
                            != 0
                        {
                            1 as ::core::ffi::c_int
                        } else {
                            -1 as ::core::ffi::c_int
                        } * Jitter(strength, rand_val_y) as ::core::ffi::c_int) as COORD,
                };
            } else {
                location = point {
                    x: ((*plotmap.offset(home as isize)).star_pt.x as ::core::ffi::c_int
                        + if rand_val_x as ::core::ffi::c_int % 2 as ::core::ffi::c_int
                            != 0
                        {
                            1 as ::core::ffi::c_int
                        } else {
                            -1 as ::core::ffi::c_int
                        } * Jitter(strength, rand_val_x) as ::core::ffi::c_int) as COORD,
                    y: ((*plotmap.offset(home as isize)).star_pt.y as ::core::ffi::c_int
                        + if rand_val_y as ::core::ffi::c_int % 2 as ::core::ffi::c_int
                            != 0
                        {
                            1 as ::core::ffi::c_int
                        } else {
                            -1 as ::core::ffi::c_int
                        } * Jitter(strength, rand_val_y) as ::core::ffi::c_int) as COORD,
                };
            }
        }
        SPECIES_ID::ANDROSYNTH_ID => {
            location = point {
                x: ((*plotmap.offset(home as isize)).star_pt.x as ::core::ffi::c_int
                    + if rand_val_x as ::core::ffi::c_int % 2 as ::core::ffi::c_int != 0
                    {
                        1 as ::core::ffi::c_int
                    } else {
                        -1 as ::core::ffi::c_int
                    } * Jitter(strength, rand_val_y) as ::core::ffi::c_int) as COORD,
                y: ((*plotmap.offset(home as isize)).star_pt.y as ::core::ffi::c_int
                    + if rand_val_y as ::core::ffi::c_int % 2 as ::core::ffi::c_int != 0
                    {
                        1 as ::core::ffi::c_int
                    } else {
                        -1 as ::core::ffi::c_int
                    } * Jitter(strength, rand_val_x) as ::core::ffi::c_int) as COORD,
            };
        }
        SPECIES_ID::CHENJESU_ID => {
            location = point {
                x: ((*plotmap.offset(home as isize)).star_pt.x as ::core::ffi::c_int
                    + if rand_val_x as ::core::ffi::c_int % 2 as ::core::ffi::c_int != 0
                    {
                        1 as ::core::ffi::c_int
                    } else {
                        -1 as ::core::ffi::c_int
                    } * Jitter(strength, rand_val_x) as ::core::ffi::c_int) as COORD,
                y: ((*plotmap.offset(home as isize)).star_pt.y as ::core::ffi::c_int
                    + if rand_val_y as ::core::ffi::c_int % 2 as ::core::ffi::c_int != 0
                    {
                        1 as ::core::ffi::c_int
                    } else {
                        -1 as ::core::ffi::c_int
                    } * Jitter(strength, rand_val_y) as ::core::ffi::c_int) as COORD,
            };
        }
        SPECIES_ID::MMRNMHRM_ID => {
            location = point {
                x: ((*plotmap.offset(home as isize)).star_pt.x as ::core::ffi::c_int
                    + if rand_val_x as ::core::ffi::c_int % 2 as ::core::ffi::c_int != 0
                    {
                        1 as ::core::ffi::c_int
                    } else {
                        -1 as ::core::ffi::c_int
                    } * Jitter(strength, rand_val_y) as ::core::ffi::c_int) as COORD,
                y: ((*plotmap.offset(home as isize)).star_pt.y as ::core::ffi::c_int
                    + if rand_val_y as ::core::ffi::c_int % 2 as ::core::ffi::c_int != 0
                    {
                        1 as ::core::ffi::c_int
                    } else {
                        -1 as ::core::ffi::c_int
                    } * Jitter(strength, rand_val_x) as ::core::ffi::c_int) as COORD,
            };
        }
        SPECIES_ID::MELNORME_ID | SPECIES_ID::SLYLANDRO_ID | _ => {}
    }
    if (location.x as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
        location.x = 0 as COORD;
    }
    if location.x as ::core::ffi::c_int >= 9999 as ::core::ffi::c_int {
        location.x = (9999 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as COORD;
    }
    if (location.y as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
        location.y = 0 as COORD;
    }
    if location.y as ::core::ffi::c_int >= 9999 as ::core::ffi::c_int {
        location.y = (9999 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as COORD;
    }
    if !StarGenRNG.is_null() && myRNG.0 != 0 {
        RandomContext_Delete(StarGenRNG);
        StarGenRNG = ::core::ptr::null_mut::<RandomContext>();
    }
    return location;
}
#[inline]
unsafe extern "C" fn LockMasterShip(
    mut pq: *const QUEUE,
    mut h: HMASTERSHIP,
) -> *mut MASTER_SHIP_INFO {
    ((*pq).object_size as usize == ::core::mem::size_of::<MASTER_SHIP_INFO>()
        || {
            __assert_fail(
                b"GetLinkSize (pq) == sizeof (MASTER_SHIP_INFO)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/uqm/master.h\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                50 as ::core::ffi::c_int,
                b"LockMasterShip\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    return LockLink(pq, h) as *mut MASTER_SHIP_INFO;
}
