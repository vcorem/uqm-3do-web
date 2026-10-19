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

// Rust translation of src/uqm/misc.c.
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
    fn DrawablesIntersect(
        pControl0: *mut INTERSECT_CONTROL,
        pControl1: *mut INTERSECT_CONTROL,
        max_time_val: TIME_VALUE,
    ) -> TIME_VALUE;
    fn GetFrameRect(Frame: FRAME, pRect: *mut RECT) -> BOOLEAN;
    fn GetFrameIndex(Frame: FRAME) -> COUNT;
    fn SetAbsFrameIndex(Frame: FRAME, FrameIndex: COUNT) -> FRAME;
    fn DecFrameIndex(Frame: FRAME) -> FRAME;
    fn SetAbsStringTableIndex(String: STRING, StringTableIndex: COUNT) -> STRING;
    fn memset(
        _: *mut ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    static mut resolutionFactor: ::core::ffi::c_uint;
    static mut optGodModes: ::core::ffi::c_int;
    static mut optMeleeObstacles: OPT_ENABLABLE;
    static mut optWindowType: DWORD;
    static mut CanvasWidth: ::core::ffi::c_int;
    static mut CanvasHeight: ::core::ffi::c_int;
    static mut sinetab: [SDWORD; 0];
    fn SetVelocityVector(
        velocityptr: *mut VELOCITY_DESC,
        magnitude: SDWORD,
        facing: COUNT,
    );
    static mut disp_q: QUEUE;
    static mut DisplayArray: [PRIMITIVE; 330];
    fn AllocElement() -> HELEMENT;
    fn TimeSpaceMatterConflict(ElementPtr: *mut ELEMENT) -> BOOLEAN;
    fn SetStarPoint(pt: POINT, i: COUNT);
    fn CalculateGravity(ElementPtr: *mut ELEMENT) -> BOOLEAN;
    static mut asteroid: [FRAME; 0];
    static mut stars_in_space: FRAME;
    static mut stars_misc: [FRAME; 3];
    fn animation_preprocess(ElementPtr: *mut ELEMENT);
    fn collision(
        ElementPtr0: *mut ELEMENT,
        pPt0: *mut POINT,
        ElementPtr1: *mut ELEMENT,
        pPt1: *mut POINT,
    );
    fn DeltaCrew(ElementPtr: *mut ELEMENT, crew_delta: SIZE) -> BOOLEAN;
    fn getGameStateUint(name: *const ::core::ffi::c_char) -> DWORD;
    static mut GlobData: GLOBDATA;
    static mut PlayerControl: [BYTE; 0];
    static mut GameSounds: SOUND;
    fn ProcessSound(Sound: SOUND, PositionalObject: *mut ELEMENT);
    fn TrackShip(Tracker: *mut ELEMENT, pfacing: *mut COUNT) -> SIZE;
    fn TFB_Random() -> DWORD;
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
pub struct OPT_ENABLABLE(pub ::core::ffi::c_uint);
impl OPT_ENABLABLE {
    pub const OPTVAL_DISABLED: Self = Self(0);
    pub const OPTVAL_ENABLED: Self = Self(1);
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
    pub stowed_ship_q: QUEUE,
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_6(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_6 {
    pub const GRAB_CREW: Self = Self(0);
    pub const SHIP_EXPLODES: Self = Self(1);
    pub const TARGET_DAMAGED_FOR_1_PT: Self = Self(2);
    pub const TARGET_DAMAGED_FOR_2_3_PT: Self = Self(3);
    pub const TARGET_DAMAGED_FOR_4_5_PT: Self = Self(4);
    pub const TARGET_DAMAGED_FOR_6_PLUS_PT: Self = Self(5);
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
unsafe extern "C" fn buildColorRgba(
    mut r: BYTE,
    mut g: BYTE,
    mut b: BYTE,
    mut a: BYTE,
) -> Color {
    let mut c: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    c.r = r;
    c.g = g;
    c.b = b;
    c.a = a;
    return c;
}
#[inline]
unsafe extern "C" fn MAKE_POINT(mut x: COORD, mut y: COORD) -> POINT {
    let mut pt: POINT = point { x: x, y: y };
    return pt;
}
unsafe extern "C" fn planet_postprocess(mut ElementPtr: *mut ELEMENT) {
    CalculateGravity(ElementPtr);
}
#[no_mangle]
pub unsafe extern "C" fn spawn_planet() {
    let mut hPlanetElement: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut pt: POINT = MAKE_POINT(0 as COORD, 0 as COORD);
    hPlanetElement = AllocElement();
    if !hPlanetElement.is_null() {
        let mut PlanetElementPtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
        extern "C" {
            static mut planet: [FRAME; 0];
        }
        PlanetElementPtr = LockLink(&raw mut disp_q, hPlanetElement) as *mut ELEMENT;
        (*PlanetElementPtr).playerNr = -1 as SIZE;
        (*PlanetElementPtr).c2rust_unnamed_0.hit_points = 200 as COUNT;
        (*PlanetElementPtr).state_flags = ((1 as ::core::ffi::c_int)
            << 3 as ::core::ffi::c_int) as ELEMENT_FLAGS;
        (*PlanetElementPtr).c2rust_unnamed.life_span = (1 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int) as COUNT;
        if optMeleeObstacles.0 != 0 && isNetwork().0 == 0 {
            (*PlanetElementPtr).state_flags = ((*PlanetElementPtr).state_flags
                as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as ELEMENT_FLAGS;
            DisplayArray[(*PlanetElementPtr).PrimIndex as usize].Type = gfx_object::POINT_PRIM
                .0 as ::core::ffi::c_int as GRAPHICS_PRIM;
            DisplayArray[(*PlanetElementPtr).PrimIndex as usize].color = buildColorRgba(
                0 as BYTE,
                0 as BYTE,
                0 as BYTE,
                0 as BYTE,
            );
            (*PlanetElementPtr).current.location.x = (if TFB_Random()
                .wrapping_rem(
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
                        << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as DWORD,
                ) as ::core::ffi::c_int
                & !(((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
            {
                (TFB_Random()
                    .wrapping_rem(
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
                            << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                            as DWORD,
                    ) as ::core::ffi::c_int
                    & !(((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int))
                    + (((CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
            } else if TFB_Random()
                .wrapping_rem(
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
                        << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as DWORD,
                ) as ::core::ffi::c_int
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
                (TFB_Random()
                    .wrapping_rem(
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
                            << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                            as DWORD,
                    ) as ::core::ffi::c_int
                    & !(((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int))
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
            } else {
                TFB_Random()
                    .wrapping_rem(
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
                            << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                            as DWORD,
                    ) as ::core::ffi::c_int
                    & !(((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int)
            }) as DWORD as COORD;
            (*PlanetElementPtr).current.location.y = (if TFB_Random()
                .wrapping_rem(
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
                        << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as DWORD,
                ) as ::core::ffi::c_int
                & !(((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
            {
                (TFB_Random()
                    .wrapping_rem(
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
                            << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                            as DWORD,
                    ) as ::core::ffi::c_int
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
            } else if TFB_Random()
                .wrapping_rem(
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
                        << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as DWORD,
                ) as ::core::ffi::c_int
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
                (TFB_Random()
                    .wrapping_rem(
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
                            << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                            as DWORD,
                    ) as ::core::ffi::c_int
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
                TFB_Random()
                    .wrapping_rem(
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
                            << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                            as DWORD,
                    ) as ::core::ffi::c_int
                    & !(((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int)
            }) as DWORD as COORD;
            (*PlanetElementPtr).collision_func = None;
            (*PlanetElementPtr).postprocess_func = None;
            memset(
                &raw mut (*PlanetElementPtr).velocity as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<VELOCITY_DESC>(),
            );
        } else {
            DisplayArray[(*PlanetElementPtr).PrimIndex as usize].Type = gfx_object::STAMP_PRIM
                .0 as ::core::ffi::c_int as GRAPHICS_PRIM;
            (*PlanetElementPtr).current.image.farray = &raw mut planet as *mut FRAME;
            (*PlanetElementPtr).current.image.frame = *(*PlanetElementPtr)
                .current
                .image
                .farray
                .offset(0isize);
            (*PlanetElementPtr).collision_func = Some(
                collision
                    as unsafe extern "C" fn(
                        *mut ELEMENT,
                        *mut POINT,
                        *mut ELEMENT,
                        *mut POINT,
                    ) -> (),
            ) as Option<ElementCollisionFunc>;
            (*PlanetElementPtr).postprocess_func = Some(
                planet_postprocess as unsafe extern "C" fn(*mut ELEMENT) -> (),
            ) as Option<ElementProcessFunc>;
            memset(
                &raw mut (*PlanetElementPtr).velocity as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<VELOCITY_DESC>(),
            );
            loop {
                (*PlanetElementPtr).current.location.x = (if TFB_Random()
                    .wrapping_rem(
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
                            << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                            as DWORD,
                    ) as ::core::ffi::c_int
                    & !(((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
                {
                    (TFB_Random()
                        .wrapping_rem(
                            (((CanvasWidth
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
                                as DWORD,
                        ) as ::core::ffi::c_int
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
                } else if TFB_Random()
                    .wrapping_rem(
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
                            << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                            as DWORD,
                    ) as ::core::ffi::c_int
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
                    (TFB_Random()
                        .wrapping_rem(
                            (((CanvasWidth
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
                                as DWORD,
                        ) as ::core::ffi::c_int
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
                    TFB_Random()
                        .wrapping_rem(
                            (((CanvasWidth
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
                                as DWORD,
                        ) as ::core::ffi::c_int
                        & !(((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                            - 1 as ::core::ffi::c_int)
                }) as DWORD as COORD;
                (*PlanetElementPtr).current.location.y = (if TFB_Random()
                    .wrapping_rem(
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
                            << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                            as DWORD,
                    ) as ::core::ffi::c_int
                    & !(((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
                {
                    (TFB_Random()
                        .wrapping_rem(
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
                                << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                                as DWORD,
                        ) as ::core::ffi::c_int
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
                } else if TFB_Random()
                    .wrapping_rem(
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
                            << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                            as DWORD,
                    ) as ::core::ffi::c_int
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
                    (TFB_Random()
                        .wrapping_rem(
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
                                << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                                as DWORD,
                        ) as ::core::ffi::c_int
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
                    TFB_Random()
                        .wrapping_rem(
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
                                << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                                as DWORD,
                        ) as ::core::ffi::c_int
                        & !(((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                            - 1 as ::core::ffi::c_int)
                }) as DWORD as COORD;
                if !(CalculateGravity(PlanetElementPtr).0 != 0
                    || uqm_lua_checked!(TimeSpaceMatterConflict(PlanetElementPtr), ()).0 != 0)
                {
                    break;
                }
            }
        }
        (*PlanetElementPtr).c2rust_unnamed_1.mass_points = (*PlanetElementPtr)
            .c2rust_unnamed_0
            .hit_points as BYTE;
        pt = (*PlanetElementPtr).current.location;
        UnlockLink(&raw mut disp_q, hPlanetElement);
        PutQueue(&raw mut disp_q, hPlanetElement);
    }
    if if GlobData.SIS_state.Extended as ::core::ffi::c_int != 0 {
        BOOLEAN::TRUE.0 as ::core::ffi::c_int
    } else {
        BOOLEAN::FALSE.0 as ::core::ffi::c_int
    } != 0
        && uqm_lua_checked!(getGameStateUint(
            b"URQUAN_PROTECTING_SAMATRA\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) != 0
    {
        pt.x = (pt.x as ::core::ffi::c_int + 1200 as ::core::ffi::c_int) as COORD;
        pt.y = (pt.y as ::core::ffi::c_int + 780 as ::core::ffi::c_int) as COORD;
        pt.x = (if (pt.x as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
            pt.x as ::core::ffi::c_int
                + (((CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
        } else if pt.x as ::core::ffi::c_int
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
            pt.x as ::core::ffi::c_int
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
        } else {
            pt.x as ::core::ffi::c_int
        }) as DWORD as COORD;
        pt.y = (if (pt.y as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
            pt.y as ::core::ffi::c_int
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
        } else if pt.y as ::core::ffi::c_int
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
            pt.y as ::core::ffi::c_int
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
            pt.y as ::core::ffi::c_int
        }) as DWORD as COORD;
        SetStarPoint(pt, 31 as COUNT);
    }
}
unsafe extern "C" fn spawn_rubble(mut AsteroidElementPtr: *mut ELEMENT) {
    let mut hRubbleElement: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
    hRubbleElement = AllocElement();
    if !hRubbleElement.is_null() {
        let mut RubbleElementPtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
        PutQueue(&raw mut disp_q, hRubbleElement);
        RubbleElementPtr = LockLink(&raw mut disp_q, hRubbleElement) as *mut ELEMENT;
        (*RubbleElementPtr).playerNr = (*AsteroidElementPtr).playerNr;
        (*RubbleElementPtr).state_flags = ((1 as ::core::ffi::c_int)
            << 3 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as ELEMENT_FLAGS;
        (*RubbleElementPtr).c2rust_unnamed.life_span = 5 as COUNT;
        (*RubbleElementPtr).c2rust_unnamed_3.next_turn = 0 as BYTE;
        (*RubbleElementPtr).c2rust_unnamed_2.turn_wait = (*RubbleElementPtr)
            .c2rust_unnamed_3
            .next_turn;
        DisplayArray[(*RubbleElementPtr).PrimIndex as usize].Type = gfx_object::STAMP_PRIM
            .0 as ::core::ffi::c_int as GRAPHICS_PRIM;
        (*RubbleElementPtr).current.image.farray = &raw mut asteroid as *mut FRAME;
        if if resolutionFactor != 2 as ::core::ffi::c_uint {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } == 0
        {
            (*RubbleElementPtr).current.image.frame = SetAbsFrameIndex(
                *(&raw mut asteroid as *mut FRAME).offset(0isize),
                (((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                    + ((1 as ::core::ffi::c_int)
                        << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int)
                    >> 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int) as COUNT,
            );
        } else {
            (*RubbleElementPtr).current.image.frame = SetAbsFrameIndex(
                *(&raw mut asteroid as *mut FRAME).offset(0isize),
                29 as COUNT,
            );
        }
        (*RubbleElementPtr).current.location = (*AsteroidElementPtr).current.location;
        (*RubbleElementPtr).preprocess_func = Some(
            animation_preprocess as unsafe extern "C" fn(*mut ELEMENT) -> (),
        ) as Option<ElementProcessFunc>;
        (*RubbleElementPtr).death_func = Some(
            spawn_asteroid as unsafe extern "C" fn(*mut ELEMENT) -> (),
        ) as Option<ElementProcessFunc>;
        UnlockLink(&raw mut disp_q, hRubbleElement);
    }
}
unsafe extern "C" fn asteroid_preprocess(mut ElementPtr: *mut ELEMENT) {
    if (*ElementPtr).c2rust_unnamed_2.turn_wait as ::core::ffi::c_int
        > 0 as ::core::ffi::c_int
    {
        (*ElementPtr).c2rust_unnamed_2.turn_wait = (*ElementPtr)
            .c2rust_unnamed_2
            .turn_wait
            .wrapping_sub(1);
    } else {
        let mut frame_index: COUNT = 0;
        frame_index = GetFrameIndex((*ElementPtr).current.image.frame);
        if (*ElementPtr).c2rust_unnamed_3.thrust_wait as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int != 0
        {
            frame_index = frame_index.wrapping_sub(1);
        } else {
            frame_index = frame_index.wrapping_add(1);
        }
        if if resolutionFactor != 2 as ::core::ffi::c_uint {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } == 0
        {
            (*ElementPtr).next.image.frame = SetAbsFrameIndex(
                (*ElementPtr).current.image.frame,
                (frame_index as ::core::ffi::c_int
                    & ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int) as DWORD as COUNT,
            );
        } else {
            (*ElementPtr).next.image.frame = SetAbsFrameIndex(
                (*ElementPtr).current.image.frame,
                (frame_index as ::core::ffi::c_int % 30 as ::core::ffi::c_int) as COUNT,
            );
        }
        (*ElementPtr).state_flags = ((*ElementPtr).state_flags as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as ELEMENT_FLAGS;
        (*ElementPtr).c2rust_unnamed_2.turn_wait = ((*ElementPtr)
            .c2rust_unnamed_3
            .thrust_wait as ::core::ffi::c_int
            & ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int)
                - 1 as ::core::ffi::c_int) as ::core::ffi::c_uchar as BYTE;
    };
}
#[no_mangle]
pub unsafe extern "C" fn spawn_asteroid(mut ElementPtr: *mut ELEMENT) {
    let mut hAsteroidElement: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
    hAsteroidElement = AllocElement();
    if hAsteroidElement.is_null() {
        if !ElementPtr.is_null() {
            (*ElementPtr).state_flags = ((*ElementPtr).state_flags as ::core::ffi::c_int
                & !((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int))
                as ELEMENT_FLAGS;
            DisplayArray[(*ElementPtr).PrimIndex as usize].Type = gfx_object::NUM_PRIMS.0
                as ::core::ffi::c_int as GRAPHICS_PRIM;
            (*ElementPtr).c2rust_unnamed.life_span = 1 as COUNT;
        }
    } else {
        let mut AsteroidElementPtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
        let mut val: COUNT = 0;
        AsteroidElementPtr = LockLink(&raw mut disp_q, hAsteroidElement) as *mut ELEMENT;
        (*AsteroidElementPtr).playerNr = -1 as SIZE;
        (*AsteroidElementPtr).c2rust_unnamed_0.hit_points = 1 as COUNT;
        (*AsteroidElementPtr).c2rust_unnamed_1.mass_points = 3 as BYTE;
        (*AsteroidElementPtr).state_flags = (if optMeleeObstacles.0 != 0
            && isNetwork().0 == 0
        {
            (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
        } else {
            (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
        }) as ELEMENT_FLAGS;
        (*AsteroidElementPtr).c2rust_unnamed.life_span = 1 as COUNT;
        DisplayArray[(*AsteroidElementPtr).PrimIndex as usize].Type = gfx_object::STAMP_PRIM
            .0 as ::core::ffi::c_int as GRAPHICS_PRIM;
        val = TFB_Random() as COUNT;
        if val as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int != 0
        {
            if val as ::core::ffi::c_int
                & (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int == 0
            {
                (*AsteroidElementPtr).current.location.x = 0 as COORD;
            } else {
                (*AsteroidElementPtr).current.location.x = (((CanvasWidth
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
                    << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as COORD;
            }
            (*AsteroidElementPtr).current.location.y = (if TFB_Random()
                .wrapping_rem(
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
                        << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as DWORD,
                ) as ::core::ffi::c_int
                & !(((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
            {
                (TFB_Random()
                    .wrapping_rem(
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
                            << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                            as DWORD,
                    ) as ::core::ffi::c_int
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
            } else if TFB_Random()
                .wrapping_rem(
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
                        << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as DWORD,
                ) as ::core::ffi::c_int
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
                (TFB_Random()
                    .wrapping_rem(
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
                            << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                            as DWORD,
                    ) as ::core::ffi::c_int
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
                TFB_Random()
                    .wrapping_rem(
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
                            << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                            as DWORD,
                    ) as ::core::ffi::c_int
                    & !(((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int)
            }) as DWORD as COORD;
        } else {
            (*AsteroidElementPtr).current.location.x = (if TFB_Random()
                .wrapping_rem(
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
                        << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as DWORD,
                ) as ::core::ffi::c_int
                & !(((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
            {
                (TFB_Random()
                    .wrapping_rem(
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
                            << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                            as DWORD,
                    ) as ::core::ffi::c_int
                    & !(((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int))
                    + (((CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
            } else if TFB_Random()
                .wrapping_rem(
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
                        << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as DWORD,
                ) as ::core::ffi::c_int
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
                (TFB_Random()
                    .wrapping_rem(
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
                            << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                            as DWORD,
                    ) as ::core::ffi::c_int
                    & !(((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int))
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
            } else {
                TFB_Random()
                    .wrapping_rem(
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
                            << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                            as DWORD,
                    ) as ::core::ffi::c_int
                    & !(((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int)
            }) as DWORD as COORD;
            if val as ::core::ffi::c_int
                & (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int == 0
            {
                (*AsteroidElementPtr).current.location.y = 0 as COORD;
            } else {
                (*AsteroidElementPtr).current.location.y = (((CanvasHeight
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
                    << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as COORD;
            }
        }
        let mut magnitude: SIZE = (((TFB_Random() as SIZE as ::core::ffi::c_int
            & 7 as ::core::ffi::c_int) + 4 as ::core::ffi::c_int)
            << 2 as ::core::ffi::c_int) as SIZE;
        let mut facing: COUNT = TFB_Random() as COUNT;
        SetVelocityVector(
            &raw mut (*AsteroidElementPtr).velocity,
            magnitude as SDWORD,
            facing,
        );
        (*AsteroidElementPtr).current.image.farray = &raw mut asteroid as *mut FRAME;
        (*AsteroidElementPtr).current.image.frame = SetAbsFrameIndex(
            *(&raw mut asteroid as *mut FRAME).offset(0isize),
            (TFB_Random()
                & (((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int) as DWORD) as COUNT,
        );
        (*AsteroidElementPtr).c2rust_unnamed_3.thrust_wait = (TFB_Random() as BYTE
            as ::core::ffi::c_int
            & (((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                - 1 as ::core::ffi::c_int) as BYTE as ::core::ffi::c_int) as BYTE;
        (*AsteroidElementPtr).c2rust_unnamed_2.turn_wait = (*AsteroidElementPtr)
            .c2rust_unnamed_3
            .thrust_wait;
        (*AsteroidElementPtr).c2rust_unnamed_3.thrust_wait = ((*AsteroidElementPtr)
            .c2rust_unnamed_3
            .thrust_wait as ::core::ffi::c_int
            | TFB_Random() as BYTE as ::core::ffi::c_int
                & ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as BYTE
                    as ::core::ffi::c_int) as BYTE;
        (*AsteroidElementPtr).preprocess_func = Some(
            asteroid_preprocess as unsafe extern "C" fn(*mut ELEMENT) -> (),
        ) as Option<ElementProcessFunc>;
        (*AsteroidElementPtr).death_func = Some(
            spawn_rubble as unsafe extern "C" fn(*mut ELEMENT) -> (),
        ) as Option<ElementProcessFunc>;
        (*AsteroidElementPtr).collision_func = Some(
            collision
                as unsafe extern "C" fn(
                    *mut ELEMENT,
                    *mut POINT,
                    *mut ELEMENT,
                    *mut POINT,
                ) -> (),
        ) as Option<ElementCollisionFunc>;
        UnlockLink(&raw mut disp_q, hAsteroidElement);
        PutQueue(&raw mut disp_q, hAsteroidElement);
    };
}
#[no_mangle]
pub unsafe extern "C" fn do_damage(mut ElementPtr: *mut ELEMENT, mut damage: SIZE) {
    if antiCheat(
            ElementPtr,
            BOOLEAN::FALSE,
            C2Rust_Unnamed::OPTVAL_INF_HEALTH.0 as ::core::ffi::c_int,
        )
        .0 != 0
        || antiCheat(
                ElementPtr,
                BOOLEAN::FALSE,
                C2Rust_Unnamed::OPTVAL_FULL_GOD.0 as ::core::ffi::c_int,
            )
            .0 != 0
    {
        damage = 0 as SIZE;
    }
    if (*ElementPtr).state_flags as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int != 0
    {
        if uqm_lua_checked!(DeltaCrew(ElementPtr, -(damage as ::core::ffi::c_int) as SIZE), ()).0 == 0 {
            (*ElementPtr).c2rust_unnamed.life_span = 0 as COUNT;
            (*ElementPtr).state_flags = ((*ElementPtr).state_flags as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as ELEMENT_FLAGS;
        }
    } else if !((*ElementPtr).c2rust_unnamed_1.mass_points as ::core::ffi::c_int
        > 10 as ::core::ffi::c_int * 10 as ::core::ffi::c_int)
    {
        if (damage as BYTE as ::core::ffi::c_int)
            < (*ElementPtr).c2rust_unnamed_0.hit_points as ::core::ffi::c_int
        {
            (*ElementPtr).c2rust_unnamed_0.hit_points = ((*ElementPtr)
                .c2rust_unnamed_0
                .hit_points as ::core::ffi::c_int - damage as BYTE as ::core::ffi::c_int)
                as COUNT;
        } else {
            (*ElementPtr).c2rust_unnamed_0.hit_points = 0 as COUNT;
            (*ElementPtr).c2rust_unnamed.life_span = 0 as COUNT;
            (*ElementPtr).state_flags = ((*ElementPtr).state_flags as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as ELEMENT_FLAGS;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn crew_preprocess(mut ElementPtr: *mut ELEMENT) {
    let mut hTarget: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut oldColor: Color = DisplayArray[(*ElementPtr).PrimIndex as usize].color;
    let mut newColor: Color = if sameColor(
        oldColor,
        colorFromRgb15(0 as BYTE, 0x14 as BYTE, 0 as BYTE),
    ) as ::core::ffi::c_int != 0
    {
        colorFromRgb15(0xa as BYTE, 0x1e as BYTE, 0xa as BYTE)
    } else {
        colorFromRgb15(0 as BYTE, 0x14 as BYTE, 0 as BYTE)
    };
    DisplayArray[(*ElementPtr).PrimIndex as usize].color = newColor;
    (*ElementPtr).state_flags = ((*ElementPtr).state_flags as ::core::ffi::c_int
        | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as ELEMENT_FLAGS;
    hTarget = (*ElementPtr).hTarget;
    if hTarget.is_null() {
        let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
        StarShipPtr = (*ElementPtr).pParent as *mut STARSHIP;
        if !StarShipPtr.is_null()
            && (*(*StarShipPtr).RaceDescPtr).ship_info.crew_level as ::core::ffi::c_int
                != 0
        {
            (*ElementPtr).hTarget = (*StarShipPtr).hShip;
        } else {
            let mut facing: COUNT = 0;
            facing = 0 as COUNT;
            TrackShip(ElementPtr, &raw mut facing);
        }
    }
    if !hTarget.is_null() {
        let mut delta: SIZE = 0;
        let mut ShipPtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
        ShipPtr = LockLink(&raw mut disp_q, hTarget) as *mut ELEMENT;
        delta = ((*ShipPtr).current.location.x as ::core::ffi::c_int
            - (*ElementPtr).current.location.x as ::core::ffi::c_int) as SIZE;
        delta = (if (delta as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
            if -(delta as ::core::ffi::c_int)
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
                delta as ::core::ffi::c_int
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
                    << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                    + delta as ::core::ffi::c_int
            }
        } else if delta as ::core::ffi::c_int
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
            delta as ::core::ffi::c_int
        } else {
            delta as ::core::ffi::c_int
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
        }) as SIZE;
        if delta as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
            (*ElementPtr).next.location.x = ((*ElementPtr).next.location.x
                as ::core::ffi::c_int
                + (((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                    << resolutionFactor)) as COORD;
        } else if (delta as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
            (*ElementPtr).next.location.x = ((*ElementPtr).next.location.x
                as ::core::ffi::c_int
                - (((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                    << resolutionFactor)) as COORD;
        }
        delta = ((*ShipPtr).current.location.y as ::core::ffi::c_int
            - (*ElementPtr).current.location.y as ::core::ffi::c_int) as SIZE;
        delta = (if (delta as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
            if -(delta as ::core::ffi::c_int)
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
                delta as ::core::ffi::c_int
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
                    << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                    + delta as ::core::ffi::c_int
            }
        } else if delta as ::core::ffi::c_int
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
            delta as ::core::ffi::c_int
        } else {
            delta as ::core::ffi::c_int
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
        }) as SIZE;
        if delta as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
            (*ElementPtr).next.location.y = ((*ElementPtr).next.location.y
                as ::core::ffi::c_int
                + (((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                    << resolutionFactor)) as COORD;
        } else if (delta as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
            (*ElementPtr).next.location.y = ((*ElementPtr).next.location.y
                as ::core::ffi::c_int
                - (((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                    << resolutionFactor)) as COORD;
        }
        UnlockLink(&raw mut disp_q, hTarget);
    }
}
#[no_mangle]
pub unsafe extern "C" fn crew_collision(
    mut ElementPtr0: *mut ELEMENT,
    mut pPt0: *mut POINT,
    mut ElementPtr1: *mut ELEMENT,
    mut pPt1: *mut POINT,
) {
    if (*ElementPtr1).state_flags as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int != 0
        && (*ElementPtr1).c2rust_unnamed.life_span as ::core::ffi::c_int
            >= 1 as ::core::ffi::c_int
        && (*ElementPtr0).c2rust_unnamed_0.hit_points as ::core::ffi::c_int
            > 0 as ::core::ffi::c_int
    {
        let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
        StarShipPtr = (*ElementPtr1).pParent as *mut STARSHIP;
        if (*(*StarShipPtr).RaceDescPtr).ship_info.ship_flags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int == 0
        {
            ProcessSound(
                SetAbsStringTableIndex(
                    GameSounds,
                    C2Rust_Unnamed_6::GRAB_CREW.0 as ::core::ffi::c_int as COUNT,
                ),
                ElementPtr1,
            );
            uqm_lua_checked!(DeltaCrew(ElementPtr1, 1 as SIZE), ());
        }
    }
    (*ElementPtr0).c2rust_unnamed_0.hit_points = 0 as COUNT;
    (*ElementPtr0).c2rust_unnamed.life_span = 0 as COUNT;
    (*ElementPtr0).state_flags = ((*ElementPtr0).state_flags as ::core::ffi::c_int
        | ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)) as ELEMENT_FLAGS;
}
#[no_mangle]
pub unsafe extern "C" fn AbandonShip(
    mut ShipPtr: *mut ELEMENT,
    mut TargetPtr: *mut ELEMENT,
    mut crew_loss: COUNT,
) {
    let mut dx: SIZE = 0;
    let mut dy: SIZE = 0;
    let mut direction: COUNT = 0;
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
    let mut hCrew: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut ShipIntersect: INTERSECT_CONTROL = INTERSECT_CONTROL {
        last_time_val: 0,
        EndPoint: POINT { x: 0, y: 0 },
        IntersectStamp: STAMP {
            origin: POINT { x: 0, y: 0 },
            frame: ::core::ptr::null_mut::<FRAME_DESC>(),
        },
    };
    StarShipPtr = (*ShipPtr).pParent as *mut STARSHIP;
    if (*(*StarShipPtr).RaceDescPtr).ship_info.ship_flags as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int != 0
    {
        return;
    }
    ShipIntersect = (*ShipPtr).IntersectControl;
    GetFrameRect(ShipIntersect.IntersectStamp.frame, &raw mut r);
    direction = (*ShipPtr).velocity.TravelAngle;
    if direction as ::core::ffi::c_int
        == (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
    {
        dy = 0 as SIZE;
        dx = dy;
    } else {
        direction = (direction as ::core::ffi::c_int
            + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int)) as COUNT;
        dx = (*(&raw mut sinetab as *mut SDWORD)
            .offset(
                (direction as ::core::ffi::c_int
                    + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                        >> 2 as ::core::ffi::c_int)
                    & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int) as DWORD as isize,
            ) as ::core::ffi::c_long
            * ((100 as ::core::ffi::c_int) << resolutionFactor) as ::core::ffi::c_long
            >> 14 as ::core::ffi::c_int) as SDWORD as SIZE;
        dy = (*(&raw mut sinetab as *mut SDWORD)
            .offset(
                (direction as ::core::ffi::c_int
                    & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int) as DWORD as isize,
            ) as ::core::ffi::c_long
            * ((100 as ::core::ffi::c_int) << resolutionFactor) as ::core::ffi::c_long
            >> 14 as ::core::ffi::c_int) as SDWORD as SIZE;
    }
    loop {
        let c2rust_fresh0 = crew_loss;
        crew_loss = crew_loss.wrapping_sub(1);
        if !(c2rust_fresh0 as ::core::ffi::c_int != 0
            && {
                hCrew = AllocElement();
                !hCrew.is_null()
            })
        {
            break;
        }
        let mut CrewPtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
        uqm_lua_checked!(DeltaCrew(ShipPtr, -1 as SIZE), ());
        PutQueue(&raw mut disp_q, hCrew);
        CrewPtr = LockLink(&raw mut disp_q, hCrew) as *mut ELEMENT;
        (*CrewPtr).playerNr = -1 as SIZE;
        (*CrewPtr).c2rust_unnamed_0.hit_points = 1 as COUNT;
        (*CrewPtr).state_flags = ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int) as ELEMENT_FLAGS;
        (*CrewPtr).c2rust_unnamed.life_span = 300 as COUNT;
        if if resolutionFactor != 2 as ::core::ffi::c_uint {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } == 0
        {
            DisplayArray[(*CrewPtr).PrimIndex as usize].Type = gfx_object::POINT_PRIM.0
                as ::core::ffi::c_int as GRAPHICS_PRIM;
            (*CrewPtr).current.image.frame = DecFrameIndex(stars_in_space);
            (*CrewPtr).current.image.farray = &raw mut stars_in_space;
        } else {
            DisplayArray[(*CrewPtr).PrimIndex as usize].Type = gfx_object::STAMPFILL_PRIM
                .0 as ::core::ffi::c_int as GRAPHICS_PRIM;
            (*CrewPtr).current.image.frame = SetAbsFrameIndex(
                stars_misc[0usize],
                0 as COUNT,
            );
            (*CrewPtr).current.image.farray = &raw mut stars_misc as *mut FRAME;
        }
        DisplayArray[(*CrewPtr).PrimIndex as usize].color = colorFromRgb15(
            0 as BYTE,
            0x14 as BYTE,
            0 as BYTE,
        );
        (*CrewPtr).preprocess_func = Some(
            crew_preprocess as unsafe extern "C" fn(*mut ELEMENT) -> (),
        ) as Option<ElementProcessFunc>;
        (*CrewPtr).collision_func = Some(
            crew_collision
                as unsafe extern "C" fn(
                    *mut ELEMENT,
                    *mut POINT,
                    *mut ELEMENT,
                    *mut POINT,
                ) -> (),
        ) as Option<ElementCollisionFunc>;
        (*CrewPtr).pParent = StarShipPtr as *mut ::core::ffi::c_void;
        StarShipPtr = (*TargetPtr).pParent as *mut STARSHIP;
        (*CrewPtr).hTarget = (*StarShipPtr).hShip;
        let mut w: SIZE = 0;
        let mut h: SIZE = 0;
        let mut CrewIntersect: INTERSECT_CONTROL = INTERSECT_CONTROL {
            last_time_val: 0,
            EndPoint: POINT { x: 0, y: 0 },
            IntersectStamp: STAMP {
                origin: POINT { x: 0, y: 0 },
                frame: ::core::ptr::null_mut::<FRAME_DESC>(),
            },
        };
        ShipIntersect.IntersectStamp.origin = (*ShipPtr).IntersectControl.EndPoint;
        w = (TFB_Random() as COUNT as ::core::ffi::c_int
            % r.extent.width as ::core::ffi::c_int) as SIZE;
        h = (TFB_Random() as COUNT as ::core::ffi::c_int
            % r.extent.height as ::core::ffi::c_int) as SIZE;
        CrewIntersect.EndPoint = ShipIntersect.EndPoint;
        CrewIntersect.IntersectStamp.frame = DecFrameIndex(stars_in_space);
        if dx as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            && dy as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        {
            CrewIntersect.EndPoint.x = (CrewIntersect.EndPoint.x as ::core::ffi::c_int
                + (w as ::core::ffi::c_int
                    - (r.extent.width as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)))
                as COORD;
            CrewIntersect.EndPoint.y = (CrewIntersect.EndPoint.y as ::core::ffi::c_int
                + (h as ::core::ffi::c_int
                    - (r.extent.height as ::core::ffi::c_int
                        >> 1 as ::core::ffi::c_int))) as COORD;
            CrewIntersect.IntersectStamp.origin = (*TargetPtr).IntersectControl.EndPoint;
        } else {
            if dx as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                CrewIntersect.EndPoint.x = (CrewIntersect.EndPoint.x
                    as ::core::ffi::c_int
                    + (w as ::core::ffi::c_int
                        - (r.extent.width as ::core::ffi::c_int
                            >> 1 as ::core::ffi::c_int))) as COORD;
            } else if dx as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                CrewIntersect.EndPoint.x = (CrewIntersect.EndPoint.x
                    as ::core::ffi::c_int + w as ::core::ffi::c_int) as COORD;
            } else {
                CrewIntersect.EndPoint.x = (CrewIntersect.EndPoint.x
                    as ::core::ffi::c_int - w as ::core::ffi::c_int) as COORD;
            }
            if dy as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                CrewIntersect.EndPoint.y = (CrewIntersect.EndPoint.y
                    as ::core::ffi::c_int
                    + (h as ::core::ffi::c_int
                        - (r.extent.height as ::core::ffi::c_int
                            >> 1 as ::core::ffi::c_int))) as COORD;
            } else if dy as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                CrewIntersect.EndPoint.y = (CrewIntersect.EndPoint.y
                    as ::core::ffi::c_int + h as ::core::ffi::c_int) as COORD;
            } else {
                CrewIntersect.EndPoint.y = (CrewIntersect.EndPoint.y
                    as ::core::ffi::c_int - h as ::core::ffi::c_int) as COORD;
            }
            CrewIntersect.IntersectStamp.origin.x = (CrewIntersect.EndPoint.x
                as ::core::ffi::c_int + dx as ::core::ffi::c_int) as COORD;
            CrewIntersect.IntersectStamp.origin.y = (CrewIntersect.EndPoint.y
                as ::core::ffi::c_int + dy as ::core::ffi::c_int) as COORD;
        }
        uqm_lua_checked!(DrawablesIntersect(
            &raw mut CrewIntersect,
            &raw mut ShipIntersect,
            (((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
                + 1 as ::core::ffi::c_int) as TIME_VALUE,
        ), ());
        (*CrewPtr).current.location.x = ((CrewIntersect.EndPoint.x as ::core::ffi::c_int)
            << 2 as ::core::ffi::c_int) as COORD;
        (*CrewPtr).current.location.y = ((CrewIntersect.EndPoint.y as ::core::ffi::c_int)
            << 2 as ::core::ffi::c_int) as COORD;
        UnlockLink(&raw mut disp_q, hCrew);
    };
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
#[inline]
unsafe extern "C" fn isNetwork() -> BOOLEAN {
    return BOOLEAN(
        (*(&raw mut PlayerControl as *mut BYTE).offset(0isize) as ::core::ffi::c_int
            & ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as BYTE
                as ::core::ffi::c_int != 0
            || *(&raw mut PlayerControl as *mut BYTE).offset(1isize)
                as ::core::ffi::c_int
                & ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as BYTE
                    as ::core::ffi::c_int != 0) as ::core::ffi::c_int
            as ::core::ffi::c_uint,
    );
}
