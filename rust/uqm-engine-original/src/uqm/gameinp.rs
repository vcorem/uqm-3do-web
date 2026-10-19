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
    fn __assert_fail(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    );
    fn SetAbsStringTableIndex(String: STRING, StringTableIndex: COUNT) -> STRING;
    fn GetTimeCounter() -> TimeCount;
    fn PauseGame() -> BOOLEAN;
    fn SleepGame();
    fn DoConfirmExit() -> BOOLEAN;
    fn PlaySoundEffect(
        S: SOUND,
        Channel: COUNT,
        Pos: SoundPosition,
        PositionalObject: *mut ::core::ffi::c_void,
        Priority: BYTE,
    );
    fn TaskSwitch();
    static mut MenuSounds: SOUND;
    fn NotPositional() -> SoundPosition;
    fn TFB_ResetControls();
    static mut GameActive: ::core::ffi::c_int;
    fn BeginInputFrame();
    fn Async_process();
    fn debugKeyPressedSynchronous();
}
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
    pub const NUM_KEYS: Self = Self(7);
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
pub type BATTLE_INPUT_STATE = UBYTE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MENU_ANNOTATIONS {
    pub key: [[DWORD; 7]; 6],
    pub menu: [DWORD; 24],
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
}
#[no_mangle]
pub static mut PlayerControls: [CONTROL_TEMPLATE; 2] = [CONTROL_TEMPLATE::CONTROL_TEMPLATE_KB_1; 2];
#[no_mangle]
pub static mut PulsedInputState: CONTROLLER_INPUT_STATE = CONTROLLER_INPUT_STATE {
    key: [[0; 7]; 6],
    menu: [0; 24],
};
#[no_mangle]
pub static mut CurrentInputState: CONTROLLER_INPUT_STATE = CONTROLLER_INPUT_STATE {
    key: [[0; 7]; 6],
    menu: [0; 24],
};
static mut OldInputState: CONTROLLER_INPUT_STATE = CONTROLLER_INPUT_STATE {
    key: [[0; 7]; 6],
    menu: [0; 24],
};
static mut CachedInputState: CONTROLLER_INPUT_STATE = CONTROLLER_INPUT_STATE {
    key: [[0; 7]; 6],
    menu: [0; 24],
};
static mut Times: MENU_ANNOTATIONS = MENU_ANNOTATIONS {
    key: [[0; 7]; 6],
    menu: [0; 24],
};
static mut RepeatDelays: MENU_ANNOTATIONS = MENU_ANNOTATIONS {
    key: [[0; 7]; 6],
    menu: [0; 24],
};
static mut GestaltRepeatDelay: DWORD = 0;
static mut GestaltTime: DWORD = 0;
static mut CachedGestalt: BOOLEAN = BOOLEAN::FALSE;
static mut OldGestalt: BOOLEAN = BOOLEAN::FALSE;
static mut _step_accel: DWORD = 0;
static mut _min_accel: DWORD = 0;
static mut _max_accel: DWORD = 0;
static mut _gestalt_keys: BOOLEAN = BOOLEAN::FALSE;
static mut sound_0: MENU_SOUND_FLAGS = 0;
static mut sound_1: MENU_SOUND_FLAGS = 0;
#[no_mangle]
pub static mut ImmediateInputState: CONTROLLER_INPUT_STATE = CONTROLLER_INPUT_STATE {
    key: [[0; 7]; 6],
    menu: [0; 24],
};
#[no_mangle]
pub static mut ExitRequested: BOOLEAN = BOOLEAN::FALSE;
#[no_mangle]
pub static mut GamePaused: BOOLEAN = BOOLEAN::FALSE;
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
        SleepGame();
    }
    if ::core::ptr::read_volatile::<BOOLEAN>(&raw const GamePaused).0 != 0 {
        PauseGame();
    }
    if ::core::ptr::read_volatile::<BOOLEAN>(&raw const ExitRequested).0 != 0 {
        ConfirmExit();
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
                        as *mut [::core::ffi::c_int; 7])
                        .offset(i as isize) as *mut ::core::ffi::c_int)
                        .offset(j as isize),
                    (&raw mut *(&raw mut CachedInputState.key
                        as *mut [::core::ffi::c_int; 7])
                        .offset(i as isize) as *mut ::core::ffi::c_int)
                        .offset(j as isize),
                    (&raw mut *(&raw mut OldInputState.key
                        as *mut [::core::ffi::c_int; 7])
                        .offset(i as isize) as *mut ::core::ffi::c_int)
                        .offset(j as isize),
                    (&raw mut *(&raw mut RepeatDelays.key as *mut [DWORD; 7])
                        .offset(i as isize) as *mut DWORD)
                        .offset(j as isize),
                    &raw mut NewTime,
                    (&raw mut *(&raw mut Times.key as *mut [DWORD; 7]).offset(i as isize)
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
        .menu[C2Rust_Unnamed_0::KEY_DEBUG.0 as ::core::ffi::c_int as usize] != 0
    {
        debugKeyPressedSynchronous();
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
                292 as ::core::ffi::c_int,
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
    _max_accel = (840 as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as DWORD;
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
        Async_process();
        TaskSwitch();
        UpdateInputState();
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
            PlaySoundEffect(
                S,
                0 as COUNT,
                NotPositional(),
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                0 as BYTE,
            );
        }
        if inputCallback.is_some() {
            inputCallback.expect("non-null function pointer")();
        }
        if (*(pInputState as *mut INPUT_STATE_DESC))
            .InputFunc
            .expect("non-null function pointer")(pInputState)
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
) -> BATTLE_INPUT_STATE {
    let mut InputState: BATTLE_INPUT_STATE = 0 as BATTLE_INPUT_STATE;
    if *keyState.offset(C2Rust_Unnamed::KEY_UP.0 as ::core::ffi::c_int as isize) != 0 {
        InputState = (InputState as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                as BATTLE_INPUT_STATE as ::core::ffi::c_int) as BATTLE_INPUT_STATE;
    }
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
    return InputState;
}
#[no_mangle]
pub unsafe extern "C" fn CurrentInputToBattleInput(
    mut player: COUNT,
) -> BATTLE_INPUT_STATE {
    return ControlInputToBattleInput(
        &raw mut *(&raw mut CurrentInputState.key as *mut [::core::ffi::c_int; 7])
            .offset(PlayerControls[player as usize].0 as isize)
            as *mut ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn PulsedInputToBattleInput(
    mut player: COUNT,
) -> BATTLE_INPUT_STATE {
    return ControlInputToBattleInput(
        &raw mut *(&raw mut PulsedInputState.key as *mut [::core::ffi::c_int; 7])
            .offset(PlayerControls[player as usize].0 as isize)
            as *mut ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn AnyButtonPress(mut CheckSpecial: BOOLEAN) -> BOOLEAN {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    UpdateInputState();
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
    result = DoConfirmExit();
    SetMenuRepeatDelay(old_min_accel, old_max_accel, old_step_accel, old_gestalt_keys);
    return result;
}
