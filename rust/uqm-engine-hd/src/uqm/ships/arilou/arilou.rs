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

// Rust translation of src/uqm/ships/arilou/arilou.c.
extern "C" {
    pub type string_table_entry;
    pub type frame_desc;
    pub type tfb_soundsample;
    fn sqrt(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn SetAbsFrameIndex(Frame: FRAME, FrameIndex: COUNT) -> FRAME;
    fn IncFrameIndex(Frame: FRAME) -> FRAME;
    fn DecFrameIndex(Frame: FRAME) -> FRAME;
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
    fn PlotIntercept(
        ElementPtr0: *mut ELEMENT,
        ElementPtr1: *mut ELEMENT,
        max_turns: COUNT,
        margin_of_error: COUNT,
    ) -> COUNT;
    static mut GlobData: GLOBDATA;
    fn TFB_Random() -> DWORD;
    fn ProcessSound(Sound: SOUND, PositionalObject: *mut ELEMENT);
    fn DeltaEnergy(ElementPtr: *mut ELEMENT, energy_delta: SIZE) -> BOOLEAN;
    fn ship_intelligence(
        ShipPtr: *mut ELEMENT,
        ObjectsOfConcern: *mut EVALUATE_DESC,
        ConcernCounter: COUNT,
    );
    fn initialize_laser(pLaserBlock: *mut LASER_BLOCK) -> HELEMENT;
    fn TrackShip(Tracker: *mut ELEMENT, pfacing: *mut COUNT) -> SIZE;
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_5(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_5 {
    pub const SUPER_MELEE: Self = Self(0);
    pub const IN_LAST_BATTLE: Self = Self(1);
    pub const IN_ENCOUNTER: Self = Self(2);
    pub const IN_HYPERSPACE: Self = Self(3);
    pub const IN_INTERPLANETARY: Self = Self(4);
    pub const WON_LAST_BATTLE: Self = Self(5);
    pub const IN_QUASISPACE: Self = Self(6);
    pub const IN_PLANET_ORBIT: Self = Self(7);
    pub const IN_STARBASE: Self = Self(8);
    pub const CHECK_PAUSE: Self = Self(256);
    pub const IN_BATTLE: Self = Self(512);
    pub const START_ENCOUNTER: Self = Self(1024);
    pub const START_INTERPLANETARY: Self = Self(2048);
    pub const CHECK_LOAD: Self = Self(4096);
    pub const CHECK_RESTART: Self = Self(8192);
    pub const CHECK_ABORT: Self = Self(16384);
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_6(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_6 {
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
static mut arilou_desc: RACE_DESC = unsafe {
    race_desc {
        c2rust_padding: [0; 8],
        c2rust_padding_0: [0; 10],
        c2rust_padding_1: [0; 2],
        c2rust_padding_2: [0; 8],
        ship_info: SHIP_INFO {
            idStr: b"skiff\0".as_ptr() as *const ::core::ffi::c_char,
            ship_flags: ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as UWORD,
            ship_cost: 16 as BYTE,
            crew_level: 6 as COUNT,
            max_crew: 6 as COUNT,
            energy_level: 20 as BYTE,
            max_energy: 20 as BYTE,
            race_strings_rsc: b"ship.arilou.text\0".as_ptr() as RESOURCE,
            icons_rsc: b"ship.arilou.icons\0".as_ptr() as RESOURCE,
            melee_icon_rsc: b"ship.arilou.meleeicons\0".as_ptr() as RESOURCE,
            race_strings: ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>(),
            icons: ::core::ptr::null_mut::<FRAME_DESC>(),
            melee_icon: ::core::ptr::null_mut::<FRAME_DESC>(),
        },
        fleet: FLEET_STUFF {
            strength: (250 as ::core::ffi::c_int / 11 as ::core::ffi::c_int
                * 2 as ::core::ffi::c_int) as COUNT,
            known_loc: point {
                x: 438 as COORD,
                y: 6372 as COORD,
            },
        },
        characteristics: CHARACTERISTIC_STUFF {
            max_thrust: 40 as COUNT,
            thrust_increment: 40 as COUNT,
            energy_regeneration: 1 as BYTE,
            weapon_energy_cost: 2 as BYTE,
            special_energy_cost: 3 as BYTE,
            energy_wait: 6 as BYTE,
            turn_wait: 0 as BYTE,
            thrust_wait: 0 as BYTE,
            weapon_wait: 1 as BYTE,
            special_wait: 2 as BYTE,
            ship_mass: 1 as BYTE,
        },
        ship_data: DATA_STUFF {
            ship_rsc: [
                b"ship.arilou.graphics.skiff.large\0".as_ptr() as RESOURCE,
                b"ship.arilou.graphics.skiff.medium\0".as_ptr() as RESOURCE,
                b"ship.arilou.graphics.skiff.small\0".as_ptr() as RESOURCE,
            ],
            weapon_rsc: [
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
            ],
            special_rsc: [
                b"ship.arilou.graphics.warp.large\0".as_ptr() as RESOURCE,
                b"ship.arilou.graphics.warp.medium\0".as_ptr() as RESOURCE,
                b"ship.arilou.graphics.warp.small\0".as_ptr() as RESOURCE,
            ],
            captain_control: captain_stuff {
                captain_rsc: b"ship.arilou.graphics.captain\0".as_ptr() as RESOURCE,
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
            victory_ditty_rsc: b"ship.arilou.ditty\0".as_ptr() as RESOURCE,
            ship_sounds_rsc: b"ship.arilou.sounds\0".as_ptr() as RESOURCE,
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
            WeaponRange: ((100 as ::core::ffi::c_int + 9 as ::core::ffi::c_int)
                << 2 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as COUNT,
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
unsafe extern "C" fn initialize_autoaim_laser(
    mut ShipPtr: *mut ELEMENT,
    mut LaserArray: *mut HELEMENT,
) -> COUNT {
    let mut orig_facing: COUNT = 0;
    let mut delta_facing: SIZE = 0;
    let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
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
    let mut LaserRange: COUNT = (if if resolutionFactor != 2 as ::core::ffi::c_uint {
        BOOLEAN::FALSE.0 as ::core::ffi::c_int
    } else {
        BOOLEAN::TRUE.0 as ::core::ffi::c_int
    } == 0
    {
        (100 as ::core::ffi::c_int + 9 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
    } else {
        ((100 as ::core::ffi::c_int + 9 as ::core::ffi::c_int)
            << 2 as ::core::ffi::c_int) << resolutionFactor
    }) as COUNT;
    StarShipPtr = (*ShipPtr).pParent as *mut STARSHIP;
    orig_facing = (*StarShipPtr).ShipFacing;
    LaserBlock.face = orig_facing;
    delta_facing = TrackShip(ShipPtr, &raw mut LaserBlock.face);
    if delta_facing as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
        LaserBlock.face = (orig_facing as ::core::ffi::c_int
            + delta_facing as ::core::ffi::c_int
            & ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                - 1 as ::core::ffi::c_int) as DWORD as COUNT;
    }
    (*ShipPtr).hTarget = ::core::ptr::null_mut::<::core::ffi::c_void>();
    LaserBlock.cx = (*ShipPtr).next.location.x;
    LaserBlock.cy = (*ShipPtr).next.location.y;
    LaserBlock.ex = (*(&raw mut sinetab as *mut SDWORD)
        .offset(
            (((LaserBlock.face as ::core::ffi::c_int)
                << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int)
                + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                    >> 2 as ::core::ffi::c_int)
                & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int) as DWORD as isize,
        ) as ::core::ffi::c_long * LaserRange as ::core::ffi::c_long
        >> 14 as ::core::ffi::c_int) as SDWORD as COORD;
    LaserBlock.ey = (*(&raw mut sinetab as *mut SDWORD)
        .offset(
            ((LaserBlock.face as ::core::ffi::c_int)
                << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int) as DWORD as isize,
        ) as ::core::ffi::c_long * LaserRange as ::core::ffi::c_long
        >> 14 as ::core::ffi::c_int) as SDWORD as COORD;
    LaserBlock.sender = (*ShipPtr).playerNr;
    LaserBlock.flags = ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
        as ELEMENT_FLAGS;
    LaserBlock.pixoffs = ((9 as ::core::ffi::c_int) << resolutionFactor) as SIZE;
    LaserBlock.color = colorFromRgb15(0x1f as BYTE, 0x1f as BYTE, 0xa as BYTE);
    *LaserArray.offset(0isize) = initialize_laser(&raw mut LaserBlock);
    return 1 as COUNT;
}
unsafe extern "C" fn arilou_intelligence(
    mut ShipPtr: *mut ELEMENT,
    mut ObjectsOfConcern: *mut EVALUATE_DESC,
    mut ConcernCounter: COUNT,
) {
    let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
    StarShipPtr = (*ShipPtr).pParent as *mut STARSHIP;
    (*StarShipPtr).ship_input_state = ((*StarShipPtr).ship_input_state
        as ::core::ffi::c_int | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
        as BYTE;
    (*ObjectsOfConcern
        .offset(C2Rust_Unnamed_6::ENEMY_SHIP_INDEX.0 as ::core::ffi::c_int as isize))
        .MoveState = MOVEMENT_STATE::ENTICE;
    uqm_lua_checked!(ship_intelligence(ShipPtr, ObjectsOfConcern, ConcernCounter), ());
    if (*StarShipPtr).special_counter as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        let mut lpEvalDesc: *mut EVALUATE_DESC = ::core::ptr::null_mut::<
            EVALUATE_DESC,
        >();
        (*StarShipPtr).ship_input_state = ((*StarShipPtr).ship_input_state
            as ::core::ffi::c_int
            & !((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)) as BYTE;
        lpEvalDesc = ObjectsOfConcern
            .offset(
                C2Rust_Unnamed_6::ENEMY_WEAPON_INDEX.0 as ::core::ffi::c_int as isize,
            );
        if !(*lpEvalDesc).ObjectPtr.is_null()
            && (*lpEvalDesc).which_turn as ::core::ffi::c_int <= 6 as ::core::ffi::c_int
        {
            let mut IsTrackingWeapon: BOOLEAN = BOOLEAN::FALSE;
            let mut EnemyStarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<
                STARSHIP,
            >();
            EnemyStarShipPtr = (*(*lpEvalDesc).ObjectPtr).pParent as *mut STARSHIP;
            if (*(*EnemyStarShipPtr).RaceDescPtr).ship_info.ship_flags
                as ::core::ffi::c_int
                & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int != 0
                && (*(*lpEvalDesc).ObjectPtr).next.image.farray
                    == &raw mut (*(*EnemyStarShipPtr).RaceDescPtr).ship_data.weapon
                        as *mut FRAME
                || (*(*EnemyStarShipPtr).RaceDescPtr).ship_info.ship_flags
                    as ::core::ffi::c_int
                    & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int != 0
                    && (*(*lpEvalDesc).ObjectPtr).next.image.farray
                        == &raw mut (*(*EnemyStarShipPtr).RaceDescPtr).ship_data.special
                            as *mut FRAME
            {
                IsTrackingWeapon = BOOLEAN::TRUE;
            } else {
                IsTrackingWeapon = BOOLEAN::FALSE;
            }
            if ((*(*lpEvalDesc).ObjectPtr).state_flags as ::core::ffi::c_int
                & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int != 0
                || IsTrackingWeapon.0 != 0
                    && ((*lpEvalDesc).which_turn as ::core::ffi::c_int
                        == 1 as ::core::ffi::c_int
                        || (*(*lpEvalDesc).ObjectPtr).state_flags as ::core::ffi::c_int
                            & (1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int != 0)
                || uqm_lua_checked!(PlotIntercept(
                    (*lpEvalDesc).ObjectPtr,
                    ShipPtr,
                    3 as COUNT,
                    0 as COUNT,
                ), ()) as ::core::ffi::c_int != 0) && TFB_Random() & 3 as DWORD == 0
            {
                (*StarShipPtr).ship_input_state = ((*StarShipPtr).ship_input_state
                    as ::core::ffi::c_int
                    & !((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                        | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                        | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
                        | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)) as BYTE;
                (*StarShipPtr).ship_input_state = ((*StarShipPtr).ship_input_state
                    as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as BYTE;
            }
        }
    }
    if (*(*StarShipPtr).RaceDescPtr).ship_info.energy_level as ::core::ffi::c_int
        <= (3 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
    {
        (*StarShipPtr).ship_input_state = ((*StarShipPtr).ship_input_state
            as ::core::ffi::c_int
            & !((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)) as BYTE;
    }
}
unsafe extern "C" fn arilou_preprocess(mut ElementPtr: *mut ELEMENT) {
    let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
    StarShipPtr = (*ElementPtr).pParent as *mut STARSHIP;
    if (*ElementPtr).state_flags as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int == 0
    {
        if (*ElementPtr).c2rust_unnamed_3.thrust_wait as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
            memset(
                &raw mut (*ElementPtr).velocity as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<VELOCITY_DESC>(),
            );
            (*StarShipPtr).cur_status_flags = ((*StarShipPtr).cur_status_flags
                as ::core::ffi::c_int
                & !((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int))
                as STATUS_FLAGS;
        }
        if (*StarShipPtr).cur_status_flags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int != 0
            && (*StarShipPtr).special_counter as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int && uqm_lua_checked!(DeltaEnergy(ElementPtr, -3 as SIZE), ()).0 != 0
        {
            memset(
                &raw mut (*ElementPtr).velocity as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<VELOCITY_DESC>(),
            );
            (*StarShipPtr).cur_status_flags = ((*StarShipPtr).cur_status_flags
                as ::core::ffi::c_int
                & !((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int))
                as STATUS_FLAGS;
            (*ElementPtr).state_flags = ((*ElementPtr).state_flags as ::core::ffi::c_int
                | ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int))
                as ELEMENT_FLAGS;
            (*ElementPtr).c2rust_unnamed.life_span = 5 as COUNT;
            (*ElementPtr).next.image.farray = &raw mut (*(*StarShipPtr).RaceDescPtr)
                .ship_data
                .special as *mut FRAME;
            (*ElementPtr).next.image.frame = (*(*StarShipPtr).RaceDescPtr)
                .ship_data
                .special[0usize];
            ProcessSound(
                SetAbsStringTableIndex(
                    (*(*StarShipPtr).RaceDescPtr).ship_data.ship_sounds,
                    1 as COUNT,
                ),
                ElementPtr,
            );
            (*StarShipPtr).special_counter = (*(*StarShipPtr).RaceDescPtr)
                .characteristics
                .special_wait;
        }
    } else if (*ElementPtr).next.image.farray
        == &raw mut (*(*StarShipPtr).RaceDescPtr).ship_data.special as *mut FRAME
    {
        let mut life_span: COUNT = 0;
        (*StarShipPtr).cur_status_flags = ((*StarShipPtr).cur_status_flags
            as ::core::ffi::c_int
            & !((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
            | (*StarShipPtr).old_status_flags as ::core::ffi::c_int
                & ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int))
            as STATUS_FLAGS;
        (*StarShipPtr).weapon_counter = (*StarShipPtr).weapon_counter.wrapping_add(1);
        (*StarShipPtr).special_counter = (*StarShipPtr).special_counter.wrapping_add(1);
        (*StarShipPtr).energy_counter = (*StarShipPtr).energy_counter.wrapping_add(1);
        (*ElementPtr).c2rust_unnamed_2.turn_wait = (*ElementPtr)
            .c2rust_unnamed_2
            .turn_wait
            .wrapping_add(1);
        (*ElementPtr).c2rust_unnamed_3.thrust_wait = (*ElementPtr)
            .c2rust_unnamed_3
            .thrust_wait
            .wrapping_add(1);
        life_span = (*ElementPtr).c2rust_unnamed.life_span;
        if life_span as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (*ElementPtr).state_flags = ((*ElementPtr).state_flags as ::core::ffi::c_int
                & !((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int))
                as ELEMENT_FLAGS;
            (*ElementPtr).state_flags = ((*ElementPtr).state_flags as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as ELEMENT_FLAGS;
            (*ElementPtr).next.image.farray = &raw mut (*(*StarShipPtr).RaceDescPtr)
                .ship_data
                .ship as *mut FRAME;
            (*ElementPtr).current.image.farray = (*ElementPtr).next.image.farray;
            (*ElementPtr).next.image.frame = SetAbsFrameIndex(
                (*(*StarShipPtr).RaceDescPtr).ship_data.ship[0usize],
                (*StarShipPtr).ShipFacing,
            );
            (*ElementPtr).current.image.frame = (*ElementPtr).next.image.frame;
            (*ElementPtr).IntersectControl.IntersectStamp.origin.x = ((*ElementPtr)
                .current
                .location
                .x as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as COORD;
            (*ElementPtr).IntersectControl.IntersectStamp.origin.y = ((*ElementPtr)
                .current
                .location
                .y as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as COORD;
        } else {
            life_span = life_span.wrapping_sub(1);
            if life_span as ::core::ffi::c_int != 2 as ::core::ffi::c_int {
                if (life_span as ::core::ffi::c_int) < 2 as ::core::ffi::c_int {
                    (*ElementPtr).next.image.frame = DecFrameIndex(
                        (*ElementPtr).next.image.frame,
                    );
                } else {
                    (*ElementPtr).next.image.frame = IncFrameIndex(
                        (*ElementPtr).next.image.frame,
                    );
                }
            } else if GlobData.Game_state.CurrentActivity as BYTE as ::core::ffi::c_int
                == C2Rust_Unnamed_5::IN_LAST_BATTLE.0 as ::core::ffi::c_int
            {
                let mut dist: SDWORD = 0 as SDWORD;
                let mut dx: SDWORD = 0;
                let mut dy: SDWORD = 0;
                loop {
                    (*ElementPtr).next.location.x = (if TFB_Random()
                        .wrapping_rem(
                            (((CanvasWidth
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
                                as DWORD,
                        ) as ::core::ffi::c_int
                        & !(((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                            - 1 as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
                    {
                        (TFB_Random()
                            .wrapping_rem(
                                (((CanvasWidth
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
                                    as DWORD,
                            ) as ::core::ffi::c_int
                            & !(((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                                - 1 as ::core::ffi::c_int))
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
                    } else if TFB_Random()
                        .wrapping_rem(
                            (((CanvasWidth
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
                                as DWORD,
                        ) as ::core::ffi::c_int
                        & !(((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                            - 1 as ::core::ffi::c_int)
                        >= ((CanvasWidth
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
                            << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    {
                        (TFB_Random()
                            .wrapping_rem(
                                (((CanvasWidth
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
                                    as DWORD,
                            ) as ::core::ffi::c_int
                            & !(((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                                - 1 as ::core::ffi::c_int))
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
                        TFB_Random()
                            .wrapping_rem(
                                (((CanvasWidth
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
                                    as DWORD,
                            ) as ::core::ffi::c_int
                            & !(((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                                - 1 as ::core::ffi::c_int)
                    }) as DWORD as COORD;
                    (*ElementPtr).next.location.y = (if TFB_Random()
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
                                << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                                as DWORD,
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
                                    << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                                    as DWORD,
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
                                << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                                as DWORD,
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
                                    << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                                    as DWORD,
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
                                    << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                                    as DWORD,
                            ) as ::core::ffi::c_int
                            & !(((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                                - 1 as ::core::ffi::c_int)
                    }) as DWORD as COORD;
                    dx = ((*ElementPtr).next.location.x as ::core::ffi::c_int
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
                            << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                            >> 1 as ::core::ffi::c_int)) as SDWORD;
                    dy = ((*ElementPtr).next.location.y as ::core::ffi::c_int
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
                            << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                            >> 1 as ::core::ffi::c_int)) as SDWORD;
                    dist = sqrt((dx * dx + dy * dy) as ::core::ffi::c_double) as SDWORD;
                    if dist >= (2800 as ::core::ffi::c_int) << resolutionFactor {
                        break;
                    }
                }
            } else {
                (*ElementPtr).next.location.x = (if TFB_Random()
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
                            << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                            as DWORD,
                    ) as ::core::ffi::c_int
                    & !(((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
                {
                    (TFB_Random()
                        .wrapping_rem(
                            (((CanvasWidth
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
                                as DWORD,
                        ) as ::core::ffi::c_int
                        & !(((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                            - 1 as ::core::ffi::c_int))
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
                            << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                            as DWORD,
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
                            (((CanvasWidth
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
                                as DWORD,
                        ) as ::core::ffi::c_int
                        & !(((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                            - 1 as ::core::ffi::c_int))
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
                    TFB_Random()
                        .wrapping_rem(
                            (((CanvasWidth
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
                                as DWORD,
                        ) as ::core::ffi::c_int
                        & !(((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                            - 1 as ::core::ffi::c_int)
                }) as DWORD as COORD;
                (*ElementPtr).next.location.y = (if TFB_Random()
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
                            << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                            as DWORD,
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
                                << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                                as DWORD,
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
                            << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                            as DWORD,
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
                                << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                                as DWORD,
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
                                << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                                as DWORD,
                        ) as ::core::ffi::c_int
                        & !(((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                            - 1 as ::core::ffi::c_int)
                }) as DWORD as COORD;
            }
        }
        (*ElementPtr).state_flags = ((*ElementPtr).state_flags as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as ELEMENT_FLAGS;
    }
}
#[no_mangle]
pub unsafe extern "C" fn init_arilou() -> *mut RACE_DESC {
    let mut RaceDescPtr: *mut RACE_DESC = ::core::ptr::null_mut::<RACE_DESC>();
    if if resolutionFactor != 2 as ::core::ffi::c_uint {
        BOOLEAN::FALSE.0 as ::core::ffi::c_int
    } else {
        BOOLEAN::TRUE.0 as ::core::ffi::c_int
    } != 0
    {
        arilou_desc.characteristics.max_thrust = ((40 as ::core::ffi::c_int)
            << resolutionFactor) as COUNT;
        arilou_desc.characteristics.thrust_increment = ((40 as ::core::ffi::c_int)
            << resolutionFactor) as COUNT;
        arilou_desc.cyborg_control.WeaponRange = (((100 as ::core::ffi::c_int
            + 9 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) << resolutionFactor
            >> 1 as ::core::ffi::c_int) as COUNT;
    } else {
        arilou_desc.characteristics.max_thrust = 40 as COUNT;
        arilou_desc.characteristics.thrust_increment = 40 as COUNT;
        arilou_desc.cyborg_control.WeaponRange = ((100 as ::core::ffi::c_int
            + 9 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
            >> 1 as ::core::ffi::c_int) as COUNT;
    }
    arilou_desc.preprocess_func = Some(
        arilou_preprocess as unsafe extern "C" fn(*mut ELEMENT) -> (),
    ) as Option<PREPROCESS_FUNC>;
    arilou_desc.init_weapon_func = Some(
        initialize_autoaim_laser
            as unsafe extern "C" fn(*mut ELEMENT, *mut HELEMENT) -> COUNT,
    ) as Option<INIT_WEAPON_FUNC>;
    arilou_desc.cyborg_control.intelligence_func = Some(
        arilou_intelligence
            as unsafe extern "C" fn(*mut ELEMENT, *mut EVALUATE_DESC, COUNT) -> (),
    ) as Option<IntelligenceFunc>;
    RaceDescPtr = &raw mut arilou_desc;
    return RaceDescPtr;
}
