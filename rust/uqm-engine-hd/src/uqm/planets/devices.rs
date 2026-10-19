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

// Rust translation of src/uqm/planets/devices.c.
extern "C" {
    pub type string_table_entry;
    pub type context_desc;
    pub type frame_desc;
    pub type font_desc;
    pub type tfb_soundsample;
    pub type FlashContext;
    fn SetContext(Context: CONTEXT) -> CONTEXT;
    fn SetContextForeGroundColor(Color: Color) -> Color;
    fn DrawStamp(pStamp: *mut STAMP);
    fn DrawFilledRectangle(pRect: *mut RECT);
    fn font_DrawText(pText: *mut TEXT);
    fn BatchGraphics();
    fn UnbatchGraphics();
    fn SetContextFont(Font: FONT) -> FONT;
    fn SetAbsFrameIndex(Frame: FRAME, FrameIndex: COUNT) -> FRAME;
    fn FadeScreen(fadeType: ScreenFadeType, TimeInterval: SIZE) -> DWORD;
    fn __assert_fail(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    ) -> !;
    fn SetAbsStringTableIndex(String: STRING, StringTableIndex: COUNT) -> STRING;
    fn GetStringAddress(String: STRING) -> STRINGPTR;
    fn utf8StringPos(
        pStr: *const ::core::ffi::c_char,
        ch: UniChar,
    ) -> ::core::ffi::c_int;
    fn skipUTF8Chars(
        ptr: *const ::core::ffi::c_char,
        num: size_t,
    ) -> *mut ::core::ffi::c_char;
    fn memset(
        _: *mut ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    static mut PulsedInputState: CONTROLLER_INPUT_STATE;
    fn DoInput(pInputState: *mut ::core::ffi::c_void, resetInput: BOOLEAN);
    static mut optWhichMenu: ::core::ffi::c_int;
    static mut optWhichFonts: ::core::ffi::c_int;
    static mut resolutionFactor: ::core::ffi::c_uint;
    static mut optInfiniteFuel: OPT_ENABLABLE;
    static mut optCustomBorder: OPT_ENABLABLE;
    fn ReinitQueue(pq: *mut QUEUE);
    fn CountLinks(pq: *mut QUEUE) -> COUNT;
    fn SleepThreadUntil(wakeTime: TimeCount);
    fn ClearSISRect(ClearFlags: BYTE);
    fn setGameStateUint(name: *const ::core::ffi::c_char, val: DWORD);
    fn getGameStateUint(name: *const ::core::ffi::c_char) -> DWORD;
    static mut GlobData: GLOBDATA;
    fn inHQSpace() -> BOOLEAN;
    fn inHyperSpace() -> BOOLEAN;
    fn isPC(optWhich: ::core::ffi::c_int) -> OPT_CONSOLETYPE;
    static mut pSolarSysState: *mut SOLARSYS_STATE;
    fn playerInSolarSystem() -> bool;
    fn playerInPlanetOrbit() -> bool;
    fn matchWorld(
        solarSys: *const SOLARSYS_STATE,
        world: *const PLANET_DESC,
        planetI: BYTE,
        moonI: BYTE,
    ) -> bool;
    fn SaveSolarSysLocation();
    static mut CurStarDescPtr: *mut STAR_DESC;
    fn CloneShipFragment(
        shipIndex: RACE_ID,
        pDstQueue: *mut QUEUE,
        crew_level: COUNT,
    ) -> HSHIPFRAG;
    fn StartSphereTracking(race: RACE_ID) -> COUNT;
    static mut EncounterGroup: BYTE;
    static mut NextActivity: ACTIVITY;
    static mut GameStrings: STRING;
    fn DrawMenuStateStrings(beg_index: BYTE, NewState: SWORD);
    fn DrawBorder(Visible: BYTE);
    static mut MiscDataFrame: FRAME;
    static mut StatusContext: CONTEXT;
    static mut StarConFont: FONT;
    static mut TinyFont: FONT;
    static mut TinyFontBold: FONT;
    static mut DeathBySuicide: BOOLEAN;
    fn PutGroupInfo(offset: DWORD, which_group: BYTE) -> DWORD;
    fn SetMenuSounds(sound_0: MENU_SOUND_FLAGS, sound_1: MENU_SOUND_FLAGS);
    fn PlayMenuSound(S: MENU_SOUND_EFFECT);
    fn DrawStarConBox(
        pRect: *mut RECT,
        BorderWidth: SIZE,
        TopLeftColor: Color,
        BottomRightColor: Color,
        FillInterior: BOOLEAN,
        InteriorColor: Color,
        CreateCorners: BOOLEAN,
        CornerColor: Color,
    );
    fn SaveSisHyperState();
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
pub type UniChar = uint32;
pub type STRING_TABLE_ENTRY_DESC = string_table_entry;
pub type STRING = *mut STRING_TABLE_ENTRY_DESC;
pub type STRINGPTR = *mut ::core::ffi::c_char;
pub type CONTEXT_DESC = context_desc;
pub type FRAME_DESC = frame_desc;
pub type FONT_DESC = font_desc;
pub type CONTEXT = *mut CONTEXT_DESC;
pub type FRAME = *mut FRAME_DESC;
pub type FONT = *mut FONT_DESC;
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
pub type TimeCount = DWORD;
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct OPT_CONSOLETYPE(pub ::core::ffi::c_uint);
impl OPT_CONSOLETYPE {
    pub const OPTVAL_PC: Self = Self(0);
    pub const OPTVAL_3DO: Self = Self(1);
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
pub type MUSIC_REF = *mut *mut TFB_SoundSample;
pub type TFB_SoundSample = tfb_soundsample;
pub type HSHIPFRAG = HLINK;
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
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_0 {
    pub const SIS_MAIN_FRAME: Self = Self(0);
    pub const SIS_STAT_FRAME: Self = Self(1);
    pub const DEVICE_CARGO_FRAME: Self = Self(2);
    pub const SIS_STAT_REPAIR_FRAME: Self = Self(3);
    pub const SIS_STAT_SUMM_FRAME: Self = Self(4);
    pub const SIS_MSG_FRAME: Self = Self(5);
    pub const SIS_TITLE_FRAME: Self = Self(6);
    pub const CAPTAIN_FRAME: Self = Self(7);
    pub const CAP_NAME_FRAME: Self = Self(8);
    pub const STAT_MSG_FRAME: Self = Self(9);
    pub const SIS_RADAR_FRAME: Self = Self(10);
    pub const TEXT_LABEL_FRAME: Self = Self(11);
    pub const SIS_REPAIR_FRAME: Self = Self(12);
    pub const LANDER_3DO_FRAME: Self = Self(13);
    pub const LANDER_DOS_FRAME: Self = Self(14);
    pub const SIS_COMM_FRAME: Self = Self(15);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_1(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_1 {
    pub const PORTAL_SPAWNER_DEVICE: Self = Self(0);
    pub const TALKING_PET_DEVICE: Self = Self(1);
    pub const UTWIG_BOMB_DEVICE: Self = Self(2);
    pub const SUN_EFFICIENCY_DEVICE: Self = Self(3);
    pub const ROSY_SPHERE_DEVICE: Self = Self(4);
    pub const AQUA_HELIX_DEVICE: Self = Self(5);
    pub const CLEAR_SPINDLE_DEVICE: Self = Self(6);
    pub const ULTRON_0_DEVICE: Self = Self(7);
    pub const ULTRON_1_DEVICE: Self = Self(8);
    pub const ULTRON_2_DEVICE: Self = Self(9);
    pub const ULTRON_3_DEVICE: Self = Self(10);
    pub const MAIDENS_DEVICE: Self = Self(11);
    pub const UMGAH_HYPERWAVE_DEVICE: Self = Self(12);
    pub const BURVIX_HYPERWAVE_DEVICE: Self = Self(13);
    pub const DATA_PLATE_1_DEVICE: Self = Self(14);
    pub const DATA_PLATE_2_DEVICE: Self = Self(15);
    pub const DATA_PLATE_3_DEVICE: Self = Self(16);
    pub const TAALO_PROTECTOR_DEVICE: Self = Self(17);
    pub const EGG_CASING0_DEVICE: Self = Self(18);
    pub const EGG_CASING1_DEVICE: Self = Self(19);
    pub const EGG_CASING2_DEVICE: Self = Self(20);
    pub const SYREEN_SHUTTLE_DEVICE: Self = Self(21);
    pub const VUX_BEAST_DEVICE: Self = Self(22);
    pub const DESTRUCT_CODE_DEVICE: Self = Self(23);
    pub const URQUAN_WARP_DEVICE: Self = Self(24);
    pub const WIMBLIS_TRIDENT_DEVICE: Self = Self(25);
    pub const GLOWING_ROD_DEVICE: Self = Self(26);
    pub const LUNAR_BASE_DEVICE: Self = Self(27);
    pub const NUM_DEVICES: Self = Self(28);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_2(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_2 {
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
pub struct C2Rust_Unnamed_3(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_3 {
    pub const ARILOU_DEFINED: Self = Self(0);
    pub const SOL_DEFINED: Self = Self(1);
    pub const SHOFIXTI_DEFINED: Self = Self(2);
    pub const MAIDENS_DEFINED: Self = Self(3);
    pub const START_COLONY_DEFINED: Self = Self(4);
    pub const SPATHI_DEFINED: Self = Self(5);
    pub const ZOQFOT_DEFINED: Self = Self(6);
    pub const MELNORME0_DEFINED: Self = Self(7);
    pub const MELNORME1_DEFINED: Self = Self(8);
    pub const MELNORME2_DEFINED: Self = Self(9);
    pub const MELNORME3_DEFINED: Self = Self(10);
    pub const MELNORME4_DEFINED: Self = Self(11);
    pub const MELNORME5_DEFINED: Self = Self(12);
    pub const MELNORME6_DEFINED: Self = Self(13);
    pub const MELNORME7_DEFINED: Self = Self(14);
    pub const MELNORME8_DEFINED: Self = Self(15);
    pub const TALKING_PET_DEFINED: Self = Self(16);
    pub const CHMMR_DEFINED: Self = Self(17);
    pub const SYREEN_DEFINED: Self = Self(18);
    pub const BURVIXESE_DEFINED: Self = Self(19);
    pub const SLYLANDRO_DEFINED: Self = Self(20);
    pub const DRUUGE_DEFINED: Self = Self(21);
    pub const BOMB_DEFINED: Self = Self(22);
    pub const AQUA_HELIX_DEFINED: Self = Self(23);
    pub const SUN_DEVICE_DEFINED: Self = Self(24);
    pub const TAALO_PROTECTOR_DEFINED: Self = Self(25);
    pub const SHIP_VAULT_DEFINED: Self = Self(26);
    pub const URQUAN_WRECK_DEFINED: Self = Self(27);
    pub const VUX_BEAST_DEFINED: Self = Self(28);
    pub const SAMATRA_DEFINED: Self = Self(29);
    pub const ZOQ_SCOUT_DEFINED: Self = Self(30);
    pub const MYCON_DEFINED: Self = Self(31);
    pub const EGG_CASE0_DEFINED: Self = Self(32);
    pub const EGG_CASE1_DEFINED: Self = Self(33);
    pub const EGG_CASE2_DEFINED: Self = Self(34);
    pub const PKUNK_DEFINED: Self = Self(35);
    pub const UTWIG_DEFINED: Self = Self(36);
    pub const SUPOX_DEFINED: Self = Self(37);
    pub const YEHAT_DEFINED: Self = Self(38);
    pub const VUX_DEFINED: Self = Self(39);
    pub const ORZ_DEFINED: Self = Self(40);
    pub const THRADD_DEFINED: Self = Self(41);
    pub const RAINBOW0_DEFINED: Self = Self(42);
    pub const RAINBOW1_DEFINED: Self = Self(43);
    pub const RAINBOW2_DEFINED: Self = Self(44);
    pub const RAINBOW3_DEFINED: Self = Self(45);
    pub const RAINBOW4_DEFINED: Self = Self(46);
    pub const RAINBOW5_DEFINED: Self = Self(47);
    pub const RAINBOW6_DEFINED: Self = Self(48);
    pub const RAINBOW7_DEFINED: Self = Self(49);
    pub const RAINBOW8_DEFINED: Self = Self(50);
    pub const RAINBOW9_DEFINED: Self = Self(51);
    pub const ILWRATH_DEFINED: Self = Self(52);
    pub const ANDROSYNTH_DEFINED: Self = Self(53);
    pub const MYCON_TRAP_DEFINED: Self = Self(54);
    pub const URQUAN0_DEFINED: Self = Self(55);
    pub const URQUAN1_DEFINED: Self = Self(56);
    pub const URQUAN2_DEFINED: Self = Self(57);
    pub const KOHRAH0_DEFINED: Self = Self(58);
    pub const KOHRAH1_DEFINED: Self = Self(59);
    pub const KOHRAH2_DEFINED: Self = Self(60);
    pub const DESTROYED_STARBASE_DEFINED: Self = Self(61);
    pub const MOTHER_ARK_DEFINED: Self = Self(62);
    pub const ZOQ_COLONY0_DEFINED: Self = Self(63);
    pub const ZOQ_COLONY1_DEFINED: Self = Self(64);
    pub const ZOQ_COLONY2_DEFINED: Self = Self(65);
    pub const ZOQ_COLONY3_DEFINED: Self = Self(66);
    pub const ALGOLITES_DEFINED: Self = Self(67);
    pub const SPATHI_MONUMENT_DEFINED: Self = Self(68);
    pub const EXCAVATION_SITE_DEFINED: Self = Self(69);
    pub const NUM_PLOTS: Self = Self(70);
    pub const WAR_ERA: Self = Self(70);
    pub const SUCCESS: Self = Self(70);
    pub const HOME: Self = Self(71);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_4(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_4 {
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
pub type MENU_STATE = menu_state;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_5(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_5 {
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
pub type MENU_SOUND_FLAGS = UWORD;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DEVICES_STATE {
    pub list: [BYTE; 28],
    pub count: COUNT,
    pub topIndex: COUNT,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct DeviceStatus(pub ::core::ffi::c_uint);
impl DeviceStatus {
    pub const DEVICE_FAILURE: Self = Self(0);
    pub const DEVICE_SUCCESS: Self = Self(1);
    pub const DEVICE_SUCCESS_NO_SOUND: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct MENU_SOUND_EFFECT(pub ::core::ffi::c_uint);
impl MENU_SOUND_EFFECT {
    pub const MENU_SOUND_MOVE: Self = Self(0);
    pub const MENU_SOUND_SUCCESS: Self = Self(1);
    pub const MENU_SOUND_FAILURE: Self = Self(2);
    pub const MENU_SOUND_INVOKED: Self = Self(3);
    pub const MENU_SOUND_BUBBLEWARP: Self = Self(4);
    pub const MENU_SOUND_TEXT: Self = Self(5);
}
pub type HIPGROUP = HLINK;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IP_GROUP {
    pub pred: HIPGROUP,
    pub succ: HIPGROUP,
    pub group_counter: UWORD,
    pub race_id: BYTE,
    pub sys_loc: BYTE,
    pub task: BYTE,
    pub in_system: BYTE,
    pub dest_loc: BYTE,
    pub orbit_pos: BYTE,
    pub group_id: BYTE,
    pub loc: POINT,
    pub melee_icon: FRAME,
    pub flags: IP_FLAGS,
}
pub type IP_FLAGS = BYTE;
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
unsafe extern "C" fn MAKE_POINT(mut x: COORD, mut y: COORD) -> POINT {
    let mut pt: POINT = point { x: x, y: y };
    return pt;
}
#[inline]
unsafe extern "C" fn logxToUniverse(mut lx: SDWORD) -> SDWORD {
    return (lx * (9999 as SDWORD + 1 as SDWORD >> 4 as ::core::ffi::c_int)
        + ((9999 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
            >> 4 as ::core::ffi::c_int)
            * ((16 as ::core::ffi::c_int) << resolutionFactor)
            >> 1 as ::core::ffi::c_int))
        / ((9999 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
            >> 4 as ::core::ffi::c_int)
            * ((16 as ::core::ffi::c_int) << resolutionFactor));
}
#[inline]
unsafe extern "C" fn logyToUniverse(mut ly: SDWORD) -> SDWORD {
    return 9999 as SDWORD
        - (ly * (9999 as SDWORD + 1 as SDWORD >> 4 as ::core::ffi::c_int)
            + ((9999 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                >> 4 as ::core::ffi::c_int)
                * ((16 as ::core::ffi::c_int) << resolutionFactor)
                >> 1 as ::core::ffi::c_int))
            / ((9999 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                >> 4 as ::core::ffi::c_int)
                * ((16 as ::core::ffi::c_int) << resolutionFactor));
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
                    b"src/uqm/planets/../displist.h\0"
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
                    b"src/uqm/planets/../displist.h\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    88 as ::core::ffi::c_int,
                    b"UnlockLink\0".as_ptr() as *const ::core::ffi::c_char,
                );
                0 as ::core::ffi::c_int != 0
            }) as ::core::ffi::c_int;
    }
}
#[inline]
unsafe extern "C" fn SaveLastLoc(mut pt: POINT) {
    uqm_lua_checked!(setGameStateUint(
        b"LAST_LOCATION_X\0".as_ptr() as *const ::core::ffi::c_char,
        pt.x as DWORD,
    ), ());
    uqm_lua_checked!(setGameStateUint(
        b"LAST_LOCATION_Y\0".as_ptr() as *const ::core::ffi::c_char,
        pt.y as DWORD,
    ), ());
}
#[inline]
unsafe extern "C" fn LockIpGroup(
    mut pq: *const QUEUE,
    mut h: HIPGROUP,
) -> *mut IP_GROUP {
    ((*pq).object_size as usize == ::core::mem::size_of::<IP_GROUP>()
        || {
            __assert_fail(
                b"GetLinkSize (pq) == sizeof (IP_GROUP)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/uqm/planets/../grpinfo.h\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                89 as ::core::ffi::c_int,
                b"LockIpGroup\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    return LockLink(pq, h) as *mut IP_GROUP;
}
unsafe extern "C" fn DrawDevice(mut device: COUNT, mut pos: COUNT, mut selected: bool) {
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
    t.align = TEXT_ALIGN::ALIGN_CENTER;
    t.baseline.x = ((40 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    r.extent.width = (((64 as ::core::ffi::c_int) << resolutionFactor)
        - ((5 as ::core::ffi::c_int) << resolutionFactor)
        + ((2 as ::core::ffi::c_int) << resolutionFactor)
        - (((4 as ::core::ffi::c_int) << resolutionFactor)
            + ((16 as ::core::ffi::c_int) << resolutionFactor))) as COORD;
    r.extent.height = (((7 as ::core::ffi::c_int) << resolutionFactor)
        * 2 as ::core::ffi::c_int) as COORD;
    r.corner.x = (((4 as ::core::ffi::c_int) << resolutionFactor)
        + ((16 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    r.corner.y = (((33 as ::core::ffi::c_int) << resolutionFactor)
        + pos as ::core::ffi::c_int
            * (((16 as ::core::ffi::c_int) << resolutionFactor)
                + ((2 as ::core::ffi::c_int) << resolutionFactor))
        + ((2 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    SetContextForeGroundColor(
        if selected as ::core::ffi::c_int != 0 {
            colorFromRgb15(0xa as BYTE, 0xa as BYTE, 0x1f as BYTE)
        } else {
            colorFromRgb15(0 as BYTE, 0 as BYTE, 0x14 as BYTE)
        },
    );
    uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
    if isPC(optWhichFonts).0 != 0 {
        SetContextFont(TinyFont);
    } else {
        SetContextFont(TinyFontBold);
    }
    SetContextForeGroundColor(
        if selected as ::core::ffi::c_int != 0 {
            colorFromRgb15(0xa as BYTE, 0x1f as BYTE, 0x1f as BYTE)
        } else {
            colorFromRgb15(0 as BYTE, 0x14 as BYTE, 0x14 as BYTE)
        },
    );
    t.baseline.y = (r.corner.y as ::core::ffi::c_int
        + ((6 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    t.pStr = GetStringAddress(
        SetAbsStringTableIndex(
            GameStrings,
            (device as ::core::ffi::c_int
                + C2Rust_Unnamed_4::DEVICE_STRING_BASE.0 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int) as COUNT,
        ),
    );
    t.CharCount = utf8StringPos(t.pStr, ' ' as UniChar) as COUNT;
    uqm_lua_checked!(font_DrawText(&raw mut t), ());
    t.baseline.y = (t.baseline.y as ::core::ffi::c_int
        + ((7 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    t.pStr = skipUTF8Chars(
        t.pStr,
        (t.CharCount as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as size_t,
    );
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    uqm_lua_checked!(font_DrawText(&raw mut t), ());
}
unsafe extern "C" fn DrawDevicesDisplay(mut devState: *mut DEVICES_STATE) {
    let mut t: TEXT = TEXT {
        baseline: POINT { x: 0, y: 0 },
        pStr: ::core::ptr::null::<UNICODE>(),
        align: TEXT_ALIGN::ALIGN_LEFT,
        CharCount: 0,
    };
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut s: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    let mut cy: COORD = 0;
    let mut i: COUNT = 0;
    r.corner.x = ((2 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    r.corner.y = ((20 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    r.extent.width = (((64 as ::core::ffi::c_int) << resolutionFactor)
        - ((5 as ::core::ffi::c_int) << resolutionFactor)
        + ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    r.extent.height = (((129 as ::core::ffi::c_int) << resolutionFactor)
        - r.corner.y as ::core::ffi::c_int) as COORD;
    if optCustomBorder.0 == 0
        && if resolutionFactor != 2 as ::core::ffi::c_uint {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } == 0
    {
        uqm_lua_checked!(DrawStarConBox(
            &raw mut r,
            ((1 as ::core::ffi::c_int) << resolutionFactor) as SIZE,
            colorFromRgb15(0x10 as BYTE, 0x10 as BYTE, 0x10 as BYTE),
            colorFromRgb15(0x8 as BYTE, 0x8 as BYTE, 0x8 as BYTE),
            BOOLEAN::TRUE,
            colorFromRgb15(0 as BYTE, 0 as BYTE, 0x14 as BYTE),
            BOOLEAN::FALSE,
            buildColorRgba(0x50 as BYTE, 0x50 as BYTE, 0x50 as BYTE, 0 as BYTE),
        ), ());
    } else {
        uqm_lua_checked!(DrawBorder(C2Rust_Unnamed_0::DEVICE_CARGO_FRAME.0 as ::core::ffi::c_int as BYTE), ());
    }
    SetContextFont(StarConFont);
    t.baseline.x = (((64 as ::core::ffi::c_int) << resolutionFactor
        >> 1 as ::core::ffi::c_int) - ((1 as ::core::ffi::c_int) << resolutionFactor))
        as COORD;
    t.baseline.y = (r.corner.y as ::core::ffi::c_int
        + ((7 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    t.align = TEXT_ALIGN::ALIGN_CENTER;
    t.pStr = GetStringAddress(
        SetAbsStringTableIndex(
            GameStrings,
            C2Rust_Unnamed_4::DEVICE_STRING_BASE.0 as ::core::ffi::c_int as COUNT,
        ),
    );
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    SetContextForeGroundColor(colorFromRgb15(0xa as BYTE, 0x1f as BYTE, 0x1f as BYTE));
    uqm_lua_checked!(font_DrawText(&raw mut t), ());
    s.origin.x = ((4 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    cy = ((33 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    i = 0 as COUNT;
    while (i as ::core::ffi::c_int)
        < (((129 as ::core::ffi::c_int) << resolutionFactor)
            - ((33 as ::core::ffi::c_int) << resolutionFactor))
            / (((16 as ::core::ffi::c_int) << resolutionFactor)
                + ((2 as ::core::ffi::c_int) << resolutionFactor))
    {
        let mut devIndex: COUNT = ((*devState).topIndex as ::core::ffi::c_int
            + i as ::core::ffi::c_int) as COUNT;
        if devIndex as ::core::ffi::c_int >= (*devState).count as ::core::ffi::c_int {
            break;
        }
        s.origin.y = (cy as ::core::ffi::c_int
            + ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        s.frame = SetAbsFrameIndex(
            MiscDataFrame,
            (77 as ::core::ffi::c_int
                + (*devState).list[devIndex as usize] as ::core::ffi::c_int) as COUNT,
        );
        uqm_lua_checked!(DrawStamp(&raw mut s), ());
        uqm_lua_checked!(DrawDevice((*devState).list[devIndex as usize] as COUNT, i, false), ());
        i = i.wrapping_add(1);
        cy = (cy as ::core::ffi::c_int
            + (((16 as ::core::ffi::c_int) << resolutionFactor)
                + ((2 as ::core::ffi::c_int) << resolutionFactor))) as COORD;
    }
}
unsafe extern "C" fn DrawDevices(
    mut devState: *mut DEVICES_STATE,
    mut OldDevice: COUNT,
    mut NewDevice: COUNT,
) {
    BatchGraphics();
    SetContext(StatusContext);
    if OldDevice as ::core::ffi::c_int
        > C2Rust_Unnamed_1::NUM_DEVICES.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(DrawDevicesDisplay(devState), ());
        OldDevice = NewDevice;
    }
    if OldDevice as ::core::ffi::c_int != NewDevice as ::core::ffi::c_int {
        uqm_lua_checked!(DrawDevice(
            (*devState).list[OldDevice as usize] as COUNT,
            (OldDevice as ::core::ffi::c_int
                - (*devState).topIndex as ::core::ffi::c_int) as COUNT,
            false,
        ), ());
    }
    if (NewDevice as ::core::ffi::c_int)
        < C2Rust_Unnamed_1::NUM_DEVICES.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(DrawDevice(
            (*devState).list[NewDevice as usize] as COUNT,
            (NewDevice as ::core::ffi::c_int
                - (*devState).topIndex as ::core::ffi::c_int) as COUNT,
            true,
        ), ());
    }
    UnbatchGraphics();
}
unsafe extern "C" fn UseCaster() -> BOOLEAN {
    if inHQSpace().0 != 0 {
        if uqm_lua_checked!(getGameStateUint(
            b"ARILOU_SPACE_SIDE\0".as_ptr() as *const ::core::ffi::c_char,
        ), BOOLEAN(0)) <= 1 as DWORD
        {
            uqm_lua_checked!(setGameStateUint(
                b"USED_BROADCASTER\0".as_ptr() as *const ::core::ffi::c_char,
                1 as DWORD,
            ), BOOLEAN(0));
            return BOOLEAN::TRUE;
        }
        return BOOLEAN::FALSE;
    }
    if GlobData.Game_state.CurrentActivity as BYTE as ::core::ffi::c_int
        != C2Rust_Unnamed_2::IN_INTERPLANETARY.0 as ::core::ffi::c_int
        || !playerInSolarSystem()
    {
        return BOOLEAN::FALSE;
    }
    if playerInPlanetOrbit() as ::core::ffi::c_int != 0
        && matchWorld(
            pSolarSysState,
            (*pSolarSysState).pOrbitalDesc,
            -2 as ::core::ffi::c_int as BYTE,
            -1 as ::core::ffi::c_int as BYTE,
        ) as ::core::ffi::c_int != 0
        && (*CurStarDescPtr).Index as ::core::ffi::c_int
            == C2Rust_Unnamed_3::CHMMR_DEFINED.0 as ::core::ffi::c_int
        && uqm_lua_checked!(getGameStateUint(b"CHMMR_UNLEASHED\0".as_ptr() as *const ::core::ffi::c_char), BOOLEAN(0))
            == 0
    {
        NextActivity = (NextActivity as ::core::ffi::c_int
            | C2Rust_Unnamed_2::CHECK_LOAD.0 as ::core::ffi::c_int) as ACTIVITY;
        GlobData.Game_state.CurrentActivity = (GlobData.Game_state.CurrentActivity
            as ::core::ffi::c_int
            | C2Rust_Unnamed_2::START_ENCOUNTER.0 as ::core::ffi::c_int) as ACTIVITY;
        EncounterGroup = 0 as BYTE;
        PutGroupInfo(
            0 as ::core::ffi::c_long as DWORD,
            !(0 as ::core::ffi::c_int) as BYTE,
        );
        ReinitQueue(&raw mut GlobData.Game_state.ip_group_q);
        (CountLinks(&raw mut GlobData.Game_state.npc_built_ship_q) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
            || {
                __assert_fail(
                    b"CountLinks (&GLOBAL (npc_built_ship_q)) == 0\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"src/uqm/planets/devices.c\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    258 as ::core::ffi::c_int,
                    b"UseCaster\0".as_ptr() as *const ::core::ffi::c_char,
                );
                0 as ::core::ffi::c_int != 0
            }) as ::core::ffi::c_int;
        uqm_lua_checked!(setGameStateUint(
            b"GLOBAL_FLAGS_AND_DATA\0".as_ptr() as *const ::core::ffi::c_char,
            ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as DWORD,
        ), BOOLEAN(0));
        uqm_lua_checked!(SaveSolarSysLocation(), BOOLEAN(0));
        return BOOLEAN::TRUE;
    }
    let mut FoundIlwrath: BOOLEAN = BOOLEAN::FALSE;
    let mut hGroup: HIPGROUP = ::core::ptr::null_mut::<::core::ffi::c_void>();
    FoundIlwrath = BOOLEAN(
        ((*CurStarDescPtr).Index as ::core::ffi::c_int
            == C2Rust_Unnamed_3::ILWRATH_DEFINED.0 as ::core::ffi::c_int
            && StartSphereTracking(RACE_ID::ILWRATH_SHIP) as ::core::ffi::c_int != 0)
            as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
    if FoundIlwrath.0 == 0
        && {
            hGroup = GlobData.Game_state.ip_group_q.head as HIPGROUP;
            !hGroup.is_null()
        }
    {
        let mut GroupPtr: *mut IP_GROUP = ::core::ptr::null_mut::<IP_GROUP>();
        GroupPtr = LockIpGroup(&raw mut GlobData.Game_state.ip_group_q, hGroup);
        FoundIlwrath = BOOLEAN(
            ((*GroupPtr).race_id as ::core::ffi::c_int
                == RACE_ID::ILWRATH_SHIP.0 as ::core::ffi::c_int) as ::core::ffi::c_int
                as ::core::ffi::c_uint,
        );
        UnlockLink(&raw mut GlobData.Game_state.ip_group_q, hGroup);
    }
    if FoundIlwrath.0 != 0 {
        NextActivity = (NextActivity as ::core::ffi::c_int
            | C2Rust_Unnamed_2::CHECK_LOAD.0 as ::core::ffi::c_int) as ACTIVITY;
        GlobData.Game_state.CurrentActivity = (GlobData.Game_state.CurrentActivity
            as ::core::ffi::c_int
            | C2Rust_Unnamed_2::START_ENCOUNTER.0 as ::core::ffi::c_int) as ACTIVITY;
        EncounterGroup = 0 as BYTE;
        PutGroupInfo(
            0 as ::core::ffi::c_long as DWORD,
            !(0 as ::core::ffi::c_int) as BYTE,
        );
        ReinitQueue(&raw mut GlobData.Game_state.ip_group_q);
        (CountLinks(&raw mut GlobData.Game_state.npc_built_ship_q) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
            || {
                __assert_fail(
                    b"CountLinks (&GLOBAL (npc_built_ship_q)) == 0\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"src/uqm/planets/devices.c\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    292 as ::core::ffi::c_int,
                    b"UseCaster\0".as_ptr() as *const ::core::ffi::c_char,
                );
                0 as ::core::ffi::c_int != 0
            }) as ::core::ffi::c_int;
        if (*CurStarDescPtr).Index as ::core::ffi::c_int
            == C2Rust_Unnamed_3::ILWRATH_DEFINED.0 as ::core::ffi::c_int
        {
            uqm_lua_checked!(setGameStateUint(
                b"GLOBAL_FLAGS_AND_DATA\0".as_ptr() as *const ::core::ffi::c_char,
                ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as DWORD,
            ), BOOLEAN(0));
        } else {
            uqm_lua_checked!(setGameStateUint(
                b"GLOBAL_FLAGS_AND_DATA\0".as_ptr() as *const ::core::ffi::c_char,
                ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as DWORD,
            ), BOOLEAN(0));
        }
        if playerInPlanetOrbit() {
            uqm_lua_checked!(SaveSolarSysLocation(), BOOLEAN(0));
        }
        return BOOLEAN::TRUE;
    }
    return BOOLEAN::FALSE;
}
unsafe extern "C" fn InvokeDevice(mut which_device: BYTE) -> DeviceStatus {
    let mut val: BYTE = 0;
    match which_device as ::core::ffi::c_int {
        4 => {
            val = uqm_lua_checked!(getGameStateUint(
                b"ULTRON_CONDITION\0".as_ptr() as *const ::core::ffi::c_char,
            ), DeviceStatus(0)) as BYTE;
            if val != 0 {
                uqm_lua_checked!(setGameStateUint(
                    b"ULTRON_CONDITION\0".as_ptr() as *const ::core::ffi::c_char,
                    (val as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as DWORD,
                ), DeviceStatus(0));
                uqm_lua_checked!(setGameStateUint(
                    b"ROSY_SPHERE_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
                    0 as DWORD,
                ), DeviceStatus(0));
                uqm_lua_checked!(setGameStateUint(
                    b"DISCUSSED_ULTRON\0".as_ptr() as *const ::core::ffi::c_char,
                    0 as DWORD,
                ), DeviceStatus(0));
                uqm_lua_checked!(setGameStateUint(
                    b"SUPOX_ULTRON_HELP\0".as_ptr() as *const ::core::ffi::c_char,
                    0 as DWORD,
                ), DeviceStatus(0));
                return DeviceStatus::DEVICE_SUCCESS;
            }
        }
        3 => {
            if GlobData.Game_state.CurrentActivity as BYTE as ::core::ffi::c_int
                == C2Rust_Unnamed_2::IN_INTERPLANETARY.0 as ::core::ffi::c_int
                && playerInPlanetOrbit() as ::core::ffi::c_int != 0
            {
                uqm_lua_checked!(PlayMenuSound(MENU_SOUND_EFFECT::MENU_SOUND_INVOKED), DeviceStatus(0));
                uqm_lua_checked!(SleepThreadUntil(
                    FadeScreen(
                            ScreenFadeType::FadeAllToWhite,
                            (840 as ::core::ffi::c_int * 1 as ::core::ffi::c_int) as SIZE,
                        )
                        .wrapping_add(
                            (840 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                                as TimeCount,
                        ),
                ), DeviceStatus(0));
                if (*CurStarDescPtr).Index as ::core::ffi::c_int
                    != C2Rust_Unnamed_3::CHMMR_DEFINED.0 as ::core::ffi::c_int
                    || !matchWorld(
                        pSolarSysState,
                        (*pSolarSysState).pOrbitalDesc,
                        -2 as ::core::ffi::c_int as BYTE,
                        -1 as ::core::ffi::c_int as BYTE,
                    )
                {
                    FadeScreen(
                        ScreenFadeType::FadeAllToColor,
                        (840 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as SIZE,
                    );
                } else {
                    uqm_lua_checked!(setGameStateUint(
                        b"CHMMR_EMERGING\0".as_ptr() as *const ::core::ffi::c_char,
                        1 as DWORD,
                    ), DeviceStatus(0));
                    EncounterGroup = 0 as BYTE;
                    GlobData.Game_state.CurrentActivity = (GlobData
                        .Game_state
                        .CurrentActivity as ::core::ffi::c_int
                        | C2Rust_Unnamed_2::START_ENCOUNTER.0 as ::core::ffi::c_int)
                        as ACTIVITY;
                    PutGroupInfo(
                        0 as ::core::ffi::c_long as DWORD,
                        !(0 as ::core::ffi::c_int) as BYTE,
                    );
                    ReinitQueue(&raw mut GlobData.Game_state.ip_group_q);
                    (CountLinks(&raw mut GlobData.Game_state.npc_built_ship_q)
                        as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                        || {
                            __assert_fail(
                                b"CountLinks (&GLOBAL (npc_built_ship_q)) == 0\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"src/uqm/planets/devices.c\0"
                                    .as_ptr() as *const ::core::ffi::c_char,
                                359 as ::core::ffi::c_int,
                                b"InvokeDevice\0".as_ptr() as *const ::core::ffi::c_char,
                            );
                            0 as ::core::ffi::c_int != 0
                        }) as ::core::ffi::c_int;
                    CloneShipFragment(
                        RACE_ID::CHMMR_SHIP,
                        &raw mut GlobData.Game_state.npc_built_ship_q,
                        0 as COUNT,
                    );
                }
                return DeviceStatus::DEVICE_SUCCESS_NO_SOUND;
            }
        }
        2 => {
            uqm_lua_checked!(setGameStateUint(
                b"UTWIG_BOMB\0".as_ptr() as *const ::core::ffi::c_char,
                0 as DWORD,
            ), DeviceStatus(0));
            GlobData.Game_state.CurrentActivity = (GlobData.Game_state.CurrentActivity
                as ::core::ffi::c_int
                & !(C2Rust_Unnamed_2::IN_BATTLE.0 as ::core::ffi::c_int)) as ACTIVITY;
            GlobData.SIS_state.CrewEnlisted = !(0 as ::core::ffi::c_int) as COUNT;
            DeathBySuicide = BOOLEAN::TRUE;
            return DeviceStatus::DEVICE_SUCCESS;
        }
        1 => {
            NextActivity = (NextActivity as ::core::ffi::c_int
                | C2Rust_Unnamed_2::CHECK_LOAD.0 as ::core::ffi::c_int) as ACTIVITY;
            GlobData.Game_state.CurrentActivity = (GlobData.Game_state.CurrentActivity
                as ::core::ffi::c_int
                | C2Rust_Unnamed_2::START_ENCOUNTER.0 as ::core::ffi::c_int) as ACTIVITY;
            uqm_lua_checked!(setGameStateUint(
                b"GLOBAL_FLAGS_AND_DATA\0".as_ptr() as *const ::core::ffi::c_char,
                0 as DWORD,
            ), DeviceStatus(0));
            if inHQSpace().0 != 0 {
                if !GlobData.Game_state.encounter_q.head.is_null() {
                    uqm_lua_checked!(setGameStateUint(
                        b"SHIP_TO_COMPEL\0".as_ptr() as *const ::core::ffi::c_char,
                        1 as DWORD,
                    ), DeviceStatus(0));
                }
                GlobData.Game_state.CurrentActivity = (GlobData
                    .Game_state
                    .CurrentActivity as ::core::ffi::c_int
                    & !(C2Rust_Unnamed_2::IN_BATTLE.0 as ::core::ffi::c_int))
                    as ACTIVITY;
                SaveSisHyperState();
            } else {
                EncounterGroup = 0 as BYTE;
                if !GlobData.Game_state.ip_group_q.head.is_null() {
                    uqm_lua_checked!(setGameStateUint(
                        b"SHIP_TO_COMPEL\0".as_ptr() as *const ::core::ffi::c_char,
                        1 as DWORD,
                    ), DeviceStatus(0));
                    PutGroupInfo(
                        0 as ::core::ffi::c_long as DWORD,
                        !(0 as ::core::ffi::c_int) as BYTE,
                    );
                    ReinitQueue(&raw mut GlobData.Game_state.ip_group_q);
                    (CountLinks(&raw mut GlobData.Game_state.npc_built_ship_q)
                        as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                        || {
                            __assert_fail(
                                b"CountLinks (&GLOBAL (npc_built_ship_q)) == 0\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"src/uqm/planets/devices.c\0"
                                    .as_ptr() as *const ::core::ffi::c_char,
                                406 as ::core::ffi::c_int,
                                b"InvokeDevice\0".as_ptr() as *const ::core::ffi::c_char,
                            );
                            0 as ::core::ffi::c_int != 0
                        }) as ::core::ffi::c_int;
                }
                if (*CurStarDescPtr).Index as ::core::ffi::c_int
                    == C2Rust_Unnamed_3::SAMATRA_DEFINED.0 as ::core::ffi::c_int
                {
                    uqm_lua_checked!(setGameStateUint(
                        b"READY_TO_CONFUSE_URQUAN\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1 as DWORD,
                    ), DeviceStatus(0));
                }
                if playerInPlanetOrbit() {
                    uqm_lua_checked!(SaveSolarSysLocation(), DeviceStatus(0));
                }
            }
            return DeviceStatus::DEVICE_SUCCESS;
        }
        5 => {
            val = uqm_lua_checked!(getGameStateUint(
                b"ULTRON_CONDITION\0".as_ptr() as *const ::core::ffi::c_char,
            ), DeviceStatus(0)) as BYTE;
            if val != 0 {
                uqm_lua_checked!(setGameStateUint(
                    b"ULTRON_CONDITION\0".as_ptr() as *const ::core::ffi::c_char,
                    (val as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as DWORD,
                ), DeviceStatus(0));
                uqm_lua_checked!(setGameStateUint(
                    b"AQUA_HELIX_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
                    0 as DWORD,
                ), DeviceStatus(0));
                uqm_lua_checked!(setGameStateUint(
                    b"DISCUSSED_ULTRON\0".as_ptr() as *const ::core::ffi::c_char,
                    0 as DWORD,
                ), DeviceStatus(0));
                uqm_lua_checked!(setGameStateUint(
                    b"SUPOX_ULTRON_HELP\0".as_ptr() as *const ::core::ffi::c_char,
                    0 as DWORD,
                ), DeviceStatus(0));
                return DeviceStatus::DEVICE_SUCCESS;
            }
        }
        6 => {
            val = uqm_lua_checked!(getGameStateUint(
                b"ULTRON_CONDITION\0".as_ptr() as *const ::core::ffi::c_char,
            ), DeviceStatus(0)) as BYTE;
            if val != 0 {
                uqm_lua_checked!(setGameStateUint(
                    b"ULTRON_CONDITION\0".as_ptr() as *const ::core::ffi::c_char,
                    (val as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as DWORD,
                ), DeviceStatus(0));
                uqm_lua_checked!(setGameStateUint(
                    b"CLEAR_SPINDLE_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
                    0 as DWORD,
                ), DeviceStatus(0));
                uqm_lua_checked!(setGameStateUint(
                    b"DISCUSSED_ULTRON\0".as_ptr() as *const ::core::ffi::c_char,
                    0 as DWORD,
                ), DeviceStatus(0));
                uqm_lua_checked!(setGameStateUint(
                    b"SUPOX_ULTRON_HELP\0".as_ptr() as *const ::core::ffi::c_char,
                    0 as DWORD,
                ), DeviceStatus(0));
                return DeviceStatus::DEVICE_SUCCESS;
            }
        }
        12 | 13 => {
            if uqm_lua_checked!(UseCaster(), DeviceStatus(0)).0 != 0 {
                return DeviceStatus::DEVICE_SUCCESS;
            }
        }
        0 => {
            if (uqm_lua_checked!(inHyperSpace(), DeviceStatus(0)).0 != 0
                || if GlobData.SIS_state.Extended as ::core::ffi::c_int != 0 {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0 && inHQSpace().0 != 0)
                && GlobData.SIS_state.FuelOnBoard
                    >= (if if GlobData.SIS_state.Difficulty as ::core::ffi::c_int != 0 {
                        GlobData.SIS_state.Difficulty as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    } == 0 as ::core::ffi::c_int
                    {
                        10 as ::core::ffi::c_int
                    } else if if GlobData.SIS_state.Difficulty as ::core::ffi::c_int != 0
                    {
                        GlobData.SIS_state.Difficulty as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    } == 1 as ::core::ffi::c_int
                    {
                        5 as ::core::ffi::c_int
                    } else {
                        20 as ::core::ffi::c_int
                    } * 100 as ::core::ffi::c_int) as DWORD
            {
                if optInfiniteFuel.0 == 0
                    || if GlobData.SIS_state.Extended as ::core::ffi::c_int != 0 {
                        BOOLEAN::TRUE.0 as ::core::ffi::c_int
                    } else {
                        BOOLEAN::FALSE.0 as ::core::ffi::c_int
                    } != 0 && optInfiniteFuel.0 == 0 && uqm_lua_checked!(inHyperSpace(), DeviceStatus(0)).0 != 0
                {
                    GlobData.SIS_state.FuelOnBoard = GlobData
                        .SIS_state
                        .FuelOnBoard
                        .wrapping_sub(
                            (if if GlobData.SIS_state.Difficulty as ::core::ffi::c_int
                                != 0
                            {
                                GlobData.SIS_state.Difficulty as ::core::ffi::c_int
                            } else {
                                0 as ::core::ffi::c_int
                            } == 0 as ::core::ffi::c_int
                            {
                                10 as ::core::ffi::c_int
                            } else if if GlobData.SIS_state.Difficulty
                                as ::core::ffi::c_int != 0
                            {
                                GlobData.SIS_state.Difficulty as ::core::ffi::c_int
                            } else {
                                0 as ::core::ffi::c_int
                            } == 1 as ::core::ffi::c_int
                            {
                                5 as ::core::ffi::c_int
                            } else {
                                20 as ::core::ffi::c_int
                            } * 100 as ::core::ffi::c_int) as DWORD,
                        );
                }
                if if GlobData.SIS_state.Extended as ::core::ffi::c_int != 0 {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0 && uqm_lua_checked!(inHyperSpace(), DeviceStatus(0)).0 != 0
                {
                    uqm_lua_checked!(SaveLastLoc(
                        MAKE_POINT(
                            logxToUniverse(GlobData.SIS_state.log_x) as COORD,
                            logyToUniverse(GlobData.SIS_state.log_y) as COORD,
                        ),
                    ), DeviceStatus(0));
                }
                uqm_lua_checked!(setGameStateUint(
                    b"PORTAL_COUNTER\0".as_ptr() as *const ::core::ffi::c_char,
                    1 as DWORD,
                ), DeviceStatus(0));
                return DeviceStatus::DEVICE_SUCCESS;
            }
        }
        25 | 26 | 7 | 8 | 9 | 10 | 11 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 | 27
        | _ => {}
    }
    return DeviceStatus::DEVICE_FAILURE;
}
#[no_mangle]
pub unsafe extern "C" fn InvokeSpawner() -> BOOLEAN {
    let mut status: DeviceStatus = uqm_lua_checked!(InvokeDevice(
        C2Rust_Unnamed_1::PORTAL_SPAWNER_DEVICE.0 as ::core::ffi::c_int as BYTE,
    ), BOOLEAN(0));
    if status.0 == DeviceStatus::DEVICE_FAILURE.0 {
        uqm_lua_checked!(PlayMenuSound(MENU_SOUND_EFFECT::MENU_SOUND_FAILURE), BOOLEAN(0));
    } else if status.0 == DeviceStatus::DEVICE_SUCCESS.0 {
        uqm_lua_checked!(PlayMenuSound(MENU_SOUND_EFFECT::MENU_SOUND_INVOKED), BOOLEAN(0));
    }
    return BOOLEAN(
        (status.0 == DeviceStatus::DEVICE_FAILURE.0) as ::core::ffi::c_int
            as ::core::ffi::c_uint,
    );
}
unsafe extern "C" fn DoManipulateDevices(mut pMS: *mut MENU_STATE) -> BOOLEAN {
    let mut devState: *mut DEVICES_STATE = (*pMS).privData as *mut DEVICES_STATE;
    let mut select: BOOLEAN = BOOLEAN::FALSE;
    let mut cancel: BOOLEAN = BOOLEAN::FALSE;
    let mut back: BOOLEAN = BOOLEAN::FALSE;
    let mut forward: BOOLEAN = BOOLEAN::FALSE;
    let mut pagefwd: BOOLEAN = BOOLEAN::FALSE;
    let mut pageback: BOOLEAN = BOOLEAN::FALSE;
    select = BOOLEAN(
        PulsedInputState
            .menu[C2Rust_Unnamed::KEY_MENU_SELECT.0 as ::core::ffi::c_int as usize]
            as ::core::ffi::c_uint,
    );
    cancel = BOOLEAN(
        PulsedInputState
            .menu[C2Rust_Unnamed::KEY_MENU_CANCEL.0 as ::core::ffi::c_int as usize]
            as ::core::ffi::c_uint,
    );
    back = BOOLEAN(
        (PulsedInputState
            .menu[C2Rust_Unnamed::KEY_MENU_UP.0 as ::core::ffi::c_int as usize] != 0
            || PulsedInputState
                .menu[C2Rust_Unnamed::KEY_MENU_LEFT.0 as ::core::ffi::c_int as usize]
                != 0) as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
    forward = BOOLEAN(
        (PulsedInputState
            .menu[C2Rust_Unnamed::KEY_MENU_DOWN.0 as ::core::ffi::c_int as usize] != 0
            || PulsedInputState
                .menu[C2Rust_Unnamed::KEY_MENU_RIGHT.0 as ::core::ffi::c_int as usize]
                != 0) as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
    pagefwd = BOOLEAN(
        PulsedInputState
            .menu[C2Rust_Unnamed::KEY_MENU_PAGE_DOWN.0 as ::core::ffi::c_int as usize]
            as ::core::ffi::c_uint,
    );
    pageback = BOOLEAN(
        PulsedInputState
            .menu[C2Rust_Unnamed::KEY_MENU_PAGE_UP.0 as ::core::ffi::c_int as usize]
            as ::core::ffi::c_uint,
    );
    if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
        & (C2Rust_Unnamed_2::CHECK_ABORT.0 as ::core::ffi::c_int
            | C2Rust_Unnamed_2::CHECK_LOAD.0 as ::core::ffi::c_int) != 0
    {
        return BOOLEAN::FALSE;
    }
    if cancel.0 != 0 {
        return BOOLEAN::FALSE
    } else if select.0 != 0 {
        let mut status: DeviceStatus = DeviceStatus::DEVICE_FAILURE;
        status = uqm_lua_checked!(InvokeDevice((*devState).list[(*pMS).CurState as usize]), BOOLEAN(0));
        if status.0 == DeviceStatus::DEVICE_FAILURE.0 {
            uqm_lua_checked!(PlayMenuSound(MENU_SOUND_EFFECT::MENU_SOUND_FAILURE), BOOLEAN(0));
        } else if status.0 == DeviceStatus::DEVICE_SUCCESS.0 {
            uqm_lua_checked!(PlayMenuSound(MENU_SOUND_EFFECT::MENU_SOUND_INVOKED), BOOLEAN(0));
        }
        return BOOLEAN(
            (status.0 == DeviceStatus::DEVICE_FAILURE.0) as ::core::ffi::c_int
                as ::core::ffi::c_uint,
        );
    } else {
        let mut NewTop: SIZE = 0;
        let mut NewState: SIZE = 0;
        NewTop = (*devState).topIndex as SIZE;
        NewState = (*pMS).CurState as SIZE;
        if back.0 != 0 {
            NewState -= 1;
        } else if forward.0 != 0 {
            NewState += 1;
        } else if pagefwd.0 != 0 {
            NewState = (NewState as ::core::ffi::c_int
                + (((129 as ::core::ffi::c_int) << resolutionFactor)
                    - ((33 as ::core::ffi::c_int) << resolutionFactor))
                    / (((16 as ::core::ffi::c_int) << resolutionFactor)
                        + ((2 as ::core::ffi::c_int) << resolutionFactor))) as SIZE;
        } else if pageback.0 != 0 {
            NewState = (NewState as ::core::ffi::c_int
                - (((129 as ::core::ffi::c_int) << resolutionFactor)
                    - ((33 as ::core::ffi::c_int) << resolutionFactor))
                    / (((16 as ::core::ffi::c_int) << resolutionFactor)
                        + ((2 as ::core::ffi::c_int) << resolutionFactor))) as SIZE;
        }
        if (NewState as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
            NewState = 0 as SIZE;
        } else if NewState as ::core::ffi::c_int
            >= (*devState).count as ::core::ffi::c_int
        {
            NewState = ((*devState).count as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as SIZE;
        }
        if (NewState as ::core::ffi::c_int) < NewTop as ::core::ffi::c_int
            || NewState as ::core::ffi::c_int
                >= NewTop as ::core::ffi::c_int
                    + (((129 as ::core::ffi::c_int) << resolutionFactor)
                        - ((33 as ::core::ffi::c_int) << resolutionFactor))
                        / (((16 as ::core::ffi::c_int) << resolutionFactor)
                            + ((2 as ::core::ffi::c_int) << resolutionFactor))
        {
            NewTop = (NewState as ::core::ffi::c_int
                - NewState as ::core::ffi::c_int
                    % ((((129 as ::core::ffi::c_int) << resolutionFactor)
                        - ((33 as ::core::ffi::c_int) << resolutionFactor))
                        / (((16 as ::core::ffi::c_int) << resolutionFactor)
                            + ((2 as ::core::ffi::c_int) << resolutionFactor)))) as SIZE;
        }
        if NewState as ::core::ffi::c_int != (*pMS).CurState as ::core::ffi::c_int {
            if NewTop as ::core::ffi::c_int != (*devState).topIndex as ::core::ffi::c_int
            {
                (*devState).topIndex = NewTop as COUNT;
                uqm_lua_checked!(DrawDevices(
                    devState,
                    !(0 as ::core::ffi::c_int) as COUNT,
                    NewState as COUNT,
                ), BOOLEAN(0));
            } else {
                uqm_lua_checked!(DrawDevices(devState, (*pMS).CurState as COUNT, NewState as COUNT), BOOLEAN(0));
            }
            (*pMS).CurState = NewState as BYTE;
        }
    }
    return BOOLEAN::TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn InventoryDevices(
    mut pDeviceMap: *mut BYTE,
    mut Size: COUNT,
) -> SIZE {
    let mut i: BYTE = 0;
    let mut DevicesOnBoard: SIZE = 0;
    DevicesOnBoard = 0 as SIZE;
    i = 0 as BYTE;
    while (i as ::core::ffi::c_int)
        < C2Rust_Unnamed_1::NUM_DEVICES.0 as ::core::ffi::c_int
        && Size as ::core::ffi::c_int > 0 as ::core::ffi::c_int
    {
        let mut DeviceState: BYTE = 0;
        DeviceState = 0 as BYTE;
        match i as ::core::ffi::c_int {
            4 => {
                DeviceState = uqm_lua_checked!(getGameStateUint(
                    b"ROSY_SPHERE_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
                ), 0) as BYTE;
            }
            25 => {
                DeviceState = uqm_lua_checked!(getGameStateUint(
                    b"WIMBLIS_TRIDENT_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
                ), 0) as BYTE;
            }
            26 => {
                DeviceState = uqm_lua_checked!(getGameStateUint(
                    b"GLOWING_ROD_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
                ), 0) as BYTE;
            }
            3 => {
                DeviceState = uqm_lua_checked!(getGameStateUint(
                    b"SUN_DEVICE_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
                ), 0) as BYTE;
            }
            2 => {
                DeviceState = uqm_lua_checked!(getGameStateUint(
                    b"UTWIG_BOMB_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
                ), 0) as BYTE;
            }
            7 => {
                DeviceState = (uqm_lua_checked!(getGameStateUint(
                    b"ULTRON_CONDITION\0".as_ptr() as *const ::core::ffi::c_char,
                ), 0) == 1 as DWORD) as ::core::ffi::c_int as BYTE;
            }
            8 => {
                DeviceState = (uqm_lua_checked!(getGameStateUint(
                    b"ULTRON_CONDITION\0".as_ptr() as *const ::core::ffi::c_char,
                ), 0) == 2 as DWORD) as ::core::ffi::c_int as BYTE;
            }
            9 => {
                DeviceState = (uqm_lua_checked!(getGameStateUint(
                    b"ULTRON_CONDITION\0".as_ptr() as *const ::core::ffi::c_char,
                ), 0) == 3 as DWORD) as ::core::ffi::c_int as BYTE;
            }
            10 => {
                DeviceState = (uqm_lua_checked!(getGameStateUint(
                    b"ULTRON_CONDITION\0".as_ptr() as *const ::core::ffi::c_char,
                ), 0) == 4 as DWORD) as ::core::ffi::c_int as BYTE;
            }
            11 => {
                DeviceState = uqm_lua_checked!(getGameStateUint(
                    b"MAIDENS_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
                ), 0) as BYTE;
            }
            1 => {
                DeviceState = uqm_lua_checked!(getGameStateUint(
                    b"TALKING_PET_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
                ), 0) as BYTE;
            }
            5 => {
                DeviceState = uqm_lua_checked!(getGameStateUint(
                    b"AQUA_HELIX_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
                ), 0) as BYTE;
            }
            6 => {
                DeviceState = uqm_lua_checked!(getGameStateUint(
                    b"CLEAR_SPINDLE_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
                ), 0) as BYTE;
            }
            12 => {
                DeviceState = uqm_lua_checked!(getGameStateUint(
                    b"UMGAH_BROADCASTERS_ON_SHIP\0".as_ptr()
                        as *const ::core::ffi::c_char,
                ), 0) as BYTE;
            }
            17 => {
                DeviceState = uqm_lua_checked!(getGameStateUint(
                    b"TAALO_PROTECTOR_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
                ), 0) as BYTE;
            }
            18 => {
                DeviceState = uqm_lua_checked!(getGameStateUint(
                    b"EGG_CASE0_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
                ), 0) as BYTE;
            }
            19 => {
                DeviceState = uqm_lua_checked!(getGameStateUint(
                    b"EGG_CASE1_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
                ), 0) as BYTE;
            }
            20 => {
                DeviceState = uqm_lua_checked!(getGameStateUint(
                    b"EGG_CASE2_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
                ), 0) as BYTE;
            }
            21 => {
                DeviceState = uqm_lua_checked!(getGameStateUint(
                    b"SYREEN_SHUTTLE_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
                ), 0) as BYTE;
            }
            22 => {
                DeviceState = uqm_lua_checked!(getGameStateUint(
                    b"VUX_BEAST_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
                ), 0) as BYTE;
            }
            0 => {
                DeviceState = uqm_lua_checked!(getGameStateUint(
                    b"PORTAL_SPAWNER_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
                ), 0) as BYTE;
            }
            24 => {
                DeviceState = uqm_lua_checked!(getGameStateUint(
                    b"PORTAL_KEY_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
                ), 0) as BYTE;
            }
            13 => {
                DeviceState = uqm_lua_checked!(getGameStateUint(
                    b"BURV_BROADCASTERS_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
                ), 0) as BYTE;
            }
            27 => {
                DeviceState = uqm_lua_checked!(getGameStateUint(
                    b"MOONBASE_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
                ), 0) as BYTE;
            }
            23 | _ => {}
        }
        if DeviceState != 0 {
            let c2rust_fresh0 = pDeviceMap;
            pDeviceMap = pDeviceMap.offset(1);
            *c2rust_fresh0 = i;
            DevicesOnBoard += 1;
            Size = Size.wrapping_sub(1);
        }
        i = i.wrapping_add(1);
    }
    return DevicesOnBoard;
}
#[no_mangle]
pub unsafe extern "C" fn DevicesMenu() -> BOOLEAN {
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
    let mut DevicesState: DEVICES_STATE = DEVICES_STATE {
        list: [0; 28],
        count: 0,
        topIndex: 0,
    };
    memset(
        &raw mut MenuState as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<MENU_STATE>(),
    );
    MenuState.privData = &raw mut DevicesState as *mut ::core::ffi::c_void;
    memset(
        &raw mut DevicesState as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<DEVICES_STATE>(),
    );
    DevicesState.count = uqm_lua_checked!(InventoryDevices(
        &raw mut DevicesState.list as *mut BYTE,
        C2Rust_Unnamed_1::NUM_DEVICES.0 as ::core::ffi::c_int as COUNT,
    ), BOOLEAN(0)) as COUNT;
    if DevicesState.count == 0 {
        return BOOLEAN::FALSE;
    }
    uqm_lua_checked!(DrawDevices(
        &raw mut DevicesState,
        !(0 as ::core::ffi::c_int) as COUNT,
        MenuState.CurState as COUNT,
    ), BOOLEAN(0));
    SetMenuSounds(
        (((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as MENU_SOUND_FLAGS
            as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as MENU_SOUND_FLAGS
                as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as MENU_SOUND_FLAGS
                as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as MENU_SOUND_FLAGS
                as ::core::ffi::c_int
            | (((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as MENU_SOUND_FLAGS
                as ::core::ffi::c_int
                | ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
                    as MENU_SOUND_FLAGS as ::core::ffi::c_int)) as MENU_SOUND_FLAGS,
        ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as MENU_SOUND_FLAGS,
    );
    if optWhichMenu == 0x2 as ::core::ffi::c_int {
        uqm_lua_checked!(DrawMenuStateStrings(
            C2Rust_Unnamed_5::PM_ALT_CARGO.0 as ::core::ffi::c_int as BYTE,
            1 as SWORD,
        ), BOOLEAN(0));
    }
    MenuState.InputFunc = Some(
        DoManipulateDevices as unsafe extern "C" fn(*mut MENU_STATE) -> BOOLEAN,
    ) as Option<unsafe extern "C" fn(*mut menu_state) -> BOOLEAN>;
    uqm_lua_checked!(DoInput(&raw mut MenuState as *mut ::core::ffi::c_void, BOOLEAN::TRUE), BOOLEAN(0));
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
    if GlobData.SIS_state.CrewEnlisted as ::core::ffi::c_int
        != !(0 as ::core::ffi::c_int) as COUNT as ::core::ffi::c_int
        && GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
            & C2Rust_Unnamed_2::CHECK_ABORT.0 as ::core::ffi::c_int == 0
    {
        uqm_lua_checked!(ClearSISRect(((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as BYTE), BOOLEAN(0));
        if uqm_lua_checked!(getGameStateUint(b"PORTAL_COUNTER\0".as_ptr() as *const ::core::ffi::c_char), BOOLEAN(0))
            == 0
            && GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
                & C2Rust_Unnamed_2::START_ENCOUNTER.0 as ::core::ffi::c_int == 0
            && GlobData.SIS_state.CrewEnlisted as ::core::ffi::c_int
                != !(0 as ::core::ffi::c_int) as COUNT as ::core::ffi::c_int
        {
            return BOOLEAN::TRUE;
        }
    }
    return BOOLEAN::FALSE;
}
