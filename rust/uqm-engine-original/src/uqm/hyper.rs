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

// Rust translation of src/uqm/hyper.c.
extern "C" {
    pub type string_table_entry;
    pub type string_table;
    pub type context_desc;
    pub type frame_desc;
    pub type drawable_desc;
    pub type tfb_soundsample;
    pub type FlashContext;
    fn __assert_fail(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    );
    fn AllocLink(pq: *mut QUEUE) -> HLINK;
    fn FreeLink(pq: *mut QUEUE, hLink: HLINK);
    fn PutQueue(pq: *mut QUEUE, hLink: HLINK);
    fn InsertQueue(pq: *mut QUEUE, hLink: HLINK, hRefLink: HLINK);
    fn RemoveQueue(pq: *mut QUEUE, hLink: HLINK);
    fn SetContext(Context: CONTEXT) -> CONTEXT;
    fn SetContextForeGroundColor(Color: Color) -> Color;
    fn SetContextBackGroundColor(Color: Color) -> Color;
    fn DrawStamp(pStamp: *mut STAMP);
    fn DrawPoint(pPoint: *mut POINT);
    fn DrawFilledRectangle(pRect: *mut RECT);
    fn BatchGraphics();
    fn UnbatchGraphics();
    fn ClearDrawable();
    fn DestroyDrawable(Drawable: DRAWABLE) -> BOOLEAN;
    fn LoadGraphicInstance(res: RESOURCE) -> *mut ::core::ffi::c_void;
    fn GetFrameCount(Frame: FRAME) -> COUNT;
    fn GetFrameIndex(Frame: FRAME) -> COUNT;
    fn SetAbsFrameIndex(Frame: FRAME, FrameIndex: COUNT) -> FRAME;
    fn SetRelFrameIndex(Frame: FRAME, FrameOffs: SIZE) -> FRAME;
    fn IncFrameIndex(Frame: FRAME) -> FRAME;
    fn DecFrameIndex(Frame: FRAME) -> FRAME;
    fn CaptureDrawable(Drawable: DRAWABLE) -> FRAME;
    fn ReleaseDrawable(Frame: FRAME) -> DRAWABLE;
    fn SetColorMap(ColorMapPtr: COLORMAPPTR) -> BOOLEAN;
    fn GetColorMapAddress(_: COLORMAP) -> COLORMAPPTR;
    fn LoadStringTableInstance(res: RESOURCE) -> STRING_TABLE;
    fn DestroyStringTable(StringTable: STRING_TABLE) -> BOOLEAN;
    fn CaptureStringTable(StringTable: STRING_TABLE) -> STRING;
    fn ReleaseStringTable(String: STRING) -> STRING_TABLE;
    static mut ScreenWidth: ::core::ffi::c_int;
    static mut ScreenHeight: ::core::ffi::c_int;
    static mut sinetab: [SIZE; 0];
    fn ARCTAN(delta_x: SIZE, delta_y: SIZE) -> COUNT;
    fn memset(
        _: *mut ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn GetCurrentVelocityComponents(
        velocityptr: *mut VELOCITY_DESC,
        pdx: *mut SIZE,
        pdy: *mut SIZE,
    );
    fn GetNextVelocityComponents(
        velocityptr: *mut VELOCITY_DESC,
        pdx: *mut SIZE,
        pdy: *mut SIZE,
        num_frames: COUNT,
    );
    fn SetVelocityComponents(velocityptr: *mut VELOCITY_DESC, dx: SIZE, dy: SIZE);
    static mut disp_q: QUEUE;
    static mut DisplayArray: [PRIMITIVE; 330];
    fn AllocElement() -> HELEMENT;
    fn GetStarShipFromIndex(pShipQ: *mut QUEUE, Index: COUNT) -> HLINK;
    static mut PulsedInputState: CONTROLLER_INPUT_STATE;
    fn WaitForNoInput(duration: TimePeriod, resetInput: BOOLEAN) -> BOOLEAN;
    fn DoInput(pInputState: *mut ::core::ffi::c_void, resetInput: BOOLEAN);
    fn DoMenuChooser(pMS: *mut MENU_STATE, BaseState: BYTE) -> BOOLEAN;
    fn DrawMenuStateStrings(beg_index: BYTE, NewState: SWORD);
    fn RepairSISBorder();
    fn DrawSISFrame();
    fn ClearSISRect(ClearFlags: BYTE);
    fn SetFlashRect(pRect: *const RECT);
    fn DrawHyperCoords(puniverse: POINT);
    fn DrawSISMessage(pStr: *const UNICODE);
    fn DeltaSISGauges(
        crew_delta: SIZE,
        fuel_delta: SIZE,
        resunit_delta: ::core::ffi::c_int,
    );
    static mut GlobData: GLOBDATA;
    fn getGameState(
        state: *mut BYTE,
        startBit: ::core::ffi::c_int,
        endBit: ::core::ffi::c_int,
    ) -> BYTE;
    fn setGameState(
        state: *mut BYTE,
        startBit: ::core::ffi::c_int,
        endBit: ::core::ffi::c_int,
        val: BYTE,
    );
    static mut RadarContext: CONTEXT;
    fn TFB_Random() -> DWORD;
    fn square_root(value: DWORD) -> COUNT;
    static mut CurStarDescPtr: *mut STAR_DESC;
    static mut star_array: *mut STAR_DESC;
    fn FindStar(
        pLastStar: *mut STAR_DESC,
        puniverse: *mut POINT,
        xbounds: SIZE,
        ybounds: SIZE,
    ) -> *mut STAR_DESC;
    fn animation_preprocess(ElementPtr: *mut ELEMENT);
    fn ship_preprocess(ElementPtr: *mut ELEMENT);
    fn ship_postprocess(ElementPtr: *mut ELEMENT);
    fn CargoMenu();
    fn RosterMenu() -> BOOLEAN;
    fn DevicesMenu() -> BOOLEAN;
    fn StarMap() -> BOOLEAN;
    fn SetMenuSounds(sound_0: MENU_SOUND_FLAGS, sound_1: MENU_SOUND_FLAGS);
    static mut optWhichMenu: ::core::ffi::c_int;
    static mut SpaceContext: CONTEXT;
    static mut StatusContext: CONTEXT;
    static mut race_q: [QUEUE; 0];
    static mut LastActivity: ACTIVITY;
    static mut stars_in_space: FRAME;
    fn SetUpElement(ElementPtr: *mut ELEMENT);
    fn GameOptions() -> BOOLEAN;
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
pub struct Color {
    pub r: BYTE,
    pub g: BYTE,
    pub b: BYTE,
    pub a: BYTE,
}
pub type RESOURCE = *const ::core::ffi::c_char;
pub type STRING_TABLE_ENTRY_DESC = string_table_entry;
pub type STRING_TABLE_DESC = string_table;
pub type STRING_TABLE = *mut STRING_TABLE_DESC;
pub type STRING = *mut STRING_TABLE_ENTRY_DESC;
pub type CONTEXT_DESC = context_desc;
pub type FRAME_DESC = frame_desc;
pub type DRAWABLE_DESC = drawable_desc;
pub type CONTEXT = *mut CONTEXT_DESC;
pub type FRAME = *mut FRAME_DESC;
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
pub type COLORMAP = STRING;
pub type COLORMAPPTR = *mut ::core::ffi::c_void;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct gfx_object(pub ::core::ffi::c_uint);
impl gfx_object {
    pub const POINT_PRIM: Self = Self(0);
    pub const STAMP_PRIM: Self = Self(1);
    pub const STAMPFILL_PRIM: Self = Self(2);
    pub const LINE_PRIM: Self = Self(3);
    pub const TEXT_PRIM: Self = Self(4);
    pub const RECT_PRIM: Self = Self(5);
    pub const RECTFILL_PRIM: Self = Self(6);
    pub const NUM_PRIMS: Self = Self(7);
}
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
pub type HELEMENT = HLINK;
pub type ELEMENT_FLAGS = UWORD;
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
pub type STATE = state;
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
pub type ElementProcessFunc = unsafe extern "C" fn(*mut ELEMENT) -> ();
pub type ELEMENT = element;
pub type ElementCollisionFunc = unsafe extern "C" fn(
    *mut ELEMENT,
    *mut POINT,
    *mut ELEMENT,
    *mut POINT,
) -> ();
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
pub type HENCOUNTER = HLINK;
pub type ENCOUNTER = encounter;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct encounter {
    pub pred: HENCOUNTER,
    pub succ: HENCOUNTER,
    pub hElement: HELEMENT,
    pub transition_state: SIZE,
    pub origin: POINT,
    pub radius: COUNT,
    pub race_id: BYTE,
    pub num_ships: BYTE,
    pub flags: BYTE,
    pub loc_pt: POINT,
    pub ShipList: [BRIEF_SHIP_INFO; 7],
    pub log_x: SDWORD,
    pub log_y: SDWORD,
}
pub type BRIEF_SHIP_INFO = brief_ship_info;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct brief_ship_info {
    pub race_id: BYTE,
    pub crew_level: COUNT,
    pub max_crew: COUNT,
    pub max_energy: BYTE,
}
pub type HSTARSHIP = HLINK;
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct TRANSITION_TYPE(pub ::core::ffi::c_uint);
impl TRANSITION_TYPE {
    pub const RANDOM_ENCOUNTER_TRANSITION: Self = Self(0);
    pub const INTERPLANETARY_TRANSITION: Self = Self(1);
    pub const ARILOU_SPACE_TRANSITION: Self = Self(2);
}
pub type STAR_DESC = star_desc;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct star_desc {
    pub star_pt: POINT,
    pub Type: BYTE,
    pub Index: BYTE,
    pub Prefix: BYTE,
    pub Postfix: BYTE,
}
pub type HFLEETINFO = HLINK;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FLEET_INFO {
    pub pred: HFLEETINFO,
    pub succ: HFLEETINFO,
    pub SpeciesID: SPECIES_ID,
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
}
pub type TimePeriod = DWORD;
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
    pub const ARILOU_SHIP: Self = Self(0);
    pub const CHMMR_SHIP: Self = Self(1);
    pub const HUMAN_SHIP: Self = Self(2);
    pub const ORZ_SHIP: Self = Self(3);
    pub const PKUNK_SHIP: Self = Self(4);
    pub const SHOFIXTI_SHIP: Self = Self(5);
    pub const SPATHI_SHIP: Self = Self(6);
    pub const SUPOX_SHIP: Self = Self(7);
    pub const THRADDASH_SHIP: Self = Self(8);
    pub const UTWIG_SHIP: Self = Self(9);
    pub const VUX_SHIP: Self = Self(10);
    pub const YEHAT_SHIP: Self = Self(11);
    pub const MELNORME_SHIP: Self = Self(12);
    pub const DRUUGE_SHIP: Self = Self(13);
    pub const ILWRATH_SHIP: Self = Self(14);
    pub const MYCON_SHIP: Self = Self(15);
    pub const SLYLANDRO_SHIP: Self = Self(16);
    pub const UMGAH_SHIP: Self = Self(17);
    pub const URQUAN_SHIP: Self = Self(18);
    pub const ZOQFOTPIK_SHIP: Self = Self(19);
    pub const SYREEN_SHIP: Self = Self(20);
    pub const BLACK_URQUAN_SHIP: Self = Self(21);
    pub const YEHAT_REBEL_SHIP: Self = Self(22);
    pub const URQUAN_DRONE_SHIP: Self = Self(23);
    pub const SAMATRA_SHIP: Self = Self(23);
    pub const NUM_AVAILABLE_RACES: Self = Self(24);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_6(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_6 {
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
pub struct C2Rust_Unnamed_7(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_7 {
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
    pub const PM_CHANGE_CAPTAIN: Self = Self(36);
    pub const PM_CHANGE_SHIP: Self = Self(37);
    pub const PM_EXIT_SETTINGS: Self = Self(38);
    pub const PM_ALT_SCAN: Self = Self(39);
    pub const PM_ALT_STARMAP: Self = Self(40);
    pub const PM_ALT_MANIFEST: Self = Self(41);
    pub const PM_ALT_GAME_MENU: Self = Self(42);
    pub const PM_ALT_NAVIGATE: Self = Self(43);
    pub const PM_ALT_CARGO: Self = Self(44);
    pub const PM_ALT_DEVICES: Self = Self(45);
    pub const PM_ALT_ROSTER: Self = Self(46);
    pub const PM_ALT_EXIT_MANIFEST: Self = Self(47);
    pub const PM_ALT_MSCAN: Self = Self(48);
    pub const PM_ALT_ESCAN: Self = Self(49);
    pub const PM_ALT_BSCAN: Self = Self(50);
    pub const PM_ALT_ASCAN: Self = Self(51);
    pub const PM_ALT_DISPATCH: Self = Self(52);
    pub const PM_ALT_EXIT_SCAN: Self = Self(53);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_8(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_8 {
    pub const SHOFIXTI_VISITS: Self = Self(0);
    pub const END_SHOFIXTI_VISITS: Self = Self(2);
    pub const SHOFIXTI_STACK1: Self = Self(3);
    pub const END_SHOFIXTI_STACK1: Self = Self(4);
    pub const SHOFIXTI_STACK2: Self = Self(5);
    pub const END_SHOFIXTI_STACK2: Self = Self(7);
    pub const SHOFIXTI_STACK3: Self = Self(8);
    pub const END_SHOFIXTI_STACK3: Self = Self(9);
    pub const SHOFIXTI_KIA: Self = Self(10);
    pub const END_SHOFIXTI_KIA: Self = Self(10);
    pub const SHOFIXTI_BRO_KIA: Self = Self(11);
    pub const END_SHOFIXTI_BRO_KIA: Self = Self(11);
    pub const SHOFIXTI_RECRUITED: Self = Self(12);
    pub const END_SHOFIXTI_RECRUITED: Self = Self(12);
    pub const SHOFIXTI_MAIDENS: Self = Self(13);
    pub const END_SHOFIXTI_MAIDENS: Self = Self(13);
    pub const MAIDENS_ON_SHIP: Self = Self(14);
    pub const END_MAIDENS_ON_SHIP: Self = Self(14);
    pub const BATTLE_SEGUE: Self = Self(15);
    pub const END_BATTLE_SEGUE: Self = Self(15);
    pub const PLANETARY_LANDING: Self = Self(16);
    pub const END_PLANETARY_LANDING: Self = Self(16);
    pub const PLANETARY_CHANGE: Self = Self(17);
    pub const END_PLANETARY_CHANGE: Self = Self(17);
    pub const SPATHI_VISITS: Self = Self(18);
    pub const END_SPATHI_VISITS: Self = Self(20);
    pub const SPATHI_HOME_VISITS: Self = Self(21);
    pub const END_SPATHI_HOME_VISITS: Self = Self(23);
    pub const FOUND_PLUTO_SPATHI: Self = Self(24);
    pub const END_FOUND_PLUTO_SPATHI: Self = Self(25);
    pub const SPATHI_SHIELDED_SELVES: Self = Self(26);
    pub const END_SPATHI_SHIELDED_SELVES: Self = Self(26);
    pub const SPATHI_CREATURES_EXAMINED: Self = Self(27);
    pub const END_SPATHI_CREATURES_EXAMINED: Self = Self(27);
    pub const SPATHI_CREATURES_ELIMINATED: Self = Self(28);
    pub const END_SPATHI_CREATURES_ELIMINATED: Self = Self(28);
    pub const UMGAH_BROADCASTERS: Self = Self(29);
    pub const END_UMGAH_BROADCASTERS: Self = Self(29);
    pub const SPATHI_MANNER: Self = Self(30);
    pub const END_SPATHI_MANNER: Self = Self(31);
    pub const SPATHI_QUEST: Self = Self(32);
    pub const END_SPATHI_QUEST: Self = Self(32);
    pub const LIED_ABOUT_CREATURES: Self = Self(33);
    pub const END_LIED_ABOUT_CREATURES: Self = Self(34);
    pub const SPATHI_PARTY: Self = Self(35);
    pub const END_SPATHI_PARTY: Self = Self(35);
    pub const KNOW_SPATHI_PASSWORD: Self = Self(36);
    pub const END_KNOW_SPATHI_PASSWORD: Self = Self(36);
    pub const ILWRATH_HOME_VISITS: Self = Self(37);
    pub const END_ILWRATH_HOME_VISITS: Self = Self(39);
    pub const ILWRATH_CHMMR_VISITS: Self = Self(40);
    pub const END_ILWRATH_CHMMR_VISITS: Self = Self(40);
    pub const ARILOU_SPACE: Self = Self(41);
    pub const END_ARILOU_SPACE: Self = Self(41);
    pub const ARILOU_SPACE_SIDE: Self = Self(42);
    pub const END_ARILOU_SPACE_SIDE: Self = Self(43);
    pub const ARILOU_SPACE_COUNTER: Self = Self(44);
    pub const END_ARILOU_SPACE_COUNTER: Self = Self(47);
    pub const LANDER_SHIELDS: Self = Self(48);
    pub const END_LANDER_SHIELDS: Self = Self(51);
    pub const MET_MELNORME: Self = Self(52);
    pub const END_MET_MELNORME: Self = Self(52);
    pub const MELNORME_RESCUE_REFUSED: Self = Self(53);
    pub const END_MELNORME_RESCUE_REFUSED: Self = Self(53);
    pub const MELNORME_RESCUE_COUNT: Self = Self(54);
    pub const END_MELNORME_RESCUE_COUNT: Self = Self(56);
    pub const TRADED_WITH_MELNORME: Self = Self(57);
    pub const END_TRADED_WITH_MELNORME: Self = Self(57);
    pub const WHY_MELNORME_PURPLE: Self = Self(58);
    pub const END_WHY_MELNORME_PURPLE: Self = Self(58);
    pub const MELNORME_CREDIT0: Self = Self(59);
    pub const END_MELNORME_CREDIT0: Self = Self(66);
    pub const MELNORME_CREDIT1: Self = Self(67);
    pub const END_MELNORME_CREDIT1: Self = Self(74);
    pub const MELNORME_BUSINESS_COUNT: Self = Self(75);
    pub const END_MELNORME_BUSINESS_COUNT: Self = Self(76);
    pub const MELNORME_YACK_STACK0: Self = Self(77);
    pub const END_MELNORME_YACK_STACK0: Self = Self(78);
    pub const MELNORME_YACK_STACK1: Self = Self(79);
    pub const END_MELNORME_YACK_STACK1: Self = Self(80);
    pub const MELNORME_YACK_STACK2: Self = Self(81);
    pub const END_MELNORME_YACK_STACK2: Self = Self(84);
    pub const MELNORME_YACK_STACK3: Self = Self(85);
    pub const END_MELNORME_YACK_STACK3: Self = Self(87);
    pub const MELNORME_YACK_STACK4: Self = Self(88);
    pub const END_MELNORME_YACK_STACK4: Self = Self(89);
    pub const WHY_MELNORME_BLUE: Self = Self(90);
    pub const END_WHY_MELNORME_BLUE: Self = Self(90);
    pub const MELNORME_ANGER: Self = Self(91);
    pub const END_MELNORME_ANGER: Self = Self(92);
    pub const MELNORME_MIFFED_COUNT: Self = Self(93);
    pub const END_MELNORME_MIFFED_COUNT: Self = Self(94);
    pub const MELNORME_PISSED_COUNT: Self = Self(95);
    pub const END_MELNORME_PISSED_COUNT: Self = Self(96);
    pub const MELNORME_HATE_COUNT: Self = Self(97);
    pub const END_MELNORME_HATE_COUNT: Self = Self(98);
    pub const PROBE_MESSAGE_DELIVERED: Self = Self(99);
    pub const END_PROBE_MESSAGE_DELIVERED: Self = Self(99);
    pub const PROBE_ILWRATH_ENCOUNTER: Self = Self(100);
    pub const END_PROBE_ILWRATH_ENCOUNTER: Self = Self(100);
    pub const STARBASE_AVAILABLE: Self = Self(101);
    pub const END_STARBASE_AVAILABLE: Self = Self(101);
    pub const STARBASE_VISITED: Self = Self(102);
    pub const END_STARBASE_VISITED: Self = Self(102);
    pub const RADIOACTIVES_PROVIDED: Self = Self(103);
    pub const END_RADIOACTIVES_PROVIDED: Self = Self(103);
    pub const LANDERS_LOST: Self = Self(104);
    pub const END_LANDERS_LOST: Self = Self(104);
    pub const GIVEN_FUEL_BEFORE: Self = Self(105);
    pub const END_GIVEN_FUEL_BEFORE: Self = Self(105);
    pub const AWARE_OF_SAMATRA: Self = Self(106);
    pub const END_AWARE_OF_SAMATRA: Self = Self(106);
    pub const YEHAT_CAVALRY_ARRIVED: Self = Self(107);
    pub const END_YEHAT_CAVALRY_ARRIVED: Self = Self(107);
    pub const URQUAN_MESSED_UP: Self = Self(108);
    pub const END_URQUAN_MESSED_UP: Self = Self(108);
    pub const MOONBASE_DESTROYED: Self = Self(109);
    pub const END_MOONBASE_DESTROYED: Self = Self(109);
    pub const WILL_DESTROY_BASE: Self = Self(110);
    pub const END_WILL_DESTROY_BASE: Self = Self(110);
    pub const ARTIFACT_2_ON_SHIP: Self = Self(111);
    pub const END_ARTIFACT_2_ON_SHIP: Self = Self(111);
    pub const ARTIFACT_3_ON_SHIP: Self = Self(112);
    pub const END_ARTIFACT_3_ON_SHIP: Self = Self(112);
    pub const KOHR_AH_KILLED_ALL: Self = Self(113);
    pub const END_KOHR_AH_KILLED_ALL: Self = Self(113);
    pub const STARBASE_YACK_STACK1: Self = Self(114);
    pub const END_STARBASE_YACK_STACK1: Self = Self(114);
    pub const DISCUSSED_PORTAL_SPAWNER: Self = Self(115);
    pub const END_DISCUSSED_PORTAL_SPAWNER: Self = Self(115);
    pub const DISCUSSED_TALKING_PET: Self = Self(116);
    pub const END_DISCUSSED_TALKING_PET: Self = Self(116);
    pub const DISCUSSED_UTWIG_BOMB: Self = Self(117);
    pub const END_DISCUSSED_UTWIG_BOMB: Self = Self(117);
    pub const DISCUSSED_SUN_EFFICIENCY: Self = Self(118);
    pub const END_DISCUSSED_SUN_EFFICIENCY: Self = Self(118);
    pub const DISCUSSED_ROSY_SPHERE: Self = Self(119);
    pub const END_DISCUSSED_ROSY_SPHERE: Self = Self(119);
    pub const DISCUSSED_AQUA_HELIX: Self = Self(120);
    pub const END_DISCUSSED_AQUA_HELIX: Self = Self(120);
    pub const DISCUSSED_CLEAR_SPINDLE: Self = Self(121);
    pub const END_DISCUSSED_CLEAR_SPINDLE: Self = Self(121);
    pub const DISCUSSED_ULTRON: Self = Self(122);
    pub const END_DISCUSSED_ULTRON: Self = Self(122);
    pub const DISCUSSED_MAIDENS: Self = Self(123);
    pub const END_DISCUSSED_MAIDENS: Self = Self(123);
    pub const DISCUSSED_UMGAH_HYPERWAVE: Self = Self(124);
    pub const END_DISCUSSED_UMGAH_HYPERWAVE: Self = Self(124);
    pub const DISCUSSED_BURVIX_HYPERWAVE: Self = Self(125);
    pub const END_DISCUSSED_BURVIX_HYPERWAVE: Self = Self(125);
    pub const SYREEN_WANT_PROOF: Self = Self(126);
    pub const END_SYREEN_WANT_PROOF: Self = Self(126);
    pub const PLAYER_HAVING_SEX: Self = Self(127);
    pub const END_PLAYER_HAVING_SEX: Self = Self(127);
    pub const MET_ARILOU: Self = Self(128);
    pub const END_MET_ARILOU: Self = Self(128);
    pub const DISCUSSED_TAALO_PROTECTOR: Self = Self(129);
    pub const END_DISCUSSED_TAALO_PROTECTOR: Self = Self(129);
    pub const DISCUSSED_EGG_CASING0: Self = Self(130);
    pub const END_DISCUSSED_EGG_CASING0: Self = Self(130);
    pub const DISCUSSED_EGG_CASING1: Self = Self(131);
    pub const END_DISCUSSED_EGG_CASING1: Self = Self(131);
    pub const DISCUSSED_EGG_CASING2: Self = Self(132);
    pub const END_DISCUSSED_EGG_CASING2: Self = Self(132);
    pub const DISCUSSED_SYREEN_SHUTTLE: Self = Self(133);
    pub const END_DISCUSSED_SYREEN_SHUTTLE: Self = Self(133);
    pub const DISCUSSED_VUX_BEAST: Self = Self(134);
    pub const END_DISCUSSED_VUX_BEAST: Self = Self(134);
    pub const DISCUSSED_DESTRUCT_CODE: Self = Self(135);
    pub const END_DISCUSSED_DESTRUCT_CODE: Self = Self(135);
    pub const DISCUSSED_URQUAN_WARP: Self = Self(136);
    pub const END_DISCUSSED_URQUAN_WARP: Self = Self(136);
    pub const DISCUSSED_ARTIFACT_2: Self = Self(137);
    pub const END_DISCUSSED_ARTIFACT_2: Self = Self(137);
    pub const DISCUSSED_ARTIFACT_3: Self = Self(138);
    pub const END_DISCUSSED_ARTIFACT_3: Self = Self(138);
    pub const ATTACKED_DRUUGE: Self = Self(139);
    pub const END_ATTACKED_DRUUGE: Self = Self(139);
    pub const NEW_ALLIANCE_NAME: Self = Self(140);
    pub const END_NEW_ALLIANCE_NAME: Self = Self(141);
    pub const PORTAL_COUNTER: Self = Self(142);
    pub const END_PORTAL_COUNTER: Self = Self(145);
    pub const BURVIXESE_BROADCASTERS: Self = Self(146);
    pub const END_BURVIXESE_BROADCASTERS: Self = Self(146);
    pub const BURV_BROADCASTERS_ON_SHIP: Self = Self(147);
    pub const END_BURV_BROADCASTERS_ON_SHIP: Self = Self(147);
    pub const UTWIG_BOMB: Self = Self(148);
    pub const END_UTWIG_BOMB: Self = Self(148);
    pub const UTWIG_BOMB_ON_SHIP: Self = Self(149);
    pub const END_UTWIG_BOMB_ON_SHIP: Self = Self(149);
    pub const AQUA_HELIX: Self = Self(150);
    pub const END_AQUA_HELIX: Self = Self(150);
    pub const AQUA_HELIX_ON_SHIP: Self = Self(151);
    pub const END_AQUA_HELIX_ON_SHIP: Self = Self(151);
    pub const SUN_DEVICE: Self = Self(152);
    pub const END_SUN_DEVICE: Self = Self(152);
    pub const SUN_DEVICE_ON_SHIP: Self = Self(153);
    pub const END_SUN_DEVICE_ON_SHIP: Self = Self(153);
    pub const TAALO_PROTECTOR: Self = Self(154);
    pub const END_TAALO_PROTECTOR: Self = Self(154);
    pub const TAALO_PROTECTOR_ON_SHIP: Self = Self(155);
    pub const END_TAALO_PROTECTOR_ON_SHIP: Self = Self(155);
    pub const SHIP_VAULT_UNLOCKED: Self = Self(156);
    pub const END_SHIP_VAULT_UNLOCKED: Self = Self(156);
    pub const SYREEN_SHUTTLE: Self = Self(157);
    pub const END_SYREEN_SHUTTLE: Self = Self(157);
    pub const PORTAL_KEY: Self = Self(158);
    pub const END_PORTAL_KEY: Self = Self(158);
    pub const PORTAL_KEY_ON_SHIP: Self = Self(159);
    pub const END_PORTAL_KEY_ON_SHIP: Self = Self(159);
    pub const VUX_BEAST: Self = Self(160);
    pub const END_VUX_BEAST: Self = Self(160);
    pub const VUX_BEAST_ON_SHIP: Self = Self(161);
    pub const END_VUX_BEAST_ON_SHIP: Self = Self(161);
    pub const TALKING_PET: Self = Self(162);
    pub const END_TALKING_PET: Self = Self(162);
    pub const TALKING_PET_ON_SHIP: Self = Self(163);
    pub const END_TALKING_PET_ON_SHIP: Self = Self(163);
    pub const MOONBASE_ON_SHIP: Self = Self(164);
    pub const END_MOONBASE_ON_SHIP: Self = Self(164);
    pub const KOHR_AH_FRENZY: Self = Self(165);
    pub const END_KOHR_AH_FRENZY: Self = Self(165);
    pub const KOHR_AH_VISITS: Self = Self(166);
    pub const END_KOHR_AH_VISITS: Self = Self(167);
    pub const KOHR_AH_BYES: Self = Self(168);
    pub const END_KOHR_AH_BYES: Self = Self(168);
    pub const SLYLANDRO_HOME_VISITS: Self = Self(169);
    pub const END_SLYLANDRO_HOME_VISITS: Self = Self(171);
    pub const DESTRUCT_CODE_ON_SHIP: Self = Self(172);
    pub const END_DESTRUCT_CODE_ON_SHIP: Self = Self(172);
    pub const ILWRATH_VISITS: Self = Self(173);
    pub const END_ILWRATH_VISITS: Self = Self(175);
    pub const ILWRATH_DECEIVED: Self = Self(176);
    pub const END_ILWRATH_DECEIVED: Self = Self(176);
    pub const FLAGSHIP_CLOAKED: Self = Self(177);
    pub const END_FLAGSHIP_CLOAKED: Self = Self(177);
    pub const MYCON_VISITS: Self = Self(178);
    pub const END_MYCON_VISITS: Self = Self(180);
    pub const MYCON_HOME_VISITS: Self = Self(181);
    pub const END_MYCON_HOME_VISITS: Self = Self(183);
    pub const MYCON_AMBUSH: Self = Self(184);
    pub const END_MYCON_AMBUSH: Self = Self(184);
    pub const MYCON_FELL_FOR_AMBUSH: Self = Self(185);
    pub const END_MYCON_FELL_FOR_AMBUSH: Self = Self(185);
    pub const GLOBAL_FLAGS_AND_DATA: Self = Self(186);
    pub const END_GLOBAL_FLAGS_AND_DATA: Self = Self(193);
    pub const ORZ_VISITS: Self = Self(194);
    pub const END_ORZ_VISITS: Self = Self(196);
    pub const TAALO_VISITS: Self = Self(197);
    pub const END_TAALO_VISITS: Self = Self(199);
    pub const ORZ_MANNER: Self = Self(200);
    pub const END_ORZ_MANNER: Self = Self(201);
    pub const PROBE_EXHIBITED_BUG: Self = Self(202);
    pub const END_PROBE_EXHIBITED_BUG: Self = Self(202);
    pub const CLEAR_SPINDLE_ON_SHIP: Self = Self(203);
    pub const END_CLEAR_SPINDLE_ON_SHIP: Self = Self(203);
    pub const URQUAN_VISITS: Self = Self(204);
    pub const END_URQUAN_VISITS: Self = Self(206);
    pub const PLAYER_HYPNOTIZED: Self = Self(207);
    pub const END_PLAYER_HYPNOTIZED: Self = Self(207);
    pub const VUX_VISITS: Self = Self(208);
    pub const END_VUX_VISITS: Self = Self(210);
    pub const VUX_HOME_VISITS: Self = Self(211);
    pub const END_VUX_HOME_VISITS: Self = Self(213);
    pub const ZEX_VISITS: Self = Self(214);
    pub const END_ZEX_VISITS: Self = Self(216);
    pub const ZEX_IS_DEAD: Self = Self(217);
    pub const END_ZEX_IS_DEAD: Self = Self(217);
    pub const KNOW_ZEX_WANTS_MONSTER: Self = Self(218);
    pub const END_KNOW_ZEX_WANTS_MONSTER: Self = Self(218);
    pub const UTWIG_VISITS: Self = Self(219);
    pub const END_UTWIG_VISITS: Self = Self(221);
    pub const UTWIG_HOME_VISITS: Self = Self(222);
    pub const END_UTWIG_HOME_VISITS: Self = Self(224);
    pub const BOMB_VISITS: Self = Self(225);
    pub const END_BOMB_VISITS: Self = Self(227);
    pub const ULTRON_CONDITION: Self = Self(228);
    pub const END_ULTRON_CONDITION: Self = Self(230);
    pub const UTWIG_HAVE_ULTRON: Self = Self(231);
    pub const END_UTWIG_HAVE_ULTRON: Self = Self(231);
    pub const BOMB_UNPROTECTED: Self = Self(232);
    pub const END_BOMB_UNPROTECTED: Self = Self(232);
    pub const TAALO_UNPROTECTED: Self = Self(233);
    pub const END_TAALO_UNPROTECTED: Self = Self(233);
    pub const TALKING_PET_VISITS: Self = Self(234);
    pub const END_TALKING_PET_VISITS: Self = Self(236);
    pub const TALKING_PET_HOME_VISITS: Self = Self(237);
    pub const END_TALKING_PET_HOME_VISITS: Self = Self(239);
    pub const UMGAH_ZOMBIE_BLOBBIES: Self = Self(240);
    pub const END_UMGAH_ZOMBIE_BLOBBIES: Self = Self(240);
    pub const KNOW_UMGAH_ZOMBIES: Self = Self(241);
    pub const END_KNOW_UMGAH_ZOMBIES: Self = Self(241);
    pub const ARILOU_VISITS: Self = Self(242);
    pub const END_ARILOU_VISITS: Self = Self(244);
    pub const ARILOU_HOME_VISITS: Self = Self(245);
    pub const END_ARILOU_HOME_VISITS: Self = Self(247);
    pub const KNOW_ARILOU_WANT_WRECK: Self = Self(248);
    pub const END_KNOW_ARILOU_WANT_WRECK: Self = Self(248);
    pub const ARILOU_CHECKED_UMGAH: Self = Self(249);
    pub const END_ARILOU_CHECKED_UMGAH: Self = Self(250);
    pub const PORTAL_SPAWNER: Self = Self(251);
    pub const END_PORTAL_SPAWNER: Self = Self(251);
    pub const PORTAL_SPAWNER_ON_SHIP: Self = Self(252);
    pub const END_PORTAL_SPAWNER_ON_SHIP: Self = Self(252);
    pub const UMGAH_VISITS: Self = Self(253);
    pub const END_UMGAH_VISITS: Self = Self(255);
    pub const UMGAH_HOME_VISITS: Self = Self(256);
    pub const END_UMGAH_HOME_VISITS: Self = Self(258);
    pub const MET_NORMAL_UMGAH: Self = Self(259);
    pub const END_MET_NORMAL_UMGAH: Self = Self(259);
    pub const SYREEN_HOME_VISITS: Self = Self(260);
    pub const END_SYREEN_HOME_VISITS: Self = Self(262);
    pub const SYREEN_SHUTTLE_ON_SHIP: Self = Self(263);
    pub const END_SYREEN_SHUTTLE_ON_SHIP: Self = Self(263);
    pub const KNOW_SYREEN_VAULT: Self = Self(264);
    pub const END_KNOW_SYREEN_VAULT: Self = Self(264);
    pub const EGG_CASE0_ON_SHIP: Self = Self(265);
    pub const END_EGG_CASE0_ON_SHIP: Self = Self(265);
    pub const SUN_DEVICE_UNGUARDED: Self = Self(266);
    pub const END_SUN_DEVICE_UNGUARDED: Self = Self(266);
    pub const ROSY_SPHERE_ON_SHIP: Self = Self(267);
    pub const END_ROSY_SPHERE_ON_SHIP: Self = Self(267);
    pub const CHMMR_HOME_VISITS: Self = Self(268);
    pub const END_CHMMR_HOME_VISITS: Self = Self(270);
    pub const CHMMR_EMERGING: Self = Self(271);
    pub const END_CHMMR_EMERGING: Self = Self(271);
    pub const CHMMR_UNLEASHED: Self = Self(272);
    pub const END_CHMMR_UNLEASHED: Self = Self(272);
    pub const CHMMR_BOMB_STATE: Self = Self(273);
    pub const END_CHMMR_BOMB_STATE: Self = Self(274);
    pub const DRUUGE_DISCLAIMER: Self = Self(275);
    pub const END_DRUUGE_DISCLAIMER: Self = Self(275);
    pub const YEHAT_VISITS: Self = Self(276);
    pub const END_YEHAT_VISITS: Self = Self(278);
    pub const YEHAT_REBEL_VISITS: Self = Self(279);
    pub const END_YEHAT_REBEL_VISITS: Self = Self(281);
    pub const YEHAT_HOME_VISITS: Self = Self(282);
    pub const END_YEHAT_HOME_VISITS: Self = Self(284);
    pub const YEHAT_CIVIL_WAR: Self = Self(285);
    pub const END_YEHAT_CIVIL_WAR: Self = Self(285);
    pub const YEHAT_ABSORBED_PKUNK: Self = Self(286);
    pub const END_YEHAT_ABSORBED_PKUNK: Self = Self(286);
    pub const YEHAT_SHIP_MONTH: Self = Self(287);
    pub const END_YEHAT_SHIP_MONTH: Self = Self(290);
    pub const YEHAT_SHIP_DAY: Self = Self(291);
    pub const END_YEHAT_SHIP_DAY: Self = Self(295);
    pub const YEHAT_SHIP_YEAR: Self = Self(296);
    pub const END_YEHAT_SHIP_YEAR: Self = Self(300);
    pub const CLEAR_SPINDLE: Self = Self(301);
    pub const END_CLEAR_SPINDLE: Self = Self(301);
    pub const PKUNK_VISITS: Self = Self(302);
    pub const END_PKUNK_VISITS: Self = Self(304);
    pub const PKUNK_HOME_VISITS: Self = Self(305);
    pub const END_PKUNK_HOME_VISITS: Self = Self(307);
    pub const PKUNK_SHIP_MONTH: Self = Self(308);
    pub const END_PKUNK_SHIP_MONTH: Self = Self(311);
    pub const PKUNK_SHIP_DAY: Self = Self(312);
    pub const END_PKUNK_SHIP_DAY: Self = Self(316);
    pub const PKUNK_SHIP_YEAR: Self = Self(317);
    pub const END_PKUNK_SHIP_YEAR: Self = Self(321);
    pub const PKUNK_MISSION: Self = Self(322);
    pub const END_PKUNK_MISSION: Self = Self(324);
    pub const SUPOX_VISITS: Self = Self(325);
    pub const END_SUPOX_VISITS: Self = Self(327);
    pub const SUPOX_HOME_VISITS: Self = Self(328);
    pub const END_SUPOX_HOME_VISITS: Self = Self(330);
    pub const THRADD_VISITS: Self = Self(331);
    pub const END_THRADD_VISITS: Self = Self(333);
    pub const THRADD_HOME_VISITS: Self = Self(334);
    pub const END_THRADD_HOME_VISITS: Self = Self(336);
    pub const HELIX_VISITS: Self = Self(337);
    pub const END_HELIX_VISITS: Self = Self(339);
    pub const HELIX_UNPROTECTED: Self = Self(340);
    pub const END_HELIX_UNPROTECTED: Self = Self(340);
    pub const THRADD_CULTURE: Self = Self(341);
    pub const END_THRADD_CULTURE: Self = Self(342);
    pub const THRADD_MISSION: Self = Self(343);
    pub const END_THRADD_MISSION: Self = Self(345);
    pub const DRUUGE_VISITS: Self = Self(346);
    pub const END_DRUUGE_VISITS: Self = Self(348);
    pub const DRUUGE_HOME_VISITS: Self = Self(349);
    pub const END_DRUUGE_HOME_VISITS: Self = Self(351);
    pub const ROSY_SPHERE: Self = Self(352);
    pub const END_ROSY_SPHERE: Self = Self(352);
    pub const SCANNED_MAIDENS: Self = Self(353);
    pub const END_SCANNED_MAIDENS: Self = Self(353);
    pub const SCANNED_FRAGMENTS: Self = Self(354);
    pub const END_SCANNED_FRAGMENTS: Self = Self(354);
    pub const SCANNED_CASTER: Self = Self(355);
    pub const END_SCANNED_CASTER: Self = Self(355);
    pub const SCANNED_SPAWNER: Self = Self(356);
    pub const END_SCANNED_SPAWNER: Self = Self(356);
    pub const SCANNED_ULTRON: Self = Self(357);
    pub const END_SCANNED_ULTRON: Self = Self(357);
    pub const ZOQFOT_INFO: Self = Self(358);
    pub const END_ZOQFOT_INFO: Self = Self(359);
    pub const ZOQFOT_HOSTILE: Self = Self(360);
    pub const END_ZOQFOT_HOSTILE: Self = Self(360);
    pub const ZOQFOT_HOME_VISITS: Self = Self(361);
    pub const END_ZOQFOT_HOME_VISITS: Self = Self(363);
    pub const MET_ZOQFOT: Self = Self(364);
    pub const END_MET_ZOQFOT: Self = Self(364);
    pub const ZOQFOT_DISTRESS: Self = Self(365);
    pub const END_ZOQFOT_DISTRESS: Self = Self(366);
    pub const EGG_CASE1_ON_SHIP: Self = Self(367);
    pub const END_EGG_CASE1_ON_SHIP: Self = Self(367);
    pub const EGG_CASE2_ON_SHIP: Self = Self(368);
    pub const END_EGG_CASE2_ON_SHIP: Self = Self(368);
    pub const MYCON_SUN_VISITS: Self = Self(369);
    pub const END_MYCON_SUN_VISITS: Self = Self(371);
    pub const ORZ_HOME_VISITS: Self = Self(372);
    pub const END_ORZ_HOME_VISITS: Self = Self(374);
    pub const MELNORME_FUEL_PROCEDURE: Self = Self(375);
    pub const END_MELNORME_FUEL_PROCEDURE: Self = Self(375);
    pub const MELNORME_TECH_PROCEDURE: Self = Self(376);
    pub const END_MELNORME_TECH_PROCEDURE: Self = Self(376);
    pub const MELNORME_INFO_PROCEDURE: Self = Self(377);
    pub const END_MELNORME_INFO_PROCEDURE: Self = Self(377);
    pub const MELNORME_TECH_STACK: Self = Self(378);
    pub const END_MELNORME_TECH_STACK: Self = Self(381);
    pub const MELNORME_EVENTS_INFO_STACK: Self = Self(382);
    pub const END_MELNORME_EVENTS_INFO_STACK: Self = Self(386);
    pub const MELNORME_ALIEN_INFO_STACK: Self = Self(387);
    pub const END_MELNORME_ALIEN_INFO_STACK: Self = Self(391);
    pub const MELNORME_HISTORY_INFO_STACK: Self = Self(392);
    pub const END_MELNORME_HISTORY_INFO_STACK: Self = Self(396);
    pub const RAINBOW_WORLD0: Self = Self(397);
    pub const END_RAINBOW_WORLD0: Self = Self(404);
    pub const RAINBOW_WORLD1: Self = Self(405);
    pub const END_RAINBOW_WORLD1: Self = Self(406);
    pub const MELNORME_RAINBOW_COUNT: Self = Self(407);
    pub const END_MELNORME_RAINBOW_COUNT: Self = Self(410);
    pub const USED_BROADCASTER: Self = Self(411);
    pub const END_USED_BROADCASTER: Self = Self(411);
    pub const BROADCASTER_RESPONSE: Self = Self(412);
    pub const END_BROADCASTER_RESPONSE: Self = Self(412);
    pub const IMPROVED_LANDER_SPEED: Self = Self(413);
    pub const END_IMPROVED_LANDER_SPEED: Self = Self(413);
    pub const IMPROVED_LANDER_CARGO: Self = Self(414);
    pub const END_IMPROVED_LANDER_CARGO: Self = Self(414);
    pub const IMPROVED_LANDER_SHOT: Self = Self(415);
    pub const END_IMPROVED_LANDER_SHOT: Self = Self(415);
    pub const MET_ORZ_BEFORE: Self = Self(416);
    pub const END_MET_ORZ_BEFORE: Self = Self(416);
    pub const YEHAT_REBEL_TOLD_PKUNK: Self = Self(417);
    pub const END_YEHAT_REBEL_TOLD_PKUNK: Self = Self(417);
    pub const PLAYER_HAD_SEX: Self = Self(418);
    pub const END_PLAYER_HAD_SEX: Self = Self(418);
    pub const UMGAH_BROADCASTERS_ON_SHIP: Self = Self(419);
    pub const END_UMGAH_BROADCASTERS_ON_SHIP: Self = Self(419);
    pub const LIGHT_MINERAL_LOAD: Self = Self(420);
    pub const END_LIGHT_MINERAL_LOAD: Self = Self(422);
    pub const MEDIUM_MINERAL_LOAD: Self = Self(423);
    pub const END_MEDIUM_MINERAL_LOAD: Self = Self(425);
    pub const HEAVY_MINERAL_LOAD: Self = Self(426);
    pub const END_HEAVY_MINERAL_LOAD: Self = Self(428);
    pub const STARBASE_BULLETS0: Self = Self(429);
    pub const END_STARBASE_BULLETS0: Self = Self(436);
    pub const STARBASE_BULLETS1: Self = Self(437);
    pub const END_STARBASE_BULLETS1: Self = Self(444);
    pub const STARBASE_BULLETS2: Self = Self(445);
    pub const END_STARBASE_BULLETS2: Self = Self(452);
    pub const STARBASE_BULLETS3: Self = Self(453);
    pub const END_STARBASE_BULLETS3: Self = Self(460);
    pub const STARBASE_MONTH: Self = Self(461);
    pub const END_STARBASE_MONTH: Self = Self(464);
    pub const STARBASE_DAY: Self = Self(465);
    pub const END_STARBASE_DAY: Self = Self(469);
    pub const CREW_SOLD_TO_DRUUGE0: Self = Self(470);
    pub const END_CREW_SOLD_TO_DRUUGE0: Self = Self(477);
    pub const CREW_PURCHASED0: Self = Self(478);
    pub const END_CREW_PURCHASED0: Self = Self(485);
    pub const CREW_PURCHASED1: Self = Self(486);
    pub const END_CREW_PURCHASED1: Self = Self(493);
    pub const URQUAN_PROTECTING_SAMATRA: Self = Self(494);
    pub const END_URQUAN_PROTECTING_SAMATRA: Self = Self(494);
    pub const THRADDASH_BODY_COUNT: Self = Self(495);
    pub const END_THRADDASH_BODY_COUNT: Self = Self(499);
    pub const UTWIG_SUPOX_MISSION: Self = Self(500);
    pub const END_UTWIG_SUPOX_MISSION: Self = Self(502);
    pub const SPATHI_INFO: Self = Self(503);
    pub const END_SPATHI_INFO: Self = Self(505);
    pub const ILWRATH_INFO: Self = Self(506);
    pub const END_ILWRATH_INFO: Self = Self(507);
    pub const ILWRATH_GODS_SPOKEN: Self = Self(508);
    pub const END_ILWRATH_GODS_SPOKEN: Self = Self(511);
    pub const ILWRATH_WORSHIP: Self = Self(512);
    pub const END_ILWRATH_WORSHIP: Self = Self(513);
    pub const ILWRATH_FIGHT_THRADDASH: Self = Self(514);
    pub const END_ILWRATH_FIGHT_THRADDASH: Self = Self(514);
    pub const READY_TO_CONFUSE_URQUAN: Self = Self(515);
    pub const END_READY_TO_CONFUSE_URQUAN: Self = Self(515);
    pub const URQUAN_HYPNO_VISITS: Self = Self(516);
    pub const END_URQUAN_HYPNO_VISITS: Self = Self(516);
    pub const MENTIONED_PET_COMPULSION: Self = Self(517);
    pub const END_MENTIONED_PET_COMPULSION: Self = Self(517);
    pub const URQUAN_INFO: Self = Self(518);
    pub const END_URQUAN_INFO: Self = Self(519);
    pub const KNOW_URQUAN_STORY: Self = Self(520);
    pub const END_KNOW_URQUAN_STORY: Self = Self(521);
    pub const MYCON_INFO: Self = Self(522);
    pub const END_MYCON_INFO: Self = Self(525);
    pub const MYCON_RAMBLE: Self = Self(526);
    pub const END_MYCON_RAMBLE: Self = Self(530);
    pub const KNOW_ABOUT_SHATTERED: Self = Self(531);
    pub const END_KNOW_ABOUT_SHATTERED: Self = Self(532);
    pub const MYCON_INSULTS: Self = Self(533);
    pub const END_MYCON_INSULTS: Self = Self(535);
    pub const MYCON_KNOW_AMBUSH: Self = Self(536);
    pub const END_MYCON_KNOW_AMBUSH: Self = Self(536);
    pub const SYREEN_INFO: Self = Self(537);
    pub const END_SYREEN_INFO: Self = Self(538);
    pub const KNOW_SYREEN_WORLD_SHATTERED: Self = Self(539);
    pub const END_KNOW_SYREEN_WORLD_SHATTERED: Self = Self(539);
    pub const SYREEN_KNOW_ABOUT_MYCON: Self = Self(540);
    pub const END_SYREEN_KNOW_ABOUT_MYCON: Self = Self(540);
    pub const TALKING_PET_INFO: Self = Self(541);
    pub const END_TALKING_PET_INFO: Self = Self(543);
    pub const TALKING_PET_SUGGESTIONS: Self = Self(544);
    pub const END_TALKING_PET_SUGGESTIONS: Self = Self(546);
    pub const LEARNED_TALKING_PET: Self = Self(547);
    pub const END_LEARNED_TALKING_PET: Self = Self(547);
    pub const DNYARRI_LIED: Self = Self(548);
    pub const END_DNYARRI_LIED: Self = Self(548);
    pub const SHIP_TO_COMPEL: Self = Self(549);
    pub const END_SHIP_TO_COMPEL: Self = Self(549);
    pub const ORZ_GENERAL_INFO: Self = Self(550);
    pub const END_ORZ_GENERAL_INFO: Self = Self(551);
    pub const ORZ_PERSONAL_INFO: Self = Self(552);
    pub const END_ORZ_PERSONAL_INFO: Self = Self(554);
    pub const ORZ_ANDRO_STATE: Self = Self(555);
    pub const END_ORZ_ANDRO_STATE: Self = Self(556);
    pub const REFUSED_ORZ_ALLIANCE: Self = Self(557);
    pub const END_REFUSED_ORZ_ALLIANCE: Self = Self(557);
    pub const PKUNK_MANNER: Self = Self(558);
    pub const END_PKUNK_MANNER: Self = Self(559);
    pub const PKUNK_ON_THE_MOVE: Self = Self(560);
    pub const END_PKUNK_ON_THE_MOVE: Self = Self(560);
    pub const PKUNK_FLEET: Self = Self(561);
    pub const END_PKUNK_FLEET: Self = Self(562);
    pub const PKUNK_MIGRATE: Self = Self(563);
    pub const END_PKUNK_MIGRATE: Self = Self(564);
    pub const PKUNK_RETURN: Self = Self(565);
    pub const END_PKUNK_RETURN: Self = Self(565);
    pub const PKUNK_WORRY: Self = Self(566);
    pub const END_PKUNK_WORRY: Self = Self(567);
    pub const PKUNK_INFO: Self = Self(568);
    pub const END_PKUNK_INFO: Self = Self(570);
    pub const PKUNK_WAR: Self = Self(571);
    pub const END_PKUNK_WAR: Self = Self(572);
    pub const PKUNK_FORTUNE: Self = Self(573);
    pub const END_PKUNK_FORTUNE: Self = Self(575);
    pub const PKUNK_MIGRATE_VISITS: Self = Self(576);
    pub const END_PKUNK_MIGRATE_VISITS: Self = Self(578);
    pub const PKUNK_REASONS: Self = Self(579);
    pub const END_PKUNK_REASONS: Self = Self(582);
    pub const PKUNK_SWITCH: Self = Self(583);
    pub const END_PKUNK_SWITCH: Self = Self(583);
    pub const PKUNK_SENSE_VICTOR: Self = Self(584);
    pub const END_PKUNK_SENSE_VICTOR: Self = Self(584);
    pub const KOHR_AH_REASONS: Self = Self(585);
    pub const END_KOHR_AH_REASONS: Self = Self(586);
    pub const KOHR_AH_PLEAD: Self = Self(587);
    pub const END_KOHR_AH_PLEAD: Self = Self(588);
    pub const KOHR_AH_INFO: Self = Self(589);
    pub const END_KOHR_AH_INFO: Self = Self(590);
    pub const KNOW_KOHR_AH_STORY: Self = Self(591);
    pub const END_KNOW_KOHR_AH_STORY: Self = Self(592);
    pub const KOHR_AH_SENSES_EVIL: Self = Self(593);
    pub const END_KOHR_AH_SENSES_EVIL: Self = Self(593);
    pub const URQUAN_SENSES_EVIL: Self = Self(594);
    pub const END_URQUAN_SENSES_EVIL: Self = Self(594);
    pub const SLYLANDRO_PROBE_VISITS: Self = Self(595);
    pub const END_SLYLANDRO_PROBE_VISITS: Self = Self(597);
    pub const SLYLANDRO_PROBE_THREAT: Self = Self(598);
    pub const END_SLYLANDRO_PROBE_THREAT: Self = Self(599);
    pub const SLYLANDRO_PROBE_WRONG: Self = Self(600);
    pub const END_SLYLANDRO_PROBE_WRONG: Self = Self(601);
    pub const SLYLANDRO_PROBE_ID: Self = Self(602);
    pub const END_SLYLANDRO_PROBE_ID: Self = Self(603);
    pub const SLYLANDRO_PROBE_INFO: Self = Self(604);
    pub const END_SLYLANDRO_PROBE_INFO: Self = Self(605);
    pub const SLYLANDRO_PROBE_EXIT: Self = Self(606);
    pub const END_SLYLANDRO_PROBE_EXIT: Self = Self(607);
    pub const UMGAH_HOSTILE: Self = Self(608);
    pub const END_UMGAH_HOSTILE: Self = Self(608);
    pub const UMGAH_EVIL_BLOBBIES: Self = Self(609);
    pub const END_UMGAH_EVIL_BLOBBIES: Self = Self(609);
    pub const UMGAH_MENTIONED_TRICKS: Self = Self(610);
    pub const END_UMGAH_MENTIONED_TRICKS: Self = Self(611);
    pub const BOMB_CARRIER: Self = Self(612);
    pub const END_BOMB_CARRIER: Self = Self(612);
    pub const THRADD_MANNER: Self = Self(613);
    pub const END_THRADD_MANNER: Self = Self(613);
    pub const THRADD_INTRO: Self = Self(614);
    pub const END_THRADD_INTRO: Self = Self(615);
    pub const THRADD_DEMEANOR: Self = Self(616);
    pub const END_THRADD_DEMEANOR: Self = Self(618);
    pub const THRADD_INFO: Self = Self(619);
    pub const END_THRADD_INFO: Self = Self(620);
    pub const THRADD_BODY_LEVEL: Self = Self(621);
    pub const END_THRADD_BODY_LEVEL: Self = Self(622);
    pub const THRADD_MISSION_VISITS: Self = Self(623);
    pub const END_THRADD_MISSION_VISITS: Self = Self(623);
    pub const THRADD_STACK_1: Self = Self(624);
    pub const END_THRADD_STACK_1: Self = Self(626);
    pub const THRADD_HOSTILE_STACK_2: Self = Self(627);
    pub const END_THRADD_HOSTILE_STACK_2: Self = Self(627);
    pub const THRADD_HOSTILE_STACK_3: Self = Self(628);
    pub const END_THRADD_HOSTILE_STACK_3: Self = Self(628);
    pub const THRADD_HOSTILE_STACK_4: Self = Self(629);
    pub const END_THRADD_HOSTILE_STACK_4: Self = Self(629);
    pub const THRADD_HOSTILE_STACK_5: Self = Self(630);
    pub const END_THRADD_HOSTILE_STACK_5: Self = Self(630);
    pub const CHMMR_STACK: Self = Self(631);
    pub const END_CHMMR_STACK: Self = Self(632);
    pub const ARILOU_MANNER: Self = Self(633);
    pub const END_ARILOU_MANNER: Self = Self(634);
    pub const NO_PORTAL_VISITS: Self = Self(635);
    pub const END_NO_PORTAL_VISITS: Self = Self(635);
    pub const ARILOU_STACK_1: Self = Self(636);
    pub const END_ARILOU_STACK_1: Self = Self(637);
    pub const ARILOU_STACK_2: Self = Self(638);
    pub const END_ARILOU_STACK_2: Self = Self(638);
    pub const ARILOU_STACK_3: Self = Self(639);
    pub const END_ARILOU_STACK_3: Self = Self(640);
    pub const ARILOU_STACK_4: Self = Self(641);
    pub const END_ARILOU_STACK_4: Self = Self(641);
    pub const ARILOU_STACK_5: Self = Self(642);
    pub const END_ARILOU_STACK_5: Self = Self(643);
    pub const ARILOU_INFO: Self = Self(644);
    pub const END_ARILOU_INFO: Self = Self(645);
    pub const ARILOU_HINTS: Self = Self(646);
    pub const END_ARILOU_HINTS: Self = Self(647);
    pub const DRUUGE_MANNER: Self = Self(648);
    pub const END_DRUUGE_MANNER: Self = Self(648);
    pub const DRUUGE_SPACE_INFO: Self = Self(649);
    pub const END_DRUUGE_SPACE_INFO: Self = Self(650);
    pub const DRUUGE_HOME_INFO: Self = Self(651);
    pub const END_DRUUGE_HOME_INFO: Self = Self(652);
    pub const DRUUGE_SALVAGE: Self = Self(653);
    pub const END_DRUUGE_SALVAGE: Self = Self(653);
    pub const KNOW_DRUUGE_SLAVERS: Self = Self(654);
    pub const END_KNOW_DRUUGE_SLAVERS: Self = Self(655);
    pub const FRAGMENTS_BOUGHT: Self = Self(656);
    pub const END_FRAGMENTS_BOUGHT: Self = Self(657);
    pub const ZEX_STACK_1: Self = Self(658);
    pub const END_ZEX_STACK_1: Self = Self(659);
    pub const ZEX_STACK_2: Self = Self(660);
    pub const END_ZEX_STACK_2: Self = Self(661);
    pub const ZEX_STACK_3: Self = Self(662);
    pub const END_ZEX_STACK_3: Self = Self(663);
    pub const VUX_INFO: Self = Self(664);
    pub const END_VUX_INFO: Self = Self(665);
    pub const VUX_STACK_1: Self = Self(666);
    pub const END_VUX_STACK_1: Self = Self(669);
    pub const VUX_STACK_2: Self = Self(670);
    pub const END_VUX_STACK_2: Self = Self(671);
    pub const VUX_STACK_3: Self = Self(672);
    pub const END_VUX_STACK_3: Self = Self(673);
    pub const VUX_STACK_4: Self = Self(674);
    pub const END_VUX_STACK_4: Self = Self(675);
    pub const SHOFIXTI_STACK4: Self = Self(676);
    pub const END_SHOFIXTI_STACK4: Self = Self(677);
    pub const YEHAT_REBEL_INFO: Self = Self(678);
    pub const END_YEHAT_REBEL_INFO: Self = Self(680);
    pub const YEHAT_ROYALIST_INFO: Self = Self(681);
    pub const END_YEHAT_ROYALIST_INFO: Self = Self(681);
    pub const YEHAT_ROYALIST_TOLD_PKUNK: Self = Self(682);
    pub const END_YEHAT_ROYALIST_TOLD_PKUNK: Self = Self(682);
    pub const NO_YEHAT_ALLY_HOME: Self = Self(683);
    pub const END_NO_YEHAT_ALLY_HOME: Self = Self(683);
    pub const NO_YEHAT_HELP_HOME: Self = Self(684);
    pub const END_NO_YEHAT_HELP_HOME: Self = Self(684);
    pub const NO_YEHAT_INFO: Self = Self(685);
    pub const END_NO_YEHAT_INFO: Self = Self(685);
    pub const NO_YEHAT_ALLY_SPACE: Self = Self(686);
    pub const END_NO_YEHAT_ALLY_SPACE: Self = Self(687);
    pub const NO_YEHAT_HELP_SPACE: Self = Self(688);
    pub const END_NO_YEHAT_HELP_SPACE: Self = Self(689);
    pub const ZOQFOT_KNOW_MASK: Self = Self(690);
    pub const END_ZOQFOT_KNOW_MASK: Self = Self(693);
    pub const SUPOX_HOSTILE: Self = Self(694);
    pub const END_SUPOX_HOSTILE: Self = Self(694);
    pub const SUPOX_INFO: Self = Self(695);
    pub const END_SUPOX_INFO: Self = Self(695);
    pub const SUPOX_WAR_NEWS: Self = Self(696);
    pub const END_SUPOX_WAR_NEWS: Self = Self(697);
    pub const SUPOX_ULTRON_HELP: Self = Self(698);
    pub const END_SUPOX_ULTRON_HELP: Self = Self(698);
    pub const SUPOX_STACK1: Self = Self(699);
    pub const END_SUPOX_STACK1: Self = Self(701);
    pub const SUPOX_STACK2: Self = Self(702);
    pub const END_SUPOX_STACK2: Self = Self(703);
    pub const UTWIG_HOSTILE: Self = Self(704);
    pub const END_UTWIG_HOSTILE: Self = Self(704);
    pub const UTWIG_INFO: Self = Self(705);
    pub const END_UTWIG_INFO: Self = Self(705);
    pub const UTWIG_WAR_NEWS: Self = Self(706);
    pub const END_UTWIG_WAR_NEWS: Self = Self(707);
    pub const UTWIG_STACK1: Self = Self(708);
    pub const END_UTWIG_STACK1: Self = Self(710);
    pub const UTWIG_STACK2: Self = Self(711);
    pub const END_UTWIG_STACK2: Self = Self(712);
    pub const BOMB_INFO: Self = Self(713);
    pub const END_BOMB_INFO: Self = Self(713);
    pub const BOMB_STACK1: Self = Self(714);
    pub const END_BOMB_STACK1: Self = Self(715);
    pub const BOMB_STACK2: Self = Self(716);
    pub const END_BOMB_STACK2: Self = Self(717);
    pub const SLYLANDRO_KNOW_BROKEN: Self = Self(718);
    pub const END_SLYLANDRO_KNOW_BROKEN: Self = Self(718);
    pub const PLAYER_KNOWS_PROBE: Self = Self(719);
    pub const END_PLAYER_KNOWS_PROBE: Self = Self(719);
    pub const PLAYER_KNOWS_PROGRAM: Self = Self(720);
    pub const END_PLAYER_KNOWS_PROGRAM: Self = Self(720);
    pub const PLAYER_KNOWS_EFFECTS: Self = Self(721);
    pub const END_PLAYER_KNOWS_EFFECTS: Self = Self(721);
    pub const PLAYER_KNOWS_PRIORITY: Self = Self(722);
    pub const END_PLAYER_KNOWS_PRIORITY: Self = Self(722);
    pub const SLYLANDRO_STACK1: Self = Self(723);
    pub const END_SLYLANDRO_STACK1: Self = Self(725);
    pub const SLYLANDRO_STACK2: Self = Self(726);
    pub const END_SLYLANDRO_STACK2: Self = Self(726);
    pub const SLYLANDRO_STACK3: Self = Self(727);
    pub const END_SLYLANDRO_STACK3: Self = Self(728);
    pub const SLYLANDRO_STACK4: Self = Self(729);
    pub const END_SLYLANDRO_STACK4: Self = Self(730);
    pub const SLYLANDRO_STACK5: Self = Self(731);
    pub const END_SLYLANDRO_STACK5: Self = Self(731);
    pub const SLYLANDRO_STACK6: Self = Self(732);
    pub const END_SLYLANDRO_STACK6: Self = Self(732);
    pub const SLYLANDRO_STACK7: Self = Self(733);
    pub const END_SLYLANDRO_STACK7: Self = Self(734);
    pub const SLYLANDRO_STACK8: Self = Self(735);
    pub const END_SLYLANDRO_STACK8: Self = Self(736);
    pub const SLYLANDRO_STACK9: Self = Self(737);
    pub const END_SLYLANDRO_STACK9: Self = Self(738);
    pub const SLYLANDRO_KNOW_EARTH: Self = Self(739);
    pub const END_SLYLANDRO_KNOW_EARTH: Self = Self(739);
    pub const SLYLANDRO_KNOW_EXPLORE: Self = Self(740);
    pub const END_SLYLANDRO_KNOW_EXPLORE: Self = Self(740);
    pub const SLYLANDRO_KNOW_GATHER: Self = Self(741);
    pub const END_SLYLANDRO_KNOW_GATHER: Self = Self(741);
    pub const SLYLANDRO_KNOW_URQUAN: Self = Self(742);
    pub const END_SLYLANDRO_KNOW_URQUAN: Self = Self(743);
    pub const RECALL_VISITS: Self = Self(744);
    pub const END_RECALL_VISITS: Self = Self(745);
    pub const SLYLANDRO_MULTIPLIER: Self = Self(746);
    pub const END_SLYLANDRO_MULTIPLIER: Self = Self(748);
    pub const KNOW_SPATHI_QUEST: Self = Self(749);
    pub const END_KNOW_SPATHI_QUEST: Self = Self(749);
    pub const KNOW_SPATHI_EVIL: Self = Self(750);
    pub const END_KNOW_SPATHI_EVIL: Self = Self(750);
    pub const BATTLE_PLANET: Self = Self(751);
    pub const END_BATTLE_PLANET: Self = Self(758);
    pub const ESCAPE_COUNTER: Self = Self(759);
    pub const END_ESCAPE_COUNTER: Self = Self(766);
    pub const CREW_SOLD_TO_DRUUGE1: Self = Self(767);
    pub const END_CREW_SOLD_TO_DRUUGE1: Self = Self(774);
    pub const PKUNK_DONE_WAR: Self = Self(775);
    pub const END_PKUNK_DONE_WAR: Self = Self(775);
    pub const SYREEN_STACK0: Self = Self(776);
    pub const END_SYREEN_STACK0: Self = Self(777);
    pub const SYREEN_STACK1: Self = Self(778);
    pub const END_SYREEN_STACK1: Self = Self(779);
    pub const SYREEN_STACK2: Self = Self(780);
    pub const END_SYREEN_STACK2: Self = Self(781);
    pub const REFUSED_ULTRON_AT_BOMB: Self = Self(782);
    pub const END_REFUSED_ULTRON_AT_BOMB: Self = Self(782);
    pub const NO_TRICK_AT_SUN: Self = Self(783);
    pub const END_NO_TRICK_AT_SUN: Self = Self(783);
    pub const SPATHI_STACK0: Self = Self(784);
    pub const END_SPATHI_STACK0: Self = Self(785);
    pub const SPATHI_STACK1: Self = Self(786);
    pub const END_SPATHI_STACK1: Self = Self(786);
    pub const SPATHI_STACK2: Self = Self(787);
    pub const END_SPATHI_STACK2: Self = Self(787);
    pub const ORZ_STACK0: Self = Self(788);
    pub const END_ORZ_STACK0: Self = Self(788);
    pub const ORZ_STACK1: Self = Self(789);
    pub const END_ORZ_STACK1: Self = Self(789);
    pub const SHOFIXTI_GRPOFFS0: Self = Self(790);
    pub const END_SHOFIXTI_GRPOFFS0: Self = Self(797);
    pub const SHOFIXTI_GRPOFFS1: Self = Self(798);
    pub const END_SHOFIXTI_GRPOFFS1: Self = Self(805);
    pub const SHOFIXTI_GRPOFFS2: Self = Self(806);
    pub const END_SHOFIXTI_GRPOFFS2: Self = Self(813);
    pub const SHOFIXTI_GRPOFFS3: Self = Self(814);
    pub const END_SHOFIXTI_GRPOFFS3: Self = Self(821);
    pub const ZOQFOT_GRPOFFS0: Self = Self(822);
    pub const END_ZOQFOT_GRPOFFS0: Self = Self(829);
    pub const ZOQFOT_GRPOFFS1: Self = Self(830);
    pub const END_ZOQFOT_GRPOFFS1: Self = Self(837);
    pub const ZOQFOT_GRPOFFS2: Self = Self(838);
    pub const END_ZOQFOT_GRPOFFS2: Self = Self(845);
    pub const ZOQFOT_GRPOFFS3: Self = Self(846);
    pub const END_ZOQFOT_GRPOFFS3: Self = Self(853);
    pub const MELNORME0_GRPOFFS0: Self = Self(854);
    pub const END_MELNORME0_GRPOFFS0: Self = Self(861);
    pub const MELNORME0_GRPOFFS1: Self = Self(862);
    pub const END_MELNORME0_GRPOFFS1: Self = Self(869);
    pub const MELNORME0_GRPOFFS2: Self = Self(870);
    pub const END_MELNORME0_GRPOFFS2: Self = Self(877);
    pub const MELNORME0_GRPOFFS3: Self = Self(878);
    pub const END_MELNORME0_GRPOFFS3: Self = Self(885);
    pub const MELNORME1_GRPOFFS0: Self = Self(886);
    pub const END_MELNORME1_GRPOFFS0: Self = Self(893);
    pub const MELNORME1_GRPOFFS1: Self = Self(894);
    pub const END_MELNORME1_GRPOFFS1: Self = Self(901);
    pub const MELNORME1_GRPOFFS2: Self = Self(902);
    pub const END_MELNORME1_GRPOFFS2: Self = Self(909);
    pub const MELNORME1_GRPOFFS3: Self = Self(910);
    pub const END_MELNORME1_GRPOFFS3: Self = Self(917);
    pub const MELNORME2_GRPOFFS0: Self = Self(918);
    pub const END_MELNORME2_GRPOFFS0: Self = Self(925);
    pub const MELNORME2_GRPOFFS1: Self = Self(926);
    pub const END_MELNORME2_GRPOFFS1: Self = Self(933);
    pub const MELNORME2_GRPOFFS2: Self = Self(934);
    pub const END_MELNORME2_GRPOFFS2: Self = Self(941);
    pub const MELNORME2_GRPOFFS3: Self = Self(942);
    pub const END_MELNORME2_GRPOFFS3: Self = Self(949);
    pub const MELNORME3_GRPOFFS0: Self = Self(950);
    pub const END_MELNORME3_GRPOFFS0: Self = Self(957);
    pub const MELNORME3_GRPOFFS1: Self = Self(958);
    pub const END_MELNORME3_GRPOFFS1: Self = Self(965);
    pub const MELNORME3_GRPOFFS2: Self = Self(966);
    pub const END_MELNORME3_GRPOFFS2: Self = Self(973);
    pub const MELNORME3_GRPOFFS3: Self = Self(974);
    pub const END_MELNORME3_GRPOFFS3: Self = Self(981);
    pub const MELNORME4_GRPOFFS0: Self = Self(982);
    pub const END_MELNORME4_GRPOFFS0: Self = Self(989);
    pub const MELNORME4_GRPOFFS1: Self = Self(990);
    pub const END_MELNORME4_GRPOFFS1: Self = Self(997);
    pub const MELNORME4_GRPOFFS2: Self = Self(998);
    pub const END_MELNORME4_GRPOFFS2: Self = Self(1005);
    pub const MELNORME4_GRPOFFS3: Self = Self(1006);
    pub const END_MELNORME4_GRPOFFS3: Self = Self(1013);
    pub const MELNORME5_GRPOFFS0: Self = Self(1014);
    pub const END_MELNORME5_GRPOFFS0: Self = Self(1021);
    pub const MELNORME5_GRPOFFS1: Self = Self(1022);
    pub const END_MELNORME5_GRPOFFS1: Self = Self(1029);
    pub const MELNORME5_GRPOFFS2: Self = Self(1030);
    pub const END_MELNORME5_GRPOFFS2: Self = Self(1037);
    pub const MELNORME5_GRPOFFS3: Self = Self(1038);
    pub const END_MELNORME5_GRPOFFS3: Self = Self(1045);
    pub const MELNORME6_GRPOFFS0: Self = Self(1046);
    pub const END_MELNORME6_GRPOFFS0: Self = Self(1053);
    pub const MELNORME6_GRPOFFS1: Self = Self(1054);
    pub const END_MELNORME6_GRPOFFS1: Self = Self(1061);
    pub const MELNORME6_GRPOFFS2: Self = Self(1062);
    pub const END_MELNORME6_GRPOFFS2: Self = Self(1069);
    pub const MELNORME6_GRPOFFS3: Self = Self(1070);
    pub const END_MELNORME6_GRPOFFS3: Self = Self(1077);
    pub const MELNORME7_GRPOFFS0: Self = Self(1078);
    pub const END_MELNORME7_GRPOFFS0: Self = Self(1085);
    pub const MELNORME7_GRPOFFS1: Self = Self(1086);
    pub const END_MELNORME7_GRPOFFS1: Self = Self(1093);
    pub const MELNORME7_GRPOFFS2: Self = Self(1094);
    pub const END_MELNORME7_GRPOFFS2: Self = Self(1101);
    pub const MELNORME7_GRPOFFS3: Self = Self(1102);
    pub const END_MELNORME7_GRPOFFS3: Self = Self(1109);
    pub const MELNORME8_GRPOFFS0: Self = Self(1110);
    pub const END_MELNORME8_GRPOFFS0: Self = Self(1117);
    pub const MELNORME8_GRPOFFS1: Self = Self(1118);
    pub const END_MELNORME8_GRPOFFS1: Self = Self(1125);
    pub const MELNORME8_GRPOFFS2: Self = Self(1126);
    pub const END_MELNORME8_GRPOFFS2: Self = Self(1133);
    pub const MELNORME8_GRPOFFS3: Self = Self(1134);
    pub const END_MELNORME8_GRPOFFS3: Self = Self(1141);
    pub const URQUAN_PROBE_GRPOFFS0: Self = Self(1142);
    pub const END_URQUAN_PROBE_GRPOFFS0: Self = Self(1149);
    pub const URQUAN_PROBE_GRPOFFS1: Self = Self(1150);
    pub const END_URQUAN_PROBE_GRPOFFS1: Self = Self(1157);
    pub const URQUAN_PROBE_GRPOFFS2: Self = Self(1158);
    pub const END_URQUAN_PROBE_GRPOFFS2: Self = Self(1165);
    pub const URQUAN_PROBE_GRPOFFS3: Self = Self(1166);
    pub const END_URQUAN_PROBE_GRPOFFS3: Self = Self(1173);
    pub const COLONY_GRPOFFS0: Self = Self(1174);
    pub const END_COLONY_GRPOFFS0: Self = Self(1181);
    pub const COLONY_GRPOFFS1: Self = Self(1182);
    pub const END_COLONY_GRPOFFS1: Self = Self(1189);
    pub const COLONY_GRPOFFS2: Self = Self(1190);
    pub const END_COLONY_GRPOFFS2: Self = Self(1197);
    pub const COLONY_GRPOFFS3: Self = Self(1198);
    pub const END_COLONY_GRPOFFS3: Self = Self(1205);
    pub const SAMATRA_GRPOFFS0: Self = Self(1206);
    pub const END_SAMATRA_GRPOFFS0: Self = Self(1213);
    pub const SAMATRA_GRPOFFS1: Self = Self(1214);
    pub const END_SAMATRA_GRPOFFS1: Self = Self(1221);
    pub const SAMATRA_GRPOFFS2: Self = Self(1222);
    pub const END_SAMATRA_GRPOFFS2: Self = Self(1229);
    pub const SAMATRA_GRPOFFS3: Self = Self(1230);
    pub const END_SAMATRA_GRPOFFS3: Self = Self(1237);
    pub const NUM_GAME_STATE_BITS: Self = Self(1238);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_9(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_9 {
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_10(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_10 {
    pub const DWARF_STAR: Self = Self(0);
    pub const GIANT_STAR: Self = Self(1);
    pub const SUPER_GIANT_STAR: Self = Self(2);
    pub const NUM_STAR_TYPES: Self = Self(3);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_11(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_11 {
    pub const BLUE_BODY: Self = Self(0);
    pub const GREEN_BODY: Self = Self(1);
    pub const ORANGE_BODY: Self = Self(2);
    pub const RED_BODY: Self = Self(3);
    pub const WHITE_BODY: Self = Self(4);
    pub const GRAY_BODY: Self = Self(4);
    pub const YELLOW_BODY: Self = Self(5);
    pub const NUM_STAR_COLORS: Self = Self(6);
    pub const CYAN_BODY: Self = Self(6);
    pub const PURPLE_BODY: Self = Self(7);
    pub const VIOLET_BODY: Self = Self(8);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct HyperMenuItems(pub ::core::ffi::c_uint);
impl HyperMenuItems {
    pub const STARMAP: Self = Self(1);
    pub const EQUIP_DEVICE: Self = Self(2);
    pub const CARGO: Self = Self(3);
    pub const ROSTER: Self = Self(4);
    pub const GAME_MENU: Self = Self(5);
    pub const NAVIGATION: Self = Self(6);
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
unsafe extern "C" fn logxToUniverse(mut lx: SDWORD) -> COORD {
    return ((lx
        * ((9999 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
            >> 4 as ::core::ffi::c_int) * 10 as ::core::ffi::c_int)
        + ((((ScreenWidth - 64 as ::core::ffi::c_int
            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
            << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
            >> 4 as ::core::ffi::c_int) * 195 as ::core::ffi::c_int
            >> 1 as ::core::ffi::c_int))
        / ((((ScreenWidth - 64 as ::core::ffi::c_int
            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
            << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
            >> 4 as ::core::ffi::c_int) * 195 as ::core::ffi::c_int)) as COORD;
}
#[inline]
unsafe extern "C" fn logyToUniverse(mut ly: SDWORD) -> COORD {
    return (9999 as ::core::ffi::c_int
        - (ly
            * (9999 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                >> 4 as ::core::ffi::c_int)
            + ((((ScreenHeight - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                >> 4 as ::core::ffi::c_int) * 25 as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int))
            / ((((ScreenHeight - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                >> 4 as ::core::ffi::c_int) * 25 as ::core::ffi::c_int)) as COORD;
}
#[inline]
unsafe extern "C" fn universeToLogx(mut ux: COORD) -> SDWORD {
    return (ux as SDWORD
        * ((((ScreenWidth - 64 as ::core::ffi::c_int
            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
            << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
            >> 4 as ::core::ffi::c_int) * 195 as SDWORD)
        + ((9999 as SDWORD + 1 as SDWORD >> 4 as ::core::ffi::c_int) * 10 as SDWORD
            >> 1 as ::core::ffi::c_int))
        / ((9999 as SDWORD + 1 as SDWORD >> 4 as ::core::ffi::c_int) * 10 as SDWORD);
}
#[inline]
unsafe extern "C" fn universeToLogy(mut uy: COORD) -> SDWORD {
    return ((9999 as SDWORD - uy as SDWORD)
        * ((((ScreenHeight - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
            << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
            >> 4 as ::core::ffi::c_int) * 25 as SDWORD)
        + (9999 as SDWORD + 1 as SDWORD >> 4 as ::core::ffi::c_int
            >> 1 as ::core::ffi::c_int))
        / (9999 as SDWORD + 1 as SDWORD >> 4 as ::core::ffi::c_int);
}
static mut hyperstars: [FRAME; 3] = [::core::ptr::null_mut::<FRAME_DESC>(); 3];
static mut hypercmaps: [COLORMAP; 2] = [::core::ptr::null_mut::<
    STRING_TABLE_ENTRY_DESC,
>(); 2];
static mut fuel_ticks: BYTE = 0;
static mut hyper_extra: COUNT = 0;
static mut hyper_dy: COUNT = 0;
static mut hyper_dx: COUNT = 0;
#[no_mangle]
pub unsafe extern "C" fn MoveSIS(mut pdx: *mut SIZE, mut pdy: *mut SIZE) {
    let mut new_dx: SIZE = 0;
    let mut new_dy: SIZE = 0;
    new_dx = *pdx;
    GlobData.SIS_state.log_x -= new_dx as ::core::ffi::c_int;
    if GlobData.SIS_state.log_x < 0 as ::core::ffi::c_int {
        new_dx = (new_dx as ::core::ffi::c_int
            + GlobData.SIS_state.log_x as SIZE as ::core::ffi::c_int) as SIZE;
        GlobData.SIS_state.log_x = 0 as ::core::ffi::c_int as SDWORD;
    } else if GlobData.SIS_state.log_x as ::core::ffi::c_long
        > (universeToLogx(
            (9999 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as COORD,
        ) - (universeToLogx(1 as COORD) >> 1 as ::core::ffi::c_int))
            as ::core::ffi::c_long - 1 as ::core::ffi::c_long
    {
        new_dx = (new_dx as ::core::ffi::c_int
            + (GlobData.SIS_state.log_x as ::core::ffi::c_long
                - ((universeToLogx(
                    (9999 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as COORD,
                ) - (universeToLogx(1 as COORD) >> 1 as ::core::ffi::c_int))
                    as ::core::ffi::c_long - 1 as ::core::ffi::c_long)) as SIZE
                as ::core::ffi::c_int) as SIZE;
        GlobData.SIS_state.log_x = ((universeToLogx(
            (9999 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as COORD,
        ) - (universeToLogx(1 as COORD) >> 1 as ::core::ffi::c_int))
            as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as SDWORD;
    }
    new_dy = *pdy;
    GlobData.SIS_state.log_y -= new_dy as ::core::ffi::c_int;
    if GlobData.SIS_state.log_y < 0 as ::core::ffi::c_int {
        new_dy = (new_dy as ::core::ffi::c_int
            + GlobData.SIS_state.log_y as SIZE as ::core::ffi::c_int) as SIZE;
        GlobData.SIS_state.log_y = 0 as ::core::ffi::c_int as SDWORD;
    } else if GlobData.SIS_state.log_y as ::core::ffi::c_long
        > (universeToLogy(-1 as COORD)
            - (universeToLogy(
                (9999 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as COORD,
            ) >> 1 as ::core::ffi::c_int)) as ::core::ffi::c_long
            - 1 as ::core::ffi::c_long
    {
        new_dy = (new_dy as ::core::ffi::c_int
            + (GlobData.SIS_state.log_y as ::core::ffi::c_long
                - ((universeToLogy(-1 as COORD)
                    - (universeToLogy(
                        (9999 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as COORD,
                    ) >> 1 as ::core::ffi::c_int)) as ::core::ffi::c_long
                    - 1 as ::core::ffi::c_long)) as SIZE as ::core::ffi::c_int) as SIZE;
        GlobData.SIS_state.log_y = ((universeToLogy(-1 as COORD)
            - (universeToLogy(
                (9999 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as COORD,
            ) >> 1 as ::core::ffi::c_int)) as ::core::ffi::c_long
            - 1 as ::core::ffi::c_long) as SDWORD;
    }
    if new_dx as ::core::ffi::c_int != *pdx as ::core::ffi::c_int
        || new_dy as ::core::ffi::c_int != *pdy as ::core::ffi::c_int
    {
        let mut hElement: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
        let mut hNextElement: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
        *pdx = new_dx;
        *pdy = new_dy;
        hElement = disp_q.tail as HELEMENT;
        while !hElement.is_null() {
            let mut ElementPtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
            ElementPtr = LockLink(&raw mut disp_q, hElement) as *mut ELEMENT;
            if (*ElementPtr).state_flags as ::core::ffi::c_int
                & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int == 0
            {
                hNextElement = (*ElementPtr).pred;
            } else {
                (*ElementPtr).next.location.x = ((((ScreenWidth
                    - 64 as ::core::ffi::c_int
                    - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                    << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    >> 1 as ::core::ffi::c_int) - new_dx as ::core::ffi::c_int) as COORD;
                (*ElementPtr).next.location.y = ((((ScreenHeight
                    - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                    << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    >> 1 as ::core::ffi::c_int) - new_dy as ::core::ffi::c_int) as COORD;
                hNextElement = ::core::ptr::null_mut::<::core::ffi::c_void>();
            }
            UnlockLink(&raw mut disp_q, hElement);
            hElement = hNextElement;
        }
    }
    if GlobData.SIS_state.FuelOnBoard != 0
        && getGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_8::ARILOU_SPACE_SIDE.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_8::END_ARILOU_SPACE_SIDE.0 as ::core::ffi::c_int,
        ) as ::core::ffi::c_int <= 1 as ::core::ffi::c_int
    {
        let mut cur_fuel_ticks: COUNT = 0;
        let mut hyper_dist: COUNT = 0;
        let mut adj_dx: DWORD = 0;
        let mut adj_dy: DWORD = 0;
        if (new_dx as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
            new_dx = -(new_dx as ::core::ffi::c_int) as SIZE;
        }
        hyper_dx = (hyper_dx as ::core::ffi::c_int + new_dx as ::core::ffi::c_int)
            as COUNT;
        if (new_dy as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
            new_dy = -(new_dy as ::core::ffi::c_int) as SIZE;
        }
        hyper_dy = (hyper_dy as ::core::ffi::c_int + new_dy as ::core::ffi::c_int)
            as COUNT;
        adj_dx = logxToUniverse(16 as SDWORD * hyper_dx as SDWORD) as DWORD;
        adj_dy = (9999 as ::core::ffi::c_int
            - logyToUniverse(16 as SDWORD * hyper_dy as SDWORD) as ::core::ffi::c_int)
            as DWORD;
        hyper_dist = (square_root(
            adj_dx.wrapping_mul(adj_dx).wrapping_add(adj_dy.wrapping_mul(adj_dy)),
        ) as ::core::ffi::c_int + hyper_extra as ::core::ffi::c_int) as COUNT;
        cur_fuel_ticks = (hyper_dist as ::core::ffi::c_int >> 4 as ::core::ffi::c_int)
            as COUNT;
        if cur_fuel_ticks as ::core::ffi::c_int
            > fuel_ticks as COUNT as ::core::ffi::c_int
        {
            DeltaSISGauges(
                0 as SIZE,
                (fuel_ticks as ::core::ffi::c_int - cur_fuel_ticks as ::core::ffi::c_int)
                    as SIZE,
                0 as ::core::ffi::c_int,
            );
            if cur_fuel_ticks as ::core::ffi::c_int > 0xff as ::core::ffi::c_int {
                hyper_dx = 0 as COUNT;
                hyper_extra = (hyper_dist as ::core::ffi::c_int
                    & ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int) as COUNT;
                hyper_dy = 0 as COUNT;
                cur_fuel_ticks = 0 as COUNT;
            }
            fuel_ticks = cur_fuel_ticks as BYTE;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn check_hyperspace_encounter() {
    let mut Type: BYTE = 0;
    let mut universe: POINT = POINT { x: 0, y: 0 };
    let mut hStarShip: HFLEETINFO = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut hNextShip: HFLEETINFO = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut EncounterPercent: [COUNT; 24] = [
        20 as COUNT,
        0 as COUNT,
        0 as COUNT,
        20 as COUNT,
        40 as COUNT,
        0 as COUNT,
        20 as COUNT,
        40 as COUNT,
        60 as COUNT,
        40 as COUNT,
        40 as COUNT,
        60 as COUNT,
        0 as COUNT,
        30 as COUNT,
        60 as COUNT,
        40 as COUNT,
        2 as COUNT,
        30 as COUNT,
        70 as COUNT,
        0 as COUNT,
        0 as COUNT,
        70 as COUNT,
        60 as COUNT,
        0 as COUNT,
    ];
    universe.x = logxToUniverse(GlobData.SIS_state.log_x);
    universe.y = logyToUniverse(GlobData.SIS_state.log_y);
    hStarShip = GlobData.Game_state.avail_race_q.head as HFLEETINFO;
    Type = 0 as BYTE;
    while !hStarShip.is_null()
        && GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
            & C2Rust_Unnamed_9::IN_BATTLE.0 as ::core::ffi::c_int != 0
    {
        let mut encounter_radius: COUNT = 0;
        let mut FleetPtr: *mut FLEET_INFO = ::core::ptr::null_mut::<FLEET_INFO>();
        FleetPtr = LockFleetInfo(&raw mut GlobData.Game_state.avail_race_q, hStarShip);
        hNextShip = (*FleetPtr).succ;
        encounter_radius = (*FleetPtr).actual_strength;
        if encounter_radius != 0 {
            let mut encounter_flags: BYTE = 0;
            let mut dx: SIZE = 0;
            let mut dy: SIZE = 0;
            let mut percent: COUNT = 0;
            let mut hEncounter: HENCOUNTER = ::core::ptr::null_mut::<
                ::core::ffi::c_void,
            >();
            let mut EncounterPtr: *mut ENCOUNTER = ::core::ptr::null_mut::<ENCOUNTER>();
            encounter_flags = 0 as BYTE;
            percent = EncounterPercent[Type as usize];
            if encounter_radius as ::core::ffi::c_int
                != !(0 as ::core::ffi::c_int) as COUNT as ::core::ffi::c_int
            {
                encounter_radius = (encounter_radius as ::core::ffi::c_int
                    * 11 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as COUNT;
            } else {
                let mut hNextEncounter: HENCOUNTER = ::core::ptr::null_mut::<
                    ::core::ffi::c_void,
                >();
                encounter_radius = ((9999 as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as COUNT;
                if Type as ::core::ffi::c_int
                    == C2Rust_Unnamed_5::SLYLANDRO_SHIP.0 as ::core::ffi::c_int
                {
                    encounter_flags = ((1 as ::core::ffi::c_int)
                        << 7 as ::core::ffi::c_int) as BYTE;
                    if getGameState(
                        &raw mut GlobData.Game_state.GameState as *mut BYTE,
                        C2Rust_Unnamed_8::STARBASE_AVAILABLE.0 as ::core::ffi::c_int,
                        C2Rust_Unnamed_8::END_STARBASE_AVAILABLE.0 as ::core::ffi::c_int,
                    ) == 0
                    {
                        percent = 100 as COUNT;
                    } else {
                        percent = (percent as ::core::ffi::c_int
                            * getGameState(
                                &raw mut GlobData.Game_state.GameState as *mut BYTE,
                                C2Rust_Unnamed_8::SLYLANDRO_MULTIPLIER.0
                                    as ::core::ffi::c_int,
                                C2Rust_Unnamed_8::END_SLYLANDRO_MULTIPLIER.0
                                    as ::core::ffi::c_int,
                            ) as ::core::ffi::c_int) as COUNT;
                    }
                } else if Type as ::core::ffi::c_int
                    == C2Rust_Unnamed_5::MELNORME_SHIP.0 as ::core::ffi::c_int
                    && (GlobData.SIS_state.FuelOnBoard == 0 as DWORD
                        || getGameState(
                            &raw mut GlobData.Game_state.GameState as *mut BYTE,
                            C2Rust_Unnamed_8::USED_BROADCASTER.0 as ::core::ffi::c_int,
                            C2Rust_Unnamed_8::END_USED_BROADCASTER.0
                                as ::core::ffi::c_int,
                        ) as ::core::ffi::c_int != 0)
                    && (getGameState(
                        &raw mut GlobData.Game_state.GameState as *mut BYTE,
                        C2Rust_Unnamed_8::MELNORME_ANGER.0 as ::core::ffi::c_int,
                        C2Rust_Unnamed_8::END_MELNORME_ANGER.0 as ::core::ffi::c_int,
                    ) as ::core::ffi::c_int) < 3 as ::core::ffi::c_int
                {
                    if getGameState(
                        &raw mut GlobData.Game_state.GameState as *mut BYTE,
                        C2Rust_Unnamed_8::USED_BROADCASTER.0 as ::core::ffi::c_int,
                        C2Rust_Unnamed_8::END_USED_BROADCASTER.0 as ::core::ffi::c_int,
                    ) == 0
                    {
                        percent = 30 as COUNT;
                    } else {
                        percent = 100 as COUNT;
                    }
                    encounter_flags = ((1 as ::core::ffi::c_int)
                        << 7 as ::core::ffi::c_int) as BYTE;
                }
                hEncounter = GlobData.Game_state.encounter_q.head as HENCOUNTER;
                while !hEncounter.is_null() {
                    EncounterPtr = LockLink(
                        &raw mut GlobData.Game_state.encounter_q,
                        hEncounter,
                    ) as *mut ENCOUNTER;
                    hNextEncounter = (*EncounterPtr).succ;
                    if (*EncounterPtr).race_id as ::core::ffi::c_int
                        == Type as ::core::ffi::c_int
                    {
                        percent = 0 as COUNT;
                        hNextEncounter = ::core::ptr::null_mut::<::core::ffi::c_void>();
                    }
                    UnlockLink(&raw mut GlobData.Game_state.encounter_q, hEncounter);
                    hEncounter = hNextEncounter;
                }
                if percent as ::core::ffi::c_int == 100 as ::core::ffi::c_int
                    && Type as ::core::ffi::c_int
                        == C2Rust_Unnamed_5::MELNORME_SHIP.0 as ::core::ffi::c_int
                {
                    setGameState(
                        &raw mut GlobData.Game_state.GameState as *mut BYTE,
                        C2Rust_Unnamed_8::BROADCASTER_RESPONSE.0 as ::core::ffi::c_int,
                        C2Rust_Unnamed_8::END_BROADCASTER_RESPONSE.0
                            as ::core::ffi::c_int,
                        1 as BYTE,
                    );
                }
            }
            dx = (universe.x as ::core::ffi::c_int
                - (*FleetPtr).loc.x as ::core::ffi::c_int) as SIZE;
            if (dx as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                dx = -(dx as ::core::ffi::c_int) as SIZE;
            }
            dy = (universe.y as ::core::ffi::c_int
                - (*FleetPtr).loc.y as ::core::ffi::c_int) as SIZE;
            if (dy as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                dy = -(dy as ::core::ffi::c_int) as SIZE;
            }
            if (dx as COUNT as ::core::ffi::c_int)
                < encounter_radius as ::core::ffi::c_int
                && (dy as COUNT as ::core::ffi::c_int)
                    < encounter_radius as ::core::ffi::c_int
                && (dx as DWORD)
                    .wrapping_mul(dx as DWORD)
                    .wrapping_add((dy as DWORD).wrapping_mul(dy as DWORD))
                    < (encounter_radius as DWORD).wrapping_mul(encounter_radius as DWORD)
                && (TFB_Random() as COUNT as ::core::ffi::c_int
                    % 100 as ::core::ffi::c_int) < percent as ::core::ffi::c_int
            {
                hEncounter = AllocLink(&raw mut GlobData.Game_state.encounter_q)
                    as HENCOUNTER;
                if !hEncounter.is_null() {
                    EncounterPtr = LockLink(
                        &raw mut GlobData.Game_state.encounter_q,
                        hEncounter,
                    ) as *mut ENCOUNTER;
                    memset(
                        EncounterPtr as *mut ::core::ffi::c_void,
                        0 as ::core::ffi::c_int,
                        ::core::mem::size_of::<ENCOUNTER>(),
                    );
                    (*EncounterPtr).origin = (*FleetPtr).loc;
                    (*EncounterPtr).radius = encounter_radius;
                    (*EncounterPtr).flags = encounter_flags;
                    (*EncounterPtr).race_id = Type;
                    UnlockLink(&raw mut GlobData.Game_state.encounter_q, hEncounter);
                    PutQueue(&raw mut GlobData.Game_state.encounter_q, hEncounter);
                }
            }
        }
        UnlockLink(&raw mut GlobData.Game_state.avail_race_q, hStarShip);
        hStarShip = hNextShip;
        Type = Type.wrapping_add(1);
    }
    setGameState(
        &raw mut GlobData.Game_state.GameState as *mut BYTE,
        C2Rust_Unnamed_8::USED_BROADCASTER.0 as ::core::ffi::c_int,
        C2Rust_Unnamed_8::END_USED_BROADCASTER.0 as ::core::ffi::c_int,
        0 as BYTE,
    );
}
#[no_mangle]
pub unsafe extern "C" fn FreeHyperData() {
    DestroyDrawable(ReleaseDrawable(hyperstars[0usize]));
    hyperstars[0usize] = ::core::ptr::null_mut::<FRAME_DESC>();
    DestroyDrawable(ReleaseDrawable(hyperstars[1usize]));
    hyperstars[1usize] = ::core::ptr::null_mut::<FRAME_DESC>();
    DestroyDrawable(ReleaseDrawable(hyperstars[2usize]));
    hyperstars[2usize] = ::core::ptr::null_mut::<FRAME_DESC>();
    DestroyStringTable(ReleaseStringTable(hypercmaps[0usize]));
    hypercmaps[0usize] = ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>();
    DestroyStringTable(ReleaseStringTable(hypercmaps[1usize]));
    hypercmaps[1usize] = ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>();
}
unsafe extern "C" fn LoadHyperData() {
    if hyperstars[0usize].is_null() {
        hyperstars[0usize] = CaptureDrawable(
            LoadGraphicInstance(b"graphics.ambient\0".as_ptr() as RESOURCE) as DRAWABLE,
        );
        hyperstars[1usize] = CaptureDrawable(
            LoadGraphicInstance(b"graphics.hyperstars\0".as_ptr() as RESOURCE)
                as DRAWABLE,
        );
        hypercmaps[0usize] = CaptureStringTable(
            LoadStringTableInstance(b"colortable.hyperspace\0".as_ptr() as RESOURCE),
        ) as COLORMAP;
        hyperstars[2usize] = CaptureDrawable(
            LoadGraphicInstance(b"graphics.quasispace\0".as_ptr() as RESOURCE)
                as DRAWABLE,
        );
        hypercmaps[1usize] = CaptureStringTable(
            LoadStringTableInstance(b"colortable.quasispace\0".as_ptr() as RESOURCE),
        ) as COLORMAP;
    }
}
#[no_mangle]
pub unsafe extern "C" fn LoadHyperspace() -> BOOLEAN {
    hyper_dx = 0 as COUNT;
    hyper_dy = 0 as COUNT;
    hyper_extra = 0 as COUNT;
    fuel_ticks = 1 as BYTE;
    GlobData.Game_state.ShipStamp.origin.x = -9999 as COORD;
    GlobData.Game_state.ShipStamp.origin.y = -9999 as COORD;
    LoadHyperData();
    let mut F: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
    F = hyperstars[0usize];
    hyperstars[0usize] = stars_in_space;
    stars_in_space = F;
    if LastActivity as ::core::ffi::c_int
        & C2Rust_Unnamed_9::CHECK_LOAD.0 as ::core::ffi::c_int == 0
    {
        RepairSISBorder();
    } else if LastActivity as BYTE as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        DrawSISFrame();
    } else {
        ClearSISRect(((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as BYTE);
        RepairSISBorder();
    }
    if !(GlobData.Game_state.autopilot.x as ::core::ffi::c_int
        != !(0 as ::core::ffi::c_int)
        && GlobData.Game_state.autopilot.y as ::core::ffi::c_int
            != !(0 as ::core::ffi::c_int))
    {
        DrawSISMessage(::core::ptr::null::<UNICODE>());
    }
    SetContext(RadarContext);
    SetContextBackGroundColor(colorFromRgb15(0 as BYTE, 0xe as BYTE, 0 as BYTE));
    SetContext(SpaceContext);
    if getGameState(
        &raw mut GlobData.Game_state.GameState as *mut BYTE,
        C2Rust_Unnamed_8::ARILOU_SPACE_SIDE.0 as ::core::ffi::c_int,
        C2Rust_Unnamed_8::END_ARILOU_SPACE_SIDE.0 as ::core::ffi::c_int,
    ) as ::core::ffi::c_int <= 1 as ::core::ffi::c_int
    {
        SetContextBackGroundColor(colorFromRgb15(0x7 as BYTE, 0 as BYTE, 0 as BYTE));
        SetColorMap(GetColorMapAddress(hypercmaps[0usize]));
    } else {
        SetContextBackGroundColor(colorFromRgb15(0 as BYTE, 0x1a as BYTE, 0 as BYTE));
        SetColorMap(GetColorMapAddress(hypercmaps[1usize]));
        setGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_8::USED_BROADCASTER.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_8::END_USED_BROADCASTER.0 as ::core::ffi::c_int,
            0 as BYTE,
        );
        setGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_8::BROADCASTER_RESPONSE.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_8::END_BROADCASTER_RESPONSE.0 as ::core::ffi::c_int,
            0 as BYTE,
        );
    }
    ClearSISRect(((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as BYTE);
    return BOOLEAN::TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn FreeHyperspace() -> BOOLEAN {
    let mut F: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
    F = hyperstars[0usize];
    hyperstars[0usize] = stars_in_space;
    stars_in_space = F;
    return BOOLEAN::TRUE;
}
unsafe extern "C" fn ElementToUniverse(
    mut ElementPtr: *mut ELEMENT,
    mut pPt: *mut POINT,
) {
    let mut log_x: SDWORD = 0;
    let mut log_y: SDWORD = 0;
    log_x = (GlobData.SIS_state.log_x
        + ((*ElementPtr).next.location.x as ::core::ffi::c_int
            - (((ScreenWidth - 64 as ::core::ffi::c_int
                - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int))) as SDWORD;
    log_y = (GlobData.SIS_state.log_y
        + ((*ElementPtr).next.location.y as ::core::ffi::c_int
            - (((ScreenHeight - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int))) as SDWORD;
    (*pPt).x = logxToUniverse(log_x);
    (*pPt).y = logyToUniverse(log_y);
}
unsafe extern "C" fn cleanup_hyperspace() {
    let mut hEncounter: HENCOUNTER = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut hNextEncounter: HENCOUNTER = ::core::ptr::null_mut::<::core::ffi::c_void>();
    hEncounter = GlobData.Game_state.encounter_q.head as HENCOUNTER;
    while !hEncounter.is_null() {
        let mut EncounterPtr: *mut ENCOUNTER = ::core::ptr::null_mut::<ENCOUNTER>();
        EncounterPtr = LockLink(&raw mut GlobData.Game_state.encounter_q, hEncounter)
            as *mut ENCOUNTER;
        hNextEncounter = (*EncounterPtr).succ;
        if !(*EncounterPtr).hElement.is_null() {
            let mut ElementPtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
            ElementPtr = LockLink(&raw mut disp_q, (*EncounterPtr).hElement)
                as *mut ELEMENT;
            if !(*ElementPtr).hTarget.is_null() {
                RemoveQueue(&raw mut GlobData.Game_state.encounter_q, hEncounter);
                InsertQueue(
                    &raw mut GlobData.Game_state.encounter_q,
                    hEncounter,
                    GlobData.Game_state.encounter_q.head,
                );
            }
            UnlockLink(&raw mut disp_q, (*EncounterPtr).hElement);
        }
        (*EncounterPtr).hElement = ::core::ptr::null_mut::<::core::ffi::c_void>();
        UnlockLink(&raw mut GlobData.Game_state.encounter_q, hEncounter);
        hEncounter = hNextEncounter;
    }
}
unsafe extern "C" fn InterplanetaryTransition(mut ElementPtr: *mut ELEMENT) {
    GlobData.Game_state.ip_planet = 0 as BYTE;
    GlobData.Game_state.in_orbit = 0 as BYTE;
    GlobData.Game_state.ShipFacing = 0 as UWORD;
    setGameState(
        &raw mut GlobData.Game_state.GameState as *mut BYTE,
        C2Rust_Unnamed_8::USED_BROADCASTER.0 as ::core::ffi::c_int,
        C2Rust_Unnamed_8::END_USED_BROADCASTER.0 as ::core::ffi::c_int,
        0 as BYTE,
    );
    if getGameState(
        &raw mut GlobData.Game_state.GameState as *mut BYTE,
        C2Rust_Unnamed_8::ARILOU_SPACE_SIDE.0 as ::core::ffi::c_int,
        C2Rust_Unnamed_8::END_ARILOU_SPACE_SIDE.0 as ::core::ffi::c_int,
    ) as ::core::ffi::c_int <= 1 as ::core::ffi::c_int
    {
        GlobData.Game_state.CurrentActivity = (GlobData.Game_state.CurrentActivity
            as ::core::ffi::c_int
            | C2Rust_Unnamed_9::START_INTERPLANETARY.0 as ::core::ffi::c_int)
            as ACTIVITY;
        setGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_8::ESCAPE_COUNTER.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_8::END_ESCAPE_COUNTER.0 as ::core::ffi::c_int,
            0 as BYTE,
        );
    } else {
        let mut pt: POINT = POINT { x: 0, y: 0 };
        GlobData.Game_state.autopilot.x = !(0 as ::core::ffi::c_int) as COORD;
        GlobData.Game_state.autopilot.y = !(0 as ::core::ffi::c_int) as COORD;
        ElementToUniverse(ElementPtr, &raw mut pt);
        CurStarDescPtr = FindStar(
            ::core::ptr::null_mut::<STAR_DESC>(),
            &raw mut pt,
            5 as SIZE,
            5 as SIZE,
        );
        if (*CurStarDescPtr).star_pt.x as ::core::ffi::c_int
            == 5000 as ::core::ffi::c_int
                + (63 as ::core::ffi::c_int * 12 as ::core::ffi::c_int
                    >> 1 as ::core::ffi::c_int) * 3 as ::core::ffi::c_int
            && (*CurStarDescPtr).star_pt.y as ::core::ffi::c_int
                == 5000 as ::core::ffi::c_int
                    + (50 as ::core::ffi::c_int * 12 as ::core::ffi::c_int
                        >> 1 as ::core::ffi::c_int) * 3 as ::core::ffi::c_int
        {
            GlobData.Game_state.CurrentActivity = (GlobData.Game_state.CurrentActivity
                as ::core::ffi::c_int
                | C2Rust_Unnamed_9::START_ENCOUNTER.0 as ::core::ffi::c_int) as ACTIVITY;
        } else {
            let mut index: COUNT = 0;
            let portal_pt: [POINT; 15] = [
                point {
                    x: 4091 as COORD,
                    y: 7748 as COORD,
                },
                point {
                    x: 3184 as COORD,
                    y: 4906 as COORD,
                },
                point {
                    x: 9211 as COORD,
                    y: 6104 as COORD,
                },
                point {
                    x: 5673 as COORD,
                    y: 1207 as COORD,
                },
                point {
                    x: 1910 as COORD,
                    y: 926 as COORD,
                },
                point {
                    x: 8607 as COORD,
                    y: 151 as COORD,
                },
                point {
                    x: 50 as COORD,
                    y: 1647 as COORD,
                },
                point {
                    x: 6117 as COORD,
                    y: 4131 as COORD,
                },
                point {
                    x: 5658 as COORD,
                    y: 9712 as COORD,
                },
                point {
                    x: 2302 as COORD,
                    y: 3988 as COORD,
                },
                point {
                    x: 112 as COORD,
                    y: 9409 as COORD,
                },
                point {
                    x: 7752 as COORD,
                    y: 8906 as COORD,
                },
                point {
                    x: 368 as COORD,
                    y: 6332 as COORD,
                },
                point {
                    x: 9735 as COORD,
                    y: 3153 as COORD,
                },
                point {
                    x: 5850 as COORD,
                    y: 6213 as COORD,
                },
            ];
            index = CurStarDescPtr
                .offset_from(
                    star_array
                        .offset(
                            (502 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                as isize,
                        ),
                ) as COUNT;
            GlobData.SIS_state.log_x = universeToLogx(portal_pt[index as usize].x);
            GlobData.SIS_state.log_y = universeToLogy(portal_pt[index as usize].y);
            setGameState(
                &raw mut GlobData.Game_state.GameState as *mut BYTE,
                C2Rust_Unnamed_8::ARILOU_SPACE_SIDE.0 as ::core::ffi::c_int,
                C2Rust_Unnamed_8::END_ARILOU_SPACE_SIDE.0 as ::core::ffi::c_int,
                0 as BYTE,
            );
        }
    };
}
unsafe extern "C" fn ArilouSpaceTransition() {
    GlobData.Game_state.ShipFacing = 0 as UWORD;
    setGameState(
        &raw mut GlobData.Game_state.GameState as *mut BYTE,
        C2Rust_Unnamed_8::USED_BROADCASTER.0 as ::core::ffi::c_int,
        C2Rust_Unnamed_8::END_USED_BROADCASTER.0 as ::core::ffi::c_int,
        0 as BYTE,
    );
    GlobData.Game_state.autopilot.x = !(0 as ::core::ffi::c_int) as COORD;
    GlobData.Game_state.autopilot.y = !(0 as ::core::ffi::c_int) as COORD;
    if getGameState(
        &raw mut GlobData.Game_state.GameState as *mut BYTE,
        C2Rust_Unnamed_8::ARILOU_SPACE_SIDE.0 as ::core::ffi::c_int,
        C2Rust_Unnamed_8::END_ARILOU_SPACE_SIDE.0 as ::core::ffi::c_int,
    ) as ::core::ffi::c_int <= 1 as ::core::ffi::c_int
    {
        GlobData.SIS_state.log_x = universeToLogx(5000 as COORD);
        GlobData.SIS_state.log_y = universeToLogy(5000 as COORD);
        if getGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_8::PORTAL_COUNTER.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_8::END_PORTAL_COUNTER.0 as ::core::ffi::c_int,
        ) as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        {
            setGameState(
                &raw mut GlobData.Game_state.GameState as *mut BYTE,
                C2Rust_Unnamed_8::ARILOU_SPACE_SIDE.0 as ::core::ffi::c_int,
                C2Rust_Unnamed_8::END_ARILOU_SPACE_SIDE.0 as ::core::ffi::c_int,
                3 as BYTE,
            );
        } else {
            setGameState(
                &raw mut GlobData.Game_state.GameState as *mut BYTE,
                C2Rust_Unnamed_8::PORTAL_COUNTER.0 as ::core::ffi::c_int,
                C2Rust_Unnamed_8::END_PORTAL_COUNTER.0 as ::core::ffi::c_int,
                0 as BYTE,
            );
            setGameState(
                &raw mut GlobData.Game_state.GameState as *mut BYTE,
                C2Rust_Unnamed_8::ARILOU_SPACE_SIDE.0 as ::core::ffi::c_int,
                C2Rust_Unnamed_8::END_ARILOU_SPACE_SIDE.0 as ::core::ffi::c_int,
                3 as BYTE,
            );
        }
    } else {
        GlobData.SIS_state.log_x = universeToLogx(438 as COORD);
        GlobData.SIS_state.log_y = universeToLogy(6372 as COORD);
        setGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_8::ARILOU_SPACE_SIDE.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_8::END_ARILOU_SPACE_SIDE.0 as ::core::ffi::c_int,
            0 as BYTE,
        );
    };
}
unsafe extern "C" fn unhyper_transition(mut ElementPtr: *mut ELEMENT) {
    let mut frame_index: COUNT = 0;
    (*ElementPtr).state_flags = ((*ElementPtr).state_flags as ::core::ffi::c_int
        | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as ELEMENT_FLAGS;
    frame_index = GetFrameIndex((*ElementPtr).current.image.frame);
    if frame_index as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        frame_index = (frame_index as ::core::ffi::c_int
            + (((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                + ((1 as ::core::ffi::c_int)
                    << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int)
                >> 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int)) as COUNT;
    } else if (frame_index as ::core::ffi::c_int)
        < ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
            + ((1 as ::core::ffi::c_int)
                << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int)
            >> 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
    {
        frame_index = (frame_index as ::core::ffi::c_int + 1 as ::core::ffi::c_int
            & ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                - 1 as ::core::ffi::c_int) as COUNT;
    } else {
        frame_index = frame_index.wrapping_add(1);
        if frame_index as ::core::ffi::c_int
            == GetFrameCount((*ElementPtr).current.image.frame) as ::core::ffi::c_int
        {
            cleanup_hyperspace();
            GlobData.Game_state.CurrentActivity = (GlobData.Game_state.CurrentActivity
                as ::core::ffi::c_int
                & !(C2Rust_Unnamed_9::IN_BATTLE.0 as ::core::ffi::c_int)) as ACTIVITY;
            match TRANSITION_TYPE(
                (*ElementPtr).c2rust_unnamed_2.turn_wait as ::core::ffi::c_uint,
            ) {
                TRANSITION_TYPE::RANDOM_ENCOUNTER_TRANSITION => {
                    SaveSisHyperState();
                    GlobData.Game_state.CurrentActivity = (GlobData
                        .Game_state
                        .CurrentActivity as ::core::ffi::c_int
                        | C2Rust_Unnamed_9::START_ENCOUNTER.0 as ::core::ffi::c_int)
                        as ACTIVITY;
                }
                TRANSITION_TYPE::INTERPLANETARY_TRANSITION => {
                    InterplanetaryTransition(ElementPtr);
                }
                TRANSITION_TYPE::ARILOU_SPACE_TRANSITION => {
                    ArilouSpaceTransition();
                }
                _ => {}
            }
            memset(
                &raw mut (*ElementPtr).velocity as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<VELOCITY_DESC>(),
            );
            DisplayArray[(*ElementPtr).PrimIndex as usize].Type = gfx_object::NUM_PRIMS.0
                as ::core::ffi::c_int as GRAPHICS_PRIM;
            return;
        }
    }
    (*ElementPtr).next.image.frame = SetAbsFrameIndex(
        (*ElementPtr).current.image.frame,
        frame_index,
    );
}
unsafe extern "C" fn init_transition(
    mut ElementPtr0: *mut ELEMENT,
    mut ElementPtr1: *mut ELEMENT,
    mut which_transition: TRANSITION_TYPE,
) {
    let mut dx: SIZE = 0;
    let mut dy: SIZE = 0;
    let mut num_turns: SIZE = 0;
    let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
    dx = (((*ElementPtr0).next.location.x as ::core::ffi::c_int
        - (*ElementPtr1).next.location.x as ::core::ffi::c_int)
        << 5 as ::core::ffi::c_int) as SIZE;
    dy = (((*ElementPtr0).next.location.y as ::core::ffi::c_int
        - (*ElementPtr1).next.location.y as ::core::ffi::c_int)
        << 5 as ::core::ffi::c_int) as SIZE;
    (*ElementPtr1).state_flags = ((*ElementPtr1).state_flags as ::core::ffi::c_int
        | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as ELEMENT_FLAGS;
    (*ElementPtr1).preprocess_func = Some(
        unhyper_transition as unsafe extern "C" fn(*mut ELEMENT) -> (),
    ) as Option<ElementProcessFunc>;
    (*ElementPtr1).postprocess_func = None;
    (*ElementPtr1).c2rust_unnamed_2.turn_wait = which_transition.0 as BYTE;
    StarShipPtr = (*ElementPtr1).pParent as *mut STARSHIP;
    num_turns = (GetFrameCount((*ElementPtr1).next.image.frame) as ::core::ffi::c_int
        - (((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
            + ((1 as ::core::ffi::c_int)
                << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int)
            >> 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int)
        + ((((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
            + ((1 as ::core::ffi::c_int)
                << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int)
            >> 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int)
            - (*StarShipPtr).ShipFacing as ::core::ffi::c_int
            & ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                - 1 as ::core::ffi::c_int) as COUNT as ::core::ffi::c_int) as SIZE;
    if num_turns as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        num_turns = 1 as SIZE;
    }
    SetVelocityComponents(
        &raw mut (*ElementPtr1).velocity,
        (dx as ::core::ffi::c_int / num_turns as ::core::ffi::c_int) as SIZE,
        (dy as ::core::ffi::c_int / num_turns as ::core::ffi::c_int) as SIZE,
    );
}
unsafe extern "C" fn hyper_preprocess(mut ElementPtr: *mut ELEMENT) {
    hyper_transition(ElementPtr);
}
#[no_mangle]
pub unsafe extern "C" fn hyper_transition(mut ElementPtr: *mut ELEMENT) -> BOOLEAN {
    if (*ElementPtr).state_flags as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int != 0
    {
        if LastActivity as ::core::ffi::c_int
            & C2Rust_Unnamed_9::CHECK_LOAD.0 as ::core::ffi::c_int != 0
        {
            LastActivity = (LastActivity as ::core::ffi::c_int
                & !(C2Rust_Unnamed_9::CHECK_LOAD.0 as ::core::ffi::c_int)) as ACTIVITY;
            (*ElementPtr).current = (*ElementPtr).next;
            SetUpElement(ElementPtr);
            (*ElementPtr).state_flags = ((*ElementPtr).state_flags as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int) as ELEMENT_FLAGS;
            return BOOLEAN::FALSE;
        } else {
            (*ElementPtr).preprocess_func = Some(
                hyper_preprocess as unsafe extern "C" fn(*mut ELEMENT) -> (),
            ) as Option<ElementProcessFunc>;
            (*ElementPtr).postprocess_func = None;
            (*ElementPtr).state_flags = ((*ElementPtr).state_flags as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as ELEMENT_FLAGS;
            (*ElementPtr).next.image.frame = SetAbsFrameIndex(
                (*ElementPtr).current.image.frame,
                (GetFrameCount((*ElementPtr).current.image.frame) as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            );
        }
    } else {
        let mut frame_index: COUNT = 0;
        frame_index = GetFrameIndex((*ElementPtr).current.image.frame);
        let c2rust_fresh0 = frame_index;
        frame_index = frame_index.wrapping_sub(1);
        if c2rust_fresh0 as ::core::ffi::c_int
            <= ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                + ((1 as ::core::ffi::c_int)
                    << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int)
                >> 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
        {
            let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
            if frame_index as ::core::ffi::c_int
                == (((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                    + ((1 as ::core::ffi::c_int)
                        << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int)
                    >> 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int
            {
                frame_index = 0 as COUNT;
            } else {
                frame_index = (frame_index as ::core::ffi::c_int
                    & ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int) as COUNT;
            }
            StarShipPtr = (*ElementPtr).pParent as *mut STARSHIP;
            if frame_index as ::core::ffi::c_int
                == (*StarShipPtr).ShipFacing as ::core::ffi::c_int
            {
                (*ElementPtr).preprocess_func = Some(
                    ship_preprocess as unsafe extern "C" fn(*mut ELEMENT) -> (),
                ) as Option<ElementProcessFunc>;
                (*ElementPtr).postprocess_func = Some(
                    ship_postprocess as unsafe extern "C" fn(*mut ELEMENT) -> (),
                ) as Option<ElementProcessFunc>;
                (*ElementPtr).state_flags = ((*ElementPtr).state_flags
                    as ::core::ffi::c_int
                    & !((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int))
                    as ELEMENT_FLAGS;
            }
        }
        (*ElementPtr).state_flags = ((*ElementPtr).state_flags as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as ELEMENT_FLAGS;
        (*ElementPtr).next.image.frame = SetAbsFrameIndex(
            (*ElementPtr).current.image.frame,
            frame_index,
        );
        if (*ElementPtr).state_flags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int == 0
        {
            (*ElementPtr).current = (*ElementPtr).next;
            SetUpElement(ElementPtr);
            (*ElementPtr).state_flags = ((*ElementPtr).state_flags as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int) as ELEMENT_FLAGS;
        }
    }
    return BOOLEAN::TRUE;
}
unsafe extern "C" fn hyper_collision(
    mut ElementPtr0: *mut ELEMENT,
    mut pPt0: *mut POINT,
    mut ElementPtr1: *mut ELEMENT,
    mut pPt1: *mut POINT,
) {
    if (*ElementPtr1).state_flags as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int != 0
        && getGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_8::PORTAL_COUNTER.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_8::END_PORTAL_COUNTER.0 as ::core::ffi::c_int,
        ) as ::core::ffi::c_int == 0 as ::core::ffi::c_int
    {
        let mut dx: SIZE = 0;
        let mut dy: SIZE = 0;
        let mut pt: POINT = POINT { x: 0, y: 0 };
        let mut SDPtr: *mut STAR_DESC = ::core::ptr::null_mut::<STAR_DESC>();
        let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
        ElementToUniverse(ElementPtr0, &raw mut pt);
        SDPtr = FindStar(
            ::core::ptr::null_mut::<STAR_DESC>(),
            &raw mut pt,
            5 as SIZE,
            5 as SIZE,
        );
        StarShipPtr = (*ElementPtr1).pParent as *mut STARSHIP;
        GetCurrentVelocityComponents(
            &raw mut (*ElementPtr1).velocity,
            &raw mut dx,
            &raw mut dy,
        );
        if SDPtr == CurStarDescPtr
            || (*ElementPtr1).state_flags as ::core::ffi::c_int
                & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int != 0
            || !(dx as ::core::ffi::c_int != 0 || dy as ::core::ffi::c_int != 0
                || (*StarShipPtr).cur_status_flags as ::core::ffi::c_int
                    & ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                        | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                        | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
                        | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                        | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) != 0)
        {
            CurStarDescPtr = SDPtr;
            (*ElementPtr0).state_flags = ((*ElementPtr0).state_flags
                as ::core::ffi::c_int
                | ((1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int))
                as ELEMENT_FLAGS;
        } else if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
            & C2Rust_Unnamed_9::IN_BATTLE.0 as ::core::ffi::c_int != 0
            && (GlobData.Game_state.autopilot.x as ::core::ffi::c_int
                == !(0 as ::core::ffi::c_int)
                || GlobData.Game_state.autopilot.y as ::core::ffi::c_int
                    == !(0 as ::core::ffi::c_int)
                || GlobData.Game_state.autopilot.x as ::core::ffi::c_int
                    == (*SDPtr).star_pt.x as ::core::ffi::c_int
                    && GlobData.Game_state.autopilot.y as ::core::ffi::c_int
                        == (*SDPtr).star_pt.y as ::core::ffi::c_int)
        {
            CurStarDescPtr = SDPtr;
            (*ElementPtr0).state_flags = ((*ElementPtr0).state_flags
                as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as ELEMENT_FLAGS;
            init_transition(
                ElementPtr0,
                ElementPtr1,
                TRANSITION_TYPE::INTERPLANETARY_TRANSITION,
            );
        }
    }
}
unsafe extern "C" fn hyper_death(mut ElementPtr: *mut ELEMENT) {
    if (*ElementPtr).state_flags as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int == 0
        && GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
            & C2Rust_Unnamed_9::IN_BATTLE.0 as ::core::ffi::c_int != 0
    {
        CurStarDescPtr = ::core::ptr::null_mut::<STAR_DESC>();
    }
}
unsafe extern "C" fn arilou_space_death(mut ElementPtr: *mut ELEMENT) {
    if (*ElementPtr).state_flags as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int == 0
        || getGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_8::ARILOU_SPACE_COUNTER.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_8::END_ARILOU_SPACE_COUNTER.0 as ::core::ffi::c_int,
        ) as ::core::ffi::c_int == 0 as ::core::ffi::c_int
    {
        if getGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_8::ARILOU_SPACE_SIDE.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_8::END_ARILOU_SPACE_SIDE.0 as ::core::ffi::c_int,
        ) as ::core::ffi::c_int <= 1 as ::core::ffi::c_int
        {
            setGameState(
                &raw mut GlobData.Game_state.GameState as *mut BYTE,
                C2Rust_Unnamed_8::ARILOU_SPACE_SIDE.0 as ::core::ffi::c_int,
                C2Rust_Unnamed_8::END_ARILOU_SPACE_SIDE.0 as ::core::ffi::c_int,
                0 as BYTE,
            );
        } else {
            setGameState(
                &raw mut GlobData.Game_state.GameState as *mut BYTE,
                C2Rust_Unnamed_8::ARILOU_SPACE_SIDE.0 as ::core::ffi::c_int,
                C2Rust_Unnamed_8::END_ARILOU_SPACE_SIDE.0 as ::core::ffi::c_int,
                3 as BYTE,
            );
        }
    }
}
unsafe extern "C" fn arilou_space_collision(
    mut ElementPtr0: *mut ELEMENT,
    mut pPt0: *mut POINT,
    mut ElementPtr1: *mut ELEMENT,
    mut pPt1: *mut POINT,
) {
    let mut which_side: COUNT = 0;
    if (*ElementPtr1).state_flags as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int == 0
    {
        return;
    }
    which_side = getGameState(
        &raw mut GlobData.Game_state.GameState as *mut BYTE,
        C2Rust_Unnamed_8::ARILOU_SPACE_SIDE.0 as ::core::ffi::c_int,
        C2Rust_Unnamed_8::END_ARILOU_SPACE_SIDE.0 as ::core::ffi::c_int,
    ) as COUNT;
    if which_side as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        || which_side as ::core::ffi::c_int == 3 as ::core::ffi::c_int
    {
        if (*ElementPtr1).state_flags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int != 0
        {
            setGameState(
                &raw mut GlobData.Game_state.GameState as *mut BYTE,
                C2Rust_Unnamed_8::ARILOU_SPACE_SIDE.0 as ::core::ffi::c_int,
                C2Rust_Unnamed_8::END_ARILOU_SPACE_SIDE.0 as ::core::ffi::c_int,
                (which_side as ::core::ffi::c_int ^ 1 as ::core::ffi::c_int) as BYTE,
            );
        } else {
            init_transition(
                ElementPtr0,
                ElementPtr1,
                TRANSITION_TYPE::ARILOU_SPACE_TRANSITION,
            );
        }
    }
    (*ElementPtr0).state_flags = ((*ElementPtr0).state_flags as ::core::ffi::c_int
        | ((1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int)) as ELEMENT_FLAGS;
}
unsafe extern "C" fn AllocHyperElement(mut elem_pt: *const POINT) -> HELEMENT {
    let mut hHyperSpaceElement: HELEMENT = ::core::ptr::null_mut::<
        ::core::ffi::c_void,
    >();
    hHyperSpaceElement = AllocElement();
    if !hHyperSpaceElement.is_null() {
        let mut HyperSpaceElementPtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
        HyperSpaceElementPtr = LockLink(&raw mut disp_q, hHyperSpaceElement)
            as *mut ELEMENT;
        (*HyperSpaceElementPtr).playerNr = -1 as SIZE;
        (*HyperSpaceElementPtr).state_flags = ((1 as ::core::ffi::c_int)
            << 5 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int) as ELEMENT_FLAGS;
        (*HyperSpaceElementPtr).c2rust_unnamed.life_span = 1 as COUNT;
        (*HyperSpaceElementPtr).c2rust_unnamed_1.mass_points = 1 as BYTE;
        let mut lx: ::core::ffi::c_long = 0;
        let mut ly: ::core::ffi::c_long = 0;
        lx = (universeToLogx((*elem_pt).x)
            + (((ScreenWidth - 64 as SDWORD - 0 as SDWORD * 2 as SDWORD)
                << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int) - GlobData.SIS_state.log_x)
            as ::core::ffi::c_long;
        (*HyperSpaceElementPtr).current.location.x = (if lx < 0 as ::core::ffi::c_long {
            lx
                + (((ScreenWidth - 64 as ::core::ffi::c_int
                    - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                    << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                    as ::core::ffi::c_long
        } else if lx
            >= (((ScreenWidth - 64 as ::core::ffi::c_int
                - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                as ::core::ffi::c_long
        {
            lx
                - (((ScreenWidth - 64 as ::core::ffi::c_int
                    - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                    << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                    as ::core::ffi::c_long
        } else {
            lx
        }) as COUNT as COORD;
        ly = (universeToLogy((*elem_pt).y)
            + (((ScreenHeight - 0 as SDWORD * 2 as SDWORD) << 2 as ::core::ffi::c_int)
                << 3 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
            - GlobData.SIS_state.log_y) as ::core::ffi::c_long;
        (*HyperSpaceElementPtr).current.location.y = (if ly < 0 as ::core::ffi::c_long {
            ly
                + (((ScreenHeight - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                    << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                    as ::core::ffi::c_long
        } else if ly
            >= (((ScreenHeight - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                as ::core::ffi::c_long
        {
            ly
                - (((ScreenHeight - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                    << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                    as ::core::ffi::c_long
        } else {
            ly
        }) as COUNT as COORD;
        DisplayArray[(*HyperSpaceElementPtr).PrimIndex as usize].Type = gfx_object::STAMP_PRIM
            .0 as ::core::ffi::c_int as GRAPHICS_PRIM;
        (*HyperSpaceElementPtr).current.image.farray = (&raw mut hyperstars
            as *mut FRAME)
            .offset(
                (1 as ::core::ffi::c_int
                    + (getGameState(
                        &raw mut GlobData.Game_state.GameState as *mut BYTE,
                        C2Rust_Unnamed_8::ARILOU_SPACE_SIDE.0 as ::core::ffi::c_int,
                        C2Rust_Unnamed_8::END_ARILOU_SPACE_SIDE.0 as ::core::ffi::c_int,
                    ) as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)) as isize,
            );
        UnlockLink(&raw mut disp_q, hHyperSpaceElement);
    }
    return hHyperSpaceElement;
}
unsafe extern "C" fn AddAmbientElement() {
    let mut hHyperSpaceElement: HELEMENT = ::core::ptr::null_mut::<
        ::core::ffi::c_void,
    >();
    hHyperSpaceElement = AllocElement();
    if !hHyperSpaceElement.is_null() {
        let mut dx: SIZE = 0;
        let mut dy: SIZE = 0;
        let mut rand_val: DWORD = 0;
        let mut HyperSpaceElementPtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
        HyperSpaceElementPtr = LockLink(&raw mut disp_q, hHyperSpaceElement)
            as *mut ELEMENT;
        (*HyperSpaceElementPtr).playerNr = -1 as SIZE;
        (*HyperSpaceElementPtr).state_flags = ((1 as ::core::ffi::c_int)
            << 3 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as ELEMENT_FLAGS;
        DisplayArray[(*HyperSpaceElementPtr).PrimIndex as usize].Type = gfx_object::STAMP_PRIM
            .0 as ::core::ffi::c_int as GRAPHICS_PRIM;
        (*HyperSpaceElementPtr).preprocess_func = Some(
            animation_preprocess as unsafe extern "C" fn(*mut ELEMENT) -> (),
        ) as Option<ElementProcessFunc>;
        rand_val = TFB_Random();
        dy = rand_val as UWORD as SIZE;
        dx = ((dy as UWORD as BYTE as ::core::ffi::c_int
            % (ScreenWidth - 64 as ::core::ffi::c_int
                - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)) as SIZE
            as ::core::ffi::c_int
            - (ScreenWidth - 64 as ::core::ffi::c_int
                - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int)) as SIZE;
        dy = (((dy as UWORD as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as BYTE
            as ::core::ffi::c_int
            % (ScreenHeight - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)) as SIZE
            as ::core::ffi::c_int
            - (ScreenHeight - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int)) as SIZE;
        (*HyperSpaceElementPtr).current.location.x = ((((ScreenWidth
            - 64 as ::core::ffi::c_int
            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
            << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
            >> 1 as ::core::ffi::c_int)
            + ((dx as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)) as COORD;
        (*HyperSpaceElementPtr).current.location.y = ((((ScreenHeight
            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
            << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
            >> 1 as ::core::ffi::c_int)
            + ((dy as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)) as COORD;
        (*HyperSpaceElementPtr).current.image.farray = &raw mut stars_in_space;
        if (rand_val >> 16 as ::core::ffi::c_int) as UWORD as ::core::ffi::c_int
            & 7 as ::core::ffi::c_int != 0
        {
            (*HyperSpaceElementPtr).c2rust_unnamed.life_span = 14 as COUNT;
            (*HyperSpaceElementPtr).current.image.frame = stars_in_space;
        } else {
            (*HyperSpaceElementPtr).c2rust_unnamed.life_span = 12 as COUNT;
            (*HyperSpaceElementPtr).current.image.frame = SetAbsFrameIndex(
                stars_in_space,
                14 as COUNT,
            );
        }
        UnlockLink(&raw mut disp_q, hHyperSpaceElement);
        InsertQueue(&raw mut disp_q, hHyperSpaceElement, disp_q.head);
    }
}
unsafe extern "C" fn encounter_transition(mut ElementPtr: *mut ELEMENT) {
    (*ElementPtr).state_flags = ((*ElementPtr).state_flags as ::core::ffi::c_int
        & !((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)) as ELEMENT_FLAGS;
    (*ElementPtr).c2rust_unnamed.life_span = 1 as COUNT;
    if (*ElementPtr).c2rust_unnamed_2.turn_wait != 0 {
        (*ElementPtr).c2rust_unnamed_2.turn_wait = (*ElementPtr)
            .c2rust_unnamed_2
            .turn_wait
            .wrapping_sub(1);
    } else {
        let mut f: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
        if (*ElementPtr).c2rust_unnamed_0.hit_points != 0 {
            f = DecFrameIndex((*ElementPtr).current.image.frame);
            (*ElementPtr).next.image.frame = f;
        } else {
            f = IncFrameIndex((*ElementPtr).current.image.frame);
            if f != *(*ElementPtr).current.image.farray.offset(0isize) {
                (*ElementPtr).next.image.frame = f;
            } else {
                (*ElementPtr).death_func = None;
            }
        }
        (*ElementPtr).c2rust_unnamed_2.turn_wait = 1 as BYTE;
    };
}
unsafe extern "C" fn getSisElement() -> HELEMENT {
    let mut hSis: HSTARSHIP = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut hShip: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
    hSis = (*(&raw mut race_q as *mut QUEUE).offset(0isize)).head as HSTARSHIP;
    if hSis.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    StarShipPtr = LockStarShip((&raw mut race_q as *mut QUEUE).offset(0isize), hSis);
    hShip = (*StarShipPtr).hShip;
    UnlockLink((&raw mut race_q as *mut QUEUE).offset(0isize), hSis);
    let mut ElementPtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
    ElementPtr = LockLink(&raw mut disp_q, hShip) as *mut ELEMENT;
    ((*ElementPtr).state_flags as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int != 0
        || {
            __assert_fail(
                b"ElementPtr->state_flags & PLAYER_SHIP\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/uqm/hyper.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                910 as ::core::ffi::c_int,
                b"getSisElement\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    UnlockLink(&raw mut disp_q, hShip);
    return hShip;
}
unsafe extern "C" fn encounter_collision(
    mut ElementPtr0: *mut ELEMENT,
    mut pPt0: *mut POINT,
    mut ElementPtr1: *mut ELEMENT,
    mut pPt1: *mut POINT,
) {
    let mut hEncounter: HENCOUNTER = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut hNextEncounter: HENCOUNTER = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if (*ElementPtr1).state_flags as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int == 0
        || GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
            & C2Rust_Unnamed_9::IN_BATTLE.0 as ::core::ffi::c_int == 0
    {
        return;
    }
    init_transition(
        ElementPtr0,
        ElementPtr1,
        TRANSITION_TYPE::RANDOM_ENCOUNTER_TRANSITION,
    );
    hEncounter = GlobData.Game_state.encounter_q.head as HENCOUNTER;
    while !hEncounter.is_null() {
        let mut EncounterPtr: *mut ENCOUNTER = ::core::ptr::null_mut::<ENCOUNTER>();
        EncounterPtr = LockLink(&raw mut GlobData.Game_state.encounter_q, hEncounter)
            as *mut ENCOUNTER;
        hNextEncounter = (*EncounterPtr).succ;
        if !(*EncounterPtr).hElement.is_null() {
            let mut ElementPtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
            ElementPtr = LockLink(&raw mut disp_q, (*EncounterPtr).hElement)
                as *mut ELEMENT;
            (*ElementPtr).state_flags = ((*ElementPtr).state_flags as ::core::ffi::c_int
                | ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int))
                as ELEMENT_FLAGS;
            UnlockLink(&raw mut disp_q, (*EncounterPtr).hElement);
        }
        UnlockLink(&raw mut GlobData.Game_state.encounter_q, hEncounter);
        hEncounter = hNextEncounter;
    }
    (*ElementPtr0).hTarget = getSisElement();
    memset(
        &raw mut (*ElementPtr0).velocity as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<VELOCITY_DESC>(),
    );
}
unsafe extern "C" fn AddEncounterElement(
    mut EncounterPtr: *mut ENCOUNTER,
    mut puniverse: *mut POINT,
) -> HELEMENT {
    let mut NewEncounter: BOOLEAN = BOOLEAN::FALSE;
    let mut hElement: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut enc_pt: POINT = POINT { x: 0, y: 0 };
    if getGameState(
        &raw mut GlobData.Game_state.GameState as *mut BYTE,
        C2Rust_Unnamed_8::ARILOU_SPACE_SIDE.0 as ::core::ffi::c_int,
        C2Rust_Unnamed_8::END_ARILOU_SPACE_SIDE.0 as ::core::ffi::c_int,
    ) as ::core::ffi::c_int >= 2 as ::core::ffi::c_int
    {
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    if (*EncounterPtr).flags as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int != 0
    {
        (*EncounterPtr).flags = ((*EncounterPtr).flags as ::core::ffi::c_int
            & !((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)) as BYTE;
        (*EncounterPtr).transition_state = 100 as SIZE;
        if (*EncounterPtr).flags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int != 0
            || (*EncounterPtr).num_ships as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        {
            return ::core::ptr::null_mut::<::core::ffi::c_void>();
        }
    }
    if (*EncounterPtr).num_ships != 0 {
        NewEncounter = BOOLEAN::FALSE;
        enc_pt = (*EncounterPtr).loc_pt;
    } else {
        let mut Type: BYTE = 0;
        let mut dx: SIZE = 0;
        let mut dy: SIZE = 0;
        let mut i: COUNT = 0;
        let mut NumShips: COUNT = 0;
        let mut radius_squared: DWORD = 0;
        let mut EncounterMakeup: [BYTE; 23] = [
            ((5 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int)
                << 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int
                | 1 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE,
            0 as BYTE,
            0 as BYTE,
            ((5 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int)
                << 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int
                | 1 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE,
            ((5 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int)
                << 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int
                | 1 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE,
            0 as BYTE,
            ((5 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int)
                << 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int
                | 1 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE,
            ((5 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int)
                << 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int
                | 1 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE,
            ((5 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int)
                << 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int
                | 1 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE,
            ((5 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int)
                << 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int
                | 1 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE,
            ((5 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int)
                << 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int
                | 1 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE,
            ((5 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int)
                << 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int
                | 1 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE,
            ((1 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int)
                << 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int
                | 1 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE,
            ((5 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int)
                << 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int
                | 1 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE,
            ((5 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int)
                << 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int
                | 1 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE,
            ((5 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int)
                << 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int
                | 1 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE,
            ((1 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int)
                << 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int
                | 1 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE,
            ((5 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int)
                << 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int
                | 1 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE,
            ((5 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int)
                << 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int
                | 1 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE,
            ((5 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int)
                << 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int
                | 1 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE,
            0 as BYTE,
            ((5 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int)
                << 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int
                | 1 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE,
            ((5 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int)
                << 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int
                | 1 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE,
        ];
        NewEncounter = BOOLEAN::TRUE;
        radius_squared = ((*EncounterPtr).radius as DWORD)
            .wrapping_mul((*EncounterPtr).radius as DWORD);
        Type = (*EncounterPtr).race_id;
        NumShips = (EncounterMakeup[Type as usize] as ::core::ffi::c_int
            & 0xf as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE as COUNT;
        i = ((EncounterMakeup[Type as usize] as ::core::ffi::c_int
            >> 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE
            as ::core::ffi::c_int - NumShips as ::core::ffi::c_int) as COUNT;
        while i != 0 {
            if (TFB_Random() as COUNT as ::core::ffi::c_int % 100 as ::core::ffi::c_int)
                < 50 as ::core::ffi::c_int
            {
                NumShips = NumShips.wrapping_add(1);
            }
            i = i.wrapping_sub(1);
        }
        if NumShips as ::core::ffi::c_int > 7 as ::core::ffi::c_int {
            NumShips = 7 as COUNT;
        }
        (*EncounterPtr).num_ships = NumShips as BYTE;
        i = 0 as COUNT;
        while (i as ::core::ffi::c_int) < NumShips as ::core::ffi::c_int {
            let mut BSIPtr: *mut BRIEF_SHIP_INFO = (&raw mut (*EncounterPtr).ShipList
                as *mut BRIEF_SHIP_INFO)
                .offset(i as isize);
            let mut hStarShip: HFLEETINFO = GetStarShipFromIndex(
                &raw mut GlobData.Game_state.avail_race_q,
                Type as COUNT,
            );
            let mut FleetPtr: *mut FLEET_INFO = LockFleetInfo(
                &raw mut GlobData.Game_state.avail_race_q,
                hStarShip,
            );
            (*BSIPtr).race_id = Type;
            (*BSIPtr).crew_level = (*FleetPtr).crew_level;
            (*BSIPtr).max_crew = (*FleetPtr).max_crew;
            (*BSIPtr).max_energy = (*FleetPtr).max_energy;
            UnlockLink(&raw mut GlobData.Game_state.avail_race_q, hStarShip);
            i = i.wrapping_add(1);
        }
        loop {
            let mut rand_val: DWORD = 0;
            rand_val = TFB_Random();
            enc_pt.x = ((*puniverse).x as ::core::ffi::c_int
                + rand_val as UWORD as ::core::ffi::c_int
                    % ((63 as ::core::ffi::c_int * 12 as ::core::ffi::c_int
                        + ((63 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                        >> 1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
                - (63 as ::core::ffi::c_int * 12 as ::core::ffi::c_int
                    + ((63 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                    >> 1 as ::core::ffi::c_int)) as COORD;
            if (enc_pt.x as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                enc_pt.x = 0 as COORD;
            } else if enc_pt.x as ::core::ffi::c_int > 9999 as ::core::ffi::c_int {
                enc_pt.x = 9999 as COORD;
            }
            enc_pt.y = ((*puniverse).y as ::core::ffi::c_int
                + (rand_val >> 16 as ::core::ffi::c_int) as UWORD as ::core::ffi::c_int
                    % ((50 as ::core::ffi::c_int * 12 as ::core::ffi::c_int
                        + ((50 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                        >> 1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
                - (50 as ::core::ffi::c_int * 12 as ::core::ffi::c_int
                    + ((50 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                    >> 1 as ::core::ffi::c_int)) as COORD;
            if (enc_pt.y as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                enc_pt.y = 0 as COORD;
            } else if enc_pt.y as ::core::ffi::c_int > 9999 as ::core::ffi::c_int {
                enc_pt.y = 9999 as COORD;
            }
            dx = (enc_pt.x as ::core::ffi::c_int
                - (*EncounterPtr).origin.x as ::core::ffi::c_int) as SIZE;
            dy = (enc_pt.y as ::core::ffi::c_int
                - (*EncounterPtr).origin.y as ::core::ffi::c_int) as SIZE;
            if (dx as ::core::ffi::c_long * dx as ::core::ffi::c_long
                + dy as ::core::ffi::c_long * dy as ::core::ffi::c_long) as DWORD
                <= radius_squared
            {
                break;
            }
        }
        (*EncounterPtr).loc_pt = enc_pt;
        (*EncounterPtr).log_x = universeToLogx(enc_pt.x);
        (*EncounterPtr).log_y = universeToLogy(enc_pt.y);
    }
    hElement = AllocHyperElement(&raw mut enc_pt);
    if !hElement.is_null() {
        let mut i_0: SIZE = 0;
        let mut ElementPtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
        ElementPtr = LockLink(&raw mut disp_q, hElement) as *mut ELEMENT;
        i_0 = (*EncounterPtr).transition_state;
        if i_0 as ::core::ffi::c_int != 0 || NewEncounter.0 != 0 {
            if (i_0 as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                i_0 = -(i_0 as ::core::ffi::c_int) as SIZE;
                (*ElementPtr).c2rust_unnamed_0.hit_points = 1 as COUNT;
            }
            if i_0 as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                || i_0 as ::core::ffi::c_int > 9 as ::core::ffi::c_int
            {
                i_0 = 9 as SIZE;
            }
            (*ElementPtr).current.image.frame = SetRelFrameIndex(
                *(*ElementPtr).current.image.farray.offset(0isize),
                -(i_0 as ::core::ffi::c_int) as SIZE,
            );
            (*ElementPtr).death_func = Some(
                encounter_transition as unsafe extern "C" fn(*mut ELEMENT) -> (),
            ) as Option<ElementProcessFunc>;
        } else {
            (*ElementPtr).current.image.frame = DecFrameIndex(
                *(*ElementPtr).current.image.farray.offset(0isize),
            );
        }
        (*ElementPtr).c2rust_unnamed_2.turn_wait = 1 as BYTE;
        (*ElementPtr).preprocess_func = None;
        (*ElementPtr).postprocess_func = None;
        (*ElementPtr).collision_func = Some(
            encounter_collision
                as unsafe extern "C" fn(
                    *mut ELEMENT,
                    *mut POINT,
                    *mut ELEMENT,
                    *mut POINT,
                ) -> (),
        ) as Option<ElementCollisionFunc>;
        SetUpElement(ElementPtr);
        (*ElementPtr).IntersectControl.IntersectStamp.frame = DecFrameIndex(
            stars_in_space,
        );
        DisplayArray[(*ElementPtr).PrimIndex as usize].Type = gfx_object::NUM_PRIMS.0
            as ::core::ffi::c_int as GRAPHICS_PRIM;
        (*ElementPtr).state_flags = ((*ElementPtr).state_flags as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int))
            as ELEMENT_FLAGS;
        UnlockLink(&raw mut disp_q, hElement);
        InsertQueue(&raw mut disp_q, hElement, disp_q.tail);
    }
    (*EncounterPtr).hElement = hElement;
    return hElement;
}
unsafe extern "C" fn DrawHyperGrid(
    mut ux: COORD,
    mut uy: COORD,
    mut ox: COORD,
    mut oy: COORD,
) {
    let mut sx: COORD = 0;
    let mut sy: COORD = 0;
    let mut ex: COORD = 0;
    let mut ey: COORD = 0;
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    ClearDrawable();
    SetContextForeGroundColor(colorFromRgb15(0 as BYTE, 0x10 as BYTE, 0 as BYTE));
    sx = (ux as ::core::ffi::c_int
        - (63 as ::core::ffi::c_int * 12 as ::core::ffi::c_int
            >> 1 as ::core::ffi::c_int)) as COORD;
    if (sx as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
        sx = 0 as COORD;
    } else {
        sx = (sx as ::core::ffi::c_int
            - sx as ::core::ffi::c_int % 200 as ::core::ffi::c_int) as COORD;
    }
    ex = (ux as ::core::ffi::c_int
        + (63 as ::core::ffi::c_int * 12 as ::core::ffi::c_int
            >> 1 as ::core::ffi::c_int)) as COORD;
    if ex as ::core::ffi::c_int > 9999 as ::core::ffi::c_int + 1 as ::core::ffi::c_int {
        ex = (9999 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as COORD;
    }
    sy = (uy as ::core::ffi::c_int
        - (50 as ::core::ffi::c_int * 12 as ::core::ffi::c_int
            >> 1 as ::core::ffi::c_int)) as COORD;
    if (sy as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
        sy = 0 as COORD;
    } else {
        sy = (sy as ::core::ffi::c_int
            - sy as ::core::ffi::c_int % 200 as ::core::ffi::c_int) as COORD;
    }
    ey = (uy as ::core::ffi::c_int
        + (50 as ::core::ffi::c_int * 12 as ::core::ffi::c_int
            >> 1 as ::core::ffi::c_int)) as COORD;
    if ey as ::core::ffi::c_int > 9999 as ::core::ffi::c_int + 1 as ::core::ffi::c_int {
        ey = (9999 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as COORD;
    }
    r.corner.y = (((9999 as ::core::ffi::c_int - ey as ::core::ffi::c_int)
        as ::core::ffi::c_long * 53 as ::core::ffi::c_long
        / (50 as ::core::ffi::c_int * 12 as ::core::ffi::c_int) as ::core::ffi::c_long)
        as COORD as ::core::ffi::c_int - oy as ::core::ffi::c_int) as COORD;
    r.extent.width = 1 as COORD;
    r.extent.height = (((9999 as ::core::ffi::c_int - sy as ::core::ffi::c_int)
        as ::core::ffi::c_long * 53 as ::core::ffi::c_long
        / (50 as ::core::ffi::c_int * 12 as ::core::ffi::c_int) as ::core::ffi::c_long)
        as COORD as ::core::ffi::c_int - oy as ::core::ffi::c_int
        - r.corner.y as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as COORD;
    ux = sx;
    while ux as ::core::ffi::c_int <= ex as ::core::ffi::c_int {
        r.corner.x = ((ux as ::core::ffi::c_long
            * (64 as ::core::ffi::c_int - 8 as ::core::ffi::c_int) as ::core::ffi::c_long
            / (63 as ::core::ffi::c_int * 12 as ::core::ffi::c_int)
                as ::core::ffi::c_long) as COORD as ::core::ffi::c_int
            - ox as ::core::ffi::c_int) as COORD;
        DrawFilledRectangle(&raw mut r);
        ux = (ux as ::core::ffi::c_int + 200 as ::core::ffi::c_int) as COORD;
    }
    r.corner.x = ((sx as ::core::ffi::c_long
        * (64 as ::core::ffi::c_int - 8 as ::core::ffi::c_int) as ::core::ffi::c_long
        / (63 as ::core::ffi::c_int * 12 as ::core::ffi::c_int) as ::core::ffi::c_long)
        as COORD as ::core::ffi::c_int - ox as ::core::ffi::c_int) as COORD;
    r.extent.width = ((ex as ::core::ffi::c_long
        * (64 as ::core::ffi::c_int - 8 as ::core::ffi::c_int) as ::core::ffi::c_long
        / (63 as ::core::ffi::c_int * 12 as ::core::ffi::c_int) as ::core::ffi::c_long)
        as COORD as ::core::ffi::c_int - ox as ::core::ffi::c_int
        - r.corner.x as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as COORD;
    r.extent.height = 1 as COORD;
    uy = sy;
    while uy as ::core::ffi::c_int <= ey as ::core::ffi::c_int {
        r.corner.y = (((9999 as ::core::ffi::c_int - uy as ::core::ffi::c_int)
            as ::core::ffi::c_long * 53 as ::core::ffi::c_long
            / (50 as ::core::ffi::c_int * 12 as ::core::ffi::c_int)
                as ::core::ffi::c_long) as COORD as ::core::ffi::c_int
            - oy as ::core::ffi::c_int) as COORD;
        DrawFilledRectangle(&raw mut r);
        uy = (uy as ::core::ffi::c_int + 200 as ::core::ffi::c_int) as COORD;
    }
}
unsafe extern "C" fn ProcessEncounter(
    mut EncounterPtr: *mut ENCOUNTER,
    mut puniverse: *mut POINT,
    mut ox: COORD,
    mut oy: COORD,
    mut stamp: *mut STAMP,
) -> bool {
    let mut ElementPtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
    let mut ex: COORD = 0;
    let mut ey: COORD = 0;
    if (*EncounterPtr).hElement.is_null()
        && AddEncounterElement(EncounterPtr, puniverse).is_null()
    {
        return false;
    }
    ElementPtr = LockLink(&raw mut disp_q, (*EncounterPtr).hElement) as *mut ELEMENT;
    if (*ElementPtr).death_func.is_some() {
        if (*EncounterPtr).transition_state as ::core::ffi::c_int != 0
            && (*ElementPtr).c2rust_unnamed_2.turn_wait as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
        {
            (*EncounterPtr).transition_state -= 1;
            if (*EncounterPtr).transition_state as ::core::ffi::c_int
                >= 9 as ::core::ffi::c_int
            {
                (*ElementPtr).c2rust_unnamed_2.turn_wait = (*ElementPtr)
                    .c2rust_unnamed_2
                    .turn_wait
                    .wrapping_add(1);
            } else if (*EncounterPtr).transition_state as ::core::ffi::c_int
                == -9 as ::core::ffi::c_int
            {
                (*ElementPtr).death_func = None;
                UnlockLink(&raw mut disp_q, (*EncounterPtr).hElement);
                return false;
            } else {
                DisplayArray[(*ElementPtr).PrimIndex as usize].Type = gfx_object::STAMP_PRIM
                    .0 as ::core::ffi::c_int as GRAPHICS_PRIM;
            }
        }
    } else {
        let mut delta_x: SIZE = 0;
        let mut delta_y: SIZE = 0;
        let mut encounter_radius: COUNT = 0;
        (*ElementPtr).c2rust_unnamed.life_span = 1 as COUNT;
        GetNextVelocityComponents(
            &raw mut (*ElementPtr).velocity,
            &raw mut delta_x,
            &raw mut delta_y,
            1 as COUNT,
        );
        if (*ElementPtr).c2rust_unnamed_3.thrust_wait != 0 {
            (*ElementPtr).c2rust_unnamed_3.thrust_wait = (*ElementPtr)
                .c2rust_unnamed_3
                .thrust_wait
                .wrapping_sub(1);
        } else if (*ElementPtr).hTarget.is_null() {
            let mut cur_facing: COUNT = 0;
            let mut delta_facing: COUNT = 0;
            cur_facing = ((*ElementPtr).velocity.TravelAngle as ::core::ffi::c_int
                + ((1 as ::core::ffi::c_int)
                    << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int)
                >> 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int) as COUNT;
            delta_facing = (cur_facing as ::core::ffi::c_int
                - (ARCTAN(
                    ((*puniverse).x as ::core::ffi::c_int
                        - (*EncounterPtr).loc_pt.x as ::core::ffi::c_int) as SIZE,
                    ((*puniverse).y as ::core::ffi::c_int
                        - (*EncounterPtr).loc_pt.y as ::core::ffi::c_int) as SIZE,
                ) as ::core::ffi::c_int
                    + ((1 as ::core::ffi::c_int)
                        << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int)
                    >> 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int)
                & ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int) as COUNT;
            if delta_facing as ::core::ffi::c_int != 0
                || delta_x as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                    && delta_y as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            {
                let mut speed: SIZE = 0;
                let RaceHyperSpeed: [SIZE; 23] = [
                    ((40 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
                        >> 1 as ::core::ffi::c_int) as SIZE,
                    ((27 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
                        >> 1 as ::core::ffi::c_int) as SIZE,
                    ((24 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
                        >> 1 as ::core::ffi::c_int) as SIZE,
                    ((40 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
                        >> 1 as ::core::ffi::c_int) as SIZE,
                    ((40 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
                        >> 1 as ::core::ffi::c_int) as SIZE,
                    ((35 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
                        >> 1 as ::core::ffi::c_int) as SIZE,
                    ((48 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
                        >> 1 as ::core::ffi::c_int) as SIZE,
                    ((40 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
                        >> 1 as ::core::ffi::c_int) as SIZE,
                    ((50 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
                        >> 1 as ::core::ffi::c_int) as SIZE,
                    ((30 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
                        >> 1 as ::core::ffi::c_int) as SIZE,
                    ((21 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
                        >> 1 as ::core::ffi::c_int) as SIZE,
                    ((30 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
                        >> 1 as ::core::ffi::c_int) as SIZE,
                    ((40 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
                        >> 1 as ::core::ffi::c_int) as SIZE,
                    ((20 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
                        >> 1 as ::core::ffi::c_int) as SIZE,
                    ((25 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
                        >> 1 as ::core::ffi::c_int) as SIZE,
                    ((27 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
                        >> 1 as ::core::ffi::c_int) as SIZE,
                    ((60 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
                        >> 1 as ::core::ffi::c_int) as SIZE,
                    ((18 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
                        >> 1 as ::core::ffi::c_int) as SIZE,
                    ((30 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
                        >> 1 as ::core::ffi::c_int) as SIZE,
                    ((40 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
                        >> 1 as ::core::ffi::c_int) as SIZE,
                    ((36 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
                        >> 1 as ::core::ffi::c_int) as SIZE,
                    ((30 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
                        >> 1 as ::core::ffi::c_int) as SIZE,
                    ((30 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
                        >> 1 as ::core::ffi::c_int) as SIZE,
                ];
                speed = RaceHyperSpeed[(*EncounterPtr).race_id as usize];
                if (delta_facing as ::core::ffi::c_int)
                    < ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                        >> 1 as ::core::ffi::c_int)
                        + ((1 as ::core::ffi::c_int)
                            << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int)
                        >> 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                {
                    cur_facing = cur_facing.wrapping_sub(1);
                } else {
                    cur_facing = cur_facing.wrapping_add(1);
                }
                if (delta_facing as ::core::ffi::c_int
                    + (((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                        >> 3 as ::core::ffi::c_int)
                        + ((1 as ::core::ffi::c_int)
                            << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int)
                        >> 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int)
                    & ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int) as COUNT as ::core::ffi::c_int
                    > ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                        >> 2 as ::core::ffi::c_int)
                        + ((1 as ::core::ffi::c_int)
                            << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int)
                        >> 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                {
                    if (delta_facing as ::core::ffi::c_int)
                        < ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                            >> 1 as ::core::ffi::c_int)
                            + ((1 as ::core::ffi::c_int)
                                << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                                    - 1 as ::core::ffi::c_int)
                            >> 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                    {
                        cur_facing = cur_facing.wrapping_sub(1);
                    } else {
                        cur_facing = cur_facing.wrapping_add(1);
                    }
                    speed = (speed as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
                        as SIZE;
                }
                cur_facing = ((cur_facing as ::core::ffi::c_int)
                    << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int) as COUNT;
                SetVelocityComponents(
                    &raw mut (*ElementPtr).velocity,
                    (*(&raw mut sinetab as *mut SIZE)
                        .offset(
                            (cur_facing as ::core::ffi::c_int
                                + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                                    >> 2 as ::core::ffi::c_int)
                                & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                                    - 1 as ::core::ffi::c_int) as COUNT as isize,
                        ) as ::core::ffi::c_long * speed as ::core::ffi::c_long
                        >> 14 as ::core::ffi::c_int) as SIZE,
                    (*(&raw mut sinetab as *mut SIZE)
                        .offset(
                            (cur_facing as ::core::ffi::c_int
                                & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                                    - 1 as ::core::ffi::c_int) as COUNT as isize,
                        ) as ::core::ffi::c_long * speed as ::core::ffi::c_long
                        >> 14 as ::core::ffi::c_int) as SIZE,
                );
                GetNextVelocityComponents(
                    &raw mut (*ElementPtr).velocity,
                    &raw mut delta_x,
                    &raw mut delta_y,
                    1 as COUNT,
                );
                (*ElementPtr).c2rust_unnamed_3.thrust_wait = 3 as BYTE;
            }
        }
        (*EncounterPtr).log_x += delta_x as ::core::ffi::c_int;
        (*EncounterPtr).log_y -= delta_y as ::core::ffi::c_int;
        (*EncounterPtr).loc_pt.x = logxToUniverse((*EncounterPtr).log_x);
        (*EncounterPtr).loc_pt.y = logyToUniverse((*EncounterPtr).log_y);
        encounter_radius = ((*EncounterPtr).radius as ::core::ffi::c_int
            + (200 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)) as COUNT;
        delta_x = ((*EncounterPtr).loc_pt.x as ::core::ffi::c_int
            - (*EncounterPtr).origin.x as ::core::ffi::c_int) as SIZE;
        if (delta_x as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
            delta_x = -(delta_x as ::core::ffi::c_int) as SIZE;
        }
        delta_y = ((*EncounterPtr).loc_pt.y as ::core::ffi::c_int
            - (*EncounterPtr).origin.y as ::core::ffi::c_int) as SIZE;
        if (delta_y as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
            delta_y = -(delta_y as ::core::ffi::c_int) as SIZE;
        }
        if delta_x as COUNT as ::core::ffi::c_int
            >= encounter_radius as ::core::ffi::c_int
            || delta_y as COUNT as ::core::ffi::c_int
                >= encounter_radius as ::core::ffi::c_int
            || (delta_x as DWORD)
                .wrapping_mul(delta_x as DWORD)
                .wrapping_add((delta_y as DWORD).wrapping_mul(delta_y as DWORD))
                >= (encounter_radius as DWORD).wrapping_mul(encounter_radius as DWORD)
        {
            (*ElementPtr).state_flags = ((*ElementPtr).state_flags as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as ELEMENT_FLAGS;
            (*ElementPtr).c2rust_unnamed.life_span = 0 as COUNT;
            if (*EncounterPtr).transition_state as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
            {
                (*ElementPtr).death_func = Some(
                    encounter_transition as unsafe extern "C" fn(*mut ELEMENT) -> (),
                ) as Option<ElementProcessFunc>;
                (*EncounterPtr).transition_state = -1 as SIZE;
                (*ElementPtr).c2rust_unnamed_0.hit_points = 1 as COUNT;
            } else {
                (*ElementPtr).death_func = None;
                UnlockLink(&raw mut disp_q, (*EncounterPtr).hElement);
                return false;
            }
        }
    }
    ex = (*EncounterPtr).loc_pt.x;
    ey = (*EncounterPtr).loc_pt.y;
    if ex as ::core::ffi::c_int - (*puniverse).x as ::core::ffi::c_int
        >= -63 as ::core::ffi::c_int
        && ex as ::core::ffi::c_int - (*puniverse).x as ::core::ffi::c_int
            <= 63 as ::core::ffi::c_int
        && ey as ::core::ffi::c_int - (*puniverse).y as ::core::ffi::c_int
            >= -50 as ::core::ffi::c_int
        && ey as ::core::ffi::c_int - (*puniverse).y as ::core::ffi::c_int
            <= 50 as ::core::ffi::c_int
    {
        (*ElementPtr).next.location.x = (((*EncounterPtr).log_x
            - GlobData.SIS_state.log_x) as SIZE as ::core::ffi::c_int
            + (((ScreenWidth - 64 as ::core::ffi::c_int
                - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int)) as COORD;
        (*ElementPtr).next.location.y = (((*EncounterPtr).log_y
            - GlobData.SIS_state.log_y) as SIZE as ::core::ffi::c_int
            + (((ScreenHeight - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int)) as COORD;
        if (*ElementPtr).state_flags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int != 0
            && (*EncounterPtr).transition_state as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
        {
            (*ElementPtr).current.location = (*ElementPtr).next.location;
            DisplayArray[(*ElementPtr).PrimIndex as usize].Type = gfx_object::STAMP_PRIM
                .0 as ::core::ffi::c_int as GRAPHICS_PRIM;
            if (*ElementPtr).death_func.is_none() {
                (*ElementPtr).IntersectControl.IntersectStamp.origin.x = ((*ElementPtr)
                    .current
                    .location
                    .x as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as COORD;
                (*ElementPtr).IntersectControl.IntersectStamp.origin.y = ((*ElementPtr)
                    .current
                    .location
                    .y as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as COORD;
                (*ElementPtr).state_flags = ((*ElementPtr).state_flags
                    as ::core::ffi::c_int
                    & !((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int))
                    as ELEMENT_FLAGS;
            }
        }
    } else {
        (*ElementPtr).state_flags = ((*ElementPtr).state_flags as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as ELEMENT_FLAGS;
        if (ex as ::core::ffi::c_int - (*puniverse).x as ::core::ffi::c_int)
            < -(63 as ::core::ffi::c_int * 12 as ::core::ffi::c_int
                + ((63 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                >> 1 as ::core::ffi::c_int)
            || ex as ::core::ffi::c_int - (*puniverse).x as ::core::ffi::c_int
                > 63 as ::core::ffi::c_int * 12 as ::core::ffi::c_int
                    + ((63 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                    >> 1 as ::core::ffi::c_int
            || (ey as ::core::ffi::c_int - (*puniverse).y as ::core::ffi::c_int)
                < -(50 as ::core::ffi::c_int * 12 as ::core::ffi::c_int
                    + ((50 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                    >> 1 as ::core::ffi::c_int)
            || ey as ::core::ffi::c_int - (*puniverse).y as ::core::ffi::c_int
                > 50 as ::core::ffi::c_int * 12 as ::core::ffi::c_int
                    + ((50 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                    >> 1 as ::core::ffi::c_int
        {
            (*ElementPtr).c2rust_unnamed.life_span = 0 as COUNT;
            (*ElementPtr).death_func = None;
            UnlockLink(&raw mut disp_q, (*EncounterPtr).hElement);
            return false;
        }
        DisplayArray[(*ElementPtr).PrimIndex as usize].Type = gfx_object::NUM_PRIMS.0
            as ::core::ffi::c_int as GRAPHICS_PRIM;
    }
    UnlockLink(&raw mut disp_q, (*EncounterPtr).hElement);
    (*stamp).origin.x = ((ex as ::core::ffi::c_long
        * (64 as ::core::ffi::c_int - 8 as ::core::ffi::c_int) as ::core::ffi::c_long
        / (63 as ::core::ffi::c_int * 12 as ::core::ffi::c_int) as ::core::ffi::c_long)
        as COORD as ::core::ffi::c_int - ox as ::core::ffi::c_int) as COORD;
    (*stamp).origin.y = (((9999 as ::core::ffi::c_int - ey as ::core::ffi::c_int)
        as ::core::ffi::c_long * 53 as ::core::ffi::c_long
        / (50 as ::core::ffi::c_int * 12 as ::core::ffi::c_int) as ::core::ffi::c_long)
        as COORD as ::core::ffi::c_int - oy as ::core::ffi::c_int) as COORD;
    DrawStamp(stamp);
    return true;
}
unsafe extern "C" fn ProcessEncounters(
    mut puniverse: *mut POINT,
    mut ox: COORD,
    mut oy: COORD,
) {
    let mut stamp: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    let mut hEncounter: HENCOUNTER = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut hNextEncounter: HENCOUNTER = ::core::ptr::null_mut::<::core::ffi::c_void>();
    stamp.frame = SetAbsFrameIndex(stars_in_space, 91 as COUNT);
    hEncounter = GlobData.Game_state.encounter_q.head as HENCOUNTER;
    while !hEncounter.is_null() {
        let mut EncounterPtr: *mut ENCOUNTER = ::core::ptr::null_mut::<ENCOUNTER>();
        EncounterPtr = LockLink(&raw mut GlobData.Game_state.encounter_q, hEncounter)
            as *mut ENCOUNTER;
        hNextEncounter = (*EncounterPtr).succ;
        if !ProcessEncounter(EncounterPtr, puniverse, ox, oy, &raw mut stamp) {
            UnlockLink(&raw mut GlobData.Game_state.encounter_q, hEncounter);
            RemoveQueue(&raw mut GlobData.Game_state.encounter_q, hEncounter);
            FreeLink(&raw mut GlobData.Game_state.encounter_q, hEncounter);
        } else {
            UnlockLink(&raw mut GlobData.Game_state.encounter_q, hEncounter);
        }
        hEncounter = hNextEncounter;
    }
}
#[no_mangle]
pub unsafe extern "C" fn SeedUniverse() {
    let mut ox: COORD = 0;
    let mut oy: COORD = 0;
    let mut sx: COORD = 0;
    let mut sy: COORD = 0;
    let mut ex: COORD = 0;
    let mut ey: COORD = 0;
    let mut portalCounter: SWORD = 0;
    let mut arilouSpaceCounter: SWORD = 0;
    let mut arilouSpaceSide: SWORD = 0;
    let mut universe: POINT = POINT { x: 0, y: 0 };
    let mut blip_frame: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
    let mut s: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    let mut SDPtr: *mut STAR_DESC = ::core::ptr::null_mut::<STAR_DESC>();
    let mut hHyperSpaceElement: HELEMENT = ::core::ptr::null_mut::<
        ::core::ffi::c_void,
    >();
    let mut HyperSpaceElementPtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
    universe.x = logxToUniverse(GlobData.SIS_state.log_x);
    universe.y = logyToUniverse(GlobData.SIS_state.log_y);
    blip_frame = SetAbsFrameIndex(stars_in_space, 90 as COUNT);
    SetContext(RadarContext);
    BatchGraphics();
    ox = ((universe.x as ::core::ffi::c_long
        * (64 as ::core::ffi::c_int - 8 as ::core::ffi::c_int) as ::core::ffi::c_long
        / (63 as ::core::ffi::c_int * 12 as ::core::ffi::c_int) as ::core::ffi::c_long)
        as COORD as ::core::ffi::c_int
        - (64 as ::core::ffi::c_int - 8 as ::core::ffi::c_int
            >> 1 as ::core::ffi::c_int)) as COORD;
    oy = (((9999 as ::core::ffi::c_int - universe.y as ::core::ffi::c_int)
        as ::core::ffi::c_long * 53 as ::core::ffi::c_long
        / (50 as ::core::ffi::c_int * 12 as ::core::ffi::c_int) as ::core::ffi::c_long)
        as COORD as ::core::ffi::c_int
        - (53 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)) as COORD;
    ex = ((GlobData.Game_state.ShipStamp.origin.x as ::core::ffi::c_long
        * (64 as ::core::ffi::c_int - 8 as ::core::ffi::c_int) as ::core::ffi::c_long
        / (63 as ::core::ffi::c_int * 12 as ::core::ffi::c_int) as ::core::ffi::c_long)
        as COORD as ::core::ffi::c_int
        - (64 as ::core::ffi::c_int - 8 as ::core::ffi::c_int
            >> 1 as ::core::ffi::c_int)) as COORD;
    ey = (((9999 as ::core::ffi::c_int
        - GlobData.Game_state.ShipStamp.origin.y as ::core::ffi::c_int)
        as ::core::ffi::c_long * 53 as ::core::ffi::c_long
        / (50 as ::core::ffi::c_int * 12 as ::core::ffi::c_int) as ::core::ffi::c_long)
        as COORD as ::core::ffi::c_int
        - (53 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)) as COORD;
    arilouSpaceCounter = getGameState(
        &raw mut GlobData.Game_state.GameState as *mut BYTE,
        C2Rust_Unnamed_8::ARILOU_SPACE_COUNTER.0 as ::core::ffi::c_int,
        C2Rust_Unnamed_8::END_ARILOU_SPACE_COUNTER.0 as ::core::ffi::c_int,
    ) as SWORD;
    arilouSpaceSide = getGameState(
        &raw mut GlobData.Game_state.GameState as *mut BYTE,
        C2Rust_Unnamed_8::ARILOU_SPACE_SIDE.0 as ::core::ffi::c_int,
        C2Rust_Unnamed_8::END_ARILOU_SPACE_SIDE.0 as ::core::ffi::c_int,
    ) as SWORD;
    DrawHyperGrid(universe.x, universe.y, ox, oy);
    SDPtr = ::core::ptr::null_mut::<STAR_DESC>();
    loop {
        SDPtr = FindStar(
            SDPtr,
            &raw mut universe,
            (63 as ::core::ffi::c_int * 12 as ::core::ffi::c_int
                + ((63 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                >> 1 as ::core::ffi::c_int) as SIZE,
            (50 as ::core::ffi::c_int * 12 as ::core::ffi::c_int
                + ((50 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                >> 1 as ::core::ffi::c_int) as SIZE,
        );
        if SDPtr.is_null() {
            break;
        }
        let mut star_type: BYTE = 0;
        ex = (*SDPtr).star_pt.x;
        ey = (*SDPtr).star_pt.y;
        star_type = (((*SDPtr).Type as ::core::ffi::c_int
            & ((0xff as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                & !(((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as BYTE
                    as ::core::ffi::c_int)) as BYTE as ::core::ffi::c_int)
            >> 3 as ::core::ffi::c_int) as BYTE;
        if arilouSpaceSide as ::core::ffi::c_int >= 2 as ::core::ffi::c_int
            && ex as ::core::ffi::c_int
                == 5000 as ::core::ffi::c_int
                    + (63 as ::core::ffi::c_int * 12 as ::core::ffi::c_int
                        >> 1 as ::core::ffi::c_int) * 3 as ::core::ffi::c_int
            && ey as ::core::ffi::c_int
                == 5000 as ::core::ffi::c_int
                    + (50 as ::core::ffi::c_int * 12 as ::core::ffi::c_int
                        >> 1 as ::core::ffi::c_int) * 3 as ::core::ffi::c_int
        {
            star_type = C2Rust_Unnamed_10::SUPER_GIANT_STAR.0 as ::core::ffi::c_int
                as BYTE;
        }
        s.origin.x = ((ex as ::core::ffi::c_long
            * (64 as ::core::ffi::c_int - 8 as ::core::ffi::c_int) as ::core::ffi::c_long
            / (63 as ::core::ffi::c_int * 12 as ::core::ffi::c_int)
                as ::core::ffi::c_long) as COORD as ::core::ffi::c_int
            - ox as ::core::ffi::c_int) as COORD;
        s.origin.y = (((9999 as ::core::ffi::c_int - ey as ::core::ffi::c_int)
            as ::core::ffi::c_long * 53 as ::core::ffi::c_long
            / (50 as ::core::ffi::c_int * 12 as ::core::ffi::c_int)
                as ::core::ffi::c_long) as COORD as ::core::ffi::c_int
            - oy as ::core::ffi::c_int) as COORD;
        s.frame = SetRelFrameIndex(
            blip_frame,
            (star_type as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as SIZE,
        );
        DrawStamp(&raw mut s);
    }
    portalCounter = getGameState(
        &raw mut GlobData.Game_state.GameState as *mut BYTE,
        C2Rust_Unnamed_8::PORTAL_COUNTER.0 as ::core::ffi::c_int,
        C2Rust_Unnamed_8::END_PORTAL_COUNTER.0 as ::core::ffi::c_int,
    ) as SWORD;
    if portalCounter as ::core::ffi::c_int != 0
        || arilouSpaceCounter as ::core::ffi::c_int != 0
    {
        let mut i: COUNT = 0;
        let mut SD: [STAR_DESC; 2] = [STAR_DESC {
            star_pt: POINT { x: 0, y: 0 },
            Type: 0,
            Index: 0,
            Prefix: 0,
            Postfix: 0,
        }; 2];
        i = 0 as COUNT;
        if portalCounter != 0 {
            static mut portal_pt: POINT = POINT { x: 0, y: 0 };
            SD[i as usize].Index = ((portalCounter as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
                + 18 as ::core::ffi::c_int) as BYTE;
            if portalCounter as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
                portal_pt = universe;
            }
            SD[i as usize].star_pt = portal_pt;
            i = i.wrapping_add(1);
            portalCounter += 1;
            if portalCounter as ::core::ffi::c_int
                == 10 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
            {
                portalCounter = (9 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                    as SWORD;
            }
            setGameState(
                &raw mut GlobData.Game_state.GameState as *mut BYTE,
                C2Rust_Unnamed_8::PORTAL_COUNTER.0 as ::core::ffi::c_int,
                C2Rust_Unnamed_8::END_PORTAL_COUNTER.0 as ::core::ffi::c_int,
                portalCounter as BYTE,
            );
        }
        if arilouSpaceCounter != 0 {
            SD[i as usize].Index = (arilouSpaceCounter as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int) as BYTE;
            if arilouSpaceSide as ::core::ffi::c_int <= 1 as ::core::ffi::c_int {
                SD[i as usize].Index = (SD[i as usize].Index as ::core::ffi::c_int
                    + 18 as ::core::ffi::c_int) as BYTE;
                SD[i as usize].star_pt.x = 438 as COORD;
                SD[i as usize].star_pt.y = 6372 as COORD;
            } else {
                SD[i as usize].star_pt.x = 5000 as COORD;
                SD[i as usize].star_pt.y = 5000 as COORD;
            }
            i = i.wrapping_add(1);
        }
        loop {
            i = i.wrapping_sub(1);
            sx = (SD[i as usize].star_pt.x as ::core::ffi::c_int
                - universe.x as ::core::ffi::c_int
                + (63 as ::core::ffi::c_int * 12 as ::core::ffi::c_int
                    + ((63 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                    >> 1 as ::core::ffi::c_int)) as COORD;
            sy = (SD[i as usize].star_pt.y as ::core::ffi::c_int
                - universe.y as ::core::ffi::c_int
                + (50 as ::core::ffi::c_int * 12 as ::core::ffi::c_int
                    + ((50 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                    >> 1 as ::core::ffi::c_int)) as COORD;
            if !((sx as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
                || (sy as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
                || sx as ::core::ffi::c_int
                    >= (63 as ::core::ffi::c_int * 12 as ::core::ffi::c_int
                        + ((63 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                        >> 1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                || sy as ::core::ffi::c_int
                    >= (50 as ::core::ffi::c_int * 12 as ::core::ffi::c_int
                        + ((50 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                        >> 1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
            {
                ex = SD[i as usize].star_pt.x;
                ey = SD[i as usize].star_pt.y;
                s.origin.x = ((ex as ::core::ffi::c_long
                    * (64 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)
                        as ::core::ffi::c_long
                    / (63 as ::core::ffi::c_int * 12 as ::core::ffi::c_int)
                        as ::core::ffi::c_long) as COORD as ::core::ffi::c_int
                    - ox as ::core::ffi::c_int) as COORD;
                s.origin.y = (((9999 as ::core::ffi::c_int - ey as ::core::ffi::c_int)
                    as ::core::ffi::c_long * 53 as ::core::ffi::c_long
                    / (50 as ::core::ffi::c_int * 12 as ::core::ffi::c_int)
                        as ::core::ffi::c_long) as COORD as ::core::ffi::c_int
                    - oy as ::core::ffi::c_int) as COORD;
                s.frame = SetAbsFrameIndex(stars_in_space, 95 as COUNT);
                DrawStamp(&raw mut s);
                ex = (ex as ::core::ffi::c_int - universe.x as ::core::ffi::c_int)
                    as COORD;
                if (ex as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                    ex = -(ex as ::core::ffi::c_int) as COORD;
                }
                ey = (ey as ::core::ffi::c_int - universe.y as ::core::ffi::c_int)
                    as COORD;
                if (ey as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                    ey = -(ey as ::core::ffi::c_int) as COORD;
                }
                if !(ex as ::core::ffi::c_int
                    > (63 as ::core::ffi::c_int * 12 as ::core::ffi::c_int
                        + ((63 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                        >> 1 as ::core::ffi::c_int) / 12 as ::core::ffi::c_int
                    || ey as ::core::ffi::c_int
                        > (50 as ::core::ffi::c_int * 12 as ::core::ffi::c_int
                            + ((50 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                            >> 1 as ::core::ffi::c_int) / 12 as ::core::ffi::c_int)
                {
                    hHyperSpaceElement = AllocHyperElement(
                        &raw mut (*(&raw mut SD as *mut STAR_DESC).offset(i as isize))
                            .star_pt,
                    );
                    if !hHyperSpaceElement.is_null() {
                        HyperSpaceElementPtr = LockLink(
                            &raw mut disp_q,
                            hHyperSpaceElement,
                        ) as *mut ELEMENT;
                        (*HyperSpaceElementPtr).current.image.frame = SetAbsFrameIndex(
                            hyperstars[(1 as ::core::ffi::c_int
                                + (getGameState(
                                    &raw mut GlobData.Game_state.GameState as *mut BYTE,
                                    C2Rust_Unnamed_8::ARILOU_SPACE_SIDE.0 as ::core::ffi::c_int,
                                    C2Rust_Unnamed_8::END_ARILOU_SPACE_SIDE.0
                                        as ::core::ffi::c_int,
                                ) as ::core::ffi::c_int >> 1 as ::core::ffi::c_int))
                                as usize],
                            SD[i as usize].Index as COUNT,
                        );
                        (*HyperSpaceElementPtr).preprocess_func = None;
                        (*HyperSpaceElementPtr).postprocess_func = None;
                        (*HyperSpaceElementPtr).collision_func = Some(
                            arilou_space_collision
                                as unsafe extern "C" fn(
                                    *mut ELEMENT,
                                    *mut POINT,
                                    *mut ELEMENT,
                                    *mut POINT,
                                ) -> (),
                        ) as Option<ElementCollisionFunc>;
                        SetUpElement(HyperSpaceElementPtr);
                        if arilouSpaceSide as ::core::ffi::c_int
                            == 1 as ::core::ffi::c_int
                            || arilouSpaceSide as ::core::ffi::c_int
                                == 2 as ::core::ffi::c_int
                        {
                            (*HyperSpaceElementPtr).death_func = Some(
                                arilou_space_death
                                    as unsafe extern "C" fn(*mut ELEMENT) -> (),
                            ) as Option<ElementProcessFunc>;
                        } else {
                            (*HyperSpaceElementPtr).death_func = None;
                            (*HyperSpaceElementPtr)
                                .IntersectControl
                                .IntersectStamp
                                .frame = DecFrameIndex(stars_in_space);
                        }
                        UnlockLink(&raw mut disp_q, hHyperSpaceElement);
                        InsertQueue(&raw mut disp_q, hHyperSpaceElement, disp_q.head);
                    }
                }
            }
            if i == 0 {
                break;
            }
        }
    }
    SDPtr = ::core::ptr::null_mut::<STAR_DESC>();
    loop {
        SDPtr = FindStar(
            SDPtr,
            &raw mut universe,
            (63 as ::core::ffi::c_int * 12 as ::core::ffi::c_int
                + ((63 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                >> 1 as ::core::ffi::c_int) as SIZE,
            (50 as ::core::ffi::c_int * 12 as ::core::ffi::c_int
                + ((50 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                >> 1 as ::core::ffi::c_int) as SIZE,
        );
        if SDPtr.is_null() {
            break;
        }
        let mut star_type_0: BYTE = 0;
        ex = ((*SDPtr).star_pt.x as ::core::ffi::c_int
            - universe.x as ::core::ffi::c_int) as COORD;
        if (ex as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
            ex = -(ex as ::core::ffi::c_int) as COORD;
        }
        ey = ((*SDPtr).star_pt.y as ::core::ffi::c_int
            - universe.y as ::core::ffi::c_int) as COORD;
        if (ey as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
            ey = -(ey as ::core::ffi::c_int) as COORD;
        }
        if ex as ::core::ffi::c_int
            > (63 as ::core::ffi::c_int * 12 as ::core::ffi::c_int
                + ((63 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                >> 1 as ::core::ffi::c_int) / 12 as ::core::ffi::c_int
            || ey as ::core::ffi::c_int
                > (50 as ::core::ffi::c_int * 12 as ::core::ffi::c_int
                    + ((50 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                    >> 1 as ::core::ffi::c_int) / 12 as ::core::ffi::c_int
        {
            continue;
        }
        hHyperSpaceElement = AllocHyperElement(&raw mut (*SDPtr).star_pt);
        if hHyperSpaceElement.is_null() {
            continue;
        }
        star_type_0 = (*SDPtr).Type;
        HyperSpaceElementPtr = LockLink(&raw mut disp_q, hHyperSpaceElement)
            as *mut ELEMENT;
        (*HyperSpaceElementPtr).current.image.frame = SetAbsFrameIndex(
            hyperstars[(1 as ::core::ffi::c_int
                + (getGameState(
                    &raw mut GlobData.Game_state.GameState as *mut BYTE,
                    C2Rust_Unnamed_8::ARILOU_SPACE_SIDE.0 as ::core::ffi::c_int,
                    C2Rust_Unnamed_8::END_ARILOU_SPACE_SIDE.0 as ::core::ffi::c_int,
                ) as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)) as usize],
            (((star_type_0 as ::core::ffi::c_int
                & ((0xff as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    & !(((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as BYTE
                        as ::core::ffi::c_int)) as BYTE as ::core::ffi::c_int)
                >> 3 as ::core::ffi::c_int) as BYTE as ::core::ffi::c_int
                * C2Rust_Unnamed_11::NUM_STAR_COLORS.0 as ::core::ffi::c_int
                + ((star_type_0 as ::core::ffi::c_int
                    & ((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as BYTE
                        as ::core::ffi::c_int) >> 5 as ::core::ffi::c_int) as BYTE
                    as ::core::ffi::c_int) as COUNT,
        );
        (*HyperSpaceElementPtr).preprocess_func = None;
        (*HyperSpaceElementPtr).postprocess_func = None;
        (*HyperSpaceElementPtr).collision_func = Some(
            hyper_collision
                as unsafe extern "C" fn(
                    *mut ELEMENT,
                    *mut POINT,
                    *mut ELEMENT,
                    *mut POINT,
                ) -> (),
        ) as Option<ElementCollisionFunc>;
        SetUpElement(HyperSpaceElementPtr);
        if SDPtr == CurStarDescPtr
            && getGameState(
                &raw mut GlobData.Game_state.GameState as *mut BYTE,
                C2Rust_Unnamed_8::PORTAL_COUNTER.0 as ::core::ffi::c_int,
                C2Rust_Unnamed_8::END_PORTAL_COUNTER.0 as ::core::ffi::c_int,
            ) as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        {
            (*HyperSpaceElementPtr).death_func = Some(
                hyper_death as unsafe extern "C" fn(*mut ELEMENT) -> (),
            ) as Option<ElementProcessFunc>;
        } else {
            (*HyperSpaceElementPtr).death_func = None;
            (*HyperSpaceElementPtr).IntersectControl.IntersectStamp.frame = DecFrameIndex(
                stars_in_space,
            );
        }
        UnlockLink(&raw mut disp_q, hHyperSpaceElement);
        InsertQueue(&raw mut disp_q, hHyperSpaceElement, disp_q.head);
    }
    ProcessEncounters(&raw mut universe, ox, oy);
    s.origin.x = (64 as ::core::ffi::c_int - 8 as ::core::ffi::c_int
        >> 1 as ::core::ffi::c_int) as COORD;
    s.origin.y = (53 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as COORD;
    s.frame = blip_frame;
    DrawStamp(&raw mut s);
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    SetContextForeGroundColor(colorFromRgb15(0xe as BYTE, 0xe as BYTE, 0xe as BYTE));
    r.corner.x = 0 as COORD;
    r.corner.y = 0 as COORD;
    r.extent.width = (64 as ::core::ffi::c_int - 8 as ::core::ffi::c_int
        - 1 as ::core::ffi::c_int) as COORD;
    r.extent.height = 1 as COORD;
    DrawFilledRectangle(&raw mut r);
    r.extent.width = 1 as COORD;
    r.extent.height = (53 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as COORD;
    DrawFilledRectangle(&raw mut r);
    SetContextForeGroundColor(colorFromRgb15(0x6 as BYTE, 0x6 as BYTE, 0x6 as BYTE));
    r.corner.x = (64 as ::core::ffi::c_int - 8 as ::core::ffi::c_int
        - 1 as ::core::ffi::c_int) as COORD;
    r.corner.y = 1 as COORD;
    r.extent.height = (53 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as COORD;
    DrawFilledRectangle(&raw mut r);
    r.corner.x = 1 as COORD;
    r.corner.y = (53 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as COORD;
    r.extent.width = (64 as ::core::ffi::c_int - 8 as ::core::ffi::c_int
        - 2 as ::core::ffi::c_int) as COORD;
    r.extent.height = 1 as COORD;
    DrawFilledRectangle(&raw mut r);
    SetContextForeGroundColor(colorFromRgb15(0x8 as BYTE, 0x8 as BYTE, 0x8 as BYTE));
    r.corner.x = 0 as COORD;
    DrawPoint(&raw mut r.corner);
    r.corner.x = (64 as ::core::ffi::c_int - 8 as ::core::ffi::c_int
        - 1 as ::core::ffi::c_int) as COORD;
    r.corner.y = 0 as COORD;
    DrawPoint(&raw mut r.corner);
    UnbatchGraphics();
    SetContext(StatusContext);
    if TFB_Random() as UWORD as ::core::ffi::c_int & 7 as ::core::ffi::c_int == 0 {
        AddAmbientElement();
    }
    if universe.x as ::core::ffi::c_int
        != GlobData.Game_state.ShipStamp.origin.x as ::core::ffi::c_int
        || universe.y as ::core::ffi::c_int
            != GlobData.Game_state.ShipStamp.origin.y as ::core::ffi::c_int
    {
        GlobData.Game_state.ShipStamp.origin = universe;
        DrawHyperCoords(universe);
    }
}
unsafe extern "C" fn DoHyperspaceMenu(mut pMS: *mut MENU_STATE) -> BOOLEAN {
    let mut select: BOOLEAN = BOOLEAN(
        PulsedInputState
            .menu[C2Rust_Unnamed_6::KEY_MENU_SELECT.0 as ::core::ffi::c_int as usize]
            as ::core::ffi::c_uint,
    );
    let mut handled: BOOLEAN = BOOLEAN::FALSE;
    if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
        & (C2Rust_Unnamed_9::CHECK_ABORT.0 as ::core::ffi::c_int
            | C2Rust_Unnamed_9::CHECK_LOAD.0 as ::core::ffi::c_int) != 0
        || GlobData.SIS_state.CrewEnlisted as ::core::ffi::c_int
            == !(0 as ::core::ffi::c_int) as COUNT as ::core::ffi::c_int
    {
        return BOOLEAN::FALSE;
    }
    handled = DoMenuChooser(
        pMS,
        C2Rust_Unnamed_7::PM_STARMAP.0 as ::core::ffi::c_int as BYTE,
    );
    if handled.0 != 0 {
        return BOOLEAN::TRUE;
    }
    if select.0 == 0 {
        return BOOLEAN::TRUE;
    }
    SetFlashRect(::core::ptr::null::<RECT>());
    match (*pMS).CurState as ::core::ffi::c_int {
        2 => {
            select = DevicesMenu();
            if getGameState(
                &raw mut GlobData.Game_state.GameState as *mut BYTE,
                C2Rust_Unnamed_8::PORTAL_COUNTER.0 as ::core::ffi::c_int,
                C2Rust_Unnamed_8::END_PORTAL_COUNTER.0 as ::core::ffi::c_int,
            ) != 0
            {
                return BOOLEAN::FALSE;
            }
            if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
                & C2Rust_Unnamed_9::START_ENCOUNTER.0 as ::core::ffi::c_int != 0
            {
                return BOOLEAN::FALSE;
            }
        }
        3 => {
            CargoMenu();
        }
        4 => {
            select = RosterMenu();
        }
        5 => {
            if GameOptions().0 == 0 {
                return BOOLEAN::FALSE;
            }
        }
        1 => {
            StarMap();
            return BOOLEAN::FALSE;
        }
        6 => return BOOLEAN::FALSE,
        _ => {}
    }
    if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
        & C2Rust_Unnamed_9::CHECK_ABORT.0 as ::core::ffi::c_int == 0
    {
        if select.0 != 0 {
            if optWhichMenu != 0x2 as ::core::ffi::c_int {
                (*pMS).CurState = HyperMenuItems::NAVIGATION.0 as ::core::ffi::c_int
                    as BYTE;
            }
            DrawMenuStateStrings(
                C2Rust_Unnamed_7::PM_STARMAP.0 as ::core::ffi::c_int as BYTE,
                (*pMS).CurState as SWORD,
            );
        }
        SetFlashRect(
            ::core::ptr::with_exposed_provenance_mut::<
                RECT,
            >(!(0 as ::core::ffi::c_long) as usize),
        );
    }
    return BOOLEAN::TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn HyperspaceMenu() {
    let mut OldColor: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    let mut OldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
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
        flash_frame0: ::core::ptr::null_mut::<FRAME_DESC>(),
        flash_frame1: ::core::ptr::null_mut::<FRAME_DESC>(),
        flashContext: ::core::ptr::null_mut::<FlashContext>(),
        hMusic: ::core::ptr::null_mut::<*mut TFB_SoundSample>(),
        privData: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    UnbatchGraphics();
    OldContext = SetContext(SpaceContext);
    OldColor = SetContextBackGroundColor(
        colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE),
    );
    memset(
        &raw mut MenuState as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<MENU_STATE>(),
    );
    MenuState.InputFunc = Some(
        DoHyperspaceMenu as unsafe extern "C" fn(*mut MENU_STATE) -> BOOLEAN,
    ) as Option<unsafe extern "C" fn(*mut menu_state) -> BOOLEAN>;
    MenuState.Initialized = BOOLEAN::TRUE.0 as ::core::ffi::c_int as SIZE;
    MenuState.CurState = HyperMenuItems::STARMAP.0 as ::core::ffi::c_int as BYTE;
    DrawMenuStateStrings(
        C2Rust_Unnamed_7::PM_STARMAP.0 as ::core::ffi::c_int as BYTE,
        HyperMenuItems::STARMAP.0 as ::core::ffi::c_int as SWORD,
    );
    SetFlashRect(
        ::core::ptr::with_exposed_provenance_mut::<RECT>(!(0 as ::core::ffi::c_long) as usize),
    );
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
    DoInput(&raw mut MenuState as *mut ::core::ffi::c_void, BOOLEAN::TRUE);
    SetFlashRect(::core::ptr::null::<RECT>());
    SetContext(SpaceContext);
    if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
        & (C2Rust_Unnamed_9::CHECK_ABORT.0 as ::core::ffi::c_int
            | C2Rust_Unnamed_9::CHECK_LOAD.0 as ::core::ffi::c_int) == 0
    {
        ClearSISRect(((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as BYTE);
        WaitForNoInput(
            (840 as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as TimePeriod,
            BOOLEAN::FALSE,
        );
    }
    SetContextBackGroundColor(OldColor);
    SetContext(OldContext);
    if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
        & C2Rust_Unnamed_9::IN_BATTLE.0 as ::core::ffi::c_int == 0
    {
        cleanup_hyperspace();
    }
    BatchGraphics();
}
#[no_mangle]
pub unsafe extern "C" fn SaveSisHyperState() {
    let mut hSisElement: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut ElementPtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
    let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
    hSisElement = getSisElement();
    if hSisElement.is_null() {
        return;
    }
    ElementPtr = LockLink(&raw mut disp_q, hSisElement) as *mut ELEMENT;
    StarShipPtr = (*ElementPtr).pParent as *mut STARSHIP;
    GlobData.Game_state.ShipFacing = ((*StarShipPtr).ShipFacing as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as UWORD;
    UnlockLink(&raw mut disp_q, hSisElement);
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
                b"src/uqm/races.h\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                395 as ::core::ffi::c_int,
                b"LockFleetInfo\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    return LockLink(pq, h) as *mut FLEET_INFO;
}
