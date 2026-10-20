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
    pub type tfb_soundsample;
    pub type FlashContext;
    fn __assert_fail(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    ) -> !;
    fn AllocLink(pq: *mut QUEUE) -> HLINK;
    fn FreeLink(pq: *mut QUEUE, hLink: HLINK);
    fn PutQueue(pq: *mut QUEUE, hLink: HLINK);
    fn InsertQueue(pq: *mut QUEUE, hLink: HLINK, hRefLink: HLINK);
    fn RemoveQueue(pq: *mut QUEUE, hLink: HLINK);
    fn SetContext(Context: CONTEXT) -> CONTEXT;
    fn SetContextForeGroundColor(Color: Color) -> Color;
    fn SetContextBackGroundColor(Color: Color) -> Color;
    fn DrawStamp(pStamp: *mut STAMP);
    fn DrawFilledRectangle(pRect: *mut RECT);
    fn BatchGraphics();
    fn UnbatchGraphics();
    fn ClearDrawable();
    fn DestroyDrawable(Drawable: DRAWABLE) -> BOOLEAN;
    fn GetFrameRect(Frame: FRAME, pRect: *mut RECT) -> BOOLEAN;
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
    fn SetAbsStringTableIndex(String: STRING, StringTableIndex: COUNT) -> STRING;
    static mut PulsedInputState: CONTROLLER_INPUT_STATE;
    fn WaitForNoInput(duration: TimePeriod, resetInput: BOOLEAN) -> BOOLEAN;
    fn DoInput(pInputState: *mut ::core::ffi::c_void, resetInput: BOOLEAN);
    static mut optWhichMenu: ::core::ffi::c_int;
    static mut resolutionFactor: ::core::ffi::c_uint;
    static mut optInfiniteFuel: OPT_ENABLABLE;
    static mut optCustomSeed: ::core::ffi::c_int;
    static mut optShipSeed: OPT_ENABLABLE;
    static mut optSmartAutoPilot: OPT_ENABLABLE;
    static mut optHyperStars: OPT_ENABLABLE;
    static mut optNoHQEncounters: OPT_ENABLABLE;
    static mut optAdvancedAutoPilot: OPT_ENABLABLE;
    static mut optWindowType: DWORD;
    static mut optHyperSpaceColor: ::core::ffi::c_int;
    static mut CanvasWidth: ::core::ffi::c_int;
    static mut CanvasHeight: ::core::ffi::c_int;
    static mut sinetab: [SDWORD; 0];
    fn ARCTAN(delta_x: SDWORD, delta_y: SDWORD) -> COUNT;
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
    fn SetVelocityComponents(velocityptr: *mut VELOCITY_DESC, dx: SDWORD, dy: SDWORD);
    static mut disp_q: QUEUE;
    static mut DisplayArray: [PRIMITIVE; 330];
    fn AllocElement() -> HELEMENT;
    fn RepairSISBorder();
    fn DrawSISFrame();
    fn ClearSISRect(ClearFlags: BYTE);
    fn SetFlashRect(pRect: *const RECT, purple: BOOLEAN);
    fn DrawHyperCoords(puniverse: POINT);
    fn DrawSISMessage(pStr: *const UNICODE);
    fn DeltaSISGauges(
        crew_delta: SIZE,
        fuel_delta: SDWORD,
        resunit_delta: ::core::ffi::c_int,
    );
    fn setGameStateUint(name: *const ::core::ffi::c_char, val: DWORD);
    fn getGameStateUint(name: *const ::core::ffi::c_char) -> DWORD;
    fn memset(
        _: *mut ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    static mut GlobData: GLOBDATA;
    static mut RadarContext: CONTEXT;
    fn inHyperSpace() -> BOOLEAN;
    fn inQuasiSpace() -> BOOLEAN;
    fn is3DO(optWhich: ::core::ffi::c_int) -> OPT_CONSOLETYPE;
    fn TFB_Random() -> DWORD;
    fn square_root(value: DWORD) -> COUNT;
    static mut CurStarDescPtr: *mut STAR_DESC;
    static mut star_array: [STAR_DESC; 0];
    static mut plot_map: [PLOT_LOCATION; 70];
    static mut portal_map: [PORTAL_LOCATION; 16];
    fn FindStar(
        pLastStar: *mut STAR_DESC,
        puniverse: *mut POINT,
        xbounds: SIZE,
        ybounds: SIZE,
    ) -> *mut STAR_DESC;
    fn GetStarShipFromIndex(pShipQ: *mut QUEUE, Index: COUNT) -> HLINK;
    fn GetSeededFleetFromIndex(Index: COUNT) -> HFLEETINFO;
    fn GetIndexFromStarShip(pShipQ: *mut QUEUE, hStarShip: HLINK) -> COUNT;
    fn GameOptions() -> BOOLEAN;
    fn SetBattlePlanet();
    fn DoMenuChooser(pMS: *mut MENU_STATE, BaseState: BYTE) -> BOOLEAN;
    fn DrawMenuStateStrings(beg_index: BYTE, NewState: SWORD);
    fn animation_preprocess(ElementPtr: *mut ELEMENT);
    fn ship_preprocess(ElementPtr: *mut ELEMENT);
    fn ship_postprocess(ElementPtr: *mut ELEMENT);
    fn CargoMenu();
    fn RosterMenu() -> BOOLEAN;
    fn DevicesMenu() -> BOOLEAN;
    fn StarMap() -> BOOLEAN;
    fn InvokeSpawner() -> BOOLEAN;
    fn SetUpElement(ElementPtr: *mut ELEMENT);
    static mut stars_in_space: FRAME;
    static mut SpaceContext: CONTEXT;
    static mut StatusContext: CONTEXT;
    static mut race_q: [QUEUE; 0];
    static mut LastActivity: ACTIVITY;
    fn GetColorMapColor(ColorMapIndex: COUNT, ColorIndex: COUNT) -> Color;
    fn SetMenuSounds(sound_0: MENU_SOUND_FLAGS, sound_1: MENU_SOUND_FLAGS);
    fn DrawRadarBorder();
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct frame_desc {
    pub Type: DRAWABLE_TYPE,
    pub Index: UWORD,
    pub HotSpot: HOT_SPOT,
    pub Bounds: EXTENT,
    pub image: *mut TFB_Image,
    pub parent: *mut drawable_desc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct drawable_desc {
    pub Flags: CREATE_FLAGS,
    pub MaxIndex: UWORD,
    pub Frame: *mut FRAME_DESC,
}
pub type FRAME_DESC = frame_desc;
pub type CREATE_FLAGS = BYTE;
pub type TFB_Image = tfb_image;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tfb_image {
    pub NormalImg: TFB_Canvas,
    pub ScaledImg: TFB_Canvas,
    pub MipmapImg: TFB_Canvas,
    pub FilledImg: TFB_Canvas,
    pub colormap_index: ::core::ffi::c_int,
    pub colormap_version: ::core::ffi::c_int,
    pub NormalHs: HOT_SPOT,
    pub MipmapHs: HOT_SPOT,
    pub last_scale_hs: HOT_SPOT,
    pub last_scale: ::core::ffi::c_int,
    pub last_scale_type: ::core::ffi::c_int,
    pub last_fill: Color,
    pub extent: EXTENT,
    pub mutex: Mutex,
    pub dirty: BOOLEAN,
}
pub type Mutex = *mut ::core::ffi::c_void;
pub type EXTENT = extent;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct extent {
    pub width: COORD,
    pub height: COORD,
}
pub type COORD = SWORD;
pub type HOT_SPOT = POINT;
pub type POINT = point;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct point {
    pub x: COORD,
    pub y: COORD,
}
pub type TFB_Canvas = *mut ::core::ffi::c_void;
pub type DRAWABLE_TYPE = UWORD;
pub type DRAWABLE_DESC = drawable_desc;
pub type CONTEXT = *mut CONTEXT_DESC;
pub type FRAME = *mut FRAME_DESC;
pub type DRAWABLE = *mut DRAWABLE_DESC;
pub type TIME_VALUE = UWORD;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dextent {
    pub width: SDWORD,
    pub height: SDWORD,
}
pub type DEXTENT = dextent;
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
    pub const POINT_PRIM_HD: Self = Self(7);
    pub const NUM_PRIMS: Self = Self(8);
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
    pub flags: BYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct INTERSECT_CONTROL {
    pub last_time_val: TIME_VALUE,
    pub EndPoint: POINT,
    pub IntersectStamp: STAMP,
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct OPT_ENABLABLE(pub ::core::ffi::c_uint);
impl OPT_ENABLABLE {
    pub const OPTVAL_DISABLED: Self = Self(0);
    pub const OPTVAL_ENABLED: Self = Self(1);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct OPT_CONSOLETYPE(pub ::core::ffi::c_uint);
impl OPT_CONSOLETYPE {
    pub const OPTVAL_PC: Self = Self(0);
    pub const OPTVAL_3DO: Self = Self(1);
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
    pub image: C2Rust_Unnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_0 {
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
    pub c2rust_unnamed: C2Rust_Unnamed_5,
    pub c2rust_unnamed_0: C2Rust_Unnamed_4,
    pub c2rust_unnamed_1: C2Rust_Unnamed_3,
    pub c2rust_unnamed_2: C2Rust_Unnamed_2,
    pub c2rust_unnamed_3: C2Rust_Unnamed_1,
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
pub union C2Rust_Unnamed_1 {
    pub thrust_wait: BYTE,
    pub blast_offset: BYTE,
    pub next_turn: BYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_2 {
    pub turn_wait: BYTE,
    pub sys_loc: BYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_3 {
    pub mass_points: BYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_4 {
    pub crew_level: COUNT,
    pub hit_points: COUNT,
    pub facing: COUNT,
    pub cycle: COUNT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_5 {
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
    pub stowed_ship_q: QUEUE,
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct RACE_ID(pub ::core::ffi::c_uint);
impl RACE_ID {
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
    pub const ANDROSYNTH_SHIP: Self = Self(22);
    pub const CHENJESU_SHIP: Self = Self(23);
    pub const MMRNMHRM_SHIP: Self = Self(24);
    pub const NUM_BUILDABLE_SHIPS: Self = Self(25);
    pub const YEHAT_REBEL_SHIP: Self = Self(25);
    pub const URQUAN_DRONE_SHIP: Self = Self(26);
    pub const SAMATRA_SHIP: Self = Self(26);
    pub const NUM_AVAILABLE_RACES: Self = Self(27);
}
pub type HFLEETINFO = HLINK;
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
    pub c2rust_padding: [u8; 8],
    pub fleet: FLEET_STUFF,
    pub c2rust_padding_0: [u8; 10],
    pub characteristics: CHARACTERISTIC_STUFF,
    pub c2rust_padding_1: [u8; 2],
    pub ship_data: DATA_STUFF,
    pub c2rust_padding_2: [u8; 8],
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
    pub tl_offset: BYTE,
    pub tr_offset: BYTE,
    pub thrust_offset: BYTE,
    pub weapon_offset: BYTE,
    pub special_offset: BYTE,
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
pub type PLOT_LOCATION = plot_location;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct plot_location {
    pub star_pt: POINT,
    pub star: *mut STAR_DESC,
    pub dist_sq: [DWORD; 70],
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
pub type PORTAL_LOCATION = portal_location;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct portal_location {
    pub star_pt: POINT,
    pub quasi_pt: POINT,
    pub nearest_star: *mut STAR_DESC,
}
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
    pub flash_rect2: RECT,
    pub flash_frame0: FRAME,
    pub flash_frame1: FRAME,
    pub flashContext: *mut FlashContext,
    pub hMusic: MUSIC_REF,
    pub privData: *mut ::core::ffi::c_void,
}
pub type MENU_SOUND_FLAGS = UWORD;
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_7(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_7 {
    pub const DWARF_STAR: Self = Self(0);
    pub const GIANT_STAR: Self = Self(1);
    pub const SUPER_GIANT_STAR: Self = Self(2);
    pub const NUM_STAR_TYPES: Self = Self(3);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_8(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_8 {
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
    pub const NUM_PLANET_COLORS: Self = Self(9);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_9(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_9 {
    pub const ARILOU_DEFINED: Self = Self(0);
    pub const SOL_DEFINED: Self = Self(1);
    pub const SHOFIXTI_DEFINED: Self = Self(2);
    pub const MAIDENS_DEFINED: Self = Self(3);
    pub const START_COLONY_DEFINED: Self = Self(4);
    pub const SPATHI_DEFINED: Self = Self(5);
    pub const ZOQFOT_DEFINED: Self = Self(6);
    pub const MELNORME0_DEFINED: Self = Self(7);
    pub const MELNORME1_DEFINED: Self = Self(8);
    pub const MELNORME2_DEFINED: Self = Self(9);
    pub const MELNORME3_DEFINED: Self = Self(10);
    pub const MELNORME4_DEFINED: Self = Self(11);
    pub const MELNORME5_DEFINED: Self = Self(12);
    pub const MELNORME6_DEFINED: Self = Self(13);
    pub const MELNORME7_DEFINED: Self = Self(14);
    pub const MELNORME8_DEFINED: Self = Self(15);
    pub const TALKING_PET_DEFINED: Self = Self(16);
    pub const CHMMR_DEFINED: Self = Self(17);
    pub const SYREEN_DEFINED: Self = Self(18);
    pub const BURVIXESE_DEFINED: Self = Self(19);
    pub const SLYLANDRO_DEFINED: Self = Self(20);
    pub const DRUUGE_DEFINED: Self = Self(21);
    pub const BOMB_DEFINED: Self = Self(22);
    pub const AQUA_HELIX_DEFINED: Self = Self(23);
    pub const SUN_DEVICE_DEFINED: Self = Self(24);
    pub const TAALO_PROTECTOR_DEFINED: Self = Self(25);
    pub const SHIP_VAULT_DEFINED: Self = Self(26);
    pub const URQUAN_WRECK_DEFINED: Self = Self(27);
    pub const VUX_BEAST_DEFINED: Self = Self(28);
    pub const SAMATRA_DEFINED: Self = Self(29);
    pub const ZOQ_SCOUT_DEFINED: Self = Self(30);
    pub const MYCON_DEFINED: Self = Self(31);
    pub const EGG_CASE0_DEFINED: Self = Self(32);
    pub const EGG_CASE1_DEFINED: Self = Self(33);
    pub const EGG_CASE2_DEFINED: Self = Self(34);
    pub const PKUNK_DEFINED: Self = Self(35);
    pub const UTWIG_DEFINED: Self = Self(36);
    pub const SUPOX_DEFINED: Self = Self(37);
    pub const YEHAT_DEFINED: Self = Self(38);
    pub const VUX_DEFINED: Self = Self(39);
    pub const ORZ_DEFINED: Self = Self(40);
    pub const THRADD_DEFINED: Self = Self(41);
    pub const RAINBOW0_DEFINED: Self = Self(42);
    pub const RAINBOW1_DEFINED: Self = Self(43);
    pub const RAINBOW2_DEFINED: Self = Self(44);
    pub const RAINBOW3_DEFINED: Self = Self(45);
    pub const RAINBOW4_DEFINED: Self = Self(46);
    pub const RAINBOW5_DEFINED: Self = Self(47);
    pub const RAINBOW6_DEFINED: Self = Self(48);
    pub const RAINBOW7_DEFINED: Self = Self(49);
    pub const RAINBOW8_DEFINED: Self = Self(50);
    pub const RAINBOW9_DEFINED: Self = Self(51);
    pub const ILWRATH_DEFINED: Self = Self(52);
    pub const ANDROSYNTH_DEFINED: Self = Self(53);
    pub const MYCON_TRAP_DEFINED: Self = Self(54);
    pub const URQUAN0_DEFINED: Self = Self(55);
    pub const URQUAN1_DEFINED: Self = Self(56);
    pub const URQUAN2_DEFINED: Self = Self(57);
    pub const KOHRAH0_DEFINED: Self = Self(58);
    pub const KOHRAH1_DEFINED: Self = Self(59);
    pub const KOHRAH2_DEFINED: Self = Self(60);
    pub const DESTROYED_STARBASE_DEFINED: Self = Self(61);
    pub const MOTHER_ARK_DEFINED: Self = Self(62);
    pub const ZOQ_COLONY0_DEFINED: Self = Self(63);
    pub const ZOQ_COLONY1_DEFINED: Self = Self(64);
    pub const ZOQ_COLONY2_DEFINED: Self = Self(65);
    pub const ZOQ_COLONY3_DEFINED: Self = Self(66);
    pub const ALGOLITES_DEFINED: Self = Self(67);
    pub const SPATHI_MONUMENT_DEFINED: Self = Self(68);
    pub const EXCAVATION_SITE_DEFINED: Self = Self(69);
    pub const NUM_PLOTS: Self = Self(70);
    pub const WAR_ERA: Self = Self(70);
    pub const SUCCESS: Self = Self(70);
    pub const HOME: Self = Self(71);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_10(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_10 {
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
#[inline]
unsafe extern "C" fn ValidPoint(mut pt: POINT) -> BOOLEAN {
    return BOOLEAN(
        (pt.x as ::core::ffi::c_int != !(0 as ::core::ffi::c_int)
            && pt.y as ::core::ffi::c_int != !(0 as ::core::ffi::c_int))
            as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
}
#[inline]
unsafe extern "C" fn logxToUniverse(mut lx: SDWORD) -> SDWORD {
    return (lx * (9999 as SDWORD + 1 as SDWORD >> 4 as ::core::ffi::c_int)
        + ((9999 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
            >> 4 as ::core::ffi::c_int)
            * ((16 as ::core::ffi::c_int) << resolutionFactor)
            >> 1 as ::core::ffi::c_int))
        / ((9999 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
            >> 4 as ::core::ffi::c_int)
            * ((16 as ::core::ffi::c_int) << resolutionFactor));
}
#[inline]
unsafe extern "C" fn logyToUniverse(mut ly: SDWORD) -> SDWORD {
    return 9999 as SDWORD
        - (ly * (9999 as SDWORD + 1 as SDWORD >> 4 as ::core::ffi::c_int)
            + ((9999 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                >> 4 as ::core::ffi::c_int)
                * ((16 as ::core::ffi::c_int) << resolutionFactor)
                >> 1 as ::core::ffi::c_int))
            / ((9999 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                >> 4 as ::core::ffi::c_int)
                * ((16 as ::core::ffi::c_int) << resolutionFactor));
}
#[inline]
unsafe extern "C" fn universeToLogx(mut ux: COORD) -> SDWORD {
    return (ux as SDWORD
        * ((9999 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
            >> 4 as ::core::ffi::c_int)
            * ((16 as ::core::ffi::c_int) << resolutionFactor))
        + (9999 as SDWORD + 1 as SDWORD >> 4 as ::core::ffi::c_int
            >> 1 as ::core::ffi::c_int))
        / (9999 as SDWORD + 1 as SDWORD >> 4 as ::core::ffi::c_int);
}
#[inline]
unsafe extern "C" fn universeToLogy(mut uy: COORD) -> SDWORD {
    return ((9999 as SDWORD - uy as SDWORD)
        * ((9999 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
            >> 4 as ::core::ffi::c_int)
            * ((16 as ::core::ffi::c_int) << resolutionFactor))
        + (9999 as SDWORD + 1 as SDWORD >> 4 as ::core::ffi::c_int
            >> 1 as ::core::ffi::c_int))
        / (9999 as SDWORD + 1 as SDWORD >> 4 as ::core::ffi::c_int);
}
static mut hyperstars: [FRAME; 3] = [::core::ptr::null_mut::<FRAME_DESC>(); 3];
static mut hypercmap: COLORMAP = ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>();
static mut fuel_ticks: BYTE = 0;
static mut hyper_extra: COUNT = 0;
static mut hyper_dy: COUNT = 0;
static mut hyper_dx: COUNT = 0;
#[inline]
unsafe extern "C" fn RaceHyperSpeed(mut Index: RACE_ID) -> SIZE {
    static mut seedStamp: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    let defaultMap: [SIZE; 26] = [
        ((40 as ::core::ffi::c_int) << resolutionFactor << 5 as ::core::ffi::c_int
            >> 1 as ::core::ffi::c_int) as SIZE,
        ((27 as ::core::ffi::c_int) << resolutionFactor << 5 as ::core::ffi::c_int
            >> 1 as ::core::ffi::c_int) as SIZE,
        ((24 as ::core::ffi::c_int) << resolutionFactor << 5 as ::core::ffi::c_int
            >> 1 as ::core::ffi::c_int) as SIZE,
        ((40 as ::core::ffi::c_int) << resolutionFactor << 5 as ::core::ffi::c_int
            >> 1 as ::core::ffi::c_int) as SIZE,
        ((40 as ::core::ffi::c_int) << resolutionFactor << 5 as ::core::ffi::c_int
            >> 1 as ::core::ffi::c_int) as SIZE,
        ((35 as ::core::ffi::c_int) << resolutionFactor << 5 as ::core::ffi::c_int
            >> 1 as ::core::ffi::c_int) as SIZE,
        ((48 as ::core::ffi::c_int) << resolutionFactor << 5 as ::core::ffi::c_int
            >> 1 as ::core::ffi::c_int) as SIZE,
        ((40 as ::core::ffi::c_int) << resolutionFactor << 5 as ::core::ffi::c_int
            >> 1 as ::core::ffi::c_int) as SIZE,
        ((50 as ::core::ffi::c_int) << resolutionFactor << 5 as ::core::ffi::c_int
            >> 1 as ::core::ffi::c_int) as SIZE,
        ((30 as ::core::ffi::c_int) << resolutionFactor << 5 as ::core::ffi::c_int
            >> 1 as ::core::ffi::c_int) as SIZE,
        ((21 as ::core::ffi::c_int) << resolutionFactor << 5 as ::core::ffi::c_int
            >> 1 as ::core::ffi::c_int) as SIZE,
        ((30 as ::core::ffi::c_int) << resolutionFactor << 5 as ::core::ffi::c_int
            >> 1 as ::core::ffi::c_int) as SIZE,
        ((40 as ::core::ffi::c_int) << resolutionFactor << 5 as ::core::ffi::c_int
            >> 1 as ::core::ffi::c_int) as SIZE,
        ((20 as ::core::ffi::c_int) << resolutionFactor << 5 as ::core::ffi::c_int
            >> 1 as ::core::ffi::c_int) as SIZE,
        ((25 as ::core::ffi::c_int) << resolutionFactor << 5 as ::core::ffi::c_int
            >> 1 as ::core::ffi::c_int) as SIZE,
        ((27 as ::core::ffi::c_int) << resolutionFactor << 5 as ::core::ffi::c_int
            >> 1 as ::core::ffi::c_int) as SIZE,
        ((60 as ::core::ffi::c_int) << resolutionFactor << 5 as ::core::ffi::c_int
            >> 1 as ::core::ffi::c_int) as SIZE,
        ((18 as ::core::ffi::c_int) << resolutionFactor << 5 as ::core::ffi::c_int
            >> 1 as ::core::ffi::c_int) as SIZE,
        ((30 as ::core::ffi::c_int) << resolutionFactor << 5 as ::core::ffi::c_int
            >> 1 as ::core::ffi::c_int) as SIZE,
        ((40 as ::core::ffi::c_int) << resolutionFactor << 5 as ::core::ffi::c_int
            >> 1 as ::core::ffi::c_int) as SIZE,
        ((36 as ::core::ffi::c_int) << resolutionFactor << 5 as ::core::ffi::c_int
            >> 1 as ::core::ffi::c_int) as SIZE,
        ((30 as ::core::ffi::c_int) << resolutionFactor << 5 as ::core::ffi::c_int
            >> 1 as ::core::ffi::c_int) as SIZE,
        ((45 as ::core::ffi::c_int) << resolutionFactor << 5 as ::core::ffi::c_int
            >> 1 as ::core::ffi::c_int) as SIZE,
        ((27 as ::core::ffi::c_int) << resolutionFactor << 5 as ::core::ffi::c_int
            >> 1 as ::core::ffi::c_int) as SIZE,
        ((50 as ::core::ffi::c_int) << resolutionFactor << 5 as ::core::ffi::c_int
            >> 1 as ::core::ffi::c_int) as SIZE,
        ((30 as ::core::ffi::c_int) << resolutionFactor << 5 as ::core::ffi::c_int
            >> 1 as ::core::ffi::c_int) as SIZE,
    ];
    let numRaces: COUNT = ::core::mem::size_of::<[SIZE; 26]>()
        .wrapping_div(::core::mem::size_of::<COUNT>()) as COUNT;
    static mut speedMap: [SIZE; 26] = [
        0 as SIZE,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    let mut x: ::core::ffi::c_int = 0;
    if optShipSeed.0 == 0
        && (seedStamp != -1 as ::core::ffi::c_int
            || speedMap[0usize] as ::core::ffi::c_int == 0 as ::core::ffi::c_int)
    {
        x = 0 as ::core::ffi::c_int;
        while x < numRaces as ::core::ffi::c_int {
            speedMap[x as usize] = defaultMap[x as usize];
            x += 1;
        }
        seedStamp = -1 as ::core::ffi::c_int;
        if speedMap[0usize] as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            speedMap[0usize] = 1 as SIZE;
        }
    }
    if optShipSeed.0 != 0
        && (seedStamp != optCustomSeed
            || speedMap[0usize] as ::core::ffi::c_int == 0 as ::core::ffi::c_int)
    {
        let mut hFleet: HFLEETINFO = ::core::ptr::null_mut::<::core::ffi::c_void>();
        x = 0 as ::core::ffi::c_int;
        while x < numRaces as ::core::ffi::c_int {
            hFleet = GetSeededFleetFromIndex(x as COUNT);
            if !hFleet.is_null() {
                speedMap[x as usize] = defaultMap[GetIndexFromStarShip(
                    &raw mut GlobData.Game_state.avail_race_q,
                    hFleet,
                ) as usize];
            } else {
                speedMap[x as usize] = defaultMap[x as usize];
            }
            x += 1;
        }
        if speedMap[0usize] as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            speedMap[0usize] = 1 as SIZE;
        }
    }
    return speedMap[Index.0 as usize];
}
#[no_mangle]
pub unsafe extern "C" fn MoveSIS(mut pdx: *mut SDWORD, mut pdy: *mut SDWORD) {
    let mut new_dx: SDWORD = 0;
    let mut new_dy: SDWORD = 0;
    new_dx = *pdx;
    GlobData.SIS_state.log_x -= new_dx;
    if GlobData.SIS_state.log_x < 0 as ::core::ffi::c_int {
        new_dx += GlobData.SIS_state.log_x as SIZE as ::core::ffi::c_int;
        GlobData.SIS_state.log_x = 0 as ::core::ffi::c_int as SDWORD;
    } else if GlobData.SIS_state.log_x as ::core::ffi::c_long
        > (universeToLogx(
            (9999 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as COORD,
        ) - (universeToLogx(1 as COORD) >> 1 as ::core::ffi::c_int))
            as ::core::ffi::c_long - 1 as ::core::ffi::c_long
    {
        new_dx
            += (GlobData.SIS_state.log_x as ::core::ffi::c_long
                - ((universeToLogx(
                    (9999 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as COORD,
                ) - (universeToLogx(1 as COORD) >> 1 as ::core::ffi::c_int))
                    as ::core::ffi::c_long - 1 as ::core::ffi::c_long)) as SIZE
                as ::core::ffi::c_int;
        GlobData.SIS_state.log_x = ((universeToLogx(
            (9999 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as COORD,
        ) - (universeToLogx(1 as COORD) >> 1 as ::core::ffi::c_int))
            as ::core::ffi::c_long - 1 as ::core::ffi::c_long) as SDWORD;
    }
    new_dy = *pdy;
    GlobData.SIS_state.log_y -= new_dy;
    if GlobData.SIS_state.log_y < 0 as ::core::ffi::c_int {
        new_dy += GlobData.SIS_state.log_y as SIZE as ::core::ffi::c_int;
        GlobData.SIS_state.log_y = 0 as ::core::ffi::c_int as SDWORD;
    } else if GlobData.SIS_state.log_y as ::core::ffi::c_long
        > (universeToLogy(-1 as COORD)
            - (universeToLogy(
                (9999 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as COORD,
            ) >> 1 as ::core::ffi::c_int)) as ::core::ffi::c_long
            - 1 as ::core::ffi::c_long
    {
        new_dy
            += (GlobData.SIS_state.log_y as ::core::ffi::c_long
                - ((universeToLogy(-1 as COORD)
                    - (universeToLogy(
                        (9999 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as COORD,
                    ) >> 1 as ::core::ffi::c_int)) as ::core::ffi::c_long
                    - 1 as ::core::ffi::c_long)) as SIZE as ::core::ffi::c_int;
        GlobData.SIS_state.log_y = ((universeToLogy(-1 as COORD)
            - (universeToLogy(
                (9999 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as COORD,
            ) >> 1 as ::core::ffi::c_int)) as ::core::ffi::c_long
            - 1 as ::core::ffi::c_long) as SDWORD;
    }
    if new_dx != *pdx || new_dy != *pdy {
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
                (*ElementPtr).next.location.x = ((((CanvasWidth
                    - ((64 as SDWORD) << resolutionFactor)
                    - (if if optWindowType == 1 as DWORD {
                        BOOLEAN::TRUE.0 as ::core::ffi::c_int
                    } else {
                        BOOLEAN::FALSE.0 as ::core::ffi::c_int
                    } != 0
                    {
                        16 as SDWORD
                    } else {
                        0 as SDWORD
                    } << resolutionFactor) * 2 as SDWORD) << 2 as ::core::ffi::c_int)
                    << 3 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) - new_dx)
                    as COORD;
                (*ElementPtr).next.location.y = ((((CanvasHeight
                    - (if if optWindowType == 1 as DWORD {
                        BOOLEAN::TRUE.0 as ::core::ffi::c_int
                    } else {
                        BOOLEAN::FALSE.0 as ::core::ffi::c_int
                    } != 0
                    {
                        16 as SDWORD
                    } else {
                        0 as SDWORD
                    } << resolutionFactor) * 2 as SDWORD) << 2 as ::core::ffi::c_int)
                    << 3 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) - new_dy)
                    as COORD;
                hNextElement = ::core::ptr::null_mut::<::core::ffi::c_void>();
            }
            UnlockLink(&raw mut disp_q, hElement);
            hElement = hNextElement;
        }
    }
    if GlobData.SIS_state.FuelOnBoard != 0
        && uqm_lua_checked!(getGameStateUint(
            b"ARILOU_SPACE_SIDE\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) <= 1 as DWORD
    {
        let mut cur_fuel_ticks: COUNT = 0;
        let mut hyper_dist: COUNT = 0;
        let mut adj_dx: DWORD = 0;
        let mut adj_dy: DWORD = 0;
        if new_dx < 0 as ::core::ffi::c_int {
            new_dx = -new_dx;
        }
        hyper_dx = (hyper_dx as ::core::ffi::c_int + new_dx as ::core::ffi::c_int)
            as COUNT;
        if new_dy < 0 as ::core::ffi::c_int {
            new_dy = -new_dy;
        }
        hyper_dy = (hyper_dy as ::core::ffi::c_int + new_dy as ::core::ffi::c_int)
            as COUNT;
        adj_dx = logxToUniverse(16 as SDWORD * hyper_dx as SDWORD) as DWORD;
        adj_dy = (9999 as SDWORD - logyToUniverse(16 as SDWORD * hyper_dy as SDWORD))
            as DWORD;
        hyper_dist = (square_root(
            adj_dx.wrapping_mul(adj_dx).wrapping_add(adj_dy.wrapping_mul(adj_dy)),
        ) as ::core::ffi::c_int + hyper_extra as ::core::ffi::c_int) as COUNT;
        cur_fuel_ticks = (hyper_dist as ::core::ffi::c_int >> 4 as ::core::ffi::c_int)
            as COUNT;
        if cur_fuel_ticks as ::core::ffi::c_int
            > fuel_ticks as COUNT as ::core::ffi::c_int
        {
            if optInfiniteFuel.0 == 0 {
                uqm_lua_checked!(DeltaSISGauges(
                    0 as SIZE,
                    fuel_ticks as SDWORD - cur_fuel_ticks as SDWORD,
                    0 as ::core::ffi::c_int,
                ), ());
            }
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
    let mut EncounterPercent: [COUNT; 27] = [
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
        0 as COUNT,
        0 as COUNT,
        0 as COUNT,
        60 as COUNT,
        0 as COUNT,
    ];
    universe.x = logxToUniverse(GlobData.SIS_state.log_x) as COORD;
    universe.y = logyToUniverse(GlobData.SIS_state.log_y) as COORD;
    hStarShip = GlobData.Game_state.avail_race_q.head as HFLEETINFO;
    Type = 0 as BYTE;
    while !hStarShip.is_null()
        && GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
            & C2Rust_Unnamed_6::IN_BATTLE.0 as ::core::ffi::c_int != 0
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
            if optNoHQEncounters.0 != 0 {
                percent = 0 as COUNT;
            }
            if encounter_radius as ::core::ffi::c_int
                != !(0 as ::core::ffi::c_int) as COUNT as ::core::ffi::c_int
            {
                encounter_radius = (encounter_radius as ::core::ffi::c_int
                    * 11 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as COUNT;
                if if GlobData.SIS_state.Extended as ::core::ffi::c_int != 0 {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                    && uqm_lua_checked!(getGameStateUint(
                        b"KOHR_AH_FRENZY\0".as_ptr() as *const ::core::ffi::c_char,
                    ), ()) != 0
                    && Type as ::core::ffi::c_int
                        == RACE_ID::BLACK_URQUAN_SHIP.0 as ::core::ffi::c_int
                    && uqm_lua_checked!(getGameStateUint(
                        b"USED_BROADCASTER\0".as_ptr() as *const ::core::ffi::c_char,
                    ), ()) != 0
                {
                    encounter_radius = ((9999 as ::core::ffi::c_int
                        + 1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as COUNT;
                    percent = 100 as COUNT;
                    encounter_flags = ((1 as ::core::ffi::c_int)
                        << 7 as ::core::ffi::c_int) as BYTE;
                    uqm_lua_checked!(setGameStateUint(
                        b"BROADCASTER_RESPONSE\0".as_ptr() as *const ::core::ffi::c_char,
                        1 as DWORD,
                    ), ());
                }
            } else {
                let mut hNextEncounter: HENCOUNTER = ::core::ptr::null_mut::<
                    ::core::ffi::c_void,
                >();
                encounter_radius = ((9999 as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as COUNT;
                if Type as ::core::ffi::c_int
                    == RACE_ID::SLYLANDRO_SHIP.0 as ::core::ffi::c_int
                {
                    encounter_flags = ((1 as ::core::ffi::c_int)
                        << 7 as ::core::ffi::c_int) as BYTE;
                    if uqm_lua_checked!(getGameStateUint(
                        b"STARBASE_AVAILABLE\0".as_ptr() as *const ::core::ffi::c_char,
                    ), ()) == 0
                    {
                        percent = 100 as COUNT;
                    } else {
                        percent = (percent as DWORD)
                            .wrapping_mul(
                                uqm_lua_checked!(getGameStateUint(
                                    b"SLYLANDRO_MULTIPLIER\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                ), ()),
                            ) as COUNT;
                    }
                    if optNoHQEncounters.0 != 0 {
                        percent = 0 as COUNT;
                    }
                }
                if if GlobData.SIS_state.Extended as ::core::ffi::c_int != 0 {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                    && uqm_lua_checked!(getGameStateUint(
                        b"KOHR_AH_FRENZY\0".as_ptr() as *const ::core::ffi::c_char,
                    ), ()) == 0
                    || if GlobData.SIS_state.Extended as ::core::ffi::c_int != 0 {
                        BOOLEAN::TRUE.0 as ::core::ffi::c_int
                    } else {
                        BOOLEAN::FALSE.0 as ::core::ffi::c_int
                    } == 0
                {
                    if Type as ::core::ffi::c_int
                        == RACE_ID::MELNORME_SHIP.0 as ::core::ffi::c_int
                        && (GlobData.SIS_state.FuelOnBoard == 0 as DWORD
                            || uqm_lua_checked!(getGameStateUint(
                                b"USED_BROADCASTER\0".as_ptr() as *const ::core::ffi::c_char,
                            ), ()) != 0)
                        && uqm_lua_checked!(getGameStateUint(
                            b"MELNORME_ANGER\0".as_ptr() as *const ::core::ffi::c_char,
                        ), ()) < 3 as DWORD
                    {
                        if uqm_lua_checked!(getGameStateUint(
                            b"USED_BROADCASTER\0".as_ptr() as *const ::core::ffi::c_char,
                        ), ()) == 0
                        {
                            percent = 30 as COUNT;
                        } else {
                            percent = 100 as COUNT;
                        }
                        encounter_flags = ((1 as ::core::ffi::c_int)
                            << 7 as ::core::ffi::c_int) as BYTE;
                    }
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
                        == RACE_ID::MELNORME_SHIP.0 as ::core::ffi::c_int
                {
                    uqm_lua_checked!(setGameStateUint(
                        b"BROADCASTER_RESPONSE\0".as_ptr() as *const ::core::ffi::c_char,
                        1 as DWORD,
                    ), ());
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
    uqm_lua_checked!(setGameStateUint(
        b"USED_BROADCASTER\0".as_ptr() as *const ::core::ffi::c_char,
        0 as DWORD,
    ), ());
}
#[no_mangle]
pub unsafe extern "C" fn FreeHyperData() {
    DestroyDrawable(ReleaseDrawable(hyperstars[0usize]));
    hyperstars[0usize] = ::core::ptr::null_mut::<FRAME_DESC>();
    DestroyDrawable(ReleaseDrawable(hyperstars[1usize]));
    hyperstars[1usize] = ::core::ptr::null_mut::<FRAME_DESC>();
    DestroyDrawable(ReleaseDrawable(hyperstars[2usize]));
    hyperstars[2usize] = ::core::ptr::null_mut::<FRAME_DESC>();
    DestroyStringTable(ReleaseStringTable(hypercmap));
    hypercmap = ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>();
}
unsafe extern "C" fn LoadHyperData() {
    if hyperstars[0usize].is_null() {
        hyperstars[0usize] = CaptureDrawable(
            uqm_lua_checked!(LoadGraphicInstance(b"graphics.beacon\0".as_ptr() as RESOURCE), ()) as DRAWABLE,
        );
        hyperstars[1usize] = CaptureDrawable(
            uqm_lua_checked!(LoadGraphicInstance(
                if uqm_lua_checked!(getGameStateUint(
                    b"ARILOU_SPACE_SIDE\0".as_ptr() as *const ::core::ffi::c_char,
                ), ()) <= 1 as DWORD
                {
                    if if resolutionFactor != 2 as ::core::ffi::c_uint {
                        BOOLEAN::FALSE.0 as ::core::ffi::c_int
                    } else {
                        BOOLEAN::TRUE.0 as ::core::ffi::c_int
                    } != 0 && optHyperStars.0 != 0
                    {
                        b"graphics.hyperstars.anim\0".as_ptr() as RESOURCE
                    } else {
                        b"graphics.hyperstars\0".as_ptr() as RESOURCE
                    }
                } else if if resolutionFactor != 2 as ::core::ffi::c_uint {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } != 0 && optHyperStars.0 != 0
                {
                    b"graphics.quasispace.anim\0".as_ptr() as RESOURCE
                } else {
                    b"graphics.quasispace\0".as_ptr() as RESOURCE
                },
            ), ()) as DRAWABLE,
        );
        hyperstars[2usize] = CaptureDrawable(
            uqm_lua_checked!(LoadGraphicInstance(b"graphics.ambient\0".as_ptr() as RESOURCE), ()) as DRAWABLE,
        );
        hypercmap = CaptureStringTable(
            uqm_lua_checked!(LoadStringTableInstance(b"colortable.hyperspace\0".as_ptr() as RESOURCE), ()),
        ) as COLORMAP;
    } else {
        DestroyDrawable(ReleaseDrawable(hyperstars[1usize]));
        hyperstars[1usize] = CaptureDrawable(
            uqm_lua_checked!(LoadGraphicInstance(
                if uqm_lua_checked!(getGameStateUint(
                    b"ARILOU_SPACE_SIDE\0".as_ptr() as *const ::core::ffi::c_char,
                ), ()) <= 1 as DWORD
                {
                    if if resolutionFactor != 2 as ::core::ffi::c_uint {
                        BOOLEAN::FALSE.0 as ::core::ffi::c_int
                    } else {
                        BOOLEAN::TRUE.0 as ::core::ffi::c_int
                    } != 0 && optHyperStars.0 != 0
                    {
                        b"graphics.hyperstars.anim\0".as_ptr() as RESOURCE
                    } else {
                        b"graphics.hyperstars\0".as_ptr() as RESOURCE
                    }
                } else if if resolutionFactor != 2 as ::core::ffi::c_uint {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } != 0 && optHyperStars.0 != 0
                {
                    b"graphics.quasispace.anim\0".as_ptr() as RESOURCE
                } else {
                    b"graphics.quasispace\0".as_ptr() as RESOURCE
                },
            ), ()) as DRAWABLE,
        );
        if if resolutionFactor != 2 as ::core::ffi::c_uint {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } != 0
        {
            DestroyDrawable(ReleaseDrawable(hyperstars[2usize]));
            hyperstars[2usize] = CaptureDrawable(
                uqm_lua_checked!(LoadGraphicInstance(b"graphics.ambient\0".as_ptr() as RESOURCE), ())
                    as DRAWABLE,
            );
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn LoadHyperspace() -> BOOLEAN {
    let mut whichPalette: BYTE = ((uqm_lua_checked!(getGameStateUint(
        b"ARILOU_SPACE_SIDE\0".as_ptr() as *const ::core::ffi::c_char,
    ), BOOLEAN(0)) > 1 as DWORD) as ::core::ffi::c_int as ::core::ffi::c_uint)
        .wrapping_add(is3DO(optHyperSpaceColor).0.wrapping_mul(2 as ::core::ffi::c_uint))
        as BYTE;
    hyper_dx = 0 as COUNT;
    hyper_dy = 0 as COUNT;
    hyper_extra = 0 as COUNT;
    fuel_ticks = 1 as BYTE;
    GlobData.Game_state.ShipStamp.origin.x = -9999 as COORD;
    GlobData.Game_state.ShipStamp.origin.y = -9999 as COORD;
    uqm_lua_checked!(LoadHyperData(), BOOLEAN(0));
    let mut F: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
    F = hyperstars[2usize];
    hyperstars[2usize] = stars_in_space;
    stars_in_space = F;
    if LastActivity as ::core::ffi::c_int
        & C2Rust_Unnamed_6::CHECK_LOAD.0 as ::core::ffi::c_int == 0
    {
        uqm_lua_checked!(RepairSISBorder(), BOOLEAN(0));
    } else if LastActivity as BYTE as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        uqm_lua_checked!(DrawSISFrame(), BOOLEAN(0));
    } else {
        uqm_lua_checked!(ClearSISRect(((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as BYTE), BOOLEAN(0));
        uqm_lua_checked!(RepairSISBorder(), BOOLEAN(0));
    }
    if !(GlobData.Game_state.autopilot.x as ::core::ffi::c_int
        != !(0 as ::core::ffi::c_int)
        && GlobData.Game_state.autopilot.y as ::core::ffi::c_int
            != !(0 as ::core::ffi::c_int))
    {
        uqm_lua_checked!(DrawSISMessage(::core::ptr::null::<UNICODE>()), BOOLEAN(0));
    }
    SetColorMap(
        GetColorMapAddress(SetAbsStringTableIndex(hypercmap, whichPalette as COUNT)),
    );
    SetContext(RadarContext);
    SetContextBackGroundColor(GetColorMapColor(0x35 as COUNT, 0xfd as COUNT));
    SetContext(SpaceContext);
    SetContextBackGroundColor(GetColorMapColor(0x35 as COUNT, 0xff as COUNT));
    uqm_lua_checked!(setGameStateUint(
        b"USED_BROADCASTER\0".as_ptr() as *const ::core::ffi::c_char,
        0 as DWORD,
    ), BOOLEAN(0));
    uqm_lua_checked!(setGameStateUint(
        b"BROADCASTER_RESPONSE\0".as_ptr() as *const ::core::ffi::c_char,
        0 as DWORD,
    ), BOOLEAN(0));
    return BOOLEAN::TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn EraseRadar() {
    BatchGraphics();
    uqm_lua_checked!(ClearSISRect(((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as BYTE), ());
    uqm_lua_checked!(SeedUniverse(), ());
    UnbatchGraphics();
}
#[no_mangle]
pub unsafe extern "C" fn FreeHyperspace() -> BOOLEAN {
    let mut F: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
    F = hyperstars[2usize];
    hyperstars[2usize] = stars_in_space;
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
            - (((CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
                - (if if optWindowType == 1 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    16 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                } << resolutionFactor) * 2 as ::core::ffi::c_int)
                << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int))) as SDWORD;
    log_y = (GlobData.SIS_state.log_y
        + ((*ElementPtr).next.location.y as ::core::ffi::c_int
            - (((CanvasHeight
                - (if if optWindowType == 1 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    16 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                } << resolutionFactor) * 2 as ::core::ffi::c_int)
                << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int))) as SDWORD;
    (*pPt).x = logxToUniverse(log_x) as COORD;
    (*pPt).y = logyToUniverse(log_y) as COORD;
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
    uqm_lua_checked!(setGameStateUint(
        b"USED_BROADCASTER\0".as_ptr() as *const ::core::ffi::c_char,
        0 as DWORD,
    ), ());
    if uqm_lua_checked!(getGameStateUint(b"ARILOU_SPACE_SIDE\0".as_ptr() as *const ::core::ffi::c_char), ())
        <= 1 as DWORD
    {
        GlobData.Game_state.CurrentActivity = (GlobData.Game_state.CurrentActivity
            as ::core::ffi::c_int
            | C2Rust_Unnamed_6::START_INTERPLANETARY.0 as ::core::ffi::c_int)
            as ACTIVITY;
        uqm_lua_checked!(setGameStateUint(
            b"ESCAPE_COUNTER\0".as_ptr() as *const ::core::ffi::c_char,
            0 as DWORD,
        ), ());
    } else {
        let mut pt: POINT = POINT { x: 0, y: 0 };
        let mut KnownQSPortals: UWORD = uqm_lua_checked!(getGameStateUint(
            b"KNOW_QS_PORTAL\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) as UWORD;
        GlobData.Game_state.autopilot.x = !(0 as ::core::ffi::c_int) as COORD;
        GlobData.Game_state.autopilot.y = !(0 as ::core::ffi::c_int) as COORD;
        ElementToUniverse(ElementPtr, &raw mut pt);
        CurStarDescPtr = uqm_lua_checked!(FindStar(
            ::core::ptr::null_mut::<STAR_DESC>(),
            &raw mut pt,
            5 as SIZE,
            5 as SIZE,
        ), ());
        if (*CurStarDescPtr).star_pt.x as ::core::ffi::c_int
            == 6134 as ::core::ffi::c_int
            && (*CurStarDescPtr).star_pt.y as ::core::ffi::c_int
                == 5900 as ::core::ffi::c_int
        {
            GlobData.Game_state.CurrentActivity = (GlobData.Game_state.CurrentActivity
                as ::core::ffi::c_int
                | C2Rust_Unnamed_6::START_ENCOUNTER.0 as ::core::ffi::c_int) as ACTIVITY;
            KnownQSPortals = (KnownQSPortals as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 15 as ::core::ffi::c_int) as UWORD;
            uqm_lua_checked!(setGameStateUint(
                b"KNOW_QS_PORTAL\0".as_ptr() as *const ::core::ffi::c_char,
                KnownQSPortals as DWORD,
            ), ());
        } else {
            let mut index: COUNT = 0;
            index = CurStarDescPtr
                .offset_from(
                    (&raw mut star_array as *mut STAR_DESC)
                        .offset(
                            (502 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                as isize,
                        ),
                ) as COUNT;
            GlobData.SIS_state.log_x = universeToLogx(
                portal_map[index as usize].star_pt.x,
            );
            GlobData.SIS_state.log_y = universeToLogy(
                portal_map[index as usize].star_pt.y,
            );
            KnownQSPortals = (KnownQSPortals as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << index as ::core::ffi::c_int) as UWORD;
            uqm_lua_checked!(setGameStateUint(
                b"KNOW_QS_PORTAL\0".as_ptr() as *const ::core::ffi::c_char,
                KnownQSPortals as DWORD,
            ), ());
            uqm_lua_checked!(setGameStateUint(
                b"ARILOU_SPACE_SIDE\0".as_ptr() as *const ::core::ffi::c_char,
                0 as DWORD,
            ), ());
        }
    };
}
unsafe extern "C" fn ArilouSpaceTransition() {
    GlobData.Game_state.ShipFacing = 0 as UWORD;
    uqm_lua_checked!(setGameStateUint(
        b"USED_BROADCASTER\0".as_ptr() as *const ::core::ffi::c_char,
        0 as DWORD,
    ), ());
    GlobData.Game_state.autopilot.x = !(0 as ::core::ffi::c_int) as COORD;
    GlobData.Game_state.autopilot.y = !(0 as ::core::ffi::c_int) as COORD;
    if uqm_lua_checked!(getGameStateUint(b"ARILOU_SPACE_SIDE\0".as_ptr() as *const ::core::ffi::c_char), ())
        <= 1 as DWORD
    {
        GlobData.SIS_state.log_x = universeToLogx(5000 as COORD);
        GlobData.SIS_state.log_y = universeToLogy(5000 as COORD);
        if uqm_lua_checked!(getGameStateUint(b"PORTAL_COUNTER\0".as_ptr() as *const ::core::ffi::c_char), ())
            == 0 as DWORD
        {
            uqm_lua_checked!(setGameStateUint(
                b"ARILOU_SPACE_SIDE\0".as_ptr() as *const ::core::ffi::c_char,
                3 as DWORD,
            ), ());
            if if GlobData.SIS_state.Extended as ::core::ffi::c_int != 0 {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                uqm_lua_checked!(ZeroLastLoc(), ());
            }
        } else {
            uqm_lua_checked!(setGameStateUint(
                b"PORTAL_COUNTER\0".as_ptr() as *const ::core::ffi::c_char,
                0 as DWORD,
            ), ());
            uqm_lua_checked!(setGameStateUint(
                b"ARILOU_SPACE_SIDE\0".as_ptr() as *const ::core::ffi::c_char,
                3 as DWORD,
            ), ());
        }
    } else {
        if if GlobData.SIS_state.Extended as ::core::ffi::c_int != 0 {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } == 0
            || if GlobData.SIS_state.Extended as ::core::ffi::c_int != 0 {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0 && ValidPoint(uqm_lua_checked!(LoadLastLoc(), ())).0 == 0
        {
            GlobData.SIS_state.log_x = universeToLogx(
                plot_map[C2Rust_Unnamed_9::ARILOU_DEFINED.0 as ::core::ffi::c_int
                        as usize]
                    .star_pt
                    .x,
            );
            GlobData.SIS_state.log_y = universeToLogy(
                plot_map[C2Rust_Unnamed_9::ARILOU_DEFINED.0 as ::core::ffi::c_int
                        as usize]
                    .star_pt
                    .y,
            );
        } else if if GlobData.SIS_state.Extended as ::core::ffi::c_int != 0 {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0 && ValidPoint(uqm_lua_checked!(LoadLastLoc(), ())).0 != 0
        {
            GlobData.SIS_state.log_x = universeToLogx(uqm_lua_checked!(LoadLastLoc(), ()).x);
            GlobData.SIS_state.log_y = universeToLogy(uqm_lua_checked!(LoadLastLoc(), ()).y);
            uqm_lua_checked!(ZeroLastLoc(), ());
        }
        uqm_lua_checked!(setGameStateUint(
            b"ARILOU_SPACE_SIDE\0".as_ptr() as *const ::core::ffi::c_char,
            0 as DWORD,
        ), ());
    };
}
unsafe extern "C" fn unhyper_transition(mut ElementPtr: *mut ELEMENT) {
    let mut frame_index: COUNT = 0;
    if optSmartAutoPilot.0 != 0
        && (GlobData.Game_state.autopilot.x as ::core::ffi::c_int
            != !(0 as ::core::ffi::c_int)
            && GlobData.Game_state.autopilot.y as ::core::ffi::c_int
                != !(0 as ::core::ffi::c_int))
    {
        let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
        let mut universe: POINT = POINT { x: 0, y: 0 };
        let mut facing: SIZE = 0;
        let mut udx: SDWORD = 0 as SDWORD;
        let mut udy: SDWORD = 0 as SDWORD;
        StarShipPtr = (*ElementPtr).pParent as *mut STARSHIP;
        universe.x = logxToUniverse(GlobData.SIS_state.log_x) as COORD;
        universe.y = logyToUniverse(GlobData.SIS_state.log_y) as COORD;
        udx = (GlobData.Game_state.autopilot.x as ::core::ffi::c_int
            - universe.x as ::core::ffi::c_int) as SDWORD;
        udy = -(GlobData.Game_state.autopilot.y as ::core::ffi::c_int
            - universe.y as ::core::ffi::c_int) as SDWORD;
        facing = (ARCTAN(udx, udy) as ::core::ffi::c_int
            + ((1 as ::core::ffi::c_int)
                << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int)
            >> 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
            & ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                - 1 as ::core::ffi::c_int) as DWORD as SIZE;
        (*StarShipPtr).ShipFacing = facing as COUNT;
        (*ElementPtr).pParent = StarShipPtr as *mut ::core::ffi::c_void;
    }
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
                - 1 as ::core::ffi::c_int) as DWORD as COUNT;
    } else {
        frame_index = frame_index.wrapping_add(1);
        if frame_index as ::core::ffi::c_int
            == GetFrameCount((*ElementPtr).current.image.frame) as ::core::ffi::c_int
        {
            cleanup_hyperspace();
            GlobData.Game_state.CurrentActivity = (GlobData.Game_state.CurrentActivity
                as ::core::ffi::c_int
                & !(C2Rust_Unnamed_6::IN_BATTLE.0 as ::core::ffi::c_int)) as ACTIVITY;
            match TRANSITION_TYPE(
                (*ElementPtr).c2rust_unnamed_2.turn_wait as ::core::ffi::c_uint,
            ) {
                TRANSITION_TYPE::RANDOM_ENCOUNTER_TRANSITION => {
                    SaveSisHyperState();
                    uqm_lua_checked!(SetBattlePlanet(), ());
                    GlobData.Game_state.CurrentActivity = (GlobData
                        .Game_state
                        .CurrentActivity as ::core::ffi::c_int
                        | C2Rust_Unnamed_6::START_ENCOUNTER.0 as ::core::ffi::c_int)
                        as ACTIVITY;
                }
                TRANSITION_TYPE::INTERPLANETARY_TRANSITION => {
                    uqm_lua_checked!(InterplanetaryTransition(ElementPtr), ());
                }
                TRANSITION_TYPE::ARILOU_SPACE_TRANSITION => {
                    uqm_lua_checked!(ArilouSpaceTransition(), ());
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
    num_turns = ((GetFrameCount((*ElementPtr1).next.image.frame) as ::core::ffi::c_int
        - (((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
            + ((1 as ::core::ffi::c_int)
                << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int)
            >> 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int)) as DWORD)
        .wrapping_add(
            ((((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                + ((1 as ::core::ffi::c_int)
                    << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int)
                >> 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int)
                - (*StarShipPtr).ShipFacing as ::core::ffi::c_int
                & ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int) as DWORD,
        ) as SIZE;
    if num_turns as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        num_turns = 1 as SIZE;
    }
    SetVelocityComponents(
        &raw mut (*ElementPtr1).velocity,
        dx as SDWORD / num_turns as SDWORD,
        dy as SDWORD / num_turns as SDWORD,
    );
}
#[no_mangle]
pub unsafe extern "C" fn DoAdvancedAutoPilot() {
    let mut QuasiPilot: POINT = uqm_lua_checked!(LoadAdvancedQuasiPilot(), ());
    let mut SavedPilot: POINT = uqm_lua_checked!(LoadAdvancedAutoPilot(), ());
    if ValidPoint(SavedPilot).0 == 0 {
        return;
    }
    if uqm_lua_checked!(inHyperSpace(), ()).0 != 0 {
        if ValidPoint(QuasiPilot).0 != 0
            && ValidPoint(GlobData.Game_state.autopilot).0 != 0
        {
            uqm_lua_checked!(InvokeSpawner(), ());
        } else {
            GlobData.Game_state.autopilot = SavedPilot;
            uqm_lua_checked!(ZeroAdvancedAutoPilot(), ());
        }
        return;
    }
    if ValidPoint(QuasiPilot).0 == 0 {
        return;
    }
    GlobData.Game_state.autopilot = QuasiPilot;
    uqm_lua_checked!(ZeroAdvancedQuasiPilot(), ());
}
unsafe extern "C" fn hyper_preprocess(mut ElementPtr: *mut ELEMENT) {
    uqm_lua_checked!(hyper_transition(ElementPtr), ());
}
#[no_mangle]
pub unsafe extern "C" fn hyper_transition(mut ElementPtr: *mut ELEMENT) -> BOOLEAN {
    if (*ElementPtr).state_flags as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int != 0
    {
        if LastActivity as ::core::ffi::c_int
            & C2Rust_Unnamed_6::CHECK_LOAD.0 as ::core::ffi::c_int != 0
        {
            LastActivity = (LastActivity as ::core::ffi::c_int
                & !(C2Rust_Unnamed_6::CHECK_LOAD.0 as ::core::ffi::c_int)) as ACTIVITY;
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
        if optSmartAutoPilot.0 != 0 && ValidPoint(GlobData.Game_state.autopilot).0 != 0 {
            let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
            let mut universe: POINT = POINT { x: 0, y: 0 };
            let mut facing: SIZE = 0;
            let mut udx: SDWORD = 0 as SDWORD;
            let mut udy: SDWORD = 0 as SDWORD;
            StarShipPtr = (*ElementPtr).pParent as *mut STARSHIP;
            universe.x = logxToUniverse(GlobData.SIS_state.log_x) as COORD;
            universe.y = logyToUniverse(GlobData.SIS_state.log_y) as COORD;
            udx = (GlobData.Game_state.autopilot.x as ::core::ffi::c_int
                - universe.x as ::core::ffi::c_int) as SDWORD;
            udy = -(GlobData.Game_state.autopilot.y as ::core::ffi::c_int
                - universe.y as ::core::ffi::c_int) as SDWORD;
            facing = (ARCTAN(udx, udy) as ::core::ffi::c_int
                + ((1 as ::core::ffi::c_int)
                    << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int)
                >> 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                & ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int) as DWORD as SIZE;
            (*StarShipPtr).ShipFacing = facing as COUNT;
            (*ElementPtr).pParent = StarShipPtr as *mut ::core::ffi::c_void;
        }
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
            let mut StarShipPtr_0: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
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
                        - 1 as ::core::ffi::c_int) as DWORD as COUNT;
            }
            StarShipPtr_0 = (*ElementPtr).pParent as *mut STARSHIP;
            if frame_index as ::core::ffi::c_int
                == (*StarShipPtr_0).ShipFacing as ::core::ffi::c_int
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
            if optAdvancedAutoPilot.0 != 0 {
                uqm_lua_checked!(DoAdvancedAutoPilot(), BOOLEAN(0));
            }
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
        && uqm_lua_checked!(getGameStateUint(b"PORTAL_COUNTER\0".as_ptr() as *const ::core::ffi::c_char), ())
            == 0 as DWORD
    {
        let mut dx: SIZE = 0;
        let mut dy: SIZE = 0;
        let mut pt: POINT = POINT { x: 0, y: 0 };
        let mut SDPtr: *mut STAR_DESC = ::core::ptr::null_mut::<STAR_DESC>();
        let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<STARSHIP>();
        ElementToUniverse(ElementPtr0, &raw mut pt);
        SDPtr = uqm_lua_checked!(FindStar(
            ::core::ptr::null_mut::<STAR_DESC>(),
            &raw mut pt,
            5 as SIZE,
            5 as SIZE,
        ), ());
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
            & C2Rust_Unnamed_6::IN_BATTLE.0 as ::core::ffi::c_int != 0
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
            & C2Rust_Unnamed_6::IN_BATTLE.0 as ::core::ffi::c_int != 0
    {
        CurStarDescPtr = ::core::ptr::null_mut::<STAR_DESC>();
    }
}
unsafe extern "C" fn arilou_space_death(mut ElementPtr: *mut ELEMENT) {
    if (*ElementPtr).state_flags as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int == 0
        || uqm_lua_checked!(getGameStateUint(
            b"ARILOU_SPACE_COUNTER\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) == 0 as DWORD
    {
        if uqm_lua_checked!(getGameStateUint(
            b"ARILOU_SPACE_SIDE\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) <= 1 as DWORD
        {
            uqm_lua_checked!(setGameStateUint(
                b"ARILOU_SPACE_SIDE\0".as_ptr() as *const ::core::ffi::c_char,
                0 as DWORD,
            ), ());
        } else {
            uqm_lua_checked!(setGameStateUint(
                b"ARILOU_SPACE_SIDE\0".as_ptr() as *const ::core::ffi::c_char,
                3 as DWORD,
            ), ());
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
    which_side = uqm_lua_checked!(getGameStateUint(
        b"ARILOU_SPACE_SIDE\0".as_ptr() as *const ::core::ffi::c_char,
    ), ()) as COUNT;
    if which_side as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        || which_side as ::core::ffi::c_int == 3 as ::core::ffi::c_int
    {
        if (*ElementPtr1).state_flags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int != 0
        {
            uqm_lua_checked!(setGameStateUint(
                b"ARILOU_SPACE_SIDE\0".as_ptr() as *const ::core::ffi::c_char,
                (which_side as ::core::ffi::c_int ^ 1 as ::core::ffi::c_int) as DWORD,
            ), ());
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
            + (((CanvasWidth - ((64 as SDWORD) << resolutionFactor)
                - (if if optWindowType == 1 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    16 as SDWORD
                } else {
                    0 as SDWORD
                } << resolutionFactor) * 2 as SDWORD) << 2 as ::core::ffi::c_int)
                << 3 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
            - GlobData.SIS_state.log_x) as ::core::ffi::c_long;
        (*HyperSpaceElementPtr).current.location.x = (if lx < 0 as ::core::ffi::c_long {
            lx
                + (((CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
                    - (if if optWindowType == 1 as DWORD {
                        BOOLEAN::TRUE.0 as ::core::ffi::c_int
                    } else {
                        BOOLEAN::FALSE.0 as ::core::ffi::c_int
                    } != 0
                    {
                        16 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    } << resolutionFactor) * 2 as ::core::ffi::c_int)
                    << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                    as ::core::ffi::c_long
        } else if lx
            >= (((CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
                - (if if optWindowType == 1 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    16 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                } << resolutionFactor) * 2 as ::core::ffi::c_int)
                << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                as ::core::ffi::c_long
        {
            lx
                - (((CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
                    - (if if optWindowType == 1 as DWORD {
                        BOOLEAN::TRUE.0 as ::core::ffi::c_int
                    } else {
                        BOOLEAN::FALSE.0 as ::core::ffi::c_int
                    } != 0
                    {
                        16 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    } << resolutionFactor) * 2 as ::core::ffi::c_int)
                    << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                    as ::core::ffi::c_long
        } else {
            lx
        }) as DWORD as COORD;
        ly = (universeToLogy((*elem_pt).y)
            + (((CanvasHeight
                - (if if optWindowType == 1 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    16 as SDWORD
                } else {
                    0 as SDWORD
                } << resolutionFactor) * 2 as SDWORD) << 2 as ::core::ffi::c_int)
                << 3 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
            - GlobData.SIS_state.log_y) as ::core::ffi::c_long;
        (*HyperSpaceElementPtr).current.location.y = (if ly < 0 as ::core::ffi::c_long {
            ly
                + (((CanvasHeight
                    - (if if optWindowType == 1 as DWORD {
                        BOOLEAN::TRUE.0 as ::core::ffi::c_int
                    } else {
                        BOOLEAN::FALSE.0 as ::core::ffi::c_int
                    } != 0
                    {
                        16 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    } << resolutionFactor) * 2 as ::core::ffi::c_int)
                    << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                    as ::core::ffi::c_long
        } else if ly
            >= (((CanvasHeight
                - (if if optWindowType == 1 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    16 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                } << resolutionFactor) * 2 as ::core::ffi::c_int)
                << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                as ::core::ffi::c_long
        {
            ly
                - (((CanvasHeight
                    - (if if optWindowType == 1 as DWORD {
                        BOOLEAN::TRUE.0 as ::core::ffi::c_int
                    } else {
                        BOOLEAN::FALSE.0 as ::core::ffi::c_int
                    } != 0
                    {
                        16 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    } << resolutionFactor) * 2 as ::core::ffi::c_int)
                    << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                    as ::core::ffi::c_long
        } else {
            ly
        }) as DWORD as COORD;
        DisplayArray[(*HyperSpaceElementPtr).PrimIndex as usize].Type = gfx_object::STAMP_PRIM
            .0 as ::core::ffi::c_int as GRAPHICS_PRIM;
        (*HyperSpaceElementPtr).current.image.farray = (&raw mut hyperstars
            as *mut FRAME)
            .offset(1isize);
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
        if if resolutionFactor != 2 as ::core::ffi::c_uint {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } != 0 && uqm_lua_checked!(inQuasiSpace(), ()).0 != 0
        {
            DisplayArray[(*HyperSpaceElementPtr).PrimIndex as usize].flags = ((1
                as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as BYTE;
        }
        rand_val = TFB_Random();
        dy = rand_val as UWORD as SIZE;
        if if resolutionFactor != 2 as ::core::ffi::c_uint {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } == 0
        {
            dx = ((dy as UWORD as BYTE as ::core::ffi::c_int
                % (CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
                    - (if if optWindowType == 1 as DWORD {
                        BOOLEAN::TRUE.0 as ::core::ffi::c_int
                    } else {
                        BOOLEAN::FALSE.0 as ::core::ffi::c_int
                    } != 0
                    {
                        16 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    } << resolutionFactor) * 2 as ::core::ffi::c_int)) as SIZE
                as ::core::ffi::c_int
                - (CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
                    - (if if optWindowType == 1 as DWORD {
                        BOOLEAN::TRUE.0 as ::core::ffi::c_int
                    } else {
                        BOOLEAN::FALSE.0 as ::core::ffi::c_int
                    } != 0
                    {
                        16 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    } << resolutionFactor) * 2 as ::core::ffi::c_int
                    >> 1 as ::core::ffi::c_int)) as SIZE;
            dy = (((dy as UWORD as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as BYTE
                as ::core::ffi::c_int
                % (CanvasHeight
                    - (if if optWindowType == 1 as DWORD {
                        BOOLEAN::TRUE.0 as ::core::ffi::c_int
                    } else {
                        BOOLEAN::FALSE.0 as ::core::ffi::c_int
                    } != 0
                    {
                        16 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    } << resolutionFactor) * 2 as ::core::ffi::c_int)) as SIZE
                as ::core::ffi::c_int
                - (CanvasHeight
                    - (if if optWindowType == 1 as DWORD {
                        BOOLEAN::TRUE.0 as ::core::ffi::c_int
                    } else {
                        BOOLEAN::FALSE.0 as ::core::ffi::c_int
                    } != 0
                    {
                        16 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    } << resolutionFactor) * 2 as ::core::ffi::c_int
                    >> 1 as ::core::ffi::c_int)) as SIZE;
        } else {
            dx = (((rand_val >> 16 as ::core::ffi::c_int) as UWORD as ::core::ffi::c_int
                % (CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
                    - (if if optWindowType == 1 as DWORD {
                        BOOLEAN::TRUE.0 as ::core::ffi::c_int
                    } else {
                        BOOLEAN::FALSE.0 as ::core::ffi::c_int
                    } != 0
                    {
                        16 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    } << resolutionFactor) * 2 as ::core::ffi::c_int)) as SIZE
                as ::core::ffi::c_int
                - (CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
                    - (if if optWindowType == 1 as DWORD {
                        BOOLEAN::TRUE.0 as ::core::ffi::c_int
                    } else {
                        BOOLEAN::FALSE.0 as ::core::ffi::c_int
                    } != 0
                    {
                        16 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    } << resolutionFactor) * 2 as ::core::ffi::c_int
                    >> 1 as ::core::ffi::c_int)) as SIZE;
            dy = ((dy as ::core::ffi::c_int
                % (CanvasHeight
                    - (if if optWindowType == 1 as DWORD {
                        BOOLEAN::TRUE.0 as ::core::ffi::c_int
                    } else {
                        BOOLEAN::FALSE.0 as ::core::ffi::c_int
                    } != 0
                    {
                        16 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    } << resolutionFactor) * 2 as ::core::ffi::c_int)) as SIZE
                as ::core::ffi::c_int
                - (CanvasHeight
                    - (if if optWindowType == 1 as DWORD {
                        BOOLEAN::TRUE.0 as ::core::ffi::c_int
                    } else {
                        BOOLEAN::FALSE.0 as ::core::ffi::c_int
                    } != 0
                    {
                        16 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    } << resolutionFactor) * 2 as ::core::ffi::c_int
                    >> 1 as ::core::ffi::c_int)) as SIZE;
        }
        (*HyperSpaceElementPtr).current.location.x = ((((CanvasWidth
            - ((64 as ::core::ffi::c_int) << resolutionFactor)
            - (if if optWindowType == 1 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                16 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } << resolutionFactor) * 2 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
            << 3 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
            + ((dx as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)) as COORD;
        (*HyperSpaceElementPtr).current.location.y = ((((CanvasHeight
            - (if if optWindowType == 1 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                16 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } << resolutionFactor) * 2 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
            << 3 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
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
unsafe extern "C" fn encounter_animation(mut ElementPtr: *mut ELEMENT) {
    let mut f_index: COUNT = GetFrameIndex((*ElementPtr).current.image.frame);
    if f_index as ::core::ffi::c_int
        >= 139 as ::core::ffi::c_int + 31 as ::core::ffi::c_int
    {
        (*ElementPtr).next.image.frame = SetAbsFrameIndex(
            (*ElementPtr).current.image.frame,
            139 as COUNT,
        );
    } else {
        (*ElementPtr).next.image.frame = IncFrameIndex(
            (*ElementPtr).current.image.frame,
        );
    };
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
                if if resolutionFactor != 2 as ::core::ffi::c_uint {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } != 0 && optHyperStars.0 != 0
                {
                    (*ElementPtr).preprocess_func = Some(
                        encounter_animation as unsafe extern "C" fn(*mut ELEMENT) -> (),
                    ) as Option<ElementProcessFunc>;
                }
                (*ElementPtr).death_func = None;
            }
        }
        (*ElementPtr).c2rust_unnamed_2.turn_wait = (if if resolutionFactor
            != 2 as ::core::ffi::c_uint
        {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } != 0 && optHyperStars.0 != 0
        {
            0 as ::core::ffi::c_int
        } else {
            1 as ::core::ffi::c_int
        }) as BYTE;
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
                1279 as ::core::ffi::c_int,
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
            & C2Rust_Unnamed_6::IN_BATTLE.0 as ::core::ffi::c_int == 0
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
    if uqm_lua_checked!(getGameStateUint(b"ARILOU_SPACE_SIDE\0".as_ptr() as *const ::core::ffi::c_char), ::core::ptr::null_mut())
        >= 2 as DWORD
    {
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    if (*EncounterPtr).flags as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int != 0
    {
        (*EncounterPtr).flags = ((*EncounterPtr).flags as ::core::ffi::c_int
            & !((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)) as BYTE;
        (*EncounterPtr).transition_state = (100 as ::core::ffi::c_int
            + if if resolutionFactor != 2 as ::core::ffi::c_uint {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } != 0 && optHyperStars.0 != 0
            {
                8 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as SIZE;
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
        let mut EncounterMakeup: [BYTE; 26] = [
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
            0 as BYTE,
            0 as BYTE,
            0 as BYTE,
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
                    % (((((63 as ::core::ffi::c_int) << resolutionFactor)
                        * 12 as ::core::ffi::c_int >> resolutionFactor)
                        + ((63 as ::core::ffi::c_int) << resolutionFactor
                            << 2 as ::core::ffi::c_int) >> 1 as ::core::ffi::c_int)
                        << 1 as ::core::ffi::c_int)
                - ((((63 as ::core::ffi::c_int) << resolutionFactor)
                    * 12 as ::core::ffi::c_int >> resolutionFactor)
                    + ((63 as ::core::ffi::c_int) << resolutionFactor
                        << 2 as ::core::ffi::c_int) >> 1 as ::core::ffi::c_int))
                as COORD;
            if (enc_pt.x as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                enc_pt.x = 0 as COORD;
            } else if enc_pt.x as ::core::ffi::c_int > 9999 as ::core::ffi::c_int {
                enc_pt.x = 9999 as COORD;
            }
            enc_pt.y = ((*puniverse).y as ::core::ffi::c_int
                + (rand_val >> 16 as ::core::ffi::c_int) as UWORD as ::core::ffi::c_int
                    % (((((50 as ::core::ffi::c_int) << resolutionFactor)
                        * 12 as ::core::ffi::c_int >> resolutionFactor)
                        + ((50 as ::core::ffi::c_int) << resolutionFactor
                            << 2 as ::core::ffi::c_int) >> 1 as ::core::ffi::c_int)
                        << 1 as ::core::ffi::c_int)
                - ((((50 as ::core::ffi::c_int) << resolutionFactor)
                    * 12 as ::core::ffi::c_int >> resolutionFactor)
                    + ((50 as ::core::ffi::c_int) << resolutionFactor
                        << 2 as ::core::ffi::c_int) >> 1 as ::core::ffi::c_int))
                as COORD;
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
                || i_0 as ::core::ffi::c_int
                    > if if resolutionFactor != 2 as ::core::ffi::c_uint {
                        BOOLEAN::FALSE.0 as ::core::ffi::c_int
                    } else {
                        BOOLEAN::TRUE.0 as ::core::ffi::c_int
                    } != 0 && optHyperStars.0 != 0
                    {
                        18 as ::core::ffi::c_int
                    } else {
                        9 as ::core::ffi::c_int
                    }
            {
                i_0 = (if if resolutionFactor != 2 as ::core::ffi::c_uint {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } != 0 && optHyperStars.0 != 0
                {
                    18 as ::core::ffi::c_int
                } else {
                    9 as ::core::ffi::c_int
                }) as SIZE;
            }
            (*ElementPtr).current.image.frame = SetRelFrameIndex(
                *(*ElementPtr).current.image.farray.offset(0isize),
                -(i_0 as ::core::ffi::c_int) as SIZE,
            );
            (*ElementPtr).death_func = Some(
                encounter_transition as unsafe extern "C" fn(*mut ELEMENT) -> (),
            ) as Option<ElementProcessFunc>;
            (*ElementPtr).preprocess_func = None;
        } else if if resolutionFactor != 2 as ::core::ffi::c_uint {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } != 0 && optHyperStars.0 != 0
        {
            (*ElementPtr).preprocess_func = Some(
                encounter_animation as unsafe extern "C" fn(*mut ELEMENT) -> (),
            ) as Option<ElementProcessFunc>;
            (*ElementPtr).current.image.frame = SetAbsFrameIndex(
                *(*ElementPtr).current.image.farray.offset(0isize),
                (139 as DWORD).wrapping_add(TFB_Random().wrapping_rem(31 as DWORD))
                    as COUNT,
            );
        } else {
            (*ElementPtr).current.image.frame = DecFrameIndex(
                *(*ElementPtr).current.image.farray.offset(0isize),
            );
            (*ElementPtr).preprocess_func = None;
        }
        (*ElementPtr).c2rust_unnamed_2.turn_wait = 1 as BYTE;
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
    uqm_lua_checked!(ClearDrawable(), ());
    SetContextForeGroundColor(GetColorMapColor(0x35 as COUNT, 0xfc as COUNT));
    sx = (ux as ::core::ffi::c_int
        - (((63 as ::core::ffi::c_int) << resolutionFactor) * 12 as ::core::ffi::c_int
            >> resolutionFactor >> 1 as ::core::ffi::c_int)) as COORD;
    if (sx as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
        sx = 0 as COORD;
    } else {
        sx = (sx as ::core::ffi::c_int
            - sx as ::core::ffi::c_int % 200 as ::core::ffi::c_int) as COORD;
    }
    ex = (ux as ::core::ffi::c_int
        + (((63 as ::core::ffi::c_int) << resolutionFactor) * 12 as ::core::ffi::c_int
            >> resolutionFactor >> 1 as ::core::ffi::c_int)) as COORD;
    if ex as ::core::ffi::c_int > 9999 as ::core::ffi::c_int + 1 as ::core::ffi::c_int {
        ex = (9999 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as COORD;
    }
    sy = (uy as ::core::ffi::c_int
        - (((50 as ::core::ffi::c_int) << resolutionFactor) * 12 as ::core::ffi::c_int
            >> resolutionFactor >> 1 as ::core::ffi::c_int)) as COORD;
    if (sy as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
        sy = 0 as COORD;
    } else {
        sy = (sy as ::core::ffi::c_int
            - sy as ::core::ffi::c_int % 200 as ::core::ffi::c_int) as COORD;
    }
    ey = (uy as ::core::ffi::c_int
        + (((50 as ::core::ffi::c_int) << resolutionFactor) * 12 as ::core::ffi::c_int
            >> resolutionFactor >> 1 as ::core::ffi::c_int)) as COORD;
    if ey as ::core::ffi::c_int > 9999 as ::core::ffi::c_int + 1 as ::core::ffi::c_int {
        ey = (9999 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as COORD;
    }
    r.corner.y = (((9999 as ::core::ffi::c_int - ey as ::core::ffi::c_int)
        as ::core::ffi::c_long
        * ((53 as ::core::ffi::c_int) << resolutionFactor) as ::core::ffi::c_long
        / (((50 as ::core::ffi::c_int) << resolutionFactor) * 12 as ::core::ffi::c_int
            >> resolutionFactor) as ::core::ffi::c_long) as COORD as ::core::ffi::c_int
        - oy as ::core::ffi::c_int) as COORD;
    r.extent.width = ((1 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    r.extent.height = (((9999 as ::core::ffi::c_int - sy as ::core::ffi::c_int)
        as ::core::ffi::c_long
        * ((53 as ::core::ffi::c_int) << resolutionFactor) as ::core::ffi::c_long
        / (((50 as ::core::ffi::c_int) << resolutionFactor) * 12 as ::core::ffi::c_int
            >> resolutionFactor) as ::core::ffi::c_long) as COORD as ::core::ffi::c_int
        - oy as ::core::ffi::c_int - r.corner.y as ::core::ffi::c_int
        + ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    ux = sx;
    while ux as ::core::ffi::c_int <= ex as ::core::ffi::c_int {
        r.corner.x = ((ux as ::core::ffi::c_long
            * (((64 as ::core::ffi::c_int) << resolutionFactor)
                - ((8 as ::core::ffi::c_int) << resolutionFactor)) as ::core::ffi::c_long
            / (((63 as ::core::ffi::c_int) << resolutionFactor)
                * 12 as ::core::ffi::c_int >> resolutionFactor) as ::core::ffi::c_long)
            as COORD as ::core::ffi::c_int - ox as ::core::ffi::c_int) as COORD;
        uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
        ux = (ux as ::core::ffi::c_int + 200 as ::core::ffi::c_int) as COORD;
    }
    r.corner.x = ((sx as ::core::ffi::c_long
        * (((64 as ::core::ffi::c_int) << resolutionFactor)
            - ((8 as ::core::ffi::c_int) << resolutionFactor)) as ::core::ffi::c_long
        / (((63 as ::core::ffi::c_int) << resolutionFactor) * 12 as ::core::ffi::c_int
            >> resolutionFactor) as ::core::ffi::c_long) as COORD as ::core::ffi::c_int
        - ox as ::core::ffi::c_int) as COORD;
    r.extent.width = ((ex as ::core::ffi::c_long
        * (((64 as ::core::ffi::c_int) << resolutionFactor)
            - ((8 as ::core::ffi::c_int) << resolutionFactor)) as ::core::ffi::c_long
        / (((63 as ::core::ffi::c_int) << resolutionFactor) * 12 as ::core::ffi::c_int
            >> resolutionFactor) as ::core::ffi::c_long) as COORD as ::core::ffi::c_int
        - ox as ::core::ffi::c_int - r.corner.x as ::core::ffi::c_int
        + ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    r.extent.height = ((1 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    uy = sy;
    while uy as ::core::ffi::c_int <= ey as ::core::ffi::c_int {
        r.corner.y = (((9999 as ::core::ffi::c_int - uy as ::core::ffi::c_int)
            as ::core::ffi::c_long
            * ((53 as ::core::ffi::c_int) << resolutionFactor) as ::core::ffi::c_long
            / (((50 as ::core::ffi::c_int) << resolutionFactor)
                * 12 as ::core::ffi::c_int >> resolutionFactor) as ::core::ffi::c_long)
            as COORD as ::core::ffi::c_int - oy as ::core::ffi::c_int) as COORD;
        uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
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
        && uqm_lua_checked!(AddEncounterElement(EncounterPtr, puniverse), false).is_null()
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
                >= if if resolutionFactor != 2 as ::core::ffi::c_uint {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } != 0 && optHyperStars.0 != 0
                {
                    18 as ::core::ffi::c_int
                } else {
                    9 as ::core::ffi::c_int
                }
            {
                (*ElementPtr).c2rust_unnamed_2.turn_wait = (*ElementPtr)
                    .c2rust_unnamed_2
                    .turn_wait
                    .wrapping_add(1);
            } else if (*EncounterPtr).transition_state as ::core::ffi::c_int
                == -if if resolutionFactor != 2 as ::core::ffi::c_uint {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } != 0 && optHyperStars.0 != 0
                {
                    18 as ::core::ffi::c_int
                } else {
                    9 as ::core::ffi::c_int
                }
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
                    (*puniverse).x as SDWORD - (*EncounterPtr).loc_pt.x as SDWORD,
                    (*puniverse).y as SDWORD - (*EncounterPtr).loc_pt.y as SDWORD,
                ) as ::core::ffi::c_int
                    + ((1 as ::core::ffi::c_int)
                        << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int)
                    >> 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int)
                & ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int) as DWORD as COUNT;
            if delta_facing as ::core::ffi::c_int != 0
                || delta_x as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                    && delta_y as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            {
                let mut speed: SIZE = 0;
                speed = RaceHyperSpeed(
                    RACE_ID((*EncounterPtr).race_id as ::core::ffi::c_uint),
                );
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
                        - 1 as ::core::ffi::c_int) as DWORD
                    > (((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                        >> 2 as ::core::ffi::c_int)
                        + ((1 as ::core::ffi::c_int)
                            << 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int)
                        >> 6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int) as DWORD
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
                    (*(&raw mut sinetab as *mut SDWORD)
                        .offset(
                            (cur_facing as ::core::ffi::c_int
                                + ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                                    >> 2 as ::core::ffi::c_int)
                                & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                                    - 1 as ::core::ffi::c_int) as DWORD as isize,
                        ) as ::core::ffi::c_long * speed as ::core::ffi::c_long
                        >> 14 as ::core::ffi::c_int) as SDWORD,
                    (*(&raw mut sinetab as *mut SDWORD)
                        .offset(
                            (cur_facing as ::core::ffi::c_int
                                & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                                    - 1 as ::core::ffi::c_int) as DWORD as isize,
                        ) as ::core::ffi::c_long * speed as ::core::ffi::c_long
                        >> 14 as ::core::ffi::c_int) as SDWORD,
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
        (*EncounterPtr).loc_pt.x = logxToUniverse((*EncounterPtr).log_x) as COORD;
        (*EncounterPtr).loc_pt.y = logyToUniverse((*EncounterPtr).log_y) as COORD;
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
                if if resolutionFactor != 2 as ::core::ffi::c_uint {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } != 0 && optHyperStars.0 != 0
                {
                    (*ElementPtr).preprocess_func = None;
                    (*ElementPtr).current.image.frame = *(*ElementPtr)
                        .current
                        .image
                        .farray
                        .offset(0isize);
                }
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
        >= -((63 as ::core::ffi::c_int) << resolutionFactor)
        && ex as ::core::ffi::c_int - (*puniverse).x as ::core::ffi::c_int
            <= (63 as ::core::ffi::c_int) << resolutionFactor
        && ey as ::core::ffi::c_int - (*puniverse).y as ::core::ffi::c_int
            >= -((50 as ::core::ffi::c_int) << resolutionFactor)
        && ey as ::core::ffi::c_int - (*puniverse).y as ::core::ffi::c_int
            <= (50 as ::core::ffi::c_int) << resolutionFactor
    {
        (*ElementPtr).next.location.x = (((*EncounterPtr).log_x
            - GlobData.SIS_state.log_x) as SIZE as ::core::ffi::c_int
            + (((CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
                - (if if optWindowType == 1 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    16 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                } << resolutionFactor) * 2 as ::core::ffi::c_int)
                << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int)) as COORD;
        (*ElementPtr).next.location.y = (((*EncounterPtr).log_y
            - GlobData.SIS_state.log_y) as SIZE as ::core::ffi::c_int
            + (((CanvasHeight
                - (if if optWindowType == 1 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    16 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                } << resolutionFactor) * 2 as ::core::ffi::c_int)
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
            < -((((63 as ::core::ffi::c_int) << resolutionFactor)
                * 12 as ::core::ffi::c_int >> resolutionFactor)
                + ((63 as ::core::ffi::c_int) << resolutionFactor
                    << 2 as ::core::ffi::c_int) >> 1 as ::core::ffi::c_int)
            || ex as ::core::ffi::c_int - (*puniverse).x as ::core::ffi::c_int
                > (((63 as ::core::ffi::c_int) << resolutionFactor)
                    * 12 as ::core::ffi::c_int >> resolutionFactor)
                    + ((63 as ::core::ffi::c_int) << resolutionFactor
                        << 2 as ::core::ffi::c_int) >> 1 as ::core::ffi::c_int
            || (ey as ::core::ffi::c_int - (*puniverse).y as ::core::ffi::c_int)
                < -((((50 as ::core::ffi::c_int) << resolutionFactor)
                    * 12 as ::core::ffi::c_int >> resolutionFactor)
                    + ((50 as ::core::ffi::c_int) << resolutionFactor
                        << 2 as ::core::ffi::c_int) >> 1 as ::core::ffi::c_int)
            || ey as ::core::ffi::c_int - (*puniverse).y as ::core::ffi::c_int
                > (((50 as ::core::ffi::c_int) << resolutionFactor)
                    * 12 as ::core::ffi::c_int >> resolutionFactor)
                    + ((50 as ::core::ffi::c_int) << resolutionFactor
                        << 2 as ::core::ffi::c_int) >> 1 as ::core::ffi::c_int
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
        * (((64 as ::core::ffi::c_int) << resolutionFactor)
            - ((8 as ::core::ffi::c_int) << resolutionFactor)) as ::core::ffi::c_long
        / (((63 as ::core::ffi::c_int) << resolutionFactor) * 12 as ::core::ffi::c_int
            >> resolutionFactor) as ::core::ffi::c_long) as COORD as ::core::ffi::c_int
        - ox as ::core::ffi::c_int) as COORD;
    (*stamp).origin.y = (((9999 as ::core::ffi::c_int - ey as ::core::ffi::c_int)
        as ::core::ffi::c_long
        * ((53 as ::core::ffi::c_int) << resolutionFactor) as ::core::ffi::c_long
        / (((50 as ::core::ffi::c_int) << resolutionFactor) * 12 as ::core::ffi::c_int
            >> resolutionFactor) as ::core::ffi::c_long) as COORD as ::core::ffi::c_int
        - oy as ::core::ffi::c_int) as COORD;
    uqm_lua_checked!(DrawStamp(stamp), false);
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
    stamp.frame = SetAbsFrameIndex(hyperstars[0usize], 1 as COUNT);
    hEncounter = GlobData.Game_state.encounter_q.head as HENCOUNTER;
    while !hEncounter.is_null() {
        let mut EncounterPtr: *mut ENCOUNTER = ::core::ptr::null_mut::<ENCOUNTER>();
        EncounterPtr = LockLink(&raw mut GlobData.Game_state.encounter_q, hEncounter)
            as *mut ENCOUNTER;
        hNextEncounter = (*EncounterPtr).succ;
        if !uqm_lua_checked!(ProcessEncounter(EncounterPtr, puniverse, ox, oy, &raw mut stamp), ()) {
            UnlockLink(&raw mut GlobData.Game_state.encounter_q, hEncounter);
            RemoveQueue(&raw mut GlobData.Game_state.encounter_q, hEncounter);
            FreeLink(&raw mut GlobData.Game_state.encounter_q, hEncounter);
        } else {
            UnlockLink(&raw mut GlobData.Game_state.encounter_q, hEncounter);
        }
        hEncounter = hNextEncounter;
    }
}
unsafe extern "C" fn GetMaskColor(mut startype: BYTE) -> Color {
    match startype as ::core::ffi::c_int {
        3 => {
            return buildColorRgba(0xf8 as BYTE, 0x20 as BYTE, 0x20 as BYTE, 0xff as BYTE);
        }
        2 => return buildColorRgba(0xff as BYTE, 0x8a as BYTE, 0 as BYTE, 0xff as BYTE),
        5 => {
            return buildColorRgba(0xde as BYTE, 0xc8 as BYTE, 0x10 as BYTE, 0xff as BYTE);
        }
        1 => return buildColorRgba(0 as BYTE, 0xc8 as BYTE, 0 as BYTE, 0xff as BYTE),
        0 => {
            return buildColorRgba(0x48 as BYTE, 0x60 as BYTE, 0xf0 as BYTE, 0xff as BYTE);
        }
        _ => return colorFromRgb15(0x1f as BYTE, 0x1f as BYTE, 0x1f as BYTE),
    };
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
    let mut s: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    let mut SDPtr: *mut STAR_DESC = ::core::ptr::null_mut::<STAR_DESC>();
    let mut hHyperSpaceElement: HELEMENT = ::core::ptr::null_mut::<
        ::core::ffi::c_void,
    >();
    let mut HyperSpaceElementPtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
    let mut lx: SDWORD = 0;
    let mut ly: SDWORD = 0;
    let mut frameRect: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut img_log: DEXTENT = DEXTENT { width: 0, height: 0 };
    static mut frameCounter: COUNT = 0 as COUNT;
    universe.x = logxToUniverse(GlobData.SIS_state.log_x) as COORD;
    universe.y = logyToUniverse(GlobData.SIS_state.log_y) as COORD;
    SetContext(RadarContext);
    BatchGraphics();
    ox = ((universe.x as ::core::ffi::c_long
        * (((64 as ::core::ffi::c_int) << resolutionFactor)
            - ((8 as ::core::ffi::c_int) << resolutionFactor)) as ::core::ffi::c_long
        / (((63 as ::core::ffi::c_int) << resolutionFactor) * 12 as ::core::ffi::c_int
            >> resolutionFactor) as ::core::ffi::c_long) as COORD as ::core::ffi::c_int
        - (((64 as ::core::ffi::c_int) << resolutionFactor)
            - ((8 as ::core::ffi::c_int) << resolutionFactor)
            >> 1 as ::core::ffi::c_int)) as COORD;
    oy = (((9999 as ::core::ffi::c_int - universe.y as ::core::ffi::c_int)
        as ::core::ffi::c_long
        * ((53 as ::core::ffi::c_int) << resolutionFactor) as ::core::ffi::c_long
        / (((50 as ::core::ffi::c_int) << resolutionFactor) * 12 as ::core::ffi::c_int
            >> resolutionFactor) as ::core::ffi::c_long) as COORD as ::core::ffi::c_int
        - ((53 as ::core::ffi::c_int) << resolutionFactor >> 1 as ::core::ffi::c_int))
        as COORD;
    ex = ((GlobData.Game_state.ShipStamp.origin.x as ::core::ffi::c_long
        * (((64 as ::core::ffi::c_int) << resolutionFactor)
            - ((8 as ::core::ffi::c_int) << resolutionFactor)) as ::core::ffi::c_long
        / (((63 as ::core::ffi::c_int) << resolutionFactor) * 12 as ::core::ffi::c_int
            >> resolutionFactor) as ::core::ffi::c_long) as COORD as ::core::ffi::c_int
        - (((64 as ::core::ffi::c_int) << resolutionFactor)
            - ((8 as ::core::ffi::c_int) << resolutionFactor)
            >> 1 as ::core::ffi::c_int)) as COORD;
    ey = (((9999 as ::core::ffi::c_int
        - GlobData.Game_state.ShipStamp.origin.y as ::core::ffi::c_int)
        as ::core::ffi::c_long
        * ((53 as ::core::ffi::c_int) << resolutionFactor) as ::core::ffi::c_long
        / (((50 as ::core::ffi::c_int) << resolutionFactor) * 12 as ::core::ffi::c_int
            >> resolutionFactor) as ::core::ffi::c_long) as COORD as ::core::ffi::c_int
        - ((53 as ::core::ffi::c_int) << resolutionFactor >> 1 as ::core::ffi::c_int))
        as COORD;
    arilouSpaceCounter = uqm_lua_checked!(getGameStateUint(
        b"ARILOU_SPACE_COUNTER\0".as_ptr() as *const ::core::ffi::c_char,
    ), ()) as SWORD;
    arilouSpaceSide = uqm_lua_checked!(getGameStateUint(
        b"ARILOU_SPACE_SIDE\0".as_ptr() as *const ::core::ffi::c_char,
    ), ()) as SWORD;
    uqm_lua_checked!(DrawHyperGrid(universe.x, universe.y, ox, oy), ());
    SDPtr = ::core::ptr::null_mut::<STAR_DESC>();
    loop {
        SDPtr = uqm_lua_checked!(FindStar(
            SDPtr,
            &raw mut universe,
            ((((63 as ::core::ffi::c_int) << resolutionFactor) * 12 as ::core::ffi::c_int
                >> resolutionFactor)
                + ((63 as ::core::ffi::c_int) << resolutionFactor
                    << 2 as ::core::ffi::c_int) >> 1 as ::core::ffi::c_int) as SIZE,
            ((((50 as ::core::ffi::c_int) << resolutionFactor) * 12 as ::core::ffi::c_int
                >> resolutionFactor)
                + ((50 as ::core::ffi::c_int) << resolutionFactor
                    << 2 as ::core::ffi::c_int) >> 1 as ::core::ffi::c_int) as SIZE,
        ), ());
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
            && ex as ::core::ffi::c_int == 6134 as ::core::ffi::c_int
            && ey as ::core::ffi::c_int == 5900 as ::core::ffi::c_int
        {
            star_type = C2Rust_Unnamed_7::SUPER_GIANT_STAR.0 as ::core::ffi::c_int
                as BYTE;
        }
        s.origin.x = ((ex as ::core::ffi::c_long
            * (((64 as ::core::ffi::c_int) << resolutionFactor)
                - ((8 as ::core::ffi::c_int) << resolutionFactor)) as ::core::ffi::c_long
            / (((63 as ::core::ffi::c_int) << resolutionFactor)
                * 12 as ::core::ffi::c_int >> resolutionFactor) as ::core::ffi::c_long)
            as COORD as ::core::ffi::c_int - ox as ::core::ffi::c_int) as COORD;
        s.origin.y = (((9999 as ::core::ffi::c_int - ey as ::core::ffi::c_int)
            as ::core::ffi::c_long
            * ((53 as ::core::ffi::c_int) << resolutionFactor) as ::core::ffi::c_long
            / (((50 as ::core::ffi::c_int) << resolutionFactor)
                * 12 as ::core::ffi::c_int >> resolutionFactor) as ::core::ffi::c_long)
            as COORD as ::core::ffi::c_int - oy as ::core::ffi::c_int) as COORD;
        s.frame = SetRelFrameIndex(
            hyperstars[0usize],
            (star_type as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as SIZE,
        );
        uqm_lua_checked!(DrawStamp(&raw mut s), ());
    }
    portalCounter = uqm_lua_checked!(getGameStateUint(
        b"PORTAL_COUNTER\0".as_ptr() as *const ::core::ffi::c_char,
    ), ()) as SWORD;
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
            if if resolutionFactor != 2 as ::core::ffi::c_uint {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } != 0 && optHyperStars.0 != 0
            {
                SD[i as usize].Index = (portalCounter as ::core::ffi::c_int
                    + 98 as ::core::ffi::c_int) as BYTE;
                if SD[i as usize].Index as ::core::ffi::c_int > 107 as ::core::ffi::c_int
                {
                    SD[i as usize].Index = SD[i as usize].Index.wrapping_sub(1);
                }
            } else {
                SD[i as usize].Index = ((portalCounter as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
                    + 18 as ::core::ffi::c_int) as BYTE;
            }
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
            uqm_lua_checked!(setGameStateUint(
                b"PORTAL_COUNTER\0".as_ptr() as *const ::core::ffi::c_char,
                portalCounter as DWORD,
            ), ());
        }
        if arilouSpaceCounter != 0 {
            if if resolutionFactor != 2 as ::core::ffi::c_uint {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } != 0 && optHyperStars.0 != 0
            {
                SD[i as usize].Index = (arilouSpaceCounter as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as BYTE;
            } else {
                SD[i as usize].Index = (arilouSpaceCounter as ::core::ffi::c_int
                    >> 1 as ::core::ffi::c_int) as BYTE;
            }
            if arilouSpaceSide as ::core::ffi::c_int <= 1 as ::core::ffi::c_int {
                if if resolutionFactor != 2 as ::core::ffi::c_uint {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } != 0 && optHyperStars.0 != 0
                {
                    SD[i as usize].Index = (SD[i as usize].Index as ::core::ffi::c_int
                        + 81 as ::core::ffi::c_int) as BYTE;
                }
                SD[i as usize].Index = (SD[i as usize].Index as ::core::ffi::c_int
                    + 18 as ::core::ffi::c_int) as BYTE;
                SD[i as usize].star_pt = plot_map[C2Rust_Unnamed_9::ARILOU_DEFINED.0
                        as ::core::ffi::c_int as usize]
                    .star_pt;
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
                + ((((63 as ::core::ffi::c_int) << resolutionFactor)
                    * 12 as ::core::ffi::c_int >> resolutionFactor)
                    + ((63 as ::core::ffi::c_int) << resolutionFactor
                        << 2 as ::core::ffi::c_int) >> 1 as ::core::ffi::c_int))
                as COORD;
            sy = (SD[i as usize].star_pt.y as ::core::ffi::c_int
                - universe.y as ::core::ffi::c_int
                + ((((50 as ::core::ffi::c_int) << resolutionFactor)
                    * 12 as ::core::ffi::c_int >> resolutionFactor)
                    + ((50 as ::core::ffi::c_int) << resolutionFactor
                        << 2 as ::core::ffi::c_int) >> 1 as ::core::ffi::c_int))
                as COORD;
            if !((sx as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
                || (sy as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
                || sx as ::core::ffi::c_int
                    >= ((((63 as ::core::ffi::c_int) << resolutionFactor)
                        * 12 as ::core::ffi::c_int >> resolutionFactor)
                        + ((63 as ::core::ffi::c_int) << resolutionFactor
                            << 2 as ::core::ffi::c_int) >> 1 as ::core::ffi::c_int)
                        << 1 as ::core::ffi::c_int
                || sy as ::core::ffi::c_int
                    >= ((((50 as ::core::ffi::c_int) << resolutionFactor)
                        * 12 as ::core::ffi::c_int >> resolutionFactor)
                        + ((50 as ::core::ffi::c_int) << resolutionFactor
                            << 2 as ::core::ffi::c_int) >> 1 as ::core::ffi::c_int)
                        << 1 as ::core::ffi::c_int)
            {
                ex = SD[i as usize].star_pt.x;
                ey = SD[i as usize].star_pt.y;
                s.origin.x = ((ex as ::core::ffi::c_long
                    * (((64 as ::core::ffi::c_int) << resolutionFactor)
                        - ((8 as ::core::ffi::c_int) << resolutionFactor))
                        as ::core::ffi::c_long
                    / (((63 as ::core::ffi::c_int) << resolutionFactor)
                        * 12 as ::core::ffi::c_int >> resolutionFactor)
                        as ::core::ffi::c_long) as COORD as ::core::ffi::c_int
                    - ox as ::core::ffi::c_int) as COORD;
                s.origin.y = (((9999 as ::core::ffi::c_int - ey as ::core::ffi::c_int)
                    as ::core::ffi::c_long
                    * ((53 as ::core::ffi::c_int) << resolutionFactor)
                        as ::core::ffi::c_long
                    / (((50 as ::core::ffi::c_int) << resolutionFactor)
                        * 12 as ::core::ffi::c_int >> resolutionFactor)
                        as ::core::ffi::c_long) as COORD as ::core::ffi::c_int
                    - oy as ::core::ffi::c_int) as COORD;
                s.frame = SetAbsFrameIndex(hyperstars[0usize], 5 as COUNT);
                uqm_lua_checked!(DrawStamp(&raw mut s), ());
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
                    > ((((63 as ::core::ffi::c_int) << resolutionFactor)
                        * 12 as ::core::ffi::c_int >> resolutionFactor)
                        + ((63 as ::core::ffi::c_int) << resolutionFactor
                            << 2 as ::core::ffi::c_int) >> 1 as ::core::ffi::c_int)
                        / 12 as ::core::ffi::c_int
                    || ey as ::core::ffi::c_int
                        > ((((50 as ::core::ffi::c_int) << resolutionFactor)
                            * 12 as ::core::ffi::c_int >> resolutionFactor)
                            + ((50 as ::core::ffi::c_int) << resolutionFactor
                                << 2 as ::core::ffi::c_int) >> 1 as ::core::ffi::c_int)
                            / 12 as ::core::ffi::c_int)
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
                        if if resolutionFactor != 2 as ::core::ffi::c_uint {
                            BOOLEAN::FALSE.0 as ::core::ffi::c_int
                        } else {
                            BOOLEAN::TRUE.0 as ::core::ffi::c_int
                        } != 0 && optHyperStars.0 != 0
                            && (SD[i as usize].Index as ::core::ffi::c_int
                                == 107 as ::core::ffi::c_int
                                && arilouSpaceSide as ::core::ffi::c_int
                                    <= 1 as ::core::ffi::c_int
                                || SD[i as usize].Index as ::core::ffi::c_int
                                    == 8 as ::core::ffi::c_int
                                    && arilouSpaceSide as ::core::ffi::c_int
                                        > 1 as ::core::ffi::c_int)
                        {
                            (*HyperSpaceElementPtr).current.image.frame = SetAbsFrameIndex(
                                hyperstars[(1 as ::core::ffi::c_int
                                    + (arilouSpaceSide as ::core::ffi::c_int
                                        >> 1 as ::core::ffi::c_int)) as usize],
                                (frameCounter as ::core::ffi::c_int
                                    + SD[i as usize].Index as ::core::ffi::c_int) as COUNT,
                            );
                        } else {
                            (*HyperSpaceElementPtr).current.image.frame = SetAbsFrameIndex(
                                hyperstars[1usize],
                                SD[i as usize].Index as COUNT,
                            );
                        }
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
        SDPtr = uqm_lua_checked!(FindStar(
            SDPtr,
            &raw mut universe,
            ((((63 as ::core::ffi::c_int) << resolutionFactor) * 12 as ::core::ffi::c_int
                >> resolutionFactor)
                + ((63 as ::core::ffi::c_int) << resolutionFactor
                    << 2 as ::core::ffi::c_int) >> 1 as ::core::ffi::c_int) as SIZE,
            ((((50 as ::core::ffi::c_int) << resolutionFactor) * 12 as ::core::ffi::c_int
                >> resolutionFactor)
                + ((50 as ::core::ffi::c_int) << resolutionFactor
                    << 2 as ::core::ffi::c_int) >> 1 as ::core::ffi::c_int) as SIZE,
        ), ());
        if SDPtr.is_null() {
            break;
        }
        let mut star_frame: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
        let mut star_type_0: BYTE = (*SDPtr).Type;
        let mut star_color: BYTE = ((star_type_0 as ::core::ffi::c_int
            & ((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as BYTE
                as ::core::ffi::c_int) >> 5 as ::core::ffi::c_int) as BYTE;
        lx = universeToLogx((*SDPtr).star_pt.x) - GlobData.SIS_state.log_x;
        ly = universeToLogy((*SDPtr).star_pt.y) - GlobData.SIS_state.log_y;
        if if resolutionFactor != 2 as ::core::ffi::c_uint {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } != 0 && optHyperStars.0 != 0
        {
            if uqm_lua_checked!(getGameStateUint(
                b"ARILOU_SPACE_SIDE\0".as_ptr() as *const ::core::ffi::c_char,
            ), ()) > 1 as DWORD
            {
                star_frame = SetAbsFrameIndex(
                    hyperstars[1usize],
                    (frameCounter as ::core::ffi::c_int
                        + if ((star_type_0 as ::core::ffi::c_int
                            & ((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int)
                                as BYTE as ::core::ffi::c_int) >> 5 as ::core::ffi::c_int)
                            as BYTE as ::core::ffi::c_int
                            == C2Rust_Unnamed_8::YELLOW_BODY.0 as ::core::ffi::c_int
                        {
                            40 as ::core::ffi::c_int
                        } else {
                            8 as ::core::ffi::c_int
                        }) as COUNT,
                );
            } else {
                star_frame = SetAbsFrameIndex(
                    hyperstars[1usize],
                    (((star_type_0 as ::core::ffi::c_int
                        & ((0xff as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                            & !(((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int)
                                as BYTE as ::core::ffi::c_int)) as BYTE
                            as ::core::ffi::c_int) >> 3 as ::core::ffi::c_int) as BYTE
                        as ::core::ffi::c_int * 32 as ::core::ffi::c_int
                        + frameCounter as ::core::ffi::c_int) as COUNT,
                );
            }
        } else {
            star_frame = SetAbsFrameIndex(
                hyperstars[1usize],
                (((star_type_0 as ::core::ffi::c_int
                    & ((0xff as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                        & !(((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int)
                            as BYTE as ::core::ffi::c_int)) as BYTE
                        as ::core::ffi::c_int) >> 3 as ::core::ffi::c_int) as BYTE
                    as ::core::ffi::c_int
                    * C2Rust_Unnamed_8::NUM_STAR_COLORS.0 as ::core::ffi::c_int
                    + ((star_type_0 as ::core::ffi::c_int
                        & ((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int)
                            as BYTE as ::core::ffi::c_int) >> 5 as ::core::ffi::c_int)
                        as BYTE as ::core::ffi::c_int) as COUNT,
            );
        }
        if star_frame.is_null() {
            continue;
        }
        GetFrameRect(star_frame, &raw mut frameRect);
        img_log.width = ((frameRect.extent.width as ::core::ffi::c_int)
            << 1 as ::core::ffi::c_int) as SDWORD;
        img_log.height = ((frameRect.extent.height as ::core::ffi::c_int)
            << 1 as ::core::ffi::c_int) as SDWORD;
        if lx + img_log.width
            < -((CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
                - (if if optWindowType == 1 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    16 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                } << resolutionFactor) * 2 as ::core::ffi::c_int
                - (((13 as ::core::ffi::c_int) << resolutionFactor)
                    + (if if optWindowType == 1 as DWORD {
                        BOOLEAN::TRUE.0 as ::core::ffi::c_int
                    } else {
                        BOOLEAN::FALSE.0 as ::core::ffi::c_int
                    } != 0
                    {
                        1 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    } << resolutionFactor)) << 2 as ::core::ffi::c_int)
                + ((((63 as ::core::ffi::c_int) << resolutionFactor)
                    - ((50 as ::core::ffi::c_int) << resolutionFactor)
                    + 1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                >> 1 as ::core::ffi::c_int)
            || lx - img_log.width
                > (CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
                    - (if if optWindowType == 1 as DWORD {
                        BOOLEAN::TRUE.0 as ::core::ffi::c_int
                    } else {
                        BOOLEAN::FALSE.0 as ::core::ffi::c_int
                    } != 0
                    {
                        16 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    } << resolutionFactor) * 2 as ::core::ffi::c_int
                    - (((13 as ::core::ffi::c_int) << resolutionFactor)
                        + (if if optWindowType == 1 as DWORD {
                            BOOLEAN::TRUE.0 as ::core::ffi::c_int
                        } else {
                            BOOLEAN::FALSE.0 as ::core::ffi::c_int
                        } != 0
                        {
                            1 as ::core::ffi::c_int
                        } else {
                            0 as ::core::ffi::c_int
                        } << resolutionFactor)) << 2 as ::core::ffi::c_int)
                    - ((((63 as ::core::ffi::c_int) << resolutionFactor)
                        - ((50 as ::core::ffi::c_int) << resolutionFactor)
                        + 1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                    >> 1 as ::core::ffi::c_int
            || ly + img_log.height
                < -((CanvasHeight
                    - (if if optWindowType == 1 as DWORD {
                        BOOLEAN::TRUE.0 as ::core::ffi::c_int
                    } else {
                        BOOLEAN::FALSE.0 as ::core::ffi::c_int
                    } != 0
                    {
                        16 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    } << resolutionFactor) * 2 as ::core::ffi::c_int
                    - ((13 as ::core::ffi::c_int) << resolutionFactor)
                    << 2 as ::core::ffi::c_int)
                    + ((((63 as ::core::ffi::c_int) << resolutionFactor)
                        - ((50 as ::core::ffi::c_int) << resolutionFactor)
                        + 1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                    >> 1 as ::core::ffi::c_int)
            || ly - img_log.height
                > (CanvasHeight
                    - (if if optWindowType == 1 as DWORD {
                        BOOLEAN::TRUE.0 as ::core::ffi::c_int
                    } else {
                        BOOLEAN::FALSE.0 as ::core::ffi::c_int
                    } != 0
                    {
                        16 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    } << resolutionFactor) * 2 as ::core::ffi::c_int
                    - ((13 as ::core::ffi::c_int) << resolutionFactor)
                    << 2 as ::core::ffi::c_int)
                    - ((((63 as ::core::ffi::c_int) << resolutionFactor)
                        - ((50 as ::core::ffi::c_int) << resolutionFactor)
                        + 1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                    >> 1 as ::core::ffi::c_int
        {
            continue;
        }
        hHyperSpaceElement = AllocHyperElement(&raw mut (*SDPtr).star_pt);
        if hHyperSpaceElement.is_null() {
            continue;
        }
        HyperSpaceElementPtr = LockLink(&raw mut disp_q, hHyperSpaceElement)
            as *mut ELEMENT;
        if if resolutionFactor != 2 as ::core::ffi::c_uint {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } != 0 && optHyperStars.0 != 0
            && uqm_lua_checked!(getGameStateUint(
                b"ARILOU_SPACE_SIDE\0".as_ptr() as *const ::core::ffi::c_char,
            ), ()) <= 1 as DWORD
            && ((star_type_0 as ::core::ffi::c_int
                & ((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as BYTE
                    as ::core::ffi::c_int) >> 5 as ::core::ffi::c_int) as BYTE
                as ::core::ffi::c_int
                != C2Rust_Unnamed_8::WHITE_BODY.0 as ::core::ffi::c_int
        {
            (*HyperSpaceElementPtr).current.image.frame = SetAbsFrameIndex(
                hyperstars[1usize],
                (96 as ::core::ffi::c_int
                    + ((star_type_0 as ::core::ffi::c_int
                        & ((0xff as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                            & !(((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int)
                                as BYTE as ::core::ffi::c_int)) as BYTE
                            as ::core::ffi::c_int) >> 3 as ::core::ffi::c_int) as BYTE
                        as ::core::ffi::c_int) as COUNT,
            );
            DisplayArray[(*HyperSpaceElementPtr).PrimIndex as usize].Type = gfx_object::STAMPFILL_PRIM
                .0 as ::core::ffi::c_int as GRAPHICS_PRIM;
            DisplayArray[(*HyperSpaceElementPtr).PrimIndex as usize].color = GetMaskColor(
                ((star_type_0 as ::core::ffi::c_int
                    & ((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as BYTE
                        as ::core::ffi::c_int) >> 5 as ::core::ffi::c_int) as BYTE,
            );
            DisplayArray[(*HyperSpaceElementPtr).PrimIndex as usize].flags = ((1
                as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as BYTE;
            (*HyperSpaceElementPtr).state_flags = ((1 as ::core::ffi::c_int)
                << 3 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as ELEMENT_FLAGS;
            UnlockLink(&raw mut disp_q, hHyperSpaceElement);
            InsertQueue(&raw mut disp_q, hHyperSpaceElement, disp_q.head);
            hHyperSpaceElement = AllocHyperElement(&raw mut (*SDPtr).star_pt);
            if hHyperSpaceElement.is_null() {
                continue;
            }
            HyperSpaceElementPtr = LockLink(&raw mut disp_q, hHyperSpaceElement)
                as *mut ELEMENT;
        }
        (*HyperSpaceElementPtr).current.image.frame = star_frame;
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
            && uqm_lua_checked!(getGameStateUint(
                b"PORTAL_COUNTER\0".as_ptr() as *const ::core::ffi::c_char,
            ), ()) == 0 as DWORD
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
    uqm_lua_checked!(ProcessEncounters(&raw mut universe, ox, oy), ());
    s.origin.x = (((64 as ::core::ffi::c_int) << resolutionFactor)
        - ((8 as ::core::ffi::c_int) << resolutionFactor) >> 1 as ::core::ffi::c_int)
        as COORD;
    s.origin.y = ((53 as ::core::ffi::c_int) << resolutionFactor
        >> 1 as ::core::ffi::c_int) as COORD;
    s.frame = hyperstars[0usize];
    uqm_lua_checked!(DrawStamp(&raw mut s), ());
    uqm_lua_checked!(DrawRadarBorder(), ());
    UnbatchGraphics();
    SetContext(StatusContext);
    if TFB_Random() as UWORD as ::core::ffi::c_int & 7 as ::core::ffi::c_int == 0 {
        uqm_lua_checked!(AddAmbientElement(), ());
    }
    if universe.x as ::core::ffi::c_int
        != GlobData.Game_state.ShipStamp.origin.x as ::core::ffi::c_int
        || universe.y as ::core::ffi::c_int
            != GlobData.Game_state.ShipStamp.origin.y as ::core::ffi::c_int
    {
        GlobData.Game_state.ShipStamp.origin = universe;
        uqm_lua_checked!(DrawHyperCoords(universe), ());
    }
    frameCounter = (frameCounter as ::core::ffi::c_int + 1 as ::core::ffi::c_int
        & 0x1f as ::core::ffi::c_int) as COUNT;
}
unsafe extern "C" fn DoHyperspaceMenu(mut pMS: *mut MENU_STATE) -> BOOLEAN {
    let mut select: BOOLEAN = BOOLEAN(
        PulsedInputState
            .menu[C2Rust_Unnamed::KEY_MENU_SELECT.0 as ::core::ffi::c_int as usize]
            as ::core::ffi::c_uint,
    );
    let mut handled: BOOLEAN = BOOLEAN::FALSE;
    if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
        & (C2Rust_Unnamed_6::CHECK_ABORT.0 as ::core::ffi::c_int
            | C2Rust_Unnamed_6::CHECK_LOAD.0 as ::core::ffi::c_int) != 0
        || GlobData.SIS_state.CrewEnlisted as ::core::ffi::c_int
            == !(0 as ::core::ffi::c_int) as COUNT as ::core::ffi::c_int
    {
        return BOOLEAN::FALSE;
    }
    handled = uqm_lua_checked!(DoMenuChooser(
        pMS,
        C2Rust_Unnamed_10::PM_STARMAP.0 as ::core::ffi::c_int as BYTE,
    ), BOOLEAN(0));
    if handled.0 != 0 {
        return BOOLEAN::TRUE;
    }
    if select.0 == 0 {
        return BOOLEAN::TRUE;
    }
    uqm_lua_checked!(SetFlashRect(::core::ptr::null::<RECT>(), BOOLEAN::FALSE), BOOLEAN(0));
    match (*pMS).CurState as ::core::ffi::c_int {
        2 => {
            select = uqm_lua_checked!(DevicesMenu(), BOOLEAN(0));
            if uqm_lua_checked!(getGameStateUint(
                b"PORTAL_COUNTER\0".as_ptr() as *const ::core::ffi::c_char,
            ), BOOLEAN(0)) != 0
            {
                return BOOLEAN::FALSE;
            }
            if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
                & C2Rust_Unnamed_6::START_ENCOUNTER.0 as ::core::ffi::c_int != 0
            {
                return BOOLEAN::FALSE;
            }
        }
        3 => {
            uqm_lua_checked!(CargoMenu(), BOOLEAN(0));
        }
        4 => {
            select = uqm_lua_checked!(RosterMenu(), BOOLEAN(0));
        }
        5 => {
            if uqm_lua_checked!(GameOptions(), BOOLEAN(0)).0 == 0 {
                return BOOLEAN::FALSE;
            }
        }
        1 => {
            uqm_lua_checked!(StarMap(), BOOLEAN(0));
            return BOOLEAN::FALSE;
        }
        6 => return BOOLEAN::FALSE,
        _ => {}
    }
    if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
        & C2Rust_Unnamed_6::CHECK_ABORT.0 as ::core::ffi::c_int == 0
    {
        if select.0 != 0 {
            if optWhichMenu != 0x2 as ::core::ffi::c_int {
                (*pMS).CurState = HyperMenuItems::NAVIGATION.0 as ::core::ffi::c_int
                    as BYTE;
            }
            uqm_lua_checked!(DrawMenuStateStrings(
                C2Rust_Unnamed_10::PM_STARMAP.0 as ::core::ffi::c_int as BYTE,
                (*pMS).CurState as SWORD,
            ), BOOLEAN(0));
        }
        uqm_lua_checked!(SetFlashRect(
            ::core::ptr::with_exposed_provenance_mut::<
                RECT,
            >(!(0 as ::core::ffi::c_long) as usize),
            BOOLEAN::FALSE,
        ), BOOLEAN(0));
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
    uqm_lua_checked!(DrawMenuStateStrings(
        C2Rust_Unnamed_10::PM_STARMAP.0 as ::core::ffi::c_int as BYTE,
        HyperMenuItems::STARMAP.0 as ::core::ffi::c_int as SWORD,
    ), ());
    uqm_lua_checked!(SetFlashRect(
        ::core::ptr::with_exposed_provenance_mut::<RECT>(!(0 as ::core::ffi::c_long) as usize),
        BOOLEAN::FALSE,
    ), ());
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
    uqm_lua_checked!(DoInput(&raw mut MenuState as *mut ::core::ffi::c_void, BOOLEAN::TRUE), ());
    uqm_lua_checked!(SetFlashRect(::core::ptr::null::<RECT>(), BOOLEAN::FALSE), ());
    SetContext(SpaceContext);
    if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
        & (C2Rust_Unnamed_6::CHECK_ABORT.0 as ::core::ffi::c_int
            | C2Rust_Unnamed_6::CHECK_LOAD.0 as ::core::ffi::c_int) == 0
    {
        uqm_lua_checked!(ClearSISRect(((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as BYTE), ());
        uqm_lua_checked!(WaitForNoInput(
            (840 as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as TimePeriod,
            BOOLEAN::FALSE,
        ), ());
    }
    SetContextBackGroundColor(OldColor);
    SetContext(OldContext);
    if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
        & C2Rust_Unnamed_6::IN_BATTLE.0 as ::core::ffi::c_int == 0
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
                304 as ::core::ffi::c_int,
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
                410 as ::core::ffi::c_int,
                b"LockFleetInfo\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    return LockLink(pq, h) as *mut FLEET_INFO;
}
#[inline]
unsafe extern "C" fn LoadLastLoc() -> POINT {
    return point {
        x: uqm_lua_checked!(getGameStateUint(b"LAST_LOCATION_X\0".as_ptr() as *const ::core::ffi::c_char), point { x: 0, y: 0 })
            as COORD,
        y: uqm_lua_checked!(getGameStateUint(b"LAST_LOCATION_Y\0".as_ptr() as *const ::core::ffi::c_char), point { x: 0, y: 0 })
            as COORD,
    };
}
#[inline]
unsafe extern "C" fn ZeroLastLoc() {
    uqm_lua_checked!(setGameStateUint(
        b"LAST_LOCATION_X\0".as_ptr() as *const ::core::ffi::c_char,
        !(0 as ::core::ffi::c_int) as DWORD,
    ), ());
    uqm_lua_checked!(setGameStateUint(
        b"LAST_LOCATION_Y\0".as_ptr() as *const ::core::ffi::c_char,
        !(0 as ::core::ffi::c_int) as DWORD,
    ), ());
}
#[inline]
unsafe extern "C" fn LoadAdvancedQuasiPilot() -> POINT {
    return point {
        x: uqm_lua_checked!(getGameStateUint(
            b"ADV_AUTOPILOT_QUASI_X\0".as_ptr() as *const ::core::ffi::c_char,
        ), point { x: 0, y: 0 }) as COORD,
        y: uqm_lua_checked!(getGameStateUint(
            b"ADV_AUTOPILOT_QUASI_Y\0".as_ptr() as *const ::core::ffi::c_char,
        ), point { x: 0, y: 0 }) as COORD,
    };
}
#[inline]
unsafe extern "C" fn LoadAdvancedAutoPilot() -> POINT {
    return point {
        x: uqm_lua_checked!(getGameStateUint(
            b"ADV_AUTOPILOT_SAVE_X\0".as_ptr() as *const ::core::ffi::c_char,
        ), point { x: 0, y: 0 }) as COORD,
        y: uqm_lua_checked!(getGameStateUint(
            b"ADV_AUTOPILOT_SAVE_Y\0".as_ptr() as *const ::core::ffi::c_char,
        ), point { x: 0, y: 0 }) as COORD,
    };
}
#[inline]
unsafe extern "C" fn ZeroAdvancedQuasiPilot() {
    uqm_lua_checked!(setGameStateUint(
        b"ADV_AUTOPILOT_QUASI_X\0".as_ptr() as *const ::core::ffi::c_char,
        !(0 as ::core::ffi::c_int) as DWORD,
    ), ());
    uqm_lua_checked!(setGameStateUint(
        b"ADV_AUTOPILOT_QUASI_Y\0".as_ptr() as *const ::core::ffi::c_char,
        !(0 as ::core::ffi::c_int) as DWORD,
    ), ());
}
#[inline]
unsafe extern "C" fn ZeroAdvancedAutoPilot() {
    uqm_lua_checked!(setGameStateUint(
        b"ADV_AUTOPILOT_SAVE_X\0".as_ptr() as *const ::core::ffi::c_char,
        !(0 as ::core::ffi::c_int) as DWORD,
    ), ());
    uqm_lua_checked!(setGameStateUint(
        b"ADV_AUTOPILOT_SAVE_Y\0".as_ptr() as *const ::core::ffi::c_char,
        !(0 as ::core::ffi::c_int) as DWORD,
    ), ());
}
