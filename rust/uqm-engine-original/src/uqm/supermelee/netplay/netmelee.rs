/*
 *  Copyright 2006  Serge van den Boom <svdb@stack.nl>
 *
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
 *  Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA  02111-1307  USA
 */

// Rust translation of src/uqm/supermelee/netplay/netmelee.c.
extern "C" {
    pub type string_table_entry;
    pub type frame_desc;
    pub type NetConnection;
    pub type MeleeTeam;
    pub type MeleeSetup;
    pub type RandomContext;
    pub type tfb_soundsample;
    pub type getmelee_struct;
    fn __assert_fail(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    );
    fn NetConnection_open(
        player: ::core::ffi::c_int,
        options: *const NetplayPeerOptions,
        connectCallback: NetConnection_ConnectCallback,
        closeCallback: NetConnection_CloseCallback,
        errorCallback: NetConnection_ErrorCallback,
        deleteCallback: NetConnection_DeleteCallback,
        extra: *mut ::core::ffi::c_void,
    ) -> *mut NetConnection;
    fn NetConnection_close(conn: *mut NetConnection);
    fn NetConnection_webPoll(conn: *mut NetConnection);
    fn NetConnection_webDisconnect(conn: *mut NetConnection, err: ::core::ffi::c_int);
    fn NetConnection_isConnected(conn: *const NetConnection) -> bool;
    fn NetConnection_getStateData(
        conn: *const NetConnection,
    ) -> *mut NetConnectionStateData;
    fn NetConnection_getExtra(conn: *const NetConnection) -> *mut ::core::ffi::c_void;
    fn NetConnection_setState(conn: *mut NetConnection, state: NetState);
    fn NetConnection_getState(conn: *const NetConnection) -> NetState;
    fn NetConnection_getPlayerNr(conn: *const NetConnection) -> ::core::ffi::c_int;
    fn NetConnection_getInputDelay(conn: *const NetConnection) -> size_t;
    fn NetConnection_setReadyCallback(
        conn: *mut NetConnection,
        callback: NetConnection_ReadyCallback,
        arg: *mut ::core::ffi::c_void,
    );
    fn NetConnection_getReadyCallback(
        conn: *const NetConnection,
    ) -> NetConnection_ReadyCallback;
    fn NetConnection_getReadyCallbackArg(
        conn: *const NetConnection,
    ) -> *mut ::core::ffi::c_void;
    fn setBattleInputDelay(delay: size_t);
    fn BattleInputBuffer_pop(
        bib: *mut BattleInputBuffer,
        input: *mut BATTLE_INPUT_STATE,
    ) -> bool;
    fn getBattleInputBuffer(player: size_t) -> *mut BattleInputBuffer;
    fn DoInput(pInputState: *mut ::core::ffi::c_void, resetInput: BOOLEAN);
    fn sendAbort(conn: *mut NetConnection, reason: NetplayAbortReason);
    static mut netplayOptions: NetplayOptions;
    fn Callback_process();
    fn Async_process();
    fn Async_timeBeforeNextMs() -> uint32;
    fn NetMelee_connectCallback(conn: *mut NetConnection);
    fn NetMelee_closeCallback(conn: *mut NetConnection);
    fn NetMelee_errorCallback(
        conn: *mut NetConnection,
        error: *const NetConnectionError,
    );
    fn flushPacketQueue(conn: *mut NetConnection) -> ::core::ffi::c_int;
    fn Netplay_confirm(conn: *mut NetConnection) -> ::core::ffi::c_int;
    fn Netplay_cancelConfirmation(conn: *mut NetConnection) -> ::core::ffi::c_int;
    fn Netplay_localReady(
        conn: *mut NetConnection,
        callback: NetConnection_ReadyCallback,
        arg: *mut ::core::ffi::c_void,
        notifyRemote: bool,
    ) -> bool;
    fn Netplay_setResetCallback(
        conn: *mut NetConnection,
        callback: NetConnection_ResetCallback,
        resetArg: *mut ::core::ffi::c_void,
    );
    fn Netplay_localReset(conn: *mut NetConnection, reason: NetplayResetReason);
    fn Netplay_isLocalReset(conn: *const NetConnection) -> bool;
    fn emscripten_sleep(ms: ::core::ffi::c_uint);
    static mut GlobData: GLOBDATA;
    fn __errno_location() -> *mut ::core::ffi::c_int;
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
pub type DIRENTRY = STRING;
pub type TimeCount = DWORD;
pub type BATTLE_INPUT_STATE = UBYTE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BattleInputBuffer {
    pub buf: *mut BATTLE_INPUT_STATE,
    pub maxSize: size_t,
    pub first: size_t,
    pub size: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NetConnectionError {
    pub state: NetState,
    pub err: ::core::ffi::c_int,
    pub extra: C2Rust_Unnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed {
    pub listenError: *const ListenError,
    pub connectError: *const ConnectError,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ConnectError {
    pub state: ConnectStateState,
    pub err: ::core::ffi::c_int,
    pub resolveError: *const ResolveError,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ResolveError {
    pub gaiRes: ::core::ffi::c_int,
    pub err: ::core::ffi::c_int,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ConnectStateState(pub ::core::ffi::c_uint);
impl ConnectStateState {
    pub const Connect_closed: Self = Self(0);
    pub const Connect_resolving: Self = Self(1);
    pub const Connect_connecting: Self = Self(2);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ListenError {
    pub state: ListenStateState,
    pub err: ::core::ffi::c_int,
    pub resolveError: *const ResolveError,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ListenStateState(pub ::core::ffi::c_uint);
impl ListenStateState {
    pub const Listen_closed: Self = Self(0);
    pub const Listen_resolving: Self = Self(1);
    pub const Listen_listening: Self = Self(2);
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
pub type NetConnection_ConnectCallback = Option<
    unsafe extern "C" fn(*mut NetConnection) -> (),
>;
pub type NetConnection_CloseCallback = Option<
    unsafe extern "C" fn(*mut NetConnection) -> (),
>;
pub type NetConnection_ErrorCallback = Option<
    unsafe extern "C" fn(*mut NetConnection, *const NetConnectionError) -> (),
>;
pub type NetConnection_DeleteCallback = Option<
    unsafe extern "C" fn(*mut NetConnection) -> (),
>;
pub type NetConnection_ReadyCallback = Option<
    unsafe extern "C" fn(*mut NetConnection, *mut ::core::ffi::c_void) -> (),
>;
pub type NetConnection_ResetCallback = Option<
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NetplayPeerOptions {
    pub isServer: bool,
    pub host: *const ::core::ffi::c_char,
    pub port: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NetplayOptions {
    pub metaServer: *const ::core::ffi::c_char,
    pub metaPort: *const ::core::ffi::c_char,
    pub peer: [NetplayPeerOptions; 2],
    pub inputDelay: size_t,
}
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
    pub image: C2Rust_Unnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_0 {
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
pub type ElementProcessFunc = unsafe extern "C" fn(*mut ELEMENT) -> ();
pub type ELEMENT = element;
pub type ElementCollisionFunc = unsafe extern "C" fn(
    *mut ELEMENT,
    *mut POINT,
    *mut ELEMENT,
    *mut POINT,
) -> ();
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct NetplayAbortReason(pub ::core::ffi::c_uint);
impl NetplayAbortReason {
    pub const AbortReason_unspecified: Self = Self(0);
    pub const AbortReason_versionMismatch: Self = Self(1);
    pub const AbortReason_invalidHash: Self = Self(2);
    pub const AbortReason_protocolError: Self = Self(3);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct NetplayResetReason(pub ::core::ffi::c_uint);
impl NetplayResetReason {
    pub const ResetReason_unspecified: Self = Self(0);
    pub const ResetReason_syncLoss: Self = Self(1);
    pub const ResetReason_manualReset: Self = Self(2);
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
pub type SOUND = STRING;
pub type TFB_SoundSample = tfb_soundsample;
pub type MUSIC_REF = *mut *mut TFB_SoundSample;
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
pub struct NetworkInputContext {
    pub handlers: *mut BattleInputHandlers,
    pub playerNr: COUNT,
}
pub type ForEachConnectionCallback = Option<
    unsafe extern "C" fn(*mut NetConnection, *mut ::core::ffi::c_void) -> bool,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BattleStateData {
    pub releaseFunction: NetConnectionStateData_ReleaseFunction,
    pub meleeState: *mut melee_state,
    pub battleState: *mut battlestate_struct,
    pub getMeleeState: *mut getmelee_struct,
    pub endFrameCount: BattleFrameCounter,
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
    pub ShipName: [UNICODE; 16],
    pub CommanderName: [UNICODE; 16],
    pub PlanetName: [UNICODE; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LocalReadyConnectionArg {
    pub readyCallback: NetConnection_ReadyCallback,
    pub readyCallbackArg: *mut ::core::ffi::c_void,
    pub notifyRemote: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NegotiateReadyState {
    pub InputFunc: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> BOOLEAN>,
    pub conn: *mut NetConnection,
    pub nextState: NetState,
    pub done: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WaitReadyState {
    pub InputFunc: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> BOOLEAN>,
    pub conn: *mut NetConnection,
    pub readyCallback: NetConnection_ReadyCallback,
    pub readyCallbackArg: *mut ::core::ffi::c_void,
    pub done: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WaitResetState {
    pub InputFunc: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> BOOLEAN>,
    pub conn: *mut NetConnection,
    pub nextState: NetState,
    pub done: bool,
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
#[no_mangle]
pub static mut netConnections: [*mut NetConnection; 2] = [::core::ptr::null_mut::<
    NetConnection,
>(); 2];
#[no_mangle]
pub static mut numNetConnections: size_t = 0;
#[no_mangle]
pub unsafe extern "C" fn addNetConnection(
    mut conn: *mut NetConnection,
    mut playerNr: ::core::ffi::c_int,
) {
    netConnections[playerNr as usize] = conn;
    numNetConnections = numNetConnections.wrapping_add(1);
}
#[no_mangle]
pub unsafe extern "C" fn removeNetConnection(mut playerNr: ::core::ffi::c_int) {
    netConnections[playerNr as usize] = ::core::ptr::null_mut::<NetConnection>();
    numNetConnections = numNetConnections.wrapping_sub(1);
}
#[no_mangle]
pub unsafe extern "C" fn getNumNetConnections() -> size_t {
    return numNetConnections;
}
#[no_mangle]
pub unsafe extern "C" fn forEachConnectedPlayer(
    mut callback: ForEachConnectionCallback,
    mut arg: *mut ::core::ffi::c_void,
) -> bool {
    let mut player: COUNT = 0;
    player = 0 as COUNT;
    while (player as ::core::ffi::c_int) < 2 as ::core::ffi::c_int {
        let mut conn: *mut NetConnection = netConnections[player as usize];
        if !conn.is_null() {
            if NetConnection_isConnected(conn) {
                if !Some(callback.expect("non-null function pointer"))
                    .expect("non-null function pointer")(conn, arg)
                {
                    return false;
                }
            }
        }
        player = player.wrapping_add(1);
    }
    return true;
}
#[no_mangle]
pub unsafe extern "C" fn closeAllConnections() {
    let mut player: COUNT = 0;
    player = 0 as COUNT;
    while (player as ::core::ffi::c_int) < 2 as ::core::ffi::c_int {
        let mut conn: *mut NetConnection = netConnections[player as usize];
        if !conn.is_null() {
            closePlayerNetworkConnection(player);
        }
        player = player.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn closeDisconnectedConnections() {
    let mut player: COUNT = 0;
    player = 0 as COUNT;
    while (player as ::core::ffi::c_int) < 2 as ::core::ffi::c_int {
        let mut conn: *mut NetConnection = netConnections[player as usize];
        if !conn.is_null() && !NetConnection_isConnected(conn) {
            closePlayerNetworkConnection(player);
        }
        player = player.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn NetMelee_getMeleeState(
    mut conn: *mut NetConnection,
) -> *mut melee_state {
    if NetConnection_getState(conn).0 > NetState::NetState_connecting.0 {
        let mut battleStateData: *mut BattleStateData = NetConnection_getStateData(conn)
            as *mut BattleStateData;
        return (*battleStateData).meleeState;
    } else {
        return NetConnection_getExtra(conn) as *mut melee_state
    };
}
#[no_mangle]
pub unsafe extern "C" fn NetMelee_getBattleState(
    mut conn: *mut NetConnection,
) -> *mut battlestate_struct {
    if NetConnection_getState(conn).0 > NetState::NetState_connecting.0 {
        let mut battleStateData: *mut BattleStateData = NetConnection_getStateData(conn)
            as *mut BattleStateData;
        return (*battleStateData).battleState;
    } else {
        return ::core::ptr::null_mut::<battlestate_struct>()
    };
}
#[inline]
unsafe extern "C" fn netInputAux(mut timeoutMs: uint32) {
    let mut player: COUNT = 0;
    player = 0 as COUNT;
    while (player as ::core::ffi::c_int) < 2 as ::core::ffi::c_int {
        if !netConnections[player as usize].is_null() {
            NetConnection_webPoll(netConnections[player as usize]);
        }
        player = player.wrapping_add(1);
    }
    if timeoutMs > 0 as uint32 {
        emscripten_sleep(
            if timeoutMs < 8 as uint32 { timeoutMs } else { 8 as ::core::ffi::c_uint },
        );
    }
    Async_process();
    flushPacketQueues();
    Callback_process();
}
#[no_mangle]
pub unsafe extern "C" fn netInput() {
    netInputAux(0 as uint32);
}
#[no_mangle]
pub unsafe extern "C" fn netInputBlocking(mut timeoutMs: uint32) {
    let mut nextAsyncMs: uint32 = 0;
    nextAsyncMs = Async_timeBeforeNextMs();
    if nextAsyncMs < timeoutMs {
        timeoutMs = nextAsyncMs;
    }
    netInputAux(timeoutMs);
}
#[no_mangle]
pub unsafe extern "C" fn flushPacketQueues() {
    let mut player: COUNT = 0;
    player = 0 as COUNT;
    while (player as ::core::ffi::c_int) < 2 as ::core::ffi::c_int {
        let mut conn: *mut NetConnection = ::core::ptr::null_mut::<NetConnection>();
        let mut flushStatus: ::core::ffi::c_int = 0;
        conn = netConnections[player as usize];
        if !conn.is_null() {
            if NetConnection_isConnected(conn) {
                flushStatus = flushPacketQueue(conn);
                if flushStatus == -1 as ::core::ffi::c_int
                    && *__errno_location() != 6 as ::core::ffi::c_int
                    && *__errno_location() != 6 as ::core::ffi::c_int
                {
                    NetConnection_webDisconnect(conn, *__errno_location());
                }
            }
        }
        player = player.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn confirmConnections() {
    let mut player: COUNT = 0;
    player = 0 as COUNT;
    while (player as ::core::ffi::c_int) < 2 as ::core::ffi::c_int {
        let mut conn: *mut NetConnection = netConnections[player as usize];
        if !conn.is_null() {
            if NetConnection_isConnected(conn) {
                Netplay_confirm(conn);
            }
        }
        player = player.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn cancelConfirmations() {
    let mut player: COUNT = 0;
    player = 0 as COUNT;
    while (player as ::core::ffi::c_int) < 2 as ::core::ffi::c_int {
        let mut conn: *mut NetConnection = netConnections[player as usize];
        if !conn.is_null() {
            if NetConnection_isConnected(conn) {
                Netplay_cancelConfirmation(conn);
            }
        }
        player = player.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn connectionsLocalReady(
    mut callback: NetConnection_ReadyCallback,
    mut arg: *mut ::core::ffi::c_void,
) {
    let mut player: COUNT = 0;
    player = 0 as COUNT;
    while (player as ::core::ffi::c_int) < 2 as ::core::ffi::c_int {
        let mut conn: *mut NetConnection = netConnections[player as usize];
        if !conn.is_null() {
            if NetConnection_isConnected(conn) {
                Netplay_localReady(conn, callback, arg, true);
            }
        }
        player = player.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn allConnected() -> bool {
    let mut player: COUNT = 0;
    player = 0 as COUNT;
    while (player as ::core::ffi::c_int) < 2 as ::core::ffi::c_int {
        let mut conn: *mut NetConnection = netConnections[player as usize];
        if !conn.is_null() {
            if !NetConnection_isConnected(conn) {
                return false;
            }
        }
        player = player.wrapping_add(1);
    }
    return true;
}
#[no_mangle]
pub unsafe extern "C" fn initBattleStateDataConnections() {
    let mut player: COUNT = 0;
    player = 0 as COUNT;
    while (player as ::core::ffi::c_int) < 2 as ::core::ffi::c_int {
        let mut battleStateData: *mut BattleStateData = ::core::ptr::null_mut::<
            BattleStateData,
        >();
        let mut conn: *mut NetConnection = netConnections[player as usize];
        if !conn.is_null() {
            battleStateData = NetConnection_getStateData(conn) as *mut BattleStateData;
            (*battleStateData).endFrameCount = 0 as BattleFrameCounter;
        }
        player = player.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn setBattleStateConnections(mut bs: *mut battlestate_struct) {
    let mut player: COUNT = 0;
    player = 0 as COUNT;
    while (player as ::core::ffi::c_int) < 2 as ::core::ffi::c_int {
        let mut battleStateData: *mut BattleStateData = ::core::ptr::null_mut::<
            BattleStateData,
        >();
        let mut conn: *mut NetConnection = netConnections[player as usize];
        if !conn.is_null() {
            battleStateData = NetConnection_getStateData(conn) as *mut BattleStateData;
            (*battleStateData).battleState = bs;
        }
        player = player.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn networkBattleInput(
    mut context: *mut NetworkInputContext,
    mut StarShipPtr: *mut STARSHIP,
) -> BATTLE_INPUT_STATE {
    let mut bib: *mut BattleInputBuffer = getBattleInputBuffer(
        (*context).playerNr as size_t,
    );
    let mut result: BATTLE_INPUT_STATE = 0;
    loop {
        let mut ok: bool = false;
        ok = BattleInputBuffer_pop(bib, &raw mut result);
        if ok {
            break;
        }
        let mut conn: *mut NetConnection = netConnections[(*context).playerNr as usize];
        netInput();
        if !NetConnection_isConnected(conn) {
            GlobData.Game_state.CurrentActivity = (GlobData.Game_state.CurrentActivity
                as ::core::ffi::c_int
                | C2Rust_Unnamed_6::CHECK_ABORT.0 as ::core::ffi::c_int) as ACTIVITY;
            return 0 as ::core::ffi::c_int as BATTLE_INPUT_STATE;
        }
        if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
            & C2Rust_Unnamed_6::CHECK_ABORT.0 as ::core::ffi::c_int != 0
        {
            return 0 as ::core::ffi::c_int as BATTLE_INPUT_STATE;
        }
        netInputBlocking(500 as uint32);
        if !NetConnection_isConnected(conn) {
            GlobData.Game_state.CurrentActivity = (GlobData.Game_state.CurrentActivity
                as ::core::ffi::c_int
                | C2Rust_Unnamed_6::CHECK_ABORT.0 as ::core::ffi::c_int) as ACTIVITY;
            return 0 as ::core::ffi::c_int as BATTLE_INPUT_STATE;
        }
    }
    return result;
}
unsafe extern "C" fn deleteConnectionCallback(mut conn: *mut NetConnection) {
    removeNetConnection(NetConnection_getPlayerNr(conn));
}
#[no_mangle]
pub unsafe extern "C" fn openPlayerNetworkConnection(
    mut player: COUNT,
    mut extra: *mut ::core::ffi::c_void,
) -> *mut NetConnection {
    let mut conn: *mut NetConnection = ::core::ptr::null_mut::<NetConnection>();
    (netConnections[player as usize].is_null()
        || {
            __assert_fail(
                b"netConnections[player] == NULL\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/uqm/supermelee/netplay/netmelee.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                400 as ::core::ffi::c_int,
                b"openPlayerNetworkConnection\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    conn = NetConnection_open(
        player as ::core::ffi::c_int,
        (&raw mut netplayOptions.peer as *mut NetplayPeerOptions)
            .offset(player as isize),
        Some(NetMelee_connectCallback as unsafe extern "C" fn(*mut NetConnection) -> ()),
        Some(NetMelee_closeCallback as unsafe extern "C" fn(*mut NetConnection) -> ()),
        Some(
            NetMelee_errorCallback
                as unsafe extern "C" fn(
                    *mut NetConnection,
                    *const NetConnectionError,
                ) -> (),
        ),
        Some(deleteConnectionCallback as unsafe extern "C" fn(*mut NetConnection) -> ()),
        extra,
    );
    addNetConnection(conn, player as ::core::ffi::c_int);
    return conn;
}
#[no_mangle]
pub unsafe extern "C" fn closePlayerNetworkConnection(mut player: COUNT) {
    (!netConnections[player as usize].is_null()
        || {
            __assert_fail(
                b"netConnections[player] != NULL\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/uqm/supermelee/netplay/netmelee.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                413 as ::core::ffi::c_int,
                b"closePlayerNetworkConnection\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    NetConnection_close(netConnections[player as usize]);
}
#[no_mangle]
pub unsafe extern "C" fn setupInputDelay(mut localInputDelay: size_t) -> bool {
    let mut player: COUNT = 0;
    let mut haveNetworkPlayer: bool = false;
    let mut inputDelay: size_t = 0 as size_t;
    player = 0 as COUNT;
    while (player as ::core::ffi::c_int) < 2 as ::core::ffi::c_int {
        let mut conn: *mut NetConnection = netConnections[player as usize];
        if !conn.is_null() {
            if NetConnection_isConnected(conn) {
                haveNetworkPlayer = true;
                if NetConnection_getInputDelay(conn) > inputDelay {
                    inputDelay = NetConnection_getInputDelay(conn);
                }
            }
        }
        player = player.wrapping_add(1);
    }
    if haveNetworkPlayer as ::core::ffi::c_int != 0 && inputDelay < localInputDelay {
        inputDelay = localInputDelay;
    }
    setBattleInputDelay(inputDelay);
    return true;
}
unsafe extern "C" fn setStateConnection(
    mut conn: *mut NetConnection,
    mut arg: *mut ::core::ffi::c_void,
) -> bool {
    let mut state: *const NetState = arg as *mut NetState;
    NetConnection_setState(conn, *state);
    return true;
}
#[no_mangle]
pub unsafe extern "C" fn setStateConnections(mut state: NetState) -> bool {
    return forEachConnectedPlayer(
        Some(
            setStateConnection
                as unsafe extern "C" fn(
                    *mut NetConnection,
                    *mut ::core::ffi::c_void,
                ) -> bool,
        ),
        &raw mut state as *mut ::core::ffi::c_void,
    );
}
unsafe extern "C" fn sendAbortConnection(
    mut conn: *mut NetConnection,
    mut arg: *mut ::core::ffi::c_void,
) -> bool {
    let mut reason: *const NetplayAbortReason = arg as *mut NetplayAbortReason;
    sendAbort(conn, *reason);
    return true;
}
#[no_mangle]
pub unsafe extern "C" fn sendAbortConnections(mut reason: NetplayAbortReason) -> bool {
    return forEachConnectedPlayer(
        Some(
            sendAbortConnection
                as unsafe extern "C" fn(
                    *mut NetConnection,
                    *mut ::core::ffi::c_void,
                ) -> bool,
        ),
        &raw mut reason as *mut ::core::ffi::c_void,
    );
}
unsafe extern "C" fn resetConnection(
    mut conn: *mut NetConnection,
    mut arg: *mut ::core::ffi::c_void,
) -> bool {
    let mut reason: *const NetplayResetReason = arg as *mut NetplayResetReason;
    Netplay_localReset(conn, *reason);
    return true;
}
#[no_mangle]
pub unsafe extern "C" fn resetConnections(mut reason: NetplayResetReason) -> bool {
    return forEachConnectedPlayer(
        Some(
            resetConnection
                as unsafe extern "C" fn(
                    *mut NetConnection,
                    *mut ::core::ffi::c_void,
                ) -> bool,
        ),
        &raw mut reason as *mut ::core::ffi::c_void,
    );
}
unsafe extern "C" fn localReadyConnection(
    mut conn: *mut NetConnection,
    mut arg: *mut ::core::ffi::c_void,
) -> bool {
    let mut readyArg: *mut LocalReadyConnectionArg = arg as *mut LocalReadyConnectionArg;
    Netplay_localReady(
        conn,
        (*readyArg).readyCallback,
        (*readyArg).readyCallbackArg,
        (*readyArg).notifyRemote,
    );
    return true;
}
#[no_mangle]
pub unsafe extern "C" fn localReadyConnections(
    mut readyCallback: NetConnection_ReadyCallback,
    mut readyArg: *mut ::core::ffi::c_void,
    mut notifyRemote: bool,
) -> bool {
    let mut arg: LocalReadyConnectionArg = LocalReadyConnectionArg {
        readyCallback: None,
        readyCallbackArg: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        notifyRemote: false,
    };
    arg.readyCallback = readyCallback;
    arg.readyCallbackArg = readyArg;
    arg.notifyRemote = notifyRemote;
    return forEachConnectedPlayer(
        Some(
            localReadyConnection
                as unsafe extern "C" fn(
                    *mut NetConnection,
                    *mut ::core::ffi::c_void,
                ) -> bool,
        ),
        &raw mut arg as *mut ::core::ffi::c_void,
    );
}
unsafe extern "C" fn negotiateReadyInputFunc(
    mut state: *mut NegotiateReadyState,
) -> BOOLEAN {
    netInputBlocking((840 as ::core::ffi::c_int / 24 as ::core::ffi::c_int) as uint32);
    if !NetConnection_isConnected((*state).conn) {
        return BOOLEAN::FALSE;
    }
    return BOOLEAN(!(*state).done as ::core::ffi::c_int as ::core::ffi::c_uint);
}
unsafe extern "C" fn negotiateReadyBothReadyCallback(
    mut conn: *mut NetConnection,
    mut arg: *mut ::core::ffi::c_void,
) {
    let mut state: *mut NegotiateReadyState = arg as *mut NegotiateReadyState;
    NetConnection_setState(conn, (*state).nextState);
    (*state).done = true;
}
#[no_mangle]
pub unsafe extern "C" fn negotiateReady(
    mut conn: *mut NetConnection,
    mut notifyRemote: bool,
    mut nextState: NetState,
) -> bool {
    let mut state: NegotiateReadyState = NegotiateReadyState {
        InputFunc: None,
        conn: ::core::ptr::null_mut::<NetConnection>(),
        nextState: NetState::NetState_unconnected,
        done: false,
    };
    state.InputFunc = ::core::mem::transmute::<
        Option<unsafe extern "C" fn(*mut NegotiateReadyState) -> BOOLEAN>,
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> BOOLEAN>,
    >(
        Some(
            negotiateReadyInputFunc
                as unsafe extern "C" fn(*mut NegotiateReadyState) -> BOOLEAN,
        ),
    );
    state.conn = conn;
    state.nextState = nextState;
    state.done = false;
    Netplay_localReady(
        conn,
        Some(
            negotiateReadyBothReadyCallback
                as unsafe extern "C" fn(
                    *mut NetConnection,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        &raw mut state as *mut ::core::ffi::c_void,
        notifyRemote,
    );
    flushPacketQueue(conn);
    if !state.done {
        DoInput(&raw mut state as *mut ::core::ffi::c_void, BOOLEAN::FALSE);
    }
    return NetConnection_isConnected(conn);
}
#[no_mangle]
pub unsafe extern "C" fn negotiateReadyConnections(
    mut notifyRemote: bool,
    mut nextState: NetState,
) -> bool {
    let mut player: COUNT = 0;
    let mut numDisconnected: size_t = 0 as size_t;
    player = 0 as COUNT;
    while (player as ::core::ffi::c_int) < 2 as ::core::ffi::c_int {
        let mut conn: *mut NetConnection = netConnections[player as usize];
        if !conn.is_null() {
            if !NetConnection_isConnected(conn) {
                numDisconnected = numDisconnected.wrapping_add(1);
            } else if !negotiateReady(conn, notifyRemote, nextState) {
                numDisconnected = numDisconnected.wrapping_add(1);
            }
        }
        player = player.wrapping_add(1);
    }
    return numDisconnected == 0 as size_t;
}
unsafe extern "C" fn waitReadyCallback(
    mut conn: *mut NetConnection,
    mut arg: *mut ::core::ffi::c_void,
) {
    let mut state: *mut WaitReadyState = arg as *mut WaitReadyState;
    (*state).done = true;
    (*state)
        .readyCallback
        .expect("non-null function pointer")(conn, (*state).readyCallbackArg);
}
unsafe extern "C" fn waitReadyInputFunc(mut state: *mut WaitReadyState) -> BOOLEAN {
    netInputBlocking((840 as ::core::ffi::c_int / 24 as ::core::ffi::c_int) as uint32);
    if !NetConnection_isConnected((*state).conn) {
        return BOOLEAN::FALSE;
    }
    return BOOLEAN(!(*state).done as ::core::ffi::c_int as ::core::ffi::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn waitReady(mut conn: *mut NetConnection) -> bool {
    let mut state: WaitReadyState = WaitReadyState {
        InputFunc: None,
        conn: ::core::ptr::null_mut::<NetConnection>(),
        readyCallback: None,
        readyCallbackArg: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        done: false,
    };
    state.InputFunc = ::core::mem::transmute::<
        Option<unsafe extern "C" fn(*mut WaitReadyState) -> BOOLEAN>,
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> BOOLEAN>,
    >(Some(waitReadyInputFunc as unsafe extern "C" fn(*mut WaitReadyState) -> BOOLEAN));
    state.conn = conn;
    state.readyCallback = NetConnection_getReadyCallback(conn);
    state.readyCallbackArg = NetConnection_getReadyCallbackArg(conn);
    state.done = false;
    NetConnection_setReadyCallback(
        conn,
        Some(
            waitReadyCallback
                as unsafe extern "C" fn(
                    *mut NetConnection,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        &raw mut state as *mut ::core::ffi::c_void,
    );
    DoInput(&raw mut state as *mut ::core::ffi::c_void, BOOLEAN::FALSE);
    return NetConnection_isConnected(conn);
}
unsafe extern "C" fn waitResetInputFunc(mut state: *mut WaitResetState) -> BOOLEAN {
    netInputBlocking((840 as ::core::ffi::c_int / 24 as ::core::ffi::c_int) as uint32);
    if !NetConnection_isConnected((*state).conn) {
        return BOOLEAN::FALSE;
    }
    return BOOLEAN(!(*state).done as ::core::ffi::c_int as ::core::ffi::c_uint);
}
unsafe extern "C" fn waitResetBothResetCallback(
    mut conn: *mut NetConnection,
    mut arg: *mut ::core::ffi::c_void,
) {
    let mut state: *mut WaitResetState = arg as *mut WaitResetState;
    if (*state).nextState.0
        != NetState(-1 as ::core::ffi::c_int as ::core::ffi::c_uint).0
    {
        NetConnection_setState(conn, (*state).nextState);
    }
    (*state).done = true;
}
#[no_mangle]
pub unsafe extern "C" fn waitReset(
    mut conn: *mut NetConnection,
    mut nextState: NetState,
) -> bool {
    let mut state: WaitResetState = WaitResetState {
        InputFunc: None,
        conn: ::core::ptr::null_mut::<NetConnection>(),
        nextState: NetState::NetState_unconnected,
        done: false,
    };
    state.InputFunc = ::core::mem::transmute::<
        Option<unsafe extern "C" fn(*mut WaitResetState) -> BOOLEAN>,
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> BOOLEAN>,
    >(Some(waitResetInputFunc as unsafe extern "C" fn(*mut WaitResetState) -> BOOLEAN));
    state.conn = conn;
    state.nextState = nextState;
    state.done = false;
    Netplay_setResetCallback(
        conn,
        Some(
            waitResetBothResetCallback
                as unsafe extern "C" fn(
                    *mut NetConnection,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        &raw mut state as *mut ::core::ffi::c_void,
    );
    if !state.done {
        if !Netplay_isLocalReset(conn) {
            Netplay_localReset(conn, NetplayResetReason::ResetReason_manualReset);
            flushPacketQueue(conn);
        }
        if !state.done {
            DoInput(&raw mut state as *mut ::core::ffi::c_void, BOOLEAN::FALSE);
        }
    }
    return NetConnection_isConnected(conn);
}
#[no_mangle]
pub unsafe extern "C" fn waitResetConnections(mut nextState: NetState) -> bool {
    let mut player: COUNT = 0;
    let mut numDisconnected: size_t = 0 as size_t;
    player = 0 as COUNT;
    while (player as ::core::ffi::c_int) < 2 as ::core::ffi::c_int {
        let mut conn: *mut NetConnection = netConnections[player as usize];
        if !conn.is_null() {
            if !NetConnection_isConnected(conn) {
                numDisconnected = numDisconnected.wrapping_add(1);
            } else if !waitReset(conn, nextState) {
                numDisconnected = numDisconnected.wrapping_add(1);
            }
        }
        player = player.wrapping_add(1);
    }
    return numDisconnected == 0 as size_t;
}
