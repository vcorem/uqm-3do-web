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

// Rust translation of src/uqm/supermelee/pickmele.c.
extern "C" {
    pub type FlashContext;
    pub type string_table_entry;
    pub type context_desc;
    pub type frame_desc;
    pub type font_desc;
    pub type drawable_desc;
    pub type tfb_soundsample;
    pub type MeleeTeam;
    pub type MeleeSetup;
    pub type RandomContext;
    pub type NetConnection;
    fn SetContext(Context: CONTEXT) -> CONTEXT;
    fn SetContextForeGroundColor(Color: Color) -> Color;
    fn SetContextFGFrame(Frame: FRAME) -> FRAME;
    fn DrawStamp(pStamp: *mut STAMP);
    fn DrawFilledRectangle(pRect: *mut RECT);
    fn font_DrawText(pText: *mut TEXT);
    fn CreateDrawable(
        CreateFlags: CREATE_FLAGS,
        width: SIZE,
        height: SIZE,
        num_frames: COUNT,
    ) -> DRAWABLE;
    fn DestroyDrawable(Drawable: DRAWABLE) -> BOOLEAN;
    fn GetFrameRect(Frame: FRAME, pRect: *mut RECT) -> BOOLEAN;
    fn LoadGraphicInstance(res: RESOURCE) -> *mut ::core::ffi::c_void;
    fn SetContextFont(Font: FONT) -> FONT;
    fn SetAbsFrameIndex(Frame: FRAME, FrameIndex: COUNT) -> FRAME;
    fn IncFrameIndex(Frame: FRAME) -> FRAME;
    fn CaptureDrawable(Drawable: DRAWABLE) -> FRAME;
    fn ReleaseDrawable(Frame: FRAME) -> DRAWABLE;
    fn FadeScreen(fadeType: ScreenFadeType, TimeInterval: SIZE) -> DWORD;
    fn FlushColorXForms();
    fn __assert_fail(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    );
    fn sprintf(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn WaitForSoundEnd(Channel: COUNT);
    static mut PlayerInput: [*mut InputContext; 0];
    fn CountLinks(pq: *mut QUEUE) -> COUNT;
    fn NetConnection_isConnected(conn: *const NetConnection) -> bool;
    fn NetConnection_getStateData(
        conn: *const NetConnection,
    ) -> *mut NetConnectionStateData;
    fn TFB_Random() -> DWORD;
    fn MeleeSetup_getShip(
        setup: *const MeleeSetup,
        teamNr: size_t,
        slotNr: FleetShipIndex,
    ) -> MeleeShip;
    fn MeleeSetup_getTeamName(
        setup: *const MeleeSetup,
        teamNr: size_t,
    ) -> *const ::core::ffi::c_char;
    fn MeleeSetup_getFleetValue(setup: *const MeleeSetup, teamNr: size_t) -> COUNT;
    fn PulsedInputToBattleInput(player: COUNT) -> BATTLE_INPUT_STATE;
    static mut PulsedInputState: CONTROLLER_INPUT_STATE;
    fn UpdateInputState();
    fn SetDefaultMenuRepeatDelay();
    fn ConfirmExit() -> BOOLEAN;
    fn DoInput(pInputState: *mut ::core::ffi::c_void, resetInput: BOOLEAN);
    fn GetTimeCounter() -> TimeCount;
    static mut battle_counter: [BYTE; 2];
    fn GetPlayerOrder(i: COUNT) -> COUNT;
    fn Build(pQueue: *mut QUEUE, SpeciesID: SPECIES_ID) -> HLINK;
    fn GetStarShipFromIndex(pShipQ: *mut QUEUE, Index: COUNT) -> HLINK;
    fn NameCaptain(pQueue: *mut QUEUE, SpeciesID: SPECIES_ID) -> BYTE;
    static mut master_q: QUEUE;
    static mut netConnections: [*mut NetConnection; 0];
    fn netInput();
    fn flushPacketQueues();
    fn allConnected() -> bool;
    fn setStateConnections(state: NetState) -> bool;
    fn negotiateReadyConnections(notifyRemote: bool, nextState: NetState) -> bool;
    fn Flash_createHighlight(
        gfxContext: CONTEXT,
        rect: *const RECT,
    ) -> *mut FlashContext;
    fn Flash_start(context: *mut FlashContext);
    fn Flash_terminate(context: *mut FlashContext);
    fn Flash_process(context: *mut FlashContext);
    fn Flash_setSpeed(
        context: *mut FlashContext,
        fadeInTime: TimeCount,
        onTime: TimeCount,
        fadeOutTime: TimeCount,
        offTime: TimeCount,
    );
    fn Flash_setMergeFactors(
        context: *mut FlashContext,
        startNumer: ::core::ffi::c_int,
        endNumer: ::core::ffi::c_int,
        denom: ::core::ffi::c_int,
    );
    fn Flash_setFrameTime(context: *mut FlashContext, frameTime: TimeCount);
    fn Flash_setRect(context: *mut FlashContext, rect: *const RECT);
    fn SleepThread(timePeriod: TimePeriod);
    fn SleepThreadUntil(wakeTime: TimeCount);
    fn TaskSwitch();
    static mut GlobData: GLOBDATA;
    static mut StatusFrame: FRAME;
    static mut OffScreenContext: CONTEXT;
    static mut ScreenContext: CONTEXT;
    static mut SpaceContext: CONTEXT;
    static mut TinyFont: FONT;
    static mut race_q: [QUEUE; 0];
    static mut PlayerControl: [BYTE; 0];
    fn SetMenuSounds(sound_0: MENU_SOUND_FLAGS, sound_1: MENU_SOUND_FLAGS);
    fn PlayMenuSound(S: MENU_SOUND_EFFECT);
    fn Async_process();
    fn log_add(_: log_Level, fmt: *const ::core::ffi::c_char, ...);
    fn Netplay_Notify_shipSelected(conn: *mut NetConnection, index: FleetShipIndex);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct getmelee_struct {
    pub InputFunc: Option<unsafe extern "C" fn(*mut getmelee_struct) -> BOOLEAN>,
    pub Initialized: BOOLEAN,
    pub player: [C2Rust_Unnamed; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed {
    pub timeIn: TimeCount,
    pub hBattleShip: HSTARSHIP,
    pub choice: COUNT,
    pub row: COUNT,
    pub col: COUNT,
    pub ships_left: COUNT,
    pub randomIndex: COUNT,
    pub selecting: BOOLEAN,
    pub done: BOOLEAN,
    pub flashContext: *mut FlashContext,
    pub remoteSelected: BOOLEAN,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct BOOLEAN(pub ::core::ffi::c_uint);
impl BOOLEAN {
    pub const FALSE: Self = Self(0);
    pub const TRUE: Self = Self(1);
}
pub type COUNT = UWORD;
pub type UWORD = uint16;
pub type uint16 = ::core::ffi::c_ushort;
pub type HSTARSHIP = HLINK;
pub type HLINK = QUEUE_HANDLE;
pub type QUEUE_HANDLE = *mut ::core::ffi::c_void;
pub type TimeCount = DWORD;
pub type DWORD = uint32;
pub type uint32 = ::core::ffi::c_uint;
pub type GETMELEE_STATE = getmelee_struct;
pub type uint8 = ::core::ffi::c_uchar;
pub type sint16 = ::core::ffi::c_short;
pub type sint32 = ::core::ffi::c_int;
pub type BYTE = uint8;
pub type UBYTE = uint8;
pub type SWORD = sint16;
pub type SDWORD = sint32;
pub type SIZE = SWORD;
pub type UNICODE = ::core::ffi::c_char;
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
pub type CONTEXT_DESC = context_desc;
pub type FRAME_DESC = frame_desc;
pub type FONT_DESC = font_desc;
pub type DRAWABLE_DESC = drawable_desc;
pub type CONTEXT = *mut CONTEXT_DESC;
pub type FRAME = *mut FRAME_DESC;
pub type FONT = *mut FONT_DESC;
pub type DRAWABLE = *mut DRAWABLE_DESC;
pub type TIME_VALUE = UWORD;
pub type COORD = SWORD;
pub type CREATE_FLAGS = BYTE;
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ScreenFadeType(pub ::core::ffi::c_uint);
impl ScreenFadeType {
    pub const FadeAllToWhite: Self = Self(250);
    pub const FadeSomeToWhite: Self = Self(251);
    pub const FadeAllToBlack: Self = Self(252);
    pub const FadeAllToColor: Self = Self(253);
    pub const FadeSomeToBlack: Self = Self(254);
    pub const FadeSomeToColor: Self = Self(255);
}
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
pub type TimePeriod = DWORD;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_6(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_6 {
    pub const KEY_PAUSE: Self = Self(0);
    pub const KEY_EXIT: Self = Self(1);
    pub const KEY_ABORT: Self = Self(2);
    pub const KEY_DEBUG: Self = Self(3);
    pub const KEY_FULLSCREEN: Self = Self(4);
    pub const KEY_MENU_UP: Self = Self(5);
    pub const KEY_MENU_DOWN: Self = Self(6);
    pub const KEY_MENU_LEFT: Self = Self(7);
    pub const KEY_MENU_RIGHT: Self = Self(8);
    pub const KEY_MENU_SELECT: Self = Self(9);
    pub const KEY_MENU_CANCEL: Self = Self(10);
    pub const KEY_MENU_SPECIAL: Self = Self(11);
    pub const KEY_MENU_PAGE_UP: Self = Self(12);
    pub const KEY_MENU_PAGE_DOWN: Self = Self(13);
    pub const KEY_MENU_HOME: Self = Self(14);
    pub const KEY_MENU_END: Self = Self(15);
    pub const KEY_MENU_ZOOM_IN: Self = Self(16);
    pub const KEY_MENU_ZOOM_OUT: Self = Self(17);
    pub const KEY_MENU_DELETE: Self = Self(18);
    pub const KEY_MENU_BACKSPACE: Self = Self(19);
    pub const KEY_MENU_EDIT_CANCEL: Self = Self(20);
    pub const KEY_MENU_SEARCH: Self = Self(21);
    pub const KEY_MENU_NEXT: Self = Self(22);
    pub const KEY_MENU_ANY: Self = Self(23);
    pub const NUM_MENU_KEYS: Self = Self(24);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _controller_input_state {
    pub key: [[::core::ffi::c_int; 7]; 6],
    pub menu: [::core::ffi::c_int; 24],
}
pub type CONTROLLER_INPUT_STATE = _controller_input_state;
pub type FleetShipIndex = COUNT;
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
pub type HMASTERSHIP = HLINK;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MASTER_SHIP_INFO {
    pub pred: HMASTERSHIP,
    pub succ: HMASTERSHIP,
    pub SpeciesID: SPECIES_ID,
    pub ShipInfo: SHIP_INFO,
    pub Fleet: FLEET_STUFF,
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
pub struct BattleStateData {
    pub releaseFunction: NetConnectionStateData_ReleaseFunction,
    pub meleeState: *mut melee_state,
    pub battleState: *mut battlestate_struct,
    pub getMeleeState: *mut getmelee_struct,
    pub endFrameCount: BattleFrameCounter,
}
pub type MENU_SOUND_FLAGS = UWORD;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct log_Level(pub ::core::ffi::c_uint);
impl log_Level {
    pub const log_Nothing: Self = Self(0);
    pub const log_User: Self = Self(1);
    pub const log_Fatal: Self = Self(1);
    pub const log_Error: Self = Self(2);
    pub const log_Warning: Self = Self(3);
    pub const log_Info: Self = Self(4);
    pub const log_Debug: Self = Self(5);
    pub const log_All: Self = Self(6);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct MENU_SOUND_EFFECT(pub ::core::ffi::c_uint);
impl MENU_SOUND_EFFECT {
    pub const MENU_SOUND_MOVE: Self = Self(0);
    pub const MENU_SOUND_SUCCESS: Self = Self(1);
    pub const MENU_SOUND_FAILURE: Self = Self(2);
    pub const MENU_SOUND_INVOKED: Self = Self(3);
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
                293 as ::core::ffi::c_int,
                b"LockStarShip\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    return LockLink(pq, h) as *mut STARSHIP;
}
#[no_mangle]
pub static mut PickMeleeFrame: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
unsafe extern "C" fn PickMelee_GetShipIndex(
    mut row: BYTE,
    mut col: BYTE,
) -> FleetShipIndex {
    return (row as ::core::ffi::c_int * 7 as ::core::ffi::c_int
        + col as ::core::ffi::c_int) as FleetShipIndex;
}
unsafe extern "C" fn PickMelee_GetShipRow(mut index: FleetShipIndex) -> BYTE {
    return (index as ::core::ffi::c_int / 7 as ::core::ffi::c_int) as BYTE;
}
unsafe extern "C" fn PickMelee_GetShipColumn(mut index: ::core::ffi::c_int) -> BYTE {
    return (index % 7 as ::core::ffi::c_int) as BYTE;
}
unsafe extern "C" fn MeleeShipByQueueIndex(
    mut queue: *const QUEUE,
    mut index: COUNT,
) -> HSTARSHIP {
    let mut hShip: HSTARSHIP = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut hNextShip: HSTARSHIP = ::core::ptr::null_mut::<::core::ffi::c_void>();
    hShip = (*queue).head as HSTARSHIP;
    while !hShip.is_null() {
        let mut StarShipPtr: *mut STARSHIP = LockStarShip(queue, hShip);
        if (*StarShipPtr).index as ::core::ffi::c_int == index as ::core::ffi::c_int {
            hNextShip = hShip;
            if (*StarShipPtr).SpeciesID.0 == SPECIES_ID::NO_ID.0 {
                hShip = ::core::ptr::null_mut::<::core::ffi::c_void>();
            }
            UnlockLink(queue, hNextShip);
            break;
        } else {
            hNextShip = (*StarShipPtr).succ as HSTARSHIP;
            UnlockLink(queue, hShip);
            hShip = hNextShip;
        }
    }
    return hShip;
}
unsafe extern "C" fn MeleeShipByUsedIndex(
    mut queue: *const QUEUE,
    mut index: COUNT,
) -> HSTARSHIP {
    let mut hShip: HSTARSHIP = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut hNextShip: HSTARSHIP = ::core::ptr::null_mut::<::core::ffi::c_void>();
    hShip = (*queue).head as HSTARSHIP;
    while !hShip.is_null() {
        let mut StarShipPtr: *mut STARSHIP = LockStarShip(queue, hShip);
        if (*StarShipPtr).SpeciesID.0 != SPECIES_ID::NO_ID.0
            && {
                let c2rust_fresh0 = index;
                index = index.wrapping_sub(1);
                c2rust_fresh0 as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            }
        {
            UnlockLink(queue, hShip);
            break;
        } else {
            hNextShip = (*StarShipPtr).succ as HSTARSHIP;
            UnlockLink(queue, hShip);
            hShip = hNextShip;
        }
    }
    return hShip;
}
unsafe extern "C" fn PickMelee_ChangedSelection(
    mut gms: *mut GETMELEE_STATE,
    mut playerI: COUNT,
) {
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    r.corner.x = (57 as ::core::ffi::c_int
        + (16 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
            * (*gms).player[playerI as usize].col as ::core::ffi::c_int) as COORD;
    r.corner.y = (24 as ::core::ffi::c_int
        + (16 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
            * (*gms).player[playerI as usize].row as ::core::ffi::c_int
        + (1 as ::core::ffi::c_int - playerI as ::core::ffi::c_int)
            * 100 as ::core::ffi::c_int) as COORD;
    r.extent.width = (16 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as COORD;
    r.extent.height = (16 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as COORD;
    Flash_setRect((*gms).player[playerI as usize].flashContext, &raw mut r);
}
#[no_mangle]
pub unsafe extern "C" fn setShipSelected(
    mut gms: *mut GETMELEE_STATE,
    mut playerI: COUNT,
    mut choice: COUNT,
    mut reportNetwork: bool,
) -> bool {
    let mut ship: HSTARSHIP = ::core::ptr::null_mut::<::core::ffi::c_void>();
    ((*gms).player[playerI as usize].done.0 == 0
        || {
            __assert_fail(
                b"!gms->player[playerI].done\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/uqm/supermelee/pickmele.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                166 as ::core::ffi::c_int,
                b"setShipSelected\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    if choice as ::core::ffi::c_int
        == !(0 as ::core::ffi::c_int) as COUNT as ::core::ffi::c_int
    {
        ship = MeleeShipByUsedIndex(
            (&raw mut race_q as *mut QUEUE).offset(playerI as isize),
            (*gms).player[playerI as usize].randomIndex,
        );
    } else {
        ship = MeleeShipByQueueIndex(
            (&raw mut race_q as *mut QUEUE).offset(playerI as isize),
            choice,
        );
    }
    if ship.is_null() {
        return false;
    }
    (*gms).player[playerI as usize].choice = choice;
    (*gms).player[playerI as usize].hBattleShip = ship;
    PlayMenuSound(MENU_SOUND_EFFECT::MENU_SOUND_SUCCESS);
    if reportNetwork {
        reportShipSelected(gms, choice);
    }
    (*gms).player[playerI as usize].done = BOOLEAN(
        1 as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
    return true;
}
unsafe extern "C" fn SelectShip_processInput(
    mut gms: *mut GETMELEE_STATE,
    mut playerI: COUNT,
    mut inputState: BATTLE_INPUT_STATE,
) -> BOOLEAN {
    if inputState as ::core::ffi::c_int
        & ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as BATTLE_INPUT_STATE
            as ::core::ffi::c_int != 0
    {
        if (*gms).player[playerI as usize].col as ::core::ffi::c_int
            == 7 as ::core::ffi::c_int
            && (*gms).player[playerI as usize].row as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
        {
            setShipSelected(
                gms,
                playerI,
                !(0 as ::core::ffi::c_int) as COUNT,
                BOOLEAN::TRUE.0 as ::core::ffi::c_int != 0,
            );
        } else if (*gms).player[playerI as usize].col as ::core::ffi::c_int
            == 7 as ::core::ffi::c_int
            && (*gms).player[playerI as usize].row as ::core::ffi::c_int
                == 1 as ::core::ffi::c_int
        {
            if ConfirmExit().0 != 0 {
                return BOOLEAN::FALSE;
            }
        } else {
            let mut slotNr: COUNT = PickMelee_GetShipIndex(
                (*gms).player[playerI as usize].row as BYTE,
                (*gms).player[playerI as usize].col as BYTE,
            );
            setShipSelected(
                gms,
                playerI,
                slotNr,
                BOOLEAN::TRUE.0 as ::core::ffi::c_int != 0,
            );
        }
    } else {
        let mut new_row: COUNT = 0;
        let mut new_col: COUNT = 0;
        new_row = (*gms).player[playerI as usize].row;
        new_col = (*gms).player[playerI as usize].col;
        if inputState as ::core::ffi::c_int
            & ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int)
                as BATTLE_INPUT_STATE as ::core::ffi::c_int != 0
        {
            let c2rust_fresh1 = new_col;
            new_col = new_col.wrapping_sub(1);
            if c2rust_fresh1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                new_col = 7 as COUNT;
            }
        } else if inputState as ::core::ffi::c_int
            & ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
                as BATTLE_INPUT_STATE as ::core::ffi::c_int != 0
        {
            let c2rust_fresh2 = new_col;
            new_col = new_col.wrapping_add(1);
            if c2rust_fresh2 as ::core::ffi::c_int == 7 as ::core::ffi::c_int {
                new_col = 0 as COUNT;
            }
        }
        if inputState as ::core::ffi::c_int
            & ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                as BATTLE_INPUT_STATE as ::core::ffi::c_int != 0
        {
            let c2rust_fresh3 = new_row;
            new_row = new_row.wrapping_sub(1);
            if c2rust_fresh3 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                new_row = (2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as COUNT;
            }
        } else if inputState as ::core::ffi::c_int
            & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                as BATTLE_INPUT_STATE as ::core::ffi::c_int != 0
        {
            new_row = new_row.wrapping_add(1);
            if new_row as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
                new_row = 0 as COUNT;
            }
        }
        if new_row as ::core::ffi::c_int
            != (*gms).player[playerI as usize].row as ::core::ffi::c_int
            || new_col as ::core::ffi::c_int
                != (*gms).player[playerI as usize].col as ::core::ffi::c_int
        {
            (*gms).player[playerI as usize].row = new_row;
            (*gms).player[playerI as usize].col = new_col;
            PlayMenuSound(MENU_SOUND_EFFECT::MENU_SOUND_MOVE);
            PickMelee_ChangedSelection(gms, playerI);
        }
    }
    return BOOLEAN::TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn selectShipHuman(
    mut context: *mut HumanInputContext,
    mut gms: *mut GETMELEE_STATE,
) -> BOOLEAN {
    let mut inputState: BATTLE_INPUT_STATE = PulsedInputToBattleInput(
        (*context).playerNr,
    );
    return SelectShip_processInput(gms, (*context).playerNr, inputState);
}
#[no_mangle]
pub unsafe extern "C" fn selectShipComputer(
    mut context: *mut ComputerInputContext,
    mut gms: *mut GETMELEE_STATE,
) -> BOOLEAN {
    let mut now: TimeCount = GetTimeCounter();
    if now
        < (*gms)
            .player[(*context).playerNr as usize]
            .timeIn
            .wrapping_add(
                (840 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as TimeCount,
            )
    {
        return BOOLEAN::TRUE;
    }
    return SelectShip_processInput(
        gms,
        (*context).playerNr,
        ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as BATTLE_INPUT_STATE,
    );
}
#[no_mangle]
pub unsafe extern "C" fn selectShipNetwork(
    mut context: *mut NetworkInputContext,
    mut gms: *mut GETMELEE_STATE,
) -> BOOLEAN {
    flushPacketQueues();
    if (*gms).player[(*context).playerNr as usize].remoteSelected.0 != 0 {
        (*gms).player[(*context).playerNr as usize].done = BOOLEAN::TRUE;
    }
    return BOOLEAN::TRUE;
}
unsafe extern "C" fn DoGetMelee(mut gms: *mut GETMELEE_STATE) -> BOOLEAN {
    let mut done: BOOLEAN = BOOLEAN::FALSE;
    let mut playerI: COUNT = 0;
    SetMenuSounds(
        0 as ::core::ffi::c_int as MENU_SOUND_FLAGS,
        0 as ::core::ffi::c_int as MENU_SOUND_FLAGS,
    );
    if (*gms).Initialized.0 == 0 {
        (*gms).Initialized = BOOLEAN::TRUE;
        return BOOLEAN::TRUE;
    }
    playerI = 0 as COUNT;
    while (playerI as ::core::ffi::c_int) < 2 as ::core::ffi::c_int {
        if (*gms).player[playerI as usize].selecting.0 != 0 {
            if (*gms).player[playerI as usize].done.0 == 0 {
                Flash_process((*gms).player[playerI as usize].flashContext);
            }
        }
        playerI = playerI.wrapping_add(1);
    }
    SleepThread((840 as ::core::ffi::c_int / 120 as ::core::ffi::c_int) as TimePeriod);
    netInput();
    '_aborted: {
        if allConnected() {
            if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
                & C2Rust_Unnamed_7::CHECK_ABORT.0 as ::core::ffi::c_int == 0
            {
                done = BOOLEAN::TRUE;
                playerI = 0 as COUNT;
                while (playerI as ::core::ffi::c_int) < 2 as ::core::ffi::c_int {
                    if (*gms).player[playerI as usize].selecting.0 != 0 {
                        if (*gms).player[playerI as usize].done.0 == 0 {
                            if (*(**(&raw mut PlayerInput as *mut *mut InputContext)
                                .offset(playerI as isize))
                                .handlers)
                                .selectShip
                                .expect(
                                    "non-null function pointer",
                                )(
                                    *(&raw mut PlayerInput as *mut *mut InputContext)
                                        .offset(playerI as isize),
                                    gms,
                                )
                                .0 == 0
                            {
                                break '_aborted;
                            }
                            if (*gms).player[playerI as usize].done.0 != 0 {
                                Flash_terminate(
                                    (*gms).player[playerI as usize].flashContext,
                                );
                                (*gms).player[playerI as usize].flashContext = ::core::ptr::null_mut::<
                                    FlashContext,
                                >();
                            } else {
                                done = BOOLEAN::FALSE;
                            }
                        }
                    }
                    playerI = playerI.wrapping_add(1);
                }
                flushPacketQueues();
                return BOOLEAN(
                    (done.0 == 0) as ::core::ffi::c_int as ::core::ffi::c_uint,
                );
            }
        }
    }
    flushPacketQueues();
    playerI = 0 as COUNT;
    while (playerI as ::core::ffi::c_int) < 2 as ::core::ffi::c_int {
        if (*gms).player[playerI as usize].selecting.0 != 0 {
            (*gms).player[playerI as usize].choice = 0 as COUNT;
            (*gms).player[playerI as usize].hBattleShip = ::core::ptr::null_mut::<
                ::core::ffi::c_void,
            >();
        }
        playerI = playerI.wrapping_add(1);
    }
    GlobData.Game_state.CurrentActivity = (GlobData.Game_state.CurrentActivity
        as ::core::ffi::c_int & !(C2Rust_Unnamed_7::CHECK_ABORT.0 as ::core::ffi::c_int))
        as ACTIVITY;
    return BOOLEAN::FALSE;
}
unsafe extern "C" fn GetRaceQueueValue(mut queue: *const QUEUE) -> COUNT {
    let mut result: COUNT = 0;
    let mut hBattleShip: HSTARSHIP = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut hNextShip: HSTARSHIP = ::core::ptr::null_mut::<::core::ffi::c_void>();
    result = 0 as COUNT;
    hBattleShip = (*queue).head as HSTARSHIP;
    while !hBattleShip.is_null() {
        let mut StarShipPtr: *mut STARSHIP = LockStarShip(queue, hBattleShip);
        hNextShip = (*StarShipPtr).succ as HSTARSHIP;
        if (*StarShipPtr).SpeciesID.0 != SPECIES_ID::NO_ID.0 {
            result = (result as ::core::ffi::c_int
                + (*StarShipPtr).ship_cost as ::core::ffi::c_int) as COUNT;
            UnlockLink(queue, hBattleShip);
        }
        hBattleShip = hNextShip;
    }
    return result;
}
unsafe extern "C" fn CrossOutShip(mut frame: FRAME, mut shipNr: COUNT) {
    let mut OldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    let mut s: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    let mut row: BYTE = PickMelee_GetShipRow(shipNr);
    let mut col: BYTE = PickMelee_GetShipColumn(shipNr as ::core::ffi::c_int);
    OldContext = SetContext(OffScreenContext);
    SetContextFGFrame(frame);
    s.origin.x = (3 as ::core::ffi::c_int
        + (16 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
            * col as ::core::ffi::c_int) as COORD;
    s.origin.y = (9 as ::core::ffi::c_int
        + (16 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
            * row as ::core::ffi::c_int) as COORD;
    s.frame = SetAbsFrameIndex(StatusFrame, 3 as COUNT);
    DrawStamp(&raw mut s);
    SetContext(OldContext);
}
unsafe extern "C" fn UpdatePickMeleeFleetValue(
    mut frame: FRAME,
    mut which_player: COUNT,
) {
    let mut OldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    let mut value: COUNT = 0;
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
    value = GetRaceQueueValue(
        (&raw mut race_q as *mut QUEUE).offset(which_player as isize),
    );
    OldContext = SetContext(OffScreenContext);
    SetContextFGFrame(frame);
    GetFrameRect(frame, &raw mut r);
    r.extent.width = (r.extent.width as ::core::ffi::c_int - 4 as ::core::ffi::c_int)
        as COORD;
    t.baseline.x = r.extent.width;
    r.corner.x = (r.extent.width as ::core::ffi::c_int
        - 6 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as COORD;
    r.corner.y = 2 as COORD;
    r.extent.width = (6 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as COORD;
    r.extent.height = (7 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as COORD;
    SetContextForeGroundColor(colorFromRgb15(0 as BYTE, 0x1 as BYTE, 0xf as BYTE));
    DrawFilledRectangle(&raw mut r);
    sprintf(
        &raw mut buf as *mut ::core::ffi::c_char,
        b"%d\0".as_ptr() as *const ::core::ffi::c_char,
        value as ::core::ffi::c_int,
    );
    t.baseline.y = 7 as COORD;
    t.align = TEXT_ALIGN::ALIGN_RIGHT;
    t.pStr = &raw mut buf as *mut UNICODE;
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    SetContextFont(TinyFont);
    SetContextForeGroundColor(colorFromRgb15(0x13 as BYTE, 0 as BYTE, 0 as BYTE));
    font_DrawText(&raw mut t);
    SetContext(OldContext);
}
#[no_mangle]
pub unsafe extern "C" fn BuildPickMeleeFrame() {
    let mut s: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    let mut OldContext: CONTEXT = SetContext(OffScreenContext);
    if !PickMeleeFrame.is_null() {
        DestroyDrawable(ReleaseDrawable(PickMeleeFrame));
    }
    PickMeleeFrame = CaptureDrawable(
        CreateDrawable(
            ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as CREATE_FLAGS,
            149 as SIZE,
            (48 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as SIZE,
            2 as COUNT,
        ),
    );
    s.origin.x = 0 as COORD;
    s.origin.y = 0 as COORD;
    s.frame = CaptureDrawable(
        LoadGraphicInstance(b"graphics.meleepickship\0".as_ptr() as RESOURCE) as DRAWABLE,
    );
    SetContextFGFrame(PickMeleeFrame);
    DrawStamp(&raw mut s);
    s.frame = IncFrameIndex(s.frame);
    SetContextFGFrame(IncFrameIndex(PickMeleeFrame));
    DrawStamp(&raw mut s);
    DestroyDrawable(ReleaseDrawable(s.frame));
    SetContext(OldContext);
}
#[no_mangle]
pub unsafe extern "C" fn FillPickMeleeFrame(mut setup: *mut MeleeSetup) {
    let mut i: COUNT = 0;
    let mut OldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    OldContext = SetContext(OffScreenContext);
    i = 0 as COUNT;
    while (i as ::core::ffi::c_int) < 2 as ::core::ffi::c_int {
        let mut side: COUNT = 0;
        let mut sideI: COUNT = 0;
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
        let mut s: STAMP = STAMP {
            origin: POINT { x: 0, y: 0 },
            frame: ::core::ptr::null_mut::<FRAME_DESC>(),
        };
        let mut buf: [UNICODE; 30] = [0; 30];
        let mut index: FleetShipIndex = 0;
        sideI = GetPlayerOrder(i);
        side = (sideI == 0) as ::core::ffi::c_int as COUNT;
        s.frame = SetAbsFrameIndex(PickMeleeFrame, side);
        SetContextFGFrame(s.frame);
        GetFrameRect(s.frame, &raw mut r);
        t.baseline.x = (r.extent.width as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
            as COORD;
        t.baseline.y = (r.extent.height as ::core::ffi::c_int - 7 as ::core::ffi::c_int
            + 4 as ::core::ffi::c_int) as COORD;
        r.corner.x = (r.corner.x as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
            as COORD;
        r.corner.y = (r.corner.y as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
            as COORD;
        r.extent.width = (r.extent.width as ::core::ffi::c_int
            - (2 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                + (16 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
                + 1 as ::core::ffi::c_int)) as COORD;
        r.extent.height = (r.extent.height as ::core::ffi::c_int
            - (2 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                + 7 as ::core::ffi::c_int)) as COORD;
        SetContextForeGroundColor(colorFromRgb15(0 as BYTE, 0x1 as BYTE, 0xf as BYTE));
        DrawFilledRectangle(&raw mut r);
        r.corner.x = (r.corner.x as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
            as COORD;
        r.extent.width = (r.extent.width as ::core::ffi::c_int
            + (16 as ::core::ffi::c_int + 2 as ::core::ffi::c_int
                - 2 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)) as COORD;
        r.corner.y = (r.corner.y as ::core::ffi::c_int
            + r.extent.height as ::core::ffi::c_int) as COORD;
        r.extent.height = 7 as COORD;
        DrawFilledRectangle(&raw mut r);
        t.align = TEXT_ALIGN::ALIGN_CENTER;
        t.pStr = MeleeSetup_getTeamName(setup, sideI as size_t) as *const UNICODE;
        t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
        SetContextFont(TinyFont);
        SetContextForeGroundColor(
            colorFromRgb15(0xa as BYTE, 0xa as BYTE, 0x1f as BYTE),
        );
        font_DrawText(&raw mut t);
        sprintf(
            &raw mut buf as *mut ::core::ffi::c_char,
            b"%u\0".as_ptr() as *const ::core::ffi::c_char,
            MeleeSetup_getFleetValue(setup, sideI as size_t) as ::core::ffi::c_int,
        );
        t.baseline.x = 4 as COORD;
        t.baseline.y = 7 as COORD;
        t.align = TEXT_ALIGN::ALIGN_LEFT;
        t.pStr = &raw mut buf as *mut UNICODE;
        t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
        SetContextForeGroundColor(
            colorFromRgb15(0x4 as BYTE, 0x5 as BYTE, 0x1f as BYTE),
        );
        font_DrawText(&raw mut t);
        (CountLinks((&raw mut race_q as *mut QUEUE).offset(side as isize))
            as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            || {
                __assert_fail(
                    b"CountLinks (&race_q[side]) == 0\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"src/uqm/supermelee/pickmele.c\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    567 as ::core::ffi::c_int,
                    b"FillPickMeleeFrame\0".as_ptr() as *const ::core::ffi::c_char,
                );
                0 as ::core::ffi::c_int != 0
            }) as ::core::ffi::c_int;
        index = 0 as FleetShipIndex;
        while (index as ::core::ffi::c_int)
            < 2 as ::core::ffi::c_int * 7 as ::core::ffi::c_int
        {
            let mut StarShip: MeleeShip = MeleeShip::MELEE_ANDROSYNTH;
            StarShip = MeleeSetup_getShip(setup, sideI as size_t, index);
            if StarShip.0 != MeleeShip::MELEE_NONE.0 {
                let mut row: BYTE = 0;
                let mut col: BYTE = 0;
                let mut ship_cost: BYTE = 0;
                let mut hMasterShip: HMASTERSHIP = ::core::ptr::null_mut::<
                    ::core::ffi::c_void,
                >();
                let mut hBuiltShip: HSTARSHIP = ::core::ptr::null_mut::<
                    ::core::ffi::c_void,
                >();
                let mut MasterPtr: *mut MASTER_SHIP_INFO = ::core::ptr::null_mut::<
                    MASTER_SHIP_INFO,
                >();
                let mut BuiltShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<
                    STARSHIP,
                >();
                let mut captains_name_index: BYTE = 0;
                hMasterShip = GetStarShipFromIndex(
                    &raw mut master_q,
                    StarShip.0 as COUNT,
                ) as HMASTERSHIP;
                MasterPtr = LockMasterShip(&raw mut master_q, hMasterShip);
                captains_name_index = NameCaptain(
                    (&raw mut race_q as *mut QUEUE).offset(side as isize),
                    (*MasterPtr).SpeciesID,
                );
                hBuiltShip = Build(
                    (&raw mut race_q as *mut QUEUE).offset(side as isize),
                    (*MasterPtr).SpeciesID,
                ) as HSTARSHIP;
                row = PickMelee_GetShipRow(index);
                col = PickMelee_GetShipColumn(index as ::core::ffi::c_int);
                s.origin.x = (4 as ::core::ffi::c_int
                    + (16 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
                        * col as ::core::ffi::c_int) as COORD;
                s.origin.y = (10 as ::core::ffi::c_int
                    + (16 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
                        * row as ::core::ffi::c_int) as COORD;
                s.frame = (*MasterPtr).ShipInfo.icons;
                DrawStamp(&raw mut s);
                ship_cost = (*MasterPtr).ShipInfo.ship_cost;
                UnlockLink(&raw mut master_q, hMasterShip);
                BuiltShipPtr = LockStarShip(
                    (&raw mut race_q as *mut QUEUE).offset(side as isize),
                    hBuiltShip,
                );
                (*BuiltShipPtr).index = index as COUNT;
                (*BuiltShipPtr).ship_cost = ship_cost;
                (*BuiltShipPtr).playerNr = side as SIZE;
                (*BuiltShipPtr).captains_name_index = captains_name_index;
                (*BuiltShipPtr).crew_level = 0 as COUNT;
                (*BuiltShipPtr).max_crew = 0 as COUNT;
                (*BuiltShipPtr).race_strings = ::core::ptr::null_mut::<
                    STRING_TABLE_ENTRY_DESC,
                >();
                (*BuiltShipPtr).icons = ::core::ptr::null_mut::<FRAME_DESC>();
                (*BuiltShipPtr).RaceDescPtr = ::core::ptr::null_mut::<RACE_DESC>();
                UnlockLink(
                    (&raw mut race_q as *mut QUEUE).offset(side as isize),
                    hBuiltShip,
                );
            }
            index = index.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    SetContext(OldContext);
}
#[no_mangle]
pub unsafe extern "C" fn DestroyPickMeleeFrame() {
    DestroyDrawable(ReleaseDrawable(PickMeleeFrame));
    PickMeleeFrame = ::core::ptr::null_mut::<FRAME_DESC>();
}
unsafe extern "C" fn DrawPickMeleeFrame(mut which_player: COUNT) {
    let mut oldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    let mut s: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    oldContext = SetContext(SpaceContext);
    s.frame = SetAbsFrameIndex(PickMeleeFrame, which_player);
    s.origin.x = (57 as ::core::ffi::c_int - 3 as ::core::ffi::c_int) as COORD;
    s.origin.y = (24 as ::core::ffi::c_int - 9 as ::core::ffi::c_int
        + (1 as ::core::ffi::c_int - which_player as ::core::ffi::c_int)
            * 100 as ::core::ffi::c_int) as COORD;
    DrawStamp(&raw mut s);
    SetContext(oldContext);
}
#[no_mangle]
pub unsafe extern "C" fn MeleeGameOver() {
    let mut playerI: COUNT = 0;
    let mut TimeOut: DWORD = 0;
    let mut PressState: BOOLEAN = BOOLEAN::FALSE;
    let mut ButtonState: BOOLEAN = BOOLEAN::FALSE;
    playerI = 0 as COUNT;
    while (playerI as ::core::ffi::c_int) < 2 as ::core::ffi::c_int {
        DrawPickMeleeFrame(playerI);
        playerI = playerI.wrapping_add(1);
    }
    if !negotiateReadyConnections(true, NetState::NetState_inSetup) {
        GlobData.Game_state.CurrentActivity = (GlobData.Game_state.CurrentActivity
            as ::core::ffi::c_int
            | C2Rust_Unnamed_7::CHECK_ABORT.0 as ::core::ffi::c_int) as ACTIVITY;
        return;
    }
    TimeOut = GetTimeCounter()
        .wrapping_add((840 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as TimeCount)
        as DWORD;
    PressState = BOOLEAN(
        (PulsedInputState
            .menu[C2Rust_Unnamed_6::KEY_MENU_SELECT.0 as ::core::ffi::c_int as usize]
            != 0
            || PulsedInputState
                .menu[C2Rust_Unnamed_6::KEY_MENU_CANCEL.0 as ::core::ffi::c_int as usize]
                != 0) as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
    loop {
        UpdateInputState();
        ButtonState = BOOLEAN(
            (PulsedInputState
                .menu[C2Rust_Unnamed_6::KEY_MENU_SELECT.0 as ::core::ffi::c_int as usize]
                != 0
                || PulsedInputState
                    .menu[C2Rust_Unnamed_6::KEY_MENU_CANCEL.0 as ::core::ffi::c_int
                    as usize] != 0) as ::core::ffi::c_int as ::core::ffi::c_uint,
        );
        if PressState.0 != 0 {
            PressState = ButtonState;
            ButtonState = BOOLEAN::FALSE;
        }
        Async_process();
        TaskSwitch();
        if !(GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
            & C2Rust_Unnamed_7::CHECK_ABORT.0 as ::core::ffi::c_int == 0
            && (ButtonState.0 == 0
                && (*(&raw mut PlayerControl as *mut BYTE).offset(0isize)
                    as ::core::ffi::c_int
                    & *(&raw mut PlayerControl as *mut BYTE).offset(1isize)
                        as ::core::ffi::c_int
                    & ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as BYTE
                        as ::core::ffi::c_int == 0 || GetTimeCounter() < TimeOut)))
        {
            break;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn MeleeShipDeath(mut ship: *mut STARSHIP) {
    let mut frame: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
    (*ship).SpeciesID = SPECIES_ID::NO_ID;
    frame = SetAbsFrameIndex(PickMeleeFrame, (*ship).playerNr as COUNT);
    CrossOutShip(frame, (*ship).index);
    UpdatePickMeleeFleetValue(frame, (*ship).playerNr as COUNT);
}
unsafe extern "C" fn GetMeleeStarShips(
    mut playerMask: COUNT,
    mut ships: *mut HSTARSHIP,
) -> BOOLEAN {
    let mut playerI: COUNT = 0;
    let mut ok: BOOLEAN = BOOLEAN::FALSE;
    let mut gmstate: GETMELEE_STATE = GETMELEE_STATE {
        InputFunc: None,
        Initialized: BOOLEAN::FALSE,
        player: [C2Rust_Unnamed {
            timeIn: 0,
            hBattleShip: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            choice: 0,
            row: 0,
            col: 0,
            ships_left: 0,
            randomIndex: 0,
            selecting: BOOLEAN::FALSE,
            done: BOOLEAN::FALSE,
            flashContext: ::core::ptr::null_mut::<FlashContext>(),
            remoteSelected: BOOLEAN::FALSE,
        }; 2],
    };
    let mut now: TimeCount = 0;
    let mut i: COUNT = 0;
    playerI = 0 as COUNT;
    while (playerI as ::core::ffi::c_int) < 2 as ::core::ffi::c_int {
        let mut conn: *mut NetConnection = ::core::ptr::null_mut::<NetConnection>();
        if playerMask as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << playerI as ::core::ffi::c_int
            != 0 as ::core::ffi::c_int
        {
            conn = *(&raw mut netConnections as *mut *mut NetConnection)
                .offset(playerI as isize) as *mut NetConnection;
            if !conn.is_null() {
                let mut battleStateData: *mut BattleStateData = ::core::ptr::null_mut::<
                    BattleStateData,
                >();
                battleStateData = NetConnection_getStateData(conn)
                    as *mut BattleStateData;
                (*battleStateData).getMeleeState = &raw mut gmstate
                    as *mut getmelee_struct;
            }
        }
        playerI = playerI.wrapping_add(1);
    }
    ok = BOOLEAN(1 as ::core::ffi::c_int as ::core::ffi::c_uint);
    now = GetTimeCounter();
    gmstate.InputFunc = Some(
        DoGetMelee as unsafe extern "C" fn(*mut GETMELEE_STATE) -> BOOLEAN,
    ) as Option<unsafe extern "C" fn(*mut getmelee_struct) -> BOOLEAN>;
    gmstate.Initialized = BOOLEAN::FALSE;
    i = 0 as COUNT;
    while (i as ::core::ffi::c_int) < 2 as ::core::ffi::c_int {
        playerI = GetPlayerOrder(i);
        gmstate.player[playerI as usize].selecting = BOOLEAN(
            (playerMask as ::core::ffi::c_int
                & (1 as ::core::ffi::c_int) << playerI as ::core::ffi::c_int
                != 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_uint,
        );
        gmstate.player[playerI as usize].ships_left = battle_counter[playerI as usize]
            as COUNT;
        gmstate.player[playerI as usize].randomIndex = (TFB_Random() as COUNT
            as ::core::ffi::c_int
            % gmstate.player[playerI as usize].ships_left as ::core::ffi::c_int)
            as COUNT;
        gmstate.player[playerI as usize].done = BOOLEAN::FALSE;
        if gmstate.player[playerI as usize].selecting.0 != 0 {
            gmstate.player[playerI as usize].timeIn = now;
            gmstate.player[playerI as usize].row = 0 as COUNT;
            gmstate.player[playerI as usize].col = 7 as COUNT;
            gmstate.player[playerI as usize].remoteSelected = BOOLEAN::FALSE;
            gmstate.player[playerI as usize].flashContext = Flash_createHighlight(
                ScreenContext,
                ::core::ptr::null::<RECT>(),
            );
            Flash_setMergeFactors(
                gmstate.player[playerI as usize].flashContext,
                2 as ::core::ffi::c_int,
                3 as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
            );
            Flash_setFrameTime(
                gmstate.player[playerI as usize].flashContext,
                (840 as ::core::ffi::c_int / 16 as ::core::ffi::c_int) as TimeCount,
            );
            if *(&raw mut PlayerControl as *mut BYTE).offset(playerI as isize)
                as ::core::ffi::c_int
                & ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as BYTE
                    as ::core::ffi::c_int != 0
            {
                Flash_setSpeed(
                    gmstate.player[playerI as usize].flashContext,
                    (840 as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as TimeCount,
                    0 as TimeCount,
                    (840 as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as TimeCount,
                    0 as TimeCount,
                );
            } else {
                Flash_setSpeed(
                    gmstate.player[playerI as usize].flashContext,
                    0 as TimeCount,
                    (840 as ::core::ffi::c_int / 16 as ::core::ffi::c_int) as TimeCount,
                    0 as TimeCount,
                    (840 as ::core::ffi::c_int / 16 as ::core::ffi::c_int) as TimeCount,
                );
            }
            PickMelee_ChangedSelection(&raw mut gmstate, playerI);
            Flash_start(gmstate.player[playerI as usize].flashContext);
        }
        i = i.wrapping_add(1);
    }
    let mut allOk: bool = negotiateReadyConnections(true, NetState::NetState_selectShip);
    if !allOk {
        ok = BOOLEAN(0 as ::core::ffi::c_int as ::core::ffi::c_uint);
    }
    SetDefaultMenuRepeatDelay();
    SetContext(OffScreenContext);
    if ok.0 != 0 {
        DoInput(&raw mut gmstate as *mut ::core::ffi::c_void, BOOLEAN::FALSE);
    }
    WaitForSoundEnd(0 as COUNT);
    playerI = 0 as COUNT;
    while (playerI as ::core::ffi::c_int) < 2 as ::core::ffi::c_int {
        if gmstate.player[playerI as usize].selecting.0 != 0 {
            if gmstate.player[playerI as usize].done.0 != 0 {
                *ships.offset(playerI as isize) = gmstate
                    .player[playerI as usize]
                    .hBattleShip;
            } else {
                Flash_terminate(gmstate.player[playerI as usize].flashContext);
                gmstate.player[playerI as usize].flashContext = ::core::ptr::null_mut::<
                    FlashContext,
                >();
                ok = BOOLEAN(0 as ::core::ffi::c_int as ::core::ffi::c_uint);
            }
        }
        playerI = playerI.wrapping_add(1);
    }
    if ok.0 != 0 {
        if !negotiateReadyConnections(true, NetState::NetState_interBattle) {
            ok = BOOLEAN(0 as ::core::ffi::c_int as ::core::ffi::c_uint);
        }
    } else {
        setStateConnections(NetState::NetState_interBattle);
    }
    if ok.0 == 0 {
        GlobData.Game_state.CurrentActivity = (GlobData.Game_state.CurrentActivity
            as ::core::ffi::c_int
            & !(C2Rust_Unnamed_7::IN_BATTLE.0 as ::core::ffi::c_int)) as ACTIVITY;
    }
    playerI = 0 as COUNT;
    while (playerI as ::core::ffi::c_int) < 2 as ::core::ffi::c_int {
        let mut conn_0: *mut NetConnection = ::core::ptr::null_mut::<NetConnection>();
        if playerMask as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << playerI as ::core::ffi::c_int
            != 0 as ::core::ffi::c_int
        {
            conn_0 = *(&raw mut netConnections as *mut *mut NetConnection)
                .offset(playerI as isize) as *mut NetConnection;
            if !conn_0.is_null()
                && NetConnection_isConnected(conn_0) as ::core::ffi::c_int != 0
            {
                let mut battleStateData_0: *mut BattleStateData = ::core::ptr::null_mut::<
                    BattleStateData,
                >();
                battleStateData_0 = NetConnection_getStateData(conn_0)
                    as *mut BattleStateData;
                (*battleStateData_0).getMeleeState = ::core::ptr::null_mut::<
                    getmelee_struct,
                >();
            }
        }
        playerI = playerI.wrapping_add(1);
    }
    return ok;
}
#[no_mangle]
pub unsafe extern "C" fn GetInitialMeleeStarShips(
    mut result: *mut HSTARSHIP,
) -> BOOLEAN {
    let mut playerI: COUNT = 0;
    let mut playerMask: COUNT = 0;
    playerI = 0 as COUNT;
    while (playerI as ::core::ffi::c_int) < 2 as ::core::ffi::c_int {
        let mut frame: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
        frame = SetAbsFrameIndex(PickMeleeFrame, playerI);
        UpdatePickMeleeFleetValue(frame, playerI);
        DrawPickMeleeFrame(playerI);
        playerI = playerI.wrapping_add(1);
    }
    SleepThreadUntil(
        FadeScreen(
                ScreenFadeType::FadeAllToColor,
                (840 as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as SIZE,
            )
            .wrapping_add(
                (840 as ::core::ffi::c_int / 60 as ::core::ffi::c_int) as TimeCount,
            ),
    );
    FlushColorXForms();
    playerMask = 0 as COUNT;
    playerI = 0 as COUNT;
    while (playerI as ::core::ffi::c_int) < 2 as ::core::ffi::c_int {
        playerMask = (playerMask as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << playerI as ::core::ffi::c_int) as COUNT;
        playerI = playerI.wrapping_add(1);
    }
    return GetMeleeStarShips(playerMask, result);
}
#[no_mangle]
pub unsafe extern "C" fn GetNextMeleeStarShip(
    mut which_player: COUNT,
    mut result: *mut HSTARSHIP,
) -> BOOLEAN {
    let mut playerMask: COUNT = 0;
    let mut ships: [HSTARSHIP; 2] = [::core::ptr::null_mut::<::core::ffi::c_void>(); 2];
    let mut ok: BOOLEAN = BOOLEAN::FALSE;
    DrawPickMeleeFrame(which_player);
    playerMask = ((1 as ::core::ffi::c_int) << which_player as ::core::ffi::c_int)
        as COUNT;
    ok = GetMeleeStarShips(playerMask, &raw mut ships as *mut HSTARSHIP);
    if ok.0 != 0 {
        *result = ships[which_player as usize];
    }
    return ok;
}
#[no_mangle]
pub unsafe extern "C" fn updateMeleeSelection(
    mut gms: *mut GETMELEE_STATE,
    mut playerI: COUNT,
    mut ship: COUNT,
) -> bool {
    if gms.is_null() || (*gms).player[playerI as usize].selecting.0 == 0
        || (*gms).player[playerI as usize].done.0 != 0
    {
        log_add(
            log_Level::log_Warning,
            b"Unexpected ship selection packet received.\n\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return false;
    }
    if !setShipSelected(gms, playerI, ship, false) {
        log_add(
            log_Level::log_Warning,
            b"Invalid ship selection received from remote party.\n\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return false;
    }
    (*gms).player[playerI as usize].remoteSelected = BOOLEAN::TRUE;
    return true;
}
unsafe extern "C" fn reportShipSelected(mut gms: *mut GETMELEE_STATE, mut index: COUNT) {
    let mut playerI: size_t = 0;
    playerI = 0 as size_t;
    while playerI < 2 as size_t {
        let mut conn: *mut NetConnection = *(&raw mut netConnections
            as *mut *mut NetConnection)
            .offset(playerI as isize);
        if !conn.is_null() {
            if NetConnection_isConnected(conn) {
                Netplay_Notify_shipSelected(conn, index);
            }
        }
        playerI = playerI.wrapping_add(1);
    }
}
#[inline]
unsafe extern "C" fn LockMasterShip(
    mut pq: *const QUEUE,
    mut h: HMASTERSHIP,
) -> *mut MASTER_SHIP_INFO {
    ((*pq).object_size as usize == ::core::mem::size_of::<MASTER_SHIP_INFO>()
        || {
            __assert_fail(
                b"GetLinkSize (pq) == sizeof (MASTER_SHIP_INFO)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/uqm/supermelee/../master.h\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                50 as ::core::ffi::c_int,
                b"LockMasterShip\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    return LockLink(pq, h) as *mut MASTER_SHIP_INFO;
}
