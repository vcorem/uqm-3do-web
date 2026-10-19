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

// Rust translation of src/uqm/loadship.c.
extern "C" {
    pub type string_table_entry;
    pub type string_table;
    pub type frame_desc;
    pub type drawable_desc;
    pub type tfb_soundsample;
    fn DestroyDrawable(Drawable: DRAWABLE) -> BOOLEAN;
    fn LoadGraphicInstance(res: RESOURCE) -> *mut ::core::ffi::c_void;
    fn CaptureDrawable(Drawable: DRAWABLE) -> FRAME;
    fn ReleaseDrawable(Frame: FRAME) -> DRAWABLE;
    fn LoadStringTableInstance(res: RESOURCE) -> STRING_TABLE;
    fn DestroyStringTable(StringTable: STRING_TABLE) -> BOOLEAN;
    fn CaptureStringTable(StringTable: STRING_TABLE) -> STRING;
    fn ReleaseStringTable(String: STRING) -> STRING_TABLE;
    fn LoadSoundInstance(res: RESOURCE) -> SOUND_REF;
    fn LoadMusicInstance(res: RESOURCE) -> MUSIC_REF;
    fn DestroySound(SoundRef: SOUND_REF) -> BOOLEAN;
    fn DestroyMusic(MusicRef: MUSIC_REF) -> BOOLEAN;
    fn LoadCodeResInstance(res: RESOURCE) -> *mut ::core::ffi::c_void;
    fn CaptureCodeRes(
        hCode: *mut ::core::ffi::c_void,
        pData: *mut ::core::ffi::c_void,
        ppLocData: *mut *mut ::core::ffi::c_void,
    ) -> *mut ::core::ffi::c_void;
    fn ReleaseCodeRes(CodeRef: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
    fn DestroyCodeRes(hCode: *mut ::core::ffi::c_void) -> BOOLEAN;
    static mut GlobData: GLOBDATA;
    fn load_animation(
        pixarray: *mut FRAME,
        big_res: RESOURCE,
        med_res: RESOURCE,
        sml_res: RESOURCE,
    ) -> BOOLEAN;
    fn free_image(pixarray: *mut FRAME) -> BOOLEAN;
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
pub type QUEUE_HANDLE = *mut ::core::ffi::c_void;
pub type HLINK = QUEUE_HANDLE;
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
pub type SOUND_REF = STRING_TABLE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GLOBDATA {
    pub SIS_state: SIS_STATE,
    pub Game_state: GAME_STATE,
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
static mut code_resources: [RESOURCE; 29] = [
    ::core::ptr::null::<::core::ffi::c_char>(),
    b"ship.arilou.code\0".as_ptr() as RESOURCE,
    b"ship.chmmr.code\0".as_ptr() as RESOURCE,
    b"ship.earthling.code\0".as_ptr() as RESOURCE,
    b"ship.orz.code\0".as_ptr() as RESOURCE,
    b"ship.pkunk.code\0".as_ptr() as RESOURCE,
    b"ship.shofixti.code\0".as_ptr() as RESOURCE,
    b"ship.spathi.code\0".as_ptr() as RESOURCE,
    b"ship.supox.code\0".as_ptr() as RESOURCE,
    b"ship.thraddash.code\0".as_ptr() as RESOURCE,
    b"ship.utwig.code\0".as_ptr() as RESOURCE,
    b"ship.vux.code\0".as_ptr() as RESOURCE,
    b"ship.yehat.code\0".as_ptr() as RESOURCE,
    b"ship.melnorme.code\0".as_ptr() as RESOURCE,
    b"ship.druuge.code\0".as_ptr() as RESOURCE,
    b"ship.ilwrath.code\0".as_ptr() as RESOURCE,
    b"ship.mycon.code\0".as_ptr() as RESOURCE,
    b"ship.slylandro.code\0".as_ptr() as RESOURCE,
    b"ship.umgah.code\0".as_ptr() as RESOURCE,
    b"ship.urquan.code\0".as_ptr() as RESOURCE,
    b"ship.zoqfotpik.code\0".as_ptr() as RESOURCE,
    b"ship.syreen.code\0".as_ptr() as RESOURCE,
    b"ship.kohrah.code\0".as_ptr() as RESOURCE,
    b"ship.androsynth.code\0".as_ptr() as RESOURCE,
    b"ship.chenjesu.code\0".as_ptr() as RESOURCE,
    b"ship.mmrnmhrm.code\0".as_ptr() as RESOURCE,
    b"ship.flagship.code\0".as_ptr() as RESOURCE,
    b"ship.samatra.code\0".as_ptr() as RESOURCE,
    b"ship.drone.code\0".as_ptr() as RESOURCE,
];
#[no_mangle]
pub unsafe extern "C" fn load_ship(
    mut SpeciesID: SPECIES_ID,
    mut LoadBattleData: BOOLEAN,
) -> *mut RACE_DESC {
    let mut RDPtr: *mut RACE_DESC = ::core::ptr::null_mut::<RACE_DESC>();
    let mut CodeRef: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
        ::core::ffi::c_void,
    >();
    if SpeciesID.0 >= SPECIES_ID::NUM_SPECIES_ID.0 {
        return ::core::ptr::null_mut::<RACE_DESC>();
    }
    CodeRef = CaptureCodeRes(
        LoadCodeResInstance(code_resources[SpeciesID.0 as usize]),
        &raw mut GlobData as *mut ::core::ffi::c_void,
        &raw mut RDPtr as *mut *mut ::core::ffi::c_void,
    );
    '_ExitFunc: {
        '_BadLoad: {
            if !CodeRef.is_null() {
                (*RDPtr).CodeRef = CodeRef;
                if !(*RDPtr).ship_info.icons_rsc.is_null() {
                    (*RDPtr).ship_info.icons = CaptureDrawable(
                        LoadGraphicInstance((*RDPtr).ship_info.icons_rsc) as DRAWABLE,
                    );
                    (*RDPtr).ship_info.icons.is_null();
                }
                if !(*RDPtr).ship_info.melee_icon_rsc.is_null() {
                    (*RDPtr).ship_info.melee_icon = CaptureDrawable(
                        LoadGraphicInstance((*RDPtr).ship_info.melee_icon_rsc)
                            as DRAWABLE,
                    );
                    (*RDPtr).ship_info.melee_icon.is_null();
                }
                if !(*RDPtr).ship_info.race_strings_rsc.is_null() {
                    (*RDPtr).ship_info.race_strings = CaptureStringTable(
                        LoadStringTableInstance((*RDPtr).ship_info.race_strings_rsc),
                    );
                    (*RDPtr).ship_info.race_strings.is_null();
                }
                if LoadBattleData.0 != 0 {
                    let mut RawPtr: *mut DATA_STUFF = &raw mut (*RDPtr).ship_data;
                    if load_animation(
                            &raw mut (*RawPtr).ship as *mut FRAME,
                            (*RawPtr).ship_rsc[0usize],
                            (*RawPtr).ship_rsc[1usize],
                            (*RawPtr).ship_rsc[2usize],
                        )
                        .0 != 0
                    {
                        if !(*RawPtr).weapon_rsc[0usize].is_null() {
                            if load_animation(
                                    &raw mut (*RawPtr).weapon as *mut FRAME,
                                    (*RawPtr).weapon_rsc[0usize],
                                    (*RawPtr).weapon_rsc[1usize],
                                    (*RawPtr).weapon_rsc[2usize],
                                )
                                .0 == 0
                            {
                                break '_BadLoad;
                            }
                        }
                        if !(*RawPtr).special_rsc[0usize].is_null() {
                            if load_animation(
                                    &raw mut (*RawPtr).special as *mut FRAME,
                                    (*RawPtr).special_rsc[0usize],
                                    (*RawPtr).special_rsc[1usize],
                                    (*RawPtr).special_rsc[2usize],
                                )
                                .0 == 0
                            {
                                break '_BadLoad;
                            }
                        }
                        if !(*RawPtr).captain_control.captain_rsc.is_null() {
                            (*RawPtr).captain_control.background = CaptureDrawable(
                                LoadGraphicInstance((*RawPtr).captain_control.captain_rsc)
                                    as DRAWABLE,
                            );
                            if (*RawPtr).captain_control.background.is_null() {
                                break '_BadLoad;
                            }
                        }
                        if !(*RawPtr).victory_ditty_rsc.is_null() {
                            (*RawPtr).victory_ditty = LoadMusicInstance(
                                (*RawPtr).victory_ditty_rsc,
                            );
                            if (*RawPtr).victory_ditty.is_null() {
                                break '_BadLoad;
                            }
                        }
                        if !(*RawPtr).ship_sounds_rsc.is_null() {
                            (*RawPtr).ship_sounds = CaptureStringTable(
                                LoadSoundInstance((*RawPtr).ship_sounds_rsc),
                            ) as SOUND;
                            if !(*RawPtr).ship_sounds.is_null() {
                                break '_ExitFunc;
                            }
                        } else {
                            break '_ExitFunc;
                        }
                    }
                } else {
                    break '_ExitFunc;
                }
            }
        }
        if !CodeRef.is_null() {
            DestroyCodeRes(ReleaseCodeRes(CodeRef));
        }
        RDPtr = ::core::ptr::null_mut::<RACE_DESC>();
    }
    return RDPtr;
}
#[no_mangle]
pub unsafe extern "C" fn free_ship(
    mut raceDescPtr: *mut RACE_DESC,
    mut FreeIconData: BOOLEAN,
    mut FreeBattleData: BOOLEAN,
) {
    if (*raceDescPtr).uninit_func.is_some() {
        Some((*raceDescPtr).uninit_func.expect("non-null function pointer"))
            .expect("non-null function pointer")(raceDescPtr);
    }
    if FreeBattleData.0 != 0 {
        let mut shipData: *mut DATA_STUFF = &raw mut (*raceDescPtr).ship_data;
        free_image(&raw mut (*shipData).special as *mut FRAME);
        free_image(&raw mut (*shipData).weapon as *mut FRAME);
        free_image(&raw mut (*shipData).ship as *mut FRAME);
        DestroyDrawable(ReleaseDrawable((*shipData).captain_control.background));
        DestroyMusic((*shipData).victory_ditty);
        DestroySound(ReleaseStringTable((*shipData).ship_sounds));
    }
    if FreeIconData.0 != 0 {
        let mut shipInfo: *mut SHIP_INFO = &raw mut (*raceDescPtr).ship_info;
        DestroyDrawable(ReleaseDrawable((*shipInfo).melee_icon));
        DestroyDrawable(ReleaseDrawable((*shipInfo).icons));
        DestroyStringTable(ReleaseStringTable((*shipInfo).race_strings));
    }
    DestroyCodeRes(ReleaseCodeRes((*raceDescPtr).CodeRef));
}
