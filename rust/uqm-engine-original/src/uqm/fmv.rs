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

// Rust translation of src/uqm/fmv.c.
extern "C" {
    pub type context_desc;
    pub type frame_desc;
    pub type drawable_desc;
    pub type tfb_soundsample;
    fn sprintf(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn SetContext(Context: CONTEXT) -> CONTEXT;
    fn SetContextClipRect(pRect: *mut RECT) -> BOOLEAN;
    fn GetContextClipRect(pRect: *mut RECT) -> BOOLEAN;
    fn DrawStamp(pStamp: *mut STAMP);
    fn DestroyDrawable(Drawable: DRAWABLE) -> BOOLEAN;
    fn LoadGraphicInstance(res: RESOURCE) -> *mut ::core::ffi::c_void;
    fn CaptureDrawable(Drawable: DRAWABLE) -> FRAME;
    fn ReleaseDrawable(Frame: FRAME) -> DRAWABLE;
    fn FadeScreen(fadeType: ScreenFadeType, TimeInterval: SIZE) -> DWORD;
    fn FlushColorXForms();
    fn ShowPresentation(presentation: RESOURCE) -> BOOLEAN;
    fn WaitForAnyButton(
        newButton: BOOLEAN,
        duration: TimePeriod,
        resetInput: BOOLEAN,
    ) -> BOOLEAN;
    fn FreeHyperData();
    fn SleepThreadUntil(wakeTime: TimeCount);
    fn StopMusic();
    fn PlayMusic(MusicRef: MUSIC_REF, Continuous: BOOLEAN, Priority: BYTE);
    static mut GlobData: GLOBDATA;
    static mut ScreenContext: CONTEXT;
    fn LoadIntoExtraScreen(r: *mut RECT);
    fn DrawFromExtraScreen(r: *mut RECT);
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
pub type RESOURCE = *const ::core::ffi::c_char;
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
pub type TFB_SoundSample = tfb_soundsample;
pub type MUSIC_REF = *mut *mut TFB_SoundSample;
pub type TimeCount = DWORD;
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
pub type TimePeriod = DWORD;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed {
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
#[no_mangle]
pub unsafe extern "C" fn DoShipSpin(mut index: COUNT, mut hMusic: MUSIC_REF) {
    let mut vnbuf: [::core::ffi::c_char; 32] = [0; 32];
    let mut old_r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    LoadIntoExtraScreen(::core::ptr::null_mut::<RECT>());
    if !hMusic.is_null() {
        StopMusic();
    }
    FreeHyperData();
    sprintf(
        &raw mut vnbuf as *mut ::core::ffi::c_char,
        b"slides.spins.%02u\0".as_ptr() as *const ::core::ffi::c_char,
        index as ::core::ffi::c_uint,
    );
    ShowPresentation(&raw mut vnbuf as *mut ::core::ffi::c_char as RESOURCE);
    SleepThreadUntil(
        FadeScreen(
            ScreenFadeType::FadeAllToBlack,
            (840 as ::core::ffi::c_int / 4 as ::core::ffi::c_int) as SIZE,
        ),
    );
    FlushColorXForms();
    GetContextClipRect(&raw mut old_r);
    SetContextClipRect(::core::ptr::null_mut::<RECT>());
    DrawFromExtraScreen(::core::ptr::null_mut::<RECT>());
    SetContextClipRect(&raw mut old_r);
    if !hMusic.is_null() {
        PlayMusic(hMusic, BOOLEAN::TRUE, 1 as BYTE);
    }
    SleepThreadUntil(
        FadeScreen(
            ScreenFadeType::FadeAllToColor,
            (840 as ::core::ffi::c_int / 4 as ::core::ffi::c_int) as SIZE,
        ),
    );
    FlushColorXForms();
}
#[no_mangle]
pub unsafe extern "C" fn SplashScreen(
    mut DoProcessing: Option<unsafe extern "C" fn(DWORD) -> ()>,
) {
    let mut s: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    let mut TimeOut: DWORD = 0;
    SleepThreadUntil(
        FadeScreen(
            ScreenFadeType::FadeAllToBlack,
            (840 as ::core::ffi::c_int / 120 as ::core::ffi::c_int) as SIZE,
        ),
    );
    SetContext(ScreenContext);
    s.origin.y = 0 as COORD;
    s.origin.x = s.origin.y;
    s.frame = CaptureDrawable(
        LoadGraphicInstance(b"graphics.title\0".as_ptr() as RESOURCE) as DRAWABLE,
    );
    DrawStamp(&raw mut s);
    DestroyDrawable(ReleaseDrawable(s.frame));
    TimeOut = FadeScreen(
        ScreenFadeType::FadeAllToColor,
        (840 as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as SIZE,
    );
    if DoProcessing.is_some() {
        DoProcessing.expect("non-null function pointer")(TimeOut);
    }
    if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
        & C2Rust_Unnamed::CHECK_ABORT.0 as ::core::ffi::c_int != 0
    {
        return;
    }
    WaitForAnyButton(
        BOOLEAN::FALSE,
        (840 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as TimePeriod,
        BOOLEAN::TRUE,
    );
    if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
        & C2Rust_Unnamed::CHECK_ABORT.0 as ::core::ffi::c_int != 0
    {
        return;
    }
    GlobData.Game_state.CurrentActivity = (GlobData.Game_state.CurrentActivity
        as ::core::ffi::c_int & !(C2Rust_Unnamed::CHECK_ABORT.0 as ::core::ffi::c_int))
        as ACTIVITY;
    SleepThreadUntil(
        FadeScreen(
            ScreenFadeType::FadeAllToBlack,
            (840 as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as SIZE,
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn Introduction() {
    ShowPresentation(b"slides.intro\0".as_ptr() as RESOURCE);
    SleepThreadUntil(
        FadeScreen(
            ScreenFadeType::FadeAllToBlack,
            (840 as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as SIZE,
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn Victory() {
    SleepThreadUntil(
        FadeScreen(
            ScreenFadeType::FadeAllToBlack,
            (840 as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as SIZE,
        ),
    );
    ShowPresentation(b"slides.ending\0".as_ptr() as RESOURCE);
    FadeScreen(ScreenFadeType::FadeAllToBlack, 0 as SIZE);
}
