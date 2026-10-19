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

// Rust translation of src/uqm/ships/orz/orz.c.
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
    fn PutQueue(pq: *mut QUEUE, hLink: HLINK);
    fn InsertQueue(pq: *mut QUEUE, hLink: HLINK, hRefLink: HLINK);
    fn GetFrameCount(Frame: FRAME) -> COUNT;
    fn SetAbsFrameIndex(Frame: FRAME, FrameIndex: COUNT) -> FRAME;
    fn IncFrameIndex(Frame: FRAME) -> FRAME;
    fn SetAbsStringTableIndex(String: STRING, StringTableIndex: COUNT) -> STRING;
    static mut ScreenWidth: ::core::ffi::c_int;
    static mut ScreenHeight: ::core::ffi::c_int;
    static mut sinetab: [SIZE; 0];
    fn ARCTAN(delta_x: SIZE, delta_y: SIZE) -> COUNT;
    fn GetNextVelocityComponents(
        velocityptr: *mut VELOCITY_DESC,
        pdx: *mut SIZE,
        pdy: *mut SIZE,
        num_frames: COUNT,
    );
    static mut disp_q: QUEUE;
    fn AllocElement() -> HELEMENT;
    fn square_root(value: DWORD) -> COUNT;
    fn ship_intelligence(
        ShipPtr: *mut ELEMENT,
        ObjectsOfConcern: *mut EVALUATE_DESC,
        ConcernCounter: COUNT,
    );
    fn DeltaCrew(ElementPtr: *mut ELEMENT, crew_delta: SIZE) -> BOOLEAN;
    fn ProcessSound(Sound: SOUND, PositionalObject: *mut ELEMENT);
    fn initialize_missile(pMissileBlock: *mut MISSILE_BLOCK) -> HELEMENT;
    fn weapon_collision(
        ElementPtr0: *mut ELEMENT,
        pPt0: *mut POINT,
        ElementPtr1: *mut ELEMENT,
        pPt1: *mut POINT,
    ) -> HELEMENT;
    fn TrackShip(Tracker: *mut ELEMENT, pfacing: *mut COUNT) -> SIZE;
    fn ModifySilhouette(
        ElementPtr: *mut ELEMENT,
        modify_stamp: *mut STAMP,
        modify_flags: BYTE,
    ) -> FRAME;
    fn inertial_thrust(ElementPtr: *mut ELEMENT) -> STATUS_FLAGS;
    fn TFB_Random() -> DWORD;
    static mut GlobData: GLOBDATA;
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
unsafe extern "C" fn elementsOfSamePlayer(
    mut ElementPtr0: *mut ELEMENT,
    mut ElementPtr1: *mut ELEMENT,
) -> BOOLEAN {
    return BOOLEAN(
        ((*ElementPtr0).playerNr as ::core::ffi::c_int
            == (*ElementPtr1).playerNr as ::core::ffi::c_int) as ::core::ffi::c_int
            as ::core::ffi::c_uint,
    );
}
static mut orz_desc: RACE_DESC = unsafe {
    race_desc {
        c2rust_padding: [0; 12],
        c2rust_padding_0: [0; 10],
        c2rust_padding_1: [0; 2],
        ship_info: SHIP_INFO {
            ship_flags: ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as UWORD,
            ship_cost: 23 as BYTE,
            crew_level: 16 as COUNT,
            max_crew: 16 as COUNT,
            energy_level: 20 as BYTE,
            max_energy: 20 as BYTE,
            race_strings_rsc: b"ship.orz.text\0".as_ptr() as RESOURCE,
            icons_rsc: b"ship.orz.icons\0".as_ptr() as RESOURCE,
            melee_icon_rsc: b"ship.orz.meleeicons\0".as_ptr() as RESOURCE,
            race_strings: ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>(),
            icons: ::core::ptr::null_mut::<FRAME_DESC>(),
            melee_icon: ::core::ptr::null_mut::<FRAME_DESC>(),
        },
        fleet: FLEET_STUFF {
            strength: (333 as ::core::ffi::c_int / 11 as ::core::ffi::c_int
                * 2 as ::core::ffi::c_int) as COUNT,
            known_loc: point {
                x: 3608 as COORD,
                y: 2637 as COORD,
            },
        },
        characteristics: CHARACTERISTIC_STUFF {
            max_thrust: 35 as COUNT,
            thrust_increment: 5 as COUNT,
            energy_regeneration: 1 as BYTE,
            weapon_energy_cost: (20 as ::core::ffi::c_int / 3 as ::core::ffi::c_int)
                as BYTE,
            special_energy_cost: 0 as BYTE,
            energy_wait: 6 as BYTE,
            turn_wait: 1 as BYTE,
            thrust_wait: 0 as BYTE,
            weapon_wait: 4 as BYTE,
            special_wait: 12 as BYTE,
            ship_mass: 4 as BYTE,
        },
        ship_data: DATA_STUFF {
            ship_rsc: [
                b"ship.orz.graphics.nemesis.large\0".as_ptr() as RESOURCE,
                b"ship.orz.graphics.nemesis.medium\0".as_ptr() as RESOURCE,
                b"ship.orz.graphics.nemesis.small\0".as_ptr() as RESOURCE,
            ],
            weapon_rsc: [
                b"ship.orz.graphics.howitzer.large\0".as_ptr() as RESOURCE,
                b"ship.orz.graphics.howitzer.medium\0".as_ptr() as RESOURCE,
                b"ship.orz.graphics.howitzer.small\0".as_ptr() as RESOURCE,
            ],
            special_rsc: [
                b"ship.orz.graphics.turret.large\0".as_ptr() as RESOURCE,
                b"ship.orz.graphics.turret.medium\0".as_ptr() as RESOURCE,
                b"ship.orz.graphics.turret.small\0".as_ptr() as RESOURCE,
            ],
            captain_control: captain_stuff {
                captain_rsc: b"ship.orz.graphics.captain\0".as_ptr() as RESOURCE,
                background: ::core::ptr::null_mut::<FRAME_DESC>(),
                turn: ::core::ptr::null_mut::<FRAME_DESC>(),
                thrust: ::core::ptr::null_mut::<FRAME_DESC>(),
                weapon: ::core::ptr::null_mut::<FRAME_DESC>(),
                special: ::core::ptr::null_mut::<FRAME_DESC>(),
            },
            victory_ditty_rsc: b"ship.orz.ditty\0".as_ptr() as RESOURCE,
            ship_sounds_rsc: b"ship.orz.sounds\0".as_ptr() as RESOURCE,
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
            WeaponRange: (((30 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
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
unsafe extern "C" fn howitzer_collision(
    mut ElementPtr0: *mut ELEMENT,
    mut pPt0: *mut POINT,
    mut ElementPtr1: *mut ELEMENT,
    mut pPt1: *mut POINT,
) {
    if elementsOfSamePlayer(ElementPtr0, ElementPtr1).0 == 0 {
        weapon_collision(ElementPtr0, pPt0, ElementPtr1, pPt1);
    }
}
unsafe extern "C" fn initialize_turret_missile(
    mut ShipPtr: *mut ELEMENT,
    mut MissileArray: *mut HELEMENT,
) -> COUNT {
    let mut TurretPtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
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
    TurretPtr = LockLink(&raw mut disp_q, (*ShipPtr).succ) as *mut ELEMENT;
    if (*TurretPtr).c2rust_unnamed_2.turn_wait as ::core::ffi::c_int
        == 0 as ::core::ffi::c_int
        && (*StarShipPtr).cur_status_flags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int != 0
        && (*StarShipPtr).cur_status_flags as ::core::ffi::c_int
            & ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) != 0
    {
        if (*StarShipPtr).cur_status_flags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int != 0
        {
            (*TurretPtr).c2rust_unnamed_3.thrust_wait = (*TurretPtr)
                .c2rust_unnamed_3
                .thrust_wait
                .wrapping_add(1);
        } else {
            (*TurretPtr).c2rust_unnamed_3.thrust_wait = (*TurretPtr)
                .c2rust_unnamed_3
                .thrust_wait
                .wrapping_sub(1);
        }
        (*TurretPtr).c2rust_unnamed_2.turn_wait = (3 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int) as BYTE;
    }
    MissileBlock.index = ((*StarShipPtr).ShipFacing as ::core::ffi::c_int
        + (*TurretPtr).c2rust_unnamed_3.thrust_wait as ::core::ffi::c_int
        & ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
            - 1 as ::core::ffi::c_int) as COUNT;
    MissileBlock.face = MissileBlock.index;
    UnlockLink(&raw mut disp_q, (*ShipPtr).succ);
    MissileBlock.sender = (*ShipPtr).playerNr;
    MissileBlock.flags = ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
        as ELEMENT_FLAGS;
    MissileBlock.pixoffs = 14 as SIZE;
    MissileBlock.speed = ((30 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as SIZE;
    MissileBlock.hit_points = 2 as SIZE;
    MissileBlock.damage = 3 as SIZE;
    MissileBlock.life = 12 as COUNT;
    MissileBlock.preprocess_func = None;
    MissileBlock.blast_offs = 1 as SIZE;
    *MissileArray.offset(0isize) = initialize_missile(&raw mut MissileBlock);
    if !(*MissileArray.offset(0isize)).is_null() {
        let mut HowitzerPtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
        HowitzerPtr = LockLink(&raw mut disp_q, *MissileArray.offset(0isize))
            as *mut ELEMENT;
        (*HowitzerPtr).collision_func = Some(
            howitzer_collision
                as unsafe extern "C" fn(
                    *mut ELEMENT,
                    *mut POINT,
                    *mut ELEMENT,
                    *mut POINT,
                ) -> (),
        ) as Option<ElementCollisionFunc>;
        UnlockLink(&raw mut disp_q, *MissileArray.offset(0isize));
    }
    return 1 as COUNT;
}
unsafe extern "C" fn count_marines(
    mut StarShipPtr: *mut STARSHIP,
    mut FindSpot: BOOLEAN,
) -> BYTE {
    let mut num_marines: BYTE = 0;
    let mut id_use: [BYTE; 8] = [0; 8];
    let mut hElement: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut hNextElement: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
    num_marines = 8 as BYTE;
    loop {
        let c2rust_fresh0 = num_marines;
        num_marines = num_marines.wrapping_sub(1);
        if c2rust_fresh0 == 0 {
            break;
        }
        id_use[num_marines as usize] = 0 as BYTE;
    }
    num_marines = 0 as BYTE;
    hElement = disp_q.tail as HELEMENT;
    while !hElement.is_null() {
        let mut ElementPtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
        ElementPtr = LockLink(&raw mut disp_q, hElement) as *mut ELEMENT;
        hNextElement = (*ElementPtr).pred;
        if (*ElementPtr).current.image.farray
            == &raw mut (*(*StarShipPtr).RaceDescPtr).ship_data.special as *mut FRAME
            && (*ElementPtr).c2rust_unnamed.life_span as ::core::ffi::c_int != 0
            && (*ElementPtr).state_flags as ::core::ffi::c_int
                & ((1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) == 0
        {
            if (*ElementPtr).state_flags as ::core::ffi::c_int
                & (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int != 0
            {
                id_use[(*ElementPtr).c2rust_unnamed_2.turn_wait as usize] = 1 as BYTE;
            }
            num_marines = num_marines.wrapping_add(1);
            if num_marines as ::core::ffi::c_int == 8 as ::core::ffi::c_int {
                UnlockLink(&raw mut disp_q, hElement);
                hNextElement = ::core::ptr::null_mut::<::core::ffi::c_void>();
            }
        }
        UnlockLink(&raw mut disp_q, hElement);
        hElement = hNextElement;
    }
    if FindSpot.0 != 0 {
        num_marines = 0 as BYTE;
        while id_use[num_marines as usize] != 0 {
            num_marines = num_marines.wrapping_add(1);
        }
    }
    return num_marines;
}
unsafe extern "C" fn orz_intelligence(
    mut ShipPtr: *mut ELEMENT,
    mut ObjectsOfConcern: *mut EVALUATE_DESC,
    mut ConcernCounter: COUNT,
) {
    let mut TurretPtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
    let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
    let mut lpEvalDesc: *mut EVALUATE_DESC = ::core::ptr::null_mut::<EVALUATE_DESC>();
    TurretPtr = LockLink(&raw mut disp_q, (*ShipPtr).succ) as *mut ELEMENT;
    (*TurretPtr).c2rust_unnamed_2.turn_wait = (*TurretPtr)
        .c2rust_unnamed_2
        .turn_wait
        .wrapping_add(1);
    ship_intelligence(ShipPtr, ObjectsOfConcern, ConcernCounter);
    (*TurretPtr).c2rust_unnamed_2.turn_wait = (*TurretPtr)
        .c2rust_unnamed_2
        .turn_wait
        .wrapping_sub(1);
    StarShipPtr = (*ShipPtr).pParent as *mut STARSHIP;
    lpEvalDesc = ObjectsOfConcern
        .offset(C2Rust_Unnamed_5::ENEMY_SHIP_INDEX.0 as ::core::ffi::c_int as isize);
    if (*lpEvalDesc).ObjectPtr.is_null() {
        (*StarShipPtr).ship_input_state = ((*StarShipPtr).ship_input_state
            as ::core::ffi::c_int
            & !((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)) as BYTE;
    } else if (*StarShipPtr).special_counter as ::core::ffi::c_int
        != 1 as ::core::ffi::c_int
    {
        let mut EnemyStarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
        if (*ShipPtr).c2rust_unnamed_2.turn_wait as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
            && (*lpEvalDesc).MoveState.0 == MOVEMENT_STATE::ENTICE.0
            && ((*lpEvalDesc).which_turn as ::core::ffi::c_int)
                < 24 as ::core::ffi::c_int
            && (*StarShipPtr).cur_status_flags as ::core::ffi::c_int
                & ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) != 0
            && (*StarShipPtr).ship_input_state as ::core::ffi::c_int
                & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int == 0
            && ((*ShipPtr).velocity.TravelAngle as ::core::ffi::c_int
                - ARCTAN(
                    ((*(*lpEvalDesc).ObjectPtr).next.location.x as ::core::ffi::c_int
                        - (*ShipPtr).next.location.x as ::core::ffi::c_int) as SIZE,
                    ((*(*lpEvalDesc).ObjectPtr).next.location.y as ::core::ffi::c_int
                        - (*ShipPtr).next.location.y as ::core::ffi::c_int) as SIZE,
                ) as ::core::ffi::c_int
                + (((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                    >> 2 as ::core::ffi::c_int)
                    - ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                        >> 3 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int))
                & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int) as COUNT as ::core::ffi::c_int
                >= ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                    >> 2 as ::core::ffi::c_int)
                    - ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                        >> 3 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
                    << 1 as ::core::ffi::c_int
        {
            (*StarShipPtr).ship_input_state = ((*StarShipPtr).ship_input_state
                as ::core::ffi::c_int
                & !((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)) as BYTE;
        }
        (*StarShipPtr).ship_input_state = ((*StarShipPtr).ship_input_state
            as ::core::ffi::c_int
            & !((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)) as BYTE;
        if (*ShipPtr).c2rust_unnamed_2.turn_wait as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
            && (*StarShipPtr).ship_input_state as ::core::ffi::c_int
                & ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) == 0
            && (*TurretPtr).c2rust_unnamed_2.turn_wait as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
        {
            let mut delta_facing: SIZE = 0;
            let mut facing: COUNT = 0;
            facing = ((*StarShipPtr).ShipFacing as ::core::ffi::c_int
                + (*TurretPtr).c2rust_unnamed_3.thrust_wait as ::core::ffi::c_int
                & ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int) as COUNT;
            delta_facing = TrackShip(TurretPtr, &raw mut facing);
            if delta_facing as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                (*StarShipPtr).ship_input_state = ((*StarShipPtr).ship_input_state
                    as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as BYTE;
                if delta_facing as ::core::ffi::c_int
                    == ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                        >> 1 as ::core::ffi::c_int)
                        + ((1 as ::core::ffi::c_int)
                            << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int)
                        >> 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                {
                    delta_facing = (delta_facing as ::core::ffi::c_int
                        + (((TFB_Random() as BYTE as ::core::ffi::c_int
                            & 1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
                            - 1 as ::core::ffi::c_int)) as SIZE;
                }
                if (delta_facing as ::core::ffi::c_int)
                    < ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                        >> 1 as ::core::ffi::c_int)
                        + ((1 as ::core::ffi::c_int)
                            << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int)
                        >> 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                {
                    (*StarShipPtr).ship_input_state = ((*StarShipPtr).ship_input_state
                        as ::core::ffi::c_int
                        | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as BYTE;
                } else {
                    (*StarShipPtr).ship_input_state = ((*StarShipPtr).ship_input_state
                        as ::core::ffi::c_int
                        | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as BYTE;
                }
            }
        }
        EnemyStarShipPtr = (*(*lpEvalDesc).ObjectPtr).pParent as *mut STARSHIP;
        if (*StarShipPtr).special_counter as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
            && (*StarShipPtr).ship_input_state as ::core::ffi::c_int
                & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int == 0
            && (*(*StarShipPtr).RaceDescPtr).ship_info.crew_level as ::core::ffi::c_int
                > ((*(*StarShipPtr).RaceDescPtr).ship_info.max_crew as ::core::ffi::c_int
                    >> 2 as ::core::ffi::c_int) as BYTE as ::core::ffi::c_int
            && (*(*EnemyStarShipPtr).RaceDescPtr).ship_info.ship_flags
                as ::core::ffi::c_int
                & (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int == 0
            && (((*(*EnemyStarShipPtr).RaceDescPtr).cyborg_control.ManeuverabilityIndex
                as ::core::ffi::c_int) < 25 as ::core::ffi::c_int
                || (*lpEvalDesc).which_turn as ::core::ffi::c_int
                    <= 12 as ::core::ffi::c_int
                || (count_marines(StarShipPtr, BOOLEAN::FALSE) as ::core::ffi::c_int)
                    < 2 as ::core::ffi::c_int)
        {
            (*StarShipPtr).ship_input_state = ((*StarShipPtr).ship_input_state
                as ::core::ffi::c_int
                | ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)) as BYTE;
        }
    }
    UnlockLink(&raw mut disp_q, (*ShipPtr).succ);
}
unsafe extern "C" fn ion_preprocess(mut ElementPtr: *mut ELEMENT) {
    static mut colorTable: [Color; 5] = [
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
            g: ((0xe as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0xe as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
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
            r: ((0x1b as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x1b as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
        Color {
            r: ((0x13 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x13 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
    ];
    let colorTabCount: size_t = ::core::mem::size_of::<[Color; 5]>()
        .wrapping_div(::core::mem::size_of::<Color>());
    (*ElementPtr).colorCycleIndex = (*ElementPtr).colorCycleIndex.wrapping_add(1);
    if (*ElementPtr).colorCycleIndex as size_t != colorTabCount {
        (*ElementPtr).c2rust_unnamed.life_span = (*ElementPtr)
            .c2rust_unnamed_3
            .thrust_wait as COUNT;
        (*GlobData.Game_state.DisplayArray.offset((*ElementPtr).PrimIndex as isize))
            .color = colorTable[(*ElementPtr).colorCycleIndex as usize];
        (*ElementPtr).state_flags = ((*ElementPtr).state_flags as ::core::ffi::c_int
            & !((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)) as ELEMENT_FLAGS;
        (*ElementPtr).state_flags = ((*ElementPtr).state_flags as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as ELEMENT_FLAGS;
    }
}
#[no_mangle]
pub unsafe extern "C" fn intruder_preprocess(mut ElementPtr: *mut ELEMENT) {
    let mut hElement: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut hNextElement: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut ShipPtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
    let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
    StarShipPtr = (*ElementPtr).pParent as *mut STARSHIP;
    ShipPtr = LockLink(&raw mut disp_q, (*StarShipPtr).hShip) as *mut ELEMENT;
    if (*ShipPtr).c2rust_unnamed_0.crew_level as ::core::ffi::c_int
        == 0 as ::core::ffi::c_int
        && (*ShipPtr).c2rust_unnamed.life_span as ::core::ffi::c_int
            == 1 as ::core::ffi::c_int
        && (*ShipPtr).state_flags as ::core::ffi::c_int
            & ((1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
            == (1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
    {
        (*ElementPtr).c2rust_unnamed.life_span = 0 as COUNT;
        (*ElementPtr).state_flags = ((*ElementPtr).state_flags as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as ELEMENT_FLAGS;
    }
    UnlockLink(&raw mut disp_q, (*StarShipPtr).hShip);
    if (*ElementPtr).c2rust_unnamed_3.thrust_wait != 0 {
        (*ElementPtr).c2rust_unnamed_3.thrust_wait = (*ElementPtr)
            .c2rust_unnamed_3
            .thrust_wait
            .wrapping_sub(1);
    }
    hElement = disp_q.head as HELEMENT;
    while !hElement.is_null() {
        ShipPtr = LockLink(&raw mut disp_q, hElement) as *mut ELEMENT;
        if (*ShipPtr).state_flags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int != 0
            && elementsOfSamePlayer(ShipPtr, ElementPtr).0 == 0
        {
            let mut s: STAMP = STAMP {
                origin: POINT { x: 0, y: 0 },
                frame: ::core::ptr::null_mut::<FRAME_DESC>(),
            };
            if (*ElementPtr).c2rust_unnamed_3.thrust_wait as ::core::ffi::c_int
                == 12 as ::core::ffi::c_int
            {
                (*ElementPtr).c2rust_unnamed_3.thrust_wait = (*ElementPtr)
                    .c2rust_unnamed_3
                    .thrust_wait
                    .wrapping_sub(1);
                s.origin.x = (16 as ::core::ffi::c_int
                    + ((*ElementPtr).c2rust_unnamed_2.turn_wait as ::core::ffi::c_int
                        & 3 as ::core::ffi::c_int) * 9 as ::core::ffi::c_int) as COORD;
                s.origin.y = (14 as ::core::ffi::c_int
                    + ((*ElementPtr).c2rust_unnamed_2.turn_wait as ::core::ffi::c_int
                        >> 2 as ::core::ffi::c_int) * 11 as ::core::ffi::c_int) as COORD;
                s.frame = SetAbsFrameIndex(
                    *(*ElementPtr).next.image.farray.offset(0isize),
                    (GetFrameCount(*(*ElementPtr).next.image.farray.offset(0isize))
                        as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as COUNT,
                );
                ModifySilhouette(ShipPtr, &raw mut s, 0 as BYTE);
            }
            (*ElementPtr).next.location = (*ShipPtr).next.location;
            's_186: {
                if (*ShipPtr).c2rust_unnamed_0.crew_level as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                    || (*ElementPtr).c2rust_unnamed.life_span as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                {
                    UnlockLink(&raw mut disp_q, hElement);
                    hElement = ::core::ptr::null_mut::<::core::ffi::c_void>();
                } else if (*ElementPtr).c2rust_unnamed_3.thrust_wait
                    as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                {
                    let mut randval: BYTE = 0;
                    (*ElementPtr).c2rust_unnamed_3.thrust_wait = 12 as BYTE;
                    randval = TFB_Random() as BYTE;
                    if (randval as ::core::ffi::c_int)
                        < 0x100 as ::core::ffi::c_int / 16 as ::core::ffi::c_int
                    {
                        (*ElementPtr).c2rust_unnamed.life_span = 0 as COUNT;
                        (*ElementPtr).state_flags = ((*ElementPtr).state_flags
                            as ::core::ffi::c_int
                            | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                            as ELEMENT_FLAGS;
                        ProcessSound(
                            SetAbsStringTableIndex(
                                (*(*StarShipPtr).RaceDescPtr).ship_data.ship_sounds,
                                4 as COUNT,
                            ),
                            ElementPtr,
                        );
                    } else {
                        if (randval as ::core::ffi::c_int)
                            < 0x100 as ::core::ffi::c_int / 2 as ::core::ffi::c_int
                                + 0x100 as ::core::ffi::c_int / 16 as ::core::ffi::c_int
                        {
                            if DeltaCrew(ShipPtr, -1 as SIZE).0 == 0 {
                                (*ShipPtr).c2rust_unnamed.life_span = 0 as COUNT;
                            }
                            (*ElementPtr).c2rust_unnamed_3.thrust_wait = (*ElementPtr)
                                .c2rust_unnamed_3
                                .thrust_wait
                                .wrapping_add(1);
                            s.origin.x = (16 as ::core::ffi::c_int
                                + ((*ElementPtr).c2rust_unnamed_2.turn_wait
                                    as ::core::ffi::c_int & 3 as ::core::ffi::c_int)
                                    * 9 as ::core::ffi::c_int) as COORD;
                            s.origin.y = (14 as ::core::ffi::c_int
                                + ((*ElementPtr).c2rust_unnamed_2.turn_wait
                                    as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
                                    * 11 as ::core::ffi::c_int) as COORD;
                            s.frame = SetAbsFrameIndex(
                                *(*ElementPtr).next.image.farray.offset(0isize),
                                (GetFrameCount(
                                    *(*ElementPtr).next.image.farray.offset(0isize),
                                ) as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as COUNT,
                            );
                            ModifySilhouette(ShipPtr, &raw mut s, 0 as BYTE);
                            ProcessSound(
                                SetAbsStringTableIndex(
                                    (*(*StarShipPtr).RaceDescPtr).ship_data.ship_sounds,
                                    3 as COUNT,
                                ),
                                ElementPtr,
                            );
                        }
                        break 's_186;
                    }
                } else {
                    break 's_186;
                }
                s.origin.x = (16 as ::core::ffi::c_int
                    + ((*ElementPtr).c2rust_unnamed_2.turn_wait as ::core::ffi::c_int
                        & 3 as ::core::ffi::c_int) * 9 as ::core::ffi::c_int) as COORD;
                s.origin.y = (14 as ::core::ffi::c_int
                    + ((*ElementPtr).c2rust_unnamed_2.turn_wait as ::core::ffi::c_int
                        >> 2 as ::core::ffi::c_int) * 11 as ::core::ffi::c_int) as COORD;
                s.frame = (*ElementPtr).next.image.frame;
                ModifySilhouette(
                    ShipPtr,
                    &raw mut s,
                    ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as BYTE,
                );
            }
            UnlockLink(&raw mut disp_q, hElement);
            break;
        } else {
            hNextElement = (*ShipPtr).succ;
            UnlockLink(&raw mut disp_q, hElement);
            hElement = hNextElement;
        }
    }
    if hElement.is_null()
        && (*ElementPtr).c2rust_unnamed.life_span as ::core::ffi::c_int != 0
    {
        (*ElementPtr).state_flags = ((*ElementPtr).state_flags as ::core::ffi::c_int
            & !((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)) as ELEMENT_FLAGS;
        (*ElementPtr).state_flags = ((*ElementPtr).state_flags as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int))
            as ELEMENT_FLAGS;
        (*GlobData.Game_state.DisplayArray.offset((*ElementPtr).PrimIndex as isize))
            .Type = gfx_object::STAMP_PRIM.0 as ::core::ffi::c_int as GRAPHICS_PRIM;
        (*ElementPtr).next.image.frame = SetAbsFrameIndex(
            (*(*StarShipPtr).RaceDescPtr).ship_data.special[0usize],
            21 as COUNT,
        );
        (*ElementPtr).current.image.frame = (*ElementPtr).next.image.frame;
        (*ElementPtr).c2rust_unnamed_3.thrust_wait = 0 as BYTE;
        (*ElementPtr).c2rust_unnamed_2.turn_wait = (((TFB_Random() as BYTE
            as ::core::ffi::c_int
            & ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                - 1 as ::core::ffi::c_int) as COUNT as BYTE as ::core::ffi::c_int)
            << 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int
            | 0 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE;
        (*ElementPtr).preprocess_func = Some(
            marine_preprocess as unsafe extern "C" fn(*mut ELEMENT) -> (),
        ) as Option<ElementProcessFunc>;
    }
}
unsafe extern "C" fn spawn_marine_ion_trail(
    mut ElementPtr: *mut ELEMENT,
    mut StarShipPtr: *mut STARSHIP,
    mut facing: COUNT,
) {
    let mut hIonElement: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
    hIonElement = AllocElement();
    if !hIonElement.is_null() {
        let mut angle: COUNT = 0;
        let mut IonElementPtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
        angle = (((facing as ::core::ffi::c_int)
            << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int)
            + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int)) as COUNT;
        InsertQueue(&raw mut disp_q, hIonElement, disp_q.head);
        IonElementPtr = LockLink(&raw mut disp_q, hIonElement) as *mut ELEMENT;
        (*IonElementPtr).playerNr = -1 as SIZE;
        (*IonElementPtr).state_flags = ((1 as ::core::ffi::c_int)
            << 3 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as ELEMENT_FLAGS;
        (*IonElementPtr).c2rust_unnamed_3.thrust_wait = 1 as BYTE;
        (*IonElementPtr).c2rust_unnamed.life_span = (*IonElementPtr)
            .c2rust_unnamed_3
            .thrust_wait as COUNT;
        (*GlobData.Game_state.DisplayArray.offset((*IonElementPtr).PrimIndex as isize))
            .Type = gfx_object::POINT_PRIM.0 as ::core::ffi::c_int as GRAPHICS_PRIM;
        (*GlobData.Game_state.DisplayArray.offset((*IonElementPtr).PrimIndex as isize))
            .color = colorFromRgb15(0x1f as BYTE, 0x15 as BYTE, 0 as BYTE);
        (*IonElementPtr).colorCycleIndex = 0 as BYTE;
        (*IonElementPtr).current.location = (*ElementPtr).current.location;
        (*IonElementPtr).current.location.x = ((*IonElementPtr).current.location.x
            as ::core::ffi::c_int
            + (*(&raw mut sinetab as *mut SIZE)
                .offset(
                    (angle as ::core::ffi::c_int
                        + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                            >> 2 as ::core::ffi::c_int)
                        & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                            - 1 as ::core::ffi::c_int) as COUNT as isize,
                ) as ::core::ffi::c_long
                * ((2 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                    as ::core::ffi::c_long >> 14 as ::core::ffi::c_int) as SIZE
                as ::core::ffi::c_int) as COORD;
        (*IonElementPtr).current.location.y = ((*IonElementPtr).current.location.y
            as ::core::ffi::c_int
            + (*(&raw mut sinetab as *mut SIZE)
                .offset(
                    (angle as ::core::ffi::c_int
                        & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                            - 1 as ::core::ffi::c_int) as COUNT as isize,
                ) as ::core::ffi::c_long
                * ((2 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                    as ::core::ffi::c_long >> 14 as ::core::ffi::c_int) as SIZE
                as ::core::ffi::c_int) as COORD;
        (*IonElementPtr).death_func = Some(
            ion_preprocess as unsafe extern "C" fn(*mut ELEMENT) -> (),
        ) as Option<ElementProcessFunc>;
        (*IonElementPtr).pParent = StarShipPtr as *mut ::core::ffi::c_void;
        (*IonElementPtr).next = (*IonElementPtr).current;
        (*IonElementPtr).c2rust_unnamed.life_span = (*IonElementPtr)
            .c2rust_unnamed
            .life_span
            .wrapping_sub(1);
        (*IonElementPtr).state_flags = ((*IonElementPtr).state_flags
            as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 11 as ::core::ffi::c_int) as ELEMENT_FLAGS;
        UnlockLink(&raw mut disp_q, hIonElement);
    }
}
unsafe extern "C" fn marine_preprocess(mut ElementPtr: *mut ELEMENT) {
    let mut ShipPtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
    let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
    StarShipPtr = (*ElementPtr).pParent as *mut STARSHIP;
    ShipPtr = LockLink(&raw mut disp_q, (*StarShipPtr).hShip) as *mut ELEMENT;
    if (*ShipPtr).c2rust_unnamed_0.crew_level as ::core::ffi::c_int
        == 0 as ::core::ffi::c_int
        && (*ShipPtr).c2rust_unnamed.life_span as ::core::ffi::c_int
            == 1 as ::core::ffi::c_int
        && (*ShipPtr).state_flags as ::core::ffi::c_int
            & ((1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
            == (1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
    {
        (*ElementPtr).c2rust_unnamed.life_span = 0 as COUNT;
        (*ElementPtr).state_flags = ((*ElementPtr).state_flags as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int))
            as ELEMENT_FLAGS;
        (*ElementPtr).c2rust_unnamed_2.turn_wait = 1 as BYTE;
    }
    UnlockLink(&raw mut disp_q, (*StarShipPtr).hShip);
    if ((*ElementPtr).c2rust_unnamed_2.turn_wait as ::core::ffi::c_int
        & 0xf as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE != 0
    {
        (*ElementPtr).c2rust_unnamed_2.turn_wait = (*ElementPtr)
            .c2rust_unnamed_2
            .turn_wait
            .wrapping_sub(1);
    } else {
        let mut facing: COUNT = 0;
        let mut pfacing: COUNT = 0 as COUNT;
        let mut delta_x: SIZE = 0;
        let mut delta_y: SIZE = 0;
        let mut delta_facing: SIZE = 0;
        let mut hObject: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
        let mut hNextObject: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
        let mut hTarget: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
        let mut ObjectPtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
        (*ElementPtr).c2rust_unnamed_3.thrust_wait = ((*ElementPtr)
            .c2rust_unnamed_3
            .thrust_wait as ::core::ffi::c_int
            & !((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
                >> 6 as ::core::ffi::c_int)) as BYTE;
        hTarget = ::core::ptr::null_mut::<::core::ffi::c_void>();
        hObject = disp_q.head as HELEMENT;
        while !hObject.is_null() {
            ObjectPtr = LockLink(&raw mut disp_q, hObject) as *mut ELEMENT;
            hNextObject = (*ObjectPtr).succ;
            if (*ObjectPtr).c2rust_unnamed_1.mass_points as ::core::ffi::c_int
                > 10 as ::core::ffi::c_int * 10 as ::core::ffi::c_int
            {
                delta_x = ((*ObjectPtr).current.location.x as ::core::ffi::c_int
                    - (*ElementPtr).current.location.x as ::core::ffi::c_int) as SIZE;
                delta_x = (if (delta_x as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                    if -(delta_x as ::core::ffi::c_int)
                        <= ((ScreenWidth - 64 as ::core::ffi::c_int
                            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                            << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                            >> 1 as ::core::ffi::c_int
                    {
                        delta_x as ::core::ffi::c_int
                    } else {
                        (((ScreenWidth - 64 as ::core::ffi::c_int
                            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                            << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                            + delta_x as ::core::ffi::c_int
                    }
                } else if delta_x as ::core::ffi::c_int
                    <= ((ScreenWidth - 64 as ::core::ffi::c_int
                        - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                        << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                        >> 1 as ::core::ffi::c_int
                {
                    delta_x as ::core::ffi::c_int
                } else {
                    delta_x as ::core::ffi::c_int
                        - (((ScreenWidth - 64 as ::core::ffi::c_int
                            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                            << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                }) as SIZE;
                delta_y = ((*ObjectPtr).current.location.y as ::core::ffi::c_int
                    - (*ElementPtr).current.location.y as ::core::ffi::c_int) as SIZE;
                delta_y = (if (delta_y as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                    if -(delta_y as ::core::ffi::c_int)
                        <= ((ScreenHeight
                            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                            << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                            >> 1 as ::core::ffi::c_int
                    {
                        delta_y as ::core::ffi::c_int
                    } else {
                        (((ScreenHeight
                            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                            << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                            + delta_y as ::core::ffi::c_int
                    }
                } else if delta_y as ::core::ffi::c_int
                    <= ((ScreenHeight
                        - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                        << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                        >> 1 as ::core::ffi::c_int
                {
                    delta_y as ::core::ffi::c_int
                } else {
                    delta_y as ::core::ffi::c_int
                        - (((ScreenHeight
                            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                            << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                }) as SIZE;
                if delta_x as ::core::ffi::c_long * delta_x as ::core::ffi::c_long
                    + delta_y as ::core::ffi::c_long * delta_y as ::core::ffi::c_long
                    <= (((255 as ::core::ffi::c_int as COUNT as ::core::ffi::c_int)
                        << 2 as ::core::ffi::c_int)
                        * ((255 as ::core::ffi::c_int as COUNT as ::core::ffi::c_int)
                            << 2 as ::core::ffi::c_int)) as ::core::ffi::c_long
                {
                    pfacing = (ARCTAN(delta_x, delta_y) as ::core::ffi::c_int
                        + ((1 as ::core::ffi::c_int)
                            << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int)
                        >> 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int) as COUNT;
                    delta_facing = (pfacing as ::core::ffi::c_int
                        - ((*ElementPtr).velocity.TravelAngle as ::core::ffi::c_int
                            + ((1 as ::core::ffi::c_int)
                                << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                                    - 1 as ::core::ffi::c_int)
                            >> 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int)
                        + (((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                            >> 3 as ::core::ffi::c_int)
                            + ((1 as ::core::ffi::c_int)
                                << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                                    - 1 as ::core::ffi::c_int)
                            >> 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int)
                        & ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                            - 1 as ::core::ffi::c_int) as COUNT as SIZE;
                    if delta_facing as ::core::ffi::c_int
                        <= ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                            >> 2 as ::core::ffi::c_int)
                            + ((1 as ::core::ffi::c_int)
                                << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                                    - 1 as ::core::ffi::c_int)
                            >> 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                    {
                        hTarget = hObject;
                        hNextObject = ::core::ptr::null_mut::<::core::ffi::c_void>();
                    }
                    (*ElementPtr).c2rust_unnamed_3.thrust_wait = ((*ElementPtr)
                        .c2rust_unnamed_3
                        .thrust_wait as ::core::ffi::c_int
                        | (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
                            >> 6 as ::core::ffi::c_int) as BYTE;
                }
            } else if (*ObjectPtr).state_flags as ::core::ffi::c_int
                & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int != 0
                && (*ObjectPtr).c2rust_unnamed_0.crew_level as ::core::ffi::c_int != 0
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
                if elementsOfSamePlayer(ObjectPtr, ElementPtr).0 == 0 {
                    if (*ElementPtr).state_flags as ::core::ffi::c_int
                        & (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int != 0
                    {
                        hTarget = hObject;
                    }
                } else if hTarget.is_null() {
                    hTarget = hObject;
                }
            }
            UnlockLink(&raw mut disp_q, hObject);
            hObject = hNextObject;
        }
        facing = ((*ElementPtr).c2rust_unnamed_2.turn_wait as ::core::ffi::c_int
            >> 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE as COUNT;
        if hTarget.is_null() {
            delta_facing = -1 as SIZE;
        } else {
            ObjectPtr = LockLink(&raw mut disp_q, hTarget) as *mut ELEMENT;
            delta_x = ((*ObjectPtr).current.location.x as ::core::ffi::c_int
                - (*ElementPtr).current.location.x as ::core::ffi::c_int) as SIZE;
            delta_x = (if (delta_x as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                if -(delta_x as ::core::ffi::c_int)
                    <= ((ScreenWidth - 64 as ::core::ffi::c_int
                        - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                        << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                        >> 1 as ::core::ffi::c_int
                {
                    delta_x as ::core::ffi::c_int
                } else {
                    (((ScreenWidth - 64 as ::core::ffi::c_int
                        - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                        << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                        + delta_x as ::core::ffi::c_int
                }
            } else if delta_x as ::core::ffi::c_int
                <= ((ScreenWidth - 64 as ::core::ffi::c_int
                    - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                    << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    >> 1 as ::core::ffi::c_int
            {
                delta_x as ::core::ffi::c_int
            } else {
                delta_x as ::core::ffi::c_int
                    - (((ScreenWidth - 64 as ::core::ffi::c_int
                        - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                        << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
            }) as SIZE;
            delta_y = ((*ObjectPtr).current.location.y as ::core::ffi::c_int
                - (*ElementPtr).current.location.y as ::core::ffi::c_int) as SIZE;
            delta_y = (if (delta_y as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                if -(delta_y as ::core::ffi::c_int)
                    <= ((ScreenHeight
                        - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                        << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                        >> 1 as ::core::ffi::c_int
                {
                    delta_y as ::core::ffi::c_int
                } else {
                    (((ScreenHeight - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                        << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                        + delta_y as ::core::ffi::c_int
                }
            } else if delta_y as ::core::ffi::c_int
                <= ((ScreenHeight - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                    << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    >> 1 as ::core::ffi::c_int
            {
                delta_y as ::core::ffi::c_int
            } else {
                delta_y as ::core::ffi::c_int
                    - (((ScreenHeight
                        - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                        << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
            }) as SIZE;
            if (*ObjectPtr).c2rust_unnamed_1.mass_points as ::core::ffi::c_int
                > 10 as ::core::ffi::c_int * 10 as ::core::ffi::c_int
            {
                delta_facing = (pfacing as ::core::ffi::c_int
                    - facing as ::core::ffi::c_int
                    + (((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                        >> 3 as ::core::ffi::c_int)
                        + ((1 as ::core::ffi::c_int)
                            << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int)
                        >> 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int)
                    & ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int) as COUNT as SIZE;
                if delta_facing as ::core::ffi::c_int
                    > ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                        >> 2 as ::core::ffi::c_int)
                        + ((1 as ::core::ffi::c_int)
                            << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int)
                        >> 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                {
                    delta_facing = 0 as SIZE;
                } else if delta_facing as ::core::ffi::c_int
                    == ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                        >> 3 as ::core::ffi::c_int)
                        + ((1 as ::core::ffi::c_int)
                            << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int)
                        >> 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                {
                    facing = (facing as ::core::ffi::c_int
                        + (((TFB_Random() as SIZE as ::core::ffi::c_int
                            & 1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
                            - 1 as ::core::ffi::c_int)) as COUNT;
                } else if (delta_facing as ::core::ffi::c_int)
                    < ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                        >> 3 as ::core::ffi::c_int)
                        + ((1 as ::core::ffi::c_int)
                            << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int)
                        >> 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                {
                    facing = facing.wrapping_add(1);
                } else {
                    facing = facing.wrapping_sub(1);
                }
            } else {
                let mut num_frames: COUNT = 0;
                let mut ShipVelocity: VELOCITY_DESC = VELOCITY_DESC {
                    TravelAngle: 0,
                    vector: EXTENT { width: 0, height: 0 },
                    fract: EXTENT { width: 0, height: 0 },
                    error: EXTENT { width: 0, height: 0 },
                    incr: EXTENT { width: 0, height: 0 },
                };
                if elementsOfSamePlayer(ObjectPtr, ElementPtr).0 != 0
                    && (*ElementPtr).state_flags as ::core::ffi::c_int
                        & (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int != 0
                {
                    (*ElementPtr).next.image.frame = SetAbsFrameIndex(
                        (*(*StarShipPtr).RaceDescPtr).ship_data.special[0usize],
                        21 as COUNT,
                    );
                    (*ElementPtr).state_flags = ((*ElementPtr).state_flags
                        as ::core::ffi::c_int
                        & !((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int))
                        as ELEMENT_FLAGS;
                    (*ElementPtr).state_flags = ((*ElementPtr).state_flags
                        as ::core::ffi::c_int
                        | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int)
                        as ELEMENT_FLAGS;
                }
                num_frames = (square_root(
                    (delta_x as ::core::ffi::c_long * delta_x as ::core::ffi::c_long
                        + delta_y as ::core::ffi::c_long
                            * delta_y as ::core::ffi::c_long) as DWORD,
                ) as ::core::ffi::c_int >> 6 as ::core::ffi::c_int) as COUNT;
                if num_frames as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    num_frames = 1 as COUNT;
                }
                ShipVelocity = (*ObjectPtr).velocity;
                GetNextVelocityComponents(
                    &raw mut ShipVelocity,
                    &raw mut delta_x,
                    &raw mut delta_y,
                    num_frames,
                );
                delta_x = ((*ObjectPtr).current.location.x as ::core::ffi::c_int
                    + delta_x as ::core::ffi::c_int
                    - (*ElementPtr).current.location.x as ::core::ffi::c_int) as SIZE;
                delta_y = ((*ObjectPtr).current.location.y as ::core::ffi::c_int
                    + delta_y as ::core::ffi::c_int
                    - (*ElementPtr).current.location.y as ::core::ffi::c_int) as SIZE;
                delta_facing = ((ARCTAN(delta_x, delta_y) as ::core::ffi::c_int
                    + ((1 as ::core::ffi::c_int)
                        << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int)
                    >> 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int)
                    - facing as ::core::ffi::c_int
                    & ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int) as COUNT as SIZE;
                if delta_facing as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                    if delta_facing as ::core::ffi::c_int
                        == ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                            >> 1 as ::core::ffi::c_int)
                            + ((1 as ::core::ffi::c_int)
                                << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                                    - 1 as ::core::ffi::c_int)
                            >> 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                    {
                        facing = (facing as ::core::ffi::c_int
                            + (((TFB_Random() as BYTE as ::core::ffi::c_int
                                & 1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
                                - 1 as ::core::ffi::c_int)) as COUNT;
                    } else if (delta_facing as ::core::ffi::c_int)
                        < ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
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
            }
            UnlockLink(&raw mut disp_q, hTarget);
        }
        (*ElementPtr).c2rust_unnamed_2.turn_wait = (((facing as ::core::ffi::c_int
            & ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                - 1 as ::core::ffi::c_int) as COUNT as BYTE as ::core::ffi::c_int)
            << 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int
            | 0 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE;
        if delta_facing as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            || (*ElementPtr).c2rust_unnamed_3.thrust_wait as ::core::ffi::c_int
                & (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                    >> 6 as ::core::ffi::c_int != 0
                && (*ElementPtr).c2rust_unnamed_3.thrust_wait as ::core::ffi::c_int
                    & (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
                        >> 6 as ::core::ffi::c_int == 0
        {
            let mut thrust_status: STATUS_FLAGS = 0;
            let mut OldFacing: COUNT = 0;
            let mut OldStatus: STATUS_FLAGS = 0;
            let mut OldIncrement: COUNT = 0;
            let mut OldThrust: COUNT = 0;
            let mut StarShipPtr_0: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
            StarShipPtr_0 = (*ElementPtr).pParent as *mut STARSHIP;
            OldFacing = (*StarShipPtr_0).ShipFacing;
            OldStatus = (*StarShipPtr_0).cur_status_flags;
            OldIncrement = (*(*StarShipPtr_0).RaceDescPtr)
                .characteristics
                .thrust_increment;
            OldThrust = (*(*StarShipPtr_0).RaceDescPtr).characteristics.max_thrust;
            (*StarShipPtr_0).ShipFacing = facing;
            (*StarShipPtr_0).cur_status_flags = (((*ElementPtr)
                .c2rust_unnamed_3
                .thrust_wait as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                as STATUS_FLAGS;
            (*(*StarShipPtr_0).RaceDescPtr).characteristics.thrust_increment = 8
                as COUNT;
            (*(*StarShipPtr_0).RaceDescPtr).characteristics.max_thrust = 32 as COUNT;
            thrust_status = inertial_thrust(ElementPtr);
            (*(*StarShipPtr_0).RaceDescPtr).characteristics.max_thrust = OldThrust;
            (*(*StarShipPtr_0).RaceDescPtr).characteristics.thrust_increment = OldIncrement;
            (*StarShipPtr_0).cur_status_flags = OldStatus;
            (*StarShipPtr_0).ShipFacing = OldFacing;
            if (*ElementPtr).c2rust_unnamed_3.thrust_wait as ::core::ffi::c_int
                & (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
                    >> 6 as ::core::ffi::c_int != 0
                || delta_facing as ::core::ffi::c_int != 0
                || thrust_status as ::core::ffi::c_int
                    & ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int
                        | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) == 0
            {
                spawn_marine_ion_trail(ElementPtr, StarShipPtr_0, facing);
            }
            (*ElementPtr).c2rust_unnamed_3.thrust_wait = (thrust_status
                as ::core::ffi::c_int >> 6 as ::core::ffi::c_int) as BYTE;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn marine_collision(
    mut ElementPtr0: *mut ELEMENT,
    mut pPt0: *mut POINT,
    mut ElementPtr1: *mut ELEMENT,
    mut pPt1: *mut POINT,
) {
    if (*ElementPtr0).c2rust_unnamed.life_span as ::core::ffi::c_int != 0
        && (*ElementPtr0).state_flags as ::core::ffi::c_int
            & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) == 0
        && (*ElementPtr1).state_flags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int == 0
    {
        if elementsOfSamePlayer(ElementPtr0, ElementPtr1).0 == 0 {
            (*ElementPtr0).c2rust_unnamed_2.turn_wait = ((((*ElementPtr0)
                .c2rust_unnamed_2
                .turn_wait as ::core::ffi::c_int
                >> 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE
                as ::core::ffi::c_int)
                << 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int
                | 5 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE;
            (*ElementPtr0).c2rust_unnamed_3.thrust_wait = ((*ElementPtr0)
                .c2rust_unnamed_3
                .thrust_wait as ::core::ffi::c_int
                & !(((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                    >> 6 as ::core::ffi::c_int)) as BYTE;
            (*ElementPtr0).state_flags = ((*ElementPtr0).state_flags
                as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as ELEMENT_FLAGS;
        }
        if (*ElementPtr1).c2rust_unnamed_1.mass_points as ::core::ffi::c_int
            > 10 as ::core::ffi::c_int * 10 as ::core::ffi::c_int
        {
            (*ElementPtr0).state_flags = ((*ElementPtr0).state_flags
                as ::core::ffi::c_int
                | ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int))
                as ELEMENT_FLAGS;
            (*ElementPtr0).c2rust_unnamed_0.hit_points = 0 as COUNT;
            (*ElementPtr0).c2rust_unnamed.life_span = 0 as COUNT;
        } else if (*ElementPtr1).state_flags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int != 0
            && ((*ElementPtr1).state_flags as ::core::ffi::c_int
                & (1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int != 0
                || (*ElementPtr1).c2rust_unnamed.life_span as ::core::ffi::c_int
                    == 1 as ::core::ffi::c_int)
        {
            (*ElementPtr1).state_flags = ((*ElementPtr1).state_flags
                as ::core::ffi::c_int
                & !((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int))
                as ELEMENT_FLAGS;
            if (*ElementPtr0).state_flags as ::core::ffi::c_int
                & (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int == 0
            {
                DeltaCrew(ElementPtr1, 1 as SIZE);
                (*ElementPtr0).state_flags = ((*ElementPtr0).state_flags
                    as ::core::ffi::c_int
                    | ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                        | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                        | (1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int))
                    as ELEMENT_FLAGS;
                (*ElementPtr0).c2rust_unnamed_0.hit_points = 0 as COUNT;
                (*ElementPtr0).c2rust_unnamed.life_span = 0 as COUNT;
            } else if (*ElementPtr0).state_flags as ::core::ffi::c_int
                & (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int != 0
                && (*ElementPtr1).c2rust_unnamed_0.crew_level as ::core::ffi::c_int != 0
            {
                let mut s: STAMP = STAMP {
                    origin: POINT { x: 0, y: 0 },
                    frame: ::core::ptr::null_mut::<FRAME_DESC>(),
                };
                let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
                StarShipPtr = (*ElementPtr0).pParent as *mut STARSHIP;
                if DeltaCrew(ElementPtr1, -1 as SIZE).0 == 0 {
                    (*ElementPtr1).c2rust_unnamed.life_span = 0 as COUNT;
                } else {
                    (*ElementPtr0).c2rust_unnamed_2.turn_wait = count_marines(
                        StarShipPtr,
                        BOOLEAN::TRUE,
                    );
                    (*ElementPtr0).c2rust_unnamed_3.thrust_wait = 12 as BYTE;
                    (*ElementPtr0).next.image.frame = SetAbsFrameIndex(
                        *(*ElementPtr0).next.image.farray.offset(0isize),
                        (22 as ::core::ffi::c_int
                            + (*ElementPtr0).c2rust_unnamed_2.turn_wait
                                as ::core::ffi::c_int) as COUNT,
                    );
                    (*ElementPtr0).state_flags = ((*ElementPtr0).state_flags
                        as ::core::ffi::c_int
                        | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                        as ELEMENT_FLAGS;
                    (*ElementPtr0).state_flags = ((*ElementPtr0).state_flags
                        as ::core::ffi::c_int
                        & !((1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int))
                        as ELEMENT_FLAGS;
                    (*GlobData
                        .Game_state
                        .DisplayArray
                        .offset((*ElementPtr0).PrimIndex as isize))
                        .Type = gfx_object::NUM_PRIMS.0 as ::core::ffi::c_int
                        as GRAPHICS_PRIM;
                    (*ElementPtr0).preprocess_func = Some(
                        intruder_preprocess as unsafe extern "C" fn(*mut ELEMENT) -> (),
                    ) as Option<ElementProcessFunc>;
                    s.origin.x = (16 as ::core::ffi::c_int
                        + ((*ElementPtr0).c2rust_unnamed_2.turn_wait
                            as ::core::ffi::c_int & 3 as ::core::ffi::c_int)
                            * 9 as ::core::ffi::c_int) as COORD;
                    s.origin.y = (14 as ::core::ffi::c_int
                        + ((*ElementPtr0).c2rust_unnamed_2.turn_wait
                            as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
                            * 11 as ::core::ffi::c_int) as COORD;
                    s.frame = (*ElementPtr0).next.image.frame;
                    ModifySilhouette(ElementPtr1, &raw mut s, 0 as BYTE);
                }
                ProcessSound(
                    SetAbsStringTableIndex(
                        (*(*StarShipPtr).RaceDescPtr).ship_data.ship_sounds,
                        2 as COUNT,
                    ),
                    ElementPtr1,
                );
            }
            (*ElementPtr0).state_flags = ((*ElementPtr0).state_flags
                as ::core::ffi::c_int
                & !((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int))
                as ELEMENT_FLAGS;
        }
    }
}
unsafe extern "C" fn animate(mut ElementPtr: *mut ELEMENT) {
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
unsafe extern "C" fn turret_postprocess(mut ElementPtr: *mut ELEMENT) {
    if (*ElementPtr).c2rust_unnamed.life_span as ::core::ffi::c_int
        == 0 as ::core::ffi::c_int
    {
        let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
        (*GlobData.Game_state.DisplayArray.offset((*ElementPtr).PrimIndex as isize))
            .Type = gfx_object::NUM_PRIMS.0 as ::core::ffi::c_int as GRAPHICS_PRIM;
        StarShipPtr = (*ElementPtr).pParent as *mut STARSHIP;
        if !(*StarShipPtr).hShip.is_null() {
            let mut facing: COUNT = 0;
            let mut hTurret: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
            let mut hSpaceMarine: HELEMENT = ::core::ptr::null_mut::<
                ::core::ffi::c_void,
            >();
            let mut ShipPtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
            ShipPtr = LockLink(&raw mut disp_q, (*StarShipPtr).hShip) as *mut ELEMENT;
            hTurret = AllocElement();
            if !hTurret.is_null() {
                let mut TurretPtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
                TurretPtr = LockLink(&raw mut disp_q, hTurret) as *mut ELEMENT;
                (*TurretPtr).playerNr = (*ElementPtr).playerNr;
                (*TurretPtr).state_flags = ((1 as ::core::ffi::c_int)
                    << 10 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 11 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                    as ELEMENT_FLAGS;
                (*TurretPtr).c2rust_unnamed.life_span = 1 as COUNT;
                (*TurretPtr).current.image = (*ElementPtr).current.image;
                (*TurretPtr).current.location = (*ShipPtr).next.location;
                (*TurretPtr).c2rust_unnamed_2.turn_wait = (*ElementPtr)
                    .c2rust_unnamed_2
                    .turn_wait;
                (*TurretPtr).c2rust_unnamed_3.thrust_wait = (*ElementPtr)
                    .c2rust_unnamed_3
                    .thrust_wait;
                if (*TurretPtr).c2rust_unnamed_2.turn_wait != 0 {
                    (*TurretPtr).c2rust_unnamed_2.turn_wait = (*TurretPtr)
                        .c2rust_unnamed_2
                        .turn_wait
                        .wrapping_sub(1);
                } else if (*StarShipPtr).cur_status_flags as ::core::ffi::c_int
                    & (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int != 0
                    && (*StarShipPtr).cur_status_flags as ::core::ffi::c_int
                        & ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                            | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) != 0
                {
                    if (*StarShipPtr).cur_status_flags as ::core::ffi::c_int
                        & (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int != 0
                    {
                        (*TurretPtr).c2rust_unnamed_3.thrust_wait = (*TurretPtr)
                            .c2rust_unnamed_3
                            .thrust_wait
                            .wrapping_add(1);
                    } else {
                        (*TurretPtr).c2rust_unnamed_3.thrust_wait = (*TurretPtr)
                            .c2rust_unnamed_3
                            .thrust_wait
                            .wrapping_sub(1);
                    }
                    (*TurretPtr).c2rust_unnamed_2.turn_wait = 3 as BYTE;
                }
                facing = ((*StarShipPtr).ShipFacing as ::core::ffi::c_int
                    + (*TurretPtr).c2rust_unnamed_3.thrust_wait as ::core::ffi::c_int
                    & ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int) as COUNT;
                (*(*StarShipPtr).RaceDescPtr).ship_info.ship_flags = ((*(*StarShipPtr)
                    .RaceDescPtr)
                    .ship_info
                    .ship_flags as ::core::ffi::c_int
                    & !((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int
                        | (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
                        | (1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int
                        | (1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int))
                    as UWORD;
                (*(*StarShipPtr).RaceDescPtr).ship_info.ship_flags = ((*(*StarShipPtr)
                    .RaceDescPtr)
                    .ship_info
                    .ship_flags as ::core::ffi::c_int
                    | ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int)
                        << (facing as ::core::ffi::c_int
                            + (((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                                >> 3 as ::core::ffi::c_int)
                                + ((1 as ::core::ffi::c_int)
                                    << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                                        - 1 as ::core::ffi::c_int)
                                >> 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int)
                            & ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                                - 1 as ::core::ffi::c_int) as COUNT as ::core::ffi::c_int
                            / (((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                                >> 2 as ::core::ffi::c_int)
                                + ((1 as ::core::ffi::c_int)
                                    << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                                        - 1 as ::core::ffi::c_int)
                                >> 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int))
                    as UWORD;
                (*TurretPtr).current.image.frame = SetAbsFrameIndex(
                    (*TurretPtr).current.image.frame,
                    facing,
                );
                facing = ((facing as ::core::ffi::c_int)
                    << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int) as COUNT;
                if (*StarShipPtr).cur_status_flags as ::core::ffi::c_int
                    & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int != 0
                {
                    let mut hTurretEffect: HELEMENT = ::core::ptr::null_mut::<
                        ::core::ffi::c_void,
                    >();
                    let mut TurretEffectPtr: *mut ELEMENT = ::core::ptr::null_mut::<
                        ELEMENT,
                    >();
                    TurretEffectPtr = LockLink(&raw mut disp_q, disp_q.tail)
                        as *mut ELEMENT;
                    if TurretEffectPtr != ElementPtr
                        && elementsOfSamePlayer(TurretEffectPtr, ElementPtr).0 != 0
                        && (*TurretEffectPtr).state_flags as ::core::ffi::c_int
                            & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int != 0
                        && (*GlobData
                            .Game_state
                            .DisplayArray
                            .offset((*TurretEffectPtr).PrimIndex as isize))
                            .Type as ::core::ffi::c_int
                            == gfx_object::STAMP_PRIM.0 as ::core::ffi::c_int
                        && {
                            hTurretEffect = AllocElement();
                            !hTurretEffect.is_null()
                        }
                    {
                        (*TurretPtr).current.location.x = ((*TurretPtr)
                            .current
                            .location
                            .x as ::core::ffi::c_int
                            - (*(&raw mut sinetab as *mut SIZE)
                                .offset(
                                    (facing as ::core::ffi::c_int
                                        + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                                            >> 2 as ::core::ffi::c_int)
                                        & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                                            - 1 as ::core::ffi::c_int) as COUNT as isize,
                                ) as ::core::ffi::c_long
                                * ((2 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                                    as ::core::ffi::c_long >> 14 as ::core::ffi::c_int) as SIZE
                                as ::core::ffi::c_int) as COORD;
                        (*TurretPtr).current.location.y = ((*TurretPtr)
                            .current
                            .location
                            .y as ::core::ffi::c_int
                            - (*(&raw mut sinetab as *mut SIZE)
                                .offset(
                                    (facing as ::core::ffi::c_int
                                        & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                                            - 1 as ::core::ffi::c_int) as COUNT as isize,
                                ) as ::core::ffi::c_long
                                * ((2 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                                    as ::core::ffi::c_long >> 14 as ::core::ffi::c_int) as SIZE
                                as ::core::ffi::c_int) as COORD;
                        TurretEffectPtr = LockLink(&raw mut disp_q, hTurretEffect)
                            as *mut ELEMENT;
                        (*TurretEffectPtr).playerNr = (*ElementPtr).playerNr;
                        (*TurretEffectPtr).state_flags = ((1 as ::core::ffi::c_int)
                            << 10 as ::core::ffi::c_int
                            | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                            | (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
                            | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                            as ELEMENT_FLAGS;
                        (*TurretEffectPtr).c2rust_unnamed.life_span = 4 as COUNT;
                        (*TurretEffectPtr).current.location.x = ((*TurretPtr)
                            .current
                            .location
                            .x as ::core::ffi::c_int
                            + (*(&raw mut sinetab as *mut SIZE)
                                .offset(
                                    (facing as ::core::ffi::c_int
                                        + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                                            >> 2 as ::core::ffi::c_int)
                                        & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                                            - 1 as ::core::ffi::c_int) as COUNT as isize,
                                ) as ::core::ffi::c_long
                                * ((14 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                                    as ::core::ffi::c_long >> 14 as ::core::ffi::c_int) as SIZE
                                as ::core::ffi::c_int) as COORD;
                        (*TurretEffectPtr).current.location.y = ((*TurretPtr)
                            .current
                            .location
                            .y as ::core::ffi::c_int
                            + (*(&raw mut sinetab as *mut SIZE)
                                .offset(
                                    (facing as ::core::ffi::c_int
                                        & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                                            - 1 as ::core::ffi::c_int) as COUNT as isize,
                                ) as ::core::ffi::c_long
                                * ((14 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                                    as ::core::ffi::c_long >> 14 as ::core::ffi::c_int) as SIZE
                                as ::core::ffi::c_int) as COORD;
                        (*TurretEffectPtr).current.image.farray = &raw mut (*(*StarShipPtr)
                            .RaceDescPtr)
                            .ship_data
                            .special as *mut FRAME;
                        (*TurretEffectPtr).current.image.frame = SetAbsFrameIndex(
                            (*(*StarShipPtr).RaceDescPtr).ship_data.special[0usize],
                            (((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                                + ((1 as ::core::ffi::c_int)
                                    << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                                        - 1 as ::core::ffi::c_int)
                                >> 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int)
                                as COUNT,
                        );
                        (*TurretEffectPtr).preprocess_func = Some(
                            animate as unsafe extern "C" fn(*mut ELEMENT) -> (),
                        ) as Option<ElementProcessFunc>;
                        (*TurretEffectPtr).pParent = StarShipPtr
                            as *mut ::core::ffi::c_void;
                        (*GlobData
                            .Game_state
                            .DisplayArray
                            .offset((*TurretEffectPtr).PrimIndex as isize))
                            .Type = gfx_object::STAMP_PRIM.0 as ::core::ffi::c_int
                            as GRAPHICS_PRIM;
                        UnlockLink(&raw mut disp_q, hTurretEffect);
                        PutQueue(&raw mut disp_q, hTurretEffect);
                    }
                    UnlockLink(&raw mut disp_q, disp_q.tail);
                }
                (*TurretPtr).next = (*TurretPtr).current;
                (*GlobData
                    .Game_state
                    .DisplayArray
                    .offset((*TurretPtr).PrimIndex as isize))
                    .Type = (*GlobData
                    .Game_state
                    .DisplayArray
                    .offset((*ShipPtr).PrimIndex as isize))
                    .Type;
                (*GlobData
                    .Game_state
                    .DisplayArray
                    .offset((*TurretPtr).PrimIndex as isize))
                    .color = (*GlobData
                    .Game_state
                    .DisplayArray
                    .offset((*ShipPtr).PrimIndex as isize))
                    .color;
                (*TurretPtr).postprocess_func = (*ElementPtr).postprocess_func;
                (*TurretPtr).pParent = StarShipPtr as *mut ::core::ffi::c_void;
                UnlockLink(&raw mut disp_q, hTurret);
                InsertQueue(&raw mut disp_q, hTurret, (*ElementPtr).succ);
            }
            if (*StarShipPtr).special_counter as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
                && (*StarShipPtr).cur_status_flags as ::core::ffi::c_int
                    & (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int != 0
                && (*StarShipPtr).cur_status_flags as ::core::ffi::c_int
                    & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int != 0
                && (*ShipPtr).c2rust_unnamed_0.crew_level as ::core::ffi::c_int
                    > 1 as ::core::ffi::c_int
                && (count_marines(StarShipPtr, BOOLEAN::FALSE) as ::core::ffi::c_int)
                    < 8 as ::core::ffi::c_int
                && TrackShip(ShipPtr, &raw mut facing) as ::core::ffi::c_int
                    >= 0 as ::core::ffi::c_int
                && {
                    hSpaceMarine = AllocElement();
                    !hSpaceMarine.is_null()
                }
            {
                let mut SpaceMarinePtr: *mut ELEMENT = ::core::ptr::null_mut::<
                    ELEMENT,
                >();
                SpaceMarinePtr = LockLink(&raw mut disp_q, hSpaceMarine) as *mut ELEMENT;
                (*SpaceMarinePtr).playerNr = (*ElementPtr).playerNr;
                (*SpaceMarinePtr).state_flags = ((1 as ::core::ffi::c_int)
                    << 8 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int)
                    as ELEMENT_FLAGS;
                (*SpaceMarinePtr).c2rust_unnamed.life_span = 1 as COUNT;
                (*SpaceMarinePtr).c2rust_unnamed_0.hit_points = 3 as COUNT;
                (*SpaceMarinePtr).c2rust_unnamed_1.mass_points = 1 as BYTE;
                facing = (((*StarShipPtr).ShipFacing as ::core::ffi::c_int)
                    << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int) as COUNT;
                (*SpaceMarinePtr).current.location.x = ((*ShipPtr).current.location.x
                    as ::core::ffi::c_int
                    - (*(&raw mut sinetab as *mut SIZE)
                        .offset(
                            (facing as ::core::ffi::c_int
                                + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                                    >> 2 as ::core::ffi::c_int)
                                & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                                    - 1 as ::core::ffi::c_int) as COUNT as isize,
                        ) as ::core::ffi::c_long
                        * ((14 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                            as ::core::ffi::c_long >> 14 as ::core::ffi::c_int) as SIZE
                        as ::core::ffi::c_int) as COORD;
                (*SpaceMarinePtr).current.location.y = ((*ShipPtr).current.location.y
                    as ::core::ffi::c_int
                    - (*(&raw mut sinetab as *mut SIZE)
                        .offset(
                            (facing as ::core::ffi::c_int
                                & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                                    - 1 as ::core::ffi::c_int) as COUNT as isize,
                        ) as ::core::ffi::c_long
                        * ((14 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                            as ::core::ffi::c_long >> 14 as ::core::ffi::c_int) as SIZE
                        as ::core::ffi::c_int) as COORD;
                (*SpaceMarinePtr).current.image.farray = &raw mut (*(*StarShipPtr)
                    .RaceDescPtr)
                    .ship_data
                    .special as *mut FRAME;
                (*SpaceMarinePtr).current.image.frame = SetAbsFrameIndex(
                    (*(*StarShipPtr).RaceDescPtr).ship_data.special[0usize],
                    20 as COUNT,
                );
                (*SpaceMarinePtr).c2rust_unnamed_2.turn_wait = (((facing
                    as ::core::ffi::c_int
                    + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                        >> 1 as ::core::ffi::c_int)
                    + ((1 as ::core::ffi::c_int)
                        << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int)
                    >> 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                    & ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int) as COUNT as BYTE
                    as ::core::ffi::c_int)
                    << 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int
                    | 0 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE;
                (*SpaceMarinePtr).preprocess_func = Some(
                    marine_preprocess as unsafe extern "C" fn(*mut ELEMENT) -> (),
                ) as Option<ElementProcessFunc>;
                (*SpaceMarinePtr).collision_func = Some(
                    marine_collision
                        as unsafe extern "C" fn(
                            *mut ELEMENT,
                            *mut POINT,
                            *mut ELEMENT,
                            *mut POINT,
                        ) -> (),
                ) as Option<ElementCollisionFunc>;
                (*SpaceMarinePtr).pParent = StarShipPtr as *mut ::core::ffi::c_void;
                (*GlobData
                    .Game_state
                    .DisplayArray
                    .offset((*SpaceMarinePtr).PrimIndex as isize))
                    .Type = gfx_object::STAMP_PRIM.0 as ::core::ffi::c_int
                    as GRAPHICS_PRIM;
                UnlockLink(&raw mut disp_q, hSpaceMarine);
                PutQueue(&raw mut disp_q, hSpaceMarine);
                DeltaCrew(ShipPtr, -1 as SIZE);
                ProcessSound(
                    SetAbsStringTableIndex(
                        (*(*StarShipPtr).RaceDescPtr).ship_data.ship_sounds,
                        1 as COUNT,
                    ),
                    SpaceMarinePtr,
                );
                (*StarShipPtr).special_counter = (*(*StarShipPtr).RaceDescPtr)
                    .characteristics
                    .special_wait;
            }
            UnlockLink(&raw mut disp_q, (*StarShipPtr).hShip);
        }
    }
}
unsafe extern "C" fn orz_preprocess(mut ElementPtr: *mut ELEMENT) {
    let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
    StarShipPtr = (*ElementPtr).pParent as *mut STARSHIP;
    if (*ElementPtr).state_flags as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int == 0
    {
        if ((*StarShipPtr).cur_status_flags as ::core::ffi::c_int
            | (*StarShipPtr).old_status_flags as ::core::ffi::c_int)
            & (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int != 0
            && (*StarShipPtr).cur_status_flags as ::core::ffi::c_int
                & ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) != 0
            && (*ElementPtr).c2rust_unnamed_2.turn_wait as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
        {
            (*ElementPtr).c2rust_unnamed_2.turn_wait = (*ElementPtr)
                .c2rust_unnamed_2
                .turn_wait
                .wrapping_add(1);
        }
        if (*StarShipPtr).cur_status_flags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int != 0
            && (*StarShipPtr).cur_status_flags as ::core::ffi::c_int
                & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int != 0
            && (*StarShipPtr).weapon_counter as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
        {
            (*StarShipPtr).weapon_counter = (*StarShipPtr)
                .weapon_counter
                .wrapping_add(1);
        }
    } else {
        let mut hTurret: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
        hTurret = AllocElement();
        if !hTurret.is_null() {
            let mut TurretPtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
            TurretPtr = LockLink(&raw mut disp_q, hTurret) as *mut ELEMENT;
            (*TurretPtr).playerNr = (*ElementPtr).playerNr;
            (*TurretPtr).state_flags = ((1 as ::core::ffi::c_int)
                << 10 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ELEMENT_FLAGS;
            (*TurretPtr).c2rust_unnamed.life_span = 1 as COUNT;
            (*TurretPtr).current.image.farray = &raw mut (*(*StarShipPtr).RaceDescPtr)
                .ship_data
                .special as *mut FRAME;
            (*TurretPtr).current.image.frame = SetAbsFrameIndex(
                (*(*StarShipPtr).RaceDescPtr).ship_data.special[0usize],
                (*StarShipPtr).ShipFacing,
            );
            (*TurretPtr).postprocess_func = Some(
                turret_postprocess as unsafe extern "C" fn(*mut ELEMENT) -> (),
            ) as Option<ElementProcessFunc>;
            (*TurretPtr).pParent = StarShipPtr as *mut ::core::ffi::c_void;
            UnlockLink(&raw mut disp_q, hTurret);
            InsertQueue(&raw mut disp_q, hTurret, (*ElementPtr).succ);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn init_orz() -> *mut RACE_DESC {
    let mut RaceDescPtr: *mut RACE_DESC = ::core::ptr::null_mut::<RACE_DESC>();
    orz_desc.preprocess_func = Some(
        orz_preprocess as unsafe extern "C" fn(*mut ELEMENT) -> (),
    ) as Option<PREPROCESS_FUNC>;
    orz_desc.init_weapon_func = Some(
        initialize_turret_missile
            as unsafe extern "C" fn(*mut ELEMENT, *mut HELEMENT) -> COUNT,
    ) as Option<INIT_WEAPON_FUNC>;
    orz_desc.cyborg_control.intelligence_func = Some(
        orz_intelligence
            as unsafe extern "C" fn(*mut ELEMENT, *mut EVALUATE_DESC, COUNT) -> (),
    ) as Option<IntelligenceFunc>;
    RaceDescPtr = &raw mut orz_desc;
    return RaceDescPtr;
}
