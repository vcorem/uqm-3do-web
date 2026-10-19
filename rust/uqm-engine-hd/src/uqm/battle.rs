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

// Rust translation of src/uqm/battle.c.
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
    ) -> !;
    fn BatchGraphics();
    fn UnbatchGraphics();
    fn memset(
        _: *mut ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn GetTimeCounter() -> TimeCount;
    fn CurrentInputToBattleInput(
        player: COUNT,
        direction: ::core::ffi::c_int,
    ) -> BATTLE_INPUT_STATE;
    static mut WarpFromMenu: BOOLEAN;
    fn DoInput(pInputState: *mut ::core::ffi::c_void, resetInput: BOOLEAN);
    static mut optMeleeScale: ::core::ffi::c_int;
    static mut resolutionFactor: ::core::ffi::c_uint;
    static mut optScrTrans: ::core::ffi::c_int;
    static mut optWindowType: DWORD;
    static mut DirJoyActive: BOOLEAN;
    static mut CanvasWidth: ::core::ffi::c_int;
    static mut CanvasHeight: ::core::ffi::c_int;
    fn InitShips() -> SIZE;
    fn UninitShips();
    static mut PlayerInput: [*mut InputContext; 0];
    fn CountLinks(pq: *mut QUEUE) -> COUNT;
    fn LoadMusicInstance(res: RESOURCE) -> MUSIC_REF;
    fn DestroyMusic(MusicRef: MUSIC_REF) -> BOOLEAN;
    fn StopSound();
    fn SetMusicPosition();
    static mut disp_q: QUEUE;
    static mut DisplayArray: [PRIMITIVE; 330];
    fn MeleeGameOver();
    fn getGameStateUint(name: *const ::core::ffi::c_char) -> DWORD;
    static mut GlobData: GLOBDATA;
    fn inHQSpace() -> BOOLEAN;
    fn inHyperSpace() -> BOOLEAN;
    fn inQuasiSpace() -> BOOLEAN;
    fn TFB_SeedRandom(seed: DWORD) -> DWORD;
    fn GetNextStarShip(LastStarShipPtr: *mut STARSHIP, which_side: COUNT) -> BOOLEAN;
    fn GetInitialStarShips() -> BOOLEAN;
    static mut race_q: [QUEUE; 0];
    static mut PlayerControl: [BYTE; 0];
    fn flee_preprocess(ElementPtr: *mut ELEMENT);
    fn StopDitty();
    fn StopMusic();
    fn PlayMusicResume(hMusic: MUSIC_REF, Volume: BYTE);
    fn RedrawQueue(clear: BOOLEAN);
    fn Async_process();
    fn SetGraphicScaleMode(mode: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn SetTransitionSource(pRect: *const RECT);
    fn ScreenTransition(transition: ::core::ffi::c_int, pRect: *const RECT);
    fn SetMenuSounds(sound_0: MENU_SOUND_FLAGS, sound_1: MENU_SOUND_FLAGS);
    fn EraseRadar();
    fn SleepThreadUntil(wakeTime: TimeCount);
    fn TaskSwitch();
}
pub type size_t = usize;
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
pub type STRING_TABLE_ENTRY_DESC = string_table_entry;
pub type STRING = *mut STRING_TABLE_ENTRY_DESC;
pub type RESOURCE = *const ::core::ffi::c_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Color {
    pub r: BYTE,
    pub g: BYTE,
    pub b: BYTE,
    pub a: BYTE,
}
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
pub type TimeCount = DWORD;
pub type BATTLE_INPUT_STATE = UBYTE;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed {
    pub const OPTVAL_NOM_OFF: Self = Self(0);
    pub const OPTVAL_NOM_EASY: Self = Self(1);
    pub const OPTVAL_NOM_NORM: Self = Self(2);
}
pub type BattleFrameCallback = unsafe extern "C" fn() -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct battlestate_struct {
    pub InputFunc: Option<unsafe extern "C" fn(*mut battlestate_struct) -> BOOLEAN>,
    pub first_time: BOOLEAN,
    pub NextTime: DWORD,
    pub frame_cb: Option<BattleFrameCallback>,
}
pub type BATTLE_STATE = battlestate_struct;
pub type MUSIC_REF = *mut *mut TFB_SoundSample;
pub type TFB_SoundSample = tfb_soundsample;
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
pub struct QUEUE {
    pub head: HLINK,
    pub tail: HLINK,
    pub pq_tab: *mut BYTE,
    pub free_list: HLINK,
    pub object_size: COUNT,
    pub num_objects: BYTE,
}
pub type HLINK = QUEUE_HANDLE;
pub type QUEUE_HANDLE = *mut ::core::ffi::c_void;
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
pub type HSTARSHIP = HLINK;
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
pub type SOUND = STRING;
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
pub type LINK = link;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct link {
    pub pred: HLINK,
    pub succ: HLINK,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct InputContext {
    pub handlers: *mut BattleInputHandlers,
    pub playerNr: COUNT,
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
pub type MENU_SOUND_FLAGS = UWORD;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HumanInputContext {
    pub handlers: *mut BattleInputHandlers,
    pub playerNr: COUNT,
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
    pub const TFB_SCALE_STEP: Self = Self(0);
    pub const TFB_SCALE_NEAREST: Self = Self(1);
    pub const TFB_SCALE_BILINEAR: Self = Self(2);
    pub const TFB_SCALE_TRILINEAR: Self = Self(3);
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
pub static mut battle_counter: [BYTE; 2] = [0; 2];
#[no_mangle]
pub static mut instantVictory: BOOLEAN = BOOLEAN::FALSE;
#[no_mangle]
pub static mut battleInputOrder: [size_t; 2] = [0; 2];
#[no_mangle]
pub unsafe extern "C" fn RunAwayAllowed() -> BOOLEAN {
    return BOOLEAN(
        ((GlobData.Game_state.CurrentActivity as BYTE as ::core::ffi::c_int
            == C2Rust_Unnamed_6::IN_ENCOUNTER.0 as ::core::ffi::c_int
            || GlobData.Game_state.CurrentActivity as BYTE as ::core::ffi::c_int
                == C2Rust_Unnamed_6::IN_LAST_BATTLE.0 as ::core::ffi::c_int)
            && (if GlobData.SIS_state.Nomad as ::core::ffi::c_int
                == C2Rust_Unnamed::OPTVAL_NOM_EASY.0 as ::core::ffi::c_int
            {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
                || uqm_lua_checked!(getGameStateUint(
                    b"STARBASE_AVAILABLE\0".as_ptr() as *const ::core::ffi::c_char,
                ), BOOLEAN(0)) != 0)
            && (if if GlobData.SIS_state.Difficulty as ::core::ffi::c_int != 0 {
                GlobData.SIS_state.Difficulty as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } == 1 as ::core::ffi::c_int
            {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } != 0
                || uqm_lua_checked!(getGameStateUint(
                    b"BOMB_CARRIER\0".as_ptr() as *const ::core::ffi::c_char,
                ), BOOLEAN(0)) == 0)) as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
}
unsafe extern "C" fn DoRunAway(mut StarShipPtr: *mut STARSHIP) {
    let mut ElementPtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
    ElementPtr = LockLink(&raw mut disp_q, (*StarShipPtr).hShip) as *mut ELEMENT;
    if DisplayArray[(*ElementPtr).PrimIndex as usize].Type as ::core::ffi::c_int
        == gfx_object::STAMP_PRIM.0 as ::core::ffi::c_int
        && (*ElementPtr).c2rust_unnamed.life_span as ::core::ffi::c_int
            == 1 as ::core::ffi::c_int
        && (*ElementPtr).state_flags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int == 0
        && (*ElementPtr).c2rust_unnamed_1.mass_points as ::core::ffi::c_int
            != 10 as ::core::ffi::c_int * 10 as ::core::ffi::c_int
        && (*ElementPtr).state_flags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int == 0
    {
        battle_counter[0usize] = battle_counter[0usize].wrapping_sub(1);
        (*ElementPtr).c2rust_unnamed_2.turn_wait = 3 as BYTE;
        (*ElementPtr).c2rust_unnamed_3.thrust_wait = 4 as BYTE;
        (*ElementPtr).colorCycleIndex = 0 as BYTE;
        (*ElementPtr).preprocess_func = Some(
            flee_preprocess as unsafe extern "C" fn(*mut ELEMENT) -> (),
        ) as Option<ElementProcessFunc>;
        (*ElementPtr).c2rust_unnamed_1.mass_points = (10 as ::core::ffi::c_int
            * 10 as ::core::ffi::c_int) as BYTE;
        memset(
            &raw mut (*ElementPtr).velocity as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<VELOCITY_DESC>(),
        );
        (*StarShipPtr).cur_status_flags = ((*StarShipPtr).cur_status_flags
            as ::core::ffi::c_int
            & !((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)) as STATUS_FLAGS;
        DisplayArray[(*ElementPtr).PrimIndex as usize].color = colorFromRgb15(
            0xb as BYTE,
            0 as BYTE,
            0 as BYTE,
        );
        DisplayArray[(*ElementPtr).PrimIndex as usize].Type = gfx_object::STAMPFILL_PRIM
            .0 as ::core::ffi::c_int as GRAPHICS_PRIM;
        (*StarShipPtr).ship_input_state = 0 as BYTE;
    }
    UnlockLink(&raw mut disp_q, (*StarShipPtr).hShip);
}
unsafe extern "C" fn setupBattleInputOrder() {
    let mut i: size_t = 0;
    i = 0 as size_t;
    while i < 2 as size_t {
        battleInputOrder[i] = i;
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn frameInputHuman(
    mut context: *mut HumanInputContext,
    mut StarShipPtr: *mut STARSHIP,
) -> BATTLE_INPUT_STATE {
    return CurrentInputToBattleInput(
        (*context).playerNr,
        if !StarShipPtr.is_null() {
            (*StarShipPtr).ShipFacing as ::core::ffi::c_int
        } else {
            -1 as ::core::ffi::c_int
        },
    );
}
unsafe extern "C" fn ProcessInput() {
    let mut CanRunAway: BOOLEAN = BOOLEAN::FALSE;
    let mut sideI: size_t = 0;
    CanRunAway = uqm_lua_checked!(RunAwayAllowed(), ());
    sideI = 0 as size_t;
    while sideI < 2 as size_t {
        let mut hBattleShip: HSTARSHIP = ::core::ptr::null_mut::<::core::ffi::c_void>();
        let mut hNextShip: HSTARSHIP = ::core::ptr::null_mut::<::core::ffi::c_void>();
        let mut cur_player: size_t = battleInputOrder[sideI];
        hBattleShip = (*(&raw mut race_q as *mut QUEUE).offset(cur_player as isize)).head
            as HSTARSHIP;
        while !hBattleShip.is_null() {
            let mut InputState: BATTLE_INPUT_STATE = 0;
            let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
            StarShipPtr = LockStarShip(
                (&raw mut race_q as *mut QUEUE).offset(cur_player as isize),
                hBattleShip,
            );
            hNextShip = (*StarShipPtr).succ as HSTARSHIP;
            if !(*StarShipPtr).hShip.is_null() {
                (*StarShipPtr).control = *(&raw mut PlayerControl as *mut BYTE)
                    .offset(cur_player as isize);
                InputState = uqm_lua_checked!((*(**(&raw mut PlayerInput as *mut *mut InputContext)
                    .offset(cur_player as isize))
                    .handlers)
                    .frameInput
                    .expect(
                        "non-null function pointer",
                    )(
                    *(&raw mut PlayerInput as *mut *mut InputContext)
                        .offset(cur_player as isize),
                    StarShipPtr,
                ), ());
                (*StarShipPtr).ship_input_state = 0 as BYTE;
                if (*(*StarShipPtr).RaceDescPtr).ship_info.crew_level != 0 {
                    if InputState as ::core::ffi::c_int
                        & ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int)
                            as BATTLE_INPUT_STATE as ::core::ffi::c_int != 0
                    {
                        (*StarShipPtr).ship_input_state = ((*StarShipPtr)
                            .ship_input_state as ::core::ffi::c_int
                            | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int)
                            as BYTE;
                    } else if InputState as ::core::ffi::c_int
                        & ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
                            as BATTLE_INPUT_STATE as ::core::ffi::c_int != 0
                    {
                        (*StarShipPtr).ship_input_state = ((*StarShipPtr)
                            .ship_input_state as ::core::ffi::c_int
                            | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
                            as BYTE;
                    }
                    if InputState as ::core::ffi::c_int
                        & ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                            as BATTLE_INPUT_STATE as ::core::ffi::c_int != 0
                    {
                        (*StarShipPtr).ship_input_state = ((*StarShipPtr)
                            .ship_input_state as ::core::ffi::c_int
                            | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                            as BYTE;
                    }
                    if InputState as ::core::ffi::c_int
                        & ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int)
                            as BATTLE_INPUT_STATE as ::core::ffi::c_int != 0
                    {
                        (*StarShipPtr).ship_input_state = ((*StarShipPtr)
                            .ship_input_state as ::core::ffi::c_int
                            | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                            as BYTE;
                    }
                    if InputState as ::core::ffi::c_int
                        & ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                            as BATTLE_INPUT_STATE as ::core::ffi::c_int != 0
                    {
                        (*StarShipPtr).ship_input_state = ((*StarShipPtr)
                            .ship_input_state as ::core::ffi::c_int
                            | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                            as BYTE;
                    }
                    if InputState as ::core::ffi::c_int
                        & ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                            as BATTLE_INPUT_STATE as ::core::ffi::c_int != 0
                    {
                        (*StarShipPtr).ship_input_state = ((*StarShipPtr)
                            .ship_input_state as ::core::ffi::c_int
                            | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                            as BYTE;
                    }
                    if CanRunAway.0 != 0 && cur_player == 0 as size_t
                        && (InputState as ::core::ffi::c_int
                            & ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int)
                                as BATTLE_INPUT_STATE as ::core::ffi::c_int != 0
                            || WarpFromMenu.0 != 0)
                    {
                        WarpFromMenu = BOOLEAN::FALSE;
                        DoRunAway(StarShipPtr);
                    }
                }
            }
            UnlockLink(
                (&raw mut race_q as *mut QUEUE).offset(cur_player as isize),
                hBattleShip,
            );
            hBattleShip = hNextShip;
        }
        sideI = sideI.wrapping_add(1);
    }
    if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
        & (C2Rust_Unnamed_6::CHECK_LOAD.0 as ::core::ffi::c_int
            | C2Rust_Unnamed_6::CHECK_ABORT.0 as ::core::ffi::c_int) != 0
    {
        GlobData.Game_state.CurrentActivity = (GlobData.Game_state.CurrentActivity
            as ::core::ffi::c_int
            & !(C2Rust_Unnamed_6::IN_BATTLE.0 as ::core::ffi::c_int)) as ACTIVITY;
    }
}
static mut BattleRef: MUSIC_REF = ::core::ptr::null_mut::<*mut TFB_SoundSample>();
#[no_mangle]
pub static mut inHSpace: BYTE = 0 as BYTE;
#[no_mangle]
pub unsafe extern "C" fn BattleSong(mut DoPlay: BOOLEAN) {
    if BattleRef.is_null() {
        if uqm_lua_checked!(inHyperSpace(), ()).0 != 0 {
            BattleRef = uqm_lua_checked!(LoadMusicInstance(b"music.hyperspace\0".as_ptr() as RESOURCE), ());
            inHSpace = 1 as BYTE;
        } else if uqm_lua_checked!(inQuasiSpace(), ()).0 != 0 {
            BattleRef = uqm_lua_checked!(LoadMusicInstance(b"music.quasispace\0".as_ptr() as RESOURCE), ());
            inHSpace = 2 as BYTE;
        } else {
            if GlobData.Game_state.CurrentActivity as BYTE as ::core::ffi::c_int
                == C2Rust_Unnamed_6::IN_LAST_BATTLE.0 as ::core::ffi::c_int
            {
                BattleRef = uqm_lua_checked!(LoadMusicInstance(
                    b"music.battle.samatra\0".as_ptr() as RESOURCE,
                ), ());
                if BattleRef.is_null() {
                    BattleRef = uqm_lua_checked!(LoadMusicInstance(
                        b"music.battle\0".as_ptr() as RESOURCE,
                    ), ());
                }
            } else {
                BattleRef = uqm_lua_checked!(LoadMusicInstance(b"music.battle\0".as_ptr() as RESOURCE), ());
            }
            inHSpace = 0 as BYTE;
        }
    }
    if DoPlay.0 != 0 {
        uqm_lua_checked!(PlayMusicResume(BattleRef, 160 as BYTE), ());
    }
}
#[no_mangle]
pub unsafe extern "C" fn FreeBattleSong() {
    uqm_lua_checked!(DestroyMusic(BattleRef), ());
    BattleRef = ::core::ptr::null_mut::<*mut TFB_SoundSample>();
}
unsafe extern "C" fn DoBattle(mut bs: *mut BATTLE_STATE) -> BOOLEAN {
    extern "C" {
        static mut nth_frame: UWORD;
    }
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut battle_speed: BYTE = 0;
    SetMenuSounds(
        0 as ::core::ffi::c_int as MENU_SOUND_FLAGS,
        0 as ::core::ffi::c_int as MENU_SOUND_FLAGS,
    );
    uqm_lua_checked!(ProcessInput(), BOOLEAN(0));
    if (*bs).first_time.0 != 0 {
        r.corner.x = (((6 as ::core::ffi::c_int) << resolutionFactor)
            + if if optWindowType == 1 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                (if if optWindowType == 1 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    16 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                } << resolutionFactor) + ((1 as ::core::ffi::c_int) << resolutionFactor)
            } else {
                0 as ::core::ffi::c_int
            }) as COORD;
        r.corner.y = (((9 as ::core::ffi::c_int) << resolutionFactor)
            + if if optWindowType == 1 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                (if if optWindowType == 1 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    16 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                } << resolutionFactor) + ((1 as ::core::ffi::c_int) << resolutionFactor)
            } else {
                0 as ::core::ffi::c_int
            }) as COORD;
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
            } << resolutionFactor) * 2 as ::core::ffi::c_int
            - (((13 as ::core::ffi::c_int) << resolutionFactor)
                + (if if optWindowType == 1 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    1 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                } << resolutionFactor))) as COORD;
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
            } << resolutionFactor) * 2 as ::core::ffi::c_int
            - ((13 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        SetTransitionSource(&raw mut r);
    }
    BatchGraphics();
    if (*bs).frame_cb.is_some() {
        uqm_lua_checked!((*bs).frame_cb.expect("non-null function pointer")(), BOOLEAN(0));
    }
    uqm_lua_checked!(RedrawQueue(BOOLEAN::TRUE), BOOLEAN(0));
    if (*bs).first_time.0 != 0 {
        (*bs).first_time = BOOLEAN::FALSE;
        uqm_lua_checked!(ScreenTransition(optScrTrans, &raw mut r), BOOLEAN(0));
    }
    UnbatchGraphics();
    if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
        & C2Rust_Unnamed_6::IN_BATTLE.0 as ::core::ffi::c_int == 0
        || GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
            & (C2Rust_Unnamed_6::CHECK_ABORT.0 as ::core::ffi::c_int
                | C2Rust_Unnamed_6::CHECK_LOAD.0 as ::core::ffi::c_int) != 0
    {
        return BOOLEAN::FALSE;
    }
    battle_speed = (nth_frame as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as BYTE;
    if battle_speed as ::core::ffi::c_int
        == !(0 as ::core::ffi::c_int) as BYTE as ::core::ffi::c_int
    {
        uqm_lua_checked!(Async_process(), BOOLEAN(0));
        TaskSwitch();
    } else {
        uqm_lua_checked!(SleepThreadUntil(
            (*bs)
                .NextTime
                .wrapping_add(
                    (840 as ::core::ffi::c_int / 24 as ::core::ffi::c_int
                        / (battle_speed as ::core::ffi::c_int + 1 as ::core::ffi::c_int))
                        as TimeCount,
                ),
        ), BOOLEAN(0));
        (*bs).NextTime = GetTimeCounter() as DWORD;
    }
    if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
        & C2Rust_Unnamed_6::IN_BATTLE.0 as ::core::ffi::c_int == 0 as ::core::ffi::c_int
    {
        return BOOLEAN::FALSE;
    }
    return BOOLEAN::TRUE;
}
unsafe extern "C" fn selectAllShips(mut num_ships: SIZE) -> BOOLEAN {
    if num_ships as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
        return uqm_lua_checked!(GetNextStarShip(::core::ptr::null_mut::<STARSHIP>(), 0 as COUNT), BOOLEAN(0));
    }
    return uqm_lua_checked!(GetInitialStarShips(), BOOLEAN(0));
}
#[no_mangle]
pub unsafe extern "C" fn Battle(mut callback: Option<BattleFrameCallback>) -> BOOLEAN {
    let mut num_ships: SIZE = 0;
    if GlobData.Game_state.CurrentActivity as BYTE as ::core::ffi::c_int
        != C2Rust_Unnamed_6::SUPER_MELEE.0 as ::core::ffi::c_int
    {
        TFB_SeedRandom(GetTimeCounter());
    }
    uqm_lua_checked!(BattleSong(BOOLEAN::FALSE), BOOLEAN(0));
    num_ships = uqm_lua_checked!(InitShips(), BOOLEAN(0));
    if instantVictory.0 != 0 {
        num_ships = 0 as SIZE;
        battle_counter[0usize] = 1 as BYTE;
        battle_counter[1usize] = 0 as BYTE;
        instantVictory = BOOLEAN::FALSE;
    }
    if num_ships != 0 {
        let mut bs: BATTLE_STATE = BATTLE_STATE {
            InputFunc: None,
            first_time: BOOLEAN::FALSE,
            NextTime: 0,
            frame_cb: None,
        };
        GlobData.Game_state.CurrentActivity = (GlobData.Game_state.CurrentActivity
            as ::core::ffi::c_int | C2Rust_Unnamed_6::IN_BATTLE.0 as ::core::ffi::c_int)
            as ACTIVITY;
        battle_counter[0usize] = CountLinks(
            (&raw mut race_q as *mut QUEUE).offset(0isize),
        ) as BYTE;
        battle_counter[1usize] = CountLinks(
            (&raw mut race_q as *mut QUEUE).offset(1isize),
        ) as BYTE;
        if optMeleeScale != C2Rust_Unnamed_7::TFB_SCALE_STEP.0 as ::core::ffi::c_int {
            SetGraphicScaleMode(optMeleeScale);
        }
        setupBattleInputOrder();
        if uqm_lua_checked!(selectAllShips(num_ships), BOOLEAN(0)).0 == 0 {
            GlobData.Game_state.CurrentActivity = (GlobData.Game_state.CurrentActivity
                as ::core::ffi::c_int
                | C2Rust_Unnamed_6::CHECK_ABORT.0 as ::core::ffi::c_int) as ACTIVITY;
        } else {
            uqm_lua_checked!(BattleSong(BOOLEAN::TRUE), BOOLEAN(0));
            DirJoyActive = BOOLEAN::TRUE;
            bs.NextTime = 0 as DWORD;
            bs.InputFunc = Some(
                DoBattle as unsafe extern "C" fn(*mut BATTLE_STATE) -> BOOLEAN,
            ) as Option<unsafe extern "C" fn(*mut battlestate_struct) -> BOOLEAN>;
            bs.frame_cb = callback;
            bs.first_time = inHQSpace();
            if bs.first_time.0 != 0 {
                uqm_lua_checked!(EraseRadar(), BOOLEAN(0));
            }
            uqm_lua_checked!(DoInput(&raw mut bs as *mut ::core::ffi::c_void, BOOLEAN::FALSE), BOOLEAN(0));
        }
        if GlobData.Game_state.CurrentActivity as BYTE as ::core::ffi::c_int
            == C2Rust_Unnamed_6::SUPER_MELEE.0 as ::core::ffi::c_int
        {
            if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
                & C2Rust_Unnamed_6::CHECK_ABORT.0 as ::core::ffi::c_int != 0
            {
                GlobData.Game_state.CurrentActivity = (GlobData
                    .Game_state
                    .CurrentActivity as ::core::ffi::c_int
                    & !(C2Rust_Unnamed_6::CHECK_ABORT.0 as ::core::ffi::c_int))
                    as ACTIVITY;
            } else {
                uqm_lua_checked!(MeleeGameOver(), BOOLEAN(0));
            }
        }
        DirJoyActive = BOOLEAN::FALSE;
        uqm_lua_checked!(SetMusicPosition(), BOOLEAN(0));
        uqm_lua_checked!(StopDitty(), BOOLEAN(0));
        uqm_lua_checked!(StopMusic(), BOOLEAN(0));
        uqm_lua_checked!(StopSound(), BOOLEAN(0));
    }
    uqm_lua_checked!(UninitShips(), BOOLEAN(0));
    uqm_lua_checked!(FreeBattleSong(), BOOLEAN(0));
    return BOOLEAN(
        ((num_ships as ::core::ffi::c_int) < 0 as ::core::ffi::c_int)
            as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
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
                b"src/uqm/supermelee/../races.h\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                304 as ::core::ffi::c_int,
                b"LockStarShip\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    return LockLink(pq, h) as *mut STARSHIP;
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
                    b"src/uqm/supermelee/../displist.h\0"
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
                    b"src/uqm/supermelee/../displist.h\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    88 as ::core::ffi::c_int,
                    b"UnlockLink\0".as_ptr() as *const ::core::ffi::c_char,
                );
                0 as ::core::ffi::c_int != 0
            }) as ::core::ffi::c_int;
    }
}
