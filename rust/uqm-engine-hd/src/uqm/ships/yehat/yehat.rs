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

// Rust translation of src/uqm/ships/yehat/yehat.c.
extern "C" {
    pub type string_table_entry;
    pub type frame_desc;
    pub type tfb_soundsample;
    fn SetEquFrameIndex(DstFrame: FRAME, SrcFrame: FRAME) -> FRAME;
    fn SetAbsStringTableIndex(String: STRING, StringTableIndex: COUNT) -> STRING;
    static mut resolutionFactor: ::core::ffi::c_uint;
    static mut sinetab: [SDWORD; 0];
    fn PlotIntercept(
        ElementPtr0: *mut ELEMENT,
        ElementPtr1: *mut ELEMENT,
        max_turns: COUNT,
        margin_of_error: COUNT,
    ) -> COUNT;
    fn TFB_Random() -> DWORD;
    fn ProcessSound(Sound: SOUND, PositionalObject: *mut ELEMENT);
    fn DeltaEnergy(ElementPtr: *mut ELEMENT, energy_delta: SIZE) -> BOOLEAN;
    fn initialize_missile(pMissileBlock: *mut MISSILE_BLOCK) -> HELEMENT;
    fn ship_intelligence(
        ShipPtr: *mut ELEMENT,
        ObjectsOfConcern: *mut EVALUATE_DESC,
        ConcernCounter: COUNT,
    );
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct BOOLEAN(pub ::core::ffi::c_uint);
impl BOOLEAN {
    pub const FALSE: Self = Self(0);
    pub const TRUE: Self = Self(1);
}
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
static mut yehat_desc: RACE_DESC = unsafe {
    race_desc {
        c2rust_padding: [0; 8],
        c2rust_padding_0: [0; 10],
        c2rust_padding_1: [0; 2],
        c2rust_padding_2: [0; 8],
        ship_info: SHIP_INFO {
            idStr: b"terminator\0".as_ptr() as *const ::core::ffi::c_char,
            ship_flags: ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 11 as ::core::ffi::c_int) as UWORD,
            ship_cost: 23 as BYTE,
            crew_level: 20 as COUNT,
            max_crew: 20 as COUNT,
            energy_level: 10 as BYTE,
            max_energy: 10 as BYTE,
            race_strings_rsc: b"ship.yehat.text\0".as_ptr() as RESOURCE,
            icons_rsc: b"ship.yehat.icons\0".as_ptr() as RESOURCE,
            melee_icon_rsc: b"ship.yehat.meleeicons\0".as_ptr() as RESOURCE,
            race_strings: ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>(),
            icons: ::core::ptr::null_mut::<FRAME_DESC>(),
            melee_icon: ::core::ptr::null_mut::<FRAME_DESC>(),
        },
        fleet: FLEET_STUFF {
            strength: (750 as ::core::ffi::c_int / 11 as ::core::ffi::c_int
                * 2 as ::core::ffi::c_int) as COUNT,
            known_loc: point {
                x: 4970 as COORD,
                y: 40 as COORD,
            },
        },
        characteristics: CHARACTERISTIC_STUFF {
            max_thrust: 30 as COUNT,
            thrust_increment: 6 as COUNT,
            energy_regeneration: 2 as BYTE,
            weapon_energy_cost: 1 as BYTE,
            special_energy_cost: 3 as BYTE,
            energy_wait: 6 as BYTE,
            turn_wait: 2 as BYTE,
            thrust_wait: 2 as BYTE,
            weapon_wait: 0 as BYTE,
            special_wait: 2 as BYTE,
            ship_mass: 3 as BYTE,
        },
        ship_data: DATA_STUFF {
            ship_rsc: [
                b"ship.yehat.graphics.terminator.large\0".as_ptr() as RESOURCE,
                b"ship.yehat.graphics.terminator.medium\0".as_ptr() as RESOURCE,
                b"ship.yehat.graphics.terminator.small\0".as_ptr() as RESOURCE,
            ],
            weapon_rsc: [
                b"ship.yehat.graphics.missile.large\0".as_ptr() as RESOURCE,
                b"ship.yehat.graphics.missile.medium\0".as_ptr() as RESOURCE,
                b"ship.yehat.graphics.missile.small\0".as_ptr() as RESOURCE,
            ],
            special_rsc: [
                b"ship.yehat.graphics.shield.large\0".as_ptr() as RESOURCE,
                b"ship.yehat.graphics.shield.medium\0".as_ptr() as RESOURCE,
                b"ship.yehat.graphics.shield.small\0".as_ptr() as RESOURCE,
            ],
            captain_control: captain_stuff {
                captain_rsc: b"ship.yehat.graphics.captain\0".as_ptr() as RESOURCE,
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
            victory_ditty_rsc: b"ship.yehat.ditty\0".as_ptr() as RESOURCE,
            ship_sounds_rsc: b"ship.yehat.sounds\0".as_ptr() as RESOURCE,
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
            WeaponRange: (((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                * 10 as ::core::ffi::c_int / 3 as ::core::ffi::c_int) as COUNT,
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
unsafe extern "C" fn initialize_standard_missiles(
    mut ShipPtr: *mut ELEMENT,
    mut MissileArray: *mut HELEMENT,
) -> COUNT {
    let mut offs_x: SDWORD = 0;
    let mut offs_y: SDWORD = 0;
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
    MissileBlock.farray = &raw mut (*(*StarShipPtr).RaceDescPtr).ship_data.weapon
        as *mut FRAME;
    MissileBlock.index = (*StarShipPtr).ShipFacing;
    MissileBlock.face = MissileBlock.index;
    MissileBlock.sender = (*ShipPtr).playerNr;
    MissileBlock.flags = ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
        as ELEMENT_FLAGS;
    MissileBlock.pixoffs = ((16 as ::core::ffi::c_int) << resolutionFactor) as SIZE;
    MissileBlock.speed = (((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
        << resolutionFactor) as SIZE;
    MissileBlock.hit_points = 1 as SIZE;
    MissileBlock.damage = 1 as SIZE;
    MissileBlock.life = 10 as COUNT;
    MissileBlock.preprocess_func = None;
    MissileBlock.blast_offs = ((1 as ::core::ffi::c_int) << resolutionFactor) as SIZE;
    offs_x = -((*(&raw mut sinetab as *mut SDWORD)
        .offset(
            ((MissileBlock.face as ::core::ffi::c_int)
                << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int) as DWORD as isize,
        ) as ::core::ffi::c_long
        * ((8 as ::core::ffi::c_int) << resolutionFactor << 2 as ::core::ffi::c_int)
            as ::core::ffi::c_long >> 14 as ::core::ffi::c_int) as SDWORD);
    offs_y = (*(&raw mut sinetab as *mut SDWORD)
        .offset(
            (((MissileBlock.face as ::core::ffi::c_int)
                << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int)
                + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                    >> 2 as ::core::ffi::c_int)
                & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int) as DWORD as isize,
        ) as ::core::ffi::c_long
        * ((8 as ::core::ffi::c_int) << resolutionFactor << 2 as ::core::ffi::c_int)
            as ::core::ffi::c_long >> 14 as ::core::ffi::c_int) as SDWORD;
    MissileBlock.cx = ((*ShipPtr).next.location.x as SDWORD + offs_x) as COORD;
    MissileBlock.cy = ((*ShipPtr).next.location.y as SDWORD + offs_y) as COORD;
    *MissileArray.offset(0isize) = initialize_missile(&raw mut MissileBlock);
    MissileBlock.cx = ((*ShipPtr).next.location.x as SDWORD - offs_x) as COORD;
    MissileBlock.cy = ((*ShipPtr).next.location.y as SDWORD - offs_y) as COORD;
    *MissileArray.offset(1isize) = initialize_missile(&raw mut MissileBlock);
    return 2 as COUNT;
}
unsafe extern "C" fn yehat_intelligence(
    mut ShipPtr: *mut ELEMENT,
    mut ObjectsOfConcern: *mut EVALUATE_DESC,
    mut ConcernCounter: COUNT,
) {
    let mut ShieldStatus: SIZE = 0;
    let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
    let mut lpEvalDesc: *mut EVALUATE_DESC = ::core::ptr::null_mut::<EVALUATE_DESC>();
    ShieldStatus = -1 as SIZE;
    lpEvalDesc = ObjectsOfConcern
        .offset(C2Rust_Unnamed_5::ENEMY_WEAPON_INDEX.0 as ::core::ffi::c_int as isize);
    if !(*lpEvalDesc).ObjectPtr.is_null()
        && (*lpEvalDesc).MoveState.0 == MOVEMENT_STATE::ENTICE.0
    {
        ShieldStatus = 0 as SIZE;
        if (*(*lpEvalDesc).ObjectPtr).state_flags as ::core::ffi::c_int
            & ((1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int) == 0
        {
            (*lpEvalDesc).MoveState = MOVEMENT_STATE::PURSUE;
        } else if (*(*lpEvalDesc).ObjectPtr).c2rust_unnamed_1.mass_points
            as ::core::ffi::c_int != 0
            || (*(*lpEvalDesc).ObjectPtr).state_flags as ::core::ffi::c_int
                & (1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int != 0
        {
            if (*(*lpEvalDesc).ObjectPtr).state_flags as ::core::ffi::c_int
                & (1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int == 0
            {
                (*lpEvalDesc).which_turn = (((*lpEvalDesc).which_turn
                    as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as COUNT;
            } else {
                (*lpEvalDesc).which_turn = ((*lpEvalDesc).which_turn
                    as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as COUNT;
                if (*lpEvalDesc).which_turn as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                {
                    (*lpEvalDesc).which_turn = 1 as COUNT;
                }
                if (*(*lpEvalDesc).ObjectPtr).c2rust_unnamed_1.mass_points != 0 {
                    (*lpEvalDesc).ObjectPtr = ::core::ptr::null_mut::<ELEMENT>();
                } else {
                    (*lpEvalDesc).MoveState = MOVEMENT_STATE::PURSUE;
                }
            }
            ShieldStatus = 1 as SIZE;
        }
    }
    StarShipPtr = (*ShipPtr).pParent as *mut STARSHIP;
    if (*StarShipPtr).special_counter as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        (*StarShipPtr).ship_input_state = ((*StarShipPtr).ship_input_state
            as ::core::ffi::c_int
            & !((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)) as BYTE;
        if ShieldStatus != 0 {
            if (*ShipPtr).c2rust_unnamed.life_span as ::core::ffi::c_int
                <= 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                && (ShieldStatus as ::core::ffi::c_int > 0 as ::core::ffi::c_int
                    || !(*lpEvalDesc).ObjectPtr.is_null())
                && (*lpEvalDesc).which_turn as ::core::ffi::c_int
                    <= 2 as ::core::ffi::c_int
                && (ShieldStatus as ::core::ffi::c_int > 0 as ::core::ffi::c_int
                    || (*(*lpEvalDesc).ObjectPtr).state_flags as ::core::ffi::c_int
                        & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int != 0
                    || uqm_lua_checked!(PlotIntercept(
                        (*lpEvalDesc).ObjectPtr,
                        ShipPtr,
                        2 as COUNT,
                        0 as COUNT,
                    ), ()) as ::core::ffi::c_int != 0) && TFB_Random() & 3 as DWORD != 0
            {
                (*StarShipPtr).ship_input_state = ((*StarShipPtr).ship_input_state
                    as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as BYTE;
            }
            if !(*lpEvalDesc).ObjectPtr.is_null()
                && (*(*lpEvalDesc).ObjectPtr).state_flags as ::core::ffi::c_int
                    & (1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int == 0
            {
                (*lpEvalDesc).ObjectPtr = ::core::ptr::null_mut::<ELEMENT>();
            }
        }
    }
    lpEvalDesc = ObjectsOfConcern
        .offset(C2Rust_Unnamed_5::ENEMY_SHIP_INDEX.0 as ::core::ffi::c_int as isize);
    if !(*lpEvalDesc).ObjectPtr.is_null() {
        let mut EnemyStarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
        EnemyStarShipPtr = (*(*lpEvalDesc).ObjectPtr).pParent as *mut STARSHIP;
        if (*(*EnemyStarShipPtr).RaceDescPtr).ship_info.ship_flags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int == 0
        {
            (*lpEvalDesc).MoveState = MOVEMENT_STATE::PURSUE;
        }
    }
    uqm_lua_checked!(ship_intelligence(ShipPtr, ObjectsOfConcern, ConcernCounter), ());
}
unsafe extern "C" fn yehat_postprocess(mut ElementPtr: *mut ELEMENT) {
    if (*ElementPtr).state_flags as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int == 0
    {
        let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
        StarShipPtr = (*ElementPtr).pParent as *mut STARSHIP;
        if (*StarShipPtr).special_counter as ::core::ffi::c_int > 0 as ::core::ffi::c_int
        {
            if (*ElementPtr).c2rust_unnamed.life_span as ::core::ffi::c_int
                == 1 as ::core::ffi::c_int
            {
                (*StarShipPtr).special_counter = 0 as BYTE;
            } else {
                ProcessSound(
                    SetAbsStringTableIndex(
                        (*(*StarShipPtr).RaceDescPtr).ship_data.ship_sounds,
                        1 as COUNT,
                    ),
                    ElementPtr,
                );
                uqm_lua_checked!(DeltaEnergy(ElementPtr, -3 as SIZE), ());
            }
        }
    }
}
unsafe extern "C" fn yehat_preprocess(mut ElementPtr: *mut ELEMENT) {
    if (*ElementPtr).state_flags as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int == 0
    {
        let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
        StarShipPtr = (*ElementPtr).pParent as *mut STARSHIP;
        if (*ElementPtr).c2rust_unnamed.life_span as ::core::ffi::c_int
            > 1 as ::core::ffi::c_int
            && {
                (*ElementPtr).c2rust_unnamed.life_span = (*ElementPtr)
                    .c2rust_unnamed
                    .life_span
                    .wrapping_sub(1);
                (*ElementPtr).c2rust_unnamed.life_span as ::core::ffi::c_int
                    == 1 as ::core::ffi::c_int
            }
            || (*ElementPtr).c2rust_unnamed.life_span as ::core::ffi::c_int
                == 1 as ::core::ffi::c_int
                && (*ElementPtr).next.image.farray
                    == &raw mut (*(*StarShipPtr).RaceDescPtr).ship_data.special
                        as *mut FRAME
        {
            (*ElementPtr).next.image.farray = &raw mut (*(*StarShipPtr).RaceDescPtr)
                .ship_data
                .ship as *mut FRAME;
            (*ElementPtr).next.image.frame = SetEquFrameIndex(
                (*(*StarShipPtr).RaceDescPtr).ship_data.ship[0usize],
                (*ElementPtr).next.image.frame,
            );
            (*ElementPtr).state_flags = ((*ElementPtr).state_flags as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as ELEMENT_FLAGS;
        }
        if (*StarShipPtr).cur_status_flags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int != 0
            && (*StarShipPtr).special_counter as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
        {
            if ((*(*StarShipPtr).RaceDescPtr).ship_info.energy_level
                as ::core::ffi::c_int) < 3 as ::core::ffi::c_int
            {
                uqm_lua_checked!(DeltaEnergy(ElementPtr, -3 as SIZE), ());
            } else {
                (*ElementPtr).c2rust_unnamed.life_span = (10 as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int) as COUNT;
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
                (*StarShipPtr).special_counter = (*(*StarShipPtr).RaceDescPtr)
                    .characteristics
                    .special_wait;
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn init_yehat() -> *mut RACE_DESC {
    let mut RaceDescPtr: *mut RACE_DESC = ::core::ptr::null_mut::<RACE_DESC>();
    if if resolutionFactor != 2 as ::core::ffi::c_uint {
        BOOLEAN::FALSE.0 as ::core::ffi::c_int
    } else {
        BOOLEAN::TRUE.0 as ::core::ffi::c_int
    } != 0
    {
        yehat_desc.characteristics.max_thrust = ((30 as ::core::ffi::c_int)
            << resolutionFactor) as COUNT;
        yehat_desc.characteristics.thrust_increment = ((6 as ::core::ffi::c_int)
            << resolutionFactor) as COUNT;
        yehat_desc.cyborg_control.WeaponRange = ((((20 as ::core::ffi::c_int)
            << 2 as ::core::ffi::c_int) << resolutionFactor) * 10 as ::core::ffi::c_int
            / 3 as ::core::ffi::c_int) as COUNT;
    } else {
        yehat_desc.characteristics.max_thrust = 30 as COUNT;
        yehat_desc.characteristics.thrust_increment = 6 as COUNT;
        yehat_desc.cyborg_control.WeaponRange = (((20 as ::core::ffi::c_int)
            << 2 as ::core::ffi::c_int) * 10 as ::core::ffi::c_int
            / 3 as ::core::ffi::c_int) as COUNT;
    }
    yehat_desc.preprocess_func = Some(
        yehat_preprocess as unsafe extern "C" fn(*mut ELEMENT) -> (),
    ) as Option<PREPROCESS_FUNC>;
    yehat_desc.postprocess_func = Some(
        yehat_postprocess as unsafe extern "C" fn(*mut ELEMENT) -> (),
    ) as Option<POSTPROCESS_FUNC>;
    yehat_desc.init_weapon_func = Some(
        initialize_standard_missiles
            as unsafe extern "C" fn(*mut ELEMENT, *mut HELEMENT) -> COUNT,
    ) as Option<INIT_WEAPON_FUNC>;
    yehat_desc.cyborg_control.intelligence_func = Some(
        yehat_intelligence
            as unsafe extern "C" fn(*mut ELEMENT, *mut EVALUATE_DESC, COUNT) -> (),
    ) as Option<IntelligenceFunc>;
    RaceDescPtr = &raw mut yehat_desc;
    return RaceDescPtr;
}
