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

// Rust translation of src/uqm/comm/zoqfot/zoqfotc.c.
extern "C" {
    pub type string_table_entry;
    pub type frame_desc;
    pub type font_desc;
    pub type tfb_soundsample;
    fn AddEvent(
        r#type: EVENT_TYPE,
        month_index: COUNT,
        day_index: COUNT,
        year_index: COUNT,
        func_index: BYTE,
    ) -> HEVENT;
    fn SetAbsStringTableIndex(String: STRING, StringTableIndex: COUNT) -> STRING;
    fn GetStringAddress(String: STRING) -> STRINGPTR;
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
    fn GetAllianceName(buf: *mut UNICODE, name_1: RESPONSE_REF);
    fn construct_response(buf: *mut UNICODE, R: ::core::ffi::c_int, ...);
    fn setSegue(segue: Segue);
    static mut ScreenWidth: ::core::ffi::c_int;
    fn AlienTalkSegue(wait_track: COUNT);
    fn AddEscortShips(race: COUNT, count: SIZE) -> COUNT;
    fn SetRaceAllied(race: COUNT, flag: BOOLEAN) -> COUNT;
    fn CheckAlliance(race: COUNT) -> COUNT;
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
pub type HEVENT = HLINK;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct EVENT_TYPE(pub ::core::ffi::c_uint);
impl EVENT_TYPE {
    pub const ABSOLUTE_EVENT: Self = Self(0);
    pub const RELATIVE_EVENT: Self = Self(1);
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
pub struct C2Rust_Unnamed(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed {
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
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_0 {
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
pub struct C2Rust_Unnamed_1(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_1 {
    pub const NULL_PHRASE: Self = Self(0);
    pub const WE_ARE0: Self = Self(1);
    pub const WE_ARE1: Self = Self(2);
    pub const WE_ARE2: Self = Self(3);
    pub const WE_ARE3: Self = Self(4);
    pub const WE_ARE4: Self = Self(5);
    pub const WE_ARE5: Self = Self(6);
    pub const WE_ARE6: Self = Self(7);
    pub const WE_ARE7: Self = Self(8);
    pub const SCOUT_HELLO0: Self = Self(9);
    pub const SCOUT_HELLO1: Self = Self(10);
    pub const SCOUT_HELLO2: Self = Self(11);
    pub const SCOUT_HELLO3: Self = Self(12);
    pub const INIT_HOME_HELLO0: Self = Self(13);
    pub const INIT_HOME_HELLO1: Self = Self(14);
    pub const INIT_HOME_HELLO2: Self = Self(15);
    pub const INIT_HOME_HELLO3: Self = Self(16);
    pub const which_fot: Self = Self(17);
    pub const HE_IS0: Self = Self(18);
    pub const HE_IS1: Self = Self(19);
    pub const HE_IS2: Self = Self(20);
    pub const HE_IS3: Self = Self(21);
    pub const HE_IS4: Self = Self(22);
    pub const HE_IS5: Self = Self(23);
    pub const HE_IS6: Self = Self(24);
    pub const HE_IS7: Self = Self(25);
    pub const we_are_vindicator0: Self = Self(26);
    pub const we_are_vindicator1: Self = Self(27);
    pub const we_are_vindicator2: Self = Self(28);
    pub const WE_GLAD0: Self = Self(29);
    pub const WE_GLAD1: Self = Self(30);
    pub const WE_GLAD2: Self = Self(31);
    pub const WE_GLAD3: Self = Self(32);
    pub const WE_GLAD4: Self = Self(33);
    pub const WE_GLAD5: Self = Self(34);
    pub const quiet_toadies: Self = Self(35);
    pub const TOLD_YOU0: Self = Self(36);
    pub const TOLD_YOU1: Self = Self(37);
    pub const TOLD_YOU2: Self = Self(38);
    pub const TOLD_YOU3: Self = Self(39);
    pub const TOLD_YOU4: Self = Self(40);
    pub const TOLD_YOU5: Self = Self(41);
    pub const TOLD_YOU6: Self = Self(42);
    pub const TOLD_YOU7: Self = Self(43);
    pub const your_race: Self = Self(44);
    pub const YEARS_AGO0: Self = Self(45);
    pub const YEARS_AGO1: Self = Self(46);
    pub const YEARS_AGO2: Self = Self(47);
    pub const YEARS_AGO3: Self = Self(48);
    pub const YEARS_AGO4: Self = Self(49);
    pub const YEARS_AGO5: Self = Self(50);
    pub const YEARS_AGO6: Self = Self(51);
    pub const YEARS_AGO7: Self = Self(52);
    pub const YEARS_AGO8: Self = Self(53);
    pub const YEARS_AGO9: Self = Self(54);
    pub const YEARS_AGO10: Self = Self(55);
    pub const YEARS_AGO11: Self = Self(56);
    pub const YEARS_AGO12: Self = Self(57);
    pub const YEARS_AGO13: Self = Self(58);
    pub const where_from: Self = Self(59);
    pub const TRAVELED_FAR0: Self = Self(60);
    pub const TRAVELED_FAR1: Self = Self(61);
    pub const TRAVELED_FAR2: Self = Self(62);
    pub const TRAVELED_FAR3: Self = Self(63);
    pub const TRAVELED_FAR4: Self = Self(64);
    pub const TRAVELED_FAR5: Self = Self(65);
    pub const what_emergency: Self = Self(66);
    pub const UNDER_ATTACK0: Self = Self(67);
    pub const UNDER_ATTACK1: Self = Self(68);
    pub const UNDER_ATTACK2: Self = Self(69);
    pub const UNDER_ATTACK3: Self = Self(70);
    pub const UNDER_ATTACK4: Self = Self(71);
    pub const UNDER_ATTACK5: Self = Self(72);
    pub const UNDER_ATTACK6: Self = Self(73);
    pub const UNDER_ATTACK7: Self = Self(74);
    pub const UNDER_ATTACK8: Self = Self(75);
    pub const UNDER_ATTACK9: Self = Self(76);
    pub const UNDER_ATTACK10: Self = Self(77);
    pub const UNDER_ATTACK11: Self = Self(78);
    pub const tough_luck: Self = Self(79);
    pub const NOT_HELPFUL0: Self = Self(80);
    pub const NOT_HELPFUL1: Self = Self(81);
    pub const NOT_HELPFUL2: Self = Self(82);
    pub const NOT_HELPFUL3: Self = Self(83);
    pub const NOT_HELPFUL4: Self = Self(84);
    pub const NOT_HELPFUL5: Self = Self(85);
    pub const what_look_like: Self = Self(86);
    pub const LOOK_LIKE0: Self = Self(87);
    pub const LOOK_LIKE1: Self = Self(88);
    pub const LOOK_LIKE2: Self = Self(89);
    pub const LOOK_LIKE3: Self = Self(90);
    pub const valuable_info: Self = Self(91);
    pub const GOODBYE0: Self = Self(92);
    pub const GOODBYE1: Self = Self(93);
    pub const GOODBYE2: Self = Self(94);
    pub const GOODBYE3: Self = Self(95);
    pub const all_very_interesting: Self = Self(96);
    pub const SEE_TOLD_YOU0: Self = Self(97);
    pub const SEE_TOLD_YOU1: Self = Self(98);
    pub const SEE_TOLD_YOU2: Self = Self(99);
    pub const SEE_TOLD_YOU3: Self = Self(100);
    pub const how_can_i_help: Self = Self(101);
    pub const ALLY_WITH_US0: Self = Self(102);
    pub const ALLY_WITH_US1: Self = Self(103);
    pub const ALLY_WITH_US2: Self = Self(104);
    pub const ALLY_WITH_US3: Self = Self(105);
    pub const ALLY_WITH_US4: Self = Self(106);
    pub const ALLY_WITH_US5: Self = Self(107);
    pub const decide_later: Self = Self(108);
    pub const PLEASE_HURRY0: Self = Self(109);
    pub const PLEASE_HURRY1: Self = Self(110);
    pub const EMMISSARIES0: Self = Self(111);
    pub const EMMISSARIES1: Self = Self(112);
    pub const EMMISSARIES2: Self = Self(113);
    pub const EMMISSARIES3: Self = Self(114);
    pub const EMMISSARIES4: Self = Self(115);
    pub const EMMISSARIES5: Self = Self(116);
    pub const EMMISSARIES6: Self = Self(117);
    pub const EMMISSARIES7: Self = Self(118);
    pub const sure: Self = Self(119);
    pub const WE_ALLY0: Self = Self(120);
    pub const WE_ALLY1: Self = Self(121);
    pub const WE_ALLY2: Self = Self(122);
    pub const WE_ALLY3: Self = Self(123);
    pub const WE_ALLY4: Self = Self(124);
    pub const WE_ALLY5: Self = Self(125);
    pub const never: Self = Self(126);
    pub const WE_ENEMIES0: Self = Self(127);
    pub const WE_ENEMIES1: Self = Self(128);
    pub const HOSTILE_HELLO_10: Self = Self(129);
    pub const HOSTILE_HELLO_11: Self = Self(130);
    pub const HOSTILE_HELLO_20: Self = Self(131);
    pub const HOSTILE_HELLO_21: Self = Self(132);
    pub const HOSTILE_HELLO_22: Self = Self(133);
    pub const HOSTILE_HELLO_23: Self = Self(134);
    pub const HOSTILE_HELLO_24: Self = Self(135);
    pub const HOSTILE_HELLO_25: Self = Self(136);
    pub const HOSTILE_HELLO_30: Self = Self(137);
    pub const HOSTILE_HELLO_31: Self = Self(138);
    pub const HOSTILE_HELLO_40: Self = Self(139);
    pub const HOSTILE_HELLO_41: Self = Self(140);
    pub const NEUTRAL_HOME_HELLO_10: Self = Self(141);
    pub const NEUTRAL_HOME_HELLO_11: Self = Self(142);
    pub const NEUTRAL_HOME_HELLO_12: Self = Self(143);
    pub const NEUTRAL_HOME_HELLO_13: Self = Self(144);
    pub const NEUTRAL_HOME_HELLO_20: Self = Self(145);
    pub const NEUTRAL_HOME_HELLO_21: Self = Self(146);
    pub const NEUTRAL_HOME_HELLO_22: Self = Self(147);
    pub const NEUTRAL_HOME_HELLO_23: Self = Self(148);
    pub const ALLIED_HOME_HELLO_10: Self = Self(149);
    pub const ALLIED_HOME_HELLO_11: Self = Self(150);
    pub const ALLIED_HOME_HELLO_12: Self = Self(151);
    pub const ALLIED_HOME_HELLO_13: Self = Self(152);
    pub const ALLIED_HOME_HELLO_20: Self = Self(153);
    pub const ALLIED_HOME_HELLO_21: Self = Self(154);
    pub const ALLIED_HOME_HELLO_22: Self = Self(155);
    pub const ALLIED_HOME_HELLO_23: Self = Self(156);
    pub const ALLIED_HOME_HELLO_24: Self = Self(157);
    pub const ALLIED_HOME_HELLO_25: Self = Self(158);
    pub const ALLIED_HOME_HELLO_26: Self = Self(159);
    pub const ALLIED_HOME_HELLO_27: Self = Self(160);
    pub const ALLIED_HOME_HELLO_30: Self = Self(161);
    pub const ALLIED_HOME_HELLO_31: Self = Self(162);
    pub const ALLIED_HOME_HELLO_40: Self = Self(163);
    pub const ALLIED_HOME_HELLO_41: Self = Self(164);
    pub const THANKS_FOR_RESCUE0: Self = Self(165);
    pub const THANKS_FOR_RESCUE1: Self = Self(166);
    pub const THANKS_FOR_RESCUE2: Self = Self(167);
    pub const THANKS_FOR_RESCUE3: Self = Self(168);
    pub const THANKS_FOR_RESCUE4: Self = Self(169);
    pub const THANKS_FOR_RESCUE5: Self = Self(170);
    pub const THANKS_FOR_RESCUE6: Self = Self(171);
    pub const THANKS_FOR_RESCUE7: Self = Self(172);
    pub const THANKS_FOR_RESCUE8: Self = Self(173);
    pub const THANKS_FOR_RESCUE9: Self = Self(174);
    pub const THANKS_FOR_RESCUE10: Self = Self(175);
    pub const THANKS_FOR_RESCUE11: Self = Self(176);
    pub const bye_homeworld: Self = Self(177);
    pub const GOODBYE_HOME0: Self = Self(178);
    pub const GOODBYE_HOME1: Self = Self(179);
    pub const whats_up_homeworld: Self = Self(180);
    pub const GENERAL_INFO_10: Self = Self(181);
    pub const GENERAL_INFO_11: Self = Self(182);
    pub const GENERAL_INFO_12: Self = Self(183);
    pub const GENERAL_INFO_13: Self = Self(184);
    pub const GENERAL_INFO_20: Self = Self(185);
    pub const GENERAL_INFO_21: Self = Self(186);
    pub const GENERAL_INFO_22: Self = Self(187);
    pub const GENERAL_INFO_23: Self = Self(188);
    pub const GENERAL_INFO_24: Self = Self(189);
    pub const GENERAL_INFO_25: Self = Self(190);
    pub const GENERAL_INFO_26: Self = Self(191);
    pub const GENERAL_INFO_27: Self = Self(192);
    pub const GENERAL_INFO_30: Self = Self(193);
    pub const GENERAL_INFO_31: Self = Self(194);
    pub const GENERAL_INFO_32: Self = Self(195);
    pub const GENERAL_INFO_33: Self = Self(196);
    pub const GENERAL_INFO_34: Self = Self(197);
    pub const GENERAL_INFO_35: Self = Self(198);
    pub const GENERAL_INFO_40: Self = Self(199);
    pub const GENERAL_INFO_41: Self = Self(200);
    pub const GENERAL_INFO_42: Self = Self(201);
    pub const GENERAL_INFO_43: Self = Self(202);
    pub const GENERAL_INFO_44: Self = Self(203);
    pub const GENERAL_INFO_45: Self = Self(204);
    pub const GENERAL_INFO_46: Self = Self(205);
    pub const GENERAL_INFO_47: Self = Self(206);
    pub const GENERAL_INFO_48: Self = Self(207);
    pub const GENERAL_INFO_49: Self = Self(208);
    pub const GENERAL_INFO_410: Self = Self(209);
    pub const GENERAL_INFO_411: Self = Self(210);
    pub const any_war_news: Self = Self(211);
    pub const UTWIG_DELAY0: Self = Self(212);
    pub const UTWIG_DELAY1: Self = Self(213);
    pub const UTWIG_DELAY2: Self = Self(214);
    pub const UTWIG_DELAY3: Self = Self(215);
    pub const UTWIG_DELAY4: Self = Self(216);
    pub const UTWIG_DELAY5: Self = Self(217);
    pub const UTWIG_DELAY6: Self = Self(218);
    pub const UTWIG_DELAY7: Self = Self(219);
    pub const UTWIG_DELAY8: Self = Self(220);
    pub const UTWIG_DELAY9: Self = Self(221);
    pub const UTWIG_DELAY10: Self = Self(222);
    pub const UTWIG_DELAY11: Self = Self(223);
    pub const UTWIG_DELAY12: Self = Self(224);
    pub const UTWIG_DELAY13: Self = Self(225);
    pub const KOHRAH_WINNING0: Self = Self(226);
    pub const KOHRAH_WINNING1: Self = Self(227);
    pub const KOHRAH_WINNING2: Self = Self(228);
    pub const KOHRAH_WINNING3: Self = Self(229);
    pub const KOHRAH_WINNING4: Self = Self(230);
    pub const KOHRAH_WINNING5: Self = Self(231);
    pub const KOHRAH_WINNING6: Self = Self(232);
    pub const KOHRAH_WINNING7: Self = Self(233);
    pub const KOHRAH_WINNING8: Self = Self(234);
    pub const KOHRAH_WINNING9: Self = Self(235);
    pub const URQUAN_NEARLY_GONE0: Self = Self(236);
    pub const URQUAN_NEARLY_GONE1: Self = Self(237);
    pub const URQUAN_NEARLY_GONE2: Self = Self(238);
    pub const URQUAN_NEARLY_GONE3: Self = Self(239);
    pub const URQUAN_NEARLY_GONE4: Self = Self(240);
    pub const URQUAN_NEARLY_GONE5: Self = Self(241);
    pub const KOHRAH_FRENZY0: Self = Self(242);
    pub const KOHRAH_FRENZY1: Self = Self(243);
    pub const KOHRAH_FRENZY2: Self = Self(244);
    pub const KOHRAH_FRENZY3: Self = Self(245);
    pub const KOHRAH_FRENZY4: Self = Self(246);
    pub const KOHRAH_FRENZY5: Self = Self(247);
    pub const KOHRAH_FRENZY6: Self = Self(248);
    pub const KOHRAH_FRENZY7: Self = Self(249);
    pub const KOHRAH_FRENZY8: Self = Self(250);
    pub const KOHRAH_FRENZY9: Self = Self(251);
    pub const KOHRAH_FRENZY10: Self = Self(252);
    pub const KOHRAH_FRENZY11: Self = Self(253);
    pub const NO_WAR_NEWS0: Self = Self(254);
    pub const NO_WAR_NEWS1: Self = Self(255);
    pub const i_want_alliance: Self = Self(256);
    pub const GOOD0: Self = Self(257);
    pub const GOOD1: Self = Self(258);
    pub const GOOD2: Self = Self(259);
    pub const GOOD3: Self = Self(260);
    pub const GOOD4: Self = Self(261);
    pub const GOOD5: Self = Self(262);
    pub const GOOD6: Self = Self(263);
    pub const GOOD7: Self = Self(264);
    pub const GOOD8: Self = Self(265);
    pub const GOOD9: Self = Self(266);
    pub const want_specific_info: Self = Self(267);
    pub const WHAT_SPECIFIC_INFO0: Self = Self(268);
    pub const WHAT_SPECIFIC_INFO1: Self = Self(269);
    pub const enough_info: Self = Self(270);
    pub const OK_ENOUGH_INFO: Self = Self(271);
    pub const what_about_others: Self = Self(272);
    pub const ABOUT_OTHERS0: Self = Self(273);
    pub const ABOUT_OTHERS1: Self = Self(274);
    pub const ABOUT_OTHERS2: Self = Self(275);
    pub const ABOUT_OTHERS3: Self = Self(276);
    pub const ABOUT_OTHERS4: Self = Self(277);
    pub const ABOUT_OTHERS5: Self = Self(278);
    pub const ABOUT_OTHERS6: Self = Self(279);
    pub const ABOUT_OTHERS7: Self = Self(280);
    pub const ABOUT_OTHERS8: Self = Self(281);
    pub const ABOUT_OTHERS9: Self = Self(282);
    pub const ABOUT_OTHERS10: Self = Self(283);
    pub const ABOUT_OTHERS11: Self = Self(284);
    pub const ABOUT_OTHERS12: Self = Self(285);
    pub const ABOUT_OTHERS13: Self = Self(286);
    pub const what_about_zebranky: Self = Self(287);
    pub const ABOUT_ZEBRANKY0: Self = Self(288);
    pub const ABOUT_ZEBRANKY1: Self = Self(289);
    pub const ABOUT_ZEBRANKY2: Self = Self(290);
    pub const ABOUT_ZEBRANKY3: Self = Self(291);
    pub const ABOUT_ZEBRANKY4: Self = Self(292);
    pub const ABOUT_ZEBRANKY5: Self = Self(293);
    pub const ABOUT_ZEBRANKY6: Self = Self(294);
    pub const ABOUT_ZEBRANKY7: Self = Self(295);
    pub const what_about_past: Self = Self(296);
    pub const ABOUT_PAST0: Self = Self(297);
    pub const ABOUT_PAST1: Self = Self(298);
    pub const ABOUT_PAST2: Self = Self(299);
    pub const ABOUT_PAST3: Self = Self(300);
    pub const ABOUT_PAST4: Self = Self(301);
    pub const ABOUT_PAST5: Self = Self(302);
    pub const ABOUT_PAST6: Self = Self(303);
    pub const ABOUT_PAST7: Self = Self(304);
    pub const ABOUT_PAST8: Self = Self(305);
    pub const ABOUT_PAST9: Self = Self(306);
    pub const ABOUT_PAST10: Self = Self(307);
    pub const ABOUT_PAST11: Self = Self(308);
    pub const what_about_stinger: Self = Self(309);
    pub const ABOUT_STINGER0: Self = Self(310);
    pub const ABOUT_STINGER1: Self = Self(311);
    pub const ABOUT_STINGER2: Self = Self(312);
    pub const ABOUT_STINGER3: Self = Self(313);
    pub const ABOUT_STINGER4: Self = Self(314);
    pub const ABOUT_STINGER5: Self = Self(315);
    pub const what_about_guy_in_back: Self = Self(316);
    pub const ABOUT_GUY0: Self = Self(317);
    pub const ABOUT_GUY1: Self = Self(318);
    pub const name_1: Self = Self(319);
    pub const name_2: Self = Self(320);
    pub const name_3: Self = Self(321);
    pub const name_40: Self = Self(322);
    pub const name_41: Self = Self(323);
    pub const OUT_TAKES0: Self = Self(324);
    pub const OUT_TAKES1: Self = Self(325);
    pub const OUT_TAKES2: Self = Self(326);
    pub const OUT_TAKES3: Self = Self(327);
    pub const OUT_TAKES4: Self = Self(328);
    pub const OUT_TAKES5: Self = Self(329);
    pub const OUT_TAKES6: Self = Self(330);
    pub const OUT_TAKES7: Self = Self(331);
    pub const OUT_TAKES8: Self = Self(332);
    pub const OUT_TAKES9: Self = Self(333);
    pub const OUT_TAKES10: Self = Self(334);
    pub const OUT_TAKES11: Self = Self(335);
    pub const OUT_TAKES12: Self = Self(336);
    pub const OUT_TAKES13: Self = Self(337);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_2(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_2 {
    pub const DEAD_GUY: Self = Self(0);
    pub const GOOD_GUY: Self = Self(1);
    pub const BAD_GUY: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_3(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_3 {
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
pub struct C2Rust_Unnamed_4(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_4 {
    pub const ARILOU_ENTRANCE_EVENT: Self = Self(0);
    pub const ARILOU_EXIT_EVENT: Self = Self(1);
    pub const HYPERSPACE_ENCOUNTER_EVENT: Self = Self(2);
    pub const KOHR_AH_VICTORIOUS_EVENT: Self = Self(3);
    pub const ADVANCE_PKUNK_MISSION: Self = Self(4);
    pub const ADVANCE_THRADD_MISSION: Self = Self(5);
    pub const ZOQFOT_DISTRESS_EVENT: Self = Self(6);
    pub const ZOQFOT_DEATH_EVENT: Self = Self(7);
    pub const SHOFIXTI_RETURN_EVENT: Self = Self(8);
    pub const ADVANCE_UTWIG_SUPOX_MISSION: Self = Self(9);
    pub const KOHR_AH_GENOCIDE_EVENT: Self = Self(10);
    pub const SPATHI_SHIELD_EVENT: Self = Self(11);
    pub const ADVANCE_ILWRATH_MISSION: Self = Self(12);
    pub const ADVANCE_MYCON_MISSION: Self = Self(13);
    pub const ARILOU_UMGAH_CHECK: Self = Self(14);
    pub const YEHAT_REBEL_EVENT: Self = Self(15);
    pub const SLYLANDRO_RAMP_UP: Self = Self(16);
    pub const SLYLANDRO_RAMP_DOWN: Self = Self(17);
    pub const NUM_EVENTS: Self = Self(18);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_5(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_5 {
    pub const ZOQ_ALIEN: Self = Self(0);
    pub const FOT_ALIEN: Self = Self(1);
    pub const PIK_ALIEN: Self = Self(2);
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
static mut zoqfot_desc: LOCDATA = LOCDATA {
    init_encounter_func: None,
    post_encounter_func: None,
    uninit_encounter_func: None,
    AlienFrameRes: b"comm.zoqfotpik.graphics\0".as_ptr() as RESOURCE,
    AlienFontRes: b"comm.zoqfotpik.font\0".as_ptr() as RESOURCE,
    AlienTextFColor: Color {
        r: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
            | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
        g: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
            | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
        b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
            | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
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
    AlienColorMapRes: b"comm.zoqfotpik.colortable\0".as_ptr() as RESOURCE,
    AlienSongRes: b"comm.zoqfotpik.music\0".as_ptr() as RESOURCE,
    AlienAltSongRes: ::core::ptr::null::<::core::ffi::c_char>(),
    AlienSongFlags: 0 as LDAS_FLAGS,
    ConversationPhrasesRes: b"comm.zoqfotpik.dialogue\0".as_ptr() as RESOURCE,
    NumAnimations: 3 as COUNT,
    AlienAmbientArray: [
        ANIMATION_DESC {
            StartIndex: 1 as COUNT,
            NumFrames: 4 as BYTE,
            AnimFlags: ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as BYTE,
            BaseFrameRate: (840 as ::core::ffi::c_int / 24 as ::core::ffi::c_int)
                as COUNT,
            RandomFrameRate: 0 as COUNT,
            BaseRestartRate: 0 as COUNT,
            RandomRestartRate: (840 as ::core::ffi::c_int * 10 as ::core::ffi::c_int)
                as COUNT,
            BlockMask: 0 as DWORD,
        },
        ANIMATION_DESC {
            StartIndex: 5 as COUNT,
            NumFrames: 5 as BYTE,
            AnimFlags: ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as BYTE,
            BaseFrameRate: (840 as ::core::ffi::c_int * 7 as ::core::ffi::c_int
                / 120 as ::core::ffi::c_int) as COUNT,
            RandomFrameRate: 0 as COUNT,
            BaseRestartRate: (840 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                as COUNT,
            RandomRestartRate: 0 as COUNT,
            BlockMask: 0 as DWORD,
        },
        ANIMATION_DESC {
            StartIndex: 10 as COUNT,
            NumFrames: 8 as BYTE,
            AnimFlags: ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as BYTE,
            BaseFrameRate: (840 as ::core::ffi::c_int / 15 as ::core::ffi::c_int)
                as COUNT,
            RandomFrameRate: 0 as COUNT,
            BaseRestartRate: 0 as COUNT,
            RandomRestartRate: (840 as ::core::ffi::c_int * 10 as ::core::ffi::c_int)
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
        StartIndex: 23 as COUNT,
        NumFrames: 3 as BYTE,
        AnimFlags: 0 as BYTE,
        BaseFrameRate: (840 as ::core::ffi::c_int / 30 as ::core::ffi::c_int) as COUNT,
        RandomFrameRate: 0 as COUNT,
        BaseRestartRate: 0 as COUNT,
        RandomRestartRate: 0 as COUNT,
        BlockMask: 0 as DWORD,
    },
    AlienTalkDesc: ANIMATION_DESC {
        StartIndex: 18 as COUNT,
        NumFrames: 5 as BYTE,
        AnimFlags: 0 as BYTE,
        BaseFrameRate: (840 as ::core::ffi::c_int / 15 as ::core::ffi::c_int) as COUNT,
        RandomFrameRate: 0 as COUNT,
        BaseRestartRate: (840 as ::core::ffi::c_int / 12 as ::core::ffi::c_int) as COUNT,
        RandomRestartRate: 0 as COUNT,
        BlockMask: 0 as DWORD,
    },
    AlienNumberSpeech: ::core::ptr::null::<NUMBER_SPEECH_DESC>(),
    AlienFrame: ::core::ptr::null_mut::<FRAME_DESC>(),
    AlienFont: ::core::ptr::null_mut::<FONT_DESC>(),
    AlienColorMap: ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>(),
    AlienSong: ::core::ptr::null_mut::<*mut TFB_SoundSample>(),
    ConversationPhrases: ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>(),
};
static mut LastAlien: ::core::ffi::c_int = 0;
unsafe extern "C" fn SelectAlienZOQ(mut arg: CallbackArg) {
    if LastAlien != C2Rust_Unnamed_5::ZOQ_ALIEN.0 as ::core::ffi::c_int {
        if LastAlien != C2Rust_Unnamed_5::FOT_ALIEN.0 as ::core::ffi::c_int {
            CommData.AlienTransitionDesc.AnimFlags = (CommData
                .AlienTransitionDesc
                .AnimFlags as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as BYTE;
        }
        LastAlien = C2Rust_Unnamed_5::ZOQ_ALIEN.0 as ::core::ffi::c_int;
        CommData.AlienTransitionDesc.AnimFlags = (CommData.AlienTransitionDesc.AnimFlags
            as ::core::ffi::c_int | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int)
            as BYTE;
        CommData.AlienTransitionDesc.StartIndex = 23 as COUNT;
        CommData.AlienTalkDesc.StartIndex = 18 as COUNT;
        CommData.AlienTalkDesc.NumFrames = 5 as BYTE;
        CommData.AlienAmbientArray[1usize].AnimFlags = (CommData
            .AlienAmbientArray[1usize]
            .AnimFlags as ::core::ffi::c_int
            & !((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)) as BYTE;
        CommData.AlienTextBaseline.x = (1 as ::core::ffi::c_int
            + (ScreenWidth - 64 as ::core::ffi::c_int
                - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                - 14 as ::core::ffi::c_int
                - ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
                >> 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)) as SWORD
            as COORD;
        CommData.AlienTextBaseline.y = 24 as COORD;
        CommData.AlienTextFColor = colorFromRgb15(
            0x1f as BYTE,
            0x1f as BYTE,
            0x1f as BYTE,
        );
        CommData.AlienTextBColor = colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE);
    }
}
unsafe extern "C" fn SelectAlienPIK(mut arg: CallbackArg) {
    if LastAlien != C2Rust_Unnamed_5::PIK_ALIEN.0 as ::core::ffi::c_int {
        if LastAlien != C2Rust_Unnamed_5::FOT_ALIEN.0 as ::core::ffi::c_int {
            CommData.AlienTransitionDesc.AnimFlags = (CommData
                .AlienTransitionDesc
                .AnimFlags as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as BYTE;
        }
        LastAlien = C2Rust_Unnamed_5::PIK_ALIEN.0 as ::core::ffi::c_int;
        CommData.AlienTransitionDesc.AnimFlags = (CommData.AlienTransitionDesc.AnimFlags
            as ::core::ffi::c_int | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int)
            as BYTE;
        CommData.AlienTransitionDesc.StartIndex = 26 as COUNT;
        CommData.AlienTalkDesc.StartIndex = 29 as COUNT;
        CommData.AlienTalkDesc.NumFrames = 2 as BYTE;
        CommData.AlienAmbientArray[1usize].AnimFlags = (CommData
            .AlienAmbientArray[1usize]
            .AnimFlags as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as BYTE;
        CommData.AlienTextBaseline.x = (ScreenWidth - 64 as ::core::ffi::c_int
            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
            - 14 as ::core::ffi::c_int
            - (1 as ::core::ffi::c_int
                + (ScreenWidth - 64 as ::core::ffi::c_int
                    - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    - 14 as ::core::ffi::c_int
                    - ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
                    >> 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int))) as SWORD
            as COORD;
        CommData.AlienTextBaseline.y = 24 as COORD;
        CommData.AlienTextFColor = colorFromRgb15(
            0x1f as BYTE,
            0x1f as BYTE,
            0x1f as BYTE,
        );
        CommData.AlienTextBColor = colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE);
    }
}
unsafe extern "C" fn ZFPTalkSegue(mut wait_track: COUNT) {
    LastAlien = C2Rust_Unnamed_5::FOT_ALIEN.0 as ::core::ffi::c_int;
    SelectAlienZOQ(::core::ptr::null_mut::<::core::ffi::c_void>());
    AlienTalkSegue(wait_track);
}
unsafe extern "C" fn ExitConversation(mut R: RESPONSE_REF) {
    setSegue(Segue::Segue_peace);
    if R as ::core::ffi::c_int == C2Rust_Unnamed_1::bye_homeworld.0 as ::core::ffi::c_int
    {
        NPCPhrase_cb(
            C2Rust_Unnamed_1::GOODBYE_HOME0.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::GOODBYE_HOME1.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        ZFPTalkSegue(!(0 as ::core::ffi::c_int) as COUNT);
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::decide_later.0 as ::core::ffi::c_int
    {
        NPCPhrase_cb(
            C2Rust_Unnamed_1::PLEASE_HURRY0.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::PLEASE_HURRY1.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        ZFPTalkSegue(!(0 as ::core::ffi::c_int) as COUNT);
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::valuable_info.0 as ::core::ffi::c_int
    {
        NPCPhrase_cb(
            C2Rust_Unnamed_1::GOODBYE0.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::GOODBYE1.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::GOODBYE2.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::GOODBYE3.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        ZFPTalkSegue(!(0 as ::core::ffi::c_int) as COUNT);
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::how_can_i_help.0 as ::core::ffi::c_int
    {
        NPCPhrase_cb(
            C2Rust_Unnamed_1::EMMISSARIES0.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::EMMISSARIES1.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::EMMISSARIES2.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::EMMISSARIES3.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::EMMISSARIES4.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::EMMISSARIES5.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::EMMISSARIES6.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::EMMISSARIES7.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        ZFPTalkSegue(!(0 as ::core::ffi::c_int) as COUNT);
    } else if R as ::core::ffi::c_int == C2Rust_Unnamed_1::sure.0 as ::core::ffi::c_int {
        NPCPhrase_cb(
            C2Rust_Unnamed_1::WE_ALLY0.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::WE_ALLY1.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::WE_ALLY2.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::WE_ALLY3.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::WE_ALLY4.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::WE_ALLY5.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        ZFPTalkSegue(!(0 as ::core::ffi::c_int) as COUNT);
        SetRaceAllied(
            C2Rust_Unnamed_3::ZOQFOTPIK_SHIP.0 as ::core::ffi::c_int as COUNT,
            BOOLEAN::TRUE,
        );
        AddEvent(
            EVENT_TYPE::RELATIVE_EVENT,
            3 as COUNT,
            0 as COUNT,
            0 as COUNT,
            C2Rust_Unnamed_4::ZOQFOT_DISTRESS_EVENT.0 as ::core::ffi::c_int as BYTE,
        );
        setGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed::ZOQFOT_HOME_VISITS.0 as ::core::ffi::c_int,
            C2Rust_Unnamed::END_ZOQFOT_HOME_VISITS.0 as ::core::ffi::c_int,
            0 as BYTE,
        );
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::all_very_interesting.0 as ::core::ffi::c_int
    {
        NPCPhrase_cb(
            C2Rust_Unnamed_1::SEE_TOLD_YOU0.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::SEE_TOLD_YOU1.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::SEE_TOLD_YOU2.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::SEE_TOLD_YOU3.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        ZFPTalkSegue(!(0 as ::core::ffi::c_int) as COUNT);
        setGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed::ZOQFOT_HOSTILE.0 as ::core::ffi::c_int,
            C2Rust_Unnamed::END_ZOQFOT_HOSTILE.0 as ::core::ffi::c_int,
            1 as BYTE,
        );
        setGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed::ZOQFOT_HOME_VISITS.0 as ::core::ffi::c_int,
            C2Rust_Unnamed::END_ZOQFOT_HOME_VISITS.0 as ::core::ffi::c_int,
            0 as BYTE,
        );
        setSegue(Segue::Segue_hostile);
    } else if R as ::core::ffi::c_int == C2Rust_Unnamed_1::never.0 as ::core::ffi::c_int
    {
        NPCPhrase_cb(
            C2Rust_Unnamed_1::WE_ENEMIES0.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::WE_ENEMIES1.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        ZFPTalkSegue(!(0 as ::core::ffi::c_int) as COUNT);
        setGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed::ZOQFOT_HOME_VISITS.0 as ::core::ffi::c_int,
            C2Rust_Unnamed::END_ZOQFOT_HOME_VISITS.0 as ::core::ffi::c_int,
            0 as BYTE,
        );
        setGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed::ZOQFOT_HOSTILE.0 as ::core::ffi::c_int,
            C2Rust_Unnamed::END_ZOQFOT_HOSTILE.0 as ::core::ffi::c_int,
            1 as BYTE,
        );
        setSegue(Segue::Segue_hostile);
    }
}
unsafe extern "C" fn FormAlliance(mut R: RESPONSE_REF) {
    NPCPhrase_cb(
        C2Rust_Unnamed_1::ALLY_WITH_US0.0 as ::core::ffi::c_int,
        Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
    );
    NPCPhrase_cb(
        C2Rust_Unnamed_1::ALLY_WITH_US1.0 as ::core::ffi::c_int,
        Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
    );
    NPCPhrase_cb(
        C2Rust_Unnamed_1::ALLY_WITH_US2.0 as ::core::ffi::c_int,
        Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
    );
    NPCPhrase_cb(
        C2Rust_Unnamed_1::ALLY_WITH_US3.0 as ::core::ffi::c_int,
        Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
    );
    NPCPhrase_cb(
        C2Rust_Unnamed_1::ALLY_WITH_US4.0 as ::core::ffi::c_int,
        Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
    );
    NPCPhrase_cb(
        C2Rust_Unnamed_1::ALLY_WITH_US5.0 as ::core::ffi::c_int,
        Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
    );
    ZFPTalkSegue(!(0 as ::core::ffi::c_int) as COUNT);
    DoResponsePhrase(
        C2Rust_Unnamed_1::sure.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(ExitConversation as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    );
    DoResponsePhrase(
        C2Rust_Unnamed_1::never.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(ExitConversation as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    );
    DoResponsePhrase(
        C2Rust_Unnamed_1::decide_later.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(ExitConversation as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    );
}
unsafe extern "C" fn ZoqFotIntro(mut R: RESPONSE_REF) {
    if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::we_are_vindicator0.0 as ::core::ffi::c_int
    {
        NPCPhrase_cb(
            C2Rust_Unnamed_1::WE_GLAD0.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::WE_GLAD1.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::WE_GLAD2.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::WE_GLAD3.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::WE_GLAD4.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::WE_GLAD5.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        ZFPTalkSegue(!(0 as ::core::ffi::c_int) as COUNT);
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_1::we_are_vindicator0.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::your_race.0 as ::core::ffi::c_int
    {
        NPCPhrase_cb(
            C2Rust_Unnamed_1::YEARS_AGO0.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::YEARS_AGO1.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::YEARS_AGO2.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::YEARS_AGO3.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::YEARS_AGO4.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::YEARS_AGO5.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::YEARS_AGO6.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::YEARS_AGO7.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::YEARS_AGO8.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::YEARS_AGO9.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::YEARS_AGO10.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::YEARS_AGO11.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::YEARS_AGO12.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::YEARS_AGO13.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        ZFPTalkSegue(!(0 as ::core::ffi::c_int) as COUNT);
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_1::your_race.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::where_from.0 as ::core::ffi::c_int
    {
        NPCPhrase_cb(
            C2Rust_Unnamed_1::TRAVELED_FAR0.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::TRAVELED_FAR1.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::TRAVELED_FAR2.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::TRAVELED_FAR3.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::TRAVELED_FAR4.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::TRAVELED_FAR5.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        ZFPTalkSegue(!(0 as ::core::ffi::c_int) as COUNT);
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_1::where_from.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::what_emergency.0 as ::core::ffi::c_int
    {
        NPCPhrase_cb(
            C2Rust_Unnamed_1::UNDER_ATTACK0.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::UNDER_ATTACK1.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::UNDER_ATTACK2.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::UNDER_ATTACK3.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::UNDER_ATTACK4.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::UNDER_ATTACK5.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::UNDER_ATTACK6.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::UNDER_ATTACK7.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::UNDER_ATTACK8.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::UNDER_ATTACK9.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::UNDER_ATTACK10.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::UNDER_ATTACK11.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        ZFPTalkSegue(!(0 as ::core::ffi::c_int) as COUNT);
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_1::what_emergency.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::tough_luck.0 as ::core::ffi::c_int
    {
        NPCPhrase_cb(
            C2Rust_Unnamed_1::NOT_HELPFUL0.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::NOT_HELPFUL1.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::NOT_HELPFUL2.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::NOT_HELPFUL3.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::NOT_HELPFUL4.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::NOT_HELPFUL5.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        ZFPTalkSegue(!(0 as ::core::ffi::c_int) as COUNT);
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_1::tough_luck.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::what_look_like.0 as ::core::ffi::c_int
    {
        NPCPhrase_cb(
            C2Rust_Unnamed_1::LOOK_LIKE0.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::LOOK_LIKE1.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::LOOK_LIKE2.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::LOOK_LIKE3.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        ZFPTalkSegue(!(0 as ::core::ffi::c_int) as COUNT);
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_1::what_look_like.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    }
    if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_1::your_race.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        || *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_1::where_from.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        || *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_1::what_emergency.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        if *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_1::your_race.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        {
            DoResponsePhrase(
                C2Rust_Unnamed_1::your_race.0 as ::core::ffi::c_int as RESPONSE_REF,
                Some(ZoqFotIntro as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            );
        }
        if *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_1::where_from.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        {
            DoResponsePhrase(
                C2Rust_Unnamed_1::where_from.0 as ::core::ffi::c_int as RESPONSE_REF,
                Some(ZoqFotIntro as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            );
        }
        if *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_1::what_emergency.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        {
            DoResponsePhrase(
                C2Rust_Unnamed_1::what_emergency.0 as ::core::ffi::c_int as RESPONSE_REF,
                Some(ZoqFotIntro as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            );
        }
    } else {
        if *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_1::tough_luck.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        {
            DoResponsePhrase(
                C2Rust_Unnamed_1::tough_luck.0 as ::core::ffi::c_int as RESPONSE_REF,
                Some(ZoqFotIntro as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            );
        }
        if *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_1::what_look_like.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        {
            DoResponsePhrase(
                C2Rust_Unnamed_1::what_look_like.0 as ::core::ffi::c_int as RESPONSE_REF,
                Some(ZoqFotIntro as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            );
        }
        DoResponsePhrase(
            C2Rust_Unnamed_1::all_very_interesting.0 as ::core::ffi::c_int
                as RESPONSE_REF,
            Some(ExitConversation as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        );
        if getGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed::GLOBAL_FLAGS_AND_DATA.0 as ::core::ffi::c_int,
            C2Rust_Unnamed::END_GLOBAL_FLAGS_AND_DATA.0 as ::core::ffi::c_int,
        ) as ::core::ffi::c_int & (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int
            != 0
        {
            DoResponsePhrase(
                C2Rust_Unnamed_1::how_can_i_help.0 as ::core::ffi::c_int as RESPONSE_REF,
                Some(FormAlliance as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            );
        } else {
            DoResponsePhrase(
                C2Rust_Unnamed_1::how_can_i_help.0 as ::core::ffi::c_int as RESPONSE_REF,
                Some(ExitConversation as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            );
        }
        DoResponsePhrase(
            C2Rust_Unnamed_1::valuable_info.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(ExitConversation as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        );
    };
}
unsafe extern "C" fn AquaintZoqFot(mut R: RESPONSE_REF) {
    if R as ::core::ffi::c_int == C2Rust_Unnamed_1::which_fot.0 as ::core::ffi::c_int {
        NPCPhrase_cb(
            C2Rust_Unnamed_1::HE_IS0.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::HE_IS1.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::HE_IS2.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::HE_IS3.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::HE_IS4.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::HE_IS5.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::HE_IS6.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::HE_IS7.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        ZFPTalkSegue(!(0 as ::core::ffi::c_int) as COUNT);
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_1::which_fot.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::quiet_toadies.0 as ::core::ffi::c_int
    {
        NPCPhrase_cb(
            C2Rust_Unnamed_1::TOLD_YOU0.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::TOLD_YOU1.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::TOLD_YOU2.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::TOLD_YOU3.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::TOLD_YOU4.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::TOLD_YOU5.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::TOLD_YOU6.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::TOLD_YOU7.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        ZFPTalkSegue(!(0 as ::core::ffi::c_int) as COUNT);
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_1::quiet_toadies.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    }
    if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_1::we_are_vindicator0.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        let mut buf: [UNICODE; 256] = [0; 256];
        GetAllianceName(
            &raw mut buf as *mut UNICODE,
            C2Rust_Unnamed_1::name_1.0 as ::core::ffi::c_int as RESPONSE_REF,
        );
        construct_response(
            &raw mut shared_phrase_buf as *mut UNICODE,
            C2Rust_Unnamed_1::we_are_vindicator0.0 as ::core::ffi::c_int,
            &raw mut buf as *mut UNICODE,
            C2Rust_Unnamed_1::we_are_vindicator1.0 as ::core::ffi::c_int,
            &raw mut GlobData.SIS_state.ShipName as *mut UNICODE,
            C2Rust_Unnamed_1::we_are_vindicator2.0 as ::core::ffi::c_int,
            ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut UNICODE,
        );
    }
    if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_1::which_fot.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        DoResponsePhrase(
            C2Rust_Unnamed_1::which_fot.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(AquaintZoqFot as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        );
    }
    if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_1::we_are_vindicator0.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        DoResponsePhrase(
            C2Rust_Unnamed_1::we_are_vindicator0.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(ZoqFotIntro as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            &raw mut shared_phrase_buf as *mut UNICODE,
        );
    }
    if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_1::quiet_toadies.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        DoResponsePhrase(
            C2Rust_Unnamed_1::quiet_toadies.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(AquaintZoqFot as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        );
    }
    DoResponsePhrase(
        C2Rust_Unnamed_1::all_very_interesting.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(ExitConversation as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    );
    DoResponsePhrase(
        C2Rust_Unnamed_1::valuable_info.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(ExitConversation as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    );
}
unsafe extern "C" fn ZoqFotInfo(mut R: RESPONSE_REF) {
    let mut InfoLeft: BYTE = 0;
    if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::want_specific_info.0 as ::core::ffi::c_int
    {
        NPCPhrase_cb(
            C2Rust_Unnamed_1::WHAT_SPECIFIC_INFO0.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::WHAT_SPECIFIC_INFO1.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        ZFPTalkSegue(!(0 as ::core::ffi::c_int) as COUNT);
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::what_about_others.0 as ::core::ffi::c_int
    {
        NPCPhrase_cb(
            C2Rust_Unnamed_1::ABOUT_OTHERS0.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::ABOUT_OTHERS1.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::ABOUT_OTHERS2.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::ABOUT_OTHERS3.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::ABOUT_OTHERS4.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::ABOUT_OTHERS5.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::ABOUT_OTHERS6.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::ABOUT_OTHERS7.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::ABOUT_OTHERS8.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::ABOUT_OTHERS9.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::ABOUT_OTHERS10.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::ABOUT_OTHERS11.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::ABOUT_OTHERS12.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::ABOUT_OTHERS13.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        ZFPTalkSegue(!(0 as ::core::ffi::c_int) as COUNT);
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_1::what_about_others.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::what_about_zebranky.0 as ::core::ffi::c_int
    {
        NPCPhrase_cb(
            C2Rust_Unnamed_1::ABOUT_ZEBRANKY0.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::ABOUT_ZEBRANKY1.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::ABOUT_ZEBRANKY2.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::ABOUT_ZEBRANKY3.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::ABOUT_ZEBRANKY4.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::ABOUT_ZEBRANKY5.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::ABOUT_ZEBRANKY6.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::ABOUT_ZEBRANKY7.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        ZFPTalkSegue(!(0 as ::core::ffi::c_int) as COUNT);
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_1::what_about_zebranky.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::what_about_stinger.0 as ::core::ffi::c_int
    {
        NPCPhrase_cb(
            C2Rust_Unnamed_1::ABOUT_STINGER0.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::ABOUT_STINGER1.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::ABOUT_STINGER2.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::ABOUT_STINGER3.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::ABOUT_STINGER4.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::ABOUT_STINGER5.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        ZFPTalkSegue(!(0 as ::core::ffi::c_int) as COUNT);
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_1::what_about_stinger.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::what_about_guy_in_back.0 as ::core::ffi::c_int
    {
        NPCPhrase_cb(
            C2Rust_Unnamed_1::ABOUT_GUY0.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::ABOUT_GUY1.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        ZFPTalkSegue(!(0 as ::core::ffi::c_int) as COUNT);
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_1::what_about_guy_in_back.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::what_about_past.0 as ::core::ffi::c_int
    {
        NPCPhrase_cb(
            C2Rust_Unnamed_1::ABOUT_PAST0.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::ABOUT_PAST1.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::ABOUT_PAST2.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::ABOUT_PAST3.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::ABOUT_PAST4.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::ABOUT_PAST5.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::ABOUT_PAST6.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::ABOUT_PAST7.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::ABOUT_PAST8.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::ABOUT_PAST9.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::ABOUT_PAST10.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::ABOUT_PAST11.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        ZFPTalkSegue(!(0 as ::core::ffi::c_int) as COUNT);
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_1::what_about_past.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    }
    InfoLeft = BOOLEAN::FALSE.0 as ::core::ffi::c_int as BYTE;
    if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_1::what_about_others.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        DoResponsePhrase(
            C2Rust_Unnamed_1::what_about_others.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(ZoqFotInfo as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        );
        InfoLeft = BOOLEAN::TRUE.0 as ::core::ffi::c_int as BYTE;
    }
    if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_1::what_about_zebranky.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        DoResponsePhrase(
            C2Rust_Unnamed_1::what_about_zebranky.0 as ::core::ffi::c_int
                as RESPONSE_REF,
            Some(ZoqFotInfo as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        );
        InfoLeft = BOOLEAN::TRUE.0 as ::core::ffi::c_int as BYTE;
    }
    if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_1::what_about_stinger.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        DoResponsePhrase(
            C2Rust_Unnamed_1::what_about_stinger.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(ZoqFotInfo as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        );
        InfoLeft = BOOLEAN::TRUE.0 as ::core::ffi::c_int as BYTE;
    }
    if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_1::what_about_guy_in_back.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        DoResponsePhrase(
            C2Rust_Unnamed_1::what_about_guy_in_back.0 as ::core::ffi::c_int
                as RESPONSE_REF,
            Some(ZoqFotInfo as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        );
        InfoLeft = BOOLEAN::TRUE.0 as ::core::ffi::c_int as BYTE;
    }
    if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_1::what_about_past.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        DoResponsePhrase(
            C2Rust_Unnamed_1::what_about_past.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(ZoqFotInfo as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        );
        InfoLeft = BOOLEAN::TRUE.0 as ::core::ffi::c_int as BYTE;
    }
    DoResponsePhrase(
        C2Rust_Unnamed_1::enough_info.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(ZoqFotHome as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    );
    if InfoLeft == 0 {
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_1::want_specific_info.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    }
}
unsafe extern "C" fn ZoqFotHome(mut R: RESPONSE_REF) {
    let mut NumVisits: BYTE = 0;
    if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::whats_up_homeworld.0 as ::core::ffi::c_int
    {
        NumVisits = getGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed::ZOQFOT_INFO.0 as ::core::ffi::c_int,
            C2Rust_Unnamed::END_ZOQFOT_INFO.0 as ::core::ffi::c_int,
        );
        let c2rust_fresh3 = NumVisits;
        NumVisits = NumVisits.wrapping_add(1);
        match c2rust_fresh3 as ::core::ffi::c_int {
            0 => {
                NPCPhrase_cb(
                    C2Rust_Unnamed_1::GENERAL_INFO_10.0 as ::core::ffi::c_int,
                    Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
                );
                NPCPhrase_cb(
                    C2Rust_Unnamed_1::GENERAL_INFO_11.0 as ::core::ffi::c_int,
                    Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
                );
                NPCPhrase_cb(
                    C2Rust_Unnamed_1::GENERAL_INFO_12.0 as ::core::ffi::c_int,
                    Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
                );
                NPCPhrase_cb(
                    C2Rust_Unnamed_1::GENERAL_INFO_13.0 as ::core::ffi::c_int,
                    Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
                );
                ZFPTalkSegue(!(0 as ::core::ffi::c_int) as COUNT);
            }
            1 => {
                NPCPhrase_cb(
                    C2Rust_Unnamed_1::GENERAL_INFO_20.0 as ::core::ffi::c_int,
                    Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
                );
                NPCPhrase_cb(
                    C2Rust_Unnamed_1::GENERAL_INFO_21.0 as ::core::ffi::c_int,
                    Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
                );
                NPCPhrase_cb(
                    C2Rust_Unnamed_1::GENERAL_INFO_22.0 as ::core::ffi::c_int,
                    Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
                );
                NPCPhrase_cb(
                    C2Rust_Unnamed_1::GENERAL_INFO_23.0 as ::core::ffi::c_int,
                    Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
                );
                NPCPhrase_cb(
                    C2Rust_Unnamed_1::GENERAL_INFO_24.0 as ::core::ffi::c_int,
                    Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
                );
                NPCPhrase_cb(
                    C2Rust_Unnamed_1::GENERAL_INFO_25.0 as ::core::ffi::c_int,
                    Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
                );
                NPCPhrase_cb(
                    C2Rust_Unnamed_1::GENERAL_INFO_26.0 as ::core::ffi::c_int,
                    Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
                );
                NPCPhrase_cb(
                    C2Rust_Unnamed_1::GENERAL_INFO_27.0 as ::core::ffi::c_int,
                    Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
                );
                ZFPTalkSegue(!(0 as ::core::ffi::c_int) as COUNT);
            }
            2 => {
                NPCPhrase_cb(
                    C2Rust_Unnamed_1::GENERAL_INFO_30.0 as ::core::ffi::c_int,
                    Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
                );
                NPCPhrase_cb(
                    C2Rust_Unnamed_1::GENERAL_INFO_31.0 as ::core::ffi::c_int,
                    Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
                );
                NPCPhrase_cb(
                    C2Rust_Unnamed_1::GENERAL_INFO_32.0 as ::core::ffi::c_int,
                    Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
                );
                NPCPhrase_cb(
                    C2Rust_Unnamed_1::GENERAL_INFO_33.0 as ::core::ffi::c_int,
                    Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
                );
                NPCPhrase_cb(
                    C2Rust_Unnamed_1::GENERAL_INFO_34.0 as ::core::ffi::c_int,
                    Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
                );
                NPCPhrase_cb(
                    C2Rust_Unnamed_1::GENERAL_INFO_35.0 as ::core::ffi::c_int,
                    Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
                );
                ZFPTalkSegue(!(0 as ::core::ffi::c_int) as COUNT);
            }
            3 => {
                NPCPhrase_cb(
                    C2Rust_Unnamed_1::GENERAL_INFO_40.0 as ::core::ffi::c_int,
                    Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
                );
                NPCPhrase_cb(
                    C2Rust_Unnamed_1::GENERAL_INFO_41.0 as ::core::ffi::c_int,
                    Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
                );
                NPCPhrase_cb(
                    C2Rust_Unnamed_1::GENERAL_INFO_42.0 as ::core::ffi::c_int,
                    Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
                );
                NPCPhrase_cb(
                    C2Rust_Unnamed_1::GENERAL_INFO_43.0 as ::core::ffi::c_int,
                    Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
                );
                NPCPhrase_cb(
                    C2Rust_Unnamed_1::GENERAL_INFO_44.0 as ::core::ffi::c_int,
                    Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
                );
                NPCPhrase_cb(
                    C2Rust_Unnamed_1::GENERAL_INFO_45.0 as ::core::ffi::c_int,
                    Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
                );
                NPCPhrase_cb(
                    C2Rust_Unnamed_1::GENERAL_INFO_46.0 as ::core::ffi::c_int,
                    Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
                );
                NPCPhrase_cb(
                    C2Rust_Unnamed_1::GENERAL_INFO_47.0 as ::core::ffi::c_int,
                    Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
                );
                NPCPhrase_cb(
                    C2Rust_Unnamed_1::GENERAL_INFO_48.0 as ::core::ffi::c_int,
                    Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
                );
                NPCPhrase_cb(
                    C2Rust_Unnamed_1::GENERAL_INFO_49.0 as ::core::ffi::c_int,
                    Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
                );
                NPCPhrase_cb(
                    C2Rust_Unnamed_1::GENERAL_INFO_410.0 as ::core::ffi::c_int,
                    Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
                );
                NPCPhrase_cb(
                    C2Rust_Unnamed_1::GENERAL_INFO_411.0 as ::core::ffi::c_int,
                    Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
                );
                ZFPTalkSegue(!(0 as ::core::ffi::c_int) as COUNT);
                NumVisits = NumVisits.wrapping_sub(1);
            }
            _ => {}
        }
        setGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed::ZOQFOT_INFO.0 as ::core::ffi::c_int,
            C2Rust_Unnamed::END_ZOQFOT_INFO.0 as ::core::ffi::c_int,
            NumVisits,
        );
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_1::whats_up_homeworld.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::any_war_news.0 as ::core::ffi::c_int
    {
        let mut KnowMask: BYTE = 0;
        NumVisits = getGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed::UTWIG_SUPOX_MISSION.0 as ::core::ffi::c_int,
            C2Rust_Unnamed::END_UTWIG_SUPOX_MISSION.0 as ::core::ffi::c_int,
        );
        KnowMask = getGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed::ZOQFOT_KNOW_MASK.0 as ::core::ffi::c_int,
            C2Rust_Unnamed::END_ZOQFOT_KNOW_MASK.0 as ::core::ffi::c_int,
        );
        if KnowMask as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int == 0
            && getGameState(
                &raw mut GlobData.Game_state.GameState as *mut BYTE,
                C2Rust_Unnamed::KOHR_AH_FRENZY.0 as ::core::ffi::c_int,
                C2Rust_Unnamed::END_KOHR_AH_FRENZY.0 as ::core::ffi::c_int,
            ) as ::core::ffi::c_int != 0
        {
            NPCPhrase_cb(
                C2Rust_Unnamed_1::KOHRAH_FRENZY0.0 as ::core::ffi::c_int,
                Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
            );
            NPCPhrase_cb(
                C2Rust_Unnamed_1::KOHRAH_FRENZY1.0 as ::core::ffi::c_int,
                Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
            );
            NPCPhrase_cb(
                C2Rust_Unnamed_1::KOHRAH_FRENZY2.0 as ::core::ffi::c_int,
                Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
            );
            NPCPhrase_cb(
                C2Rust_Unnamed_1::KOHRAH_FRENZY3.0 as ::core::ffi::c_int,
                Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
            );
            NPCPhrase_cb(
                C2Rust_Unnamed_1::KOHRAH_FRENZY4.0 as ::core::ffi::c_int,
                Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
            );
            NPCPhrase_cb(
                C2Rust_Unnamed_1::KOHRAH_FRENZY5.0 as ::core::ffi::c_int,
                Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
            );
            NPCPhrase_cb(
                C2Rust_Unnamed_1::KOHRAH_FRENZY6.0 as ::core::ffi::c_int,
                Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
            );
            NPCPhrase_cb(
                C2Rust_Unnamed_1::KOHRAH_FRENZY7.0 as ::core::ffi::c_int,
                Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
            );
            NPCPhrase_cb(
                C2Rust_Unnamed_1::KOHRAH_FRENZY8.0 as ::core::ffi::c_int,
                Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
            );
            NPCPhrase_cb(
                C2Rust_Unnamed_1::KOHRAH_FRENZY9.0 as ::core::ffi::c_int,
                Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
            );
            NPCPhrase_cb(
                C2Rust_Unnamed_1::KOHRAH_FRENZY10.0 as ::core::ffi::c_int,
                Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
            );
            NPCPhrase_cb(
                C2Rust_Unnamed_1::KOHRAH_FRENZY11.0 as ::core::ffi::c_int,
                Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
            );
            ZFPTalkSegue(!(0 as ::core::ffi::c_int) as COUNT);
            KnowMask = ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as BYTE;
        } else if KnowMask as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int == 0
            && NumVisits as ::core::ffi::c_int > 0 as ::core::ffi::c_int
            && (NumVisits as ::core::ffi::c_int) < 5 as ::core::ffi::c_int
        {
            NPCPhrase_cb(
                C2Rust_Unnamed_1::UTWIG_DELAY0.0 as ::core::ffi::c_int,
                Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
            );
            NPCPhrase_cb(
                C2Rust_Unnamed_1::UTWIG_DELAY1.0 as ::core::ffi::c_int,
                Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
            );
            NPCPhrase_cb(
                C2Rust_Unnamed_1::UTWIG_DELAY2.0 as ::core::ffi::c_int,
                Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
            );
            NPCPhrase_cb(
                C2Rust_Unnamed_1::UTWIG_DELAY3.0 as ::core::ffi::c_int,
                Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
            );
            NPCPhrase_cb(
                C2Rust_Unnamed_1::UTWIG_DELAY4.0 as ::core::ffi::c_int,
                Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
            );
            NPCPhrase_cb(
                C2Rust_Unnamed_1::UTWIG_DELAY5.0 as ::core::ffi::c_int,
                Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
            );
            NPCPhrase_cb(
                C2Rust_Unnamed_1::UTWIG_DELAY6.0 as ::core::ffi::c_int,
                Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
            );
            NPCPhrase_cb(
                C2Rust_Unnamed_1::UTWIG_DELAY7.0 as ::core::ffi::c_int,
                Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
            );
            NPCPhrase_cb(
                C2Rust_Unnamed_1::UTWIG_DELAY8.0 as ::core::ffi::c_int,
                Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
            );
            NPCPhrase_cb(
                C2Rust_Unnamed_1::UTWIG_DELAY9.0 as ::core::ffi::c_int,
                Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
            );
            NPCPhrase_cb(
                C2Rust_Unnamed_1::UTWIG_DELAY10.0 as ::core::ffi::c_int,
                Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
            );
            NPCPhrase_cb(
                C2Rust_Unnamed_1::UTWIG_DELAY11.0 as ::core::ffi::c_int,
                Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
            );
            NPCPhrase_cb(
                C2Rust_Unnamed_1::UTWIG_DELAY12.0 as ::core::ffi::c_int,
                Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
            );
            NPCPhrase_cb(
                C2Rust_Unnamed_1::UTWIG_DELAY13.0 as ::core::ffi::c_int,
                Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
            );
            ZFPTalkSegue(!(0 as ::core::ffi::c_int) as COUNT);
            KnowMask = (KnowMask as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as BYTE;
        } else {
            let mut i: SIZE = 0;
            i = (2155 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as SIZE;
            if NumVisits != 0 {
                i += 1;
            }
            i = (i as ::core::ffi::c_int
                - GlobData.Game_state.GameClock.year_index as ::core::ffi::c_int)
                as SIZE;
            if i as ::core::ffi::c_int == 1 as ::core::ffi::c_int
                && GlobData.Game_state.GameClock.month_index as ::core::ffi::c_int
                    > 2 as ::core::ffi::c_int
            {
                i = 0 as SIZE;
            }
            if KnowMask as ::core::ffi::c_int
                & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int == 0
                && i as ::core::ffi::c_int <= 0 as ::core::ffi::c_int
            {
                NPCPhrase_cb(
                    C2Rust_Unnamed_1::URQUAN_NEARLY_GONE0.0 as ::core::ffi::c_int,
                    Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
                );
                NPCPhrase_cb(
                    C2Rust_Unnamed_1::URQUAN_NEARLY_GONE1.0 as ::core::ffi::c_int,
                    Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
                );
                NPCPhrase_cb(
                    C2Rust_Unnamed_1::URQUAN_NEARLY_GONE2.0 as ::core::ffi::c_int,
                    Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
                );
                NPCPhrase_cb(
                    C2Rust_Unnamed_1::URQUAN_NEARLY_GONE3.0 as ::core::ffi::c_int,
                    Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
                );
                NPCPhrase_cb(
                    C2Rust_Unnamed_1::URQUAN_NEARLY_GONE4.0 as ::core::ffi::c_int,
                    Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
                );
                NPCPhrase_cb(
                    C2Rust_Unnamed_1::URQUAN_NEARLY_GONE5.0 as ::core::ffi::c_int,
                    Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
                );
                ZFPTalkSegue(!(0 as ::core::ffi::c_int) as COUNT);
                KnowMask = (KnowMask as ::core::ffi::c_int
                    | ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                        | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)) as BYTE;
            } else if KnowMask as ::core::ffi::c_int
                & (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int == 0
                && i as ::core::ffi::c_int == 1 as ::core::ffi::c_int
            {
                NPCPhrase_cb(
                    C2Rust_Unnamed_1::KOHRAH_WINNING0.0 as ::core::ffi::c_int,
                    Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
                );
                NPCPhrase_cb(
                    C2Rust_Unnamed_1::KOHRAH_WINNING1.0 as ::core::ffi::c_int,
                    Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
                );
                NPCPhrase_cb(
                    C2Rust_Unnamed_1::KOHRAH_WINNING2.0 as ::core::ffi::c_int,
                    Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
                );
                NPCPhrase_cb(
                    C2Rust_Unnamed_1::KOHRAH_WINNING3.0 as ::core::ffi::c_int,
                    Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
                );
                NPCPhrase_cb(
                    C2Rust_Unnamed_1::KOHRAH_WINNING4.0 as ::core::ffi::c_int,
                    Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
                );
                NPCPhrase_cb(
                    C2Rust_Unnamed_1::KOHRAH_WINNING5.0 as ::core::ffi::c_int,
                    Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
                );
                NPCPhrase_cb(
                    C2Rust_Unnamed_1::KOHRAH_WINNING6.0 as ::core::ffi::c_int,
                    Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
                );
                NPCPhrase_cb(
                    C2Rust_Unnamed_1::KOHRAH_WINNING7.0 as ::core::ffi::c_int,
                    Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
                );
                NPCPhrase_cb(
                    C2Rust_Unnamed_1::KOHRAH_WINNING8.0 as ::core::ffi::c_int,
                    Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
                );
                NPCPhrase_cb(
                    C2Rust_Unnamed_1::KOHRAH_WINNING9.0 as ::core::ffi::c_int,
                    Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
                );
                ZFPTalkSegue(!(0 as ::core::ffi::c_int) as COUNT);
                KnowMask = (KnowMask as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as BYTE;
            } else {
                NPCPhrase_cb(
                    C2Rust_Unnamed_1::NO_WAR_NEWS0.0 as ::core::ffi::c_int,
                    Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
                );
                NPCPhrase_cb(
                    C2Rust_Unnamed_1::NO_WAR_NEWS1.0 as ::core::ffi::c_int,
                    Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
                );
                ZFPTalkSegue(!(0 as ::core::ffi::c_int) as COUNT);
            }
        }
        setGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed::ZOQFOT_KNOW_MASK.0 as ::core::ffi::c_int,
            C2Rust_Unnamed::END_ZOQFOT_KNOW_MASK.0 as ::core::ffi::c_int,
            KnowMask,
        );
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_1::any_war_news.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::i_want_alliance.0 as ::core::ffi::c_int
    {
        NPCPhrase_cb(
            C2Rust_Unnamed_1::GOOD0.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::GOOD1.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::GOOD2.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::GOOD3.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::GOOD4.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::GOOD5.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::GOOD6.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::GOOD7.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::GOOD8.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::GOOD9.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        ZFPTalkSegue(!(0 as ::core::ffi::c_int) as COUNT);
        SetRaceAllied(
            C2Rust_Unnamed_3::ZOQFOTPIK_SHIP.0 as ::core::ffi::c_int as COUNT,
            BOOLEAN::TRUE,
        );
        AddEvent(
            EVENT_TYPE::RELATIVE_EVENT,
            3 as COUNT,
            0 as COUNT,
            0 as COUNT,
            C2Rust_Unnamed_4::ZOQFOT_DISTRESS_EVENT.0 as ::core::ffi::c_int as BYTE,
        );
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::enough_info.0 as ::core::ffi::c_int
    {
        NPCPhrase_cb(
            C2Rust_Unnamed_1::OK_ENOUGH_INFO.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        ZFPTalkSegue(!(0 as ::core::ffi::c_int) as COUNT);
    }
    if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_1::whats_up_homeworld.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        DoResponsePhrase(
            C2Rust_Unnamed_1::whats_up_homeworld.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(ZoqFotHome as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        );
    }
    if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_1::any_war_news.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        DoResponsePhrase(
            C2Rust_Unnamed_1::any_war_news.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(ZoqFotHome as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        );
    }
    if CheckAlliance(C2Rust_Unnamed_3::ZOQFOTPIK_SHIP.0 as ::core::ffi::c_int as COUNT)
        as ::core::ffi::c_int != C2Rust_Unnamed_2::GOOD_GUY.0 as ::core::ffi::c_int
    {
        DoResponsePhrase(
            C2Rust_Unnamed_1::i_want_alliance.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(ZoqFotHome as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        );
    } else if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_1::want_specific_info.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        DoResponsePhrase(
            C2Rust_Unnamed_1::want_specific_info.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(ZoqFotInfo as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        );
    }
    DoResponsePhrase(
        C2Rust_Unnamed_1::bye_homeworld.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(ExitConversation as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    );
}
unsafe extern "C" fn Intro() {
    let mut NumVisits: BYTE = 0;
    if GlobData.Game_state.CurrentActivity as BYTE as ::core::ffi::c_int
        == C2Rust_Unnamed_0::WON_LAST_BATTLE.0 as ::core::ffi::c_int
    {
        NPCPhrase_cb(
            C2Rust_Unnamed_1::OUT_TAKES0.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::OUT_TAKES1.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::OUT_TAKES2.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::OUT_TAKES3.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::OUT_TAKES4.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::OUT_TAKES5.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::OUT_TAKES6.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::OUT_TAKES7.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::OUT_TAKES8.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::OUT_TAKES9.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::OUT_TAKES10.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::OUT_TAKES11.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::OUT_TAKES12.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::OUT_TAKES13.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        ZFPTalkSegue(!(0 as ::core::ffi::c_int) as COUNT);
        setSegue(Segue::Segue_peace);
        return;
    }
    if getGameState(
        &raw mut GlobData.Game_state.GameState as *mut BYTE,
        C2Rust_Unnamed::ZOQFOT_HOSTILE.0 as ::core::ffi::c_int,
        C2Rust_Unnamed::END_ZOQFOT_HOSTILE.0 as ::core::ffi::c_int,
    ) != 0
    {
        NumVisits = getGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed::ZOQFOT_HOME_VISITS.0 as ::core::ffi::c_int,
            C2Rust_Unnamed::END_ZOQFOT_HOME_VISITS.0 as ::core::ffi::c_int,
        );
        let c2rust_fresh0 = NumVisits;
        NumVisits = NumVisits.wrapping_add(1);
        match c2rust_fresh0 as ::core::ffi::c_int {
            0 => {
                NPCPhrase_cb(
                    C2Rust_Unnamed_1::HOSTILE_HELLO_10.0 as ::core::ffi::c_int,
                    Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
                );
                NPCPhrase_cb(
                    C2Rust_Unnamed_1::HOSTILE_HELLO_11.0 as ::core::ffi::c_int,
                    Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
                );
                ZFPTalkSegue(!(0 as ::core::ffi::c_int) as COUNT);
            }
            1 => {
                NPCPhrase_cb(
                    C2Rust_Unnamed_1::HOSTILE_HELLO_20.0 as ::core::ffi::c_int,
                    Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
                );
                NPCPhrase_cb(
                    C2Rust_Unnamed_1::HOSTILE_HELLO_21.0 as ::core::ffi::c_int,
                    Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
                );
                NPCPhrase_cb(
                    C2Rust_Unnamed_1::HOSTILE_HELLO_22.0 as ::core::ffi::c_int,
                    Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
                );
                NPCPhrase_cb(
                    C2Rust_Unnamed_1::HOSTILE_HELLO_23.0 as ::core::ffi::c_int,
                    Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
                );
                NPCPhrase_cb(
                    C2Rust_Unnamed_1::HOSTILE_HELLO_24.0 as ::core::ffi::c_int,
                    Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
                );
                NPCPhrase_cb(
                    C2Rust_Unnamed_1::HOSTILE_HELLO_25.0 as ::core::ffi::c_int,
                    Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
                );
                ZFPTalkSegue(!(0 as ::core::ffi::c_int) as COUNT);
            }
            2 => {
                NPCPhrase_cb(
                    C2Rust_Unnamed_1::HOSTILE_HELLO_30.0 as ::core::ffi::c_int,
                    Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
                );
                NPCPhrase_cb(
                    C2Rust_Unnamed_1::HOSTILE_HELLO_31.0 as ::core::ffi::c_int,
                    Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
                );
                ZFPTalkSegue(!(0 as ::core::ffi::c_int) as COUNT);
            }
            3 => {
                NPCPhrase_cb(
                    C2Rust_Unnamed_1::HOSTILE_HELLO_40.0 as ::core::ffi::c_int,
                    Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
                );
                NPCPhrase_cb(
                    C2Rust_Unnamed_1::HOSTILE_HELLO_41.0 as ::core::ffi::c_int,
                    Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
                );
                ZFPTalkSegue(!(0 as ::core::ffi::c_int) as COUNT);
                NumVisits = NumVisits.wrapping_sub(1);
            }
            _ => {}
        }
        setGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed::ZOQFOT_HOME_VISITS.0 as ::core::ffi::c_int,
            C2Rust_Unnamed::END_ZOQFOT_HOME_VISITS.0 as ::core::ffi::c_int,
            NumVisits,
        );
        setSegue(Segue::Segue_hostile);
    } else if getGameState(
        &raw mut GlobData.Game_state.GameState as *mut BYTE,
        C2Rust_Unnamed::MET_ZOQFOT.0 as ::core::ffi::c_int,
        C2Rust_Unnamed::END_MET_ZOQFOT.0 as ::core::ffi::c_int,
    ) == 0
    {
        setGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed::MET_ZOQFOT.0 as ::core::ffi::c_int,
            C2Rust_Unnamed::END_MET_ZOQFOT.0 as ::core::ffi::c_int,
            1 as BYTE,
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::WE_ARE0.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::WE_ARE1.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::WE_ARE2.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::WE_ARE3.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::WE_ARE4.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::WE_ARE5.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::WE_ARE6.0 as ::core::ffi::c_int,
            Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_1::WE_ARE7.0 as ::core::ffi::c_int,
            Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
        );
        if getGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed::GLOBAL_FLAGS_AND_DATA.0 as ::core::ffi::c_int,
            C2Rust_Unnamed::END_GLOBAL_FLAGS_AND_DATA.0 as ::core::ffi::c_int,
        ) as ::core::ffi::c_int & (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int
            != 0
        {
            NPCPhrase_cb(
                C2Rust_Unnamed_1::INIT_HOME_HELLO0.0 as ::core::ffi::c_int,
                Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
            );
            NPCPhrase_cb(
                C2Rust_Unnamed_1::INIT_HOME_HELLO1.0 as ::core::ffi::c_int,
                Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
            );
            NPCPhrase_cb(
                C2Rust_Unnamed_1::INIT_HOME_HELLO2.0 as ::core::ffi::c_int,
                Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
            );
            NPCPhrase_cb(
                C2Rust_Unnamed_1::INIT_HOME_HELLO3.0 as ::core::ffi::c_int,
                Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
            );
        } else {
            NPCPhrase_cb(
                C2Rust_Unnamed_1::SCOUT_HELLO0.0 as ::core::ffi::c_int,
                Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
            );
            NPCPhrase_cb(
                C2Rust_Unnamed_1::SCOUT_HELLO1.0 as ::core::ffi::c_int,
                Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
            );
            NPCPhrase_cb(
                C2Rust_Unnamed_1::SCOUT_HELLO2.0 as ::core::ffi::c_int,
                Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
            );
            NPCPhrase_cb(
                C2Rust_Unnamed_1::SCOUT_HELLO3.0 as ::core::ffi::c_int,
                Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
            );
        }
        ZFPTalkSegue(!(0 as ::core::ffi::c_int) as COUNT);
        AquaintZoqFot(0 as RESPONSE_REF);
    } else {
        if getGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed::ZOQFOT_DISTRESS.0 as ::core::ffi::c_int,
            C2Rust_Unnamed::END_ZOQFOT_DISTRESS.0 as ::core::ffi::c_int,
        ) != 0
        {
            NPCPhrase_cb(
                C2Rust_Unnamed_1::THANKS_FOR_RESCUE0.0 as ::core::ffi::c_int,
                Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
            );
            NPCPhrase_cb(
                C2Rust_Unnamed_1::THANKS_FOR_RESCUE1.0 as ::core::ffi::c_int,
                Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
            );
            NPCPhrase_cb(
                C2Rust_Unnamed_1::THANKS_FOR_RESCUE2.0 as ::core::ffi::c_int,
                Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
            );
            NPCPhrase_cb(
                C2Rust_Unnamed_1::THANKS_FOR_RESCUE3.0 as ::core::ffi::c_int,
                Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
            );
            NPCPhrase_cb(
                C2Rust_Unnamed_1::THANKS_FOR_RESCUE4.0 as ::core::ffi::c_int,
                Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
            );
            NPCPhrase_cb(
                C2Rust_Unnamed_1::THANKS_FOR_RESCUE5.0 as ::core::ffi::c_int,
                Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
            );
            NPCPhrase_cb(
                C2Rust_Unnamed_1::THANKS_FOR_RESCUE6.0 as ::core::ffi::c_int,
                Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
            );
            NPCPhrase_cb(
                C2Rust_Unnamed_1::THANKS_FOR_RESCUE7.0 as ::core::ffi::c_int,
                Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
            );
            NPCPhrase_cb(
                C2Rust_Unnamed_1::THANKS_FOR_RESCUE8.0 as ::core::ffi::c_int,
                Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
            );
            NPCPhrase_cb(
                C2Rust_Unnamed_1::THANKS_FOR_RESCUE9.0 as ::core::ffi::c_int,
                Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
            );
            NPCPhrase_cb(
                C2Rust_Unnamed_1::THANKS_FOR_RESCUE10.0 as ::core::ffi::c_int,
                Some(SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> ()),
            );
            NPCPhrase_cb(
                C2Rust_Unnamed_1::THANKS_FOR_RESCUE11.0 as ::core::ffi::c_int,
                Some(SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> ()),
            );
            ZFPTalkSegue(!(0 as ::core::ffi::c_int) as COUNT);
            setGameState(
                &raw mut GlobData.Game_state.GameState as *mut BYTE,
                C2Rust_Unnamed::ZOQFOT_DISTRESS.0 as ::core::ffi::c_int,
                C2Rust_Unnamed::END_ZOQFOT_DISTRESS.0 as ::core::ffi::c_int,
                0 as BYTE,
            );
            AddEscortShips(
                C2Rust_Unnamed_3::ZOQFOTPIK_SHIP.0 as ::core::ffi::c_int as COUNT,
                4 as SIZE,
            );
        } else {
            NumVisits = getGameState(
                &raw mut GlobData.Game_state.GameState as *mut BYTE,
                C2Rust_Unnamed::ZOQFOT_HOME_VISITS.0 as ::core::ffi::c_int,
                C2Rust_Unnamed::END_ZOQFOT_HOME_VISITS.0 as ::core::ffi::c_int,
            );
            if CheckAlliance(
                C2Rust_Unnamed_3::ZOQFOTPIK_SHIP.0 as ::core::ffi::c_int as COUNT,
            ) as ::core::ffi::c_int != C2Rust_Unnamed_2::GOOD_GUY.0 as ::core::ffi::c_int
            {
                let c2rust_fresh1 = NumVisits;
                NumVisits = NumVisits.wrapping_add(1);
                match c2rust_fresh1 as ::core::ffi::c_int {
                    0 => {
                        NPCPhrase_cb(
                            C2Rust_Unnamed_1::NEUTRAL_HOME_HELLO_10.0
                                as ::core::ffi::c_int,
                            Some(
                                SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> (),
                            ),
                        );
                        NPCPhrase_cb(
                            C2Rust_Unnamed_1::NEUTRAL_HOME_HELLO_11.0
                                as ::core::ffi::c_int,
                            Some(
                                SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> (),
                            ),
                        );
                        NPCPhrase_cb(
                            C2Rust_Unnamed_1::NEUTRAL_HOME_HELLO_12.0
                                as ::core::ffi::c_int,
                            Some(
                                SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> (),
                            ),
                        );
                        NPCPhrase_cb(
                            C2Rust_Unnamed_1::NEUTRAL_HOME_HELLO_13.0
                                as ::core::ffi::c_int,
                            Some(
                                SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> (),
                            ),
                        );
                    }
                    1 => {
                        NPCPhrase_cb(
                            C2Rust_Unnamed_1::NEUTRAL_HOME_HELLO_20.0
                                as ::core::ffi::c_int,
                            Some(
                                SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> (),
                            ),
                        );
                        NPCPhrase_cb(
                            C2Rust_Unnamed_1::NEUTRAL_HOME_HELLO_21.0
                                as ::core::ffi::c_int,
                            Some(
                                SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> (),
                            ),
                        );
                        NPCPhrase_cb(
                            C2Rust_Unnamed_1::NEUTRAL_HOME_HELLO_22.0
                                as ::core::ffi::c_int,
                            Some(
                                SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> (),
                            ),
                        );
                        NPCPhrase_cb(
                            C2Rust_Unnamed_1::NEUTRAL_HOME_HELLO_23.0
                                as ::core::ffi::c_int,
                            Some(
                                SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> (),
                            ),
                        );
                        NumVisits = NumVisits.wrapping_sub(1);
                    }
                    _ => {}
                }
                ZFPTalkSegue(!(0 as ::core::ffi::c_int) as COUNT);
            } else {
                let c2rust_fresh2 = NumVisits;
                NumVisits = NumVisits.wrapping_add(1);
                match c2rust_fresh2 as ::core::ffi::c_int {
                    0 => {
                        NPCPhrase_cb(
                            C2Rust_Unnamed_1::ALLIED_HOME_HELLO_10.0
                                as ::core::ffi::c_int,
                            Some(
                                SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> (),
                            ),
                        );
                        NPCPhrase_cb(
                            C2Rust_Unnamed_1::ALLIED_HOME_HELLO_11.0
                                as ::core::ffi::c_int,
                            Some(
                                SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> (),
                            ),
                        );
                        NPCPhrase_cb(
                            C2Rust_Unnamed_1::ALLIED_HOME_HELLO_12.0
                                as ::core::ffi::c_int,
                            Some(
                                SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> (),
                            ),
                        );
                        NPCPhrase_cb(
                            C2Rust_Unnamed_1::ALLIED_HOME_HELLO_13.0
                                as ::core::ffi::c_int,
                            Some(
                                SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> (),
                            ),
                        );
                        ZFPTalkSegue(!(0 as ::core::ffi::c_int) as COUNT);
                    }
                    1 => {
                        NPCPhrase_cb(
                            C2Rust_Unnamed_1::ALLIED_HOME_HELLO_20.0
                                as ::core::ffi::c_int,
                            Some(
                                SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> (),
                            ),
                        );
                        NPCPhrase_cb(
                            C2Rust_Unnamed_1::ALLIED_HOME_HELLO_21.0
                                as ::core::ffi::c_int,
                            Some(
                                SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> (),
                            ),
                        );
                        NPCPhrase_cb(
                            C2Rust_Unnamed_1::ALLIED_HOME_HELLO_22.0
                                as ::core::ffi::c_int,
                            Some(
                                SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> (),
                            ),
                        );
                        NPCPhrase_cb(
                            C2Rust_Unnamed_1::ALLIED_HOME_HELLO_23.0
                                as ::core::ffi::c_int,
                            Some(
                                SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> (),
                            ),
                        );
                        NPCPhrase_cb(
                            C2Rust_Unnamed_1::ALLIED_HOME_HELLO_24.0
                                as ::core::ffi::c_int,
                            Some(
                                SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> (),
                            ),
                        );
                        NPCPhrase_cb(
                            C2Rust_Unnamed_1::ALLIED_HOME_HELLO_25.0
                                as ::core::ffi::c_int,
                            Some(
                                SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> (),
                            ),
                        );
                        NPCPhrase_cb(
                            C2Rust_Unnamed_1::ALLIED_HOME_HELLO_26.0
                                as ::core::ffi::c_int,
                            Some(
                                SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> (),
                            ),
                        );
                        NPCPhrase_cb(
                            C2Rust_Unnamed_1::ALLIED_HOME_HELLO_27.0
                                as ::core::ffi::c_int,
                            Some(
                                SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> (),
                            ),
                        );
                        ZFPTalkSegue(!(0 as ::core::ffi::c_int) as COUNT);
                    }
                    2 => {
                        NPCPhrase_cb(
                            C2Rust_Unnamed_1::ALLIED_HOME_HELLO_30.0
                                as ::core::ffi::c_int,
                            Some(
                                SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> (),
                            ),
                        );
                        NPCPhrase_cb(
                            C2Rust_Unnamed_1::ALLIED_HOME_HELLO_31.0
                                as ::core::ffi::c_int,
                            Some(
                                SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> (),
                            ),
                        );
                        ZFPTalkSegue(!(0 as ::core::ffi::c_int) as COUNT);
                    }
                    3 => {
                        NPCPhrase_cb(
                            C2Rust_Unnamed_1::ALLIED_HOME_HELLO_40.0
                                as ::core::ffi::c_int,
                            Some(
                                SelectAlienZOQ as unsafe extern "C" fn(CallbackArg) -> (),
                            ),
                        );
                        NPCPhrase_cb(
                            C2Rust_Unnamed_1::ALLIED_HOME_HELLO_41.0
                                as ::core::ffi::c_int,
                            Some(
                                SelectAlienPIK as unsafe extern "C" fn(CallbackArg) -> (),
                            ),
                        );
                        ZFPTalkSegue(!(0 as ::core::ffi::c_int) as COUNT);
                        NumVisits = NumVisits.wrapping_sub(1);
                    }
                    _ => {}
                }
            }
            setGameState(
                &raw mut GlobData.Game_state.GameState as *mut BYTE,
                C2Rust_Unnamed::ZOQFOT_HOME_VISITS.0 as ::core::ffi::c_int,
                C2Rust_Unnamed::END_ZOQFOT_HOME_VISITS.0 as ::core::ffi::c_int,
                NumVisits,
            );
        }
        ZoqFotHome(0 as RESPONSE_REF);
    };
}
unsafe extern "C" fn uninit_zoqfot() -> COUNT {
    return 0 as COUNT;
}
unsafe extern "C" fn post_zoqfot_enc() {}
#[no_mangle]
pub unsafe extern "C" fn init_zoqfot_comm() -> *mut LOCDATA {
    let mut retval: *mut LOCDATA = ::core::ptr::null_mut::<LOCDATA>();
    zoqfot_desc.init_encounter_func = Some(Intro as unsafe extern "C" fn() -> ())
        as Option<unsafe extern "C" fn() -> ()>;
    zoqfot_desc.post_encounter_func = Some(
        post_zoqfot_enc as unsafe extern "C" fn() -> (),
    ) as Option<unsafe extern "C" fn() -> ()>;
    zoqfot_desc.uninit_encounter_func = Some(
        uninit_zoqfot as unsafe extern "C" fn() -> COUNT,
    ) as Option<unsafe extern "C" fn() -> COUNT>;
    zoqfot_desc.AlienTextWidth = ((ScreenWidth - 64 as ::core::ffi::c_int
        - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int - 14 as ::core::ffi::c_int
        - ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
        >> 1 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int) as COUNT;
    if CheckAlliance(C2Rust_Unnamed_3::ZOQFOTPIK_SHIP.0 as ::core::ffi::c_int as COUNT)
        as ::core::ffi::c_int == C2Rust_Unnamed_2::GOOD_GUY.0 as ::core::ffi::c_int
        || GlobData.Game_state.CurrentActivity as BYTE as ::core::ffi::c_int
            == C2Rust_Unnamed_0::WON_LAST_BATTLE.0 as ::core::ffi::c_int
    {
        setSegue(Segue::Segue_peace);
    } else {
        setSegue(Segue::Segue_hostile);
    }
    retval = &raw mut zoqfot_desc;
    return retval;
}
