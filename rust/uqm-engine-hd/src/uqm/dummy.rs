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

/*
 * This file seems to be a collection of functions that don't do
 * much.
 */

// Rust translation of src/uqm/dummy.c.
extern "C" {
    pub type string_table_entry;
    pub type frame_desc;
    pub type tfb_soundsample;
    fn HMalloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn HFree(p: *mut ::core::ffi::c_void);
    fn InstallResTypeVectors(
        res_type: *const ::core::ffi::c_char,
        loadFun: Option<ResourceLoadFun>,
        freeFun: Option<ResourceFreeFun>,
        stringFun: Option<ResourceStringFun>,
    ) -> BOOLEAN;
    fn res_GetResource(res: RESOURCE) -> *mut ::core::ffi::c_void;
    fn res_DetachResource(res: RESOURCE) -> *mut ::core::ffi::c_void;
    fn atoi(_: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn init_androsynth() -> *mut RACE_DESC;
    fn init_arilou() -> *mut RACE_DESC;
    fn init_black_urquan() -> *mut RACE_DESC;
    fn init_chenjesu() -> *mut RACE_DESC;
    fn init_chmmr() -> *mut RACE_DESC;
    fn init_druuge() -> *mut RACE_DESC;
    fn init_mmrnmhrm() -> *mut RACE_DESC;
    fn init_melnorme() -> *mut RACE_DESC;
    fn init_ilwrath() -> *mut RACE_DESC;
    fn init_human() -> *mut RACE_DESC;
    fn init_mycon() -> *mut RACE_DESC;
    fn init_orz() -> *mut RACE_DESC;
    fn init_pkunk() -> *mut RACE_DESC;
    fn init_shofixti() -> *mut RACE_DESC;
    fn init_slylandro() -> *mut RACE_DESC;
    fn init_spathi() -> *mut RACE_DESC;
    fn init_supox() -> *mut RACE_DESC;
    fn init_syreen() -> *mut RACE_DESC;
    fn init_thraddash() -> *mut RACE_DESC;
    fn init_umgah() -> *mut RACE_DESC;
    fn init_urquan() -> *mut RACE_DESC;
    fn init_utwig() -> *mut RACE_DESC;
    fn init_vux() -> *mut RACE_DESC;
    fn init_yehat() -> *mut RACE_DESC;
    fn init_zoqfotpik() -> *mut RACE_DESC;
    fn init_samatra() -> *mut RACE_DESC;
    fn init_sis() -> *mut RACE_DESC;
    fn init_probe() -> *mut RACE_DESC;
    fn log_add(_: log_Level, fmt: *const ::core::ffi::c_char, ...);
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
pub type size_t = usize;
pub type RESOURCE = *const ::core::ffi::c_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub union RESOURCE_DATA {
    pub num: DWORD,
    pub ptr: *mut ::core::ffi::c_void,
    pub str: *const ::core::ffi::c_char,
}
pub type ResourceLoadFun = unsafe extern "C" fn(
    *const ::core::ffi::c_char,
    *mut RESOURCE_DATA,
) -> ();
pub type ResourceFreeFun = unsafe extern "C" fn(*mut ::core::ffi::c_void) -> BOOLEAN;
pub type ResourceStringFun = unsafe extern "C" fn(
    *mut RESOURCE_DATA,
    *mut ::core::ffi::c_char,
    ::core::ffi::c_uint,
) -> ();
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
pub type QUEUE_HANDLE = *mut ::core::ffi::c_void;
pub type HLINK = QUEUE_HANDLE;
pub type HELEMENT = HLINK;
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
pub type RaceDescInitFunc = Option<unsafe extern "C" fn() -> *mut RACE_DESC>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ShipCodeRes(pub ::core::ffi::c_uint);
impl ShipCodeRes {
    pub const ANDROSYN_CODE_RES: Self = Self(0);
    pub const ARILOU_CODE_RES: Self = Self(1);
    pub const BLACKURQ_CODE_RES: Self = Self(2);
    pub const CHENJESU_CODE_RES: Self = Self(3);
    pub const CHMMR_CODE_RES: Self = Self(4);
    pub const DRUUGE_CODE_RES: Self = Self(5);
    pub const HUMAN_CODE_RES: Self = Self(6);
    pub const ILWRATH_CODE_RES: Self = Self(7);
    pub const MELNORME_CODE_RES: Self = Self(8);
    pub const MMRNMHRM_CODE_RES: Self = Self(9);
    pub const MYCON_CODE_RES: Self = Self(10);
    pub const ORZ_CODE_RES: Self = Self(11);
    pub const PKUNK_CODE_RES: Self = Self(12);
    pub const SHOFIXTI_CODE_RES: Self = Self(13);
    pub const SLYLANDR_CODE_RES: Self = Self(14);
    pub const SPATHI_CODE_RES: Self = Self(15);
    pub const SUPOX_CODE_RES: Self = Self(16);
    pub const SYREEN_CODE_RES: Self = Self(17);
    pub const THRADD_CODE_RES: Self = Self(18);
    pub const UMGAH_CODE_RES: Self = Self(19);
    pub const URQUAN_CODE_RES: Self = Self(20);
    pub const UTWIG_CODE_RES: Self = Self(21);
    pub const VUX_CODE_RES: Self = Self(22);
    pub const YEHAT_CODE_RES: Self = Self(23);
    pub const ZOQFOT_CODE_RES: Self = Self(24);
    pub const SAMATRA_CODE_RES: Self = Self(25);
    pub const SIS_CODE_RES: Self = Self(26);
    pub const PROBE_CODE_RES: Self = Self(27);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct log_Level(pub ::core::ffi::c_uint);
impl log_Level {
    pub const log_Nothing: Self = Self(0);
    pub const log_User: Self = Self(1);
    pub const log_Fatal: Self = Self(1);
    pub const log_Error: Self = Self(2);
    pub const log_Warning: Self = Self(3);
    pub const log_Info: Self = Self(4);
    pub const log_Debug: Self = Self(5);
    pub const log_All: Self = Self(6);
}
#[derive(Copy, Clone)]
#[repr(C, align(16))]
pub struct CODERES_STRUCT {
    pub data: RACE_DESC,
}
unsafe extern "C" fn CodeResToInitFunc(mut res: ShipCodeRes) -> RaceDescInitFunc {
    match res {
        ShipCodeRes::ANDROSYN_CODE_RES => {
            return Some(init_androsynth as unsafe extern "C" fn() -> *mut RACE_DESC);
        }
        ShipCodeRes::ARILOU_CODE_RES => {
            return Some(init_arilou as unsafe extern "C" fn() -> *mut RACE_DESC);
        }
        ShipCodeRes::BLACKURQ_CODE_RES => {
            return Some(init_black_urquan as unsafe extern "C" fn() -> *mut RACE_DESC);
        }
        ShipCodeRes::CHENJESU_CODE_RES => {
            return Some(init_chenjesu as unsafe extern "C" fn() -> *mut RACE_DESC);
        }
        ShipCodeRes::CHMMR_CODE_RES => {
            return Some(init_chmmr as unsafe extern "C" fn() -> *mut RACE_DESC);
        }
        ShipCodeRes::DRUUGE_CODE_RES => {
            return Some(init_druuge as unsafe extern "C" fn() -> *mut RACE_DESC);
        }
        ShipCodeRes::HUMAN_CODE_RES => {
            return Some(init_human as unsafe extern "C" fn() -> *mut RACE_DESC);
        }
        ShipCodeRes::ILWRATH_CODE_RES => {
            return Some(init_ilwrath as unsafe extern "C" fn() -> *mut RACE_DESC);
        }
        ShipCodeRes::MELNORME_CODE_RES => {
            return Some(init_melnorme as unsafe extern "C" fn() -> *mut RACE_DESC);
        }
        ShipCodeRes::MMRNMHRM_CODE_RES => {
            return Some(init_mmrnmhrm as unsafe extern "C" fn() -> *mut RACE_DESC);
        }
        ShipCodeRes::MYCON_CODE_RES => {
            return Some(init_mycon as unsafe extern "C" fn() -> *mut RACE_DESC);
        }
        ShipCodeRes::ORZ_CODE_RES => {
            return Some(init_orz as unsafe extern "C" fn() -> *mut RACE_DESC);
        }
        ShipCodeRes::PKUNK_CODE_RES => {
            return Some(init_pkunk as unsafe extern "C" fn() -> *mut RACE_DESC);
        }
        ShipCodeRes::SHOFIXTI_CODE_RES => {
            return Some(init_shofixti as unsafe extern "C" fn() -> *mut RACE_DESC);
        }
        ShipCodeRes::SLYLANDR_CODE_RES => {
            return Some(init_slylandro as unsafe extern "C" fn() -> *mut RACE_DESC);
        }
        ShipCodeRes::SPATHI_CODE_RES => {
            return Some(init_spathi as unsafe extern "C" fn() -> *mut RACE_DESC);
        }
        ShipCodeRes::SUPOX_CODE_RES => {
            return Some(init_supox as unsafe extern "C" fn() -> *mut RACE_DESC);
        }
        ShipCodeRes::SYREEN_CODE_RES => {
            return Some(init_syreen as unsafe extern "C" fn() -> *mut RACE_DESC);
        }
        ShipCodeRes::THRADD_CODE_RES => {
            return Some(init_thraddash as unsafe extern "C" fn() -> *mut RACE_DESC);
        }
        ShipCodeRes::UMGAH_CODE_RES => {
            return Some(init_umgah as unsafe extern "C" fn() -> *mut RACE_DESC);
        }
        ShipCodeRes::URQUAN_CODE_RES => {
            return Some(init_urquan as unsafe extern "C" fn() -> *mut RACE_DESC);
        }
        ShipCodeRes::UTWIG_CODE_RES => {
            return Some(init_utwig as unsafe extern "C" fn() -> *mut RACE_DESC);
        }
        ShipCodeRes::VUX_CODE_RES => {
            return Some(init_vux as unsafe extern "C" fn() -> *mut RACE_DESC);
        }
        ShipCodeRes::YEHAT_CODE_RES => {
            return Some(init_yehat as unsafe extern "C" fn() -> *mut RACE_DESC);
        }
        ShipCodeRes::ZOQFOT_CODE_RES => {
            return Some(init_zoqfotpik as unsafe extern "C" fn() -> *mut RACE_DESC);
        }
        ShipCodeRes::SAMATRA_CODE_RES => {
            return Some(init_samatra as unsafe extern "C" fn() -> *mut RACE_DESC);
        }
        ShipCodeRes::SIS_CODE_RES => {
            return Some(init_sis as unsafe extern "C" fn() -> *mut RACE_DESC);
        }
        ShipCodeRes::PROBE_CODE_RES => {
            return Some(init_probe as unsafe extern "C" fn() -> *mut RACE_DESC);
        }
        _ => {
            log_add(
                log_Level::log_Warning,
                b"Unknown SHIP identifier '%d'\0".as_ptr() as *const ::core::ffi::c_char,
                res.0,
            );
            return None;
        }
    };
}
unsafe extern "C" fn GetCodeResData(
    mut ship_id: *const ::core::ffi::c_char,
    mut resdata: *mut RESOURCE_DATA,
) {
    let mut which_res: BYTE = 0;
    let mut hData: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
        ::core::ffi::c_void,
    >();
    which_res = atoi(ship_id) as BYTE;
    hData = HMalloc(::core::mem::size_of::<CODERES_STRUCT>());
    if !hData.is_null() {
        let mut initFunc: RaceDescInitFunc = CodeResToInitFunc(
            ShipCodeRes(which_res as ::core::ffi::c_uint),
        );
        let mut RDPtr: *mut RACE_DESC = if initFunc.is_none() {
            ::core::ptr::null_mut::<RACE_DESC>()
        } else {
            uqm_lua_checked!(Some(initFunc.expect("non-null function pointer"))
                .expect("non-null function pointer")(), ())
        };
        if RDPtr.is_null() {
            HFree(hData);
            hData = ::core::ptr::null_mut::<::core::ffi::c_void>();
        } else {
            let mut cs: *mut CODERES_STRUCT = ::core::ptr::null_mut::<CODERES_STRUCT>();
            cs = hData as *mut CODERES_STRUCT;
            (*cs).data = *RDPtr;
        }
    }
    (*resdata).ptr = hData;
}
unsafe extern "C" fn _ReleaseCodeResData(mut data: *mut ::core::ffi::c_void) -> BOOLEAN {
    HFree(data);
    return BOOLEAN::TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn InstallCodeResType() -> BOOLEAN {
    return InstallResTypeVectors(
        b"SHIP\0".as_ptr() as *const ::core::ffi::c_char,
        Some(
            GetCodeResData
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_char,
                    *mut RESOURCE_DATA,
                ) -> (),
        ),
        Some(
            _ReleaseCodeResData
                as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> BOOLEAN,
        ),
        None,
    );
}
#[no_mangle]
pub unsafe extern "C" fn LoadCodeResInstance(
    mut res: RESOURCE,
) -> *mut ::core::ffi::c_void {
    let mut hData: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
        ::core::ffi::c_void,
    >();
    hData = uqm_lua_checked!(res_GetResource(res), ::core::ptr::null_mut());
    if !hData.is_null() {
        res_DetachResource(res);
    }
    return hData;
}
#[no_mangle]
pub unsafe extern "C" fn DestroyCodeRes(mut hCode: *mut ::core::ffi::c_void) -> BOOLEAN {
    HFree(hCode);
    return BOOLEAN::TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn CaptureCodeRes(
    mut hCode: *mut ::core::ffi::c_void,
    mut pData: *mut ::core::ffi::c_void,
    mut ppLocData: *mut *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    let mut cs: *mut CODERES_STRUCT = ::core::ptr::null_mut::<CODERES_STRUCT>();
    if hCode.is_null() {
        log_add(
            log_Level::log_Fatal,
            b"dummy.c::CaptureCodeRes() hCode==NULL! FATAL!\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    cs = hCode as *mut CODERES_STRUCT;
    *ppLocData = &raw mut (*cs).data as *mut ::core::ffi::c_void;
    return cs as *mut ::core::ffi::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn ReleaseCodeRes(
    mut CodeRef: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    return CodeRef;
}
