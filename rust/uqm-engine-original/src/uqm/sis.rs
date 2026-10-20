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

// Rust translation of src/uqm/sis.c.
extern "C" {
    pub type string_table_entry;
    pub type context_desc;
    pub type frame_desc;
    pub type font_desc;
    pub type FlashContext;
    fn SetContext(Context: CONTEXT) -> CONTEXT;
    fn SetContextForeGroundColor(Color: Color) -> Color;
    fn SetContextBackGroundColor(Color: Color) -> Color;
    fn SetContextFGFrame(Frame: FRAME) -> FRAME;
    fn SetContextClipRect(pRect: *mut RECT) -> BOOLEAN;
    fn GetContextClipRect(pRect: *mut RECT) -> BOOLEAN;
    fn DrawStamp(pStamp: *mut STAMP);
    fn DrawFilledRectangle(pRect: *mut RECT);
    fn font_DrawText(pText: *mut TEXT);
    fn BatchGraphics();
    fn UnbatchGraphics();
    fn ClearDrawable();
    fn GetFrameRect(Frame: FRAME, pRect: *mut RECT) -> BOOLEAN;
    fn SetContextFontEffect(EffectFrame: FRAME) -> FRAME;
    fn SetContextFont(Font: FONT) -> FONT;
    fn TextRect(pText: *mut TEXT, pRect: *mut RECT, pdelta: *mut BYTE) -> BOOLEAN;
    fn GetContextFontLeading(pheight: *mut SIZE) -> BOOLEAN;
    fn SetAbsFrameIndex(Frame: FRAME, FrameIndex: COUNT) -> FRAME;
    fn IncFrameIndex(Frame: FRAME) -> FRAME;
    fn __assert_fail(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    );
    fn snprintf(
        _: *mut ::core::ffi::c_char,
        _: size_t,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn skipUTF8Chars(
        ptr: *const ::core::ffi::c_char,
        num: size_t,
    ) -> *mut ::core::ffi::c_char;
    fn SetAbsStringTableIndex(String: STRING, StringTableIndex: COUNT) -> STRING;
    fn GetStringAddress(String: STRING) -> STRINGPTR;
    static mut ScreenWidth: ::core::ffi::c_int;
    static mut ScreenHeight: ::core::ffi::c_int;
    fn strupr(_: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn Flash_createHighlight(
        gfxContext: CONTEXT,
        rect: *const RECT,
    ) -> *mut FlashContext;
    fn Flash_start(context: *mut FlashContext);
    fn Flash_terminate(context: *mut FlashContext);
    fn Flash_process(context: *mut FlashContext);
    fn Flash_setSpeed(
        context: *mut FlashContext,
        fadeInTime: TimeCount,
        onTime: TimeCount,
        fadeOutTime: TimeCount,
        offTime: TimeCount,
    );
    fn Flash_setMergeFactors(
        context: *mut FlashContext,
        startNumer: ::core::ffi::c_int,
        endNumer: ::core::ffi::c_int,
        denom: ::core::ffi::c_int,
    );
    fn Flash_setFrameTime(context: *mut FlashContext, frameTime: TimeCount);
    fn Flash_nextTime(context: *mut FlashContext) -> TimeCount;
    fn Flash_setRect(context: *mut FlashContext, rect: *const RECT);
    fn Flash_preUpdate(context: *mut FlashContext);
    fn Flash_postUpdate(context: *mut FlashContext);
    fn GetTimeCounter() -> TimeCount;
    fn DrawMenuStateStrings(beg_index: BYTE, NewState: SWORD);
    static mut optWhichMenu: ::core::ffi::c_int;
    static mut optWhichFonts: ::core::ffi::c_int;
    static mut GameStrings: STRING;
    static mut CurStarDescPtr: *mut STAR_DESC;
    fn GetClusterName(pSD: *const STAR_DESC, buf: *mut UNICODE);
    static mut GlobData: GLOBDATA;
    fn getGameState(
        state: *mut BYTE,
        startBit: ::core::ffi::c_int,
        endBit: ::core::ffi::c_int,
    ) -> BYTE;
    fn inHyperSpace() -> BOOLEAN;
    static mut Screen: FRAME;
    static mut FlagStatFrame: FRAME;
    static mut FontGradFrame: FRAME;
    static mut OffScreenContext: CONTEXT;
    static mut ScreenContext: CONTEXT;
    static mut SpaceContext: CONTEXT;
    static mut StatusContext: CONTEXT;
    static mut StarConFont: FONT;
    static mut MicroFont: FONT;
    static mut TinyFont: FONT;
    fn Alarm_addAbsoluteMs(
        ms: uint32,
        callback: AlarmCallback,
        arg: AlarmCallbackArg,
    ) -> *mut Alarm;
    fn Alarm_remove(alarm: *mut Alarm);
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
pub type STRINGPTR = *mut ::core::ffi::c_char;
pub type CONTEXT_DESC = context_desc;
pub type FRAME_DESC = frame_desc;
pub type FONT_DESC = font_desc;
pub type CONTEXT = *mut CONTEXT_DESC;
pub type FRAME = *mut FRAME_DESC;
pub type FONT = *mut FONT_DESC;
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
pub struct C2Rust_Unnamed(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed {
    pub const PLANET_LANDER: Self = Self(0);
    pub const FUSION_THRUSTER: Self = Self(1);
    pub const TURNING_JETS: Self = Self(2);
    pub const CREW_POD: Self = Self(3);
    pub const STORAGE_BAY: Self = Self(4);
    pub const FUEL_TANK: Self = Self(5);
    pub const HIGHEFF_FUELSYS: Self = Self(6);
    pub const DYNAMO_UNIT: Self = Self(7);
    pub const SHIVA_FURNACE: Self = Self(8);
    pub const GUN_WEAPON: Self = Self(9);
    pub const BLASTER_WEAPON: Self = Self(10);
    pub const CANNON_WEAPON: Self = Self(11);
    pub const TRACKING_SYSTEM: Self = Self(12);
    pub const ANTIMISSILE_DEFENSE: Self = Self(13);
    pub const NUM_PURCHASE_MODULES: Self = Self(14);
    pub const BOMB_MODULE_0: Self = Self(14);
    pub const BOMB_MODULE_1: Self = Self(15);
    pub const BOMB_MODULE_2: Self = Self(16);
    pub const BOMB_MODULE_3: Self = Self(17);
    pub const BOMB_MODULE_4: Self = Self(18);
    pub const BOMB_MODULE_5: Self = Self(19);
    pub const NUM_MODULES: Self = Self(20);
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
pub type ACTIVITY = UWORD;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GLOBDATA {
    pub SIS_state: SIS_STATE,
    pub Game_state: GAME_STATE,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct StatMsgMode(pub ::core::ffi::c_uint);
impl StatMsgMode {
    pub const SMM_UNDEFINED: Self = Self(0);
    pub const SMM_DATE: Self = Self(1);
    pub const SMM_RES_UNITS: Self = Self(2);
    pub const SMM_CREDITS: Self = Self(3);
    pub const SMM_DEFAULT: Self = Self(1);
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
pub type LINK = link;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct link {
    pub pred: HLINK,
    pub succ: HLINK,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Alarm {
    pub index: size_t,
    pub time: AlarmTime,
    pub callback: AlarmCallback,
    pub arg: AlarmCallbackArg,
}
pub type AlarmCallbackArg = *mut ::core::ffi::c_void;
pub type AlarmCallback = Option<unsafe extern "C" fn(AlarmCallbackArg) -> ()>;
pub type AlarmTime = uint32;
pub type TimeCount = DWORD;
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_0 {
    pub const STAR_STRING_BASE: Self = Self(0);
    pub const DEVICE_STRING_BASE: Self = Self(133);
    pub const CARGO_STRING_BASE: Self = Self(162);
    pub const ELEMENTS_STRING_BASE: Self = Self(172);
    pub const SCAN_STRING_BASE: Self = Self(305);
    pub const STAR_NUMBER_BASE: Self = Self(361);
    pub const PLANET_NUMBER_BASE: Self = Self(375);
    pub const MONTHS_STRING_BASE: Self = Self(408);
    pub const FEEDBACK_STRING_BASE: Self = Self(420);
    pub const STARBASE_STRING_BASE: Self = Self(422);
    pub const ENCOUNTER_STRING_BASE: Self = Self(427);
    pub const NAVIGATION_STRING_BASE: Self = Self(435);
    pub const NAMING_STRING_BASE: Self = Self(441);
    pub const MELEE_STRING_BASE: Self = Self(445);
    pub const SAVEGAME_STRING_BASE: Self = Self(454);
    pub const OPTION_STRING_BASE: Self = Self(459);
    pub const QUITMENU_STRING_BASE: Self = Self(464);
    pub const STATUS_STRING_BASE: Self = Self(467);
    pub const FLAGSHIP_STRING_BASE: Self = Self(473);
    pub const ORBITSCAN_STRING_BASE: Self = Self(486);
    pub const MAINMENU_STRING_BASE: Self = Self(505);
    pub const NETMELEE_STRING_BASE: Self = Self(560);
    pub const GAMESTR_COUNT: Self = Self(579);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_1(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_1 {
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
pub struct C2Rust_Unnamed_2(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_2 {
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
static mut curMsgMode: StatMsgMode = StatMsgMode::SMM_DEFAULT;
#[no_mangle]
pub unsafe extern "C" fn RepairSISBorder() {
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut OldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    OldContext = SetContext(ScreenContext);
    BatchGraphics();
    r.corner.x = (7 as ::core::ffi::c_int + 0 as ::core::ffi::c_int
        - 1 as ::core::ffi::c_int) as COORD;
    r.corner.y = (10 as ::core::ffi::c_int + 0 as ::core::ffi::c_int
        - 1 as ::core::ffi::c_int) as COORD;
    r.extent.width = 1 as COORD;
    r.extent.height = (ScreenHeight - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
        - 13 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as COORD;
    SetContextForeGroundColor(colorFromRgb15(0x10 as BYTE, 0x10 as BYTE, 0x10 as BYTE));
    DrawFilledRectangle(&raw mut r);
    SetContextForeGroundColor(colorFromRgb15(0x8 as BYTE, 0x8 as BYTE, 0x8 as BYTE));
    r.corner.x = (r.corner.x as ::core::ffi::c_int
        + (ScreenWidth - 64 as ::core::ffi::c_int
            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
            - 14 as ::core::ffi::c_int + 2 as ::core::ffi::c_int
            - 1 as ::core::ffi::c_int)) as COORD;
    DrawFilledRectangle(&raw mut r);
    r.corner.x = (7 as ::core::ffi::c_int + 0 as ::core::ffi::c_int
        - 1 as ::core::ffi::c_int) as COORD;
    r.corner.y = (r.corner.y as ::core::ffi::c_int
        + (ScreenHeight - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
            - 13 as ::core::ffi::c_int + 2 as ::core::ffi::c_int
            - 1 as ::core::ffi::c_int)) as COORD;
    r.extent.width = (ScreenWidth - 64 as ::core::ffi::c_int
        - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int - 14 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as COORD;
    r.extent.height = 1 as COORD;
    DrawFilledRectangle(&raw mut r);
    UnbatchGraphics();
    SetContext(OldContext);
}
#[no_mangle]
pub unsafe extern "C" fn ClearSISRect(mut ClearFlags: BYTE) {
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut OldColor: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    let mut OldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    OldContext = SetContext(StatusContext);
    OldColor = SetContextForeGroundColor(
        colorFromRgb15(0xa as BYTE, 0xa as BYTE, 0xa as BYTE),
    );
    r.corner.x = 2 as COORD;
    r.extent.width = (64 as ::core::ffi::c_int - 4 as ::core::ffi::c_int) as COORD;
    BatchGraphics();
    if ClearFlags as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int != 0
    {
        DeltaSISGauges(0x7fff as SIZE, 0x7fff as SIZE, 0x7fff as ::core::ffi::c_int);
    }
    if ClearFlags as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int != 0
    {
        DrawMenuStateStrings(!(0 as ::core::ffi::c_int) as BYTE, 1 as SWORD);
    }
    UnbatchGraphics();
    SetContextForeGroundColor(OldColor);
    SetContext(OldContext);
}
#[no_mangle]
pub unsafe extern "C" fn DrawSISTitle(mut pStr: *mut UNICODE) {
    let mut t: TEXT = TEXT {
        baseline: POINT { x: 0, y: 0 },
        pStr: ::core::ptr::null::<UNICODE>(),
        align: TEXT_ALIGN::ALIGN_LEFT,
        CharCount: 0,
    };
    let mut OldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    t.baseline.x = (57 as ::core::ffi::c_int - 2 as ::core::ffi::c_int
        >> 1 as ::core::ffi::c_int) as COORD;
    t.baseline.y = (8 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as COORD;
    t.align = TEXT_ALIGN::ALIGN_CENTER;
    t.pStr = pStr;
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    OldContext = SetContext(OffScreenContext);
    r.corner.x = (7 as ::core::ffi::c_int + 0 as ::core::ffi::c_int
        + (ScreenWidth - 64 as ::core::ffi::c_int
            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
            - 14 as ::core::ffi::c_int) - 57 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as COORD;
    r.corner.y = (10 as ::core::ffi::c_int + 0 as ::core::ffi::c_int
        - 8 as ::core::ffi::c_int) as COORD;
    r.extent.width = (57 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as COORD;
    r.extent.height = (8 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as COORD;
    SetContextFGFrame(Screen);
    SetContextClipRect(&raw mut r);
    SetContextFont(TinyFont);
    BatchGraphics();
    SetContextBackGroundColor(colorFromRgb15(0 as BYTE, 0 as BYTE, 0x14 as BYTE));
    ClearDrawable();
    SetContextForeGroundColor(colorFromRgb15(0x1b as BYTE, 0 as BYTE, 0x1b as BYTE));
    font_DrawText(&raw mut t);
    UnbatchGraphics();
    SetContextClipRect(::core::ptr::null_mut::<RECT>());
    SetContext(OldContext);
}
#[no_mangle]
pub unsafe extern "C" fn DrawHyperCoords(mut universe: POINT) {
    let mut buf: [UNICODE; 100] = [0; 100];
    snprintf(
        &raw mut buf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[UNICODE; 100]>(),
        b"%03u.%01u : %03u.%01u\0".as_ptr() as *const ::core::ffi::c_char,
        universe.x as ::core::ffi::c_int / 10 as ::core::ffi::c_int,
        universe.x as ::core::ffi::c_int % 10 as ::core::ffi::c_int,
        universe.y as ::core::ffi::c_int / 10 as ::core::ffi::c_int,
        universe.y as ::core::ffi::c_int % 10 as ::core::ffi::c_int,
    );
    DrawSISTitle(&raw mut buf as *mut UNICODE);
}
#[no_mangle]
pub unsafe extern "C" fn DrawSISMessage(mut pStr: *const UNICODE) {
    DrawSISMessageEx(pStr, -1 as SIZE, -1 as SIZE, 0 as COUNT);
}
#[no_mangle]
pub unsafe extern "C" fn DrawSISMessageEx(
    mut pStr: *const UNICODE,
    mut CurPos: SIZE,
    mut ExPos: SIZE,
    mut flags: COUNT,
) -> BOOLEAN {
    let mut buf: [UNICODE; 256] = [0; 256];
    let mut OldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    let mut t: TEXT = TEXT {
        baseline: POINT { x: 0, y: 0 },
        pStr: ::core::ptr::null::<UNICODE>(),
        align: TEXT_ALIGN::ALIGN_LEFT,
        CharCount: 0,
    };
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    OldContext = SetContext(OffScreenContext);
    r.corner.x = (7 as ::core::ffi::c_int + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as COORD;
    r.corner.y = (10 as ::core::ffi::c_int + 0 as ::core::ffi::c_int
        - 8 as ::core::ffi::c_int) as COORD;
    r.extent.width = (ScreenWidth - 64 as ::core::ffi::c_int
        - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int - 14 as ::core::ffi::c_int
        - 57 as ::core::ffi::c_int - 12 as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
        as COORD;
    r.extent.height = (8 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as COORD;
    SetContextFGFrame(Screen);
    SetContextClipRect(&raw mut r);
    BatchGraphics();
    SetContextBackGroundColor(colorFromRgb15(0 as BYTE, 0 as BYTE, 0x14 as BYTE));
    if pStr.is_null() {
        match GlobData.Game_state.CurrentActivity as BYTE as ::core::ffi::c_int {
            1 | 4 => {
                GetClusterName(CurStarDescPtr, &raw mut buf as *mut UNICODE);
                pStr = &raw mut buf as *mut UNICODE;
            }
            3 => {
                if inHyperSpace().0 != 0 {
                    pStr = GetStringAddress(
                        SetAbsStringTableIndex(
                            GameStrings,
                            C2Rust_Unnamed_0::NAVIGATION_STRING_BASE.0
                                as ::core::ffi::c_int as COUNT,
                        ),
                    );
                } else {
                    pStr = GetStringAddress(
                        SetAbsStringTableIndex(
                            GameStrings,
                            (C2Rust_Unnamed_0::NAVIGATION_STRING_BASE.0
                                as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as COUNT,
                        ),
                    );
                }
            }
            2 | _ => {
                pStr = b"\0".as_ptr() as *const ::core::ffi::c_char as *const UNICODE;
            }
        }
    }
    if flags as ::core::ffi::c_int & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
        == 0
    {
        SetContextForeGroundColor(colorFromRgb15(0x1b as BYTE, 0 as BYTE, 0x1b as BYTE));
    }
    t.baseline.y = (8 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as COORD;
    t.pStr = pStr;
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    SetContextFont(TinyFont);
    if flags as ::core::ffi::c_int & (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
        != 0
    {
        SetFlashRect(::core::ptr::null::<RECT>());
    }
    if (CurPos as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
        && (ExPos as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
    {
        ClearDrawable();
        t.baseline.x = (ScreenWidth - 64 as ::core::ffi::c_int
            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
            - 14 as ::core::ffi::c_int - 57 as ::core::ffi::c_int
            - 12 as ::core::ffi::c_int - 2 as ::core::ffi::c_int
            >> 1 as ::core::ffi::c_int) as COORD;
        t.align = TEXT_ALIGN::ALIGN_CENTER;
        font_DrawText(&raw mut t);
    } else {
        let mut i: ::core::ffi::c_int = 0;
        let mut text_r: RECT = RECT {
            corner: POINT { x: 0, y: 0 },
            extent: EXTENT { width: 0, height: 0 },
        };
        let mut char_deltas: [BYTE; 128] = [0; 128];
        let mut pchar_deltas: *mut BYTE = ::core::ptr::null_mut::<BYTE>();
        t.baseline.x = 3 as COORD;
        t.align = TEXT_ALIGN::ALIGN_LEFT;
        TextRect(&raw mut t, &raw mut text_r, &raw mut char_deltas as *mut BYTE);
        if text_r.extent.width as ::core::ffi::c_int + t.baseline.x as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int >= r.extent.width as ::core::ffi::c_int
        {
            UnbatchGraphics();
            SetContextClipRect(::core::ptr::null_mut::<RECT>());
            SetContext(OldContext);
            return BOOLEAN::FALSE;
        }
        ClearDrawable();
        if CurPos as ::core::ffi::c_int >= 0 as ::core::ffi::c_int
            && CurPos as ::core::ffi::c_int <= t.CharCount as ::core::ffi::c_int
        {
            let mut cur_r: RECT = text_r;
            i = CurPos as ::core::ffi::c_int;
            pchar_deltas = &raw mut char_deltas as *mut BYTE;
            while i > 0 as ::core::ffi::c_int {
                let c2rust_fresh2 = pchar_deltas;
                pchar_deltas = pchar_deltas.offset(1);
                cur_r.corner.x = (cur_r.corner.x as ::core::ffi::c_int
                    + *c2rust_fresh2 as SIZE as ::core::ffi::c_int) as COORD;
                i -= 1;
            }
            if (CurPos as ::core::ffi::c_int) < t.CharCount as ::core::ffi::c_int {
                cur_r.corner.x -= 1;
            }
            if flags as ::core::ffi::c_int
                & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int != 0
            {
                if CurPos as ::core::ffi::c_int == t.CharCount as ::core::ffi::c_int {
                    cur_r.extent.width = 1 as COORD;
                } else if CurPos as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                    == t.CharCount as ::core::ffi::c_int
                {
                    cur_r.extent.width = (*pchar_deltas as SIZE as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int) as COORD;
                } else {
                    cur_r.extent.width = (*pchar_deltas as SIZE as ::core::ffi::c_int
                        + 1 as ::core::ffi::c_int) as COORD;
                }
            } else {
                cur_r.extent.width = 1 as COORD;
            }
            cur_r.corner.y = 0 as COORD;
            cur_r.extent.height = r.extent.height;
            SetContextForeGroundColor(colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE));
            DrawFilledRectangle(&raw mut cur_r);
        }
        SetContextForeGroundColor(colorFromRgb15(0x1b as BYTE, 0 as BYTE, 0x1b as BYTE));
        if ExPos as ::core::ffi::c_int >= 0 as ::core::ffi::c_int
            && (ExPos as ::core::ffi::c_int) < t.CharCount as ::core::ffi::c_int
        {
            t.CharCount = ExPos as COUNT;
            font_DrawText(&raw mut t);
            SetContextForeGroundColor(
                colorFromRgb15(0x12 as BYTE, 0 as BYTE, 0x12 as BYTE),
            );
            i = ExPos as ::core::ffi::c_int;
            pchar_deltas = &raw mut char_deltas as *mut BYTE;
            while i > 0 as ::core::ffi::c_int {
                let c2rust_fresh3 = pchar_deltas;
                pchar_deltas = pchar_deltas.offset(1);
                t.baseline.x = (t.baseline.x as ::core::ffi::c_int
                    + *c2rust_fresh3 as SIZE as ::core::ffi::c_int) as COORD;
                i -= 1;
            }
            t.pStr = skipUTF8Chars(t.pStr, ExPos as size_t);
            t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
            font_DrawText(&raw mut t);
        } else {
            font_DrawText(&raw mut t);
        }
    }
    if flags as ::core::ffi::c_int & (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
        != 0
    {
        r.corner.x = 0 as COORD;
        r.corner.y = 0 as COORD;
        SetFlashRect(&raw mut r);
    }
    UnbatchGraphics();
    SetContextClipRect(::core::ptr::null_mut::<RECT>());
    SetContext(OldContext);
    return BOOLEAN::TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn DateToString(
    mut buf: *mut ::core::ffi::c_char,
    mut bufLen: size_t,
    mut month_index: BYTE,
    mut day_index: BYTE,
    mut year_index: COUNT,
) {
    snprintf(
        buf,
        bufLen,
        b"%s %02d\xC2\xB7%04d\0".as_ptr() as *const ::core::ffi::c_char,
        GetStringAddress(
            SetAbsStringTableIndex(
                GameStrings,
                (C2Rust_Unnamed_0::MONTHS_STRING_BASE.0 as ::core::ffi::c_int
                    + month_index as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                    as COUNT,
            ),
        ),
        day_index as ::core::ffi::c_int,
        year_index as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn GetStatusMessageRect(mut r: *mut RECT) {
    (*r).corner.x = 2 as COORD;
    (*r).corner.y = 130 as COORD;
    (*r).extent.width = (64 as ::core::ffi::c_int - 4 as ::core::ffi::c_int) as COORD;
    (*r).extent.height = 7 as COORD;
}
#[no_mangle]
pub unsafe extern "C" fn DrawStatusMessage(mut pStr: *const UNICODE) {
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut ctxRect: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut t: TEXT = TEXT {
        baseline: POINT { x: 0, y: 0 },
        pStr: ::core::ptr::null::<UNICODE>(),
        align: TEXT_ALIGN::ALIGN_LEFT,
        CharCount: 0,
    };
    let mut buf: [UNICODE; 128] = [0; 128];
    let mut OldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    OldContext = SetContext(StatusContext);
    GetContextClipRect(&raw mut ctxRect);
    SetContext(OffScreenContext);
    SetContextFGFrame(Screen);
    GetStatusMessageRect(&raw mut r);
    r.corner.x = (r.corner.x as ::core::ffi::c_int
        + ctxRect.corner.x as ::core::ffi::c_int) as COORD;
    r.corner.y = (r.corner.y as ::core::ffi::c_int
        + ctxRect.corner.y as ::core::ffi::c_int) as COORD;
    SetContextClipRect(&raw mut r);
    BatchGraphics();
    SetContextBackGroundColor(colorFromRgb15(0 as BYTE, 0x8 as BYTE, 0 as BYTE));
    ClearDrawable();
    if pStr.is_null() {
        if curMsgMode.0 == StatMsgMode::SMM_CREDITS.0 {
            snprintf(
                &raw mut buf as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[UNICODE; 128]>(),
                b"%u %s\0".as_ptr() as *const ::core::ffi::c_char,
                ((getGameState(
                    &raw mut GlobData.Game_state.GameState as *mut BYTE,
                    C2Rust_Unnamed_1::MELNORME_CREDIT1.0 as ::core::ffi::c_int,
                    C2Rust_Unnamed_1::END_MELNORME_CREDIT1.0 as ::core::ffi::c_int,
                ) as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as UWORD
                    as ::core::ffi::c_int
                    | getGameState(
                        &raw mut GlobData.Game_state.GameState as *mut BYTE,
                        C2Rust_Unnamed_1::MELNORME_CREDIT0.0 as ::core::ffi::c_int,
                        C2Rust_Unnamed_1::END_MELNORME_CREDIT0.0 as ::core::ffi::c_int,
                    ) as ::core::ffi::c_int,
                GetStringAddress(
                    SetAbsStringTableIndex(
                        GameStrings,
                        (C2Rust_Unnamed_0::STATUS_STRING_BASE.0 as ::core::ffi::c_int
                            + 0 as ::core::ffi::c_int) as COUNT,
                    ),
                ),
            );
        } else if curMsgMode.0 == StatMsgMode::SMM_RES_UNITS.0 {
            if (getGameState(
                &raw mut GlobData.Game_state.GameState as *mut BYTE,
                C2Rust_Unnamed_1::CHMMR_BOMB_STATE.0 as ::core::ffi::c_int,
                C2Rust_Unnamed_1::END_CHMMR_BOMB_STATE.0 as ::core::ffi::c_int,
            ) as ::core::ffi::c_int) < 2 as ::core::ffi::c_int
            {
                snprintf(
                    &raw mut buf as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[UNICODE; 128]>(),
                    b"%u %s\0".as_ptr() as *const ::core::ffi::c_char,
                    GlobData.SIS_state.ResUnits,
                    GetStringAddress(
                        SetAbsStringTableIndex(
                            GameStrings,
                            (C2Rust_Unnamed_0::STATUS_STRING_BASE.0 as ::core::ffi::c_int
                                + 1 as ::core::ffi::c_int) as COUNT,
                        ),
                    ),
                );
            } else {
                snprintf(
                    &raw mut buf as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[UNICODE; 128]>(),
                    b"%s %s\0".as_ptr() as *const ::core::ffi::c_char,
                    if optWhichMenu == 0x2 as ::core::ffi::c_int {
                        GetStringAddress(
                            SetAbsStringTableIndex(
                                GameStrings,
                                (C2Rust_Unnamed_0::STATUS_STRING_BASE.0
                                    as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as COUNT,
                            ),
                        ) as *const UNICODE
                    } else {
                        b"\xE2\x88\x9E\0".as_ptr() as *const UNICODE
                    },
                    GetStringAddress(
                        SetAbsStringTableIndex(
                            GameStrings,
                            (C2Rust_Unnamed_0::STATUS_STRING_BASE.0 as ::core::ffi::c_int
                                + 1 as ::core::ffi::c_int) as COUNT,
                        ),
                    ),
                );
            }
        } else {
            DateToString(
                &raw mut buf as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[UNICODE; 128]>(),
                GlobData.Game_state.GameClock.month_index,
                GlobData.Game_state.GameClock.day_index,
                GlobData.Game_state.GameClock.year_index,
            );
        }
        pStr = &raw mut buf as *mut UNICODE;
    }
    t.baseline.x = (64 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
        >> 1 as ::core::ffi::c_int) as COORD;
    t.baseline.y = (7 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as COORD;
    t.align = TEXT_ALIGN::ALIGN_CENTER;
    t.pStr = pStr;
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    SetContextFont(TinyFont);
    SetContextForeGroundColor(colorFromRgb15(0 as BYTE, 0x10 as BYTE, 0 as BYTE));
    font_DrawText(&raw mut t);
    UnbatchGraphics();
    SetContextClipRect(::core::ptr::null_mut::<RECT>());
    SetContext(OldContext);
}
#[no_mangle]
pub unsafe extern "C" fn SetStatusMessageMode(mut newMode: StatMsgMode) -> StatMsgMode {
    let mut oldMode: StatMsgMode = curMsgMode;
    curMsgMode = newMode;
    return oldMode;
}
#[no_mangle]
pub unsafe extern "C" fn DrawCaptainsName() {
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut t: TEXT = TEXT {
        baseline: POINT { x: 0, y: 0 },
        pStr: ::core::ptr::null::<UNICODE>(),
        align: TEXT_ALIGN::ALIGN_LEFT,
        CharCount: 0,
    };
    let mut OldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    let mut OldFont: FONT = ::core::ptr::null_mut::<FONT_DESC>();
    let mut OldColor: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    OldContext = SetContext(StatusContext);
    OldFont = SetContextFont(TinyFont);
    OldColor = SetContextForeGroundColor(
        colorFromRgb15(0 as BYTE, 0 as BYTE, 0x14 as BYTE),
    );
    r.corner.x = (2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as COORD;
    r.corner.y = 10 as COORD;
    r.extent.width = (64 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
        - 2 as ::core::ffi::c_int) as COORD;
    r.extent.height = 7 as COORD;
    DrawFilledRectangle(&raw mut r);
    t.baseline.x = ((64 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
        - 1 as ::core::ffi::c_int) as COORD;
    t.baseline.y = (r.corner.y as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as COORD;
    t.align = TEXT_ALIGN::ALIGN_CENTER;
    t.pStr = &raw mut GlobData.SIS_state.CommanderName as *mut UNICODE;
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    SetContextForeGroundColor(colorFromRgb15(0x16 as BYTE, 0xb as BYTE, 0x1f as BYTE));
    font_DrawText(&raw mut t);
    SetContextForeGroundColor(OldColor);
    SetContextFont(OldFont);
    SetContext(OldContext);
}
#[no_mangle]
pub unsafe extern "C" fn DrawFlagshipName(mut InStatusArea: BOOLEAN) {
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut t: TEXT = TEXT {
        baseline: POINT { x: 0, y: 0 },
        pStr: ::core::ptr::null::<UNICODE>(),
        align: TEXT_ALIGN::ALIGN_LEFT,
        CharCount: 0,
    };
    let mut OldFont: FONT = ::core::ptr::null_mut::<FONT_DESC>();
    let mut OldColor: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    let mut OldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    let mut OldFontEffect: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
    let mut buf: [UNICODE; 250] = [0; 250];
    if InStatusArea.0 != 0 {
        OldContext = SetContext(StatusContext);
        OldFont = SetContextFont(StarConFont);
        r.corner.x = 2 as COORD;
        r.corner.y = 20 as COORD;
        r.extent.width = (64 as ::core::ffi::c_int - 4 as ::core::ffi::c_int) as COORD;
        r.extent.height = 7 as COORD;
        t.pStr = &raw mut GlobData.SIS_state.ShipName as *mut UNICODE;
    } else {
        OldContext = SetContext(SpaceContext);
        OldFont = SetContextFont(MicroFont);
        r.corner.x = 0 as COORD;
        r.corner.y = 1 as COORD;
        r.extent.width = (ScreenWidth - 64 as ::core::ffi::c_int
            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
            - 14 as ::core::ffi::c_int) as COORD;
        r.extent.height = 7 as COORD;
        t.pStr = &raw mut buf as *mut UNICODE;
        snprintf(
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[UNICODE; 250]>(),
            b"%s %s\0".as_ptr() as *const ::core::ffi::c_char,
            GetStringAddress(
                SetAbsStringTableIndex(
                    GameStrings,
                    (C2Rust_Unnamed_0::NAMING_STRING_BASE.0 as ::core::ffi::c_int
                        + 1 as ::core::ffi::c_int) as COUNT,
                ),
            ),
            &raw mut GlobData.SIS_state.ShipName as *mut UNICODE,
        );
        strupr(&raw mut buf as *mut ::core::ffi::c_char);
    }
    OldFontEffect = SetContextFontEffect(::core::ptr::null_mut::<FRAME_DESC>());
    OldColor = SetContextForeGroundColor(
        colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE),
    );
    DrawFilledRectangle(&raw mut r);
    t.baseline.x = (r.corner.x as ::core::ffi::c_int
        + (r.extent.width as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)) as COORD;
    t.baseline.y = (r.corner.y as ::core::ffi::c_uint)
        .wrapping_add((7 as ::core::ffi::c_uint).wrapping_sub(InStatusArea.0)) as COORD;
    t.align = TEXT_ALIGN::ALIGN_CENTER;
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    if optWhichFonts == 0x2 as ::core::ffi::c_int {
        SetContextFontEffect(
            SetAbsFrameIndex(
                FontGradFrame,
                (if InStatusArea.0 != 0 {
                    0 as ::core::ffi::c_int
                } else {
                    3 as ::core::ffi::c_int
                }) as COUNT,
            ),
        );
    } else {
        SetContextForeGroundColor(colorFromRgb15(0x14 as BYTE, 0xa as BYTE, 0 as BYTE));
    }
    font_DrawText(&raw mut t);
    SetContextFontEffect(OldFontEffect);
    SetContextForeGroundColor(OldColor);
    SetContextFont(OldFont);
    SetContext(OldContext);
}
#[no_mangle]
pub unsafe extern "C" fn DrawFlagshipStats() {
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut t: TEXT = TEXT {
        baseline: POINT { x: 0, y: 0 },
        pStr: ::core::ptr::null::<UNICODE>(),
        align: TEXT_ALIGN::ALIGN_LEFT,
        CharCount: 0,
    };
    let mut OldFont: FONT = ::core::ptr::null_mut::<FONT_DESC>();
    let mut OldColor: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    let mut OldFontEffect: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
    let mut OldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    let mut buf: [UNICODE; 128] = [0; 128];
    let mut leading: SIZE = 0;
    let mut i: BYTE = 0;
    let mut energy_regeneration: BYTE = 0;
    let mut energy_wait: BYTE = 0;
    let mut turn_wait: BYTE = 0;
    let mut max_thrust: COUNT = 0;
    let mut fuel: DWORD = 0;
    energy_regeneration = 1 as BYTE;
    energy_wait = 10 as BYTE;
    max_thrust = 10 as COUNT;
    turn_wait = 17 as BYTE;
    fuel = (10 as ::core::ffi::c_int * 100 as ::core::ffi::c_int) as DWORD;
    i = 0 as BYTE;
    while (i as ::core::ffi::c_int) < 16 as ::core::ffi::c_int {
        match GlobData.SIS_state.ModuleSlots[i as usize] as ::core::ffi::c_int {
            5 => {
                fuel = fuel
                    .wrapping_add(
                        (50 as ::core::ffi::c_int * 100 as ::core::ffi::c_int) as DWORD,
                    );
            }
            6 => {
                fuel = fuel
                    .wrapping_add(
                        (100 as ::core::ffi::c_int * 100 as ::core::ffi::c_int) as DWORD,
                    );
            }
            7 => {
                energy_wait = (energy_wait as ::core::ffi::c_int
                    - 2 as ::core::ffi::c_int) as BYTE;
                if (energy_wait as ::core::ffi::c_int) < 4 as ::core::ffi::c_int {
                    energy_wait = 4 as BYTE;
                }
            }
            8 => {
                energy_regeneration = energy_regeneration.wrapping_add(1);
            }
            _ => {}
        }
        i = i.wrapping_add(1);
    }
    i = 0 as BYTE;
    while (i as ::core::ffi::c_int) < 11 as ::core::ffi::c_int {
        if GlobData.SIS_state.DriveSlots[i as usize] as ::core::ffi::c_int
            == C2Rust_Unnamed::FUSION_THRUSTER.0 as ::core::ffi::c_int
        {
            max_thrust = (max_thrust as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
                as COUNT;
        }
        i = i.wrapping_add(1);
    }
    i = 0 as BYTE;
    while (i as ::core::ffi::c_int) < 8 as ::core::ffi::c_int {
        if GlobData.SIS_state.JetSlots[i as usize] as ::core::ffi::c_int
            == C2Rust_Unnamed::TURNING_JETS.0 as ::core::ffi::c_int
        {
            turn_wait = (turn_wait as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
                as BYTE;
        }
        i = i.wrapping_add(1);
    }
    OldContext = SetContext(SpaceContext);
    OldFont = SetContextFont(StarConFont);
    OldFontEffect = SetContextFontEffect(::core::ptr::null_mut::<FRAME_DESC>());
    GetContextFontLeading(&raw mut leading);
    r.corner.x = 0 as COORD;
    r.corner.y = (ScreenHeight - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
        - 13 as ::core::ffi::c_int
        - 4 as ::core::ffi::c_int * leading as ::core::ffi::c_int) as COORD;
    r.extent.width = (ScreenWidth - 64 as ::core::ffi::c_int
        - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int - 14 as ::core::ffi::c_int)
        as COORD;
    r.extent.height = (4 as ::core::ffi::c_int * leading as ::core::ffi::c_int) as COORD;
    OldColor = SetContextForeGroundColor(
        colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE),
    );
    DrawFilledRectangle(&raw mut r);
    leading = (leading as ::core::ffi::c_int - 3 as ::core::ffi::c_int) as SIZE;
    t.baseline.x = ((ScreenWidth - 64 as ::core::ffi::c_int
        - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int - 14 as ::core::ffi::c_int)
        / 6 as ::core::ffi::c_int) as COORD;
    t.baseline.y = (r.corner.y as ::core::ffi::c_int + leading as ::core::ffi::c_int
        + 3 as ::core::ffi::c_int) as COORD;
    t.align = TEXT_ALIGN::ALIGN_RIGHT;
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    SetContextFontEffect(SetAbsFrameIndex(FontGradFrame, 4 as COUNT));
    t.pStr = GetStringAddress(
        SetAbsStringTableIndex(
            GameStrings,
            (C2Rust_Unnamed_0::FLAGSHIP_STRING_BASE.0 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int) as COUNT,
        ),
    );
    font_DrawText(&raw mut t);
    t.baseline.y = (t.baseline.y as ::core::ffi::c_int + leading as ::core::ffi::c_int)
        as COORD;
    t.pStr = GetStringAddress(
        SetAbsStringTableIndex(
            GameStrings,
            (C2Rust_Unnamed_0::FLAGSHIP_STRING_BASE.0 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int) as COUNT,
        ),
    );
    font_DrawText(&raw mut t);
    t.baseline.y = (t.baseline.y as ::core::ffi::c_int + leading as ::core::ffi::c_int)
        as COORD;
    t.pStr = GetStringAddress(
        SetAbsStringTableIndex(
            GameStrings,
            (C2Rust_Unnamed_0::FLAGSHIP_STRING_BASE.0 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int) as COUNT,
        ),
    );
    font_DrawText(&raw mut t);
    t.baseline.y = (t.baseline.y as ::core::ffi::c_int + leading as ::core::ffi::c_int)
        as COORD;
    t.pStr = GetStringAddress(
        SetAbsStringTableIndex(
            GameStrings,
            (C2Rust_Unnamed_0::FLAGSHIP_STRING_BASE.0 as ::core::ffi::c_int
                + 3 as ::core::ffi::c_int) as COUNT,
        ),
    );
    font_DrawText(&raw mut t);
    t.baseline.x = (t.baseline.x as ::core::ffi::c_int + 5 as ::core::ffi::c_int)
        as COORD;
    t.baseline.y = (r.corner.y as ::core::ffi::c_int + leading as ::core::ffi::c_int
        + 3 as ::core::ffi::c_int) as COORD;
    t.align = TEXT_ALIGN::ALIGN_LEFT;
    t.pStr = &raw mut buf as *mut UNICODE;
    snprintf(
        &raw mut buf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[UNICODE; 128]>(),
        b"%-7.7s\0".as_ptr() as *const ::core::ffi::c_char,
        describeWeapon(GlobData.SIS_state.ModuleSlots[15usize]),
    );
    font_DrawText(&raw mut t);
    t.baseline.y = (t.baseline.y as ::core::ffi::c_int + leading as ::core::ffi::c_int)
        as COORD;
    snprintf(
        &raw mut buf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[UNICODE; 128]>(),
        b"%-7.7s\0".as_ptr() as *const ::core::ffi::c_char,
        describeWeapon(GlobData.SIS_state.ModuleSlots[14usize]),
    );
    font_DrawText(&raw mut t);
    t.baseline.y = (t.baseline.y as ::core::ffi::c_int + leading as ::core::ffi::c_int)
        as COORD;
    snprintf(
        &raw mut buf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[UNICODE; 128]>(),
        b"%-7.7s\0".as_ptr() as *const ::core::ffi::c_char,
        describeWeapon(GlobData.SIS_state.ModuleSlots[13usize]),
    );
    font_DrawText(&raw mut t);
    t.baseline.y = (t.baseline.y as ::core::ffi::c_int + leading as ::core::ffi::c_int)
        as COORD;
    snprintf(
        &raw mut buf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[UNICODE; 128]>(),
        b"%-7.7s\0".as_ptr() as *const ::core::ffi::c_char,
        describeWeapon(GlobData.SIS_state.ModuleSlots[0usize]),
    );
    font_DrawText(&raw mut t);
    t.baseline.x = (r.extent.width as ::core::ffi::c_int - 25 as ::core::ffi::c_int)
        as COORD;
    t.baseline.y = (r.corner.y as ::core::ffi::c_int + leading as ::core::ffi::c_int
        + 3 as ::core::ffi::c_int) as COORD;
    t.align = TEXT_ALIGN::ALIGN_RIGHT;
    SetContextFontEffect(SetAbsFrameIndex(FontGradFrame, 5 as COUNT));
    t.pStr = GetStringAddress(
        SetAbsStringTableIndex(
            GameStrings,
            (C2Rust_Unnamed_0::FLAGSHIP_STRING_BASE.0 as ::core::ffi::c_int
                + 4 as ::core::ffi::c_int) as COUNT,
        ),
    );
    font_DrawText(&raw mut t);
    t.baseline.y = (t.baseline.y as ::core::ffi::c_int + leading as ::core::ffi::c_int)
        as COORD;
    t.pStr = GetStringAddress(
        SetAbsStringTableIndex(
            GameStrings,
            (C2Rust_Unnamed_0::FLAGSHIP_STRING_BASE.0 as ::core::ffi::c_int
                + 5 as ::core::ffi::c_int) as COUNT,
        ),
    );
    font_DrawText(&raw mut t);
    t.baseline.y = (t.baseline.y as ::core::ffi::c_int + leading as ::core::ffi::c_int)
        as COORD;
    t.pStr = GetStringAddress(
        SetAbsStringTableIndex(
            GameStrings,
            (C2Rust_Unnamed_0::FLAGSHIP_STRING_BASE.0 as ::core::ffi::c_int
                + 6 as ::core::ffi::c_int) as COUNT,
        ),
    );
    font_DrawText(&raw mut t);
    t.baseline.y = (t.baseline.y as ::core::ffi::c_int + leading as ::core::ffi::c_int)
        as COORD;
    t.pStr = GetStringAddress(
        SetAbsStringTableIndex(
            GameStrings,
            (C2Rust_Unnamed_0::FLAGSHIP_STRING_BASE.0 as ::core::ffi::c_int
                + 7 as ::core::ffi::c_int) as COUNT,
        ),
    );
    font_DrawText(&raw mut t);
    t.baseline.x = (r.extent.width as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
        as COORD;
    t.baseline.y = (r.corner.y as ::core::ffi::c_int + leading as ::core::ffi::c_int
        + 3 as ::core::ffi::c_int) as COORD;
    t.pStr = &raw mut buf as *mut UNICODE;
    snprintf(
        &raw mut buf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[UNICODE; 128]>(),
        b"%4u\0".as_ptr() as *const ::core::ffi::c_char,
        max_thrust as ::core::ffi::c_int * 4 as ::core::ffi::c_int,
    );
    font_DrawText(&raw mut t);
    t.baseline.y = (t.baseline.y as ::core::ffi::c_int + leading as ::core::ffi::c_int)
        as COORD;
    snprintf(
        &raw mut buf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[UNICODE; 128]>(),
        b"%4u\0".as_ptr() as *const ::core::ffi::c_char,
        1 as ::core::ffi::c_int + 17 as ::core::ffi::c_int
            - turn_wait as ::core::ffi::c_int,
    );
    font_DrawText(&raw mut t);
    t.baseline.y = (t.baseline.y as ::core::ffi::c_int + leading as ::core::ffi::c_int)
        as COORD;
    let mut energy_per_10_sec: ::core::ffi::c_uint = ((100 as ::core::ffi::c_int
        * 840 as ::core::ffi::c_int * energy_regeneration as ::core::ffi::c_int
        / ((1 as ::core::ffi::c_int + energy_wait as ::core::ffi::c_int)
            * (840 as ::core::ffi::c_int / 24 as ::core::ffi::c_int))
        + 5 as ::core::ffi::c_int) / 10 as ::core::ffi::c_int) as ::core::ffi::c_uint;
    snprintf(
        &raw mut buf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[UNICODE; 128]>(),
        b"%2u.%1u\0".as_ptr() as *const ::core::ffi::c_char,
        energy_per_10_sec.wrapping_div(10 as ::core::ffi::c_uint),
        energy_per_10_sec.wrapping_rem(10 as ::core::ffi::c_uint),
    );
    font_DrawText(&raw mut t);
    t.baseline.y = (t.baseline.y as ::core::ffi::c_int + leading as ::core::ffi::c_int)
        as COORD;
    snprintf(
        &raw mut buf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[UNICODE; 128]>(),
        b"%4u\0".as_ptr() as *const ::core::ffi::c_char,
        fuel.wrapping_div(100 as DWORD),
    );
    font_DrawText(&raw mut t);
    SetContextFontEffect(OldFontEffect);
    SetContextForeGroundColor(OldColor);
    SetContextFont(OldFont);
    SetContext(OldContext);
}
unsafe extern "C" fn describeWeapon(mut moduleType: BYTE) -> *const UNICODE {
    match moduleType as ::core::ffi::c_int {
        9 => {
            return GetStringAddress(
                SetAbsStringTableIndex(
                    GameStrings,
                    (C2Rust_Unnamed_0::FLAGSHIP_STRING_BASE.0 as ::core::ffi::c_int
                        + 8 as ::core::ffi::c_int) as COUNT,
                ),
            );
        }
        10 => {
            return GetStringAddress(
                SetAbsStringTableIndex(
                    GameStrings,
                    (C2Rust_Unnamed_0::FLAGSHIP_STRING_BASE.0 as ::core::ffi::c_int
                        + 9 as ::core::ffi::c_int) as COUNT,
                ),
            );
        }
        11 => {
            return GetStringAddress(
                SetAbsStringTableIndex(
                    GameStrings,
                    (C2Rust_Unnamed_0::FLAGSHIP_STRING_BASE.0 as ::core::ffi::c_int
                        + 10 as ::core::ffi::c_int) as COUNT,
                ),
            );
        }
        14 | 15 | 16 | 17 | 18 | 19 => {
            return GetStringAddress(
                SetAbsStringTableIndex(
                    GameStrings,
                    (C2Rust_Unnamed_0::FLAGSHIP_STRING_BASE.0 as ::core::ffi::c_int
                        + 11 as ::core::ffi::c_int) as COUNT,
                ),
            );
        }
        _ => {
            return GetStringAddress(
                SetAbsStringTableIndex(
                    GameStrings,
                    (C2Rust_Unnamed_0::FLAGSHIP_STRING_BASE.0 as ::core::ffi::c_int
                        + 12 as ::core::ffi::c_int) as COUNT,
                ),
            );
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn DrawLanders() {
    let mut i: BYTE = 0;
    let mut width: SIZE = 0;
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut s: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    let mut OldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    OldContext = SetContext(StatusContext);
    s.frame = IncFrameIndex(FlagStatFrame);
    GetFrameRect(s.frame, &raw mut r);
    i = GlobData.SIS_state.NumLanders;
    r.corner.x = ((64 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
        - r.corner.x as ::core::ffi::c_int) as COORD;
    s.origin.x = (r.corner.x as ::core::ffi::c_int
        - (r.extent.width as ::core::ffi::c_int * i as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int
                * (i as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
            >> 1 as ::core::ffi::c_int)) as COORD;
    s.origin.y = 29 as COORD;
    width = (r.extent.width as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as SIZE;
    r.extent.width = (r.extent.width as ::core::ffi::c_int * 10 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int * (10 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
        + 2 as ::core::ffi::c_int) as COORD;
    r.corner.x = (r.corner.x as ::core::ffi::c_int
        - (r.extent.width as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)) as COORD;
    r.corner.y = (r.corner.y as ::core::ffi::c_int + s.origin.y as ::core::ffi::c_int)
        as COORD;
    SetContextForeGroundColor(colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE));
    DrawFilledRectangle(&raw mut r);
    loop {
        let c2rust_fresh1 = i;
        i = i.wrapping_sub(1);
        if c2rust_fresh1 == 0 {
            break;
        }
        DrawStamp(&raw mut s);
        s.origin.x = (s.origin.x as ::core::ffi::c_int + width as ::core::ffi::c_int)
            as COORD;
    }
    SetContext(OldContext);
}
#[no_mangle]
pub unsafe extern "C" fn DrawStorageBays(mut Refresh: BOOLEAN) {
    let mut i: BYTE = 0;
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut OldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    OldContext = SetContext(StatusContext);
    r.extent.width = 2 as COORD;
    r.extent.height = 4 as COORD;
    r.corner.y = 123 as COORD;
    if Refresh.0 != 0 {
        r.extent.width = (16 as ::core::ffi::c_int
            * (r.extent.width as ::core::ffi::c_int + 1 as ::core::ffi::c_int)) as COORD;
        r.corner.x = ((64 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
            - (r.extent.width as ::core::ffi::c_int >> 1 as ::core::ffi::c_int))
            as COORD;
        SetContextForeGroundColor(colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE));
        DrawFilledRectangle(&raw mut r);
        r.extent.width = 2 as COORD;
    }
    i = CountSISPieces(C2Rust_Unnamed::STORAGE_BAY.0 as ::core::ffi::c_int as BYTE)
        as BYTE;
    if i != 0 {
        let mut j: COUNT = 0;
        r.corner.x = ((64 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
            - (i as ::core::ffi::c_int
                * (r.extent.width as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                >> 1 as ::core::ffi::c_int)) as COORD;
        SetContextForeGroundColor(colorFromRgb15(0xa as BYTE, 0xa as BYTE, 0xa as BYTE));
        j = GlobData.SIS_state.TotalElementMass;
        while (j as ::core::ffi::c_int) >= 500 as ::core::ffi::c_int {
            DrawFilledRectangle(&raw mut r);
            r.corner.x = (r.corner.x as ::core::ffi::c_int
                + (r.extent.width as ::core::ffi::c_int + 1 as ::core::ffi::c_int))
                as COORD;
            i = i.wrapping_sub(1);
            j = (j as ::core::ffi::c_int - 500 as ::core::ffi::c_int) as COUNT;
        }
        r.extent.height = ((4 as ::core::ffi::c_int * j as ::core::ffi::c_int
            + (500 as ::core::ffi::c_int - 1 as ::core::ffi::c_int))
            / 500 as ::core::ffi::c_int) as COORD;
        if r.extent.height != 0 {
            r.corner.y = (r.corner.y as ::core::ffi::c_int
                + (4 as ::core::ffi::c_int - r.extent.height as ::core::ffi::c_int))
                as COORD;
            DrawFilledRectangle(&raw mut r);
            r.extent.height = (4 as ::core::ffi::c_int
                - r.extent.height as ::core::ffi::c_int) as COORD;
            if r.extent.height != 0 {
                r.corner.y = 123 as COORD;
                SetContextForeGroundColor(
                    colorFromRgb15(0x6 as BYTE, 0x6 as BYTE, 0x6 as BYTE),
                );
                DrawFilledRectangle(&raw mut r);
            }
            r.corner.x = (r.corner.x as ::core::ffi::c_int
                + (r.extent.width as ::core::ffi::c_int + 1 as ::core::ffi::c_int))
                as COORD;
            i = i.wrapping_sub(1);
        }
        r.extent.height = 4 as COORD;
        SetContextForeGroundColor(colorFromRgb15(0x6 as BYTE, 0x6 as BYTE, 0x6 as BYTE));
        loop {
            let c2rust_fresh0 = i;
            i = i.wrapping_sub(1);
            if c2rust_fresh0 == 0 {
                break;
            }
            DrawFilledRectangle(&raw mut r);
            r.corner.x = (r.corner.x as ::core::ffi::c_int
                + (r.extent.width as ::core::ffi::c_int + 1 as ::core::ffi::c_int))
                as COORD;
        }
    }
    SetContext(OldContext);
}
#[no_mangle]
pub unsafe extern "C" fn GetGaugeRect(mut pRect: *mut RECT, mut IsCrewRect: BOOLEAN) {
    (*pRect).extent.width = 24 as COORD;
    (*pRect).corner.x = ((64 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
        - ((*pRect).extent.width as ::core::ffi::c_int >> 1 as ::core::ffi::c_int))
        as COORD;
    (*pRect).extent.height = 5 as COORD;
    (*pRect).corner.y = (if IsCrewRect.0 != 0 {
        117 as ::core::ffi::c_int
    } else {
        38 as ::core::ffi::c_int
    }) as COORD;
}
unsafe extern "C" fn DrawPC_SIS() {
    let mut t: TEXT = TEXT {
        baseline: POINT { x: 0, y: 0 },
        pStr: ::core::ptr::null::<UNICODE>(),
        align: TEXT_ALIGN::ALIGN_LEFT,
        CharCount: 0,
    };
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    GetGaugeRect(&raw mut r, BOOLEAN::FALSE);
    t.baseline.x = (64 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as COORD;
    t.baseline.y = (r.corner.y as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as COORD;
    t.align = TEXT_ALIGN::ALIGN_CENTER;
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    SetContextFont(TinyFont);
    SetContextForeGroundColor(colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE));
    r.corner.y = (r.corner.y as ::core::ffi::c_int - 6 as ::core::ffi::c_int) as COORD;
    r.corner.x -= 1;
    r.extent.width = (r.extent.width as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
        as COORD;
    DrawFilledRectangle(&raw mut r);
    SetContextFontEffect(SetAbsFrameIndex(FontGradFrame, 1 as COUNT));
    t.pStr = GetStringAddress(
        SetAbsStringTableIndex(
            GameStrings,
            (C2Rust_Unnamed_0::STATUS_STRING_BASE.0 as ::core::ffi::c_int
                + 3 as ::core::ffi::c_int) as COUNT,
        ),
    );
    font_DrawText(&raw mut t);
    r.corner.y = (r.corner.y as ::core::ffi::c_int + 79 as ::core::ffi::c_int) as COORD;
    t.baseline.y = (t.baseline.y as ::core::ffi::c_int + 79 as ::core::ffi::c_int)
        as COORD;
    DrawFilledRectangle(&raw mut r);
    SetContextFontEffect(SetAbsFrameIndex(FontGradFrame, 2 as COUNT));
    t.pStr = GetStringAddress(
        SetAbsStringTableIndex(
            GameStrings,
            (C2Rust_Unnamed_0::STATUS_STRING_BASE.0 as ::core::ffi::c_int
                + 4 as ::core::ffi::c_int) as COUNT,
        ),
    );
    font_DrawText(&raw mut t);
    SetContextFontEffect(::core::ptr::null_mut::<FRAME_DESC>());
    r.corner.x = (2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as COORD;
    r.corner.y = 3 as COORD;
    r.extent.width = 58 as COORD;
    r.extent.height = 7 as COORD;
    SetContextForeGroundColor(colorFromRgb15(0 as BYTE, 0 as BYTE, 0x14 as BYTE));
    DrawFilledRectangle(&raw mut r);
    SetContextForeGroundColor(colorFromRgb15(0x2 as BYTE, 0x4 as BYTE, 0x1e as BYTE));
    t.baseline.y = (r.corner.y as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as COORD;
    t.pStr = GetStringAddress(
        SetAbsStringTableIndex(
            GameStrings,
            (C2Rust_Unnamed_0::STATUS_STRING_BASE.0 as ::core::ffi::c_int
                + 5 as ::core::ffi::c_int) as COUNT,
        ),
    );
    font_DrawText(&raw mut t);
}
unsafe extern "C" fn DrawThrusters() {
    let mut s: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    let mut i: COUNT = 0;
    s.origin.x = 1 as COORD;
    s.origin.y = 0 as COORD;
    i = 0 as COUNT;
    while (i as ::core::ffi::c_int) < 11 as ::core::ffi::c_int {
        let mut which_piece: BYTE = GlobData.SIS_state.DriveSlots[i as usize];
        if (which_piece as ::core::ffi::c_int)
            < C2Rust_Unnamed::NUM_MODULES.0 as ::core::ffi::c_int
        {
            s.frame = SetAbsFrameIndex(
                FlagStatFrame,
                (which_piece as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                    + 0 as ::core::ffi::c_int) as COUNT,
            );
            DrawStamp(&raw mut s);
            s.frame = IncFrameIndex(s.frame);
            DrawStamp(&raw mut s);
        }
        s.origin.y = (s.origin.y as ::core::ffi::c_int - 3 as ::core::ffi::c_int)
            as COORD;
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn DrawTurningJets() {
    let mut s: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    let mut i: COUNT = 0;
    s.origin.x = 1 as COORD;
    s.origin.y = 0 as COORD;
    i = 0 as COUNT;
    while (i as ::core::ffi::c_int) < 8 as ::core::ffi::c_int {
        let mut which_piece: BYTE = GlobData.SIS_state.JetSlots[i as usize];
        if (which_piece as ::core::ffi::c_int)
            < C2Rust_Unnamed::NUM_MODULES.0 as ::core::ffi::c_int
        {
            s.frame = SetAbsFrameIndex(
                FlagStatFrame,
                (which_piece as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int) as COUNT,
            );
            DrawStamp(&raw mut s);
            s.frame = IncFrameIndex(s.frame);
            DrawStamp(&raw mut s);
        }
        s.origin.y = (s.origin.y as ::core::ffi::c_int - 3 as ::core::ffi::c_int)
            as COORD;
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn DrawModules() {
    let mut s: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    let mut i: COUNT = 0;
    s.origin.x = 1 as COORD;
    s.origin.y = 1 as COORD;
    i = 0 as COUNT;
    while (i as ::core::ffi::c_int) < 16 as ::core::ffi::c_int {
        let mut which_piece: BYTE = GlobData.SIS_state.ModuleSlots[i as usize];
        if (which_piece as ::core::ffi::c_int)
            < C2Rust_Unnamed::NUM_MODULES.0 as ::core::ffi::c_int
        {
            s.frame = SetAbsFrameIndex(
                FlagStatFrame,
                (which_piece as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int) as COUNT,
            );
            DrawStamp(&raw mut s);
        }
        s.origin.y = (s.origin.y as ::core::ffi::c_int - 3 as ::core::ffi::c_int)
            as COORD;
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn DrawSupportShips() {
    let mut hStarShip: HSHIPFRAG = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut hNextShip: HSHIPFRAG = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut pship_pos: *const POINT = ::core::ptr::null::<POINT>();
    let ship_pos: [POINT; 12] = [
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
    hStarShip = GlobData.Game_state.built_ship_q.head as HSHIPFRAG;
    pship_pos = &raw const ship_pos as *const POINT;
    while !hStarShip.is_null() {
        let mut StarShipPtr: *mut SHIP_FRAGMENT = ::core::ptr::null_mut::<
            SHIP_FRAGMENT,
        >();
        let mut s: STAMP = STAMP {
            origin: POINT { x: 0, y: 0 },
            frame: ::core::ptr::null_mut::<FRAME_DESC>(),
        };
        StarShipPtr = LockShipFrag(&raw mut GlobData.Game_state.built_ship_q, hStarShip);
        hNextShip = (*StarShipPtr).succ as HSHIPFRAG;
        s.origin = *pship_pos;
        s.frame = (*StarShipPtr).icons;
        DrawStamp(&raw mut s);
        UnlockLink(&raw mut GlobData.Game_state.built_ship_q, hStarShip);
        hStarShip = hNextShip;
        pship_pos = pship_pos.offset(1);
    }
}
unsafe extern "C" fn DeltaSISGauges_crewDelta(mut crew_delta: SIZE) {
    if crew_delta as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        return;
    }
    if crew_delta as ::core::ffi::c_int != 0x7fff as ::core::ffi::c_int {
        let mut CrewCapacity: COUNT = 0;
        if (crew_delta as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
            && GlobData.SIS_state.CrewEnlisted as ::core::ffi::c_int
                <= -(crew_delta as ::core::ffi::c_int) as COUNT as ::core::ffi::c_int
        {
            GlobData.SIS_state.CrewEnlisted = 0 as COUNT;
        } else {
            GlobData.SIS_state.CrewEnlisted = (GlobData.SIS_state.CrewEnlisted
                as ::core::ffi::c_int + crew_delta as ::core::ffi::c_int) as COUNT;
            CrewCapacity = GetCrewPodCapacity();
            if GlobData.SIS_state.CrewEnlisted as ::core::ffi::c_int
                > CrewCapacity as ::core::ffi::c_int
            {
                GlobData.SIS_state.CrewEnlisted = CrewCapacity;
            }
        }
    }
    let mut t: TEXT = TEXT {
        baseline: POINT { x: 0, y: 0 },
        pStr: ::core::ptr::null::<UNICODE>(),
        align: TEXT_ALIGN::ALIGN_LEFT,
        CharCount: 0,
    };
    let mut buf: [UNICODE; 60] = [0; 60];
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    snprintf(
        &raw mut buf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[UNICODE; 60]>(),
        b"%u\0".as_ptr() as *const ::core::ffi::c_char,
        GlobData.SIS_state.CrewEnlisted as ::core::ffi::c_int,
    );
    GetGaugeRect(&raw mut r, BOOLEAN::TRUE);
    t.baseline.x = (64 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as COORD;
    t.baseline.y = (r.corner.y as ::core::ffi::c_int
        + r.extent.height as ::core::ffi::c_int) as COORD;
    t.align = TEXT_ALIGN::ALIGN_CENTER;
    t.pStr = &raw mut buf as *mut UNICODE;
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    SetContextForeGroundColor(colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE));
    DrawFilledRectangle(&raw mut r);
    SetContextForeGroundColor(colorFromRgb15(0 as BYTE, 0xe as BYTE, 0 as BYTE));
    font_DrawText(&raw mut t);
}
unsafe extern "C" fn DeltaSISGauges_fuelDelta(mut fuel_delta: SIZE) {
    let mut old_coarse_fuel: COUNT = 0;
    let mut new_coarse_fuel: COUNT = 0;
    if fuel_delta as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        return;
    }
    if fuel_delta as ::core::ffi::c_int == 0x7fff as ::core::ffi::c_int {
        old_coarse_fuel = !(0 as ::core::ffi::c_int) as COUNT;
    } else {
        old_coarse_fuel = GlobData.SIS_state.FuelOnBoard.wrapping_div(100 as DWORD)
            as COUNT;
        if (fuel_delta as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
            && GlobData.SIS_state.FuelOnBoard
                <= -(fuel_delta as ::core::ffi::c_int) as DWORD
        {
            GlobData.SIS_state.FuelOnBoard = 0 as DWORD;
        } else {
            let mut FuelCapacity: DWORD = GetFuelTankCapacity();
            GlobData.SIS_state.FuelOnBoard = GlobData
                .SIS_state
                .FuelOnBoard
                .wrapping_add(fuel_delta as DWORD);
            if GlobData.SIS_state.FuelOnBoard > FuelCapacity {
                GlobData.SIS_state.FuelOnBoard = FuelCapacity;
            }
        }
    }
    new_coarse_fuel = GlobData.SIS_state.FuelOnBoard.wrapping_div(100 as DWORD) as COUNT;
    if new_coarse_fuel as ::core::ffi::c_int != old_coarse_fuel as ::core::ffi::c_int {
        let mut t: TEXT = TEXT {
            baseline: POINT { x: 0, y: 0 },
            pStr: ::core::ptr::null::<UNICODE>(),
            align: TEXT_ALIGN::ALIGN_LEFT,
            CharCount: 0,
        };
        let mut buf: [UNICODE; 60] = [0; 60];
        let mut r: RECT = RECT {
            corner: POINT { x: 0, y: 0 },
            extent: EXTENT { width: 0, height: 0 },
        };
        snprintf(
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[UNICODE; 60]>(),
            b"%u\0".as_ptr() as *const ::core::ffi::c_char,
            new_coarse_fuel as ::core::ffi::c_int,
        );
        GetGaugeRect(&raw mut r, BOOLEAN::FALSE);
        t.baseline.x = (64 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as COORD;
        t.baseline.y = (r.corner.y as ::core::ffi::c_int
            + r.extent.height as ::core::ffi::c_int) as COORD;
        t.align = TEXT_ALIGN::ALIGN_CENTER;
        t.pStr = &raw mut buf as *mut UNICODE;
        t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
        SetContextForeGroundColor(colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE));
        DrawFilledRectangle(&raw mut r);
        SetContextForeGroundColor(colorFromRgb15(0x13 as BYTE, 0 as BYTE, 0 as BYTE));
        font_DrawText(&raw mut t);
    }
}
unsafe extern "C" fn DeltaSISGauges_resunitDelta(mut resunit_delta: SIZE) {
    if resunit_delta as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        return;
    }
    if resunit_delta as ::core::ffi::c_int != 0x7fff as ::core::ffi::c_int {
        if (resunit_delta as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
            && GlobData.SIS_state.ResUnits
                <= -(resunit_delta as ::core::ffi::c_int) as DWORD
        {
            GlobData.SIS_state.ResUnits = 0 as DWORD;
        } else {
            GlobData.SIS_state.ResUnits = GlobData
                .SIS_state
                .ResUnits
                .wrapping_add(resunit_delta as DWORD);
        }
        (curMsgMode.0 == StatMsgMode::SMM_RES_UNITS.0
            || {
                __assert_fail(
                    b"curMsgMode == SMM_RES_UNITS\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"src/uqm/sis.c\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    1096 as ::core::ffi::c_int,
                    b"DeltaSISGauges_resunitDelta\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                0 as ::core::ffi::c_int != 0
            }) as ::core::ffi::c_int;
    } else {
        let mut r: RECT = RECT {
            corner: POINT { x: 0, y: 0 },
            extent: EXTENT { width: 0, height: 0 },
        };
        r.corner.x = 2 as COORD;
        r.corner.y = 130 as COORD;
        r.extent.width = (64 as ::core::ffi::c_int - 4 as ::core::ffi::c_int) as COORD;
        r.extent.height = 7 as COORD;
        SetContextForeGroundColor(colorFromRgb15(0 as BYTE, 0x8 as BYTE, 0 as BYTE));
        DrawFilledRectangle(&raw mut r);
    }
    DrawStatusMessage(::core::ptr::null::<UNICODE>());
}
#[no_mangle]
pub unsafe extern "C" fn DeltaSISGauges(
    mut crew_delta: SIZE,
    mut fuel_delta: SIZE,
    mut resunit_delta: ::core::ffi::c_int,
) {
    let mut OldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    if crew_delta as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        && fuel_delta as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        && resunit_delta == 0 as ::core::ffi::c_int
    {
        return;
    }
    OldContext = SetContext(StatusContext);
    BatchGraphics();
    if crew_delta as ::core::ffi::c_int == 0x7fff as ::core::ffi::c_int {
        let mut s: STAMP = STAMP {
            origin: POINT { x: 0, y: 0 },
            frame: ::core::ptr::null_mut::<FRAME_DESC>(),
        };
        s.origin.x = 0 as COORD;
        s.origin.y = 0 as COORD;
        s.frame = FlagStatFrame;
        DrawStamp(&raw mut s);
        if optWhichFonts == 0x2 as ::core::ffi::c_int {
            DrawPC_SIS();
        }
        DrawThrusters();
        DrawTurningJets();
        DrawModules();
        DrawSupportShips();
    }
    SetContextFont(TinyFont);
    DeltaSISGauges_crewDelta(crew_delta);
    DeltaSISGauges_fuelDelta(fuel_delta);
    if crew_delta as ::core::ffi::c_int == 0x7fff as ::core::ffi::c_int {
        DrawFlagshipName(BOOLEAN::TRUE);
        DrawCaptainsName();
        DrawLanders();
        DrawStorageBays(BOOLEAN::FALSE);
    }
    DeltaSISGauges_resunitDelta(resunit_delta as SIZE);
    UnbatchGraphics();
    SetContext(OldContext);
}
#[no_mangle]
pub unsafe extern "C" fn GetCrewCount() -> COUNT {
    return GlobData.SIS_state.CrewEnlisted;
}
#[no_mangle]
pub unsafe extern "C" fn GetModuleCrewCapacity(mut moduleType: BYTE) -> COUNT {
    if moduleType as ::core::ffi::c_int
        == C2Rust_Unnamed::CREW_POD.0 as ::core::ffi::c_int
    {
        return 50 as COUNT;
    }
    return 0 as COUNT;
}
#[no_mangle]
pub unsafe extern "C" fn GetCrewPodCapacity() -> COUNT {
    let mut capacity: COUNT = 0 as COUNT;
    let mut slotI: COUNT = 0;
    slotI = 0 as COUNT;
    while (slotI as ::core::ffi::c_int) < 16 as ::core::ffi::c_int {
        let mut moduleType: BYTE = GlobData.SIS_state.ModuleSlots[slotI as usize];
        capacity = (capacity as ::core::ffi::c_int
            + GetModuleCrewCapacity(moduleType) as ::core::ffi::c_int) as COUNT;
        slotI = slotI.wrapping_add(1);
    }
    return capacity;
}
unsafe extern "C" fn GetCrewPodForCrewMember(
    mut crewNr: COUNT,
    mut slotNr: *mut COUNT,
    mut seatNr: *mut COUNT,
) -> bool {
    let mut slotI: COUNT = 0;
    let mut capacity: COUNT = 0 as COUNT;
    slotI = 16 as COUNT;
    loop {
        let c2rust_fresh4 = slotI;
        slotI = slotI.wrapping_sub(1);
        if c2rust_fresh4 == 0 {
            break;
        }
        let mut moduleType: BYTE = GlobData.SIS_state.ModuleSlots[slotI as usize];
        let mut moduleCapacity: COUNT = GetModuleCrewCapacity(moduleType);
        if (crewNr as ::core::ffi::c_int)
            < capacity as ::core::ffi::c_int + moduleCapacity as ::core::ffi::c_int
        {
            *slotNr = slotI;
            *seatNr = (crewNr as ::core::ffi::c_int - capacity as ::core::ffi::c_int)
                as COUNT;
            return true;
        }
        capacity = (capacity as ::core::ffi::c_int
            + moduleCapacity as ::core::ffi::c_int) as COUNT;
    }
    return false;
}
#[no_mangle]
pub unsafe extern "C" fn GetCPodCapacity(mut ppt: *mut POINT) -> COUNT {
    let mut crewCount: COUNT = 0;
    let mut slotNr: COUNT = 0;
    let mut seatNr: COUNT = 0;
    let mut rowNr: COUNT = 0;
    let mut colNr: COUNT = 0;
    static mut crewRows: [Color; 10] = [
        Color {
            r: ((0xa as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0xa as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0x1e as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x1e as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0x9 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x9 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
        Color {
            r: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0x1e as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x1e as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
        Color {
            r: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0x1b as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x1b as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
        Color {
            r: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0x18 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x18 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
        Color {
            r: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0x15 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x15 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
        Color {
            r: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0x12 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x12 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
        Color {
            r: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0x10 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x10 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
        Color {
            r: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0xd as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0xd as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
        Color {
            r: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0xa as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0xa as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
        Color {
            r: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0x7 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x7 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
    ];
    crewCount = GetCrewCount();
    if !GetCrewPodForCrewMember(crewCount, &raw mut slotNr, &raw mut seatNr) {
        return GetCrewPodCapacity();
    }
    rowNr = (seatNr as ::core::ffi::c_int / 5 as ::core::ffi::c_int) as COUNT;
    colNr = (seatNr as ::core::ffi::c_int % 5 as ::core::ffi::c_int) as COUNT;
    if optWhichFonts == 0x2 as ::core::ffi::c_int {
        SetContextForeGroundColor(crewRows[rowNr as usize]);
    } else {
        SetContextForeGroundColor(
            colorFromRgb15(0x5 as BYTE, 0x10 as BYTE, 0x5 as BYTE),
        );
    }
    (*ppt).x = (27 as ::core::ffi::c_int
        + slotNr as ::core::ffi::c_int * 12 as ::core::ffi::c_int
        - colNr as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as COORD;
    (*ppt).y = (34 as ::core::ffi::c_int
        - rowNr as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as COORD;
    return GetCrewPodCapacity();
}
unsafe extern "C" fn GetElementMass() -> COUNT {
    return GlobData.SIS_state.TotalElementMass;
}
#[no_mangle]
pub unsafe extern "C" fn GetModuleStorageCapacity(mut moduleType: BYTE) -> COUNT {
    if moduleType as ::core::ffi::c_int
        == C2Rust_Unnamed::STORAGE_BAY.0 as ::core::ffi::c_int
    {
        return 500 as COUNT;
    }
    return 0 as COUNT;
}
#[no_mangle]
pub unsafe extern "C" fn GetStorageBayCapacity() -> COUNT {
    let mut capacity: COUNT = 0 as COUNT;
    let mut slotI: COUNT = 0;
    slotI = 0 as COUNT;
    while (slotI as ::core::ffi::c_int) < 16 as ::core::ffi::c_int {
        let mut moduleType: BYTE = GlobData.SIS_state.ModuleSlots[slotI as usize];
        capacity = (capacity as ::core::ffi::c_int
            + GetModuleStorageCapacity(moduleType) as ::core::ffi::c_int) as COUNT;
        slotI = slotI.wrapping_add(1);
    }
    return capacity;
}
unsafe extern "C" fn GetStorageCellForMineralUnit(
    mut unitNr: COUNT,
    mut slotNr: *mut COUNT,
    mut cellNr: *mut COUNT,
) -> bool {
    let mut slotI: COUNT = 0;
    let mut capacity: COUNT = 0 as COUNT;
    slotI = 16 as COUNT;
    loop {
        let c2rust_fresh5 = slotI;
        slotI = slotI.wrapping_sub(1);
        if c2rust_fresh5 == 0 {
            break;
        }
        let mut moduleType: BYTE = GlobData.SIS_state.ModuleSlots[slotI as usize];
        let mut moduleCapacity: COUNT = GetModuleStorageCapacity(moduleType);
        if unitNr as ::core::ffi::c_int
            <= capacity as ::core::ffi::c_int + moduleCapacity as ::core::ffi::c_int
        {
            *slotNr = slotI;
            *cellNr = (unitNr as ::core::ffi::c_int - capacity as ::core::ffi::c_int)
                as COUNT;
            return true;
        }
        capacity = (capacity as ::core::ffi::c_int
            + moduleCapacity as ::core::ffi::c_int) as COUNT;
    }
    return false;
}
#[no_mangle]
pub unsafe extern "C" fn GetSBayCapacity(mut ppt: *mut POINT) -> COUNT {
    let mut massCount: COUNT = 0;
    let mut slotNr: COUNT = 0;
    let mut cellNr: COUNT = 0;
    let mut rowNr: COUNT = 0;
    let mut colNr: COUNT = 0;
    static mut colorBars: [Color; 10] = [
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
            r: ((0x1c as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x1c as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0x1c as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x1c as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0x1c as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x1c as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
        Color {
            r: ((0x18 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x18 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0x18 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x18 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0x18 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x18 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
        Color {
            r: ((0x15 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x15 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0x15 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x15 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0x15 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x15 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
        Color {
            r: ((0x12 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x12 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0x12 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x12 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0x12 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x12 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
        Color {
            r: ((0x10 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x10 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0x10 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x10 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0x10 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x10 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
        Color {
            r: ((0xd as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0xd as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0xd as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0xd as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0xd as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0xd as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
        Color {
            r: ((0xa as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0xa as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0xa as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0xa as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0xa as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0xa as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
        Color {
            r: ((0x8 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x8 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0x8 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x8 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0x8 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x8 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
        Color {
            r: ((0x5 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x5 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0x5 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x5 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0x5 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x5 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
    ];
    massCount = GetElementMass();
    if !GetStorageCellForMineralUnit(massCount, &raw mut slotNr, &raw mut cellNr) {
        return GetStorageBayCapacity();
    }
    rowNr = (cellNr as ::core::ffi::c_int / 50 as ::core::ffi::c_int) as COUNT;
    colNr = (cellNr as ::core::ffi::c_int % 50 as ::core::ffi::c_int) as COUNT;
    if rowNr as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        SetContextForeGroundColor(colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE));
    } else {
        rowNr = rowNr.wrapping_sub(1);
        SetContextForeGroundColor(colorBars[rowNr as usize]);
    }
    (*ppt).x = (19 as ::core::ffi::c_int
        + slotNr as ::core::ffi::c_int * 12 as ::core::ffi::c_int) as COORD;
    (*ppt).y = (34 as ::core::ffi::c_int
        - rowNr as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as COORD;
    return GetStorageBayCapacity();
}
unsafe extern "C" fn GetFuelTotal() -> DWORD {
    return GlobData.SIS_state.FuelOnBoard;
}
#[no_mangle]
pub unsafe extern "C" fn GetModuleFuelCapacity(mut moduleType: BYTE) -> DWORD {
    if moduleType as ::core::ffi::c_int
        == C2Rust_Unnamed::FUEL_TANK.0 as ::core::ffi::c_int
    {
        return (50 as ::core::ffi::c_int * 100 as ::core::ffi::c_int) as DWORD;
    }
    if moduleType as ::core::ffi::c_int
        == C2Rust_Unnamed::HIGHEFF_FUELSYS.0 as ::core::ffi::c_int
    {
        return (100 as ::core::ffi::c_int * 100 as ::core::ffi::c_int) as DWORD;
    }
    return 0 as DWORD;
}
#[no_mangle]
pub unsafe extern "C" fn GetFuelTankCapacity() -> DWORD {
    let mut capacity: DWORD = (100 as ::core::ffi::c_int * 100 as ::core::ffi::c_int
        / 10 as ::core::ffi::c_int) as DWORD;
    let mut slotI: COUNT = 0;
    slotI = 0 as COUNT;
    while (slotI as ::core::ffi::c_int) < 16 as ::core::ffi::c_int {
        let mut moduleType: BYTE = GlobData.SIS_state.ModuleSlots[slotI as usize];
        capacity = capacity.wrapping_add(GetModuleFuelCapacity(moduleType));
        slotI = slotI.wrapping_add(1);
    }
    return capacity;
}
unsafe extern "C" fn GetFuelTankForFuelUnit(
    mut unitNr: DWORD,
    mut slotNr: *mut COUNT,
    mut compartmentNr: *mut DWORD,
) -> bool {
    let mut slotI: COUNT = 0;
    let mut capacity: DWORD = (100 as ::core::ffi::c_int * 100 as ::core::ffi::c_int
        / 10 as ::core::ffi::c_int) as DWORD;
    (unitNr
        >= (100 as ::core::ffi::c_int * 100 as ::core::ffi::c_int
            / 10 as ::core::ffi::c_int) as DWORD
        || {
            __assert_fail(
                b"unitNr >= FUEL_RESERVE\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/uqm/sis.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                1422 as ::core::ffi::c_int,
                b"GetFuelTankForFuelUnit\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    slotI = 16 as COUNT;
    loop {
        let c2rust_fresh6 = slotI;
        slotI = slotI.wrapping_sub(1);
        if c2rust_fresh6 == 0 {
            break;
        }
        let mut moduleType: BYTE = GlobData.SIS_state.ModuleSlots[slotI as usize];
        capacity = capacity.wrapping_add(GetModuleFuelCapacity(moduleType));
        if unitNr < capacity {
            *slotNr = slotI;
            *compartmentNr = capacity.wrapping_sub(unitNr);
            return true;
        }
    }
    return false;
}
#[no_mangle]
pub unsafe extern "C" fn GetFTankCapacity(mut ppt: *mut POINT) -> DWORD {
    let mut capacity: DWORD = 0;
    let mut fuelAmount: DWORD = 0;
    let mut slotNr: COUNT = 0;
    let mut compartmentNr: DWORD = 0;
    let mut moduleType: BYTE = 0;
    let mut volume: DWORD = 0;
    let mut rowNr: COUNT = 0;
    static mut fuelColors: [Color; 11] = [
        Color {
            r: ((0xf as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0xf as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
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
            r: ((0x17 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x17 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
        Color {
            r: ((0x1b as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x1b as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
        Color {
            r: ((0x1f as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x1f as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0x3 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x3 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
        Color {
            r: ((0x1f as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x1f as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0x7 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x7 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
        Color {
            r: ((0x1f as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x1f as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0xa as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0xa as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
        Color {
            r: ((0x1f as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x1f as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0xe as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0xe as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
        Color {
            r: ((0x1f as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x1f as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0x11 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x11 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
        Color {
            r: ((0x1f as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x1f as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0x15 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x15 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
        Color {
            r: ((0x1f as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x1f as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0x18 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x18 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
    ];
    capacity = GetFuelTankCapacity();
    fuelAmount = GetFuelTotal();
    if fuelAmount
        < (100 as ::core::ffi::c_int * 100 as ::core::ffi::c_int
            / 10 as ::core::ffi::c_int) as DWORD
    {
        return capacity;
    }
    if !GetFuelTankForFuelUnit(fuelAmount, &raw mut slotNr, &raw mut compartmentNr) {
        return capacity;
    }
    moduleType = GlobData.SIS_state.ModuleSlots[slotNr as usize];
    volume = GetModuleFuelCapacity(moduleType);
    rowNr = volume
        .wrapping_sub(compartmentNr)
        .wrapping_mul(10 as DWORD)
        .wrapping_div((100 as ::core::ffi::c_int * 100 as ::core::ffi::c_int) as DWORD)
        as COUNT;
    (*ppt).x = (21 as ::core::ffi::c_int
        + slotNr as ::core::ffi::c_int * 12 as ::core::ffi::c_int) as COORD;
    if volume == (50 as ::core::ffi::c_int * 100 as ::core::ffi::c_int) as DWORD {
        (*ppt).y = (27 as ::core::ffi::c_int - rowNr as ::core::ffi::c_int) as COORD;
    } else {
        (*ppt).y = (30 as ::core::ffi::c_int - rowNr as ::core::ffi::c_int) as COORD;
    }
    ((rowNr as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
        < ::core::mem::size_of::<[Color; 11]>()
            .wrapping_div(::core::mem::size_of::<Color>()) as COUNT as ::core::ffi::c_int
        || {
            __assert_fail(
                b"rowNr + 1 < (COUNT) (sizeof fuelColors / sizeof fuelColors[0])\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"src/uqm/sis.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                1484 as ::core::ffi::c_int,
                b"GetFTankCapacity\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    SetContextForeGroundColor(fuelColors[rowNr as usize]);
    SetContextBackGroundColor(
        fuelColors[(rowNr as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize],
    );
    return capacity;
}
#[no_mangle]
pub unsafe extern "C" fn CountSISPieces(mut piece_type: BYTE) -> COUNT {
    let mut i: COUNT = 0;
    let mut num_pieces: COUNT = 0;
    num_pieces = 0 as COUNT;
    if piece_type as ::core::ffi::c_int
        == C2Rust_Unnamed::FUSION_THRUSTER.0 as ::core::ffi::c_int
    {
        i = 0 as COUNT;
        while (i as ::core::ffi::c_int) < 11 as ::core::ffi::c_int {
            if GlobData.SIS_state.DriveSlots[i as usize] as ::core::ffi::c_int
                == piece_type as ::core::ffi::c_int
            {
                num_pieces = num_pieces.wrapping_add(1);
            }
            i = i.wrapping_add(1);
        }
    } else if piece_type as ::core::ffi::c_int
        == C2Rust_Unnamed::TURNING_JETS.0 as ::core::ffi::c_int
    {
        i = 0 as COUNT;
        while (i as ::core::ffi::c_int) < 8 as ::core::ffi::c_int {
            if GlobData.SIS_state.JetSlots[i as usize] as ::core::ffi::c_int
                == piece_type as ::core::ffi::c_int
            {
                num_pieces = num_pieces.wrapping_add(1);
            }
            i = i.wrapping_add(1);
        }
    } else {
        i = 0 as COUNT;
        while (i as ::core::ffi::c_int) < 16 as ::core::ffi::c_int {
            if GlobData.SIS_state.ModuleSlots[i as usize] as ::core::ffi::c_int
                == piece_type as ::core::ffi::c_int
            {
                num_pieces = num_pieces.wrapping_add(1);
            }
            i = i.wrapping_add(1);
        }
    }
    return num_pieces;
}
#[no_mangle]
pub unsafe extern "C" fn DrawAutoPilotMessage(mut Reset: BOOLEAN) {
    static mut LastPilot: BOOLEAN = BOOLEAN::FALSE;
    static mut NextTime: TimeCount = 0 as TimeCount;
    static mut cycle_index: DWORD = 0 as DWORD;
    let mut OnAutoPilot: BOOLEAN = BOOLEAN::FALSE;
    static mut cycle_tab: [Color; 7] = [
        Color {
            r: ((0xa as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0xa as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0x14 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x14 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0x18 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x18 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
        Color {
            r: ((0x6 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x6 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0x10 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x10 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0x16 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x16 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
        Color {
            r: ((0x3 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x3 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0xe as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0xe as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0x14 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x14 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
        Color {
            r: ((0x2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0xc as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0xc as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0x11 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x11 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
        Color {
            r: ((0x1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x1 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0xb as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0xb as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0xf as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0xf as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
        Color {
            r: ((0x1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x1 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0x9 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x9 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0xd as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0xd as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
        Color {
            r: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0x7 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x7 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0xb as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0xb as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
    ];
    let cycleCount: size_t = ::core::mem::size_of::<[Color; 7]>()
        .wrapping_div(::core::mem::size_of::<Color>());
    if Reset.0 != 0 {
        LastPilot = BOOLEAN::FALSE;
        return;
    }
    OnAutoPilot = BOOLEAN(
        (GlobData.Game_state.autopilot.x as ::core::ffi::c_int
            != !(0 as ::core::ffi::c_int)
            && GlobData.Game_state.autopilot.y as ::core::ffi::c_int
                != !(0 as ::core::ffi::c_int)
            || GlobData.SIS_state.FuelOnBoard == 0 as DWORD) as ::core::ffi::c_int
            as ::core::ffi::c_uint,
    );
    if OnAutoPilot.0 != 0 || LastPilot.0 != 0 {
        if OnAutoPilot.0 == 0 {
            DrawSISMessage(::core::ptr::null::<UNICODE>());
            cycle_index = 0 as DWORD;
        } else if GetTimeCounter() >= NextTime {
            if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
                & C2Rust_Unnamed_2::CHECK_ABORT.0 as ::core::ffi::c_int == 0
                && GlobData.SIS_state.CrewEnlisted as ::core::ffi::c_int
                    != !(0 as ::core::ffi::c_int) as COUNT as ::core::ffi::c_int
            {
                let mut OldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
                OldContext = SetContext(OffScreenContext);
                SetContextForeGroundColor(cycle_tab[cycle_index as usize]);
                if GlobData.SIS_state.FuelOnBoard == 0 as DWORD {
                    DrawSISMessageEx(
                        GetStringAddress(
                            SetAbsStringTableIndex(
                                GameStrings,
                                (C2Rust_Unnamed_0::NAVIGATION_STRING_BASE.0
                                    as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as COUNT,
                            ),
                        ),
                        -1 as SIZE,
                        -1 as SIZE,
                        ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as COUNT,
                    );
                } else {
                    DrawSISMessageEx(
                        GetStringAddress(
                            SetAbsStringTableIndex(
                                GameStrings,
                                (C2Rust_Unnamed_0::NAVIGATION_STRING_BASE.0
                                    as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as COUNT,
                            ),
                        ),
                        -1 as SIZE,
                        -1 as SIZE,
                        ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as COUNT,
                    );
                }
                SetContext(OldContext);
            }
            cycle_index = (cycle_index.wrapping_add(1 as DWORD) as size_t)
                .wrapping_rem(cycleCount) as DWORD;
            NextTime = GetTimeCounter()
                .wrapping_add(
                    (840 as ::core::ffi::c_int * 3 as ::core::ffi::c_int
                        / 40 as ::core::ffi::c_int) as TimeCount,
                );
        }
        LastPilot = OnAutoPilot;
    }
}
static mut flashContext: *mut FlashContext = ::core::ptr::null_mut::<FlashContext>();
static mut flash_rect: RECT = RECT {
    corner: POINT { x: 0, y: 0 },
    extent: EXTENT { width: 0, height: 0 },
};
static mut flashAlarm: *mut Alarm = ::core::ptr::null_mut::<Alarm>();
static mut flashPaused: BOOLEAN = BOOLEAN::FALSE;
unsafe extern "C" fn updateFlashRect(mut arg: *mut ::core::ffi::c_void) {
    if flashContext.is_null() {
        return;
    }
    Flash_process(flashContext);
    scheduleFlashAlarm();
}
unsafe extern "C" fn scheduleFlashAlarm() {
    let mut nextTime: TimeCount = Flash_nextTime(flashContext);
    let mut nextTimeMs: DWORD = nextTime
        .wrapping_div(840 as DWORD)
        .wrapping_mul(1000 as DWORD)
        .wrapping_add(
            nextTime
                .wrapping_rem(840 as DWORD)
                .wrapping_mul(1000 as DWORD)
                .wrapping_div(840 as DWORD),
        );
    flashAlarm = Alarm_addAbsoluteMs(
        nextTimeMs,
        Some(updateFlashRect as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()),
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn SetFlashRect(mut pRect: *const RECT) {
    let mut clip_r: RECT = rect {
        corner: point {
            x: 0 as COORD,
            y: 0 as COORD,
        },
        extent: extent {
            width: 0 as COORD,
            height: 0 as COORD,
        },
    };
    let mut temp_r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    if pRect
        != ::core::ptr::with_exposed_provenance_mut::<
            RECT,
        >(!(0 as ::core::ffi::c_long) as usize) as *const RECT
        && pRect
            != ::core::ptr::with_exposed_provenance_mut::<
                RECT,
            >(!(1 as ::core::ffi::c_long) as usize) as *const RECT
    {
        GetContextClipRect(&raw mut clip_r);
    } else if optWhichMenu == 0x2 as ::core::ffi::c_int
        && pRect
            != ::core::ptr::with_exposed_provenance_mut::<
                RECT,
            >(!(1 as ::core::ffi::c_long) as usize) as *const RECT
    {
        pRect = ::core::ptr::null::<RECT>();
    } else {
        let mut OldContext: CONTEXT = SetContext(StatusContext);
        GetContextClipRect(&raw mut clip_r);
        pRect = &raw mut temp_r;
        temp_r.corner.x = (4 as ::core::ffi::c_int
            + (ScreenWidth - 64 as ::core::ffi::c_int - 0 as ::core::ffi::c_int)
            - clip_r.corner.x as ::core::ffi::c_int) as COORD;
        temp_r.corner.y = (10 as ::core::ffi::c_int + 0 as ::core::ffi::c_int
            + (ScreenHeight - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                - 13 as ::core::ffi::c_int) - 53 as ::core::ffi::c_int
            - clip_r.corner.y as ::core::ffi::c_int) as COORD;
        temp_r.extent.width = (64 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)
            as COORD;
        temp_r.extent.height = 53 as COORD;
        SetContext(OldContext);
    }
    if !pRect.is_null()
        && (*pRect).extent.width as ::core::ffi::c_int != 0 as ::core::ffi::c_int
    {
        flash_rect = *pRect;
        flash_rect.corner.x = (flash_rect.corner.x as ::core::ffi::c_int
            + clip_r.corner.x as ::core::ffi::c_int) as COORD;
        flash_rect.corner.y = (flash_rect.corner.y as ::core::ffi::c_int
            + clip_r.corner.y as ::core::ffi::c_int) as COORD;
        if flashContext.is_null() {
            flashContext = Flash_createHighlight(ScreenContext, &raw mut flash_rect);
            Flash_setMergeFactors(
                flashContext,
                3 as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
            );
            Flash_setSpeed(
                flashContext,
                0 as TimeCount,
                (840 as ::core::ffi::c_int / 16 as ::core::ffi::c_int) as TimeCount,
                0 as TimeCount,
                (840 as ::core::ffi::c_int / 16 as ::core::ffi::c_int) as TimeCount,
            );
            Flash_setFrameTime(
                flashContext,
                (840 as ::core::ffi::c_int / 16 as ::core::ffi::c_int) as TimeCount,
            );
            Flash_start(flashContext);
            scheduleFlashAlarm();
        } else {
            Flash_setRect(flashContext, &raw mut flash_rect);
        }
    } else if !flashContext.is_null() {
        Alarm_remove(flashAlarm);
        flashAlarm = ::core::ptr::null_mut::<Alarm>();
        Flash_terminate(flashContext);
        flashContext = ::core::ptr::null_mut::<FlashContext>();
    }
}
#[no_mangle]
pub static mut updateFlashRectRecursion: COUNT = 0 as COUNT;
#[no_mangle]
pub unsafe extern "C" fn PreUpdateFlashRect() {
    if !flashAlarm.is_null() {
        updateFlashRectRecursion = updateFlashRectRecursion.wrapping_add(1);
        if updateFlashRectRecursion as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
            return;
        }
        Flash_preUpdate(flashContext);
    }
}
#[no_mangle]
pub unsafe extern "C" fn PostUpdateFlashRect() {
    if !flashAlarm.is_null() {
        updateFlashRectRecursion = updateFlashRectRecursion.wrapping_sub(1);
        if updateFlashRectRecursion as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
            return;
        }
        Flash_postUpdate(flashContext);
    }
}
#[no_mangle]
pub unsafe extern "C" fn PauseFlash() {
    if !flashContext.is_null() {
        Alarm_remove(flashAlarm);
        flashAlarm = ::core::ptr::null_mut::<Alarm>();
        flashPaused = BOOLEAN::TRUE;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ContinueFlash() {
    if flashPaused.0 != 0 {
        scheduleFlashAlarm();
        flashPaused = BOOLEAN::FALSE;
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
                b"src/uqm/races.h\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                329 as ::core::ffi::c_int,
                b"LockShipFrag\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    return LockLink(pq, h) as *mut SHIP_FRAGMENT;
}
