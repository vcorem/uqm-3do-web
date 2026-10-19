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
    pub type string_table_entry;
    pub type context_desc;
    pub type frame_desc;
    pub type drawable_desc;
    pub type tfb_soundsample;
    fn __assert_fail(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    ) -> !;
    fn sprintf(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn SetContext(Context: CONTEXT) -> CONTEXT;
    fn SetContextForeGroundColor(Color: Color) -> Color;
    fn SetContextClipRect(pRect: *mut RECT) -> BOOLEAN;
    fn GetContextClipRect(pRect: *mut RECT) -> BOOLEAN;
    fn DrawStamp(pStamp: *mut STAMP);
    fn DrawFilledRectangle(pRect: *mut RECT);
    fn DestroyDrawable(Drawable: DRAWABLE) -> BOOLEAN;
    fn LoadGraphicInstance(res: RESOURCE) -> *mut ::core::ffi::c_void;
    fn SetAbsFrameIndex(Frame: FRAME, FrameIndex: COUNT) -> FRAME;
    fn CaptureDrawable(Drawable: DRAWABLE) -> FRAME;
    fn ReleaseDrawable(Frame: FRAME) -> DRAWABLE;
    fn FadeScreen(fadeType: ScreenFadeType, TimeInterval: SIZE) -> DWORD;
    fn FlushColorXForms();
    fn ShowPresentation(presentation: RESOURCE) -> BOOLEAN;
    fn GetTimeCounter() -> TimeCount;
    static mut optWhichIntro: ::core::ffi::c_int;
    static mut resolutionFactor: ::core::ffi::c_uint;
    static mut optRequiresReload: BOOLEAN;
    static mut optSkipIntro: OPT_ENABLABLE;
    static mut optShipSeed: OPT_ENABLABLE;
    static mut optFlagshipColor: ::core::ffi::c_int;
    static mut optWindowType: DWORD;
    static mut CanvasHeight: ::core::ffi::c_int;
    fn FreeHyperData();
    static mut master_q: QUEUE;
    fn FindMasterShipIndex(ship_ref: SPECIES_ID) -> ::core::ffi::c_int;
    fn StopMusic();
    fn PlayMusic(MusicRef: MUSIC_REF, Continuous: BOOLEAN, Priority: BYTE);
    fn SleepThreadUntil(wakeTime: TimeCount);
    static mut GlobData: GLOBDATA;
    fn isPC(optWhich: ::core::ffi::c_int) -> OPT_CONSOLETYPE;
    fn is3DO(optWhich: ::core::ffi::c_int) -> OPT_CONSOLETYPE;
    static mut ScreenContext: CONTEXT;
    fn LoadIntoExtraScreen(r: *mut RECT);
    fn DrawFromExtraScreen(r: *mut RECT);
    fn ReloadGameContent();
    static mut OutfitOrShipyard: BYTE;
    fn GetStarShipFromIndex(pShipQ: *mut QUEUE, Index: COUNT) -> HLINK;
    fn SeedShip(SpeciesID: SPECIES_ID, loadWindow: BOOLEAN) -> SPECIES_ID;
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
pub type STRING_TABLE_ENTRY_DESC = string_table_entry;
pub type STRING = *mut STRING_TABLE_ENTRY_DESC;
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
pub type TFB_SoundSample = tfb_soundsample;
pub type MUSIC_REF = *mut *mut TFB_SoundSample;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed {
    pub const DEATH_MARCH: Self = Self(0);
    pub const DIED_IN_BATTLE: Self = Self(1);
    pub const SUICIDE: Self = Self(2);
    pub const SURRENDERED: Self = Self(3);
}
pub type TimeCount = DWORD;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct OPT_ENABLABLE(pub ::core::ffi::c_uint);
impl OPT_ENABLABLE {
    pub const OPTVAL_DISABLED: Self = Self(0);
    pub const OPTVAL_ENABLED: Self = Self(1);
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
pub struct OPT_CONSOLETYPE(pub ::core::ffi::c_uint);
impl OPT_CONSOLETYPE {
    pub const OPTVAL_PC: Self = Self(0);
    pub const OPTVAL_3DO: Self = Self(1);
}
pub type HMASTERSHIP = HLINK;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct SPECIES_ID(pub ::core::ffi::c_uint);
impl SPECIES_ID {
    pub const NO_ID: Self = Self(0);
    pub const ARILOU_ID: Self = Self(1);
    pub const CHMMR_ID: Self = Self(2);
    pub const EARTHLING_ID: Self = Self(3);
    pub const ORZ_ID: Self = Self(4);
    pub const PKUNK_ID: Self = Self(5);
    pub const SHOFIXTI_ID: Self = Self(6);
    pub const SPATHI_ID: Self = Self(7);
    pub const SUPOX_ID: Self = Self(8);
    pub const THRADDASH_ID: Self = Self(9);
    pub const UTWIG_ID: Self = Self(10);
    pub const VUX_ID: Self = Self(11);
    pub const YEHAT_ID: Self = Self(12);
    pub const MELNORME_ID: Self = Self(13);
    pub const DRUUGE_ID: Self = Self(14);
    pub const ILWRATH_ID: Self = Self(15);
    pub const MYCON_ID: Self = Self(16);
    pub const SLYLANDRO_ID: Self = Self(17);
    pub const UMGAH_ID: Self = Self(18);
    pub const UR_QUAN_ID: Self = Self(19);
    pub const ZOQFOTPIK_ID: Self = Self(20);
    pub const SYREEN_ID: Self = Self(21);
    pub const KOHR_AH_ID: Self = Self(22);
    pub const ANDROSYNTH_ID: Self = Self(23);
    pub const CHENJESU_ID: Self = Self(24);
    pub const MMRNMHRM_ID: Self = Self(25);
    pub const LAST_MELEE_ID: Self = Self(25);
    pub const SIS_SHIP_ID: Self = Self(26);
    pub const SA_MATRA_ID: Self = Self(27);
    pub const UR_QUAN_PROBE_ID: Self = Self(28);
    pub const NUM_SPECIES_ID: Self = Self(29);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MASTER_SHIP_INFO {
    pub pred: HMASTERSHIP,
    pub succ: HMASTERSHIP,
    pub SpeciesID: SPECIES_ID,
    pub ShipInfo: SHIP_INFO,
    pub Fleet: FLEET_STUFF,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FLEET_STUFF {
    pub strength: COUNT,
    pub known_loc: POINT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SHIP_INFO {
    pub idStr: *const ::core::ffi::c_char,
    pub ship_flags: UWORD,
    pub ship_cost: BYTE,
    pub crew_level: COUNT,
    pub max_crew: COUNT,
    pub energy_level: BYTE,
    pub max_energy: BYTE,
    pub race_strings_rsc: RESOURCE,
    pub icons_rsc: RESOURCE,
    pub melee_icon_rsc: RESOURCE,
    pub race_strings: STRING,
    pub icons: FRAME,
    pub melee_icon: FRAME,
}
pub type LINK = link;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct link {
    pub pred: HLINK,
    pub succ: HLINK,
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
#[no_mangle]
pub unsafe extern "C" fn DoShipSpin(mut index: COUNT, mut hMusic: MUSIC_REF) {
    let mut vnbuf: [::core::ffi::c_char; 24] = [0; 24];
    let mut old_r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut OldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    OldContext = SetContext(ScreenContext);
    if optShipSeed.0 != 0 {
        let mut hMaster: HMASTERSHIP = GetStarShipFromIndex(&raw mut master_q, index);
        let mut mPtr: *mut MASTER_SHIP_INFO = LockMasterShip(&raw mut master_q, hMaster);
        index = FindMasterShipIndex(
            SeedShip(
                (*mPtr).SpeciesID,
                BOOLEAN(0 as ::core::ffi::c_int as ::core::ffi::c_uint),
            ),
        ) as COUNT;
        UnlockLink(&raw mut master_q, hMaster);
    }
    LoadIntoExtraScreen(::core::ptr::null_mut::<RECT>());
    if !hMusic.is_null() {
        uqm_lua_checked!(StopMusic(), ());
    }
    FreeHyperData();
    if isPC(optWhichIntro).0 != 0 {
        if if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0 || OutfitOrShipyard as ::core::ffi::c_int == 3 as ::core::ffi::c_int
        {
            uqm_lua_checked!(SleepThreadUntil(
                FadeScreen(
                    ScreenFadeType::FadeAllToBlack,
                    (840 as ::core::ffi::c_int / 4 as ::core::ffi::c_int) as SIZE,
                ),
            ), ());
            FlushColorXForms();
        }
        uqm_lua_checked!(ShowPresentation(b"slides.accessdata\0".as_ptr() as RESOURCE), ());
    }
    sprintf(
        &raw mut vnbuf as *mut ::core::ffi::c_char,
        b"slides.spins.%02u\0".as_ptr() as *const ::core::ffi::c_char,
        index as ::core::ffi::c_uint,
    );
    uqm_lua_checked!(ShowPresentation(&raw mut vnbuf as *mut ::core::ffi::c_char as RESOURCE), ());
    if is3DO(optWhichIntro).0 != 0 {
        uqm_lua_checked!(SleepThreadUntil(
            FadeScreen(
                ScreenFadeType::FadeAllToBlack,
                (840 as ::core::ffi::c_int / 4 as ::core::ffi::c_int) as SIZE,
            ),
        ), ());
    }
    FlushColorXForms();
    SetContext(OldContext);
    GetContextClipRect(&raw mut old_r);
    SetContextClipRect(::core::ptr::null_mut::<RECT>());
    DrawFromExtraScreen(::core::ptr::null_mut::<RECT>());
    SetContextClipRect(&raw mut old_r);
    if !hMusic.is_null() {
        uqm_lua_checked!(PlayMusic(hMusic, BOOLEAN::TRUE, 1 as BYTE), ());
    }
    if is3DO(optWhichIntro).0 != 0 {
        uqm_lua_checked!(SleepThreadUntil(
            FadeScreen(
                ScreenFadeType::FadeAllToColor,
                (840 as ::core::ffi::c_int / 4 as ::core::ffi::c_int) as SIZE,
            ),
        ), ());
    }
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
    let mut TimeOut: DWORD = 0 as DWORD;
    let mut OverallWait: TimeCount = GetTimeCounter()
        .wrapping_add(
            (840 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as TimeCount,
        );
    if optSkipIntro.0 == 0 {
        SetContext(ScreenContext);
        s.origin.y = 0 as COORD;
        s.origin.x = s.origin.y;
        s.frame = CaptureDrawable(
            uqm_lua_checked!(LoadGraphicInstance(b"graphics.title\0".as_ptr() as RESOURCE), ()) as DRAWABLE,
        );
        if optFlagshipColor == 0x1 as ::core::ffi::c_int {
            s.frame = SetAbsFrameIndex(s.frame, 1 as COUNT);
        } else {
            s.frame = SetAbsFrameIndex(s.frame, 0 as COUNT);
        }
        uqm_lua_checked!(DrawStamp(&raw mut s), ());
        DestroyDrawable(ReleaseDrawable(s.frame));
        TimeOut = FadeScreen(
            ScreenFadeType::FadeAllToColor,
            (840 as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as SIZE,
        );
    }
    if DoProcessing.is_some() {
        uqm_lua_checked!(DoProcessing.expect("non-null function pointer")(TimeOut), ());
    }
    if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
        & C2Rust_Unnamed_0::CHECK_ABORT.0 as ::core::ffi::c_int != 0
    {
        return;
    }
    if optSkipIntro.0 == 0 {
        uqm_lua_checked!(SleepThreadUntil(OverallWait), ());
    }
    if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
        & C2Rust_Unnamed_0::CHECK_ABORT.0 as ::core::ffi::c_int != 0
    {
        return;
    }
    GlobData.Game_state.CurrentActivity = (GlobData.Game_state.CurrentActivity
        as ::core::ffi::c_int & !(C2Rust_Unnamed_0::CHECK_ABORT.0 as ::core::ffi::c_int))
        as ACTIVITY;
    if optSkipIntro.0 == 0 {
        uqm_lua_checked!(SleepThreadUntil(
            FadeScreen(
                ScreenFadeType::FadeAllToBlack,
                (840 as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as SIZE,
            ),
        ), ());
    }
}
#[no_mangle]
pub unsafe extern "C" fn Introduction() {
    uqm_lua_checked!(ShowPresentation(b"slides.intro\0".as_ptr() as RESOURCE), ());
    uqm_lua_checked!(SleepThreadUntil(
        FadeScreen(
            ScreenFadeType::FadeAllToBlack,
            (840 as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as SIZE,
        ),
    ), ());
}
#[no_mangle]
pub unsafe extern "C" fn Victory() {
    uqm_lua_checked!(SleepThreadUntil(
        FadeScreen(
            ScreenFadeType::FadeAllToBlack,
            (840 as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as SIZE,
        ),
    ), ());
    uqm_lua_checked!(ShowPresentation(b"slides.ending\0".as_ptr() as RESOURCE), ());
    FadeScreen(ScreenFadeType::FadeAllToBlack, 0 as SIZE);
}
#[no_mangle]
pub unsafe extern "C" fn GameOver(mut DeathType: BYTE) {
    if DeathType as ::core::ffi::c_int != C2Rust_Unnamed::SUICIDE.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(SleepThreadUntil(
            FadeScreen(
                ScreenFadeType::FadeAllToBlack,
                (840 as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as SIZE,
            ),
        ), ());
    }
    match DeathType as ::core::ffi::c_int {
        0 => {
            uqm_lua_checked!(ShowPresentation(b"slides.deathmarch\0".as_ptr() as RESOURCE), ());
        }
        2 => {
            uqm_lua_checked!(SleepThreadUntil(
                FadeScreen(
                    ScreenFadeType::FadeAllToWhite,
                    (840 as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as SIZE,
                ),
            ), ());
            uqm_lua_checked!(ShowPresentation(b"slides.suicide\0".as_ptr() as RESOURCE), ());
        }
        3 => {
            uqm_lua_checked!(ShowPresentation(b"slides.surrendered\0".as_ptr() as RESOURCE), ());
        }
        1 | _ => {
            uqm_lua_checked!(ShowPresentation(b"slides.defeated\0".as_ptr() as RESOURCE), ());
        }
    }
    FadeScreen(ScreenFadeType::FadeAllToBlack, 0 as SIZE);
}
#[no_mangle]
pub unsafe extern "C" fn Logo() {
    uqm_lua_checked!(ShowPresentation(b"slides.logo\0".as_ptr() as RESOURCE), ());
    FadeScreen(ScreenFadeType::FadeAllToBlack, 0 as SIZE);
}
#[no_mangle]
pub unsafe extern "C" fn Drumall() {
    uqm_lua_checked!(ShowPresentation(b"slides.drumall\0".as_ptr() as RESOURCE), ());
    uqm_lua_checked!(SleepThreadUntil(
        FadeScreen(
            ScreenFadeType::FadeAllToBlack,
            (840 as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as SIZE,
        ),
    ), ());
}
#[no_mangle]
pub unsafe extern "C" fn Reload() {
    uqm_lua_checked!(ShowPresentation(b"slides.reload\0".as_ptr() as RESOURCE), ());
    uqm_lua_checked!(ReloadGameContent(), ());
    uqm_lua_checked!(SleepThreadUntil(
        FadeScreen(
            ScreenFadeType::FadeAllToBlack,
            (840 as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as SIZE,
        ),
    ), ());
}
#[no_mangle]
pub unsafe extern "C" fn AdvanceLoadProgress() {
    if optRequiresReload.0 != 0 {
        let mut r: RECT = RECT {
            corner: POINT { x: 0, y: 0 },
            extent: EXTENT { width: 0, height: 0 },
        };
        static mut i: COUNT = 0 as COUNT;
        r.corner.x = ((16 as ::core::ffi::c_int) << resolutionFactor) as COORD;
        r.corner.y = (CanvasHeight
            - (if if optWindowType == 0 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                30 as ::core::ffi::c_int
            } else {
                50 as ::core::ffi::c_int
            } << resolutionFactor)) as COORD;
        r.extent.height = ((15 as ::core::ffi::c_int) << resolutionFactor) as COORD;
        r.extent.width = (((i as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
            * 7 as ::core::ffi::c_int) << resolutionFactor) as COORD;
        if i as ::core::ffi::c_int == 40 as ::core::ffi::c_int {
            SetContextForeGroundColor(
                buildColorRgba(0 as BYTE, 192 as BYTE, 0 as BYTE, 255 as BYTE),
            );
            i = 0 as COUNT;
        } else {
            SetContextForeGroundColor(
                buildColorRgba(0 as BYTE, 128 as BYTE, 192 as BYTE, 255 as BYTE),
            );
            i = i.wrapping_add(1);
        }
        uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
    }
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
                    b"src/uqm/displist.h\0"
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
                    b"src/uqm/displist.h\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    88 as ::core::ffi::c_int,
                    b"UnlockLink\0".as_ptr() as *const ::core::ffi::c_char,
                );
                0 as ::core::ffi::c_int != 0
            }) as ::core::ffi::c_int;
    }
}
#[inline]
unsafe extern "C" fn LockMasterShip(
    mut pq: *const QUEUE,
    mut h: HMASTERSHIP,
) -> *mut MASTER_SHIP_INFO {
    ((*pq).object_size as usize == ::core::mem::size_of::<MASTER_SHIP_INFO>()
        || {
            __assert_fail(
                b"GetLinkSize (pq) == sizeof (MASTER_SHIP_INFO)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/uqm/master.h\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                50 as ::core::ffi::c_int,
                b"LockMasterShip\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    return LockLink(pq, h) as *mut MASTER_SHIP_INFO;
}
