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

// Rust translation of src/uqm/starcon.c.
extern "C" {
    pub type frame_desc;
    fn rand() -> ::core::ffi::c_int;
    fn srand(_: ::core::ffi::c_uint);
    fn abort() -> !;
    fn exit(_: ::core::ffi::c_int) -> !;
    fn GetTimeCounter() -> TimeCount;
    fn TaskSwitch();
    fn InitGameClock() -> BOOLEAN;
    fn UninitGameClock() -> BOOLEAN;
    fn SetGameClockRate(seconds_per_day: COUNT);
    fn GameClockTick();
    fn StopSound();
    fn memset(
        _: *mut ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn printf(_: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn time(_: *mut time_t) -> time_t;
    fn localtime(_: *const time_t) -> *mut tm;
    static mut ImmediateInputState: CONTROLLER_INPUT_STATE;
    fn UpdateInputState();
    fn FlushInput();
    static mut GamePaused: BOOLEAN;
    static mut resolutionFactor: ::core::ffi::c_uint;
    static mut optSkipIntro: OPT_ENABLABLE;
    static mut optSeedType: ::core::ffi::c_int;
    static mut optCustomSeed: ::core::ffi::c_int;
    static mut optDifficulty: ::core::ffi::c_int;
    static mut optExtended: OPT_ENABLABLE;
    static mut optNomad: ::core::ffi::c_int;
    static mut optGameOver: OPT_ENABLABLE;
    static mut optMaskOfDeceit: BOOLEAN;
    static mut baseContentPath: [::core::ffi::c_char; 4096];
    fn SetFlashRect(pRect: *const RECT, purple: BOOLEAN);
    fn SetStatusMessageMode(_: StatMsgMode) -> StatMsgMode;
    fn DrawAutoPilotMessage(Reset: BOOLEAN);
    fn luaUqm_initState();
    fn luaUqm_uninitState();
    fn luaUqm_reinitState();
    fn setGameStateUint(name: *const ::core::ffi::c_char, val: DWORD);
    fn getGameStateUint(name: *const ::core::ffi::c_char) -> DWORD;
    static mut GlobData: GLOBDATA;
    fn InitGameStructures() -> BOOLEAN;
    fn UninitGameStructures();
    fn InitCommunication(which_comm: CONVERSATION) -> COUNT;
    fn RaceCommunication();
    fn Battle(_: Option<BattleFrameCallback>) -> BOOLEAN;
    fn SplashScreen(DoProcessing: Option<unsafe extern "C" fn(DWORD) -> ()>);
    fn GameOver(DeathType: BYTE);
    fn Logo();
    fn InitMenuMusic();
    fn initEventSystem();
    fn uninitEventSystem();
    fn AddInitialGameEvents();
    fn StartGame() -> BOOLEAN;
    fn InstallBombAtEarth();
    fn VisitStarBase();
    static mut NextActivity: ACTIVITY;
    static mut Rando: BYTE;
    static mut DeathBySurrender: BOOLEAN;
    static mut LastActivity: ACTIVITY;
    fn InitGameKernel() -> BOOLEAN;
    fn UninitGameKernel();
    fn LoadKernel(
        argc: ::core::ffi::c_int,
        argv: *mut *mut ::core::ffi::c_char,
    ) -> BOOLEAN;
    fn FreeKernel();
    fn SetPlayerInputAll() -> bool;
    fn ClearPlayerInputAll();
    fn LoadMasterShipList(YieldProcessing: Option<unsafe extern "C" fn() -> ()>);
    fn FreeMasterShipList();
    static mut soundflags: ::core::ffi::c_int;
    static mut snddriver: ::core::ffi::c_int;
    fn SeedUniverse();
    fn ExploreSolarSys();
    static mut debugHook: Option<unsafe extern "C" fn() -> ()>;
    static mut DebugKeyPressed: BOOLEAN;
    fn debugKeyPressed();
    fn debugKey2Pressed();
    fn debugKey3Pressed();
    fn debugKey4Pressed();
    fn log_showBox(show: bool, err: bool);
    fn log_add(_: log_Level, fmt: *const ::core::ffi::c_char, ...);
    static mut GfxFlags: ::core::ffi::c_int;
    static mut WindowWidth: ::core::ffi::c_int;
    static mut WindowHeight: ::core::ffi::c_int;
    static mut GraphicsDriver: ::core::ffi::c_int;
    fn TFB_ScreenShot();
    fn TFB_DrawScreen_ReinitVideo(
        driver: ::core::ffi::c_int,
        flags: ::core::ffi::c_int,
        width: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
    );
    fn QuickLoadDeferred() -> BOOLEAN;
}
pub type size_t = ::core::ffi::c_ulong;
pub type time_t = ::core::ffi::c_longlong;
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
pub type TimeCount = DWORD;
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
pub type FRAME_DESC = frame_desc;
pub type FRAME = *mut FRAME_DESC;
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
pub struct tm {
    pub tm_sec: ::core::ffi::c_int,
    pub tm_min: ::core::ffi::c_int,
    pub tm_hour: ::core::ffi::c_int,
    pub tm_mday: ::core::ffi::c_int,
    pub tm_mon: ::core::ffi::c_int,
    pub tm_year: ::core::ffi::c_int,
    pub tm_wday: ::core::ffi::c_int,
    pub tm_yday: ::core::ffi::c_int,
    pub tm_isdst: ::core::ffi::c_int,
    pub tm_gmtoff: ::core::ffi::c_long,
    pub tm_zone: *const ::core::ffi::c_char,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed {
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
    pub const KEY_MENU_TOGGLEMAP: Self = Self(23);
    pub const KEY_SCREENSHOT: Self = Self(24);
    pub const KEY_QUICKSAVE: Self = Self(25);
    pub const KEY_QUICKLOAD: Self = Self(26);
    pub const KEY_DEBUG_2: Self = Self(27);
    pub const KEY_DEBUG_3: Self = Self(28);
    pub const KEY_DEBUG_4: Self = Self(29);
    pub const KEY_MENU_ANY: Self = Self(30);
    pub const NUM_MENU_KEYS: Self = Self(31);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _controller_input_state {
    pub key: [[::core::ffi::c_int; 8]; 2],
    pub menu: [::core::ffi::c_int; 31],
}
pub type CONTROLLER_INPUT_STATE = _controller_input_state;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct OPT_ENABLABLE(pub ::core::ffi::c_uint);
impl OPT_ENABLABLE {
    pub const OPTVAL_DISABLED: Self = Self(0);
    pub const OPTVAL_ENABLED: Self = Self(1);
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
pub struct StatMsgMode(pub ::core::ffi::c_uint);
impl StatMsgMode {
    pub const SMM_UNDEFINED: Self = Self(0);
    pub const SMM_DATE: Self = Self(1);
    pub const SMM_RES_UNITS: Self = Self(2);
    pub const SMM_CREDITS: Self = Self(3);
    pub const SMM_WARNING: Self = Self(4);
    pub const SMM_ALERT: Self = Self(5);
    pub const SMM_DEFAULT: Self = Self(1);
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct CONVERSATION(pub ::core::ffi::c_uint);
impl CONVERSATION {
    pub const ARILOU_CONVERSATION: Self = Self(0);
    pub const CHMMR_CONVERSATION: Self = Self(1);
    pub const COMMANDER_CONVERSATION: Self = Self(2);
    pub const ORZ_CONVERSATION: Self = Self(3);
    pub const PKUNK_CONVERSATION: Self = Self(4);
    pub const SHOFIXTI_CONVERSATION: Self = Self(5);
    pub const SPATHI_CONVERSATION: Self = Self(6);
    pub const SUPOX_CONVERSATION: Self = Self(7);
    pub const THRADD_CONVERSATION: Self = Self(8);
    pub const UTWIG_CONVERSATION: Self = Self(9);
    pub const VUX_CONVERSATION: Self = Self(10);
    pub const YEHAT_CONVERSATION: Self = Self(11);
    pub const MELNORME_CONVERSATION: Self = Self(12);
    pub const DRUUGE_CONVERSATION: Self = Self(13);
    pub const ILWRATH_CONVERSATION: Self = Self(14);
    pub const MYCON_CONVERSATION: Self = Self(15);
    pub const SLYLANDRO_CONVERSATION: Self = Self(16);
    pub const UMGAH_CONVERSATION: Self = Self(17);
    pub const URQUAN_CONVERSATION: Self = Self(18);
    pub const ZOQFOTPIK_CONVERSATION: Self = Self(19);
    pub const SYREEN_CONVERSATION: Self = Self(20);
    pub const BLACKURQ_CONVERSATION: Self = Self(21);
    pub const TALKING_PET_CONVERSATION: Self = Self(22);
    pub const SLYLANDRO_HOME_CONVERSATION: Self = Self(23);
    pub const URQUAN_DRONE_CONVERSATION: Self = Self(24);
    pub const YEHAT_REBEL_CONVERSATION: Self = Self(25);
    pub const INVALID_CONVERSATION: Self = Self(26);
    pub const NUM_CONVERSATIONS: Self = Self(27);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_0 {
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
pub type BattleFrameCallback = unsafe extern "C" fn() -> ();
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_1(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_1 {
    pub const DEATH_MARCH: Self = Self(0);
    pub const DIED_IN_BATTLE: Self = Self(1);
    pub const SUICIDE: Self = Self(2);
    pub const SURRENDERED: Self = Self(3);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_2(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_2 {
    pub const CLOSING: Self = Self(0);
    pub const OPENING: Self = Self(1);
}
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
#[no_mangle]
pub static mut MainExited: ::core::ffi::c_int = BOOLEAN::FALSE.0 as ::core::ffi::c_int;
unsafe extern "C" fn checkArilouGate() {
    let mut counter: BYTE = 0;
    counter = uqm_lua_checked!(getGameStateUint(
        b"ARILOU_SPACE_COUNTER\0".as_ptr() as *const ::core::ffi::c_char,
    ), ()) as BYTE;
    if uqm_lua_checked!(getGameStateUint(b"ARILOU_SPACE\0".as_ptr() as *const ::core::ffi::c_char), ())
        == C2Rust_Unnamed_2::OPENING.0 as ::core::ffi::c_int as DWORD
    {
        if (counter as ::core::ffi::c_int) < 9 as ::core::ffi::c_int {
            counter = counter.wrapping_add(1);
        }
    } else if counter as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
        counter = counter.wrapping_sub(1);
    }
    uqm_lua_checked!(setGameStateUint(
        b"ARILOU_SPACE_COUNTER\0".as_ptr() as *const ::core::ffi::c_char,
        counter as DWORD,
    ), ());
}
unsafe extern "C" fn on_battle_frame() {
    uqm_lua_checked!(GameClockTick(), ());
    uqm_lua_checked!(checkArilouGate(), ());
    if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
        & (C2Rust_Unnamed_0::CHECK_ABORT.0 as ::core::ffi::c_int
            | C2Rust_Unnamed_0::CHECK_LOAD.0 as ::core::ffi::c_int) == 0
    {
        uqm_lua_checked!(SeedUniverse(), ());
    }
    uqm_lua_checked!(DrawAutoPilotMessage(BOOLEAN::FALSE), ());
}
unsafe extern "C" fn BackgroundInitKernel(mut TimeOut: DWORD) {
    uqm_lua_checked!(LoadMasterShipList(Some(TaskSwitch as unsafe extern "C" fn() -> ())), ());
    TaskSwitch();
    uqm_lua_checked!(InitGameKernel(), ());
    while GetTimeCounter() <= TimeOut
        && GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
            & C2Rust_Unnamed_0::CHECK_ABORT.0 as ::core::ffi::c_int == 0
    {
        uqm_lua_checked!(UpdateInputState(), ());
        TaskSwitch();
    }
}
#[no_mangle]
pub unsafe extern "C" fn SignalStopMainThread() {
    ::core::ptr::write_volatile(&raw mut GamePaused, BOOLEAN::FALSE);
    GlobData.Game_state.CurrentActivity = (GlobData.Game_state.CurrentActivity
        as ::core::ffi::c_int | C2Rust_Unnamed_0::CHECK_ABORT.0 as ::core::ffi::c_int)
        as ACTIVITY;
    TaskSwitch();
}
#[no_mangle]
pub unsafe extern "C" fn ProcessUtilityKeys() {
    if ::core::ptr::read_volatile::<
        ::core::ffi::c_int,
    >(
        &raw const ImmediateInputState
            .menu[C2Rust_Unnamed::KEY_ABORT.0 as ::core::ffi::c_int as usize],
    ) != 0
    {
        log_showBox(false, false);
        exit(0 as ::core::ffi::c_int);
    }
    if ::core::ptr::read_volatile::<
        ::core::ffi::c_int,
    >(
        &raw const ImmediateInputState
            .menu[C2Rust_Unnamed::KEY_FULLSCREEN.0 as ::core::ffi::c_int as usize],
    ) != 0
    {
        let mut flags: ::core::ffi::c_int = GfxFlags;
        if flags & (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int != 0 {
            flags &= !((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int);
            flags &= !((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int);
        } else {
            flags ^= (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int;
        }
        if if resolutionFactor != 2 as ::core::ffi::c_uint {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } != 0
        {
            flags ^= (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
        }
        FlushInput();
        TFB_DrawScreen_ReinitVideo(GraphicsDriver, flags, WindowWidth, WindowHeight);
    }
    if ::core::ptr::read_volatile::<
        ::core::ffi::c_int,
    >(
        &raw const ImmediateInputState
            .menu[C2Rust_Unnamed::KEY_SCREENSHOT.0 as ::core::ffi::c_int as usize],
    ) != 0
    {
        FlushInput();
        TFB_ScreenShot();
    }
    static mut debugKeyState: ::core::ffi::c_int = 0;
    static mut debugKey2State: ::core::ffi::c_int = 0;
    static mut debugKey3State: ::core::ffi::c_int = 0;
    static mut debugKey4State: ::core::ffi::c_int = 0;
    if ::core::ptr::read_volatile::<
        ::core::ffi::c_int,
    >(
        &raw const ImmediateInputState
            .menu[C2Rust_Unnamed::KEY_DEBUG.0 as ::core::ffi::c_int as usize],
    ) != 0 && debugKeyState == 0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(debugKeyPressed(), ());
    }
    debugKeyState = ::core::ptr::read_volatile::<
        ::core::ffi::c_int,
    >(
        &raw const ImmediateInputState
            .menu[C2Rust_Unnamed::KEY_DEBUG.0 as ::core::ffi::c_int as usize],
    );
    if ::core::ptr::read_volatile::<
        ::core::ffi::c_int,
    >(
        &raw const ImmediateInputState
            .menu[C2Rust_Unnamed::KEY_DEBUG_2.0 as ::core::ffi::c_int as usize],
    ) != 0 && debugKey2State == 0 as ::core::ffi::c_int
    {
        debugKey2Pressed();
    }
    debugKey2State = ::core::ptr::read_volatile::<
        ::core::ffi::c_int,
    >(
        &raw const ImmediateInputState
            .menu[C2Rust_Unnamed::KEY_DEBUG_2.0 as ::core::ffi::c_int as usize],
    );
    if ::core::ptr::read_volatile::<
        ::core::ffi::c_int,
    >(
        &raw const ImmediateInputState
            .menu[C2Rust_Unnamed::KEY_DEBUG_3.0 as ::core::ffi::c_int as usize],
    ) != 0 && debugKey3State == 0 as ::core::ffi::c_int
    {
        debugKey3Pressed();
    }
    debugKey3State = ::core::ptr::read_volatile::<
        ::core::ffi::c_int,
    >(
        &raw const ImmediateInputState
            .menu[C2Rust_Unnamed::KEY_DEBUG_3.0 as ::core::ffi::c_int as usize],
    );
    if ::core::ptr::read_volatile::<
        ::core::ffi::c_int,
    >(
        &raw const ImmediateInputState
            .menu[C2Rust_Unnamed::KEY_DEBUG_4.0 as ::core::ffi::c_int as usize],
    ) != 0 && debugKey4State == 0 as ::core::ffi::c_int
    {
        debugKey4Pressed();
    }
    debugKey4State = ::core::ptr::read_volatile::<
        ::core::ffi::c_int,
    >(
        &raw const ImmediateInputState
            .menu[C2Rust_Unnamed::KEY_DEBUG_4.0 as ::core::ffi::c_int as usize],
    );
}
unsafe extern "C" fn SetRandomMenuMusic() {
    let mut t: time_t = time(::core::ptr::null_mut::<time_t>());
    let mut tm: tm = *localtime(&raw mut t);
    srand(t as ::core::ffi::c_uint);
    Rando = (rand() % 3 as ::core::ffi::c_int) as BYTE;
    optMaskOfDeceit = BOOLEAN(
        (tm.tm_mon == 3 as ::core::ffi::c_int && tm.tm_mday == 1 as ::core::ffi::c_int)
            as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
}
#[no_mangle]
pub unsafe extern "C" fn Starcon2Main(
    mut threadArg: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    extern "C" {
        #[link_name = "initAudio"]
        fn initAudio_0(driver: sint32, flags: sint32) -> sint32;
    }
    uqm_lua_checked!(initAudio_0(snddriver, soundflags), 0);
    if uqm_lua_checked!(LoadKernel(
            0 as ::core::ffi::c_int,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        ), 0)
        .0 == 0
    {
        log_add(
            log_Level::log_Fatal,
            b"\n  *** FATAL ERROR: Could not load basic content ***\n\nUQM requires at least the base content pack to run properly.\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        log_add(
            log_Level::log_Fatal,
            b"This file is typically called mm-%d.%d.%d-content.uqm.  UQM was expecting\0"
                .as_ptr() as *const ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
            8 as ::core::ffi::c_int,
            5 as ::core::ffi::c_int,
        );
        log_add(
            log_Level::log_Fatal,
            b"it in the %s/packages directory.\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut baseContentPath as *mut ::core::ffi::c_char,
        );
        log_add(
            log_Level::log_Fatal,
            b"Either your installation did not install the content pack at all, or it\ninstalled it in a different directory.\n\nFix your installation and rerun UQM.\n\n  *******************\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        log_showBox(true, true);
        ::core::ptr::write_volatile(
            &raw mut MainExited,
            BOOLEAN::TRUE.0 as ::core::ffi::c_int,
        );
        return 1 as ::core::ffi::c_int;
    }
    log_add(
        log_Level::log_Info,
        b"We've loaded the Kernel\0".as_ptr() as *const ::core::ffi::c_char,
    );
    GlobData.Game_state.CurrentActivity = 0 as ACTIVITY;
    uqm_lua_checked!(luaUqm_initState(), 0);
    if optSkipIntro.0 == 0 {
        uqm_lua_checked!(Logo(), 0);
    }
    SetRandomMenuMusic();
    uqm_lua_checked!(InitMenuMusic(), 0);
    uqm_lua_checked!(SplashScreen(Some(BackgroundInitKernel as unsafe extern "C" fn(DWORD) -> ())), 0);
    printf(b"Set Seed Type: %s\n\0".as_ptr() as *const ::core::ffi::c_char, SeedStr());
    printf(b"Set Seed: %d\n\0".as_ptr() as *const ::core::ffi::c_char, optCustomSeed);
    printf(
        b"Set Difficulty: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
        if optDifficulty == 0 as ::core::ffi::c_int {
            b"Normal\0".as_ptr() as *const ::core::ffi::c_char
        } else if optDifficulty == 1 as ::core::ffi::c_int {
            b"Easy\0".as_ptr() as *const ::core::ffi::c_char
        } else if optDifficulty == 2 as ::core::ffi::c_int {
            b"Hard\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"CYO\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    printf(
        b"Set Extended: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
        if optExtended.0 != 0 {
            b"True\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"False\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    printf(
        b"Set Nomad: %s\n\n\0".as_ptr() as *const ::core::ffi::c_char,
        if optNomad == 2 as ::core::ffi::c_int {
            b"Normal\0".as_ptr() as *const ::core::ffi::c_char
        } else if optNomad == 1 as ::core::ffi::c_int {
            b"Easy\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"Off\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    log_add(
        log_Level::log_Info,
        b"Set Seed Type: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
        SeedStr(),
    );
    log_add(
        log_Level::log_Info,
        b"Set Seed: %d\n\0".as_ptr() as *const ::core::ffi::c_char,
        optCustomSeed,
    );
    log_add(
        log_Level::log_Info,
        b"Set Difficulty: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
        if optDifficulty == 0 as ::core::ffi::c_int {
            b"Normal\0".as_ptr() as *const ::core::ffi::c_char
        } else if optDifficulty == 1 as ::core::ffi::c_int {
            b"Easy\0".as_ptr() as *const ::core::ffi::c_char
        } else if optDifficulty == 2 as ::core::ffi::c_int {
            b"Hard\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"CYO\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    log_add(
        log_Level::log_Info,
        b"Set Extended: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
        if optExtended.0 != 0 {
            b"True\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"False\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    log_add(
        log_Level::log_Info,
        b"Set Nomad: %s\n\n\0".as_ptr() as *const ::core::ffi::c_char,
        if optNomad == 2 as ::core::ffi::c_int {
            b"Normal\0".as_ptr() as *const ::core::ffi::c_char
        } else if optNomad == 1 as ::core::ffi::c_int {
            b"Easy\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"Off\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    while uqm_lua_checked!(StartGame(), 0).0 != 0 {
        if !SetPlayerInputAll() {
            log_add(
                log_Level::log_Fatal,
                b"Could not set player input.\0".as_ptr() as *const ::core::ffi::c_char,
            );
            explode();
        }
        uqm_lua_checked!(luaUqm_reinitState(), 0);
        uqm_lua_checked!(InitGameStructures(), 0);
        InitGameClock();
        uqm_lua_checked!(initEventSystem(), 0);
        uqm_lua_checked!(AddInitialGameEvents(), 0);
        DebugKeyPressed = BOOLEAN::FALSE;
        if LastActivity as ::core::ffi::c_int
            == C2Rust_Unnamed_0::CHECK_LOAD.0 as ::core::ffi::c_int
                | C2Rust_Unnamed_0::CHECK_RESTART.0 as ::core::ffi::c_int
        {
            printf(
                b"New Game Seed Type: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                SeedStr(),
            );
            printf(
                b"New Game Seed: %d\n\0".as_ptr() as *const ::core::ffi::c_char,
                GlobData.SIS_state.Seed,
            );
            printf(
                b"New Game Difficulty: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                if GlobData.SIS_state.Difficulty as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                {
                    b"Normal\0".as_ptr() as *const ::core::ffi::c_char
                } else if GlobData.SIS_state.Difficulty as ::core::ffi::c_int
                    == 1 as ::core::ffi::c_int
                {
                    b"Easy\0".as_ptr() as *const ::core::ffi::c_char
                } else if GlobData.SIS_state.Difficulty as ::core::ffi::c_int
                    == 2 as ::core::ffi::c_int
                {
                    b"Hard\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"CYO\0".as_ptr() as *const ::core::ffi::c_char
                },
            );
            printf(
                b"New Game Extended: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                if GlobData.SIS_state.Extended as ::core::ffi::c_int != 0 {
                    b"True\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"False\0".as_ptr() as *const ::core::ffi::c_char
                },
            );
            printf(
                b"New Game Nomad: %s\n\n\0".as_ptr() as *const ::core::ffi::c_char,
                if GlobData.SIS_state.Nomad as ::core::ffi::c_int
                    == 2 as ::core::ffi::c_int
                {
                    b"Normal\0".as_ptr() as *const ::core::ffi::c_char
                } else if GlobData.SIS_state.Nomad as ::core::ffi::c_int
                    == 1 as ::core::ffi::c_int
                {
                    b"Easy\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"Off\0".as_ptr() as *const ::core::ffi::c_char
                },
            );
            log_add(
                log_Level::log_Info,
                b"New Game Seed Type: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                SeedStr(),
            );
            log_add(
                log_Level::log_Info,
                b"New Game Seed: %d\n\0".as_ptr() as *const ::core::ffi::c_char,
                GlobData.SIS_state.Seed,
            );
            log_add(
                log_Level::log_Info,
                b"New Game Difficulty: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                if GlobData.SIS_state.Difficulty as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                {
                    b"Normal\0".as_ptr() as *const ::core::ffi::c_char
                } else if GlobData.SIS_state.Difficulty as ::core::ffi::c_int
                    == 1 as ::core::ffi::c_int
                {
                    b"Easy\0".as_ptr() as *const ::core::ffi::c_char
                } else if GlobData.SIS_state.Difficulty as ::core::ffi::c_int
                    == 2 as ::core::ffi::c_int
                {
                    b"Hard\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"CYO\0".as_ptr() as *const ::core::ffi::c_char
                },
            );
            log_add(
                log_Level::log_Info,
                b"New Game Extended: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                if GlobData.SIS_state.Extended as ::core::ffi::c_int != 0 {
                    b"True\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"False\0".as_ptr() as *const ::core::ffi::c_char
                },
            );
            log_add(
                log_Level::log_Info,
                b"New Game Nomad: %s\n\n\0".as_ptr() as *const ::core::ffi::c_char,
                if GlobData.SIS_state.Nomad as ::core::ffi::c_int
                    == 2 as ::core::ffi::c_int
                {
                    b"Normal\0".as_ptr() as *const ::core::ffi::c_char
                } else if GlobData.SIS_state.Nomad as ::core::ffi::c_int
                    == 1 as ::core::ffi::c_int
                {
                    b"Easy\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"Off\0".as_ptr() as *const ::core::ffi::c_char
                },
            );
        }
        loop {
            if ::core::ptr::read_volatile::<
                Option<unsafe extern "C" fn() -> ()>,
            >(&raw const debugHook)
                .is_some()
            {
                let mut saveDebugHook: Option<unsafe extern "C" fn() -> ()> = None;
                saveDebugHook = ::core::ptr::read_volatile::<
                    Option<unsafe extern "C" fn() -> ()>,
                >(&raw const debugHook);
                ::core::ptr::write_volatile(&raw mut debugHook, None);
                uqm_lua_checked!(Some(saveDebugHook.expect("non-null function pointer"))
                    .expect("non-null function pointer")(), 0);
            } else if uqm_lua_checked!(QuickLoadDeferred(), 0).0 == 0 {
                SetStatusMessageMode(StatMsgMode::SMM_DEFAULT);
                if (GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
                    | NextActivity as ::core::ffi::c_int)
                    & C2Rust_Unnamed_0::CHECK_LOAD.0 as ::core::ffi::c_int == 0
                {
                    memset(
                        &raw mut GlobData.Game_state.velocity
                            as *mut ::core::ffi::c_void,
                        0 as ::core::ffi::c_int,
                        ::core::mem::size_of::<VELOCITY_DESC>() as size_t,
                    );
                } else if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
                    & C2Rust_Unnamed_0::CHECK_LOAD.0 as ::core::ffi::c_int != 0
                {
                    GlobData.Game_state.CurrentActivity = NextActivity;
                }
                if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
                    & C2Rust_Unnamed_0::START_ENCOUNTER.0 as ::core::ffi::c_int != 0
                    || uqm_lua_checked!(getGameStateUint(
                        b"CHMMR_BOMB_STATE\0".as_ptr() as *const ::core::ffi::c_char,
                    ), 0) == 2 as DWORD
                {
                    if uqm_lua_checked!(getGameStateUint(
                        b"CHMMR_BOMB_STATE\0".as_ptr() as *const ::core::ffi::c_char,
                    ), 0) == 2 as DWORD
                        && uqm_lua_checked!(getGameStateUint(
                            b"STARBASE_AVAILABLE\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        ), 0) == 0
                    {
                        uqm_lua_checked!(InstallBombAtEarth(), 0);
                    } else if uqm_lua_checked!(getGameStateUint(
                        b"GLOBAL_FLAGS_AND_DATA\0".as_ptr() as *const ::core::ffi::c_char,
                    ), 0) == !(0 as ::core::ffi::c_int) as BYTE as DWORD
                        || uqm_lua_checked!(getGameStateUint(
                            b"CHMMR_BOMB_STATE\0".as_ptr() as *const ::core::ffi::c_char,
                        ), 0) == 2 as DWORD
                    {
                        GlobData.Game_state.CurrentActivity = (GlobData
                            .Game_state
                            .CurrentActivity as ::core::ffi::c_int
                            | C2Rust_Unnamed_0::START_ENCOUNTER.0 as ::core::ffi::c_int)
                            as ACTIVITY;
                        uqm_lua_checked!(VisitStarBase(), 0);
                    } else {
                        GlobData.Game_state.CurrentActivity = (GlobData
                            .Game_state
                            .CurrentActivity as ::core::ffi::c_int
                            | C2Rust_Unnamed_0::START_ENCOUNTER.0 as ::core::ffi::c_int)
                            as ACTIVITY;
                        uqm_lua_checked!(RaceCommunication(), 0);
                    }
                    if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
                        & (C2Rust_Unnamed_0::CHECK_ABORT.0 as ::core::ffi::c_int
                            | C2Rust_Unnamed_0::CHECK_LOAD.0 as ::core::ffi::c_int) == 0
                    {
                        GlobData.Game_state.CurrentActivity = (GlobData
                            .Game_state
                            .CurrentActivity as ::core::ffi::c_int
                            & !(C2Rust_Unnamed_0::START_ENCOUNTER.0
                                as ::core::ffi::c_int)) as ACTIVITY;
                        if GlobData.Game_state.CurrentActivity as BYTE
                            as ::core::ffi::c_int
                            == C2Rust_Unnamed_0::IN_INTERPLANETARY.0
                                as ::core::ffi::c_int
                        {
                            GlobData.Game_state.CurrentActivity = (GlobData
                                .Game_state
                                .CurrentActivity as ::core::ffi::c_int
                                | C2Rust_Unnamed_0::START_INTERPLANETARY.0
                                    as ::core::ffi::c_int) as ACTIVITY;
                        }
                    }
                } else if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
                    & C2Rust_Unnamed_0::START_INTERPLANETARY.0 as ::core::ffi::c_int != 0
                {
                    GlobData.Game_state.CurrentActivity = (((0 as ::core::ffi::c_int
                        as BYTE as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
                        as UWORD as ::core::ffi::c_int
                        | C2Rust_Unnamed_0::IN_INTERPLANETARY.0 as ::core::ffi::c_int
                            as BYTE as ::core::ffi::c_int) as ACTIVITY;
                    uqm_lua_checked!(DrawAutoPilotMessage(BOOLEAN::TRUE), 0);
                    SetGameClockRate(30 as COUNT);
                    uqm_lua_checked!(ExploreSolarSys(), 0);
                } else {
                    GlobData.Game_state.CurrentActivity = (((0 as ::core::ffi::c_int
                        as BYTE as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
                        as UWORD as ::core::ffi::c_int
                        | C2Rust_Unnamed_0::IN_HYPERSPACE.0 as ::core::ffi::c_int as BYTE
                            as ::core::ffi::c_int) as ACTIVITY;
                    uqm_lua_checked!(DrawAutoPilotMessage(BOOLEAN::TRUE), 0);
                    SetGameClockRate(5 as COUNT);
                    uqm_lua_checked!(Battle(Some(on_battle_frame as unsafe extern "C" fn() -> ())), 0);
                }
                uqm_lua_checked!(SetFlashRect(::core::ptr::null::<RECT>(), BOOLEAN::FALSE), 0);
                LastActivity = GlobData.Game_state.CurrentActivity;
                if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
                    & (C2Rust_Unnamed_0::CHECK_ABORT.0 as ::core::ffi::c_int
                        | C2Rust_Unnamed_0::CHECK_LOAD.0 as ::core::ffi::c_int) == 0
                    && (GlobData.Game_state.CurrentActivity as BYTE as ::core::ffi::c_int
                        == C2Rust_Unnamed_0::WON_LAST_BATTLE.0 as ::core::ffi::c_int
                        || GlobData.SIS_state.CrewEnlisted as ::core::ffi::c_int
                            == !(0 as ::core::ffi::c_int) as COUNT as ::core::ffi::c_int)
                {
                    if uqm_lua_checked!(getGameStateUint(
                        b"KOHR_AH_KILLED_ALL\0".as_ptr() as *const ::core::ffi::c_char,
                    ), 0) != 0
                    {
                        uqm_lua_checked!(InitCommunication(CONVERSATION::BLACKURQ_CONVERSATION), 0);
                        if optGameOver.0 != 0 {
                            uqm_lua_checked!(GameOver(
                                C2Rust_Unnamed_1::DEATH_MARCH.0 as ::core::ffi::c_int
                                    as BYTE,
                            ), 0);
                        }
                    } else if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
                        & C2Rust_Unnamed_0::CHECK_RESTART.0 as ::core::ffi::c_int != 0
                    {
                        DeathBySurrender = BOOLEAN::TRUE;
                        GlobData.Game_state.CurrentActivity = (GlobData
                            .Game_state
                            .CurrentActivity as ::core::ffi::c_int
                            & !(C2Rust_Unnamed_0::CHECK_RESTART.0 as ::core::ffi::c_int))
                            as ACTIVITY;
                    }
                    break;
                }
            }
            if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
                & C2Rust_Unnamed_0::CHECK_ABORT.0 as ::core::ffi::c_int != 0
            {
                break;
            }
        }
        uqm_lua_checked!(StopSound(), 0);
        uninitEventSystem();
        UninitGameClock();
        UninitGameStructures();
        uqm_lua_checked!(ClearPlayerInputAll(), 0);
    }
    uqm_lua_checked!(luaUqm_uninitState(), 0);
    uqm_lua_checked!(UninitGameKernel(), 0);
    FreeMasterShipList();
    uqm_lua_checked!(FreeKernel(), 0);
    log_showBox(false, false);
    ::core::ptr::write_volatile(
        &raw mut MainExited,
        BOOLEAN::TRUE.0 as ::core::ffi::c_int,
    );
    return 0 as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn SeedStr() -> *mut UNICODE {
    match optSeedType {
        0 => return b"None\0".as_ptr() as *const ::core::ffi::c_char as *mut UNICODE,
        1 => return b"Planet\0".as_ptr() as *const ::core::ffi::c_char as *mut UNICODE,
        2 => return b"MRQ\0".as_ptr() as *const ::core::ffi::c_char as *mut UNICODE,
        3 => return b"StarSeed\0".as_ptr() as *const ::core::ffi::c_char as *mut UNICODE,
        _ => return b"???\0".as_ptr() as *const ::core::ffi::c_char as *mut UNICODE,
    };
}
#[inline]
unsafe extern "C" fn explode() -> ! {
    abort();
}
