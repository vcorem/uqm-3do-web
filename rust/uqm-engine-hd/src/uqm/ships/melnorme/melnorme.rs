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

// Rust translation of src/uqm/ships/melnorme/melnorme.c.
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
    fn GetFrameRect(Frame: FRAME, pRect: *mut RECT) -> BOOLEAN;
    fn GetFrameIndex(Frame: FRAME) -> COUNT;
    fn SetAbsFrameIndex(Frame: FRAME, FrameIndex: COUNT) -> FRAME;
    fn SetRelFrameIndex(Frame: FRAME, FrameOffs: SIZE) -> FRAME;
    fn IncFrameIndex(Frame: FRAME) -> FRAME;
    fn SetAbsStringTableIndex(String: STRING, StringTableIndex: COUNT) -> STRING;
    fn memset(
        _: *mut ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    static mut resolutionFactor: ::core::ffi::c_uint;
    static mut optGodModes: ::core::ffi::c_int;
    static mut sinetab: [SDWORD; 0];
    fn SetVelocityComponents(velocityptr: *mut VELOCITY_DESC, dx: SDWORD, dy: SDWORD);
    static mut disp_q: QUEUE;
    fn AllocElement() -> HELEMENT;
    static mut GlobData: GLOBDATA;
    fn TFB_Random() -> DWORD;
    static mut PlayerControl: [BYTE; 0];
    fn ProcessSound(Sound: SOUND, PositionalObject: *mut ELEMENT);
    fn DeltaEnergy(ElementPtr: *mut ELEMENT, energy_delta: SIZE) -> BOOLEAN;
    fn initialize_missile(pMissileBlock: *mut MISSILE_BLOCK) -> HELEMENT;
    fn weapon_collision(
        ElementPtr0: *mut ELEMENT,
        pPt0: *mut POINT,
        ElementPtr1: *mut ELEMENT,
        pPt1: *mut POINT,
    ) -> HELEMENT;
    fn ship_intelligence(
        ShipPtr: *mut ELEMENT,
        ObjectsOfConcern: *mut EVALUATE_DESC,
        ConcernCounter: COUNT,
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
static mut melnorme_desc: RACE_DESC = unsafe {
    race_desc {
        c2rust_padding: [0; 8],
        c2rust_padding_0: [0; 10],
        c2rust_padding_1: [0; 2],
        c2rust_padding_2: [0; 8],
        ship_info: SHIP_INFO {
            idStr: b"trader\0".as_ptr() as *const ::core::ffi::c_char,
            ship_flags: ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as UWORD,
            ship_cost: 18 as BYTE,
            crew_level: 20 as COUNT,
            max_crew: 20 as COUNT,
            energy_level: 42 as BYTE,
            max_energy: 42 as BYTE,
            race_strings_rsc: b"ship.melnorme.text\0".as_ptr() as RESOURCE,
            icons_rsc: b"ship.melnorme.icons\0".as_ptr() as RESOURCE,
            melee_icon_rsc: b"ship.melnorme.meleeicons\0".as_ptr() as RESOURCE,
            race_strings: ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>(),
            icons: ::core::ptr::null_mut::<FRAME_DESC>(),
            melee_icon: ::core::ptr::null_mut::<FRAME_DESC>(),
        },
        fleet: FLEET_STUFF {
            strength: !(0 as ::core::ffi::c_int) as COUNT,
            known_loc: point {
                x: (9999 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as COORD,
                y: (9999 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as COORD,
            },
        },
        characteristics: CHARACTERISTIC_STUFF {
            max_thrust: 36 as COUNT,
            thrust_increment: 6 as COUNT,
            energy_regeneration: 1 as BYTE,
            weapon_energy_cost: 5 as BYTE,
            special_energy_cost: 20 as BYTE,
            energy_wait: 4 as BYTE,
            turn_wait: 4 as BYTE,
            thrust_wait: 4 as BYTE,
            weapon_wait: 1 as BYTE,
            special_wait: 20 as BYTE,
            ship_mass: 7 as BYTE,
        },
        ship_data: DATA_STUFF {
            ship_rsc: [
                b"ship.melnorme.graphics.trader.large\0".as_ptr() as RESOURCE,
                b"ship.melnorme.graphics.trader.medium\0".as_ptr() as RESOURCE,
                b"ship.melnorme.graphics.trader.small\0".as_ptr() as RESOURCE,
            ],
            weapon_rsc: [
                b"ship.melnorme.graphics.pumpup.large\0".as_ptr() as RESOURCE,
                b"ship.melnorme.graphics.pumpup.medium\0".as_ptr() as RESOURCE,
                b"ship.melnorme.graphics.pumpup.small\0".as_ptr() as RESOURCE,
            ],
            special_rsc: [
                b"ship.melnorme.graphics.confuse.large\0".as_ptr() as RESOURCE,
                b"ship.melnorme.graphics.confuse.medium\0".as_ptr() as RESOURCE,
                b"ship.melnorme.graphics.confuse.small\0".as_ptr() as RESOURCE,
            ],
            captain_control: captain_stuff {
                captain_rsc: b"ship.melnorme.graphics.captain\0".as_ptr() as RESOURCE,
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
            victory_ditty_rsc: b"ship.melnorme.ditty\0".as_ptr() as RESOURCE,
            ship_sounds_rsc: b"ship.melnorme.sounds\0".as_ptr() as RESOURCE,
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
            WeaponRange: (((45 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
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
unsafe extern "C" fn pump_up_preprocess(mut ElementPtr: *mut ELEMENT) {
    (*ElementPtr).c2rust_unnamed_3.thrust_wait = (*ElementPtr)
        .c2rust_unnamed_3
        .thrust_wait
        .wrapping_sub(1);
    if (*ElementPtr).c2rust_unnamed_3.thrust_wait as ::core::ffi::c_int
        & 1 as ::core::ffi::c_int != 0
    {
        let mut frame_index: COUNT = 0;
        frame_index = GetFrameIndex((*ElementPtr).current.image.frame);
        if (*ElementPtr).c2rust_unnamed_2.turn_wait as ::core::ffi::c_int
            & ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as BYTE
                as ::core::ffi::c_int != 0
            && frame_index as ::core::ffi::c_int % 5 as ::core::ffi::c_int
                != 0 as ::core::ffi::c_int
            || (*ElementPtr).c2rust_unnamed_2.turn_wait as ::core::ffi::c_int
                & ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as BYTE
                    as ::core::ffi::c_int == 0
                && (frame_index as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                    % 5 as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        {
            frame_index = frame_index.wrapping_sub(1);
            (*ElementPtr).c2rust_unnamed_2.turn_wait = ((*ElementPtr)
                .c2rust_unnamed_2
                .turn_wait as ::core::ffi::c_int
                | ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as BYTE
                    as ::core::ffi::c_int) as BYTE;
        } else {
            frame_index = frame_index.wrapping_add(1);
            (*ElementPtr).c2rust_unnamed_2.turn_wait = ((*ElementPtr)
                .c2rust_unnamed_2
                .turn_wait as ::core::ffi::c_int
                & !(((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as BYTE
                    as ::core::ffi::c_int)) as BYTE;
        }
        (*ElementPtr).next.image.frame = SetAbsFrameIndex(
            (*ElementPtr).current.image.frame,
            frame_index,
        );
        (*ElementPtr).state_flags = ((*ElementPtr).state_flags as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as ELEMENT_FLAGS;
    }
}
unsafe extern "C" fn pump_up_postprocess(mut ElementPtr: *mut ELEMENT) {
    if (*ElementPtr).state_flags as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int != 0
    {
        memset(
            &raw mut (*ElementPtr).velocity as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<VELOCITY_DESC>(),
        );
    } else {
        let mut hPumpUp: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
        let mut EPtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
        let mut ShipPtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
        let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
        StarShipPtr = (*ElementPtr).pParent as *mut STARSHIP;
        ShipPtr = LockLink(&raw mut disp_q, (*StarShipPtr).hShip) as *mut ELEMENT;
        initialize_pump_up(ShipPtr, &raw mut hPumpUp);
        uqm_lua_checked!(DeltaEnergy(ShipPtr, 0 as SIZE), ());
        UnlockLink(&raw mut disp_q, (*StarShipPtr).hShip);
        EPtr = LockLink(&raw mut disp_q, hPumpUp) as *mut ELEMENT;
        (*EPtr).current.image.frame = (*ElementPtr).current.image.frame;
        (*EPtr).c2rust_unnamed_2.turn_wait = (*ElementPtr).c2rust_unnamed_2.turn_wait;
        (*EPtr).c2rust_unnamed_3.thrust_wait = (*ElementPtr)
            .c2rust_unnamed_3
            .thrust_wait;
        (*EPtr).c2rust_unnamed_3.thrust_wait = (*EPtr)
            .c2rust_unnamed_3
            .thrust_wait
            .wrapping_sub(1);
        if (*EPtr).c2rust_unnamed_3.thrust_wait as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
            if (*EPtr).c2rust_unnamed_2.turn_wait as ::core::ffi::c_int
                & !(((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as BYTE
                    as ::core::ffi::c_int)
                < 4 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
            {
                (*EPtr).c2rust_unnamed_2.turn_wait = (*EPtr)
                    .c2rust_unnamed_2
                    .turn_wait
                    .wrapping_add(1);
                (*EPtr).current.image.frame = SetRelFrameIndex(
                    (*EPtr).current.image.frame,
                    5 as SIZE,
                );
                ProcessSound(
                    SetAbsStringTableIndex(
                        (*(*StarShipPtr).RaceDescPtr).ship_data.ship_sounds,
                        0 as COUNT,
                    ),
                    EPtr,
                );
            }
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
                (*EPtr).c2rust_unnamed_3.thrust_wait = 5 as BYTE;
            } else {
                (*EPtr).c2rust_unnamed_3.thrust_wait = 72 as BYTE;
            }
        }
        (*EPtr).c2rust_unnamed_0.hit_points = ((2 as ::core::ffi::c_int)
            << ((*ElementPtr).c2rust_unnamed_2.turn_wait as ::core::ffi::c_int
                & !(((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as BYTE
                    as ::core::ffi::c_int))) as COUNT;
        (*EPtr).c2rust_unnamed_1.mass_points = (*EPtr).c2rust_unnamed_0.hit_points
            as BYTE;
        (*EPtr).pParent = StarShipPtr as *mut ::core::ffi::c_void;
        if (*EPtr).c2rust_unnamed_3.thrust_wait as ::core::ffi::c_int
            & 1 as ::core::ffi::c_int != 0
        {
            let mut frame_index: COUNT = 0;
            frame_index = GetFrameIndex((*EPtr).current.image.frame);
            if (*EPtr).c2rust_unnamed_2.turn_wait as ::core::ffi::c_int
                & ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as BYTE
                    as ::core::ffi::c_int != 0
                && frame_index as ::core::ffi::c_int % 5 as ::core::ffi::c_int
                    != 0 as ::core::ffi::c_int
                || (*EPtr).c2rust_unnamed_2.turn_wait as ::core::ffi::c_int
                    & ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as BYTE
                        as ::core::ffi::c_int == 0
                    && (frame_index as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                        % 5 as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            {
                frame_index = frame_index.wrapping_sub(1);
                (*EPtr).c2rust_unnamed_2.turn_wait = ((*EPtr).c2rust_unnamed_2.turn_wait
                    as ::core::ffi::c_int
                    | ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as BYTE
                        as ::core::ffi::c_int) as BYTE;
            } else {
                frame_index = frame_index.wrapping_add(1);
                (*EPtr).c2rust_unnamed_2.turn_wait = ((*EPtr).c2rust_unnamed_2.turn_wait
                    as ::core::ffi::c_int
                    & !(((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as BYTE
                        as ::core::ffi::c_int)) as BYTE;
            }
            (*EPtr).current.image.frame = SetAbsFrameIndex(
                (*EPtr).current.image.frame,
                frame_index,
            );
        }
        if (*StarShipPtr).cur_status_flags as ::core::ffi::c_int
            & (*StarShipPtr).old_status_flags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int != 0
        {
            (*StarShipPtr).weapon_counter = 1 as BYTE;
        } else {
            let mut angle: COUNT = 0;
            (*EPtr).c2rust_unnamed.life_span = 10 as COUNT;
            (*EPtr).preprocess_func = Some(
                pump_up_preprocess as unsafe extern "C" fn(*mut ELEMENT) -> (),
            ) as Option<ElementProcessFunc>;
            (*EPtr).postprocess_func = None;
            angle = (((*StarShipPtr).ShipFacing as ::core::ffi::c_int)
                << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int) as COUNT;
            SetVelocityComponents(
                &raw mut (*EPtr).velocity,
                (*(&raw mut sinetab as *mut SDWORD)
                    .offset(
                        (angle as ::core::ffi::c_int
                            + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                                >> 2 as ::core::ffi::c_int)
                            & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                                - 1 as ::core::ffi::c_int) as DWORD as isize,
                    ) as ::core::ffi::c_long
                    * (((45 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                        << resolutionFactor << 5 as ::core::ffi::c_int)
                        as ::core::ffi::c_long >> 14 as ::core::ffi::c_int) as SDWORD,
                (*(&raw mut sinetab as *mut SDWORD)
                    .offset(
                        (angle as ::core::ffi::c_int
                            & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                                - 1 as ::core::ffi::c_int) as DWORD as isize,
                    ) as ::core::ffi::c_long
                    * (((45 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                        << resolutionFactor << 5 as ::core::ffi::c_int)
                        as ::core::ffi::c_long >> 14 as ::core::ffi::c_int) as SDWORD,
            );
            ProcessSound(
                SetAbsStringTableIndex(
                    (*(*StarShipPtr).RaceDescPtr).ship_data.ship_sounds,
                    1 as COUNT,
                ),
                EPtr,
            );
        }
        UnlockLink(&raw mut disp_q, hPumpUp);
        PutQueue(&raw mut disp_q, hPumpUp);
        (*GlobData.Game_state.DisplayArray.offset((*ElementPtr).PrimIndex as isize))
            .Type = gfx_object::NUM_PRIMS.0 as ::core::ffi::c_int as GRAPHICS_PRIM;
        (*ElementPtr).state_flags = ((*ElementPtr).state_flags as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as ELEMENT_FLAGS;
    };
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
unsafe extern "C" fn pump_up_collision(
    mut ElementPtr0: *mut ELEMENT,
    mut pPt0: *mut POINT,
    mut ElementPtr1: *mut ELEMENT,
    mut pPt1: *mut POINT,
) {
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut old_thrust_wait: BYTE = 0;
    let mut hBlastElement: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
    GetFrameRect((*ElementPtr0).next.image.frame, &raw mut r);
    old_thrust_wait = (*ElementPtr0).c2rust_unnamed_3.thrust_wait;
    (*ElementPtr0).c2rust_unnamed_3.blast_offset = (r.extent.width as ::core::ffi::c_int
        >> 1 as ::core::ffi::c_int) as BYTE;
    hBlastElement = uqm_lua_checked!(weapon_collision(ElementPtr0, pPt0, ElementPtr1, pPt1), ());
    (*ElementPtr0).c2rust_unnamed_3.thrust_wait = old_thrust_wait;
    if !hBlastElement.is_null() {
        let mut BlastElementPtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
        BlastElementPtr = LockLink(&raw mut disp_q, hBlastElement) as *mut ELEMENT;
        (*BlastElementPtr).c2rust_unnamed.life_span = (3 as ::core::ffi::c_int
            + ((*ElementPtr0).c2rust_unnamed_2.turn_wait as ::core::ffi::c_int
                & !(((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as BYTE
                    as ::core::ffi::c_int))) as COUNT;
        (*BlastElementPtr).c2rust_unnamed_3.next_turn = 0 as BYTE;
        (*BlastElementPtr).c2rust_unnamed_2.turn_wait = (*BlastElementPtr)
            .c2rust_unnamed_3
            .next_turn;
        (*BlastElementPtr).preprocess_func = Some(
            animate as unsafe extern "C" fn(*mut ELEMENT) -> (),
        ) as Option<ElementProcessFunc>;
        (*BlastElementPtr).current.image.farray = (*ElementPtr0).next.image.farray;
        (*BlastElementPtr).current.image.frame = SetAbsFrameIndex(
            *(*BlastElementPtr).current.image.farray.offset(0isize),
            (4 as ::core::ffi::c_int * 5 as ::core::ffi::c_int) as COUNT,
        );
        UnlockLink(&raw mut disp_q, hBlastElement);
    }
}
unsafe extern "C" fn initialize_pump_up(
    mut ShipPtr: *mut ELEMENT,
    mut PumpUpArray: *mut HELEMENT,
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
    MissileBlock.pixoffs = ((24 as ::core::ffi::c_int) << resolutionFactor) as SIZE;
    MissileBlock.speed = ((24 as ::core::ffi::c_int) << resolutionFactor
        << 2 as ::core::ffi::c_int) as SIZE;
    MissileBlock.hit_points = 2 as SIZE;
    MissileBlock.damage = 2 as SIZE;
    MissileBlock.life = 2 as COUNT;
    MissileBlock.preprocess_func = None;
    MissileBlock.blast_offs = 0 as SIZE;
    *PumpUpArray.offset(0isize) = initialize_missile(&raw mut MissileBlock);
    if !(*PumpUpArray.offset(0isize)).is_null() {
        let mut PumpUpPtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
        PumpUpPtr = LockLink(&raw mut disp_q, *PumpUpArray.offset(0isize))
            as *mut ELEMENT;
        (*PumpUpPtr).postprocess_func = Some(
            pump_up_postprocess as unsafe extern "C" fn(*mut ELEMENT) -> (),
        ) as Option<ElementProcessFunc>;
        (*PumpUpPtr).collision_func = Some(
            pump_up_collision
                as unsafe extern "C" fn(
                    *mut ELEMENT,
                    *mut POINT,
                    *mut ELEMENT,
                    *mut POINT,
                ) -> (),
        ) as Option<ElementCollisionFunc>;
        if antiCheat(
                ShipPtr,
                BOOLEAN::FALSE,
                C2Rust_Unnamed::OPTVAL_INF_HEALTH.0 as ::core::ffi::c_int,
            )
            .0 != 0
            || antiCheat(
                    ShipPtr,
                    BOOLEAN::FALSE,
                    C2Rust_Unnamed::OPTVAL_FULL_GOD.0 as ::core::ffi::c_int,
                )
                .0 != 0
        {
            (*PumpUpPtr).c2rust_unnamed_3.thrust_wait = 5 as BYTE;
        } else {
            (*PumpUpPtr).c2rust_unnamed_3.thrust_wait = 72 as BYTE;
        }
        UnlockLink(&raw mut disp_q, *PumpUpArray.offset(0isize));
    }
    return 1 as COUNT;
}
unsafe extern "C" fn confuse_preprocess(mut ElementPtr: *mut ELEMENT) {
    if (*ElementPtr).state_flags as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int == 0
    {
        (*ElementPtr).next.image.frame = SetAbsFrameIndex(
            (*ElementPtr).current.image.frame,
            (GetFrameIndex((*ElementPtr).current.image.frame) as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as COUNT,
        );
        (*ElementPtr).state_flags = ((*ElementPtr).state_flags as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as ELEMENT_FLAGS;
    } else if (*ElementPtr).hTarget.is_null() {
        (*ElementPtr).c2rust_unnamed.life_span = 0 as COUNT;
        (*ElementPtr).state_flags = ((*ElementPtr).state_flags as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as ELEMENT_FLAGS;
    } else {
        let mut eptr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
        eptr = LockLink(&raw mut disp_q, (*ElementPtr).hTarget) as *mut ELEMENT;
        (*ElementPtr).next.location = (*eptr).next.location;
        if (*ElementPtr).c2rust_unnamed_2.turn_wait != 0 {
            let mut hEffect: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
            let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
            (*ElementPtr).next.image.frame = IncFrameIndex(
                (*ElementPtr).current.image.frame,
            );
            if GetFrameIndex((*ElementPtr).next.image.frame) as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
            {
                (*ElementPtr).next.image.frame = SetRelFrameIndex(
                    (*ElementPtr).next.image.frame,
                    -8 as SIZE,
                );
            }
            StarShipPtr = (*eptr).pParent as *mut STARSHIP;
            (*StarShipPtr).ship_input_state = ((*StarShipPtr).ship_input_state
                as ::core::ffi::c_int
                & !((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                | (*ElementPtr).c2rust_unnamed_2.turn_wait as ::core::ffi::c_int)
                as BYTE;
            hEffect = AllocElement();
            if !hEffect.is_null() {
                eptr = LockLink(&raw mut disp_q, hEffect) as *mut ELEMENT;
                (*eptr).playerNr = (*ElementPtr).playerNr;
                (*eptr).state_flags = ((1 as ::core::ffi::c_int)
                    << 10 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int)
                    as ELEMENT_FLAGS;
                (*eptr).c2rust_unnamed.life_span = 1 as COUNT;
                (*eptr).next = (*ElementPtr).next;
                (*eptr).current = (*eptr).next;
                (*eptr).preprocess_func = Some(
                    confuse_preprocess as unsafe extern "C" fn(*mut ELEMENT) -> (),
                ) as Option<ElementProcessFunc>;
                (*GlobData.Game_state.DisplayArray.offset((*eptr).PrimIndex as isize))
                    .Type = gfx_object::STAMP_PRIM.0 as ::core::ffi::c_int
                    as GRAPHICS_PRIM;
                StarShipPtr = (*ElementPtr).pParent as *mut STARSHIP;
                (*eptr).pParent = StarShipPtr as *mut ::core::ffi::c_void;
                (*eptr).hTarget = (*ElementPtr).hTarget;
                UnlockLink(&raw mut disp_q, hEffect);
                PutQueue(&raw mut disp_q, hEffect);
            }
        }
        UnlockLink(&raw mut disp_q, (*ElementPtr).hTarget);
    };
}
unsafe extern "C" fn confusion_collision(
    mut ElementPtr0: *mut ELEMENT,
    mut pPt0: *mut POINT,
    mut ElementPtr1: *mut ELEMENT,
    mut pPt1: *mut POINT,
) {
    if (*ElementPtr1).state_flags as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int != 0
    {
        let mut hConfusionElement: HELEMENT = ::core::ptr::null_mut::<
            ::core::ffi::c_void,
        >();
        let mut hNextElement: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
        let mut ConfusionPtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
        let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
        StarShipPtr = (*ElementPtr0).pParent as *mut STARSHIP;
        hConfusionElement = disp_q.head as HELEMENT;
        while !hConfusionElement.is_null() {
            ConfusionPtr = LockLink(&raw mut disp_q, hConfusionElement) as *mut ELEMENT;
            if elementsOfSamePlayer(ConfusionPtr, ElementPtr0).0 != 0
                && (*ConfusionPtr).current.image.farray
                    == &raw mut (*(*StarShipPtr).RaceDescPtr).ship_data.special
                        as *mut FRAME
                && (*ConfusionPtr).state_flags as ::core::ffi::c_int
                    & (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int != 0
            {
                UnlockLink(&raw mut disp_q, hConfusionElement);
                break;
            } else {
                hNextElement = (*ConfusionPtr).succ;
                UnlockLink(&raw mut disp_q, hConfusionElement);
                hConfusionElement = hNextElement;
            }
        }
        if !hConfusionElement.is_null()
            || {
                hConfusionElement = AllocElement();
                !hConfusionElement.is_null()
            }
        {
            ConfusionPtr = LockLink(&raw mut disp_q, hConfusionElement) as *mut ELEMENT;
            if (*ConfusionPtr).state_flags as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
            {
                InsertQueue(&raw mut disp_q, hConfusionElement, disp_q.head);
                (*ConfusionPtr).current = (*ElementPtr0).next;
                (*ConfusionPtr).current.image.frame = SetAbsFrameIndex(
                    (*ConfusionPtr).current.image.frame,
                    8 as COUNT,
                );
                (*ConfusionPtr).next = (*ConfusionPtr).current;
                (*ConfusionPtr).playerNr = (*ElementPtr0).playerNr;
                (*ConfusionPtr).state_flags = ((1 as ::core::ffi::c_int)
                    << 10 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int)
                    as ELEMENT_FLAGS;
                (*ConfusionPtr).preprocess_func = Some(
                    confuse_preprocess as unsafe extern "C" fn(*mut ELEMENT) -> (),
                ) as Option<ElementProcessFunc>;
                (*GlobData
                    .Game_state
                    .DisplayArray
                    .offset((*ConfusionPtr).PrimIndex as isize))
                    .Type = gfx_object::NUM_PRIMS.0 as ::core::ffi::c_int
                    as GRAPHICS_PRIM;
                (*ConfusionPtr).pParent = StarShipPtr as *mut ::core::ffi::c_void;
                StarShipPtr = (*ElementPtr1).pParent as *mut STARSHIP;
                (*ConfusionPtr).hTarget = (*StarShipPtr).hShip;
            }
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
                (*ConfusionPtr).c2rust_unnamed.life_span = 400 as COUNT;
            }
            (*ConfusionPtr).c2rust_unnamed_2.turn_wait = ((1 as ::core::ffi::c_int)
                << (TFB_Random() as BYTE as ::core::ffi::c_int
                    & 1 as ::core::ffi::c_int)) as BYTE;
            UnlockLink(&raw mut disp_q, hConfusionElement);
        }
        (*ElementPtr0).c2rust_unnamed_0.hit_points = 0 as COUNT;
        (*ElementPtr0).c2rust_unnamed.life_span = 0 as COUNT;
        (*ElementPtr0).state_flags = ((*ElementPtr0).state_flags as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int))
            as ELEMENT_FLAGS;
    }
}
unsafe extern "C" fn initialize_confusion(
    mut ShipPtr: *mut ELEMENT,
    mut ConfusionArray: *mut HELEMENT,
) -> COUNT {
    let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
    let mut ConfusionBlock: MISSILE_BLOCK = MISSILE_BLOCK {
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
    ConfusionBlock.cx = (*ShipPtr).next.location.x;
    ConfusionBlock.cy = (*ShipPtr).next.location.y;
    ConfusionBlock.farray = &raw mut (*(*StarShipPtr).RaceDescPtr).ship_data.special
        as *mut FRAME;
    ConfusionBlock.index = 0 as COUNT;
    ConfusionBlock.face = (*StarShipPtr).ShipFacing;
    ConfusionBlock.sender = (*ShipPtr).playerNr;
    ConfusionBlock.flags = ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
        as ELEMENT_FLAGS;
    ConfusionBlock.pixoffs = ((24 as ::core::ffi::c_int) << resolutionFactor) as SIZE;
    ConfusionBlock.speed = ((30 as ::core::ffi::c_int) << resolutionFactor
        << 2 as ::core::ffi::c_int) as SIZE;
    ConfusionBlock.hit_points = 200 as SIZE;
    ConfusionBlock.damage = 0 as SIZE;
    ConfusionBlock.life = 20 as COUNT;
    ConfusionBlock.preprocess_func = Some(
        confuse_preprocess as unsafe extern "C" fn(*mut ELEMENT) -> (),
    ) as Option<unsafe extern "C" fn(*mut ELEMENT) -> ()>;
    ConfusionBlock.blast_offs = ((4 as ::core::ffi::c_int) << resolutionFactor) as SIZE;
    *ConfusionArray.offset(0isize) = initialize_missile(&raw mut ConfusionBlock);
    if !(*ConfusionArray.offset(0isize)).is_null() {
        let mut CMissilePtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
        CMissilePtr = LockLink(&raw mut disp_q, *ConfusionArray.offset(0isize))
            as *mut ELEMENT;
        (*CMissilePtr).collision_func = Some(
            confusion_collision
                as unsafe extern "C" fn(
                    *mut ELEMENT,
                    *mut POINT,
                    *mut ELEMENT,
                    *mut POINT,
                ) -> (),
        ) as Option<ElementCollisionFunc>;
        (*CMissilePtr).pParent = StarShipPtr as *mut ::core::ffi::c_void;
        UnlockLink(&raw mut disp_q, *ConfusionArray.offset(0isize));
    }
    return 1 as COUNT;
}
unsafe extern "C" fn initialize_test_pump_up(
    mut ShipPtr: *mut ELEMENT,
    mut PumpUpArray: *mut HELEMENT,
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
    MissileBlock.pixoffs = ((24 as ::core::ffi::c_int) << resolutionFactor) as SIZE;
    MissileBlock.speed = (((45 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
        << resolutionFactor) as SIZE;
    MissileBlock.hit_points = 2 as SIZE;
    MissileBlock.damage = 2 as SIZE;
    MissileBlock.life = 10 as COUNT;
    MissileBlock.preprocess_func = None;
    MissileBlock.blast_offs = 0 as SIZE;
    *PumpUpArray.offset(0isize) = initialize_missile(&raw mut MissileBlock);
    return 1 as COUNT;
}
unsafe extern "C" fn melnorme_intelligence(
    mut ShipPtr: *mut ELEMENT,
    mut ObjectsOfConcern: *mut EVALUATE_DESC,
    mut ConcernCounter: COUNT,
) {
    let mut old_count: BYTE = 0;
    let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
    let mut lpEvalDesc: *mut EVALUATE_DESC = ::core::ptr::null_mut::<EVALUATE_DESC>();
    StarShipPtr = (*ShipPtr).pParent as *mut STARSHIP;
    (*(*StarShipPtr).RaceDescPtr).init_weapon_func = Some(
        initialize_test_pump_up
            as unsafe extern "C" fn(*mut ELEMENT, *mut HELEMENT) -> COUNT,
    ) as Option<INIT_WEAPON_FUNC>;
    old_count = (*StarShipPtr).weapon_counter;
    if (*StarShipPtr).weapon_counter as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
        (*StarShipPtr).weapon_counter = 0 as BYTE;
    }
    lpEvalDesc = ObjectsOfConcern
        .offset(C2Rust_Unnamed_6::ENEMY_SHIP_INDEX.0 as ::core::ffi::c_int as isize);
    if !(*lpEvalDesc).ObjectPtr.is_null() {
        if ((*(*StarShipPtr).RaceDescPtr).ship_info.energy_level as ::core::ffi::c_int)
            < 20 as ::core::ffi::c_int + 5 as ::core::ffi::c_int
            && (*StarShipPtr).old_status_flags as ::core::ffi::c_int
                & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int == 0
        {
            (*lpEvalDesc).MoveState = MOVEMENT_STATE::ENTICE;
        } else {
            let mut EnemyStarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<
                STARSHIP,
            >();
            EnemyStarShipPtr = (*(*lpEvalDesc).ObjectPtr).pParent as *mut STARSHIP;
            if (*(*EnemyStarShipPtr).RaceDescPtr).ship_info.ship_flags
                as ::core::ffi::c_int
                & (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int == 0
            {
                (*lpEvalDesc).MoveState = MOVEMENT_STATE::PURSUE;
            }
        }
    }
    uqm_lua_checked!(ship_intelligence(ShipPtr, ObjectsOfConcern, ConcernCounter), ());
    if (*StarShipPtr).weapon_counter as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        && (old_count as ::core::ffi::c_int != 0 as ::core::ffi::c_int
            || ((*StarShipPtr).special_counter as ::core::ffi::c_int != 0
                || (*(*StarShipPtr).RaceDescPtr).ship_info.energy_level
                    as ::core::ffi::c_int
                    >= 20 as ::core::ffi::c_int + 5 as ::core::ffi::c_int)
                && (*StarShipPtr).ship_input_state as ::core::ffi::c_int
                    & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int == 0)
    {
        (*StarShipPtr).ship_input_state = ((*StarShipPtr).ship_input_state
            as ::core::ffi::c_int ^ (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
            as BYTE;
    }
    (*StarShipPtr).ship_input_state = ((*StarShipPtr).ship_input_state
        as ::core::ffi::c_int & !((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int))
        as BYTE;
    if (*StarShipPtr).special_counter as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        && (*(*StarShipPtr).RaceDescPtr).ship_info.energy_level as ::core::ffi::c_int
            >= 20 as ::core::ffi::c_int
    {
        let mut old_input_state: BYTE = 0;
        old_input_state = (*StarShipPtr).ship_input_state;
        (*(*StarShipPtr).RaceDescPtr).init_weapon_func = Some(
            initialize_confusion
                as unsafe extern "C" fn(*mut ELEMENT, *mut HELEMENT) -> COUNT,
        ) as Option<INIT_WEAPON_FUNC>;
        (*ShipPtr).c2rust_unnamed_2.turn_wait = (*ShipPtr)
            .c2rust_unnamed_2
            .turn_wait
            .wrapping_add(1);
        (*ShipPtr).c2rust_unnamed_3.thrust_wait = (*ShipPtr)
            .c2rust_unnamed_3
            .thrust_wait
            .wrapping_add(1);
        uqm_lua_checked!(ship_intelligence(
            ShipPtr,
            ObjectsOfConcern,
            (C2Rust_Unnamed_6::ENEMY_SHIP_INDEX.0 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int) as COUNT,
        ), ());
        (*ShipPtr).c2rust_unnamed_3.thrust_wait = (*ShipPtr)
            .c2rust_unnamed_3
            .thrust_wait
            .wrapping_sub(1);
        (*ShipPtr).c2rust_unnamed_2.turn_wait = (*ShipPtr)
            .c2rust_unnamed_2
            .turn_wait
            .wrapping_sub(1);
        if (*StarShipPtr).ship_input_state as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int != 0
        {
            (*StarShipPtr).ship_input_state = ((*StarShipPtr).ship_input_state
                as ::core::ffi::c_int
                & !((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)) as BYTE;
            (*StarShipPtr).ship_input_state = ((*StarShipPtr).ship_input_state
                as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as BYTE;
        }
        (*StarShipPtr).ship_input_state = (old_input_state as ::core::ffi::c_int
            | (*StarShipPtr).ship_input_state as ::core::ffi::c_int
                & (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
            as ::core::ffi::c_uchar as BYTE;
    }
    (*StarShipPtr).weapon_counter = old_count;
    (*(*StarShipPtr).RaceDescPtr).init_weapon_func = Some(
        initialize_pump_up as unsafe extern "C" fn(*mut ELEMENT, *mut HELEMENT) -> COUNT,
    ) as Option<INIT_WEAPON_FUNC>;
}
unsafe extern "C" fn melnorme_postprocess(mut ElementPtr: *mut ELEMENT) {
    let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
    StarShipPtr = (*ElementPtr).pParent as *mut STARSHIP;
    if (*StarShipPtr).cur_status_flags as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int != 0
        && (*StarShipPtr).special_counter as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int && uqm_lua_checked!(DeltaEnergy(ElementPtr, -20 as SIZE), ()).0 != 0
    {
        let mut Confusion: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
        initialize_confusion(ElementPtr, &raw mut Confusion);
        if !Confusion.is_null() {
            let mut CMissilePtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
            CMissilePtr = LockLink(&raw mut disp_q, Confusion) as *mut ELEMENT;
            ProcessSound(
                SetAbsStringTableIndex(
                    (*(*StarShipPtr).RaceDescPtr).ship_data.ship_sounds,
                    1 as COUNT,
                ),
                CMissilePtr,
            );
            UnlockLink(&raw mut disp_q, Confusion);
            PutQueue(&raw mut disp_q, Confusion);
            (*StarShipPtr).special_counter = (*(*StarShipPtr).RaceDescPtr)
                .characteristics
                .special_wait;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn init_melnorme() -> *mut RACE_DESC {
    let mut RaceDescPtr: *mut RACE_DESC = ::core::ptr::null_mut::<RACE_DESC>();
    if if resolutionFactor != 2 as ::core::ffi::c_uint {
        BOOLEAN::FALSE.0 as ::core::ffi::c_int
    } else {
        BOOLEAN::TRUE.0 as ::core::ffi::c_int
    } != 0
    {
        melnorme_desc.characteristics.max_thrust = ((36 as ::core::ffi::c_int)
            << resolutionFactor) as COUNT;
        melnorme_desc.characteristics.thrust_increment = ((6 as ::core::ffi::c_int)
            << resolutionFactor) as COUNT;
        melnorme_desc.cyborg_control.WeaponRange = ((((45 as ::core::ffi::c_int)
            << 2 as ::core::ffi::c_int) << resolutionFactor) * 10 as ::core::ffi::c_int)
            as COUNT;
    } else {
        melnorme_desc.characteristics.max_thrust = 36 as COUNT;
        melnorme_desc.characteristics.thrust_increment = 6 as COUNT;
        melnorme_desc.cyborg_control.WeaponRange = (((45 as ::core::ffi::c_int)
            << 2 as ::core::ffi::c_int) * 10 as ::core::ffi::c_int) as COUNT;
    }
    melnorme_desc.postprocess_func = Some(
        melnorme_postprocess as unsafe extern "C" fn(*mut ELEMENT) -> (),
    ) as Option<POSTPROCESS_FUNC>;
    melnorme_desc.init_weapon_func = Some(
        initialize_pump_up as unsafe extern "C" fn(*mut ELEMENT, *mut HELEMENT) -> COUNT,
    ) as Option<INIT_WEAPON_FUNC>;
    melnorme_desc.cyborg_control.intelligence_func = Some(
        melnorme_intelligence
            as unsafe extern "C" fn(*mut ELEMENT, *mut EVALUATE_DESC, COUNT) -> (),
    ) as Option<IntelligenceFunc>;
    RaceDescPtr = &raw mut melnorme_desc;
    return RaceDescPtr;
}
