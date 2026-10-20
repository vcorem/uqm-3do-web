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

// Rust translation of src/uqm/ships/pkunk/pkunk.c.
extern "C" {
    pub type string_table_entry;
    pub type string_table;
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
    fn InsertQueue(pq: *mut QUEUE, hLink: HLINK, hRefLink: HLINK);
    fn SetAbsFrameIndex(Frame: FRAME, FrameIndex: COUNT) -> FRAME;
    fn SetEquFrameIndex(DstFrame: FRAME, SrcFrame: FRAME) -> FRAME;
    fn IncFrameIndex(Frame: FRAME) -> FRAME;
    fn CaptureStringTable(StringTable: STRING_TABLE) -> STRING;
    fn ReleaseStringTable(String: STRING) -> STRING_TABLE;
    fn GetStringTableCount(String: STRING) -> COUNT;
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
    fn GetCurrentVelocityComponentsSdword(
        velocityptr: *mut VELOCITY_DESC,
        pdx: *mut SDWORD,
        pdy: *mut SDWORD,
    );
    fn DeltaVelocityComponents(velocityptr: *mut VELOCITY_DESC, dx: SDWORD, dy: SDWORD);
    static mut disp_q: QUEUE;
    fn AllocElement() -> HELEMENT;
    fn FreeElement(hElement: HELEMENT);
    fn RemoveElement(hLink: HLINK);
    fn TimeSpaceMatterConflict(ElementPtr: *mut ELEMENT) -> BOOLEAN;
    fn CalculateGravity(ElementPtr: *mut ELEMENT) -> BOOLEAN;
    fn LoadSoundInstance(res: RESOURCE) -> SOUND_REF;
    fn DestroySound(SoundRef: SOUND_REF) -> BOOLEAN;
    static mut GlobData: GLOBDATA;
    fn TFB_Random() -> DWORD;
    fn ProcessSound(Sound: SOUND, PositionalObject: *mut ELEMENT);
    fn DeltaEnergy(ElementPtr: *mut ELEMENT, energy_delta: SIZE) -> BOOLEAN;
    fn ship_intelligence(
        ShipPtr: *mut ELEMENT,
        ObjectsOfConcern: *mut EVALUATE_DESC,
        ConcernCounter: COUNT,
    );
    fn initialize_missile(pMissileBlock: *mut MISSILE_BLOCK) -> HELEMENT;
    fn StopAllBattleMusic();
    fn StartShipExplosion(ShipPtr: *mut ELEMENT, playSound: bool);
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
pub type STRING_TABLE_DESC = string_table;
pub type STRING_TABLE = *mut STRING_TABLE_DESC;
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
pub type SOUND_REF = STRING_TABLE;
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
pub type PKUNK_DATA = CustomShipData_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CustomShipData_t {
    pub hPhoenix: HELEMENT,
    pub saved_preprocess_func: Option<ElementProcessFunc>,
    pub saved_postprocess_func: Option<ElementProcessFunc>,
    pub saved_death_func: Option<ElementProcessFunc>,
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
static mut pkunk_desc: RACE_DESC = unsafe {
    race_desc {
        c2rust_padding: [0; 8],
        c2rust_padding_0: [0; 10],
        c2rust_padding_1: [0; 2],
        c2rust_padding_2: [0; 8],
        ship_info: SHIP_INFO {
            idStr: b"fury\0".as_ptr() as *const ::core::ffi::c_char,
            ship_flags: ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as UWORD,
            ship_cost: 20 as BYTE,
            crew_level: 8 as COUNT,
            max_crew: 8 as COUNT,
            energy_level: 12 as BYTE,
            max_energy: 12 as BYTE,
            race_strings_rsc: b"ship.pkunk.text\0".as_ptr() as RESOURCE,
            icons_rsc: b"ship.pkunk.icons\0".as_ptr() as RESOURCE,
            melee_icon_rsc: b"ship.pkunk.meleeicons\0".as_ptr() as RESOURCE,
            race_strings: ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>(),
            icons: ::core::ptr::null_mut::<FRAME_DESC>(),
            melee_icon: ::core::ptr::null_mut::<FRAME_DESC>(),
        },
        fleet: FLEET_STUFF {
            strength: (666 as ::core::ffi::c_int / 11 as ::core::ffi::c_int
                * 2 as ::core::ffi::c_int) as COUNT,
            known_loc: point {
                x: 502 as COORD,
                y: 401 as COORD,
            },
        },
        characteristics: CHARACTERISTIC_STUFF {
            max_thrust: 64 as COUNT,
            thrust_increment: 16 as COUNT,
            energy_regeneration: 0 as BYTE,
            weapon_energy_cost: 1 as BYTE,
            special_energy_cost: 2 as BYTE,
            energy_wait: 0 as BYTE,
            turn_wait: 0 as BYTE,
            thrust_wait: 0 as BYTE,
            weapon_wait: 0 as BYTE,
            special_wait: 0 as BYTE,
            ship_mass: 1 as BYTE,
        },
        ship_data: DATA_STUFF {
            ship_rsc: [
                b"ship.pkunk.graphics.fury.large\0".as_ptr() as RESOURCE,
                b"ship.pkunk.graphics.fury.medium\0".as_ptr() as RESOURCE,
                b"ship.pkunk.graphics.fury.small\0".as_ptr() as RESOURCE,
            ],
            weapon_rsc: [
                b"ship.pkunk.graphics.bug.large\0".as_ptr() as RESOURCE,
                b"ship.pkunk.graphics.bug.medium\0".as_ptr() as RESOURCE,
                b"ship.pkunk.graphics.bug.small\0".as_ptr() as RESOURCE,
            ],
            special_rsc: [
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
            ],
            captain_control: captain_stuff {
                captain_rsc: b"ship.pkunk.graphics.captain\0".as_ptr() as RESOURCE,
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
            victory_ditty_rsc: b"ship.pkunk.ditty\0".as_ptr() as RESOURCE,
            ship_sounds_rsc: b"ship.pkunk.sounds\0".as_ptr() as RESOURCE,
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
            WeaponRange: (((50 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                + 1 as ::core::ffi::c_int) as COUNT,
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
unsafe extern "C" fn initialize_bug_missile(
    mut ShipPtr: *mut ELEMENT,
    mut MissileArray: *mut HELEMENT,
) -> COUNT {
    let mut i: COUNT = 0;
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
    MissileBlock.index = 0 as COUNT;
    MissileBlock.sender = (*ShipPtr).playerNr;
    MissileBlock.flags = ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
        as ELEMENT_FLAGS;
    MissileBlock.pixoffs = ((15 as ::core::ffi::c_int) << resolutionFactor) as SIZE;
    MissileBlock.speed = (((24 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
        << resolutionFactor) as SIZE;
    MissileBlock.hit_points = 1 as SIZE;
    MissileBlock.damage = 1 as SIZE;
    MissileBlock.life = 5 as COUNT;
    MissileBlock.preprocess_func = None;
    MissileBlock.blast_offs = ((1 as ::core::ffi::c_int) << resolutionFactor) as SIZE;
    i = 0 as COUNT;
    while (i as ::core::ffi::c_int) < 3 as ::core::ffi::c_int {
        MissileBlock.face = ((*StarShipPtr).ShipFacing as ::core::ffi::c_int
            + (((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                >> 2 as ::core::ffi::c_int)
                + ((1 as ::core::ffi::c_int)
                    << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int)
                >> 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int)
                * i as ::core::ffi::c_int) as COUNT;
        if i as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            MissileBlock.face = (MissileBlock.face as ::core::ffi::c_int
                + (((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                    >> 2 as ::core::ffi::c_int)
                    + ((1 as ::core::ffi::c_int)
                        << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int)
                    >> 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int)) as COUNT;
        }
        MissileBlock.face = (MissileBlock.face as ::core::ffi::c_int
            & ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                - 1 as ::core::ffi::c_int) as DWORD as COUNT;
        *MissileArray.offset(i as isize) = initialize_missile(&raw mut MissileBlock);
        if !(*MissileArray.offset(i as isize)).is_null() {
            let mut dx: SDWORD = 0;
            let mut dy: SDWORD = 0;
            let mut MissilePtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
            MissilePtr = LockLink(&raw mut disp_q, *MissileArray.offset(i as isize))
                as *mut ELEMENT;
            GetCurrentVelocityComponentsSdword(
                &raw mut (*ShipPtr).velocity,
                &raw mut dx,
                &raw mut dy,
            );
            DeltaVelocityComponents(&raw mut (*MissilePtr).velocity, dx, dy);
            (*MissilePtr).current.location.x = ((*MissilePtr).current.location.x
                as ::core::ffi::c_int
                - (dx >> 5 as ::core::ffi::c_int) as ::core::ffi::c_int) as COORD;
            (*MissilePtr).current.location.y = ((*MissilePtr).current.location.y
                as ::core::ffi::c_int
                - (dy >> 5 as ::core::ffi::c_int) as ::core::ffi::c_int) as COORD;
            (*MissilePtr).preprocess_func = Some(
                animate as unsafe extern "C" fn(*mut ELEMENT) -> (),
            ) as Option<ElementProcessFunc>;
            UnlockLink(&raw mut disp_q, *MissileArray.offset(i as isize));
        }
        i = i.wrapping_add(1);
    }
    return 3 as COUNT;
}
unsafe extern "C" fn pkunk_intelligence(
    mut ShipPtr: *mut ELEMENT,
    mut ObjectsOfConcern: *mut EVALUATE_DESC,
    mut ConcernCounter: COUNT,
) {
    let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
    let mut PkunkData: *mut PKUNK_DATA = ::core::ptr::null_mut::<PKUNK_DATA>();
    StarShipPtr = (*ShipPtr).pParent as *mut STARSHIP;
    PkunkData = GetCustomShipData((*StarShipPtr).RaceDescPtr) as *mut PKUNK_DATA;
    if !(*PkunkData).hPhoenix.is_null()
        && (*StarShipPtr).control as ::core::ffi::c_int
            & ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as BYTE
                as ::core::ffi::c_int != 0
    {
        RemoveElement((*PkunkData).hPhoenix);
        FreeElement((*PkunkData).hPhoenix);
        (*PkunkData).hPhoenix = ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    if ((*(*StarShipPtr).RaceDescPtr).ship_info.energy_level as ::core::ffi::c_int)
        < (*(*StarShipPtr).RaceDescPtr).ship_info.max_energy as ::core::ffi::c_int
        && ((*StarShipPtr).special_counter as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
            || (TFB_Random() as BYTE as ::core::ffi::c_int) < 20 as ::core::ffi::c_int)
    {
        (*StarShipPtr).ship_input_state = ((*StarShipPtr).ship_input_state
            as ::core::ffi::c_int | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
            as BYTE;
    } else {
        (*StarShipPtr).ship_input_state = ((*StarShipPtr).ship_input_state
            as ::core::ffi::c_int
            & !((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)) as BYTE;
    }
    uqm_lua_checked!(ship_intelligence(ShipPtr, ObjectsOfConcern, ConcernCounter), ());
}
unsafe extern "C" fn new_pkunk(mut ElementPtr: *mut ELEMENT) {
    let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
    let mut PkunkData: *mut PKUNK_DATA = ::core::ptr::null_mut::<PKUNK_DATA>();
    StarShipPtr = (*ElementPtr).pParent as *mut STARSHIP;
    PkunkData = GetCustomShipData((*StarShipPtr).RaceDescPtr) as *mut PKUNK_DATA;
    (*ElementPtr).state_flags = ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
        | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
        | (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ELEMENT_FLAGS;
    (*ElementPtr).c2rust_unnamed_1.mass_points = 1 as BYTE;
    (*ElementPtr).preprocess_func = (*PkunkData).saved_preprocess_func;
    (*ElementPtr).postprocess_func = (*PkunkData).saved_postprocess_func;
    (*ElementPtr).death_func = (*PkunkData).saved_death_func;
    (*(*StarShipPtr).RaceDescPtr).preprocess_func = Some(
        pkunk_preprocess as unsafe extern "C" fn(*mut ELEMENT) -> (),
    ) as Option<PREPROCESS_FUNC>;
    (*(*StarShipPtr).RaceDescPtr).ship_info.crew_level = 8 as COUNT;
    (*(*StarShipPtr).RaceDescPtr).ship_info.energy_level = 12 as BYTE;
    (*(*StarShipPtr).RaceDescPtr).characteristics.max_thrust = ((64
        as ::core::ffi::c_int) << resolutionFactor) as COUNT;
    (*(*StarShipPtr).RaceDescPtr).characteristics.thrust_increment = ((16
        as ::core::ffi::c_int) << resolutionFactor) as COUNT;
    (*(*StarShipPtr).RaceDescPtr).characteristics.turn_wait = 0 as BYTE;
    (*(*StarShipPtr).RaceDescPtr).characteristics.thrust_wait = 0 as BYTE;
    (*(*StarShipPtr).RaceDescPtr).characteristics.special_wait = 0 as BYTE;
    (*StarShipPtr).ship_input_state = 0 as BYTE;
    (*StarShipPtr).cur_status_flags = ((*StarShipPtr).cur_status_flags
        as ::core::ffi::c_int & (1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int)
        as STATUS_FLAGS;
    (*StarShipPtr).old_status_flags = 0 as STATUS_FLAGS;
    (*StarShipPtr).energy_counter = 0 as BYTE;
    (*StarShipPtr).weapon_counter = 0 as BYTE;
    (*StarShipPtr).special_counter = 0 as BYTE;
    (*ElementPtr).c2rust_unnamed_0.crew_level = 0 as COUNT;
    (*ElementPtr).c2rust_unnamed_2.turn_wait = 0 as BYTE;
    (*ElementPtr).c2rust_unnamed_3.thrust_wait = 0 as BYTE;
    (*ElementPtr).c2rust_unnamed.life_span = 1 as COUNT;
    (*StarShipPtr).ShipFacing = (TFB_Random()
        & (((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
            - 1 as ::core::ffi::c_int) as DWORD) as COUNT;
    (*ElementPtr).current.image.farray = &raw mut (*(*StarShipPtr).RaceDescPtr)
        .ship_data
        .ship as *mut FRAME;
    (*ElementPtr).current.image.frame = SetAbsFrameIndex(
        (*(*StarShipPtr).RaceDescPtr).ship_data.ship[0usize],
        (*StarShipPtr).ShipFacing,
    );
    (*GlobData.Game_state.DisplayArray.offset((*ElementPtr).PrimIndex as isize)).Type = gfx_object::STAMP_PRIM
        .0 as ::core::ffi::c_int as GRAPHICS_PRIM;
    loop {
        (*ElementPtr).current.location.x = (if TFB_Random()
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
                        << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as DWORD,
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
                        << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as DWORD,
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
                        << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as DWORD,
                ) as ::core::ffi::c_int
                & !(((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int)
        }) as DWORD as COORD;
        (*ElementPtr).current.location.y = (if TFB_Random()
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
                        << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as DWORD,
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
                        << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as DWORD,
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
                        << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as DWORD,
                ) as ::core::ffi::c_int
                & !(((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int)
        }) as DWORD as COORD;
        if !(CalculateGravity(ElementPtr).0 != 0
            || uqm_lua_checked!(TimeSpaceMatterConflict(ElementPtr), ()).0 != 0)
        {
            break;
        }
    }
    memset(
        &raw mut (*ElementPtr).velocity as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<VELOCITY_DESC>(),
    );
    (*ElementPtr).hTarget = (*StarShipPtr).hShip;
}
unsafe extern "C" fn pkunk_reincarnation_death(mut ShipPtr: *mut ELEMENT) {
    uqm_lua_checked!(StopAllBattleMusic(), ());
    uqm_lua_checked!(StartShipExplosion(ShipPtr, true), ());
    (*ShipPtr).death_func = Some(new_pkunk as unsafe extern "C" fn(*mut ELEMENT) -> ())
        as Option<ElementProcessFunc>;
}
unsafe extern "C" fn intercept_pkunk_death(mut ElementPtr: *mut ELEMENT) {
    let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
    let mut PkunkData: *mut PKUNK_DATA = ::core::ptr::null_mut::<PKUNK_DATA>();
    let mut ShipPtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
    StarShipPtr = (*ElementPtr).pParent as *mut STARSHIP;
    PkunkData = GetCustomShipData((*StarShipPtr).RaceDescPtr) as *mut PKUNK_DATA;
    if (*(*StarShipPtr).RaceDescPtr).ship_info.crew_level as ::core::ffi::c_int
        != 0 as ::core::ffi::c_int
    {
        (*ElementPtr).state_flags = ((*ElementPtr).state_flags as ::core::ffi::c_int
            & !((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)) as ELEMENT_FLAGS;
        (*ElementPtr).c2rust_unnamed.life_span = 1 as COUNT;
        return;
    }
    ShipPtr = LockLink(&raw mut disp_q, (*StarShipPtr).hShip) as *mut ELEMENT;
    if !((*ShipPtr).c2rust_unnamed_1.mass_points as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int > 10 as ::core::ffi::c_int * 10 as ::core::ffi::c_int)
    {
        (*ShipPtr).c2rust_unnamed_1.mass_points = (10 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int) as BYTE;
        (*PkunkData).saved_preprocess_func = (*ShipPtr).preprocess_func;
        (*PkunkData).saved_postprocess_func = (*ShipPtr).postprocess_func;
        (*PkunkData).saved_death_func = (*ShipPtr).death_func;
        (*ShipPtr).death_func = Some(
            pkunk_reincarnation_death as unsafe extern "C" fn(*mut ELEMENT) -> (),
        ) as Option<ElementProcessFunc>;
    }
    UnlockLink(&raw mut disp_q, (*StarShipPtr).hShip);
}
unsafe extern "C" fn spawn_phoenix_trail(mut ElementPtr: *mut ELEMENT) {
    static mut colorTable: [Color; 12] = [
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
            g: ((0x11 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x11 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
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
            g: ((0xa as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0xa as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
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
            g: ((0x3 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x3 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
        Color {
            r: ((0x1f as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x1f as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
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
            r: ((0x17 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x17 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
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
        Color {
            r: ((0xf as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0xf as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
        Color {
            r: ((0xb as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0xb as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
    ];
    let colorTableCount: size_t = ::core::mem::size_of::<[Color; 12]>()
        .wrapping_div(::core::mem::size_of::<Color>());
    (*ElementPtr).colorCycleIndex = (*ElementPtr).colorCycleIndex.wrapping_add(1);
    if (*ElementPtr).colorCycleIndex as size_t != colorTableCount {
        (*ElementPtr).c2rust_unnamed.life_span = 1 as COUNT;
        (*GlobData.Game_state.DisplayArray.offset((*ElementPtr).PrimIndex as isize))
            .color = colorTable[(*ElementPtr).colorCycleIndex as usize];
        (*ElementPtr).state_flags = ((*ElementPtr).state_flags as ::core::ffi::c_int
            & !((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)) as ELEMENT_FLAGS;
        (*ElementPtr).state_flags = ((*ElementPtr).state_flags as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as ELEMENT_FLAGS;
    }
}
unsafe extern "C" fn phoenix_transition(mut ElementPtr: *mut ELEMENT) {
    let mut hShipImage: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut ShipImagePtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
    let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
    StarShipPtr = (*ElementPtr).pParent as *mut STARSHIP;
    ShipImagePtr = LockLink(&raw mut disp_q, (*StarShipPtr).hShip) as *mut ELEMENT;
    if (*ShipImagePtr).state_flags as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int == 0
    {
        (*ElementPtr).preprocess_func = None;
    } else {
        hShipImage = AllocElement();
        if !hShipImage.is_null() {
            let mut angle: COUNT = 0;
            PutQueue(&raw mut disp_q, hShipImage);
            ShipImagePtr = LockLink(&raw mut disp_q, hShipImage) as *mut ELEMENT;
            (*ShipImagePtr).playerNr = -1 as SIZE;
            (*ShipImagePtr).state_flags = ((1 as ::core::ffi::c_int)
                << 3 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as ELEMENT_FLAGS;
            (*ShipImagePtr).c2rust_unnamed.life_span = 1 as COUNT;
            (*GlobData
                .Game_state
                .DisplayArray
                .offset((*ShipImagePtr).PrimIndex as isize))
                .Type = gfx_object::STAMPFILL_PRIM.0 as ::core::ffi::c_int
                as GRAPHICS_PRIM;
            (*GlobData
                .Game_state
                .DisplayArray
                .offset((*ShipImagePtr).PrimIndex as isize))
                .color = colorFromRgb15(0x1f as BYTE, 0x15 as BYTE, 0 as BYTE);
            (*ShipImagePtr).colorCycleIndex = 0 as BYTE;
            (*ShipImagePtr).current.image = (*ElementPtr).current.image;
            (*ShipImagePtr).current.location = (*ElementPtr).current.location;
            if (*ElementPtr).state_flags as ::core::ffi::c_int
                & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int == 0
            {
                angle = (*ElementPtr).c2rust_unnamed_1.mass_points as COUNT;
                (*ShipImagePtr).current.location.x = ((*ShipImagePtr).current.location.x
                    as ::core::ffi::c_int
                    + (*(&raw mut sinetab as *mut SDWORD)
                        .offset(
                            (angle as ::core::ffi::c_int
                                + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                                    >> 2 as ::core::ffi::c_int)
                                & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                                    - 1 as ::core::ffi::c_int) as DWORD as isize,
                        ) as ::core::ffi::c_long
                        * ((20 as ::core::ffi::c_int) << resolutionFactor
                            << 2 as ::core::ffi::c_int) as ::core::ffi::c_long
                        >> 14 as ::core::ffi::c_int) as SDWORD as ::core::ffi::c_int)
                    as COORD;
                (*ShipImagePtr).current.location.y = ((*ShipImagePtr).current.location.y
                    as ::core::ffi::c_int
                    + (*(&raw mut sinetab as *mut SDWORD)
                        .offset(
                            (angle as ::core::ffi::c_int
                                & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                                    - 1 as ::core::ffi::c_int) as DWORD as isize,
                        ) as ::core::ffi::c_long
                        * ((20 as ::core::ffi::c_int) << resolutionFactor
                            << 2 as ::core::ffi::c_int) as ::core::ffi::c_long
                        >> 14 as ::core::ffi::c_int) as SDWORD as ::core::ffi::c_int)
                    as COORD;
                (*ElementPtr).preprocess_func = None;
            } else {
                let mut temp_x: SDWORD = 0;
                let mut temp_y: SDWORD = 0;
                angle = (((*StarShipPtr).ShipFacing as ::core::ffi::c_int)
                    << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int) as COUNT;
                temp_x = ((*ShipImagePtr).current.location.x as ::core::ffi::c_int
                    - (*(&raw mut sinetab as *mut SDWORD)
                        .offset(
                            (angle as ::core::ffi::c_int
                                + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                                    >> 2 as ::core::ffi::c_int)
                                & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                                    - 1 as ::core::ffi::c_int) as DWORD as isize,
                        ) as ::core::ffi::c_long
                        * ((20 as ::core::ffi::c_int) << resolutionFactor
                            << 2 as ::core::ffi::c_int) as ::core::ffi::c_long
                        >> 14 as ::core::ffi::c_int) as ::core::ffi::c_int
                        * ((*ElementPtr).c2rust_unnamed.life_span as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int)) as SDWORD;
                temp_y = ((*ShipImagePtr).current.location.y as ::core::ffi::c_int
                    - (*(&raw mut sinetab as *mut SDWORD)
                        .offset(
                            (angle as ::core::ffi::c_int
                                & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                                    - 1 as ::core::ffi::c_int) as DWORD as isize,
                        ) as ::core::ffi::c_long
                        * ((20 as ::core::ffi::c_int) << resolutionFactor
                            << 2 as ::core::ffi::c_int) as ::core::ffi::c_long
                        >> 14 as ::core::ffi::c_int) as ::core::ffi::c_int
                        * ((*ElementPtr).c2rust_unnamed.life_span as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int)) as SDWORD;
                (*ShipImagePtr).current.location.x = (if temp_x < 0 as ::core::ffi::c_int
                {
                    temp_x
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
                    temp_x
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
                }) as DWORD as COORD;
                (*ShipImagePtr).current.location.y = (if temp_y < 0 as ::core::ffi::c_int
                {
                    temp_y
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
                    temp_y
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
                }) as DWORD as COORD;
            }
            (*ShipImagePtr).c2rust_unnamed_1.mass_points = angle as BYTE;
            (*ShipImagePtr).preprocess_func = Some(
                phoenix_transition as unsafe extern "C" fn(*mut ELEMENT) -> (),
            ) as Option<ElementProcessFunc>;
            (*ShipImagePtr).death_func = Some(
                spawn_phoenix_trail as unsafe extern "C" fn(*mut ELEMENT) -> (),
            ) as Option<ElementProcessFunc>;
            (*ShipImagePtr).pParent = StarShipPtr as *mut ::core::ffi::c_void;
            UnlockLink(&raw mut disp_q, hShipImage);
        }
    }
    UnlockLink(&raw mut disp_q, (*StarShipPtr).hShip);
}
#[no_mangle]
pub static mut FwiffoSounds: SOUND = ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>();
unsafe extern "C" fn pkunk_preprocess(mut ElementPtr: *mut ELEMENT) {
    let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
    let mut PkunkData: *mut PKUNK_DATA = ::core::ptr::null_mut::<PKUNK_DATA>();
    StarShipPtr = (*ElementPtr).pParent as *mut STARSHIP;
    PkunkData = GetCustomShipData((*StarShipPtr).RaceDescPtr) as *mut PKUNK_DATA;
    let mut Fwiffo: BOOLEAN = BOOLEAN(
        ((*StarShipPtr).captains_name_index as ::core::ffi::c_int
            == 3 as ::core::ffi::c_int + 2 as ::core::ffi::c_int
                + 16 as ::core::ffi::c_int
            && (*StarShipPtr).SpeciesID.0 == SPECIES_ID::SPATHI_ID.0)
            as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
    if Fwiffo.0 != 0 && FwiffoSounds.is_null() {
        FwiffoSounds = CaptureStringTable(
            uqm_lua_checked!(LoadSoundInstance(b"ship.spathi.sounds\0".as_ptr() as RESOURCE), ()),
        ) as SOUND;
    }
    if (*ElementPtr).state_flags as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int != 0
    {
        let mut hPhoenix: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if TFB_Random() & 1 as DWORD != 0 {
            hPhoenix = AllocElement();
        }
        if !hPhoenix.is_null() {
            let mut PhoenixPtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
            PhoenixPtr = LockLink(&raw mut disp_q, hPhoenix) as *mut ELEMENT;
            (*PhoenixPtr).playerNr = (*ElementPtr).playerNr;
            (*PhoenixPtr).state_flags = ((1 as ::core::ffi::c_int)
                << 10 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ELEMENT_FLAGS;
            (*PhoenixPtr).c2rust_unnamed.life_span = 1 as COUNT;
            (*PhoenixPtr).death_func = Some(
                intercept_pkunk_death as unsafe extern "C" fn(*mut ELEMENT) -> (),
            ) as Option<ElementProcessFunc>;
            (*PhoenixPtr).pParent = StarShipPtr as *mut ::core::ffi::c_void;
            UnlockLink(&raw mut disp_q, hPhoenix);
            InsertQueue(&raw mut disp_q, hPhoenix, disp_q.head);
        }
        (*PkunkData).hPhoenix = hPhoenix;
        if (*ElementPtr).hTarget.is_null() {
            (*(*StarShipPtr).RaceDescPtr).preprocess_func = None;
        } else {
            let mut angle: COUNT = 0;
            let mut facing: COUNT = 0;
            if Fwiffo.0 != 0 {
                ProcessSound(
                    SetAbsStringTableIndex(FwiffoSounds, 1 as COUNT),
                    ElementPtr,
                );
            } else {
                ProcessSound(
                    SetAbsStringTableIndex(
                        (*(*StarShipPtr).RaceDescPtr).ship_data.ship_sounds,
                        1 as COUNT,
                    ),
                    ElementPtr,
                );
            }
            (*ElementPtr).c2rust_unnamed.life_span = 12 as COUNT;
            (*GlobData.Game_state.DisplayArray.offset((*ElementPtr).PrimIndex as isize))
                .Type = gfx_object::NUM_PRIMS.0 as ::core::ffi::c_int as GRAPHICS_PRIM;
            (*ElementPtr).state_flags = ((*ElementPtr).state_flags as ::core::ffi::c_int
                | ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int))
                as ELEMENT_FLAGS;
            facing = (*StarShipPtr).ShipFacing;
            angle = ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                >> 3 as ::core::ffi::c_int) as COUNT;
            while (angle as ::core::ffi::c_int)
                < (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
            {
                (*StarShipPtr).ShipFacing = (facing as ::core::ffi::c_int
                    + (angle as ::core::ffi::c_int
                        + ((1 as ::core::ffi::c_int)
                            << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int)
                        >> 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int)
                    & ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int) as DWORD as COUNT;
                phoenix_transition(ElementPtr);
                angle = (angle as ::core::ffi::c_int
                    + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                        >> 2 as ::core::ffi::c_int)) as COUNT;
            }
            (*StarShipPtr).ShipFacing = facing;
        }
    }
    if (*(*StarShipPtr).RaceDescPtr).preprocess_func.is_some() {
        (*StarShipPtr).cur_status_flags = ((*StarShipPtr).cur_status_flags
            as ::core::ffi::c_int
            & !((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)) as STATUS_FLAGS;
        if (*ElementPtr).c2rust_unnamed.life_span as ::core::ffi::c_int
            == 1 as ::core::ffi::c_int
        {
            (*ElementPtr).next.image.frame = SetEquFrameIndex(
                *(*ElementPtr).current.image.farray.offset(0isize),
                (*ElementPtr).current.image.frame,
            );
            (*ElementPtr).current.image.frame = (*ElementPtr).next.image.frame;
            (*GlobData.Game_state.DisplayArray.offset((*ElementPtr).PrimIndex as isize))
                .Type = gfx_object::STAMP_PRIM.0 as ::core::ffi::c_int as GRAPHICS_PRIM;
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
            memset(
                &raw mut (*ElementPtr).velocity as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<VELOCITY_DESC>(),
            );
            (*ElementPtr).state_flags = ((*ElementPtr).state_flags as ::core::ffi::c_int
                & !((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int))
                as ELEMENT_FLAGS;
            (*ElementPtr).state_flags = ((*ElementPtr).state_flags as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as ELEMENT_FLAGS;
            (*(*StarShipPtr).RaceDescPtr).preprocess_func = None;
        }
    }
}
static mut LastSound: COUNT = 0 as COUNT;
unsafe extern "C" fn pkunk_postprocess(mut ElementPtr: *mut ELEMENT) {
    let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
    StarShipPtr = (*ElementPtr).pParent as *mut STARSHIP;
    if (*(*StarShipPtr).RaceDescPtr).characteristics.special_wait != 0 {
        (*(*StarShipPtr).RaceDescPtr).characteristics.special_wait = (*(*StarShipPtr)
            .RaceDescPtr)
            .characteristics
            .special_wait
            .wrapping_sub(1);
    } else if (*StarShipPtr).cur_status_flags as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int != 0
        && ((*(*StarShipPtr).RaceDescPtr).ship_info.energy_level as ::core::ffi::c_int)
            < (*(*StarShipPtr).RaceDescPtr).ship_info.max_energy as ::core::ffi::c_int
    {
        let mut CurSound: COUNT = 0;
        loop {
            CurSound = (2 as ::core::ffi::c_int
                + TFB_Random() as COUNT as ::core::ffi::c_int
                    % (GetStringTableCount(
                        (*(*StarShipPtr).RaceDescPtr).ship_data.ship_sounds,
                    ) as ::core::ffi::c_int - 2 as ::core::ffi::c_int)) as COUNT;
            if CurSound as ::core::ffi::c_int != LastSound as ::core::ffi::c_int {
                break;
            }
        }
        ProcessSound(
            SetAbsStringTableIndex(
                (*(*StarShipPtr).RaceDescPtr).ship_data.ship_sounds,
                CurSound,
            ),
            ElementPtr,
        );
        LastSound = CurSound;
        uqm_lua_checked!(DeltaEnergy(ElementPtr, 2 as SIZE), ());
        (*(*StarShipPtr).RaceDescPtr).characteristics.special_wait = 16 as BYTE;
    }
}
unsafe extern "C" fn uninit_pkunk(mut pRaceDesc: *mut RACE_DESC) {
    SetCustomShipData(pRaceDesc, ::core::ptr::null::<CustomShipData_t>());
    if !FwiffoSounds.is_null() {
        uqm_lua_checked!(DestroySound(ReleaseStringTable(FwiffoSounds)), ());
        FwiffoSounds = ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>();
    }
}
#[no_mangle]
pub unsafe extern "C" fn init_pkunk() -> *mut RACE_DESC {
    static mut new_pkunk_desc: RACE_DESC = RACE_DESC {
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
    let mut RaceDescPtr: *mut RACE_DESC = ::core::ptr::null_mut::<RACE_DESC>();
    let mut empty_data: PKUNK_DATA = PKUNK_DATA {
        hPhoenix: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        saved_preprocess_func: None,
        saved_postprocess_func: None,
        saved_death_func: None,
    };
    if if resolutionFactor != 2 as ::core::ffi::c_uint {
        BOOLEAN::FALSE.0 as ::core::ffi::c_int
    } else {
        BOOLEAN::TRUE.0 as ::core::ffi::c_int
    } != 0
    {
        pkunk_desc.characteristics.max_thrust = ((64 as ::core::ffi::c_int)
            << resolutionFactor) as COUNT;
        pkunk_desc.characteristics.thrust_increment = ((16 as ::core::ffi::c_int)
            << resolutionFactor) as COUNT;
        pkunk_desc.cyborg_control.WeaponRange = (((200 as ::core::ffi::c_int)
            << 2 as ::core::ffi::c_int) + 4 as ::core::ffi::c_int) as COUNT;
    } else {
        pkunk_desc.characteristics.max_thrust = 64 as COUNT;
        pkunk_desc.characteristics.thrust_increment = 16 as COUNT;
        pkunk_desc.cyborg_control.WeaponRange = (((50 as ::core::ffi::c_int)
            << 2 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int) as COUNT;
    }
    memset(
        &raw mut empty_data as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<PKUNK_DATA>(),
    );
    pkunk_desc.uninit_func = Some(
        uninit_pkunk as unsafe extern "C" fn(*mut RACE_DESC) -> (),
    ) as Option<UNINIT_FUNC>;
    pkunk_desc.preprocess_func = Some(
        pkunk_preprocess as unsafe extern "C" fn(*mut ELEMENT) -> (),
    ) as Option<PREPROCESS_FUNC>;
    pkunk_desc.postprocess_func = Some(
        pkunk_postprocess as unsafe extern "C" fn(*mut ELEMENT) -> (),
    ) as Option<POSTPROCESS_FUNC>;
    pkunk_desc.init_weapon_func = Some(
        initialize_bug_missile
            as unsafe extern "C" fn(*mut ELEMENT, *mut HELEMENT) -> COUNT,
    ) as Option<INIT_WEAPON_FUNC>;
    pkunk_desc.cyborg_control.intelligence_func = Some(
        pkunk_intelligence
            as unsafe extern "C" fn(*mut ELEMENT, *mut EVALUATE_DESC, COUNT) -> (),
    ) as Option<IntelligenceFunc>;
    new_pkunk_desc = pkunk_desc;
    SetCustomShipData(&raw mut new_pkunk_desc, &raw mut empty_data);
    RaceDescPtr = &raw mut new_pkunk_desc;
    LastSound = 0 as COUNT;
    return RaceDescPtr;
}
