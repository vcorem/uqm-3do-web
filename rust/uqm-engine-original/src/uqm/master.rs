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

// Rust translation of src/uqm/master.c.
extern "C" {
    pub type string_table_entry;
    pub type string_table;
    pub type frame_desc;
    pub type drawable_desc;
    pub type tfb_soundsample;
    fn DestroyDrawable(Drawable: DRAWABLE) -> BOOLEAN;
    fn ReleaseDrawable(Frame: FRAME) -> DRAWABLE;
    fn __assert_fail(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    );
    fn DestroyStringTable(StringTable: STRING_TABLE) -> BOOLEAN;
    fn ReleaseStringTable(String: STRING) -> STRING_TABLE;
    fn SetAbsStringTableIndex(String: STRING, StringTableIndex: COUNT) -> STRING;
    fn GetStringAddress(String: STRING) -> STRINGPTR;
    fn AllocLink(pq: *mut QUEUE) -> HLINK;
    fn InitQueue(pq: *mut QUEUE, num_elements: COUNT, size: OBJ_SIZE) -> BOOLEAN;
    fn UninitQueue(pq: *mut QUEUE) -> BOOLEAN;
    fn InsertQueue(pq: *mut QUEUE, hLink: HLINK, hRefLink: HLINK);
    fn strcmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn GetStarShipFromIndex(pShipQ: *mut QUEUE, Index: COUNT) -> HLINK;
    fn load_ship(SpeciesID: SPECIES_ID, LoadBattleData: BOOLEAN) -> *mut RACE_DESC;
    fn free_ship(
        RaceDescPtr: *mut RACE_DESC,
        FreeIconData: BOOLEAN,
        FreeBattleData: BOOLEAN,
    );
}
pub type uint8 = ::core::ffi::c_uchar;
pub type uint16 = ::core::ffi::c_ushort;
pub type sint16 = ::core::ffi::c_short;
pub type BYTE = uint8;
pub type UWORD = uint16;
pub type SWORD = sint16;
pub type COUNT = UWORD;
pub type SIZE = SWORD;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct BOOLEAN(pub ::core::ffi::c_uint);
impl BOOLEAN {
    pub const FALSE: Self = Self(0);
    pub const TRUE: Self = Self(1);
}
pub type RESOURCE = *const ::core::ffi::c_char;
pub type STRING_TABLE_ENTRY_DESC = string_table_entry;
pub type STRING_TABLE_DESC = string_table;
pub type STRING_TABLE = *mut STRING_TABLE_DESC;
pub type STRING = *mut STRING_TABLE_ENTRY_DESC;
pub type STRINGPTR = *mut ::core::ffi::c_char;
pub type FRAME_DESC = frame_desc;
pub type DRAWABLE_DESC = drawable_desc;
pub type FRAME = *mut FRAME_DESC;
pub type DRAWABLE = *mut DRAWABLE_DESC;
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
pub type QUEUE_HANDLE = *mut ::core::ffi::c_void;
pub type OBJ_SIZE = UWORD;
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
pub type HELEMENT = HLINK;
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
pub type ELEMENT = element;
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
pub type STATE = state;
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
pub type VELOCITY_DESC = velocity_desc;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct velocity_desc {
    pub TravelAngle: COUNT,
    pub vector: EXTENT,
    pub fract: EXTENT,
    pub error: EXTENT,
    pub incr: EXTENT,
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
pub type ELEMENT_FLAGS = UWORD;
pub type ElementProcessFunc = unsafe extern "C" fn(*mut ELEMENT) -> ();
pub type ElementCollisionFunc = unsafe extern "C" fn(
    *mut ELEMENT,
    *mut POINT,
    *mut ELEMENT,
    *mut POINT,
) -> ();
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
pub type HMASTERSHIP = HLINK;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MASTER_SHIP_INFO {
    pub pred: HMASTERSHIP,
    pub succ: HMASTERSHIP,
    pub SpeciesID: SPECIES_ID,
    pub ShipInfo: SHIP_INFO,
    pub Fleet: FLEET_STUFF,
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
pub static mut master_q: QUEUE = QUEUE {
    head: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    tail: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    pq_tab: ::core::ptr::null_mut::<BYTE>(),
    free_list: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    object_size: 0,
    num_objects: 0,
};
#[no_mangle]
pub unsafe extern "C" fn LoadMasterShipList(
    mut YieldProcessing: Option<unsafe extern "C" fn() -> ()>,
) {
    let mut num_entries: COUNT = 0;
    let mut s_id: SPECIES_ID = SPECIES_ID::ARILOU_ID;
    num_entries = (SPECIES_ID::LAST_MELEE_ID.0 as ::core::ffi::c_int
        - SPECIES_ID::ARILOU_ID.0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
        as COUNT;
    InitQueue(
        &raw mut master_q,
        num_entries,
        ::core::mem::size_of::<MASTER_SHIP_INFO>() as OBJ_SIZE,
    );
    loop {
        let c2rust_fresh0 = num_entries;
        num_entries = num_entries.wrapping_sub(1);
        if c2rust_fresh0 == 0 {
            break;
        }
        let mut hBuiltShip: HMASTERSHIP = ::core::ptr::null_mut::<::core::ffi::c_void>();
        let mut builtName: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut hStarShip: HMASTERSHIP = ::core::ptr::null_mut::<::core::ffi::c_void>();
        let mut hNextShip: HMASTERSHIP = ::core::ptr::null_mut::<::core::ffi::c_void>();
        let mut BuiltPtr: *mut MASTER_SHIP_INFO = ::core::ptr::null_mut::<
            MASTER_SHIP_INFO,
        >();
        let mut RDPtr: *mut RACE_DESC = ::core::ptr::null_mut::<RACE_DESC>();
        hBuiltShip = AllocLink(&raw mut master_q) as HMASTERSHIP;
        if hBuiltShip.is_null() {
            continue;
        }
        if YieldProcessing.is_some() {
            YieldProcessing.expect("non-null function pointer")();
        }
        BuiltPtr = LockMasterShip(&raw mut master_q, hBuiltShip);
        let c2rust_fresh1 = s_id;
        s_id = SPECIES_ID(s_id.0.wrapping_add(1));
        (*BuiltPtr).SpeciesID = c2rust_fresh1;
        RDPtr = load_ship((*BuiltPtr).SpeciesID, BOOLEAN::FALSE);
        if RDPtr.is_null() {
            UnlockLink(&raw mut master_q, hBuiltShip);
        } else {
            (*BuiltPtr).ShipInfo = (*RDPtr).ship_info;
            (*BuiltPtr).Fleet = (*RDPtr).fleet;
            free_ship(RDPtr, BOOLEAN::FALSE, BOOLEAN::FALSE);
            builtName = GetStringAddress(
                SetAbsStringTableIndex((*BuiltPtr).ShipInfo.race_strings, 2 as COUNT),
            ) as *mut ::core::ffi::c_char;
            UnlockLink(&raw mut master_q, hBuiltShip);
            hStarShip = master_q.head as HMASTERSHIP;
            while !hStarShip.is_null() {
                let mut curName: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                    ::core::ffi::c_char,
                >();
                let mut MasterPtr: *mut MASTER_SHIP_INFO = ::core::ptr::null_mut::<
                    MASTER_SHIP_INFO,
                >();
                MasterPtr = LockMasterShip(&raw mut master_q, hStarShip);
                hNextShip = (*MasterPtr).succ;
                curName = GetStringAddress(
                    SetAbsStringTableIndex(
                        (*MasterPtr).ShipInfo.race_strings,
                        2 as COUNT,
                    ),
                ) as *mut ::core::ffi::c_char;
                UnlockLink(&raw mut master_q, hStarShip);
                if strcmp(builtName, curName) < 0 as ::core::ffi::c_int {
                    break;
                }
                hStarShip = hNextShip;
            }
            InsertQueue(&raw mut master_q, hBuiltShip, hStarShip);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn FreeMasterShipList() {
    let mut hStarShip: HMASTERSHIP = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut hNextShip: HMASTERSHIP = ::core::ptr::null_mut::<::core::ffi::c_void>();
    hStarShip = master_q.head as HMASTERSHIP;
    while !hStarShip.is_null() {
        let mut MasterPtr: *mut MASTER_SHIP_INFO = ::core::ptr::null_mut::<
            MASTER_SHIP_INFO,
        >();
        MasterPtr = LockMasterShip(&raw mut master_q, hStarShip);
        hNextShip = (*MasterPtr).succ;
        DestroyDrawable(ReleaseDrawable((*MasterPtr).ShipInfo.melee_icon));
        DestroyDrawable(ReleaseDrawable((*MasterPtr).ShipInfo.icons));
        DestroyStringTable(ReleaseStringTable((*MasterPtr).ShipInfo.race_strings));
        UnlockLink(&raw mut master_q, hStarShip);
        hStarShip = hNextShip;
    }
    UninitQueue(&raw mut master_q);
}
#[no_mangle]
pub unsafe extern "C" fn FindMasterShip(mut ship_ref: SPECIES_ID) -> HMASTERSHIP {
    let mut hStarShip: HMASTERSHIP = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut hNextShip: HMASTERSHIP = ::core::ptr::null_mut::<::core::ffi::c_void>();
    hStarShip = master_q.head as HMASTERSHIP;
    while !hStarShip.is_null() {
        let mut r#ref: SPECIES_ID = SPECIES_ID::NO_ID;
        let mut MasterPtr: *mut MASTER_SHIP_INFO = ::core::ptr::null_mut::<
            MASTER_SHIP_INFO,
        >();
        MasterPtr = LockMasterShip(&raw mut master_q, hStarShip);
        hNextShip = (*MasterPtr).succ;
        r#ref = (*MasterPtr).SpeciesID;
        UnlockLink(&raw mut master_q, hStarShip);
        if r#ref.0 == ship_ref.0 {
            break;
        }
        hStarShip = hNextShip;
    }
    return hStarShip;
}
#[no_mangle]
pub unsafe extern "C" fn FindMasterShipIndex(
    mut ship_ref: SPECIES_ID,
) -> ::core::ffi::c_int {
    let mut hStarShip: HMASTERSHIP = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut hNextShip: HMASTERSHIP = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut index: ::core::ffi::c_int = 0;
    index = 0 as ::core::ffi::c_int;
    hStarShip = master_q.head as HMASTERSHIP;
    while !hStarShip.is_null() {
        let mut r#ref: SPECIES_ID = SPECIES_ID::NO_ID;
        let mut MasterPtr: *mut MASTER_SHIP_INFO = ::core::ptr::null_mut::<
            MASTER_SHIP_INFO,
        >();
        MasterPtr = LockMasterShip(&raw mut master_q, hStarShip);
        hNextShip = (*MasterPtr).succ;
        r#ref = (*MasterPtr).SpeciesID;
        UnlockLink(&raw mut master_q, hStarShip);
        if r#ref.0 == ship_ref.0 {
            break;
        }
        index += 1;
        hStarShip = hNextShip;
    }
    return if !hStarShip.is_null() { index } else { -1 as ::core::ffi::c_int };
}
#[no_mangle]
pub unsafe extern "C" fn GetShipCostFromIndex(mut Index: ::core::ffi::c_uint) -> COUNT {
    let mut hMasterShip: HMASTERSHIP = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut MasterPtr: *mut MASTER_SHIP_INFO = ::core::ptr::null_mut::<
        MASTER_SHIP_INFO,
    >();
    let mut val: COUNT = 0;
    hMasterShip = GetStarShipFromIndex(&raw mut master_q, Index as COUNT) as HMASTERSHIP;
    if hMasterShip.is_null() {
        return 0 as COUNT;
    }
    MasterPtr = LockMasterShip(&raw mut master_q, hMasterShip);
    val = (*MasterPtr).ShipInfo.ship_cost as COUNT;
    UnlockLink(&raw mut master_q, hMasterShip);
    return val;
}
#[no_mangle]
pub unsafe extern "C" fn GetShipIconsFromIndex(mut Index: ::core::ffi::c_uint) -> FRAME {
    let mut hMasterShip: HMASTERSHIP = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut MasterPtr: *mut MASTER_SHIP_INFO = ::core::ptr::null_mut::<
        MASTER_SHIP_INFO,
    >();
    let mut val: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
    hMasterShip = GetStarShipFromIndex(&raw mut master_q, Index as COUNT) as HMASTERSHIP;
    if hMasterShip.is_null() {
        return ::core::ptr::null_mut::<FRAME_DESC>();
    }
    MasterPtr = LockMasterShip(&raw mut master_q, hMasterShip);
    val = (*MasterPtr).ShipInfo.icons;
    UnlockLink(&raw mut master_q, hMasterShip);
    return val;
}
#[no_mangle]
pub unsafe extern "C" fn GetShipMeleeIconsFromIndex(
    mut Index: ::core::ffi::c_uint,
) -> FRAME {
    let mut hMasterShip: HMASTERSHIP = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut MasterPtr: *mut MASTER_SHIP_INFO = ::core::ptr::null_mut::<
        MASTER_SHIP_INFO,
    >();
    let mut val: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
    hMasterShip = GetStarShipFromIndex(&raw mut master_q, Index as COUNT) as HMASTERSHIP;
    if hMasterShip.is_null() {
        return ::core::ptr::null_mut::<FRAME_DESC>();
    }
    MasterPtr = LockMasterShip(&raw mut master_q, hMasterShip);
    val = (*MasterPtr).ShipInfo.melee_icon;
    UnlockLink(&raw mut master_q, hMasterShip);
    return val;
}
#[inline]
unsafe extern "C" fn LockMasterShip(
    mut pq: *const QUEUE,
    mut h: HMASTERSHIP,
) -> *mut MASTER_SHIP_INFO {
    ((*pq).object_size as usize == ::core::mem::size_of::<MASTER_SHIP_INFO>()
        || {
            __assert_fail(
                b"GetLinkSize (pq) == sizeof (MASTER_SHIP_INFO)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/uqm/master.h\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                50 as ::core::ffi::c_int,
                b"LockMasterShip\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    return LockLink(pq, h) as *mut MASTER_SHIP_INFO;
}
