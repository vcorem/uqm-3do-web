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

// Rust translation of src/uqm/planets/roster.c.
extern "C" {
    pub type string_table_entry;
    pub type context_desc;
    pub type frame_desc;
    pub type tfb_soundsample;
    pub type FlashContext;
    fn SetContext(Context: CONTEXT) -> CONTEXT;
    fn SetContextForeGroundColor(Color: Color) -> Color;
    fn DrawStamp(pStamp: *mut STAMP);
    fn DrawFilledStamp(pStamp: *mut STAMP);
    fn BatchGraphics();
    fn UnbatchGraphics();
    fn SetAbsFrameIndex(Frame: FRAME, FrameIndex: COUNT) -> FRAME;
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
    fn GetTimeCounter() -> TimeCount;
    fn qsort(
        _: *mut ::core::ffi::c_void,
        _: size_t,
        _: size_t,
        _: Option<
            unsafe extern "C" fn(
                *const ::core::ffi::c_void,
                *const ::core::ffi::c_void,
            ) -> ::core::ffi::c_int,
        >,
    );
    fn memcpy(
        _: *mut ::core::ffi::c_void,
        _: *const ::core::ffi::c_void,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        _: *mut ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    static mut PulsedInputState: CONTROLLER_INPUT_STATE;
    fn DoInput(pInputState: *mut ::core::ffi::c_void, resetInput: BOOLEAN);
    static mut optWhichMenu: ::core::ffi::c_int;
    static mut resolutionFactor: ::core::ffi::c_uint;
    fn CountLinks(pq: *mut QUEUE) -> COUNT;
    fn SetFlashRect(pRect: *const RECT, purple: BOOLEAN);
    fn PreUpdateFlashRect();
    fn PostUpdateFlashRect();
    fn GetStatusMessageRect(r: *mut RECT);
    fn DrawStatusMessage(pStr: *const UNICODE);
    fn DeltaSISGauges(
        crew_delta: SIZE,
        fuel_delta: SDWORD,
        resunit_delta: ::core::ffi::c_int,
    );
    fn GetCrewPodCapacity() -> COUNT;
    static mut GlobData: GLOBDATA;
    fn GetStarShipFromIndex(pShipQ: *mut QUEUE, Index: COUNT) -> HLINK;
    fn SaveContextFrame(saveRect: *const RECT) -> STAMP;
    static mut StatusContext: CONTEXT;
    fn SetMenuSounds(sound_0: MENU_SOUND_FLAGS, sound_1: MENU_SOUND_FLAGS);
    fn PlayMenuSound(S: MENU_SOUND_EFFECT);
    fn DrawMenuStateStrings(beg_index: BYTE, NewState: SWORD);
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
pub type size_t = usize;
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
pub type CONTEXT_DESC = context_desc;
pub type FRAME_DESC = frame_desc;
pub type CONTEXT = *mut CONTEXT_DESC;
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
    pub flags: BYTE,
}
pub type TimeCount = DWORD;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _controller_input_state {
    pub key: [[::core::ffi::c_int; 8]; 2],
    pub menu: [::core::ffi::c_int; 31],
}
pub type CONTROLLER_INPUT_STATE = _controller_input_state;
pub type QUEUE_HANDLE = *mut ::core::ffi::c_void;
pub type HLINK = QUEUE_HANDLE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct link {
    pub pred: HLINK,
    pub succ: HLINK,
}
pub type LINK = link;
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
pub type MUSIC_REF = *mut *mut TFB_SoundSample;
pub type TFB_SoundSample = tfb_soundsample;
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
pub type HSHIPFRAG = HLINK;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SHIP_FRAGMENT {
    pub pred: HLINK,
    pub succ: HLINK,
    pub SpeciesID: SPECIES_ID,
    pub captains_name_index: BYTE,
    pub race_id: BYTE,
    pub index: BYTE,
    pub crew_level: COUNT,
    pub max_crew: COUNT,
    pub energy_level: BYTE,
    pub max_energy: BYTE,
    pub race_strings: STRING,
    pub icons: FRAME,
    pub melee_icon: FRAME,
}
pub type HFLEETINFO = HLINK;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FLEET_INFO {
    pub pred: HFLEETINFO,
    pub succ: HFLEETINFO,
    pub SpeciesID: SPECIES_ID,
    pub shipIdStr: *const ::core::ffi::c_char,
    pub allied_state: UWORD,
    pub days_left: BYTE,
    pub growth_fract: BYTE,
    pub crew_level: COUNT,
    pub max_crew: COUNT,
    pub growth: BYTE,
    pub max_energy: BYTE,
    pub loc: POINT,
    pub race_strings: STRING,
    pub icons: FRAME,
    pub melee_icon: FRAME,
    pub actual_strength: COUNT,
    pub known_strength: COUNT,
    pub known_loc: POINT,
    pub growth_err_term: BYTE,
    pub func_index: BYTE,
    pub dest_loc: POINT,
    pub can_build: BOOLEAN,
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
pub struct GLOBDATA {
    pub SIS_state: SIS_STATE,
    pub Game_state: GAME_STATE,
}
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
    pub flash_rect2: RECT,
    pub flash_frame0: FRAME,
    pub flash_frame1: FRAME,
    pub flashContext: *mut FlashContext,
    pub hMusic: MUSIC_REF,
    pub privData: *mut ::core::ffi::c_void,
}
pub type MENU_STATE = menu_state;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_1(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_1 {
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
    pub const PM_READ_VERY_SLOW: Self = Self(36);
    pub const PM_READ_SLOW: Self = Self(37);
    pub const PM_READ_MODERATE: Self = Self(38);
    pub const PM_READ_FAST: Self = Self(39);
    pub const PM_READ_VERY_FAST: Self = Self(40);
    pub const PM_CHANGE_CAPTAIN: Self = Self(41);
    pub const PM_CHANGE_SHIP: Self = Self(42);
    pub const PM_EXIT_SETTINGS: Self = Self(43);
    pub const PM_ALT_SCAN: Self = Self(44);
    pub const PM_ALT_STARMAP: Self = Self(45);
    pub const PM_ALT_MANIFEST: Self = Self(46);
    pub const PM_ALT_GAME_MENU: Self = Self(47);
    pub const PM_ALT_NAVIGATE: Self = Self(48);
    pub const PM_ALT_CARGO: Self = Self(49);
    pub const PM_ALT_DEVICES: Self = Self(50);
    pub const PM_ALT_ROSTER: Self = Self(51);
    pub const PM_ALT_EXIT_MANIFEST: Self = Self(52);
    pub const PM_ALT_MSCAN: Self = Self(53);
    pub const PM_ALT_ESCAN: Self = Self(54);
    pub const PM_ALT_BSCAN: Self = Self(55);
    pub const PM_ALT_ASCAN: Self = Self(56);
    pub const PM_ALT_DISPATCH: Self = Self(57);
    pub const PM_ALT_EXIT_SCAN: Self = Self(58);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ROSTER_STATE {
    pub shipPos: [POINT; 12],
    pub count: COUNT,
    pub curShipPt: POINT,
    pub curShipFrame: FRAME,
    pub modifyingCrew: bool,
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
pub struct MENU_SOUND_EFFECT(pub ::core::ffi::c_uint);
impl MENU_SOUND_EFFECT {
    pub const MENU_SOUND_MOVE: Self = Self(0);
    pub const MENU_SOUND_SUCCESS: Self = Self(1);
    pub const MENU_SOUND_FAILURE: Self = Self(2);
    pub const MENU_SOUND_INVOKED: Self = Self(3);
    pub const MENU_SOUND_BUBBLEWARP: Self = Self(4);
    pub const MENU_SOUND_TEXT: Self = Self(5);
}
pub type MENU_SOUND_FLAGS = UWORD;
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
#[inline]
unsafe extern "C" fn pointsEqual(mut p1: POINT, mut p2: POINT) -> bool {
    return p1.x as ::core::ffi::c_int == p2.x as ::core::ffi::c_int
        && p1.y as ::core::ffi::c_int == p2.y as ::core::ffi::c_int;
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
unsafe extern "C" fn LockShipFrag(
    mut pq: *const QUEUE,
    mut h: HSHIPFRAG,
) -> *mut SHIP_FRAGMENT {
    ((*pq).object_size as usize == ::core::mem::size_of::<SHIP_FRAGMENT>()
        || {
            __assert_fail(
                b"GetLinkSize (pq) == sizeof (SHIP_FRAGMENT)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/uqm/planets/../races.h\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                340 as ::core::ffi::c_int,
                b"LockShipFrag\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    return LockLink(pq, h) as *mut SHIP_FRAGMENT;
}
#[inline]
unsafe extern "C" fn LockFleetInfo(
    mut pq: *const QUEUE,
    mut h: HFLEETINFO,
) -> *mut FLEET_INFO {
    ((*pq).object_size as usize == ::core::mem::size_of::<FLEET_INFO>()
        || {
            __assert_fail(
                b"GetLinkSize (pq) == sizeof (FLEET_INFO)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/uqm/planets/../races.h\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                410 as ::core::ffi::c_int,
                b"LockFleetInfo\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    return LockLink(pq, h) as *mut FLEET_INFO;
}
static mut savedShipFrame_r: RECT = RECT {
    corner: POINT { x: 0, y: 0 },
    extent: EXTENT { width: 0, height: 0 },
};
static mut savedShipFrame: STAMP = STAMP {
    origin: POINT { x: 0, y: 0 },
    frame: ::core::ptr::null_mut::<FRAME_DESC>(),
};
static mut ship_pos: [POINT; 12] = [
    point {
        x: (3 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as COORD,
        y: (30 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as COORD,
    },
    point {
        x: (3 as ::core::ffi::c_int + 42 as ::core::ffi::c_int) as COORD,
        y: (30 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as COORD,
    },
    point {
        x: (3 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as COORD,
        y: (30 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as COORD,
    },
    point {
        x: (3 as ::core::ffi::c_int + 42 as ::core::ffi::c_int) as COORD,
        y: (30 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as COORD,
    },
    point {
        x: (3 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as COORD,
        y: (30 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as COORD,
    },
    point {
        x: (3 as ::core::ffi::c_int + 42 as ::core::ffi::c_int) as COORD,
        y: (30 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as COORD,
    },
    point {
        x: (3 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as COORD,
        y: (30 as ::core::ffi::c_int
            + 4 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as COORD,
    },
    point {
        x: (3 as ::core::ffi::c_int + 42 as ::core::ffi::c_int) as COORD,
        y: (30 as ::core::ffi::c_int
            + 4 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as COORD,
    },
    point {
        x: (3 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as COORD,
        y: (30 as ::core::ffi::c_int
            + 0 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as COORD,
    },
    point {
        x: (3 as ::core::ffi::c_int + 42 as ::core::ffi::c_int) as COORD,
        y: (30 as ::core::ffi::c_int
            + 0 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as COORD,
    },
    point {
        x: (3 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as COORD,
        y: (30 as ::core::ffi::c_int
            + 5 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as COORD,
    },
    point {
        x: (3 as ::core::ffi::c_int + 42 as ::core::ffi::c_int) as COORD,
        y: (30 as ::core::ffi::c_int
            + 5 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as COORD,
    },
];
unsafe extern "C" fn drawSupportShip(
    mut rosterState: *mut ROSTER_STATE,
    mut filled: bool,
    mut saveFrame: bool,
) {
    let mut s: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    if (*rosterState).curShipFrame.is_null() {
        return;
    }
    s.origin.x = (((*rosterState).curShipPt.x as ::core::ffi::c_int) << resolutionFactor)
        as COORD;
    s.origin.y = (((*rosterState).curShipPt.y as ::core::ffi::c_int) << resolutionFactor)
        as COORD;
    s.frame = (*rosterState).curShipFrame;
    if saveFrame {
        savedShipFrame_r.corner.x = s.origin.x;
        savedShipFrame_r.corner.y = s.origin.y;
        savedShipFrame_r.extent.width = ((16 as ::core::ffi::c_int) << resolutionFactor)
            as COORD;
        savedShipFrame_r.extent.height = ((16 as ::core::ffi::c_int) << resolutionFactor)
            as COORD;
        savedShipFrame = SaveContextFrame(&raw mut savedShipFrame_r);
        log_add(
            log_Level::log_Debug,
            b"Saved x:%u, y:%u\0".as_ptr() as *const ::core::ffi::c_char,
            savedShipFrame_r.corner.x as ::core::ffi::c_int,
            savedShipFrame_r.corner.y as ::core::ffi::c_int,
        );
    } else if filled {
        uqm_lua_checked!(DrawFilledStamp(&raw mut s), ());
    } else {
        uqm_lua_checked!(DrawStamp(&raw mut s), ());
    };
}
unsafe extern "C" fn getSupportShipIcon(mut rosterState: *mut ROSTER_STATE) {
    let mut hShipFrag: HSHIPFRAG = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut ShipFragPtr: *mut SHIP_FRAGMENT = ::core::ptr::null_mut::<SHIP_FRAGMENT>();
    (*rosterState).curShipFrame = ::core::ptr::null_mut::<FRAME_DESC>();
    ShipFragPtr = LockSupportShip(rosterState, &raw mut hShipFrag);
    if ShipFragPtr.is_null() {
        return;
    }
    (*rosterState).curShipFrame = SetAbsFrameIndex((*ShipFragPtr).icons, 2 as COUNT);
    UnlockLink(&raw mut GlobData.Game_state.built_ship_q, hShipFrag);
}
unsafe extern "C" fn flashSupportShip(
    mut rosterState: *mut ROSTER_STATE,
    mut saveFrame: bool,
) {
    static mut c: Color = Color {
        r: ((0x1f as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
            | 0x1f as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
        g: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
            | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
        b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
            | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
        a: 0xff as BYTE,
    };
    static mut NextTime: TimeCount = 0 as TimeCount;
    uqm_lua_checked!(drawSupportShip(rosterState, BOOLEAN::TRUE.0 as ::core::ffi::c_int != 0, saveFrame), ());
    if GetTimeCounter() >= NextTime {
        NextTime = GetTimeCounter()
            .wrapping_add(
                (840 as ::core::ffi::c_int / 15 as ::core::ffi::c_int) as TimeCount,
            );
        if c.g as ::core::ffi::c_int
            >= (0x19 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x19 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int
        {
            c = colorFromRgb15(0x1f as BYTE, 0 as BYTE, 0 as BYTE);
        } else {
            c.g = (c.g as ::core::ffi::c_int
                + ((0x2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)) as BYTE;
            c.b = (c.b as ::core::ffi::c_int
                + ((0x2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)) as BYTE;
        }
        SetContextForeGroundColor(c);
        uqm_lua_checked!(drawSupportShip(
            rosterState,
            BOOLEAN::TRUE.0 as ::core::ffi::c_int != 0,
            BOOLEAN::FALSE.0 as ::core::ffi::c_int != 0,
        ), ());
    }
}
unsafe extern "C" fn LockSupportShip(
    mut rosterState: *mut ROSTER_STATE,
    mut phFrag: *mut HSHIPFRAG,
) -> *mut SHIP_FRAGMENT {
    let mut pship_pos: *const POINT = ::core::ptr::null::<POINT>();
    let mut hStarShip: HSHIPFRAG = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut hNextShip: HSHIPFRAG = ::core::ptr::null_mut::<::core::ffi::c_void>();
    hStarShip = GlobData.Game_state.built_ship_q.head as HSHIPFRAG;
    pship_pos = &raw const ship_pos as *const POINT;
    while !hStarShip.is_null() {
        let mut StarShipPtr: *mut SHIP_FRAGMENT = ::core::ptr::null_mut::<
            SHIP_FRAGMENT,
        >();
        StarShipPtr = LockShipFrag(&raw mut GlobData.Game_state.built_ship_q, hStarShip);
        if pointsEqual(*pship_pos, (*rosterState).curShipPt) {
            *phFrag = hStarShip;
            return StarShipPtr;
        }
        hNextShip = (*StarShipPtr).succ as HSHIPFRAG;
        UnlockLink(&raw mut GlobData.Game_state.built_ship_q, hStarShip);
        hStarShip = hNextShip;
        pship_pos = pship_pos.offset(1);
    }
    return ::core::ptr::null_mut::<SHIP_FRAGMENT>();
}
unsafe extern "C" fn flashSupportShipCrew() {
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    SetContext(StatusContext);
    GetStatusMessageRect(&raw mut r);
    uqm_lua_checked!(SetFlashRect(&raw mut r, BOOLEAN::FALSE), ());
}
unsafe extern "C" fn DeltaSupportCrew(
    mut rosterState: *mut ROSTER_STATE,
    mut crew_delta: SIZE,
) -> BOOLEAN {
    let mut ret: BOOLEAN = BOOLEAN::FALSE;
    let mut buf: [UNICODE; 40] = [0; 40];
    let mut hTemplate: HFLEETINFO = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut hShipFrag: HSHIPFRAG = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut StarShipPtr: *mut SHIP_FRAGMENT = ::core::ptr::null_mut::<SHIP_FRAGMENT>();
    let mut TemplatePtr: *mut FLEET_INFO = ::core::ptr::null_mut::<FLEET_INFO>();
    StarShipPtr = LockSupportShip(rosterState, &raw mut hShipFrag);
    if StarShipPtr.is_null() {
        return BOOLEAN::FALSE;
    }
    hTemplate = GetStarShipFromIndex(
        &raw mut GlobData.Game_state.avail_race_q,
        (*StarShipPtr).race_id as COUNT,
    ) as HFLEETINFO;
    TemplatePtr = LockFleetInfo(&raw mut GlobData.Game_state.avail_race_q, hTemplate);
    (*StarShipPtr).crew_level = ((*StarShipPtr).crew_level as ::core::ffi::c_int
        + crew_delta as ::core::ffi::c_int) as COUNT;
    if (*StarShipPtr).crew_level as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        (*StarShipPtr).crew_level = 1 as COUNT;
    } else if (*StarShipPtr).crew_level as ::core::ffi::c_int
        > (*TemplatePtr).crew_level as ::core::ffi::c_int
        && crew_delta as ::core::ffi::c_int > 0 as ::core::ffi::c_int
    {
        (*StarShipPtr).crew_level = ((*StarShipPtr).crew_level as ::core::ffi::c_int
            - crew_delta as ::core::ffi::c_int) as COUNT;
    } else {
        if (*StarShipPtr).crew_level as ::core::ffi::c_int
            >= (*TemplatePtr).crew_level as ::core::ffi::c_int
        {
            sprintf(
                &raw mut buf as *mut ::core::ffi::c_char,
                b"%u\0".as_ptr() as *const ::core::ffi::c_char,
                (*StarShipPtr).crew_level as ::core::ffi::c_int,
            );
        } else {
            sprintf(
                &raw mut buf as *mut ::core::ffi::c_char,
                b"%u/%u\0".as_ptr() as *const ::core::ffi::c_char,
                (*StarShipPtr).crew_level as ::core::ffi::c_int,
                (*TemplatePtr).crew_level as ::core::ffi::c_int,
            );
        }
        uqm_lua_checked!(PreUpdateFlashRect(), BOOLEAN(0));
        uqm_lua_checked!(DrawStatusMessage(&raw mut buf as *mut UNICODE), BOOLEAN(0));
        uqm_lua_checked!(PostUpdateFlashRect(), BOOLEAN(0));
        uqm_lua_checked!(DeltaSISGauges(
            -(crew_delta as ::core::ffi::c_int) as SIZE,
            0 as SDWORD,
            0 as ::core::ffi::c_int,
        ), BOOLEAN(0));
        if crew_delta != 0 {
            uqm_lua_checked!(flashSupportShipCrew(), BOOLEAN(0));
        }
        ret = BOOLEAN::TRUE;
    }
    UnlockLink(&raw mut GlobData.Game_state.avail_race_q, hTemplate);
    UnlockLink(&raw mut GlobData.Game_state.built_ship_q, hShipFrag);
    return ret;
}
unsafe extern "C" fn drawModifiedSupportShip(mut rosterState: *mut ROSTER_STATE) {
    SetContext(StatusContext);
    SetContextForeGroundColor(colorFromRgb15(0x1f as BYTE, 0x1f as BYTE, 0x1f as BYTE));
    uqm_lua_checked!(drawSupportShip(
        rosterState,
        BOOLEAN::TRUE.0 as ::core::ffi::c_int != 0,
        BOOLEAN::FALSE.0 as ::core::ffi::c_int != 0,
    ), ());
}
unsafe extern "C" fn selectSupportShip(
    mut rosterState: *mut ROSTER_STATE,
    mut shipIndex: COUNT,
) {
    (*rosterState).curShipPt = (*rosterState).shipPos[shipIndex as usize];
    getSupportShipIcon(rosterState);
    uqm_lua_checked!(DeltaSupportCrew(rosterState, 0 as SIZE), ());
}
unsafe extern "C" fn DoModifyRoster(mut pMS: *mut MENU_STATE) -> BOOLEAN {
    let mut rosterState: *mut ROSTER_STATE = (*pMS).privData as *mut ROSTER_STATE;
    let mut select: BOOLEAN = BOOLEAN::FALSE;
    let mut cancel: BOOLEAN = BOOLEAN::FALSE;
    let mut up: BOOLEAN = BOOLEAN::FALSE;
    let mut down: BOOLEAN = BOOLEAN::FALSE;
    let mut pgup: BOOLEAN = BOOLEAN::FALSE;
    let mut pgdn: BOOLEAN = BOOLEAN::FALSE;
    let mut horiz: BOOLEAN = BOOLEAN::FALSE;
    if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
        & (C2Rust_Unnamed_0::CHECK_ABORT.0 as ::core::ffi::c_int
            | C2Rust_Unnamed_0::CHECK_LOAD.0 as ::core::ffi::c_int) != 0
    {
        return BOOLEAN::FALSE;
    }
    select = BOOLEAN(
        PulsedInputState
            .menu[C2Rust_Unnamed::KEY_MENU_SELECT.0 as ::core::ffi::c_int as usize]
            as ::core::ffi::c_uint,
    );
    cancel = BOOLEAN(
        PulsedInputState
            .menu[C2Rust_Unnamed::KEY_MENU_CANCEL.0 as ::core::ffi::c_int as usize]
            as ::core::ffi::c_uint,
    );
    up = BOOLEAN(
        PulsedInputState
            .menu[C2Rust_Unnamed::KEY_MENU_UP.0 as ::core::ffi::c_int as usize]
            as ::core::ffi::c_uint,
    );
    down = BOOLEAN(
        PulsedInputState
            .menu[C2Rust_Unnamed::KEY_MENU_DOWN.0 as ::core::ffi::c_int as usize]
            as ::core::ffi::c_uint,
    );
    horiz = BOOLEAN(
        (PulsedInputState
            .menu[C2Rust_Unnamed::KEY_MENU_LEFT.0 as ::core::ffi::c_int as usize] != 0
            || PulsedInputState
                .menu[C2Rust_Unnamed::KEY_MENU_RIGHT.0 as ::core::ffi::c_int as usize]
                != 0) as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
    pgup = BOOLEAN(
        PulsedInputState
            .menu[C2Rust_Unnamed::KEY_MENU_ZOOM_IN.0 as ::core::ffi::c_int as usize]
            as ::core::ffi::c_uint,
    );
    pgdn = BOOLEAN(
        PulsedInputState
            .menu[C2Rust_Unnamed::KEY_MENU_ZOOM_OUT.0 as ::core::ffi::c_int as usize]
            as ::core::ffi::c_uint,
    );
    if cancel.0 != 0 && !(*rosterState).modifyingCrew {
        return BOOLEAN::FALSE
    } else if select.0 != 0 || cancel.0 != 0 {
        (*rosterState).modifyingCrew = (*rosterState).modifyingCrew as ::core::ffi::c_int
            ^ 1 as ::core::ffi::c_int != 0;
        if !(*rosterState).modifyingCrew {
            uqm_lua_checked!(SetFlashRect(::core::ptr::null::<RECT>(), BOOLEAN::FALSE), BOOLEAN(0));
            SetMenuSounds(
                (((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int)
                    as MENU_SOUND_FLAGS as ::core::ffi::c_int
                    | ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
                        as MENU_SOUND_FLAGS as ::core::ffi::c_int
                    | ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                        as MENU_SOUND_FLAGS as ::core::ffi::c_int
                    | ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                        as MENU_SOUND_FLAGS as ::core::ffi::c_int
                    | (((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int)
                        as MENU_SOUND_FLAGS as ::core::ffi::c_int
                        | ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
                            as MENU_SOUND_FLAGS as ::core::ffi::c_int))
                    as MENU_SOUND_FLAGS,
                ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                    as MENU_SOUND_FLAGS,
            );
        } else {
            uqm_lua_checked!(drawModifiedSupportShip(rosterState), BOOLEAN(0));
            uqm_lua_checked!(flashSupportShipCrew(), BOOLEAN(0));
            SetMenuSounds(
                (((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int)
                    as MENU_SOUND_FLAGS as ::core::ffi::c_int
                    | ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
                        as MENU_SOUND_FLAGS as ::core::ffi::c_int
                    | (((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int)
                        as MENU_SOUND_FLAGS as ::core::ffi::c_int
                        | ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
                            as MENU_SOUND_FLAGS as ::core::ffi::c_int))
                    as MENU_SOUND_FLAGS,
                (((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                    as MENU_SOUND_FLAGS as ::core::ffi::c_int
                    | ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int)
                        as MENU_SOUND_FLAGS as ::core::ffi::c_int) as MENU_SOUND_FLAGS,
            );
        }
    } else if (*rosterState).modifyingCrew {
        let mut r#loop: COUNT = 0;
        let mut DoLoop: COUNT = 0 as COUNT;
        let mut failed: BOOLEAN = BOOLEAN::FALSE;
        if up.0 != 0 || pgup.0 != 0 {
            if GlobData.SIS_state.CrewEnlisted != 0 {
                DoLoop = (if pgup.0 != 0 {
                    10 as ::core::ffi::c_int
                } else {
                    1 as ::core::ffi::c_int
                }) as COUNT;
            } else {
                failed = BOOLEAN::TRUE;
            }
        } else if down.0 != 0 || pgdn.0 != 0 {
            if (GlobData.SIS_state.CrewEnlisted as ::core::ffi::c_int)
                < GetCrewPodCapacity() as ::core::ffi::c_int
            {
                DoLoop = (if pgdn.0 != 0 {
                    10 as ::core::ffi::c_int
                } else {
                    1 as ::core::ffi::c_int
                }) as COUNT;
            } else {
                failed = BOOLEAN::TRUE;
            }
        }
        if DoLoop as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            r#loop = 0 as COUNT;
            while (r#loop as ::core::ffi::c_int) < DoLoop as ::core::ffi::c_int {
                failed = BOOLEAN(
                    (uqm_lua_checked!(DeltaSupportCrew(
                            rosterState,
                            (if down.0 != 0 || pgdn.0 != 0 {
                                -1 as ::core::ffi::c_int
                            } else {
                                1 as ::core::ffi::c_int
                            }) as SIZE,
                        ), BOOLEAN(0))
                        .0 == 0) as ::core::ffi::c_int as ::core::ffi::c_uint,
                );
                r#loop = r#loop.wrapping_add(1);
            }
        }
        if failed.0 != 0 {
            uqm_lua_checked!(PlayMenuSound(MENU_SOUND_EFFECT::MENU_SOUND_FAILURE), BOOLEAN(0));
        }
    } else {
        let mut NewState: COUNT = 0;
        let mut pship_pos: *mut POINT = &raw mut (*rosterState).shipPos as *mut POINT;
        let mut top_right: COUNT = ((*rosterState).count as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as COUNT;
        NewState = (*pMS).CurState as COUNT;
        if ((*rosterState).count as ::core::ffi::c_int) >= 2 as ::core::ffi::c_int {
            if horiz.0 != 0 {
                if NewState as ::core::ffi::c_int
                    == top_right as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                {
                    NewState = ((*rosterState).count as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int) as COUNT;
                } else if NewState as ::core::ffi::c_int
                    >= top_right as ::core::ffi::c_int
                {
                    NewState = (NewState as ::core::ffi::c_int
                        - top_right as ::core::ffi::c_int) as COUNT;
                    if ((*pship_pos.offset(NewState as isize)).y as ::core::ffi::c_int)
                        < (*pship_pos.offset((*pMS).CurState as isize)).y
                            as ::core::ffi::c_int
                    {
                        NewState = NewState.wrapping_add(1);
                    }
                } else {
                    NewState = (NewState as ::core::ffi::c_int
                        + top_right as ::core::ffi::c_int) as COUNT;
                    if NewState as ::core::ffi::c_int != top_right as ::core::ffi::c_int
                        && (*pship_pos.offset(NewState as isize)).y as ::core::ffi::c_int
                            > (*pship_pos.offset((*pMS).CurState as isize)).y
                                as ::core::ffi::c_int
                    {
                        NewState = NewState.wrapping_sub(1);
                    }
                }
            } else if down.0 != 0 {
                NewState = NewState.wrapping_add(1);
                if NewState as ::core::ffi::c_int
                    == (*rosterState).count as ::core::ffi::c_int
                {
                    NewState = top_right;
                } else if NewState as ::core::ffi::c_int
                    == top_right as ::core::ffi::c_int
                {
                    NewState = 0 as COUNT;
                }
            } else if up.0 != 0 {
                if NewState as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    NewState = (top_right as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int) as COUNT;
                } else if NewState as ::core::ffi::c_int
                    == top_right as ::core::ffi::c_int
                {
                    NewState = ((*rosterState).count as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int) as COUNT;
                } else {
                    NewState = NewState.wrapping_sub(1);
                }
            }
        }
        BatchGraphics();
        SetContext(StatusContext);
        if NewState as ::core::ffi::c_int != (*pMS).CurState as ::core::ffi::c_int {
            if if resolutionFactor != 2 as ::core::ffi::c_uint {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } != 0
            {
                uqm_lua_checked!(DrawStamp(&raw mut savedShipFrame), BOOLEAN(0));
            } else {
                uqm_lua_checked!(drawSupportShip(
                    rosterState,
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int != 0,
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int != 0,
                ), BOOLEAN(0));
            }
            uqm_lua_checked!(selectSupportShip(rosterState, NewState), BOOLEAN(0));
            (*pMS).CurState = NewState as BYTE;
            if if resolutionFactor != 2 as ::core::ffi::c_uint {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } != 0
            {
                uqm_lua_checked!(flashSupportShip(
                    rosterState,
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int != 0,
                ), BOOLEAN(0));
            } else {
                uqm_lua_checked!(flashSupportShip(
                    rosterState,
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int != 0,
                ), BOOLEAN(0));
            }
        } else {
            uqm_lua_checked!(flashSupportShip(rosterState, BOOLEAN::FALSE.0 as ::core::ffi::c_int != 0), BOOLEAN(0));
        }
        UnbatchGraphics();
    }
    return BOOLEAN::TRUE;
}
unsafe extern "C" fn compShipPos(
    mut ptr1: *const ::core::ffi::c_void,
    mut ptr2: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut pt1: *const POINT = ptr1 as *const POINT;
    let mut pt2: *const POINT = ptr2 as *const POINT;
    if ((*pt1).x as ::core::ffi::c_int) < (*pt2).x as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int
    } else if (*pt1).x as ::core::ffi::c_int > (*pt2).x as ::core::ffi::c_int {
        return 1 as ::core::ffi::c_int
    }
    if ((*pt1).y as ::core::ffi::c_int) < (*pt2).y as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int
    } else if (*pt1).y as ::core::ffi::c_int > (*pt2).y as ::core::ffi::c_int {
        return 1 as ::core::ffi::c_int
    } else {
        return 0 as ::core::ffi::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn RosterMenu() -> BOOLEAN {
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
        flash_rect2: RECT {
            corner: POINT { x: 0, y: 0 },
            extent: EXTENT { width: 0, height: 0 },
        },
        flash_frame0: ::core::ptr::null_mut::<FRAME_DESC>(),
        flash_frame1: ::core::ptr::null_mut::<FRAME_DESC>(),
        flashContext: ::core::ptr::null_mut::<FlashContext>(),
        hMusic: ::core::ptr::null_mut::<*mut TFB_SoundSample>(),
        privData: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    let mut RosterState: ROSTER_STATE = ROSTER_STATE {
        shipPos: [POINT { x: 0, y: 0 }; 12],
        count: 0,
        curShipPt: POINT { x: 0, y: 0 },
        curShipFrame: ::core::ptr::null_mut::<FRAME_DESC>(),
        modifyingCrew: false,
    };
    memset(
        &raw mut MenuState as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<MENU_STATE>(),
    );
    MenuState.privData = &raw mut RosterState as *mut ::core::ffi::c_void;
    memset(
        &raw mut RosterState as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<ROSTER_STATE>(),
    );
    RosterState.count = CountLinks(&raw mut GlobData.Game_state.built_ship_q);
    if RosterState.count == 0 {
        return BOOLEAN::FALSE;
    }
    (::core::mem::size_of::<[POINT; 12]>() == ::core::mem::size_of::<[POINT; 12]>()
        || {
            __assert_fail(
                b"sizeof (RosterState.shipPos) == sizeof (ship_pos)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/uqm/planets/roster.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                441 as ::core::ffi::c_int,
                b"RosterMenu\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    memcpy(
        &raw mut RosterState.shipPos as *mut POINT as *mut ::core::ffi::c_void,
        &raw const ship_pos as *const POINT as *const ::core::ffi::c_void,
        ::core::mem::size_of::<[POINT; 12]>(),
    );
    qsort(
        &raw mut RosterState.shipPos as *mut POINT as *mut ::core::ffi::c_void,
        RosterState.count as size_t,
        ::core::mem::size_of::<POINT>(),
        Some(
            compShipPos
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_void,
                    *const ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
    );
    SetContext(StatusContext);
    uqm_lua_checked!(selectSupportShip(&raw mut RosterState, MenuState.CurState as COUNT), BOOLEAN(0));
    if if resolutionFactor != 2 as ::core::ffi::c_uint {
        BOOLEAN::FALSE.0 as ::core::ffi::c_int
    } else {
        BOOLEAN::TRUE.0 as ::core::ffi::c_int
    } != 0
    {
        uqm_lua_checked!(drawSupportShip(
            &raw mut RosterState,
            BOOLEAN::TRUE.0 as ::core::ffi::c_int != 0,
            BOOLEAN::TRUE.0 as ::core::ffi::c_int != 0,
        ), BOOLEAN(0));
    }
    if optWhichMenu == 0x2 as ::core::ffi::c_int {
        uqm_lua_checked!(DrawMenuStateStrings(
            C2Rust_Unnamed_1::PM_ALT_CARGO.0 as ::core::ffi::c_int as BYTE,
            2 as SWORD,
        ), BOOLEAN(0));
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
    MenuState.InputFunc = Some(
        DoModifyRoster as unsafe extern "C" fn(*mut MENU_STATE) -> BOOLEAN,
    ) as Option<unsafe extern "C" fn(*mut menu_state) -> BOOLEAN>;
    uqm_lua_checked!(DoInput(&raw mut MenuState as *mut ::core::ffi::c_void, BOOLEAN::TRUE), BOOLEAN(0));
    SetContext(StatusContext);
    if if resolutionFactor != 2 as ::core::ffi::c_uint {
        BOOLEAN::FALSE.0 as ::core::ffi::c_int
    } else {
        BOOLEAN::TRUE.0 as ::core::ffi::c_int
    } != 0
    {
        uqm_lua_checked!(DrawStamp(&raw mut savedShipFrame), BOOLEAN(0));
    } else {
        uqm_lua_checked!(drawSupportShip(
            &raw mut RosterState,
            BOOLEAN::FALSE.0 as ::core::ffi::c_int != 0,
            BOOLEAN::FALSE.0 as ::core::ffi::c_int != 0,
        ), BOOLEAN(0));
    }
    uqm_lua_checked!(DrawStatusMessage(::core::ptr::null::<UNICODE>()), BOOLEAN(0));
    return BOOLEAN::TRUE;
}
