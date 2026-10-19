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

// Rust translation of src/uqm/ships/ilwrath/ilwrath.c.
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
    fn SetAbsFrameIndex(Frame: FRAME, FrameIndex: COUNT) -> FRAME;
    fn IncFrameIndex(Frame: FRAME) -> FRAME;
    fn SetAbsStringTableIndex(String: STRING, StringTableIndex: COUNT) -> STRING;
    static mut resolutionFactor: ::core::ffi::c_uint;
    fn ARCTAN(delta_x: SDWORD, delta_y: SDWORD) -> COUNT;
    fn GetCurrentVelocityComponentsSdword(
        velocityptr: *mut VELOCITY_DESC,
        pdx: *mut SDWORD,
        pdy: *mut SDWORD,
    );
    fn GetNextVelocityComponents(
        velocityptr: *mut VELOCITY_DESC,
        pdx: *mut SIZE,
        pdy: *mut SIZE,
        num_frames: COUNT,
    );
    fn DeltaVelocityComponents(velocityptr: *mut VELOCITY_DESC, dx: SDWORD, dy: SDWORD);
    static mut disp_q: QUEUE;
    fn getGameStateUint(name: *const ::core::ffi::c_char) -> DWORD;
    static mut GlobData: GLOBDATA;
    fn ProcessSound(Sound: SOUND, PositionalObject: *mut ELEMENT);
    fn DeltaEnergy(ElementPtr: *mut ELEMENT, energy_delta: SIZE) -> BOOLEAN;
    fn ship_intelligence(
        ShipPtr: *mut ELEMENT,
        ObjectsOfConcern: *mut EVALUATE_DESC,
        ConcernCounter: COUNT,
    );
    fn initialize_missile(pMissileBlock: *mut MISSILE_BLOCK) -> HELEMENT;
    fn weapon_collision(
        ElementPtr0: *mut ELEMENT,
        pPt0: *mut POINT,
        ElementPtr1: *mut ELEMENT,
        pPt1: *mut POINT,
    ) -> HELEMENT;
    fn TrackShip(Tracker: *mut ELEMENT, pfacing: *mut COUNT) -> SIZE;
    fn Untarget(ElementPtr: *mut ELEMENT);
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
static mut ilwrath_desc: RACE_DESC = unsafe {
    race_desc {
        c2rust_padding: [0; 8],
        c2rust_padding_0: [0; 10],
        c2rust_padding_1: [0; 2],
        c2rust_padding_2: [0; 8],
        ship_info: SHIP_INFO {
            idStr: b"avenger\0".as_ptr() as *const ::core::ffi::c_char,
            ship_flags: ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as UWORD,
            ship_cost: 10 as BYTE,
            crew_level: 22 as COUNT,
            max_crew: 22 as COUNT,
            energy_level: 16 as BYTE,
            max_energy: 16 as BYTE,
            race_strings_rsc: b"ship.ilwrath.text\0".as_ptr() as RESOURCE,
            icons_rsc: b"ship.ilwrath.icons\0".as_ptr() as RESOURCE,
            melee_icon_rsc: b"ship.ilwrath.meleeicons\0".as_ptr() as RESOURCE,
            race_strings: ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>(),
            icons: ::core::ptr::null_mut::<FRAME_DESC>(),
            melee_icon: ::core::ptr::null_mut::<FRAME_DESC>(),
        },
        fleet: FLEET_STUFF {
            strength: (1410 as ::core::ffi::c_int / 11 as ::core::ffi::c_int
                * 2 as ::core::ffi::c_int) as COUNT,
            known_loc: point {
                x: 48 as COORD,
                y: 1700 as COORD,
            },
        },
        characteristics: CHARACTERISTIC_STUFF {
            max_thrust: 25 as COUNT,
            thrust_increment: 5 as COUNT,
            energy_regeneration: 4 as BYTE,
            weapon_energy_cost: 1 as BYTE,
            special_energy_cost: 3 as BYTE,
            energy_wait: 4 as BYTE,
            turn_wait: 2 as BYTE,
            thrust_wait: 0 as BYTE,
            weapon_wait: 0 as BYTE,
            special_wait: 13 as BYTE,
            ship_mass: 7 as BYTE,
        },
        ship_data: DATA_STUFF {
            ship_rsc: [
                b"ship.ilwrath.graphics.avenger.large\0".as_ptr() as RESOURCE,
                b"ship.ilwrath.graphics.avenger.medium\0".as_ptr() as RESOURCE,
                b"ship.ilwrath.graphics.avenger.small\0".as_ptr() as RESOURCE,
            ],
            weapon_rsc: [
                b"ship.ilwrath.graphics.fire.large\0".as_ptr() as RESOURCE,
                b"ship.ilwrath.graphics.fire.medium\0".as_ptr() as RESOURCE,
                b"ship.ilwrath.graphics.fire.small\0".as_ptr() as RESOURCE,
            ],
            special_rsc: [
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
            ],
            captain_control: captain_stuff {
                captain_rsc: b"ship.ilwrath.graphics.captain\0".as_ptr() as RESOURCE,
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
            victory_ditty_rsc: b"ship.ilwrath.ditty\0".as_ptr() as RESOURCE,
            ship_sounds_rsc: b"ship.ilwrath.sounds\0".as_ptr() as RESOURCE,
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
unsafe extern "C" fn flame_preprocess(mut ElementPtr: *mut ELEMENT) {
    if (*ElementPtr).c2rust_unnamed_2.turn_wait as ::core::ffi::c_int
        > 0 as ::core::ffi::c_int
    {
        (*ElementPtr).c2rust_unnamed_2.turn_wait = (*ElementPtr)
            .c2rust_unnamed_2
            .turn_wait
            .wrapping_sub(1);
    } else {
        (*ElementPtr).next.image.frame = IncFrameIndex(
            (*ElementPtr).current.image.frame,
        );
        (*ElementPtr).state_flags = ((*ElementPtr).state_flags as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as ELEMENT_FLAGS;
        (*ElementPtr).c2rust_unnamed_2.turn_wait = (*ElementPtr)
            .c2rust_unnamed_3
            .next_turn;
    };
}
unsafe extern "C" fn flame_collision(
    mut ElementPtr0: *mut ELEMENT,
    mut pPt0: *mut POINT,
    mut ElementPtr1: *mut ELEMENT,
    mut pPt1: *mut POINT,
) {
    uqm_lua_checked!(weapon_collision(ElementPtr0, pPt0, ElementPtr1, pPt1), ());
    (*ElementPtr0).state_flags = ((*ElementPtr0).state_flags as ::core::ffi::c_int
        & !((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)) as ELEMENT_FLAGS;
    (*ElementPtr0).state_flags = ((*ElementPtr0).state_flags as ::core::ffi::c_int
        | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as ELEMENT_FLAGS;
}
unsafe extern "C" fn ilwrath_intelligence(
    mut ShipPtr: *mut ELEMENT,
    mut ObjectsOfConcern: *mut EVALUATE_DESC,
    mut ConcernCounter: COUNT,
) {
    let mut lpEvalDesc: *mut EVALUATE_DESC = ::core::ptr::null_mut::<EVALUATE_DESC>();
    let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
    lpEvalDesc = ObjectsOfConcern
        .offset(C2Rust_Unnamed_6::ENEMY_SHIP_INDEX.0 as ::core::ffi::c_int as isize);
    (*lpEvalDesc).MoveState = MOVEMENT_STATE::PURSUE;
    if !(*lpEvalDesc).ObjectPtr.is_null()
        && (*lpEvalDesc).which_turn as ::core::ffi::c_int <= 10 as ::core::ffi::c_int
    {
        (*ObjectsOfConcern
            .offset(
                C2Rust_Unnamed_6::ENEMY_WEAPON_INDEX.0 as ::core::ffi::c_int as isize,
            ))
            .ObjectPtr = ::core::ptr::null_mut::<ELEMENT>();
    }
    uqm_lua_checked!(ship_intelligence(ShipPtr, ObjectsOfConcern, ConcernCounter), ());
    StarShipPtr = (*ShipPtr).pParent as *mut STARSHIP;
    if !(*lpEvalDesc).ObjectPtr.is_null()
        && ((*lpEvalDesc).which_turn as ::core::ffi::c_int <= 6 as ::core::ffi::c_int
            || (*lpEvalDesc).which_turn as ::core::ffi::c_int <= 10 as ::core::ffi::c_int
                && (*ObjectsOfConcern
                    .offset(
                        C2Rust_Unnamed_6::ENEMY_WEAPON_INDEX.0 as ::core::ffi::c_int
                            as isize,
                    ))
                    .which_turn as ::core::ffi::c_int <= 10 as ::core::ffi::c_int)
    {
        (*StarShipPtr).ship_input_state = ((*StarShipPtr).ship_input_state
            as ::core::ffi::c_int
            & !((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)) as BYTE;
        if (*GlobData.Game_state.DisplayArray.offset((*ShipPtr).PrimIndex as isize)).Type
            as ::core::ffi::c_int >= gfx_object::NUM_PRIMS.0 as ::core::ffi::c_int
            || (*GlobData.Game_state.DisplayArray.offset((*ShipPtr).PrimIndex as isize))
                .Type as ::core::ffi::c_int
                == gfx_object::STAMPFILL_PRIM.0 as ::core::ffi::c_int
                && sameColor(
                    (*GlobData
                        .Game_state
                        .DisplayArray
                        .offset((*ShipPtr).PrimIndex as isize))
                        .color,
                    colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE),
                ) as ::core::ffi::c_int != 0
        {
            (*StarShipPtr).ship_input_state = ((*StarShipPtr).ship_input_state
                as ::core::ffi::c_int
                & (!((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int)
                    | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)) as BYTE;
            (*StarShipPtr).ship_input_state = ((*StarShipPtr).ship_input_state
                as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as BYTE;
        }
        (*StarShipPtr).ship_input_state = ((*StarShipPtr).ship_input_state
            as ::core::ffi::c_int | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
            as BYTE;
    } else if (*StarShipPtr).special_counter as ::core::ffi::c_int
        == 0 as ::core::ffi::c_int
        && (GlobData.Game_state.CurrentActivity as BYTE as ::core::ffi::c_int
            != C2Rust_Unnamed_5::IN_ENCOUNTER.0 as ::core::ffi::c_int
            || uqm_lua_checked!(getGameStateUint(
                b"PROBE_ILWRATH_ENCOUNTER\0".as_ptr() as *const ::core::ffi::c_char,
            ), ()) == 0)
    {
        (*StarShipPtr).ship_input_state = ((*StarShipPtr).ship_input_state
            as ::core::ffi::c_int
            & !((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)) as BYTE;
        if !((*GlobData.Game_state.DisplayArray.offset((*ShipPtr).PrimIndex as isize))
            .Type as ::core::ffi::c_int >= gfx_object::NUM_PRIMS.0 as ::core::ffi::c_int
            || (*GlobData.Game_state.DisplayArray.offset((*ShipPtr).PrimIndex as isize))
                .Type as ::core::ffi::c_int
                == gfx_object::STAMPFILL_PRIM.0 as ::core::ffi::c_int
                && sameColor(
                    (*GlobData
                        .Game_state
                        .DisplayArray
                        .offset((*ShipPtr).PrimIndex as isize))
                        .color,
                    colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE),
                ) as ::core::ffi::c_int != 0)
            && (*StarShipPtr).ship_input_state as ::core::ffi::c_int
                & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int == 0
        {
            (*StarShipPtr).ship_input_state = ((*StarShipPtr).ship_input_state
                as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as BYTE;
        }
    }
}
unsafe extern "C" fn initialize_flame(
    mut ShipPtr: *mut ELEMENT,
    mut FlameArray: *mut HELEMENT,
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
    MissileBlock.face = (*StarShipPtr).ShipFacing;
    MissileBlock.index = 0 as COUNT;
    MissileBlock.sender = (*ShipPtr).playerNr;
    MissileBlock.flags = ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
        as ELEMENT_FLAGS;
    MissileBlock.pixoffs = ((29 as ::core::ffi::c_int) << resolutionFactor) as SIZE;
    MissileBlock.speed = ((25 as ::core::ffi::c_int) << resolutionFactor) as SIZE;
    MissileBlock.hit_points = 1 as SIZE;
    MissileBlock.damage = 1 as SIZE;
    MissileBlock.life = 8 as COUNT;
    MissileBlock.preprocess_func = Some(
        flame_preprocess as unsafe extern "C" fn(*mut ELEMENT) -> (),
    ) as Option<unsafe extern "C" fn(*mut ELEMENT) -> ()>;
    MissileBlock.blast_offs = 0 as SIZE;
    *FlameArray.offset(0isize) = initialize_missile(&raw mut MissileBlock);
    if !(*FlameArray.offset(0isize)).is_null() {
        let mut dx: SDWORD = 0;
        let mut dy: SDWORD = 0;
        let mut FlamePtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
        FlamePtr = LockLink(&raw mut disp_q, *FlameArray.offset(0isize)) as *mut ELEMENT;
        GetCurrentVelocityComponentsSdword(
            &raw mut (*ShipPtr).velocity,
            &raw mut dx,
            &raw mut dy,
        );
        DeltaVelocityComponents(&raw mut (*FlamePtr).velocity, dx, dy);
        (*FlamePtr).current.location.x = ((*FlamePtr).current.location.x
            as ::core::ffi::c_int
            - (dx >> 5 as ::core::ffi::c_int) as ::core::ffi::c_int) as COORD;
        (*FlamePtr).current.location.y = ((*FlamePtr).current.location.y
            as ::core::ffi::c_int
            - (dy >> 5 as ::core::ffi::c_int) as ::core::ffi::c_int) as COORD;
        (*FlamePtr).collision_func = Some(
            flame_collision
                as unsafe extern "C" fn(
                    *mut ELEMENT,
                    *mut POINT,
                    *mut ELEMENT,
                    *mut POINT,
                ) -> (),
        ) as Option<ElementCollisionFunc>;
        (*FlamePtr).c2rust_unnamed_2.turn_wait = 0 as BYTE;
        UnlockLink(&raw mut disp_q, *FlameArray.offset(0isize));
    }
    return 1 as COUNT;
}
unsafe extern "C" fn ilwrath_preprocess(mut ElementPtr: *mut ELEMENT) {
    let mut status_flags: STATUS_FLAGS = 0;
    let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
    let mut lpPrim: *mut PRIMITIVE = ::core::ptr::null_mut::<PRIMITIVE>();
    StarShipPtr = (*ElementPtr).pParent as *mut STARSHIP;
    status_flags = (*StarShipPtr).cur_status_flags;
    lpPrim = GlobData.Game_state.DisplayArray.offset((*ElementPtr).PrimIndex as isize);
    if (*lpPrim).Type as ::core::ffi::c_int
        == gfx_object::STAMPFILL_PRIM.0 as ::core::ffi::c_int
    {
        let mut color: Color = Color { r: 0, g: 0, b: 0, a: 0 };
        let mut weapon_discharge: BOOLEAN = BOOLEAN::FALSE;
        color = (*lpPrim).color;
        weapon_discharge = BOOLEAN(
            (status_flags as ::core::ffi::c_int
                & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int != 0
                && (*(*StarShipPtr).RaceDescPtr).ship_info.energy_level
                    as ::core::ffi::c_int >= 1 as ::core::ffi::c_int)
                as ::core::ffi::c_int as ::core::ffi::c_uint,
        );
        if weapon_discharge.0 != 0
            || (*StarShipPtr).special_counter as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
                && (status_flags as ::core::ffi::c_int
                    & (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int != 0
                    || !sameColor(
                        color,
                        colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE),
                    ))
        {
            if sameColor(
                color,
                colorFromRgb15(0x1f as BYTE, 0x1f as BYTE, 0x1f as BYTE),
            ) {
                (*lpPrim).Type = gfx_object::STAMP_PRIM.0 as ::core::ffi::c_int
                    as GRAPHICS_PRIM;
            } else if sameColor(
                color,
                colorFromRgb15(0xa as BYTE, 0x1f as BYTE, 0x1f as BYTE),
            ) {
                (*lpPrim).color = colorFromRgb15(
                    0x1f as BYTE,
                    0x1f as BYTE,
                    0x1f as BYTE,
                );
            } else if sameColor(
                color,
                colorFromRgb15(0 as BYTE, 0x14 as BYTE, 0x14 as BYTE),
            ) {
                (*lpPrim).color = colorFromRgb15(
                    0xa as BYTE,
                    0x1f as BYTE,
                    0x1f as BYTE,
                );
            } else if sameColor(
                color,
                colorFromRgb15(0xa as BYTE, 0xa as BYTE, 0x1f as BYTE),
            ) {
                (*lpPrim).color = colorFromRgb15(0 as BYTE, 0x14 as BYTE, 0x14 as BYTE);
            } else if sameColor(
                color,
                colorFromRgb15(0 as BYTE, 0 as BYTE, 0x14 as BYTE),
            ) {
                (*lpPrim).color = colorFromRgb15(0xa as BYTE, 0xa as BYTE, 0x1f as BYTE);
            } else {
                ProcessSound(
                    SetAbsStringTableIndex(
                        (*(*StarShipPtr).RaceDescPtr).ship_data.ship_sounds,
                        2 as COUNT,
                    ),
                    ElementPtr,
                );
                (*lpPrim).color = colorFromRgb15(0 as BYTE, 0 as BYTE, 0x14 as BYTE);
                if weapon_discharge.0 != 0 {
                    let mut facing: COUNT = 0;
                    facing = (*StarShipPtr).ShipFacing;
                    if TrackShip(ElementPtr, &raw mut facing) as ::core::ffi::c_int
                        >= 0 as ::core::ffi::c_int
                    {
                        let mut eptr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
                        let mut dx0: SIZE = 0;
                        let mut dy0: SIZE = 0;
                        let mut dx1: SIZE = 0;
                        let mut dy1: SIZE = 0;
                        let mut v: VELOCITY_DESC = VELOCITY_DESC {
                            TravelAngle: 0,
                            vector: EXTENT { width: 0, height: 0 },
                            fract: EXTENT { width: 0, height: 0 },
                            error: EXTENT { width: 0, height: 0 },
                            incr: EXTENT { width: 0, height: 0 },
                        };
                        eptr = LockLink(&raw mut disp_q, (*ElementPtr).hTarget)
                            as *mut ELEMENT;
                        v = (*eptr).velocity;
                        GetNextVelocityComponents(
                            &raw mut v,
                            &raw mut dx0,
                            &raw mut dy0,
                            4 as COUNT,
                        );
                        v = (*ElementPtr).velocity;
                        GetNextVelocityComponents(
                            &raw mut v,
                            &raw mut dx1,
                            &raw mut dy1,
                            4 as COUNT,
                        );
                        dx0 = ((*eptr).current.location.x as ::core::ffi::c_int
                            + dx0 as ::core::ffi::c_int
                            - ((*ElementPtr).current.location.x as ::core::ffi::c_int
                                + dx1 as ::core::ffi::c_int)) as SIZE;
                        dy0 = ((*eptr).current.location.y as ::core::ffi::c_int
                            + dy0 as ::core::ffi::c_int
                            - ((*ElementPtr).current.location.y as ::core::ffi::c_int
                                + dy1 as ::core::ffi::c_int)) as SIZE;
                        UnlockLink(&raw mut disp_q, (*ElementPtr).hTarget);
                        (*StarShipPtr).ShipFacing = (ARCTAN(dx0 as SDWORD, dy0 as SDWORD)
                            as ::core::ffi::c_int
                            + ((1 as ::core::ffi::c_int)
                                << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                                    - 1 as ::core::ffi::c_int)
                            >> 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                            & ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                                - 1 as ::core::ffi::c_int) as DWORD as COUNT;
                        if (*ElementPtr).c2rust_unnamed_2.turn_wait as ::core::ffi::c_int
                            == 0 as ::core::ffi::c_int
                        {
                            (*ElementPtr).c2rust_unnamed_2.turn_wait = (*ElementPtr)
                                .c2rust_unnamed_2
                                .turn_wait
                                .wrapping_add(1);
                        }
                        (*ElementPtr).next.image.frame = SetAbsFrameIndex(
                            (*ElementPtr).next.image.frame,
                            (*StarShipPtr).ShipFacing,
                        );
                    }
                    (*ElementPtr).hTarget = ::core::ptr::null_mut::<
                        ::core::ffi::c_void,
                    >();
                }
            }
            (*ElementPtr).state_flags = ((*ElementPtr).state_flags as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as ELEMENT_FLAGS;
            status_flags = (status_flags as ::core::ffi::c_int
                & !((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int))
                as STATUS_FLAGS;
            (*StarShipPtr).special_counter = 0 as BYTE;
        } else if !sameColor(color, colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE)) {
            if sameColor(color, colorFromRgb15(0 as BYTE, 0 as BYTE, 0x14 as BYTE)) {
                (*lpPrim).color = colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE);
                Untarget(ElementPtr);
            } else if sameColor(
                color,
                colorFromRgb15(0xa as BYTE, 0xa as BYTE, 0x1f as BYTE),
            ) {
                (*lpPrim).color = colorFromRgb15(0 as BYTE, 0 as BYTE, 0x14 as BYTE);
            } else if sameColor(
                color,
                colorFromRgb15(0 as BYTE, 0x14 as BYTE, 0x14 as BYTE),
            ) {
                (*lpPrim).color = colorFromRgb15(0xa as BYTE, 0xa as BYTE, 0x1f as BYTE);
            } else if sameColor(
                color,
                colorFromRgb15(0xa as BYTE, 0x1f as BYTE, 0x1f as BYTE),
            ) {
                (*lpPrim).color = colorFromRgb15(0 as BYTE, 0x14 as BYTE, 0x14 as BYTE);
            } else {
                (*lpPrim).color = colorFromRgb15(
                    0xa as BYTE,
                    0x1f as BYTE,
                    0x1f as BYTE,
                );
            }
            (*ElementPtr).state_flags = ((*ElementPtr).state_flags as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as ELEMENT_FLAGS;
        }
    }
    if status_flags as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int != 0
        && (*StarShipPtr).special_counter as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int && uqm_lua_checked!(DeltaEnergy(ElementPtr, -3 as SIZE), ()).0 != 0
    {
        (*lpPrim).color = colorFromRgb15(0x1f as BYTE, 0x1f as BYTE, 0x1f as BYTE);
        (*lpPrim).Type = gfx_object::STAMPFILL_PRIM.0 as ::core::ffi::c_int
            as GRAPHICS_PRIM;
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
        (*ElementPtr).state_flags = ((*ElementPtr).state_flags as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as ELEMENT_FLAGS;
    }
}
#[no_mangle]
pub unsafe extern "C" fn init_ilwrath() -> *mut RACE_DESC {
    let mut RaceDescPtr: *mut RACE_DESC = ::core::ptr::null_mut::<RACE_DESC>();
    if if resolutionFactor != 2 as ::core::ffi::c_uint {
        BOOLEAN::FALSE.0 as ::core::ffi::c_int
    } else {
        BOOLEAN::TRUE.0 as ::core::ffi::c_int
    } != 0
    {
        ilwrath_desc.characteristics.max_thrust = ((25 as ::core::ffi::c_int)
            << resolutionFactor) as COUNT;
        ilwrath_desc.characteristics.thrust_increment = ((5 as ::core::ffi::c_int)
            << resolutionFactor) as COUNT;
        ilwrath_desc.cyborg_control.WeaponRange = ((200 as ::core::ffi::c_int)
            << 2 as ::core::ffi::c_int) as COUNT;
    } else {
        ilwrath_desc.characteristics.max_thrust = 25 as COUNT;
        ilwrath_desc.characteristics.thrust_increment = 5 as COUNT;
        ilwrath_desc.cyborg_control.WeaponRange = ((50 as ::core::ffi::c_int)
            << 2 as ::core::ffi::c_int) as COUNT;
    }
    ilwrath_desc.preprocess_func = Some(
        ilwrath_preprocess as unsafe extern "C" fn(*mut ELEMENT) -> (),
    ) as Option<PREPROCESS_FUNC>;
    ilwrath_desc.init_weapon_func = Some(
        initialize_flame as unsafe extern "C" fn(*mut ELEMENT, *mut HELEMENT) -> COUNT,
    ) as Option<INIT_WEAPON_FUNC>;
    ilwrath_desc.cyborg_control.intelligence_func = Some(
        ilwrath_intelligence
            as unsafe extern "C" fn(*mut ELEMENT, *mut EVALUATE_DESC, COUNT) -> (),
    ) as Option<IntelligenceFunc>;
    RaceDescPtr = &raw mut ilwrath_desc;
    return RaceDescPtr;
}
