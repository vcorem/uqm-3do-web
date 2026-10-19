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

// Rust translation of src/uqm/weapon.c.
extern "C" {
    pub type string_table_entry;
    pub type context_desc;
    pub type frame_desc;
    pub type tfb_soundsample;
    fn __assert_fail(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    );
    fn PutQueue(pq: *mut QUEUE, hLink: HLINK);
    fn SetContext(Context: CONTEXT) -> CONTEXT;
    fn DrawablesIntersect(
        pControl0: *mut INTERSECT_CONTROL,
        pControl1: *mut INTERSECT_CONTROL,
        max_time_val: TIME_VALUE,
    ) -> TIME_VALUE;
    fn DrawStamp(pStamp: *mut STAMP);
    fn GetFrameRect(Frame: FRAME, pRect: *mut RECT) -> BOOLEAN;
    fn GetFrameCount(Frame: FRAME) -> COUNT;
    fn SetAbsFrameIndex(Frame: FRAME, FrameIndex: COUNT) -> FRAME;
    fn DecFrameIndex(Frame: FRAME) -> FRAME;
    fn SetAbsStringTableIndex(String: STRING, StringTableIndex: COUNT) -> STRING;
    static mut ScreenWidth: ::core::ffi::c_int;
    static mut ScreenHeight: ::core::ffi::c_int;
    static mut sinetab: [SIZE; 0];
    fn ARCTAN(delta_x: SIZE, delta_y: SIZE) -> COUNT;
    fn SetVelocityComponents(velocityptr: *mut VELOCITY_DESC, dx: SIZE, dy: SIZE);
    static mut disp_q: QUEUE;
    static mut DisplayArray: [PRIMITIVE; 330];
    fn AllocElement() -> HELEMENT;
    fn do_damage(ElementPtr: *mut ELEMENT, damage: SIZE);
    static mut GlobData: GLOBDATA;
    static mut status_y_offsets: [COORD; 0];
    fn InitShipStatus(
        ShipInfoPtr: *mut SHIP_INFO,
        StarShipPtr: *mut STARSHIP,
        pClipRect: *mut RECT,
    );
    static mut stars_in_space: FRAME;
    fn animation_preprocess(ElementPtr: *mut ELEMENT);
    static mut StatusContext: CONTEXT;
    static mut GameSounds: SOUND;
    fn ProcessSound(Sound: SOUND, PositionalObject: *mut ELEMENT);
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
pub type CONTEXT_DESC = context_desc;
pub type FRAME_DESC = frame_desc;
pub type CONTEXT = *mut CONTEXT_DESC;
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
pub type SOUND = STRING;
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_5(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_5 {
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
unsafe extern "C" fn weapon_collision_cb(
    mut WeaponElementPtr: *mut ELEMENT,
    mut pWPt: *mut POINT,
    mut HitElementPtr: *mut ELEMENT,
    mut pHPt: *mut POINT,
) {
    weapon_collision(WeaponElementPtr, pWPt, HitElementPtr, pHPt);
}
#[no_mangle]
pub unsafe extern "C" fn initialize_laser(
    mut pLaserBlock: *mut LASER_BLOCK,
) -> HELEMENT {
    let mut hLaserElement: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
    hLaserElement = AllocElement();
    if !hLaserElement.is_null() {
        let mut LaserElementPtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
        LaserElementPtr = LockLink(&raw mut disp_q, hLaserElement) as *mut ELEMENT;
        (*LaserElementPtr).playerNr = (*pLaserBlock).sender;
        (*LaserElementPtr).c2rust_unnamed_0.hit_points = 1 as COUNT;
        (*LaserElementPtr).c2rust_unnamed_1.mass_points = 1 as BYTE;
        (*LaserElementPtr).state_flags = ((1 as ::core::ffi::c_int)
            << 3 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int
            | (*pLaserBlock).flags as ::core::ffi::c_int) as ELEMENT_FLAGS;
        (*LaserElementPtr).c2rust_unnamed.life_span = 1 as COUNT;
        (*LaserElementPtr).collision_func = Some(
            weapon_collision_cb
                as unsafe extern "C" fn(
                    *mut ELEMENT,
                    *mut POINT,
                    *mut ELEMENT,
                    *mut POINT,
                ) -> (),
        ) as Option<ElementCollisionFunc>;
        (*LaserElementPtr).c2rust_unnamed_3.blast_offset = 1 as BYTE;
        (*LaserElementPtr).current.location.x = ((*pLaserBlock).cx as ::core::ffi::c_int
            + (*(&raw mut sinetab as *mut SIZE)
                .offset(
                    ((((*pLaserBlock).face as ::core::ffi::c_int)
                        << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int)
                        + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                            >> 2 as ::core::ffi::c_int)
                        & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                            - 1 as ::core::ffi::c_int) as COUNT as isize,
                ) as ::core::ffi::c_long
                * (((*pLaserBlock).pixoffs as ::core::ffi::c_int)
                    << 2 as ::core::ffi::c_int) as ::core::ffi::c_long
                >> 14 as ::core::ffi::c_int) as SIZE as ::core::ffi::c_int) as COORD;
        (*LaserElementPtr).current.location.y = ((*pLaserBlock).cy as ::core::ffi::c_int
            + (*(&raw mut sinetab as *mut SIZE)
                .offset(
                    (((*pLaserBlock).face as ::core::ffi::c_int)
                        << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                        & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                            - 1 as ::core::ffi::c_int) as COUNT as isize,
                ) as ::core::ffi::c_long
                * (((*pLaserBlock).pixoffs as ::core::ffi::c_int)
                    << 2 as ::core::ffi::c_int) as ::core::ffi::c_long
                >> 14 as ::core::ffi::c_int) as SIZE as ::core::ffi::c_int) as COORD;
        DisplayArray[(*LaserElementPtr).PrimIndex as usize].Type = gfx_object::LINE_PRIM
            .0 as ::core::ffi::c_int as GRAPHICS_PRIM;
        DisplayArray[(*LaserElementPtr).PrimIndex as usize].color = (*pLaserBlock).color;
        (*LaserElementPtr).current.image.frame = DecFrameIndex(stars_in_space);
        (*LaserElementPtr).current.image.farray = &raw mut stars_in_space;
        SetVelocityComponents(
            &raw mut (*LaserElementPtr).velocity,
            (((*pLaserBlock).cx as ::core::ffi::c_int
                + (*pLaserBlock).ex as ::core::ffi::c_int
                - (*LaserElementPtr).current.location.x as ::core::ffi::c_int)
                << 5 as ::core::ffi::c_int) as SIZE,
            (((*pLaserBlock).cy as ::core::ffi::c_int
                + (*pLaserBlock).ey as ::core::ffi::c_int
                - (*LaserElementPtr).current.location.y as ::core::ffi::c_int)
                << 5 as ::core::ffi::c_int) as SIZE,
        );
        UnlockLink(&raw mut disp_q, hLaserElement);
    }
    return hLaserElement;
}
#[no_mangle]
pub unsafe extern "C" fn initialize_missile(
    mut pMissileBlock: *mut MISSILE_BLOCK,
) -> HELEMENT {
    let mut hMissileElement: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
    hMissileElement = AllocElement();
    if !hMissileElement.is_null() {
        let mut delta_x: SIZE = 0;
        let mut delta_y: SIZE = 0;
        let mut angle: COUNT = 0;
        let mut MissileElementPtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
        MissileElementPtr = LockLink(&raw mut disp_q, hMissileElement) as *mut ELEMENT;
        (*MissileElementPtr).c2rust_unnamed_0.hit_points = (*pMissileBlock).hit_points
            as BYTE as COUNT;
        (*MissileElementPtr).c2rust_unnamed_1.mass_points = (*pMissileBlock).damage
            as BYTE;
        (*MissileElementPtr).playerNr = (*pMissileBlock).sender;
        (*MissileElementPtr).state_flags = ((1 as ::core::ffi::c_int)
            << 3 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int
            | (*pMissileBlock).flags as ::core::ffi::c_int) as ELEMENT_FLAGS;
        (*MissileElementPtr).c2rust_unnamed.life_span = (*pMissileBlock).life;
        DisplayArray[(*MissileElementPtr).PrimIndex as usize].Type = gfx_object::STAMP_PRIM
            .0 as ::core::ffi::c_int as GRAPHICS_PRIM;
        (*MissileElementPtr).current.image.farray = (*pMissileBlock).farray;
        (*MissileElementPtr).current.image.frame = SetAbsFrameIndex(
            *(*pMissileBlock).farray.offset(0isize),
            (*pMissileBlock).index,
        );
        (*MissileElementPtr).preprocess_func = (*pMissileBlock).preprocess_func
            as Option<ElementProcessFunc>;
        (*MissileElementPtr).collision_func = Some(
            weapon_collision_cb
                as unsafe extern "C" fn(
                    *mut ELEMENT,
                    *mut POINT,
                    *mut ELEMENT,
                    *mut POINT,
                ) -> (),
        ) as Option<ElementCollisionFunc>;
        (*MissileElementPtr).c2rust_unnamed_3.blast_offset = (*pMissileBlock).blast_offs
            as BYTE;
        angle = (((*pMissileBlock).face as ::core::ffi::c_int)
            << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int) as COUNT;
        (*MissileElementPtr).current.location.x = ((*pMissileBlock).cx
            as ::core::ffi::c_int
            + (*(&raw mut sinetab as *mut SIZE)
                .offset(
                    (angle as ::core::ffi::c_int
                        + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                            >> 2 as ::core::ffi::c_int)
                        & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                            - 1 as ::core::ffi::c_int) as COUNT as isize,
                ) as ::core::ffi::c_long
                * (((*pMissileBlock).pixoffs as ::core::ffi::c_int)
                    << 2 as ::core::ffi::c_int) as ::core::ffi::c_long
                >> 14 as ::core::ffi::c_int) as SIZE as ::core::ffi::c_int) as COORD;
        (*MissileElementPtr).current.location.y = ((*pMissileBlock).cy
            as ::core::ffi::c_int
            + (*(&raw mut sinetab as *mut SIZE)
                .offset(
                    (angle as ::core::ffi::c_int
                        & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                            - 1 as ::core::ffi::c_int) as COUNT as isize,
                ) as ::core::ffi::c_long
                * (((*pMissileBlock).pixoffs as ::core::ffi::c_int)
                    << 2 as ::core::ffi::c_int) as ::core::ffi::c_long
                >> 14 as ::core::ffi::c_int) as SIZE as ::core::ffi::c_int) as COORD;
        delta_x = (*(&raw mut sinetab as *mut SIZE)
            .offset(
                (angle as ::core::ffi::c_int
                    + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                        >> 2 as ::core::ffi::c_int)
                    & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int) as COUNT as isize,
            ) as ::core::ffi::c_long
            * (((*pMissileBlock).speed as ::core::ffi::c_int) << 5 as ::core::ffi::c_int)
                as ::core::ffi::c_long >> 14 as ::core::ffi::c_int) as SIZE;
        delta_y = (*(&raw mut sinetab as *mut SIZE)
            .offset(
                (angle as ::core::ffi::c_int
                    & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int) as COUNT as isize,
            ) as ::core::ffi::c_long
            * (((*pMissileBlock).speed as ::core::ffi::c_int) << 5 as ::core::ffi::c_int)
                as ::core::ffi::c_long >> 14 as ::core::ffi::c_int) as SIZE;
        SetVelocityComponents(&raw mut (*MissileElementPtr).velocity, delta_x, delta_y);
        (*MissileElementPtr).current.location.x = ((*MissileElementPtr)
            .current
            .location
            .x as ::core::ffi::c_int
            - (delta_x as ::core::ffi::c_int >> 5 as ::core::ffi::c_int)) as COORD;
        (*MissileElementPtr).current.location.y = ((*MissileElementPtr)
            .current
            .location
            .y as ::core::ffi::c_int
            - (delta_y as ::core::ffi::c_int >> 5 as ::core::ffi::c_int)) as COORD;
        UnlockLink(&raw mut disp_q, hMissileElement);
    }
    return hMissileElement;
}
#[no_mangle]
pub unsafe extern "C" fn weapon_collision(
    mut WeaponElementPtr: *mut ELEMENT,
    mut pWPt: *mut POINT,
    mut HitElementPtr: *mut ELEMENT,
    mut pHPt: *mut POINT,
) -> HELEMENT {
    let mut damage: SIZE = 0;
    let mut hBlastElement: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if (*WeaponElementPtr).state_flags as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int != 0
    {
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    damage = (*WeaponElementPtr).c2rust_unnamed_1.mass_points as SIZE;
    if damage as ::core::ffi::c_int != 0
        && ((*HitElementPtr).state_flags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int != 0
            || (*HitElementPtr).c2rust_unnamed.life_span as ::core::ffi::c_int
                == 1 as ::core::ffi::c_int)
    {
        do_damage(HitElementPtr, damage);
        if (*HitElementPtr).c2rust_unnamed_0.hit_points != 0 {
            (*WeaponElementPtr).state_flags = ((*WeaponElementPtr).state_flags
                as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as ELEMENT_FLAGS;
        }
    }
    if (*HitElementPtr).state_flags as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int == 0
        || (*HitElementPtr).state_flags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int == 0
            && (*WeaponElementPtr).c2rust_unnamed_0.hit_points as ::core::ffi::c_int
                <= (*HitElementPtr).c2rust_unnamed_1.mass_points as ::core::ffi::c_int
    {
        if damage != 0 {
            damage = (C2Rust_Unnamed_5::TARGET_DAMAGED_FOR_1_PT.0 as ::core::ffi::c_int
                + (damage as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)) as SIZE;
            if damage as ::core::ffi::c_int
                > C2Rust_Unnamed_5::TARGET_DAMAGED_FOR_6_PLUS_PT.0 as ::core::ffi::c_int
            {
                damage = C2Rust_Unnamed_5::TARGET_DAMAGED_FOR_6_PLUS_PT.0
                    as ::core::ffi::c_int as SIZE;
            }
            ProcessSound(
                SetAbsStringTableIndex(GameSounds, damage as COUNT),
                HitElementPtr,
            );
        }
        if DisplayArray[(*WeaponElementPtr).PrimIndex as usize].Type
            as ::core::ffi::c_int != gfx_object::LINE_PRIM.0 as ::core::ffi::c_int
        {
            (*WeaponElementPtr).state_flags = ((*WeaponElementPtr).state_flags
                as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as ELEMENT_FLAGS;
        }
        (*WeaponElementPtr).c2rust_unnamed_0.hit_points = 0 as COUNT;
        (*WeaponElementPtr).c2rust_unnamed.life_span = 0 as COUNT;
        (*WeaponElementPtr).state_flags = ((*WeaponElementPtr).state_flags
            as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int))
            as ELEMENT_FLAGS;
        hBlastElement = AllocElement();
        if !hBlastElement.is_null() {
            let mut blast_index: COUNT = 0;
            let mut blast_offs: SIZE = 0;
            let mut angle: COUNT = 0;
            let mut num_blast_frames: COUNT = 0;
            let mut BlastElementPtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
            extern "C" {
                static mut blast: [FRAME; 0];
            }
            PutQueue(&raw mut disp_q, hBlastElement);
            BlastElementPtr = LockLink(&raw mut disp_q, hBlastElement) as *mut ELEMENT;
            (*BlastElementPtr).playerNr = (*WeaponElementPtr).playerNr;
            (*BlastElementPtr).state_flags = ((1 as ::core::ffi::c_int)
                << 3 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as ELEMENT_FLAGS;
            DisplayArray[(*BlastElementPtr).PrimIndex as usize].Type = gfx_object::STAMP_PRIM
                .0 as ::core::ffi::c_int as GRAPHICS_PRIM;
            (*BlastElementPtr).current.location.x = (((*pWPt).x as ::core::ffi::c_int)
                << 2 as ::core::ffi::c_int) as COORD;
            (*BlastElementPtr).current.location.y = (((*pWPt).y as ::core::ffi::c_int)
                << 2 as ::core::ffi::c_int) as COORD;
            angle = (*WeaponElementPtr).velocity.TravelAngle;
            blast_offs = (*WeaponElementPtr).c2rust_unnamed_3.blast_offset as SIZE;
            if blast_offs as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                (*BlastElementPtr).current.location.x = ((*BlastElementPtr)
                    .current
                    .location
                    .x as ::core::ffi::c_int
                    + (*(&raw mut sinetab as *mut SIZE)
                        .offset(
                            (angle as ::core::ffi::c_int
                                + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                                    >> 2 as ::core::ffi::c_int)
                                & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                                    - 1 as ::core::ffi::c_int) as COUNT as isize,
                        ) as ::core::ffi::c_long
                        * ((blast_offs as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                            as ::core::ffi::c_long >> 14 as ::core::ffi::c_int) as SIZE
                        as ::core::ffi::c_int) as COORD;
                (*BlastElementPtr).current.location.y = ((*BlastElementPtr)
                    .current
                    .location
                    .y as ::core::ffi::c_int
                    + (*(&raw mut sinetab as *mut SIZE)
                        .offset(
                            (angle as ::core::ffi::c_int
                                & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                                    - 1 as ::core::ffi::c_int) as COUNT as isize,
                        ) as ::core::ffi::c_long
                        * ((blast_offs as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                            as ::core::ffi::c_long >> 14 as ::core::ffi::c_int) as SIZE
                        as ::core::ffi::c_int) as COORD;
            }
            blast_index = (angle as ::core::ffi::c_int
                + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                    >> 1 as ::core::ffi::c_int)
                + ((1 as ::core::ffi::c_int)
                    << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int)
                >> 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                & ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int) as COUNT;
            blast_index = (((blast_index as ::core::ffi::c_int
                >> 2 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
                + if blast_index as ::core::ffi::c_int & 0x3 as ::core::ffi::c_int != 0 {
                    1 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                }) as COUNT;
            num_blast_frames = GetFrameCount((*WeaponElementPtr).next.image.frame);
            if num_blast_frames as ::core::ffi::c_int
                <= ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                    + ((1 as ::core::ffi::c_int)
                        << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int)
                    >> 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
            {
                (*BlastElementPtr).c2rust_unnamed.life_span = 2 as COUNT;
                (*BlastElementPtr).current.image.farray = &raw mut blast as *mut FRAME;
                (*BlastElementPtr).current.image.frame = SetAbsFrameIndex(
                    *(&raw mut blast as *mut FRAME).offset(0isize),
                    blast_index,
                );
            } else {
                (*BlastElementPtr).c2rust_unnamed.life_span = (num_blast_frames
                    as ::core::ffi::c_int
                    - (((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                        + ((1 as ::core::ffi::c_int)
                            << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int)
                        >> 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int)) as COUNT;
                (*BlastElementPtr).c2rust_unnamed_3.next_turn = 0 as BYTE;
                (*BlastElementPtr).c2rust_unnamed_2.turn_wait = (*BlastElementPtr)
                    .c2rust_unnamed_3
                    .next_turn;
                (*BlastElementPtr).preprocess_func = Some(
                    animation_preprocess as unsafe extern "C" fn(*mut ELEMENT) -> (),
                ) as Option<ElementProcessFunc>;
                (*BlastElementPtr).current.image.farray = (*WeaponElementPtr)
                    .next
                    .image
                    .farray;
                (*BlastElementPtr).current.image.frame = SetAbsFrameIndex(
                    *(*BlastElementPtr).current.image.farray.offset(0isize),
                    (((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                        + ((1 as ::core::ffi::c_int)
                            << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int)
                        >> 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int) as COUNT,
                );
            }
            UnlockLink(&raw mut disp_q, hBlastElement);
            return hBlastElement;
        }
    }
    return ::core::ptr::null_mut::<::core::ffi::c_void>();
}
#[no_mangle]
pub unsafe extern "C" fn ModifySilhouette(
    mut ElementPtr: *mut ELEMENT,
    mut modify_stamp: *mut STAMP,
    mut modify_flags: BYTE,
) -> FRAME {
    let mut f: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut or: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut ShipIntersect: INTERSECT_CONTROL = INTERSECT_CONTROL {
        last_time_val: 0,
        EndPoint: POINT { x: 0, y: 0 },
        IntersectStamp: STAMP {
            origin: POINT { x: 0, y: 0 },
            frame: ::core::ptr::null_mut::<FRAME_DESC>(),
        },
    };
    let mut ObjectIntersect: INTERSECT_CONTROL = INTERSECT_CONTROL {
        last_time_val: 0,
        EndPoint: POINT { x: 0, y: 0 },
        IntersectStamp: STAMP {
            origin: POINT { x: 0, y: 0 },
            frame: ::core::ptr::null_mut::<FRAME_DESC>(),
        },
    };
    let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
    let mut OldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    f = ::core::ptr::null_mut::<FRAME_DESC>();
    ObjectIntersect.IntersectStamp = *modify_stamp;
    GetFrameRect(ObjectIntersect.IntersectStamp.frame, &raw mut or);
    StarShipPtr = (*ElementPtr).pParent as *mut STARSHIP;
    if modify_flags as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int != 0
    {
        ShipIntersect.IntersectStamp.frame = SetAbsFrameIndex(
            (*(*StarShipPtr).RaceDescPtr).ship_info.icons,
            1 as COUNT,
        );
        if ShipIntersect.IntersectStamp.frame.is_null() {
            return ::core::ptr::null_mut::<FRAME_DESC>();
        }
        GetFrameRect(ShipIntersect.IntersectStamp.frame, &raw mut r);
        ShipIntersect.IntersectStamp.origin.x = 0 as COORD;
        ShipIntersect.IntersectStamp.origin.y = 0 as COORD;
        ShipIntersect.EndPoint = ShipIntersect.IntersectStamp.origin;
        loop {
            ObjectIntersect.IntersectStamp.origin.x = (TFB_Random() as COUNT
                as ::core::ffi::c_int
                % (r.extent.width as ::core::ffi::c_int
                    - or.extent.width as ::core::ffi::c_int)
                + (or.extent.width as ::core::ffi::c_int
                    - r.extent.width as ::core::ffi::c_int >> 1 as ::core::ffi::c_int))
                as COORD;
            ObjectIntersect.IntersectStamp.origin.y = (TFB_Random() as COUNT
                as ::core::ffi::c_int
                % (r.extent.height as ::core::ffi::c_int
                    - or.extent.height as ::core::ffi::c_int)
                + (or.extent.height as ::core::ffi::c_int
                    - r.extent.height as ::core::ffi::c_int >> 1 as ::core::ffi::c_int))
                as COORD;
            ObjectIntersect.EndPoint = ObjectIntersect.IntersectStamp.origin;
            if DrawablesIntersect(
                &raw mut ObjectIntersect,
                &raw mut ShipIntersect,
                (((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
                    + 1 as ::core::ffi::c_int) as TIME_VALUE,
            ) != 0
            {
                break;
            }
        }
        ObjectIntersect.IntersectStamp.origin.x = (ObjectIntersect
            .IntersectStamp
            .origin
            .x as ::core::ffi::c_int
            + (64 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)) as COORD;
        ObjectIntersect.IntersectStamp.origin.y = (ObjectIntersect
            .IntersectStamp
            .origin
            .y as ::core::ffi::c_int + 31 as ::core::ffi::c_int) as COORD;
    }
    ObjectIntersect.IntersectStamp.origin.y = (ObjectIntersect.IntersectStamp.origin.y
        as ::core::ffi::c_int
        + *(&raw mut status_y_offsets as *mut COORD)
            .offset((*ElementPtr).playerNr as isize) as ::core::ffi::c_int) as COORD;
    if modify_flags as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int != 0
    {
        or.corner.x = (or.corner.x as ::core::ffi::c_int
            + ObjectIntersect.IntersectStamp.origin.x as ::core::ffi::c_int) as COORD;
        or.corner.y = (or.corner.y as ::core::ffi::c_int
            + ObjectIntersect.IntersectStamp.origin.y as ::core::ffi::c_int) as COORD;
        InitShipStatus(
            &raw mut (*(*StarShipPtr).RaceDescPtr).ship_info,
            StarShipPtr,
            &raw mut or,
        );
    } else {
        OldContext = SetContext(StatusContext);
        DrawStamp(&raw mut ObjectIntersect.IntersectStamp);
        SetContext(OldContext);
    }
    return f;
}
#[no_mangle]
pub unsafe extern "C" fn TrackShip(
    mut Tracker: *mut ELEMENT,
    mut pfacing: *mut COUNT,
) -> SIZE {
    let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
    enum C2Rust_Block {
        CheckTracking,
        S_30,
    }
    let mut c2rust_current_block: C2Rust_Block;
    let mut best_delta_facing: SIZE = 0;
    let mut best_delta: SIZE = 0;
    let mut hShip: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut hNextShip: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut Trackee: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
    best_delta = 0 as SIZE;
    best_delta_facing = -1 as SIZE;
    hShip = (*Tracker).hTarget;
    if !hShip.is_null() {
        Trackee = LockLink(&raw mut disp_q, hShip) as *mut ELEMENT;
        hNextShip = ::core::ptr::null_mut::<::core::ffi::c_void>();
        (*Tracker).hTarget = hNextShip;
        c2rust_current_block = C2Rust_Block::CheckTracking;
    } else {
        hShip = disp_q.head as HELEMENT;
        c2rust_current_block = C2Rust_Block::S_30;
    }
    loop {
        match c2rust_current_block {
            C2Rust_Block::S_30 => {
                if hShip.is_null() {
                    break;
                }
                Trackee = LockLink(&raw mut disp_q, hShip) as *mut ELEMENT;
                hNextShip = (*Trackee).succ;
                if (*Trackee).state_flags as ::core::ffi::c_int
                    & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int != 0
                    && elementsOfSamePlayer(Trackee, Tracker).0 == 0
                    && (!((*GlobData
                        .Game_state
                        .DisplayArray
                        .offset((*Trackee).PrimIndex as isize))
                        .Type as ::core::ffi::c_int
                        >= gfx_object::NUM_PRIMS.0 as ::core::ffi::c_int
                        || (*GlobData
                            .Game_state
                            .DisplayArray
                            .offset((*Trackee).PrimIndex as isize))
                            .Type as ::core::ffi::c_int
                            == gfx_object::STAMPFILL_PRIM.0 as ::core::ffi::c_int
                            && sameColor(
                                (*GlobData
                                    .Game_state
                                    .DisplayArray
                                    .offset((*Trackee).PrimIndex as isize))
                                    .color,
                                colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE),
                            ) as ::core::ffi::c_int != 0)
                        || (*Tracker).state_flags as ::core::ffi::c_int
                            & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int != 0
                            && (*Tracker).state_flags as ::core::ffi::c_int
                                & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int != 0)
                {
                    StarShipPtr = ::core::ptr::null_mut::<STARSHIP>();
                    c2rust_current_block = C2Rust_Block::CheckTracking;
                    continue;
                }
            }
            C2Rust_Block::CheckTracking => {
                StarShipPtr = (*Trackee).pParent as *mut STARSHIP;
                if (*Trackee).c2rust_unnamed.life_span as ::core::ffi::c_int != 0
                    && (*(*StarShipPtr).RaceDescPtr).ship_info.crew_level
                        as ::core::ffi::c_int != 0
                {
                    let mut delta_x: SIZE = 0;
                    let mut delta_y: SIZE = 0;
                    let mut delta_facing: SIZE = 0;
                    if (*Tracker).state_flags as ::core::ffi::c_int
                        & (1 as ::core::ffi::c_int) << 11 as ::core::ffi::c_int != 0
                    {
                        delta_x = ((*Trackee).next.location.x as ::core::ffi::c_int
                            - (*Tracker).next.location.x as ::core::ffi::c_int) as SIZE;
                        delta_y = ((*Trackee).next.location.y as ::core::ffi::c_int
                            - (*Tracker).next.location.y as ::core::ffi::c_int) as SIZE;
                    } else {
                        delta_x = ((*Trackee).current.location.x as ::core::ffi::c_int
                            - (*Tracker).current.location.x as ::core::ffi::c_int)
                            as SIZE;
                        delta_y = ((*Trackee).current.location.y as ::core::ffi::c_int
                            - (*Tracker).current.location.y as ::core::ffi::c_int)
                            as SIZE;
                    }
                    delta_x = (if (delta_x as ::core::ffi::c_int)
                        < 0 as ::core::ffi::c_int
                    {
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
                    delta_y = (if (delta_y as ::core::ffi::c_int)
                        < 0 as ::core::ffi::c_int
                    {
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
                    delta_facing = ((ARCTAN(delta_x, delta_y) as ::core::ffi::c_int
                        + ((1 as ::core::ffi::c_int)
                            << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int)
                        >> 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int)
                        - *pfacing as ::core::ffi::c_int
                        & ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                            - 1 as ::core::ffi::c_int) as COUNT as SIZE;
                    if (delta_x as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                        delta_x = -(delta_x as ::core::ffi::c_int) as SIZE;
                    }
                    if (delta_y as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                        delta_y = -(delta_y as ::core::ffi::c_int) as SIZE;
                    }
                    delta_x = (delta_x as ::core::ffi::c_int
                        + delta_y as ::core::ffi::c_int) as SIZE;
                    if best_delta as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                        || (delta_x as ::core::ffi::c_int)
                            < best_delta as ::core::ffi::c_int
                    {
                        best_delta = delta_x;
                        best_delta_facing = delta_facing;
                        (*Tracker).hTarget = hShip;
                    }
                }
            }
        }
        UnlockLink(&raw mut disp_q, hShip);
        hShip = hNextShip;
        c2rust_current_block = C2Rust_Block::S_30;
    }
    if best_delta_facing as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
        let mut facing: COUNT = 0;
        facing = *pfacing;
        if best_delta_facing as ::core::ffi::c_int
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
        } else if (best_delta_facing as ::core::ffi::c_int)
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
        *pfacing = (facing as ::core::ffi::c_int
            & ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                - 1 as ::core::ffi::c_int) as COUNT;
    }
    return best_delta_facing;
}
