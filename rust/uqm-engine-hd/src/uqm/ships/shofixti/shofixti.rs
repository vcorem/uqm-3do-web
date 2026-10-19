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

// Rust translation of src/uqm/ships/shofixti/shofixti.c.
extern "C" {
    pub type string_table_entry;
    pub type frame_desc;
    pub type tfb_soundsample;
    fn __assert_fail(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    ) -> !;
    fn PutQueue(pq: *mut QUEUE, hLink: HLINK);
    fn GetFrameCount(Frame: FRAME) -> COUNT;
    fn IncFrameIndex(Frame: FRAME) -> FRAME;
    fn SetAbsStringTableIndex(String: STRING, StringTableIndex: COUNT) -> STRING;
    fn memset(
        _: *mut ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    static mut resolutionFactor: ::core::ffi::c_uint;
    static mut optGodModes: ::core::ffi::c_int;
    static mut disp_q: QUEUE;
    fn AllocElement() -> HELEMENT;
    fn PlotIntercept(
        ElementPtr0: *mut ELEMENT,
        ElementPtr1: *mut ELEMENT,
        max_turns: COUNT,
        margin_of_error: COUNT,
    ) -> COUNT;
    fn getGameStateUint(name: *const ::core::ffi::c_char) -> DWORD;
    static mut GlobData: GLOBDATA;
    fn TFB_Random() -> DWORD;
    fn square_root(value: DWORD) -> COUNT;
    static mut PlayerControl: [BYTE; 0];
    fn PlaySound(
        S: SOUND,
        Pos: SoundPosition,
        PositionalObject: *mut ELEMENT,
        Priority: BYTE,
    );
    fn CalcSoundPosition(ElementPtr: *mut ELEMENT) -> SoundPosition;
    fn DeltaCrew(ElementPtr: *mut ELEMENT, crew_delta: SIZE) -> BOOLEAN;
    fn initialize_missile(pMissileBlock: *mut MISSILE_BLOCK) -> HELEMENT;
    fn ship_intelligence(
        ShipPtr: *mut ELEMENT,
        ObjectsOfConcern: *mut EVALUATE_DESC,
        ConcernCounter: COUNT,
    );
    fn StopAllBattleMusic();
    fn FindAliveStarShip(deadShip: *mut ELEMENT) -> *mut STARSHIP;
    fn GetWinnerStarShip() -> *mut STARSHIP;
    fn SetWinnerStarShip(winner: *mut STARSHIP);
    fn RecordShipDeath(deadShip: *mut ELEMENT);
    fn StartShipExplosion(ShipPtr: *mut ELEMENT, playSound: bool);
    static mut CurStarDescPtr: *mut STAR_DESC;
    fn intruder_preprocess(ElementPtr: *mut ELEMENT);
    fn marine_collision(
        ElementPtr0: *mut ELEMENT,
        pPt0: *mut POINT,
        ElementPtr1: *mut ELEMENT,
        pPt1: *mut POINT,
    );
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct gfx_object(pub ::core::ffi::c_uint);
impl gfx_object {
    pub const POINT_PRIM: Self = Self(0);
    pub const STAMP_PRIM: Self = Self(1);
    pub const STAMPFILL_PRIM: Self = Self(2);
    pub const LINE_PRIM: Self = Self(3);
    pub const TEXT_PRIM: Self = Self(4);
    pub const RECT_PRIM: Self = Self(5);
    pub const RECTFILL_PRIM: Self = Self(6);
    pub const POINT_PRIM_HD: Self = Self(7);
    pub const NUM_PRIMS: Self = Self(8);
}
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
pub struct C2Rust_Unnamed(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed {
    pub const OPTVAL_NO_GOD: Self = Self(0);
    pub const OPTVAL_INF_ENERGY: Self = Self(1);
    pub const OPTVAL_INF_HEALTH: Self = Self(2);
    pub const OPTVAL_FULL_GOD: Self = Self(3);
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
pub type HELEMENT = HLINK;
pub type ELEMENT_FLAGS = UWORD;
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
pub type STATE = state;
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
pub type ElementProcessFunc = unsafe extern "C" fn(*mut ELEMENT) -> ();
pub type ELEMENT = element;
pub type ElementCollisionFunc = unsafe extern "C" fn(
    *mut ELEMENT,
    *mut POINT,
    *mut ELEMENT,
    *mut POINT,
) -> ();
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
pub struct soundposition {
    pub positional: BOOLEAN,
    pub x: ::core::ffi::c_int,
    pub y: ::core::ffi::c_int,
}
pub type SoundPosition = soundposition;
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
pub struct C2Rust_Unnamed_7(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_7 {
    pub const ENEMY_SHIP_INDEX: Self = Self(0);
    pub const CREW_OBJECT_INDEX: Self = Self(1);
    pub const ENEMY_WEAPON_INDEX: Self = Self(2);
    pub const GRAVITY_MASS_INDEX: Self = Self(3);
    pub const FIRST_EMPTY_INDEX: Self = Self(4);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MISSILE_BLOCK {
    pub cx: COORD,
    pub cy: COORD,
    pub flags: ELEMENT_FLAGS,
    pub sender: SIZE,
    pub pixoffs: SIZE,
    pub speed: SIZE,
    pub hit_points: SIZE,
    pub damage: SIZE,
    pub face: COUNT,
    pub index: COUNT,
    pub life: COUNT,
    pub farray: *mut FRAME,
    pub preprocess_func: Option<unsafe extern "C" fn(*mut ELEMENT) -> ()>,
    pub blast_offs: SIZE,
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
pub struct C2Rust_Unnamed_8(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_8 {
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
                    b"src/uqm/displist.h\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"src/uqm/displist.h\0".as_ptr() as *const ::core::ffi::c_char,
                    88 as ::core::ffi::c_int,
                    b"UnlockLink\0".as_ptr() as *const ::core::ffi::c_char,
                );
                0 as ::core::ffi::c_int != 0
            }) as ::core::ffi::c_int;
    }
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
unsafe extern "C" fn antiCheat(
    mut ElementPtr: *mut ELEMENT,
    mut SwapBool: BOOLEAN,
    mut GodModeType: ::core::ffi::c_int,
) -> BOOLEAN {
    return BOOLEAN(
        (!(*(&raw mut PlayerControl as *mut BYTE).offset(0isize) as ::core::ffi::c_int
            & (((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as BYTE
                as ::core::ffi::c_int
                | ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as BYTE
                    as ::core::ffi::c_int) != 0
            && *(&raw mut PlayerControl as *mut BYTE).offset(1isize)
                as ::core::ffi::c_int
                & (((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as BYTE
                    as ::core::ffi::c_int
                    | ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as BYTE
                        as ::core::ffi::c_int) != 0)
            && (optGodModes == GodModeType
                && (*(&raw mut PlayerControl as *mut BYTE).offset(0isize)
                    as ::core::ffi::c_int
                    & (((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as BYTE
                        as ::core::ffi::c_int
                        | ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as BYTE
                            as ::core::ffi::c_int) != 0
                    && (*ElementPtr).playerNr as ::core::ffi::c_int
                        == if SwapBool.0 != 0 {
                            0 as ::core::ffi::c_int
                        } else {
                            1 as ::core::ffi::c_int
                        }
                    || *(&raw mut PlayerControl as *mut BYTE).offset(1isize)
                        as ::core::ffi::c_int
                        & (((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as BYTE
                            as ::core::ffi::c_int
                            | ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                                as BYTE as ::core::ffi::c_int) != 0
                        && (*ElementPtr).playerNr as ::core::ffi::c_int
                            == if SwapBool.0 != 0 {
                                1 as ::core::ffi::c_int
                            } else {
                                0 as ::core::ffi::c_int
                            }))) as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
}
static mut shofixti_desc: RACE_DESC = unsafe {
    race_desc {
        c2rust_padding: [0; 8],
        c2rust_padding_0: [0; 10],
        c2rust_padding_1: [0; 2],
        c2rust_padding_2: [0; 8],
        ship_info: SHIP_INFO {
            idStr: b"scout\0".as_ptr() as *const ::core::ffi::c_char,
            ship_flags: ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as UWORD,
            ship_cost: 5 as BYTE,
            crew_level: 6 as COUNT,
            max_crew: 6 as COUNT,
            energy_level: 4 as BYTE,
            max_energy: 4 as BYTE,
            race_strings_rsc: b"ship.shofixti.text\0".as_ptr() as RESOURCE,
            icons_rsc: b"ship.shofixti.icons\0".as_ptr() as RESOURCE,
            melee_icon_rsc: b"ship.shofixti.meleeicons\0".as_ptr() as RESOURCE,
            race_strings: ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>(),
            icons: ::core::ptr::null_mut::<FRAME_DESC>(),
            melee_icon: ::core::ptr::null_mut::<FRAME_DESC>(),
        },
        fleet: FLEET_STUFF {
            strength: 0 as COUNT,
            known_loc: point {
                x: 0 as COORD,
                y: 0 as COORD,
            },
        },
        characteristics: CHARACTERISTIC_STUFF {
            max_thrust: 35 as COUNT,
            thrust_increment: 5 as COUNT,
            energy_regeneration: 1 as BYTE,
            weapon_energy_cost: 1 as BYTE,
            special_energy_cost: 0 as BYTE,
            energy_wait: 9 as BYTE,
            turn_wait: 1 as BYTE,
            thrust_wait: 0 as BYTE,
            weapon_wait: 3 as BYTE,
            special_wait: 0 as BYTE,
            ship_mass: 1 as BYTE,
        },
        ship_data: DATA_STUFF {
            ship_rsc: [
                b"ship.shofixti.graphics.scout.large\0".as_ptr() as RESOURCE,
                b"ship.shofixti.graphics.scout.medium\0".as_ptr() as RESOURCE,
                b"ship.shofixti.graphics.scout.small\0".as_ptr() as RESOURCE,
            ],
            weapon_rsc: [
                b"ship.shofixti.graphics.missile.large\0".as_ptr() as RESOURCE,
                b"ship.shofixti.graphics.missile.medium\0".as_ptr() as RESOURCE,
                b"ship.shofixti.graphics.missile.small\0".as_ptr() as RESOURCE,
            ],
            special_rsc: [
                b"ship.shofixti.graphics.destruct.large\0".as_ptr() as RESOURCE,
                b"ship.shofixti.graphics.destruct.medium\0".as_ptr() as RESOURCE,
                b"ship.shofixti.graphics.destruct.small\0".as_ptr() as RESOURCE,
            ],
            captain_control: captain_stuff {
                captain_rsc: b"ship.shofixti.graphics.captain\0".as_ptr() as RESOURCE,
                background: ::core::ptr::null_mut::<FRAME_DESC>(),
                turn: ::core::ptr::null_mut::<FRAME_DESC>(),
                thrust: ::core::ptr::null_mut::<FRAME_DESC>(),
                weapon: ::core::ptr::null_mut::<FRAME_DESC>(),
                special: ::core::ptr::null_mut::<FRAME_DESC>(),
                tl_offset: 0 as BYTE,
                tr_offset: 0 as BYTE,
                thrust_offset: 0 as BYTE,
                weapon_offset: 0 as BYTE,
                special_offset: 0 as BYTE,
            },
            victory_ditty_rsc: b"ship.shofixti.ditty\0".as_ptr() as RESOURCE,
            ship_sounds_rsc: b"ship.shofixti.sounds\0".as_ptr() as RESOURCE,
            ship: [
                ::core::ptr::null_mut::<FRAME_DESC>(),
                ::core::ptr::null_mut::<FRAME_DESC>(),
                ::core::ptr::null_mut::<FRAME_DESC>(),
            ],
            weapon: [
                ::core::ptr::null_mut::<FRAME_DESC>(),
                ::core::ptr::null_mut::<FRAME_DESC>(),
                ::core::ptr::null_mut::<FRAME_DESC>(),
            ],
            special: [
                ::core::ptr::null_mut::<FRAME_DESC>(),
                ::core::ptr::null_mut::<FRAME_DESC>(),
                ::core::ptr::null_mut::<FRAME_DESC>(),
            ],
            victory_ditty: ::core::ptr::null_mut::<*mut TFB_SoundSample>(),
            ship_sounds: ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>(),
        },
        cyborg_control: INTEL_STUFF {
            ManeuverabilityIndex: 0 as COUNT,
            WeaponRange: (((24 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                * 10 as ::core::ffi::c_int) as COUNT,
            intelligence_func: None,
        },
        uninit_func: ::core::mem::transmute::<
            *mut ::core::ffi::c_void,
            Option<UNINIT_FUNC>,
        >(::core::ptr::null_mut::<::core::ffi::c_void>()),
        preprocess_func: ::core::mem::transmute::<
            *mut ::core::ffi::c_void,
            Option<PREPROCESS_FUNC>,
        >(::core::ptr::null_mut::<::core::ffi::c_void>()),
        postprocess_func: ::core::mem::transmute::<
            *mut ::core::ffi::c_void,
            Option<POSTPROCESS_FUNC>,
        >(::core::ptr::null_mut::<::core::ffi::c_void>()),
        init_weapon_func: ::core::mem::transmute::<
            *mut ::core::ffi::c_void,
            Option<INIT_WEAPON_FUNC>,
        >(::core::ptr::null_mut::<::core::ffi::c_void>()),
        data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        CodeRef: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    }
};
unsafe extern "C" fn initialize_standard_missile(
    mut ShipPtr: *mut ELEMENT,
    mut MissileArray: *mut HELEMENT,
) -> COUNT {
    let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
    let mut MissileBlock: MISSILE_BLOCK = MISSILE_BLOCK {
        cx: 0,
        cy: 0,
        flags: 0,
        sender: 0,
        pixoffs: 0,
        speed: 0,
        hit_points: 0,
        damage: 0,
        face: 0,
        index: 0,
        life: 0,
        farray: ::core::ptr::null_mut::<FRAME>(),
        preprocess_func: None,
        blast_offs: 0,
    };
    StarShipPtr = (*ShipPtr).pParent as *mut STARSHIP;
    MissileBlock.cx = (*ShipPtr).next.location.x;
    MissileBlock.cy = (*ShipPtr).next.location.y;
    MissileBlock.farray = &raw mut (*(*StarShipPtr).RaceDescPtr).ship_data.weapon
        as *mut FRAME;
    MissileBlock.index = (*StarShipPtr).ShipFacing;
    MissileBlock.face = MissileBlock.index;
    MissileBlock.sender = (*ShipPtr).playerNr;
    MissileBlock.flags = ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
        as ELEMENT_FLAGS;
    MissileBlock.pixoffs = (if if resolutionFactor != 2 as ::core::ffi::c_uint {
        BOOLEAN::FALSE.0 as ::core::ffi::c_int
    } else {
        BOOLEAN::TRUE.0 as ::core::ffi::c_int
    } == 0
    {
        15 as ::core::ffi::c_int
    } else {
        51 as ::core::ffi::c_int
    }) as SIZE;
    MissileBlock.speed = (((24 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
        << resolutionFactor) as SIZE;
    MissileBlock.hit_points = 1 as SIZE;
    MissileBlock.damage = 1 as SIZE;
    MissileBlock.life = 10 as COUNT;
    MissileBlock.preprocess_func = None;
    MissileBlock.blast_offs = ((1 as ::core::ffi::c_int) << resolutionFactor) as SIZE;
    *MissileArray.offset(0isize) = initialize_missile(&raw mut MissileBlock);
    return 1 as COUNT;
}
unsafe extern "C" fn destruct_preprocess(mut ElementPtr: *mut ELEMENT) {
    let mut lpPrim: *mut PRIMITIVE = ::core::ptr::null_mut::<PRIMITIVE>();
    lpPrim = GlobData.Game_state.DisplayArray.offset((*ElementPtr).PrimIndex as isize);
    (*ElementPtr).state_flags = ((*ElementPtr).state_flags as ::core::ffi::c_int
        | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as ELEMENT_FLAGS;
    if (*ElementPtr).c2rust_unnamed.life_span as ::core::ffi::c_int
        > 12 as ::core::ffi::c_int * 3 as ::core::ffi::c_int - 3 as ::core::ffi::c_int
    {
        (*lpPrim).Type = gfx_object::STAMPFILL_PRIM.0 as ::core::ffi::c_int
            as GRAPHICS_PRIM;
        if (*ElementPtr).c2rust_unnamed.life_span as ::core::ffi::c_int
            == 12 as ::core::ffi::c_int * 3 as ::core::ffi::c_int
                - 3 as ::core::ffi::c_int + 2 as ::core::ffi::c_int
        {
            (*lpPrim).color = colorFromRgb15(0x1f as BYTE, 0x1f as BYTE, 0xa as BYTE);
        } else {
            (*lpPrim).color = colorFromRgb15(0x1f as BYTE, 0x1f as BYTE, 0x1f as BYTE);
        }
    } else if ((*ElementPtr).c2rust_unnamed.life_span as ::core::ffi::c_int)
        < 12 as ::core::ffi::c_int * 3 as ::core::ffi::c_int - 3 as ::core::ffi::c_int
    {
        let mut color: Color = (*lpPrim).color;
        (*ElementPtr).next.image.frame = IncFrameIndex(
            (*ElementPtr).current.image.frame,
        );
        if sameColor(color, colorFromRgb15(0x1f as BYTE, 0x1f as BYTE, 0x1f as BYTE)) {
            (*lpPrim).color = colorFromRgb15(0x1f as BYTE, 0x1f as BYTE, 0xa as BYTE);
        } else if sameColor(
            color,
            colorFromRgb15(0x1f as BYTE, 0x1f as BYTE, 0xa as BYTE),
        ) {
            (*lpPrim).color = colorFromRgb15(0x1f as BYTE, 0xa as BYTE, 0xa as BYTE);
        } else if sameColor(
            color,
            colorFromRgb15(0x1f as BYTE, 0xa as BYTE, 0xa as BYTE),
        ) {
            (*lpPrim).color = colorFromRgb15(0x14 as BYTE, 0xa as BYTE, 0 as BYTE);
        } else if sameColor(
            color,
            colorFromRgb15(0x14 as BYTE, 0xa as BYTE, 0 as BYTE),
        ) {
            (*lpPrim).color = colorFromRgb15(0x14 as BYTE, 0 as BYTE, 0 as BYTE);
        }
    } else {
        let mut hDestruct: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
        (*lpPrim).Type = gfx_object::NUM_PRIMS.0 as ::core::ffi::c_int as GRAPHICS_PRIM;
        (*ElementPtr).preprocess_func = None;
        hDestruct = AllocElement();
        if !hDestruct.is_null() {
            let mut DestructPtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
            let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
            StarShipPtr = (*ElementPtr).pParent as *mut STARSHIP;
            PutQueue(&raw mut disp_q, hDestruct);
            DestructPtr = LockLink(&raw mut disp_q, hDestruct) as *mut ELEMENT;
            (*DestructPtr).pParent = StarShipPtr as *mut ::core::ffi::c_void;
            (*DestructPtr).c2rust_unnamed_0.hit_points = 0 as COUNT;
            (*DestructPtr).c2rust_unnamed_1.mass_points = 0 as BYTE;
            (*DestructPtr).playerNr = -1 as SIZE;
            (*DestructPtr).state_flags = ((1 as ::core::ffi::c_int)
                << 3 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as ELEMENT_FLAGS;
            (*GlobData.Game_state.DisplayArray.offset((*DestructPtr).PrimIndex as isize))
                .Type = gfx_object::STAMPFILL_PRIM.0 as ::core::ffi::c_int
                as GRAPHICS_PRIM;
            (*GlobData.Game_state.DisplayArray.offset((*DestructPtr).PrimIndex as isize))
                .color = colorFromRgb15(0x1f as BYTE, 0x1f as BYTE, 0x1f as BYTE);
            (*DestructPtr).current.image.farray = &raw mut (*(*StarShipPtr).RaceDescPtr)
                .ship_data
                .special as *mut FRAME;
            (*DestructPtr).current.image.frame = (*(*StarShipPtr).RaceDescPtr)
                .ship_data
                .special[0usize];
            (*DestructPtr).c2rust_unnamed.life_span = GetFrameCount(
                (*DestructPtr).current.image.frame,
            );
            (*DestructPtr).current.location = (*ElementPtr).current.location;
            (*DestructPtr).preprocess_func = Some(
                destruct_preprocess as unsafe extern "C" fn(*mut ELEMENT) -> (),
            ) as Option<ElementProcessFunc>;
            (*DestructPtr).postprocess_func = None;
            (*DestructPtr).death_func = None;
            memset(
                &raw mut (*DestructPtr).velocity as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<VELOCITY_DESC>(),
            );
            UnlockLink(&raw mut disp_q, hDestruct);
        }
    };
}
unsafe extern "C" fn self_destruct_kill_objects(mut ElementPtr: *mut ELEMENT) {
    let mut hElement: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut hNextElement: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
    hElement = disp_q.head as HELEMENT;
    while !hElement.is_null() {
        let mut ObjPtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
        let mut delta_x: SDWORD = 0;
        let mut delta_y: SDWORD = 0;
        let mut dist: DWORD = 0;
        ObjPtr = LockLink(&raw mut disp_q, hElement) as *mut ELEMENT;
        hNextElement = (*ObjPtr).succ;
        if (*ObjPtr).state_flags as ::core::ffi::c_int
            & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) != 0
            && !((*ObjPtr).preprocess_func
                == Some(intruder_preprocess as unsafe extern "C" fn(*mut ELEMENT) -> ())
                && (*ObjPtr).collision_func
                    == Some(
                        marine_collision
                            as unsafe extern "C" fn(
                                *mut ELEMENT,
                                *mut POINT,
                                *mut ELEMENT,
                                *mut POINT,
                            ) -> (),
                    ))
        {
            UnlockLink(&raw mut disp_q, hElement);
        } else {
            delta_x = ((*ObjPtr).next.location.x as ::core::ffi::c_int
                - (*ElementPtr).next.location.x as ::core::ffi::c_int) as SDWORD;
            if delta_x < 0 as ::core::ffi::c_int {
                delta_x = -delta_x;
            }
            delta_y = ((*ObjPtr).next.location.y as ::core::ffi::c_int
                - (*ElementPtr).next.location.y as ::core::ffi::c_int) as SDWORD;
            if delta_y < 0 as ::core::ffi::c_int {
                delta_y = -delta_y;
            }
            delta_x = delta_x >> 2 as ::core::ffi::c_int;
            delta_y = delta_y >> 2 as ::core::ffi::c_int;
            dist = (delta_x * delta_x + delta_y * delta_y) as DWORD;
            if delta_x <= (180 as ::core::ffi::c_int) << resolutionFactor
                && delta_y <= (180 as ::core::ffi::c_int) << resolutionFactor
                && dist as ::core::ffi::c_int
                    <= ((180 as ::core::ffi::c_int) << resolutionFactor)
                        * ((180 as ::core::ffi::c_int) << resolutionFactor)
            {
                let mut destruction: ::core::ffi::c_int = 1 as ::core::ffi::c_int
                    + 180 as ::core::ffi::c_int / 10 as ::core::ffi::c_int
                        * (((180 as ::core::ffi::c_int) << resolutionFactor)
                            - square_root(dist) as ::core::ffi::c_int)
                        / ((180 as ::core::ffi::c_int) << resolutionFactor);
                if (*ObjPtr).state_flags as ::core::ffi::c_int
                    & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int != 0
                {
                    if !(antiCheat(
                            ElementPtr,
                            BOOLEAN::TRUE,
                            C2Rust_Unnamed::OPTVAL_INF_HEALTH.0 as ::core::ffi::c_int,
                        )
                        .0 != 0
                        || antiCheat(
                                ElementPtr,
                                BOOLEAN::TRUE,
                                C2Rust_Unnamed::OPTVAL_FULL_GOD.0 as ::core::ffi::c_int,
                            )
                            .0 != 0)
                    {
                        if uqm_lua_checked!(DeltaCrew(ObjPtr, -destruction as SIZE), ()).0 == 0 {
                            (*ObjPtr).c2rust_unnamed.life_span = 0 as COUNT;
                        }
                    }
                } else if !((*ObjPtr).c2rust_unnamed_1.mass_points as ::core::ffi::c_int
                    > 10 as ::core::ffi::c_int * 10 as ::core::ffi::c_int)
                {
                    if destruction
                        < (*ObjPtr).c2rust_unnamed_0.hit_points as ::core::ffi::c_int
                    {
                        (*ObjPtr).c2rust_unnamed_0.hit_points = ((*ObjPtr)
                            .c2rust_unnamed_0
                            .hit_points as ::core::ffi::c_int - destruction) as COUNT;
                    } else {
                        (*ObjPtr).c2rust_unnamed_0.hit_points = 0 as COUNT;
                        (*ObjPtr).c2rust_unnamed.life_span = 0 as COUNT;
                    }
                }
            }
            UnlockLink(&raw mut disp_q, hElement);
        }
        hElement = hNextElement;
    }
}
unsafe extern "C" fn shofixti_destruct_death(mut ShipPtr: *mut ELEMENT) {
    let mut StarShip: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
    let mut winner: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
    StarShip = (*ShipPtr).pParent as *mut STARSHIP;
    uqm_lua_checked!(StopAllBattleMusic(), ());
    uqm_lua_checked!(StartShipExplosion(ShipPtr, false), ());
    (*ShipPtr).preprocess_func = Some(
        destruct_preprocess as unsafe extern "C" fn(*mut ELEMENT) -> (),
    ) as Option<ElementProcessFunc>;
    uqm_lua_checked!(PlaySound(
        SetAbsStringTableIndex(
            (*(*StarShip).RaceDescPtr).ship_data.ship_sounds,
            1 as COUNT,
        ),
        CalcSoundPosition(ShipPtr),
        ShipPtr,
        (2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as BYTE,
    ), ());
    winner = GetWinnerStarShip();
    if winner.is_null() {
        winner = FindAliveStarShip(ShipPtr);
        if winner.is_null() {
            winner = StarShip;
        }
        SetWinnerStarShip(winner);
    } else if winner == StarShip {
        (*StarShip).cur_status_flags = ((*StarShip).cur_status_flags
            as ::core::ffi::c_int
            & !((1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int)) as STATUS_FLAGS;
    }
    uqm_lua_checked!(RecordShipDeath(ShipPtr), ());
}
unsafe extern "C" fn self_destruct(mut ElementPtr: *mut ELEMENT) {
    let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
    let mut hDestruct: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
    StarShipPtr = (*ElementPtr).pParent as *mut STARSHIP;
    hDestruct = AllocElement();
    if !hDestruct.is_null() {
        let mut DestructPtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
        DestructPtr = LockLink(&raw mut disp_q, hDestruct) as *mut ELEMENT;
        (*DestructPtr).playerNr = (*ElementPtr).playerNr;
        (*DestructPtr).state_flags = ((1 as ::core::ffi::c_int)
            << 3 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int) as ELEMENT_FLAGS;
        (*DestructPtr).next.location = (*ElementPtr).next.location;
        (*DestructPtr).c2rust_unnamed.life_span = 0 as COUNT;
        (*DestructPtr).pParent = (*ElementPtr).pParent;
        (*DestructPtr).hTarget = ::core::ptr::null_mut::<::core::ffi::c_void>();
        (*DestructPtr).death_func = Some(
            self_destruct_kill_objects as unsafe extern "C" fn(*mut ELEMENT) -> (),
        ) as Option<ElementProcessFunc>;
        UnlockLink(&raw mut disp_q, hDestruct);
        PutQueue(&raw mut disp_q, hDestruct);
    }
    uqm_lua_checked!(DeltaCrew(
        ElementPtr,
        -((*ElementPtr).c2rust_unnamed_0.crew_level as ::core::ffi::c_int) as SIZE,
    ), ());
    memset(
        &raw mut (*ElementPtr).velocity as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<VELOCITY_DESC>(),
    );
    (*ElementPtr).state_flags = ((*ElementPtr).state_flags as ::core::ffi::c_int
        | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as ELEMENT_FLAGS;
    (*ElementPtr).c2rust_unnamed.life_span = 0 as COUNT;
    (*ElementPtr).death_func = Some(
        shofixti_destruct_death as unsafe extern "C" fn(*mut ELEMENT) -> (),
    ) as Option<ElementProcessFunc>;
}
unsafe extern "C" fn shofixti_intelligence(
    mut ShipPtr: *mut ELEMENT,
    mut ObjectsOfConcern: *mut EVALUATE_DESC,
    mut ConcernCounter: COUNT,
) {
    let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
    uqm_lua_checked!(ship_intelligence(ShipPtr, ObjectsOfConcern, ConcernCounter), ());
    StarShipPtr = (*ShipPtr).pParent as *mut STARSHIP;
    if (*StarShipPtr).special_counter as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
        return;
    }
    if (*StarShipPtr).ship_input_state as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int != 0
    {
        (*StarShipPtr).ship_input_state = ((*StarShipPtr).ship_input_state
            as ::core::ffi::c_int
            & !((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)) as BYTE;
    } else {
        let mut lpWeaponEvalDesc: *mut EVALUATE_DESC = ::core::ptr::null_mut::<
            EVALUATE_DESC,
        >();
        let mut lpShipEvalDesc: *mut EVALUATE_DESC = ::core::ptr::null_mut::<
            EVALUATE_DESC,
        >();
        lpWeaponEvalDesc = ObjectsOfConcern
            .offset(
                C2Rust_Unnamed_7::ENEMY_WEAPON_INDEX.0 as ::core::ffi::c_int as isize,
            );
        lpShipEvalDesc = ObjectsOfConcern
            .offset(C2Rust_Unnamed_7::ENEMY_SHIP_INDEX.0 as ::core::ffi::c_int as isize);
        if !(*(*StarShipPtr).RaceDescPtr).ship_data.special[0usize].is_null()
            && (((*(*StarShipPtr).RaceDescPtr).ship_data.captain_control.special_offset
                as ::core::ffi::c_int) < 4 as ::core::ffi::c_int
                || !(*lpShipEvalDesc).ObjectPtr.is_null()
                    && (*lpShipEvalDesc).which_turn as ::core::ffi::c_int
                        <= 4 as ::core::ffi::c_int
                || !(*lpWeaponEvalDesc).ObjectPtr.is_null()
                    && ((*(*lpWeaponEvalDesc).ObjectPtr).state_flags
                        as ::core::ffi::c_int
                        & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int != 0
                        && (*ShipPtr).c2rust_unnamed_0.crew_level as ::core::ffi::c_int
                            == 1 as ::core::ffi::c_int
                        || uqm_lua_checked!(PlotIntercept(
                            (*lpWeaponEvalDesc).ObjectPtr,
                            ShipPtr,
                            2 as COUNT,
                            0 as COUNT,
                        ), ()) as ::core::ffi::c_int != 0
                            && (*(*lpWeaponEvalDesc).ObjectPtr)
                                .c2rust_unnamed_1
                                .mass_points as ::core::ffi::c_int
                                >= (*ShipPtr).c2rust_unnamed_0.crew_level
                                    as ::core::ffi::c_int && TFB_Random() & 1 as DWORD != 0))
        {
            (*StarShipPtr).ship_input_state = ((*StarShipPtr).ship_input_state
                as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as BYTE;
        }
    };
}
unsafe extern "C" fn shofixti_preprocess(mut ElementPtr: *mut ELEMENT) {
    let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
    StarShipPtr = (*ElementPtr).pParent as *mut STARSHIP;
    if ((*StarShipPtr).cur_status_flags as ::core::ffi::c_int
        ^ (*StarShipPtr).old_status_flags as ::core::ffi::c_int)
        & (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int != 0
        && (*StarShipPtr).cur_status_flags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int == 0
    {
        if (*(*StarShipPtr).RaceDescPtr).ship_data.captain_control.special_offset
            as ::core::ffi::c_int & 1 as ::core::ffi::c_int == 0
        {
            (*(*StarShipPtr).RaceDescPtr).ship_data.captain_control.special_offset = (*(*StarShipPtr)
                .RaceDescPtr)
                .ship_data
                .captain_control
                .special_offset
                .wrapping_add(1);
        } else {
            (*(*StarShipPtr).RaceDescPtr).ship_data.captain_control.special_offset = ((*(*StarShipPtr)
                .RaceDescPtr)
                .ship_data
                .captain_control
                .special_offset as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as BYTE;
        }
    }
}
unsafe extern "C" fn shofixti_postprocess(mut ElementPtr: *mut ELEMENT) {
    let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
    StarShipPtr = (*ElementPtr).pParent as *mut STARSHIP;
    if (*(*StarShipPtr).RaceDescPtr).ship_data.captain_control.special_offset
        as ::core::ffi::c_int == 5 as ::core::ffi::c_int
    {
        uqm_lua_checked!(self_destruct(ElementPtr), ());
        (*StarShipPtr).cur_status_flags = ((*StarShipPtr).cur_status_flags
            as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int) as STATUS_FLAGS;
    }
}
#[no_mangle]
pub unsafe extern "C" fn init_shofixti() -> *mut RACE_DESC {
    let mut RaceDescPtr: *mut RACE_DESC = ::core::ptr::null_mut::<RACE_DESC>();
    static mut new_shofixti_desc: RACE_DESC = RACE_DESC {
        ship_info: SHIP_INFO {
            idStr: ::core::ptr::null::<::core::ffi::c_char>(),
            ship_flags: 0,
            ship_cost: 0,
            crew_level: 0,
            max_crew: 0,
            energy_level: 0,
            max_energy: 0,
            race_strings_rsc: ::core::ptr::null::<::core::ffi::c_char>(),
            icons_rsc: ::core::ptr::null::<::core::ffi::c_char>(),
            melee_icon_rsc: ::core::ptr::null::<::core::ffi::c_char>(),
            race_strings: ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>(),
            icons: ::core::ptr::null_mut::<FRAME_DESC>(),
            melee_icon: ::core::ptr::null_mut::<FRAME_DESC>(),
        },
        c2rust_padding: [0; 8],
        fleet: FLEET_STUFF {
            strength: 0,
            known_loc: POINT { x: 0, y: 0 },
        },
        c2rust_padding_0: [0; 10],
        characteristics: CHARACTERISTIC_STUFF {
            max_thrust: 0,
            thrust_increment: 0,
            energy_regeneration: 0,
            weapon_energy_cost: 0,
            special_energy_cost: 0,
            energy_wait: 0,
            turn_wait: 0,
            thrust_wait: 0,
            weapon_wait: 0,
            special_wait: 0,
            ship_mass: 0,
        },
        c2rust_padding_1: [0; 2],
        ship_data: DATA_STUFF {
            ship_rsc: [::core::ptr::null::<::core::ffi::c_char>(); 3],
            weapon_rsc: [::core::ptr::null::<::core::ffi::c_char>(); 3],
            special_rsc: [::core::ptr::null::<::core::ffi::c_char>(); 3],
            captain_control: CAPTAIN_STUFF {
                captain_rsc: ::core::ptr::null::<::core::ffi::c_char>(),
                background: ::core::ptr::null_mut::<FRAME_DESC>(),
                turn: ::core::ptr::null_mut::<FRAME_DESC>(),
                thrust: ::core::ptr::null_mut::<FRAME_DESC>(),
                weapon: ::core::ptr::null_mut::<FRAME_DESC>(),
                special: ::core::ptr::null_mut::<FRAME_DESC>(),
                tl_offset: 0,
                tr_offset: 0,
                thrust_offset: 0,
                weapon_offset: 0,
                special_offset: 0,
            },
            victory_ditty_rsc: ::core::ptr::null::<::core::ffi::c_char>(),
            ship_sounds_rsc: ::core::ptr::null::<::core::ffi::c_char>(),
            ship: [::core::ptr::null_mut::<FRAME_DESC>(); 3],
            weapon: [::core::ptr::null_mut::<FRAME_DESC>(); 3],
            special: [::core::ptr::null_mut::<FRAME_DESC>(); 3],
            victory_ditty: ::core::ptr::null_mut::<*mut TFB_SoundSample>(),
            ship_sounds: ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>(),
        },
        c2rust_padding_2: [0; 8],
        cyborg_control: INTEL_STUFF {
            ManeuverabilityIndex: 0,
            WeaponRange: 0,
            intelligence_func: None,
        },
        uninit_func: None,
        preprocess_func: None,
        postprocess_func: None,
        init_weapon_func: None,
        data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        CodeRef: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    if if resolutionFactor != 2 as ::core::ffi::c_uint {
        BOOLEAN::FALSE.0 as ::core::ffi::c_int
    } else {
        BOOLEAN::TRUE.0 as ::core::ffi::c_int
    } != 0
    {
        shofixti_desc.characteristics.max_thrust = ((35 as ::core::ffi::c_int)
            << resolutionFactor) as COUNT;
        shofixti_desc.characteristics.thrust_increment = ((5 as ::core::ffi::c_int)
            << resolutionFactor) as COUNT;
        shofixti_desc.cyborg_control.WeaponRange = ((((24 as ::core::ffi::c_int)
            << 2 as ::core::ffi::c_int) << resolutionFactor) * 10 as ::core::ffi::c_int)
            as COUNT;
    } else {
        shofixti_desc.characteristics.max_thrust = 35 as COUNT;
        shofixti_desc.characteristics.thrust_increment = 5 as COUNT;
        shofixti_desc.cyborg_control.WeaponRange = (((24 as ::core::ffi::c_int)
            << 2 as ::core::ffi::c_int) * 10 as ::core::ffi::c_int) as COUNT;
    }
    shofixti_desc.preprocess_func = Some(
        shofixti_preprocess as unsafe extern "C" fn(*mut ELEMENT) -> (),
    ) as Option<PREPROCESS_FUNC>;
    shofixti_desc.postprocess_func = Some(
        shofixti_postprocess as unsafe extern "C" fn(*mut ELEMENT) -> (),
    ) as Option<POSTPROCESS_FUNC>;
    shofixti_desc.init_weapon_func = Some(
        initialize_standard_missile
            as unsafe extern "C" fn(*mut ELEMENT, *mut HELEMENT) -> COUNT,
    ) as Option<INIT_WEAPON_FUNC>;
    shofixti_desc.cyborg_control.intelligence_func = Some(
        shofixti_intelligence
            as unsafe extern "C" fn(*mut ELEMENT, *mut EVALUATE_DESC, COUNT) -> (),
    ) as Option<IntelligenceFunc>;
    new_shofixti_desc = shofixti_desc;
    if GlobData.Game_state.CurrentActivity as BYTE as ::core::ffi::c_int
        == C2Rust_Unnamed_6::IN_ENCOUNTER.0 as ::core::ffi::c_int
        && !CurStarDescPtr.is_null()
        && (*CurStarDescPtr).Index as ::core::ffi::c_int
            == C2Rust_Unnamed_8::SHOFIXTI_DEFINED.0 as ::core::ffi::c_int
        && uqm_lua_checked!(getGameStateUint(
            b"SHOFIXTI_RECRUITED\0".as_ptr() as *const ::core::ffi::c_char,
        ), ::core::ptr::null_mut()) == 0
    {
        let mut i: COUNT = 0;
        new_shofixti_desc.ship_data.ship_rsc[0usize] = b"ship.shofixti.graphics.oldscout.large\0"
            .as_ptr() as *const ::core::ffi::c_char as RESOURCE;
        new_shofixti_desc.ship_data.ship_rsc[1usize] = b"ship.shofixti.graphics.oldscout.medium\0"
            .as_ptr() as *const ::core::ffi::c_char as RESOURCE;
        new_shofixti_desc.ship_data.ship_rsc[2usize] = b"ship.shofixti.graphics.oldscout.small\0"
            .as_ptr() as *const ::core::ffi::c_char as RESOURCE;
        new_shofixti_desc.ship_data.special_rsc[0usize] = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        new_shofixti_desc.ship_data.special_rsc[1usize] = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        new_shofixti_desc.ship_data.special_rsc[2usize] = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        new_shofixti_desc.ship_data.captain_control.captain_rsc = b"ship.shofixti.graphics.oldcaptain\0"
            .as_ptr() as *const ::core::ffi::c_char as RESOURCE;
        new_shofixti_desc.ship_info.icons_rsc = b"ship.shofixti.oldicons\0".as_ptr()
            as *const ::core::ffi::c_char as RESOURCE;
        if if if GlobData.SIS_state.Difficulty as ::core::ffi::c_int != 0 {
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
            new_shofixti_desc.characteristics.weapon_wait = 10 as BYTE;
        }
        i = 0 as COUNT;
        while (i as ::core::ffi::c_int) < 3 as ::core::ffi::c_int {
            if if if GlobData.SIS_state.Difficulty as ::core::ffi::c_int != 0 {
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
                new_shofixti_desc.characteristics.turn_wait = new_shofixti_desc
                    .characteristics
                    .turn_wait
                    .wrapping_add(1);
                if new_shofixti_desc.characteristics.turn_wait as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                {
                    new_shofixti_desc.characteristics.turn_wait = new_shofixti_desc
                        .characteristics
                        .turn_wait
                        .wrapping_sub(1);
                }
                new_shofixti_desc.characteristics.thrust_wait = new_shofixti_desc
                    .characteristics
                    .thrust_wait
                    .wrapping_add(1);
                if new_shofixti_desc.characteristics.thrust_wait as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                {
                    new_shofixti_desc.characteristics.thrust_wait = new_shofixti_desc
                        .characteristics
                        .thrust_wait
                        .wrapping_sub(1);
                }
                if new_shofixti_desc.characteristics.thrust_increment
                    as ::core::ffi::c_int
                    <= (1 as ::core::ffi::c_int) << resolutionFactor
                        << 2 as ::core::ffi::c_int
                {
                    new_shofixti_desc.characteristics.max_thrust = ((new_shofixti_desc
                        .characteristics
                        .thrust_increment as ::core::ffi::c_int)
                        << 1 as ::core::ffi::c_int) as COUNT;
                } else {
                    let mut num_thrusts: COUNT = 0;
                    num_thrusts = (new_shofixti_desc.characteristics.max_thrust
                        as ::core::ffi::c_int
                        / new_shofixti_desc.characteristics.thrust_increment
                            as ::core::ffi::c_int) as COUNT;
                    new_shofixti_desc.characteristics.thrust_increment = (new_shofixti_desc
                        .characteristics
                        .thrust_increment as ::core::ffi::c_int
                        - ((1 as ::core::ffi::c_int) << resolutionFactor)) as COUNT;
                    new_shofixti_desc.characteristics.max_thrust = (new_shofixti_desc
                        .characteristics
                        .thrust_increment as ::core::ffi::c_int
                        * num_thrusts as ::core::ffi::c_int) as COUNT;
                }
            }
            i = i.wrapping_add(1);
        }
    }
    RaceDescPtr = &raw mut new_shofixti_desc;
    return RaceDescPtr;
}
