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
    fn Alarm_init();
    fn Alarm_uninit();
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
    fn __assert_fail(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    );
    fn mem_init() -> bool;
    fn mem_uninit() -> bool;
    fn HMalloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn HFree(p: *mut ::core::ffi::c_void);
    fn HRealloc(p: *mut ::core::ffi::c_void, size: size_t) -> *mut ::core::ffi::c_void;
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
    ) -> ::core::ffi::c_int;
    fn TFB_UninitGraphics();
    fn TFB_ProcessEvents();
    fn TFB_FlushGraphics();
    fn TFB_PurgeDanglingGraphics();
    fn InitColorMaps();
    fn UninitColorMaps();
    static mut soundflags: ::core::ffi::c_int;
    static mut snddriver: ::core::ffi::c_int;
    fn unInitAudio();
    fn InitTimeSystem();
    fn UnInitTimeSystem();
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
    static mut QuitPosted: ::core::ffi::c_int;
    static mut GameActive: ::core::ffi::c_int;
    fn InitTaskSystem();
    fn CleanupTaskSystem();
    static mut ImmediateInputState: CONTROLLER_INPUT_STATE;
    static mut PlayerControls: [CONTROL_TEMPLATE; 0];
    static mut optWhichCoarseScan: ::core::ffi::c_int;
    static mut optWhichMenu: ::core::ffi::c_int;
    static mut optWhichFonts: ::core::ffi::c_int;
    static mut optWhichIntro: ::core::ffi::c_int;
    static mut optWhichShield: ::core::ffi::c_int;
    static mut optSmoothScroll: ::core::ffi::c_int;
    static mut optMeleeScale: ::core::ffi::c_int;
    static mut opt3doMusic: BOOLEAN;
    static mut optRemixMusic: BOOLEAN;
    static mut optSpeech: BOOLEAN;
    static mut optSubtitles: BOOLEAN;
    static mut optStereoSFX: BOOLEAN;
    static mut optKeepAspectRatio: BOOLEAN;
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
    fn prepareShadowAddons(addons: *mut *const ::core::ffi::c_char);
    fn unprepareAllDirs();
    fn setGammaCorrection(gamma: ::core::ffi::c_float) -> bool;
    static mut force_platform: PLATFORM_TYPE;
    fn log_init(max_lines: ::core::ffi::c_int);
    fn log_initThreads();
    fn log_setOutput(out: *mut FILE) -> *mut FILE;
    fn log_showBox(show: bool, err: bool);
    fn log_captureLines(num: ::core::ffi::c_int);
    fn log_add(_: log_Level, fmt: *const ::core::ffi::c_char, ...);
    fn strcmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strlen(_: *const ::core::ffi::c_char) -> size_t;
    fn Callback_init();
    fn Callback_uninit();
    fn init_communication();
    fn uninit_communication();
    static mut netplayOptions: NetplayOptions;
    static mut webNetplayRole: ::core::ffi::c_int;
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
pub type uint32 = ::core::ffi::c_uint;
pub type sint32 = ::core::ffi::c_int;
pub type __isoc_va_list = __builtin_va_list;
pub type FILE = _IO_FILE;
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
    pub const NUM_KEYS: Self = Self(7);
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
    pub const KEY_MENU_ANY: Self = Self(23);
    pub const NUM_MENU_KEYS: Self = Self(24);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct CONTROL_TEMPLATE(pub ::core::ffi::c_uint);
