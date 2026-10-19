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

// Rust translation of src/uqm/comm/melnorm/melnorm.c.
extern "C" {
    pub type string_table_entry;
    pub type frame_desc;
    pub type font_desc;
    pub type tfb_soundsample;
    pub type RandomContext;
    fn GetTimeCounter() -> TimeCount;
    fn SleepThread(timePeriod: TimePeriod);
    fn __assert_fail(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    );
    fn XFormColorMap(ColorMapPtr: COLORMAPPTR, TimeInterval: SIZE) -> DWORD;
    fn GetColorMapAddress(_: COLORMAP) -> COLORMAPPTR;
    fn SetAbsStringTableIndex(String: STRING, StringTableIndex: COUNT) -> STRING;
    fn GetStringAddress(String: STRING) -> STRINGPTR;
    fn ClearSISRect(ClearFlags: BYTE);
    fn DrawStatusMessage(pStr: *const UNICODE);
    fn SetStatusMessageMode(_: StatMsgMode) -> StatMsgMode;
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
    static mut CommData: LOCDATA;
    static mut shared_phrase_buf: [UNICODE; 2048];
    fn DoResponsePhrase(
        R: RESPONSE_REF,
        response_func: RESPONSE_FUNC,
        ContstructStr: *mut UNICODE,
    );
    fn NPCPhrase_cb(index: ::core::ffi::c_int, cb: CallbackFunction);
    fn NPCNumber(number: ::core::ffi::c_int, fmt: *const ::core::ffi::c_char);
    fn GetAllianceName(buf: *mut UNICODE, name_1: RESPONSE_REF);
    fn construct_response(buf: *mut UNICODE, R: ::core::ffi::c_int, ...);
    fn setSegue(segue: Segue);
    static mut ScreenWidth: ::core::ffi::c_int;
    static mut ScreenHeight: ::core::ffi::c_int;
    fn AlienTalkSegue(wait_track: COUNT);
    fn DrawCargoStrings(OldElement: BYTE, NewElement: BYTE);
    fn RandomContext_New() -> *mut RandomContext;
    fn RandomContext_Delete(context: *mut RandomContext);
    fn RandomContext_Random(context: *mut RandomContext) -> DWORD;
    fn RandomContext_SeedRandom(context: *mut RandomContext, new_seed: DWORD) -> DWORD;
    fn square_root(value: DWORD) -> COUNT;
    fn AnyButtonPress(DetectSpecial: BOOLEAN) -> BOOLEAN;
}
pub type size_t = ::core::ffi::c_ulong;
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
pub type PVOIDFUNC = Option<unsafe extern "C" fn() -> ()>;
pub type TimeCount = DWORD;
pub type TimePeriod = DWORD;
pub type QUEUE_HANDLE = *mut ::core::ffi::c_void;
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
pub type RESOURCE = *const ::core::ffi::c_char;
pub type STRING_TABLE_ENTRY_DESC = string_table_entry;
pub type STRING = *mut STRING_TABLE_ENTRY_DESC;
pub type STRINGPTR = *mut ::core::ffi::c_char;
pub type FRAME_DESC = frame_desc;
pub type FONT_DESC = font_desc;
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct TEXT_VALIGN(pub ::core::ffi::c_uint);
impl TEXT_VALIGN {
    pub const VALIGN_TOP: Self = Self(0);
    pub const VALIGN_MIDDLE: Self = Self(1);
    pub const VALIGN_BOTTOM: Self = Self(2);
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
}
pub type TFB_SoundSample = tfb_soundsample;
pub type MUSIC_REF = *mut *mut TFB_SoundSample;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ANIMATION_DESC {
    pub StartIndex: COUNT,
    pub NumFrames: BYTE,
    pub AnimFlags: BYTE,
    pub BaseFrameRate: COUNT,
    pub RandomFrameRate: COUNT,
    pub BaseRestartRate: COUNT,
    pub RandomRestartRate: COUNT,
    pub BlockMask: DWORD,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SPEECH_DIGITNAME {
    pub Divider: ::core::ffi::c_int,
    pub MaxRemainder: ::core::ffi::c_int,
    pub StrIndex: COUNT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SPEECH_DIGIT {
    pub Divider: ::core::ffi::c_int,
    pub Subtrahend: ::core::ffi::c_int,
    pub StrDigits: *mut COUNT,
    pub Names: *mut SPEECH_DIGITNAME,
    pub CommonNameIndex: COUNT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NUMBER_SPEECH_DESC {
    pub NumDigits: COUNT,
    pub Digits: [SPEECH_DIGIT; 7],
}
pub type NUMBER_SPEECH = *const NUMBER_SPEECH_DESC;
pub type LDAS_FLAGS = DWORD;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LOCDATA {
    pub init_encounter_func: Option<unsafe extern "C" fn() -> ()>,
    pub post_encounter_func: Option<unsafe extern "C" fn() -> ()>,
    pub uninit_encounter_func: Option<unsafe extern "C" fn() -> COUNT>,
    pub AlienFrameRes: RESOURCE,
    pub AlienFontRes: RESOURCE,
    pub AlienTextFColor: Color,
    pub AlienTextBColor: Color,
    pub AlienTextBaseline: POINT,
    pub AlienTextWidth: COUNT,
    pub AlienTextAlign: TEXT_ALIGN,
    pub AlienTextValign: TEXT_VALIGN,
    pub AlienColorMapRes: RESOURCE,
    pub AlienSongRes: RESOURCE,
    pub AlienAltSongRes: RESOURCE,
    pub AlienSongFlags: LDAS_FLAGS,
    pub ConversationPhrasesRes: RESOURCE,
    pub NumAnimations: COUNT,
    pub AlienAmbientArray: [ANIMATION_DESC; 20],
    pub AlienTransitionDesc: ANIMATION_DESC,
    pub AlienTalkDesc: ANIMATION_DESC,
    pub AlienNumberSpeech: NUMBER_SPEECH,
    pub AlienFrame: FRAME,
    pub AlienFont: FONT,
    pub AlienColorMap: COLORMAP,
    pub AlienSong: MUSIC_REF,
    pub ConversationPhrases: STRING,
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
pub type CallbackArg = *mut ::core::ffi::c_void;
pub type CallbackFunction = Option<unsafe extern "C" fn(CallbackArg) -> ()>;
pub type RESPONSE_REF = COUNT;
pub type RESPONSE_FUNC = Option<unsafe extern "C" fn(RESPONSE_REF) -> ()>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Segue(pub ::core::ffi::c_uint);
impl Segue {
    pub const Segue_peace: Self = Self(0);
    pub const Segue_hostile: Self = Self(1);
    pub const Segue_victory: Self = Self(2);
    pub const Segue_defeat: Self = Self(3);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TechSaleData {
    pub techId: TechId_t,
    pub price: ::core::ffi::c_int,
    pub sale_line: ::core::ffi::c_int,
    pub sold_line: ::core::ffi::c_int,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct TechId_t(pub ::core::ffi::c_uint);
impl TechId_t {
    pub const TECH_MODULE_BLASTER: Self = Self(0);
    pub const TECH_LANDER_SPEED: Self = Self(1);
    pub const TECH_MODULE_ANTIMISSILE: Self = Self(2);
    pub const TECH_LANDER_SHIELD_BIO: Self = Self(3);
    pub const TECH_LANDER_CARGO: Self = Self(4);
    pub const TECH_MODULE_BIGFUELTANK: Self = Self(5);
    pub const TECH_LANDER_RAPIDFIRE: Self = Self(6);
    pub const TECH_LANDER_SHIELD_QUAKE: Self = Self(7);
    pub const TECH_MODULE_TRACKING: Self = Self(8);
    pub const TECH_LANDER_SHIELD_LIGHTNING: Self = Self(9);
    pub const TECH_LANDER_SHIELD_HEAT: Self = Self(10);
    pub const TECH_MODULE_CANNON: Self = Self(11);
    pub const TECH_MODULE_FURNACE: Self = Self(12);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TechData {
    pub id: TechId_t,
    pub r#type: TechType_t,
    pub subtype: ::core::ffi::c_int,
    pub info: ::core::ffi::c_int,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct TechType_t(pub ::core::ffi::c_uint);
impl TechType_t {
    pub const TECH_TYPE_MODULE: Self = Self(0);
    pub const TECH_TYPE_LANDER_SHIELD: Self = Self(1);
    pub const TECH_TYPE_STATE: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_2(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_2 {
    pub const NULL_PHRASE: Self = Self(0);
    pub const HELLO_NOW_DOWN_TO_BUSINESS_1: Self = Self(1);
    pub const HELLO_NOW_DOWN_TO_BUSINESS_2: Self = Self(2);
    pub const HELLO_NOW_DOWN_TO_BUSINESS_3: Self = Self(3);
    pub const KNOW_OF_YOU: Self = Self(4);
    pub const how_know: Self = Self(5);
    pub const KNOW_BECAUSE: Self = Self(6);
    pub const what_about_yourselves: Self = Self(7);
    pub const NO_TALK_ABOUT_OURSELVES: Self = Self(8);
    pub const what_factors: Self = Self(9);
    pub const FACTORS_ARE: Self = Self(10);
    pub const what_about_universe: Self = Self(11);
    pub const NO_FREE_LUNCH: Self = Self(12);
    pub const giving_is_good_1: Self = Self(13);
    pub const GIVING_IS_BAD_1: Self = Self(14);
    pub const giving_is_good_2: Self = Self(15);
    pub const GIVING_IS_BAD_2: Self = Self(16);
    pub const get_on_with_business: Self = Self(17);
    pub const trade_is_for_the_weak: Self = Self(18);
    pub const WERE_NOT_AFRAID: Self = Self(19);
    pub const no_trade_now: Self = Self(20);
    pub const OK_NO_TRADE_NOW_BYE: Self = Self(21);
    pub const HELLO_AND_DOWN_TO_BUSINESS_1: Self = Self(22);
    pub const HELLO_AND_DOWN_TO_BUSINESS_2: Self = Self(23);
    pub const HELLO_AND_DOWN_TO_BUSINESS_3: Self = Self(24);
    pub const HELLO_AND_DOWN_TO_BUSINESS_4: Self = Self(25);
    pub const HELLO_AND_DOWN_TO_BUSINESS_5: Self = Self(26);
    pub const HELLO_AND_DOWN_TO_BUSINESS_6: Self = Self(27);
    pub const HELLO_AND_DOWN_TO_BUSINESS_7: Self = Self(28);
    pub const HELLO_AND_DOWN_TO_BUSINESS_8: Self = Self(29);
    pub const HELLO_AND_DOWN_TO_BUSINESS_9: Self = Self(30);
    pub const HELLO_AND_DOWN_TO_BUSINESS_10: Self = Self(31);
    pub const whats_my_credit: Self = Self(32);
    pub const HELLO_SLIGHTLY_ANGRY_1: Self = Self(33);
    pub const HELLO_SLIGHTLY_ANGRY_2: Self = Self(34);
    pub const HELLO_SLIGHTLY_ANGRY_3: Self = Self(35);
    pub const explore_relationship: Self = Self(36);
    pub const EXAMPLE_OF_RELATIONSHIP: Self = Self(37);
    pub const excuse_1: Self = Self(38);
    pub const NO_EXCUSE_1: Self = Self(39);
    pub const excuse_2: Self = Self(40);
    pub const NO_EXCUSE_2: Self = Self(41);
    pub const excuse_3: Self = Self(42);
    pub const NO_EXCUSE_3: Self = Self(43);
    pub const we_apologize: Self = Self(44);
    pub const APOLOGY_ACCEPTED: Self = Self(45);
    pub const so_we_can_attack: Self = Self(46);
    pub const DECEITFUL_HUMAN: Self = Self(47);
    pub const bye_melnorme_slightly_angry: Self = Self(48);
    pub const MELNORME_SLIGHTLY_ANGRY_GOODBYE: Self = Self(49);
    pub const HELLO_HATE_YOU_1: Self = Self(50);
    pub const HELLO_HATE_YOU_2: Self = Self(51);
    pub const HELLO_HATE_YOU_3: Self = Self(52);
    pub const well_if_thats_the_way_you_feel: Self = Self(53);
    pub const you_hate_us_so_we_go_away: Self = Self(54);
    pub const HATE_YOU_GOODBYE: Self = Self(55);
    pub const WE_FIGHT_AGAIN: Self = Self(56);
    pub const RESCUE_EXPLANATION: Self = Self(57);
    pub const RESCUE_AGAIN_1: Self = Self(58);
    pub const RESCUE_AGAIN_2: Self = Self(59);
    pub const RESCUE_AGAIN_3: Self = Self(60);
    pub const RESCUE_AGAIN_4: Self = Self(61);
    pub const RESCUE_AGAIN_5: Self = Self(62);
    pub const CHANGED_MIND: Self = Self(63);
    pub const no_changed_mind: Self = Self(64);
    pub const yes_changed_mind: Self = Self(65);
    pub const SHOULD_WE_HELP_YOU: Self = Self(66);
    pub const yes_help: Self = Self(67);
    pub const no_help: Self = Self(68);
    pub const RESCUE_OFFER: Self = Self(69);
    pub const RESCUE_TANKS: Self = Self(70);
    pub const RESCUE_HOME: Self = Self(71);
    pub const take_it: Self = Self(72);
    pub const leave_it: Self = Self(73);
    pub const HAPPY_TO_HAVE_RESCUED: Self = Self(74);
    pub const MAYBE_SEE_YOU_LATER: Self = Self(75);
    pub const GOODBYE_AND_GOODLUCK: Self = Self(76);
    pub const GOODBYE_AND_GOODLUCK_AGAIN: Self = Self(77);
    pub const HELLO_PISSED_OFF_1: Self = Self(78);
    pub const HELLO_PISSED_OFF_2: Self = Self(79);
    pub const HELLO_PISSED_OFF_3: Self = Self(80);
    pub const beg_forgiveness: Self = Self(81);
    pub const LOTS_TO_MAKE_UP_FOR: Self = Self(82);
    pub const you_are_so_right: Self = Self(83);
    pub const ONE_LAST_CHANCE: Self = Self(84);
    pub const ok_strip_me: Self = Self(85);
    pub const no_strip_now: Self = Self(86);
    pub const NOT_WORTH_STRIPPING: Self = Self(87);
    pub const FAIR_JUSTICE: Self = Self(88);
    pub const bye_melnorme_pissed_off: Self = Self(89);
    pub const MELNORME_PISSED_OFF_GOODBYE: Self = Self(90);
    pub const fight_some_more: Self = Self(91);
    pub const OK_FIGHT_SOME_MORE: Self = Self(92);
    pub const why_blue_light: Self = Self(93);
    pub const BLUE_IS_MAD: Self = Self(94);
    pub const we_strong_1: Self = Self(95);
    pub const YOU_NOT_STRONG_1: Self = Self(96);
    pub const we_strong_2: Self = Self(97);
    pub const YOU_NOT_STRONG_2: Self = Self(98);
    pub const we_strong_3: Self = Self(99);
    pub const YOU_NOT_STRONG_3: Self = Self(100);
    pub const just_testing: Self = Self(101);
    pub const REALLY_TESTING: Self = Self(102);
    pub const yes_really_testing: Self = Self(103);
    pub const TEST_RESULTS: Self = Self(104);
    pub const youre_on: Self = Self(105);
    pub const YOU_GIVE_US_NO_CHOICE: Self = Self(106);
    pub const TRADING_INFO: Self = Self(107);
    pub const BUY_OR_SELL: Self = Self(108);
    pub const goodbye: Self = Self(109);
    pub const why_turned_purple: Self = Self(110);
    pub const buy: Self = Self(111);
    pub const sell: Self = Self(112);
    pub const TURNED_PURPLE_BECAUSE: Self = Self(113);
    pub const NOTHING_TO_SELL: Self = Self(114);
    pub const WHAT_TO_SELL: Self = Self(115);
    pub const OK_DONE_SELLING: Self = Self(116);
    pub const sell_life_data: Self = Self(117);
    pub const SOLD_LIFE_DATA1: Self = Self(118);
    pub const SOLD_LIFE_DATA2: Self = Self(119);
    pub const SOLD_LIFE_DATA3: Self = Self(120);
    pub const sell_rainbow_locations: Self = Self(121);
    pub const SOLD_RAINBOW_LOCATIONS1: Self = Self(122);
    pub const SOLD_RAINBOW_LOCATIONS2: Self = Self(123);
    pub const SOLD_RAINBOW_LOCATIONS3: Self = Self(124);
    pub const sell_precursor_find: Self = Self(125);
    pub const SOLD_PRECURSOR_FIND: Self = Self(126);
    pub const changed_mind_no_sell: Self = Self(127);
    pub const done_selling: Self = Self(128);
    pub const NEED_CREDIT: Self = Self(129);
    pub const WHAT_TO_BUY: Self = Self(130);
    pub const WHAT_MORE_TO_BUY: Self = Self(131);
    pub const OK_DONE_BUYING: Self = Self(132);
    pub const buy_fuel: Self = Self(133);
    pub const done_buying: Self = Self(134);
    pub const be_leaving_now: Self = Self(135);
    pub const HOW_MUCH_FUEL: Self = Self(136);
    pub const buy_1_fuel: Self = Self(137);
    pub const GOT_FUEL: Self = Self(138);
    pub const buy_5_fuel: Self = Self(139);
    pub const buy_10_fuel: Self = Self(140);
    pub const buy_25_fuel: Self = Self(141);
    pub const done_buying_fuel: Self = Self(142);
    pub const FRIENDLY_GOODBYE: Self = Self(143);
    pub const CREDIT_IS0: Self = Self(144);
    pub const CREDIT_IS1: Self = Self(145);
    pub const NEED_MORE_CREDIT0: Self = Self(146);
    pub const NEED_MORE_CREDIT1: Self = Self(147);
    pub const BUY_FUEL_INTRO: Self = Self(148);
    pub const NO_ROOM_FOR_FUEL: Self = Self(149);
    pub const buy_info: Self = Self(150);
    pub const buy_technology: Self = Self(151);
    pub const buy_current_events: Self = Self(152);
    pub const buy_alien_races: Self = Self(153);
    pub const buy_history: Self = Self(154);
    pub const done_buying_info: Self = Self(155);
    pub const no_buy_info: Self = Self(156);
    pub const BUY_INFO_INTRO: Self = Self(157);
    pub const OK_BUY_INFO: Self = Self(158);
    pub const OK_NO_BUY_INFO: Self = Self(159);
    pub const OK_DONE_BUYING_INFO: Self = Self(160);
    pub const OK_BUY_EVENT_1: Self = Self(161);
    pub const OK_BUY_EVENT_2: Self = Self(162);
    pub const OK_BUY_EVENT_3: Self = Self(163);
    pub const OK_BUY_EVENT_4: Self = Self(164);
    pub const OK_BUY_EVENT_5: Self = Self(165);
    pub const OK_BUY_EVENT_6: Self = Self(166);
    pub const OK_BUY_EVENT_7: Self = Self(167);
    pub const OK_BUY_EVENT_8: Self = Self(168);
    pub const OK_BUY_ALIEN_RACE_1: Self = Self(169);
    pub const OK_BUY_ALIEN_RACE_2: Self = Self(170);
    pub const OK_BUY_ALIEN_RACE_3: Self = Self(171);
    pub const OK_BUY_ALIEN_RACE_4: Self = Self(172);
    pub const OK_BUY_ALIEN_RACE_5: Self = Self(173);
    pub const OK_BUY_ALIEN_RACE_6: Self = Self(174);
    pub const OK_BUY_ALIEN_RACE_7: Self = Self(175);
    pub const OK_BUY_ALIEN_RACE_8: Self = Self(176);
    pub const OK_BUY_ALIEN_RACE_9: Self = Self(177);
    pub const OK_BUY_ALIEN_RACE_10: Self = Self(178);
    pub const OK_BUY_ALIEN_RACE_11: Self = Self(179);
    pub const OK_BUY_ALIEN_RACE_12: Self = Self(180);
    pub const OK_BUY_ALIEN_RACE_13: Self = Self(181);
    pub const OK_BUY_ALIEN_RACE_14: Self = Self(182);
    pub const OK_BUY_ALIEN_RACE_15: Self = Self(183);
    pub const OK_BUY_ALIEN_RACE_16: Self = Self(184);
    pub const OK_BUY_HISTORY_1: Self = Self(185);
    pub const OK_BUY_HISTORY_2: Self = Self(186);
    pub const OK_BUY_HISTORY_3: Self = Self(187);
    pub const OK_BUY_HISTORY_4: Self = Self(188);
    pub const OK_BUY_HISTORY_5: Self = Self(189);
    pub const OK_BUY_HISTORY_6: Self = Self(190);
    pub const OK_BUY_HISTORY_7: Self = Self(191);
    pub const OK_BUY_HISTORY_8: Self = Self(192);
    pub const OK_BUY_HISTORY_9: Self = Self(193);
    pub const INFO_ALL_GONE: Self = Self(194);
    pub const buy_new_tech: Self = Self(195);
    pub const no_buy_new_tech: Self = Self(196);
    pub const done_buying_new_tech: Self = Self(197);
    pub const fill_me_up: Self = Self(198);
    pub const OK_FILL_YOU_UP: Self = Self(199);
    pub const BUY_NEW_TECH_INTRO: Self = Self(200);
    pub const OK_BUY_NEW_TECH: Self = Self(201);
    pub const OK_NO_BUY_NEW_TECH: Self = Self(202);
    pub const OK_DONE_BUYING_NEW_TECH: Self = Self(203);
    pub const OK_DONE_BUYING_FUEL: Self = Self(204);
    pub const NEW_TECH_1: Self = Self(205);
    pub const NEW_TECH_2: Self = Self(206);
    pub const NEW_TECH_3: Self = Self(207);
    pub const NEW_TECH_4: Self = Self(208);
    pub const NEW_TECH_5: Self = Self(209);
    pub const NEW_TECH_6: Self = Self(210);
    pub const NEW_TECH_7: Self = Self(211);
    pub const NEW_TECH_8: Self = Self(212);
    pub const NEW_TECH_9: Self = Self(213);
    pub const NEW_TECH_10: Self = Self(214);
    pub const NEW_TECH_11: Self = Self(215);
    pub const NEW_TECH_12: Self = Self(216);
    pub const NEW_TECH_13: Self = Self(217);
    pub const OK_BUY_NEW_TECH_1: Self = Self(218);
    pub const OK_BUY_NEW_TECH_2: Self = Self(219);
    pub const OK_BUY_NEW_TECH_3: Self = Self(220);
    pub const OK_BUY_NEW_TECH_4: Self = Self(221);
    pub const OK_BUY_NEW_TECH_5: Self = Self(222);
    pub const OK_BUY_NEW_TECH_6: Self = Self(223);
    pub const OK_BUY_NEW_TECH_7: Self = Self(224);
    pub const OK_BUY_NEW_TECH_8: Self = Self(225);
    pub const OK_BUY_NEW_TECH_9: Self = Self(226);
    pub const OK_BUY_NEW_TECH_10: Self = Self(227);
    pub const OK_BUY_NEW_TECH_11: Self = Self(228);
    pub const OK_BUY_NEW_TECH_12: Self = Self(229);
    pub const OK_BUY_NEW_TECH_13: Self = Self(230);
    pub const CHARITY: Self = Self(231);
    pub const NEW_TECH_ALL_GONE: Self = Self(232);
    pub const we_are_from_alliance0: Self = Self(233);
    pub const STRIP_HEAD: Self = Self(234);
    pub const LANDERS: Self = Self(235);
    pub const THRUSTERS: Self = Self(236);
    pub const JETS: Self = Self(237);
    pub const PODS: Self = Self(238);
    pub const BAYS: Self = Self(239);
    pub const DYNAMOS: Self = Self(240);
    pub const FURNACES: Self = Self(241);
    pub const GUNS: Self = Self(242);
    pub const BLASTERS: Self = Self(243);
    pub const CANNONS: Self = Self(244);
    pub const TRACKERS: Self = Self(245);
    pub const DEFENSES: Self = Self(246);
    pub const name_1: Self = Self(247);
    pub const name_2: Self = Self(248);
    pub const name_3: Self = Self(249);
    pub const name_40: Self = Self(250);
    pub const name_41: Self = Self(251);
    pub const ENUMERATE_ONE: Self = Self(252);
    pub const ENUMERATE_TWO: Self = Self(253);
    pub const ENUMERATE_THREE: Self = Self(254);
    pub const ENUMERATE_FOUR: Self = Self(255);
    pub const ENUMERATE_FIVE: Self = Self(256);
    pub const ENUMERATE_SIX: Self = Self(257);
    pub const ENUMERATE_SEVEN: Self = Self(258);
    pub const ENUMERATE_EIGHT: Self = Self(259);
    pub const ENUMERATE_NINE: Self = Self(260);
    pub const ENUMERATE_TEN: Self = Self(261);
    pub const ENUMERATE_ELEVEN: Self = Self(262);
    pub const ENUMERATE_TWELVE: Self = Self(263);
    pub const ENUMERATE_THIRTEEN: Self = Self(264);
    pub const ENUMERATE_FOURTEEN: Self = Self(265);
    pub const ENUMERATE_FIFTEEN: Self = Self(266);
    pub const ENUMERATE_SIXTEEN: Self = Self(267);
    pub const END_LIST_WITH_AND: Self = Self(268);
    pub const ENUMERATE_ZERO: Self = Self(269);
    pub const ENUMERATE_SEVENTEEN: Self = Self(270);
    pub const ENUMERATE_EIGHTEEN: Self = Self(271);
    pub const ENUMERATE_NINETEEN: Self = Self(272);
    pub const ENUMERATE_TWENTY: Self = Self(273);
    pub const ENUMERATE_THIRTY: Self = Self(274);
    pub const ENUMERATE_FOURTY: Self = Self(275);
    pub const ENUMERATE_FIFTY: Self = Self(276);
    pub const ENUMERATE_SIXTY: Self = Self(277);
    pub const ENUMERATE_SEVENTY: Self = Self(278);
    pub const ENUMERATE_EIGHTY: Self = Self(279);
    pub const ENUMERATE_NINETY: Self = Self(280);
    pub const ENUMERATE_HUNDRED: Self = Self(281);
    pub const ENUMERATE_THOUSAND: Self = Self(282);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_3(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_3 {
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
static mut melnorme_desc: LOCDATA = unsafe {
    LOCDATA {
        init_encounter_func: None,
        post_encounter_func: None,
        uninit_encounter_func: None,
        AlienFrameRes: b"comm.melnorme.graphics\0".as_ptr() as RESOURCE,
        AlienFontRes: b"comm.melnorme.font\0".as_ptr() as RESOURCE,
        AlienTextFColor: Color {
            r: ((0x1f as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x1f as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0x1f as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x1f as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0x1f as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x1f as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
        AlienTextBColor: Color {
            r: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
        AlienTextBaseline: point {
            x: 0 as COORD,
            y: 0 as COORD,
        },
        AlienTextWidth: 0 as COUNT,
        AlienTextAlign: TEXT_ALIGN::ALIGN_CENTER,
        AlienTextValign: TEXT_VALIGN::VALIGN_TOP,
        AlienColorMapRes: b"comm.melnorme.colortable\0".as_ptr() as RESOURCE,
        AlienSongRes: b"comm.melnorme.music\0".as_ptr() as RESOURCE,
        AlienAltSongRes: ::core::ptr::null::<::core::ffi::c_char>(),
        AlienSongFlags: 0 as LDAS_FLAGS,
        ConversationPhrasesRes: b"comm.melnorme.dialogue\0".as_ptr() as RESOURCE,
        NumAnimations: 4 as COUNT,
        AlienAmbientArray: [
            ANIMATION_DESC {
                StartIndex: 6 as COUNT,
                NumFrames: 5 as BYTE,
                AnimFlags: ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                    as BYTE,
                BaseFrameRate: (840 as ::core::ffi::c_int / 12 as ::core::ffi::c_int)
                    as COUNT,
                RandomFrameRate: 0 as COUNT,
                BaseRestartRate: (840 as ::core::ffi::c_int * 4 as ::core::ffi::c_int)
                    as COUNT,
                RandomRestartRate: (840 as ::core::ffi::c_int * 4 as ::core::ffi::c_int)
                    as COUNT,
                BlockMask: ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
                    as DWORD,
            },
            ANIMATION_DESC {
                StartIndex: 11 as COUNT,
                NumFrames: 9 as BYTE,
                AnimFlags: ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                    as BYTE,
                BaseFrameRate: (840 as ::core::ffi::c_int / 20 as ::core::ffi::c_int)
                    as COUNT,
                RandomFrameRate: 0 as COUNT,
                BaseRestartRate: (840 as ::core::ffi::c_int * 4 as ::core::ffi::c_int)
                    as COUNT,
                RandomRestartRate: (840 as ::core::ffi::c_int * 4 as ::core::ffi::c_int)
                    as COUNT,
                BlockMask: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int)
                    as DWORD,
            },
            ANIMATION_DESC {
                StartIndex: 20 as COUNT,
                NumFrames: 2 as BYTE,
                AnimFlags: ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                    as BYTE,
                BaseFrameRate: (840 as ::core::ffi::c_int / 10 as ::core::ffi::c_int)
                    as COUNT,
                RandomFrameRate: (840 as ::core::ffi::c_int / 15 as ::core::ffi::c_int)
                    as COUNT,
                BaseRestartRate: 840 as COUNT,
                RandomRestartRate: (840 as ::core::ffi::c_int * 3 as ::core::ffi::c_int)
                    as COUNT,
                BlockMask: 0 as DWORD,
            },
            ANIMATION_DESC {
                StartIndex: 22 as COUNT,
                NumFrames: 2 as BYTE,
                AnimFlags: ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                    as BYTE,
                BaseFrameRate: (840 as ::core::ffi::c_int / 10 as ::core::ffi::c_int)
                    as COUNT,
                RandomFrameRate: (840 as ::core::ffi::c_int / 15 as ::core::ffi::c_int)
                    as COUNT,
                BaseRestartRate: 840 as COUNT,
                RandomRestartRate: (840 as ::core::ffi::c_int * 3 as ::core::ffi::c_int)
                    as COUNT,
                BlockMask: 0 as DWORD,
            },
            ANIMATION_DESC {
                StartIndex: 0,
                NumFrames: 0,
                AnimFlags: 0,
                BaseFrameRate: 0,
                RandomFrameRate: 0,
                BaseRestartRate: 0,
                RandomRestartRate: 0,
                BlockMask: 0,
            },
            ANIMATION_DESC {
                StartIndex: 0,
                NumFrames: 0,
                AnimFlags: 0,
                BaseFrameRate: 0,
                RandomFrameRate: 0,
                BaseRestartRate: 0,
                RandomRestartRate: 0,
                BlockMask: 0,
            },
            ANIMATION_DESC {
                StartIndex: 0,
                NumFrames: 0,
                AnimFlags: 0,
                BaseFrameRate: 0,
                RandomFrameRate: 0,
                BaseRestartRate: 0,
                RandomRestartRate: 0,
                BlockMask: 0,
            },
            ANIMATION_DESC {
                StartIndex: 0,
                NumFrames: 0,
                AnimFlags: 0,
                BaseFrameRate: 0,
                RandomFrameRate: 0,
                BaseRestartRate: 0,
                RandomRestartRate: 0,
                BlockMask: 0,
            },
            ANIMATION_DESC {
                StartIndex: 0,
                NumFrames: 0,
                AnimFlags: 0,
                BaseFrameRate: 0,
                RandomFrameRate: 0,
                BaseRestartRate: 0,
                RandomRestartRate: 0,
                BlockMask: 0,
            },
            ANIMATION_DESC {
                StartIndex: 0,
                NumFrames: 0,
                AnimFlags: 0,
                BaseFrameRate: 0,
                RandomFrameRate: 0,
                BaseRestartRate: 0,
                RandomRestartRate: 0,
                BlockMask: 0,
            },
            ANIMATION_DESC {
                StartIndex: 0,
                NumFrames: 0,
                AnimFlags: 0,
                BaseFrameRate: 0,
                RandomFrameRate: 0,
                BaseRestartRate: 0,
                RandomRestartRate: 0,
                BlockMask: 0,
            },
            ANIMATION_DESC {
                StartIndex: 0,
                NumFrames: 0,
                AnimFlags: 0,
                BaseFrameRate: 0,
                RandomFrameRate: 0,
                BaseRestartRate: 0,
                RandomRestartRate: 0,
                BlockMask: 0,
            },
            ANIMATION_DESC {
                StartIndex: 0,
                NumFrames: 0,
                AnimFlags: 0,
                BaseFrameRate: 0,
                RandomFrameRate: 0,
                BaseRestartRate: 0,
                RandomRestartRate: 0,
                BlockMask: 0,
            },
            ANIMATION_DESC {
                StartIndex: 0,
                NumFrames: 0,
                AnimFlags: 0,
                BaseFrameRate: 0,
                RandomFrameRate: 0,
                BaseRestartRate: 0,
                RandomRestartRate: 0,
                BlockMask: 0,
            },
            ANIMATION_DESC {
                StartIndex: 0,
                NumFrames: 0,
                AnimFlags: 0,
                BaseFrameRate: 0,
                RandomFrameRate: 0,
                BaseRestartRate: 0,
                RandomRestartRate: 0,
                BlockMask: 0,
            },
            ANIMATION_DESC {
                StartIndex: 0,
                NumFrames: 0,
                AnimFlags: 0,
                BaseFrameRate: 0,
                RandomFrameRate: 0,
                BaseRestartRate: 0,
                RandomRestartRate: 0,
                BlockMask: 0,
            },
            ANIMATION_DESC {
                StartIndex: 0,
                NumFrames: 0,
                AnimFlags: 0,
                BaseFrameRate: 0,
                RandomFrameRate: 0,
                BaseRestartRate: 0,
                RandomRestartRate: 0,
                BlockMask: 0,
            },
            ANIMATION_DESC {
                StartIndex: 0,
                NumFrames: 0,
                AnimFlags: 0,
                BaseFrameRate: 0,
                RandomFrameRate: 0,
                BaseRestartRate: 0,
                RandomRestartRate: 0,
                BlockMask: 0,
            },
            ANIMATION_DESC {
                StartIndex: 0,
                NumFrames: 0,
                AnimFlags: 0,
                BaseFrameRate: 0,
                RandomFrameRate: 0,
                BaseRestartRate: 0,
                RandomRestartRate: 0,
                BlockMask: 0,
            },
            ANIMATION_DESC {
                StartIndex: 0,
                NumFrames: 0,
                AnimFlags: 0,
                BaseFrameRate: 0,
                RandomFrameRate: 0,
                BaseRestartRate: 0,
                RandomRestartRate: 0,
                BlockMask: 0,
            },
        ],
        AlienTransitionDesc: ANIMATION_DESC {
            StartIndex: 0 as COUNT,
            NumFrames: 0 as BYTE,
            AnimFlags: 0 as BYTE,
            BaseFrameRate: 0 as COUNT,
            RandomFrameRate: 0 as COUNT,
            BaseRestartRate: 0 as COUNT,
            RandomRestartRate: 0 as COUNT,
            BlockMask: 0 as DWORD,
        },
        AlienTalkDesc: ANIMATION_DESC {
            StartIndex: 1 as COUNT,
            NumFrames: 5 as BYTE,
            AnimFlags: 0 as BYTE,
            BaseFrameRate: (840 as ::core::ffi::c_int / 15 as ::core::ffi::c_int)
                as COUNT,
            RandomFrameRate: 0 as COUNT,
            BaseRestartRate: (840 as ::core::ffi::c_int / 12 as ::core::ffi::c_int)
                as COUNT,
            RandomRestartRate: 0 as COUNT,
            BlockMask: 0 as DWORD,
        },
        AlienNumberSpeech: &raw const melnorme_numbers_english,
        AlienFrame: ::core::ptr::null_mut::<FRAME_DESC>(),
        AlienFont: ::core::ptr::null_mut::<FONT_DESC>(),
        AlienColorMap: ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>(),
        AlienSong: ::core::ptr::null_mut::<*mut TFB_SoundSample>(),
        ConversationPhrases: ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>(),
    }
};
static mut melnorme_digit_names: [COUNT; 10] = [
    C2Rust_Unnamed_2::ENUMERATE_ZERO.0 as ::core::ffi::c_int as COUNT,
    C2Rust_Unnamed_2::ENUMERATE_ONE.0 as ::core::ffi::c_int as COUNT,
    C2Rust_Unnamed_2::ENUMERATE_TWO.0 as ::core::ffi::c_int as COUNT,
    C2Rust_Unnamed_2::ENUMERATE_THREE.0 as ::core::ffi::c_int as COUNT,
    C2Rust_Unnamed_2::ENUMERATE_FOUR.0 as ::core::ffi::c_int as COUNT,
    C2Rust_Unnamed_2::ENUMERATE_FIVE.0 as ::core::ffi::c_int as COUNT,
    C2Rust_Unnamed_2::ENUMERATE_SIX.0 as ::core::ffi::c_int as COUNT,
    C2Rust_Unnamed_2::ENUMERATE_SEVEN.0 as ::core::ffi::c_int as COUNT,
    C2Rust_Unnamed_2::ENUMERATE_EIGHT.0 as ::core::ffi::c_int as COUNT,
    C2Rust_Unnamed_2::ENUMERATE_NINE.0 as ::core::ffi::c_int as COUNT,
];
static mut melnorme_teen_names: [COUNT; 10] = [
    C2Rust_Unnamed_2::ENUMERATE_TEN.0 as ::core::ffi::c_int as COUNT,
    C2Rust_Unnamed_2::ENUMERATE_ELEVEN.0 as ::core::ffi::c_int as COUNT,
    C2Rust_Unnamed_2::ENUMERATE_TWELVE.0 as ::core::ffi::c_int as COUNT,
    C2Rust_Unnamed_2::ENUMERATE_THIRTEEN.0 as ::core::ffi::c_int as COUNT,
    C2Rust_Unnamed_2::ENUMERATE_FOURTEEN.0 as ::core::ffi::c_int as COUNT,
    C2Rust_Unnamed_2::ENUMERATE_FIFTEEN.0 as ::core::ffi::c_int as COUNT,
    C2Rust_Unnamed_2::ENUMERATE_SIXTEEN.0 as ::core::ffi::c_int as COUNT,
    C2Rust_Unnamed_2::ENUMERATE_SEVENTEEN.0 as ::core::ffi::c_int as COUNT,
    C2Rust_Unnamed_2::ENUMERATE_EIGHTEEN.0 as ::core::ffi::c_int as COUNT,
    C2Rust_Unnamed_2::ENUMERATE_NINETEEN.0 as ::core::ffi::c_int as COUNT,
];
static mut melnorme_tens_names: [COUNT; 10] = [
    0 as COUNT,
    0 as COUNT,
    C2Rust_Unnamed_2::ENUMERATE_TWENTY.0 as ::core::ffi::c_int as COUNT,
    C2Rust_Unnamed_2::ENUMERATE_THIRTY.0 as ::core::ffi::c_int as COUNT,
    C2Rust_Unnamed_2::ENUMERATE_FOURTY.0 as ::core::ffi::c_int as COUNT,
    C2Rust_Unnamed_2::ENUMERATE_FIFTY.0 as ::core::ffi::c_int as COUNT,
    C2Rust_Unnamed_2::ENUMERATE_SIXTY.0 as ::core::ffi::c_int as COUNT,
    C2Rust_Unnamed_2::ENUMERATE_SEVENTY.0 as ::core::ffi::c_int as COUNT,
    C2Rust_Unnamed_2::ENUMERATE_EIGHTY.0 as ::core::ffi::c_int as COUNT,
    C2Rust_Unnamed_2::ENUMERATE_NINETY.0 as ::core::ffi::c_int as COUNT,
];
static mut melnorme_numbers_english: NUMBER_SPEECH_DESC = unsafe {
    NUMBER_SPEECH_DESC {
        NumDigits: 5 as COUNT,
        Digits: [
            SPEECH_DIGIT {
                Divider: 1000 as ::core::ffi::c_int,
                Subtrahend: 0 as ::core::ffi::c_int,
                StrDigits: ::core::ptr::null_mut::<COUNT>(),
                Names: ::core::ptr::null_mut::<SPEECH_DIGITNAME>(),
                CommonNameIndex: C2Rust_Unnamed_2::ENUMERATE_THOUSAND.0
                    as ::core::ffi::c_int as COUNT,
            },
            SPEECH_DIGIT {
                Divider: 100 as ::core::ffi::c_int,
                Subtrahend: 0 as ::core::ffi::c_int,
                StrDigits: &raw const melnorme_digit_names as *mut COUNT,
                Names: ::core::ptr::null_mut::<SPEECH_DIGITNAME>(),
                CommonNameIndex: C2Rust_Unnamed_2::ENUMERATE_HUNDRED.0
                    as ::core::ffi::c_int as COUNT,
            },
            SPEECH_DIGIT {
                Divider: 10 as ::core::ffi::c_int,
                Subtrahend: 0 as ::core::ffi::c_int,
                StrDigits: &raw const melnorme_tens_names as *mut COUNT,
                Names: ::core::ptr::null_mut::<SPEECH_DIGITNAME>(),
                CommonNameIndex: 0 as COUNT,
            },
            SPEECH_DIGIT {
                Divider: 1 as ::core::ffi::c_int,
                Subtrahend: 10 as ::core::ffi::c_int,
                StrDigits: &raw const melnorme_teen_names as *mut COUNT,
                Names: ::core::ptr::null_mut::<SPEECH_DIGITNAME>(),
                CommonNameIndex: 0 as COUNT,
            },
            SPEECH_DIGIT {
                Divider: 1 as ::core::ffi::c_int,
                Subtrahend: 0 as ::core::ffi::c_int,
                StrDigits: &raw const melnorme_digit_names as *mut COUNT,
                Names: ::core::ptr::null_mut::<SPEECH_DIGITNAME>(),
                CommonNameIndex: 0 as COUNT,
            },
            SPEECH_DIGIT {
                Divider: 0,
                Subtrahend: 0,
                StrDigits: ::core::ptr::null_mut::<COUNT>(),
                Names: ::core::ptr::null_mut::<SPEECH_DIGITNAME>(),
                CommonNameIndex: 0,
            },
            SPEECH_DIGIT {
                Divider: 0,
                Subtrahend: 0,
                StrDigits: ::core::ptr::null_mut::<COUNT>(),
                Names: ::core::ptr::null_mut::<SPEECH_DIGITNAME>(),
                CommonNameIndex: 0,
            },
        ],
    }
};
static mut tech_data_table: [TechData; 13] = [
    TechData {
        id: TechId_t::TECH_MODULE_BLASTER,
        r#type: TechType_t::TECH_TYPE_MODULE,
        subtype: C2Rust_Unnamed_0::BLASTER_WEAPON.0 as ::core::ffi::c_int,
        info: 4000 as ::core::ffi::c_int,
    },
    TechData {
        id: TechId_t::TECH_LANDER_SPEED,
        r#type: TechType_t::TECH_TYPE_STATE,
        subtype: C2Rust_Unnamed_1::IMPROVED_LANDER_SPEED.0 as ::core::ffi::c_int,
        info: 1 as ::core::ffi::c_int,
    },
    TechData {
        id: TechId_t::TECH_MODULE_ANTIMISSILE,
        r#type: TechType_t::TECH_TYPE_MODULE,
        subtype: C2Rust_Unnamed_0::ANTIMISSILE_DEFENSE.0 as ::core::ffi::c_int,
        info: 4000 as ::core::ffi::c_int,
    },
    TechData {
        id: TechId_t::TECH_LANDER_SHIELD_BIO,
        r#type: TechType_t::TECH_TYPE_LANDER_SHIELD,
        subtype: C2Rust_Unnamed_3::BIOLOGICAL_DISASTER.0 as ::core::ffi::c_int,
        info: -1 as ::core::ffi::c_int,
    },
    TechData {
        id: TechId_t::TECH_LANDER_CARGO,
        r#type: TechType_t::TECH_TYPE_STATE,
        subtype: C2Rust_Unnamed_1::IMPROVED_LANDER_CARGO.0 as ::core::ffi::c_int,
        info: 1 as ::core::ffi::c_int,
    },
    TechData {
        id: TechId_t::TECH_MODULE_BIGFUELTANK,
        r#type: TechType_t::TECH_TYPE_MODULE,
        subtype: C2Rust_Unnamed_0::HIGHEFF_FUELSYS.0 as ::core::ffi::c_int,
        info: 1000 as ::core::ffi::c_int,
    },
    TechData {
        id: TechId_t::TECH_LANDER_RAPIDFIRE,
        r#type: TechType_t::TECH_TYPE_STATE,
        subtype: C2Rust_Unnamed_1::IMPROVED_LANDER_SHOT.0 as ::core::ffi::c_int,
        info: 1 as ::core::ffi::c_int,
    },
    TechData {
        id: TechId_t::TECH_LANDER_SHIELD_QUAKE,
        r#type: TechType_t::TECH_TYPE_LANDER_SHIELD,
        subtype: C2Rust_Unnamed_3::EARTHQUAKE_DISASTER.0 as ::core::ffi::c_int,
        info: -1 as ::core::ffi::c_int,
    },
    TechData {
        id: TechId_t::TECH_MODULE_TRACKING,
        r#type: TechType_t::TECH_TYPE_MODULE,
        subtype: C2Rust_Unnamed_0::TRACKING_SYSTEM.0 as ::core::ffi::c_int,
        info: 5000 as ::core::ffi::c_int,
    },
    TechData {
        id: TechId_t::TECH_LANDER_SHIELD_LIGHTNING,
        r#type: TechType_t::TECH_TYPE_LANDER_SHIELD,
        subtype: C2Rust_Unnamed_3::LIGHTNING_DISASTER.0 as ::core::ffi::c_int,
        info: -1 as ::core::ffi::c_int,
    },
    TechData {
        id: TechId_t::TECH_LANDER_SHIELD_HEAT,
        r#type: TechType_t::TECH_TYPE_LANDER_SHIELD,
        subtype: C2Rust_Unnamed_3::LAVASPOT_DISASTER.0 as ::core::ffi::c_int,
        info: -1 as ::core::ffi::c_int,
    },
    TechData {
        id: TechId_t::TECH_MODULE_CANNON,
        r#type: TechType_t::TECH_TYPE_MODULE,
        subtype: C2Rust_Unnamed_0::CANNON_WEAPON.0 as ::core::ffi::c_int,
        info: 6000 as ::core::ffi::c_int,
    },
    TechData {
        id: TechId_t::TECH_MODULE_FURNACE,
        r#type: TechType_t::TECH_TYPE_MODULE,
        subtype: C2Rust_Unnamed_0::SHIVA_FURNACE.0 as ::core::ffi::c_int,
        info: 4000 as ::core::ffi::c_int,
    },
];
#[no_mangle]
pub static mut NUM_TECHNOLOGIES: size_t = 0;
unsafe extern "C" fn GetTechData(mut techId: TechId_t) -> *const TechData {
    let mut i: size_t = 0 as size_t;
    i = 0 as size_t;
    while i < NUM_TECHNOLOGIES {
        if tech_data_table[i as usize].id.0 == techId.0 {
            return (&raw const tech_data_table as *const TechData).offset(i as isize);
        }
        i = i.wrapping_add(1);
    }
    return ::core::ptr::null::<TechData>();
}
unsafe extern "C" fn HasStateTech(mut stateId: ::core::ffi::c_int) -> bool {
    match stateId {
        413 => {
            return getGameState(
                &raw mut GlobData.Game_state.GameState as *mut BYTE,
                C2Rust_Unnamed_1::IMPROVED_LANDER_SPEED.0 as ::core::ffi::c_int,
                C2Rust_Unnamed_1::END_IMPROVED_LANDER_SPEED.0 as ::core::ffi::c_int,
            ) != 0;
        }
        414 => {
            return getGameState(
                &raw mut GlobData.Game_state.GameState as *mut BYTE,
                C2Rust_Unnamed_1::IMPROVED_LANDER_CARGO.0 as ::core::ffi::c_int,
                C2Rust_Unnamed_1::END_IMPROVED_LANDER_CARGO.0 as ::core::ffi::c_int,
            ) != 0;
        }
        415 => {
            return getGameState(
                &raw mut GlobData.Game_state.GameState as *mut BYTE,
                C2Rust_Unnamed_1::IMPROVED_LANDER_SHOT.0 as ::core::ffi::c_int,
                C2Rust_Unnamed_1::END_IMPROVED_LANDER_SHOT.0 as ::core::ffi::c_int,
            ) != 0;
        }
        _ => {}
    }
    return false;
}
unsafe extern "C" fn GrantStateTech(mut stateId: ::core::ffi::c_int, mut value: BYTE) {
    match stateId {
        413 => {
            setGameState(
                &raw mut GlobData.Game_state.GameState as *mut BYTE,
                C2Rust_Unnamed_1::IMPROVED_LANDER_SPEED.0 as ::core::ffi::c_int,
                C2Rust_Unnamed_1::END_IMPROVED_LANDER_SPEED.0 as ::core::ffi::c_int,
                value,
            );
            return;
        }
        414 => {
            setGameState(
                &raw mut GlobData.Game_state.GameState as *mut BYTE,
                C2Rust_Unnamed_1::IMPROVED_LANDER_CARGO.0 as ::core::ffi::c_int,
                C2Rust_Unnamed_1::END_IMPROVED_LANDER_CARGO.0 as ::core::ffi::c_int,
                value,
            );
            return;
        }
        415 => {
            setGameState(
                &raw mut GlobData.Game_state.GameState as *mut BYTE,
                C2Rust_Unnamed_1::IMPROVED_LANDER_SHOT.0 as ::core::ffi::c_int,
                C2Rust_Unnamed_1::END_IMPROVED_LANDER_SHOT.0 as ::core::ffi::c_int,
                value,
            );
            return;
        }
        _ => {}
    };
}
unsafe extern "C" fn HasTech(mut techId: TechId_t) -> bool {
    let mut techData: *const TechData = GetTechData(techId);
    if techData.is_null() {
        return false;
    }
    match (*techData).r#type {
        TechType_t::TECH_TYPE_MODULE => {
            return GlobData.Game_state.ModuleCost[(*techData).subtype as usize]
                as ::core::ffi::c_int != 0 as ::core::ffi::c_int;
        }
        TechType_t::TECH_TYPE_LANDER_SHIELD => {
            return getGameState(
                &raw mut GlobData.Game_state.GameState as *mut BYTE,
                C2Rust_Unnamed_1::LANDER_SHIELDS.0 as ::core::ffi::c_int,
                C2Rust_Unnamed_1::END_LANDER_SHIELDS.0 as ::core::ffi::c_int,
            ) as ::core::ffi::c_int & (1 as ::core::ffi::c_int) << (*techData).subtype
                != 0 as ::core::ffi::c_int;
        }
        TechType_t::TECH_TYPE_STATE => return HasStateTech((*techData).subtype),
        _ => {}
    }
    return false;
}
unsafe extern "C" fn GrantTech(mut techId: TechId_t) {
    let mut techData: *const TechData = GetTechData(techId);
    if techData.is_null() {
        return;
    }
    match (*techData).r#type {
        TechType_t::TECH_TYPE_MODULE => {
            GlobData.Game_state.ModuleCost[(*techData).subtype as usize] = ((*techData)
                .info / 50 as ::core::ffi::c_int) as BYTE;
            return;
        }
        TechType_t::TECH_TYPE_LANDER_SHIELD => {
            let mut state: COUNT = (getGameState(
                &raw mut GlobData.Game_state.GameState as *mut BYTE,
                C2Rust_Unnamed_1::LANDER_SHIELDS.0 as ::core::ffi::c_int,
                C2Rust_Unnamed_1::END_LANDER_SHIELDS.0 as ::core::ffi::c_int,
            ) as ::core::ffi::c_int | (1 as ::core::ffi::c_int) << (*techData).subtype)
                as COUNT;
            setGameState(
                &raw mut GlobData.Game_state.GameState as *mut BYTE,
                C2Rust_Unnamed_1::LANDER_SHIELDS.0 as ::core::ffi::c_int,
                C2Rust_Unnamed_1::END_LANDER_SHIELDS.0 as ::core::ffi::c_int,
                state as BYTE,
            );
            return;
        }
        TechType_t::TECH_TYPE_STATE => {
            GrantStateTech((*techData).subtype, (*techData).info as BYTE);
            return;
        }
        _ => {}
    };
}
static mut ok_buy_event_lines: [::core::ffi::c_int; 8] = [
    C2Rust_Unnamed_2::OK_BUY_EVENT_1.0 as ::core::ffi::c_int,
    C2Rust_Unnamed_2::OK_BUY_EVENT_2.0 as ::core::ffi::c_int,
    C2Rust_Unnamed_2::OK_BUY_EVENT_3.0 as ::core::ffi::c_int,
    C2Rust_Unnamed_2::OK_BUY_EVENT_4.0 as ::core::ffi::c_int,
    C2Rust_Unnamed_2::OK_BUY_EVENT_5.0 as ::core::ffi::c_int,
    C2Rust_Unnamed_2::OK_BUY_EVENT_6.0 as ::core::ffi::c_int,
    C2Rust_Unnamed_2::OK_BUY_EVENT_7.0 as ::core::ffi::c_int,
    C2Rust_Unnamed_2::OK_BUY_EVENT_8.0 as ::core::ffi::c_int,
];
#[no_mangle]
pub static mut NUM_EVENT_ITEMS: size_t = 0;
static mut ok_buy_alien_race_lines: [::core::ffi::c_int; 16] = [
    C2Rust_Unnamed_2::OK_BUY_ALIEN_RACE_1.0 as ::core::ffi::c_int,
    C2Rust_Unnamed_2::OK_BUY_ALIEN_RACE_2.0 as ::core::ffi::c_int,
    C2Rust_Unnamed_2::OK_BUY_ALIEN_RACE_3.0 as ::core::ffi::c_int,
    C2Rust_Unnamed_2::OK_BUY_ALIEN_RACE_4.0 as ::core::ffi::c_int,
    C2Rust_Unnamed_2::OK_BUY_ALIEN_RACE_5.0 as ::core::ffi::c_int,
    C2Rust_Unnamed_2::OK_BUY_ALIEN_RACE_6.0 as ::core::ffi::c_int,
    C2Rust_Unnamed_2::OK_BUY_ALIEN_RACE_7.0 as ::core::ffi::c_int,
    C2Rust_Unnamed_2::OK_BUY_ALIEN_RACE_8.0 as ::core::ffi::c_int,
    C2Rust_Unnamed_2::OK_BUY_ALIEN_RACE_9.0 as ::core::ffi::c_int,
    C2Rust_Unnamed_2::OK_BUY_ALIEN_RACE_10.0 as ::core::ffi::c_int,
    C2Rust_Unnamed_2::OK_BUY_ALIEN_RACE_11.0 as ::core::ffi::c_int,
    C2Rust_Unnamed_2::OK_BUY_ALIEN_RACE_12.0 as ::core::ffi::c_int,
    C2Rust_Unnamed_2::OK_BUY_ALIEN_RACE_13.0 as ::core::ffi::c_int,
    C2Rust_Unnamed_2::OK_BUY_ALIEN_RACE_14.0 as ::core::ffi::c_int,
    C2Rust_Unnamed_2::OK_BUY_ALIEN_RACE_15.0 as ::core::ffi::c_int,
    C2Rust_Unnamed_2::OK_BUY_ALIEN_RACE_16.0 as ::core::ffi::c_int,
];
#[no_mangle]
pub static mut NUM_ALIEN_RACE_ITEMS: size_t = 0;
static mut ok_buy_history_lines: [::core::ffi::c_int; 9] = [
    C2Rust_Unnamed_2::OK_BUY_HISTORY_1.0 as ::core::ffi::c_int,
    C2Rust_Unnamed_2::OK_BUY_HISTORY_2.0 as ::core::ffi::c_int,
    C2Rust_Unnamed_2::OK_BUY_HISTORY_3.0 as ::core::ffi::c_int,
    C2Rust_Unnamed_2::OK_BUY_HISTORY_4.0 as ::core::ffi::c_int,
    C2Rust_Unnamed_2::OK_BUY_HISTORY_5.0 as ::core::ffi::c_int,
    C2Rust_Unnamed_2::OK_BUY_HISTORY_6.0 as ::core::ffi::c_int,
    C2Rust_Unnamed_2::OK_BUY_HISTORY_7.0 as ::core::ffi::c_int,
    C2Rust_Unnamed_2::OK_BUY_HISTORY_8.0 as ::core::ffi::c_int,
    C2Rust_Unnamed_2::OK_BUY_HISTORY_9.0 as ::core::ffi::c_int,
];
#[no_mangle]
pub static mut NUM_HISTORY_ITEMS: size_t = 0;
static mut hello_and_down_to_business_lines: [::core::ffi::c_int; 10] = [
    C2Rust_Unnamed_2::HELLO_AND_DOWN_TO_BUSINESS_1.0 as ::core::ffi::c_int,
    C2Rust_Unnamed_2::HELLO_AND_DOWN_TO_BUSINESS_2.0 as ::core::ffi::c_int,
    C2Rust_Unnamed_2::HELLO_AND_DOWN_TO_BUSINESS_3.0 as ::core::ffi::c_int,
    C2Rust_Unnamed_2::HELLO_AND_DOWN_TO_BUSINESS_4.0 as ::core::ffi::c_int,
    C2Rust_Unnamed_2::HELLO_AND_DOWN_TO_BUSINESS_5.0 as ::core::ffi::c_int,
    C2Rust_Unnamed_2::HELLO_AND_DOWN_TO_BUSINESS_6.0 as ::core::ffi::c_int,
    C2Rust_Unnamed_2::HELLO_AND_DOWN_TO_BUSINESS_7.0 as ::core::ffi::c_int,
    C2Rust_Unnamed_2::HELLO_AND_DOWN_TO_BUSINESS_8.0 as ::core::ffi::c_int,
    C2Rust_Unnamed_2::HELLO_AND_DOWN_TO_BUSINESS_9.0 as ::core::ffi::c_int,
    C2Rust_Unnamed_2::HELLO_AND_DOWN_TO_BUSINESS_10.0 as ::core::ffi::c_int,
];
#[no_mangle]
pub static mut NUM_HELLO_LINES: size_t = 0;
static mut rescue_lines: [::core::ffi::c_int; 6] = [
    C2Rust_Unnamed_2::RESCUE_EXPLANATION.0 as ::core::ffi::c_int,
    C2Rust_Unnamed_2::RESCUE_AGAIN_1.0 as ::core::ffi::c_int,
    C2Rust_Unnamed_2::RESCUE_AGAIN_2.0 as ::core::ffi::c_int,
    C2Rust_Unnamed_2::RESCUE_AGAIN_3.0 as ::core::ffi::c_int,
    C2Rust_Unnamed_2::RESCUE_AGAIN_4.0 as ::core::ffi::c_int,
    C2Rust_Unnamed_2::RESCUE_AGAIN_5.0 as ::core::ffi::c_int,
];
#[no_mangle]
pub static mut NUM_RESCUE_LINES: size_t = 0;
unsafe extern "C" fn GetNumLines(mut array: *const ::core::ffi::c_int) -> size_t {
    if array == &raw const ok_buy_event_lines as *const ::core::ffi::c_int {
        return NUM_EVENT_ITEMS
    } else if array == &raw const ok_buy_alien_race_lines as *const ::core::ffi::c_int {
        return NUM_ALIEN_RACE_ITEMS
    } else if array == &raw const ok_buy_history_lines as *const ::core::ffi::c_int {
        return NUM_HISTORY_ITEMS
    } else if array
        == &raw const hello_and_down_to_business_lines as *const ::core::ffi::c_int
    {
        return NUM_HELLO_LINES
    } else if array == &raw const rescue_lines as *const ::core::ffi::c_int {
        return NUM_RESCUE_LINES
    }
    return 0 as size_t;
}
unsafe extern "C" fn GetLineSafe(
    mut array: *const ::core::ffi::c_int,
    mut linenum: size_t,
) -> ::core::ffi::c_int {
    let array_size: size_t = GetNumLines(array);
    (array_size > 0 as size_t
        || {
            __assert_fail(
                b"array_size > 0\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/uqm/comm/melnorm/melnorm.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                432 as ::core::ffi::c_int,
                b"GetLineSafe\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    if linenum >= array_size {
        linenum = array_size.wrapping_sub(1 as size_t);
    }
    return *array.offset(linenum as isize);
}
static mut tech_sale_catalog: [TechSaleData; 13] = [
    TechSaleData {
        techId: TechId_t::TECH_MODULE_BLASTER,
        price: 75 as ::core::ffi::c_int * 2 as ::core::ffi::c_int,
        sale_line: C2Rust_Unnamed_2::NEW_TECH_1.0 as ::core::ffi::c_int,
        sold_line: C2Rust_Unnamed_2::OK_BUY_NEW_TECH_1.0 as ::core::ffi::c_int,
    },
    TechSaleData {
        techId: TechId_t::TECH_LANDER_SPEED,
        price: 75 as ::core::ffi::c_int * 2 as ::core::ffi::c_int,
        sale_line: C2Rust_Unnamed_2::NEW_TECH_2.0 as ::core::ffi::c_int,
        sold_line: C2Rust_Unnamed_2::OK_BUY_NEW_TECH_2.0 as ::core::ffi::c_int,
    },
    TechSaleData {
        techId: TechId_t::TECH_MODULE_ANTIMISSILE,
        price: 75 as ::core::ffi::c_int * 2 as ::core::ffi::c_int,
        sale_line: C2Rust_Unnamed_2::NEW_TECH_3.0 as ::core::ffi::c_int,
        sold_line: C2Rust_Unnamed_2::OK_BUY_NEW_TECH_3.0 as ::core::ffi::c_int,
    },
    TechSaleData {
        techId: TechId_t::TECH_LANDER_SHIELD_BIO,
        price: 75 as ::core::ffi::c_int * 2 as ::core::ffi::c_int,
        sale_line: C2Rust_Unnamed_2::NEW_TECH_4.0 as ::core::ffi::c_int,
        sold_line: C2Rust_Unnamed_2::OK_BUY_NEW_TECH_4.0 as ::core::ffi::c_int,
    },
    TechSaleData {
        techId: TechId_t::TECH_LANDER_CARGO,
        price: 75 as ::core::ffi::c_int * 2 as ::core::ffi::c_int,
        sale_line: C2Rust_Unnamed_2::NEW_TECH_5.0 as ::core::ffi::c_int,
        sold_line: C2Rust_Unnamed_2::OK_BUY_NEW_TECH_5.0 as ::core::ffi::c_int,
    },
    TechSaleData {
        techId: TechId_t::TECH_MODULE_BIGFUELTANK,
        price: 75 as ::core::ffi::c_int * 2 as ::core::ffi::c_int,
        sale_line: C2Rust_Unnamed_2::NEW_TECH_6.0 as ::core::ffi::c_int,
        sold_line: C2Rust_Unnamed_2::OK_BUY_NEW_TECH_6.0 as ::core::ffi::c_int,
    },
    TechSaleData {
        techId: TechId_t::TECH_LANDER_RAPIDFIRE,
        price: 75 as ::core::ffi::c_int * 2 as ::core::ffi::c_int,
        sale_line: C2Rust_Unnamed_2::NEW_TECH_7.0 as ::core::ffi::c_int,
        sold_line: C2Rust_Unnamed_2::OK_BUY_NEW_TECH_7.0 as ::core::ffi::c_int,
    },
    TechSaleData {
        techId: TechId_t::TECH_LANDER_SHIELD_QUAKE,
        price: 75 as ::core::ffi::c_int * 2 as ::core::ffi::c_int,
        sale_line: C2Rust_Unnamed_2::NEW_TECH_8.0 as ::core::ffi::c_int,
        sold_line: C2Rust_Unnamed_2::OK_BUY_NEW_TECH_8.0 as ::core::ffi::c_int,
    },
    TechSaleData {
        techId: TechId_t::TECH_MODULE_TRACKING,
        price: 75 as ::core::ffi::c_int * 2 as ::core::ffi::c_int,
        sale_line: C2Rust_Unnamed_2::NEW_TECH_9.0 as ::core::ffi::c_int,
        sold_line: C2Rust_Unnamed_2::OK_BUY_NEW_TECH_9.0 as ::core::ffi::c_int,
    },
    TechSaleData {
        techId: TechId_t::TECH_LANDER_SHIELD_LIGHTNING,
        price: 75 as ::core::ffi::c_int * 2 as ::core::ffi::c_int,
        sale_line: C2Rust_Unnamed_2::NEW_TECH_10.0 as ::core::ffi::c_int,
        sold_line: C2Rust_Unnamed_2::OK_BUY_NEW_TECH_10.0 as ::core::ffi::c_int,
    },
    TechSaleData {
        techId: TechId_t::TECH_LANDER_SHIELD_HEAT,
        price: 75 as ::core::ffi::c_int * 2 as ::core::ffi::c_int,
        sale_line: C2Rust_Unnamed_2::NEW_TECH_11.0 as ::core::ffi::c_int,
        sold_line: C2Rust_Unnamed_2::OK_BUY_NEW_TECH_11.0 as ::core::ffi::c_int,
    },
    TechSaleData {
        techId: TechId_t::TECH_MODULE_CANNON,
        price: 75 as ::core::ffi::c_int * 2 as ::core::ffi::c_int,
        sale_line: C2Rust_Unnamed_2::NEW_TECH_12.0 as ::core::ffi::c_int,
        sold_line: C2Rust_Unnamed_2::OK_BUY_NEW_TECH_12.0 as ::core::ffi::c_int,
    },
    TechSaleData {
        techId: TechId_t::TECH_MODULE_FURNACE,
        price: 75 as ::core::ffi::c_int * 2 as ::core::ffi::c_int,
        sale_line: C2Rust_Unnamed_2::NEW_TECH_13.0 as ::core::ffi::c_int,
        sold_line: C2Rust_Unnamed_2::OK_BUY_NEW_TECH_13.0 as ::core::ffi::c_int,
    },
];
#[no_mangle]
pub static mut NUM_TECH_ITEMS: size_t = 0;
unsafe extern "C" fn GetNextTechForSale() -> *const TechSaleData {
    let mut i: size_t = 0 as size_t;
    i = 0 as size_t;
    while i < NUM_TECH_ITEMS {
        if !HasTech(tech_sale_catalog[i as usize].techId) {
            return (&raw const tech_sale_catalog as *const TechSaleData)
                .offset(i as isize);
        }
        i = i.wrapping_add(1);
    }
    return ::core::ptr::null::<TechSaleData>();
}
static mut prevMsgMode: StatMsgMode = StatMsgMode::SMM_UNDEFINED;
unsafe extern "C" fn ShipWorth() -> COUNT {
    let mut i: BYTE = 0;
    let mut crew_pods: SBYTE = 0;
    let mut worth: COUNT = 0;
    worth = (GlobData.SIS_state.NumLanders as ::core::ffi::c_int
        * GlobData
            .Game_state
            .ModuleCost[C2Rust_Unnamed_0::PLANET_LANDER.0 as ::core::ffi::c_int as usize]
            as ::core::ffi::c_int) as COUNT;
    i = 0 as BYTE;
    while (i as ::core::ffi::c_int) < 11 as ::core::ffi::c_int {
        if (GlobData.SIS_state.DriveSlots[i as usize] as ::core::ffi::c_int)
            < C2Rust_Unnamed_0::NUM_MODULES.0 as ::core::ffi::c_int
        {
            worth = (worth as ::core::ffi::c_int
                + GlobData
                    .Game_state
                    .ModuleCost[C2Rust_Unnamed_0::FUSION_THRUSTER.0 as ::core::ffi::c_int
                    as usize] as ::core::ffi::c_int) as COUNT;
        }
        i = i.wrapping_add(1);
    }
    i = 0 as BYTE;
    while (i as ::core::ffi::c_int) < 8 as ::core::ffi::c_int {
        if (GlobData.SIS_state.JetSlots[i as usize] as ::core::ffi::c_int)
            < C2Rust_Unnamed_0::NUM_MODULES.0 as ::core::ffi::c_int
        {
            worth = (worth as ::core::ffi::c_int
                + GlobData
                    .Game_state
                    .ModuleCost[C2Rust_Unnamed_0::TURNING_JETS.0 as ::core::ffi::c_int
                    as usize] as ::core::ffi::c_int) as COUNT;
        }
        i = i.wrapping_add(1);
    }
    crew_pods = -(((GlobData.SIS_state.CrewEnlisted as ::core::ffi::c_int
        + 50 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) / 50 as ::core::ffi::c_int)
        as SBYTE as ::core::ffi::c_int) as SBYTE;
    i = 0 as BYTE;
    while (i as ::core::ffi::c_int) < 16 as ::core::ffi::c_int {
        let mut which_module: BYTE = 0;
        which_module = GlobData.SIS_state.ModuleSlots[i as usize];
        if (which_module as ::core::ffi::c_int)
            < C2Rust_Unnamed_0::BOMB_MODULE_0.0 as ::core::ffi::c_int
            && (which_module as ::core::ffi::c_int
                != C2Rust_Unnamed_0::CREW_POD.0 as ::core::ffi::c_int
                || {
                    crew_pods += 1;
                    crew_pods as ::core::ffi::c_int > 0 as ::core::ffi::c_int
                })
        {
            worth = (worth as ::core::ffi::c_int
                + GlobData.Game_state.ModuleCost[which_module as usize]
                    as ::core::ffi::c_int) as COUNT;
        }
        i = i.wrapping_add(1);
    }
    return worth;
}
static mut rescue_fuel: COUNT = 0;
static mut SIS_copy: SIS_STATE = SIS_STATE {
    log_x: 0,
    log_y: 0,
    ResUnits: 0,
    FuelOnBoard: 0,
    CrewEnlisted: 0,
    TotalElementMass: 0,
    TotalBioMass: 0,
    ModuleSlots: [0; 16],
    DriveSlots: [0; 11],
    JetSlots: [0; 8],
    NumLanders: 0,
    ElementAmounts: [0; 8],
    ShipName: [0; 16],
    CommanderName: [0; 16],
    PlanetName: [0; 16],
};
unsafe extern "C" fn GetStripModuleRef(
    mut moduleID: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    match moduleID {
        0 => return C2Rust_Unnamed_2::LANDERS.0 as ::core::ffi::c_int,
        1 => return C2Rust_Unnamed_2::THRUSTERS.0 as ::core::ffi::c_int,
        2 => return C2Rust_Unnamed_2::JETS.0 as ::core::ffi::c_int,
        3 => return C2Rust_Unnamed_2::PODS.0 as ::core::ffi::c_int,
        4 => return C2Rust_Unnamed_2::BAYS.0 as ::core::ffi::c_int,
        7 => return C2Rust_Unnamed_2::DYNAMOS.0 as ::core::ffi::c_int,
        8 => return C2Rust_Unnamed_2::FURNACES.0 as ::core::ffi::c_int,
        9 => return C2Rust_Unnamed_2::GUNS.0 as ::core::ffi::c_int,
        10 => return C2Rust_Unnamed_2::BLASTERS.0 as ::core::ffi::c_int,
        11 => return C2Rust_Unnamed_2::CANNONS.0 as ::core::ffi::c_int,
        12 => return C2Rust_Unnamed_2::TRACKERS.0 as ::core::ffi::c_int,
        13 => return C2Rust_Unnamed_2::DEFENSES.0 as ::core::ffi::c_int,
        _ => {
            (false
                && !(b"Unknown module\0".as_ptr() as *const ::core::ffi::c_char)
                    .is_null()
                || {
                    __assert_fail(
                        b"0 && \"Unknown module\"\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"src/uqm/comm/melnorm/melnorm.c\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        554 as ::core::ffi::c_int,
                        b"GetStripModuleRef\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    0 as ::core::ffi::c_int != 0
                }) as ::core::ffi::c_int;
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn getStripRandomSeed() -> DWORD {
    let mut x: DWORD = 0;
    let mut y: DWORD = 0;
    x = (logxToUniverse(GlobData.SIS_state.log_x) as ::core::ffi::c_int
        / 100 as ::core::ffi::c_int) as DWORD;
    y = (logyToUniverse(GlobData.SIS_state.log_y) as ::core::ffi::c_int
        / 100 as ::core::ffi::c_int) as DWORD;
    return y
        .wrapping_mul(1013 as DWORD)
        .wrapping_add(x)
        .wrapping_add(GlobData.SIS_state.NumLanders as DWORD)
        .wrapping_add(GlobData.SIS_state.ModuleSlots[1usize] as DWORD)
        .wrapping_add(GlobData.SIS_state.ModuleSlots[4usize] as DWORD)
        .wrapping_add(GlobData.SIS_state.ModuleSlots[7usize] as DWORD)
        .wrapping_add(GlobData.SIS_state.ModuleSlots[10usize] as DWORD);
}
unsafe extern "C" fn StripShip(mut fuel_required: COUNT) -> BOOLEAN {
    let mut i: BYTE = 0;
    let mut which_module: BYTE = 0;
    let mut crew_pods: SBYTE = 0;
    setGameState(
        &raw mut GlobData.Game_state.GameState as *mut BYTE,
        C2Rust_Unnamed_1::MELNORME_RESCUE_REFUSED.0 as ::core::ffi::c_int,
        C2Rust_Unnamed_1::END_MELNORME_RESCUE_REFUSED.0 as ::core::ffi::c_int,
        0 as BYTE,
    );
    crew_pods = -(((GlobData.SIS_state.CrewEnlisted as ::core::ffi::c_int
        + 50 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) / 50 as ::core::ffi::c_int)
        as SBYTE as ::core::ffi::c_int) as SBYTE;
    if fuel_required as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        GlobData.SIS_state = SIS_copy;
        DeltaSISGauges(
            0x7fff as SIZE,
            rescue_fuel as SIZE,
            0x7fff as ::core::ffi::c_int,
        );
    } else if fuel_required as ::core::ffi::c_int
        == !(0 as ::core::ffi::c_int) as COUNT as ::core::ffi::c_int
    {
        GlobData.SIS_state.NumLanders = 0 as BYTE;
        i = 0 as BYTE;
        while (i as ::core::ffi::c_int) < 11 as ::core::ffi::c_int {
            GlobData.SIS_state.DriveSlots[i as usize] = (C2Rust_Unnamed_0::NUM_MODULES.0
                as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as BYTE;
            i = i.wrapping_add(1);
        }
        i = 0 as BYTE;
        while (i as ::core::ffi::c_int) < 8 as ::core::ffi::c_int {
            GlobData.SIS_state.JetSlots[i as usize] = (C2Rust_Unnamed_0::NUM_MODULES.0
                as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as BYTE;
            i = i.wrapping_add(1);
        }
        if GlobData.SIS_state.FuelOnBoard
            > (100 as ::core::ffi::c_int * 100 as ::core::ffi::c_int
                / 10 as ::core::ffi::c_int) as DWORD
        {
            GlobData.SIS_state.FuelOnBoard = (100 as ::core::ffi::c_int
                * 100 as ::core::ffi::c_int / 10 as ::core::ffi::c_int) as DWORD;
        }
        GlobData.SIS_state.TotalBioMass = 0 as COUNT;
        GlobData.SIS_state.TotalElementMass = 0 as COUNT;
        i = 0 as BYTE;
        while (i as ::core::ffi::c_int)
            < C2Rust_Unnamed::NUM_ELEMENT_CATEGORIES.0 as ::core::ffi::c_int
        {
            GlobData.SIS_state.ElementAmounts[i as usize] = 0 as COUNT;
            i = i.wrapping_add(1);
        }
        i = 0 as BYTE;
        while (i as ::core::ffi::c_int) < 16 as ::core::ffi::c_int {
            which_module = GlobData.SIS_state.ModuleSlots[i as usize];
            if (which_module as ::core::ffi::c_int)
                < C2Rust_Unnamed_0::BOMB_MODULE_0.0 as ::core::ffi::c_int
                && (which_module as ::core::ffi::c_int
                    != C2Rust_Unnamed_0::CREW_POD.0 as ::core::ffi::c_int
                    || {
                        crew_pods += 1;
                        crew_pods as ::core::ffi::c_int > 0 as ::core::ffi::c_int
                    })
            {
                GlobData.SIS_state.ModuleSlots[i as usize] = (C2Rust_Unnamed_0::NUM_MODULES
                    .0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as BYTE;
            }
            i = i.wrapping_add(1);
        }
        DeltaSISGauges(0x7fff as SIZE, 0x7fff as SIZE, 0x7fff as ::core::ffi::c_int);
    } else if fuel_required != 0 {
        let mut bays: SBYTE = 0;
        let mut num_searches: BYTE = 0;
        let mut beg_mod: BYTE = 0;
        let mut end_mod: BYTE = 0;
        let mut worth: COUNT = 0;
        let mut total: COUNT = 0;
        let mut module_count: [BYTE; 14] = [0; 14];
        let mut slot: BYTE = 0;
        let mut capacity: DWORD = 0;
        let mut rc: *mut RandomContext = ::core::ptr::null_mut::<RandomContext>();
        rc = RandomContext_New();
        RandomContext_SeedRandom(rc, getStripRandomSeed());
        SIS_copy = GlobData.SIS_state;
        i = C2Rust_Unnamed_0::PLANET_LANDER.0 as ::core::ffi::c_int as BYTE;
        while (i as ::core::ffi::c_int)
            < C2Rust_Unnamed_0::BOMB_MODULE_0.0 as ::core::ffi::c_int
        {
            module_count[i as usize] = 0 as BYTE;
            i = i.wrapping_add(1);
        }
        capacity = (100 as ::core::ffi::c_int * 100 as ::core::ffi::c_int
            / 10 as ::core::ffi::c_int) as DWORD;
        slot = (16 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as BYTE;
        loop {
            if SIS_copy.ModuleSlots[slot as usize] as ::core::ffi::c_int
                == C2Rust_Unnamed_0::FUEL_TANK.0 as ::core::ffi::c_int
                || SIS_copy.ModuleSlots[slot as usize] as ::core::ffi::c_int
                    == C2Rust_Unnamed_0::HIGHEFF_FUELSYS.0 as ::core::ffi::c_int
            {
                let mut volume: COUNT = 0;
                volume = (if SIS_copy.ModuleSlots[slot as usize] as ::core::ffi::c_int
                    == C2Rust_Unnamed_0::FUEL_TANK.0 as ::core::ffi::c_int
                {
                    50 as ::core::ffi::c_int * 100 as ::core::ffi::c_int
                } else {
                    100 as ::core::ffi::c_int * 100 as ::core::ffi::c_int
                }) as COUNT;
                capacity = capacity.wrapping_add(volume as DWORD);
            }
            let c2rust_fresh1 = slot;
            slot = slot.wrapping_sub(1);
            if c2rust_fresh1 == 0 {
                break;
            }
        }
        if fuel_required as DWORD > capacity {
            fuel_required = capacity as COUNT;
        }
        bays = -(((SIS_copy.TotalElementMass as ::core::ffi::c_int
            + 500 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
            / 500 as ::core::ffi::c_int) as SBYTE as ::core::ffi::c_int) as SBYTE;
        i = 0 as BYTE;
        while (i as ::core::ffi::c_int) < 16 as ::core::ffi::c_int {
            which_module = SIS_copy.ModuleSlots[i as usize];
            if which_module as ::core::ffi::c_int
                == C2Rust_Unnamed_0::CREW_POD.0 as ::core::ffi::c_int
            {
                crew_pods += 1;
            } else if which_module as ::core::ffi::c_int
                == C2Rust_Unnamed_0::STORAGE_BAY.0 as ::core::ffi::c_int
            {
                bays += 1;
            }
            i = i.wrapping_add(1);
        }
        worth = (fuel_required as ::core::ffi::c_int / 100 as ::core::ffi::c_int)
            as COUNT;
        total = 0 as COUNT;
        num_searches = 0 as BYTE;
        end_mod = !(0 as ::core::ffi::c_int) as BYTE;
        beg_mod = end_mod;
        while (total as ::core::ffi::c_int) < worth as ::core::ffi::c_int
            && ShipWorth() as ::core::ffi::c_int != 0
            && {
                num_searches = num_searches.wrapping_add(1);
                num_searches as ::core::ffi::c_int != 0
            }
        {
            let mut rand_val: DWORD = 0;
            rand_val = RandomContext_Random(rc);
            which_module = (rand_val as UWORD as BYTE as ::core::ffi::c_int
                % (C2Rust_Unnamed_0::CREW_POD.0 as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int)) as BYTE;
            match which_module as ::core::ffi::c_int {
                0 => {
                    if SIS_copy.NumLanders as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                    {
                        continue;
                    }
                    SIS_copy.NumLanders = SIS_copy.NumLanders.wrapping_sub(1);
                }
                1 => {
                    i = 0 as BYTE;
                    while (i as ::core::ffi::c_int) < 11 as ::core::ffi::c_int {
                        if (SIS_copy.DriveSlots[i as usize] as ::core::ffi::c_int)
                            < C2Rust_Unnamed_0::NUM_MODULES.0 as ::core::ffi::c_int
                        {
                            break;
                        }
                        i = i.wrapping_add(1);
                    }
                    if i as ::core::ffi::c_int == 11 as ::core::ffi::c_int {
                        continue;
                    }
                    SIS_copy.DriveSlots[i as usize] = (C2Rust_Unnamed_0::NUM_MODULES.0
                        as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as BYTE;
                }
                2 => {
                    i = 0 as BYTE;
                    while (i as ::core::ffi::c_int) < 8 as ::core::ffi::c_int {
                        if (SIS_copy.JetSlots[i as usize] as ::core::ffi::c_int)
                            < C2Rust_Unnamed_0::NUM_MODULES.0 as ::core::ffi::c_int
                        {
                            break;
                        }
                        i = i.wrapping_add(1);
                    }
                    if i as ::core::ffi::c_int == 8 as ::core::ffi::c_int {
                        continue;
                    }
                    SIS_copy.JetSlots[i as usize] = (C2Rust_Unnamed_0::NUM_MODULES.0
                        as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as BYTE;
                }
                3 => {
                    i = ((rand_val as UWORD as ::core::ffi::c_int
                        >> 8 as ::core::ffi::c_int) as BYTE as ::core::ffi::c_int
                        % 16 as ::core::ffi::c_int) as BYTE;
                    which_module = SIS_copy.ModuleSlots[i as usize];
                    if which_module as ::core::ffi::c_int
                        >= C2Rust_Unnamed_0::BOMB_MODULE_0.0 as ::core::ffi::c_int
                        || which_module as ::core::ffi::c_int
                            == C2Rust_Unnamed_0::FUEL_TANK.0 as ::core::ffi::c_int
                        || which_module as ::core::ffi::c_int
                            == C2Rust_Unnamed_0::HIGHEFF_FUELSYS.0 as ::core::ffi::c_int
                        || which_module as ::core::ffi::c_int
                            == C2Rust_Unnamed_0::STORAGE_BAY.0 as ::core::ffi::c_int
                            && module_count[C2Rust_Unnamed_0::STORAGE_BAY.0
                                as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                                >= bays as ::core::ffi::c_int
                        || which_module as ::core::ffi::c_int
                            == C2Rust_Unnamed_0::CREW_POD.0 as ::core::ffi::c_int
                            && module_count[C2Rust_Unnamed_0::CREW_POD.0
                                as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                                >= crew_pods as ::core::ffi::c_int
                    {
                        continue;
                    }
                    SIS_copy.ModuleSlots[i as usize] = (C2Rust_Unnamed_0::NUM_MODULES.0
                        as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as BYTE;
                }
                _ => {}
            }
            if beg_mod as ::core::ffi::c_int
                == !(0 as ::core::ffi::c_int) as BYTE as ::core::ffi::c_int
            {
                end_mod = which_module;
                beg_mod = end_mod;
            } else if which_module as ::core::ffi::c_int > end_mod as ::core::ffi::c_int
            {
                end_mod = which_module;
            }
            module_count[which_module as usize] = module_count[which_module as usize]
                .wrapping_add(1);
            total = (total as ::core::ffi::c_int
                + GlobData.Game_state.ModuleCost[which_module as usize]
                    as ::core::ffi::c_int) as COUNT;
        }
        RandomContext_Delete(rc);
        if total as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            NPCPhrase_cb(C2Rust_Unnamed_2::CHARITY.0 as ::core::ffi::c_int, None);
            DeltaSISGauges(0 as SIZE, fuel_required as SIZE, 0 as ::core::ffi::c_int);
            return BOOLEAN::FALSE;
        } else {
            NPCPhrase_cb(C2Rust_Unnamed_2::RESCUE_OFFER.0 as ::core::ffi::c_int, None);
            rescue_fuel = fuel_required;
            if rescue_fuel as DWORD == capacity {
                NPCPhrase_cb(
                    C2Rust_Unnamed_2::RESCUE_TANKS.0 as ::core::ffi::c_int,
                    None,
                );
            } else {
                NPCPhrase_cb(
                    C2Rust_Unnamed_2::RESCUE_HOME.0 as ::core::ffi::c_int,
                    None,
                );
            }
            i = C2Rust_Unnamed_0::PLANET_LANDER.0 as ::core::ffi::c_int as BYTE;
            while (i as ::core::ffi::c_int)
                < C2Rust_Unnamed_0::BOMB_MODULE_0.0 as ::core::ffi::c_int
            {
                if module_count[i as usize] != 0 {
                    if i as ::core::ffi::c_int == end_mod as ::core::ffi::c_int
                        && i as ::core::ffi::c_int != beg_mod as ::core::ffi::c_int
                    {
                        NPCPhrase_cb(
                            C2Rust_Unnamed_2::END_LIST_WITH_AND.0 as ::core::ffi::c_int,
                            None,
                        );
                    }
                    NPCPhrase_cb(
                        C2Rust_Unnamed_2::ENUMERATE_ONE.0 as ::core::ffi::c_int
                            + (module_count[i as usize] as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int),
                        None,
                    );
                    NPCPhrase_cb(GetStripModuleRef(i as ::core::ffi::c_int), None);
                }
                i = i.wrapping_add(1);
            }
        }
    }
    return BOOLEAN::TRUE;
}
unsafe extern "C" fn ExitConversation(mut R: RESPONSE_REF) {
    if R as ::core::ffi::c_int == C2Rust_Unnamed_2::no_trade_now.0 as ::core::ffi::c_int
    {
        NPCPhrase_cb(
            C2Rust_Unnamed_2::OK_NO_TRADE_NOW_BYE.0 as ::core::ffi::c_int,
            None,
        );
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_2::youre_on.0 as ::core::ffi::c_int
    {
        NPCPhrase_cb(
            C2Rust_Unnamed_2::YOU_GIVE_US_NO_CHOICE.0 as ::core::ffi::c_int,
            None,
        );
        setGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_1::MELNORME_ANGER.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_1::END_MELNORME_ANGER.0 as ::core::ffi::c_int,
            1 as BYTE,
        );
        setSegue(Segue::Segue_hostile);
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_2::so_we_can_attack.0 as ::core::ffi::c_int
    {
        NPCPhrase_cb(C2Rust_Unnamed_2::DECEITFUL_HUMAN.0 as ::core::ffi::c_int, None);
        setGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_1::MELNORME_ANGER.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_1::END_MELNORME_ANGER.0 as ::core::ffi::c_int,
            2 as BYTE,
        );
        setSegue(Segue::Segue_hostile);
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_2::bye_melnorme_slightly_angry.0 as ::core::ffi::c_int
    {
        NPCPhrase_cb(
            C2Rust_Unnamed_2::MELNORME_SLIGHTLY_ANGRY_GOODBYE.0 as ::core::ffi::c_int,
            None,
        );
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_2::ok_strip_me.0 as ::core::ffi::c_int
    {
        if (ShipWorth() as ::core::ffi::c_int)
            < 4000 as ::core::ffi::c_int / 50 as ::core::ffi::c_int
        {
            NPCPhrase_cb(
                C2Rust_Unnamed_2::NOT_WORTH_STRIPPING.0 as ::core::ffi::c_int,
                None,
            );
        } else {
            setGameState(
                &raw mut GlobData.Game_state.GameState as *mut BYTE,
                C2Rust_Unnamed_1::MELNORME_ANGER.0 as ::core::ffi::c_int,
                C2Rust_Unnamed_1::END_MELNORME_ANGER.0 as ::core::ffi::c_int,
                0 as BYTE,
            );
            StripShip(!(0 as ::core::ffi::c_int) as COUNT);
            NPCPhrase_cb(C2Rust_Unnamed_2::FAIR_JUSTICE.0 as ::core::ffi::c_int, None);
        }
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_2::fight_some_more.0 as ::core::ffi::c_int
    {
        NPCPhrase_cb(C2Rust_Unnamed_2::OK_FIGHT_SOME_MORE.0 as ::core::ffi::c_int, None);
        setGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_1::MELNORME_ANGER.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_1::END_MELNORME_ANGER.0 as ::core::ffi::c_int,
            3 as BYTE,
        );
        setSegue(Segue::Segue_hostile);
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_2::bye_melnorme_pissed_off.0 as ::core::ffi::c_int
    {
        NPCPhrase_cb(
            C2Rust_Unnamed_2::MELNORME_PISSED_OFF_GOODBYE.0 as ::core::ffi::c_int,
            None,
        );
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_2::well_if_thats_the_way_you_feel.0 as ::core::ffi::c_int
    {
        NPCPhrase_cb(C2Rust_Unnamed_2::WE_FIGHT_AGAIN.0 as ::core::ffi::c_int, None);
        setSegue(Segue::Segue_hostile);
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_2::you_hate_us_so_we_go_away.0 as ::core::ffi::c_int
    {
        NPCPhrase_cb(C2Rust_Unnamed_2::HATE_YOU_GOODBYE.0 as ::core::ffi::c_int, None);
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_2::take_it.0 as ::core::ffi::c_int
    {
        StripShip(0 as COUNT);
        NPCPhrase_cb(
            C2Rust_Unnamed_2::HAPPY_TO_HAVE_RESCUED.0 as ::core::ffi::c_int,
            None,
        );
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_2::leave_it.0 as ::core::ffi::c_int
    {
        setGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_1::MELNORME_RESCUE_REFUSED.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_1::END_MELNORME_RESCUE_REFUSED.0 as ::core::ffi::c_int,
            1 as BYTE,
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_2::MAYBE_SEE_YOU_LATER.0 as ::core::ffi::c_int,
            None,
        );
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_2::no_help.0 as ::core::ffi::c_int
    {
        setGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_1::MELNORME_RESCUE_REFUSED.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_1::END_MELNORME_RESCUE_REFUSED.0 as ::core::ffi::c_int,
            1 as BYTE,
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_2::GOODBYE_AND_GOODLUCK.0 as ::core::ffi::c_int,
            None,
        );
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_2::no_changed_mind.0 as ::core::ffi::c_int
    {
        NPCPhrase_cb(
            C2Rust_Unnamed_2::GOODBYE_AND_GOODLUCK_AGAIN.0 as ::core::ffi::c_int,
            None,
        );
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_2::be_leaving_now.0 as ::core::ffi::c_int
        || R as ::core::ffi::c_int == C2Rust_Unnamed_2::goodbye.0 as ::core::ffi::c_int
    {
        NPCPhrase_cb(C2Rust_Unnamed_2::FRIENDLY_GOODBYE.0 as ::core::ffi::c_int, None);
    }
}
unsafe extern "C" fn DoRescue(mut R: RESPONSE_REF) {
    let mut dx: SIZE = 0;
    let mut dy: SIZE = 0;
    let mut fuel_required: COUNT = 0;
    dx = (logxToUniverse(GlobData.SIS_state.log_x) as ::core::ffi::c_int
        - 1752 as ::core::ffi::c_int) as SIZE;
    dy = (logyToUniverse(GlobData.SIS_state.log_y) as ::core::ffi::c_int
        - 1450 as ::core::ffi::c_int) as SIZE;
    fuel_required = (square_root(
        (dx as ::core::ffi::c_long * dx as ::core::ffi::c_long
            + dy as ::core::ffi::c_long * dy as ::core::ffi::c_long) as DWORD,
    ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int * 100 as ::core::ffi::c_int)
        as COUNT;
    if StripShip(fuel_required).0 != 0 {
        DoResponsePhrase(
            C2Rust_Unnamed_2::take_it.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(ExitConversation as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        );
        DoResponsePhrase(
            C2Rust_Unnamed_2::leave_it.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(ExitConversation as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        );
    }
}
unsafe extern "C" fn GetAvailableCredits() -> COUNT {
    return (((getGameState(
        &raw mut GlobData.Game_state.GameState as *mut BYTE,
        C2Rust_Unnamed_1::MELNORME_CREDIT1.0 as ::core::ffi::c_int,
        C2Rust_Unnamed_1::END_MELNORME_CREDIT1.0 as ::core::ffi::c_int,
    ) as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as UWORD as ::core::ffi::c_int
        | getGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_1::MELNORME_CREDIT0.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_1::END_MELNORME_CREDIT0.0 as ::core::ffi::c_int,
        ) as ::core::ffi::c_int) as COUNT;
}
unsafe extern "C" fn SetAvailableCredits(mut credits: COUNT) {
    setGameState(
        &raw mut GlobData.Game_state.GameState as *mut BYTE,
        C2Rust_Unnamed_1::MELNORME_CREDIT0.0 as ::core::ffi::c_int,
        C2Rust_Unnamed_1::END_MELNORME_CREDIT0.0 as ::core::ffi::c_int,
        credits as BYTE,
    );
    setGameState(
        &raw mut GlobData.Game_state.GameState as *mut BYTE,
        C2Rust_Unnamed_1::MELNORME_CREDIT1.0 as ::core::ffi::c_int,
        C2Rust_Unnamed_1::END_MELNORME_CREDIT1.0 as ::core::ffi::c_int,
        (credits as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as BYTE,
    );
}
unsafe extern "C" fn DeltaCredit(mut delta_credit: SIZE) -> bool {
    let mut Credit: COUNT = GetAvailableCredits();
    if delta_credit as ::core::ffi::c_int >= 0 as ::core::ffi::c_int
        || -(delta_credit as ::core::ffi::c_int) <= Credit as ::core::ffi::c_int
    {
        Credit = (Credit as ::core::ffi::c_int + delta_credit as ::core::ffi::c_int)
            as COUNT;
        SetAvailableCredits(Credit);
        DrawStatusMessage(::core::ptr::null::<UNICODE>());
        return true;
    }
    NPCPhrase_cb(C2Rust_Unnamed_2::NEED_MORE_CREDIT0.0 as ::core::ffi::c_int, None);
    NPCNumber(
        -(delta_credit as ::core::ffi::c_int) - Credit as ::core::ffi::c_int,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    NPCPhrase_cb(C2Rust_Unnamed_2::NEED_MORE_CREDIT1.0 as ::core::ffi::c_int, None);
    return false;
}
unsafe extern "C" fn CurrentEvents() {
    let mut stack: BYTE = getGameState(
        &raw mut GlobData.Game_state.GameState as *mut BYTE,
        C2Rust_Unnamed_1::MELNORME_EVENTS_INFO_STACK.0 as ::core::ffi::c_int,
        C2Rust_Unnamed_1::END_MELNORME_EVENTS_INFO_STACK.0 as ::core::ffi::c_int,
    );
    let phraseId: ::core::ffi::c_int = GetLineSafe(
        &raw const ok_buy_event_lines as *const ::core::ffi::c_int,
        stack as size_t,
    );
    NPCPhrase_cb(phraseId, None);
    setGameState(
        &raw mut GlobData.Game_state.GameState as *mut BYTE,
        C2Rust_Unnamed_1::MELNORME_EVENTS_INFO_STACK.0 as ::core::ffi::c_int,
        C2Rust_Unnamed_1::END_MELNORME_EVENTS_INFO_STACK.0 as ::core::ffi::c_int,
        (stack as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as BYTE,
    );
}
unsafe extern "C" fn AlienRaces() {
    let mut stack: BYTE = getGameState(
        &raw mut GlobData.Game_state.GameState as *mut BYTE,
        C2Rust_Unnamed_1::MELNORME_ALIEN_INFO_STACK.0 as ::core::ffi::c_int,
        C2Rust_Unnamed_1::END_MELNORME_ALIEN_INFO_STACK.0 as ::core::ffi::c_int,
    );
    let phraseId: ::core::ffi::c_int = GetLineSafe(
        &raw const ok_buy_alien_race_lines as *const ::core::ffi::c_int,
        stack as size_t,
    );
    match phraseId {
        182 => {
            if getGameState(
                &raw mut GlobData.Game_state.GameState as *mut BYTE,
                C2Rust_Unnamed_1::FOUND_PLUTO_SPATHI.0 as ::core::ffi::c_int,
                C2Rust_Unnamed_1::END_FOUND_PLUTO_SPATHI.0 as ::core::ffi::c_int,
            ) == 0
            {
                setGameState(
                    &raw mut GlobData.Game_state.GameState as *mut BYTE,
                    C2Rust_Unnamed_1::KNOW_SPATHI_PASSWORD.0 as ::core::ffi::c_int,
                    C2Rust_Unnamed_1::END_KNOW_SPATHI_PASSWORD.0 as ::core::ffi::c_int,
                    1 as BYTE,
                );
                setGameState(
                    &raw mut GlobData.Game_state.GameState as *mut BYTE,
                    C2Rust_Unnamed_1::SPATHI_HOME_VISITS.0 as ::core::ffi::c_int,
                    C2Rust_Unnamed_1::END_SPATHI_HOME_VISITS.0 as ::core::ffi::c_int,
                    7 as BYTE,
                );
            }
        }
        183 => {
            if (getGameState(
                &raw mut GlobData.Game_state.GameState as *mut BYTE,
                C2Rust_Unnamed_1::KNOW_ABOUT_SHATTERED.0 as ::core::ffi::c_int,
                C2Rust_Unnamed_1::END_KNOW_ABOUT_SHATTERED.0 as ::core::ffi::c_int,
            ) as ::core::ffi::c_int) < 2 as ::core::ffi::c_int
            {
                setGameState(
                    &raw mut GlobData.Game_state.GameState as *mut BYTE,
                    C2Rust_Unnamed_1::KNOW_ABOUT_SHATTERED.0 as ::core::ffi::c_int,
                    C2Rust_Unnamed_1::END_KNOW_ABOUT_SHATTERED.0 as ::core::ffi::c_int,
                    2 as BYTE,
                );
            }
            setGameState(
                &raw mut GlobData.Game_state.GameState as *mut BYTE,
                C2Rust_Unnamed_1::KNOW_SYREEN_WORLD_SHATTERED.0 as ::core::ffi::c_int,
                C2Rust_Unnamed_1::END_KNOW_SYREEN_WORLD_SHATTERED.0
                    as ::core::ffi::c_int,
                1 as BYTE,
            );
        }
        _ => {}
    }
    NPCPhrase_cb(phraseId, None);
    setGameState(
        &raw mut GlobData.Game_state.GameState as *mut BYTE,
        C2Rust_Unnamed_1::MELNORME_ALIEN_INFO_STACK.0 as ::core::ffi::c_int,
        C2Rust_Unnamed_1::END_MELNORME_ALIEN_INFO_STACK.0 as ::core::ffi::c_int,
        (stack as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as BYTE,
    );
}
unsafe extern "C" fn History() {
    let mut stack: BYTE = getGameState(
        &raw mut GlobData.Game_state.GameState as *mut BYTE,
        C2Rust_Unnamed_1::MELNORME_HISTORY_INFO_STACK.0 as ::core::ffi::c_int,
        C2Rust_Unnamed_1::END_MELNORME_HISTORY_INFO_STACK.0 as ::core::ffi::c_int,
    );
    let phraseId: ::core::ffi::c_int = GetLineSafe(
        &raw const ok_buy_history_lines as *const ::core::ffi::c_int,
        stack as size_t,
    );
    NPCPhrase_cb(phraseId, None);
    setGameState(
        &raw mut GlobData.Game_state.GameState as *mut BYTE,
        C2Rust_Unnamed_1::MELNORME_HISTORY_INFO_STACK.0 as ::core::ffi::c_int,
        C2Rust_Unnamed_1::END_MELNORME_HISTORY_INFO_STACK.0 as ::core::ffi::c_int,
        (stack as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as BYTE,
    );
}
unsafe extern "C" fn AnyInfoLeftToSell() -> bool {
    return (getGameState(
        &raw mut GlobData.Game_state.GameState as *mut BYTE,
        C2Rust_Unnamed_1::MELNORME_EVENTS_INFO_STACK.0 as ::core::ffi::c_int,
        C2Rust_Unnamed_1::END_MELNORME_EVENTS_INFO_STACK.0 as ::core::ffi::c_int,
    ) as size_t) < NUM_EVENT_ITEMS
        || (getGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_1::MELNORME_ALIEN_INFO_STACK.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_1::END_MELNORME_ALIEN_INFO_STACK.0 as ::core::ffi::c_int,
        ) as size_t) < NUM_ALIEN_RACE_ITEMS
        || (getGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_1::MELNORME_HISTORY_INFO_STACK.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_1::END_MELNORME_HISTORY_INFO_STACK.0 as ::core::ffi::c_int,
        ) as size_t) < NUM_HISTORY_ITEMS;
}
static mut AskedToBuy: BYTE = 0;
unsafe extern "C" fn DoBuy(mut R: RESPONSE_REF) {
    let mut credit: COUNT = 0;
    let mut needed_credit: SIZE = 0;
    let mut slot: BYTE = 0;
    let mut capacity: DWORD = 0;
    credit = GetAvailableCredits();
    capacity = (100 as ::core::ffi::c_int * 100 as ::core::ffi::c_int
        / 10 as ::core::ffi::c_int) as DWORD;
    slot = (16 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as BYTE;
    loop {
        if GlobData.SIS_state.ModuleSlots[slot as usize] as ::core::ffi::c_int
            == C2Rust_Unnamed_0::FUEL_TANK.0 as ::core::ffi::c_int
            || GlobData.SIS_state.ModuleSlots[slot as usize] as ::core::ffi::c_int
                == C2Rust_Unnamed_0::HIGHEFF_FUELSYS.0 as ::core::ffi::c_int
        {
            let mut volume: COUNT = 0;
            volume = (if GlobData.SIS_state.ModuleSlots[slot as usize]
                as ::core::ffi::c_int
                == C2Rust_Unnamed_0::FUEL_TANK.0 as ::core::ffi::c_int
            {
                50 as ::core::ffi::c_int * 100 as ::core::ffi::c_int
            } else {
                100 as ::core::ffi::c_int * 100 as ::core::ffi::c_int
            }) as COUNT;
            capacity = capacity.wrapping_add(volume as DWORD);
        }
        let c2rust_fresh5 = slot;
        slot = slot.wrapping_sub(1);
        if c2rust_fresh5 == 0 {
            break;
        }
    }
    's_475: {
        if credit as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            AskedToBuy = BOOLEAN::TRUE.0 as ::core::ffi::c_int as BYTE;
            NPCPhrase_cb(C2Rust_Unnamed_2::NEED_CREDIT.0 as ::core::ffi::c_int, None);
            NatureOfConversation(R);
        } else {
            '_BuyBuyBuy: {
                if R as ::core::ffi::c_int
                    == C2Rust_Unnamed_2::buy_fuel.0 as ::core::ffi::c_int
                    || R as ::core::ffi::c_int
                        == C2Rust_Unnamed_2::buy_1_fuel.0 as ::core::ffi::c_int
                    || R as ::core::ffi::c_int
                        == C2Rust_Unnamed_2::buy_5_fuel.0 as ::core::ffi::c_int
                    || R as ::core::ffi::c_int
                        == C2Rust_Unnamed_2::buy_10_fuel.0 as ::core::ffi::c_int
                    || R as ::core::ffi::c_int
                        == C2Rust_Unnamed_2::buy_25_fuel.0 as ::core::ffi::c_int
                    || R as ::core::ffi::c_int
                        == C2Rust_Unnamed_2::fill_me_up.0 as ::core::ffi::c_int
                {
                    needed_credit = 0 as SIZE;
                    if R as ::core::ffi::c_int
                        == C2Rust_Unnamed_2::buy_1_fuel.0 as ::core::ffi::c_int
                    {
                        needed_credit = 1 as SIZE;
                    } else if R as ::core::ffi::c_int
                        == C2Rust_Unnamed_2::buy_5_fuel.0 as ::core::ffi::c_int
                    {
                        needed_credit = 5 as SIZE;
                    } else if R as ::core::ffi::c_int
                        == C2Rust_Unnamed_2::buy_10_fuel.0 as ::core::ffi::c_int
                    {
                        needed_credit = 10 as SIZE;
                    } else if R as ::core::ffi::c_int
                        == C2Rust_Unnamed_2::buy_25_fuel.0 as ::core::ffi::c_int
                    {
                        needed_credit = 25 as SIZE;
                    } else if R as ::core::ffi::c_int
                        == C2Rust_Unnamed_2::fill_me_up.0 as ::core::ffi::c_int
                    {
                        needed_credit = capacity
                            .wrapping_sub(GlobData.SIS_state.FuelOnBoard)
                            .wrapping_add(100 as DWORD)
                            .wrapping_sub(1 as DWORD)
                            .wrapping_div(100 as DWORD) as SIZE;
                    }
                    '_TryFuelAgain: {
                        if needed_credit as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                        {
                            if getGameState(
                                &raw mut GlobData.Game_state.GameState as *mut BYTE,
                                C2Rust_Unnamed_1::MELNORME_FUEL_PROCEDURE.0
                                    as ::core::ffi::c_int,
                                C2Rust_Unnamed_1::END_MELNORME_FUEL_PROCEDURE.0
                                    as ::core::ffi::c_int,
                            ) == 0
                            {
                                NPCPhrase_cb(
                                    C2Rust_Unnamed_2::BUY_FUEL_INTRO.0 as ::core::ffi::c_int,
                                    None,
                                );
                                setGameState(
                                    &raw mut GlobData.Game_state.GameState as *mut BYTE,
                                    C2Rust_Unnamed_1::MELNORME_FUEL_PROCEDURE.0
                                        as ::core::ffi::c_int,
                                    C2Rust_Unnamed_1::END_MELNORME_FUEL_PROCEDURE.0
                                        as ::core::ffi::c_int,
                                    1 as BYTE,
                                );
                            }
                        } else if GlobData
                            .SIS_state
                            .FuelOnBoard
                            .wrapping_div(100 as DWORD)
                            .wrapping_add(needed_credit as DWORD)
                            > capacity.wrapping_div(100 as DWORD)
                        {
                            NPCPhrase_cb(
                                C2Rust_Unnamed_2::NO_ROOM_FOR_FUEL.0 as ::core::ffi::c_int,
                                None,
                            );
                            break '_TryFuelAgain;
                        } else {
                            if needed_credit as ::core::ffi::c_int
                                * (2 as ::core::ffi::c_int / 2 as ::core::ffi::c_int)
                                <= credit as ::core::ffi::c_int
                            {
                                let mut f: DWORD = 0;
                                NPCPhrase_cb(
                                    C2Rust_Unnamed_2::GOT_FUEL.0 as ::core::ffi::c_int,
                                    None,
                                );
                                f = (needed_credit as DWORD).wrapping_mul(100 as DWORD);
                                while f as ::core::ffi::c_ulong
                                    > 0x3fff as ::core::ffi::c_ulong
                                {
                                    DeltaSISGauges(
                                        0 as SIZE,
                                        0x3fff as SIZE,
                                        0 as ::core::ffi::c_int,
                                    );
                                    f = f.wrapping_sub(0x3fff as DWORD);
                                }
                                DeltaSISGauges(
                                    0 as SIZE,
                                    f as SIZE,
                                    0 as ::core::ffi::c_int,
                                );
                            }
                            needed_credit = (needed_credit as ::core::ffi::c_int
                                * (2 as ::core::ffi::c_int / 2 as ::core::ffi::c_int))
                                as SIZE;
                        }
                        if needed_credit != 0 {
                            DeltaCredit(-(needed_credit as ::core::ffi::c_int) as SIZE);
                            if GlobData.SIS_state.FuelOnBoard >= capacity {
                                break '_BuyBuyBuy;
                            }
                        }
                    }
                    NPCPhrase_cb(
                        C2Rust_Unnamed_2::HOW_MUCH_FUEL.0 as ::core::ffi::c_int,
                        None,
                    );
                    DoResponsePhrase(
                        C2Rust_Unnamed_2::buy_1_fuel.0 as ::core::ffi::c_int
                            as RESPONSE_REF,
                        Some(DoBuy as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                        ::core::ptr::null_mut::<UNICODE>(),
                    );
                    DoResponsePhrase(
                        C2Rust_Unnamed_2::buy_5_fuel.0 as ::core::ffi::c_int
                            as RESPONSE_REF,
                        Some(DoBuy as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                        ::core::ptr::null_mut::<UNICODE>(),
                    );
                    DoResponsePhrase(
                        C2Rust_Unnamed_2::buy_10_fuel.0 as ::core::ffi::c_int
                            as RESPONSE_REF,
                        Some(DoBuy as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                        ::core::ptr::null_mut::<UNICODE>(),
                    );
                    DoResponsePhrase(
                        C2Rust_Unnamed_2::buy_25_fuel.0 as ::core::ffi::c_int
                            as RESPONSE_REF,
                        Some(DoBuy as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                        ::core::ptr::null_mut::<UNICODE>(),
                    );
                    DoResponsePhrase(
                        C2Rust_Unnamed_2::fill_me_up.0 as ::core::ffi::c_int
                            as RESPONSE_REF,
                        Some(DoBuy as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                        ::core::ptr::null_mut::<UNICODE>(),
                    );
                    DoResponsePhrase(
                        C2Rust_Unnamed_2::done_buying_fuel.0 as ::core::ffi::c_int
                            as RESPONSE_REF,
                        Some(DoBuy as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                        ::core::ptr::null_mut::<UNICODE>(),
                    );
                    break 's_475;
                } else if R as ::core::ffi::c_int
                    == C2Rust_Unnamed_2::buy_technology.0 as ::core::ffi::c_int
                    || R as ::core::ffi::c_int
                        == C2Rust_Unnamed_2::buy_new_tech.0 as ::core::ffi::c_int
                {
                    let mut nextTech: *const TechSaleData = ::core::ptr::null::<
                        TechSaleData,
                    >();
                    if getGameState(
                        &raw mut GlobData.Game_state.GameState as *mut BYTE,
                        C2Rust_Unnamed_1::MELNORME_TECH_PROCEDURE.0
                            as ::core::ffi::c_int,
                        C2Rust_Unnamed_1::END_MELNORME_TECH_PROCEDURE.0
                            as ::core::ffi::c_int,
                    ) == 0
                    {
                        NPCPhrase_cb(
                            C2Rust_Unnamed_2::BUY_NEW_TECH_INTRO.0 as ::core::ffi::c_int,
                            None,
                        );
                        setGameState(
                            &raw mut GlobData.Game_state.GameState as *mut BYTE,
                            C2Rust_Unnamed_1::MELNORME_TECH_PROCEDURE.0
                                as ::core::ffi::c_int,
                            C2Rust_Unnamed_1::END_MELNORME_TECH_PROCEDURE.0
                                as ::core::ffi::c_int,
                            1 as BYTE,
                        );
                    }
                    if R as ::core::ffi::c_int
                        == C2Rust_Unnamed_2::buy_new_tech.0 as ::core::ffi::c_int
                    {
                        nextTech = GetNextTechForSale();
                        if nextTech.is_null() {
                            break '_BuyBuyBuy;
                        } else if !DeltaCredit(-(*nextTech).price as SIZE) {
                            break '_BuyBuyBuy;
                        } else {
                            GrantTech((*nextTech).techId);
                            NPCPhrase_cb((*nextTech).sold_line, None);
                        }
                    }
                    nextTech = GetNextTechForSale();
                    if nextTech.is_null() {
                        NPCPhrase_cb(
                            C2Rust_Unnamed_2::NEW_TECH_ALL_GONE.0 as ::core::ffi::c_int,
                            None,
                        );
                    } else {
                        NPCPhrase_cb((*nextTech).sale_line, None);
                        DoResponsePhrase(
                            C2Rust_Unnamed_2::buy_new_tech.0 as ::core::ffi::c_int
                                as RESPONSE_REF,
                            Some(DoBuy as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                            ::core::ptr::null_mut::<UNICODE>(),
                        );
                        DoResponsePhrase(
                            C2Rust_Unnamed_2::no_buy_new_tech.0 as ::core::ffi::c_int
                                as RESPONSE_REF,
                            Some(DoBuy as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                            ::core::ptr::null_mut::<UNICODE>(),
                        );
                        break 's_475;
                    }
                } else if R as ::core::ffi::c_int
                    == C2Rust_Unnamed_2::buy_info.0 as ::core::ffi::c_int
                    || R as ::core::ffi::c_int
                        == C2Rust_Unnamed_2::buy_current_events.0 as ::core::ffi::c_int
                    || R as ::core::ffi::c_int
                        == C2Rust_Unnamed_2::buy_alien_races.0 as ::core::ffi::c_int
                    || R as ::core::ffi::c_int
                        == C2Rust_Unnamed_2::buy_history.0 as ::core::ffi::c_int
                {
                    if getGameState(
                        &raw mut GlobData.Game_state.GameState as *mut BYTE,
                        C2Rust_Unnamed_1::MELNORME_INFO_PROCEDURE.0
                            as ::core::ffi::c_int,
                        C2Rust_Unnamed_1::END_MELNORME_INFO_PROCEDURE.0
                            as ::core::ffi::c_int,
                    ) == 0
                    {
                        NPCPhrase_cb(
                            C2Rust_Unnamed_2::BUY_INFO_INTRO.0 as ::core::ffi::c_int,
                            None,
                        );
                        setGameState(
                            &raw mut GlobData.Game_state.GameState as *mut BYTE,
                            C2Rust_Unnamed_1::MELNORME_INFO_PROCEDURE.0
                                as ::core::ffi::c_int,
                            C2Rust_Unnamed_1::END_MELNORME_INFO_PROCEDURE.0
                                as ::core::ffi::c_int,
                            1 as BYTE,
                        );
                    } else if R as ::core::ffi::c_int
                        == C2Rust_Unnamed_2::buy_info.0 as ::core::ffi::c_int
                    {
                        NPCPhrase_cb(
                            C2Rust_Unnamed_2::OK_BUY_INFO.0 as ::core::ffi::c_int,
                            None,
                        );
                    } else if !DeltaCredit(-75 as SIZE) {
                        break '_BuyBuyBuy;
                    } else if R as ::core::ffi::c_int
                        == C2Rust_Unnamed_2::buy_current_events.0 as ::core::ffi::c_int
                    {
                        CurrentEvents();
                    } else if R as ::core::ffi::c_int
                        == C2Rust_Unnamed_2::buy_alien_races.0 as ::core::ffi::c_int
                    {
                        AlienRaces();
                    } else if R as ::core::ffi::c_int
                        == C2Rust_Unnamed_2::buy_history.0 as ::core::ffi::c_int
                    {
                        History();
                    }
                    if !AnyInfoLeftToSell() {
                        NPCPhrase_cb(
                            C2Rust_Unnamed_2::INFO_ALL_GONE.0 as ::core::ffi::c_int,
                            None,
                        );
                    } else {
                        if (getGameState(
                            &raw mut GlobData.Game_state.GameState as *mut BYTE,
                            C2Rust_Unnamed_1::MELNORME_EVENTS_INFO_STACK.0
                                as ::core::ffi::c_int,
                            C2Rust_Unnamed_1::END_MELNORME_EVENTS_INFO_STACK.0
                                as ::core::ffi::c_int,
                        ) as size_t) < NUM_EVENT_ITEMS
                        {
                            DoResponsePhrase(
                                C2Rust_Unnamed_2::buy_current_events.0 as ::core::ffi::c_int
                                    as RESPONSE_REF,
                                Some(DoBuy as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                                ::core::ptr::null_mut::<UNICODE>(),
                            );
                        }
                        if (getGameState(
                            &raw mut GlobData.Game_state.GameState as *mut BYTE,
                            C2Rust_Unnamed_1::MELNORME_ALIEN_INFO_STACK.0
                                as ::core::ffi::c_int,
                            C2Rust_Unnamed_1::END_MELNORME_ALIEN_INFO_STACK.0
                                as ::core::ffi::c_int,
                        ) as size_t) < NUM_ALIEN_RACE_ITEMS
                        {
                            DoResponsePhrase(
                                C2Rust_Unnamed_2::buy_alien_races.0 as ::core::ffi::c_int
                                    as RESPONSE_REF,
                                Some(DoBuy as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                                ::core::ptr::null_mut::<UNICODE>(),
                            );
                        }
                        if (getGameState(
                            &raw mut GlobData.Game_state.GameState as *mut BYTE,
                            C2Rust_Unnamed_1::MELNORME_HISTORY_INFO_STACK.0
                                as ::core::ffi::c_int,
                            C2Rust_Unnamed_1::END_MELNORME_HISTORY_INFO_STACK.0
                                as ::core::ffi::c_int,
                        ) as size_t) < NUM_HISTORY_ITEMS
                        {
                            DoResponsePhrase(
                                C2Rust_Unnamed_2::buy_history.0 as ::core::ffi::c_int
                                    as RESPONSE_REF,
                                Some(DoBuy as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                                ::core::ptr::null_mut::<UNICODE>(),
                            );
                        }
                        DoResponsePhrase(
                            C2Rust_Unnamed_2::done_buying_info.0 as ::core::ffi::c_int
                                as RESPONSE_REF,
                            Some(DoBuy as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                            ::core::ptr::null_mut::<UNICODE>(),
                        );
                        break 's_475;
                    }
                } else if R as ::core::ffi::c_int
                    == C2Rust_Unnamed_2::done_buying_fuel.0 as ::core::ffi::c_int
                {
                    NPCPhrase_cb(
                        C2Rust_Unnamed_2::OK_DONE_BUYING_FUEL.0 as ::core::ffi::c_int,
                        None,
                    );
                } else if R as ::core::ffi::c_int
                    == C2Rust_Unnamed_2::no_buy_new_tech.0 as ::core::ffi::c_int
                {
                    NPCPhrase_cb(
                        C2Rust_Unnamed_2::OK_NO_BUY_NEW_TECH.0 as ::core::ffi::c_int,
                        None,
                    );
                } else if R as ::core::ffi::c_int
                    == C2Rust_Unnamed_2::done_buying_info.0 as ::core::ffi::c_int
                {
                    NPCPhrase_cb(
                        C2Rust_Unnamed_2::OK_DONE_BUYING_INFO.0 as ::core::ffi::c_int,
                        None,
                    );
                } else {
                    NPCPhrase_cb(
                        C2Rust_Unnamed_2::WHAT_TO_BUY.0 as ::core::ffi::c_int,
                        None,
                    );
                }
            }
            if GlobData.SIS_state.FuelOnBoard < capacity {
                DoResponsePhrase(
                    C2Rust_Unnamed_2::buy_fuel.0 as ::core::ffi::c_int as RESPONSE_REF,
                    Some(DoBuy as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                    ::core::ptr::null_mut::<UNICODE>(),
                );
            }
            if !GetNextTechForSale().is_null() {
                DoResponsePhrase(
                    C2Rust_Unnamed_2::buy_technology.0 as ::core::ffi::c_int
                        as RESPONSE_REF,
                    Some(DoBuy as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                    ::core::ptr::null_mut::<UNICODE>(),
                );
            }
            if AnyInfoLeftToSell() {
                DoResponsePhrase(
                    C2Rust_Unnamed_2::buy_info.0 as ::core::ffi::c_int as RESPONSE_REF,
                    Some(DoBuy as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                    ::core::ptr::null_mut::<UNICODE>(),
                );
            }
            DoResponsePhrase(
                C2Rust_Unnamed_2::done_buying.0 as ::core::ffi::c_int as RESPONSE_REF,
                Some(NatureOfConversation as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            );
            DoResponsePhrase(
                C2Rust_Unnamed_2::be_leaving_now.0 as ::core::ffi::c_int as RESPONSE_REF,
                Some(ExitConversation as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            );
        }
    };
}
unsafe extern "C" fn DoSell(mut R: RESPONSE_REF) {
    let mut num_new_rainbows: BYTE = 0;
    let mut rainbow_mask: UWORD = 0;
    let mut added_credit: SIZE = 0;
    let mut what_to_sell_queued: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    rainbow_mask = (((getGameState(
        &raw mut GlobData.Game_state.GameState as *mut BYTE,
        C2Rust_Unnamed_1::RAINBOW_WORLD1.0 as ::core::ffi::c_int,
        C2Rust_Unnamed_1::END_RAINBOW_WORLD1.0 as ::core::ffi::c_int,
    ) as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as UWORD as ::core::ffi::c_int
        | getGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_1::RAINBOW_WORLD0.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_1::END_RAINBOW_WORLD0.0 as ::core::ffi::c_int,
        ) as ::core::ffi::c_int) as UWORD;
    num_new_rainbows = -(getGameState(
        &raw mut GlobData.Game_state.GameState as *mut BYTE,
        C2Rust_Unnamed_1::MELNORME_RAINBOW_COUNT.0 as ::core::ffi::c_int,
        C2Rust_Unnamed_1::END_MELNORME_RAINBOW_COUNT.0 as ::core::ffi::c_int,
    ) as ::core::ffi::c_int) as BYTE;
    while rainbow_mask != 0 {
        if rainbow_mask as ::core::ffi::c_int & 1 as ::core::ffi::c_int != 0 {
            num_new_rainbows = num_new_rainbows.wrapping_add(1);
        }
        rainbow_mask = (rainbow_mask as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
            as UWORD;
    }
    if !(R as ::core::ffi::c_int == C2Rust_Unnamed_2::sell.0 as ::core::ffi::c_int) {
        if R as ::core::ffi::c_int
            == C2Rust_Unnamed_2::sell_life_data.0 as ::core::ffi::c_int
        {
            let mut TimeIn: DWORD = 0;
            added_credit = (GlobData.SIS_state.TotalBioMass as ::core::ffi::c_int
                * 2 as ::core::ffi::c_int) as SIZE;
            NPCPhrase_cb(
                C2Rust_Unnamed_2::SOLD_LIFE_DATA1.0 as ::core::ffi::c_int,
                None,
            );
            NPCNumber(
                GlobData.SIS_state.TotalBioMass as ::core::ffi::c_int,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
            NPCPhrase_cb(
                C2Rust_Unnamed_2::SOLD_LIFE_DATA2.0 as ::core::ffi::c_int,
                None,
            );
            NPCNumber(
                added_credit as ::core::ffi::c_int,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
            NPCPhrase_cb(
                C2Rust_Unnamed_2::SOLD_LIFE_DATA3.0 as ::core::ffi::c_int,
                None,
            );
            if num_new_rainbows != 0 {
                NPCPhrase_cb(
                    C2Rust_Unnamed_2::WHAT_TO_SELL.0 as ::core::ffi::c_int,
                    None,
                );
                what_to_sell_queued = 1 as ::core::ffi::c_int;
            }
            AlienTalkSegue(1 as COUNT);
            DrawCargoStrings(
                !(0 as ::core::ffi::c_int) as BYTE,
                !(0 as ::core::ffi::c_int) as BYTE,
            );
            SleepThread(
                (840 as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as TimePeriod,
            );
            TimeIn = GetTimeCounter() as DWORD;
            DrawCargoStrings(
                C2Rust_Unnamed::NUM_ELEMENT_CATEGORIES.0 as ::core::ffi::c_int as BYTE,
                C2Rust_Unnamed::NUM_ELEMENT_CATEGORIES.0 as ::core::ffi::c_int as BYTE,
            );
            loop {
                TimeIn = GetTimeCounter() as DWORD;
                if AnyButtonPress(BOOLEAN::TRUE).0 != 0 {
                    DeltaCredit(
                        (GlobData.SIS_state.TotalBioMass as ::core::ffi::c_int
                            * 2 as ::core::ffi::c_int) as SIZE,
                    );
                    GlobData.SIS_state.TotalBioMass = 0 as COUNT;
                } else {
                    GlobData.SIS_state.TotalBioMass = GlobData
                        .SIS_state
                        .TotalBioMass
                        .wrapping_sub(1);
                    DeltaCredit(2 as SIZE);
                }
                DrawCargoStrings(
                    C2Rust_Unnamed::NUM_ELEMENT_CATEGORIES.0 as ::core::ffi::c_int
                        as BYTE,
                    C2Rust_Unnamed::NUM_ELEMENT_CATEGORIES.0 as ::core::ffi::c_int
                        as BYTE,
                );
                if GlobData.SIS_state.TotalBioMass == 0 {
                    break;
                }
            }
            SleepThread(
                (840 as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as TimePeriod,
            );
            ClearSISRect(((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as BYTE);
        } else {
            added_credit = (num_new_rainbows as ::core::ffi::c_int
                * (250 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)) as SIZE;
            NPCPhrase_cb(
                C2Rust_Unnamed_2::SOLD_RAINBOW_LOCATIONS1.0 as ::core::ffi::c_int,
                None,
            );
            NPCNumber(
                num_new_rainbows as ::core::ffi::c_int,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
            NPCPhrase_cb(
                C2Rust_Unnamed_2::SOLD_RAINBOW_LOCATIONS2.0 as ::core::ffi::c_int,
                None,
            );
            NPCNumber(
                added_credit as ::core::ffi::c_int,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
            NPCPhrase_cb(
                C2Rust_Unnamed_2::SOLD_RAINBOW_LOCATIONS3.0 as ::core::ffi::c_int,
                None,
            );
            num_new_rainbows = (num_new_rainbows as ::core::ffi::c_int
                + getGameState(
                    &raw mut GlobData.Game_state.GameState as *mut BYTE,
                    C2Rust_Unnamed_1::MELNORME_RAINBOW_COUNT.0 as ::core::ffi::c_int,
                    C2Rust_Unnamed_1::END_MELNORME_RAINBOW_COUNT.0 as ::core::ffi::c_int,
                ) as ::core::ffi::c_int) as BYTE;
            setGameState(
                &raw mut GlobData.Game_state.GameState as *mut BYTE,
                C2Rust_Unnamed_1::MELNORME_RAINBOW_COUNT.0 as ::core::ffi::c_int,
                C2Rust_Unnamed_1::END_MELNORME_RAINBOW_COUNT.0 as ::core::ffi::c_int,
                num_new_rainbows,
            );
            num_new_rainbows = 0 as BYTE;
            DeltaCredit(added_credit);
        }
        AskedToBuy = BOOLEAN::FALSE.0 as ::core::ffi::c_int as BYTE;
    }
    if GlobData.SIS_state.TotalBioMass as ::core::ffi::c_int != 0
        || num_new_rainbows as ::core::ffi::c_int != 0
    {
        if what_to_sell_queued == 0 {
            NPCPhrase_cb(C2Rust_Unnamed_2::WHAT_TO_SELL.0 as ::core::ffi::c_int, None);
        }
        if GlobData.SIS_state.TotalBioMass != 0 {
            DoResponsePhrase(
                C2Rust_Unnamed_2::sell_life_data.0 as ::core::ffi::c_int as RESPONSE_REF,
                Some(DoSell as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            );
        }
        if num_new_rainbows != 0 {
            DoResponsePhrase(
                C2Rust_Unnamed_2::sell_rainbow_locations.0 as ::core::ffi::c_int
                    as RESPONSE_REF,
                Some(DoSell as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            );
        }
        DoResponsePhrase(
            C2Rust_Unnamed_2::done_selling.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(NatureOfConversation as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        );
    } else {
        if R as ::core::ffi::c_int == C2Rust_Unnamed_2::sell.0 as ::core::ffi::c_int {
            NPCPhrase_cb(
                C2Rust_Unnamed_2::NOTHING_TO_SELL.0 as ::core::ffi::c_int,
                None,
            );
        }
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_2::sell.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
        NatureOfConversation(R);
    };
}
unsafe extern "C" fn NatureOfConversation(mut R: RESPONSE_REF) {
    let mut num_new_rainbows: BYTE = 0;
    let mut rainbow_mask: UWORD = 0;
    let mut Credit: COUNT = 0;
    if R as ::core::ffi::c_int
        == C2Rust_Unnamed_2::get_on_with_business.0 as ::core::ffi::c_int
    {
        setGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_1::MELNORME_YACK_STACK2.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_1::END_MELNORME_YACK_STACK2.0 as ::core::ffi::c_int,
            5 as BYTE,
        );
        R = 0 as RESPONSE_REF;
    }
    DeltaCredit(0 as SIZE);
    Credit = GetAvailableCredits();
    if R as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        let mut stack: BYTE = (getGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_1::MELNORME_YACK_STACK2.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_1::END_MELNORME_YACK_STACK2.0 as ::core::ffi::c_int,
        ) as ::core::ffi::c_int - 5 as ::core::ffi::c_int) as BYTE;
        NPCPhrase_cb(
            GetLineSafe(
                &raw const hello_and_down_to_business_lines as *const ::core::ffi::c_int,
                stack as size_t,
            ),
            None,
        );
        if (stack as size_t) < NUM_HELLO_LINES.wrapping_sub(1 as size_t) {
            stack = stack.wrapping_add(1);
        }
        setGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_1::MELNORME_YACK_STACK2.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_1::END_MELNORME_YACK_STACK2.0 as ::core::ffi::c_int,
            (stack as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as BYTE,
        );
    }
    rainbow_mask = (((getGameState(
        &raw mut GlobData.Game_state.GameState as *mut BYTE,
        C2Rust_Unnamed_1::RAINBOW_WORLD1.0 as ::core::ffi::c_int,
        C2Rust_Unnamed_1::END_RAINBOW_WORLD1.0 as ::core::ffi::c_int,
    ) as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as UWORD as ::core::ffi::c_int
        | getGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_1::RAINBOW_WORLD0.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_1::END_RAINBOW_WORLD0.0 as ::core::ffi::c_int,
        ) as ::core::ffi::c_int) as UWORD;
    num_new_rainbows = -(getGameState(
        &raw mut GlobData.Game_state.GameState as *mut BYTE,
        C2Rust_Unnamed_1::MELNORME_RAINBOW_COUNT.0 as ::core::ffi::c_int,
        C2Rust_Unnamed_1::END_MELNORME_RAINBOW_COUNT.0 as ::core::ffi::c_int,
    ) as ::core::ffi::c_int) as BYTE;
    while rainbow_mask != 0 {
        if rainbow_mask as ::core::ffi::c_int & 1 as ::core::ffi::c_int != 0 {
            num_new_rainbows = num_new_rainbows.wrapping_add(1);
        }
        rainbow_mask = (rainbow_mask as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
            as UWORD;
    }
    if GlobData.SIS_state.FuelOnBoard > 0 as DWORD
        || GlobData.SIS_state.TotalBioMass as ::core::ffi::c_int != 0
        || Credit as ::core::ffi::c_int != 0
        || num_new_rainbows as ::core::ffi::c_int != 0
    {
        if getGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_1::TRADED_WITH_MELNORME.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_1::END_TRADED_WITH_MELNORME.0 as ::core::ffi::c_int,
        ) == 0
        {
            setGameState(
                &raw mut GlobData.Game_state.GameState as *mut BYTE,
                C2Rust_Unnamed_1::TRADED_WITH_MELNORME.0 as ::core::ffi::c_int,
                C2Rust_Unnamed_1::END_TRADED_WITH_MELNORME.0 as ::core::ffi::c_int,
                1 as BYTE,
            );
            NPCPhrase_cb(C2Rust_Unnamed_2::TRADING_INFO.0 as ::core::ffi::c_int, None);
        }
        if R as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            NPCPhrase_cb(C2Rust_Unnamed_2::BUY_OR_SELL.0 as ::core::ffi::c_int, None);
            AlienTalkSegue(1 as COUNT);
            XFormColorMap(
                GetColorMapAddress(
                    SetAbsStringTableIndex(CommData.AlienColorMap, 1 as COUNT),
                ),
                (840 as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as SIZE,
            );
            AlienTalkSegue(!(0 as ::core::ffi::c_int) as COUNT);
        } else if R as ::core::ffi::c_int
            == C2Rust_Unnamed_2::why_turned_purple.0 as ::core::ffi::c_int
        {
            setGameState(
                &raw mut GlobData.Game_state.GameState as *mut BYTE,
                C2Rust_Unnamed_1::WHY_MELNORME_PURPLE.0 as ::core::ffi::c_int,
                C2Rust_Unnamed_1::END_WHY_MELNORME_PURPLE.0 as ::core::ffi::c_int,
                1 as BYTE,
            );
            NPCPhrase_cb(
                C2Rust_Unnamed_2::TURNED_PURPLE_BECAUSE.0 as ::core::ffi::c_int,
                None,
            );
        } else if R as ::core::ffi::c_int
            == C2Rust_Unnamed_2::done_selling.0 as ::core::ffi::c_int
        {
            NPCPhrase_cb(
                C2Rust_Unnamed_2::OK_DONE_SELLING.0 as ::core::ffi::c_int,
                None,
            );
        } else if R as ::core::ffi::c_int
            == C2Rust_Unnamed_2::done_buying.0 as ::core::ffi::c_int
        {
            NPCPhrase_cb(C2Rust_Unnamed_2::OK_DONE_BUYING.0 as ::core::ffi::c_int, None);
        }
        if getGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_1::WHY_MELNORME_PURPLE.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_1::END_WHY_MELNORME_PURPLE.0 as ::core::ffi::c_int,
        ) == 0
        {
            DoResponsePhrase(
                C2Rust_Unnamed_2::why_turned_purple.0 as ::core::ffi::c_int
                    as RESPONSE_REF,
                Some(NatureOfConversation as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            );
        }
        if AskedToBuy == 0 {
            DoResponsePhrase(
                C2Rust_Unnamed_2::buy.0 as ::core::ffi::c_int as RESPONSE_REF,
                Some(DoBuy as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            );
        }
        if *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_2::sell.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        {
            DoResponsePhrase(
                C2Rust_Unnamed_2::sell.0 as ::core::ffi::c_int as RESPONSE_REF,
                Some(DoSell as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            );
        }
        DoResponsePhrase(
            C2Rust_Unnamed_2::goodbye.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(ExitConversation as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        );
    } else if getGameState(
        &raw mut GlobData.Game_state.GameState as *mut BYTE,
        C2Rust_Unnamed_1::MELNORME_RESCUE_REFUSED.0 as ::core::ffi::c_int,
        C2Rust_Unnamed_1::END_MELNORME_RESCUE_REFUSED.0 as ::core::ffi::c_int,
    ) != 0
    {
        NPCPhrase_cb(C2Rust_Unnamed_2::CHANGED_MIND.0 as ::core::ffi::c_int, None);
        DoResponsePhrase(
            C2Rust_Unnamed_2::yes_changed_mind.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(DoRescue as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        );
        DoResponsePhrase(
            C2Rust_Unnamed_2::no_changed_mind.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(ExitConversation as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        );
    } else {
        let mut num_rescues: BYTE = getGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_1::MELNORME_RESCUE_COUNT.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_1::END_MELNORME_RESCUE_COUNT.0 as ::core::ffi::c_int,
        );
        NPCPhrase_cb(
            GetLineSafe(
                &raw const rescue_lines as *const ::core::ffi::c_int,
                num_rescues as size_t,
            ),
            None,
        );
        if (num_rescues as size_t) < NUM_RESCUE_LINES.wrapping_sub(1 as size_t) {
            num_rescues = num_rescues.wrapping_add(1);
            setGameState(
                &raw mut GlobData.Game_state.GameState as *mut BYTE,
                C2Rust_Unnamed_1::MELNORME_RESCUE_COUNT.0 as ::core::ffi::c_int,
                C2Rust_Unnamed_1::END_MELNORME_RESCUE_COUNT.0 as ::core::ffi::c_int,
                num_rescues,
            );
        }
        NPCPhrase_cb(C2Rust_Unnamed_2::SHOULD_WE_HELP_YOU.0 as ::core::ffi::c_int, None);
        DoResponsePhrase(
            C2Rust_Unnamed_2::yes_help.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(DoRescue as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        );
        DoResponsePhrase(
            C2Rust_Unnamed_2::no_help.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(ExitConversation as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        );
    };
}
static mut local_stack1: BYTE = 0;
static mut local_stack0: BYTE = 0;
unsafe extern "C" fn DoBluster(mut R: RESPONSE_REF) {
    if R as ::core::ffi::c_int
        == C2Rust_Unnamed_2::trade_is_for_the_weak.0 as ::core::ffi::c_int
    {
        XFormColorMap(
            GetColorMapAddress(
                SetAbsStringTableIndex(CommData.AlienColorMap, 2 as COUNT),
            ),
            (840 as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as SIZE,
        );
        setGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_1::MELNORME_YACK_STACK2.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_1::END_MELNORME_YACK_STACK2.0 as ::core::ffi::c_int,
            4 as BYTE,
        );
        NPCPhrase_cb(C2Rust_Unnamed_2::WERE_NOT_AFRAID.0 as ::core::ffi::c_int, None);
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_2::why_blue_light.0 as ::core::ffi::c_int
    {
        setGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_1::WHY_MELNORME_BLUE.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_1::END_WHY_MELNORME_BLUE.0 as ::core::ffi::c_int,
            1 as BYTE,
        );
        NPCPhrase_cb(C2Rust_Unnamed_2::BLUE_IS_MAD.0 as ::core::ffi::c_int, None);
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_2::we_strong_1.0 as ::core::ffi::c_int
    {
        local_stack0 = 1 as BYTE;
        NPCPhrase_cb(C2Rust_Unnamed_2::YOU_NOT_STRONG_1.0 as ::core::ffi::c_int, None);
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_2::we_strong_2.0 as ::core::ffi::c_int
    {
        local_stack0 = 2 as BYTE;
        NPCPhrase_cb(C2Rust_Unnamed_2::YOU_NOT_STRONG_2.0 as ::core::ffi::c_int, None);
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_2::we_strong_3.0 as ::core::ffi::c_int
    {
        local_stack0 = 3 as BYTE;
        NPCPhrase_cb(C2Rust_Unnamed_2::YOU_NOT_STRONG_3.0 as ::core::ffi::c_int, None);
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_2::just_testing.0 as ::core::ffi::c_int
    {
        local_stack1 = 1 as BYTE;
        NPCPhrase_cb(C2Rust_Unnamed_2::REALLY_TESTING.0 as ::core::ffi::c_int, None);
    }
    if getGameState(
        &raw mut GlobData.Game_state.GameState as *mut BYTE,
        C2Rust_Unnamed_1::WHY_MELNORME_BLUE.0 as ::core::ffi::c_int,
        C2Rust_Unnamed_1::END_WHY_MELNORME_BLUE.0 as ::core::ffi::c_int,
    ) == 0
    {
        DoResponsePhrase(
            C2Rust_Unnamed_2::why_blue_light.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(DoBluster as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        );
    }
    match local_stack0 as ::core::ffi::c_int {
        0 => {
            DoResponsePhrase(
                C2Rust_Unnamed_2::we_strong_1.0 as ::core::ffi::c_int as RESPONSE_REF,
                Some(DoBluster as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            );
        }
        1 => {
            DoResponsePhrase(
                C2Rust_Unnamed_2::we_strong_2.0 as ::core::ffi::c_int as RESPONSE_REF,
                Some(DoBluster as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            );
        }
        2 => {
            DoResponsePhrase(
                C2Rust_Unnamed_2::we_strong_3.0 as ::core::ffi::c_int as RESPONSE_REF,
                Some(DoBluster as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            );
        }
        _ => {}
    }
    match local_stack1 as ::core::ffi::c_int {
        0 => {
            DoResponsePhrase(
                C2Rust_Unnamed_2::just_testing.0 as ::core::ffi::c_int as RESPONSE_REF,
                Some(DoBluster as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            );
        }
        1 => {
            DoResponsePhrase(
                C2Rust_Unnamed_2::yes_really_testing.0 as ::core::ffi::c_int
                    as RESPONSE_REF,
                Some(DoFirstMeeting as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            );
        }
        _ => {}
    }
    DoResponsePhrase(
        C2Rust_Unnamed_2::youre_on.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(ExitConversation as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    );
}
unsafe extern "C" fn yack0_respond() {
    match getGameState(
        &raw mut GlobData.Game_state.GameState as *mut BYTE,
        C2Rust_Unnamed_1::MELNORME_YACK_STACK0.0 as ::core::ffi::c_int,
        C2Rust_Unnamed_1::END_MELNORME_YACK_STACK0.0 as ::core::ffi::c_int,
    ) as ::core::ffi::c_int
    {
        0 => {
            let mut buf: [UNICODE; 256] = [0; 256];
            GetAllianceName(
                &raw mut buf as *mut UNICODE,
                C2Rust_Unnamed_2::name_1.0 as ::core::ffi::c_int as RESPONSE_REF,
            );
            construct_response(
                &raw mut shared_phrase_buf as *mut UNICODE,
                C2Rust_Unnamed_2::we_are_from_alliance0.0 as ::core::ffi::c_int,
                &raw mut buf as *mut UNICODE,
                -1 as ::core::ffi::c_int as RESPONSE_REF as ::core::ffi::c_int,
            );
            DoResponsePhrase(
                C2Rust_Unnamed_2::we_are_from_alliance0.0 as ::core::ffi::c_int
                    as RESPONSE_REF,
                Some(DoFirstMeeting as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                &raw mut shared_phrase_buf as *mut UNICODE,
            );
        }
        1 => {
            DoResponsePhrase(
                C2Rust_Unnamed_2::how_know.0 as ::core::ffi::c_int as RESPONSE_REF,
                Some(DoFirstMeeting as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            );
        }
        _ => {}
    };
}
unsafe extern "C" fn yack1_respond() {
    's_23: {
        match getGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_1::MELNORME_YACK_STACK1.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_1::END_MELNORME_YACK_STACK1.0 as ::core::ffi::c_int,
        ) as ::core::ffi::c_int
        {
            0 => {
                DoResponsePhrase(
                    C2Rust_Unnamed_2::what_about_yourselves.0 as ::core::ffi::c_int
                        as RESPONSE_REF,
                    Some(DoFirstMeeting as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                    ::core::ptr::null_mut::<UNICODE>(),
                );
                break 's_23;
            }
            1 => {
                DoResponsePhrase(
                    C2Rust_Unnamed_2::what_factors.0 as ::core::ffi::c_int
                        as RESPONSE_REF,
                    Some(DoFirstMeeting as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                    ::core::ptr::null_mut::<UNICODE>(),
                );
            }
            2 => {}
            _ => {
                break 's_23;
            }
        }
        DoResponsePhrase(
            C2Rust_Unnamed_2::get_on_with_business.0 as ::core::ffi::c_int
                as RESPONSE_REF,
            Some(NatureOfConversation as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        );
    };
}
unsafe extern "C" fn yack2_respond() {
    match getGameState(
        &raw mut GlobData.Game_state.GameState as *mut BYTE,
        C2Rust_Unnamed_1::MELNORME_YACK_STACK2.0 as ::core::ffi::c_int,
        C2Rust_Unnamed_1::END_MELNORME_YACK_STACK2.0 as ::core::ffi::c_int,
    ) as ::core::ffi::c_int
    {
        0 => {
            DoResponsePhrase(
                C2Rust_Unnamed_2::what_about_universe.0 as ::core::ffi::c_int
                    as RESPONSE_REF,
                Some(DoFirstMeeting as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            );
        }
        1 => {
            DoResponsePhrase(
                C2Rust_Unnamed_2::giving_is_good_1.0 as ::core::ffi::c_int
                    as RESPONSE_REF,
                Some(DoFirstMeeting as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            );
        }
        2 => {
            DoResponsePhrase(
                C2Rust_Unnamed_2::giving_is_good_2.0 as ::core::ffi::c_int
                    as RESPONSE_REF,
                Some(DoFirstMeeting as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            );
        }
        3 => {
            DoResponsePhrase(
                C2Rust_Unnamed_2::trade_is_for_the_weak.0 as ::core::ffi::c_int
                    as RESPONSE_REF,
                Some(DoBluster as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            );
        }
        _ => {}
    };
}
unsafe extern "C" fn DoFirstMeeting(mut R: RESPONSE_REF) {
    let mut last_stack: BYTE = 0 as BYTE;
    let mut temp_func: PVOIDFUNC = None;
    let mut stack_func: [PVOIDFUNC; 3] = [
        Some(yack0_respond as unsafe extern "C" fn() -> ()),
        Some(yack1_respond as unsafe extern "C" fn() -> ()),
        Some(yack2_respond as unsafe extern "C" fn() -> ()),
    ];
    if R as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        let mut business_count: BYTE = 0;
        business_count = getGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_1::MELNORME_BUSINESS_COUNT.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_1::END_MELNORME_BUSINESS_COUNT.0 as ::core::ffi::c_int,
        );
        let c2rust_fresh4 = business_count;
        business_count = business_count.wrapping_add(1);
        match c2rust_fresh4 as ::core::ffi::c_int {
            0 => {
                NPCPhrase_cb(
                    C2Rust_Unnamed_2::HELLO_NOW_DOWN_TO_BUSINESS_1.0
                        as ::core::ffi::c_int,
                    None,
                );
            }
            1 => {
                NPCPhrase_cb(
                    C2Rust_Unnamed_2::HELLO_NOW_DOWN_TO_BUSINESS_2.0
                        as ::core::ffi::c_int,
                    None,
                );
            }
            2 => {
                NPCPhrase_cb(
                    C2Rust_Unnamed_2::HELLO_NOW_DOWN_TO_BUSINESS_3.0
                        as ::core::ffi::c_int,
                    None,
                );
                business_count = business_count.wrapping_sub(1);
            }
            _ => {}
        }
        setGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_1::MELNORME_BUSINESS_COUNT.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_1::END_MELNORME_BUSINESS_COUNT.0 as ::core::ffi::c_int,
            business_count,
        );
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_2::we_are_from_alliance0.0 as ::core::ffi::c_int
    {
        setGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_1::MELNORME_YACK_STACK0.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_1::END_MELNORME_YACK_STACK0.0 as ::core::ffi::c_int,
            1 as BYTE,
        );
        NPCPhrase_cb(C2Rust_Unnamed_2::KNOW_OF_YOU.0 as ::core::ffi::c_int, None);
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_2::how_know.0 as ::core::ffi::c_int
    {
        setGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_1::MELNORME_YACK_STACK0.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_1::END_MELNORME_YACK_STACK0.0 as ::core::ffi::c_int,
            2 as BYTE,
        );
        NPCPhrase_cb(C2Rust_Unnamed_2::KNOW_BECAUSE.0 as ::core::ffi::c_int, None);
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_2::what_about_yourselves.0 as ::core::ffi::c_int
    {
        last_stack = 1 as BYTE;
        setGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_1::MELNORME_YACK_STACK1.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_1::END_MELNORME_YACK_STACK1.0 as ::core::ffi::c_int,
            1 as BYTE,
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_2::NO_TALK_ABOUT_OURSELVES.0 as ::core::ffi::c_int,
            None,
        );
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_2::what_factors.0 as ::core::ffi::c_int
    {
        last_stack = 1 as BYTE;
        setGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_1::MELNORME_YACK_STACK1.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_1::END_MELNORME_YACK_STACK1.0 as ::core::ffi::c_int,
            2 as BYTE,
        );
        NPCPhrase_cb(C2Rust_Unnamed_2::FACTORS_ARE.0 as ::core::ffi::c_int, None);
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_2::what_about_universe.0 as ::core::ffi::c_int
    {
        last_stack = 2 as BYTE;
        setGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_1::MELNORME_YACK_STACK2.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_1::END_MELNORME_YACK_STACK2.0 as ::core::ffi::c_int,
            1 as BYTE,
        );
        NPCPhrase_cb(C2Rust_Unnamed_2::NO_FREE_LUNCH.0 as ::core::ffi::c_int, None);
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_2::giving_is_good_1.0 as ::core::ffi::c_int
    {
        last_stack = 2 as BYTE;
        setGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_1::MELNORME_YACK_STACK2.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_1::END_MELNORME_YACK_STACK2.0 as ::core::ffi::c_int,
            2 as BYTE,
        );
        NPCPhrase_cb(C2Rust_Unnamed_2::GIVING_IS_BAD_1.0 as ::core::ffi::c_int, None);
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_2::giving_is_good_2.0 as ::core::ffi::c_int
    {
        last_stack = 2 as BYTE;
        setGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_1::MELNORME_YACK_STACK2.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_1::END_MELNORME_YACK_STACK2.0 as ::core::ffi::c_int,
            3 as BYTE,
        );
        NPCPhrase_cb(C2Rust_Unnamed_2::GIVING_IS_BAD_2.0 as ::core::ffi::c_int, None);
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_2::yes_really_testing.0 as ::core::ffi::c_int
    {
        XFormColorMap(
            GetColorMapAddress(
                SetAbsStringTableIndex(CommData.AlienColorMap, 0 as COUNT),
            ),
            (840 as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as SIZE,
        );
        NPCPhrase_cb(C2Rust_Unnamed_2::TEST_RESULTS.0 as ::core::ffi::c_int, None);
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_2::we_apologize.0 as ::core::ffi::c_int
    {
        setGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_1::MELNORME_ANGER.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_1::END_MELNORME_ANGER.0 as ::core::ffi::c_int,
            0 as BYTE,
        );
        XFormColorMap(
            GetColorMapAddress(
                SetAbsStringTableIndex(CommData.AlienColorMap, 0 as COUNT),
            ),
            (840 as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as SIZE,
        );
        NPCPhrase_cb(C2Rust_Unnamed_2::APOLOGY_ACCEPTED.0 as ::core::ffi::c_int, None);
    }
    temp_func = stack_func[0usize];
    stack_func[0usize] = stack_func[last_stack as usize];
    stack_func[last_stack as usize] = temp_func;
    Some(stack_func[0usize].expect("non-null function pointer"))
        .expect("non-null function pointer")();
    Some(stack_func[1usize].expect("non-null function pointer"))
        .expect("non-null function pointer")();
    Some(stack_func[2usize].expect("non-null function pointer"))
        .expect("non-null function pointer")();
    DoResponsePhrase(
        C2Rust_Unnamed_2::no_trade_now.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(ExitConversation as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    );
}
unsafe extern "C" fn DoMelnormeMiffed(mut R: RESPONSE_REF) {
    if R as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        let mut miffed_count: BYTE = 0;
        miffed_count = getGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_1::MELNORME_MIFFED_COUNT.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_1::END_MELNORME_MIFFED_COUNT.0 as ::core::ffi::c_int,
        );
        let c2rust_fresh3 = miffed_count;
        miffed_count = miffed_count.wrapping_add(1);
        match c2rust_fresh3 as ::core::ffi::c_int {
            0 => {
                NPCPhrase_cb(
                    C2Rust_Unnamed_2::HELLO_SLIGHTLY_ANGRY_1.0 as ::core::ffi::c_int,
                    None,
                );
            }
            1 => {
                NPCPhrase_cb(
                    C2Rust_Unnamed_2::HELLO_SLIGHTLY_ANGRY_2.0 as ::core::ffi::c_int,
                    None,
                );
            }
            _ => {
                miffed_count = miffed_count.wrapping_sub(1);
                NPCPhrase_cb(
                    C2Rust_Unnamed_2::HELLO_SLIGHTLY_ANGRY_3.0 as ::core::ffi::c_int,
                    None,
                );
            }
        }
        setGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_1::MELNORME_MIFFED_COUNT.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_1::END_MELNORME_MIFFED_COUNT.0 as ::core::ffi::c_int,
            miffed_count,
        );
        XFormColorMap(
            GetColorMapAddress(
                SetAbsStringTableIndex(CommData.AlienColorMap, 2 as COUNT),
            ),
            (840 as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as SIZE,
        );
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_2::explore_relationship.0 as ::core::ffi::c_int
    {
        setGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_1::MELNORME_YACK_STACK3.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_1::END_MELNORME_YACK_STACK3.0 as ::core::ffi::c_int,
            1 as BYTE,
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_2::EXAMPLE_OF_RELATIONSHIP.0 as ::core::ffi::c_int,
            None,
        );
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_2::excuse_1.0 as ::core::ffi::c_int
    {
        setGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_1::MELNORME_YACK_STACK3.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_1::END_MELNORME_YACK_STACK3.0 as ::core::ffi::c_int,
            2 as BYTE,
        );
        NPCPhrase_cb(C2Rust_Unnamed_2::NO_EXCUSE_1.0 as ::core::ffi::c_int, None);
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_2::excuse_2.0 as ::core::ffi::c_int
    {
        setGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_1::MELNORME_YACK_STACK3.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_1::END_MELNORME_YACK_STACK3.0 as ::core::ffi::c_int,
            3 as BYTE,
        );
        NPCPhrase_cb(C2Rust_Unnamed_2::NO_EXCUSE_2.0 as ::core::ffi::c_int, None);
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_2::excuse_3.0 as ::core::ffi::c_int
    {
        setGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_1::MELNORME_YACK_STACK3.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_1::END_MELNORME_YACK_STACK3.0 as ::core::ffi::c_int,
            4 as BYTE,
        );
        NPCPhrase_cb(C2Rust_Unnamed_2::NO_EXCUSE_3.0 as ::core::ffi::c_int, None);
    }
    match getGameState(
        &raw mut GlobData.Game_state.GameState as *mut BYTE,
        C2Rust_Unnamed_1::MELNORME_YACK_STACK3.0 as ::core::ffi::c_int,
        C2Rust_Unnamed_1::END_MELNORME_YACK_STACK3.0 as ::core::ffi::c_int,
    ) as ::core::ffi::c_int
    {
        0 => {
            DoResponsePhrase(
                C2Rust_Unnamed_2::explore_relationship.0 as ::core::ffi::c_int
                    as RESPONSE_REF,
                Some(DoMelnormeMiffed as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            );
        }
        1 => {
            DoResponsePhrase(
                C2Rust_Unnamed_2::excuse_1.0 as ::core::ffi::c_int as RESPONSE_REF,
                Some(DoMelnormeMiffed as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            );
        }
        2 => {
            DoResponsePhrase(
                C2Rust_Unnamed_2::excuse_2.0 as ::core::ffi::c_int as RESPONSE_REF,
                Some(DoMelnormeMiffed as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            );
        }
        3 => {
            DoResponsePhrase(
                C2Rust_Unnamed_2::excuse_3.0 as ::core::ffi::c_int as RESPONSE_REF,
                Some(DoMelnormeMiffed as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            );
        }
        _ => {}
    }
    DoResponsePhrase(
        C2Rust_Unnamed_2::we_apologize.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(DoFirstMeeting as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    );
    DoResponsePhrase(
        C2Rust_Unnamed_2::so_we_can_attack.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(ExitConversation as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    );
    DoResponsePhrase(
        C2Rust_Unnamed_2::bye_melnorme_slightly_angry.0 as ::core::ffi::c_int
            as RESPONSE_REF,
        Some(ExitConversation as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    );
}
unsafe extern "C" fn DoMelnormePissed(mut R: RESPONSE_REF) {
    if R as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        let mut pissed_count: BYTE = 0;
        pissed_count = getGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_1::MELNORME_PISSED_COUNT.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_1::END_MELNORME_PISSED_COUNT.0 as ::core::ffi::c_int,
        );
        let c2rust_fresh2 = pissed_count;
        pissed_count = pissed_count.wrapping_add(1);
        match c2rust_fresh2 as ::core::ffi::c_int {
            0 => {
                NPCPhrase_cb(
                    C2Rust_Unnamed_2::HELLO_PISSED_OFF_1.0 as ::core::ffi::c_int,
                    None,
                );
            }
            1 => {
                NPCPhrase_cb(
                    C2Rust_Unnamed_2::HELLO_PISSED_OFF_2.0 as ::core::ffi::c_int,
                    None,
                );
            }
            _ => {
                pissed_count = pissed_count.wrapping_sub(1);
                NPCPhrase_cb(
                    C2Rust_Unnamed_2::HELLO_PISSED_OFF_3.0 as ::core::ffi::c_int,
                    None,
                );
            }
        }
        setGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_1::MELNORME_PISSED_COUNT.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_1::END_MELNORME_PISSED_COUNT.0 as ::core::ffi::c_int,
            pissed_count,
        );
        XFormColorMap(
            GetColorMapAddress(
                SetAbsStringTableIndex(CommData.AlienColorMap, 2 as COUNT),
            ),
            (840 as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as SIZE,
        );
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_2::beg_forgiveness.0 as ::core::ffi::c_int
    {
        setGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_1::MELNORME_YACK_STACK4.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_1::END_MELNORME_YACK_STACK4.0 as ::core::ffi::c_int,
            1 as BYTE,
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_2::LOTS_TO_MAKE_UP_FOR.0 as ::core::ffi::c_int,
            None,
        );
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_2::you_are_so_right.0 as ::core::ffi::c_int
    {
        setGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_1::MELNORME_YACK_STACK4.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_1::END_MELNORME_YACK_STACK4.0 as ::core::ffi::c_int,
            2 as BYTE,
        );
        NPCPhrase_cb(C2Rust_Unnamed_2::ONE_LAST_CHANCE.0 as ::core::ffi::c_int, None);
    }
    match getGameState(
        &raw mut GlobData.Game_state.GameState as *mut BYTE,
        C2Rust_Unnamed_1::MELNORME_YACK_STACK4.0 as ::core::ffi::c_int,
        C2Rust_Unnamed_1::END_MELNORME_YACK_STACK4.0 as ::core::ffi::c_int,
    ) as ::core::ffi::c_int
    {
        0 => {
            DoResponsePhrase(
                C2Rust_Unnamed_2::beg_forgiveness.0 as ::core::ffi::c_int
                    as RESPONSE_REF,
                Some(DoMelnormePissed as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            );
        }
        1 => {
            DoResponsePhrase(
                C2Rust_Unnamed_2::you_are_so_right.0 as ::core::ffi::c_int
                    as RESPONSE_REF,
                Some(DoMelnormePissed as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            );
        }
        2 => {
            DoResponsePhrase(
                C2Rust_Unnamed_2::ok_strip_me.0 as ::core::ffi::c_int as RESPONSE_REF,
                Some(ExitConversation as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            );
        }
        _ => {}
    }
    DoResponsePhrase(
        C2Rust_Unnamed_2::fight_some_more.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(ExitConversation as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    );
    DoResponsePhrase(
        C2Rust_Unnamed_2::bye_melnorme_pissed_off.0 as ::core::ffi::c_int
            as RESPONSE_REF,
        Some(ExitConversation as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    );
}
unsafe extern "C" fn DoMelnormeHate(mut R: RESPONSE_REF) {
    let mut hate_count: BYTE = 0;
    hate_count = getGameState(
        &raw mut GlobData.Game_state.GameState as *mut BYTE,
        C2Rust_Unnamed_1::MELNORME_HATE_COUNT.0 as ::core::ffi::c_int,
        C2Rust_Unnamed_1::END_MELNORME_HATE_COUNT.0 as ::core::ffi::c_int,
    );
    let c2rust_fresh0 = hate_count;
    hate_count = hate_count.wrapping_add(1);
    match c2rust_fresh0 as ::core::ffi::c_int {
        0 => {
            NPCPhrase_cb(
                C2Rust_Unnamed_2::HELLO_HATE_YOU_1.0 as ::core::ffi::c_int,
                None,
            );
        }
        1 => {
            NPCPhrase_cb(
                C2Rust_Unnamed_2::HELLO_HATE_YOU_2.0 as ::core::ffi::c_int,
                None,
            );
        }
        _ => {
            hate_count = hate_count.wrapping_sub(1);
            NPCPhrase_cb(
                C2Rust_Unnamed_2::HELLO_HATE_YOU_3.0 as ::core::ffi::c_int,
                None,
            );
        }
    }
    setGameState(
        &raw mut GlobData.Game_state.GameState as *mut BYTE,
        C2Rust_Unnamed_1::MELNORME_HATE_COUNT.0 as ::core::ffi::c_int,
        C2Rust_Unnamed_1::END_MELNORME_HATE_COUNT.0 as ::core::ffi::c_int,
        hate_count,
    );
    XFormColorMap(
        GetColorMapAddress(SetAbsStringTableIndex(CommData.AlienColorMap, 2 as COUNT)),
        (840 as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as SIZE,
    );
    DoResponsePhrase(
        C2Rust_Unnamed_2::well_if_thats_the_way_you_feel.0 as ::core::ffi::c_int
            as RESPONSE_REF,
        Some(ExitConversation as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    );
    DoResponsePhrase(
        C2Rust_Unnamed_2::you_hate_us_so_we_go_away.0 as ::core::ffi::c_int
            as RESPONSE_REF,
        Some(ExitConversation as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    );
}
unsafe extern "C" fn Intro() {
    prevMsgMode = SetStatusMessageMode(StatMsgMode::SMM_CREDITS);
    if getGameState(
        &raw mut GlobData.Game_state.GameState as *mut BYTE,
        C2Rust_Unnamed_1::MET_MELNORME.0 as ::core::ffi::c_int,
        C2Rust_Unnamed_1::END_MET_MELNORME.0 as ::core::ffi::c_int,
    ) as ::core::ffi::c_int == 0 as ::core::ffi::c_int
    {
        setGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_1::MET_MELNORME.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_1::END_MET_MELNORME.0 as ::core::ffi::c_int,
            1 as BYTE,
        );
        DoFirstMeeting(0 as RESPONSE_REF);
    } else {
        match getGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_1::MELNORME_ANGER.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_1::END_MELNORME_ANGER.0 as ::core::ffi::c_int,
        ) as ::core::ffi::c_int
        {
            0 => {
                if getGameState(
                    &raw mut GlobData.Game_state.GameState as *mut BYTE,
                    C2Rust_Unnamed_1::MELNORME_YACK_STACK2.0 as ::core::ffi::c_int,
                    C2Rust_Unnamed_1::END_MELNORME_YACK_STACK2.0 as ::core::ffi::c_int,
                ) as ::core::ffi::c_int <= 5 as ::core::ffi::c_int
                {
                    DoFirstMeeting(0 as RESPONSE_REF);
                } else {
                    NatureOfConversation(0 as RESPONSE_REF);
                }
            }
            1 => {
                DoMelnormeMiffed(0 as RESPONSE_REF);
            }
            2 => {
                DoMelnormePissed(0 as RESPONSE_REF);
            }
            _ => {
                DoMelnormeHate(0 as RESPONSE_REF);
            }
        }
    };
}
unsafe extern "C" fn uninit_melnorme() -> COUNT {
    return 0 as COUNT;
}
unsafe extern "C" fn post_melnorme_enc() {
    if prevMsgMode.0 != StatMsgMode::SMM_UNDEFINED.0 {
        SetStatusMessageMode(prevMsgMode);
    }
    DrawStatusMessage(::core::ptr::null::<UNICODE>());
}
#[no_mangle]
pub unsafe extern "C" fn init_melnorme_comm() -> *mut LOCDATA {
    let mut retval: *mut LOCDATA = ::core::ptr::null_mut::<LOCDATA>();
    melnorme_desc.init_encounter_func = Some(Intro as unsafe extern "C" fn() -> ())
        as Option<unsafe extern "C" fn() -> ()>;
    melnorme_desc.post_encounter_func = Some(
        post_melnorme_enc as unsafe extern "C" fn() -> (),
    ) as Option<unsafe extern "C" fn() -> ()>;
    melnorme_desc.uninit_encounter_func = Some(
        uninit_melnorme as unsafe extern "C" fn() -> COUNT,
    ) as Option<unsafe extern "C" fn() -> COUNT>;
    melnorme_desc.AlienTextBaseline.x = (1 as ::core::ffi::c_int
        + (ScreenWidth - 64 as ::core::ffi::c_int
            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
            - 14 as ::core::ffi::c_int
            - ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
            >> 1 as ::core::ffi::c_int)) as COORD;
    melnorme_desc.AlienTextBaseline.y = 0 as COORD;
    melnorme_desc.AlienTextWidth = (ScreenWidth - 64 as ::core::ffi::c_int
        - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int - 14 as ::core::ffi::c_int
        - ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
        - 16 as ::core::ffi::c_int) as COUNT;
    local_stack0 = 0 as BYTE;
    local_stack1 = 0 as BYTE;
    prevMsgMode = StatMsgMode::SMM_UNDEFINED;
    setSegue(Segue::Segue_peace);
    AskedToBuy = BOOLEAN::FALSE.0 as ::core::ffi::c_int as BYTE;
    retval = &raw mut melnorme_desc;
    return retval;
}
#[export_name = "uqm_c2rust_init_c0a05c6c2fe9d518"]
unsafe extern "C" fn c2rust_run_static_initializers() {
    NUM_RESCUE_LINES = (::core::mem::size_of::<[::core::ffi::c_int; 6]>() as size_t)
        .wrapping_div(::core::mem::size_of::<::core::ffi::c_int>() as size_t);
    NUM_HELLO_LINES = (::core::mem::size_of::<[::core::ffi::c_int; 10]>() as size_t)
        .wrapping_div(::core::mem::size_of::<::core::ffi::c_int>() as size_t);
    NUM_HISTORY_ITEMS = (::core::mem::size_of::<[::core::ffi::c_int; 9]>() as size_t)
        .wrapping_div(::core::mem::size_of::<::core::ffi::c_int>() as size_t);
    NUM_ALIEN_RACE_ITEMS = (::core::mem::size_of::<[::core::ffi::c_int; 16]>() as size_t)
        .wrapping_div(::core::mem::size_of::<::core::ffi::c_int>() as size_t);
    NUM_EVENT_ITEMS = (::core::mem::size_of::<[::core::ffi::c_int; 8]>() as size_t)
        .wrapping_div(::core::mem::size_of::<::core::ffi::c_int>() as size_t);
    NUM_TECHNOLOGIES = (::core::mem::size_of::<[TechData; 13]>() as size_t)
        .wrapping_div(::core::mem::size_of::<TechData>() as size_t);
    NUM_TECH_ITEMS = (::core::mem::size_of::<[TechSaleData; 13]>() as size_t)
        .wrapping_div(::core::mem::size_of::<TechSaleData>() as size_t);
}
// Explicit engine initialization replaces platform constructor tables.
