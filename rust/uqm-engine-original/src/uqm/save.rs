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

// Rust translation of src/uqm/save.c.
extern "C" {
    pub type uio_DirHandle;
    pub type uio_Stream;
    pub type string_table_entry;
    pub type context_desc;
    pub type frame_desc;
    pub type font_desc;
    pub type drawable_desc;
    pub type GAME_STATE_FILE;
    fn __assert_fail(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    );
    fn BoxUnion(pr1: *mut RECT, pr2: *mut RECT, punion: *mut RECT);
    fn SetContext(Context: CONTEXT) -> CONTEXT;
    fn SetContextForeGroundColor(Color: Color) -> Color;
    fn DrawStamp(pStamp: *mut STAMP);
    fn font_DrawText(pText: *mut TEXT);
    fn BatchGraphics();
    fn UnbatchGraphics();
    fn FlushGraphics();
    fn DestroyDrawable(Drawable: DRAWABLE) -> BOOLEAN;
    fn SetContextFont(Font: FONT) -> FONT;
    fn TextRect(pText: *mut TEXT, pRect: *mut RECT, pdelta: *mut BYTE) -> BOOLEAN;
    fn ReleaseDrawable(Frame: FRAME) -> DRAWABLE;
    fn sprintf(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn res_OpenResFile(
        dir: *mut uio_DirHandle,
        filename: *const ::core::ffi::c_char,
        mode: *const ::core::ffi::c_char,
    ) -> *mut uio_Stream;
    fn WriteResFile(
        lpBuf: *const ::core::ffi::c_void,
        size: size_t,
        count: size_t,
        fp: *mut uio_Stream,
    ) -> size_t;
    fn res_CloseResFile(fp: *mut uio_Stream) -> BOOLEAN;
    fn DeleteResFile(
        dir: *mut uio_DirHandle,
        filename: *const ::core::ffi::c_char,
    ) -> BOOLEAN;
    fn SetAbsStringTableIndex(String: STRING, StringTableIndex: COUNT) -> STRING;
    fn GetStringAddress(String: STRING) -> STRINGPTR;
    fn utf8StringCopy(
        dst: *mut ::core::ffi::c_char,
        size: size_t,
        src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_uchar;
    fn CountLinks(pq: *mut QUEUE) -> COUNT;
    fn memset(
        _: *mut ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strncpy(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: size_t,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(_: *const ::core::ffi::c_char) -> size_t;
    static mut GlobData: GLOBDATA;
    fn getGameState(
        state: *mut BYTE,
        startBit: ::core::ffi::c_int,
        endBit: ::core::ffi::c_int,
    ) -> BYTE;
    fn getGameState32(state: *mut BYTE, startBit: ::core::ffi::c_int) -> DWORD;
    fn inHQSpace() -> BOOLEAN;
    fn inQuasiSpace() -> BOOLEAN;
    fn GetIndexFromStarShip(pShipQ: *mut QUEUE, hStarShip: HLINK) -> COUNT;
    static mut ScreenWidth: ::core::ffi::c_int;
    static mut ScreenHeight: ::core::ffi::c_int;
    fn WaitForAnyButton(
        newButton: BOOLEAN,
        duration: TimePeriod,
        resetInput: BOOLEAN,
    ) -> BOOLEAN;
    fn playerInSolarSystem() -> bool;
    fn playerInPlanetOrbit() -> bool;
    fn GetPlanetOrMoonName(buf: *mut UNICODE, bufsize: COUNT);
    fn SaveSolarSysLocation();
    static mut CurStarDescPtr: *mut STAR_DESC;
    static mut GameStrings: STRING;
    static mut saveDir: *mut uio_DirHandle;
    fn InventoryDevices(pDeviceMap: *mut BYTE, Size: COUNT) -> SIZE;
    static mut SpaceContext: CONTEXT;
    static mut StarConFont: FONT;
    fn ReadGroupHeader(fp: *mut GAME_STATE_FILE, pGH: *mut GROUP_HEADER);
    fn ReadShipFragment(fp: *mut GAME_STATE_FILE, FragPtr: *mut SHIP_FRAGMENT);
    fn ReadIpGroup(fp: *mut GAME_STATE_FILE, GroupPtr: *mut IP_GROUP);
    fn DrawStarConBox(
        pRect: *mut RECT,
        BorderWidth: SIZE,
        TopLeftColor: Color,
        BottomRightColor: Color,
        FillInterior: BOOLEAN,
        InteriorColor: Color,
    );
    fn SaveContextFrame(saveRect: *const RECT) -> STAMP;
    fn PutGroupInfo(offset: DWORD, which_group: BYTE) -> DWORD;
    fn OpenStateFile(
        stateFile: ::core::ffi::c_int,
        mode: *const ::core::ffi::c_char,
    ) -> *mut GAME_STATE_FILE;
    fn CloseStateFile(fp: *mut GAME_STATE_FILE);
    fn LengthStateFile(fp: *mut GAME_STATE_FILE) -> DWORD;
    fn ReadStateFile(
        lpBuf: *mut ::core::ffi::c_void,
        size: COUNT,
        count: COUNT,
        fp: *mut GAME_STATE_FILE,
    ) -> ::core::ffi::c_int;
    fn SeekStateFile(
        fp: *mut GAME_STATE_FILE,
        offset: ::core::ffi::c_long,
        whence: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn SaveSisHyperState();
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
pub type STRINGPTR = *mut ::core::ffi::c_char;
pub type CONTEXT_DESC = context_desc;
pub type FRAME_DESC = frame_desc;
pub type FONT_DESC = font_desc;
pub type DRAWABLE_DESC = drawable_desc;
pub type CONTEXT = *mut CONTEXT_DESC;
pub type FRAME = *mut FRAME_DESC;
pub type FONT = *mut FONT_DESC;
pub type DRAWABLE = *mut DRAWABLE_DESC;
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
    pub const COMMON: Self = Self(0);
    pub const CORROSIVE: Self = Self(1);
    pub const BASE_METAL: Self = Self(2);
    pub const NOBLE: Self = Self(3);
    pub const RARE_EARTH: Self = Self(4);
    pub const PRECIOUS: Self = Self(5);
    pub const RADIOACTIVE: Self = Self(6);
    pub const EXOTIC: Self = Self(7);
    pub const NUM_ELEMENT_CATEGORIES: Self = Self(8);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_0 {
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
pub type TimePeriod = DWORD;
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
pub struct CLOCK_STATE {
    pub day_index: BYTE,
    pub month_index: BYTE,
    pub year_index: COUNT,
    pub tick_count: SIZE,
    pub day_in_ticks: SIZE,
    pub event_q: QUEUE,
}
pub type HEVENT = HLINK;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event {
    pub pred: HEVENT,
    pub succ: HEVENT,
    pub day_index: BYTE,
    pub month_index: BYTE,
    pub year_index: COUNT,
    pub func_index: BYTE,
}
pub type EVENT = event;
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
pub struct GLOBDATA {
    pub SIS_state: SIS_STATE,
    pub Game_state: GAME_STATE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SUMMARY_DESC {
    pub SS: SIS_STATE,
    pub Activity: BYTE,
    pub Flags: BYTE,
    pub day_index: BYTE,
    pub month_index: BYTE,
    pub year_index: COUNT,
    pub MCreditLo: BYTE,
    pub MCreditHi: BYTE,
    pub NumShips: BYTE,
    pub NumDevices: BYTE,
    pub ShipList: [BYTE; 12],
    pub DeviceList: [BYTE; 16],
    pub SaveName: [UNICODE; 64],
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GROUP_HEADER {
    pub NumGroups: BYTE,
    pub day_index: BYTE,
    pub month_index: BYTE,
    pub star_index: COUNT,
    pub year_index: COUNT,
    pub GroupOffset: [DWORD; 65],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IP_GROUP {
    pub pred: HIPGROUP,
    pub succ: HIPGROUP,
    pub group_counter: UWORD,
    pub race_id: BYTE,
    pub sys_loc: BYTE,
    pub task: BYTE,
    pub in_system: BYTE,
    pub dest_loc: BYTE,
    pub orbit_pos: BYTE,
    pub group_id: BYTE,
    pub loc: POINT,
    pub melee_icon: FRAME,
}
pub type HIPGROUP = HLINK;
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
pub type HELEMENT = HLINK;
pub type HSHIPFRAG = HLINK;
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_3(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_3 {
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
static mut io_ok: BOOLEAN = BOOLEAN::TRUE;
#[inline]
unsafe extern "C" fn write_8(mut fp: *mut ::core::ffi::c_void, mut v: BYTE) {
    if io_ok.0 != 0 {
        if WriteResFile(
            &raw mut v as *const ::core::ffi::c_void,
            1 as size_t,
            1 as size_t,
            fp as *mut uio_Stream,
        ) != 1 as size_t
        {
            io_ok = BOOLEAN::FALSE;
        }
    }
}
#[inline]
unsafe extern "C" fn write_16(mut fp: *mut ::core::ffi::c_void, mut v: UWORD) {
    write_8(fp, (v as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as BYTE);
    write_8(
        fp,
        (v as ::core::ffi::c_int >> 8 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int)
            as BYTE,
    );
}
#[inline]
unsafe extern "C" fn write_32(mut fp: *mut ::core::ffi::c_void, mut v: DWORD) {
    write_8(fp, (v & 0xff as DWORD) as BYTE);
    write_8(fp, (v >> 8 as ::core::ffi::c_int & 0xff as DWORD) as BYTE);
    write_8(fp, (v >> 16 as ::core::ffi::c_int & 0xff as DWORD) as BYTE);
    write_8(fp, (v >> 24 as ::core::ffi::c_int & 0xff as DWORD) as BYTE);
}
#[inline]
unsafe extern "C" fn write_a8(
    mut fp: *mut ::core::ffi::c_void,
    mut ar: *const BYTE,
    mut count: COUNT,
) {
    if io_ok.0 != 0 {
        if WriteResFile(
            ar as *const ::core::ffi::c_void,
            1 as size_t,
            count as size_t,
            fp as *mut uio_Stream,
        ) != count as size_t
        {
            io_ok = BOOLEAN::FALSE;
        }
    }
}
#[inline]
unsafe extern "C" fn write_str(
    mut fp: *mut ::core::ffi::c_void,
    mut str: *const ::core::ffi::c_char,
    mut count: COUNT,
) {
    write_a8(fp, str as *const BYTE, count);
}
#[inline]
unsafe extern "C" fn write_a16(
    mut fp: *mut ::core::ffi::c_void,
    mut ar: *const UWORD,
    mut count: COUNT,
) {
    while count as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
        if io_ok.0 == 0 {
            break;
        }
        write_16(fp, *ar);
        count = count.wrapping_sub(1);
        ar = ar.offset(1);
    }
}
unsafe extern "C" fn SaveShipQueue(
    mut fh: *mut uio_Stream,
    mut pQueue: *mut QUEUE,
    mut tag: DWORD,
) {
    let mut num_links: COUNT = 0;
    let mut hStarShip: HSHIPFRAG = ::core::ptr::null_mut::<::core::ffi::c_void>();
    num_links = CountLinks(pQueue);
    if num_links as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        return;
    }
    write_32(fh as *mut ::core::ffi::c_void, tag);
    write_32(
        fh as *mut ::core::ffi::c_void,
        (num_links as ::core::ffi::c_int * 11 as ::core::ffi::c_int) as DWORD,
    );
    hStarShip = (*pQueue).head as HSHIPFRAG;
    loop {
        let c2rust_fresh2 = num_links;
        num_links = num_links.wrapping_sub(1);
        if c2rust_fresh2 == 0 {
            break;
        }
        let mut hNextShip: HSHIPFRAG = ::core::ptr::null_mut::<::core::ffi::c_void>();
        let mut FragPtr: *mut SHIP_FRAGMENT = ::core::ptr::null_mut::<SHIP_FRAGMENT>();
        let mut Index: COUNT = 0;
        FragPtr = LockShipFrag(pQueue, hStarShip);
        hNextShip = (*FragPtr).succ as HSHIPFRAG;
        Index = (*FragPtr).race_id as COUNT;
        write_16(fh as *mut ::core::ffi::c_void, Index);
        write_8(fh as *mut ::core::ffi::c_void, (*FragPtr).captains_name_index);
        write_8(fh as *mut ::core::ffi::c_void, (*FragPtr).race_id);
        write_8(fh as *mut ::core::ffi::c_void, (*FragPtr).index);
        write_16(fh as *mut ::core::ffi::c_void, (*FragPtr).crew_level);
        write_16(fh as *mut ::core::ffi::c_void, (*FragPtr).max_crew);
        write_8(fh as *mut ::core::ffi::c_void, (*FragPtr).energy_level);
        write_8(fh as *mut ::core::ffi::c_void, (*FragPtr).max_energy);
        UnlockLink(pQueue, hStarShip);
        hStarShip = hNextShip;
    };
}
unsafe extern "C" fn SaveRaceQueue(mut fh: *mut uio_Stream, mut pQueue: *mut QUEUE) {
    let mut num_links: COUNT = 0;
    let mut hFleet: HFLEETINFO = ::core::ptr::null_mut::<::core::ffi::c_void>();
    num_links = CountLinks(pQueue);
    if num_links as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        return;
    }
    write_32(fh as *mut ::core::ffi::c_void, 0x51636152 as ::core::ffi::c_int as DWORD);
    write_32(
        fh as *mut ::core::ffi::c_void,
        (num_links as ::core::ffi::c_int * 30 as ::core::ffi::c_int) as DWORD,
    );
    hFleet = (*pQueue).head as HFLEETINFO;
    loop {
        let c2rust_fresh3 = num_links;
        num_links = num_links.wrapping_sub(1);
        if c2rust_fresh3 == 0 {
            break;
        }
        let mut hNextFleet: HFLEETINFO = ::core::ptr::null_mut::<::core::ffi::c_void>();
        let mut FleetPtr: *mut FLEET_INFO = ::core::ptr::null_mut::<FLEET_INFO>();
        let mut Index: COUNT = 0;
        FleetPtr = LockFleetInfo(pQueue, hFleet);
        hNextFleet = (*FleetPtr).succ;
        Index = GetIndexFromStarShip(pQueue, hFleet);
        write_16(fh as *mut ::core::ffi::c_void, Index);
        write_16(fh as *mut ::core::ffi::c_void, (*FleetPtr).allied_state);
        write_8(fh as *mut ::core::ffi::c_void, (*FleetPtr).days_left);
        write_8(fh as *mut ::core::ffi::c_void, (*FleetPtr).growth_fract);
        write_16(fh as *mut ::core::ffi::c_void, (*FleetPtr).crew_level);
        write_16(fh as *mut ::core::ffi::c_void, (*FleetPtr).max_crew);
        write_8(fh as *mut ::core::ffi::c_void, (*FleetPtr).growth);
        write_8(fh as *mut ::core::ffi::c_void, (*FleetPtr).max_energy);
        write_16(fh as *mut ::core::ffi::c_void, (*FleetPtr).loc.x as UWORD);
        write_16(fh as *mut ::core::ffi::c_void, (*FleetPtr).loc.y as UWORD);
        write_16(fh as *mut ::core::ffi::c_void, (*FleetPtr).actual_strength);
        write_16(fh as *mut ::core::ffi::c_void, (*FleetPtr).known_strength);
        write_16(fh as *mut ::core::ffi::c_void, (*FleetPtr).known_loc.x as UWORD);
        write_16(fh as *mut ::core::ffi::c_void, (*FleetPtr).known_loc.y as UWORD);
        write_8(fh as *mut ::core::ffi::c_void, (*FleetPtr).growth_err_term);
        write_8(fh as *mut ::core::ffi::c_void, (*FleetPtr).func_index);
        write_16(fh as *mut ::core::ffi::c_void, (*FleetPtr).dest_loc.x as UWORD);
        write_16(fh as *mut ::core::ffi::c_void, (*FleetPtr).dest_loc.y as UWORD);
        UnlockLink(pQueue, hFleet);
        hFleet = hNextFleet;
    };
}
unsafe extern "C" fn SaveGroupQueue(mut fh: *mut uio_Stream, mut pQueue: *mut QUEUE) {
    let mut hGroup: HIPGROUP = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut hNextGroup: HIPGROUP = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut num_links: COUNT = 0;
    num_links = CountLinks(pQueue);
    if num_links as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        return;
    }
    write_32(fh as *mut ::core::ffi::c_void, 0x51704749 as ::core::ffi::c_int as DWORD);
    write_32(
        fh as *mut ::core::ffi::c_void,
        (num_links as ::core::ffi::c_int * 13 as ::core::ffi::c_int) as DWORD,
    );
    hGroup = (*pQueue).head as HIPGROUP;
    while !hGroup.is_null() {
        let mut GroupPtr: *mut IP_GROUP = ::core::ptr::null_mut::<IP_GROUP>();
        GroupPtr = LockIpGroup(pQueue, hGroup);
        hNextGroup = (*GroupPtr).succ;
        write_16(fh as *mut ::core::ffi::c_void, (*GroupPtr).group_counter);
        write_8(fh as *mut ::core::ffi::c_void, (*GroupPtr).race_id);
        write_8(fh as *mut ::core::ffi::c_void, (*GroupPtr).sys_loc);
        write_8(fh as *mut ::core::ffi::c_void, (*GroupPtr).task);
        write_8(fh as *mut ::core::ffi::c_void, (*GroupPtr).in_system);
        write_8(fh as *mut ::core::ffi::c_void, (*GroupPtr).dest_loc);
        write_8(fh as *mut ::core::ffi::c_void, (*GroupPtr).orbit_pos);
        write_8(fh as *mut ::core::ffi::c_void, (*GroupPtr).group_id);
        write_16(fh as *mut ::core::ffi::c_void, (*GroupPtr).loc.x as UWORD);
        write_16(fh as *mut ::core::ffi::c_void, (*GroupPtr).loc.y as UWORD);
        UnlockLink(pQueue, hGroup);
        hGroup = hNextGroup;
    }
}
unsafe extern "C" fn SaveEncounters(mut fh: *mut uio_Stream) {
    let mut num_links: COUNT = 0;
    let mut hEncounter: HENCOUNTER = ::core::ptr::null_mut::<::core::ffi::c_void>();
    num_links = CountLinks(&raw mut GlobData.Game_state.encounter_q);
    if num_links as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        return;
    }
    write_32(fh as *mut ::core::ffi::c_void, 0x74636e45 as ::core::ffi::c_int as DWORD);
    write_32(
        fh as *mut ::core::ffi::c_void,
        (65 as ::core::ffi::c_int * num_links as ::core::ffi::c_int) as DWORD,
    );
    hEncounter = GlobData.Game_state.encounter_q.head as HENCOUNTER;
    loop {
        let c2rust_fresh0 = num_links;
        num_links = num_links.wrapping_sub(1);
        if c2rust_fresh0 == 0 {
            break;
        }
        let mut hNextEncounter: HENCOUNTER = ::core::ptr::null_mut::<
            ::core::ffi::c_void,
        >();
        let mut EncounterPtr: *mut ENCOUNTER = ::core::ptr::null_mut::<ENCOUNTER>();
        let mut i: COUNT = 0;
        EncounterPtr = LockLink(&raw mut GlobData.Game_state.encounter_q, hEncounter)
            as *mut ENCOUNTER;
        hNextEncounter = (*EncounterPtr).succ;
        write_16(
            fh as *mut ::core::ffi::c_void,
            (*EncounterPtr).transition_state as UWORD,
        );
        write_16(fh as *mut ::core::ffi::c_void, (*EncounterPtr).origin.x as UWORD);
        write_16(fh as *mut ::core::ffi::c_void, (*EncounterPtr).origin.y as UWORD);
        write_16(fh as *mut ::core::ffi::c_void, (*EncounterPtr).radius);
        write_16(fh as *mut ::core::ffi::c_void, (*EncounterPtr).loc_pt.x as UWORD);
        write_16(fh as *mut ::core::ffi::c_void, (*EncounterPtr).loc_pt.y as UWORD);
        write_8(fh as *mut ::core::ffi::c_void, (*EncounterPtr).race_id);
        write_8(fh as *mut ::core::ffi::c_void, (*EncounterPtr).num_ships);
        write_8(fh as *mut ::core::ffi::c_void, (*EncounterPtr).flags);
        i = 0 as COUNT;
        while (i as ::core::ffi::c_int) < 7 as ::core::ffi::c_int {
            let mut ShipInfo: *const BRIEF_SHIP_INFO = (&raw mut (*EncounterPtr).ShipList
                as *mut BRIEF_SHIP_INFO)
                .offset(i as isize);
            write_8(fh as *mut ::core::ffi::c_void, (*ShipInfo).race_id);
            write_16(fh as *mut ::core::ffi::c_void, (*ShipInfo).crew_level);
            write_16(fh as *mut ::core::ffi::c_void, (*ShipInfo).max_crew);
            write_8(fh as *mut ::core::ffi::c_void, (*ShipInfo).max_energy);
            i = i.wrapping_add(1);
        }
        write_32(fh as *mut ::core::ffi::c_void, (*EncounterPtr).log_x as DWORD);
        write_32(fh as *mut ::core::ffi::c_void, (*EncounterPtr).log_y as DWORD);
        UnlockLink(&raw mut GlobData.Game_state.encounter_q, hEncounter);
        hEncounter = hNextEncounter;
    };
}
unsafe extern "C" fn SaveEvents(mut fh: *mut uio_Stream) {
    let mut num_links: COUNT = 0;
    let mut hEvent: HEVENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
    num_links = CountLinks(&raw mut GlobData.Game_state.GameClock.event_q);
    if num_links as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        return;
    }
    write_32(fh as *mut ::core::ffi::c_void, 0x73747645 as ::core::ffi::c_int as DWORD);
    write_32(
        fh as *mut ::core::ffi::c_void,
        (num_links as ::core::ffi::c_int * 5 as ::core::ffi::c_int) as DWORD,
    );
    hEvent = GlobData.Game_state.GameClock.event_q.head as HEVENT;
    loop {
        let c2rust_fresh1 = num_links;
        num_links = num_links.wrapping_sub(1);
        if c2rust_fresh1 == 0 {
            break;
        }
        let mut hNextEvent: HEVENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
        let mut EventPtr: *mut EVENT = ::core::ptr::null_mut::<EVENT>();
        EventPtr = LockLink(&raw mut GlobData.Game_state.GameClock.event_q, hEvent)
            as *mut EVENT;
        hNextEvent = (*EventPtr).succ;
        write_8(fh as *mut ::core::ffi::c_void, (*EventPtr).day_index);
        write_8(fh as *mut ::core::ffi::c_void, (*EventPtr).month_index);
        write_16(fh as *mut ::core::ffi::c_void, (*EventPtr).year_index);
        write_8(fh as *mut ::core::ffi::c_void, (*EventPtr).func_index);
        UnlockLink(&raw mut GlobData.Game_state.GameClock.event_q, hEvent);
        hEvent = hNextEvent;
    };
}
unsafe extern "C" fn SaveClockState(
    mut ClockPtr: *const CLOCK_STATE,
    mut fh: *mut uio_Stream,
) {
    write_8(fh as *mut ::core::ffi::c_void, (*ClockPtr).day_index);
    write_8(fh as *mut ::core::ffi::c_void, (*ClockPtr).month_index);
    write_16(fh as *mut ::core::ffi::c_void, (*ClockPtr).year_index);
    write_16(fh as *mut ::core::ffi::c_void, (*ClockPtr).tick_count as UWORD);
    write_16(fh as *mut ::core::ffi::c_void, (*ClockPtr).day_in_ticks as UWORD);
}
unsafe extern "C" fn SaveGameState(
    mut GSPtr: *const GAME_STATE,
    mut fh: *mut uio_Stream,
) {
    write_32(fh as *mut ::core::ffi::c_void, 0x74536c47 as ::core::ffi::c_int as DWORD);
    write_32(fh as *mut ::core::ffi::c_void, 75 as DWORD);
    write_8(fh as *mut ::core::ffi::c_void, (*GSPtr).glob_flags);
    write_8(fh as *mut ::core::ffi::c_void, (*GSPtr).CrewCost);
    write_8(fh as *mut ::core::ffi::c_void, (*GSPtr).FuelCost);
    write_a8(
        fh as *mut ::core::ffi::c_void,
        &raw const (*GSPtr).ModuleCost as *const BYTE,
        C2Rust_Unnamed_0::NUM_MODULES.0 as ::core::ffi::c_int as COUNT,
    );
    write_a8(
        fh as *mut ::core::ffi::c_void,
        &raw const (*GSPtr).ElementWorth as *const BYTE,
        C2Rust_Unnamed::NUM_ELEMENT_CATEGORIES.0 as ::core::ffi::c_int as COUNT,
    );
    write_16(fh as *mut ::core::ffi::c_void, (*GSPtr).CurrentActivity);
    SaveClockState(&raw const (*GSPtr).GameClock, fh);
    write_16(fh as *mut ::core::ffi::c_void, (*GSPtr).autopilot.x as UWORD);
    write_16(fh as *mut ::core::ffi::c_void, (*GSPtr).autopilot.y as UWORD);
    write_16(fh as *mut ::core::ffi::c_void, (*GSPtr).ip_location.x as UWORD);
    write_16(fh as *mut ::core::ffi::c_void, (*GSPtr).ip_location.y as UWORD);
    write_16(fh as *mut ::core::ffi::c_void, (*GSPtr).ShipStamp.origin.x as UWORD);
    write_16(fh as *mut ::core::ffi::c_void, (*GSPtr).ShipStamp.origin.y as UWORD);
    write_16(fh as *mut ::core::ffi::c_void, (*GSPtr).ShipFacing);
    write_8(fh as *mut ::core::ffi::c_void, (*GSPtr).ip_planet);
    write_8(fh as *mut ::core::ffi::c_void, (*GSPtr).in_orbit);
    write_16(fh as *mut ::core::ffi::c_void, (*GSPtr).velocity.TravelAngle);
    write_16(fh as *mut ::core::ffi::c_void, (*GSPtr).velocity.vector.width as UWORD);
    write_16(fh as *mut ::core::ffi::c_void, (*GSPtr).velocity.vector.height as UWORD);
    write_16(fh as *mut ::core::ffi::c_void, (*GSPtr).velocity.fract.width as UWORD);
    write_16(fh as *mut ::core::ffi::c_void, (*GSPtr).velocity.fract.height as UWORD);
    write_16(fh as *mut ::core::ffi::c_void, (*GSPtr).velocity.error.width as UWORD);
    write_16(fh as *mut ::core::ffi::c_void, (*GSPtr).velocity.error.height as UWORD);
    write_16(fh as *mut ::core::ffi::c_void, (*GSPtr).velocity.incr.width as UWORD);
    write_16(fh as *mut ::core::ffi::c_void, (*GSPtr).velocity.incr.height as UWORD);
    write_32(fh as *mut ::core::ffi::c_void, 0x74536d47 as ::core::ffi::c_int as DWORD);
    write_32(
        fh as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[BYTE; 155]>() as DWORD,
    );
    write_a8(
        fh as *mut ::core::ffi::c_void,
        &raw const (*GSPtr).GameState as *const BYTE,
        ::core::mem::size_of::<[BYTE; 155]>() as COUNT,
    );
}
unsafe extern "C" fn SaveSisState(
    mut SSPtr: *const SIS_STATE,
    mut fp: *mut ::core::ffi::c_void,
) {
    write_32(fp, (*SSPtr).log_x as DWORD);
    write_32(fp, (*SSPtr).log_y as DWORD);
    write_32(fp, (*SSPtr).ResUnits);
    write_32(fp, (*SSPtr).FuelOnBoard);
    write_16(fp, (*SSPtr).CrewEnlisted);
    write_16(fp, (*SSPtr).TotalElementMass);
    write_16(fp, (*SSPtr).TotalBioMass);
    write_a8(fp, &raw const (*SSPtr).ModuleSlots as *const BYTE, 16 as COUNT);
    write_a8(fp, &raw const (*SSPtr).DriveSlots as *const BYTE, 11 as COUNT);
    write_a8(fp, &raw const (*SSPtr).JetSlots as *const BYTE, 8 as COUNT);
    write_8(fp, (*SSPtr).NumLanders);
    write_a16(
        fp,
        &raw const (*SSPtr).ElementAmounts as *const UWORD,
        C2Rust_Unnamed::NUM_ELEMENT_CATEGORIES.0 as ::core::ffi::c_int as COUNT,
    );
    write_str(
        fp,
        &raw const (*SSPtr).ShipName as *const ::core::ffi::c_char,
        16 as COUNT,
    );
    write_str(
        fp,
        &raw const (*SSPtr).CommanderName as *const ::core::ffi::c_char,
        16 as COUNT,
    );
    write_str(
        fp,
        &raw const (*SSPtr).PlanetName as *const ::core::ffi::c_char,
        16 as COUNT,
    );
}
unsafe extern "C" fn SaveSummary(
    mut SummPtr: *const SUMMARY_DESC,
    mut fp: *mut ::core::ffi::c_void,
) {
    write_32(fp, 0x6d6d7553 as ::core::ffi::c_int as DWORD);
    write_32(
        fp,
        (160 as size_t)
            .wrapping_add(
                strlen(&raw const (*SummPtr).SaveName as *const ::core::ffi::c_char),
            ) as DWORD,
    );
    SaveSisState(&raw const (*SummPtr).SS, fp);
    write_8(fp, (*SummPtr).Activity);
    write_8(fp, (*SummPtr).Flags);
    write_8(fp, (*SummPtr).day_index);
    write_8(fp, (*SummPtr).month_index);
    write_16(fp, (*SummPtr).year_index);
    write_8(fp, (*SummPtr).MCreditLo);
    write_8(fp, (*SummPtr).MCreditHi);
    write_8(fp, (*SummPtr).NumShips);
    write_8(fp, (*SummPtr).NumDevices);
    write_a8(fp, &raw const (*SummPtr).ShipList as *const BYTE, 12 as COUNT);
    write_a8(fp, &raw const (*SummPtr).DeviceList as *const BYTE, 16 as COUNT);
    write_a8(
        fp,
        &raw const (*SummPtr).SaveName as *const UNICODE as *mut BYTE,
        strlen(&raw const (*SummPtr).SaveName as *const ::core::ffi::c_char) as COUNT,
    );
}
unsafe extern "C" fn SaveStarDesc(mut SDPtr: *const STAR_DESC, mut fh: *mut uio_Stream) {
    write_32(fh as *mut ::core::ffi::c_void, 0x72617453 as ::core::ffi::c_int as DWORD);
    write_32(fh as *mut ::core::ffi::c_void, 8 as DWORD);
    write_16(fh as *mut ::core::ffi::c_void, (*SDPtr).star_pt.x as UWORD);
    write_16(fh as *mut ::core::ffi::c_void, (*SDPtr).star_pt.y as UWORD);
    write_8(fh as *mut ::core::ffi::c_void, (*SDPtr).Type);
    write_8(fh as *mut ::core::ffi::c_void, (*SDPtr).Index);
    write_8(fh as *mut ::core::ffi::c_void, (*SDPtr).Prefix);
    write_8(fh as *mut ::core::ffi::c_void, (*SDPtr).Postfix);
}
unsafe extern "C" fn PrepareSummary(
    mut SummPtr: *mut SUMMARY_DESC,
    mut name: *const ::core::ffi::c_char,
) {
    (*SummPtr).SS = GlobData.SIS_state;
    (*SummPtr).Activity = GlobData.Game_state.CurrentActivity as BYTE;
    match (*SummPtr).Activity as ::core::ffi::c_int {
        3 => {
            if inQuasiSpace().0 != 0 {
                (*SummPtr).Activity = C2Rust_Unnamed_2::IN_QUASISPACE.0
                    as ::core::ffi::c_int as BYTE;
            }
        }
        4 => {
            GetPlanetOrMoonName(
                &raw mut (*SummPtr).SS.PlanetName as *mut UNICODE,
                ::core::mem::size_of::<[UNICODE; 16]>() as COUNT,
            );
            if getGameState(
                &raw mut GlobData.Game_state.GameState as *mut BYTE,
                C2Rust_Unnamed_1::GLOBAL_FLAGS_AND_DATA.0 as ::core::ffi::c_int,
                C2Rust_Unnamed_1::END_GLOBAL_FLAGS_AND_DATA.0 as ::core::ffi::c_int,
            ) as ::core::ffi::c_int
                == !(0 as ::core::ffi::c_int) as BYTE as ::core::ffi::c_int
            {
                (*SummPtr).Activity = C2Rust_Unnamed_2::IN_STARBASE.0
                    as ::core::ffi::c_int as BYTE;
            } else if playerInPlanetOrbit() {
                (*SummPtr).Activity = C2Rust_Unnamed_2::IN_PLANET_ORBIT.0
                    as ::core::ffi::c_int as BYTE;
            }
        }
        1 => {
            utf8StringCopy(
                &raw mut (*SummPtr).SS.PlanetName as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[UNICODE; 16]>(),
                GetStringAddress(
                    SetAbsStringTableIndex(
                        GameStrings,
                        (C2Rust_Unnamed_3::PLANET_NUMBER_BASE.0 as ::core::ffi::c_int
                            + 32 as ::core::ffi::c_int) as COUNT,
                    ),
                ),
            );
        }
        _ => {}
    }
    (*SummPtr).MCreditLo = getGameState(
        &raw mut GlobData.Game_state.GameState as *mut BYTE,
        C2Rust_Unnamed_1::MELNORME_CREDIT0.0 as ::core::ffi::c_int,
        C2Rust_Unnamed_1::END_MELNORME_CREDIT0.0 as ::core::ffi::c_int,
    );
    (*SummPtr).MCreditHi = getGameState(
        &raw mut GlobData.Game_state.GameState as *mut BYTE,
        C2Rust_Unnamed_1::MELNORME_CREDIT1.0 as ::core::ffi::c_int,
        C2Rust_Unnamed_1::END_MELNORME_CREDIT1.0 as ::core::ffi::c_int,
    );
    let mut hStarShip: HSHIPFRAG = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut hNextShip: HSHIPFRAG = ::core::ptr::null_mut::<::core::ffi::c_void>();
    hStarShip = GlobData.Game_state.built_ship_q.head as HSHIPFRAG;
    (*SummPtr).NumShips = 0 as BYTE;
    while !hStarShip.is_null() {
        let mut StarShipPtr: *mut SHIP_FRAGMENT = ::core::ptr::null_mut::<
            SHIP_FRAGMENT,
        >();
        StarShipPtr = LockShipFrag(&raw mut GlobData.Game_state.built_ship_q, hStarShip);
        hNextShip = (*StarShipPtr).succ as HSHIPFRAG;
        (*SummPtr).ShipList[(*SummPtr).NumShips as usize] = (*StarShipPtr).race_id;
        UnlockLink(&raw mut GlobData.Game_state.built_ship_q, hStarShip);
        hStarShip = hNextShip;
        (*SummPtr).NumShips = (*SummPtr).NumShips.wrapping_add(1);
    }
    (*SummPtr).NumDevices = InventoryDevices(
        &raw mut (*SummPtr).DeviceList as *mut BYTE,
        16 as COUNT,
    ) as BYTE;
    (*SummPtr).Flags = (getGameState(
        &raw mut GlobData.Game_state.GameState as *mut BYTE,
        C2Rust_Unnamed_1::LANDER_SHIELDS.0 as ::core::ffi::c_int,
        C2Rust_Unnamed_1::END_LANDER_SHIELDS.0 as ::core::ffi::c_int,
    ) as ::core::ffi::c_int
        | (getGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_1::IMPROVED_LANDER_SPEED.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_1::END_IMPROVED_LANDER_SPEED.0 as ::core::ffi::c_int,
        ) as ::core::ffi::c_int) << 4 as ::core::ffi::c_int + 0 as ::core::ffi::c_int
        | (getGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_1::IMPROVED_LANDER_CARGO.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_1::END_IMPROVED_LANDER_CARGO.0 as ::core::ffi::c_int,
        ) as ::core::ffi::c_int) << 4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
        | (getGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_1::IMPROVED_LANDER_SHOT.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_1::END_IMPROVED_LANDER_SHOT.0 as ::core::ffi::c_int,
        ) as ::core::ffi::c_int) << 4 as ::core::ffi::c_int + 2 as ::core::ffi::c_int
        | if (getGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_1::CHMMR_BOMB_STATE.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_1::END_CHMMR_BOMB_STATE.0 as ::core::ffi::c_int,
        ) as ::core::ffi::c_int) < 2 as ::core::ffi::c_int
        {
            0 as ::core::ffi::c_int
        } else {
            1 as ::core::ffi::c_int
        } << 4 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as BYTE;
    (*SummPtr).day_index = GlobData.Game_state.GameClock.day_index;
    (*SummPtr).month_index = GlobData.Game_state.GameClock.month_index;
    (*SummPtr).year_index = GlobData.Game_state.GameClock.year_index;
    (*SummPtr).SaveName[(64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as usize] = 0
        as UNICODE;
    strncpy(
        &raw mut (*SummPtr).SaveName as *mut ::core::ffi::c_char,
        name,
        (64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as size_t,
    );
}
unsafe extern "C" fn SaveProblemMessage(mut MsgStamp: *mut STAMP) {
    let mut r: RECT = rect {
        corner: point {
            x: 0 as COORD,
            y: 0 as COORD,
        },
        extent: extent {
            width: 0 as COORD,
            height: 0 as COORD,
        },
    };
    let mut i: COUNT = 0;
    let mut t: TEXT = TEXT {
        baseline: POINT { x: 0, y: 0 },
        pStr: ::core::ptr::null::<UNICODE>(),
        align: TEXT_ALIGN::ALIGN_LEFT,
        CharCount: 0,
    };
    let mut ppStr: [*mut UNICODE; 1] = [::core::ptr::null_mut::<UNICODE>(); 1];
    ppStr[0usize] = GetStringAddress(
        SetAbsStringTableIndex(
            GameStrings,
            (C2Rust_Unnamed_3::SAVEGAME_STRING_BASE.0 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int) as COUNT,
        ),
    );
    SetContextFont(StarConFont);
    t.baseline.y = 0 as COORD;
    t.baseline.x = t.baseline.y;
    t.align = TEXT_ALIGN::ALIGN_CENTER;
    i = 0 as COUNT;
    while (i as ::core::ffi::c_int) < 1 as ::core::ffi::c_int {
        let mut tr: RECT = RECT {
            corner: POINT { x: 0, y: 0 },
            extent: EXTENT { width: 0, height: 0 },
        };
        t.pStr = ppStr[i as usize];
        if *t.pStr as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
            break;
        }
        t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
        TextRect(&raw mut t, &raw mut tr, ::core::ptr::null_mut::<BYTE>());
        if i as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            r = tr;
        } else {
            BoxUnion(&raw mut tr, &raw mut r, &raw mut r);
        }
        t.baseline.y = (t.baseline.y as ::core::ffi::c_int + 11 as ::core::ffi::c_int)
            as COORD;
        i = i.wrapping_add(1);
    }
    t.baseline.x = ((ScreenWidth - 64 as ::core::ffi::c_int
        - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int - 14 as ::core::ffi::c_int
        >> 1 as ::core::ffi::c_int)
        - (r.extent.width as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
        - r.corner.x as ::core::ffi::c_int) as COORD;
    t.baseline.y = ((ScreenHeight - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
        - 13 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
        - (r.extent.height as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
        - r.corner.y as ::core::ffi::c_int) as COORD;
    r.corner.x = (r.corner.x as ::core::ffi::c_int
        + (t.baseline.x as ::core::ffi::c_int - 4 as ::core::ffi::c_int)) as COORD;
    r.corner.y = (r.corner.y as ::core::ffi::c_int
        + (t.baseline.y as ::core::ffi::c_int - 4 as ::core::ffi::c_int)) as COORD;
    r.extent.width = (r.extent.width as ::core::ffi::c_int + 8 as ::core::ffi::c_int)
        as COORD;
    r.extent.height = (r.extent.height as ::core::ffi::c_int + 8 as ::core::ffi::c_int)
        as COORD;
    *MsgStamp = SaveContextFrame(&raw mut r);
    BatchGraphics();
    DrawStarConBox(
        &raw mut r,
        2 as SIZE,
        colorFromRgb15(0x10 as BYTE, 0x10 as BYTE, 0x10 as BYTE),
        colorFromRgb15(0x8 as BYTE, 0x8 as BYTE, 0x8 as BYTE),
        BOOLEAN::TRUE,
        colorFromRgb15(0xa as BYTE, 0xa as BYTE, 0xa as BYTE),
    );
    SetContextForeGroundColor(colorFromRgb15(0x14 as BYTE, 0x14 as BYTE, 0x14 as BYTE));
    i = 0 as COUNT;
    while (i as ::core::ffi::c_int) < 1 as ::core::ffi::c_int {
        t.pStr = ppStr[i as usize];
        if *t.pStr as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
            break;
        }
        t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
        font_DrawText(&raw mut t);
        t.baseline.y = (t.baseline.y as ::core::ffi::c_int + 11 as ::core::ffi::c_int)
            as COORD;
        i = i.wrapping_add(1);
    }
    UnbatchGraphics();
}
#[no_mangle]
pub unsafe extern "C" fn SaveProblem() {
    let mut s: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    let mut OldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    OldContext = SetContext(SpaceContext);
    SaveProblemMessage(&raw mut s);
    FlushGraphics();
    WaitForAnyButton(
        BOOLEAN::TRUE,
        -1 as ::core::ffi::c_int as TimePeriod,
        BOOLEAN::FALSE,
    );
    DrawStamp(&raw mut s);
    SetContext(OldContext);
    DestroyDrawable(ReleaseDrawable(s.frame));
}
unsafe extern "C" fn SaveFlagshipState() {
    if inHQSpace().0 != 0 {
        SaveSisHyperState();
    } else if playerInSolarSystem() {
        SaveSolarSysLocation();
    }
}
unsafe extern "C" fn SaveStarInfo(mut fh: *mut uio_Stream) {
    let mut fp: *mut GAME_STATE_FILE = ::core::ptr::null_mut::<GAME_STATE_FILE>();
    fp = OpenStateFile(
        0 as ::core::ffi::c_int,
        b"rb\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !fp.is_null() {
        let mut flen: DWORD = LengthStateFile(fp);
        if flen.wrapping_rem(4 as DWORD) != 0 {
            log_add(
                log_Level::log_Warning,
                b"Unexpected Star Info length! Expected an integral number of DWORDS.\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        } else {
            write_32(
                fh as *mut ::core::ffi::c_void,
                0x6e616353 as ::core::ffi::c_int as DWORD,
            );
            write_32(fh as *mut ::core::ffi::c_void, flen);
            while flen != 0 {
                let mut val: DWORD = 0;
                sread_32(fp, &raw mut val);
                write_32(fh as *mut ::core::ffi::c_void, val);
                flen = flen.wrapping_sub(4 as DWORD);
            }
        }
        CloseStateFile(fp);
    }
}
unsafe extern "C" fn SaveBattleGroup(
    mut fp: *mut GAME_STATE_FILE,
    mut encounter_id: DWORD,
    mut grpoffs: DWORD,
    mut fh: *mut uio_Stream,
) {
    let mut h: GROUP_HEADER = GROUP_HEADER {
        NumGroups: 0,
        day_index: 0,
        month_index: 0,
        star_index: 0,
        year_index: 0,
        GroupOffset: [0; 65],
    };
    let mut size: DWORD = 12 as DWORD;
    let mut i: ::core::ffi::c_int = 0;
    SeekStateFile(fp, grpoffs as ::core::ffi::c_long, 0 as ::core::ffi::c_int);
    ReadGroupHeader(fp, &raw mut h);
    i = 1 as ::core::ffi::c_int;
    while i <= h.NumGroups as ::core::ffi::c_int {
        let mut NumShips: BYTE = 0;
        SeekStateFile(
            fp,
            h.GroupOffset[i as usize] as ::core::ffi::c_long,
            0 as ::core::ffi::c_int,
        );
        sread_8(fp, ::core::ptr::null_mut::<BYTE>());
        sread_8(fp, &raw mut NumShips);
        size = size
            .wrapping_add(
                (2 as ::core::ffi::c_int
                    + 10 as ::core::ffi::c_int * NumShips as ::core::ffi::c_int) as DWORD,
            );
        i += 1;
    }
    write_32(fh as *mut ::core::ffi::c_void, 0x70477442 as ::core::ffi::c_int as DWORD);
    write_32(fh as *mut ::core::ffi::c_void, size);
    write_32(fh as *mut ::core::ffi::c_void, encounter_id);
    write_8(
        fh as *mut ::core::ffi::c_void,
        (if grpoffs != 0 && GlobData.Game_state.BattleGroupRef == grpoffs {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as BYTE,
    );
    write_16(fh as *mut ::core::ffi::c_void, h.star_index);
    write_8(fh as *mut ::core::ffi::c_void, h.day_index);
    write_8(fh as *mut ::core::ffi::c_void, h.month_index);
    write_16(fh as *mut ::core::ffi::c_void, h.year_index);
    write_8(fh as *mut ::core::ffi::c_void, h.NumGroups);
    i = 1 as ::core::ffi::c_int;
    while i <= h.NumGroups as ::core::ffi::c_int {
        let mut j: ::core::ffi::c_int = 0;
        let mut b: BYTE = 0;
        SeekStateFile(
            fp,
            h.GroupOffset[i as usize] as ::core::ffi::c_long,
            0 as ::core::ffi::c_int,
        );
        sread_8(fp, &raw mut b);
        write_8(fh as *mut ::core::ffi::c_void, b);
        sread_8(fp, &raw mut b);
        write_8(fh as *mut ::core::ffi::c_void, b);
        j = 0 as ::core::ffi::c_int;
        while j < b as ::core::ffi::c_int {
            let mut race_outer: BYTE = 0;
            let mut sf: SHIP_FRAGMENT = SHIP_FRAGMENT {
                pred: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                succ: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                SpeciesID: SPECIES_ID::NO_ID,
                captains_name_index: 0,
                race_id: 0,
                index: 0,
                crew_level: 0,
                max_crew: 0,
                energy_level: 0,
                max_energy: 0,
                race_strings: ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>(),
                icons: ::core::ptr::null_mut::<FRAME_DESC>(),
                melee_icon: ::core::ptr::null_mut::<FRAME_DESC>(),
            };
            sread_8(fp, &raw mut race_outer);
            ReadShipFragment(fp, &raw mut sf);
            write_8(fh as *mut ::core::ffi::c_void, race_outer);
            write_8(fh as *mut ::core::ffi::c_void, sf.captains_name_index);
            write_8(fh as *mut ::core::ffi::c_void, sf.race_id);
            write_8(fh as *mut ::core::ffi::c_void, sf.index);
            write_16(fh as *mut ::core::ffi::c_void, sf.crew_level);
            write_16(fh as *mut ::core::ffi::c_void, sf.max_crew);
            write_8(fh as *mut ::core::ffi::c_void, sf.energy_level);
            write_8(fh as *mut ::core::ffi::c_void, sf.max_energy);
            j += 1;
        }
        i += 1;
    }
}
unsafe extern "C" fn SaveGroups(mut fh: *mut uio_Stream) {
    let mut fp: *mut GAME_STATE_FILE = ::core::ptr::null_mut::<GAME_STATE_FILE>();
    fp = OpenStateFile(
        1 as ::core::ffi::c_int,
        b"rb\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !fp.is_null() && LengthStateFile(fp) > 0 as DWORD {
        let mut h: GROUP_HEADER = GROUP_HEADER {
            NumGroups: 0,
            day_index: 0,
            month_index: 0,
            star_index: 0,
            year_index: 0,
            GroupOffset: [0; 65],
        };
        let mut lastenc: BYTE = 0;
        let mut count: BYTE = 0;
        let mut i: ::core::ffi::c_int = 0;
        ReadGroupHeader(fp, &raw mut h);
        SeekStateFile(
            fp,
            h.GroupOffset[0usize] as ::core::ffi::c_long,
            0 as ::core::ffi::c_int,
        );
        sread_8(fp, &raw mut lastenc);
        sread_8(fp, &raw mut count);
        write_32(
            fh as *mut ::core::ffi::c_void,
            0x73707247 as ::core::ffi::c_int as DWORD,
        );
        write_32(
            fh as *mut ::core::ffi::c_void,
            (1 as ::core::ffi::c_int
                + 14 as ::core::ffi::c_int * count as ::core::ffi::c_int) as DWORD,
        );
        write_8(fh as *mut ::core::ffi::c_void, lastenc);
        i = 0 as ::core::ffi::c_int;
        while i < count as ::core::ffi::c_int {
            let mut race_outer: BYTE = 0;
            let mut ip: IP_GROUP = IP_GROUP {
                pred: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                succ: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                group_counter: 0,
                race_id: 0,
                sys_loc: 0,
                task: 0,
                in_system: 0,
                dest_loc: 0,
                orbit_pos: 0,
                group_id: 0,
                loc: POINT { x: 0, y: 0 },
                melee_icon: ::core::ptr::null_mut::<FRAME_DESC>(),
            };
            sread_8(fp, &raw mut race_outer);
            ReadIpGroup(fp, &raw mut ip);
            write_8(fh as *mut ::core::ffi::c_void, race_outer);
            write_16(fh as *mut ::core::ffi::c_void, ip.group_counter);
            write_8(fh as *mut ::core::ffi::c_void, ip.race_id);
            write_8(fh as *mut ::core::ffi::c_void, ip.sys_loc);
            write_8(fh as *mut ::core::ffi::c_void, ip.task);
            write_8(fh as *mut ::core::ffi::c_void, ip.in_system);
            write_8(fh as *mut ::core::ffi::c_void, ip.dest_loc);
            write_8(fh as *mut ::core::ffi::c_void, ip.orbit_pos);
            write_8(fh as *mut ::core::ffi::c_void, ip.group_id);
            write_16(fh as *mut ::core::ffi::c_void, ip.loc.x as UWORD);
            write_16(fh as *mut ::core::ffi::c_void, ip.loc.y as UWORD);
            i += 1;
        }
        SaveBattleGroup(fp, 0 as DWORD, 0 as DWORD, fh);
        CloseStateFile(fp);
    }
    fp = OpenStateFile(
        2 as ::core::ffi::c_int,
        b"rb\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !fp.is_null() && LengthStateFile(fp) > 0 as DWORD {
        let mut state_index: ::core::ffi::c_int = C2Rust_Unnamed_1::SHOFIXTI_GRPOFFS0.0
            as ::core::ffi::c_int;
        let mut encounter_index: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        while state_index < C2Rust_Unnamed_1::NUM_GAME_STATE_BITS.0 as ::core::ffi::c_int
        {
            let mut grpoffs: DWORD = getGameState32(
                &raw mut GlobData.Game_state.GameState as *mut BYTE,
                state_index,
            );
            if grpoffs != 0 {
                SaveBattleGroup(fp, encounter_index as DWORD, grpoffs, fh);
            }
            encounter_index += 1;
            state_index += 32 as ::core::ffi::c_int;
        }
        CloseStateFile(fp);
    }
}
#[no_mangle]
pub unsafe extern "C" fn SaveGame(
    mut which_game: COUNT,
    mut SummPtr: *mut SUMMARY_DESC,
    mut name: *const ::core::ffi::c_char,
) -> BOOLEAN {
    let mut out_fp: *mut uio_Stream = ::core::ptr::null_mut::<uio_Stream>();
    let mut pt: POINT = POINT { x: 0, y: 0 };
    let mut SD: STAR_DESC = STAR_DESC {
        star_pt: POINT { x: 0, y: 0 },
        Type: 0,
        Index: 0,
        Prefix: 0,
        Postfix: 0,
    };
    let mut file: [::core::ffi::c_char; 4096] = [0; 4096];
    if !CurStarDescPtr.is_null() {
        SD = *CurStarDescPtr;
    } else {
        memset(
            &raw mut SD as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<STAR_DESC>(),
        );
    }
    pt = GlobData.Game_state.ip_location;
    SaveFlagshipState();
    if GlobData.Game_state.CurrentActivity as BYTE as ::core::ffi::c_int
        == C2Rust_Unnamed_2::IN_INTERPLANETARY.0 as ::core::ffi::c_int
        && GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
            & (C2Rust_Unnamed_2::START_ENCOUNTER.0 as ::core::ffi::c_int
                | C2Rust_Unnamed_2::START_INTERPLANETARY.0 as ::core::ffi::c_int) == 0
    {
        PutGroupInfo(
            0 as ::core::ffi::c_long as DWORD,
            !(0 as ::core::ffi::c_int) as BYTE,
        );
    }
    sprintf(
        &raw mut file as *mut ::core::ffi::c_char,
        b"uqmsave.%02u\0".as_ptr() as *const ::core::ffi::c_char,
        which_game as ::core::ffi::c_int,
    );
    out_fp = res_OpenResFile(
        saveDir,
        &raw mut file as *mut ::core::ffi::c_char,
        b"wb\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !out_fp.is_null() {
        io_ok = BOOLEAN::TRUE;
        write_32(
            out_fp as *mut ::core::ffi::c_void,
            0x1534d55 as ::core::ffi::c_int as DWORD,
        );
        PrepareSummary(SummPtr, name);
        SaveSummary(SummPtr, out_fp as *mut ::core::ffi::c_void);
        SaveGameState(&raw mut GlobData.Game_state, out_fp);
        GlobData.Game_state.ip_location = pt;
        GlobData.Game_state.in_orbit = 0 as BYTE;
        SaveRaceQueue(out_fp, &raw mut GlobData.Game_state.avail_race_q);
        if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
            & C2Rust_Unnamed_2::START_INTERPLANETARY.0 as ::core::ffi::c_int == 0
        {
            if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
                & C2Rust_Unnamed_2::START_ENCOUNTER.0 as ::core::ffi::c_int != 0
            {
                SaveShipQueue(
                    out_fp,
                    &raw mut GlobData.Game_state.npc_built_ship_q,
                    0x5163704e as ::core::ffi::c_int as DWORD,
                );
            } else if GlobData.Game_state.CurrentActivity as BYTE as ::core::ffi::c_int
                == C2Rust_Unnamed_2::IN_INTERPLANETARY.0 as ::core::ffi::c_int
            {
                SaveGroupQueue(out_fp, &raw mut GlobData.Game_state.ip_group_q);
            }
        }
        SaveShipQueue(
            out_fp,
            &raw mut GlobData.Game_state.built_ship_q,
            0x51706853 as ::core::ffi::c_int as DWORD,
        );
        SaveEvents(out_fp);
        SaveEncounters(out_fp);
        SaveStarInfo(out_fp);
        SaveGroups(out_fp);
        SaveStarDesc(&raw mut SD, out_fp);
        res_CloseResFile(out_fp);
        if io_ok.0 == 0 {
            DeleteResFile(saveDir, &raw mut file as *mut ::core::ffi::c_char);
            return BOOLEAN::FALSE;
        }
    } else {
        return BOOLEAN::FALSE
    }
    return BOOLEAN::TRUE;
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
#[inline]
unsafe extern "C" fn LockIpGroup(
    mut pq: *const QUEUE,
    mut h: HIPGROUP,
) -> *mut IP_GROUP {
    ((*pq).object_size as usize == ::core::mem::size_of::<IP_GROUP>()
        || {
            __assert_fail(
                b"GetLinkSize (pq) == sizeof (IP_GROUP)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/uqm/grpinfo.h\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                80 as ::core::ffi::c_int,
                b"LockIpGroup\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    return LockLink(pq, h) as *mut IP_GROUP;
}
#[inline]
unsafe extern "C" fn sread_8(mut fp: *mut GAME_STATE_FILE, mut v: *mut BYTE) -> COUNT {
    let mut t: BYTE = 0;
    if v.is_null() {
        v = &raw mut t;
    }
    return ReadStateFile(v as *mut ::core::ffi::c_void, 1 as COUNT, 1 as COUNT, fp)
        as COUNT;
}
#[inline]
unsafe extern "C" fn sread_32(mut fp: *mut GAME_STATE_FILE, mut v: *mut DWORD) -> COUNT {
    let mut t: DWORD = 0;
    if v.is_null() {
        v = &raw mut t;
    }
    return ReadStateFile(v as *mut ::core::ffi::c_void, 4 as COUNT, 1 as COUNT, fp)
        as COUNT;
}
