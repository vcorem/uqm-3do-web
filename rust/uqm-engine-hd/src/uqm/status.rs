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

// Rust translation of src/uqm/status.c.
extern "C" {
    pub type string_table_entry;
    pub type frame_desc;
    pub type font_desc;
    pub type tfb_soundsample;
    fn SetContextForeGroundColor(Color: Color) -> Color;
    fn DrawStamp(pStamp: *mut STAMP);
    fn DrawRectangle(pRect: *mut RECT, scaled: BOOLEAN);
    fn DrawFilledRectangle(pRect: *mut RECT);
    fn font_DrawText(pText: *mut TEXT);
    fn BatchGraphics();
    fn UnbatchGraphics();
    fn SetContextFont(Font: FONT) -> FONT;
    fn SetAbsFrameIndex(Frame: FRAME, FrameIndex: COUNT) -> FRAME;
    fn SetRelFrameIndex(Frame: FRAME, FrameOffs: SIZE) -> FRAME;
    fn IncFrameIndex(Frame: FRAME) -> FRAME;
    fn DecFrameIndex(Frame: FRAME) -> FRAME;
    fn __assert_fail(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    ) -> !;
    fn sprintf(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    static mut optWhichMenu: ::core::ffi::c_int;
    static mut resolutionFactor: ::core::ffi::c_uint;
    static mut optGodModes: ::core::ffi::c_int;
    static mut optWindowType: DWORD;
    static mut CanvasHeight: ::core::ffi::c_int;
    fn DrawCrewFuelString(y: COORD, state: SIZE);
    fn DeltaStatistics(
        ShipInfoPtr: *mut SHIP_INFO,
        y_offs: COORD,
        crew_delta: SIZE,
        energy_delta: SIZE,
    );
    static mut GlobData: GLOBDATA;
    fn isPC(optWhich: ::core::ffi::c_int) -> OPT_CONSOLETYPE;
    static mut StarConFont: FONT;
    static mut TinyFont: FONT;
    static mut TinyFontBold: FONT;
    static mut PlayerControl: [BYTE; 0];
    fn DrawRenderedBox(
        r: *mut RECT,
        filled: BOOLEAN,
        fill_color: Color,
        r#type: ::core::ffi::c_int,
        custom: ::core::ffi::c_int,
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
pub type FRAME_DESC = frame_desc;
pub type FONT_DESC = font_desc;
pub type FRAME = *mut FRAME_DESC;
pub type FONT = *mut FONT_DESC;
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct OPT_CONSOLETYPE(pub ::core::ffi::c_uint);
impl OPT_CONSOLETYPE {
    pub const OPTVAL_PC: Self = Self(0);
    pub const OPTVAL_3DO: Self = Self(1);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed {
    pub const OPTVAL_NO_GOD: Self = Self(0);
    pub const OPTVAL_INF_ENERGY: Self = Self(1);
    pub const OPTVAL_INF_HEALTH: Self = Self(2);
    pub const OPTVAL_FULL_GOD: Self = Self(3);
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
    pub c2rust_unnamed: C2Rust_Unnamed_5,
    pub c2rust_unnamed_0: C2Rust_Unnamed_4,
    pub c2rust_unnamed_1: C2Rust_Unnamed_3,
    pub c2rust_unnamed_2: C2Rust_Unnamed_2,
    pub c2rust_unnamed_3: C2Rust_Unnamed_1,
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
    pub image: C2Rust_Unnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_0 {
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
pub union C2Rust_Unnamed_1 {
    pub thrust_wait: BYTE,
    pub blast_offset: BYTE,
    pub next_turn: BYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_2 {
    pub turn_wait: BYTE,
    pub sys_loc: BYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_3 {
    pub mass_points: BYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_4 {
    pub crew_level: COUNT,
    pub hit_points: COUNT,
    pub facing: COUNT,
    pub cycle: COUNT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_5 {
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
    pub stowed_ship_q: QUEUE,
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_6(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_6 {
    pub const DOS_MENU_HILITE: Self = Self(0);
    pub const SPECIAL_BEVEL: Self = Self(1);
    pub const THIN_INNER_BEVEL: Self = Self(9);
    pub const THICK_INNER_BEVEL: Self = Self(17);
    pub const THIN_OUTER_BEVEL: Self = Self(25);
    pub const THICK_OUTER_BEVEL: Self = Self(33);
    pub const THICK_OUT_INV_BEVEL: Self = Self(41);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_7(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_7 {
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
#[no_mangle]
pub static mut status_y_offsets: [COORD; 2] = [0; 2];
#[no_mangle]
pub unsafe extern "C" fn InitStatusOffsets() {
    status_y_offsets[0usize] = ((CanvasHeight >> 1 as ::core::ffi::c_int)
        - (if if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            16 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        } << resolutionFactor)) as COORD;
    status_y_offsets[1usize] = 0 as COORD;
}
#[no_mangle]
pub unsafe extern "C" fn animIsInTransition(mut CSPtr: *mut CAPTAIN_STUFF) -> bool {
    return (*CSPtr).thrust_offset as ::core::ffi::c_int & 1 as ::core::ffi::c_int != 0
        || (*CSPtr).weapon_offset as ::core::ffi::c_int & 1 as ::core::ffi::c_int != 0
        || (*CSPtr).special_offset as ::core::ffi::c_int & 1 as ::core::ffi::c_int != 0
        || (*CSPtr).tl_offset as ::core::ffi::c_int & 1 as ::core::ffi::c_int != 0
        || (*CSPtr).tr_offset as ::core::ffi::c_int & 1 as ::core::ffi::c_int != 0;
}
unsafe extern "C" fn CalculateAnimOffsets(
    mut CSPtr: *mut CAPTAIN_STUFF,
    mut delta_status_flags: STATUS_FLAGS,
    mut cur_status_flags: STATUS_FLAGS,
) -> BYTE {
    let mut redraw_flag: BYTE = 0 as BYTE;
    if delta_status_flags as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int != 0
        || (*CSPtr).tl_offset as ::core::ffi::c_int & 1 as ::core::ffi::c_int != 0
    {
        if delta_status_flags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int == 0
        {
            (*CSPtr).tr_offset = 0 as BYTE;
            if cur_status_flags as ::core::ffi::c_int
                & (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int != 0
            {
                (*CSPtr).tl_offset = (*CSPtr).tl_offset.wrapping_add(1);
            } else {
                (*CSPtr).tl_offset = (*CSPtr).tl_offset.wrapping_sub(1);
            }
            redraw_flag = (redraw_flag as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as BYTE;
        } else if cur_status_flags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int != 0
        {
            (*CSPtr).tl_offset = 0 as BYTE;
            (*CSPtr).tr_offset = 1 as BYTE;
            redraw_flag = (redraw_flag as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as BYTE;
        } else {
            (*CSPtr).tr_offset = 0 as BYTE;
            (*CSPtr).tl_offset = 1 as BYTE;
            redraw_flag = (redraw_flag as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as BYTE;
        }
    } else if delta_status_flags as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int != 0
        || (*CSPtr).tr_offset as ::core::ffi::c_int & 1 as ::core::ffi::c_int != 0
    {
        (*CSPtr).tl_offset = 0 as BYTE;
        if cur_status_flags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int != 0
        {
            (*CSPtr).tr_offset = (*CSPtr).tr_offset.wrapping_add(1);
        } else {
            (*CSPtr).tr_offset = (*CSPtr).tr_offset.wrapping_sub(1);
        }
        redraw_flag = (redraw_flag as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as BYTE;
    }
    if delta_status_flags as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int != 0
        || (*CSPtr).thrust_offset as ::core::ffi::c_int & 1 as ::core::ffi::c_int != 0
    {
        if cur_status_flags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int != 0
        {
            (*CSPtr).thrust_offset = (*CSPtr).thrust_offset.wrapping_add(1);
        } else {
            (*CSPtr).thrust_offset = (*CSPtr).thrust_offset.wrapping_sub(1);
        }
        redraw_flag = (redraw_flag as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as BYTE;
    }
    if delta_status_flags as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int != 0
        || (*CSPtr).weapon_offset as ::core::ffi::c_int & 1 as ::core::ffi::c_int != 0
    {
        if cur_status_flags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int != 0
        {
            (*CSPtr).weapon_offset = (*CSPtr).weapon_offset.wrapping_add(1);
        } else {
            (*CSPtr).weapon_offset = (*CSPtr).weapon_offset.wrapping_sub(1);
        }
        redraw_flag = (redraw_flag as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as BYTE;
    }
    if delta_status_flags as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int != 0
        || (*CSPtr).special_offset as ::core::ffi::c_int & 1 as ::core::ffi::c_int != 0
    {
        if cur_status_flags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int != 0
        {
            (*CSPtr).special_offset = (*CSPtr).special_offset.wrapping_add(1);
        } else {
            (*CSPtr).special_offset = (*CSPtr).special_offset.wrapping_sub(1);
        }
        redraw_flag = (redraw_flag as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as BYTE;
    }
    return redraw_flag;
}
unsafe extern "C" fn DrawCaptainWindowAnimation(
    mut CSPtr: *mut CAPTAIN_STUFF,
    mut y: COORD,
    mut redraw_flag: BYTE,
) {
    let mut Stamp: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    Stamp.origin.x = ((((64 as ::core::ffi::c_int) << resolutionFactor)
        - ((55 as ::core::ffi::c_int) << resolutionFactor) >> resolutionFactor
        >> 1 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    Stamp.origin.y = (y as ::core::ffi::c_int
        + (((65 as ::core::ffi::c_int) << resolutionFactor)
            + (if isPC(optWhichMenu).0 != 0 {
                1 as ::core::ffi::c_int
            } else if if optWindowType == 0 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                1 as ::core::ffi::c_int
            } else {
                4 as ::core::ffi::c_int
            } << resolutionFactor))) as COORD;
    if redraw_flag as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int != 0
    {
        let mut i: BYTE = 0;
        Stamp.frame = SetRelFrameIndex((*CSPtr).turn, 1 as SIZE);
        uqm_lua_checked!(DrawStamp(&raw mut Stamp), ());
        Stamp.frame = IncFrameIndex(Stamp.frame);
        uqm_lua_checked!(DrawStamp(&raw mut Stamp), ());
        i = 0 as BYTE;
        while (i as ::core::ffi::c_int) < (*CSPtr).tl_offset as ::core::ffi::c_int {
            Stamp.frame = IncFrameIndex(Stamp.frame);
            uqm_lua_checked!(DrawStamp(&raw mut Stamp), ());
            i = i.wrapping_add(1);
        }
    } else if redraw_flag as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int != 0
    {
        let mut i_0: BYTE = 0;
        Stamp.frame = SetRelFrameIndex((*CSPtr).turn, 3 as SIZE);
        uqm_lua_checked!(DrawStamp(&raw mut Stamp), ());
        Stamp.frame = DecFrameIndex(Stamp.frame);
        uqm_lua_checked!(DrawStamp(&raw mut Stamp), ());
        i_0 = 0 as BYTE;
        while (i_0 as ::core::ffi::c_int) < (*CSPtr).tr_offset as ::core::ffi::c_int {
            Stamp.frame = DecFrameIndex(Stamp.frame);
            uqm_lua_checked!(DrawStamp(&raw mut Stamp), ());
            i_0 = i_0.wrapping_add(1);
        }
    }
    if redraw_flag as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int != 0
    {
        Stamp.frame = SetRelFrameIndex((*CSPtr).thrust, (*CSPtr).thrust_offset as SIZE);
        uqm_lua_checked!(DrawStamp(&raw mut Stamp), ());
    }
    if redraw_flag as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int != 0
    {
        Stamp.frame = SetRelFrameIndex((*CSPtr).weapon, (*CSPtr).weapon_offset as SIZE);
        uqm_lua_checked!(DrawStamp(&raw mut Stamp), ());
    }
    if redraw_flag as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int != 0
    {
        Stamp.frame = SetRelFrameIndex(
            (*CSPtr).special,
            (*CSPtr).special_offset as SIZE,
        );
        uqm_lua_checked!(DrawStamp(&raw mut Stamp), ());
    }
}
unsafe extern "C" fn DrawCaptainWindowFrame(mut fr: FRAME, mut y: COORD) {
    let mut Stamp: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    Stamp.origin.x = ((((64 as ::core::ffi::c_int) << resolutionFactor)
        - ((55 as ::core::ffi::c_int) << resolutionFactor) >> resolutionFactor
        >> 1 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    Stamp.origin.y = (y as ::core::ffi::c_int
        + (((65 as ::core::ffi::c_int) << resolutionFactor)
            + (if isPC(optWhichMenu).0 != 0 {
                1 as ::core::ffi::c_int
            } else if if optWindowType == 0 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                1 as ::core::ffi::c_int
            } else {
                4 as ::core::ffi::c_int
            } << resolutionFactor))) as COORD;
    Stamp.frame = fr;
    uqm_lua_checked!(DrawStamp(&raw mut Stamp), ());
}
#[no_mangle]
pub unsafe extern "C" fn DrawBattleCrewAmount(
    mut ShipInfoPtr: *mut SHIP_INFO,
    mut y_offs: COORD,
) {
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut t: TEXT = TEXT {
        baseline: POINT { x: 0, y: 0 },
        pStr: ::core::ptr::null::<UNICODE>(),
        align: TEXT_ALIGN::ALIGN_LEFT,
        CharCount: 0,
    };
    let mut buf: [UNICODE; 40] = [0; 40];
    t.baseline.x = (((8 as ::core::ffi::c_int) << resolutionFactor)
        + (if isPC(optWhichMenu).0 != 0 {
            4 as ::core::ffi::c_int
        } else {
            2 as ::core::ffi::c_int
        } << resolutionFactor)) as COORD;
    t.baseline.y = (((64 as ::core::ffi::c_int) << resolutionFactor)
        - (if if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            16 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        } << resolutionFactor) + y_offs as ::core::ffi::c_int) as COORD;
    t.align = TEXT_ALIGN(
        (if isPC(optWhichMenu).0 != 0 {
            TEXT_ALIGN::ALIGN_CENTER.0 as ::core::ffi::c_int
        } else {
            TEXT_ALIGN::ALIGN_LEFT.0 as ::core::ffi::c_int
        }) as ::core::ffi::c_uint,
    );
    t.pStr = &raw mut buf as *mut UNICODE;
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    r.corner.x = (t.baseline.x as ::core::ffi::c_int
        - (if isPC(optWhichMenu).0 != 0 {
            9 as ::core::ffi::c_int
        } else {
            2 as ::core::ffi::c_int
        } << resolutionFactor)) as COORD;
    r.corner.y = (t.baseline.y as ::core::ffi::c_int
        - ((5 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    r.extent.width = ((6 as ::core::ffi::c_int * 3 as ::core::ffi::c_int
        + 6 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    r.extent.height = ((5 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    sprintf(
        &raw mut buf as *mut ::core::ffi::c_char,
        b"%u\0".as_ptr() as *const ::core::ffi::c_char,
        (*ShipInfoPtr).crew_level as ::core::ffi::c_int,
    );
    SetContextFont(StarConFont);
    SetContextForeGroundColor(colorFromRgb15(0xa as BYTE, 0xa as BYTE, 0xa as BYTE));
    uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
    SetContextForeGroundColor(colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE));
    uqm_lua_checked!(font_DrawText(&raw mut t), ());
}
#[no_mangle]
pub unsafe extern "C" fn DrawCaptainsWindow(mut StarShipPtr: *mut STARSHIP) {
    let mut y: COORD = 0;
    let mut y_offs: COORD = 0;
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut s: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    let mut Frame: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
    let mut RDPtr: *mut RACE_DESC = ::core::ptr::null_mut::<RACE_DESC>();
    RDPtr = (*StarShipPtr).RaceDescPtr;
    Frame = (*RDPtr).ship_data.captain_control.background;
    if !Frame.is_null() {
        Frame = SetAbsFrameIndex(Frame, 0 as COUNT);
        (*RDPtr).ship_data.captain_control.background = Frame;
        Frame = SetRelFrameIndex(Frame, 1 as SIZE);
        (*RDPtr).ship_data.captain_control.turn = Frame;
        Frame = SetRelFrameIndex(Frame, 5 as SIZE);
        (*RDPtr).ship_data.captain_control.thrust = Frame;
        Frame = SetRelFrameIndex(Frame, 3 as SIZE);
        (*RDPtr).ship_data.captain_control.weapon = Frame;
        Frame = SetRelFrameIndex(Frame, 3 as SIZE);
        (*RDPtr).ship_data.captain_control.special = Frame;
    }
    BatchGraphics();
    ((*StarShipPtr).playerNr as ::core::ffi::c_int >= 0 as ::core::ffi::c_int
        || {
            __assert_fail(
                b"StarShipPtr->playerNr >= 0\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/uqm/status.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                352 as ::core::ffi::c_int,
                b"DrawCaptainsWindow\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    y_offs = status_y_offsets[(*StarShipPtr).playerNr as usize];
    r.corner.x = (((((64 as ::core::ffi::c_int) << resolutionFactor)
        - ((55 as ::core::ffi::c_int) << resolutionFactor) >> resolutionFactor
        >> 1 as ::core::ffi::c_int) << resolutionFactor)
        - ((2 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    r.corner.y = (y_offs as ::core::ffi::c_int
        + ((65 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    r.extent.width = (((64 as ::core::ffi::c_int) << resolutionFactor)
        - ((((64 as ::core::ffi::c_int) << resolutionFactor)
            - ((55 as ::core::ffi::c_int) << resolutionFactor) >> resolutionFactor
            >> 1 as ::core::ffi::c_int) << resolutionFactor)
        + if if resolutionFactor != 2 as ::core::ffi::c_uint {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } == 0
        {
            0 as ::core::ffi::c_int
        } else {
            4 as ::core::ffi::c_int
        }) as COORD;
    r.extent.height = ((CanvasHeight >> 1 as ::core::ffi::c_int)
        - (if if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            16 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        } << resolutionFactor)
        - (((65 as ::core::ffi::c_int) << resolutionFactor)
            + (if isPC(optWhichMenu).0 != 0 {
                1 as ::core::ffi::c_int
            } else if if optWindowType == 0 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                1 as ::core::ffi::c_int
            } else {
                4 as ::core::ffi::c_int
            } << resolutionFactor))
        + (if isPC(optWhichMenu).0 != 0 {
            1 as ::core::ffi::c_int
        } else if if optWindowType == 0 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            0 as ::core::ffi::c_int
        } else {
            2 as ::core::ffi::c_int
        } << resolutionFactor)) as COORD;
    SetContextForeGroundColor(colorFromRgb15(0xa as BYTE, 0xa as BYTE, 0xa as BYTE));
    uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
    SetContextForeGroundColor(colorFromRgb15(0x8 as BYTE, 0x8 as BYTE, 0x8 as BYTE));
    r.corner.x = ((1 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    r.corner.y = (y_offs as ::core::ffi::c_int
        + ((65 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    r.extent.width = ((1 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    r.extent.height = ((CanvasHeight >> 1 as ::core::ffi::c_int)
        - (if if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            16 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        } << resolutionFactor) - ((65 as ::core::ffi::c_int) << resolutionFactor)
        - ((2 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
    r.corner.x = 0 as COORD;
    r.extent.height = (r.extent.height as ::core::ffi::c_int
        + ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
    SetContextForeGroundColor(colorFromRgb15(0x10 as BYTE, 0x10 as BYTE, 0x10 as BYTE));
    r.corner.x = (((64 as ::core::ffi::c_int) << resolutionFactor)
        - ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    r.corner.y = (y_offs as ::core::ffi::c_int
        + ((65 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    r.extent.width = ((1 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    r.extent.height = ((CanvasHeight >> 1 as ::core::ffi::c_int)
        - (if if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            16 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        } << resolutionFactor) - ((65 as ::core::ffi::c_int) << resolutionFactor))
        as COORD;
    uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
    r.corner.x = (((64 as ::core::ffi::c_int) << resolutionFactor)
        - ((2 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
    r.corner.x = ((1 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    r.extent.width = (((64 as ::core::ffi::c_int) << resolutionFactor)
        - ((2 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    r.corner.y = (y_offs as ::core::ffi::c_int
        + ((CanvasHeight >> 1 as ::core::ffi::c_int)
            - (if if optWindowType == 1 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                16 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } << resolutionFactor) - ((2 as ::core::ffi::c_int) << resolutionFactor)))
        as COORD;
    r.extent.height = ((1 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
    r.corner.x = 0 as COORD;
    r.extent.width = (r.extent.width as ::core::ffi::c_int
        + ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    r.corner.y = (r.corner.y as ::core::ffi::c_int
        + ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
    y = (y_offs as ::core::ffi::c_int
        + (((65 as ::core::ffi::c_int) << resolutionFactor)
            + (if isPC(optWhichMenu).0 != 0 {
                1 as ::core::ffi::c_int
            } else if if optWindowType == 0 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                1 as ::core::ffi::c_int
            } else {
                4 as ::core::ffi::c_int
            } << resolutionFactor))) as COORD;
    if if resolutionFactor != 2 as ::core::ffi::c_uint {
        BOOLEAN::FALSE.0 as ::core::ffi::c_int
    } else {
        BOOLEAN::TRUE.0 as ::core::ffi::c_int
    } == 0
    {
        SetContextForeGroundColor(colorFromRgb15(0x8 as BYTE, 0x8 as BYTE, 0x8 as BYTE));
        r.corner.x = (((55 as ::core::ffi::c_int) << resolutionFactor)
            + ((((64 as ::core::ffi::c_int) << resolutionFactor)
                - ((55 as ::core::ffi::c_int) << resolutionFactor) >> resolutionFactor
                >> 1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        r.corner.y = y;
        r.extent.width = ((1 as ::core::ffi::c_int) << resolutionFactor) as COORD;
        r.extent.height = ((30 as ::core::ffi::c_int) << resolutionFactor) as COORD;
        uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
        r.corner.x = (((((64 as ::core::ffi::c_int) << resolutionFactor)
            - ((55 as ::core::ffi::c_int) << resolutionFactor) >> resolutionFactor
            >> 1 as ::core::ffi::c_int) << resolutionFactor)
            - ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        r.corner.y = (r.corner.y as ::core::ffi::c_int
            + ((30 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        r.extent.width = (((55 as ::core::ffi::c_int) << resolutionFactor)
            + ((2 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        r.extent.height = ((1 as ::core::ffi::c_int) << resolutionFactor) as COORD;
        uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
        SetContextForeGroundColor(
            colorFromRgb15(0x10 as BYTE, 0x10 as BYTE, 0x10 as BYTE),
        );
        r.corner.x = (((((64 as ::core::ffi::c_int) << resolutionFactor)
            - ((55 as ::core::ffi::c_int) << resolutionFactor) >> resolutionFactor
            >> 1 as ::core::ffi::c_int) << resolutionFactor)
            - ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        r.extent.width = (((55 as ::core::ffi::c_int) << resolutionFactor)
            + ((2 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        r.corner.y = (y as ::core::ffi::c_int
            - ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        r.extent.height = ((1 as ::core::ffi::c_int) << resolutionFactor) as COORD;
        uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
        r.corner.x = (((((64 as ::core::ffi::c_int) << resolutionFactor)
            - ((55 as ::core::ffi::c_int) << resolutionFactor) >> resolutionFactor
            >> 1 as ::core::ffi::c_int) << resolutionFactor)
            - ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        r.extent.width = ((1 as ::core::ffi::c_int) << resolutionFactor) as COORD;
        r.corner.y = y;
        r.extent.height = ((30 as ::core::ffi::c_int) << resolutionFactor) as COORD;
        uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
    } else {
        r.corner.x = (((((64 as ::core::ffi::c_int) << resolutionFactor)
            - ((55 as ::core::ffi::c_int) << resolutionFactor) >> resolutionFactor
            >> 1 as ::core::ffi::c_int) << resolutionFactor) - 4 as ::core::ffi::c_int)
            as COORD;
        r.corner.y = (y as ::core::ffi::c_int - 4 as ::core::ffi::c_int) as COORD;
        r.extent.width = (((55 as ::core::ffi::c_int) << resolutionFactor)
            + 8 as ::core::ffi::c_int) as COORD;
        r.extent.height = (((30 as ::core::ffi::c_int) << resolutionFactor)
            + 8 as ::core::ffi::c_int) as COORD;
        uqm_lua_checked!(DrawRenderedBox(
            &raw mut r,
            BOOLEAN::TRUE,
            colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE),
            C2Rust_Unnamed_6::THIN_INNER_BEVEL.0 as ::core::ffi::c_int,
            BOOLEAN::FALSE.0 as ::core::ffi::c_int,
        ), ());
    }
    s.frame = (*RDPtr).ship_data.captain_control.background;
    s.origin.x = ((((64 as ::core::ffi::c_int) << resolutionFactor)
        - ((55 as ::core::ffi::c_int) << resolutionFactor) >> resolutionFactor
        >> 1 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    s.origin.y = y;
    uqm_lua_checked!(DrawStamp(&raw mut s), ());
    if (*StarShipPtr).captains_name_index as ::core::ffi::c_int
        == 0 as ::core::ffi::c_int
        && (*StarShipPtr).playerNr as ::core::ffi::c_int == 0 as ::core::ffi::c_int
    {
        let mut t: TEXT = TEXT {
            baseline: POINT { x: 0, y: 0 },
            pStr: ::core::ptr::null::<UNICODE>(),
            align: TEXT_ALIGN::ALIGN_LEFT,
            CharCount: 0,
        };
        t.baseline.x = ((64 as ::core::ffi::c_int) << resolutionFactor
            >> 1 as ::core::ffi::c_int) as COORD;
        t.baseline.y = (y as ::core::ffi::c_int
            + ((6 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        t.align = TEXT_ALIGN::ALIGN_CENTER;
        t.pStr = &raw mut GlobData.SIS_state.CommanderName as *mut UNICODE;
        t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
        SetContextForeGroundColor(colorFromRgb15(0 as BYTE, 0x14 as BYTE, 0 as BYTE));
        SetContextFont(if isPC(optWhichMenu).0 != 0 { TinyFont } else { TinyFontBold });
        uqm_lua_checked!(font_DrawText(&raw mut t), ());
    }
    if (*RDPtr).ship_info.max_crew as ::core::ffi::c_int > 42 as ::core::ffi::c_int
        || (*RDPtr).ship_info.ship_flags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int != 0
    {
        y_offs = (y_offs as ::core::ffi::c_int
            + (if if optWindowType == 1 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                16 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } << resolutionFactor)) as COORD;
        uqm_lua_checked!(DrawBattleCrewAmount(&raw mut (*RDPtr).ship_info, y_offs), ());
    }
    UnbatchGraphics();
}
#[no_mangle]
pub unsafe extern "C" fn DeltaEnergy(
    mut ElementPtr: *mut ELEMENT,
    mut energy_delta: SIZE,
) -> BOOLEAN {
    let mut retval: BOOLEAN = BOOLEAN::FALSE;
    let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
    let mut ShipInfoPtr: *mut SHIP_INFO = ::core::ptr::null_mut::<SHIP_INFO>();
    retval = BOOLEAN::TRUE;
    if antiCheat(
            ElementPtr,
            BOOLEAN::FALSE,
            C2Rust_Unnamed::OPTVAL_INF_ENERGY.0 as ::core::ffi::c_int,
        )
        .0 != 0
        || antiCheat(
                ElementPtr,
                BOOLEAN::FALSE,
                C2Rust_Unnamed::OPTVAL_FULL_GOD.0 as ::core::ffi::c_int,
            )
            .0 != 0
    {
        return retval;
    }
    StarShipPtr = (*ElementPtr).pParent as *mut STARSHIP;
    ShipInfoPtr = &raw mut (*(*StarShipPtr).RaceDescPtr).ship_info;
    if energy_delta as ::core::ffi::c_int >= 0 as ::core::ffi::c_int {
        if ((*ShipInfoPtr).energy_level as ::core::ffi::c_int
            + energy_delta as BYTE as ::core::ffi::c_int) as BYTE as ::core::ffi::c_int
            > (*ShipInfoPtr).max_energy as ::core::ffi::c_int
        {
            energy_delta = ((*ShipInfoPtr).max_energy as ::core::ffi::c_int
                - (*ShipInfoPtr).energy_level as ::core::ffi::c_int) as SIZE;
        }
    } else if -(energy_delta as ::core::ffi::c_int) as BYTE as ::core::ffi::c_int
        > (*ShipInfoPtr).energy_level as ::core::ffi::c_int
    {
        retval = BOOLEAN::FALSE;
    }
    if retval.0 == 0 {
        (*StarShipPtr).cur_status_flags = ((*StarShipPtr).cur_status_flags
            as ::core::ffi::c_int | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int)
            as STATUS_FLAGS;
    } else {
        (*StarShipPtr).cur_status_flags = ((*StarShipPtr).cur_status_flags
            as ::core::ffi::c_int
            & !((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int)) as STATUS_FLAGS;
        (*StarShipPtr).energy_counter = (*(*StarShipPtr).RaceDescPtr)
            .characteristics
            .energy_wait;
        uqm_lua_checked!(DeltaStatistics(
            ShipInfoPtr,
            status_y_offsets[(*StarShipPtr).playerNr as usize],
            0 as SIZE,
            energy_delta,
        ), BOOLEAN(0));
    }
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn DeltaCrew(
    mut ElementPtr: *mut ELEMENT,
    mut crew_delta: SIZE,
) -> BOOLEAN {
    let mut retval: BOOLEAN = BOOLEAN::FALSE;
    let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
    let mut ShipInfoPtr: *mut SHIP_INFO = ::core::ptr::null_mut::<SHIP_INFO>();
    if GlobData.Game_state.CurrentActivity as BYTE as ::core::ffi::c_int
        == C2Rust_Unnamed_7::IN_LAST_BATTLE.0 as ::core::ffi::c_int
        && (*ElementPtr).playerNr as ::core::ffi::c_int == 1 as ::core::ffi::c_int
    {
        return BOOLEAN::TRUE;
    }
    retval = BOOLEAN::TRUE;
    StarShipPtr = (*ElementPtr).pParent as *mut STARSHIP;
    ShipInfoPtr = &raw mut (*(*StarShipPtr).RaceDescPtr).ship_info;
    if crew_delta as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
        (*ElementPtr).c2rust_unnamed_0.crew_level = ((*ElementPtr)
            .c2rust_unnamed_0
            .crew_level as ::core::ffi::c_int + crew_delta as ::core::ffi::c_int)
            as COUNT;
        if (*ElementPtr).c2rust_unnamed_0.crew_level as ::core::ffi::c_int
            > (*ShipInfoPtr).max_crew as ::core::ffi::c_int
        {
            crew_delta = ((*ShipInfoPtr).max_crew as ::core::ffi::c_int
                - (*ShipInfoPtr).crew_level as ::core::ffi::c_int) as SIZE;
            (*ElementPtr).c2rust_unnamed_0.crew_level = (*ShipInfoPtr).max_crew;
        }
    } else if (crew_delta as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
        if (*ElementPtr).c2rust_unnamed_0.crew_level as ::core::ffi::c_int
            > -(crew_delta as ::core::ffi::c_int) as COUNT as ::core::ffi::c_int
        {
            (*ElementPtr).c2rust_unnamed_0.crew_level = ((*ElementPtr)
                .c2rust_unnamed_0
                .crew_level as ::core::ffi::c_int + crew_delta as ::core::ffi::c_int)
                as COUNT;
        } else {
            crew_delta = -((*ElementPtr).c2rust_unnamed_0.crew_level as SIZE
                as ::core::ffi::c_int) as SIZE;
            (*ElementPtr).c2rust_unnamed_0.crew_level = 0 as COUNT;
            retval = BOOLEAN::FALSE;
        }
    }
    uqm_lua_checked!(DeltaStatistics(
        ShipInfoPtr,
        status_y_offsets[(*StarShipPtr).playerNr as usize],
        crew_delta,
        0 as SIZE,
    ), BOOLEAN(0));
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn PreProcessStatus(mut ShipPtr: *mut ELEMENT) {
    let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
    StarShipPtr = (*ShipPtr).pParent as *mut STARSHIP;
    if (*StarShipPtr).captains_name_index as ::core::ffi::c_int != 0
        || (*StarShipPtr).playerNr as ::core::ffi::c_int == 0 as ::core::ffi::c_int
    {
        let mut old_status_flags: STATUS_FLAGS = 0;
        let mut cur_status_flags: STATUS_FLAGS = 0;
        let mut CSPtr: *mut CAPTAIN_STUFF = ::core::ptr::null_mut::<CAPTAIN_STUFF>();
        let mut redraw_flags: BYTE = 0 as BYTE;
        cur_status_flags = (*StarShipPtr).cur_status_flags;
        old_status_flags = (*StarShipPtr).old_status_flags;
        old_status_flags = (old_status_flags as ::core::ffi::c_int
            ^ cur_status_flags as ::core::ffi::c_int) as STATUS_FLAGS;
        CSPtr = &raw mut (*(*StarShipPtr).RaceDescPtr).ship_data.captain_control;
        old_status_flags = (old_status_flags as ::core::ffi::c_int
            & ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)) as STATUS_FLAGS;
        if old_status_flags as ::core::ffi::c_int != 0
            || animIsInTransition(CSPtr) as ::core::ffi::c_int != 0
        {
            ((*StarShipPtr).playerNr as ::core::ffi::c_int >= 0 as ::core::ffi::c_int
                || {
                    __assert_fail(
                        b"StarShipPtr->playerNr >= 0\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"src/uqm/status.c\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        577 as ::core::ffi::c_int,
                        b"PreProcessStatus\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    0 as ::core::ffi::c_int != 0
                }) as ::core::ffi::c_int;
            redraw_flags = CalculateAnimOffsets(
                CSPtr,
                old_status_flags,
                cur_status_flags,
            );
            if redraw_flags != 0 {
                uqm_lua_checked!(DrawCaptainWindowAnimation(
                    CSPtr,
                    status_y_offsets[(*StarShipPtr).playerNr as usize],
                    redraw_flags,
                ), ());
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn PostProcessStatus(mut ShipPtr: *mut ELEMENT) {
    let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
    StarShipPtr = (*ShipPtr).pParent as *mut STARSHIP;
    if (*StarShipPtr).captains_name_index as ::core::ffi::c_int != 0
        || (*StarShipPtr).playerNr as ::core::ffi::c_int == 0 as ::core::ffi::c_int
    {
        let mut y: COORD = 0;
        let mut cur_status_flags: STATUS_FLAGS = 0;
        let mut old_status_flags: STATUS_FLAGS = 0;
        cur_status_flags = (*StarShipPtr).cur_status_flags;
        ((*StarShipPtr).playerNr as ::core::ffi::c_int >= 0 as ::core::ffi::c_int
            || {
                __assert_fail(
                    b"StarShipPtr->playerNr >= 0\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"src/uqm/status.c\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    601 as ::core::ffi::c_int,
                    b"PostProcessStatus\0".as_ptr() as *const ::core::ffi::c_char,
                );
                0 as ::core::ffi::c_int != 0
            }) as ::core::ffi::c_int;
        y = status_y_offsets[(*StarShipPtr).playerNr as usize];
        if (*ShipPtr).c2rust_unnamed_0.crew_level as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
            (*StarShipPtr).cur_status_flags = ((*StarShipPtr).cur_status_flags
                as ::core::ffi::c_int
                & !((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int))
                as STATUS_FLAGS;
            if (*(*StarShipPtr).RaceDescPtr).ship_info.crew_level as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
                && (*ShipPtr).state_flags as ::core::ffi::c_int
                    & (1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int != 0
            {
                let mut i: BYTE = 0;
                let mut j: BYTE = 0;
                let mut c: Color = Color { r: 0, g: 0, b: 0, a: 0 };
                let mut r: RECT = RECT {
                    corner: POINT { x: 0, y: 0 },
                    extent: EXTENT { width: 0, height: 0 },
                };
                i = ((12 as ::core::ffi::c_int * 3 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as BYTE as ::core::ffi::c_int
                    - (*ShipPtr).c2rust_unnamed.life_span as ::core::ffi::c_int) as BYTE;
                if cur_status_flags as ::core::ffi::c_int
                    & (1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int != 0
                    && i as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                {
                    let mut CSPtr: *mut CAPTAIN_STUFF = ::core::ptr::null_mut::<
                        CAPTAIN_STUFF,
                    >();
                    CSPtr = &raw mut (*(*StarShipPtr).RaceDescPtr)
                        .ship_data
                        .captain_control;
                    cur_status_flags = (cur_status_flags as ::core::ffi::c_int
                        & !((1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int))
                        as STATUS_FLAGS;
                    uqm_lua_checked!(DrawCaptainWindowFrame(DecFrameIndex((*CSPtr).background), y), ());
                } else {
                    if i as ::core::ffi::c_int <= 4 as ::core::ffi::c_int {
                        static mut flash_tab0: [Color; 5] = [
                            Color {
                                r: ((0xf as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                    | 0xf as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
                                    as BYTE,
                                g: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                    | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
                                    as BYTE,
                                b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                    | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
                                    as BYTE,
                                a: 0xff as BYTE,
                            },
                            Color {
                                r: ((0x1f as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                    | 0x1f as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
                                    as BYTE,
                                g: ((0x19 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                    | 0x19 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
                                    as BYTE,
                                b: ((0x19 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                    | 0x19 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
                                    as BYTE,
                                a: 0xff as BYTE,
                            },
                            Color {
                                r: ((0x1f as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                    | 0x1f as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
                                    as BYTE,
                                g: ((0x11 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                    | 0x11 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
                                    as BYTE,
                                b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                    | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
                                    as BYTE,
                                a: 0xff as BYTE,
                            },
                            Color {
                                r: ((0x1f as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                    | 0x1f as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
                                    as BYTE,
                                g: ((0x1c as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                    | 0x1c as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
                                    as BYTE,
                                b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                    | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
                                    as BYTE,
                                a: 0xff as BYTE,
                            },
                            Color {
                                r: ((0x1f as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                    | 0x1f as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
                                    as BYTE,
                                g: ((0x1f as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                    | 0x1f as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
                                    as BYTE,
                                b: ((0x1f as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                    | 0x1f as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
                                    as BYTE,
                                a: 0xff as BYTE,
                            },
                        ];
                        c = flash_tab0[i as usize];
                        r.corner.x = ((((64 as ::core::ffi::c_int) << resolutionFactor)
                            - ((55 as ::core::ffi::c_int) << resolutionFactor)
                            >> resolutionFactor >> 1 as ::core::ffi::c_int)
                            << resolutionFactor) as COORD;
                        r.corner.y = (y as ::core::ffi::c_int
                            + (((65 as ::core::ffi::c_int) << resolutionFactor)
                                + (if isPC(optWhichMenu).0 != 0 {
                                    1 as ::core::ffi::c_int
                                } else if if optWindowType == 0 as DWORD {
                                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                                } else {
                                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                                } != 0
                                {
                                    1 as ::core::ffi::c_int
                                } else {
                                    4 as ::core::ffi::c_int
                                } << resolutionFactor))) as COORD;
                        r.extent.width = ((55 as ::core::ffi::c_int) << resolutionFactor)
                            as COORD;
                        r.extent.height = ((30 as ::core::ffi::c_int)
                            << resolutionFactor) as COORD;
                    } else {
                        SetContextForeGroundColor(
                            colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE),
                        );
                        i = (i as ::core::ffi::c_int - 5 as ::core::ffi::c_int) as BYTE;
                        if i as ::core::ffi::c_int <= 14 as ::core::ffi::c_int {
                            static mut flash_tab1: [Color; 15] = [
                                Color {
                                    r: ((0x1e as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                        | 0x1e as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
                                        as BYTE,
                                    g: ((0x1f as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                        | 0x1f as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
                                        as BYTE,
                                    b: ((0x12 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                        | 0x12 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
                                        as BYTE,
                                    a: 0xff as BYTE,
                                },
                                Color {
                                    r: ((0x1f as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                        | 0x1f as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
                                        as BYTE,
                                    g: ((0x1f as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                        | 0x1f as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
                                        as BYTE,
                                    b: ((0xa as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                        | 0xa as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
                                        as BYTE,
                                    a: 0xff as BYTE,
                                },
                                Color {
                                    r: ((0x1f as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                        | 0x1f as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
                                        as BYTE,
                                    g: ((0x1f as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                        | 0x1f as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
                                        as BYTE,
                                    b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                        | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
                                        as BYTE,
                                    a: 0xff as BYTE,
                                },
                                Color {
                                    r: ((0x1f as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                        | 0x1f as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
                                        as BYTE,
                                    g: ((0x1c as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                        | 0x1c as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
                                        as BYTE,
                                    b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                        | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
                                        as BYTE,
                                    a: 0xff as BYTE,
                                },
                                Color {
                                    r: ((0x1f as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                        | 0x1f as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
                                        as BYTE,
                                    g: ((0x18 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                        | 0x18 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
                                        as BYTE,
                                    b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                        | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
                                        as BYTE,
                                    a: 0xff as BYTE,
                                },
                                Color {
                                    r: ((0x1f as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                        | 0x1f as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
                                        as BYTE,
                                    g: ((0x15 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                        | 0x15 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
                                        as BYTE,
                                    b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                        | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
                                        as BYTE,
                                    a: 0xff as BYTE,
                                },
                                Color {
                                    r: ((0x1f as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                        | 0x1f as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
                                        as BYTE,
                                    g: ((0x11 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                        | 0x11 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
                                        as BYTE,
                                    b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                        | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
                                        as BYTE,
                                    a: 0xff as BYTE,
                                },
                                Color {
                                    r: ((0x1f as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                        | 0x1f as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
                                        as BYTE,
                                    g: ((0xe as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                        | 0xe as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
                                        as BYTE,
                                    b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                        | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
                                        as BYTE,
                                    a: 0xff as BYTE,
                                },
                                Color {
                                    r: ((0x1f as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                        | 0x1f as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
                                        as BYTE,
                                    g: ((0xa as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                        | 0xa as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
                                        as BYTE,
                                    b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                        | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
                                        as BYTE,
                                    a: 0xff as BYTE,
                                },
                                Color {
                                    r: ((0x1f as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                        | 0x1f as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
                                        as BYTE,
                                    g: ((0x7 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                        | 0x7 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
                                        as BYTE,
                                    b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                        | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
                                        as BYTE,
                                    a: 0xff as BYTE,
                                },
                                Color {
                                    r: ((0x1f as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                        | 0x1f as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
                                        as BYTE,
                                    g: ((0x3 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                        | 0x3 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
                                        as BYTE,
                                    b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                        | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
                                        as BYTE,
                                    a: 0xff as BYTE,
                                },
                                Color {
                                    r: ((0x1b as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                        | 0x1b as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
                                        as BYTE,
                                    g: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                        | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
                                        as BYTE,
                                    b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                        | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
                                        as BYTE,
                                    a: 0xff as BYTE,
                                },
                                Color {
                                    r: ((0x17 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                        | 0x17 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
                                        as BYTE,
                                    g: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                        | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
                                        as BYTE,
                                    b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                        | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
                                        as BYTE,
                                    a: 0xff as BYTE,
                                },
                                Color {
                                    r: ((0x13 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                        | 0x13 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
                                        as BYTE,
                                    g: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                        | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
                                        as BYTE,
                                    b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                        | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
                                        as BYTE,
                                    a: 0xff as BYTE,
                                },
                                Color {
                                    r: ((0xf as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                        | 0xf as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
                                        as BYTE,
                                    g: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                        | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
                                        as BYTE,
                                    b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                        | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
                                        as BYTE,
                                    a: 0xff as BYTE,
                                },
                            ];
                            c = flash_tab1[i as usize];
                            r.corner.x = (((((64 as ::core::ffi::c_int)
                                << resolutionFactor)
                                - ((55 as ::core::ffi::c_int) << resolutionFactor)
                                >> resolutionFactor >> 1 as ::core::ffi::c_int)
                                << resolutionFactor)
                                + ((i as ::core::ffi::c_int) << resolutionFactor)) as COORD;
                            r.corner.y = (y as ::core::ffi::c_int
                                + (((65 as ::core::ffi::c_int) << resolutionFactor)
                                    + (if isPC(optWhichMenu).0 != 0 {
                                        1 as ::core::ffi::c_int
                                    } else if if optWindowType == 0 as DWORD {
                                        BOOLEAN::TRUE.0 as ::core::ffi::c_int
                                    } else {
                                        BOOLEAN::FALSE.0 as ::core::ffi::c_int
                                    } != 0
                                    {
                                        1 as ::core::ffi::c_int
                                    } else {
                                        4 as ::core::ffi::c_int
                                    } << resolutionFactor))
                                + ((i as ::core::ffi::c_int) << resolutionFactor)) as COORD;
                            r.extent.width = (((55 as ::core::ffi::c_int)
                                << resolutionFactor)
                                - (((i as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
                                    << resolutionFactor)) as COORD;
                            r.extent.height = (((30 as ::core::ffi::c_int)
                                << resolutionFactor)
                                - (((i as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
                                    << resolutionFactor)) as COORD;
                            if r.extent.height as ::core::ffi::c_int
                                == (2 as ::core::ffi::c_int) << resolutionFactor
                            {
                                r.extent.height = (r.extent.height as ::core::ffi::c_int
                                    + ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
                            }
                            j = 0 as BYTE;
                            while (j as ::core::ffi::c_int)
                                < (1 as ::core::ffi::c_int) << resolutionFactor
                            {
                                uqm_lua_checked!(DrawRectangle(&raw mut r, BOOLEAN::FALSE), ());
                                r.corner.x += 1;
                                r.corner.y += 1;
                                r.extent.width = (r.extent.width as ::core::ffi::c_int
                                    - 2 as ::core::ffi::c_int) as COORD;
                                r.extent.height = (r.extent.height as ::core::ffi::c_int
                                    - 2 as ::core::ffi::c_int) as COORD;
                                j = j.wrapping_add(1);
                            }
                        } else {
                            i = (i as ::core::ffi::c_int - 15 as ::core::ffi::c_int)
                                as BYTE;
                            if i as ::core::ffi::c_int <= 4 as ::core::ffi::c_int {
                                r.corner.y = (y as ::core::ffi::c_int
                                    + (((65 as ::core::ffi::c_int) << resolutionFactor)
                                        + (if isPC(optWhichMenu).0 != 0 {
                                            1 as ::core::ffi::c_int
                                        } else if if optWindowType == 0 as DWORD {
                                            BOOLEAN::TRUE.0 as ::core::ffi::c_int
                                        } else {
                                            BOOLEAN::FALSE.0 as ::core::ffi::c_int
                                        } != 0
                                        {
                                            1 as ::core::ffi::c_int
                                        } else {
                                            4 as ::core::ffi::c_int
                                        } << resolutionFactor)
                                        + ((15 as ::core::ffi::c_int) << resolutionFactor)))
                                    as COORD;
                                r.extent.width = ((i as ::core::ffi::c_int
                                    + 1 as ::core::ffi::c_int) << resolutionFactor) as COORD;
                                r.extent.height = ((1 as ::core::ffi::c_int)
                                    << resolutionFactor) as COORD;
                                match i as ::core::ffi::c_int {
                                    0 => {
                                        r.corner.x = (((((64 as ::core::ffi::c_int)
                                            << resolutionFactor)
                                            - ((55 as ::core::ffi::c_int) << resolutionFactor)
                                            >> resolutionFactor >> 1 as ::core::ffi::c_int)
                                            << resolutionFactor)
                                            + ((15 as ::core::ffi::c_int) << resolutionFactor))
                                            as COORD;
                                        i = (((55 as ::core::ffi::c_int) << resolutionFactor)
                                            - (((15 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                                << 1 as ::core::ffi::c_int) << resolutionFactor)) as BYTE;
                                        c = colorFromRgb15(0x13 as BYTE, 0 as BYTE, 0 as BYTE);
                                    }
                                    1 => {
                                        r.corner.x = (((((64 as ::core::ffi::c_int)
                                            << resolutionFactor)
                                            - ((55 as ::core::ffi::c_int) << resolutionFactor)
                                            >> resolutionFactor >> 1 as ::core::ffi::c_int)
                                            << resolutionFactor)
                                            + ((16 as ::core::ffi::c_int) << resolutionFactor))
                                            as COORD;
                                        i = (((55 as ::core::ffi::c_int) << resolutionFactor)
                                            - (((17 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                                << 1 as ::core::ffi::c_int) << resolutionFactor)) as BYTE;
                                        c = colorFromRgb15(0x7 as BYTE, 0 as BYTE, 0 as BYTE);
                                    }
                                    2 => {
                                        r.corner.x = (((((64 as ::core::ffi::c_int)
                                            << resolutionFactor)
                                            - ((55 as ::core::ffi::c_int) << resolutionFactor)
                                            >> resolutionFactor >> 1 as ::core::ffi::c_int)
                                            << resolutionFactor)
                                            + ((18 as ::core::ffi::c_int) << resolutionFactor))
                                            as COORD;
                                        i = (((55 as ::core::ffi::c_int) << resolutionFactor)
                                            - (((20 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                                << 1 as ::core::ffi::c_int) << resolutionFactor)) as BYTE;
                                        c = colorFromRgb15(0x1b as BYTE, 0 as BYTE, 0 as BYTE);
                                    }
                                    3 => {
                                        r.corner.x = (((((64 as ::core::ffi::c_int)
                                            << resolutionFactor)
                                            - ((55 as ::core::ffi::c_int) << resolutionFactor)
                                            >> resolutionFactor >> 1 as ::core::ffi::c_int)
                                            << resolutionFactor)
                                            + ((21 as ::core::ffi::c_int) << resolutionFactor))
                                            as COORD;
                                        i = (((55 as ::core::ffi::c_int) << resolutionFactor)
                                            - (((24 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                                << 1 as ::core::ffi::c_int) << resolutionFactor)) as BYTE;
                                        c = colorFromRgb15(0x1f as BYTE, 0 as BYTE, 0 as BYTE);
                                    }
                                    4 => {
                                        r.corner.x = (((((64 as ::core::ffi::c_int)
                                            << resolutionFactor)
                                            - ((55 as ::core::ffi::c_int) << resolutionFactor)
                                            >> resolutionFactor >> 1 as ::core::ffi::c_int)
                                            << resolutionFactor)
                                            + ((25 as ::core::ffi::c_int) << resolutionFactor))
                                            as COORD;
                                        i = ((1 as ::core::ffi::c_int) << resolutionFactor) as BYTE;
                                        r.extent.width = ((2 as ::core::ffi::c_int)
                                            << resolutionFactor) as COORD;
                                        c = colorFromRgb15(0x1f as BYTE, 0x50 as BYTE, 0x5 as BYTE);
                                    }
                                    _ => {
                                        c = colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE);
                                    }
                                }
                                uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
                                r.corner.x = (r.corner.x as ::core::ffi::c_int
                                    + (i as ::core::ffi::c_int
                                        + r.extent.width as ::core::ffi::c_int)) as COORD;
                                uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
                                r.corner.x = (r.corner.x as ::core::ffi::c_int
                                    - i as ::core::ffi::c_int) as COORD;
                                r.extent.width = i as COORD;
                            } else {
                                i = (i as ::core::ffi::c_int - 5 as ::core::ffi::c_int)
                                    as BYTE;
                                if i as ::core::ffi::c_int > 2 as ::core::ffi::c_int {
                                    c = colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE);
                                } else {
                                    static mut flash_tab2: [Color; 3] = [
                                        Color {
                                            r: ((0x17 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                                | 0x17 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
                                                as BYTE,
                                            g: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
                                                as BYTE,
                                            b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
                                                as BYTE,
                                            a: 0xff as BYTE,
                                        },
                                        Color {
                                            r: ((0xf as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                                | 0xf as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
                                                as BYTE,
                                            g: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
                                                as BYTE,
                                            b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
                                                as BYTE,
                                            a: 0xff as BYTE,
                                        },
                                        Color {
                                            r: ((0xb as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                                | 0xb as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
                                                as BYTE,
                                            g: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
                                                as BYTE,
                                            b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
                                                as BYTE,
                                            a: 0xff as BYTE,
                                        },
                                    ];
                                    c = flash_tab2[i as usize];
                                }
                                r.corner.x = (((((64 as ::core::ffi::c_int)
                                    << resolutionFactor)
                                    - ((55 as ::core::ffi::c_int) << resolutionFactor)
                                    >> resolutionFactor >> 1 as ::core::ffi::c_int)
                                    << resolutionFactor)
                                    + ((55 as ::core::ffi::c_int) << resolutionFactor
                                        >> 1 as ::core::ffi::c_int)
                                    - if if resolutionFactor != 2 as ::core::ffi::c_uint {
                                        BOOLEAN::FALSE.0 as ::core::ffi::c_int
                                    } else {
                                        BOOLEAN::TRUE.0 as ::core::ffi::c_int
                                    } == 0
                                    {
                                        0 as ::core::ffi::c_int
                                    } else {
                                        2 as ::core::ffi::c_int
                                    }) as COORD;
                                r.corner.y = (y as ::core::ffi::c_int
                                    + (((65 as ::core::ffi::c_int) << resolutionFactor)
                                        + (if isPC(optWhichMenu).0 != 0 {
                                            1 as ::core::ffi::c_int
                                        } else if if optWindowType == 0 as DWORD {
                                            BOOLEAN::TRUE.0 as ::core::ffi::c_int
                                        } else {
                                            BOOLEAN::FALSE.0 as ::core::ffi::c_int
                                        } != 0
                                        {
                                            1 as ::core::ffi::c_int
                                        } else {
                                            4 as ::core::ffi::c_int
                                        } << resolutionFactor))
                                    + (((30 as ::core::ffi::c_int) << resolutionFactor)
                                        + ((1 as ::core::ffi::c_int) << resolutionFactor)
                                        >> 1 as ::core::ffi::c_int)
                                    - if if resolutionFactor != 2 as ::core::ffi::c_uint {
                                        BOOLEAN::FALSE.0 as ::core::ffi::c_int
                                    } else {
                                        BOOLEAN::TRUE.0 as ::core::ffi::c_int
                                    } == 0
                                    {
                                        0 as ::core::ffi::c_int
                                    } else {
                                        2 as ::core::ffi::c_int
                                    }) as COORD;
                                r.extent.width = ((1 as ::core::ffi::c_int)
                                    << resolutionFactor) as COORD;
                                r.extent.height = ((1 as ::core::ffi::c_int)
                                    << resolutionFactor) as COORD;
                            }
                        }
                    }
                    SetContextForeGroundColor(c);
                    uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
                }
            }
        }
        old_status_flags = (*StarShipPtr).old_status_flags;
        old_status_flags = ((old_status_flags as ::core::ffi::c_int
            ^ cur_status_flags as ::core::ffi::c_int)
            & ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int)) as STATUS_FLAGS;
        if old_status_flags != 0 {
            if old_status_flags as ::core::ffi::c_int
                & (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int != 0
            {
                if cur_status_flags as ::core::ffi::c_int
                    & (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int == 0
                {
                    uqm_lua_checked!(DrawCrewFuelString(y, 1 as SIZE), ());
                } else {
                    uqm_lua_checked!(DrawCrewFuelString(y, -1 as SIZE), ());
                }
            }
        }
        (*StarShipPtr).old_status_flags = cur_status_flags;
    }
}
#[inline]
unsafe extern "C" fn antiCheat(
    mut ElementPtr: *mut ELEMENT,
    mut SwapBool: BOOLEAN,
    mut GodModeType: ::core::ffi::c_int,
) -> BOOLEAN {
    return BOOLEAN(
        (!(*(&raw mut PlayerControl as *mut BYTE).offset(0isize) as ::core::ffi::c_int
            & (((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as BYTE
                as ::core::ffi::c_int
                | ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as BYTE
                    as ::core::ffi::c_int) != 0
            && *(&raw mut PlayerControl as *mut BYTE).offset(1isize)
                as ::core::ffi::c_int
                & (((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as BYTE
                    as ::core::ffi::c_int
                    | ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as BYTE
                        as ::core::ffi::c_int) != 0)
            && (optGodModes == GodModeType
                && (*(&raw mut PlayerControl as *mut BYTE).offset(0isize)
                    as ::core::ffi::c_int
                    & (((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as BYTE
                        as ::core::ffi::c_int
                        | ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as BYTE
                            as ::core::ffi::c_int) != 0
                    && (*ElementPtr).playerNr as ::core::ffi::c_int
                        == if SwapBool.0 != 0 {
                            0 as ::core::ffi::c_int
                        } else {
                            1 as ::core::ffi::c_int
                        }
                    || *(&raw mut PlayerControl as *mut BYTE).offset(1isize)
                        as ::core::ffi::c_int
                        & (((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as BYTE
                            as ::core::ffi::c_int
                            | ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                                as BYTE as ::core::ffi::c_int) != 0
                        && (*ElementPtr).playerNr as ::core::ffi::c_int
                            == if SwapBool.0 != 0 {
                                1 as ::core::ffi::c_int
                            } else {
                                0 as ::core::ffi::c_int
                            }))) as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
}
