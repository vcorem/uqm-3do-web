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

// Rust translation of src/uqm/shipstat.c.
extern "C" {
    pub type string_table_entry;
    pub type context_desc;
    pub type frame_desc;
    pub type font_desc;
    pub type tfb_soundsample;
    fn __assert_fail(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    ) -> !;
    fn SetContext(Context: CONTEXT) -> CONTEXT;
    fn SetContextForeGroundColor(Color: Color) -> Color;
    fn SetContextClipRect(pRect: *mut RECT) -> BOOLEAN;
    fn GetContextClipRect(pRect: *mut RECT) -> BOOLEAN;
    fn SetContextOrigin(orgOffset: POINT) -> POINT;
    fn DrawStamp(pStamp: *mut STAMP);
    fn DrawFilledStamp(pStamp: *mut STAMP);
    fn DrawFilledRectangle(pRect: *mut RECT);
    fn font_DrawText(pText: *mut TEXT);
    fn BatchGraphics();
    fn UnbatchGraphics();
    fn GetFrameRect(Frame: FRAME, pRect: *mut RECT) -> BOOLEAN;
    fn SetContextFont(Font: FONT) -> FONT;
    fn SetAbsFrameIndex(Frame: FRAME, FrameIndex: COUNT) -> FRAME;
    fn IncFrameIndex(Frame: FRAME) -> FRAME;
    fn SetAbsStringTableIndex(String: STRING, StringTableIndex: COUNT) -> STRING;
    fn GetStringLength(String: STRING) -> COUNT;
    fn GetStringAddress(String: STRING) -> STRINGPTR;
    fn utf8StringCount(start: *const ::core::ffi::c_char) -> size_t;
    fn utf8StringCopy(
        dst: *mut ::core::ffi::c_char,
        size: size_t,
        src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_uchar;
    fn sprintf(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    static mut optWhichMenu: ::core::ffi::c_int;
    static mut resolutionFactor: ::core::ffi::c_uint;
    static mut optWindowType: DWORD;
    static mut GlobData: GLOBDATA;
    fn isPC(optWhich: ::core::ffi::c_int) -> OPT_CONSOLETYPE;
    fn is3DO(optWhich: ::core::ffi::c_int) -> OPT_CONSOLETYPE;
    fn replaceChar(
        pStr: *mut ::core::ffi::c_char,
        find: ::core::ffi::c_char,
        replace: ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    static mut status_y_offsets: [COORD; 0];
    fn DrawBattleCrewAmount(ShipInfoPtr: *mut SHIP_INFO, y_offs: COORD);
    static mut StatusFrame: FRAME;
    static mut StatusContext: CONTEXT;
    static mut StarConFont: FONT;
    static mut TinyFont: FONT;
    static mut TinyFontBold: FONT;
    fn DrawRenderedBox(
        r: *mut RECT,
        filled: BOOLEAN,
        fill_color: Color,
        r#type: ::core::ffi::c_int,
        custom: ::core::ffi::c_int,
    );
}
pub type size_t = ::core::ffi::c_ulong;
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
pub struct Color {
    pub r: BYTE,
    pub g: BYTE,
    pub b: BYTE,
    pub a: BYTE,
}
pub type RESOURCE = *const ::core::ffi::c_char;
pub type STRING_TABLE_ENTRY_DESC = string_table_entry;
pub type STRING = *mut STRING_TABLE_ENTRY_DESC;
pub type STRINGPTR = *mut ::core::ffi::c_char;
pub type CONTEXT_DESC = context_desc;
pub type FRAME_DESC = frame_desc;
pub type FONT_DESC = font_desc;
pub type CONTEXT = *mut CONTEXT_DESC;
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
pub type SOUND = STRING;
pub type TFB_SoundSample = tfb_soundsample;
pub type MUSIC_REF = *mut *mut TFB_SoundSample;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct OPT_CONSOLETYPE(pub ::core::ffi::c_uint);
impl OPT_CONSOLETYPE {
    pub const OPTVAL_PC: Self = Self(0);
    pub const OPTVAL_3DO: Self = Self(1);
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
pub struct C2Rust_Unnamed(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed {
    pub const DOS_MENU_HILITE: Self = Self(0);
    pub const SPECIAL_BEVEL: Self = Self(1);
    pub const THIN_INNER_BEVEL: Self = Self(9);
    pub const THICK_INNER_BEVEL: Self = Self(17);
    pub const THIN_OUTER_BEVEL: Self = Self(25);
    pub const THICK_OUTER_BEVEL: Self = Self(33);
    pub const THICK_OUT_INV_BEVEL: Self = Self(41);
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
// Presentation palette only. Keep collision, control and gauge semantics intact.
unsafe fn melee_ui_active() -> bool {
    extern "C" { fn UQM_GetMeleeRemastered() -> ::core::ffi::c_int; }
    UQM_GetMeleeRemastered() != 0 &&
        crate::src::uqm::globdata::GlobData.Game_state.CurrentActivity & 0xff == 0
}
unsafe fn melee_ui_color(mut c: Color) -> Color {
    if !melee_ui_active() || c.a != 255 { return c; }
    let next = match (c.r, c.g, c.b) {
        (0, 0, 74) => (38, 62, 77),
        (0, 0, 115) => (49, 79, 96),
        (0, 0, 99) => (20, 37, 47),
        (57, 0, 99) => (141, 225, 223),
        (99, 0, 165) => (71, 125, 136),
        (82, 0, 140) => (28, 57, 73),
        (0, 0, 165) => (18, 35, 45),
        (0, 82, 165) => (87, 123, 140),
        (165, 0, 165) => (33, 63, 80),
        (2, 4, 62) => (18, 38, 49),
        (4, 8, 124) => (30, 65, 83),
        (35, 140, 210) => (157, 189, 207),
        (40, 160, 240) => (201, 242, 245),
        (40, 40, 124) => (25, 48, 59),
        (60, 60, 186) => (49, 89, 103),
        (31, 0, 31) => (18, 35, 45),
        (62, 0, 62) => (49, 89, 103),
        (66, 66, 66) => (38, 60, 73),
        (87, 87, 87) => (26, 42, 54),
        (85, 85, 85) => (26, 42, 54),
        (82, 82, 82) => (20, 34, 45),
        (132, 132, 132) => (74, 107, 123),
        (116, 116, 116) => (166, 191, 204),
        (138, 138, 138) => (175, 201, 215),
        (69, 69, 69) => (35, 59, 73),
        (248, 224, 0) => (167, 240, 227),
        (123, 132, 222) => (155, 182, 200),
        (189, 198, 239) => (222, 240, 246),
        (50, 50, 155) => (135, 182, 200),
        _ => return c,
    };
    c.r=next.0; c.g=next.1; c.b=next.2;
    c
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
    return melee_ui_color(c);
}
// Gauges keep the exact original color semantics, including robot crew.
unsafe fn raw_color_from_rgb15(r: BYTE, g: BYTE, b: BYTE) -> Color {
    Color { r: (r << 3) | (r >> 2), g: (g << 3) | (g >> 2), b: (b << 3) | (b >> 2), a: 255 }
}

#[inline]
unsafe extern "C" fn MAKE_POINT(mut x: COORD, mut y: COORD) -> POINT {
    let mut pt: POINT = point { x: x, y: y };
    return pt;
}
#[no_mangle]
pub unsafe extern "C" fn DrawCrewFuelString(mut y: COORD, mut state: SIZE) {
    let mut Stamp: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    Stamp.origin.y = (y as ::core::ffi::c_int
        + (((65 as ::core::ffi::c_int) << resolutionFactor)
            - ((10 as ::core::ffi::c_int) << resolutionFactor))
        + ((7 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    if state as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        Stamp.origin.x = (((4 as ::core::ffi::c_int) << resolutionFactor)
            + ((((2 as ::core::ffi::c_int) << resolutionFactor) * 2 as ::core::ffi::c_int
                + ((3 as ::core::ffi::c_int) << resolutionFactor) >> resolutionFactor
                >> 1 as ::core::ffi::c_int) << resolutionFactor)
            + ((5 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        if optWhichMenu == 0x2 as ::core::ffi::c_int {
            Stamp.frame = SetAbsFrameIndex(StatusFrame, 4 as COUNT);
        } else {
            Stamp.frame = SetAbsFrameIndex(StatusFrame, 0 as COUNT);
        }
        if melee_ui_active() {
            let old_color = SetContextForeGroundColor(Color { r:94, g:214, b:153, a:255 });
            uqm_lua_checked!(DrawFilledStamp(&raw mut Stamp), ());
            SetContextForeGroundColor(old_color);
        } else {
            uqm_lua_checked!(DrawStamp(&raw mut Stamp), ());
        }
    }
    Stamp.origin.x = (((52 as ::core::ffi::c_int) << resolutionFactor)
        + ((((2 as ::core::ffi::c_int) << resolutionFactor) * 2 as ::core::ffi::c_int
            + ((3 as ::core::ffi::c_int) << resolutionFactor) >> resolutionFactor
            >> 1 as ::core::ffi::c_int) << resolutionFactor)
        - ((4 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    if optWhichMenu == 0x2 as ::core::ffi::c_int {
        Stamp.frame = SetAbsFrameIndex(StatusFrame, 5 as COUNT);
    } else {
        Stamp.frame = SetAbsFrameIndex(StatusFrame, 1 as COUNT);
    }
    if state as ::core::ffi::c_int >= 0 as ::core::ffi::c_int {
        if if resolutionFactor != 2 as ::core::ffi::c_uint {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } != 0
        {
            let mut r: RECT = RECT {
                corner: POINT { x: 0, y: 0 },
                extent: EXTENT { width: 0, height: 0 },
            };
            GetFrameRect(Stamp.frame, &raw mut r);
            r.corner.x = (r.corner.x as ::core::ffi::c_int
                + Stamp.origin.x as ::core::ffi::c_int) as COORD;
            r.corner.y = (r.corner.y as ::core::ffi::c_int
                + Stamp.origin.y as ::core::ffi::c_int) as COORD;
            SetContextForeGroundColor(
                colorFromRgb15(0xa as BYTE, 0xa as BYTE, 0xa as BYTE),
            );
            uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
        }
        if melee_ui_active() {
            let old_color = SetContextForeGroundColor(Color { r:235, g:113, b:105, a:255 });
            uqm_lua_checked!(DrawFilledStamp(&raw mut Stamp), ());
            SetContextForeGroundColor(old_color);
        } else {
            uqm_lua_checked!(DrawStamp(&raw mut Stamp), ());
        }
    } else {
        SetContextForeGroundColor(
            if melee_ui_active() { Color { r:255, g:224, b:107, a:255 } }
            else { colorFromRgb15(0x1f as BYTE, 0x1f as BYTE, 0xa as BYTE) },
        );
        uqm_lua_checked!(DrawFilledStamp(&raw mut Stamp), ());
    };
}
unsafe extern "C" fn DrawShipNameString(
    mut pStr: *mut UNICODE,
    mut CharCount: COUNT,
    mut y: COORD,
) {
    let mut Text: TEXT = TEXT {
        baseline: POINT { x: 0, y: 0 },
        pStr: ::core::ptr::null::<UNICODE>(),
        align: TEXT_ALIGN::ALIGN_LEFT,
        CharCount: 0,
    };
    let mut OldFont: FONT = ::core::ptr::null_mut::<FONT_DESC>();
    OldFont = SetContextFont(StarConFont);
    Text.pStr = pStr;
    Text.CharCount = CharCount;
    Text.align = TEXT_ALIGN::ALIGN_CENTER;
    Text.baseline.y = (((7 as ::core::ffi::c_int) << resolutionFactor)
        + ((3 as ::core::ffi::c_int) << resolutionFactor) + y as ::core::ffi::c_int)
        as COORD;
    Text.baseline.x = ((64 as ::core::ffi::c_int) << resolutionFactor
        >> 1 as ::core::ffi::c_int) as COORD;
    SetContextForeGroundColor(colorFromRgb15(0x10 as BYTE, 0x10 as BYTE, 0x10 as BYTE));
    uqm_lua_checked!(font_DrawText(&raw mut Text), ());
    Text.baseline.y = (Text.baseline.y as ::core::ffi::c_int
        - ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    SetContextForeGroundColor(if melee_ui_active() { Color { r:214, g:232, b:240, a:255 } } else { colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE) });
    uqm_lua_checked!(font_DrawText(&raw mut Text), ());
    SetContextFont(OldFont);
}
#[no_mangle]
pub unsafe extern "C" fn ClearShipStatus(mut y: COORD) {
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    SetContextForeGroundColor(colorFromRgb15(0xa as BYTE, 0xa as BYTE, 0xa as BYTE));
    r.corner.x = ((2 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    r.corner.y = (((3 as ::core::ffi::c_int) << resolutionFactor)
        + y as ::core::ffi::c_int) as COORD;
    r.extent.width = (((64 as ::core::ffi::c_int) << resolutionFactor)
        - ((4 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    r.extent.height = (((65 as ::core::ffi::c_int) << resolutionFactor)
        - ((3 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
}
#[no_mangle]
pub unsafe extern "C" fn OutlineShipStatus(mut y: COORD) {
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    SetContextForeGroundColor(colorFromRgb15(0x8 as BYTE, 0x8 as BYTE, 0x8 as BYTE));
    r.corner.x = 0 as COORD;
    r.corner.y = (((1 as ::core::ffi::c_int) << resolutionFactor)
        + y as ::core::ffi::c_int) as COORD;
    r.extent.width = ((64 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    r.extent.height = ((1 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
    r.corner.y = (r.corner.y as ::core::ffi::c_int
        + ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    r.extent.width = (r.extent.width as ::core::ffi::c_int
        - ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
    r.extent.width = ((1 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    r.extent.height = (((65 as ::core::ffi::c_int) << resolutionFactor)
        - ((2 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
    r.corner.x = (r.corner.x as ::core::ffi::c_int
        + ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
    SetContextForeGroundColor(colorFromRgb15(0x10 as BYTE, 0x10 as BYTE, 0x10 as BYTE));
    r.corner.x = (((64 as ::core::ffi::c_int) << resolutionFactor)
        - ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
    r.corner.x = (((64 as ::core::ffi::c_int) << resolutionFactor)
        - ((2 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    r.corner.y = (r.corner.y as ::core::ffi::c_int
        + ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    r.extent.height = (r.extent.height as ::core::ffi::c_int
        - ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
    SetContextForeGroundColor(colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE));
    r.corner.x = 0 as COORD;
    r.corner.y = y;
    r.extent.width = ((64 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    r.extent.height = ((1 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
}
#[no_mangle]
pub unsafe extern "C" fn InitShipStatus(
    mut SIPtr: *mut SHIP_INFO,
    mut StarShipPtr: *mut STARSHIP,
    mut pClipRect: *mut RECT,
    mut inMeleeMenu: BOOLEAN,
) {
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut y: COORD = 0 as COORD;
    let mut Stamp: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    let mut OldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    let mut oldClipRect: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut oldOrigin: POINT = point {
        x: 0 as COORD,
        y: 0 as COORD,
    };
    if !StarShipPtr.is_null() {
        ((*StarShipPtr).playerNr as ::core::ffi::c_int >= 0 as ::core::ffi::c_int
            || {
                __assert_fail(
                    b"StarShipPtr->playerNr >= 0\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"src/uqm/shipstat.c\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    161 as ::core::ffi::c_int,
                    b"InitShipStatus\0".as_ptr() as *const ::core::ffi::c_char,
                );
                0 as ::core::ffi::c_int != 0
            }) as ::core::ffi::c_int;
        y = *(&raw mut status_y_offsets as *mut COORD)
            .offset((*StarShipPtr).playerNr as isize);
        let mut cs: *mut CAPTAIN_STUFF = &raw mut (*(*StarShipPtr).RaceDescPtr)
            .ship_data
            .captain_control;
        (*cs).tl_offset = 0 as BYTE;
        (*cs).tr_offset = 0 as BYTE;
        (*cs).thrust_offset = 0 as BYTE;
        (*cs).weapon_offset = 0 as BYTE;
        (*cs).special_offset = 0 as BYTE;
    }
    OldContext = SetContext(StatusContext);
    if !pClipRect.is_null() {
        GetContextClipRect(&raw mut oldClipRect);
        r = oldClipRect;
        r.corner.x = (r.corner.x as ::core::ffi::c_int
            + (*pClipRect).corner.x as ::core::ffi::c_int) as COORD;
        r.corner.y = (r.corner.y as ::core::ffi::c_int
            + ((*pClipRect).corner.y as ::core::ffi::c_int & !(1 as ::core::ffi::c_int)))
            as COORD;
        r.extent = (*pClipRect).extent;
        r.extent.height = (r.extent.height as ::core::ffi::c_int
            + ((*pClipRect).corner.y as ::core::ffi::c_int & 1 as ::core::ffi::c_int))
            as COORD;
        SetContextClipRect(&raw mut r);
        oldOrigin = SetContextOrigin(
            MAKE_POINT(
                -((*pClipRect).corner.x as ::core::ffi::c_int) as COORD,
                -((*pClipRect).corner.y as ::core::ffi::c_int
                    & !(1 as ::core::ffi::c_int)) as COORD,
            ),
        );
    }
    BatchGraphics();
    if inMeleeMenu.0 == 0 {
        uqm_lua_checked!(OutlineShipStatus(y), ());
    }
    uqm_lua_checked!(ClearShipStatus(y), ());
    Stamp.origin.x = ((64 as ::core::ffi::c_int) << resolutionFactor
        >> 1 as ::core::ffi::c_int) as COORD;
    Stamp.origin.y = (((31 as ::core::ffi::c_int) << resolutionFactor)
        + y as ::core::ffi::c_int) as COORD;
    Stamp.frame = IncFrameIndex((*SIPtr).icons);
    uqm_lua_checked!(DrawStamp(&raw mut Stamp), ());
    let mut crew_height: SIZE = 0;
    let mut energy_height: SIZE = 0;
    if if resolutionFactor != 2 as ::core::ffi::c_uint {
        BOOLEAN::FALSE.0 as ::core::ffi::c_int
    } else {
        BOOLEAN::TRUE.0 as ::core::ffi::c_int
    } == 0
    {
        crew_height = ((if (*SIPtr).max_crew as ::core::ffi::c_int
            <= 42 as ::core::ffi::c_int
        {
            (*SIPtr).max_crew as ::core::ffi::c_int
        } else {
            42 as ::core::ffi::c_int
        } + 1 as ::core::ffi::c_int & !(1 as ::core::ffi::c_int))
            + 1 as ::core::ffi::c_int) as SIZE;
    } else {
        crew_height = ((if (*SIPtr).max_crew as ::core::ffi::c_int
            <= 42 as ::core::ffi::c_int
        {
            (*SIPtr).max_crew as ::core::ffi::c_int
        } else {
            42 as ::core::ffi::c_int
        } + 1 as ::core::ffi::c_int) * ((1 as ::core::ffi::c_int) << resolutionFactor))
            as SIZE;
    }
    energy_height = ((((*SIPtr).max_energy as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
        << if if resolutionFactor != 2 as ::core::ffi::c_uint {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } == 0
        {
            1 as ::core::ffi::c_int
        } else {
            1 as ::core::ffi::c_int * 3 as ::core::ffi::c_int
        }) + ((1 as ::core::ffi::c_int) << resolutionFactor)) as SIZE;
    if if resolutionFactor != 2 as ::core::ffi::c_uint {
        BOOLEAN::FALSE.0 as ::core::ffi::c_int
    } else {
        BOOLEAN::TRUE.0 as ::core::ffi::c_int
    } == 0
    {
        SetContextForeGroundColor(colorFromRgb15(0x8 as BYTE, 0x8 as BYTE, 0x8 as BYTE));
        r.corner.x = (((4 as ::core::ffi::c_int) << resolutionFactor)
            - ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        r.corner.y = (((65 as ::core::ffi::c_int) << resolutionFactor)
            - ((10 as ::core::ffi::c_int) << resolutionFactor)
            + ((1 as ::core::ffi::c_int) << resolutionFactor) + y as ::core::ffi::c_int)
            as COORD;
        r.extent.width = (((2 as ::core::ffi::c_int) << resolutionFactor)
            * 2 as ::core::ffi::c_int + ((3 as ::core::ffi::c_int) << resolutionFactor)
            + ((2 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        r.extent.height = ((1 as ::core::ffi::c_int) << resolutionFactor) as COORD;
        uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
        r.corner.x = (((52 as ::core::ffi::c_int) << resolutionFactor)
            - ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
        r.corner.x = (((52 as ::core::ffi::c_int) << resolutionFactor)
            + (((2 as ::core::ffi::c_int) << resolutionFactor) * 2 as ::core::ffi::c_int
                + ((3 as ::core::ffi::c_int) << resolutionFactor))) as COORD;
        r.corner.y = (r.corner.y as ::core::ffi::c_int
            - energy_height as ::core::ffi::c_int) as COORD;
        r.extent.width = ((1 as ::core::ffi::c_int) << resolutionFactor) as COORD;
        r.extent.height = energy_height as COORD;
        uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
        r.corner.x = (((4 as ::core::ffi::c_int) << resolutionFactor)
            + (((2 as ::core::ffi::c_int) << resolutionFactor) * 2 as ::core::ffi::c_int
                + ((3 as ::core::ffi::c_int) << resolutionFactor))) as COORD;
        r.corner.y = (((65 as ::core::ffi::c_int) << resolutionFactor)
            - ((10 as ::core::ffi::c_int) << resolutionFactor)
            + ((1 as ::core::ffi::c_int) << resolutionFactor) + y as ::core::ffi::c_int
            - crew_height as ::core::ffi::c_int) as COORD;
        r.extent.width = ((1 as ::core::ffi::c_int) << resolutionFactor) as COORD;
        r.extent.height = crew_height as COORD;
        uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
        SetContextForeGroundColor(
            colorFromRgb15(0x10 as BYTE, 0x10 as BYTE, 0x10 as BYTE),
        );
        r.corner.x = (((4 as ::core::ffi::c_int) << resolutionFactor)
            - ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        r.corner.y = (((65 as ::core::ffi::c_int) << resolutionFactor)
            - ((10 as ::core::ffi::c_int) << resolutionFactor)
            - crew_height as ::core::ffi::c_int + y as ::core::ffi::c_int) as COORD;
        r.extent.width = (((2 as ::core::ffi::c_int) << resolutionFactor)
            * 2 as ::core::ffi::c_int + ((3 as ::core::ffi::c_int) << resolutionFactor)
            + ((2 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        r.extent.height = ((1 as ::core::ffi::c_int) << resolutionFactor) as COORD;
        uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
        r.corner.x = (((52 as ::core::ffi::c_int) << resolutionFactor)
            - ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        r.corner.y = (((65 as ::core::ffi::c_int) << resolutionFactor)
            - ((10 as ::core::ffi::c_int) << resolutionFactor)
            - energy_height as ::core::ffi::c_int + y as ::core::ffi::c_int) as COORD;
        uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
        r.extent.width = ((1 as ::core::ffi::c_int) << resolutionFactor) as COORD;
        r.extent.height = (energy_height as ::core::ffi::c_int
            + ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
        r.corner.x = (((4 as ::core::ffi::c_int) << resolutionFactor)
            - ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        r.corner.y = (((65 as ::core::ffi::c_int) << resolutionFactor)
            - ((10 as ::core::ffi::c_int) << resolutionFactor)
            - crew_height as ::core::ffi::c_int + y as ::core::ffi::c_int) as COORD;
        r.extent.height = (crew_height as ::core::ffi::c_int
            + ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
        SetContextForeGroundColor(colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE));
        r.extent.width = (((2 as ::core::ffi::c_int) << resolutionFactor)
            * 2 as ::core::ffi::c_int + ((3 as ::core::ffi::c_int) << resolutionFactor))
            as COORD;
        r.corner.x = ((4 as ::core::ffi::c_int) << resolutionFactor) as COORD;
        r.extent.height = crew_height as COORD;
        r.corner.y = (y as ::core::ffi::c_int - r.extent.height as ::core::ffi::c_int
            + (((65 as ::core::ffi::c_int) << resolutionFactor)
                - ((10 as ::core::ffi::c_int) << resolutionFactor))
            + ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
        r.extent.width = (((2 as ::core::ffi::c_int) << resolutionFactor)
            * 2 as ::core::ffi::c_int + ((3 as ::core::ffi::c_int) << resolutionFactor))
            as COORD;
        r.corner.x = ((52 as ::core::ffi::c_int) << resolutionFactor) as COORD;
        r.extent.height = energy_height as COORD;
        r.corner.y = (y as ::core::ffi::c_int - r.extent.height as ::core::ffi::c_int
            + (((65 as ::core::ffi::c_int) << resolutionFactor)
                - ((10 as ::core::ffi::c_int) << resolutionFactor))
            + ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
    } else {
        r.corner.x = (((4 as ::core::ffi::c_int) << resolutionFactor)
            - 4 as ::core::ffi::c_int) as COORD;
        r.corner.y = (((65 as ::core::ffi::c_int) << resolutionFactor)
            - ((10 as ::core::ffi::c_int) << resolutionFactor)
            - crew_height as ::core::ffi::c_int + y as ::core::ffi::c_int) as COORD;
        r.extent.width = (((2 as ::core::ffi::c_int) << resolutionFactor)
            * 2 as ::core::ffi::c_int + ((3 as ::core::ffi::c_int) << resolutionFactor)
            + 8 as ::core::ffi::c_int) as COORD;
        r.extent.height = (crew_height as ::core::ffi::c_int + 8 as ::core::ffi::c_int)
            as COORD;
        uqm_lua_checked!(DrawRenderedBox(
            &raw mut r,
            BOOLEAN::TRUE,
            colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE),
            C2Rust_Unnamed::THIN_INNER_BEVEL.0 as ::core::ffi::c_int,
            BOOLEAN::FALSE.0 as ::core::ffi::c_int,
        ), ());
        r.corner.x = (((52 as ::core::ffi::c_int) << resolutionFactor)
            - 4 as ::core::ffi::c_int) as COORD;
        r.corner.y = (((65 as ::core::ffi::c_int) << resolutionFactor)
            - ((10 as ::core::ffi::c_int) << resolutionFactor)
            - energy_height as ::core::ffi::c_int + y as ::core::ffi::c_int) as COORD;
        r.extent.width = (((2 as ::core::ffi::c_int) << resolutionFactor)
            * 2 as ::core::ffi::c_int + ((3 as ::core::ffi::c_int) << resolutionFactor)
            + 8 as ::core::ffi::c_int) as COORD;
        r.extent.height = (energy_height as ::core::ffi::c_int + 8 as ::core::ffi::c_int)
            as COORD;
        uqm_lua_checked!(DrawRenderedBox(
            &raw mut r,
            BOOLEAN::TRUE,
            colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE),
            C2Rust_Unnamed::THIN_INNER_BEVEL.0 as ::core::ffi::c_int,
            BOOLEAN::FALSE.0 as ::core::ffi::c_int,
        ), ());
    }
    if StarShipPtr.is_null()
        || (*StarShipPtr).captains_name_index as ::core::ffi::c_int != 0
    {
        let mut locString: STRING = ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>();
        uqm_lua_checked!(DrawCrewFuelString(y, 0 as SIZE), ());
        locString = SetAbsStringTableIndex((*SIPtr).race_strings, 1 as COUNT);
        uqm_lua_checked!(DrawShipNameString(GetStringAddress(locString), GetStringLength(locString), y), ());
        let mut buf: [UNICODE; 30] = [0; 30];
        let mut Text: TEXT = TEXT {
            baseline: POINT { x: 0, y: 0 },
            pStr: ::core::ptr::null::<UNICODE>(),
            align: TEXT_ALIGN::ALIGN_LEFT,
            CharCount: 0,
        };
        let mut OldFont: FONT = ::core::ptr::null_mut::<FONT_DESC>();
        OldFont = SetContextFont(
            if isPC(optWhichMenu).0 != 0 { TinyFont } else { TinyFontBold },
        );
        if StarShipPtr.is_null() {
            sprintf(
                &raw mut buf as *mut ::core::ffi::c_char,
                b"%d\0".as_ptr() as *const ::core::ffi::c_char,
                (*SIPtr).ship_cost as ::core::ffi::c_int,
            );
            Text.pStr = &raw mut buf as *mut UNICODE;
            Text.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
        } else {
            locString = SetAbsStringTableIndex(
                (*SIPtr).race_strings,
                (*StarShipPtr).captains_name_index as COUNT,
            );
            utf8StringCopy(
                &raw mut buf as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[UNICODE; 30]>() as size_t,
                GetStringAddress(locString),
            );
            if is3DO(optWhichMenu).0 != 0 {
                replaceChar(
                    &raw mut buf as *mut ::core::ffi::c_char,
                    0x20 as ::core::ffi::c_char,
                    0x9 as ::core::ffi::c_char,
                );
            }
            Text.pStr = &raw mut buf as *mut UNICODE;
            Text.CharCount = utf8StringCount(&raw mut buf as *mut UNICODE) as COUNT;
        }
        Text.align = TEXT_ALIGN::ALIGN_CENTER;
        Text.baseline.x = ((64 as ::core::ffi::c_int) << resolutionFactor
            >> 1 as ::core::ffi::c_int) as COORD;
        Text.baseline.y = (y as ::core::ffi::c_int
            + (((65 as ::core::ffi::c_int) << resolutionFactor)
                - ((10 as ::core::ffi::c_int) << resolutionFactor))
            + ((3 as ::core::ffi::c_int) << resolutionFactor)
            + (if isPC(optWhichMenu).0 != 0 {
                0 as ::core::ffi::c_int
            } else {
                6 as ::core::ffi::c_int
            } << resolutionFactor)) as COORD;
        SetContextForeGroundColor(if melee_ui_active() { Color { r:214, g:232, b:240, a:255 } } else { colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE) });
        uqm_lua_checked!(font_DrawText(&raw mut Text), ());
        SetContextFont(OldFont);
    } else if (*StarShipPtr).playerNr as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        uqm_lua_checked!(DrawCrewFuelString(y, 0 as SIZE), ());
        uqm_lua_checked!(DrawShipNameString(
            &raw mut GlobData.SIS_state.ShipName as *mut UNICODE,
            !(0 as ::core::ffi::c_int) as COUNT,
            y,
        ), ());
    }
    let mut crew_delta: SIZE = 0;
    let mut energy_delta: SIZE = 0;
    crew_delta = (*SIPtr).crew_level as SIZE;
    energy_delta = (*SIPtr).energy_level as SIZE;
    (*SIPtr).crew_level = 0 as COUNT;
    (*SIPtr).energy_level = 0 as BYTE;
    uqm_lua_checked!(DeltaStatistics(SIPtr, y, crew_delta, energy_delta), ());
    UnbatchGraphics();
    if !pClipRect.is_null() {
        SetContextOrigin(oldOrigin);
        SetContextClipRect(&raw mut oldClipRect);
    }
    SetContext(OldContext);
}
#[no_mangle]
pub unsafe extern "C" fn DeltaStatistics(
    mut ShipInfoPtr: *mut SHIP_INFO,
    mut y_offs: COORD,
    mut crew_delta: SIZE,
    mut energy_delta: SIZE,
) {
    let mut x: COORD = 0;
    let mut y: COORD = 0;
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    if crew_delta as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        && energy_delta as ::core::ffi::c_int == 0 as ::core::ffi::c_int
    {
        return;
    }
    x = 0 as COORD;
    y = (((65 as ::core::ffi::c_int) << resolutionFactor)
        - ((10 as ::core::ffi::c_int) << resolutionFactor)
        + y_offs as ::core::ffi::c_int) as COORD;
    r.extent.width = ((2 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    r.extent.height = ((1 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    if crew_delta as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
        let mut oldNumBlocks: COUNT = 0;
        let mut newNumBlocks: COUNT = 0;
        let mut blockI: COUNT = 0;
        let mut newCrewLevel: COUNT = 0;
        oldNumBlocks = (if (*ShipInfoPtr).crew_level as ::core::ffi::c_int
            <= 42 as ::core::ffi::c_int
        {
            (*ShipInfoPtr).crew_level as ::core::ffi::c_int
        } else {
            42 as ::core::ffi::c_int
        }) as COUNT;
        newCrewLevel = ((*ShipInfoPtr).crew_level as ::core::ffi::c_int
            + crew_delta as ::core::ffi::c_int) as COUNT;
        newNumBlocks = (if newCrewLevel as ::core::ffi::c_int <= 42 as ::core::ffi::c_int
        {
            newCrewLevel as ::core::ffi::c_int
        } else {
            42 as ::core::ffi::c_int
        }) as COUNT;
        if crew_delta as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
            r.corner.y = (y as ::core::ffi::c_int
                + ((1 as ::core::ffi::c_int) << resolutionFactor)
                - (oldNumBlocks as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                    >> 1 as ::core::ffi::c_int)
                    * (((1 as ::core::ffi::c_int) << resolutionFactor)
                        + ((1 as ::core::ffi::c_int) << resolutionFactor))) as COORD;
            SetContextForeGroundColor(
                if (*ShipInfoPtr).ship_flags as ::core::ffi::c_int
                    & (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int != 0
                {
                    raw_color_from_rgb15(0xa as BYTE, 0xa as BYTE, 0xa as BYTE)
                } else {
                    raw_color_from_rgb15(0 as BYTE, 0x14 as BYTE, 0 as BYTE)
                },
            );
            blockI = oldNumBlocks;
            while (blockI as ::core::ffi::c_int) < newNumBlocks as ::core::ffi::c_int {
                r.corner.x = (x as ::core::ffi::c_int
                    + (((4 as ::core::ffi::c_int) << resolutionFactor)
                        + ((1 as ::core::ffi::c_int) << resolutionFactor))) as COORD;
                if blockI as ::core::ffi::c_int & 1 as ::core::ffi::c_int == 0 {
                    r.corner.x = (r.corner.x as ::core::ffi::c_int
                        + (((2 as ::core::ffi::c_int) << resolutionFactor)
                            + ((1 as ::core::ffi::c_int) << resolutionFactor))) as COORD;
                    r.corner.y = (r.corner.y as ::core::ffi::c_int
                        - (((1 as ::core::ffi::c_int) << resolutionFactor)
                            + ((1 as ::core::ffi::c_int) << resolutionFactor))) as COORD;
                }
                uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
                blockI = blockI.wrapping_add(1);
            }
        } else {
            SetContextForeGroundColor(raw_color_from_rgb15(0 as BYTE, 0 as BYTE, 0 as BYTE));
            r.corner.y = (y as ::core::ffi::c_int
                + ((1 as ::core::ffi::c_int) << resolutionFactor)
                - (oldNumBlocks as ::core::ffi::c_int + 2 as ::core::ffi::c_int
                    >> 1 as ::core::ffi::c_int)
                    * (((1 as ::core::ffi::c_int) << resolutionFactor)
                        + ((1 as ::core::ffi::c_int) << resolutionFactor))) as COORD;
            blockI = oldNumBlocks;
            while blockI as ::core::ffi::c_int > newNumBlocks as ::core::ffi::c_int {
                r.corner.x = (x as ::core::ffi::c_int
                    + (((4 as ::core::ffi::c_int) << resolutionFactor)
                        + ((1 as ::core::ffi::c_int) << resolutionFactor)
                        + ((2 as ::core::ffi::c_int) << resolutionFactor)
                        + ((1 as ::core::ffi::c_int) << resolutionFactor))) as COORD;
                if blockI as ::core::ffi::c_int & 1 as ::core::ffi::c_int == 0 {
                    r.corner.x = (r.corner.x as ::core::ffi::c_int
                        - (((2 as ::core::ffi::c_int) << resolutionFactor)
                            + ((1 as ::core::ffi::c_int) << resolutionFactor))) as COORD;
                    r.corner.y = (r.corner.y as ::core::ffi::c_int
                        + (((1 as ::core::ffi::c_int) << resolutionFactor)
                            + ((1 as ::core::ffi::c_int) << resolutionFactor))) as COORD;
                }
                uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
                blockI = blockI.wrapping_sub(1);
            }
        }
        if (*ShipInfoPtr).ship_flags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int != 0
        {
            if (((*ShipInfoPtr).crew_level as ::core::ffi::c_int
                > 42 as ::core::ffi::c_int) as ::core::ffi::c_int
                != (newCrewLevel as ::core::ffi::c_int > 42 as ::core::ffi::c_int)
                    as ::core::ffi::c_int
                || (*ShipInfoPtr).crew_level as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int)
                && newCrewLevel as ::core::ffi::c_int != 0 as ::core::ffi::c_int
            {
                SetContextForeGroundColor(
                    if newCrewLevel as ::core::ffi::c_int > 42 as ::core::ffi::c_int {
                        raw_color_from_rgb15(0 as BYTE, 0x14 as BYTE, 0 as BYTE)
                    } else {
                        raw_color_from_rgb15(0xa as BYTE, 0xa as BYTE, 0x1f as BYTE)
                    },
                );
                r.corner.x = (x as ::core::ffi::c_int
                    + (((4 as ::core::ffi::c_int) << resolutionFactor)
                        + ((1 as ::core::ffi::c_int) << resolutionFactor))
                    + (((2 as ::core::ffi::c_int) << resolutionFactor)
                        + ((1 as ::core::ffi::c_int) << resolutionFactor))) as COORD;
                r.corner.y = (y as ::core::ffi::c_int
                    - ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
                uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
            }
        }
        (*ShipInfoPtr).crew_level = newCrewLevel;
        if (*ShipInfoPtr).max_crew as ::core::ffi::c_int > 42 as ::core::ffi::c_int
            || (*ShipInfoPtr).ship_flags as ::core::ffi::c_int
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
            uqm_lua_checked!(DrawBattleCrewAmount(ShipInfoPtr, y_offs), ());
        }
    }
    if energy_delta as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
        if energy_delta as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
            SetContextForeGroundColor(
                raw_color_from_rgb15(0x14 as BYTE, 0 as BYTE, 0 as BYTE),
            );
            r.corner.y = (y as ::core::ffi::c_int
                + ((1 as ::core::ffi::c_int) << resolutionFactor)
                - ((*ShipInfoPtr).energy_level as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
                    * (((1 as ::core::ffi::c_int) << resolutionFactor)
                        + ((1 as ::core::ffi::c_int) << resolutionFactor))) as COORD;
            loop {
                r.corner.x = (x as ::core::ffi::c_int
                    + (((52 as ::core::ffi::c_int) << resolutionFactor)
                        + ((1 as ::core::ffi::c_int) << resolutionFactor))) as COORD;
                if (*ShipInfoPtr).energy_level as ::core::ffi::c_int
                    & 1 as ::core::ffi::c_int == 0
                {
                    r.corner.x = (r.corner.x as ::core::ffi::c_int
                        + (((2 as ::core::ffi::c_int) << resolutionFactor)
                            + ((1 as ::core::ffi::c_int) << resolutionFactor))) as COORD;
                    r.corner.y = (r.corner.y as ::core::ffi::c_int
                        - (((1 as ::core::ffi::c_int) << resolutionFactor)
                            + ((1 as ::core::ffi::c_int) << resolutionFactor))) as COORD;
                }
                uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
                (*ShipInfoPtr).energy_level = (*ShipInfoPtr)
                    .energy_level
                    .wrapping_add(1);
                energy_delta -= 1;
                if energy_delta == 0 {
                    break;
                }
            }
        } else {
            SetContextForeGroundColor(raw_color_from_rgb15(0 as BYTE, 0 as BYTE, 0 as BYTE));
            r.corner.y = (y as ::core::ffi::c_int
                + ((1 as ::core::ffi::c_int) << resolutionFactor)
                - ((*ShipInfoPtr).energy_level as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
                    * (((1 as ::core::ffi::c_int) << resolutionFactor)
                        + ((1 as ::core::ffi::c_int) << resolutionFactor))) as COORD;
            loop {
                r.corner.x = (x as ::core::ffi::c_int
                    + (((52 as ::core::ffi::c_int) << resolutionFactor)
                        + ((1 as ::core::ffi::c_int) << resolutionFactor)
                        + ((2 as ::core::ffi::c_int) << resolutionFactor)
                        + ((1 as ::core::ffi::c_int) << resolutionFactor))) as COORD;
                if (*ShipInfoPtr).energy_level as ::core::ffi::c_int
                    & 1 as ::core::ffi::c_int == 0
                {
                    r.corner.x = (r.corner.x as ::core::ffi::c_int
                        - (((2 as ::core::ffi::c_int) << resolutionFactor)
                            + ((1 as ::core::ffi::c_int) << resolutionFactor))) as COORD;
                    r.corner.y = (r.corner.y as ::core::ffi::c_int
                        + (((1 as ::core::ffi::c_int) << resolutionFactor)
                            + ((1 as ::core::ffi::c_int) << resolutionFactor))) as COORD;
                }
                uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
                (*ShipInfoPtr).energy_level = (*ShipInfoPtr)
                    .energy_level
                    .wrapping_sub(1);
                energy_delta += 1;
                if energy_delta == 0 {
                    break;
                }
            }
        }
    }
}
