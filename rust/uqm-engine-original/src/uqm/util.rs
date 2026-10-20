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

// Rust translation of src/uqm/util.c.
extern "C" {
    pub type context_desc;
    pub type frame_desc;
    pub type drawable_desc;
    fn SetContext(Context: CONTEXT) -> CONTEXT;
    fn SetContextForeGroundColor(Color: Color) -> Color;
    fn SetContextClipRect(pRect: *mut RECT) -> BOOLEAN;
    fn GetContextClipRect(pRect: *mut RECT) -> BOOLEAN;
    fn SetContextOrigin(orgOffset: POINT) -> POINT;
    fn CopyContextRect(area: *const RECT) -> DRAWABLE;
    fn DrawStamp(pStamp: *mut STAMP);
    fn DrawFilledRectangle(pRect: *mut RECT);
    fn FlushGraphics();
    fn DestroyDrawable(Drawable: DRAWABLE) -> BOOLEAN;
    fn GetFrameRect(Frame: FRAME, pRect: *mut RECT) -> BOOLEAN;
    fn CaptureDrawable(Drawable: DRAWABLE) -> FRAME;
    fn ReleaseDrawable(Frame: FRAME) -> DRAWABLE;
    fn SetSystemRect(pRect: *const RECT);
    fn ClearSystemRect();
    fn GetTimeCounter() -> TimeCount;
    fn SleepThread(timePeriod: TimePeriod);
    fn TaskSwitch();
    static mut ImmediateInputState: CONTROLLER_INPUT_STATE;
    fn FlushInput();
    static mut GamePaused: BOOLEAN;
    static mut GlobData: GLOBDATA;
    static mut ActivityFrame: FRAME;
    static mut ScreenContext: CONTEXT;
    static mut LastActivity: ACTIVITY;
    fn ResumeMusic();
    fn PauseMusic();
    fn AnyButtonPress(DetectSpecial: BOOLEAN) -> BOOLEAN;
    static mut QuitPosted: ::core::ffi::c_int;
    static mut GameActive: ::core::ffi::c_int;
    fn BeginInputFrame();
    fn PauseTrack();
    fn ResumeTrack();
    fn PlayingTrack() -> COUNT;
    fn TFB_SeedRandom(seed: DWORD) -> DWORD;
    fn log_add(_: log_Level, fmt: *const ::core::ffi::c_char, ...);
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Color {
    pub r: BYTE,
    pub g: BYTE,
    pub b: BYTE,
    pub a: BYTE,
}
pub type CONTEXT_DESC = context_desc;
pub type FRAME_DESC = frame_desc;
pub type DRAWABLE_DESC = drawable_desc;
pub type CONTEXT = *mut CONTEXT_DESC;
pub type FRAME = *mut FRAME_DESC;
pub type DRAWABLE = *mut DRAWABLE_DESC;
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
pub type TimeCount = DWORD;
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
#[inline]
unsafe extern "C" fn MAKE_POINT(mut x: COORD, mut y: COORD) -> POINT {
    let mut pt: POINT = point { x: x, y: y };
    return pt;
}
#[no_mangle]
pub unsafe extern "C" fn DrawStarConBox(
    mut pRect: *mut RECT,
    mut BorderWidth: SIZE,
    mut TopLeftColor: Color,
    mut BottomRightColor: Color,
    mut FillInterior: BOOLEAN,
    mut InteriorColor: Color,
) {
    let mut locRect: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    if BorderWidth as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        BorderWidth = 2 as SIZE;
    } else {
        SetContextForeGroundColor(TopLeftColor);
        locRect.corner = (*pRect).corner;
        locRect.extent.width = (*pRect).extent.width;
        locRect.extent.height = 1 as COORD;
        DrawFilledRectangle(&raw mut locRect);
        if BorderWidth as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            locRect.corner.x += 1;
            locRect.corner.y += 1;
            locRect.extent.width = (locRect.extent.width as ::core::ffi::c_int
                - 2 as ::core::ffi::c_int) as COORD;
            DrawFilledRectangle(&raw mut locRect);
        }
        locRect.corner = (*pRect).corner;
        locRect.extent.width = 1 as COORD;
        locRect.extent.height = (*pRect).extent.height;
        DrawFilledRectangle(&raw mut locRect);
        if BorderWidth as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            locRect.corner.x += 1;
            locRect.corner.y += 1;
            locRect.extent.height = (locRect.extent.height as ::core::ffi::c_int
                - 2 as ::core::ffi::c_int) as COORD;
            DrawFilledRectangle(&raw mut locRect);
        }
        SetContextForeGroundColor(BottomRightColor);
        locRect.corner.x = ((*pRect).corner.x as ::core::ffi::c_int
            + (*pRect).extent.width as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
            as COORD;
        locRect.corner.y = ((*pRect).corner.y as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int) as COORD;
        locRect.extent.height = ((*pRect).extent.height as ::core::ffi::c_int
            - 1 as ::core::ffi::c_int) as COORD;
        DrawFilledRectangle(&raw mut locRect);
        if BorderWidth as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            locRect.corner.x -= 1;
            locRect.corner.y += 1;
            locRect.extent.height = (locRect.extent.height as ::core::ffi::c_int
                - 2 as ::core::ffi::c_int) as COORD;
            DrawFilledRectangle(&raw mut locRect);
        }
        locRect.corner.x = (*pRect).corner.x;
        locRect.extent.width = (*pRect).extent.width;
        locRect.corner.y = ((*pRect).corner.y as ::core::ffi::c_int
            + (*pRect).extent.height as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
            as COORD;
        locRect.extent.height = 1 as COORD;
        DrawFilledRectangle(&raw mut locRect);
        if BorderWidth as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            locRect.corner.x += 1;
            locRect.corner.y -= 1;
            locRect.extent.width = (locRect.extent.width as ::core::ffi::c_int
                - 2 as ::core::ffi::c_int) as COORD;
            DrawFilledRectangle(&raw mut locRect);
        }
    }
    if FillInterior.0 != 0 {
        SetContextForeGroundColor(InteriorColor);
        locRect.corner.x = ((*pRect).corner.x as ::core::ffi::c_int
            + BorderWidth as ::core::ffi::c_int) as COORD;
        locRect.corner.y = ((*pRect).corner.y as ::core::ffi::c_int
            + BorderWidth as ::core::ffi::c_int) as COORD;
        locRect.extent.width = ((*pRect).extent.width as ::core::ffi::c_int
            - ((BorderWidth as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)) as COORD;
        locRect.extent.height = ((*pRect).extent.height as ::core::ffi::c_int
            - ((BorderWidth as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)) as COORD;
        DrawFilledRectangle(&raw mut locRect);
    }
}
#[no_mangle]
pub unsafe extern "C" fn SeedRandomNumbers() -> DWORD {
    let mut cur_time: DWORD = 0;
    cur_time = GetTimeCounter() as DWORD;
    TFB_SeedRandom(cur_time);
    return cur_time;
}
#[no_mangle]
pub unsafe extern "C" fn SaveContextFrame(mut saveRect: *const RECT) -> STAMP {
    let mut s: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    if !saveRect.is_null() {
        s.origin = (*saveRect).corner;
    } else {
        s.origin.x = 0 as COORD;
        s.origin.y = 0 as COORD;
    }
    s.frame = CaptureDrawable(CopyContextRect(saveRect));
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn PauseGame() -> BOOLEAN {
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut s: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    let mut OldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    let mut saveStamp: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    let mut ctxRect: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut oldOrigin: POINT = POINT { x: 0, y: 0 };
    let mut OldRect: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    if ActivityFrame.is_null()
        || GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
            & (C2Rust_Unnamed_0::CHECK_ABORT.0 as ::core::ffi::c_int
                | C2Rust_Unnamed_0::CHECK_PAUSE.0 as ::core::ffi::c_int) != 0
        || LastActivity as ::core::ffi::c_int
            & (C2Rust_Unnamed_0::CHECK_LOAD.0 as ::core::ffi::c_int
                | C2Rust_Unnamed_0::CHECK_RESTART.0 as ::core::ffi::c_int) != 0
    {
        return BOOLEAN::FALSE;
    }
    GlobData.Game_state.CurrentActivity = (GlobData.Game_state.CurrentActivity
        as ::core::ffi::c_int | C2Rust_Unnamed_0::CHECK_PAUSE.0 as ::core::ffi::c_int)
        as ACTIVITY;
    if PlayingTrack() != 0 {
        PauseTrack();
    }
    OldContext = SetContext(ScreenContext);
    oldOrigin = SetContextOrigin(MAKE_POINT(0 as COORD, 0 as COORD));
    GetContextClipRect(&raw mut OldRect);
    SetContextClipRect(::core::ptr::null_mut::<RECT>());
    GetContextClipRect(&raw mut ctxRect);
    GetFrameRect(ActivityFrame, &raw mut r);
    r.corner.x = (ctxRect.extent.width as ::core::ffi::c_int
        - r.extent.width as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as COORD;
    r.corner.y = (ctxRect.extent.height as ::core::ffi::c_int
        - r.extent.height as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as COORD;
    saveStamp = SaveContextFrame(&raw mut r);
    s.origin = r.corner;
    s.frame = ActivityFrame;
    SetSystemRect(&raw mut r);
    DrawStamp(&raw mut s);
    FlushGraphics();
    while ::core::ptr::read_volatile::<
        ::core::ffi::c_int,
    >(
        &raw const ImmediateInputState
            .menu[C2Rust_Unnamed::KEY_PAUSE.0 as ::core::ffi::c_int as usize],
    ) != 0 && ::core::ptr::read_volatile::<BOOLEAN>(&raw const GamePaused).0 != 0
    {
        BeginInputFrame();
        TaskSwitch();
    }
    while ::core::ptr::read_volatile::<
        ::core::ffi::c_int,
    >(
        &raw const ImmediateInputState
            .menu[C2Rust_Unnamed::KEY_PAUSE.0 as ::core::ffi::c_int as usize],
    ) == 0 && ::core::ptr::read_volatile::<BOOLEAN>(&raw const GamePaused).0 != 0
    {
        BeginInputFrame();
        TaskSwitch();
    }
    while ::core::ptr::read_volatile::<
        ::core::ffi::c_int,
    >(
        &raw const ImmediateInputState
            .menu[C2Rust_Unnamed::KEY_PAUSE.0 as ::core::ffi::c_int as usize],
    ) != 0 && ::core::ptr::read_volatile::<BOOLEAN>(&raw const GamePaused).0 != 0
    {
        BeginInputFrame();
        TaskSwitch();
    }
    ::core::ptr::write_volatile(&raw mut GamePaused, BOOLEAN::FALSE);
    DrawStamp(&raw mut saveStamp);
    DestroyDrawable(ReleaseDrawable(saveStamp.frame));
    ClearSystemRect();
    SetContextClipRect(&raw mut OldRect);
    SetContextOrigin(oldOrigin);
    SetContext(OldContext);
    WaitForNoInput(
        (840 as ::core::ffi::c_int / 4 as ::core::ffi::c_int) as TimePeriod,
        BOOLEAN::TRUE,
    );
    if PlayingTrack() != 0 {
        ResumeTrack();
    }
    TaskSwitch();
    GlobData.Game_state.CurrentActivity = (GlobData.Game_state.CurrentActivity
        as ::core::ffi::c_int & !(C2Rust_Unnamed_0::CHECK_PAUSE.0 as ::core::ffi::c_int))
        as ACTIVITY;
    return BOOLEAN::TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn WaitForAnyButtonUntil(
    mut newButton: BOOLEAN,
    mut timeOut: TimeCount,
    mut resetInput: BOOLEAN,
) -> BOOLEAN {
    let mut buttonPressed: BOOLEAN = BOOLEAN::FALSE;
    if newButton.0 != 0 && WaitForNoInputUntil(timeOut, BOOLEAN::FALSE).0 == 0 {
        return BOOLEAN::FALSE;
    }
    buttonPressed = AnyButtonPress(BOOLEAN::TRUE);
    while buttonPressed.0 == 0
        && (timeOut == -1 as ::core::ffi::c_int as TimePeriod
            || GetTimeCounter() < timeOut)
        && GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
            & C2Rust_Unnamed_0::CHECK_ABORT.0 as ::core::ffi::c_int == 0
        && ::core::ptr::read_volatile::<::core::ffi::c_int>(&raw const QuitPosted) == 0
    {
        SleepThread(
            (840 as ::core::ffi::c_int / 40 as ::core::ffi::c_int) as TimePeriod,
        );
        buttonPressed = AnyButtonPress(BOOLEAN::TRUE);
    }
    if resetInput.0 != 0 {
        FlushInput();
    }
    return buttonPressed;
}
#[no_mangle]
pub unsafe extern "C" fn WaitForAnyButton(
    mut newButton: BOOLEAN,
    mut duration: TimePeriod,
    mut resetInput: BOOLEAN,
) -> BOOLEAN {
    let mut timeOut: TimeCount = duration;
    if duration != -1 as ::core::ffi::c_int as TimePeriod {
        timeOut = timeOut.wrapping_add(GetTimeCounter());
    }
    return WaitForAnyButtonUntil(newButton, timeOut, resetInput);
}
#[no_mangle]
pub unsafe extern "C" fn WaitForNoInputUntil(
    mut timeOut: TimeCount,
    mut resetInput: BOOLEAN,
) -> BOOLEAN {
    let mut buttonPressed: BOOLEAN = BOOLEAN::FALSE;
    buttonPressed = AnyButtonPress(BOOLEAN::TRUE);
    while buttonPressed.0 != 0
        && (timeOut == -1 as ::core::ffi::c_int as TimePeriod
            || GetTimeCounter() < timeOut)
        && GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
            & C2Rust_Unnamed_0::CHECK_ABORT.0 as ::core::ffi::c_int == 0
        && ::core::ptr::read_volatile::<::core::ffi::c_int>(&raw const QuitPosted) == 0
    {
        SleepThread(
            (840 as ::core::ffi::c_int / 40 as ::core::ffi::c_int) as TimePeriod,
        );
        buttonPressed = AnyButtonPress(BOOLEAN::TRUE);
    }
    if resetInput.0 != 0 {
        FlushInput();
    }
    return BOOLEAN((buttonPressed.0 == 0) as ::core::ffi::c_int as ::core::ffi::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn WaitForNoInput(
    mut duration: TimePeriod,
    mut resetInput: BOOLEAN,
) -> BOOLEAN {
    let mut timeOut: TimeCount = duration;
    if duration != -1 as ::core::ffi::c_int as TimePeriod {
        timeOut = timeOut.wrapping_add(GetTimeCounter());
    }
    return WaitForNoInputUntil(timeOut, resetInput);
}
#[no_mangle]
pub unsafe extern "C" fn SleepGame() {
    if ::core::ptr::read_volatile::<::core::ffi::c_int>(&raw const QuitPosted) != 0 {
        return;
    }
    log_add(
        log_Level::log_Debug,
        b"Game is going to sleep\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if PlayingTrack() != 0 {
        PauseTrack();
    }
    PauseMusic();
    while ::core::ptr::read_volatile::<::core::ffi::c_int>(&raw const GameActive) == 0
        && ::core::ptr::read_volatile::<::core::ffi::c_int>(&raw const QuitPosted) == 0
    {
        SleepThread((840 as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as TimePeriod);
    }
    log_add(
        log_Level::log_Debug,
        b"Game is waking up\0".as_ptr() as *const ::core::ffi::c_char,
    );
    WaitForNoInput(
        (840 as ::core::ffi::c_int / 10 as ::core::ffi::c_int) as TimePeriod,
        BOOLEAN::TRUE,
    );
    ResumeMusic();
    if PlayingTrack() != 0 {
        ResumeTrack();
    }
    TaskSwitch();
}
