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

// Rust translation of src/uqm/gravity.c.
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
    fn DrawablesIntersect(
        pControl0: *mut INTERSECT_CONTROL,
        pControl1: *mut INTERSECT_CONTROL,
        max_time_val: TIME_VALUE,
    ) -> TIME_VALUE;
    fn SetEquFrameIndex(DstFrame: FRAME, SrcFrame: FRAME) -> FRAME;
    static mut ScreenWidth: ::core::ffi::c_int;
    static mut ScreenHeight: ::core::ffi::c_int;
    static mut sinetab: [SIZE; 0];
    fn ARCTAN(delta_x: SIZE, delta_y: SIZE) -> COUNT;
    fn DeltaVelocityComponents(velocityptr: *mut VELOCITY_DESC, dx: SIZE, dy: SIZE);
    static mut disp_q: QUEUE;
}
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
pub type STATUS_FLAGS = UWORD;
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
#[no_mangle]
pub unsafe extern "C" fn CalculateGravity(mut ElementPtr: *mut ELEMENT) -> BOOLEAN {
    let mut retval: BOOLEAN = BOOLEAN::FALSE;
    let mut HasGravity: BOOLEAN = BOOLEAN::FALSE;
    let mut hTestElement: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut hSuccElement: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
    retval = BOOLEAN::FALSE;
    HasGravity = BOOLEAN(
        ((*ElementPtr).state_flags as ::core::ffi::c_int
            & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) == 0
            && (*ElementPtr).c2rust_unnamed_1.mass_points as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int
                > 10 as ::core::ffi::c_int * 10 as ::core::ffi::c_int)
            as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
    hTestElement = disp_q.head as HELEMENT;
    while !hTestElement.is_null() {
        let mut TestHasGravity: BOOLEAN = BOOLEAN::FALSE;
        let mut TestElementPtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
        TestElementPtr = LockLink(&raw mut disp_q, hTestElement) as *mut ELEMENT;
        if TestElementPtr != ElementPtr
            && (*TestElementPtr).state_flags as ::core::ffi::c_int
                & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) == 0
            && {
                TestHasGravity = BOOLEAN(
                    ((*TestElementPtr).c2rust_unnamed_1.mass_points as ::core::ffi::c_int
                        + 1 as ::core::ffi::c_int
                        > 10 as ::core::ffi::c_int * 10 as ::core::ffi::c_int)
                        as ::core::ffi::c_int as ::core::ffi::c_uint,
                );
                TestHasGravity.0 != HasGravity.0
            }
        {
            let mut abs_dx: COUNT = 0;
            let mut abs_dy: COUNT = 0;
            let mut dx: SIZE = 0;
            let mut dy: SIZE = 0;
            if (*ElementPtr).state_flags as ::core::ffi::c_int
                & (1 as ::core::ffi::c_int) << 11 as ::core::ffi::c_int == 0
            {
                dx = ((*ElementPtr).current.location.x as ::core::ffi::c_int
                    - (*TestElementPtr).current.location.x as ::core::ffi::c_int)
                    as SIZE;
                dy = ((*ElementPtr).current.location.y as ::core::ffi::c_int
                    - (*TestElementPtr).current.location.y as ::core::ffi::c_int)
                    as SIZE;
            } else {
                dx = ((*ElementPtr).next.location.x as ::core::ffi::c_int
                    - (*TestElementPtr).next.location.x as ::core::ffi::c_int) as SIZE;
                dy = ((*ElementPtr).next.location.y as ::core::ffi::c_int
                    - (*TestElementPtr).next.location.y as ::core::ffi::c_int) as SIZE;
            }
            dx = (if (dx as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                if -(dx as ::core::ffi::c_int)
                    <= ((ScreenWidth - 64 as ::core::ffi::c_int
                        - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                        << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                        >> 1 as ::core::ffi::c_int
                {
                    dx as ::core::ffi::c_int
                } else {
                    (((ScreenWidth - 64 as ::core::ffi::c_int
                        - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                        << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                        + dx as ::core::ffi::c_int
                }
            } else if dx as ::core::ffi::c_int
                <= ((ScreenWidth - 64 as ::core::ffi::c_int
                    - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                    << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    >> 1 as ::core::ffi::c_int
            {
                dx as ::core::ffi::c_int
            } else {
                dx as ::core::ffi::c_int
                    - (((ScreenWidth - 64 as ::core::ffi::c_int
                        - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                        << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
            }) as SIZE;
            dy = (if (dy as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                if -(dy as ::core::ffi::c_int)
                    <= ((ScreenHeight
                        - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                        << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                        >> 1 as ::core::ffi::c_int
                {
                    dy as ::core::ffi::c_int
                } else {
                    (((ScreenHeight - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                        << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                        + dy as ::core::ffi::c_int
                }
            } else if dy as ::core::ffi::c_int
                <= ((ScreenHeight - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                    << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    >> 1 as ::core::ffi::c_int
            {
                dy as ::core::ffi::c_int
            } else {
                dy as ::core::ffi::c_int
                    - (((ScreenHeight
                        - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                        << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
            }) as SIZE;
            abs_dx = (if dx as ::core::ffi::c_int >= 0 as ::core::ffi::c_int {
                dx as ::core::ffi::c_int
            } else {
                -(dx as ::core::ffi::c_int)
            }) as COUNT;
            abs_dy = (if dy as ::core::ffi::c_int >= 0 as ::core::ffi::c_int {
                dy as ::core::ffi::c_int
            } else {
                -(dy as ::core::ffi::c_int)
            }) as COUNT;
            abs_dx = (abs_dx as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as COUNT;
            abs_dy = (abs_dy as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as COUNT;
            if abs_dx as ::core::ffi::c_int
                <= 255 as ::core::ffi::c_int as COUNT as ::core::ffi::c_int
                && abs_dy as ::core::ffi::c_int
                    <= 255 as ::core::ffi::c_int as COUNT as ::core::ffi::c_int
            {
                let mut dist_squared: DWORD = 0;
                dist_squared = ((abs_dx as ::core::ffi::c_int
                    * abs_dx as ::core::ffi::c_int) as DWORD)
                    .wrapping_add(
                        (abs_dy as ::core::ffi::c_int * abs_dy as ::core::ffi::c_int)
                            as DWORD,
                    );
                if dist_squared
                    <= (255 as ::core::ffi::c_int as COUNT as ::core::ffi::c_int
                        * 255 as ::core::ffi::c_int as COUNT as ::core::ffi::c_int)
                        as DWORD
                {
                    if TestHasGravity.0 != 0 {
                        retval = BOOLEAN::TRUE;
                        UnlockLink(&raw mut disp_q, hTestElement);
                        break;
                    } else {
                        let mut angle: COUNT = 0;
                        angle = ARCTAN(dx, dy);
                        DeltaVelocityComponents(
                            &raw mut (*TestElementPtr).velocity,
                            (*(&raw mut sinetab as *mut SIZE)
                                .offset(
                                    (angle as ::core::ffi::c_int
                                        + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                                            >> 2 as ::core::ffi::c_int)
                                        & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                                            - 1 as ::core::ffi::c_int) as COUNT as isize,
                                ) as ::core::ffi::c_long
                                * ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int)
                                    as ::core::ffi::c_long >> 14 as ::core::ffi::c_int) as SIZE,
                            (*(&raw mut sinetab as *mut SIZE)
                                .offset(
                                    (angle as ::core::ffi::c_int
                                        & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                                            - 1 as ::core::ffi::c_int) as COUNT as isize,
                                ) as ::core::ffi::c_long
                                * ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int)
                                    as ::core::ffi::c_long >> 14 as ::core::ffi::c_int) as SIZE,
                        );
                        if (*TestElementPtr).state_flags as ::core::ffi::c_int
                            & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int != 0
                        {
                            let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<
                                STARSHIP,
                            >();
                            StarShipPtr = (*TestElementPtr).pParent as *mut STARSHIP;
                            (*StarShipPtr).cur_status_flags = ((*StarShipPtr)
                                .cur_status_flags as ::core::ffi::c_int
                                & !((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int))
                                as STATUS_FLAGS;
                            (*StarShipPtr).cur_status_flags = ((*StarShipPtr)
                                .cur_status_flags as ::core::ffi::c_int
                                | (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
                                as STATUS_FLAGS;
                        }
                    }
                }
            }
        }
        hSuccElement = (*TestElementPtr).succ;
        UnlockLink(&raw mut disp_q, hTestElement);
        hTestElement = hSuccElement;
    }
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn TimeSpaceMatterConflict(
    mut ElementPtr: *mut ELEMENT,
) -> BOOLEAN {
    let mut hTestElement: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut hSuccElement: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut ElementControl: INTERSECT_CONTROL = INTERSECT_CONTROL {
        last_time_val: 0,
        EndPoint: POINT { x: 0, y: 0 },
        IntersectStamp: STAMP {
            origin: POINT { x: 0, y: 0 },
            frame: ::core::ptr::null_mut::<FRAME_DESC>(),
        },
    };
    ElementControl.IntersectStamp.origin.x = ((*ElementPtr).current.location.x
        as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as COORD;
    ElementControl.IntersectStamp.origin.y = ((*ElementPtr).current.location.y
        as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as COORD;
    ElementControl.IntersectStamp.frame = SetEquFrameIndex(
        *(*ElementPtr).current.image.farray.offset(0isize),
        (*ElementPtr).current.image.frame,
    );
    ElementControl.EndPoint = ElementControl.IntersectStamp.origin;
    hTestElement = disp_q.head as HELEMENT;
    while !hTestElement.is_null() {
        let mut TestElementPtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
        TestElementPtr = LockLink(&raw mut disp_q, hTestElement) as *mut ELEMENT;
        hSuccElement = (*TestElementPtr).succ;
        if TestElementPtr != ElementPtr
            && ((*TestElementPtr).state_flags as ::core::ffi::c_int
                & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) == 0
                || (*TestElementPtr).state_flags as ::core::ffi::c_int
                    & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int != 0)
        {
            let mut TestElementControl: INTERSECT_CONTROL = INTERSECT_CONTROL {
                last_time_val: 0,
                EndPoint: POINT { x: 0, y: 0 },
                IntersectStamp: STAMP {
                    origin: POINT { x: 0, y: 0 },
                    frame: ::core::ptr::null_mut::<FRAME_DESC>(),
                },
            };
            TestElementControl.IntersectStamp.origin.x = ((*TestElementPtr)
                .current
                .location
                .x as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as COORD;
            TestElementControl.IntersectStamp.origin.y = ((*TestElementPtr)
                .current
                .location
                .y as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as COORD;
            TestElementControl.IntersectStamp.frame = SetEquFrameIndex(
                *(*TestElementPtr).current.image.farray.offset(0isize),
                (*TestElementPtr).current.image.frame,
            );
            TestElementControl.EndPoint = TestElementControl.IntersectStamp.origin;
            if DrawablesIntersect(
                &raw mut ElementControl,
                &raw mut TestElementControl,
                (((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
                    + 1 as ::core::ffi::c_int) as TIME_VALUE,
            ) != 0
            {
                UnlockLink(&raw mut disp_q, hTestElement);
                break;
            }
        }
        UnlockLink(&raw mut disp_q, hTestElement);
        hTestElement = hSuccElement;
    }
    return BOOLEAN(
        (if !hTestElement.is_null() {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        }) as ::core::ffi::c_uint,
    );
}
