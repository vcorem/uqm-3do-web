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

// Rust translation of src/uqm/planets/planets.c.
extern "C" {
    pub type frame_desc;
    pub type font_desc;
    pub type string_table_entry;
    pub type string_table;
    pub type context_desc;
    pub type drawable_desc;
    pub type tfb_soundsample;
    pub type FlashContext;
    static mut pSolarSysState: *mut SOLARSYS_STATE;
    fn DrawPlanet(dy: ::core::ffi::c_int, tintColor: Color);
    fn DrawStarBackGround();
    fn UninitSphereRotation();
    fn DrawDefaultPlanetSphere();
    fn ZoomInPlanetSphere();
    fn RotatePlanetSphere(keepRate: BOOLEAN);
    fn GeneratePlanetSurface(pPlanetDesc: *mut PLANET_DESC, SurfDefFrame: FRAME);
    fn HFree(p: *mut ::core::ffi::c_void);
    fn __assert_fail(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    );
    fn PLRPlaying(MusicRef: MUSIC_REF) -> BOOLEAN;
    fn DoMenuChooser(pMS: *mut MENU_STATE, BaseState: BYTE) -> BOOLEAN;
    fn DrawMenuStateStrings(beg_index: BYTE, NewState: SWORD);
    static mut ScreenWidth: ::core::ffi::c_int;
    fn DestroyStringTable(StringTable: STRING_TABLE) -> BOOLEAN;
    fn ReleaseStringTable(String: STRING) -> STRING_TABLE;
    fn ScanSystem();
    fn GeneratePlanetSide();
    fn GetScanContext(owner: *mut BOOLEAN) -> CONTEXT;
    fn DestroyScanContext();
    fn memset(
        _: *mut ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn SetContext(Context: CONTEXT) -> CONTEXT;
    fn SetContextForeGroundColor(Color: Color) -> Color;
    fn SetContextFGFrame(Frame: FRAME) -> FRAME;
    fn SetContextClipRect(pRect: *mut RECT) -> BOOLEAN;
    fn GetContextClipRect(pRect: *mut RECT) -> BOOLEAN;
    fn DrawStamp(pStamp: *mut STAMP);
    fn DrawFilledRectangle(pRect: *mut RECT);
    fn BatchGraphics();
    fn UnbatchGraphics();
    fn CreateContextAux(name: *const ::core::ffi::c_char) -> CONTEXT;
    fn DestroyContext(ContextRef: CONTEXT) -> BOOLEAN;
    fn DestroyDrawable(Drawable: DRAWABLE) -> BOOLEAN;
    fn LoadGraphicInstance(res: RESOURCE) -> *mut ::core::ffi::c_void;
    fn DestroyFont(FontRef: FONT) -> BOOLEAN;
    fn CaptureDrawable(Drawable: DRAWABLE) -> FRAME;
    fn ReleaseDrawable(Frame: FRAME) -> DRAWABLE;
    static mut disp_q: QUEUE;
    static mut LanderMusic: MUSIC_REF;
    fn SetPlanetMusic(planet_type: BYTE);
    fn StopMusic();
    fn PlayMusic(MusicRef: MUSIC_REF, Continuous: BOOLEAN, Priority: BYTE);
    fn SetMenuSounds(sound_0: MENU_SOUND_FLAGS, sound_1: MENU_SOUND_FLAGS);
    fn CargoMenu();
    fn RosterMenu() -> BOOLEAN;
    fn DevicesMenu() -> BOOLEAN;
    fn StarMap() -> BOOLEAN;
    fn GameOptions() -> BOOLEAN;
    static mut PulsedInputState: CONTROLLER_INPUT_STATE;
    fn SetInputCallback(_: Option<InputFrameCallback>) -> Option<InputFrameCallback>;
    fn DoInput(pInputState: *mut ::core::ffi::c_void, resetInput: BOOLEAN);
    fn RepairSISBorder();
    fn DrawSISFrame();
    fn SetFlashRect(pRect: *const RECT);
    fn DrawSISTitle(pStr: *mut UNICODE);
    fn DrawSISMessage(pStr: *const UNICODE);
    static mut GlobData: GLOBDATA;
    static mut Screen: FRAME;
    static mut SpaceContext: CONTEXT;
    static mut LastActivity: ACTIVITY;
    static mut disableInteractivity: BOOLEAN;
    static mut optWhichMenu: ::core::ffi::c_int;
    fn LoadIntoExtraScreen(r: *mut RECT);
    fn SetTransitionSource(pRect: *const RECT);
    fn ScreenTransition(transition: ::core::ffi::c_int, pRect: *const RECT);
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
pub type PLANET_DESC = planet_desc;
pub type STAMP = stamp;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stamp {
    pub origin: POINT,
    pub frame: FRAME,
}
pub type FRAME = *mut FRAME_DESC;
pub type FRAME_DESC = frame_desc;
pub type POINT = point;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct point {
    pub x: COORD,
    pub y: COORD,
}
pub type COORD = SWORD;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Color {
    pub r: BYTE,
    pub g: BYTE,
    pub b: BYTE,
    pub a: BYTE,
}
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
pub type PLANET_ORBIT = planet_orbit;
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
    pub PlanetToSunDist: UWORD,
    pub PlanDataPtr: *const PlanetFrame,
    pub ScanSeed: [DWORD; 3],
    pub ScanRetrieveMask: [DWORD; 3],
    pub DiscoveryString: STRING,
    pub LanderFont: FONT,
    pub LanderFontEff: FRAME,
}
pub type FONT = *mut FONT_DESC;
pub type FONT_DESC = font_desc;
pub type STRING = *mut STRING_TABLE_ENTRY_DESC;
pub type STRING_TABLE_ENTRY_DESC = string_table_entry;
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
pub type RESOURCE = *const ::core::ffi::c_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ELEMENT_ENTRY {
    pub ElementType: BYTE,
    pub Density: BYTE,
}
pub type COLORMAP = STRING;
pub type size_t = usize;
pub type STRING_TABLE_DESC = string_table;
pub type STRING_TABLE = *mut STRING_TABLE_DESC;
pub type CONTEXT_DESC = context_desc;
pub type DRAWABLE_DESC = drawable_desc;
pub type CONTEXT = *mut CONTEXT_DESC;
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
pub type TFB_SoundSample = tfb_soundsample;
pub type MUSIC_REF = *mut *mut TFB_SoundSample;
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
pub struct C2Rust_Unnamed(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed {
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
    pub const PM_CHANGE_CAPTAIN: Self = Self(36);
    pub const PM_CHANGE_SHIP: Self = Self(37);
    pub const PM_EXIT_SETTINGS: Self = Self(38);
    pub const PM_ALT_SCAN: Self = Self(39);
    pub const PM_ALT_STARMAP: Self = Self(40);
    pub const PM_ALT_MANIFEST: Self = Self(41);
    pub const PM_ALT_GAME_MENU: Self = Self(42);
    pub const PM_ALT_NAVIGATE: Self = Self(43);
    pub const PM_ALT_CARGO: Self = Self(44);
    pub const PM_ALT_DEVICES: Self = Self(45);
    pub const PM_ALT_ROSTER: Self = Self(46);
    pub const PM_ALT_EXIT_MANIFEST: Self = Self(47);
    pub const PM_ALT_MSCAN: Self = Self(48);
    pub const PM_ALT_ESCAN: Self = Self(49);
    pub const PM_ALT_BSCAN: Self = Self(50);
    pub const PM_ALT_ASCAN: Self = Self(51);
    pub const PM_ALT_DISPATCH: Self = Self(52);
    pub const PM_ALT_EXIT_SCAN: Self = Self(53);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_0 {
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
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct DRAW_ORBITAL_MODE(pub ::core::ffi::c_uint);
impl DRAW_ORBITAL_MODE {
    pub const DRAW_ORBITAL_FULL: Self = Self(0);
    pub const DRAW_ORBITAL_WAIT: Self = Self(1);
    pub const DRAW_ORBITAL_UPDATE: Self = Self(2);
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
pub struct GLOBDATA {
    pub SIS_state: SIS_STATE,
    pub Game_state: GAME_STATE,
}
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
pub type ACTIVITY = UWORD;
pub type HELEMENT = HLINK;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_1 {
    pub frame: FRAME,
    pub farray: *mut FRAME,
}
pub type STATE = state;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct state {
    pub location: POINT,
    pub image: C2Rust_Unnamed_1,
}
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
    pub c2rust_unnamed: C2Rust_Unnamed_6,
    pub c2rust_unnamed_0: C2Rust_Unnamed_5,
    pub c2rust_unnamed_1: C2Rust_Unnamed_4,
    pub c2rust_unnamed_2: C2Rust_Unnamed_3,
    pub c2rust_unnamed_3: C2Rust_Unnamed_2,
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
pub union C2Rust_Unnamed_2 {
    pub thrust_wait: BYTE,
    pub blast_offset: BYTE,
    pub next_turn: BYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_3 {
    pub turn_wait: BYTE,
    pub sys_loc: BYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_4 {
    pub mass_points: BYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_5 {
    pub crew_level: COUNT,
    pub hit_points: COUNT,
    pub facing: COUNT,
    pub cycle: COUNT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_6 {
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
pub type LINK = link;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct link {
    pub pred: HLINK,
    pub succ: HLINK,
}
pub type InputFrameCallback = unsafe extern "C" fn() -> ();
pub type CONTROLLER_INPUT_STATE = _controller_input_state;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _controller_input_state {
    pub key: [[::core::ffi::c_int; 7]; 6],
    pub menu: [::core::ffi::c_int; 24],
}
pub type MENU_SOUND_FLAGS = UWORD;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_7(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_7 {
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_8(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_8 {
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
pub struct PlanetMenuItems(pub ::core::ffi::c_uint);
impl PlanetMenuItems {
    pub const SCAN: Self = Self(0);
    pub const STARMAP: Self = Self(1);
    pub const EQUIP_DEVICE: Self = Self(2);
    pub const CARGO: Self = Self(3);
    pub const ROSTER: Self = Self(4);
    pub const GAME_MENU: Self = Self(5);
    pub const NAVIGATION: Self = Self(6);
}
#[no_mangle]
pub static mut PlanetContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
unsafe extern "C" fn CreatePlanetContext() {
    let mut oldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    (PlanetContext.is_null()
        || {
            __assert_fail(
                b"PlanetContext == NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/uqm/planets/planets.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                60 as ::core::ffi::c_int,
                b"CreatePlanetContext\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    oldContext = SetContext(SpaceContext);
    GetContextClipRect(&raw mut r);
    PlanetContext = CreateContextAux(
        b"PlanetContext\0".as_ptr() as *const ::core::ffi::c_char,
    );
    SetContext(PlanetContext);
    SetContextFGFrame(Screen);
    r.extent.height = (r.extent.height as ::core::ffi::c_int
        - (75 as ::core::ffi::c_int - 0 as ::core::ffi::c_int + 5 as ::core::ffi::c_int))
        as COORD;
    SetContextClipRect(&raw mut r);
    SetContext(oldContext);
}
unsafe extern "C" fn DestroyPlanetContext() {
    if !PlanetContext.is_null() {
        DestroyContext(PlanetContext);
        PlanetContext = ::core::ptr::null_mut::<CONTEXT_DESC>();
    }
}
#[no_mangle]
pub unsafe extern "C" fn DrawScannedObjects(mut Reversed: BOOLEAN) {
    let mut hElement: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut hNextElement: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
    hElement = (if Reversed.0 != 0 { disp_q.tail } else { disp_q.head }) as HELEMENT;
    while !hElement.is_null() {
        let mut ElementPtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
        ElementPtr = LockLink(&raw mut disp_q, hElement) as *mut ELEMENT;
        hNextElement = (if Reversed.0 != 0 {
            (*ElementPtr).pred
        } else {
            (*ElementPtr).succ
        }) as HELEMENT;
        if (*ElementPtr).state_flags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int != 0
        {
            let mut s: STAMP = STAMP {
                origin: POINT { x: 0, y: 0 },
                frame: ::core::ptr::null_mut::<FRAME_DESC>(),
            };
            s.origin = (*ElementPtr).current.location;
            s.frame = (*ElementPtr).next.image.frame;
            DrawStamp(&raw mut s);
        }
        UnlockLink(&raw mut disp_q, hElement);
        hElement = hNextElement;
    }
}
#[no_mangle]
pub unsafe extern "C" fn DrawPlanetSurfaceBorder() {
    let mut oldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    let mut oldClipRect: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut clipRect: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    oldContext = SetContext(SpaceContext);
    GetContextClipRect(&raw mut oldClipRect);
    clipRect = oldClipRect;
    clipRect.corner.x = (clipRect.corner.x as ::core::ffi::c_int
        - 1 as ::core::ffi::c_int) as COORD;
    clipRect.extent.width = (clipRect.extent.width as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as COORD;
    clipRect.extent.height = (clipRect.extent.height as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as COORD;
    SetContextClipRect(&raw mut clipRect);
    BatchGraphics();
    SetContextForeGroundColor(colorFromRgb15(0xa as BYTE, 0xa as BYTE, 0xa as BYTE));
    r.corner.x = 0 as COORD;
    r.corner.y = (clipRect.extent.height as ::core::ffi::c_int
        - (75 as ::core::ffi::c_int - 0 as ::core::ffi::c_int) - 5 as ::core::ffi::c_int)
        as COORD;
    r.extent.width = clipRect.extent.width;
    r.extent.height = (5 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as COORD;
    DrawFilledRectangle(&raw mut r);
    SetContextForeGroundColor(colorFromRgb15(0x8 as BYTE, 0x8 as BYTE, 0x8 as BYTE));
    r.extent.width = (r.extent.width as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
        as COORD;
    r.extent.height = 1 as COORD;
    r.corner.x = 1 as COORD;
    r.corner.y = (r.corner.y as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as COORD;
    DrawFilledRectangle(&raw mut r);
    r.extent.width = 1 as COORD;
    r.extent.height = (75 as ::core::ffi::c_int - 0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as COORD;
    r.corner.y = (r.corner.y as ::core::ffi::c_int
        + (5 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)) as COORD;
    r.corner.x = (clipRect.extent.width as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
        as COORD;
    DrawFilledRectangle(&raw mut r);
    SetContextForeGroundColor(colorFromRgb15(0x10 as BYTE, 0x10 as BYTE, 0x10 as BYTE));
    r.corner.x = (r.corner.x as ::core::ffi::c_int
        - (ScreenWidth - 64 as ::core::ffi::c_int
            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
            - 14 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)) as COORD;
    DrawFilledRectangle(&raw mut r);
    r.extent.width = (ScreenWidth - 64 as ::core::ffi::c_int
        - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int - 14 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as COORD;
    r.extent.height = 1 as COORD;
    DrawFilledRectangle(&raw mut r);
    UnbatchGraphics();
    SetContextClipRect(&raw mut oldClipRect);
    SetContext(oldContext);
}
unsafe extern "C" fn DrawOrbitalDisplay(mut Mode: DRAW_ORBITAL_MODE) {
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    SetContext(SpaceContext);
    GetContextClipRect(&raw mut r);
    BatchGraphics();
    if Mode.0 != DRAW_ORBITAL_MODE::DRAW_ORBITAL_UPDATE.0 {
        SetTransitionSource(::core::ptr::null::<RECT>());
        DrawSISFrame();
        DrawSISMessage(::core::ptr::null::<UNICODE>());
        DrawSISTitle(&raw mut GlobData.SIS_state.PlanetName as *mut UNICODE);
        DrawStarBackGround();
        DrawPlanetSurfaceBorder();
    }
    if Mode.0 == DRAW_ORBITAL_MODE::DRAW_ORBITAL_WAIT.0 {
        let mut s: STAMP = STAMP {
            origin: POINT { x: 0, y: 0 },
            frame: ::core::ptr::null_mut::<FRAME_DESC>(),
        };
        SetContext(GetScanContext(::core::ptr::null_mut::<BOOLEAN>()));
        s.frame = CaptureDrawable(
            LoadGraphicInstance(b"graphics.orbitenter\0".as_ptr() as RESOURCE)
                as DRAWABLE,
        );
        s.origin.x = -0 as COORD;
        s.origin.y = 0 as COORD;
        DrawStamp(&raw mut s);
        DestroyDrawable(ReleaseDrawable(s.frame));
    } else if Mode.0 == DRAW_ORBITAL_MODE::DRAW_ORBITAL_FULL.0 {
        DrawDefaultPlanetSphere();
    }
    if Mode.0 != DRAW_ORBITAL_MODE::DRAW_ORBITAL_WAIT.0 {
        SetContext(GetScanContext(::core::ptr::null_mut::<BOOLEAN>()));
        DrawPlanet(
            0 as ::core::ffi::c_int,
            colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE),
        );
    }
    if Mode.0 != DRAW_ORBITAL_MODE::DRAW_ORBITAL_UPDATE.0 {
        ScreenTransition(3 as ::core::ffi::c_int, &raw mut r);
    }
    UnbatchGraphics();
    LoadIntoExtraScreen(&raw mut r);
}
#[no_mangle]
pub unsafe extern "C" fn LoadPlanet(mut SurfDefFrame: FRAME) {
    let mut WaitMode: bool = LastActivity as ::core::ffi::c_int
        & C2Rust_Unnamed_8::CHECK_LOAD.0 as ::core::ffi::c_int == 0;
    let mut pPlanetDesc: *mut PLANET_DESC = ::core::ptr::null_mut::<PLANET_DESC>();
    if disableInteractivity.0 != 0 {
        return;
    }
    ((*pSolarSysState).InOrbit.0 != 0 && (*pSolarSysState).TopoFrame.is_null()
        || {
            __assert_fail(
                b"pSolarSysState->InOrbit && !pSolarSysState->TopoFrame\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/uqm/planets/planets.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                257 as ::core::ffi::c_int,
                b"LoadPlanet\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    CreatePlanetContext();
    if WaitMode {
        DrawOrbitalDisplay(DRAW_ORBITAL_MODE::DRAW_ORBITAL_WAIT);
    }
    StopMusic();
    pPlanetDesc = (*pSolarSysState).pOrbitalDesc;
    GeneratePlanetSurface(pPlanetDesc, SurfDefFrame);
    SetPlanetMusic(
        ((*pPlanetDesc).data_index as ::core::ffi::c_int
            & !(C2Rust_Unnamed_0::PLANET_SHIELDED.0 as ::core::ffi::c_int)) as BYTE,
    );
    GeneratePlanetSide();
    if PLRPlaying(
            ::core::ptr::with_exposed_provenance_mut::<
                *mut TFB_SoundSample,
            >(!(0 as ::core::ffi::c_int) as usize),
        )
        .0 == 0
    {
        PlayMusic(LanderMusic, BOOLEAN::TRUE, 1 as BYTE);
    }
    if WaitMode {
        ZoomInPlanetSphere();
        DrawOrbitalDisplay(DRAW_ORBITAL_MODE::DRAW_ORBITAL_UPDATE);
    } else {
        DrawOrbitalDisplay(DRAW_ORBITAL_MODE::DRAW_ORBITAL_FULL);
    };
}
#[no_mangle]
pub unsafe extern "C" fn FreePlanet() {
    let mut i: COUNT = 0;
    let mut Orbit: *mut PLANET_ORBIT = &raw mut (*pSolarSysState).Orbit;
    UninitSphereRotation();
    StopMusic();
    i = 0 as COUNT;
    while (i as usize)
        < ::core::mem::size_of::<[FRAME; 6]>()
            .wrapping_div(::core::mem::size_of::<FRAME>())
    {
        DestroyDrawable(ReleaseDrawable((*pSolarSysState).PlanetSideFrame[i as usize]));
        (*pSolarSysState).PlanetSideFrame[i as usize] = ::core::ptr::null_mut::<
            FRAME_DESC,
        >();
        i = i.wrapping_add(1);
    }
    DestroyStringTable(ReleaseStringTable((*pSolarSysState).XlatRef));
    (*pSolarSysState).XlatRef = ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>();
    DestroyDrawable(ReleaseDrawable((*pSolarSysState).TopoFrame));
    (*pSolarSysState).TopoFrame = ::core::ptr::null_mut::<FRAME_DESC>();
    DestroyStringTable(ReleaseStringTable((*pSolarSysState).OrbitalCMap));
    (*pSolarSysState).OrbitalCMap = ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>();
    HFree((*Orbit).lpTopoData as *mut ::core::ffi::c_void);
    (*Orbit).lpTopoData = ::core::ptr::null_mut::<SBYTE>();
    DestroyDrawable(ReleaseDrawable((*Orbit).TopoZoomFrame));
    (*Orbit).TopoZoomFrame = ::core::ptr::null_mut::<FRAME_DESC>();
    DestroyDrawable(ReleaseDrawable((*Orbit).SphereFrame));
    (*Orbit).SphereFrame = ::core::ptr::null_mut::<FRAME_DESC>();
    DestroyDrawable(ReleaseDrawable((*Orbit).TintFrame));
    (*Orbit).TintFrame = ::core::ptr::null_mut::<FRAME_DESC>();
    (*Orbit).TintColor = colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE);
    DestroyDrawable(ReleaseDrawable((*Orbit).ObjectFrame));
    (*Orbit).ObjectFrame = ::core::ptr::null_mut::<FRAME_DESC>();
    DestroyDrawable(ReleaseDrawable((*Orbit).WorkFrame));
    (*Orbit).WorkFrame = ::core::ptr::null_mut::<FRAME_DESC>();
    HFree((*Orbit).TopoColors as *mut ::core::ffi::c_void);
    (*Orbit).TopoColors = ::core::ptr::null_mut::<Color>();
    HFree((*Orbit).ScratchArray as *mut ::core::ffi::c_void);
    (*Orbit).ScratchArray = ::core::ptr::null_mut::<Color>();
    DestroyStringTable(
        ReleaseStringTable((*pSolarSysState).SysInfo.PlanetInfo.DiscoveryString),
    );
    (*pSolarSysState).SysInfo.PlanetInfo.DiscoveryString = ::core::ptr::null_mut::<
        STRING_TABLE_ENTRY_DESC,
    >();
    FreeLanderFont(&raw mut (*pSolarSysState).SysInfo.PlanetInfo);
    SetContext(SpaceContext);
    DestroyPlanetContext();
    DestroyScanContext();
}
#[no_mangle]
pub unsafe extern "C" fn LoadStdLanderFont(mut info: *mut PLANET_INFO) {
    (*info).LanderFont = LoadGraphicInstance(b"font.lander\0".as_ptr() as RESOURCE)
        as FONT;
    (*info).LanderFontEff = CaptureDrawable(
        LoadGraphicInstance(b"graphics.landerfonteffect\0".as_ptr() as RESOURCE)
            as DRAWABLE,
    );
}
#[no_mangle]
pub unsafe extern "C" fn FreeLanderFont(mut info: *mut PLANET_INFO) {
    DestroyFont((*info).LanderFont);
    (*info).LanderFont = ::core::ptr::null_mut::<FONT_DESC>();
    DestroyDrawable(ReleaseDrawable((*info).LanderFontEff));
    (*info).LanderFontEff = ::core::ptr::null_mut::<FRAME_DESC>();
}
unsafe extern "C" fn DoPlanetOrbit(mut pMS: *mut MENU_STATE) -> BOOLEAN {
    let mut select: BOOLEAN = BOOLEAN(
        PulsedInputState
            .menu[C2Rust_Unnamed_7::KEY_MENU_SELECT.0 as ::core::ffi::c_int as usize]
            as ::core::ffi::c_uint,
    );
    let mut handled: BOOLEAN = BOOLEAN::FALSE;
    if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
        & (C2Rust_Unnamed_8::CHECK_ABORT.0 as ::core::ffi::c_int
            | C2Rust_Unnamed_8::CHECK_LOAD.0 as ::core::ffi::c_int) != 0
        || GlobData.SIS_state.CrewEnlisted as ::core::ffi::c_int
            == !(0 as ::core::ffi::c_int) as COUNT as ::core::ffi::c_int
    {
        return BOOLEAN::FALSE;
    }
    handled = DoMenuChooser(
        pMS,
        C2Rust_Unnamed::PM_SCAN.0 as ::core::ffi::c_int as BYTE,
    );
    if handled.0 != 0 {
        return BOOLEAN::TRUE;
    }
    if select.0 == 0 {
        return BOOLEAN::TRUE;
    }
    SetFlashRect(::core::ptr::null::<RECT>());
    's_136: {
        match (*pMS).CurState as ::core::ffi::c_int {
            0 => {
                ScanSystem();
                if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
                    & C2Rust_Unnamed_8::START_ENCOUNTER.0 as ::core::ffi::c_int != 0
                {
                    return BOOLEAN::FALSE;
                }
                break 's_136;
            }
            2 => {
                select = DevicesMenu();
                if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
                    & C2Rust_Unnamed_8::START_ENCOUNTER.0 as ::core::ffi::c_int != 0
                {
                    return BOOLEAN::FALSE;
                }
                break 's_136;
            }
            3 => {
                CargoMenu();
                break 's_136;
            }
            4 => {
                select = RosterMenu();
                break 's_136;
            }
            5 => {
                if GameOptions().0 == 0 {
                    return BOOLEAN::FALSE;
                }
                break 's_136;
            }
            1 => {
                let mut AutoPilotSet: BOOLEAN = BOOLEAN::FALSE;
                let mut oldCallback: Option<InputFrameCallback> = None;
                oldCallback = SetInputCallback(None);
                RepairSISBorder();
                AutoPilotSet = StarMap();
                if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
                    & C2Rust_Unnamed_8::CHECK_ABORT.0 as ::core::ffi::c_int != 0
                {
                    return BOOLEAN::FALSE;
                }
                SetInputCallback(oldCallback);
                if AutoPilotSet.0 == 0 {
                    DrawOrbitalDisplay(DRAW_ORBITAL_MODE::DRAW_ORBITAL_FULL);
                    break 's_136;
                }
            }
            6 => {}
            _ => {
                break 's_136;
            }
        }
        return BOOLEAN::FALSE;
    }
    if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
        & C2Rust_Unnamed_8::CHECK_ABORT.0 as ::core::ffi::c_int == 0
    {
        if select.0 != 0 {
            if optWhichMenu != 0x2 as ::core::ffi::c_int {
                (*pMS).CurState = PlanetMenuItems::NAVIGATION.0 as ::core::ffi::c_int
                    as BYTE;
            }
            DrawMenuStateStrings(
                C2Rust_Unnamed::PM_SCAN.0 as ::core::ffi::c_int as BYTE,
                (*pMS).CurState as SWORD,
            );
        }
        SetFlashRect(
            ::core::ptr::with_exposed_provenance_mut::<
                RECT,
            >(!(0 as ::core::ffi::c_long) as usize),
        );
    }
    return BOOLEAN::TRUE;
}
unsafe extern "C" fn on_input_frame() {
    RotatePlanetSphere(BOOLEAN::TRUE);
}
#[no_mangle]
pub unsafe extern "C" fn PlanetOrbitMenu() {
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
    let mut oldCallback: Option<InputFrameCallback> = None;
    memset(
        &raw mut MenuState as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<MENU_STATE>(),
    );
    DrawMenuStateStrings(
        C2Rust_Unnamed::PM_SCAN.0 as ::core::ffi::c_int as BYTE,
        PlanetMenuItems::SCAN.0 as ::core::ffi::c_int as SWORD,
    );
    SetFlashRect(
        ::core::ptr::with_exposed_provenance_mut::<RECT>(!(0 as ::core::ffi::c_long) as usize),
    );
    MenuState.CurState = PlanetMenuItems::SCAN.0 as ::core::ffi::c_int as BYTE;
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
    oldCallback = SetInputCallback(Some(on_input_frame as unsafe extern "C" fn() -> ()));
    MenuState.InputFunc = Some(
        DoPlanetOrbit as unsafe extern "C" fn(*mut MENU_STATE) -> BOOLEAN,
    ) as Option<unsafe extern "C" fn(*mut menu_state) -> BOOLEAN>;
    DoInput(&raw mut MenuState as *mut ::core::ffi::c_void, BOOLEAN::TRUE);
    SetInputCallback(oldCallback);
    SetFlashRect(::core::ptr::null::<RECT>());
    DrawMenuStateStrings(
        C2Rust_Unnamed::PM_STARMAP.0 as ::core::ffi::c_int as BYTE,
        -(PlanetMenuItems::NAVIGATION.0 as ::core::ffi::c_int) as SWORD,
    );
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
