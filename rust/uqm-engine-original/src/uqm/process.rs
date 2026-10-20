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

// Rust translation of src/uqm/process.c.
extern "C" {
    pub type string_table_entry;
    pub type context_desc;
    pub type tfb_soundsample;
    fn SetContext(Context: CONTEXT) -> CONTEXT;
    fn DrawablesIntersect(
        pControl0: *mut INTERSECT_CONTROL,
        pControl1: *mut INTERSECT_CONTROL,
        max_time_val: TIME_VALUE,
    ) -> TIME_VALUE;
    fn DrawBatch(
        pBasePrim: *mut PRIMITIVE,
        PrimLinks: PRIM_LINKS,
        BatchFlags: BATCH_FLAGS,
    );
    fn ClearDrawable();
    fn GetFrameHot(Frame: FRAME) -> HOT_SPOT;
    fn GetFrameIndex(Frame: FRAME) -> COUNT;
    fn SetEquFrameIndex(DstFrame: FRAME, SrcFrame: FRAME) -> FRAME;
    fn __assert_fail(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    );
    fn TFB_DrawScreen_SetMipmap(
        img: *mut TFB_Image,
        mmimg: *mut TFB_Image,
        hotx: ::core::ffi::c_int,
        hoty: ::core::ffi::c_int,
    );
    fn AllocLink(pq: *mut QUEUE) -> HLINK;
    fn FreeLink(pq: *mut QUEUE, hLink: HLINK);
    fn ReinitQueue(pq: *mut QUEUE);
    fn RemoveQueue(pq: *mut QUEUE, hLink: HLINK);
    static mut ScreenWidth: ::core::ffi::c_int;
    static mut ScreenHeight: ::core::ffi::c_int;
    fn GetNextVelocityComponents(
        velocityptr: *mut VELOCITY_DESC,
        pdx: *mut SIZE,
        pdy: *mut SIZE,
        num_frames: COUNT,
    );
    static mut disp_q: QUEUE;
    fn do_damage(ElementPtr: *mut ELEMENT, damage: SIZE);
    fn MoveGalaxy(view_state: VIEW_STATE, dx: SIZE, dy: SIZE);
    static mut SpaceOrg: POINT;
    static mut optMeleeScale: ::core::ffi::c_int;
    static mut optStereoSFX: BOOLEAN;
    fn collide(ElementPtr0: *mut ELEMENT, ElementPtr1: *mut ELEMENT);
    static mut GlobData: GLOBDATA;
    fn inHQSpace() -> BOOLEAN;
    static mut SpaceContext: CONTEXT;
    static mut StatusContext: CONTEXT;
    fn ProcessSound(Sound: SOUND, PositionalObject: *mut ELEMENT);
    fn UpdateSoundPositions();
    fn FlushSounds();
    fn RemoveSoundsForObject(PosObj: *mut ELEMENT);
    fn MoveSIS(pdx: *mut SIZE, pdy: *mut SIZE);
    static mut battle_counter: [BYTE; 2];
    fn abort() -> !;
    fn SetGraphicScale(scale: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn log_add(_: log_Level, fmt: *const ::core::ffi::c_char, ...);
    fn memset(
        _: *mut ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
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
pub type RESOURCE = *const ::core::ffi::c_char;
pub type STRING_TABLE_ENTRY_DESC = string_table_entry;
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
pub type CONTEXT = *mut CONTEXT_DESC;
pub type FRAME = *mut FRAME_DESC;
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
pub type BATCH_FLAGS = BYTE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct INTERSECT_CONTROL {
    pub last_time_val: TIME_VALUE,
    pub EndPoint: POINT,
    pub IntersectStamp: STAMP,
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
pub struct velocity_desc {
    pub TravelAngle: COUNT,
    pub vector: EXTENT,
    pub fract: EXTENT,
    pub error: EXTENT,
    pub incr: EXTENT,
}
pub type VELOCITY_DESC = velocity_desc;
pub type HELEMENT = HLINK;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct VIEW_STATE(pub ::core::ffi::c_uint);
impl VIEW_STATE {
    pub const VIEW_STABLE: Self = Self(0);
    pub const VIEW_SCROLL: Self = Self(1);
    pub const VIEW_CHANGE: Self = Self(2);
}
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
pub type SOUND = STRING;
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
pub struct C2Rust_Unnamed_5(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_5 {
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
pub struct C2Rust_Unnamed_6(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_6 {
    pub const TFB_SCALE_STEP: Self = Self(0);
    pub const TFB_SCALE_NEAREST: Self = Self(1);
    pub const TFB_SCALE_BILINEAR: Self = Self(2);
    pub const TFB_SCALE_TRILINEAR: Self = Self(3);
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
#[no_mangle]
pub static mut DisplayFreeList: COUNT = 0;
#[no_mangle]
pub static mut DisplayArray: [PRIMITIVE; 330] = [PRIMITIVE {
    Links: 0,
    Type: 0,
    color: Color { r: 0, g: 0, b: 0, a: 0 },
    Object: PRIM_DESC {
        Point: POINT { x: 0, y: 0 },
    },
}; 330];
#[no_mangle]
pub static mut zoom_out: SIZE = ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
    as SIZE;
static mut opt_max_zoom_out: SIZE = 0;
#[inline]
unsafe extern "C" fn CALC_ZOOM_STUFF(mut idx: *mut COUNT, mut sc: *mut COUNT) {
    let mut i: ::core::ffi::c_int = 0;
    if (zoom_out as ::core::ffi::c_int)
        < (2 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
    {
        i = 0 as ::core::ffi::c_int;
    } else if (zoom_out as ::core::ffi::c_int)
        < (4 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
    {
        i = 1 as ::core::ffi::c_int;
    } else {
        i = 2 as ::core::ffi::c_int;
    }
    *idx = i as COUNT;
    *sc = (((1 as ::core::ffi::c_int)
        << i + 8 as ::core::ffi::c_int + 8 as ::core::ffi::c_int)
        / zoom_out as ::core::ffi::c_int) as COUNT;
}
#[no_mangle]
pub unsafe extern "C" fn AllocElement() -> HELEMENT {
    let mut hElement: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
    hElement = AllocLink(&raw mut disp_q) as HELEMENT;
    if !hElement.is_null() {
        let mut ElementPtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
        ElementPtr = LockLink(&raw mut disp_q, hElement) as *mut ELEMENT;
        memset(
            ElementPtr as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<ELEMENT>(),
        );
        (*ElementPtr).PrimIndex = DisplayFreeList;
        DisplayFreeList = (DisplayArray[DisplayFreeList as usize].Links
            >> 16 as ::core::ffi::c_int) as UWORD as COUNT;
        if (*ElementPtr).PrimIndex as ::core::ffi::c_int
            == 0xffff as ::core::ffi::c_int as COUNT as ::core::ffi::c_int
        {
            log_add(
                log_Level::log_Error,
                b"AllocElement: Out of display prims!\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            explode();
        }
        DisplayArray[(*ElementPtr).PrimIndex as usize].Type = gfx_object::NUM_PRIMS.0
            as ::core::ffi::c_int as GRAPHICS_PRIM;
        UnlockLink(&raw mut disp_q, hElement);
    }
    return hElement;
}
#[no_mangle]
pub unsafe extern "C" fn FreeElement(mut hElement: HELEMENT) {
    if !hElement.is_null() {
        let mut ElementPtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
        ElementPtr = LockLink(&raw mut disp_q, hElement) as *mut ELEMENT;
        DisplayArray[(*ElementPtr).PrimIndex as usize].Links = ((DisplayFreeList
            as DWORD) << 16 as ::core::ffi::c_int
            | 0xffff as ::core::ffi::c_int as COUNT as DWORD) as PRIM_LINKS;
        DisplayFreeList = (*ElementPtr).PrimIndex;
        UnlockLink(&raw mut disp_q, hElement);
        FreeLink(&raw mut disp_q, hElement);
    }
}
#[no_mangle]
pub unsafe extern "C" fn SetUpElement(mut ElementPtr: *mut ELEMENT) {
    (*ElementPtr).next = (*ElementPtr).current;
    if (*ElementPtr).state_flags as ::core::ffi::c_int
        & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) == 0
    {
        (*ElementPtr).IntersectControl.IntersectStamp.origin.x = ((*ElementPtr)
            .current
            .location
            .x as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as COORD;
        (*ElementPtr).IntersectControl.IntersectStamp.origin.y = ((*ElementPtr)
            .current
            .location
            .y as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as COORD;
        (*ElementPtr).IntersectControl.EndPoint.x = ((*ElementPtr).next.location.x
            as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as COORD;
        (*ElementPtr).IntersectControl.EndPoint.y = ((*ElementPtr).next.location.y
            as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as COORD;
        (*ElementPtr).IntersectControl.IntersectStamp.frame = SetEquFrameIndex(
            *(*ElementPtr).next.image.farray.offset(0isize),
            (*ElementPtr).next.image.frame,
        );
    }
}
unsafe extern "C" fn PreProcess(mut ElementPtr: *mut ELEMENT) {
    let mut state_flags: ELEMENT_FLAGS = 0;
    if (*ElementPtr).c2rust_unnamed.life_span as ::core::ffi::c_int
        == 0 as ::core::ffi::c_int
    {
        if !(*ElementPtr).pParent.is_null() {
            Untarget(ElementPtr);
        }
        (*ElementPtr).state_flags = ((*ElementPtr).state_flags as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as ELEMENT_FLAGS;
        if (*ElementPtr).death_func.is_some() {
            Some((*ElementPtr).death_func.expect("non-null function pointer"))
                .expect("non-null function pointer")(ElementPtr);
        }
    }
    state_flags = (*ElementPtr).state_flags;
    if state_flags as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int == 0
    {
        if state_flags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int != 0
        {
            SetUpElement(ElementPtr);
            if state_flags as ::core::ffi::c_int
                & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int != 0
            {
                state_flags = (state_flags as ::core::ffi::c_int
                    & !((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int))
                    as ELEMENT_FLAGS;
            }
        }
        if (*ElementPtr).preprocess_func.is_some()
            && state_flags as ::core::ffi::c_int
                & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int == 0
        {
            Some((*ElementPtr).preprocess_func.expect("non-null function pointer"))
                .expect("non-null function pointer")(ElementPtr);
            state_flags = (*ElementPtr).state_flags;
            if state_flags as ::core::ffi::c_int
                & (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int != 0
                && (*ElementPtr).state_flags as ::core::ffi::c_int
                    & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                        | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) == 0
            {
                (*ElementPtr).IntersectControl.IntersectStamp.frame = SetEquFrameIndex(
                    *(*ElementPtr).next.image.farray.offset(0isize),
                    (*ElementPtr).next.image.frame,
                );
            }
        }
        if state_flags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int == 0
        {
            let mut delta_x: SIZE = 0;
            let mut delta_y: SIZE = 0;
            GetNextVelocityComponents(
                &raw mut (*ElementPtr).velocity,
                &raw mut delta_x,
                &raw mut delta_y,
                1 as COUNT,
            );
            if delta_x as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                || delta_y as ::core::ffi::c_int != 0 as ::core::ffi::c_int
            {
                state_flags = (state_flags as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int)
                    as ELEMENT_FLAGS;
                (*ElementPtr).next.location.x = ((*ElementPtr).next.location.x
                    as ::core::ffi::c_int + delta_x as ::core::ffi::c_int) as COORD;
                (*ElementPtr).next.location.y = ((*ElementPtr).next.location.y
                    as ::core::ffi::c_int + delta_y as ::core::ffi::c_int) as COORD;
            }
        }
        if (*ElementPtr).state_flags as ::core::ffi::c_int
            & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) == 0
        {
            (*ElementPtr).IntersectControl.EndPoint.x = ((*ElementPtr).next.location.x
                as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as COORD;
            (*ElementPtr).IntersectControl.EndPoint.y = ((*ElementPtr).next.location.y
                as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as COORD;
        }
        if state_flags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int != 0
        {
            (*ElementPtr).c2rust_unnamed.life_span = (*ElementPtr)
                .c2rust_unnamed
                .life_span
                .wrapping_sub(1);
        }
    }
    (*ElementPtr).state_flags = (state_flags as ::core::ffi::c_int
        & !((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int)
        | (1 as ::core::ffi::c_int) << 11 as ::core::ffi::c_int) as ELEMENT_FLAGS;
}
unsafe extern "C" fn PostProcess(mut ElementPtr: *mut ELEMENT) {
    if (*ElementPtr).postprocess_func.is_some() {
        Some((*ElementPtr).postprocess_func.expect("non-null function pointer"))
            .expect("non-null function pointer")(ElementPtr);
    }
    (*ElementPtr).current = (*ElementPtr).next;
    if (*ElementPtr).state_flags as ::core::ffi::c_int
        & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) == 0
    {
        (*ElementPtr).IntersectControl.IntersectStamp.origin.x = ((*ElementPtr)
            .current
            .location
            .x as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as COORD;
        (*ElementPtr).IntersectControl.IntersectStamp.origin.y = ((*ElementPtr)
            .current
            .location
            .y as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as COORD;
        (*ElementPtr).IntersectControl.EndPoint.x = ((*ElementPtr).next.location.x
            as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as COORD;
        (*ElementPtr).IntersectControl.EndPoint.y = ((*ElementPtr).next.location.y
            as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as COORD;
    }
    (*ElementPtr).state_flags = ((*ElementPtr).state_flags as ::core::ffi::c_int
        & !((1 as ::core::ffi::c_int) << 11 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
        | (1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int) as ELEMENT_FLAGS;
}
unsafe extern "C" fn CalcReduction(mut dx: SIZE, mut dy: SIZE) -> COUNT {
    let mut next_reduction: COUNT = 0;
    if optMeleeScale == C2Rust_Unnamed_6::TFB_SCALE_STEP.0 as ::core::ffi::c_int {
        let mut sdx: SIZE = 0;
        let mut sdy: SIZE = 0;
        if GlobData.Game_state.CurrentActivity as BYTE as ::core::ffi::c_int
            > C2Rust_Unnamed_5::IN_ENCOUNTER.0 as ::core::ffi::c_int
        {
            return 0 as COUNT;
        }
        sdx = dx;
        sdy = dy;
        next_reduction = 2 as COUNT;
        loop {
            dx = ((dx as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as SIZE;
            if !(dx as ::core::ffi::c_int
                <= ((ScreenWidth - 64 as ::core::ffi::c_int
                    - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                    << 2 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
                && {
                    dy = ((dy as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as SIZE;
                    dy as ::core::ffi::c_int
                        <= ((ScreenHeight
                            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                            << 2 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
                } && next_reduction as ::core::ffi::c_int > 0 as ::core::ffi::c_int)
            {
                break;
            }
            next_reduction = (next_reduction as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT;
        }
        if (next_reduction as ::core::ffi::c_int) < zoom_out as ::core::ffi::c_int
            && zoom_out as ::core::ffi::c_int <= 2 as ::core::ffi::c_int
        {
            if sdx as ::core::ffi::c_int
                + ((24 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                << 2 as ::core::ffi::c_int - next_reduction as ::core::ffi::c_int
                > ((ScreenWidth - 64 as ::core::ffi::c_int
                    - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                    << 2 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
                || sdy as ::core::ffi::c_int
                    + ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                    << 2 as ::core::ffi::c_int - next_reduction as ::core::ffi::c_int
                    > ((ScreenHeight - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                        << 2 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
            {
                next_reduction = (next_reduction as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int) as COUNT;
            }
        }
        if next_reduction as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            && GlobData.Game_state.CurrentActivity as BYTE as ::core::ffi::c_int
                == C2Rust_Unnamed_5::IN_LAST_BATTLE.0 as ::core::ffi::c_int
        {
            next_reduction = (next_reduction as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int) as COUNT;
        }
    } else {
        if GlobData.Game_state.CurrentActivity as BYTE as ::core::ffi::c_int
            > C2Rust_Unnamed_5::IN_ENCOUNTER.0 as ::core::ffi::c_int
        {
            return ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as COUNT;
        }
        dx = (dx as ::core::ffi::c_int
            * ((1 as ::core::ffi::c_int)
                << 8 as ::core::ffi::c_int + 3 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int)
            / (((ScreenWidth - 64 as ::core::ffi::c_int
                - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                >> 2 as ::core::ffi::c_int)) as SIZE;
        if (dx as ::core::ffi::c_int)
            < (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
        {
            dx = ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as SIZE;
        } else if dx as ::core::ffi::c_int
            > (1 as ::core::ffi::c_int)
                << 8 as ::core::ffi::c_int + 3 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int
        {
            dx = ((1 as ::core::ffi::c_int)
                << 8 as ::core::ffi::c_int + 3 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as SIZE;
        }
        dy = (dy as ::core::ffi::c_int
            * ((1 as ::core::ffi::c_int)
                << 8 as ::core::ffi::c_int + 3 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int)
            / (((ScreenHeight - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                >> 2 as ::core::ffi::c_int)) as SIZE;
        if (dy as ::core::ffi::c_int)
            < (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
        {
            dy = ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as SIZE;
        } else if dy as ::core::ffi::c_int
            > (1 as ::core::ffi::c_int)
                << 8 as ::core::ffi::c_int + 3 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int
        {
            dy = ((1 as ::core::ffi::c_int)
                << 8 as ::core::ffi::c_int + 3 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as SIZE;
        }
        if dy as ::core::ffi::c_int > dx as ::core::ffi::c_int {
            next_reduction = dy as COUNT;
        } else {
            next_reduction = dx as COUNT;
        }
        if (next_reduction as ::core::ffi::c_int)
            < (2 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
            && GlobData.Game_state.CurrentActivity as BYTE as ::core::ffi::c_int
                == C2Rust_Unnamed_5::IN_LAST_BATTLE.0 as ::core::ffi::c_int
        {
            next_reduction = ((2 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
                as COUNT;
        }
    }
    return next_reduction;
}
unsafe extern "C" fn CalcView(
    mut pNewScrollPt: *mut POINT,
    mut next_reduction: SIZE,
    mut pdx: *mut SIZE,
    mut pdy: *mut SIZE,
    mut ships_alive: COUNT,
) -> VIEW_STATE {
    let mut dx: SIZE = 0;
    let mut dy: SIZE = 0;
    let mut view_state: VIEW_STATE = VIEW_STATE::VIEW_STABLE;
    dx = ((((ScreenWidth - 64 as ::core::ffi::c_int
        - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
        << 3 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as COORD
        as ::core::ffi::c_int - (*pNewScrollPt).x as ::core::ffi::c_int) as SIZE;
    dy = ((((ScreenHeight - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
        << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
        >> 1 as ::core::ffi::c_int) as COORD as ::core::ffi::c_int
        - (*pNewScrollPt).y as ::core::ffi::c_int) as SIZE;
    dx = (if (dx as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
        if -(dx as ::core::ffi::c_int)
            <= ((ScreenWidth - 64 as ::core::ffi::c_int
                - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int
        {
            dx as ::core::ffi::c_int
        } else {
            (((ScreenWidth - 64 as ::core::ffi::c_int
                - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                + dx as ::core::ffi::c_int
        }
    } else if dx as ::core::ffi::c_int
        <= ((ScreenWidth - 64 as ::core::ffi::c_int
            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
            << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
            >> 1 as ::core::ffi::c_int
    {
        dx as ::core::ffi::c_int
    } else {
        dx as ::core::ffi::c_int
            - (((ScreenWidth - 64 as ::core::ffi::c_int
                - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
    }) as SIZE;
    dy = (if (dy as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
        if -(dy as ::core::ffi::c_int)
            <= ((ScreenHeight - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int
        {
            dy as ::core::ffi::c_int
        } else {
            (((ScreenHeight - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                + dy as ::core::ffi::c_int
        }
    } else if dy as ::core::ffi::c_int
        <= ((ScreenHeight - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
            << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
            >> 1 as ::core::ffi::c_int
    {
        dy as ::core::ffi::c_int
    } else {
        dy as ::core::ffi::c_int
            - (((ScreenHeight - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
    }) as SIZE;
    if ships_alive as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
        if dx as ::core::ffi::c_int
            > (((((ScreenWidth - 64 as ::core::ffi::c_int
                - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                / 75 as ::core::ffi::c_int) as COORD as ::core::ffi::c_int
                & !(((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int)) as SIZE as ::core::ffi::c_int
        {
            dx = (((((ScreenWidth - 64 as ::core::ffi::c_int
                - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                / 75 as ::core::ffi::c_int) as COORD as ::core::ffi::c_int
                & !(((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int)) as SIZE;
        } else if (dx as ::core::ffi::c_int)
            < -((((((ScreenWidth - 64 as ::core::ffi::c_int
                - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                / 75 as ::core::ffi::c_int) as COORD as ::core::ffi::c_int
                & !(((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int)) as SIZE as ::core::ffi::c_int)
        {
            dx = -((((((ScreenWidth - 64 as ::core::ffi::c_int
                - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                / 75 as ::core::ffi::c_int) as COORD as ::core::ffi::c_int
                & !(((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int)) as SIZE as ::core::ffi::c_int) as SIZE;
        }
        if dy as ::core::ffi::c_int
            > (((((ScreenHeight - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                / 75 as ::core::ffi::c_int) as COORD as ::core::ffi::c_int
                & !(((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int)) as SIZE as ::core::ffi::c_int
        {
            dy = (((((ScreenHeight - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                / 75 as ::core::ffi::c_int) as COORD as ::core::ffi::c_int
                & !(((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int)) as SIZE;
        } else if (dy as ::core::ffi::c_int)
            < -((((((ScreenHeight - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                / 75 as ::core::ffi::c_int) as COORD as ::core::ffi::c_int
                & !(((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int)) as SIZE as ::core::ffi::c_int)
        {
            dy = -((((((ScreenHeight - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                / 75 as ::core::ffi::c_int) as COORD as ::core::ffi::c_int
                & !(((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int)) as SIZE as ::core::ffi::c_int) as SIZE;
        }
    }
    if (dx as ::core::ffi::c_int != 0 || dy as ::core::ffi::c_int != 0)
        && inHQSpace().0 != 0
    {
        MoveSIS(&raw mut dx, &raw mut dy);
    }
    if zoom_out as ::core::ffi::c_int == next_reduction as ::core::ffi::c_int {
        view_state = VIEW_STATE(
            (if dx as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                && dy as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                && inHQSpace().0 == 0
            {
                VIEW_STATE::VIEW_STABLE.0 as ::core::ffi::c_int
            } else {
                VIEW_STATE::VIEW_SCROLL.0 as ::core::ffi::c_int
            }) as ::core::ffi::c_uint,
        );
    } else {
        if optMeleeScale == C2Rust_Unnamed_6::TFB_SCALE_STEP.0 as ::core::ffi::c_int {
            SpaceOrg.x = ((((ScreenWidth - 64 as ::core::ffi::c_int
                - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int) as COORD as ::core::ffi::c_int
                - (((ScreenWidth - 64 as ::core::ffi::c_int
                    - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                    << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    >> 3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                        - next_reduction as ::core::ffi::c_int)) as COORD;
            SpaceOrg.y = ((((ScreenHeight
                - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int) as COORD as ::core::ffi::c_int
                - (((ScreenHeight - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                    << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    >> 3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                        - next_reduction as ::core::ffi::c_int)) as COORD;
        } else {
            if ships_alive as ::core::ffi::c_int == 1 as ::core::ffi::c_int
                && zoom_out as ::core::ffi::c_int > next_reduction as ::core::ffi::c_int
                && zoom_out as ::core::ffi::c_int
                    <= (1 as ::core::ffi::c_int)
                        << 8 as ::core::ffi::c_int + 3 as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int
                && zoom_out as ::core::ffi::c_int - next_reduction as ::core::ffi::c_int
                    > (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
                        >> 3 as ::core::ffi::c_int
            {
                next_reduction = (zoom_out as ::core::ffi::c_int
                    - ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
                        >> 3 as ::core::ffi::c_int)) as SIZE;
            }
            SpaceOrg.x = (((((ScreenWidth - 64 as ::core::ffi::c_int
                - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int)
                - (((ScreenWidth - 64 as ::core::ffi::c_int
                    - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                    << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                    * next_reduction as ::core::ffi::c_int
                    / (((1 as ::core::ffi::c_int)
                        << 8 as ::core::ffi::c_int + 3 as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int))
                as COORD as ::core::ffi::c_int
                & !(((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int)) as COORD;
            SpaceOrg.y = (((((ScreenHeight
                - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int)
                - (((ScreenHeight - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                    << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                    * next_reduction as ::core::ffi::c_int
                    / (((1 as ::core::ffi::c_int)
                        << 8 as ::core::ffi::c_int + 3 as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int))
                as COORD as ::core::ffi::c_int
                & !(((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int)) as COORD;
        }
        zoom_out = next_reduction;
        view_state = VIEW_STATE::VIEW_CHANGE;
    }
    if GlobData.Game_state.CurrentActivity as BYTE as ::core::ffi::c_int
        <= C2Rust_Unnamed_5::IN_HYPERSPACE.0 as ::core::ffi::c_int
    {
        MoveGalaxy(view_state, dx, dy);
    }
    *pdx = dx;
    *pdy = dy;
    return view_state;
}
unsafe extern "C" fn ProcessCollisions(
    mut hSuccElement: HELEMENT,
    mut ElementPtr: *mut ELEMENT,
    mut min_time: TIME_VALUE,
    mut process_flags: ELEMENT_FLAGS,
) -> ELEMENT_FLAGS {
    let mut hTestElement: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
    loop {
        hTestElement = hSuccElement;
        if hTestElement.is_null() {
            break;
        }
        let mut TestElementPtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
        TestElementPtr = LockLink(&raw mut disp_q, hTestElement) as *mut ELEMENT;
        if (*TestElementPtr).state_flags as ::core::ffi::c_int
            & process_flags as ::core::ffi::c_int == 0
        {
            PreProcess(TestElementPtr);
        }
        hSuccElement = (*TestElementPtr).succ;
        if TestElementPtr == ElementPtr {
            UnlockLink(&raw mut disp_q, hTestElement);
        } else {
            if (*TestElementPtr).state_flags as ::core::ffi::c_int
                & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) == 0
                && ((*ElementPtr).state_flags as ::core::ffi::c_int
                    & (*TestElementPtr).state_flags as ::core::ffi::c_int
                    & (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int == 0
                    && ((*ElementPtr).state_flags as ::core::ffi::c_int
                        & (*TestElementPtr).state_flags as ::core::ffi::c_int
                        & (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int == 0
                        || (*ElementPtr).pParent != (*TestElementPtr).pParent)
                    && ((*ElementPtr).c2rust_unnamed_1.mass_points as ::core::ffi::c_int
                        != 0
                        || (*TestElementPtr).c2rust_unnamed_1.mass_points
                            as ::core::ffi::c_int != 0))
            {
                let mut state_flags: ELEMENT_FLAGS = 0;
                let mut test_state_flags: ELEMENT_FLAGS = 0;
                let mut time_val: TIME_VALUE = 0;
                state_flags = (*ElementPtr).state_flags;
                test_state_flags = (*TestElementPtr).state_flags;
                if (state_flags as ::core::ffi::c_int
                    | test_state_flags as ::core::ffi::c_int)
                    & (1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int != 0
                    && (state_flags as ::core::ffi::c_int
                        & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int != 0
                        && (*ElementPtr).c2rust_unnamed.life_span as ::core::ffi::c_int
                            > 1 as ::core::ffi::c_int
                        || test_state_flags as ::core::ffi::c_int
                            & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int != 0
                            && (*TestElementPtr).c2rust_unnamed.life_span
                                as ::core::ffi::c_int > 1 as ::core::ffi::c_int)
                {
                    time_val = 0 as TIME_VALUE;
                } else {
                    loop {
                        time_val = DrawablesIntersect(
                            &raw mut (*ElementPtr).IntersectControl,
                            &raw mut (*TestElementPtr).IntersectControl,
                            min_time,
                        );
                        if !(time_val as ::core::ffi::c_int == 1 as ::core::ffi::c_int
                            && (state_flags as ::core::ffi::c_int
                                | test_state_flags as ::core::ffi::c_int)
                                & (1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int
                                == 0)
                        {
                            break;
                        }
                        if state_flags as ::core::ffi::c_int
                            & (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int != 0
                        {
                            (*TestElementPtr).IntersectControl.EndPoint.x = ((*TestElementPtr)
                                .next
                                .location
                                .x as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
                                as COORD;
                            (*TestElementPtr).IntersectControl.EndPoint.y = ((*TestElementPtr)
                                .next
                                .location
                                .y as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
                                as COORD;
                            (*TestElementPtr).IntersectControl.IntersectStamp.origin = (*TestElementPtr)
                                .IntersectControl
                                .EndPoint;
                            time_val = DrawablesIntersect(
                                &raw mut (*ElementPtr).IntersectControl,
                                &raw mut (*TestElementPtr).IntersectControl,
                                1 as TIME_VALUE,
                            );
                            (*TestElementPtr).IntersectControl.IntersectStamp.origin.x = ((*TestElementPtr)
                                .current
                                .location
                                .x as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
                                as COORD;
                            (*TestElementPtr).IntersectControl.IntersectStamp.origin.y = ((*TestElementPtr)
                                .current
                                .location
                                .y as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
                                as COORD;
                        }
                        if time_val as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
                            let mut CurFrame: FRAME = ::core::ptr::null_mut::<
                                FRAME_DESC,
                            >();
                            let mut NextFrame: FRAME = ::core::ptr::null_mut::<
                                FRAME_DESC,
                            >();
                            let mut TestCurFrame: FRAME = ::core::ptr::null_mut::<
                                FRAME_DESC,
                            >();
                            let mut TestNextFrame: FRAME = ::core::ptr::null_mut::<
                                FRAME_DESC,
                            >();
                            CurFrame = (*ElementPtr).current.image.frame;
                            NextFrame = (*ElementPtr).next.image.frame;
                            TestCurFrame = (*TestElementPtr).current.image.frame;
                            TestNextFrame = (*TestElementPtr).next.image.frame;
                            if NextFrame == CurFrame && TestNextFrame == TestCurFrame {
                                if test_state_flags as ::core::ffi::c_int
                                    & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int != 0
                                {
                                    do_damage(
                                        TestElementPtr,
                                        (*TestElementPtr).c2rust_unnamed_0.hit_points as SIZE,
                                    );
                                    if !(*TestElementPtr).pParent.is_null() {
                                        Untarget(TestElementPtr);
                                    }
                                    (*TestElementPtr).state_flags = ((*TestElementPtr)
                                        .state_flags as ::core::ffi::c_int
                                        | ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int
                                            | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int))
                                        as ELEMENT_FLAGS;
                                    if (*TestElementPtr).death_func.is_some() {
                                        Some(
                                                (*TestElementPtr)
                                                    .death_func
                                                    .expect("non-null function pointer"),
                                            )
                                            .expect("non-null function pointer")(TestElementPtr);
                                    }
                                }
                                if state_flags as ::core::ffi::c_int
                                    & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int != 0
                                {
                                    do_damage(
                                        ElementPtr,
                                        (*ElementPtr).c2rust_unnamed_0.hit_points as SIZE,
                                    );
                                    if !(*ElementPtr).pParent.is_null() {
                                        Untarget(ElementPtr);
                                    }
                                    (*ElementPtr).state_flags = ((*ElementPtr).state_flags
                                        as ::core::ffi::c_int
                                        | ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int
                                            | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int))
                                        as ELEMENT_FLAGS;
                                    if (*ElementPtr).death_func.is_some() {
                                        Some(
                                                (*ElementPtr).death_func.expect("non-null function pointer"),
                                            )
                                            .expect("non-null function pointer")(ElementPtr);
                                    }
                                    UnlockLink(&raw mut disp_q, hTestElement);
                                    return ((1 as ::core::ffi::c_int)
                                        << 7 as ::core::ffi::c_int) as ELEMENT_FLAGS;
                                }
                                time_val = 0 as TIME_VALUE;
                            } else {
                                if GetFrameIndex(CurFrame) as ::core::ffi::c_int
                                    != GetFrameIndex(NextFrame) as ::core::ffi::c_int
                                {
                                    (*ElementPtr).next.image.frame = SetEquFrameIndex(
                                        NextFrame,
                                        CurFrame,
                                    );
                                } else if NextFrame != CurFrame {
                                    (*ElementPtr).next.image = (*ElementPtr).current.image;
                                    if (*ElementPtr).c2rust_unnamed.life_span
                                        as ::core::ffi::c_int > 1 as ::core::ffi::c_int
                                    {
                                        (*ElementPtr).c2rust_unnamed.life_span = 1 as COUNT;
                                    }
                                }
                                if GetFrameIndex(TestCurFrame) as ::core::ffi::c_int
                                    != GetFrameIndex(TestNextFrame) as ::core::ffi::c_int
                                {
                                    (*TestElementPtr).next.image.frame = SetEquFrameIndex(
                                        TestNextFrame,
                                        TestCurFrame,
                                    );
                                } else if TestNextFrame != TestCurFrame {
                                    (*TestElementPtr).next.image = (*TestElementPtr)
                                        .current
                                        .image;
                                    if (*TestElementPtr).c2rust_unnamed.life_span
                                        as ::core::ffi::c_int > 1 as ::core::ffi::c_int
                                    {
                                        (*TestElementPtr).c2rust_unnamed.life_span = 1 as COUNT;
                                    }
                                }
                                (*ElementPtr).IntersectControl.IntersectStamp.origin.x = ((*ElementPtr)
                                    .current
                                    .location
                                    .x as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
                                    as COORD;
                                (*ElementPtr).IntersectControl.IntersectStamp.origin.y = ((*ElementPtr)
                                    .current
                                    .location
                                    .y as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
                                    as COORD;
                                (*ElementPtr).IntersectControl.EndPoint.x = ((*ElementPtr)
                                    .next
                                    .location
                                    .x as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
                                    as COORD;
                                (*ElementPtr).IntersectControl.EndPoint.y = ((*ElementPtr)
                                    .next
                                    .location
                                    .y as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
                                    as COORD;
                                (*ElementPtr).IntersectControl.IntersectStamp.frame = SetEquFrameIndex(
                                    *(*ElementPtr).next.image.farray.offset(0isize),
                                    (*ElementPtr).next.image.frame,
                                );
                                if state_flags as ::core::ffi::c_int
                                    & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int != 0
                                {
                                    let mut StarShipPtr: *mut STARSHIP = ::core::ptr::null_mut::<
                                        STARSHIP,
                                    >();
                                    StarShipPtr = (*ElementPtr).pParent as *mut STARSHIP;
                                    (*StarShipPtr).ShipFacing = GetFrameIndex(
                                        (*ElementPtr).next.image.frame,
                                    );
                                }
                                (*TestElementPtr)
                                    .IntersectControl
                                    .IntersectStamp
                                    .origin
                                    .x = ((*TestElementPtr).current.location.x
                                    as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as COORD;
                                (*TestElementPtr)
                                    .IntersectControl
                                    .IntersectStamp
                                    .origin
                                    .y = ((*TestElementPtr).current.location.y
                                    as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as COORD;
                                (*TestElementPtr).IntersectControl.EndPoint.x = ((*TestElementPtr)
                                    .next
                                    .location
                                    .x as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
                                    as COORD;
                                (*TestElementPtr).IntersectControl.EndPoint.y = ((*TestElementPtr)
                                    .next
                                    .location
                                    .y as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
                                    as COORD;
                                (*TestElementPtr).IntersectControl.IntersectStamp.frame = SetEquFrameIndex(
                                    *(*TestElementPtr).next.image.farray.offset(0isize),
                                    (*TestElementPtr).next.image.frame,
                                );
                                if test_state_flags as ::core::ffi::c_int
                                    & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int != 0
                                {
                                    let mut StarShipPtr_0: *mut STARSHIP = ::core::ptr::null_mut::<
                                        STARSHIP,
                                    >();
                                    StarShipPtr_0 = (*TestElementPtr).pParent as *mut STARSHIP;
                                    (*StarShipPtr_0).ShipFacing = GetFrameIndex(
                                        (*TestElementPtr).next.image.frame,
                                    );
                                }
                            }
                        }
                        if time_val as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                            continue;
                        }
                        (*ElementPtr).IntersectControl.EndPoint.x = ((*ElementPtr)
                            .next
                            .location
                            .x as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
                            as COORD;
                        (*ElementPtr).IntersectControl.EndPoint.y = ((*ElementPtr)
                            .next
                            .location
                            .y as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
                            as COORD;
                        (*TestElementPtr).IntersectControl.EndPoint.x = ((*TestElementPtr)
                            .next
                            .location
                            .x as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
                            as COORD;
                        (*TestElementPtr).IntersectControl.EndPoint.y = ((*TestElementPtr)
                            .next
                            .location
                            .y as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
                            as COORD;
                        break;
                    }
                }
                if time_val as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                    let mut SavePt: POINT = POINT { x: 0, y: 0 };
                    let mut TestSavePt: POINT = POINT { x: 0, y: 0 };
                    SavePt = (*ElementPtr).IntersectControl.EndPoint;
                    TestSavePt = (*TestElementPtr).IntersectControl.EndPoint;
                    (*ElementPtr).IntersectControl.EndPoint.x = ((*ElementPtr)
                        .next
                        .location
                        .x as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as COORD;
                    (*ElementPtr).IntersectControl.EndPoint.y = ((*ElementPtr)
                        .next
                        .location
                        .y as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as COORD;
                    (*TestElementPtr).IntersectControl.EndPoint.x = ((*TestElementPtr)
                        .next
                        .location
                        .x as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as COORD;
                    (*TestElementPtr).IntersectControl.EndPoint.y = ((*TestElementPtr)
                        .next
                        .location
                        .y as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as COORD;
                    if time_val as ::core::ffi::c_int == 1 as ::core::ffi::c_int
                        || (state_flags as ::core::ffi::c_int
                            & (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int != 0
                            || ProcessCollisions(
                                hSuccElement,
                                ElementPtr,
                                (time_val as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                                    as TIME_VALUE,
                                process_flags,
                            ) == 0)
                            && (test_state_flags as ::core::ffi::c_int
                                & (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int != 0
                                || ProcessCollisions(
                                    if (*TestElementPtr).state_flags as ::core::ffi::c_int
                                        & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int == 0
                                    {
                                        (*ElementPtr).succ
                                    } else {
                                        disp_q.head
                                    },
                                    TestElementPtr,
                                    (time_val as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                                        as TIME_VALUE,
                                    process_flags,
                                ) == 0)
                    {
                        state_flags = (*ElementPtr).state_flags;
                        test_state_flags = (*TestElementPtr).state_flags;
                        if test_state_flags as ::core::ffi::c_int
                            & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int != 0
                        {
                            Some(
                                    (*TestElementPtr)
                                        .collision_func
                                        .expect("non-null function pointer"),
                                )
                                .expect(
                                    "non-null function pointer",
                                )(
                                TestElementPtr,
                                &raw mut TestSavePt,
                                ElementPtr,
                                &raw mut SavePt,
                            );
                            Some(
                                    (*ElementPtr)
                                        .collision_func
                                        .expect("non-null function pointer"),
                                )
                                .expect(
                                    "non-null function pointer",
                                )(
                                ElementPtr,
                                &raw mut SavePt,
                                TestElementPtr,
                                &raw mut TestSavePt,
                            );
                        } else {
                            Some(
                                    (*ElementPtr)
                                        .collision_func
                                        .expect("non-null function pointer"),
                                )
                                .expect(
                                    "non-null function pointer",
                                )(
                                ElementPtr,
                                &raw mut SavePt,
                                TestElementPtr,
                                &raw mut TestSavePt,
                            );
                            Some(
                                    (*TestElementPtr)
                                        .collision_func
                                        .expect("non-null function pointer"),
                                )
                                .expect(
                                    "non-null function pointer",
                                )(
                                TestElementPtr,
                                &raw mut TestSavePt,
                                ElementPtr,
                                &raw mut SavePt,
                            );
                        }
                        if (*TestElementPtr).state_flags as ::core::ffi::c_int
                            & (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int != 0
                        {
                            if test_state_flags as ::core::ffi::c_int
                                & (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int == 0
                            {
                                (*TestElementPtr).IntersectControl.IntersectStamp.origin = TestSavePt;
                                (*TestElementPtr).next.location.x = ((TestSavePt.x
                                    as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as COORD;
                                (*TestElementPtr).next.location.y = ((TestSavePt.y
                                    as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as COORD;
                                (*TestElementPtr).IntersectControl.EndPoint.x = ((*TestElementPtr)
                                    .next
                                    .location
                                    .x as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
                                    as COORD;
                                (*TestElementPtr).IntersectControl.EndPoint.y = ((*TestElementPtr)
                                    .next
                                    .location
                                    .y as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
                                    as COORD;
                            }
                        }
                        if (*ElementPtr).state_flags as ::core::ffi::c_int
                            & (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int != 0
                        {
                            if state_flags as ::core::ffi::c_int
                                & (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int == 0
                            {
                                (*ElementPtr).IntersectControl.IntersectStamp.origin = SavePt;
                                (*ElementPtr).next.location.x = ((SavePt.x
                                    as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as COORD;
                                (*ElementPtr).next.location.y = ((SavePt.y
                                    as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as COORD;
                                (*ElementPtr).IntersectControl.EndPoint.x = ((*ElementPtr)
                                    .next
                                    .location
                                    .x as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
                                    as COORD;
                                (*ElementPtr).IntersectControl.EndPoint.y = ((*ElementPtr)
                                    .next
                                    .location
                                    .y as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
                                    as COORD;
                                if state_flags as ::core::ffi::c_int
                                    & (1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int == 0
                                    && test_state_flags as ::core::ffi::c_int
                                        & (1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int == 0
                                {
                                    collide(ElementPtr, TestElementPtr);
                                    ProcessCollisions(
                                        disp_q.head,
                                        ElementPtr,
                                        (((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
                                            + 1 as ::core::ffi::c_int) as TIME_VALUE,
                                        process_flags,
                                    );
                                    ProcessCollisions(
                                        disp_q.head,
                                        TestElementPtr,
                                        (((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
                                            + 1 as ::core::ffi::c_int) as TIME_VALUE,
                                        process_flags,
                                    );
                                }
                            }
                            UnlockLink(&raw mut disp_q, hTestElement);
                            return ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int)
                                as ELEMENT_FLAGS;
                        }
                        if (*ElementPtr).state_flags as ::core::ffi::c_int
                            & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                                | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) != 0
                        {
                            (*ElementPtr).state_flags = ((*ElementPtr).state_flags
                                as ::core::ffi::c_int
                                | (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int)
                                as ELEMENT_FLAGS;
                            UnlockLink(&raw mut disp_q, hTestElement);
                            return ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int)
                                as ELEMENT_FLAGS;
                        }
                    }
                }
            }
            UnlockLink(&raw mut disp_q, hTestElement);
        }
    }
    return ((*ElementPtr).state_flags as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as ELEMENT_FLAGS;
}
unsafe extern "C" fn PreProcessQueue(
    mut pscroll_x: *mut SIZE,
    mut pscroll_y: *mut SIZE,
) -> VIEW_STATE {
    let mut min_reduction: SIZE = 0;
    let mut max_reduction: SIZE = 0;
    let mut sides_active: COUNT = 0;
    let mut Origin: POINT = POINT { x: 0, y: 0 };
    let mut hElement: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut ships_alive: COUNT = 0;
    sides_active = (if battle_counter[0usize] as ::core::ffi::c_int != 0 {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    }
        + if battle_counter[1usize] as ::core::ffi::c_int != 0 {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as COUNT;
    if optMeleeScale == C2Rust_Unnamed_6::TFB_SCALE_STEP.0 as ::core::ffi::c_int {
        max_reduction = (2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as SIZE;
        min_reduction = max_reduction;
    } else {
        max_reduction = (((1 as ::core::ffi::c_int)
            << 8 as ::core::ffi::c_int + 3 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int)
            + ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)) as SIZE;
        min_reduction = max_reduction;
    }
    Origin.x = (((ScreenWidth - 64 as ::core::ffi::c_int
        - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
        << 3 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as COORD;
    Origin.y = (((ScreenHeight - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
        << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
        >> 1 as ::core::ffi::c_int) as COORD;
    hElement = disp_q.head as HELEMENT;
    ships_alive = 0 as COUNT;
    while !hElement.is_null() {
        let mut ElementPtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
        let mut hNextElement: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
        ElementPtr = LockLink(&raw mut disp_q, hElement) as *mut ELEMENT;
        if (*ElementPtr).state_flags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 11 as ::core::ffi::c_int == 0
        {
            PreProcess(ElementPtr);
        }
        hNextElement = (*ElementPtr).succ;
        if (*ElementPtr).state_flags as ::core::ffi::c_int
            & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) == 0
            && (*ElementPtr).state_flags as ::core::ffi::c_int
                & (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int == 0
        {
            ProcessCollisions(
                hNextElement,
                ElementPtr,
                (((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
                    + 1 as ::core::ffi::c_int) as TIME_VALUE,
                ((1 as ::core::ffi::c_int) << 11 as ::core::ffi::c_int) as ELEMENT_FLAGS,
            );
        }
        if (*ElementPtr).state_flags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int != 0
        {
            let mut dx: SIZE = 0;
            let mut dy: SIZE = 0;
            ships_alive = ships_alive.wrapping_add(1);
            if max_reduction as ::core::ffi::c_int
                > opt_max_zoom_out as ::core::ffi::c_int
                && min_reduction as ::core::ffi::c_int
                    > opt_max_zoom_out as ::core::ffi::c_int
            {
                Origin.x = ((*ElementPtr).next.location.x as ::core::ffi::c_int
                    & !(((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int)) as COORD;
                Origin.y = ((*ElementPtr).next.location.y as ::core::ffi::c_int
                    & !(((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int)) as COORD;
            }
            dx = (((*ElementPtr).next.location.x as ::core::ffi::c_int
                & !(((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int)) - Origin.x as ::core::ffi::c_int)
                as SIZE;
            dx = (if (dx as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                if -(dx as ::core::ffi::c_int)
                    <= ((ScreenWidth - 64 as ::core::ffi::c_int
                        - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                        << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                        >> 1 as ::core::ffi::c_int
                {
                    dx as ::core::ffi::c_int
                } else {
                    (((ScreenWidth - 64 as ::core::ffi::c_int
                        - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                        << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                        + dx as ::core::ffi::c_int
                }
            } else if dx as ::core::ffi::c_int
                <= ((ScreenWidth - 64 as ::core::ffi::c_int
                    - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                    << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    >> 1 as ::core::ffi::c_int
            {
                dx as ::core::ffi::c_int
            } else {
                dx as ::core::ffi::c_int
                    - (((ScreenWidth - 64 as ::core::ffi::c_int
                        - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                        << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
            }) as SIZE;
            dy = (((*ElementPtr).next.location.y as ::core::ffi::c_int
                & !(((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int)) - Origin.y as ::core::ffi::c_int)
                as SIZE;
            dy = (if (dy as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                if -(dy as ::core::ffi::c_int)
                    <= ((ScreenHeight
                        - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                        << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                        >> 1 as ::core::ffi::c_int
                {
                    dy as ::core::ffi::c_int
                } else {
                    (((ScreenHeight - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                        << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                        + dy as ::core::ffi::c_int
                }
            } else if dy as ::core::ffi::c_int
                <= ((ScreenHeight - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                    << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    >> 1 as ::core::ffi::c_int
            {
                dy as ::core::ffi::c_int
            } else {
                dy as ::core::ffi::c_int
                    - (((ScreenHeight
                        - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                        << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
            }) as SIZE;
            if sides_active as ::core::ffi::c_int <= 2 as ::core::ffi::c_int
                || (*ElementPtr).playerNr as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
            {
                Origin.x = ((Origin.x as ::core::ffi::c_int
                    + (dx as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)) as COORD
                    as ::core::ffi::c_int
                    & !(((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int)) as COORD;
                Origin.y = ((Origin.y as ::core::ffi::c_int
                    + (dy as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)) as COORD
                    as ::core::ffi::c_int
                    & !(((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int)) as COORD;
                if (dx as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                    dx = -(dx as ::core::ffi::c_int) as SIZE;
                }
                if (dy as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                    dy = -(dy as ::core::ffi::c_int) as SIZE;
                }
                max_reduction = CalcReduction(dx, dy) as SIZE;
            } else if max_reduction as ::core::ffi::c_int
                > opt_max_zoom_out as ::core::ffi::c_int
                && min_reduction as ::core::ffi::c_int
                    <= opt_max_zoom_out as ::core::ffi::c_int
            {
                Origin.x = ((Origin.x as ::core::ffi::c_int
                    + (dx as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)) as COORD
                    as ::core::ffi::c_int
                    & !(((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int)) as COORD;
                Origin.y = ((Origin.y as ::core::ffi::c_int
                    + (dy as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)) as COORD
                    as ::core::ffi::c_int
                    & !(((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int)) as COORD;
                if (dx as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                    dx = -(dx as ::core::ffi::c_int) as SIZE;
                }
                if (dy as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                    dy = -(dy as ::core::ffi::c_int) as SIZE;
                }
                min_reduction = CalcReduction(dx, dy) as SIZE;
            } else {
                let mut reduction: SIZE = 0;
                if (dx as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                    dx = -(dx as ::core::ffi::c_int) as SIZE;
                }
                if (dy as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                    dy = -(dy as ::core::ffi::c_int) as SIZE;
                }
                reduction = CalcReduction(
                    ((dx as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as SIZE,
                    ((dy as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as SIZE,
                ) as SIZE;
                if min_reduction as ::core::ffi::c_int
                    > opt_max_zoom_out as ::core::ffi::c_int
                    || (reduction as ::core::ffi::c_int)
                        < min_reduction as ::core::ffi::c_int
                {
                    min_reduction = reduction;
                }
            }
        }
        UnlockLink(&raw mut disp_q, hElement);
        hElement = hNextElement;
    }
    if (min_reduction as ::core::ffi::c_int > opt_max_zoom_out as ::core::ffi::c_int
        || min_reduction as ::core::ffi::c_int <= max_reduction as ::core::ffi::c_int)
        && {
            min_reduction = max_reduction;
            min_reduction as ::core::ffi::c_int > opt_max_zoom_out as ::core::ffi::c_int
        }
        && {
            min_reduction = zoom_out;
            min_reduction as ::core::ffi::c_int > opt_max_zoom_out as ::core::ffi::c_int
        }
    {
        if optMeleeScale == C2Rust_Unnamed_6::TFB_SCALE_STEP.0 as ::core::ffi::c_int {
            min_reduction = 0 as SIZE;
        } else {
            min_reduction = ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
                as SIZE;
        }
    }
    return CalcView(&raw mut Origin, min_reduction, pscroll_x, pscroll_y, ships_alive);
}
#[no_mangle]
pub unsafe extern "C" fn InsertPrim(
    mut pLinks: *mut PRIM_LINKS,
    mut primIndex: COUNT,
    mut iPI: COUNT,
) {
    let mut Link: COUNT = 0;
    let mut PL: PRIM_LINKS = 0;
    if iPI as ::core::ffi::c_int
        == 0xffff as ::core::ffi::c_int as COUNT as ::core::ffi::c_int
    {
        Link = (*pLinks >> 16 as ::core::ffi::c_int) as UWORD as COUNT;
        if Link as ::core::ffi::c_int
            == 0xffff as ::core::ffi::c_int as COUNT as ::core::ffi::c_int
        {
            *pLinks = ((primIndex as DWORD) << 16 as ::core::ffi::c_int
                | primIndex as DWORD) as PRIM_LINKS;
        } else {
            *pLinks = ((primIndex as DWORD) << 16 as ::core::ffi::c_int
                | *pLinks as UWORD as DWORD) as PRIM_LINKS;
        }
    } else {
        PL = DisplayArray[iPI as usize].Links;
        if iPI as ::core::ffi::c_int != *pLinks as UWORD as ::core::ffi::c_int {
            Link = PL as UWORD as COUNT;
        } else {
            Link = 0xffff as ::core::ffi::c_int as COUNT;
            *pLinks = (((*pLinks >> 16 as ::core::ffi::c_int) as UWORD as DWORD)
                << 16 as ::core::ffi::c_int | primIndex as DWORD) as PRIM_LINKS;
        }
        DisplayArray[iPI as usize].Links = (((PL >> 16 as ::core::ffi::c_int) as UWORD
            as DWORD) << 16 as ::core::ffi::c_int | primIndex as DWORD) as PRIM_LINKS;
    }
    if Link as ::core::ffi::c_int
        != 0xffff as ::core::ffi::c_int as COUNT as ::core::ffi::c_int
    {
        PL = DisplayArray[Link as usize].Links;
        DisplayArray[Link as usize].Links = ((primIndex as DWORD)
            << 16 as ::core::ffi::c_int | PL as UWORD as DWORD) as PRIM_LINKS;
    }
    DisplayArray[primIndex as usize].Links = ((iPI as DWORD) << 16 as ::core::ffi::c_int
        | Link as DWORD) as PRIM_LINKS;
}
#[no_mangle]
pub static mut DisplayLinks: PRIM_LINKS = 0;
#[inline]
unsafe extern "C" fn CalcDisplayCoord(
    mut c: COORD,
    mut orgc: COORD,
    mut reduction: SIZE,
) -> COORD {
    if optMeleeScale == C2Rust_Unnamed_6::TFB_SCALE_STEP.0 as ::core::ffi::c_int {
        return (c as ::core::ffi::c_int - orgc as ::core::ffi::c_int
            >> reduction as ::core::ffi::c_int) as COORD
    } else {
        return (((c as ::core::ffi::c_int - orgc as ::core::ffi::c_int)
            << 8 as ::core::ffi::c_int) / reduction as ::core::ffi::c_int) as COORD
    };
}
unsafe extern "C" fn PostProcessQueue(
    mut view_state: VIEW_STATE,
    mut scroll_x: SIZE,
    mut scroll_y: SIZE,
) {
    let mut delta: POINT = POINT { x: 0, y: 0 };
    let mut reduction: SIZE = 0;
    let mut hElement: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if optMeleeScale == C2Rust_Unnamed_6::TFB_SCALE_STEP.0 as ::core::ffi::c_int {
        reduction = (zoom_out as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as SIZE;
    } else {
        reduction = ((zoom_out as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
            as SIZE;
    }
    hElement = disp_q.head as HELEMENT;
    while !hElement.is_null() {
        let mut state_flags: ELEMENT_FLAGS = 0;
        let mut ElementPtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
        let mut hNextElement: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
        ElementPtr = LockLink(&raw mut disp_q, hElement) as *mut ELEMENT;
        state_flags = (*ElementPtr).state_flags;
        if state_flags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 11 as ::core::ffi::c_int != 0
        {
            if state_flags as ::core::ffi::c_int
                & (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int == 0
            {
                (*ElementPtr).state_flags = ((*ElementPtr).state_flags
                    as ::core::ffi::c_int
                    & !((1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int))
                    as ELEMENT_FLAGS;
            } else {
                (*ElementPtr).state_flags = ((*ElementPtr).state_flags
                    as ::core::ffi::c_int
                    & !((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int))
                    as ELEMENT_FLAGS;
            }
            if state_flags as ::core::ffi::c_int
                & (1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int != 0
            {
                delta.x = 0 as COORD;
                delta.y = 0 as COORD;
            } else {
                delta.x = scroll_x as COORD;
                delta.y = scroll_y as COORD;
            }
        } else {
            let mut hPostElement: HELEMENT = ::core::ptr::null_mut::<
                ::core::ffi::c_void,
            >();
            hPostElement = hElement;
            loop {
                let mut PostElementPtr: *mut ELEMENT = ::core::ptr::null_mut::<
                    ELEMENT,
                >();
                PostElementPtr = LockLink(&raw mut disp_q, hPostElement) as *mut ELEMENT;
                if (*PostElementPtr).state_flags as ::core::ffi::c_int
                    & (1 as ::core::ffi::c_int) << 11 as ::core::ffi::c_int == 0
                {
                    PreProcess(PostElementPtr);
                }
                hNextElement = (*PostElementPtr).succ;
                if (*PostElementPtr).state_flags as ::core::ffi::c_int
                    & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                        | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) == 0
                    && (*PostElementPtr).state_flags as ::core::ffi::c_int
                        & (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int == 0
                {
                    ProcessCollisions(
                        disp_q.head,
                        PostElementPtr,
                        (((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
                            + 1 as ::core::ffi::c_int) as TIME_VALUE,
                        ((1 as ::core::ffi::c_int) << 11 as ::core::ffi::c_int
                            | (1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                            as ELEMENT_FLAGS,
                    );
                }
                UnlockLink(&raw mut disp_q, hPostElement);
                hPostElement = hNextElement;
                if hPostElement.is_null() {
                    break;
                }
            }
            scroll_x = 0 as SIZE;
            scroll_y = 0 as SIZE;
            delta.x = 0 as COORD;
            delta.y = 0 as COORD;
            state_flags = (*ElementPtr).state_flags;
        }
        if state_flags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int != 0
        {
            hNextElement = (*ElementPtr).succ;
            UnlockLink(&raw mut disp_q, hElement);
            RemoveElement(hElement);
            FreeElement(hElement);
        } else {
            let mut ObjType: GRAPHICS_PRIM = 0;
            ObjType = DisplayArray[(*ElementPtr).PrimIndex as usize].Type;
            if view_state.0 != VIEW_STATE::VIEW_STABLE.0
                || state_flags as ::core::ffi::c_int
                    & ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                        | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) != 0
            {
                let mut next: POINT = POINT { x: 0, y: 0 };
                if ObjType as ::core::ffi::c_int
                    == gfx_object::LINE_PRIM.0 as ::core::ffi::c_int
                {
                    let mut dx: SIZE = 0;
                    let mut dy: SIZE = 0;
                    dx = ((*ElementPtr).next.location.x as ::core::ffi::c_int
                        - (*ElementPtr).current.location.x as ::core::ffi::c_int)
                        as SIZE;
                    dy = ((*ElementPtr).next.location.y as ::core::ffi::c_int
                        - (*ElementPtr).current.location.y as ::core::ffi::c_int)
                        as SIZE;
                    next.x = (if ((*ElementPtr).current.location.x as ::core::ffi::c_int
                        + delta.x as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
                    {
                        (*ElementPtr).current.location.x as ::core::ffi::c_int
                            + delta.x as ::core::ffi::c_int
                            + (((ScreenWidth - 64 as ::core::ffi::c_int
                                - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                                << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                    } else if (*ElementPtr).current.location.x as ::core::ffi::c_int
                        + delta.x as ::core::ffi::c_int
                        >= ((ScreenWidth - 64 as ::core::ffi::c_int
                            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                            << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    {
                        (*ElementPtr).current.location.x as ::core::ffi::c_int
                            + delta.x as ::core::ffi::c_int
                            - (((ScreenWidth - 64 as ::core::ffi::c_int
                                - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                                << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                    } else {
                        (*ElementPtr).current.location.x as ::core::ffi::c_int
                            + delta.x as ::core::ffi::c_int
                    }) as COUNT as COORD;
                    next.y = (if ((*ElementPtr).current.location.y as ::core::ffi::c_int
                        + delta.y as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
                    {
                        (*ElementPtr).current.location.y as ::core::ffi::c_int
                            + delta.y as ::core::ffi::c_int
                            + (((ScreenHeight
                                - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                                << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                    } else if (*ElementPtr).current.location.y as ::core::ffi::c_int
                        + delta.y as ::core::ffi::c_int
                        >= ((ScreenHeight
                            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                            << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    {
                        (*ElementPtr).current.location.y as ::core::ffi::c_int
                            + delta.y as ::core::ffi::c_int
                            - (((ScreenHeight
                                - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                                << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                    } else {
                        (*ElementPtr).current.location.y as ::core::ffi::c_int
                            + delta.y as ::core::ffi::c_int
                    }) as COUNT as COORD;
                    DisplayArray[(*ElementPtr).PrimIndex as usize].Object.Line.first.x = CalcDisplayCoord(
                        next.x,
                        SpaceOrg.x,
                        reduction,
                    );
                    DisplayArray[(*ElementPtr).PrimIndex as usize].Object.Line.first.y = CalcDisplayCoord(
                        next.y,
                        SpaceOrg.y,
                        reduction,
                    );
                    next.x = (next.x as ::core::ffi::c_int + dx as ::core::ffi::c_int)
                        as COORD;
                    next.y = (next.y as ::core::ffi::c_int + dy as ::core::ffi::c_int)
                        as COORD;
                    DisplayArray[(*ElementPtr).PrimIndex as usize]
                        .Object
                        .Line
                        .second
                        .x = CalcDisplayCoord(next.x, SpaceOrg.x, reduction);
                    DisplayArray[(*ElementPtr).PrimIndex as usize]
                        .Object
                        .Line
                        .second
                        .y = CalcDisplayCoord(next.y, SpaceOrg.y, reduction);
                } else {
                    next.x = (if ((*ElementPtr).next.location.x as ::core::ffi::c_int
                        + delta.x as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
                    {
                        (*ElementPtr).next.location.x as ::core::ffi::c_int
                            + delta.x as ::core::ffi::c_int
                            + (((ScreenWidth - 64 as ::core::ffi::c_int
                                - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                                << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                    } else if (*ElementPtr).next.location.x as ::core::ffi::c_int
                        + delta.x as ::core::ffi::c_int
                        >= ((ScreenWidth - 64 as ::core::ffi::c_int
                            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                            << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    {
                        (*ElementPtr).next.location.x as ::core::ffi::c_int
                            + delta.x as ::core::ffi::c_int
                            - (((ScreenWidth - 64 as ::core::ffi::c_int
                                - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                                << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                    } else {
                        (*ElementPtr).next.location.x as ::core::ffi::c_int
                            + delta.x as ::core::ffi::c_int
                    }) as COUNT as COORD;
                    next.y = (if ((*ElementPtr).next.location.y as ::core::ffi::c_int
                        + delta.y as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
                    {
                        (*ElementPtr).next.location.y as ::core::ffi::c_int
                            + delta.y as ::core::ffi::c_int
                            + (((ScreenHeight
                                - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                                << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                    } else if (*ElementPtr).next.location.y as ::core::ffi::c_int
                        + delta.y as ::core::ffi::c_int
                        >= ((ScreenHeight
                            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                            << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    {
                        (*ElementPtr).next.location.y as ::core::ffi::c_int
                            + delta.y as ::core::ffi::c_int
                            - (((ScreenHeight
                                - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                                << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                    } else {
                        (*ElementPtr).next.location.y as ::core::ffi::c_int
                            + delta.y as ::core::ffi::c_int
                    }) as COUNT as COORD;
                    DisplayArray[(*ElementPtr).PrimIndex as usize].Object.Point.x = CalcDisplayCoord(
                        next.x,
                        SpaceOrg.x,
                        reduction,
                    );
                    DisplayArray[(*ElementPtr).PrimIndex as usize].Object.Point.y = CalcDisplayCoord(
                        next.y,
                        SpaceOrg.y,
                        reduction,
                    );
                    if ObjType as ::core::ffi::c_int
                        == gfx_object::STAMP_PRIM.0 as ::core::ffi::c_int
                        || ObjType as ::core::ffi::c_int
                            == gfx_object::STAMPFILL_PRIM.0 as ::core::ffi::c_int
                    {
                        if view_state.0 == VIEW_STATE::VIEW_CHANGE.0
                            || state_flags as ::core::ffi::c_int
                                & ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                    | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) != 0
                        {
                            let mut index: COUNT = 0;
                            let mut scale: COUNT = 256 as COUNT;
                            if optMeleeScale
                                == C2Rust_Unnamed_6::TFB_SCALE_STEP.0 as ::core::ffi::c_int
                            {
                                index = zoom_out as COUNT;
                            } else {
                                CALC_ZOOM_STUFF(&raw mut index, &raw mut scale);
                            }
                            (*ElementPtr).next.image.frame = SetEquFrameIndex(
                                *(*ElementPtr).next.image.farray.offset(index as isize),
                                (*ElementPtr).next.image.frame,
                            );
                            if optMeleeScale
                                == C2Rust_Unnamed_6::TFB_SCALE_TRILINEAR.0
                                    as ::core::ffi::c_int
                                && (index as ::core::ffi::c_int) < 2 as ::core::ffi::c_int
                                && scale as ::core::ffi::c_int != 256 as ::core::ffi::c_int
                            {
                                let mut frame: FRAME = (*ElementPtr).next.image.frame;
                                let mut mmframe: FRAME = SetEquFrameIndex(
                                    *(*ElementPtr)
                                        .next
                                        .image
                                        .farray
                                        .offset(
                                            (index as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                                as isize,
                                        ),
                                    frame,
                                );
                                if !frame.is_null() && !mmframe.is_null() {
                                    let mut mmhs: HOT_SPOT = GetFrameHot(mmframe);
                                    TFB_DrawScreen_SetMipmap(
                                        (*frame).image,
                                        (*mmframe).image,
                                        mmhs.x as ::core::ffi::c_int,
                                        mmhs.y as ::core::ffi::c_int,
                                    );
                                }
                            }
                        }
                        DisplayArray[(*ElementPtr).PrimIndex as usize]
                            .Object
                            .Stamp
                            .frame = (*ElementPtr).next.image.frame;
                    }
                }
                (*ElementPtr).next.location = next;
            }
            PostProcess(ElementPtr);
            if (ObjType as ::core::ffi::c_int)
                < gfx_object::NUM_PRIMS.0 as ::core::ffi::c_int
            {
                InsertPrim(
                    &raw mut DisplayLinks,
                    (*ElementPtr).PrimIndex,
                    0xffff as ::core::ffi::c_int as COUNT,
                );
            }
            hNextElement = (*ElementPtr).succ;
            UnlockLink(&raw mut disp_q, hElement);
        }
        hElement = hNextElement;
    }
}
#[no_mangle]
pub unsafe extern "C" fn InitDisplayList() {
    let mut i: COUNT = 0;
    if optMeleeScale == C2Rust_Unnamed_6::TFB_SCALE_STEP.0 as ::core::ffi::c_int {
        zoom_out = (2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as SIZE;
        opt_max_zoom_out = 2 as SIZE;
    } else {
        zoom_out = (((1 as ::core::ffi::c_int)
            << 8 as ::core::ffi::c_int + 3 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int)
            + ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)) as SIZE;
        opt_max_zoom_out = ((1 as ::core::ffi::c_int)
            << 8 as ::core::ffi::c_int + 3 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as SIZE;
    }
    ReinitQueue(&raw mut disp_q);
    i = 0 as COUNT;
    while (i as ::core::ffi::c_int) < 330 as ::core::ffi::c_int {
        DisplayArray[i as usize].Links = (((i as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int) as DWORD) << 16 as ::core::ffi::c_int
            | 0xffff as ::core::ffi::c_int as COUNT as DWORD) as PRIM_LINKS;
        i = i.wrapping_add(1);
    }
    DisplayArray[(i as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as usize].Links = ((0xffff
        as ::core::ffi::c_int as COUNT as DWORD) << 16 as ::core::ffi::c_int
        | 0xffff as ::core::ffi::c_int as COUNT as DWORD) as PRIM_LINKS;
    DisplayFreeList = 0 as COUNT;
    DisplayLinks = ((0xffff as ::core::ffi::c_int as COUNT as DWORD)
        << 16 as ::core::ffi::c_int | 0xffff as ::core::ffi::c_int as COUNT as DWORD)
        as PRIM_LINKS;
}
#[no_mangle]
pub static mut nth_frame: UWORD = 0 as UWORD;
#[no_mangle]
pub unsafe extern "C" fn RedrawQueue(mut clear: BOOLEAN) {
    let mut scroll_x: SIZE = 0;
    let mut scroll_y: SIZE = 0;
    let mut view_state: VIEW_STATE = VIEW_STATE::VIEW_STABLE;
    SetContext(StatusContext);
    view_state = PreProcessQueue(&raw mut scroll_x, &raw mut scroll_y);
    PostProcessQueue(view_state, scroll_x, scroll_y);
    if optStereoSFX.0 != 0 {
        UpdateSoundPositions();
    }
    SetContext(SpaceContext);
    if GlobData.Game_state.CurrentActivity as BYTE as ::core::ffi::c_int
        == C2Rust_Unnamed_5::SUPER_MELEE.0 as ::core::ffi::c_int
        || GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
            & (C2Rust_Unnamed_5::CHECK_ABORT.0 as ::core::ffi::c_int
                | C2Rust_Unnamed_5::CHECK_LOAD.0 as ::core::ffi::c_int) == 0
    {
        let mut skip_frames: BYTE = 0;
        skip_frames = (nth_frame as ::core::ffi::c_int >> 8 as ::core::ffi::c_int)
            as BYTE;
        if skip_frames as ::core::ffi::c_int
            != !(0 as ::core::ffi::c_int) as BYTE as ::core::ffi::c_int
            && (skip_frames as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                || {
                    nth_frame = nth_frame.wrapping_sub(1);
                    nth_frame as ::core::ffi::c_int & 0xff as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                })
        {
            nth_frame = (nth_frame as ::core::ffi::c_int
                + skip_frames as ::core::ffi::c_int) as UWORD;
            if clear.0 != 0 {
                ClearDrawable();
            }
            if optMeleeScale != C2Rust_Unnamed_6::TFB_SCALE_STEP.0 as ::core::ffi::c_int
            {
                let mut index: COUNT = 0;
                let mut scale: COUNT = 0;
                CALC_ZOOM_STUFF(&raw mut index, &raw mut scale);
                SetGraphicScale(scale as ::core::ffi::c_int);
            }
            DrawBatch(
                &raw mut DisplayArray as *mut PRIMITIVE,
                DisplayLinks,
                0 as BATCH_FLAGS,
            );
            SetGraphicScale(0 as ::core::ffi::c_int);
        }
        FlushSounds();
    } else {
        ProcessSound(
            ::core::ptr::with_exposed_provenance_mut::<
                ::core::ffi::c_void,
            >(!(0 as ::core::ffi::c_int) as usize) as SOUND,
            ::core::ptr::null_mut::<ELEMENT>(),
        );
        FlushSounds();
    }
    DisplayLinks = ((0xffff as ::core::ffi::c_int as COUNT as DWORD)
        << 16 as ::core::ffi::c_int | 0xffff as ::core::ffi::c_int as COUNT as DWORD)
        as PRIM_LINKS;
}
#[no_mangle]
pub unsafe extern "C" fn Untarget(mut ElementPtr: *mut ELEMENT) {
    let mut hElement: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut hNextElement: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
    hElement = disp_q.head as HELEMENT;
    while !hElement.is_null() {
        let mut hTarget: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
        let mut ListPtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
        ListPtr = LockLink(&raw mut disp_q, hElement) as *mut ELEMENT;
        hNextElement = (*ListPtr).succ;
        hTarget = (*ListPtr).hTarget;
        if !hTarget.is_null() {
            let mut TargetElementPtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
            TargetElementPtr = LockLink(&raw mut disp_q, hTarget) as *mut ELEMENT;
            if TargetElementPtr == ElementPtr {
                (*ListPtr).hTarget = ::core::ptr::null_mut::<::core::ffi::c_void>();
            }
            UnlockLink(&raw mut disp_q, hTarget);
        }
        UnlockLink(&raw mut disp_q, hElement);
        hElement = hNextElement;
    }
}
#[no_mangle]
pub unsafe extern "C" fn RemoveElement(mut hLink: HLINK) {
    if optStereoSFX.0 != 0 {
        let mut ElementPtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
        ElementPtr = LockLink(&raw mut disp_q, hLink) as *mut ELEMENT;
        if !ElementPtr.is_null() {
            RemoveSoundsForObject(ElementPtr);
        }
        UnlockLink(&raw mut disp_q, hLink);
    }
    RemoveQueue(&raw mut disp_q, hLink);
}
#[inline]
unsafe extern "C" fn explode() -> ! {
    abort();
}
