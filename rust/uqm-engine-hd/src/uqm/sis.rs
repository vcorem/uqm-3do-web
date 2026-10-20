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
    fn pow(_: ::core::ffi::c_double, _: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn sqrt(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn SetContext(Context: CONTEXT) -> CONTEXT;
    fn SetContextForeGroundColor(Color: Color) -> Color;
    fn SetContextBackGroundColor(Color: Color) -> Color;
    fn SetContextFGFrame(Frame: FRAME) -> FRAME;
    fn SetContextClipRect(pRect: *mut RECT) -> BOOLEAN;
    fn GetContextClipRect(pRect: *mut RECT) -> BOOLEAN;
    fn DrawStamp(pStamp: *mut STAMP);
    fn DrawFilledRectangle(pRect: *mut RECT);
    fn font_GetTextRect(pText: *mut TEXT) -> RECT;
    fn font_DrawText(pText: *mut TEXT);
    fn BatchGraphics();
    fn UnbatchGraphics();
    fn ClearDrawable();
    fn GetFrameRect(Frame: FRAME, pRect: *mut RECT) -> BOOLEAN;
    fn GetFrameHot(Frame: FRAME) -> HOT_SPOT;
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
    ) -> !;
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
    fn skipUTF8Chars(
        ptr: *const ::core::ffi::c_char,
        num: size_t,
    ) -> *mut ::core::ffi::c_char;
    fn GetTimeCounter() -> TimeCount;
    fn strupr(_: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    static mut optWhichMenu: ::core::ffi::c_int;
    static mut optWhichFonts: ::core::ffi::c_int;
    static mut resolutionFactor: ::core::ffi::c_uint;
    static mut optBubbleWarp: OPT_ENABLABLE;
    static mut optInfiniteRU: OPT_ENABLABLE;
    static mut optDateFormat: ::core::ffi::c_int;
    static mut optInfiniteFuel: OPT_ENABLABLE;
    static mut optInfiniteCredits: OPT_ENABLABLE;
    static mut optCustomBorder: OPT_ENABLABLE;
    static mut optWholeFuel: OPT_ENABLABLE;
    static mut optSmartAutoPilot: OPT_ENABLABLE;
    static mut optFlagshipColor: ::core::ffi::c_int;
    static mut optWindowType: DWORD;
    static mut CanvasWidth: ::core::ffi::c_int;
    static mut CanvasHeight: ::core::ffi::c_int;
    static mut GlobData: GLOBDATA;
    fn inHyperSpace() -> BOOLEAN;
    fn inQuasiSpace() -> BOOLEAN;
    fn isPC(optWhich: ::core::ffi::c_int) -> OPT_CONSOLETYPE;
    fn is3DO(optWhich: ::core::ffi::c_int) -> OPT_CONSOLETYPE;
    fn replaceChar(
        pStr: *mut ::core::ffi::c_char,
        find: ::core::ffi::c_char,
        replace: ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn setGameStateUint(name: *const ::core::ffi::c_char, val: DWORD);
    fn getGameStateUint(name: *const ::core::ffi::c_char) -> DWORD;
    static mut CurStarDescPtr: *mut STAR_DESC;
    fn FindStar(
        pLastStar: *mut STAR_DESC,
        puniverse: *mut POINT,
        xbounds: SIZE,
        ybounds: SIZE,
    ) -> *mut STAR_DESC;
    fn GetClusterName(pSD: *const STAR_DESC, buf: *mut UNICODE);
    fn Flash_createHighlight(
        gfxContext: CONTEXT,
        rect: *const RECT,
    ) -> *mut FlashContext;
    fn Flash_start(context: *mut FlashContext);
    fn Flash_terminate(context: *mut FlashContext);
    fn Flash_pause(context: *mut FlashContext);
    fn Flash_continue(context: *mut FlashContext);
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
    fn Flash_setPulseBox(context: *mut FlashContext, isPulsing: BOOLEAN);
    fn Flash_getPulseBox(context: *mut FlashContext) -> BOOLEAN;
    fn Flash_nextTime(context: *mut FlashContext) -> TimeCount;
    fn Flash_setRect(context: *mut FlashContext, rect: *const RECT);
    fn Flash_preUpdate(context: *mut FlashContext);
    fn Flash_postUpdate(context: *mut FlashContext);
    fn DrawMenuStateStrings(beg_index: BYTE, NewState: SWORD);
    fn DrawBorder(Visible: BYTE);
    static mut GameStrings: STRING;
    fn SetCursorRect(r: *mut RECT, context: CONTEXT);
    fn SetCursorFlashBlock(state: BOOLEAN);
    fn Alarm_addAbsoluteMs(
        ms: uint32,
        callback: AlarmCallback,
        arg: AlarmCallbackArg,
    ) -> *mut Alarm;
    fn Alarm_remove(alarm: *mut Alarm);
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
    static mut TinyFontBold: FONT;
    static mut OutfitOrShipyard: BYTE;
    static mut SaveOrLoad: BOOLEAN;
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
    pub flags: BYTE,
}
pub type HOT_SPOT = POINT;
pub type TimeCount = DWORD;
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
    pub stowed_ship_q: QUEUE,
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
    pub const SIS_MAIN_FRAME: Self = Self(0);
    pub const SIS_STAT_FRAME: Self = Self(1);
    pub const DEVICE_CARGO_FRAME: Self = Self(2);
    pub const SIS_STAT_REPAIR_FRAME: Self = Self(3);
    pub const SIS_STAT_SUMM_FRAME: Self = Self(4);
    pub const SIS_MSG_FRAME: Self = Self(5);
    pub const SIS_TITLE_FRAME: Self = Self(6);
    pub const CAPTAIN_FRAME: Self = Self(7);
    pub const CAP_NAME_FRAME: Self = Self(8);
    pub const STAT_MSG_FRAME: Self = Self(9);
    pub const SIS_RADAR_FRAME: Self = Self(10);
    pub const TEXT_LABEL_FRAME: Self = Self(11);
    pub const SIS_REPAIR_FRAME: Self = Self(12);
    pub const LANDER_3DO_FRAME: Self = Self(13);
    pub const LANDER_DOS_FRAME: Self = Self(14);
    pub const SIS_COMM_FRAME: Self = Self(15);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_1(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_1 {
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
pub struct C2Rust_Unnamed_2(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_2 {
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
unsafe extern "C" fn MAKE_POINT(mut x: COORD, mut y: COORD) -> POINT {
    let mut pt: POINT = point { x: x, y: y };
    return pt;
}
#[inline]
unsafe extern "C" fn pointsEqual(mut p1: POINT, mut p2: POINT) -> bool {
    return p1.x as ::core::ffi::c_int == p2.x as ::core::ffi::c_int
        && p1.y as ::core::ffi::c_int == p2.y as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn ptDistance(mut p1: POINT, mut p2: POINT) -> ::core::ffi::c_double {
    return sqrt(
        pow(
            p2.x as ::core::ffi::c_double - p1.x as ::core::ffi::c_double,
            2 as ::core::ffi::c_int as ::core::ffi::c_double,
        )
            + pow(
                p2.y as ::core::ffi::c_double - p1.y as ::core::ffi::c_double,
                2 as ::core::ffi::c_int as ::core::ffi::c_double,
            ),
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
static mut curMsgMode: StatMsgMode = StatMsgMode::SMM_DEFAULT;
#[no_mangle]
pub static mut hdFuelFrame: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
#[no_mangle]
pub unsafe extern "C" fn RepairSISBorder() {
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut OldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    OldContext = SetContext(ScreenContext);
    BatchGraphics();
    r.corner.x = (((6 as ::core::ffi::c_int) << resolutionFactor)
        + if if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            (if if optWindowType == 1 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                16 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } << resolutionFactor) + ((1 as ::core::ffi::c_int) << resolutionFactor)
        } else {
            0 as ::core::ffi::c_int
        } - ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    r.corner.y = (((9 as ::core::ffi::c_int) << resolutionFactor)
        + if if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            (if if optWindowType == 1 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                16 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } << resolutionFactor) + ((1 as ::core::ffi::c_int) << resolutionFactor)
        } else {
            0 as ::core::ffi::c_int
        } - ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    r.extent.width = ((1 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    r.extent.height = (CanvasHeight
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
        + ((2 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    SetContextForeGroundColor(colorFromRgb15(0x10 as BYTE, 0x10 as BYTE, 0x10 as BYTE));
    uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
    SetContextForeGroundColor(colorFromRgb15(0x8 as BYTE, 0x8 as BYTE, 0x8 as BYTE));
    r.corner.x = (r.corner.x as ::core::ffi::c_int
        + (CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
                } << resolutionFactor)) + ((2 as ::core::ffi::c_int) << resolutionFactor)
            - ((1 as ::core::ffi::c_int) << resolutionFactor))) as COORD;
    uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
    r.corner.x = (((6 as ::core::ffi::c_int) << resolutionFactor)
        + if if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            (if if optWindowType == 1 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                16 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } << resolutionFactor) + ((1 as ::core::ffi::c_int) << resolutionFactor)
        } else {
            0 as ::core::ffi::c_int
        } - ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    r.corner.y = (r.corner.y as ::core::ffi::c_int
        + (CanvasHeight
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
            + ((2 as ::core::ffi::c_int) << resolutionFactor)
            - ((1 as ::core::ffi::c_int) << resolutionFactor))) as COORD;
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
            } << resolutionFactor)) + ((1 as ::core::ffi::c_int) << resolutionFactor))
        as COORD;
    r.extent.height = ((1 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
    uqm_lua_checked!(DrawBorder(C2Rust_Unnamed_0::SIS_REPAIR_FRAME.0 as ::core::ffi::c_int as BYTE), ());
    UnbatchGraphics();
    SetContext(OldContext);
}
#[no_mangle]
pub unsafe extern "C" fn ClearSISRect(mut ClearFlags: BYTE) {
    let mut OldColor: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    let mut OldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    OldContext = SetContext(StatusContext);
    OldColor = SetContextForeGroundColor(
        colorFromRgb15(0xa as BYTE, 0xa as BYTE, 0xa as BYTE),
    );
    BatchGraphics();
    if ClearFlags as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int != 0
    {
        uqm_lua_checked!(DeltaSISGauges(0x7fff as SIZE, 0x7fff as SDWORD, 0x7fff as ::core::ffi::c_int), ());
    }
    if ClearFlags as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int != 0
    {
        uqm_lua_checked!(DrawMenuStateStrings(!(0 as ::core::ffi::c_int) as BYTE, 1 as SWORD), ());
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
    t.baseline.x = ((((57 as ::core::ffi::c_int) << resolutionFactor)
        - ((2 as ::core::ffi::c_int) << resolutionFactor) >> resolutionFactor
        >> 1 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    t.baseline.y = (((8 as ::core::ffi::c_int) << resolutionFactor)
        - ((2 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    t.align = TEXT_ALIGN::ALIGN_CENTER;
    t.pStr = pStr;
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    OldContext = SetContext(OffScreenContext);
    r.corner.x = (((6 as ::core::ffi::c_int) << resolutionFactor)
        + if if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            (if if optWindowType == 1 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                16 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } << resolutionFactor) + ((1 as ::core::ffi::c_int) << resolutionFactor)
        } else {
            0 as ::core::ffi::c_int
        }
        + (CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
                } << resolutionFactor)))
        - ((57 as ::core::ffi::c_int) << resolutionFactor)
        + ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    r.corner.y = (((9 as ::core::ffi::c_int) << resolutionFactor)
        + if if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            (if if optWindowType == 1 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                16 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } << resolutionFactor) + ((1 as ::core::ffi::c_int) << resolutionFactor)
        } else {
            0 as ::core::ffi::c_int
        } - ((8 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    r.extent.width = (((57 as ::core::ffi::c_int) << resolutionFactor)
        - ((2 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    r.extent.height = (((8 as ::core::ffi::c_int) << resolutionFactor)
        - ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    SetContextFGFrame(Screen);
    SetContextClipRect(&raw mut r);
    if isPC(optWhichFonts).0 != 0 || SaveOrLoad.0 != 0 {
        SetContextFont(TinyFont);
    } else {
        let mut buf: *mut UNICODE = pStr;
        SetContextFont(TinyFontBold);
        replaceChar(buf, 0x20 as ::core::ffi::c_char, 0x9 as ::core::ffi::c_char);
        t.pStr = buf;
        t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    }
    BatchGraphics();
    SetContextBackGroundColor(colorFromRgb15(0 as BYTE, 0 as BYTE, 0x14 as BYTE));
    uqm_lua_checked!(ClearDrawable(), ());
    uqm_lua_checked!(DrawBorder(C2Rust_Unnamed_0::SIS_TITLE_FRAME.0 as ::core::ffi::c_int as BYTE), ());
    SetContextForeGroundColor(colorFromRgb15(0x1b as BYTE, 0 as BYTE, 0x1b as BYTE));
    uqm_lua_checked!(font_DrawText(&raw mut t), ());
    UnbatchGraphics();
    SetContextClipRect(::core::ptr::null_mut::<RECT>());
    SetContext(OldContext);
}
#[no_mangle]
pub unsafe extern "C" fn DrawHyperCoords(mut universe: POINT) {
    let mut buf: [UNICODE; 100] = [0; 100];
    let mut SpaceOrNull: *mut ::core::ffi::c_char = (if isPC(optWhichFonts).0 != 0 {
        b" \0".as_ptr() as *const ::core::ffi::c_char
    } else {
        b"\0".as_ptr() as *const ::core::ffi::c_char
    }) as *mut ::core::ffi::c_char;
    snprintf(
        &raw mut buf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[UNICODE; 100]>(),
        b"%03u.%01u%s:%s%03u.%01u\0".as_ptr() as *const ::core::ffi::c_char,
        universe.x as ::core::ffi::c_int / 10 as ::core::ffi::c_int,
        universe.x as ::core::ffi::c_int % 10 as ::core::ffi::c_int,
        SpaceOrNull,
        SpaceOrNull,
        universe.y as ::core::ffi::c_int / 10 as ::core::ffi::c_int,
        universe.y as ::core::ffi::c_int % 10 as ::core::ffi::c_int,
    );
    uqm_lua_checked!(DrawSISTitle(&raw mut buf as *mut UNICODE), ());
}
#[no_mangle]
pub unsafe extern "C" fn DrawSaveInfo(mut SisState: SIS_STATE) {
    let mut buf: [UNICODE; 100] = [0; 100];
    let mut TempDiff: [UNICODE; 11] = [0; 11];
    let mut TempExt: [UNICODE; 12] = ::core::mem::transmute::<
        [u8; 12],
        [UNICODE; 12],
    >(*b"\0\0\0\0\0\0\0\0\0\0\0\0");
    let mut TempNom: [UNICODE; 10] = ::core::mem::transmute::<
        [u8; 10],
        [UNICODE; 10],
    >(*b"\0\0\0\0\0\0\0\0\0\0");
    let mut TempVer: [UNICODE; 32] = ::core::mem::transmute::<
        [u8; 32],
        [UNICODE; 32],
    >(*b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0");
    if SisState.SaveVersion as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
        utf8StringCopy(
            &raw mut TempVer as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[UNICODE; 32]>(),
            GetStringAddress(
                SetAbsStringTableIndex(
                    GameStrings,
                    (C2Rust_Unnamed_2::SAVEGAME_STRING_BASE.0 as ::core::ffi::c_int
                        + 4 as ::core::ffi::c_int
                        + SisState.SaveVersion as ::core::ffi::c_int) as COUNT,
                ),
            ),
        );
    }
    uqm_lua_checked!(DrawSISMessage(&raw mut TempVer as *mut UNICODE), ());
    uqm_lua_checked!(DrawSISTitle(b"\0".as_ptr() as *const ::core::ffi::c_char as *mut UNICODE), ());
    if SisState.Seed != 0 {
        if SisState.Nomad != 0 {
            snprintf(
                &raw mut TempNom as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[UNICODE; 10]>(),
                b"%s%s\0".as_ptr() as *const ::core::ffi::c_char,
                GetStringAddress(
                    SetAbsStringTableIndex(
                        GameStrings,
                        (C2Rust_Unnamed_2::MAINMENU_STRING_BASE.0 as ::core::ffi::c_int
                            + 60 as ::core::ffi::c_int) as COUNT,
                    ),
                ),
                if SisState.Nomad as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
                    b"+\0".as_ptr() as *const ::core::ffi::c_char
                } else if SisState.Nomad as ::core::ffi::c_int == 1 as ::core::ffi::c_int
                {
                    b"-\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"\0".as_ptr() as *const ::core::ffi::c_char
                },
            );
        }
        if SisState.Extended != 0 {
            utf8StringCopy(
                &raw mut TempExt as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[UNICODE; 12]>(),
                GetStringAddress(
                    SetAbsStringTableIndex(
                        GameStrings,
                        (C2Rust_Unnamed_2::MAINMENU_STRING_BASE.0 as ::core::ffi::c_int
                            + 59 as ::core::ffi::c_int) as COUNT,
                    ),
                ),
            );
        }
        utf8StringCopy(
            &raw mut TempDiff as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[UNICODE; 11]>(),
            GetStringAddress(
                SetAbsStringTableIndex(
                    GameStrings,
                    (C2Rust_Unnamed_2::MAINMENU_STRING_BASE.0 as ::core::ffi::c_int
                        + 56 as ::core::ffi::c_int
                        + SisState.Difficulty as ::core::ffi::c_int) as COUNT,
                ),
            ),
        );
        snprintf(
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[UNICODE; 100]>(),
            b"%s %s%s%s\0".as_ptr() as *const ::core::ffi::c_char,
            GetStringAddress(
                SetAbsStringTableIndex(
                    GameStrings,
                    (C2Rust_Unnamed_2::MAINMENU_STRING_BASE.0 as ::core::ffi::c_int
                        + 55 as ::core::ffi::c_int) as COUNT,
                ),
            ),
            &raw mut TempDiff as *mut UNICODE,
            &raw mut TempExt as *mut UNICODE,
            &raw mut TempNom as *mut UNICODE,
        );
        uqm_lua_checked!(DrawSISMessage(&raw mut buf as *mut UNICODE), ());
        snprintf(
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[UNICODE; 100]>(),
            b"%u\0".as_ptr() as *const ::core::ffi::c_char,
            SisState.Seed,
        );
        uqm_lua_checked!(DrawSISTitle(&raw mut buf as *mut UNICODE), ());
    }
}
#[no_mangle]
pub unsafe extern "C" fn DrawSISMessage(mut pStr: *const UNICODE) {
    uqm_lua_checked!(DrawSISMessageEx(pStr, -1 as SIZE, -1 as SIZE, 0 as COUNT), ());
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
    r.corner.x = (((6 as ::core::ffi::c_int) << resolutionFactor)
        + if if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            (if if optWindowType == 1 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                16 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } << resolutionFactor) + ((1 as ::core::ffi::c_int) << resolutionFactor)
        } else {
            0 as ::core::ffi::c_int
        } + ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    r.corner.y = (((9 as ::core::ffi::c_int) << resolutionFactor)
        + if if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            (if if optWindowType == 1 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                16 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } << resolutionFactor) + ((1 as ::core::ffi::c_int) << resolutionFactor)
        } else {
            0 as ::core::ffi::c_int
        } - ((8 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
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
            } << resolutionFactor)) - ((57 as ::core::ffi::c_int) << resolutionFactor)
        - ((12 as ::core::ffi::c_int) << resolutionFactor)
        - ((2 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    r.extent.height = (((8 as ::core::ffi::c_int) << resolutionFactor)
        - ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
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
                if uqm_lua_checked!(inHyperSpace(), BOOLEAN(0)).0 != 0 {
                    pStr = GetStringAddress(
                        SetAbsStringTableIndex(
                            GameStrings,
                            C2Rust_Unnamed_2::NAVIGATION_STRING_BASE.0
                                as ::core::ffi::c_int as COUNT,
                        ),
                    );
                } else {
                    let mut Log: POINT = MAKE_POINT(
                        logxToUniverse(GlobData.SIS_state.log_x) as COORD,
                        logyToUniverse(GlobData.SIS_state.log_y) as COORD,
                    );
                    pStr = GetStringAddress(
                        SetAbsStringTableIndex(
                            GameStrings,
                            (C2Rust_Unnamed_2::NAVIGATION_STRING_BASE.0
                                as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as COUNT,
                        ),
                    );
                    if uqm_lua_checked!(getGameStateUint(
                        b"ARILOU_HOME_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
                    ), BOOLEAN(0)) != 0
                        && (Log.x as ::core::ffi::c_int == 6134 as ::core::ffi::c_int
                            && Log.y as ::core::ffi::c_int == 5900 as ::core::ffi::c_int)
                    {
                        utf8StringCopy(
                            &raw mut GlobData.SIS_state.PlanetName
                                as *mut ::core::ffi::c_char,
                            ::core::mem::size_of::<[UNICODE; 32]>(),
                            GetStringAddress(
                                SetAbsStringTableIndex(
                                    GameStrings,
                                    (C2Rust_Unnamed_2::STAR_STRING_BASE.0 as ::core::ffi::c_int
                                        + 148 as ::core::ffi::c_int) as COUNT,
                                ),
                            ),
                        );
                    }
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
    t.baseline.y = (((8 as ::core::ffi::c_int) << resolutionFactor)
        - ((2 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
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
            } << resolutionFactor)) - ((57 as ::core::ffi::c_int) << resolutionFactor)
        - ((12 as ::core::ffi::c_int) << resolutionFactor)
        - ((2 as ::core::ffi::c_int) << resolutionFactor) >> resolutionFactor
        >> 1 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    t.pStr = pStr;
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    if isPC(optWhichFonts).0 != 0 || SaveOrLoad.0 != 0 {
        SetContextFont(TinyFont);
    } else {
        SetContextFont(TinyFontBold);
        if (CurPos as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
            && (ExPos as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
        {
            let mut buf_0: [UNICODE; 100] = [0; 100];
            utf8StringCopy(
                &raw mut buf_0 as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[UNICODE; 100]>(),
                pStr,
            );
            replaceChar(
                &raw mut buf_0 as *mut ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x9 as ::core::ffi::c_char,
            );
            t.pStr = &raw mut buf_0 as *mut UNICODE;
            t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
        }
    }
    if flags as ::core::ffi::c_int & (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
        != 0
    {
        uqm_lua_checked!(SetFlashRect(::core::ptr::null::<RECT>(), BOOLEAN::FALSE), BOOLEAN(0));
    }
    if (CurPos as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
        && (ExPos as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(ClearDrawable(), BOOLEAN(0));
        uqm_lua_checked!(DrawBorder(C2Rust_Unnamed_0::SIS_MSG_FRAME.0 as ::core::ffi::c_int as BYTE), BOOLEAN(0));
        t.align = TEXT_ALIGN::ALIGN_CENTER;
        uqm_lua_checked!(font_DrawText(&raw mut t), BOOLEAN(0));
    } else {
        let mut i: ::core::ffi::c_int = 0;
        let mut text_r: RECT = RECT {
            corner: POINT { x: 0, y: 0 },
            extent: EXTENT { width: 0, height: 0 },
        };
        let mut char_deltas: [BYTE; 128] = [0; 128];
        let mut pchar_deltas: *mut BYTE = ::core::ptr::null_mut::<BYTE>();
        t.baseline.x = ((3 as ::core::ffi::c_int) << resolutionFactor) as COORD;
        t.align = TEXT_ALIGN::ALIGN_LEFT;
        TextRect(&raw mut t, &raw mut text_r, &raw mut char_deltas as *mut BYTE);
        uqm_lua_checked!(ClearDrawable(), BOOLEAN(0));
        uqm_lua_checked!(DrawBorder(C2Rust_Unnamed_0::SIS_MSG_FRAME.0 as ::core::ffi::c_int as BYTE), BOOLEAN(0));
        if CurPos as ::core::ffi::c_int >= 0 as ::core::ffi::c_int
            && CurPos as ::core::ffi::c_int <= t.CharCount as ::core::ffi::c_int
        {
            let mut cur_r: RECT = text_r;
            pchar_deltas = &raw mut char_deltas as *mut BYTE;
            i = CurPos as ::core::ffi::c_int;
            while i > 0 as ::core::ffi::c_int {
                let c2rust_fresh2 = pchar_deltas;
                pchar_deltas = pchar_deltas.offset(1);
                cur_r.corner.x = (cur_r.corner.x as ::core::ffi::c_int
                    + *c2rust_fresh2 as SIZE as ::core::ffi::c_int) as COORD;
                i -= 1;
            }
            if (CurPos as ::core::ffi::c_int) < t.CharCount as ::core::ffi::c_int {
                cur_r.corner.x = (cur_r.corner.x as ::core::ffi::c_int
                    - ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
            }
            if flags as ::core::ffi::c_int
                & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int != 0
            {
                cur_r.corner.y = 0 as COORD;
                cur_r.extent.height = r.extent.height;
                SetCursorFlashBlock(BOOLEAN::TRUE);
                if CurPos as ::core::ffi::c_int == t.CharCount as ::core::ffi::c_int {
                    cur_r.extent.width = ((1 as ::core::ffi::c_int) << resolutionFactor)
                        as COORD;
                    cur_r.corner.x = (cur_r.corner.x as ::core::ffi::c_int
                        - if if resolutionFactor != 2 as ::core::ffi::c_uint {
                            BOOLEAN::FALSE.0 as ::core::ffi::c_int
                        } else {
                            BOOLEAN::TRUE.0 as ::core::ffi::c_int
                        } == 0
                        {
                            0 as ::core::ffi::c_int
                        } else {
                            3 as ::core::ffi::c_int
                        }) as COORD;
                } else if CurPos as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                    == t.CharCount as ::core::ffi::c_int
                {
                    cur_r.extent.width = (*pchar_deltas as SIZE as ::core::ffi::c_int
                        - if if resolutionFactor != 2 as ::core::ffi::c_uint {
                            BOOLEAN::FALSE.0 as ::core::ffi::c_int
                        } else {
                            BOOLEAN::TRUE.0 as ::core::ffi::c_int
                        } == 0
                        {
                            0 as ::core::ffi::c_int
                        } else {
                            3 as ::core::ffi::c_int
                        }) as COORD;
                    cur_r.corner.x = (cur_r.corner.x as ::core::ffi::c_int
                        + ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
                } else if (CurPos as ::core::ffi::c_int) < ExPos as ::core::ffi::c_int {
                    cur_r.extent.width = (*pchar_deltas as SIZE as ::core::ffi::c_int
                        - ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
                    cur_r.corner.x = (cur_r.corner.x as ::core::ffi::c_int
                        + ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
                } else {
                    cur_r.extent.width = *pchar_deltas as SIZE as COORD;
                    cur_r.corner.x = (cur_r.corner.x as ::core::ffi::c_int
                        + ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
                }
                if cur_r.extent.width as ::core::ffi::c_int >= 200 as ::core::ffi::c_int
                {
                    cur_r.extent.width = ((1 as ::core::ffi::c_int) << resolutionFactor)
                        as COORD;
                    cur_r.corner.x = (cur_r.corner.x as ::core::ffi::c_int
                        - if if resolutionFactor != 2 as ::core::ffi::c_uint {
                            BOOLEAN::FALSE.0 as ::core::ffi::c_int
                        } else {
                            BOOLEAN::TRUE.0 as ::core::ffi::c_int
                        } == 0
                        {
                            0 as ::core::ffi::c_int
                        } else {
                            3 as ::core::ffi::c_int
                        }) as COORD;
                } else {
                    SetContextForeGroundColor(
                        colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE),
                    );
                    uqm_lua_checked!(DrawFilledRectangle(&raw mut cur_r), BOOLEAN(0));
                }
            } else {
                cur_r.corner.y = ((1 as ::core::ffi::c_int) << resolutionFactor)
                    as COORD;
                cur_r.extent.height = (r.extent.height as ::core::ffi::c_int
                    - ((2 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
                cur_r.extent.width = ((1 as ::core::ffi::c_int) << resolutionFactor)
                    as COORD;
                if CurPos as ::core::ffi::c_int == t.CharCount as ::core::ffi::c_int {
                    text_r.corner.x = (text_r.corner.x as ::core::ffi::c_int
                        - if if resolutionFactor != 2 as ::core::ffi::c_uint {
                            BOOLEAN::FALSE.0 as ::core::ffi::c_int
                        } else {
                            BOOLEAN::TRUE.0 as ::core::ffi::c_int
                        } == 0
                        {
                            0 as ::core::ffi::c_int
                        } else {
                            3 as ::core::ffi::c_int
                        }) as COORD;
                }
                SetCursorFlashBlock(BOOLEAN::FALSE);
            }
            SetCursorRect(&raw mut cur_r, OffScreenContext);
        }
        SetContextForeGroundColor(colorFromRgb15(0x1b as BYTE, 0 as BYTE, 0x1b as BYTE));
        if ExPos as ::core::ffi::c_int >= 0 as ::core::ffi::c_int
            && (ExPos as ::core::ffi::c_int) < t.CharCount as ::core::ffi::c_int
        {
            t.CharCount = ExPos as COUNT;
            uqm_lua_checked!(font_DrawText(&raw mut t), BOOLEAN(0));
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
            uqm_lua_checked!(font_DrawText(&raw mut t), BOOLEAN(0));
        } else {
            uqm_lua_checked!(font_DrawText(&raw mut t), BOOLEAN(0));
        }
    }
    if flags as ::core::ffi::c_int & (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
        != 0
    {
        r.corner.x = 0 as COORD;
        r.corner.y = 0 as COORD;
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
    match optDateFormat {
        1 => {
            snprintf(
                buf,
                bufLen,
                b"%02d%s%02d%s%04d\0".as_ptr() as *const ::core::ffi::c_char,
                month_index as ::core::ffi::c_int,
                b"\xC2\xB7\0".as_ptr() as *const ::core::ffi::c_char,
                day_index as ::core::ffi::c_int,
                b"\xC2\xB7\0".as_ptr() as *const ::core::ffi::c_char,
                year_index as ::core::ffi::c_int,
            );
        }
        2 => {
            snprintf(
                buf,
                bufLen,
                b"%02d %s%s%04d\0".as_ptr() as *const ::core::ffi::c_char,
                day_index as ::core::ffi::c_int,
                GetStringAddress(
                    SetAbsStringTableIndex(
                        GameStrings,
                        (C2Rust_Unnamed_2::MONTHS_STRING_BASE.0 as ::core::ffi::c_int
                            + month_index as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int) as COUNT,
                    ),
                ),
                b"\xC2\xB7\0".as_ptr() as *const ::core::ffi::c_char,
                year_index as ::core::ffi::c_int,
            );
        }
        3 => {
            snprintf(
                buf,
                bufLen,
                b"%02d%s%02d%s%04d\0".as_ptr() as *const ::core::ffi::c_char,
                day_index as ::core::ffi::c_int,
                b"\xC2\xB7\0".as_ptr() as *const ::core::ffi::c_char,
                month_index as ::core::ffi::c_int,
                b"\xC2\xB7\0".as_ptr() as *const ::core::ffi::c_char,
                year_index as ::core::ffi::c_int,
            );
        }
        0 | _ => {
            snprintf(
                buf,
                bufLen,
                b"%s %02d%s%04d\0".as_ptr() as *const ::core::ffi::c_char,
                GetStringAddress(
                    SetAbsStringTableIndex(
                        GameStrings,
                        (C2Rust_Unnamed_2::MONTHS_STRING_BASE.0 as ::core::ffi::c_int
                            + month_index as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int) as COUNT,
                    ),
                ),
                day_index as ::core::ffi::c_int,
                b"\xC2\xB7\0".as_ptr() as *const ::core::ffi::c_char,
                year_index as ::core::ffi::c_int,
            );
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn GetStatusMessageRect(mut r: *mut RECT) {
    (*r).corner.x = ((2 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    (*r).corner.y = ((130 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    (*r).extent.width = (((64 as ::core::ffi::c_int) << resolutionFactor)
        - ((4 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    (*r).extent.height = ((7 as ::core::ffi::c_int) << resolutionFactor) as COORD;
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
    uqm_lua_checked!(ClearDrawable(), ());
    uqm_lua_checked!(DrawBorder(C2Rust_Unnamed_0::STAT_MSG_FRAME.0 as ::core::ffi::c_int as BYTE), ());
    if pStr.is_null() {
        if curMsgMode.0 == StatMsgMode::SMM_CREDITS.0 {
            if optInfiniteCredits.0 != 0 {
                snprintf(
                    &raw mut buf as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[UNICODE; 128]>(),
                    b"%s %s\0".as_ptr() as *const ::core::ffi::c_char,
                    if isPC(optWhichMenu).0 != 0 && isPC(optWhichFonts).0 != 0 {
                        GetStringAddress(
                            SetAbsStringTableIndex(
                                GameStrings,
                                (C2Rust_Unnamed_2::STATUS_STRING_BASE.0
                                    as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as COUNT,
                            ),
                        ) as *const UNICODE
                    } else {
                        b"\xE2\x88\x9E\0".as_ptr() as *const UNICODE
                    },
                    GetStringAddress(
                        SetAbsStringTableIndex(
                            GameStrings,
                            (C2Rust_Unnamed_2::STATUS_STRING_BASE.0 as ::core::ffi::c_int
                                + 0 as ::core::ffi::c_int) as COUNT,
                        ),
                    ),
                );
            } else {
                snprintf(
                    &raw mut buf as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[UNICODE; 128]>(),
                    b"%u %s\0".as_ptr() as *const ::core::ffi::c_char,
                    ((uqm_lua_checked!(getGameStateUint(
                        b"MELNORME_CREDIT1\0".as_ptr() as *const ::core::ffi::c_char,
                    ), ()) as BYTE as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as UWORD
                        as ::core::ffi::c_int
                        | uqm_lua_checked!(getGameStateUint(
                            b"MELNORME_CREDIT0\0".as_ptr() as *const ::core::ffi::c_char,
                        ), ()) as BYTE as ::core::ffi::c_int,
                    GetStringAddress(
                        SetAbsStringTableIndex(
                            GameStrings,
                            (C2Rust_Unnamed_2::STATUS_STRING_BASE.0 as ::core::ffi::c_int
                                + 0 as ::core::ffi::c_int) as COUNT,
                        ),
                    ),
                );
            }
        } else if curMsgMode.0 == StatMsgMode::SMM_RES_UNITS.0 {
            if uqm_lua_checked!(getGameStateUint(
                b"CHMMR_BOMB_STATE\0".as_ptr() as *const ::core::ffi::c_char,
            ), ()) >= 2 as DWORD || optInfiniteRU.0 != 0
            {
                snprintf(
                    &raw mut buf as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[UNICODE; 128]>(),
                    b"%s %s\0".as_ptr() as *const ::core::ffi::c_char,
                    if isPC(optWhichMenu).0 != 0 && isPC(optWhichFonts).0 != 0 {
                        GetStringAddress(
                            SetAbsStringTableIndex(
                                GameStrings,
                                (C2Rust_Unnamed_2::STATUS_STRING_BASE.0
                                    as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as COUNT,
                            ),
                        ) as *const UNICODE
                    } else {
                        b"\xE2\x88\x9E\0".as_ptr() as *const UNICODE
                    },
                    GetStringAddress(
                        SetAbsStringTableIndex(
                            GameStrings,
                            (C2Rust_Unnamed_2::STATUS_STRING_BASE.0 as ::core::ffi::c_int
                                + 1 as ::core::ffi::c_int) as COUNT,
                        ),
                    ),
                );
            } else {
                snprintf(
                    &raw mut buf as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[UNICODE; 128]>(),
                    b"%u %s\0".as_ptr() as *const ::core::ffi::c_char,
                    GlobData.SIS_state.ResUnits,
                    GetStringAddress(
                        SetAbsStringTableIndex(
                            GameStrings,
                            (C2Rust_Unnamed_2::STATUS_STRING_BASE.0 as ::core::ffi::c_int
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
    t.baseline.x = (((64 as ::core::ffi::c_int) << resolutionFactor)
        - ((4 as ::core::ffi::c_int) << resolutionFactor) >> 1 as ::core::ffi::c_int)
        as COORD;
    t.baseline.y = (((7 as ::core::ffi::c_int) << resolutionFactor)
        - ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    t.align = TEXT_ALIGN::ALIGN_CENTER;
    t.pStr = pStr;
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    let mut statusColor: Color = colorFromRgb15(0 as BYTE, 0x10 as BYTE, 0 as BYTE);
    if curMsgMode.0 == StatMsgMode::SMM_WARNING.0 {
        statusColor = colorFromRgb15(0x10 as BYTE, 0x10 as BYTE, 0 as BYTE);
    }
    if curMsgMode.0 == StatMsgMode::SMM_ALERT.0 {
        statusColor = colorFromRgb15(0x10 as BYTE, 0 as BYTE, 0 as BYTE);
    }
    SetContextForeGroundColor(statusColor);
    if isPC(optWhichFonts).0 != 0 || optCustomBorder.0 != 0 {
        SetContextFont(TinyFont);
    } else {
        let mut buf_0: [UNICODE; 100] = [0; 100];
        SetContextFont(TinyFontBold);
        utf8StringCopy(
            &raw mut buf_0 as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[UNICODE; 100]>(),
            pStr,
        );
        replaceChar(
            &raw mut buf_0 as *mut ::core::ffi::c_char,
            0x20 as ::core::ffi::c_char,
            0x9 as ::core::ffi::c_char,
        );
        t.pStr = &raw mut buf_0 as *mut UNICODE;
        t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    }
    SetContextForeGroundColor(colorFromRgb15(0 as BYTE, 0x10 as BYTE, 0 as BYTE));
    uqm_lua_checked!(font_DrawText(&raw mut t), ());
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
pub unsafe extern "C" fn DrawCaptainsName(mut NewGame: bool) {
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
    if isPC(optWhichFonts).0 != 0 {
        OldFont = SetContextFont(TinyFont);
    } else {
        OldFont = SetContextFont(TinyFontBold);
    }
    OldColor = SetContextForeGroundColor(
        colorFromRgb15(0 as BYTE, 0 as BYTE, 0x14 as BYTE),
    );
    r.corner.x = ((2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
        << resolutionFactor) as COORD;
    r.corner.y = ((10 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    r.extent.width = (((64 as ::core::ffi::c_int) << resolutionFactor)
        - ((4 as ::core::ffi::c_int) << resolutionFactor)
        - ((2 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    r.extent.height = ((7 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
    if !NewGame {
        uqm_lua_checked!(DrawBorder(C2Rust_Unnamed_0::CAP_NAME_FRAME.0 as ::core::ffi::c_int as BYTE), ());
    }
    t.baseline.x = (((64 as ::core::ffi::c_int) << resolutionFactor
        >> 1 as ::core::ffi::c_int) - ((1 as ::core::ffi::c_int) << resolutionFactor))
        as COORD;
    t.baseline.y = (r.corner.y as ::core::ffi::c_int
        + ((6 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    t.align = TEXT_ALIGN::ALIGN_CENTER;
    t.pStr = &raw mut GlobData.SIS_state.CommanderName as *mut UNICODE;
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    SetContextForeGroundColor(colorFromRgb15(0x16 as BYTE, 0xb as BYTE, 0x1f as BYTE));
    uqm_lua_checked!(font_DrawText(&raw mut t), ());
    SetContextForeGroundColor(OldColor);
    SetContextFont(OldFont);
    SetContext(OldContext);
}
#[no_mangle]
pub unsafe extern "C" fn DrawFlagshipName(mut InStatusArea: BOOLEAN, mut NewGame: bool) {
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut rHD: RECT = RECT {
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
    OldFontEffect = SetContextFontEffect(::core::ptr::null_mut::<FRAME_DESC>());
    OldColor = SetContextForeGroundColor(
        colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE),
    );
    if InStatusArea.0 != 0 {
        OldContext = SetContext(StatusContext);
        OldFont = SetContextFont(StarConFont);
        r.corner.x = ((2 as ::core::ffi::c_int) << resolutionFactor) as COORD;
        r.corner.y = ((20 as ::core::ffi::c_int) << resolutionFactor) as COORD;
        r.extent.width = (((64 as ::core::ffi::c_int) << resolutionFactor)
            - ((4 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        r.extent.height = ((7 as ::core::ffi::c_int) << resolutionFactor) as COORD;
        t.pStr = &raw mut GlobData.SIS_state.ShipName as *mut UNICODE;
        uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
    } else {
        OldContext = SetContext(SpaceContext);
        OldFont = SetContextFont(MicroFont);
        r.corner.x = 0 as COORD;
        r.corner.y = ((1 as ::core::ffi::c_int) << resolutionFactor) as COORD;
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
        r.extent.height = ((7 as ::core::ffi::c_int) << resolutionFactor) as COORD;
        t.pStr = &raw mut buf as *mut UNICODE;
        snprintf(
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[UNICODE; 250]>(),
            b"%s %s\0".as_ptr() as *const ::core::ffi::c_char,
            GetStringAddress(
                SetAbsStringTableIndex(
                    GameStrings,
                    (C2Rust_Unnamed_2::NAMING_STRING_BASE.0 as ::core::ffi::c_int
                        + 1 as ::core::ffi::c_int) as COUNT,
                ),
            ),
            &raw mut GlobData.SIS_state.ShipName as *mut UNICODE,
        );
        strupr(&raw mut buf as *mut ::core::ffi::c_char);
        let mut ptr: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<
            ::core::ffi::c_uchar,
        >();
        ptr = &raw mut buf as *mut UNICODE as *mut ::core::ffi::c_uchar;
        while *ptr != 0 {
            if *ptr as ::core::ffi::c_int == 0xc3 as ::core::ffi::c_int {
                ptr = ptr.offset(1);
                if *ptr as ::core::ffi::c_int == 0xb6 as ::core::ffi::c_int
                    || *ptr as ::core::ffi::c_int == 0xa4 as ::core::ffi::c_int
                {
                    *ptr = (*ptr as ::core::ffi::c_int
                        + ('A' as ::core::ffi::c_int - 'a' as ::core::ffi::c_int))
                        as ::core::ffi::c_uchar;
                }
            }
            ptr = ptr.offset(1);
        }
        if if resolutionFactor != 2 as ::core::ffi::c_uint {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } != 0
        {
            rHD = r;
            rHD.extent.width = (rHD.extent.width as ::core::ffi::c_double * 0.75f64)
                as COORD;
            rHD.corner.x = ((r.extent.width as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int)
                - (rHD.extent.width as ::core::ffi::c_int >> 1 as ::core::ffi::c_int))
                as COORD;
            SetContextForeGroundColor(colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE));
        }
        uqm_lua_checked!(DrawFilledRectangle(
            if if resolutionFactor != 2 as ::core::ffi::c_uint {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } == 0
            {
                &raw mut r
            } else {
                &raw mut rHD
            },
        ), ());
    }
    if !NewGame {
        uqm_lua_checked!(DrawBorder(
            C2Rust_Unnamed_0::SIS_STAT_REPAIR_FRAME.0 as ::core::ffi::c_int as BYTE,
        ), ());
    }
    t.baseline.x = (r.corner.x as ::core::ffi::c_int
        + ((r.extent.width as ::core::ffi::c_int >> resolutionFactor
            >> 1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    t.baseline.y = (r.corner.y as ::core::ffi::c_uint)
        .wrapping_add(
            (((7 as ::core::ffi::c_int) << resolutionFactor) as ::core::ffi::c_uint)
                .wrapping_sub(InStatusArea.0 << resolutionFactor),
        ) as COORD;
    t.align = TEXT_ALIGN::ALIGN_CENTER;
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    if isPC(optWhichFonts).0 != 0 {
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
    uqm_lua_checked!(font_DrawText(&raw mut t), ());
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
    let mut num_dynamos: BYTE = 0;
    let mut num_shivas: BYTE = 0;
    let mut max_thrust: COUNT = 0;
    let mut fuel: DWORD = 0;
    let mut base_y: SIZE = 0;
    if is3DO(optWhichFonts).0 != 0
        || if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
    {
        return;
    }
    energy_regeneration = 1 as BYTE;
    energy_wait = 10 as BYTE;
    max_thrust = 10 as COUNT;
    turn_wait = 17 as BYTE;
    fuel = (10 as ::core::ffi::c_int * 100 as ::core::ffi::c_int) as DWORD;
    num_dynamos = 0 as BYTE;
    num_shivas = 0 as BYTE;
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
                num_dynamos = num_dynamos.wrapping_add(1);
                if (energy_wait as ::core::ffi::c_int) < 4 as ::core::ffi::c_int {
                    energy_wait = 4 as BYTE;
                }
            }
            8 => {
                energy_regeneration = energy_regeneration.wrapping_add(1);
                num_shivas = num_shivas.wrapping_add(1);
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
    r.corner.y = (CanvasHeight
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
        - if if optWindowType == 0 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            3 as ::core::ffi::c_int
        } else {
            4 as ::core::ffi::c_int
        } * leading as ::core::ffi::c_int) as COORD;
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
    r.extent.height = (if if optWindowType == 0 as DWORD {
        BOOLEAN::TRUE.0 as ::core::ffi::c_int
    } else {
        BOOLEAN::FALSE.0 as ::core::ffi::c_int
    } != 0
    {
        3 as ::core::ffi::c_int
    } else {
        4 as ::core::ffi::c_int
    } * leading as ::core::ffi::c_int) as COORD;
    OldColor = SetContextForeGroundColor(
        colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE),
    );
    uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
    leading = (leading as ::core::ffi::c_int
        - ((2 as ::core::ffi::c_int) << resolutionFactor)) as SIZE;
    base_y = (r.corner.y as ::core::ffi::c_int + leading as ::core::ffi::c_int
        - ((2 as ::core::ffi::c_int) << resolutionFactor)) as SIZE;
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
            } << resolutionFactor)) >> resolutionFactor) / 6 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    t.baseline.y = base_y as COORD;
    t.align = TEXT_ALIGN::ALIGN_RIGHT;
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    SetContextFontEffect(SetAbsFrameIndex(FontGradFrame, 4 as COUNT));
    t.pStr = GetStringAddress(
        SetAbsStringTableIndex(
            GameStrings,
            (C2Rust_Unnamed_2::FLAGSHIP_STRING_BASE.0 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int) as COUNT,
        ),
    );
    uqm_lua_checked!(font_DrawText(&raw mut t), ());
    t.baseline.y = (t.baseline.y as ::core::ffi::c_int + leading as ::core::ffi::c_int)
        as COORD;
    t.pStr = GetStringAddress(
        SetAbsStringTableIndex(
            GameStrings,
            (C2Rust_Unnamed_2::FLAGSHIP_STRING_BASE.0 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int) as COUNT,
        ),
    );
    uqm_lua_checked!(font_DrawText(&raw mut t), ());
    t.baseline.y = (t.baseline.y as ::core::ffi::c_int + leading as ::core::ffi::c_int)
        as COORD;
    t.pStr = GetStringAddress(
        SetAbsStringTableIndex(
            GameStrings,
            (C2Rust_Unnamed_2::FLAGSHIP_STRING_BASE.0 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int) as COUNT,
        ),
    );
    uqm_lua_checked!(font_DrawText(&raw mut t), ());
    t.baseline.y = (t.baseline.y as ::core::ffi::c_int + leading as ::core::ffi::c_int)
        as COORD;
    t.pStr = GetStringAddress(
        SetAbsStringTableIndex(
            GameStrings,
            (C2Rust_Unnamed_2::FLAGSHIP_STRING_BASE.0 as ::core::ffi::c_int
                + 3 as ::core::ffi::c_int) as COUNT,
        ),
    );
    uqm_lua_checked!(font_DrawText(&raw mut t), ());
    t.baseline.x = (t.baseline.x as ::core::ffi::c_int
        + ((3 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    t.baseline.y = base_y as COORD;
    t.align = TEXT_ALIGN::ALIGN_LEFT;
    t.pStr = &raw mut buf as *mut UNICODE;
    snprintf(
        &raw mut buf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[UNICODE; 128]>(),
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        describeWeapon(GlobData.SIS_state.ModuleSlots[15usize]),
    );
    uqm_lua_checked!(font_DrawText(&raw mut t), ());
    t.baseline.y = (t.baseline.y as ::core::ffi::c_int + leading as ::core::ffi::c_int)
        as COORD;
    snprintf(
        &raw mut buf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[UNICODE; 128]>(),
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        describeWeapon(GlobData.SIS_state.ModuleSlots[14usize]),
    );
    uqm_lua_checked!(font_DrawText(&raw mut t), ());
    t.baseline.y = (t.baseline.y as ::core::ffi::c_int + leading as ::core::ffi::c_int)
        as COORD;
    snprintf(
        &raw mut buf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[UNICODE; 128]>(),
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        describeWeapon(GlobData.SIS_state.ModuleSlots[13usize]),
    );
    uqm_lua_checked!(font_DrawText(&raw mut t), ());
    t.baseline.y = (t.baseline.y as ::core::ffi::c_int + leading as ::core::ffi::c_int)
        as COORD;
    snprintf(
        &raw mut buf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[UNICODE; 128]>(),
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        describeWeapon(GlobData.SIS_state.ModuleSlots[0usize]),
    );
    uqm_lua_checked!(font_DrawText(&raw mut t), ());
    t.baseline.x = (r.extent.width as ::core::ffi::c_int
        - ((26 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    t.baseline.y = base_y as COORD;
    t.align = TEXT_ALIGN::ALIGN_RIGHT;
    SetContextFontEffect(SetAbsFrameIndex(FontGradFrame, 5 as COUNT));
    t.pStr = GetStringAddress(
        SetAbsStringTableIndex(
            GameStrings,
            (C2Rust_Unnamed_2::FLAGSHIP_STRING_BASE.0 as ::core::ffi::c_int
                + 4 as ::core::ffi::c_int) as COUNT,
        ),
    );
    uqm_lua_checked!(font_DrawText(&raw mut t), ());
    t.baseline.y = (t.baseline.y as ::core::ffi::c_int + leading as ::core::ffi::c_int)
        as COORD;
    t.pStr = GetStringAddress(
        SetAbsStringTableIndex(
            GameStrings,
            (C2Rust_Unnamed_2::FLAGSHIP_STRING_BASE.0 as ::core::ffi::c_int
                + 5 as ::core::ffi::c_int) as COUNT,
        ),
    );
    uqm_lua_checked!(font_DrawText(&raw mut t), ());
    t.baseline.y = (t.baseline.y as ::core::ffi::c_int + leading as ::core::ffi::c_int)
        as COORD;
    t.pStr = GetStringAddress(
        SetAbsStringTableIndex(
            GameStrings,
            (C2Rust_Unnamed_2::FLAGSHIP_STRING_BASE.0 as ::core::ffi::c_int
                + 6 as ::core::ffi::c_int) as COUNT,
        ),
    );
    uqm_lua_checked!(font_DrawText(&raw mut t), ());
    t.baseline.y = (t.baseline.y as ::core::ffi::c_int + leading as ::core::ffi::c_int)
        as COORD;
    t.pStr = GetStringAddress(
        SetAbsStringTableIndex(
            GameStrings,
            (C2Rust_Unnamed_2::FLAGSHIP_STRING_BASE.0 as ::core::ffi::c_int
                + 7 as ::core::ffi::c_int) as COUNT,
        ),
    );
    uqm_lua_checked!(font_DrawText(&raw mut t), ());
    t.baseline.x = (r.extent.width as ::core::ffi::c_int
        - ((2 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    t.baseline.y = base_y as COORD;
    t.pStr = &raw mut buf as *mut UNICODE;
    snprintf(
        &raw mut buf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[UNICODE; 128]>(),
        b"%4u\0".as_ptr() as *const ::core::ffi::c_char,
        max_thrust as ::core::ffi::c_int * 4 as ::core::ffi::c_int,
    );
    uqm_lua_checked!(font_DrawText(&raw mut t), ());
    t.baseline.y = (t.baseline.y as ::core::ffi::c_int + leading as ::core::ffi::c_int)
        as COORD;
    snprintf(
        &raw mut buf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[UNICODE; 128]>(),
        b"%4u\0".as_ptr() as *const ::core::ffi::c_char,
        1 as ::core::ffi::c_int + 17 as ::core::ffi::c_int
            - turn_wait as ::core::ffi::c_int,
    );
    uqm_lua_checked!(font_DrawText(&raw mut t), ());
    t.baseline.y = (t.baseline.y as ::core::ffi::c_int + leading as ::core::ffi::c_int)
        as COORD;
    if if optWindowType == 0 as DWORD {
        BOOLEAN::TRUE.0 as ::core::ffi::c_int
    } else {
        BOOLEAN::FALSE.0 as ::core::ffi::c_int
    } == 0
    {
        let mut energy_per_10_sec: ::core::ffi::c_uint = ((100 as ::core::ffi::c_int
            * 840 as ::core::ffi::c_int * energy_regeneration as ::core::ffi::c_int
            / ((1 as ::core::ffi::c_int + energy_wait as ::core::ffi::c_int)
                * (840 as ::core::ffi::c_int / 24 as ::core::ffi::c_int))
            + 5 as ::core::ffi::c_int) / 10 as ::core::ffi::c_int)
            as ::core::ffi::c_uint;
        snprintf(
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[UNICODE; 128]>(),
            b"%2u.%1u\0".as_ptr() as *const ::core::ffi::c_char,
            energy_per_10_sec.wrapping_div(10 as ::core::ffi::c_uint),
            energy_per_10_sec.wrapping_rem(10 as ::core::ffi::c_uint),
        );
    } else {
        snprintf(
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[UNICODE; 128]>(),
            b"%u\0".as_ptr() as *const ::core::ffi::c_char,
            num_dynamos as ::core::ffi::c_int * 30 as ::core::ffi::c_int
                + num_shivas as ::core::ffi::c_int * 60 as ::core::ffi::c_int,
        );
    }
    uqm_lua_checked!(font_DrawText(&raw mut t), ());
    t.baseline.y = (t.baseline.y as ::core::ffi::c_int + leading as ::core::ffi::c_int)
        as COORD;
    snprintf(
        &raw mut buf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[UNICODE; 128]>(),
        b"%4u\0".as_ptr() as *const ::core::ffi::c_char,
        fuel.wrapping_div(100 as DWORD),
    );
    uqm_lua_checked!(font_DrawText(&raw mut t), ());
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
                    (C2Rust_Unnamed_2::FLAGSHIP_STRING_BASE.0 as ::core::ffi::c_int
                        + 8 as ::core::ffi::c_int) as COUNT,
                ),
            );
        }
        10 => {
            return GetStringAddress(
                SetAbsStringTableIndex(
                    GameStrings,
                    (C2Rust_Unnamed_2::FLAGSHIP_STRING_BASE.0 as ::core::ffi::c_int
                        + 9 as ::core::ffi::c_int) as COUNT,
                ),
            );
        }
        11 => {
            return GetStringAddress(
                SetAbsStringTableIndex(
                    GameStrings,
                    (C2Rust_Unnamed_2::FLAGSHIP_STRING_BASE.0 as ::core::ffi::c_int
                        + 10 as ::core::ffi::c_int) as COUNT,
                ),
            );
        }
        14 | 15 | 16 | 17 | 18 | 19 => {
            return GetStringAddress(
                SetAbsStringTableIndex(
                    GameStrings,
                    (C2Rust_Unnamed_2::FLAGSHIP_STRING_BASE.0 as ::core::ffi::c_int
                        + 11 as ::core::ffi::c_int) as COUNT,
                ),
            );
        }
        _ => {
            return GetStringAddress(
                SetAbsStringTableIndex(
                    GameStrings,
                    (C2Rust_Unnamed_2::FLAGSHIP_STRING_BASE.0 as ::core::ffi::c_int
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
    r.corner.x = (((64 as ::core::ffi::c_int) << resolutionFactor
        >> 1 as ::core::ffi::c_int) - r.corner.x as ::core::ffi::c_int) as COORD;
    s.origin.x = (r.corner.x as ::core::ffi::c_int
        - ((r.extent.width as ::core::ffi::c_int * i as ::core::ffi::c_int
            + ((2 as ::core::ffi::c_int) << resolutionFactor)
                * (i as ::core::ffi::c_int - 1 as ::core::ffi::c_int) >> resolutionFactor
            >> 1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    s.origin.y = ((29 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    width = (r.extent.width as ::core::ffi::c_int
        + ((2 as ::core::ffi::c_int) << resolutionFactor)) as SIZE;
    r.extent.width = (r.extent.width as ::core::ffi::c_int * 10 as ::core::ffi::c_int
        + ((2 as ::core::ffi::c_int) << resolutionFactor)
            * (10 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
        + ((2 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    r.corner.x = (r.corner.x as ::core::ffi::c_int
        - ((r.extent.width as ::core::ffi::c_int >> resolutionFactor
            >> 1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    r.corner.y = (r.corner.y as ::core::ffi::c_int + s.origin.y as ::core::ffi::c_int)
        as COORD;
    SetContextForeGroundColor(colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE));
    uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
    loop {
        let c2rust_fresh1 = i;
        i = i.wrapping_sub(1);
        if c2rust_fresh1 == 0 {
            break;
        }
        uqm_lua_checked!(DrawStamp(&raw mut s), ());
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
    r.extent.width = ((2 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    r.extent.height = ((4 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    r.corner.y = ((123 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    if Refresh.0 != 0 {
        r.extent.width = (16 as ::core::ffi::c_int
            * (r.extent.width as ::core::ffi::c_int + 1 as ::core::ffi::c_int)) as COORD;
        r.corner.x = (((64 as ::core::ffi::c_int) << resolutionFactor
            >> 1 as ::core::ffi::c_int)
            - (r.extent.width as ::core::ffi::c_int >> 1 as ::core::ffi::c_int))
            as COORD;
        SetContextForeGroundColor(colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE));
        uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
        r.extent.width = ((2 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    }
    i = CountSISPieces(C2Rust_Unnamed::STORAGE_BAY.0 as ::core::ffi::c_int as BYTE)
        as BYTE;
    if i != 0 {
        let mut j: COUNT = 0;
        r.corner.x = (((64 as ::core::ffi::c_int) << resolutionFactor
            >> 1 as ::core::ffi::c_int)
            - ((i as ::core::ffi::c_int
                * (r.extent.width as ::core::ffi::c_int
                    + ((1 as ::core::ffi::c_int) << resolutionFactor))
                >> resolutionFactor >> 1 as ::core::ffi::c_int) << resolutionFactor))
            as COORD;
        SetContextForeGroundColor(colorFromRgb15(0xa as BYTE, 0xa as BYTE, 0xa as BYTE));
        j = GlobData.SIS_state.TotalElementMass;
        while (j as ::core::ffi::c_int) >= 500 as ::core::ffi::c_int {
            uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
            r.corner.x = (r.corner.x as ::core::ffi::c_int
                + (r.extent.width as ::core::ffi::c_int
                    + ((1 as ::core::ffi::c_int) << resolutionFactor))) as COORD;
            i = i.wrapping_sub(1);
            j = (j as ::core::ffi::c_int - 500 as ::core::ffi::c_int) as COUNT;
        }
        r.extent.height = ((((4 as ::core::ffi::c_int) << resolutionFactor)
            * j as ::core::ffi::c_int
            + (500 as ::core::ffi::c_int - 1 as ::core::ffi::c_int))
            / 500 as ::core::ffi::c_int) as COORD;
        if r.extent.height != 0 {
            r.corner.y = (r.corner.y as ::core::ffi::c_int
                + (((4 as ::core::ffi::c_int) << resolutionFactor)
                    - r.extent.height as ::core::ffi::c_int)) as COORD;
            uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
            r.extent.height = (((4 as ::core::ffi::c_int) << resolutionFactor)
                - r.extent.height as ::core::ffi::c_int) as COORD;
            if r.extent.height != 0 {
                r.corner.y = ((123 as ::core::ffi::c_int) << resolutionFactor) as COORD;
                SetContextForeGroundColor(
                    colorFromRgb15(0x6 as BYTE, 0x6 as BYTE, 0x6 as BYTE),
                );
                uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
            }
            r.corner.x = (r.corner.x as ::core::ffi::c_int
                + (r.extent.width as ::core::ffi::c_int
                    + ((1 as ::core::ffi::c_int) << resolutionFactor))) as COORD;
            i = i.wrapping_sub(1);
        }
        r.extent.height = ((4 as ::core::ffi::c_int) << resolutionFactor) as COORD;
        SetContextForeGroundColor(colorFromRgb15(0x6 as BYTE, 0x6 as BYTE, 0x6 as BYTE));
        loop {
            let c2rust_fresh0 = i;
            i = i.wrapping_sub(1);
            if c2rust_fresh0 == 0 {
                break;
            }
            uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
            r.corner.x = (r.corner.x as ::core::ffi::c_int
                + (r.extent.width as ::core::ffi::c_int
                    + ((1 as ::core::ffi::c_int) << resolutionFactor))) as COORD;
        }
    }
    SetContext(OldContext);
}
#[no_mangle]
pub unsafe extern "C" fn GetGaugeRect(mut pRect: *mut RECT, mut IsCrewRect: BOOLEAN) {
    (*pRect).extent.width = ((24 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    (*pRect).corner.x = (((64 as ::core::ffi::c_int) << resolutionFactor
        >> 1 as ::core::ffi::c_int)
        - ((*pRect).extent.width as ::core::ffi::c_int >> 1 as ::core::ffi::c_int))
        as COORD;
    (*pRect).extent.height = ((5 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    (*pRect).corner.y = (if IsCrewRect.0 != 0 {
        (117 as ::core::ffi::c_int) << resolutionFactor
    } else {
        (38 as ::core::ffi::c_int) << resolutionFactor
    }) as COORD;
}
unsafe extern "C" fn Draw_SIS() {
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
    let mut flat: BOOLEAN = BOOLEAN(is3DO(optWhichFonts).0);
    GetGaugeRect(&raw mut r, BOOLEAN::FALSE);
    t.baseline.x = ((64 as ::core::ffi::c_int) << resolutionFactor
        >> 1 as ::core::ffi::c_int) as COORD;
    t.baseline.y = (r.corner.y as ::core::ffi::c_int
        - ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    t.align = TEXT_ALIGN::ALIGN_CENTER;
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    SetContextFont(if flat.0 != 0 { TinyFontBold } else { TinyFont });
    SetContextForeGroundColor(colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE));
    r.corner.y = (r.corner.y as ::core::ffi::c_int
        - ((6 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    r.corner.x = (r.corner.x as ::core::ffi::c_int
        - ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    r.extent.width = (r.extent.width as ::core::ffi::c_int
        + ((2 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
    SetContextFontEffect(
        SetAbsFrameIndex(
            FontGradFrame,
            (if flat.0 != 0 {
                10 as ::core::ffi::c_int
            } else {
                1 as ::core::ffi::c_int
            }) as COUNT,
        ),
    );
    t.pStr = GetStringAddress(
        SetAbsStringTableIndex(
            GameStrings,
            (C2Rust_Unnamed_2::STATUS_STRING_BASE.0 as ::core::ffi::c_int
                + if flat.0 != 0 {
                    17 as ::core::ffi::c_int
                } else {
                    3 as ::core::ffi::c_int
                }) as COUNT,
        ),
    );
    uqm_lua_checked!(font_DrawText(&raw mut t), ());
    r.corner.y = (r.corner.y as ::core::ffi::c_int
        + ((79 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    t.baseline.y = (t.baseline.y as ::core::ffi::c_int
        + ((79 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
    SetContextFontEffect(
        SetAbsFrameIndex(
            FontGradFrame,
            (if flat.0 != 0 {
                11 as ::core::ffi::c_int
            } else {
                2 as ::core::ffi::c_int
            }) as COUNT,
        ),
    );
    t.pStr = GetStringAddress(
        SetAbsStringTableIndex(
            GameStrings,
            (C2Rust_Unnamed_2::STATUS_STRING_BASE.0 as ::core::ffi::c_int
                + if flat.0 != 0 {
                    18 as ::core::ffi::c_int
                } else {
                    4 as ::core::ffi::c_int
                }) as COUNT,
        ),
    );
    uqm_lua_checked!(font_DrawText(&raw mut t), ());
    SetContextFontEffect(::core::ptr::null_mut::<FRAME_DESC>());
    r.corner.x = ((2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
        << resolutionFactor) as COORD;
    r.corner.y = ((3 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    r.extent.width = ((58 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    r.extent.height = ((7 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    SetContextForeGroundColor(colorFromRgb15(0 as BYTE, 0 as BYTE, 0x14 as BYTE));
    uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
    uqm_lua_checked!(DrawBorder(C2Rust_Unnamed_0::CAPTAIN_FRAME.0 as ::core::ffi::c_int as BYTE), ());
    SetContextForeGroundColor(
        if flat.0 != 0 {
            buildColorRgba(0x48 as BYTE, 0x98 as BYTE, 0xc0 as BYTE, 0xff as BYTE)
        } else {
            colorFromRgb15(0x2 as BYTE, 0x4 as BYTE, 0x1e as BYTE)
        },
    );
    t.baseline.y = (r.corner.y as ::core::ffi::c_int
        + ((6 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    t.baseline.x = (t.baseline.x as ::core::ffi::c_int
        - ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    t.pStr = GetStringAddress(
        SetAbsStringTableIndex(
            GameStrings,
            (C2Rust_Unnamed_2::STATUS_STRING_BASE.0 as ::core::ffi::c_int
                + if flat.0 != 0 {
                    19 as ::core::ffi::c_int
                } else {
                    5 as ::core::ffi::c_int
                }) as COUNT,
        ),
    );
    uqm_lua_checked!(font_DrawText(&raw mut t), ());
}
unsafe extern "C" fn DrawThrusters() {
    let mut s: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    let mut i: COUNT = 0;
    s.origin.x = ((1 as ::core::ffi::c_int) << resolutionFactor) as COORD;
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
            uqm_lua_checked!(DrawStamp(&raw mut s), ());
            s.frame = IncFrameIndex(s.frame);
            uqm_lua_checked!(DrawStamp(&raw mut s), ());
        }
        s.origin.y = (s.origin.y as ::core::ffi::c_int
            - ((3 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn DrawTurningJets() {
    let mut s: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    let mut i: COUNT = 0;
    s.origin.x = ((1 as ::core::ffi::c_int) << resolutionFactor) as COORD;
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
            uqm_lua_checked!(DrawStamp(&raw mut s), ());
            s.frame = IncFrameIndex(s.frame);
            uqm_lua_checked!(DrawStamp(&raw mut s), ());
        }
        s.origin.y = (s.origin.y as ::core::ffi::c_int
            - ((3 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn DrawModules() {
    let mut s: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    let mut i: COUNT = 0;
    s.origin.x = ((1 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    s.origin.y = ((1 as ::core::ffi::c_int) << resolutionFactor) as COORD;
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
            uqm_lua_checked!(DrawStamp(&raw mut s), ());
        }
        s.origin.y = (s.origin.y as ::core::ffi::c_int
            - ((3 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
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
        s.origin = MAKE_POINT(
            (((*pship_pos).x as ::core::ffi::c_int) << resolutionFactor) as COORD,
            (((*pship_pos).y as ::core::ffi::c_int) << resolutionFactor) as COORD,
        );
        s.frame = SetAbsFrameIndex((*StarShipPtr).icons, 0 as COUNT);
        uqm_lua_checked!(DrawStamp(&raw mut s), ());
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
    t.baseline.x = ((64 as ::core::ffi::c_int) << resolutionFactor
        >> 1 as ::core::ffi::c_int) as COORD;
    t.baseline.y = (r.corner.y as ::core::ffi::c_int
        + r.extent.height as ::core::ffi::c_int) as COORD;
    t.align = TEXT_ALIGN::ALIGN_CENTER;
    t.pStr = &raw mut buf as *mut UNICODE;
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    SetContextForeGroundColor(colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE));
    uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
    SetContextForeGroundColor(colorFromRgb15(0 as BYTE, 0xe as BYTE, 0 as BYTE));
    uqm_lua_checked!(font_DrawText(&raw mut t), ());
}
unsafe extern "C" fn DeltaSISGauges_fuelDelta(mut fuel_delta: SDWORD) {
    let mut OldCoarseFuel: DWORD = 0;
    let mut NewCoarseFuel: DWORD = 0;
    if fuel_delta == 0 as ::core::ffi::c_int {
        return;
    }
    if fuel_delta == 0x7fff as ::core::ffi::c_int {
        OldCoarseFuel = !(0 as ::core::ffi::c_int) as DWORD;
    } else {
        OldCoarseFuel = GlobData
            .SIS_state
            .FuelOnBoard
            .wrapping_div(
                (if optWholeFuel.0 == 0 {
                    100 as ::core::ffi::c_int
                } else {
                    1 as ::core::ffi::c_int
                }) as DWORD,
            );
        if fuel_delta < 0 as ::core::ffi::c_int
            && GlobData.SIS_state.FuelOnBoard <= -fuel_delta as DWORD
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
    NewCoarseFuel = GlobData
        .SIS_state
        .FuelOnBoard
        .wrapping_div(
            (if optWholeFuel.0 == 0 {
                100 as ::core::ffi::c_int
            } else {
                1 as ::core::ffi::c_int
            }) as DWORD,
        );
    if NewCoarseFuel != OldCoarseFuel {
        let mut t: TEXT = TEXT {
            baseline: POINT { x: 0, y: 0 },
            pStr: ::core::ptr::null::<UNICODE>(),
            align: TEXT_ALIGN::ALIGN_LEFT,
            CharCount: 0,
        };
        let mut buf: [UNICODE; 7] = [0; 7];
        let mut r: RECT = RECT {
            corner: POINT { x: 0, y: 0 },
            extent: EXTENT { width: 0, height: 0 },
        };
        let mut dblFuelOnBoard: ::core::ffi::c_double = NewCoarseFuel
            as ::core::ffi::c_double
            / 100 as ::core::ffi::c_int as ::core::ffi::c_double;
        if optInfiniteFuel.0 == 0 {
            if optWholeFuel.0 == 0 {
                snprintf(
                    &raw mut buf as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[UNICODE; 7]>(),
                    b"%u\0".as_ptr() as *const ::core::ffi::c_char,
                    NewCoarseFuel,
                );
            } else if dblFuelOnBoard > 999.99f64 {
                snprintf(
                    &raw mut buf as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[UNICODE; 7]>(),
                    b"%.1f\0".as_ptr() as *const ::core::ffi::c_char,
                    dblFuelOnBoard,
                );
            } else {
                snprintf(
                    &raw mut buf as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[UNICODE; 7]>(),
                    b"%.2f\0".as_ptr() as *const ::core::ffi::c_char,
                    dblFuelOnBoard,
                );
            }
        } else {
            snprintf(
                &raw mut buf as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[UNICODE; 7]>(),
                b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                b"\xE2\x88\x9E\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        GetGaugeRect(&raw mut r, BOOLEAN::FALSE);
        if optWhichFonts == 0x1 as ::core::ffi::c_int && optWholeFuel.0 == 0 {
            SetContextFont(TinyFontBold);
        } else {
            SetContextFont(TinyFont);
        }
        t.baseline.x = ((64 as ::core::ffi::c_int) << resolutionFactor
            >> 1 as ::core::ffi::c_int) as COORD;
        t.baseline.y = (r.corner.y as ::core::ffi::c_int
            + r.extent.height as ::core::ffi::c_int) as COORD;
        t.align = TEXT_ALIGN::ALIGN_CENTER;
        t.pStr = &raw mut buf as *mut UNICODE;
        t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
        SetContextForeGroundColor(colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE));
        if optWholeFuel.0 != 0 {
            r.corner.x = (r.corner.x as ::core::ffi::c_int
                - ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
            r.extent.width = (r.extent.width as ::core::ffi::c_int
                + ((2 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        }
        uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
        SetContextForeGroundColor(colorFromRgb15(0x13 as BYTE, 0 as BYTE, 0 as BYTE));
        uqm_lua_checked!(font_DrawText(&raw mut t), ());
    }
}
unsafe extern "C" fn DeltaSISGauges_resunitDelta(mut resunit_delta: SIZE) {
    if resunit_delta as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        uqm_lua_checked!(DrawStatusMessage(::core::ptr::null::<UNICODE>()), ());
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
                    1438 as ::core::ffi::c_int,
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
        GetStatusMessageRect(&raw mut r);
        SetContextForeGroundColor(colorFromRgb15(0 as BYTE, 0x8 as BYTE, 0 as BYTE));
        uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
    }
    uqm_lua_checked!(DrawStatusMessage(::core::ptr::null::<UNICODE>()), ());
}
unsafe extern "C" fn isUndefinedDelta(
    mut size: SIZE,
    mut sdword: SDWORD,
    mut integer: ::core::ffi::c_int,
) -> BOOLEAN {
    return BOOLEAN(
        (size as ::core::ffi::c_int != 0 && sdword != 0 && integer != 0
            && (size as ::core::ffi::c_int == 0x7fff as ::core::ffi::c_int
                || sdword == 0x7fff as ::core::ffi::c_int
                || integer == 0x7fff as ::core::ffi::c_int)) as ::core::ffi::c_int
            as ::core::ffi::c_uint,
    );
}
#[no_mangle]
pub unsafe extern "C" fn DeltaSISGauges(
    mut crew_delta: SIZE,
    mut fuel_delta: SDWORD,
    mut resunit_delta: ::core::ffi::c_int,
) {
    let mut OldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    if crew_delta as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        && fuel_delta == 0 as ::core::ffi::c_int
        && resunit_delta == 0 as ::core::ffi::c_int
    {
        return;
    }
    OldContext = SetContext(StatusContext);
    BatchGraphics();
    if isUndefinedDelta(crew_delta, fuel_delta, resunit_delta).0 != 0 {
        let mut s: STAMP = STAMP {
            origin: POINT { x: 0, y: 0 },
            frame: ::core::ptr::null_mut::<FRAME_DESC>(),
        };
        s.origin.x = 0 as COORD;
        s.origin.y = 0 as COORD;
        s.frame = SetAbsFrameIndex(FlagStatFrame, 0 as COUNT);
        uqm_lua_checked!(DrawStamp(&raw mut s), ());
        if is3DO(optFlagshipColor).0 != 0 {
            s.frame = SetAbsFrameIndex(FlagStatFrame, 23 as COUNT);
            uqm_lua_checked!(DrawStamp(&raw mut s), ());
        }
        uqm_lua_checked!(DrawBorder(C2Rust_Unnamed_0::SIS_STAT_FRAME.0 as ::core::ffi::c_int as BYTE), ());
        uqm_lua_checked!(Draw_SIS(), ());
        uqm_lua_checked!(DrawThrusters(), ());
        uqm_lua_checked!(DrawTurningJets(), ());
        uqm_lua_checked!(DrawModules(), ());
        uqm_lua_checked!(DrawSupportShips(), ());
    }
    if isPC(optWhichFonts).0 != 0 {
        SetContextFont(TinyFont);
    } else {
        SetContextFont(TinyFontBold);
    }
    uqm_lua_checked!(DeltaSISGauges_crewDelta(crew_delta), ());
    uqm_lua_checked!(DeltaSISGauges_fuelDelta(fuel_delta), ());
    if isUndefinedDelta(crew_delta, fuel_delta, resunit_delta).0 != 0 {
        uqm_lua_checked!(DrawFlagshipName(BOOLEAN::TRUE, BOOLEAN::FALSE.0 as ::core::ffi::c_int != 0), ());
        uqm_lua_checked!(DrawCaptainsName(BOOLEAN::FALSE.0 as ::core::ffi::c_int != 0), ());
        uqm_lua_checked!(DrawLanders(), ());
        uqm_lua_checked!(DrawStorageBays(BOOLEAN::FALSE), ());
    }
    uqm_lua_checked!(DeltaSISGauges_resunitDelta(resunit_delta as SIZE), ());
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
    (*ppt).x = (((27 as ::core::ffi::c_int) << resolutionFactor)
        + slotNr as ::core::ffi::c_int * ((12 as ::core::ffi::c_int) << resolutionFactor)
        - ((colNr as ::core::ffi::c_int * 2 as ::core::ffi::c_int) << resolutionFactor)
        - if if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            (if if optWindowType == 1 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                16 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } << resolutionFactor) - ((3 as ::core::ffi::c_int) << resolutionFactor)
        } else {
            0 as ::core::ffi::c_int
        }) as COORD;
    (*ppt).y = (((34 as ::core::ffi::c_int
        - rowNr as ::core::ffi::c_int * 2 as ::core::ffi::c_int) << resolutionFactor)
        + if if resolutionFactor != 2 as ::core::ffi::c_uint {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } == 0
        {
            0 as ::core::ffi::c_int
        } else {
            2 as ::core::ffi::c_int
        }) as COORD;
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
    if rowNr as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        SetContextForeGroundColor(colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE));
    } else {
        rowNr = rowNr.wrapping_sub(1);
        SetContextForeGroundColor(colorBars[rowNr as usize]);
    }
    (*ppt).x = (((19 as ::core::ffi::c_int) << resolutionFactor)
        + slotNr as ::core::ffi::c_int * ((12 as ::core::ffi::c_int) << resolutionFactor)
        - if if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            (if if optWindowType == 1 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                16 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } << resolutionFactor) - ((3 as ::core::ffi::c_int) << resolutionFactor)
        } else {
            0 as ::core::ffi::c_int
        }) as COORD;
    (*ppt).y = (((34 as ::core::ffi::c_int
        - rowNr as ::core::ffi::c_int * 2 as ::core::ffi::c_int) << resolutionFactor)
        + if if resolutionFactor != 2 as ::core::ffi::c_uint {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } == 0
        {
            0 as ::core::ffi::c_int
        } else {
            2 as ::core::ffi::c_int
        }) as COORD;
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
    let mut capacity: DWORD = (10 as ::core::ffi::c_int * 100 as ::core::ffi::c_int)
        as DWORD;
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
    let mut capacity: DWORD = (10 as ::core::ffi::c_int * 100 as ::core::ffi::c_int)
        as DWORD;
    (unitNr >= (10 as ::core::ffi::c_int * 100 as ::core::ffi::c_int) as DWORD
        || {
            __assert_fail(
                b"unitNr >= FUEL_RESERVE\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/uqm/sis.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                1781 as ::core::ffi::c_int,
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
unsafe extern "C" fn GetFTankScreenPos(mut ppt: *mut POINT) -> DWORD {
    let mut fuelAmount: DWORD = 0;
    let mut slotNr: COUNT = 0;
    let mut compartmentNr: DWORD = 0;
    let mut moduleType: BYTE = 0;
    let mut volume: DWORD = 0;
    let mut rowNr: DWORD = 0;
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
    fuelAmount = GetFuelTotal();
    if fuelAmount < (10 as ::core::ffi::c_int * 100 as ::core::ffi::c_int) as DWORD {
        return 0 as DWORD;
    }
    if !GetFuelTankForFuelUnit(fuelAmount, &raw mut slotNr, &raw mut compartmentNr) {
        return 0 as DWORD;
    }
    moduleType = GlobData.SIS_state.ModuleSlots[slotNr as usize];
    volume = GetModuleFuelCapacity(moduleType);
    rowNr = volume
        .wrapping_sub(compartmentNr)
        .wrapping_mul(10 as DWORD)
        .wrapping_div((100 as ::core::ffi::c_int * 100 as ::core::ffi::c_int) as DWORD);
    (*ppt).x = (((21 as ::core::ffi::c_int) << resolutionFactor)
        + slotNr as ::core::ffi::c_int * ((12 as ::core::ffi::c_int) << resolutionFactor)
        + if if resolutionFactor != 2 as ::core::ffi::c_uint {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } == 0
        {
            0 as ::core::ffi::c_int
        } else if OutfitOrShipyard as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            0 as ::core::ffi::c_int
        } else {
            2 as ::core::ffi::c_int
        }
        - if if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            (if if optWindowType == 1 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                16 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } << resolutionFactor) - ((3 as ::core::ffi::c_int) << resolutionFactor)
        } else {
            0 as ::core::ffi::c_int
        }) as COORD;
    if volume == (50 as ::core::ffi::c_int * 100 as ::core::ffi::c_int) as DWORD {
        (*ppt).y = ((27 as DWORD).wrapping_sub(rowNr) << resolutionFactor) as COORD;
    } else {
        (*ppt).y = ((30 as DWORD).wrapping_sub(rowNr) << resolutionFactor) as COORD;
    }
    (rowNr.wrapping_add(1 as DWORD)
        < ::core::mem::size_of::<[Color; 11]>()
            .wrapping_div(::core::mem::size_of::<Color>()) as COUNT as DWORD
        || {
            __assert_fail(
                b"rowNr + 1 < (COUNT)ARRAY_SIZE (fuelColors)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/uqm/sis.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                1841 as ::core::ffi::c_int,
                b"GetFTankScreenPos\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    SetContextForeGroundColor(fuelColors[rowNr as usize]);
    SetContextBackGroundColor(fuelColors[rowNr.wrapping_add(1 as DWORD) as usize]);
    return volume;
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
unsafe extern "C" fn AutoPilotTextLogic() {
    let mut buf: [UNICODE; 4096] = [0; 4096];
    let mut star_cluster: [UNICODE; 45] = [0; 45];
    let mut Falayalaralfali: POINT = POINT { x: 0, y: 0 };
    let mut destination: POINT = POINT { x: 0, y: 0 };
    let mut current_position: POINT = POINT { x: 0, y: 0 };
    let mut StarPointer: *mut STAR_DESC = ::core::ptr::null_mut::<STAR_DESC>();
    let mut target_distance: ::core::ffi::c_double = 0.;
    let mut temp: TEXT = TEXT {
        baseline: POINT { x: 0, y: 0 },
        pStr: ::core::ptr::null::<UNICODE>(),
        align: TEXT_ALIGN::ALIGN_LEFT,
        CharCount: 0,
    };
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    if GlobData.SIS_state.FuelOnBoard == 0 as DWORD {
        uqm_lua_checked!(DrawSISMessageEx(
            GetStringAddress(
                SetAbsStringTableIndex(
                    GameStrings,
                    (C2Rust_Unnamed_2::NAVIGATION_STRING_BASE.0 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int) as COUNT,
                ),
            ),
            -1 as SIZE,
            -1 as SIZE,
            ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as COUNT,
        ), ());
        return;
    }
    if optSmartAutoPilot.0 == 0 {
        uqm_lua_checked!(DrawSISMessageEx(
            GetStringAddress(
                SetAbsStringTableIndex(
                    GameStrings,
                    (C2Rust_Unnamed_2::NAVIGATION_STRING_BASE.0 as ::core::ffi::c_int
                        + 3 as ::core::ffi::c_int) as COUNT,
                ),
            ),
            -1 as SIZE,
            -1 as SIZE,
            ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as COUNT,
        ), ());
        return;
    }
    Falayalaralfali.x = 6134 as COORD;
    Falayalaralfali.y = 5900 as COORD;
    current_position.x = logxToUniverse(GlobData.SIS_state.log_x) as COORD;
    current_position.y = logyToUniverse(GlobData.SIS_state.log_y) as COORD;
    destination = GlobData.Game_state.autopilot;
    target_distance = ptDistance(current_position, destination)
        / 10 as ::core::ffi::c_int as ::core::ffi::c_double;
    StarPointer = uqm_lua_checked!(FindStar(
        ::core::ptr::null_mut::<STAR_DESC>(),
        &raw mut destination,
        1 as SIZE,
        1 as SIZE,
    ), ());
    if uqm_lua_checked!(inQuasiSpace(), ()).0 != 0
        && (!pointsEqual(destination, Falayalaralfali)
            || pointsEqual(destination, Falayalaralfali) as ::core::ffi::c_int != 0
                && uqm_lua_checked!(getGameStateUint(
                    b"KNOW_QS_PORTAL\0".as_ptr() as *const ::core::ffi::c_char,
                ), ()) & ((1 as ::core::ffi::c_int) << 15 as ::core::ffi::c_int) as DWORD
                    == 0)
    {
        StarPointer = ::core::ptr::null_mut::<STAR_DESC>();
    }
    if StarPointer.is_null() {
        snprintf(
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[UNICODE; 4096]>(),
            b"%s %s %03u.%01u:%03u.%01u - %.1f\0".as_ptr() as *const ::core::ffi::c_char,
            GetStringAddress(
                SetAbsStringTableIndex(
                    GameStrings,
                    (C2Rust_Unnamed_2::NAVIGATION_STRING_BASE.0 as ::core::ffi::c_int
                        + 3 as ::core::ffi::c_int) as COUNT,
                ),
            ),
            GetStringAddress(
                SetAbsStringTableIndex(
                    GameStrings,
                    (C2Rust_Unnamed_2::NAVIGATION_STRING_BASE.0 as ::core::ffi::c_int
                        + 6 as ::core::ffi::c_int) as COUNT,
                ),
            ),
            destination.x as ::core::ffi::c_int / 10 as ::core::ffi::c_int,
            destination.x as ::core::ffi::c_int % 10 as ::core::ffi::c_int,
            destination.y as ::core::ffi::c_int / 10 as ::core::ffi::c_int,
            destination.y as ::core::ffi::c_int % 10 as ::core::ffi::c_int,
            target_distance,
        );
        uqm_lua_checked!(DrawSISMessageEx(
            &raw mut buf as *mut UNICODE,
            -1 as SIZE,
            -1 as SIZE,
            ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as COUNT,
        ), ());
        return;
    }
    if pointsEqual(uqm_lua_checked!(LoadLastLoc(), ()), destination) {
        snprintf(
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[UNICODE; 4096]>(),
            b"%s %s %s\0".as_ptr() as *const ::core::ffi::c_char,
            GetStringAddress(
                SetAbsStringTableIndex(
                    GameStrings,
                    (C2Rust_Unnamed_2::NAVIGATION_STRING_BASE.0 as ::core::ffi::c_int
                        + 3 as ::core::ffi::c_int) as COUNT,
                ),
            ),
            GetStringAddress(
                SetAbsStringTableIndex(
                    GameStrings,
                    (C2Rust_Unnamed_2::NAVIGATION_STRING_BASE.0 as ::core::ffi::c_int
                        + 6 as ::core::ffi::c_int) as COUNT,
                ),
            ),
            GetStringAddress(
                SetAbsStringTableIndex(
                    GameStrings,
                    C2Rust_Unnamed_2::NAVIGATION_STRING_BASE.0 as ::core::ffi::c_int
                        as COUNT,
                ),
            ),
        );
        uqm_lua_checked!(DrawSISMessageEx(
            &raw mut buf as *mut UNICODE,
            -1 as SIZE,
            -1 as SIZE,
            ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as COUNT,
        ), ());
        return;
    }
    GetClusterName(StarPointer, &raw mut star_cluster as *mut UNICODE);
    snprintf(
        &raw mut buf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[UNICODE; 4096]>(),
        b"%s %s %s - %.1f\0".as_ptr() as *const ::core::ffi::c_char,
        GetStringAddress(
            SetAbsStringTableIndex(
                GameStrings,
                (C2Rust_Unnamed_2::NAVIGATION_STRING_BASE.0 as ::core::ffi::c_int
                    + 3 as ::core::ffi::c_int) as COUNT,
            ),
        ),
        GetStringAddress(
            SetAbsStringTableIndex(
                GameStrings,
                (C2Rust_Unnamed_2::NAVIGATION_STRING_BASE.0 as ::core::ffi::c_int
                    + 6 as ::core::ffi::c_int) as COUNT,
            ),
        ),
        &raw mut star_cluster as *mut UNICODE,
        target_distance,
    );
    temp.pStr = &raw mut buf as *mut UNICODE;
    r = uqm_lua_checked!(font_GetTextRect(&raw mut temp), ());
    if r.extent.width as ::core::ffi::c_int
        > CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
            - ((57 as ::core::ffi::c_int) << resolutionFactor)
            - ((12 as ::core::ffi::c_int) << resolutionFactor)
            - ((2 as ::core::ffi::c_int) << resolutionFactor)
    {
        snprintf(
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[UNICODE; 4096]>(),
            b"%s %s - %.1f\0".as_ptr() as *const ::core::ffi::c_char,
            GetStringAddress(
                SetAbsStringTableIndex(
                    GameStrings,
                    (C2Rust_Unnamed_2::NAVIGATION_STRING_BASE.0 as ::core::ffi::c_int
                        + 7 as ::core::ffi::c_int) as COUNT,
                ),
            ),
            &raw mut star_cluster as *mut UNICODE,
            target_distance,
        );
    }
    temp.pStr = &raw mut buf as *mut UNICODE;
    r = uqm_lua_checked!(font_GetTextRect(&raw mut temp), ());
    if r.extent.width as ::core::ffi::c_int
        > CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
            - ((57 as ::core::ffi::c_int) << resolutionFactor)
            - ((12 as ::core::ffi::c_int) << resolutionFactor)
            - ((2 as ::core::ffi::c_int) << resolutionFactor)
    {
        snprintf(
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[UNICODE; 4096]>(),
            b"%s - %.1f\0".as_ptr() as *const ::core::ffi::c_char,
            GetStringAddress(
                SetAbsStringTableIndex(
                    GameStrings,
                    (C2Rust_Unnamed_2::NAVIGATION_STRING_BASE.0 as ::core::ffi::c_int
                        + 3 as ::core::ffi::c_int) as COUNT,
                ),
            ),
            target_distance,
        );
    }
    uqm_lua_checked!(DrawSISMessageEx(
        &raw mut buf as *mut UNICODE,
        -1 as SIZE,
        -1 as SIZE,
        ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as COUNT,
    ), ());
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
    if Reset.0 != 0 || optBubbleWarp.0 != 0 {
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
            uqm_lua_checked!(DrawSISMessage(::core::ptr::null::<UNICODE>()), ());
            cycle_index = 0 as DWORD;
            if if GlobData.SIS_state.Extended as ::core::ffi::c_int != 0 {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                uqm_lua_checked!(ZeroLastLoc(), ());
            }
        } else if GetTimeCounter() >= NextTime {
            if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
                & C2Rust_Unnamed_1::CHECK_ABORT.0 as ::core::ffi::c_int == 0
                && GlobData.SIS_state.CrewEnlisted as ::core::ffi::c_int
                    != !(0 as ::core::ffi::c_int) as COUNT as ::core::ffi::c_int
            {
                let mut OldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
                OldContext = SetContext(OffScreenContext);
                SetContextForeGroundColor(cycle_tab[cycle_index as usize]);
                uqm_lua_checked!(AutoPilotTextLogic(), ());
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
#[no_mangle]
pub unsafe extern "C" fn DrawFuelInFTanks(mut isOutfit: BOOLEAN) {
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let FuelVolume: DWORD = GlobData.SIS_state.FuelOnBoard;
    let mut capacity: DWORD = GetFuelTankCapacity();
    let mut volume: DWORD = 0;
    let mut c: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    if isOutfit.0 != 0 {
        c = colorFromRgb15(0xb as BYTE, 0 as BYTE, 0 as BYTE);
    } else {
        c = colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE);
    }
    r.extent.height = ((1 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    GlobData.SIS_state.FuelOnBoard = (10 as ::core::ffi::c_int
        * 100 as ::core::ffi::c_int) as DWORD;
    while GlobData.SIS_state.FuelOnBoard < capacity {
        if if resolutionFactor != 2 as ::core::ffi::c_uint {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } != 0 && !hdFuelFrame.is_null()
        {
            volume = GetFTankScreenPos(&raw mut r.corner);
            if GlobData.SIS_state.FuelOnBoard < FuelVolume {
                let mut s: STAMP = STAMP {
                    origin: POINT { x: 0, y: 0 },
                    frame: ::core::ptr::null_mut::<FRAME_DESC>(),
                };
                s.origin.x = r.corner.x;
                s.origin.y = 0 as COORD;
                s.frame = SetAbsFrameIndex(
                    hdFuelFrame,
                    (if volume > 5000 as DWORD {
                        0 as ::core::ffi::c_int
                    } else {
                        1 as ::core::ffi::c_int
                    }) as COUNT,
                );
                uqm_lua_checked!(DrawStamp(&raw mut s), ());
                GlobData.SIS_state.FuelOnBoard = GlobData
                    .SIS_state
                    .FuelOnBoard
                    .wrapping_add(volume);
                if GlobData.SIS_state.FuelOnBoard > FuelVolume {
                    r.extent.width = ((5 as ::core::ffi::c_int) << resolutionFactor)
                        as COORD;
                    r.extent.height = GlobData
                        .SIS_state
                        .FuelOnBoard
                        .wrapping_sub(FuelVolume)
                        .wrapping_div(
                            (100 as ::core::ffi::c_int * 100 as ::core::ffi::c_int
                                / 10 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
                                as DWORD,
                        ) as COORD;
                    r.corner.y = -(GetFrameHot(s.frame).y as ::core::ffi::c_int)
                        as COORD;
                    SetContextForeGroundColor(c);
                    uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
                }
            } else {
                r.extent.width = ((5 as ::core::ffi::c_int) << resolutionFactor)
                    as COORD;
                r.extent.height = (if volume > 5000 as DWORD {
                    40 as ::core::ffi::c_int
                } else {
                    20 as ::core::ffi::c_int
                }) as COORD;
                r.corner.y = (r.corner.y as ::core::ffi::c_int
                    - (r.extent.height as ::core::ffi::c_int - 4 as ::core::ffi::c_int))
                    as COORD;
                SetContextForeGroundColor(c);
                uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
                GlobData.SIS_state.FuelOnBoard = GlobData
                    .SIS_state
                    .FuelOnBoard
                    .wrapping_add(volume);
            }
        } else {
            GetFTankScreenPos(&raw mut r.corner);
            if GlobData.SIS_state.FuelOnBoard < FuelVolume {
                r.extent.width = ((5 as ::core::ffi::c_int) << resolutionFactor)
                    as COORD;
                uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
                r.extent.width = ((3 as ::core::ffi::c_int) << resolutionFactor)
                    as COORD;
                r.corner.x = (r.corner.x as ::core::ffi::c_int
                    + ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
                SetContextForeGroundColor(
                    SetContextBackGroundColor(
                        colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE),
                    ),
                );
            } else {
                r.extent.width = ((5 as ::core::ffi::c_int) << resolutionFactor)
                    as COORD;
                SetContextForeGroundColor(c);
            }
            uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
            GlobData.SIS_state.FuelOnBoard = GlobData
                .SIS_state
                .FuelOnBoard
                .wrapping_add(
                    (100 as ::core::ffi::c_int * 100 as ::core::ffi::c_int
                        / 10 as ::core::ffi::c_int) as DWORD,
                );
        }
    }
    GlobData.SIS_state.FuelOnBoard = FuelVolume;
}
static mut flashContext: [*mut FlashContext; 5] = [
    ::core::ptr::null_mut::<FlashContext>(),
    ::core::ptr::null_mut::<FlashContext>(),
    ::core::ptr::null_mut::<FlashContext>(),
    ::core::ptr::null_mut::<FlashContext>(),
    ::core::ptr::null_mut::<FlashContext>(),
];
static mut flash_rect: [RECT; 5] = [RECT {
    corner: POINT { x: 0, y: 0 },
    extent: EXTENT { width: 0, height: 0 },
}; 5];
static mut flashAlarm: *mut Alarm = ::core::ptr::null_mut::<Alarm>();
static mut flashPaused: BOOLEAN = BOOLEAN::FALSE;
static mut count_r: BYTE = 1 as BYTE;
unsafe extern "C" fn updateFlashRect(mut arg: *mut ::core::ffi::c_void) {
    let mut i: COUNT = 0;
    if flashContext[0usize].is_null() {
        return;
    }
    i = 0 as COUNT;
    while (i as ::core::ffi::c_int) < count_r as ::core::ffi::c_int {
        uqm_lua_checked!(Flash_process(flashContext[i as usize]), ());
        i = i.wrapping_add(1);
    }
    uqm_lua_checked!(scheduleFlashAlarm(), ());
}
unsafe extern "C" fn scheduleFlashAlarm() {
    let mut nextTime: TimeCount = Flash_nextTime(flashContext[0usize]);
    let mut nextTimeMs: DWORD = nextTime
        .wrapping_div(840 as DWORD)
        .wrapping_mul(1000 as DWORD)
        .wrapping_add(
            nextTime
                .wrapping_rem(840 as DWORD)
                .wrapping_mul(1000 as DWORD)
                .wrapping_div(840 as DWORD),
        );
    flashAlarm = uqm_lua_checked!(Alarm_addAbsoluteMs(
        nextTimeMs,
        Some(updateFlashRect as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()),
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    ), ());
}
#[no_mangle]
pub unsafe extern "C" fn SetAdditionalRect(mut pRect: *const RECT, mut number: COUNT) {
    if !pRect.is_null() && count_r as ::core::ffi::c_int != 5 as ::core::ffi::c_int {
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
        GetContextClipRect(&raw mut clip_r);
        flash_rect[number as usize] = *pRect;
        flash_rect[number as usize].corner.x = (flash_rect[number as usize].corner.x
            as ::core::ffi::c_int + clip_r.corner.x as ::core::ffi::c_int) as COORD;
        flash_rect[number as usize].corner.y = (flash_rect[number as usize].corner.y
            as ::core::ffi::c_int + clip_r.corner.y as ::core::ffi::c_int) as COORD;
        if number as ::core::ffi::c_int == count_r as ::core::ffi::c_int {
            count_r = count_r.wrapping_add(1);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn DumpAdditionalRect() {
    let mut i: COUNT = 0;
    i = count_r as COUNT;
    while i as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
        if !flashContext[i as usize].is_null() {
            uqm_lua_checked!(Flash_terminate(flashContext[i as usize]), ());
            flashContext[i as usize] = ::core::ptr::null_mut::<FlashContext>();
        }
        i = i.wrapping_sub(1);
    }
    count_r = 1 as BYTE;
}
#[no_mangle]
pub unsafe extern "C" fn SetFlashRect(mut pRect: *const RECT, mut pcRect: BOOLEAN) {
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
    let mut i: COUNT = 0;
    if pRect
        != ::core::ptr::with_exposed_provenance_mut::<
            RECT,
        >(!(0 as ::core::ffi::c_long) as usize) as *const RECT
        && pRect
            != ::core::ptr::with_exposed_provenance_mut::<
                RECT,
            >(!(1 as ::core::ffi::c_long) as usize) as *const RECT
        && pRect
            != ::core::ptr::with_exposed_provenance_mut::<
                RECT,
            >(!(2 as ::core::ffi::c_long) as usize) as *const RECT
    {
        GetContextClipRect(&raw mut clip_r);
    } else if optWhichMenu == 0x2 as ::core::ffi::c_int
        && pRect
            != ::core::ptr::with_exposed_provenance_mut::<
                RECT,
            >(!(1 as ::core::ffi::c_long) as usize) as *const RECT
        || pRect
            == ::core::ptr::with_exposed_provenance_mut::<
                RECT,
            >(!(2 as ::core::ffi::c_long) as usize) as *const RECT
    {
        pRect = ::core::ptr::null::<RECT>();
    } else {
        let mut OldContext: CONTEXT = SetContext(StatusContext);
        GetContextClipRect(&raw mut clip_r);
        pRect = &raw mut temp_r;
        temp_r.corner.x = (((4 as ::core::ffi::c_int) << resolutionFactor)
            + (CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
                } << resolutionFactor)) - clip_r.corner.x as ::core::ffi::c_int)
            as COORD;
        temp_r.corner.y = (((9 as ::core::ffi::c_int) << resolutionFactor)
            + if if optWindowType == 1 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                (if if optWindowType == 1 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    16 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                } << resolutionFactor) + ((1 as ::core::ffi::c_int) << resolutionFactor)
            } else {
                0 as ::core::ffi::c_int
            }
            + (CanvasHeight
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
                - ((13 as ::core::ffi::c_int) << resolutionFactor))
            - ((53 as ::core::ffi::c_int) << resolutionFactor)
            - clip_r.corner.y as ::core::ffi::c_int) as COORD;
        temp_r.corner.y = (temp_r.corner.y as ::core::ffi::c_int
            + (if if optWindowType == 0 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                10 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } << resolutionFactor)) as COORD;
        temp_r.extent.width = (((64 as ::core::ffi::c_int) << resolutionFactor)
            - ((8 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        temp_r.extent.height = ((53 as ::core::ffi::c_int) << resolutionFactor) as COORD;
        temp_r.extent.height = (temp_r.extent.height as ::core::ffi::c_int
            - (if if optWindowType == 0 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                10 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } << resolutionFactor)) as COORD;
        SetContext(OldContext);
    }
    if !pRect.is_null()
        && (*pRect).extent.width as ::core::ffi::c_int != 0 as ::core::ffi::c_int
    {
        flash_rect[0usize] = *pRect;
        flash_rect[0usize].corner.x = (flash_rect[0usize].corner.x as ::core::ffi::c_int
            + clip_r.corner.x as ::core::ffi::c_int) as COORD;
        flash_rect[0usize].corner.y = (flash_rect[0usize].corner.y as ::core::ffi::c_int
            + clip_r.corner.y as ::core::ffi::c_int) as COORD;
        i = 0 as COUNT;
        while (i as ::core::ffi::c_int) < count_r as ::core::ffi::c_int {
            if flashContext[i as usize].is_null() {
                flashContext[i as usize] = Flash_createHighlight(
                    ScreenContext,
                    (&raw mut flash_rect as *mut RECT).offset(i as isize),
                );
                uqm_lua_checked!(Flash_setMergeFactors(
                    flashContext[i as usize],
                    3 as ::core::ffi::c_int,
                    2 as ::core::ffi::c_int,
                    2 as ::core::ffi::c_int,
                ), ());
                Flash_setSpeed(
                    flashContext[i as usize],
                    0 as TimeCount,
                    (840 as ::core::ffi::c_int / 16 as ::core::ffi::c_int) as TimeCount,
                    0 as TimeCount,
                    (840 as ::core::ffi::c_int / 16 as ::core::ffi::c_int) as TimeCount,
                );
                Flash_setFrameTime(
                    flashContext[i as usize],
                    (840 as ::core::ffi::c_int / 16 as ::core::ffi::c_int) as TimeCount,
                );
                Flash_setPulseBox(flashContext[i as usize], pcRect);
                uqm_lua_checked!(Flash_start(flashContext[i as usize]), ());
                if i as ::core::ffi::c_int
                    == count_r as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                {
                    uqm_lua_checked!(scheduleFlashAlarm(), ());
                }
            } else {
                Flash_setPulseBox(flashContext[i as usize], pcRect);
                uqm_lua_checked!(Flash_setRect(
                    flashContext[i as usize],
                    (&raw mut flash_rect as *mut RECT).offset(i as isize),
                ), ());
            }
            i = i.wrapping_add(1);
        }
    } else if !flashContext[0usize].is_null() {
        uqm_lua_checked!(Alarm_remove(flashAlarm), ());
        flashAlarm = ::core::ptr::null_mut::<Alarm>();
        i = 0 as COUNT;
        while (i as ::core::ffi::c_int) < count_r as ::core::ffi::c_int {
            uqm_lua_checked!(Flash_terminate(flashContext[i as usize]), ());
            flashContext[i as usize] = ::core::ptr::null_mut::<FlashContext>();
            i = i.wrapping_add(1);
        }
        count_r = 1 as BYTE;
    }
}
#[no_mangle]
pub static mut updateFlashRectRecursion: COUNT = 0 as COUNT;
#[no_mangle]
pub unsafe extern "C" fn PreUpdateFlashRect() {
    let mut i: COUNT = 0;
    if !flashAlarm.is_null() {
        updateFlashRectRecursion = updateFlashRectRecursion.wrapping_add(1);
        if updateFlashRectRecursion as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
            return;
        }
        i = 0 as COUNT;
        while (i as ::core::ffi::c_int) < count_r as ::core::ffi::c_int {
            uqm_lua_checked!(Flash_preUpdate(flashContext[i as usize]), ());
            i = i.wrapping_add(1);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn PostUpdateFlashRect() {
    let mut i: COUNT = 0;
    if !flashAlarm.is_null() {
        updateFlashRectRecursion = updateFlashRectRecursion.wrapping_sub(1);
        if updateFlashRectRecursion as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
            return;
        }
        i = 0 as COUNT;
        while (i as ::core::ffi::c_int) < count_r as ::core::ffi::c_int {
            uqm_lua_checked!(Flash_postUpdate(flashContext[i as usize]), ());
            i = i.wrapping_add(1);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn PauseFlash() -> BOOLEAN {
    let mut i: BYTE = 0;
    if !flashContext[0usize].is_null() {
        if flashPaused.0 != 0 {
            return BOOLEAN::FALSE;
        }
        uqm_lua_checked!(Alarm_remove(flashAlarm), BOOLEAN(0));
        flashAlarm = ::core::ptr::null_mut::<Alarm>();
        flashPaused = BOOLEAN::TRUE;
    }
    i = 0 as BYTE;
    while (i as ::core::ffi::c_int) < count_r as ::core::ffi::c_int {
        if !flashContext[i as usize].is_null()
            && Flash_getPulseBox(flashContext[i as usize]).0 != 0
        {
            Flash_pause(flashContext[i as usize]);
        }
        i = i.wrapping_add(1);
    }
    return BOOLEAN::TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn ContinueFlash() {
    let mut i: BYTE = 0;
    if flashPaused.0 != 0 {
        i = 0 as BYTE;
        while (i as ::core::ffi::c_int) < count_r as ::core::ffi::c_int {
            if !flashContext[i as usize].is_null()
                && Flash_getPulseBox(flashContext[i as usize]).0 != 0
            {
                Flash_continue(flashContext[i as usize]);
            }
            i = i.wrapping_add(1);
        }
        uqm_lua_checked!(scheduleFlashAlarm(), ());
        flashPaused = BOOLEAN::FALSE;
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
                340 as ::core::ffi::c_int,
                b"LockShipFrag\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    return LockLink(pq, h) as *mut SHIP_FRAGMENT;
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
