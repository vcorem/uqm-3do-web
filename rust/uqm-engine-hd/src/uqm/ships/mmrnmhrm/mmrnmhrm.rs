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

// Rust translation of src/uqm/ships/mmrnmhrm/mmrnmhrm.c.
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
    fn HMalloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn HFree(p: *mut ::core::ffi::c_void);
    fn GetFrameIndex(Frame: FRAME) -> COUNT;
    fn SetAbsFrameIndex(Frame: FRAME, FrameIndex: COUNT) -> FRAME;
    fn SetEquFrameIndex(DstFrame: FRAME, SrcFrame: FRAME) -> FRAME;
    fn SetAbsStringTableIndex(String: STRING, StringTableIndex: COUNT) -> STRING;
    static mut resolutionFactor: ::core::ffi::c_uint;
    static mut sinetab: [SDWORD; 0];
    fn ARCTAN(delta_x: SDWORD, delta_y: SDWORD) -> COUNT;
    fn GetCurrentVelocityComponentsSdword(
        velocityptr: *mut VELOCITY_DESC,
        pdx: *mut SDWORD,
        pdy: *mut SDWORD,
    );
    fn SetVelocityVector(
        velocityptr: *mut VELOCITY_DESC,
        magnitude: SDWORD,
        facing: COUNT,
    );
    static mut disp_q: QUEUE;
    fn ProcessSound(Sound: SOUND, PositionalObject: *mut ELEMENT);
    fn DeltaEnergy(ElementPtr: *mut ELEMENT, energy_delta: SIZE) -> BOOLEAN;
    fn initialize_laser(pLaserBlock: *mut LASER_BLOCK) -> HELEMENT;
    fn initialize_missile(pMissileBlock: *mut MISSILE_BLOCK) -> HELEMENT;
    fn weapon_collision(
        ElementPtr0: *mut ELEMENT,
        pPt0: *mut POINT,
        ElementPtr1: *mut ELEMENT,
        pPt1: *mut POINT,
    ) -> HELEMENT;
    fn TrackShip(Tracker: *mut ELEMENT, pfacing: *mut COUNT) -> SIZE;
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
pub type CustomShipData_t = MMRNMHRM_DATA;
pub type MMRNMHRM_DATA = CHARACTERISTIC_STUFF;
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
static mut mmrnmhrm_desc: RACE_DESC = unsafe {
    race_desc {
        c2rust_padding: [0; 8],
        c2rust_padding_0: [0; 10],
        c2rust_padding_1: [0; 2],
        c2rust_padding_2: [0; 8],
        ship_info: SHIP_INFO {
            idStr: b"xform\0".as_ptr() as *const ::core::ffi::c_char,
            ship_flags: ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as UWORD,
            ship_cost: 19 as BYTE,
            crew_level: 20 as COUNT,
            max_crew: 20 as COUNT,
            energy_level: 10 as BYTE,
            max_energy: 10 as BYTE,
            race_strings_rsc: b"ship.mmrnmhrm.text\0".as_ptr() as RESOURCE,
            icons_rsc: b"ship.mmrnmhrm.icons\0".as_ptr() as RESOURCE,
            melee_icon_rsc: b"ship.mmrnmhrm.meleeicons\0".as_ptr() as RESOURCE,
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
            max_thrust: 20 as COUNT,
            thrust_increment: 5 as COUNT,
            energy_regeneration: 2 as BYTE,
            weapon_energy_cost: 1 as BYTE,
            special_energy_cost: 10 as BYTE,
            energy_wait: 6 as BYTE,
            turn_wait: 2 as BYTE,
            thrust_wait: 1 as BYTE,
            weapon_wait: 0 as BYTE,
            special_wait: 0 as BYTE,
            ship_mass: 3 as BYTE,
        },
        ship_data: DATA_STUFF {
            ship_rsc: [
                b"ship.mmrnmhrm.graphics.xform.large\0".as_ptr() as RESOURCE,
                b"ship.mmrnmhrm.graphics.xform.medium\0".as_ptr() as RESOURCE,
                b"ship.mmrnmhrm.graphics.xform.small\0".as_ptr() as RESOURCE,
            ],
            weapon_rsc: [
                b"ship.mmrnmhrm.graphics.torpedo.large\0".as_ptr() as RESOURCE,
                b"ship.mmrnmhrm.graphics.torpedo.medium\0".as_ptr() as RESOURCE,
                b"ship.mmrnmhrm.graphics.torpedo.small\0".as_ptr() as RESOURCE,
            ],
            special_rsc: [
                b"ship.mmrnmhrm.graphics.ywing.large\0".as_ptr() as RESOURCE,
                b"ship.mmrnmhrm.graphics.ywing.medium\0".as_ptr() as RESOURCE,
                b"ship.mmrnmhrm.graphics.ywing.small\0".as_ptr() as RESOURCE,
            ],
            captain_control: captain_stuff {
                captain_rsc: b"ship.mmrnmhrm.graphics.captain\0".as_ptr() as RESOURCE,
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
            victory_ditty_rsc: b"ship.mmrnmhrm.ditty\0".as_ptr() as RESOURCE,
            ship_sounds_rsc: b"ship.mmrnmhrm.sounds\0".as_ptr() as RESOURCE,
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
unsafe extern "C" fn missile_preprocess(mut ElementPtr: *mut ELEMENT) {
    if (*ElementPtr).c2rust_unnamed_2.turn_wait as ::core::ffi::c_int
        > 0 as ::core::ffi::c_int
    {
        (*ElementPtr).c2rust_unnamed_2.turn_wait = (*ElementPtr)
            .c2rust_unnamed_2
            .turn_wait
            .wrapping_sub(1);
    } else {
        let mut facing: COUNT = 0;
        facing = GetFrameIndex((*ElementPtr).next.image.frame);
        if TrackShip(ElementPtr, &raw mut facing) as ::core::ffi::c_int
            > 0 as ::core::ffi::c_int
        {
            (*ElementPtr).next.image.frame = SetAbsFrameIndex(
                (*ElementPtr).next.image.frame,
                facing,
            );
            (*ElementPtr).state_flags = ((*ElementPtr).state_flags as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as ELEMENT_FLAGS;
            SetVelocityVector(
                &raw mut (*ElementPtr).velocity,
                (20 as SDWORD) << resolutionFactor << 2 as ::core::ffi::c_int,
                facing,
            );
        }
        (*ElementPtr).c2rust_unnamed_2.turn_wait = 5 as BYTE;
    };
}
unsafe extern "C" fn mmrnmhrm_intelligence(
    mut ShipPtr: *mut ELEMENT,
    mut ObjectsOfConcern: *mut EVALUATE_DESC,
    mut ConcernCounter: COUNT,
) {
    let mut CanTransform: BOOLEAN = BOOLEAN::FALSE;
    let mut lpEvalDesc: *mut EVALUATE_DESC = ::core::ptr::null_mut::<EVALUATE_DESC>();
    let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
    let mut EnemyStarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
    StarShipPtr = (*ShipPtr).pParent as *mut STARSHIP;
    CanTransform = BOOLEAN(
        ((*StarShipPtr).special_counter as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            && (*(*StarShipPtr).RaceDescPtr).ship_info.energy_level as ::core::ffi::c_int
                >= (*(*StarShipPtr).RaceDescPtr).characteristics.special_energy_cost
                    as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
    lpEvalDesc = ObjectsOfConcern
        .offset(C2Rust_Unnamed_5::ENEMY_SHIP_INDEX.0 as ::core::ffi::c_int as isize);
    if !(*lpEvalDesc).ObjectPtr.is_null() {
        EnemyStarShipPtr = (*(*lpEvalDesc).ObjectPtr).pParent as *mut STARSHIP;
    }
    uqm_lua_checked!(ship_intelligence(ShipPtr, ObjectsOfConcern, ConcernCounter), ());
    (*StarShipPtr).ship_input_state = ((*StarShipPtr).ship_input_state
        as ::core::ffi::c_int & !((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int))
        as BYTE;
    if CanTransform.0 != 0 && !(*lpEvalDesc).ObjectPtr.is_null()
        && (*StarShipPtr).ship_input_state as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int == 0
    {
        let mut delta_x: SDWORD = 0;
        let mut delta_y: SDWORD = 0;
        let mut travel_angle: COUNT = 0;
        let mut direction_angle: COUNT = 0;
        GetCurrentVelocityComponentsSdword(
            &raw mut (*(*lpEvalDesc).ObjectPtr).velocity,
            &raw mut delta_x,
            &raw mut delta_y,
        );
        if delta_x == 0 as ::core::ffi::c_int && delta_y == 0 as ::core::ffi::c_int {
            travel_angle = 0 as COUNT;
            direction_angle = travel_angle;
        } else {
            delta_x = ((*(*lpEvalDesc).ObjectPtr).current.location.x
                as ::core::ffi::c_int
                - (*ShipPtr).current.location.x as ::core::ffi::c_int) as SDWORD;
            delta_y = ((*(*lpEvalDesc).ObjectPtr).current.location.y
                as ::core::ffi::c_int
                - (*ShipPtr).current.location.y as ::core::ffi::c_int) as SDWORD;
            direction_angle = ARCTAN(-delta_x, -delta_y);
            travel_angle = (*(*lpEvalDesc).ObjectPtr).velocity.TravelAngle;
        }
        if (*ShipPtr).next.image.farray
            == &raw mut (*(*StarShipPtr).RaceDescPtr).ship_data.ship as *mut FRAME
        {
            if (*lpEvalDesc).which_turn as ::core::ffi::c_int > 8 as ::core::ffi::c_int {
                if (*(*EnemyStarShipPtr).RaceDescPtr).cyborg_control.ManeuverabilityIndex
                    as ::core::ffi::c_int
                    <= (25 as ::core::ffi::c_int) << resolutionFactor << resolutionFactor
                    || (direction_angle as ::core::ffi::c_int
                        - travel_angle as ::core::ffi::c_int
                        + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                            >> 2 as ::core::ffi::c_int)
                        & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                            - 1 as ::core::ffi::c_int) as DWORD
                        > ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                            >> 1 as ::core::ffi::c_int) as DWORD
                {
                    (*StarShipPtr).ship_input_state = ((*StarShipPtr).ship_input_state
                        as ::core::ffi::c_int
                        | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as BYTE;
                }
            }
        } else {
            let mut ship_delta_x: SDWORD = 0;
            let mut ship_delta_y: SDWORD = 0;
            GetCurrentVelocityComponentsSdword(
                &raw mut (*ShipPtr).velocity,
                &raw mut ship_delta_x,
                &raw mut ship_delta_y,
            );
            delta_x -= ship_delta_x;
            delta_y -= ship_delta_y;
            travel_angle = ARCTAN(delta_x, delta_y);
            if ((*lpEvalDesc).which_turn as ::core::ffi::c_int)
                < 16 as ::core::ffi::c_int
            {
                if (*lpEvalDesc).which_turn as ::core::ffi::c_int
                    <= 8 as ::core::ffi::c_int
                    || (direction_angle as ::core::ffi::c_int
                        - travel_angle as ::core::ffi::c_int
                        + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                            >> 3 as ::core::ffi::c_int)
                        & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                            - 1 as ::core::ffi::c_int) as DWORD
                        <= ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                            >> 2 as ::core::ffi::c_int) as DWORD
                {
                    (*StarShipPtr).ship_input_state = ((*StarShipPtr).ship_input_state
                        as ::core::ffi::c_int
                        | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as BYTE;
                }
            } else if (*lpEvalDesc).which_turn as ::core::ffi::c_int
                > 32 as ::core::ffi::c_int
                && (direction_angle as ::core::ffi::c_int
                    - travel_angle as ::core::ffi::c_int
                    + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                        >> 2 as ::core::ffi::c_int)
                    & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int) as DWORD
                    > ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                        >> 1 as ::core::ffi::c_int) as DWORD
            {
                (*StarShipPtr).ship_input_state = ((*StarShipPtr).ship_input_state
                    as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as BYTE;
            }
        }
    }
    if (*ShipPtr).current.image.farray
        == &raw mut (*(*StarShipPtr).RaceDescPtr).ship_data.special as *mut FRAME
    {
        if (*StarShipPtr).ship_input_state as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int == 0
            && !(*lpEvalDesc).ObjectPtr.is_null()
        {
            (*StarShipPtr).ship_input_state = ((*StarShipPtr).ship_input_state
                as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as BYTE;
        } else {
            (*StarShipPtr).ship_input_state = ((*StarShipPtr).ship_input_state
                as ::core::ffi::c_int
                & !((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)) as BYTE;
        }
    }
}
unsafe extern "C" fn twin_laser_collision(
    mut ElementPtr0: *mut ELEMENT,
    mut pPt0: *mut POINT,
    mut ElementPtr1: *mut ELEMENT,
    mut pPt1: *mut POINT,
) {
    if (*ElementPtr1).state_flags as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int == 0
        || elementsOfSamePlayer(ElementPtr0, ElementPtr1).0 == 0
    {
        uqm_lua_checked!(weapon_collision(ElementPtr0, pPt0, ElementPtr1, pPt1), ());
    }
}
unsafe extern "C" fn initialize_dual_weapons(
    mut ShipPtr: *mut ELEMENT,
    mut WeaponArray: *mut HELEMENT,
) -> COUNT {
    let mut cx: COORD = 0;
    let mut cy: COORD = 0;
    let mut facing: COUNT = 0;
    let mut angle: COUNT = 0;
    let mut offs_x: SDWORD = 0;
    let mut offs_y: SDWORD = 0;
    let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
    StarShipPtr = (*ShipPtr).pParent as *mut STARSHIP;
    facing = (*StarShipPtr).ShipFacing;
    angle = ((facing as ::core::ffi::c_int)
        << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int) as COUNT;
    cx = ((*ShipPtr).next.location.x as SDWORD
        + (*(&raw mut sinetab as *mut SDWORD)
            .offset(
                (angle as ::core::ffi::c_int
                    + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                        >> 2 as ::core::ffi::c_int)
                    & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int) as DWORD as isize,
            ) as ::core::ffi::c_long
            * ((4 as ::core::ffi::c_int) << resolutionFactor << 2 as ::core::ffi::c_int)
                as ::core::ffi::c_long >> 14 as ::core::ffi::c_int) as SDWORD) as COORD;
    cy = ((*ShipPtr).next.location.y as SDWORD
        + (*(&raw mut sinetab as *mut SDWORD)
            .offset(
                (angle as ::core::ffi::c_int
                    & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int) as DWORD as isize,
            ) as ::core::ffi::c_long
            * ((4 as ::core::ffi::c_int) << resolutionFactor << 2 as ::core::ffi::c_int)
                as ::core::ffi::c_long >> 14 as ::core::ffi::c_int) as SDWORD) as COORD;
    if (*ShipPtr).next.image.farray
        == &raw mut (*(*StarShipPtr).RaceDescPtr).ship_data.ship as *mut FRAME
    {
        let mut ex: COORD = 0;
        let mut ey: COORD = 0;
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
        let mut LaserPtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
        LaserBlock.sender = (*ShipPtr).playerNr;
        LaserBlock.flags = 0 as ELEMENT_FLAGS;
        LaserBlock.pixoffs = 0 as SIZE;
        LaserBlock.color = colorFromRgb15(0x1f as BYTE, 0xa as BYTE, 0xa as BYTE);
        LaserBlock.face = facing;
        ex = (cx as SDWORD
            + (*(&raw mut sinetab as *mut SDWORD)
                .offset(
                    (angle as ::core::ffi::c_int
                        + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                            >> 2 as ::core::ffi::c_int)
                        & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                            - 1 as ::core::ffi::c_int) as DWORD as isize,
                ) as ::core::ffi::c_long
                * (((125 as ::core::ffi::c_int) << resolutionFactor)
                    + ((16 as ::core::ffi::c_int) << resolutionFactor)
                    << 2 as ::core::ffi::c_int) as ::core::ffi::c_long
                >> 14 as ::core::ffi::c_int) as SDWORD) as COORD;
        ey = (cy as SDWORD
            + (*(&raw mut sinetab as *mut SDWORD)
                .offset(
                    (angle as ::core::ffi::c_int
                        & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                            - 1 as ::core::ffi::c_int) as DWORD as isize,
                ) as ::core::ffi::c_long
                * (((125 as ::core::ffi::c_int) << resolutionFactor)
                    + ((16 as ::core::ffi::c_int) << resolutionFactor)
                    << 2 as ::core::ffi::c_int) as ::core::ffi::c_long
                >> 14 as ::core::ffi::c_int) as SDWORD) as COORD;
        offs_x = -((*(&raw mut sinetab as *mut SDWORD)
            .offset(
                (angle as ::core::ffi::c_int
                    & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int) as DWORD as isize,
            ) as ::core::ffi::c_long
            * (if if resolutionFactor != 2 as ::core::ffi::c_uint {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } == 0
            {
                10 as ::core::ffi::c_int
            } else {
                48 as ::core::ffi::c_int
            } << 2 as ::core::ffi::c_int) as ::core::ffi::c_long
            >> 14 as ::core::ffi::c_int) as SDWORD);
        offs_y = (*(&raw mut sinetab as *mut SDWORD)
            .offset(
                (angle as ::core::ffi::c_int
                    + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                        >> 2 as ::core::ffi::c_int)
                    & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int) as DWORD as isize,
            ) as ::core::ffi::c_long
            * (if if resolutionFactor != 2 as ::core::ffi::c_uint {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } == 0
            {
                10 as ::core::ffi::c_int
            } else {
                48 as ::core::ffi::c_int
            } << 2 as ::core::ffi::c_int) as ::core::ffi::c_long
            >> 14 as ::core::ffi::c_int) as SDWORD;
        LaserBlock.cx = (cx as SDWORD + offs_x) as COORD;
        LaserBlock.cy = (cy as SDWORD + offs_y) as COORD;
        LaserBlock.ex = (ex as ::core::ffi::c_int - LaserBlock.cx as ::core::ffi::c_int)
            as COORD;
        LaserBlock.ey = (ey as ::core::ffi::c_int - LaserBlock.cy as ::core::ffi::c_int)
            as COORD;
        *WeaponArray.offset(0isize) = initialize_laser(&raw mut LaserBlock);
        if !(*WeaponArray.offset(0isize)).is_null() {
            LaserPtr = LockLink(&raw mut disp_q, *WeaponArray.offset(0isize))
                as *mut ELEMENT;
            (*LaserPtr).collision_func = Some(
                twin_laser_collision
                    as unsafe extern "C" fn(
                        *mut ELEMENT,
                        *mut POINT,
                        *mut ELEMENT,
                        *mut POINT,
                    ) -> (),
            ) as Option<ElementCollisionFunc>;
            UnlockLink(&raw mut disp_q, *WeaponArray.offset(0isize));
        }
        LaserBlock.cx = (cx as SDWORD - offs_x) as COORD;
        LaserBlock.cy = (cy as SDWORD - offs_y) as COORD;
        LaserBlock.ex = (ex as ::core::ffi::c_int - LaserBlock.cx as ::core::ffi::c_int)
            as COORD;
        LaserBlock.ey = (ey as ::core::ffi::c_int - LaserBlock.cy as ::core::ffi::c_int)
            as COORD;
        *WeaponArray.offset(1isize) = initialize_laser(&raw mut LaserBlock);
        if !(*WeaponArray.offset(1isize)).is_null() {
            LaserPtr = LockLink(&raw mut disp_q, *WeaponArray.offset(1isize))
                as *mut ELEMENT;
            (*LaserPtr).collision_func = Some(
                twin_laser_collision
                    as unsafe extern "C" fn(
                        *mut ELEMENT,
                        *mut POINT,
                        *mut ELEMENT,
                        *mut POINT,
                    ) -> (),
            ) as Option<ElementCollisionFunc>;
            UnlockLink(&raw mut disp_q, *WeaponArray.offset(1isize));
        }
    } else {
        let mut TorpBlock: MISSILE_BLOCK = MISSILE_BLOCK {
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
        let mut TorpPtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
        TorpBlock.farray = &raw mut (*(*StarShipPtr).RaceDescPtr).ship_data.weapon
            as *mut FRAME;
        TorpBlock.sender = (*ShipPtr).playerNr;
        TorpBlock.flags = ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
            as ELEMENT_FLAGS;
        TorpBlock.pixoffs = 0 as SIZE;
        TorpBlock.speed = ((20 as ::core::ffi::c_int) << resolutionFactor
            << 2 as ::core::ffi::c_int) as SIZE;
        TorpBlock.hit_points = 1 as SIZE;
        TorpBlock.damage = 1 as SIZE;
        TorpBlock.life = 40 as COUNT;
        TorpBlock.preprocess_func = Some(
            missile_preprocess as unsafe extern "C" fn(*mut ELEMENT) -> (),
        ) as Option<unsafe extern "C" fn(*mut ELEMENT) -> ()>;
        TorpBlock.blast_offs = 0 as SIZE;
        TorpBlock.index = (facing as ::core::ffi::c_int - 1 as ::core::ffi::c_int
            & ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                - 1 as ::core::ffi::c_int) as DWORD as COUNT;
        TorpBlock.face = TorpBlock.index;
        offs_x = -((*(&raw mut sinetab as *mut SDWORD)
            .offset(
                ((TorpBlock.face as ::core::ffi::c_int)
                    << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                    & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int) as DWORD as isize,
            ) as ::core::ffi::c_long
            * ((4 as ::core::ffi::c_int) << resolutionFactor << 2 as ::core::ffi::c_int)
                as ::core::ffi::c_long >> 14 as ::core::ffi::c_int) as SDWORD);
        offs_y = (*(&raw mut sinetab as *mut SDWORD)
            .offset(
                (((TorpBlock.face as ::core::ffi::c_int)
                    << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int)
                    + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                        >> 2 as ::core::ffi::c_int)
                    & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int) as DWORD as isize,
            ) as ::core::ffi::c_long
            * ((4 as ::core::ffi::c_int) << resolutionFactor << 2 as ::core::ffi::c_int)
                as ::core::ffi::c_long >> 14 as ::core::ffi::c_int) as SDWORD;
        TorpBlock.cx = (cx as SDWORD + offs_x) as COORD;
        TorpBlock.cy = (cy as SDWORD + offs_y) as COORD;
        *WeaponArray.offset(0isize) = initialize_missile(&raw mut TorpBlock);
        if !(*WeaponArray.offset(0isize)).is_null() {
            TorpPtr = LockLink(&raw mut disp_q, *WeaponArray.offset(0isize))
                as *mut ELEMENT;
            (*TorpPtr).c2rust_unnamed_2.turn_wait = 5 as BYTE;
            UnlockLink(&raw mut disp_q, *WeaponArray.offset(0isize));
        }
        TorpBlock.index = (facing as ::core::ffi::c_int + 1 as ::core::ffi::c_int
            & ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                - 1 as ::core::ffi::c_int) as DWORD as COUNT;
        TorpBlock.face = TorpBlock.index;
        TorpBlock.cx = (cx as SDWORD - offs_x) as COORD;
        TorpBlock.cy = (cy as SDWORD - offs_y) as COORD;
        *WeaponArray.offset(1isize) = initialize_missile(&raw mut TorpBlock);
        if !(*WeaponArray.offset(1isize)).is_null() {
            TorpPtr = LockLink(&raw mut disp_q, *WeaponArray.offset(1isize))
                as *mut ELEMENT;
            (*TorpPtr).c2rust_unnamed_2.turn_wait = 5 as BYTE;
            UnlockLink(&raw mut disp_q, *WeaponArray.offset(1isize));
        }
    }
    return 2 as COUNT;
}
unsafe extern "C" fn mmrnmhrm_postprocess(mut ElementPtr: *mut ELEMENT) {
    let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
    StarShipPtr = (*ElementPtr).pParent as *mut STARSHIP;
    if (*ElementPtr).next.image.farray != (*ElementPtr).current.image.farray {
        let mut tempShipData: MMRNMHRM_DATA = CHARACTERISTIC_STUFF {
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
        };
        let mut otherwing_desc: *mut MMRNMHRM_DATA = ::core::ptr::null_mut::<
            MMRNMHRM_DATA,
        >();
        ProcessSound(
            SetAbsStringTableIndex(
                (*(*StarShipPtr).RaceDescPtr).ship_data.ship_sounds,
                1 as COUNT,
            ),
            ElementPtr,
        );
        (*StarShipPtr).weapon_counter = 0 as BYTE;
        otherwing_desc = GetCustomShipData((*StarShipPtr).RaceDescPtr)
            as *mut MMRNMHRM_DATA;
        if otherwing_desc.is_null() {
            return;
        }
        tempShipData = *otherwing_desc;
        SetCustomShipData(
            (*StarShipPtr).RaceDescPtr,
            &raw mut (*(*StarShipPtr).RaceDescPtr).characteristics,
        );
        (*(*StarShipPtr).RaceDescPtr).characteristics = tempShipData
            as CHARACTERISTIC_STUFF;
        (*(*StarShipPtr).RaceDescPtr).cyborg_control.ManeuverabilityIndex = 0 as COUNT;
        if (*ElementPtr).next.image.farray
            == &raw mut (*(*StarShipPtr).RaceDescPtr).ship_data.special as *mut FRAME
        {
            (*(*StarShipPtr).RaceDescPtr).cyborg_control.WeaponRange = ((((1000
                as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                - 1 as ::core::ffi::c_int) << resolutionFactor) as COUNT;
            (*(*StarShipPtr).RaceDescPtr).ship_info.ship_flags = ((*(*StarShipPtr)
                .RaceDescPtr)
                .ship_info
                .ship_flags as ::core::ffi::c_int
                & !((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int)) as UWORD;
            (*(*StarShipPtr).RaceDescPtr).ship_info.ship_flags = ((*(*StarShipPtr)
                .RaceDescPtr)
                .ship_info
                .ship_flags as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as UWORD;
            (*(*StarShipPtr).RaceDescPtr).ship_data.ship_sounds = SetAbsStringTableIndex(
                (*(*StarShipPtr).RaceDescPtr).ship_data.ship_sounds,
                2 as COUNT,
            ) as SOUND;
            (*StarShipPtr).cur_status_flags = ((*StarShipPtr).cur_status_flags
                as ::core::ffi::c_int
                & !((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int))
                as STATUS_FLAGS;
        } else {
            (*(*StarShipPtr).RaceDescPtr).cyborg_control.WeaponRange = (((50
                as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) << resolutionFactor)
                as COUNT;
            (*(*StarShipPtr).RaceDescPtr).ship_info.ship_flags = ((*(*StarShipPtr)
                .RaceDescPtr)
                .ship_info
                .ship_flags as ::core::ffi::c_int
                & !((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)) as UWORD;
            (*(*StarShipPtr).RaceDescPtr).ship_info.ship_flags = ((*(*StarShipPtr)
                .RaceDescPtr)
                .ship_info
                .ship_flags as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as UWORD;
            (*(*StarShipPtr).RaceDescPtr).ship_data.ship_sounds = SetAbsStringTableIndex(
                (*(*StarShipPtr).RaceDescPtr).ship_data.ship_sounds,
                0 as COUNT,
            ) as SOUND;
            if (*StarShipPtr).cur_status_flags as ::core::ffi::c_int
                & ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) != 0
            {
                (*StarShipPtr).cur_status_flags = ((*StarShipPtr).cur_status_flags
                    as ::core::ffi::c_int
                    | ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int
                        | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int))
                    as STATUS_FLAGS;
            }
        }
    }
}
unsafe extern "C" fn mmrnmhrm_preprocess(mut ElementPtr: *mut ELEMENT) {
    let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
    StarShipPtr = (*ElementPtr).pParent as *mut STARSHIP;
    if (*ElementPtr).state_flags as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int == 0
    {
        if (*StarShipPtr).cur_status_flags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int != 0
            && (*StarShipPtr).special_counter as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
        {
            if uqm_lua_checked!(DeltaEnergy(
                    ElementPtr,
                    -((*(*StarShipPtr).RaceDescPtr).characteristics.special_energy_cost
                        as ::core::ffi::c_int) as SIZE,
                ), ())
                .0 != 0
            {
                if (*ElementPtr).next.image.farray
                    == &raw mut (*(*StarShipPtr).RaceDescPtr).ship_data.ship
                        as *mut FRAME
                {
                    (*ElementPtr).next.image.farray = &raw mut (*(*StarShipPtr)
                        .RaceDescPtr)
                        .ship_data
                        .special as *mut FRAME;
                } else {
                    (*ElementPtr).next.image.farray = &raw mut (*(*StarShipPtr)
                        .RaceDescPtr)
                        .ship_data
                        .ship as *mut FRAME;
                }
                (*ElementPtr).next.image.frame = SetEquFrameIndex(
                    *(*ElementPtr).next.image.farray.offset(0isize),
                    (*ElementPtr).next.image.frame,
                );
                (*ElementPtr).state_flags = ((*ElementPtr).state_flags
                    as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int)
                    as ELEMENT_FLAGS;
                (*StarShipPtr).special_counter = (*(*StarShipPtr).RaceDescPtr)
                    .characteristics
                    .special_wait;
            }
        }
    }
}
unsafe extern "C" fn uninit_mmrnmhrm(mut pRaceDesc: *mut RACE_DESC) {
    SetCustomShipData(pRaceDesc, ::core::ptr::null::<CustomShipData_t>());
}
#[no_mangle]
pub unsafe extern "C" fn init_mmrnmhrm() -> *mut RACE_DESC {
    let mut RaceDescPtr: *mut RACE_DESC = ::core::ptr::null_mut::<RACE_DESC>();
    static mut new_mmrnmhrm_desc: RACE_DESC = RACE_DESC {
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
    let mut otherwing_desc: MMRNMHRM_DATA = CHARACTERISTIC_STUFF {
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
    };
    if if resolutionFactor != 2 as ::core::ffi::c_uint {
        BOOLEAN::FALSE.0 as ::core::ffi::c_int
    } else {
        BOOLEAN::TRUE.0 as ::core::ffi::c_int
    } != 0
    {
        mmrnmhrm_desc.characteristics.max_thrust = ((20 as ::core::ffi::c_int)
            << resolutionFactor) as COUNT;
        mmrnmhrm_desc.characteristics.thrust_increment = ((5 as ::core::ffi::c_int)
            << resolutionFactor) as COUNT;
        mmrnmhrm_desc.cyborg_control.WeaponRange = ((200 as ::core::ffi::c_int)
            << 2 as ::core::ffi::c_int) as COUNT;
    } else {
        mmrnmhrm_desc.characteristics.max_thrust = 20 as COUNT;
        mmrnmhrm_desc.characteristics.thrust_increment = 5 as COUNT;
        mmrnmhrm_desc.cyborg_control.WeaponRange = ((50 as ::core::ffi::c_int)
            << 2 as ::core::ffi::c_int) as COUNT;
    }
    mmrnmhrm_desc.uninit_func = Some(
        uninit_mmrnmhrm as unsafe extern "C" fn(*mut RACE_DESC) -> (),
    ) as Option<UNINIT_FUNC>;
    mmrnmhrm_desc.preprocess_func = Some(
        mmrnmhrm_preprocess as unsafe extern "C" fn(*mut ELEMENT) -> (),
    ) as Option<PREPROCESS_FUNC>;
    mmrnmhrm_desc.postprocess_func = Some(
        mmrnmhrm_postprocess as unsafe extern "C" fn(*mut ELEMENT) -> (),
    ) as Option<POSTPROCESS_FUNC>;
    mmrnmhrm_desc.init_weapon_func = Some(
        initialize_dual_weapons
            as unsafe extern "C" fn(*mut ELEMENT, *mut HELEMENT) -> COUNT,
    ) as Option<INIT_WEAPON_FUNC>;
    mmrnmhrm_desc.cyborg_control.intelligence_func = Some(
        mmrnmhrm_intelligence
            as unsafe extern "C" fn(*mut ELEMENT, *mut EVALUATE_DESC, COUNT) -> (),
    ) as Option<IntelligenceFunc>;
    new_mmrnmhrm_desc = mmrnmhrm_desc;
    otherwing_desc.max_thrust = ((50 as ::core::ffi::c_int) << resolutionFactor)
        as COUNT;
    otherwing_desc.thrust_increment = ((10 as ::core::ffi::c_int) << resolutionFactor)
        as COUNT;
    otherwing_desc.energy_regeneration = 1 as BYTE;
    otherwing_desc.weapon_energy_cost = 1 as BYTE;
    otherwing_desc.special_energy_cost = 10 as BYTE;
    otherwing_desc.energy_wait = 6 as BYTE;
    otherwing_desc.turn_wait = 14 as BYTE;
    otherwing_desc.thrust_wait = 0 as BYTE;
    otherwing_desc.weapon_wait = 20 as BYTE;
    otherwing_desc.special_wait = 0 as BYTE;
    otherwing_desc.ship_mass = 3 as BYTE;
    SetCustomShipData(&raw mut new_mmrnmhrm_desc, &raw mut otherwing_desc);
    RaceDescPtr = &raw mut new_mmrnmhrm_desc;
    return RaceDescPtr;
}
