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

// Rust translation of src/uqm/ships/vux/vux.c.
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
    fn SetAbsFrameIndex(Frame: FRAME, FrameIndex: COUNT) -> FRAME;
    fn SetEquFrameIndex(DstFrame: FRAME, SrcFrame: FRAME) -> FRAME;
    fn IncFrameIndex(Frame: FRAME) -> FRAME;
    fn SetAbsStringTableIndex(String: STRING, StringTableIndex: COUNT) -> STRING;
    static mut resolutionFactor: ::core::ffi::c_uint;
    static mut optGodModes: ::core::ffi::c_int;
    static mut optWindowType: DWORD;
    static mut CanvasWidth: ::core::ffi::c_int;
    static mut CanvasHeight: ::core::ffi::c_int;
    static mut sinetab: [SDWORD; 0];
    fn ARCTAN(delta_x: SDWORD, delta_y: SDWORD) -> COUNT;
    fn SetVelocityVector(
        velocityptr: *mut VELOCITY_DESC,
        magnitude: SDWORD,
        facing: COUNT,
    );
    static mut disp_q: QUEUE;
    fn TimeSpaceMatterConflict(ElementPtr: *mut ELEMENT) -> BOOLEAN;
    fn CalculateGravity(ElementPtr: *mut ELEMENT) -> BOOLEAN;
    static mut GlobData: GLOBDATA;
    fn TFB_Random() -> DWORD;
    static mut PlayerControl: [BYTE; 0];
    fn ProcessSound(Sound: SOUND, PositionalObject: *mut ELEMENT);
    fn DeltaEnergy(ElementPtr: *mut ELEMENT, energy_delta: SIZE) -> BOOLEAN;
    fn initialize_laser(pLaserBlock: *mut LASER_BLOCK) -> HELEMENT;
    fn initialize_missile(pMissileBlock: *mut MISSILE_BLOCK) -> HELEMENT;
    fn TrackShip(Tracker: *mut ELEMENT, pfacing: *mut COUNT) -> SIZE;
    fn ModifySilhouette(
        ElementPtr: *mut ELEMENT,
        modify_stamp: *mut STAMP,
        modify_flags: BYTE,
    ) -> FRAME;
    fn ship_intelligence(
        ShipPtr: *mut ELEMENT,
        ObjectsOfConcern: *mut EVALUATE_DESC,
        ConcernCounter: COUNT,
    );
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
static mut vux_desc: RACE_DESC = unsafe {
    race_desc {
        c2rust_padding: [0; 8],
        c2rust_padding_0: [0; 10],
        c2rust_padding_1: [0; 2],
        c2rust_padding_2: [0; 8],
        ship_info: SHIP_INFO {
            idStr: b"intruder\0".as_ptr() as *const ::core::ffi::c_char,
            ship_flags: ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as UWORD,
            ship_cost: 12 as BYTE,
            crew_level: 20 as COUNT,
            max_crew: 20 as COUNT,
            energy_level: 40 as BYTE,
            max_energy: 40 as BYTE,
            race_strings_rsc: b"ship.vux.text\0".as_ptr() as RESOURCE,
            icons_rsc: b"ship.vux.icons\0".as_ptr() as RESOURCE,
            melee_icon_rsc: b"ship.vux.meleeicons\0".as_ptr() as RESOURCE,
            race_strings: ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>(),
            icons: ::core::ptr::null_mut::<FRAME_DESC>(),
            melee_icon: ::core::ptr::null_mut::<FRAME_DESC>(),
        },
        fleet: FLEET_STUFF {
            strength: (900 as ::core::ffi::c_int / 11 as ::core::ffi::c_int
                * 2 as ::core::ffi::c_int) as COUNT,
            known_loc: point {
                x: 4412 as COORD,
                y: 1558 as COORD,
            },
        },
        characteristics: CHARACTERISTIC_STUFF {
            max_thrust: 21 as COUNT,
            thrust_increment: 7 as COUNT,
            energy_regeneration: 1 as BYTE,
            weapon_energy_cost: 1 as BYTE,
            special_energy_cost: 2 as BYTE,
            energy_wait: 8 as BYTE,
            turn_wait: 6 as BYTE,
            thrust_wait: 4 as BYTE,
            weapon_wait: 0 as BYTE,
            special_wait: 7 as BYTE,
            ship_mass: 6 as BYTE,
        },
        ship_data: DATA_STUFF {
            ship_rsc: [
                b"ship.vux.graphics.intruder.large\0".as_ptr() as RESOURCE,
                b"ship.vux.graphics.intruder.medium\0".as_ptr() as RESOURCE,
                b"ship.vux.graphics.intruder.small\0".as_ptr() as RESOURCE,
            ],
            weapon_rsc: [
                b"ship.vux.graphics.slime\0".as_ptr() as RESOURCE,
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
            ],
            special_rsc: [
                b"ship.vux.graphics.limpets.large\0".as_ptr() as RESOURCE,
                b"ship.vux.graphics.limpets.medium\0".as_ptr() as RESOURCE,
                b"ship.vux.graphics.limpets.small\0".as_ptr() as RESOURCE,
            ],
            captain_control: captain_stuff {
                captain_rsc: b"ship.vux.graphics.captain\0".as_ptr() as RESOURCE,
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
            victory_ditty_rsc: b"ship.vux.ditty\0".as_ptr() as RESOURCE,
            ship_sounds_rsc: b"ship.vux.sounds\0".as_ptr() as RESOURCE,
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
            WeaponRange: ((50 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                as COUNT,
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
unsafe extern "C" fn limpet_preprocess(mut ElementPtr: *mut ELEMENT) {
    let mut facing: COUNT = 0;
    let mut orig_facing: COUNT = 0;
    let mut delta_facing: SIZE = 0;
    orig_facing = ((*ElementPtr).velocity.TravelAngle as ::core::ffi::c_int
        + ((1 as ::core::ffi::c_int)
            << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int)
        >> 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
        & ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
            - 1 as ::core::ffi::c_int) as DWORD as COUNT;
    facing = orig_facing;
    delta_facing = TrackShip(ElementPtr, &raw mut facing);
    if delta_facing as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
        facing = (orig_facing as ::core::ffi::c_int + delta_facing as ::core::ffi::c_int)
            as COUNT;
        SetVelocityVector(
            &raw mut (*ElementPtr).velocity,
            (25 as SDWORD) << resolutionFactor,
            facing,
        );
    }
    (*ElementPtr).next.image.frame = IncFrameIndex((*ElementPtr).next.image.frame);
    (*ElementPtr).state_flags = ((*ElementPtr).state_flags as ::core::ffi::c_int
        | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as ELEMENT_FLAGS;
}
unsafe extern "C" fn limpet_collision(
    mut ElementPtr0: *mut ELEMENT,
    mut pPt0: *mut POINT,
    mut ElementPtr1: *mut ELEMENT,
    mut pPt1: *mut POINT,
) {
    if (*ElementPtr1).state_flags as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int != 0
    {
        let mut s: STAMP = STAMP {
            origin: POINT { x: 0, y: 0 },
            frame: ::core::ptr::null_mut::<FRAME_DESC>(),
        };
        let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
        let mut RDPtr: *mut RACE_DESC = ::core::ptr::null_mut::<RACE_DESC>();
        StarShipPtr = (*ElementPtr1).pParent as *mut STARSHIP;
        RDPtr = (*StarShipPtr).RaceDescPtr;
        if !(antiCheat(
                ElementPtr1,
                BOOLEAN::FALSE,
                C2Rust_Unnamed::OPTVAL_INF_HEALTH.0 as ::core::ffi::c_int,
            )
            .0 != 0
            || antiCheat(
                    ElementPtr1,
                    BOOLEAN::FALSE,
                    C2Rust_Unnamed::OPTVAL_FULL_GOD.0 as ::core::ffi::c_int,
                )
                .0 != 0)
        {
            (*RDPtr).characteristics.turn_wait = (*RDPtr)
                .characteristics
                .turn_wait
                .wrapping_add(1);
            if (*RDPtr).characteristics.turn_wait as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
            {
                (*RDPtr).characteristics.turn_wait = (*RDPtr)
                    .characteristics
                    .turn_wait
                    .wrapping_sub(1);
            }
            (*RDPtr).characteristics.thrust_wait = (*RDPtr)
                .characteristics
                .thrust_wait
                .wrapping_add(1);
            if (*RDPtr).characteristics.thrust_wait as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
            {
                (*RDPtr).characteristics.thrust_wait = (*RDPtr)
                    .characteristics
                    .thrust_wait
                    .wrapping_sub(1);
            }
            if (*RDPtr).characteristics.thrust_increment as ::core::ffi::c_int
                <= (1 as ::core::ffi::c_int) << resolutionFactor
                    << 2 as ::core::ffi::c_int
            {
                (*RDPtr).characteristics.max_thrust = (((*RDPtr)
                    .characteristics
                    .thrust_increment as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
                    as COUNT;
            } else {
                let mut num_thrusts: COUNT = 0;
                num_thrusts = ((*RDPtr).characteristics.max_thrust as ::core::ffi::c_int
                    / (*RDPtr).characteristics.thrust_increment as ::core::ffi::c_int)
                    as COUNT;
                (*RDPtr).characteristics.thrust_increment = ((*RDPtr)
                    .characteristics
                    .thrust_increment as ::core::ffi::c_int
                    - ((1 as ::core::ffi::c_int) << resolutionFactor)) as COUNT;
                (*RDPtr).characteristics.max_thrust = ((*RDPtr)
                    .characteristics
                    .thrust_increment as ::core::ffi::c_int
                    * num_thrusts as ::core::ffi::c_int) as COUNT;
            }
            (*RDPtr).cyborg_control.ManeuverabilityIndex = 0 as COUNT;
            StarShipPtr = (*ElementPtr0).pParent as *mut STARSHIP;
            ProcessSound(
                SetAbsStringTableIndex(
                    (*(*StarShipPtr).RaceDescPtr).ship_data.ship_sounds,
                    2 as COUNT,
                ),
                ElementPtr1,
            );
            s.frame = SetAbsFrameIndex(
                (*(*StarShipPtr).RaceDescPtr).ship_data.weapon[0usize],
                TFB_Random() as COUNT,
            );
            uqm_lua_checked!(ModifySilhouette(
                ElementPtr1,
                &raw mut s,
                ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as BYTE,
            ), ());
        } else {
            StarShipPtr = (*ElementPtr0).pParent as *mut STARSHIP;
            ProcessSound(
                SetAbsStringTableIndex(
                    (*(*StarShipPtr).RaceDescPtr).ship_data.ship_sounds,
                    2 as COUNT,
                ),
                ElementPtr1,
            );
        }
    }
    (*ElementPtr0).c2rust_unnamed_0.hit_points = 0 as COUNT;
    (*ElementPtr0).c2rust_unnamed.life_span = 0 as COUNT;
    (*ElementPtr0).state_flags = ((*ElementPtr0).state_flags as ::core::ffi::c_int
        | ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)) as ELEMENT_FLAGS;
}
unsafe extern "C" fn spawn_limpets(mut ElementPtr: *mut ELEMENT) {
    let mut Limpet: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
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
    StarShipPtr = (*ElementPtr).pParent as *mut STARSHIP;
    MissileBlock.farray = &raw mut (*(*StarShipPtr).RaceDescPtr).ship_data.special
        as *mut FRAME;
    MissileBlock.face = ((*StarShipPtr).ShipFacing as ::core::ffi::c_int
        + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
            >> 1 as ::core::ffi::c_int)) as COUNT;
    MissileBlock.index = 0 as COUNT;
    MissileBlock.sender = (*ElementPtr).playerNr;
    MissileBlock.flags = ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
        as ELEMENT_FLAGS;
    MissileBlock.pixoffs = ((8 as ::core::ffi::c_int) << resolutionFactor) as SIZE;
    MissileBlock.speed = ((25 as ::core::ffi::c_int) << resolutionFactor) as SIZE;
    MissileBlock.hit_points = 1 as SIZE;
    MissileBlock.damage = 0 as SIZE;
    MissileBlock.life = 80 as COUNT;
    MissileBlock.preprocess_func = Some(
        limpet_preprocess as unsafe extern "C" fn(*mut ELEMENT) -> (),
    ) as Option<unsafe extern "C" fn(*mut ELEMENT) -> ()>;
    MissileBlock.blast_offs = 0 as SIZE;
    MissileBlock.cx = (*ElementPtr).next.location.x;
    MissileBlock.cy = (*ElementPtr).next.location.y;
    Limpet = initialize_missile(&raw mut MissileBlock);
    if !Limpet.is_null() {
        let mut LimpetPtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
        LimpetPtr = LockLink(&raw mut disp_q, Limpet) as *mut ELEMENT;
        (*LimpetPtr).collision_func = Some(
            limpet_collision
                as unsafe extern "C" fn(
                    *mut ELEMENT,
                    *mut POINT,
                    *mut ELEMENT,
                    *mut POINT,
                ) -> (),
        ) as Option<ElementCollisionFunc>;
        (*LimpetPtr).pParent = StarShipPtr as *mut ::core::ffi::c_void;
        UnlockLink(&raw mut disp_q, Limpet);
        PutQueue(&raw mut disp_q, Limpet);
    }
}
unsafe extern "C" fn initialize_horrific_laser(
    mut ShipPtr: *mut ELEMENT,
    mut LaserArray: *mut HELEMENT,
) -> COUNT {
    let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
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
    StarShipPtr = (*ShipPtr).pParent as *mut STARSHIP;
    LaserBlock.face = (*StarShipPtr).ShipFacing;
    LaserBlock.cx = (*ShipPtr).next.location.x;
    LaserBlock.cy = (*ShipPtr).next.location.y;
    LaserBlock.ex = (*(&raw mut sinetab as *mut SDWORD)
        .offset(
            (((LaserBlock.face as ::core::ffi::c_int)
                << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int)
                + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                    >> 2 as ::core::ffi::c_int)
                & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int) as DWORD as isize,
        ) as ::core::ffi::c_long
        * (((150 as ::core::ffi::c_int) << resolutionFactor)
            + if if resolutionFactor != 2 as ::core::ffi::c_uint {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } == 0
            {
                12 as ::core::ffi::c_int
            } else {
                38 as ::core::ffi::c_int
            } << 2 as ::core::ffi::c_int) as ::core::ffi::c_long
        >> 14 as ::core::ffi::c_int) as SDWORD as COORD;
    LaserBlock.ey = (*(&raw mut sinetab as *mut SDWORD)
        .offset(
            ((LaserBlock.face as ::core::ffi::c_int)
                << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int) as DWORD as isize,
        ) as ::core::ffi::c_long
        * (((150 as ::core::ffi::c_int) << resolutionFactor)
            + if if resolutionFactor != 2 as ::core::ffi::c_uint {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } == 0
            {
                12 as ::core::ffi::c_int
            } else {
                38 as ::core::ffi::c_int
            } << 2 as ::core::ffi::c_int) as ::core::ffi::c_long
        >> 14 as ::core::ffi::c_int) as SDWORD as COORD;
    LaserBlock.sender = (*ShipPtr).playerNr;
    LaserBlock.flags = ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
        as ELEMENT_FLAGS;
    LaserBlock.pixoffs = (if if resolutionFactor != 2 as ::core::ffi::c_uint {
        BOOLEAN::FALSE.0 as ::core::ffi::c_int
    } else {
        BOOLEAN::TRUE.0 as ::core::ffi::c_int
    } == 0
    {
        12 as ::core::ffi::c_int
    } else {
        38 as ::core::ffi::c_int
    }) as SIZE;
    LaserBlock.color = colorFromRgb15(0xa as BYTE, 0x1f as BYTE, 0xa as BYTE);
    *LaserArray.offset(0isize) = initialize_laser(&raw mut LaserBlock);
    return 1 as COUNT;
}
unsafe extern "C" fn vux_intelligence(
    mut ShipPtr: *mut ELEMENT,
    mut ObjectsOfConcern: *mut EVALUATE_DESC,
    mut ConcernCounter: COUNT,
) {
    let mut lpEvalDesc: *mut EVALUATE_DESC = ::core::ptr::null_mut::<EVALUATE_DESC>();
    let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
    lpEvalDesc = ObjectsOfConcern
        .offset(C2Rust_Unnamed_7::ENEMY_SHIP_INDEX.0 as ::core::ffi::c_int as isize);
    (*lpEvalDesc).MoveState = MOVEMENT_STATE::PURSUE;
    if !(*ObjectsOfConcern
        .offset(C2Rust_Unnamed_7::ENEMY_WEAPON_INDEX.0 as ::core::ffi::c_int as isize))
        .ObjectPtr
        .is_null()
        && (*ObjectsOfConcern
            .offset(
                C2Rust_Unnamed_7::ENEMY_WEAPON_INDEX.0 as ::core::ffi::c_int as isize,
            ))
            .MoveState
            .0 == MOVEMENT_STATE::ENTICE.0
    {
        if (*(*ObjectsOfConcern
            .offset(
                C2Rust_Unnamed_7::ENEMY_WEAPON_INDEX.0 as ::core::ffi::c_int as isize,
            ))
            .ObjectPtr)
            .state_flags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int != 0
            && (*(*ObjectsOfConcern
                .offset(
                    C2Rust_Unnamed_7::ENEMY_WEAPON_INDEX.0 as ::core::ffi::c_int as isize,
                ))
                .ObjectPtr)
                .state_flags as ::core::ffi::c_int
                & (1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int == 0
        {
            (*ObjectsOfConcern
                .offset(
                    C2Rust_Unnamed_7::ENEMY_WEAPON_INDEX.0 as ::core::ffi::c_int as isize,
                ))
                .MoveState = MOVEMENT_STATE::AVOID;
        } else {
            (*ObjectsOfConcern
                .offset(
                    C2Rust_Unnamed_7::ENEMY_WEAPON_INDEX.0 as ::core::ffi::c_int as isize,
                ))
                .MoveState = MOVEMENT_STATE::PURSUE;
        }
    }
    uqm_lua_checked!(ship_intelligence(ShipPtr, ObjectsOfConcern, ConcernCounter), ());
    StarShipPtr = (*ShipPtr).pParent as *mut STARSHIP;
    if (*StarShipPtr).special_counter as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        && !(*lpEvalDesc).ObjectPtr.is_null()
        && (*lpEvalDesc).which_turn as ::core::ffi::c_int <= 12 as ::core::ffi::c_int
        && (*StarShipPtr).ship_input_state as ::core::ffi::c_int
            & ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) != 0
        && (*(*StarShipPtr).RaceDescPtr).ship_info.energy_level as ::core::ffi::c_int
            >= ((*(*StarShipPtr).RaceDescPtr).ship_info.max_energy as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int) as BYTE as ::core::ffi::c_int
    {
        (*StarShipPtr).ship_input_state = ((*StarShipPtr).ship_input_state
            as ::core::ffi::c_int | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
            as BYTE;
    } else {
        (*StarShipPtr).ship_input_state = ((*StarShipPtr).ship_input_state
            as ::core::ffi::c_int
            & !((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)) as BYTE;
    };
}
unsafe extern "C" fn vux_postprocess(mut ElementPtr: *mut ELEMENT) {
    let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
    StarShipPtr = (*ElementPtr).pParent as *mut STARSHIP;
    if (*StarShipPtr).cur_status_flags as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int != 0
        && (*StarShipPtr).special_counter as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int && uqm_lua_checked!(DeltaEnergy(ElementPtr, -2 as SIZE), ()).0 != 0
    {
        ProcessSound(
            SetAbsStringTableIndex(
                (*(*StarShipPtr).RaceDescPtr).ship_data.ship_sounds,
                1 as COUNT,
            ),
            ElementPtr,
        );
        spawn_limpets(ElementPtr);
        (*StarShipPtr).special_counter = (*(*StarShipPtr).RaceDescPtr)
            .characteristics
            .special_wait;
    }
}
unsafe extern "C" fn vux_preprocess(mut ElementPtr: *mut ELEMENT) {
    if (*ElementPtr).state_flags as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int != 0
    {
        let mut facing: COUNT = 0;
        let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
        StarShipPtr = (*ElementPtr).pParent as *mut STARSHIP;
        facing = (*StarShipPtr).ShipFacing;
        if (GlobData.Game_state.CurrentActivity as BYTE as ::core::ffi::c_int
            != C2Rust_Unnamed_6::IN_ENCOUNTER.0 as ::core::ffi::c_int
            || if if GlobData.SIS_state.Difficulty as ::core::ffi::c_int != 0 {
                GlobData.SIS_state.Difficulty as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } == 2 as ::core::ffi::c_int
            {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } != 0)
            && TrackShip(ElementPtr, &raw mut facing) as ::core::ffi::c_int
                >= 0 as ::core::ffi::c_int
        {
            let mut OtherShipPtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
            let mut SA_MATRA_EXTRA_DIST: SIZE = 0 as SIZE;
            OtherShipPtr = LockLink(&raw mut disp_q, (*ElementPtr).hTarget)
                as *mut ELEMENT;
            if GlobData.Game_state.CurrentActivity as BYTE as ::core::ffi::c_int
                == C2Rust_Unnamed_6::IN_LAST_BATTLE.0 as ::core::ffi::c_int
            {
                SA_MATRA_EXTRA_DIST = (SA_MATRA_EXTRA_DIST as ::core::ffi::c_int
                    + ((1000 as ::core::ffi::c_int) << resolutionFactor)) as SIZE;
            }
            loop {
                let mut dx: SDWORD = 0;
                let mut dy: SDWORD = 0;
                let mut temp_x: SDWORD = (*OtherShipPtr).current.location.x as SDWORD
                    - ((((150 as SDWORD) << resolutionFactor)
                        + if if resolutionFactor != 2 as ::core::ffi::c_uint {
                            BOOLEAN::FALSE.0 as ::core::ffi::c_int
                        } else {
                            BOOLEAN::TRUE.0 as ::core::ffi::c_int
                        } == 0
                        {
                            12 as SDWORD
                        } else {
                            38 as SDWORD
                        } + ((46 as SDWORD) << resolutionFactor)
                        << 1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
                        >> 1 as ::core::ffi::c_int)
                    + TFB_Random() as COUNT as SDWORD
                        % ((((150 as SDWORD) << resolutionFactor)
                            + if if resolutionFactor != 2 as ::core::ffi::c_uint {
                                BOOLEAN::FALSE.0 as ::core::ffi::c_int
                            } else {
                                BOOLEAN::TRUE.0 as ::core::ffi::c_int
                            } == 0
                            {
                                12 as SDWORD
                            } else {
                                38 as SDWORD
                            } + ((46 as SDWORD) << resolutionFactor)
                            << 1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int);
                let mut temp_y: SDWORD = (*OtherShipPtr).current.location.y as SDWORD
                    - ((((150 as SDWORD) << resolutionFactor)
                        + if if resolutionFactor != 2 as ::core::ffi::c_uint {
                            BOOLEAN::FALSE.0 as ::core::ffi::c_int
                        } else {
                            BOOLEAN::TRUE.0 as ::core::ffi::c_int
                        } == 0
                        {
                            12 as SDWORD
                        } else {
                            38 as SDWORD
                        } + ((46 as SDWORD) << resolutionFactor)
                        << 1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
                        >> 1 as ::core::ffi::c_int)
                    + TFB_Random() as COUNT as SDWORD
                        % ((((150 as SDWORD) << resolutionFactor)
                            + if if resolutionFactor != 2 as ::core::ffi::c_uint {
                                BOOLEAN::FALSE.0 as ::core::ffi::c_int
                            } else {
                                BOOLEAN::TRUE.0 as ::core::ffi::c_int
                            } == 0
                            {
                                12 as SDWORD
                            } else {
                                38 as SDWORD
                            } + ((46 as SDWORD) << resolutionFactor)
                            << 1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int);
                temp_x
                    += if temp_x > 0 as ::core::ffi::c_int {
                        SA_MATRA_EXTRA_DIST as ::core::ffi::c_int
                    } else {
                        -(SA_MATRA_EXTRA_DIST as ::core::ffi::c_int)
                    };
                temp_y
                    += if temp_y > 0 as ::core::ffi::c_int {
                        SA_MATRA_EXTRA_DIST as ::core::ffi::c_int
                    } else {
                        -(SA_MATRA_EXTRA_DIST as ::core::ffi::c_int)
                    };
                dx = (*OtherShipPtr).current.location.x as SDWORD - temp_x;
                dy = (*OtherShipPtr).current.location.y as SDWORD - temp_y;
                facing = (ARCTAN(dx, dy) as ::core::ffi::c_int
                    + ((1 as ::core::ffi::c_int)
                        << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int)
                    >> 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                    & ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int) as DWORD as COUNT;
                (*ElementPtr).current.image.frame = SetAbsFrameIndex(
                    (*ElementPtr).current.image.frame,
                    facing,
                );
                (*ElementPtr).current.location.x = (if temp_x
                    & !(((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
                {
                    (temp_x
                        & !(((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                            - 1 as ::core::ffi::c_int))
                        + (((CanvasWidth
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
                            } << resolutionFactor) * 2 as ::core::ffi::c_int)
                            << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                } else if temp_x
                    & !(((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int)
                    >= ((CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
                        - (if if optWindowType == 1 as DWORD {
                            BOOLEAN::TRUE.0 as ::core::ffi::c_int
                        } else {
                            BOOLEAN::FALSE.0 as ::core::ffi::c_int
                        } != 0
                        {
                            16 as ::core::ffi::c_int
                        } else {
                            0 as ::core::ffi::c_int
                        } << resolutionFactor) * 2 as ::core::ffi::c_int)
                        << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                {
                    (temp_x
                        & !(((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                            - 1 as ::core::ffi::c_int))
                        - (((CanvasWidth
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
                            } << resolutionFactor) * 2 as ::core::ffi::c_int)
                            << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                } else {
                    temp_x
                        & !(((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                            - 1 as ::core::ffi::c_int)
                }) as DWORD as COORD;
                (*ElementPtr).current.location.y = (if temp_y
                    & !(((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
                {
                    (temp_y
                        & !(((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                            - 1 as ::core::ffi::c_int))
                        + (((CanvasHeight
                            - (if if optWindowType == 1 as DWORD {
                                BOOLEAN::TRUE.0 as ::core::ffi::c_int
                            } else {
                                BOOLEAN::FALSE.0 as ::core::ffi::c_int
                            } != 0
                            {
                                16 as ::core::ffi::c_int
                            } else {
                                0 as ::core::ffi::c_int
                            } << resolutionFactor) * 2 as ::core::ffi::c_int)
                            << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                } else if temp_y
                    & !(((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int)
                    >= ((CanvasHeight
                        - (if if optWindowType == 1 as DWORD {
                            BOOLEAN::TRUE.0 as ::core::ffi::c_int
                        } else {
                            BOOLEAN::FALSE.0 as ::core::ffi::c_int
                        } != 0
                        {
                            16 as ::core::ffi::c_int
                        } else {
                            0 as ::core::ffi::c_int
                        } << resolutionFactor) * 2 as ::core::ffi::c_int)
                        << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                {
                    (temp_y
                        & !(((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                            - 1 as ::core::ffi::c_int))
                        - (((CanvasHeight
                            - (if if optWindowType == 1 as DWORD {
                                BOOLEAN::TRUE.0 as ::core::ffi::c_int
                            } else {
                                BOOLEAN::FALSE.0 as ::core::ffi::c_int
                            } != 0
                            {
                                16 as ::core::ffi::c_int
                            } else {
                                0 as ::core::ffi::c_int
                            } << resolutionFactor) * 2 as ::core::ffi::c_int)
                            << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                } else {
                    temp_y
                        & !(((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                            - 1 as ::core::ffi::c_int)
                }) as DWORD as COORD;
                if !(CalculateGravity(ElementPtr).0 != 0
                    || uqm_lua_checked!(TimeSpaceMatterConflict(ElementPtr), ()).0 != 0)
                {
                    break;
                }
            }
            UnlockLink(&raw mut disp_q, (*ElementPtr).hTarget);
            (*ElementPtr).hTarget = ::core::ptr::null_mut::<::core::ffi::c_void>();
            (*ElementPtr).next = (*ElementPtr).current;
            (*ElementPtr).IntersectControl.IntersectStamp.origin.x = ((*ElementPtr)
                .current
                .location
                .x as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as COORD;
            (*ElementPtr).IntersectControl.IntersectStamp.origin.y = ((*ElementPtr)
                .current
                .location
                .y as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as COORD;
            (*ElementPtr).IntersectControl.EndPoint.x = ((*ElementPtr).next.location.x
                as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as COORD;
            (*ElementPtr).IntersectControl.EndPoint.y = ((*ElementPtr).next.location.y
                as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as COORD;
            (*ElementPtr).IntersectControl.IntersectStamp.frame = SetEquFrameIndex(
                *(*ElementPtr).next.image.farray.offset(0isize),
                (*ElementPtr).next.image.frame,
            );
            (*StarShipPtr).ShipFacing = facing;
        }
        (*(*StarShipPtr).RaceDescPtr).preprocess_func = None;
    }
}
#[no_mangle]
pub unsafe extern "C" fn init_vux() -> *mut RACE_DESC {
    let mut RaceDescPtr: *mut RACE_DESC = ::core::ptr::null_mut::<RACE_DESC>();
    if if resolutionFactor != 2 as ::core::ffi::c_uint {
        BOOLEAN::FALSE.0 as ::core::ffi::c_int
    } else {
        BOOLEAN::TRUE.0 as ::core::ffi::c_int
    } != 0
    {
        vux_desc.characteristics.max_thrust = ((21 as ::core::ffi::c_int)
            << resolutionFactor) as COUNT;
        vux_desc.characteristics.thrust_increment = ((7 as ::core::ffi::c_int)
            << resolutionFactor) as COUNT;
        vux_desc.cyborg_control.WeaponRange = ((200 as ::core::ffi::c_int)
            << 2 as ::core::ffi::c_int) as COUNT;
    } else {
        vux_desc.characteristics.max_thrust = 21 as COUNT;
        vux_desc.characteristics.thrust_increment = 7 as COUNT;
        vux_desc.cyborg_control.WeaponRange = ((50 as ::core::ffi::c_int)
            << 2 as ::core::ffi::c_int) as COUNT;
    }
    vux_desc.preprocess_func = Some(
        vux_preprocess as unsafe extern "C" fn(*mut ELEMENT) -> (),
    ) as Option<PREPROCESS_FUNC>;
    vux_desc.postprocess_func = Some(
        vux_postprocess as unsafe extern "C" fn(*mut ELEMENT) -> (),
    ) as Option<POSTPROCESS_FUNC>;
    vux_desc.init_weapon_func = Some(
        initialize_horrific_laser
            as unsafe extern "C" fn(*mut ELEMENT, *mut HELEMENT) -> COUNT,
    ) as Option<INIT_WEAPON_FUNC>;
    vux_desc.cyborg_control.intelligence_func = Some(
        vux_intelligence
            as unsafe extern "C" fn(*mut ELEMENT, *mut EVALUATE_DESC, COUNT) -> (),
    ) as Option<IntelligenceFunc>;
    RaceDescPtr = &raw mut vux_desc;
    return RaceDescPtr;
}
