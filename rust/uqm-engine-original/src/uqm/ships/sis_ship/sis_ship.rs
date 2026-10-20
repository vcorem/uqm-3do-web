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
    );
    fn HMalloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn HFree(p: *mut ::core::ffi::c_void);
    fn PutQueue(pq: *mut QUEUE, hLink: HLINK);
    fn GetFrameIndex(Frame: FRAME) -> COUNT;
    fn SetAbsFrameIndex(Frame: FRAME, FrameIndex: COUNT) -> FRAME;
    fn IncFrameIndex(Frame: FRAME) -> FRAME;
    fn SetAbsStringTableIndex(String: STRING, StringTableIndex: COUNT) -> STRING;
    static mut ScreenWidth: ::core::ffi::c_int;
    static mut ScreenHeight: ::core::ffi::c_int;
    fn ARCTAN(delta_x: SIZE, delta_y: SIZE) -> COUNT;
    fn memset(
        _: *mut ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn GetCurrentVelocityComponents(
        velocityptr: *mut VELOCITY_DESC,
        pdx: *mut SIZE,
        pdy: *mut SIZE,
    );
    fn SetVelocityVector(
        velocityptr: *mut VELOCITY_DESC,
        magnitude: SIZE,
        facing: COUNT,
    );
    fn SetVelocityComponents(velocityptr: *mut VELOCITY_DESC, dx: SIZE, dy: SIZE);
    static mut disp_q: QUEUE;
    fn AllocElement() -> HELEMENT;
    static mut PulsedInputState: CONTROLLER_INPUT_STATE;
    fn square_root(value: DWORD) -> COUNT;
    fn ship_intelligence(
        ShipPtr: *mut ELEMENT,
        ObjectsOfConcern: *mut EVALUATE_DESC,
        ConcernCounter: COUNT,
    );
    fn DeltaEnergy(ElementPtr: *mut ELEMENT, energy_delta: SIZE) -> BOOLEAN;
    fn ProcessSound(Sound: SOUND, PositionalObject: *mut ELEMENT);
    fn initialize_laser(pLaserBlock: *mut LASER_BLOCK) -> HELEMENT;
    fn initialize_missile(pMissileBlock: *mut MISSILE_BLOCK) -> HELEMENT;
    fn weapon_collision(
        ElementPtr0: *mut ELEMENT,
        pPt0: *mut POINT,
        ElementPtr1: *mut ELEMENT,
        pPt1: *mut POINT,
    ) -> HELEMENT;
    fn TrackShip(Tracker: *mut ELEMENT, pfacing: *mut COUNT) -> SIZE;
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
    fn inHQSpace() -> BOOLEAN;
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
    pub const NUM_PRIMS: Self = Self(7);
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
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct INTERSECT_CONTROL {
    pub last_time_val: TIME_VALUE,
    pub EndPoint: POINT,
    pub IntersectStamp: STAMP,
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
    pub image: C2Rust_Unnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed {
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
    pub c2rust_unnamed: C2Rust_Unnamed_4,
    pub c2rust_unnamed_0: C2Rust_Unnamed_3,
    pub c2rust_unnamed_1: C2Rust_Unnamed_2,
    pub c2rust_unnamed_2: C2Rust_Unnamed_1,
    pub c2rust_unnamed_3: C2Rust_Unnamed_0,
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
pub union C2Rust_Unnamed_0 {
    pub thrust_wait: BYTE,
    pub blast_offset: BYTE,
    pub next_turn: BYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_1 {
    pub turn_wait: BYTE,
    pub sys_loc: BYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_2 {
    pub mass_points: BYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_3 {
    pub crew_level: COUNT,
    pub hit_points: COUNT,
    pub facing: COUNT,
    pub cycle: COUNT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_4 {
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
    pub c2rust_padding: [u8; 12],
    pub fleet: FLEET_STUFF,
    pub c2rust_padding_0: [u8; 10],
    pub characteristics: CHARACTERISTIC_STUFF,
    pub c2rust_padding_1: [u8; 2],
    pub ship_data: DATA_STUFF,
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_5(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_5 {
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
    pub const KEY_MENU_ANY: Self = Self(23);
    pub const NUM_MENU_KEYS: Self = Self(24);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _controller_input_state {
    pub key: [[::core::ffi::c_int; 7]; 6],
    pub menu: [::core::ffi::c_int; 24],
}
pub type CONTROLLER_INPUT_STATE = _controller_input_state;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_6(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_6 {
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GLOBDATA {
    pub SIS_state: SIS_STATE,
    pub Game_state: GAME_STATE,
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
pub struct SIS_DATA {
    pub num_trackers: COUNT,
    pub num_blasters: COUNT,
    pub MissileBlock: [MISSILE_BLOCK; 6],
}
pub type CustomShipData_t = SIS_DATA;
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_8(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_8 {
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
pub struct C2Rust_Unnamed_9(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_9 {
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
unsafe extern "C" fn logxToUniverse(mut lx: SDWORD) -> COORD {
    return ((lx
        * ((9999 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
            >> 4 as ::core::ffi::c_int) * 10 as ::core::ffi::c_int)
        + ((((ScreenWidth - 64 as ::core::ffi::c_int
            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
            << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
            >> 4 as ::core::ffi::c_int) * 195 as ::core::ffi::c_int
            >> 1 as ::core::ffi::c_int))
        / ((((ScreenWidth - 64 as ::core::ffi::c_int
            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
            << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
            >> 4 as ::core::ffi::c_int) * 195 as ::core::ffi::c_int)) as COORD;
}
#[inline]
unsafe extern "C" fn logyToUniverse(mut ly: SDWORD) -> COORD {
    return (9999 as ::core::ffi::c_int
        - (ly
            * (9999 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                >> 4 as ::core::ffi::c_int)
            + ((((ScreenHeight - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                >> 4 as ::core::ffi::c_int) * 25 as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int))
            / ((((ScreenHeight - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                >> 4 as ::core::ffi::c_int) * 25 as ::core::ffi::c_int)) as COORD;
}
static mut sis_desc: RACE_DESC = unsafe {
    race_desc {
        c2rust_padding: [0; 12],
        c2rust_padding_0: [0; 10],
        c2rust_padding_1: [0; 2],
        ship_info: SHIP_INFO {
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
    let mut udx: SIZE = 0 as SIZE;
    let mut udy: SIZE = 0 as SIZE;
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
    's_215: {
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
                universe.x = logxToUniverse(GlobData.SIS_state.log_x);
                universe.y = logyToUniverse(GlobData.SIS_state.log_y);
                udx = (GlobData.Game_state.autopilot.x as ::core::ffi::c_int
                    - universe.x as ::core::ffi::c_int) as SIZE;
                udy = -(GlobData.Game_state.autopilot.y as ::core::ffi::c_int
                    - universe.y as ::core::ffi::c_int) as SIZE;
                dx = udx;
                if (dx as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                    dx = -(dx as ::core::ffi::c_int) as SIZE;
                }
                dy = udy;
                if (dy as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                    dy = -(dy as ::core::ffi::c_int) as SIZE;
                }
                if !(dx as ::core::ffi::c_int <= 1 as ::core::ffi::c_int
                    && dy as ::core::ffi::c_int <= 1 as ::core::ffi::c_int)
                {
                    facing = (ARCTAN(udx, udy) as ::core::ffi::c_int
                        + ((1 as ::core::ffi::c_int)
                            << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int)
                        >> 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                        & ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                            - 1 as ::core::ffi::c_int) as COUNT as SIZE;
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
                                - 1 as ::core::ffi::c_int) as COUNT as ::core::ffi::c_int
                            > ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                                >> 1 as ::core::ffi::c_int)
                                + ((1 as ::core::ffi::c_int)
                                    << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                                        - 1 as ::core::ffi::c_int)
                                >> 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
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
                                - 1 as ::core::ffi::c_int) as COUNT as ::core::ffi::c_int
                            >= ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                                >> 1 as ::core::ffi::c_int)
                                + ((1 as ::core::ffi::c_int)
                                    << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                                        - 1 as ::core::ffi::c_int)
                                >> 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                        {
                            facing = (facing as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int
                                & ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                                    - 1 as ::core::ffi::c_int) as COUNT as SIZE;
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
                                    - 1 as ::core::ffi::c_int) as COUNT as SIZE;
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
                        || getGameState(
                            &raw mut GlobData.Game_state.GameState as *mut BYTE,
                            C2Rust_Unnamed_8::ARILOU_SPACE_SIDE.0 as ::core::ffi::c_int,
                            C2Rust_Unnamed_8::END_ARILOU_SPACE_SIDE.0
                                as ::core::ffi::c_int,
                        ) as ::core::ffi::c_int > 1 as ::core::ffi::c_int)
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
                        udx = ((dx as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                            as SIZE;
                        udy = ((dy as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                            as SIZE;
                    }
                    break 's_215;
                }
            }
        }
        GlobData.Game_state.autopilot.y = !(0 as ::core::ffi::c_int) as COORD;
        GlobData.Game_state.autopilot.x = GlobData.Game_state.autopilot.y;
        if (*StarShipPtr).cur_status_flags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int == 0
            || GlobData.SIS_state.FuelOnBoard == 0 as DWORD
                && getGameState(
                    &raw mut GlobData.Game_state.GameState as *mut BYTE,
                    C2Rust_Unnamed_8::ARILOU_SPACE_SIDE.0 as ::core::ffi::c_int,
                    C2Rust_Unnamed_8::END_ARILOU_SPACE_SIDE.0 as ::core::ffi::c_int,
                ) as ::core::ffi::c_int <= 1 as ::core::ffi::c_int
        {
            AccelerateDirection = -1 as SIZE;
            GetCurrentVelocityComponents(
                &raw mut (*ElementPtr).velocity,
                &raw mut dx,
                &raw mut dy,
            );
            udx = ((dx as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as SIZE;
            udy = ((dy as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as SIZE;
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
            / dist as ::core::ffi::c_long) as SIZE;
        dy = (udy as ::core::ffi::c_long * speed as ::core::ffi::c_long
            / dist as ::core::ffi::c_long) as SIZE;
        SetVelocityComponents(&raw mut (*ElementPtr).velocity, dx, dy);
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
            .menu[C2Rust_Unnamed_5::KEY_MENU_CANCEL.0 as ::core::ffi::c_int as usize]
            != 0)
        && (*StarShipPtr).special_counter as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
    {
        HyperspaceMenu();
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
                let mut delta_x: SIZE = 0;
                let mut delta_y: SIZE = 0;
                delta_x = ((*ObjectPtr).next.location.x as ::core::ffi::c_int
                    - (*ShipPtr).next.location.x as ::core::ffi::c_int) as SIZE;
                delta_y = ((*ObjectPtr).next.location.y as ::core::ffi::c_int
                    - (*ShipPtr).next.location.y as ::core::ffi::c_int) as SIZE;
                if (delta_x as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                    delta_x = -(delta_x as ::core::ffi::c_int) as SIZE;
                }
                if (delta_y as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                    delta_y = -(delta_y as ::core::ffi::c_int) as SIZE;
                }
                delta_x = (delta_x as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
                    as SIZE;
                delta_y = (delta_y as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
                    as SIZE;
                if delta_x as UWORD as ::core::ffi::c_int
                    <= 100 as ::core::ffi::c_int as UWORD as ::core::ffi::c_int
                    && delta_y as UWORD as ::core::ffi::c_int
                        <= 100 as ::core::ffi::c_int as UWORD as ::core::ffi::c_int
                    && delta_x as UWORD as ::core::ffi::c_int
                        * delta_x as UWORD as ::core::ffi::c_int
                        + delta_y as UWORD as ::core::ffi::c_int
                            * delta_y as UWORD as ::core::ffi::c_int
                        <= 100 as ::core::ffi::c_int as UWORD as ::core::ffi::c_int
                            * 100 as ::core::ffi::c_int as UWORD as ::core::ffi::c_int
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
                        if DeltaEnergy(
                                ShipPtr,
                                -(((*(*StarShipPtr).RaceDescPtr)
                                    .characteristics
                                    .special_energy_cost as ::core::ffi::c_int)
                                    << 2 as ::core::ffi::c_int) as SIZE,
                            )
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
        spawn_point_defense(ElementPtr);
    }
}
unsafe extern "C" fn blaster_collision(
    mut ElementPtr0: *mut ELEMENT,
    mut pPt0: *mut POINT,
    mut ElementPtr1: *mut ELEMENT,
    mut pPt1: *mut POINT,
) {
    let mut hBlastElement: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
    hBlastElement = weapon_collision(ElementPtr0, pPt0, ElementPtr1, pPt1);
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
                    - 1 as ::core::ffi::c_int) as COUNT;
            if TrackShip(ElementPtr, &raw mut facing) as ::core::ffi::c_int
                > 0 as ::core::ffi::c_int
            {
                SetVelocityVector(
                    &raw mut (*ElementPtr).velocity,
                    ((24 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as SIZE,
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
                - 1 as ::core::ffi::c_int) as COUNT;
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
        .offset(C2Rust_Unnamed_6::ENEMY_WEAPON_INDEX.0 as ::core::ffi::c_int as isize);
    if !(*lpEvalDesc).ObjectPtr.is_null() {
        if (*(*StarShipPtr).RaceDescPtr).characteristics.special_energy_cost != 0 {
            if (*StarShipPtr).special_counter as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
                && (!(*lpEvalDesc).ObjectPtr.is_null()
                    && (*lpEvalDesc).which_turn as ::core::ffi::c_int
                        <= 2 as ::core::ffi::c_int
                    || !(*ObjectsOfConcern
                        .offset(
                            C2Rust_Unnamed_6::ENEMY_SHIP_INDEX.0 as ::core::ffi::c_int
                                as isize,
                        ))
                        .ObjectPtr
                        .is_null()
                        && (*ObjectsOfConcern
                            .offset(
                                C2Rust_Unnamed_6::ENEMY_SHIP_INDEX.0 as ::core::ffi::c_int
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
            as ::core::ffi::c_int) < 45 as ::core::ffi::c_int
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
                            C2Rust_Unnamed_6::ENEMY_SHIP_INDEX.0 as ::core::ffi::c_int
                                as isize,
                        ))
                        .which_turn as ::core::ffi::c_int > 16 as ::core::ffi::c_int)
        {
            (*lpEvalDesc).MoveState = MOVEMENT_STATE::PURSUE;
        }
    }
    ship_intelligence(ShipPtr, ObjectsOfConcern, ConcernCounter);
    lpEvalDesc = ObjectsOfConcern
        .offset(C2Rust_Unnamed_6::ENEMY_SHIP_INDEX.0 as ::core::ffi::c_int as isize);
    if (*SisData).num_trackers as ::core::ffi::c_int != 0
        && (*StarShipPtr).weapon_counter as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        && (*StarShipPtr).ship_input_state as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int == 0
        && !(*lpEvalDesc).ObjectPtr.is_null()
        && (*lpEvalDesc).which_turn as ::core::ffi::c_int <= 16 as ::core::ffi::c_int
    {
        let mut direction_facing: COUNT = 0;
        let mut delta_x: SIZE = 0;
        let mut delta_y: SIZE = 0;
        let mut fire_flags: UWORD = 0;
        let mut ship_flags: UWORD = 0;
        let mut facing: COUNT = 0;
        delta_x = ((*(*lpEvalDesc).ObjectPtr).current.location.x as ::core::ffi::c_int
            - (*ShipPtr).current.location.x as ::core::ffi::c_int) as SIZE;
        delta_y = ((*(*lpEvalDesc).ObjectPtr).current.location.y as ::core::ffi::c_int
            - (*ShipPtr).current.location.y as ::core::ffi::c_int) as SIZE;
        direction_facing = (ARCTAN(delta_x, delta_y) as ::core::ffi::c_int
            + ((1 as ::core::ffi::c_int)
                << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int)
            >> 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
            & ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                - 1 as ::core::ffi::c_int) as COUNT;
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
                        - 1 as ::core::ffi::c_int) as COUNT as ::core::ffi::c_int
                    <= ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                        >> 2 as ::core::ffi::c_int)
                        + ((1 as ::core::ffi::c_int)
                            << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int)
                        >> 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
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
        if i as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            i = (16 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as COUNT;
        }
        which_gun = *ModuleSlots
            .offset(
                (16 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                    - i as ::core::ffi::c_int) as isize,
            ) as COUNT;
        if !((which_gun as ::core::ffi::c_int)
            < C2Rust_Unnamed_7::GUN_WEAPON.0 as ::core::ffi::c_int
            || which_gun as ::core::ffi::c_int
                > C2Rust_Unnamed_7::CANNON_WEAPON.0 as ::core::ffi::c_int)
        {
            which_gun = (which_gun as ::core::ffi::c_int
                - (C2Rust_Unnamed_7::GUN_WEAPON.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int)) as COUNT;
            (*RaceDescPtr).characteristics.weapon_energy_cost = ((*RaceDescPtr)
                .characteristics
                .weapon_energy_cost as ::core::ffi::c_int
                + which_gun as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as BYTE;
            (*lpMB).flags = ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
                as ELEMENT_FLAGS;
            (*lpMB).blast_offs = 8 as SIZE;
            (*lpMB).speed = ((24 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                as SIZE;
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
                    (*lpMB).pixoffs = 28 as SIZE;
                    (*lpMB).face = 0 as COUNT;
                }
                1 => {
                    (*RaceDescPtr).ship_info.ship_flags = ((*RaceDescPtr)
                        .ship_info
                        .ship_flags as ::core::ffi::c_int
                        | (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as UWORD;
                    (*lpMB).pixoffs = 28 as SIZE;
                    (*lpMB).face = 1 as ::core::ffi::c_int as COUNT;
                    *lpMB.offset(1isize) = *lpMB.offset(0isize);
                    lpMB = lpMB.offset(1);
                    (*SisData).num_blasters = (*SisData).num_blasters.wrapping_add(1);
                    (*lpMB).face = (-1 as ::core::ffi::c_int
                        & ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                            - 1 as ::core::ffi::c_int) as COUNT;
                }
                2 => {
                    (*RaceDescPtr).ship_info.ship_flags = ((*RaceDescPtr)
                        .ship_info
                        .ship_flags as ::core::ffi::c_int
                        | ((1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int
                            | (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int))
                        as UWORD;
                    (*lpMB).pixoffs = 20 as SIZE;
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
                            - 1 as ::core::ffi::c_int) as COUNT;
                }
                15 => {
                    (*RaceDescPtr).ship_info.ship_flags = ((*RaceDescPtr)
                        .ship_info
                        .ship_flags as ::core::ffi::c_int
                        | (1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int) as UWORD;
                    (*lpMB).pixoffs = 28 as SIZE;
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
        + num_trackers as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as BYTE;
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
                    .max_thrust as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
                    as COUNT;
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
        c2rust_padding: [0; 12],
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
            },
            victory_ditty_rsc: ::core::ptr::null::<::core::ffi::c_char>(),
            ship_sounds_rsc: ::core::ptr::null::<::core::ffi::c_char>(),
            ship: [::core::ptr::null_mut::<FRAME_DESC>(); 3],
            weapon: [::core::ptr::null_mut::<FRAME_DESC>(); 3],
            special: [::core::ptr::null_mut::<FRAME_DESC>(); 3],
            victory_ditty: ::core::ptr::null_mut::<*mut TFB_SoundSample>(),
            ship_sounds: ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>(),
        },
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
            .max_thrust as ::core::ffi::c_int - 4 as ::core::ffi::c_int) as COUNT;
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
        if getGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_8::CHMMR_BOMB_STATE.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_8::END_CHMMR_BOMB_STATE.0 as ::core::ffi::c_int,
        ) as ::core::ffi::c_int == 3 as ::core::ffi::c_int
        {
            setGameState(
                &raw mut GlobData.Game_state.GameState as *mut BYTE,
                C2Rust_Unnamed_8::BOMB_CARRIER.0 as ::core::ffi::c_int,
                C2Rust_Unnamed_8::END_BOMB_CARRIER.0 as ::core::ffi::c_int,
                1 as BYTE,
            );
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
        == C2Rust_Unnamed_9::SUPER_MELEE.0 as ::core::ffi::c_int
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
