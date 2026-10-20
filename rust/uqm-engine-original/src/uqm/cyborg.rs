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

// Rust translation of src/uqm/cyborg.c.
extern "C" {
    pub type string_table_entry;
    pub type frame_desc;
    pub type tfb_soundsample;
    pub type getmelee_struct;
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
    fn GetFrameRect(Frame: FRAME, pRect: *mut RECT) -> BOOLEAN;
    fn SetAbsFrameIndex(Frame: FRAME, FrameIndex: COUNT) -> FRAME;
    fn SetEquFrameIndex(DstFrame: FRAME, SrcFrame: FRAME) -> FRAME;
    fn IncFrameIndex(Frame: FRAME) -> FRAME;
    fn DecFrameIndex(Frame: FRAME) -> FRAME;
    static mut ScreenWidth: ::core::ffi::c_int;
    static mut ScreenHeight: ::core::ffi::c_int;
    fn ARCTAN(delta_x: SIZE, delta_y: SIZE) -> COUNT;
    fn GetCurrentVelocityComponents(
        velocityptr: *mut VELOCITY_DESC,
        pdx: *mut SIZE,
        pdy: *mut SIZE,
    );
    fn GetNextVelocityComponents(
        velocityptr: *mut VELOCITY_DESC,
        pdx: *mut SIZE,
        pdy: *mut SIZE,
        num_frames: COUNT,
    );
    static mut disp_q: QUEUE;
    static mut DisplayArray: [PRIMITIVE; 330];
    fn FreeElement(hElement: HELEMENT);
    fn crew_preprocess(ElementPtr: *mut ELEMENT);
    fn inertial_thrust(ElementPtr: *mut ELEMENT) -> STATUS_FLAGS;
    static mut GlobData: GLOBDATA;
    fn TFB_Random() -> DWORD;
    fn square_root(value: DWORD) -> COUNT;
}
pub type uint8 = ::core::ffi::c_uchar;
pub type uint16 = ::core::ffi::c_ushort;
pub type sint16 = ::core::ffi::c_short;
pub type uint32 = ::core::ffi::c_uint;
pub type sint32 = ::core::ffi::c_int;
pub type BYTE = uint8;
pub type UBYTE = uint8;
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct gfx_object(pub ::core::ffi::c_uint);
impl gfx_object {
    pub const POINT_PRIM: Self = Self(0);
    pub const STAMP_PRIM: Self = Self(1);
    pub const STAMPFILL_PRIM: Self = Self(2);
    pub const LINE_PRIM: Self = Self(3);
    pub const TEXT_PRIM: Self = Self(4);
    pub const RECT_PRIM: Self = Self(5);
    pub const RECTFILL_PRIM: Self = Self(6);
    pub const NUM_PRIMS: Self = Self(7);
}
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
    pub ShipName: [UNICODE; 16],
    pub CommanderName: [UNICODE; 16],
    pub PlanetName: [UNICODE; 16],
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
    pub GameState: [BYTE; 155],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GLOBDATA {
    pub SIS_state: SIS_STATE,
    pub Game_state: GAME_STATE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BattleInputHandlers {
    pub frameInput: BattleFrameInputFunction,
    pub selectShip: SelectShipFunction,
    pub battleEndReady: BattleEndReadyFunction,
    pub deleteContext: DeleteInputContextFunction,
}
pub type DeleteInputContextFunction = Option<
    unsafe extern "C" fn(*mut InputContext) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct InputContext {
    pub handlers: *mut BattleInputHandlers,
    pub playerNr: COUNT,
}
pub type BattleEndReadyFunction = Option<
    unsafe extern "C" fn(*mut InputContext) -> bool,
>;
pub type SelectShipFunction = Option<
    unsafe extern "C" fn(*mut InputContext, *mut GETMELEE_STATE) -> BOOLEAN,
>;
pub type GETMELEE_STATE = getmelee_struct;
pub type BattleFrameInputFunction = Option<
    unsafe extern "C" fn(*mut InputContext, *mut STARSHIP) -> BATTLE_INPUT_STATE,
>;
pub type BATTLE_INPUT_STATE = UBYTE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ComputerInputContext {
    pub handlers: *mut BattleInputHandlers,
    pub playerNr: COUNT,
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
unsafe extern "C" fn sameColor(mut c1: Color, mut c2: Color) -> bool {
    return c1.r as ::core::ffi::c_int == c2.r as ::core::ffi::c_int
        && c1.g as ::core::ffi::c_int == c2.g as ::core::ffi::c_int
        && c1.b as ::core::ffi::c_int == c2.b as ::core::ffi::c_int
        && c1.a as ::core::ffi::c_int == c2.a as ::core::ffi::c_int;
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
unsafe extern "C" fn IsVelocityZero(mut vptr: *mut VELOCITY_DESC) -> bool {
    return (*vptr).vector.width as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        && (*vptr).vector.height as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        && (*vptr).incr.width as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        && (*vptr).incr.height as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        && (*vptr).fract.width as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        && (*vptr).fract.height as ::core::ffi::c_int == 0 as ::core::ffi::c_int;
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
#[no_mangle]
pub unsafe extern "C" fn PlotIntercept(
    mut ElementPtr0: *mut ELEMENT,
    mut ElementPtr1: *mut ELEMENT,
    mut max_turns: COUNT,
    mut margin_of_error: COUNT,
) -> COUNT {
    let mut dy: SIZE = 0;
    let mut time_y_0: SIZE = 0;
    let mut time_y_1: SIZE = 0;
    let mut dst: [POINT; 2] = [POINT { x: 0, y: 0 }; 2];
    let mut r0: RECT = rect {
        corner: point {
            x: 0 as COORD,
            y: 0 as COORD,
        },
        extent: extent {
            width: 0 as COORD,
            height: 0 as COORD,
        },
    };
    let mut r1: RECT = rect {
        corner: point {
            x: 0 as COORD,
            y: 0 as COORD,
        },
        extent: extent {
            width: 0 as COORD,
            height: 0 as COORD,
        },
    };
    let mut dx_0: SIZE = 0;
    let mut dy_0: SIZE = 0;
    let mut dx_1: SIZE = 0;
    let mut dy_1: SIZE = 0;
    if ((*ElementPtr0).state_flags as ::core::ffi::c_int
        | (*ElementPtr1).state_flags as ::core::ffi::c_int)
        & (1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int != 0
    {
        if (*ElementPtr0).state_flags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int == 0
        {
            if ((*ElementPtr1).c2rust_unnamed.life_span as ::core::ffi::c_int)
                < max_turns as ::core::ffi::c_int
            {
                max_turns = (*ElementPtr1).c2rust_unnamed.life_span;
            }
        } else if (*ElementPtr1).state_flags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int == 0
        {
            if ((*ElementPtr0).c2rust_unnamed.life_span as ::core::ffi::c_int)
                < max_turns as ::core::ffi::c_int
            {
                max_turns = (*ElementPtr0).c2rust_unnamed.life_span;
            }
        } else {
            if ((*ElementPtr0).c2rust_unnamed.life_span as ::core::ffi::c_int)
                < max_turns as ::core::ffi::c_int
            {
                max_turns = (*ElementPtr0).c2rust_unnamed.life_span;
            }
            if ((*ElementPtr1).c2rust_unnamed.life_span as ::core::ffi::c_int)
                < max_turns as ::core::ffi::c_int
            {
                max_turns = (*ElementPtr1).c2rust_unnamed.life_span;
            }
        }
    }
    dst[0usize] = (*ElementPtr0).current.location;
    GetCurrentVelocityComponents(
        &raw mut (*ElementPtr0).velocity,
        &raw mut dx_0,
        &raw mut dy_0,
    );
    dx_0 = (dx_0 as ::core::ffi::c_long * max_turns as ::core::ffi::c_long
        >> 5 as ::core::ffi::c_int) as SIZE;
    dy_0 = (dy_0 as ::core::ffi::c_long * max_turns as ::core::ffi::c_long
        >> 5 as ::core::ffi::c_int) as SIZE;
    dst[1usize] = (*ElementPtr1).current.location;
    GetCurrentVelocityComponents(
        &raw mut (*ElementPtr1).velocity,
        &raw mut dx_1,
        &raw mut dy_1,
    );
    dx_1 = (dx_1 as ::core::ffi::c_long * max_turns as ::core::ffi::c_long
        >> 5 as ::core::ffi::c_int) as SIZE;
    dy_1 = (dy_1 as ::core::ffi::c_long * max_turns as ::core::ffi::c_long
        >> 5 as ::core::ffi::c_int) as SIZE;
    if margin_of_error != 0 {
        dst[1usize].y = (dst[1usize].y as ::core::ffi::c_int
            - margin_of_error as ::core::ffi::c_int) as COORD;
        time_y_0 = 1 as SIZE;
        time_y_1 = ((margin_of_error as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
            as SIZE;
    } else {
        GetFrameRect((*ElementPtr0).IntersectControl.IntersectStamp.frame, &raw mut r0);
        GetFrameRect((*ElementPtr1).IntersectControl.IntersectStamp.frame, &raw mut r1);
        dst[0usize].y = (dst[0usize].y as ::core::ffi::c_int
            + ((r0.corner.y as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)) as COORD;
        dst[1usize].y = (dst[1usize].y as ::core::ffi::c_int
            + ((r1.corner.y as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)) as COORD;
        time_y_0 = ((r0.extent.height as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
            as SIZE;
        time_y_1 = ((r1.extent.height as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
            as SIZE;
    }
    dy = (dst[1usize].y as ::core::ffi::c_int - dst[0usize].y as ::core::ffi::c_int)
        as SIZE;
    time_y_0 = (dy as ::core::ffi::c_int - time_y_0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as SIZE;
    time_y_1 = (dy as ::core::ffi::c_int + time_y_1 as ::core::ffi::c_int
        - 1 as ::core::ffi::c_int) as SIZE;
    dy = (dy_0 as ::core::ffi::c_int - dy_1 as ::core::ffi::c_int) as SIZE;
    if time_y_0 as ::core::ffi::c_int <= 0 as ::core::ffi::c_int
        && time_y_1 as ::core::ffi::c_int >= 0 as ::core::ffi::c_int
        || time_y_0 as ::core::ffi::c_int > 0 as ::core::ffi::c_int
            && dy as ::core::ffi::c_int >= time_y_0 as ::core::ffi::c_int
        || (time_y_1 as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
            && dy as ::core::ffi::c_int <= time_y_1 as ::core::ffi::c_int
    {
        let mut dx: SIZE = 0;
        let mut time_x_0: SIZE = 0;
        let mut time_x_1: SIZE = 0;
        if margin_of_error != 0 {
            dst[1usize].x = (dst[1usize].x as ::core::ffi::c_int
                - margin_of_error as ::core::ffi::c_int) as COORD;
            time_x_0 = 1 as SIZE;
            time_x_1 = ((margin_of_error as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int) as SIZE;
        } else {
            dst[0usize].x = (dst[0usize].x as ::core::ffi::c_int
                + ((r0.corner.x as ::core::ffi::c_int) << 2 as ::core::ffi::c_int))
                as COORD;
            dst[1usize].x = (dst[1usize].x as ::core::ffi::c_int
                + ((r1.corner.x as ::core::ffi::c_int) << 2 as ::core::ffi::c_int))
                as COORD;
            time_x_0 = ((r0.extent.width as ::core::ffi::c_int)
                << 2 as ::core::ffi::c_int) as SIZE;
            time_x_1 = ((r1.extent.width as ::core::ffi::c_int)
                << 2 as ::core::ffi::c_int) as SIZE;
        }
        dx = (dst[1usize].x as ::core::ffi::c_int - dst[0usize].x as ::core::ffi::c_int)
            as SIZE;
        time_x_0 = (dx as ::core::ffi::c_int - time_x_0 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int) as SIZE;
        time_x_1 = (dx as ::core::ffi::c_int + time_x_1 as ::core::ffi::c_int
            - 1 as ::core::ffi::c_int) as SIZE;
        dx = (dx_0 as ::core::ffi::c_int - dx_1 as ::core::ffi::c_int) as SIZE;
        if time_x_0 as ::core::ffi::c_int <= 0 as ::core::ffi::c_int
            && time_x_1 as ::core::ffi::c_int >= 0 as ::core::ffi::c_int
            || time_x_0 as ::core::ffi::c_int > 0 as ::core::ffi::c_int
                && dx as ::core::ffi::c_int >= time_x_0 as ::core::ffi::c_int
            || (time_x_1 as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
                && dx as ::core::ffi::c_int <= time_x_1 as ::core::ffi::c_int
        {
            if dx as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                && dy as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            {
                time_y_1 = 0 as SIZE;
                time_y_0 = time_y_1;
            } else {
                let mut t: SIZE = 0;
                let mut time_beg: ::core::ffi::c_long = 0;
                let mut time_end: ::core::ffi::c_long = 0;
                let mut fract: ::core::ffi::c_long = 0;
                if (time_y_1 as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                    t = time_y_0;
                    time_y_0 = -(time_y_1 as ::core::ffi::c_int) as SIZE;
                    time_y_1 = -(t as ::core::ffi::c_int) as SIZE;
                } else if time_y_0 as ::core::ffi::c_int <= 0 as ::core::ffi::c_int {
                    if (dy as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                        time_y_1 = -(time_y_0 as ::core::ffi::c_int) as SIZE;
                    }
                    time_y_0 = 0 as SIZE;
                }
                if (dy as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                    dy = -(dy as ::core::ffi::c_int) as SIZE;
                }
                if (dy as ::core::ffi::c_int) < time_y_1 as ::core::ffi::c_int {
                    time_y_1 = dy;
                }
                if (time_x_1 as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                    t = time_x_0;
                    time_x_0 = -(time_x_1 as ::core::ffi::c_int) as SIZE;
                    time_x_1 = -(t as ::core::ffi::c_int) as SIZE;
                } else if time_x_0 as ::core::ffi::c_int <= 0 as ::core::ffi::c_int {
                    if (dx as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                        time_x_1 = -(time_x_0 as ::core::ffi::c_int) as SIZE;
                    }
                    time_x_0 = 0 as SIZE;
                }
                if (dx as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                    dx = -(dx as ::core::ffi::c_int) as SIZE;
                }
                if (dx as ::core::ffi::c_int) < time_x_1 as ::core::ffi::c_int {
                    time_x_1 = dx;
                }
                if dx as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    time_beg = time_y_0 as ::core::ffi::c_long;
                    time_end = time_y_1 as ::core::ffi::c_long;
                    fract = dy as ::core::ffi::c_long;
                } else if dy as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    time_beg = time_x_0 as ::core::ffi::c_long;
                    time_end = time_x_1 as ::core::ffi::c_long;
                    fract = dx as ::core::ffi::c_long;
                } else {
                    let mut time_x: ::core::ffi::c_long = 0;
                    let mut time_y: ::core::ffi::c_long = 0;
                    time_x = time_x_0 as ::core::ffi::c_long * dy as ::core::ffi::c_long;
                    time_y = time_y_0 as ::core::ffi::c_long * dx as ::core::ffi::c_long;
                    time_beg = if time_x < time_y { time_y } else { time_x };
                    time_x = time_x_1 as ::core::ffi::c_long * dy as ::core::ffi::c_long;
                    time_y = time_y_1 as ::core::ffi::c_long * dx as ::core::ffi::c_long;
                    time_end = if time_x > time_y { time_y } else { time_x };
                    fract = dx as ::core::ffi::c_long * dy as ::core::ffi::c_long;
                }
                time_beg *= max_turns as ::core::ffi::c_long;
                if time_beg < fract {
                    time_y_0 = 0 as SIZE;
                } else {
                    time_y_0 = (time_beg / fract) as SIZE;
                }
                if time_end >= fract {
                    time_y_1 = (max_turns as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int) as SIZE;
                } else {
                    time_y_1 = (time_end * max_turns as ::core::ffi::c_long / fract)
                        as SIZE;
                }
            }
            if time_y_0 as ::core::ffi::c_int <= time_y_1 as ::core::ffi::c_int {
                if margin_of_error as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                    return (time_y_0 as COUNT as ::core::ffi::c_int
                        + 1 as ::core::ffi::c_int) as COUNT
                } else {
                    let mut Pt0: POINT = POINT { x: 0, y: 0 };
                    let mut Pt1: POINT = POINT { x: 0, y: 0 };
                    let mut Velocity0: VELOCITY_DESC = VELOCITY_DESC {
                        TravelAngle: 0,
                        vector: EXTENT { width: 0, height: 0 },
                        fract: EXTENT { width: 0, height: 0 },
                        error: EXTENT { width: 0, height: 0 },
                        incr: EXTENT { width: 0, height: 0 },
                    };
                    let mut Velocity1: VELOCITY_DESC = VELOCITY_DESC {
                        TravelAngle: 0,
                        vector: EXTENT { width: 0, height: 0 },
                        fract: EXTENT { width: 0, height: 0 },
                        error: EXTENT { width: 0, height: 0 },
                        incr: EXTENT { width: 0, height: 0 },
                    };
                    let mut Control0: INTERSECT_CONTROL = INTERSECT_CONTROL {
                        last_time_val: 0,
                        EndPoint: POINT { x: 0, y: 0 },
                        IntersectStamp: STAMP {
                            origin: POINT { x: 0, y: 0 },
                            frame: ::core::ptr::null_mut::<FRAME_DESC>(),
                        },
                    };
                    let mut Control1: INTERSECT_CONTROL = INTERSECT_CONTROL {
                        last_time_val: 0,
                        EndPoint: POINT { x: 0, y: 0 },
                        IntersectStamp: STAMP {
                            origin: POINT { x: 0, y: 0 },
                            frame: ::core::ptr::null_mut::<FRAME_DESC>(),
                        },
                    };
                    Pt0 = (*ElementPtr0).current.location;
                    Velocity0 = (*ElementPtr0).velocity;
                    Control0 = (*ElementPtr0).IntersectControl;
                    Pt1 = (*ElementPtr1).current.location;
                    Velocity1 = (*ElementPtr1).velocity;
                    Control1 = (*ElementPtr1).IntersectControl;
                    if time_y_0 != 0 {
                        GetNextVelocityComponents(
                            &raw mut Velocity0,
                            &raw mut dx_0,
                            &raw mut dy_0,
                            time_y_0 as COUNT,
                        );
                        Pt0.x = (Pt0.x as ::core::ffi::c_int
                            + dx_0 as ::core::ffi::c_int) as COORD;
                        Pt0.y = (Pt0.y as ::core::ffi::c_int
                            + dy_0 as ::core::ffi::c_int) as COORD;
                        Control0.EndPoint.x = (Pt0.x as ::core::ffi::c_int
                            >> 2 as ::core::ffi::c_int) as COORD;
                        Control0.EndPoint.y = (Pt0.y as ::core::ffi::c_int
                            >> 2 as ::core::ffi::c_int) as COORD;
                        GetNextVelocityComponents(
                            &raw mut Velocity1,
                            &raw mut dx_1,
                            &raw mut dy_1,
                            time_y_0 as COUNT,
                        );
                        Pt1.x = (Pt1.x as ::core::ffi::c_int
                            + dx_1 as ::core::ffi::c_int) as COORD;
                        Pt1.y = (Pt1.y as ::core::ffi::c_int
                            + dy_1 as ::core::ffi::c_int) as COORD;
                        Control1.EndPoint.x = (Pt1.x as ::core::ffi::c_int
                            >> 2 as ::core::ffi::c_int) as COORD;
                        Control1.EndPoint.y = (Pt1.y as ::core::ffi::c_int
                            >> 2 as ::core::ffi::c_int) as COORD;
                    }
                    loop {
                        let mut when: TIME_VALUE = 0;
                        time_y_0 += 1;
                        GetNextVelocityComponents(
                            &raw mut Velocity0,
                            &raw mut dx_0,
                            &raw mut dy_0,
                            1 as COUNT,
                        );
                        Pt0.x = (Pt0.x as ::core::ffi::c_int
                            + dx_0 as ::core::ffi::c_int) as COORD;
                        Pt0.y = (Pt0.y as ::core::ffi::c_int
                            + dy_0 as ::core::ffi::c_int) as COORD;
                        GetNextVelocityComponents(
                            &raw mut Velocity1,
                            &raw mut dx_1,
                            &raw mut dy_1,
                            1 as COUNT,
                        );
                        Pt1.x = (Pt1.x as ::core::ffi::c_int
                            + dx_1 as ::core::ffi::c_int) as COORD;
                        Pt1.y = (Pt1.y as ::core::ffi::c_int
                            + dy_1 as ::core::ffi::c_int) as COORD;
                        Control0.IntersectStamp.origin = Control0.EndPoint;
                        Control0.EndPoint.x = (Pt0.x as ::core::ffi::c_int
                            >> 2 as ::core::ffi::c_int) as COORD;
                        Control0.EndPoint.y = (Pt0.y as ::core::ffi::c_int
                            >> 2 as ::core::ffi::c_int) as COORD;
                        Control1.IntersectStamp.origin = Control1.EndPoint;
                        Control1.EndPoint.x = (Pt1.x as ::core::ffi::c_int
                            >> 2 as ::core::ffi::c_int) as COORD;
                        Control1.EndPoint.y = (Pt1.y as ::core::ffi::c_int
                            >> 2 as ::core::ffi::c_int) as COORD;
                        when = DrawablesIntersect(
                            &raw mut Control0,
                            &raw mut Control1,
                            (((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
                                + 1 as ::core::ffi::c_int) as TIME_VALUE,
                        );
                        if when != 0 {
                            if when as ::core::ffi::c_int == 1 as ::core::ffi::c_int
                                && time_y_0 as ::core::ffi::c_int == 1 as ::core::ffi::c_int
                                && ((*ElementPtr0).state_flags as ::core::ffi::c_int
                                    | (*ElementPtr1).state_flags as ::core::ffi::c_int)
                                    & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int != 0
                            {
                                when = 0 as TIME_VALUE;
                                Control0.EndPoint.x = (Pt0.x as ::core::ffi::c_int
                                    >> 2 as ::core::ffi::c_int) as COORD;
                                Control0.EndPoint.y = (Pt0.y as ::core::ffi::c_int
                                    >> 2 as ::core::ffi::c_int) as COORD;
                                Control1.EndPoint.x = (Pt1.x as ::core::ffi::c_int
                                    >> 2 as ::core::ffi::c_int) as COORD;
                                Control1.EndPoint.y = (Pt1.y as ::core::ffi::c_int
                                    >> 2 as ::core::ffi::c_int) as COORD;
                            }
                            if when != 0 {
                                return time_y_0 as COUNT;
                            }
                        }
                        if (time_y_0 as ::core::ffi::c_int)
                            >= time_y_1 as ::core::ffi::c_int
                        {
                            break;
                        }
                    }
                }
            }
        }
    }
    return 0 as COUNT;
}
unsafe extern "C" fn InitCyborg(mut StarShipPtr: *mut STARSHIP) {
    let mut Index: COUNT = 0;
    let mut Divisor: COUNT = 0;
    Index = ((*(*StarShipPtr).RaceDescPtr).characteristics.max_thrust
        as ::core::ffi::c_int
        * (*(*StarShipPtr).RaceDescPtr).characteristics.thrust_increment
            as ::core::ffi::c_int) as COUNT;
    Divisor = ((*(*StarShipPtr).RaceDescPtr).characteristics.turn_wait
        as ::core::ffi::c_int
        + (*(*StarShipPtr).RaceDescPtr).characteristics.thrust_wait
            as ::core::ffi::c_int) as COUNT;
    if Divisor as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
        Index = (Index as ::core::ffi::c_int / Divisor as ::core::ffi::c_int) as COUNT;
    } else {
        Index = (Index as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as COUNT;
    }
    (*(*StarShipPtr).RaceDescPtr).cyborg_control.ManeuverabilityIndex = Index;
}
unsafe extern "C" fn ship_movement(
    mut ShipPtr: *mut ELEMENT,
    mut EvalDescPtr: *mut EVALUATE_DESC,
) {
    if (*EvalDescPtr).which_turn as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        (*EvalDescPtr).which_turn = 1 as COUNT;
    }
    match (*EvalDescPtr).MoveState {
        MOVEMENT_STATE::PURSUE => {
            Pursue(ShipPtr, EvalDescPtr);
        }
        MOVEMENT_STATE::AVOID | MOVEMENT_STATE::ENTICE => {
            Entice(ShipPtr, EvalDescPtr);
        }
        MOVEMENT_STATE::NO_MOVEMENT | _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn ship_weapons(
    mut ShipPtr: *mut ELEMENT,
    mut OtherPtr: *mut ELEMENT,
    mut margin_of_error: COUNT,
) -> BOOLEAN {
    let mut delta_x: SIZE = 0;
    let mut delta_y: SIZE = 0;
    let mut n: COUNT = 0;
    let mut num_weapons: COUNT = 0;
    let mut Ship: ELEMENT = ELEMENT {
        pred: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        succ: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        preprocess_func: None,
        postprocess_func: None,
        collision_func: None,
        death_func: None,
        playerNr: 0,
        state_flags: 0,
        c2rust_unnamed: C2Rust_Unnamed_4 { life_span: 0 },
        c2rust_unnamed_0: C2Rust_Unnamed_3 { crew_level: 0 },
        c2rust_unnamed_1: C2Rust_Unnamed_2 { mass_points: 0 },
        c2rust_unnamed_2: C2Rust_Unnamed_1 { turn_wait: 0 },
        c2rust_unnamed_3: C2Rust_Unnamed_0 { thrust_wait: 0 },
        colorCycleIndex: 0,
        velocity: VELOCITY_DESC {
            TravelAngle: 0,
            vector: EXTENT { width: 0, height: 0 },
            fract: EXTENT { width: 0, height: 0 },
            error: EXTENT { width: 0, height: 0 },
            incr: EXTENT { width: 0, height: 0 },
        },
        IntersectControl: INTERSECT_CONTROL {
            last_time_val: 0,
            EndPoint: POINT { x: 0, y: 0 },
            IntersectStamp: STAMP {
                origin: POINT { x: 0, y: 0 },
                frame: ::core::ptr::null_mut::<FRAME_DESC>(),
            },
        },
        PrimIndex: 0,
        current: STATE {
            location: POINT { x: 0, y: 0 },
            image: C2Rust_Unnamed {
                frame: ::core::ptr::null_mut::<FRAME_DESC>(),
                farray: ::core::ptr::null_mut::<FRAME>(),
            },
        },
        next: STATE {
            location: POINT { x: 0, y: 0 },
            image: C2Rust_Unnamed {
                frame: ::core::ptr::null_mut::<FRAME_DESC>(),
                farray: ::core::ptr::null_mut::<FRAME>(),
            },
        },
        pParent: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        hTarget: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    let mut Weapon: [HELEMENT; 6] = [::core::ptr::null_mut::<::core::ffi::c_void>(); 6];
    let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
    if (*GlobData.Game_state.DisplayArray.offset((*OtherPtr).PrimIndex as isize)).Type
        as ::core::ffi::c_int >= gfx_object::NUM_PRIMS.0 as ::core::ffi::c_int
        || (*GlobData.Game_state.DisplayArray.offset((*OtherPtr).PrimIndex as isize))
            .Type as ::core::ffi::c_int
            == gfx_object::STAMPFILL_PRIM.0 as ::core::ffi::c_int
            && sameColor(
                (*GlobData
                    .Game_state
                    .DisplayArray
                    .offset((*OtherPtr).PrimIndex as isize))
                    .color,
                colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE),
            ) as ::core::ffi::c_int != 0
    {
        margin_of_error = (margin_of_error as ::core::ffi::c_int
            + ((40 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)) as COUNT;
    }
    Ship = *ShipPtr;
    GetNextVelocityComponents(
        &raw mut Ship.velocity,
        &raw mut delta_x,
        &raw mut delta_y,
        1 as COUNT,
    );
    Ship.next.location.x = (Ship.current.location.x as ::core::ffi::c_int
        + delta_x as ::core::ffi::c_int) as COORD;
    Ship.next.location.y = (Ship.current.location.y as ::core::ffi::c_int
        + delta_y as ::core::ffi::c_int) as COORD;
    Ship.current.location = Ship.next.location;
    StarShipPtr = Ship.pParent as *mut STARSHIP;
    num_weapons = Some(
            (*(*StarShipPtr).RaceDescPtr)
                .init_weapon_func
                .expect("non-null function pointer"),
        )
        .expect(
            "non-null function pointer",
        )(&raw mut Ship, &raw mut Weapon as *mut HELEMENT);
    n = num_weapons;
    if n != 0 {
        let mut WeaponPtr: *mut HELEMENT = ::core::ptr::null_mut::<HELEMENT>();
        let mut w: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
        let mut EPtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
        WeaponPtr = (&raw mut Weapon as *mut HELEMENT).offset(0isize);
        loop {
            w = *WeaponPtr;
            if !w.is_null() {
                EPtr = LockLink(&raw mut disp_q, w) as *mut ELEMENT;
                if (*EPtr).state_flags as ::core::ffi::c_int
                    & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int != 0
                {
                    (*EPtr).next = (*EPtr).current;
                    (*EPtr).IntersectControl.IntersectStamp.origin.x = ((*EPtr)
                        .current
                        .location
                        .x as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as COORD;
                    (*EPtr).IntersectControl.IntersectStamp.origin.y = ((*EPtr)
                        .current
                        .location
                        .y as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as COORD;
                    (*EPtr).IntersectControl.EndPoint.x = ((*EPtr).next.location.x
                        as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as COORD;
                    (*EPtr).IntersectControl.EndPoint.y = ((*EPtr).next.location.y
                        as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as COORD;
                    (*EPtr).IntersectControl.IntersectStamp.frame = SetEquFrameIndex(
                        *(*EPtr).next.image.farray.offset(0isize),
                        (*EPtr).next.image.frame,
                    );
                }
                if PlotIntercept(
                    EPtr,
                    OtherPtr,
                    (*EPtr).c2rust_unnamed.life_span,
                    margin_of_error,
                ) != 0
                {
                    UnlockLink(&raw mut disp_q, w);
                    break;
                } else {
                    UnlockLink(&raw mut disp_q, w);
                    FreeElement(w);
                }
            }
            WeaponPtr = WeaponPtr.offset(1);
            n = n.wrapping_sub(1);
            if n == 0 {
                break;
            }
        }
        num_weapons = n;
        if num_weapons != 0 {
            loop {
                let c2rust_fresh0 = WeaponPtr;
                WeaponPtr = WeaponPtr.offset(1);
                w = *c2rust_fresh0;
                if !w.is_null() {
                    FreeElement(w);
                }
                n = n.wrapping_sub(1);
                if n == 0 {
                    break;
                }
            }
        }
    }
    return BOOLEAN(
        (num_weapons as ::core::ffi::c_int > 0 as ::core::ffi::c_int)
            as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ship_intelligence(
    mut ShipPtr: *mut ELEMENT,
    mut ObjectsOfConcern: *mut EVALUATE_DESC,
    mut ConcernCounter: COUNT,
) {
    let mut ShipMoved: BOOLEAN = BOOLEAN::FALSE;
    let mut ShipFired: BOOLEAN = BOOLEAN::FALSE;
    let mut margin_of_error: COUNT = 0;
    let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
    StarShipPtr = (*ShipPtr).pParent as *mut STARSHIP;
    ShipMoved = BOOLEAN::TRUE;
    if (*ShipPtr).c2rust_unnamed_2.turn_wait as ::core::ffi::c_int
        == 0 as ::core::ffi::c_int
    {
        ShipMoved = BOOLEAN::FALSE;
    }
    if (*ShipPtr).c2rust_unnamed_3.thrust_wait as ::core::ffi::c_int
        == 0 as ::core::ffi::c_int
    {
        ShipMoved = BOOLEAN::FALSE;
    }
    ShipFired = BOOLEAN::TRUE;
    if (*StarShipPtr).weapon_counter as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        (*StarShipPtr).ship_input_state = ((*StarShipPtr).ship_input_state
            as ::core::ffi::c_int
            & !((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)) as BYTE;
        if (*(*StarShipPtr).RaceDescPtr).ship_info.ship_flags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int == 0
        {
            ShipFired = BOOLEAN::FALSE;
        }
    }
    if (*StarShipPtr).control as ::core::ffi::c_int
        & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as BYTE
            as ::core::ffi::c_int != 0
    {
        margin_of_error = 0 as COUNT;
    } else if (*StarShipPtr).control as ::core::ffi::c_int
        & ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as BYTE
            as ::core::ffi::c_int != 0
    {
        margin_of_error = ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
            as COUNT;
    } else {
        margin_of_error = ((40 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
            as COUNT;
    }
    ObjectsOfConcern = ObjectsOfConcern
        .offset(ConcernCounter as ::core::ffi::c_int as isize);
    loop {
        let c2rust_fresh1 = ConcernCounter;
        ConcernCounter = ConcernCounter.wrapping_sub(1);
        if c2rust_fresh1 == 0 {
            break;
        }
        ObjectsOfConcern = ObjectsOfConcern.offset(-1);
        if !(*ObjectsOfConcern).ObjectPtr.is_null() {
            if ShipMoved.0 == 0
                && (ConcernCounter as ::core::ffi::c_int
                    != C2Rust_Unnamed_5::ENEMY_WEAPON_INDEX.0 as ::core::ffi::c_int
                    || (*ObjectsOfConcern).MoveState.0 == MOVEMENT_STATE::PURSUE.0
                    || (*(*ObjectsOfConcern).ObjectPtr).state_flags as ::core::ffi::c_int
                        & (1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int != 0
                    || (*(*StarShipPtr).RaceDescPtr).cyborg_control.ManeuverabilityIndex
                        as ::core::ffi::c_int >= 45 as ::core::ffi::c_int)
            {
                ship_movement(ShipPtr, ObjectsOfConcern);
                ShipMoved = BOOLEAN::TRUE;
            }
            if ShipFired.0 == 0
                && (ConcernCounter as ::core::ffi::c_int
                    == C2Rust_Unnamed_5::ENEMY_SHIP_INDEX.0 as ::core::ffi::c_int
                    || ConcernCounter as ::core::ffi::c_int
                        == C2Rust_Unnamed_5::ENEMY_WEAPON_INDEX.0 as ::core::ffi::c_int
                        && (*ObjectsOfConcern).MoveState.0 != MOVEMENT_STATE::AVOID.0)
            {
                ShipFired = ship_weapons(
                    ShipPtr,
                    (*ObjectsOfConcern).ObjectPtr,
                    margin_of_error,
                );
                if ShipFired.0 != 0 {
                    (*StarShipPtr).ship_input_state = ((*StarShipPtr).ship_input_state
                        as ::core::ffi::c_int
                        | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as BYTE;
                }
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn TurnShip(
    mut ShipPtr: *mut ELEMENT,
    mut angle: COUNT,
) -> BOOLEAN {
    let mut f: COUNT = 0;
    let mut ship_delta_facing: COUNT = 0;
    let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
    StarShipPtr = (*ShipPtr).pParent as *mut STARSHIP;
    f = (*StarShipPtr).ShipFacing;
    ship_delta_facing = ((angle as ::core::ffi::c_int
        + ((1 as ::core::ffi::c_int)
            << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int)
        >> 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int) - f as ::core::ffi::c_int
        & ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
            - 1 as ::core::ffi::c_int) as COUNT;
    if ship_delta_facing != 0 {
        if ship_delta_facing as ::core::ffi::c_int
            == ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int)
                + ((1 as ::core::ffi::c_int)
                    << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int)
                >> 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
        {
            ship_delta_facing = (ship_delta_facing as ::core::ffi::c_int
                + if TFB_Random() & 1 as DWORD != 0 {
                    ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                        >> 3 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
                        + ((1 as ::core::ffi::c_int)
                            << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int)
                        >> 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                } else {
                    -(((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                        >> 3 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
                        + ((1 as ::core::ffi::c_int)
                            << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int)
                        >> 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int)
                }
                & ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int) as COUNT;
        }
        if (ship_delta_facing as ::core::ffi::c_int)
            < ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int)
                + ((1 as ::core::ffi::c_int)
                    << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int)
                >> 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
        {
            (*StarShipPtr).ship_input_state = ((*StarShipPtr).ship_input_state
                as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as BYTE;
            f = f.wrapping_add(1);
            (*ShipPtr).next.image.frame = IncFrameIndex((*ShipPtr).current.image.frame);
        } else {
            (*StarShipPtr).ship_input_state = ((*StarShipPtr).ship_input_state
                as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as BYTE;
            f = f.wrapping_sub(1);
            (*ShipPtr).next.image.frame = DecFrameIndex((*ShipPtr).current.image.frame);
        }
        (*StarShipPtr).ShipFacing = (f as ::core::ffi::c_int
            & ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                - 1 as ::core::ffi::c_int) as COUNT;
        return BOOLEAN::TRUE;
    }
    return BOOLEAN::FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn ThrustShip(
    mut ShipPtr: *mut ELEMENT,
    mut angle: COUNT,
) -> BOOLEAN {
    let mut ShouldThrust: BOOLEAN = BOOLEAN::FALSE;
    let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
    StarShipPtr = (*ShipPtr).pParent as *mut STARSHIP;
    if (*StarShipPtr).ship_input_state as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int != 0
    {
        ShouldThrust = BOOLEAN::TRUE;
    } else if ((angle as ::core::ffi::c_int
        + ((1 as ::core::ffi::c_int)
            << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int)
        >> 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int)
        - ((*ShipPtr).velocity.TravelAngle as ::core::ffi::c_int
            + ((1 as ::core::ffi::c_int)
                << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int)
            >> 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int)
        & ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
            - 1 as ::core::ffi::c_int) as COUNT as ::core::ffi::c_int
        == 0 as ::core::ffi::c_int
        && (*StarShipPtr).cur_status_flags as ::core::ffi::c_int
            & ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) != 0
        && (*StarShipPtr).cur_status_flags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int == 0
    {
        ShouldThrust = BOOLEAN::FALSE;
    } else {
        let mut ship_delta_facing: SIZE = 0;
        ship_delta_facing = ((angle as ::core::ffi::c_int
            + ((1 as ::core::ffi::c_int)
                << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int)
            >> 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int)
            - (*StarShipPtr).ShipFacing as ::core::ffi::c_int
            + (((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                >> 2 as ::core::ffi::c_int)
                + ((1 as ::core::ffi::c_int)
                    << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int)
                >> 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int)
            & ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                - 1 as ::core::ffi::c_int) as COUNT as SIZE;
        if ship_delta_facing as ::core::ffi::c_int
            == ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                >> 2 as ::core::ffi::c_int)
                + ((1 as ::core::ffi::c_int)
                    << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int)
                >> 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
            || (*StarShipPtr).cur_status_flags as ::core::ffi::c_int
                & (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int != 0
                && ship_delta_facing as ::core::ffi::c_int
                    <= ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                        >> 1 as ::core::ffi::c_int)
                        + ((1 as ::core::ffi::c_int)
                            << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int)
                        >> 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
        {
            ShouldThrust = BOOLEAN::TRUE;
        } else {
            ShouldThrust = BOOLEAN::FALSE;
        }
    }
    if ShouldThrust.0 != 0 {
        inertial_thrust(ShipPtr);
        (*StarShipPtr).ship_input_state = ((*StarShipPtr).ship_input_state
            as ::core::ffi::c_int | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
            as BYTE;
    }
    return ShouldThrust;
}
#[no_mangle]
pub unsafe extern "C" fn Pursue(
    mut ShipPtr: *mut ELEMENT,
    mut EvalDescPtr: *mut EVALUATE_DESC,
) {
    let mut maneuver_state: BYTE = 0;
    let mut desired_thrust_angle: COUNT = 0;
    let mut desired_turn_angle: COUNT = 0;
    let mut delta_x: SIZE = 0;
    let mut delta_y: SIZE = 0;
    let mut ship_delta_x: SIZE = 0;
    let mut ship_delta_y: SIZE = 0;
    let mut other_delta_x: SIZE = 0;
    let mut other_delta_y: SIZE = 0;
    let mut OtherObjPtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
    let mut ShipVelocity: VELOCITY_DESC = VELOCITY_DESC {
        TravelAngle: 0,
        vector: EXTENT { width: 0, height: 0 },
        fract: EXTENT { width: 0, height: 0 },
        error: EXTENT { width: 0, height: 0 },
        incr: EXTENT { width: 0, height: 0 },
    };
    let mut OtherVelocity: VELOCITY_DESC = VELOCITY_DESC {
        TravelAngle: 0,
        vector: EXTENT { width: 0, height: 0 },
        fract: EXTENT { width: 0, height: 0 },
        error: EXTENT { width: 0, height: 0 },
        incr: EXTENT { width: 0, height: 0 },
    };
    ShipVelocity = (*ShipPtr).velocity;
    GetNextVelocityComponents(
        &raw mut ShipVelocity,
        &raw mut ship_delta_x,
        &raw mut ship_delta_y,
        (*EvalDescPtr).which_turn,
    );
    (*ShipPtr).next.location.x = ((*ShipPtr).current.location.x as ::core::ffi::c_int
        + ship_delta_x as ::core::ffi::c_int) as COORD;
    (*ShipPtr).next.location.y = ((*ShipPtr).current.location.y as ::core::ffi::c_int
        + ship_delta_y as ::core::ffi::c_int) as COORD;
    OtherObjPtr = (*EvalDescPtr).ObjectPtr;
    OtherVelocity = (*OtherObjPtr).velocity;
    GetNextVelocityComponents(
        &raw mut OtherVelocity,
        &raw mut other_delta_x,
        &raw mut other_delta_y,
        (*EvalDescPtr).which_turn,
    );
    delta_x = ((*OtherObjPtr).current.location.x as ::core::ffi::c_int
        + other_delta_x as ::core::ffi::c_int
        - (*ShipPtr).next.location.x as ::core::ffi::c_int) as SIZE;
    delta_y = ((*OtherObjPtr).current.location.y as ::core::ffi::c_int
        + other_delta_y as ::core::ffi::c_int
        - (*ShipPtr).next.location.y as ::core::ffi::c_int) as SIZE;
    delta_x = (if (delta_x as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
        if -(delta_x as ::core::ffi::c_int)
            <= ((ScreenWidth - 64 as ::core::ffi::c_int
                - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int
        {
            delta_x as ::core::ffi::c_int
        } else {
            (((ScreenWidth - 64 as ::core::ffi::c_int
                - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                + delta_x as ::core::ffi::c_int
        }
    } else if delta_x as ::core::ffi::c_int
        <= ((ScreenWidth - 64 as ::core::ffi::c_int
            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
            << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
            >> 1 as ::core::ffi::c_int
    {
        delta_x as ::core::ffi::c_int
    } else {
        delta_x as ::core::ffi::c_int
            - (((ScreenWidth - 64 as ::core::ffi::c_int
                - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
    }) as SIZE;
    delta_y = (if (delta_y as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
        if -(delta_y as ::core::ffi::c_int)
            <= ((ScreenHeight - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int
        {
            delta_y as ::core::ffi::c_int
        } else {
            (((ScreenHeight - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                + delta_y as ::core::ffi::c_int
        }
    } else if delta_y as ::core::ffi::c_int
        <= ((ScreenHeight - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
            << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
            >> 1 as ::core::ffi::c_int
    {
        delta_y as ::core::ffi::c_int
    } else {
        delta_y as ::core::ffi::c_int
            - (((ScreenHeight - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
    }) as SIZE;
    desired_thrust_angle = ARCTAN(delta_x, delta_y);
    maneuver_state = 0 as BYTE;
    if (*ShipPtr).c2rust_unnamed_2.turn_wait as ::core::ffi::c_int
        == 0 as ::core::ffi::c_int
    {
        maneuver_state = (maneuver_state as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)) as BYTE;
    }
    if (*ShipPtr).c2rust_unnamed_3.thrust_wait as ::core::ffi::c_int
        == 0 as ::core::ffi::c_int
        && ((*OtherObjPtr).state_flags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int != 0
            || elementsOfSamePlayer(OtherObjPtr, ShipPtr).0 != 0
            || (*OtherObjPtr).preprocess_func
                == Some(crew_preprocess as unsafe extern "C" fn(*mut ELEMENT) -> ()))
    {
        maneuver_state = (maneuver_state as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as BYTE;
    }
    desired_turn_angle = (desired_thrust_angle as ::core::ffi::c_int
        + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
            >> 1 as ::core::ffi::c_int)
        & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
            - 1 as ::core::ffi::c_int) as COUNT;
    if (*OtherObjPtr).state_flags as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int != 0
        && (*OtherObjPtr).c2rust_unnamed_1.mass_points as ::core::ffi::c_int
            <= 10 as ::core::ffi::c_int
    {
        let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
        let mut EnemyStarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
        StarShipPtr = (*ShipPtr).pParent as *mut STARSHIP;
        EnemyStarShipPtr = (*OtherObjPtr).pParent as *mut STARSHIP;
        if (*(*StarShipPtr).RaceDescPtr).cyborg_control.ManeuverabilityIndex
            as ::core::ffi::c_int >= 150 as ::core::ffi::c_int
            && (*(*StarShipPtr).RaceDescPtr).cyborg_control.WeaponRange
                as ::core::ffi::c_int
                > (50 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
            || (*EvalDescPtr).which_turn as ::core::ffi::c_int
                >= 24 as ::core::ffi::c_int
                && (((*(*StarShipPtr).RaceDescPtr).characteristics.max_thrust
                    as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    / 3 as ::core::ffi::c_int)
                    < (*(*EnemyStarShipPtr).RaceDescPtr).characteristics.max_thrust
                        as ::core::ffi::c_int
                    || (*EnemyStarShipPtr).cur_status_flags as ::core::ffi::c_int
                        & (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int != 0)
        {
            let mut ship_flags: UWORD = 0;
            ship_flags = (*(*EnemyStarShipPtr).RaceDescPtr).ship_info.ship_flags;
            if (*(*StarShipPtr).RaceDescPtr).cyborg_control.ManeuverabilityIndex
                as ::core::ffi::c_int >= 45 as ::core::ffi::c_int
            {
                let mut fire_flags: UWORD = 0;
                let mut facing: COUNT = 0;
                fire_flags = ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int)
                    as UWORD;
                facing = (*EvalDescPtr).facing;
                while fire_flags as ::core::ffi::c_int
                    <= (1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int
                {
                    if ship_flags as ::core::ffi::c_int
                        & fire_flags as ::core::ffi::c_int != 0
                        && (desired_turn_angle as ::core::ffi::c_int
                            - facing as ::core::ffi::c_int
                            + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                                >> 3 as ::core::ffi::c_int)
                            & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                                - 1 as ::core::ffi::c_int) as COUNT as ::core::ffi::c_int
                            <= (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                                >> 2 as ::core::ffi::c_int
                        && ((other_delta_x as ::core::ffi::c_int
                            != 0 as ::core::ffi::c_int
                            || other_delta_y as ::core::ffi::c_int
                                != 0 as ::core::ffi::c_int)
                            && (OtherVelocity.TravelAngle as ::core::ffi::c_int
                                + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                                    >> 1 as ::core::ffi::c_int) - facing as ::core::ffi::c_int
                                + (((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                                    >> 3 as ::core::ffi::c_int)
                                    + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                                        >> 3 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int))
                                & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                                    - 1 as ::core::ffi::c_int) as COUNT as ::core::ffi::c_int
                                <= ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                                    >> 3 as ::core::ffi::c_int)
                                    + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                                        >> 3 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
                                    << 1 as ::core::ffi::c_int)
                    {
                        desired_thrust_angle = desired_turn_angle;
                        break;
                    } else {
                        fire_flags = ((fire_flags as ::core::ffi::c_int)
                            << 1 as ::core::ffi::c_int) as UWORD;
                        facing = (facing as ::core::ffi::c_int
                            + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                                >> 2 as ::core::ffi::c_int)) as COUNT;
                    }
                }
            }
            if desired_thrust_angle as ::core::ffi::c_int
                != desired_turn_angle as ::core::ffi::c_int
                && (other_delta_x as ::core::ffi::c_int != 0
                    || other_delta_y as ::core::ffi::c_int != 0)
                && (*EvalDescPtr).which_turn as ::core::ffi::c_int
                    >= 24 as ::core::ffi::c_int
                && (desired_thrust_angle as ::core::ffi::c_int
                    - OtherVelocity.TravelAngle as ::core::ffi::c_int
                    + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                        >> 3 as ::core::ffi::c_int)
                    & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int) as COUNT as ::core::ffi::c_int
                    <= (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                        >> 2 as ::core::ffi::c_int
                && ((OtherVelocity.TravelAngle as ::core::ffi::c_int
                    - ShipVelocity.TravelAngle as ::core::ffi::c_int
                    + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                        >> 3 as ::core::ffi::c_int)
                    & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int) as COUNT as ::core::ffi::c_int
                    <= (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                        >> 2 as ::core::ffi::c_int
                    && ((*StarShipPtr).cur_status_flags as ::core::ffi::c_int
                        & (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int != 0
                        && (*StarShipPtr).cur_status_flags as ::core::ffi::c_int
                            & (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int == 0
                        || ship_flags as ::core::ffi::c_int
                            & (1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int != 0)
                    || (desired_turn_angle as ::core::ffi::c_int
                        - (((*StarShipPtr).ShipFacing as ::core::ffi::c_int)
                            << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int)
                        + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                            >> 3 as ::core::ffi::c_int)
                        & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                            - 1 as ::core::ffi::c_int) as COUNT as ::core::ffi::c_int
                        <= (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                            >> 2 as ::core::ffi::c_int)
            {
                desired_thrust_angle = desired_turn_angle;
            }
        }
    }
    if maneuver_state as ::core::ffi::c_int
        & ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) != 0
    {
        TurnShip(ShipPtr, desired_thrust_angle);
    }
    if maneuver_state as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int != 0
    {
        ThrustShip(ShipPtr, desired_thrust_angle);
    }
}
#[no_mangle]
pub unsafe extern "C" fn Entice(
    mut ShipPtr: *mut ELEMENT,
    mut EvalDescPtr: *mut EVALUATE_DESC,
) {
    let mut maneuver_state: BYTE = 0;
    let mut desired_thrust_angle: COUNT = 0;
    let mut desired_turn_angle: COUNT = 0;
    let mut cone_of_fire: COUNT = 0;
    let mut travel_angle: COUNT = 0;
    let mut delta_x: SIZE = 0;
    let mut delta_y: SIZE = 0;
    let mut ship_delta_x: SIZE = 0;
    let mut ship_delta_y: SIZE = 0;
    let mut other_delta_x: SIZE = 0;
    let mut other_delta_y: SIZE = 0;
    let mut OtherObjPtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
    let mut ShipVelocity: VELOCITY_DESC = VELOCITY_DESC {
        TravelAngle: 0,
        vector: EXTENT { width: 0, height: 0 },
        fract: EXTENT { width: 0, height: 0 },
        error: EXTENT { width: 0, height: 0 },
        incr: EXTENT { width: 0, height: 0 },
    };
    let mut OtherVelocity: VELOCITY_DESC = VELOCITY_DESC {
        TravelAngle: 0,
        vector: EXTENT { width: 0, height: 0 },
        fract: EXTENT { width: 0, height: 0 },
        error: EXTENT { width: 0, height: 0 },
        incr: EXTENT { width: 0, height: 0 },
    };
    let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
    let mut RDPtr: *mut RACE_DESC = ::core::ptr::null_mut::<RACE_DESC>();
    ShipVelocity = (*ShipPtr).velocity;
    GetNextVelocityComponents(
        &raw mut ShipVelocity,
        &raw mut ship_delta_x,
        &raw mut ship_delta_y,
        (*EvalDescPtr).which_turn,
    );
    (*ShipPtr).next.location.x = ((*ShipPtr).current.location.x as ::core::ffi::c_int
        + ship_delta_x as ::core::ffi::c_int) as COORD;
    (*ShipPtr).next.location.y = ((*ShipPtr).current.location.y as ::core::ffi::c_int
        + ship_delta_y as ::core::ffi::c_int) as COORD;
    OtherObjPtr = (*EvalDescPtr).ObjectPtr;
    OtherVelocity = (*OtherObjPtr).velocity;
    GetNextVelocityComponents(
        &raw mut OtherVelocity,
        &raw mut other_delta_x,
        &raw mut other_delta_y,
        (*EvalDescPtr).which_turn,
    );
    delta_x = ((*OtherObjPtr).current.location.x as ::core::ffi::c_int
        + other_delta_x as ::core::ffi::c_int
        - (*ShipPtr).next.location.x as ::core::ffi::c_int) as SIZE;
    delta_y = ((*OtherObjPtr).current.location.y as ::core::ffi::c_int
        + other_delta_y as ::core::ffi::c_int
        - (*ShipPtr).next.location.y as ::core::ffi::c_int) as SIZE;
    delta_x = (if (delta_x as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
        if -(delta_x as ::core::ffi::c_int)
            <= ((ScreenWidth - 64 as ::core::ffi::c_int
                - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int
        {
            delta_x as ::core::ffi::c_int
        } else {
            (((ScreenWidth - 64 as ::core::ffi::c_int
                - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                + delta_x as ::core::ffi::c_int
        }
    } else if delta_x as ::core::ffi::c_int
        <= ((ScreenWidth - 64 as ::core::ffi::c_int
            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
            << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
            >> 1 as ::core::ffi::c_int
    {
        delta_x as ::core::ffi::c_int
    } else {
        delta_x as ::core::ffi::c_int
            - (((ScreenWidth - 64 as ::core::ffi::c_int
                - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
    }) as SIZE;
    delta_y = (if (delta_y as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
        if -(delta_y as ::core::ffi::c_int)
            <= ((ScreenHeight - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int
        {
            delta_y as ::core::ffi::c_int
        } else {
            (((ScreenHeight - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                + delta_y as ::core::ffi::c_int
        }
    } else if delta_y as ::core::ffi::c_int
        <= ((ScreenHeight - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
            << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
            >> 1 as ::core::ffi::c_int
    {
        delta_y as ::core::ffi::c_int
    } else {
        delta_y as ::core::ffi::c_int
            - (((ScreenHeight - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
    }) as SIZE;
    desired_thrust_angle = ARCTAN(delta_x, delta_y);
    maneuver_state = 0 as BYTE;
    if (*ShipPtr).c2rust_unnamed_2.turn_wait as ::core::ffi::c_int
        == 0 as ::core::ffi::c_int
    {
        maneuver_state = (maneuver_state as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)) as BYTE;
    }
    if (*ShipPtr).c2rust_unnamed_3.thrust_wait as ::core::ffi::c_int
        == 0 as ::core::ffi::c_int
    {
        maneuver_state = (maneuver_state as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as BYTE;
    }
    delta_x = (ship_delta_x as ::core::ffi::c_int - other_delta_x as ::core::ffi::c_int)
        as SIZE;
    delta_y = (ship_delta_y as ::core::ffi::c_int - other_delta_y as ::core::ffi::c_int)
        as SIZE;
    travel_angle = ARCTAN(delta_x, delta_y);
    desired_turn_angle = (desired_thrust_angle as ::core::ffi::c_int
        + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
            >> 1 as ::core::ffi::c_int)
        & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
            - 1 as ::core::ffi::c_int) as COUNT;
    StarShipPtr = (*ShipPtr).pParent as *mut STARSHIP;
    RDPtr = (*StarShipPtr).RaceDescPtr;
    '_DoManeuver: {
        if (*EvalDescPtr).MoveState.0 == MOVEMENT_STATE::AVOID.0 {
            desired_turn_angle = (desired_turn_angle as ::core::ffi::c_int
                - (*EvalDescPtr).facing as ::core::ffi::c_int
                & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int) as COUNT;
            if (desired_turn_angle as ::core::ffi::c_int
                + ((1 as ::core::ffi::c_int)
                    << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int)
                >> 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                & ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int) as COUNT != 0
            {
                if desired_turn_angle as ::core::ffi::c_int
                    <= (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                        >> 1 as ::core::ffi::c_int
                {
                    desired_thrust_angle = ((1 as ::core::ffi::c_int)
                        << 1 as ::core::ffi::c_int) as COUNT;
                } else {
                    desired_thrust_angle = ((1 as ::core::ffi::c_int)
                        << 0 as ::core::ffi::c_int) as COUNT;
                }
            } else {
                desired_turn_angle = ((((*StarShipPtr).ShipFacing as ::core::ffi::c_int)
                    << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int)
                    - (*EvalDescPtr).facing as ::core::ffi::c_int
                    & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int) as COUNT;
                if desired_turn_angle as ::core::ffi::c_int
                    & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                        >> 1 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                {
                    desired_thrust_angle = (if TFB_Random() & 1 as DWORD != 0 {
                        (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                    } else {
                        (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                    }) as COUNT;
                } else {
                    desired_thrust_angle = (if (desired_turn_angle as ::core::ffi::c_int)
                        < (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                            >> 1 as ::core::ffi::c_int
                    {
                        (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                    } else {
                        (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                    }) as COUNT;
                }
            }
            if desired_thrust_angle as ::core::ffi::c_int
                == (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            {
                desired_thrust_angle = ((*EvalDescPtr).facing as ::core::ffi::c_int
                    + -((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                        >> 2 as ::core::ffi::c_int)
                    - (-((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                        >> 3 as ::core::ffi::c_int) >> 1 as ::core::ffi::c_int))
                    as COUNT;
            } else {
                desired_thrust_angle = ((*EvalDescPtr).facing as ::core::ffi::c_int
                    + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                        >> 2 as ::core::ffi::c_int)
                    - ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                        >> 3 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)) as COUNT;
            }
            desired_thrust_angle = (desired_thrust_angle as ::core::ffi::c_int
                & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int) as COUNT;
        } else if (*OtherObjPtr).c2rust_unnamed_1.mass_points as ::core::ffi::c_int
            > 10 as ::core::ffi::c_int * 10 as ::core::ffi::c_int
        {
            let mut planet_facing: COUNT = 0;
            planet_facing = (desired_thrust_angle as ::core::ffi::c_int
                + ((1 as ::core::ffi::c_int)
                    << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int)
                >> 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                & ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int) as COUNT;
            cone_of_fire = (planet_facing as ::core::ffi::c_int
                - (*StarShipPtr).ShipFacing as ::core::ffi::c_int
                + (((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                    >> 2 as ::core::ffi::c_int)
                    + ((1 as ::core::ffi::c_int)
                        << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int)
                    >> 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int)
                & ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int) as COUNT;
            if (*RDPtr).characteristics.thrust_increment as ::core::ffi::c_int
                != (*RDPtr).characteristics.max_thrust as ::core::ffi::c_int
            {
                maneuver_state = (maneuver_state as ::core::ffi::c_int
                    & !((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)) as BYTE;
            }
            if cone_of_fire as ::core::ffi::c_int
                > (((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                    >> 2 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
                    + ((1 as ::core::ffi::c_int)
                        << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int)
                    >> 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
            {
                desired_turn_angle = desired_thrust_angle;
            } else if cone_of_fire as ::core::ffi::c_int
                == ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                    >> 2 as ::core::ffi::c_int)
                    + ((1 as ::core::ffi::c_int)
                        << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int)
                    >> 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                && (travel_angle as ::core::ffi::c_int
                    + ((1 as ::core::ffi::c_int)
                        << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int)
                    >> 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                    & ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int) as COUNT as ::core::ffi::c_int
                    != planet_facing as ::core::ffi::c_int
            {
                desired_turn_angle = travel_angle;
            } else if cone_of_fire as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                || cone_of_fire as ::core::ffi::c_int
                    == (((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                        >> 2 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
                        + ((1 as ::core::ffi::c_int)
                            << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int)
                        >> 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                || maneuver_state as ::core::ffi::c_int
                    & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int == 0
                    && ((cone_of_fire as ::core::ffi::c_int)
                        < ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                            >> 3 as ::core::ffi::c_int)
                            + ((1 as ::core::ffi::c_int)
                                << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                                    - 1 as ::core::ffi::c_int)
                            >> 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                        || cone_of_fire as ::core::ffi::c_int
                            > (((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                                >> 2 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
                                - ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                                    >> 3 as ::core::ffi::c_int)
                                + ((1 as ::core::ffi::c_int)
                                    << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                                        - 1 as ::core::ffi::c_int)
                                >> 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int)
            {
                desired_turn_angle = (((*StarShipPtr).ShipFacing as ::core::ffi::c_int)
                    << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int) as COUNT;
                if (desired_turn_angle as ::core::ffi::c_int
                    - travel_angle as ::core::ffi::c_int
                    + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                        >> 2 as ::core::ffi::c_int)
                    & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int) as COUNT as ::core::ffi::c_int
                    > (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                        >> 1 as ::core::ffi::c_int
                {
                    desired_turn_angle = travel_angle;
                }
                if (*ShipPtr).c2rust_unnamed_3.thrust_wait as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                {
                    maneuver_state = (maneuver_state as ::core::ffi::c_int
                        | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as BYTE;
                }
            }
            desired_thrust_angle = desired_turn_angle;
        } else {
            let mut WRange: COUNT = 0;
            WRange = (*RDPtr).cyborg_control.WeaponRange;
            cone_of_fire = (desired_turn_angle as ::core::ffi::c_int
                - (*EvalDescPtr).facing as ::core::ffi::c_int
                + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                    >> 3 as ::core::ffi::c_int)
                & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int) as COUNT;
            if (*OtherObjPtr).state_flags as ::core::ffi::c_int
                & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int != 0
            {
                let mut fire_flags: UWORD = 0;
                let mut ship_flags: UWORD = 0;
                let mut facing: COUNT = 0;
                let mut EnemyStarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<
                    STARSHIP,
                >();
                EnemyStarShipPtr = (*OtherObjPtr).pParent as *mut STARSHIP;
                ship_flags = (*(*EnemyStarShipPtr).RaceDescPtr).ship_info.ship_flags;
                fire_flags = ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int)
                    as UWORD;
                facing = (*EvalDescPtr).facing;
                while fire_flags as ::core::ffi::c_int
                    <= (1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int
                {
                    if ship_flags as ::core::ffi::c_int
                        & fire_flags as ::core::ffi::c_int != 0
                        && {
                            cone_of_fire = (desired_turn_angle as ::core::ffi::c_int
                                - facing as ::core::ffi::c_int
                                + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                                    >> 3 as ::core::ffi::c_int)
                                & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                                    - 1 as ::core::ffi::c_int) as COUNT;
                            cone_of_fire as ::core::ffi::c_int
                                <= (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                                    >> 2 as ::core::ffi::c_int
                        }
                        && ((other_delta_x as ::core::ffi::c_int
                            != 0 as ::core::ffi::c_int
                            || other_delta_y as ::core::ffi::c_int
                                != 0 as ::core::ffi::c_int)
                            && (OtherVelocity.TravelAngle as ::core::ffi::c_int
                                + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                                    >> 1 as ::core::ffi::c_int) - facing as ::core::ffi::c_int
                                + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                                    >> 3 as ::core::ffi::c_int)
                                & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                                    - 1 as ::core::ffi::c_int) as COUNT as ::core::ffi::c_int
                                <= (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                                    >> 2 as ::core::ffi::c_int)
                    {
                        if !((WRange as ::core::ffi::c_int)
                            < (1000 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
                            && (*EvalDescPtr).which_turn as ::core::ffi::c_int
                                <= 32 as ::core::ffi::c_int)
                        {
                            break;
                        }
                        desired_thrust_angle = desired_turn_angle;
                        break '_DoManeuver;
                    } else {
                        fire_flags = ((fire_flags as ::core::ffi::c_int)
                            << 1 as ::core::ffi::c_int) as UWORD;
                        facing = (facing as ::core::ffi::c_int
                            + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                                >> 2 as ::core::ffi::c_int)) as COUNT;
                    }
                }
                if (*EvalDescPtr).which_turn as ::core::ffi::c_int
                    <= 8 as ::core::ffi::c_int
                    && (*RDPtr).characteristics.max_thrust as ::core::ffi::c_int
                        <= (*(*EnemyStarShipPtr).RaceDescPtr).characteristics.max_thrust
                            as ::core::ffi::c_int
                {
                    break '_DoManeuver;
                }
            }
            if (*StarShipPtr).cur_status_flags as ::core::ffi::c_int
                & ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) == 0
                && PlotIntercept(
                    ShipPtr,
                    OtherObjPtr,
                    40 as COUNT,
                    (((50 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                        << 1 as ::core::ffi::c_int) as COUNT,
                ) as ::core::ffi::c_int != 0
            {
                if (desired_thrust_angle as ::core::ffi::c_int
                    - (((*StarShipPtr).ShipFacing as ::core::ffi::c_int)
                        << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int)
                    + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                        >> 3 as ::core::ffi::c_int)
                    & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int) as COUNT as ::core::ffi::c_int
                    <= (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                        >> 2 as ::core::ffi::c_int
                    || cone_of_fire as ::core::ffi::c_int
                        > (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                            >> 2 as ::core::ffi::c_int
                {
                    desired_thrust_angle = desired_turn_angle;
                } else {
                    desired_thrust_angle = (((*StarShipPtr).ShipFacing
                        as ::core::ffi::c_int)
                        << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int) as COUNT;
                    desired_turn_angle = desired_thrust_angle;
                }
            } else {
                cone_of_fire = PlotIntercept(
                    ShipPtr,
                    OtherObjPtr,
                    10 as COUNT,
                    (WRange as ::core::ffi::c_int
                        - (WRange as ::core::ffi::c_int >> 2 as ::core::ffi::c_int))
                        as COUNT,
                );
                if cone_of_fire != 0 {
                    if (*RDPtr).characteristics.thrust_increment as ::core::ffi::c_int
                        != (*RDPtr).characteristics.max_thrust as ::core::ffi::c_int
                        && (*StarShipPtr).cur_status_flags as ::core::ffi::c_int
                            & ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int
                                | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) != 0
                        && ((desired_turn_angle as ::core::ffi::c_int
                            - ARCTAN(ship_delta_x, ship_delta_y) as ::core::ffi::c_int
                            + (((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                                >> 3 as ::core::ffi::c_int) + 2 as ::core::ffi::c_int)
                            & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                                - 1 as ::core::ffi::c_int) as COUNT as ::core::ffi::c_int
                            <= (((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                                >> 3 as ::core::ffi::c_int) + 2 as ::core::ffi::c_int)
                                << 1 as ::core::ffi::c_int
                            || PlotIntercept(
                                ShipPtr,
                                OtherObjPtr,
                                30 as COUNT,
                                (((50 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                                    << 1 as ::core::ffi::c_int) as COUNT,
                            ) == 0)
                    {
                        maneuver_state = (maneuver_state as ::core::ffi::c_int
                            & !((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int))
                            as BYTE;
                    } else if cone_of_fire as ::core::ffi::c_int
                        == 1 as ::core::ffi::c_int
                        || (*RDPtr).characteristics.thrust_increment
                            as ::core::ffi::c_int
                            != (*RDPtr).characteristics.max_thrust as ::core::ffi::c_int
                    {
                        if maneuver_state as ::core::ffi::c_int
                            & ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) != 0
                        {
                            TurnShip(ShipPtr, desired_turn_angle);
                            maneuver_state = (maneuver_state as ::core::ffi::c_int
                                & !((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                                    | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int))
                                as BYTE;
                        }
                        if (desired_thrust_angle as ::core::ffi::c_int
                            - ARCTAN(ship_delta_x, ship_delta_y) as ::core::ffi::c_int
                            + (((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                                >> 3 as ::core::ffi::c_int) + 2 as ::core::ffi::c_int)
                            & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                                - 1 as ::core::ffi::c_int) as COUNT as ::core::ffi::c_int
                            <= (((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                                >> 3 as ::core::ffi::c_int) + 2 as ::core::ffi::c_int)
                                << 1 as ::core::ffi::c_int
                        {
                            desired_thrust_angle = (((*StarShipPtr).ShipFacing
                                as ::core::ffi::c_int)
                                << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int)
                                as COUNT;
                        } else {
                            desired_thrust_angle = desired_turn_angle;
                        }
                    }
                }
            }
        }
    }
    if maneuver_state as ::core::ffi::c_int
        & ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) != 0
    {
        TurnShip(ShipPtr, desired_thrust_angle);
    }
    if maneuver_state as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int != 0
    {
        ThrustShip(ShipPtr, desired_thrust_angle);
    }
}
#[no_mangle]
pub unsafe extern "C" fn Avoid(
    mut ShipPtr: *mut ELEMENT,
    mut EvalDescPtr: *mut EVALUATE_DESC,
) {}
#[no_mangle]
pub unsafe extern "C" fn tactical_intelligence(
    mut context: *mut ComputerInputContext,
    mut StarShipPtr: *mut STARSHIP,
) -> BATTLE_INPUT_STATE {
    let mut ShipPtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
    let mut Ship: ELEMENT = ELEMENT {
        pred: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        succ: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        preprocess_func: None,
        postprocess_func: None,
        collision_func: None,
        death_func: None,
        playerNr: 0,
        state_flags: 0,
        c2rust_unnamed: C2Rust_Unnamed_4 { life_span: 0 },
        c2rust_unnamed_0: C2Rust_Unnamed_3 { crew_level: 0 },
        c2rust_unnamed_1: C2Rust_Unnamed_2 { mass_points: 0 },
        c2rust_unnamed_2: C2Rust_Unnamed_1 { turn_wait: 0 },
        c2rust_unnamed_3: C2Rust_Unnamed_0 { thrust_wait: 0 },
        colorCycleIndex: 0,
        velocity: VELOCITY_DESC {
            TravelAngle: 0,
            vector: EXTENT { width: 0, height: 0 },
            fract: EXTENT { width: 0, height: 0 },
            error: EXTENT { width: 0, height: 0 },
            incr: EXTENT { width: 0, height: 0 },
        },
        IntersectControl: INTERSECT_CONTROL {
            last_time_val: 0,
            EndPoint: POINT { x: 0, y: 0 },
            IntersectStamp: STAMP {
                origin: POINT { x: 0, y: 0 },
                frame: ::core::ptr::null_mut::<FRAME_DESC>(),
            },
        },
        PrimIndex: 0,
        current: STATE {
            location: POINT { x: 0, y: 0 },
            image: C2Rust_Unnamed {
                frame: ::core::ptr::null_mut::<FRAME_DESC>(),
                farray: ::core::ptr::null_mut::<FRAME>(),
            },
        },
        next: STATE {
            location: POINT { x: 0, y: 0 },
            image: C2Rust_Unnamed {
                frame: ::core::ptr::null_mut::<FRAME_DESC>(),
                farray: ::core::ptr::null_mut::<FRAME>(),
            },
        },
        pParent: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        hTarget: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    let mut ShipFacing: COUNT = 0;
    let mut hElement: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut hNextElement: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut ConcernCounter: COUNT = 0;
    let mut ObjectsOfConcern: [EVALUATE_DESC; 10] = [EVALUATE_DESC {
        ObjectPtr: ::core::ptr::null_mut::<ELEMENT>(),
        facing: 0,
        which_turn: 0,
        MoveState: MOVEMENT_STATE::PURSUE,
    }; 10];
    let mut ShipMoved: BOOLEAN = BOOLEAN::FALSE;
    let mut UltraManeuverable: BOOLEAN = BOOLEAN::FALSE;
    let mut EnemyStarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
    let mut RDPtr: *mut RACE_DESC = ::core::ptr::null_mut::<RACE_DESC>();
    let mut EnemyRDPtr: *mut RACE_DESC = ::core::ptr::null_mut::<RACE_DESC>();
    RDPtr = (*StarShipPtr).RaceDescPtr;
    if (*RDPtr).cyborg_control.ManeuverabilityIndex as ::core::ffi::c_int
        == 0 as ::core::ffi::c_int
    {
        InitCyborg(StarShipPtr);
    }
    ShipPtr = LockLink(&raw mut disp_q, (*StarShipPtr).hShip) as *mut ELEMENT;
    if (*RDPtr).ship_info.crew_level as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        || DisplayArray[(*ShipPtr).PrimIndex as usize].Type as ::core::ffi::c_int
            == gfx_object::NUM_PRIMS.0 as ::core::ffi::c_int
    {
        UnlockLink(&raw mut disp_q, (*StarShipPtr).hShip);
        return 0 as BATTLE_INPUT_STATE;
    }
    ShipMoved = BOOLEAN::TRUE;
    if (*StarShipPtr).control as ::core::ffi::c_int
        & ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as BYTE
            as ::core::ffi::c_int != 0
    {
        (*StarShipPtr).special_counter = (*StarShipPtr).special_counter.wrapping_add(1);
    }
    Ship = *ShipPtr;
    UnlockLink(&raw mut disp_q, (*StarShipPtr).hShip);
    ShipFacing = (*StarShipPtr).ShipFacing;
    ConcernCounter = 0 as COUNT;
    while ConcernCounter as ::core::ffi::c_int
        <= C2Rust_Unnamed_5::FIRST_EMPTY_INDEX.0 as ::core::ffi::c_int
    {
        ObjectsOfConcern[ConcernCounter as usize].ObjectPtr = ::core::ptr::null_mut::<
            ELEMENT,
        >();
        ObjectsOfConcern[ConcernCounter as usize].MoveState = MOVEMENT_STATE::NO_MOVEMENT;
        ObjectsOfConcern[ConcernCounter as usize].which_turn = !(0 as ::core::ffi::c_int)
            as COUNT;
        ConcernCounter = ConcernCounter.wrapping_add(1);
    }
    ConcernCounter = ConcernCounter.wrapping_sub(1);
    UltraManeuverable = BOOLEAN(
        ((*RDPtr).characteristics.thrust_increment as ::core::ffi::c_int
            == (*RDPtr).characteristics.max_thrust as ::core::ffi::c_int
            && (*RDPtr).cyborg_control.ManeuverabilityIndex as ::core::ffi::c_int
                >= 45 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
    if Ship.c2rust_unnamed_2.turn_wait as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        ShipMoved = BOOLEAN::FALSE;
        (*StarShipPtr).ship_input_state = ((*StarShipPtr).ship_input_state
            as ::core::ffi::c_int
            & !((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)) as BYTE;
    }
    if Ship.c2rust_unnamed_3.thrust_wait as ::core::ffi::c_int == 0 as ::core::ffi::c_int
    {
        ShipMoved = BOOLEAN::FALSE;
        (*StarShipPtr).ship_input_state = ((*StarShipPtr).ship_input_state
            as ::core::ffi::c_int
            & !((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)) as BYTE;
    }
    hElement = disp_q.head as HELEMENT;
    while !hElement.is_null() {
        let mut ed: EVALUATE_DESC = EVALUATE_DESC {
            ObjectPtr: ::core::ptr::null_mut::<ELEMENT>(),
            facing: 0,
            which_turn: 0,
            MoveState: MOVEMENT_STATE::PURSUE,
        };
        ed.MoveState = MOVEMENT_STATE::NO_MOVEMENT;
        ed.ObjectPtr = LockLink(&raw mut disp_q, hElement) as *mut ELEMENT;
        hNextElement = (*ed.ObjectPtr).succ;
        's_131: {
            if (*ed.ObjectPtr).state_flags as ::core::ffi::c_int
                & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) == 0
                && (Ship.state_flags as ::core::ffi::c_int
                    & (*ed.ObjectPtr).state_flags as ::core::ffi::c_int
                    & (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int == 0
                    && (Ship.state_flags as ::core::ffi::c_int
                        & (*ed.ObjectPtr).state_flags as ::core::ffi::c_int
                        & (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int == 0
                        || Ship.pParent != (*ed.ObjectPtr).pParent)
                    && (Ship.c2rust_unnamed_1.mass_points as ::core::ffi::c_int != 0
                        || (*ed.ObjectPtr).c2rust_unnamed_1.mass_points
                            as ::core::ffi::c_int != 0))
            {
                let mut dx: SIZE = 0;
                let mut dy: SIZE = 0;
                dx = ((*ed.ObjectPtr).next.location.x as ::core::ffi::c_int
                    - Ship.next.location.x as ::core::ffi::c_int) as SIZE;
                dy = ((*ed.ObjectPtr).next.location.y as ::core::ffi::c_int
                    - Ship.next.location.y as ::core::ffi::c_int) as SIZE;
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
                        (((ScreenHeight
                            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                            << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                            + dy as ::core::ffi::c_int
                    }
                } else if dy as ::core::ffi::c_int
                    <= ((ScreenHeight
                        - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
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
                if (*ed.ObjectPtr).c2rust_unnamed_1.mass_points as ::core::ffi::c_int
                    > 10 as ::core::ffi::c_int * 10 as ::core::ffi::c_int
                {
                    let mut maneuver_turn: COUNT = 0;
                    let mut ship_bounds: COUNT = 0;
                    let mut ship_footprint: RECT = rect {
                        corner: point {
                            x: 0 as COORD,
                            y: 0 as COORD,
                        },
                        extent: extent {
                            width: 0 as COORD,
                            height: 0 as COORD,
                        },
                    };
                    if UltraManeuverable.0 != 0 {
                        maneuver_turn = 16 as COUNT;
                    } else if (*RDPtr).cyborg_control.ManeuverabilityIndex
                        as ::core::ffi::c_int <= 45 as ::core::ffi::c_int
                    {
                        maneuver_turn = 48 as COUNT;
                    } else {
                        maneuver_turn = 32 as COUNT;
                    }
                    GetFrameRect(
                        SetAbsFrameIndex(
                            Ship.IntersectControl.IntersectStamp.frame,
                            0 as COUNT,
                        ),
                        &raw mut ship_footprint,
                    );
                    ship_bounds = (ship_footprint.extent.width as ::core::ffi::c_int
                        + ship_footprint.extent.height as ::core::ffi::c_int) as COUNT;
                    if ShipMoved.0 == 0
                        && {
                            ed.which_turn = PlotIntercept(
                                ed.ObjectPtr,
                                &raw mut Ship,
                                maneuver_turn,
                                ((30 as ::core::ffi::c_int
                                    + ship_bounds as ::core::ffi::c_int
                                        * 3 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                                    as COUNT,
                            );
                            ed.which_turn as ::core::ffi::c_int != 0
                        }
                    {
                        if ed.which_turn as ::core::ffi::c_int > 1 as ::core::ffi::c_int
                            || PlotIntercept(
                                ed.ObjectPtr,
                                &raw mut Ship,
                                1 as COUNT,
                                ((35 as ::core::ffi::c_int
                                    + ship_bounds as ::core::ffi::c_int)
                                    << 2 as ::core::ffi::c_int) as COUNT,
                            ) as ::core::ffi::c_int != 0
                            || PlotIntercept(
                                ed.ObjectPtr,
                                &raw mut Ship,
                                ((maneuver_turn as ::core::ffi::c_int)
                                    << 1 as ::core::ffi::c_int) as COUNT,
                                ((40 as ::core::ffi::c_int
                                    + ship_bounds as ::core::ffi::c_int)
                                    << 2 as ::core::ffi::c_int) as COUNT,
                            ) as ::core::ffi::c_int > 1 as ::core::ffi::c_int
                        {
                            ed.facing = ARCTAN(
                                -(dx as ::core::ffi::c_int) as SIZE,
                                -(dy as ::core::ffi::c_int) as SIZE,
                            );
                            if UltraManeuverable.0 != 0 {
                                ed.MoveState = MOVEMENT_STATE::AVOID;
                            } else {
                                ed.MoveState = MOVEMENT_STATE::ENTICE;
                            }
                            ObjectsOfConcern[C2Rust_Unnamed_5::GRAVITY_MASS_INDEX.0
                                as ::core::ffi::c_int as usize] = ed;
                        } else if UltraManeuverable.0 == 0
                            && !IsVelocityZero(&raw mut Ship.velocity)
                        {
                            Ship.c2rust_unnamed_3.thrust_wait = Ship
                                .c2rust_unnamed_3
                                .thrust_wait
                                .wrapping_add(1);
                            if Ship.c2rust_unnamed_2.turn_wait != 0 {
                                ShipMoved = BOOLEAN::TRUE;
                            }
                        }
                    }
                } else if (*ed.ObjectPtr).state_flags as ::core::ffi::c_int
                    & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int != 0
                {
                    EnemyStarShipPtr = (*ed.ObjectPtr).pParent as *mut STARSHIP;
                    EnemyRDPtr = (*EnemyStarShipPtr).RaceDescPtr;
                    if (*EnemyRDPtr).cyborg_control.ManeuverabilityIndex
                        as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                    {
                        InitCyborg(EnemyStarShipPtr);
                    }
                    ed.which_turn = (square_root(
                        (dx as ::core::ffi::c_long * dx as ::core::ffi::c_long
                            + dy as ::core::ffi::c_long * dy as ::core::ffi::c_long)
                            as DWORD,
                    ) as ::core::ffi::c_int >> 6 as ::core::ffi::c_int) as COUNT;
                    if ed.which_turn as ::core::ffi::c_int
                        > ObjectsOfConcern[C2Rust_Unnamed_5::ENEMY_SHIP_INDEX.0
                                as ::core::ffi::c_int as usize]
                            .which_turn as ::core::ffi::c_int
                    {
                        UnlockLink(&raw mut disp_q, hElement);
                        break 's_131;
                    } else {
                        if ed.which_turn as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                        {
                            ed.which_turn = 1 as COUNT;
                        }
                        ObjectsOfConcern[C2Rust_Unnamed_5::ENEMY_SHIP_INDEX.0
                                as ::core::ffi::c_int as usize]
                            .ObjectPtr = ed.ObjectPtr;
                        ObjectsOfConcern[C2Rust_Unnamed_5::ENEMY_SHIP_INDEX.0
                                as ::core::ffi::c_int as usize]
                            .facing = (((*EnemyStarShipPtr).ShipFacing
                            as ::core::ffi::c_int)
                            << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int)
                            as COUNT;
                        ObjectsOfConcern[C2Rust_Unnamed_5::ENEMY_SHIP_INDEX.0
                                as ::core::ffi::c_int as usize]
                            .which_turn = ed.which_turn;
                        if ShipMoved.0 != 0
                            || (*ed.ObjectPtr).c2rust_unnamed_1.mass_points
                                as ::core::ffi::c_int > 10 as ::core::ffi::c_int
                            || ((*RDPtr).cyborg_control.WeaponRange
                                as ::core::ffi::c_int)
                                < (1000 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
                                && ((*RDPtr).cyborg_control.WeaponRange
                                    as ::core::ffi::c_int
                                    <= (50 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
                                    || (*EnemyRDPtr).cyborg_control.WeaponRange
                                        as ::core::ffi::c_int
                                        >= (1000 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
                                        && (*(*EnemyStarShipPtr).RaceDescPtr).ship_info.ship_flags
                                            as ::core::ffi::c_int
                                            & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int != 0
                                    || ((*RDPtr).characteristics.max_thrust
                                        as ::core::ffi::c_int)
                                        < (*EnemyRDPtr).characteristics.max_thrust
                                            as ::core::ffi::c_int
                                        && ((*RDPtr).cyborg_control.WeaponRange
                                            as ::core::ffi::c_int)
                                            < (*EnemyRDPtr).cyborg_control.WeaponRange
                                                as ::core::ffi::c_int)
                        {
                            ObjectsOfConcern[C2Rust_Unnamed_5::ENEMY_SHIP_INDEX.0
                                    as ::core::ffi::c_int as usize]
                                .MoveState = MOVEMENT_STATE::PURSUE;
                        } else {
                            ObjectsOfConcern[C2Rust_Unnamed_5::ENEMY_SHIP_INDEX.0
                                    as ::core::ffi::c_int as usize]
                                .MoveState = MOVEMENT_STATE::ENTICE;
                        }
                        if (*(*EnemyStarShipPtr).RaceDescPtr).ship_info.ship_flags
                            as ::core::ffi::c_int
                            & (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int != 0
                            && ship_weapons(ed.ObjectPtr, &raw mut Ship, 0 as COUNT).0
                                != 0
                        {
                            ed.which_turn = 1 as COUNT;
                            ed.MoveState = MOVEMENT_STATE::AVOID;
                            ed.facing = ObjectsOfConcern[C2Rust_Unnamed_5::ENEMY_SHIP_INDEX
                                    .0 as ::core::ffi::c_int as usize]
                                .facing;
                            ObjectsOfConcern[C2Rust_Unnamed_5::ENEMY_WEAPON_INDEX.0
                                as ::core::ffi::c_int as usize] = ed;
                        }
                    }
                } else if (*ed.ObjectPtr).pParent.is_null() {
                    if (*ed.ObjectPtr).state_flags as ::core::ffi::c_int
                        & (1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int == 0
                    {
                        ed.which_turn = (square_root(
                            (dx as ::core::ffi::c_long * dx as ::core::ffi::c_long
                                + dy as ::core::ffi::c_long * dy as ::core::ffi::c_long)
                                as DWORD,
                        ) as ::core::ffi::c_int >> 6 as ::core::ffi::c_int) as COUNT;
                        if (ed.which_turn as ::core::ffi::c_int)
                            < ObjectsOfConcern[C2Rust_Unnamed_5::FIRST_EMPTY_INDEX.0
                                    as ::core::ffi::c_int as usize]
                                .which_turn as ::core::ffi::c_int
                        {
                            ed.MoveState = MOVEMENT_STATE::PURSUE;
                            ed.facing = (*ed.ObjectPtr).velocity.TravelAngle;
                            ObjectsOfConcern[C2Rust_Unnamed_5::FIRST_EMPTY_INDEX.0
                                as ::core::ffi::c_int as usize] = ed;
                        }
                    }
                } else if elementsOfSamePlayer(ed.ObjectPtr, &raw mut Ship).0 == 0
                    && (*ed.ObjectPtr).preprocess_func
                        != Some(
                            crew_preprocess as unsafe extern "C" fn(*mut ELEMENT) -> (),
                        )
                    && ObjectsOfConcern[C2Rust_Unnamed_5::ENEMY_WEAPON_INDEX.0
                            as ::core::ffi::c_int as usize]
                        .which_turn as ::core::ffi::c_int > 1 as ::core::ffi::c_int
                    && (*ed.ObjectPtr).c2rust_unnamed.life_span as ::core::ffi::c_int
                        > 0 as ::core::ffi::c_int
                {
                    EnemyStarShipPtr = (*ed.ObjectPtr).pParent as *mut STARSHIP;
                    EnemyRDPtr = (*EnemyStarShipPtr).RaceDescPtr;
                    if (*EnemyRDPtr).ship_info.ship_flags as ::core::ffi::c_int
                        & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int != 0
                        && (*ed.ObjectPtr).next.image.farray
                            != &raw mut (*EnemyRDPtr).ship_data.special as *mut FRAME
                        || (*EnemyRDPtr).ship_info.ship_flags as ::core::ffi::c_int
                            & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int != 0
                            && (*ed.ObjectPtr).next.image.farray
                                == &raw mut (*EnemyRDPtr).ship_data.special as *mut FRAME
                    {
                        if (*ed.ObjectPtr).state_flags as ::core::ffi::c_int
                            & ((1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int
                                | (1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int)
                            == 0
                            && (*RDPtr).characteristics.max_thrust as ::core::ffi::c_int
                                > (8 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
                            || ((*ed.ObjectPtr).velocity.TravelAngle
                                as ::core::ffi::c_int
                                - ARCTAN(
                                    -(dx as ::core::ffi::c_int) as SIZE,
                                    -(dy as ::core::ffi::c_int) as SIZE,
                                ) as ::core::ffi::c_int
                                + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                                    >> 2 as ::core::ffi::c_int)
                                & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                                    - 1 as ::core::ffi::c_int) as COUNT as ::core::ffi::c_int
                                > (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                                    >> 1 as ::core::ffi::c_int
                        {
                            ed.which_turn = 0 as COUNT;
                        } else {
                            ed.which_turn = (square_root(
                                (dx as ::core::ffi::c_long * dx as ::core::ffi::c_long
                                    + dy as ::core::ffi::c_long * dy as ::core::ffi::c_long)
                                    as DWORD,
                            ) as ::core::ffi::c_int >> 6 as ::core::ffi::c_int) as COUNT;
                            ed.MoveState = MOVEMENT_STATE::ENTICE;
                            if UltraManeuverable.0 != 0 {
                                if ed.which_turn as ::core::ffi::c_int
                                    == 0 as ::core::ffi::c_int
                                {
                                    ed.which_turn = 1 as COUNT;
                                } else if ed.which_turn as ::core::ffi::c_int
                                    > 16 as ::core::ffi::c_int
                                {
                                    ed.which_turn = 0 as COUNT;
                                }
                            } else if ed.which_turn as ::core::ffi::c_int
                                == 0 as ::core::ffi::c_int
                            {
                                ed.which_turn = 1 as COUNT;
                            } else if ed.which_turn as ::core::ffi::c_int
                                > 16 as ::core::ffi::c_int
                                || (*RDPtr).cyborg_control.ManeuverabilityIndex
                                    as ::core::ffi::c_int > 45 as ::core::ffi::c_int
                                    && ed.which_turn as ::core::ffi::c_int
                                        > 8 as ::core::ffi::c_int
                            {
                                ed.which_turn = 0 as COUNT;
                            }
                        }
                    } else if (*StarShipPtr).control as ::core::ffi::c_int
                        & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as BYTE
                            as ::core::ffi::c_int == 0
                    {
                        ed.which_turn = 0 as COUNT;
                    } else {
                        ed.which_turn = PlotIntercept(
                            ed.ObjectPtr,
                            &raw mut Ship,
                            (*ed.ObjectPtr).c2rust_unnamed.life_span,
                            ((40 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                                as COUNT,
                        );
                        ed.MoveState = MOVEMENT_STATE::AVOID;
                    }
                    if ed.which_turn as ::core::ffi::c_int > 0 as ::core::ffi::c_int
                        && ((ed.which_turn as ::core::ffi::c_int)
                            < ObjectsOfConcern[C2Rust_Unnamed_5::ENEMY_WEAPON_INDEX.0
                                    as ::core::ffi::c_int as usize]
                                .which_turn as ::core::ffi::c_int
                            || ed.which_turn as ::core::ffi::c_int
                                == ObjectsOfConcern[C2Rust_Unnamed_5::ENEMY_WEAPON_INDEX.0
                                        as ::core::ffi::c_int as usize]
                                    .which_turn as ::core::ffi::c_int
                                && ed.MoveState.0 == MOVEMENT_STATE::AVOID.0)
                    {
                        ed.facing = (*ed.ObjectPtr).velocity.TravelAngle;
                        ObjectsOfConcern[C2Rust_Unnamed_5::ENEMY_WEAPON_INDEX.0
                            as ::core::ffi::c_int as usize] = ed;
                    }
                } else if (*ed.ObjectPtr).state_flags as ::core::ffi::c_int
                    & (1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int != 0
                    && ((*ed.ObjectPtr).state_flags as ::core::ffi::c_int
                        & (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int == 0
                        && elementsOfSamePlayer(ed.ObjectPtr, &raw mut Ship).0 != 0
                        || (*ed.ObjectPtr).preprocess_func
                            == Some(
                                crew_preprocess as unsafe extern "C" fn(*mut ELEMENT) -> (),
                            ))
                    && ObjectsOfConcern[C2Rust_Unnamed_5::CREW_OBJECT_INDEX.0
                            as ::core::ffi::c_int as usize]
                        .which_turn as ::core::ffi::c_int > 1 as ::core::ffi::c_int
                {
                    ed.which_turn = (square_root(
                        (dx as ::core::ffi::c_long * dx as ::core::ffi::c_long
                            + dy as ::core::ffi::c_long * dy as ::core::ffi::c_long)
                            as DWORD,
                    ) as ::core::ffi::c_int >> 6 as ::core::ffi::c_int) as COUNT;
                    if ed.which_turn as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                        ed.which_turn = 1 as COUNT;
                    }
                    if ObjectsOfConcern[C2Rust_Unnamed_5::CREW_OBJECT_INDEX.0
                            as ::core::ffi::c_int as usize]
                        .which_turn as ::core::ffi::c_int
                        > ed.which_turn as ::core::ffi::c_int
                        && (ObjectsOfConcern[C2Rust_Unnamed_5::ENEMY_SHIP_INDEX.0
                                as ::core::ffi::c_int as usize]
                            .which_turn as ::core::ffi::c_int > 32 as ::core::ffi::c_int
                            || ObjectsOfConcern[C2Rust_Unnamed_5::ENEMY_SHIP_INDEX.0
                                    as ::core::ffi::c_int as usize]
                                .which_turn as ::core::ffi::c_int > 8 as ::core::ffi::c_int
                                && (*StarShipPtr).hShip == (*ed.ObjectPtr).hTarget)
                    {
                        ed.MoveState = MOVEMENT_STATE::PURSUE;
                        ed.facing = 0 as COUNT;
                        ObjectsOfConcern[C2Rust_Unnamed_5::CREW_OBJECT_INDEX.0
                            as ::core::ffi::c_int as usize] = ed;
                    }
                }
            }
            UnlockLink(&raw mut disp_q, hElement);
        }
        hElement = hNextElement;
    }
    (*RDPtr)
        .cyborg_control
        .intelligence_func
        .expect(
            "non-null function pointer",
        )(
        &raw mut Ship,
        &raw mut ObjectsOfConcern as *mut EVALUATE_DESC,
        ConcernCounter,
    );
    (*StarShipPtr).ShipFacing = ShipFacing;
    let mut InputState: BATTLE_INPUT_STATE = 0;
    InputState = 0 as BATTLE_INPUT_STATE;
    if (*StarShipPtr).ship_input_state as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int != 0
    {
        InputState = (InputState as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int)
                as BATTLE_INPUT_STATE as ::core::ffi::c_int) as BATTLE_INPUT_STATE;
    } else if (*StarShipPtr).ship_input_state as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int != 0
    {
        InputState = (InputState as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
                as BATTLE_INPUT_STATE as ::core::ffi::c_int) as BATTLE_INPUT_STATE;
    }
    if (*StarShipPtr).ship_input_state as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int != 0
    {
        InputState = (InputState as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                as BATTLE_INPUT_STATE as ::core::ffi::c_int) as BATTLE_INPUT_STATE;
    }
    if (*StarShipPtr).ship_input_state as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int != 0
    {
        InputState = (InputState as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                as BATTLE_INPUT_STATE as ::core::ffi::c_int) as BATTLE_INPUT_STATE;
    }
    if (*StarShipPtr).ship_input_state as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int != 0
    {
        InputState = (InputState as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                as BATTLE_INPUT_STATE as ::core::ffi::c_int) as BATTLE_INPUT_STATE;
    }
    return InputState;
}
