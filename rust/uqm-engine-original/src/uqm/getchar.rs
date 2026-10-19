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

// Rust translation of src/uqm/getchar.c.
extern "C" {
    pub type string_table_entry;
    pub type string_table;
    pub type frame_desc;
    fn LoadStringTableInstance(res: RESOURCE) -> STRING_TABLE;
    fn DestroyStringTable(StringTable: STRING_TABLE) -> BOOLEAN;
    fn CaptureStringTable(StringTable: STRING_TABLE) -> STRING;
    fn ReleaseStringTable(String: STRING) -> STRING_TABLE;
    fn SetAbsStringTableIndex(String: STRING, StringTableIndex: COUNT) -> STRING;
    fn GetStringAddress(String: STRING) -> STRINGPTR;
    fn HMalloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn HFree(p: *mut ::core::ffi::c_void);
    fn utf8StringCount(start: *const ::core::ffi::c_char) -> size_t;
    fn skipUTF8Chars(
        ptr: *const ::core::ffi::c_char,
        num: size_t,
    ) -> *mut ::core::ffi::c_char;
    fn getStringFromChar(
        ptr: *mut ::core::ffi::c_char,
        size: size_t,
        ch: UniChar,
    ) -> ::core::ffi::c_int;
    fn UniChar_isPrint(ch: UniChar) -> ::core::ffi::c_int;
    fn SleepThread(timePeriod: TimePeriod);
    fn log_add(_: log_Level, fmt: *const ::core::ffi::c_char, ...);
    fn EnterCharacterMode();
    fn ExitCharacterMode();
    fn GetNextCharacter() -> UniChar;
    fn GetLastCharacter() -> UniChar;
    static mut PulsedInputState: CONTROLLER_INPUT_STATE;
    fn DoInput(pInputState: *mut ::core::ffi::c_void, resetInput: BOOLEAN);
    fn memcpy(
        _: *mut ::core::ffi::c_void,
        _: *const ::core::ffi::c_void,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memmove(
        _: *mut ::core::ffi::c_void,
        _: *const ::core::ffi::c_void,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strcmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strlen(_: *const ::core::ffi::c_char) -> size_t;
    static mut GlobData: GLOBDATA;
    fn PlayMenuSound(S: MENU_SOUND_EFFECT);
}
pub type size_t = usize;
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
pub type UniChar = uint32;
pub type STRING_TABLE_ENTRY_DESC = string_table_entry;
pub type STRING_TABLE_DESC = string_table;
pub type STRING_TABLE = *mut STRING_TABLE_DESC;
pub type STRING = *mut STRING_TABLE_ENTRY_DESC;
pub type STRINGPTR = *mut ::core::ffi::c_char;
pub type RESOURCE = *const ::core::ffi::c_char;
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
}
pub type TimePeriod = DWORD;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct joy_char {
    pub len: ::core::ffi::c_uchar,
    pub enc: [::core::ffi::c_char; 7],
}
pub type joy_char_t = joy_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct textentry_state {
    pub InputFunc: Option<unsafe extern "C" fn(*mut textentry_state) -> BOOLEAN>,
    pub Initialized: BOOLEAN,
    pub NextTime: DWORD,
    pub Success: BOOLEAN,
    pub CacheStr: *mut UNICODE,
    pub JoyAlphaString: STRING,
    pub JoystickMode: BOOLEAN,
    pub UpperRegister: BOOLEAN,
    pub JoyAlpha: *mut joy_char_t,
    pub JoyAlphaLength: ::core::ffi::c_int,
    pub JoyUpper: *mut joy_char_t,
    pub JoyLower: *mut joy_char_t,
    pub JoyRegLength: ::core::ffi::c_int,
    pub InsPt: *mut UNICODE,
    pub BaseStr: *mut UNICODE,
    pub CursorPos: ::core::ffi::c_int,
    pub MaxSize: ::core::ffi::c_int,
    pub ChangeCallback: Option<unsafe extern "C" fn(*mut textentry_state) -> BOOLEAN>,
    pub FrameCallback: Option<unsafe extern "C" fn(*mut textentry_state) -> BOOLEAN>,
    pub CbParam: *mut ::core::ffi::c_void,
}
pub type TEXTENTRY_STATE = textentry_state;
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
unsafe extern "C" fn ReadOneChar(
    mut ch: *mut joy_char_t,
    mut str: *const UNICODE,
) -> ::core::ffi::c_int {
    let mut next: *mut UNICODE = skipUTF8Chars(str, 1 as size_t);
    let mut len: ::core::ffi::c_int = next.offset_from(str) as ::core::ffi::c_int;
    (*ch).len = len as ::core::ffi::c_uchar;
    memcpy(
        &raw mut (*ch).enc as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        str as *const ::core::ffi::c_void,
        len as size_t,
    );
    (*ch).enc[len as usize] = '\0' as ::core::ffi::c_char;
    return len;
}
unsafe extern "C" fn LoadJoystickAlpha(
    mut String: STRING,
    mut count: *mut ::core::ffi::c_int,
) -> *mut joy_char_t {
    let mut str: *mut UNICODE = ::core::ptr::null_mut::<UNICODE>();
    let mut c: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut chars: *mut joy_char_t = ::core::ptr::null_mut::<joy_char_t>();
    let mut cur: *mut UNICODE = ::core::ptr::null_mut::<UNICODE>();
    *count = 0 as ::core::ffi::c_int;
    str = GetStringAddress(String) as *mut UNICODE;
    if str.is_null() {
        return ::core::ptr::null_mut::<joy_char_t>();
    }
    c = utf8StringCount(str) as ::core::ffi::c_int;
    chars = HMalloc((c as size_t).wrapping_mul(::core::mem::size_of::<joy_char_t>()))
        as *mut joy_char_t;
    if chars.is_null() {
        return ::core::ptr::null_mut::<joy_char_t>();
    }
    i = 0 as ::core::ffi::c_int;
    cur = str;
    while i < c {
        let mut len: ::core::ffi::c_int = ReadOneChar(chars.offset(i as isize), cur);
        cur = cur.offset(len as isize);
        i += 1;
    }
    *count = c;
    return chars;
}
unsafe extern "C" fn JoyCharFindIn(
    mut ch: *const joy_char_t,
    mut set: *const joy_char_t,
    mut setsize: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < setsize
        && strcmp(
            &raw const (*set.offset(i as isize)).enc as *const ::core::ffi::c_char,
            &raw const (*ch).enc as *const ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
    {
        i += 1;
    }
    return if i < setsize { i } else { -1 as ::core::ffi::c_int };
}
unsafe extern "C" fn JoyCharIsLower(
    mut ch: *const joy_char_t,
    mut pTES: *mut TEXTENTRY_STATE,
) -> ::core::ffi::c_int {
    return (0 as ::core::ffi::c_int
        <= JoyCharFindIn(ch, (*pTES).JoyLower, (*pTES).JoyRegLength))
        as ::core::ffi::c_int;
}
unsafe extern "C" fn JoyCharSwitchReg(
    mut ch: *mut joy_char_t,
    mut from: *const joy_char_t,
    mut to: *const joy_char_t,
    mut regsize: ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = JoyCharFindIn(ch, from, regsize);
    if i >= 0 as ::core::ffi::c_int {
        *ch = *to.offset(i as isize);
    }
}
unsafe extern "C" fn JoyCharToUpper(
    mut outch: *mut joy_char_t,
    mut ch: *const joy_char_t,
    mut pTES: *mut TEXTENTRY_STATE,
) {
    *outch = *ch;
    JoyCharSwitchReg(outch, (*pTES).JoyLower, (*pTES).JoyUpper, (*pTES).JoyRegLength);
}
unsafe extern "C" fn JoyCharToLower(
    mut outch: *mut joy_char_t,
    mut ch: *const joy_char_t,
    mut pTES: *mut TEXTENTRY_STATE,
) {
    *outch = *ch;
    JoyCharSwitchReg(outch, (*pTES).JoyUpper, (*pTES).JoyLower, (*pTES).JoyRegLength);
}
#[no_mangle]
pub unsafe extern "C" fn DoTextEntry(mut pTES: *mut TEXTENTRY_STATE) -> BOOLEAN {
    let mut ch: UniChar = 0;
    let mut pStr: *mut UNICODE = ::core::ptr::null_mut::<UNICODE>();
    let mut CacheInsPt: *mut UNICODE = ::core::ptr::null_mut::<UNICODE>();
    let mut CacheCursorPos: ::core::ffi::c_int = 0;
    let mut len: ::core::ffi::c_int = 0;
    let mut changed: BOOLEAN = BOOLEAN::FALSE;
    if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
        & C2Rust_Unnamed_0::CHECK_ABORT.0 as ::core::ffi::c_int != 0
    {
        return BOOLEAN::FALSE;
    }
    if (*pTES).Initialized.0 == 0 {
        let mut lwlen: ::core::ffi::c_int = 0;
        (*pTES).InputFunc = Some(
            DoTextEntry as unsafe extern "C" fn(*mut TEXTENTRY_STATE) -> BOOLEAN,
        ) as Option<unsafe extern "C" fn(*mut textentry_state) -> BOOLEAN>;
        (*pTES).Success = BOOLEAN::FALSE;
        (*pTES).Initialized = BOOLEAN::TRUE;
        (*pTES).JoystickMode = BOOLEAN::FALSE;
        (*pTES).UpperRegister = BOOLEAN::TRUE;
        if (*pTES).CursorPos as size_t > utf8StringCount((*pTES).BaseStr) {
            (*pTES).CursorPos = utf8StringCount((*pTES).BaseStr) as ::core::ffi::c_int;
        }
        (*pTES).InsPt = skipUTF8Chars((*pTES).BaseStr, (*pTES).CursorPos as size_t)
            as *mut UNICODE;
        (*pTES).JoyAlphaString = CaptureStringTable(
            LoadStringTableInstance(b"text.joyalpha\0".as_ptr() as RESOURCE),
        );
        (*pTES).JoyAlpha = LoadJoystickAlpha(
            SetAbsStringTableIndex((*pTES).JoyAlphaString, 0 as COUNT),
            &raw mut (*pTES).JoyAlphaLength,
        );
        (*pTES).JoyUpper = LoadJoystickAlpha(
            SetAbsStringTableIndex((*pTES).JoyAlphaString, 1 as COUNT),
            &raw mut (*pTES).JoyRegLength,
        );
        (*pTES).JoyLower = LoadJoystickAlpha(
            SetAbsStringTableIndex((*pTES).JoyAlphaString, 2 as COUNT),
            &raw mut lwlen,
        );
        if lwlen != (*pTES).JoyRegLength {
            if lwlen < (*pTES).JoyRegLength {
                (*pTES).JoyRegLength = lwlen;
            }
            log_add(
                log_Level::log_Warning,
                b"Warning: Joystick upper-lower registers size mismatch; using the smallest subset (%d)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                (*pTES).JoyRegLength,
            );
        }
        (*pTES).CacheStr = HMalloc(
            ((*pTES).MaxSize as size_t).wrapping_mul(::core::mem::size_of::<UNICODE>()),
        ) as *mut UNICODE;
        EnterCharacterMode();
        DoInput(pTES as *mut ::core::ffi::c_void, BOOLEAN::TRUE);
        ExitCharacterMode();
        if !(*pTES).CacheStr.is_null() {
            HFree((*pTES).CacheStr as *mut ::core::ffi::c_void);
        }
        if !(*pTES).JoyLower.is_null() {
            HFree((*pTES).JoyLower as *mut ::core::ffi::c_void);
        }
        if !(*pTES).JoyUpper.is_null() {
            HFree((*pTES).JoyUpper as *mut ::core::ffi::c_void);
        }
        if !(*pTES).JoyAlpha.is_null() {
            HFree((*pTES).JoyAlpha as *mut ::core::ffi::c_void);
        }
        DestroyStringTable(ReleaseStringTable((*pTES).JoyAlphaString));
        return (*pTES).Success;
    }
    pStr = (*pTES).InsPt;
    len = strlen(pStr) as ::core::ffi::c_int;
    CacheInsPt = (*pTES).InsPt;
    CacheCursorPos = (*pTES).CursorPos;
    memcpy(
        (*pTES).CacheStr as *mut ::core::ffi::c_void,
        (*pTES).BaseStr as *const ::core::ffi::c_void,
        (*pTES).MaxSize as size_t,
    );
    ch = GetNextCharacter();
    if ch == 0
        && PulsedInputState
            .menu[C2Rust_Unnamed::KEY_MENU_ANY.0 as ::core::ffi::c_int as usize] != 0
    {
        ch = GetLastCharacter();
    }
    while ch != 0 {
        let mut chbuf: [UNICODE; 8] = [0; 8];
        let mut chsize: ::core::ffi::c_int = 0;
        (*pTES).JoystickMode = BOOLEAN::FALSE;
        chsize = getStringFromChar(
            &raw mut chbuf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[UNICODE; 8]>(),
            ch,
        );
        if UniChar_isPrint(ch) != 0 && chsize > 0 as ::core::ffi::c_int {
            if (pStr.offset(len as isize).offset_from((*pTES).BaseStr) + chsize as isize)
                < (*pTES).MaxSize as isize
            {
                memmove(
                    pStr.offset(chsize as isize) as *mut ::core::ffi::c_void,
                    pStr as *const ::core::ffi::c_void,
                    (len + 1 as ::core::ffi::c_int) as size_t,
                );
                memcpy(
                    pStr as *mut ::core::ffi::c_void,
                    &raw mut chbuf as *mut UNICODE as *const ::core::ffi::c_void,
                    chsize as size_t,
                );
                pStr = pStr.offset(chsize as isize);
                (*pTES).CursorPos += 1;
                changed = BOOLEAN::TRUE;
            } else {
                PlayMenuSound(MENU_SOUND_EFFECT::MENU_SOUND_FAILURE);
            }
        }
        ch = GetNextCharacter();
    }
    if PulsedInputState
        .menu[C2Rust_Unnamed::KEY_MENU_DELETE.0 as ::core::ffi::c_int as usize] != 0
    {
        if len != 0 {
            let mut ch_0: joy_char_t = joy_char_t { len: 0, enc: [0; 7] };
            ReadOneChar(&raw mut ch_0, pStr);
            memmove(
                pStr as *mut ::core::ffi::c_void,
                pStr.offset(ch_0.len as ::core::ffi::c_int as isize)
                    as *const ::core::ffi::c_void,
                (len - ch_0.len as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                    as size_t,
            );
            len -= ch_0.len as ::core::ffi::c_int;
            changed = BOOLEAN::TRUE;
        }
    } else if PulsedInputState
        .menu[C2Rust_Unnamed::KEY_MENU_BACKSPACE.0 as ::core::ffi::c_int as usize] != 0
    {
        if pStr > (*pTES).BaseStr {
            let mut prev: *mut UNICODE = skipUTF8Chars(
                (*pTES).BaseStr,
                ((*pTES).CursorPos - 1 as ::core::ffi::c_int) as size_t,
            );
            memmove(
                prev as *mut ::core::ffi::c_void,
                pStr as *const ::core::ffi::c_void,
                (len + 1 as ::core::ffi::c_int) as size_t,
            );
            pStr = prev;
            (*pTES).CursorPos -= 1;
            changed = BOOLEAN::TRUE;
        }
    } else if PulsedInputState
        .menu[C2Rust_Unnamed::KEY_MENU_LEFT.0 as ::core::ffi::c_int as usize] != 0
    {
        if pStr > (*pTES).BaseStr {
            let mut prev_0: *mut UNICODE = skipUTF8Chars(
                (*pTES).BaseStr,
                ((*pTES).CursorPos - 1 as ::core::ffi::c_int) as size_t,
            );
            pStr = prev_0;
            len = (len as ::core::ffi::c_long
                + prev_0.offset_from(pStr) as ::core::ffi::c_long) as ::core::ffi::c_int;
            (*pTES).CursorPos -= 1;
            changed = BOOLEAN::TRUE;
        }
    } else if PulsedInputState
        .menu[C2Rust_Unnamed::KEY_MENU_RIGHT.0 as ::core::ffi::c_int as usize] != 0
    {
        if len > 0 as ::core::ffi::c_int {
            let mut ch_1: joy_char_t = joy_char_t { len: 0, enc: [0; 7] };
            ReadOneChar(&raw mut ch_1, pStr);
            pStr = pStr.offset(ch_1.len as ::core::ffi::c_int as isize);
            len -= ch_1.len as ::core::ffi::c_int;
            (*pTES).CursorPos += 1;
            changed = BOOLEAN::TRUE;
        }
    } else if PulsedInputState
        .menu[C2Rust_Unnamed::KEY_MENU_HOME.0 as ::core::ffi::c_int as usize] != 0
    {
        if pStr > (*pTES).BaseStr {
            pStr = (*pTES).BaseStr;
            len = strlen(pStr) as ::core::ffi::c_int;
            (*pTES).CursorPos = 0 as ::core::ffi::c_int;
            changed = BOOLEAN::TRUE;
        }
    } else if PulsedInputState
        .menu[C2Rust_Unnamed::KEY_MENU_END.0 as ::core::ffi::c_int as usize] != 0
    {
        if len > 0 as ::core::ffi::c_int {
            (*pTES).CursorPos = ((*pTES).CursorPos as size_t)
                .wrapping_add(utf8StringCount(pStr)) as ::core::ffi::c_int;
            pStr = pStr.offset(len as isize);
            len = 0 as ::core::ffi::c_int;
            changed = BOOLEAN::TRUE;
        }
    }
    if !(*pTES).JoyAlpha.is_null()
        && (PulsedInputState
            .menu[C2Rust_Unnamed::KEY_MENU_UP.0 as ::core::ffi::c_int as usize] != 0
            || PulsedInputState
                .menu[C2Rust_Unnamed::KEY_MENU_DOWN.0 as ::core::ffi::c_int as usize]
                != 0
            || PulsedInputState
                .menu[C2Rust_Unnamed::KEY_MENU_PAGE_UP.0 as ::core::ffi::c_int as usize]
                != 0
            || PulsedInputState
                .menu[C2Rust_Unnamed::KEY_MENU_PAGE_DOWN.0 as ::core::ffi::c_int
                as usize] != 0)
    {
        let mut ch_2: joy_char_t = joy_char_t { len: 0, enc: [0; 7] };
        let mut newch: joy_char_t = joy_char_t { len: 0, enc: [0; 7] };
        let mut cmpch: joy_char_t = joy_char_t { len: 0, enc: [0; 7] };
        let mut i: ::core::ffi::c_int = 0;
        let mut curCharUpper: BOOLEAN = BOOLEAN::FALSE;
        (*pTES).JoystickMode = BOOLEAN::TRUE;
        if len != 0 {
            ReadOneChar(&raw mut ch_2, pStr);
            curCharUpper = BOOLEAN(
                (JoyCharIsLower(&raw mut ch_2, pTES) == 0) as ::core::ffi::c_int
                    as ::core::ffi::c_uint,
            );
        } else {
            ch_2 = *(*pTES).JoyAlpha.offset(0isize);
            curCharUpper = (*pTES).UpperRegister;
        }
        newch = ch_2;
        JoyCharToUpper(&raw mut cmpch, &raw mut ch_2, pTES);
        i = JoyCharFindIn(&raw mut cmpch, (*pTES).JoyAlpha, (*pTES).JoyAlphaLength);
        if PulsedInputState
            .menu[C2Rust_Unnamed::KEY_MENU_UP.0 as ::core::ffi::c_int as usize] != 0
        {
            i -= 1;
            if i < 0 as ::core::ffi::c_int {
                i = (*pTES).JoyAlphaLength - 1 as ::core::ffi::c_int;
            }
            newch = *(*pTES).JoyAlpha.offset(i as isize);
        } else if PulsedInputState
            .menu[C2Rust_Unnamed::KEY_MENU_DOWN.0 as ::core::ffi::c_int as usize] != 0
        {
            i += 1;
            if i >= (*pTES).JoyAlphaLength {
                i = 0 as ::core::ffi::c_int;
            }
            newch = *(*pTES).JoyAlpha.offset(i as isize);
        }
        if PulsedInputState
            .menu[C2Rust_Unnamed::KEY_MENU_PAGE_UP.0 as ::core::ffi::c_int as usize] != 0
            || PulsedInputState
                .menu[C2Rust_Unnamed::KEY_MENU_PAGE_DOWN.0 as ::core::ffi::c_int
                as usize] != 0
        {
            if len != 0 {
                if curCharUpper.0 == 0 {
                    JoyCharToUpper(&raw mut newch, &raw mut newch, pTES);
                } else {
                    JoyCharToLower(&raw mut newch, &raw mut newch, pTES);
                }
            } else {
                (*pTES).UpperRegister = BOOLEAN(
                    ((*pTES).UpperRegister.0 == 0) as ::core::ffi::c_int
                        as ::core::ffi::c_uint,
                );
            }
        } else if curCharUpper.0 != 0 {
            JoyCharToUpper(&raw mut newch, &raw mut newch, pTES);
        } else {
            JoyCharToLower(&raw mut newch, &raw mut newch, pTES);
        }
        if strcmp(
            &raw mut newch.enc as *mut ::core::ffi::c_char,
            &raw mut ch_2.enc as *mut ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
        {
            if len != 0 {
                let mut l: ::core::ffi::c_int = len - ch_2.len as ::core::ffi::c_int;
                if (pStr.offset(l as isize).offset_from((*pTES).BaseStr)
                    + newch.len as isize) < (*pTES).MaxSize as isize
                {
                    if newch.len as ::core::ffi::c_int != ch_2.len as ::core::ffi::c_int
                    {
                        memmove(
                            pStr.offset(newch.len as ::core::ffi::c_int as isize)
                                as *mut ::core::ffi::c_void,
                            pStr.offset(ch_2.len as ::core::ffi::c_int as isize)
                                as *const ::core::ffi::c_void,
                            (l + 1 as ::core::ffi::c_int) as size_t,
                        );
                    }
                    memcpy(
                        pStr as *mut ::core::ffi::c_void,
                        &raw mut newch.enc as *mut ::core::ffi::c_char
                            as *const ::core::ffi::c_void,
                        newch.len as size_t,
                    );
                    len = l + newch.len as ::core::ffi::c_int;
                    changed = BOOLEAN::TRUE;
                }
            } else if (pStr.offset(len as isize).offset_from((*pTES).BaseStr)
                + newch.len as isize) < (*pTES).MaxSize as isize
            {
                memcpy(
                    pStr as *mut ::core::ffi::c_void,
                    &raw mut newch.enc as *mut ::core::ffi::c_char
                        as *const ::core::ffi::c_void,
                    newch.len as size_t,
                );
                *pStr.offset(newch.len as isize) = '\0' as UNICODE;
                len += newch.len as ::core::ffi::c_int;
                changed = BOOLEAN::TRUE;
            } else {
                PlayMenuSound(MENU_SOUND_EFFECT::MENU_SOUND_FAILURE);
            }
        }
    }
    if PulsedInputState
        .menu[C2Rust_Unnamed::KEY_MENU_SELECT.0 as ::core::ffi::c_int as usize] != 0
    {
        (*pTES).Success = BOOLEAN::TRUE;
        return BOOLEAN::FALSE;
    } else if PulsedInputState
        .menu[C2Rust_Unnamed::KEY_MENU_EDIT_CANCEL.0 as ::core::ffi::c_int as usize] != 0
    {
        (*pTES).Success = BOOLEAN::FALSE;
        return BOOLEAN::FALSE;
    }
    (*pTES).InsPt = pStr;
    if changed.0 != 0 && (*pTES).ChangeCallback.is_some() {
        if (*pTES).ChangeCallback.expect("non-null function pointer")(pTES).0 == 0 {
            memcpy(
                (*pTES).BaseStr as *mut ::core::ffi::c_void,
                (*pTES).CacheStr as *const ::core::ffi::c_void,
                (*pTES).MaxSize as size_t,
            );
            (*pTES).InsPt = CacheInsPt;
            (*pTES).CursorPos = CacheCursorPos;
            PlayMenuSound(MENU_SOUND_EFFECT::MENU_SOUND_FAILURE);
        }
    }
    if (*pTES).FrameCallback.is_some() {
        return (*pTES).FrameCallback.expect("non-null function pointer")(pTES)
    } else {
        SleepThread(
            (840 as ::core::ffi::c_int / 30 as ::core::ffi::c_int) as TimePeriod,
        );
    }
    return BOOLEAN::TRUE;
}
