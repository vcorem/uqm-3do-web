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

// Rust translation of src/uqm/tactrans.c.
extern "C" {
    pub type string_table_entry;
    pub type frame_desc;
    pub type tfb_soundsample;
    pub type getmelee_struct;
    pub type MeleeTeam;
    pub type MeleeSetup;
    pub type RandomContext;
    pub type NetConnection;
    fn BatchGraphics();
    fn UnbatchGraphics();
    fn GetFrameRect(Frame: FRAME, pRect: *mut RECT) -> BOOLEAN;
    fn SetAbsFrameIndex(Frame: FRAME, FrameIndex: COUNT) -> FRAME;
    fn SetEquFrameIndex(DstFrame: FRAME, SrcFrame: FRAME) -> FRAME;
    fn DecFrameIndex(Frame: FRAME) -> FRAME;
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
    static mut optExtended: OPT_ENABLABLE;
    static mut optWindowType: DWORD;
    static mut CanvasWidth: ::core::ffi::c_int;
    static mut CanvasHeight: ::core::ffi::c_int;
    static mut sinetab: [SDWORD; 0];
    fn PutQueue(pq: *mut QUEUE, hLink: HLINK);
    fn InsertQueue(pq: *mut QUEUE, hLink: HLINK, hRefLink: HLINK);
    fn PLRPlaying(MusicRef: MUSIC_REF) -> BOOLEAN;
    fn StopSound();
    fn SetMusicPosition();
    static mut disp_q: QUEUE;
    static mut DisplayArray: [PRIMITIVE; 330];
    fn AllocElement() -> HELEMENT;
    fn crew_preprocess(ElementPtr: *mut ELEMENT);
    fn SetVelocityComponents(velocityptr: *mut VELOCITY_DESC, dx: SDWORD, dy: SDWORD);
    static mut PlayerInput: [*mut InputContext; 0];
    fn MeleeShipDeath(_: *mut STARSHIP);
    static mut stars_in_space: FRAME;
    static mut stars_misc: [FRAME; 3];
    static mut GlobData: GLOBDATA;
    fn TFB_Random() -> DWORD;
    fn NetConnection_isConnected(conn: *const NetConnection) -> bool;
    fn NetConnection_getStateData(
        conn: *const NetConnection,
    ) -> *mut NetConnectionStateData;
    fn NetConnection_setState(conn: *mut NetConnection, state: NetState);
    fn NetConnection_getState(conn: *const NetConnection) -> NetState;
    static mut battle_counter: [BYTE; 2];
    static mut battleFrameCount: BattleFrameCounter;
    fn BattleSong(DoPlay: BOOLEAN);
    fn free_ship(
        RaceDescPtr: *mut RACE_DESC,
        FreeIconData: BOOLEAN,
        FreeBattleData: BOOLEAN,
    );
    fn GetNextStarShip(LastStarShipPtr: *mut STARSHIP, which_side: COUNT) -> BOOLEAN;
    fn animation_preprocess(ElementPtr: *mut ELEMENT);
    fn ship_preprocess(ElementPtr: *mut ELEMENT);
    fn ship_postprocess(ElementPtr: *mut ELEMENT);
    static mut netConnections: [*mut NetConnection; 0];
    fn initBattleStateDataConnections();
    fn negotiateReady(
        conn: *mut NetConnection,
        notifyRemote: bool,
        nextState: NetState,
    ) -> bool;
    fn negotiateReadyConnections(notifyRemote: bool, nextState: NetState) -> bool;
    fn waitReady(conn: *mut NetConnection) -> bool;
    fn Netplay_Notify_frameCount(
        conn: *mut NetConnection,
        frameCount: BattleFrameCounter,
    );
    fn DeltaEnergy(ElementPtr: *mut ELEMENT, energy_delta: SIZE) -> BOOLEAN;
    fn PreProcessStatus(ShipPtr: *mut ELEMENT);
    fn PostProcessStatus(ShipPtr: *mut ELEMENT);
    fn Netplay_localReady(
        conn: *mut NetConnection,
        callback: NetConnection_ReadyCallback,
        arg: *mut ::core::ffi::c_void,
        notifyRemote: bool,
    ) -> bool;
    fn Netplay_isLocalReady(conn: *const NetConnection) -> bool;
    fn flushPacketQueue(conn: *mut NetConnection) -> ::core::ffi::c_int;
    fn FleetIsInfinite(playerNr: COUNT) -> BOOLEAN;
    fn UpdateShipFragCrew(_: *mut STARSHIP);
    fn StopMusic();
    fn PlayMusic(MusicRef: MUSIC_REF, Continuous: BOOLEAN, Priority: BYTE);
    static mut GameSounds: SOUND;
    fn PlaySound(
        S: SOUND,
        Pos: SoundPosition,
        PositionalObject: *mut ELEMENT,
        Priority: BYTE,
    );
    fn ProcessSound(Sound: SOUND, PositionalObject: *mut ELEMENT);
    fn CalcSoundPosition(ElementPtr: *mut ELEMENT) -> SoundPosition;
    static mut PlayerControl: [BYTE; 0];
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
pub type DIRENTRY = STRING;
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
pub struct OPT_ENABLABLE(pub ::core::ffi::c_uint);
impl OPT_ENABLABLE {
    pub const OPTVAL_DISABLED: Self = Self(0);
    pub const OPTVAL_ENABLED: Self = Self(1);
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct soundposition {
    pub positional: BOOLEAN,
    pub x: ::core::ffi::c_int,
    pub y: ::core::ffi::c_int,
}
pub type SoundPosition = soundposition;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ComputerInputContext {
    pub handlers: *mut BattleInputHandlers,
    pub playerNr: COUNT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HumanInputContext {
    pub handlers: *mut BattleInputHandlers,
    pub playerNr: COUNT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NetworkInputContext {
    pub handlers: *mut BattleInputHandlers,
    pub playerNr: COUNT,
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
pub struct NetState(pub ::core::ffi::c_uint);
impl NetState {
    pub const NetState_unconnected: Self = Self(0);
    pub const NetState_connecting: Self = Self(1);
    pub const NetState_init: Self = Self(2);
    pub const NetState_inSetup: Self = Self(3);
    pub const NetState_preBattle: Self = Self(4);
    pub const NetState_interBattle: Self = Self(5);
    pub const NetState_selectShip: Self = Self(6);
    pub const NetState_inBattle: Self = Self(7);
    pub const NetState_endingBattle: Self = Self(8);
    pub const NetState_endingBattle2: Self = Self(9);
}
pub type NetConnection_ReadyCallback = Option<
    unsafe extern "C" fn(*mut NetConnection, *mut ::core::ffi::c_void) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NetConnectionStateData {
    pub releaseFunction: NetConnectionStateData_ReleaseFunction,
}
pub type NetConnectionStateData_ReleaseFunction = Option<
    unsafe extern "C" fn(*mut NetConnectionStateData) -> (),
>;
pub type BattleFrameCounter = DWORD;
pub type BattleFrameCallback = unsafe extern "C" fn() -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct battlestate_struct {
    pub InputFunc: Option<unsafe extern "C" fn(*mut battlestate_struct) -> BOOLEAN>,
    pub first_time: BOOLEAN,
    pub NextTime: DWORD,
    pub frame_cb: Option<BattleFrameCallback>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct melee_state {
    pub InputFunc: Option<unsafe extern "C" fn(*mut melee_state) -> BOOLEAN>,
    pub Initialized: BOOLEAN,
    pub meleeStarted: BOOLEAN,
    pub MeleeOption: MELEE_OPTIONS,
    pub side: COUNT,
    pub row: COUNT,
    pub col: COUNT,
    pub meleeSetup: *mut MeleeSetup,
    pub load: melee_load_state,
    pub currentShip: MeleeShip,
    pub CurIndex: COUNT,
    pub buildPickConfirmed: BOOLEAN,
    pub randomContext: *mut RandomContext,
    pub LastInputTime: TimeCount,
    pub hMusic: MUSIC_REF,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct MeleeShip(pub ::core::ffi::c_uint);
impl MeleeShip {
    pub const MELEE_ANDROSYNTH: Self = Self(0);
    pub const MELEE_ARILOU: Self = Self(1);
    pub const MELEE_CHENJESU: Self = Self(2);
    pub const MELEE_CHMMR: Self = Self(3);
    pub const MELEE_DRUUGE: Self = Self(4);
    pub const MELEE_EARTHLING: Self = Self(5);
    pub const MELEE_ILWRATH: Self = Self(6);
    pub const MELEE_KOHR_AH: Self = Self(7);
    pub const MELEE_MELNORME: Self = Self(8);
    pub const MELEE_MMRNMHRM: Self = Self(9);
    pub const MELEE_MYCON: Self = Self(10);
    pub const MELEE_ORZ: Self = Self(11);
    pub const MELEE_PKUNK: Self = Self(12);
    pub const MELEE_SHOFIXTI: Self = Self(13);
    pub const MELEE_SLYLANDRO: Self = Self(14);
    pub const MELEE_SPATHI: Self = Self(15);
    pub const MELEE_SUPOX: Self = Self(16);
    pub const MELEE_SYREEN: Self = Self(17);
    pub const MELEE_THRADDASH: Self = Self(18);
    pub const MELEE_UMGAH: Self = Self(19);
    pub const MELEE_URQUAN: Self = Self(20);
    pub const MELEE_UTWIG: Self = Self(21);
    pub const MELEE_VUX: Self = Self(22);
    pub const MELEE_YEHAT: Self = Self(23);
    pub const MELEE_ZOQFOTPIK: Self = Self(24);
    pub const MELEE_UNSET: Self = Self(254);
    pub const MELEE_NONE: Self = Self(255);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct melee_load_state {
    pub preBuiltList: *mut *mut MeleeTeam,
    pub preBuiltCount: COUNT,
    pub dirEntries: DIRENTRY,
    pub entryIndices: *mut COUNT,
    pub numIndices: COUNT,
    pub view: [*mut MeleeTeam; 5],
    pub top: COUNT,
    pub bot: COUNT,
    pub cur: COUNT,
    pub viewSize: COUNT,
}
pub type MELEE_OPTIONS = BYTE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BattleStateData {
    pub releaseFunction: NetConnectionStateData_ReleaseFunction,
    pub meleeState: *mut melee_state,
    pub battleState: *mut battlestate_struct,
    pub getMeleeState: *mut getmelee_struct,
    pub endFrameCount: BattleFrameCounter,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_6(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_6 {
    pub const GRAB_CREW: Self = Self(0);
    pub const SHIP_EXPLODES: Self = Self(1);
    pub const TARGET_DAMAGED_FOR_1_PT: Self = Self(2);
    pub const TARGET_DAMAGED_FOR_2_3_PT: Self = Self(3);
    pub const TARGET_DAMAGED_FOR_4_5_PT: Self = Self(4);
    pub const TARGET_DAMAGED_FOR_6_PLUS_PT: Self = Self(5);
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
static mut dittyIsPlaying: BOOLEAN = BOOLEAN::FALSE;
static mut winnerStarShip: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
#[no_mangle]
pub unsafe extern "C" fn OpponentAlive(mut TestStarShipPtr: *mut STARSHIP) -> BOOLEAN {
    let mut hElement: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut hSuccElement: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
    hElement = disp_q.head as HELEMENT;
    while !hElement.is_null() {
        let mut ElementPtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
        let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
        ElementPtr = LockLink(&raw mut disp_q, hElement) as *mut ELEMENT;
        hSuccElement = (*ElementPtr).succ;
        StarShipPtr = (*ElementPtr).pParent as *mut STARSHIP;
        UnlockLink(&raw mut disp_q, hElement);
        if !StarShipPtr.is_null() && StarShipPtr != TestStarShipPtr
            && (*(*StarShipPtr).RaceDescPtr).ship_info.crew_level as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
        {
            return BOOLEAN::FALSE;
        }
        hElement = hSuccElement;
    }
    return BOOLEAN::TRUE;
}
unsafe extern "C" fn PlayDitty(mut ship: *mut STARSHIP) {
    uqm_lua_checked!(PlayMusic((*(*ship).RaceDescPtr).ship_data.victory_ditty, BOOLEAN::FALSE, 3 as BYTE), ());
    dittyIsPlaying = BOOLEAN::TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn StopDitty() {
    if dittyIsPlaying.0 != 0 {
        uqm_lua_checked!(StopMusic(), ());
    }
    dittyIsPlaying = BOOLEAN::FALSE;
}
unsafe extern "C" fn DittyPlaying() -> BOOLEAN {
    if dittyIsPlaying.0 == 0 {
        return BOOLEAN::FALSE;
    }
    dittyIsPlaying = PLRPlaying(
        ::core::ptr::with_exposed_provenance_mut::<
            *mut TFB_SoundSample,
        >(!(0 as ::core::ffi::c_int) as usize),
    );
    return dittyIsPlaying;
}
#[no_mangle]
pub unsafe extern "C" fn ResetWinnerStarShip() {
    winnerStarShip = ::core::ptr::null_mut::<STARSHIP>();
}
unsafe extern "C" fn readyToEnd2Callback(
    mut conn: *mut NetConnection,
    mut arg: *mut ::core::ffi::c_void,
) {
    NetConnection_setState(conn, NetState::NetState_endingBattle2);
}
unsafe extern "C" fn readyToEndCallback(
    mut conn: *mut NetConnection,
    mut arg: *mut ::core::ffi::c_void,
) {
    let mut battleStateData: *mut BattleStateData = ::core::ptr::null_mut::<
        BattleStateData,
    >();
    battleStateData = NetConnection_getStateData(conn) as *mut BattleStateData;
    NetConnection_setState(conn, NetState::NetState_endingBattle);
    if battleFrameCount.wrapping_add(1 as BattleFrameCounter)
        > (*battleStateData).endFrameCount
    {
        (*battleStateData).endFrameCount = battleFrameCount
            .wrapping_add(1 as BattleFrameCounter);
    }
    Netplay_Notify_frameCount(
        conn,
        battleFrameCount.wrapping_add(1 as BattleFrameCounter),
    );
    flushPacketQueue(conn);
    uqm_lua_checked!(Netplay_localReady(
        conn,
        Some(
            readyToEnd2Callback
                as unsafe extern "C" fn(
                    *mut NetConnection,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
        false,
    ), ());
}
unsafe extern "C" fn readyForBattleEndPlayer(mut conn: *mut NetConnection) -> bool {
    let mut battleStateData: *mut BattleStateData = ::core::ptr::null_mut::<
        BattleStateData,
    >();
    if conn.is_null() || !NetConnection_isConnected(conn) {
        GlobData.Game_state.CurrentActivity = (GlobData.Game_state.CurrentActivity
            as ::core::ffi::c_int
            | C2Rust_Unnamed_5::CHECK_ABORT.0 as ::core::ffi::c_int) as ACTIVITY;
        return false;
    }
    battleStateData = NetConnection_getStateData(conn) as *mut BattleStateData;
    if NetConnection_getState(conn).0 == NetState::NetState_interBattle.0
        || NetConnection_getState(conn).0 == NetState::NetState_inSetup.0
    {
        return true;
    }
    if NetConnection_getState(conn).0 == NetState::NetState_inBattle.0 {
        if Netplay_isLocalReady(conn) {
            return false;
        }
        uqm_lua_checked!(Netplay_localReady(
            conn,
            Some(
                readyToEndCallback
                    as unsafe extern "C" fn(
                        *mut NetConnection,
                        *mut ::core::ffi::c_void,
                    ) -> (),
            ),
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            true,
        ), false);
        if NetConnection_getState(conn).0 == NetState::NetState_inBattle.0 {
            return false;
        }
    }
    (NetConnection_getState(conn).0 == NetState::NetState_endingBattle.0
        || NetConnection_getState(conn).0 == NetState::NetState_endingBattle2.0
        || {
            __assert_fail(
                b"NetConnection_getState (conn) == NetState_endingBattle || NetConnection_getState (conn) == NetState_endingBattle2\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"src/uqm/tactrans.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                207 as ::core::ffi::c_int,
                b"readyForBattleEndPlayer\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    if battleFrameCount < (*battleStateData).endFrameCount {
        return false;
    }
    if NetConnection_getState(conn).0 == NetState::NetState_endingBattle.0 {
        if !uqm_lua_checked!(waitReady(conn), false) {
            GlobData.Game_state.CurrentActivity = (GlobData.Game_state.CurrentActivity
                as ::core::ffi::c_int
                | C2Rust_Unnamed_5::CHECK_ABORT.0 as ::core::ffi::c_int) as ACTIVITY;
            return false;
        }
        (NetConnection_getState(conn).0 == NetState::NetState_endingBattle2.0
            || {
                __assert_fail(
                    b"NetConnection_getState (conn) == NetState_endingBattle2\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"src/uqm/tactrans.c\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    226 as ::core::ffi::c_int,
                    b"readyForBattleEndPlayer\0".as_ptr() as *const ::core::ffi::c_char,
                );
                0 as ::core::ffi::c_int != 0
            }) as ::core::ffi::c_int;
        if battleFrameCount < (*battleStateData).endFrameCount {
            return false;
        }
    }
    if !uqm_lua_checked!(negotiateReady(conn, true, NetState::NetState_interBattle), false) {
        GlobData.Game_state.CurrentActivity = (GlobData.Game_state.CurrentActivity
            as ::core::ffi::c_int
            | C2Rust_Unnamed_5::CHECK_ABORT.0 as ::core::ffi::c_int) as ACTIVITY;
        return false;
    }
    return true;
}
#[no_mangle]
pub unsafe extern "C" fn battleEndReadyHuman(
    mut context: *mut HumanInputContext,
) -> bool {
    return true;
}
#[no_mangle]
pub unsafe extern "C" fn battleEndReadyComputer(
    mut context: *mut ComputerInputContext,
) -> bool {
    return true;
}
#[no_mangle]
pub unsafe extern "C" fn battleEndReadyNetwork(
    mut context: *mut NetworkInputContext,
) -> bool {
    return uqm_lua_checked!(readyForBattleEndPlayer(
        *(&raw mut netConnections as *mut *mut NetConnection)
            .offset((*context).playerNr as isize),
    ), false);
}
#[inline]
unsafe extern "C" fn readyForBattleEnd() -> bool {
    let mut playerI: ::core::ffi::c_int = 0;
    if DittyPlaying().0 != 0 {
        return false;
    }
    playerI = 0 as ::core::ffi::c_int;
    while playerI < 2 as ::core::ffi::c_int {
        if !uqm_lua_checked!((*(**(&raw mut PlayerInput as *mut *mut InputContext)
            .offset(playerI as isize))
            .handlers)
            .battleEndReady
            .expect(
                "non-null function pointer",
            )(*(&raw mut PlayerInput as *mut *mut InputContext).offset(playerI as isize)), false)
        {
            return false;
        }
        playerI += 1;
    }
    return true;
}
unsafe extern "C" fn preprocess_dead_ship(mut DeadShipPtr: *mut ELEMENT) {
    ProcessSound(
        ::core::ptr::with_exposed_provenance_mut::<
            ::core::ffi::c_void,
        >(!(0 as ::core::ffi::c_int) as usize) as SOUND,
        ::core::ptr::null_mut::<ELEMENT>(),
    );
}
unsafe extern "C" fn cleanup_dead_ship(mut DeadShipPtr: *mut ELEMENT) {
    let mut DeadStarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
    ProcessSound(
        ::core::ptr::with_exposed_provenance_mut::<
            ::core::ffi::c_void,
        >(!(0 as ::core::ffi::c_int) as usize) as SOUND,
        ::core::ptr::null_mut::<ELEMENT>(),
    );
    DeadStarShipPtr = (*DeadShipPtr).pParent as *mut STARSHIP;
    let mut MusicStarted: BOOLEAN = BOOLEAN::FALSE;
    let mut hElement: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut hSuccElement: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
    (*DeadStarShipPtr).crew_level = (*(*DeadStarShipPtr).RaceDescPtr)
        .ship_info
        .crew_level;
    MusicStarted = BOOLEAN::FALSE;
    hElement = disp_q.head as HELEMENT;
    while !hElement.is_null() {
        let mut ElementPtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
        let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
        ElementPtr = LockLink(&raw mut disp_q, hElement) as *mut ELEMENT;
        hSuccElement = (*ElementPtr).succ;
        StarShipPtr = (*ElementPtr).pParent as *mut STARSHIP;
        if StarShipPtr == DeadStarShipPtr {
            (*ElementPtr).pParent = ::core::ptr::null_mut::<::core::ffi::c_void>();
            if (*ElementPtr).state_flags as ::core::ffi::c_int
                & (1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int == 0
                || (*ElementPtr).preprocess_func
                    != Some(crew_preprocess as unsafe extern "C" fn(*mut ELEMENT) -> ())
            {
                DisplayArray[(*ElementPtr).PrimIndex as usize].Type = gfx_object::NUM_PRIMS
                    .0 as ::core::ffi::c_int as GRAPHICS_PRIM;
                (*ElementPtr).c2rust_unnamed.life_span = 0 as COUNT;
                (*ElementPtr).state_flags = ((1 as ::core::ffi::c_int)
                    << 6 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int)
                    as ELEMENT_FLAGS;
                (*ElementPtr).preprocess_func = None;
                (*ElementPtr).postprocess_func = None;
                (*ElementPtr).death_func = None;
                (*ElementPtr).collision_func = None;
            }
        }
        if !StarShipPtr.is_null()
            && (*StarShipPtr).cur_status_flags as ::core::ffi::c_int
                & (1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int != 0
        {
            MusicStarted = BOOLEAN::TRUE;
            uqm_lua_checked!(PlayDitty(StarShipPtr), ());
            (*StarShipPtr).cur_status_flags = ((*StarShipPtr).cur_status_flags
                as ::core::ffi::c_int
                & !((1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int))
                as STATUS_FLAGS;
        }
        UnlockLink(&raw mut disp_q, hElement);
        hElement = hSuccElement;
    }
    (*DeadShipPtr).c2rust_unnamed.life_span = (if MusicStarted.0 != 0 {
        840 as ::core::ffi::c_int * 3 as ::core::ffi::c_int
            / (840 as ::core::ffi::c_int / 24 as ::core::ffi::c_int)
    } else {
        1 as ::core::ffi::c_int
    }) as COUNT;
    if DeadStarShipPtr == winnerStarShip {
        (*DeadShipPtr).c2rust_unnamed.life_span = (840 as ::core::ffi::c_int
            * 3 as ::core::ffi::c_int
            / (840 as ::core::ffi::c_int / 24 as ::core::ffi::c_int)
            + 1 as ::core::ffi::c_int) as COUNT;
    }
    (*DeadShipPtr).death_func = Some(
        new_ship as unsafe extern "C" fn(*mut ELEMENT) -> (),
    ) as Option<ElementProcessFunc>;
    (*DeadShipPtr).preprocess_func = Some(
        preprocess_dead_ship as unsafe extern "C" fn(*mut ELEMENT) -> (),
    ) as Option<ElementProcessFunc>;
    (*DeadShipPtr).state_flags = ((*DeadShipPtr).state_flags as ::core::ffi::c_int
        & !((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)) as ELEMENT_FLAGS;
    (*DeadShipPtr).c2rust_unnamed.life_span = (*DeadShipPtr)
        .c2rust_unnamed
        .life_span
        .wrapping_add(1);
    (*DeadShipPtr).pParent = DeadStarShipPtr as *mut ::core::ffi::c_void;
}
unsafe extern "C" fn setMinShipLifeSpan(mut ship: *mut ELEMENT, mut life_span: COUNT) {
    if (*ship).death_func == Some(new_ship as unsafe extern "C" fn(*mut ELEMENT) -> ()) {
        ((*ship).state_flags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int != 0
            || {
                __assert_fail(
                    b"ship->state_flags & FINITE_LIFE\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"src/uqm/tactrans.c\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    395 as ::core::ffi::c_int,
                    b"setMinShipLifeSpan\0".as_ptr() as *const ::core::ffi::c_char,
                );
                0 as ::core::ffi::c_int != 0
            }) as ::core::ffi::c_int;
        ((*ship).state_flags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int == 0
            || {
                __assert_fail(
                    b"!(ship->state_flags & DISAPPEARING)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"src/uqm/tactrans.c\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    396 as ::core::ffi::c_int,
                    b"setMinShipLifeSpan\0".as_ptr() as *const ::core::ffi::c_char,
                );
                0 as ::core::ffi::c_int != 0
            }) as ::core::ffi::c_int;
        if ((*ship).c2rust_unnamed.life_span as ::core::ffi::c_int)
            < life_span as ::core::ffi::c_int
        {
            (*ship).c2rust_unnamed.life_span = life_span;
        }
    }
}
unsafe extern "C" fn setMinStarShipLifeSpan(
    mut starShip: *mut STARSHIP,
    mut life_span: COUNT,
) {
    let mut ship: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
    ship = LockLink(&raw mut disp_q, (*starShip).hShip) as *mut ELEMENT;
    setMinShipLifeSpan(ship, life_span);
    UnlockLink(&raw mut disp_q, (*starShip).hShip);
}
unsafe extern "C" fn checkOtherShipLifeSpan(mut deadShip: *mut ELEMENT) {
    let mut deadStarShip: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
    deadStarShip = (*deadShip).pParent as *mut STARSHIP;
    if !winnerStarShip.is_null() && deadStarShip != winnerStarShip
        && (*(*winnerStarShip).RaceDescPtr).ship_info.crew_level as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
    {
        setMinStarShipLifeSpan(
            winnerStarShip,
            ((*deadShip).c2rust_unnamed.life_span as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int) as COUNT,
        );
    } else if winnerStarShip.is_null() {
        let mut hElement: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
        let mut hNextElement: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
        hElement = disp_q.head as HELEMENT;
        while !hElement.is_null() {
            let mut element: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
            let mut starShip: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
            element = LockLink(&raw mut disp_q, hElement) as *mut ELEMENT;
            hNextElement = (*element).succ;
            starShip = (*element).pParent as *mut STARSHIP;
            if !starShip.is_null() && element != deadShip
                && (*(*starShip).RaceDescPtr).ship_info.crew_level as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
            {
                setMinShipLifeSpan(element, (*deadShip).c2rust_unnamed.life_span);
            }
            UnlockLink(&raw mut disp_q, hElement);
            hElement = hNextElement;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn new_ship(mut DeadShipPtr: *mut ELEMENT) {
    let mut DeadStarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
    DeadStarShipPtr = (*DeadShipPtr).pParent as *mut STARSHIP;
    if !uqm_lua_checked!(readyForBattleEnd(), ()) {
        (*DeadShipPtr).state_flags = ((*DeadShipPtr).state_flags as ::core::ffi::c_int
            & !((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)) as ELEMENT_FLAGS;
        (*DeadShipPtr).c2rust_unnamed.life_span = (*DeadShipPtr)
            .c2rust_unnamed
            .life_span
            .wrapping_add(1);
        checkOtherShipLifeSpan(DeadShipPtr);
        return;
    }
    winnerStarShip = ::core::ptr::null_mut::<STARSHIP>();
    let mut RestartMusic: BOOLEAN = BOOLEAN::FALSE;
    uqm_lua_checked!(StopDitty(), ());
    uqm_lua_checked!(StopMusic(), ());
    uqm_lua_checked!(StopSound(), ());
    (*DeadShipPtr).pParent = ::core::ptr::null_mut::<::core::ffi::c_void>();
    RestartMusic = OpponentAlive(DeadStarShipPtr);
    uqm_lua_checked!(free_ship((*DeadStarShipPtr).RaceDescPtr, BOOLEAN::TRUE, BOOLEAN::TRUE), ());
    (*DeadStarShipPtr).RaceDescPtr = ::core::ptr::null_mut::<RACE_DESC>();
    UnbatchGraphics();
    initBattleStateDataConnections();
    let mut allOk: bool = uqm_lua_checked!(negotiateReadyConnections(
        true,
        NetState::NetState_interBattle,
    ), ());
    if !allOk {
        GlobData.Game_state.CurrentActivity = (GlobData.Game_state.CurrentActivity
            as ::core::ffi::c_int
            & !(C2Rust_Unnamed_5::IN_BATTLE.0 as ::core::ffi::c_int)) as ACTIVITY;
        BatchGraphics();
        return;
    }
    if FleetIsInfinite((*DeadStarShipPtr).playerNr as COUNT).0 == 0 {
        UpdateShipFragCrew(DeadStarShipPtr);
        (*DeadStarShipPtr).SpeciesID = SPECIES_ID::NO_ID;
    }
    if uqm_lua_checked!(GetNextStarShip(DeadStarShipPtr, (*DeadStarShipPtr).playerNr as COUNT), ()).0 != 0 {
        let mut allOk_0: bool = uqm_lua_checked!(negotiateReadyConnections(
            true,
            NetState::NetState_inBattle,
        ), ());
        if !allOk_0 {
            GlobData.Game_state.CurrentActivity = (GlobData.Game_state.CurrentActivity
                as ::core::ffi::c_int
                & !(C2Rust_Unnamed_5::IN_BATTLE.0 as ::core::ffi::c_int)) as ACTIVITY;
            BatchGraphics();
            return;
        }
        if RestartMusic.0 != 0 {
            uqm_lua_checked!(BattleSong(BOOLEAN::TRUE), ());
        }
    } else if battle_counter[0usize] as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        || battle_counter[1usize] as ::core::ffi::c_int == 0 as ::core::ffi::c_int
    {
        GlobData.Game_state.CurrentActivity = (GlobData.Game_state.CurrentActivity
            as ::core::ffi::c_int
            & !(C2Rust_Unnamed_5::IN_BATTLE.0 as ::core::ffi::c_int)) as ACTIVITY;
    } else {
        GlobData.Game_state.CurrentActivity = (GlobData.Game_state.CurrentActivity
            as ::core::ffi::c_int
            | C2Rust_Unnamed_5::CHECK_ABORT.0 as ::core::ffi::c_int) as ACTIVITY;
    }
    BatchGraphics();
}
unsafe extern "C" fn explosion_preprocess(mut ShipPtr: *mut ELEMENT) {
    let mut i: BYTE = 0;
    i = (12 as ::core::ffi::c_int * 3 as ::core::ffi::c_int
        - (*ShipPtr).c2rust_unnamed.life_span as ::core::ffi::c_int) as BYTE;
    's_71: {
        'c_27971: {
            match i as ::core::ffi::c_int {
                25 => {
                    (*ShipPtr).preprocess_func = None;
                }
                0 | 1 | 2 | 20 | 21 | 22 | 23 | 24 => {}
                3 | 4 | 5 | 18 | 19 => {
                    i = 2 as BYTE;
                    break 's_71;
                }
                15 => {
                    DisplayArray[(*ShipPtr).PrimIndex as usize].Type = gfx_object::NUM_PRIMS
                        .0 as ::core::ffi::c_int as GRAPHICS_PRIM;
                    (*ShipPtr).state_flags = ((*ShipPtr).state_flags
                        as ::core::ffi::c_int
                        | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int)
                        as ELEMENT_FLAGS;
                    break 'c_27971;
                }
                _ => {
                    break 'c_27971;
                }
            }
            i = 1 as BYTE;
            break 's_71;
        }
        i = 3 as BYTE;
    }
    loop {
        let mut hElement: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
        hElement = AllocElement();
        if !hElement.is_null() {
            let mut angle: COUNT = 0;
            let mut dist: COUNT = 0;
            let mut rand_val: DWORD = 0;
            let mut ElementPtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
            extern "C" {
                static mut explosion: [FRAME; 0];
            }
            PutQueue(&raw mut disp_q, hElement);
            ElementPtr = LockLink(&raw mut disp_q, hElement) as *mut ELEMENT;
            (*ElementPtr).playerNr = -1 as SIZE;
            (*ElementPtr).state_flags = ((1 as ::core::ffi::c_int)
                << 3 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as ELEMENT_FLAGS;
            (*ElementPtr).c2rust_unnamed.life_span = 9 as COUNT;
            DisplayArray[(*ElementPtr).PrimIndex as usize].Type = gfx_object::STAMP_PRIM
                .0 as ::core::ffi::c_int as GRAPHICS_PRIM;
            (*ElementPtr).current.image.farray = &raw mut explosion as *mut FRAME;
            (*ElementPtr).current.image.frame = *(&raw mut explosion as *mut FRAME)
                .offset(0isize);
            rand_val = TFB_Random();
            angle = (rand_val >> 16 as ::core::ffi::c_int) as UWORD as BYTE as COUNT;
            dist = ((rand_val as UWORD as BYTE as ::core::ffi::c_int
                % 8 as ::core::ffi::c_int) << resolutionFactor
                << 2 as ::core::ffi::c_int) as COUNT;
            if ((rand_val as UWORD as ::core::ffi::c_int >> 8 as ::core::ffi::c_int)
                as BYTE as ::core::ffi::c_int)
                < 256 as ::core::ffi::c_int * 1 as ::core::ffi::c_int
                    / 3 as ::core::ffi::c_int
            {
                dist = (dist as ::core::ffi::c_int
                    + ((8 as ::core::ffi::c_int) << resolutionFactor
                        << 2 as ::core::ffi::c_int)) as COUNT;
            }
            (*ElementPtr).current.location.x = ((*ShipPtr).current.location.x as SDWORD
                + (*(&raw mut sinetab as *mut SDWORD)
                    .offset(
                        (angle as ::core::ffi::c_int
                            + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                                >> 2 as ::core::ffi::c_int)
                            & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                                - 1 as ::core::ffi::c_int) as DWORD as isize,
                    ) as ::core::ffi::c_long * dist as ::core::ffi::c_long
                    >> 14 as ::core::ffi::c_int) as SDWORD) as COORD;
            (*ElementPtr).current.location.y = ((*ShipPtr).current.location.y as SDWORD
                + (*(&raw mut sinetab as *mut SDWORD)
                    .offset(
                        (angle as ::core::ffi::c_int
                            & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                                - 1 as ::core::ffi::c_int) as DWORD as isize,
                    ) as ::core::ffi::c_long * dist as ::core::ffi::c_long
                    >> 14 as ::core::ffi::c_int) as SDWORD) as COORD;
            (*ElementPtr).preprocess_func = Some(
                animation_preprocess as unsafe extern "C" fn(*mut ELEMENT) -> (),
            ) as Option<ElementProcessFunc>;
            rand_val = TFB_Random();
            angle = rand_val as UWORD as BYTE as COUNT;
            dist = ((((rand_val as UWORD as ::core::ffi::c_int
                >> 8 as ::core::ffi::c_int) as BYTE as ::core::ffi::c_int
                % 5 as ::core::ffi::c_int) << resolutionFactor
                << 2 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as COUNT;
            SetVelocityComponents(
                &raw mut (*ElementPtr).velocity,
                (*(&raw mut sinetab as *mut SDWORD)
                    .offset(
                        (angle as ::core::ffi::c_int
                            + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                                >> 2 as ::core::ffi::c_int)
                            & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                                - 1 as ::core::ffi::c_int) as DWORD as isize,
                    ) as ::core::ffi::c_long * dist as ::core::ffi::c_long
                    >> 14 as ::core::ffi::c_int) as SDWORD,
                (*(&raw mut sinetab as *mut SDWORD)
                    .offset(
                        (angle as ::core::ffi::c_int
                            & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                                - 1 as ::core::ffi::c_int) as DWORD as isize,
                    ) as ::core::ffi::c_long * dist as ::core::ffi::c_long
                    >> 14 as ::core::ffi::c_int) as SDWORD,
            );
            UnlockLink(&raw mut disp_q, hElement);
        }
        i = i.wrapping_sub(1);
        if i == 0 {
            break;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn StopAllBattleMusic() {
    uqm_lua_checked!(StopDitty(), ());
    uqm_lua_checked!(StopMusic(), ());
}
#[no_mangle]
pub unsafe extern "C" fn FindAliveStarShip(mut deadShip: *mut ELEMENT) -> *mut STARSHIP {
    let mut aliveShip: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
    let mut hElement: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut hNextElement: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
    hElement = disp_q.head as HELEMENT;
    while !hElement.is_null() {
        let mut ElementPtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
        ElementPtr = LockLink(&raw mut disp_q, hElement) as *mut ELEMENT;
        if (*ElementPtr).state_flags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int != 0
            && ElementPtr != deadShip
            && (*ElementPtr).c2rust_unnamed_1.mass_points as ::core::ffi::c_int
                <= 10 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
        {
            aliveShip = (*ElementPtr).pParent as *mut STARSHIP;
            (!aliveShip.is_null()
                || {
                    __assert_fail(
                        b"aliveShip != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                        b"src/uqm/tactrans.c\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        658 as ::core::ffi::c_int,
                        b"FindAliveStarShip\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    0 as ::core::ffi::c_int != 0
                }) as ::core::ffi::c_int;
            if (*(*aliveShip).RaceDescPtr).ship_info.crew_level as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
                && (*ElementPtr).c2rust_unnamed_1.mass_points as ::core::ffi::c_int
                    != 10 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
            {
                aliveShip = ::core::ptr::null_mut::<STARSHIP>();
            }
            UnlockLink(&raw mut disp_q, hElement);
            break;
        } else {
            hNextElement = (*ElementPtr).succ;
            UnlockLink(&raw mut disp_q, hElement);
            hElement = hNextElement;
        }
    }
    return aliveShip;
}
#[no_mangle]
pub unsafe extern "C" fn GetWinnerStarShip() -> *mut STARSHIP {
    return winnerStarShip;
}
#[no_mangle]
pub unsafe extern "C" fn SetWinnerStarShip(mut winner: *mut STARSHIP) {
    if winner.is_null() {
        return;
    }
    (*winner).cur_status_flags = ((*winner).cur_status_flags as ::core::ffi::c_int
        | (1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int) as STATUS_FLAGS;
    if winnerStarShip.is_null() {
        winnerStarShip = winner;
    }
}
#[no_mangle]
pub unsafe extern "C" fn RecordShipDeath(mut deadShip: *mut ELEMENT) {
    let mut deadStarShip: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
    deadStarShip = (*deadShip).pParent as *mut STARSHIP;
    (!deadStarShip.is_null()
        || {
            __assert_fail(
                b"deadStarShip != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/uqm/tactrans.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                703 as ::core::ffi::c_int,
                b"RecordShipDeath\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    if (*deadShip).c2rust_unnamed_1.mass_points as ::core::ffi::c_int
        <= 10 as ::core::ffi::c_int
    {
        ((*deadStarShip).playerNr as ::core::ffi::c_int >= 0 as ::core::ffi::c_int
            || {
                __assert_fail(
                    b"deadStarShip->playerNr >= 0\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"src/uqm/tactrans.c\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    709 as ::core::ffi::c_int,
                    b"RecordShipDeath\0".as_ptr() as *const ::core::ffi::c_char,
                );
                0 as ::core::ffi::c_int != 0
            }) as ::core::ffi::c_int;
        battle_counter[(*deadStarShip).playerNr as usize] = battle_counter[(*deadStarShip)
                .playerNr as usize]
            .wrapping_sub(1);
    }
    if GlobData.Game_state.CurrentActivity as BYTE as ::core::ffi::c_int
        == C2Rust_Unnamed_5::SUPER_MELEE.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(MeleeShipDeath(deadStarShip), ());
    }
}
#[no_mangle]
pub unsafe extern "C" fn StartShipExplosion(
    mut ShipPtr: *mut ELEMENT,
    mut playSound: bool,
) {
    let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
    StarShipPtr = (*ShipPtr).pParent as *mut STARSHIP;
    if optExtended.0 == 0 || isNetwork().0 != 0 {
        memset(
            &raw mut (*ShipPtr).velocity as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<VELOCITY_DESC>(),
        );
    }
    uqm_lua_checked!(DeltaEnergy(
        ShipPtr,
        -((*(*StarShipPtr).RaceDescPtr).ship_info.energy_level as SIZE
            as ::core::ffi::c_int) as SIZE,
    ), ());
    (*ShipPtr).c2rust_unnamed.life_span = (12 as ::core::ffi::c_int
        * 3 as ::core::ffi::c_int) as COUNT;
    (*ShipPtr).state_flags = ((*ShipPtr).state_flags as ::core::ffi::c_int
        & !((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)) as ELEMENT_FLAGS;
    (*ShipPtr).state_flags = ((*ShipPtr).state_flags as ::core::ffi::c_int
        | ((1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)) as ELEMENT_FLAGS;
    (*ShipPtr).preprocess_func = Some(
        explosion_preprocess as unsafe extern "C" fn(*mut ELEMENT) -> (),
    ) as Option<ElementProcessFunc>;
    (*ShipPtr).postprocess_func = Some(
        PostProcessStatus as unsafe extern "C" fn(*mut ELEMENT) -> (),
    ) as Option<ElementProcessFunc>;
    (*ShipPtr).death_func = Some(
        cleanup_dead_ship as unsafe extern "C" fn(*mut ELEMENT) -> (),
    ) as Option<ElementProcessFunc>;
    (*ShipPtr).hTarget = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if playSound {
        uqm_lua_checked!(PlaySound(
            SetAbsStringTableIndex(
                GameSounds,
                C2Rust_Unnamed_6::SHIP_EXPLODES.0 as ::core::ffi::c_int as COUNT,
            ),
            CalcSoundPosition(ShipPtr),
            ShipPtr,
            (2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as BYTE,
        ), ());
    }
}
#[no_mangle]
pub unsafe extern "C" fn ship_death(mut ShipPtr: *mut ELEMENT) {
    let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
    let mut winner: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
    StarShipPtr = (*ShipPtr).pParent as *mut STARSHIP;
    uqm_lua_checked!(SetMusicPosition(), ());
    uqm_lua_checked!(StopAllBattleMusic(), ());
    (*StarShipPtr).cur_status_flags = ((*StarShipPtr).cur_status_flags
        as ::core::ffi::c_int & !((1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int))
        as STATUS_FLAGS;
    uqm_lua_checked!(StartShipExplosion(ShipPtr, true), ());
    winner = FindAliveStarShip(ShipPtr);
    SetWinnerStarShip(winner);
    uqm_lua_checked!(RecordShipDeath(ShipPtr), ());
}
unsafe extern "C" fn cycle_ion_trail(mut ElementPtr: *mut ELEMENT) {
    static mut colorTab: [Color; 12] = [
        Color {
            r: ((0x1f as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x1f as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0x15 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x15 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
        Color {
            r: ((0x1f as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x1f as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0x11 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x11 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
        Color {
            r: ((0x1f as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x1f as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0xe as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0xe as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
        Color {
            r: ((0x1f as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x1f as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0xa as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0xa as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
        Color {
            r: ((0x1f as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x1f as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0x7 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x7 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
        Color {
            r: ((0x1f as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x1f as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0x3 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x3 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
        Color {
            r: ((0x1f as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x1f as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
        Color {
            r: ((0x1b as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x1b as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
        Color {
            r: ((0x17 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x17 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
        Color {
            r: ((0x13 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x13 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
        Color {
            r: ((0xf as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0xf as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
        Color {
            r: ((0xb as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0xb as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
    ];
    let colorTabCount: size_t = ::core::mem::size_of::<[Color; 12]>()
        .wrapping_div(::core::mem::size_of::<Color>());
    ((*ElementPtr).state_flags as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int == 0
        || {
            __assert_fail(
                b"!(ElementPtr->state_flags & PLAYER_SHIP)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/uqm/tactrans.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                792 as ::core::ffi::c_int,
                b"cycle_ion_trail\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    (*ElementPtr).colorCycleIndex = (*ElementPtr).colorCycleIndex.wrapping_add(1);
    if (*ElementPtr).colorCycleIndex as size_t != colorTabCount {
        (*ElementPtr).c2rust_unnamed.life_span = (*ElementPtr)
            .c2rust_unnamed_3
            .thrust_wait as COUNT;
        DisplayArray[(*ElementPtr).PrimIndex as usize].color = colorTab[(*ElementPtr)
            .colorCycleIndex as usize];
        (*ElementPtr).state_flags = ((*ElementPtr).state_flags as ::core::ffi::c_int
            & !((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)) as ELEMENT_FLAGS;
        (*ElementPtr).state_flags = ((*ElementPtr).state_flags as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as ELEMENT_FLAGS;
    }
}
#[no_mangle]
pub unsafe extern "C" fn spawn_ion_trail(
    mut ElementPtr: *mut ELEMENT,
    mut x_offset: SIZE,
    mut y_offset: SIZE,
) {
    let mut hIonElement: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
    ((*ElementPtr).state_flags as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int != 0
        || {
            __assert_fail(
                b"ElementPtr->state_flags & PLAYER_SHIP\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/uqm/tactrans.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                813 as ::core::ffi::c_int,
                b"spawn_ion_trail\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    hIonElement = AllocElement();
    if !hIonElement.is_null() {
        let mut angle: COUNT = 0;
        let mut r: RECT = RECT {
            corner: POINT { x: 0, y: 0 },
            extent: EXTENT { width: 0, height: 0 },
        };
        let mut IonElementPtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
        let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
        StarShipPtr = (*ElementPtr).pParent as *mut STARSHIP;
        angle = ((((*StarShipPtr).ShipFacing as ::core::ffi::c_int)
            << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int)
            + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int)) as COUNT;
        GetFrameRect((*(*StarShipPtr).RaceDescPtr).ship_data.ship[0usize], &raw mut r);
        r.extent.height = ((r.extent.height as ::core::ffi::c_int
            + r.corner.y as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as COORD;
        InsertQueue(&raw mut disp_q, hIonElement, disp_q.head);
        IonElementPtr = LockLink(&raw mut disp_q, hIonElement) as *mut ELEMENT;
        (*IonElementPtr).playerNr = -1 as SIZE;
        (*IonElementPtr).state_flags = ((1 as ::core::ffi::c_int)
            << 3 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as ELEMENT_FLAGS;
        (*IonElementPtr).c2rust_unnamed_3.thrust_wait = 1 as BYTE;
        (*IonElementPtr).c2rust_unnamed.life_span = (*IonElementPtr)
            .c2rust_unnamed_3
            .thrust_wait as COUNT;
        if if resolutionFactor != 2 as ::core::ffi::c_uint {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } == 0
        {
            DisplayArray[(*IonElementPtr).PrimIndex as usize].Type = gfx_object::POINT_PRIM
                .0 as ::core::ffi::c_int as GRAPHICS_PRIM;
            (*IonElementPtr).current.image.frame = DecFrameIndex(stars_in_space);
            (*IonElementPtr).current.image.farray = &raw mut stars_in_space;
        } else {
            DisplayArray[(*IonElementPtr).PrimIndex as usize].Type = gfx_object::STAMPFILL_PRIM
                .0 as ::core::ffi::c_int as GRAPHICS_PRIM;
            (*IonElementPtr).current.image.frame = SetAbsFrameIndex(
                stars_misc[0usize],
                1 as COUNT,
            );
            (*IonElementPtr).current.image.farray = &raw mut stars_misc as *mut FRAME;
            (*IonElementPtr).next.image.frame = (*IonElementPtr).current.image.frame;
            (*IonElementPtr).next.image.farray = (*IonElementPtr).current.image.farray;
        }
        DisplayArray[(*IonElementPtr).PrimIndex as usize].color = colorFromRgb15(
            0x1f as BYTE,
            0x15 as BYTE,
            0 as BYTE,
        );
        (*IonElementPtr).colorCycleIndex = 0 as BYTE;
        (*IonElementPtr).current.location = (*ElementPtr).current.location;
        (*IonElementPtr).current.location.x = ((*IonElementPtr).current.location.x
            as ::core::ffi::c_int
            + ((*(&raw mut sinetab as *mut SDWORD)
                .offset(
                    (angle as ::core::ffi::c_int
                        + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                            >> 2 as ::core::ffi::c_int)
                        & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                            - 1 as ::core::ffi::c_int) as DWORD as isize,
                ) as ::core::ffi::c_long * r.extent.height as ::core::ffi::c_long
                >> 14 as ::core::ffi::c_int) as SDWORD as COORD as ::core::ffi::c_int
                + x_offset as ::core::ffi::c_int)) as COORD;
        (*IonElementPtr).current.location.y = ((*IonElementPtr).current.location.y
            as ::core::ffi::c_int
            + ((*(&raw mut sinetab as *mut SDWORD)
                .offset(
                    (angle as ::core::ffi::c_int
                        & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                            - 1 as ::core::ffi::c_int) as DWORD as isize,
                ) as ::core::ffi::c_long * r.extent.height as ::core::ffi::c_long
                >> 14 as ::core::ffi::c_int) as SDWORD as COORD as ::core::ffi::c_int
                + y_offset as ::core::ffi::c_int)) as COORD;
        (*IonElementPtr).death_func = Some(
            cycle_ion_trail as unsafe extern "C" fn(*mut ELEMENT) -> (),
        ) as Option<ElementProcessFunc>;
        (*IonElementPtr).pParent = StarShipPtr as *mut ::core::ffi::c_void;
        (*IonElementPtr).next = (*IonElementPtr).current;
        (*IonElementPtr).c2rust_unnamed.life_span = (*IonElementPtr)
            .c2rust_unnamed
            .life_span
            .wrapping_sub(1);
        (*IonElementPtr).state_flags = ((*IonElementPtr).state_flags
            as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 11 as ::core::ffi::c_int) as ELEMENT_FLAGS;
        UnlockLink(&raw mut disp_q, hIonElement);
    }
}
#[no_mangle]
pub unsafe extern "C" fn ship_transition(mut ElementPtr: *mut ELEMENT) {
    if (*ElementPtr).state_flags as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int != 0
    {
        if (*ElementPtr).state_flags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int != 0
        {
            (*ElementPtr).c2rust_unnamed.life_span = 15 as COUNT;
            (*ElementPtr).preprocess_func = Some(
                ship_transition as unsafe extern "C" fn(*mut ELEMENT) -> (),
            ) as Option<ElementProcessFunc>;
            (*ElementPtr).postprocess_func = None;
            DisplayArray[(*ElementPtr).PrimIndex as usize].Type = gfx_object::NUM_PRIMS.0
                as ::core::ffi::c_int as GRAPHICS_PRIM;
            (*ElementPtr).state_flags = ((*ElementPtr).state_flags as ::core::ffi::c_int
                | ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int))
                as ELEMENT_FLAGS;
        } else if ((*ElementPtr).c2rust_unnamed.life_span as ::core::ffi::c_int)
            < 15 as ::core::ffi::c_int
        {
            if (*ElementPtr).c2rust_unnamed.life_span as ::core::ffi::c_int
                == 1 as ::core::ffi::c_int
                && (*ElementPtr).c2rust_unnamed_0.crew_level as ::core::ffi::c_int != 0
            {
                (*ElementPtr).next.image.frame = SetEquFrameIndex(
                    *(*ElementPtr).current.image.farray.offset(0isize),
                    (*ElementPtr).current.image.frame,
                );
                (*ElementPtr).current.image.frame = (*ElementPtr).next.image.frame;
                DisplayArray[(*ElementPtr).PrimIndex as usize].Type = gfx_object::STAMP_PRIM
                    .0 as ::core::ffi::c_int as GRAPHICS_PRIM;
                (*ElementPtr).IntersectControl.IntersectStamp.origin.x = ((*ElementPtr)
                    .current
                    .location
                    .x as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as COORD;
                (*ElementPtr).IntersectControl.IntersectStamp.origin.y = ((*ElementPtr)
                    .current
                    .location
                    .y as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as COORD;
                (*ElementPtr).IntersectControl.EndPoint.x = ((*ElementPtr)
                    .next
                    .location
                    .x as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as COORD;
                (*ElementPtr).IntersectControl.EndPoint.y = ((*ElementPtr)
                    .next
                    .location
                    .y as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as COORD;
                (*ElementPtr).IntersectControl.IntersectStamp.frame = SetEquFrameIndex(
                    *(*ElementPtr).next.image.farray.offset(0isize),
                    (*ElementPtr).next.image.frame,
                );
                memset(
                    &raw mut (*ElementPtr).velocity as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    ::core::mem::size_of::<VELOCITY_DESC>(),
                );
                (*ElementPtr).state_flags = ((*ElementPtr).state_flags
                    as ::core::ffi::c_int
                    & !((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                        | (1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int))
                    as ELEMENT_FLAGS;
                (*ElementPtr).state_flags = ((*ElementPtr).state_flags
                    as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int)
                    as ELEMENT_FLAGS;
                (*ElementPtr).preprocess_func = Some(
                    ship_preprocess as unsafe extern "C" fn(*mut ELEMENT) -> (),
                ) as Option<ElementProcessFunc>;
                (*ElementPtr).postprocess_func = Some(
                    ship_postprocess as unsafe extern "C" fn(*mut ELEMENT) -> (),
                ) as Option<ElementProcessFunc>;
            }
            return;
        }
    }
    let mut hShipImage: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut ShipImagePtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
    let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
    StarShipPtr = (*ElementPtr).pParent as *mut STARSHIP;
    ShipImagePtr = LockLink(&raw mut disp_q, (*StarShipPtr).hShip) as *mut ELEMENT;
    if (*ShipImagePtr).state_flags as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int == 0
    {
        (*ElementPtr).preprocess_func = None;
    } else {
        hShipImage = AllocElement();
        if !hShipImage.is_null() {
            let mut angle: COUNT = 0;
            PutQueue(&raw mut disp_q, hShipImage);
            angle = (((*StarShipPtr).ShipFacing as ::core::ffi::c_int)
                << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int) as COUNT;
            ShipImagePtr = LockLink(&raw mut disp_q, hShipImage) as *mut ELEMENT;
            (*ShipImagePtr).playerNr = -1 as SIZE;
            (*ShipImagePtr).state_flags = ((1 as ::core::ffi::c_int)
                << 3 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as ELEMENT_FLAGS;
            (*ShipImagePtr).c2rust_unnamed_3.thrust_wait = 1 as BYTE;
            (*ShipImagePtr).c2rust_unnamed.life_span = (*ShipImagePtr)
                .c2rust_unnamed_3
                .thrust_wait as COUNT;
            DisplayArray[(*ShipImagePtr).PrimIndex as usize].Type = gfx_object::STAMPFILL_PRIM
                .0 as ::core::ffi::c_int as GRAPHICS_PRIM;
            DisplayArray[(*ShipImagePtr).PrimIndex as usize].color = colorFromRgb15(
                0x1f as BYTE,
                0x15 as BYTE,
                0 as BYTE,
            );
            (*ShipImagePtr).colorCycleIndex = 0 as BYTE;
            (*ShipImagePtr).current.image = (*ElementPtr).current.image;
            (*ShipImagePtr).current.location = (*ElementPtr).current.location;
            if (*ElementPtr).state_flags as ::core::ffi::c_int
                & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int == 0
            {
                (*ShipImagePtr).current.location.x = ((*ShipImagePtr).current.location.x
                    as ::core::ffi::c_int
                    + (*(&raw mut sinetab as *mut SDWORD)
                        .offset(
                            (angle as ::core::ffi::c_int
                                + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                                    >> 2 as ::core::ffi::c_int)
                                & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                                    - 1 as ::core::ffi::c_int) as DWORD as isize,
                        ) as ::core::ffi::c_long
                        * ((40 as ::core::ffi::c_int) << resolutionFactor
                            << 2 as ::core::ffi::c_int) as ::core::ffi::c_long
                        >> 14 as ::core::ffi::c_int) as SDWORD as ::core::ffi::c_int)
                    as COORD;
                (*ShipImagePtr).current.location.y = ((*ShipImagePtr).current.location.y
                    as ::core::ffi::c_int
                    + (*(&raw mut sinetab as *mut SDWORD)
                        .offset(
                            (angle as ::core::ffi::c_int
                                & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                                    - 1 as ::core::ffi::c_int) as DWORD as isize,
                        ) as ::core::ffi::c_long
                        * ((40 as ::core::ffi::c_int) << resolutionFactor
                            << 2 as ::core::ffi::c_int) as ::core::ffi::c_long
                        >> 14 as ::core::ffi::c_int) as SDWORD as ::core::ffi::c_int)
                    as COORD;
                (*ElementPtr).preprocess_func = None;
            } else if (*ElementPtr).c2rust_unnamed_0.crew_level != 0 {
                let mut temp_x: SDWORD = (*ShipImagePtr).current.location.x as SDWORD
                    - (*(&raw mut sinetab as *mut SDWORD)
                        .offset(
                            (angle as ::core::ffi::c_int
                                + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                                    >> 2 as ::core::ffi::c_int)
                                & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                                    - 1 as ::core::ffi::c_int) as DWORD as isize,
                        ) as ::core::ffi::c_long
                        * ((40 as ::core::ffi::c_int) << resolutionFactor
                            << 2 as ::core::ffi::c_int) as ::core::ffi::c_long
                        >> 14 as ::core::ffi::c_int) as SDWORD
                        * ((*ElementPtr).c2rust_unnamed.life_span as SDWORD
                            - 1 as SDWORD);
                let mut temp_y: SDWORD = (*ShipImagePtr).current.location.y as SDWORD
                    - (*(&raw mut sinetab as *mut SDWORD)
                        .offset(
                            (angle as ::core::ffi::c_int
                                & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                                    - 1 as ::core::ffi::c_int) as DWORD as isize,
                        ) as ::core::ffi::c_long
                        * ((40 as ::core::ffi::c_int) << resolutionFactor
                            << 2 as ::core::ffi::c_int) as ::core::ffi::c_long
                        >> 14 as ::core::ffi::c_int) as SDWORD
                        * ((*ElementPtr).c2rust_unnamed.life_span as SDWORD
                            - 1 as SDWORD);
                (*ShipImagePtr).current.location.x = (if temp_x < 0 as ::core::ffi::c_int
                {
                    temp_x
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
                } else if temp_x
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
                    temp_x
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
                    temp_x
                }) as DWORD as COORD;
                (*ShipImagePtr).current.location.y = (if temp_y < 0 as ::core::ffi::c_int
                {
                    temp_y
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
                } else if temp_y
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
                    temp_y
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
                    temp_y
                }) as DWORD as COORD;
            }
            (*ShipImagePtr).preprocess_func = Some(
                ship_transition as unsafe extern "C" fn(*mut ELEMENT) -> (),
            ) as Option<ElementProcessFunc>;
            (*ShipImagePtr).death_func = Some(
                cycle_ion_trail as unsafe extern "C" fn(*mut ELEMENT) -> (),
            ) as Option<ElementProcessFunc>;
            (*ShipImagePtr).pParent = StarShipPtr as *mut ::core::ffi::c_void;
            UnlockLink(&raw mut disp_q, hShipImage);
        }
    }
    UnlockLink(&raw mut disp_q, (*StarShipPtr).hShip);
}
#[no_mangle]
pub unsafe extern "C" fn flee_preprocess(mut ElementPtr: *mut ELEMENT) {
    let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
    (*ElementPtr).c2rust_unnamed_2.turn_wait = (*ElementPtr)
        .c2rust_unnamed_2
        .turn_wait
        .wrapping_sub(1);
    if (*ElementPtr).c2rust_unnamed_2.turn_wait as ::core::ffi::c_int
        == 0 as ::core::ffi::c_int
    {
        static mut colorTab: [Color; 20] = [
            Color {
                r: ((0xa as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0xa as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                g: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                a: 0xff as BYTE,
            },
            Color {
                r: ((0xe as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0xe as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                g: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                a: 0xff as BYTE,
            },
            Color {
                r: ((0x13 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x13 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                g: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                a: 0xff as BYTE,
            },
            Color {
                r: ((0x17 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x17 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                g: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                a: 0xff as BYTE,
            },
            Color {
                r: ((0x1b as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x1b as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                g: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                a: 0xff as BYTE,
            },
            Color {
                r: ((0x1f as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x1f as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                g: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                a: 0xff as BYTE,
            },
            Color {
                r: ((0x1f as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x1f as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                g: ((0x4 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x4 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                b: ((0x4 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x4 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                a: 0xff as BYTE,
            },
            Color {
                r: ((0x1f as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x1f as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                g: ((0xa as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0xa as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                b: ((0xa as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0xa as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                a: 0xff as BYTE,
            },
            Color {
                r: ((0x1f as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x1f as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                g: ((0xf as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0xf as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                b: ((0xf as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0xf as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                a: 0xff as BYTE,
            },
            Color {
                r: ((0x1f as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x1f as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                g: ((0x13 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x13 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                b: ((0x13 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x13 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                a: 0xff as BYTE,
            },
            Color {
                r: ((0x1f as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x1f as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                g: ((0x19 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x19 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                b: ((0x19 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x19 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                a: 0xff as BYTE,
            },
            Color {
                r: ((0x1f as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x1f as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                g: ((0x13 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x13 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                b: ((0x13 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x13 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                a: 0xff as BYTE,
            },
            Color {
                r: ((0x1f as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x1f as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                g: ((0xf as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0xf as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                b: ((0xf as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0xf as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                a: 0xff as BYTE,
            },
            Color {
                r: ((0x1f as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x1f as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                g: ((0xa as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0xa as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                b: ((0xa as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0xa as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                a: 0xff as BYTE,
            },
            Color {
                r: ((0x1f as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x1f as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                g: ((0x4 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x4 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                b: ((0x4 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x4 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                a: 0xff as BYTE,
            },
            Color {
                r: ((0x1f as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x1f as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                g: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                a: 0xff as BYTE,
            },
            Color {
                r: ((0x1b as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x1b as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                g: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                a: 0xff as BYTE,
            },
            Color {
                r: ((0x17 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x17 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                g: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                a: 0xff as BYTE,
            },
            Color {
                r: ((0x13 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x13 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                g: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                a: 0xff as BYTE,
            },
            Color {
                r: ((0xe as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0xe as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                g: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                a: 0xff as BYTE,
            },
        ];
        let colorTabCount: size_t = ::core::mem::size_of::<[Color; 20]>()
            .wrapping_div(::core::mem::size_of::<Color>());
        (*ElementPtr).colorCycleIndex = (*ElementPtr).colorCycleIndex.wrapping_add(1);
        if (*ElementPtr).colorCycleIndex as size_t == colorTabCount {
            (*ElementPtr).colorCycleIndex = 0 as BYTE;
        }
        DisplayArray[(*ElementPtr).PrimIndex as usize].color = colorTab[(*ElementPtr)
            .colorCycleIndex as usize];
        if (*ElementPtr).colorCycleIndex as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        {
            (*ElementPtr).c2rust_unnamed_3.thrust_wait = (*ElementPtr)
                .c2rust_unnamed_3
                .thrust_wait
                .wrapping_sub(1);
        }
        (*ElementPtr).c2rust_unnamed_2.turn_wait = (*ElementPtr)
            .c2rust_unnamed_3
            .thrust_wait;
        if (*ElementPtr).c2rust_unnamed_2.turn_wait != 0 {
            (*ElementPtr).c2rust_unnamed_2.turn_wait = (((*ElementPtr)
                .c2rust_unnamed_2
                .turn_wait as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int) as BYTE;
        } else if (*ElementPtr).colorCycleIndex as size_t
            != colorTabCount.wrapping_div(2 as size_t)
        {
            (*ElementPtr).c2rust_unnamed_2.turn_wait = 1 as BYTE;
        } else {
            (*ElementPtr).death_func = Some(
                cleanup_dead_ship as unsafe extern "C" fn(*mut ELEMENT) -> (),
            ) as Option<ElementProcessFunc>;
            (*ElementPtr).c2rust_unnamed_0.crew_level = 0 as COUNT;
            (*ElementPtr).c2rust_unnamed.life_span = (15 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int) as COUNT;
            (*ElementPtr).preprocess_func = Some(
                ship_transition as unsafe extern "C" fn(*mut ELEMENT) -> (),
            ) as Option<ElementProcessFunc>;
            (*ElementPtr).postprocess_func = None;
            DisplayArray[(*ElementPtr).PrimIndex as usize].Type = gfx_object::NUM_PRIMS.0
                as ::core::ffi::c_int as GRAPHICS_PRIM;
            (*ElementPtr).state_flags = ((*ElementPtr).state_flags as ::core::ffi::c_int
                | ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int))
                as ELEMENT_FLAGS;
        }
    }
    StarShipPtr = (*ElementPtr).pParent as *mut STARSHIP;
    (*StarShipPtr).cur_status_flags = ((*StarShipPtr).cur_status_flags
        as ::core::ffi::c_int
        & !((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)) as STATUS_FLAGS;
    uqm_lua_checked!(PreProcessStatus(ElementPtr), ());
}
#[inline]
unsafe extern "C" fn isNetwork() -> BOOLEAN {
    return BOOLEAN(
        (*(&raw mut PlayerControl as *mut BYTE).offset(0isize) as ::core::ffi::c_int
            & ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as BYTE
                as ::core::ffi::c_int != 0
            || *(&raw mut PlayerControl as *mut BYTE).offset(1isize)
                as ::core::ffi::c_int
                & ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as BYTE
                    as ::core::ffi::c_int != 0) as ::core::ffi::c_int
            as ::core::ffi::c_uint,
    );
}
