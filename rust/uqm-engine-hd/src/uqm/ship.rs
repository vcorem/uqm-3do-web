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

// Rust translation of src/uqm/ship.c.
extern "C" {
    pub type string_table_entry;
    pub type context_desc;
    pub type frame_desc;
    pub type drawable_desc;
    pub type tfb_soundsample;
    fn SetContext(Context: CONTEXT) -> CONTEXT;
    fn DrawStamp(pStamp: *mut STAMP);
    fn ApplyMask(layer: FRAME, base: FRAME, mode: DrawMode, fill: *mut Color);
    fn DestroyDrawable(Drawable: DRAWABLE) -> BOOLEAN;
    fn GetFrameCount(Frame: FRAME) -> COUNT;
    fn SetAbsFrameIndex(Frame: FRAME, FrameIndex: COUNT) -> FRAME;
    fn IncFrameIndex(Frame: FRAME) -> FRAME;
    fn ReleaseDrawable(Frame: FRAME) -> DRAWABLE;
    fn __assert_fail(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    ) -> !;
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
    fn PutQueue(pq: *mut QUEUE, hLink: HLINK);
    fn InsertQueue(pq: *mut QUEUE, hLink: HLINK, hRefLink: HLINK);
    fn PLRPlaying(MusicRef: MUSIC_REF) -> BOOLEAN;
    static mut disp_q: QUEUE;
    static mut DisplayArray: [PRIMITIVE; 330];
    fn AllocElement() -> HELEMENT;
    fn do_damage(ElementPtr: *mut ELEMENT, damage: SIZE);
    fn TimeSpaceMatterConflict(ElementPtr: *mut ELEMENT) -> BOOLEAN;
    fn CalculateGravity(ElementPtr: *mut ELEMENT) -> BOOLEAN;
    fn GetCurrentVelocityComponents(
        velocityptr: *mut VELOCITY_DESC,
        pdx: *mut SIZE,
        pdy: *mut SIZE,
    );
    fn SetVelocityVector(
        velocityptr: *mut VELOCITY_DESC,
        magnitude: SDWORD,
        facing: COUNT,
    );
    fn SetVelocityComponents(velocityptr: *mut VELOCITY_DESC, dx: SDWORD, dy: SDWORD);
    fn DeltaVelocityComponents(velocityptr: *mut VELOCITY_DESC, dx: SDWORD, dy: SDWORD);
    static mut GlobData: GLOBDATA;
    fn inHQSpace() -> BOOLEAN;
    fn TFB_Random() -> DWORD;
    fn load_ship(SpeciesID: SPECIES_ID, LoadBattleData: BOOLEAN) -> *mut RACE_DESC;
    static mut status_y_offsets: [COORD; 0];
    fn InitShipStatus(
        ShipInfoPtr: *mut SHIP_INFO,
        StarShipPtr: *mut STARSHIP,
        pClipRect: *mut RECT,
        inMeleeMenu: BOOLEAN,
    );
    fn DrawCaptainsWindow(StarShipPtr: *mut STARSHIP);
    fn DeltaEnergy(ElementPtr: *mut ELEMENT, energy_delta: SIZE) -> BOOLEAN;
    fn PreProcessStatus(ShipPtr: *mut ELEMENT);
    fn PostProcessStatus(ShipPtr: *mut ELEMENT);
    fn GetInitialMeleeStarShips(result: *mut HSTARSHIP) -> BOOLEAN;
    fn hyper_transition(ElementPtr: *mut ELEMENT) -> BOOLEAN;
    fn ship_transition(ElementPtr: *mut ELEMENT);
    fn OpponentAlive(TestStarShipPtr: *mut STARSHIP) -> BOOLEAN;
    fn ship_death(ShipPtr: *mut ELEMENT);
    fn spawn_ion_trail(ElementPtr: *mut ELEMENT, x: SIZE, y: SIZE);
    static mut StatusContext: CONTEXT;
    static mut race_q: [QUEUE; 0];
    fn GetEncounterStarShip(
        LastStarShipPtr: *mut STARSHIP,
        which_player: COUNT,
    ) -> HSTARSHIP;
    static mut GameSounds: SOUND;
    fn ProcessSound(Sound: SOUND, PositionalObject: *mut ELEMENT);
    fn DrawRenderedBox(
        r: *mut RECT,
        filled: BOOLEAN,
        fill_color: Color,
        r#type: ::core::ffi::c_int,
        custom: ::core::ffi::c_int,
    );
    fn GetColorMapColor(ColorMapIndex: COUNT, ColorIndex: COUNT) -> Color;
    fn GetPlayerOrder(i: COUNT) -> COUNT;
    fn BattleSong(DoPlay: BOOLEAN);
}
pub type uint8 = ::core::ffi::c_uchar;
pub type sint8 = ::core::ffi::c_schar;
pub type uint16 = ::core::ffi::c_ushort;
pub type sint16 = ::core::ffi::c_short;
pub type uint32 = ::core::ffi::c_uint;
pub type sint32 = ::core::ffi::c_int;
pub type BYTE = uint8;
pub type SBYTE = sint8;
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
pub type size_t = usize;
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
    pub const POINT_PRIM_HD: Self = Self(7);
    pub const NUM_PRIMS: Self = Self(8);
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
pub struct DrawKind(pub ::core::ffi::c_uint);
impl DrawKind {
    pub const DRAW_REPLACE: Self = Self(0);
    pub const DRAW_ADDITIVE: Self = Self(1);
    pub const DRAW_ALPHA: Self = Self(2);
    pub const DRAW_MULTIPLY: Self = Self(3);
    pub const DRAW_OVERLAY: Self = Self(4);
    pub const DRAW_SCREEN: Self = Self(5);
    pub const DRAW_GRAYSCALE: Self = Self(6);
    pub const DRAW_LINEARBURN: Self = Self(7);
    pub const DRAW_HYPTOQUAS: Self = Self(8);
    pub const DRAW_DESATURATE: Self = Self(9);
    pub const DRAW_DEFAULT: Self = Self(0);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DrawMode {
    pub kind: BYTE,
    pub factor: SWORD,
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
pub struct GLOBDATA {
    pub SIS_state: SIS_STATE,
    pub Game_state: GAME_STATE,
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
pub struct C2Rust_Unnamed_6(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_6 {
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_7(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_7 {
    pub const GRAB_CREW: Self = Self(0);
    pub const SHIP_EXPLODES: Self = Self(1);
    pub const TARGET_DAMAGED_FOR_1_PT: Self = Self(2);
    pub const TARGET_DAMAGED_FOR_2_3_PT: Self = Self(3);
    pub const TARGET_DAMAGED_FOR_4_5_PT: Self = Self(4);
    pub const TARGET_DAMAGED_FOR_6_PLUS_PT: Self = Self(5);
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
unsafe extern "C" fn buildColorRgba(
    mut r: BYTE,
    mut g: BYTE,
    mut b: BYTE,
    mut a: BYTE,
) -> Color {
    let mut c: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    c.r = r;
    c.g = g;
    c.b = b;
    c.a = a;
    return c;
}
#[inline]
unsafe extern "C" fn MAKE_DRAW_MODE(mut kind: DrawKind, mut factor: SWORD) -> DrawMode {
    let mut mode: DrawMode = DrawMode { kind: 0, factor: 0 };
    mode.kind = kind.0 as BYTE;
    mode.factor = factor;
    return mode;
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
#[inline]
unsafe extern "C" fn VelocitySquared(mut dx: SIZE, mut dy: SIZE) -> DWORD {
    return (dx as ::core::ffi::c_long * dx as ::core::ffi::c_long
        + dy as ::core::ffi::c_long * dy as ::core::ffi::c_long) as DWORD;
}
#[no_mangle]
pub unsafe extern "C" fn animation_preprocess(mut ElementPtr: *mut ELEMENT) {
    if (*ElementPtr).c2rust_unnamed_2.turn_wait as ::core::ffi::c_int
        > 0 as ::core::ffi::c_int
    {
        (*ElementPtr).c2rust_unnamed_2.turn_wait = (*ElementPtr)
            .c2rust_unnamed_2
            .turn_wait
            .wrapping_sub(1);
    } else {
        (*ElementPtr).next.image.frame = IncFrameIndex(
            (*ElementPtr).current.image.frame,
        );
        (*ElementPtr).state_flags = ((*ElementPtr).state_flags as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as ELEMENT_FLAGS;
        (*ElementPtr).c2rust_unnamed_2.turn_wait = (*ElementPtr)
            .c2rust_unnamed_3
            .next_turn;
    };
}
#[no_mangle]
pub unsafe extern "C" fn inertial_thrust(mut ElementPtr: *mut ELEMENT) -> STATUS_FLAGS {
    let mut CurrentAngle: COUNT = 0;
    let mut TravelAngle: COUNT = 0;
    let mut max_thrust: COUNT = 0;
    let mut thrust_increment: COUNT = 0;
    let mut VelocityPtr: *mut VELOCITY_DESC = ::core::ptr::null_mut::<VELOCITY_DESC>();
    let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
    VelocityPtr = &raw mut (*ElementPtr).velocity;
    StarShipPtr = (*ElementPtr).pParent as *mut STARSHIP;
    CurrentAngle = (((*StarShipPtr).ShipFacing as ::core::ffi::c_int)
        << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int) as COUNT;
    TravelAngle = (*VelocityPtr).TravelAngle;
    thrust_increment = (*(*StarShipPtr).RaceDescPtr).characteristics.thrust_increment;
    max_thrust = (*(*StarShipPtr).RaceDescPtr).characteristics.max_thrust;
    if thrust_increment as ::core::ffi::c_int == max_thrust as ::core::ffi::c_int {
        SetVelocityVector(VelocityPtr, max_thrust as SDWORD, (*StarShipPtr).ShipFacing);
        return ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as STATUS_FLAGS;
    } else if TravelAngle as ::core::ffi::c_int == CurrentAngle as ::core::ffi::c_int
        && (*StarShipPtr).cur_status_flags as ::core::ffi::c_int
            & ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) != 0
        && (*StarShipPtr).cur_status_flags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int == 0
    {
        return ((*StarShipPtr).cur_status_flags as ::core::ffi::c_int
            & ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)) as STATUS_FLAGS
    } else {
        let mut delta_x: SIZE = 0;
        let mut delta_y: SIZE = 0;
        let mut cur_delta_x: SIZE = 0;
        let mut cur_delta_y: SIZE = 0;
        let mut desired_speed: DWORD = 0;
        let mut max_speed: DWORD = 0;
        let mut current_speed: DWORD = 0;
        thrust_increment = ((thrust_increment as ::core::ffi::c_int)
            << 5 as ::core::ffi::c_int) as COUNT;
        GetCurrentVelocityComponents(
            VelocityPtr,
            &raw mut cur_delta_x,
            &raw mut cur_delta_y,
        );
        current_speed = VelocitySquared(cur_delta_x, cur_delta_y);
        delta_x = (cur_delta_x as SDWORD
            + (*(&raw mut sinetab as *mut SDWORD)
                .offset(
                    (CurrentAngle as ::core::ffi::c_int
                        + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                            >> 2 as ::core::ffi::c_int)
                        & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                            - 1 as ::core::ffi::c_int) as DWORD as isize,
                ) as ::core::ffi::c_long * thrust_increment as ::core::ffi::c_long
                >> 14 as ::core::ffi::c_int) as SDWORD) as SIZE;
        delta_y = (cur_delta_y as SDWORD
            + (*(&raw mut sinetab as *mut SDWORD)
                .offset(
                    (CurrentAngle as ::core::ffi::c_int
                        & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                            - 1 as ::core::ffi::c_int) as DWORD as isize,
                ) as ::core::ffi::c_long * thrust_increment as ::core::ffi::c_long
                >> 14 as ::core::ffi::c_int) as SDWORD) as SIZE;
        desired_speed = VelocitySquared(delta_x, delta_y);
        max_speed = VelocitySquared(
            ((max_thrust as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as SIZE,
            0 as SIZE,
        );
        if desired_speed <= max_speed {
            SetVelocityComponents(VelocityPtr, delta_x as SDWORD, delta_y as SDWORD);
        } else if (*StarShipPtr).cur_status_flags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int != 0
            && desired_speed
                <= ((((18 as ::core::ffi::c_int) << resolutionFactor
                    << 2 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as DWORD)
                    .wrapping_mul(
                        (((18 as ::core::ffi::c_int) << resolutionFactor
                            << 2 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int)
                            as DWORD,
                    ) || desired_speed < current_speed
        {
            SetVelocityComponents(VelocityPtr, delta_x as SDWORD, delta_y as SDWORD);
            return ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as STATUS_FLAGS;
        } else if TravelAngle as ::core::ffi::c_int == CurrentAngle as ::core::ffi::c_int
        {
            if current_speed <= max_speed {
                SetVelocityVector(
                    VelocityPtr,
                    max_thrust as SDWORD,
                    (*StarShipPtr).ShipFacing,
                );
            }
            return ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int)
                as STATUS_FLAGS;
        } else {
            let mut v: VELOCITY_DESC = *VelocityPtr;
            DeltaVelocityComponents(
                &raw mut v,
                (*(&raw mut sinetab as *mut SDWORD)
                    .offset(
                        (CurrentAngle as ::core::ffi::c_int
                            + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                                >> 2 as ::core::ffi::c_int)
                            & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                                - 1 as ::core::ffi::c_int) as DWORD as isize,
                    ) as ::core::ffi::c_long
                    * (thrust_increment as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
                        as ::core::ffi::c_long >> 14 as ::core::ffi::c_int) as SDWORD
                    - (*(&raw mut sinetab as *mut SDWORD)
                        .offset(
                            (TravelAngle as ::core::ffi::c_int
                                + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                                    >> 2 as ::core::ffi::c_int)
                                & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                                    - 1 as ::core::ffi::c_int) as DWORD as isize,
                        ) as ::core::ffi::c_long
                        * thrust_increment as ::core::ffi::c_long
                        >> 14 as ::core::ffi::c_int) as SDWORD,
                (*(&raw mut sinetab as *mut SDWORD)
                    .offset(
                        (CurrentAngle as ::core::ffi::c_int
                            & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                                - 1 as ::core::ffi::c_int) as DWORD as isize,
                    ) as ::core::ffi::c_long
                    * (thrust_increment as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
                        as ::core::ffi::c_long >> 14 as ::core::ffi::c_int) as SDWORD
                    - (*(&raw mut sinetab as *mut SDWORD)
                        .offset(
                            (TravelAngle as ::core::ffi::c_int
                                & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                                    - 1 as ::core::ffi::c_int) as DWORD as isize,
                        ) as ::core::ffi::c_long
                        * thrust_increment as ::core::ffi::c_long
                        >> 14 as ::core::ffi::c_int) as SDWORD,
            );
            GetCurrentVelocityComponents(
                &raw mut v,
                &raw mut cur_delta_x,
                &raw mut cur_delta_y,
            );
            desired_speed = VelocitySquared(cur_delta_x, cur_delta_y);
            if desired_speed > max_speed {
                if desired_speed < current_speed {
                    *VelocityPtr = v;
                }
                return ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                    as STATUS_FLAGS;
            }
            *VelocityPtr = v;
        }
        return 0 as STATUS_FLAGS;
    };
}
#[no_mangle]
pub unsafe extern "C" fn DrawHDMeleeBorder(mut StarShipPtr: *mut STARSHIP) {
    let mut y: COORD = 0 as COORD;
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    if GlobData.Game_state.CurrentActivity as BYTE as ::core::ffi::c_int
        == C2Rust_Unnamed_6::IN_LAST_BATTLE.0 as ::core::ffi::c_int
        && (*StarShipPtr).playerNr as ::core::ffi::c_int != 0 as ::core::ffi::c_int
    {
        return;
    }
    y = *(&raw mut status_y_offsets as *mut COORD)
        .offset((*StarShipPtr).playerNr as isize);
    r.corner.x = 0 as COORD;
    r.corner.y = (4 as ::core::ffi::c_int + y as ::core::ffi::c_int) as COORD;
    r.extent.width = ((64 as ::core::ffi::c_int) << resolutionFactor) as COORD;
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
        } << resolutionFactor) - 4 as ::core::ffi::c_int) as COORD;
    uqm_lua_checked!(DrawRenderedBox(
        &raw mut r,
        BOOLEAN::FALSE,
        buildColorRgba(0 as BYTE, 0 as BYTE, 0 as BYTE, 0 as BYTE),
        C2Rust_Unnamed_5::THICK_OUTER_BEVEL.0 as ::core::ffi::c_int,
        BOOLEAN::FALSE.0 as ::core::ffi::c_int,
    ), ());
}
#[no_mangle]
pub unsafe extern "C" fn ship_preprocess(mut ElementPtr: *mut ELEMENT) {
    let mut cur_status_flags: STATUS_FLAGS = 0;
    let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
    let mut RDPtr: *mut RACE_DESC = ::core::ptr::null_mut::<RACE_DESC>();
    StarShipPtr = (*ElementPtr).pParent as *mut STARSHIP;
    RDPtr = (*StarShipPtr).RaceDescPtr;
    cur_status_flags = ((*StarShipPtr).cur_status_flags as ::core::ffi::c_int
        & !((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)) as STATUS_FLAGS;
    if (*ElementPtr).state_flags as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int == 0
    {
        cur_status_flags = (cur_status_flags as ::core::ffi::c_int
            | (*StarShipPtr).ship_input_state as ::core::ffi::c_int
                & ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int))
            as STATUS_FLAGS;
    } else {
        (*ElementPtr).c2rust_unnamed_0.crew_level = (*RDPtr).ship_info.crew_level;
        if (*ElementPtr).playerNr as ::core::ffi::c_int == 1 as ::core::ffi::c_int
            && GlobData.Game_state.CurrentActivity as BYTE as ::core::ffi::c_int
                == C2Rust_Unnamed_6::IN_LAST_BATTLE.0 as ::core::ffi::c_int
        {
            let mut s: STAMP = STAMP {
                origin: POINT { x: 0, y: 0 },
                frame: ::core::ptr::null_mut::<FRAME_DESC>(),
            };
            let mut OldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
            OldContext = SetContext(StatusContext);
            s.origin.y = 0 as COORD;
            s.origin.x = s.origin.y;
            s.frame = (*RDPtr).ship_data.captain_control.background;
            uqm_lua_checked!(DrawStamp(&raw mut s), ());
            DestroyDrawable(ReleaseDrawable(s.frame));
            (*RDPtr).ship_data.captain_control.background = ::core::ptr::null_mut::<
                FRAME_DESC,
            >();
            SetContext(OldContext);
        } else if GlobData.Game_state.CurrentActivity as BYTE as ::core::ffi::c_int
            <= C2Rust_Unnamed_6::IN_ENCOUNTER.0 as ::core::ffi::c_int
        {
            let mut OldContext_0: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
            uqm_lua_checked!(InitShipStatus(
                &raw mut (*RDPtr).ship_info,
                StarShipPtr,
                ::core::ptr::null_mut::<RECT>(),
                BOOLEAN::FALSE,
            ), ());
            OldContext_0 = SetContext(StatusContext);
            uqm_lua_checked!(DrawCaptainsWindow(StarShipPtr), ());
            if if resolutionFactor != 2 as ::core::ffi::c_uint {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } != 0
            {
                uqm_lua_checked!(DrawHDMeleeBorder(StarShipPtr), ());
            }
            SetContext(OldContext_0);
            if (*RDPtr).preprocess_func.is_some() {
                uqm_lua_checked!(Some((*RDPtr).preprocess_func.expect("non-null function pointer"))
                    .expect("non-null function pointer")(ElementPtr), ());
            }
            if (*ElementPtr).hTarget.is_null() {
                ship_transition(ElementPtr);
            } else {
                (*ElementPtr).hTarget = ::core::ptr::null_mut::<::core::ffi::c_void>();
                if PLRPlaying(
                        ::core::ptr::with_exposed_provenance_mut::<
                            *mut TFB_SoundSample,
                        >(!(0 as ::core::ffi::c_int) as usize),
                    )
                    .0 == 0 && OpponentAlive(StarShipPtr).0 != 0
                {
                    uqm_lua_checked!(BattleSong(BOOLEAN::TRUE), ());
                }
            }
            return;
        } else {
            (*ElementPtr).current.location.x = (((CanvasWidth
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
                >> 1 as ::core::ffi::c_int) as COORD;
            (*ElementPtr).current.location.y = (((CanvasHeight
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
                >> 1 as ::core::ffi::c_int) as COORD;
            (*ElementPtr).next.location = (*ElementPtr).current.location;
            (*ElementPtr).IntersectControl.IntersectStamp.origin.x = ((*ElementPtr)
                .current
                .location
                .x as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as COORD;
            (*ElementPtr).IntersectControl.IntersectStamp.origin.y = ((*ElementPtr)
                .current
                .location
                .y as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as COORD;
            (*ElementPtr).IntersectControl.EndPoint.x = ((*ElementPtr).next.location.x
                as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as COORD;
            (*ElementPtr).IntersectControl.EndPoint.y = ((*ElementPtr).next.location.y
                as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as COORD;
            if uqm_lua_checked!(hyper_transition(ElementPtr), ()).0 != 0 {
                return;
            }
        }
    }
    (*StarShipPtr).cur_status_flags = cur_status_flags;
    if (*StarShipPtr).energy_counter != 0 {
        (*StarShipPtr).energy_counter = (*StarShipPtr).energy_counter.wrapping_sub(1);
    } else if ((*RDPtr).ship_info.energy_level as ::core::ffi::c_int)
        < (*RDPtr).ship_info.max_energy as ::core::ffi::c_int
        || ((*RDPtr).characteristics.energy_regeneration as SBYTE as ::core::ffi::c_int)
            < 0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(DeltaEnergy(
            ElementPtr,
            (*RDPtr).characteristics.energy_regeneration as SBYTE as SIZE,
        ), ());
    }
    if (*RDPtr).preprocess_func.is_some() {
        uqm_lua_checked!(Some((*RDPtr).preprocess_func.expect("non-null function pointer"))
            .expect("non-null function pointer")(ElementPtr), ());
        cur_status_flags = (*StarShipPtr).cur_status_flags;
    }
    if (*ElementPtr).c2rust_unnamed_2.turn_wait != 0 {
        (*ElementPtr).c2rust_unnamed_2.turn_wait = (*ElementPtr)
            .c2rust_unnamed_2
            .turn_wait
            .wrapping_sub(1);
    } else if cur_status_flags as ::core::ffi::c_int
        & ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) != 0
    {
        if cur_status_flags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int != 0
        {
            (*StarShipPtr).ShipFacing = ((*StarShipPtr).ShipFacing as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int
                & ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int) as DWORD as COUNT;
        } else {
            (*StarShipPtr).ShipFacing = ((*StarShipPtr).ShipFacing as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int
                & ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int) as DWORD as COUNT;
        }
        (*ElementPtr).next.image.frame = SetAbsFrameIndex(
            (*ElementPtr).next.image.frame,
            (*StarShipPtr).ShipFacing,
        );
        (*ElementPtr).state_flags = ((*ElementPtr).state_flags as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as ELEMENT_FLAGS;
        (*ElementPtr).c2rust_unnamed_2.turn_wait = (*RDPtr).characteristics.turn_wait;
    }
    if (*ElementPtr).c2rust_unnamed_3.thrust_wait != 0 {
        (*ElementPtr).c2rust_unnamed_3.thrust_wait = (*ElementPtr)
            .c2rust_unnamed_3
            .thrust_wait
            .wrapping_sub(1);
    } else if cur_status_flags as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int != 0
    {
        let mut thrust_status: STATUS_FLAGS = 0;
        thrust_status = inertial_thrust(ElementPtr);
        (*StarShipPtr).cur_status_flags = ((*StarShipPtr).cur_status_flags
            as ::core::ffi::c_int
            & !((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)) as STATUS_FLAGS;
        (*StarShipPtr).cur_status_flags = ((*StarShipPtr).cur_status_flags
            as ::core::ffi::c_int | thrust_status as ::core::ffi::c_int) as STATUS_FLAGS;
        (*ElementPtr).c2rust_unnamed_3.thrust_wait = (*RDPtr)
            .characteristics
            .thrust_wait;
        if !((*GlobData.Game_state.DisplayArray.offset((*ElementPtr).PrimIndex as isize))
            .Type as ::core::ffi::c_int >= gfx_object::NUM_PRIMS.0 as ::core::ffi::c_int
            || (*GlobData
                .Game_state
                .DisplayArray
                .offset((*ElementPtr).PrimIndex as isize))
                .Type as ::core::ffi::c_int
                == gfx_object::STAMPFILL_PRIM.0 as ::core::ffi::c_int
                && sameColor(
                    (*GlobData
                        .Game_state
                        .DisplayArray
                        .offset((*ElementPtr).PrimIndex as isize))
                        .color,
                    colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE),
                ) as ::core::ffi::c_int != 0)
            && GlobData.Game_state.CurrentActivity as BYTE as ::core::ffi::c_int
                <= C2Rust_Unnamed_6::IN_ENCOUNTER.0 as ::core::ffi::c_int
        {
            spawn_ion_trail(ElementPtr, 0 as SIZE, 0 as SIZE);
        }
    }
    if GlobData.Game_state.CurrentActivity as BYTE as ::core::ffi::c_int
        <= C2Rust_Unnamed_6::IN_ENCOUNTER.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(PreProcessStatus(ElementPtr), ());
    }
}
#[no_mangle]
pub unsafe extern "C" fn ship_postprocess(mut ElementPtr: *mut ELEMENT) {
    let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
    let mut RDPtr: *mut RACE_DESC = ::core::ptr::null_mut::<RACE_DESC>();
    if (*ElementPtr).c2rust_unnamed_0.crew_level as ::core::ffi::c_int
        == 0 as ::core::ffi::c_int
    {
        return;
    }
    StarShipPtr = (*ElementPtr).pParent as *mut STARSHIP;
    RDPtr = (*StarShipPtr).RaceDescPtr;
    if (*StarShipPtr).weapon_counter != 0 {
        (*StarShipPtr).weapon_counter = (*StarShipPtr).weapon_counter.wrapping_sub(1);
    } else if (*StarShipPtr).cur_status_flags as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int != 0
        && uqm_lua_checked!(DeltaEnergy(
                ElementPtr,
                -((*RDPtr).characteristics.weapon_energy_cost as ::core::ffi::c_int)
                    as SIZE,
            ), ())
            .0 != 0
    {
        let mut num_weapons: COUNT = 0;
        let mut Weapon: [HELEMENT; 6] = [::core::ptr::null_mut::<
            ::core::ffi::c_void,
        >(); 6];
        num_weapons = uqm_lua_checked!(Some((*RDPtr).init_weapon_func.expect("non-null function pointer"))
            .expect(
                "non-null function pointer",
            )(ElementPtr, &raw mut Weapon as *mut HELEMENT), ());
        if num_weapons != 0 {
            let mut WeaponPtr: *mut HELEMENT = ::core::ptr::null_mut::<HELEMENT>();
            let mut StarShipPtr_0: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
            let mut played_sfx: BOOLEAN = BOOLEAN::FALSE;
            StarShipPtr_0 = (*ElementPtr).pParent as *mut STARSHIP;
            WeaponPtr = (&raw mut Weapon as *mut HELEMENT).offset(0isize);
            loop {
                let mut w: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
                let c2rust_fresh0 = WeaponPtr;
                WeaponPtr = WeaponPtr.offset(1);
                w = *c2rust_fresh0;
                if !w.is_null() {
                    let mut EPtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
                    EPtr = LockLink(&raw mut disp_q, w) as *mut ELEMENT;
                    (*EPtr).pParent = StarShipPtr_0 as *mut ::core::ffi::c_void;
                    if played_sfx.0 == 0 {
                        ProcessSound((*RDPtr).ship_data.ship_sounds, EPtr);
                        played_sfx = BOOLEAN::TRUE;
                    }
                    UnlockLink(&raw mut disp_q, w);
                    PutQueue(&raw mut disp_q, w);
                }
                num_weapons = num_weapons.wrapping_sub(1);
                if num_weapons == 0 {
                    break;
                }
            }
            if played_sfx.0 == 0 {
                ProcessSound((*RDPtr).ship_data.ship_sounds, ElementPtr);
            }
        }
        (*StarShipPtr).weapon_counter = (*RDPtr).characteristics.weapon_wait;
    }
    if (*StarShipPtr).special_counter != 0 {
        (*StarShipPtr).special_counter = (*StarShipPtr).special_counter.wrapping_sub(1);
    }
    if (*RDPtr).postprocess_func.is_some() {
        uqm_lua_checked!(Some((*RDPtr).postprocess_func.expect("non-null function pointer"))
            .expect("non-null function pointer")(ElementPtr), ());
    }
    if GlobData.Game_state.CurrentActivity as BYTE as ::core::ffi::c_int
        <= C2Rust_Unnamed_6::IN_ENCOUNTER.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(PostProcessStatus(ElementPtr), ());
    }
}
#[no_mangle]
pub unsafe extern "C" fn collision(
    mut ElementPtr0: *mut ELEMENT,
    mut pPt0: *mut POINT,
    mut ElementPtr1: *mut ELEMENT,
    mut pPt1: *mut POINT,
) {
    if (*ElementPtr1).state_flags as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int == 0
    {
        (*ElementPtr0).state_flags = ((*ElementPtr0).state_flags as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as ELEMENT_FLAGS;
        if (*ElementPtr1).c2rust_unnamed_1.mass_points as ::core::ffi::c_int
            > 10 as ::core::ffi::c_int * 10 as ::core::ffi::c_int
        {
            let mut damage: SIZE = 0;
            damage = ((*ElementPtr0).c2rust_unnamed_0.hit_points as ::core::ffi::c_int
                >> 2 as ::core::ffi::c_int) as SIZE;
            if damage as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                damage = 1 as SIZE;
            }
            uqm_lua_checked!(do_damage(ElementPtr0, damage), ());
            damage = (C2Rust_Unnamed_7::TARGET_DAMAGED_FOR_1_PT.0 as ::core::ffi::c_int
                + (damage as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)) as SIZE;
            if damage as ::core::ffi::c_int
                > C2Rust_Unnamed_7::TARGET_DAMAGED_FOR_6_PLUS_PT.0 as ::core::ffi::c_int
            {
                damage = C2Rust_Unnamed_7::TARGET_DAMAGED_FOR_6_PLUS_PT.0
                    as ::core::ffi::c_int as SIZE;
            }
            ProcessSound(
                SetAbsStringTableIndex(GameSounds, damage as COUNT),
                ElementPtr0,
            );
        }
    }
}
unsafe extern "C" fn spawn_ship(mut StarShipPtr: *mut STARSHIP) -> BOOLEAN {
    let mut hShip: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut RDPtr: *mut RACE_DESC = ::core::ptr::null_mut::<RACE_DESC>();
    RDPtr = uqm_lua_checked!(load_ship((*StarShipPtr).SpeciesID, BOOLEAN::TRUE), BOOLEAN(0));
    if RDPtr.is_null() {
        return BOOLEAN::FALSE;
    }
    (*StarShipPtr).RaceDescPtr = RDPtr;
    (*StarShipPtr).ship_input_state = 0 as BYTE;
    (*StarShipPtr).cur_status_flags = 0 as STATUS_FLAGS;
    (*StarShipPtr).old_status_flags = 0 as STATUS_FLAGS;
    if GlobData.Game_state.CurrentActivity as BYTE as ::core::ffi::c_int
        == C2Rust_Unnamed_6::IN_ENCOUNTER.0 as ::core::ffi::c_int
        || GlobData.Game_state.CurrentActivity as BYTE as ::core::ffi::c_int
            == C2Rust_Unnamed_6::IN_LAST_BATTLE.0 as ::core::ffi::c_int
    {
        if (*StarShipPtr).crew_level as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            (*RDPtr).ship_info.crew_level = (*StarShipPtr).crew_level;
        }
        if (*RDPtr).ship_info.crew_level as ::core::ffi::c_int
            > (*RDPtr).ship_info.max_crew as ::core::ffi::c_int
        {
            (*RDPtr).ship_info.crew_level = (*RDPtr).ship_info.max_crew;
        }
    }
    (*StarShipPtr).energy_counter = 0 as BYTE;
    (*StarShipPtr).weapon_counter = 0 as BYTE;
    (*StarShipPtr).special_counter = 0 as BYTE;
    hShip = (*StarShipPtr).hShip;
    if hShip.is_null() {
        hShip = AllocElement();
        if !hShip.is_null() {
            InsertQueue(&raw mut disp_q, hShip, disp_q.head);
        }
    }
    (*StarShipPtr).hShip = hShip;
    if !(*StarShipPtr).hShip.is_null() {
        let mut ShipElementPtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
        ShipElementPtr = LockLink(&raw mut disp_q, hShip) as *mut ELEMENT;
        (*ShipElementPtr).playerNr = (*StarShipPtr).playerNr;
        (*ShipElementPtr).c2rust_unnamed_0.crew_level = 0 as COUNT;
        (*ShipElementPtr).c2rust_unnamed_1.mass_points = (*RDPtr)
            .characteristics
            .ship_mass;
        (*ShipElementPtr).state_flags = ((1 as ::core::ffi::c_int)
            << 3 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ELEMENT_FLAGS;
        (*ShipElementPtr).c2rust_unnamed_2.turn_wait = 0 as BYTE;
        (*ShipElementPtr).c2rust_unnamed_3.thrust_wait = 0 as BYTE;
        (*ShipElementPtr).c2rust_unnamed.life_span = 1 as COUNT;
        (*ShipElementPtr).colorCycleIndex = 0 as BYTE;
        DisplayArray[(*ShipElementPtr).PrimIndex as usize].Type = gfx_object::STAMP_PRIM
            .0 as ::core::ffi::c_int as GRAPHICS_PRIM;
        (*ShipElementPtr).current.image.farray = &raw mut (*RDPtr).ship_data.ship
            as *mut FRAME;
        if (*ShipElementPtr).playerNr as ::core::ffi::c_int == 1 as ::core::ffi::c_int
            && GlobData.Game_state.CurrentActivity as BYTE as ::core::ffi::c_int
                == C2Rust_Unnamed_6::IN_LAST_BATTLE.0 as ::core::ffi::c_int
        {
            (*StarShipPtr).ShipFacing = 0 as COUNT;
            (*ShipElementPtr).current.image.frame = SetAbsFrameIndex(
                (*RDPtr).ship_data.ship[0usize],
                (*StarShipPtr).ShipFacing,
            );
            (*ShipElementPtr).current.location.x = (((CanvasWidth
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
                >> 1 as ::core::ffi::c_int) as COORD;
            (*ShipElementPtr).current.location.y = (((CanvasHeight
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
                >> 1 as ::core::ffi::c_int) as COORD;
            (*ShipElementPtr).c2rust_unnamed.life_span = (*ShipElementPtr)
                .c2rust_unnamed
                .life_span
                .wrapping_add(1);
        } else {
            (*StarShipPtr).ShipFacing = (TFB_Random()
                & (((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int) as DWORD) as COUNT;
            if inHQSpace().0 != 0 {
                let mut facing: COUNT = GlobData.Game_state.ShipFacing;
                if facing as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                    facing = facing.wrapping_sub(1);
                }
                (*StarShipPtr).ShipFacing = facing;
                if if resolutionFactor != 2 as ::core::ffi::c_uint {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } != 0
                {
                    let mut i: COUNT = 0;
                    let mut numFrames: COUNT = GetFrameCount(
                        (*RDPtr).ship_data.ship[0usize],
                    );
                    let mut mode: DrawMode = MAKE_DRAW_MODE(
                        DrawKind::DRAW_LINEARBURN,
                        0xff as SWORD,
                    );
                    let mut c: Color = GetColorMapColor(0x35 as COUNT, 0xfb as COUNT);
                    i = 0 as COUNT;
                    while (i as ::core::ffi::c_int) < numFrames as ::core::ffi::c_int {
                        uqm_lua_checked!(ApplyMask(
                            ::core::ptr::null_mut::<FRAME_DESC>(),
                            SetAbsFrameIndex((*RDPtr).ship_data.ship[0usize], i),
                            mode,
                            &raw mut c,
                        ), BOOLEAN(0));
                        i = i.wrapping_add(1);
                    }
                }
            }
            (*ShipElementPtr).current.image.frame = SetAbsFrameIndex(
                (*RDPtr).ship_data.ship[0usize],
                (*StarShipPtr).ShipFacing,
            );
            loop {
                (*ShipElementPtr).current.location.x = (if TFB_Random()
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
                (*ShipElementPtr).current.location.y = (if TFB_Random()
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
                if !(CalculateGravity(ShipElementPtr).0 != 0
                    || uqm_lua_checked!(TimeSpaceMatterConflict(ShipElementPtr), BOOLEAN(0)).0 != 0)
                {
                    break;
                }
            }
        }
        (*ShipElementPtr).preprocess_func = Some(
            ship_preprocess as unsafe extern "C" fn(*mut ELEMENT) -> (),
        ) as Option<ElementProcessFunc>;
        (*ShipElementPtr).postprocess_func = Some(
            ship_postprocess as unsafe extern "C" fn(*mut ELEMENT) -> (),
        ) as Option<ElementProcessFunc>;
        (*ShipElementPtr).death_func = Some(
            ship_death as unsafe extern "C" fn(*mut ELEMENT) -> (),
        ) as Option<ElementProcessFunc>;
        (*ShipElementPtr).collision_func = Some(
            collision
                as unsafe extern "C" fn(
                    *mut ELEMENT,
                    *mut POINT,
                    *mut ELEMENT,
                    *mut POINT,
                ) -> (),
        ) as Option<ElementCollisionFunc>;
        memset(
            &raw mut (*ShipElementPtr).velocity as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<VELOCITY_DESC>(),
        );
        (*ShipElementPtr).pParent = StarShipPtr as *mut ::core::ffi::c_void;
        (*ShipElementPtr).hTarget = ::core::ptr::null_mut::<::core::ffi::c_void>();
        UnlockLink(&raw mut disp_q, hShip);
    }
    return BOOLEAN(!hShip.is_null() as ::core::ffi::c_int as ::core::ffi::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn GetNextStarShip(
    mut LastStarShipPtr: *mut STARSHIP,
    mut which_side: COUNT,
) -> BOOLEAN {
    let mut hBattleShip: HSTARSHIP = ::core::ptr::null_mut::<::core::ffi::c_void>();
    hBattleShip = uqm_lua_checked!(GetEncounterStarShip(LastStarShipPtr, which_side), BOOLEAN(0));
    if !hBattleShip.is_null() {
        let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
        StarShipPtr = LockStarShip(
            (&raw mut race_q as *mut QUEUE).offset(which_side as isize),
            hBattleShip,
        );
        if !LastStarShipPtr.is_null() {
            if StarShipPtr == LastStarShipPtr {
                LastStarShipPtr = ::core::ptr::null_mut::<STARSHIP>();
            } else {
                (*StarShipPtr).hShip = (*LastStarShipPtr).hShip;
            }
        }
        if uqm_lua_checked!(spawn_ship(StarShipPtr), BOOLEAN(0)).0 == 0 {
            UnlockLink(
                (&raw mut race_q as *mut QUEUE).offset(which_side as isize),
                hBattleShip,
            );
            return BOOLEAN::FALSE;
        }
        UnlockLink(
            (&raw mut race_q as *mut QUEUE).offset(which_side as isize),
            hBattleShip,
        );
    }
    if !LastStarShipPtr.is_null() {
        (*LastStarShipPtr).hShip = ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    return BOOLEAN(!hBattleShip.is_null() as ::core::ffi::c_int as ::core::ffi::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn GetInitialStarShips() -> BOOLEAN {
    if GlobData.Game_state.CurrentActivity as BYTE as ::core::ffi::c_int
        == C2Rust_Unnamed_6::SUPER_MELEE.0 as ::core::ffi::c_int
    {
        let mut ships: [HSTARSHIP; 2] = [::core::ptr::null_mut::<
            ::core::ffi::c_void,
        >(); 2];
        let mut i: COUNT = 0;
        if uqm_lua_checked!(GetInitialMeleeStarShips(&raw mut ships as *mut HSTARSHIP), BOOLEAN(0)).0 == 0 {
            return BOOLEAN::FALSE;
        }
        i = 0 as COUNT;
        while (i as ::core::ffi::c_int) < 2 as ::core::ffi::c_int {
            let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
            let mut playerI: COUNT = GetPlayerOrder(i);
            StarShipPtr = LockStarShip(
                (&raw mut race_q as *mut QUEUE).offset(playerI as isize),
                ships[playerI as usize],
            );
            if uqm_lua_checked!(spawn_ship(StarShipPtr), BOOLEAN(0)).0 == 0 {
                UnlockLink(
                    (&raw mut race_q as *mut QUEUE).offset(playerI as isize),
                    ships[playerI as usize],
                );
                return BOOLEAN::FALSE;
            }
            UnlockLink(
                (&raw mut race_q as *mut QUEUE).offset(playerI as isize),
                ships[playerI as usize],
            );
            i = i.wrapping_add(1);
        }
        return BOOLEAN::TRUE;
    } else {
        let mut i_0: ::core::ffi::c_int = 0;
        i_0 = 2 as ::core::ffi::c_int;
        while i_0 > 0 as ::core::ffi::c_int {
            if uqm_lua_checked!(GetNextStarShip(
                    ::core::ptr::null_mut::<STARSHIP>(),
                    (i_0 - 1 as ::core::ffi::c_int) as COUNT,
                ), BOOLEAN(0))
                .0 == 0
            {
                return BOOLEAN::FALSE;
            }
            i_0 -= 1;
        }
        return BOOLEAN::TRUE;
    };
}
