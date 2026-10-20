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

// Rust translation of src/uqm/init.c.
extern "C" {
    pub type string_table_entry;
    pub type context_desc;
    pub type frame_desc;
    pub type drawable_desc;
    pub type tfb_soundsample;
    fn SetContext(Context: CONTEXT) -> CONTEXT;
    fn SetContextBackGroundColor(Color: Color) -> Color;
    fn SetContextFGFrame(Frame: FRAME) -> FRAME;
    fn SetContextClipRect(pRect: *mut RECT) -> BOOLEAN;
    fn ClearDrawable();
    fn DestroyDrawable(Drawable: DRAWABLE) -> BOOLEAN;
    fn LoadGraphicInstance(res: RESOURCE) -> *mut ::core::ffi::c_void;
    fn CaptureDrawable(Drawable: DRAWABLE) -> FRAME;
    fn ReleaseDrawable(Frame: FRAME) -> DRAWABLE;
    fn __assert_fail(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    ) -> !;
    static mut resolutionFactor: ::core::ffi::c_uint;
    static mut optRequiresReload: BOOLEAN;
    static mut optWindowType: DWORD;
    static mut contentDirPath: *mut ::core::ffi::c_char;
    static mut addonDirPath: *mut ::core::ffi::c_char;
    fn prepareContentDir(
        contentDirName: *const ::core::ffi::c_char,
        addonDirName: *const ::core::ffi::c_char,
        execFile: *const ::core::ffi::c_char,
    );
    static mut CanvasWidth: ::core::ffi::c_int;
    static mut CanvasHeight: ::core::ffi::c_int;
    fn ReinitQueue(pq: *mut QUEUE);
    fn StopSound();
    static mut disp_q: QUEUE;
    fn spawn_planet();
    fn spawn_asteroid(ElementPtr: *mut ELEMENT);
    fn InitGalaxy();
    fn InitStarOffsets();
    fn UninitStarOffsets();
    fn TaskSwitch();
    static mut GlobData: GLOBDATA;
    fn inHQSpace() -> BOOLEAN;
    fn UninitGameStructures();
    fn Build(pQueue: *mut QUEUE, SpeciesID: SPECIES_ID) -> HLINK;
    fn free_ship(
        RaceDescPtr: *mut RACE_DESC,
        FreeIconData: BOOLEAN,
        FreeBattleData: BOOLEAN,
    );
    fn free_gravity_well();
    fn new_ship(ElementPtr: *mut ELEMENT);
    fn FleetIsInfinite(playerNr: COUNT) -> BOOLEAN;
    fn UpdateShipFragCrew(_: *mut STARSHIP);
    fn InitDisplayList();
    static mut Screen: FRAME;
    static mut ScreenContext: CONTEXT;
    static mut SpaceContext: CONTEXT;
    static mut race_q: [QUEUE; 0];
    fn InitGameKernel() -> BOOLEAN;
    fn UninitGameKernel();
    fn LoadKernel(
        argc: ::core::ffi::c_int,
        argv: *mut *mut ::core::ffi::c_char,
    ) -> BOOLEAN;
    fn ClearPlayerInputAll();
    fn LoadHyperspace() -> BOOLEAN;
    fn FreeHyperspace() -> BOOLEAN;
    fn LoadMasterShipList(YieldProcessing: Option<unsafe extern "C" fn() -> ()>);
    fn FreeMasterShipList();
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
pub type STRING = *mut STRING_TABLE_ENTRY_DESC;
pub type CONTEXT_DESC = context_desc;
pub type FRAME_DESC = frame_desc;
pub type DRAWABLE_DESC = drawable_desc;
pub type CONTEXT = *mut CONTEXT_DESC;
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
    pub flags: BYTE,
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
pub type HELEMENT = HLINK;
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
pub type HSTARSHIP = HLINK;
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
#[inline]
unsafe extern "C" fn LockStarShip(
    mut pq: *const QUEUE,
    mut h: HSTARSHIP,
) -> *mut STARSHIP {
    ((*pq).object_size as usize == ::core::mem::size_of::<STARSHIP>()
        || {
            __assert_fail(
                b"GetLinkSize (pq) == sizeof (STARSHIP)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/uqm/races.h\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                304 as ::core::ffi::c_int,
                b"LockStarShip\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    return LockLink(pq, h) as *mut STARSHIP;
}
#[no_mangle]
pub static mut stars_in_space: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
#[no_mangle]
pub static mut stars_misc: [FRAME; 3] = [::core::ptr::null_mut::<FRAME_DESC>(); 3];
#[no_mangle]
pub static mut scenery: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
#[no_mangle]
pub static mut asteroid: [FRAME; 3] = [::core::ptr::null_mut::<FRAME_DESC>(); 3];
#[no_mangle]
pub static mut blast: [FRAME; 3] = [::core::ptr::null_mut::<FRAME_DESC>(); 3];
#[no_mangle]
pub static mut explosion: [FRAME; 3] = [::core::ptr::null_mut::<FRAME_DESC>(); 3];
#[no_mangle]
pub unsafe extern "C" fn load_animation(
    mut pixarray: *mut FRAME,
    mut big_res: RESOURCE,
    mut med_res: RESOURCE,
    mut sml_res: RESOURCE,
) -> BOOLEAN {
    let mut d: DRAWABLE = ::core::ptr::null_mut::<DRAWABLE_DESC>();
    d = uqm_lua_checked!(LoadGraphicInstance(big_res), BOOLEAN(0)) as DRAWABLE;
    if d.is_null() {
        return BOOLEAN::FALSE;
    }
    *pixarray.offset(0isize) = CaptureDrawable(d);
    if !med_res.is_null() {
        d = uqm_lua_checked!(LoadGraphicInstance(med_res), BOOLEAN(0)) as DRAWABLE;
        if d.is_null() {
            return BOOLEAN::FALSE;
        }
    }
    *pixarray.offset(1isize) = CaptureDrawable(d);
    if !sml_res.is_null() {
        d = uqm_lua_checked!(LoadGraphicInstance(sml_res), BOOLEAN(0)) as DRAWABLE;
        if d.is_null() {
            return BOOLEAN::FALSE;
        }
    }
    *pixarray.offset(2isize) = CaptureDrawable(d);
    return BOOLEAN::TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn free_image(mut pixarray: *mut FRAME) -> BOOLEAN {
    let mut retval: BOOLEAN = BOOLEAN::FALSE;
    let mut i: COUNT = 0;
    let mut j: COUNT = 0;
    let mut already_freed: [*mut ::core::ffi::c_void; 3] = [::core::ptr::null_mut::<
        ::core::ffi::c_void,
    >(); 3];
    retval = BOOLEAN::TRUE;
    i = 0 as COUNT;
    while (i as ::core::ffi::c_int) < 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int {
        if !(*pixarray.offset(i as isize)).is_null() {
            let mut ok: BOOLEAN = BOOLEAN::TRUE;
            j = 0 as COUNT;
            while (j as ::core::ffi::c_int) < i as ::core::ffi::c_int {
                if already_freed[j as usize]
                    == *pixarray.offset(i as isize) as *mut ::core::ffi::c_void
                {
                    ok = BOOLEAN::FALSE;
                    break;
                } else {
                    j = j.wrapping_add(1);
                }
            }
            if ok.0 != 0 {
                if DestroyDrawable(ReleaseDrawable(*pixarray.offset(i as isize))).0 == 0
                {
                    retval = BOOLEAN::FALSE;
                }
            }
            already_freed[i as usize] = *pixarray.offset(i as isize)
                as *mut ::core::ffi::c_void;
            *pixarray.offset(i as isize) = ::core::ptr::null_mut::<FRAME_DESC>();
        }
        i = i.wrapping_add(1);
    }
    return retval;
}
static mut space_ini_cnt: BYTE = 0;
#[no_mangle]
pub unsafe extern "C" fn InitSpace() -> BOOLEAN {
    let c2rust_fresh0 = space_ini_cnt;
    space_ini_cnt = space_ini_cnt.wrapping_add(1);
    if c2rust_fresh0 as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        && GlobData.Game_state.CurrentActivity as BYTE as ::core::ffi::c_int
            <= C2Rust_Unnamed_5::IN_ENCOUNTER.0 as ::core::ffi::c_int
        || optRequiresReload.0 != 0
    {
        stars_in_space = CaptureDrawable(
            uqm_lua_checked!(LoadGraphicInstance(b"graphics.stars\0".as_ptr() as RESOURCE), BOOLEAN(0)) as DRAWABLE,
        );
        if stars_in_space.is_null() {
            return BOOLEAN::FALSE;
        }
        scenery = CaptureDrawable(
            uqm_lua_checked!(LoadGraphicInstance(b"graphics.scenery\0".as_ptr() as RESOURCE), BOOLEAN(0)) as DRAWABLE,
        );
        if scenery.is_null() {
            return BOOLEAN::FALSE;
        }
        if if resolutionFactor != 2 as ::core::ffi::c_uint {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } != 0
        {
            if uqm_lua_checked!(load_animation(
                    &raw mut stars_misc as *mut FRAME,
                    b"graphics.starsmisc.large\0".as_ptr() as RESOURCE,
                    b"graphics.starsmisc.medium\0".as_ptr() as RESOURCE,
                    b"graphics.starsmisc.small\0".as_ptr() as RESOURCE,
                ), BOOLEAN(0))
                .0 == 0
            {
                return BOOLEAN::FALSE;
            }
        }
        if uqm_lua_checked!(load_animation(
                &raw mut explosion as *mut FRAME,
                b"graphics.boom.large\0".as_ptr() as RESOURCE,
                b"graphics.boom.medium\0".as_ptr() as RESOURCE,
                b"graphics.boom.small\0".as_ptr() as RESOURCE,
            ), BOOLEAN(0))
            .0 == 0
        {
            return BOOLEAN::FALSE;
        }
        if uqm_lua_checked!(load_animation(
                &raw mut blast as *mut FRAME,
                b"graphics.blast.large\0".as_ptr() as RESOURCE,
                b"graphics.blast.medium\0".as_ptr() as RESOURCE,
                b"graphics.blast.small\0".as_ptr() as RESOURCE,
            ), BOOLEAN(0))
            .0 == 0
        {
            return BOOLEAN::FALSE;
        }
        if uqm_lua_checked!(load_animation(
                &raw mut asteroid as *mut FRAME,
                b"graphics.asteroid.large\0".as_ptr() as RESOURCE,
                b"graphics.asteroid.medium\0".as_ptr() as RESOURCE,
                b"graphics.asteroid.small\0".as_ptr() as RESOURCE,
            ), BOOLEAN(0))
            .0 == 0
        {
            return BOOLEAN::FALSE;
        }
    }
    return BOOLEAN::TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn UninitSpace() {
    if space_ini_cnt as ::core::ffi::c_int != 0
        && {
            space_ini_cnt = space_ini_cnt.wrapping_sub(1);
            space_ini_cnt as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        }
    {
        free_image(&raw mut blast as *mut FRAME);
        free_image(&raw mut explosion as *mut FRAME);
        free_image(&raw mut asteroid as *mut FRAME);
        free_image(&raw mut stars_misc as *mut FRAME);
        DestroyDrawable(ReleaseDrawable(stars_in_space));
        stars_in_space = ::core::ptr::null_mut::<FRAME_DESC>();
        DestroyDrawable(ReleaseDrawable(scenery));
        scenery = ::core::ptr::null_mut::<FRAME_DESC>();
    }
}
unsafe extern "C" fn BuildSIS() -> HSTARSHIP {
    let mut hStarShip: HSTARSHIP = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
    hStarShip = Build(
        (&raw mut race_q as *mut QUEUE).offset(0isize),
        SPECIES_ID::SIS_SHIP_ID,
    ) as HSTARSHIP;
    if hStarShip.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    StarShipPtr = LockStarShip(
        (&raw mut race_q as *mut QUEUE).offset(0isize),
        hStarShip,
    );
    (*StarShipPtr).playerNr = 0 as SIZE;
    (*StarShipPtr).captains_name_index = 0 as BYTE;
    UnlockLink((&raw mut race_q as *mut QUEUE).offset(0isize), hStarShip);
    return hStarShip;
}
#[no_mangle]
pub unsafe extern "C" fn InitShips() -> SIZE {
    let mut num_ships: SIZE = 0;
    uqm_lua_checked!(InitSpace(), 0);
    if inHQSpace().0 != 0 {
        ReinitQueue((&raw mut race_q as *mut QUEUE).offset(0isize));
        ReinitQueue((&raw mut race_q as *mut QUEUE).offset(1isize));
        BuildSIS();
        uqm_lua_checked!(LoadHyperspace(), 0);
        InitDisplayList();
        InitStarOffsets();
        uqm_lua_checked!(InitGalaxy(), 0);
        num_ships = 1 as SIZE;
    } else {
        let mut i: COUNT = 0;
        let mut r: RECT = RECT {
            corner: POINT { x: 0, y: 0 },
            extent: EXTENT { width: 0, height: 0 },
        };
        SetContext(SpaceContext);
        InitDisplayList();
        uqm_lua_checked!(InitGalaxy(), 0);
        SetContextFGFrame(Screen);
        r.corner.x = (if if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            16 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        } << resolutionFactor) as COORD;
        r.corner.y = (if if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            16 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        } << resolutionFactor) as COORD;
        r.extent.width = (CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
            - (if if optWindowType == 1 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                16 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } << resolutionFactor) * 2 as ::core::ffi::c_int) as COORD;
        r.extent.height = (CanvasHeight
            - (if if optWindowType == 1 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                16 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } << resolutionFactor) * 2 as ::core::ffi::c_int) as COORD;
        SetContextClipRect(&raw mut r);
        SetContextBackGroundColor(colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE));
        let mut OldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
        OldContext = SetContext(ScreenContext);
        SetContextBackGroundColor(colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE));
        uqm_lua_checked!(ClearDrawable(), 0);
        SetContext(OldContext);
        if GlobData.Game_state.CurrentActivity as BYTE as ::core::ffi::c_int
            == C2Rust_Unnamed_5::IN_LAST_BATTLE.0 as ::core::ffi::c_int
        {
            free_gravity_well();
        } else {
            i = 0 as COUNT;
            while (i as ::core::ffi::c_int) < 5 as ::core::ffi::c_int {
                spawn_asteroid(::core::ptr::null_mut::<ELEMENT>());
                i = i.wrapping_add(1);
            }
            i = 0 as COUNT;
            while (i as ::core::ffi::c_int) < 1 as ::core::ffi::c_int {
                uqm_lua_checked!(spawn_planet(), 0);
                i = i.wrapping_add(1);
            }
        }
        num_ships = 2 as SIZE;
    }
    return num_ships;
}
unsafe extern "C" fn CountCrewElements() -> COUNT {
    let mut result: COUNT = 0;
    let mut hElement: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut hNextElement: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
    result = 0 as COUNT;
    hElement = disp_q.head as HELEMENT;
    while !hElement.is_null() {
        let mut ElementPtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
        ElementPtr = LockLink(&raw mut disp_q, hElement) as *mut ELEMENT;
        hNextElement = (*ElementPtr).succ;
        if (*ElementPtr).state_flags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int != 0
        {
            result = result.wrapping_add(1);
        }
        UnlockLink(&raw mut disp_q, hElement);
        hElement = hNextElement;
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn UninitShips() {
    let mut crew_retrieved: COUNT = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut hElement: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut hNextElement: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut SPtr: [*mut STARSHIP; 2] = [::core::ptr::null_mut::<STARSHIP>(); 2];
    uqm_lua_checked!(StopSound(), ());
    UninitSpace();
    i = 0 as ::core::ffi::c_int;
    while i < 2 as ::core::ffi::c_int {
        SPtr[i as usize] = ::core::ptr::null_mut::<STARSHIP>();
        i += 1;
    }
    crew_retrieved = CountCrewElements();
    hElement = disp_q.head as HELEMENT;
    while !hElement.is_null() {
        let mut ElementPtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
        ElementPtr = LockLink(&raw mut disp_q, hElement) as *mut ELEMENT;
        hNextElement = (*ElementPtr).succ;
        if (*ElementPtr).state_flags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int != 0
            || (*ElementPtr).death_func
                == Some(new_ship as unsafe extern "C" fn(*mut ELEMENT) -> ())
        {
            let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
            StarShipPtr = (*ElementPtr).pParent as *mut STARSHIP;
            if (*(*StarShipPtr).RaceDescPtr).ship_info.crew_level != 0 {
                if crew_retrieved as ::core::ffi::c_int
                    >= (*(*StarShipPtr).RaceDescPtr).ship_info.max_crew
                        as ::core::ffi::c_int
                        - (*(*StarShipPtr).RaceDescPtr).ship_info.crew_level
                            as ::core::ffi::c_int
                {
                    (*(*StarShipPtr).RaceDescPtr).ship_info.crew_level = (*(*StarShipPtr)
                        .RaceDescPtr)
                        .ship_info
                        .max_crew;
                } else {
                    (*(*StarShipPtr).RaceDescPtr).ship_info.crew_level = ((*(*StarShipPtr)
                        .RaceDescPtr)
                        .ship_info
                        .crew_level as ::core::ffi::c_int
                        + crew_retrieved as ::core::ffi::c_int) as COUNT;
                }
            }
            (*StarShipPtr).crew_level = (*(*StarShipPtr).RaceDescPtr)
                .ship_info
                .crew_level;
            SPtr[(*StarShipPtr).playerNr as usize] = StarShipPtr;
            uqm_lua_checked!(free_ship((*StarShipPtr).RaceDescPtr, BOOLEAN::TRUE, BOOLEAN::TRUE), ());
            (*StarShipPtr).RaceDescPtr = ::core::ptr::null_mut::<RACE_DESC>();
        }
        UnlockLink(&raw mut disp_q, hElement);
        hElement = hNextElement;
    }
    GlobData.Game_state.CurrentActivity = (GlobData.Game_state.CurrentActivity
        as ::core::ffi::c_int & !(C2Rust_Unnamed_5::IN_BATTLE.0 as ::core::ffi::c_int))
        as ACTIVITY;
    if GlobData.Game_state.CurrentActivity as BYTE as ::core::ffi::c_int
        == C2Rust_Unnamed_5::IN_ENCOUNTER.0 as ::core::ffi::c_int
        && GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
            & C2Rust_Unnamed_5::CHECK_ABORT.0 as ::core::ffi::c_int == 0
    {
        i = 2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
        while i >= 0 as ::core::ffi::c_int {
            if !SPtr[i as usize].is_null() && FleetIsInfinite(i as COUNT).0 == 0 {
                UpdateShipFragCrew(SPtr[i as usize]);
            }
            i -= 1;
        }
    }
    if GlobData.Game_state.CurrentActivity as BYTE as ::core::ffi::c_int
        != C2Rust_Unnamed_5::IN_ENCOUNTER.0 as ::core::ffi::c_int
    {
        i = 0 as ::core::ffi::c_int;
        while i < 2 as ::core::ffi::c_int {
            ReinitQueue((&raw mut race_q as *mut QUEUE).offset(i as isize));
            i += 1;
        }
        if inHQSpace().0 != 0 {
            UninitStarOffsets();
            FreeHyperspace();
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn ReloadGameContent() {
    UninitGameStructures();
    uqm_lua_checked!(ClearPlayerInputAll(), ());
    uqm_lua_checked!(UninitGameKernel(), ());
    FreeMasterShipList();
    uqm_lua_checked!(prepareContentDir(
        contentDirPath,
        addonDirPath,
        ::core::ptr::null::<::core::ffi::c_char>(),
    ), ());
    if uqm_lua_checked!(LoadKernel(
            0 as ::core::ffi::c_int,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        ), ())
        .0 != 0
    {
        uqm_lua_checked!(LoadMasterShipList(Some(TaskSwitch as unsafe extern "C" fn() -> ())), ());
        TaskSwitch();
        uqm_lua_checked!(InitGameKernel(), ());
    }
}
