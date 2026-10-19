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

// Rust translation of src/uqm/setup.c.
extern "C" {
    pub type uio_DirHandle;
    pub type uio_Repository;
    pub type resource_index_desc;
    pub type string_table_entry;
    pub type string_table;
    pub type context_desc;
    pub type frame_desc;
    pub type font_desc;
    pub type drawable_desc;
    pub type tfb_soundsample;
    pub type getmelee_struct;
    fn __assert_fail(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    ) -> !;
    fn InitQueue(pq: *mut QUEUE, num_elements: COUNT, size: OBJ_SIZE) -> BOOLEAN;
    fn SetContext(Context: CONTEXT) -> CONTEXT;
    fn SetContextFGFrame(Frame: FRAME) -> FRAME;
    fn SetContextClipRect(pRect: *mut RECT) -> BOOLEAN;
    fn SetContextOrigin(orgOffset: POINT) -> POINT;
    fn CreateContextAux(name: *const ::core::ffi::c_char) -> CONTEXT;
    fn CreateDisplay(
        CreateFlags: CREATE_FLAGS,
        pwidth: *mut SIZE,
        pheight: *mut SIZE,
    ) -> DRAWABLE;
    fn LoadGraphicInstance(res: RESOURCE) -> *mut ::core::ffi::c_void;
    fn CaptureDrawable(Drawable: DRAWABLE) -> FRAME;
    fn SetColorMap(ColorMapPtr: COLORMAPPTR) -> BOOLEAN;
    fn GetColorMapAddress(_: COLORMAP) -> COLORMAPPTR;
    fn uio_init();
    fn uio_unInit();
    fn uio_openRepository(flags: ::core::ffi::c_int) -> *mut uio_Repository;
    fn uio_closeRepository(repository_0: *mut uio_Repository);
    fn uio_openDir(
        repository_0: *mut uio_Repository,
        path: *const ::core::ffi::c_char,
        flags: ::core::ffi::c_int,
    ) -> *mut uio_DirHandle;
    fn uio_closeDir(dirHandle: *mut uio_DirHandle) -> ::core::ffi::c_int;
    fn InitResourceSystem() -> RESOURCE_INDEX;
    fn InitSound(
        argc: ::core::ffi::c_int,
        argv: *mut *mut ::core::ffi::c_char,
    ) -> BOOLEAN;
    fn LoadSoundInstance(res: RESOURCE) -> SOUND_REF;
    fn LoadStringTableInstance(res: RESOURCE) -> STRING_TABLE;
    fn DestroyStringTable(StringTable: STRING_TABLE) -> BOOLEAN;
    fn CaptureStringTable(StringTable: STRING_TABLE) -> STRING;
    fn ReleaseStringTable(String: STRING) -> STRING_TABLE;
    fn abort() -> !;
    static mut optWhichIntro: ::core::ffi::c_int;
    static mut resolutionFactor: ::core::ffi::c_uint;
    static mut optSpaceMusic: ::core::ffi::c_int;
    static mut optVolasMusic: OPT_ENABLABLE;
    static mut optWindowType: DWORD;
    static mut optNoClassic: BOOLEAN;
    static mut opt3doMusic: OPT_ENABLABLE;
    static mut optRemixMusic: OPT_ENABLABLE;
    static mut optSpeech: OPT_ENABLABLE;
    static mut contentDir: *mut uio_DirHandle;
    static mut optAddons: *mut *const ::core::ffi::c_char;
    fn prepareAddons(addons: *mut *const ::core::ffi::c_char);
    fn loadAddon(addon: *const ::core::ffi::c_char) -> BOOLEAN;
    fn loadIndices(baseDir: *mut uio_DirHandle) -> ::core::ffi::c_int;
    fn isAddonAvailable(addon_name: *const ::core::ffi::c_char) -> BOOLEAN;
    static mut CanvasWidth: ::core::ffi::c_int;
    static mut CanvasHeight: ::core::ffi::c_int;
    static mut GlobData: GLOBDATA;
    fn InitSpace() -> BOOLEAN;
    fn ComputerInputContext_new(playerNr: COUNT) -> *mut ComputerInputContext;
    fn HumanInputContext_new(playerNr: COUNT) -> *mut HumanInputContext;
    static mut PlayerInput: [*mut InputContext; 0];
    static mut MenuSounds: SOUND;
    fn AdvanceLoadProgress();
    fn InitStatusOffsets();
    fn InitVideoPlayer(UseCDROM: BOOLEAN) -> BOOLEAN;
    fn log_add(_: log_Level, fmt: *const ::core::ffi::c_char, ...);
}
pub type uint8 = ::core::ffi::c_uchar;
pub type uint16 = ::core::ffi::c_ushort;
pub type sint16 = ::core::ffi::c_short;
pub type uint32 = ::core::ffi::c_uint;
pub type sint32 = ::core::ffi::c_int;
pub type BYTE = uint8;
pub type UBYTE = uint8;
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
pub type OBJ_SIZE = UWORD;
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
pub struct Color {
    pub r: BYTE,
    pub g: BYTE,
    pub b: BYTE,
    pub a: BYTE,
}
pub type RESOURCE_INDEX_DESC = resource_index_desc;
pub type RESOURCE_INDEX = *mut RESOURCE_INDEX_DESC;
pub type RESOURCE = *const ::core::ffi::c_char;
pub type STRING_TABLE_ENTRY_DESC = string_table_entry;
pub type STRING_TABLE_DESC = string_table;
pub type STRING_TABLE = *mut STRING_TABLE_DESC;
pub type STRING = *mut STRING_TABLE_ENTRY_DESC;
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
pub type COLORMAP = STRING;
pub type COLORMAPPTR = *mut ::core::ffi::c_void;
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
pub type SOUND_REF = STRING_TABLE;
pub type SOUND = STRING;
pub type TFB_SoundSample = tfb_soundsample;
pub type MUSIC_REF = *mut *mut TFB_SoundSample;
pub type BATTLE_INPUT_STATE = UBYTE;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct OPT_ENABLABLE(pub ::core::ffi::c_uint);
impl OPT_ENABLABLE {
    pub const OPTVAL_DISABLED: Self = Self(0);
    pub const OPTVAL_ENABLED: Self = Self(1);
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
pub type HELEMENT = HLINK;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct InputContext {
    pub handlers: *mut BattleInputHandlers,
    pub playerNr: COUNT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BattleInputHandlers {
    pub frameInput: BattleFrameInputFunction,
    pub selectShip: SelectShipFunction,
    pub battleEndReady: BattleEndReadyFunction,
    pub deleteContext: DeleteInputContextFunction,
}
pub type DeleteInputContextFunction = Option<
    unsafe extern "C" fn(*mut InputContext) -> (),
>;
pub type BattleEndReadyFunction = Option<
    unsafe extern "C" fn(*mut InputContext) -> bool,
>;
pub type SelectShipFunction = Option<
    unsafe extern "C" fn(*mut InputContext, *mut GETMELEE_STATE) -> BOOLEAN,
>;
pub type GETMELEE_STATE = getmelee_struct;
pub type BattleFrameInputFunction = Option<
    unsafe extern "C" fn(*mut InputContext, *mut STARSHIP) -> BATTLE_INPUT_STATE,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ComputerInputContext {
    pub handlers: *mut BattleInputHandlers,
    pub playerNr: COUNT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HumanInputContext {
    pub handlers: *mut BattleInputHandlers,
    pub playerNr: COUNT,
}
#[inline]
unsafe extern "C" fn MAKE_POINT(mut x: COORD, mut y: COORD) -> POINT {
    let mut pt: POINT = point { x: x, y: y };
    return pt;
}
#[no_mangle]
pub static mut LastActivity: ACTIVITY = 0;
#[no_mangle]
pub static mut PlayerControl: [BYTE; 2] = [0; 2];
#[no_mangle]
pub static mut hResIndex: RESOURCE_INDEX = ::core::ptr::null_mut::<
    RESOURCE_INDEX_DESC,
>();
#[no_mangle]
pub static mut ScreenContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
#[no_mangle]
pub static mut SpaceContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
#[no_mangle]
pub static mut StatusContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
#[no_mangle]
pub static mut OffScreenContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
#[no_mangle]
pub static mut screen_width: SIZE = 0;
#[no_mangle]
pub static mut screen_height: SIZE = 0;
#[no_mangle]
pub static mut Screen: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
#[no_mangle]
pub static mut StarConFont: FONT = ::core::ptr::null_mut::<FONT_DESC>();
#[no_mangle]
pub static mut MicroFont: FONT = ::core::ptr::null_mut::<FONT_DESC>();
#[no_mangle]
pub static mut TinyFont: FONT = ::core::ptr::null_mut::<FONT_DESC>();
#[no_mangle]
pub static mut TinyFontBold: FONT = ::core::ptr::null_mut::<FONT_DESC>();
#[no_mangle]
pub static mut TinyFontCond: FONT = ::core::ptr::null_mut::<FONT_DESC>();
#[no_mangle]
pub static mut PlyrFont: FONT = ::core::ptr::null_mut::<FONT_DESC>();
#[no_mangle]
pub static mut LabelFont: FONT = ::core::ptr::null_mut::<FONT_DESC>();
#[no_mangle]
pub static mut SlabFont: FONT = ::core::ptr::null_mut::<FONT_DESC>();
#[no_mangle]
pub static mut SquareFont: FONT = ::core::ptr::null_mut::<FONT_DESC>();
#[no_mangle]
pub static mut PlayMenuFont: FONT = ::core::ptr::null_mut::<FONT_DESC>();
#[no_mangle]
pub static mut race_q: [QUEUE; 2] = [QUEUE {
    head: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    tail: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    pq_tab: ::core::ptr::null_mut::<BYTE>(),
    free_list: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    object_size: 0,
    num_objects: 0,
}; 2];
#[no_mangle]
pub static mut ActivityFrame: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
#[no_mangle]
pub static mut StatusFrame: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
#[no_mangle]
pub static mut SubmenuFrame: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
#[no_mangle]
pub static mut FlagStatFrame: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
#[no_mangle]
pub static mut MiscDataFrame: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
#[no_mangle]
pub static mut visitedStarsFrame: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
#[no_mangle]
pub static mut FontGradFrame: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
#[no_mangle]
pub static mut BorderFrame: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
#[no_mangle]
pub static mut HDBorderFrame: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
#[no_mangle]
pub static mut CustBevelFrame: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
#[no_mangle]
pub static mut DefBevelFrame: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
#[no_mangle]
pub static mut GameStrings: STRING = ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>();
#[no_mangle]
pub static mut disp_q: QUEUE = QUEUE {
    head: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    tail: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    pq_tab: ::core::ptr::null_mut::<BYTE>(),
    free_list: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    object_size: 0,
    num_objects: 0,
};
#[no_mangle]
pub static mut solTexturesPresent: BOOLEAN = BOOLEAN::FALSE;
#[no_mangle]
pub static mut SyreenVoiceFix: BOOLEAN = BOOLEAN::FALSE;
#[no_mangle]
pub static mut EndlessSCLoaded: BOOLEAN = BOOLEAN::FALSE;
#[no_mangle]
pub static mut Rando: BYTE = 0;
#[no_mangle]
pub static mut HDPackPresent: BOOLEAN = BOOLEAN::FALSE;
#[no_mangle]
pub static mut VolasPackPresent: BOOLEAN = BOOLEAN::FALSE;
#[no_mangle]
pub static mut DeathBySurrender: BOOLEAN = BOOLEAN::FALSE;
#[no_mangle]
pub static mut DeathByMelee: BOOLEAN = BOOLEAN::FALSE;
#[no_mangle]
pub static mut DeathBySuicide: BOOLEAN = BOOLEAN::FALSE;
#[no_mangle]
pub static mut SpaceMusicOK: BOOLEAN = BOOLEAN::FALSE;
#[no_mangle]
pub static mut oldPlanetsPresent: BOOLEAN = BOOLEAN::FALSE;
#[no_mangle]
pub static mut classicPackPresent: BOOLEAN = BOOLEAN::FALSE;
#[no_mangle]
pub static mut repository: *mut uio_Repository = ::core::ptr::null_mut::<
    uio_Repository,
>();
#[no_mangle]
pub static mut rootDir: *mut uio_DirHandle = ::core::ptr::null_mut::<uio_DirHandle>();
#[no_mangle]
pub static mut usingSpeech: BOOLEAN = BOOLEAN::FALSE;
unsafe extern "C" fn InitPlayerInput() {}
#[no_mangle]
pub unsafe extern "C" fn UninitPlayerInput() {}
#[no_mangle]
pub unsafe extern "C" fn LoadKernel(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> BOOLEAN {
    InitSound(argc, argv);
    uqm_lua_checked!(InitVideoPlayer(BOOLEAN::TRUE), BOOLEAN(0));
    ScreenContext = CreateContextAux(
        b"ScreenContext\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if ScreenContext.is_null() {
        return BOOLEAN::FALSE;
    }
    Screen = CaptureDrawable(
        CreateDisplay(
            (((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as CREATE_FLAGS
                as ::core::ffi::c_int
                | ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as CREATE_FLAGS
                    as ::core::ffi::c_int) as CREATE_FLAGS,
            &raw mut screen_width,
            &raw mut screen_height,
        ),
    );
    if Screen.is_null() {
        return BOOLEAN::FALSE;
    }
    SetContext(ScreenContext);
    SetContextFGFrame(Screen);
    SetContextOrigin(MAKE_POINT(0 as COORD, 0 as COORD));
    hResIndex = InitResourceSystem();
    if hResIndex.is_null() {
        return BOOLEAN::FALSE;
    }
    if uqm_lua_checked!(loadIndices(contentDir), BOOLEAN(0)) == 0 as ::core::ffi::c_int {
        return BOOLEAN::FALSE;
    }
    classicPackPresent = BOOLEAN::FALSE;
    if if resolutionFactor != 2 as ::core::ffi::c_uint {
        BOOLEAN::FALSE.0 as ::core::ffi::c_int
    } else {
        BOOLEAN::TRUE.0 as ::core::ffi::c_int
    } == 0
    {
        EndlessSCLoaded = uqm_lua_checked!(loadAddon(
            b"EndlessSC-SD\0".as_ptr() as *const ::core::ffi::c_char,
        ), BOOLEAN(0));
        solTexturesPresent = uqm_lua_checked!(loadAddon(
            b"sol-textures-sd\0".as_ptr() as *const ::core::ffi::c_char,
        ), BOOLEAN(0));
        uqm_lua_checked!(loadAddon(b"yellow-fried-sd\0".as_ptr() as *const ::core::ffi::c_char), BOOLEAN(0));
        uqm_lua_checked!(loadAddon(b"PurpleQuanSD\0".as_ptr() as *const ::core::ffi::c_char), BOOLEAN(0));
    } else if uqm_lua_checked!(loadAddon(b"mm-hd\0".as_ptr() as *const ::core::ffi::c_char), BOOLEAN(0)).0 != 0 {
        HDPackPresent = BOOLEAN::TRUE;
        solTexturesPresent = uqm_lua_checked!(loadAddon(
            b"sol-textures-hd\0".as_ptr() as *const ::core::ffi::c_char,
        ), BOOLEAN(0));
        uqm_lua_checked!(loadAddon(b"yellow-fried-hd\0".as_ptr() as *const ::core::ffi::c_char), BOOLEAN(0));
        uqm_lua_checked!(loadAddon(b"PurpleQuanHD\0".as_ptr() as *const ::core::ffi::c_char), BOOLEAN(0));
        if optWindowType == 2 as DWORD {
            classicPackPresent = BOOLEAN(
                if optNoClassic.0 != 0 {
                    BOOLEAN::FALSE.0
                } else {
                    uqm_lua_checked!(loadAddon(b"classic-pack\0".as_ptr() as *const ::core::ffi::c_char), BOOLEAN(0)).0
                },
            );
        }
    }
    if if optWindowType == 1 as DWORD {
        BOOLEAN::TRUE.0 as ::core::ffi::c_int
    } else {
        BOOLEAN::FALSE.0 as ::core::ffi::c_int
    } != 0
        && isAddonAvailable(
                if if resolutionFactor != 2 as ::core::ffi::c_uint {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } != 0
                {
                    b"3do-mode-hd\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"3do-mode-sd\0".as_ptr() as *const ::core::ffi::c_char
                },
            )
            .0 != 0
    {
        uqm_lua_checked!(loadAddon(
            if if resolutionFactor != 2 as ::core::ffi::c_uint {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } != 0
            {
                b"3do-mode-hd\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"3do-mode-sd\0".as_ptr() as *const ::core::ffi::c_char
            },
        ), BOOLEAN(0));
    }
    if if optWindowType == 0 as DWORD {
        BOOLEAN::TRUE.0 as ::core::ffi::c_int
    } else {
        BOOLEAN::FALSE.0 as ::core::ffi::c_int
    } != 0
        && isAddonAvailable(
                if if resolutionFactor != 2 as ::core::ffi::c_uint {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } != 0
                {
                    b"dos-mode-hd\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"dos-mode-sd\0".as_ptr() as *const ::core::ffi::c_char
                },
            )
            .0 != 0
    {
        uqm_lua_checked!(loadAddon(
            if if resolutionFactor != 2 as ::core::ffi::c_uint {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } != 0
            {
                b"dos-mode-hd\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"dos-mode-sd\0".as_ptr() as *const ::core::ffi::c_char
            },
        ), BOOLEAN(0));
    }
    usingSpeech = BOOLEAN(optSpeech.0);
    if optSpeech.0 != 0
        && uqm_lua_checked!(loadAddon(b"mm-3dovoice\0".as_ptr() as *const ::core::ffi::c_char), BOOLEAN(0)).0 == 0
    {
        usingSpeech = BOOLEAN::FALSE;
    }
    if usingSpeech.0 != 0 {
        uqm_lua_checked!(loadAddon(b"mm-rmx-utwig\0".as_ptr() as *const ::core::ffi::c_char), BOOLEAN(0));
        uqm_lua_checked!(loadAddon(b"mm-MelnormeVoiceFix\0".as_ptr() as *const ::core::ffi::c_char), BOOLEAN(0));
        uqm_lua_checked!(loadAddon(b"mm-distorted-hayes\0".as_ptr() as *const ::core::ffi::c_char), BOOLEAN(0));
        SyreenVoiceFix = uqm_lua_checked!(loadAddon(
            b"mm-SyreenVoiceFix\0".as_ptr() as *const ::core::ffi::c_char,
        ), BOOLEAN(0));
    }
    if opt3doMusic.0 != 0 {
        uqm_lua_checked!(loadAddon(b"3domusic\0".as_ptr() as *const ::core::ffi::c_char), BOOLEAN(0));
    }
    if optRemixMusic.0 != 0 {
        uqm_lua_checked!(loadAddon(b"remix\0".as_ptr() as *const ::core::ffi::c_char), BOOLEAN(0));
    }
    if optVolasMusic.0 != 0 {
        VolasPackPresent = uqm_lua_checked!(loadAddon(
            b"volasaurus-remix-pack\0".as_ptr() as *const ::core::ffi::c_char,
        ), BOOLEAN(0));
        SpaceMusicOK = BOOLEAN(
            (optSpaceMusic != 0 && VolasPackPresent.0 != 0) as ::core::ffi::c_int
                as ::core::ffi::c_uint,
        );
    }
    if VolasPackPresent.0 == 0 {
        SpaceMusicOK = BOOLEAN(
            (optSpaceMusic != 0
                && uqm_lua_checked!(loadAddon(b"SpaceMusic\0".as_ptr() as *const ::core::ffi::c_char), BOOLEAN(0)).0
                    != 0) as ::core::ffi::c_int as ::core::ffi::c_uint,
        );
    }
    if optWhichIntro == 0x1 as ::core::ffi::c_int {
        uqm_lua_checked!(loadAddon(b"3dovideo\0".as_ptr() as *const ::core::ffi::c_char), BOOLEAN(0));
    }
    uqm_lua_checked!(loadAddon(b"ProfanePkunk\0".as_ptr() as *const ::core::ffi::c_char), BOOLEAN(0));
    uqm_lua_checked!(loadAddon(b"GlaDOS\0".as_ptr() as *const ::core::ffi::c_char), BOOLEAN(0));
    if if resolutionFactor != 2 as ::core::ffi::c_uint {
        BOOLEAN::FALSE.0 as ::core::ffi::c_int
    } else {
        BOOLEAN::TRUE.0 as ::core::ffi::c_int
    } == 0
    {
        uqm_lua_checked!(loadAddon(b"xlat-finnish-sd\0".as_ptr() as *const ::core::ffi::c_char), BOOLEAN(0));
        uqm_lua_checked!(loadAddon(b"xlat-german-sd\0".as_ptr() as *const ::core::ffi::c_char), BOOLEAN(0));
        uqm_lua_checked!(loadAddon(b"xlat-japanese-sd\0".as_ptr() as *const ::core::ffi::c_char), BOOLEAN(0));
        uqm_lua_checked!(loadAddon(b"xlat-russian-sd\0".as_ptr() as *const ::core::ffi::c_char), BOOLEAN(0));
        uqm_lua_checked!(loadAddon(b"automods-sd\0".as_ptr() as *const ::core::ffi::c_char), BOOLEAN(0));
    } else if HDPackPresent.0 != 0 {
        uqm_lua_checked!(loadAddon(b"xlat-finnish-hd\0".as_ptr() as *const ::core::ffi::c_char), BOOLEAN(0));
        uqm_lua_checked!(loadAddon(b"xlat-german-hd\0".as_ptr() as *const ::core::ffi::c_char), BOOLEAN(0));
        uqm_lua_checked!(loadAddon(b"xlat-japanese-hd\0".as_ptr() as *const ::core::ffi::c_char), BOOLEAN(0));
        uqm_lua_checked!(loadAddon(b"xlat-russian-hd\0".as_ptr() as *const ::core::ffi::c_char), BOOLEAN(0));
        uqm_lua_checked!(loadAddon(b"automods-hd\0".as_ptr() as *const ::core::ffi::c_char), BOOLEAN(0));
    }
    uqm_lua_checked!(prepareAddons(optAddons), BOOLEAN(0));
    let mut ColorMapTab: COLORMAP = ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>();
    ColorMapTab = CaptureStringTable(
        uqm_lua_checked!(LoadStringTableInstance(b"colortable.main\0".as_ptr() as RESOURCE), BOOLEAN(0)),
    ) as COLORMAP;
    if ColorMapTab.is_null() {
        return BOOLEAN::FALSE;
    }
    SetColorMap(GetColorMapAddress(ColorMapTab));
    DestroyStringTable(ReleaseStringTable(ColorMapTab));
    InitPlayerInput();
    GlobData.Game_state.CurrentActivity = !(0 as ::core::ffi::c_int) as ACTIVITY;
    return BOOLEAN::TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn InitContexts() -> BOOLEAN {
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut oldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    StatusContext = CreateContextAux(
        b"StatusContext\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if StatusContext.is_null() {
        return BOOLEAN::FALSE;
    }
    uqm_lua_checked!(AdvanceLoadProgress(), BOOLEAN(0));
    oldContext = SetContext(StatusContext);
    SetContextFGFrame(Screen);
    r.corner.x = (CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
        + (if if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            16 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        } << resolutionFactor)) as COORD;
    r.corner.y = (if if optWindowType == 1 as DWORD {
        BOOLEAN::TRUE.0 as ::core::ffi::c_int
    } else {
        BOOLEAN::FALSE.0 as ::core::ffi::c_int
    } != 0
    {
        16 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    } << resolutionFactor) as COORD;
    r.extent.width = ((64 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    r.extent.height = CanvasHeight as COORD;
    SetContextClipRect(&raw mut r);
    SpaceContext = CreateContextAux(
        b"SpaceContext\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if SpaceContext.is_null() {
        return BOOLEAN::FALSE;
    }
    SetContext(oldContext);
    uqm_lua_checked!(AdvanceLoadProgress(), BOOLEAN(0));
    OffScreenContext = CreateContextAux(
        b"OffScreenContext\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if OffScreenContext.is_null() {
        return BOOLEAN::FALSE;
    }
    uqm_lua_checked!(AdvanceLoadProgress(), BOOLEAN(0));
    if InitQueue(
            &raw mut disp_q,
            150 as COUNT,
            ::core::mem::size_of::<ELEMENT>() as OBJ_SIZE,
        )
        .0 == 0
    {
        return BOOLEAN::FALSE;
    }
    uqm_lua_checked!(AdvanceLoadProgress(), BOOLEAN(0));
    return BOOLEAN::TRUE;
}
unsafe extern "C" fn InitKernel() -> BOOLEAN {
    let mut counter: COUNT = 0;
    counter = 0 as COUNT;
    while (counter as ::core::ffi::c_int) < 2 as ::core::ffi::c_int {
        InitQueue(
            (&raw mut race_q as *mut QUEUE).offset(counter as isize),
            14 as COUNT,
            ::core::mem::size_of::<STARSHIP>() as OBJ_SIZE,
        );
        counter = counter.wrapping_add(1);
    }
    StarConFont = uqm_lua_checked!(LoadGraphicInstance(b"font.starcon\0".as_ptr() as RESOURCE), BOOLEAN(0)) as FONT;
    if StarConFont.is_null() {
        return BOOLEAN::FALSE;
    }
    uqm_lua_checked!(AdvanceLoadProgress(), BOOLEAN(0));
    TinyFont = uqm_lua_checked!(LoadGraphicInstance(b"font.tiny\0".as_ptr() as RESOURCE), BOOLEAN(0)) as FONT;
    if TinyFont.is_null() {
        return BOOLEAN::FALSE;
    }
    uqm_lua_checked!(AdvanceLoadProgress(), BOOLEAN(0));
    TinyFontBold = uqm_lua_checked!(LoadGraphicInstance(b"font.tiny.bold\0".as_ptr() as RESOURCE), BOOLEAN(0)) as FONT;
    if TinyFontBold.is_null() {
        return BOOLEAN::FALSE;
    }
    uqm_lua_checked!(AdvanceLoadProgress(), BOOLEAN(0));
    TinyFontCond = uqm_lua_checked!(LoadGraphicInstance(b"font.tiny.condensed\0".as_ptr() as RESOURCE), BOOLEAN(0))
        as FONT;
    if TinyFontCond.is_null() {
        return BOOLEAN::FALSE;
    }
    PlyrFont = uqm_lua_checked!(LoadGraphicInstance(b"font.player\0".as_ptr() as RESOURCE), BOOLEAN(0)) as FONT;
    if PlyrFont.is_null() {
        return BOOLEAN::FALSE;
    }
    uqm_lua_checked!(AdvanceLoadProgress(), BOOLEAN(0));
    PlayMenuFont = uqm_lua_checked!(LoadGraphicInstance(b"font.playmenu\0".as_ptr() as RESOURCE), BOOLEAN(0)) as FONT;
    if PlayMenuFont.is_null() {
        return BOOLEAN::FALSE;
    }
    BorderFrame = CaptureDrawable(
        uqm_lua_checked!(LoadGraphicInstance(b"graphics.border\0".as_ptr() as RESOURCE), BOOLEAN(0)) as DRAWABLE,
    );
    if BorderFrame.is_null() {
        return BOOLEAN::FALSE;
    }
    uqm_lua_checked!(AdvanceLoadProgress(), BOOLEAN(0));
    CustBevelFrame = CaptureDrawable(
        uqm_lua_checked!(LoadGraphicInstance(b"graphics.cust.bevel\0".as_ptr() as RESOURCE), BOOLEAN(0)) as DRAWABLE,
    );
    if CustBevelFrame.is_null() {
        return BOOLEAN::FALSE;
    }
    if HDPackPresent.0 != 0 {
        HDBorderFrame = CaptureDrawable(
            uqm_lua_checked!(LoadGraphicInstance(b"graphics.hdborder\0".as_ptr() as RESOURCE), BOOLEAN(0)) as DRAWABLE,
        );
        if HDBorderFrame.is_null() {
            return BOOLEAN::FALSE;
        }
        DefBevelFrame = CaptureDrawable(
            uqm_lua_checked!(LoadGraphicInstance(b"graphics.def.bevel\0".as_ptr() as RESOURCE), BOOLEAN(0)) as DRAWABLE,
        );
        if DefBevelFrame.is_null() {
            return BOOLEAN::FALSE;
        }
    }
    ActivityFrame = CaptureDrawable(
        uqm_lua_checked!(LoadGraphicInstance(b"graphics.activity\0".as_ptr() as RESOURCE), BOOLEAN(0)) as DRAWABLE,
    );
    if ActivityFrame.is_null() {
        return BOOLEAN::FALSE;
    }
    uqm_lua_checked!(AdvanceLoadProgress(), BOOLEAN(0));
    StatusFrame = CaptureDrawable(
        uqm_lua_checked!(LoadGraphicInstance(b"graphics.status\0".as_ptr() as RESOURCE), BOOLEAN(0)) as DRAWABLE,
    );
    if StatusFrame.is_null() {
        return BOOLEAN::FALSE;
    }
    uqm_lua_checked!(AdvanceLoadProgress(), BOOLEAN(0));
    uqm_lua_checked!(AdvanceLoadProgress(), BOOLEAN(0));
    GameStrings = CaptureStringTable(
        uqm_lua_checked!(LoadStringTableInstance(b"text.starcon\0".as_ptr() as RESOURCE), BOOLEAN(0)),
    );
    if GameStrings.is_null() {
        return BOOLEAN::FALSE;
    }
    uqm_lua_checked!(AdvanceLoadProgress(), BOOLEAN(0));
    MicroFont = uqm_lua_checked!(LoadGraphicInstance(b"font.micro\0".as_ptr() as RESOURCE), BOOLEAN(0)) as FONT;
    if MicroFont.is_null() {
        return BOOLEAN::FALSE;
    }
    uqm_lua_checked!(AdvanceLoadProgress(), BOOLEAN(0));
    LabelFont = uqm_lua_checked!(LoadGraphicInstance(b"font.label\0".as_ptr() as RESOURCE), BOOLEAN(0)) as FONT;
    if LabelFont.is_null() {
        return BOOLEAN::FALSE;
    }
    SquareFont = uqm_lua_checked!(LoadGraphicInstance(b"font.square\0".as_ptr() as RESOURCE), BOOLEAN(0)) as FONT;
    if SquareFont.is_null() {
        return BOOLEAN::FALSE;
    }
    SlabFont = uqm_lua_checked!(LoadGraphicInstance(b"font.slab\0".as_ptr() as RESOURCE), BOOLEAN(0)) as FONT;
    if SlabFont.is_null() {
        return BOOLEAN::FALSE;
    }
    MenuSounds = CaptureStringTable(
        uqm_lua_checked!(LoadSoundInstance(b"sounds.menu\0".as_ptr() as RESOURCE), BOOLEAN(0)),
    ) as SOUND;
    if MenuSounds.is_null() {
        return BOOLEAN::FALSE;
    }
    uqm_lua_checked!(AdvanceLoadProgress(), BOOLEAN(0));
    InitStatusOffsets();
    uqm_lua_checked!(InitSpace(), BOOLEAN(0));
    uqm_lua_checked!(AdvanceLoadProgress(), BOOLEAN(0));
    return BOOLEAN::TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn InitGameKernel() -> BOOLEAN {
    if ActivityFrame.is_null() {
        uqm_lua_checked!(InitKernel(), BOOLEAN(0));
        uqm_lua_checked!(InitContexts(), BOOLEAN(0));
    }
    return BOOLEAN::TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn SetPlayerInput(mut playerI: COUNT) -> bool {
    ((*(&raw mut PlayerInput as *mut *mut InputContext).offset(playerI as isize))
        .is_null()
        || {
            __assert_fail(
                b"PlayerInput[playerI] == NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/uqm/setup.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                399 as ::core::ffi::c_int,
                b"SetPlayerInput\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    match PlayerControl[playerI as usize] as ::core::ffi::c_int
        & (((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as BYTE
            as ::core::ffi::c_int
            | (((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as BYTE
                as ::core::ffi::c_int
                | ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as BYTE
                    as ::core::ffi::c_int)
            | ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as BYTE
                as ::core::ffi::c_int)
    {
        1 => {
            *(&raw mut PlayerInput as *mut *mut InputContext).offset(playerI as isize) = HumanInputContext_new(
                playerI,
            ) as *mut InputContext;
        }
        6 | 2 => {
            *(&raw mut PlayerInput as *mut *mut InputContext).offset(playerI as isize) = ComputerInputContext_new(
                playerI,
            ) as *mut InputContext;
        }
        _ => {
            log_add(
                log_Level::log_Fatal,
                b"Invalid control method in SetPlayerInput().\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            explode();
        }
    }
    return !(*(&raw mut PlayerInput as *mut *mut InputContext).offset(playerI as isize))
        .is_null();
}
#[no_mangle]
pub unsafe extern "C" fn SetPlayerInputAll() -> bool {
    let mut playerI: COUNT = 0;
    playerI = 0 as COUNT;
    while (playerI as ::core::ffi::c_int) < 2 as ::core::ffi::c_int {
        if !SetPlayerInput(playerI) {
            return false;
        }
        playerI = playerI.wrapping_add(1);
    }
    return true;
}
#[no_mangle]
pub unsafe extern "C" fn ClearPlayerInput(mut playerI: COUNT) {
    if (*(&raw mut PlayerInput as *mut *mut InputContext).offset(playerI as isize))
        .is_null()
    {
        log_add(
            log_Level::log_Debug,
            b"ClearPlayerInput(): PlayerInput[%d] was NULL.\0".as_ptr()
                as *const ::core::ffi::c_char,
            playerI as ::core::ffi::c_int,
        );
        return;
    }
    uqm_lua_checked!((*(**(&raw mut PlayerInput as *mut *mut InputContext).offset(playerI as isize))
        .handlers)
        .deleteContext
        .expect(
            "non-null function pointer",
        )(*(&raw mut PlayerInput as *mut *mut InputContext).offset(playerI as isize)), ());
    *(&raw mut PlayerInput as *mut *mut InputContext).offset(playerI as isize) = ::core::ptr::null_mut::<
        InputContext,
    >();
}
#[no_mangle]
pub unsafe extern "C" fn ClearPlayerInputAll() {
    let mut playerI: COUNT = 0;
    playerI = 0 as COUNT;
    while (playerI as ::core::ffi::c_int) < 2 as ::core::ffi::c_int {
        uqm_lua_checked!(ClearPlayerInput(playerI), ());
        playerI = playerI.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn initIO() -> ::core::ffi::c_int {
    uqm_lua_checked!(uio_init(), 0);
    repository = uio_openRepository(0 as ::core::ffi::c_int);
    rootDir = uqm_lua_checked!(uio_openDir(
        repository,
        b"/\0".as_ptr() as *const ::core::ffi::c_char,
        0 as ::core::ffi::c_int,
    ), 0);
    if rootDir.is_null() {
        log_add(
            log_Level::log_Fatal,
            b"Could not open '/' dir.\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn uninitIO() {
    uio_closeDir(rootDir);
    uqm_lua_checked!(uio_closeRepository(repository), ());
    uqm_lua_checked!(uio_unInit(), ());
}
#[inline]
unsafe extern "C" fn explode() -> ! {
    abort();
}
