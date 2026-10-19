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

// Rust translation of src/uqm.c.
extern "C" {
    pub type _IO_FILE;
    pub type uio_DirHandle;
    static mut optarg: *mut ::core::ffi::c_char;
    static mut optind: ::core::ffi::c_int;
    static mut opterr: ::core::ffi::c_int;
    static mut optopt: ::core::ffi::c_int;
    fn getopt_long(
        __argc: ::core::ffi::c_int,
        __argv: *const *mut ::core::ffi::c_char,
        __shortopts: *const ::core::ffi::c_char,
        __longopts: *const option,
        __longind: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    static stdout: *mut FILE;
    static stderr: *mut FILE;
    fn freopen(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: *mut FILE,
    ) -> *mut FILE;
    fn printf(_: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn snprintf(
        _: *mut ::core::ffi::c_char,
        _: size_t,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn vsnprintf(
        _: *mut ::core::ffi::c_char,
        _: size_t,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::VaList,
    ) -> ::core::ffi::c_int;
    fn sscanf(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn strtod(
        _: *const ::core::ffi::c_char,
        _: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_double;
    fn strtol(
        _: *const ::core::ffi::c_char,
        _: *mut *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_long;
    fn mem_init() -> bool;
    fn mem_uninit() -> bool;
    fn HMalloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn HFree(p: *mut ::core::ffi::c_void);
    fn HRealloc(p: *mut ::core::ffi::c_void, size: size_t) -> *mut ::core::ffi::c_void;
    fn __assert_fail(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    ) -> !;
    fn LoadResourceIndex(
        dir: *mut uio_DirHandle,
        filename: *const ::core::ffi::c_char,
        prefix: *const ::core::ffi::c_char,
    );
    fn res_IsString(key: *const ::core::ffi::c_char) -> BOOLEAN;
    fn res_GetString(key: *const ::core::ffi::c_char) -> *const ::core::ffi::c_char;
    fn res_IsInteger(key: *const ::core::ffi::c_char) -> BOOLEAN;
    fn res_GetInteger(key: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn res_IsBoolean(key: *const ::core::ffi::c_char) -> BOOLEAN;
    fn res_GetBoolean(key: *const ::core::ffi::c_char) -> BOOLEAN;
    fn res_Remove(key: *const ::core::ffi::c_char) -> BOOLEAN;
    fn TFB_PreInit();
    fn TFB_InitGraphics(
        driver: ::core::ffi::c_int,
        flags: ::core::ffi::c_int,
        renderer: *const ::core::ffi::c_char,
        width: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
        resFactor: *mut ::core::ffi::c_uint,
        windowType: *mut ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
    fn TFB_UninitGraphics();
    fn TFB_ProcessEvents();
    fn TFB_FlushGraphics();
    fn TFB_PurgeDanglingGraphics();
    static mut CanvasWidth: ::core::ffi::c_int;
    static mut CanvasHeight: ::core::ffi::c_int;
    static mut SavedWidth: ::core::ffi::c_int;
    static mut SavedHeight: ::core::ffi::c_int;
    fn InitColorMaps();
    fn UninitColorMaps();
    static mut snddriver: ::core::ffi::c_int;
    static mut soundflags: ::core::ffi::c_int;
    fn unInitAudio();
    fn InitThreadSystem();
    fn UnInitThreadSystem();
    fn StartThread_Core(
        func: ThreadFunction,
        data: *mut ::core::ffi::c_void,
        stackSize: SDWORD,
        name: *const ::core::ffi::c_char,
    );
    fn HibernateThread(timePeriod: TimePeriod);
    fn ProcessThreadLifecycles();
    fn InitTimeSystem();
    fn UnInitTimeSystem();
    static mut musicVolumeScale: ::core::ffi::c_float;
    static mut sfxVolumeScale: ::core::ffi::c_float;
    static mut speechVolumeScale: ::core::ffi::c_float;
    fn TFB_InitInput(
        driver: ::core::ffi::c_int,
        flags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn TFB_UninitInput();
    fn TFB_SetInputVectors(
        menu: *mut ::core::ffi::c_int,
        num_menu: ::core::ffi::c_int,
        flight: *mut ::core::ffi::c_int,
        num_templ: ::core::ffi::c_int,
        num_flight: ::core::ffi::c_int,
    );
    fn memset(
        _: *mut ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strcmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strlen(_: *const ::core::ffi::c_char) -> size_t;
    static mut QuitPosted: ::core::ffi::c_int;
    static mut GameActive: ::core::ffi::c_int;
    fn InitTaskSystem();
    fn CleanupTaskSystem();
    fn luaUqm_init();
    fn luaUqm_uninit();
    fn Alarm_init();
    fn Alarm_uninit();
    static mut ImmediateInputState: CONTROLLER_INPUT_STATE;
    static mut PlayerControls: [CONTROL_TEMPLATE; 0];
    static mut optWhichCoarseScan: ::core::ffi::c_int;
    static mut optWhichMenu: ::core::ffi::c_int;
    static mut optWhichFonts: ::core::ffi::c_int;
    static mut optWhichIntro: ::core::ffi::c_int;
    static mut optWhichShield: ::core::ffi::c_int;
    static mut optSmoothScroll: ::core::ffi::c_int;
    static mut optMeleeScale: ::core::ffi::c_int;
    static mut loresBlowupScale: ::core::ffi::c_uint;
    static mut resolutionFactor: ::core::ffi::c_uint;
    static mut optRequiresReload: BOOLEAN;
    static mut optRequiresRestart: BOOLEAN;
    static mut optCheatMode: OPT_ENABLABLE;
    static mut optGodModes: ::core::ffi::c_int;
    static mut timeDilationScale: ::core::ffi::c_int;
    static mut optBubbleWarp: OPT_ENABLABLE;
    static mut optUnlockShips: OPT_ENABLABLE;
    static mut optHeadStart: OPT_ENABLABLE;
    static mut optAutoButtons: OPT_ENABLABLE;
    static mut optInfiniteRU: OPT_ENABLABLE;
    static mut optSkipIntro: OPT_ENABLABLE;
    static mut optMainMenuMusic: OPT_ENABLABLE;
    static mut optNebulae: OPT_ENABLABLE;
    static mut optOrbitingPlanets: OPT_ENABLABLE;
    static mut optTexturedPlanets: OPT_ENABLABLE;
    static mut optDateFormat: ::core::ffi::c_int;
    static mut optInfiniteFuel: OPT_ENABLABLE;
    static mut optPartialPickup: OPT_ENABLABLE;
    static mut optSubmenu: OPT_ENABLABLE;
    static mut optInfiniteCredits: OPT_ENABLABLE;
    static mut optSuperMelee: BOOLEAN;
    static mut optLoadGame: BOOLEAN;
    static mut optCustomBorder: OPT_ENABLABLE;
    static mut optSeedType: ::core::ffi::c_int;
    static mut optCustomSeed: ::core::ffi::c_int;
    static mut optShipSeed: OPT_ENABLABLE;
    static mut optSphereColors: ::core::ffi::c_int;
    static mut optSpaceMusic: ::core::ffi::c_int;
    static mut optVolasMusic: OPT_ENABLABLE;
    static mut optWholeFuel: OPT_ENABLABLE;
    static mut optDirJoy: [::core::ffi::c_int; 2];
    static mut optLanderHold: ::core::ffi::c_int;
    static mut optScrTrans: ::core::ffi::c_int;
    static mut optDifficulty: ::core::ffi::c_int;
    static mut optDiffChooser: ::core::ffi::c_int;
    static mut optFuelRange: ::core::ffi::c_int;
    static mut optExtended: OPT_ENABLABLE;
    static mut optNomad: ::core::ffi::c_int;
    static mut optGameOver: OPT_ENABLABLE;
    static mut optShipDirectionIP: OPT_ENABLABLE;
    static mut optHazardColors: OPT_ENABLABLE;
    static mut optOrzCompFont: OPT_ENABLABLE;
    static mut optControllerType: ::core::ffi::c_int;
    static mut optSmartAutoPilot: OPT_ENABLABLE;
    static mut optTintPlanSphere: ::core::ffi::c_int;
    static mut optPlanetStyle: ::core::ffi::c_int;
    static mut optStarBackground: ::core::ffi::c_int;
    static mut optScanStyle: ::core::ffi::c_int;
    static mut optNonStopOscill: OPT_ENABLABLE;
    static mut optScopeStyle: ::core::ffi::c_int;
    static mut optSuperPC: ::core::ffi::c_int;
    static mut optHyperStars: OPT_ENABLABLE;
    static mut optPlanetTexture: OPT_ENABLABLE;
    static mut optFlagshipColor: ::core::ffi::c_int;
    static mut optNoHQEncounters: OPT_ENABLABLE;
    static mut optDeCleansing: OPT_ENABLABLE;
    static mut optMeleeObstacles: OPT_ENABLABLE;
    static mut optShowVisitedStars: OPT_ENABLABLE;
    static mut optUnscaledStarSystem: OPT_ENABLABLE;
    static mut optScanSphere: ::core::ffi::c_int;
    static mut optNebulaeVolume: ::core::ffi::c_int;
    static mut optSlaughterMode: OPT_ENABLABLE;
    static mut optAdvancedAutoPilot: OPT_ENABLABLE;
    static mut optMeleeToolTips: OPT_ENABLABLE;
    static mut optMusicResume: ::core::ffi::c_int;
    static mut optWindowType: DWORD;
    static mut optNoClassic: BOOLEAN;
    static mut optScatterElements: OPT_ENABLABLE;
    static mut optShowUpgrades: OPT_ENABLABLE;
    static mut optFleetPointSys: OPT_ENABLABLE;
    static mut optShipStore: OPT_ENABLABLE;
    static mut optCaptainNames: OPT_ENABLABLE;
    static mut optDosMenus: OPT_ENABLABLE;
    static mut optDeviceArray: [OPT_ADD_REMOVE; 28];
    static mut optUpgradeArray: [OPT_ADD_REMOVE; 13];
    static mut optHyperSpaceColor: ::core::ffi::c_int;
    static mut DeadZoneLeftStick: [::core::ffi::c_int; 2];
    static mut DeadZoneRightStick: [::core::ffi::c_int; 2];
    static mut opt3doMusic: OPT_ENABLABLE;
    static mut optRemixMusic: OPT_ENABLABLE;
    static mut optSpeech: OPT_ENABLABLE;
    static mut optSubtitles: OPT_ENABLABLE;
    static mut optStereoSFX: OPT_ENABLABLE;
    static mut optKeepAspectRatio: OPT_ENABLABLE;
    static mut optGamma: ::core::ffi::c_float;
    static mut configDir: *mut uio_DirHandle;
    static mut optAddons: *mut *const ::core::ffi::c_char;
    fn prepareContentDir(
        contentDirName: *const ::core::ffi::c_char,
        addonDirName: *const ::core::ffi::c_char,
        execFile: *const ::core::ffi::c_char,
    );
    fn prepareConfigDir(configDirName: *const ::core::ffi::c_char);
    fn prepareMeleeDir();
    fn prepareSaveDir();
    fn prepareScrShotDir();
    fn prepareShadowAddons(addons: *mut *const ::core::ffi::c_char);
    fn unprepareAllDirs();
    fn isAddonAvailable(addon_name: *const ::core::ffi::c_char) -> BOOLEAN;
    fn setGammaCorrection(gamma: ::core::ffi::c_float) -> bool;
    static mut force_platform: PLATFORM_TYPE;
    fn log_init(max_lines: ::core::ffi::c_int);
    fn log_initThreads();
    fn log_setOutput(out: *mut FILE) -> *mut FILE;
    fn log_showBox(show: bool, err: bool);
    fn log_captureLines(num: ::core::ffi::c_int);
    fn log_add(_: log_Level, fmt: *const ::core::ffi::c_char, ...);
    fn Callback_init();
    fn Callback_uninit();
    fn init_communication();
    fn uninit_communication();
    fn initIO() -> ::core::ffi::c_int;
    fn uninitIO();
    static mut MainExited: ::core::ffi::c_int;
    fn SignalStopMainThread();
    fn ProcessUtilityKeys();
    fn Starcon2Main(threadArg: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
}
pub type __builtin_va_list = *mut ::core::ffi::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const ::core::ffi::c_char,
    pub has_arg: ::core::ffi::c_int,
    pub flag: *mut ::core::ffi::c_int,
    pub val: ::core::ffi::c_int,
}
pub type va_list = __builtin_va_list;
pub type size_t = usize;
pub type __isoc_va_list = __builtin_va_list;
pub type FILE = _IO_FILE;
pub type uint32 = ::core::ffi::c_uint;
pub type sint32 = ::core::ffi::c_int;
pub type DWORD = uint32;
pub type SDWORD = sint32;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct BOOLEAN(pub ::core::ffi::c_uint);
impl BOOLEAN {
    pub const FALSE: Self = Self(0);
    pub const TRUE: Self = Self(1);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed {
    pub const TFB_GFXDRIVER_SDL_OPENGL: Self = Self(0);
    pub const TFB_GFXDRIVER_SDL_PURE: Self = Self(1);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_0 {
    pub const TFB_SCALE_STEP: Self = Self(0);
    pub const TFB_SCALE_NEAREST: Self = Self(1);
    pub const TFB_SCALE_BILINEAR: Self = Self(2);
    pub const TFB_SCALE_TRILINEAR: Self = Self(3);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_1(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_1 {
    pub const audio_DRIVER_MIXSDL: Self = Self(0);
    pub const audio_DRIVER_NOSOUND: Self = Self(1);
    pub const audio_DRIVER_OPENAL: Self = Self(2);
}
pub type TimePeriod = DWORD;
pub type ThreadFunction = Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_2(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_2 {
    pub const TFB_INPUTDRIVER_SDL: Self = Self(0);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_3(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_3 {
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
pub struct C2Rust_Unnamed_4(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_4 {
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct OPT_ENABLABLE(pub ::core::ffi::c_uint);
impl OPT_ENABLABLE {
    pub const OPTVAL_DISABLED: Self = Self(0);
    pub const OPTVAL_ENABLED: Self = Self(1);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_5(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_5 {
    pub const OPTVAL_PC_WINDOW: Self = Self(0);
    pub const OPTVAL_3DO_WINDOW: Self = Self(1);
    pub const OPTVAL_UQM_WINDOW: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct OPT_ADD_REMOVE(pub ::core::ffi::c_uint);
impl OPT_ADD_REMOVE {
    pub const OPTVAL_DO_NOTHING: Self = Self(0);
    pub const OPTVAL_REM_THING: Self = Self(1);
    pub const OPTVAL_ADD_THING: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct PLATFORM_TYPE(pub ::core::ffi::c_uint);
impl PLATFORM_TYPE {
    pub const PLATFORM_NULL: Self = Self(0);
    pub const PLATFORM_C: Self = Self(1);
    pub const PLATFORM_MMX: Self = Self(2);
    pub const PLATFORM_SSE: Self = Self(3);
    pub const PLATFORM_3DNOW: Self = Self(4);
    pub const PLATFORM_ALTIVEC: Self = Self(5);
    pub const PLATFORM_LAST: Self = Self(5);
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bool_option {
    pub value: bool,
    pub set: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct int_option {
    pub value: ::core::ffi::c_int,
    pub set: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct float_option {
    pub value: ::core::ffi::c_float,
    pub set: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct options_struct {
    pub logFile: *const ::core::ffi::c_char,
    pub runMode: C2Rust_Unnamed_7,
    pub configDir: *const ::core::ffi::c_char,
    pub contentDir: *const ::core::ffi::c_char,
    pub addonDir: *const ::core::ffi::c_char,
    pub addons: *mut *const ::core::ffi::c_char,
    pub numAddons: ::core::ffi::c_int,
    pub graphicsBackend: *const ::core::ffi::c_char,
    pub opengl: bool_option,
    pub resolution: C2Rust_Unnamed_6,
    pub fullscreen: int_option,
    pub scanlines: bool_option,
    pub scaler: int_option,
    pub showFps: bool_option,
    pub keepAspectRatio: bool_option,
    pub gamma: float_option,
    pub soundDriver: int_option,
    pub soundQuality: int_option,
    pub use3doMusic: bool_option,
    pub useRemixMusic: bool_option,
    pub useSpeech: bool_option,
    pub whichCoarseScan: int_option,
    pub whichMenu: int_option,
    pub whichFonts: int_option,
    pub whichIntro: int_option,
    pub whichShield: int_option,
    pub smoothScroll: int_option,
    pub meleeScale: int_option,
    pub subtitles: bool_option,
    pub stereoSFX: bool_option,
    pub musicVolumeScale: float_option,
    pub sfxVolumeScale: float_option,
    pub speechVolumeScale: float_option,
    pub safeMode: bool_option,
    pub resolutionFactor: int_option,
    pub loresBlowupScale: int_option,
    pub cheatMode: bool_option,
    pub optGodModes: int_option,
    pub timeDilationScale: int_option,
    pub bubbleWarp: bool_option,
    pub unlockShips: bool_option,
    pub headStart: bool_option,
    pub autoButtons: bool_option,
    pub infiniteRU: bool_option,
    pub skipIntro: bool_option,
    pub mainMenuMusic: bool_option,
    pub nebulae: bool_option,
    pub orbitingPlanets: bool_option,
    pub texturedPlanets: bool_option,
    pub optDateFormat: int_option,
    pub infiniteFuel: bool_option,
    pub partialPickup: bool_option,
    pub submenu: bool_option,
    pub infiniteCredits: bool_option,
    pub customBorder: bool_option,
    pub seedType: int_option,
    pub customSeed: int_option,
    pub shipSeed: bool_option,
    pub sphereColors: int_option,
    pub spaceMusic: int_option,
    pub volasMusic: bool_option,
    pub wholeFuel: bool_option,
    pub dirJoyP1: int_option,
    pub landerHold: int_option,
    pub scrTrans: int_option,
    pub optDifficulty: int_option,
    pub optDiffChooser: int_option,
    pub optFuelRange: int_option,
    pub extended: bool_option,
    pub nomad: int_option,
    pub gameOver: bool_option,
    pub shipDirectionIP: bool_option,
    pub hazardColors: bool_option,
    pub orzCompFont: bool_option,
    pub optControllerType: int_option,
    pub smartAutoPilot: bool_option,
    pub tintPlanSphere: int_option,
    pub planetStyle: int_option,
    pub starBackground: int_option,
    pub scanStyle: int_option,
    pub nonStopOscill: bool_option,
    pub scopeStyle: int_option,
    pub hyperStars: bool_option,
    pub landerStyle: int_option,
    pub planetTexture: bool_option,
    pub flagshipColor: int_option,
    pub noHQEncounters: bool_option,
    pub deCleansing: bool_option,
    pub meleeObstacles: bool_option,
    pub showVisitedStars: bool_option,
    pub unscaledStarSystem: bool_option,
    pub sphereType: int_option,
    pub nebulaevol: int_option,
    pub slaughterMode: bool_option,
    pub advancedAutoPilot: bool_option,
    pub meleeToolTips: bool_option,
    pub musicResume: int_option,
    pub windowType: int_option,
    pub scatterElements: bool_option,
    pub showUpgrades: bool_option,
    pub fleetPointSys: bool_option,
    pub shipStore: bool_option,
    pub captainNames: bool_option,
    pub dosMenus: bool_option,
    pub hyperSpaceColor: int_option,
    pub deadZoneLeftP1: int_option,
    pub deadZoneRightP1: int_option,
    pub deadZoneLeftP2: int_option,
    pub deadZoneRightP2: int_option,
    pub dirJoyP2: int_option,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_6 {
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub set: bool,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_7(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_7 {
    pub const runMode_normal: Self = Self(0);
    pub const runMode_usage: Self = Self(1);
    pub const runMode_version: Self = Self(2);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option_list_value {
    pub str: *const ::core::ffi::c_char,
    pub value: ::core::ffi::c_int,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_8(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_8 {
    pub const CSCAN_OPT: Self = Self(1000);
    pub const MENU_OPT: Self = Self(1001);
    pub const FONT_OPT: Self = Self(1002);
    pub const SHIELD_OPT: Self = Self(1003);
    pub const SCROLL_OPT: Self = Self(1004);
    pub const SOUND_OPT: Self = Self(1005);
    pub const STEREOSFX_OPT: Self = Self(1006);
    pub const ADDON_OPT: Self = Self(1007);
    pub const ADDONDIR_OPT: Self = Self(1008);
    pub const ACCEL_OPT: Self = Self(1009);
    pub const SAFEMODE_OPT: Self = Self(1010);
    pub const RENDERER_OPT: Self = Self(1011);
    pub const CHEATMODE_OPT: Self = Self(1012);
    pub const GODMODE_OPT: Self = Self(1013);
    pub const TDM_OPT: Self = Self(1014);
    pub const BWARP_OPT: Self = Self(1015);
    pub const UNLOCKSHIPS_OPT: Self = Self(1016);
    pub const HEADSTART_OPT: Self = Self(1017);
    pub const AUTOBUTT_OPT: Self = Self(1018);
    pub const INFINITERU_OPT: Self = Self(1019);
    pub const SKIPINTRO_OPT: Self = Self(1020);
    pub const MENUMUS_OPT: Self = Self(1021);
    pub const NEBU_OPT: Self = Self(1022);
    pub const ORBITS_OPT: Self = Self(1023);
    pub const TEXTPLAN_OPT: Self = Self(1024);
    pub const DATE_OPT: Self = Self(1025);
    pub const INFFUEL_OPT: Self = Self(1026);
    pub const PICKUP_OPT: Self = Self(1027);
    pub const SUBMENU_OPT: Self = Self(1028);
    pub const INFCRED_OPT: Self = Self(1029);
    pub const CUSTBORD_OPT: Self = Self(1030);
    pub const SEEDTYPE_OPT: Self = Self(1031);
    pub const EXSEED_OPT: Self = Self(1032);
    pub const SHIPSEED_OPT: Self = Self(1033);
    pub const SPHERECOLORS_OPT: Self = Self(1034);
    pub const SPACEMUSIC_OPT: Self = Self(1035);
    pub const WHOLEFUEL_OPT: Self = Self(1036);
    pub const DIRJOY_OPT: Self = Self(1037);
    pub const LANDHOLD_OPT: Self = Self(1038);
    pub const SCRTRANS_OPT: Self = Self(1039);
    pub const DIFFICULTY_OPT: Self = Self(1040);
    pub const FUELRANGE_OPT: Self = Self(1041);
    pub const EXTENDED_OPT: Self = Self(1042);
    pub const NOMAD_OPT: Self = Self(1043);
    pub const GAMEOVER_OPT: Self = Self(1044);
    pub const SHIPDIRIP_OPT: Self = Self(1045);
    pub const HAZCOLORS_OPT: Self = Self(1046);
    pub const ORZFONT_OPT: Self = Self(1047);
    pub const CONTYPE_OPT: Self = Self(1048);
    pub const SISFACEHS_OPT: Self = Self(1049);
    pub const COLORPLAN_OPT: Self = Self(1050);
    pub const PLANSTYLE_OPT: Self = Self(1051);
    pub const STARBACK_OPT: Self = Self(1052);
    pub const SCANSTYLE_OPT: Self = Self(1053);
    pub const OSCILLO_OPT: Self = Self(1054);
    pub const OSCSTYLE_OPT: Self = Self(1055);
    pub const HYPERSTARS_OPT: Self = Self(1056);
    pub const LANDSTYLE_OPT: Self = Self(1057);
    pub const PLANTEX_OPT: Self = Self(1058);
    pub const SISENGINE_OPT: Self = Self(1059);
    pub const NOHSENC_OPT: Self = Self(1060);
    pub const DECLEANSE_OPT: Self = Self(1061);
    pub const NOMELEEOBJ_OPT: Self = Self(1062);
    pub const SHOWSTARS_OPT: Self = Self(1063);
    pub const UNSCALEDSS_OPT: Self = Self(1064);
    pub const SCANSPH_OPT: Self = Self(1065);
    pub const SLAUGHTER_OPT: Self = Self(1066);
    pub const SISADVAP_OPT: Self = Self(1067);
    pub const MELEETIPS_OPT: Self = Self(1068);
    pub const MUSICRESUME_OPT: Self = Self(1069);
    pub const WINDTYPE_OPT: Self = Self(1070);
    pub const SCATTERELEMS_OPT: Self = Self(1071);
    pub const SHOWUPG_OPT: Self = Self(1072);
    pub const FLTPTSYS_OPT: Self = Self(1073);
    pub const SHIPSTORE_OPT: Self = Self(1074);
    pub const CAPTNAMES_OPT: Self = Self(1075);
    pub const DOSMENUS_OPT: Self = Self(1076);
    pub const MELEE_OPT: Self = Self(1077);
    pub const LOADGAME_OPT: Self = Self(1078);
    pub const NEBUVOL_OPT: Self = Self(1079);
    pub const CLAPAK_OPT: Self = Self(1080);
    pub const HSCOLOR_OPT: Self = Self(1081);
    pub const DZLP1_OPT: Self = Self(1082);
    pub const DZRP1_OPT: Self = Self(1083);
    pub const DZLP2_OPT: Self = Self(1084);
    pub const DZRP2_OPT: Self = Self(1085);
}
#[no_mangle]
pub static mut restartGame: BOOLEAN = BOOLEAN::FALSE;
static mut scalerList: [option_list_value; 8] = [
    option_list_value {
        str: b"bilinear\0".as_ptr() as *const ::core::ffi::c_char,
        value: (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int,
    },
    option_list_value {
        str: b"biadapt\0".as_ptr() as *const ::core::ffi::c_char,
        value: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int,
    },
    option_list_value {
        str: b"biadv\0".as_ptr() as *const ::core::ffi::c_char,
        value: (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int,
    },
    option_list_value {
        str: b"triscan\0".as_ptr() as *const ::core::ffi::c_char,
        value: (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int,
    },
    option_list_value {
        str: b"hq\0".as_ptr() as *const ::core::ffi::c_char,
        value: (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int,
    },
    option_list_value {
        str: b"none\0".as_ptr() as *const ::core::ffi::c_char,
        value: 0 as ::core::ffi::c_int,
    },
    option_list_value {
        str: b"no\0".as_ptr() as *const ::core::ffi::c_char,
        value: 0 as ::core::ffi::c_int,
    },
    option_list_value {
        str: ::core::ptr::null::<::core::ffi::c_char>(),
        value: 0 as ::core::ffi::c_int,
    },
];
static mut meleeScaleList: [option_list_value; 7] = [
    option_list_value {
        str: b"smooth\0".as_ptr() as *const ::core::ffi::c_char,
        value: C2Rust_Unnamed_0::TFB_SCALE_TRILINEAR.0 as ::core::ffi::c_int,
    },
    option_list_value {
        str: b"3do\0".as_ptr() as *const ::core::ffi::c_char,
        value: C2Rust_Unnamed_0::TFB_SCALE_TRILINEAR.0 as ::core::ffi::c_int,
    },
    option_list_value {
        str: b"step\0".as_ptr() as *const ::core::ffi::c_char,
        value: C2Rust_Unnamed_0::TFB_SCALE_STEP.0 as ::core::ffi::c_int,
    },
    option_list_value {
        str: b"pc\0".as_ptr() as *const ::core::ffi::c_char,
        value: C2Rust_Unnamed_0::TFB_SCALE_STEP.0 as ::core::ffi::c_int,
    },
    option_list_value {
        str: b"bilinear\0".as_ptr() as *const ::core::ffi::c_char,
        value: C2Rust_Unnamed_0::TFB_SCALE_BILINEAR.0 as ::core::ffi::c_int,
    },
    option_list_value {
        str: b"nearest\0".as_ptr() as *const ::core::ffi::c_char,
        value: C2Rust_Unnamed_0::TFB_SCALE_NEAREST.0 as ::core::ffi::c_int,
    },
    option_list_value {
        str: ::core::ptr::null::<::core::ffi::c_char>(),
        value: 0 as ::core::ffi::c_int,
    },
];
static mut audioDriverList: [option_list_value; 5] = [
    option_list_value {
        str: b"openal\0".as_ptr() as *const ::core::ffi::c_char,
        value: C2Rust_Unnamed_1::audio_DRIVER_OPENAL.0 as ::core::ffi::c_int,
    },
    option_list_value {
        str: b"mixsdl\0".as_ptr() as *const ::core::ffi::c_char,
        value: C2Rust_Unnamed_1::audio_DRIVER_MIXSDL.0 as ::core::ffi::c_int,
    },
    option_list_value {
        str: b"none\0".as_ptr() as *const ::core::ffi::c_char,
        value: C2Rust_Unnamed_1::audio_DRIVER_NOSOUND.0 as ::core::ffi::c_int,
    },
    option_list_value {
        str: b"nosound\0".as_ptr() as *const ::core::ffi::c_char,
        value: C2Rust_Unnamed_1::audio_DRIVER_NOSOUND.0 as ::core::ffi::c_int,
    },
    option_list_value {
        str: ::core::ptr::null::<::core::ffi::c_char>(),
        value: 0 as ::core::ffi::c_int,
    },
];
static mut audioQualityList: [option_list_value; 4] = [
    option_list_value {
        str: b"low\0".as_ptr() as *const ::core::ffi::c_char,
        value: (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int,
    },
    option_list_value {
        str: b"medium\0".as_ptr() as *const ::core::ffi::c_char,
        value: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
    },
    option_list_value {
        str: b"high\0".as_ptr() as *const ::core::ffi::c_char,
        value: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
    },
    option_list_value {
        str: ::core::ptr::null::<::core::ffi::c_char>(),
        value: 0 as ::core::ffi::c_int,
    },
];
static mut choiceList: [option_list_value; 3] = [
    option_list_value {
        str: b"pc\0".as_ptr() as *const ::core::ffi::c_char,
        value: 0x2 as ::core::ffi::c_int,
    },
    option_list_value {
        str: b"3do\0".as_ptr() as *const ::core::ffi::c_char,
        value: 0x1 as ::core::ffi::c_int,
    },
    option_list_value {
        str: ::core::ptr::null::<::core::ffi::c_char>(),
        value: 0 as ::core::ffi::c_int,
    },
];
static mut accelList: [option_list_value; 6] = [
    option_list_value {
        str: b"mmx\0".as_ptr() as *const ::core::ffi::c_char,
        value: PLATFORM_TYPE::PLATFORM_MMX.0 as ::core::ffi::c_int,
    },
    option_list_value {
        str: b"sse\0".as_ptr() as *const ::core::ffi::c_char,
        value: PLATFORM_TYPE::PLATFORM_SSE.0 as ::core::ffi::c_int,
    },
    option_list_value {
        str: b"3dnow\0".as_ptr() as *const ::core::ffi::c_char,
        value: PLATFORM_TYPE::PLATFORM_3DNOW.0 as ::core::ffi::c_int,
    },
    option_list_value {
        str: b"none\0".as_ptr() as *const ::core::ffi::c_char,
        value: PLATFORM_TYPE::PLATFORM_C.0 as ::core::ffi::c_int,
    },
    option_list_value {
        str: b"detect\0".as_ptr() as *const ::core::ffi::c_char,
        value: PLATFORM_TYPE::PLATFORM_NULL.0 as ::core::ffi::c_int,
    },
    option_list_value {
        str: ::core::ptr::null::<::core::ffi::c_char>(),
        value: 0 as ::core::ffi::c_int,
    },
];
static mut errBuffer: [::core::ffi::c_char; 512] = [0; 512];
#[export_name = "uqm_rust_main"]
pub unsafe extern "C" fn main_0(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    crate::uqm_engine_init::uqm_rust_engine_init();
    let mut options: options_struct = options_struct {
        logFile: ::core::ptr::null::<::core::ffi::c_char>(),
        runMode: C2Rust_Unnamed_7::runMode_normal,
        configDir: ::core::ptr::null::<::core::ffi::c_char>(),
        contentDir: ::core::ptr::null::<::core::ffi::c_char>(),
        addonDir: ::core::ptr::null::<::core::ffi::c_char>(),
        addons: ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        numAddons: 0 as ::core::ffi::c_int,
        graphicsBackend: ::core::ptr::null::<::core::ffi::c_char>(),
        opengl: bool_option {
            value: false,
            set: false,
        },
        resolution: C2Rust_Unnamed_6 {
            width: 640 as ::core::ffi::c_int,
            height: 480 as ::core::ffi::c_int,
            set: false,
        },
        fullscreen: int_option {
            value: 2 as ::core::ffi::c_int,
            set: false,
        },
        scanlines: bool_option {
            value: false,
            set: false,
        },
        scaler: int_option {
            value: 0 as ::core::ffi::c_int,
            set: false,
        },
        showFps: bool_option {
            value: false,
            set: false,
        },
        keepAspectRatio: bool_option {
            value: false,
            set: false,
        },
        gamma: float_option {
            value: 1.0f32,
            set: false,
        },
        soundDriver: int_option {
            value: C2Rust_Unnamed_1::audio_DRIVER_OPENAL.0 as ::core::ffi::c_int,
            set: false,
        },
        soundQuality: int_option {
            value: (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            set: false,
        },
        use3doMusic: bool_option {
            value: true,
            set: false,
        },
        useRemixMusic: bool_option {
            value: false,
            set: false,
        },
        useSpeech: bool_option {
            value: true,
            set: false,
        },
        whichCoarseScan: int_option {
            value: 0 as ::core::ffi::c_int,
            set: false,
        },
        whichMenu: int_option {
            value: 0x2 as ::core::ffi::c_int,
            set: false,
        },
        whichFonts: int_option {
            value: 0x2 as ::core::ffi::c_int,
            set: false,
        },
        whichIntro: int_option {
            value: 0x2 as ::core::ffi::c_int,
            set: false,
        },
        whichShield: int_option {
            value: 0x2 as ::core::ffi::c_int,
            set: false,
        },
        smoothScroll: int_option {
            value: 0x2 as ::core::ffi::c_int,
            set: false,
        },
        meleeScale: int_option {
            value: C2Rust_Unnamed_0::TFB_SCALE_TRILINEAR.0 as ::core::ffi::c_int,
            set: false,
        },
        subtitles: bool_option {
            value: true,
            set: false,
        },
        stereoSFX: bool_option {
            value: false,
            set: false,
        },
        musicVolumeScale: float_option {
            value: 1.0f32,
            set: false,
        },
        sfxVolumeScale: float_option {
            value: 1.0f32,
            set: false,
        },
        speechVolumeScale: float_option {
            value: 1.0f32,
            set: false,
        },
        safeMode: bool_option {
            value: false,
            set: false,
        },
        resolutionFactor: int_option {
            value: 0 as ::core::ffi::c_int,
            set: false,
        },
        loresBlowupScale: int_option {
            value: 1 as ::core::ffi::c_int,
            set: false,
        },
        cheatMode: bool_option {
            value: false,
            set: false,
        },
        optGodModes: int_option {
            value: 0 as ::core::ffi::c_int,
            set: false,
        },
        timeDilationScale: int_option {
            value: 0 as ::core::ffi::c_int,
            set: false,
        },
        bubbleWarp: bool_option {
            value: false,
            set: false,
        },
        unlockShips: bool_option {
            value: false,
            set: false,
        },
        headStart: bool_option {
            value: false,
            set: false,
        },
        autoButtons: bool_option {
            value: true,
            set: false,
        },
        infiniteRU: bool_option {
            value: false,
            set: false,
        },
        skipIntro: bool_option {
            value: false,
            set: false,
        },
        mainMenuMusic: bool_option {
            value: true,
            set: false,
        },
        nebulae: bool_option {
            value: false,
            set: false,
        },
        orbitingPlanets: bool_option {
            value: false,
            set: false,
        },
        texturedPlanets: bool_option {
            value: false,
            set: false,
        },
        optDateFormat: int_option {
            value: 0 as ::core::ffi::c_int,
            set: false,
        },
        infiniteFuel: bool_option {
            value: false,
            set: false,
        },
        partialPickup: bool_option {
            value: false,
            set: false,
        },
        submenu: bool_option {
            value: false,
            set: false,
        },
        infiniteCredits: bool_option {
            value: false,
            set: false,
        },
        customBorder: bool_option {
            value: false,
            set: false,
        },
        seedType: int_option {
            value: 0 as ::core::ffi::c_int,
            set: false,
        },
        customSeed: int_option {
            value: 16807 as ::core::ffi::c_int,
            set: false,
        },
        shipSeed: bool_option {
            value: false,
            set: false,
        },
        sphereColors: int_option {
            value: 0 as ::core::ffi::c_int,
            set: false,
        },
        spaceMusic: int_option {
            value: 0 as ::core::ffi::c_int,
            set: false,
        },
        volasMusic: bool_option {
            value: false,
            set: false,
        },
        wholeFuel: bool_option {
            value: false,
            set: false,
        },
        dirJoyP1: int_option {
            value: 0 as ::core::ffi::c_int,
            set: false,
        },
        landerHold: int_option {
            value: 0x1 as ::core::ffi::c_int,
            set: false,
        },
        scrTrans: int_option {
            value: 0x1 as ::core::ffi::c_int,
            set: false,
        },
        optDifficulty: int_option {
            value: 0 as ::core::ffi::c_int,
            set: false,
        },
        optDiffChooser: int_option {
            value: 3 as ::core::ffi::c_int,
            set: false,
        },
        optFuelRange: int_option {
            value: 0 as ::core::ffi::c_int,
            set: false,
        },
        extended: bool_option {
            value: false,
            set: false,
        },
        nomad: int_option {
            value: 0 as ::core::ffi::c_int,
            set: false,
        },
        gameOver: bool_option {
            value: false,
            set: false,
        },
        shipDirectionIP: bool_option {
            value: false,
            set: false,
        },
        hazardColors: bool_option {
            value: false,
            set: false,
        },
        orzCompFont: bool_option {
            value: false,
            set: false,
        },
        optControllerType: int_option {
            value: 0 as ::core::ffi::c_int,
            set: false,
        },
        smartAutoPilot: bool_option {
            value: false,
            set: false,
        },
        tintPlanSphere: int_option {
            value: 0x1 as ::core::ffi::c_int,
            set: false,
        },
        planetStyle: int_option {
            value: 0x1 as ::core::ffi::c_int,
            set: false,
        },
        starBackground: int_option {
            value: 0 as ::core::ffi::c_int,
            set: false,
        },
        scanStyle: int_option {
            value: 0x1 as ::core::ffi::c_int,
            set: false,
        },
        nonStopOscill: bool_option {
            value: false,
            set: false,
        },
        scopeStyle: int_option {
            value: 0x1 as ::core::ffi::c_int,
            set: false,
        },
        hyperStars: bool_option {
            value: false,
            set: false,
        },
        landerStyle: int_option {
            value: 0x1 as ::core::ffi::c_int,
            set: false,
        },
        planetTexture: bool_option {
            value: true,
            set: false,
        },
        flagshipColor: int_option {
            value: 0x2 as ::core::ffi::c_int,
            set: false,
        },
        noHQEncounters: bool_option {
            value: false,
            set: false,
        },
        deCleansing: bool_option {
            value: false,
            set: false,
        },
        meleeObstacles: bool_option {
            value: false,
            set: false,
        },
        showVisitedStars: bool_option {
            value: false,
            set: false,
        },
        unscaledStarSystem: bool_option {
            value: false,
            set: false,
        },
        sphereType: int_option {
            value: 2 as ::core::ffi::c_int,
            set: false,
        },
        nebulaevol: int_option {
            value: 16 as ::core::ffi::c_int,
            set: false,
        },
        slaughterMode: bool_option {
            value: false,
            set: false,
        },
        advancedAutoPilot: bool_option {
            value: false,
            set: false,
        },
        meleeToolTips: bool_option {
            value: false,
            set: false,
        },
        musicResume: int_option {
            value: 0 as ::core::ffi::c_int,
            set: false,
        },
        windowType: int_option {
            value: 2 as ::core::ffi::c_int,
            set: false,
        },
        scatterElements: bool_option {
            value: false,
            set: false,
        },
        showUpgrades: bool_option {
            value: false,
            set: false,
        },
        fleetPointSys: bool_option {
            value: false,
            set: false,
        },
        shipStore: bool_option {
            value: false,
            set: false,
        },
        captainNames: bool_option {
            value: false,
            set: false,
        },
        dosMenus: bool_option {
            value: false,
            set: false,
        },
        hyperSpaceColor: int_option {
            value: 0x1 as ::core::ffi::c_int,
            set: false,
        },
        deadZoneLeftP1: int_option {
            value: 6881 as ::core::ffi::c_int,
            set: false,
        },
        deadZoneRightP1: int_option {
            value: 6881 as ::core::ffi::c_int,
            set: false,
        },
        deadZoneLeftP2: int_option {
            value: 6881 as ::core::ffi::c_int,
            set: false,
        },
        deadZoneRightP2: int_option {
            value: 6881 as ::core::ffi::c_int,
            set: false,
        },
        dirJoyP2: int_option {
            value: 0 as ::core::ffi::c_int,
            set: false,
        },
    };
    let mut defaults: options_struct = options;
    let mut optionsResult: ::core::ffi::c_int = 0;
    let mut gfxDriver: ::core::ffi::c_int = 0;
    let mut gfxFlags: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    optionsResult = parseOptions(argc, argv, &raw mut options);
    log_init(15 as ::core::ffi::c_int);
    if !options.logFile.is_null() {
        let mut i_0: ::core::ffi::c_int = 0;
        if freopen(
                options.logFile,
                b"w\0".as_ptr() as *const ::core::ffi::c_char,
                stderr,
            )
            .is_null()
        {
            printf(
                b"Error %d calling freopen() on stderr\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                *__errno_location(),
            );
            return 1 as ::core::ffi::c_int;
        }
        i_0 = 0 as ::core::ffi::c_int;
        while i_0 < argc {
            log_add(
                log_Level::log_User,
                b"argv[%d] = [%s]\0".as_ptr() as *const ::core::ffi::c_char,
                i_0,
                *argv.offset(i_0 as isize),
            );
            i_0 += 1;
        }
    }
    if options.runMode.0 == C2Rust_Unnamed_7::runMode_version.0 {
        printf(
            b"%d.%d.%d %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
            8 as ::core::ffi::c_int,
            5 as ::core::ffi::c_int,
            if resolutionFactor != 0 {
                b"HD MegaMod Debug\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"MegaMod Debug\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
        log_showBox(false, false);
        return 0 as ::core::ffi::c_int;
    }
    log_add(
        log_Level::log_User,
        b"The Ur-Quan Masters v%d.%d.%d %s (compiled %s %s)\nThis software comes with ABSOLUTELY NO WARRANTY;\nfor details see the included 'COPYING' file.\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        0 as ::core::ffi::c_int,
        8 as ::core::ffi::c_int,
        5 as ::core::ffi::c_int,
        if resolutionFactor != 0 {
            b"HD MegaMod Debug\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"MegaMod Debug\0".as_ptr() as *const ::core::ffi::c_char
        },
        b"Oct 19 2026\0".as_ptr() as *const ::core::ffi::c_char,
        b"19:17:44\0".as_ptr() as *const ::core::ffi::c_char,
    );
    printf(
        b"GCC_VERSION: %d.%d.%d\n\n\0".as_ptr() as *const ::core::ffi::c_char,
        4 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
    );
    log_add(
        log_Level::log_Info,
        b"GCC_VERSION: %d.%d.%d\n\0".as_ptr() as *const ::core::ffi::c_char,
        4 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
    );
    printf(
        b"CLANG_VERSION: %d.%d.%d\n\n\0".as_ptr() as *const ::core::ffi::c_char,
        18 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        8 as ::core::ffi::c_int,
    );
    log_add(
        log_Level::log_Info,
        b"CLANG_VERSION: %d.%d.%d\n\0".as_ptr() as *const ::core::ffi::c_char,
        18 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        8 as ::core::ffi::c_int,
    );
    printf(
        b"Build Time: %s %s\n\n\0".as_ptr() as *const ::core::ffi::c_char,
        b"Oct 19 2026\0".as_ptr() as *const ::core::ffi::c_char,
        b"19:17:44\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if errBuffer[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        log_add(
            log_Level::log_Error,
            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut errBuffer as *mut ::core::ffi::c_char,
        );
        errBuffer[0usize] = '\0' as ::core::ffi::c_char;
    }
    if options.runMode.0 == C2Rust_Unnamed_7::runMode_usage.0 {
        usage(stdout, &raw mut defaults);
        log_showBox(true, false);
        return 0 as ::core::ffi::c_int;
    }
    if optionsResult != 0 as ::core::ffi::c_int {
        log_add(
            log_Level::log_Fatal,
            b"Run with -h to see the allowed arguments.\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return optionsResult;
    }
    TFB_PreInit();
    mem_init();
    InitThreadSystem();
    log_initThreads();
    uqm_lua_checked!(initIO(), 0);
    uqm_lua_checked!(prepareConfigDir(options.configDir), 0);
    *(&raw mut PlayerControls as *mut CONTROL_TEMPLATE).offset(0isize) = CONTROL_TEMPLATE::CONTROL_TEMPLATE_PL_1;
    *(&raw mut PlayerControls as *mut CONTROL_TEMPLATE).offset(1isize) = CONTROL_TEMPLATE::CONTROL_TEMPLATE_PL_2;
    if !options.safeMode.value {
        uqm_lua_checked!(LoadResourceIndex(
            configDir,
            b"uqm.cfg\0".as_ptr() as *const ::core::ffi::c_char,
            b"config.\0".as_ptr() as *const ::core::ffi::c_char,
        ), 0);
        uqm_lua_checked!(LoadResourceIndex(
            configDir,
            b"cheats.cfg\0".as_ptr() as *const ::core::ffi::c_char,
            b"cheat.\0".as_ptr() as *const ::core::ffi::c_char,
        ), 0);
        uqm_lua_checked!(LoadResourceIndex(
            configDir,
            b"megamod.cfg\0".as_ptr() as *const ::core::ffi::c_char,
            b"mm.\0".as_ptr() as *const ::core::ffi::c_char,
        ), 0);
        getUserConfigOptions(&raw mut options);
    }
    let mut i_1: ::core::ffi::c_int = 0;
    i_1 = 0 as ::core::ffi::c_int;
    while i_1 < 6 as ::core::ffi::c_int {
        let mut cfgkey: [::core::ffi::c_char; 64] = [0; 64];
        snprintf(
            &raw mut cfgkey as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
            b"config.keys.%d.name\0".as_ptr() as *const ::core::ffi::c_char,
            i_1 + 1 as ::core::ffi::c_int,
        );
        cfgkey[::core::mem::size_of::<[::core::ffi::c_char; 64]>()
            .wrapping_sub(1usize)] = '\0' as ::core::ffi::c_char;
        uqm_lua_checked!(res_Remove(&raw mut cfgkey as *mut ::core::ffi::c_char), 0);
        i_1 += 1;
    }
    snddriver = options.soundDriver.value;
    soundflags = options.soundQuality.value;
    opt3doMusic = OPT_ENABLABLE(options.use3doMusic.value as ::core::ffi::c_uint);
    optRemixMusic = OPT_ENABLABLE(options.useRemixMusic.value as ::core::ffi::c_uint);
    optSpeech = OPT_ENABLABLE(options.useSpeech.value as ::core::ffi::c_uint);
    optWhichCoarseScan = options.whichCoarseScan.value;
    optWhichMenu = options.whichMenu.value;
    optWhichFonts = options.whichFonts.value;
    optWhichIntro = options.whichIntro.value;
    optWhichShield = options.whichShield.value;
    optSmoothScroll = options.smoothScroll.value;
    optMeleeScale = options.meleeScale.value;
    optSubtitles = OPT_ENABLABLE(options.subtitles.value as ::core::ffi::c_uint);
    optStereoSFX = OPT_ENABLABLE(options.stereoSFX.value as ::core::ffi::c_uint);
    musicVolumeScale = options.musicVolumeScale.value;
    sfxVolumeScale = options.sfxVolumeScale.value;
    speechVolumeScale = options.speechVolumeScale.value;
    optAddons = options.addons;
    optGodModes = options.optGodModes.value;
    timeDilationScale = options.timeDilationScale.value;
    optBubbleWarp = OPT_ENABLABLE(options.bubbleWarp.value as ::core::ffi::c_uint);
    optUnlockShips = OPT_ENABLABLE(options.unlockShips.value as ::core::ffi::c_uint);
    optHeadStart = OPT_ENABLABLE(options.headStart.value as ::core::ffi::c_uint);
    optAutoButtons = OPT_ENABLABLE(options.autoButtons.value as ::core::ffi::c_uint);
    optInfiniteRU = OPT_ENABLABLE(options.infiniteRU.value as ::core::ffi::c_uint);
    optSkipIntro = OPT_ENABLABLE(options.skipIntro.value as ::core::ffi::c_uint);
    optMainMenuMusic = OPT_ENABLABLE(options.mainMenuMusic.value as ::core::ffi::c_uint);
    optNebulae = OPT_ENABLABLE(options.nebulae.value as ::core::ffi::c_uint);
    optOrbitingPlanets = OPT_ENABLABLE(
        options.orbitingPlanets.value as ::core::ffi::c_uint,
    );
    optTexturedPlanets = OPT_ENABLABLE(
        options.texturedPlanets.value as ::core::ffi::c_uint,
    );
    optCheatMode = OPT_ENABLABLE(options.cheatMode.value as ::core::ffi::c_uint);
    optDateFormat = options.optDateFormat.value;
    optInfiniteFuel = OPT_ENABLABLE(options.infiniteFuel.value as ::core::ffi::c_uint);
    optPartialPickup = OPT_ENABLABLE(options.partialPickup.value as ::core::ffi::c_uint);
    optSubmenu = OPT_ENABLABLE(options.submenu.value as ::core::ffi::c_uint);
    optInfiniteCredits = OPT_ENABLABLE(
        options.infiniteCredits.value as ::core::ffi::c_uint,
    );
    optCustomBorder = OPT_ENABLABLE(options.customBorder.value as ::core::ffi::c_uint);
    optSeedType = options.seedType.value;
    optCustomSeed = options.customSeed.value;
    optShipSeed = OPT_ENABLABLE(options.shipSeed.value as ::core::ffi::c_uint);
    optSphereColors = options.sphereColors.value;
    optRequiresReload = BOOLEAN::FALSE;
    optRequiresRestart = BOOLEAN::FALSE;
    optSpaceMusic = options.spaceMusic.value;
    optVolasMusic = OPT_ENABLABLE(options.volasMusic.value as ::core::ffi::c_uint);
    optWholeFuel = OPT_ENABLABLE(options.wholeFuel.value as ::core::ffi::c_uint);
    optDirJoy[0usize] = options.dirJoyP1.value;
    optLanderHold = options.landerHold.value;
    optScrTrans = options.scrTrans.value;
    optDifficulty = options.optDifficulty.value;
    optDiffChooser = options.optDiffChooser.value;
    optFuelRange = options.optFuelRange.value;
    optExtended = OPT_ENABLABLE(options.extended.value as ::core::ffi::c_uint);
    optNomad = options.nomad.value;
    optGameOver = OPT_ENABLABLE(options.gameOver.value as ::core::ffi::c_uint);
    optShipDirectionIP = OPT_ENABLABLE(
        options.shipDirectionIP.value as ::core::ffi::c_uint,
    );
    optHazardColors = OPT_ENABLABLE(options.hazardColors.value as ::core::ffi::c_uint);
    optOrzCompFont = OPT_ENABLABLE(options.orzCompFont.value as ::core::ffi::c_uint);
    optControllerType = options.optControllerType.value;
    optSmartAutoPilot = OPT_ENABLABLE(
        options.smartAutoPilot.value as ::core::ffi::c_uint,
    );
    optTintPlanSphere = options.tintPlanSphere.value;
    optPlanetStyle = options.planetStyle.value;
    optStarBackground = options.starBackground.value;
    optScanStyle = options.scanStyle.value;
    optNonStopOscill = OPT_ENABLABLE(options.nonStopOscill.value as ::core::ffi::c_uint);
    optScopeStyle = options.scopeStyle.value;
    optHyperStars = OPT_ENABLABLE(options.hyperStars.value as ::core::ffi::c_uint);
    optSuperPC = options.landerStyle.value;
    optPlanetTexture = OPT_ENABLABLE(options.planetTexture.value as ::core::ffi::c_uint);
    optFlagshipColor = options.flagshipColor.value;
    optNoHQEncounters = OPT_ENABLABLE(
        options.noHQEncounters.value as ::core::ffi::c_uint,
    );
    optDeCleansing = OPT_ENABLABLE(options.deCleansing.value as ::core::ffi::c_uint);
    optMeleeObstacles = OPT_ENABLABLE(
        options.meleeObstacles.value as ::core::ffi::c_uint,
    );
    optShowVisitedStars = OPT_ENABLABLE(
        options.showVisitedStars.value as ::core::ffi::c_uint,
    );
    optUnscaledStarSystem = OPT_ENABLABLE(
        options.unscaledStarSystem.value as ::core::ffi::c_uint,
    );
    optScanSphere = options.sphereType.value;
    optNebulaeVolume = options.nebulaevol.value;
    optSlaughterMode = OPT_ENABLABLE(options.slaughterMode.value as ::core::ffi::c_uint);
    optAdvancedAutoPilot = OPT_ENABLABLE(
        options.advancedAutoPilot.value as ::core::ffi::c_uint,
    );
    optMeleeToolTips = OPT_ENABLABLE(options.meleeToolTips.value as ::core::ffi::c_uint);
    optMusicResume = options.musicResume.value;
    optScatterElements = OPT_ENABLABLE(
        options.scatterElements.value as ::core::ffi::c_uint,
    );
    optShowUpgrades = OPT_ENABLABLE(options.showUpgrades.value as ::core::ffi::c_uint);
    optFleetPointSys = OPT_ENABLABLE(options.fleetPointSys.value as ::core::ffi::c_uint);
    optShipStore = OPT_ENABLABLE(options.shipStore.value as ::core::ffi::c_uint);
    optCaptainNames = OPT_ENABLABLE(options.captainNames.value as ::core::ffi::c_uint);
    optDosMenus = OPT_ENABLABLE(options.dosMenus.value as ::core::ffi::c_uint);
    optHyperSpaceColor = options.hyperSpaceColor.value;
    DeadZoneLeftStick[0usize] = options.deadZoneLeftP1.value;
    DeadZoneRightStick[0usize] = options.deadZoneRightP1.value;
    DeadZoneLeftStick[1usize] = options.deadZoneLeftP2.value;
    DeadZoneRightStick[1usize] = options.deadZoneRightP2.value;
    optDirJoy[1usize] = options.dirJoyP2.value;
    uqm_lua_checked!(prepareContentDir(options.contentDir, options.addonDir, *argv.offset(0isize)), 0);
    resolutionFactor = if isAddonAvailable(
            b"mm-hd\0".as_ptr() as *const ::core::ffi::c_char,
        )
        .0 != 0
    {
        options.resolutionFactor.value as ::core::ffi::c_uint
    } else {
        0 as ::core::ffi::c_uint
    };
    options.resolutionFactor.value = resolutionFactor as ::core::ffi::c_int;
    options.resolutionFactor.set = true;
    loresBlowupScale = options.loresBlowupScale.value as ::core::ffi::c_uint;
    optKeepAspectRatio = OPT_ENABLABLE(
        options.keepAspectRatio.value as ::core::ffi::c_uint,
    );
    optWindowType = C2Rust_Unnamed_5::OPTVAL_UQM_WINDOW.0 as ::core::ffi::c_int as DWORD;
    if options.windowType.value
        < C2Rust_Unnamed_5::OPTVAL_UQM_WINDOW.0 as ::core::ffi::c_int
        && isAddonAvailable(
                if options.windowType.value == 0 as ::core::ffi::c_int {
                    if resolutionFactor != 0 {
                        b"dos-mode-hd\0".as_ptr() as *const ::core::ffi::c_char
                    } else {
                        b"dos-mode-sd\0".as_ptr() as *const ::core::ffi::c_char
                    }
                } else if resolutionFactor != 0 {
                    b"3do-mode-hd\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"3do-mode-sd\0".as_ptr() as *const ::core::ffi::c_char
                },
            )
            .0 != 0
    {
        optWindowType = options.windowType.value as DWORD;
    }
    options.windowType.value = optWindowType as ::core::ffi::c_int;
    options.windowType.set = true;
    let mut w: ::core::ffi::c_int = 320 as ::core::ffi::c_int;
    let mut h: ::core::ffi::c_int = if if optWindowType == 0 as DWORD {
        BOOLEAN::TRUE.0 as ::core::ffi::c_int
    } else {
        BOOLEAN::FALSE.0 as ::core::ffi::c_int
    } != 0
    {
        200 as ::core::ffi::c_int
    } else {
        240 as ::core::ffi::c_int
    };
    let mut scaleFactor: ::core::ffi::c_int = loresBlowupScale
        .wrapping_add(1 as ::core::ffi::c_uint) as ::core::ffi::c_int;
    CanvasWidth = w << resolutionFactor;
    CanvasHeight = h << resolutionFactor;
    if loresBlowupScale < 6 as ::core::ffi::c_uint {
        SavedWidth = if loresBlowupScale != 0 { w * scaleFactor } else { CanvasWidth };
        SavedHeight = if loresBlowupScale != 0 { h * scaleFactor } else { CanvasHeight };
    } else {
        SavedWidth = inBounds(options.resolution.width, 320 as SDWORD, 1920 as SDWORD)
            as ::core::ffi::c_int;
        SavedHeight = inBounds(options.resolution.height, 200 as SDWORD, 1440 as SDWORD)
            as ::core::ffi::c_int;
    }
    if optKeepAspectRatio.0 != 0 {
        let mut threshold: ::core::ffi::c_float = 0.75f32;
        let mut ratio: ::core::ffi::c_float = SavedHeight as ::core::ffi::c_float
            / SavedWidth as ::core::ffi::c_float;
        if ratio > threshold {
            options.resolution.width = (SavedHeight as ::core::ffi::c_float / threshold)
                as ::core::ffi::c_int;
        } else if ratio < threshold {
            options.resolution.height = (SavedWidth as ::core::ffi::c_float * threshold)
                as ::core::ffi::c_int;
        }
    } else {
        options.resolution.width = SavedWidth;
        options.resolution.height = SavedHeight;
    }
    uqm_lua_checked!(prepareMeleeDir(), 0);
    uqm_lua_checked!(prepareSaveDir(), 0);
    uqm_lua_checked!(prepareScrShotDir(), 0);
    uqm_lua_checked!(prepareShadowAddons(options.addons), 0);
    InitTimeSystem();
    InitTaskSystem();
    luaUqm_init();
    Alarm_init();
    Callback_init();
    gfxDriver = C2Rust_Unnamed::TFB_GFXDRIVER_SDL_PURE.0 as ::core::ffi::c_int;
    gfxFlags = options.scaler.value;
    if options.fullscreen.value != 0 {
        if options.fullscreen.value > 1 as ::core::ffi::c_int {
            gfxFlags |= (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int;
        } else {
            gfxFlags |= (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
        }
    }
    if options.scanlines.value {
        gfxFlags |= (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int;
    }
    if options.showFps.value {
        gfxFlags |= (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int;
    }
    TFB_InitGraphics(
        gfxDriver,
        gfxFlags,
        options.graphicsBackend,
        options.resolution.width,
        options.resolution.height,
        &raw mut resolutionFactor,
        &raw mut optWindowType,
    );
    if options.gamma.set as ::core::ffi::c_int != 0
        && setGammaCorrection(options.gamma.value) as ::core::ffi::c_int != 0
    {
        optGamma = options.gamma.value;
    } else {
        optGamma = 1.0f32;
    }
    InitColorMaps();
    init_communication();
    (::core::mem::size_of::<[::core::ffi::c_int; 16]>()
        == ::core::mem::size_of::<[[::core::ffi::c_int; 8]; 2]>()
        || {
            __assert_fail(
                b"sizeof (int [NUM_TEMPLATES * NUM_KEYS]) == sizeof (int [NUM_TEMPLATES][NUM_KEYS])\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"src/uqm.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                782 as ::core::ffi::c_int,
                b"main\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    TFB_SetInputVectors(
        &raw mut ImmediateInputState.menu as *mut ::core::ffi::c_int,
        C2Rust_Unnamed_4::NUM_MENU_KEYS.0 as ::core::ffi::c_int,
        &raw mut ImmediateInputState.key as *mut [::core::ffi::c_int; 8]
            as *mut ::core::ffi::c_int,
        CONTROL_TEMPLATE::NUM_TEMPLATES.0 as ::core::ffi::c_int,
        C2Rust_Unnamed_3::NUM_KEYS.0 as ::core::ffi::c_int,
    );
    uqm_lua_checked!(TFB_InitInput(
        C2Rust_Unnamed_2::TFB_INPUTDRIVER_SDL.0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    ), 0);
    StartThread_Core(
        Some(
            Starcon2Main
                as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
        ),
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
        1024 as SDWORD,
        b"Starcon2Main\0".as_ptr() as *const ::core::ffi::c_char,
    );
    i = 0 as ::core::ffi::c_int;
    while i < 2000 as ::core::ffi::c_int
        && ::core::ptr::read_volatile::<::core::ffi::c_int>(&raw const MainExited) == 0
    {
        if ::core::ptr::read_volatile::<::core::ffi::c_int>(&raw const QuitPosted) != 0 {
            SignalStopMainThread();
            i += 1;
        } else if ::core::ptr::read_volatile::<::core::ffi::c_int>(&raw const GameActive)
            == 0
        {
            HibernateThread(
                (840 as ::core::ffi::c_int / 4 as ::core::ffi::c_int) as TimePeriod,
            );
        }
        uqm_lua_checked!(TFB_ProcessEvents(), 0);
        uqm_lua_checked!(ProcessUtilityKeys(), 0);
        ProcessThreadLifecycles();
        uqm_lua_checked!(TFB_FlushGraphics(), 0);
    }
    if ::core::ptr::read_volatile::<::core::ffi::c_int>(&raw const MainExited) != 0 {
        TFB_UninitInput();
        uqm_lua_checked!(unInitAudio(), 0);
        uninit_communication();
        TFB_PurgeDanglingGraphics();
        UninitColorMaps();
        TFB_UninitGraphics();
        Callback_uninit();
        uqm_lua_checked!(Alarm_uninit(), 0);
        luaUqm_uninit();
        CleanupTaskSystem();
        UnInitTimeSystem();
        unprepareAllDirs();
        uqm_lua_checked!(uninitIO(), 0);
        UnInitThreadSystem();
        mem_uninit();
    }
    HFree(options.addons as *mut ::core::ffi::c_void);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn saveErrorV(
    mut fmt: *const ::core::ffi::c_char,
    mut list: ::core::ffi::VaList,
) {
    let mut len: ::core::ffi::c_int = strlen(
        &raw mut errBuffer as *mut ::core::ffi::c_char,
    ) as ::core::ffi::c_int;
    let mut left: ::core::ffi::c_int = ::core::mem::size_of::<
        [::core::ffi::c_char; 512],
    >()
        .wrapping_sub(len as usize) as ::core::ffi::c_int;
    if len > 0 as ::core::ffi::c_int && left > 0 as ::core::ffi::c_int {
        errBuffer[len as usize] = '\n' as ::core::ffi::c_char;
        len += 1;
        left -= 1;
    }
    vsnprintf(
        (&raw mut errBuffer as *mut ::core::ffi::c_char).offset(len as isize),
        left as size_t,
        fmt,
        list.as_va_list(),
    );
    errBuffer[::core::mem::size_of::<[::core::ffi::c_char; 512]>()
        .wrapping_sub(1usize)] = '\0' as ::core::ffi::c_char;
}
unsafe extern "C" fn saveError(
    mut fmt: *const ::core::ffi::c_char,
    mut c2rust_args: ...
) {
    let mut list: ::core::ffi::VaListImpl;
    list = c2rust_args.clone();
    saveErrorV(fmt, list.as_va_list());
}
unsafe extern "C" fn lookupOptionValue(
    mut list: *const option_list_value,
    mut strval: *const ::core::ffi::c_char,
    mut ret: *mut ::core::ffi::c_int,
) -> bool {
    if list.is_null() {
        return false;
    }
    while !(*list).str.is_null()
        && strcmp(strval, (*list).str) != 0 as ::core::ffi::c_int
    {
        list = list.offset(1);
    }
    if (*list).str.is_null() {
        return false;
    }
    *ret = (*list).value;
    return true;
}
unsafe extern "C" fn getBoolConfigValue(
    mut option: *mut bool_option,
    mut config_val: *const ::core::ffi::c_char,
) {
    if (*option).set as ::core::ffi::c_int != 0 || res_IsBoolean(config_val).0 == 0 {
        return;
    }
    (*option).value = res_GetBoolean(config_val).0 != 0;
    (*option).set = true;
}
unsafe extern "C" fn getBoolConfigValueXlat(
    mut option: *mut int_option,
    mut config_val: *const ::core::ffi::c_char,
    mut true_val: ::core::ffi::c_int,
    mut false_val: ::core::ffi::c_int,
) {
    if (*option).set as ::core::ffi::c_int != 0 || res_IsBoolean(config_val).0 == 0 {
        return;
    }
    (*option).value = if res_GetBoolean(config_val).0 != 0 {
        true_val
    } else {
        false_val
    };
    (*option).set = true;
}
unsafe extern "C" fn getVolumeConfigValue(
    mut option: *mut float_option,
    mut config_val: *const ::core::ffi::c_char,
) {
    if (*option).set as ::core::ffi::c_int != 0 || res_IsInteger(config_val).0 == 0 {
        return;
    }
    parseIntVolume(res_GetInteger(config_val), &raw mut (*option).value);
    (*option).set = true;
}
unsafe extern "C" fn getGammaConfigValue(
    mut option: *mut float_option,
    mut config_val: *const ::core::ffi::c_char,
) {
    let mut val: ::core::ffi::c_int = 0;
    if (*option).set as ::core::ffi::c_int != 0 || res_IsInteger(config_val).0 == 0 {
        return;
    }
    val = res_GetInteger(config_val);
    if val
        < (0.03f64 * 1000 as ::core::ffi::c_int as ::core::ffi::c_double)
            as ::core::ffi::c_int
        || val
            > (9.9f64 * 1000 as ::core::ffi::c_int as ::core::ffi::c_double)
                as ::core::ffi::c_int
    {
        return;
    }
    (*option).value = val as ::core::ffi::c_float
        / 1000 as ::core::ffi::c_int as ::core::ffi::c_float;
    if (*option).value != 1.0f32 {
        (*option).set = true;
    }
}
unsafe extern "C" fn getListConfigValue(
    mut option: *mut int_option,
    mut config_val: *const ::core::ffi::c_char,
    mut list: *const option_list_value,
) -> bool {
    let mut strval: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    let mut found: bool = false;
    if (*option).set as ::core::ffi::c_int != 0 || res_IsString(config_val).0 == 0
        || list.is_null()
    {
        return false;
    }
    strval = res_GetString(config_val);
    found = lookupOptionValue(list, strval, &raw mut (*option).value);
    (*option).set = found;
    return found;
}
unsafe extern "C" fn getDeadzoneConfigValue(
    mut option: *mut int_option,
    mut config_val: *const ::core::ffi::c_char,
) {
    if res_IsInteger(config_val).0 == 0 {
        return;
    }
    if (*option).set {
        return;
    }
    (*option).value = res_GetInteger(config_val);
    (*option).set = true;
    if (*option).value > 32767 as ::core::ffi::c_int
        || (*option).value < 0 as ::core::ffi::c_int
    {
        (*option).value = 6881 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn getUserConfigOptions(mut options: *mut options_struct) {
    if res_IsInteger(b"config.reswidth\0".as_ptr() as *const ::core::ffi::c_char).0 != 0
        && res_IsInteger(b"config.resheight\0".as_ptr() as *const ::core::ffi::c_char).0
            != 0 && !(*options).resolution.set
    {
        (*options).resolution.width = res_GetInteger(
            b"config.reswidth\0".as_ptr() as *const ::core::ffi::c_char,
        );
        (*options).resolution.height = res_GetInteger(
            b"config.resheight\0".as_ptr() as *const ::core::ffi::c_char,
        );
        (*options).resolution.set = true;
    }
    if res_IsBoolean(b"config.alwaysgl\0".as_ptr() as *const ::core::ffi::c_char).0 != 0
        && !(*options).opengl.set
    {
        if res_GetBoolean(b"config.alwaysgl\0".as_ptr() as *const ::core::ffi::c_char).0
            != 0
        {
            (*options).opengl.value = true;
            (*options).opengl.set = true;
        }
    }
    getBoolConfigValue(
        &raw mut (*options).opengl,
        b"config.usegl\0".as_ptr() as *const ::core::ffi::c_char,
    );
    getListConfigValue(
        &raw mut (*options).scaler,
        b"config.scaler\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const scalerList as *const option_list_value,
    );
    if res_IsInteger(b"config.fullscreen\0".as_ptr() as *const ::core::ffi::c_char).0
        != 0 && !(*options).fullscreen.set
    {
        (*options).fullscreen.value = res_GetInteger(
            b"config.fullscreen\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    getBoolConfigValue(
        &raw mut (*options).scanlines,
        b"config.scanlines\0".as_ptr() as *const ::core::ffi::c_char,
    );
    getBoolConfigValue(
        &raw mut (*options).showFps,
        b"config.showfps\0".as_ptr() as *const ::core::ffi::c_char,
    );
    getBoolConfigValue(
        &raw mut (*options).keepAspectRatio,
        b"config.keepaspectratio\0".as_ptr() as *const ::core::ffi::c_char,
    );
    getGammaConfigValue(
        &raw mut (*options).gamma,
        b"config.gamma\0".as_ptr() as *const ::core::ffi::c_char,
    );
    getBoolConfigValue(
        &raw mut (*options).subtitles,
        b"config.subtitles\0".as_ptr() as *const ::core::ffi::c_char,
    );
    getBoolConfigValueXlat(
        &raw mut (*options).whichMenu,
        b"config.textmenu\0".as_ptr() as *const ::core::ffi::c_char,
        0x1 as ::core::ffi::c_int,
        0x2 as ::core::ffi::c_int,
    );
    getBoolConfigValueXlat(
        &raw mut (*options).whichFonts,
        b"config.textgradients\0".as_ptr() as *const ::core::ffi::c_char,
        0x1 as ::core::ffi::c_int,
        0x2 as ::core::ffi::c_int,
    );
    if res_IsInteger(b"config.iconicscan\0".as_ptr() as *const ::core::ffi::c_char).0
        != 0 && !(*options).whichCoarseScan.set
    {
        (*options).whichCoarseScan.value = res_GetInteger(
            b"config.iconicscan\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    getBoolConfigValueXlat(
        &raw mut (*options).smoothScroll,
        b"config.smoothscroll\0".as_ptr() as *const ::core::ffi::c_char,
        0x1 as ::core::ffi::c_int,
        0x2 as ::core::ffi::c_int,
    );
    getBoolConfigValueXlat(
        &raw mut (*options).whichShield,
        b"config.pulseshield\0".as_ptr() as *const ::core::ffi::c_char,
        0x1 as ::core::ffi::c_int,
        0x2 as ::core::ffi::c_int,
    );
    getBoolConfigValueXlat(
        &raw mut (*options).whichIntro,
        b"config.3domovies\0".as_ptr() as *const ::core::ffi::c_char,
        0x1 as ::core::ffi::c_int,
        0x2 as ::core::ffi::c_int,
    );
    getBoolConfigValue(
        &raw mut (*options).use3doMusic,
        b"config.3domusic\0".as_ptr() as *const ::core::ffi::c_char,
    );
    getBoolConfigValue(
        &raw mut (*options).useRemixMusic,
        b"config.remixmusic\0".as_ptr() as *const ::core::ffi::c_char,
    );
    getBoolConfigValue(
        &raw mut (*options).useSpeech,
        b"config.speech\0".as_ptr() as *const ::core::ffi::c_char,
    );
    getBoolConfigValueXlat(
        &raw mut (*options).meleeScale,
        b"config.smoothmelee\0".as_ptr() as *const ::core::ffi::c_char,
        C2Rust_Unnamed_0::TFB_SCALE_TRILINEAR.0 as ::core::ffi::c_int,
        C2Rust_Unnamed_0::TFB_SCALE_STEP.0 as ::core::ffi::c_int,
    );
    getListConfigValue(
        &raw mut (*options).soundDriver,
        b"config.audiodriver\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const audioDriverList as *const option_list_value,
    );
    getListConfigValue(
        &raw mut (*options).soundQuality,
        b"config.audioquality\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const audioQualityList as *const option_list_value,
    );
    getBoolConfigValue(
        &raw mut (*options).stereoSFX,
        b"config.positionalsfx\0".as_ptr() as *const ::core::ffi::c_char,
    );
    getVolumeConfigValue(
        &raw mut (*options).musicVolumeScale,
        b"config.musicvol\0".as_ptr() as *const ::core::ffi::c_char,
    );
    getVolumeConfigValue(
        &raw mut (*options).sfxVolumeScale,
        b"config.sfxvol\0".as_ptr() as *const ::core::ffi::c_char,
    );
    getVolumeConfigValue(
        &raw mut (*options).speechVolumeScale,
        b"config.speechvol\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if res_IsInteger(b"config.resolutionfactor\0".as_ptr() as *const ::core::ffi::c_char)
        .0 != 0 && !(*options).resolutionFactor.set
    {
        (*options).resolutionFactor.value = res_GetInteger(
            b"config.resolutionfactor\0".as_ptr() as *const ::core::ffi::c_char,
        );
        (*options).resolutionFactor.set = true;
    }
    if res_IsInteger(b"config.loresBlowupScale\0".as_ptr() as *const ::core::ffi::c_char)
        .0 != 0
    {
        (*options).loresBlowupScale.value = res_GetInteger(
            b"config.loresBlowupScale\0".as_ptr() as *const ::core::ffi::c_char,
        );
        (*options).loresBlowupScale.set = true;
    }
    getBoolConfigValue(
        &raw mut (*options).cheatMode,
        b"cheat.kohrStahp\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if res_IsInteger(b"cheat.godModes\0".as_ptr() as *const ::core::ffi::c_char).0 != 0
        && !(*options).optGodModes.set
    {
        (*options).optGodModes.value = res_GetInteger(
            b"cheat.godModes\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if res_IsInteger(b"cheat.timeDilation\0".as_ptr() as *const ::core::ffi::c_char).0
        != 0 && !(*options).timeDilationScale.set
    {
        (*options).timeDilationScale.value = res_GetInteger(
            b"cheat.timeDilation\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    getBoolConfigValue(
        &raw mut (*options).bubbleWarp,
        b"cheat.bubbleWarp\0".as_ptr() as *const ::core::ffi::c_char,
    );
    getBoolConfigValue(
        &raw mut (*options).unlockShips,
        b"cheat.unlockShips\0".as_ptr() as *const ::core::ffi::c_char,
    );
    getBoolConfigValue(
        &raw mut (*options).headStart,
        b"cheat.headStart\0".as_ptr() as *const ::core::ffi::c_char,
    );
    getBoolConfigValue(
        &raw mut (*options).autoButtons,
        b"mm.autoButtons\0".as_ptr() as *const ::core::ffi::c_char,
    );
    getBoolConfigValue(
        &raw mut (*options).infiniteRU,
        b"cheat.infiniteRU\0".as_ptr() as *const ::core::ffi::c_char,
    );
    getBoolConfigValue(
        &raw mut (*options).skipIntro,
        b"mm.skipIntro\0".as_ptr() as *const ::core::ffi::c_char,
    );
    getBoolConfigValue(
        &raw mut (*options).mainMenuMusic,
        b"mm.mainMenuMusic\0".as_ptr() as *const ::core::ffi::c_char,
    );
    getBoolConfigValue(
        &raw mut (*options).nebulae,
        b"mm.nebulae\0".as_ptr() as *const ::core::ffi::c_char,
    );
    getBoolConfigValue(
        &raw mut (*options).orbitingPlanets,
        b"mm.orbitingPlanets\0".as_ptr() as *const ::core::ffi::c_char,
    );
    getBoolConfigValue(
        &raw mut (*options).texturedPlanets,
        b"mm.texturedPlanets\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if res_IsInteger(b"mm.dateFormat\0".as_ptr() as *const ::core::ffi::c_char).0 != 0
        && !(*options).optDateFormat.set
    {
        (*options).optDateFormat.value = res_GetInteger(
            b"mm.dateFormat\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    getBoolConfigValue(
        &raw mut (*options).infiniteFuel,
        b"cheat.infiniteFuel\0".as_ptr() as *const ::core::ffi::c_char,
    );
    getBoolConfigValue(
        &raw mut (*options).partialPickup,
        b"mm.partialPickup\0".as_ptr() as *const ::core::ffi::c_char,
    );
    getBoolConfigValue(
        &raw mut (*options).submenu,
        b"mm.submenu\0".as_ptr() as *const ::core::ffi::c_char,
    );
    getBoolConfigValue(
        &raw mut (*options).infiniteCredits,
        b"cheat.infiniteCredits\0".as_ptr() as *const ::core::ffi::c_char,
    );
    getBoolConfigValue(
        &raw mut (*options).customBorder,
        b"mm.customBorder\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if res_IsInteger(b"mm.seedType\0".as_ptr() as *const ::core::ffi::c_char).0 != 0
        && !(*options).seedType.set
    {
        (*options).seedType.value = res_GetInteger(
            b"mm.seedType\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if res_IsInteger(b"mm.customSeed\0".as_ptr() as *const ::core::ffi::c_char).0 != 0
        && !(*options).customSeed.set
    {
        (*options).customSeed.value = res_GetInteger(
            b"mm.customSeed\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if if (*options).customSeed.value < 3 as ::core::ffi::c_int
            || (*options).customSeed.value > 2147483644 as ::core::ffi::c_int
        {
            0 as ::core::ffi::c_int
        } else {
            1 as ::core::ffi::c_int
        } == 0
        {
            (*options).customSeed.value = 16807 as ::core::ffi::c_int;
        }
    }
    getBoolConfigValue(
        &raw mut (*options).shipSeed,
        b"mm.shipSeed\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if res_IsInteger(b"mm.sphereColors\0".as_ptr() as *const ::core::ffi::c_char).0 != 0
        && !(*options).sphereColors.set
    {
        (*options).sphereColors.value = res_GetInteger(
            b"mm.sphereColors\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if res_IsInteger(b"mm.spaceMusic\0".as_ptr() as *const ::core::ffi::c_char).0 != 0
        && !(*options).spaceMusic.set
    {
        (*options).spaceMusic.value = res_GetInteger(
            b"mm.spaceMusic\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    getBoolConfigValue(
        &raw mut (*options).volasMusic,
        b"mm.volasMusic\0".as_ptr() as *const ::core::ffi::c_char,
    );
    getBoolConfigValue(
        &raw mut (*options).wholeFuel,
        b"mm.wholeFuel\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if res_IsInteger(b"mm.dirJoyP1\0".as_ptr() as *const ::core::ffi::c_char).0 != 0
        && !(*options).dirJoyP1.set
    {
        (*options).dirJoyP1.value = res_GetInteger(
            b"mm.dirJoyP1\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    getBoolConfigValueXlat(
        &raw mut (*options).landerHold,
        b"mm.landerHold\0".as_ptr() as *const ::core::ffi::c_char,
        0x1 as ::core::ffi::c_int,
        0x2 as ::core::ffi::c_int,
    );
    getBoolConfigValueXlat(
        &raw mut (*options).scrTrans,
        b"mm.scrTransition\0".as_ptr() as *const ::core::ffi::c_char,
        0x1 as ::core::ffi::c_int,
        0x2 as ::core::ffi::c_int,
    );
    if res_IsInteger(b"mm.difficulty\0".as_ptr() as *const ::core::ffi::c_char).0 != 0
        && !(*options).optDifficulty.set
    {
        (*options).optDifficulty.value = res_GetInteger(
            b"mm.difficulty\0".as_ptr() as *const ::core::ffi::c_char,
        );
        (*options).optDiffChooser.value = (*options).optDifficulty.value;
        if (*options).optDifficulty.value > 2 as ::core::ffi::c_int {
            (*options).optDifficulty.value = 0 as ::core::ffi::c_int;
        }
    }
    if res_IsInteger(b"mm.fuelRange\0".as_ptr() as *const ::core::ffi::c_char).0 != 0
        && !(*options).optFuelRange.set
    {
        (*options).optFuelRange.value = res_GetInteger(
            b"mm.fuelRange\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    getBoolConfigValue(
        &raw mut (*options).extended,
        b"mm.extended\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if res_IsInteger(b"mm.nomad\0".as_ptr() as *const ::core::ffi::c_char).0 != 0
        && !(*options).nomad.set
    {
        (*options).nomad.value = res_GetInteger(
            b"mm.nomad\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if (*options).nomad.value > 2 as ::core::ffi::c_int {
            (*options).nomad.value = 0 as ::core::ffi::c_int;
        }
    }
    getBoolConfigValue(
        &raw mut (*options).gameOver,
        b"mm.gameOver\0".as_ptr() as *const ::core::ffi::c_char,
    );
    getBoolConfigValue(
        &raw mut (*options).shipDirectionIP,
        b"mm.shipDirectionIP\0".as_ptr() as *const ::core::ffi::c_char,
    );
    getBoolConfigValue(
        &raw mut (*options).hazardColors,
        b"mm.hazardColors\0".as_ptr() as *const ::core::ffi::c_char,
    );
    getBoolConfigValue(
        &raw mut (*options).orzCompFont,
        b"mm.orzCompFont\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if res_IsInteger(b"mm.controllerType\0".as_ptr() as *const ::core::ffi::c_char).0
        != 0 && !(*options).optControllerType.set
    {
        (*options).optControllerType.value = res_GetInteger(
            b"mm.controllerType\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    getBoolConfigValue(
        &raw mut (*options).smartAutoPilot,
        b"mm.smartAutoPilot\0".as_ptr() as *const ::core::ffi::c_char,
    );
    getBoolConfigValueXlat(
        &raw mut (*options).tintPlanSphere,
        b"mm.tintPlanSphere\0".as_ptr() as *const ::core::ffi::c_char,
        0x1 as ::core::ffi::c_int,
        0x2 as ::core::ffi::c_int,
    );
    getBoolConfigValueXlat(
        &raw mut (*options).planetStyle,
        b"mm.planetStyle\0".as_ptr() as *const ::core::ffi::c_char,
        0x1 as ::core::ffi::c_int,
        0x2 as ::core::ffi::c_int,
    );
    if res_IsInteger(b"mm.starBackground\0".as_ptr() as *const ::core::ffi::c_char).0
        != 0 && !(*options).starBackground.set
    {
        (*options).starBackground.value = res_GetInteger(
            b"mm.starBackground\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    getBoolConfigValueXlat(
        &raw mut (*options).scanStyle,
        b"mm.scanStyle\0".as_ptr() as *const ::core::ffi::c_char,
        0x1 as ::core::ffi::c_int,
        0x2 as ::core::ffi::c_int,
    );
    getBoolConfigValue(
        &raw mut (*options).nonStopOscill,
        b"mm.nonStopOscill\0".as_ptr() as *const ::core::ffi::c_char,
    );
    getBoolConfigValueXlat(
        &raw mut (*options).scopeStyle,
        b"mm.scopeStyle\0".as_ptr() as *const ::core::ffi::c_char,
        0x1 as ::core::ffi::c_int,
        0x2 as ::core::ffi::c_int,
    );
    getBoolConfigValue(
        &raw mut (*options).hyperStars,
        b"mm.hyperStars\0".as_ptr() as *const ::core::ffi::c_char,
    );
    getBoolConfigValueXlat(
        &raw mut (*options).landerStyle,
        b"mm.landerStyle\0".as_ptr() as *const ::core::ffi::c_char,
        0x1 as ::core::ffi::c_int,
        0x2 as ::core::ffi::c_int,
    );
    getBoolConfigValue(
        &raw mut (*options).planetTexture,
        b"mm.planetTexture\0".as_ptr() as *const ::core::ffi::c_char,
    );
    getBoolConfigValueXlat(
        &raw mut (*options).flagshipColor,
        b"mm.flagshipColor\0".as_ptr() as *const ::core::ffi::c_char,
        0x1 as ::core::ffi::c_int,
        0x2 as ::core::ffi::c_int,
    );
    getBoolConfigValue(
        &raw mut (*options).noHQEncounters,
        b"cheat.noHQEncounters\0".as_ptr() as *const ::core::ffi::c_char,
    );
    getBoolConfigValue(
        &raw mut (*options).deCleansing,
        b"cheat.deCleansing\0".as_ptr() as *const ::core::ffi::c_char,
    );
    getBoolConfigValue(
        &raw mut (*options).meleeObstacles,
        b"cheat.meleeObstacles\0".as_ptr() as *const ::core::ffi::c_char,
    );
    getBoolConfigValue(
        &raw mut (*options).showVisitedStars,
        b"mm.showVisitedStars\0".as_ptr() as *const ::core::ffi::c_char,
    );
    getBoolConfigValue(
        &raw mut (*options).unscaledStarSystem,
        b"mm.unscaledStarSystem\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if res_IsInteger(b"mm.sphereType\0".as_ptr() as *const ::core::ffi::c_char).0 != 0
        && !(*options).sphereType.set
    {
        (*options).sphereType.value = res_GetInteger(
            b"mm.sphereType\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if res_IsInteger(b"mm.nebulaevol\0".as_ptr() as *const ::core::ffi::c_char).0 != 0
        && !(*options).nebulaevol.set
    {
        (*options).nebulaevol.value = res_GetInteger(
            b"mm.nebulaevol\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if (*options).nebulaevol.value > 50 as ::core::ffi::c_int {
            (*options).nebulaevol.value = 11 as ::core::ffi::c_int;
        }
    }
    getBoolConfigValue(
        &raw mut (*options).slaughterMode,
        b"mm.slaughterMode\0".as_ptr() as *const ::core::ffi::c_char,
    );
    getBoolConfigValue(
        &raw mut (*options).advancedAutoPilot,
        b"mm.advancedAutoPilot\0".as_ptr() as *const ::core::ffi::c_char,
    );
    getBoolConfigValue(
        &raw mut (*options).meleeToolTips,
        b"mm.meleeToolTips\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if res_IsInteger(b"mm.musicResume\0".as_ptr() as *const ::core::ffi::c_char).0 != 0
        && !(*options).musicResume.set
    {
        (*options).musicResume.value = res_GetInteger(
            b"mm.musicResume\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if res_IsInteger(b"mm.windowType\0".as_ptr() as *const ::core::ffi::c_char).0 != 0
        && !(*options).windowType.set
    {
        (*options).windowType.value = res_GetInteger(
            b"mm.windowType\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    getBoolConfigValue(
        &raw mut (*options).scatterElements,
        b"mm.scatterElements\0".as_ptr() as *const ::core::ffi::c_char,
    );
    getBoolConfigValue(
        &raw mut (*options).showUpgrades,
        b"mm.showUpgrades\0".as_ptr() as *const ::core::ffi::c_char,
    );
    getBoolConfigValue(
        &raw mut (*options).fleetPointSys,
        b"mm.fleetPointSys\0".as_ptr() as *const ::core::ffi::c_char,
    );
    getBoolConfigValue(
        &raw mut (*options).shipStore,
        b"mm.shipStore\0".as_ptr() as *const ::core::ffi::c_char,
    );
    getBoolConfigValue(
        &raw mut (*options).captainNames,
        b"mm.captainNames\0".as_ptr() as *const ::core::ffi::c_char,
    );
    getBoolConfigValue(
        &raw mut (*options).dosMenus,
        b"mm.dosMenus\0".as_ptr() as *const ::core::ffi::c_char,
    );
    getBoolConfigValueXlat(
        &raw mut (*options).hyperSpaceColor,
        b"mm.hyperSpaceColor\0".as_ptr() as *const ::core::ffi::c_char,
        0x1 as ::core::ffi::c_int,
        0x2 as ::core::ffi::c_int,
    );
    getDeadzoneConfigValue(
        &raw mut (*options).deadZoneLeftP1,
        b"mm.deadZoneLeftP1\0".as_ptr() as *const ::core::ffi::c_char,
    );
    getDeadzoneConfigValue(
        &raw mut (*options).deadZoneRightP1,
        b"mm.deadZoneRightP1\0".as_ptr() as *const ::core::ffi::c_char,
    );
    getDeadzoneConfigValue(
        &raw mut (*options).deadZoneLeftP2,
        b"mm.deadZoneLeftP2\0".as_ptr() as *const ::core::ffi::c_char,
    );
    getDeadzoneConfigValue(
        &raw mut (*options).deadZoneRightP2,
        b"mm.deadZoneRightP2\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if res_IsInteger(b"mm.dirJoyP2\0".as_ptr() as *const ::core::ffi::c_char).0 != 0
        && !(*options).dirJoyP2.set
    {
        (*options).dirJoyP2.value = res_GetInteger(
            b"mm.dirJoyP2\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    memset(
        &raw mut optDeviceArray as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[OPT_ADD_REMOVE; 28]>(),
    );
    memset(
        &raw mut optUpgradeArray as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[OPT_ADD_REMOVE; 13]>(),
    );
}
static mut optString: *const ::core::ffi::c_char = b"+r:f:oc:b:spC:n:?hM:S:T:q:ug:l:i:vwxk\0"
    .as_ptr() as *const ::core::ffi::c_char;
static mut longOptions: [option; 108] = [
    option {
        name: b"res\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 1 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'r' as ::core::ffi::c_int,
    },
    option {
        name: b"fullscreen\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 1 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'f' as ::core::ffi::c_int,
    },
    option {
        name: b"opengl\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'o' as ::core::ffi::c_int,
    },
    option {
        name: b"scale\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 1 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'c' as ::core::ffi::c_int,
    },
    option {
        name: b"meleezoom\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 1 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'b' as ::core::ffi::c_int,
    },
    option {
        name: b"scanlines\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 's' as ::core::ffi::c_int,
    },
    option {
        name: b"fps\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'p' as ::core::ffi::c_int,
    },
    option {
        name: b"configdir\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 1 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'C' as ::core::ffi::c_int,
    },
    option {
        name: b"contentdir\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 1 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'n' as ::core::ffi::c_int,
    },
    option {
        name: b"help\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'h' as ::core::ffi::c_int,
    },
    option {
        name: b"musicvol\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 1 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'M' as ::core::ffi::c_int,
    },
    option {
        name: b"sfxvol\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 1 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'S' as ::core::ffi::c_int,
    },
    option {
        name: b"speechvol\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 1 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'T' as ::core::ffi::c_int,
    },
    option {
        name: b"audioquality\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 1 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'q' as ::core::ffi::c_int,
    },
    option {
        name: b"nosubtitles\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'u' as ::core::ffi::c_int,
    },
    option {
        name: b"gamma\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 1 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'g' as ::core::ffi::c_int,
    },
    option {
        name: b"logfile\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 1 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'l' as ::core::ffi::c_int,
    },
    option {
        name: b"intro\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 1 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'i' as ::core::ffi::c_int,
    },
    option {
        name: b"version\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'v' as ::core::ffi::c_int,
    },
    option {
        name: b"windowed\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'w' as ::core::ffi::c_int,
    },
    option {
        name: b"nogl\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'x' as ::core::ffi::c_int,
    },
    option {
        name: b"keepaspectratio\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'k' as ::core::ffi::c_int,
    },
    option {
        name: b"cscan\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 1 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_8::CSCAN_OPT.0 as ::core::ffi::c_int,
    },
    option {
        name: b"menu\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 1 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_8::MENU_OPT.0 as ::core::ffi::c_int,
    },
    option {
        name: b"font\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 1 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_8::FONT_OPT.0 as ::core::ffi::c_int,
    },
    option {
        name: b"shield\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 1 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_8::SHIELD_OPT.0 as ::core::ffi::c_int,
    },
    option {
        name: b"scroll\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 1 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_8::SCROLL_OPT.0 as ::core::ffi::c_int,
    },
    option {
        name: b"sound\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 1 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_8::SOUND_OPT.0 as ::core::ffi::c_int,
    },
    option {
        name: b"stereosfx\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_8::STEREOSFX_OPT.0 as ::core::ffi::c_int,
    },
    option {
        name: b"addon\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 1 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_8::ADDON_OPT.0 as ::core::ffi::c_int,
    },
    option {
        name: b"addondir\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 1 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_8::ADDONDIR_OPT.0 as ::core::ffi::c_int,
    },
    option {
        name: b"accel\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 1 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_8::ACCEL_OPT.0 as ::core::ffi::c_int,
    },
    option {
        name: b"safe\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_8::SAFEMODE_OPT.0 as ::core::ffi::c_int,
    },
    option {
        name: b"renderer\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 1 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_8::RENDERER_OPT.0 as ::core::ffi::c_int,
    },
    option {
        name: b"kohrstahp\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_8::CHEATMODE_OPT.0 as ::core::ffi::c_int,
    },
    option {
        name: b"precursormode\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 1 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_8::GODMODE_OPT.0 as ::core::ffi::c_int,
    },
    option {
        name: b"timedilation\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 1 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_8::TDM_OPT.0 as ::core::ffi::c_int,
    },
    option {
        name: b"bubblewarp\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_8::BWARP_OPT.0 as ::core::ffi::c_int,
    },
    option {
        name: b"unlockships\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_8::UNLOCKSHIPS_OPT.0 as ::core::ffi::c_int,
    },
    option {
        name: b"headstart\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_8::HEADSTART_OPT.0 as ::core::ffi::c_int,
    },
    option {
        name: b"autobuttons\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_8::AUTOBUTT_OPT.0 as ::core::ffi::c_int,
    },
    option {
        name: b"infiniteru\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_8::INFINITERU_OPT.0 as ::core::ffi::c_int,
    },
    option {
        name: b"skipintro\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_8::SKIPINTRO_OPT.0 as ::core::ffi::c_int,
    },
    option {
        name: b"mainmenumusic\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_8::MENUMUS_OPT.0 as ::core::ffi::c_int,
    },
    option {
        name: b"nebulae\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_8::NEBU_OPT.0 as ::core::ffi::c_int,
    },
    option {
        name: b"orbitingplanets\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_8::ORBITS_OPT.0 as ::core::ffi::c_int,
    },
    option {
        name: b"texturedplanets\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_8::TEXTPLAN_OPT.0 as ::core::ffi::c_int,
    },
    option {
        name: b"dateformat\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 1 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_8::DATE_OPT.0 as ::core::ffi::c_int,
    },
    option {
        name: b"infinitefuel\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_8::INFFUEL_OPT.0 as ::core::ffi::c_int,
    },
    option {
        name: b"partialpickup\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_8::PICKUP_OPT.0 as ::core::ffi::c_int,
    },
    option {
        name: b"submenu\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_8::SUBMENU_OPT.0 as ::core::ffi::c_int,
    },
    option {
        name: b"infinitecredits\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_8::INFCRED_OPT.0 as ::core::ffi::c_int,
    },
    option {
        name: b"customborder\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_8::CUSTBORD_OPT.0 as ::core::ffi::c_int,
    },
    option {
        name: b"seedtype\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_8::SEEDTYPE_OPT.0 as ::core::ffi::c_int,
    },
    option {
        name: b"customseed\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 1 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_8::EXSEED_OPT.0 as ::core::ffi::c_int,
    },
    option {
        name: b"shipseed\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_8::SHIPSEED_OPT.0 as ::core::ffi::c_int,
    },
    option {
        name: b"spherecolors\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_8::SPHERECOLORS_OPT.0 as ::core::ffi::c_int,
    },
    option {
        name: b"spacemusic\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 1 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_8::SPACEMUSIC_OPT.0 as ::core::ffi::c_int,
    },
    option {
        name: b"wholefuel\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_8::WHOLEFUEL_OPT.0 as ::core::ffi::c_int,
    },
    option {
        name: b"dirjoystick\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 1 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_8::DIRJOY_OPT.0 as ::core::ffi::c_int,
    },
    option {
        name: b"landerhold\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_8::LANDHOLD_OPT.0 as ::core::ffi::c_int,
    },
    option {
        name: b"scrtrans\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 1 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_8::SCRTRANS_OPT.0 as ::core::ffi::c_int,
    },
    option {
        name: b"melee\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_8::MELEE_OPT.0 as ::core::ffi::c_int,
    },
    option {
        name: b"loadgame\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_8::LOADGAME_OPT.0 as ::core::ffi::c_int,
    },
    option {
        name: b"difficulty\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 1 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_8::DIFFICULTY_OPT.0 as ::core::ffi::c_int,
    },
    option {
        name: b"fuelrange\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 1 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_8::FUELRANGE_OPT.0 as ::core::ffi::c_int,
    },
    option {
        name: b"extended\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_8::EXTENDED_OPT.0 as ::core::ffi::c_int,
    },
    option {
        name: b"nomad\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 1 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_8::NOMAD_OPT.0 as ::core::ffi::c_int,
    },
    option {
        name: b"gameover\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_8::GAMEOVER_OPT.0 as ::core::ffi::c_int,
    },
    option {
        name: b"shipdirectionip\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_8::SHIPDIRIP_OPT.0 as ::core::ffi::c_int,
    },
    option {
        name: b"hazardcolors\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_8::HAZCOLORS_OPT.0 as ::core::ffi::c_int,
    },
    option {
        name: b"orzcompfont\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_8::ORZFONT_OPT.0 as ::core::ffi::c_int,
    },
    option {
        name: b"smartautopilot\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_8::SISFACEHS_OPT.0 as ::core::ffi::c_int,
    },
    option {
        name: b"tintplansphere\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 1 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_8::COLORPLAN_OPT.0 as ::core::ffi::c_int,
    },
    option {
        name: b"planetstyle\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 1 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_8::PLANSTYLE_OPT.0 as ::core::ffi::c_int,
    },
    option {
        name: b"starbackground\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 1 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_8::STARBACK_OPT.0 as ::core::ffi::c_int,
    },
    option {
        name: b"scanstyle\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 1 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_8::SCANSTYLE_OPT.0 as ::core::ffi::c_int,
    },
    option {
        name: b"nonstoposcill\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_8::OSCILLO_OPT.0 as ::core::ffi::c_int,
    },
    option {
        name: b"scopestyle\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 1 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_8::OSCSTYLE_OPT.0 as ::core::ffi::c_int,
    },
    option {
        name: b"animhyperstars\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_8::HYPERSTARS_OPT.0 as ::core::ffi::c_int,
    },
    option {
        name: b"landerview\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 1 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_8::LANDSTYLE_OPT.0 as ::core::ffi::c_int,
    },
    option {
        name: b"planettexture\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 1 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_8::PLANTEX_OPT.0 as ::core::ffi::c_int,
    },
    option {
        name: b"sisenginecolor\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 1 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_8::SISENGINE_OPT.0 as ::core::ffi::c_int,
    },
    option {
        name: b"nohqencounters\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_8::NOHSENC_OPT.0 as ::core::ffi::c_int,
    },
    option {
        name: b"decleanse\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_8::DECLEANSE_OPT.0 as ::core::ffi::c_int,
    },
    option {
        name: b"nomeleeobstacles\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_8::NOMELEEOBJ_OPT.0 as ::core::ffi::c_int,
    },
    option {
        name: b"showvisitstars\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_8::SHOWSTARS_OPT.0 as ::core::ffi::c_int,
    },
    option {
        name: b"unscaledstarsystem\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_8::UNSCALEDSS_OPT.0 as ::core::ffi::c_int,
    },
    option {
        name: b"spheretype\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 1 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_8::SCANSPH_OPT.0 as ::core::ffi::c_int,
    },
    option {
        name: b"nebulaevol\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 1 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_8::NEBUVOL_OPT.0 as ::core::ffi::c_int,
    },
    option {
        name: b"slaughtermode\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_8::SLAUGHTER_OPT.0 as ::core::ffi::c_int,
    },
    option {
        name: b"advancedautopilot\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_8::SISADVAP_OPT.0 as ::core::ffi::c_int,
    },
    option {
        name: b"meleetooltips\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_8::MELEETIPS_OPT.0 as ::core::ffi::c_int,
    },
    option {
        name: b"musicresume\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 1 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_8::MUSICRESUME_OPT.0 as ::core::ffi::c_int,
    },
    option {
        name: b"windowtype\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 1 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_8::WINDTYPE_OPT.0 as ::core::ffi::c_int,
    },
    option {
        name: b"noclassic\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_8::CLAPAK_OPT.0 as ::core::ffi::c_int,
    },
    option {
        name: b"scatterelements\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_8::SCATTERELEMS_OPT.0 as ::core::ffi::c_int,
    },
    option {
        name: b"showupgrades\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_8::SHOWUPG_OPT.0 as ::core::ffi::c_int,
    },
    option {
        name: b"fleetpointsys\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_8::FLTPTSYS_OPT.0 as ::core::ffi::c_int,
    },
    option {
        name: b"shipstore\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_8::SHIPSTORE_OPT.0 as ::core::ffi::c_int,
    },
    option {
        name: b"captainnames\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_8::CAPTNAMES_OPT.0 as ::core::ffi::c_int,
    },
    option {
        name: b"dosmenus\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_8::DOSMENUS_OPT.0 as ::core::ffi::c_int,
    },
    option {
        name: b"hyperspacecolor\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 1 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_8::HSCOLOR_OPT.0 as ::core::ffi::c_int,
    },
    option {
        name: b"deadzoneleftp1\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 1 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_8::DZLP1_OPT.0 as ::core::ffi::c_int,
    },
    option {
        name: b"deadzonerightp1\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 1 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_8::DZRP1_OPT.0 as ::core::ffi::c_int,
    },
    option {
        name: b"deadzoneleftp2\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 1 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_8::DZLP2_OPT.0 as ::core::ffi::c_int,
    },
    option {
        name: b"deadzonerightp2\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 1 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_8::DZRP2_OPT.0 as ::core::ffi::c_int,
    },
    option {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 0 as ::core::ffi::c_int,
    },
];
#[inline]
unsafe extern "C" fn setBoolOption(mut option: *mut bool_option, mut value: bool) {
    (*option).value = value;
    (*option).set = true;
}
unsafe extern "C" fn setFloatOption(
    mut option: *mut float_option,
    mut strval: *const ::core::ffi::c_char,
    mut optName: *const ::core::ffi::c_char,
) -> bool {
    if parseFloatOption(strval, &raw mut (*option).value, optName)
        != 0 as ::core::ffi::c_int
    {
        return false;
    }
    (*option).set = true;
    return true;
}
unsafe extern "C" fn setListOption(
    mut option: *mut int_option,
    mut strval: *const ::core::ffi::c_char,
    mut list: *const option_list_value,
) -> bool {
    let mut found: bool = false;
    if list.is_null() {
        return false;
    }
    found = lookupOptionValue(list, strval, &raw mut (*option).value);
    (*option).set = found;
    return found;
}
#[inline]
unsafe extern "C" fn setChoiceOption(
    mut option: *mut int_option,
    mut strval: *const ::core::ffi::c_char,
) -> bool {
    return setListOption(
        option,
        strval,
        &raw const choiceList as *const option_list_value,
    );
}
unsafe extern "C" fn setVolumeOption(
    mut option: *mut float_option,
    mut strval: *const ::core::ffi::c_char,
    mut optName: *const ::core::ffi::c_char,
) -> bool {
    let mut intVol: ::core::ffi::c_int = 0;
    if parseIntOption(strval, &raw mut intVol, optName) != 0 as ::core::ffi::c_int {
        return false;
    }
    parseIntVolume(intVol, &raw mut (*option).value);
    (*option).set = true;
    return true;
}
unsafe extern "C" fn parseOptions(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
    mut options: *mut options_struct,
) -> ::core::ffi::c_int {
    let mut optionIndex: ::core::ffi::c_int = 0;
    let mut badArg: bool = false;
    opterr = 0 as ::core::ffi::c_int;
    (*options).addons = HMalloc(
        (1 as size_t).wrapping_mul(::core::mem::size_of::<*const ::core::ffi::c_char>()),
    ) as *mut *const ::core::ffi::c_char;
    *(*options).addons.offset(0isize) = ::core::ptr::null::<::core::ffi::c_char>();
    (*options).numAddons = 0 as ::core::ffi::c_int;
    if argc == 0 as ::core::ffi::c_int {
        saveError(b"Error: Bad command line.\0".as_ptr() as *const ::core::ffi::c_char);
        return 1 as ::core::ffi::c_int;
    }
    while !badArg {
        let mut c: ::core::ffi::c_int = 0;
        optionIndex = -1 as ::core::ffi::c_int;
        c = getopt_long(
            argc,
            argv as *const *mut ::core::ffi::c_char,
            optString,
            &raw mut longOptions as *mut option,
            &raw mut optionIndex,
        );
        if c == -1 as ::core::ffi::c_int {
            break;
        }
        's_1864: {
            'c_23164: {
                match c {
                    63 => {
                        if optopt != '?' as ::core::ffi::c_int {
                            saveError(
                                b"\nInvalid option or its argument\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                            );
                            badArg = true;
                            break 's_1864;
                        }
                    }
                    104 => {}
                    118 => {
                        (*options).runMode = C2Rust_Unnamed_7::runMode_version;
                        return 0 as ::core::ffi::c_int;
                    }
                    114 => {
                        let mut width: ::core::ffi::c_int = 0;
                        let mut height: ::core::ffi::c_int = 0;
                        if sscanf(
                            optarg,
                            b"%dx%d\0".as_ptr() as *const ::core::ffi::c_char,
                            &raw mut width,
                            &raw mut height,
                        ) != 2 as ::core::ffi::c_int
                        {
                            saveError(
                                b"Error: invalid argument specified as resolution.\0"
                                    .as_ptr() as *const ::core::ffi::c_char,
                            );
                            badArg = true;
                            break 's_1864;
                        } else {
                            (*options).resolution.width = width;
                            (*options).resolution.height = height;
                            (*options).resolution.set = true;
                            break 's_1864;
                        }
                    }
                    102 => {
                        let mut temp: ::core::ffi::c_int = 0;
                        if parseIntOption(
                            optarg,
                            &raw mut temp,
                            b"Fullscreen\0".as_ptr() as *const ::core::ffi::c_char,
                        ) == -1 as ::core::ffi::c_int
                        {
                            badArg = true;
                            break 's_1864;
                        } else {
                            if temp < 0 as ::core::ffi::c_int
                                || temp > 2 as ::core::ffi::c_int
                            {
                                saveError(
                                    b"\nFullscreen has to be 0, 1, or 2.\n\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                );
                                badArg = true;
                            } else {
                                (*options).fullscreen.value = temp;
                                (*options).fullscreen.set = true;
                            }
                            break 's_1864;
                        }
                    }
                    119 => {
                        (*options).fullscreen.value = 0 as ::core::ffi::c_int;
                        (*options).fullscreen.set = true;
                        break 's_1864;
                    }
                    111 => {
                        setBoolOption(&raw mut (*options).opengl, true);
                        break 's_1864;
                    }
                    120 => {
                        setBoolOption(&raw mut (*options).opengl, false);
                        break 's_1864;
                    }
                    107 => {
                        setBoolOption(&raw mut (*options).keepAspectRatio, true);
                        break 's_1864;
                    }
                    99 => {
                        if !setListOption(
                            &raw mut (*options).scaler,
                            optarg,
                            &raw const scalerList as *const option_list_value,
                        ) {
                            InvalidArgument(
                                optarg,
                                b"--scale or -c\0".as_ptr() as *const ::core::ffi::c_char,
                            );
                            badArg = true;
                        }
                        break 's_1864;
                    }
                    98 => {
                        if !setListOption(
                            &raw mut (*options).meleeScale,
                            optarg,
                            &raw const meleeScaleList as *const option_list_value,
                        ) {
                            InvalidArgument(
                                optarg,
                                b"--meleezoom or -b\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                            );
                            badArg = true;
                        }
                        break 's_1864;
                    }
                    115 => {
                        setBoolOption(&raw mut (*options).scanlines, true);
                        break 's_1864;
                    }
                    112 => {
                        setBoolOption(&raw mut (*options).showFps, true);
                        break 's_1864;
                    }
                    110 => {
                        (*options).contentDir = optarg;
                        break 's_1864;
                    }
                    77 => {
                        if !setVolumeOption(
                            &raw mut (*options).musicVolumeScale,
                            optarg,
                            b"music volume\0".as_ptr() as *const ::core::ffi::c_char,
                        ) {
                            badArg = true;
                        }
                        break 's_1864;
                    }
                    83 => {
                        if !setVolumeOption(
                            &raw mut (*options).sfxVolumeScale,
                            optarg,
                            b"sfx volume\0".as_ptr() as *const ::core::ffi::c_char,
                        ) {
                            badArg = true;
                        }
                        break 's_1864;
                    }
                    84 => {
                        if !setVolumeOption(
                            &raw mut (*options).speechVolumeScale,
                            optarg,
                            b"speech volume\0".as_ptr() as *const ::core::ffi::c_char,
                        ) {
                            badArg = true;
                        }
                        break 's_1864;
                    }
                    113 => {
                        if !setListOption(
                            &raw mut (*options).soundQuality,
                            optarg,
                            &raw const audioQualityList as *const option_list_value,
                        ) {
                            InvalidArgument(
                                optarg,
                                b"--audioquality or -q\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                            );
                            badArg = true;
                        }
                        break 's_1864;
                    }
                    117 => {
                        setBoolOption(&raw mut (*options).subtitles, false);
                        break 's_1864;
                    }
                    103 => {
                        if !setFloatOption(
                            &raw mut (*options).gamma,
                            optarg,
                            b"gamma correction\0".as_ptr() as *const ::core::ffi::c_char,
                        ) {
                            badArg = true;
                        }
                        break 's_1864;
                    }
                    108 => {
                        (*options).logFile = optarg;
                        break 's_1864;
                    }
                    67 => {
                        (*options).configDir = optarg;
                        break 's_1864;
                    }
                    105 => {
                        if !setChoiceOption(&raw mut (*options).whichIntro, optarg) {
                            InvalidArgument(
                                optarg,
                                b"--intro or -i\0".as_ptr() as *const ::core::ffi::c_char,
                            );
                            badArg = true;
                        }
                        break 's_1864;
                    }
                    1000 => {
                        let mut temp_0: ::core::ffi::c_int = 0;
                        if parseIntOption(
                            optarg,
                            &raw mut temp_0,
                            b"Coarse Scans\0".as_ptr() as *const ::core::ffi::c_char,
                        ) == -1 as ::core::ffi::c_int
                        {
                            badArg = true;
                            break 's_1864;
                        } else {
                            if temp_0 < 0 as ::core::ffi::c_int
                                || temp_0 > 2 as ::core::ffi::c_int
                            {
                                saveError(
                                    b"\nCoarse Scan has to be 0, 1, 2 or 3.\n\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                );
                                badArg = true;
                            } else {
                                (*options).whichCoarseScan.value = temp_0;
                            }
                            break 's_1864;
                        }
                    }
                    1001 => {
                        if !setChoiceOption(&raw mut (*options).whichMenu, optarg) {
                            InvalidArgument(
                                optarg,
                                b"--menu\0".as_ptr() as *const ::core::ffi::c_char,
                            );
                            badArg = true;
                        }
                        break 's_1864;
                    }
                    1002 => {
                        if !setChoiceOption(&raw mut (*options).whichFonts, optarg) {
                            InvalidArgument(
                                optarg,
                                b"--font\0".as_ptr() as *const ::core::ffi::c_char,
                            );
                            badArg = true;
                        }
                        break 's_1864;
                    }
                    1003 => {
                        if !setChoiceOption(&raw mut (*options).whichShield, optarg) {
                            InvalidArgument(
                                optarg,
                                b"--shield\0".as_ptr() as *const ::core::ffi::c_char,
                            );
                            badArg = true;
                        }
                        break 's_1864;
                    }
                    1004 => {
                        if !setChoiceOption(&raw mut (*options).smoothScroll, optarg) {
                            InvalidArgument(
                                optarg,
                                b"--scroll\0".as_ptr() as *const ::core::ffi::c_char,
                            );
                            badArg = true;
                        }
                        break 's_1864;
                    }
                    1005 => {
                        if !setListOption(
                            &raw mut (*options).soundDriver,
                            optarg,
                            &raw const audioDriverList as *const option_list_value,
                        ) {
                            InvalidArgument(
                                optarg,
                                b"--sound\0".as_ptr() as *const ::core::ffi::c_char,
                            );
                            badArg = true;
                        }
                        break 's_1864;
                    }
                    1006 => {
                        setBoolOption(&raw mut (*options).stereoSFX, true);
                        break 's_1864;
                    }
                    1007 => {
                        (*options).numAddons += 1;
                        (*options).addons = HRealloc(
                            (*options).addons as *mut ::core::ffi::c_void,
                            (((*options).numAddons + 1 as ::core::ffi::c_int) as size_t)
                                .wrapping_mul(
                                    ::core::mem::size_of::<*const ::core::ffi::c_char>(),
                                ),
                        ) as *mut *const ::core::ffi::c_char;
                        *(*options)
                            .addons
                            .offset(
                                ((*options).numAddons - 1 as ::core::ffi::c_int) as isize,
                            ) = optarg;
                        *(*options).addons.offset((*options).numAddons as isize) = ::core::ptr::null::<
                            ::core::ffi::c_char,
                        >();
                        break 's_1864;
                    }
                    1008 => {
                        (*options).addonDir = optarg;
                        break 's_1864;
                    }
                    1009 => {
                        let mut value: ::core::ffi::c_int = 0;
                        if lookupOptionValue(
                            &raw const accelList as *const option_list_value,
                            optarg,
                            &raw mut value,
                        ) {
                            force_platform = PLATFORM_TYPE(value as ::core::ffi::c_uint);
                        } else {
                            InvalidArgument(
                                optarg,
                                b"--accel\0".as_ptr() as *const ::core::ffi::c_char,
                            );
                            badArg = true;
                        }
                        break 's_1864;
                    }
                    1010 => {
                        setBoolOption(&raw mut (*options).safeMode, true);
                        break 's_1864;
                    }
                    1011 => {
                        (*options).graphicsBackend = optarg;
                        break 's_1864;
                    }
                    1012 => {
                        setBoolOption(&raw mut (*options).cheatMode, true);
                        break 's_1864;
                    }
                    1013 => {
                        let mut temp_1: ::core::ffi::c_int = 0;
                        if parseIntOption(
                            optarg,
                            &raw mut temp_1,
                            b"God Modes\0".as_ptr() as *const ::core::ffi::c_char,
                        ) == -1 as ::core::ffi::c_int
                        {
                            badArg = true;
                            break 's_1864;
                        } else {
                            if temp_1 < 0 as ::core::ffi::c_int
                                || temp_1 > 2 as ::core::ffi::c_int
                            {
                                saveError(
                                    b"\nGod Mode has to be 0, 1, or 2.\n\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                );
                                badArg = true;
                            } else {
                                (*options).optGodModes.value = temp_1;
                                (*options).optGodModes.set = true;
                            }
                            break 's_1864;
                        }
                    }
                    1014 => {
                        let mut temp_2: ::core::ffi::c_int = 0;
                        if parseIntOption(
                            optarg,
                            &raw mut temp_2,
                            b"Time Dilation scale\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        ) == -1 as ::core::ffi::c_int
                        {
                            badArg = true;
                            break 's_1864;
                        } else {
                            if temp_2 < 0 as ::core::ffi::c_int
                                || temp_2 > 2 as ::core::ffi::c_int
                            {
                                saveError(
                                    b"\nTime Dilation scale has to be 0, 1, or 2.\n\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                );
                                badArg = true;
                            } else {
                                (*options).timeDilationScale.value = temp_2;
                                (*options).timeDilationScale.set = true;
                            }
                            break 's_1864;
                        }
                    }
                    1015 => {
                        setBoolOption(&raw mut (*options).bubbleWarp, true);
                        break 's_1864;
                    }
                    1016 => {
                        setBoolOption(&raw mut (*options).unlockShips, true);
                        break 's_1864;
                    }
                    1017 => {
                        setBoolOption(&raw mut (*options).headStart, true);
                        break 's_1864;
                    }
                    1018 => {
                        setBoolOption(&raw mut (*options).autoButtons, true);
                        break 's_1864;
                    }
                    1019 => {
                        setBoolOption(&raw mut (*options).infiniteRU, true);
                        break 's_1864;
                    }
                    1020 => {
                        setBoolOption(&raw mut (*options).skipIntro, true);
                        break 's_1864;
                    }
                    1021 => {
                        setBoolOption(&raw mut (*options).mainMenuMusic, true);
                        break 's_1864;
                    }
                    1022 => {
                        setBoolOption(&raw mut (*options).nebulae, true);
                        break 's_1864;
                    }
                    1023 => {
                        setBoolOption(&raw mut (*options).orbitingPlanets, true);
                        break 's_1864;
                    }
                    1024 => {
                        setBoolOption(&raw mut (*options).texturedPlanets, true);
                        break 's_1864;
                    }
                    1025 => {
                        let mut temp_3: ::core::ffi::c_int = 0;
                        if parseIntOption(
                            optarg,
                            &raw mut temp_3,
                            b"Date Format\0".as_ptr() as *const ::core::ffi::c_char,
                        ) == -1 as ::core::ffi::c_int
                        {
                            badArg = true;
                            break 's_1864;
                        } else {
                            if temp_3 < 0 as ::core::ffi::c_int
                                || temp_3 > 3 as ::core::ffi::c_int
                            {
                                saveError(
                                    b"\nDate Format has to be 0, 1, 2, or 3.\n\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                );
                                badArg = true;
                            } else {
                                (*options).optDateFormat.value = temp_3;
                                (*options).optDateFormat.set = true;
                            }
                            break 's_1864;
                        }
                    }
                    1026 => {
                        setBoolOption(&raw mut (*options).infiniteFuel, true);
                        break 's_1864;
                    }
                    1027 => {
                        setBoolOption(&raw mut (*options).partialPickup, true);
                        break 's_1864;
                    }
                    1028 => {
                        setBoolOption(&raw mut (*options).submenu, true);
                        break 's_1864;
                    }
                    1029 => {
                        setBoolOption(&raw mut (*options).infiniteCredits, true);
                        break 's_1864;
                    }
                    1030 => {
                        setBoolOption(&raw mut (*options).customBorder, true);
                        break 's_1864;
                    }
                    1031 => {
                        let mut temp_4: ::core::ffi::c_int = 0;
                        if parseIntOption(
                            optarg,
                            &raw mut temp_4,
                            b"Seed Type\0".as_ptr() as *const ::core::ffi::c_char,
                        ) == -1 as ::core::ffi::c_int
                        {
                            badArg = true;
                            break 's_1864;
                        } else {
                            if temp_4 < 0 as ::core::ffi::c_int
                                || temp_4 > 3 as ::core::ffi::c_int
                            {
                                saveError(
                                    b"\nSeed Type has to be 0, 1, 2, or 3.\n\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                );
                                badArg = true;
                            } else {
                                (*options).seedType.value = temp_4;
                                (*options).seedType.set = true;
                            }
                            break 's_1864;
                        }
                    }
                    1032 => {
                        let mut temp_5: ::core::ffi::c_int = 0;
                        if parseIntOption(
                            optarg,
                            &raw mut temp_5,
                            b"Custom Seed\0".as_ptr() as *const ::core::ffi::c_char,
                        ) == -1 as ::core::ffi::c_int
                        {
                            badArg = true;
                            break 's_1864;
                        } else {
                            if if temp_5 < 3 as ::core::ffi::c_int
                                || temp_5 > 2147483644 as ::core::ffi::c_int
                            {
                                0 as ::core::ffi::c_int
                            } else {
                                1 as ::core::ffi::c_int
                            } == 0
                            {
                                saveError(
                                    b"\nCustom Seed can not be less than %d or greater than %d.\n\0"
                                        .as_ptr() as *const ::core::ffi::c_char,
                                    3 as ::core::ffi::c_int,
                                    2147483644 as ::core::ffi::c_int,
                                );
                                badArg = true;
                            } else {
                                (*options).customSeed.value = temp_5;
                                (*options).customSeed.set = true;
                            }
                            break 's_1864;
                        }
                    }
                    1033 => {
                        setBoolOption(&raw mut (*options).shipSeed, true);
                        break 's_1864;
                    }
                    1034 => {
                        let mut temp_6: ::core::ffi::c_int = 0;
                        if parseIntOption(
                            optarg,
                            &raw mut temp_6,
                            b"Sphere Colors\0".as_ptr() as *const ::core::ffi::c_char,
                        ) == -1 as ::core::ffi::c_int
                        {
                            badArg = true;
                            break 's_1864;
                        } else {
                            if temp_6 < 0 as ::core::ffi::c_int
                                || temp_6 > 1 as ::core::ffi::c_int
                            {
                                saveError(
                                    b"\nSphere Colors has to 0 or 1.\n\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                );
                                badArg = true;
                            } else {
                                (*options).sphereColors.value = temp_6;
                                (*options).sphereColors.set = true;
                            }
                            break 's_1864;
                        }
                    }
                    1035 => {
                        let mut temp_7: ::core::ffi::c_int = 0;
                        if parseIntOption(
                            optarg,
                            &raw mut temp_7,
                            b"Ambient Space Music\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        ) == -1 as ::core::ffi::c_int
                        {
                            badArg = true;
                            break 's_1864;
                        } else {
                            if temp_7 < 0 as ::core::ffi::c_int
                                || temp_7 > 2 as ::core::ffi::c_int
                            {
                                saveError(
                                    b"\nAmbient Space Music has to be 0, 1, or 2.\n\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                );
                                badArg = true;
                            } else {
                                (*options).spaceMusic.value = temp_7;
                                (*options).spaceMusic.set = true;
                            }
                            break 's_1864;
                        }
                    }
                    1036 => {
                        setBoolOption(&raw mut (*options).wholeFuel, true);
                        break 's_1864;
                    }
                    1037 => {
                        let mut temp_8: ::core::ffi::c_int = 0;
                        if parseIntOption(
                            optarg,
                            &raw mut temp_8,
                            b"Directional Joystick\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        ) == -1 as ::core::ffi::c_int
                        {
                            badArg = true;
                            break 's_1864;
                        } else {
                            if temp_8 < 0 as ::core::ffi::c_int
                                || temp_8 > 2 as ::core::ffi::c_int
                            {
                                saveError(
                                    b"\nDirectional Joystick has to be 0, 1, 2, 3, or 4.\n\0"
                                        .as_ptr() as *const ::core::ffi::c_char,
                                );
                                badArg = true;
                            } else {
                                (*options).dirJoyP1.value = temp_8;
                                (*options).dirJoyP1.set = true;
                            }
                            break 's_1864;
                        }
                    }
                    1038 => {
                        if !setChoiceOption(&raw mut (*options).landerHold, optarg) {
                            InvalidArgument(
                                optarg,
                                b"--landerhold\0".as_ptr() as *const ::core::ffi::c_char,
                            );
                            badArg = true;
                        }
                        break 's_1864;
                    }
                    1039 => {
                        if !setChoiceOption(&raw mut (*options).scrTrans, optarg) {
                            InvalidArgument(
                                optarg,
                                b"--scrtrans\0".as_ptr() as *const ::core::ffi::c_char,
                            );
                            badArg = true;
                        }
                        break 's_1864;
                    }
                    1040 => {
                        let mut temp_9: ::core::ffi::c_int = 0;
                        if parseIntOption(
                            optarg,
                            &raw mut temp_9,
                            b"Difficulty\0".as_ptr() as *const ::core::ffi::c_char,
                        ) == -1 as ::core::ffi::c_int
                        {
                            badArg = true;
                            break 's_1864;
                        } else {
                            if temp_9 < 0 as ::core::ffi::c_int
                                || temp_9 > 3 as ::core::ffi::c_int
                            {
                                saveError(
                                    b"\nDifficulty has to be 0, 1, 2, or 3.\n\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                );
                                badArg = true;
                            } else {
                                (*options).optDifficulty.value = temp_9;
                                (*options).optDiffChooser.value = (*options)
                                    .optDifficulty
                                    .value;
                                if temp_9 > 2 as ::core::ffi::c_int {
                                    (*options).optDifficulty.value = 0 as ::core::ffi::c_int;
                                }
                                (*options).optDifficulty.set = true;
                            }
                            break 's_1864;
                        }
                    }
                    1041 => {
                        let mut temp_10: ::core::ffi::c_int = 0;
                        if parseIntOption(
                            optarg,
                            &raw mut temp_10,
                            b"Fuel range indicator\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        ) == -1 as ::core::ffi::c_int
                        {
                            badArg = true;
                            break 's_1864;
                        } else {
                            if temp_10 < 0 as ::core::ffi::c_int
                                || temp_10 > 3 as ::core::ffi::c_int
                            {
                                saveError(
                                    b"\nFuel range indicator has to be 0, 1, 2, or 3.\n\0"
                                        .as_ptr() as *const ::core::ffi::c_char,
                                );
                                badArg = true;
                            } else {
                                (*options).optFuelRange.value = temp_10;
                                (*options).optFuelRange.set = true;
                            }
                            break 's_1864;
                        }
                    }
                    1042 => {
                        setBoolOption(&raw mut (*options).extended, true);
                        break 's_1864;
                    }
                    1043 => {
                        let mut temp_11: ::core::ffi::c_int = 0;
                        if parseIntOption(
                            optarg,
                            &raw mut temp_11,
                            b"Nomad Mode type\0".as_ptr() as *const ::core::ffi::c_char,
                        ) == -1 as ::core::ffi::c_int
                        {
                            badArg = true;
                            break 's_1864;
                        } else {
                            if temp_11 < 0 as ::core::ffi::c_int
                                || temp_11 > 2 as ::core::ffi::c_int
                            {
                                saveError(
                                    b"\nNomad Mode has to be 0, 1, or 2.\n\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                );
                                badArg = true;
                            } else {
                                (*options).nomad.value = temp_11;
                                (*options).nomad.set = true;
                            }
                            break 's_1864;
                        }
                    }
                    1044 => {
                        setBoolOption(&raw mut (*options).gameOver, true);
                        break 's_1864;
                    }
                    1045 => {
                        setBoolOption(&raw mut (*options).shipDirectionIP, true);
                        break 's_1864;
                    }
                    1046 => {
                        setBoolOption(&raw mut (*options).hazardColors, true);
                        break 's_1864;
                    }
                    1047 => {
                        setBoolOption(&raw mut (*options).orzCompFont, true);
                        break 's_1864;
                    }
                    1048 => {
                        let mut temp_12: ::core::ffi::c_int = 0;
                        if parseIntOption(
                            optarg,
                            &raw mut temp_12,
                            b"Controller Type\0".as_ptr() as *const ::core::ffi::c_char,
                        ) == -1 as ::core::ffi::c_int
                        {
                            badArg = true;
                            break 's_1864;
                        } else {
                            if temp_12 < 0 as ::core::ffi::c_int
                                || temp_12 > 2 as ::core::ffi::c_int
                            {
                                saveError(
                                    b"\nController type has to be 0, 1, or 2.\n\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                );
                                badArg = true;
                            } else {
                                (*options).optControllerType.value = temp_12;
                                (*options).optControllerType.set = true;
                            }
                            break 's_1864;
                        }
                    }
                    1049 => {
                        setBoolOption(&raw mut (*options).smartAutoPilot, true);
                        break 's_1864;
                    }
                    1050 => {
                        if !setChoiceOption(&raw mut (*options).tintPlanSphere, optarg) {
                            InvalidArgument(
                                optarg,
                                b"--tintplansphere\0".as_ptr() as *const ::core::ffi::c_char,
                            );
                            badArg = true;
                        }
                        break 's_1864;
                    }
                    1051 => {
                        if !setChoiceOption(&raw mut (*options).planetStyle, optarg) {
                            InvalidArgument(
                                optarg,
                                b"--planetstyle\0".as_ptr() as *const ::core::ffi::c_char,
                            );
                            badArg = true;
                        }
                        break 's_1864;
                    }
                    1052 => {
                        let mut temp_13: ::core::ffi::c_int = 0;
                        if parseIntOption(
                            optarg,
                            &raw mut temp_13,
                            b"Star Background\0".as_ptr() as *const ::core::ffi::c_char,
                        ) == -1 as ::core::ffi::c_int
                        {
                            badArg = true;
                            break 's_1864;
                        } else {
                            if temp_13 < 0 as ::core::ffi::c_int
                                || temp_13 > 3 as ::core::ffi::c_int
                            {
                                saveError(
                                    b"\nStar background has to be between 0-3.\n\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                );
                                badArg = true;
                            } else {
                                (*options).starBackground.value = temp_13;
                                (*options).starBackground.set = true;
                            }
                            break 's_1864;
                        }
                    }
                    1053 => {
                        if !setChoiceOption(&raw mut (*options).scanStyle, optarg) {
                            InvalidArgument(
                                optarg,
                                b"--scanstyle\0".as_ptr() as *const ::core::ffi::c_char,
                            );
                            badArg = true;
                        }
                        break 's_1864;
                    }
                    1054 => {
                        setBoolOption(&raw mut (*options).nonStopOscill, true);
                        break 's_1864;
                    }
                    1055 => {
                        if !setChoiceOption(&raw mut (*options).scopeStyle, optarg) {
                            InvalidArgument(
                                optarg,
                                b"--scopestyle\0".as_ptr() as *const ::core::ffi::c_char,
                            );
                            badArg = true;
                        }
                        break 's_1864;
                    }
                    1056 => {
                        setBoolOption(&raw mut (*options).hyperStars, true);
                        break 's_1864;
                    }
                    1057 => {
                        if !setChoiceOption(&raw mut (*options).landerStyle, optarg) {
                            InvalidArgument(
                                optarg,
                                b"--landerview\0".as_ptr() as *const ::core::ffi::c_char,
                            );
                            badArg = true;
                        }
                        break 's_1864;
                    }
                    1058 => {
                        if strcmp(
                            optarg,
                            b"uqm\0".as_ptr() as *const ::core::ffi::c_char,
                        ) == 0 as ::core::ffi::c_int
                        {
                            setBoolOption(&raw mut (*options).planetTexture, true);
                        } else if strcmp(
                            optarg,
                            b"3do\0".as_ptr() as *const ::core::ffi::c_char,
                        ) == 0 as ::core::ffi::c_int
                        {
                            setBoolOption(&raw mut (*options).planetTexture, false);
                        } else {
                            InvalidArgument(
                                optarg,
                                b"--planettexture\0".as_ptr() as *const ::core::ffi::c_char,
                            );
                            saveError(
                                b"\nPlanet Texture can only be set to '3do' or 'uqm'\n\0"
                                    .as_ptr() as *const ::core::ffi::c_char,
                            );
                            badArg = true;
                        }
                        break 's_1864;
                    }
                    1059 => {
                        if !setChoiceOption(&raw mut (*options).flagshipColor, optarg) {
                            InvalidArgument(
                                optarg,
                                b"--sisenginecolor\0".as_ptr() as *const ::core::ffi::c_char,
                            );
                            badArg = true;
                        }
                        break 's_1864;
                    }
                    1060 => {
                        setBoolOption(&raw mut (*options).noHQEncounters, true);
                        break 's_1864;
                    }
                    1061 => {
                        setBoolOption(&raw mut (*options).deCleansing, true);
                        break 's_1864;
                    }
                    1062 => {
                        setBoolOption(&raw mut (*options).meleeObstacles, true);
                        break 's_1864;
                    }
                    1063 => {
                        setBoolOption(&raw mut (*options).showVisitedStars, true);
                        break 's_1864;
                    }
                    1064 => {
                        setBoolOption(&raw mut (*options).unscaledStarSystem, true);
                        break 's_1864;
                    }
                    1065 => {
                        let mut temp_14: ::core::ffi::c_int = 0;
                        if parseIntOption(
                            optarg,
                            &raw mut temp_14,
                            b"Sphere Type\0".as_ptr() as *const ::core::ffi::c_char,
                        ) == -1 as ::core::ffi::c_int
                        {
                            badArg = true;
                            break 's_1864;
                        } else {
                            if temp_14 < 0 as ::core::ffi::c_int
                                || temp_14 > 2 as ::core::ffi::c_int
                            {
                                saveError(
                                    b"\nSphere Type has to be between 0-2\n\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                );
                                badArg = true;
                            } else {
                                (*options).sphereType.value = temp_14;
                                (*options).sphereType.set = true;
                            }
                            break 's_1864;
                        }
                    }
                    1066 => {
                        setBoolOption(&raw mut (*options).slaughterMode, true);
                        break 's_1864;
                    }
                    1067 => {
                        setBoolOption(&raw mut (*options).advancedAutoPilot, true);
                        break 's_1864;
                    }
                    1068 => {
                        setBoolOption(&raw mut (*options).meleeToolTips, true);
                        break 's_1864;
                    }
                    1069 => {
                        let mut temp_15: ::core::ffi::c_int = 0;
                        if parseIntOption(
                            optarg,
                            &raw mut temp_15,
                            b"Music Resume\0".as_ptr() as *const ::core::ffi::c_char,
                        ) == -1 as ::core::ffi::c_int
                        {
                            badArg = true;
                            break 's_1864;
                        } else {
                            if temp_15 < 0 as ::core::ffi::c_int
                                || temp_15 > 2 as ::core::ffi::c_int
                            {
                                saveError(
                                    b"\nMusic Resume has to be between 0-2\n\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                );
                                badArg = true;
                            } else {
                                (*options).musicResume.value = temp_15;
                                (*options).musicResume.set = true;
                            }
                            break 's_1864;
                        }
                    }
                    1070 => {
                        let mut temp_16: ::core::ffi::c_int = 0;
                        if parseIntOption(
                            optarg,
                            &raw mut temp_16,
                            b"Window Type\0".as_ptr() as *const ::core::ffi::c_char,
                        ) == -1 as ::core::ffi::c_int
                        {
                            badArg = true;
                            break 's_1864;
                        } else {
                            if temp_16 < 0 as ::core::ffi::c_int
                                || temp_16 > 2 as ::core::ffi::c_int
                            {
                                saveError(
                                    b"\nWindow type has to be between 0-2\n\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                );
                                badArg = true;
                            } else {
                                (*options).windowType.value = temp_16;
                                (*options).windowType.set = true;
                            }
                            break 's_1864;
                        }
                    }
                    1071 => {
                        setBoolOption(&raw mut (*options).scatterElements, true);
                        break 's_1864;
                    }
                    1072 => {
                        setBoolOption(&raw mut (*options).showUpgrades, true);
                        break 's_1864;
                    }
                    1073 => {
                        setBoolOption(&raw mut (*options).fleetPointSys, true);
                        break 's_1864;
                    }
                    1074 => {
                        setBoolOption(&raw mut (*options).shipStore, true);
                        break 's_1864;
                    }
                    1075 => {
                        setBoolOption(&raw mut (*options).captainNames, true);
                        break 's_1864;
                    }
                    1076 => {
                        setBoolOption(&raw mut (*options).dosMenus, true);
                        break 's_1864;
                    }
                    1081 => {
                        if !setChoiceOption(&raw mut (*options).flagshipColor, optarg) {
                            InvalidArgument(
                                optarg,
                                b"--hyperspacecolor\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                            );
                            badArg = true;
                        }
                        break 's_1864;
                    }
                    1077 => {
                        optSuperMelee = BOOLEAN::TRUE;
                        break 's_1864;
                    }
                    1078 => {
                        optLoadGame = BOOLEAN::TRUE;
                        break 's_1864;
                    }
                    1079 => {
                        let mut temp_17: ::core::ffi::c_int = 0;
                        if parseIntOption(
                            optarg,
                            &raw mut temp_17,
                            b"Nebulae Volume\0".as_ptr() as *const ::core::ffi::c_char,
                        ) == -1 as ::core::ffi::c_int
                        {
                            badArg = true;
                            break 's_1864;
                        } else {
                            if temp_17 < 0 as ::core::ffi::c_int
                                || temp_17 > 50 as ::core::ffi::c_int
                            {
                                saveError(
                                    b"\nNebulae volume has to be between 0-50\n\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                );
                                badArg = true;
                            } else {
                                (*options).nebulaevol.value = temp_17;
                                (*options).nebulaevol.set = true;
                            }
                            break 's_1864;
                        }
                    }
                    1082 | 1083 | 1084 | 1085 => {
                        saveError(
                            b"\nEditing Deadzone in commandline has not yet been implemented\n\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                        );
                        badArg = true;
                        break 'c_23164;
                    }
                    1080 => {
                        break 'c_23164;
                    }
                    _ => {
                        saveError(
                            b"Error: Unknown option '%s'\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            if optionIndex < 0 as ::core::ffi::c_int {
                                b"<unknown>\0".as_ptr() as *const ::core::ffi::c_char
                            } else {
                                longOptions[optionIndex as usize].name
                            },
                        );
                        badArg = true;
                        break 's_1864;
                    }
                }
                (*options).runMode = C2Rust_Unnamed_7::runMode_usage;
                return 0 as ::core::ffi::c_int;
            }
            optNoClassic = BOOLEAN::TRUE;
        }
    }
    if !badArg && optind != argc {
        saveError(
            b"\nError: Extra arguments found on the command line.\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        badArg = true;
    }
    return if badArg as ::core::ffi::c_int != 0 {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    };
}
unsafe extern "C" fn parseIntVolume(
    mut intVol: ::core::ffi::c_int,
    mut vol: *mut ::core::ffi::c_float,
) {
    if intVol < 0 as ::core::ffi::c_int {
        *vol = 0.0f32;
        return;
    }
    if intVol > 100 as ::core::ffi::c_int {
        *vol = 1.0f32;
        return;
    }
    *vol = intVol as ::core::ffi::c_float / 100.0f32;
}
unsafe extern "C" fn parseIntOption(
    mut str: *const ::core::ffi::c_char,
    mut result: *mut ::core::ffi::c_int,
    mut optName: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut endPtr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut temp: ::core::ffi::c_int = 0;
    if str.is_null()
        || *str.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
    {
        saveError(
            b"Error: Invalid value for '%s'.\0".as_ptr() as *const ::core::ffi::c_char,
            optName,
        );
        return -1 as ::core::ffi::c_int;
    }
    temp = strtol(str, &raw mut endPtr, 10 as ::core::ffi::c_int) as ::core::ffi::c_int;
    if *endPtr as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        saveError(
            b"Error: Junk characters in argument '%s'.\0".as_ptr()
                as *const ::core::ffi::c_char,
            optName,
        );
        return -1 as ::core::ffi::c_int;
    }
    *result = temp;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn parseFloatOption(
    mut str: *const ::core::ffi::c_char,
    mut f: *mut ::core::ffi::c_float,
    mut optName: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut endPtr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut temp: ::core::ffi::c_float = 0.;
    if *str.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
        saveError(
            b"Error: Invalid value for '%s'.\0".as_ptr() as *const ::core::ffi::c_char,
            optName,
        );
        return -1 as ::core::ffi::c_int;
    }
    temp = strtod(str, &raw mut endPtr) as ::core::ffi::c_float;
    if *endPtr as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        saveError(
            b"Error: Junk characters in argument '%s'.\0".as_ptr()
                as *const ::core::ffi::c_char,
            optName,
        );
        return -1 as ::core::ffi::c_int;
    }
    *f = temp;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn usage(mut out: *mut FILE, mut defaults: *const options_struct) {
    let mut old: *mut FILE = log_setOutput(out);
    log_captureLines(1000000 as ::core::ffi::c_int);
    log_add(log_Level::log_User, b"Options:\0".as_ptr() as *const ::core::ffi::c_char);
    log_add(
        log_Level::log_User,
        b"  -r, --res=WIDTHxHEIGHT (default: 640x480, bigger works only with --opengl)\0"
            .as_ptr() as *const ::core::ffi::c_char,
    );
    log_add(
        log_Level::log_User,
        b"  -f, --fullscreen (default: 0)\0".as_ptr() as *const ::core::ffi::c_char,
    );
    log_add(
        log_Level::log_User,
        b"  -w, --windowed (default: true)\0".as_ptr() as *const ::core::ffi::c_char,
    );
    log_add(
        log_Level::log_User,
        b"  -o, --opengl (default: %s)\0".as_ptr() as *const ::core::ffi::c_char,
        boolOptString(&raw const (*defaults).opengl),
    );
    log_add(
        log_Level::log_User,
        b"  -x, --nogl (default: %s)\0".as_ptr() as *const ::core::ffi::c_char,
        boolNotOptString(&raw const (*defaults).opengl),
    );
    log_add(
        log_Level::log_User,
        b"  -k, --keepaspectratio (default: %s)\0".as_ptr()
            as *const ::core::ffi::c_char,
        boolOptString(&raw const (*defaults).keepAspectRatio),
    );
    log_add(
        log_Level::log_User,
        b"  -c, --scale=MODE (bilinear, biadapt, biadv, triscan, hq or none (default) )\0"
            .as_ptr() as *const ::core::ffi::c_char,
    );
    log_add(
        log_Level::log_User,
        b"  -b, --meleezoom=MODE (step, aka pc, or smooth, aka 3do; default is 3do)\0"
            .as_ptr() as *const ::core::ffi::c_char,
    );
    log_add(
        log_Level::log_User,
        b"  -s, --scanlines (default: %s)\0".as_ptr() as *const ::core::ffi::c_char,
        boolOptString(&raw const (*defaults).scanlines),
    );
    log_add(
        log_Level::log_User,
        b"  -p, --fps (default: %s)\0".as_ptr() as *const ::core::ffi::c_char,
        boolOptString(&raw const (*defaults).showFps),
    );
    log_add(
        log_Level::log_User,
        b"  -g, --gamma=CORRECTIONVALUE (default: 1.0, which causes no change)\0"
            .as_ptr() as *const ::core::ffi::c_char,
    );
    log_add(
        log_Level::log_User,
        b"  -C, --configdir=CONFIGDIR\0".as_ptr() as *const ::core::ffi::c_char,
    );
    log_add(
        log_Level::log_User,
        b"  -n, --contentdir=CONTENTDIR\0".as_ptr() as *const ::core::ffi::c_char,
    );
    log_add(
        log_Level::log_User,
        b"  -M, --musicvol=VOLUME (0-100, default 100)\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    log_add(
        log_Level::log_User,
        b"  -S, --sfxvol=VOLUME (0-100, default 100)\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    log_add(
        log_Level::log_User,
        b"  -T, --speechvol=VOLUME (0-100, default 100)\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    log_add(
        log_Level::log_User,
        b"  -q, --audioquality=QUALITY (high, medium or low, default medium)\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    log_add(
        log_Level::log_User,
        b"  -u, --nosubtitles\0".as_ptr() as *const ::core::ffi::c_char,
    );
    log_add(
        log_Level::log_User,
        b"  -l, --logfile=FILE (sends console output to logfile FILE)\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    log_add(
        log_Level::log_User,
        b"  --addon ADDON (using a specific addon; may be specified multiple times)\0"
            .as_ptr() as *const ::core::ffi::c_char,
    );
    log_add(
        log_Level::log_User,
        b"  --addondir=ADDONDIR (directory where addons reside)\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    log_add(
        log_Level::log_User,
        b"  --renderer=name (Select named rendering engine if possible)\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    log_add(
        log_Level::log_User,
        b"  --sound=DRIVER (openal, mixsdl, none; default mixsdl)\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    log_add(
        log_Level::log_User,
        b"  --stereosfx (enables positional sound effects, currently only for openal)\0"
            .as_ptr() as *const ::core::ffi::c_char,
    );
    log_add(
        log_Level::log_User,
        b"  --safe (start in safe mode)\0".as_ptr() as *const ::core::ffi::c_char,
    );
    log_add(
        log_Level::log_User,
        b"The following options can take either '3do' or 'pc' as an option:\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    log_add(
        log_Level::log_User,
        b"  -i, --intro : Intro/ending version (default: %s)\0".as_ptr()
            as *const ::core::ffi::c_char,
        choiceOptString(&raw const (*defaults).whichIntro),
    );
    log_add(
        log_Level::log_User,
        b"  --cscan     : coarse-scan display, pc=text, 3do=hieroglyphs (default: %s)\0"
            .as_ptr() as *const ::core::ffi::c_char,
        choiceOptString(&raw const (*defaults).whichCoarseScan),
    );
    log_add(
        log_Level::log_User,
        b"  --menu      : menu type, pc=text, 3do=graphical (default: %s)\0".as_ptr()
            as *const ::core::ffi::c_char,
        choiceOptString(&raw const (*defaults).whichMenu),
    );
    log_add(
        log_Level::log_User,
        b"  --font      : font types and colors (default: %s)\0".as_ptr()
            as *const ::core::ffi::c_char,
        choiceOptString(&raw const (*defaults).whichFonts),
    );
    log_add(
        log_Level::log_User,
        b"  --shield    : slave shield type; pc=static, 3do=throbbing (default: %s)\0"
            .as_ptr() as *const ::core::ffi::c_char,
        choiceOptString(&raw const (*defaults).whichShield),
    );
    log_add(
        log_Level::log_User,
        b"  --scroll    : ff/frev during comm.  pc=per-page, 3do=smooth (default: %s)\0"
            .as_ptr() as *const ::core::ffi::c_char,
        choiceOptString(&raw const (*defaults).smoothScroll),
    );
    log_add(
        log_Level::log_User,
        b"\nThe following options are MegaMod specific\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    log_add(
        log_Level::log_User,
        b"  --kohrstahp : Stops Kohr-Ah advancing. (default: %s)\0".as_ptr()
            as *const ::core::ffi::c_char,
        boolOptString(&raw const (*defaults).cheatMode),
    );
    log_add(
        log_Level::log_User,
        b"  --precursormode : =1 Infinite ship battery. =2 No damage=3 Infinite ship battery and no damage (default: 0)\0"
            .as_ptr() as *const ::core::ffi::c_char,
    );
    log_add(
        log_Level::log_User,
        b"  --timedilation : =1 Time is slowed down times 6. =2 Time is sped up times 5 (default: 0)\0"
            .as_ptr() as *const ::core::ffi::c_char,
    );
    log_add(
        log_Level::log_User,
        b"  --bubblewarp : Instantaneous travel to any point on the Starmap. (default: %s)\0"
            .as_ptr() as *const ::core::ffi::c_char,
        boolOptString(&raw const (*defaults).bubbleWarp),
    );
    log_add(
        log_Level::log_User,
        b"  --unlockships : Allows you to purchase ships that youcan't normally acquire in the main game. (default: %s)\0"
            .as_ptr() as *const ::core::ffi::c_char,
        boolOptString(&raw const (*defaults).unlockShips),
    );
    log_add(
        log_Level::log_User,
        b"  --headstart : Gives you an extra storage bay full ofminerals, Fwiffo, and the Moonbase during a new game (default: %s)\0"
            .as_ptr() as *const ::core::ffi::c_char,
        boolOptString(&raw const (*defaults).headStart),
    );
    log_add(
        log_Level::log_User,
        b"  --autobuttons : Automatically detect gamepad button type to display (default: %s)\0"
            .as_ptr() as *const ::core::ffi::c_char,
        boolOptString(&raw const (*defaults).autoButtons),
    );
    log_add(
        log_Level::log_User,
        b"  --infiniteru : Gives you infinite R.U. as long as thecheat is on (default: %s)\0"
            .as_ptr() as *const ::core::ffi::c_char,
        boolOptString(&raw const (*defaults).infiniteRU),
    );
    log_add(
        log_Level::log_User,
        b"  --skipintro : Skips the intro and Logo fmv (default: %s)\0".as_ptr()
            as *const ::core::ffi::c_char,
        boolOptString(&raw const (*defaults).skipIntro),
    );
    log_add(
        log_Level::log_User,
        b"  --mainmenumusic : Switches the main menumusic on/off (default: %s)\0"
            .as_ptr() as *const ::core::ffi::c_char,
        boolOptString(&raw const (*defaults).mainMenuMusic),
    );
    log_add(
        log_Level::log_User,
        b"  --nebulae : Enables/Disables nebulae instar systems (default: %s)\0".as_ptr()
            as *const ::core::ffi::c_char,
        boolOptString(&raw const (*defaults).nebulae),
    );
    log_add(
        log_Level::log_User,
        b"  --orbitingplanets : Enables/Disables orbitingplanets in star systems (default: %s)\0"
            .as_ptr() as *const ::core::ffi::c_char,
        boolOptString(&raw const (*defaults).orbitingPlanets),
    );
    log_add(
        log_Level::log_User,
        b"  --texturedplanets : Enables/Disables texturedplanets in star systems (default: %s)\0"
            .as_ptr() as *const ::core::ffi::c_char,
        boolOptString(&raw const (*defaults).texturedPlanets),
    );
    log_add(
        log_Level::log_User,
        b"  --infinitefuel : Infinite fuel in themain game (default: %s)\0".as_ptr()
            as *const ::core::ffi::c_char,
        boolOptString(&raw const (*defaults).infiniteFuel),
    );
    log_add(
        log_Level::log_User,
        b"  --partialpickup : Enables/Disables partialmineral pickup  (default: %s)\0"
            .as_ptr() as *const ::core::ffi::c_char,
        boolOptString(&raw const (*defaults).partialPickup),
    );
    log_add(
        log_Level::log_User,
        b"  --submenu : Enables/Disables mineral and starmap keys submenu  (default: %s)\0"
            .as_ptr() as *const ::core::ffi::c_char,
        boolOptString(&raw const (*defaults).submenu),
    );
    log_add(
        log_Level::log_User,
        b"  --dateformat : 0: MMM DD.YYYY | 1: MM.DD.YYYY | 2: DD MMM.YYYY | 3: DD.MM.YYYY (default: 0)\0"
            .as_ptr() as *const ::core::ffi::c_char,
    );
    log_add(
        log_Level::log_User,
        b"  --infinitecredits: Gives you infinite MelnormeCredits  (default: %s)\0"
            .as_ptr() as *const ::core::ffi::c_char,
        boolOptString(&raw const (*defaults).infiniteCredits),
    );
    log_add(
        log_Level::log_User,
        b"  --melee : Takes you straight to Super Meleeafter the splash screen.\0"
            .as_ptr() as *const ::core::ffi::c_char,
    );
    log_add(
        log_Level::log_User,
        b"  --loadgame : Takes you straight to the LoadGame sceen after the splash screen.\0"
            .as_ptr() as *const ::core::ffi::c_char,
    );
    log_add(
        log_Level::log_User,
        b"  --customborder : Enables the custom borderframe. (default: %s)\0".as_ptr()
            as *const ::core::ffi::c_char,
        boolOptString(&raw const (*defaults).customBorder),
    );
    log_add(
        log_Level::log_User,
        b"  --seedtype: 0: Default seed | 1: Seed planets | 2: Seed Melnorme/Rainbow/Quasispace | 3: Seed Starmap (default: 0)\0"
            .as_ptr() as *const ::core::ffi::c_char,
    );
    log_add(
        log_Level::log_User,
        b"  --customseed=# : Allows you to customize the internal seed used to generate the solar systems in-game. (default: 16807)\0"
            .as_ptr() as *const ::core::ffi::c_char,
    );
    log_add(
        log_Level::log_User,
        b"  --shipseed: Seed the ships assigned to each race. Uses --customseed value (default %s)\0"
            .as_ptr() as *const ::core::ffi::c_char,
        boolOptString(&raw const (*defaults).shipSeed),
    );
    log_add(
        log_Level::log_User,
        b"  --spherecolors: 0: Default colors | 1: StarSeed colors (default: 0)\0"
            .as_ptr() as *const ::core::ffi::c_char,
    );
    log_add(
        log_Level::log_User,
        b"  --spacemusic #: Enables localized music for aliens when you are in their sphere of influence\n0: Default (OFF) | 1: No Spoilers | 2: Spoilers\0"
            .as_ptr() as *const ::core::ffi::c_char,
    );
    log_add(
        log_Level::log_User,
        b"  --wholefuel : Enables the display of the whole fuel value in the ship status (default: %s)\0"
            .as_ptr() as *const ::core::ffi::c_char,
        boolOptString(&raw const (*defaults).wholeFuel),
    );
    log_add(
        log_Level::log_User,
        b"  --dirjoystick : Enables the use of directionaljoystick controls (default: 0)\0"
            .as_ptr() as *const ::core::ffi::c_char,
    );
    log_add(
        log_Level::log_User,
        b"  --landerhold : Switch between PC/3DO max lander hold, pc=64, 3do=50 (default: %s)\0"
            .as_ptr() as *const ::core::ffi::c_char,
        choiceOptString(&raw const (*defaults).landerHold),
    );
    log_add(
        log_Level::log_User,
        b"  --scrtrans : Screen transitions, pc=instantaneous, 3do=crossfade (default: %s)\0"
            .as_ptr() as *const ::core::ffi::c_char,
        choiceOptString(&raw const (*defaults).scrTrans),
    );
    log_add(
        log_Level::log_User,
        b"  --difficulty : 0: Normal | 1: Easy | 2: Hard| 3: Choose at Start (default: 0)\0"
            .as_ptr() as *const ::core::ffi::c_char,
    );
    log_add(
        log_Level::log_User,
        b"  --fuelrange : Enables extra fuel range indicators : 0: No indicators | 1: Fuel range at destination | 2: Remaining fuel range to Sol | 3: Both option 1 and 2 enabled simultaneously (default: 0)\0"
            .as_ptr() as *const ::core::ffi::c_char,
    );
    log_add(
        log_Level::log_User,
        b"  --extended : Enables Extended Editionfeatures (default: %s)\0".as_ptr()
            as *const ::core::ffi::c_char,
        boolOptString(&raw const (*defaults).extended),
    );
    log_add(
        log_Level::log_User,
        b"  --nomad : Enables 'Nomad Mode' (No Starbase) : 0: Off | 1: Easy | 2: Normal (default: 0)\0"
            .as_ptr() as *const ::core::ffi::c_char,
    );
    log_add(
        log_Level::log_User,
        b"  --gameover : Enables Game Over cutscenes (default: %s)\0".as_ptr()
            as *const ::core::ffi::c_char,
        boolOptString(&raw const (*defaults).gameOver),
    );
    log_add(
        log_Level::log_User,
        b"  --shipdirectionip : Enable NPC ships in IPto face their direction of travel (default: %s)\0"
            .as_ptr() as *const ::core::ffi::c_char,
        boolOptString(&raw const (*defaults).shipDirectionIP),
    );
    log_add(
        log_Level::log_User,
        b"  --hazardcolors : Enable colored text based onhazard severity when viewing planetary scans (default: %s)\0"
            .as_ptr() as *const ::core::ffi::c_char,
        boolOptString(&raw const (*defaults).hazardColors),
    );
    log_add(
        log_Level::log_User,
        b"  --orzcompfont : Enable alternate font foruntranslatable Orz speech (default: %s)\0"
            .as_ptr() as *const ::core::ffi::c_char,
        boolOptString(&raw const (*defaults).orzCompFont),
    );
    log_add(
        log_Level::log_User,
        b"  --smartautopilot : Activating Auto-Pilot within Solar System pilots the Flagship out via the shortest route. (default: %s)\0"
            .as_ptr() as *const ::core::ffi::c_char,
        boolOptString(&raw const (*defaults).smartAutoPilot),
    );
    log_add(
        log_Level::log_User,
        b"  --controllertype : 0: Keyboard | 1: Xbox | 2: PlayStation 4 (default: 0)\0"
            .as_ptr() as *const ::core::ffi::c_char,
    );
    log_add(
        log_Level::log_User,
        b"  --tintplansphere : Tint the planet sphere with scan color during scan (default: %s)\0"
            .as_ptr() as *const ::core::ffi::c_char,
        choiceOptString(&raw const (*defaults).tintPlanSphere),
    );
    log_add(
        log_Level::log_User,
        b"  --planetstyle : Choose between PC or 3DO planet color and shading (default: %s)\0"
            .as_ptr() as *const ::core::ffi::c_char,
        choiceOptString(&raw const (*defaults).planetStyle),
    );
    log_add(
        log_Level::log_User,
        b"  --starbackground : Set the background stars in solar system between PC, 3DO, UQM, or HD-mod patterns (default: pc)\0"
            .as_ptr() as *const ::core::ffi::c_char,
    );
    log_add(
        log_Level::log_User,
        b"  --scanstyle : Choose between PC or 3DO scanning types (default: %s)\0"
            .as_ptr() as *const ::core::ffi::c_char,
        choiceOptString(&raw const (*defaults).scanStyle),
    );
    log_add(
        log_Level::log_User,
        b"  --nonstoposcill : Oscilloscope uses both voice  and music data (default: %s)\0"
            .as_ptr() as *const ::core::ffi::c_char,
        boolOptString(&raw const (*defaults).nonStopOscill),
    );
    log_add(
        log_Level::log_User,
        b"  --scopestyle : Choose between either the PC or 3DO oscilloscope type (default: %s)\0"
            .as_ptr() as *const ::core::ffi::c_char,
        choiceOptString(&raw const (*defaults).scopeStyle),
    );
    log_add(
        log_Level::log_User,
        b"  --animhyperstars : HD only - Use old HD-mod animated HyperSpace stars (default: %s)\0"
            .as_ptr() as *const ::core::ffi::c_char,
        boolOptString(&raw const (*defaults).hyperStars),
    );
    log_add(
        log_Level::log_User,
        b"  --landerview : Choose between either the PC or 3DO lander view (default: %s)\0"
            .as_ptr() as *const ::core::ffi::c_char,
        choiceOptString(&raw const (*defaults).landerStyle),
    );
    log_add(
        log_Level::log_User,
        b"  --planettexture : Choose between either 3DO or UQM planet map texture [when not using custom seed] (default: 3do)\0"
            .as_ptr() as *const ::core::ffi::c_char,
    );
    log_add(
        log_Level::log_User,
        b"  --sisenginecolor : Choose between either the PC or 3DO Flagship engine color (default: %s)\0"
            .as_ptr() as *const ::core::ffi::c_char,
        choiceOptString(&raw const (*defaults).flagshipColor),
    );
    log_add(
        log_Level::log_User,
        b"  --nohqencounters : Disables HyperSpace encounters (default: %s)\0".as_ptr()
            as *const ::core::ffi::c_char,
        boolOptString(&raw const (*defaults).noHQEncounters),
    );
    log_add(
        log_Level::log_User,
        b"  --decleanse : Moves the Death March 100 years ahead from its actual start date [does not work once the Death March has started] (default: %s)\0"
            .as_ptr() as *const ::core::ffi::c_char,
        boolOptString(&raw const (*defaults).deCleansing),
    );
    log_add(
        log_Level::log_User,
        b"  --nomeleeobstacles : Removes the planet and asteroids from Super Melee (default: %s)\0"
            .as_ptr() as *const ::core::ffi::c_char,
        boolOptString(&raw const (*defaults).meleeObstacles),
    );
    log_add(
        log_Level::log_User,
        b"  --showvisitstars : Dim visited stars on the  StarMap and encase the star name in parenthesis (default: %s)\0"
            .as_ptr() as *const ::core::ffi::c_char,
        boolOptString(&raw const (*defaults).showVisitedStars),
    );
    log_add(
        log_Level::log_User,
        b"  --unscaledstarsystem : Show the classic HD-mod  Beta Star System view (default: %s)\0"
            .as_ptr() as *const ::core::ffi::c_char,
        boolOptString(&raw const (*defaults).unscaledStarSystem),
    );
    log_add(
        log_Level::log_User,
        b"  --spheretype : Choose between PC, 3DO, or UQM scan sphere styles (default: %s)\0"
            .as_ptr() as *const ::core::ffi::c_char,
        choiceOptString(&raw const (*defaults).sphereType),
    );
    log_add(
        log_Level::log_User,
        b"--nebulaevol=VOLUME (0-50, default 11)\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    log_add(
        log_Level::log_User,
        b"--slaughtermode : Affect a race's SOI by destroying their ships in battle (default: %s)\0"
            .as_ptr() as *const ::core::ffi::c_char,
        boolOptString(&raw const (*defaults).slaughterMode),
    );
    log_add(
        log_Level::log_User,
        b"  --advancedautopilot : Finds the route that usesthe least amount of fuel through HyperSpace or QuasiSpace and Auto-Pilots the Flagship on the best route (default: %s)\0"
            .as_ptr() as *const ::core::ffi::c_char,
        boolOptString(&raw const (*defaults).advancedAutoPilot),
    );
    log_add(
        log_Level::log_User,
        b"  --meleetooltips : Show SC1-style shipdescription tooltips at the bottom of the Super-Melee screenwhen picking a ship for your fleet (default: %s)\0"
            .as_ptr() as *const ::core::ffi::c_char,
        boolOptString(&raw const (*defaults).meleeToolTips),
    );
    log_add(
        log_Level::log_User,
        b"  --musicresume : Resumes the musicin UQM where it last left off : 0: Off | 1: 5 Minutes | 2: Indefinite (default: 0)\0"
            .as_ptr() as *const ::core::ffi::c_char,
    );
    log_add(
        log_Level::log_User,
        b"  --windowtype : Choose between DOS, 3DO or UQM window types : 0: DOS | 1: 3DO | 2: UQM (default: 0)\0"
            .as_ptr() as *const ::core::ffi::c_char,
    );
    log_add(
        log_Level::log_User,
        b"  --scatterelements : Scatter a percentage of theelements in the lander's cargo hold onto the planet's surface when the lander explodes (default: %s)\0"
            .as_ptr() as *const ::core::ffi::c_char,
        boolOptString(&raw const (*defaults).scatterElements),
    );
    log_add(
        log_Level::log_User,
        b"  --showupgrades : Show lander upgrade graphics when exploring planets (default: %s)\0"
            .as_ptr() as *const ::core::ffi::c_char,
        boolOptString(&raw const (*defaults).showUpgrades),
    );
    log_add(
        log_Level::log_User,
        b"  --fleetpointsys : Restrict the amount of ships that can be purchased via their melee points (default: %s)\0"
            .as_ptr() as *const ::core::ffi::c_char,
        boolOptString(&raw const (*defaults).fleetPointSys),
    );
    log_add(
        log_Level::log_User,
        b"  --shipstore : Enable a storage queue accessed at the shipyard (default: %s)\0"
            .as_ptr() as *const ::core::ffi::c_char,
        boolOptString(&raw const (*defaults).shipStore),
    );
    log_add(
        log_Level::log_User,
        b"  --captainnames : Display captain names at shipyard (default: %s)\0".as_ptr()
            as *const ::core::ffi::c_char,
        boolOptString(&raw const (*defaults).captainNames),
    );
    log_add(
        log_Level::log_User,
        b"  --dosmenus : Display DOS style menu in shipyard in place of SIS window (default: %s)\0"
            .as_ptr() as *const ::core::ffi::c_char,
        boolOptString(&raw const (*defaults).dosMenus),
    );
    log_add(
        log_Level::log_User,
        b"  --hyperspacecolor : Choose between either the PC or 3DO Flagship engine color (default: %s)\0"
            .as_ptr() as *const ::core::ffi::c_char,
        choiceOptString(&raw const (*defaults).hyperSpaceColor),
    );
    log_setOutput(old);
}
unsafe extern "C" fn InvalidArgument(
    mut supplied: *const ::core::ffi::c_char,
    mut opt_name: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    saveError(
        b"Invalid argument '%s' to option %s.\0".as_ptr() as *const ::core::ffi::c_char,
        supplied,
        opt_name,
    );
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn choiceOptString(
    mut option: *const int_option,
) -> *const ::core::ffi::c_char {
    match (*option).value {
        0x1 => return b"3do\0".as_ptr() as *const ::core::ffi::c_char,
        0x2 => return b"pc\0".as_ptr() as *const ::core::ffi::c_char,
        _ => return b"none\0".as_ptr() as *const ::core::ffi::c_char,
    };
}
unsafe extern "C" fn boolOptString(
    mut option: *const bool_option,
) -> *const ::core::ffi::c_char {
    return if (*option).value as ::core::ffi::c_int != 0 {
        b"on\0".as_ptr() as *const ::core::ffi::c_char
    } else {
        b"off\0".as_ptr() as *const ::core::ffi::c_char
    };
}
unsafe extern "C" fn boolNotOptString(
    mut option: *const bool_option,
) -> *const ::core::ffi::c_char {
    return if (*option).value as ::core::ffi::c_int != 0 {
        b"off\0".as_ptr() as *const ::core::ffi::c_char
    } else {
        b"on\0".as_ptr() as *const ::core::ffi::c_char
    };
}
#[inline]
unsafe extern "C" fn inBounds(
    mut val: SDWORD,
    mut min: SDWORD,
    mut max: SDWORD,
) -> SDWORD {
    if val < min {
        return min;
    }
    if val > max {
        return max;
    }
    return val;
}
