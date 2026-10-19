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

// Rust translation of src/uqm/ships/lastbat/lastbat.c.
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
    fn InsertQueue(pq: *mut QUEUE, hLink: HLINK, hRefLink: HLINK);
    fn GetFrameIndex(Frame: FRAME) -> COUNT;
    fn SetAbsFrameIndex(Frame: FRAME, FrameIndex: COUNT) -> FRAME;
    fn IncFrameIndex(Frame: FRAME) -> FRAME;
    fn SetAbsStringTableIndex(String: STRING, StringTableIndex: COUNT) -> STRING;
    fn memset(
        _: *mut ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    static mut resolutionFactor: ::core::ffi::c_uint;
    static mut optWindowType: DWORD;
    static mut CanvasWidth: ::core::ffi::c_int;
    static mut CanvasHeight: ::core::ffi::c_int;
    static mut sinetab: [SDWORD; 0];
    fn ARCTAN(delta_x: SDWORD, delta_y: SDWORD) -> COUNT;
    fn GetCurrentVelocityComponents(
        velocityptr: *mut VELOCITY_DESC,
        pdx: *mut SIZE,
        pdy: *mut SIZE,
    );
    fn GetCurrentVelocityComponentsSdword(
        velocityptr: *mut VELOCITY_DESC,
        pdx: *mut SDWORD,
        pdy: *mut SDWORD,
    );
    fn GetNextVelocityComponentsSdword(
        velocityptr: *mut VELOCITY_DESC,
        pdx: *mut SDWORD,
        pdy: *mut SDWORD,
        num_frames: DWORD,
    );
    fn SetVelocityVector(
        velocityptr: *mut VELOCITY_DESC,
        magnitude: SDWORD,
        facing: COUNT,
    );
    fn SetVelocityComponents(velocityptr: *mut VELOCITY_DESC, dx: SDWORD, dy: SDWORD);
    fn DeltaVelocityComponents(velocityptr: *mut VELOCITY_DESC, dx: SDWORD, dy: SDWORD);
    static mut disp_q: QUEUE;
    fn AllocElement() -> HELEMENT;
    fn FreeElement(hElement: HELEMENT);
    fn RemoveElement(hLink: HLINK);
    fn getGameStateUint(name: *const ::core::ffi::c_char) -> DWORD;
    static mut GlobData: GLOBDATA;
    fn TFB_Random() -> DWORD;
    fn square_root(value: DWORD) -> COUNT;
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
static mut samatra_desc: RACE_DESC = unsafe {
    race_desc {
        c2rust_padding: [0; 8],
        c2rust_padding_0: [0; 10],
        c2rust_padding_1: [0; 2],
        c2rust_padding_2: [0; 8],
        ship_info: SHIP_INFO {
            idStr: b"samatra\0".as_ptr() as *const ::core::ffi::c_char,
            ship_flags: ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as UWORD,
            ship_cost: 16 as BYTE,
            crew_level: 1 as COUNT,
            max_crew: 1 as COUNT,
            energy_level: 42 as BYTE,
            max_energy: 42 as BYTE,
            race_strings_rsc: ::core::ptr::null::<::core::ffi::c_char>(),
            icons_rsc: ::core::ptr::null::<::core::ffi::c_char>(),
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
            max_thrust: 0 as COUNT,
            thrust_increment: 0 as COUNT,
            energy_regeneration: 1 as BYTE,
            weapon_energy_cost: 2 as BYTE,
            special_energy_cost: 3 as BYTE,
            energy_wait: 6 as BYTE,
            turn_wait: 0 as BYTE,
            thrust_wait: 0 as BYTE,
            weapon_wait: (840 as ::core::ffi::c_int
                / (840 as ::core::ffi::c_int / 24 as ::core::ffi::c_int)
                * 10 as ::core::ffi::c_int) as BYTE,
            special_wait: (840 as ::core::ffi::c_int
                / (840 as ::core::ffi::c_int / 24 as ::core::ffi::c_int)
                * 3 as ::core::ffi::c_int) as BYTE,
            ship_mass: (10 as ::core::ffi::c_int * 10 as ::core::ffi::c_int) as BYTE,
        },
        ship_data: DATA_STUFF {
            ship_rsc: [
                b"ship.samatra.graphics.samatra.large\0".as_ptr() as RESOURCE,
                b"ship.samatra.graphics.samatra.medium\0".as_ptr() as RESOURCE,
                b"ship.samatra.graphics.samatra.small\0".as_ptr() as RESOURCE,
            ],
            weapon_rsc: [
                b"ship.samatra.graphics.sentinel.large\0".as_ptr() as RESOURCE,
                b"ship.samatra.graphics.sentinel.medium\0".as_ptr() as RESOURCE,
                b"ship.samatra.graphics.sentinel.small\0".as_ptr() as RESOURCE,
            ],
            special_rsc: [
                b"ship.samatra.graphics.generator.large\0".as_ptr() as RESOURCE,
                b"ship.samatra.graphics.generator.medium\0".as_ptr() as RESOURCE,
                b"ship.samatra.graphics.generator.small\0".as_ptr() as RESOURCE,
            ],
            captain_control: captain_stuff {
                captain_rsc: b"ship.samatra.graphics.captain\0".as_ptr() as RESOURCE,
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
            victory_ditty_rsc: ::core::ptr::null::<::core::ffi::c_char>(),
            ship_sounds_rsc: b"ship.samatra.sounds\0".as_ptr() as RESOURCE,
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
            WeaponRange: 0 as COUNT,
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
unsafe extern "C" fn comet_preprocess(mut ElementPtr: *mut ELEMENT) {
    let mut frame_index: COUNT = 0;
    frame_index = (GetFrameIndex((*ElementPtr).current.image.frame) as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as COUNT;
    if (frame_index as ::core::ffi::c_int) < 29 as ::core::ffi::c_int {
        if frame_index as ::core::ffi::c_int == 25 as ::core::ffi::c_int {
            let mut cur_delta_x: SIZE = 0;
            let mut cur_delta_y: SIZE = 0;
            let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
            StarShipPtr = (*ElementPtr).pParent as *mut STARSHIP;
            (*(*StarShipPtr).RaceDescPtr).characteristics.weapon_wait = (*(*StarShipPtr)
                .RaceDescPtr)
                .characteristics
                .weapon_wait
                .wrapping_add(1);
            spawn_comet(ElementPtr);
            (*ElementPtr).state_flags = ((*ElementPtr).state_flags as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as ELEMENT_FLAGS;
            GetCurrentVelocityComponents(
                &raw mut (*ElementPtr).velocity,
                &raw mut cur_delta_x,
                &raw mut cur_delta_y,
            );
            SetVelocityComponents(
                &raw mut (*ElementPtr).velocity,
                cur_delta_x as SDWORD / 2 as SDWORD,
                cur_delta_y as SDWORD / 2 as SDWORD,
            );
        }
        (*ElementPtr).c2rust_unnamed.life_span = (*ElementPtr)
            .c2rust_unnamed
            .life_span
            .wrapping_add(1);
    }
    (*ElementPtr).next.image.frame = SetAbsFrameIndex(
        (*ElementPtr).current.image.frame,
        frame_index,
    );
    (*ElementPtr).state_flags = ((*ElementPtr).state_flags as ::core::ffi::c_int
        | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as ELEMENT_FLAGS;
}
unsafe extern "C" fn comet_collision(
    mut ElementPtr0: *mut ELEMENT,
    mut pPt0: *mut POINT,
    mut ElementPtr1: *mut ELEMENT,
    mut pPt1: *mut POINT,
) {
    if (*ElementPtr1).playerNr as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        let mut old_hits: BYTE = 0;
        let mut old_life: COUNT = 0;
        let mut hBlastElement: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if (*ElementPtr1).state_flags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int != 0
        {
            (*ElementPtr0).c2rust_unnamed_1.mass_points = 2 as BYTE;
        } else {
            (*ElementPtr0).c2rust_unnamed_1.mass_points = 50 as BYTE;
        }
        old_hits = (*ElementPtr0).c2rust_unnamed_0.hit_points as BYTE;
        old_life = (*ElementPtr0).c2rust_unnamed.life_span;
        hBlastElement = uqm_lua_checked!(weapon_collision(ElementPtr0, pPt0, ElementPtr1, pPt1), ());
        if (*ElementPtr1).state_flags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int != 0
        {
            (*ElementPtr0).c2rust_unnamed_0.hit_points = old_hits as COUNT;
            (*ElementPtr0).c2rust_unnamed.life_span = old_life;
            (*ElementPtr0).state_flags = ((*ElementPtr0).state_flags
                as ::core::ffi::c_int
                & !((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int))
                as ELEMENT_FLAGS;
            if !hBlastElement.is_null() {
                RemoveElement(hBlastElement);
                FreeElement(hBlastElement);
            }
        }
        if (*ElementPtr0).state_flags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int != 0
        {
            let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
            StarShipPtr = (*ElementPtr0).pParent as *mut STARSHIP;
            (*(*StarShipPtr).RaceDescPtr).characteristics.weapon_wait = (*(*StarShipPtr)
                .RaceDescPtr)
                .characteristics
                .weapon_wait
                .wrapping_sub(1);
        }
    }
}
unsafe extern "C" fn spawn_comet(mut ElementPtr: *mut ELEMENT) -> HELEMENT {
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
    let mut hComet: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
    StarShipPtr = (*ElementPtr).pParent as *mut STARSHIP;
    MissileBlock.cx = (*ElementPtr).next.location.x;
    MissileBlock.cy = (*ElementPtr).next.location.y;
    MissileBlock.farray = &raw mut (*(*StarShipPtr).RaceDescPtr).ship_data.weapon
        as *mut FRAME;
    MissileBlock.face = 0 as COUNT;
    MissileBlock.index = 24 as COUNT;
    MissileBlock.sender = (*ElementPtr).playerNr;
    MissileBlock.flags = ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
        as ELEMENT_FLAGS;
    MissileBlock.pixoffs = 0 as SIZE;
    MissileBlock.speed = 0 as SIZE;
    MissileBlock.hit_points = (if if GlobData.SIS_state.Difficulty as ::core::ffi::c_int
        != 0
    {
        GlobData.SIS_state.Difficulty as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    } == 0 as ::core::ffi::c_int
    {
        12 as ::core::ffi::c_int
    } else if if GlobData.SIS_state.Difficulty as ::core::ffi::c_int != 0 {
        GlobData.SIS_state.Difficulty as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    } == 1 as ::core::ffi::c_int
    {
        10 as ::core::ffi::c_int
    } else {
        15 as ::core::ffi::c_int
    }) as SIZE;
    MissileBlock.damage = 2 as SIZE;
    MissileBlock.life = 2 as COUNT;
    MissileBlock.preprocess_func = Some(
        comet_preprocess as unsafe extern "C" fn(*mut ELEMENT) -> (),
    ) as Option<unsafe extern "C" fn(*mut ELEMENT) -> ()>;
    MissileBlock.blast_offs = 0 as SIZE;
    hComet = initialize_missile(&raw mut MissileBlock);
    if !hComet.is_null() {
        let mut CometPtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
        PutQueue(&raw mut disp_q, hComet);
        CometPtr = LockLink(&raw mut disp_q, hComet) as *mut ELEMENT;
        (*CometPtr).collision_func = Some(
            comet_collision
                as unsafe extern "C" fn(
                    *mut ELEMENT,
                    *mut POINT,
                    *mut ELEMENT,
                    *mut POINT,
                ) -> (),
        ) as Option<ElementCollisionFunc>;
        (*CometPtr).pParent = StarShipPtr as *mut ::core::ffi::c_void;
        let mut facing: COUNT = 0;
        (*CometPtr).c2rust_unnamed_2.turn_wait = (*ElementPtr)
            .c2rust_unnamed_2
            .turn_wait;
        (*CometPtr).hTarget = (*ElementPtr).hTarget;
        if (*ElementPtr).state_flags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int != 0
        {
            (*CometPtr).c2rust_unnamed_2.turn_wait = 0 as BYTE;
            facing = TFB_Random() as COUNT;
            SetVelocityVector(
                &raw mut (*CometPtr).velocity,
                (if if if GlobData.SIS_state.Difficulty as ::core::ffi::c_int != 0 {
                    GlobData.SIS_state.Difficulty as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                } == 1 as ::core::ffi::c_int
                {
                    1 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                } == 0
                {
                    10 as SDWORD
                } else {
                    12 as SDWORD
                } << 2 as ::core::ffi::c_int) << resolutionFactor,
                facing,
            );
        } else {
            (*CometPtr).velocity = (*ElementPtr).velocity;
            (*CometPtr).c2rust_unnamed_0.hit_points = (*ElementPtr)
                .c2rust_unnamed_0
                .hit_points;
            facing = ((*CometPtr).velocity.TravelAngle as ::core::ffi::c_int
                + ((1 as ::core::ffi::c_int)
                    << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int)
                >> 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int) as COUNT;
        }
        if (*CometPtr).c2rust_unnamed_2.turn_wait != 0 {
            (*CometPtr).c2rust_unnamed_2.turn_wait = (*CometPtr)
                .c2rust_unnamed_2
                .turn_wait
                .wrapping_sub(1);
        } else {
            facing = (facing as ::core::ffi::c_int
                & ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int) as DWORD as COUNT;
            if TrackShip(CometPtr, &raw mut facing) as ::core::ffi::c_int
                > 0 as ::core::ffi::c_int
            {
                SetVelocityVector(
                    &raw mut (*CometPtr).velocity,
                    (if if if GlobData.SIS_state.Difficulty as ::core::ffi::c_int != 0 {
                        GlobData.SIS_state.Difficulty as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    } == 1 as ::core::ffi::c_int
                    {
                        1 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    } == 0
                    {
                        10 as SDWORD
                    } else {
                        12 as SDWORD
                    } << 2 as ::core::ffi::c_int) << resolutionFactor,
                    facing,
                );
            }
            (*CometPtr).c2rust_unnamed_2.turn_wait = 3 as BYTE;
        }
        UnlockLink(&raw mut disp_q, hComet);
    }
    return hComet;
}
unsafe extern "C" fn turret_preprocess(mut ElementPtr: *mut ELEMENT) {
    if (*ElementPtr).c2rust_unnamed_2.turn_wait as ::core::ffi::c_int
        > 0 as ::core::ffi::c_int
    {
        (*ElementPtr).c2rust_unnamed_2.turn_wait = (*ElementPtr)
            .c2rust_unnamed_2
            .turn_wait
            .wrapping_sub(1);
    } else {
        (*ElementPtr).next.image.frame = SetAbsFrameIndex(
            (*ElementPtr).current.image.frame,
            (GetFrameIndex((*ElementPtr).current.image.frame) as ::core::ffi::c_int
                % 10 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as COUNT,
        );
        (*ElementPtr).state_flags = ((*ElementPtr).state_flags as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as ELEMENT_FLAGS;
        (*ElementPtr).c2rust_unnamed_2.turn_wait = 0 as BYTE;
    };
}
unsafe extern "C" fn gate_collision(
    mut ElementPtr0: *mut ELEMENT,
    mut pPt0: *mut POINT,
    mut ElementPtr1: *mut ELEMENT,
    mut pPt1: *mut POINT,
) {
    if (*ElementPtr1).playerNr as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
        StarShipPtr = (*ElementPtr0).pParent as *mut STARSHIP;
        if (*(*StarShipPtr).RaceDescPtr).characteristics.max_thrust as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
            if (*ElementPtr1).state_flags as ::core::ffi::c_int
                & (1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int == 0
            {
                (*ElementPtr0).state_flags = ((*ElementPtr0).state_flags
                    as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int)
                    as ELEMENT_FLAGS;
            }
            if (*ElementPtr1).state_flags as ::core::ffi::c_int
                & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int != 0
                && (*GlobData
                    .Game_state
                    .DisplayArray
                    .offset((*ElementPtr0).PrimIndex as isize))
                    .Type as ::core::ffi::c_int
                    == gfx_object::STAMPFILL_PRIM.0 as ::core::ffi::c_int
                && uqm_lua_checked!(getGameStateUint(
                    b"BOMB_CARRIER\0".as_ptr() as *const ::core::ffi::c_char,
                ), ()) != 0
            {
                GlobData.Game_state.CurrentActivity = (GlobData
                    .Game_state
                    .CurrentActivity as ::core::ffi::c_int
                    & !(C2Rust_Unnamed_5::IN_BATTLE.0 as ::core::ffi::c_int))
                    as ACTIVITY;
            }
        } else {
            let mut hBlastElement: HELEMENT = ::core::ptr::null_mut::<
                ::core::ffi::c_void,
            >();
            if (*ElementPtr1).state_flags as ::core::ffi::c_int
                & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int != 0
            {
                (*ElementPtr0).c2rust_unnamed_1.mass_points = 1 as BYTE;
            } else {
                (*ElementPtr0).c2rust_unnamed_1.mass_points = 50 as BYTE;
            }
            (*ElementPtr0).c2rust_unnamed_0.hit_points = 100 as COUNT;
            hBlastElement = uqm_lua_checked!(weapon_collision(ElementPtr0, pPt0, ElementPtr1, pPt1), ());
            (*ElementPtr0).state_flags = ((*ElementPtr0).state_flags
                as ::core::ffi::c_int
                & !((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int))
                as ELEMENT_FLAGS;
            (*ElementPtr0).c2rust_unnamed.life_span = 1 as COUNT;
            if !hBlastElement.is_null() {
                RemoveElement(hBlastElement);
                FreeElement(hBlastElement);
            }
        }
    }
}
unsafe extern "C" fn gate_preprocess(mut ElementPtr: *mut ELEMENT) {
    let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
    StarShipPtr = (*ElementPtr).pParent as *mut STARSHIP;
    if (*(*StarShipPtr).RaceDescPtr).characteristics.max_thrust as ::core::ffi::c_int
        == 0 as ::core::ffi::c_int
    {
        (*ElementPtr).c2rust_unnamed_1.mass_points = (10 as ::core::ffi::c_int
            * 10 as ::core::ffi::c_int) as BYTE;
        (*ElementPtr).state_flags = ((*ElementPtr).state_flags as ::core::ffi::c_int
            & !((1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int)) as ELEMENT_FLAGS;
        (*ElementPtr).c2rust_unnamed.life_span = (1 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int) as COUNT;
        (*ElementPtr).preprocess_func = None;
        (*GlobData.Game_state.DisplayArray.offset((*ElementPtr).PrimIndex as isize))
            .color = colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE);
        (*GlobData.Game_state.DisplayArray.offset((*ElementPtr).PrimIndex as isize))
            .Type = gfx_object::STAMPFILL_PRIM.0 as ::core::ffi::c_int as GRAPHICS_PRIM;
    } else {
        (*ElementPtr).c2rust_unnamed.life_span = (*ElementPtr)
            .c2rust_unnamed
            .life_span
            .wrapping_add(1);
        (*ElementPtr).next.image.frame = IncFrameIndex(
            (*ElementPtr).current.image.frame,
        );
        if GetFrameIndex((*ElementPtr).next.image.frame) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
            (*ElementPtr).next.image.frame = SetAbsFrameIndex(
                (*ElementPtr).next.image.frame,
                11 as COUNT,
            );
        }
        (*ElementPtr).state_flags = ((*ElementPtr).state_flags as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as ELEMENT_FLAGS;
    };
}
unsafe extern "C" fn generator_death(mut ElementPtr: *mut ELEMENT) {
    if (*ElementPtr).state_flags as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int == 0
    {
        let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
        StarShipPtr = (*ElementPtr).pParent as *mut STARSHIP;
        (*(*StarShipPtr).RaceDescPtr).characteristics.max_thrust = (*(*StarShipPtr)
            .RaceDescPtr)
            .characteristics
            .max_thrust
            .wrapping_sub(1);
        (*ElementPtr).state_flags = ((*ElementPtr).state_flags as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int))
            as ELEMENT_FLAGS;
        (*ElementPtr).preprocess_func = None;
        (*ElementPtr).c2rust_unnamed_2.turn_wait = 12 as BYTE;
        (*ElementPtr).c2rust_unnamed_3.thrust_wait = 0 as BYTE;
        (*ElementPtr).current.image.frame = SetAbsFrameIndex(
            (*ElementPtr).current.image.frame,
            (10 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as COUNT,
        );
    }
    if (*ElementPtr).c2rust_unnamed_3.thrust_wait != 0 {
        (*ElementPtr).c2rust_unnamed_3.thrust_wait = (*ElementPtr)
            .c2rust_unnamed_3
            .thrust_wait
            .wrapping_sub(1);
        (*ElementPtr).state_flags = ((*ElementPtr).state_flags as ::core::ffi::c_int
            & !((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)) as ELEMENT_FLAGS;
        (*ElementPtr).state_flags = ((*ElementPtr).state_flags as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as ELEMENT_FLAGS;
        (*ElementPtr).c2rust_unnamed.life_span = (*ElementPtr)
            .c2rust_unnamed
            .life_span
            .wrapping_add(1);
    } else {
        let c2rust_fresh0 = (*ElementPtr).c2rust_unnamed_2.turn_wait;
        (*ElementPtr).c2rust_unnamed_2.turn_wait = (*ElementPtr)
            .c2rust_unnamed_2
            .turn_wait
            .wrapping_sub(1);
        if c2rust_fresh0 != 0 {
            (*ElementPtr).state_flags = ((*ElementPtr).state_flags as ::core::ffi::c_int
                & !((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int))
                as ELEMENT_FLAGS;
            (*ElementPtr).state_flags = ((*ElementPtr).state_flags as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as ELEMENT_FLAGS;
            (*ElementPtr).c2rust_unnamed.life_span = (*ElementPtr)
                .c2rust_unnamed
                .life_span
                .wrapping_add(1);
            (*ElementPtr).next.image.frame = IncFrameIndex(
                (*ElementPtr).current.image.frame,
            );
            (*ElementPtr).c2rust_unnamed_3.thrust_wait = 1 as BYTE;
        }
    };
}
unsafe extern "C" fn generator_preprocess(mut ElementPtr: *mut ELEMENT) {
    if (*ElementPtr).c2rust_unnamed_2.turn_wait as ::core::ffi::c_int
        > 0 as ::core::ffi::c_int
    {
        (*ElementPtr).c2rust_unnamed_2.turn_wait = (*ElementPtr)
            .c2rust_unnamed_2
            .turn_wait
            .wrapping_sub(1);
    } else {
        (*ElementPtr).c2rust_unnamed_2.turn_wait = ((if if GlobData.SIS_state.Difficulty
            as ::core::ffi::c_int != 0
        {
            GlobData.SIS_state.Difficulty as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        } == 0 as ::core::ffi::c_int
        {
            15 as ::core::ffi::c_int
        } else if if GlobData.SIS_state.Difficulty as ::core::ffi::c_int != 0 {
            GlobData.SIS_state.Difficulty as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        } == 1 as ::core::ffi::c_int
        {
            12 as ::core::ffi::c_int
        } else {
            18 as ::core::ffi::c_int
        } - (*ElementPtr).c2rust_unnamed_0.hit_points as ::core::ffi::c_int)
            / 5 as ::core::ffi::c_int) as BYTE;
        if ((*ElementPtr).c2rust_unnamed_2.turn_wait as ::core::ffi::c_int)
            < 3 as ::core::ffi::c_int
        {
            (*ElementPtr).next.image.frame = SetAbsFrameIndex(
                (*ElementPtr).current.image.frame,
                ((GetFrameIndex((*ElementPtr).current.image.frame) as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int) % 10 as ::core::ffi::c_int) as COUNT,
            );
            (*ElementPtr).state_flags = ((*ElementPtr).state_flags as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as ELEMENT_FLAGS;
        }
    };
}
unsafe extern "C" fn generator_collision(
    mut ElementPtr0: *mut ELEMENT,
    mut pPt0: *mut POINT,
    mut ElementPtr1: *mut ELEMENT,
    mut pPt1: *mut POINT,
) {
    if (*ElementPtr1).state_flags as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int == 0
    {
        (*ElementPtr0).state_flags = ((*ElementPtr0).state_flags as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as ELEMENT_FLAGS;
    }
}
unsafe extern "C" fn sentinel_preprocess(mut ElementPtr: *mut ELEMENT) {
    let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
    StarShipPtr = (*ElementPtr).pParent as *mut STARSHIP;
    (*(*StarShipPtr).RaceDescPtr).characteristics.special_wait = (*(*StarShipPtr)
        .RaceDescPtr)
        .characteristics
        .special_wait
        .wrapping_add(1);
    (*ElementPtr).c2rust_unnamed.life_span = (*ElementPtr)
        .c2rust_unnamed
        .life_span
        .wrapping_add(1);
    if (*ElementPtr).c2rust_unnamed_3.thrust_wait != 0 {
        (*ElementPtr).c2rust_unnamed_3.thrust_wait = (*ElementPtr)
            .c2rust_unnamed_3
            .thrust_wait
            .wrapping_sub(1);
    } else {
        (*ElementPtr).next.image.frame = SetAbsFrameIndex(
            (*ElementPtr).current.image.frame,
            ((GetFrameIndex((*ElementPtr).current.image.frame) as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int) % 6 as ::core::ffi::c_int) as COUNT,
        );
        (*ElementPtr).state_flags = ((*ElementPtr).state_flags as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as ELEMENT_FLAGS;
        (*ElementPtr).c2rust_unnamed_3.thrust_wait = 1 as BYTE;
    }
    if (*ElementPtr).c2rust_unnamed_2.turn_wait as ::core::ffi::c_int
        > 0 as ::core::ffi::c_int
    {
        (*ElementPtr).c2rust_unnamed_2.turn_wait = (*ElementPtr)
            .c2rust_unnamed_2
            .turn_wait
            .wrapping_sub(1);
    } else {
        let mut facing: COUNT = 0;
        let mut hTarget: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if (*ElementPtr).state_flags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int == 0
        {
            facing = ((*ElementPtr).velocity.TravelAngle as ::core::ffi::c_int
                + ((1 as ::core::ffi::c_int)
                    << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int)
                >> 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int) as COUNT;
        } else {
            (*ElementPtr).state_flags = ((*ElementPtr).state_flags as ::core::ffi::c_int
                & !((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int))
                as ELEMENT_FLAGS;
            facing = TFB_Random() as COUNT;
            SetVelocityVector(
                &raw mut (*ElementPtr).velocity,
                (8 as SDWORD) << resolutionFactor << 2 as ::core::ffi::c_int,
                facing,
            );
        }
        facing = (facing as ::core::ffi::c_int
            & ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                - 1 as ::core::ffi::c_int) as DWORD as COUNT;
        if (*ElementPtr).hTarget.is_null() {
            let mut f: COUNT = 0;
            f = facing;
            TrackShip(ElementPtr, &raw mut f);
        }
        if (*ElementPtr).hTarget.is_null() {
            hTarget = (*StarShipPtr).hShip;
        } else if (*StarShipPtr).hShip.is_null() {
            hTarget = (*ElementPtr).hTarget;
        } else {
            let mut delta_x0: SDWORD = 0;
            let mut delta_y0: SDWORD = 0;
            let mut delta_x1: SDWORD = 0;
            let mut delta_y1: SDWORD = 0;
            let mut ShipPtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
            let mut EnemyShipPtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
            EnemyShipPtr = LockLink(&raw mut disp_q, (*ElementPtr).hTarget)
                as *mut ELEMENT;
            ShipPtr = LockLink(&raw mut disp_q, (*StarShipPtr).hShip) as *mut ELEMENT;
            delta_x0 = (*ShipPtr).current.location.x as SDWORD
                - (*ElementPtr).current.location.x as SDWORD;
            delta_y0 = (*ShipPtr).current.location.y as SDWORD
                - (*ElementPtr).current.location.y as SDWORD;
            delta_x1 = (*ShipPtr).current.location.x as SDWORD
                - (*EnemyShipPtr).current.location.x as SDWORD;
            delta_y1 = (*ShipPtr).current.location.y as SDWORD
                - (*EnemyShipPtr).current.location.y as SDWORD;
            UnlockLink(&raw mut disp_q, (*StarShipPtr).hShip);
            if delta_x0 as ::core::ffi::c_long * delta_x0 as ::core::ffi::c_long
                + delta_y0 as ::core::ffi::c_long * delta_y0 as ::core::ffi::c_long
                > delta_x1 as ::core::ffi::c_long * delta_x1 as ::core::ffi::c_long
                    + delta_y1 as ::core::ffi::c_long * delta_y1 as ::core::ffi::c_long
            {
                hTarget = (*StarShipPtr).hShip;
            } else {
                hTarget = (*ElementPtr).hTarget;
            }
            UnlockLink(&raw mut disp_q, (*ElementPtr).hTarget);
        }
        if !hTarget.is_null() {
            let mut num_frames: COUNT = 0;
            let mut delta_x: SDWORD = 0;
            let mut delta_y: SDWORD = 0;
            let mut TargetPtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
            let mut TargetVelocity: VELOCITY_DESC = VELOCITY_DESC {
                TravelAngle: 0,
                vector: EXTENT { width: 0, height: 0 },
                fract: EXTENT { width: 0, height: 0 },
                error: EXTENT { width: 0, height: 0 },
                incr: EXTENT { width: 0, height: 0 },
            };
            TargetPtr = LockLink(&raw mut disp_q, hTarget) as *mut ELEMENT;
            delta_x = (*TargetPtr).current.location.x as SDWORD
                - (*ElementPtr).current.location.x as SDWORD;
            delta_x = (if delta_x < 0 as ::core::ffi::c_int {
                if -delta_x
                    <= ((CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
                        >> 1 as ::core::ffi::c_int
                {
                    delta_x
                } else {
                    (((CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
                        << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) + delta_x
                }
            } else if delta_x
                <= ((CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
                    >> 1 as ::core::ffi::c_int
            {
                delta_x
            } else {
                delta_x
                    - (((CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
            }) as SDWORD;
            delta_y = (*TargetPtr).current.location.y as SDWORD
                - (*ElementPtr).current.location.y as SDWORD;
            delta_y = (if delta_y < 0 as ::core::ffi::c_int {
                if -delta_y
                    <= ((CanvasHeight
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
                        >> 1 as ::core::ffi::c_int
                {
                    delta_y
                } else {
                    (((CanvasHeight
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
                        << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) + delta_y
                }
            } else if delta_y
                <= ((CanvasHeight
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
                    >> 1 as ::core::ffi::c_int
            {
                delta_y
            } else {
                delta_y
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
            }) as SDWORD;
            num_frames = (square_root(
                (delta_x as ::core::ffi::c_long * delta_x as ::core::ffi::c_long
                    + delta_y as ::core::ffi::c_long * delta_y as ::core::ffi::c_long)
                    as DWORD,
            ) as ::core::ffi::c_int >> 6 as ::core::ffi::c_int >> resolutionFactor)
                as COUNT;
            if num_frames as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                num_frames = 1 as COUNT;
            }
            TargetVelocity = (*TargetPtr).velocity;
            GetNextVelocityComponentsSdword(
                &raw mut TargetVelocity,
                &raw mut delta_x,
                &raw mut delta_y,
                num_frames as DWORD,
            );
            delta_x = (*TargetPtr).current.location.x as SDWORD + delta_x
                - (*ElementPtr).current.location.x as SDWORD;
            delta_x = (if delta_x < 0 as ::core::ffi::c_int {
                if -delta_x
                    <= ((CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
                        >> 1 as ::core::ffi::c_int
                {
                    delta_x
                } else {
                    (((CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
                        << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) + delta_x
                }
            } else if delta_x
                <= ((CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
                    >> 1 as ::core::ffi::c_int
            {
                delta_x
            } else {
                delta_x
                    - (((CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
            }) as SDWORD;
            delta_y = (*TargetPtr).current.location.y as SDWORD + delta_y
                - (*ElementPtr).current.location.y as SDWORD;
            delta_y = (if delta_y < 0 as ::core::ffi::c_int {
                if -delta_y
                    <= ((CanvasHeight
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
                        >> 1 as ::core::ffi::c_int
                {
                    delta_y
                } else {
                    (((CanvasHeight
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
                        << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) + delta_y
                }
            } else if delta_y
                <= ((CanvasHeight
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
                    >> 1 as ::core::ffi::c_int
            {
                delta_y
            } else {
                delta_y
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
            }) as SDWORD;
            UnlockLink(&raw mut disp_q, hTarget);
            delta_x = ((ARCTAN(delta_x, delta_y) as ::core::ffi::c_int
                + ((1 as ::core::ffi::c_int)
                    << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int)
                >> 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int)
                - facing as ::core::ffi::c_int
                & ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int) as DWORD as SDWORD;
            if delta_x > 0 as ::core::ffi::c_int {
                if delta_x
                    <= ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                        >> 1 as ::core::ffi::c_int)
                        + ((1 as ::core::ffi::c_int)
                            << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int)
                        >> 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                {
                    facing = facing.wrapping_add(1);
                } else {
                    facing = facing.wrapping_sub(1);
                }
            }
            SetVelocityVector(
                &raw mut (*ElementPtr).velocity,
                (8 as SDWORD) << resolutionFactor << 2 as ::core::ffi::c_int,
                facing,
            );
        }
        (*ElementPtr).c2rust_unnamed_2.turn_wait = 1 as BYTE;
    };
}
unsafe extern "C" fn sentinel_collision(
    mut ElementPtr0: *mut ELEMENT,
    mut pPt0: *mut POINT,
    mut ElementPtr1: *mut ELEMENT,
    mut pPt1: *mut POINT,
) {
    let mut angle: COUNT = 0;
    let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
    if (*ElementPtr1).playerNr as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
        if (*ElementPtr0).preprocess_func == (*ElementPtr1).preprocess_func
            && (*ElementPtr0).state_flags as ::core::ffi::c_int
                & (1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int == 0
            && ((*pPt0).x as ::core::ffi::c_int
                != (*ElementPtr0).IntersectControl.IntersectStamp.origin.x
                    as ::core::ffi::c_int
                || (*pPt0).y as ::core::ffi::c_int
                    != (*ElementPtr0).IntersectControl.IntersectStamp.origin.y
                        as ::core::ffi::c_int)
        {
            angle = ARCTAN(
                (*pPt0).x as SDWORD - (*pPt1).x as SDWORD,
                (*pPt0).y as SDWORD - (*pPt1).y as SDWORD,
            );
            SetVelocityComponents(
                &raw mut (*ElementPtr0).velocity,
                (*(&raw mut sinetab as *mut SDWORD)
                    .offset(
                        (angle as ::core::ffi::c_int
                            + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                                >> 2 as ::core::ffi::c_int)
                            & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                                - 1 as ::core::ffi::c_int) as DWORD as isize,
                    ) as ::core::ffi::c_long
                    * (((8 as ::core::ffi::c_int) << resolutionFactor
                        << 2 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int)
                        as ::core::ffi::c_long >> 14 as ::core::ffi::c_int) as SDWORD,
                (*(&raw mut sinetab as *mut SDWORD)
                    .offset(
                        (angle as ::core::ffi::c_int
                            & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                                - 1 as ::core::ffi::c_int) as DWORD as isize,
                    ) as ::core::ffi::c_long
                    * (((8 as ::core::ffi::c_int) << resolutionFactor
                        << 2 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int)
                        as ::core::ffi::c_long >> 14 as ::core::ffi::c_int) as SDWORD,
            );
            (*ElementPtr0).c2rust_unnamed_2.turn_wait = 1 as BYTE;
            (*ElementPtr0).state_flags = ((*ElementPtr0).state_flags
                as ::core::ffi::c_int
                | ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int))
                as ELEMENT_FLAGS;
        }
    } else {
        let mut old_hits: BYTE = 0;
        let mut old_life: COUNT = 0;
        let mut hBlastElement: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
        old_hits = (*ElementPtr0).c2rust_unnamed_0.hit_points as BYTE;
        old_life = (*ElementPtr0).c2rust_unnamed.life_span;
        (*ElementPtr0).c2rust_unnamed_3.blast_offset = 0 as BYTE;
        hBlastElement = uqm_lua_checked!(weapon_collision(ElementPtr0, pPt0, ElementPtr1, pPt1), ());
        (*ElementPtr0).c2rust_unnamed_3.thrust_wait = 0 as BYTE;
        if (*ElementPtr1).state_flags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int != 0
            && (*ElementPtr1).c2rust_unnamed_0.crew_level as ::core::ffi::c_int != 0
            && !((*ElementPtr1).c2rust_unnamed_1.mass_points as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int
                > 10 as ::core::ffi::c_int * 10 as ::core::ffi::c_int)
        {
            let mut cur_delta_x: SDWORD = 0;
            let mut cur_delta_y: SDWORD = 0;
            (*ElementPtr0).c2rust_unnamed.life_span = old_life;
            (*ElementPtr0).c2rust_unnamed_0.hit_points = old_hits as COUNT;
            (*ElementPtr0).state_flags = ((*ElementPtr0).state_flags
                as ::core::ffi::c_int
                & !((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int))
                as ELEMENT_FLAGS;
            (*ElementPtr0).state_flags = ((*ElementPtr0).state_flags
                as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int) as ELEMENT_FLAGS;
            (*ElementPtr0).c2rust_unnamed_2.turn_wait = (840 as ::core::ffi::c_int
                / (840 as ::core::ffi::c_int / 24 as ::core::ffi::c_int)
                >> 1 as ::core::ffi::c_int) as BYTE;
            StarShipPtr = (*ElementPtr1).pParent as *mut STARSHIP;
            (*StarShipPtr).cur_status_flags = ((*StarShipPtr).cur_status_flags
                as ::core::ffi::c_int
                & !((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int))
                as STATUS_FLAGS;
            if ((*ElementPtr1).c2rust_unnamed_2.turn_wait as ::core::ffi::c_int)
                < 1 as ::core::ffi::c_int
            {
                (*ElementPtr1).c2rust_unnamed_2.turn_wait = ((*ElementPtr1)
                    .c2rust_unnamed_2
                    .turn_wait as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as BYTE;
            }
            if ((*ElementPtr1).c2rust_unnamed_3.thrust_wait as ::core::ffi::c_int)
                < 3 as ::core::ffi::c_int
            {
                (*ElementPtr1).c2rust_unnamed_3.thrust_wait = ((*ElementPtr1)
                    .c2rust_unnamed_3
                    .thrust_wait as ::core::ffi::c_int + 3 as ::core::ffi::c_int)
                    as BYTE;
            }
            angle = (*ElementPtr0).velocity.TravelAngle;
            DeltaVelocityComponents(
                &raw mut (*ElementPtr1).velocity,
                (*(&raw mut sinetab as *mut SDWORD)
                    .offset(
                        (angle as ::core::ffi::c_int
                            + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                                >> 2 as ::core::ffi::c_int)
                            & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                                - 1 as ::core::ffi::c_int) as DWORD as isize,
                    ) as ::core::ffi::c_long
                    * ((if if if GlobData.SIS_state.Difficulty as ::core::ffi::c_int != 0
                    {
                        GlobData.SIS_state.Difficulty as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    } == 1 as ::core::ffi::c_int
                    {
                        1 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    } == 0
                    {
                        8 as ::core::ffi::c_int
                    } else {
                        10 as ::core::ffi::c_int
                    } << 2 as ::core::ffi::c_int) << resolutionFactor
                        << 5 as ::core::ffi::c_int) as ::core::ffi::c_long
                    >> 14 as ::core::ffi::c_int) as SDWORD,
                (*(&raw mut sinetab as *mut SDWORD)
                    .offset(
                        (angle as ::core::ffi::c_int
                            & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                                - 1 as ::core::ffi::c_int) as DWORD as isize,
                    ) as ::core::ffi::c_long
                    * ((if if if GlobData.SIS_state.Difficulty as ::core::ffi::c_int != 0
                    {
                        GlobData.SIS_state.Difficulty as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    } == 1 as ::core::ffi::c_int
                    {
                        1 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    } == 0
                    {
                        8 as ::core::ffi::c_int
                    } else {
                        10 as ::core::ffi::c_int
                    } << 2 as ::core::ffi::c_int) << resolutionFactor
                        << 5 as ::core::ffi::c_int) as ::core::ffi::c_long
                    >> 14 as ::core::ffi::c_int) as SDWORD,
            );
            GetCurrentVelocityComponentsSdword(
                &raw mut (*ElementPtr1).velocity,
                &raw mut cur_delta_x,
                &raw mut cur_delta_y,
            );
            if cur_delta_x as ::core::ffi::c_long * cur_delta_x as ::core::ffi::c_long
                + cur_delta_y as ::core::ffi::c_long * cur_delta_y as ::core::ffi::c_long
                > (((if if if GlobData.SIS_state.Difficulty as ::core::ffi::c_int != 0 {
                    GlobData.SIS_state.Difficulty as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                } == 1 as ::core::ffi::c_int
                {
                    1 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                } == 0
                {
                    8 as ::core::ffi::c_int
                } else {
                    10 as ::core::ffi::c_int
                } << 2 as ::core::ffi::c_int) << resolutionFactor
                    << 5 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int)
                    as ::core::ffi::c_long
                    * (((if if if GlobData.SIS_state.Difficulty as ::core::ffi::c_int
                        != 0
                    {
                        GlobData.SIS_state.Difficulty as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    } == 1 as ::core::ffi::c_int
                    {
                        1 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    } == 0
                    {
                        8 as ::core::ffi::c_int
                    } else {
                        10 as ::core::ffi::c_int
                    } << 2 as ::core::ffi::c_int) << resolutionFactor
                        << 5 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int)
                        as ::core::ffi::c_long
            {
                angle = ARCTAN(cur_delta_x, cur_delta_y);
                SetVelocityComponents(
                    &raw mut (*ElementPtr1).velocity,
                    (*(&raw mut sinetab as *mut SDWORD)
                        .offset(
                            (angle as ::core::ffi::c_int
                                + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                                    >> 2 as ::core::ffi::c_int)
                                & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                                    - 1 as ::core::ffi::c_int) as DWORD as isize,
                        ) as ::core::ffi::c_long
                        * (((if if if GlobData.SIS_state.Difficulty as ::core::ffi::c_int
                            != 0
                        {
                            GlobData.SIS_state.Difficulty as ::core::ffi::c_int
                        } else {
                            0 as ::core::ffi::c_int
                        } == 1 as ::core::ffi::c_int
                        {
                            1 as ::core::ffi::c_int
                        } else {
                            0 as ::core::ffi::c_int
                        } == 0
                        {
                            8 as ::core::ffi::c_int
                        } else {
                            10 as ::core::ffi::c_int
                        } << 2 as ::core::ffi::c_int) << resolutionFactor
                            << 5 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int)
                            as ::core::ffi::c_long >> 14 as ::core::ffi::c_int)
                        as SDWORD,
                    (*(&raw mut sinetab as *mut SDWORD)
                        .offset(
                            (angle as ::core::ffi::c_int
                                & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                                    - 1 as ::core::ffi::c_int) as DWORD as isize,
                        ) as ::core::ffi::c_long
                        * (((if if if GlobData.SIS_state.Difficulty as ::core::ffi::c_int
                            != 0
                        {
                            GlobData.SIS_state.Difficulty as ::core::ffi::c_int
                        } else {
                            0 as ::core::ffi::c_int
                        } == 1 as ::core::ffi::c_int
                        {
                            1 as ::core::ffi::c_int
                        } else {
                            0 as ::core::ffi::c_int
                        } == 0
                        {
                            8 as ::core::ffi::c_int
                        } else {
                            10 as ::core::ffi::c_int
                        } << 2 as ::core::ffi::c_int) << resolutionFactor
                            << 5 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int)
                            as ::core::ffi::c_long >> 14 as ::core::ffi::c_int) as SDWORD,
                );
            }
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
                if ((*(*StarShipPtr).RaceDescPtr).ship_info.energy_level
                    as ::core::ffi::c_int) < 8 as ::core::ffi::c_int
                {
                    uqm_lua_checked!(DeltaEnergy(
                        ElementPtr1,
                        -((*(*StarShipPtr).RaceDescPtr).ship_info.energy_level
                            as ::core::ffi::c_int) as SIZE,
                    ), ());
                } else {
                    uqm_lua_checked!(DeltaEnergy(ElementPtr1, -8 as SIZE), ());
                }
                if if GlobData.SIS_state.Extended as ::core::ffi::c_int != 0 {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    StarShipPtr = (*ElementPtr0).pParent as *mut STARSHIP;
                    ProcessSound(
                        SetAbsStringTableIndex(
                            (*(*StarShipPtr).RaceDescPtr).ship_data.ship_sounds,
                            1 as COUNT,
                        ),
                        ElementPtr0,
                    );
                }
            }
            memset(
                &raw mut (*ElementPtr0).velocity as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<VELOCITY_DESC>(),
            );
        }
        if (*ElementPtr0).state_flags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int != 0
        {
            StarShipPtr = (*ElementPtr0).pParent as *mut STARSHIP;
            (*(*StarShipPtr).RaceDescPtr).characteristics.special_wait = (*(*StarShipPtr)
                .RaceDescPtr)
                .characteristics
                .special_wait
                .wrapping_sub(1);
            if !hBlastElement.is_null() {
                let mut BlastElementPtr: *mut ELEMENT = ::core::ptr::null_mut::<
                    ELEMENT,
                >();
                BlastElementPtr = LockLink(&raw mut disp_q, hBlastElement)
                    as *mut ELEMENT;
                (*BlastElementPtr).c2rust_unnamed.life_span = 7 as COUNT;
                (*BlastElementPtr).current.image.frame = SetAbsFrameIndex(
                    *(*BlastElementPtr).current.image.farray.offset(0isize),
                    6 as COUNT,
                );
                UnlockLink(&raw mut disp_q, hBlastElement);
            }
        }
    };
}
unsafe extern "C" fn samatra_intelligence(
    mut ShipPtr: *mut ELEMENT,
    mut ObjectsOfConcern: *mut EVALUATE_DESC,
    mut ConcernCounter: COUNT,
) {
    uqm_lua_checked!(ship_intelligence(ShipPtr, ObjectsOfConcern, ConcernCounter), ());
}
unsafe extern "C" fn samatra_postprocess(mut ElementPtr: *mut ELEMENT) {
    let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
    StarShipPtr = (*ElementPtr).pParent as *mut STARSHIP;
    if (*(*StarShipPtr).RaceDescPtr).characteristics.max_thrust != 0 {
        if (*StarShipPtr).weapon_counter as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            && ((*(*StarShipPtr).RaceDescPtr).characteristics.weapon_wait
                as ::core::ffi::c_int)
                < if if GlobData.SIS_state.Difficulty as ::core::ffi::c_int != 0 {
                    GlobData.SIS_state.Difficulty as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                } == 0 as ::core::ffi::c_int
                {
                    3 as ::core::ffi::c_int
                } else if if GlobData.SIS_state.Difficulty as ::core::ffi::c_int != 0 {
                    GlobData.SIS_state.Difficulty as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                } == 1 as ::core::ffi::c_int
                {
                    2 as ::core::ffi::c_int
                } else {
                    4 as ::core::ffi::c_int
                } && !spawn_comet(ElementPtr).is_null()
        {
            (*StarShipPtr).weapon_counter = (840 as ::core::ffi::c_int
                / (840 as ::core::ffi::c_int / 24 as ::core::ffi::c_int)
                * 10 as ::core::ffi::c_int) as BYTE;
            if if GlobData.SIS_state.Extended as ::core::ffi::c_int != 0 {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                ProcessSound(
                    SetAbsStringTableIndex(
                        (*(*StarShipPtr).RaceDescPtr).ship_data.ship_sounds,
                        0 as COUNT,
                    ),
                    ElementPtr,
                );
            }
        }
        if (*StarShipPtr).special_counter as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
            && ((*(*StarShipPtr).RaceDescPtr).characteristics.special_wait
                as ::core::ffi::c_int)
                < if if GlobData.SIS_state.Difficulty as ::core::ffi::c_int != 0 {
                    GlobData.SIS_state.Difficulty as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                } == 0 as ::core::ffi::c_int
                {
                    4 as ::core::ffi::c_int
                } else if if GlobData.SIS_state.Difficulty as ::core::ffi::c_int != 0 {
                    GlobData.SIS_state.Difficulty as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                } == 1 as ::core::ffi::c_int
                {
                    4 as ::core::ffi::c_int
                } else {
                    5 as ::core::ffi::c_int
                }
        {
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
            let mut hSentinel: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
            MissileBlock.cx = (*ElementPtr).next.location.x;
            MissileBlock.cy = (*ElementPtr).next.location.y;
            MissileBlock.farray = &raw mut (*(*StarShipPtr).RaceDescPtr).ship_data.weapon
                as *mut FRAME;
            MissileBlock.face = 0 as COUNT;
            MissileBlock.index = 0 as COUNT;
            MissileBlock.sender = (*ElementPtr).playerNr;
            MissileBlock.flags = 0 as ELEMENT_FLAGS;
            MissileBlock.pixoffs = 0 as SIZE;
            MissileBlock.speed = ((8 as ::core::ffi::c_int) << resolutionFactor
                << 2 as ::core::ffi::c_int) as SIZE;
            MissileBlock.hit_points = (if if GlobData.SIS_state.Difficulty
                as ::core::ffi::c_int != 0
            {
                GlobData.SIS_state.Difficulty as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } == 0 as ::core::ffi::c_int
            {
                10 as ::core::ffi::c_int
            } else if if GlobData.SIS_state.Difficulty as ::core::ffi::c_int != 0 {
                GlobData.SIS_state.Difficulty as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } == 1 as ::core::ffi::c_int
            {
                9 as ::core::ffi::c_int
            } else {
                12 as ::core::ffi::c_int
            }) as SIZE;
            MissileBlock.damage = (if if GlobData.SIS_state.Difficulty
                as ::core::ffi::c_int != 0
            {
                GlobData.SIS_state.Difficulty as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } == 0 as ::core::ffi::c_int
            {
                1 as ::core::ffi::c_int
            } else if if GlobData.SIS_state.Difficulty as ::core::ffi::c_int != 0 {
                GlobData.SIS_state.Difficulty as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } == 1 as ::core::ffi::c_int
            {
                1 as ::core::ffi::c_int
            } else {
                2 as ::core::ffi::c_int
            }) as SIZE;
            MissileBlock.life = 2 as COUNT;
            MissileBlock.preprocess_func = Some(
                sentinel_preprocess as unsafe extern "C" fn(*mut ELEMENT) -> (),
            ) as Option<unsafe extern "C" fn(*mut ELEMENT) -> ()>;
            MissileBlock.blast_offs = 0 as SIZE;
            hSentinel = initialize_missile(&raw mut MissileBlock);
            if !hSentinel.is_null() {
                let mut SentinelPtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
                SentinelPtr = LockLink(&raw mut disp_q, hSentinel) as *mut ELEMENT;
                (*SentinelPtr).collision_func = Some(
                    sentinel_collision
                        as unsafe extern "C" fn(
                            *mut ELEMENT,
                            *mut POINT,
                            *mut ELEMENT,
                            *mut POINT,
                        ) -> (),
                ) as Option<ElementCollisionFunc>;
                (*SentinelPtr).c2rust_unnamed_2.turn_wait = (1 as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int) as BYTE;
                (*SentinelPtr).pParent = StarShipPtr as *mut ::core::ffi::c_void;
                UnlockLink(&raw mut disp_q, hSentinel);
                (*StarShipPtr).special_counter = (840 as ::core::ffi::c_int
                    / (840 as ::core::ffi::c_int / 24 as ::core::ffi::c_int)
                    * 3 as ::core::ffi::c_int) as BYTE;
                PutQueue(&raw mut disp_q, hSentinel);
            }
        }
    }
}
unsafe extern "C" fn samatra_preprocess(mut ElementPtr: *mut ELEMENT) {
    let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
    StarShipPtr = (*ElementPtr).pParent as *mut STARSHIP;
    (*(*StarShipPtr).RaceDescPtr).characteristics.weapon_wait = 0 as BYTE;
    (*(*StarShipPtr).RaceDescPtr).characteristics.special_wait = 0 as BYTE;
    if (*ElementPtr).state_flags as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int == 0
    {
        (*ElementPtr).c2rust_unnamed_2.turn_wait = (*ElementPtr)
            .c2rust_unnamed_2
            .turn_wait
            .wrapping_add(1);
        (*ElementPtr).c2rust_unnamed_3.thrust_wait = (*ElementPtr)
            .c2rust_unnamed_3
            .thrust_wait
            .wrapping_add(1);
    } else {
        let mut offs: *mut POINT = ::core::ptr::null_mut::<POINT>();
        let mut offs_orig: [POINT; 8] = [
            point {
                x: (-127 as ::core::ffi::c_int - 9 as ::core::ffi::c_int) as COORD,
                y: (-53 as ::core::ffi::c_int + 18 as ::core::ffi::c_int) as COORD,
            },
            point {
                x: (-38 as ::core::ffi::c_int - 9 as ::core::ffi::c_int) as COORD,
                y: (-88 as ::core::ffi::c_int + 18 as ::core::ffi::c_int) as COORD,
            },
            point {
                x: (44 as ::core::ffi::c_int - 9 as ::core::ffi::c_int) as COORD,
                y: (-85 as ::core::ffi::c_int + 18 as ::core::ffi::c_int) as COORD,
            },
            point {
                x: (127 as ::core::ffi::c_int - 9 as ::core::ffi::c_int) as COORD,
                y: (-60 as ::core::ffi::c_int + 18 as ::core::ffi::c_int) as COORD,
            },
            point {
                x: (124 as ::core::ffi::c_int - 9 as ::core::ffi::c_int) as COORD,
                y: (28 as ::core::ffi::c_int + 18 as ::core::ffi::c_int) as COORD,
            },
            point {
                x: (73 as ::core::ffi::c_int - 9 as ::core::ffi::c_int) as COORD,
                y: (61 as ::core::ffi::c_int + 18 as ::core::ffi::c_int) as COORD,
            },
            point {
                x: (-87 as ::core::ffi::c_int - 9 as ::core::ffi::c_int) as COORD,
                y: (58 as ::core::ffi::c_int + 18 as ::core::ffi::c_int) as COORD,
            },
            point {
                x: (-136 as ::core::ffi::c_int - 9 as ::core::ffi::c_int) as COORD,
                y: (29 as ::core::ffi::c_int + 18 as ::core::ffi::c_int) as COORD,
            },
        ];
        let mut offs_hd: [POINT; 8] = [
            point {
                x: (-544 as ::core::ffi::c_int + 44 as ::core::ffi::c_int) as COORD,
                y: (-140 as ::core::ffi::c_int - 16 as ::core::ffi::c_int) as COORD,
            },
            point {
                x: (-188 as ::core::ffi::c_int + 38 as ::core::ffi::c_int) as COORD,
                y: (-280 as ::core::ffi::c_int - 20 as ::core::ffi::c_int) as COORD,
            },
            point {
                x: (140 as ::core::ffi::c_int + 34 as ::core::ffi::c_int) as COORD,
                y: (-268 as ::core::ffi::c_int - 16 as ::core::ffi::c_int) as COORD,
            },
            point {
                x: (472 as ::core::ffi::c_int + 36 as ::core::ffi::c_int) as COORD,
                y: (-168 as ::core::ffi::c_int - 20 as ::core::ffi::c_int) as COORD,
            },
            point {
                x: (460 as ::core::ffi::c_int + 32 as ::core::ffi::c_int) as COORD,
                y: (184 as ::core::ffi::c_int - 20 as ::core::ffi::c_int) as COORD,
            },
            point {
                x: (256 as ::core::ffi::c_int + 44 as ::core::ffi::c_int) as COORD,
                y: (316 as ::core::ffi::c_int - 24 as ::core::ffi::c_int) as COORD,
            },
            point {
                x: (-384 as ::core::ffi::c_int + 44 as ::core::ffi::c_int) as COORD,
                y: (304 as ::core::ffi::c_int - 12 as ::core::ffi::c_int) as COORD,
            },
            point {
                x: (-580 as ::core::ffi::c_int + 32 as ::core::ffi::c_int) as COORD,
                y: (188 as ::core::ffi::c_int - 24 as ::core::ffi::c_int) as COORD,
            },
        ];
        offs = if if resolutionFactor != 2 as ::core::ffi::c_uint {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } == 0
        {
            &raw mut offs_orig as *mut POINT
        } else {
            &raw mut offs_hd as *mut POINT
        };
        (*(*StarShipPtr).RaceDescPtr).characteristics.max_thrust = 0 as COUNT;
        while ((*(*StarShipPtr).RaceDescPtr).characteristics.max_thrust
            as ::core::ffi::c_int) < 8 as ::core::ffi::c_int
        {
            let mut hGenerator: HELEMENT = ::core::ptr::null_mut::<
                ::core::ffi::c_void,
            >();
            hGenerator = AllocElement();
            if !hGenerator.is_null() {
                let mut GeneratorPtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
                GeneratorPtr = LockLink(&raw mut disp_q, hGenerator) as *mut ELEMENT;
                (*GeneratorPtr).c2rust_unnamed_0.hit_points = (if if GlobData
                    .SIS_state
                    .Difficulty as ::core::ffi::c_int != 0
                {
                    GlobData.SIS_state.Difficulty as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                } == 0 as ::core::ffi::c_int
                {
                    15 as ::core::ffi::c_int
                } else if if GlobData.SIS_state.Difficulty as ::core::ffi::c_int != 0 {
                    GlobData.SIS_state.Difficulty as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                } == 1 as ::core::ffi::c_int
                {
                    12 as ::core::ffi::c_int
                } else {
                    18 as ::core::ffi::c_int
                }) as COUNT;
                (*GeneratorPtr).c2rust_unnamed_1.mass_points = (10 as ::core::ffi::c_int
                    * 10 as ::core::ffi::c_int) as BYTE;
                (*GeneratorPtr).c2rust_unnamed.life_span = 1 as COUNT;
                (*GeneratorPtr).playerNr = (*ElementPtr).playerNr;
                (*GeneratorPtr).state_flags = ((1 as ::core::ffi::c_int)
                    << 3 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
                    as ELEMENT_FLAGS;
                (*GlobData
                    .Game_state
                    .DisplayArray
                    .offset((*GeneratorPtr).PrimIndex as isize))
                    .Type = gfx_object::STAMP_PRIM.0 as ::core::ffi::c_int
                    as GRAPHICS_PRIM;
                (*GeneratorPtr).current.location.x = ((((CanvasWidth
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
                    << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    >> 1 as ::core::ffi::c_int)
                    + (((*offs
                        .offset(
                            (*(*StarShipPtr).RaceDescPtr).characteristics.max_thrust
                                as isize,
                        ))
                        .x as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                    & !((((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                        << 2 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)) as COORD;
                (*GeneratorPtr).current.location.y = ((((CanvasHeight
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
                    >> 1 as ::core::ffi::c_int)
                    + (((*offs
                        .offset(
                            (*(*StarShipPtr).RaceDescPtr).characteristics.max_thrust
                                as isize,
                        ))
                        .y as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                    & !((((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                        << 2 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)) as COORD;
                (*GeneratorPtr).current.image.farray = &raw mut (*(*StarShipPtr)
                    .RaceDescPtr)
                    .ship_data
                    .special as *mut FRAME;
                (*GeneratorPtr).current.image.frame = SetAbsFrameIndex(
                    (*(*StarShipPtr).RaceDescPtr).ship_data.special[0usize],
                    (TFB_Random() as BYTE as ::core::ffi::c_int
                        % 10 as ::core::ffi::c_int) as COUNT,
                );
                (*GeneratorPtr).preprocess_func = Some(
                    generator_preprocess as unsafe extern "C" fn(*mut ELEMENT) -> (),
                ) as Option<ElementProcessFunc>;
                (*GeneratorPtr).collision_func = Some(
                    generator_collision
                        as unsafe extern "C" fn(
                            *mut ELEMENT,
                            *mut POINT,
                            *mut ELEMENT,
                            *mut POINT,
                        ) -> (),
                ) as Option<ElementCollisionFunc>;
                (*GeneratorPtr).death_func = Some(
                    generator_death as unsafe extern "C" fn(*mut ELEMENT) -> (),
                ) as Option<ElementProcessFunc>;
                (*GeneratorPtr).pParent = StarShipPtr as *mut ::core::ffi::c_void;
                UnlockLink(&raw mut disp_q, hGenerator);
                InsertQueue(&raw mut disp_q, hGenerator, disp_q.head);
            }
            (*(*StarShipPtr).RaceDescPtr).characteristics.max_thrust = (*(*StarShipPtr)
                .RaceDescPtr)
                .characteristics
                .max_thrust
                .wrapping_add(1);
        }
        let mut hTurret: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
        hTurret = AllocElement();
        if !hTurret.is_null() {
            let mut TurretPtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
            TurretPtr = LockLink(&raw mut disp_q, hTurret) as *mut ELEMENT;
            (*TurretPtr).c2rust_unnamed_0.hit_points = 1 as COUNT;
            (*TurretPtr).c2rust_unnamed.life_span = 1 as COUNT;
            (*TurretPtr).playerNr = (*ElementPtr).playerNr;
            (*TurretPtr).state_flags = ((1 as ::core::ffi::c_int)
                << 3 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as ELEMENT_FLAGS;
            (*GlobData.Game_state.DisplayArray.offset((*TurretPtr).PrimIndex as isize))
                .Type = gfx_object::STAMP_PRIM.0 as ::core::ffi::c_int as GRAPHICS_PRIM;
            (*TurretPtr).current.location.x = (((CanvasWidth
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
                << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int) as COORD;
            (*TurretPtr).current.location.y = (((CanvasHeight
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
                >> 1 as ::core::ffi::c_int) as COORD;
            (*TurretPtr).current.image.farray = &raw mut (*(*StarShipPtr).RaceDescPtr)
                .ship_data
                .ship as *mut FRAME;
            (*TurretPtr).current.image.frame = SetAbsFrameIndex(
                (*(*StarShipPtr).RaceDescPtr).ship_data.ship[0usize],
                1 as COUNT,
            );
            (*TurretPtr).preprocess_func = Some(
                turret_preprocess as unsafe extern "C" fn(*mut ELEMENT) -> (),
            ) as Option<ElementProcessFunc>;
            (*TurretPtr).pParent = StarShipPtr as *mut ::core::ffi::c_void;
            UnlockLink(&raw mut disp_q, hTurret);
            InsertQueue(&raw mut disp_q, hTurret, (*ElementPtr).succ);
        }
        let mut hGate: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
        hGate = AllocElement();
        if !hGate.is_null() {
            let mut GatePtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
            GatePtr = LockLink(&raw mut disp_q, hGate) as *mut ELEMENT;
            (*GatePtr).c2rust_unnamed_0.hit_points = 100 as COUNT;
            (*GatePtr).c2rust_unnamed_1.mass_points = 1 as BYTE;
            (*GatePtr).c2rust_unnamed.life_span = 2 as COUNT;
            (*GatePtr).playerNr = (*ElementPtr).playerNr;
            (*GatePtr).state_flags = ((1 as ::core::ffi::c_int)
                << 3 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ELEMENT_FLAGS;
            (*GlobData.Game_state.DisplayArray.offset((*GatePtr).PrimIndex as isize))
                .Type = gfx_object::STAMP_PRIM.0 as ::core::ffi::c_int as GRAPHICS_PRIM;
            (*GatePtr).current.location.x = (((CanvasWidth
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
                << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int) as COORD;
            (*GatePtr).current.location.y = (((CanvasHeight
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
                >> 1 as ::core::ffi::c_int) as COORD;
            (*GatePtr).current.image.farray = &raw mut (*(*StarShipPtr).RaceDescPtr)
                .ship_data
                .ship as *mut FRAME;
            (*GatePtr).current.image.frame = SetAbsFrameIndex(
                (*(*StarShipPtr).RaceDescPtr).ship_data.ship[0usize],
                11 as COUNT,
            );
            (*GatePtr).preprocess_func = Some(
                gate_preprocess as unsafe extern "C" fn(*mut ELEMENT) -> (),
            ) as Option<ElementProcessFunc>;
            (*GatePtr).collision_func = Some(
                gate_collision
                    as unsafe extern "C" fn(
                        *mut ELEMENT,
                        *mut POINT,
                        *mut ELEMENT,
                        *mut POINT,
                    ) -> (),
            ) as Option<ElementCollisionFunc>;
            (*GatePtr).pParent = StarShipPtr as *mut ::core::ffi::c_void;
            UnlockLink(&raw mut disp_q, hGate);
            InsertQueue(&raw mut disp_q, hGate, (*ElementPtr).succ);
        }
        (*StarShipPtr).weapon_counter = (840 as ::core::ffi::c_int
            / (840 as ::core::ffi::c_int / 24 as ::core::ffi::c_int)
            * 10 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as BYTE;
        (*StarShipPtr).special_counter = (840 as ::core::ffi::c_int
            / (840 as ::core::ffi::c_int / 24 as ::core::ffi::c_int)
            * 3 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as BYTE;
    };
}
#[no_mangle]
pub unsafe extern "C" fn init_samatra() -> *mut RACE_DESC {
    let mut RaceDescPtr: *mut RACE_DESC = ::core::ptr::null_mut::<RACE_DESC>();
    samatra_desc.preprocess_func = Some(
        samatra_preprocess as unsafe extern "C" fn(*mut ELEMENT) -> (),
    ) as Option<PREPROCESS_FUNC>;
    samatra_desc.postprocess_func = Some(
        samatra_postprocess as unsafe extern "C" fn(*mut ELEMENT) -> (),
    ) as Option<POSTPROCESS_FUNC>;
    samatra_desc.cyborg_control.intelligence_func = Some(
        samatra_intelligence
            as unsafe extern "C" fn(*mut ELEMENT, *mut EVALUATE_DESC, COUNT) -> (),
    ) as Option<IntelligenceFunc>;
    RaceDescPtr = &raw mut samatra_desc;
    return RaceDescPtr;
}