impl CONTROL_TEMPLATE {
    pub const CONTROL_TEMPLATE_KB_1: Self = Self(0);
    pub const CONTROL_TEMPLATE_KB_2: Self = Self(1);
    pub const CONTROL_TEMPLATE_KB_3: Self = Self(2);
    pub const CONTROL_TEMPLATE_JOY_1: Self = Self(3);
    pub const CONTROL_TEMPLATE_JOY_2: Self = Self(4);
    pub const CONTROL_TEMPLATE_JOY_3: Self = Self(5);
    pub const NUM_TEMPLATES: Self = Self(6);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _controller_input_state {
    pub key: [[::core::ffi::c_int; 7]; 6],
    pub menu: [::core::ffi::c_int; 24],
}
pub type CONTROLLER_INPUT_STATE = _controller_input_state;
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
    pub runMode: C2Rust_Unnamed_6,
    pub configDir: *const ::core::ffi::c_char,
    pub contentDir: *const ::core::ffi::c_char,
    pub addonDir: *const ::core::ffi::c_char,
    pub addons: *mut *const ::core::ffi::c_char,
    pub numAddons: ::core::ffi::c_int,
    pub graphicsBackend: *const ::core::ffi::c_char,
    pub opengl: bool_option,
    pub resolution: C2Rust_Unnamed_5,
    pub fullscreen: bool_option,
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
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_5 {
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub set: bool,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_6(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_6 {
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
pub struct C2Rust_Unnamed_7(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_7 {
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
    pub const SUBTITLES_OPT: Self = Self(1012);
    pub const NETHOST1_OPT: Self = Self(1013);
    pub const NETPORT1_OPT: Self = Self(1014);
    pub const NETHOST2_OPT: Self = Self(1015);
    pub const NETPORT2_OPT: Self = Self(1016);
    pub const NETDELAY_OPT: Self = Self(1017);
    pub const WEB_NETPLAY_OPT: Self = Self(1018);
}
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
static mut meleeScaleList: [option_list_value; 6] = [
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
        runMode: C2Rust_Unnamed_6::runMode_normal,
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
        resolution: C2Rust_Unnamed_5 {
            width: 640 as ::core::ffi::c_int,
            height: 480 as ::core::ffi::c_int,
            set: false,
        },
        fullscreen: bool_option {
            value: false,
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
            value: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
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
            value: 0x2 as ::core::ffi::c_int,
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
    if options.runMode.0 == C2Rust_Unnamed_6::runMode_version.0 {
        printf(
            b"%d.%d.%d%s\n\0".as_ptr() as *const ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
            8 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            b"\0".as_ptr() as *const ::core::ffi::c_char,
        );
        log_showBox(false, false);
        return 0 as ::core::ffi::c_int;
    }
    log_add(
        log_Level::log_User,
        b"The Ur-Quan Masters v%d.%d.%d%s (compiled %s %s)\nThis software comes with ABSOLUTELY NO WARRANTY;\nfor details see the included 'COPYING' file.\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        0 as ::core::ffi::c_int,
        8 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        b"\0".as_ptr() as *const ::core::ffi::c_char,
        b"Oct 19 2026\0".as_ptr() as *const ::core::ffi::c_char,
        b"19:16:52\0".as_ptr() as *const ::core::ffi::c_char,
    );
    log_add(
        log_Level::log_User,
        b"Netplay protocol version %d.%d. Netplay opponent must have UQM %d.%d.%d or later.\0"
            .as_ptr() as *const ::core::ffi::c_char,
        0 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        6 as ::core::ffi::c_int,
        9 as ::core::ffi::c_int,
    );
    if errBuffer[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        log_add(
            log_Level::log_Error,
            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut errBuffer as *mut ::core::ffi::c_char,
        );
        errBuffer[0usize] = '\0' as ::core::ffi::c_char;
    }
    if options.runMode.0 == C2Rust_Unnamed_6::runMode_usage.0 {
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
    initIO();
    prepareConfigDir(options.configDir);
    *(&raw mut PlayerControls as *mut CONTROL_TEMPLATE).offset(0isize) = CONTROL_TEMPLATE::CONTROL_TEMPLATE_KB_1;
    *(&raw mut PlayerControls as *mut CONTROL_TEMPLATE).offset(1isize) = CONTROL_TEMPLATE::CONTROL_TEMPLATE_JOY_1;
    if !options.safeMode.value {
        LoadResourceIndex(
            configDir,
            b"uqm.cfg\0".as_ptr() as *const ::core::ffi::c_char,
            b"config.\0".as_ptr() as *const ::core::ffi::c_char,
        );
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
        res_Remove(&raw mut cfgkey as *mut ::core::ffi::c_char);
        i_1 += 1;
    }
    snddriver = options.soundDriver.value;
    soundflags = options.soundQuality.value;
    opt3doMusic = BOOLEAN(options.use3doMusic.value as ::core::ffi::c_uint);
    optRemixMusic = BOOLEAN(options.useRemixMusic.value as ::core::ffi::c_uint);
    optSpeech = BOOLEAN(options.useSpeech.value as ::core::ffi::c_uint);
    optWhichCoarseScan = options.whichCoarseScan.value;
    optWhichMenu = options.whichMenu.value;
    optWhichFonts = options.whichFonts.value;
    optWhichIntro = options.whichIntro.value;
    optWhichShield = options.whichShield.value;
    optSmoothScroll = options.smoothScroll.value;
    optMeleeScale = options.meleeScale.value;
    optKeepAspectRatio = BOOLEAN(options.keepAspectRatio.value as ::core::ffi::c_uint);
    optSubtitles = BOOLEAN(options.subtitles.value as ::core::ffi::c_uint);
    optStereoSFX = BOOLEAN(options.stereoSFX.value as ::core::ffi::c_uint);
    musicVolumeScale = options.musicVolumeScale.value;
    sfxVolumeScale = options.sfxVolumeScale.value;
    speechVolumeScale = options.speechVolumeScale.value;
    optAddons = options.addons;
    prepareContentDir(options.contentDir, options.addonDir, *argv.offset(0isize));
    prepareMeleeDir();
    prepareSaveDir();
    prepareShadowAddons(options.addons);
    InitTimeSystem();
    InitTaskSystem();
    Alarm_init();
    Callback_init();
    gfxDriver = if options.opengl.value as ::core::ffi::c_int != 0 {
        C2Rust_Unnamed::TFB_GFXDRIVER_SDL_OPENGL.0 as ::core::ffi::c_int
    } else {
        C2Rust_Unnamed::TFB_GFXDRIVER_SDL_PURE.0 as ::core::ffi::c_int
    };
    gfxFlags = options.scaler.value;
    if options.fullscreen.value {
        gfxFlags |= (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int;
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
    (::core::mem::size_of::<[::core::ffi::c_int; 42]>()
        == ::core::mem::size_of::<[[::core::ffi::c_int; 7]; 6]>()
        || {
            __assert_fail(
                b"sizeof (int [NUM_TEMPLATES * NUM_KEYS]) == sizeof (int [NUM_TEMPLATES][NUM_KEYS])\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"src/uqm.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                444 as ::core::ffi::c_int,
                b"main\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    TFB_SetInputVectors(
        &raw mut ImmediateInputState.menu as *mut ::core::ffi::c_int,
        C2Rust_Unnamed_4::NUM_MENU_KEYS.0 as ::core::ffi::c_int,
        &raw mut ImmediateInputState.key as *mut [::core::ffi::c_int; 7]
            as *mut ::core::ffi::c_int,
        CONTROL_TEMPLATE::NUM_TEMPLATES.0 as ::core::ffi::c_int,
        C2Rust_Unnamed_3::NUM_KEYS.0 as ::core::ffi::c_int,
    );
    TFB_InitInput(
        C2Rust_Unnamed_2::TFB_INPUTDRIVER_SDL.0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    );
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
        TFB_ProcessEvents();
        ProcessUtilityKeys();
        ProcessThreadLifecycles();
        TFB_FlushGraphics();
    }
    if ::core::ptr::read_volatile::<::core::ffi::c_int>(&raw const MainExited) != 0 {
        TFB_UninitInput();
        unInitAudio();
        uninit_communication();
        TFB_PurgeDanglingGraphics();
        UninitColorMaps();
        TFB_UninitGraphics();
        Callback_uninit();
        Alarm_uninit();
        CleanupTaskSystem();
        UnInitTimeSystem();
        unprepareAllDirs();
        uninitIO();
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
    getBoolConfigValue(
        &raw mut (*options).fullscreen,
        b"config.fullscreen\0".as_ptr() as *const ::core::ffi::c_char,
    );
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
        0x2 as ::core::ffi::c_int,
        0x1 as ::core::ffi::c_int,
    );
    getBoolConfigValueXlat(
        &raw mut (*options).whichFonts,
        b"config.textgradients\0".as_ptr() as *const ::core::ffi::c_char,
        0x2 as ::core::ffi::c_int,
        0x1 as ::core::ffi::c_int,
    );
    getBoolConfigValueXlat(
        &raw mut (*options).whichCoarseScan,
        b"config.iconicscan\0".as_ptr() as *const ::core::ffi::c_char,
        0x1 as ::core::ffi::c_int,
        0x2 as ::core::ffi::c_int,
    );
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
    if res_IsInteger(b"config.player1control\0".as_ptr() as *const ::core::ffi::c_char).0
        != 0
    {
        *(&raw mut PlayerControls as *mut CONTROL_TEMPLATE).offset(0isize) = CONTROL_TEMPLATE(
            res_GetInteger(
                b"config.player1control\0".as_ptr() as *const ::core::ffi::c_char,
            ) as ::core::ffi::c_uint,
        );
        if (*(&raw mut PlayerControls as *mut CONTROL_TEMPLATE).offset(0isize)).0
            >= CONTROL_TEMPLATE::NUM_TEMPLATES.0
        {
            log_add(
                log_Level::log_Error,
                b"Illegal control template '%d' for Player One.\0".as_ptr()
                    as *const ::core::ffi::c_char,
                (*(&raw mut PlayerControls as *mut CONTROL_TEMPLATE).offset(0isize)).0,
            );
            *(&raw mut PlayerControls as *mut CONTROL_TEMPLATE).offset(0isize) = CONTROL_TEMPLATE::CONTROL_TEMPLATE_KB_1;
        }
    }
    if res_IsInteger(b"config.player2control\0".as_ptr() as *const ::core::ffi::c_char).0
        != 0
    {
        *(&raw mut PlayerControls as *mut CONTROL_TEMPLATE).offset(1isize) = CONTROL_TEMPLATE(
            res_GetInteger(
                b"config.player2control\0".as_ptr() as *const ::core::ffi::c_char,
            ) as ::core::ffi::c_uint,
        );
        if (*(&raw mut PlayerControls as *mut CONTROL_TEMPLATE).offset(1isize)).0
            >= CONTROL_TEMPLATE::NUM_TEMPLATES.0
        {
            log_add(
                log_Level::log_Error,
                b"Illegal control template '%d' for Player Two.\0".as_ptr()
                    as *const ::core::ffi::c_char,
                (*(&raw mut PlayerControls as *mut CONTROL_TEMPLATE).offset(1isize)).0,
            );
            *(&raw mut PlayerControls as *mut CONTROL_TEMPLATE).offset(1isize) = CONTROL_TEMPLATE::CONTROL_TEMPLATE_JOY_1;
        }
    }
}
static mut optString: *const ::core::ffi::c_char = b"+r:foc:b:spC:n:?hM:S:T:q:ug:l:i:vwxk\0"
    .as_ptr() as *const ::core::ffi::c_char;
static mut longOptions: [option; 42] = [
    option {
        name: b"res\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 1 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'r' as ::core::ffi::c_int,
    },
    option {
        name: b"fullscreen\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
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
        name: b"subtitles\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_7::SUBTITLES_OPT.0 as ::core::ffi::c_int,
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
        val: C2Rust_Unnamed_7::CSCAN_OPT.0 as ::core::ffi::c_int,
    },
    option {
        name: b"menu\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 1 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_7::MENU_OPT.0 as ::core::ffi::c_int,
    },
    option {
        name: b"font\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 1 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_7::FONT_OPT.0 as ::core::ffi::c_int,
    },
    option {
        name: b"shield\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 1 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_7::SHIELD_OPT.0 as ::core::ffi::c_int,
    },
    option {
        name: b"scroll\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 1 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_7::SCROLL_OPT.0 as ::core::ffi::c_int,
    },
    option {
        name: b"sound\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 1 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_7::SOUND_OPT.0 as ::core::ffi::c_int,
    },
    option {
        name: b"stereosfx\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_7::STEREOSFX_OPT.0 as ::core::ffi::c_int,
    },
    option {
        name: b"addon\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 1 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_7::ADDON_OPT.0 as ::core::ffi::c_int,
    },
    option {
        name: b"addondir\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 1 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_7::ADDONDIR_OPT.0 as ::core::ffi::c_int,
    },
    option {
        name: b"accel\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 1 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_7::ACCEL_OPT.0 as ::core::ffi::c_int,
    },
    option {
        name: b"safe\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_7::SAFEMODE_OPT.0 as ::core::ffi::c_int,
    },
    option {
        name: b"renderer\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 1 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_7::RENDERER_OPT.0 as ::core::ffi::c_int,
    },
    option {
        name: b"nethost1\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 1 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_7::NETHOST1_OPT.0 as ::core::ffi::c_int,
    },
    option {
        name: b"netport1\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 1 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_7::NETPORT1_OPT.0 as ::core::ffi::c_int,
    },
    option {
        name: b"nethost2\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 1 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_7::NETHOST2_OPT.0 as ::core::ffi::c_int,
    },
    option {
        name: b"netport2\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 1 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_7::NETPORT2_OPT.0 as ::core::ffi::c_int,
    },
    option {
        name: b"netdelay\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 1 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_7::NETDELAY_OPT.0 as ::core::ffi::c_int,
    },
    option {
        name: b"web-netplay\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 1 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_7::WEB_NETPLAY_OPT.0 as ::core::ffi::c_int,
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
        's_636: {
            match c {
                63 => {
                    if optopt != '?' as ::core::ffi::c_int {
                        saveError(
                            b"\nInvalid option or its argument\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                        badArg = true;
                        break 's_636;
                    }
                }
                104 => {}
                118 => {
                    (*options).runMode = C2Rust_Unnamed_6::runMode_version;
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
                        break 's_636;
                    } else {
                        (*options).resolution.width = width;
                        (*options).resolution.height = height;
                        (*options).resolution.set = true;
                        break 's_636;
                    }
                }
                102 => {
                    setBoolOption(&raw mut (*options).fullscreen, true);
                    break 's_636;
                }
                119 => {
                    setBoolOption(&raw mut (*options).fullscreen, false);
                    break 's_636;
                }
                111 => {
                    setBoolOption(&raw mut (*options).opengl, true);
                    break 's_636;
                }
                120 => {
                    setBoolOption(&raw mut (*options).opengl, false);
                    break 's_636;
                }
                107 => {
                    setBoolOption(&raw mut (*options).keepAspectRatio, true);
                    break 's_636;
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
                    break 's_636;
                }
                98 => {
                    if !setListOption(
                        &raw mut (*options).meleeScale,
                        optarg,
                        &raw const meleeScaleList as *const option_list_value,
                    ) {
                        InvalidArgument(
                            optarg,
                            b"--meleezoom or -b\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        badArg = true;
                    }
                    break 's_636;
                }
                115 => {
                    setBoolOption(&raw mut (*options).scanlines, true);
                    break 's_636;
                }
                112 => {
                    setBoolOption(&raw mut (*options).showFps, true);
                    break 's_636;
                }
                110 => {
                    (*options).contentDir = optarg;
                    break 's_636;
                }
                77 => {
                    if !setVolumeOption(
                        &raw mut (*options).musicVolumeScale,
                        optarg,
                        b"music volume\0".as_ptr() as *const ::core::ffi::c_char,
                    ) {
                        badArg = true;
                    }
                    break 's_636;
                }
                83 => {
                    if !setVolumeOption(
                        &raw mut (*options).sfxVolumeScale,
                        optarg,
                        b"sfx volume\0".as_ptr() as *const ::core::ffi::c_char,
                    ) {
                        badArg = true;
                    }
                    break 's_636;
                }
                84 => {
                    if !setVolumeOption(
                        &raw mut (*options).speechVolumeScale,
                        optarg,
                        b"speech volume\0".as_ptr() as *const ::core::ffi::c_char,
                    ) {
                        badArg = true;
                    }
                    break 's_636;
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
                    break 's_636;
                }
                1012 => {
                    setBoolOption(&raw mut (*options).subtitles, true);
                    break 's_636;
                }
                117 => {
                    setBoolOption(&raw mut (*options).subtitles, false);
                    break 's_636;
                }
                103 => {
                    if !setFloatOption(
                        &raw mut (*options).gamma,
                        optarg,
                        b"gamma correction\0".as_ptr() as *const ::core::ffi::c_char,
                    ) {
                        badArg = true;
                    }
                    break 's_636;
                }
                108 => {
                    (*options).logFile = optarg;
                    break 's_636;
                }
                67 => {
                    (*options).configDir = optarg;
                    break 's_636;
                }
                105 => {
                    if !setChoiceOption(&raw mut (*options).whichIntro, optarg) {
                        InvalidArgument(
                            optarg,
                            b"--intro or -i\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        badArg = true;
                    }
                    break 's_636;
                }
                1000 => {
                    if !setChoiceOption(&raw mut (*options).whichCoarseScan, optarg) {
                        InvalidArgument(
                            optarg,
                            b"--cscan\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        badArg = true;
                    }
                    break 's_636;
                }
                1001 => {
                    if !setChoiceOption(&raw mut (*options).whichMenu, optarg) {
                        InvalidArgument(
                            optarg,
                            b"--menu\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        badArg = true;
                    }
                    break 's_636;
                }
                1002 => {
                    if !setChoiceOption(&raw mut (*options).whichFonts, optarg) {
                        InvalidArgument(
                            optarg,
                            b"--font\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        badArg = true;
                    }
                    break 's_636;
                }
                1003 => {
                    if !setChoiceOption(&raw mut (*options).whichShield, optarg) {
                        InvalidArgument(
                            optarg,
                            b"--shield\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        badArg = true;
                    }
                    break 's_636;
                }
                1004 => {
                    if !setChoiceOption(&raw mut (*options).smoothScroll, optarg) {
                        InvalidArgument(
                            optarg,
                            b"--scroll\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        badArg = true;
                    }
                    break 's_636;
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
                    break 's_636;
                }
                1006 => {
                    setBoolOption(&raw mut (*options).stereoSFX, true);
                    break 's_636;
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
                    break 's_636;
                }
                1008 => {
                    (*options).addonDir = optarg;
                    break 's_636;
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
                    break 's_636;
                }
                1010 => {
                    setBoolOption(&raw mut (*options).safeMode, true);
                    break 's_636;
                }
                1011 => {
                    (*options).graphicsBackend = optarg;
                    break 's_636;
                }
                1018 => {
                    if strcmp(optarg, b"host\0".as_ptr() as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int
                    {
                        webNetplayRole = 0 as ::core::ffi::c_int;
                    } else if strcmp(
                        optarg,
                        b"guest\0".as_ptr() as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                    {
                        webNetplayRole = 1 as ::core::ffi::c_int;
                    } else {
                        saveError(
                            b"Invalid web-netplay role; use host or guest.\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                        badArg = true;
                    }
                    netplayOptions.inputDelay = 3 as size_t;
                    break 's_636;
                }
                1013 => {
                    netplayOptions.peer[0usize].isServer = false;
                    netplayOptions.peer[0usize].host = optarg;
                    break 's_636;
                }
                1014 => {
                    netplayOptions.peer[0usize].port = optarg;
                    break 's_636;
                }
                1015 => {
                    netplayOptions.peer[1usize].isServer = false;
                    netplayOptions.peer[1usize].host = optarg;
                    break 's_636;
                }
                1016 => {
                    netplayOptions.peer[1usize].port = optarg;
                    break 's_636;
                }
                1017 => {
                    let mut temp: ::core::ffi::c_int = 0;
                    if parseIntOption(
                        optarg,
                        &raw mut temp,
                        b"network input delay\0".as_ptr() as *const ::core::ffi::c_char,
                    ) == -1 as ::core::ffi::c_int
                    {
                        badArg = true;
                        break 's_636;
                    } else {
                        netplayOptions.inputDelay = temp as size_t;
                        if netplayOptions.inputDelay
                            > (840 as ::core::ffi::c_int / 24 as ::core::ffi::c_int)
                                as size_t
                        {
                            saveError(
                                b"Network input delay is absurdly large.\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                            );
                            badArg = true;
                        }
                        break 's_636;
                    }
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
                    break 's_636;
                }
            }
            (*options).runMode = C2Rust_Unnamed_6::runMode_usage;
            return 0 as ::core::ffi::c_int;
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
    if *str.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
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
        b"  -r, --res=WIDTHxHEIGHT (default 640x480, bigger works only with --opengl)\0"
            .as_ptr() as *const ::core::ffi::c_char,
    );
    log_add(
        log_Level::log_User,
        b"  -f, --fullscreen (default %s)\0".as_ptr() as *const ::core::ffi::c_char,
        boolOptString(&raw const (*defaults).fullscreen),
    );
    log_add(
        log_Level::log_User,
        b"  -w, --windowed (default %s)\0".as_ptr() as *const ::core::ffi::c_char,
        boolNotOptString(&raw const (*defaults).fullscreen),
    );
    log_add(
        log_Level::log_User,
        b"  -o, --opengl (default %s)\0".as_ptr() as *const ::core::ffi::c_char,
        boolOptString(&raw const (*defaults).opengl),
    );
    log_add(
        log_Level::log_User,
        b"  -x, --nogl (default %s)\0".as_ptr() as *const ::core::ffi::c_char,
        boolNotOptString(&raw const (*defaults).opengl),
    );
    log_add(
        log_Level::log_User,
        b"  -k, --keepaspectratio (default %s)\0".as_ptr() as *const ::core::ffi::c_char,
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
        b"  -s, --scanlines (default %s)\0".as_ptr() as *const ::core::ffi::c_char,
        boolOptString(&raw const (*defaults).scanlines),
    );
    log_add(
        log_Level::log_User,
        b"  -p, --fps (default %s)\0".as_ptr() as *const ::core::ffi::c_char,
        boolOptString(&raw const (*defaults).showFps),
    );
    log_add(
        log_Level::log_User,
        b"  -g, --gamma=CORRECTIONVALUE (default 1.0, which causes no change)\0".as_ptr()
            as *const ::core::ffi::c_char,
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
        b"  --subtitles (enable dialogue text, overriding saved preferences)\0".as_ptr()
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
        b"  --nethostN=HOSTNAME (server to connect to for player N (1=bottom, 2=top)\0"
            .as_ptr() as *const ::core::ffi::c_char,
    );
    log_add(
        log_Level::log_User,
        b"  --netportN=PORT (port to connect to/listen on for player N (1=bottom, 2=top)\0"
            .as_ptr() as *const ::core::ffi::c_char,
    );
    log_add(
        log_Level::log_User,
        b"  --netdelay=FRAMES (number of frames to buffer/delay network input for\0"
            .as_ptr() as *const ::core::ffi::c_char,
    );
    log_add(
        log_Level::log_User,
        b"The following options can take either '3do' or 'pc' as an option:\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    log_add(
        log_Level::log_User,
        b"  -i, --intro : Intro/ending version (default %s)\0".as_ptr()
            as *const ::core::ffi::c_char,
        choiceOptString(&raw const (*defaults).whichIntro),
    );
    log_add(
        log_Level::log_User,
        b"  --cscan     : coarse-scan display, pc=text, 3do=hieroglyphs (default %s)\0"
            .as_ptr() as *const ::core::ffi::c_char,
        choiceOptString(&raw const (*defaults).whichCoarseScan),
    );
    log_add(
        log_Level::log_User,
        b"  --menu      : menu type, pc=text, 3do=graphical (default %s)\0".as_ptr()
            as *const ::core::ffi::c_char,
        choiceOptString(&raw const (*defaults).whichMenu),
    );
    log_add(
        log_Level::log_User,
        b"  --font      : font types and colors (default %s)\0".as_ptr()
            as *const ::core::ffi::c_char,
        choiceOptString(&raw const (*defaults).whichFonts),
    );
    log_add(
        log_Level::log_User,
        b"  --shield    : slave shield type; pc=static, 3do=throbbing (default %s)\0"
            .as_ptr() as *const ::core::ffi::c_char,
        choiceOptString(&raw const (*defaults).whichShield),
    );
    log_add(
        log_Level::log_User,
        b"  --scroll    : ff/frev during comm.  pc=per-page, 3do=smooth (default %s)\0"
            .as_ptr() as *const ::core::ffi::c_char,
        choiceOptString(&raw const (*defaults).smoothScroll),
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
