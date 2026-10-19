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

// Rust translation of src/uqm/comm/starbas/starbas.c.
extern "C" {
    pub type string_table_entry;
    pub type frame_desc;
    pub type font_desc;
    pub type tfb_soundsample;
    fn GetTimeCounter() -> TimeCount;
    fn SleepThread(timePeriod: TimePeriod);
    fn SleepThreadUntil(wakeTime: TimeCount);
    fn TaskSwitch();
    fn ClearSISRect(ClearFlags: BYTE);
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
    fn getGameState32(state: *mut BYTE, startBit: ::core::ffi::c_int) -> DWORD;
    fn setGameState32(state: *mut BYTE, startBit: ::core::ffi::c_int, val: DWORD);
    static mut shared_phrase_buf: [UNICODE; 2048];
    fn DoResponsePhrase(
        R: RESPONSE_REF,
        response_func: RESPONSE_FUNC,
        ContstructStr: *mut UNICODE,
    );
    fn NPCPhrase_cb(index: ::core::ffi::c_int, cb: CallbackFunction);
    fn construct_response(buf: *mut UNICODE, R: ::core::ffi::c_int, ...);
    fn setSegue(segue: Segue);
    fn AlienTalkSegue(wait_track: COUNT);
    fn EnableTalkingAnim(enable: BOOLEAN);
    fn AddEscortShips(race: COUNT, count: SIZE) -> COUNT;
    fn CalculateEscortsWorth() -> COUNT;
    fn EscortFeasibilityStudy(race: COUNT) -> COUNT;
    fn CheckAlliance(race: COUNT) -> COUNT;
    static mut usingSpeech: BOOLEAN;
    fn DrawCargoStrings(OldElement: BYTE, NewElement: BYTE);
    fn ShowRemainingCapacity();
    fn TFB_Random() -> DWORD;
    fn AnyButtonPress(DetectSpecial: BOOLEAN) -> BOOLEAN;
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
    pub const PORTAL_SPAWNER_DEVICE: Self = Self(0);
    pub const TALKING_PET_DEVICE: Self = Self(1);
    pub const UTWIG_BOMB_DEVICE: Self = Self(2);
    pub const SUN_EFFICIENCY_DEVICE: Self = Self(3);
    pub const ROSY_SPHERE_DEVICE: Self = Self(4);
    pub const AQUA_HELIX_DEVICE: Self = Self(5);
    pub const CLEAR_SPINDLE_DEVICE: Self = Self(6);
    pub const ULTRON_0_DEVICE: Self = Self(7);
    pub const ULTRON_1_DEVICE: Self = Self(8);
    pub const ULTRON_2_DEVICE: Self = Self(9);
    pub const ULTRON_3_DEVICE: Self = Self(10);
    pub const MAIDENS_DEVICE: Self = Self(11);
    pub const UMGAH_HYPERWAVE_DEVICE: Self = Self(12);
    pub const BURVIX_HYPERWAVE_DEVICE: Self = Self(13);
    pub const DATA_PLATE_1_DEVICE: Self = Self(14);
    pub const DATA_PLATE_2_DEVICE: Self = Self(15);
    pub const DATA_PLATE_3_DEVICE: Self = Self(16);
    pub const TAALO_PROTECTOR_DEVICE: Self = Self(17);
    pub const EGG_CASING0_DEVICE: Self = Self(18);
    pub const EGG_CASING1_DEVICE: Self = Self(19);
    pub const EGG_CASING2_DEVICE: Self = Self(20);
    pub const SYREEN_SHUTTLE_DEVICE: Self = Self(21);
    pub const VUX_BEAST_DEVICE: Self = Self(22);
    pub const DESTRUCT_CODE_DEVICE: Self = Self(23);
    pub const URQUAN_WARP_DEVICE: Self = Self(24);
    pub const ARTIFACT_2_DEVICE: Self = Self(25);
    pub const ARTIFACT_3_DEVICE: Self = Self(26);
    pub const LUNAR_BASE_DEVICE: Self = Self(27);
    pub const NUM_DEVICES: Self = Self(28);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_2(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_2 {
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
pub struct C2Rust_Unnamed_3(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_3 {
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
pub type CallbackArg = *mut ::core::ffi::c_void;
pub type CallbackFunction = Option<unsafe extern "C" fn(CallbackArg) -> ()>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_4(pub ::core::ffi::c_int);
impl C2Rust_Unnamed_4 {
    pub const GLOBAL_PLAYER_NAME: Self = Self(-1000000);
    pub const GLOBAL_SHIP_NAME: Self = Self(-999999);
    pub const GLOBAL_ALLIANCE_NAME: Self = Self(-999998);
}
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_5(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_5 {
    pub const NULL_PHRASE: Self = Self(0);
    pub const BEFORE_WE_GO_ON_1: Self = Self(1);
    pub const BEFORE_WE_GO_ON_2: Self = Self(2);
    pub const BEFORE_WE_GO_ON_3: Self = Self(3);
    pub const BEFORE_WE_GO_ON_4: Self = Self(4);
    pub const BEFORE_WE_GO_ON_5: Self = Self(5);
    pub const BEFORE_WE_GO_ON_6: Self = Self(6);
    pub const BEFORE_WE_GO_ON_7: Self = Self(7);
    pub const NORMAL_HELLO_A0: Self = Self(8);
    pub const NORMAL_HELLO_A1: Self = Self(9);
    pub const NORMAL_HELLO_B0: Self = Self(10);
    pub const NORMAL_HELLO_B1: Self = Self(11);
    pub const NORMAL_HELLO_C0: Self = Self(12);
    pub const NORMAL_HELLO_C1: Self = Self(13);
    pub const NORMAL_HELLO_D0: Self = Self(14);
    pub const NORMAL_HELLO_D1: Self = Self(15);
    pub const NORMAL_HELLO_E0: Self = Self(16);
    pub const NORMAL_HELLO_E1: Self = Self(17);
    pub const NORMAL_HELLO_F0: Self = Self(18);
    pub const NORMAL_HELLO_F1: Self = Self(19);
    pub const NORMAL_HELLO_G0: Self = Self(20);
    pub const NORMAL_HELLO_G1: Self = Self(21);
    pub const NORMAL_HELLO_H0: Self = Self(22);
    pub const NORMAL_HELLO_H1: Self = Self(23);
    pub const RETURN_HELLO: Self = Self(24);
    pub const NORMAL_HELLO_TAIL: Self = Self(25);
    pub const NORMAL_GOODBYE_A0: Self = Self(26);
    pub const NORMAL_GOODBYE_A1: Self = Self(27);
    pub const NORMAL_GOODBYE_B0: Self = Self(28);
    pub const NORMAL_GOODBYE_B1: Self = Self(29);
    pub const NORMAL_GOODBYE_C0: Self = Self(30);
    pub const NORMAL_GOODBYE_C1: Self = Self(31);
    pub const NORMAL_GOODBYE_D0: Self = Self(32);
    pub const NORMAL_GOODBYE_D1: Self = Self(33);
    pub const NORMAL_GOODBYE_E0: Self = Self(34);
    pub const NORMAL_GOODBYE_E1: Self = Self(35);
    pub const NORMAL_GOODBYE_F0: Self = Self(36);
    pub const NORMAL_GOODBYE_F1: Self = Self(37);
    pub const NORMAL_GOODBYE_G0: Self = Self(38);
    pub const NORMAL_GOODBYE_G1: Self = Self(39);
    pub const NORMAL_GOODBYE_H0: Self = Self(40);
    pub const NORMAL_GOODBYE_H1: Self = Self(41);
    pub const LIGHT_LOAD_A0: Self = Self(42);
    pub const LIGHT_LOAD_A1: Self = Self(43);
    pub const LIGHT_LOAD_B0: Self = Self(44);
    pub const LIGHT_LOAD_B1: Self = Self(45);
    pub const LIGHT_LOAD_C0: Self = Self(46);
    pub const LIGHT_LOAD_C1: Self = Self(47);
    pub const LIGHT_LOAD_D0: Self = Self(48);
    pub const LIGHT_LOAD_D1: Self = Self(49);
    pub const LIGHT_LOAD_E0: Self = Self(50);
    pub const LIGHT_LOAD_E1: Self = Self(51);
    pub const LIGHT_LOAD_F0: Self = Self(52);
    pub const LIGHT_LOAD_F1: Self = Self(53);
    pub const LIGHT_LOAD_G0: Self = Self(54);
    pub const LIGHT_LOAD_G1: Self = Self(55);
    pub const MEDIUM_LOAD_A0: Self = Self(56);
    pub const MEDIUM_LOAD_A1: Self = Self(57);
    pub const MEDIUM_LOAD_B0: Self = Self(58);
    pub const MEDIUM_LOAD_B1: Self = Self(59);
    pub const MEDIUM_LOAD_C0: Self = Self(60);
    pub const MEDIUM_LOAD_C1: Self = Self(61);
    pub const MEDIUM_LOAD_D0: Self = Self(62);
    pub const MEDIUM_LOAD_D1: Self = Self(63);
    pub const MEDIUM_LOAD_E0: Self = Self(64);
    pub const MEDIUM_LOAD_E1: Self = Self(65);
    pub const MEDIUM_LOAD_F0: Self = Self(66);
    pub const MEDIUM_LOAD_F1: Self = Self(67);
    pub const MEDIUM_LOAD_G0: Self = Self(68);
    pub const MEDIUM_LOAD_G1: Self = Self(69);
    pub const HEAVY_LOAD_A0: Self = Self(70);
    pub const HEAVY_LOAD_A1: Self = Self(71);
    pub const HEAVY_LOAD_B0: Self = Self(72);
    pub const HEAVY_LOAD_B1: Self = Self(73);
    pub const HEAVY_LOAD_C0: Self = Self(74);
    pub const HEAVY_LOAD_C1: Self = Self(75);
    pub const HEAVY_LOAD_D0: Self = Self(76);
    pub const HEAVY_LOAD_D1: Self = Self(77);
    pub const HEAVY_LOAD_E0: Self = Self(78);
    pub const HEAVY_LOAD_E1: Self = Self(79);
    pub const HEAVY_LOAD_F0: Self = Self(80);
    pub const HEAVY_LOAD_F1: Self = Self(81);
    pub const HEAVY_LOAD_G0: Self = Self(82);
    pub const HEAVY_LOAD_G1: Self = Self(83);
    pub const STARBASE_IS_READY_A: Self = Self(84);
    pub const STARBASE_IS_READY_B: Self = Self(85);
    pub const STARBASE_IS_READY_C: Self = Self(86);
    pub const WHAT_KIND_OF_INFO: Self = Self(87);
    pub const WHICH_FUNCTION: Self = Self(88);
    pub const WHICH_HISTORY: Self = Self(89);
    pub const WHICH_MISSION: Self = Self(90);
    pub const OK_NO_NEED_INFO: Self = Self(91);
    pub const ABOUT_FUEL: Self = Self(92);
    pub const ABOUT_MODULES: Self = Self(93);
    pub const ABOUT_CREW0: Self = Self(94);
    pub const ABOUT_CREW1: Self = Self(95);
    pub const ABOUT_SHIPS: Self = Self(96);
    pub const ABOUT_RU: Self = Self(97);
    pub const ABOUT_MINERALS: Self = Self(98);
    pub const ABOUT_LIFE: Self = Self(99);
    pub const OK_ENOUGH_STARBASE: Self = Self(100);
    pub const OK_ENOUGH_MISSION: Self = Self(101);
    pub const GET_MINERALS: Self = Self(102);
    pub const ABOUT_ALIENS: Self = Self(103);
    pub const MUST_DEFEAT: Self = Self(104);
    pub const DEFEAT_LIKE_SO: Self = Self(105);
    pub const FIND_URQUAN: Self = Self(106);
    pub const FIGHT_URQUAN: Self = Self(107);
    pub const ALLY_LIKE_SO: Self = Self(108);
    pub const STRONG_LIKE_SO: Self = Self(109);
    pub const OK_ENOUGH_DEFEAT: Self = Self(110);
    pub const WHICH_ALIEN: Self = Self(111);
    pub const WHICH_WAR: Self = Self(112);
    pub const WHICH_ANCIENT: Self = Self(113);
    pub const OK_ENOUGH_HISTORY: Self = Self(114);
    pub const WHICH_ALLIANCE: Self = Self(115);
    pub const WHICH_HIERARCHY: Self = Self(116);
    pub const ABOUT_OTHER: Self = Self(117);
    pub const OK_ENOUGH_ALIENS: Self = Self(118);
    pub const ABOUT_SHOFIXTI: Self = Self(119);
    pub const ABOUT_YEHAT: Self = Self(120);
    pub const ABOUT_ARILOU: Self = Self(121);
    pub const ABOUT_CHENJESU: Self = Self(122);
    pub const ABOUT_MMRNMHRM: Self = Self(123);
    pub const ABOUT_SYREEN: Self = Self(124);
    pub const OK_ENOUGH_ALLIANCE: Self = Self(125);
    pub const ABOUT_URQUAN: Self = Self(126);
    pub const ABOUT_MYCON: Self = Self(127);
    pub const ABOUT_SPATHI: Self = Self(128);
    pub const ABOUT_UMGAH: Self = Self(129);
    pub const ABOUT_ANDROSYNTH: Self = Self(130);
    pub const ABOUT_VUX: Self = Self(131);
    pub const ABOUT_ILWRATH: Self = Self(132);
    pub const OK_ENOUGH_HIERARCHY: Self = Self(133);
    pub const ABOUT_PRECURSORS: Self = Self(134);
    pub const ABOUT_OLD_RACES: Self = Self(135);
    pub const ABOUT_ALIENS_ON_EARTH: Self = Self(136);
    pub const OK_ENOUGH_ANCIENT: Self = Self(137);
    pub const URQUAN_STARTED_WAR: Self = Self(138);
    pub const WAR_WAS_LIKE_SO: Self = Self(139);
    pub const LOST_WAR_BECAUSE: Self = Self(140);
    pub const AFTER_WAR: Self = Self(141);
    pub const OK_ENOUGH_WAR: Self = Self(142);
    pub const STARBASE_BULLETIN_TAIL: Self = Self(143);
    pub const BETWEEN_BULLETINS: Self = Self(144);
    pub const STARBASE_BULLETIN_1: Self = Self(145);
    pub const STARBASE_BULLETIN_2: Self = Self(146);
    pub const STARBASE_BULLETIN_3: Self = Self(147);
    pub const STARBASE_BULLETIN_4: Self = Self(148);
    pub const STARBASE_BULLETIN_5: Self = Self(149);
    pub const STARBASE_BULLETIN_6: Self = Self(150);
    pub const STARBASE_BULLETIN_7: Self = Self(151);
    pub const STARBASE_BULLETIN_8: Self = Self(152);
    pub const STARBASE_BULLETIN_9: Self = Self(153);
    pub const STARBASE_BULLETIN_10: Self = Self(154);
    pub const STARBASE_BULLETIN_11: Self = Self(155);
    pub const STARBASE_BULLETIN_12: Self = Self(156);
    pub const STARBASE_BULLETIN_13: Self = Self(157);
    pub const STARBASE_BULLETIN_14: Self = Self(158);
    pub const STARBASE_BULLETIN_15: Self = Self(159);
    pub const STARBASE_BULLETIN_16: Self = Self(160);
    pub const STARBASE_BULLETIN_18: Self = Self(161);
    pub const STARBASE_BULLETIN_19: Self = Self(162);
    pub const STARBASE_BULLETIN_22: Self = Self(163);
    pub const STARBASE_BULLETIN_27: Self = Self(164);
    pub const STARBASE_BULLETIN_28: Self = Self(165);
    pub const STARBASE_BULLETIN_29: Self = Self(166);
    pub const STARBASE_BULLETIN_30: Self = Self(167);
    pub const DEVICE_HEAD: Self = Self(168);
    pub const BETWEEN_DEVICES: Self = Self(169);
    pub const DEVICE_TAIL: Self = Self(170);
    pub const ABOUT_PORTAL: Self = Self(171);
    pub const ABOUT_TALKPET: Self = Self(172);
    pub const ABOUT_BOMB: Self = Self(173);
    pub const ABOUT_SUN: Self = Self(174);
    pub const ABOUT_MAIDENS: Self = Self(175);
    pub const ABOUT_SPHERE: Self = Self(176);
    pub const ABOUT_HELIX: Self = Self(177);
    pub const ABOUT_SPINDLE: Self = Self(178);
    pub const ABOUT_ULTRON_0: Self = Self(179);
    pub const ABOUT_ULTRON_1: Self = Self(180);
    pub const ABOUT_ULTRON_2: Self = Self(181);
    pub const ABOUT_ULTRON_3: Self = Self(182);
    pub const ABOUT_UCASTER: Self = Self(183);
    pub const ABOUT_BCASTER: Self = Self(184);
    pub const ABOUT_SHIELD: Self = Self(185);
    pub const ABOUT_EGGCASE_0: Self = Self(186);
    pub const ABOUT_SHUTTLE: Self = Self(187);
    pub const ABOUT_VUXBEAST0: Self = Self(188);
    pub const ABOUT_VUXBEAST1: Self = Self(189);
    pub const ABOUT_DESTRUCT: Self = Self(190);
    pub const ABOUT_WARPPOD: Self = Self(191);
    pub const ABOUT_ARTIFACT_2: Self = Self(192);
    pub const ABOUT_ARTIFACT_3: Self = Self(193);
    pub const LETS_SEE: Self = Self(194);
    pub const GO_GET_MINERALS: Self = Self(195);
    pub const IMPROVE_FLAGSHIP_WITH_RU: Self = Self(196);
    pub const GOT_OK_FLAGSHIP: Self = Self(197);
    pub const GO_ALLY_WITH_ALIENS: Self = Self(198);
    pub const MADE_SOME_ALLIES: Self = Self(199);
    pub const GET_SHIPS_BY_MINING_OR_ALLIANCE: Self = Self(200);
    pub const GOT_OK_FLEET: Self = Self(201);
    pub const BUY_COMBAT_SHIPS: Self = Self(202);
    pub const GO_LEARN_ABOUT_URQUAN: Self = Self(203);
    pub const MAKE_FLAGSHIP_AWESOME: Self = Self(204);
    pub const KNOW_ABOUT_SAMATRA: Self = Self(205);
    pub const GOT_AWESOME_FLAGSHIP: Self = Self(206);
    pub const GOT_BOMB: Self = Self(207);
    pub const FIND_WAY_TO_DESTROY_SAMATRA: Self = Self(208);
    pub const MUST_INCREASE_BOMB_STRENGTH: Self = Self(209);
    pub const MUST_ACQUIRE_AWESOME_FLEET: Self = Self(210);
    pub const MUST_ELIMINATE_URQUAN_GUARDS: Self = Self(211);
    pub const CHMMR_IMPROVED_BOMB: Self = Self(212);
    pub const GOT_AWESOME_FLEET: Self = Self(213);
    pub const GO_DESTROY_SAMATRA: Self = Self(214);
    pub const GOOD_LUCK_AGAIN: Self = Self(215);
    pub const IMPROVE_1: Self = Self(216);
    pub const IMPROVE_2: Self = Self(217);
    pub const NEED_THRUSTERS_1: Self = Self(218);
    pub const NEED_THRUSTERS_2: Self = Self(219);
    pub const NEED_TURN_1: Self = Self(220);
    pub const NEED_TURN_2: Self = Self(221);
    pub const NEED_GUNS_1: Self = Self(222);
    pub const NEED_GUNS_2: Self = Self(223);
    pub const NEED_CREW_1: Self = Self(224);
    pub const NEED_CREW_2: Self = Self(225);
    pub const NEED_FUEL_1: Self = Self(226);
    pub const NEED_FUEL_2: Self = Self(227);
    pub const NEED_STORAGE_1: Self = Self(228);
    pub const NEED_LANDERS_2: Self = Self(229);
    pub const NEED_LANDERS_1: Self = Self(230);
    pub const NEED_DYNAMOS_1: Self = Self(231);
    pub const NEED_DYNAMOS_2: Self = Self(232);
    pub const NEED_POINT: Self = Self(233);
    pub const have_minerals: Self = Self(234);
    pub const goodbye_commander: Self = Self(235);
    pub const repeat_bulletins: Self = Self(236);
    pub const need_info: Self = Self(237);
    pub const starbase_functions: Self = Self(238);
    pub const history: Self = Self(239);
    pub const our_mission: Self = Self(240);
    pub const no_need_info: Self = Self(241);
    pub const enough_starbase: Self = Self(242);
    pub const enough_mission: Self = Self(243);
    pub const tell_me_about_fuel0: Self = Self(244);
    pub const tell_me_about_fuel1: Self = Self(245);
    pub const tell_me_about_modules0: Self = Self(246);
    pub const tell_me_about_modules1: Self = Self(247);
    pub const tell_me_about_crew: Self = Self(248);
    pub const tell_me_about_ships: Self = Self(249);
    pub const tell_me_about_ru: Self = Self(250);
    pub const tell_me_about_minerals: Self = Self(251);
    pub const tell_me_about_life: Self = Self(252);
    pub const where_get_minerals: Self = Self(253);
    pub const what_about_aliens: Self = Self(254);
    pub const what_about_urquan: Self = Self(255);
    pub const how_defeat: Self = Self(256);
    pub const how_find_urquan: Self = Self(257);
    pub const how_fight_urquan: Self = Self(258);
    pub const how_ally: Self = Self(259);
    pub const enough_defeat: Self = Self(260);
    pub const alien_races: Self = Self(261);
    pub const the_war: Self = Self(262);
    pub const ancient_history: Self = Self(263);
    pub const enough_history: Self = Self(264);
    pub const what_about_alliance: Self = Self(265);
    pub const what_about_hierarchy: Self = Self(266);
    pub const what_about_other: Self = Self(267);
    pub const enough_aliens: Self = Self(268);
    pub const shofixti: Self = Self(269);
    pub const yehat: Self = Self(270);
    pub const arilou: Self = Self(271);
    pub const chenjesu: Self = Self(272);
    pub const mmrnmhrm: Self = Self(273);
    pub const syreen: Self = Self(274);
    pub const enough_alliance: Self = Self(275);
    pub const urquan: Self = Self(276);
    pub const mycon: Self = Self(277);
    pub const spathi: Self = Self(278);
    pub const umgah: Self = Self(279);
    pub const androsynth: Self = Self(280);
    pub const vux: Self = Self(281);
    pub const ilwrath: Self = Self(282);
    pub const enough_hierarchy: Self = Self(283);
    pub const precursors: Self = Self(284);
    pub const old_races: Self = Self(285);
    pub const aliens_on_earth: Self = Self(286);
    pub const enough_ancient: Self = Self(287);
    pub const what_started_war: Self = Self(288);
    pub const what_was_war_like: Self = Self(289);
    pub const why_lose_war: Self = Self(290);
    pub const what_after_war: Self = Self(291);
    pub const enough_war: Self = Self(292);
    pub const new_devices: Self = Self(293);
    pub const how_get_strong: Self = Self(294);
    pub const what_do_now: Self = Self(295);
    pub const YOUR_FLAGSHIP_PC: Self = Self(296);
    pub const YOUR_FLAGSHIP_3DO0: Self = Self(297);
    pub const YOUR_FLAGSHIP_3DO1: Self = Self(298);
    pub const YOUR_FLAGSHIP_3DO2: Self = Self(299);
    pub const SPACE: Self = Self(300);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_6(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_6 {
    pub const DEAD_GUY: Self = Self(0);
    pub const GOOD_GUY: Self = Self(1);
    pub const BAD_GUY: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_7(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_7 {
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
static mut commander_desc: LOCDATA = LOCDATA {
    init_encounter_func: None,
    post_encounter_func: None,
    uninit_encounter_func: None,
    AlienFrameRes: b"comm.commander.graphics\0".as_ptr() as RESOURCE,
    AlienFontRes: b"comm.commander.font\0".as_ptr() as RESOURCE,
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
    AlienTextValign: TEXT_VALIGN::VALIGN_MIDDLE,
    AlienColorMapRes: b"comm.commander.colortable\0".as_ptr() as RESOURCE,
    AlienSongRes: b"comm.commander.music\0".as_ptr() as RESOURCE,
    AlienAltSongRes: ::core::ptr::null::<::core::ffi::c_char>(),
    AlienSongFlags: 0 as LDAS_FLAGS,
    ConversationPhrasesRes: b"comm.starbase.dialogue\0".as_ptr() as RESOURCE,
    NumAnimations: 10 as COUNT,
    AlienAmbientArray: [
        ANIMATION_DESC {
            StartIndex: 1 as COUNT,
            NumFrames: 3 as BYTE,
            AnimFlags: ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as BYTE,
            BaseFrameRate: (840 as ::core::ffi::c_int / 15 as ::core::ffi::c_int)
                as COUNT,
            RandomFrameRate: 0 as COUNT,
            BaseRestartRate: 0 as COUNT,
            RandomRestartRate: (840 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                as COUNT,
            BlockMask: 0 as DWORD,
        },
        ANIMATION_DESC {
            StartIndex: 10 as COUNT,
            NumFrames: 30 as BYTE,
            AnimFlags: ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as BYTE,
            BaseFrameRate: (840 as ::core::ffi::c_int / 40 as ::core::ffi::c_int)
                as COUNT,
            RandomFrameRate: 0 as COUNT,
            BaseRestartRate: (840 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                as COUNT,
            RandomRestartRate: 0 as COUNT,
            BlockMask: 0 as DWORD,
        },
        ANIMATION_DESC {
            StartIndex: 40 as COUNT,
            NumFrames: 7 as BYTE,
            AnimFlags: ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as BYTE,
            BaseFrameRate: (840 as ::core::ffi::c_int / 40 as ::core::ffi::c_int)
                as COUNT,
            RandomFrameRate: 0 as COUNT,
            BaseRestartRate: 0 as COUNT,
            RandomRestartRate: (840 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                as COUNT,
            BlockMask: 0 as DWORD,
        },
        ANIMATION_DESC {
            StartIndex: 47 as COUNT,
            NumFrames: 8 as BYTE,
            AnimFlags: ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as BYTE,
            BaseFrameRate: (840 as ::core::ffi::c_int / 40 as ::core::ffi::c_int)
                as COUNT,
            RandomFrameRate: 0 as COUNT,
            BaseRestartRate: 0 as COUNT,
            RandomRestartRate: (840 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                as COUNT,
            BlockMask: 0 as DWORD,
        },
        ANIMATION_DESC {
            StartIndex: 55 as COUNT,
            NumFrames: 6 as BYTE,
            AnimFlags: ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as BYTE,
            BaseFrameRate: (840 as ::core::ffi::c_int / 40 as ::core::ffi::c_int)
                as COUNT,
            RandomFrameRate: 0 as COUNT,
            BaseRestartRate: 0 as COUNT,
            RandomRestartRate: (840 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                as COUNT,
            BlockMask: 0 as DWORD,
        },
        ANIMATION_DESC {
            StartIndex: 61 as COUNT,
            NumFrames: 6 as BYTE,
            AnimFlags: ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as BYTE,
            BaseFrameRate: (840 as ::core::ffi::c_int / 40 as ::core::ffi::c_int)
                as COUNT,
            RandomFrameRate: 0 as COUNT,
            BaseRestartRate: 0 as COUNT,
            RandomRestartRate: (840 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                as COUNT,
            BlockMask: 0 as DWORD,
        },
        ANIMATION_DESC {
            StartIndex: 67 as COUNT,
            NumFrames: 7 as BYTE,
            AnimFlags: ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as BYTE,
            BaseFrameRate: (840 as ::core::ffi::c_int / 40 as ::core::ffi::c_int)
                as COUNT,
            RandomFrameRate: 0 as COUNT,
            BaseRestartRate: 0 as COUNT,
            RandomRestartRate: (840 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                as COUNT,
            BlockMask: 0 as DWORD,
        },
        ANIMATION_DESC {
            StartIndex: 74 as COUNT,
            NumFrames: 11 as BYTE,
            AnimFlags: ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as BYTE,
            BaseFrameRate: (840 as ::core::ffi::c_int / 40 as ::core::ffi::c_int)
                as COUNT,
            RandomFrameRate: 0 as COUNT,
            BaseRestartRate: 0 as COUNT,
            RandomRestartRate: (840 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                as COUNT,
            BlockMask: 0 as DWORD,
        },
        ANIMATION_DESC {
            StartIndex: 85 as COUNT,
            NumFrames: 10 as BYTE,
            AnimFlags: ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as BYTE,
            BaseFrameRate: (840 as ::core::ffi::c_int / 40 as ::core::ffi::c_int)
                as COUNT,
            RandomFrameRate: 0 as COUNT,
            BaseRestartRate: 0 as COUNT,
            RandomRestartRate: (840 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                as COUNT,
            BlockMask: 0 as DWORD,
        },
        ANIMATION_DESC {
            StartIndex: 95 as COUNT,
            NumFrames: 1 as BYTE,
            AnimFlags: 0 as BYTE,
            BaseFrameRate: 0 as COUNT,
            RandomFrameRate: 0 as COUNT,
            BaseRestartRate: 0 as COUNT,
            RandomRestartRate: 0 as COUNT,
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
        StartIndex: 4 as COUNT,
        NumFrames: 6 as BYTE,
        AnimFlags: 0 as BYTE,
        BaseFrameRate: (840 as ::core::ffi::c_int / 10 as ::core::ffi::c_int) as COUNT,
        RandomFrameRate: (840 as ::core::ffi::c_int / 15 as ::core::ffi::c_int) as COUNT,
        BaseRestartRate: (840 as ::core::ffi::c_int * 7 as ::core::ffi::c_int
            / 60 as ::core::ffi::c_int) as COUNT,
        RandomRestartRate: (840 as ::core::ffi::c_int / 12 as ::core::ffi::c_int)
            as COUNT,
        BlockMask: 0 as DWORD,
    },
    AlienNumberSpeech: ::core::ptr::null::<NUMBER_SPEECH_DESC>(),
    AlienFrame: ::core::ptr::null_mut::<FRAME_DESC>(),
    AlienFont: ::core::ptr::null_mut::<FONT_DESC>(),
    AlienColorMap: ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>(),
    AlienSong: ::core::ptr::null_mut::<*mut TFB_SoundSample>(),
    ConversationPhrases: ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>(),
};
static mut CurBulletinMask: DWORD = 0;
unsafe extern "C" fn ByeBye(mut R: RESPONSE_REF) {
    CurBulletinMask
        |= getGameState32(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_2::STARBASE_BULLETS0.0 as ::core::ffi::c_int,
        );
    setGameState32(
        &raw mut GlobData.Game_state.GameState as *mut BYTE,
        C2Rust_Unnamed_2::STARBASE_BULLETS0.0 as ::core::ffi::c_int,
        CurBulletinMask,
    );
    if getGameState(
        &raw mut GlobData.Game_state.GameState as *mut BYTE,
        C2Rust_Unnamed_2::CHMMR_BOMB_STATE.0 as ::core::ffi::c_int,
        C2Rust_Unnamed_2::END_CHMMR_BOMB_STATE.0 as ::core::ffi::c_int,
    ) as ::core::ffi::c_int >= 2 as ::core::ffi::c_int
    {
        NPCPhrase_cb(C2Rust_Unnamed_5::GOOD_LUCK_AGAIN.0 as ::core::ffi::c_int, None);
    } else {
        let mut pStr0: RESPONSE_REF = 0 as RESPONSE_REF;
        let mut pStr1: RESPONSE_REF = 0 as RESPONSE_REF;
        match TFB_Random() as BYTE as ::core::ffi::c_int & 7 as ::core::ffi::c_int {
            0 => {
                pStr0 = C2Rust_Unnamed_5::NORMAL_GOODBYE_A0.0 as ::core::ffi::c_int
                    as RESPONSE_REF;
                pStr1 = C2Rust_Unnamed_5::NORMAL_GOODBYE_A1.0 as ::core::ffi::c_int
                    as RESPONSE_REF;
            }
            1 => {
                pStr0 = C2Rust_Unnamed_5::NORMAL_GOODBYE_B0.0 as ::core::ffi::c_int
                    as RESPONSE_REF;
                pStr1 = C2Rust_Unnamed_5::NORMAL_GOODBYE_B1.0 as ::core::ffi::c_int
                    as RESPONSE_REF;
            }
            2 => {
                pStr0 = C2Rust_Unnamed_5::NORMAL_GOODBYE_C0.0 as ::core::ffi::c_int
                    as RESPONSE_REF;
                pStr1 = C2Rust_Unnamed_5::NORMAL_GOODBYE_C1.0 as ::core::ffi::c_int
                    as RESPONSE_REF;
            }
            3 => {
                pStr0 = C2Rust_Unnamed_5::NORMAL_GOODBYE_D0.0 as ::core::ffi::c_int
                    as RESPONSE_REF;
                pStr1 = C2Rust_Unnamed_5::NORMAL_GOODBYE_D1.0 as ::core::ffi::c_int
                    as RESPONSE_REF;
            }
            4 => {
                pStr0 = C2Rust_Unnamed_5::NORMAL_GOODBYE_E0.0 as ::core::ffi::c_int
                    as RESPONSE_REF;
                pStr1 = C2Rust_Unnamed_5::NORMAL_GOODBYE_E1.0 as ::core::ffi::c_int
                    as RESPONSE_REF;
            }
            5 => {
                pStr0 = C2Rust_Unnamed_5::NORMAL_GOODBYE_F0.0 as ::core::ffi::c_int
                    as RESPONSE_REF;
                pStr1 = C2Rust_Unnamed_5::NORMAL_GOODBYE_F1.0 as ::core::ffi::c_int
                    as RESPONSE_REF;
            }
            6 => {
                pStr0 = C2Rust_Unnamed_5::NORMAL_GOODBYE_G0.0 as ::core::ffi::c_int
                    as RESPONSE_REF;
                pStr1 = C2Rust_Unnamed_5::NORMAL_GOODBYE_G1.0 as ::core::ffi::c_int
                    as RESPONSE_REF;
            }
            7 => {
                pStr0 = C2Rust_Unnamed_5::NORMAL_GOODBYE_H0.0 as ::core::ffi::c_int
                    as RESPONSE_REF;
                pStr1 = C2Rust_Unnamed_5::NORMAL_GOODBYE_H1.0 as ::core::ffi::c_int
                    as RESPONSE_REF;
            }
            _ => {}
        }
        NPCPhrase_cb(pStr0 as ::core::ffi::c_int, None);
        if usingSpeech.0 == 0 {
            NPCPhrase_cb(C2Rust_Unnamed_5::SPACE.0 as ::core::ffi::c_int, None);
            NPCPhrase_cb(C2Rust_Unnamed_4::GLOBAL_PLAYER_NAME.0, None);
        }
        NPCPhrase_cb(pStr1 as ::core::ffi::c_int, None);
    };
}
static mut stack0: BYTE = 0;
static mut stack1: BYTE = 0;
static mut stack2: BYTE = 0;
static mut stack3: BYTE = 0;
unsafe extern "C" fn AllianceInfo(mut R: RESPONSE_REF) {
    static mut AllianceMask: BYTE = 0 as BYTE;
    if R as ::core::ffi::c_int
        == C2Rust_Unnamed_5::what_about_alliance.0 as ::core::ffi::c_int
    {
        NPCPhrase_cb(C2Rust_Unnamed_5::WHICH_ALLIANCE.0 as ::core::ffi::c_int, None);
        AllianceMask = 0 as BYTE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_5::shofixti.0 as ::core::ffi::c_int
    {
        NPCPhrase_cb(C2Rust_Unnamed_5::ABOUT_SHOFIXTI.0 as ::core::ffi::c_int, None);
        AllianceMask = (AllianceMask as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as BYTE;
    } else if R as ::core::ffi::c_int == C2Rust_Unnamed_5::yehat.0 as ::core::ffi::c_int
    {
        NPCPhrase_cb(C2Rust_Unnamed_5::ABOUT_YEHAT.0 as ::core::ffi::c_int, None);
        AllianceMask = (AllianceMask as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as BYTE;
    } else if R as ::core::ffi::c_int == C2Rust_Unnamed_5::arilou.0 as ::core::ffi::c_int
    {
        NPCPhrase_cb(C2Rust_Unnamed_5::ABOUT_ARILOU.0 as ::core::ffi::c_int, None);
        AllianceMask = (AllianceMask as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as BYTE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_5::chenjesu.0 as ::core::ffi::c_int
    {
        NPCPhrase_cb(C2Rust_Unnamed_5::ABOUT_CHENJESU.0 as ::core::ffi::c_int, None);
        AllianceMask = (AllianceMask as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as BYTE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_5::mmrnmhrm.0 as ::core::ffi::c_int
    {
        NPCPhrase_cb(C2Rust_Unnamed_5::ABOUT_MMRNMHRM.0 as ::core::ffi::c_int, None);
        AllianceMask = (AllianceMask as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as BYTE;
    } else if R as ::core::ffi::c_int == C2Rust_Unnamed_5::syreen.0 as ::core::ffi::c_int
    {
        NPCPhrase_cb(C2Rust_Unnamed_5::ABOUT_SYREEN.0 as ::core::ffi::c_int, None);
        AllianceMask = (AllianceMask as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as BYTE;
    }
    if AllianceMask as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int == 0
    {
        DoResponsePhrase(
            C2Rust_Unnamed_5::shofixti.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(AllianceInfo as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        );
    }
    if AllianceMask as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int == 0
    {
        DoResponsePhrase(
            C2Rust_Unnamed_5::yehat.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(AllianceInfo as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        );
    }
    if AllianceMask as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int == 0
    {
        DoResponsePhrase(
            C2Rust_Unnamed_5::arilou.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(AllianceInfo as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        );
    }
    if AllianceMask as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int == 0
    {
        DoResponsePhrase(
            C2Rust_Unnamed_5::chenjesu.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(AllianceInfo as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        );
    }
    if AllianceMask as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int == 0
    {
        DoResponsePhrase(
            C2Rust_Unnamed_5::mmrnmhrm.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(AllianceInfo as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        );
    }
    if AllianceMask as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int == 0
    {
        DoResponsePhrase(
            C2Rust_Unnamed_5::syreen.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(AllianceInfo as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        );
    }
    DoResponsePhrase(
        C2Rust_Unnamed_5::enough_alliance.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(AlienRaces as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    );
}
unsafe extern "C" fn HierarchyInfo(mut R: RESPONSE_REF) {
    static mut HierarchyMask: BYTE = 0 as BYTE;
    if R as ::core::ffi::c_int
        == C2Rust_Unnamed_5::what_about_hierarchy.0 as ::core::ffi::c_int
    {
        NPCPhrase_cb(C2Rust_Unnamed_5::WHICH_HIERARCHY.0 as ::core::ffi::c_int, None);
        HierarchyMask = 0 as BYTE;
    } else if R as ::core::ffi::c_int == C2Rust_Unnamed_5::urquan.0 as ::core::ffi::c_int
    {
        NPCPhrase_cb(C2Rust_Unnamed_5::ABOUT_URQUAN.0 as ::core::ffi::c_int, None);
        HierarchyMask = (HierarchyMask as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as BYTE;
    } else if R as ::core::ffi::c_int == C2Rust_Unnamed_5::mycon.0 as ::core::ffi::c_int
    {
        NPCPhrase_cb(C2Rust_Unnamed_5::ABOUT_MYCON.0 as ::core::ffi::c_int, None);
        HierarchyMask = (HierarchyMask as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as BYTE;
    } else if R as ::core::ffi::c_int == C2Rust_Unnamed_5::spathi.0 as ::core::ffi::c_int
    {
        NPCPhrase_cb(C2Rust_Unnamed_5::ABOUT_SPATHI.0 as ::core::ffi::c_int, None);
        HierarchyMask = (HierarchyMask as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as BYTE;
    } else if R as ::core::ffi::c_int == C2Rust_Unnamed_5::umgah.0 as ::core::ffi::c_int
    {
        NPCPhrase_cb(C2Rust_Unnamed_5::ABOUT_UMGAH.0 as ::core::ffi::c_int, None);
        HierarchyMask = (HierarchyMask as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as BYTE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_5::androsynth.0 as ::core::ffi::c_int
    {
        NPCPhrase_cb(C2Rust_Unnamed_5::ABOUT_ANDROSYNTH.0 as ::core::ffi::c_int, None);
        HierarchyMask = (HierarchyMask as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as BYTE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_5::ilwrath.0 as ::core::ffi::c_int
    {
        NPCPhrase_cb(C2Rust_Unnamed_5::ABOUT_ILWRATH.0 as ::core::ffi::c_int, None);
        HierarchyMask = (HierarchyMask as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as BYTE;
    } else if R as ::core::ffi::c_int == C2Rust_Unnamed_5::vux.0 as ::core::ffi::c_int {
        NPCPhrase_cb(C2Rust_Unnamed_5::ABOUT_VUX.0 as ::core::ffi::c_int, None);
        HierarchyMask = (HierarchyMask as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as BYTE;
    }
    if HierarchyMask as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int == 0
    {
        DoResponsePhrase(
            C2Rust_Unnamed_5::urquan.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(HierarchyInfo as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        );
    }
    if HierarchyMask as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int == 0
    {
        DoResponsePhrase(
            C2Rust_Unnamed_5::mycon.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(HierarchyInfo as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        );
    }
    if HierarchyMask as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int == 0
    {
        DoResponsePhrase(
            C2Rust_Unnamed_5::spathi.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(HierarchyInfo as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        );
    }
    if HierarchyMask as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int == 0
    {
        DoResponsePhrase(
            C2Rust_Unnamed_5::umgah.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(HierarchyInfo as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        );
    }
    if HierarchyMask as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int == 0
    {
        DoResponsePhrase(
            C2Rust_Unnamed_5::androsynth.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(HierarchyInfo as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        );
    }
    if HierarchyMask as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int == 0
    {
        DoResponsePhrase(
            C2Rust_Unnamed_5::ilwrath.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(HierarchyInfo as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        );
    }
    if HierarchyMask as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int == 0
    {
        DoResponsePhrase(
            C2Rust_Unnamed_5::vux.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(HierarchyInfo as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        );
    }
    DoResponsePhrase(
        C2Rust_Unnamed_5::enough_hierarchy.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(AlienRaces as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    );
}
unsafe extern "C" fn AlienRaces(mut R: RESPONSE_REF) {
    static mut RacesMask: BYTE = 0 as BYTE;
    if R as ::core::ffi::c_int == C2Rust_Unnamed_5::alien_races.0 as ::core::ffi::c_int {
        NPCPhrase_cb(C2Rust_Unnamed_5::WHICH_ALIEN.0 as ::core::ffi::c_int, None);
        RacesMask = 0 as BYTE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_5::enough_alliance.0 as ::core::ffi::c_int
    {
        NPCPhrase_cb(C2Rust_Unnamed_5::OK_ENOUGH_ALLIANCE.0 as ::core::ffi::c_int, None);
        RacesMask = (RacesMask as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as BYTE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_5::enough_hierarchy.0 as ::core::ffi::c_int
    {
        NPCPhrase_cb(
            C2Rust_Unnamed_5::OK_ENOUGH_HIERARCHY.0 as ::core::ffi::c_int,
            None,
        );
        RacesMask = (RacesMask as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as BYTE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_5::what_about_other.0 as ::core::ffi::c_int
    {
        NPCPhrase_cb(C2Rust_Unnamed_5::ABOUT_OTHER.0 as ::core::ffi::c_int, None);
        RacesMask = (RacesMask as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as BYTE;
    }
    if RacesMask as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int == 0
    {
        DoResponsePhrase(
            C2Rust_Unnamed_5::what_about_alliance.0 as ::core::ffi::c_int
                as RESPONSE_REF,
            Some(AllianceInfo as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        );
    }
    if RacesMask as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int == 0
    {
        DoResponsePhrase(
            C2Rust_Unnamed_5::what_about_hierarchy.0 as ::core::ffi::c_int
                as RESPONSE_REF,
            Some(HierarchyInfo as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        );
    }
    if RacesMask as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int == 0
    {
        DoResponsePhrase(
            C2Rust_Unnamed_5::what_about_other.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(AlienRaces as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        );
    }
    DoResponsePhrase(
        C2Rust_Unnamed_5::enough_aliens.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(TellHistory as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    );
}
unsafe extern "C" fn WarInfo(mut R: RESPONSE_REF) {
    static mut WarMask: BYTE = 0 as BYTE;
    if R as ::core::ffi::c_int == C2Rust_Unnamed_5::the_war.0 as ::core::ffi::c_int {
        NPCPhrase_cb(C2Rust_Unnamed_5::WHICH_WAR.0 as ::core::ffi::c_int, None);
        WarMask = 0 as BYTE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_5::what_started_war.0 as ::core::ffi::c_int
    {
        NPCPhrase_cb(C2Rust_Unnamed_5::URQUAN_STARTED_WAR.0 as ::core::ffi::c_int, None);
        WarMask = (WarMask as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as BYTE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_5::what_was_war_like.0 as ::core::ffi::c_int
    {
        NPCPhrase_cb(C2Rust_Unnamed_5::WAR_WAS_LIKE_SO.0 as ::core::ffi::c_int, None);
        WarMask = (WarMask as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as BYTE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_5::why_lose_war.0 as ::core::ffi::c_int
    {
        NPCPhrase_cb(C2Rust_Unnamed_5::LOST_WAR_BECAUSE.0 as ::core::ffi::c_int, None);
        WarMask = (WarMask as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as BYTE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_5::what_after_war.0 as ::core::ffi::c_int
    {
        NPCPhrase_cb(C2Rust_Unnamed_5::AFTER_WAR.0 as ::core::ffi::c_int, None);
        WarMask = (WarMask as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as BYTE;
    }
    if WarMask as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int == 0
    {
        DoResponsePhrase(
            C2Rust_Unnamed_5::what_started_war.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(WarInfo as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        );
    }
    if WarMask as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int == 0
    {
        DoResponsePhrase(
            C2Rust_Unnamed_5::what_was_war_like.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(WarInfo as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        );
    }
    if WarMask as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int == 0
    {
        DoResponsePhrase(
            C2Rust_Unnamed_5::why_lose_war.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(WarInfo as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        );
    }
    if WarMask as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int == 0
    {
        DoResponsePhrase(
            C2Rust_Unnamed_5::what_after_war.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(WarInfo as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        );
    }
    DoResponsePhrase(
        C2Rust_Unnamed_5::enough_war.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(TellHistory as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    );
}
unsafe extern "C" fn AncientHistory(mut R: RESPONSE_REF) {
    static mut AncientMask: BYTE = 0 as BYTE;
    if R as ::core::ffi::c_int
        == C2Rust_Unnamed_5::ancient_history.0 as ::core::ffi::c_int
    {
        NPCPhrase_cb(C2Rust_Unnamed_5::WHICH_ANCIENT.0 as ::core::ffi::c_int, None);
        AncientMask = 0 as BYTE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_5::precursors.0 as ::core::ffi::c_int
    {
        NPCPhrase_cb(C2Rust_Unnamed_5::ABOUT_PRECURSORS.0 as ::core::ffi::c_int, None);
        AncientMask = (AncientMask as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as BYTE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_5::old_races.0 as ::core::ffi::c_int
    {
        NPCPhrase_cb(C2Rust_Unnamed_5::ABOUT_OLD_RACES.0 as ::core::ffi::c_int, None);
        AncientMask = (AncientMask as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as BYTE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_5::aliens_on_earth.0 as ::core::ffi::c_int
    {
        NPCPhrase_cb(
            C2Rust_Unnamed_5::ABOUT_ALIENS_ON_EARTH.0 as ::core::ffi::c_int,
            None,
        );
        AncientMask = (AncientMask as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as BYTE;
    }
    if AncientMask as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int == 0
    {
        DoResponsePhrase(
            C2Rust_Unnamed_5::precursors.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(AncientHistory as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        );
    }
    if AncientMask as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int == 0
    {
        DoResponsePhrase(
            C2Rust_Unnamed_5::old_races.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(AncientHistory as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        );
    }
    if AncientMask as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int == 0
    {
        DoResponsePhrase(
            C2Rust_Unnamed_5::aliens_on_earth.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(AncientHistory as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        );
    }
    DoResponsePhrase(
        C2Rust_Unnamed_5::enough_ancient.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(TellHistory as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    );
}
unsafe extern "C" fn TellHistory(mut R: RESPONSE_REF) {
    let mut pstack: [RESPONSE_REF; 3] = [0; 3];
    if R as ::core::ffi::c_int == C2Rust_Unnamed_5::history.0 as ::core::ffi::c_int {
        NPCPhrase_cb(C2Rust_Unnamed_5::WHICH_HISTORY.0 as ::core::ffi::c_int, None);
        stack0 = 0 as BYTE;
        stack1 = 0 as BYTE;
        stack2 = 0 as BYTE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_5::enough_aliens.0 as ::core::ffi::c_int
    {
        NPCPhrase_cb(C2Rust_Unnamed_5::OK_ENOUGH_ALIENS.0 as ::core::ffi::c_int, None);
        stack0 = 1 as BYTE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_5::enough_war.0 as ::core::ffi::c_int
    {
        NPCPhrase_cb(C2Rust_Unnamed_5::OK_ENOUGH_WAR.0 as ::core::ffi::c_int, None);
        stack1 = 1 as BYTE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_5::enough_ancient.0 as ::core::ffi::c_int
    {
        NPCPhrase_cb(C2Rust_Unnamed_5::OK_ENOUGH_ANCIENT.0 as ::core::ffi::c_int, None);
        stack2 = 1 as BYTE;
    }
    match stack0 as ::core::ffi::c_int {
        0 => {
            pstack[0usize] = C2Rust_Unnamed_5::alien_races.0 as ::core::ffi::c_int
                as RESPONSE_REF;
        }
        _ => {
            pstack[0usize] = 0 as RESPONSE_REF;
        }
    }
    match stack1 as ::core::ffi::c_int {
        0 => {
            pstack[1usize] = C2Rust_Unnamed_5::the_war.0 as ::core::ffi::c_int
                as RESPONSE_REF;
        }
        _ => {
            pstack[1usize] = 0 as RESPONSE_REF;
        }
    }
    match stack2 as ::core::ffi::c_int {
        0 => {
            pstack[2usize] = C2Rust_Unnamed_5::ancient_history.0 as ::core::ffi::c_int
                as RESPONSE_REF;
        }
        _ => {
            pstack[2usize] = 0 as RESPONSE_REF;
        }
    }
    if pstack[0usize] != 0 {
        DoResponsePhrase(
            pstack[0usize],
            Some(AlienRaces as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        );
    }
    if pstack[1usize] != 0 {
        DoResponsePhrase(
            pstack[1usize],
            Some(WarInfo as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        );
    }
    if pstack[2usize] != 0 {
        DoResponsePhrase(
            pstack[2usize],
            Some(AncientHistory as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        );
    }
    DoResponsePhrase(
        C2Rust_Unnamed_5::enough_history.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(NeedInfo as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    );
}
unsafe extern "C" fn DefeatUrquan(mut R: RESPONSE_REF) {
    static mut DefeatMask: BYTE = 0 as BYTE;
    if R as ::core::ffi::c_int == C2Rust_Unnamed_5::how_defeat.0 as ::core::ffi::c_int {
        NPCPhrase_cb(C2Rust_Unnamed_5::DEFEAT_LIKE_SO.0 as ::core::ffi::c_int, None);
        DefeatMask = 0 as BYTE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_5::how_find_urquan.0 as ::core::ffi::c_int
    {
        NPCPhrase_cb(C2Rust_Unnamed_5::FIND_URQUAN.0 as ::core::ffi::c_int, None);
        DefeatMask = (DefeatMask as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as BYTE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_5::how_fight_urquan.0 as ::core::ffi::c_int
    {
        NPCPhrase_cb(C2Rust_Unnamed_5::FIGHT_URQUAN.0 as ::core::ffi::c_int, None);
        DefeatMask = (DefeatMask as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as BYTE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_5::how_ally.0 as ::core::ffi::c_int
    {
        NPCPhrase_cb(C2Rust_Unnamed_5::ALLY_LIKE_SO.0 as ::core::ffi::c_int, None);
        DefeatMask = (DefeatMask as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as BYTE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_5::how_get_strong.0 as ::core::ffi::c_int
    {
        NPCPhrase_cb(C2Rust_Unnamed_5::STRONG_LIKE_SO.0 as ::core::ffi::c_int, None);
        DefeatMask = (DefeatMask as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as BYTE;
    }
    if DefeatMask as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int == 0
    {
        DoResponsePhrase(
            C2Rust_Unnamed_5::how_find_urquan.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(DefeatUrquan as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        );
    }
    if DefeatMask as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int == 0
    {
        DoResponsePhrase(
            C2Rust_Unnamed_5::how_fight_urquan.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(DefeatUrquan as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        );
    }
    if DefeatMask as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int == 0
    {
        DoResponsePhrase(
            C2Rust_Unnamed_5::how_ally.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(DefeatUrquan as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        );
    }
    if DefeatMask as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int == 0
    {
        DoResponsePhrase(
            C2Rust_Unnamed_5::how_get_strong.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(DefeatUrquan as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        );
    }
    DoResponsePhrase(
        C2Rust_Unnamed_5::enough_defeat.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(TellMission as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    );
}
unsafe extern "C" fn AnalyzeCondition() {
    let mut i: BYTE = 0;
    let mut num_thrusters: BYTE = 0 as BYTE;
    let mut num_jets: BYTE = 0 as BYTE;
    let mut num_guns: BYTE = 0 as BYTE;
    let mut num_bays: BYTE = 0 as BYTE;
    let mut num_batts: BYTE = 0 as BYTE;
    let mut num_track: BYTE = 0 as BYTE;
    let mut num_defense: BYTE = 0 as BYTE;
    let mut HasMinimum: BOOLEAN = BOOLEAN::FALSE;
    i = 0 as BYTE;
    while (i as ::core::ffi::c_int) < 11 as ::core::ffi::c_int {
        if (GlobData.SIS_state.DriveSlots[i as usize] as ::core::ffi::c_int)
            < C2Rust_Unnamed_0::NUM_MODULES.0 as ::core::ffi::c_int
        {
            num_thrusters = num_thrusters.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    i = 0 as BYTE;
    while (i as ::core::ffi::c_int) < 8 as ::core::ffi::c_int {
        if (GlobData.SIS_state.JetSlots[i as usize] as ::core::ffi::c_int)
            < C2Rust_Unnamed_0::NUM_MODULES.0 as ::core::ffi::c_int
        {
            num_jets = num_jets.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    i = 0 as BYTE;
    while (i as ::core::ffi::c_int) < 16 as ::core::ffi::c_int {
        let mut which_piece: BYTE = 0;
        which_piece = GlobData.SIS_state.ModuleSlots[i as usize];
        match which_piece as ::core::ffi::c_int {
            4 => {
                num_bays = num_bays.wrapping_add(1);
            }
            7 | 8 => {
                num_batts = (num_batts as ::core::ffi::c_int
                    + (1 as ::core::ffi::c_int
                        + (which_piece as ::core::ffi::c_int
                            - C2Rust_Unnamed_0::DYNAMO_UNIT.0 as ::core::ffi::c_int)))
                    as BYTE;
            }
            9 | 10 | 11 => {
                num_guns = (num_guns as ::core::ffi::c_int
                    + (1 as ::core::ffi::c_int
                        + (which_piece as ::core::ffi::c_int
                            - C2Rust_Unnamed_0::GUN_WEAPON.0 as ::core::ffi::c_int)))
                    as BYTE;
            }
            12 => {
                num_track = num_track.wrapping_add(1);
            }
            13 => {
                num_defense = num_defense.wrapping_add(1);
            }
            _ => {}
        }
        i = i.wrapping_add(1);
    }
    if num_track as ::core::ffi::c_int != 0 && num_guns as ::core::ffi::c_int != 0 {
        num_guns = (num_guns as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as BYTE;
    }
    HasMinimum = BOOLEAN(
        (num_thrusters as ::core::ffi::c_int >= 7 as ::core::ffi::c_int
            && num_jets as ::core::ffi::c_int >= 5 as ::core::ffi::c_int
            && GlobData.SIS_state.CrewEnlisted as ::core::ffi::c_int
                >= 50 as ::core::ffi::c_int
            && GlobData.SIS_state.FuelOnBoard
                >= (50 as ::core::ffi::c_int * 100 as ::core::ffi::c_int) as DWORD
            && num_bays as ::core::ffi::c_int >= 1 as ::core::ffi::c_int
            && GlobData.SIS_state.NumLanders as ::core::ffi::c_int != 0
            && num_batts as ::core::ffi::c_int >= 1 as ::core::ffi::c_int
            && num_guns as ::core::ffi::c_int >= 2 as ::core::ffi::c_int)
            as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
    NPCPhrase_cb(C2Rust_Unnamed_5::LETS_SEE.0 as ::core::ffi::c_int, None);
    if HasMinimum.0 == 0
        && (getGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_2::CHMMR_BOMB_STATE.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_2::END_CHMMR_BOMB_STATE.0 as ::core::ffi::c_int,
        ) as ::core::ffi::c_int) < 2 as ::core::ffi::c_int
    {
        NPCPhrase_cb(C2Rust_Unnamed_5::IMPROVE_1.0 as ::core::ffi::c_int, None);
        if (num_thrusters as ::core::ffi::c_int) < 7 as ::core::ffi::c_int {
            NPCPhrase_cb(
                C2Rust_Unnamed_5::NEED_THRUSTERS_1.0 as ::core::ffi::c_int,
                None,
            );
        }
        if (num_jets as ::core::ffi::c_int) < 5 as ::core::ffi::c_int {
            NPCPhrase_cb(C2Rust_Unnamed_5::NEED_TURN_1.0 as ::core::ffi::c_int, None);
        }
        if (num_guns as ::core::ffi::c_int) < 2 as ::core::ffi::c_int {
            NPCPhrase_cb(C2Rust_Unnamed_5::NEED_GUNS_1.0 as ::core::ffi::c_int, None);
        }
        if (GlobData.SIS_state.CrewEnlisted as ::core::ffi::c_int)
            < 50 as ::core::ffi::c_int
        {
            NPCPhrase_cb(C2Rust_Unnamed_5::NEED_CREW_1.0 as ::core::ffi::c_int, None);
        }
        if GlobData.SIS_state.FuelOnBoard
            < (50 as ::core::ffi::c_int * 100 as ::core::ffi::c_int) as DWORD
        {
            NPCPhrase_cb(C2Rust_Unnamed_5::NEED_FUEL_1.0 as ::core::ffi::c_int, None);
        }
        if (num_bays as ::core::ffi::c_int) < 1 as ::core::ffi::c_int {
            NPCPhrase_cb(C2Rust_Unnamed_5::NEED_STORAGE_1.0 as ::core::ffi::c_int, None);
        }
        if GlobData.SIS_state.NumLanders as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        {
            NPCPhrase_cb(C2Rust_Unnamed_5::NEED_LANDERS_2.0 as ::core::ffi::c_int, None);
        }
        if (num_batts as ::core::ffi::c_int) < 1 as ::core::ffi::c_int {
            NPCPhrase_cb(C2Rust_Unnamed_5::NEED_DYNAMOS_1.0 as ::core::ffi::c_int, None);
        }
        if GlobData.SIS_state.ResUnits >= 3000 as DWORD {
            NPCPhrase_cb(
                C2Rust_Unnamed_5::IMPROVE_FLAGSHIP_WITH_RU.0 as ::core::ffi::c_int,
                None,
            );
        } else {
            NPCPhrase_cb(
                C2Rust_Unnamed_5::GO_GET_MINERALS.0 as ::core::ffi::c_int,
                None,
            );
        }
    } else {
        let mut num_aliens: BYTE = 0 as BYTE;
        let mut FleetStrength: COUNT = 0;
        let mut HasMaximum: BOOLEAN = BOOLEAN::FALSE;
        FleetStrength = CalculateEscortsWorth();
        i = 0 as BYTE;
        while (i as ::core::ffi::c_int)
            < C2Rust_Unnamed_7::NUM_AVAILABLE_RACES.0 as ::core::ffi::c_int
        {
            if i as ::core::ffi::c_int
                != C2Rust_Unnamed_7::HUMAN_SHIP.0 as ::core::ffi::c_int
                && CheckAlliance(i as COUNT) as ::core::ffi::c_int
                    == C2Rust_Unnamed_6::GOOD_GUY.0 as ::core::ffi::c_int
            {
                num_aliens = num_aliens.wrapping_add(1);
            }
            i = i.wrapping_add(1);
        }
        HasMaximum = BOOLEAN(
            (num_thrusters as ::core::ffi::c_int == 11 as ::core::ffi::c_int
                && num_jets as ::core::ffi::c_int == 8 as ::core::ffi::c_int
                && GlobData.SIS_state.CrewEnlisted as ::core::ffi::c_int
                    >= 50 as ::core::ffi::c_int * 3 as ::core::ffi::c_int
                && GlobData.SIS_state.FuelOnBoard
                    >= (50 as ::core::ffi::c_int * 100 as ::core::ffi::c_int
                        * 3 as ::core::ffi::c_int) as DWORD
                && GlobData.SIS_state.NumLanders as ::core::ffi::c_int
                    >= 3 as ::core::ffi::c_int
                && num_batts as ::core::ffi::c_int >= 4 as ::core::ffi::c_int
                && num_guns as ::core::ffi::c_int >= 7 as ::core::ffi::c_int
                && num_defense as ::core::ffi::c_int >= 2 as ::core::ffi::c_int)
                as ::core::ffi::c_int as ::core::ffi::c_uint,
        );
        if HasMaximum.0 == 0
            && (getGameState(
                &raw mut GlobData.Game_state.GameState as *mut BYTE,
                C2Rust_Unnamed_2::CHMMR_BOMB_STATE.0 as ::core::ffi::c_int,
                C2Rust_Unnamed_2::END_CHMMR_BOMB_STATE.0 as ::core::ffi::c_int,
            ) as ::core::ffi::c_int) < 2 as ::core::ffi::c_int
        {
            NPCPhrase_cb(
                C2Rust_Unnamed_5::GOT_OK_FLAGSHIP.0 as ::core::ffi::c_int,
                None,
            );
        } else {
            NPCPhrase_cb(
                C2Rust_Unnamed_5::GOT_AWESOME_FLAGSHIP.0 as ::core::ffi::c_int,
                None,
            );
        }
        if getGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_2::CHMMR_BOMB_STATE.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_2::END_CHMMR_BOMB_STATE.0 as ::core::ffi::c_int,
        ) as ::core::ffi::c_int >= 2 as ::core::ffi::c_int
        {
            NPCPhrase_cb(
                C2Rust_Unnamed_5::CHMMR_IMPROVED_BOMB.0 as ::core::ffi::c_int,
                None,
            );
            if (FleetStrength as ::core::ffi::c_int) < 20000 as ::core::ffi::c_int {
                NPCPhrase_cb(
                    C2Rust_Unnamed_5::MUST_ACQUIRE_AWESOME_FLEET.0 as ::core::ffi::c_int,
                    None,
                );
            } else {
                NPCPhrase_cb(
                    C2Rust_Unnamed_5::GOT_AWESOME_FLEET.0 as ::core::ffi::c_int,
                    None,
                );
                if getGameState(
                    &raw mut GlobData.Game_state.GameState as *mut BYTE,
                    C2Rust_Unnamed_2::TALKING_PET_ON_SHIP.0 as ::core::ffi::c_int,
                    C2Rust_Unnamed_2::END_TALKING_PET_ON_SHIP.0 as ::core::ffi::c_int,
                ) == 0
                {
                    NPCPhrase_cb(
                        C2Rust_Unnamed_5::MUST_ELIMINATE_URQUAN_GUARDS.0
                            as ::core::ffi::c_int,
                        None,
                    );
                } else {
                    NPCPhrase_cb(
                        C2Rust_Unnamed_5::GO_DESTROY_SAMATRA.0 as ::core::ffi::c_int,
                        None,
                    );
                }
            }
        } else if (num_aliens as ::core::ffi::c_int) < 2 as ::core::ffi::c_int {
            NPCPhrase_cb(
                C2Rust_Unnamed_5::GO_ALLY_WITH_ALIENS.0 as ::core::ffi::c_int,
                None,
            );
        } else {
            NPCPhrase_cb(
                C2Rust_Unnamed_5::MADE_SOME_ALLIES.0 as ::core::ffi::c_int,
                None,
            );
            if (FleetStrength as ::core::ffi::c_int) < 6000 as ::core::ffi::c_int {
                if GlobData.SIS_state.ResUnits >= 3000 as DWORD {
                    NPCPhrase_cb(
                        C2Rust_Unnamed_5::BUY_COMBAT_SHIPS.0 as ::core::ffi::c_int,
                        None,
                    );
                } else {
                    NPCPhrase_cb(
                        C2Rust_Unnamed_5::GET_SHIPS_BY_MINING_OR_ALLIANCE.0
                            as ::core::ffi::c_int,
                        None,
                    );
                }
            } else {
                NPCPhrase_cb(
                    C2Rust_Unnamed_5::GOT_OK_FLEET.0 as ::core::ffi::c_int,
                    None,
                );
                if HasMaximum.0 == 0 {
                    NPCPhrase_cb(
                        C2Rust_Unnamed_5::IMPROVE_2.0 as ::core::ffi::c_int,
                        None,
                    );
                    if (num_thrusters as ::core::ffi::c_int) < 11 as ::core::ffi::c_int {
                        NPCPhrase_cb(
                            C2Rust_Unnamed_5::NEED_THRUSTERS_2.0 as ::core::ffi::c_int,
                            None,
                        );
                    }
                    if (num_jets as ::core::ffi::c_int) < 8 as ::core::ffi::c_int {
                        NPCPhrase_cb(
                            C2Rust_Unnamed_5::NEED_TURN_2.0 as ::core::ffi::c_int,
                            None,
                        );
                    }
                    if (num_guns as ::core::ffi::c_int) < 7 as ::core::ffi::c_int {
                        NPCPhrase_cb(
                            C2Rust_Unnamed_5::NEED_GUNS_2.0 as ::core::ffi::c_int,
                            None,
                        );
                    }
                    if (GlobData.SIS_state.CrewEnlisted as ::core::ffi::c_int)
                        < 50 as ::core::ffi::c_int * 3 as ::core::ffi::c_int
                    {
                        NPCPhrase_cb(
                            C2Rust_Unnamed_5::NEED_CREW_2.0 as ::core::ffi::c_int,
                            None,
                        );
                    }
                    if GlobData.SIS_state.FuelOnBoard
                        < (50 as ::core::ffi::c_int * 100 as ::core::ffi::c_int
                            * 3 as ::core::ffi::c_int) as DWORD
                    {
                        NPCPhrase_cb(
                            C2Rust_Unnamed_5::NEED_FUEL_2.0 as ::core::ffi::c_int,
                            None,
                        );
                    }
                    if (GlobData.SIS_state.NumLanders as ::core::ffi::c_int)
                        < 3 as ::core::ffi::c_int
                    {
                        NPCPhrase_cb(
                            C2Rust_Unnamed_5::NEED_LANDERS_1.0 as ::core::ffi::c_int,
                            None,
                        );
                    }
                    if (num_batts as ::core::ffi::c_int) < 4 as ::core::ffi::c_int {
                        NPCPhrase_cb(
                            C2Rust_Unnamed_5::NEED_DYNAMOS_2.0 as ::core::ffi::c_int,
                            None,
                        );
                    }
                    if (num_defense as ::core::ffi::c_int) < 2 as ::core::ffi::c_int {
                        NPCPhrase_cb(
                            C2Rust_Unnamed_5::NEED_POINT.0 as ::core::ffi::c_int,
                            None,
                        );
                    }
                } else if getGameState(
                    &raw mut GlobData.Game_state.GameState as *mut BYTE,
                    C2Rust_Unnamed_2::AWARE_OF_SAMATRA.0 as ::core::ffi::c_int,
                    C2Rust_Unnamed_2::END_AWARE_OF_SAMATRA.0 as ::core::ffi::c_int,
                ) == 0
                {
                    NPCPhrase_cb(
                        C2Rust_Unnamed_5::GO_LEARN_ABOUT_URQUAN.0 as ::core::ffi::c_int,
                        None,
                    );
                } else {
                    NPCPhrase_cb(
                        C2Rust_Unnamed_5::KNOW_ABOUT_SAMATRA.0 as ::core::ffi::c_int,
                        None,
                    );
                    if getGameState(
                        &raw mut GlobData.Game_state.GameState as *mut BYTE,
                        C2Rust_Unnamed_2::UTWIG_BOMB.0 as ::core::ffi::c_int,
                        C2Rust_Unnamed_2::END_UTWIG_BOMB.0 as ::core::ffi::c_int,
                    ) == 0
                    {
                        NPCPhrase_cb(
                            C2Rust_Unnamed_5::FIND_WAY_TO_DESTROY_SAMATRA.0
                                as ::core::ffi::c_int,
                            None,
                        );
                    } else if getGameState(
                        &raw mut GlobData.Game_state.GameState as *mut BYTE,
                        C2Rust_Unnamed_2::UTWIG_BOMB_ON_SHIP.0 as ::core::ffi::c_int,
                        C2Rust_Unnamed_2::END_UTWIG_BOMB_ON_SHIP.0 as ::core::ffi::c_int,
                    ) != 0
                    {
                        NPCPhrase_cb(
                            C2Rust_Unnamed_5::MUST_INCREASE_BOMB_STRENGTH.0
                                as ::core::ffi::c_int,
                            None,
                        );
                    }
                }
            }
        }
    };
}
unsafe extern "C" fn TellMission(mut R: RESPONSE_REF) {
    let mut pstack: [RESPONSE_REF; 4] = [0; 4];
    if R as ::core::ffi::c_int == C2Rust_Unnamed_5::our_mission.0 as ::core::ffi::c_int {
        NPCPhrase_cb(C2Rust_Unnamed_5::WHICH_MISSION.0 as ::core::ffi::c_int, None);
        stack0 = 0 as BYTE;
        stack1 = 0 as BYTE;
        stack2 = 0 as BYTE;
        stack3 = 0 as BYTE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_5::where_get_minerals.0 as ::core::ffi::c_int
    {
        NPCPhrase_cb(C2Rust_Unnamed_5::GET_MINERALS.0 as ::core::ffi::c_int, None);
        stack0 = 1 as BYTE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_5::what_about_aliens.0 as ::core::ffi::c_int
    {
        NPCPhrase_cb(C2Rust_Unnamed_5::ABOUT_ALIENS.0 as ::core::ffi::c_int, None);
        stack1 = 1 as BYTE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_5::what_do_now.0 as ::core::ffi::c_int
    {
        AnalyzeCondition();
        stack2 = 1 as BYTE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_5::what_about_urquan.0 as ::core::ffi::c_int
    {
        NPCPhrase_cb(C2Rust_Unnamed_5::MUST_DEFEAT.0 as ::core::ffi::c_int, None);
        stack3 = 1 as BYTE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_5::enough_defeat.0 as ::core::ffi::c_int
    {
        NPCPhrase_cb(C2Rust_Unnamed_5::OK_ENOUGH_DEFEAT.0 as ::core::ffi::c_int, None);
        stack3 = 2 as BYTE;
    }
    match stack0 as ::core::ffi::c_int {
        0 => {
            pstack[0usize] = C2Rust_Unnamed_5::where_get_minerals.0 as ::core::ffi::c_int
                as RESPONSE_REF;
        }
        _ => {
            pstack[0usize] = 0 as RESPONSE_REF;
        }
    }
    match stack1 as ::core::ffi::c_int {
        0 => {
            pstack[1usize] = C2Rust_Unnamed_5::what_about_aliens.0 as ::core::ffi::c_int
                as RESPONSE_REF;
        }
        _ => {
            pstack[1usize] = 0 as RESPONSE_REF;
        }
    }
    match stack2 as ::core::ffi::c_int {
        0 => {
            pstack[2usize] = C2Rust_Unnamed_5::what_do_now.0 as ::core::ffi::c_int
                as RESPONSE_REF;
        }
        _ => {
            pstack[2usize] = 0 as RESPONSE_REF;
        }
    }
    match stack3 as ::core::ffi::c_int {
        0 => {
            pstack[3usize] = C2Rust_Unnamed_5::what_about_urquan.0 as ::core::ffi::c_int
                as RESPONSE_REF;
        }
        1 => {
            pstack[3usize] = C2Rust_Unnamed_5::how_defeat.0 as ::core::ffi::c_int
                as RESPONSE_REF;
        }
        _ => {
            pstack[3usize] = 0 as RESPONSE_REF;
        }
    }
    if pstack[0usize] != 0 {
        DoResponsePhrase(
            pstack[0usize],
            Some(TellMission as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        );
    }
    if pstack[1usize] != 0 {
        DoResponsePhrase(
            pstack[1usize],
            Some(TellMission as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        );
    }
    if pstack[2usize] != 0 {
        DoResponsePhrase(
            pstack[2usize],
            Some(TellMission as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        );
    }
    if pstack[3usize] != 0 {
        if stack3 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            DoResponsePhrase(
                pstack[3usize],
                Some(DefeatUrquan as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            );
        } else {
            DoResponsePhrase(
                pstack[3usize],
                Some(TellMission as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            );
        }
    }
    DoResponsePhrase(
        C2Rust_Unnamed_5::enough_mission.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(NeedInfo as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    );
}
unsafe extern "C" fn TellStarBase(mut R: RESPONSE_REF) {
    let mut pstack: [RESPONSE_REF; 4] = [0; 4];
    static mut buf0: [UNICODE; 80] = [0; 80];
    if R as ::core::ffi::c_int
        == C2Rust_Unnamed_5::starbase_functions.0 as ::core::ffi::c_int
    {
        NPCPhrase_cb(C2Rust_Unnamed_5::WHICH_FUNCTION.0 as ::core::ffi::c_int, None);
        stack0 = 0 as BYTE;
        stack1 = 0 as BYTE;
        stack2 = 0 as BYTE;
        stack3 = 0 as BYTE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_5::tell_me_about_fuel0.0 as ::core::ffi::c_int
    {
        NPCPhrase_cb(C2Rust_Unnamed_5::ABOUT_FUEL.0 as ::core::ffi::c_int, None);
        stack1 = 1 as BYTE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_5::tell_me_about_crew.0 as ::core::ffi::c_int
    {
        NPCPhrase_cb(C2Rust_Unnamed_5::ABOUT_CREW0.0 as ::core::ffi::c_int, None);
        if usingSpeech.0 != 0 {
            NPCPhrase_cb(
                C2Rust_Unnamed_5::YOUR_FLAGSHIP_3DO2.0 as ::core::ffi::c_int,
                None,
            );
        } else {
            NPCPhrase_cb(
                C2Rust_Unnamed_5::YOUR_FLAGSHIP_PC.0 as ::core::ffi::c_int,
                None,
            );
            NPCPhrase_cb(C2Rust_Unnamed_4::GLOBAL_SHIP_NAME.0, None);
        }
        NPCPhrase_cb(C2Rust_Unnamed_5::ABOUT_CREW1.0 as ::core::ffi::c_int, None);
        stack2 = 2 as BYTE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_5::tell_me_about_modules0.0 as ::core::ffi::c_int
    {
        NPCPhrase_cb(C2Rust_Unnamed_5::ABOUT_MODULES.0 as ::core::ffi::c_int, None);
        stack0 = 1 as BYTE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_5::tell_me_about_ships.0 as ::core::ffi::c_int
    {
        NPCPhrase_cb(C2Rust_Unnamed_5::ABOUT_SHIPS.0 as ::core::ffi::c_int, None);
        stack2 = 1 as BYTE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_5::tell_me_about_ru.0 as ::core::ffi::c_int
    {
        NPCPhrase_cb(C2Rust_Unnamed_5::ABOUT_RU.0 as ::core::ffi::c_int, None);
        stack3 = 1 as BYTE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_5::tell_me_about_minerals.0 as ::core::ffi::c_int
    {
        NPCPhrase_cb(C2Rust_Unnamed_5::ABOUT_MINERALS.0 as ::core::ffi::c_int, None);
        stack3 = 2 as BYTE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_5::tell_me_about_life.0 as ::core::ffi::c_int
    {
        NPCPhrase_cb(C2Rust_Unnamed_5::ABOUT_LIFE.0 as ::core::ffi::c_int, None);
        stack3 = 3 as BYTE;
    }
    match stack0 as ::core::ffi::c_int {
        0 => {
            construct_response(
                &raw mut buf0 as *mut UNICODE,
                C2Rust_Unnamed_5::tell_me_about_modules0.0 as ::core::ffi::c_int,
                &raw mut GlobData.SIS_state.ShipName as *mut UNICODE,
                C2Rust_Unnamed_5::tell_me_about_modules1.0 as ::core::ffi::c_int,
                ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut UNICODE,
            );
            pstack[0usize] = C2Rust_Unnamed_5::tell_me_about_modules0.0
                as ::core::ffi::c_int as RESPONSE_REF;
        }
        _ => {
            pstack[0usize] = 0 as RESPONSE_REF;
        }
    }
    match stack1 as ::core::ffi::c_int {
        0 => {
            construct_response(
                &raw mut shared_phrase_buf as *mut UNICODE,
                C2Rust_Unnamed_5::tell_me_about_fuel0.0 as ::core::ffi::c_int,
                &raw mut GlobData.SIS_state.ShipName as *mut UNICODE,
                C2Rust_Unnamed_5::tell_me_about_fuel1.0 as ::core::ffi::c_int,
                ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut UNICODE,
            );
            pstack[1usize] = C2Rust_Unnamed_5::tell_me_about_fuel0.0
                as ::core::ffi::c_int as RESPONSE_REF;
        }
        _ => {
            pstack[1usize] = 0 as RESPONSE_REF;
        }
    }
    match stack2 as ::core::ffi::c_int {
        0 => {
            pstack[2usize] = C2Rust_Unnamed_5::tell_me_about_ships.0
                as ::core::ffi::c_int as RESPONSE_REF;
        }
        1 => {
            pstack[2usize] = C2Rust_Unnamed_5::tell_me_about_crew.0 as ::core::ffi::c_int
                as RESPONSE_REF;
        }
        _ => {
            pstack[2usize] = 0 as RESPONSE_REF;
        }
    }
    match stack3 as ::core::ffi::c_int {
        0 => {
            pstack[3usize] = C2Rust_Unnamed_5::tell_me_about_ru.0 as ::core::ffi::c_int
                as RESPONSE_REF;
        }
        1 => {
            pstack[3usize] = C2Rust_Unnamed_5::tell_me_about_minerals.0
                as ::core::ffi::c_int as RESPONSE_REF;
        }
        2 => {
            pstack[3usize] = C2Rust_Unnamed_5::tell_me_about_life.0 as ::core::ffi::c_int
                as RESPONSE_REF;
        }
        _ => {
            pstack[3usize] = 0 as RESPONSE_REF;
        }
    }
    if pstack[0usize] != 0 {
        DoResponsePhrase(
            pstack[0usize],
            Some(TellStarBase as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            &raw mut buf0 as *mut UNICODE,
        );
    }
    if pstack[1usize] != 0 {
        DoResponsePhrase(
            pstack[1usize],
            Some(TellStarBase as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            &raw mut shared_phrase_buf as *mut UNICODE,
        );
    }
    if pstack[2usize] != 0 {
        DoResponsePhrase(
            pstack[2usize],
            Some(TellStarBase as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        );
    }
    if pstack[3usize] != 0 {
        DoResponsePhrase(
            pstack[3usize],
            Some(TellStarBase as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        );
    }
    DoResponsePhrase(
        C2Rust_Unnamed_5::enough_starbase.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(NeedInfo as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    );
}
unsafe extern "C" fn NeedInfo(mut R: RESPONSE_REF) {
    if R as ::core::ffi::c_int == C2Rust_Unnamed_5::need_info.0 as ::core::ffi::c_int {
        NPCPhrase_cb(C2Rust_Unnamed_5::WHAT_KIND_OF_INFO.0 as ::core::ffi::c_int, None);
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_5::enough_starbase.0 as ::core::ffi::c_int
    {
        NPCPhrase_cb(C2Rust_Unnamed_5::OK_ENOUGH_STARBASE.0 as ::core::ffi::c_int, None);
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_5::enough_history.0 as ::core::ffi::c_int
    {
        NPCPhrase_cb(C2Rust_Unnamed_5::OK_ENOUGH_HISTORY.0 as ::core::ffi::c_int, None);
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_5::enough_mission.0 as ::core::ffi::c_int
    {
        NPCPhrase_cb(C2Rust_Unnamed_5::OK_ENOUGH_MISSION.0 as ::core::ffi::c_int, None);
    }
    DoResponsePhrase(
        C2Rust_Unnamed_5::starbase_functions.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(TellStarBase as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    );
    DoResponsePhrase(
        C2Rust_Unnamed_5::history.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(TellHistory as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    );
    DoResponsePhrase(
        C2Rust_Unnamed_5::our_mission.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(TellMission as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    );
    DoResponsePhrase(
        C2Rust_Unnamed_5::no_need_info.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(NormalStarbase as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    );
}
unsafe extern "C" fn DiscussDevices(mut TalkAbout: BOOLEAN) -> BOOLEAN {
    let mut i: COUNT = 0;
    let mut VuxBeastIndex: COUNT = 0;
    let mut PhraseIndex: COUNT = 0;
    let mut Undiscussed: BOOLEAN = BOOLEAN::FALSE;
    if TalkAbout.0 != 0 {
        NPCPhrase_cb(C2Rust_Unnamed_5::DEVICE_HEAD.0 as ::core::ffi::c_int, None);
    }
    PhraseIndex = 2 as COUNT;
    VuxBeastIndex = 0 as COUNT;
    Undiscussed = BOOLEAN::FALSE;
    i = 0 as COUNT;
    while (i as ::core::ffi::c_int)
        < C2Rust_Unnamed_1::NUM_DEVICES.0 as ::core::ffi::c_int
    {
        let mut pStr: RESPONSE_REF = 0;
        pStr = 0 as RESPONSE_REF;
        match i as ::core::ffi::c_int {
            4 => {
                if getGameState(
                    &raw mut GlobData.Game_state.GameState as *mut BYTE,
                    C2Rust_Unnamed_2::ROSY_SPHERE_ON_SHIP.0 as ::core::ffi::c_int,
                    C2Rust_Unnamed_2::END_ROSY_SPHERE_ON_SHIP.0 as ::core::ffi::c_int,
                ) as ::core::ffi::c_int != 0
                    && getGameState(
                        &raw mut GlobData.Game_state.GameState as *mut BYTE,
                        C2Rust_Unnamed_2::DISCUSSED_ROSY_SPHERE.0 as ::core::ffi::c_int,
                        C2Rust_Unnamed_2::END_DISCUSSED_ROSY_SPHERE.0
                            as ::core::ffi::c_int,
                    ) == 0
                {
                    pStr = C2Rust_Unnamed_5::ABOUT_SPHERE.0 as ::core::ffi::c_int
                        as RESPONSE_REF;
                    setGameState(
                        &raw mut GlobData.Game_state.GameState as *mut BYTE,
                        C2Rust_Unnamed_2::DISCUSSED_ROSY_SPHERE.0 as ::core::ffi::c_int,
                        C2Rust_Unnamed_2::END_DISCUSSED_ROSY_SPHERE.0
                            as ::core::ffi::c_int,
                        TalkAbout.0 as BYTE,
                    );
                }
            }
            25 => {
                if getGameState(
                    &raw mut GlobData.Game_state.GameState as *mut BYTE,
                    C2Rust_Unnamed_2::ARTIFACT_2_ON_SHIP.0 as ::core::ffi::c_int,
                    C2Rust_Unnamed_2::END_ARTIFACT_2_ON_SHIP.0 as ::core::ffi::c_int,
                ) as ::core::ffi::c_int != 0
                    && getGameState(
                        &raw mut GlobData.Game_state.GameState as *mut BYTE,
                        C2Rust_Unnamed_2::DISCUSSED_ARTIFACT_2.0 as ::core::ffi::c_int,
                        C2Rust_Unnamed_2::END_DISCUSSED_ARTIFACT_2.0
                            as ::core::ffi::c_int,
                    ) == 0
                {
                    pStr = C2Rust_Unnamed_5::ABOUT_ARTIFACT_2.0 as ::core::ffi::c_int
                        as RESPONSE_REF;
                    setGameState(
                        &raw mut GlobData.Game_state.GameState as *mut BYTE,
                        C2Rust_Unnamed_2::DISCUSSED_ARTIFACT_2.0 as ::core::ffi::c_int,
                        C2Rust_Unnamed_2::END_DISCUSSED_ARTIFACT_2.0
                            as ::core::ffi::c_int,
                        TalkAbout.0 as BYTE,
                    );
                }
            }
            26 => {
                if getGameState(
                    &raw mut GlobData.Game_state.GameState as *mut BYTE,
                    C2Rust_Unnamed_2::ARTIFACT_3_ON_SHIP.0 as ::core::ffi::c_int,
                    C2Rust_Unnamed_2::END_ARTIFACT_3_ON_SHIP.0 as ::core::ffi::c_int,
                ) as ::core::ffi::c_int != 0
                    && getGameState(
                        &raw mut GlobData.Game_state.GameState as *mut BYTE,
                        C2Rust_Unnamed_2::DISCUSSED_ARTIFACT_3.0 as ::core::ffi::c_int,
                        C2Rust_Unnamed_2::END_DISCUSSED_ARTIFACT_3.0
                            as ::core::ffi::c_int,
                    ) == 0
                {
                    pStr = C2Rust_Unnamed_5::ABOUT_ARTIFACT_3.0 as ::core::ffi::c_int
                        as RESPONSE_REF;
                    setGameState(
                        &raw mut GlobData.Game_state.GameState as *mut BYTE,
                        C2Rust_Unnamed_2::DISCUSSED_ARTIFACT_3.0 as ::core::ffi::c_int,
                        C2Rust_Unnamed_2::END_DISCUSSED_ARTIFACT_3.0
                            as ::core::ffi::c_int,
                        TalkAbout.0 as BYTE,
                    );
                }
            }
            3 => {
                if getGameState(
                    &raw mut GlobData.Game_state.GameState as *mut BYTE,
                    C2Rust_Unnamed_2::SUN_DEVICE_ON_SHIP.0 as ::core::ffi::c_int,
                    C2Rust_Unnamed_2::END_SUN_DEVICE_ON_SHIP.0 as ::core::ffi::c_int,
                ) as ::core::ffi::c_int != 0
                    && getGameState(
                        &raw mut GlobData.Game_state.GameState as *mut BYTE,
                        C2Rust_Unnamed_2::DISCUSSED_SUN_EFFICIENCY.0
                            as ::core::ffi::c_int,
                        C2Rust_Unnamed_2::END_DISCUSSED_SUN_EFFICIENCY.0
                            as ::core::ffi::c_int,
                    ) == 0
                {
                    pStr = C2Rust_Unnamed_5::ABOUT_SUN.0 as ::core::ffi::c_int
                        as RESPONSE_REF;
                    setGameState(
                        &raw mut GlobData.Game_state.GameState as *mut BYTE,
                        C2Rust_Unnamed_2::DISCUSSED_SUN_EFFICIENCY.0
                            as ::core::ffi::c_int,
                        C2Rust_Unnamed_2::END_DISCUSSED_SUN_EFFICIENCY.0
                            as ::core::ffi::c_int,
                        TalkAbout.0 as BYTE,
                    );
                }
            }
            2 => {
                if getGameState(
                    &raw mut GlobData.Game_state.GameState as *mut BYTE,
                    C2Rust_Unnamed_2::UTWIG_BOMB_ON_SHIP.0 as ::core::ffi::c_int,
                    C2Rust_Unnamed_2::END_UTWIG_BOMB_ON_SHIP.0 as ::core::ffi::c_int,
                ) as ::core::ffi::c_int != 0
                    && getGameState(
                        &raw mut GlobData.Game_state.GameState as *mut BYTE,
                        C2Rust_Unnamed_2::DISCUSSED_UTWIG_BOMB.0 as ::core::ffi::c_int,
                        C2Rust_Unnamed_2::END_DISCUSSED_UTWIG_BOMB.0
                            as ::core::ffi::c_int,
                    ) == 0
                {
                    pStr = C2Rust_Unnamed_5::ABOUT_BOMB.0 as ::core::ffi::c_int
                        as RESPONSE_REF;
                    setGameState(
                        &raw mut GlobData.Game_state.GameState as *mut BYTE,
                        C2Rust_Unnamed_2::DISCUSSED_UTWIG_BOMB.0 as ::core::ffi::c_int,
                        C2Rust_Unnamed_2::END_DISCUSSED_UTWIG_BOMB.0
                            as ::core::ffi::c_int,
                        TalkAbout.0 as BYTE,
                    );
                }
            }
            7 => {
                if getGameState(
                    &raw mut GlobData.Game_state.GameState as *mut BYTE,
                    C2Rust_Unnamed_2::ULTRON_CONDITION.0 as ::core::ffi::c_int,
                    C2Rust_Unnamed_2::END_ULTRON_CONDITION.0 as ::core::ffi::c_int,
                ) as ::core::ffi::c_int == 1 as ::core::ffi::c_int
                    && getGameState(
                        &raw mut GlobData.Game_state.GameState as *mut BYTE,
                        C2Rust_Unnamed_2::DISCUSSED_ULTRON.0 as ::core::ffi::c_int,
                        C2Rust_Unnamed_2::END_DISCUSSED_ULTRON.0 as ::core::ffi::c_int,
                    ) == 0
                {
                    pStr = C2Rust_Unnamed_5::ABOUT_ULTRON_0.0 as ::core::ffi::c_int
                        as RESPONSE_REF;
                    setGameState(
                        &raw mut GlobData.Game_state.GameState as *mut BYTE,
                        C2Rust_Unnamed_2::DISCUSSED_ULTRON.0 as ::core::ffi::c_int,
                        C2Rust_Unnamed_2::END_DISCUSSED_ULTRON.0 as ::core::ffi::c_int,
                        TalkAbout.0 as BYTE,
                    );
                }
            }
            8 => {
                if getGameState(
                    &raw mut GlobData.Game_state.GameState as *mut BYTE,
                    C2Rust_Unnamed_2::ULTRON_CONDITION.0 as ::core::ffi::c_int,
                    C2Rust_Unnamed_2::END_ULTRON_CONDITION.0 as ::core::ffi::c_int,
                ) as ::core::ffi::c_int == 2 as ::core::ffi::c_int
                    && getGameState(
                        &raw mut GlobData.Game_state.GameState as *mut BYTE,
                        C2Rust_Unnamed_2::DISCUSSED_ULTRON.0 as ::core::ffi::c_int,
                        C2Rust_Unnamed_2::END_DISCUSSED_ULTRON.0 as ::core::ffi::c_int,
                    ) == 0
                {
                    pStr = C2Rust_Unnamed_5::ABOUT_ULTRON_1.0 as ::core::ffi::c_int
                        as RESPONSE_REF;
                    setGameState(
                        &raw mut GlobData.Game_state.GameState as *mut BYTE,
                        C2Rust_Unnamed_2::DISCUSSED_ULTRON.0 as ::core::ffi::c_int,
                        C2Rust_Unnamed_2::END_DISCUSSED_ULTRON.0 as ::core::ffi::c_int,
                        TalkAbout.0 as BYTE,
                    );
                }
            }
            9 => {
                if getGameState(
                    &raw mut GlobData.Game_state.GameState as *mut BYTE,
                    C2Rust_Unnamed_2::ULTRON_CONDITION.0 as ::core::ffi::c_int,
                    C2Rust_Unnamed_2::END_ULTRON_CONDITION.0 as ::core::ffi::c_int,
                ) as ::core::ffi::c_int == 3 as ::core::ffi::c_int
                    && getGameState(
                        &raw mut GlobData.Game_state.GameState as *mut BYTE,
                        C2Rust_Unnamed_2::DISCUSSED_ULTRON.0 as ::core::ffi::c_int,
                        C2Rust_Unnamed_2::END_DISCUSSED_ULTRON.0 as ::core::ffi::c_int,
                    ) == 0
                {
                    pStr = C2Rust_Unnamed_5::ABOUT_ULTRON_2.0 as ::core::ffi::c_int
                        as RESPONSE_REF;
                    setGameState(
                        &raw mut GlobData.Game_state.GameState as *mut BYTE,
                        C2Rust_Unnamed_2::DISCUSSED_ULTRON.0 as ::core::ffi::c_int,
                        C2Rust_Unnamed_2::END_DISCUSSED_ULTRON.0 as ::core::ffi::c_int,
                        TalkAbout.0 as BYTE,
                    );
                }
            }
            10 => {
                if getGameState(
                    &raw mut GlobData.Game_state.GameState as *mut BYTE,
                    C2Rust_Unnamed_2::ULTRON_CONDITION.0 as ::core::ffi::c_int,
                    C2Rust_Unnamed_2::END_ULTRON_CONDITION.0 as ::core::ffi::c_int,
                ) as ::core::ffi::c_int == 4 as ::core::ffi::c_int
                    && getGameState(
                        &raw mut GlobData.Game_state.GameState as *mut BYTE,
                        C2Rust_Unnamed_2::DISCUSSED_ULTRON.0 as ::core::ffi::c_int,
                        C2Rust_Unnamed_2::END_DISCUSSED_ULTRON.0 as ::core::ffi::c_int,
                    ) == 0
                {
                    pStr = C2Rust_Unnamed_5::ABOUT_ULTRON_3.0 as ::core::ffi::c_int
                        as RESPONSE_REF;
                    setGameState(
                        &raw mut GlobData.Game_state.GameState as *mut BYTE,
                        C2Rust_Unnamed_2::DISCUSSED_ULTRON.0 as ::core::ffi::c_int,
                        C2Rust_Unnamed_2::END_DISCUSSED_ULTRON.0 as ::core::ffi::c_int,
                        TalkAbout.0 as BYTE,
                    );
                }
            }
            11 => {
                if getGameState(
                    &raw mut GlobData.Game_state.GameState as *mut BYTE,
                    C2Rust_Unnamed_2::MAIDENS_ON_SHIP.0 as ::core::ffi::c_int,
                    C2Rust_Unnamed_2::END_MAIDENS_ON_SHIP.0 as ::core::ffi::c_int,
                ) as ::core::ffi::c_int != 0
                    && getGameState(
                        &raw mut GlobData.Game_state.GameState as *mut BYTE,
                        C2Rust_Unnamed_2::DISCUSSED_MAIDENS.0 as ::core::ffi::c_int,
                        C2Rust_Unnamed_2::END_DISCUSSED_MAIDENS.0 as ::core::ffi::c_int,
                    ) == 0
                {
                    pStr = C2Rust_Unnamed_5::ABOUT_MAIDENS.0 as ::core::ffi::c_int
                        as RESPONSE_REF;
                    setGameState(
                        &raw mut GlobData.Game_state.GameState as *mut BYTE,
                        C2Rust_Unnamed_2::DISCUSSED_MAIDENS.0 as ::core::ffi::c_int,
                        C2Rust_Unnamed_2::END_DISCUSSED_MAIDENS.0 as ::core::ffi::c_int,
                        TalkAbout.0 as BYTE,
                    );
                }
            }
            1 => {
                if getGameState(
                    &raw mut GlobData.Game_state.GameState as *mut BYTE,
                    C2Rust_Unnamed_2::TALKING_PET_ON_SHIP.0 as ::core::ffi::c_int,
                    C2Rust_Unnamed_2::END_TALKING_PET_ON_SHIP.0 as ::core::ffi::c_int,
                ) as ::core::ffi::c_int != 0
                    && getGameState(
                        &raw mut GlobData.Game_state.GameState as *mut BYTE,
                        C2Rust_Unnamed_2::DISCUSSED_TALKING_PET.0 as ::core::ffi::c_int,
                        C2Rust_Unnamed_2::END_DISCUSSED_TALKING_PET.0
                            as ::core::ffi::c_int,
                    ) == 0
                {
                    pStr = C2Rust_Unnamed_5::ABOUT_TALKPET.0 as ::core::ffi::c_int
                        as RESPONSE_REF;
                    setGameState(
                        &raw mut GlobData.Game_state.GameState as *mut BYTE,
                        C2Rust_Unnamed_2::DISCUSSED_TALKING_PET.0 as ::core::ffi::c_int,
                        C2Rust_Unnamed_2::END_DISCUSSED_TALKING_PET.0
                            as ::core::ffi::c_int,
                        TalkAbout.0 as BYTE,
                    );
                }
            }
            5 => {
                if getGameState(
                    &raw mut GlobData.Game_state.GameState as *mut BYTE,
                    C2Rust_Unnamed_2::AQUA_HELIX_ON_SHIP.0 as ::core::ffi::c_int,
                    C2Rust_Unnamed_2::END_AQUA_HELIX_ON_SHIP.0 as ::core::ffi::c_int,
                ) as ::core::ffi::c_int != 0
                    && getGameState(
                        &raw mut GlobData.Game_state.GameState as *mut BYTE,
                        C2Rust_Unnamed_2::DISCUSSED_AQUA_HELIX.0 as ::core::ffi::c_int,
                        C2Rust_Unnamed_2::END_DISCUSSED_AQUA_HELIX.0
                            as ::core::ffi::c_int,
                    ) == 0
                {
                    pStr = C2Rust_Unnamed_5::ABOUT_HELIX.0 as ::core::ffi::c_int
                        as RESPONSE_REF;
                    setGameState(
                        &raw mut GlobData.Game_state.GameState as *mut BYTE,
                        C2Rust_Unnamed_2::DISCUSSED_AQUA_HELIX.0 as ::core::ffi::c_int,
                        C2Rust_Unnamed_2::END_DISCUSSED_AQUA_HELIX.0
                            as ::core::ffi::c_int,
                        TalkAbout.0 as BYTE,
                    );
                }
            }
            6 => {
                if getGameState(
                    &raw mut GlobData.Game_state.GameState as *mut BYTE,
                    C2Rust_Unnamed_2::CLEAR_SPINDLE_ON_SHIP.0 as ::core::ffi::c_int,
                    C2Rust_Unnamed_2::END_CLEAR_SPINDLE_ON_SHIP.0 as ::core::ffi::c_int,
                ) as ::core::ffi::c_int != 0
                    && getGameState(
                        &raw mut GlobData.Game_state.GameState as *mut BYTE,
                        C2Rust_Unnamed_2::DISCUSSED_CLEAR_SPINDLE.0
                            as ::core::ffi::c_int,
                        C2Rust_Unnamed_2::END_DISCUSSED_CLEAR_SPINDLE.0
                            as ::core::ffi::c_int,
                    ) == 0
                {
                    pStr = C2Rust_Unnamed_5::ABOUT_SPINDLE.0 as ::core::ffi::c_int
                        as RESPONSE_REF;
                    setGameState(
                        &raw mut GlobData.Game_state.GameState as *mut BYTE,
                        C2Rust_Unnamed_2::DISCUSSED_CLEAR_SPINDLE.0
                            as ::core::ffi::c_int,
                        C2Rust_Unnamed_2::END_DISCUSSED_CLEAR_SPINDLE.0
                            as ::core::ffi::c_int,
                        TalkAbout.0 as BYTE,
                    );
                }
            }
            12 => {
                if getGameState(
                    &raw mut GlobData.Game_state.GameState as *mut BYTE,
                    C2Rust_Unnamed_2::UMGAH_BROADCASTERS_ON_SHIP.0 as ::core::ffi::c_int,
                    C2Rust_Unnamed_2::END_UMGAH_BROADCASTERS_ON_SHIP.0
                        as ::core::ffi::c_int,
                ) as ::core::ffi::c_int != 0
                    && getGameState(
                        &raw mut GlobData.Game_state.GameState as *mut BYTE,
                        C2Rust_Unnamed_2::DISCUSSED_UMGAH_HYPERWAVE.0
                            as ::core::ffi::c_int,
                        C2Rust_Unnamed_2::END_DISCUSSED_UMGAH_HYPERWAVE.0
                            as ::core::ffi::c_int,
                    ) == 0
                {
                    pStr = C2Rust_Unnamed_5::ABOUT_UCASTER.0 as ::core::ffi::c_int
                        as RESPONSE_REF;
                    setGameState(
                        &raw mut GlobData.Game_state.GameState as *mut BYTE,
                        C2Rust_Unnamed_2::DISCUSSED_UMGAH_HYPERWAVE.0
                            as ::core::ffi::c_int,
                        C2Rust_Unnamed_2::END_DISCUSSED_UMGAH_HYPERWAVE.0
                            as ::core::ffi::c_int,
                        TalkAbout.0 as BYTE,
                    );
                }
            }
            17 => {
                if getGameState(
                    &raw mut GlobData.Game_state.GameState as *mut BYTE,
                    C2Rust_Unnamed_2::TAALO_PROTECTOR_ON_SHIP.0 as ::core::ffi::c_int,
                    C2Rust_Unnamed_2::END_TAALO_PROTECTOR_ON_SHIP.0 as ::core::ffi::c_int,
                ) as ::core::ffi::c_int != 0
                    && getGameState(
                        &raw mut GlobData.Game_state.GameState as *mut BYTE,
                        C2Rust_Unnamed_2::DISCUSSED_TAALO_PROTECTOR.0
                            as ::core::ffi::c_int,
                        C2Rust_Unnamed_2::END_DISCUSSED_TAALO_PROTECTOR.0
                            as ::core::ffi::c_int,
                    ) == 0
                {
                    pStr = C2Rust_Unnamed_5::ABOUT_SHIELD.0 as ::core::ffi::c_int
                        as RESPONSE_REF;
                    setGameState(
                        &raw mut GlobData.Game_state.GameState as *mut BYTE,
                        C2Rust_Unnamed_2::DISCUSSED_TAALO_PROTECTOR.0
                            as ::core::ffi::c_int,
                        C2Rust_Unnamed_2::END_DISCUSSED_TAALO_PROTECTOR.0
                            as ::core::ffi::c_int,
                        TalkAbout.0 as BYTE,
                    );
                }
            }
            18 => {
                if getGameState(
                    &raw mut GlobData.Game_state.GameState as *mut BYTE,
                    C2Rust_Unnamed_2::EGG_CASE0_ON_SHIP.0 as ::core::ffi::c_int,
                    C2Rust_Unnamed_2::END_EGG_CASE0_ON_SHIP.0 as ::core::ffi::c_int,
                ) as ::core::ffi::c_int != 0
                    && getGameState(
                        &raw mut GlobData.Game_state.GameState as *mut BYTE,
                        C2Rust_Unnamed_2::DISCUSSED_EGG_CASING0.0 as ::core::ffi::c_int,
                        C2Rust_Unnamed_2::END_DISCUSSED_EGG_CASING0.0
                            as ::core::ffi::c_int,
                    ) == 0
                {
                    pStr = C2Rust_Unnamed_5::ABOUT_EGGCASE_0.0 as ::core::ffi::c_int
                        as RESPONSE_REF;
                    setGameState(
                        &raw mut GlobData.Game_state.GameState as *mut BYTE,
                        C2Rust_Unnamed_2::DISCUSSED_EGG_CASING0.0 as ::core::ffi::c_int,
                        C2Rust_Unnamed_2::END_DISCUSSED_EGG_CASING0.0
                            as ::core::ffi::c_int,
                        TalkAbout.0 as BYTE,
                    );
                    setGameState(
                        &raw mut GlobData.Game_state.GameState as *mut BYTE,
                        C2Rust_Unnamed_2::DISCUSSED_EGG_CASING1.0 as ::core::ffi::c_int,
                        C2Rust_Unnamed_2::END_DISCUSSED_EGG_CASING1.0
                            as ::core::ffi::c_int,
                        TalkAbout.0 as BYTE,
                    );
                    setGameState(
                        &raw mut GlobData.Game_state.GameState as *mut BYTE,
                        C2Rust_Unnamed_2::DISCUSSED_EGG_CASING2.0 as ::core::ffi::c_int,
                        C2Rust_Unnamed_2::END_DISCUSSED_EGG_CASING2.0
                            as ::core::ffi::c_int,
                        TalkAbout.0 as BYTE,
                    );
                }
            }
            19 => {
                if getGameState(
                    &raw mut GlobData.Game_state.GameState as *mut BYTE,
                    C2Rust_Unnamed_2::EGG_CASE1_ON_SHIP.0 as ::core::ffi::c_int,
                    C2Rust_Unnamed_2::END_EGG_CASE1_ON_SHIP.0 as ::core::ffi::c_int,
                ) as ::core::ffi::c_int != 0
                    && getGameState(
                        &raw mut GlobData.Game_state.GameState as *mut BYTE,
                        C2Rust_Unnamed_2::DISCUSSED_EGG_CASING1.0 as ::core::ffi::c_int,
                        C2Rust_Unnamed_2::END_DISCUSSED_EGG_CASING1.0
                            as ::core::ffi::c_int,
                    ) == 0
                {
                    pStr = C2Rust_Unnamed_5::ABOUT_EGGCASE_0.0 as ::core::ffi::c_int
                        as RESPONSE_REF;
                    setGameState(
                        &raw mut GlobData.Game_state.GameState as *mut BYTE,
                        C2Rust_Unnamed_2::DISCUSSED_EGG_CASING0.0 as ::core::ffi::c_int,
                        C2Rust_Unnamed_2::END_DISCUSSED_EGG_CASING0.0
                            as ::core::ffi::c_int,
                        TalkAbout.0 as BYTE,
                    );
                    setGameState(
                        &raw mut GlobData.Game_state.GameState as *mut BYTE,
                        C2Rust_Unnamed_2::DISCUSSED_EGG_CASING1.0 as ::core::ffi::c_int,
                        C2Rust_Unnamed_2::END_DISCUSSED_EGG_CASING1.0
                            as ::core::ffi::c_int,
                        TalkAbout.0 as BYTE,
                    );
                    setGameState(
                        &raw mut GlobData.Game_state.GameState as *mut BYTE,
                        C2Rust_Unnamed_2::DISCUSSED_EGG_CASING2.0 as ::core::ffi::c_int,
                        C2Rust_Unnamed_2::END_DISCUSSED_EGG_CASING2.0
                            as ::core::ffi::c_int,
                        TalkAbout.0 as BYTE,
                    );
                }
            }
            20 => {
                if getGameState(
                    &raw mut GlobData.Game_state.GameState as *mut BYTE,
                    C2Rust_Unnamed_2::EGG_CASE2_ON_SHIP.0 as ::core::ffi::c_int,
                    C2Rust_Unnamed_2::END_EGG_CASE2_ON_SHIP.0 as ::core::ffi::c_int,
                ) as ::core::ffi::c_int != 0
                    && getGameState(
                        &raw mut GlobData.Game_state.GameState as *mut BYTE,
                        C2Rust_Unnamed_2::DISCUSSED_EGG_CASING2.0 as ::core::ffi::c_int,
                        C2Rust_Unnamed_2::END_DISCUSSED_EGG_CASING2.0
                            as ::core::ffi::c_int,
                    ) == 0
                {
                    pStr = C2Rust_Unnamed_5::ABOUT_EGGCASE_0.0 as ::core::ffi::c_int
                        as RESPONSE_REF;
                    setGameState(
                        &raw mut GlobData.Game_state.GameState as *mut BYTE,
                        C2Rust_Unnamed_2::DISCUSSED_EGG_CASING0.0 as ::core::ffi::c_int,
                        C2Rust_Unnamed_2::END_DISCUSSED_EGG_CASING0.0
                            as ::core::ffi::c_int,
                        TalkAbout.0 as BYTE,
                    );
                    setGameState(
                        &raw mut GlobData.Game_state.GameState as *mut BYTE,
                        C2Rust_Unnamed_2::DISCUSSED_EGG_CASING1.0 as ::core::ffi::c_int,
                        C2Rust_Unnamed_2::END_DISCUSSED_EGG_CASING1.0
                            as ::core::ffi::c_int,
                        TalkAbout.0 as BYTE,
                    );
                    setGameState(
                        &raw mut GlobData.Game_state.GameState as *mut BYTE,
                        C2Rust_Unnamed_2::DISCUSSED_EGG_CASING2.0 as ::core::ffi::c_int,
                        C2Rust_Unnamed_2::END_DISCUSSED_EGG_CASING2.0
                            as ::core::ffi::c_int,
                        TalkAbout.0 as BYTE,
                    );
                }
            }
            21 => {
                if getGameState(
                    &raw mut GlobData.Game_state.GameState as *mut BYTE,
                    C2Rust_Unnamed_2::SYREEN_SHUTTLE_ON_SHIP.0 as ::core::ffi::c_int,
                    C2Rust_Unnamed_2::END_SYREEN_SHUTTLE_ON_SHIP.0 as ::core::ffi::c_int,
                ) as ::core::ffi::c_int != 0
                    && getGameState(
                        &raw mut GlobData.Game_state.GameState as *mut BYTE,
                        C2Rust_Unnamed_2::DISCUSSED_SYREEN_SHUTTLE.0
                            as ::core::ffi::c_int,
                        C2Rust_Unnamed_2::END_DISCUSSED_SYREEN_SHUTTLE.0
                            as ::core::ffi::c_int,
                    ) == 0
                {
                    pStr = C2Rust_Unnamed_5::ABOUT_SHUTTLE.0 as ::core::ffi::c_int
                        as RESPONSE_REF;
                    setGameState(
                        &raw mut GlobData.Game_state.GameState as *mut BYTE,
                        C2Rust_Unnamed_2::DISCUSSED_SYREEN_SHUTTLE.0
                            as ::core::ffi::c_int,
                        C2Rust_Unnamed_2::END_DISCUSSED_SYREEN_SHUTTLE.0
                            as ::core::ffi::c_int,
                        TalkAbout.0 as BYTE,
                    );
                }
            }
            22 => {
                if getGameState(
                    &raw mut GlobData.Game_state.GameState as *mut BYTE,
                    C2Rust_Unnamed_2::VUX_BEAST_ON_SHIP.0 as ::core::ffi::c_int,
                    C2Rust_Unnamed_2::END_VUX_BEAST_ON_SHIP.0 as ::core::ffi::c_int,
                ) as ::core::ffi::c_int != 0
                    && getGameState(
                        &raw mut GlobData.Game_state.GameState as *mut BYTE,
                        C2Rust_Unnamed_2::DISCUSSED_VUX_BEAST.0 as ::core::ffi::c_int,
                        C2Rust_Unnamed_2::END_DISCUSSED_VUX_BEAST.0 as ::core::ffi::c_int,
                    ) == 0
                {
                    pStr = C2Rust_Unnamed_5::ABOUT_VUXBEAST0.0 as ::core::ffi::c_int
                        as RESPONSE_REF;
                    setGameState(
                        &raw mut GlobData.Game_state.GameState as *mut BYTE,
                        C2Rust_Unnamed_2::DISCUSSED_VUX_BEAST.0 as ::core::ffi::c_int,
                        C2Rust_Unnamed_2::END_DISCUSSED_VUX_BEAST.0
                            as ::core::ffi::c_int,
                        TalkAbout.0 as BYTE,
                    );
                }
            }
            23 => {
                if getGameState(
                    &raw mut GlobData.Game_state.GameState as *mut BYTE,
                    C2Rust_Unnamed_2::DESTRUCT_CODE_ON_SHIP.0 as ::core::ffi::c_int,
                    C2Rust_Unnamed_2::END_DESTRUCT_CODE_ON_SHIP.0 as ::core::ffi::c_int,
                ) as ::core::ffi::c_int != 0
                    && getGameState(
                        &raw mut GlobData.Game_state.GameState as *mut BYTE,
                        C2Rust_Unnamed_2::DISCUSSED_DESTRUCT_CODE.0
                            as ::core::ffi::c_int,
                        C2Rust_Unnamed_2::END_DISCUSSED_DESTRUCT_CODE.0
                            as ::core::ffi::c_int,
                    ) == 0
                {
                    pStr = C2Rust_Unnamed_5::ABOUT_DESTRUCT.0 as ::core::ffi::c_int
                        as RESPONSE_REF;
                    setGameState(
                        &raw mut GlobData.Game_state.GameState as *mut BYTE,
                        C2Rust_Unnamed_2::DISCUSSED_DESTRUCT_CODE.0
                            as ::core::ffi::c_int,
                        C2Rust_Unnamed_2::END_DISCUSSED_DESTRUCT_CODE.0
                            as ::core::ffi::c_int,
                        TalkAbout.0 as BYTE,
                    );
                }
            }
            0 => {
                if getGameState(
                    &raw mut GlobData.Game_state.GameState as *mut BYTE,
                    C2Rust_Unnamed_2::PORTAL_SPAWNER_ON_SHIP.0 as ::core::ffi::c_int,
                    C2Rust_Unnamed_2::END_PORTAL_SPAWNER_ON_SHIP.0 as ::core::ffi::c_int,
                ) as ::core::ffi::c_int != 0
                    && getGameState(
                        &raw mut GlobData.Game_state.GameState as *mut BYTE,
                        C2Rust_Unnamed_2::DISCUSSED_PORTAL_SPAWNER.0
                            as ::core::ffi::c_int,
                        C2Rust_Unnamed_2::END_DISCUSSED_PORTAL_SPAWNER.0
                            as ::core::ffi::c_int,
                    ) == 0
                {
                    pStr = C2Rust_Unnamed_5::ABOUT_PORTAL.0 as ::core::ffi::c_int
                        as RESPONSE_REF;
                    setGameState(
                        &raw mut GlobData.Game_state.GameState as *mut BYTE,
                        C2Rust_Unnamed_2::DISCUSSED_PORTAL_SPAWNER.0
                            as ::core::ffi::c_int,
                        C2Rust_Unnamed_2::END_DISCUSSED_PORTAL_SPAWNER.0
                            as ::core::ffi::c_int,
                        TalkAbout.0 as BYTE,
                    );
                }
            }
            24 => {
                if getGameState(
                    &raw mut GlobData.Game_state.GameState as *mut BYTE,
                    C2Rust_Unnamed_2::PORTAL_KEY_ON_SHIP.0 as ::core::ffi::c_int,
                    C2Rust_Unnamed_2::END_PORTAL_KEY_ON_SHIP.0 as ::core::ffi::c_int,
                ) as ::core::ffi::c_int != 0
                    && getGameState(
                        &raw mut GlobData.Game_state.GameState as *mut BYTE,
                        C2Rust_Unnamed_2::DISCUSSED_URQUAN_WARP.0 as ::core::ffi::c_int,
                        C2Rust_Unnamed_2::END_DISCUSSED_URQUAN_WARP.0
                            as ::core::ffi::c_int,
                    ) == 0
                {
                    pStr = C2Rust_Unnamed_5::ABOUT_WARPPOD.0 as ::core::ffi::c_int
                        as RESPONSE_REF;
                    setGameState(
                        &raw mut GlobData.Game_state.GameState as *mut BYTE,
                        C2Rust_Unnamed_2::DISCUSSED_URQUAN_WARP.0 as ::core::ffi::c_int,
                        C2Rust_Unnamed_2::END_DISCUSSED_URQUAN_WARP.0
                            as ::core::ffi::c_int,
                        TalkAbout.0 as BYTE,
                    );
                }
            }
            13 => {
                if getGameState(
                    &raw mut GlobData.Game_state.GameState as *mut BYTE,
                    C2Rust_Unnamed_2::BURV_BROADCASTERS_ON_SHIP.0 as ::core::ffi::c_int,
                    C2Rust_Unnamed_2::END_BURV_BROADCASTERS_ON_SHIP.0
                        as ::core::ffi::c_int,
                ) as ::core::ffi::c_int != 0
                    && getGameState(
                        &raw mut GlobData.Game_state.GameState as *mut BYTE,
                        C2Rust_Unnamed_2::DISCUSSED_BURVIX_HYPERWAVE.0
                            as ::core::ffi::c_int,
                        C2Rust_Unnamed_2::END_DISCUSSED_BURVIX_HYPERWAVE.0
                            as ::core::ffi::c_int,
                    ) == 0
                {
                    pStr = C2Rust_Unnamed_5::ABOUT_BCASTER.0 as ::core::ffi::c_int
                        as RESPONSE_REF;
                    setGameState(
                        &raw mut GlobData.Game_state.GameState as *mut BYTE,
                        C2Rust_Unnamed_2::DISCUSSED_BURVIX_HYPERWAVE.0
                            as ::core::ffi::c_int,
                        C2Rust_Unnamed_2::END_DISCUSSED_BURVIX_HYPERWAVE.0
                            as ::core::ffi::c_int,
                        TalkAbout.0 as BYTE,
                    );
                }
            }
            _ => {}
        }
        if pStr != 0 {
            if TalkAbout.0 != 0 {
                if PhraseIndex as ::core::ffi::c_int > 2 as ::core::ffi::c_int {
                    NPCPhrase_cb(
                        C2Rust_Unnamed_5::BETWEEN_DEVICES.0 as ::core::ffi::c_int,
                        None,
                    );
                }
                NPCPhrase_cb(pStr as ::core::ffi::c_int, None);
                if pStr as ::core::ffi::c_int
                    == C2Rust_Unnamed_5::ABOUT_VUXBEAST0.0 as ::core::ffi::c_int
                {
                    PhraseIndex = PhraseIndex.wrapping_add(1);
                    VuxBeastIndex = PhraseIndex;
                    NPCPhrase_cb(
                        C2Rust_Unnamed_5::ABOUT_VUXBEAST1.0 as ::core::ffi::c_int,
                        None,
                    );
                }
            }
            PhraseIndex = (PhraseIndex as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
                as COUNT;
        }
        i = i.wrapping_add(1);
    }
    if TalkAbout.0 != 0 {
        NPCPhrase_cb(C2Rust_Unnamed_5::DEVICE_TAIL.0 as ::core::ffi::c_int, None);
        if VuxBeastIndex != 0 {
            AlienTalkSegue(
                (VuxBeastIndex as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as COUNT,
            );
            EnableTalkingAnim(BOOLEAN::FALSE);
            AlienTalkSegue(VuxBeastIndex);
            EnableTalkingAnim(BOOLEAN::TRUE);
            AlienTalkSegue(!(0 as ::core::ffi::c_int) as COUNT);
        }
    }
    return BOOLEAN(
        (PhraseIndex as ::core::ffi::c_int > 2 as ::core::ffi::c_int)
            as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
}
unsafe extern "C" fn CheckTiming(
    mut month_index: COUNT,
    mut day_index: COUNT,
) -> BOOLEAN {
    let mut mi: COUNT = 0;
    let mut year_index: COUNT = 0;
    let mut days_in_month: [BYTE; 12] = [
        31 as BYTE,
        28 as BYTE,
        31 as BYTE,
        30 as BYTE,
        31 as BYTE,
        30 as BYTE,
        31 as BYTE,
        31 as BYTE,
        30 as BYTE,
        31 as BYTE,
        30 as BYTE,
        31 as BYTE,
    ];
    mi = getGameState(
        &raw mut GlobData.Game_state.GameState as *mut BYTE,
        C2Rust_Unnamed_2::STARBASE_MONTH.0 as ::core::ffi::c_int,
        C2Rust_Unnamed_2::END_STARBASE_MONTH.0 as ::core::ffi::c_int,
    ) as COUNT;
    year_index = 2155 as COUNT;
    day_index = (day_index as ::core::ffi::c_int
        + getGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_2::STARBASE_DAY.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_2::END_STARBASE_DAY.0 as ::core::ffi::c_int,
        ) as ::core::ffi::c_int) as COUNT;
    while day_index as ::core::ffi::c_int
        > days_in_month[(mi as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as usize]
            as ::core::ffi::c_int
    {
        day_index = (day_index as ::core::ffi::c_int
            - days_in_month[(mi as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                as usize] as ::core::ffi::c_int) as COUNT;
        mi = mi.wrapping_add(1);
        if mi as ::core::ffi::c_int > 12 as ::core::ffi::c_int {
            mi = 1 as COUNT;
            year_index = year_index.wrapping_add(1);
        }
    }
    month_index = (month_index as ::core::ffi::c_int + mi as ::core::ffi::c_int)
        as COUNT;
    year_index = (year_index as ::core::ffi::c_int
        + (month_index as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
            / 12 as ::core::ffi::c_int) as COUNT;
    month_index = ((month_index as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
        % 12 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as COUNT;
    return BOOLEAN(
        ((year_index as ::core::ffi::c_int)
            < GlobData.Game_state.GameClock.year_index as ::core::ffi::c_int
            || year_index as ::core::ffi::c_int
                == GlobData.Game_state.GameClock.year_index as ::core::ffi::c_int
                && ((month_index as ::core::ffi::c_int)
                    < GlobData.Game_state.GameClock.month_index as ::core::ffi::c_int
                    || month_index as ::core::ffi::c_int
                        == GlobData.Game_state.GameClock.month_index
                            as ::core::ffi::c_int
                        && (day_index as ::core::ffi::c_int)
                            < GlobData.Game_state.GameClock.day_index
                                as ::core::ffi::c_int)) as ::core::ffi::c_int
            as ::core::ffi::c_uint,
    );
}
unsafe extern "C" fn CheckBulletins(mut Repeat: BOOLEAN) {
    let mut pIntro: RESPONSE_REF = 0;
    let mut b0: BYTE = 0;
    let mut BulletinMask: DWORD = 0;
    if Repeat.0 != 0 {
        BulletinMask = (CurBulletinMask as ::core::ffi::c_ulong
            ^ 0xffffffff as ::core::ffi::c_ulong) as DWORD;
    } else {
        BulletinMask = getGameState32(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_2::STARBASE_BULLETS0.0 as ::core::ffi::c_int,
        );
    }
    pIntro = 0 as RESPONSE_REF;
    b0 = 0 as BYTE;
    while (b0 as ::core::ffi::c_int) < 32 as ::core::ffi::c_int {
        if BulletinMask as ::core::ffi::c_ulong
            & ((1 as ::core::ffi::c_long) << b0 as ::core::ffi::c_int)
                as ::core::ffi::c_ulong == 0
        {
            let mut pStr: RESPONSE_REF = 0;
            pStr = 0 as RESPONSE_REF;
            match b0 as ::core::ffi::c_int {
                0 => {
                    if CheckAlliance(
                        C2Rust_Unnamed_7::SPATHI_SHIP.0 as ::core::ffi::c_int as COUNT,
                    ) as ::core::ffi::c_int
                        == C2Rust_Unnamed_6::GOOD_GUY.0 as ::core::ffi::c_int
                    {
                        pStr = C2Rust_Unnamed_5::STARBASE_BULLETIN_1.0
                            as ::core::ffi::c_int as RESPONSE_REF;
                    }
                }
                1 => {
                    if CheckAlliance(
                        C2Rust_Unnamed_7::ZOQFOTPIK_SHIP.0 as ::core::ffi::c_int as COUNT,
                    ) as ::core::ffi::c_int
                        == C2Rust_Unnamed_6::GOOD_GUY.0 as ::core::ffi::c_int
                    {
                        pStr = C2Rust_Unnamed_5::STARBASE_BULLETIN_2.0
                            as ::core::ffi::c_int as RESPONSE_REF;
                    }
                }
                2 => {
                    if CheckAlliance(
                        C2Rust_Unnamed_7::SUPOX_SHIP.0 as ::core::ffi::c_int as COUNT,
                    ) as ::core::ffi::c_int
                        == C2Rust_Unnamed_6::GOOD_GUY.0 as ::core::ffi::c_int
                    {
                        pStr = C2Rust_Unnamed_5::STARBASE_BULLETIN_3.0
                            as ::core::ffi::c_int as RESPONSE_REF;
                    }
                }
                3 => {
                    if CheckAlliance(
                        C2Rust_Unnamed_7::UTWIG_SHIP.0 as ::core::ffi::c_int as COUNT,
                    ) as ::core::ffi::c_int
                        == C2Rust_Unnamed_6::GOOD_GUY.0 as ::core::ffi::c_int
                    {
                        pStr = C2Rust_Unnamed_5::STARBASE_BULLETIN_4.0
                            as ::core::ffi::c_int as RESPONSE_REF;
                    }
                }
                4 => {
                    if CheckAlliance(
                        C2Rust_Unnamed_7::ORZ_SHIP.0 as ::core::ffi::c_int as COUNT,
                    ) as ::core::ffi::c_int
                        == C2Rust_Unnamed_6::GOOD_GUY.0 as ::core::ffi::c_int
                    {
                        pStr = C2Rust_Unnamed_5::STARBASE_BULLETIN_5.0
                            as ::core::ffi::c_int as RESPONSE_REF;
                    }
                }
                5 => {
                    if getGameState(
                        &raw mut GlobData.Game_state.GameState as *mut BYTE,
                        C2Rust_Unnamed_2::ARILOU_MANNER.0 as ::core::ffi::c_int,
                        C2Rust_Unnamed_2::END_ARILOU_MANNER.0 as ::core::ffi::c_int,
                    ) as ::core::ffi::c_int == 2 as ::core::ffi::c_int
                    {
                        BulletinMask = (BulletinMask as ::core::ffi::c_ulong
                            | ((1 as ::core::ffi::c_long) << b0 as ::core::ffi::c_int)
                                as ::core::ffi::c_ulong) as DWORD;
                    } else if getGameState(
                        &raw mut GlobData.Game_state.GameState as *mut BYTE,
                        C2Rust_Unnamed_2::PORTAL_SPAWNER.0 as ::core::ffi::c_int,
                        C2Rust_Unnamed_2::END_PORTAL_SPAWNER.0 as ::core::ffi::c_int,
                    ) as ::core::ffi::c_int != 0
                        && (Repeat.0 != 0
                            || EscortFeasibilityStudy(
                                C2Rust_Unnamed_7::ARILOU_SHIP.0 as ::core::ffi::c_int
                                    as COUNT,
                            ) as ::core::ffi::c_int != 0)
                    {
                        pStr = C2Rust_Unnamed_5::STARBASE_BULLETIN_6.0
                            as ::core::ffi::c_int as RESPONSE_REF;
                        if Repeat.0 == 0 {
                            AddEscortShips(
                                C2Rust_Unnamed_7::ARILOU_SHIP.0 as ::core::ffi::c_int
                                    as COUNT,
                                3 as SIZE,
                            );
                        }
                    }
                }
                6 => {
                    if getGameState(
                        &raw mut GlobData.Game_state.GameState as *mut BYTE,
                        C2Rust_Unnamed_2::ZOQFOT_DISTRESS.0 as ::core::ffi::c_int,
                        C2Rust_Unnamed_2::END_ZOQFOT_DISTRESS.0 as ::core::ffi::c_int,
                    ) as ::core::ffi::c_int == 1 as ::core::ffi::c_int
                    {
                        pStr = C2Rust_Unnamed_5::STARBASE_BULLETIN_7.0
                            as ::core::ffi::c_int as RESPONSE_REF;
                    }
                }
                7 => {
                    if getGameState(
                        &raw mut GlobData.Game_state.GameState as *mut BYTE,
                        C2Rust_Unnamed_2::MET_MELNORME.0 as ::core::ffi::c_int,
                        C2Rust_Unnamed_2::END_MET_MELNORME.0 as ::core::ffi::c_int,
                    ) != 0
                    {
                        BulletinMask = (BulletinMask as ::core::ffi::c_ulong
                            | ((1 as ::core::ffi::c_long) << b0 as ::core::ffi::c_int)
                                as ::core::ffi::c_ulong) as DWORD;
                    } else if CheckTiming(3 as COUNT, 0 as COUNT).0 != 0 {
                        pStr = C2Rust_Unnamed_5::STARBASE_BULLETIN_8.0
                            as ::core::ffi::c_int as RESPONSE_REF;
                    }
                }
                8 => {
                    if getGameState(
                        &raw mut GlobData.Game_state.GameState as *mut BYTE,
                        C2Rust_Unnamed_2::MET_MELNORME.0 as ::core::ffi::c_int,
                        C2Rust_Unnamed_2::END_MET_MELNORME.0 as ::core::ffi::c_int,
                    ) != 0
                    {
                        BulletinMask = (BulletinMask as ::core::ffi::c_ulong
                            | ((1 as ::core::ffi::c_long) << b0 as ::core::ffi::c_int)
                                as ::core::ffi::c_ulong) as DWORD;
                    } else if CheckTiming(6 as COUNT, 0 as COUNT).0 != 0 {
                        pStr = C2Rust_Unnamed_5::STARBASE_BULLETIN_9.0
                            as ::core::ffi::c_int as RESPONSE_REF;
                    }
                }
                9 => {
                    if getGameState(
                        &raw mut GlobData.Game_state.GameState as *mut BYTE,
                        C2Rust_Unnamed_2::FOUND_PLUTO_SPATHI.0 as ::core::ffi::c_int,
                        C2Rust_Unnamed_2::END_FOUND_PLUTO_SPATHI.0 as ::core::ffi::c_int,
                    ) != 0
                    {
                        BulletinMask = (BulletinMask as ::core::ffi::c_ulong
                            | ((1 as ::core::ffi::c_long) << b0 as ::core::ffi::c_int)
                                as ::core::ffi::c_ulong) as DWORD;
                    } else if CheckTiming(0 as COUNT, 7 as COUNT).0 != 0 {
                        pStr = C2Rust_Unnamed_5::STARBASE_BULLETIN_10.0
                            as ::core::ffi::c_int as RESPONSE_REF;
                    }
                }
                10 => {
                    if getGameState(
                        &raw mut GlobData.Game_state.GameState as *mut BYTE,
                        C2Rust_Unnamed_2::SPATHI_SHIELDED_SELVES.0 as ::core::ffi::c_int,
                        C2Rust_Unnamed_2::END_SPATHI_SHIELDED_SELVES.0
                            as ::core::ffi::c_int,
                    ) != 0
                    {
                        pStr = C2Rust_Unnamed_5::STARBASE_BULLETIN_11.0
                            as ::core::ffi::c_int as RESPONSE_REF;
                    }
                }
                11 => {
                    if getGameState(
                        &raw mut GlobData.Game_state.GameState as *mut BYTE,
                        C2Rust_Unnamed_2::ZOQFOT_HOME_VISITS.0 as ::core::ffi::c_int,
                        C2Rust_Unnamed_2::END_ZOQFOT_HOME_VISITS.0 as ::core::ffi::c_int,
                    ) as ::core::ffi::c_int != 0
                        || getGameState32(
                            &raw mut GlobData.Game_state.GameState as *mut BYTE,
                            C2Rust_Unnamed_2::ZOQFOT_GRPOFFS0.0 as ::core::ffi::c_int,
                        ) != 0
                    {
                        BulletinMask = (BulletinMask as ::core::ffi::c_ulong
                            | ((1 as ::core::ffi::c_long) << b0 as ::core::ffi::c_int)
                                as ::core::ffi::c_ulong) as DWORD;
                    } else if CheckTiming(0 as COUNT, 42 as COUNT).0 != 0 {
                        pStr = C2Rust_Unnamed_5::STARBASE_BULLETIN_12.0
                            as ::core::ffi::c_int as RESPONSE_REF;
                    }
                }
                12 => {
                    if CheckAlliance(
                        C2Rust_Unnamed_7::CHMMR_SHIP.0 as ::core::ffi::c_int as COUNT,
                    ) as ::core::ffi::c_int
                        == C2Rust_Unnamed_6::GOOD_GUY.0 as ::core::ffi::c_int
                    {
                        pStr = C2Rust_Unnamed_5::STARBASE_BULLETIN_13.0
                            as ::core::ffi::c_int as RESPONSE_REF;
                    }
                }
                13 => {
                    if CheckAlliance(
                        C2Rust_Unnamed_7::SHOFIXTI_SHIP.0 as ::core::ffi::c_int as COUNT,
                    ) as ::core::ffi::c_int
                        == C2Rust_Unnamed_6::GOOD_GUY.0 as ::core::ffi::c_int
                    {
                        pStr = C2Rust_Unnamed_5::STARBASE_BULLETIN_14.0
                            as ::core::ffi::c_int as RESPONSE_REF;
                    }
                }
                14 => {
                    if getGameState(
                        &raw mut GlobData.Game_state.GameState as *mut BYTE,
                        C2Rust_Unnamed_2::PKUNK_MISSION.0 as ::core::ffi::c_int,
                        C2Rust_Unnamed_2::END_PKUNK_MISSION.0 as ::core::ffi::c_int,
                    ) != 0
                    {
                        pStr = C2Rust_Unnamed_5::STARBASE_BULLETIN_15.0
                            as ::core::ffi::c_int as RESPONSE_REF;
                    }
                }
                15 => {
                    if getGameState(
                        &raw mut GlobData.Game_state.GameState as *mut BYTE,
                        C2Rust_Unnamed_2::DESTRUCT_CODE_ON_SHIP.0 as ::core::ffi::c_int,
                        C2Rust_Unnamed_2::END_DESTRUCT_CODE_ON_SHIP.0
                            as ::core::ffi::c_int,
                    ) != 0
                    {
                        BulletinMask = (BulletinMask as ::core::ffi::c_ulong
                            | ((1 as ::core::ffi::c_long) << b0 as ::core::ffi::c_int)
                                as ::core::ffi::c_ulong) as DWORD;
                    } else if CheckTiming(7 as COUNT, 0 as COUNT).0 != 0 {
                        pStr = C2Rust_Unnamed_5::STARBASE_BULLETIN_16.0
                            as ::core::ffi::c_int as RESPONSE_REF;
                    }
                }
                17 => {
                    if getGameState(
                        &raw mut GlobData.Game_state.GameState as *mut BYTE,
                        C2Rust_Unnamed_2::YEHAT_ABSORBED_PKUNK.0 as ::core::ffi::c_int,
                        C2Rust_Unnamed_2::END_YEHAT_ABSORBED_PKUNK.0
                            as ::core::ffi::c_int,
                    ) != 0
                    {
                        pStr = C2Rust_Unnamed_5::STARBASE_BULLETIN_18.0
                            as ::core::ffi::c_int as RESPONSE_REF;
                    }
                }
                18 => {
                    if getGameState(
                        &raw mut GlobData.Game_state.GameState as *mut BYTE,
                        C2Rust_Unnamed_2::CHMMR_BOMB_STATE.0 as ::core::ffi::c_int,
                        C2Rust_Unnamed_2::END_CHMMR_BOMB_STATE.0 as ::core::ffi::c_int,
                    ) as ::core::ffi::c_int == 2 as ::core::ffi::c_int
                    {
                        pStr = C2Rust_Unnamed_5::STARBASE_BULLETIN_19.0
                            as ::core::ffi::c_int as RESPONSE_REF;
                    }
                }
                21 => {
                    if getGameState(
                        &raw mut GlobData.Game_state.GameState as *mut BYTE,
                        C2Rust_Unnamed_2::ZOQFOT_DISTRESS.0 as ::core::ffi::c_int,
                        C2Rust_Unnamed_2::END_ZOQFOT_DISTRESS.0 as ::core::ffi::c_int,
                    ) as ::core::ffi::c_int == 2 as ::core::ffi::c_int
                    {
                        pStr = C2Rust_Unnamed_5::STARBASE_BULLETIN_22.0
                            as ::core::ffi::c_int as RESPONSE_REF;
                    }
                }
                26 => {
                    let mut crew_sold: COUNT = 0;
                    crew_sold = (((getGameState(
                        &raw mut GlobData.Game_state.GameState as *mut BYTE,
                        C2Rust_Unnamed_2::CREW_SOLD_TO_DRUUGE1.0 as ::core::ffi::c_int,
                        C2Rust_Unnamed_2::END_CREW_SOLD_TO_DRUUGE1.0
                            as ::core::ffi::c_int,
                    ) as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as UWORD
                        as ::core::ffi::c_int
                        | getGameState(
                            &raw mut GlobData.Game_state.GameState as *mut BYTE,
                            C2Rust_Unnamed_2::CREW_SOLD_TO_DRUUGE0.0
                                as ::core::ffi::c_int,
                            C2Rust_Unnamed_2::END_CREW_SOLD_TO_DRUUGE0.0
                                as ::core::ffi::c_int,
                        ) as ::core::ffi::c_int) as COUNT;
                    if crew_sold as ::core::ffi::c_int > 100 as ::core::ffi::c_int {
                        BulletinMask = (BulletinMask as ::core::ffi::c_ulong
                            | ((1 as ::core::ffi::c_long) << b0 as ::core::ffi::c_int)
                                as ::core::ffi::c_ulong) as DWORD;
                    } else if crew_sold != 0 {
                        pStr = C2Rust_Unnamed_5::STARBASE_BULLETIN_27.0
                            as ::core::ffi::c_int as RESPONSE_REF;
                    }
                }
                27 => {
                    let mut crew_sold_0: COUNT = 0;
                    crew_sold_0 = (((getGameState(
                        &raw mut GlobData.Game_state.GameState as *mut BYTE,
                        C2Rust_Unnamed_2::CREW_SOLD_TO_DRUUGE1.0 as ::core::ffi::c_int,
                        C2Rust_Unnamed_2::END_CREW_SOLD_TO_DRUUGE1.0
                            as ::core::ffi::c_int,
                    ) as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as UWORD
                        as ::core::ffi::c_int
                        | getGameState(
                            &raw mut GlobData.Game_state.GameState as *mut BYTE,
                            C2Rust_Unnamed_2::CREW_SOLD_TO_DRUUGE0.0
                                as ::core::ffi::c_int,
                            C2Rust_Unnamed_2::END_CREW_SOLD_TO_DRUUGE0.0
                                as ::core::ffi::c_int,
                        ) as ::core::ffi::c_int) as COUNT;
                    if crew_sold_0 as ::core::ffi::c_int > 250 as ::core::ffi::c_int {
                        BulletinMask = (BulletinMask as ::core::ffi::c_ulong
                            | ((1 as ::core::ffi::c_long) << b0 as ::core::ffi::c_int)
                                as ::core::ffi::c_ulong) as DWORD;
                    } else if crew_sold_0 as ::core::ffi::c_int
                        > 100 as ::core::ffi::c_int
                    {
                        pStr = C2Rust_Unnamed_5::STARBASE_BULLETIN_28.0
                            as ::core::ffi::c_int as RESPONSE_REF;
                    }
                }
                28 => {
                    let mut crew_bought: COUNT = 0;
                    crew_bought = (((getGameState(
                        &raw mut GlobData.Game_state.GameState as *mut BYTE,
                        C2Rust_Unnamed_2::CREW_PURCHASED1.0 as ::core::ffi::c_int,
                        C2Rust_Unnamed_2::END_CREW_PURCHASED1.0 as ::core::ffi::c_int,
                    ) as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as UWORD
                        as ::core::ffi::c_int
                        | getGameState(
                            &raw mut GlobData.Game_state.GameState as *mut BYTE,
                            C2Rust_Unnamed_2::CREW_PURCHASED0.0 as ::core::ffi::c_int,
                            C2Rust_Unnamed_2::END_CREW_PURCHASED0.0 as ::core::ffi::c_int,
                        ) as ::core::ffi::c_int) as COUNT;
                    if crew_bought as ::core::ffi::c_int >= 1000 as ::core::ffi::c_int {
                        pStr = C2Rust_Unnamed_5::STARBASE_BULLETIN_29.0
                            as ::core::ffi::c_int as RESPONSE_REF;
                    }
                }
                29 => {
                    if ((getGameState(
                        &raw mut GlobData.Game_state.GameState as *mut BYTE,
                        C2Rust_Unnamed_2::CREW_SOLD_TO_DRUUGE1.0 as ::core::ffi::c_int,
                        C2Rust_Unnamed_2::END_CREW_SOLD_TO_DRUUGE1.0
                            as ::core::ffi::c_int,
                    ) as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as UWORD
                        as ::core::ffi::c_int
                        | getGameState(
                            &raw mut GlobData.Game_state.GameState as *mut BYTE,
                            C2Rust_Unnamed_2::CREW_SOLD_TO_DRUUGE0.0
                                as ::core::ffi::c_int,
                            C2Rust_Unnamed_2::END_CREW_SOLD_TO_DRUUGE0.0
                                as ::core::ffi::c_int,
                        ) as ::core::ffi::c_int > 250 as ::core::ffi::c_int
                    {
                        pStr = C2Rust_Unnamed_5::STARBASE_BULLETIN_30.0
                            as ::core::ffi::c_int as RESPONSE_REF;
                    }
                }
                16 | 19 | 20 | 22 | 23 | 24 | 25 | 30 | 31 | _ => {}
            }
            if pStr != 0 {
                if pIntro != 0 {
                    NPCPhrase_cb(
                        C2Rust_Unnamed_5::BETWEEN_BULLETINS.0 as ::core::ffi::c_int,
                        None,
                    );
                } else if Repeat.0 != 0 {
                    pIntro = C2Rust_Unnamed_5::BEFORE_WE_GO_ON_1.0 as ::core::ffi::c_int
                        as RESPONSE_REF;
                } else {
                    match TFB_Random() as BYTE as ::core::ffi::c_int
                        % 7 as ::core::ffi::c_int
                    {
                        0 => {
                            pIntro = C2Rust_Unnamed_5::BEFORE_WE_GO_ON_1.0
                                as ::core::ffi::c_int as RESPONSE_REF;
                        }
                        1 => {
                            pIntro = C2Rust_Unnamed_5::BEFORE_WE_GO_ON_2.0
                                as ::core::ffi::c_int as RESPONSE_REF;
                        }
                        2 => {
                            pIntro = C2Rust_Unnamed_5::BEFORE_WE_GO_ON_3.0
                                as ::core::ffi::c_int as RESPONSE_REF;
                        }
                        3 => {
                            pIntro = C2Rust_Unnamed_5::BEFORE_WE_GO_ON_4.0
                                as ::core::ffi::c_int as RESPONSE_REF;
                        }
                        4 => {
                            pIntro = C2Rust_Unnamed_5::BEFORE_WE_GO_ON_5.0
                                as ::core::ffi::c_int as RESPONSE_REF;
                        }
                        5 => {
                            pIntro = C2Rust_Unnamed_5::BEFORE_WE_GO_ON_6.0
                                as ::core::ffi::c_int as RESPONSE_REF;
                        }
                        _ => {
                            pIntro = C2Rust_Unnamed_5::BEFORE_WE_GO_ON_7.0
                                as ::core::ffi::c_int as RESPONSE_REF;
                        }
                    }
                    NPCPhrase_cb(pIntro as ::core::ffi::c_int, None);
                }
                NPCPhrase_cb(pStr as ::core::ffi::c_int, None);
                CurBulletinMask = (CurBulletinMask as ::core::ffi::c_ulong
                    | ((1 as ::core::ffi::c_long) << b0 as ::core::ffi::c_int)
                        as ::core::ffi::c_ulong) as DWORD;
            }
        }
        b0 = b0.wrapping_add(1);
    }
    if pIntro as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        && getGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_2::STARBASE_VISITED.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_2::END_STARBASE_VISITED.0 as ::core::ffi::c_int,
        ) as ::core::ffi::c_int != 0
    {
        NPCPhrase_cb(C2Rust_Unnamed_5::RETURN_HELLO.0 as ::core::ffi::c_int, None);
    } else if Repeat.0 == 0 {
        setGameState32(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_2::STARBASE_BULLETS0.0 as ::core::ffi::c_int,
            BulletinMask,
        );
    }
}
unsafe extern "C" fn NormalStarbase(mut R: RESPONSE_REF) {
    if R as ::core::ffi::c_int == C2Rust_Unnamed_5::no_need_info.0 as ::core::ffi::c_int
    {
        NPCPhrase_cb(C2Rust_Unnamed_5::OK_NO_NEED_INFO.0 as ::core::ffi::c_int, None);
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_5::new_devices.0 as ::core::ffi::c_int
    {
        DiscussDevices(BOOLEAN::TRUE);
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_5::repeat_bulletins.0 as ::core::ffi::c_int
    {
        CheckBulletins(BOOLEAN::TRUE);
    } else if R as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        if getGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_2::MOONBASE_ON_SHIP.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_2::END_MOONBASE_ON_SHIP.0 as ::core::ffi::c_int,
        ) != 0
        {
            NPCPhrase_cb(
                C2Rust_Unnamed_5::STARBASE_IS_READY_A.0 as ::core::ffi::c_int,
                None,
            );
            if usingSpeech.0 != 0 {
                NPCPhrase_cb(
                    C2Rust_Unnamed_5::YOUR_FLAGSHIP_3DO1.0 as ::core::ffi::c_int,
                    None,
                );
            } else {
                NPCPhrase_cb(
                    C2Rust_Unnamed_5::YOUR_FLAGSHIP_PC.0 as ::core::ffi::c_int,
                    None,
                );
                NPCPhrase_cb(C2Rust_Unnamed_4::GLOBAL_SHIP_NAME.0, None);
            }
            NPCPhrase_cb(
                C2Rust_Unnamed_5::STARBASE_IS_READY_B.0 as ::core::ffi::c_int,
                None,
            );
            if usingSpeech.0 != 0 {
                NPCPhrase_cb(
                    C2Rust_Unnamed_5::YOUR_FLAGSHIP_3DO0.0 as ::core::ffi::c_int,
                    None,
                );
            } else {
                NPCPhrase_cb(C2Rust_Unnamed_4::GLOBAL_SHIP_NAME.0, None);
            }
            NPCPhrase_cb(
                C2Rust_Unnamed_5::STARBASE_IS_READY_C.0 as ::core::ffi::c_int,
                None,
            );
            DeltaSISGauges(0 as SIZE, 0 as SIZE, 2500 as ::core::ffi::c_int);
            setGameState(
                &raw mut GlobData.Game_state.GameState as *mut BYTE,
                C2Rust_Unnamed_2::STARBASE_MONTH.0 as ::core::ffi::c_int,
                C2Rust_Unnamed_2::END_STARBASE_MONTH.0 as ::core::ffi::c_int,
                GlobData.Game_state.GameClock.month_index,
            );
            setGameState(
                &raw mut GlobData.Game_state.GameState as *mut BYTE,
                C2Rust_Unnamed_2::STARBASE_DAY.0 as ::core::ffi::c_int,
                C2Rust_Unnamed_2::END_STARBASE_DAY.0 as ::core::ffi::c_int,
                GlobData.Game_state.GameClock.day_index,
            );
        } else if getGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_2::STARBASE_VISITED.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_2::END_STARBASE_VISITED.0 as ::core::ffi::c_int,
        ) != 0
        {
            CheckBulletins(BOOLEAN::FALSE);
        } else {
            let mut pStr0: RESPONSE_REF = 0 as RESPONSE_REF;
            let mut pStr1: RESPONSE_REF = 0 as RESPONSE_REF;
            match TFB_Random() as BYTE as ::core::ffi::c_int & 7 as ::core::ffi::c_int {
                0 => {
                    pStr0 = C2Rust_Unnamed_5::NORMAL_HELLO_A0.0 as ::core::ffi::c_int
                        as RESPONSE_REF;
                    pStr1 = C2Rust_Unnamed_5::NORMAL_HELLO_A1.0 as ::core::ffi::c_int
                        as RESPONSE_REF;
                }
                1 => {
                    pStr0 = C2Rust_Unnamed_5::NORMAL_HELLO_B0.0 as ::core::ffi::c_int
                        as RESPONSE_REF;
                    pStr1 = C2Rust_Unnamed_5::NORMAL_HELLO_B1.0 as ::core::ffi::c_int
                        as RESPONSE_REF;
                }
                2 => {
                    pStr0 = C2Rust_Unnamed_5::NORMAL_HELLO_C0.0 as ::core::ffi::c_int
                        as RESPONSE_REF;
                    pStr1 = C2Rust_Unnamed_5::NORMAL_HELLO_C1.0 as ::core::ffi::c_int
                        as RESPONSE_REF;
                }
                3 => {
                    pStr0 = C2Rust_Unnamed_5::NORMAL_HELLO_D0.0 as ::core::ffi::c_int
                        as RESPONSE_REF;
                    pStr1 = C2Rust_Unnamed_5::NORMAL_HELLO_D1.0 as ::core::ffi::c_int
                        as RESPONSE_REF;
                }
                4 => {
                    pStr0 = C2Rust_Unnamed_5::NORMAL_HELLO_E0.0 as ::core::ffi::c_int
                        as RESPONSE_REF;
                    pStr1 = C2Rust_Unnamed_5::NORMAL_HELLO_E1.0 as ::core::ffi::c_int
                        as RESPONSE_REF;
                }
                5 => {
                    pStr0 = C2Rust_Unnamed_5::NORMAL_HELLO_F0.0 as ::core::ffi::c_int
                        as RESPONSE_REF;
                    pStr1 = C2Rust_Unnamed_5::NORMAL_HELLO_F1.0 as ::core::ffi::c_int
                        as RESPONSE_REF;
                }
                6 => {
                    pStr0 = C2Rust_Unnamed_5::NORMAL_HELLO_G0.0 as ::core::ffi::c_int
                        as RESPONSE_REF;
                    pStr1 = C2Rust_Unnamed_5::NORMAL_HELLO_G1.0 as ::core::ffi::c_int
                        as RESPONSE_REF;
                }
                7 => {
                    pStr0 = C2Rust_Unnamed_5::NORMAL_HELLO_H0.0 as ::core::ffi::c_int
                        as RESPONSE_REF;
                    pStr1 = C2Rust_Unnamed_5::NORMAL_HELLO_H1.0 as ::core::ffi::c_int
                        as RESPONSE_REF;
                }
                _ => {}
            }
            NPCPhrase_cb(pStr0 as ::core::ffi::c_int, None);
            if usingSpeech.0 == 0 {
                NPCPhrase_cb(C2Rust_Unnamed_5::SPACE.0 as ::core::ffi::c_int, None);
                NPCPhrase_cb(C2Rust_Unnamed_4::GLOBAL_PLAYER_NAME.0, None);
            }
            NPCPhrase_cb(pStr1 as ::core::ffi::c_int, None);
            CheckBulletins(BOOLEAN::FALSE);
        }
        setGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_2::STARBASE_VISITED.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_2::END_STARBASE_VISITED.0 as ::core::ffi::c_int,
            1 as BYTE,
        );
    }
    if GlobData.SIS_state.TotalElementMass != 0 {
        DoResponsePhrase(
            C2Rust_Unnamed_5::have_minerals.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(SellMinerals as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        );
    }
    if DiscussDevices(BOOLEAN::FALSE).0 != 0 {
        DoResponsePhrase(
            C2Rust_Unnamed_5::new_devices.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(NormalStarbase as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        );
    }
    if CurBulletinMask != 0 {
        DoResponsePhrase(
            C2Rust_Unnamed_5::repeat_bulletins.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(NormalStarbase as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        );
    }
    DoResponsePhrase(
        C2Rust_Unnamed_5::need_info.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(NeedInfo as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    );
    DoResponsePhrase(
        C2Rust_Unnamed_5::goodbye_commander.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(ByeBye as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    );
}
unsafe extern "C" fn SellMinerals(mut R: RESPONSE_REF) {
    let mut i: COUNT = 0;
    let mut total: COUNT = 0;
    let mut Sleepy: BOOLEAN = BOOLEAN::FALSE;
    let mut pStr1: RESPONSE_REF = 0 as RESPONSE_REF;
    let mut pStr2: RESPONSE_REF = 0 as RESPONSE_REF;
    total = 0 as COUNT;
    Sleepy = BOOLEAN::TRUE;
    i = 0 as COUNT;
    while (i as ::core::ffi::c_int)
        < C2Rust_Unnamed::NUM_ELEMENT_CATEGORIES.0 as ::core::ffi::c_int
    {
        let mut amount: COUNT = 0;
        let mut TimeIn: DWORD = 0 as DWORD;
        if i as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            DrawCargoStrings(
                !(0 as ::core::ffi::c_int) as BYTE,
                !(0 as ::core::ffi::c_int) as BYTE,
            );
            SleepThread(
                (840 as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as TimePeriod,
            );
            TimeIn = GetTimeCounter() as DWORD;
            DrawCargoStrings(
                0 as ::core::ffi::c_int as BYTE,
                0 as ::core::ffi::c_int as BYTE,
            );
        } else if Sleepy.0 != 0 {
            DrawCargoStrings(
                (i as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as BYTE,
                i as BYTE,
            );
            TimeIn = GetTimeCounter() as DWORD;
        }
        amount = GlobData.SIS_state.ElementAmounts[i as usize];
        if amount as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            total = (total as ::core::ffi::c_int
                + amount as ::core::ffi::c_int
                    * GlobData.Game_state.ElementWorth[i as usize] as ::core::ffi::c_int)
                as COUNT;
            loop {
                if Sleepy.0 == 0 || AnyButtonPress(BOOLEAN::TRUE).0 != 0
                    || GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
                        & C2Rust_Unnamed_3::CHECK_ABORT.0 as ::core::ffi::c_int != 0
                {
                    Sleepy = BOOLEAN::FALSE;
                    GlobData.SIS_state.ElementAmounts[i as usize] = 0 as COUNT;
                    GlobData.SIS_state.TotalElementMass = (GlobData
                        .SIS_state
                        .TotalElementMass as ::core::ffi::c_int
                        - amount as ::core::ffi::c_int) as COUNT;
                    DeltaSISGauges(
                        0 as SIZE,
                        0 as SIZE,
                        amount as ::core::ffi::c_int
                            * GlobData.Game_state.ElementWorth[i as usize]
                                as ::core::ffi::c_int,
                    );
                    break;
                } else {
                    GlobData.SIS_state.ElementAmounts[i as usize] = GlobData
                        .SIS_state
                        .ElementAmounts[i as usize]
                        .wrapping_sub(1);
                    GlobData.SIS_state.TotalElementMass = GlobData
                        .SIS_state
                        .TotalElementMass
                        .wrapping_sub(1);
                    TaskSwitch();
                    TimeIn = GetTimeCounter() as DWORD;
                    DrawCargoStrings(i as BYTE, i as BYTE);
                    ShowRemainingCapacity();
                    DeltaSISGauges(
                        0 as SIZE,
                        0 as SIZE,
                        GlobData.Game_state.ElementWorth[i as usize]
                            as ::core::ffi::c_int,
                    );
                    amount = amount.wrapping_sub(1);
                    if amount == 0 {
                        break;
                    }
                }
            }
        }
        if Sleepy.0 != 0 {
            SleepThreadUntil(
                TimeIn
                    .wrapping_add(
                        (840 as ::core::ffi::c_int / 4 as ::core::ffi::c_int)
                            as TimeCount,
                    ),
            );
            TimeIn = GetTimeCounter() as DWORD;
        }
        i = i.wrapping_add(1);
    }
    SleepThread((840 as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as TimePeriod);
    ClearSISRect(((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as BYTE);
    if (total as ::core::ffi::c_int) < 1000 as ::core::ffi::c_int {
        total = getGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_2::LIGHT_MINERAL_LOAD.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_2::END_LIGHT_MINERAL_LOAD.0 as ::core::ffi::c_int,
        ) as COUNT;
        let c2rust_fresh0 = total;
        total = total.wrapping_add(1);
        match c2rust_fresh0 as ::core::ffi::c_int {
            0 => {
                pStr1 = C2Rust_Unnamed_5::LIGHT_LOAD_A0.0 as ::core::ffi::c_int
                    as RESPONSE_REF;
                pStr2 = C2Rust_Unnamed_5::LIGHT_LOAD_A1.0 as ::core::ffi::c_int
                    as RESPONSE_REF;
            }
            1 => {
                pStr1 = C2Rust_Unnamed_5::LIGHT_LOAD_B0.0 as ::core::ffi::c_int
                    as RESPONSE_REF;
                pStr2 = C2Rust_Unnamed_5::LIGHT_LOAD_B1.0 as ::core::ffi::c_int
                    as RESPONSE_REF;
            }
            2 => {
                pStr1 = C2Rust_Unnamed_5::LIGHT_LOAD_C0.0 as ::core::ffi::c_int
                    as RESPONSE_REF;
                pStr2 = C2Rust_Unnamed_5::LIGHT_LOAD_C1.0 as ::core::ffi::c_int
                    as RESPONSE_REF;
            }
            3 => {
                pStr1 = C2Rust_Unnamed_5::LIGHT_LOAD_D0.0 as ::core::ffi::c_int
                    as RESPONSE_REF;
                pStr2 = C2Rust_Unnamed_5::LIGHT_LOAD_D1.0 as ::core::ffi::c_int
                    as RESPONSE_REF;
            }
            4 => {
                pStr1 = C2Rust_Unnamed_5::LIGHT_LOAD_E0.0 as ::core::ffi::c_int
                    as RESPONSE_REF;
                pStr2 = C2Rust_Unnamed_5::LIGHT_LOAD_E1.0 as ::core::ffi::c_int
                    as RESPONSE_REF;
            }
            5 => {
                pStr1 = C2Rust_Unnamed_5::LIGHT_LOAD_F0.0 as ::core::ffi::c_int
                    as RESPONSE_REF;
                pStr2 = C2Rust_Unnamed_5::LIGHT_LOAD_F1.0 as ::core::ffi::c_int
                    as RESPONSE_REF;
            }
            6 => {
                total = total.wrapping_sub(1);
                pStr1 = C2Rust_Unnamed_5::LIGHT_LOAD_G0.0 as ::core::ffi::c_int
                    as RESPONSE_REF;
                pStr2 = C2Rust_Unnamed_5::LIGHT_LOAD_G1.0 as ::core::ffi::c_int
                    as RESPONSE_REF;
            }
            _ => {}
        }
        setGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_2::LIGHT_MINERAL_LOAD.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_2::END_LIGHT_MINERAL_LOAD.0 as ::core::ffi::c_int,
            total as BYTE,
        );
    } else if (total as ::core::ffi::c_int) < 2500 as ::core::ffi::c_int {
        total = getGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_2::MEDIUM_MINERAL_LOAD.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_2::END_MEDIUM_MINERAL_LOAD.0 as ::core::ffi::c_int,
        ) as COUNT;
        let c2rust_fresh1 = total;
        total = total.wrapping_add(1);
        match c2rust_fresh1 as ::core::ffi::c_int {
            0 => {
                pStr1 = C2Rust_Unnamed_5::MEDIUM_LOAD_A0.0 as ::core::ffi::c_int
                    as RESPONSE_REF;
                pStr2 = C2Rust_Unnamed_5::MEDIUM_LOAD_A1.0 as ::core::ffi::c_int
                    as RESPONSE_REF;
            }
            1 => {
                pStr1 = C2Rust_Unnamed_5::MEDIUM_LOAD_B0.0 as ::core::ffi::c_int
                    as RESPONSE_REF;
                pStr2 = C2Rust_Unnamed_5::MEDIUM_LOAD_B1.0 as ::core::ffi::c_int
                    as RESPONSE_REF;
            }
            2 => {
                pStr1 = C2Rust_Unnamed_5::MEDIUM_LOAD_C0.0 as ::core::ffi::c_int
                    as RESPONSE_REF;
                pStr2 = C2Rust_Unnamed_5::MEDIUM_LOAD_C1.0 as ::core::ffi::c_int
                    as RESPONSE_REF;
            }
            3 => {
                pStr1 = C2Rust_Unnamed_5::MEDIUM_LOAD_D0.0 as ::core::ffi::c_int
                    as RESPONSE_REF;
                pStr2 = C2Rust_Unnamed_5::MEDIUM_LOAD_D1.0 as ::core::ffi::c_int
                    as RESPONSE_REF;
            }
            4 => {
                pStr1 = C2Rust_Unnamed_5::MEDIUM_LOAD_E0.0 as ::core::ffi::c_int
                    as RESPONSE_REF;
                pStr2 = C2Rust_Unnamed_5::MEDIUM_LOAD_E1.0 as ::core::ffi::c_int
                    as RESPONSE_REF;
            }
            5 => {
                pStr1 = C2Rust_Unnamed_5::MEDIUM_LOAD_F0.0 as ::core::ffi::c_int
                    as RESPONSE_REF;
                pStr2 = C2Rust_Unnamed_5::MEDIUM_LOAD_F1.0 as ::core::ffi::c_int
                    as RESPONSE_REF;
            }
            6 => {
                total = total.wrapping_sub(1);
                pStr1 = C2Rust_Unnamed_5::MEDIUM_LOAD_G0.0 as ::core::ffi::c_int
                    as RESPONSE_REF;
                pStr2 = C2Rust_Unnamed_5::MEDIUM_LOAD_G1.0 as ::core::ffi::c_int
                    as RESPONSE_REF;
            }
            _ => {}
        }
        setGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_2::MEDIUM_MINERAL_LOAD.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_2::END_MEDIUM_MINERAL_LOAD.0 as ::core::ffi::c_int,
            total as BYTE,
        );
    } else {
        total = getGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_2::HEAVY_MINERAL_LOAD.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_2::END_HEAVY_MINERAL_LOAD.0 as ::core::ffi::c_int,
        ) as COUNT;
        let c2rust_fresh2 = total;
        total = total.wrapping_add(1);
        match c2rust_fresh2 as ::core::ffi::c_int {
            0 => {
                pStr1 = C2Rust_Unnamed_5::HEAVY_LOAD_A0.0 as ::core::ffi::c_int
                    as RESPONSE_REF;
                pStr2 = C2Rust_Unnamed_5::HEAVY_LOAD_A1.0 as ::core::ffi::c_int
                    as RESPONSE_REF;
            }
            1 => {
                pStr1 = C2Rust_Unnamed_5::HEAVY_LOAD_B0.0 as ::core::ffi::c_int
                    as RESPONSE_REF;
                pStr2 = C2Rust_Unnamed_5::HEAVY_LOAD_B1.0 as ::core::ffi::c_int
                    as RESPONSE_REF;
            }
            2 => {
                pStr1 = C2Rust_Unnamed_5::HEAVY_LOAD_C0.0 as ::core::ffi::c_int
                    as RESPONSE_REF;
                pStr2 = C2Rust_Unnamed_5::HEAVY_LOAD_C1.0 as ::core::ffi::c_int
                    as RESPONSE_REF;
            }
            3 => {
                pStr1 = C2Rust_Unnamed_5::HEAVY_LOAD_D0.0 as ::core::ffi::c_int
                    as RESPONSE_REF;
                pStr2 = C2Rust_Unnamed_5::HEAVY_LOAD_D1.0 as ::core::ffi::c_int
                    as RESPONSE_REF;
            }
            4 => {
                pStr1 = C2Rust_Unnamed_5::HEAVY_LOAD_E0.0 as ::core::ffi::c_int
                    as RESPONSE_REF;
                pStr2 = C2Rust_Unnamed_5::HEAVY_LOAD_E1.0 as ::core::ffi::c_int
                    as RESPONSE_REF;
            }
            5 => {
                pStr1 = C2Rust_Unnamed_5::HEAVY_LOAD_F0.0 as ::core::ffi::c_int
                    as RESPONSE_REF;
                pStr2 = C2Rust_Unnamed_5::HEAVY_LOAD_F1.0 as ::core::ffi::c_int
                    as RESPONSE_REF;
            }
            6 => {
                total = total.wrapping_sub(1);
                pStr1 = C2Rust_Unnamed_5::HEAVY_LOAD_G0.0 as ::core::ffi::c_int
                    as RESPONSE_REF;
                pStr2 = C2Rust_Unnamed_5::HEAVY_LOAD_G1.0 as ::core::ffi::c_int
                    as RESPONSE_REF;
            }
            _ => {}
        }
        setGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_2::HEAVY_MINERAL_LOAD.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_2::END_HEAVY_MINERAL_LOAD.0 as ::core::ffi::c_int,
            total as BYTE,
        );
    }
    NPCPhrase_cb(pStr1 as ::core::ffi::c_int, None);
    if usingSpeech.0 == 0 {
        NPCPhrase_cb(C2Rust_Unnamed_5::SPACE.0 as ::core::ffi::c_int, None);
        NPCPhrase_cb(C2Rust_Unnamed_4::GLOBAL_PLAYER_NAME.0, None);
    }
    NPCPhrase_cb(pStr2 as ::core::ffi::c_int, None);
    NormalStarbase(R);
}
unsafe extern "C" fn Intro() {
    NormalStarbase(0 as RESPONSE_REF);
}
unsafe extern "C" fn uninit_starbase() -> COUNT {
    return 0 as COUNT;
}
unsafe extern "C" fn post_starbase_enc() {
    setGameState(
        &raw mut GlobData.Game_state.GameState as *mut BYTE,
        C2Rust_Unnamed_2::MOONBASE_ON_SHIP.0 as ::core::ffi::c_int,
        C2Rust_Unnamed_2::END_MOONBASE_ON_SHIP.0 as ::core::ffi::c_int,
        0 as BYTE,
    );
    if getGameState(
        &raw mut GlobData.Game_state.GameState as *mut BYTE,
        C2Rust_Unnamed_2::CHMMR_BOMB_STATE.0 as ::core::ffi::c_int,
        C2Rust_Unnamed_2::END_CHMMR_BOMB_STATE.0 as ::core::ffi::c_int,
    ) as ::core::ffi::c_int == 2 as ::core::ffi::c_int
    {
        setGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_2::CHMMR_BOMB_STATE.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_2::END_CHMMR_BOMB_STATE.0 as ::core::ffi::c_int,
            3 as BYTE,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn init_starbase_comm() -> *mut LOCDATA {
    let mut retval: *mut LOCDATA = ::core::ptr::null_mut::<LOCDATA>();
    commander_desc.init_encounter_func = Some(Intro as unsafe extern "C" fn() -> ())
        as Option<unsafe extern "C" fn() -> ()>;
    commander_desc.post_encounter_func = Some(
        post_starbase_enc as unsafe extern "C" fn() -> (),
    ) as Option<unsafe extern "C" fn() -> ()>;
    commander_desc.uninit_encounter_func = Some(
        uninit_starbase as unsafe extern "C" fn() -> COUNT,
    ) as Option<unsafe extern "C" fn() -> COUNT>;
    commander_desc.AlienTextWidth = 143 as COUNT;
    commander_desc.AlienTextBaseline.x = 164 as COORD;
    commander_desc.AlienTextBaseline.y = 20 as COORD;
    commander_desc.AlienAltSongRes = b"comm.starbase.music\0".as_ptr()
        as *const ::core::ffi::c_char as RESOURCE;
    commander_desc.AlienSongFlags
        |= ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as LDAS_FLAGS;
    CurBulletinMask = 0 as DWORD;
    setSegue(Segue::Segue_peace);
    retval = &raw mut commander_desc;
    return retval;
}
