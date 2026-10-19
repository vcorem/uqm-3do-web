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

// Rust translation of src/uqm/gameinp.c.
extern "C" {
    pub type string_table_entry;
    fn abs(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn __assert_fail(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    ) -> !;
    fn SetAbsStringTableIndex(String: STRING, StringTableIndex: COUNT) -> STRING;
    fn GetTimeCounter() -> TimeCount;
    static mut last_input: [LAST_INPUT; 0];
    fn VControl_GetJoyAxis(
        port: ::core::ffi::c_int,
        axis: SDL_GameControllerAxis,
    ) -> ::core::ffi::c_int;
    fn PauseGame() -> BOOLEAN;
    fn SleepGame();
    fn DoConfirmExit() -> BOOLEAN;
    static mut InSetupMenu: BOOLEAN;
    fn TaskSwitch();
    fn inSavablePos() -> BOOLEAN;
    static mut MenuSounds: SOUND;
    fn NotPositional() -> SoundPosition;
    fn PlaySoundEffect(
        S: SOUND,
        Channel: COUNT,
        Pos: SoundPosition,
        PositionalObject: *mut ::core::ffi::c_void,
        Priority: BYTE,
    );
    fn PlaySpeechEffect(
        S: SOUND,
        Pos: SoundPosition,
        PositionalObject: *mut ::core::ffi::c_void,
        Priority: BYTE,
    );
    fn debugKeyPressedSynchronous();
    fn debugKey2PressedSynchronous();
    fn debugKey3PressedSynchronous();
    fn debugKey4PressedSynchronous();
    fn Async_process();
    fn TFB_ResetControls();
    static mut GameActive: ::core::ffi::c_int;
    fn BeginInputFrame();
    fn log_add(_: log_Level, fmt: *const ::core::ffi::c_char, ...);
    fn RequestQuickLoad();
    fn QuickSave() -> BOOLEAN;
    static mut optAutoButtons: OPT_ENABLABLE;
    static mut optDirJoy: [::core::ffi::c_int; 2];
    static mut optControllerType: ::core::ffi::c_int;
    static mut DeadZoneLeftStick: [::core::ffi::c_int; 2];
    static mut DeadZoneRightStick: [::core::ffi::c_int; 2];
    static mut DirJoyActive: BOOLEAN;
}
pub type int32_t = ::core::ffi::c_int;
pub type uint8 = ::core::ffi::c_uchar;
pub type uint16 = ::core::ffi::c_ushort;
pub type uint32 = ::core::ffi::c_uint;
pub type BYTE = uint8;
pub type UBYTE = uint8;
pub type UWORD = uint16;
pub type DWORD = uint32;
pub type COUNT = UWORD;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct BOOLEAN(pub ::core::ffi::c_uint);
impl BOOLEAN {
    pub const FALSE: Self = Self(0);
    pub const TRUE: Self = Self(1);
}
pub type STRING_TABLE_ENTRY_DESC = string_table_entry;
pub type STRING = *mut STRING_TABLE_ENTRY_DESC;
pub type TimeCount = DWORD;
pub type Sint32 = int32_t;
pub type SDL_JoystickID = Sint32;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct SDL_GameControllerType(pub ::core::ffi::c_uint);
impl SDL_GameControllerType {
    pub const SDL_CONTROLLER_TYPE_UNKNOWN: Self = Self(0);
    pub const SDL_CONTROLLER_TYPE_XBOX360: Self = Self(1);
    pub const SDL_CONTROLLER_TYPE_XBOXONE: Self = Self(2);
    pub const SDL_CONTROLLER_TYPE_PS3: Self = Self(3);
    pub const SDL_CONTROLLER_TYPE_PS4: Self = Self(4);
    pub const SDL_CONTROLLER_TYPE_NINTENDO_SWITCH_PRO: Self = Self(5);
    pub const SDL_CONTROLLER_TYPE_VIRTUAL: Self = Self(6);
    pub const SDL_CONTROLLER_TYPE_PS5: Self = Self(7);
    pub const SDL_CONTROLLER_TYPE_AMAZON_LUNA: Self = Self(8);
    pub const SDL_CONTROLLER_TYPE_GOOGLE_STADIA: Self = Self(9);
    pub const SDL_CONTROLLER_TYPE_NVIDIA_SHIELD: Self = Self(10);
    pub const SDL_CONTROLLER_TYPE_NINTENDO_SWITCH_JOYCON_LEFT: Self = Self(11);
    pub const SDL_CONTROLLER_TYPE_NINTENDO_SWITCH_JOYCON_RIGHT: Self = Self(12);
    pub const SDL_CONTROLLER_TYPE_NINTENDO_SWITCH_JOYCON_PAIR: Self = Self(13);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct SDL_GameControllerAxis(pub ::core::ffi::c_int);
impl SDL_GameControllerAxis {
    pub const SDL_CONTROLLER_AXIS_INVALID: Self = Self(-1);
    pub const SDL_CONTROLLER_AXIS_LEFTX: Self = Self(0);
    pub const SDL_CONTROLLER_AXIS_LEFTY: Self = Self(1);
    pub const SDL_CONTROLLER_AXIS_RIGHTX: Self = Self(2);
    pub const SDL_CONTROLLER_AXIS_RIGHTY: Self = Self(3);
    pub const SDL_CONTROLLER_AXIS_TRIGGERLEFT: Self = Self(4);
    pub const SDL_CONTROLLER_AXIS_TRIGGERRIGHT: Self = Self(5);
    pub const SDL_CONTROLLER_AXIS_MAX: Self = Self(6);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LAST_INPUT {
    pub r#type: ::core::ffi::c_int,
    pub gamepad: ::core::ffi::c_int,
    pub pressed: ::core::ffi::c_int,
    pub actions: ::core::ffi::c_int,
}
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
pub struct OPT_ENABLABLE(pub ::core::ffi::c_uint);
impl OPT_ENABLABLE {
    pub const OPTVAL_DISABLED: Self = Self(0);
    pub const OPTVAL_ENABLED: Self = Self(1);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MENU_ANNOTATIONS {
    pub key: [[DWORD; 8]; 2],
    pub menu: [DWORD; 31],
}
pub type InputFrameCallback = unsafe extern "C" fn() -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct INPUT_STATE_DESC {
    pub InputFunc: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> BOOLEAN>,
}
pub type SoundPosition = soundposition;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct soundposition {
    pub positional: BOOLEAN,
    pub x: ::core::ffi::c_int,
    pub y: ::core::ffi::c_int,
}
pub type SOUND = STRING;
pub type MENU_SOUND_FLAGS = UWORD;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_1(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_1 {
    pub const MENU_SOUND_MOVE: Self = Self(0);
    pub const MENU_SOUND_SUCCESS: Self = Self(1);
    pub const MENU_SOUND_FAILURE: Self = Self(2);
    pub const MENU_SOUND_INVOKED: Self = Self(3);
    pub const MENU_SOUND_BUBBLEWARP: Self = Self(4);
    pub const MENU_SOUND_TEXT: Self = Self(5);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_2(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_2 {
    pub const atan2i_coeff_1: Self = Self(51471);
    pub const atan2i_coeff_2: Self = Self(154413);
    pub const atan2i_PI: Self = Self(205887);
    pub const SHIP_DIRECTIONS: Self = Self(16);
}
#[no_mangle]
pub static mut PlayerControls: [CONTROL_TEMPLATE; 2] = [CONTROL_TEMPLATE::CONTROL_TEMPLATE_PL_1; 2];
#[no_mangle]
pub static mut PulsedInputState: CONTROLLER_INPUT_STATE = CONTROLLER_INPUT_STATE {
    key: [[0; 8]; 2],
    menu: [0; 31],
};
#[no_mangle]
pub static mut CurrentInputState: CONTROLLER_INPUT_STATE = CONTROLLER_INPUT_STATE {
    key: [[0; 8]; 2],
    menu: [0; 31],
};
static mut OldInputState: CONTROLLER_INPUT_STATE = CONTROLLER_INPUT_STATE {
    key: [[0; 8]; 2],
    menu: [0; 31],
};
static mut CachedInputState: CONTROLLER_INPUT_STATE = CONTROLLER_INPUT_STATE {
    key: [[0; 8]; 2],
    menu: [0; 31],
};
static mut Times: MENU_ANNOTATIONS = MENU_ANNOTATIONS {
    key: [[0; 8]; 2],
    menu: [0; 31],
};
static mut RepeatDelays: MENU_ANNOTATIONS = MENU_ANNOTATIONS {
    key: [[0; 8]; 2],
    menu: [0; 31],
};
static mut GestaltTime: DWORD = 0;
static mut GestaltRepeatDelay: DWORD = 0;
static mut CachedGestalt: BOOLEAN = BOOLEAN::FALSE;
static mut OldGestalt: BOOLEAN = BOOLEAN::FALSE;
static mut _max_accel: DWORD = 0;
static mut _min_accel: DWORD = 0;
static mut _step_accel: DWORD = 0;
static mut _gestalt_keys: BOOLEAN = BOOLEAN::FALSE;
static mut sound_1: MENU_SOUND_FLAGS = 0;
static mut sound_0: MENU_SOUND_FLAGS = 0;
#[no_mangle]
pub static mut ImmediateInputState: CONTROLLER_INPUT_STATE = CONTROLLER_INPUT_STATE {
    key: [[0; 8]; 2],
    menu: [0; 31],
};
#[no_mangle]
pub static mut ExitRequested: BOOLEAN = BOOLEAN::FALSE;
#[no_mangle]
pub static mut GamePaused: BOOLEAN = BOOLEAN::FALSE;
#[no_mangle]
pub static mut OnScreenKeyboardLocked: BOOLEAN = BOOLEAN::FALSE;
static mut inputCallback: Option<InputFrameCallback> = None;
unsafe extern "C" fn _clear_menu_state() {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < CONTROL_TEMPLATE::NUM_TEMPLATES.0 as ::core::ffi::c_int {
        j = 0 as ::core::ffi::c_int;
        while j < C2Rust_Unnamed::NUM_KEYS.0 as ::core::ffi::c_int {
            PulsedInputState.key[i as usize][j as usize] = 0 as ::core::ffi::c_int;
            CachedInputState.key[i as usize][j as usize] = 0 as ::core::ffi::c_int;
            j += 1;
        }
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < C2Rust_Unnamed_0::NUM_MENU_KEYS.0 as ::core::ffi::c_int {
        PulsedInputState.menu[i as usize] = 0 as ::core::ffi::c_int;
        CachedInputState.menu[i as usize] = 0 as ::core::ffi::c_int;
        i += 1;
    }
    CachedGestalt = BOOLEAN::FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn ResetKeyRepeat() {
    let mut initTime: DWORD = GetTimeCounter();
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < CONTROL_TEMPLATE::NUM_TEMPLATES.0 as ::core::ffi::c_int {
        j = 0 as ::core::ffi::c_int;
        while j < C2Rust_Unnamed::NUM_KEYS.0 as ::core::ffi::c_int {
            RepeatDelays.key[i as usize][j as usize] = _max_accel;
            Times.key[i as usize][j as usize] = initTime;
            j += 1;
        }
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < C2Rust_Unnamed_0::NUM_MENU_KEYS.0 as ::core::ffi::c_int {
        RepeatDelays.menu[i as usize] = _max_accel;
        Times.menu[i as usize] = initTime;
        i += 1;
    }
    GestaltRepeatDelay = _max_accel;
    GestaltTime = initTime;
}
unsafe extern "C" fn _check_for_pulse(
    mut current: *mut ::core::ffi::c_int,
    mut cached: *mut ::core::ffi::c_int,
    mut old: *mut ::core::ffi::c_int,
    mut accel: *mut DWORD,
    mut newtime: *mut DWORD,
    mut oldtime: *mut DWORD,
) {
    if *cached != 0 && *old != 0 {
        if (*newtime).wrapping_sub(*oldtime) < *accel {
            *current = 0 as ::core::ffi::c_int;
        } else {
            *current = *cached;
            if *accel > _min_accel {
                *accel = (*accel).wrapping_sub(_step_accel);
            }
            if *accel < _min_accel {
                *accel = _min_accel;
            }
            *oldtime = *newtime;
        }
    } else {
        *current = *cached;
        *oldtime = *newtime;
        *accel = _max_accel;
    };
}
unsafe extern "C" fn _check_gestalt(mut NewTime: DWORD) {
    let mut CurrentGestalt: BOOLEAN = BOOLEAN::FALSE;
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    OldGestalt = CachedGestalt;
    CachedGestalt = BOOLEAN(0 as ::core::ffi::c_int as ::core::ffi::c_uint);
    CurrentGestalt = BOOLEAN(0 as ::core::ffi::c_int as ::core::ffi::c_uint);
    i = 0 as ::core::ffi::c_int;
    while i < CONTROL_TEMPLATE::NUM_TEMPLATES.0 as ::core::ffi::c_int {
        j = 0 as ::core::ffi::c_int;
        while j < C2Rust_Unnamed::NUM_KEYS.0 as ::core::ffi::c_int {
            CachedGestalt = BOOLEAN(
                CachedGestalt.0
                    | ::core::ptr::read_volatile::<
                        ::core::ffi::c_int,
                    >(&raw const ImmediateInputState.key[i as usize][j as usize])
                        as ::core::ffi::c_uint,
            );
            CurrentGestalt = BOOLEAN(
                CurrentGestalt.0
                    | PulsedInputState.key[i as usize][j as usize] as ::core::ffi::c_uint,
            );
            j += 1;
        }
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < C2Rust_Unnamed_0::NUM_MENU_KEYS.0 as ::core::ffi::c_int {
        CachedGestalt = BOOLEAN(
            CachedGestalt.0
                | ::core::ptr::read_volatile::<
                    ::core::ffi::c_int,
                >(&raw const ImmediateInputState.menu[i as usize]) as ::core::ffi::c_uint,
        );
        CurrentGestalt = BOOLEAN(
            CurrentGestalt.0 | PulsedInputState.menu[i as usize] as ::core::ffi::c_uint,
        );
        i += 1;
    }
    if OldGestalt.0 != 0 && CachedGestalt.0 != 0 {
        if NewTime.wrapping_sub(GestaltTime) < GestaltRepeatDelay {
            i = 0 as ::core::ffi::c_int;
            while i < CONTROL_TEMPLATE::NUM_TEMPLATES.0 as ::core::ffi::c_int {
                j = 0 as ::core::ffi::c_int;
                while j < C2Rust_Unnamed::NUM_KEYS.0 as ::core::ffi::c_int {
                    PulsedInputState.key[i as usize][j as usize] = 0
                        as ::core::ffi::c_int;
                    j += 1;
                }
                i += 1;
            }
            i = 0 as ::core::ffi::c_int;
            while i < C2Rust_Unnamed_0::NUM_MENU_KEYS.0 as ::core::ffi::c_int {
                PulsedInputState.menu[i as usize] = 0 as ::core::ffi::c_int;
                i += 1;
            }
        } else {
            i = 0 as ::core::ffi::c_int;
            while i < CONTROL_TEMPLATE::NUM_TEMPLATES.0 as ::core::ffi::c_int {
                j = 0 as ::core::ffi::c_int;
                while j < C2Rust_Unnamed::NUM_KEYS.0 as ::core::ffi::c_int {
                    PulsedInputState.key[i as usize][j as usize] = CachedInputState
                        .key[i as usize][j as usize];
                    j += 1;
                }
                i += 1;
            }
            i = 0 as ::core::ffi::c_int;
            while i < C2Rust_Unnamed_0::NUM_MENU_KEYS.0 as ::core::ffi::c_int {
                PulsedInputState.menu[i as usize] = CachedInputState.menu[i as usize];
                i += 1;
            }
            if GestaltRepeatDelay > _min_accel {
                GestaltRepeatDelay = GestaltRepeatDelay.wrapping_sub(_step_accel);
            }
            if GestaltRepeatDelay < _min_accel {
                GestaltRepeatDelay = _min_accel;
            }
            GestaltTime = NewTime;
        }
    } else {
        i = 0 as ::core::ffi::c_int;
        while i < CONTROL_TEMPLATE::NUM_TEMPLATES.0 as ::core::ffi::c_int {
            j = 0 as ::core::ffi::c_int;
            while j < C2Rust_Unnamed::NUM_KEYS.0 as ::core::ffi::c_int {
                PulsedInputState.key[i as usize][j as usize] = CachedInputState
                    .key[i as usize][j as usize];
                j += 1;
            }
            i += 1;
        }
        i = 0 as ::core::ffi::c_int;
        while i < C2Rust_Unnamed_0::NUM_MENU_KEYS.0 as ::core::ffi::c_int {
            PulsedInputState.menu[i as usize] = CachedInputState.menu[i as usize];
            i += 1;
        }
        GestaltTime = NewTime;
        GestaltRepeatDelay = _max_accel;
    };
}
#[no_mangle]
pub unsafe extern "C" fn UpdateInputState() {
    let mut NewTime: DWORD = 0;
    if ::core::ptr::read_volatile::<::core::ffi::c_int>(&raw const GameActive) == 0 {
        uqm_lua_checked!(SleepGame(), ());
    }
    if ::core::ptr::read_volatile::<BOOLEAN>(&raw const GamePaused).0 != 0 {
        uqm_lua_checked!(PauseGame(), ());
    }
    if ::core::ptr::read_volatile::<BOOLEAN>(&raw const ExitRequested).0 != 0 {
        uqm_lua_checked!(ConfirmExit(), ());
    }
    CurrentInputState = ::core::ptr::read_volatile::<
        CONTROLLER_INPUT_STATE,
    >(&raw const ImmediateInputState);
    OldInputState = CachedInputState;
    CachedInputState = ::core::ptr::read_volatile::<
        CONTROLLER_INPUT_STATE,
    >(&raw const ImmediateInputState);
    BeginInputFrame();
    NewTime = GetTimeCounter() as DWORD;
    if _gestalt_keys.0 != 0 {
        _check_gestalt(NewTime);
    } else {
        let mut i: ::core::ffi::c_int = 0;
        let mut j: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while i < CONTROL_TEMPLATE::NUM_TEMPLATES.0 as ::core::ffi::c_int {
            j = 0 as ::core::ffi::c_int;
            while j < C2Rust_Unnamed::NUM_KEYS.0 as ::core::ffi::c_int {
                _check_for_pulse(
                    (&raw mut *(&raw mut PulsedInputState.key
                        as *mut [::core::ffi::c_int; 8])
                        .offset(i as isize) as *mut ::core::ffi::c_int)
                        .offset(j as isize),
                    (&raw mut *(&raw mut CachedInputState.key
                        as *mut [::core::ffi::c_int; 8])
                        .offset(i as isize) as *mut ::core::ffi::c_int)
                        .offset(j as isize),
                    (&raw mut *(&raw mut OldInputState.key
                        as *mut [::core::ffi::c_int; 8])
                        .offset(i as isize) as *mut ::core::ffi::c_int)
                        .offset(j as isize),
                    (&raw mut *(&raw mut RepeatDelays.key as *mut [DWORD; 8])
                        .offset(i as isize) as *mut DWORD)
                        .offset(j as isize),
                    &raw mut NewTime,
                    (&raw mut *(&raw mut Times.key as *mut [DWORD; 8]).offset(i as isize)
                        as *mut DWORD)
                        .offset(j as isize),
                );
                j += 1;
            }
            i += 1;
        }
        i = 0 as ::core::ffi::c_int;
        while i < C2Rust_Unnamed_0::NUM_MENU_KEYS.0 as ::core::ffi::c_int {
            _check_for_pulse(
                (&raw mut PulsedInputState.menu as *mut ::core::ffi::c_int)
                    .offset(i as isize),
                (&raw mut CachedInputState.menu as *mut ::core::ffi::c_int)
                    .offset(i as isize),
                (&raw mut OldInputState.menu as *mut ::core::ffi::c_int)
                    .offset(i as isize),
                (&raw mut RepeatDelays.menu as *mut DWORD).offset(i as isize),
                &raw mut NewTime,
                (&raw mut Times.menu as *mut DWORD).offset(i as isize),
            );
            i += 1;
        }
    }
    if CurrentInputState
        .menu[C2Rust_Unnamed_0::KEY_PAUSE.0 as ::core::ffi::c_int as usize] != 0
    {
        ::core::ptr::write_volatile(&raw mut GamePaused, BOOLEAN::TRUE);
    }
    if CurrentInputState
        .menu[C2Rust_Unnamed_0::KEY_EXIT.0 as ::core::ffi::c_int as usize] != 0
    {
        ::core::ptr::write_volatile(&raw mut ExitRequested, BOOLEAN::TRUE);
    }
    if PulsedInputState
        .menu[C2Rust_Unnamed_0::KEY_QUICKLOAD.0 as ::core::ffi::c_int as usize] != 0
    {
        FlushInput();
        uqm_lua_checked!(RequestQuickLoad(), ());
    }
    if PulsedInputState
        .menu[C2Rust_Unnamed_0::KEY_QUICKSAVE.0 as ::core::ffi::c_int as usize] != 0
    {
        FlushInput();
        if uqm_lua_checked!(inSavablePos(), ()).0 != 0 {
            uqm_lua_checked!(QuickSave(), ());
        }
    }
    if optAutoButtons.0 != 0 && InSetupMenu.0 == 0 {
        ControllerTypeSwitcher();
    }
    if PulsedInputState
        .menu[C2Rust_Unnamed_0::KEY_DEBUG.0 as ::core::ffi::c_int as usize] != 0
    {
        uqm_lua_checked!(debugKeyPressedSynchronous(), ());
    }
    if PulsedInputState
        .menu[C2Rust_Unnamed_0::KEY_DEBUG_2.0 as ::core::ffi::c_int as usize] != 0
    {
        uqm_lua_checked!(debugKey2PressedSynchronous(), ());
    }
    if PulsedInputState
        .menu[C2Rust_Unnamed_0::KEY_DEBUG_3.0 as ::core::ffi::c_int as usize] != 0
    {
        debugKey3PressedSynchronous();
    }
    if PulsedInputState
        .menu[C2Rust_Unnamed_0::KEY_DEBUG_4.0 as ::core::ffi::c_int as usize] != 0
    {
        debugKey4PressedSynchronous();
    }
}
#[no_mangle]
pub unsafe extern "C" fn SetInputCallback(
    mut callback: Option<InputFrameCallback>,
) -> Option<InputFrameCallback> {
    let mut old: Option<InputFrameCallback> = inputCallback;
    (old.is_none() || callback.is_none()
        || {
            __assert_fail(
                b"!old || !callback\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/uqm/gameinp.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                328 as ::core::ffi::c_int,
                b"SetInputCallback\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    inputCallback = callback;
    return old;
}
#[no_mangle]
pub unsafe extern "C" fn SetMenuRepeatDelay(
    mut min: DWORD,
    mut max: DWORD,
    mut step: DWORD,
    mut gestalt: BOOLEAN,
) {
    _min_accel = min;
    _max_accel = max;
    _step_accel = step;
    _gestalt_keys = gestalt;
    ResetKeyRepeat();
}
#[no_mangle]
pub unsafe extern "C" fn SetDefaultMenuRepeatDelay() {
    _min_accel = (840 as ::core::ffi::c_int / 12 as ::core::ffi::c_int) as DWORD;
    _max_accel = (840 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as DWORD;
    _step_accel = (840 as ::core::ffi::c_int / 12 as ::core::ffi::c_int) as DWORD;
    _gestalt_keys = BOOLEAN::FALSE;
    ResetKeyRepeat();
}
#[no_mangle]
pub unsafe extern "C" fn FlushInput() {
    TFB_ResetControls();
    _clear_menu_state();
}
unsafe extern "C" fn MenuKeysToSoundFlags(
    mut state: *const CONTROLLER_INPUT_STATE,
) -> MENU_SOUND_FLAGS {
    let mut soundFlags: MENU_SOUND_FLAGS = 0;
    soundFlags = 0 as ::core::ffi::c_int as MENU_SOUND_FLAGS;
    if (*state).menu[C2Rust_Unnamed_0::KEY_MENU_UP.0 as ::core::ffi::c_int as usize] != 0
    {
        soundFlags = (soundFlags as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as MENU_SOUND_FLAGS
                as ::core::ffi::c_int) as MENU_SOUND_FLAGS;
    }
    if (*state).menu[C2Rust_Unnamed_0::KEY_MENU_DOWN.0 as ::core::ffi::c_int as usize]
        != 0
    {
        soundFlags = (soundFlags as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as MENU_SOUND_FLAGS
                as ::core::ffi::c_int) as MENU_SOUND_FLAGS;
    }
    if (*state).menu[C2Rust_Unnamed_0::KEY_MENU_LEFT.0 as ::core::ffi::c_int as usize]
        != 0
    {
        soundFlags = (soundFlags as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as MENU_SOUND_FLAGS
                as ::core::ffi::c_int) as MENU_SOUND_FLAGS;
    }
    if (*state).menu[C2Rust_Unnamed_0::KEY_MENU_RIGHT.0 as ::core::ffi::c_int as usize]
        != 0
    {
        soundFlags = (soundFlags as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as MENU_SOUND_FLAGS
                as ::core::ffi::c_int) as MENU_SOUND_FLAGS;
    }
    if (*state).menu[C2Rust_Unnamed_0::KEY_MENU_SELECT.0 as ::core::ffi::c_int as usize]
        != 0
    {
        soundFlags = (soundFlags as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as MENU_SOUND_FLAGS
                as ::core::ffi::c_int) as MENU_SOUND_FLAGS;
    }
    if (*state).menu[C2Rust_Unnamed_0::KEY_MENU_CANCEL.0 as ::core::ffi::c_int as usize]
        != 0
    {
        soundFlags = (soundFlags as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as MENU_SOUND_FLAGS
                as ::core::ffi::c_int) as MENU_SOUND_FLAGS;
    }
    if (*state).menu[C2Rust_Unnamed_0::KEY_MENU_SPECIAL.0 as ::core::ffi::c_int as usize]
        != 0
    {
        soundFlags = (soundFlags as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as MENU_SOUND_FLAGS
                as ::core::ffi::c_int) as MENU_SOUND_FLAGS;
    }
    if (*state).menu[C2Rust_Unnamed_0::KEY_MENU_PAGE_UP.0 as ::core::ffi::c_int as usize]
        != 0
    {
        soundFlags = (soundFlags as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as MENU_SOUND_FLAGS
                as ::core::ffi::c_int) as MENU_SOUND_FLAGS;
    }
    if (*state)
        .menu[C2Rust_Unnamed_0::KEY_MENU_PAGE_DOWN.0 as ::core::ffi::c_int as usize] != 0
    {
        soundFlags = (soundFlags as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as MENU_SOUND_FLAGS
                as ::core::ffi::c_int) as MENU_SOUND_FLAGS;
    }
    if (*state).menu[C2Rust_Unnamed_0::KEY_MENU_DELETE.0 as ::core::ffi::c_int as usize]
        != 0
    {
        soundFlags = (soundFlags as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int) as MENU_SOUND_FLAGS
                as ::core::ffi::c_int) as MENU_SOUND_FLAGS;
    }
    if (*state)
        .menu[C2Rust_Unnamed_0::KEY_MENU_BACKSPACE.0 as ::core::ffi::c_int as usize] != 0
    {
        soundFlags = (soundFlags as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int) as MENU_SOUND_FLAGS
                as ::core::ffi::c_int) as MENU_SOUND_FLAGS;
    }
    return soundFlags;
}
#[no_mangle]
pub unsafe extern "C" fn DoInput(
    mut pInputState: *mut ::core::ffi::c_void,
    mut resetInput: BOOLEAN,
) {
    if resetInput.0 != 0 {
        FlushInput();
    }
    loop {
        let mut soundFlags: MENU_SOUND_FLAGS = 0;
        uqm_lua_checked!(Async_process(), ());
        TaskSwitch();
        uqm_lua_checked!(UpdateInputState(), ());
        soundFlags = MenuKeysToSoundFlags(&raw mut PulsedInputState);
        if !MenuSounds.is_null()
            && soundFlags as ::core::ffi::c_int
                & (sound_0 as ::core::ffi::c_int | sound_1 as ::core::ffi::c_int) != 0
        {
            let mut S: SOUND = ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>();
            S = MenuSounds;
            if soundFlags as ::core::ffi::c_int & sound_1 as ::core::ffi::c_int != 0 {
                S = SetAbsStringTableIndex(
                    S,
                    C2Rust_Unnamed_1::MENU_SOUND_SUCCESS.0 as ::core::ffi::c_int as COUNT,
                ) as SOUND;
            }
            uqm_lua_checked!(PlaySoundEffect(
                S,
                0 as COUNT,
                NotPositional(),
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                0 as BYTE,
            ), ());
        }
        if inputCallback.is_some() {
            uqm_lua_checked!(inputCallback.expect("non-null function pointer")(), ());
        }
        if uqm_lua_checked!((*(pInputState as *mut INPUT_STATE_DESC))
            .InputFunc
            .expect("non-null function pointer")(pInputState), ())
            .0 == 0
        {
            break;
        }
    }
    if resetInput.0 != 0 {
        FlushInput();
    }
}
#[no_mangle]
pub unsafe extern "C" fn SetMenuSounds(
    mut s0: MENU_SOUND_FLAGS,
    mut s1: MENU_SOUND_FLAGS,
) {
    sound_0 = s0;
    sound_1 = s1;
}
#[no_mangle]
pub unsafe extern "C" fn GetMenuSounds(
    mut s0: *mut MENU_SOUND_FLAGS,
    mut s1: *mut MENU_SOUND_FLAGS,
) {
    *s0 = sound_0;
    *s1 = sound_1;
}
unsafe extern "C" fn ControlInputToBattleInput(
    mut keyState: *const ::core::ffi::c_int,
    mut player: COUNT,
    mut direction: ::core::ffi::c_int,
) -> BATTLE_INPUT_STATE {
    let mut InputState: BATTLE_INPUT_STATE = 0 as BATTLE_INPUT_STATE;
    InputState = (InputState as ::core::ffi::c_int
        | GetDirectionalJoystickInput(direction, player as ::core::ffi::c_int)
            as ::core::ffi::c_int) as BATTLE_INPUT_STATE;
    if *keyState.offset(C2Rust_Unnamed::KEY_LEFT.0 as ::core::ffi::c_int as isize) != 0 {
        InputState = (InputState as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int)
                as BATTLE_INPUT_STATE as ::core::ffi::c_int) as BATTLE_INPUT_STATE;
    }
    if *keyState.offset(C2Rust_Unnamed::KEY_RIGHT.0 as ::core::ffi::c_int as isize) != 0
    {
        InputState = (InputState as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
                as BATTLE_INPUT_STATE as ::core::ffi::c_int) as BATTLE_INPUT_STATE;
    }
    if *keyState.offset(C2Rust_Unnamed::KEY_UP.0 as ::core::ffi::c_int as isize) != 0 {
        InputState = (InputState as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                as BATTLE_INPUT_STATE as ::core::ffi::c_int) as BATTLE_INPUT_STATE;
    }
    if *keyState.offset(C2Rust_Unnamed::KEY_WEAPON.0 as ::core::ffi::c_int as isize) != 0
    {
        InputState = (InputState as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                as BATTLE_INPUT_STATE as ::core::ffi::c_int) as BATTLE_INPUT_STATE;
    }
    if *keyState.offset(C2Rust_Unnamed::KEY_SPECIAL.0 as ::core::ffi::c_int as isize)
        != 0
    {
        InputState = (InputState as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                as BATTLE_INPUT_STATE as ::core::ffi::c_int) as BATTLE_INPUT_STATE;
    }
    if *keyState.offset(C2Rust_Unnamed::KEY_ESCAPE.0 as ::core::ffi::c_int as isize) != 0
    {
        InputState = (InputState as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int)
                as BATTLE_INPUT_STATE as ::core::ffi::c_int) as BATTLE_INPUT_STATE;
    }
    if *keyState.offset(C2Rust_Unnamed::KEY_DOWN.0 as ::core::ffi::c_int as isize) != 0 {
        InputState = (InputState as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                as BATTLE_INPUT_STATE as ::core::ffi::c_int) as BATTLE_INPUT_STATE;
    }
    if *keyState.offset(C2Rust_Unnamed::KEY_THRUST.0 as ::core::ffi::c_int as isize) != 0
    {
        InputState = (InputState as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int)
                as BATTLE_INPUT_STATE as ::core::ffi::c_int) as BATTLE_INPUT_STATE;
    }
    return InputState;
}
#[no_mangle]
pub unsafe extern "C" fn CurrentInputToBattleInput(
    mut player: COUNT,
    mut direction: ::core::ffi::c_int,
) -> BATTLE_INPUT_STATE {
    return ControlInputToBattleInput(
        &raw mut *(&raw mut CurrentInputState.key as *mut [::core::ffi::c_int; 8])
            .offset(PlayerControls[player as usize].0 as isize)
            as *mut ::core::ffi::c_int,
        player,
        direction,
    );
}
#[no_mangle]
pub unsafe extern "C" fn PulsedInputToBattleInput(
    mut player: COUNT,
) -> BATTLE_INPUT_STATE {
    return ControlInputToBattleInput(
        &raw mut *(&raw mut PulsedInputState.key as *mut [::core::ffi::c_int; 8])
            .offset(PlayerControls[player as usize].0 as isize)
            as *mut ::core::ffi::c_int,
        player,
        -1 as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn AnyButtonPress(mut CheckSpecial: BOOLEAN) -> BOOLEAN {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    uqm_lua_checked!(UpdateInputState(), BOOLEAN(0));
    i = 0 as ::core::ffi::c_int;
    while i < CONTROL_TEMPLATE::NUM_TEMPLATES.0 as ::core::ffi::c_int {
        j = 0 as ::core::ffi::c_int;
        while j < C2Rust_Unnamed::NUM_KEYS.0 as ::core::ffi::c_int {
            if CurrentInputState.key[i as usize][j as usize] != 0 {
                return BOOLEAN::TRUE;
            }
            j += 1;
        }
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < C2Rust_Unnamed_0::NUM_MENU_KEYS.0 as ::core::ffi::c_int {
        if CurrentInputState.menu[i as usize] != 0 {
            return BOOLEAN::TRUE;
        }
        i += 1;
    }
    return BOOLEAN::FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn DirKeysPress() -> BOOLEAN {
    return BOOLEAN(
        (CurrentInputState
            .menu[C2Rust_Unnamed_0::KEY_MENU_LEFT.0 as ::core::ffi::c_int as usize] != 0
            || CurrentInputState
                .menu[C2Rust_Unnamed_0::KEY_MENU_RIGHT.0 as ::core::ffi::c_int as usize]
                != 0
            || CurrentInputState
                .menu[C2Rust_Unnamed_0::KEY_MENU_UP.0 as ::core::ffi::c_int as usize]
                != 0
            || CurrentInputState
                .menu[C2Rust_Unnamed_0::KEY_MENU_DOWN.0 as ::core::ffi::c_int as usize]
                != 0) as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ActKeysPress() -> BOOLEAN {
    uqm_lua_checked!(UpdateInputState(), BOOLEAN(0));
    return BOOLEAN(
        (CurrentInputState
            .key[PlayerControls[0usize].0
            as usize][C2Rust_Unnamed::KEY_WEAPON.0 as ::core::ffi::c_int as usize] != 0
            || CurrentInputState
                .key[PlayerControls[0usize].0
                as usize][C2Rust_Unnamed::KEY_SPECIAL.0 as ::core::ffi::c_int as usize]
                != 0
            || CurrentInputState
                .key[PlayerControls[0usize].0
                as usize][C2Rust_Unnamed::KEY_ESCAPE.0 as ::core::ffi::c_int as usize]
                != 0
            || CurrentInputState
                .menu[C2Rust_Unnamed_0::KEY_MENU_SELECT.0 as ::core::ffi::c_int as usize]
                != 0
            || CurrentInputState
                .menu[C2Rust_Unnamed_0::KEY_MENU_CANCEL.0 as ::core::ffi::c_int as usize]
                != 0
            || CurrentInputState
                .menu[C2Rust_Unnamed_0::KEY_MENU_SPECIAL.0 as ::core::ffi::c_int
                as usize] != 0) as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
}
unsafe extern "C" fn SDL_GameControllerTypeToString(
    mut r#type: SDL_GameControllerType,
) -> *const ::core::ffi::c_char {
    static mut strings: [*const ::core::ffi::c_char; 15] = [
        b"Unknown\0".as_ptr() as *const ::core::ffi::c_char,
        b"Xbox 360\0".as_ptr() as *const ::core::ffi::c_char,
        b"Xbox One\0".as_ptr() as *const ::core::ffi::c_char,
        b"PlayStation 3\0".as_ptr() as *const ::core::ffi::c_char,
        b"PlayStation 4\0".as_ptr() as *const ::core::ffi::c_char,
        b"Nintendo Switch Pro\0".as_ptr() as *const ::core::ffi::c_char,
        b"Virtual\0".as_ptr() as *const ::core::ffi::c_char,
        b"PlayStation 5\0".as_ptr() as *const ::core::ffi::c_char,
        b"Amazon Luna\0".as_ptr() as *const ::core::ffi::c_char,
        b"Google Stadia\0".as_ptr() as *const ::core::ffi::c_char,
        b"NVIDIA Shield\0".as_ptr() as *const ::core::ffi::c_char,
        b"Nintendo Switch Joy-Con (Left)\0".as_ptr() as *const ::core::ffi::c_char,
        b"Nintendo Switch Joy-Con (Right)\0".as_ptr() as *const ::core::ffi::c_char,
        b"Nintendo Switch Joy-Con Pair\0".as_ptr() as *const ::core::ffi::c_char,
        b"MAX (Invalid)\0".as_ptr() as *const ::core::ffi::c_char,
    ];
    if r#type.0 as usize
        >= ::core::mem::size_of::<[*const ::core::ffi::c_char; 15]>()
            .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>())
    {
        return b"Unknown (Invalid)\0".as_ptr() as *const ::core::ffi::c_char;
    }
    return strings[r#type.0 as usize];
}
static mut input_tracker: LAST_INPUT = LAST_INPUT {
    r#type: -1 as ::core::ffi::c_int,
    gamepad: 0,
    pressed: 0,
    actions: 0,
};
unsafe extern "C" fn ControllerTypeSwitcher() {
    let mut i: ::core::ffi::c_int = 0;
    let mut pressed: BOOLEAN = BOOLEAN::FALSE;
    if (*(&raw mut last_input as *mut LAST_INPUT).offset(0isize)).pressed == 0 {
        return;
    }
    if input_tracker.r#type == 1 as ::core::ffi::c_int
        && input_tracker.gamepad
            == (*(&raw mut last_input as *mut LAST_INPUT).offset(0isize)).gamepad
        || input_tracker.r#type == 0 as ::core::ffi::c_int
            && (*(&raw mut last_input as *mut LAST_INPUT).offset(0isize)).r#type
                == 0 as ::core::ffi::c_int
    {
        return;
    }
    i = 0 as ::core::ffi::c_int;
    while i < C2Rust_Unnamed::NUM_KEYS.0 as ::core::ffi::c_int {
        if CurrentInputState.key[PlayerControls[0usize].0 as usize][i as usize] != 0 {
            pressed = BOOLEAN::TRUE;
        }
        i += 1;
    }
    if (*(&raw mut last_input as *mut LAST_INPUT).offset(0isize)).pressed != 0
        && (*(&raw mut last_input as *mut LAST_INPUT).offset(0isize)).actions
            == -1 as ::core::ffi::c_int
    {
        input_tracker.pressed = 0 as ::core::ffi::c_int;
        (*(&raw mut last_input as *mut LAST_INPUT).offset(0isize)).pressed = 0
            as ::core::ffi::c_int;
        return;
    }
    if pressed.0 != 0
        && (*(&raw mut last_input as *mut LAST_INPUT).offset(0isize)).r#type
            == 1 as ::core::ffi::c_int
        && input_tracker.gamepad
            != (*(&raw mut last_input as *mut LAST_INPUT).offset(0isize)).gamepad
    {
        let mut gamepad: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        match (*(&raw mut last_input as *mut LAST_INPUT).offset(0isize)).gamepad {
            3 | 4 | 7 => {
                optControllerType = 2 as ::core::ffi::c_int;
            }
            5 | 11 | 12 | 13 => {
                optControllerType = 3 as ::core::ffi::c_int;
            }
            1 | 2 | 6 | _ => {
                optControllerType = 1 as ::core::ffi::c_int;
            }
        }
        gamepad = SDL_GameControllerTypeToString(
            SDL_GameControllerType(
                (*(&raw mut last_input as *mut LAST_INPUT).offset(0isize)).gamepad
                    as ::core::ffi::c_uint,
            ),
        );
        log_add(
            log_Level::log_Info,
            b"Last input player 1: CONTROLLER -> type: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            gamepad,
        );
    }
    if pressed.0 != 0
        && (*(&raw mut last_input as *mut LAST_INPUT).offset(0isize)).r#type
            == 0 as ::core::ffi::c_int && optControllerType > 0 as ::core::ffi::c_int
    {
        optControllerType = 0 as ::core::ffi::c_int;
        log_add(
            log_Level::log_Info,
            b"Last input player 1: KEYBOARD\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if pressed.0 != 0
        && (*(&raw mut last_input as *mut LAST_INPUT).offset(0isize)).pressed != 0
    {
        input_tracker = *(&raw mut last_input as *mut LAST_INPUT).offset(0isize);
        pressed = BOOLEAN(input_tracker.pressed as ::core::ffi::c_uint);
    }
}
#[no_mangle]
pub unsafe extern "C" fn ConfirmExit() -> BOOLEAN {
    let mut old_max_accel: DWORD = 0;
    let mut old_min_accel: DWORD = 0;
    let mut old_step_accel: DWORD = 0;
    let mut old_gestalt_keys: BOOLEAN = BOOLEAN::FALSE;
    let mut result: BOOLEAN = BOOLEAN::FALSE;
    old_max_accel = _max_accel;
    old_min_accel = _min_accel;
    old_step_accel = _step_accel;
    old_gestalt_keys = _gestalt_keys;
    SetDefaultMenuRepeatDelay();
    result = uqm_lua_checked!(DoConfirmExit(), BOOLEAN(0));
    SetMenuRepeatDelay(old_min_accel, old_max_accel, old_step_accel, old_gestalt_keys);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn TestSpeechSound(mut snd: STRING) {
    uqm_lua_checked!(PlaySpeechEffect(
        snd,
        NotPositional(),
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
        0 as BYTE,
    ), ());
}
#[inline]
unsafe extern "C" fn atan2i(
    mut y: ::core::ffi::c_int,
    mut x: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut angle: ::core::ffi::c_int = 0;
    let mut abs_y: ::core::ffi::c_int = abs(y);
    if abs_y == 0 as ::core::ffi::c_int {
        abs_y = 1 as ::core::ffi::c_int;
    }
    if x >= 0 as ::core::ffi::c_int {
        angle = C2Rust_Unnamed_2::atan2i_coeff_1.0 as ::core::ffi::c_int
            - C2Rust_Unnamed_2::atan2i_coeff_1.0 as ::core::ffi::c_int * (x - abs_y)
                / (x + abs_y);
    } else {
        angle = C2Rust_Unnamed_2::atan2i_coeff_2.0 as ::core::ffi::c_int
            - C2Rust_Unnamed_2::atan2i_coeff_1.0 as ::core::ffi::c_int * (x + abs_y)
                / (abs_y - x);
    }
    if y < 0 as ::core::ffi::c_int { return -angle } else { return angle };
}
#[no_mangle]
pub unsafe extern "C" fn GetDirectionalJoystickInput(
    mut direction: ::core::ffi::c_int,
    mut player: ::core::ffi::c_int,
) -> BATTLE_INPUT_STATE {
    let mut axisX: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut axisY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut InputState: BATTLE_INPUT_STATE = 0 as BATTLE_INPUT_STATE;
    let mut instance_id: SDL_JoystickID = -1 as SDL_JoystickID;
    if DirJoyActive.0 == 0 || optDirJoy[player as usize] == 0 {
        return InputState;
    }
    if optDirJoy[player as usize] == 1 as ::core::ffi::c_int
        || optDirJoy[player as usize] == 3 as ::core::ffi::c_int
    {
        axisX = VControl_GetJoyAxis(
            player,
            SDL_GameControllerAxis::SDL_CONTROLLER_AXIS_LEFTX,
        );
        axisY = VControl_GetJoyAxis(
            player,
            SDL_GameControllerAxis::SDL_CONTROLLER_AXIS_LEFTY,
        );
    }
    if optDirJoy[player as usize] == 2 as ::core::ffi::c_int
        || optDirJoy[player as usize] == 4 as ::core::ffi::c_int
    {
        axisX = VControl_GetJoyAxis(
            player,
            SDL_GameControllerAxis::SDL_CONTROLLER_AXIS_RIGHTX,
        );
        axisY = VControl_GetJoyAxis(
            player,
            SDL_GameControllerAxis::SDL_CONTROLLER_AXIS_RIGHTY,
        );
    }
    if axisX != 0 as ::core::ffi::c_int || axisY != 0 as ::core::ffi::c_int {
        let mut angle: ::core::ffi::c_int = atan2i(axisY, axisX);
        let mut diff: ::core::ffi::c_int = 0;
        angle
            += C2Rust_Unnamed_2::atan2i_PI.0 as ::core::ffi::c_int
                / C2Rust_Unnamed_2::SHIP_DIRECTIONS.0 as ::core::ffi::c_int;
        if angle < 0 as ::core::ffi::c_int {
            angle
                += C2Rust_Unnamed_2::atan2i_PI.0 as ::core::ffi::c_int
                    * 2 as ::core::ffi::c_int;
        }
        if angle
            > C2Rust_Unnamed_2::atan2i_PI.0 as ::core::ffi::c_int
                * 2 as ::core::ffi::c_int
        {
            angle
                -= C2Rust_Unnamed_2::atan2i_PI.0 as ::core::ffi::c_int
                    * 2 as ::core::ffi::c_int;
        }
        angle = angle * C2Rust_Unnamed_2::SHIP_DIRECTIONS.0 as ::core::ffi::c_int
            / C2Rust_Unnamed_2::atan2i_PI.0 as ::core::ffi::c_int
            / 2 as ::core::ffi::c_int;
        diff = angle - direction
            - C2Rust_Unnamed_2::SHIP_DIRECTIONS.0 as ::core::ffi::c_int
                / 4 as ::core::ffi::c_int;
        while diff >= C2Rust_Unnamed_2::SHIP_DIRECTIONS.0 as ::core::ffi::c_int {
            diff -= C2Rust_Unnamed_2::SHIP_DIRECTIONS.0 as ::core::ffi::c_int;
        }
        while diff < 0 as ::core::ffi::c_int {
            diff += C2Rust_Unnamed_2::SHIP_DIRECTIONS.0 as ::core::ffi::c_int;
        }
        if diff
            < C2Rust_Unnamed_2::SHIP_DIRECTIONS.0 as ::core::ffi::c_int
                / 2 as ::core::ffi::c_int
        {
            InputState = (InputState as ::core::ffi::c_int
                | ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int)
                    as BATTLE_INPUT_STATE as ::core::ffi::c_int) as BATTLE_INPUT_STATE;
        }
        if diff
            > C2Rust_Unnamed_2::SHIP_DIRECTIONS.0 as ::core::ffi::c_int
                / 2 as ::core::ffi::c_int
        {
            InputState = (InputState as ::core::ffi::c_int
                | ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
                    as BATTLE_INPUT_STATE as ::core::ffi::c_int) as BATTLE_INPUT_STATE;
        }
        if optDirJoy[player as usize] > 2 as ::core::ffi::c_int
            && (diff > 6 as ::core::ffi::c_int && diff < 10 as ::core::ffi::c_int)
        {
            let mut undead_zone: ::core::ffi::c_int = 0;
            let mut dzone: ::core::ffi::c_int = 6881 as ::core::ffi::c_int;
            if optDirJoy[player as usize] == 1 as ::core::ffi::c_int
                || optDirJoy[player as usize] == 2 as ::core::ffi::c_int
            {
                dzone = DeadZoneLeftStick[player as usize];
            }
            if optDirJoy[player as usize] == 3 as ::core::ffi::c_int
                || optDirJoy[player as usize] == 4 as ::core::ffi::c_int
            {
                dzone = DeadZoneRightStick[player as usize];
            }
            undead_zone = ((32767 as ::core::ffi::c_int - dzone) as ::core::ffi::c_float
                as ::core::ffi::c_double * 0.65f64 + dzone as ::core::ffi::c_double)
                as ::core::ffi::c_int;
            if dzone as ::core::ffi::c_double
                >= 32767 as ::core::ffi::c_int as ::core::ffi::c_double * 0.35f64
            {
                undead_zone = 0 as ::core::ffi::c_int;
            }
            if abs(axisX) > undead_zone || abs(axisY) > undead_zone {
                InputState = (InputState as ::core::ffi::c_int
                    | ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                        as BATTLE_INPUT_STATE as ::core::ffi::c_int)
                    as BATTLE_INPUT_STATE;
            }
        }
    }
    return InputState;
}
