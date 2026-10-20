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

// Rust translation of src/uqm/supermelee/buildpick.c.
extern "C" {
    pub type string_table_entry;
    pub type context_desc;
    pub type frame_desc;
    pub type drawable_desc;
    pub type RandomContext;
    pub type tfb_soundsample;
    pub type MeleeTeam;
    pub type MeleeSetup;
    fn MAKE_HOT_SPOT(_: COORD, _: COORD) -> HOT_SPOT;
    fn BoxUnion(pr1: *mut RECT, pr2: *mut RECT, punion: *mut RECT);
    fn SetContext(Context: CONTEXT) -> CONTEXT;
    fn SetContextForeGroundColor(Color: Color) -> Color;
    fn SetContextFGFrame(Frame: FRAME) -> FRAME;
    fn DrawStamp(pStamp: *mut STAMP);
    fn DrawFilledStamp(pStamp: *mut STAMP);
    fn CreateDrawable(
        CreateFlags: CREATE_FLAGS,
        width: SIZE,
        height: SIZE,
        num_frames: COUNT,
    ) -> DRAWABLE;
    fn DestroyDrawable(Drawable: DRAWABLE) -> BOOLEAN;
    fn GetFrameRect(Frame: FRAME, pRect: *mut RECT) -> BOOLEAN;
    fn SetFrameHot(Frame: FRAME, HotSpot: HOT_SPOT) -> HOT_SPOT;
    fn SetAbsFrameIndex(Frame: FRAME, FrameIndex: COUNT) -> FRAME;
    fn CaptureDrawable(Drawable: DRAWABLE) -> FRAME;
    fn ReleaseDrawable(Frame: FRAME) -> DRAWABLE;
    fn GetTimeCounter() -> TimeCount;
    fn GetShipBox(pRect: *mut RECT, side: COUNT, row: COUNT, col: COUNT);
    fn DrawMeleeShipStrings(pMS: *mut MELEE_STATE, NewStarShip: MeleeShip);
    static mut MeleeFrame: FRAME;
    fn Melee_flashSelection(pMS: *mut MELEE_STATE);
    static mut PulsedInputState: CONTROLLER_INPUT_STATE;
    fn FlushInput();
    fn DoInput(pInputState: *mut ::core::ffi::c_void, resetInput: BOOLEAN);
    static mut GamePaused: BOOLEAN;
    fn DoShipSpin(index: COUNT, hMusic: MUSIC_REF);
    fn GetShipIconsFromIndex(Index: ::core::ffi::c_uint) -> FRAME;
    fn SleepThreadUntil(wakeTime: TimeCount);
    static mut GlobData: GLOBDATA;
    static mut OffScreenContext: CONTEXT;
    fn SetMenuSounds(sound_0: MENU_SOUND_FLAGS, sound_1: MENU_SOUND_FLAGS);
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
pub type STRING_TABLE_ENTRY_DESC = string_table_entry;
pub type STRING = *mut STRING_TABLE_ENTRY_DESC;
pub type DIRENTRY = STRING;
pub type CONTEXT_DESC = context_desc;
pub type FRAME_DESC = frame_desc;
pub type DRAWABLE_DESC = drawable_desc;
pub type CONTEXT = *mut CONTEXT_DESC;
pub type FRAME = *mut FRAME_DESC;
pub type DRAWABLE = *mut DRAWABLE_DESC;
pub type COORD = SWORD;
pub type CREATE_FLAGS = BYTE;
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
pub type HOT_SPOT = POINT;
pub type TFB_SoundSample = tfb_soundsample;
pub type MUSIC_REF = *mut *mut TFB_SoundSample;
pub type TimeCount = DWORD;
pub type QUEUE_HANDLE = *mut ::core::ffi::c_void;
pub type HLINK = QUEUE_HANDLE;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct velocity_desc {
    pub TravelAngle: COUNT,
    pub vector: EXTENT,
    pub fract: EXTENT,
    pub error: EXTENT,
    pub incr: EXTENT,
}
pub type VELOCITY_DESC = velocity_desc;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct melee_state {
    pub InputFunc: Option<unsafe extern "C" fn(*mut melee_state) -> BOOLEAN>,
    pub Initialized: BOOLEAN,
    pub meleeStarted: BOOLEAN,
    pub MeleeOption: MELEE_OPTIONS,
    pub side: COUNT,
    pub row: COUNT,
    pub col: COUNT,
    pub meleeSetup: *mut MeleeSetup,
    pub load: melee_load_state,
    pub currentShip: MeleeShip,
    pub CurIndex: COUNT,
    pub buildPickConfirmed: BOOLEAN,
    pub randomContext: *mut RandomContext,
    pub LastInputTime: TimeCount,
    pub hMusic: MUSIC_REF,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct MeleeShip(pub ::core::ffi::c_uint);
impl MeleeShip {
    pub const MELEE_ANDROSYNTH: Self = Self(0);
    pub const MELEE_ARILOU: Self = Self(1);
    pub const MELEE_CHENJESU: Self = Self(2);
    pub const MELEE_CHMMR: Self = Self(3);
    pub const MELEE_DRUUGE: Self = Self(4);
    pub const MELEE_EARTHLING: Self = Self(5);
    pub const MELEE_ILWRATH: Self = Self(6);
    pub const MELEE_KOHR_AH: Self = Self(7);
    pub const MELEE_MELNORME: Self = Self(8);
    pub const MELEE_MMRNMHRM: Self = Self(9);
    pub const MELEE_MYCON: Self = Self(10);
    pub const MELEE_ORZ: Self = Self(11);
    pub const MELEE_PKUNK: Self = Self(12);
    pub const MELEE_SHOFIXTI: Self = Self(13);
    pub const MELEE_SLYLANDRO: Self = Self(14);
    pub const MELEE_SPATHI: Self = Self(15);
    pub const MELEE_SUPOX: Self = Self(16);
    pub const MELEE_SYREEN: Self = Self(17);
    pub const MELEE_THRADDASH: Self = Self(18);
    pub const MELEE_UMGAH: Self = Self(19);
    pub const MELEE_URQUAN: Self = Self(20);
    pub const MELEE_UTWIG: Self = Self(21);
    pub const MELEE_VUX: Self = Self(22);
    pub const MELEE_YEHAT: Self = Self(23);
    pub const MELEE_ZOQFOTPIK: Self = Self(24);
    pub const MELEE_UNSET: Self = Self(254);
    pub const MELEE_NONE: Self = Self(255);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct melee_load_state {
    pub preBuiltList: *mut *mut MeleeTeam,
    pub preBuiltCount: COUNT,
    pub dirEntries: DIRENTRY,
    pub entryIndices: *mut COUNT,
    pub numIndices: COUNT,
    pub view: [*mut MeleeTeam; 5],
    pub top: COUNT,
    pub bot: COUNT,
    pub cur: COUNT,
    pub viewSize: COUNT,
}
pub type MELEE_OPTIONS = BYTE;
pub type MELEE_STATE = melee_state;
pub type CONTROLLER_INPUT_STATE = _controller_input_state;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _controller_input_state {
    pub key: [[::core::ffi::c_int; 7]; 6],
    pub menu: [::core::ffi::c_int; 24],
}
pub type MENU_SOUND_FLAGS = UWORD;
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
static mut BuildPickFrame: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
#[no_mangle]
pub unsafe extern "C" fn BuildBuildPickFrame() {
    let mut s: STAMP = STAMP {
        origin: HOT_SPOT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    let mut r: RECT = RECT {
        corner: HOT_SPOT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut i: COUNT = 0;
    let mut OldContext: CONTEXT = SetContext(OffScreenContext);
    s.origin.x = 0 as COORD;
    s.origin.y = 0 as COORD;
    s.frame = SetAbsFrameIndex(MeleeFrame, 27 as COUNT);
    GetFrameRect(s.frame, &raw mut r);
    BuildPickFrame = CaptureDrawable(
        CreateDrawable(
            ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as CREATE_FLAGS,
            r.extent.width,
            r.extent.height,
            1 as COUNT,
        ),
    );
    SetContextFGFrame(BuildPickFrame);
    SetFrameHot(s.frame, MAKE_HOT_SPOT(0 as COORD, 0 as COORD));
    DrawStamp(&raw mut s);
    i = 0 as COUNT;
    while (i as ::core::ffi::c_int) < 5 as ::core::ffi::c_int * 5 as ::core::ffi::c_int {
        DrawPickIcon(MeleeShip(i as ::core::ffi::c_uint), true);
        i = i.wrapping_add(1);
    }
    SetContext(OldContext);
}
#[no_mangle]
pub unsafe extern "C" fn DestroyBuildPickFrame() {
    DestroyDrawable(ReleaseDrawable(BuildPickFrame));
    BuildPickFrame = ::core::ptr::null_mut::<FRAME_DESC>();
}
#[no_mangle]
pub unsafe extern "C" fn DrawPickIcon(mut ship: MeleeShip, mut DrawErase: bool) {
    let mut s: STAMP = STAMP {
        origin: HOT_SPOT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    let mut r: RECT = RECT {
        corner: HOT_SPOT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    GetFrameRect(BuildPickFrame, &raw mut r);
    s.origin.x = ((r.corner.x as ::core::ffi::c_int + 20 as ::core::ffi::c_int)
        as ::core::ffi::c_uint)
        .wrapping_add(
            ship
                .0
                .wrapping_rem(5 as ::core::ffi::c_uint)
                .wrapping_mul(18 as ::core::ffi::c_uint),
        ) as COORD;
    s.origin.y = ((r.corner.y as ::core::ffi::c_int + 5 as ::core::ffi::c_int)
        as ::core::ffi::c_uint)
        .wrapping_add(
            ship
                .0
                .wrapping_div(5 as ::core::ffi::c_uint)
                .wrapping_mul(18 as ::core::ffi::c_uint),
        ) as COORD;
    s.frame = GetShipIconsFromIndex(ship.0);
    if DrawErase {
        DrawStamp(&raw mut s);
    } else {
        let mut OldColor: Color = Color { r: 0, g: 0, b: 0, a: 0 };
        OldColor = SetContextForeGroundColor(
            colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE),
        );
        DrawFilledStamp(&raw mut s);
        SetContextForeGroundColor(OldColor);
    };
}
#[no_mangle]
pub unsafe extern "C" fn DrawPickFrame(mut pMS: *mut MELEE_STATE) {
    let mut r: RECT = RECT {
        corner: HOT_SPOT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut r0: RECT = RECT {
        corner: HOT_SPOT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut r1: RECT = RECT {
        corner: HOT_SPOT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut ship_r: RECT = RECT {
        corner: HOT_SPOT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut s: STAMP = STAMP {
        origin: HOT_SPOT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    GetShipBox(&raw mut r0, 0 as COUNT, 0 as COUNT, 0 as COUNT);
    GetShipBox(
        &raw mut r1,
        1 as COUNT,
        (2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as COUNT,
        (7 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as COUNT,
    );
    BoxUnion(&raw mut r0, &raw mut r1, &raw mut ship_r);
    s.frame = SetAbsFrameIndex(BuildPickFrame, 0 as COUNT);
    GetFrameRect(s.frame, &raw mut r);
    r.corner.x = -(ship_r.corner.x as ::core::ffi::c_int
        + (ship_r.extent.width as ::core::ffi::c_int
            - r.extent.width as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)) as COORD;
    if (*pMS).side != 0 {
        r.corner.y = -(ship_r.corner.y as ::core::ffi::c_int) as COORD;
    } else {
        r.corner.y = -(ship_r.corner.y as ::core::ffi::c_int
            + (ship_r.extent.height as ::core::ffi::c_int
                - r.extent.height as ::core::ffi::c_int)) as COORD;
    }
    SetFrameHot(s.frame, MAKE_HOT_SPOT(r.corner.x, r.corner.y));
    s.origin.x = 0 as COORD;
    s.origin.y = 0 as COORD;
    DrawStamp(&raw mut s);
    DrawMeleeShipStrings(pMS, (*pMS).currentShip);
}
#[no_mangle]
pub unsafe extern "C" fn GetBuildPickFrameRect(mut r: *mut RECT) {
    GetFrameRect(BuildPickFrame, r);
}
unsafe extern "C" fn DoPickShip(mut pMS: *mut MELEE_STATE) -> BOOLEAN {
    let mut TimeIn: DWORD = GetTimeCounter();
    ::core::ptr::write_volatile(&raw mut GamePaused, BOOLEAN::FALSE);
    if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
        & C2Rust_Unnamed_0::CHECK_ABORT.0 as ::core::ffi::c_int != 0
    {
        (*pMS).buildPickConfirmed = BOOLEAN(
            0 as ::core::ffi::c_int as ::core::ffi::c_uint,
        );
        return BOOLEAN::FALSE;
    }
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
    if PulsedInputState
        .menu[C2Rust_Unnamed::KEY_MENU_SELECT.0 as ::core::ffi::c_int as usize] != 0
        || PulsedInputState
            .menu[C2Rust_Unnamed::KEY_MENU_CANCEL.0 as ::core::ffi::c_int as usize] != 0
    {
        (*pMS).buildPickConfirmed = BOOLEAN(
            (PulsedInputState
                .menu[C2Rust_Unnamed::KEY_MENU_CANCEL.0 as ::core::ffi::c_int as usize]
                == 0) as ::core::ffi::c_int as ::core::ffi::c_uint,
        );
        return BOOLEAN::FALSE;
    }
    if PulsedInputState
        .menu[C2Rust_Unnamed::KEY_MENU_SPECIAL.0 as ::core::ffi::c_int as usize] != 0
        && (*pMS).currentShip.0 != MeleeShip::MELEE_NONE.0
    {
        DoShipSpin(
            (*pMS).currentShip.0 as COUNT,
            ::core::ptr::null_mut::<*mut TFB_SoundSample>(),
        );
        return BOOLEAN::TRUE;
    }
    let mut newSelectedShip: MeleeShip = MeleeShip::MELEE_ANDROSYNTH;
    newSelectedShip = (*pMS).currentShip;
    if PulsedInputState
        .menu[C2Rust_Unnamed::KEY_MENU_LEFT.0 as ::core::ffi::c_int as usize] != 0
    {
        if newSelectedShip.0.wrapping_rem(5 as ::core::ffi::c_uint)
            == 0 as ::core::ffi::c_uint
        {
            newSelectedShip = MeleeShip(
                newSelectedShip.0.wrapping_add(5 as ::core::ffi::c_uint),
            );
        }
        newSelectedShip = MeleeShip(newSelectedShip.0.wrapping_sub(1));
    } else if PulsedInputState
        .menu[C2Rust_Unnamed::KEY_MENU_RIGHT.0 as ::core::ffi::c_int as usize] != 0
    {
        newSelectedShip = MeleeShip(newSelectedShip.0.wrapping_add(1));
        if newSelectedShip.0.wrapping_rem(5 as ::core::ffi::c_uint)
            == 0 as ::core::ffi::c_uint
        {
            newSelectedShip = MeleeShip(
                newSelectedShip.0.wrapping_sub(5 as ::core::ffi::c_uint),
            );
        }
    }
    if PulsedInputState
        .menu[C2Rust_Unnamed::KEY_MENU_UP.0 as ::core::ffi::c_int as usize] != 0
    {
        if newSelectedShip.0 >= 5 as ::core::ffi::c_uint {
            newSelectedShip = MeleeShip(
                newSelectedShip.0.wrapping_sub(5 as ::core::ffi::c_uint),
            );
        } else {
            newSelectedShip = MeleeShip(
                newSelectedShip
                    .0
                    .wrapping_add(
                        (5 as ::core::ffi::c_int
                            * (5 as ::core::ffi::c_int - 1 as ::core::ffi::c_int))
                            as ::core::ffi::c_uint,
                    ),
            );
        }
    } else if PulsedInputState
        .menu[C2Rust_Unnamed::KEY_MENU_DOWN.0 as ::core::ffi::c_int as usize] != 0
    {
        if newSelectedShip.0
            < (5 as ::core::ffi::c_int
                * (5 as ::core::ffi::c_int - 1 as ::core::ffi::c_int))
                as ::core::ffi::c_uint
        {
            newSelectedShip = MeleeShip(
                newSelectedShip.0.wrapping_add(5 as ::core::ffi::c_uint),
            );
        } else {
            newSelectedShip = MeleeShip(
                newSelectedShip
                    .0
                    .wrapping_sub(
                        (5 as ::core::ffi::c_int
                            * (5 as ::core::ffi::c_int - 1 as ::core::ffi::c_int))
                            as ::core::ffi::c_uint,
                    ),
            );
        }
    }
    if newSelectedShip.0 != (*pMS).currentShip.0 {
        DrawPickIcon((*pMS).currentShip, true);
        (*pMS).currentShip = newSelectedShip;
        DrawMeleeShipStrings(pMS, newSelectedShip);
    }
    Melee_flashSelection(pMS);
    SleepThreadUntil(
        TimeIn
            .wrapping_add(
                (840 as ::core::ffi::c_int / 30 as ::core::ffi::c_int) as TimeCount,
            ),
    );
    return BOOLEAN::TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn BuildPickShip(mut pMS: *mut MELEE_STATE) -> bool {
    FlushInput();
    if (*pMS).currentShip.0 == MeleeShip::MELEE_NONE.0 {
        (*pMS).currentShip = MeleeShip(0 as ::core::ffi::c_int as ::core::ffi::c_uint);
    }
    DrawPickFrame(pMS);
    (*pMS).InputFunc = Some(
        DoPickShip as unsafe extern "C" fn(*mut MELEE_STATE) -> BOOLEAN,
    ) as Option<unsafe extern "C" fn(*mut melee_state) -> BOOLEAN>;
    DoInput(pMS as *mut ::core::ffi::c_void, BOOLEAN::FALSE);
    return (*pMS).buildPickConfirmed.0 != 0;
}
