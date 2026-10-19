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

// Rust translation of src/uqm/ships/sis_ship/sis_ship.c.
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
    fn HMalloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn HFree(p: *mut ::core::ffi::c_void);
    fn PutQueue(pq: *mut QUEUE, hLink: HLINK);
    fn GetFrameIndex(Frame: FRAME) -> COUNT;
    fn SetAbsFrameIndex(Frame: FRAME, FrameIndex: COUNT) -> FRAME;
    fn IncFrameIndex(Frame: FRAME) -> FRAME;
    fn SetAbsStringTableIndex(String: STRING, StringTableIndex: COUNT) -> STRING;
    fn memset(
        _: *mut ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    static mut PulsedInputState: CONTROLLER_INPUT_STATE;
    static mut resolutionFactor: ::core::ffi::c_uint;
    static mut optFlagshipColor: ::core::ffi::c_int;
    fn ARCTAN(delta_x: SDWORD, delta_y: SDWORD) -> COUNT;
    fn GetCurrentVelocityComponents(
        velocityptr: *mut VELOCITY_DESC,
        pdx: *mut SIZE,
        pdy: *mut SIZE,
    );
    fn SetVelocityVector(
        velocityptr: *mut VELOCITY_DESC,
        magnitude: SDWORD,
        facing: COUNT,
    );
    fn SetVelocityComponents(velocityptr: *mut VELOCITY_DESC, dx: SDWORD, dy: SDWORD);
    static mut disp_q: QUEUE;
    fn AllocElement() -> HELEMENT;
    fn setGameStateUint(name: *const ::core::ffi::c_char, val: DWORD);
    fn getGameStateUint(name: *const ::core::ffi::c_char) -> DWORD;
    static mut GlobData: GLOBDATA;
    fn inHQSpace() -> BOOLEAN;
    fn square_root(value: DWORD) -> COUNT;
    fn ProcessSound(Sound: SOUND, PositionalObject: *mut ELEMENT);
    fn DeltaEnergy(ElementPtr: *mut ELEMENT, energy_delta: SIZE) -> BOOLEAN;
    fn ship_intelligence(
        ShipPtr: *mut ELEMENT,
        ObjectsOfConcern: *mut EVALUATE_DESC,
        ConcernCounter: COUNT,
    );
    fn initialize_laser(pLaserBlock: *mut LASER_BLOCK) -> HELEMENT;
    fn initialize_missile(pMissileBlock: *mut MISSILE_BLOCK) -> HELEMENT;
    fn weapon_collision(
        ElementPtr0: *mut ELEMENT,
        pPt0: *mut POINT,
        ElementPtr1: *mut ELEMENT,
        pPt1: *mut POINT,
    ) -> HELEMENT;
    fn TrackShip(Tracker: *mut ELEMENT, pfacing: *mut COUNT) -> SIZE;
    fn HyperspaceMenu();
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
pub struct C2Rust_Unnamed_6(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_6 {
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
pub struct C2Rust_Unnamed_7(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_7 {
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
pub struct C2Rust_Unnamed_8(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_8 {
    pub const ENEMY_SHIP_INDEX: Self = Self(0);
    pub const CREW_OBJECT_INDEX: Self = Self(1);
    pub const ENEMY_WEAPON_INDEX: Self = Self(2);
    pub const GRAVITY_MASS_INDEX: Self = Self(3);
    pub const FIRST_EMPTY_INDEX: Self = Self(4);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LASER_BLOCK {
    pub cx: COORD,
    pub cy: COORD,
    pub ex: COORD,
    pub ey: COORD,
    pub flags: ELEMENT_FLAGS,
    pub sender: SIZE,
    pub pixoffs: SIZE,
    pub face: COUNT,
    pub color: Color,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SIS_DATA {
    pub num_trackers: COUNT,
    pub num_blasters: COUNT,
    pub MissileBlock: [MISSILE_BLOCK; 6],
}
pub type CustomShipData_t = SIS_DATA;
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
unsafe extern "C" fn ValidPoint(mut pt: POINT) -> BOOLEAN {
    return BOOLEAN(
        (pt.x as ::core::ffi::c_int != !(0 as ::core::ffi::c_int)
            && pt.y as ::core::ffi::c_int != !(0 as ::core::ffi::c_int))
            as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
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
#[inline]
unsafe extern "C" fn LoadAdvancedQuasiPilot() -> POINT {
    return point {
        x: uqm_lua_checked!(getGameStateUint(
            b"ADV_AUTOPILOT_QUASI_X\0".as_ptr() as *const ::core::ffi::c_char,
        ), point { x: 0, y: 0 }) as COORD,
        y: uqm_lua_checked!(getGameStateUint(
            b"ADV_AUTOPILOT_QUASI_Y\0".as_ptr() as *const ::core::ffi::c_char,
        ), point { x: 0, y: 0 }) as COORD,
    };
}
#[inline]
unsafe extern "C" fn LoadAdvancedAutoPilot() -> POINT {
    return point {
        x: uqm_lua_checked!(getGameStateUint(
            b"ADV_AUTOPILOT_SAVE_X\0".as_ptr() as *const ::core::ffi::c_char,
        ), point { x: 0, y: 0 }) as COORD,
        y: uqm_lua_checked!(getGameStateUint(
            b"ADV_AUTOPILOT_SAVE_Y\0".as_ptr() as *const ::core::ffi::c_char,
        ), point { x: 0, y: 0 }) as COORD,
    };
}
#[inline]
unsafe extern "C" fn ZeroAdvancedQuasiPilot() {
    uqm_lua_checked!(setGameStateUint(
        b"ADV_AUTOPILOT_QUASI_X\0".as_ptr() as *const ::core::ffi::c_char,
        !(0 as ::core::ffi::c_int) as DWORD,
    ), ());
    uqm_lua_checked!(setGameStateUint(
        b"ADV_AUTOPILOT_QUASI_Y\0".as_ptr() as *const ::core::ffi::c_char,
        !(0 as ::core::ffi::c_int) as DWORD,
    ), ());
}
#[inline]
unsafe extern "C" fn ZeroAdvancedAutoPilot() {
    uqm_lua_checked!(setGameStateUint(
        b"ADV_AUTOPILOT_SAVE_X\0".as_ptr() as *const ::core::ffi::c_char,
        !(0 as ::core::ffi::c_int) as DWORD,
    ), ());
    uqm_lua_checked!(setGameStateUint(
        b"ADV_AUTOPILOT_SAVE_Y\0".as_ptr() as *const ::core::ffi::c_char,
        !(0 as ::core::ffi::c_int) as DWORD,
    ), ());
}
static mut sis_desc: RACE_DESC = unsafe {
    race_desc {
        c2rust_padding: [0; 8],
        c2rust_padding_0: [0; 10],
        c2rust_padding_1: [0; 2],
        c2rust_padding_2: [0; 8],
        ship_info: SHIP_INFO {
            idStr: b"flagship\0".as_ptr() as *const ::core::ffi::c_char,
            ship_flags: 0 as UWORD,
            ship_cost: 16 as BYTE,
            crew_level: 42 as COUNT,
            max_crew: 42 as COUNT,
            energy_level: 42 as BYTE,
            max_energy: 42 as BYTE,
            race_strings_rsc: ::core::ptr::null::<::core::ffi::c_char>(),
            icons_rsc: b"ship.flagship.icons\0".as_ptr() as RESOURCE,
            melee_icon_rsc: ::core::ptr::null::<::core::ffi::c_char>(),
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
            max_thrust: 10 as COUNT,
            thrust_increment: 4 as COUNT,
            energy_regeneration: 1 as BYTE,
            weapon_energy_cost: 1 as BYTE,
            special_energy_cost: 0 as BYTE,
            energy_wait: 10 as BYTE,
            turn_wait: 17 as BYTE,
            thrust_wait: 6 as BYTE,
            weapon_wait: 6 as BYTE,
            special_wait: 9 as BYTE,
            ship_mass: 10 as BYTE,
        },
        ship_data: DATA_STUFF {
            ship_rsc: [
                b"ship.flagship.graphics.flagship.large\0".as_ptr() as RESOURCE,
                b"ship.flagship.graphics.flagship.medium\0".as_ptr() as RESOURCE,
                b"ship.flagship.graphics.flagship.small\0".as_ptr() as RESOURCE,
            ],
            weapon_rsc: [
                b"ship.flagship.graphics.blaster.large\0".as_ptr() as RESOURCE,
                b"ship.flagship.graphics.blaster.medium\0".as_ptr() as RESOURCE,
                b"ship.flagship.graphics.blaster.small\0".as_ptr() as RESOURCE,
            ],
            special_rsc: [
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
            ],
            captain_control: captain_stuff {
                captain_rsc: b"ship.flagship.graphics.captain\0".as_ptr() as RESOURCE,
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
            victory_ditty_rsc: b"ship.flagship.ditty\0".as_ptr() as RESOURCE,
            ship_sounds_rsc: b"ship.flagship.sounds\0".as_ptr() as RESOURCE,
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
                * 12 as ::core::ffi::c_int) as COUNT,
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
unsafe extern "C" fn GetCustomShipData(
    mut pRaceDesc: *mut RACE_DESC,
) -> *mut CustomShipData_t {
    return (*pRaceDesc).data as *mut CustomShipData_t;
}
unsafe extern "C" fn SetCustomShipData(
    mut pRaceDesc: *mut RACE_DESC,
    mut data: *const CustomShipData_t,
) {
    if (*pRaceDesc).data == data as *mut ::core::ffi::c_void {
        return;
    }
    if !(*pRaceDesc).data.is_null() {
        HFree((*pRaceDesc).data);
        (*pRaceDesc).data = ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    if !data.is_null() {
        let mut newData: *mut CustomShipData_t = HMalloc(
            ::core::mem::size_of::<CustomShipData_t>(),
        ) as *mut CustomShipData_t;
        *newData = *data;
        (*pRaceDesc).data = newData as *mut ::core::ffi::c_void;
    }
}
unsafe extern "C" fn sis_hyper_preprocess(mut ElementPtr: *mut ELEMENT) {
    let mut udx: SDWORD = 0 as SDWORD;
    let mut udy: SDWORD = 0 as SDWORD;
    let mut dx: SIZE = 0 as SIZE;
    let mut dy: SIZE = 0 as SIZE;
    let mut AccelerateDirection: SIZE = 0;
    let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
    if (*ElementPtr).state_flags as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int != 0
    {
        (*ElementPtr).velocity = GlobData.Game_state.velocity;
    }
    AccelerateDirection = 0 as SIZE;
    StarShipPtr = (*ElementPtr).pParent as *mut STARSHIP;
    (*StarShipPtr).weapon_counter = (*StarShipPtr).weapon_counter.wrapping_add(1);
    's_230: {
        '_LeaveAutoPilot: {
            if !(GlobData.Game_state.autopilot.x as ::core::ffi::c_int
                == !(0 as ::core::ffi::c_int)
                || GlobData.Game_state.autopilot.y as ::core::ffi::c_int
                    == !(0 as ::core::ffi::c_int)
                || (*StarShipPtr).cur_status_flags as ::core::ffi::c_int
                    & ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                        | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                        | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) != 0)
            {
                let mut facing: SIZE = 0;
                let mut universe: POINT = POINT { x: 0, y: 0 };
                universe.x = logxToUniverse(GlobData.SIS_state.log_x) as COORD;
                universe.y = logyToUniverse(GlobData.SIS_state.log_y) as COORD;
                udx = (GlobData.Game_state.autopilot.x as ::core::ffi::c_int
                    - universe.x as ::core::ffi::c_int) as SDWORD;
                udy = -(GlobData.Game_state.autopilot.y as ::core::ffi::c_int
                    - universe.y as ::core::ffi::c_int) as SDWORD;
                dx = udx as SIZE;
                if (dx as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                    dx = -(dx as ::core::ffi::c_int) as SIZE;
                }
                dy = udy as SIZE;
                if (dy as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                    dy = -(dy as ::core::ffi::c_int) as SIZE;
                }
                if !(dx as ::core::ffi::c_int
                    <= (1 as ::core::ffi::c_int) << resolutionFactor
                    && dy as ::core::ffi::c_int
                        <= (1 as ::core::ffi::c_int) << resolutionFactor)
                {
                    facing = (ARCTAN(udx, udy) as ::core::ffi::c_int
                        + ((1 as ::core::ffi::c_int)
                            << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int)
                        >> 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                        & ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                            - 1 as ::core::ffi::c_int) as DWORD as SIZE;
                    if (*StarShipPtr).cur_status_flags as ::core::ffi::c_int
                        & (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int != 0
                    {
                        if ((*StarShipPtr).ShipFacing as ::core::ffi::c_int
                            + (((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                                >> 2 as ::core::ffi::c_int)
                                + ((1 as ::core::ffi::c_int)
                                    << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                                        - 1 as ::core::ffi::c_int)
                                >> 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int)
                            - facing as ::core::ffi::c_int
                            & ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                                - 1 as ::core::ffi::c_int) as DWORD
                            > (((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                                >> 1 as ::core::ffi::c_int)
                                + ((1 as ::core::ffi::c_int)
                                    << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                                        - 1 as ::core::ffi::c_int)
                                >> 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int)
                                as DWORD
                        {
                            break '_LeaveAutoPilot;
                        } else {
                            facing = (*StarShipPtr).ShipFacing as SIZE;
                        }
                    } else if facing as ::core::ffi::c_int
                        != (*StarShipPtr).ShipFacing as ::core::ffi::c_int
                        && (*ElementPtr).c2rust_unnamed_2.turn_wait as ::core::ffi::c_int
                            == 0 as ::core::ffi::c_int
                    {
                        if ((*StarShipPtr).ShipFacing as ::core::ffi::c_int
                            - facing as ::core::ffi::c_int
                            & ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                                - 1 as ::core::ffi::c_int) as DWORD
                            >= (((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                                >> 1 as ::core::ffi::c_int)
                                + ((1 as ::core::ffi::c_int)
                                    << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                                        - 1 as ::core::ffi::c_int)
                                >> 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int)
                                as DWORD
                        {
                            facing = (facing as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int
                                & ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                                    - 1 as ::core::ffi::c_int) as DWORD as SIZE;
                            (*StarShipPtr).cur_status_flags = ((*StarShipPtr)
                                .cur_status_flags as ::core::ffi::c_int
                                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
                                as STATUS_FLAGS;
                        } else if (*StarShipPtr).ShipFacing as ::core::ffi::c_int
                            != facing as ::core::ffi::c_int
                        {
                            facing = (facing as ::core::ffi::c_int
                                + 1 as ::core::ffi::c_int
                                & ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                                    - 1 as ::core::ffi::c_int) as DWORD as SIZE;
                            (*StarShipPtr).cur_status_flags = ((*StarShipPtr)
                                .cur_status_flags as ::core::ffi::c_int
                                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int)
                                as STATUS_FLAGS;
                        }
                        if facing as ::core::ffi::c_int
                            == (*StarShipPtr).ShipFacing as ::core::ffi::c_int
                        {
                            memset(
                                &raw mut (*ElementPtr).velocity as *mut ::core::ffi::c_void,
                                0 as ::core::ffi::c_int,
                                ::core::mem::size_of::<VELOCITY_DESC>(),
                            );
                        }
                    }
                    GetCurrentVelocityComponents(
                        &raw mut (*ElementPtr).velocity,
                        &raw mut dx,
                        &raw mut dy,
                    );
                    if (GlobData.SIS_state.FuelOnBoard != 0
                        || uqm_lua_checked!(getGameStateUint(
                            b"ARILOU_SPACE_SIDE\0".as_ptr() as *const ::core::ffi::c_char,
                        ), ()) > 1 as DWORD)
                        && facing as ::core::ffi::c_int
                            == (*StarShipPtr).ShipFacing as ::core::ffi::c_int
                    {
                        (*StarShipPtr).cur_status_flags = ((*StarShipPtr)
                            .cur_status_flags as ::core::ffi::c_int
                            | (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int)
                            as STATUS_FLAGS;
                        AccelerateDirection = 1 as SIZE;
                    } else {
                        AccelerateDirection = -1 as SIZE;
                        udx = ((dx as ::core::ffi::c_int)
                            << if if resolutionFactor != 2 as ::core::ffi::c_uint {
                                BOOLEAN::FALSE.0 as ::core::ffi::c_int
                            } else {
                                BOOLEAN::TRUE.0 as ::core::ffi::c_int
                            } == 0
                            {
                                0 as ::core::ffi::c_int
                            } else {
                                4 as ::core::ffi::c_int
                            }) as SDWORD;
                        udy = ((dy as ::core::ffi::c_int)
                            << if if resolutionFactor != 2 as ::core::ffi::c_uint {
                                BOOLEAN::FALSE.0 as ::core::ffi::c_int
                            } else {
                                BOOLEAN::TRUE.0 as ::core::ffi::c_int
                            } == 0
                            {
                                0 as ::core::ffi::c_int
                            } else {
                                4 as ::core::ffi::c_int
                            }) as SDWORD;
                    }
                    break 's_230;
                }
            }
        }
        GlobData.Game_state.autopilot.y = !(0 as ::core::ffi::c_int) as COORD;
        GlobData.Game_state.autopilot.x = GlobData.Game_state.autopilot.y;
        if (*StarShipPtr).cur_status_flags as ::core::ffi::c_int
            & ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) != 0
            && (ValidPoint(uqm_lua_checked!(LoadAdvancedAutoPilot(), ())).0 != 0
                || ValidPoint(uqm_lua_checked!(LoadAdvancedQuasiPilot(), ())).0 != 0)
        {
            uqm_lua_checked!(ZeroAdvancedAutoPilot(), ());
            uqm_lua_checked!(ZeroAdvancedQuasiPilot(), ());
        }
        if (*StarShipPtr).cur_status_flags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int == 0
            || GlobData.SIS_state.FuelOnBoard == 0 as DWORD
                && uqm_lua_checked!(getGameStateUint(
                    b"ARILOU_SPACE_SIDE\0".as_ptr() as *const ::core::ffi::c_char,
                ), ()) <= 1 as DWORD
        {
            AccelerateDirection = -1 as SIZE;
            GetCurrentVelocityComponents(
                &raw mut (*ElementPtr).velocity,
                &raw mut dx,
                &raw mut dy,
            );
            udx = (if if resolutionFactor != 2 as ::core::ffi::c_uint {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } == 0
            {
                (dx as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            } else {
                dx as ::core::ffi::c_int
            }) as SDWORD;
            udy = (if if resolutionFactor != 2 as ::core::ffi::c_uint {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } == 0
            {
                (dy as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            } else {
                dy as ::core::ffi::c_int
            }) as SDWORD;
            (*StarShipPtr).cur_status_flags = ((*StarShipPtr).cur_status_flags
                as ::core::ffi::c_int
                & !((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int))
                as STATUS_FLAGS;
        }
    }
    if (*ElementPtr).c2rust_unnamed_3.thrust_wait as ::core::ffi::c_int
        == 0 as ::core::ffi::c_int && AccelerateDirection as ::core::ffi::c_int != 0
    {
        let mut dist: COUNT = 0;
        let mut speed: SIZE = 0;
        let mut velocity_increment: SIZE = 0;
        velocity_increment = (((*(*StarShipPtr).RaceDescPtr)
            .characteristics
            .thrust_increment as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as SIZE;
        dist = square_root(
            (udx as ::core::ffi::c_long * udx as ::core::ffi::c_long
                + udy as ::core::ffi::c_long * udy as ::core::ffi::c_long) as DWORD,
        );
        if dist as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            dist = 1 as COUNT;
        }
        speed = square_root(
            (dx as ::core::ffi::c_long * dx as ::core::ffi::c_long
                + dy as ::core::ffi::c_long * dy as ::core::ffi::c_long) as DWORD,
        ) as SIZE;
        if (AccelerateDirection as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
            dy = ((speed as ::core::ffi::c_int / velocity_increment as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) * velocity_increment as ::core::ffi::c_int)
                as SIZE;
            if (dy as ::core::ffi::c_int)
                < speed as ::core::ffi::c_int - velocity_increment as ::core::ffi::c_int
            {
                dy = (speed as ::core::ffi::c_int
                    - velocity_increment as ::core::ffi::c_int) as SIZE;
            }
            speed = dy;
            if (speed as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                speed = 0 as SIZE;
            }
            (*StarShipPtr).cur_status_flags = ((*StarShipPtr).cur_status_flags
                as ::core::ffi::c_int
                & !((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int))
                as STATUS_FLAGS;
        } else {
            let mut max_velocity: SIZE = 0;
            AccelerateDirection = 0 as SIZE;
            max_velocity = (((*(*StarShipPtr).RaceDescPtr).characteristics.max_thrust
                as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as SIZE;
            dy = ((speed as ::core::ffi::c_int / velocity_increment as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int) * velocity_increment as ::core::ffi::c_int)
                as SIZE;
            if (dy as ::core::ffi::c_int)
                < speed as ::core::ffi::c_int + velocity_increment as ::core::ffi::c_int
            {
                dy = (speed as ::core::ffi::c_int
                    + velocity_increment as ::core::ffi::c_int) as SIZE;
            }
            speed = dy;
            if speed as ::core::ffi::c_int > max_velocity as ::core::ffi::c_int {
                speed = max_velocity;
                (*StarShipPtr).cur_status_flags = ((*StarShipPtr).cur_status_flags
                    as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int)
                    as STATUS_FLAGS;
            }
        }
        dx = (udx as ::core::ffi::c_long * speed as ::core::ffi::c_long
            / dist as ::core::ffi::c_long) as SDWORD as SIZE;
        dy = (udy as ::core::ffi::c_long * speed as ::core::ffi::c_long
            / dist as ::core::ffi::c_long) as SDWORD as SIZE;
        SetVelocityComponents(
            &raw mut (*ElementPtr).velocity,
            dx as SDWORD,
            dy as SDWORD,
        );
        (*ElementPtr).c2rust_unnamed_3.thrust_wait = (*(*StarShipPtr).RaceDescPtr)
            .characteristics
            .thrust_wait;
    }
}
unsafe extern "C" fn sis_hyper_postprocess(mut ElementPtr: *mut ELEMENT) {
    let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
    GlobData.Game_state.velocity = (*ElementPtr).velocity;
    StarShipPtr = (*ElementPtr).pParent as *mut STARSHIP;
    if ((*StarShipPtr).cur_status_flags as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int != 0
        || PulsedInputState
            .menu[C2Rust_Unnamed::KEY_MENU_CANCEL.0 as ::core::ffi::c_int as usize] != 0)
        && (*StarShipPtr).special_counter as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(HyperspaceMenu(), ());
        (*StarShipPtr).cur_status_flags = ((*StarShipPtr).cur_status_flags
            as ::core::ffi::c_int
            & !((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int)) as STATUS_FLAGS;
        (*StarShipPtr).special_counter = 10 as BYTE;
    }
}
unsafe extern "C" fn spawn_point_defense(mut ElementPtr: *mut ELEMENT) {
    let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
    StarShipPtr = (*ElementPtr).pParent as *mut STARSHIP;
    if (*ElementPtr).state_flags as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int != 0
    {
        let mut hDefense: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
        hDefense = AllocElement();
        if !hDefense.is_null() {
            let mut DefensePtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
            DefensePtr = LockLink(&raw mut disp_q, hDefense) as *mut ELEMENT;
            (*DefensePtr).playerNr = (*ElementPtr).playerNr;
            (*DefensePtr).state_flags = ((1 as ::core::ffi::c_int)
                << 3 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int)
                as ELEMENT_FLAGS;
            (*DefensePtr).death_func = Some(
                spawn_point_defense as unsafe extern "C" fn(*mut ELEMENT) -> (),
            ) as Option<ElementProcessFunc>;
            StarShipPtr = (*ElementPtr).pParent as *mut STARSHIP;
            (*DefensePtr).pParent = StarShipPtr as *mut ::core::ffi::c_void;
            UnlockLink(&raw mut disp_q, hDefense);
            PutQueue(&raw mut disp_q, hDefense);
        }
    } else {
        let mut PaidFor: BOOLEAN = BOOLEAN::FALSE;
        let mut hObject: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
        let mut hNextObject: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
        let mut ShipPtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
        let mut LaserColor: Color = Color { r: 0, g: 0, b: 0, a: 0 };
        static mut ColorRange: [Color; 8] = [
            Color {
                r: ((0x1f as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x1f as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                g: ((0x3 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x3 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                a: 0xff as BYTE,
            },
            Color {
                r: ((0x1f as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x1f as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                g: ((0x7 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x7 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                a: 0xff as BYTE,
            },
            Color {
                r: ((0x1f as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x1f as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                g: ((0xa as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0xa as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                a: 0xff as BYTE,
            },
            Color {
                r: ((0x1f as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x1f as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                g: ((0xe as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0xe as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                a: 0xff as BYTE,
            },
            Color {
                r: ((0x1f as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x1f as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                g: ((0x11 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x11 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                a: 0xff as BYTE,
            },
            Color {
                r: ((0x1f as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x1f as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                g: ((0x15 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x15 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                a: 0xff as BYTE,
            },
            Color {
                r: ((0x1f as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x1f as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                g: ((0x18 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x18 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                a: 0xff as BYTE,
            },
            Color {
                r: ((0x1f as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x1f as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                g: ((0x1c as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x1c as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                a: 0xff as BYTE,
            },
        ];
        PaidFor = BOOLEAN::FALSE;
        LaserColor = ColorRange[(*(*StarShipPtr).RaceDescPtr)
            .characteristics
            .special_energy_cost as usize];
        ShipPtr = LockLink(&raw mut disp_q, (*StarShipPtr).hShip) as *mut ELEMENT;
        hObject = disp_q.tail as HELEMENT;
        while !hObject.is_null() {
            let mut ObjectPtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
            ObjectPtr = LockLink(&raw mut disp_q, hObject) as *mut ELEMENT;
            hNextObject = (*ObjectPtr).pred;
            if ObjectPtr != ShipPtr
                && (*ObjectPtr).state_flags as ::core::ffi::c_int
                    & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                        | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) == 0
                && !((*GlobData
                    .Game_state
                    .DisplayArray
                    .offset((*ObjectPtr).PrimIndex as isize))
                    .Type as ::core::ffi::c_int
                    >= gfx_object::NUM_PRIMS.0 as ::core::ffi::c_int
                    || (*GlobData
                        .Game_state
                        .DisplayArray
                        .offset((*ObjectPtr).PrimIndex as isize))
                        .Type as ::core::ffi::c_int
                        == gfx_object::STAMPFILL_PRIM.0 as ::core::ffi::c_int
                        && sameColor(
                            (*GlobData
                                .Game_state
                                .DisplayArray
                                .offset((*ObjectPtr).PrimIndex as isize))
                                .color,
                            colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE),
                        ) as ::core::ffi::c_int != 0)
            {
                let mut delta_x: SDWORD = 0;
                let mut delta_y: SDWORD = 0;
                delta_x = ((*ObjectPtr).next.location.x as ::core::ffi::c_int
                    - (*ShipPtr).next.location.x as ::core::ffi::c_int) as SDWORD;
                delta_y = ((*ObjectPtr).next.location.y as ::core::ffi::c_int
                    - (*ShipPtr).next.location.y as ::core::ffi::c_int) as SDWORD;
                if delta_x < 0 as ::core::ffi::c_int {
                    delta_x = -delta_x;
                }
                if delta_y < 0 as ::core::ffi::c_int {
                    delta_y = -delta_y;
                }
                delta_x = delta_x >> 2 as ::core::ffi::c_int;
                delta_y = delta_y >> 2 as ::core::ffi::c_int;
                if delta_x as UWORD as ::core::ffi::c_int
                    <= ((100 as ::core::ffi::c_int) << resolutionFactor) as UWORD
                        as ::core::ffi::c_int
                    && delta_y as UWORD as ::core::ffi::c_int
                        <= ((100 as ::core::ffi::c_int) << resolutionFactor) as UWORD
                            as ::core::ffi::c_int
                    && delta_x as UWORD as ::core::ffi::c_int
                        * delta_x as UWORD as ::core::ffi::c_int
                        + delta_y as UWORD as ::core::ffi::c_int
                            * delta_y as UWORD as ::core::ffi::c_int
                        <= ((100 as ::core::ffi::c_int) << resolutionFactor) as UWORD
                            as ::core::ffi::c_int
                            * ((100 as ::core::ffi::c_int) << resolutionFactor) as UWORD
                                as ::core::ffi::c_int
                {
                    let mut hPointDefense: HELEMENT = ::core::ptr::null_mut::<
                        ::core::ffi::c_void,
                    >();
                    let mut LaserBlock: LASER_BLOCK = LASER_BLOCK {
                        cx: 0,
                        cy: 0,
                        ex: 0,
                        ey: 0,
                        flags: 0,
                        sender: 0,
                        pixoffs: 0,
                        face: 0,
                        color: Color { r: 0, g: 0, b: 0, a: 0 },
                    };
                    if PaidFor.0 == 0 {
                        if uqm_lua_checked!(DeltaEnergy(
                                ShipPtr,
                                -(((*(*StarShipPtr).RaceDescPtr)
                                    .characteristics
                                    .special_energy_cost as ::core::ffi::c_int)
                                    << 2 as ::core::ffi::c_int) as SIZE,
                            ), ())
                            .0 == 0
                        {
                            break;
                        }
                        ProcessSound(
                            SetAbsStringTableIndex(
                                (*(*StarShipPtr).RaceDescPtr).ship_data.ship_sounds,
                                1 as COUNT,
                            ),
                            ElementPtr,
                        );
                        (*StarShipPtr).special_counter = (*(*StarShipPtr).RaceDescPtr)
                            .characteristics
                            .special_wait;
                        PaidFor = BOOLEAN::TRUE;
                    }
                    LaserBlock.cx = (*ShipPtr).next.location.x;
                    LaserBlock.cy = (*ShipPtr).next.location.y;
                    LaserBlock.face = 0 as COUNT;
                    LaserBlock.ex = ((*ObjectPtr).next.location.x as ::core::ffi::c_int
                        - (*ShipPtr).next.location.x as ::core::ffi::c_int) as COORD;
                    LaserBlock.ey = ((*ObjectPtr).next.location.y as ::core::ffi::c_int
                        - (*ShipPtr).next.location.y as ::core::ffi::c_int) as COORD;
                    LaserBlock.sender = (*ShipPtr).playerNr;
                    LaserBlock.flags = ((1 as ::core::ffi::c_int)
                        << 8 as ::core::ffi::c_int) as ELEMENT_FLAGS;
                    LaserBlock.pixoffs = 0 as SIZE;
                    LaserBlock.color = LaserColor;
                    hPointDefense = initialize_laser(&raw mut LaserBlock);
                    if !hPointDefense.is_null() {
                        let mut PDPtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
                        PDPtr = LockLink(&raw mut disp_q, hPointDefense) as *mut ELEMENT;
                        (*PDPtr).c2rust_unnamed_1.mass_points = (*(*StarShipPtr)
                            .RaceDescPtr)
                            .characteristics
                            .special_energy_cost;
                        (*PDPtr).pParent = StarShipPtr as *mut ::core::ffi::c_void;
                        (*PDPtr).hTarget = ::core::ptr::null_mut::<
                            ::core::ffi::c_void,
                        >();
                        UnlockLink(&raw mut disp_q, hPointDefense);
                        PutQueue(&raw mut disp_q, hPointDefense);
                    }
                }
            }
            UnlockLink(&raw mut disp_q, hObject);
            hObject = hNextObject;
        }
        UnlockLink(&raw mut disp_q, (*StarShipPtr).hShip);
    };
}
unsafe extern "C" fn sis_battle_preprocess(mut ElementPtr: *mut ELEMENT) {
    let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
    StarShipPtr = (*ElementPtr).pParent as *mut STARSHIP;
    if (*(*StarShipPtr).RaceDescPtr).characteristics.special_energy_cost
        as ::core::ffi::c_int == 0 as ::core::ffi::c_int
    {
        (*StarShipPtr).cur_status_flags = ((*StarShipPtr).cur_status_flags
            as ::core::ffi::c_int
            & !((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)) as STATUS_FLAGS;
        (*StarShipPtr).special_counter = 2 as BYTE;
    }
    if (*(*StarShipPtr).RaceDescPtr).ship_info.ship_flags as ::core::ffi::c_int
        & ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int) == 0
    {
        (*StarShipPtr).cur_status_flags = ((*StarShipPtr).cur_status_flags
            as ::core::ffi::c_int
            & !((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)) as STATUS_FLAGS;
        (*StarShipPtr).weapon_counter = 2 as BYTE;
    }
}
unsafe extern "C" fn sis_battle_postprocess(mut ElementPtr: *mut ELEMENT) {
    let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
    StarShipPtr = (*ElementPtr).pParent as *mut STARSHIP;
    if (*StarShipPtr).cur_status_flags as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int != 0
        && (*StarShipPtr).special_counter as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        && (*(*StarShipPtr).RaceDescPtr).characteristics.special_energy_cost
            as ::core::ffi::c_int != 0
    {
        uqm_lua_checked!(spawn_point_defense(ElementPtr), ());
    }
}
unsafe extern "C" fn blaster_collision(
    mut ElementPtr0: *mut ELEMENT,
    mut pPt0: *mut POINT,
    mut ElementPtr1: *mut ELEMENT,
    mut pPt1: *mut POINT,
) {
    let mut hBlastElement: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
    hBlastElement = uqm_lua_checked!(weapon_collision(ElementPtr0, pPt0, ElementPtr1, pPt1), ());
    if !hBlastElement.is_null() {
        let mut BlastElementPtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
        BlastElementPtr = LockLink(&raw mut disp_q, hBlastElement) as *mut ELEMENT;
        match (*ElementPtr0).c2rust_unnamed_1.mass_points as ::core::ffi::c_int {
            2 => {
                (*BlastElementPtr).c2rust_unnamed.life_span = 2 as COUNT;
                (*BlastElementPtr).current.image.frame = SetAbsFrameIndex(
                    (*ElementPtr0).current.image.frame,
                    0 as COUNT,
                );
                (*BlastElementPtr).preprocess_func = None;
            }
            4 => {
                (*BlastElementPtr).c2rust_unnamed.life_span = 6 as COUNT;
                (*BlastElementPtr).current.image.frame = IncFrameIndex(
                    (*ElementPtr0).current.image.frame,
                );
            }
            6 => {
                (*BlastElementPtr).c2rust_unnamed.life_span = 7 as COUNT;
                (*BlastElementPtr).current.image.frame = SetAbsFrameIndex(
                    (*ElementPtr0).current.image.frame,
                    20 as COUNT,
                );
            }
            _ => {}
        }
        UnlockLink(&raw mut disp_q, hBlastElement);
    }
}
unsafe extern "C" fn blaster_preprocess(mut ElementPtr: *mut ELEMENT) {
    let mut wait: BYTE = 0;
    match (*ElementPtr).c2rust_unnamed_1.mass_points as ::core::ffi::c_int {
        2 => {
            if (GetFrameIndex((*ElementPtr).current.image.frame) as ::core::ffi::c_int)
                < 8 as ::core::ffi::c_int
            {
                (*ElementPtr).next.image.frame = IncFrameIndex(
                    (*ElementPtr).current.image.frame,
                );
                (*ElementPtr).state_flags = ((*ElementPtr).state_flags
                    as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int)
                    as ELEMENT_FLAGS;
            }
        }
        6 => {
            if (GetFrameIndex((*ElementPtr).current.image.frame) as ::core::ffi::c_int)
                < 19 as ::core::ffi::c_int
            {
                (*ElementPtr).next.image.frame = IncFrameIndex(
                    (*ElementPtr).current.image.frame,
                );
            } else {
                (*ElementPtr).next.image.frame = SetAbsFrameIndex(
                    (*ElementPtr).current.image.frame,
                    16 as COUNT,
                );
            }
            (*ElementPtr).state_flags = ((*ElementPtr).state_flags as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as ELEMENT_FLAGS;
        }
        _ => {}
    }
    if ((*ElementPtr).c2rust_unnamed_2.turn_wait as ::core::ffi::c_int
        & 0xf as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE != 0
    {
        (*ElementPtr).c2rust_unnamed_2.turn_wait = (*ElementPtr)
            .c2rust_unnamed_2
            .turn_wait
            .wrapping_sub(1);
    } else {
        wait = ((*ElementPtr).c2rust_unnamed_2.turn_wait as ::core::ffi::c_int
            >> 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE;
        if wait != 0 {
            let mut facing: COUNT = 0;
            facing = ((*ElementPtr).velocity.TravelAngle as ::core::ffi::c_int
                + ((1 as ::core::ffi::c_int)
                    << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int)
                >> 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                & ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int) as DWORD as COUNT;
            if TrackShip(ElementPtr, &raw mut facing) as ::core::ffi::c_int
                > 0 as ::core::ffi::c_int
            {
                SetVelocityVector(
                    &raw mut (*ElementPtr).velocity,
                    if if resolutionFactor != 2 as ::core::ffi::c_uint {
                        BOOLEAN::FALSE.0 as ::core::ffi::c_int
                    } else {
                        BOOLEAN::TRUE.0 as ::core::ffi::c_int
                    } == 0
                    {
                        (24 as SDWORD) << 2 as ::core::ffi::c_int
                    } else {
                        ((24 as SDWORD) << 2 as ::core::ffi::c_int) << resolutionFactor
                    },
                    facing,
                );
            }
            (*ElementPtr).c2rust_unnamed_2.turn_wait = ((wait as ::core::ffi::c_int)
                << 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int
                | wait as ::core::ffi::c_int) as BYTE;
        }
    };
}
unsafe extern "C" fn initialize_blasters(
    mut ShipPtr: *mut ELEMENT,
    mut BlasterArray: *mut HELEMENT,
) -> COUNT {
    let mut nt: BYTE = 0;
    let mut i: COUNT = 0;
    let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
    let mut SisData: *mut SIS_DATA = ::core::ptr::null_mut::<SIS_DATA>();
    StarShipPtr = (*ShipPtr).pParent as *mut STARSHIP;
    SisData = GetCustomShipData((*StarShipPtr).RaceDescPtr) as *mut SIS_DATA;
    nt = (4 as ::core::ffi::c_int - (*SisData).num_trackers as ::core::ffi::c_int
        & 3 as ::core::ffi::c_int) as BYTE;
    i = 0 as COUNT;
    while (i as ::core::ffi::c_int) < (*SisData).num_blasters as ::core::ffi::c_int {
        let mut MissileBlock: MISSILE_BLOCK = (*SisData).MissileBlock[i as usize];
        MissileBlock.cx = (*ShipPtr).next.location.x;
        MissileBlock.cy = (*ShipPtr).next.location.y;
        MissileBlock.farray = &raw mut (*(*StarShipPtr).RaceDescPtr).ship_data.weapon
            as *mut FRAME;
        MissileBlock.sender = (*ShipPtr).playerNr;
        MissileBlock.face = ((*StarShipPtr).ShipFacing as ::core::ffi::c_int
            + MissileBlock.face as ::core::ffi::c_int
            & ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                - 1 as ::core::ffi::c_int) as DWORD as COUNT;
        *BlasterArray.offset(i as isize) = initialize_missile(&raw mut MissileBlock);
        if !(*BlasterArray.offset(i as isize)).is_null() {
            let mut BlasterPtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
            BlasterPtr = LockLink(&raw mut disp_q, *BlasterArray.offset(i as isize))
                as *mut ELEMENT;
            (*BlasterPtr).collision_func = Some(
                blaster_collision
                    as unsafe extern "C" fn(
                        *mut ELEMENT,
                        *mut POINT,
                        *mut ELEMENT,
                        *mut POINT,
                    ) -> (),
            ) as Option<ElementCollisionFunc>;
            (*BlasterPtr).c2rust_unnamed_2.turn_wait = ((nt as ::core::ffi::c_int)
                << 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int
                | nt as ::core::ffi::c_int) as BYTE;
            UnlockLink(&raw mut disp_q, *BlasterArray.offset(i as isize));
        }
        i = i.wrapping_add(1);
    }
    return (*SisData).num_blasters;
}
unsafe extern "C" fn sis_intelligence(
    mut ShipPtr: *mut ELEMENT,
    mut ObjectsOfConcern: *mut EVALUATE_DESC,
    mut ConcernCounter: COUNT,
) {
    let mut lpEvalDesc: *mut EVALUATE_DESC = ::core::ptr::null_mut::<EVALUATE_DESC>();
    let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
    let mut SisData: *mut SIS_DATA = ::core::ptr::null_mut::<SIS_DATA>();
    StarShipPtr = (*ShipPtr).pParent as *mut STARSHIP;
    SisData = GetCustomShipData((*StarShipPtr).RaceDescPtr) as *mut SIS_DATA;
    lpEvalDesc = ObjectsOfConcern
        .offset(C2Rust_Unnamed_8::ENEMY_WEAPON_INDEX.0 as ::core::ffi::c_int as isize);
    if !(*lpEvalDesc).ObjectPtr.is_null() {
        if (*(*StarShipPtr).RaceDescPtr).characteristics.special_energy_cost != 0 {
            if (*StarShipPtr).special_counter as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
                && (!(*lpEvalDesc).ObjectPtr.is_null()
                    && (*lpEvalDesc).which_turn as ::core::ffi::c_int
                        <= 2 as ::core::ffi::c_int
                    || !(*ObjectsOfConcern
                        .offset(
                            C2Rust_Unnamed_8::ENEMY_SHIP_INDEX.0 as ::core::ffi::c_int
                                as isize,
                        ))
                        .ObjectPtr
                        .is_null()
                        && (*ObjectsOfConcern
                            .offset(
                                C2Rust_Unnamed_8::ENEMY_SHIP_INDEX.0 as ::core::ffi::c_int
                                    as isize,
                            ))
                            .which_turn as ::core::ffi::c_int <= 4 as ::core::ffi::c_int)
            {
                (*StarShipPtr).ship_input_state = ((*StarShipPtr).ship_input_state
                    as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as BYTE;
            } else {
                (*StarShipPtr).ship_input_state = ((*StarShipPtr).ship_input_state
                    as ::core::ffi::c_int
                    & !((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)) as BYTE;
            }
            (*lpEvalDesc).ObjectPtr = ::core::ptr::null_mut::<ELEMENT>();
        } else if ((*(*StarShipPtr).RaceDescPtr).cyborg_control.ManeuverabilityIndex
            as ::core::ffi::c_int)
            < (45 as ::core::ffi::c_int) << resolutionFactor << resolutionFactor
            && (*lpEvalDesc).MoveState.0 == MOVEMENT_STATE::ENTICE.0
            && ((*(*lpEvalDesc).ObjectPtr).state_flags as ::core::ffi::c_int
                & (1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int == 0
                || (*lpEvalDesc).which_turn as ::core::ffi::c_int
                    <= 8 as ::core::ffi::c_int)
            && ((*(*lpEvalDesc).ObjectPtr).state_flags as ::core::ffi::c_int
                & (1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int == 0
                || (*(*lpEvalDesc).ObjectPtr).c2rust_unnamed_1.mass_points
                    as ::core::ffi::c_int >= 4 as ::core::ffi::c_int
                    && (*lpEvalDesc).which_turn as ::core::ffi::c_int
                        == 2 as ::core::ffi::c_int
                    && (*ObjectsOfConcern
                        .offset(
                            C2Rust_Unnamed_8::ENEMY_SHIP_INDEX.0 as ::core::ffi::c_int
                                as isize,
                        ))
                        .which_turn as ::core::ffi::c_int > 16 as ::core::ffi::c_int)
        {
            (*lpEvalDesc).MoveState = MOVEMENT_STATE::PURSUE;
        }
    }
    uqm_lua_checked!(ship_intelligence(ShipPtr, ObjectsOfConcern, ConcernCounter), ());
    lpEvalDesc = ObjectsOfConcern
        .offset(C2Rust_Unnamed_8::ENEMY_SHIP_INDEX.0 as ::core::ffi::c_int as isize);
    if (*SisData).num_trackers as ::core::ffi::c_int != 0
        && (*StarShipPtr).weapon_counter as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        && (*StarShipPtr).ship_input_state as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int == 0
        && !(*lpEvalDesc).ObjectPtr.is_null()
        && (*lpEvalDesc).which_turn as ::core::ffi::c_int <= 16 as ::core::ffi::c_int
    {
        let mut direction_facing: COUNT = 0;
        let mut delta_x: SDWORD = 0;
        let mut delta_y: SDWORD = 0;
        let mut fire_flags: UWORD = 0;
        let mut ship_flags: UWORD = 0;
        let mut facing: COUNT = 0;
        delta_x = ((*(*lpEvalDesc).ObjectPtr).current.location.x as ::core::ffi::c_int
            - (*ShipPtr).current.location.x as ::core::ffi::c_int) as SDWORD;
        delta_y = ((*(*lpEvalDesc).ObjectPtr).current.location.y as ::core::ffi::c_int
            - (*ShipPtr).current.location.y as ::core::ffi::c_int) as SDWORD;
        direction_facing = (ARCTAN(delta_x, delta_y) as ::core::ffi::c_int
            + ((1 as ::core::ffi::c_int)
                << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int)
            >> 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
            & ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                - 1 as ::core::ffi::c_int) as DWORD as COUNT;
        ship_flags = (*(*StarShipPtr).RaceDescPtr).ship_info.ship_flags;
        fire_flags = ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as UWORD;
        facing = (*StarShipPtr).ShipFacing;
        while fire_flags as ::core::ffi::c_int
            <= (1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int
        {
            if ship_flags as ::core::ffi::c_int & fire_flags as ::core::ffi::c_int != 0
                && (direction_facing as ::core::ffi::c_int - facing as ::core::ffi::c_int
                    + (((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                        >> 3 as ::core::ffi::c_int)
                        + ((1 as ::core::ffi::c_int)
                            << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int)
                        >> 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int)
                    & ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int) as DWORD
                    <= (((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                        >> 2 as ::core::ffi::c_int)
                        + ((1 as ::core::ffi::c_int)
                            << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int)
                        >> 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int) as DWORD
            {
                (*StarShipPtr).ship_input_state = ((*StarShipPtr).ship_input_state
                    as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as BYTE;
                break;
            } else {
                fire_flags = ((fire_flags as ::core::ffi::c_int)
                    << 1 as ::core::ffi::c_int) as UWORD;
                facing = (facing as ::core::ffi::c_int
                    + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                        >> 2 as ::core::ffi::c_int)) as COUNT;
            }
        }
    }
}
unsafe extern "C" fn InitWeaponSlots(
    mut RaceDescPtr: *mut RACE_DESC,
    mut ModuleSlots: *const BYTE,
) {
    let mut i: COUNT = 0;
    let mut SisData: *mut SIS_DATA = GetCustomShipData(RaceDescPtr);
    let mut lpMB: *mut MISSILE_BLOCK = &raw mut (*SisData).MissileBlock
        as *mut MISSILE_BLOCK;
    (*SisData).num_blasters = 0 as COUNT;
    i = 0 as COUNT;
    while (i as ::core::ffi::c_int) < 16 as ::core::ffi::c_int {
        let mut which_gun: COUNT = 0;
        let mut IfHard: BOOLEAN = BOOLEAN(
            (if if if GlobData.SIS_state.Difficulty as ::core::ffi::c_int != 0 {
                GlobData.SIS_state.Difficulty as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } == 2 as ::core::ffi::c_int
            {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } != 0
                && (i as ::core::ffi::c_int == 1 as ::core::ffi::c_int
                    || i as ::core::ffi::c_int == 2 as ::core::ffi::c_int)
            {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            }) as ::core::ffi::c_uint,
        );
        if i as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            i = (16 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as COUNT;
        }
        which_gun = *ModuleSlots
            .offset(
                (16 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                    - i as ::core::ffi::c_int) as isize,
            ) as COUNT;
        if !((which_gun as ::core::ffi::c_int)
            < C2Rust_Unnamed_6::GUN_WEAPON.0 as ::core::ffi::c_int
            || which_gun as ::core::ffi::c_int
                > C2Rust_Unnamed_6::CANNON_WEAPON.0 as ::core::ffi::c_int)
        {
            which_gun = (which_gun as ::core::ffi::c_int
                - (C2Rust_Unnamed_6::GUN_WEAPON.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int)) as COUNT;
            (*RaceDescPtr).characteristics.weapon_energy_cost = ((*RaceDescPtr)
                .characteristics
                .weapon_energy_cost as ::core::ffi::c_int
                + which_gun as ::core::ffi::c_int
                    * if IfHard.0 != 0 {
                        4 as ::core::ffi::c_int
                    } else {
                        2 as ::core::ffi::c_int
                    }) as BYTE;
            (*lpMB).flags = ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
                as ELEMENT_FLAGS;
            (*lpMB).blast_offs = ((8 as ::core::ffi::c_int) << resolutionFactor) as SIZE;
            (*lpMB).speed = (if if resolutionFactor != 2 as ::core::ffi::c_uint {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } == 0
            {
                (24 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
            } else {
                ((24 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                    << resolutionFactor
            }) as SIZE;
            (*lpMB).preprocess_func = Some(
                blaster_preprocess as unsafe extern "C" fn(*mut ELEMENT) -> (),
            ) as Option<unsafe extern "C" fn(*mut ELEMENT) -> ()>;
            (*lpMB).hit_points = (2 as ::core::ffi::c_int
                * which_gun as ::core::ffi::c_int) as SIZE;
            (*lpMB).damage = (2 as ::core::ffi::c_int * which_gun as ::core::ffi::c_int)
                as SIZE;
            (*lpMB).life = (12 as ::core::ffi::c_int
                + (12 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
                    * (which_gun as ::core::ffi::c_int - 1 as ::core::ffi::c_int))
                as COUNT;
            if which_gun as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
                (*lpMB).index = 0 as COUNT;
            } else if which_gun as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
                (*lpMB).index = 9 as COUNT;
            } else {
                (*lpMB).index = 16 as COUNT;
            }
            match i as ::core::ffi::c_int {
                0 => {
                    (*RaceDescPtr).ship_info.ship_flags = ((*RaceDescPtr)
                        .ship_info
                        .ship_flags as ::core::ffi::c_int
                        | (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as UWORD;
                    (*lpMB).pixoffs = ((28 as ::core::ffi::c_int) << resolutionFactor)
                        as SIZE;
                    (*lpMB).face = 0 as COUNT;
                }
                1 => {
                    (*RaceDescPtr).ship_info.ship_flags = ((*RaceDescPtr)
                        .ship_info
                        .ship_flags as ::core::ffi::c_int
                        | (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as UWORD;
                    (*lpMB).pixoffs = ((28 as ::core::ffi::c_int) << resolutionFactor)
                        as SIZE;
                    (*lpMB).face = 1 as ::core::ffi::c_int as COUNT;
                    *lpMB.offset(1isize) = *lpMB.offset(0isize);
                    lpMB = lpMB.offset(1);
                    (*SisData).num_blasters = (*SisData).num_blasters.wrapping_add(1);
                    (*lpMB).face = (-1 as ::core::ffi::c_int
                        & ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                            - 1 as ::core::ffi::c_int) as DWORD as COUNT;
                }
                2 => {
                    (*RaceDescPtr).ship_info.ship_flags = ((*RaceDescPtr)
                        .ship_info
                        .ship_flags as ::core::ffi::c_int
                        | ((1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int
                            | (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int))
                        as UWORD;
                    (*lpMB).pixoffs = ((20 as ::core::ffi::c_int) << resolutionFactor)
                        as SIZE;
                    (*lpMB).face = (((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                        >> 2 as ::core::ffi::c_int)
                        + ((1 as ::core::ffi::c_int)
                            << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int)
                        >> 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int) as COUNT;
                    *lpMB.offset(1isize) = *lpMB.offset(0isize);
                    lpMB = lpMB.offset(1);
                    (*SisData).num_blasters = (*SisData).num_blasters.wrapping_add(1);
                    (*lpMB).face = (-(((1 as ::core::ffi::c_int)
                        << 6 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
                        + ((1 as ::core::ffi::c_int)
                            << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int)
                        >> 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int)
                        & ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                            - 1 as ::core::ffi::c_int) as DWORD as COUNT;
                }
                15 => {
                    (*RaceDescPtr).ship_info.ship_flags = ((*RaceDescPtr)
                        .ship_info
                        .ship_flags as ::core::ffi::c_int
                        | (1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int) as UWORD;
                    (*lpMB).pixoffs = ((28 as ::core::ffi::c_int) << resolutionFactor)
                        as SIZE;
                    (*lpMB).face = (((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                        >> 1 as ::core::ffi::c_int)
                        + ((1 as ::core::ffi::c_int)
                            << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int)
                        >> 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int) as COUNT;
                }
                _ => {}
            }
            lpMB = lpMB.offset(1);
            (*SisData).num_blasters = (*SisData).num_blasters.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn InitModuleSlots(
    mut RaceDescPtr: *mut RACE_DESC,
    mut ModuleSlots: *const BYTE,
) {
    let mut i: COUNT = 0;
    let mut num_trackers: COUNT = 0;
    let mut SisData: *mut SIS_DATA = GetCustomShipData(RaceDescPtr);
    (*RaceDescPtr).ship_info.max_crew = 0 as COUNT;
    num_trackers = 0 as COUNT;
    i = 0 as COUNT;
    while (i as ::core::ffi::c_int) < 16 as ::core::ffi::c_int {
        let mut which_mod: BYTE = 0;
        which_mod = *ModuleSlots
            .offset(
                (16 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                    - i as ::core::ffi::c_int) as isize,
            );
        match which_mod as ::core::ffi::c_int {
            3 => {
                (*RaceDescPtr).ship_info.max_crew = ((*RaceDescPtr).ship_info.max_crew
                    as ::core::ffi::c_int + 50 as ::core::ffi::c_int) as COUNT;
            }
            12 => {
                num_trackers = num_trackers.wrapping_add(1);
            }
            13 => {
                (*RaceDescPtr).characteristics.special_energy_cost = ((*RaceDescPtr)
                    .characteristics
                    .special_energy_cost as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                    as BYTE;
            }
            8 => {
                (*RaceDescPtr).characteristics.energy_regeneration = ((*RaceDescPtr)
                    .characteristics
                    .energy_regeneration as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                    as BYTE;
            }
            7 => {
                (*RaceDescPtr).characteristics.energy_wait = ((*RaceDescPtr)
                    .characteristics
                    .energy_wait as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
                    as BYTE;
                if ((*RaceDescPtr).characteristics.energy_wait as ::core::ffi::c_int)
                    < 4 as ::core::ffi::c_int
                {
                    (*RaceDescPtr).characteristics.energy_wait = 4 as BYTE;
                }
            }
            _ => {}
        }
        i = i.wrapping_add(1);
    }
    if num_trackers as ::core::ffi::c_int > 3 as ::core::ffi::c_int {
        num_trackers = 3 as COUNT;
    }
    (*RaceDescPtr).characteristics.weapon_energy_cost = ((*RaceDescPtr)
        .characteristics
        .weapon_energy_cost as ::core::ffi::c_int
        + num_trackers as ::core::ffi::c_int
            * if if if GlobData.SIS_state.Difficulty as ::core::ffi::c_int != 0 {
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
                3 as ::core::ffi::c_int
            } else {
                6 as ::core::ffi::c_int
            }) as BYTE;
    (*SisData).num_trackers = num_trackers;
    if (*RaceDescPtr).characteristics.special_energy_cost != 0 {
        (*RaceDescPtr).ship_info.ship_flags = ((*RaceDescPtr).ship_info.ship_flags
            as ::core::ffi::c_int | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
            as UWORD;
        if (*RaceDescPtr).characteristics.special_energy_cost as ::core::ffi::c_int
            > 8 as ::core::ffi::c_int
        {
            (*RaceDescPtr).characteristics.special_energy_cost = 8 as BYTE;
        }
    }
}
unsafe extern "C" fn InitDriveSlots(
    mut RaceDescPtr: *mut RACE_DESC,
    mut DriveSlots: *const BYTE,
) {
    let mut i: COUNT = 0;
    (*RaceDescPtr).characteristics.thrust_wait = 0 as BYTE;
    i = 0 as COUNT;
    while (i as ::core::ffi::c_int) < 11 as ::core::ffi::c_int {
        match *DriveSlots.offset(i as isize) as ::core::ffi::c_int {
            1 => {
                (*RaceDescPtr).characteristics.max_thrust = ((*RaceDescPtr)
                    .characteristics
                    .max_thrust as ::core::ffi::c_int
                    + ((2 as ::core::ffi::c_int) << resolutionFactor)) as COUNT;
                (*RaceDescPtr).characteristics.thrust_wait = (*RaceDescPtr)
                    .characteristics
                    .thrust_wait
                    .wrapping_add(1);
            }
            _ => {}
        }
        i = i.wrapping_add(1);
    }
    (*RaceDescPtr).characteristics.thrust_wait = (6 as ::core::ffi::c_int
        - ((*RaceDescPtr).characteristics.thrust_wait as ::core::ffi::c_int
            >> 1 as ::core::ffi::c_int)) as BYTE;
    (*RaceDescPtr).characteristics.max_thrust = (((*RaceDescPtr)
        .characteristics
        .max_thrust as ::core::ffi::c_int
        / (*RaceDescPtr).characteristics.thrust_increment as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int)
        * (*RaceDescPtr).characteristics.thrust_increment as ::core::ffi::c_int)
        as COUNT;
}
unsafe extern "C" fn InitJetSlots(
    mut RaceDescPtr: *mut RACE_DESC,
    mut JetSlots: *const BYTE,
) {
    let mut i: COUNT = 0;
    i = 0 as COUNT;
    while (i as ::core::ffi::c_int) < 8 as ::core::ffi::c_int {
        match *JetSlots.offset(i as isize) as ::core::ffi::c_int {
            2 => {
                (*RaceDescPtr).characteristics.turn_wait = ((*RaceDescPtr)
                    .characteristics
                    .turn_wait as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as BYTE;
            }
            _ => {}
        }
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn init_sis() -> *mut RACE_DESC {
    let mut RaceDescPtr: *mut RACE_DESC = ::core::ptr::null_mut::<RACE_DESC>();
    let mut i: COUNT = 0;
    static mut new_sis_desc: RACE_DESC = RACE_DESC {
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
    let mut empty_data: SIS_DATA = SIS_DATA {
        num_trackers: 0,
        num_blasters: 0,
        MissileBlock: [MISSILE_BLOCK {
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
        }; 6],
    };
    memset(
        &raw mut empty_data as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<SIS_DATA>(),
    );
    if if resolutionFactor != 2 as ::core::ffi::c_uint {
        BOOLEAN::FALSE.0 as ::core::ffi::c_int
    } else {
        BOOLEAN::TRUE.0 as ::core::ffi::c_int
    } != 0
    {
        sis_desc.characteristics.max_thrust = ((10 as ::core::ffi::c_int)
            << resolutionFactor) as COUNT;
        sis_desc.characteristics.thrust_increment = ((4 as ::core::ffi::c_int)
            << resolutionFactor) as COUNT;
        sis_desc.cyborg_control.WeaponRange = ((((24 as ::core::ffi::c_int)
            << 2 as ::core::ffi::c_int) << resolutionFactor) * 12 as ::core::ffi::c_int)
            as COUNT;
    } else {
        sis_desc.characteristics.max_thrust = 10 as COUNT;
        sis_desc.characteristics.thrust_increment = 4 as COUNT;
        sis_desc.cyborg_control.WeaponRange = (((24 as ::core::ffi::c_int)
            << 2 as ::core::ffi::c_int) * 12 as ::core::ffi::c_int) as COUNT;
    }
    new_sis_desc = sis_desc;
    new_sis_desc.uninit_func = Some(
        uninit_sis as unsafe extern "C" fn(*mut RACE_DESC) -> (),
    ) as Option<UNINIT_FUNC>;
    if inHQSpace().0 != 0 {
        i = 0 as COUNT;
        while (i as ::core::ffi::c_int)
            < 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
        {
            new_sis_desc.ship_data.ship_rsc[i as usize] = ::core::ptr::null::<
                ::core::ffi::c_char,
            >();
            new_sis_desc.ship_data.weapon_rsc[i as usize] = ::core::ptr::null::<
                ::core::ffi::c_char,
            >();
            new_sis_desc.ship_data.special_rsc[i as usize] = ::core::ptr::null::<
                ::core::ffi::c_char,
            >();
            i = i.wrapping_add(1);
        }
        new_sis_desc.ship_info.icons_rsc = ::core::ptr::null::<::core::ffi::c_char>();
        new_sis_desc.ship_data.captain_control.captain_rsc = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        new_sis_desc.ship_data.victory_ditty_rsc = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        new_sis_desc.ship_data.ship_sounds_rsc = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        new_sis_desc.ship_data.ship_rsc[0usize] = b"ship.flagship.graphics.hyperspace\0"
            .as_ptr() as *const ::core::ffi::c_char as RESOURCE;
        new_sis_desc.preprocess_func = Some(
            sis_hyper_preprocess as unsafe extern "C" fn(*mut ELEMENT) -> (),
        ) as Option<PREPROCESS_FUNC>;
        new_sis_desc.postprocess_func = Some(
            sis_hyper_postprocess as unsafe extern "C" fn(*mut ELEMENT) -> (),
        ) as Option<POSTPROCESS_FUNC>;
        new_sis_desc.characteristics.max_thrust = (new_sis_desc
            .characteristics
            .max_thrust as ::core::ffi::c_int
            - ((4 as ::core::ffi::c_int) << resolutionFactor)) as COUNT;
    } else {
        new_sis_desc.preprocess_func = Some(
            sis_battle_preprocess as unsafe extern "C" fn(*mut ELEMENT) -> (),
        ) as Option<PREPROCESS_FUNC>;
        new_sis_desc.postprocess_func = Some(
            sis_battle_postprocess as unsafe extern "C" fn(*mut ELEMENT) -> (),
        ) as Option<POSTPROCESS_FUNC>;
        new_sis_desc.init_weapon_func = Some(
            initialize_blasters
                as unsafe extern "C" fn(*mut ELEMENT, *mut HELEMENT) -> COUNT,
        ) as Option<INIT_WEAPON_FUNC>;
        new_sis_desc.cyborg_control.intelligence_func = Some(
            sis_intelligence
                as unsafe extern "C" fn(*mut ELEMENT, *mut EVALUATE_DESC, COUNT) -> (),
        ) as Option<IntelligenceFunc>;
        if optFlagshipColor == 0x1 as ::core::ffi::c_int {
            new_sis_desc.ship_info.icons_rsc = b"ship.flagship.red.icons\0".as_ptr()
                as *const ::core::ffi::c_char as RESOURCE;
            new_sis_desc.ship_data.ship_rsc[0usize] = b"ship.flagship.graphics.flagship.red.large\0"
                .as_ptr() as *const ::core::ffi::c_char as RESOURCE;
            new_sis_desc.ship_data.ship_rsc[1usize] = b"ship.flagship.graphics.flagship.red.medium\0"
                .as_ptr() as *const ::core::ffi::c_char as RESOURCE;
            new_sis_desc.ship_data.ship_rsc[2usize] = b"ship.flagship.graphics.flagship.red.small\0"
                .as_ptr() as *const ::core::ffi::c_char as RESOURCE;
        } else {
            new_sis_desc.ship_info.icons_rsc = b"ship.flagship.icons\0".as_ptr()
                as *const ::core::ffi::c_char as RESOURCE;
            new_sis_desc.ship_data.ship_rsc[0usize] = b"ship.flagship.graphics.flagship.large\0"
                .as_ptr() as *const ::core::ffi::c_char as RESOURCE;
            new_sis_desc.ship_data.ship_rsc[1usize] = b"ship.flagship.graphics.flagship.medium\0"
                .as_ptr() as *const ::core::ffi::c_char as RESOURCE;
            new_sis_desc.ship_data.ship_rsc[2usize] = b"ship.flagship.graphics.flagship.small\0"
                .as_ptr() as *const ::core::ffi::c_char as RESOURCE;
        }
        if uqm_lua_checked!(getGameStateUint(b"CHMMR_BOMB_STATE\0".as_ptr() as *const ::core::ffi::c_char), ::core::ptr::null_mut())
            == 3 as DWORD
        {
            uqm_lua_checked!(setGameStateUint(
                b"BOMB_CARRIER\0".as_ptr() as *const ::core::ffi::c_char,
                1 as DWORD,
            ), ::core::ptr::null_mut());
        }
    }
    SetCustomShipData(&raw mut new_sis_desc, &raw mut empty_data);
    InitModuleSlots(
        &raw mut new_sis_desc,
        &raw mut GlobData.SIS_state.ModuleSlots as *mut BYTE,
    );
    InitWeaponSlots(
        &raw mut new_sis_desc,
        &raw mut GlobData.SIS_state.ModuleSlots as *mut BYTE,
    );
    InitDriveSlots(
        &raw mut new_sis_desc,
        &raw mut GlobData.SIS_state.DriveSlots as *mut BYTE,
    );
    InitJetSlots(
        &raw mut new_sis_desc,
        &raw mut GlobData.SIS_state.JetSlots as *mut BYTE,
    );
    if GlobData.Game_state.CurrentActivity as BYTE as ::core::ffi::c_int
        == C2Rust_Unnamed_7::SUPER_MELEE.0 as ::core::ffi::c_int
    {
        new_sis_desc.ship_info.crew_level = new_sis_desc.ship_info.max_crew;
    } else {
        new_sis_desc.ship_info.max_crew = new_sis_desc
            .ship_info
            .max_crew
            .wrapping_add(1);
        new_sis_desc.ship_info.crew_level = (GlobData.SIS_state.CrewEnlisted
            as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as COUNT;
        new_sis_desc.ship_info.ship_flags = (new_sis_desc.ship_info.ship_flags
            as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int) as UWORD;
    }
    new_sis_desc.ship_info.energy_level = new_sis_desc.ship_info.max_energy;
    RaceDescPtr = &raw mut new_sis_desc;
    return RaceDescPtr;
}
unsafe extern "C" fn uninit_sis(mut pRaceDesc: *mut RACE_DESC) {
    if inHQSpace().0 == 0 {
        GlobData.SIS_state.CrewEnlisted = (*pRaceDesc).ship_info.crew_level;
        if (*pRaceDesc).ship_info.ship_flags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int != 0
        {
            GlobData.SIS_state.CrewEnlisted = GlobData
                .SIS_state
                .CrewEnlisted
                .wrapping_sub(1);
        }
    }
    SetCustomShipData(pRaceDesc, ::core::ptr::null::<CustomShipData_t>());
}
