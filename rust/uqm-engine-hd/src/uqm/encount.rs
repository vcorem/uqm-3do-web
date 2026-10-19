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

// Rust translation of src/uqm/encount.c.
extern "C" {
    pub type string_table_entry;
    pub type string_table;
    pub type context_desc;
    pub type frame_desc;
    pub type font_desc;
    pub type drawable_desc;
    pub type tfb_soundsample;
    pub type getmelee_struct;
    pub type FlashContext;
    fn SleepThread(timePeriod: TimePeriod);
    fn SleepThreadUntil(wakeTime: TimeCount);
    fn GetTimeCounter() -> TimeCount;
    fn __assert_fail(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    ) -> !;
    fn FreeLink(pq: *mut QUEUE, hLink: HLINK);
    fn ReinitQueue(pq: *mut QUEUE);
    fn RemoveQueue(pq: *mut QUEUE, hLink: HLINK);
    fn printf(_: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn sprintf(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn CaptureStringTable(StringTable: STRING_TABLE) -> STRING;
    fn ReleaseStringTable(String: STRING) -> STRING_TABLE;
    fn SetAbsStringTableIndex(String: STRING, StringTableIndex: COUNT) -> STRING;
    fn GetStringAddress(String: STRING) -> STRINGPTR;
    fn utf8StringCopy(
        dst: *mut ::core::ffi::c_char,
        size: size_t,
        src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_uchar;
    fn LoadSoundInstance(res: RESOURCE) -> SOUND_REF;
    fn LoadMusicInstance(res: RESOURCE) -> MUSIC_REF;
    fn DestroySound(SoundRef: SOUND_REF) -> BOOLEAN;
    fn DestroyMusic(MusicRef: MUSIC_REF) -> BOOLEAN;
    fn WaitForSoundEnd(Channel: COUNT);
    fn abort() -> !;
    fn strupr(_: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn SetContext(Context: CONTEXT) -> CONTEXT;
    fn SetContextForeGroundColor(Color: Color) -> Color;
    fn SetContextBackGroundColor(Color: Color) -> Color;
    fn DrawStamp(pStamp: *mut STAMP);
    fn DrawFilledStamp(pStamp: *mut STAMP);
    fn DrawFilledRectangle(pRect: *mut RECT);
    fn font_GetTextRect(pText: *mut TEXT) -> RECT;
    fn font_DrawText(pText: *mut TEXT);
    fn BatchGraphics();
    fn UnbatchGraphics();
    fn ClearDrawable();
    fn DestroyDrawable(Drawable: DRAWABLE) -> BOOLEAN;
    fn GetFrameRect(Frame: FRAME, pRect: *mut RECT) -> BOOLEAN;
    fn GetFrameHot(Frame: FRAME) -> HOT_SPOT;
    fn SetContextFont(Font: FONT) -> FONT;
    fn TextRect(pText: *mut TEXT, pRect: *mut RECT, pdelta: *mut BYTE) -> BOOLEAN;
    fn SetAbsFrameIndex(Frame: FRAME, FrameIndex: COUNT) -> FRAME;
    fn IncFrameIndex(Frame: FRAME) -> FRAME;
    fn ReleaseDrawable(Frame: FRAME) -> DRAWABLE;
    static mut CurrentInputState: CONTROLLER_INPUT_STATE;
    static mut PulsedInputState: CONTROLLER_INPUT_STATE;
    static mut PlayerControls: [CONTROL_TEMPLATE; 0];
    fn FlushInput();
    fn WaitForAnyButton(
        newButton: BOOLEAN,
        duration: TimePeriod,
        resetInput: BOOLEAN,
    ) -> BOOLEAN;
    fn DoInput(pInputState: *mut ::core::ffi::c_void, resetInput: BOOLEAN);
    static mut optWhichFonts: ::core::ffi::c_int;
    static mut resolutionFactor: ::core::ffi::c_uint;
    static mut optScrTrans: ::core::ffi::c_int;
    static mut optSlaughterMode: OPT_ENABLABLE;
    static mut optWindowType: DWORD;
    static mut CanvasWidth: ::core::ffi::c_int;
    static mut CanvasHeight: ::core::ffi::c_int;
    static mut sinetab: [SDWORD; 0];
    fn ARCTAN(delta_x: SDWORD, delta_y: SDWORD) -> COUNT;
    fn InitSISContexts();
    fn DrawSISFrame();
    fn SetFlashRect(pRect: *const RECT, purple: BOOLEAN);
    fn DrawHyperCoords(puniverse: POINT);
    fn DrawSISTitle(pStr: *mut UNICODE);
    fn DrawSISMessage(pStr: *const UNICODE);
    fn DrawStatusMessage(pStr: *const UNICODE);
    fn SetStatusMessageMode(_: StatMsgMode) -> StatMsgMode;
    fn DeltaSISGauges(
        crew_delta: SIZE,
        fuel_delta: SDWORD,
        resunit_delta: ::core::ffi::c_int,
    );
    fn setGameStateUint(name: *const ::core::ffi::c_char, val: DWORD);
    fn getGameStateUint(name: *const ::core::ffi::c_char) -> DWORD;
    static mut GlobData: GLOBDATA;
    fn inHQSpace() -> BOOLEAN;
    fn isPC(optWhich: ::core::ffi::c_int) -> OPT_CONSOLETYPE;
    fn TFB_Random() -> DWORD;
    fn square_root(value: DWORD) -> COUNT;
    static mut pSolarSysState: *mut SOLARSYS_STATE;
    fn worldIsMoon(solarSys: *const SOLARSYS_STATE, world: *const PLANET_DESC) -> bool;
    fn moonIndex(solarSys: *const SOLARSYS_STATE, moon: *const PLANET_DESC) -> COUNT;
    fn GetStarBackGround(encounter: BOOLEAN) -> FRAME;
    static mut PlayerInput: [*mut InputContext; 0];
    static mut battle_counter: [BYTE; 2];
    fn Battle(_: Option<BattleFrameCallback>) -> BOOLEAN;
    static mut EncounterRace: SIZE;
    fn load_gravity_well(selector: BYTE);
    fn free_gravity_well();
    fn Build(pQueue: *mut QUEUE, SpeciesID: SPECIES_ID) -> HLINK;
    fn GetStarShipFromIndex(pShipQ: *mut QUEUE, Index: COUNT) -> HLINK;
    fn CheckSphereTracking(race: RACE_ID) -> BOOLEAN;
    fn DoMenuChooser(pMS: *mut MENU_STATE, BaseState: BYTE) -> BOOLEAN;
    fn DrawMenuStateStrings(beg_index: BYTE, NewState: SWORD);
    fn GameOptions() -> BOOLEAN;
    static mut CurStarDescPtr: *mut STAR_DESC;
    fn GetClusterName(pSD: *const STAR_DESC, buf: *mut UNICODE);
    fn DrawArmadaPickShip(draw_salvage_frame: BOOLEAN, pPickRect: *mut RECT);
    fn InitPickFrame();
    fn DestroyPickFrame();
    fn RepairPickFrame(pRect: *mut RECT, frame: COUNT);
    static mut SpaceContext: CONTEXT;
    static mut MicroFont: FONT;
    static mut TinyFont: FONT;
    static mut TinyFontBold: FONT;
    static mut race_q: [QUEUE; 0];
    static mut PlayerControl: [BYTE; 0];
    fn SetPlayerInput(playerI: COUNT) -> bool;
    fn ClearPlayerInput(playerI: COUNT);
    static mut GameStrings: STRING;
    fn StopMusic();
    fn PlayMusic(MusicRef: MUSIC_REF, Continuous: BOOLEAN, Priority: BYTE);
    static mut GameSounds: SOUND;
    fn SetMenuSounds(sound_0: MENU_SOUND_FLAGS, sound_1: MENU_SOUND_FLAGS);
    fn SetTransitionSource(pRect: *const RECT);
    fn ScreenTransition(transition: ::core::ffi::c_int, pRect: *const RECT);
    fn log_add(_: log_Level, fmt: *const ::core::ffi::c_char, ...);
    fn AnyButtonPress(DetectSpecial: BOOLEAN) -> BOOLEAN;
    fn ShipCost(race_id: BYTE) -> COUNT;
}
pub type BYTE = uint8;
pub type uint8 = ::core::ffi::c_uchar;
pub type COUNT = UWORD;
pub type UWORD = uint16;
pub type uint16 = ::core::ffi::c_ushort;
pub type SDWORD = sint32;
pub type sint32 = ::core::ffi::c_int;
pub type POINT = point;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct point {
    pub x: COORD,
    pub y: COORD,
}
pub type COORD = SWORD;
pub type SWORD = sint16;
pub type sint16 = ::core::ffi::c_short;
pub type SIZE = SWORD;
pub type HELEMENT = HLINK;
pub type HLINK = QUEUE_HANDLE;
pub type QUEUE_HANDLE = *mut ::core::ffi::c_void;
pub type size_t = ::core::ffi::c_ulong;
pub type sint8 = ::core::ffi::c_schar;
pub type uint32 = ::core::ffi::c_uint;
pub type UBYTE = uint8;
pub type SBYTE = sint8;
pub type DWORD = uint32;
pub type UNICODE = ::core::ffi::c_char;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct BOOLEAN(pub ::core::ffi::c_uint);
impl BOOLEAN {
    pub const FALSE: Self = Self(0);
    pub const TRUE: Self = Self(1);
}
pub type TimeCount = DWORD;
pub type TimePeriod = DWORD;
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
pub type STRING_TABLE_DESC = string_table;
pub type STRING_TABLE = *mut STRING_TABLE_DESC;
pub type STRING = *mut STRING_TABLE_ENTRY_DESC;
pub type STRINGPTR = *mut ::core::ffi::c_char;
pub type CONTEXT_DESC = context_desc;
pub type FRAME_DESC = frame_desc;
pub type FONT_DESC = font_desc;
pub type DRAWABLE_DESC = drawable_desc;
pub type CONTEXT = *mut CONTEXT_DESC;
pub type FRAME = *mut FRAME_DESC;
pub type FONT = *mut FONT_DESC;
pub type DRAWABLE = *mut DRAWABLE_DESC;
pub type TIME_VALUE = UWORD;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct extent {
    pub width: COORD,
    pub height: COORD,
}
pub type EXTENT = extent;
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
pub type COLORMAP = STRING;
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
pub type HOT_SPOT = POINT;
pub type SOUND_REF = STRING_TABLE;
pub type SOUND = STRING;
pub type TFB_SoundSample = tfb_soundsample;
pub type MUSIC_REF = *mut *mut TFB_SoundSample;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed {
    pub const KEY_UP: Self = Self(0);
    pub const KEY_DOWN: Self = Self(1);
    pub const KEY_LEFT: Self = Self(2);
    pub const KEY_RIGHT: Self = Self(3);
    pub const KEY_WEAPON: Self = Self(4);
    pub const KEY_SPECIAL: Self = Self(5);
    pub const KEY_ESCAPE: Self = Self(6);
    pub const KEY_THRUST: Self = Self(7);
    pub const NUM_KEYS: Self = Self(8);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_0 {
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct CONTROL_TEMPLATE(pub ::core::ffi::c_uint);
impl CONTROL_TEMPLATE {
    pub const CONTROL_TEMPLATE_PL_1: Self = Self(0);
    pub const CONTROL_TEMPLATE_PL_2: Self = Self(1);
    pub const NUM_TEMPLATES: Self = Self(2);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _controller_input_state {
    pub key: [[::core::ffi::c_int; 8]; 2],
    pub menu: [::core::ffi::c_int; 31],
}
pub type CONTROLLER_INPUT_STATE = _controller_input_state;
pub type BATTLE_INPUT_STATE = UBYTE;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct OPT_ENABLABLE(pub ::core::ffi::c_uint);
impl OPT_ENABLABLE {
    pub const OPTVAL_DISABLED: Self = Self(0);
    pub const OPTVAL_ENABLED: Self = Self(1);
}
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
pub struct C2Rust_Unnamed_1(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_1 {
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct planet_desc {
    pub rand_seed: DWORD,
    pub data_index: BYTE,
    pub NumPlanets: BYTE,
    pub radius: SIZE,
    pub location: POINT,
    pub temp_color: Color,
    pub NextIndex: COUNT,
    pub image: STAMP,
    pub pPrevDesc: *mut PLANET_DESC,
    pub orbit: PLANET_ORBIT,
    pub size: COUNT,
    pub angle: COUNT,
    pub rotFrameIndex: ::core::ffi::c_int,
    pub rotPointIndex: ::core::ffi::c_int,
    pub rotwidth: ::core::ffi::c_int,
    pub rotheight: ::core::ffi::c_int,
    pub orb_speed: ::core::ffi::c_double,
    pub rot_speed: ::core::ffi::c_double,
    pub PlanetByte: BYTE,
    pub MoonByte: BYTE,
    pub frame_offset: BYTE,
}
pub type PLANET_ORBIT = planet_orbit;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct planet_orbit {
    pub TopoZoomFrame: FRAME,
    pub lpTopoData: *mut SBYTE,
    pub SphereFrame: FRAME,
    pub ObjectFrame: FRAME,
    pub TintFrame: FRAME,
    pub TintColor: Color,
    pub TopoColors: *mut Color,
    pub ScanColors: *mut *mut Color,
    pub ScratchArray: *mut Color,
    pub WorkFrame: FRAME,
    pub BackFrame: FRAME,
    pub light_diff: *mut *mut DWORD,
    pub map_rotate: *mut *mut MAP3D_POINT,
    pub TopoMask: FRAME,
    pub sphereBytes: *mut BYTE,
    pub sphereMap: COLORMAP,
    pub scanType: COUNT,
    pub Shade: FRAME,
    pub ShadeColors: *mut Color,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MAP3D_POINT {
    pub p: [POINT; 4],
    pub m: [DWORD; 4],
}
pub type PLANET_DESC = planet_desc;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct star_desc {
    pub star_pt: POINT,
    pub Type: BYTE,
    pub Index: BYTE,
    pub Prefix: BYTE,
    pub Postfix: BYTE,
}
pub type STAR_DESC = star_desc;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct node_info {
    pub loc_pt: POINT,
    pub density: COUNT,
    pub r#type: COUNT,
}
pub type NODE_INFO = node_info;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct solarsys_state {
    pub InputFunc: Option<unsafe extern "C" fn(*mut solarsys_state) -> BOOLEAN>,
    pub InIpFlight: BOOLEAN,
    pub WaitIntersect: COUNT,
    pub SunDesc: [PLANET_DESC; 1],
    pub PlanetDesc: [PLANET_DESC; 16],
    pub MoonDesc: [PLANET_DESC; 4],
    pub pBaseDesc: *mut PLANET_DESC,
    pub pOrbitalDesc: *mut PLANET_DESC,
    pub FirstPlanetIndex: SIZE,
    pub LastPlanetIndex: SIZE,
    pub turn_counter: BYTE,
    pub turn_wait: BYTE,
    pub thrust_counter: BYTE,
    pub max_ship_speed: BYTE,
    pub XlatRef: STRING,
    pub XlatPtr: *const ::core::ffi::c_void,
    pub OrbitalCMap: COLORMAP,
    pub SysInfo: SYSTEM_INFO,
    pub genFuncs: *const GenerateFunctions,
    pub PlanetSideFrame: [FRAME; 6],
    pub TopoFrame: FRAME,
    pub Orbit: PLANET_ORBIT,
    pub InOrbit: BOOLEAN,
    pub ScanFrame: [FRAME; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GenerateFunctions {
    pub initNpcs: InitNpcsFunction,
    pub reinitNpcs: ReinitNpcsFunction,
    pub uninitNpcs: UninitNpcsFunction,
    pub generatePlanets: GeneratePlanetsFunction,
    pub generateMoons: GenerateMoonsFunction,
    pub generateName: GenerateNameFunction,
    pub generateOrbital: GenerateOrbitalFunction,
    pub generateMinerals: GenerateMineralsFunction,
    pub generateEnergy: GenerateEnergyFunction,
    pub generateLife: GenerateLifeFunction,
    pub pickupMinerals: PickupMineralsFunction,
    pub pickupEnergy: PickupEnergyFunction,
    pub pickupLife: PickupLifeFunction,
}
pub type PickupLifeFunction = Option<
    unsafe extern "C" fn(*mut SOLARSYS_STATE, *mut PLANET_DESC, COUNT) -> bool,
>;
pub type SOLARSYS_STATE = solarsys_state;
pub type PickupEnergyFunction = Option<
    unsafe extern "C" fn(*mut SOLARSYS_STATE, *mut PLANET_DESC, COUNT) -> bool,
>;
pub type PickupMineralsFunction = Option<
    unsafe extern "C" fn(*mut SOLARSYS_STATE, *mut PLANET_DESC, COUNT) -> bool,
>;
pub type GenerateLifeFunction = Option<
    unsafe extern "C" fn(
        *const SOLARSYS_STATE,
        *const PLANET_DESC,
        COUNT,
        *mut NODE_INFO,
    ) -> COUNT,
>;
pub type GenerateEnergyFunction = Option<
    unsafe extern "C" fn(
        *const SOLARSYS_STATE,
        *const PLANET_DESC,
        COUNT,
        *mut NODE_INFO,
    ) -> COUNT,
>;
pub type GenerateMineralsFunction = Option<
    unsafe extern "C" fn(
        *const SOLARSYS_STATE,
        *const PLANET_DESC,
        COUNT,
        *mut NODE_INFO,
    ) -> COUNT,
>;
pub type GenerateOrbitalFunction = Option<
    unsafe extern "C" fn(*mut SOLARSYS_STATE, *mut PLANET_DESC) -> bool,
>;
pub type GenerateNameFunction = Option<
    unsafe extern "C" fn(*const SOLARSYS_STATE, *const PLANET_DESC) -> bool,
>;
pub type GenerateMoonsFunction = Option<
    unsafe extern "C" fn(*mut SOLARSYS_STATE, *mut PLANET_DESC) -> bool,
>;
pub type GeneratePlanetsFunction = Option<
    unsafe extern "C" fn(*mut SOLARSYS_STATE) -> bool,
>;
pub type UninitNpcsFunction = Option<unsafe extern "C" fn(*mut SOLARSYS_STATE) -> bool>;
pub type ReinitNpcsFunction = Option<unsafe extern "C" fn(*mut SOLARSYS_STATE) -> bool>;
pub type InitNpcsFunction = Option<unsafe extern "C" fn(*mut SOLARSYS_STATE) -> bool>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SYSTEM_INFO {
    pub StarSize: BYTE,
    pub StarIntensity: BYTE,
    pub StarEnergy: UWORD,
    pub PlanetInfo: PLANET_INFO,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PLANET_INFO {
    pub AxialTilt: SIZE,
    pub Tectonics: UWORD,
    pub Weather: UWORD,
    pub PlanetDensity: UWORD,
    pub PlanetRadius: UWORD,
    pub SurfaceGravity: UWORD,
    pub SurfaceTemperature: SIZE,
    pub RotationPeriod: UWORD,
    pub AtmoDensity: UWORD,
    pub LifeChance: SIZE,
    pub PlanetToSunDist: DWORD,
    pub PlanDataPtr: *const PlanetFrame,
    pub ScanSeed: [DWORD; 3],
    pub ScanRetrieveMask: [DWORD; 3],
    pub PartiallyScavengedList: [[BYTE; 32]; 3],
    pub DiscoveryString: STRING,
    pub LanderFont: FONT,
    pub LanderFontEff: FRAME,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PlanetFrame {
    pub Type: BYTE,
    pub BaseTectonics: BYTE,
    pub AtmoAndDensity: BYTE,
    pub UsefulElements: [ELEMENT_ENTRY; 8],
    pub CMapInstance: RESOURCE,
    pub XlatTabInstance: RESOURCE,
    pub num_faults: COUNT,
    pub fault_depth: SIZE,
    pub num_blemishes: COUNT,
    pub base_elevation: SIZE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ELEMENT_ENTRY {
    pub ElementType: BYTE,
    pub Density: BYTE,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_2(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_2 {
    pub const FIRST_ROCKY_WORLD: Self = Self(0);
    pub const FIRST_SMALL_ROCKY_WORLD: Self = Self(0);
    pub const OOLITE_WORLD: Self = Self(0);
    pub const YTTRIC_WORLD: Self = Self(1);
    pub const QUASI_DEGENERATE_WORLD: Self = Self(2);
    pub const LANTHANIDE_WORLD: Self = Self(3);
    pub const TREASURE_WORLD: Self = Self(4);
    pub const UREA_WORLD: Self = Self(5);
    pub const METAL_WORLD: Self = Self(6);
    pub const RADIOACTIVE_WORLD: Self = Self(7);
    pub const OPALESCENT_WORLD: Self = Self(8);
    pub const CYANIC_WORLD: Self = Self(9);
    pub const ACID_WORLD: Self = Self(10);
    pub const ALKALI_WORLD: Self = Self(11);
    pub const HALIDE_WORLD: Self = Self(12);
    pub const GREEN_WORLD: Self = Self(13);
    pub const COPPER_WORLD: Self = Self(14);
    pub const CARBIDE_WORLD: Self = Self(15);
    pub const ULTRAMARINE_WORLD: Self = Self(16);
    pub const NOBLE_WORLD: Self = Self(17);
    pub const AZURE_WORLD: Self = Self(18);
    pub const CHONDRITE_WORLD: Self = Self(19);
    pub const PURPLE_WORLD: Self = Self(20);
    pub const SUPER_DENSE_WORLD: Self = Self(21);
    pub const PELLUCID_WORLD: Self = Self(22);
    pub const DUST_WORLD: Self = Self(23);
    pub const CRIMSON_WORLD: Self = Self(24);
    pub const CIMMERIAN_WORLD: Self = Self(25);
    pub const INFRARED_WORLD: Self = Self(26);
    pub const SELENIC_WORLD: Self = Self(27);
    pub const AURIC_WORLD: Self = Self(28);
    pub const LAST_SMALL_ROCKY_WORLD: Self = Self(28);
    pub const FIRST_LARGE_ROCKY_WORLD: Self = Self(29);
    pub const FLUORESCENT_WORLD: Self = Self(29);
    pub const ULTRAVIOLET_WORLD: Self = Self(30);
    pub const PLUTONIC_WORLD: Self = Self(31);
    pub const RAINBOW_WORLD: Self = Self(32);
    pub const SHATTERED_WORLD: Self = Self(33);
    pub const SAPPHIRE_WORLD: Self = Self(34);
    pub const ORGANIC_WORLD: Self = Self(35);
    pub const XENOLITHIC_WORLD: Self = Self(36);
    pub const REDUX_WORLD: Self = Self(37);
    pub const PRIMORDIAL_WORLD: Self = Self(38);
    pub const EMERALD_WORLD: Self = Self(39);
    pub const CHLORINE_WORLD: Self = Self(40);
    pub const MAGNETIC_WORLD: Self = Self(41);
    pub const WATER_WORLD: Self = Self(42);
    pub const TELLURIC_WORLD: Self = Self(43);
    pub const HYDROCARBON_WORLD: Self = Self(44);
    pub const IODINE_WORLD: Self = Self(45);
    pub const VINYLOGOUS_WORLD: Self = Self(46);
    pub const RUBY_WORLD: Self = Self(47);
    pub const MAGMA_WORLD: Self = Self(48);
    pub const MAROON_WORLD: Self = Self(49);
    pub const LAST_LARGE_ROCKY_WORLD: Self = Self(49);
    pub const LAST_ROCKY_WORLD: Self = Self(49);
    pub const FIRST_GAS_GIANT: Self = Self(50);
    pub const BLU_GAS_GIANT: Self = Self(50);
    pub const CYA_GAS_GIANT: Self = Self(51);
    pub const GRN_GAS_GIANT: Self = Self(52);
    pub const GRY_GAS_GIANT: Self = Self(53);
    pub const ORA_GAS_GIANT: Self = Self(54);
    pub const PUR_GAS_GIANT: Self = Self(55);
    pub const RED_GAS_GIANT: Self = Self(56);
    pub const VIO_GAS_GIANT: Self = Self(57);
    pub const YEL_GAS_GIANT: Self = Self(58);
    pub const LAST_GAS_GIANT: Self = Self(58);
    pub const NUMBER_OF_PLANET_TYPES: Self = Self(59);
    pub const WORLD_TYPE_SPECIAL: Self = Self(128);
    pub const PLANET_SHIELDED: Self = Self(128);
    pub const HIERARCHY_STARBASE: Self = Self(255);
    pub const SA_MATRA: Self = Self(254);
    pub const DESTROYED_STARBASE: Self = Self(253);
    pub const PRECURSOR_STARBASE: Self = Self(252);
}
pub type ELEMENT_FLAGS = UWORD;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct state {
    pub location: POINT,
    pub image: C2Rust_Unnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_3 {
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
    pub c2rust_unnamed: C2Rust_Unnamed_8,
    pub c2rust_unnamed_0: C2Rust_Unnamed_7,
    pub c2rust_unnamed_1: C2Rust_Unnamed_6,
    pub c2rust_unnamed_2: C2Rust_Unnamed_5,
    pub c2rust_unnamed_3: C2Rust_Unnamed_4,
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
pub union C2Rust_Unnamed_4 {
    pub thrust_wait: BYTE,
    pub blast_offset: BYTE,
    pub next_turn: BYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_5 {
    pub turn_wait: BYTE,
    pub sys_loc: BYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_6 {
    pub mass_points: BYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_7 {
    pub crew_level: COUNT,
    pub hit_points: COUNT,
    pub facing: COUNT,
    pub cycle: COUNT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_8 {
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
pub type HSTARSHIP = HLINK;
pub type HSHIPFRAG = HLINK;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SHIP_FRAGMENT {
    pub pred: HLINK,
    pub succ: HLINK,
    pub SpeciesID: SPECIES_ID,
    pub captains_name_index: BYTE,
    pub race_id: BYTE,
    pub index: BYTE,
    pub crew_level: COUNT,
    pub max_crew: COUNT,
    pub energy_level: BYTE,
    pub max_energy: BYTE,
    pub race_strings: STRING,
    pub icons: FRAME,
    pub melee_icon: FRAME,
}
pub type HFLEETINFO = HLINK;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FLEET_INFO {
    pub pred: HFLEETINFO,
    pub succ: HFLEETINFO,
    pub SpeciesID: SPECIES_ID,
    pub shipIdStr: *const ::core::ffi::c_char,
    pub allied_state: UWORD,
    pub days_left: BYTE,
    pub growth_fract: BYTE,
    pub crew_level: COUNT,
    pub max_crew: COUNT,
    pub growth: BYTE,
    pub max_energy: BYTE,
    pub loc: POINT,
    pub race_strings: STRING,
    pub icons: FRAME,
    pub melee_icon: FRAME,
    pub actual_strength: COUNT,
    pub known_strength: COUNT,
    pub known_loc: POINT,
    pub growth_err_term: BYTE,
    pub func_index: BYTE,
    pub dest_loc: POINT,
    pub can_build: BOOLEAN,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_9(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_9 {
    pub const DEAD_GUY: Self = Self(0);
    pub const GOOD_GUY: Self = Self(1);
    pub const BAD_GUY: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct RACE_ID(pub ::core::ffi::c_uint);
impl RACE_ID {
    pub const ARILOU_SHIP: Self = Self(0);
    pub const CHMMR_SHIP: Self = Self(1);
    pub const HUMAN_SHIP: Self = Self(2);
    pub const ORZ_SHIP: Self = Self(3);
    pub const PKUNK_SHIP: Self = Self(4);
    pub const SHOFIXTI_SHIP: Self = Self(5);
    pub const SPATHI_SHIP: Self = Self(6);
    pub const SUPOX_SHIP: Self = Self(7);
    pub const THRADDASH_SHIP: Self = Self(8);
    pub const UTWIG_SHIP: Self = Self(9);
    pub const VUX_SHIP: Self = Self(10);
    pub const YEHAT_SHIP: Self = Self(11);
    pub const MELNORME_SHIP: Self = Self(12);
    pub const DRUUGE_SHIP: Self = Self(13);
    pub const ILWRATH_SHIP: Self = Self(14);
    pub const MYCON_SHIP: Self = Self(15);
    pub const SLYLANDRO_SHIP: Self = Self(16);
    pub const UMGAH_SHIP: Self = Self(17);
    pub const URQUAN_SHIP: Self = Self(18);
    pub const ZOQFOTPIK_SHIP: Self = Self(19);
    pub const SYREEN_SHIP: Self = Self(20);
    pub const BLACK_URQUAN_SHIP: Self = Self(21);
    pub const ANDROSYNTH_SHIP: Self = Self(22);
    pub const CHENJESU_SHIP: Self = Self(23);
    pub const MMRNMHRM_SHIP: Self = Self(24);
    pub const NUM_BUILDABLE_SHIPS: Self = Self(25);
    pub const YEHAT_REBEL_SHIP: Self = Self(25);
    pub const URQUAN_DRONE_SHIP: Self = Self(26);
    pub const SAMATRA_SHIP: Self = Self(26);
    pub const NUM_AVAILABLE_RACES: Self = Self(27);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_10(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_10 {
    pub const HAIL: Self = Self(0);
    pub const ATTACK: Self = Self(1);
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
pub type BattleFrameCallback = unsafe extern "C" fn() -> ();
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
pub type MENU_STATE = menu_state;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct menu_state {
    pub InputFunc: Option<unsafe extern "C" fn(*mut menu_state) -> BOOLEAN>,
    pub Initialized: SIZE,
    pub CurState: BYTE,
    pub CurFrame: FRAME,
    pub CurString: STRING,
    pub first_item: POINT,
    pub delta_item: SIZE,
    pub ModuleFrame: FRAME,
    pub flash_rect0: RECT,
    pub flash_rect1: RECT,
    pub flash_rect2: RECT,
    pub flash_frame0: FRAME,
    pub flash_frame1: FRAME,
    pub flashContext: *mut FlashContext,
    pub hMusic: MUSIC_REF,
    pub privData: *mut ::core::ffi::c_void,
}
pub type MENU_SOUND_FLAGS = UWORD;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_11(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_11 {
    pub const PM_SCAN: Self = Self(0);
    pub const PM_STARMAP: Self = Self(1);
    pub const PM_DEVICES: Self = Self(2);
    pub const PM_CARGO: Self = Self(3);
    pub const PM_ROSTER: Self = Self(4);
    pub const PM_GAME_MENU: Self = Self(5);
    pub const PM_NAVIGATE: Self = Self(6);
    pub const PM_MIN_SCAN: Self = Self(7);
    pub const PM_ENE_SCAN: Self = Self(8);
    pub const PM_BIO_SCAN: Self = Self(9);
    pub const PM_EXIT_SCAN: Self = Self(10);
    pub const PM_AUTO_SCAN: Self = Self(11);
    pub const PM_LAUNCH_LANDER: Self = Self(12);
    pub const PM_SAVE_GAME: Self = Self(13);
    pub const PM_LOAD_GAME: Self = Self(14);
    pub const PM_QUIT_GAME: Self = Self(15);
    pub const PM_CHANGE_SETTINGS: Self = Self(16);
    pub const PM_EXIT_GAME_MENU: Self = Self(17);
    pub const PM_CONVERSE: Self = Self(18);
    pub const PM_ATTACK: Self = Self(19);
    pub const PM_ENCOUNTER_GAME_MENU: Self = Self(20);
    pub const PM_FUEL: Self = Self(21);
    pub const PM_MODULE: Self = Self(22);
    pub const PM_OUTFIT_GAME_MENU: Self = Self(23);
    pub const PM_EXIT_OUTFIT: Self = Self(24);
    pub const PM_CREW: Self = Self(25);
    pub const PM_SHIPYARD_GAME_MENU: Self = Self(26);
    pub const PM_EXIT_SHIPYARD: Self = Self(27);
    pub const PM_SOUND_ON: Self = Self(28);
    pub const PM_SOUND_OFF: Self = Self(29);
    pub const PM_MUSIC_ON: Self = Self(30);
    pub const PM_MUSIC_OFF: Self = Self(31);
    pub const PM_CYBORG_OFF: Self = Self(32);
    pub const PM_CYBORG_NORMAL: Self = Self(33);
    pub const PM_CYBORG_DOUBLE: Self = Self(34);
    pub const PM_CYBORG_SUPER: Self = Self(35);
    pub const PM_READ_VERY_SLOW: Self = Self(36);
    pub const PM_READ_SLOW: Self = Self(37);
    pub const PM_READ_MODERATE: Self = Self(38);
    pub const PM_READ_FAST: Self = Self(39);
    pub const PM_READ_VERY_FAST: Self = Self(40);
    pub const PM_CHANGE_CAPTAIN: Self = Self(41);
    pub const PM_CHANGE_SHIP: Self = Self(42);
    pub const PM_EXIT_SETTINGS: Self = Self(43);
    pub const PM_ALT_SCAN: Self = Self(44);
    pub const PM_ALT_STARMAP: Self = Self(45);
    pub const PM_ALT_MANIFEST: Self = Self(46);
    pub const PM_ALT_GAME_MENU: Self = Self(47);
    pub const PM_ALT_NAVIGATE: Self = Self(48);
    pub const PM_ALT_CARGO: Self = Self(49);
    pub const PM_ALT_DEVICES: Self = Self(50);
    pub const PM_ALT_ROSTER: Self = Self(51);
    pub const PM_ALT_EXIT_MANIFEST: Self = Self(52);
    pub const PM_ALT_MSCAN: Self = Self(53);
    pub const PM_ALT_ESCAN: Self = Self(54);
    pub const PM_ALT_BSCAN: Self = Self(55);
    pub const PM_ALT_ASCAN: Self = Self(56);
    pub const PM_ALT_DISPATCH: Self = Self(57);
    pub const PM_ALT_EXIT_SCAN: Self = Self(58);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_12(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_12 {
    pub const STAR_STRING_BASE: Self = Self(0);
    pub const DEVICE_STRING_BASE: Self = Self(149);
    pub const CARGO_STRING_BASE: Self = Self(178);
    pub const ELEMENTS_STRING_BASE: Self = Self(192);
    pub const SCAN_STRING_BASE: Self = Self(325);
    pub const STAR_NUMBER_BASE: Self = Self(391);
    pub const PLANET_NUMBER_BASE: Self = Self(405);
    pub const MONTHS_STRING_BASE: Self = Self(448);
    pub const FEEDBACK_STRING_BASE: Self = Self(460);
    pub const STARBASE_STRING_BASE: Self = Self(468);
    pub const ENCOUNTER_STRING_BASE: Self = Self(510);
    pub const NAVIGATION_STRING_BASE: Self = Self(518);
    pub const NAMING_STRING_BASE: Self = Self(527);
    pub const MELEE_STRING_BASE: Self = Self(535);
    pub const SAVEGAME_STRING_BASE: Self = Self(560);
    pub const OPTION_STRING_BASE: Self = Self(570);
    pub const QUITMENU_STRING_BASE: Self = Self(580);
    pub const STATUS_STRING_BASE: Self = Self(585);
    pub const FLAGSHIP_STRING_BASE: Self = Self(607);
    pub const ORBITSCAN_STRING_BASE: Self = Self(620);
    pub const MAINMENU_STRING_BASE: Self = Self(639);
    pub const NETMELEE_STRING_BASE: Self = Self(713);
    pub const BIOLOGICAL_STRING_BASE: Self = Self(747);
    pub const PLAYMENU_STRING_BASE: Self = Self(773);
    pub const LABEL_STRING_BASE: Self = Self(803);
    pub const GAMESTR_COUNT: Self = Self(829);
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
unsafe extern "C" fn LockShipFrag(
    mut pq: *const QUEUE,
    mut h: HSHIPFRAG,
) -> *mut SHIP_FRAGMENT {
    ((*pq).object_size as usize == ::core::mem::size_of::<SHIP_FRAGMENT>()
        || {
            __assert_fail(
                b"GetLinkSize (pq) == sizeof (SHIP_FRAGMENT)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/uqm/races.h\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                340 as ::core::ffi::c_int,
                b"LockShipFrag\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    return LockLink(pq, h) as *mut SHIP_FRAGMENT;
}
#[inline]
unsafe extern "C" fn LockFleetInfo(
    mut pq: *const QUEUE,
    mut h: HFLEETINFO,
) -> *mut FLEET_INFO {
    ((*pq).object_size as usize == ::core::mem::size_of::<FLEET_INFO>()
        || {
            __assert_fail(
                b"GetLinkSize (pq) == sizeof (FLEET_INFO)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/uqm/races.h\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                410 as ::core::ffi::c_int,
                b"LockFleetInfo\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    return LockLink(pq, h) as *mut FLEET_INFO;
}
unsafe extern "C" fn DoSelectAction(mut pMS: *mut MENU_STATE) -> BOOLEAN {
    SetMenuSounds(
        (((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as MENU_SOUND_FLAGS
            as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as MENU_SOUND_FLAGS
                as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as MENU_SOUND_FLAGS
                as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as MENU_SOUND_FLAGS
                as ::core::ffi::c_int) as MENU_SOUND_FLAGS,
        ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as MENU_SOUND_FLAGS,
    );
    if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
        & C2Rust_Unnamed_1::CHECK_ABORT.0 as ::core::ffi::c_int != 0
    {
        (*pMS).CurState = (C2Rust_Unnamed_10::ATTACK.0 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int) as BYTE;
        return BOOLEAN::FALSE;
    }
    if (*pMS).Initialized == 0 {
        (*pMS).Initialized = BOOLEAN::TRUE.0 as ::core::ffi::c_int as SIZE;
        (*pMS).InputFunc = Some(
            DoSelectAction as unsafe extern "C" fn(*mut MENU_STATE) -> BOOLEAN,
        ) as Option<unsafe extern "C" fn(*mut menu_state) -> BOOLEAN>;
    } else if PulsedInputState
        .menu[C2Rust_Unnamed_0::KEY_MENU_SELECT.0 as ::core::ffi::c_int as usize] != 0
    {
        match (*pMS).CurState as ::core::ffi::c_int {
            0 | 1 => {
                uqm_lua_checked!(DrawMenuStateStrings(
                    C2Rust_Unnamed_11::PM_CONVERSE.0 as ::core::ffi::c_int as BYTE,
                    (*pMS).CurState as SWORD,
                ), BOOLEAN(0));
                if GlobData.Game_state.CurrentActivity as BYTE as ::core::ffi::c_int
                    == C2Rust_Unnamed_1::IN_LAST_BATTLE.0 as ::core::ffi::c_int
                {
                    (*pMS).CurState = C2Rust_Unnamed_10::HAIL.0 as ::core::ffi::c_int
                        as BYTE;
                }
                return BOOLEAN::FALSE;
            }
            2 => {
                if uqm_lua_checked!(GameOptions(), BOOLEAN(0)).0 == 0 {
                    return BOOLEAN::FALSE;
                }
                uqm_lua_checked!(DrawMenuStateStrings(
                    C2Rust_Unnamed_11::PM_CONVERSE.0 as ::core::ffi::c_int as BYTE,
                    (*pMS).CurState as SWORD,
                ), BOOLEAN(0));
                uqm_lua_checked!(SetFlashRect(
                    ::core::ptr::with_exposed_provenance_mut::<
                        RECT,
                    >(!(0 as ::core::ffi::c_long) as usize),
                    BOOLEAN::FALSE,
                ), BOOLEAN(0));
            }
            _ => {
                printf(
                    b"Unknown option: %d\n\0".as_ptr() as *const ::core::ffi::c_char,
                    (*pMS).CurState as ::core::ffi::c_int,
                );
            }
        }
    }
    uqm_lua_checked!(DoMenuChooser(pMS, C2Rust_Unnamed_11::PM_CONVERSE.0 as ::core::ffi::c_int as BYTE), BOOLEAN(0));
    return BOOLEAN::TRUE;
}
unsafe extern "C" fn GetShipFragQueueForPlayer(mut playerNr: COUNT) -> *mut QUEUE {
    if playerNr as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        return &raw mut GlobData.Game_state.built_ship_q
    } else {
        return &raw mut GlobData.Game_state.npc_built_ship_q
    };
}
#[no_mangle]
pub unsafe extern "C" fn SetBattlePlanet() {
    let mut selector: BYTE = 0;
    selector = (TFB_Random() as COUNT as ::core::ffi::c_int
        % C2Rust_Unnamed_2::NUMBER_OF_PLANET_TYPES.0 as ::core::ffi::c_int) as BYTE;
    if if GlobData.SIS_state.Extended as ::core::ffi::c_int != 0 {
        BOOLEAN::TRUE.0 as ::core::ffi::c_int
    } else {
        BOOLEAN::FALSE.0 as ::core::ffi::c_int
    } != 0
        && (selector as ::core::ffi::c_int
            == C2Rust_Unnamed_2::RAINBOW_WORLD.0 as ::core::ffi::c_int
            || selector as ::core::ffi::c_int
                == C2Rust_Unnamed_2::SHATTERED_WORLD.0 as ::core::ffi::c_int)
    {
        if selector as ::core::ffi::c_int
            == C2Rust_Unnamed_2::RAINBOW_WORLD.0 as ::core::ffi::c_int
        {
            selector = selector.wrapping_sub(1);
        }
        if selector as ::core::ffi::c_int
            == C2Rust_Unnamed_2::SHATTERED_WORLD.0 as ::core::ffi::c_int
        {
            selector = selector.wrapping_add(1);
        }
    }
    uqm_lua_checked!(setGameStateUint(
        b"BATTLE_PLANET\0".as_ptr() as *const ::core::ffi::c_char,
        selector as DWORD,
    ), ());
}
#[no_mangle]
pub unsafe extern "C" fn BuildBattle(mut which_player: COUNT) {
    let mut pQueue: *mut QUEUE = ::core::ptr::null_mut::<QUEUE>();
    let mut hStarShip: HSHIPFRAG = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut hNextShip: HSHIPFRAG = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut hBuiltShip: HSTARSHIP = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut BuiltShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
    EncounterRace = -1 as SIZE;
    if GlobData.Game_state.npc_built_ship_q.head.is_null() {
        uqm_lua_checked!(setGameStateUint(
            b"BATTLE_SEGUE\0".as_ptr() as *const ::core::ffi::c_char,
            0 as DWORD,
        ), ());
        return;
    }
    if which_player as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
        match GlobData.Game_state.CurrentActivity as BYTE as ::core::ffi::c_int {
            1 => {
                uqm_lua_checked!(load_gravity_well(
                    C2Rust_Unnamed_2::SA_MATRA.0 as ::core::ffi::c_int as BYTE,
                ), ());
            }
            3 => {
                if uqm_lua_checked!(getGameStateUint(
                    b"BATTLE_PLANET\0".as_ptr() as *const ::core::ffi::c_char,
                ), ())
                    > C2Rust_Unnamed_2::NUMBER_OF_PLANET_TYPES.0 as ::core::ffi::c_int
                        as DWORD
                {
                    uqm_lua_checked!(SetBattlePlanet(), ());
                }
                uqm_lua_checked!(load_gravity_well(
                    uqm_lua_checked!(getGameStateUint(
                        b"BATTLE_PLANET\0".as_ptr() as *const ::core::ffi::c_char,
                    ), ()) as BYTE,
                ), ());
            }
            _ => {
                uqm_lua_checked!(setGameStateUint(
                    b"ESCAPE_COUNTER\0".as_ptr() as *const ::core::ffi::c_char,
                    110 as DWORD,
                ), ());
                if if GlobData.SIS_state.Extended as ::core::ffi::c_int != 0 {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0 && !pSolarSysState.is_null()
                    && worldIsMoon(pSolarSysState, (*pSolarSysState).pOrbitalDesc)
                        as ::core::ffi::c_int != 0
                {
                    let mut selector: BYTE = (*pSolarSysState)
                        .MoonDesc[moonIndex(
                            pSolarSysState,
                            (*pSolarSysState).pOrbitalDesc,
                        ) as usize]
                        .data_index;
                    if selector as ::core::ffi::c_int
                        == C2Rust_Unnamed_2::SA_MATRA.0 as ::core::ffi::c_int
                    {
                        utf8StringCopy(
                            &raw mut GlobData.SIS_state.PlanetName
                                as *mut ::core::ffi::c_char,
                            ::core::mem::size_of::<[UNICODE; 32]>() as size_t,
                            GetStringAddress(
                                SetAbsStringTableIndex(
                                    GameStrings,
                                    (C2Rust_Unnamed_12::PLANET_NUMBER_BASE.0
                                        as ::core::ffi::c_int + 32 as ::core::ffi::c_int) as COUNT,
                                ),
                            ),
                        );
                        uqm_lua_checked!(DrawSISTitle(
                            GetStringAddress(
                                SetAbsStringTableIndex(
                                    GameStrings,
                                    (C2Rust_Unnamed_12::PLANET_NUMBER_BASE.0
                                        as ::core::ffi::c_int + 32 as ::core::ffi::c_int) as COUNT,
                                ),
                            ),
                        ), ());
                    }
                    uqm_lua_checked!(load_gravity_well(selector), ());
                } else {
                    uqm_lua_checked!(load_gravity_well(
                        uqm_lua_checked!(getGameStateUint(
                            b"BATTLE_PLANET\0".as_ptr() as *const ::core::ffi::c_char,
                        ), ()) as BYTE,
                    ), ());
                }
            }
        }
    }
    pQueue = GetShipFragQueueForPlayer(which_player);
    ReinitQueue((&raw mut race_q as *mut QUEUE).offset(which_player as isize));
    hStarShip = (*pQueue).head as HSHIPFRAG;
    while !hStarShip.is_null() {
        let mut FragPtr: *mut SHIP_FRAGMENT = ::core::ptr::null_mut::<SHIP_FRAGMENT>();
        FragPtr = LockShipFrag(pQueue, hStarShip);
        hNextShip = (*FragPtr).succ as HSHIPFRAG;
        hBuiltShip = Build(
            (&raw mut race_q as *mut QUEUE).offset(which_player as isize),
            SPECIES_ID(
                if (*FragPtr).race_id as ::core::ffi::c_int
                    == RACE_ID::SAMATRA_SHIP.0 as ::core::ffi::c_int
                {
                    SPECIES_ID::SA_MATRA_ID.0
                } else {
                    (*FragPtr).SpeciesID.0
                },
            ),
        ) as HSTARSHIP;
        if !hBuiltShip.is_null() {
            BuiltShipPtr = LockStarShip(
                (&raw mut race_q as *mut QUEUE).offset(which_player as isize),
                hBuiltShip,
            );
            (*BuiltShipPtr).captains_name_index = (*FragPtr).captains_name_index;
            (*BuiltShipPtr).playerNr = which_player as SIZE;
            if (*FragPtr).crew_level as ::core::ffi::c_int
                != !(0 as ::core::ffi::c_int) as COUNT as ::core::ffi::c_int
            {
                (*BuiltShipPtr).crew_level = (*FragPtr).crew_level;
            } else {
                (*BuiltShipPtr).crew_level = (*FragPtr).max_crew;
            }
            (*BuiltShipPtr).max_crew = (*FragPtr).max_crew;
            (*BuiltShipPtr).race_strings = (*FragPtr).race_strings;
            (*BuiltShipPtr).icons = (*FragPtr).icons;
            (*BuiltShipPtr).index = (*FragPtr).index as COUNT;
            (*BuiltShipPtr).ship_cost = 0 as BYTE;
            (*BuiltShipPtr).RaceDescPtr = ::core::ptr::null_mut::<RACE_DESC>();
            UnlockLink(
                (&raw mut race_q as *mut QUEUE).offset(which_player as isize),
                hBuiltShip,
            );
        }
        UnlockLink(pQueue, hStarShip);
        hStarShip = hNextShip;
    }
    if which_player as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        && {
            hBuiltShip = Build(
                (&raw mut race_q as *mut QUEUE).offset(0isize),
                SPECIES_ID::SIS_SHIP_ID,
            ) as HSTARSHIP;
            !hBuiltShip.is_null()
        }
    {
        BuiltShipPtr = LockStarShip(
            (&raw mut race_q as *mut QUEUE).offset(0isize),
            hBuiltShip,
        );
        (*BuiltShipPtr).captains_name_index = 0 as BYTE;
        (*BuiltShipPtr).playerNr = 0 as SIZE;
        (*BuiltShipPtr).crew_level = 0 as COUNT;
        (*BuiltShipPtr).max_crew = 0 as COUNT;
        (*BuiltShipPtr).race_strings = ::core::ptr::null_mut::<
            STRING_TABLE_ENTRY_DESC,
        >();
        (*BuiltShipPtr).icons = ::core::ptr::null_mut::<FRAME_DESC>();
        (*BuiltShipPtr).index = -1 as ::core::ffi::c_int as COUNT;
        (*BuiltShipPtr).ship_cost = 0 as BYTE;
        (*BuiltShipPtr).energy_counter = 42 as BYTE;
        (*BuiltShipPtr).RaceDescPtr = ::core::ptr::null_mut::<RACE_DESC>();
        UnlockLink((&raw mut race_q as *mut QUEUE).offset(0isize), hBuiltShip);
    }
}
#[no_mangle]
pub unsafe extern "C" fn FleetIsInfinite(mut playerNr: COUNT) -> BOOLEAN {
    let mut pQueue: *mut QUEUE = ::core::ptr::null_mut::<QUEUE>();
    let mut hShipFrag: HSHIPFRAG = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut FragPtr: *mut SHIP_FRAGMENT = ::core::ptr::null_mut::<SHIP_FRAGMENT>();
    let mut ret: BOOLEAN = BOOLEAN::FALSE;
    pQueue = GetShipFragQueueForPlayer(playerNr);
    hShipFrag = (*pQueue).head as HSHIPFRAG;
    if hShipFrag.is_null() {
        return BOOLEAN::FALSE;
    }
    FragPtr = LockShipFrag(pQueue, hShipFrag);
    ret = BOOLEAN(
        ((*FragPtr).crew_level as ::core::ffi::c_int
            == !(0 as ::core::ffi::c_int) as COUNT as ::core::ffi::c_int)
            as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
    UnlockLink(pQueue, hShipFrag);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn UpdateShipFragCrew(mut StarShipPtr: *mut STARSHIP) {
    let mut frag_q: *mut QUEUE = ::core::ptr::null_mut::<QUEUE>();
    let mut hShipFrag: HSHIPFRAG = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut hNextFrag: HSHIPFRAG = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut frag: *mut SHIP_FRAGMENT = ::core::ptr::null_mut::<SHIP_FRAGMENT>();
    let mut ship_q: *mut QUEUE = ::core::ptr::null_mut::<QUEUE>();
    let mut hStarShip: HSTARSHIP = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut hNextShip: HSTARSHIP = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut ship: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
    frag_q = GetShipFragQueueForPlayer((*StarShipPtr).playerNr as COUNT);
    ship_q = (&raw mut race_q as *mut QUEUE).offset((*StarShipPtr).playerNr as isize);
    hShipFrag = (*frag_q).head as HSHIPFRAG;
    hStarShip = (*ship_q).head as HSTARSHIP;
    while !hShipFrag.is_null() && !hStarShip.is_null() {
        ship = LockStarShip(ship_q, hStarShip);
        hNextShip = (*ship).succ as HSTARSHIP;
        frag = LockShipFrag(frag_q, hShipFrag);
        hNextFrag = (*frag).succ as HSHIPFRAG;
        if ship == StarShipPtr {
            ((*frag).crew_level as ::core::ffi::c_int
                != !(0 as ::core::ffi::c_int) as COUNT as ::core::ffi::c_int
                || {
                    __assert_fail(
                        b"frag->crew_level != INFINITE_FLEET\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"src/uqm/encount.c\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        300 as ::core::ffi::c_int,
                        b"UpdateShipFragCrew\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    0 as ::core::ffi::c_int != 0
                }) as ::core::ffi::c_int;
            (*frag).crew_level = (*ship).crew_level;
            UnlockLink(frag_q, hShipFrag);
            UnlockLink(ship_q, hStarShip);
            break;
        } else {
            UnlockLink(frag_q, hShipFrag);
            UnlockLink(ship_q, hStarShip);
            hShipFrag = hNextFrag;
            hStarShip = hNextShip;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn InitEncounter() -> COUNT {
    let mut i: COUNT = 0;
    let mut SegueFrame: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
    let mut s: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    let mut t: TEXT = TEXT {
        baseline: POINT { x: 0, y: 0 },
        pStr: ::core::ptr::null::<UNICODE>(),
        align: TEXT_ALIGN::ALIGN_LEFT,
        CharCount: 0,
    };
    extern "C" {
        static mut planet: [FRAME; 0];
    }
    let mut MR: MUSIC_REF = ::core::ptr::null_mut::<*mut TFB_SoundSample>();
    SetContext(SpaceContext);
    SetContextFont(MicroFont);
    MR = uqm_lua_checked!(LoadMusicInstance(b"music.redalert\0".as_ptr() as RESOURCE), 0);
    uqm_lua_checked!(PlayMusic(MR, BOOLEAN::FALSE, 1 as BYTE), 0);
    SegueFrame = uqm_lua_checked!(GetStarBackGround(BOOLEAN::TRUE), 0);
    uqm_lua_checked!(WaitForSoundEnd(!(0 as ::core::ffi::c_int) as COUNT), 0);
    uqm_lua_checked!(StopMusic(), 0);
    uqm_lua_checked!(DestroyMusic(MR), 0);
    s.origin.y = 0 as COORD;
    s.origin.x = s.origin.y;
    SetTransitionSource(::core::ptr::null::<RECT>());
    BatchGraphics();
    SetContextBackGroundColor(colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE));
    uqm_lua_checked!(ClearDrawable(), 0);
    s.frame = SegueFrame;
    uqm_lua_checked!(DrawStamp(&raw mut s), 0);
    t.baseline.x = ((CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
            } << resolutionFactor)) >> resolutionFactor >> 1 as ::core::ffi::c_int)
        << resolutionFactor) as COORD;
    t.baseline.y = ((10 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    t.align = TEXT_ALIGN::ALIGN_CENTER;
    SetContextForeGroundColor(colorFromRgb15(0 as BYTE, 0 as BYTE, 0x14 as BYTE));
    if inHQSpace().0 != 0 {
        t.pStr = GetStringAddress(
            SetAbsStringTableIndex(
                GameStrings,
                (C2Rust_Unnamed_12::ENCOUNTER_STRING_BASE.0 as ::core::ffi::c_int
                    + 0 as ::core::ffi::c_int) as COUNT,
            ),
        );
        t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
        uqm_lua_checked!(font_DrawText(&raw mut t), 0);
        t.baseline.y = (t.baseline.y as ::core::ffi::c_int
            + ((12 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        t.pStr = GetStringAddress(
            SetAbsStringTableIndex(
                GameStrings,
                (C2Rust_Unnamed_12::ENCOUNTER_STRING_BASE.0 as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int) as COUNT,
            ),
        );
        t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
        uqm_lua_checked!(font_DrawText(&raw mut t), 0);
    } else {
        let mut buf: [UNICODE; 256] = [0; 256];
        t.pStr = GetStringAddress(
            SetAbsStringTableIndex(
                GameStrings,
                (C2Rust_Unnamed_12::ENCOUNTER_STRING_BASE.0 as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int) as COUNT,
            ),
        );
        t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
        uqm_lua_checked!(font_DrawText(&raw mut t), 0);
        t.baseline.y = (t.baseline.y as ::core::ffi::c_int
            + ((12 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        GetClusterName(CurStarDescPtr, &raw mut buf as *mut UNICODE);
        t.pStr = &raw mut buf as *mut UNICODE;
        t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
        uqm_lua_checked!(font_DrawText(&raw mut t), 0);
        t.baseline.y = (t.baseline.y as ::core::ffi::c_int
            + ((12 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        t.pStr = &raw mut GlobData.SIS_state.PlanetName as *mut UNICODE;
        t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
        uqm_lua_checked!(font_DrawText(&raw mut t), 0);
    }
    uqm_lua_checked!(DrawSISMessage(::core::ptr::null::<UNICODE>()), 0);
    s.origin.x = ((CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
            } << resolutionFactor)) >> resolutionFactor >> 1 as ::core::ffi::c_int)
        << resolutionFactor) as COORD;
    s.origin.y = (CanvasHeight
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
        - ((13 as ::core::ffi::c_int) << resolutionFactor) >> 1 as ::core::ffi::c_int)
        as COORD;
    s.frame = *(&raw mut planet as *mut FRAME).offset(0isize);
    uqm_lua_checked!(DrawStamp(&raw mut s), 0);
    if GlobData.Game_state.CurrentActivity as BYTE as ::core::ffi::c_int
        != C2Rust_Unnamed_1::IN_LAST_BATTLE.0 as ::core::ffi::c_int
    {
        let mut hStarShip: HSHIPFRAG = ::core::ptr::null_mut::<::core::ffi::c_void>();
        let mut hNextShip: HSHIPFRAG = ::core::ptr::null_mut::<::core::ffi::c_void>();
        let mut display_pt: [POINT; 10] = [
            point {
                x: 10 as COORD,
                y: 51 as COORD,
            },
            point {
                x: -10 as COORD,
                y: 51 as COORD,
            },
            point {
                x: 33 as COORD,
                y: 40 as COORD,
            },
            point {
                x: -33 as COORD,
                y: 40 as COORD,
            },
            point {
                x: 49 as COORD,
                y: 18 as COORD,
            },
            point {
                x: -49 as COORD,
                y: 18 as COORD,
            },
            point {
                x: 52 as COORD,
                y: -6 as COORD,
            },
            point {
                x: -52 as COORD,
                y: -6 as COORD,
            },
            point {
                x: 44 as COORD,
                y: -27 as COORD,
            },
            point {
                x: -44 as COORD,
                y: -27 as COORD,
            },
        ];
        hStarShip = GlobData.Game_state.npc_built_ship_q.head as HSHIPFRAG;
        i = 0 as COUNT;
        while !hStarShip.is_null()
            && (i as ::core::ffi::c_int) < 60 as ::core::ffi::c_int
        {
            let mut r: RECT = RECT {
                corner: POINT { x: 0, y: 0 },
                extent: EXTENT { width: 0, height: 0 },
            };
            let mut FragPtr: *mut SHIP_FRAGMENT = ::core::ptr::null_mut::<
                SHIP_FRAGMENT,
            >();
            FragPtr = LockShipFrag(
                &raw mut GlobData.Game_state.npc_built_ship_q,
                hStarShip,
            );
            if (*FragPtr).crew_level as ::core::ffi::c_int
                != !(0 as ::core::ffi::c_int) as COUNT as ::core::ffi::c_int
            {
                hNextShip = (*FragPtr).succ as HSHIPFRAG;
            } else {
                hNextShip = hStarShip;
            }
            s.origin = display_pt[(i as usize)
                .wrapping_rem(
                    ::core::mem::size_of::<[POINT; 10]>()
                        .wrapping_div(::core::mem::size_of::<POINT>()),
                )];
            if i as usize
                >= ::core::mem::size_of::<[POINT; 10]>()
                    .wrapping_div(::core::mem::size_of::<POINT>())
            {
                let mut angle: COUNT = 0;
                let mut radius: COUNT = 0;
                radius = (square_root(
                    (s.origin.x as ::core::ffi::c_long
                        * s.origin.x as ::core::ffi::c_long
                        + s.origin.y as ::core::ffi::c_long
                            * s.origin.y as ::core::ffi::c_long) as DWORD,
                ) as usize)
                    .wrapping_add(
                        (i as usize)
                            .wrapping_div(
                                ::core::mem::size_of::<[POINT; 10]>()
                                    .wrapping_div(::core::mem::size_of::<POINT>()),
                            )
                            .wrapping_mul(18usize),
                    ) as COUNT;
                radius = ((radius as ::core::ffi::c_int) << resolutionFactor) as COUNT;
                angle = ARCTAN(s.origin.x as SDWORD, s.origin.y as SDWORD);
                s.origin.x = (*(&raw mut sinetab as *mut SDWORD)
                    .offset(
                        (angle as ::core::ffi::c_int
                            + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                                >> 2 as ::core::ffi::c_int)
                            & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                                - 1 as ::core::ffi::c_int) as DWORD as isize,
                    ) as ::core::ffi::c_long * radius as ::core::ffi::c_long
                    >> 14 as ::core::ffi::c_int) as SDWORD as COORD;
                s.origin.y = (*(&raw mut sinetab as *mut SDWORD)
                    .offset(
                        (angle as ::core::ffi::c_int
                            & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                                - 1 as ::core::ffi::c_int) as DWORD as isize,
                    ) as ::core::ffi::c_long * radius as ::core::ffi::c_long
                    >> 14 as ::core::ffi::c_int) as SDWORD as COORD;
            } else {
                s.origin.x = ((s.origin.x as ::core::ffi::c_int) << resolutionFactor)
                    as COORD;
                s.origin.y = ((s.origin.y as ::core::ffi::c_int) << resolutionFactor)
                    as COORD;
            }
            s.frame = SetAbsFrameIndex((*FragPtr).icons, 0 as COUNT);
            GetFrameRect(s.frame, &raw mut r);
            s.origin.x = (s.origin.x as ::core::ffi::c_int
                + (((CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
                        } << resolutionFactor)) >> resolutionFactor
                    >> 1 as ::core::ffi::c_int) << resolutionFactor)
                    - (r.extent.width as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)))
                as COORD;
            s.origin.y = (s.origin.y as ::core::ffi::c_int
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
                    } << resolutionFactor) * 2 as ::core::ffi::c_int
                    - ((13 as ::core::ffi::c_int) << resolutionFactor)
                    >> resolutionFactor >> 1 as ::core::ffi::c_int) << resolutionFactor)
                    - (r.extent.height as ::core::ffi::c_int
                        >> 1 as ::core::ffi::c_int))) as COORD;
            uqm_lua_checked!(DrawStamp(&raw mut s), 0);
            UnlockLink(&raw mut GlobData.Game_state.npc_built_ship_q, hStarShip);
            hStarShip = hNextShip;
            i = i.wrapping_add(1);
        }
    }
    UnbatchGraphics();
    DestroyDrawable(ReleaseDrawable(SegueFrame));
    uqm_lua_checked!(ScreenTransition(optScrTrans, ::core::ptr::null::<RECT>()), 0);
    let mut MenuState: MENU_STATE = MENU_STATE {
        InputFunc: None,
        Initialized: 0,
        CurState: 0,
        CurFrame: ::core::ptr::null_mut::<FRAME_DESC>(),
        CurString: ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>(),
        first_item: POINT { x: 0, y: 0 },
        delta_item: 0,
        ModuleFrame: ::core::ptr::null_mut::<FRAME_DESC>(),
        flash_rect0: RECT {
            corner: POINT { x: 0, y: 0 },
            extent: EXTENT { width: 0, height: 0 },
        },
        flash_rect1: RECT {
            corner: POINT { x: 0, y: 0 },
            extent: EXTENT { width: 0, height: 0 },
        },
        flash_rect2: RECT {
            corner: POINT { x: 0, y: 0 },
            extent: EXTENT { width: 0, height: 0 },
        },
        flash_frame0: ::core::ptr::null_mut::<FRAME_DESC>(),
        flash_frame1: ::core::ptr::null_mut::<FRAME_DESC>(),
        flashContext: ::core::ptr::null_mut::<FlashContext>(),
        hMusic: ::core::ptr::null_mut::<*mut TFB_SoundSample>(),
        privData: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    MenuState.InputFunc = Some(
        DoSelectAction as unsafe extern "C" fn(*mut MENU_STATE) -> BOOLEAN,
    ) as Option<unsafe extern "C" fn(*mut menu_state) -> BOOLEAN>;
    MenuState.Initialized = BOOLEAN::FALSE.0 as ::core::ffi::c_int as SIZE;
    MenuState.CurState = C2Rust_Unnamed_10::HAIL.0 as ::core::ffi::c_int as BYTE;
    uqm_lua_checked!(DrawMenuStateStrings(
        C2Rust_Unnamed_11::PM_CONVERSE.0 as ::core::ffi::c_int as BYTE,
        MenuState.CurState as SWORD,
    ), 0);
    uqm_lua_checked!(SetFlashRect(
        ::core::ptr::with_exposed_provenance_mut::<RECT>(!(0 as ::core::ffi::c_long) as usize),
        BOOLEAN::FALSE,
    ), 0);
    uqm_lua_checked!(DoInput(&raw mut MenuState as *mut ::core::ffi::c_void, BOOLEAN::TRUE), 0);
    uqm_lua_checked!(SetFlashRect(::core::ptr::null::<RECT>(), BOOLEAN::FALSE), 0);
    return MenuState.CurState as COUNT;
}
unsafe extern "C" fn DrawFadeText(
    mut str1: *const UNICODE,
    mut str2: *const UNICODE,
    mut fade_in: BOOLEAN,
    mut pRect: *mut RECT,
) {
    let mut i: SIZE = 0;
    let mut TimeIn: DWORD = 0;
    let mut t1: TEXT = TEXT {
        baseline: POINT { x: 0, y: 0 },
        pStr: ::core::ptr::null::<UNICODE>(),
        align: TEXT_ALIGN::ALIGN_LEFT,
        CharCount: 0,
    };
    let mut t2: TEXT = TEXT {
        baseline: POINT { x: 0, y: 0 },
        pStr: ::core::ptr::null::<UNICODE>(),
        align: TEXT_ALIGN::ALIGN_LEFT,
        CharCount: 0,
    };
    let mut r1: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut r2: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    static mut fade_cycle: [Color; 7] = [
        Color {
            r: ((0xa as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0xa as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0xa as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0xa as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0xa as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0xa as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
        Color {
            r: ((0x9 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x9 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0x9 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x9 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0x9 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x9 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
        Color {
            r: ((0x8 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x8 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0x8 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x8 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0x8 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x8 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
        Color {
            r: ((0x6 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x6 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0x6 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x6 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0x6 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x6 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
        Color {
            r: ((0x5 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x5 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0x5 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x5 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0x5 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x5 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
        Color {
            r: ((0x4 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x4 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0x4 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x4 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0x4 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x4 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
        Color {
            r: ((0x3 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x3 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0x3 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x3 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0x3 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x3 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
    ];
    t1.baseline.x = ((*pRect).corner.x as ::core::ffi::c_int
        + ((100 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    t1.baseline.y = ((*pRect).corner.y as ::core::ffi::c_int
        + ((45 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    t1.align = TEXT_ALIGN::ALIGN_CENTER;
    t1.pStr = str1;
    t1.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    t2 = t1;
    t2.baseline.y = (t2.baseline.y as ::core::ffi::c_int
        + ((11 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    t2.pStr = str2;
    TextRect(&raw mut t1, &raw mut r1, ::core::ptr::null_mut::<BYTE>());
    TextRect(&raw mut t2, &raw mut r2, ::core::ptr::null_mut::<BYTE>());
    FlushInput();
    TimeIn = GetTimeCounter() as DWORD;
    i = 0 as SIZE;
    while (i as ::core::ffi::c_int)
        < ::core::mem::size_of::<[Color; 7]>()
            .wrapping_div(::core::mem::size_of::<Color>()) as SIZE as ::core::ffi::c_int
    {
        if uqm_lua_checked!(AnyButtonPress(BOOLEAN::TRUE), ()).0 != 0 {
            i = ::core::mem::size_of::<[Color; 7]>()
                .wrapping_div(::core::mem::size_of::<Color>())
                .wrapping_sub(1usize) as SIZE;
        }
        if if resolutionFactor != 2 as ::core::ffi::c_uint {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } != 0
        {
            uqm_lua_checked!(RepairPickFrame(&raw mut r1, 1 as COUNT), ());
            uqm_lua_checked!(RepairPickFrame(&raw mut r2, 1 as COUNT), ());
        }
        SetContextForeGroundColor(
            fade_cycle[(if fade_in.0 != 0 {
                i as ::core::ffi::c_int
            } else {
                ::core::mem::size_of::<[Color; 7]>()
                    .wrapping_div(::core::mem::size_of::<Color>())
                    .wrapping_sub(i as usize)
                    .wrapping_sub(1usize) as SIZE as ::core::ffi::c_int
            }) as usize],
        );
        uqm_lua_checked!(font_DrawText(&raw mut t1), ());
        uqm_lua_checked!(font_DrawText(&raw mut t2), ());
        uqm_lua_checked!(SleepThreadUntil(
            TimeIn
                .wrapping_add(
                    (840 as ::core::ffi::c_int / 20 as ::core::ffi::c_int) as TimeCount,
                ),
        ), ());
        TimeIn = GetTimeCounter() as DWORD;
        i += 1;
    }
    if fade_in.0 == 0 {
        if if resolutionFactor != 2 as ::core::ffi::c_uint {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } != 0
        {
            uqm_lua_checked!(RepairPickFrame(&raw mut r1, 1 as COUNT), ());
            uqm_lua_checked!(RepairPickFrame(&raw mut r2, 1 as COUNT), ());
        } else {
            SetContextForeGroundColor(
                buildColorRgba(0x50 as BYTE, 0x50 as BYTE, 0x50 as BYTE, 0xff as BYTE),
            );
            uqm_lua_checked!(DrawFilledRectangle(&raw mut r1), ());
            uqm_lua_checked!(DrawFilledRectangle(&raw mut r2), ());
        }
    }
}
unsafe extern "C" fn ClearRectBack(mut pRect: *mut RECT) {
    if if resolutionFactor != 2 as ::core::ffi::c_uint {
        BOOLEAN::FALSE.0 as ::core::ffi::c_int
    } else {
        BOOLEAN::TRUE.0 as ::core::ffi::c_int
    } != 0
    {
        uqm_lua_checked!(RepairPickFrame(pRect, 1 as COUNT), ());
    } else {
        uqm_lua_checked!(DrawFilledRectangle(pRect), ());
    };
}
#[no_mangle]
pub unsafe extern "C" fn UninitEncounter() -> COUNT {
    let mut ships_killed: COUNT = 0;
    ships_killed = 0 as COUNT;
    free_gravity_well();
    if !(GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
        & (C2Rust_Unnamed_1::CHECK_ABORT.0 as ::core::ffi::c_int
            | C2Rust_Unnamed_1::CHECK_LOAD.0 as ::core::ffi::c_int) != 0
        || GlobData.SIS_state.CrewEnlisted as ::core::ffi::c_int
            == !(0 as ::core::ffi::c_int) as COUNT as ::core::ffi::c_int
        || GlobData.Game_state.CurrentActivity as BYTE as ::core::ffi::c_int
            == C2Rust_Unnamed_1::IN_LAST_BATTLE.0 as ::core::ffi::c_int
        || GlobData.Game_state.CurrentActivity as BYTE as ::core::ffi::c_int
            == C2Rust_Unnamed_1::WON_LAST_BATTLE.0 as ::core::ffi::c_int)
    {
        if uqm_lua_checked!(getGameStateUint(b"BATTLE_SEGUE\0".as_ptr() as *const ::core::ffi::c_char), 0)
            == 0 as DWORD
        {
            ReinitQueue((&raw mut race_q as *mut QUEUE).offset(0isize));
            ReinitQueue((&raw mut race_q as *mut QUEUE).offset(1isize));
        } else {
            let mut Sleepy: BOOLEAN = BOOLEAN::FALSE;
            let mut VictoryState: SIZE = 0;
            let mut i: SIZE = 0;
            let mut RecycleAmount: COUNT = 0 as COUNT;
            let mut r: RECT = RECT {
                corner: POINT { x: 0, y: 0 },
                extent: EXTENT { width: 0, height: 0 },
            };
            let mut scavenge_r: RECT = rect {
                corner: point {
                    x: 0 as COORD,
                    y: 0 as COORD,
                },
                extent: extent {
                    width: 0 as COORD,
                    height: 0 as COORD,
                },
            };
            let mut t: TEXT = TEXT {
                baseline: POINT { x: 0, y: 0 },
                pStr: ::core::ptr::null::<UNICODE>(),
                align: TEXT_ALIGN::ALIGN_LEFT,
                CharCount: 0,
            };
            let mut ship_s: STAMP = STAMP {
                origin: POINT { x: 0, y: 0 },
                frame: ::core::ptr::null_mut::<FRAME_DESC>(),
            };
            let mut str1: *const UNICODE = ::core::ptr::null::<UNICODE>();
            let mut str2: *const UNICODE = ::core::ptr::null::<UNICODE>();
            let mut prevMsgMode: StatMsgMode = StatMsgMode::SMM_UNDEFINED;
            let mut buf: [UNICODE; 80] = [0; 80];
            let mut hStarShip: HSHIPFRAG = ::core::ptr::null_mut::<
                ::core::ffi::c_void,
            >();
            let mut FragPtr: *mut SHIP_FRAGMENT = ::core::ptr::null_mut::<
                SHIP_FRAGMENT,
            >();
            static mut fade_ship_cycle: [Color; 10] = [
                Color {
                    r: ((0x7 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                        | 0x7 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
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
                    r: ((0x17 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                        | 0x17 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                    g: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                        | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                    b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                        | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
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
                    g: ((0x14 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                        | 0x14 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                    b: ((0x14 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                        | 0x14 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                    a: 0xff as BYTE,
                },
                Color {
                    r: ((0x1f as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                        | 0x1f as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                    g: ((0x1f as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                        | 0x1f as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                    b: ((0x1f as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                        | 0x1f as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                    a: 0xff as BYTE,
                },
                Color {
                    r: ((0x1f as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                        | 0x1f as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                    g: ((0x14 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                        | 0x14 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                    b: ((0x14 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                        | 0x14 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
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
            ];
            uqm_lua_checked!(setGameStateUint(
                b"BATTLE_SEGUE\0".as_ptr() as *const ::core::ffi::c_char,
                0 as DWORD,
            ), 0);
            uqm_lua_checked!(setGameStateUint(
                b"BOMB_CARRIER\0".as_ptr() as *const ::core::ffi::c_char,
                0 as DWORD,
            ), 0);
            VictoryState = (if battle_counter[1usize] as ::core::ffi::c_int != 0
                || battle_counter[0usize] == 0
                || uqm_lua_checked!(getGameStateUint(
                    b"URQUAN_PROTECTING_SAMATRA\0".as_ptr() as *const ::core::ffi::c_char,
                ), 0) != 0
            {
                0 as ::core::ffi::c_int
            } else {
                1 as ::core::ffi::c_int
            }) as SIZE;
            hStarShip = GlobData.Game_state.npc_built_ship_q.head as HSHIPFRAG;
            FragPtr = LockShipFrag(
                &raw mut GlobData.Game_state.npc_built_ship_q,
                hStarShip,
            );
            EncounterRace = (*FragPtr).race_id as SIZE;
            if GetStarShipFromIndex(
                    &raw mut GlobData.Game_state.avail_race_q,
                    EncounterRace as COUNT,
                )
                .is_null()
            {
                VictoryState = -1 as SIZE;
                InitSISContexts();
            }
            UnlockLink(&raw mut GlobData.Game_state.npc_built_ship_q, hStarShip);
            if VictoryState as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                prevMsgMode = SetStatusMessageMode(StatMsgMode::SMM_RES_UNITS);
            }
            ship_s.origin.x = 0 as COORD;
            ship_s.origin.y = 0 as COORD;
            Sleepy = BOOLEAN::TRUE;
            i = 0 as SIZE;
            while (i as ::core::ffi::c_int) < 2 as ::core::ffi::c_int {
                let mut pQueue: *mut QUEUE = ::core::ptr::null_mut::<QUEUE>();
                let mut hNextShip: HSHIPFRAG = ::core::ptr::null_mut::<
                    ::core::ffi::c_void,
                >();
                if i as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    pQueue = &raw mut GlobData.Game_state.built_ship_q;
                } else {
                    if (VictoryState as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                        VictoryState = 0 as SIZE;
                    } else {
                        uqm_lua_checked!(DrawSISFrame(), 0);
                        uqm_lua_checked!(DrawSISMessage(::core::ptr::null::<UNICODE>()), 0);
                        if inHQSpace().0 != 0 {
                            uqm_lua_checked!(DrawHyperCoords(GlobData.Game_state.ShipStamp.origin), 0);
                        } else if GlobData.Game_state.ip_planet as ::core::ffi::c_int
                            == 0 as ::core::ffi::c_int
                        {
                            uqm_lua_checked!(DrawHyperCoords((*CurStarDescPtr).star_pt), 0);
                        } else {
                            uqm_lua_checked!(DrawSISTitle(
                                &raw mut GlobData.SIS_state.PlanetName as *mut UNICODE,
                            ), 0);
                        }
                        SetContext(SpaceContext);
                        if VictoryState != 0 {
                            uqm_lua_checked!(InitPickFrame(), 0);
                            uqm_lua_checked!(DrawArmadaPickShip(BOOLEAN::TRUE, &raw mut scavenge_r), 0);
                        }
                    }
                    pQueue = &raw mut GlobData.Game_state.npc_built_ship_q;
                }
                ReinitQueue(
                    (&raw mut race_q as *mut QUEUE)
                        .offset(
                            (2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                                - i as ::core::ffi::c_int) as isize,
                        ),
                );
                hStarShip = (*pQueue).head as HSHIPFRAG;
                while !hStarShip.is_null() {
                    FragPtr = LockShipFrag(pQueue, hStarShip);
                    hNextShip = (*FragPtr).succ as HSHIPFRAG;
                    if (*FragPtr).crew_level as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                        || VictoryState as ::core::ffi::c_int != 0
                            && i as ::core::ffi::c_int
                                == 2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                    {
                        if i as ::core::ffi::c_int
                            == 2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                        {
                            ships_killed = ships_killed.wrapping_add(1);
                            if VictoryState != 0 {
                                let mut j: COUNT = 0;
                                if ships_killed as ::core::ffi::c_int
                                    == 1 as ::core::ffi::c_int
                                {
                                    RecycleAmount = 0 as COUNT;
                                    uqm_lua_checked!(DrawStatusMessage(::core::ptr::null::<UNICODE>()), 0);
                                    ship_s.origin.x = (scavenge_r.corner.x as ::core::ffi::c_int
                                        + ((32 as ::core::ffi::c_int) << resolutionFactor))
                                        as COORD;
                                    ship_s.origin.y = (scavenge_r.corner.y as ::core::ffi::c_int
                                        + ((56 as ::core::ffi::c_int) << resolutionFactor))
                                        as COORD;
                                    ship_s.frame = IncFrameIndex((*FragPtr).icons);
                                    uqm_lua_checked!(DrawStamp(&raw mut ship_s), 0);
                                    SetContextForeGroundColor(
                                        colorFromRgb15(0x8 as BYTE, 0x8 as BYTE, 0x8 as BYTE),
                                    );
                                    SetContextFont(
                                        if isPC(optWhichFonts).0 != 0 {
                                            TinyFont
                                        } else {
                                            TinyFontBold
                                        },
                                    );
                                    utf8StringCopy(
                                        &raw mut buf as *mut ::core::ffi::c_char,
                                        ::core::mem::size_of::<[UNICODE; 80]>() as size_t,
                                        GetStringAddress((*FragPtr).race_strings)
                                            as *const ::core::ffi::c_char,
                                    );
                                    strupr(&raw mut buf as *mut ::core::ffi::c_char);
                                    let mut ptr: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<
                                        ::core::ffi::c_uchar,
                                    >();
                                    ptr = &raw mut buf as *mut UNICODE
                                        as *mut ::core::ffi::c_uchar;
                                    while *ptr != 0 {
                                        if *ptr as ::core::ffi::c_int == 0xc3 as ::core::ffi::c_int
                                        {
                                            ptr = ptr.offset(1);
                                            if *ptr as ::core::ffi::c_int == 0xb6 as ::core::ffi::c_int
                                                || *ptr as ::core::ffi::c_int == 0xa4 as ::core::ffi::c_int
                                            {
                                                *ptr = (*ptr as ::core::ffi::c_int
                                                    + ('A' as ::core::ffi::c_int - 'a' as ::core::ffi::c_int))
                                                    as ::core::ffi::c_uchar;
                                            }
                                        }
                                        ptr = ptr.offset(1);
                                    }
                                    t.baseline.x = (scavenge_r.corner.x as ::core::ffi::c_int
                                        + ((100 as ::core::ffi::c_int) << resolutionFactor))
                                        as COORD;
                                    t.baseline.y = (scavenge_r.corner.y as ::core::ffi::c_int
                                        + ((68 as ::core::ffi::c_int) << resolutionFactor))
                                        as COORD;
                                    t.align = TEXT_ALIGN::ALIGN_CENTER;
                                    t.pStr = &raw mut buf as *mut UNICODE;
                                    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
                                    uqm_lua_checked!(font_DrawText(&raw mut t), 0);
                                    t.baseline.y = (t.baseline.y as ::core::ffi::c_int
                                        + ((6 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
                                    t.pStr = GetStringAddress(
                                        SetAbsStringTableIndex(
                                            GameStrings,
                                            (C2Rust_Unnamed_12::ENCOUNTER_STRING_BASE.0
                                                as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as COUNT,
                                        ),
                                    );
                                    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
                                    uqm_lua_checked!(font_DrawText(&raw mut t), 0);
                                    ship_s.frame = (*FragPtr).icons;
                                    SetContextFont(MicroFont);
                                    str1 = GetStringAddress(
                                        SetAbsStringTableIndex(
                                            GameStrings,
                                            (C2Rust_Unnamed_12::ENCOUNTER_STRING_BASE.0
                                                as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as COUNT,
                                        ),
                                    );
                                    str2 = GetStringAddress(
                                        SetAbsStringTableIndex(
                                            GameStrings,
                                            (C2Rust_Unnamed_12::ENCOUNTER_STRING_BASE.0
                                                as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as COUNT,
                                        ),
                                    );
                                    uqm_lua_checked!(DrawFadeText(
                                        str1,
                                        str2,
                                        BOOLEAN::TRUE,
                                        &raw mut scavenge_r,
                                    ), 0);
                                }
                                r.corner.y = (scavenge_r.corner.y as ::core::ffi::c_int
                                    + ((9 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
                                r.extent.height = ((22 as ::core::ffi::c_int)
                                    << resolutionFactor) as COORD;
                                SetContextForeGroundColor(
                                    colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE),
                                );
                                r.extent.width = ((34 as ::core::ffi::c_int)
                                    << resolutionFactor) as COORD;
                                r.corner.x = (scavenge_r.corner.x as ::core::ffi::c_int
                                    + scavenge_r.extent.width as ::core::ffi::c_int
                                    - (((10 as ::core::ffi::c_int) << resolutionFactor)
                                        + r.extent.width as ::core::ffi::c_int)) as COORD;
                                BatchGraphics();
                                uqm_lua_checked!(ClearRectBack(&raw mut r), 0);
                                j = (ShipCost(EncounterRace as BYTE) as ::core::ffi::c_int
                                    >> 3 as ::core::ffi::c_int) as COUNT;
                                if EncounterRace as ::core::ffi::c_int
                                    == RACE_ID::SLYLANDRO_SHIP.0 as ::core::ffi::c_int
                                {
                                    j = 550 as COUNT;
                                }
                                if EncounterRace as ::core::ffi::c_int
                                    == RACE_ID::MELNORME_SHIP.0 as ::core::ffi::c_int
                                {
                                    j = (j as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                                        as COUNT;
                                }
                                RecycleAmount = (RecycleAmount as ::core::ffi::c_int
                                    + j as ::core::ffi::c_int) as COUNT;
                                sprintf(
                                    &raw mut buf as *mut ::core::ffi::c_char,
                                    b"%u\0".as_ptr() as *const ::core::ffi::c_char,
                                    RecycleAmount as ::core::ffi::c_int,
                                );
                                t.baseline.x = (r.corner.x as ::core::ffi::c_int
                                    + r.extent.width as ::core::ffi::c_int
                                    - ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
                                t.baseline.y = (r.corner.y as ::core::ffi::c_int
                                    + ((14 as ::core::ffi::c_int) << resolutionFactor))
                                    as COORD;
                                t.align = TEXT_ALIGN::ALIGN_RIGHT;
                                t.pStr = &raw mut buf as *mut UNICODE;
                                t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
                                SetContextForeGroundColor(
                                    colorFromRgb15(0 as BYTE, 0 as BYTE, 0x18 as BYTE),
                                );
                                uqm_lua_checked!(font_DrawText(&raw mut t), 0);
                                UnbatchGraphics();
                                uqm_lua_checked!(DeltaSISGauges(
                                    0 as SIZE,
                                    0 as SDWORD,
                                    j as ::core::ffi::c_int,
                                ), 0);
                                let c2rust_fresh0 = VictoryState;
                                VictoryState += 1;
                                if (c2rust_fresh0 as ::core::ffi::c_int
                                    - 1 as ::core::ffi::c_int) % 5 as ::core::ffi::c_int != 0
                                {
                                    ship_s.origin.x = (ship_s.origin.x as ::core::ffi::c_int
                                        + ((17 as ::core::ffi::c_int) << resolutionFactor))
                                        as COORD;
                                } else {
                                    let mut textRect: RECT = uqm_lua_checked!(font_GetTextRect(&raw mut t), 0);
                                    SetContextForeGroundColor(
                                        colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE),
                                    );
                                    r.corner.x = (scavenge_r.corner.x as ::core::ffi::c_int
                                        + ((10 as ::core::ffi::c_int) << resolutionFactor))
                                        as COORD;
                                    r.extent.width = (((132 as ::core::ffi::c_int)
                                        << resolutionFactor)
                                        - (textRect.extent.width as ::core::ffi::c_int
                                            + ((1 as ::core::ffi::c_int) << resolutionFactor)))
                                        as COORD;
                                    if Sleepy.0 != 0 {
                                        uqm_lua_checked!(SleepThread(
                                            (840 as ::core::ffi::c_int / 15 as ::core::ffi::c_int)
                                                as TimePeriod,
                                        ), 0);
                                    }
                                    uqm_lua_checked!(ClearRectBack(&raw mut r), 0);
                                    ship_s.origin.x = (r.corner.x as ::core::ffi::c_int
                                        + ((2 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
                                    ship_s.origin.y = (scavenge_r.corner.y as ::core::ffi::c_int
                                        + ((12 as ::core::ffi::c_int) << resolutionFactor))
                                        as COORD;
                                }
                                if Sleepy.0 != 0 {
                                    let mut Time: TimeCount = GetTimeCounter();
                                    j = 0 as COUNT;
                                    while (j as usize)
                                        < ::core::mem::size_of::<[Color; 10]>()
                                            .wrapping_div(::core::mem::size_of::<Color>())
                                    {
                                        Sleepy = BOOLEAN(
                                            (BOOLEAN(
                                                    (uqm_lua_checked!(AnyButtonPress(BOOLEAN::TRUE), 0).0 == 0) as ::core::ffi::c_int
                                                        as ::core::ffi::c_uint,
                                                )
                                                .0 != 0
                                                && GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
                                                    & C2Rust_Unnamed_1::CHECK_ABORT.0 as ::core::ffi::c_int
                                                    == 0) as ::core::ffi::c_int as ::core::ffi::c_uint,
                                        );
                                        if if resolutionFactor != 2 as ::core::ffi::c_uint {
                                            BOOLEAN::FALSE.0 as ::core::ffi::c_int
                                        } else {
                                            BOOLEAN::TRUE.0 as ::core::ffi::c_int
                                        } != 0
                                        {
                                            let mut p: POINT = POINT { x: 0, y: 0 };
                                            let mut ship_r: RECT = RECT {
                                                corner: POINT { x: 0, y: 0 },
                                                extent: EXTENT { width: 0, height: 0 },
                                            };
                                            GetFrameRect(ship_s.frame, &raw mut ship_r);
                                            p = GetFrameHot(ship_s.frame) as POINT;
                                            ship_r.corner = ship_s.origin;
                                            ship_r.corner.x = (ship_r.corner.x as ::core::ffi::c_int
                                                - p.x as ::core::ffi::c_int) as COORD;
                                            ship_r.corner.y = (ship_r.corner.y as ::core::ffi::c_int
                                                - p.y as ::core::ffi::c_int) as COORD;
                                            uqm_lua_checked!(RepairPickFrame(&raw mut ship_r, 1 as COUNT), 0);
                                        }
                                        if Sleepy.0 == 0 {
                                            break;
                                        }
                                        SetContextForeGroundColor(fade_ship_cycle[j as usize]);
                                        uqm_lua_checked!(DrawFilledStamp(&raw mut ship_s), 0);
                                        uqm_lua_checked!(SleepThreadUntil(
                                            Time
                                                .wrapping_add(
                                                    (840 as ::core::ffi::c_int / 15 as ::core::ffi::c_int)
                                                        as TimeCount,
                                                ),
                                        ), 0);
                                        Time = GetTimeCounter();
                                        j = j.wrapping_add(1);
                                    }
                                }
                                uqm_lua_checked!(DrawStamp(&raw mut ship_s), 0);
                            }
                        }
                        UnlockLink(pQueue, hStarShip);
                        RemoveQueue(pQueue, hStarShip);
                        FreeLink(pQueue, hStarShip);
                    } else {
                        UnlockLink(pQueue, hStarShip);
                    }
                    hStarShip = hNextShip;
                }
                i += 1;
            }
            if prevMsgMode.0 != 0 {
                SetStatusMessageMode(prevMsgMode);
            }
            if VictoryState != 0 {
                uqm_lua_checked!(WaitForAnyButton(
                    BOOLEAN::TRUE,
                    (840 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as TimePeriod,
                    BOOLEAN::FALSE,
                ), 0);
                if CurrentInputState
                    .key[(*(&raw mut PlayerControls as *mut CONTROL_TEMPLATE)
                    .offset(0isize))
                    .0
                    as usize][C2Rust_Unnamed::KEY_ESCAPE.0 as ::core::ffi::c_int
                    as usize] == 0
                {
                    uqm_lua_checked!(DrawFadeText(str1, str2, BOOLEAN::FALSE, &raw mut scavenge_r), 0);
                    if CurrentInputState
                        .key[(*(&raw mut PlayerControls as *mut CONTROL_TEMPLATE)
                        .offset(0isize))
                        .0
                        as usize][C2Rust_Unnamed::KEY_ESCAPE.0 as ::core::ffi::c_int
                        as usize] == 0
                    {
                        SetContextForeGroundColor(
                            colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE),
                        );
                        r.corner.x = (scavenge_r.corner.x as ::core::ffi::c_int
                            + ((10 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
                        r.extent.width = ((132 as ::core::ffi::c_int)
                            << resolutionFactor) as COORD;
                        uqm_lua_checked!(ClearRectBack(&raw mut r), 0);
                        sprintf(
                            &raw mut buf as *mut ::core::ffi::c_char,
                            b"%u %s\0".as_ptr() as *const ::core::ffi::c_char,
                            RecycleAmount as ::core::ffi::c_int,
                            GetStringAddress(
                                SetAbsStringTableIndex(
                                    GameStrings,
                                    (C2Rust_Unnamed_12::STATUS_STRING_BASE.0
                                        as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as COUNT,
                                ),
                            ),
                        );
                        t.baseline.x = (r.corner.x as ::core::ffi::c_int
                            + (r.extent.width as ::core::ffi::c_int
                                >> 1 as ::core::ffi::c_int)) as COORD;
                        t.baseline.y = (r.corner.y as ::core::ffi::c_int
                            + ((14 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
                        t.align = TEXT_ALIGN::ALIGN_CENTER;
                        t.pStr = &raw mut buf as *mut UNICODE;
                        t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
                        SetContextForeGroundColor(
                            colorFromRgb15(0 as BYTE, 0 as BYTE, 0x18 as BYTE),
                        );
                        uqm_lua_checked!(font_DrawText(&raw mut t), 0);
                        str1 = GetStringAddress(
                            SetAbsStringTableIndex(
                                GameStrings,
                                (C2Rust_Unnamed_12::ENCOUNTER_STRING_BASE.0
                                    as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as COUNT,
                            ),
                        );
                        str2 = GetStringAddress(
                            SetAbsStringTableIndex(
                                GameStrings,
                                (C2Rust_Unnamed_12::ENCOUNTER_STRING_BASE.0
                                    as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as COUNT,
                            ),
                        );
                        uqm_lua_checked!(DrawFadeText(str1, str2, BOOLEAN::TRUE, &raw mut scavenge_r), 0);
                        uqm_lua_checked!(WaitForAnyButton(
                            BOOLEAN::TRUE,
                            (840 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                                as TimePeriod,
                            BOOLEAN::FALSE,
                        ), 0);
                        if CurrentInputState
                            .key[(*(&raw mut PlayerControls as *mut CONTROL_TEMPLATE)
                            .offset(0isize))
                            .0
                            as usize][C2Rust_Unnamed::KEY_ESCAPE.0 as ::core::ffi::c_int
                            as usize] == 0
                        {
                            uqm_lua_checked!(DrawFadeText(
                                str1,
                                str2,
                                BOOLEAN::FALSE,
                                &raw mut scavenge_r,
                            ), 0);
                        }
                    }
                }
                uqm_lua_checked!(DrawStatusMessage(::core::ptr::null::<UNICODE>()), 0);
                DestroyPickFrame();
            }
            if ships_killed as ::core::ffi::c_int != 0
                && EncounterRace as ::core::ffi::c_int
                    == RACE_ID::THRADDASH_SHIP.0 as ::core::ffi::c_int
                && uqm_lua_checked!(getGameStateUint(
                    b"THRADD_MANNER\0".as_ptr() as *const ::core::ffi::c_char,
                ), 0) == 0
            {
                ships_killed = (ships_killed as DWORD)
                    .wrapping_add(
                        uqm_lua_checked!(getGameStateUint(
                            b"THRADDASH_BODY_COUNT\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        ), 0),
                    ) as COUNT;
                if ships_killed as ::core::ffi::c_int
                    > if if GlobData.SIS_state.Difficulty as ::core::ffi::c_int != 0 {
                        GlobData.SIS_state.Difficulty as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    } == 0 as ::core::ffi::c_int
                    {
                        25 as ::core::ffi::c_int
                    } else if if GlobData.SIS_state.Difficulty as ::core::ffi::c_int != 0
                    {
                        GlobData.SIS_state.Difficulty as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    } == 1 as ::core::ffi::c_int
                    {
                        15 as ::core::ffi::c_int
                    } else {
                        30 as ::core::ffi::c_int
                    }
                {
                    ships_killed = (if if GlobData.SIS_state.Difficulty
                        as ::core::ffi::c_int != 0
                    {
                        GlobData.SIS_state.Difficulty as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    } == 0 as ::core::ffi::c_int
                    {
                        25 as ::core::ffi::c_int
                    } else if if GlobData.SIS_state.Difficulty as ::core::ffi::c_int != 0
                    {
                        GlobData.SIS_state.Difficulty as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    } == 1 as ::core::ffi::c_int
                    {
                        15 as ::core::ffi::c_int
                    } else {
                        30 as ::core::ffi::c_int
                    }) as COUNT;
                }
                uqm_lua_checked!(setGameStateUint(
                    b"THRADDASH_BODY_COUNT\0".as_ptr() as *const ::core::ffi::c_char,
                    ships_killed as DWORD,
                ), 0);
            }
            if optSlaughterMode.0 != 0
                && CheckSphereTracking(RACE_ID(EncounterRace as ::core::ffi::c_uint)).0
                    != 0 && ships_killed as ::core::ffi::c_int != 0
            {
                let mut hEncounter: HFLEETINFO = ::core::ptr::null_mut::<
                    ::core::ffi::c_void,
                >();
                let mut EncounterPtr: *mut FLEET_INFO = ::core::ptr::null_mut::<
                    FLEET_INFO,
                >();
                let mut isBanned: BOOLEAN = BOOLEAN::FALSE;
                let mut j_0: BYTE = 0;
                let bannedShip: [BYTE; 7] = [
                    RACE_ID::PKUNK_SHIP.0 as ::core::ffi::c_int as BYTE,
                    RACE_ID::SHOFIXTI_SHIP.0 as ::core::ffi::c_int as BYTE,
                    RACE_ID::THRADDASH_SHIP.0 as ::core::ffi::c_int as BYTE,
                    RACE_ID::YEHAT_SHIP.0 as ::core::ffi::c_int as BYTE,
                    RACE_ID::MELNORME_SHIP.0 as ::core::ffi::c_int as BYTE,
                    RACE_ID::ILWRATH_SHIP.0 as ::core::ffi::c_int as BYTE,
                    RACE_ID::SLYLANDRO_SHIP.0 as ::core::ffi::c_int as BYTE,
                ];
                j_0 = 0 as BYTE;
                while (j_0 as usize)
                    < ::core::mem::size_of::<[BYTE; 7]>()
                        .wrapping_div(::core::mem::size_of::<BYTE>())
                {
                    if bannedShip[j_0 as usize] as ::core::ffi::c_int
                        == EncounterRace as ::core::ffi::c_int
                    {
                        isBanned = BOOLEAN::TRUE;
                        break;
                    } else {
                        j_0 = j_0.wrapping_add(1);
                    }
                }
                if isBanned.0 == 0 {
                    hEncounter = GetStarShipFromIndex(
                        &raw mut GlobData.Game_state.avail_race_q,
                        EncounterRace as COUNT,
                    ) as HFLEETINFO;
                    EncounterPtr = LockFleetInfo(
                        &raw mut GlobData.Game_state.avail_race_q,
                        hEncounter,
                    );
                    if (*EncounterPtr).actual_strength as ::core::ffi::c_int
                        > 0 as ::core::ffi::c_int
                    {
                        let mut actualStrength: SIZE = (*EncounterPtr).actual_strength
                            as SIZE;
                        actualStrength = (actualStrength as ::core::ffi::c_int
                            - ships_killed as ::core::ffi::c_int) as SIZE;
                        if actualStrength as ::core::ffi::c_int
                            <= 0 as ::core::ffi::c_int
                        {
                            (*EncounterPtr).actual_strength = 0 as COUNT;
                            (*EncounterPtr).allied_state = C2Rust_Unnamed_9::DEAD_GUY.0
                                as ::core::ffi::c_int as UWORD;
                        } else {
                            (*EncounterPtr).actual_strength = actualStrength as COUNT;
                        }
                    }
                    UnlockLink(&raw mut GlobData.Game_state.avail_race_q, hEncounter);
                }
            }
        }
    }
    return ships_killed;
}
#[no_mangle]
pub unsafe extern "C" fn EncounterBattle() {
    let mut OldActivity: ACTIVITY = 0;
    extern "C" {
        static mut nth_frame: UWORD;
    }
    let mut savedPlayerInput: *mut InputContext = ::core::ptr::null_mut::<
        InputContext,
    >();
    uqm_lua_checked!(setGameStateUint(
        b"BATTLE_SEGUE\0".as_ptr() as *const ::core::ffi::c_char,
        1 as DWORD,
    ), ());
    OldActivity = GlobData.Game_state.CurrentActivity;
    if OldActivity as BYTE as ::core::ffi::c_int
        == C2Rust_Unnamed_1::IN_LAST_BATTLE.0 as ::core::ffi::c_int
    {
        GlobData.Game_state.CurrentActivity = (((0 as ::core::ffi::c_int as BYTE
            as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as UWORD
            as ::core::ffi::c_int
            | C2Rust_Unnamed_1::IN_LAST_BATTLE.0 as ::core::ffi::c_int as BYTE
                as ::core::ffi::c_int) as ACTIVITY;
    } else {
        GlobData.Game_state.CurrentActivity = (((0 as ::core::ffi::c_int as BYTE
            as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as UWORD
            as ::core::ffi::c_int
            | C2Rust_Unnamed_1::IN_ENCOUNTER.0 as ::core::ffi::c_int as BYTE
                as ::core::ffi::c_int) as ACTIVITY;
    }
    uqm_lua_checked!(WaitForSoundEnd(!(0 as ::core::ffi::c_int) as COUNT), ());
    if GlobData.Game_state.glob_flags as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int != 0
    {
        let mut cur_speed: BYTE = 0;
        cur_speed = ((GlobData.Game_state.glob_flags as ::core::ffi::c_int
            & (((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                - 1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as BYTE
            as ::core::ffi::c_int >> 6 as ::core::ffi::c_int) as BYTE;
        if cur_speed as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            cur_speed = 0 as BYTE;
        } else if cur_speed as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            cur_speed = cur_speed.wrapping_add(1);
        } else {
            cur_speed = !(0 as ::core::ffi::c_int) as BYTE;
        }
        nth_frame = (((cur_speed as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
            as UWORD as ::core::ffi::c_int
            | 1 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as UWORD;
        *(&raw mut PlayerControl as *mut BYTE).offset(0isize) = (((1
            as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as BYTE
            as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as BYTE
                as ::core::ffi::c_int) as BYTE;
        savedPlayerInput = *(&raw mut PlayerInput as *mut *mut InputContext)
            .offset(0isize);
        *(&raw mut PlayerInput as *mut *mut InputContext).offset(0isize) = ::core::ptr::null_mut::<
            InputContext,
        >();
        if !SetPlayerInput(0 as COUNT) {
            log_add(
                log_Level::log_Fatal,
                b"Could not set cyborg player input.\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            explode();
        }
    }
    if if if GlobData.SIS_state.Difficulty as ::core::ffi::c_int != 0 {
        GlobData.SIS_state.Difficulty as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    } == 1 as ::core::ffi::c_int
    {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    } != 0
    {
        *(&raw mut PlayerControl as *mut BYTE).offset(1isize) = (((1
            as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as BYTE
            as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as BYTE
                as ::core::ffi::c_int) as BYTE;
    }
    GameSounds = CaptureStringTable(
        uqm_lua_checked!(LoadSoundInstance(b"sounds.battle\0".as_ptr() as RESOURCE), ()),
    ) as SOUND;
    uqm_lua_checked!(Battle(None), ());
    uqm_lua_checked!(DestroySound(ReleaseStringTable(GameSounds)), ());
    GameSounds = ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>();
    if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
        & C2Rust_Unnamed_1::CHECK_ABORT.0 as ::core::ffi::c_int != 0
    {
        GlobData.SIS_state.CrewEnlisted = !(0 as ::core::ffi::c_int) as COUNT;
    }
    if GlobData.Game_state.glob_flags as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int != 0
    {
        nth_frame = (((0 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int)
            << 8 as ::core::ffi::c_int) as UWORD as ::core::ffi::c_int
            | 0 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as UWORD;
        *(&raw mut PlayerControl as *mut BYTE).offset(0isize) = (((1
            as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as BYTE
            as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as BYTE
                as ::core::ffi::c_int) as BYTE;
        uqm_lua_checked!(ClearPlayerInput(0 as COUNT), ());
        *(&raw mut PlayerInput as *mut *mut InputContext).offset(0isize) = savedPlayerInput;
    }
    GlobData.Game_state.CurrentActivity = OldActivity;
}
#[inline]
unsafe extern "C" fn explode() -> ! {
    abort();
}
