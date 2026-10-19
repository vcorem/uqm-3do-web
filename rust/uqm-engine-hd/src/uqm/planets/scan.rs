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

// Rust translation of src/uqm/planets/scan.c.
extern "C" {
    pub type string_table_entry;
    pub type context_desc;
    pub type font_desc;
    pub type tfb_soundsample;
    pub type FlashContext;
    static mut Elements: *const BYTE;
    fn SetContext(Context: CONTEXT) -> CONTEXT;
    fn SetContextForeGroundColor(Color: Color) -> Color;
    fn GetContextForeGroundColor() -> Color;
    fn SetContextFGFrame(Frame: FRAME) -> FRAME;
    fn SetContextClipRect(pRect: *mut RECT) -> BOOLEAN;
    fn GetContextClipRect(pRect: *mut RECT) -> BOOLEAN;
    fn CopyContextRect(area: *const RECT) -> DRAWABLE;
    fn DrawStamp(pStamp: *mut STAMP);
    fn DrawFilledStamp(pStamp: *mut STAMP);
    fn font_DrawText(pText: *mut TEXT);
    fn font_DrawTracedText(pText: *mut TEXT, text: Color, trace: Color);
    fn BatchGraphics();
    fn UnbatchGraphics();
    fn CreateContextAux(name: *const ::core::ffi::c_char) -> CONTEXT;
    fn DestroyContext(ContextRef: CONTEXT) -> BOOLEAN;
    fn DestroyDrawable(Drawable: DRAWABLE) -> BOOLEAN;
    fn SetFrameHot(Frame: FRAME, HotSpot: HOT_SPOT) -> HOT_SPOT;
    fn GetFrameHot(Frame: FRAME) -> HOT_SPOT;
    fn LoadGraphicInstance(res: RESOURCE) -> *mut ::core::ffi::c_void;
    fn SetContextFont(Font: FONT) -> FONT;
    fn TextRect(pText: *mut TEXT, pRect: *mut RECT, pdelta: *mut BYTE) -> BOOLEAN;
    fn GetFrameIndex(Frame: FRAME) -> COUNT;
    fn SetAbsFrameIndex(Frame: FRAME, FrameIndex: COUNT) -> FRAME;
    fn SetRelFrameIndex(Frame: FRAME, FrameOffs: SIZE) -> FRAME;
    fn IncFrameIndex(Frame: FRAME) -> FRAME;
    fn CaptureDrawable(Drawable: DRAWABLE) -> FRAME;
    fn ReleaseDrawable(Frame: FRAME) -> DRAWABLE;
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
    fn snprintf(
        _: *mut ::core::ffi::c_char,
        _: size_t,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn SetAbsStringTableIndex(String: STRING, StringTableIndex: COUNT) -> STRING;
    fn GetStringAddress(String: STRING) -> STRINGPTR;
    fn utf8StringCopy(
        dst: *mut ::core::ffi::c_char,
        size: size_t,
        src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_uchar;
    fn GetTimeCounter() -> TimeCount;
    fn ReinitQueue(pq: *mut QUEUE);
    fn PutQueue(pq: *mut QUEUE, hLink: HLINK);
    fn CountLinks(pq: *mut QUEUE) -> COUNT;
    fn memset(
        _: *mut ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strcpy(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    static mut CurrentInputState: CONTROLLER_INPUT_STATE;
    static mut PulsedInputState: CONTROLLER_INPUT_STATE;
    fn FlushInput();
    fn SetInputCallback(_: Option<InputFrameCallback>) -> Option<InputFrameCallback>;
    fn DoInput(pInputState: *mut ::core::ffi::c_void, resetInput: BOOLEAN);
    static mut optWhichCoarseScan: ::core::ffi::c_int;
    static mut optWhichMenu: ::core::ffi::c_int;
    static mut resolutionFactor: ::core::ffi::c_uint;
    static mut optNebulae: OPT_ENABLABLE;
    static mut optInfiniteFuel: OPT_ENABLABLE;
    static mut optSubmenu: OPT_ENABLABLE;
    static mut optHazardColors: OPT_ENABLABLE;
    static mut optScanStyle: ::core::ffi::c_int;
    static mut optSuperPC: ::core::ffi::c_int;
    static mut optPlanetTexture: OPT_ENABLABLE;
    static mut optWindowType: DWORD;
    static mut CanvasWidth: ::core::ffi::c_int;
    static mut CanvasHeight: ::core::ffi::c_int;
    static mut disp_q: QUEUE;
    static mut DisplayArray: [PRIMITIVE; 330];
    fn AllocElement() -> HELEMENT;
    fn PlanetSide(planetLoc_0: POINT);
    fn object_animation(ElementPtr: *mut ELEMENT);
    fn GetThermalHazardRating(temp: ::core::ffi::c_int) -> ::core::ffi::c_uint;
    static mut CreatureData: [LIFEFORM_DESC; 0];
    fn ClearSISRect(ClearFlags: BYTE);
    fn SetFlashRect(pRect: *const RECT, purple: BOOLEAN);
    fn DrawStatusMessage(pStr: *const UNICODE);
    fn SetStatusMessageMode(_: StatMsgMode) -> StatMsgMode;
    fn DeltaSISGauges(
        crew_delta: SIZE,
        fuel_delta: SDWORD,
        resunit_delta: ::core::ffi::c_int,
    );
    fn getGameStateUint(name: *const ::core::ffi::c_char) -> DWORD;
    fn TFB_Random() -> DWORD;
    static mut pSolarSysState: *mut SOLARSYS_STATE;
    static mut PlanetContext: CONTEXT;
    static mut useDosSpheres: BOOLEAN;
    fn DrawPlanet(dy: ::core::ffi::c_int, tintColor: Color);
    fn DrawPCScanTint(cur_scan: COUNT);
    fn DrawStarBackGround();
    fn InitLander(LanderFlags: BYTE);
    fn DrawDefaultPlanetSphere();
    fn RerenderPlanetSphere();
    fn RotatePlanetSphere(keepRate: BOOLEAN, onTop: *mut STAMP);
    fn DrawScannedObjects(Reversed: BOOLEAN);
    fn GetNamedPlanetaryBody() -> *mut UNICODE;
    fn SaveSolarSysLocation();
    static mut SpaceJunkFrame: FRAME;
    fn CloneShipFragment(
        shipIndex: RACE_ID,
        pDstQueue: *mut QUEUE,
        crew_level: COUNT,
    ) -> HSHIPFRAG;
    fn load_life_form(selector: BYTE) -> FRAME;
    static mut EncounterGroup: BYTE;
    fn DoMenuChooser(pMS: *mut MENU_STATE, BaseState: BYTE) -> BOOLEAN;
    fn DrawMenuStateStrings(beg_index: BYTE, NewState: SWORD);
    fn DrawMineralHelpers();
    static mut GlobData: GLOBDATA;
    fn isPC(optWhich: ::core::ffi::c_int) -> OPT_CONSOLETYPE;
    fn is3DO(optWhich: ::core::ffi::c_int) -> OPT_CONSOLETYPE;
    static mut GameStrings: STRING;
    fn DrawRadarBorder();
    fn get_fuel_to_sol() -> DWORD;
    fn InitDisplayList();
    static mut Screen: FRAME;
    static mut MiscDataFrame: FRAME;
    static mut SpaceContext: CONTEXT;
    static mut MicroFont: FONT;
    static mut TinyFont: FONT;
    static mut TinyFontCond: FONT;
    fn SetMenuSounds(sound_0: MENU_SOUND_FLAGS, sound_1: MENU_SOUND_FLAGS);
    fn PlayMenuSound(S: MENU_SOUND_EFFECT);
    static mut NextActivity: ACTIVITY;
    fn DrawFromExtraScreen(r: *mut RECT);
    fn PutGroupInfo(offset: DWORD, which_group: BYTE) -> DWORD;
    fn AnyButtonPress(DetectSpecial: BOOLEAN) -> BOOLEAN;
    fn DirKeysPress() -> BOOLEAN;
}
pub type uint8 = ::core::ffi::c_uchar;
pub type sint8 = ::core::ffi::c_schar;
pub type uint16 = ::core::ffi::c_ushort;
pub type sint16 = ::core::ffi::c_short;
pub type uint32 = ::core::ffi::c_uint;
pub type sint32 = ::core::ffi::c_int;
pub type BYTE = uint8;
pub type SBYTE = sint8;
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
pub type RESOURCE = *const ::core::ffi::c_char;
pub type STRING_TABLE_ENTRY_DESC = string_table_entry;
pub type STRING = *mut STRING_TABLE_ENTRY_DESC;
pub type STRINGPTR = *mut ::core::ffi::c_char;
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
pub type FONT_DESC = font_desc;
pub type DRAWABLE_DESC = drawable_desc;
pub type CONTEXT = *mut CONTEXT_DESC;
pub type FRAME = *mut FRAME_DESC;
pub type FONT = *mut FONT_DESC;
pub type DRAWABLE = *mut DRAWABLE_DESC;
pub type TIME_VALUE = UWORD;
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
pub type TFB_SoundSample = tfb_soundsample;
pub type MUSIC_REF = *mut *mut TFB_SoundSample;
pub type TimeCount = DWORD;
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
pub type InputFrameCallback = unsafe extern "C" fn() -> ();
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_6(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_6 {
    pub const PS_HUMAN_PLAYER: Self = Self(0);
    pub const PS_NON_PLAYER: Self = Self(1);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LIFEFORM_DESC {
    pub Attributes: BYTE,
    pub ValueAndHitPoints: BYTE,
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
pub struct StatMsgMode(pub ::core::ffi::c_uint);
impl StatMsgMode {
    pub const SMM_UNDEFINED: Self = Self(0);
    pub const SMM_DATE: Self = Self(1);
    pub const SMM_RES_UNITS: Self = Self(2);
    pub const SMM_CREDITS: Self = Self(3);
    pub const SMM_WARNING: Self = Self(4);
    pub const SMM_ALERT: Self = Self(5);
    pub const SMM_DEFAULT: Self = Self(1);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_7(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_7 {
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct PlanetScanTypes(pub ::core::ffi::c_uint);
impl PlanetScanTypes {
    pub const MINERAL_SCAN: Self = Self(0);
    pub const ENERGY_SCAN: Self = Self(1);
    pub const BIOLOGICAL_SCAN: Self = Self(2);
    pub const NUM_SCAN_TYPES: Self = Self(3);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_8(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_8 {
    pub const BIOLOGICAL_DISASTER: Self = Self(0);
    pub const EARTHQUAKE_DISASTER: Self = Self(1);
    pub const LIGHTNING_DISASTER: Self = Self(2);
    pub const LAVASPOT_DISASTER: Self = Self(3);
    pub const LANDER_INJURED: Self = Self(4);
    pub const LANDER_SHOOTS: Self = Self(5);
    pub const LANDER_HITS: Self = Self(6);
    pub const LIFEFORM_CANNED: Self = Self(7);
    pub const LANDER_PICKUP: Self = Self(8);
    pub const LANDER_FULL: Self = Self(9);
    pub const LANDER_DEPARTS: Self = Self(10);
    pub const LANDER_RETURNS: Self = Self(11);
    pub const LANDER_DESTROYED: Self = Self(12);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct planet_desc {
    pub rand_seed: DWORD,
    pub data_index: BYTE,
    pub NumPlanets: BYTE,
    pub radius: SIZE,
    pub location: POINT,
    pub temp_color: Color,
    pub NextIndex: COUNT,
    pub image: STAMP,
    pub pPrevDesc: *mut PLANET_DESC,
    pub orbit: PLANET_ORBIT,
    pub size: COUNT,
    pub angle: COUNT,
    pub rotFrameIndex: ::core::ffi::c_int,
    pub rotPointIndex: ::core::ffi::c_int,
    pub rotwidth: ::core::ffi::c_int,
    pub rotheight: ::core::ffi::c_int,
    pub orb_speed: ::core::ffi::c_double,
    pub rot_speed: ::core::ffi::c_double,
    pub PlanetByte: BYTE,
    pub MoonByte: BYTE,
    pub frame_offset: BYTE,
}
pub type PLANET_ORBIT = planet_orbit;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct planet_orbit {
    pub TopoZoomFrame: FRAME,
    pub lpTopoData: *mut SBYTE,
    pub SphereFrame: FRAME,
    pub ObjectFrame: FRAME,
    pub TintFrame: FRAME,
    pub TintColor: Color,
    pub TopoColors: *mut Color,
    pub ScanColors: *mut *mut Color,
    pub ScratchArray: *mut Color,
    pub WorkFrame: FRAME,
    pub BackFrame: FRAME,
    pub light_diff: *mut *mut DWORD,
    pub map_rotate: *mut *mut MAP3D_POINT,
    pub TopoMask: FRAME,
    pub sphereBytes: *mut BYTE,
    pub sphereMap: COLORMAP,
    pub scanType: COUNT,
    pub Shade: FRAME,
    pub ShadeColors: *mut Color,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MAP3D_POINT {
    pub p: [POINT; 4],
    pub m: [DWORD; 4],
}
pub type PLANET_DESC = planet_desc;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct node_info {
    pub loc_pt: POINT,
    pub density: COUNT,
    pub r#type: COUNT,
}
pub type NODE_INFO = node_info;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct solarsys_state {
    pub InputFunc: Option<unsafe extern "C" fn(*mut solarsys_state) -> BOOLEAN>,
    pub InIpFlight: BOOLEAN,
    pub WaitIntersect: COUNT,
    pub SunDesc: [PLANET_DESC; 1],
    pub PlanetDesc: [PLANET_DESC; 16],
    pub MoonDesc: [PLANET_DESC; 4],
    pub pBaseDesc: *mut PLANET_DESC,
    pub pOrbitalDesc: *mut PLANET_DESC,
    pub FirstPlanetIndex: SIZE,
    pub LastPlanetIndex: SIZE,
    pub turn_counter: BYTE,
    pub turn_wait: BYTE,
    pub thrust_counter: BYTE,
    pub max_ship_speed: BYTE,
    pub XlatRef: STRING,
    pub XlatPtr: *const ::core::ffi::c_void,
    pub OrbitalCMap: COLORMAP,
    pub SysInfo: SYSTEM_INFO,
    pub genFuncs: *const GenerateFunctions,
    pub PlanetSideFrame: [FRAME; 6],
    pub TopoFrame: FRAME,
    pub Orbit: PLANET_ORBIT,
    pub InOrbit: BOOLEAN,
    pub ScanFrame: [FRAME; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GenerateFunctions {
    pub initNpcs: InitNpcsFunction,
    pub reinitNpcs: ReinitNpcsFunction,
    pub uninitNpcs: UninitNpcsFunction,
    pub generatePlanets: GeneratePlanetsFunction,
    pub generateMoons: GenerateMoonsFunction,
    pub generateName: GenerateNameFunction,
    pub generateOrbital: GenerateOrbitalFunction,
    pub generateMinerals: GenerateMineralsFunction,
    pub generateEnergy: GenerateEnergyFunction,
    pub generateLife: GenerateLifeFunction,
    pub pickupMinerals: PickupMineralsFunction,
    pub pickupEnergy: PickupEnergyFunction,
    pub pickupLife: PickupLifeFunction,
}
pub type PickupLifeFunction = Option<
    unsafe extern "C" fn(*mut SOLARSYS_STATE, *mut PLANET_DESC, COUNT) -> bool,
>;
pub type SOLARSYS_STATE = solarsys_state;
pub type PickupEnergyFunction = Option<
    unsafe extern "C" fn(*mut SOLARSYS_STATE, *mut PLANET_DESC, COUNT) -> bool,
>;
pub type PickupMineralsFunction = Option<
    unsafe extern "C" fn(*mut SOLARSYS_STATE, *mut PLANET_DESC, COUNT) -> bool,
>;
pub type GenerateLifeFunction = Option<
    unsafe extern "C" fn(
        *const SOLARSYS_STATE,
        *const PLANET_DESC,
        COUNT,
        *mut NODE_INFO,
    ) -> COUNT,
>;
pub type GenerateEnergyFunction = Option<
    unsafe extern "C" fn(
        *const SOLARSYS_STATE,
        *const PLANET_DESC,
        COUNT,
        *mut NODE_INFO,
    ) -> COUNT,
>;
pub type GenerateMineralsFunction = Option<
    unsafe extern "C" fn(
        *const SOLARSYS_STATE,
        *const PLANET_DESC,
        COUNT,
        *mut NODE_INFO,
    ) -> COUNT,
>;
pub type GenerateOrbitalFunction = Option<
    unsafe extern "C" fn(*mut SOLARSYS_STATE, *mut PLANET_DESC) -> bool,
>;
pub type GenerateNameFunction = Option<
    unsafe extern "C" fn(*const SOLARSYS_STATE, *const PLANET_DESC) -> bool,
>;
pub type GenerateMoonsFunction = Option<
    unsafe extern "C" fn(*mut SOLARSYS_STATE, *mut PLANET_DESC) -> bool,
>;
pub type GeneratePlanetsFunction = Option<
    unsafe extern "C" fn(*mut SOLARSYS_STATE) -> bool,
>;
pub type UninitNpcsFunction = Option<unsafe extern "C" fn(*mut SOLARSYS_STATE) -> bool>;
pub type ReinitNpcsFunction = Option<unsafe extern "C" fn(*mut SOLARSYS_STATE) -> bool>;
pub type InitNpcsFunction = Option<unsafe extern "C" fn(*mut SOLARSYS_STATE) -> bool>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SYSTEM_INFO {
    pub StarSize: BYTE,
    pub StarIntensity: BYTE,
    pub StarEnergy: UWORD,
    pub PlanetInfo: PLANET_INFO,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PLANET_INFO {
    pub AxialTilt: SIZE,
    pub Tectonics: UWORD,
    pub Weather: UWORD,
    pub PlanetDensity: UWORD,
    pub PlanetRadius: UWORD,
    pub SurfaceGravity: UWORD,
    pub SurfaceTemperature: SIZE,
    pub RotationPeriod: UWORD,
    pub AtmoDensity: UWORD,
    pub LifeChance: SIZE,
    pub PlanetToSunDist: DWORD,
    pub PlanDataPtr: *const PlanetFrame,
    pub ScanSeed: [DWORD; 3],
    pub ScanRetrieveMask: [DWORD; 3],
    pub PartiallyScavengedList: [[BYTE; 32]; 3],
    pub DiscoveryString: STRING,
    pub LanderFont: FONT,
    pub LanderFontEff: FRAME,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PlanetFrame {
    pub Type: BYTE,
    pub BaseTectonics: BYTE,
    pub AtmoAndDensity: BYTE,
    pub UsefulElements: [ELEMENT_ENTRY; 8],
    pub CMapInstance: RESOURCE,
    pub XlatTabInstance: RESOURCE,
    pub num_faults: COUNT,
    pub fault_depth: SIZE,
    pub num_blemishes: COUNT,
    pub base_elevation: SIZE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ELEMENT_ENTRY {
    pub ElementType: BYTE,
    pub Density: BYTE,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_9(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_9 {
    pub const SMALL_ROCKY_WORLD: Self = Self(0);
    pub const LARGE_ROCKY_WORLD: Self = Self(1);
    pub const GAS_GIANT: Self = Self(2);
    pub const NUM_PLANET_TYPES: Self = Self(3);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_10(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_10 {
    pub const FIRST_ROCKY_WORLD: Self = Self(0);
    pub const FIRST_SMALL_ROCKY_WORLD: Self = Self(0);
    pub const OOLITE_WORLD: Self = Self(0);
    pub const YTTRIC_WORLD: Self = Self(1);
    pub const QUASI_DEGENERATE_WORLD: Self = Self(2);
    pub const LANTHANIDE_WORLD: Self = Self(3);
    pub const TREASURE_WORLD: Self = Self(4);
    pub const UREA_WORLD: Self = Self(5);
    pub const METAL_WORLD: Self = Self(6);
    pub const RADIOACTIVE_WORLD: Self = Self(7);
    pub const OPALESCENT_WORLD: Self = Self(8);
    pub const CYANIC_WORLD: Self = Self(9);
    pub const ACID_WORLD: Self = Self(10);
    pub const ALKALI_WORLD: Self = Self(11);
    pub const HALIDE_WORLD: Self = Self(12);
    pub const GREEN_WORLD: Self = Self(13);
    pub const COPPER_WORLD: Self = Self(14);
    pub const CARBIDE_WORLD: Self = Self(15);
    pub const ULTRAMARINE_WORLD: Self = Self(16);
    pub const NOBLE_WORLD: Self = Self(17);
    pub const AZURE_WORLD: Self = Self(18);
    pub const CHONDRITE_WORLD: Self = Self(19);
    pub const PURPLE_WORLD: Self = Self(20);
    pub const SUPER_DENSE_WORLD: Self = Self(21);
    pub const PELLUCID_WORLD: Self = Self(22);
    pub const DUST_WORLD: Self = Self(23);
    pub const CRIMSON_WORLD: Self = Self(24);
    pub const CIMMERIAN_WORLD: Self = Self(25);
    pub const INFRARED_WORLD: Self = Self(26);
    pub const SELENIC_WORLD: Self = Self(27);
    pub const AURIC_WORLD: Self = Self(28);
    pub const LAST_SMALL_ROCKY_WORLD: Self = Self(28);
    pub const FIRST_LARGE_ROCKY_WORLD: Self = Self(29);
    pub const FLUORESCENT_WORLD: Self = Self(29);
    pub const ULTRAVIOLET_WORLD: Self = Self(30);
    pub const PLUTONIC_WORLD: Self = Self(31);
    pub const RAINBOW_WORLD: Self = Self(32);
    pub const SHATTERED_WORLD: Self = Self(33);
    pub const SAPPHIRE_WORLD: Self = Self(34);
    pub const ORGANIC_WORLD: Self = Self(35);
    pub const XENOLITHIC_WORLD: Self = Self(36);
    pub const REDUX_WORLD: Self = Self(37);
    pub const PRIMORDIAL_WORLD: Self = Self(38);
    pub const EMERALD_WORLD: Self = Self(39);
    pub const CHLORINE_WORLD: Self = Self(40);
    pub const MAGNETIC_WORLD: Self = Self(41);
    pub const WATER_WORLD: Self = Self(42);
    pub const TELLURIC_WORLD: Self = Self(43);
    pub const HYDROCARBON_WORLD: Self = Self(44);
    pub const IODINE_WORLD: Self = Self(45);
    pub const VINYLOGOUS_WORLD: Self = Self(46);
    pub const RUBY_WORLD: Self = Self(47);
    pub const MAGMA_WORLD: Self = Self(48);
    pub const MAROON_WORLD: Self = Self(49);
    pub const LAST_LARGE_ROCKY_WORLD: Self = Self(49);
    pub const LAST_ROCKY_WORLD: Self = Self(49);
    pub const FIRST_GAS_GIANT: Self = Self(50);
    pub const BLU_GAS_GIANT: Self = Self(50);
    pub const CYA_GAS_GIANT: Self = Self(51);
    pub const GRN_GAS_GIANT: Self = Self(52);
    pub const GRY_GAS_GIANT: Self = Self(53);
    pub const ORA_GAS_GIANT: Self = Self(54);
    pub const PUR_GAS_GIANT: Self = Self(55);
    pub const RED_GAS_GIANT: Self = Self(56);
    pub const VIO_GAS_GIANT: Self = Self(57);
    pub const YEL_GAS_GIANT: Self = Self(58);
    pub const LAST_GAS_GIANT: Self = Self(58);
    pub const NUMBER_OF_PLANET_TYPES: Self = Self(59);
    pub const WORLD_TYPE_SPECIAL: Self = Self(128);
    pub const PLANET_SHIELDED: Self = Self(128);
    pub const HIERARCHY_STARBASE: Self = Self(255);
    pub const SA_MATRA: Self = Self(254);
    pub const DESTROYED_STARBASE: Self = Self(253);
    pub const PRECURSOR_STARBASE: Self = Self(252);
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PICK_PLANET_STATE {
    pub success: bool,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_11(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_11 {
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
pub struct C2Rust_Unnamed_12(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_12 {
    pub const STAR_STRING_BASE: Self = Self(0);
    pub const DEVICE_STRING_BASE: Self = Self(149);
    pub const CARGO_STRING_BASE: Self = Self(178);
    pub const ELEMENTS_STRING_BASE: Self = Self(192);
    pub const SCAN_STRING_BASE: Self = Self(325);
    pub const STAR_NUMBER_BASE: Self = Self(391);
    pub const PLANET_NUMBER_BASE: Self = Self(405);
    pub const MONTHS_STRING_BASE: Self = Self(448);
    pub const FEEDBACK_STRING_BASE: Self = Self(460);
    pub const STARBASE_STRING_BASE: Self = Self(468);
    pub const ENCOUNTER_STRING_BASE: Self = Self(510);
    pub const NAVIGATION_STRING_BASE: Self = Self(518);
    pub const NAMING_STRING_BASE: Self = Self(527);
    pub const MELEE_STRING_BASE: Self = Self(535);
    pub const SAVEGAME_STRING_BASE: Self = Self(560);
    pub const OPTION_STRING_BASE: Self = Self(570);
    pub const QUITMENU_STRING_BASE: Self = Self(580);
    pub const STATUS_STRING_BASE: Self = Self(585);
    pub const FLAGSHIP_STRING_BASE: Self = Self(607);
    pub const ORBITSCAN_STRING_BASE: Self = Self(620);
    pub const MAINMENU_STRING_BASE: Self = Self(639);
    pub const NETMELEE_STRING_BASE: Self = Self(713);
    pub const BIOLOGICAL_STRING_BASE: Self = Self(747);
    pub const PLAYMENU_STRING_BASE: Self = Self(773);
    pub const LABEL_STRING_BASE: Self = Self(803);
    pub const GAMESTR_COUNT: Self = Self(829);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ScanMenuItems(pub ::core::ffi::c_uint);
impl ScanMenuItems {
    pub const EXIT_SCAN: Self = Self(3);
    pub const AUTO_SCAN: Self = Self(4);
    pub const DISPATCH_SHUTTLE: Self = Self(5);
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
static mut ScanContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
static mut planetLoc: POINT = POINT { x: 0, y: 0 };
static mut cursorRect: RECT = RECT {
    corner: POINT { x: 0, y: 0 },
    extent: EXTENT { width: 0, height: 0 },
};
static mut eraseFrame: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
#[no_mangle]
pub unsafe extern "C" fn RepairBackRect(mut pRect: *mut RECT) {
    let mut new_r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut old_r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    GetContextClipRect(&raw mut old_r);
    new_r.corner.x = ((*pRect).corner.x as ::core::ffi::c_int
        + old_r.corner.x as ::core::ffi::c_int) as COORD;
    new_r.corner.y = ((*pRect).corner.y as ::core::ffi::c_int
        + old_r.corner.y as ::core::ffi::c_int) as COORD;
    new_r.extent = (*pRect).extent;
    new_r.extent.height = (new_r.extent.height as ::core::ffi::c_int
        + (new_r.corner.y as ::core::ffi::c_int & 1 as ::core::ffi::c_int)) as COORD;
    new_r.corner.y = (new_r.corner.y as ::core::ffi::c_int & !(1 as ::core::ffi::c_int))
        as COORD;
    DrawFromExtraScreen(&raw mut new_r);
}
unsafe extern "C" fn EraseCoarseScan() {
    SetContext(PlanetContext);
    BatchGraphics();
    uqm_lua_checked!(DrawStarBackGround(), ());
    uqm_lua_checked!(DrawDefaultPlanetSphere(), ());
    UnbatchGraphics();
}
unsafe extern "C" fn PrintScanTitleText(mut t: *mut TEXT) {
    SetContextForeGroundColor(
        if optWhichCoarseScan != 0 {
            colorFromRgb15(0 as BYTE, 0x14 as BYTE, 0x14 as BYTE)
        } else {
            colorFromRgb15(0 as BYTE, 0 as BYTE, 0x15 as BYTE)
        },
    );
    if optNebulae.0 == 0 {
        uqm_lua_checked!(font_DrawText(t), ());
    } else {
        uqm_lua_checked!(font_DrawTracedText(
            t,
            if optWhichCoarseScan != 0 {
                colorFromRgb15(0 as BYTE, 0x14 as BYTE, 0x14 as BYTE)
            } else {
                GetContextForeGroundColor()
            },
            buildColorRgba(0 as BYTE, 0 as BYTE, 0 as BYTE, 0x66 as BYTE),
        ), ());
    }
    SetContextForeGroundColor(colorFromRgb15(0xf as BYTE, 0 as BYTE, 0x19 as BYTE));
}
unsafe extern "C" fn PrintScanTitlePC(
    mut t: *mut TEXT,
    mut buf: *mut UNICODE,
    mut xpos: COORD,
) {
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    (*t).baseline.x = xpos;
    (*t).pStr = buf;
    (*t).CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    uqm_lua_checked!(PrintScanTitleText(t), ());
    TextRect(t, &raw mut r, ::core::ptr::null_mut::<BYTE>());
    (*t).baseline.x = ((*t).baseline.x as ::core::ffi::c_int
        + r.extent.width as ::core::ffi::c_int) as COORD;
}
unsafe extern "C" fn PrintScanText(mut t: *mut TEXT) {
    if optWhichCoarseScan & 1 as ::core::ffi::c_int == 0 {
        (*t).CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    }
    if optNebulae.0 == 0 {
        uqm_lua_checked!(font_DrawText(t), ());
    } else {
        uqm_lua_checked!(font_DrawTracedText(
            t,
            GetContextForeGroundColor(),
            buildColorRgba(0 as BYTE, 0 as BYTE, 0 as BYTE, 0x66 as BYTE),
        ), ());
    };
}
unsafe extern "C" fn MakeScanValue(
    mut buf: *mut UNICODE,
    mut val: ::core::ffi::c_long,
    mut extra: *const UNICODE,
) {
    if val
        >= (10 as ::core::ffi::c_int * 100 as ::core::ffi::c_int) as ::core::ffi::c_long
    {
        sprintf(
            buf,
            b"%ld.%ld%s\0".as_ptr() as *const ::core::ffi::c_char,
            val / 100 as ::core::ffi::c_long,
            val / 10 as ::core::ffi::c_long % 10 as ::core::ffi::c_long,
            extra,
        );
    } else {
        sprintf(
            buf,
            b"%ld.%02ld%s\0".as_ptr() as *const ::core::ffi::c_char,
            val / 100 as ::core::ffi::c_long,
            val % 100 as ::core::ffi::c_long,
            extra,
        );
    };
}
unsafe extern "C" fn MakeDayValue(
    mut buf: *mut UNICODE,
    mut val: ::core::ffi::c_long,
    mut extra: *const UNICODE,
) {
    if ((*pSolarSysState).SysInfo.PlanetInfo.RotationPeriod as ::core::ffi::c_int)
        < 240 as ::core::ffi::c_int * 10 as ::core::ffi::c_int
    {
        sprintf(
            buf,
            b"%ld.%02ld%s\0".as_ptr() as *const ::core::ffi::c_char,
            val / 100 as ::core::ffi::c_long,
            val % 100 as ::core::ffi::c_long,
            extra,
        );
    } else {
        sprintf(
            buf,
            b"%ld.%ld%s\0".as_ptr() as *const ::core::ffi::c_char,
            val / 10 as ::core::ffi::c_long,
            val % 10 as ::core::ffi::c_long,
            extra,
        );
    };
}
unsafe extern "C" fn GetRotationalPeriod() -> SIZE {
    if ((*pSolarSysState).SysInfo.PlanetInfo.RotationPeriod as ::core::ffi::c_int)
        < 240 as ::core::ffi::c_int * 10 as ::core::ffi::c_int
    {
        return ((*pSolarSysState).SysInfo.PlanetInfo.RotationPeriod as ::core::ffi::c_int
            * 10 as ::core::ffi::c_int / 24 as ::core::ffi::c_int) as SIZE
    } else {
        return (((*pSolarSysState).SysInfo.PlanetInfo.RotationPeriod
            as ::core::ffi::c_int
            + (24 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int))
            / 24 as ::core::ffi::c_int) as SIZE
    };
}
#[no_mangle]
pub unsafe extern "C" fn GetPlanetTitle(mut buf: *mut UNICODE, mut bufsize: COUNT) {
    let mut val: ::core::ffi::c_int = 0;
    let mut named: *mut UNICODE = uqm_lua_checked!(GetNamedPlanetaryBody(), ());
    if !named.is_null() {
        utf8StringCopy(buf, bufsize as size_t, named);
        return;
    }
    val = (*(*pSolarSysState).pOrbitalDesc).data_index as ::core::ffi::c_int
        & !(C2Rust_Unnamed_10::PLANET_SHIELDED.0 as ::core::ffi::c_int);
    if val >= C2Rust_Unnamed_10::FIRST_GAS_GIANT.0 as ::core::ffi::c_int {
        if if GlobData.SIS_state.Extended as ::core::ffi::c_int != 0 {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } == 0
        {
            sprintf(
                buf,
                b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                GetStringAddress(
                    SetAbsStringTableIndex(
                        GameStrings,
                        (C2Rust_Unnamed_12::SCAN_STRING_BASE.0 as ::core::ffi::c_int
                            + 4 as ::core::ffi::c_int + 51 as ::core::ffi::c_int)
                            as COUNT,
                    ),
                ),
            );
        } else if val
            <= C2Rust_Unnamed_10::NUMBER_OF_PLANET_TYPES.0 as ::core::ffi::c_int
        {
            sprintf(
                buf,
                b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                GetStringAddress(
                    SetAbsStringTableIndex(
                        GameStrings,
                        (C2Rust_Unnamed_12::SCAN_STRING_BASE.0 as ::core::ffi::c_int
                            + 4 as ::core::ffi::c_int + 2 as ::core::ffi::c_int + val)
                            as COUNT,
                    ),
                ),
            );
        }
    } else {
        sprintf(
            buf,
            b"%s %s\0".as_ptr() as *const ::core::ffi::c_char,
            GetStringAddress(
                SetAbsStringTableIndex(
                    GameStrings,
                    (C2Rust_Unnamed_12::SCAN_STRING_BASE.0 as ::core::ffi::c_int
                        + 4 as ::core::ffi::c_int + val) as COUNT,
                ),
            ),
            GetStringAddress(
                SetAbsStringTableIndex(
                    GameStrings,
                    (C2Rust_Unnamed_12::SCAN_STRING_BASE.0 as ::core::ffi::c_int
                        + 4 as ::core::ffi::c_int + 50 as ::core::ffi::c_int) as COUNT,
                ),
            ),
        );
    };
}
unsafe extern "C" fn HazardCase(mut hazard: BYTE) {
    let mut HazardColor: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    let mut Temperature: UWORD = GetThermalHazardRating(
        (*pSolarSysState).SysInfo.PlanetInfo.SurfaceTemperature as ::core::ffi::c_int,
    ) as UWORD;
    let mut Weather: UWORD = ((*pSolarSysState).SysInfo.PlanetInfo.Weather
        as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as UWORD;
    let mut Tectonics: UWORD = ((*pSolarSysState).SysInfo.PlanetInfo.Tectonics
        as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as UWORD;
    match hazard as ::core::ffi::c_int {
        3 => {
            HazardColor = if Temperature as ::core::ffi::c_int > 1 as ::core::ffi::c_int
                && (Temperature as ::core::ffi::c_int) < 5 as ::core::ffi::c_int
            {
                colorFromRgb15(0xf as BYTE, 0xf as BYTE, 0 as BYTE)
            } else if Temperature as ::core::ffi::c_int > 4 as ::core::ffi::c_int {
                colorFromRgb15(0x1f as BYTE, 0 as BYTE, 0 as BYTE)
            } else {
                colorFromRgb15(0xf as BYTE, 0 as BYTE, 0x19 as BYTE)
            };
        }
        2 => {
            HazardColor = if Weather as ::core::ffi::c_int > 2 as ::core::ffi::c_int
                && (Weather as ::core::ffi::c_int) < 5 as ::core::ffi::c_int
            {
                colorFromRgb15(0xf as BYTE, 0xf as BYTE, 0 as BYTE)
            } else if Weather as ::core::ffi::c_int > 4 as ::core::ffi::c_int {
                colorFromRgb15(0x1f as BYTE, 0 as BYTE, 0 as BYTE)
            } else {
                colorFromRgb15(0xf as BYTE, 0 as BYTE, 0x19 as BYTE)
            };
        }
        1 => {
            HazardColor = if Tectonics as ::core::ffi::c_int > 2 as ::core::ffi::c_int
                && (Tectonics as ::core::ffi::c_int) < 6 as ::core::ffi::c_int
            {
                colorFromRgb15(0xf as BYTE, 0xf as BYTE, 0 as BYTE)
            } else if Tectonics as ::core::ffi::c_int > 5 as ::core::ffi::c_int {
                colorFromRgb15(0x1f as BYTE, 0 as BYTE, 0 as BYTE)
            } else {
                colorFromRgb15(0xf as BYTE, 0 as BYTE, 0x19 as BYTE)
            };
        }
        _ => {
            HazardColor = colorFromRgb15(0xf as BYTE, 0 as BYTE, 0x19 as BYTE);
        }
    }
    SetContextForeGroundColor(HazardColor);
}
unsafe extern "C" fn PrintCoarseScanPC() {
    let mut val: SDWORD = 0;
    let mut t: TEXT = TEXT {
        baseline: POINT { x: 0, y: 0 },
        pStr: ::core::ptr::null::<UNICODE>(),
        align: TEXT_ALIGN::ALIGN_LEFT,
        CharCount: 0,
    };
    let mut buf: [UNICODE; 200] = [0; 200];
    uqm_lua_checked!(GetPlanetTitle(
        &raw mut buf as *mut UNICODE,
        ::core::mem::size_of::<[UNICODE; 200]>() as COUNT,
    ), ());
    SetContext(PlanetContext);
    t.align = TEXT_ALIGN::ALIGN_CENTER;
    t.baseline.x = ((CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
            } << resolutionFactor)) >> resolutionFactor >> 1 as ::core::ffi::c_int)
        << resolutionFactor) as COORD;
    t.baseline.y = ((13 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    t.pStr = &raw mut buf as *mut UNICODE;
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    SetContextForeGroundColor(
        if optWhichCoarseScan != 0 {
            colorFromRgb15(0xf as BYTE, 0 as BYTE, 0x19 as BYTE)
        } else {
            colorFromRgb15(0 as BYTE, 0 as BYTE, 0x15 as BYTE)
        },
    );
    SetContextFont(MicroFont);
    BatchGraphics();
    uqm_lua_checked!(PrintScanText(&raw mut t), ());
    SetContextFont(
        if if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            TinyFontCond
        } else {
            TinyFont
        },
    );
    t.baseline.y = ((((CanvasHeight
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
        - ((13 as ::core::ffi::c_int) << resolutionFactor) >> resolutionFactor)
        - 67 as ::core::ffi::c_int
        - ((5 as ::core::ffi::c_int) << resolutionFactor >> resolutionFactor)
        >> 1 as ::core::ffi::c_int) << resolutionFactor)
        - (if useDosSpheres.0 != 0 {
            2 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        } << resolutionFactor) - ((13 as ::core::ffi::c_int) << resolutionFactor))
        as COORD;
    t.align = TEXT_ALIGN::ALIGN_LEFT;
    utf8StringCopy(
        &raw mut buf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[UNICODE; 200]>(),
        GetStringAddress(
            SetAbsStringTableIndex(
                GameStrings,
                (C2Rust_Unnamed_12::ORBITSCAN_STRING_BASE.0 as ::core::ffi::c_int
                    + 0 as ::core::ffi::c_int) as COUNT,
            ),
        ),
    );
    uqm_lua_checked!(PrintScanTitlePC(
        &raw mut t,
        &raw mut buf as *mut UNICODE,
        ((2 as ::core::ffi::c_int) << resolutionFactor) as COORD,
    ), ());
    val = ((*pSolarSysState).SysInfo.PlanetInfo.PlanetToSunDist as ::core::ffi::c_ulong)
        .wrapping_mul(100 as ::core::ffi::c_ulong)
        .wrapping_add(
            ((8 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int) as ::core::ffi::c_ulong,
        )
        .wrapping_div(
            ((8 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                as ::core::ffi::c_ulong,
        ) as SDWORD;
    MakeScanValue(
        &raw mut buf as *mut UNICODE,
        val as ::core::ffi::c_long,
        GetStringAddress(
            SetAbsStringTableIndex(
                GameStrings,
                (C2Rust_Unnamed_12::ORBITSCAN_STRING_BASE.0 as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int) as COUNT,
            ),
        ),
    );
    uqm_lua_checked!(PrintScanText(&raw mut t), ());
    t.baseline.y = (t.baseline.y as ::core::ffi::c_int
        + ((10 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    utf8StringCopy(
        &raw mut buf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[UNICODE; 200]>(),
        GetStringAddress(
            SetAbsStringTableIndex(
                GameStrings,
                (C2Rust_Unnamed_12::ORBITSCAN_STRING_BASE.0 as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int) as COUNT,
            ),
        ),
    );
    uqm_lua_checked!(PrintScanTitlePC(
        &raw mut t,
        &raw mut buf as *mut UNICODE,
        ((2 as ::core::ffi::c_int) << resolutionFactor) as COORD,
    ), ());
    if (*pSolarSysState).SysInfo.PlanetInfo.AtmoDensity as ::core::ffi::c_int
        == 0xffff as ::core::ffi::c_int
    {
        utf8StringCopy(
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[UNICODE; 200]>(),
            GetStringAddress(
                SetAbsStringTableIndex(
                    GameStrings,
                    (C2Rust_Unnamed_12::ORBITSCAN_STRING_BASE.0 as ::core::ffi::c_int
                        + 3 as ::core::ffi::c_int) as COUNT,
                ),
            ),
        );
    } else if (*pSolarSysState).SysInfo.PlanetInfo.AtmoDensity as ::core::ffi::c_int
        == 0 as ::core::ffi::c_int
    {
        utf8StringCopy(
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[UNICODE; 200]>(),
            GetStringAddress(
                SetAbsStringTableIndex(
                    GameStrings,
                    (C2Rust_Unnamed_12::ORBITSCAN_STRING_BASE.0 as ::core::ffi::c_int
                        + 4 as ::core::ffi::c_int) as COUNT,
                ),
            ),
        );
    } else {
        val = (((*pSolarSysState).SysInfo.PlanetInfo.AtmoDensity as ::core::ffi::c_int
            * 100 as ::core::ffi::c_int
            + (50 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int))
            / 50 as ::core::ffi::c_int) as SDWORD;
        MakeScanValue(
            &raw mut buf as *mut UNICODE,
            val as ::core::ffi::c_long,
            GetStringAddress(
                SetAbsStringTableIndex(
                    GameStrings,
                    (C2Rust_Unnamed_12::ORBITSCAN_STRING_BASE.0 as ::core::ffi::c_int
                        + 5 as ::core::ffi::c_int) as COUNT,
                ),
            ),
        );
    }
    uqm_lua_checked!(PrintScanText(&raw mut t), ());
    t.baseline.y = (t.baseline.y as ::core::ffi::c_int
        + ((10 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    utf8StringCopy(
        &raw mut buf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[UNICODE; 200]>(),
        GetStringAddress(
            SetAbsStringTableIndex(
                GameStrings,
                (C2Rust_Unnamed_12::ORBITSCAN_STRING_BASE.0 as ::core::ffi::c_int
                    + 6 as ::core::ffi::c_int) as COUNT,
            ),
        ),
    );
    uqm_lua_checked!(PrintScanTitlePC(
        &raw mut t,
        &raw mut buf as *mut UNICODE,
        ((2 as ::core::ffi::c_int) << resolutionFactor) as COORD,
    ), ());
    snprintf(
        &raw mut buf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[UNICODE; 200]>(),
        b"%d%s c\0".as_ptr() as *const ::core::ffi::c_char,
        (*pSolarSysState).SysInfo.PlanetInfo.SurfaceTemperature as ::core::ffi::c_int,
        b"\xC2\xB0\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if optHazardColors.0 != 0 {
        HazardCase(C2Rust_Unnamed_8::LAVASPOT_DISASTER.0 as ::core::ffi::c_int as BYTE);
    }
    uqm_lua_checked!(PrintScanText(&raw mut t), ());
    t.baseline.y = (t.baseline.y as ::core::ffi::c_int
        + ((10 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    utf8StringCopy(
        &raw mut buf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[UNICODE; 200]>(),
        GetStringAddress(
            SetAbsStringTableIndex(
                GameStrings,
                (C2Rust_Unnamed_12::ORBITSCAN_STRING_BASE.0 as ::core::ffi::c_int
                    + 7 as ::core::ffi::c_int) as COUNT,
            ),
        ),
    );
    uqm_lua_checked!(PrintScanTitlePC(
        &raw mut t,
        &raw mut buf as *mut UNICODE,
        ((2 as ::core::ffi::c_int) << resolutionFactor) as COORD,
    ), ());
    if (*pSolarSysState).SysInfo.PlanetInfo.AtmoDensity as ::core::ffi::c_int
        == 0 as ::core::ffi::c_int
    {
        utf8StringCopy(
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[UNICODE; 200]>(),
            GetStringAddress(
                SetAbsStringTableIndex(
                    GameStrings,
                    (C2Rust_Unnamed_12::ORBITSCAN_STRING_BASE.0 as ::core::ffi::c_int
                        + 8 as ::core::ffi::c_int) as COUNT,
                ),
            ),
        );
    } else {
        snprintf(
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[UNICODE; 200]>(),
            b"%s %u\0".as_ptr() as *const ::core::ffi::c_char,
            GetStringAddress(
                SetAbsStringTableIndex(
                    GameStrings,
                    (C2Rust_Unnamed_12::ORBITSCAN_STRING_BASE.0 as ::core::ffi::c_int
                        + 9 as ::core::ffi::c_int) as COUNT,
                ),
            ),
            (*pSolarSysState).SysInfo.PlanetInfo.Weather as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int,
        );
    }
    if optHazardColors.0 != 0 {
        HazardCase(C2Rust_Unnamed_8::LIGHTNING_DISASTER.0 as ::core::ffi::c_int as BYTE);
    }
    uqm_lua_checked!(PrintScanText(&raw mut t), ());
    t.baseline.y = (t.baseline.y as ::core::ffi::c_int
        + ((10 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    utf8StringCopy(
        &raw mut buf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[UNICODE; 200]>(),
        GetStringAddress(
            SetAbsStringTableIndex(
                GameStrings,
                (C2Rust_Unnamed_12::ORBITSCAN_STRING_BASE.0 as ::core::ffi::c_int
                    + 10 as ::core::ffi::c_int) as COUNT,
            ),
        ),
    );
    uqm_lua_checked!(PrintScanTitlePC(
        &raw mut t,
        &raw mut buf as *mut UNICODE,
        ((2 as ::core::ffi::c_int) << resolutionFactor) as COORD,
    ), ());
    if ((*(*pSolarSysState).SysInfo.PlanetInfo.PlanDataPtr).Type as ::core::ffi::c_int
        & 0x3 as ::core::ffi::c_int) as BYTE as ::core::ffi::c_int
        == C2Rust_Unnamed_9::GAS_GIANT.0 as ::core::ffi::c_int
    {
        utf8StringCopy(
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[UNICODE; 200]>(),
            GetStringAddress(
                SetAbsStringTableIndex(
                    GameStrings,
                    (C2Rust_Unnamed_12::ORBITSCAN_STRING_BASE.0 as ::core::ffi::c_int
                        + 8 as ::core::ffi::c_int) as COUNT,
                ),
            ),
        );
    } else {
        snprintf(
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[UNICODE; 200]>(),
            b"%s %u\0".as_ptr() as *const ::core::ffi::c_char,
            GetStringAddress(
                SetAbsStringTableIndex(
                    GameStrings,
                    (C2Rust_Unnamed_12::ORBITSCAN_STRING_BASE.0 as ::core::ffi::c_int
                        + 9 as ::core::ffi::c_int) as COUNT,
                ),
            ),
            (*pSolarSysState).SysInfo.PlanetInfo.Tectonics as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int,
        );
    }
    if optHazardColors.0 != 0 {
        HazardCase(
            C2Rust_Unnamed_8::EARTHQUAKE_DISASTER.0 as ::core::ffi::c_int as BYTE,
        );
    }
    uqm_lua_checked!(PrintScanText(&raw mut t), ());
    t.baseline.y = ((((CanvasHeight
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
        - ((13 as ::core::ffi::c_int) << resolutionFactor) >> resolutionFactor)
        - 67 as ::core::ffi::c_int
        - ((5 as ::core::ffi::c_int) << resolutionFactor >> resolutionFactor)
        >> 1 as ::core::ffi::c_int) << resolutionFactor)
        - (if useDosSpheres.0 != 0 {
            2 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        } << resolutionFactor) - ((13 as ::core::ffi::c_int) << resolutionFactor))
        as COORD;
    utf8StringCopy(
        &raw mut buf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[UNICODE; 200]>(),
        GetStringAddress(
            SetAbsStringTableIndex(
                GameStrings,
                (C2Rust_Unnamed_12::ORBITSCAN_STRING_BASE.0 as ::core::ffi::c_int
                    + 11 as ::core::ffi::c_int) as COUNT,
            ),
        ),
    );
    uqm_lua_checked!(PrintScanTitlePC(
        &raw mut t,
        &raw mut buf as *mut UNICODE,
        (CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
                } << resolutionFactor))
            - ((73 as ::core::ffi::c_int) << resolutionFactor)
            + (if if optWindowType == 1 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                16 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } << resolutionFactor)) as COORD,
    ), ());
    val = (*pSolarSysState).SysInfo.PlanetInfo.PlanetRadius as SDWORD;
    val = ((val as DWORD).wrapping_mul(val as DWORD).wrapping_mul(val as DWORD)
        as ::core::ffi::c_ulong)
        .wrapping_div(100 as ::core::ffi::c_ulong)
        .wrapping_mul(
            (*pSolarSysState).SysInfo.PlanetInfo.PlanetDensity as ::core::ffi::c_ulong,
        )
        .wrapping_add(
            (100 as ::core::ffi::c_long * 100 as ::core::ffi::c_long
                >> 1 as ::core::ffi::c_int) as ::core::ffi::c_ulong,
        )
        .wrapping_div(
            (100 as ::core::ffi::c_long * 100 as ::core::ffi::c_long)
                as ::core::ffi::c_ulong,
        ) as SDWORD;
    if val == 0 as ::core::ffi::c_int {
        val = 1 as ::core::ffi::c_int as SDWORD;
    }
    MakeScanValue(
        &raw mut buf as *mut UNICODE,
        val as ::core::ffi::c_long,
        GetStringAddress(
            SetAbsStringTableIndex(
                GameStrings,
                (C2Rust_Unnamed_12::ORBITSCAN_STRING_BASE.0 as ::core::ffi::c_int
                    + 12 as ::core::ffi::c_int) as COUNT,
            ),
        ),
    );
    uqm_lua_checked!(PrintScanText(&raw mut t), ());
    t.baseline.y = (t.baseline.y as ::core::ffi::c_int
        + ((10 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    utf8StringCopy(
        &raw mut buf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[UNICODE; 200]>(),
        GetStringAddress(
            SetAbsStringTableIndex(
                GameStrings,
                (C2Rust_Unnamed_12::ORBITSCAN_STRING_BASE.0 as ::core::ffi::c_int
                    + 13 as ::core::ffi::c_int) as COUNT,
            ),
        ),
    );
    uqm_lua_checked!(PrintScanTitlePC(
        &raw mut t,
        &raw mut buf as *mut UNICODE,
        (CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
                } << resolutionFactor))
            - ((73 as ::core::ffi::c_int) << resolutionFactor)
            + (if if optWindowType == 1 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                16 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } << resolutionFactor)) as COORD,
    ), ());
    val = (*pSolarSysState).SysInfo.PlanetInfo.PlanetRadius as SDWORD;
    MakeScanValue(
        &raw mut buf as *mut UNICODE,
        val as ::core::ffi::c_long,
        GetStringAddress(
            SetAbsStringTableIndex(
                GameStrings,
                (C2Rust_Unnamed_12::ORBITSCAN_STRING_BASE.0 as ::core::ffi::c_int
                    + 12 as ::core::ffi::c_int) as COUNT,
            ),
        ),
    );
    uqm_lua_checked!(PrintScanText(&raw mut t), ());
    t.baseline.y = (t.baseline.y as ::core::ffi::c_int
        + ((10 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    utf8StringCopy(
        &raw mut buf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[UNICODE; 200]>(),
        GetStringAddress(
            SetAbsStringTableIndex(
                GameStrings,
                (C2Rust_Unnamed_12::ORBITSCAN_STRING_BASE.0 as ::core::ffi::c_int
                    + 14 as ::core::ffi::c_int) as COUNT,
            ),
        ),
    );
    uqm_lua_checked!(PrintScanTitlePC(
        &raw mut t,
        &raw mut buf as *mut UNICODE,
        (CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
                } << resolutionFactor))
            - ((73 as ::core::ffi::c_int) << resolutionFactor)
            + (if if optWindowType == 1 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                16 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } << resolutionFactor)) as COORD,
    ), ());
    val = (*pSolarSysState).SysInfo.PlanetInfo.SurfaceGravity as SDWORD;
    if val == 0 as ::core::ffi::c_int {
        val = 1 as ::core::ffi::c_int as SDWORD;
    }
    MakeScanValue(
        &raw mut buf as *mut UNICODE,
        val as ::core::ffi::c_long,
        GetStringAddress(
            SetAbsStringTableIndex(
                GameStrings,
                (C2Rust_Unnamed_12::ORBITSCAN_STRING_BASE.0 as ::core::ffi::c_int
                    + 15 as ::core::ffi::c_int) as COUNT,
            ),
        ),
    );
    uqm_lua_checked!(PrintScanText(&raw mut t), ());
    t.baseline.y = (t.baseline.y as ::core::ffi::c_int
        + ((10 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    utf8StringCopy(
        &raw mut buf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[UNICODE; 200]>(),
        GetStringAddress(
            SetAbsStringTableIndex(
                GameStrings,
                (C2Rust_Unnamed_12::ORBITSCAN_STRING_BASE.0 as ::core::ffi::c_int
                    + 16 as ::core::ffi::c_int) as COUNT,
            ),
        ),
    );
    uqm_lua_checked!(PrintScanTitlePC(
        &raw mut t,
        &raw mut buf as *mut UNICODE,
        (CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
                } << resolutionFactor))
            - ((73 as ::core::ffi::c_int) << resolutionFactor)
            + (if if optWindowType == 1 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                16 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } << resolutionFactor)) as COORD,
    ), ());
    val = GetRotationalPeriod() as SDWORD;
    MakeDayValue(
        &raw mut buf as *mut UNICODE,
        val as ::core::ffi::c_long,
        GetStringAddress(
            SetAbsStringTableIndex(
                GameStrings,
                (C2Rust_Unnamed_12::ORBITSCAN_STRING_BASE.0 as ::core::ffi::c_int
                    + 17 as ::core::ffi::c_int) as COUNT,
            ),
        ),
    );
    uqm_lua_checked!(PrintScanText(&raw mut t), ());
    t.baseline.y = (t.baseline.y as ::core::ffi::c_int
        + ((10 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    utf8StringCopy(
        &raw mut buf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[UNICODE; 200]>(),
        GetStringAddress(
            SetAbsStringTableIndex(
                GameStrings,
                (C2Rust_Unnamed_12::ORBITSCAN_STRING_BASE.0 as ::core::ffi::c_int
                    + 18 as ::core::ffi::c_int) as COUNT,
            ),
        ),
    );
    uqm_lua_checked!(PrintScanTitlePC(
        &raw mut t,
        &raw mut buf as *mut UNICODE,
        (CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
                } << resolutionFactor))
            - ((73 as ::core::ffi::c_int) << resolutionFactor)
            + (if if optWindowType == 1 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                16 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } << resolutionFactor)) as COORD,
    ), ());
    val = (*pSolarSysState).SysInfo.PlanetInfo.AxialTilt as SDWORD;
    if val < 0 as ::core::ffi::c_int {
        val = -val;
    }
    snprintf(
        &raw mut buf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[UNICODE; 200]>(),
        b"%d%s\0".as_ptr() as *const ::core::ffi::c_char,
        val,
        b"\xC2\xB0\0".as_ptr() as *const ::core::ffi::c_char,
    );
    uqm_lua_checked!(PrintScanText(&raw mut t), ());
    UnbatchGraphics();
}
unsafe extern "C" fn PrintCoarseScan3DO() {
    let mut val: SDWORD = 0;
    let mut t: TEXT = TEXT {
        baseline: POINT { x: 0, y: 0 },
        pStr: ::core::ptr::null::<UNICODE>(),
        align: TEXT_ALIGN::ALIGN_LEFT,
        CharCount: 0,
    };
    let mut s: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    let mut buf: [UNICODE; 200] = [0; 200];
    let mut frameIndex: COUNT = 20 as COUNT;
    if optWhichCoarseScan == 3 as ::core::ffi::c_int {
        frameIndex = 24 as COUNT;
    }
    SetContext(PlanetContext);
    uqm_lua_checked!(GetPlanetTitle(
        &raw mut buf as *mut UNICODE,
        ::core::mem::size_of::<[UNICODE; 200]>() as COUNT,
    ), ());
    BatchGraphics();
    t.align = TEXT_ALIGN::ALIGN_CENTER;
    t.baseline.x = ((CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
            } << resolutionFactor)) >> resolutionFactor >> 1 as ::core::ffi::c_int)
        << resolutionFactor) as COORD;
    t.baseline.y = ((13 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    t.pStr = &raw mut buf as *mut UNICODE;
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    SetContextForeGroundColor(colorFromRgb15(0xf as BYTE, 0 as BYTE, 0x19 as BYTE));
    SetContextFont(MicroFont);
    uqm_lua_checked!(PrintScanText(&raw mut t), ());
    s.frame = SetAbsFrameIndex(SpaceJunkFrame, frameIndex);
    s.origin.x = (((CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
            } << resolutionFactor)) >> resolutionFactor >> 1 as ::core::ffi::c_int)
        << resolutionFactor) - ((107 as ::core::ffi::c_int) << resolutionFactor))
        as COORD;
    s.origin.y = ((((CanvasHeight
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
        - ((13 as ::core::ffi::c_int) << resolutionFactor) >> resolutionFactor)
        - 67 as ::core::ffi::c_int
        - ((5 as ::core::ffi::c_int) << resolutionFactor >> resolutionFactor)
        >> 1 as ::core::ffi::c_int) << resolutionFactor)
        - (if useDosSpheres.0 != 0 {
            2 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        } << resolutionFactor) - ((46 as ::core::ffi::c_int) << resolutionFactor)
        + (if useDosSpheres.0 != 0 {
            2 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        } << resolutionFactor)) as COORD;
    uqm_lua_checked!(DrawStamp(&raw mut s), ());
    t.baseline.x = (((CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
            } << resolutionFactor)) >> resolutionFactor >> 1 as ::core::ffi::c_int)
        << resolutionFactor) - ((78 as ::core::ffi::c_int) << resolutionFactor))
        as COORD;
    t.baseline.y = ((((CanvasHeight
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
        - ((13 as ::core::ffi::c_int) << resolutionFactor) >> resolutionFactor)
        - 67 as ::core::ffi::c_int
        - ((5 as ::core::ffi::c_int) << resolutionFactor >> resolutionFactor)
        >> 1 as ::core::ffi::c_int) << resolutionFactor)
        - (if useDosSpheres.0 != 0 {
            2 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        } << resolutionFactor) - ((36 as ::core::ffi::c_int) << resolutionFactor)
        + (if useDosSpheres.0 != 0 {
            2 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        } << resolutionFactor)) as COORD;
    t.align = TEXT_ALIGN::ALIGN_LEFT;
    val = ((*pSolarSysState).SysInfo.PlanetInfo.PlanetToSunDist as ::core::ffi::c_ulong)
        .wrapping_mul(100 as ::core::ffi::c_ulong)
        .wrapping_add(
            ((8 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int) as ::core::ffi::c_ulong,
        )
        .wrapping_div(
            ((8 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                as ::core::ffi::c_ulong,
        ) as SDWORD;
    MakeScanValue(
        &raw mut buf as *mut UNICODE,
        val as ::core::ffi::c_long,
        b"\xE2\x99\x81\0".as_ptr() as *const UNICODE,
    );
    uqm_lua_checked!(PrintScanText(&raw mut t), ());
    t.baseline.y = (t.baseline.y as ::core::ffi::c_int
        + ((19 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    if (*pSolarSysState).SysInfo.PlanetInfo.AtmoDensity as ::core::ffi::c_int
        == 0xffff as ::core::ffi::c_int
    {
        strcpy(
            &raw mut buf as *mut ::core::ffi::c_char,
            b"\xE2\x88\x9E\0".as_ptr() as *const ::core::ffi::c_char,
        );
    } else {
        val = (((*pSolarSysState).SysInfo.PlanetInfo.AtmoDensity as ::core::ffi::c_int
            * 100 as ::core::ffi::c_int
            + (50 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int))
            / 50 as ::core::ffi::c_int) as SDWORD;
        MakeScanValue(
            &raw mut buf as *mut UNICODE,
            val as ::core::ffi::c_long,
            b"\xE2\x99\x81\0".as_ptr() as *const UNICODE,
        );
    }
    uqm_lua_checked!(PrintScanText(&raw mut t), ());
    t.baseline.y = (t.baseline.y as ::core::ffi::c_int
        + ((19 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    snprintf(
        &raw mut buf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[UNICODE; 200]>(),
        b"%d%s\0".as_ptr() as *const ::core::ffi::c_char,
        (*pSolarSysState).SysInfo.PlanetInfo.SurfaceTemperature as ::core::ffi::c_int,
        b"\xC2\xB0\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if optHazardColors.0 != 0 {
        HazardCase(C2Rust_Unnamed_8::LAVASPOT_DISASTER.0 as ::core::ffi::c_int as BYTE);
    }
    uqm_lua_checked!(PrintScanText(&raw mut t), ());
    t.baseline.y = (t.baseline.y as ::core::ffi::c_int
        + ((19 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    snprintf(
        &raw mut buf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[UNICODE; 200]>(),
        b"<%u>\0".as_ptr() as *const ::core::ffi::c_char,
        if (*pSolarSysState).SysInfo.PlanetInfo.AtmoDensity as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
            0 as ::core::ffi::c_int
        } else {
            (*pSolarSysState).SysInfo.PlanetInfo.Weather as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int
        },
    );
    if optHazardColors.0 != 0 {
        HazardCase(C2Rust_Unnamed_8::LIGHTNING_DISASTER.0 as ::core::ffi::c_int as BYTE);
    }
    uqm_lua_checked!(PrintScanText(&raw mut t), ());
    t.baseline.y = (t.baseline.y as ::core::ffi::c_int
        + ((19 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    snprintf(
        &raw mut buf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[UNICODE; 200]>(),
        b"<%u>\0".as_ptr() as *const ::core::ffi::c_char,
        if ((*(*pSolarSysState).SysInfo.PlanetInfo.PlanDataPtr).Type
            as ::core::ffi::c_int & 0x3 as ::core::ffi::c_int) as BYTE
            as ::core::ffi::c_int == C2Rust_Unnamed_9::GAS_GIANT.0 as ::core::ffi::c_int
        {
            0 as ::core::ffi::c_int
        } else {
            (*pSolarSysState).SysInfo.PlanetInfo.Tectonics as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int
        },
    );
    if optHazardColors.0 != 0 {
        HazardCase(
            C2Rust_Unnamed_8::EARTHQUAKE_DISASTER.0 as ::core::ffi::c_int as BYTE,
        );
    }
    uqm_lua_checked!(PrintScanText(&raw mut t), ());
    t.baseline.x = (((CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
            } << resolutionFactor)) >> resolutionFactor >> 1 as ::core::ffi::c_int)
        << resolutionFactor) + ((78 as ::core::ffi::c_int) << resolutionFactor))
        as COORD;
    t.baseline.y = ((((CanvasHeight
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
        - ((13 as ::core::ffi::c_int) << resolutionFactor) >> resolutionFactor)
        - 67 as ::core::ffi::c_int
        - ((5 as ::core::ffi::c_int) << resolutionFactor >> resolutionFactor)
        >> 1 as ::core::ffi::c_int) << resolutionFactor)
        - (if useDosSpheres.0 != 0 {
            2 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        } << resolutionFactor) - ((36 as ::core::ffi::c_int) << resolutionFactor)
        + (if useDosSpheres.0 != 0 {
            2 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        } << resolutionFactor)) as COORD;
    t.align = TEXT_ALIGN::ALIGN_RIGHT;
    val = (*pSolarSysState).SysInfo.PlanetInfo.PlanetRadius as SDWORD;
    val = ((val as DWORD).wrapping_mul(val as DWORD).wrapping_mul(val as DWORD)
        as ::core::ffi::c_ulong)
        .wrapping_div(100 as ::core::ffi::c_ulong)
        .wrapping_mul(
            (*pSolarSysState).SysInfo.PlanetInfo.PlanetDensity as ::core::ffi::c_ulong,
        )
        .wrapping_add(
            (100 as ::core::ffi::c_long * 100 as ::core::ffi::c_long
                >> 1 as ::core::ffi::c_int) as ::core::ffi::c_ulong,
        )
        .wrapping_div(
            (100 as ::core::ffi::c_long * 100 as ::core::ffi::c_long)
                as ::core::ffi::c_ulong,
        ) as SDWORD;
    if val == 0 as ::core::ffi::c_int {
        val = 1 as ::core::ffi::c_int as SDWORD;
    }
    MakeScanValue(
        &raw mut buf as *mut UNICODE,
        val as ::core::ffi::c_long,
        b"\xE2\x99\x81\0".as_ptr() as *const UNICODE,
    );
    if optHazardColors.0 != 0 {
        SetContextForeGroundColor(colorFromRgb15(0xf as BYTE, 0 as BYTE, 0x19 as BYTE));
    }
    uqm_lua_checked!(PrintScanText(&raw mut t), ());
    t.baseline.y = (t.baseline.y as ::core::ffi::c_int
        + ((19 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    val = (*pSolarSysState).SysInfo.PlanetInfo.PlanetRadius as SDWORD;
    MakeScanValue(
        &raw mut buf as *mut UNICODE,
        val as ::core::ffi::c_long,
        b"\xE2\x99\x81\0".as_ptr() as *const UNICODE,
    );
    uqm_lua_checked!(PrintScanText(&raw mut t), ());
    t.baseline.y = (t.baseline.y as ::core::ffi::c_int
        + ((19 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    val = (*pSolarSysState).SysInfo.PlanetInfo.SurfaceGravity as SDWORD;
    if val == 0 as ::core::ffi::c_int {
        val = 1 as ::core::ffi::c_int as SDWORD;
    }
    MakeScanValue(
        &raw mut buf as *mut UNICODE,
        val as ::core::ffi::c_long,
        b"\xE2\x99\x81\0".as_ptr() as *const UNICODE,
    );
    uqm_lua_checked!(PrintScanText(&raw mut t), ());
    t.baseline.y = (t.baseline.y as ::core::ffi::c_int
        + ((19 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    val = (*pSolarSysState).SysInfo.PlanetInfo.AxialTilt as SDWORD;
    if val < 0 as ::core::ffi::c_int {
        val = -val;
    }
    snprintf(
        &raw mut buf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[UNICODE; 200]>(),
        b"%d%s\0".as_ptr() as *const ::core::ffi::c_char,
        val,
        b"\xC2\xB0\0".as_ptr() as *const ::core::ffi::c_char,
    );
    uqm_lua_checked!(PrintScanText(&raw mut t), ());
    t.baseline.y = (t.baseline.y as ::core::ffi::c_int
        + ((19 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    val = GetRotationalPeriod() as SDWORD;
    MakeDayValue(
        &raw mut buf as *mut UNICODE,
        val as ::core::ffi::c_long,
        b"\xE2\x99\x81\0".as_ptr() as *const UNICODE,
    );
    uqm_lua_checked!(PrintScanText(&raw mut t), ());
    UnbatchGraphics();
}
unsafe extern "C" fn initPlanetLocationImage() {
    let mut cursorFrame: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
    cursorFrame = SetAbsFrameIndex(MiscDataFrame, 105 as COUNT);
    cursorRect.extent = (*cursorFrame).Bounds;
}
unsafe extern "C" fn savePlanetLocationImage() {
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut cursorFrame: FRAME = SetAbsFrameIndex(MiscDataFrame, 105 as COUNT);
    let mut hs: HOT_SPOT = GetFrameHot(cursorFrame);
    DestroyDrawable(ReleaseDrawable(eraseFrame));
    r = cursorRect;
    r.corner.x = (r.corner.x as ::core::ffi::c_int - hs.x as ::core::ffi::c_int)
        as COORD;
    r.corner.y = (r.corner.y as ::core::ffi::c_int - hs.y as ::core::ffi::c_int)
        as COORD;
    eraseFrame = CaptureDrawable(CopyContextRect(&raw mut r));
    SetFrameHot(eraseFrame, hs);
}
unsafe extern "C" fn restorePlanetLocationImage() {
    let mut s: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    s.origin = cursorRect.corner;
    s.frame = eraseFrame;
    uqm_lua_checked!(DrawStamp(&raw mut s), ());
}
unsafe extern "C" fn drawPlanetCursor(mut filled: BOOLEAN) {
    let mut s: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    s.origin = cursorRect.corner;
    s.frame = SetAbsFrameIndex(MiscDataFrame, 105 as COUNT);
    if filled.0 != 0 {
        uqm_lua_checked!(DrawFilledStamp(&raw mut s), ());
    } else {
        uqm_lua_checked!(DrawStamp(&raw mut s), ());
    };
}
unsafe extern "C" fn setPlanetCursorLoc(mut new_pt: POINT) {
    new_pt.x = (new_pt.x as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as COORD;
    new_pt.y = (new_pt.y as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as COORD;
    cursorRect.corner = new_pt;
}
unsafe extern "C" fn setPlanetLoc(mut new_pt: POINT, mut restoreOld: BOOLEAN) {
    planetLoc = new_pt;
    SetContext(ScanContext);
    if restoreOld.0 != 0 {
        uqm_lua_checked!(restorePlanetLocationImage(), ());
    }
    setPlanetCursorLoc(new_pt);
    savePlanetLocationImage();
}
unsafe extern "C" fn flashPlanetLocation() {
    static mut c: BYTE = 0 as BYTE;
    static mut val: ::core::ffi::c_int = -2 as ::core::ffi::c_int;
    static mut prevPt: POINT = POINT { x: 0, y: 0 };
    static mut NextTime: TimeCount = 0 as TimeCount;
    let mut locChanged: BOOLEAN = BOOLEAN::FALSE;
    let mut Now: TimeCount = GetTimeCounter();
    locChanged = BOOLEAN(
        (prevPt.x as ::core::ffi::c_int != cursorRect.corner.x as ::core::ffi::c_int
            || prevPt.y as ::core::ffi::c_int
                != cursorRect.corner.y as ::core::ffi::c_int) as ::core::ffi::c_int
            as ::core::ffi::c_uint,
    );
    if locChanged.0 == 0 && Now < NextTime {
        return;
    }
    if locChanged.0 != 0 {
        c = 0 as BYTE;
        val = -2 as ::core::ffi::c_int;
        prevPt = cursorRect.corner;
        NextTime = Now
            .wrapping_add(
                (840 as ::core::ffi::c_int / 16 as ::core::ffi::c_int) as TimeCount,
            );
    } else {
        if c as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            || c as ::core::ffi::c_int == 0x1a as ::core::ffi::c_int
        {
            val = -val;
        }
        c = (c as ::core::ffi::c_int + val) as BYTE;
        if Now.wrapping_sub(NextTime)
            > (840 as ::core::ffi::c_int / 16 as ::core::ffi::c_int) as TimeCount
        {
            NextTime = Now
                .wrapping_add(
                    (840 as ::core::ffi::c_int / 16 as ::core::ffi::c_int) as TimeCount,
                );
        } else {
            NextTime = NextTime
                .wrapping_add(
                    (840 as ::core::ffi::c_int / 16 as ::core::ffi::c_int) as TimeCount,
                );
        }
    }
    SetContext(ScanContext);
    SetContextForeGroundColor(colorFromRgb15(c, c, c));
    uqm_lua_checked!(drawPlanetCursor(BOOLEAN::TRUE), ());
}
#[no_mangle]
pub unsafe extern "C" fn RedrawSurfaceScan(mut newLoc: *const POINT) {
    let mut OldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    OldContext = SetContext(ScanContext);
    BatchGraphics();
    uqm_lua_checked!(DrawPlanet(0 as ::core::ffi::c_int, colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE)), ());
    uqm_lua_checked!(DrawScannedObjects(BOOLEAN::TRUE), ());
    if !newLoc.is_null() {
        uqm_lua_checked!(setPlanetLoc(*newLoc, BOOLEAN::FALSE), ());
        uqm_lua_checked!(drawPlanetCursor(BOOLEAN::FALSE), ());
    }
    UnbatchGraphics();
    SetContext(OldContext);
}
unsafe extern "C" fn getLandingFuelNeeded() -> COUNT {
    let mut fuel: COUNT = 0;
    fuel = (((*pSolarSysState).SysInfo.PlanetInfo.SurfaceGravity as ::core::ffi::c_int)
        << 1 as ::core::ffi::c_int) as COUNT;
    if fuel as ::core::ffi::c_int > 3 as ::core::ffi::c_int * 100 as ::core::ffi::c_int {
        fuel = (3 as ::core::ffi::c_int * 100 as ::core::ffi::c_int) as COUNT;
    }
    return fuel;
}
unsafe extern "C" fn spawnFwiffo() {
    let mut hStarShip: HSHIPFRAG = ::core::ptr::null_mut::<::core::ffi::c_void>();
    EncounterGroup = 0 as BYTE;
    PutGroupInfo(0 as ::core::ffi::c_long as DWORD, !(0 as ::core::ffi::c_int) as BYTE);
    ReinitQueue(&raw mut GlobData.Game_state.ip_group_q);
    (CountLinks(&raw mut GlobData.Game_state.npc_built_ship_q) as ::core::ffi::c_int
        == 0 as ::core::ffi::c_int
        || {
            __assert_fail(
                b"CountLinks (&GLOBAL (npc_built_ship_q)) == 0\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/uqm/planets/scan.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                707 as ::core::ffi::c_int,
                b"spawnFwiffo\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    hStarShip = CloneShipFragment(
        RACE_ID::SPATHI_SHIP,
        &raw mut GlobData.Game_state.npc_built_ship_q,
        1 as COUNT,
    );
    if !hStarShip.is_null() {
        let mut StarShipPtr: *mut SHIP_FRAGMENT = ::core::ptr::null_mut::<
            SHIP_FRAGMENT,
        >();
        StarShipPtr = LockShipFrag(
            &raw mut GlobData.Game_state.npc_built_ship_q,
            hStarShip,
        );
        (*StarShipPtr).captains_name_index = (3 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int + 16 as ::core::ffi::c_int) as BYTE;
        UnlockLink(&raw mut GlobData.Game_state.npc_built_ship_q, hStarShip);
    }
}
unsafe extern "C" fn DispatchLander() -> BOOLEAN {
    let mut oldCallback: Option<InputFrameCallback> = None;
    let mut landingFuel: SIZE = getLandingFuelNeeded() as SIZE;
    uqm_lua_checked!(EraseCoarseScan(), BOOLEAN(0));
    oldCallback = SetInputCallback(None);
    if optInfiniteFuel.0 == 0 {
        GlobData.SIS_state.FuelOnBoard = GlobData
            .SIS_state
            .FuelOnBoard
            .wrapping_sub(landingFuel as DWORD);
        uqm_lua_checked!(DrawMineralHelpers(), BOOLEAN(0));
        if optSubmenu.0 == 0 {
            uqm_lua_checked!(DeltaSISGauges(0 as SIZE, 0x7fff as SDWORD, 0 as ::core::ffi::c_int), BOOLEAN(0));
        }
    }
    SetContext(ScanContext);
    uqm_lua_checked!(drawPlanetCursor(BOOLEAN::FALSE), BOOLEAN(0));
    uqm_lua_checked!(PlanetSide(planetLoc), BOOLEAN(0));
    if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
        & C2Rust_Unnamed_7::CHECK_ABORT.0 as ::core::ffi::c_int != 0
    {
        return BOOLEAN::FALSE;
    }
    if uqm_lua_checked!(getGameStateUint(b"FOUND_PLUTO_SPATHI\0".as_ptr() as *const ::core::ffi::c_char), BOOLEAN(0))
        == 1 as DWORD
    {
        spawnFwiffo();
        NextActivity = (NextActivity as ::core::ffi::c_int
            | C2Rust_Unnamed_7::CHECK_LOAD.0 as ::core::ffi::c_int) as ACTIVITY;
        GlobData.Game_state.CurrentActivity = (GlobData.Game_state.CurrentActivity
            as ::core::ffi::c_int
            | C2Rust_Unnamed_7::START_ENCOUNTER.0 as ::core::ffi::c_int) as ACTIVITY;
        uqm_lua_checked!(SaveSolarSysLocation(), BOOLEAN(0));
        return BOOLEAN::FALSE;
    }
    if optWhichCoarseScan & 1 as ::core::ffi::c_int != 0 {
        uqm_lua_checked!(PrintCoarseScan3DO(), BOOLEAN(0));
    } else {
        uqm_lua_checked!(PrintCoarseScanPC(), BOOLEAN(0));
    }
    SetInputCallback(oldCallback);
    return BOOLEAN::TRUE;
}
unsafe extern "C" fn DoPickPlanetSide(mut pMS: *mut MENU_STATE) -> BOOLEAN {
    let mut pickState: *mut PICK_PLANET_STATE = (*pMS).privData
        as *mut PICK_PLANET_STATE;
    let mut TimeIn: DWORD = GetTimeCounter();
    let mut select: BOOLEAN = BOOLEAN::FALSE;
    let mut cancel: BOOLEAN = BOOLEAN::FALSE;
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
    if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
        & C2Rust_Unnamed_7::CHECK_ABORT.0 as ::core::ffi::c_int != 0
    {
        (*pickState).success = false;
        return BOOLEAN::FALSE;
    }
    if cancel.0 != 0 {
        (*pickState).success = false;
        return BOOLEAN::FALSE;
    } else if select.0 != 0 {
        (*pickState).success = true;
        return BOOLEAN::FALSE;
    } else {
        let mut dx: SIZE = 0 as SIZE;
        let mut dy: SIZE = 0 as SIZE;
        let mut new_pt: POINT = POINT { x: 0, y: 0 };
        static mut tNext: DWORD = 0;
        new_pt = planetLoc;
        if tNext != 0 && TimeIn >= tNext {
            tNext = 0 as DWORD;
        }
        if tNext == 0 || TimeIn >= tNext {
            if CurrentInputState
                .menu[C2Rust_Unnamed::KEY_MENU_LEFT.0 as ::core::ffi::c_int as usize]
                != 0
            {
                dx = -((1 as ::core::ffi::c_int) << resolutionFactor) as SIZE;
            }
            if CurrentInputState
                .menu[C2Rust_Unnamed::KEY_MENU_RIGHT.0 as ::core::ffi::c_int as usize]
                != 0
            {
                dx = ((1 as ::core::ffi::c_int) << resolutionFactor) as SIZE;
            }
            if CurrentInputState
                .menu[C2Rust_Unnamed::KEY_MENU_UP.0 as ::core::ffi::c_int as usize] != 0
            {
                dy = -((1 as ::core::ffi::c_int) << resolutionFactor) as SIZE;
            }
            if CurrentInputState
                .menu[C2Rust_Unnamed::KEY_MENU_DOWN.0 as ::core::ffi::c_int as usize]
                != 0
            {
                dy = ((1 as ::core::ffi::c_int) << resolutionFactor) as SIZE;
            }
            if DirKeysPress().0 != 0
                && CurrentInputState
                    .menu[C2Rust_Unnamed::KEY_MENU_NEXT.0 as ::core::ffi::c_int as usize]
                    != 0
            {
                dx = (dx as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as SIZE;
                dy = (dy as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as SIZE;
            }
        }
        BatchGraphics();
        dx = ((dx as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as SIZE;
        if dx != 0 {
            new_pt.x = (new_pt.x as ::core::ffi::c_int + dx as ::core::ffi::c_int)
                as COORD;
            if (new_pt.x as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                new_pt.x = (new_pt.x as ::core::ffi::c_int
                    + (if is3DO(optSuperPC).0 != 0 {
                        243 as ::core::ffi::c_int
                    } else {
                        210 as ::core::ffi::c_int
                    }
                        - if if optWindowType == 1 as DWORD {
                            BOOLEAN::TRUE.0 as ::core::ffi::c_int
                        } else {
                            BOOLEAN::FALSE.0 as ::core::ffi::c_int
                        } != 0
                        {
                            33 as ::core::ffi::c_int
                        } else {
                            0 as ::core::ffi::c_int
                        } << resolutionFactor << 2 as ::core::ffi::c_int)) as COORD;
            } else if new_pt.x as ::core::ffi::c_int
                >= if is3DO(optSuperPC).0 != 0 {
                    243 as ::core::ffi::c_int
                } else {
                    210 as ::core::ffi::c_int
                }
                    - if if optWindowType == 1 as DWORD {
                        BOOLEAN::TRUE.0 as ::core::ffi::c_int
                    } else {
                        BOOLEAN::FALSE.0 as ::core::ffi::c_int
                    } != 0
                    {
                        33 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    } << resolutionFactor << 2 as ::core::ffi::c_int
            {
                new_pt.x = (new_pt.x as ::core::ffi::c_int
                    - (if is3DO(optSuperPC).0 != 0 {
                        243 as ::core::ffi::c_int
                    } else {
                        210 as ::core::ffi::c_int
                    }
                        - if if optWindowType == 1 as DWORD {
                            BOOLEAN::TRUE.0 as ::core::ffi::c_int
                        } else {
                            BOOLEAN::FALSE.0 as ::core::ffi::c_int
                        } != 0
                        {
                            33 as ::core::ffi::c_int
                        } else {
                            0 as ::core::ffi::c_int
                        } << resolutionFactor << 2 as ::core::ffi::c_int)) as COORD;
            }
        }
        dy = ((dy as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as SIZE;
        if dy != 0 {
            new_pt.y = (new_pt.y as ::core::ffi::c_int + dy as ::core::ffi::c_int)
                as COORD;
            if (new_pt.y as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
                || new_pt.y as ::core::ffi::c_int
                    >= (67 as ::core::ffi::c_int) << resolutionFactor
                        << 2 as ::core::ffi::c_int
            {
                new_pt.y = planetLoc.y;
            }
        }
        if !pointsEqual(new_pt, planetLoc) {
            uqm_lua_checked!(setPlanetLoc(new_pt, BOOLEAN::TRUE), BOOLEAN(0));
        }
        uqm_lua_checked!(flashPlanetLocation(), BOOLEAN(0));
        UnbatchGraphics();
        if dx as ::core::ffi::c_int != 0 || dy as ::core::ffi::c_int != 0 {
            tNext = TimeIn
                .wrapping_add(
                    (840 as ::core::ffi::c_int / 40 as ::core::ffi::c_int) as DWORD,
                );
        }
    }
    return BOOLEAN::TRUE;
}
unsafe extern "C" fn drawLandingFuelUsage(mut fuel: COUNT) {
    let mut old_status_message_mode: StatMsgMode = StatMsgMode::SMM_UNDEFINED;
    let mut buf: [UNICODE; 100] = [0; 100];
    if GlobData.SIS_state.FuelOnBoard as ::core::ffi::c_int - fuel as ::core::ffi::c_int
        <= uqm_lua_checked!(get_fuel_to_sol(), ()) as SDWORD
    {
        old_status_message_mode = SetStatusMessageMode(StatMsgMode::SMM_ALERT);
    } else if GlobData.SIS_state.FuelOnBoard as ::core::ffi::c_int
        - fuel as ::core::ffi::c_int
        <= uqm_lua_checked!(get_fuel_to_sol(), ())
            .wrapping_add((5 as ::core::ffi::c_int * 100 as ::core::ffi::c_int) as DWORD)
            as SDWORD
    {
        old_status_message_mode = SetStatusMessageMode(StatMsgMode::SMM_WARNING);
    }
    sprintf(
        &raw mut buf as *mut ::core::ffi::c_char,
        b"%s%1.1f\0".as_ptr() as *const ::core::ffi::c_char,
        GetStringAddress(
            SetAbsStringTableIndex(
                GameStrings,
                (C2Rust_Unnamed_12::NAVIGATION_STRING_BASE.0 as ::core::ffi::c_int
                    + 5 as ::core::ffi::c_int) as COUNT,
            ),
        ),
        (fuel as ::core::ffi::c_float
            / 100 as ::core::ffi::c_int as ::core::ffi::c_float) as ::core::ffi::c_double,
    );
    uqm_lua_checked!(DrawStatusMessage(&raw mut buf as *mut UNICODE), ());
    if old_status_message_mode.0 != StatMsgMode::SMM_UNDEFINED.0 {
        SetStatusMessageMode(old_status_message_mode);
    }
}
unsafe extern "C" fn eraseLandingFuelUsage() {
    uqm_lua_checked!(DrawStatusMessage(::core::ptr::null::<UNICODE>()), ());
}
unsafe extern "C" fn PickPlanetSide() -> BOOLEAN {
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
    let mut PickState: PICK_PLANET_STATE = PICK_PLANET_STATE {
        success: false,
    };
    let mut fuel: COUNT = getLandingFuelNeeded();
    let mut retval: BOOLEAN = BOOLEAN::TRUE;
    memset(
        &raw mut MenuState as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<MENU_STATE>(),
    );
    MenuState.privData = &raw mut PickState as *mut ::core::ffi::c_void;
    if is3DO(optSuperPC).0 != 0 {
        uqm_lua_checked!(ClearSISRect(((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as BYTE), BOOLEAN(0));
    }
    SetContext(ScanContext);
    BatchGraphics();
    uqm_lua_checked!(DrawPlanet(0 as ::core::ffi::c_int, colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE)), BOOLEAN(0));
    uqm_lua_checked!(DrawScannedObjects(BOOLEAN::FALSE), BOOLEAN(0));
    UnbatchGraphics();
    uqm_lua_checked!(drawLandingFuelUsage(fuel), BOOLEAN(0));
    setPlanetCursorLoc(planetLoc);
    savePlanetLocationImage();
    if is3DO(optSuperPC).0 != 0 {
        uqm_lua_checked!(InitLander(0 as BYTE), BOOLEAN(0));
        uqm_lua_checked!(DrawRadarBorder(), BOOLEAN(0));
    }
    SetMenuSounds(
        0 as ::core::ffi::c_int as MENU_SOUND_FLAGS,
        ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as MENU_SOUND_FLAGS,
    );
    PickState.success = false;
    MenuState.InputFunc = Some(
        DoPickPlanetSide as unsafe extern "C" fn(*mut MENU_STATE) -> BOOLEAN,
    ) as Option<unsafe extern "C" fn(*mut menu_state) -> BOOLEAN>;
    uqm_lua_checked!(DoInput(&raw mut MenuState as *mut ::core::ffi::c_void, BOOLEAN::TRUE), BOOLEAN(0));
    uqm_lua_checked!(eraseLandingFuelUsage(), BOOLEAN(0));
    if PickState.success {
        retval = uqm_lua_checked!(DispatchLander(), BOOLEAN(0));
    } else {
        uqm_lua_checked!(restorePlanetLocationImage(), BOOLEAN(0));
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
    return retval;
}
unsafe extern "C" fn DrawScannedStuff(mut y: COUNT, mut scan: COUNT) {
    let mut hElement: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut hNextElement: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut OldColor: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    OldColor = SetContextForeGroundColor(
        colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE),
    );
    hElement = disp_q.head as HELEMENT;
    while !hElement.is_null() {
        let mut ElementPtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
        let mut dy: SIZE = 0;
        let mut s: STAMP = STAMP {
            origin: POINT { x: 0, y: 0 },
            frame: ::core::ptr::null_mut::<FRAME_DESC>(),
        };
        ElementPtr = LockLink(&raw mut disp_q, hElement) as *mut ELEMENT;
        hNextElement = (*ElementPtr).succ;
        dy = (y as ::core::ffi::c_int
            - (*ElementPtr).current.location.y as ::core::ffi::c_int) as SIZE;
        if (*ElementPtr).c2rust_unnamed.scan_node as BYTE as ::core::ffi::c_int
            != scan as ::core::ffi::c_int
            || (dy as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
        {
            UnlockLink(&raw mut disp_q, hElement);
        } else {
            (*ElementPtr).state_flags = ((*ElementPtr).state_flags as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as ELEMENT_FLAGS;
            s.origin = (*ElementPtr).current.location;
            if dy as ::core::ffi::c_int >= 8 as ::core::ffi::c_int {
                s.frame = (*ElementPtr).next.image.frame;
                uqm_lua_checked!(DrawStamp(&raw mut s), ());
            } else {
                let mut grad: BYTE = 0;
                let mut c: Color = colorFromRgb15(
                    0x1f as BYTE,
                    0x1f as BYTE,
                    0x1f as BYTE,
                );
                let mut nodeSize: COUNT = 0;
                grad = (0xff as ::core::ffi::c_int
                    - 0xff as ::core::ffi::c_int * dy as ::core::ffi::c_int
                        / (8 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)) as BYTE;
                match scan as ::core::ffi::c_int {
                    0 => {
                        c.r = grad;
                    }
                    1 => {
                        c.g = grad;
                        c.b = grad;
                    }
                    2 => {
                        c.g = grad;
                    }
                    _ => {}
                }
                SetContextForeGroundColor(c);
                nodeSize = (GetFrameIndex((*ElementPtr).next.image.frame)
                    as ::core::ffi::c_int
                    - GetFrameIndex((*ElementPtr).current.image.frame)
                        as ::core::ffi::c_int) as COUNT;
                if dy as ::core::ffi::c_int > nodeSize as ::core::ffi::c_int {
                    dy = nodeSize as SIZE;
                }
                s.frame = SetRelFrameIndex((*ElementPtr).current.image.frame, dy);
                uqm_lua_checked!(DrawFilledStamp(&raw mut s), ());
            }
            UnlockLink(&raw mut disp_q, hElement);
        }
        hElement = hNextElement;
    }
    SetContextForeGroundColor(OldColor);
}
unsafe extern "C" fn DrawPCScannedStuff(mut scan: COUNT) {
    let mut hElement: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut hNextElement: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut interval: TimeCount = 0;
    let mut now: TimeCount = 0;
    let mut ElementPtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
    let mut s: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    interval = (840 as ::core::ffi::c_int / 10 as ::core::ffi::c_int) as TimeCount;
    hElement = disp_q.head as HELEMENT;
    now = GetTimeCounter().wrapping_add(interval);
    while !hElement.is_null() && uqm_lua_checked!(AnyButtonPress(BOOLEAN::TRUE), ()).0 == 0 {
        if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
            & C2Rust_Unnamed_7::CHECK_ABORT.0 as ::core::ffi::c_int != 0
        {
            return;
        }
        if GetTimeCounter() >= now {
            ElementPtr = LockLink(&raw mut disp_q, hElement) as *mut ELEMENT;
            hNextElement = (*ElementPtr).succ;
            if (*ElementPtr).c2rust_unnamed.scan_node as BYTE as ::core::ffi::c_int
                != scan as ::core::ffi::c_int
            {
                UnlockLink(&raw mut disp_q, hElement);
                hElement = hNextElement;
            } else {
                let mut nodeSize: COUNT = 0;
                let mut growth: COUNT = 0;
                let mut diff: COUNT = 0;
                growth = 0 as COUNT;
                (*ElementPtr).state_flags = ((*ElementPtr).state_flags
                    as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                    as ELEMENT_FLAGS;
                s.origin = (*ElementPtr).current.location;
                now = GetTimeCounter().wrapping_add(17 as TimeCount);
                while (growth as ::core::ffi::c_int) < 8 as ::core::ffi::c_int
                    && uqm_lua_checked!(AnyButtonPress(BOOLEAN::TRUE), ()).0 == 0
                {
                    if GetTimeCounter() >= now {
                        now = GetTimeCounter().wrapping_add(17 as TimeCount);
                        diff = growth;
                        nodeSize = (GetFrameIndex((*ElementPtr).next.image.frame)
                            as ::core::ffi::c_int
                            - GetFrameIndex((*ElementPtr).current.image.frame)
                                as ::core::ffi::c_int) as COUNT;
                        if diff as ::core::ffi::c_int > nodeSize as ::core::ffi::c_int {
                            diff = nodeSize;
                        }
                        s.frame = SetRelFrameIndex(
                            (*ElementPtr).current.image.frame,
                            diff as SIZE,
                        );
                        uqm_lua_checked!(DrawStamp(&raw mut s), ());
                        growth = growth.wrapping_add(1);
                    }
                    uqm_lua_checked!(RotatePlanetSphere(BOOLEAN::TRUE, ::core::ptr::null_mut::<STAMP>()), ());
                }
                if (growth as ::core::ffi::c_int) < 8 as ::core::ffi::c_int {
                    s.frame = (*ElementPtr).next.image.frame;
                    uqm_lua_checked!(DrawStamp(&raw mut s), ());
                }
                UnlockLink(&raw mut disp_q, hElement);
                now = GetTimeCounter().wrapping_add(interval);
                hElement = hNextElement;
            }
        }
        uqm_lua_checked!(RotatePlanetSphere(BOOLEAN::TRUE, ::core::ptr::null_mut::<STAMP>()), ());
    }
    if !hElement.is_null() {
        while !hElement.is_null() {
            ElementPtr = LockLink(&raw mut disp_q, hElement) as *mut ELEMENT;
            hNextElement = (*ElementPtr).succ;
            if (*ElementPtr).c2rust_unnamed.scan_node as BYTE as ::core::ffi::c_int
                != scan as ::core::ffi::c_int
            {
                UnlockLink(&raw mut disp_q, hElement);
                hElement = hNextElement;
            } else {
                (*ElementPtr).state_flags = ((*ElementPtr).state_flags
                    as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                    as ELEMENT_FLAGS;
                UnlockLink(&raw mut disp_q, hElement);
                hElement = hNextElement;
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn callGenerateForScanType(
    mut solarSys: *const SOLARSYS_STATE,
    mut world: *const PLANET_DESC,
    mut node: COUNT,
    mut scanType: BYTE,
    mut info: *mut NODE_INFO,
) -> COUNT {
    match scanType as ::core::ffi::c_int {
        0 => {
            return uqm_lua_checked!(Some(
                    (*(*solarSys).genFuncs)
                        .generateMinerals
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(solarSys, world, node, info), 0);
        }
        1 => {
            return uqm_lua_checked!(Some(
                    (*(*solarSys).genFuncs)
                        .generateEnergy
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(solarSys, world, node, info), 0);
        }
        2 => {
            return uqm_lua_checked!(Some(
                    (*(*solarSys).genFuncs)
                        .generateLife
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(solarSys, world, node, info), 0);
        }
        _ => {}
    }
    (false
        || {
            __assert_fail(
                b"false\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/uqm/planets/scan.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                1152 as ::core::ffi::c_int,
                b"callGenerateForScanType\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    return 0 as COUNT;
}
#[no_mangle]
pub unsafe extern "C" fn callPickupForScanType(
    mut solarSys: *mut SOLARSYS_STATE,
    mut world: *mut PLANET_DESC,
    mut node: COUNT,
    mut scanType: BYTE,
) -> bool {
    match scanType as ::core::ffi::c_int {
        0 => {
            return uqm_lua_checked!(Some(
                    (*(*solarSys).genFuncs)
                        .pickupMinerals
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(solarSys, world, node), false);
        }
        1 => {
            return uqm_lua_checked!(Some(
                    (*(*solarSys).genFuncs)
                        .pickupEnergy
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(solarSys, world, node), false);
        }
        2 => {
            return uqm_lua_checked!(Some(
                    (*(*solarSys).genFuncs)
                        .pickupLife
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(solarSys, world, node), false);
        }
        _ => {}
    }
    (false
        || {
            __assert_fail(
                b"false\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/uqm/planets/scan.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                1173 as ::core::ffi::c_int,
                b"callPickupForScanType\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    return false;
}
unsafe extern "C" fn ScanPlanet(mut scanType: COUNT) {
    let mut startScan: COUNT = 0;
    let mut endScan: COUNT = 0;
    let mut scan: COUNT = 0;
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    static mut textColors: [Color; 3] = [
        Color {
            r: ((0x13 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x13 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
        Color {
            r: ((0xc as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0xc as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0xc as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0xc as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0xc as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0xc as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
        Color {
            r: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0xe as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0xe as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
    ];
    static mut tintColors: [Color; 3] = [
        Color {
            r: ((0x1f as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x1f as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
        Color {
            r: ((0x1f as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x1f as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0x1f as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x1f as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0x1f as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x1f as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
        Color {
            r: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0x1f as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x1f as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
    ];
    if scanType as ::core::ffi::c_int == ScanMenuItems::AUTO_SCAN.0 as ::core::ffi::c_int
    {
        startScan = PlanetScanTypes::MINERAL_SCAN.0 as ::core::ffi::c_int as COUNT;
        endScan = PlanetScanTypes::BIOLOGICAL_SCAN.0 as ::core::ffi::c_int as COUNT;
    } else {
        startScan = scanType;
        endScan = scanType;
    }
    if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
        & C2Rust_Unnamed_7::CHECK_ABORT.0 as ::core::ffi::c_int != 0
    {
        return;
    }
    uqm_lua_checked!(DrawMenuStateStrings(
        C2Rust_Unnamed_11::PM_MIN_SCAN.0 as ::core::ffi::c_int as BYTE,
        scanType as SWORD,
    ), ());
    scan = startScan;
    while scan as ::core::ffi::c_int <= endScan as ::core::ffi::c_int {
        let mut t: TEXT = TEXT {
            baseline: POINT { x: 0, y: 0 },
            pStr: ::core::ptr::null::<UNICODE>(),
            align: TEXT_ALIGN::ALIGN_LEFT,
            CharCount: 0,
        };
        let mut i: SWORD = 0 as SWORD;
        let mut tintColor: Color = Color { r: 0, g: 0, b: 0, a: 0 };
        static mut TimeOut: TimeCount = 0;
        static mut TimeOutDraw: TimeCount = 0;
        let mut Now: TimeCount = 0;
        t.baseline.x = ((CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
                } << resolutionFactor)) >> resolutionFactor >> 1 as ::core::ffi::c_int)
            << resolutionFactor) as COORD;
        t.baseline.y = (CanvasHeight
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
            - ((67 as ::core::ffi::c_int) << resolutionFactor)
            - ((7 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        t.align = TEXT_ALIGN::ALIGN_CENTER;
        t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
        t.pStr = GetStringAddress(
            SetAbsStringTableIndex(
                GameStrings,
                (C2Rust_Unnamed_12::SCAN_STRING_BASE.0 as ::core::ffi::c_int
                    + scan as ::core::ffi::c_int) as COUNT,
            ),
        );
        SetContext(PlanetContext);
        r.corner.x = 0 as COORD;
        r.corner.y = (t.baseline.y as ::core::ffi::c_int
            - ((10 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        r.extent.width = (CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
                } << resolutionFactor))) as COORD;
        r.extent.height = (t.baseline.y as ::core::ffi::c_int
            - r.corner.y as ::core::ffi::c_int
            + ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        RepairBackRect(&raw mut r);
        SetContextFont(MicroFont);
        SetContextForeGroundColor(textColors[scan as usize]);
        uqm_lua_checked!(font_DrawText(&raw mut t), ());
        SetContext(ScanContext);
        if optScanStyle != 0x2 as ::core::ffi::c_int {
            uqm_lua_checked!(DrawPlanet(
                0 as ::core::ffi::c_int,
                colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE),
            ), ());
        }
        tintColor = tintColors[scan as usize];
        FlushInput();
        (*pSolarSysState).Orbit.scanType = scan;
        uqm_lua_checked!(RerenderPlanetSphere(), ());
        if optScanStyle != 0x2 as ::core::ffi::c_int {
            while (i as ::core::ffi::c_int)
                < (if optPlanetTexture.0 != 0 {
                    75 as ::core::ffi::c_int
                } else {
                    67 as ::core::ffi::c_int
                } + 8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                    << resolutionFactor
            {
                if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
                    & C2Rust_Unnamed_7::CHECK_ABORT.0 as ::core::ffi::c_int != 0
                {
                    return;
                }
                Now = GetTimeCounter();
                if Now >= TimeOut {
                    if uqm_lua_checked!(AnyButtonPress(BOOLEAN::TRUE), ()).0 != 0 {
                        break;
                    }
                    TimeOut = Now
                        .wrapping_add(
                            ((840 as ::core::ffi::c_int * 7 as ::core::ffi::c_int
                                / 4 as ::core::ffi::c_int
                                + if if resolutionFactor != 2 as ::core::ffi::c_uint {
                                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                                } else {
                                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                                } == 0
                                {
                                    0xff as ::core::ffi::c_int
                                } else {
                                    0 as ::core::ffi::c_int
                                })
                                / (if optPlanetTexture.0 != 0 {
                                    75 as ::core::ffi::c_int
                                } else {
                                    67 as ::core::ffi::c_int
                                } + 8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int))
                                as TimeCount,
                        );
                    i = (i as ::core::ffi::c_int
                        + ((1 as ::core::ffi::c_int) << resolutionFactor)) as SWORD;
                    if Now >= TimeOutDraw {
                        TimeOutDraw = Now
                            .wrapping_add(
                                (840 as ::core::ffi::c_int
                                    / if if resolutionFactor != 2 as ::core::ffi::c_uint {
                                        BOOLEAN::FALSE.0 as ::core::ffi::c_int
                                    } else {
                                        BOOLEAN::TRUE.0 as ::core::ffi::c_int
                                    } == 0
                                    {
                                        42 as ::core::ffi::c_int
                                    } else {
                                        60 as ::core::ffi::c_int
                                    }) as TimeCount,
                            );
                        BatchGraphics();
                        uqm_lua_checked!(DrawPlanet(i as ::core::ffi::c_int, tintColor), ());
                        uqm_lua_checked!(DrawScannedStuff(i as COUNT, scan), ());
                        UnbatchGraphics();
                    }
                }
                uqm_lua_checked!(RotatePlanetSphere(BOOLEAN::TRUE, ::core::ptr::null_mut::<STAMP>()), ());
            }
        } else {
            uqm_lua_checked!(DrawPCScanTint(scan), ());
            uqm_lua_checked!(DrawPCScannedStuff(scan), ());
            if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
                & C2Rust_Unnamed_7::CHECK_ABORT.0 as ::core::ffi::c_int != 0
            {
                return;
            }
            if scanType as ::core::ffi::c_int
                == ScanMenuItems::AUTO_SCAN.0 as ::core::ffi::c_int
            {
                TimeOut = GetTimeCounter().wrapping_add(840 as TimeCount);
                while GetTimeCounter() < TimeOut && uqm_lua_checked!(AnyButtonPress(BOOLEAN::TRUE), ()).0 == 0
                {
                    uqm_lua_checked!(RotatePlanetSphere(BOOLEAN::TRUE, ::core::ptr::null_mut::<STAMP>()), ());
                }
            } else {
                while uqm_lua_checked!(AnyButtonPress(BOOLEAN::TRUE), ()).0 == 0 {
                    uqm_lua_checked!(RotatePlanetSphere(BOOLEAN::TRUE, ::core::ptr::null_mut::<STAMP>()), ());
                }
            }
        }
        if (i as ::core::ffi::c_int)
            < (if optPlanetTexture.0 != 0 {
                75 as ::core::ffi::c_int
            } else {
                67 as ::core::ffi::c_int
            } + 8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) << resolutionFactor
            && optScanStyle != 0x2 as ::core::ffi::c_int
        {
            BatchGraphics();
            uqm_lua_checked!(DrawPlanet(
                ((if optPlanetTexture.0 != 0 {
                    75 as ::core::ffi::c_int
                } else {
                    67 as ::core::ffi::c_int
                } + 8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                    << resolutionFactor) - 1 as ::core::ffi::c_int,
                tintColor,
            ), ());
            uqm_lua_checked!(DrawScannedStuff(
                (((if optPlanetTexture.0 != 0 {
                    75 as ::core::ffi::c_int
                } else {
                    67 as ::core::ffi::c_int
                } + 8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                    << resolutionFactor) - 1 as ::core::ffi::c_int) as COUNT,
                scan,
            ), ());
            UnbatchGraphics();
        }
        scan = scan.wrapping_add(1);
    }
    SetContext(PlanetContext);
    RepairBackRect(&raw mut r);
    SetContext(ScanContext);
    (*pSolarSysState).Orbit.scanType = PlanetScanTypes::NUM_SCAN_TYPES.0
        as ::core::ffi::c_int as COUNT;
    if optScanStyle == 0x2 as ::core::ffi::c_int || useDosSpheres.0 != 0 {
        uqm_lua_checked!(RerenderPlanetSphere(), ());
    }
    if scanType as ::core::ffi::c_int == ScanMenuItems::AUTO_SCAN.0 as ::core::ffi::c_int
        || optScanStyle == 0x2 as ::core::ffi::c_int
    {
        uqm_lua_checked!(DrawPlanet(
            0 as ::core::ffi::c_int,
            colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE),
        ), ());
        uqm_lua_checked!(DrawDefaultPlanetSphere(), ());
        uqm_lua_checked!(DrawScannedObjects(BOOLEAN::FALSE), ());
    }
    FlushInput();
}
unsafe extern "C" fn DoScan(mut pMS: *mut MENU_STATE) -> BOOLEAN {
    let mut select: BOOLEAN = BOOLEAN::FALSE;
    let mut cancel: BOOLEAN = BOOLEAN::FALSE;
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
    if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
        & (C2Rust_Unnamed_7::CHECK_ABORT.0 as ::core::ffi::c_int
            | C2Rust_Unnamed_7::CHECK_LOAD.0 as ::core::ffi::c_int) != 0
    {
        return BOOLEAN::FALSE;
    }
    if cancel.0 != 0
        || select.0 != 0
            && (*pMS).CurState as ::core::ffi::c_int
                == ScanMenuItems::EXIT_SCAN.0 as ::core::ffi::c_int
    {
        return BOOLEAN::FALSE
    } else if select.0 != 0 {
        if (*pMS).CurState as ::core::ffi::c_int
            == ScanMenuItems::DISPATCH_SHUTTLE.0 as ::core::ffi::c_int
        {
            let mut fuel_required: COUNT = 0;
            if (*(*pSolarSysState).pOrbitalDesc).data_index as ::core::ffi::c_int
                & C2Rust_Unnamed_10::PLANET_SHIELDED.0 as ::core::ffi::c_int != 0
                || (*pSolarSysState).SysInfo.PlanetInfo.AtmoDensity as ::core::ffi::c_int
                    == 0xffff as ::core::ffi::c_int
            {
                uqm_lua_checked!(PlayMenuSound(MENU_SOUND_EFFECT::MENU_SOUND_FAILURE), BOOLEAN(0));
                return BOOLEAN::TRUE;
            }
            fuel_required = getLandingFuelNeeded();
            if GlobData.SIS_state.FuelOnBoard < fuel_required as DWORD
                || GlobData.SIS_state.NumLanders as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                || GlobData.SIS_state.CrewEnlisted as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
            {
                uqm_lua_checked!(PlayMenuSound(MENU_SOUND_EFFECT::MENU_SOUND_FAILURE), BOOLEAN(0));
                return BOOLEAN::TRUE;
            }
            uqm_lua_checked!(SetFlashRect(::core::ptr::null::<RECT>(), BOOLEAN::FALSE), BOOLEAN(0));
            uqm_lua_checked!(DrawMenuStateStrings(
                C2Rust_Unnamed_11::PM_MIN_SCAN.0 as ::core::ffi::c_int as BYTE,
                (*pMS).CurState as SWORD,
            ), BOOLEAN(0));
            if uqm_lua_checked!(PickPlanetSide(), BOOLEAN(0)).0 == 0 {
                return BOOLEAN::FALSE;
            }
            uqm_lua_checked!(DrawMenuStateStrings(
                C2Rust_Unnamed_11::PM_MIN_SCAN.0 as ::core::ffi::c_int as BYTE,
                (*pMS).CurState as SWORD,
            ), BOOLEAN(0));
            uqm_lua_checked!(SetFlashRect(
                ::core::ptr::with_exposed_provenance_mut::<
                    RECT,
                >(!(0 as ::core::ffi::c_long) as usize),
                BOOLEAN::FALSE,
            ), BOOLEAN(0));
            return BOOLEAN::TRUE;
        }
        if (*(*pSolarSysState).pOrbitalDesc).data_index as ::core::ffi::c_int
            & C2Rust_Unnamed_10::PLANET_SHIELDED.0 as ::core::ffi::c_int != 0
        {
            uqm_lua_checked!(PlayMenuSound(MENU_SOUND_EFFECT::MENU_SOUND_FAILURE), BOOLEAN(0));
            return BOOLEAN::TRUE;
        }
        uqm_lua_checked!(ScanPlanet((*pMS).CurState as COUNT), BOOLEAN(0));
        if (*pMS).CurState as ::core::ffi::c_int
            == ScanMenuItems::AUTO_SCAN.0 as ::core::ffi::c_int
        {
            (*pMS).CurState = ScanMenuItems::DISPATCH_SHUTTLE.0 as ::core::ffi::c_int
                as BYTE;
        }
        uqm_lua_checked!(DrawMenuStateStrings(
            C2Rust_Unnamed_11::PM_MIN_SCAN.0 as ::core::ffi::c_int as BYTE,
            (*pMS).CurState as SWORD,
        ), BOOLEAN(0));
    } else if optWhichMenu == 0x2 as ::core::ffi::c_int
        || (*(*pSolarSysState).pOrbitalDesc).data_index as ::core::ffi::c_int
            & C2Rust_Unnamed_10::PLANET_SHIELDED.0 as ::core::ffi::c_int == 0
            && (*pSolarSysState).SysInfo.PlanetInfo.AtmoDensity as ::core::ffi::c_int
                != 0xffff as ::core::ffi::c_int
    {
        uqm_lua_checked!(DoMenuChooser(
            pMS,
            C2Rust_Unnamed_11::PM_MIN_SCAN.0 as ::core::ffi::c_int as BYTE,
        ), BOOLEAN(0));
    }
    return BOOLEAN::TRUE;
}
unsafe extern "C" fn CreateScanContext(mut inSpace: BOOLEAN) -> CONTEXT {
    let mut oldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    let mut context: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut x_offset: COORD = 0;
    oldContext = SetContext(SpaceContext);
    GetContextClipRect(&raw mut r);
    context = CreateContextAux(b"ScanContext\0".as_ptr() as *const ::core::ffi::c_char);
    SetContext(context);
    SetContextFGFrame(Screen);
    x_offset = (r.extent.width as ::core::ffi::c_int
        - (if is3DO(optSuperPC).0 != 0 {
            243 as ::core::ffi::c_int
        } else {
            210 as ::core::ffi::c_int
        }
            - if if optWindowType == 1 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                33 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } << resolutionFactor) >> resolutionFactor) as COORD;
    r.extent.width = (if is3DO(optSuperPC).0 != 0 {
        243 as ::core::ffi::c_int
    } else {
        210 as ::core::ffi::c_int
    }
        - if if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            33 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        } << resolutionFactor) as COORD;
    if inSpace.0 != 0 && x_offset as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
        x_offset = (x_offset as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as COORD;
        x_offset = (x_offset as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as COORD;
        r.extent.width = (r.extent.width as ::core::ffi::c_int
            - ((5 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    }
    r.corner.x = (r.corner.x as ::core::ffi::c_int
        + ((x_offset as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    r.corner.y = (r.corner.y as ::core::ffi::c_int
        + (r.extent.height as ::core::ffi::c_int
            - ((67 as ::core::ffi::c_int) << resolutionFactor))) as COORD;
    r.extent.height = ((67 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    SetContextClipRect(&raw mut r);
    SetContext(oldContext);
    return context;
}
#[no_mangle]
pub unsafe extern "C" fn GetScanContext(mut owner: *mut BOOLEAN) -> CONTEXT {
    if !ScanContext.is_null() {
        if !owner.is_null() {
            *owner = BOOLEAN::FALSE;
        }
    } else if !owner.is_null() {
        *owner = BOOLEAN::TRUE;
        ScanContext = CreateScanContext(BOOLEAN::TRUE);
    } else {
        ScanContext = CreateScanContext(BOOLEAN::FALSE);
    }
    return ScanContext;
}
#[no_mangle]
pub unsafe extern "C" fn DestroyScanContext() {
    if !ScanContext.is_null() {
        DestroyContext(ScanContext);
        ScanContext = ::core::ptr::null_mut::<CONTEXT_DESC>();
    }
}
#[no_mangle]
pub unsafe extern "C" fn ScanSystem() {
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
    memset(
        &raw mut MenuState as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<MENU_STATE>(),
    );
    GetScanContext(::core::ptr::null_mut::<BOOLEAN>());
    if optWhichMenu == 0x1 as ::core::ffi::c_int
        && ((*(*pSolarSysState).pOrbitalDesc).data_index as ::core::ffi::c_int
            & C2Rust_Unnamed_10::PLANET_SHIELDED.0 as ::core::ffi::c_int != 0
            || (*pSolarSysState).SysInfo.PlanetInfo.AtmoDensity as ::core::ffi::c_int
                == 0xffff as ::core::ffi::c_int)
    {
        MenuState.CurState = ScanMenuItems::EXIT_SCAN.0 as ::core::ffi::c_int as BYTE;
    } else {
        MenuState.CurState = ScanMenuItems::AUTO_SCAN.0 as ::core::ffi::c_int as BYTE;
        planetLoc.x = ((if is3DO(optSuperPC).0 != 0 {
            243 as ::core::ffi::c_int
        } else {
            210 as ::core::ffi::c_int
        }
            - if if optWindowType == 1 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                33 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } << resolutionFactor >> 1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
            as COORD;
        planetLoc.y = (((67 as ::core::ffi::c_int) << resolutionFactor
            >> 1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as COORD;
        initPlanetLocationImage();
        SetContext(ScanContext);
        uqm_lua_checked!(DrawScannedObjects(BOOLEAN::FALSE), ());
    }
    uqm_lua_checked!(DrawMenuStateStrings(
        C2Rust_Unnamed_11::PM_MIN_SCAN.0 as ::core::ffi::c_int as BYTE,
        MenuState.CurState as SWORD,
    ), ());
    uqm_lua_checked!(SetFlashRect(
        ::core::ptr::with_exposed_provenance_mut::<RECT>(!(0 as ::core::ffi::c_long) as usize),
        BOOLEAN::FALSE,
    ), ());
    if optWhichCoarseScan & 1 as ::core::ffi::c_int != 0 {
        uqm_lua_checked!(PrintCoarseScan3DO(), ());
    } else {
        uqm_lua_checked!(PrintCoarseScanPC(), ());
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
        DoScan as unsafe extern "C" fn(*mut MENU_STATE) -> BOOLEAN,
    ) as Option<unsafe extern "C" fn(*mut menu_state) -> BOOLEAN>;
    uqm_lua_checked!(DoInput(&raw mut MenuState as *mut ::core::ffi::c_void, BOOLEAN::FALSE), ());
    uqm_lua_checked!(SetFlashRect(::core::ptr::null::<RECT>(), BOOLEAN::FALSE), ());
    if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
        & C2Rust_Unnamed_7::CHECK_ABORT.0 as ::core::ffi::c_int == 0
    {
        BatchGraphics();
        SetContext(ScanContext);
        uqm_lua_checked!(DrawPlanet(
            0 as ::core::ffi::c_int,
            colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE),
        ), ());
        uqm_lua_checked!(EraseCoarseScan(), ());
        UnbatchGraphics();
    }
    DestroyDrawable(ReleaseDrawable(eraseFrame));
    eraseFrame = ::core::ptr::null_mut::<FRAME_DESC>();
}
unsafe extern "C" fn generateBioNode(
    mut system: *mut SOLARSYS_STATE,
    mut NodeElementPtr: *mut ELEMENT,
    mut life_init_tab: *mut BYTE,
    mut creatureType: COUNT,
) {
    let mut i: COUNT = 0;
    let mut j: DWORD = 0;
    if if if GlobData.SIS_state.Difficulty as ::core::ffi::c_int != 0 {
        GlobData.SIS_state.Difficulty as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    } == 2 as ::core::ffi::c_int
    {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    } != 0
    {
        (*(&raw mut CreatureData as *mut LIFEFORM_DESC).offset(23isize)).Attributes = ((0
            as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            | (2 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (3 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as BYTE;
        (*(&raw mut CreatureData as *mut LIFEFORM_DESC).offset(23isize))
            .ValueAndHitPoints = ((5 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int)
            << 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int
            | 5 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE;
    } else if if if GlobData.SIS_state.Difficulty as ::core::ffi::c_int != 0 {
        GlobData.SIS_state.Difficulty as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    } == 1 as ::core::ffi::c_int
    {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    } != 0
    {
        (*(&raw mut CreatureData as *mut LIFEFORM_DESC)
            .offset((23 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize))
            .Attributes = ((0 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            | (0 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (3 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as BYTE;
        (*(&raw mut CreatureData as *mut LIFEFORM_DESC)
            .offset((23 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize))
            .ValueAndHitPoints = ((8 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int)
            << 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int
            | 15 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE;
    }
    if (*(&raw mut CreatureData as *mut LIFEFORM_DESC).offset(creatureType as isize))
        .Attributes as ::core::ffi::c_int & 0x30 as ::core::ffi::c_int != 0
    {
        i = TFB_Random() as COUNT;
        j = TFB_Random();
        (*NodeElementPtr).current.location.x = (if if resolutionFactor
            != 2 as ::core::ffi::c_uint
        {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } == 0
        {
            i as BYTE as ::core::ffi::c_int
        } else {
            j as UWORD as ::core::ffi::c_int
        }
            % ((if is3DO(optSuperPC).0 != 0 {
                243 as ::core::ffi::c_int
            } else {
                210 as ::core::ffi::c_int
            }
                - if if optWindowType == 1 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    33 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                } << resolutionFactor)
                - ((8 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int))
            + 8 as ::core::ffi::c_int) as COORD;
        (*NodeElementPtr).current.location.y = (if if resolutionFactor
            != 2 as ::core::ffi::c_uint
        {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } == 0
        {
            (i as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as BYTE
                as ::core::ffi::c_int
        } else {
            (j >> 16 as ::core::ffi::c_int) as UWORD as ::core::ffi::c_int
        }
            % (((67 as ::core::ffi::c_int) << resolutionFactor)
                - ((8 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int))
            + 8 as ::core::ffi::c_int) as COORD;
    }
    if (*system).PlanetSideFrame[0usize].is_null() {
        (*system).PlanetSideFrame[0usize] = CaptureDrawable(
            uqm_lua_checked!(LoadGraphicInstance(b"graphics.lifecan\0".as_ptr() as RESOURCE), ()) as DRAWABLE,
        );
    }
    i = 0 as COUNT;
    while (i as ::core::ffi::c_int) < 3 as ::core::ffi::c_int
        && *life_init_tab.offset(i as isize) as ::core::ffi::c_int
            != (creatureType as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as BYTE
                as ::core::ffi::c_int
    {
        if *life_init_tab.offset(i as isize) as ::core::ffi::c_int
            != 0 as ::core::ffi::c_int
        {
            i = i.wrapping_add(1);
        } else {
            *life_init_tab.offset(i as isize) = (creatureType as BYTE
                as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as BYTE;
            (*system)
                .PlanetSideFrame[(i as ::core::ffi::c_int + 3 as ::core::ffi::c_int)
                as usize] = uqm_lua_checked!(load_life_form(creatureType as BYTE), ());
            break;
        }
    }
    (*NodeElementPtr).c2rust_unnamed_1.mass_points = creatureType as BYTE;
    (*NodeElementPtr).c2rust_unnamed_0.hit_points = ((*(&raw mut CreatureData
        as *mut LIFEFORM_DESC)
        .offset(creatureType as isize))
        .ValueAndHitPoints as ::core::ffi::c_int
        >> 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE as COUNT;
    DisplayArray[(*NodeElementPtr).PrimIndex as usize].Object.Stamp.frame = SetAbsFrameIndex(
        (*system)
            .PlanetSideFrame[(i as ::core::ffi::c_int + 3 as ::core::ffi::c_int)
            as usize],
        TFB_Random() as COUNT,
    );
}
#[no_mangle]
pub unsafe extern "C" fn GeneratePlanetSide() {
    let mut scan: SIZE = 0;
    let mut life_init_tab: [BYTE; 3] = [0; 3];
    InitDisplayList();
    if (*(*pSolarSysState).pOrbitalDesc).data_index as ::core::ffi::c_int
        & C2Rust_Unnamed_10::PLANET_SHIELDED.0 as ::core::ffi::c_int != 0
    {
        return;
    }
    memset(
        &raw mut life_init_tab as *mut BYTE as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[BYTE; 3]>(),
    );
    scan = PlanetScanTypes::BIOLOGICAL_SCAN.0 as ::core::ffi::c_int as SIZE;
    while (scan as ::core::ffi::c_int)
        >= PlanetScanTypes::MINERAL_SCAN.0 as ::core::ffi::c_int
    {
        let mut num_nodes: COUNT = 0;
        let mut f: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
        f = SetAbsFrameIndex(
            MiscDataFrame,
            (4 as ::core::ffi::c_int
                * (scan as ::core::ffi::c_int
                    - PlanetScanTypes::ENERGY_SCAN.0 as ::core::ffi::c_int)) as COUNT,
        );
        num_nodes = uqm_lua_checked!(callGenerateForScanType(
            pSolarSysState,
            (*pSolarSysState).pOrbitalDesc,
            !(0 as ::core::ffi::c_int) as COUNT,
            scan as BYTE,
            ::core::ptr::null_mut::<NODE_INFO>(),
        ), ());
        loop {
            let c2rust_fresh0 = num_nodes;
            num_nodes = num_nodes.wrapping_sub(1);
            if c2rust_fresh0 == 0 {
                break;
            }
            let mut hNodeElement: HELEMENT = ::core::ptr::null_mut::<
                ::core::ffi::c_void,
            >();
            let mut NodeElementPtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
            let mut info: NODE_INFO = NODE_INFO {
                loc_pt: POINT { x: 0, y: 0 },
                density: 0,
                r#type: 0,
            };
            if isNodeRetrieved(
                &raw mut (*pSolarSysState).SysInfo.PlanetInfo,
                scan as BYTE,
                num_nodes as BYTE,
            ) {
                continue;
            }
            hNodeElement = AllocElement();
            if hNodeElement.is_null() {
                continue;
            }
            NodeElementPtr = LockLink(&raw mut disp_q, hNodeElement) as *mut ELEMENT;
            uqm_lua_checked!(callGenerateForScanType(
                pSolarSysState,
                (*pSolarSysState).pOrbitalDesc,
                num_nodes,
                scan as BYTE,
                &raw mut info,
            ), ());
            (*NodeElementPtr).c2rust_unnamed.scan_node = ((((num_nodes
                as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as BYTE
                as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as UWORD
                as ::core::ffi::c_int | scan as BYTE as ::core::ffi::c_int) as COUNT;
            (*NodeElementPtr).playerNr = C2Rust_Unnamed_6::PS_NON_PLAYER.0
                as ::core::ffi::c_int as SIZE;
            (*NodeElementPtr).current.location.x = info.loc_pt.x;
            (*NodeElementPtr).current.location.y = info.loc_pt.y;
            DisplayArray[(*NodeElementPtr).PrimIndex as usize].Type = gfx_object::STAMP_PRIM
                .0 as ::core::ffi::c_int as GRAPHICS_PRIM;
            if scan as ::core::ffi::c_int
                == PlanetScanTypes::MINERAL_SCAN.0 as ::core::ffi::c_int
            {
                (*NodeElementPtr).c2rust_unnamed_2.turn_wait = info.r#type as BYTE;
                (*NodeElementPtr).c2rust_unnamed_1.mass_points = ((info.density
                    as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as BYTE
                    as ::core::ffi::c_int
                    - (*pSolarSysState)
                        .SysInfo
                        .PlanetInfo
                        .PartiallyScavengedList[scan as usize][num_nodes as usize]
                        as ::core::ffi::c_int) as BYTE;
                (*NodeElementPtr).current.image.frame = SetAbsFrameIndex(
                    MiscDataFrame,
                    (4 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                        + (*Elements.offset(info.r#type as isize) as ::core::ffi::c_int
                            & 0x7 as ::core::ffi::c_int) * 5 as ::core::ffi::c_int)
                        as COUNT,
                );
                (*NodeElementPtr).next.image.frame = SetRelFrameIndex(
                    (*NodeElementPtr).current.image.frame,
                    (info.density as BYTE as ::core::ffi::c_int
                        + 1 as ::core::ffi::c_int) as SIZE,
                );
                DisplayArray[(*NodeElementPtr).PrimIndex as usize].Object.Stamp.frame = IncFrameIndex(
                    (*NodeElementPtr).next.image.frame,
                );
            } else {
                (*NodeElementPtr).current.image.frame = f;
                (*NodeElementPtr).next.image.frame = SetRelFrameIndex(
                    f,
                    (4 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as SIZE,
                );
                (*NodeElementPtr).c2rust_unnamed_2.turn_wait = ((4 as ::core::ffi::c_int
                    as BYTE as ::core::ffi::c_int)
                    << 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int
                    | 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE;
                (*NodeElementPtr).preprocess_func = Some(
                    object_animation as unsafe extern "C" fn(*mut ELEMENT) -> (),
                ) as Option<ElementProcessFunc>;
                if scan as ::core::ffi::c_int
                    == PlanetScanTypes::ENERGY_SCAN.0 as ::core::ffi::c_int
                {
                    (*NodeElementPtr).c2rust_unnamed_1.mass_points = (if isPC(optSuperPC)
                        .0 != 0
                    {
                        64 as ::core::ffi::c_int
                    } else {
                        50 as ::core::ffi::c_int
                    }) as BYTE;
                    DisplayArray[(*NodeElementPtr).PrimIndex as usize]
                        .Object
                        .Stamp
                        .frame = (*pSolarSysState).PlanetSideFrame[1usize];
                } else {
                    uqm_lua_checked!(generateBioNode(
                        pSolarSysState,
                        NodeElementPtr,
                        &raw mut life_init_tab as *mut BYTE,
                        info.r#type,
                    ), ());
                }
            }
            (*NodeElementPtr).next.location.x = (((*NodeElementPtr).current.location.x
                as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as COORD;
            (*NodeElementPtr).next.location.y = (((*NodeElementPtr).current.location.y
                as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as COORD;
            UnlockLink(&raw mut disp_q, hNodeElement);
            PutQueue(&raw mut disp_q, hNodeElement);
        }
        scan -= 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn isNodeRetrieved(
    mut planetInfo: *mut PLANET_INFO,
    mut scanType: BYTE,
    mut nodeNr: BYTE,
) -> bool {
    return (*planetInfo).ScanRetrieveMask[scanType as usize]
        & (1 as ::core::ffi::c_int as DWORD) << nodeNr as ::core::ffi::c_int
        != 0 as DWORD;
}
#[no_mangle]
pub unsafe extern "C" fn countNodesRetrieved(
    mut planetInfo: *mut PLANET_INFO,
    mut scanType: BYTE,
) -> COUNT {
    let mut count: COUNT = 0;
    let mut mask: DWORD = (*planetInfo).ScanRetrieveMask[scanType as usize];
    count = 0 as COUNT;
    while mask != 0 as DWORD {
        if mask & 1 as DWORD != 0 {
            count = count.wrapping_add(1);
        }
        mask >>= 1 as ::core::ffi::c_int;
    }
    return count;
}
#[no_mangle]
pub unsafe extern "C" fn setNodeRetrieved(
    mut planetInfo: *mut PLANET_INFO,
    mut scanType: BYTE,
    mut nodeNr: BYTE,
) {
    (*planetInfo).ScanRetrieveMask[scanType as usize]
        |= (1 as ::core::ffi::c_int as DWORD) << nodeNr as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn setNodeNotRetrieved(
    mut planetInfo: *mut PLANET_INFO,
    mut scanType: BYTE,
    mut nodeNr: BYTE,
) {
    (*planetInfo).ScanRetrieveMask[scanType as usize]
        &= !((1 as ::core::ffi::c_int as DWORD) << nodeNr as ::core::ffi::c_int);
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
