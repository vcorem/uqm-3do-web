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

// Rust translation of src/uqm/ships/androsyn/androsyn.c.
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
    fn HMalloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn HFree(p: *mut ::core::ffi::c_void);
    fn SetEquFrameIndex(DstFrame: FRAME, SrcFrame: FRAME) -> FRAME;
    fn IncFrameIndex(Frame: FRAME) -> FRAME;
    fn SetAbsStringTableIndex(String: STRING, StringTableIndex: COUNT) -> STRING;
    fn ARCTAN(delta_x: SIZE, delta_y: SIZE) -> COUNT;
    fn memset(
        _: *mut ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn GetCurrentVelocityComponents(
        velocityptr: *mut VELOCITY_DESC,
        pdx: *mut SIZE,
        pdy: *mut SIZE,
    );
    fn SetVelocityVector(
        velocityptr: *mut VELOCITY_DESC,
        magnitude: SIZE,
        facing: COUNT,
    );
    static mut disp_q: QUEUE;
    fn ship_intelligence(
        ShipPtr: *mut ELEMENT,
        ObjectsOfConcern: *mut EVALUATE_DESC,
        ConcernCounter: COUNT,
    );
    fn DeltaEnergy(ElementPtr: *mut ELEMENT, energy_delta: SIZE) -> BOOLEAN;
    fn ProcessSound(Sound: SOUND, PositionalObject: *mut ELEMENT);
    fn initialize_missile(pMissileBlock: *mut MISSILE_BLOCK) -> HELEMENT;
    fn weapon_collision(
        ElementPtr0: *mut ELEMENT,
        pPt0: *mut POINT,
        ElementPtr1: *mut ELEMENT,
        pPt1: *mut POINT,
    ) -> HELEMENT;
    fn TrackShip(Tracker: *mut ELEMENT, pfacing: *mut COUNT) -> SIZE;
    fn collision(
        ElementPtr0: *mut ELEMENT,
        pPt0: *mut POINT,
        ElementPtr1: *mut ELEMENT,
        pPt1: *mut POINT,
    );
    fn TFB_Random() -> DWORD;
}
pub type size_t = usize;
pub type uint8 = ::core::ffi::c_uchar;
pub type uint16 = ::core::ffi::c_ushort;
pub type sint16 = ::core::ffi::c_short;
pub type uint32 = ::core::ffi::c_uint;
pub type BYTE = uint8;
pub type UWORD = uint16;
pub type SWORD = sint16;
pub type DWORD = uint32;
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
pub struct ANDROSYNTH_DATA {
    pub collision_func: Option<ElementCollisionFunc>,
}
pub type CustomShipData_t = ANDROSYNTH_DATA;
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
static mut androsynth_desc: RACE_DESC = unsafe {
    race_desc {
        c2rust_padding: [0; 12],
        c2rust_padding_0: [0; 10],
        c2rust_padding_1: [0; 2],
        ship_info: SHIP_INFO {
            ship_flags: ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as UWORD,
            ship_cost: 15 as BYTE,
            crew_level: 20 as COUNT,
            max_crew: 20 as COUNT,
            energy_level: 24 as BYTE,
            max_energy: 24 as BYTE,
            race_strings_rsc: b"ship.androsynth.text\0".as_ptr() as RESOURCE,
            icons_rsc: b"ship.androsynth.icons\0".as_ptr() as RESOURCE,
            melee_icon_rsc: b"ship.androsynth.meleeicons\0".as_ptr() as RESOURCE,
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
            max_thrust: 24 as COUNT,
            thrust_increment: 3 as COUNT,
            energy_regeneration: 1 as BYTE,
            weapon_energy_cost: 3 as BYTE,
            special_energy_cost: 2 as BYTE,
            energy_wait: 8 as BYTE,
            turn_wait: 4 as BYTE,
            thrust_wait: 0 as BYTE,
            weapon_wait: 0 as BYTE,
            special_wait: 0 as BYTE,
            ship_mass: 6 as BYTE,
        },
        ship_data: DATA_STUFF {
            ship_rsc: [
                b"ship.androsynth.graphics.guardian.large\0".as_ptr() as RESOURCE,
                b"ship.androsynth.graphics.guardian.medium\0".as_ptr() as RESOURCE,
                b"ship.androsynth.graphics.guardian.small\0".as_ptr() as RESOURCE,
            ],
            weapon_rsc: [
                b"ship.androsynth.graphics.bubble.large\0".as_ptr() as RESOURCE,
                b"ship.androsynth.graphics.bubble.medium\0".as_ptr() as RESOURCE,
                b"ship.androsynth.graphics.bubble.small\0".as_ptr() as RESOURCE,
            ],
            special_rsc: [
                b"ship.androsynth.graphics.blazer.large\0".as_ptr() as RESOURCE,
                b"ship.androsynth.graphics.blazer.medium\0".as_ptr() as RESOURCE,
                b"ship.androsynth.graphics.blazer.small\0".as_ptr() as RESOURCE,
            ],
            captain_control: captain_stuff {
                captain_rsc: b"ship.androsynth.graphics.captain\0".as_ptr() as RESOURCE,
                background: ::core::ptr::null_mut::<FRAME_DESC>(),
                turn: ::core::ptr::null_mut::<FRAME_DESC>(),
                thrust: ::core::ptr::null_mut::<FRAME_DESC>(),
                weapon: ::core::ptr::null_mut::<FRAME_DESC>(),
                special: ::core::ptr::null_mut::<FRAME_DESC>(),
            },
            victory_ditty_rsc: b"ship.androsynth.ditty\0".as_ptr() as RESOURCE,
            ship_sounds_rsc: b"ship.androsynth.sounds\0".as_ptr() as RESOURCE,
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
            WeaponRange: ((1000 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
                >> 2 as ::core::ffi::c_int) as COUNT,
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
unsafe extern "C" fn blazer_collision(
    mut ElementPtr0: *mut ELEMENT,
    mut pPt0: *mut POINT,
    mut ElementPtr1: *mut ELEMENT,
    mut pPt1: *mut POINT,
) {
    let mut old_offs: BYTE = 0;
    let mut old_crew_level: COUNT = 0;
    let mut old_life: COUNT = 0;
    old_crew_level = (*ElementPtr0).c2rust_unnamed_0.crew_level;
    old_life = (*ElementPtr0).c2rust_unnamed.life_span;
    old_offs = (*ElementPtr0).c2rust_unnamed_3.blast_offset;
    (*ElementPtr0).c2rust_unnamed_3.blast_offset = 10 as BYTE;
    (*ElementPtr0).c2rust_unnamed_1.mass_points = 3 as BYTE;
    weapon_collision(ElementPtr0, pPt0, ElementPtr1, pPt1);
    (*ElementPtr0).c2rust_unnamed_1.mass_points = 1 as BYTE;
    (*ElementPtr0).c2rust_unnamed_3.blast_offset = old_offs;
    (*ElementPtr0).c2rust_unnamed.life_span = old_life;
    (*ElementPtr0).c2rust_unnamed_0.crew_level = old_crew_level;
    (*ElementPtr0).state_flags = ((*ElementPtr0).state_flags as ::core::ffi::c_int
        & !((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)) as ELEMENT_FLAGS;
    collision(ElementPtr0, pPt0, ElementPtr1, pPt1);
}
unsafe extern "C" fn bubble_preprocess(mut ElementPtr: *mut ELEMENT) {
    let mut thrust_wait: BYTE = 0;
    let mut turn_wait: BYTE = 0;
    thrust_wait = ((*ElementPtr).c2rust_unnamed_2.turn_wait as ::core::ffi::c_int
        >> 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE;
    turn_wait = ((*ElementPtr).c2rust_unnamed_2.turn_wait as ::core::ffi::c_int
        & 0xf as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE;
    if thrust_wait as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
        thrust_wait = thrust_wait.wrapping_sub(1);
    } else {
        (*ElementPtr).next.image.frame = IncFrameIndex(
            (*ElementPtr).current.image.frame,
        );
        (*ElementPtr).state_flags = ((*ElementPtr).state_flags as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as ELEMENT_FLAGS;
        thrust_wait = (TFB_Random() as COUNT as ::core::ffi::c_int
            & 3 as ::core::ffi::c_int) as BYTE;
    }
    if turn_wait as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
        turn_wait = turn_wait.wrapping_sub(1);
    } else {
        let mut facing: COUNT = 0;
        let mut delta_facing: SIZE = 0;
        facing = ((*ElementPtr).velocity.TravelAngle as ::core::ffi::c_int
            + ((1 as ::core::ffi::c_int)
                << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int)
            >> 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
            & ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                - 1 as ::core::ffi::c_int) as COUNT;
        delta_facing = TrackShip(ElementPtr, &raw mut facing);
        if delta_facing as ::core::ffi::c_int == -1 as ::core::ffi::c_int {
            facing = TFB_Random() as COUNT;
        } else if delta_facing as ::core::ffi::c_int
            <= ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int)
                + ((1 as ::core::ffi::c_int)
                    << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int)
                >> 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
        {
            facing = (facing as ::core::ffi::c_int
                + (TFB_Random() as COUNT as ::core::ffi::c_int
                    & (((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                        >> 1 as ::core::ffi::c_int)
                        + ((1 as ::core::ffi::c_int)
                            << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int)
                        >> 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int)) as COUNT;
        } else {
            facing = (facing as ::core::ffi::c_int
                - (TFB_Random() as COUNT as ::core::ffi::c_int
                    & (((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                        >> 1 as ::core::ffi::c_int)
                        + ((1 as ::core::ffi::c_int)
                            << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int)
                        >> 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int)) as COUNT;
        }
        SetVelocityVector(
            &raw mut (*ElementPtr).velocity,
            ((8 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as SIZE,
            facing,
        );
        turn_wait = 2 as BYTE;
    }
    (*ElementPtr).c2rust_unnamed_2.turn_wait = ((thrust_wait as ::core::ffi::c_int)
        << 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int
        | turn_wait as ::core::ffi::c_int) as BYTE;
}
unsafe extern "C" fn initialize_bubble(
    mut ShipPtr: *mut ELEMENT,
    mut BubbleArray: *mut HELEMENT,
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
    MissileBlock.pixoffs = 14 as SIZE;
    MissileBlock.speed = ((8 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as SIZE;
    MissileBlock.hit_points = 3 as SIZE;
    MissileBlock.damage = 2 as SIZE;
    MissileBlock.life = 200 as COUNT;
    MissileBlock.preprocess_func = Some(
        bubble_preprocess as unsafe extern "C" fn(*mut ELEMENT) -> (),
    ) as Option<unsafe extern "C" fn(*mut ELEMENT) -> ()>;
    MissileBlock.blast_offs = 3 as SIZE;
    *BubbleArray.offset(0isize) = initialize_missile(&raw mut MissileBlock);
    if !(*BubbleArray.offset(0isize)).is_null() {
        let mut BubblePtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
        BubblePtr = LockLink(&raw mut disp_q, *BubbleArray.offset(0isize))
            as *mut ELEMENT;
        (*BubblePtr).c2rust_unnamed_2.turn_wait = 0 as BYTE;
        UnlockLink(&raw mut disp_q, *BubbleArray.offset(0isize));
    }
    return 1 as COUNT;
}
unsafe extern "C" fn androsynth_intelligence(
    mut ShipPtr: *mut ELEMENT,
    mut ObjectsOfConcern: *mut EVALUATE_DESC,
    mut ConcernCounter: COUNT,
) {
    let mut lpEvalDesc: *mut EVALUATE_DESC = ::core::ptr::null_mut::<EVALUATE_DESC>();
    let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
    StarShipPtr = (*ShipPtr).pParent as *mut STARSHIP;
    lpEvalDesc = ObjectsOfConcern
        .offset(C2Rust_Unnamed_5::ENEMY_WEAPON_INDEX.0 as ::core::ffi::c_int as isize);
    if (*ShipPtr).next.image.farray
        == &raw mut (*(*StarShipPtr).RaceDescPtr).ship_data.special as *mut FRAME
    {
        (*ObjectsOfConcern
            .offset(
                C2Rust_Unnamed_5::CREW_OBJECT_INDEX.0 as ::core::ffi::c_int as isize,
            ))
            .ObjectPtr = ::core::ptr::null_mut::<ELEMENT>();
        if !(*lpEvalDesc).ObjectPtr.is_null()
            && (*lpEvalDesc).MoveState.0 == MOVEMENT_STATE::ENTICE.0
        {
            if (*(*lpEvalDesc).ObjectPtr).state_flags as ::core::ffi::c_int
                & (1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int != 0
                && (*(*lpEvalDesc).ObjectPtr).state_flags as ::core::ffi::c_int
                    & (1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int == 0
            {
                (*lpEvalDesc).MoveState = MOVEMENT_STATE::AVOID;
            } else {
                (*lpEvalDesc).ObjectPtr = ::core::ptr::null_mut::<ELEMENT>();
            }
        }
        ship_intelligence(ShipPtr, ObjectsOfConcern, ConcernCounter);
    } else {
        let mut pEnemyStarShip: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
        lpEvalDesc = ObjectsOfConcern
            .offset(C2Rust_Unnamed_5::ENEMY_SHIP_INDEX.0 as ::core::ffi::c_int as isize);
        if !(*lpEvalDesc).ObjectPtr.is_null() {
            pEnemyStarShip = (*(*lpEvalDesc).ObjectPtr).pParent as *mut STARSHIP;
            if (*lpEvalDesc).which_turn as ::core::ffi::c_int <= 16 as ::core::ffi::c_int
                && ((*StarShipPtr).special_counter as ::core::ffi::c_int
                    > 0 as ::core::ffi::c_int
                    || ((*(*StarShipPtr).RaceDescPtr).ship_info.energy_level
                        as ::core::ffi::c_int)
                        < 24 as ::core::ffi::c_int / 3 as ::core::ffi::c_int
                    || ((*(*pEnemyStarShip).RaceDescPtr).cyborg_control.WeaponRange
                        as ::core::ffi::c_int
                        <= (50 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
                        && (*(*lpEvalDesc).ObjectPtr).c2rust_unnamed_0.crew_level
                            as ::core::ffi::c_int > 3 as ::core::ffi::c_int
                        || (*(*lpEvalDesc).ObjectPtr).c2rust_unnamed_0.crew_level
                            as ::core::ffi::c_int
                            > 3 as ::core::ffi::c_int * 3 as ::core::ffi::c_int
                            && (*(*pEnemyStarShip).RaceDescPtr)
                                .cyborg_control
                                .ManeuverabilityIndex as ::core::ffi::c_int
                                > 25 as ::core::ffi::c_int))
            {
                (*lpEvalDesc).MoveState = MOVEMENT_STATE::ENTICE;
            }
        }
        ship_intelligence(ShipPtr, ObjectsOfConcern, ConcernCounter);
        if (*StarShipPtr).special_counter as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
            (*StarShipPtr).ship_input_state = ((*StarShipPtr).ship_input_state
                as ::core::ffi::c_int
                & !((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)) as BYTE;
            if !(*ObjectsOfConcern
                .offset(
                    C2Rust_Unnamed_5::ENEMY_WEAPON_INDEX.0 as ::core::ffi::c_int as isize,
                ))
                .ObjectPtr
                .is_null()
                && (*ObjectsOfConcern
                    .offset(
                        C2Rust_Unnamed_5::ENEMY_WEAPON_INDEX.0 as ::core::ffi::c_int
                            as isize,
                    ))
                    .which_turn as ::core::ffi::c_int <= 4 as ::core::ffi::c_int
                || !(*lpEvalDesc).ObjectPtr.is_null()
                    && (*(*StarShipPtr).RaceDescPtr).ship_info.energy_level
                        as ::core::ffi::c_int
                        >= 24 as ::core::ffi::c_int / 3 as ::core::ffi::c_int
                    && ((*(*pEnemyStarShip).RaceDescPtr).cyborg_control.WeaponRange
                        as ::core::ffi::c_int
                        >= ((*(*StarShipPtr).RaceDescPtr).cyborg_control.WeaponRange
                            as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                        || ((*lpEvalDesc).which_turn as ::core::ffi::c_int)
                            < 16 as ::core::ffi::c_int
                            && ((*(*pEnemyStarShip).RaceDescPtr)
                                .cyborg_control
                                .WeaponRange as ::core::ffi::c_int
                                > (50 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
                                || (*(*lpEvalDesc).ObjectPtr).c2rust_unnamed_0.crew_level
                                    as ::core::ffi::c_int <= 3 as ::core::ffi::c_int)
                            && ((*(*lpEvalDesc).ObjectPtr).c2rust_unnamed_0.crew_level
                                as ::core::ffi::c_int
                                <= 3 as ::core::ffi::c_int * 3 as ::core::ffi::c_int
                                || (*(*pEnemyStarShip).RaceDescPtr)
                                    .cyborg_control
                                    .ManeuverabilityIndex as ::core::ffi::c_int
                                    <= 25 as ::core::ffi::c_int))
            {
                (*StarShipPtr).ship_input_state = ((*StarShipPtr).ship_input_state
                    as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as BYTE;
            }
        }
        if (*StarShipPtr).ship_input_state as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int == 0
            && (*StarShipPtr).weapon_counter as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int && !(*lpEvalDesc).ObjectPtr.is_null()
        {
            if (*lpEvalDesc).which_turn as ::core::ffi::c_int <= 4 as ::core::ffi::c_int
            {
                (*StarShipPtr).ship_input_state = ((*StarShipPtr).ship_input_state
                    as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as BYTE;
            } else if (*lpEvalDesc).MoveState.0 != MOVEMENT_STATE::PURSUE.0
                && (*lpEvalDesc).which_turn as ::core::ffi::c_int
                    <= 12 as ::core::ffi::c_int
            {
                let mut travel_facing: COUNT = 0;
                let mut direction_facing: COUNT = 0;
                let mut delta_x: SIZE = 0;
                let mut delta_y: SIZE = 0;
                let mut ship_delta_x: SIZE = 0;
                let mut ship_delta_y: SIZE = 0;
                let mut other_delta_x: SIZE = 0;
                let mut other_delta_y: SIZE = 0;
                GetCurrentVelocityComponents(
                    &raw mut (*ShipPtr).velocity,
                    &raw mut ship_delta_x,
                    &raw mut ship_delta_y,
                );
                GetCurrentVelocityComponents(
                    &raw mut (*(*lpEvalDesc).ObjectPtr).velocity,
                    &raw mut other_delta_x,
                    &raw mut other_delta_y,
                );
                delta_x = (ship_delta_x as ::core::ffi::c_int
                    - other_delta_x as ::core::ffi::c_int) as SIZE;
                delta_y = (ship_delta_y as ::core::ffi::c_int
                    - other_delta_y as ::core::ffi::c_int) as SIZE;
                travel_facing = ARCTAN(delta_x, delta_y);
                delta_x = ((*(*lpEvalDesc).ObjectPtr).next.location.x
                    as ::core::ffi::c_int
                    - (*ShipPtr).next.location.x as ::core::ffi::c_int) as SIZE;
                delta_y = ((*(*lpEvalDesc).ObjectPtr).next.location.y
                    as ::core::ffi::c_int
                    - (*ShipPtr).next.location.y as ::core::ffi::c_int) as SIZE;
                direction_facing = ARCTAN(delta_x, delta_y);
                if (travel_facing as ::core::ffi::c_int
                    - direction_facing as ::core::ffi::c_int
                    + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                        >> 3 as ::core::ffi::c_int)
                    & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int) as COUNT as ::core::ffi::c_int
                    <= (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                        >> 2 as ::core::ffi::c_int
                {
                    (*StarShipPtr).ship_input_state = ((*StarShipPtr).ship_input_state
                        as ::core::ffi::c_int
                        | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as BYTE;
                }
            }
        }
    };
}
unsafe extern "C" fn androsynth_postprocess(mut ElementPtr: *mut ELEMENT) {
    let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
    StarShipPtr = (*ElementPtr).pParent as *mut STARSHIP;
    if (*ElementPtr).next.image.farray
        == &raw mut (*(*StarShipPtr).RaceDescPtr).ship_data.special as *mut FRAME
    {
        if (*StarShipPtr).cur_status_flags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int != 0
            || (*(*StarShipPtr).RaceDescPtr).ship_info.energy_level as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
        {
            (*(*StarShipPtr).RaceDescPtr).characteristics.energy_regeneration = -1
                as ::core::ffi::c_int as BYTE;
            (*StarShipPtr).energy_counter = 8 as BYTE;
            if (*StarShipPtr).cur_status_flags as ::core::ffi::c_int
                & (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int != 0
            {
                ProcessSound(
                    SetAbsStringTableIndex(
                        (*(*StarShipPtr).RaceDescPtr).ship_data.ship_sounds,
                        1 as COUNT,
                    ),
                    ElementPtr,
                );
                (*ElementPtr).c2rust_unnamed_2.turn_wait = 0 as BYTE;
                (*ElementPtr).c2rust_unnamed_3.thrust_wait = 0 as BYTE;
                (*(*StarShipPtr).RaceDescPtr).characteristics.special_wait = (*(*StarShipPtr)
                    .RaceDescPtr)
                    .characteristics
                    .turn_wait;
                (*ElementPtr).c2rust_unnamed_1.mass_points = 1 as BYTE;
                (*(*StarShipPtr).RaceDescPtr).characteristics.turn_wait = 1 as BYTE;
                let shipData: ANDROSYNTH_DATA = ANDROSYNTH_DATA {
                    collision_func: (*ElementPtr).collision_func,
                };
                SetCustomShipData((*StarShipPtr).RaceDescPtr, &raw const shipData);
                (*ElementPtr).collision_func = Some(
                    blazer_collision
                        as unsafe extern "C" fn(
                            *mut ELEMENT,
                            *mut POINT,
                            *mut ELEMENT,
                            *mut POINT,
                        ) -> (),
                ) as Option<ElementCollisionFunc>;
            }
        }
        if (*(*StarShipPtr).RaceDescPtr).ship_info.energy_level as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
            DeltaEnergy(ElementPtr, -(-1 as ::core::ffi::c_int) as SIZE);
            (*StarShipPtr).energy_counter = 1 as BYTE;
        }
    }
}
unsafe extern "C" fn androsynth_preprocess(mut ElementPtr: *mut ELEMENT) {
    let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
    let mut cur_status_flags: STATUS_FLAGS = 0;
    StarShipPtr = (*ElementPtr).pParent as *mut STARSHIP;
    cur_status_flags = (*StarShipPtr).cur_status_flags;
    if (*ElementPtr).next.image.farray
        == &raw mut (*(*StarShipPtr).RaceDescPtr).ship_data.ship as *mut FRAME
    {
        if cur_status_flags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int != 0
        {
            if ((*(*StarShipPtr).RaceDescPtr).ship_info.energy_level
                as ::core::ffi::c_int) < 2 as ::core::ffi::c_int
            {
                DeltaEnergy(ElementPtr, -2 as SIZE);
            } else {
                cur_status_flags = (cur_status_flags as ::core::ffi::c_int
                    & !((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int))
                    as STATUS_FLAGS;
                (*ElementPtr).next.image.farray = &raw mut (*(*StarShipPtr).RaceDescPtr)
                    .ship_data
                    .special as *mut FRAME;
                (*ElementPtr).next.image.frame = SetEquFrameIndex(
                    (*(*StarShipPtr).RaceDescPtr).ship_data.special[0usize],
                    (*ElementPtr).next.image.frame,
                );
                (*ElementPtr).state_flags = ((*ElementPtr).state_flags
                    as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int)
                    as ELEMENT_FLAGS;
            }
        }
    } else {
        cur_status_flags = (cur_status_flags as ::core::ffi::c_int
            & !((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)) as STATUS_FLAGS;
        if (*(*StarShipPtr).RaceDescPtr).characteristics.turn_wait as ::core::ffi::c_int
            > 1 as ::core::ffi::c_int
        {
            (*(*StarShipPtr).RaceDescPtr).characteristics.special_wait = ((*(*StarShipPtr)
                .RaceDescPtr)
                .characteristics
                .special_wait as ::core::ffi::c_int
                + ((*(*StarShipPtr).RaceDescPtr).characteristics.turn_wait
                    as ::core::ffi::c_int - 1 as ::core::ffi::c_int)) as BYTE;
            (*(*StarShipPtr).RaceDescPtr).characteristics.turn_wait = 1 as BYTE;
        }
        if (*(*StarShipPtr).RaceDescPtr).ship_info.energy_level as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
            memset(
                &raw mut (*ElementPtr).velocity as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<VELOCITY_DESC>(),
            );
            cur_status_flags = (cur_status_flags as ::core::ffi::c_int
                & !((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int))
                as STATUS_FLAGS;
            (*(*StarShipPtr).RaceDescPtr).characteristics.turn_wait = (*(*StarShipPtr)
                .RaceDescPtr)
                .characteristics
                .special_wait;
            (*(*StarShipPtr).RaceDescPtr).characteristics.energy_regeneration = 1
                as BYTE;
            (*ElementPtr).c2rust_unnamed_1.mass_points = 6 as BYTE;
            (*ElementPtr).collision_func = (*GetCustomShipData(
                (*StarShipPtr).RaceDescPtr,
            ))
                .collision_func;
            (*ElementPtr).next.image.farray = &raw mut (*(*StarShipPtr).RaceDescPtr)
                .ship_data
                .ship as *mut FRAME;
            (*ElementPtr).next.image.frame = SetEquFrameIndex(
                (*(*StarShipPtr).RaceDescPtr).ship_data.ship[0usize],
                (*ElementPtr).next.image.frame,
            );
            (*ElementPtr).state_flags = ((*ElementPtr).state_flags as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as ELEMENT_FLAGS;
        } else if (*ElementPtr).c2rust_unnamed_3.thrust_wait != 0 {
            (*ElementPtr).c2rust_unnamed_3.thrust_wait = (*ElementPtr)
                .c2rust_unnamed_3
                .thrust_wait
                .wrapping_sub(1);
        } else {
            let mut facing: COUNT = 0;
            facing = (*StarShipPtr).ShipFacing;
            if (*ElementPtr).c2rust_unnamed_2.turn_wait as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
                && cur_status_flags as ::core::ffi::c_int
                    & ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                        | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) != 0
            {
                if cur_status_flags as ::core::ffi::c_int
                    & (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int != 0
                {
                    facing = facing.wrapping_sub(1);
                } else {
                    facing = facing.wrapping_add(1);
                }
            }
            SetVelocityVector(
                &raw mut (*ElementPtr).velocity,
                60 as SIZE,
                (facing as ::core::ffi::c_int
                    & ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int) as COUNT,
            );
            cur_status_flags = (cur_status_flags as ::core::ffi::c_int
                | ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int))
                as STATUS_FLAGS;
        }
    }
    (*StarShipPtr).cur_status_flags = cur_status_flags;
}
unsafe extern "C" fn uninit_androsynth(mut pRaceDesc: *mut RACE_DESC) {
    SetCustomShipData(pRaceDesc, ::core::ptr::null::<CustomShipData_t>());
}
#[no_mangle]
pub unsafe extern "C" fn init_androsynth() -> *mut RACE_DESC {
    let mut RaceDescPtr: *mut RACE_DESC = ::core::ptr::null_mut::<RACE_DESC>();
    androsynth_desc.uninit_func = Some(
        uninit_androsynth as unsafe extern "C" fn(*mut RACE_DESC) -> (),
    ) as Option<UNINIT_FUNC>;
    androsynth_desc.preprocess_func = Some(
        androsynth_preprocess as unsafe extern "C" fn(*mut ELEMENT) -> (),
    ) as Option<PREPROCESS_FUNC>;
    androsynth_desc.postprocess_func = Some(
        androsynth_postprocess as unsafe extern "C" fn(*mut ELEMENT) -> (),
    ) as Option<POSTPROCESS_FUNC>;
    androsynth_desc.init_weapon_func = Some(
        initialize_bubble as unsafe extern "C" fn(*mut ELEMENT, *mut HELEMENT) -> COUNT,
    ) as Option<INIT_WEAPON_FUNC>;
    androsynth_desc.cyborg_control.intelligence_func = Some(
        androsynth_intelligence
            as unsafe extern "C" fn(*mut ELEMENT, *mut EVALUATE_DESC, COUNT) -> (),
    ) as Option<IntelligenceFunc>;
    RaceDescPtr = &raw mut androsynth_desc;
    return RaceDescPtr;
}
