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

// Rust translation of src/uqm/pickship.c.
extern "C" {
    pub type string_table_entry;
    pub type context_desc;
    pub type frame_desc;
    pub type font_desc;
    pub type drawable_desc;
    pub type tfb_soundsample;
    pub type FlashContext;
    fn MAKE_HOT_SPOT(_: COORD, _: COORD) -> HOT_SPOT;
    fn SetContext(Context: CONTEXT) -> CONTEXT;
    fn SetContextForeGroundColor(Color: Color) -> Color;
    fn GetContextClipRect(pRect: *mut RECT) -> BOOLEAN;
    fn DrawStamp(pStamp: *mut STAMP);
    fn DrawFilledStamp(pStamp: *mut STAMP);
    fn DrawFilledRectangle(pRect: *mut RECT);
    fn font_DrawText(pText: *mut TEXT);
    fn BatchGraphics();
    fn UnbatchGraphics();
    fn DestroyDrawable(Drawable: DRAWABLE) -> BOOLEAN;
    fn GetFrameRect(Frame: FRAME, pRect: *mut RECT) -> BOOLEAN;
    fn SetFrameHot(Frame: FRAME, HotSpot: HOT_SPOT) -> HOT_SPOT;
    fn LoadGraphicInstance(res: RESOURCE) -> *mut ::core::ffi::c_void;
    fn SetContextFont(Font: FONT) -> FONT;
    fn SetAbsFrameIndex(Frame: FRAME, FrameIndex: COUNT) -> FRAME;
    fn IncFrameIndex(Frame: FRAME) -> FRAME;
    fn DecFrameIndex(Frame: FRAME) -> FRAME;
    fn CaptureDrawable(Drawable: DRAWABLE) -> FRAME;
    fn ReleaseDrawable(Frame: FRAME) -> DRAWABLE;
    fn __assert_fail(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    );
    fn sprintf(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn SetAbsStringTableIndex(String: STRING, StringTableIndex: COUNT) -> STRING;
    fn GetStringLength(String: STRING) -> COUNT;
    fn GetStringAddress(String: STRING) -> STRINGPTR;
    fn WaitForSoundEnd(Channel: COUNT);
    fn GetNextMeleeStarShip(which_player: COUNT, result: *mut HSTARSHIP) -> BOOLEAN;
    static mut PulsedInputState: CONTROLLER_INPUT_STATE;
    fn DoInput(pInputState: *mut ::core::ffi::c_void, resetInput: BOOLEAN);
    fn TFB_Random() -> DWORD;
    static mut battle_counter: [BYTE; 2];
    fn FleetIsInfinite(playerNr: COUNT) -> BOOLEAN;
    fn SleepThread(timePeriod: TimePeriod);
    static mut GlobData: GLOBDATA;
    fn inHQSpace() -> BOOLEAN;
    fn SetFlashRect(pRect: *const RECT);
    fn GetCrewPodCapacity() -> COUNT;
    static mut StatusFrame: FRAME;
    static mut SpaceContext: CONTEXT;
    static mut StarConFont: FONT;
    static mut TinyFont: FONT;
    static mut race_q: [QUEUE; 0];
    fn SetMenuSounds(sound_0: MENU_SOUND_FLAGS, sound_1: MENU_SOUND_FLAGS);
    fn PlayMenuSound(S: MENU_SOUND_EFFECT);
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
pub type RESOURCE = *const ::core::ffi::c_char;
pub type STRING_TABLE_ENTRY_DESC = string_table_entry;
pub type STRING = *mut STRING_TABLE_ENTRY_DESC;
pub type STRINGPTR = *mut ::core::ffi::c_char;
pub type CONTEXT_DESC = context_desc;
pub type FRAME_DESC = frame_desc;
pub type FONT_DESC = font_desc;
pub type DRAWABLE_DESC = drawable_desc;
pub type CONTEXT = *mut CONTEXT_DESC;
pub type FRAME = *mut FRAME_DESC;
pub type FONT = *mut FONT_DESC;
pub type DRAWABLE = *mut DRAWABLE_DESC;
pub type TIME_VALUE = UWORD;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct INTERSECT_CONTROL {
    pub last_time_val: TIME_VALUE,
    pub EndPoint: POINT,
    pub IntersectStamp: STAMP,
}
pub type HOT_SPOT = POINT;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct STARSHIP {
    pub pred: HLINK,
    pub succ: HLINK,
    pub SpeciesID: SPECIES_ID,
    pub captains_name_index: BYTE,
    pub RaceDescPtr: *mut RACE_DESC,
    pub crew_level: COUNT,
    pub max_crew: COUNT,
    pub ship_cost: BYTE,
    pub index: COUNT,
    pub race_strings: STRING,
    pub icons: FRAME,
    pub weapon_counter: BYTE,
    pub special_counter: BYTE,
    pub energy_counter: BYTE,
    pub ship_input_state: BYTE,
    pub cur_status_flags: STATUS_FLAGS,
    pub old_status_flags: STATUS_FLAGS,
    pub hShip: HELEMENT,
    pub ShipFacing: COUNT,
    pub playerNr: SIZE,
    pub control: BYTE,
}
pub type HELEMENT = HLINK;
pub type STATUS_FLAGS = UWORD;
pub type RACE_DESC = race_desc;
#[derive(Copy, Clone)]
#[repr(C, align(16))]
pub struct race_desc {
    pub ship_info: SHIP_INFO,
    pub c2rust_padding: [u8; 12],
    pub fleet: FLEET_STUFF,
    pub c2rust_padding_0: [u8; 10],
    pub characteristics: CHARACTERISTIC_STUFF,
    pub c2rust_padding_1: [u8; 2],
    pub ship_data: DATA_STUFF,
    pub cyborg_control: INTEL_STUFF,
    pub uninit_func: Option<UNINIT_FUNC>,
    pub preprocess_func: Option<PREPROCESS_FUNC>,
    pub postprocess_func: Option<POSTPROCESS_FUNC>,
    pub init_weapon_func: Option<INIT_WEAPON_FUNC>,
    pub data: *mut ::core::ffi::c_void,
    pub CodeRef: *mut ::core::ffi::c_void,
}
pub type INIT_WEAPON_FUNC = unsafe extern "C" fn(*mut ELEMENT, *mut HELEMENT) -> COUNT;
pub type ELEMENT = element;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct element {
    pub pred: HELEMENT,
    pub succ: HELEMENT,
    pub preprocess_func: Option<ElementProcessFunc>,
    pub postprocess_func: Option<ElementProcessFunc>,
    pub collision_func: Option<ElementCollisionFunc>,
    pub death_func: Option<ElementProcessFunc>,
    pub playerNr: SIZE,
    pub state_flags: ELEMENT_FLAGS,
    pub c2rust_unnamed: C2Rust_Unnamed_4,
    pub c2rust_unnamed_0: C2Rust_Unnamed_3,
    pub c2rust_unnamed_1: C2Rust_Unnamed_2,
    pub c2rust_unnamed_2: C2Rust_Unnamed_1,
    pub c2rust_unnamed_3: C2Rust_Unnamed_0,
    pub colorCycleIndex: BYTE,
    pub velocity: VELOCITY_DESC,
    pub IntersectControl: INTERSECT_CONTROL,
    pub PrimIndex: COUNT,
    pub current: STATE,
    pub next: STATE,
    pub pParent: *mut ::core::ffi::c_void,
    pub hTarget: HELEMENT,
}
pub type STATE = state;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct state {
    pub location: POINT,
    pub image: C2Rust_Unnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed {
    pub frame: FRAME,
    pub farray: *mut FRAME,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_0 {
    pub thrust_wait: BYTE,
    pub blast_offset: BYTE,
    pub next_turn: BYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_1 {
    pub turn_wait: BYTE,
    pub sys_loc: BYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_2 {
    pub mass_points: BYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_3 {
    pub crew_level: COUNT,
    pub hit_points: COUNT,
    pub facing: COUNT,
    pub cycle: COUNT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_4 {
    pub life_span: COUNT,
    pub scan_node: COUNT,
}
pub type ELEMENT_FLAGS = UWORD;
pub type ElementProcessFunc = unsafe extern "C" fn(*mut ELEMENT) -> ();
pub type ElementCollisionFunc = unsafe extern "C" fn(
    *mut ELEMENT,
    *mut POINT,
    *mut ELEMENT,
    *mut POINT,
) -> ();
pub type POSTPROCESS_FUNC = unsafe extern "C" fn(*mut ELEMENT) -> ();
pub type PREPROCESS_FUNC = unsafe extern "C" fn(*mut ELEMENT) -> ();
pub type UNINIT_FUNC = unsafe extern "C" fn(*mut RACE_DESC) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct INTEL_STUFF {
    pub ManeuverabilityIndex: COUNT,
    pub WeaponRange: COUNT,
    pub intelligence_func: Option<IntelligenceFunc>,
}
pub type IntelligenceFunc = unsafe extern "C" fn(
    *mut ELEMENT,
    *mut EVALUATE_DESC,
    COUNT,
) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EVALUATE_DESC {
    pub ObjectPtr: *mut ELEMENT,
    pub facing: COUNT,
    pub which_turn: COUNT,
    pub MoveState: MOVEMENT_STATE,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct MOVEMENT_STATE(pub ::core::ffi::c_uint);
impl MOVEMENT_STATE {
    pub const PURSUE: Self = Self(0);
    pub const AVOID: Self = Self(1);
    pub const ENTICE: Self = Self(2);
    pub const NO_MOVEMENT: Self = Self(3);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DATA_STUFF {
    pub ship_rsc: [RESOURCE; 3],
    pub weapon_rsc: [RESOURCE; 3],
    pub special_rsc: [RESOURCE; 3],
    pub captain_control: CAPTAIN_STUFF,
    pub victory_ditty_rsc: RESOURCE,
    pub ship_sounds_rsc: RESOURCE,
    pub ship: [FRAME; 3],
    pub weapon: [FRAME; 3],
    pub special: [FRAME; 3],
    pub victory_ditty: MUSIC_REF,
    pub ship_sounds: SOUND,
}
pub type SOUND = STRING;
pub type MUSIC_REF = *mut *mut TFB_SoundSample;
pub type TFB_SoundSample = tfb_soundsample;
pub type CAPTAIN_STUFF = captain_stuff;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct captain_stuff {
    pub captain_rsc: RESOURCE,
    pub background: FRAME,
    pub turn: FRAME,
    pub thrust: FRAME,
    pub weapon: FRAME,
    pub special: FRAME,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CHARACTERISTIC_STUFF {
    pub max_thrust: COUNT,
    pub thrust_increment: COUNT,
    pub energy_regeneration: BYTE,
    pub weapon_energy_cost: BYTE,
    pub special_energy_cost: BYTE,
    pub energy_wait: BYTE,
    pub turn_wait: BYTE,
    pub thrust_wait: BYTE,
    pub weapon_wait: BYTE,
    pub special_wait: BYTE,
    pub ship_mass: BYTE,
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
pub type HSTARSHIP = HLINK;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GLOBDATA {
    pub SIS_state: SIS_STATE,
    pub Game_state: GAME_STATE,
}
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
pub type ACTIVITY = UWORD;
pub type MENU_STATE = menu_state;
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
    pub flash_frame0: FRAME,
    pub flash_frame1: FRAME,
    pub flashContext: *mut FlashContext,
    pub hMusic: MUSIC_REF,
    pub privData: *mut ::core::ffi::c_void,
}
pub type MENU_SOUND_FLAGS = UWORD;
pub type TimePeriod = DWORD;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct MENU_SOUND_EFFECT(pub ::core::ffi::c_uint);
impl MENU_SOUND_EFFECT {
    pub const MENU_SOUND_MOVE: Self = Self(0);
    pub const MENU_SOUND_SUCCESS: Self = Self(1);
    pub const MENU_SOUND_FAILURE: Self = Self(2);
    pub const MENU_SOUND_INVOKED: Self = Self(3);
}
pub type CONTROLLER_INPUT_STATE = _controller_input_state;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _controller_input_state {
    pub key: [[::core::ffi::c_int; 7]; 6],
    pub menu: [::core::ffi::c_int; 24],
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_5(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_5 {
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
pub struct C2Rust_Unnamed_6(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_6 {
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
unsafe extern "C" fn LockStarShip(
    mut pq: *const QUEUE,
    mut h: HSTARSHIP,
) -> *mut STARSHIP {
    ((*pq).object_size as usize == ::core::mem::size_of::<STARSHIP>()
        || {
            __assert_fail(
                b"GetLinkSize (pq) == sizeof (STARSHIP)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/uqm/races.h\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                293 as ::core::ffi::c_int,
                b"LockStarShip\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    return LockLink(pq, h) as *mut STARSHIP;
}
unsafe extern "C" fn DoPickBattleShip(mut pMS: *mut MENU_STATE) -> BOOLEAN {
    if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
        & C2Rust_Unnamed_6::CHECK_ABORT.0 as ::core::ffi::c_int != 0
    {
        (*pMS).CurFrame = ::core::ptr::null_mut::<FRAME_DESC>();
        return BOOLEAN::FALSE;
    }
    SetMenuSounds(
        0 as ::core::ffi::c_int as MENU_SOUND_FLAGS,
        0 as ::core::ffi::c_int as MENU_SOUND_FLAGS,
    );
    let mut new_row: COORD = 0;
    let mut new_col: COORD = 0;
    let mut dx: ::core::ffi::c_int = 0;
    let mut dy: ::core::ffi::c_int = 0;
    let mut r: RECT = RECT {
        corner: HOT_SPOT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut t: TEXT = TEXT {
        baseline: HOT_SPOT { x: 0, y: 0 },
        pStr: ::core::ptr::null::<UNICODE>(),
        align: TEXT_ALIGN::ALIGN_LEFT,
        CharCount: 0,
    };
    let mut crew_level: COUNT = 0;
    let mut max_crew: COUNT = 0;
    let mut ship_index: COUNT = 0;
    let mut hBattleShip: HSTARSHIP = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut hNextShip: HSTARSHIP = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
    's_424: {
        if (*pMS).Initialized == 0 {
            (*pMS).Initialized = BOOLEAN::TRUE.0 as ::core::ffi::c_int as SIZE;
            (*pMS).InputFunc = Some(
                DoPickBattleShip as unsafe extern "C" fn(*mut MENU_STATE) -> BOOLEAN,
            ) as Option<unsafe extern "C" fn(*mut menu_state) -> BOOLEAN>;
        } else if PulsedInputState
            .menu[C2Rust_Unnamed_5::KEY_MENU_SELECT.0 as ::core::ffi::c_int as usize]
            != 0
        {
            if !((*pMS).CurFrame as HSTARSHIP).is_null() {
                PlayMenuSound(MENU_SOUND_EFFECT::MENU_SOUND_SUCCESS);
                return BOOLEAN::FALSE;
            }
            break 's_424;
        } else {
            new_row = 0;
            new_col = 0;
            dx = 0 as ::core::ffi::c_int;
            dy = 0 as ::core::ffi::c_int;
            if PulsedInputState
                .menu[C2Rust_Unnamed_5::KEY_MENU_RIGHT.0 as ::core::ffi::c_int as usize]
                != 0
            {
                dx = 1 as ::core::ffi::c_int;
            }
            if PulsedInputState
                .menu[C2Rust_Unnamed_5::KEY_MENU_LEFT.0 as ::core::ffi::c_int as usize]
                != 0
            {
                dx = -1 as ::core::ffi::c_int;
            }
            if PulsedInputState
                .menu[C2Rust_Unnamed_5::KEY_MENU_UP.0 as ::core::ffi::c_int as usize]
                != 0
            {
                dy = -1 as ::core::ffi::c_int;
            }
            if PulsedInputState
                .menu[C2Rust_Unnamed_5::KEY_MENU_DOWN.0 as ::core::ffi::c_int as usize]
                != 0
            {
                dy = 1 as ::core::ffi::c_int;
            }
            new_col = ((*pMS).first_item.x as ::core::ffi::c_int + dx) as COORD;
            new_row = ((*pMS).first_item.y as ::core::ffi::c_int + dy) as COORD;
            if new_row as ::core::ffi::c_int != (*pMS).first_item.y as ::core::ffi::c_int
                || new_col as ::core::ffi::c_int
                    != (*pMS).first_item.x as ::core::ffi::c_int
            {
                r = RECT {
                    corner: HOT_SPOT { x: 0, y: 0 },
                    extent: EXTENT { width: 0, height: 0 },
                };
                t = TEXT {
                    baseline: HOT_SPOT { x: 0, y: 0 },
                    pStr: ::core::ptr::null::<UNICODE>(),
                    align: TEXT_ALIGN::ALIGN_LEFT,
                    CharCount: 0,
                };
                crew_level = 0;
                max_crew = 0;
                ship_index = 0;
                hBattleShip = ::core::ptr::null_mut::<::core::ffi::c_void>();
                hNextShip = ::core::ptr::null_mut::<::core::ffi::c_void>();
                StarShipPtr = ::core::ptr::null_mut::<STARSHIP>();
                if (new_col as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                    new_col = 6 as COORD;
                } else if new_col as ::core::ffi::c_int > 6 as ::core::ffi::c_int {
                    new_col = 0 as COORD;
                }
                if (new_row as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                    new_row = (2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                        as COORD;
                } else if new_row as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
                    new_row = 0 as COORD;
                }
                PlayMenuSound(MENU_SOUND_EFFECT::MENU_SOUND_MOVE);
                (*pMS).first_item.y = new_row;
                (*pMS).first_item.x = new_col;
            } else {
                break 's_424;
            }
        }
        if (*pMS).first_item.x as ::core::ffi::c_int
            == 6 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int
        {
            (*pMS).flash_rect0.corner.x = ((*pMS).flash_rect1.corner.x
                as ::core::ffi::c_int - 2 as ::core::ffi::c_int
                + 65 as ::core::ffi::c_int) as COORD;
            (*pMS).flash_rect0.corner.y = ((*pMS).flash_rect1.corner.y
                as ::core::ffi::c_int - 2 as ::core::ffi::c_int
                + 4 as ::core::ffi::c_int) as COORD;
            (*pMS).flash_rect0.extent.width = (22 as ::core::ffi::c_int
                + 4 as ::core::ffi::c_int) as COORD;
            (*pMS).flash_rect0.extent.height = (48 as ::core::ffi::c_int
                + 4 as ::core::ffi::c_int) as COORD;
            hBattleShip = (*(&raw mut race_q as *mut QUEUE).offset(0isize)).tail
                as HSTARSHIP;
        } else {
            new_col = (*pMS).first_item.x;
            (*pMS).flash_rect0.corner.x = (5 as ::core::ffi::c_int
                + (*pMS).flash_rect1.corner.x as ::core::ffi::c_int
                - 2 as ::core::ffi::c_int
                + (16 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                    * new_col as ::core::ffi::c_int) as COORD;
            if new_col as ::core::ffi::c_int
                > 6 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int
            {
                new_col -= 1;
                (*pMS).flash_rect0.corner.x = ((*pMS).flash_rect0.corner.x
                    as ::core::ffi::c_int
                    + (22 as ::core::ffi::c_int - 16 as ::core::ffi::c_int)) as COORD;
            }
            (*pMS).flash_rect0.corner.y = (16 as ::core::ffi::c_int
                + (*pMS).flash_rect1.corner.y as ::core::ffi::c_int
                - 2 as ::core::ffi::c_int
                + (16 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                    * (*pMS).first_item.y as ::core::ffi::c_int) as COORD;
            (*pMS).flash_rect0.extent.width = (16 as ::core::ffi::c_int
                + 4 as ::core::ffi::c_int) as COORD;
            (*pMS).flash_rect0.extent.height = (16 as ::core::ffi::c_int
                + 4 as ::core::ffi::c_int) as COORD;
            ship_index = ((*pMS).first_item.y as ::core::ffi::c_int
                * 6 as ::core::ffi::c_int + new_col as ::core::ffi::c_int) as COUNT;
            hBattleShip = (*(&raw mut race_q as *mut QUEUE).offset(0isize)).head
                as HSTARSHIP;
            while hBattleShip != (*(&raw mut race_q as *mut QUEUE).offset(0isize)).tail {
                StarShipPtr = LockStarShip(
                    (&raw mut race_q as *mut QUEUE).offset(0isize),
                    hBattleShip,
                );
                if (*StarShipPtr).index as ::core::ffi::c_int
                    == ship_index as ::core::ffi::c_int
                    && (*StarShipPtr).SpeciesID.0 != SPECIES_ID::NO_ID.0
                {
                    UnlockLink(
                        (&raw mut race_q as *mut QUEUE).offset(0isize),
                        hBattleShip,
                    );
                    break;
                } else {
                    hNextShip = (*StarShipPtr).succ as HSTARSHIP;
                    UnlockLink(
                        (&raw mut race_q as *mut QUEUE).offset(0isize),
                        hBattleShip,
                    );
                    hBattleShip = hNextShip;
                }
            }
            if hBattleShip == (*(&raw mut race_q as *mut QUEUE).offset(0isize)).tail {
                hBattleShip = ::core::ptr::null_mut::<::core::ffi::c_void>();
            }
        }
        (*pMS).CurFrame = hBattleShip as FRAME;
        SetContextForeGroundColor(colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE));
        r.corner.x = ((*pMS).flash_rect1.corner.x as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int) as COORD;
        r.corner.y = ((*pMS).flash_rect1.corner.y as ::core::ffi::c_int
            + 5 as ::core::ffi::c_int) as COORD;
        r.extent.width = ((16 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
            * 3 as ::core::ffi::c_int - 4 as ::core::ffi::c_int) as COORD;
        r.extent.height = 7 as COORD;
        DrawFilledRectangle(&raw mut r);
        if hBattleShip.is_null() {
            crew_level = 0 as COUNT;
            max_crew = 0 as COUNT;
        } else {
            SetContextFont(TinyFont);
            t.baseline.x = (r.corner.x as ::core::ffi::c_int
                + (r.extent.width as ::core::ffi::c_int >> 1 as ::core::ffi::c_int))
                as COORD;
            t.baseline.y = (r.corner.y as ::core::ffi::c_int
                + (r.extent.height as ::core::ffi::c_int - 1 as ::core::ffi::c_int))
                as COORD;
            t.align = TEXT_ALIGN::ALIGN_CENTER;
            StarShipPtr = LockStarShip(
                (&raw mut race_q as *mut QUEUE).offset(0isize),
                hBattleShip,
            );
            if (*StarShipPtr).captains_name_index as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
            {
                t.pStr = &raw mut GlobData.SIS_state.CommanderName as *mut UNICODE;
                t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
                crew_level = GlobData.SIS_state.CrewEnlisted;
                max_crew = GetCrewPodCapacity();
            } else {
                let mut locString: STRING = ::core::ptr::null_mut::<
                    STRING_TABLE_ENTRY_DESC,
                >();
                locString = SetAbsStringTableIndex(
                    (*StarShipPtr).race_strings,
                    (*StarShipPtr).captains_name_index as COUNT,
                );
                t.pStr = GetStringAddress(locString);
                t.CharCount = GetStringLength(locString);
                crew_level = (*StarShipPtr).crew_level;
                max_crew = (*StarShipPtr).max_crew;
            }
            UnlockLink((&raw mut race_q as *mut QUEUE).offset(0isize), hBattleShip);
            SetContextForeGroundColor(
                colorFromRgb15(0x14 as BYTE, 0xa as BYTE, 0 as BYTE),
            );
            font_DrawText(&raw mut t);
            SetContextForeGroundColor(colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE));
        }
        r.corner.x = (r.corner.x as ::core::ffi::c_int
            + ((16 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                * ((6 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
                    + 1 as ::core::ffi::c_int) + 22 as ::core::ffi::c_int
                - 16 as ::core::ffi::c_int)) as COORD;
        DrawFilledRectangle(&raw mut r);
        if crew_level != 0 {
            let mut buf: [::core::ffi::c_char; 80] = [0; 80];
            t.baseline.x = (r.corner.x as ::core::ffi::c_int
                + (r.extent.width as ::core::ffi::c_int >> 1 as ::core::ffi::c_int))
                as COORD;
            t.pStr = &raw mut buf as *mut ::core::ffi::c_char;
            t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
            if crew_level as ::core::ffi::c_int >= max_crew as ::core::ffi::c_int {
                sprintf(
                    &raw mut buf as *mut ::core::ffi::c_char,
                    b"%u\0".as_ptr() as *const ::core::ffi::c_char,
                    crew_level as ::core::ffi::c_int,
                );
            } else {
                sprintf(
                    &raw mut buf as *mut ::core::ffi::c_char,
                    b"%u/%u\0".as_ptr() as *const ::core::ffi::c_char,
                    crew_level as ::core::ffi::c_int,
                    max_crew as ::core::ffi::c_int,
                );
            }
            SetContextForeGroundColor(
                colorFromRgb15(0 as BYTE, 0x14 as BYTE, 0 as BYTE),
            );
            font_DrawText(&raw mut t);
        }
        SetFlashRect(::core::ptr::null::<RECT>());
        SetFlashRect(&raw mut (*pMS).flash_rect0);
    }
    SleepThread((840 as ::core::ffi::c_int / 30 as ::core::ffi::c_int) as TimePeriod);
    return BOOLEAN::TRUE;
}
unsafe extern "C" fn GetArmadaStarShip() -> HSTARSHIP {
    let mut pick_r: RECT = RECT {
        corner: HOT_SPOT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut OldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    let mut hBattleShip: HSTARSHIP = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if battle_counter[1usize] as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    OldContext = SetContext(SpaceContext);
    DrawArmadaPickShip(BOOLEAN::FALSE, &raw mut pick_r);
    let mut MenuState: MENU_STATE = MENU_STATE {
        InputFunc: None,
        Initialized: 0,
        CurState: 0,
        CurFrame: ::core::ptr::null_mut::<FRAME_DESC>(),
        CurString: ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>(),
        first_item: HOT_SPOT { x: 0, y: 0 },
        delta_item: 0,
        ModuleFrame: ::core::ptr::null_mut::<FRAME_DESC>(),
        flash_rect0: RECT {
            corner: HOT_SPOT { x: 0, y: 0 },
            extent: EXTENT { width: 0, height: 0 },
        },
        flash_rect1: RECT {
            corner: HOT_SPOT { x: 0, y: 0 },
            extent: EXTENT { width: 0, height: 0 },
        },
        flash_frame0: ::core::ptr::null_mut::<FRAME_DESC>(),
        flash_frame1: ::core::ptr::null_mut::<FRAME_DESC>(),
        flashContext: ::core::ptr::null_mut::<FlashContext>(),
        hMusic: ::core::ptr::null_mut::<*mut TFB_SoundSample>(),
        privData: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    MenuState.InputFunc = Some(
        DoPickBattleShip as unsafe extern "C" fn(*mut MENU_STATE) -> BOOLEAN,
    ) as Option<unsafe extern "C" fn(*mut menu_state) -> BOOLEAN>;
    MenuState.Initialized = BOOLEAN::FALSE.0 as ::core::ffi::c_int as SIZE;
    MenuState.first_item.x = (6 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
        as COORD;
    MenuState.first_item.y = 0 as COORD;
    MenuState.CurFrame = ::core::ptr::null_mut::<FRAME_DESC>();
    MenuState.flash_rect1.corner = pick_r.corner;
    MenuState.flash_rect1.extent.width = 0 as COORD;
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
    DoInput(&raw mut MenuState as *mut ::core::ffi::c_void, BOOLEAN::FALSE);
    SetFlashRect(::core::ptr::null::<RECT>());
    hBattleShip = MenuState.CurFrame as HSTARSHIP;
    if !hBattleShip.is_null() {
        if hBattleShip == (*(&raw mut race_q as *mut QUEUE).offset(0isize)).tail {
            battle_counter[0usize] = 1 as BYTE;
        }
        WaitForSoundEnd(0 as COUNT);
    }
    SetContext(OldContext);
    return hBattleShip;
}
#[no_mangle]
pub unsafe extern "C" fn GetEncounterStarShip(
    mut LastStarShipPtr: *mut STARSHIP,
    mut which_player: COUNT,
) -> HSTARSHIP {
    if inHQSpace().0 != 0 {
        (which_player as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            || {
                __assert_fail(
                    b"which_player == RPG_PLAYER_NUM\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"src/uqm/pickship.c\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    309 as ::core::ffi::c_int,
                    b"GetEncounterStarShip\0".as_ptr() as *const ::core::ffi::c_char,
                );
                0 as ::core::ffi::c_int != 0
            }) as ::core::ffi::c_int;
        return (*(&raw mut race_q as *mut QUEUE).offset(which_player as isize)).head;
    } else if GlobData.Game_state.CurrentActivity as BYTE as ::core::ffi::c_int
        == C2Rust_Unnamed_6::SUPER_MELEE.0 as ::core::ffi::c_int
    {
        let mut hBattleShip: HSTARSHIP = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if battle_counter[0usize] as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            || battle_counter[1usize] as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        {
            return ::core::ptr::null_mut::<::core::ffi::c_void>();
        }
        if GetNextMeleeStarShip(which_player, &raw mut hBattleShip).0 == 0 {
            return ::core::ptr::null_mut::<::core::ffi::c_void>();
        }
        return hBattleShip;
    } else if which_player as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        if LastStarShipPtr.is_null()
            && battle_counter[which_player as usize] as ::core::ffi::c_int
                == 1 as ::core::ffi::c_int
        {
            return (*(&raw mut race_q as *mut QUEUE).offset(which_player as isize)).tail
        } else if battle_counter[which_player as usize] != 0 {
            return GetArmadaStarShip()
        } else if !LastStarShipPtr.is_null() {
            if (*LastStarShipPtr).crew_level as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
            {
                GlobData.SIS_state.CrewEnlisted = !(0 as ::core::ffi::c_int) as COUNT;
            } else if GlobData.SIS_state.FuelOnBoard
                > (5 as ::core::ffi::c_int * 100 as ::core::ffi::c_int) as DWORD
            {
                GlobData.SIS_state.FuelOnBoard = GlobData
                    .SIS_state
                    .FuelOnBoard
                    .wrapping_sub(
                        (5 as ::core::ffi::c_int * 100 as ::core::ffi::c_int) as DWORD,
                    );
            } else {
                GlobData.SIS_state.FuelOnBoard = 0 as DWORD;
            }
        }
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    } else {
        if FleetIsInfinite(which_player).0 != 0 {
            if !LastStarShipPtr.is_null() {
                (*LastStarShipPtr).crew_level = (*LastStarShipPtr).max_crew;
                (*LastStarShipPtr).playerNr = which_player as SIZE;
                (*LastStarShipPtr).captains_name_index = ((TFB_Random() as COUNT
                    as ::core::ffi::c_int
                    & 16 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                    + 5 as ::core::ffi::c_int) as BYTE;
            }
            battle_counter[which_player as usize] = battle_counter[which_player as usize]
                .wrapping_add(1);
            return (*(&raw mut race_q as *mut QUEUE).offset(which_player as isize)).head;
        }
        if !LastStarShipPtr.is_null() {
            return (*LastStarShipPtr).succ;
        }
        return (*(&raw mut race_q as *mut QUEUE).offset(which_player as isize)).head;
    };
}
#[no_mangle]
pub unsafe extern "C" fn DrawArmadaPickShip(
    mut draw_salvage_frame: BOOLEAN,
    mut pPickRect: *mut RECT,
) {
    let mut hBattleShip: HSTARSHIP = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut hNextShip: HSTARSHIP = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
    let mut r: RECT = RECT {
        corner: HOT_SPOT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut pick_r: RECT = RECT {
        corner: HOT_SPOT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut s: STAMP = STAMP {
        origin: HOT_SPOT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    let mut t: TEXT = TEXT {
        baseline: HOT_SPOT { x: 0, y: 0 },
        pStr: ::core::ptr::null::<UNICODE>(),
        align: TEXT_ALIGN::ALIGN_LEFT,
        CharCount: 0,
    };
    let mut OldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    let mut PickFrame: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
    OldContext = SetContext(SpaceContext);
    PickFrame = CaptureDrawable(
        LoadGraphicInstance(b"graphics.pickship\0".as_ptr() as RESOURCE) as DRAWABLE,
    );
    BatchGraphics();
    s.frame = PickFrame;
    SetFrameHot(s.frame, MAKE_HOT_SPOT(0 as COORD, 0 as COORD));
    GetFrameRect(s.frame, &raw mut pick_r);
    GetContextClipRect(&raw mut r);
    pick_r.corner.x = ((r.extent.width as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
        - (pick_r.extent.width as ::core::ffi::c_int >> 1 as ::core::ffi::c_int))
        as COORD;
    pick_r.corner.y = ((r.extent.height as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
        - (pick_r.extent.height as ::core::ffi::c_int >> 1 as ::core::ffi::c_int))
        as COORD;
    if draw_salvage_frame.0 == 0 {
        *pPickRect = pick_r;
    } else {
        s.origin.x = (r.extent.width as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
            as COORD;
        s.frame = IncFrameIndex(s.frame);
        SetFrameHot(s.frame, MAKE_HOT_SPOT(0 as COORD, 0 as COORD));
        GetFrameRect(s.frame, &raw mut r);
        s.origin.x = (s.origin.x as ::core::ffi::c_int
            - (r.extent.width as ::core::ffi::c_int >> 1 as ::core::ffi::c_int))
            as COORD;
        s.origin.y = (pick_r.corner.y as ::core::ffi::c_int
            - (r.extent.height as ::core::ffi::c_int >> 1 as ::core::ffi::c_int))
            as COORD;
        DrawStamp(&raw mut s);
        s.frame = DecFrameIndex(s.frame);
        pick_r.corner.y = (s.origin.y as ::core::ffi::c_int
            + r.extent.height as ::core::ffi::c_int) as COORD;
        r.corner.x = pick_r.corner.x;
        r.corner.y = s.origin.y;
        *pPickRect = r;
    }
    s.origin = pick_r.corner;
    DrawStamp(&raw mut s);
    t.baseline.x = (pick_r.corner.x as ::core::ffi::c_int
        + (pick_r.extent.width as ::core::ffi::c_int >> 1 as ::core::ffi::c_int))
        as COORD;
    t.baseline.y = (pick_r.corner.y as ::core::ffi::c_int
        + pick_r.extent.height as ::core::ffi::c_int - 5 as ::core::ffi::c_int) as COORD;
    t.align = TEXT_ALIGN::ALIGN_CENTER;
    t.pStr = &raw mut GlobData.SIS_state.ShipName as *mut UNICODE;
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    SetContextForeGroundColor(colorFromRgb15(0x12 as BYTE, 0x12 as BYTE, 0x12 as BYTE));
    SetContextFont(StarConFont);
    font_DrawText(&raw mut t);
    r.extent.width = 16 as COORD;
    r.extent.height = 16 as COORD;
    hBattleShip = (*(&raw mut race_q as *mut QUEUE).offset(0isize)).head as HSTARSHIP;
    while !hBattleShip.is_null() {
        StarShipPtr = LockStarShip(
            (&raw mut race_q as *mut QUEUE).offset(0isize),
            hBattleShip,
        );
        if (*StarShipPtr).captains_name_index != 0 {
            let mut ship_index: COUNT = 0;
            ship_index = (*StarShipPtr).index;
            s.origin.x = (pick_r.corner.x as ::core::ffi::c_int
                + (5 as ::core::ffi::c_int
                    + (16 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                        * (ship_index as ::core::ffi::c_int % 6 as ::core::ffi::c_int)))
                as COORD;
            if ship_index as ::core::ffi::c_int % 6 as ::core::ffi::c_int
                >= 6 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int
            {
                s.origin.x = (s.origin.x as ::core::ffi::c_int
                    + (22 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)) as COORD;
            }
            s.origin.y = (pick_r.corner.y as ::core::ffi::c_int
                + (16 as ::core::ffi::c_int
                    + (16 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                        * (ship_index as ::core::ffi::c_int / 6 as ::core::ffi::c_int)))
                as COORD;
            s.frame = (*StarShipPtr).icons;
            r.corner = s.origin;
            SetContextForeGroundColor(colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE));
            DrawFilledRectangle(&raw mut r);
            if (*StarShipPtr).SpeciesID.0 != SPECIES_ID::NO_ID.0
                || (*StarShipPtr).crew_level as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
            {
                DrawStamp(&raw mut s);
                if (*StarShipPtr).SpeciesID.0 == SPECIES_ID::NO_ID.0 {
                    s.origin.x = (s.origin.x as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int) as COORD;
                    s.frame = SetAbsFrameIndex(StatusFrame, 3 as COUNT);
                    DrawStamp(&raw mut s);
                }
            } else {
                SetContextForeGroundColor(
                    colorFromRgb15(0 as BYTE, 0 as BYTE, 0x14 as BYTE),
                );
                DrawFilledStamp(&raw mut s);
            }
        }
        hNextShip = (*StarShipPtr).succ as HSTARSHIP;
        UnlockLink((&raw mut race_q as *mut QUEUE).offset(0isize), hBattleShip);
        hBattleShip = hNextShip;
    }
    UnbatchGraphics();
    DestroyDrawable(ReleaseDrawable(PickFrame));
    SetContext(OldContext);
}
