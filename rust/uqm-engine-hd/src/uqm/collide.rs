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

// Rust translation of src/uqm/collide.c.
extern "C" {
    pub type string_table_entry;
    pub type frame_desc;
    pub type tfb_soundsample;
    fn memset(
        _: *mut ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    static mut sinetab: [SDWORD; 0];
    fn ARCTAN(delta_x: SDWORD, delta_y: SDWORD) -> COUNT;
    fn GetCurrentVelocityComponents(
        velocityptr: *mut VELOCITY_DESC,
        pdx: *mut SIZE,
        pdy: *mut SIZE,
    );
    fn SetVelocityComponents(velocityptr: *mut VELOCITY_DESC, dx: SDWORD, dy: SDWORD);
    fn DeltaVelocityComponents(velocityptr: *mut VELOCITY_DESC, dx: SDWORD, dy: SDWORD);
    fn square_root(value: DWORD) -> COUNT;
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
pub type QUEUE_HANDLE = *mut ::core::ffi::c_void;
pub type HLINK = QUEUE_HANDLE;
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
#[no_mangle]
pub unsafe extern "C" fn collide(
    mut ElementPtr0: *mut ELEMENT,
    mut ElementPtr1: *mut ELEMENT,
) {
    let mut speed: SIZE = 0;
    let mut dx0: SIZE = 0;
    let mut dy0: SIZE = 0;
    let mut dx1: SIZE = 0;
    let mut dy1: SIZE = 0;
    let mut dx_rel: SIZE = 0;
    let mut dy_rel: SIZE = 0;
    let mut TravelAngle0: SIZE = 0;
    let mut TravelAngle1: SIZE = 0;
    let mut ImpactAngle0: SIZE = 0;
    let mut ImpactAngle1: SIZE = 0;
    let mut RelTravelAngle: SIZE = 0;
    let mut Directness: SIZE = 0;
    dx_rel = ((*ElementPtr0).next.location.x as ::core::ffi::c_int
        - (*ElementPtr1).next.location.x as ::core::ffi::c_int) as SIZE;
    dy_rel = ((*ElementPtr0).next.location.y as ::core::ffi::c_int
        - (*ElementPtr1).next.location.y as ::core::ffi::c_int) as SIZE;
    ImpactAngle0 = ARCTAN(dx_rel as SDWORD, dy_rel as SDWORD) as SIZE;
    ImpactAngle1 = (ImpactAngle0 as ::core::ffi::c_int
        + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
            >> 1 as ::core::ffi::c_int)
        & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
            - 1 as ::core::ffi::c_int) as DWORD as SIZE;
    GetCurrentVelocityComponents(
        &raw mut (*ElementPtr0).velocity,
        &raw mut dx0,
        &raw mut dy0,
    );
    TravelAngle0 = (*ElementPtr0).velocity.TravelAngle as SIZE;
    GetCurrentVelocityComponents(
        &raw mut (*ElementPtr1).velocity,
        &raw mut dx1,
        &raw mut dy1,
    );
    TravelAngle1 = (*ElementPtr1).velocity.TravelAngle as SIZE;
    dx_rel = (dx0 as ::core::ffi::c_int - dx1 as ::core::ffi::c_int) as SIZE;
    dy_rel = (dy0 as ::core::ffi::c_int - dy1 as ::core::ffi::c_int) as SIZE;
    RelTravelAngle = ARCTAN(dx_rel as SDWORD, dy_rel as SDWORD) as SIZE;
    speed = square_root(
        (dx_rel as ::core::ffi::c_long * dx_rel as ::core::ffi::c_long
            + dy_rel as ::core::ffi::c_long * dy_rel as ::core::ffi::c_long) as DWORD,
    ) as SIZE;
    Directness = (RelTravelAngle as ::core::ffi::c_int
        - ImpactAngle0 as ::core::ffi::c_int
        & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
            - 1 as ::core::ffi::c_int) as DWORD as SIZE;
    if Directness as ::core::ffi::c_int
        <= (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int
        || Directness as ::core::ffi::c_int
            >= ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int)
                + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                    >> 2 as ::core::ffi::c_int)
    {
        Directness = ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
            >> 1 as ::core::ffi::c_int) as SIZE;
        ImpactAngle0 = (TravelAngle0 as ::core::ffi::c_int
            + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int)) as SIZE;
        ImpactAngle1 = (TravelAngle1 as ::core::ffi::c_int
            + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int)) as SIZE;
    }
    if (*ElementPtr0).next.location.x as ::core::ffi::c_int
        == (*ElementPtr0).current.location.x as ::core::ffi::c_int
        && (*ElementPtr0).next.location.y as ::core::ffi::c_int
            == (*ElementPtr0).current.location.y as ::core::ffi::c_int
        && (*ElementPtr1).next.location.x as ::core::ffi::c_int
            == (*ElementPtr1).current.location.x as ::core::ffi::c_int
        && (*ElementPtr1).next.location.y as ::core::ffi::c_int
            == (*ElementPtr1).current.location.y as ::core::ffi::c_int
    {
        if (*ElementPtr0).state_flags as ::core::ffi::c_int
            & (*ElementPtr1).state_flags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int != 0
        {
            ImpactAngle0 = (TravelAngle0 as ::core::ffi::c_int
                + (((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                    >> 1 as ::core::ffi::c_int)
                    - ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                        >> 3 as ::core::ffi::c_int))) as SIZE;
            ImpactAngle1 = (TravelAngle1 as ::core::ffi::c_int
                + (((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                    >> 1 as ::core::ffi::c_int)
                    - ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                        >> 3 as ::core::ffi::c_int))) as SIZE;
            memset(
                &raw mut (*ElementPtr0).velocity as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<VELOCITY_DESC>(),
            );
            memset(
                &raw mut (*ElementPtr1).velocity as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<VELOCITY_DESC>(),
            );
        }
        (*ElementPtr0).state_flags = ((*ElementPtr0).state_flags as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int))
            as ELEMENT_FLAGS;
        (*ElementPtr1).state_flags = ((*ElementPtr1).state_flags as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int))
            as ELEMENT_FLAGS;
    }
    let mut mass0: SIZE = 0;
    let mut mass1: SIZE = 0;
    let mut scalar: ::core::ffi::c_long = 0;
    mass0 = (*ElementPtr0).c2rust_unnamed_1.mass_points as SIZE;
    mass1 = (*ElementPtr1).c2rust_unnamed_1.mass_points as SIZE;
    scalar = (*(&raw mut sinetab as *mut SDWORD)
        .offset(
            (Directness as ::core::ffi::c_int
                & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int) as DWORD as isize,
        ) as ::core::ffi::c_long
        * ((speed as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
            as ::core::ffi::c_long >> 14 as ::core::ffi::c_int) as SDWORD
        as ::core::ffi::c_long
        * (mass0 as ::core::ffi::c_int * mass1 as ::core::ffi::c_int)
            as ::core::ffi::c_long;
    if !((*ElementPtr0).c2rust_unnamed_1.mass_points as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int > 10 as ::core::ffi::c_int * 10 as ::core::ffi::c_int)
    {
        if (*ElementPtr0).state_flags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int != 0
        {
            let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
            StarShipPtr = (*ElementPtr0).pParent as *mut STARSHIP;
            (*StarShipPtr).cur_status_flags = ((*StarShipPtr).cur_status_flags
                as ::core::ffi::c_int
                & !((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int))
                as STATUS_FLAGS;
            if (*ElementPtr0).state_flags as ::core::ffi::c_int
                & (1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int == 0
            {
                if ((*ElementPtr0).c2rust_unnamed_2.turn_wait as ::core::ffi::c_int)
                    < 1 as ::core::ffi::c_int
                {
                    (*ElementPtr0).c2rust_unnamed_2.turn_wait = ((*ElementPtr0)
                        .c2rust_unnamed_2
                        .turn_wait as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                        as BYTE;
                }
                if ((*ElementPtr0).c2rust_unnamed_3.thrust_wait as ::core::ffi::c_int)
                    < 3 as ::core::ffi::c_int
                {
                    (*ElementPtr0).c2rust_unnamed_3.thrust_wait = ((*ElementPtr0)
                        .c2rust_unnamed_3
                        .thrust_wait as ::core::ffi::c_int + 3 as ::core::ffi::c_int)
                        as BYTE;
                }
            }
        }
        speed = (scalar
            / (mass0 as ::core::ffi::c_long
                * (mass0 as ::core::ffi::c_int + mass1 as ::core::ffi::c_int)
                    as ::core::ffi::c_long)) as SIZE;
        DeltaVelocityComponents(
            &raw mut (*ElementPtr0).velocity,
            (*(&raw mut sinetab as *mut SDWORD)
                .offset(
                    (ImpactAngle0 as ::core::ffi::c_int
                        + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                            >> 2 as ::core::ffi::c_int)
                        & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                            - 1 as ::core::ffi::c_int) as DWORD as isize,
                ) as ::core::ffi::c_long * speed as ::core::ffi::c_long
                >> 14 as ::core::ffi::c_int) as SDWORD,
            (*(&raw mut sinetab as *mut SDWORD)
                .offset(
                    (ImpactAngle0 as ::core::ffi::c_int
                        & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                            - 1 as ::core::ffi::c_int) as DWORD as isize,
                ) as ::core::ffi::c_long * speed as ::core::ffi::c_long
                >> 14 as ::core::ffi::c_int) as SDWORD,
        );
        GetCurrentVelocityComponents(
            &raw mut (*ElementPtr0).velocity,
            &raw mut dx0,
            &raw mut dy0,
        );
        if (dx0 as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
            dx0 = -(dx0 as ::core::ffi::c_int) as SIZE;
        }
        if (dy0 as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
            dy0 = -(dy0 as ::core::ffi::c_int) as SIZE;
        }
        if (dx0 as ::core::ffi::c_int + dy0 as ::core::ffi::c_int
            >> 5 as ::core::ffi::c_int)
            < (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
        {
            SetVelocityComponents(
                &raw mut (*ElementPtr0).velocity,
                (*(&raw mut sinetab as *mut SDWORD)
                    .offset(
                        (ImpactAngle0 as ::core::ffi::c_int
                            + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                                >> 2 as ::core::ffi::c_int)
                            & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                                - 1 as ::core::ffi::c_int) as DWORD as isize,
                    ) as ::core::ffi::c_long
                    * ((((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                        << 5 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                        as ::core::ffi::c_long >> 14 as ::core::ffi::c_int) as SDWORD,
                (*(&raw mut sinetab as *mut SDWORD)
                    .offset(
                        (ImpactAngle0 as ::core::ffi::c_int
                            & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                                - 1 as ::core::ffi::c_int) as DWORD as isize,
                    ) as ::core::ffi::c_long
                    * ((((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                        << 5 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                        as ::core::ffi::c_long >> 14 as ::core::ffi::c_int) as SDWORD,
            );
        }
    }
    if !((*ElementPtr1).c2rust_unnamed_1.mass_points as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int > 10 as ::core::ffi::c_int * 10 as ::core::ffi::c_int)
    {
        if (*ElementPtr1).state_flags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int != 0
        {
            let mut StarShipPtr_0: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
            StarShipPtr_0 = (*ElementPtr1).pParent as *mut STARSHIP;
            (*StarShipPtr_0).cur_status_flags = ((*StarShipPtr_0).cur_status_flags
                as ::core::ffi::c_int
                & !((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int))
                as STATUS_FLAGS;
            if (*ElementPtr1).state_flags as ::core::ffi::c_int
                & (1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int == 0
            {
                if ((*ElementPtr1).c2rust_unnamed_2.turn_wait as ::core::ffi::c_int)
                    < 1 as ::core::ffi::c_int
                {
                    (*ElementPtr1).c2rust_unnamed_2.turn_wait = ((*ElementPtr1)
                        .c2rust_unnamed_2
                        .turn_wait as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                        as BYTE;
                }
                if ((*ElementPtr1).c2rust_unnamed_3.thrust_wait as ::core::ffi::c_int)
                    < 3 as ::core::ffi::c_int
                {
                    (*ElementPtr1).c2rust_unnamed_3.thrust_wait = ((*ElementPtr1)
                        .c2rust_unnamed_3
                        .thrust_wait as ::core::ffi::c_int + 3 as ::core::ffi::c_int)
                        as BYTE;
                }
            }
        }
        speed = (scalar
            / (mass1 as ::core::ffi::c_long
                * (mass0 as ::core::ffi::c_int + mass1 as ::core::ffi::c_int)
                    as ::core::ffi::c_long)) as SIZE;
        DeltaVelocityComponents(
            &raw mut (*ElementPtr1).velocity,
            (*(&raw mut sinetab as *mut SDWORD)
                .offset(
                    (ImpactAngle1 as ::core::ffi::c_int
                        + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                            >> 2 as ::core::ffi::c_int)
                        & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                            - 1 as ::core::ffi::c_int) as DWORD as isize,
                ) as ::core::ffi::c_long * speed as ::core::ffi::c_long
                >> 14 as ::core::ffi::c_int) as SDWORD,
            (*(&raw mut sinetab as *mut SDWORD)
                .offset(
                    (ImpactAngle1 as ::core::ffi::c_int
                        & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                            - 1 as ::core::ffi::c_int) as DWORD as isize,
                ) as ::core::ffi::c_long * speed as ::core::ffi::c_long
                >> 14 as ::core::ffi::c_int) as SDWORD,
        );
        GetCurrentVelocityComponents(
            &raw mut (*ElementPtr1).velocity,
            &raw mut dx1,
            &raw mut dy1,
        );
        if (dx1 as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
            dx1 = -(dx1 as ::core::ffi::c_int) as SIZE;
        }
        if (dy1 as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
            dy1 = -(dy1 as ::core::ffi::c_int) as SIZE;
        }
        if (dx1 as ::core::ffi::c_int + dy1 as ::core::ffi::c_int
            >> 5 as ::core::ffi::c_int)
            < (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
        {
            SetVelocityComponents(
                &raw mut (*ElementPtr1).velocity,
                (*(&raw mut sinetab as *mut SDWORD)
                    .offset(
                        (ImpactAngle1 as ::core::ffi::c_int
                            + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                                >> 2 as ::core::ffi::c_int)
                            & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                                - 1 as ::core::ffi::c_int) as DWORD as isize,
                    ) as ::core::ffi::c_long
                    * ((((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                        << 5 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                        as ::core::ffi::c_long >> 14 as ::core::ffi::c_int) as SDWORD,
                (*(&raw mut sinetab as *mut SDWORD)
                    .offset(
                        (ImpactAngle1 as ::core::ffi::c_int
                            & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                                - 1 as ::core::ffi::c_int) as DWORD as isize,
                    ) as ::core::ffi::c_long
                    * ((((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                        << 5 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                        as ::core::ffi::c_long >> 14 as ::core::ffi::c_int) as SDWORD,
            );
        }
    }
}
