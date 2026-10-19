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
    );
    fn utf8StringPos(
        pStr: *const ::core::ffi::c_char,
        ch: UniChar,
    ) -> ::core::ffi::c_int;
    fn skipUTF8Chars(
        ptr: *const ::core::ffi::c_char,
        num: size_t,
    ) -> *mut ::core::ffi::c_char;
    fn SetAbsStringTableIndex(String: STRING, StringTableIndex: COUNT) -> STRING;
    fn GetStringAddress(String: STRING) -> STRINGPTR;
    fn ReinitQueue(pq: *mut QUEUE);
    fn CountLinks(pq: *mut QUEUE) -> COUNT;
    fn memset(
        _: *mut ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
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
    fn CloneShipFragment(
        shipIndex: COUNT,
        pDstQueue: *mut QUEUE,
        crew_level: COUNT,
    ) -> HSHIPFRAG;
    fn StartSphereTracking(race: COUNT) -> COUNT;
    static mut CurStarDescPtr: *mut STAR_DESC;
    static mut EncounterGroup: BYTE;
    fn SleepThread(timePeriod: TimePeriod);
    fn SleepThreadUntil(wakeTime: TimeCount);
    fn ClearSISRect(ClearFlags: BYTE);
    static mut GlobData: GLOBDATA;
    fn getGameState(
        state: *mut BYTE,
        startBit: ::core::ffi::c_int,
        endBit: ::core::ffi::c_int,
    ) -> BYTE;
    fn setGameState(
        state: *mut BYTE,
        startBit: ::core::ffi::c_int,
        endBit: ::core::ffi::c_int,
        val: BYTE,
    );
    fn inHQSpace() -> BOOLEAN;
    fn inHyperSpace() -> BOOLEAN;
    static mut GameStrings: STRING;
    static mut PulsedInputState: CONTROLLER_INPUT_STATE;
    fn DoInput(pInputState: *mut ::core::ffi::c_void, resetInput: BOOLEAN);
    static mut NextActivity: ACTIVITY;
    static mut MiscDataFrame: FRAME;
    static mut StatusContext: CONTEXT;
    static mut StarConFont: FONT;
    static mut TinyFont: FONT;
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
pub struct C2Rust_Unnamed(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed {
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
    pub const YEHAT_REBEL_SHIP: Self = Self(22);
    pub const URQUAN_DRONE_SHIP: Self = Self(23);
    pub const SAMATRA_SHIP: Self = Self(23);
    pub const NUM_AVAILABLE_RACES: Self = Self(24);
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
pub type PLANET_DESC = planet_desc;
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
}
pub type SOLARSYS_STATE = solarsys_state;
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
    pub ScratchArray: *mut Color,
    pub WorkFrame: FRAME,
}
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
    pub PlanetToSunDist: UWORD,
    pub PlanDataPtr: *const PlanetFrame,
    pub ScanSeed: [DWORD; 3],
    pub ScanRetrieveMask: [DWORD; 3],
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
pub type NODE_INFO = node_info;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct node_info {
    pub loc_pt: POINT,
    pub density: COUNT,
    pub r#type: COUNT,
}
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
pub struct star_desc {
    pub star_pt: POINT,
    pub Type: BYTE,
    pub Index: BYTE,
    pub Prefix: BYTE,
    pub Postfix: BYTE,
}
pub type STAR_DESC = star_desc;
pub type TimeCount = DWORD;
pub type TimePeriod = DWORD;
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
    pub flash_frame0: FRAME,
    pub flash_frame1: FRAME,
    pub flashContext: *mut FlashContext,
    pub hMusic: MUSIC_REF,
    pub privData: *mut ::core::ffi::c_void,
}
pub type MENU_STATE = menu_state;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_0 {
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
    pub const RAINBOW_DEFINED: Self = Self(42);
    pub const ILWRATH_DEFINED: Self = Self(43);
    pub const ANDROSYNTH_DEFINED: Self = Self(44);
    pub const MYCON_TRAP_DEFINED: Self = Self(45);
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
    pub ShipName: [UNICODE; 16],
    pub CommanderName: [UNICODE; 16],
    pub PlanetName: [UNICODE; 16],
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
    pub const ARTIFACT_2_DEVICE: Self = Self(25);
    pub const ARTIFACT_3_DEVICE: Self = Self(26);
    pub const LUNAR_BASE_DEVICE: Self = Self(27);
    pub const NUM_DEVICES: Self = Self(28);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_2(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_2 {
    pub const SHOFIXTI_VISITS: Self = Self(0);
    pub const END_SHOFIXTI_VISITS: Self = Self(2);
    pub const SHOFIXTI_STACK1: Self = Self(3);
    pub const END_SHOFIXTI_STACK1: Self = Self(4);
    pub const SHOFIXTI_STACK2: Self = Self(5);
    pub const END_SHOFIXTI_STACK2: Self = Self(7);
    pub const SHOFIXTI_STACK3: Self = Self(8);
    pub const END_SHOFIXTI_STACK3: Self = Self(9);
    pub const SHOFIXTI_KIA: Self = Self(10);
    pub const END_SHOFIXTI_KIA: Self = Self(10);
    pub const SHOFIXTI_BRO_KIA: Self = Self(11);
    pub const END_SHOFIXTI_BRO_KIA: Self = Self(11);
    pub const SHOFIXTI_RECRUITED: Self = Self(12);
    pub const END_SHOFIXTI_RECRUITED: Self = Self(12);
    pub const SHOFIXTI_MAIDENS: Self = Self(13);
    pub const END_SHOFIXTI_MAIDENS: Self = Self(13);
    pub const MAIDENS_ON_SHIP: Self = Self(14);
    pub const END_MAIDENS_ON_SHIP: Self = Self(14);
    pub const BATTLE_SEGUE: Self = Self(15);
    pub const END_BATTLE_SEGUE: Self = Self(15);
    pub const PLANETARY_LANDING: Self = Self(16);
    pub const END_PLANETARY_LANDING: Self = Self(16);
    pub const PLANETARY_CHANGE: Self = Self(17);
    pub const END_PLANETARY_CHANGE: Self = Self(17);
    pub const SPATHI_VISITS: Self = Self(18);
    pub const END_SPATHI_VISITS: Self = Self(20);
    pub const SPATHI_HOME_VISITS: Self = Self(21);
    pub const END_SPATHI_HOME_VISITS: Self = Self(23);
    pub const FOUND_PLUTO_SPATHI: Self = Self(24);
    pub const END_FOUND_PLUTO_SPATHI: Self = Self(25);
    pub const SPATHI_SHIELDED_SELVES: Self = Self(26);
    pub const END_SPATHI_SHIELDED_SELVES: Self = Self(26);
    pub const SPATHI_CREATURES_EXAMINED: Self = Self(27);
    pub const END_SPATHI_CREATURES_EXAMINED: Self = Self(27);
    pub const SPATHI_CREATURES_ELIMINATED: Self = Self(28);
    pub const END_SPATHI_CREATURES_ELIMINATED: Self = Self(28);
    pub const UMGAH_BROADCASTERS: Self = Self(29);
    pub const END_UMGAH_BROADCASTERS: Self = Self(29);
    pub const SPATHI_MANNER: Self = Self(30);
    pub const END_SPATHI_MANNER: Self = Self(31);
    pub const SPATHI_QUEST: Self = Self(32);
    pub const END_SPATHI_QUEST: Self = Self(32);
    pub const LIED_ABOUT_CREATURES: Self = Self(33);
    pub const END_LIED_ABOUT_CREATURES: Self = Self(34);
    pub const SPATHI_PARTY: Self = Self(35);
    pub const END_SPATHI_PARTY: Self = Self(35);
    pub const KNOW_SPATHI_PASSWORD: Self = Self(36);
    pub const END_KNOW_SPATHI_PASSWORD: Self = Self(36);
    pub const ILWRATH_HOME_VISITS: Self = Self(37);
    pub const END_ILWRATH_HOME_VISITS: Self = Self(39);
    pub const ILWRATH_CHMMR_VISITS: Self = Self(40);
    pub const END_ILWRATH_CHMMR_VISITS: Self = Self(40);
    pub const ARILOU_SPACE: Self = Self(41);
    pub const END_ARILOU_SPACE: Self = Self(41);
    pub const ARILOU_SPACE_SIDE: Self = Self(42);
    pub const END_ARILOU_SPACE_SIDE: Self = Self(43);
    pub const ARILOU_SPACE_COUNTER: Self = Self(44);
    pub const END_ARILOU_SPACE_COUNTER: Self = Self(47);
    pub const LANDER_SHIELDS: Self = Self(48);
    pub const END_LANDER_SHIELDS: Self = Self(51);
    pub const MET_MELNORME: Self = Self(52);
    pub const END_MET_MELNORME: Self = Self(52);
    pub const MELNORME_RESCUE_REFUSED: Self = Self(53);
    pub const END_MELNORME_RESCUE_REFUSED: Self = Self(53);
    pub const MELNORME_RESCUE_COUNT: Self = Self(54);
    pub const END_MELNORME_RESCUE_COUNT: Self = Self(56);
    pub const TRADED_WITH_MELNORME: Self = Self(57);
    pub const END_TRADED_WITH_MELNORME: Self = Self(57);
    pub const WHY_MELNORME_PURPLE: Self = Self(58);
    pub const END_WHY_MELNORME_PURPLE: Self = Self(58);
    pub const MELNORME_CREDIT0: Self = Self(59);
    pub const END_MELNORME_CREDIT0: Self = Self(66);
    pub const MELNORME_CREDIT1: Self = Self(67);
    pub const END_MELNORME_CREDIT1: Self = Self(74);
    pub const MELNORME_BUSINESS_COUNT: Self = Self(75);
    pub const END_MELNORME_BUSINESS_COUNT: Self = Self(76);
    pub const MELNORME_YACK_STACK0: Self = Self(77);
    pub const END_MELNORME_YACK_STACK0: Self = Self(78);
    pub const MELNORME_YACK_STACK1: Self = Self(79);
    pub const END_MELNORME_YACK_STACK1: Self = Self(80);
    pub const MELNORME_YACK_STACK2: Self = Self(81);
    pub const END_MELNORME_YACK_STACK2: Self = Self(84);
    pub const MELNORME_YACK_STACK3: Self = Self(85);
    pub const END_MELNORME_YACK_STACK3: Self = Self(87);
    pub const MELNORME_YACK_STACK4: Self = Self(88);
    pub const END_MELNORME_YACK_STACK4: Self = Self(89);
    pub const WHY_MELNORME_BLUE: Self = Self(90);
    pub const END_WHY_MELNORME_BLUE: Self = Self(90);
    pub const MELNORME_ANGER: Self = Self(91);
    pub const END_MELNORME_ANGER: Self = Self(92);
    pub const MELNORME_MIFFED_COUNT: Self = Self(93);
    pub const END_MELNORME_MIFFED_COUNT: Self = Self(94);
    pub const MELNORME_PISSED_COUNT: Self = Self(95);
    pub const END_MELNORME_PISSED_COUNT: Self = Self(96);
    pub const MELNORME_HATE_COUNT: Self = Self(97);
    pub const END_MELNORME_HATE_COUNT: Self = Self(98);
    pub const PROBE_MESSAGE_DELIVERED: Self = Self(99);
    pub const END_PROBE_MESSAGE_DELIVERED: Self = Self(99);
    pub const PROBE_ILWRATH_ENCOUNTER: Self = Self(100);
    pub const END_PROBE_ILWRATH_ENCOUNTER: Self = Self(100);
    pub const STARBASE_AVAILABLE: Self = Self(101);
    pub const END_STARBASE_AVAILABLE: Self = Self(101);
    pub const STARBASE_VISITED: Self = Self(102);
    pub const END_STARBASE_VISITED: Self = Self(102);
    pub const RADIOACTIVES_PROVIDED: Self = Self(103);
    pub const END_RADIOACTIVES_PROVIDED: Self = Self(103);
    pub const LANDERS_LOST: Self = Self(104);
    pub const END_LANDERS_LOST: Self = Self(104);
    pub const GIVEN_FUEL_BEFORE: Self = Self(105);
    pub const END_GIVEN_FUEL_BEFORE: Self = Self(105);
    pub const AWARE_OF_SAMATRA: Self = Self(106);
    pub const END_AWARE_OF_SAMATRA: Self = Self(106);
    pub const YEHAT_CAVALRY_ARRIVED: Self = Self(107);
    pub const END_YEHAT_CAVALRY_ARRIVED: Self = Self(107);
    pub const URQUAN_MESSED_UP: Self = Self(108);
    pub const END_URQUAN_MESSED_UP: Self = Self(108);
    pub const MOONBASE_DESTROYED: Self = Self(109);
    pub const END_MOONBASE_DESTROYED: Self = Self(109);
    pub const WILL_DESTROY_BASE: Self = Self(110);
    pub const END_WILL_DESTROY_BASE: Self = Self(110);
    pub const ARTIFACT_2_ON_SHIP: Self = Self(111);
    pub const END_ARTIFACT_2_ON_SHIP: Self = Self(111);
    pub const ARTIFACT_3_ON_SHIP: Self = Self(112);
    pub const END_ARTIFACT_3_ON_SHIP: Self = Self(112);
    pub const KOHR_AH_KILLED_ALL: Self = Self(113);
    pub const END_KOHR_AH_KILLED_ALL: Self = Self(113);
    pub const STARBASE_YACK_STACK1: Self = Self(114);
    pub const END_STARBASE_YACK_STACK1: Self = Self(114);
    pub const DISCUSSED_PORTAL_SPAWNER: Self = Self(115);
    pub const END_DISCUSSED_PORTAL_SPAWNER: Self = Self(115);
    pub const DISCUSSED_TALKING_PET: Self = Self(116);
    pub const END_DISCUSSED_TALKING_PET: Self = Self(116);
    pub const DISCUSSED_UTWIG_BOMB: Self = Self(117);
    pub const END_DISCUSSED_UTWIG_BOMB: Self = Self(117);
    pub const DISCUSSED_SUN_EFFICIENCY: Self = Self(118);
    pub const END_DISCUSSED_SUN_EFFICIENCY: Self = Self(118);
    pub const DISCUSSED_ROSY_SPHERE: Self = Self(119);
    pub const END_DISCUSSED_ROSY_SPHERE: Self = Self(119);
    pub const DISCUSSED_AQUA_HELIX: Self = Self(120);
    pub const END_DISCUSSED_AQUA_HELIX: Self = Self(120);
    pub const DISCUSSED_CLEAR_SPINDLE: Self = Self(121);
    pub const END_DISCUSSED_CLEAR_SPINDLE: Self = Self(121);
    pub const DISCUSSED_ULTRON: Self = Self(122);
    pub const END_DISCUSSED_ULTRON: Self = Self(122);
    pub const DISCUSSED_MAIDENS: Self = Self(123);
    pub const END_DISCUSSED_MAIDENS: Self = Self(123);
    pub const DISCUSSED_UMGAH_HYPERWAVE: Self = Self(124);
    pub const END_DISCUSSED_UMGAH_HYPERWAVE: Self = Self(124);
    pub const DISCUSSED_BURVIX_HYPERWAVE: Self = Self(125);
    pub const END_DISCUSSED_BURVIX_HYPERWAVE: Self = Self(125);
    pub const SYREEN_WANT_PROOF: Self = Self(126);
    pub const END_SYREEN_WANT_PROOF: Self = Self(126);
    pub const PLAYER_HAVING_SEX: Self = Self(127);
    pub const END_PLAYER_HAVING_SEX: Self = Self(127);
    pub const MET_ARILOU: Self = Self(128);
    pub const END_MET_ARILOU: Self = Self(128);
    pub const DISCUSSED_TAALO_PROTECTOR: Self = Self(129);
    pub const END_DISCUSSED_TAALO_PROTECTOR: Self = Self(129);
    pub const DISCUSSED_EGG_CASING0: Self = Self(130);
    pub const END_DISCUSSED_EGG_CASING0: Self = Self(130);
    pub const DISCUSSED_EGG_CASING1: Self = Self(131);
    pub const END_DISCUSSED_EGG_CASING1: Self = Self(131);
    pub const DISCUSSED_EGG_CASING2: Self = Self(132);
    pub const END_DISCUSSED_EGG_CASING2: Self = Self(132);
    pub const DISCUSSED_SYREEN_SHUTTLE: Self = Self(133);
    pub const END_DISCUSSED_SYREEN_SHUTTLE: Self = Self(133);
    pub const DISCUSSED_VUX_BEAST: Self = Self(134);
    pub const END_DISCUSSED_VUX_BEAST: Self = Self(134);
    pub const DISCUSSED_DESTRUCT_CODE: Self = Self(135);
    pub const END_DISCUSSED_DESTRUCT_CODE: Self = Self(135);
    pub const DISCUSSED_URQUAN_WARP: Self = Self(136);
    pub const END_DISCUSSED_URQUAN_WARP: Self = Self(136);
    pub const DISCUSSED_ARTIFACT_2: Self = Self(137);
    pub const END_DISCUSSED_ARTIFACT_2: Self = Self(137);
    pub const DISCUSSED_ARTIFACT_3: Self = Self(138);
    pub const END_DISCUSSED_ARTIFACT_3: Self = Self(138);
    pub const ATTACKED_DRUUGE: Self = Self(139);
    pub const END_ATTACKED_DRUUGE: Self = Self(139);
    pub const NEW_ALLIANCE_NAME: Self = Self(140);
    pub const END_NEW_ALLIANCE_NAME: Self = Self(141);
    pub const PORTAL_COUNTER: Self = Self(142);
    pub const END_PORTAL_COUNTER: Self = Self(145);
    pub const BURVIXESE_BROADCASTERS: Self = Self(146);
    pub const END_BURVIXESE_BROADCASTERS: Self = Self(146);
    pub const BURV_BROADCASTERS_ON_SHIP: Self = Self(147);
    pub const END_BURV_BROADCASTERS_ON_SHIP: Self = Self(147);
    pub const UTWIG_BOMB: Self = Self(148);
    pub const END_UTWIG_BOMB: Self = Self(148);
    pub const UTWIG_BOMB_ON_SHIP: Self = Self(149);
    pub const END_UTWIG_BOMB_ON_SHIP: Self = Self(149);
    pub const AQUA_HELIX: Self = Self(150);
    pub const END_AQUA_HELIX: Self = Self(150);
    pub const AQUA_HELIX_ON_SHIP: Self = Self(151);
    pub const END_AQUA_HELIX_ON_SHIP: Self = Self(151);
    pub const SUN_DEVICE: Self = Self(152);
    pub const END_SUN_DEVICE: Self = Self(152);
    pub const SUN_DEVICE_ON_SHIP: Self = Self(153);
    pub const END_SUN_DEVICE_ON_SHIP: Self = Self(153);
    pub const TAALO_PROTECTOR: Self = Self(154);
    pub const END_TAALO_PROTECTOR: Self = Self(154);
    pub const TAALO_PROTECTOR_ON_SHIP: Self = Self(155);
    pub const END_TAALO_PROTECTOR_ON_SHIP: Self = Self(155);
    pub const SHIP_VAULT_UNLOCKED: Self = Self(156);
    pub const END_SHIP_VAULT_UNLOCKED: Self = Self(156);
    pub const SYREEN_SHUTTLE: Self = Self(157);
    pub const END_SYREEN_SHUTTLE: Self = Self(157);
    pub const PORTAL_KEY: Self = Self(158);
    pub const END_PORTAL_KEY: Self = Self(158);
    pub const PORTAL_KEY_ON_SHIP: Self = Self(159);
    pub const END_PORTAL_KEY_ON_SHIP: Self = Self(159);
    pub const VUX_BEAST: Self = Self(160);
    pub const END_VUX_BEAST: Self = Self(160);
    pub const VUX_BEAST_ON_SHIP: Self = Self(161);
    pub const END_VUX_BEAST_ON_SHIP: Self = Self(161);
    pub const TALKING_PET: Self = Self(162);
    pub const END_TALKING_PET: Self = Self(162);
    pub const TALKING_PET_ON_SHIP: Self = Self(163);
    pub const END_TALKING_PET_ON_SHIP: Self = Self(163);
    pub const MOONBASE_ON_SHIP: Self = Self(164);
    pub const END_MOONBASE_ON_SHIP: Self = Self(164);
    pub const KOHR_AH_FRENZY: Self = Self(165);
    pub const END_KOHR_AH_FRENZY: Self = Self(165);
    pub const KOHR_AH_VISITS: Self = Self(166);
    pub const END_KOHR_AH_VISITS: Self = Self(167);
    pub const KOHR_AH_BYES: Self = Self(168);
    pub const END_KOHR_AH_BYES: Self = Self(168);
    pub const SLYLANDRO_HOME_VISITS: Self = Self(169);
    pub const END_SLYLANDRO_HOME_VISITS: Self = Self(171);
    pub const DESTRUCT_CODE_ON_SHIP: Self = Self(172);
    pub const END_DESTRUCT_CODE_ON_SHIP: Self = Self(172);
    pub const ILWRATH_VISITS: Self = Self(173);
    pub const END_ILWRATH_VISITS: Self = Self(175);
    pub const ILWRATH_DECEIVED: Self = Self(176);
    pub const END_ILWRATH_DECEIVED: Self = Self(176);
    pub const FLAGSHIP_CLOAKED: Self = Self(177);
    pub const END_FLAGSHIP_CLOAKED: Self = Self(177);
    pub const MYCON_VISITS: Self = Self(178);
    pub const END_MYCON_VISITS: Self = Self(180);
    pub const MYCON_HOME_VISITS: Self = Self(181);
    pub const END_MYCON_HOME_VISITS: Self = Self(183);
    pub const MYCON_AMBUSH: Self = Self(184);
    pub const END_MYCON_AMBUSH: Self = Self(184);
    pub const MYCON_FELL_FOR_AMBUSH: Self = Self(185);
    pub const END_MYCON_FELL_FOR_AMBUSH: Self = Self(185);
    pub const GLOBAL_FLAGS_AND_DATA: Self = Self(186);
    pub const END_GLOBAL_FLAGS_AND_DATA: Self = Self(193);
    pub const ORZ_VISITS: Self = Self(194);
    pub const END_ORZ_VISITS: Self = Self(196);
    pub const TAALO_VISITS: Self = Self(197);
    pub const END_TAALO_VISITS: Self = Self(199);
    pub const ORZ_MANNER: Self = Self(200);
    pub const END_ORZ_MANNER: Self = Self(201);
    pub const PROBE_EXHIBITED_BUG: Self = Self(202);
    pub const END_PROBE_EXHIBITED_BUG: Self = Self(202);
    pub const CLEAR_SPINDLE_ON_SHIP: Self = Self(203);
    pub const END_CLEAR_SPINDLE_ON_SHIP: Self = Self(203);
    pub const URQUAN_VISITS: Self = Self(204);
    pub const END_URQUAN_VISITS: Self = Self(206);
    pub const PLAYER_HYPNOTIZED: Self = Self(207);
    pub const END_PLAYER_HYPNOTIZED: Self = Self(207);
    pub const VUX_VISITS: Self = Self(208);
    pub const END_VUX_VISITS: Self = Self(210);
    pub const VUX_HOME_VISITS: Self = Self(211);
    pub const END_VUX_HOME_VISITS: Self = Self(213);
    pub const ZEX_VISITS: Self = Self(214);
    pub const END_ZEX_VISITS: Self = Self(216);
    pub const ZEX_IS_DEAD: Self = Self(217);
    pub const END_ZEX_IS_DEAD: Self = Self(217);
    pub const KNOW_ZEX_WANTS_MONSTER: Self = Self(218);
    pub const END_KNOW_ZEX_WANTS_MONSTER: Self = Self(218);
    pub const UTWIG_VISITS: Self = Self(219);
    pub const END_UTWIG_VISITS: Self = Self(221);
    pub const UTWIG_HOME_VISITS: Self = Self(222);
    pub const END_UTWIG_HOME_VISITS: Self = Self(224);
    pub const BOMB_VISITS: Self = Self(225);
    pub const END_BOMB_VISITS: Self = Self(227);
    pub const ULTRON_CONDITION: Self = Self(228);
    pub const END_ULTRON_CONDITION: Self = Self(230);
    pub const UTWIG_HAVE_ULTRON: Self = Self(231);
    pub const END_UTWIG_HAVE_ULTRON: Self = Self(231);
    pub const BOMB_UNPROTECTED: Self = Self(232);
    pub const END_BOMB_UNPROTECTED: Self = Self(232);
    pub const TAALO_UNPROTECTED: Self = Self(233);
    pub const END_TAALO_UNPROTECTED: Self = Self(233);
    pub const TALKING_PET_VISITS: Self = Self(234);
    pub const END_TALKING_PET_VISITS: Self = Self(236);
    pub const TALKING_PET_HOME_VISITS: Self = Self(237);
    pub const END_TALKING_PET_HOME_VISITS: Self = Self(239);
    pub const UMGAH_ZOMBIE_BLOBBIES: Self = Self(240);
    pub const END_UMGAH_ZOMBIE_BLOBBIES: Self = Self(240);
    pub const KNOW_UMGAH_ZOMBIES: Self = Self(241);
    pub const END_KNOW_UMGAH_ZOMBIES: Self = Self(241);
    pub const ARILOU_VISITS: Self = Self(242);
    pub const END_ARILOU_VISITS: Self = Self(244);
    pub const ARILOU_HOME_VISITS: Self = Self(245);
    pub const END_ARILOU_HOME_VISITS: Self = Self(247);
    pub const KNOW_ARILOU_WANT_WRECK: Self = Self(248);
    pub const END_KNOW_ARILOU_WANT_WRECK: Self = Self(248);
    pub const ARILOU_CHECKED_UMGAH: Self = Self(249);
    pub const END_ARILOU_CHECKED_UMGAH: Self = Self(250);
    pub const PORTAL_SPAWNER: Self = Self(251);
    pub const END_PORTAL_SPAWNER: Self = Self(251);
    pub const PORTAL_SPAWNER_ON_SHIP: Self = Self(252);
    pub const END_PORTAL_SPAWNER_ON_SHIP: Self = Self(252);
    pub const UMGAH_VISITS: Self = Self(253);
    pub const END_UMGAH_VISITS: Self = Self(255);
    pub const UMGAH_HOME_VISITS: Self = Self(256);
    pub const END_UMGAH_HOME_VISITS: Self = Self(258);
    pub const MET_NORMAL_UMGAH: Self = Self(259);
    pub const END_MET_NORMAL_UMGAH: Self = Self(259);
    pub const SYREEN_HOME_VISITS: Self = Self(260);
    pub const END_SYREEN_HOME_VISITS: Self = Self(262);
    pub const SYREEN_SHUTTLE_ON_SHIP: Self = Self(263);
    pub const END_SYREEN_SHUTTLE_ON_SHIP: Self = Self(263);
    pub const KNOW_SYREEN_VAULT: Self = Self(264);
    pub const END_KNOW_SYREEN_VAULT: Self = Self(264);
    pub const EGG_CASE0_ON_SHIP: Self = Self(265);
    pub const END_EGG_CASE0_ON_SHIP: Self = Self(265);
    pub const SUN_DEVICE_UNGUARDED: Self = Self(266);
    pub const END_SUN_DEVICE_UNGUARDED: Self = Self(266);
    pub const ROSY_SPHERE_ON_SHIP: Self = Self(267);
    pub const END_ROSY_SPHERE_ON_SHIP: Self = Self(267);
    pub const CHMMR_HOME_VISITS: Self = Self(268);
    pub const END_CHMMR_HOME_VISITS: Self = Self(270);
    pub const CHMMR_EMERGING: Self = Self(271);
    pub const END_CHMMR_EMERGING: Self = Self(271);
    pub const CHMMR_UNLEASHED: Self = Self(272);
    pub const END_CHMMR_UNLEASHED: Self = Self(272);
    pub const CHMMR_BOMB_STATE: Self = Self(273);
    pub const END_CHMMR_BOMB_STATE: Self = Self(274);
    pub const DRUUGE_DISCLAIMER: Self = Self(275);
    pub const END_DRUUGE_DISCLAIMER: Self = Self(275);
    pub const YEHAT_VISITS: Self = Self(276);
    pub const END_YEHAT_VISITS: Self = Self(278);
    pub const YEHAT_REBEL_VISITS: Self = Self(279);
    pub const END_YEHAT_REBEL_VISITS: Self = Self(281);
    pub const YEHAT_HOME_VISITS: Self = Self(282);
    pub const END_YEHAT_HOME_VISITS: Self = Self(284);
    pub const YEHAT_CIVIL_WAR: Self = Self(285);
    pub const END_YEHAT_CIVIL_WAR: Self = Self(285);
    pub const YEHAT_ABSORBED_PKUNK: Self = Self(286);
    pub const END_YEHAT_ABSORBED_PKUNK: Self = Self(286);
    pub const YEHAT_SHIP_MONTH: Self = Self(287);
    pub const END_YEHAT_SHIP_MONTH: Self = Self(290);
    pub const YEHAT_SHIP_DAY: Self = Self(291);
    pub const END_YEHAT_SHIP_DAY: Self = Self(295);
    pub const YEHAT_SHIP_YEAR: Self = Self(296);
    pub const END_YEHAT_SHIP_YEAR: Self = Self(300);
    pub const CLEAR_SPINDLE: Self = Self(301);
    pub const END_CLEAR_SPINDLE: Self = Self(301);
    pub const PKUNK_VISITS: Self = Self(302);
    pub const END_PKUNK_VISITS: Self = Self(304);
    pub const PKUNK_HOME_VISITS: Self = Self(305);
    pub const END_PKUNK_HOME_VISITS: Self = Self(307);
    pub const PKUNK_SHIP_MONTH: Self = Self(308);
    pub const END_PKUNK_SHIP_MONTH: Self = Self(311);
    pub const PKUNK_SHIP_DAY: Self = Self(312);
    pub const END_PKUNK_SHIP_DAY: Self = Self(316);
    pub const PKUNK_SHIP_YEAR: Self = Self(317);
    pub const END_PKUNK_SHIP_YEAR: Self = Self(321);
    pub const PKUNK_MISSION: Self = Self(322);
    pub const END_PKUNK_MISSION: Self = Self(324);
    pub const SUPOX_VISITS: Self = Self(325);
    pub const END_SUPOX_VISITS: Self = Self(327);
    pub const SUPOX_HOME_VISITS: Self = Self(328);
    pub const END_SUPOX_HOME_VISITS: Self = Self(330);
    pub const THRADD_VISITS: Self = Self(331);
    pub const END_THRADD_VISITS: Self = Self(333);
    pub const THRADD_HOME_VISITS: Self = Self(334);
    pub const END_THRADD_HOME_VISITS: Self = Self(336);
    pub const HELIX_VISITS: Self = Self(337);
    pub const END_HELIX_VISITS: Self = Self(339);
    pub const HELIX_UNPROTECTED: Self = Self(340);
    pub const END_HELIX_UNPROTECTED: Self = Self(340);
    pub const THRADD_CULTURE: Self = Self(341);
    pub const END_THRADD_CULTURE: Self = Self(342);
    pub const THRADD_MISSION: Self = Self(343);
    pub const END_THRADD_MISSION: Self = Self(345);
    pub const DRUUGE_VISITS: Self = Self(346);
    pub const END_DRUUGE_VISITS: Self = Self(348);
    pub const DRUUGE_HOME_VISITS: Self = Self(349);
    pub const END_DRUUGE_HOME_VISITS: Self = Self(351);
    pub const ROSY_SPHERE: Self = Self(352);
    pub const END_ROSY_SPHERE: Self = Self(352);
    pub const SCANNED_MAIDENS: Self = Self(353);
    pub const END_SCANNED_MAIDENS: Self = Self(353);
    pub const SCANNED_FRAGMENTS: Self = Self(354);
    pub const END_SCANNED_FRAGMENTS: Self = Self(354);
    pub const SCANNED_CASTER: Self = Self(355);
    pub const END_SCANNED_CASTER: Self = Self(355);
    pub const SCANNED_SPAWNER: Self = Self(356);
    pub const END_SCANNED_SPAWNER: Self = Self(356);
    pub const SCANNED_ULTRON: Self = Self(357);
    pub const END_SCANNED_ULTRON: Self = Self(357);
    pub const ZOQFOT_INFO: Self = Self(358);
    pub const END_ZOQFOT_INFO: Self = Self(359);
    pub const ZOQFOT_HOSTILE: Self = Self(360);
    pub const END_ZOQFOT_HOSTILE: Self = Self(360);
    pub const ZOQFOT_HOME_VISITS: Self = Self(361);
    pub const END_ZOQFOT_HOME_VISITS: Self = Self(363);
    pub const MET_ZOQFOT: Self = Self(364);
    pub const END_MET_ZOQFOT: Self = Self(364);
    pub const ZOQFOT_DISTRESS: Self = Self(365);
    pub const END_ZOQFOT_DISTRESS: Self = Self(366);
    pub const EGG_CASE1_ON_SHIP: Self = Self(367);
    pub const END_EGG_CASE1_ON_SHIP: Self = Self(367);
    pub const EGG_CASE2_ON_SHIP: Self = Self(368);
    pub const END_EGG_CASE2_ON_SHIP: Self = Self(368);
    pub const MYCON_SUN_VISITS: Self = Self(369);
    pub const END_MYCON_SUN_VISITS: Self = Self(371);
    pub const ORZ_HOME_VISITS: Self = Self(372);
    pub const END_ORZ_HOME_VISITS: Self = Self(374);
    pub const MELNORME_FUEL_PROCEDURE: Self = Self(375);
    pub const END_MELNORME_FUEL_PROCEDURE: Self = Self(375);
    pub const MELNORME_TECH_PROCEDURE: Self = Self(376);
    pub const END_MELNORME_TECH_PROCEDURE: Self = Self(376);
    pub const MELNORME_INFO_PROCEDURE: Self = Self(377);
    pub const END_MELNORME_INFO_PROCEDURE: Self = Self(377);
    pub const MELNORME_TECH_STACK: Self = Self(378);
    pub const END_MELNORME_TECH_STACK: Self = Self(381);
    pub const MELNORME_EVENTS_INFO_STACK: Self = Self(382);
    pub const END_MELNORME_EVENTS_INFO_STACK: Self = Self(386);
    pub const MELNORME_ALIEN_INFO_STACK: Self = Self(387);
    pub const END_MELNORME_ALIEN_INFO_STACK: Self = Self(391);
    pub const MELNORME_HISTORY_INFO_STACK: Self = Self(392);
    pub const END_MELNORME_HISTORY_INFO_STACK: Self = Self(396);
    pub const RAINBOW_WORLD0: Self = Self(397);
    pub const END_RAINBOW_WORLD0: Self = Self(404);
    pub const RAINBOW_WORLD1: Self = Self(405);
    pub const END_RAINBOW_WORLD1: Self = Self(406);
    pub const MELNORME_RAINBOW_COUNT: Self = Self(407);
    pub const END_MELNORME_RAINBOW_COUNT: Self = Self(410);
    pub const USED_BROADCASTER: Self = Self(411);
    pub const END_USED_BROADCASTER: Self = Self(411);
    pub const BROADCASTER_RESPONSE: Self = Self(412);
    pub const END_BROADCASTER_RESPONSE: Self = Self(412);
    pub const IMPROVED_LANDER_SPEED: Self = Self(413);
    pub const END_IMPROVED_LANDER_SPEED: Self = Self(413);
    pub const IMPROVED_LANDER_CARGO: Self = Self(414);
    pub const END_IMPROVED_LANDER_CARGO: Self = Self(414);
    pub const IMPROVED_LANDER_SHOT: Self = Self(415);
    pub const END_IMPROVED_LANDER_SHOT: Self = Self(415);
    pub const MET_ORZ_BEFORE: Self = Self(416);
    pub const END_MET_ORZ_BEFORE: Self = Self(416);
    pub const YEHAT_REBEL_TOLD_PKUNK: Self = Self(417);
    pub const END_YEHAT_REBEL_TOLD_PKUNK: Self = Self(417);
    pub const PLAYER_HAD_SEX: Self = Self(418);
    pub const END_PLAYER_HAD_SEX: Self = Self(418);
    pub const UMGAH_BROADCASTERS_ON_SHIP: Self = Self(419);
    pub const END_UMGAH_BROADCASTERS_ON_SHIP: Self = Self(419);
    pub const LIGHT_MINERAL_LOAD: Self = Self(420);
    pub const END_LIGHT_MINERAL_LOAD: Self = Self(422);
    pub const MEDIUM_MINERAL_LOAD: Self = Self(423);
    pub const END_MEDIUM_MINERAL_LOAD: Self = Self(425);
    pub const HEAVY_MINERAL_LOAD: Self = Self(426);
    pub const END_HEAVY_MINERAL_LOAD: Self = Self(428);
    pub const STARBASE_BULLETS0: Self = Self(429);
    pub const END_STARBASE_BULLETS0: Self = Self(436);
    pub const STARBASE_BULLETS1: Self = Self(437);
    pub const END_STARBASE_BULLETS1: Self = Self(444);
    pub const STARBASE_BULLETS2: Self = Self(445);
    pub const END_STARBASE_BULLETS2: Self = Self(452);
    pub const STARBASE_BULLETS3: Self = Self(453);
    pub const END_STARBASE_BULLETS3: Self = Self(460);
    pub const STARBASE_MONTH: Self = Self(461);
    pub const END_STARBASE_MONTH: Self = Self(464);
    pub const STARBASE_DAY: Self = Self(465);
    pub const END_STARBASE_DAY: Self = Self(469);
    pub const CREW_SOLD_TO_DRUUGE0: Self = Self(470);
    pub const END_CREW_SOLD_TO_DRUUGE0: Self = Self(477);
    pub const CREW_PURCHASED0: Self = Self(478);
    pub const END_CREW_PURCHASED0: Self = Self(485);
    pub const CREW_PURCHASED1: Self = Self(486);
    pub const END_CREW_PURCHASED1: Self = Self(493);
    pub const URQUAN_PROTECTING_SAMATRA: Self = Self(494);
    pub const END_URQUAN_PROTECTING_SAMATRA: Self = Self(494);
    pub const THRADDASH_BODY_COUNT: Self = Self(495);
    pub const END_THRADDASH_BODY_COUNT: Self = Self(499);
    pub const UTWIG_SUPOX_MISSION: Self = Self(500);
    pub const END_UTWIG_SUPOX_MISSION: Self = Self(502);
    pub const SPATHI_INFO: Self = Self(503);
    pub const END_SPATHI_INFO: Self = Self(505);
    pub const ILWRATH_INFO: Self = Self(506);
    pub const END_ILWRATH_INFO: Self = Self(507);
    pub const ILWRATH_GODS_SPOKEN: Self = Self(508);
    pub const END_ILWRATH_GODS_SPOKEN: Self = Self(511);
    pub const ILWRATH_WORSHIP: Self = Self(512);
    pub const END_ILWRATH_WORSHIP: Self = Self(513);
    pub const ILWRATH_FIGHT_THRADDASH: Self = Self(514);
    pub const END_ILWRATH_FIGHT_THRADDASH: Self = Self(514);
    pub const READY_TO_CONFUSE_URQUAN: Self = Self(515);
    pub const END_READY_TO_CONFUSE_URQUAN: Self = Self(515);
    pub const URQUAN_HYPNO_VISITS: Self = Self(516);
    pub const END_URQUAN_HYPNO_VISITS: Self = Self(516);
    pub const MENTIONED_PET_COMPULSION: Self = Self(517);
    pub const END_MENTIONED_PET_COMPULSION: Self = Self(517);
    pub const URQUAN_INFO: Self = Self(518);
    pub const END_URQUAN_INFO: Self = Self(519);
    pub const KNOW_URQUAN_STORY: Self = Self(520);
    pub const END_KNOW_URQUAN_STORY: Self = Self(521);
    pub const MYCON_INFO: Self = Self(522);
    pub const END_MYCON_INFO: Self = Self(525);
    pub const MYCON_RAMBLE: Self = Self(526);
    pub const END_MYCON_RAMBLE: Self = Self(530);
    pub const KNOW_ABOUT_SHATTERED: Self = Self(531);
    pub const END_KNOW_ABOUT_SHATTERED: Self = Self(532);
    pub const MYCON_INSULTS: Self = Self(533);
    pub const END_MYCON_INSULTS: Self = Self(535);
    pub const MYCON_KNOW_AMBUSH: Self = Self(536);
    pub const END_MYCON_KNOW_AMBUSH: Self = Self(536);
    pub const SYREEN_INFO: Self = Self(537);
    pub const END_SYREEN_INFO: Self = Self(538);
    pub const KNOW_SYREEN_WORLD_SHATTERED: Self = Self(539);
    pub const END_KNOW_SYREEN_WORLD_SHATTERED: Self = Self(539);
    pub const SYREEN_KNOW_ABOUT_MYCON: Self = Self(540);
    pub const END_SYREEN_KNOW_ABOUT_MYCON: Self = Self(540);
    pub const TALKING_PET_INFO: Self = Self(541);
    pub const END_TALKING_PET_INFO: Self = Self(543);
    pub const TALKING_PET_SUGGESTIONS: Self = Self(544);
    pub const END_TALKING_PET_SUGGESTIONS: Self = Self(546);
    pub const LEARNED_TALKING_PET: Self = Self(547);
    pub const END_LEARNED_TALKING_PET: Self = Self(547);
    pub const DNYARRI_LIED: Self = Self(548);
    pub const END_DNYARRI_LIED: Self = Self(548);
    pub const SHIP_TO_COMPEL: Self = Self(549);
    pub const END_SHIP_TO_COMPEL: Self = Self(549);
    pub const ORZ_GENERAL_INFO: Self = Self(550);
    pub const END_ORZ_GENERAL_INFO: Self = Self(551);
    pub const ORZ_PERSONAL_INFO: Self = Self(552);
    pub const END_ORZ_PERSONAL_INFO: Self = Self(554);
    pub const ORZ_ANDRO_STATE: Self = Self(555);
    pub const END_ORZ_ANDRO_STATE: Self = Self(556);
    pub const REFUSED_ORZ_ALLIANCE: Self = Self(557);
    pub const END_REFUSED_ORZ_ALLIANCE: Self = Self(557);
    pub const PKUNK_MANNER: Self = Self(558);
    pub const END_PKUNK_MANNER: Self = Self(559);
    pub const PKUNK_ON_THE_MOVE: Self = Self(560);
    pub const END_PKUNK_ON_THE_MOVE: Self = Self(560);
    pub const PKUNK_FLEET: Self = Self(561);
    pub const END_PKUNK_FLEET: Self = Self(562);
    pub const PKUNK_MIGRATE: Self = Self(563);
    pub const END_PKUNK_MIGRATE: Self = Self(564);
    pub const PKUNK_RETURN: Self = Self(565);
    pub const END_PKUNK_RETURN: Self = Self(565);
    pub const PKUNK_WORRY: Self = Self(566);
    pub const END_PKUNK_WORRY: Self = Self(567);
    pub const PKUNK_INFO: Self = Self(568);
    pub const END_PKUNK_INFO: Self = Self(570);
    pub const PKUNK_WAR: Self = Self(571);
    pub const END_PKUNK_WAR: Self = Self(572);
    pub const PKUNK_FORTUNE: Self = Self(573);
    pub const END_PKUNK_FORTUNE: Self = Self(575);
    pub const PKUNK_MIGRATE_VISITS: Self = Self(576);
    pub const END_PKUNK_MIGRATE_VISITS: Self = Self(578);
    pub const PKUNK_REASONS: Self = Self(579);
    pub const END_PKUNK_REASONS: Self = Self(582);
    pub const PKUNK_SWITCH: Self = Self(583);
    pub const END_PKUNK_SWITCH: Self = Self(583);
    pub const PKUNK_SENSE_VICTOR: Self = Self(584);
    pub const END_PKUNK_SENSE_VICTOR: Self = Self(584);
    pub const KOHR_AH_REASONS: Self = Self(585);
    pub const END_KOHR_AH_REASONS: Self = Self(586);
    pub const KOHR_AH_PLEAD: Self = Self(587);
    pub const END_KOHR_AH_PLEAD: Self = Self(588);
    pub const KOHR_AH_INFO: Self = Self(589);
    pub const END_KOHR_AH_INFO: Self = Self(590);
    pub const KNOW_KOHR_AH_STORY: Self = Self(591);
    pub const END_KNOW_KOHR_AH_STORY: Self = Self(592);
    pub const KOHR_AH_SENSES_EVIL: Self = Self(593);
    pub const END_KOHR_AH_SENSES_EVIL: Self = Self(593);
    pub const URQUAN_SENSES_EVIL: Self = Self(594);
    pub const END_URQUAN_SENSES_EVIL: Self = Self(594);
    pub const SLYLANDRO_PROBE_VISITS: Self = Self(595);
    pub const END_SLYLANDRO_PROBE_VISITS: Self = Self(597);
    pub const SLYLANDRO_PROBE_THREAT: Self = Self(598);
    pub const END_SLYLANDRO_PROBE_THREAT: Self = Self(599);
    pub const SLYLANDRO_PROBE_WRONG: Self = Self(600);
    pub const END_SLYLANDRO_PROBE_WRONG: Self = Self(601);
    pub const SLYLANDRO_PROBE_ID: Self = Self(602);
    pub const END_SLYLANDRO_PROBE_ID: Self = Self(603);
    pub const SLYLANDRO_PROBE_INFO: Self = Self(604);
    pub const END_SLYLANDRO_PROBE_INFO: Self = Self(605);
    pub const SLYLANDRO_PROBE_EXIT: Self = Self(606);
    pub const END_SLYLANDRO_PROBE_EXIT: Self = Self(607);
    pub const UMGAH_HOSTILE: Self = Self(608);
    pub const END_UMGAH_HOSTILE: Self = Self(608);
    pub const UMGAH_EVIL_BLOBBIES: Self = Self(609);
    pub const END_UMGAH_EVIL_BLOBBIES: Self = Self(609);
    pub const UMGAH_MENTIONED_TRICKS: Self = Self(610);
    pub const END_UMGAH_MENTIONED_TRICKS: Self = Self(611);
    pub const BOMB_CARRIER: Self = Self(612);
    pub const END_BOMB_CARRIER: Self = Self(612);
    pub const THRADD_MANNER: Self = Self(613);
    pub const END_THRADD_MANNER: Self = Self(613);
    pub const THRADD_INTRO: Self = Self(614);
    pub const END_THRADD_INTRO: Self = Self(615);
    pub const THRADD_DEMEANOR: Self = Self(616);
    pub const END_THRADD_DEMEANOR: Self = Self(618);
    pub const THRADD_INFO: Self = Self(619);
    pub const END_THRADD_INFO: Self = Self(620);
    pub const THRADD_BODY_LEVEL: Self = Self(621);
    pub const END_THRADD_BODY_LEVEL: Self = Self(622);
    pub const THRADD_MISSION_VISITS: Self = Self(623);
    pub const END_THRADD_MISSION_VISITS: Self = Self(623);
    pub const THRADD_STACK_1: Self = Self(624);
    pub const END_THRADD_STACK_1: Self = Self(626);
    pub const THRADD_HOSTILE_STACK_2: Self = Self(627);
    pub const END_THRADD_HOSTILE_STACK_2: Self = Self(627);
    pub const THRADD_HOSTILE_STACK_3: Self = Self(628);
    pub const END_THRADD_HOSTILE_STACK_3: Self = Self(628);
    pub const THRADD_HOSTILE_STACK_4: Self = Self(629);
    pub const END_THRADD_HOSTILE_STACK_4: Self = Self(629);
    pub const THRADD_HOSTILE_STACK_5: Self = Self(630);
    pub const END_THRADD_HOSTILE_STACK_5: Self = Self(630);
    pub const CHMMR_STACK: Self = Self(631);
    pub const END_CHMMR_STACK: Self = Self(632);
    pub const ARILOU_MANNER: Self = Self(633);
    pub const END_ARILOU_MANNER: Self = Self(634);
    pub const NO_PORTAL_VISITS: Self = Self(635);
    pub const END_NO_PORTAL_VISITS: Self = Self(635);
    pub const ARILOU_STACK_1: Self = Self(636);
    pub const END_ARILOU_STACK_1: Self = Self(637);
    pub const ARILOU_STACK_2: Self = Self(638);
    pub const END_ARILOU_STACK_2: Self = Self(638);
    pub const ARILOU_STACK_3: Self = Self(639);
    pub const END_ARILOU_STACK_3: Self = Self(640);
    pub const ARILOU_STACK_4: Self = Self(641);
    pub const END_ARILOU_STACK_4: Self = Self(641);
    pub const ARILOU_STACK_5: Self = Self(642);
    pub const END_ARILOU_STACK_5: Self = Self(643);
    pub const ARILOU_INFO: Self = Self(644);
    pub const END_ARILOU_INFO: Self = Self(645);
    pub const ARILOU_HINTS: Self = Self(646);
    pub const END_ARILOU_HINTS: Self = Self(647);
    pub const DRUUGE_MANNER: Self = Self(648);
    pub const END_DRUUGE_MANNER: Self = Self(648);
    pub const DRUUGE_SPACE_INFO: Self = Self(649);
    pub const END_DRUUGE_SPACE_INFO: Self = Self(650);
    pub const DRUUGE_HOME_INFO: Self = Self(651);
    pub const END_DRUUGE_HOME_INFO: Self = Self(652);
    pub const DRUUGE_SALVAGE: Self = Self(653);
    pub const END_DRUUGE_SALVAGE: Self = Self(653);
    pub const KNOW_DRUUGE_SLAVERS: Self = Self(654);
    pub const END_KNOW_DRUUGE_SLAVERS: Self = Self(655);
    pub const FRAGMENTS_BOUGHT: Self = Self(656);
    pub const END_FRAGMENTS_BOUGHT: Self = Self(657);
    pub const ZEX_STACK_1: Self = Self(658);
    pub const END_ZEX_STACK_1: Self = Self(659);
    pub const ZEX_STACK_2: Self = Self(660);
    pub const END_ZEX_STACK_2: Self = Self(661);
    pub const ZEX_STACK_3: Self = Self(662);
    pub const END_ZEX_STACK_3: Self = Self(663);
    pub const VUX_INFO: Self = Self(664);
    pub const END_VUX_INFO: Self = Self(665);
    pub const VUX_STACK_1: Self = Self(666);
    pub const END_VUX_STACK_1: Self = Self(669);
    pub const VUX_STACK_2: Self = Self(670);
    pub const END_VUX_STACK_2: Self = Self(671);
    pub const VUX_STACK_3: Self = Self(672);
    pub const END_VUX_STACK_3: Self = Self(673);
    pub const VUX_STACK_4: Self = Self(674);
    pub const END_VUX_STACK_4: Self = Self(675);
    pub const SHOFIXTI_STACK4: Self = Self(676);
    pub const END_SHOFIXTI_STACK4: Self = Self(677);
    pub const YEHAT_REBEL_INFO: Self = Self(678);
    pub const END_YEHAT_REBEL_INFO: Self = Self(680);
    pub const YEHAT_ROYALIST_INFO: Self = Self(681);
    pub const END_YEHAT_ROYALIST_INFO: Self = Self(681);
    pub const YEHAT_ROYALIST_TOLD_PKUNK: Self = Self(682);
    pub const END_YEHAT_ROYALIST_TOLD_PKUNK: Self = Self(682);
    pub const NO_YEHAT_ALLY_HOME: Self = Self(683);
    pub const END_NO_YEHAT_ALLY_HOME: Self = Self(683);
    pub const NO_YEHAT_HELP_HOME: Self = Self(684);
    pub const END_NO_YEHAT_HELP_HOME: Self = Self(684);
    pub const NO_YEHAT_INFO: Self = Self(685);
    pub const END_NO_YEHAT_INFO: Self = Self(685);
    pub const NO_YEHAT_ALLY_SPACE: Self = Self(686);
    pub const END_NO_YEHAT_ALLY_SPACE: Self = Self(687);
    pub const NO_YEHAT_HELP_SPACE: Self = Self(688);
    pub const END_NO_YEHAT_HELP_SPACE: Self = Self(689);
    pub const ZOQFOT_KNOW_MASK: Self = Self(690);
    pub const END_ZOQFOT_KNOW_MASK: Self = Self(693);
    pub const SUPOX_HOSTILE: Self = Self(694);
    pub const END_SUPOX_HOSTILE: Self = Self(694);
    pub const SUPOX_INFO: Self = Self(695);
    pub const END_SUPOX_INFO: Self = Self(695);
    pub const SUPOX_WAR_NEWS: Self = Self(696);
    pub const END_SUPOX_WAR_NEWS: Self = Self(697);
    pub const SUPOX_ULTRON_HELP: Self = Self(698);
    pub const END_SUPOX_ULTRON_HELP: Self = Self(698);
    pub const SUPOX_STACK1: Self = Self(699);
    pub const END_SUPOX_STACK1: Self = Self(701);
    pub const SUPOX_STACK2: Self = Self(702);
    pub const END_SUPOX_STACK2: Self = Self(703);
    pub const UTWIG_HOSTILE: Self = Self(704);
    pub const END_UTWIG_HOSTILE: Self = Self(704);
    pub const UTWIG_INFO: Self = Self(705);
    pub const END_UTWIG_INFO: Self = Self(705);
    pub const UTWIG_WAR_NEWS: Self = Self(706);
    pub const END_UTWIG_WAR_NEWS: Self = Self(707);
    pub const UTWIG_STACK1: Self = Self(708);
    pub const END_UTWIG_STACK1: Self = Self(710);
    pub const UTWIG_STACK2: Self = Self(711);
    pub const END_UTWIG_STACK2: Self = Self(712);
    pub const BOMB_INFO: Self = Self(713);
    pub const END_BOMB_INFO: Self = Self(713);
    pub const BOMB_STACK1: Self = Self(714);
    pub const END_BOMB_STACK1: Self = Self(715);
    pub const BOMB_STACK2: Self = Self(716);
    pub const END_BOMB_STACK2: Self = Self(717);
    pub const SLYLANDRO_KNOW_BROKEN: Self = Self(718);
    pub const END_SLYLANDRO_KNOW_BROKEN: Self = Self(718);
    pub const PLAYER_KNOWS_PROBE: Self = Self(719);
    pub const END_PLAYER_KNOWS_PROBE: Self = Self(719);
    pub const PLAYER_KNOWS_PROGRAM: Self = Self(720);
    pub const END_PLAYER_KNOWS_PROGRAM: Self = Self(720);
    pub const PLAYER_KNOWS_EFFECTS: Self = Self(721);
    pub const END_PLAYER_KNOWS_EFFECTS: Self = Self(721);
    pub const PLAYER_KNOWS_PRIORITY: Self = Self(722);
    pub const END_PLAYER_KNOWS_PRIORITY: Self = Self(722);
    pub const SLYLANDRO_STACK1: Self = Self(723);
    pub const END_SLYLANDRO_STACK1: Self = Self(725);
    pub const SLYLANDRO_STACK2: Self = Self(726);
    pub const END_SLYLANDRO_STACK2: Self = Self(726);
    pub const SLYLANDRO_STACK3: Self = Self(727);
    pub const END_SLYLANDRO_STACK3: Self = Self(728);
    pub const SLYLANDRO_STACK4: Self = Self(729);
    pub const END_SLYLANDRO_STACK4: Self = Self(730);
    pub const SLYLANDRO_STACK5: Self = Self(731);
    pub const END_SLYLANDRO_STACK5: Self = Self(731);
    pub const SLYLANDRO_STACK6: Self = Self(732);
    pub const END_SLYLANDRO_STACK6: Self = Self(732);
    pub const SLYLANDRO_STACK7: Self = Self(733);
    pub const END_SLYLANDRO_STACK7: Self = Self(734);
    pub const SLYLANDRO_STACK8: Self = Self(735);
    pub const END_SLYLANDRO_STACK8: Self = Self(736);
    pub const SLYLANDRO_STACK9: Self = Self(737);
    pub const END_SLYLANDRO_STACK9: Self = Self(738);
    pub const SLYLANDRO_KNOW_EARTH: Self = Self(739);
    pub const END_SLYLANDRO_KNOW_EARTH: Self = Self(739);
    pub const SLYLANDRO_KNOW_EXPLORE: Self = Self(740);
    pub const END_SLYLANDRO_KNOW_EXPLORE: Self = Self(740);
    pub const SLYLANDRO_KNOW_GATHER: Self = Self(741);
    pub const END_SLYLANDRO_KNOW_GATHER: Self = Self(741);
    pub const SLYLANDRO_KNOW_URQUAN: Self = Self(742);
    pub const END_SLYLANDRO_KNOW_URQUAN: Self = Self(743);
    pub const RECALL_VISITS: Self = Self(744);
    pub const END_RECALL_VISITS: Self = Self(745);
    pub const SLYLANDRO_MULTIPLIER: Self = Self(746);
    pub const END_SLYLANDRO_MULTIPLIER: Self = Self(748);
    pub const KNOW_SPATHI_QUEST: Self = Self(749);
    pub const END_KNOW_SPATHI_QUEST: Self = Self(749);
    pub const KNOW_SPATHI_EVIL: Self = Self(750);
    pub const END_KNOW_SPATHI_EVIL: Self = Self(750);
    pub const BATTLE_PLANET: Self = Self(751);
    pub const END_BATTLE_PLANET: Self = Self(758);
    pub const ESCAPE_COUNTER: Self = Self(759);
    pub const END_ESCAPE_COUNTER: Self = Self(766);
    pub const CREW_SOLD_TO_DRUUGE1: Self = Self(767);
    pub const END_CREW_SOLD_TO_DRUUGE1: Self = Self(774);
    pub const PKUNK_DONE_WAR: Self = Self(775);
    pub const END_PKUNK_DONE_WAR: Self = Self(775);
    pub const SYREEN_STACK0: Self = Self(776);
    pub const END_SYREEN_STACK0: Self = Self(777);
    pub const SYREEN_STACK1: Self = Self(778);
    pub const END_SYREEN_STACK1: Self = Self(779);
    pub const SYREEN_STACK2: Self = Self(780);
    pub const END_SYREEN_STACK2: Self = Self(781);
    pub const REFUSED_ULTRON_AT_BOMB: Self = Self(782);
    pub const END_REFUSED_ULTRON_AT_BOMB: Self = Self(782);
    pub const NO_TRICK_AT_SUN: Self = Self(783);
    pub const END_NO_TRICK_AT_SUN: Self = Self(783);
    pub const SPATHI_STACK0: Self = Self(784);
    pub const END_SPATHI_STACK0: Self = Self(785);
    pub const SPATHI_STACK1: Self = Self(786);
    pub const END_SPATHI_STACK1: Self = Self(786);
    pub const SPATHI_STACK2: Self = Self(787);
    pub const END_SPATHI_STACK2: Self = Self(787);
    pub const ORZ_STACK0: Self = Self(788);
    pub const END_ORZ_STACK0: Self = Self(788);
    pub const ORZ_STACK1: Self = Self(789);
    pub const END_ORZ_STACK1: Self = Self(789);
    pub const SHOFIXTI_GRPOFFS0: Self = Self(790);
    pub const END_SHOFIXTI_GRPOFFS0: Self = Self(797);
    pub const SHOFIXTI_GRPOFFS1: Self = Self(798);
    pub const END_SHOFIXTI_GRPOFFS1: Self = Self(805);
    pub const SHOFIXTI_GRPOFFS2: Self = Self(806);
    pub const END_SHOFIXTI_GRPOFFS2: Self = Self(813);
    pub const SHOFIXTI_GRPOFFS3: Self = Self(814);
    pub const END_SHOFIXTI_GRPOFFS3: Self = Self(821);
    pub const ZOQFOT_GRPOFFS0: Self = Self(822);
    pub const END_ZOQFOT_GRPOFFS0: Self = Self(829);
    pub const ZOQFOT_GRPOFFS1: Self = Self(830);
    pub const END_ZOQFOT_GRPOFFS1: Self = Self(837);
    pub const ZOQFOT_GRPOFFS2: Self = Self(838);
    pub const END_ZOQFOT_GRPOFFS2: Self = Self(845);
    pub const ZOQFOT_GRPOFFS3: Self = Self(846);
    pub const END_ZOQFOT_GRPOFFS3: Self = Self(853);
    pub const MELNORME0_GRPOFFS0: Self = Self(854);
    pub const END_MELNORME0_GRPOFFS0: Self = Self(861);
    pub const MELNORME0_GRPOFFS1: Self = Self(862);
    pub const END_MELNORME0_GRPOFFS1: Self = Self(869);
    pub const MELNORME0_GRPOFFS2: Self = Self(870);
    pub const END_MELNORME0_GRPOFFS2: Self = Self(877);
    pub const MELNORME0_GRPOFFS3: Self = Self(878);
    pub const END_MELNORME0_GRPOFFS3: Self = Self(885);
    pub const MELNORME1_GRPOFFS0: Self = Self(886);
    pub const END_MELNORME1_GRPOFFS0: Self = Self(893);
    pub const MELNORME1_GRPOFFS1: Self = Self(894);
    pub const END_MELNORME1_GRPOFFS1: Self = Self(901);
    pub const MELNORME1_GRPOFFS2: Self = Self(902);
    pub const END_MELNORME1_GRPOFFS2: Self = Self(909);
    pub const MELNORME1_GRPOFFS3: Self = Self(910);
    pub const END_MELNORME1_GRPOFFS3: Self = Self(917);
    pub const MELNORME2_GRPOFFS0: Self = Self(918);
    pub const END_MELNORME2_GRPOFFS0: Self = Self(925);
    pub const MELNORME2_GRPOFFS1: Self = Self(926);
    pub const END_MELNORME2_GRPOFFS1: Self = Self(933);
    pub const MELNORME2_GRPOFFS2: Self = Self(934);
    pub const END_MELNORME2_GRPOFFS2: Self = Self(941);
    pub const MELNORME2_GRPOFFS3: Self = Self(942);
    pub const END_MELNORME2_GRPOFFS3: Self = Self(949);
    pub const MELNORME3_GRPOFFS0: Self = Self(950);
    pub const END_MELNORME3_GRPOFFS0: Self = Self(957);
    pub const MELNORME3_GRPOFFS1: Self = Self(958);
    pub const END_MELNORME3_GRPOFFS1: Self = Self(965);
    pub const MELNORME3_GRPOFFS2: Self = Self(966);
    pub const END_MELNORME3_GRPOFFS2: Self = Self(973);
    pub const MELNORME3_GRPOFFS3: Self = Self(974);
    pub const END_MELNORME3_GRPOFFS3: Self = Self(981);
    pub const MELNORME4_GRPOFFS0: Self = Self(982);
    pub const END_MELNORME4_GRPOFFS0: Self = Self(989);
    pub const MELNORME4_GRPOFFS1: Self = Self(990);
    pub const END_MELNORME4_GRPOFFS1: Self = Self(997);
    pub const MELNORME4_GRPOFFS2: Self = Self(998);
    pub const END_MELNORME4_GRPOFFS2: Self = Self(1005);
    pub const MELNORME4_GRPOFFS3: Self = Self(1006);
    pub const END_MELNORME4_GRPOFFS3: Self = Self(1013);
    pub const MELNORME5_GRPOFFS0: Self = Self(1014);
    pub const END_MELNORME5_GRPOFFS0: Self = Self(1021);
    pub const MELNORME5_GRPOFFS1: Self = Self(1022);
    pub const END_MELNORME5_GRPOFFS1: Self = Self(1029);
    pub const MELNORME5_GRPOFFS2: Self = Self(1030);
    pub const END_MELNORME5_GRPOFFS2: Self = Self(1037);
    pub const MELNORME5_GRPOFFS3: Self = Self(1038);
    pub const END_MELNORME5_GRPOFFS3: Self = Self(1045);
    pub const MELNORME6_GRPOFFS0: Self = Self(1046);
    pub const END_MELNORME6_GRPOFFS0: Self = Self(1053);
    pub const MELNORME6_GRPOFFS1: Self = Self(1054);
    pub const END_MELNORME6_GRPOFFS1: Self = Self(1061);
    pub const MELNORME6_GRPOFFS2: Self = Self(1062);
    pub const END_MELNORME6_GRPOFFS2: Self = Self(1069);
    pub const MELNORME6_GRPOFFS3: Self = Self(1070);
    pub const END_MELNORME6_GRPOFFS3: Self = Self(1077);
    pub const MELNORME7_GRPOFFS0: Self = Self(1078);
    pub const END_MELNORME7_GRPOFFS0: Self = Self(1085);
    pub const MELNORME7_GRPOFFS1: Self = Self(1086);
    pub const END_MELNORME7_GRPOFFS1: Self = Self(1093);
    pub const MELNORME7_GRPOFFS2: Self = Self(1094);
    pub const END_MELNORME7_GRPOFFS2: Self = Self(1101);
    pub const MELNORME7_GRPOFFS3: Self = Self(1102);
    pub const END_MELNORME7_GRPOFFS3: Self = Self(1109);
    pub const MELNORME8_GRPOFFS0: Self = Self(1110);
    pub const END_MELNORME8_GRPOFFS0: Self = Self(1117);
    pub const MELNORME8_GRPOFFS1: Self = Self(1118);
    pub const END_MELNORME8_GRPOFFS1: Self = Self(1125);
    pub const MELNORME8_GRPOFFS2: Self = Self(1126);
    pub const END_MELNORME8_GRPOFFS2: Self = Self(1133);
    pub const MELNORME8_GRPOFFS3: Self = Self(1134);
    pub const END_MELNORME8_GRPOFFS3: Self = Self(1141);
    pub const URQUAN_PROBE_GRPOFFS0: Self = Self(1142);
    pub const END_URQUAN_PROBE_GRPOFFS0: Self = Self(1149);
    pub const URQUAN_PROBE_GRPOFFS1: Self = Self(1150);
    pub const END_URQUAN_PROBE_GRPOFFS1: Self = Self(1157);
    pub const URQUAN_PROBE_GRPOFFS2: Self = Self(1158);
    pub const END_URQUAN_PROBE_GRPOFFS2: Self = Self(1165);
    pub const URQUAN_PROBE_GRPOFFS3: Self = Self(1166);
    pub const END_URQUAN_PROBE_GRPOFFS3: Self = Self(1173);
    pub const COLONY_GRPOFFS0: Self = Self(1174);
    pub const END_COLONY_GRPOFFS0: Self = Self(1181);
    pub const COLONY_GRPOFFS1: Self = Self(1182);
    pub const END_COLONY_GRPOFFS1: Self = Self(1189);
    pub const COLONY_GRPOFFS2: Self = Self(1190);
    pub const END_COLONY_GRPOFFS2: Self = Self(1197);
    pub const COLONY_GRPOFFS3: Self = Self(1198);
    pub const END_COLONY_GRPOFFS3: Self = Self(1205);
    pub const SAMATRA_GRPOFFS0: Self = Self(1206);
    pub const END_SAMATRA_GRPOFFS0: Self = Self(1213);
    pub const SAMATRA_GRPOFFS1: Self = Self(1214);
    pub const END_SAMATRA_GRPOFFS1: Self = Self(1221);
    pub const SAMATRA_GRPOFFS2: Self = Self(1222);
    pub const END_SAMATRA_GRPOFFS2: Self = Self(1229);
    pub const SAMATRA_GRPOFFS3: Self = Self(1230);
    pub const END_SAMATRA_GRPOFFS3: Self = Self(1237);
    pub const NUM_GAME_STATE_BITS: Self = Self(1238);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_3(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_3 {
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
    pub GameState: [BYTE; 155],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GLOBDATA {
    pub SIS_state: SIS_STATE,
    pub Game_state: GAME_STATE,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_4(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_4 {
    pub const STAR_STRING_BASE: Self = Self(0);
    pub const DEVICE_STRING_BASE: Self = Self(133);
    pub const CARGO_STRING_BASE: Self = Self(162);
    pub const ELEMENTS_STRING_BASE: Self = Self(172);
    pub const SCAN_STRING_BASE: Self = Self(305);
    pub const STAR_NUMBER_BASE: Self = Self(361);
    pub const PLANET_NUMBER_BASE: Self = Self(375);
    pub const MONTHS_STRING_BASE: Self = Self(408);
    pub const FEEDBACK_STRING_BASE: Self = Self(420);
    pub const STARBASE_STRING_BASE: Self = Self(422);
    pub const ENCOUNTER_STRING_BASE: Self = Self(427);
    pub const NAVIGATION_STRING_BASE: Self = Self(435);
    pub const NAMING_STRING_BASE: Self = Self(441);
    pub const MELEE_STRING_BASE: Self = Self(445);
    pub const SAVEGAME_STRING_BASE: Self = Self(454);
    pub const OPTION_STRING_BASE: Self = Self(459);
    pub const QUITMENU_STRING_BASE: Self = Self(464);
    pub const STATUS_STRING_BASE: Self = Self(467);
    pub const FLAGSHIP_STRING_BASE: Self = Self(473);
    pub const ORBITSCAN_STRING_BASE: Self = Self(486);
    pub const MAINMENU_STRING_BASE: Self = Self(505);
    pub const NETMELEE_STRING_BASE: Self = Self(560);
    pub const GAMESTR_COUNT: Self = Self(579);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_5(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_5 {
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
    t.baseline.x = 40 as COORD;
    r.extent.width = (64 as ::core::ffi::c_int - 5 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int - (4 as ::core::ffi::c_int + 16 as ::core::ffi::c_int)
        + 1 as ::core::ffi::c_int) as COORD;
    r.extent.height = (7 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as COORD;
    r.corner.x = (4 as ::core::ffi::c_int + 16 as ::core::ffi::c_int) as COORD;
    r.corner.y = (33 as ::core::ffi::c_int
        + pos as ::core::ffi::c_int
            * (16 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
        + 2 as ::core::ffi::c_int) as COORD;
    SetContextForeGroundColor(
        if selected as ::core::ffi::c_int != 0 {
            colorFromRgb15(0xa as BYTE, 0xa as BYTE, 0x1f as BYTE)
        } else {
            colorFromRgb15(0 as BYTE, 0 as BYTE, 0x14 as BYTE)
        },
    );
    DrawFilledRectangle(&raw mut r);
    SetContextFont(TinyFont);
    SetContextForeGroundColor(
        if selected as ::core::ffi::c_int != 0 {
            colorFromRgb15(0xa as BYTE, 0x1f as BYTE, 0x1f as BYTE)
        } else {
            colorFromRgb15(0 as BYTE, 0x14 as BYTE, 0x14 as BYTE)
        },
    );
    t.baseline.y = (r.corner.y as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as COORD;
    t.pStr = GetStringAddress(
        SetAbsStringTableIndex(
            GameStrings,
            (device as ::core::ffi::c_int
                + C2Rust_Unnamed_4::DEVICE_STRING_BASE.0 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int) as COUNT,
        ),
    );
    t.CharCount = utf8StringPos(t.pStr, ' ' as UniChar) as COUNT;
    font_DrawText(&raw mut t);
    t.baseline.y = (t.baseline.y as ::core::ffi::c_int + 7 as ::core::ffi::c_int)
        as COORD;
    t.pStr = skipUTF8Chars(
        t.pStr,
        (t.CharCount as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as size_t,
    );
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    font_DrawText(&raw mut t);
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
    r.corner.x = 2 as COORD;
    r.corner.y = 20 as COORD;
    r.extent.width = (64 as ::core::ffi::c_int - 5 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as COORD;
    r.extent.height = (129 as ::core::ffi::c_int - r.corner.y as ::core::ffi::c_int)
        as COORD;
    DrawStarConBox(
        &raw mut r,
        1 as SIZE,
        colorFromRgb15(0x10 as BYTE, 0x10 as BYTE, 0x10 as BYTE),
        colorFromRgb15(0x8 as BYTE, 0x8 as BYTE, 0x8 as BYTE),
        BOOLEAN::TRUE,
        colorFromRgb15(0 as BYTE, 0 as BYTE, 0x14 as BYTE),
    );
    SetContextFont(StarConFont);
    t.baseline.x = ((64 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
        - 1 as ::core::ffi::c_int) as COORD;
    t.baseline.y = (r.corner.y as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as COORD;
    t.align = TEXT_ALIGN::ALIGN_CENTER;
    t.pStr = GetStringAddress(
        SetAbsStringTableIndex(
            GameStrings,
            C2Rust_Unnamed_4::DEVICE_STRING_BASE.0 as ::core::ffi::c_int as COUNT,
        ),
    );
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    SetContextForeGroundColor(colorFromRgb15(0xa as BYTE, 0x1f as BYTE, 0x1f as BYTE));
    font_DrawText(&raw mut t);
    s.origin.x = 4 as COORD;
    cy = 33 as COORD;
    i = 0 as COUNT;
    while (i as ::core::ffi::c_int)
        < (129 as ::core::ffi::c_int - 33 as ::core::ffi::c_int)
            / (16 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
    {
        let mut devIndex: COUNT = ((*devState).topIndex as ::core::ffi::c_int
            + i as ::core::ffi::c_int) as COUNT;
        if devIndex as ::core::ffi::c_int >= (*devState).count as ::core::ffi::c_int {
            break;
        }
        s.origin.y = (cy as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as COORD;
        s.frame = SetAbsFrameIndex(
            MiscDataFrame,
            (77 as ::core::ffi::c_int
                + (*devState).list[devIndex as usize] as ::core::ffi::c_int) as COUNT,
        );
        DrawStamp(&raw mut s);
        DrawDevice((*devState).list[devIndex as usize] as COUNT, i, false);
        i = i.wrapping_add(1);
        cy = (cy as ::core::ffi::c_int
            + (16 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)) as COORD;
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
        DrawDevicesDisplay(devState);
        OldDevice = NewDevice;
    }
    if OldDevice as ::core::ffi::c_int != NewDevice as ::core::ffi::c_int {
        DrawDevice(
            (*devState).list[OldDevice as usize] as COUNT,
            (OldDevice as ::core::ffi::c_int
                - (*devState).topIndex as ::core::ffi::c_int) as COUNT,
            false,
        );
    }
    if (NewDevice as ::core::ffi::c_int)
        < C2Rust_Unnamed_1::NUM_DEVICES.0 as ::core::ffi::c_int
    {
        DrawDevice(
            (*devState).list[NewDevice as usize] as COUNT,
            (NewDevice as ::core::ffi::c_int
                - (*devState).topIndex as ::core::ffi::c_int) as COUNT,
            true,
        );
    }
    UnbatchGraphics();
}
unsafe extern "C" fn UseCaster() -> BOOLEAN {
    if inHQSpace().0 != 0 {
        if getGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_2::ARILOU_SPACE_SIDE.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_2::END_ARILOU_SPACE_SIDE.0 as ::core::ffi::c_int,
        ) as ::core::ffi::c_int <= 1 as ::core::ffi::c_int
        {
            setGameState(
                &raw mut GlobData.Game_state.GameState as *mut BYTE,
                C2Rust_Unnamed_2::USED_BROADCASTER.0 as ::core::ffi::c_int,
                C2Rust_Unnamed_2::END_USED_BROADCASTER.0 as ::core::ffi::c_int,
                1 as BYTE,
            );
            return BOOLEAN::TRUE;
        }
        return BOOLEAN::FALSE;
    }
    if GlobData.Game_state.CurrentActivity as BYTE as ::core::ffi::c_int
        != C2Rust_Unnamed_3::IN_INTERPLANETARY.0 as ::core::ffi::c_int
        || !playerInSolarSystem()
    {
        return BOOLEAN::FALSE;
    }
    if playerInPlanetOrbit() as ::core::ffi::c_int != 0
        && matchWorld(
            pSolarSysState,
            (*pSolarSysState).pOrbitalDesc,
            1 as BYTE,
            -1 as ::core::ffi::c_int as BYTE,
        ) as ::core::ffi::c_int != 0
        && (*CurStarDescPtr).Index as ::core::ffi::c_int
            == C2Rust_Unnamed_0::CHMMR_DEFINED.0 as ::core::ffi::c_int
        && getGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_2::CHMMR_UNLEASHED.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_2::END_CHMMR_UNLEASHED.0 as ::core::ffi::c_int,
        ) == 0
    {
        NextActivity = (NextActivity as ::core::ffi::c_int
            | C2Rust_Unnamed_3::CHECK_LOAD.0 as ::core::ffi::c_int) as ACTIVITY;
        GlobData.Game_state.CurrentActivity = (GlobData.Game_state.CurrentActivity
            as ::core::ffi::c_int
            | C2Rust_Unnamed_3::START_ENCOUNTER.0 as ::core::ffi::c_int) as ACTIVITY;
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
                    247 as ::core::ffi::c_int,
                    b"UseCaster\0".as_ptr() as *const ::core::ffi::c_char,
                );
                0 as ::core::ffi::c_int != 0
            }) as ::core::ffi::c_int;
        setGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_2::GLOBAL_FLAGS_AND_DATA.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_2::END_GLOBAL_FLAGS_AND_DATA.0 as ::core::ffi::c_int,
            ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as BYTE,
        );
        SaveSolarSysLocation();
        return BOOLEAN::TRUE;
    }
    let mut FoundIlwrath: BOOLEAN = BOOLEAN::FALSE;
    let mut hGroup: HIPGROUP = ::core::ptr::null_mut::<::core::ffi::c_void>();
    FoundIlwrath = BOOLEAN(
        ((*CurStarDescPtr).Index as ::core::ffi::c_int
            == C2Rust_Unnamed_0::ILWRATH_DEFINED.0 as ::core::ffi::c_int
            && StartSphereTracking(
                C2Rust_Unnamed::ILWRATH_SHIP.0 as ::core::ffi::c_int as COUNT,
            ) as ::core::ffi::c_int != 0) as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                == C2Rust_Unnamed::ILWRATH_SHIP.0 as ::core::ffi::c_int)
                as ::core::ffi::c_int as ::core::ffi::c_uint,
        );
        UnlockLink(&raw mut GlobData.Game_state.ip_group_q, hGroup);
    }
    if FoundIlwrath.0 != 0 {
        NextActivity = (NextActivity as ::core::ffi::c_int
            | C2Rust_Unnamed_3::CHECK_LOAD.0 as ::core::ffi::c_int) as ACTIVITY;
        GlobData.Game_state.CurrentActivity = (GlobData.Game_state.CurrentActivity
            as ::core::ffi::c_int
            | C2Rust_Unnamed_3::START_ENCOUNTER.0 as ::core::ffi::c_int) as ACTIVITY;
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
                    281 as ::core::ffi::c_int,
                    b"UseCaster\0".as_ptr() as *const ::core::ffi::c_char,
                );
                0 as ::core::ffi::c_int != 0
            }) as ::core::ffi::c_int;
        if (*CurStarDescPtr).Index as ::core::ffi::c_int
            == C2Rust_Unnamed_0::ILWRATH_DEFINED.0 as ::core::ffi::c_int
        {
            setGameState(
                &raw mut GlobData.Game_state.GameState as *mut BYTE,
                C2Rust_Unnamed_2::GLOBAL_FLAGS_AND_DATA.0 as ::core::ffi::c_int,
                C2Rust_Unnamed_2::END_GLOBAL_FLAGS_AND_DATA.0 as ::core::ffi::c_int,
                ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as BYTE,
            );
        } else {
            setGameState(
                &raw mut GlobData.Game_state.GameState as *mut BYTE,
                C2Rust_Unnamed_2::GLOBAL_FLAGS_AND_DATA.0 as ::core::ffi::c_int,
                C2Rust_Unnamed_2::END_GLOBAL_FLAGS_AND_DATA.0 as ::core::ffi::c_int,
                ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as BYTE,
            );
        }
        if playerInPlanetOrbit() {
            SaveSolarSysLocation();
        }
        return BOOLEAN::TRUE;
    }
    return BOOLEAN::FALSE;
}
unsafe extern "C" fn InvokeDevice(mut which_device: BYTE) -> DeviceStatus {
    let mut val: BYTE = 0;
    match which_device as ::core::ffi::c_int {
        4 => {
            val = getGameState(
                &raw mut GlobData.Game_state.GameState as *mut BYTE,
                C2Rust_Unnamed_2::ULTRON_CONDITION.0 as ::core::ffi::c_int,
                C2Rust_Unnamed_2::END_ULTRON_CONDITION.0 as ::core::ffi::c_int,
            );
            if val != 0 {
                setGameState(
                    &raw mut GlobData.Game_state.GameState as *mut BYTE,
                    C2Rust_Unnamed_2::ULTRON_CONDITION.0 as ::core::ffi::c_int,
                    C2Rust_Unnamed_2::END_ULTRON_CONDITION.0 as ::core::ffi::c_int,
                    (val as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as BYTE,
                );
                setGameState(
                    &raw mut GlobData.Game_state.GameState as *mut BYTE,
                    C2Rust_Unnamed_2::ROSY_SPHERE_ON_SHIP.0 as ::core::ffi::c_int,
                    C2Rust_Unnamed_2::END_ROSY_SPHERE_ON_SHIP.0 as ::core::ffi::c_int,
                    0 as BYTE,
                );
                setGameState(
                    &raw mut GlobData.Game_state.GameState as *mut BYTE,
                    C2Rust_Unnamed_2::DISCUSSED_ULTRON.0 as ::core::ffi::c_int,
                    C2Rust_Unnamed_2::END_DISCUSSED_ULTRON.0 as ::core::ffi::c_int,
                    0 as BYTE,
                );
                setGameState(
                    &raw mut GlobData.Game_state.GameState as *mut BYTE,
                    C2Rust_Unnamed_2::SUPOX_ULTRON_HELP.0 as ::core::ffi::c_int,
                    C2Rust_Unnamed_2::END_SUPOX_ULTRON_HELP.0 as ::core::ffi::c_int,
                    0 as BYTE,
                );
                return DeviceStatus::DEVICE_SUCCESS;
            }
        }
        3 => {
            if GlobData.Game_state.CurrentActivity as BYTE as ::core::ffi::c_int
                == C2Rust_Unnamed_3::IN_INTERPLANETARY.0 as ::core::ffi::c_int
                && playerInPlanetOrbit() as ::core::ffi::c_int != 0
            {
                PlayMenuSound(MENU_SOUND_EFFECT::MENU_SOUND_INVOKED);
                SleepThreadUntil(
                    FadeScreen(
                            ScreenFadeType::FadeAllToWhite,
                            (840 as ::core::ffi::c_int * 1 as ::core::ffi::c_int) as SIZE,
                        )
                        .wrapping_add(
                            (840 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                                as TimeCount,
                        ),
                );
                if (*CurStarDescPtr).Index as ::core::ffi::c_int
                    != C2Rust_Unnamed_0::CHMMR_DEFINED.0 as ::core::ffi::c_int
                    || !matchWorld(
                        pSolarSysState,
                        (*pSolarSysState).pOrbitalDesc,
                        1 as BYTE,
                        -1 as ::core::ffi::c_int as BYTE,
                    )
                {
                    FadeScreen(
                        ScreenFadeType::FadeAllToColor,
                        (840 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as SIZE,
                    );
                } else {
                    setGameState(
                        &raw mut GlobData.Game_state.GameState as *mut BYTE,
                        C2Rust_Unnamed_2::CHMMR_EMERGING.0 as ::core::ffi::c_int,
                        C2Rust_Unnamed_2::END_CHMMR_EMERGING.0 as ::core::ffi::c_int,
                        1 as BYTE,
                    );
                    EncounterGroup = 0 as BYTE;
                    GlobData.Game_state.CurrentActivity = (GlobData
                        .Game_state
                        .CurrentActivity as ::core::ffi::c_int
                        | C2Rust_Unnamed_3::START_ENCOUNTER.0 as ::core::ffi::c_int)
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
                                348 as ::core::ffi::c_int,
                                b"InvokeDevice\0".as_ptr() as *const ::core::ffi::c_char,
                            );
                            0 as ::core::ffi::c_int != 0
                        }) as ::core::ffi::c_int;
                    CloneShipFragment(
                        C2Rust_Unnamed::CHMMR_SHIP.0 as ::core::ffi::c_int as COUNT,
                        &raw mut GlobData.Game_state.npc_built_ship_q,
                        0 as COUNT,
                    );
                }
                return DeviceStatus::DEVICE_SUCCESS_NO_SOUND;
            }
        }
        2 => {
            setGameState(
                &raw mut GlobData.Game_state.GameState as *mut BYTE,
                C2Rust_Unnamed_2::UTWIG_BOMB.0 as ::core::ffi::c_int,
                C2Rust_Unnamed_2::END_UTWIG_BOMB.0 as ::core::ffi::c_int,
                0 as BYTE,
            );
            GlobData.Game_state.CurrentActivity = (GlobData.Game_state.CurrentActivity
                as ::core::ffi::c_int
                & !(C2Rust_Unnamed_3::IN_BATTLE.0 as ::core::ffi::c_int)) as ACTIVITY;
            GlobData.SIS_state.CrewEnlisted = !(0 as ::core::ffi::c_int) as COUNT;
            return DeviceStatus::DEVICE_SUCCESS;
        }
        1 => {
            NextActivity = (NextActivity as ::core::ffi::c_int
                | C2Rust_Unnamed_3::CHECK_LOAD.0 as ::core::ffi::c_int) as ACTIVITY;
            GlobData.Game_state.CurrentActivity = (GlobData.Game_state.CurrentActivity
                as ::core::ffi::c_int
                | C2Rust_Unnamed_3::START_ENCOUNTER.0 as ::core::ffi::c_int) as ACTIVITY;
            setGameState(
                &raw mut GlobData.Game_state.GameState as *mut BYTE,
                C2Rust_Unnamed_2::GLOBAL_FLAGS_AND_DATA.0 as ::core::ffi::c_int,
                C2Rust_Unnamed_2::END_GLOBAL_FLAGS_AND_DATA.0 as ::core::ffi::c_int,
                0 as BYTE,
            );
            if inHQSpace().0 != 0 {
                if !GlobData.Game_state.encounter_q.head.is_null() {
                    setGameState(
                        &raw mut GlobData.Game_state.GameState as *mut BYTE,
                        C2Rust_Unnamed_2::SHIP_TO_COMPEL.0 as ::core::ffi::c_int,
                        C2Rust_Unnamed_2::END_SHIP_TO_COMPEL.0 as ::core::ffi::c_int,
                        1 as BYTE,
                    );
                }
                GlobData.Game_state.CurrentActivity = (GlobData
                    .Game_state
                    .CurrentActivity as ::core::ffi::c_int
                    & !(C2Rust_Unnamed_3::IN_BATTLE.0 as ::core::ffi::c_int))
                    as ACTIVITY;
                SaveSisHyperState();
            } else {
                EncounterGroup = 0 as BYTE;
                if !GlobData.Game_state.ip_group_q.head.is_null() {
                    setGameState(
                        &raw mut GlobData.Game_state.GameState as *mut BYTE,
                        C2Rust_Unnamed_2::SHIP_TO_COMPEL.0 as ::core::ffi::c_int,
                        C2Rust_Unnamed_2::END_SHIP_TO_COMPEL.0 as ::core::ffi::c_int,
                        1 as BYTE,
                    );
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
                                394 as ::core::ffi::c_int,
                                b"InvokeDevice\0".as_ptr() as *const ::core::ffi::c_char,
                            );
                            0 as ::core::ffi::c_int != 0
                        }) as ::core::ffi::c_int;
                }
                if (*CurStarDescPtr).Index as ::core::ffi::c_int
                    == C2Rust_Unnamed_0::SAMATRA_DEFINED.0 as ::core::ffi::c_int
                {
                    setGameState(
                        &raw mut GlobData.Game_state.GameState as *mut BYTE,
                        C2Rust_Unnamed_2::READY_TO_CONFUSE_URQUAN.0
                            as ::core::ffi::c_int,
                        C2Rust_Unnamed_2::END_READY_TO_CONFUSE_URQUAN.0
                            as ::core::ffi::c_int,
                        1 as BYTE,
                    );
                }
                if playerInPlanetOrbit() {
                    SaveSolarSysLocation();
                }
            }
            return DeviceStatus::DEVICE_SUCCESS;
        }
        5 => {
            val = getGameState(
                &raw mut GlobData.Game_state.GameState as *mut BYTE,
                C2Rust_Unnamed_2::ULTRON_CONDITION.0 as ::core::ffi::c_int,
                C2Rust_Unnamed_2::END_ULTRON_CONDITION.0 as ::core::ffi::c_int,
            );
            if val != 0 {
                setGameState(
                    &raw mut GlobData.Game_state.GameState as *mut BYTE,
                    C2Rust_Unnamed_2::ULTRON_CONDITION.0 as ::core::ffi::c_int,
                    C2Rust_Unnamed_2::END_ULTRON_CONDITION.0 as ::core::ffi::c_int,
                    (val as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as BYTE,
                );
                setGameState(
                    &raw mut GlobData.Game_state.GameState as *mut BYTE,
                    C2Rust_Unnamed_2::AQUA_HELIX_ON_SHIP.0 as ::core::ffi::c_int,
                    C2Rust_Unnamed_2::END_AQUA_HELIX_ON_SHIP.0 as ::core::ffi::c_int,
                    0 as BYTE,
                );
                setGameState(
                    &raw mut GlobData.Game_state.GameState as *mut BYTE,
                    C2Rust_Unnamed_2::DISCUSSED_ULTRON.0 as ::core::ffi::c_int,
                    C2Rust_Unnamed_2::END_DISCUSSED_ULTRON.0 as ::core::ffi::c_int,
                    0 as BYTE,
                );
                setGameState(
                    &raw mut GlobData.Game_state.GameState as *mut BYTE,
                    C2Rust_Unnamed_2::SUPOX_ULTRON_HELP.0 as ::core::ffi::c_int,
                    C2Rust_Unnamed_2::END_SUPOX_ULTRON_HELP.0 as ::core::ffi::c_int,
                    0 as BYTE,
                );
                return DeviceStatus::DEVICE_SUCCESS;
            }
        }
        6 => {
            val = getGameState(
                &raw mut GlobData.Game_state.GameState as *mut BYTE,
                C2Rust_Unnamed_2::ULTRON_CONDITION.0 as ::core::ffi::c_int,
                C2Rust_Unnamed_2::END_ULTRON_CONDITION.0 as ::core::ffi::c_int,
            );
            if val != 0 {
                setGameState(
                    &raw mut GlobData.Game_state.GameState as *mut BYTE,
                    C2Rust_Unnamed_2::ULTRON_CONDITION.0 as ::core::ffi::c_int,
                    C2Rust_Unnamed_2::END_ULTRON_CONDITION.0 as ::core::ffi::c_int,
                    (val as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as BYTE,
                );
                setGameState(
                    &raw mut GlobData.Game_state.GameState as *mut BYTE,
                    C2Rust_Unnamed_2::CLEAR_SPINDLE_ON_SHIP.0 as ::core::ffi::c_int,
                    C2Rust_Unnamed_2::END_CLEAR_SPINDLE_ON_SHIP.0 as ::core::ffi::c_int,
                    0 as BYTE,
                );
                setGameState(
                    &raw mut GlobData.Game_state.GameState as *mut BYTE,
                    C2Rust_Unnamed_2::DISCUSSED_ULTRON.0 as ::core::ffi::c_int,
                    C2Rust_Unnamed_2::END_DISCUSSED_ULTRON.0 as ::core::ffi::c_int,
                    0 as BYTE,
                );
                setGameState(
                    &raw mut GlobData.Game_state.GameState as *mut BYTE,
                    C2Rust_Unnamed_2::SUPOX_ULTRON_HELP.0 as ::core::ffi::c_int,
                    C2Rust_Unnamed_2::END_SUPOX_ULTRON_HELP.0 as ::core::ffi::c_int,
                    0 as BYTE,
                );
                return DeviceStatus::DEVICE_SUCCESS;
            }
        }
        12 | 13 => {
            if UseCaster().0 != 0 {
                return DeviceStatus::DEVICE_SUCCESS;
            }
        }
        0 => {
            if inHyperSpace().0 != 0
                && GlobData.SIS_state.FuelOnBoard
                    >= (10 as ::core::ffi::c_int * 100 as ::core::ffi::c_int) as DWORD
            {
                GlobData.SIS_state.FuelOnBoard = GlobData
                    .SIS_state
                    .FuelOnBoard
                    .wrapping_sub(
                        (10 as ::core::ffi::c_int * 100 as ::core::ffi::c_int) as DWORD,
                    );
                setGameState(
                    &raw mut GlobData.Game_state.GameState as *mut BYTE,
                    C2Rust_Unnamed_2::PORTAL_COUNTER.0 as ::core::ffi::c_int,
                    C2Rust_Unnamed_2::END_PORTAL_COUNTER.0 as ::core::ffi::c_int,
                    1 as BYTE,
                );
                return DeviceStatus::DEVICE_SUCCESS;
            }
        }
        25 | 26 | 7 | 8 | 9 | 10 | 11 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 | 27
        | _ => {}
    }
    return DeviceStatus::DEVICE_FAILURE;
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
            .menu[C2Rust_Unnamed_5::KEY_MENU_SELECT.0 as ::core::ffi::c_int as usize]
            as ::core::ffi::c_uint,
    );
    cancel = BOOLEAN(
        PulsedInputState
            .menu[C2Rust_Unnamed_5::KEY_MENU_CANCEL.0 as ::core::ffi::c_int as usize]
            as ::core::ffi::c_uint,
    );
    back = BOOLEAN(
        (PulsedInputState
            .menu[C2Rust_Unnamed_5::KEY_MENU_UP.0 as ::core::ffi::c_int as usize] != 0
            || PulsedInputState
                .menu[C2Rust_Unnamed_5::KEY_MENU_LEFT.0 as ::core::ffi::c_int as usize]
                != 0) as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
    forward = BOOLEAN(
        (PulsedInputState
            .menu[C2Rust_Unnamed_5::KEY_MENU_DOWN.0 as ::core::ffi::c_int as usize] != 0
            || PulsedInputState
                .menu[C2Rust_Unnamed_5::KEY_MENU_RIGHT.0 as ::core::ffi::c_int as usize]
                != 0) as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
    pagefwd = BOOLEAN(
        PulsedInputState
            .menu[C2Rust_Unnamed_5::KEY_MENU_PAGE_DOWN.0 as ::core::ffi::c_int as usize]
            as ::core::ffi::c_uint,
    );
    pageback = BOOLEAN(
        PulsedInputState
            .menu[C2Rust_Unnamed_5::KEY_MENU_PAGE_UP.0 as ::core::ffi::c_int as usize]
            as ::core::ffi::c_uint,
    );
    if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
        & C2Rust_Unnamed_3::CHECK_ABORT.0 as ::core::ffi::c_int != 0
    {
        return BOOLEAN::FALSE;
    }
    if cancel.0 != 0 {
        return BOOLEAN::FALSE
    } else if select.0 != 0 {
        let mut status: DeviceStatus = DeviceStatus::DEVICE_FAILURE;
        status = InvokeDevice((*devState).list[(*pMS).CurState as usize]);
        if status.0 == DeviceStatus::DEVICE_FAILURE.0 {
            PlayMenuSound(MENU_SOUND_EFFECT::MENU_SOUND_FAILURE);
        } else if status.0 == DeviceStatus::DEVICE_SUCCESS.0 {
            PlayMenuSound(MENU_SOUND_EFFECT::MENU_SOUND_INVOKED);
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
                + (129 as ::core::ffi::c_int - 33 as ::core::ffi::c_int)
                    / (16 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)) as SIZE;
        } else if pageback.0 != 0 {
            NewState = (NewState as ::core::ffi::c_int
                - (129 as ::core::ffi::c_int - 33 as ::core::ffi::c_int)
                    / (16 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)) as SIZE;
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
                    + (129 as ::core::ffi::c_int - 33 as ::core::ffi::c_int)
                        / (16 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
        {
            NewTop = (NewState as ::core::ffi::c_int
                - NewState as ::core::ffi::c_int
                    % ((129 as ::core::ffi::c_int - 33 as ::core::ffi::c_int)
                        / (16 as ::core::ffi::c_int + 2 as ::core::ffi::c_int))) as SIZE;
        }
        if NewState as ::core::ffi::c_int != (*pMS).CurState as ::core::ffi::c_int {
            if NewTop as ::core::ffi::c_int != (*devState).topIndex as ::core::ffi::c_int
            {
                (*devState).topIndex = NewTop as COUNT;
                DrawDevices(
                    devState,
                    !(0 as ::core::ffi::c_int) as COUNT,
                    NewState as COUNT,
                );
            } else {
                DrawDevices(devState, (*pMS).CurState as COUNT, NewState as COUNT);
            }
            (*pMS).CurState = NewState as BYTE;
        }
        SleepThread(
            (840 as ::core::ffi::c_int / 30 as ::core::ffi::c_int) as TimePeriod,
        );
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
                DeviceState = getGameState(
                    &raw mut GlobData.Game_state.GameState as *mut BYTE,
                    C2Rust_Unnamed_2::ROSY_SPHERE_ON_SHIP.0 as ::core::ffi::c_int,
                    C2Rust_Unnamed_2::END_ROSY_SPHERE_ON_SHIP.0 as ::core::ffi::c_int,
                );
            }
            25 => {
                DeviceState = getGameState(
                    &raw mut GlobData.Game_state.GameState as *mut BYTE,
                    C2Rust_Unnamed_2::ARTIFACT_2_ON_SHIP.0 as ::core::ffi::c_int,
                    C2Rust_Unnamed_2::END_ARTIFACT_2_ON_SHIP.0 as ::core::ffi::c_int,
                );
            }
            26 => {
                DeviceState = getGameState(
                    &raw mut GlobData.Game_state.GameState as *mut BYTE,
                    C2Rust_Unnamed_2::ARTIFACT_3_ON_SHIP.0 as ::core::ffi::c_int,
                    C2Rust_Unnamed_2::END_ARTIFACT_3_ON_SHIP.0 as ::core::ffi::c_int,
                );
            }
            3 => {
                DeviceState = getGameState(
                    &raw mut GlobData.Game_state.GameState as *mut BYTE,
                    C2Rust_Unnamed_2::SUN_DEVICE_ON_SHIP.0 as ::core::ffi::c_int,
                    C2Rust_Unnamed_2::END_SUN_DEVICE_ON_SHIP.0 as ::core::ffi::c_int,
                );
            }
            2 => {
                DeviceState = getGameState(
                    &raw mut GlobData.Game_state.GameState as *mut BYTE,
                    C2Rust_Unnamed_2::UTWIG_BOMB_ON_SHIP.0 as ::core::ffi::c_int,
                    C2Rust_Unnamed_2::END_UTWIG_BOMB_ON_SHIP.0 as ::core::ffi::c_int,
                );
            }
            7 => {
                DeviceState = (getGameState(
                    &raw mut GlobData.Game_state.GameState as *mut BYTE,
                    C2Rust_Unnamed_2::ULTRON_CONDITION.0 as ::core::ffi::c_int,
                    C2Rust_Unnamed_2::END_ULTRON_CONDITION.0 as ::core::ffi::c_int,
                ) as ::core::ffi::c_int == 1 as ::core::ffi::c_int) as ::core::ffi::c_int
                    as BYTE;
            }
            8 => {
                DeviceState = (getGameState(
                    &raw mut GlobData.Game_state.GameState as *mut BYTE,
                    C2Rust_Unnamed_2::ULTRON_CONDITION.0 as ::core::ffi::c_int,
                    C2Rust_Unnamed_2::END_ULTRON_CONDITION.0 as ::core::ffi::c_int,
                ) as ::core::ffi::c_int == 2 as ::core::ffi::c_int) as ::core::ffi::c_int
                    as BYTE;
            }
            9 => {
                DeviceState = (getGameState(
                    &raw mut GlobData.Game_state.GameState as *mut BYTE,
                    C2Rust_Unnamed_2::ULTRON_CONDITION.0 as ::core::ffi::c_int,
                    C2Rust_Unnamed_2::END_ULTRON_CONDITION.0 as ::core::ffi::c_int,
                ) as ::core::ffi::c_int == 3 as ::core::ffi::c_int) as ::core::ffi::c_int
                    as BYTE;
            }
            10 => {
                DeviceState = (getGameState(
                    &raw mut GlobData.Game_state.GameState as *mut BYTE,
                    C2Rust_Unnamed_2::ULTRON_CONDITION.0 as ::core::ffi::c_int,
                    C2Rust_Unnamed_2::END_ULTRON_CONDITION.0 as ::core::ffi::c_int,
                ) as ::core::ffi::c_int == 4 as ::core::ffi::c_int) as ::core::ffi::c_int
                    as BYTE;
            }
            11 => {
                DeviceState = getGameState(
                    &raw mut GlobData.Game_state.GameState as *mut BYTE,
                    C2Rust_Unnamed_2::MAIDENS_ON_SHIP.0 as ::core::ffi::c_int,
                    C2Rust_Unnamed_2::END_MAIDENS_ON_SHIP.0 as ::core::ffi::c_int,
                );
            }
            1 => {
                DeviceState = getGameState(
                    &raw mut GlobData.Game_state.GameState as *mut BYTE,
                    C2Rust_Unnamed_2::TALKING_PET_ON_SHIP.0 as ::core::ffi::c_int,
                    C2Rust_Unnamed_2::END_TALKING_PET_ON_SHIP.0 as ::core::ffi::c_int,
                );
            }
            5 => {
                DeviceState = getGameState(
                    &raw mut GlobData.Game_state.GameState as *mut BYTE,
                    C2Rust_Unnamed_2::AQUA_HELIX_ON_SHIP.0 as ::core::ffi::c_int,
                    C2Rust_Unnamed_2::END_AQUA_HELIX_ON_SHIP.0 as ::core::ffi::c_int,
                );
            }
            6 => {
                DeviceState = getGameState(
                    &raw mut GlobData.Game_state.GameState as *mut BYTE,
                    C2Rust_Unnamed_2::CLEAR_SPINDLE_ON_SHIP.0 as ::core::ffi::c_int,
                    C2Rust_Unnamed_2::END_CLEAR_SPINDLE_ON_SHIP.0 as ::core::ffi::c_int,
                );
            }
            12 => {
                DeviceState = getGameState(
                    &raw mut GlobData.Game_state.GameState as *mut BYTE,
                    C2Rust_Unnamed_2::UMGAH_BROADCASTERS_ON_SHIP.0 as ::core::ffi::c_int,
                    C2Rust_Unnamed_2::END_UMGAH_BROADCASTERS_ON_SHIP.0
                        as ::core::ffi::c_int,
                );
            }
            17 => {
                DeviceState = getGameState(
                    &raw mut GlobData.Game_state.GameState as *mut BYTE,
                    C2Rust_Unnamed_2::TAALO_PROTECTOR_ON_SHIP.0 as ::core::ffi::c_int,
                    C2Rust_Unnamed_2::END_TAALO_PROTECTOR_ON_SHIP.0 as ::core::ffi::c_int,
                );
            }
            18 => {
                DeviceState = getGameState(
                    &raw mut GlobData.Game_state.GameState as *mut BYTE,
                    C2Rust_Unnamed_2::EGG_CASE0_ON_SHIP.0 as ::core::ffi::c_int,
                    C2Rust_Unnamed_2::END_EGG_CASE0_ON_SHIP.0 as ::core::ffi::c_int,
                );
            }
            19 => {
                DeviceState = getGameState(
                    &raw mut GlobData.Game_state.GameState as *mut BYTE,
                    C2Rust_Unnamed_2::EGG_CASE1_ON_SHIP.0 as ::core::ffi::c_int,
                    C2Rust_Unnamed_2::END_EGG_CASE1_ON_SHIP.0 as ::core::ffi::c_int,
                );
            }
            20 => {
                DeviceState = getGameState(
                    &raw mut GlobData.Game_state.GameState as *mut BYTE,
                    C2Rust_Unnamed_2::EGG_CASE2_ON_SHIP.0 as ::core::ffi::c_int,
                    C2Rust_Unnamed_2::END_EGG_CASE2_ON_SHIP.0 as ::core::ffi::c_int,
                );
            }
            21 => {
                DeviceState = getGameState(
                    &raw mut GlobData.Game_state.GameState as *mut BYTE,
                    C2Rust_Unnamed_2::SYREEN_SHUTTLE_ON_SHIP.0 as ::core::ffi::c_int,
                    C2Rust_Unnamed_2::END_SYREEN_SHUTTLE_ON_SHIP.0 as ::core::ffi::c_int,
                );
            }
            22 => {
                DeviceState = getGameState(
                    &raw mut GlobData.Game_state.GameState as *mut BYTE,
                    C2Rust_Unnamed_2::VUX_BEAST_ON_SHIP.0 as ::core::ffi::c_int,
                    C2Rust_Unnamed_2::END_VUX_BEAST_ON_SHIP.0 as ::core::ffi::c_int,
                );
            }
            0 => {
                DeviceState = getGameState(
                    &raw mut GlobData.Game_state.GameState as *mut BYTE,
                    C2Rust_Unnamed_2::PORTAL_SPAWNER_ON_SHIP.0 as ::core::ffi::c_int,
                    C2Rust_Unnamed_2::END_PORTAL_SPAWNER_ON_SHIP.0 as ::core::ffi::c_int,
                );
            }
            24 => {
                DeviceState = getGameState(
                    &raw mut GlobData.Game_state.GameState as *mut BYTE,
                    C2Rust_Unnamed_2::PORTAL_KEY_ON_SHIP.0 as ::core::ffi::c_int,
                    C2Rust_Unnamed_2::END_PORTAL_KEY_ON_SHIP.0 as ::core::ffi::c_int,
                );
            }
            13 => {
                DeviceState = getGameState(
                    &raw mut GlobData.Game_state.GameState as *mut BYTE,
                    C2Rust_Unnamed_2::BURV_BROADCASTERS_ON_SHIP.0 as ::core::ffi::c_int,
                    C2Rust_Unnamed_2::END_BURV_BROADCASTERS_ON_SHIP.0
                        as ::core::ffi::c_int,
                );
            }
            27 => {
                DeviceState = getGameState(
                    &raw mut GlobData.Game_state.GameState as *mut BYTE,
                    C2Rust_Unnamed_2::MOONBASE_ON_SHIP.0 as ::core::ffi::c_int,
                    C2Rust_Unnamed_2::END_MOONBASE_ON_SHIP.0 as ::core::ffi::c_int,
                );
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
    DevicesState.count = InventoryDevices(
        &raw mut DevicesState.list as *mut BYTE,
        C2Rust_Unnamed_1::NUM_DEVICES.0 as ::core::ffi::c_int as COUNT,
    ) as COUNT;
    if DevicesState.count == 0 {
        return BOOLEAN::FALSE;
    }
    DrawDevices(
        &raw mut DevicesState,
        !(0 as ::core::ffi::c_int) as COUNT,
        MenuState.CurState as COUNT,
    );
    SetMenuSounds(
        (((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as MENU_SOUND_FLAGS
            as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as MENU_SOUND_FLAGS
                as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as MENU_SOUND_FLAGS
                as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as MENU_SOUND_FLAGS
                as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as MENU_SOUND_FLAGS
                as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as MENU_SOUND_FLAGS
                as ::core::ffi::c_int) as MENU_SOUND_FLAGS,
        ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as MENU_SOUND_FLAGS,
    );
    MenuState.InputFunc = Some(
        DoManipulateDevices as unsafe extern "C" fn(*mut MENU_STATE) -> BOOLEAN,
    ) as Option<unsafe extern "C" fn(*mut menu_state) -> BOOLEAN>;
    DoInput(&raw mut MenuState as *mut ::core::ffi::c_void, BOOLEAN::TRUE);
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
            & C2Rust_Unnamed_3::CHECK_ABORT.0 as ::core::ffi::c_int == 0
    {
        ClearSISRect(((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as BYTE);
        if getGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_2::PORTAL_COUNTER.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_2::END_PORTAL_COUNTER.0 as ::core::ffi::c_int,
        ) == 0
            && GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
                & C2Rust_Unnamed_3::START_ENCOUNTER.0 as ::core::ffi::c_int == 0
            && GlobData.SIS_state.CrewEnlisted as ::core::ffi::c_int
                != !(0 as ::core::ffi::c_int) as COUNT as ::core::ffi::c_int
        {
            return BOOLEAN::TRUE;
        }
    }
    return BOOLEAN::FALSE;
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
                80 as ::core::ffi::c_int,
                b"LockIpGroup\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    return LockLink(pq, h) as *mut IP_GROUP;
}
