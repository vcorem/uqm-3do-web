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
    pub type string_table_entry;
    pub type frame_desc;
    pub type tfb_soundsample;
    fn __assert_fail(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    );
    fn AllocLink(pq: *mut QUEUE) -> HLINK;
    fn FreeLink(pq: *mut QUEUE, hLink: HLINK);
    fn PutQueue(pq: *mut QUEUE, hLink: HLINK);
    fn InsertQueue(pq: *mut QUEUE, hLink: HLINK, hRefLink: HLINK);
    fn RemoveQueue(pq: *mut QUEUE, hLink: HLINK);
    fn CountLinks(pq: *mut QUEUE) -> COUNT;
    fn memset(
        _: *mut ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn DeltaSISGauges(
        crew_delta: SIZE,
        fuel_delta: SIZE,
        resunit_delta: ::core::ffi::c_int,
    );
    static mut GlobData: GLOBDATA;
    fn TFB_Random() -> DWORD;
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
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct INTERSECT_CONTROL {
    pub last_time_val: TIME_VALUE,
    pub EndPoint: POINT,
    pub IntersectStamp: STAMP,
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
pub type STATE = state;
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
pub struct C2Rust_Unnamed_5(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_5 {
    pub const DEAD_GUY: Self = Self(0);
    pub const GOOD_GUY: Self = Self(1);
    pub const BAD_GUY: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_6(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_6 {
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
                37 as ::core::ffi::c_int,
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
pub unsafe extern "C" fn AddEscortShips(mut race: COUNT, mut count: SIZE) -> COUNT {
    let mut hFleet: HFLEETINFO = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut which_window: BYTE = 0;
    let mut i: COUNT = 0;
    hFleet = GetStarShipFromIndex(&raw mut GlobData.Game_state.avail_race_q, race)
        as HFLEETINFO;
    if hFleet.is_null() {
        return 0 as COUNT;
    }
    (count as ::core::ffi::c_int > 0 as ::core::ffi::c_int
        || {
            __assert_fail(
                b"count > 0\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/uqm/build.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                112 as ::core::ffi::c_int,
                b"AddEscortShips\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    which_window = 0 as BYTE;
    i = 0 as COUNT;
    while (i as ::core::ffi::c_int) < count as COUNT as ::core::ffi::c_int {
        let mut hStarShip: HSHIPFRAG = ::core::ptr::null_mut::<::core::ffi::c_void>();
        let mut hOldShip: HSHIPFRAG = ::core::ptr::null_mut::<::core::ffi::c_void>();
        let mut StarShipPtr: *mut SHIP_FRAGMENT = ::core::ptr::null_mut::<
            SHIP_FRAGMENT,
        >();
        hStarShip = CloneShipFragment(
            race,
            &raw mut GlobData.Game_state.built_ship_q,
            0 as COUNT,
        );
        if hStarShip.is_null() {
            break;
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
        StarShipPtr = LockShipFrag(&raw mut GlobData.Game_state.built_ship_q, hStarShip);
        (*StarShipPtr).index = (which_window as ::core::ffi::c_int
            - 1 as ::core::ffi::c_int) as BYTE;
        UnlockLink(&raw mut GlobData.Game_state.built_ship_q, hStarShip);
        InsertQueue(&raw mut GlobData.Game_state.built_ship_q, hStarShip, hOldShip);
        i = i.wrapping_add(1);
    }
    DeltaSISGauges(0x7fff as SIZE, 0x7fff as SIZE, 0x7fff as ::core::ffi::c_int);
    return i;
}
#[no_mangle]
pub unsafe extern "C" fn CalculateEscortsWorth() -> COUNT {
    let mut ShipCost: [COUNT; 23] = [
        1600 as COUNT,
        3000 as COUNT,
        1100 as COUNT,
        2300 as COUNT,
        2000 as COUNT,
        500 as COUNT,
        1800 as COUNT,
        1600 as COUNT,
        1000 as COUNT,
        2200 as COUNT,
        1200 as COUNT,
        2300 as COUNT,
        3600 as COUNT,
        1700 as COUNT,
        1000 as COUNT,
        2100 as COUNT,
        4400 as COUNT,
        700 as COUNT,
        3000 as COUNT,
        600 as COUNT,
        1300 as COUNT,
        3000 as COUNT,
        2300 as COUNT,
    ];
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
            + ShipCost[(*StarShipPtr).race_id as usize] as ::core::ffi::c_int) as COUNT;
        UnlockLink(&raw mut GlobData.Game_state.built_ship_q, hStarShip);
        hStarShip = hNextShip;
    }
    return total;
}
#[no_mangle]
pub unsafe extern "C" fn SetRaceAllied(mut race: COUNT, mut flag: BOOLEAN) -> COUNT {
    let mut hFleet: HFLEETINFO = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut FleetPtr: *mut FLEET_INFO = ::core::ptr::null_mut::<FLEET_INFO>();
    hFleet = GetStarShipFromIndex(&raw mut GlobData.Game_state.avail_race_q, race)
        as HFLEETINFO;
    if hFleet.is_null() {
        return 0 as COUNT;
    }
    FleetPtr = LockFleetInfo(&raw mut GlobData.Game_state.avail_race_q, hFleet);
    if (*FleetPtr).allied_state as ::core::ffi::c_int
        != C2Rust_Unnamed_5::DEAD_GUY.0 as ::core::ffi::c_int
    {
        (*FleetPtr).allied_state = (if flag.0 != 0 {
            C2Rust_Unnamed_5::GOOD_GUY.0 as ::core::ffi::c_int
        } else {
            C2Rust_Unnamed_5::BAD_GUY.0 as ::core::ffi::c_int
        }) as UWORD;
    }
    UnlockLink(&raw mut GlobData.Game_state.avail_race_q, hFleet);
    return 1 as COUNT;
}
#[no_mangle]
pub unsafe extern "C" fn StartSphereTracking(mut race: COUNT) -> COUNT {
    let mut hFleet: HFLEETINFO = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut FleetPtr: *mut FLEET_INFO = ::core::ptr::null_mut::<FLEET_INFO>();
    hFleet = GetStarShipFromIndex(&raw mut GlobData.Game_state.avail_race_q, race)
        as HFLEETINFO;
    if hFleet.is_null() {
        return 0 as COUNT;
    }
    FleetPtr = LockFleetInfo(&raw mut GlobData.Game_state.avail_race_q, hFleet);
    if (*FleetPtr).actual_strength as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        if (*FleetPtr).allied_state as ::core::ffi::c_int
            == C2Rust_Unnamed_5::DEAD_GUY.0 as ::core::ffi::c_int
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
    return race;
}
#[no_mangle]
pub unsafe extern "C" fn CountEscortShips(mut race: COUNT) -> COUNT {
    let mut hFleet: HFLEETINFO = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut hStarShip: HSHIPFRAG = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut hNextShip: HSHIPFRAG = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut result: COUNT = 0 as COUNT;
    hFleet = GetStarShipFromIndex(&raw mut GlobData.Game_state.avail_race_q, race)
        as HFLEETINFO;
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
        if ship_type as ::core::ffi::c_int == race as ::core::ffi::c_int {
            result = result.wrapping_add(1);
        }
        hStarShip = hNextShip;
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn HaveEscortShip(mut race: COUNT) -> BOOLEAN {
    return BOOLEAN(
        (CountEscortShips(race) as ::core::ffi::c_int > 0 as ::core::ffi::c_int)
            as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
}
#[no_mangle]
pub unsafe extern "C" fn EscortFeasibilityStudy(mut race: COUNT) -> COUNT {
    let mut hFleet: HFLEETINFO = ::core::ptr::null_mut::<::core::ffi::c_void>();
    hFleet = GetStarShipFromIndex(&raw mut GlobData.Game_state.avail_race_q, race)
        as HFLEETINFO;
    if hFleet.is_null() {
        return 0 as COUNT;
    }
    return (12 as ::core::ffi::c_int
        - CountLinks(&raw mut GlobData.Game_state.built_ship_q) as ::core::ffi::c_int)
        as COUNT;
}
#[no_mangle]
pub unsafe extern "C" fn CheckAlliance(mut race: COUNT) -> COUNT {
    let mut hFleet: HFLEETINFO = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut flags: UWORD = 0;
    let mut FleetPtr: *mut FLEET_INFO = ::core::ptr::null_mut::<FLEET_INFO>();
    hFleet = GetStarShipFromIndex(&raw mut GlobData.Game_state.avail_race_q, race)
        as HFLEETINFO;
    if hFleet.is_null() {
        return 0 as COUNT;
    }
    FleetPtr = LockFleetInfo(&raw mut GlobData.Game_state.avail_race_q, hFleet);
    flags = (*FleetPtr).allied_state;
    UnlockLink(&raw mut GlobData.Game_state.avail_race_q, hFleet);
    return flags;
}
#[no_mangle]
pub unsafe extern "C" fn RemoveSomeEscortShips(
    mut race: COUNT,
    mut count: COUNT,
) -> COUNT {
    let mut hStarShip: HSHIPFRAG = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut hNextShip: HSHIPFRAG = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if count as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        return 0 as COUNT;
    }
    hStarShip = GlobData.Game_state.built_ship_q.head as HSHIPFRAG;
    while !hStarShip.is_null() {
        let mut RemoveShip: BOOLEAN = BOOLEAN::FALSE;
        let mut StarShipPtr: *mut SHIP_FRAGMENT = ::core::ptr::null_mut::<
            SHIP_FRAGMENT,
        >();
        StarShipPtr = LockShipFrag(&raw mut GlobData.Game_state.built_ship_q, hStarShip);
        hNextShip = (*StarShipPtr).succ as HSHIPFRAG;
        RemoveShip = BOOLEAN(
            ((*StarShipPtr).race_id as ::core::ffi::c_int == race as ::core::ffi::c_int)
                as ::core::ffi::c_int as ::core::ffi::c_uint,
        );
        UnlockLink(&raw mut GlobData.Game_state.built_ship_q, hStarShip);
        if RemoveShip.0 != 0 {
            RemoveQueue(&raw mut GlobData.Game_state.built_ship_q, hStarShip);
            FreeLink(&raw mut GlobData.Game_state.built_ship_q, hStarShip);
            count = count.wrapping_sub(1);
            if count as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                break;
            }
        }
        hStarShip = hNextShip;
    }
    if count as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
        DeltaSISGauges(0x7fff as SIZE, 0x7fff as SIZE, 0x7fff as ::core::ffi::c_int);
    }
    return count;
}
#[no_mangle]
pub unsafe extern "C" fn RemoveEscortShips(mut race: COUNT) {
    RemoveSomeEscortShips(race, -1 as ::core::ffi::c_int as COUNT);
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
                434 as ::core::ffi::c_int,
                b"NameCaptain\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    loop {
        let mut hNextShip: HLINK = ::core::ptr::null_mut::<::core::ffi::c_void>();
        name_index = ((TFB_Random() as COUNT as ::core::ffi::c_int
            & 16 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
            + 5 as ::core::ffi::c_int) as BYTE;
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
    mut shipIndex: COUNT,
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
                471 as ::core::ffi::c_int,
                b"CloneShipFragment\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    hFleet = GetStarShipFromIndex(&raw mut GlobData.Game_state.avail_race_q, shipIndex)
        as HFLEETINFO;
    if hFleet.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    TemplatePtr = LockFleetInfo(&raw mut GlobData.Game_state.avail_race_q, hFleet);
    if shipIndex as ::core::ffi::c_int
        == C2Rust_Unnamed_6::SAMATRA_SHIP.0 as ::core::ffi::c_int
    {
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
        (*ShipFragPtr).race_id = shipIndex as BYTE;
        (*ShipFragPtr).index = 0 as BYTE;
        UnlockLink(pDstQueue, hBuiltShip);
    }
    UnlockLink(&raw mut GlobData.Game_state.avail_race_q, hFleet);
    return hBuiltShip;
}
#[no_mangle]
pub unsafe extern "C" fn SetEscortCrewComplement(
    mut which_ship: COUNT,
    mut crew_level: COUNT,
    mut captain: BYTE,
) -> ::core::ffi::c_int {
    let mut hFleet: HFLEETINFO = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut TemplatePtr: *mut FLEET_INFO = ::core::ptr::null_mut::<FLEET_INFO>();
    let mut hStarShip: HSHIPFRAG = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut hNextShip: HSHIPFRAG = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut StarShipPtr: *mut SHIP_FRAGMENT = ::core::ptr::null_mut::<SHIP_FRAGMENT>();
    let mut Index: ::core::ffi::c_int = 0;
    hFleet = GetStarShipFromIndex(&raw mut GlobData.Game_state.avail_race_q, which_ship)
        as HFLEETINFO;
    if hFleet.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    TemplatePtr = LockFleetInfo(&raw mut GlobData.Game_state.avail_race_q, hFleet);
    hStarShip = GlobData.Game_state.built_ship_q.head as HSHIPFRAG;
    Index = 0 as ::core::ffi::c_int;
    while !hStarShip.is_null() {
        StarShipPtr = LockShipFrag(&raw mut GlobData.Game_state.built_ship_q, hStarShip);
        hNextShip = (*StarShipPtr).succ as HSHIPFRAG;
        if which_ship as ::core::ffi::c_int
            == (*StarShipPtr).race_id as ::core::ffi::c_int
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
        (*StarShipPtr).crew_level = crew_level;
        (*StarShipPtr).captains_name_index = captain;
        UnlockLink(&raw mut GlobData.Game_state.built_ship_q, hStarShip);
    } else {
        Index = -1 as ::core::ffi::c_int;
    }
    UnlockLink(&raw mut GlobData.Game_state.avail_race_q, hFleet);
    return Index;
}
