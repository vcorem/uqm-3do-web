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

// Rust translation of src/uqm/comm/comandr/comandr.c.
extern "C" {
    pub type string_table_entry;
    pub type frame_desc;
    pub type font_desc;
    pub type tfb_soundsample;
    fn XFormColorMap(ColorMapPtr: COLORMAPPTR, TimeInterval: SIZE) -> DWORD;
    fn GetColorMapAddress(_: COLORMAP) -> COLORMAPPTR;
    fn SetAbsStringTableIndex(String: STRING, StringTableIndex: COUNT) -> STRING;
    fn GetStringAddress(String: STRING) -> STRINGPTR;
    fn DrawLanders();
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
    fn construct_response(buf: *mut UNICODE, R: ::core::ffi::c_int, ...);
    fn setSegue(segue: Segue);
    fn AlienTalkSegue(wait_track: COUNT);
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
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_0 {
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_1(pub ::core::ffi::c_int);
impl C2Rust_Unnamed_1 {
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
pub struct C2Rust_Unnamed_2(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_2 {
    pub const NULL_PHRASE: Self = Self(0);
    pub const GLAD_WHEN_YOU_COME_BACK: Self = Self(1);
    pub const GIVE_FUEL: Self = Self(2);
    pub const GIVE_FUEL_AGAIN: Self = Self(3);
    pub const ARE_YOU_SUPPLY_SHIP: Self = Self(4);
    pub const DO_YOU_HAVE_RADIO_THIS_TIME: Self = Self(5);
    pub const HERE_IS_ANOTHER_LANDER: Self = Self(6);
    pub const THE_WHAT_FROM_WHERE: Self = Self(7);
    pub const ABOUT_TIME: Self = Self(8);
    pub const MESSAGE_GARBLED_1: Self = Self(9);
    pub const MESSAGE_GARBLED_2: Self = Self(10);
    pub const HERE_IS_A_NEW_LANDER: Self = Self(11);
    pub const THIS_MAY_SEEM_SILLY: Self = Self(12);
    pub const OK_THE_NAFS: Self = Self(13);
    pub const OK_THE_CAN: Self = Self(14);
    pub const OK_THE_UFW: Self = Self(15);
    pub const OK_THE_NAME_IS_EMPIRE0: Self = Self(16);
    pub const OK_THE_NAME_IS_EMPIRE1: Self = Self(17);
    pub const FUEL_UP0: Self = Self(18);
    pub const FUEL_UP1: Self = Self(19);
    pub const WHAT_KIND_OF_IDIOT: Self = Self(20);
    pub const DONT_KNOW_WHO_YOU_ARE: Self = Self(21);
    pub const THATS_IMPOSSIBLE: Self = Self(22);
    pub const ASK_AWAY: Self = Self(23);
    pub const RADIOS_ON_MERCURY: Self = Self(24);
    pub const THANKS_FOR_HELPING: Self = Self(25);
    pub const STARBASE_IS: Self = Self(26);
    pub const HAPPENED_TO_EARTH: Self = Self(27);
    pub const URQUAN_LEFT: Self = Self(28);
    pub const BASE_ON_MOON: Self = Self(29);
    pub const ACKNOWLEDGE_SECRET: Self = Self(30);
    pub const ABOUT_BASE: Self = Self(31);
    pub const GOOD_LUCK_WITH_BASE: Self = Self(32);
    pub const DEALT_WITH_BASE_YET: Self = Self(33);
    pub const HERE_COMES_ILWRATH: Self = Self(34);
    pub const VERY_IMPRESSIVE: Self = Self(35);
    pub const IT_WAS_ABANDONED: Self = Self(36);
    pub const YOU_REALLY_FOUGHT_BASE: Self = Self(37);
    pub const IM_GLAD_YOU_WON: Self = Self(38);
    pub const IM_SURE_IT_WAS_DIFFICULT: Self = Self(39);
    pub const THAT_WAS_PROBE: Self = Self(40);
    pub const DEEP_TROUBLE: Self = Self(41);
    pub const GOOD_NEWS: Self = Self(42);
    pub const SURE_HOPE: Self = Self(43);
    pub const ABOUT_BASE_AGAIN: Self = Self(44);
    pub const COOK_BUTTS: Self = Self(45);
    pub const OVERTHROW_ALIENS: Self = Self(46);
    pub const KILL_MONSTERS: Self = Self(47);
    pub const GOOD_LUCK_AGAIN: Self = Self(48);
    pub const STARBASE_WILL_BE_READY: Self = Self(49);
    pub const overthrow_evil_aliens: Self = Self(50);
    pub const annihilate_those_monsters: Self = Self(51);
    pub const cook_their_butts: Self = Self(52);
    pub const where_get_radios: Self = Self(53);
    pub const well_go_get_them_now: Self = Self(54);
    pub const we_will_transfer_now: Self = Self(55);
    pub const what_will_you_give_us: Self = Self(56);
    pub const before_radios_we_need_info: Self = Self(57);
    pub const no_but_well_help0: Self = Self(58);
    pub const no_but_well_help1: Self = Self(59);
    pub const yes_this_is_supply_ship: Self = Self(60);
    pub const what_slave_planet: Self = Self(61);
    pub const i_lied: Self = Self(62);
    pub const plumb_out: Self = Self(63);
    pub const we_are_vindicator0: Self = Self(64);
    pub const we_are_vindicator1: Self = Self(65);
    pub const we_are_vindicator2: Self = Self(66);
    pub const first_give_info: Self = Self(67);
    pub const we_must_go_now: Self = Self(68);
    pub const where_can_i_get_radios: Self = Self(69);
    pub const ok_i_will_get_radios: Self = Self(70);
    pub const whats_this_starbase: Self = Self(71);
    pub const what_about_earth: Self = Self(72);
    pub const where_are_urquan: Self = Self(73);
    pub const our_mission_was_secret: Self = Self(74);
    pub const we_are_here_to_help: Self = Self(75);
    pub const tell_me_about_base: Self = Self(76);
    pub const we_will_take_care_of_base: Self = Self(77);
    pub const tell_me_again: Self = Self(78);
    pub const base_was_abandoned: Self = Self(79);
    pub const we_fought_them: Self = Self(80);
    pub const oh_yes_big_fight: Self = Self(81);
    pub const i_lied_it_was_abandoned: Self = Self(82);
    pub const i_cant_talk_about_it: Self = Self(83);
    pub const name_1: Self = Self(84);
    pub const name_2: Self = Self(85);
    pub const name_3: Self = Self(86);
    pub const name_40: Self = Self(87);
    pub const name_41: Self = Self(88);
    pub const i_lost_my_lander: Self = Self(89);
    pub const i_lost_another_lander: Self = Self(90);
    pub const need_fuel_mercury: Self = Self(91);
    pub const need_fuel_luna: Self = Self(92);
    pub const need_fuel_again: Self = Self(93);
    pub const what_was_red_thing: Self = Self(94);
    pub const it_went_away: Self = Self(95);
    pub const we_destroyed_it: Self = Self(96);
    pub const what_probe: Self = Self(97);
    pub const take_care_of_base_again: Self = Self(98);
    pub const goodbye_commander: Self = Self(99);
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
    ConversationPhrasesRes: b"comm.commander.dialogue\0".as_ptr() as RESOURCE,
    NumAnimations: 3 as COUNT,
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
            StartIndex: 1 as COUNT,
            NumFrames: 3 as BYTE,
            AnimFlags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as BYTE,
            BaseFrameRate: 0 as COUNT,
            RandomFrameRate: (840 as ::core::ffi::c_int / 30 as ::core::ffi::c_int)
                as COUNT,
            BaseRestartRate: 0 as COUNT,
            RandomRestartRate: (840 as ::core::ffi::c_int / 15 as ::core::ffi::c_int)
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
unsafe extern "C" fn ByeBye(mut R: RESPONSE_REF) {
    if R as ::core::ffi::c_int
        == C2Rust_Unnamed_2::ok_i_will_get_radios.0 as ::core::ffi::c_int
    {
        NPCPhrase_cb(C2Rust_Unnamed_2::THANKS_FOR_HELPING.0 as ::core::ffi::c_int, None);
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_2::well_go_get_them_now.0 as ::core::ffi::c_int
    {
        NPCPhrase_cb(
            C2Rust_Unnamed_2::GLAD_WHEN_YOU_COME_BACK.0 as ::core::ffi::c_int,
            None,
        );
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_2::we_will_take_care_of_base.0 as ::core::ffi::c_int
    {
        NPCPhrase_cb(
            C2Rust_Unnamed_2::GOOD_LUCK_WITH_BASE.0 as ::core::ffi::c_int,
            None,
        );
        setGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_0::WILL_DESTROY_BASE.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_0::END_WILL_DESTROY_BASE.0 as ::core::ffi::c_int,
            1 as BYTE,
        );
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_2::take_care_of_base_again.0 as ::core::ffi::c_int
    {
        NPCPhrase_cb(C2Rust_Unnamed_2::GOOD_LUCK_AGAIN.0 as ::core::ffi::c_int, None);
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_2::base_was_abandoned.0 as ::core::ffi::c_int
        || R as ::core::ffi::c_int
            == C2Rust_Unnamed_2::i_lied_it_was_abandoned.0 as ::core::ffi::c_int
    {
        NPCPhrase_cb(C2Rust_Unnamed_2::IT_WAS_ABANDONED.0 as ::core::ffi::c_int, None);
        NPCPhrase_cb(C2Rust_Unnamed_2::HERE_COMES_ILWRATH.0 as ::core::ffi::c_int, None);
        setGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_0::PROBE_ILWRATH_ENCOUNTER.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_0::END_PROBE_ILWRATH_ENCOUNTER.0 as ::core::ffi::c_int,
            1 as BYTE,
        );
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_2::oh_yes_big_fight.0 as ::core::ffi::c_int
    {
        NPCPhrase_cb(C2Rust_Unnamed_2::IM_GLAD_YOU_WON.0 as ::core::ffi::c_int, None);
        NPCPhrase_cb(C2Rust_Unnamed_2::HERE_COMES_ILWRATH.0 as ::core::ffi::c_int, None);
        setGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_0::PROBE_ILWRATH_ENCOUNTER.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_0::END_PROBE_ILWRATH_ENCOUNTER.0 as ::core::ffi::c_int,
            1 as BYTE,
        );
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_2::i_cant_talk_about_it.0 as ::core::ffi::c_int
    {
        NPCPhrase_cb(
            C2Rust_Unnamed_2::IM_SURE_IT_WAS_DIFFICULT.0 as ::core::ffi::c_int,
            None,
        );
        NPCPhrase_cb(C2Rust_Unnamed_2::HERE_COMES_ILWRATH.0 as ::core::ffi::c_int, None);
        setGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_0::PROBE_ILWRATH_ENCOUNTER.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_0::END_PROBE_ILWRATH_ENCOUNTER.0 as ::core::ffi::c_int,
            1 as BYTE,
        );
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_2::cook_their_butts.0 as ::core::ffi::c_int
        || R as ::core::ffi::c_int
            == C2Rust_Unnamed_2::overthrow_evil_aliens.0 as ::core::ffi::c_int
        || R as ::core::ffi::c_int
            == C2Rust_Unnamed_2::annihilate_those_monsters.0 as ::core::ffi::c_int
    {
        setGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_0::PROBE_ILWRATH_ENCOUNTER.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_0::END_PROBE_ILWRATH_ENCOUNTER.0 as ::core::ffi::c_int,
            0 as BYTE,
        );
        if R as ::core::ffi::c_int
            == C2Rust_Unnamed_2::cook_their_butts.0 as ::core::ffi::c_int
        {
            NPCPhrase_cb(C2Rust_Unnamed_2::COOK_BUTTS.0 as ::core::ffi::c_int, None);
        } else if R as ::core::ffi::c_int
            == C2Rust_Unnamed_2::overthrow_evil_aliens.0 as ::core::ffi::c_int
        {
            NPCPhrase_cb(
                C2Rust_Unnamed_2::OVERTHROW_ALIENS.0 as ::core::ffi::c_int,
                None,
            );
        } else {
            NPCPhrase_cb(C2Rust_Unnamed_2::KILL_MONSTERS.0 as ::core::ffi::c_int, None);
        }
        construct_response(
            &raw mut shared_phrase_buf as *mut UNICODE,
            C2Rust_Unnamed_2::name_40.0 as ::core::ffi::c_int,
            &raw mut GlobData.SIS_state.CommanderName as *mut UNICODE,
            C2Rust_Unnamed_2::name_41.0 as ::core::ffi::c_int,
            ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut UNICODE,
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_2::THIS_MAY_SEEM_SILLY.0 as ::core::ffi::c_int,
            None,
        );
        DoResponsePhrase(
            C2Rust_Unnamed_2::name_1.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(ByeBye as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        );
        DoResponsePhrase(
            C2Rust_Unnamed_2::name_2.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(ByeBye as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        );
        DoResponsePhrase(
            C2Rust_Unnamed_2::name_3.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(ByeBye as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        );
        DoResponsePhrase(
            C2Rust_Unnamed_2::name_40.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(ByeBye as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            &raw mut shared_phrase_buf as *mut UNICODE,
        );
        setGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_0::STARBASE_AVAILABLE.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_0::END_STARBASE_AVAILABLE.0 as ::core::ffi::c_int,
            1 as BYTE,
        );
    } else {
        if R as ::core::ffi::c_int == C2Rust_Unnamed_2::name_1.0 as ::core::ffi::c_int {
            NPCPhrase_cb(C2Rust_Unnamed_2::OK_THE_NAFS.0 as ::core::ffi::c_int, None);
            setGameState(
                &raw mut GlobData.Game_state.GameState as *mut BYTE,
                C2Rust_Unnamed_0::NEW_ALLIANCE_NAME.0 as ::core::ffi::c_int,
                C2Rust_Unnamed_0::END_NEW_ALLIANCE_NAME.0 as ::core::ffi::c_int,
                0 as BYTE,
            );
        } else if R as ::core::ffi::c_int
            == C2Rust_Unnamed_2::name_2.0 as ::core::ffi::c_int
        {
            NPCPhrase_cb(C2Rust_Unnamed_2::OK_THE_CAN.0 as ::core::ffi::c_int, None);
            setGameState(
                &raw mut GlobData.Game_state.GameState as *mut BYTE,
                C2Rust_Unnamed_0::NEW_ALLIANCE_NAME.0 as ::core::ffi::c_int,
                C2Rust_Unnamed_0::END_NEW_ALLIANCE_NAME.0 as ::core::ffi::c_int,
                1 as BYTE,
            );
        } else if R as ::core::ffi::c_int
            == C2Rust_Unnamed_2::name_3.0 as ::core::ffi::c_int
        {
            NPCPhrase_cb(C2Rust_Unnamed_2::OK_THE_UFW.0 as ::core::ffi::c_int, None);
            setGameState(
                &raw mut GlobData.Game_state.GameState as *mut BYTE,
                C2Rust_Unnamed_0::NEW_ALLIANCE_NAME.0 as ::core::ffi::c_int,
                C2Rust_Unnamed_0::END_NEW_ALLIANCE_NAME.0 as ::core::ffi::c_int,
                2 as BYTE,
            );
        } else {
            NPCPhrase_cb(
                C2Rust_Unnamed_2::OK_THE_NAME_IS_EMPIRE0.0 as ::core::ffi::c_int,
                None,
            );
            NPCPhrase_cb(C2Rust_Unnamed_1::GLOBAL_PLAYER_NAME.0, None);
            NPCPhrase_cb(
                C2Rust_Unnamed_2::OK_THE_NAME_IS_EMPIRE1.0 as ::core::ffi::c_int,
                None,
            );
            setGameState(
                &raw mut GlobData.Game_state.GameState as *mut BYTE,
                C2Rust_Unnamed_0::NEW_ALLIANCE_NAME.0 as ::core::ffi::c_int,
                C2Rust_Unnamed_0::END_NEW_ALLIANCE_NAME.0 as ::core::ffi::c_int,
                3 as BYTE,
            );
        }
        NPCPhrase_cb(
            C2Rust_Unnamed_2::STARBASE_WILL_BE_READY.0 as ::core::ffi::c_int,
            None,
        );
    };
}
unsafe extern "C" fn NoRadioactives(mut R: RESPONSE_REF) {
    if R as ::core::ffi::c_int
        == C2Rust_Unnamed_2::yes_this_is_supply_ship.0 as ::core::ffi::c_int
    {
        NPCPhrase_cb(C2Rust_Unnamed_2::ABOUT_TIME.0 as ::core::ffi::c_int, None);
        if GlobData
            .SIS_state
            .ElementAmounts[C2Rust_Unnamed::RADIOACTIVE.0 as ::core::ffi::c_int as usize]
            != 0
        {
            GiveRadios(0 as RESPONSE_REF);
        } else {
            DoResponsePhrase(
                C2Rust_Unnamed_2::i_lied.0 as ::core::ffi::c_int as RESPONSE_REF,
                Some(NoRadioactives as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            );
            DoResponsePhrase(
                C2Rust_Unnamed_2::plumb_out.0 as ::core::ffi::c_int as RESPONSE_REF,
                Some(NoRadioactives as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            );
        }
    } else {
        if R as ::core::ffi::c_int
            == C2Rust_Unnamed_2::where_can_i_get_radios.0 as ::core::ffi::c_int
        {
            NPCPhrase_cb(
                C2Rust_Unnamed_2::RADIOS_ON_MERCURY.0 as ::core::ffi::c_int,
                None,
            );
            *GetStringAddress(
                SetAbsStringTableIndex(
                    CommData.ConversationPhrases,
                    (C2Rust_Unnamed_2::where_can_i_get_radios.0 as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int) as COUNT,
                ),
            ) = '\0' as UNICODE;
        } else if R as ::core::ffi::c_int
            == C2Rust_Unnamed_2::no_but_well_help0.0 as ::core::ffi::c_int
        {
            NPCPhrase_cb(
                C2Rust_Unnamed_2::THE_WHAT_FROM_WHERE.0 as ::core::ffi::c_int,
                None,
            );
        } else if R as ::core::ffi::c_int
            == C2Rust_Unnamed_2::what_slave_planet.0 as ::core::ffi::c_int
            || R as ::core::ffi::c_int
                == C2Rust_Unnamed_2::i_lied.0 as ::core::ffi::c_int
        {
            NPCPhrase_cb(
                C2Rust_Unnamed_2::DONT_KNOW_WHO_YOU_ARE.0 as ::core::ffi::c_int,
                None,
            );
        } else if R as ::core::ffi::c_int
            == C2Rust_Unnamed_2::plumb_out.0 as ::core::ffi::c_int
        {
            NPCPhrase_cb(
                C2Rust_Unnamed_2::WHAT_KIND_OF_IDIOT.0 as ::core::ffi::c_int,
                None,
            );
        } else if R as ::core::ffi::c_int
            == C2Rust_Unnamed_2::i_lost_my_lander.0 as ::core::ffi::c_int
        {
            NPCPhrase_cb(
                C2Rust_Unnamed_2::HERE_IS_A_NEW_LANDER.0 as ::core::ffi::c_int,
                None,
            );
            GlobData.SIS_state.NumLanders = GlobData
                .SIS_state
                .NumLanders
                .wrapping_add(1);
            DrawLanders();
            DeltaSISGauges(4 as SIZE, 0 as SIZE, 0 as ::core::ffi::c_int);
            setGameState(
                &raw mut GlobData.Game_state.GameState as *mut BYTE,
                C2Rust_Unnamed_0::LANDERS_LOST.0 as ::core::ffi::c_int,
                C2Rust_Unnamed_0::END_LANDERS_LOST.0 as ::core::ffi::c_int,
                1 as BYTE,
            );
        } else if R as ::core::ffi::c_int
            == C2Rust_Unnamed_2::i_lost_another_lander.0 as ::core::ffi::c_int
        {
            NPCPhrase_cb(
                C2Rust_Unnamed_2::HERE_IS_ANOTHER_LANDER.0 as ::core::ffi::c_int,
                None,
            );
            GlobData.SIS_state.NumLanders = GlobData
                .SIS_state
                .NumLanders
                .wrapping_add(1);
            DrawLanders();
            DeltaSISGauges(4 as SIZE, 0 as SIZE, 0 as ::core::ffi::c_int);
        } else if R as ::core::ffi::c_int
            == C2Rust_Unnamed_2::need_fuel_mercury.0 as ::core::ffi::c_int
            || R as ::core::ffi::c_int
                == C2Rust_Unnamed_2::need_fuel_luna.0 as ::core::ffi::c_int
        {
            NPCPhrase_cb(C2Rust_Unnamed_2::GIVE_FUEL.0 as ::core::ffi::c_int, None);
            DeltaSISGauges(
                0 as SIZE,
                (5 as ::core::ffi::c_int * 100 as ::core::ffi::c_int) as SIZE,
                0 as ::core::ffi::c_int,
            );
            setGameState(
                &raw mut GlobData.Game_state.GameState as *mut BYTE,
                C2Rust_Unnamed_0::GIVEN_FUEL_BEFORE.0 as ::core::ffi::c_int,
                C2Rust_Unnamed_0::END_GIVEN_FUEL_BEFORE.0 as ::core::ffi::c_int,
                1 as BYTE,
            );
        } else if R as ::core::ffi::c_int
            == C2Rust_Unnamed_2::need_fuel_again.0 as ::core::ffi::c_int
        {
            NPCPhrase_cb(
                C2Rust_Unnamed_2::GIVE_FUEL_AGAIN.0 as ::core::ffi::c_int,
                None,
            );
            DeltaSISGauges(
                0 as SIZE,
                (5 as ::core::ffi::c_int * 100 as ::core::ffi::c_int) as SIZE,
                0 as ::core::ffi::c_int,
            );
        }
        if GlobData
            .SIS_state
            .ElementAmounts[C2Rust_Unnamed::RADIOACTIVE.0 as ::core::ffi::c_int as usize]
            != 0
        {
            GiveRadios(0 as RESPONSE_REF);
        } else {
            if GlobData.SIS_state.NumLanders as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
                && (getGameState(
                    &raw mut GlobData.Game_state.GameState as *mut BYTE,
                    C2Rust_Unnamed_0::CHMMR_BOMB_STATE.0 as ::core::ffi::c_int,
                    C2Rust_Unnamed_0::END_CHMMR_BOMB_STATE.0 as ::core::ffi::c_int,
                ) as ::core::ffi::c_int) < 2 as ::core::ffi::c_int
            {
                if getGameState(
                    &raw mut GlobData.Game_state.GameState as *mut BYTE,
                    C2Rust_Unnamed_0::LANDERS_LOST.0 as ::core::ffi::c_int,
                    C2Rust_Unnamed_0::END_LANDERS_LOST.0 as ::core::ffi::c_int,
                ) != 0
                {
                    DoResponsePhrase(
                        C2Rust_Unnamed_2::i_lost_another_lander.0 as ::core::ffi::c_int
                            as RESPONSE_REF,
                        Some(NoRadioactives as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                        ::core::ptr::null_mut::<UNICODE>(),
                    );
                } else {
                    DoResponsePhrase(
                        C2Rust_Unnamed_2::i_lost_my_lander.0 as ::core::ffi::c_int
                            as RESPONSE_REF,
                        Some(NoRadioactives as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                        ::core::ptr::null_mut::<UNICODE>(),
                    );
                }
            }
            if GlobData.SIS_state.FuelOnBoard
                < (2 as ::core::ffi::c_int * 100 as ::core::ffi::c_int) as DWORD
            {
                if getGameState(
                    &raw mut GlobData.Game_state.GameState as *mut BYTE,
                    C2Rust_Unnamed_0::GIVEN_FUEL_BEFORE.0 as ::core::ffi::c_int,
                    C2Rust_Unnamed_0::END_GIVEN_FUEL_BEFORE.0 as ::core::ffi::c_int,
                ) != 0
                {
                    DoResponsePhrase(
                        C2Rust_Unnamed_2::need_fuel_again.0 as ::core::ffi::c_int
                            as RESPONSE_REF,
                        Some(NoRadioactives as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                        ::core::ptr::null_mut::<UNICODE>(),
                    );
                } else {
                    DoResponsePhrase(
                        C2Rust_Unnamed_2::need_fuel_mercury.0 as ::core::ffi::c_int
                            as RESPONSE_REF,
                        Some(NoRadioactives as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                        ::core::ptr::null_mut::<UNICODE>(),
                    );
                }
            }
            DoResponsePhrase(
                C2Rust_Unnamed_2::ok_i_will_get_radios.0 as ::core::ffi::c_int
                    as RESPONSE_REF,
                Some(ByeBye as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            );
            if *GetStringAddress(
                SetAbsStringTableIndex(
                    CommData.ConversationPhrases,
                    (C2Rust_Unnamed_2::where_can_i_get_radios.0 as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int) as COUNT,
                ),
            ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
            {
                DoResponsePhrase(
                    C2Rust_Unnamed_2::where_can_i_get_radios.0 as ::core::ffi::c_int
                        as RESPONSE_REF,
                    Some(NoRadioactives as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                    ::core::ptr::null_mut::<UNICODE>(),
                );
            }
        }
    };
}
unsafe extern "C" fn AskAfterRadios(mut R: RESPONSE_REF) {
    if R as ::core::ffi::c_int
        == C2Rust_Unnamed_2::i_lost_my_lander.0 as ::core::ffi::c_int
    {
        NPCPhrase_cb(
            C2Rust_Unnamed_2::HERE_IS_A_NEW_LANDER.0 as ::core::ffi::c_int,
            None,
        );
        GlobData.SIS_state.NumLanders = GlobData.SIS_state.NumLanders.wrapping_add(1);
        DrawLanders();
        DeltaSISGauges(4 as SIZE, 0 as SIZE, 0 as ::core::ffi::c_int);
        setGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_0::LANDERS_LOST.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_0::END_LANDERS_LOST.0 as ::core::ffi::c_int,
            1 as BYTE,
        );
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_2::i_lost_another_lander.0 as ::core::ffi::c_int
    {
        NPCPhrase_cb(
            C2Rust_Unnamed_2::HERE_IS_ANOTHER_LANDER.0 as ::core::ffi::c_int,
            None,
        );
        GlobData.SIS_state.NumLanders = GlobData.SIS_state.NumLanders.wrapping_add(1);
        DrawLanders();
        DeltaSISGauges(4 as SIZE, 0 as SIZE, 0 as ::core::ffi::c_int);
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_2::need_fuel_mercury.0 as ::core::ffi::c_int
        || R as ::core::ffi::c_int
            == C2Rust_Unnamed_2::need_fuel_luna.0 as ::core::ffi::c_int
    {
        NPCPhrase_cb(C2Rust_Unnamed_2::GIVE_FUEL.0 as ::core::ffi::c_int, None);
        DeltaSISGauges(
            0 as SIZE,
            (5 as ::core::ffi::c_int * 100 as ::core::ffi::c_int) as SIZE,
            0 as ::core::ffi::c_int,
        );
        setGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_0::GIVEN_FUEL_BEFORE.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_0::END_GIVEN_FUEL_BEFORE.0 as ::core::ffi::c_int,
            1 as BYTE,
        );
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_2::need_fuel_again.0 as ::core::ffi::c_int
    {
        NPCPhrase_cb(C2Rust_Unnamed_2::GIVE_FUEL_AGAIN.0 as ::core::ffi::c_int, None);
        DeltaSISGauges(
            0 as SIZE,
            (5 as ::core::ffi::c_int * 100 as ::core::ffi::c_int) as SIZE,
            0 as ::core::ffi::c_int,
        );
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_2::where_get_radios.0 as ::core::ffi::c_int
    {
        NPCPhrase_cb(C2Rust_Unnamed_2::RADIOS_ON_MERCURY.0 as ::core::ffi::c_int, None);
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_2::where_get_radios.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    }
    if GlobData.SIS_state.NumLanders as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        && (getGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_0::CHMMR_BOMB_STATE.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_0::END_CHMMR_BOMB_STATE.0 as ::core::ffi::c_int,
        ) as ::core::ffi::c_int) < 2 as ::core::ffi::c_int
    {
        if getGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_0::LANDERS_LOST.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_0::END_LANDERS_LOST.0 as ::core::ffi::c_int,
        ) != 0
        {
            DoResponsePhrase(
                C2Rust_Unnamed_2::i_lost_another_lander.0 as ::core::ffi::c_int
                    as RESPONSE_REF,
                Some(AskAfterRadios as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            );
        } else {
            DoResponsePhrase(
                C2Rust_Unnamed_2::i_lost_my_lander.0 as ::core::ffi::c_int
                    as RESPONSE_REF,
                Some(AskAfterRadios as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            );
        }
    }
    if GlobData.SIS_state.FuelOnBoard
        < (2 as ::core::ffi::c_int * 100 as ::core::ffi::c_int) as DWORD
    {
        if getGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_0::GIVEN_FUEL_BEFORE.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_0::END_GIVEN_FUEL_BEFORE.0 as ::core::ffi::c_int,
        ) != 0
        {
            DoResponsePhrase(
                C2Rust_Unnamed_2::need_fuel_again.0 as ::core::ffi::c_int
                    as RESPONSE_REF,
                Some(AskAfterRadios as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            );
        } else {
            DoResponsePhrase(
                C2Rust_Unnamed_2::need_fuel_mercury.0 as ::core::ffi::c_int
                    as RESPONSE_REF,
                Some(AskAfterRadios as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            );
        }
    }
    DoResponsePhrase(
        C2Rust_Unnamed_2::well_go_get_them_now.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(ByeBye as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    );
    if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_2::where_get_radios.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        DoResponsePhrase(
            C2Rust_Unnamed_2::where_get_radios.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(AskAfterRadios as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        );
    }
}
unsafe extern "C" fn BaseDestroyed(mut R: RESPONSE_REF) {
    if R as ::core::ffi::c_int
        == C2Rust_Unnamed_2::we_fought_them.0 as ::core::ffi::c_int
    {
        NPCPhrase_cb(
            C2Rust_Unnamed_2::YOU_REALLY_FOUGHT_BASE.0 as ::core::ffi::c_int,
            None,
        );
        DoResponsePhrase(
            C2Rust_Unnamed_2::oh_yes_big_fight.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(ByeBye as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        );
        DoResponsePhrase(
            C2Rust_Unnamed_2::i_lied_it_was_abandoned.0 as ::core::ffi::c_int
                as RESPONSE_REF,
            Some(ByeBye as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        );
        DoResponsePhrase(
            C2Rust_Unnamed_2::i_cant_talk_about_it.0 as ::core::ffi::c_int
                as RESPONSE_REF,
            Some(ByeBye as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        );
    } else {
        if R as ::core::ffi::c_int
            == C2Rust_Unnamed_2::we_are_here_to_help.0 as ::core::ffi::c_int
        {
            NPCPhrase_cb(C2Rust_Unnamed_2::BASE_ON_MOON.0 as ::core::ffi::c_int, None);
        } else {
            NPCPhrase_cb(
                C2Rust_Unnamed_2::DEALT_WITH_BASE_YET.0 as ::core::ffi::c_int,
                None,
            );
        }
        DoResponsePhrase(
            C2Rust_Unnamed_2::base_was_abandoned.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(ByeBye as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        );
        DoResponsePhrase(
            C2Rust_Unnamed_2::we_fought_them.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(BaseDestroyed as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        );
    };
}
unsafe extern "C" fn TellMoonBase(mut R: RESPONSE_REF) {
    if R as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        NPCPhrase_cb(
            C2Rust_Unnamed_2::DEALT_WITH_BASE_YET.0 as ::core::ffi::c_int,
            None,
        );
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_2::i_lost_my_lander.0 as ::core::ffi::c_int
    {
        NPCPhrase_cb(
            C2Rust_Unnamed_2::HERE_IS_A_NEW_LANDER.0 as ::core::ffi::c_int,
            None,
        );
        GlobData.SIS_state.NumLanders = GlobData.SIS_state.NumLanders.wrapping_add(1);
        DrawLanders();
        DeltaSISGauges(4 as SIZE, 0 as SIZE, 0 as ::core::ffi::c_int);
        setGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_0::LANDERS_LOST.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_0::END_LANDERS_LOST.0 as ::core::ffi::c_int,
            1 as BYTE,
        );
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_2::i_lost_another_lander.0 as ::core::ffi::c_int
    {
        NPCPhrase_cb(
            C2Rust_Unnamed_2::HERE_IS_ANOTHER_LANDER.0 as ::core::ffi::c_int,
            None,
        );
        GlobData.SIS_state.NumLanders = GlobData.SIS_state.NumLanders.wrapping_add(1);
        DrawLanders();
        DeltaSISGauges(4 as SIZE, 0 as SIZE, 0 as ::core::ffi::c_int);
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_2::need_fuel_mercury.0 as ::core::ffi::c_int
        || R as ::core::ffi::c_int
            == C2Rust_Unnamed_2::need_fuel_luna.0 as ::core::ffi::c_int
    {
        NPCPhrase_cb(C2Rust_Unnamed_2::GIVE_FUEL.0 as ::core::ffi::c_int, None);
        DeltaSISGauges(
            0 as SIZE,
            (5 as ::core::ffi::c_int * 100 as ::core::ffi::c_int) as SIZE,
            0 as ::core::ffi::c_int,
        );
        setGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_0::GIVEN_FUEL_BEFORE.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_0::END_GIVEN_FUEL_BEFORE.0 as ::core::ffi::c_int,
            1 as BYTE,
        );
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_2::need_fuel_again.0 as ::core::ffi::c_int
    {
        NPCPhrase_cb(C2Rust_Unnamed_2::GIVE_FUEL_AGAIN.0 as ::core::ffi::c_int, None);
        DeltaSISGauges(
            0 as SIZE,
            (5 as ::core::ffi::c_int * 100 as ::core::ffi::c_int) as SIZE,
            0 as ::core::ffi::c_int,
        );
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_2::we_are_here_to_help.0 as ::core::ffi::c_int
    {
        NPCPhrase_cb(C2Rust_Unnamed_2::BASE_ON_MOON.0 as ::core::ffi::c_int, None);
    } else if getGameState(
        &raw mut GlobData.Game_state.GameState as *mut BYTE,
        C2Rust_Unnamed_0::STARBASE_YACK_STACK1.0 as ::core::ffi::c_int,
        C2Rust_Unnamed_0::END_STARBASE_YACK_STACK1.0 as ::core::ffi::c_int,
    ) as ::core::ffi::c_int == 0 as ::core::ffi::c_int
    {
        NPCPhrase_cb(C2Rust_Unnamed_2::ABOUT_BASE.0 as ::core::ffi::c_int, None);
        setGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_0::STARBASE_YACK_STACK1.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_0::END_STARBASE_YACK_STACK1.0 as ::core::ffi::c_int,
            1 as BYTE,
        );
    } else {
        NPCPhrase_cb(C2Rust_Unnamed_2::ABOUT_BASE_AGAIN.0 as ::core::ffi::c_int, None);
    }
    if GlobData.SIS_state.NumLanders as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        && (getGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_0::CHMMR_BOMB_STATE.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_0::END_CHMMR_BOMB_STATE.0 as ::core::ffi::c_int,
        ) as ::core::ffi::c_int) < 2 as ::core::ffi::c_int
    {
        if getGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_0::LANDERS_LOST.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_0::END_LANDERS_LOST.0 as ::core::ffi::c_int,
        ) != 0
        {
            DoResponsePhrase(
                C2Rust_Unnamed_2::i_lost_another_lander.0 as ::core::ffi::c_int
                    as RESPONSE_REF,
                Some(TellMoonBase as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            );
        } else {
            DoResponsePhrase(
                C2Rust_Unnamed_2::i_lost_my_lander.0 as ::core::ffi::c_int
                    as RESPONSE_REF,
                Some(TellMoonBase as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            );
        }
    }
    if GlobData.SIS_state.FuelOnBoard
        < (2 as ::core::ffi::c_int * 100 as ::core::ffi::c_int) as DWORD
    {
        if getGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_0::GIVEN_FUEL_BEFORE.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_0::END_GIVEN_FUEL_BEFORE.0 as ::core::ffi::c_int,
        ) != 0
        {
            DoResponsePhrase(
                C2Rust_Unnamed_2::need_fuel_again.0 as ::core::ffi::c_int
                    as RESPONSE_REF,
                Some(TellMoonBase as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            );
        } else {
            DoResponsePhrase(
                C2Rust_Unnamed_2::need_fuel_luna.0 as ::core::ffi::c_int as RESPONSE_REF,
                Some(TellMoonBase as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            );
        }
    }
    if getGameState(
        &raw mut GlobData.Game_state.GameState as *mut BYTE,
        C2Rust_Unnamed_0::WILL_DESTROY_BASE.0 as ::core::ffi::c_int,
        C2Rust_Unnamed_0::END_WILL_DESTROY_BASE.0 as ::core::ffi::c_int,
    ) as ::core::ffi::c_int == 0 as ::core::ffi::c_int
    {
        DoResponsePhrase(
            C2Rust_Unnamed_2::we_will_take_care_of_base.0 as ::core::ffi::c_int
                as RESPONSE_REF,
            Some(ByeBye as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        );
    } else {
        DoResponsePhrase(
            C2Rust_Unnamed_2::take_care_of_base_again.0 as ::core::ffi::c_int
                as RESPONSE_REF,
            Some(ByeBye as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        );
    }
    if getGameState(
        &raw mut GlobData.Game_state.GameState as *mut BYTE,
        C2Rust_Unnamed_0::STARBASE_YACK_STACK1.0 as ::core::ffi::c_int,
        C2Rust_Unnamed_0::END_STARBASE_YACK_STACK1.0 as ::core::ffi::c_int,
    ) as ::core::ffi::c_int == 0 as ::core::ffi::c_int
    {
        DoResponsePhrase(
            C2Rust_Unnamed_2::tell_me_about_base.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(TellMoonBase as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        );
    } else {
        DoResponsePhrase(
            C2Rust_Unnamed_2::tell_me_again.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(TellMoonBase as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        );
    };
}
unsafe extern "C" fn TellProbe(mut R: RESPONSE_REF) {
    NPCPhrase_cb(C2Rust_Unnamed_2::THAT_WAS_PROBE.0 as ::core::ffi::c_int, None);
    *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_2::what_was_red_thing.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) = '\0' as UNICODE;
    DoResponsePhrase(
        C2Rust_Unnamed_2::it_went_away.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(RevealSelf as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    );
    DoResponsePhrase(
        C2Rust_Unnamed_2::we_destroyed_it.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(RevealSelf as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    );
    DoResponsePhrase(
        C2Rust_Unnamed_2::what_probe.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(RevealSelf as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    );
}
unsafe extern "C" fn RevealSelf(mut R: RESPONSE_REF) {
    let mut i: BYTE = 0;
    let mut stack: BYTE = 0;
    stack = 0 as BYTE;
    if R as ::core::ffi::c_int
        == C2Rust_Unnamed_2::we_are_vindicator0.0 as ::core::ffi::c_int
    {
        NPCPhrase_cb(C2Rust_Unnamed_2::THATS_IMPOSSIBLE.0 as ::core::ffi::c_int, None);
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_2::we_are_vindicator0.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_2::our_mission_was_secret.0 as ::core::ffi::c_int
    {
        NPCPhrase_cb(C2Rust_Unnamed_2::ACKNOWLEDGE_SECRET.0 as ::core::ffi::c_int, None);
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_2::our_mission_was_secret.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_2::first_give_info.0 as ::core::ffi::c_int
    {
        NPCPhrase_cb(C2Rust_Unnamed_2::ASK_AWAY.0 as ::core::ffi::c_int, None);
        stack = 1 as BYTE;
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_2::first_give_info.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_2::whats_this_starbase.0 as ::core::ffi::c_int
    {
        NPCPhrase_cb(C2Rust_Unnamed_2::STARBASE_IS.0 as ::core::ffi::c_int, None);
        stack = 1 as BYTE;
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_2::whats_this_starbase.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_2::what_about_earth.0 as ::core::ffi::c_int
    {
        NPCPhrase_cb(C2Rust_Unnamed_2::HAPPENED_TO_EARTH.0 as ::core::ffi::c_int, None);
        stack = 1 as BYTE;
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_2::what_about_earth.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_2::where_are_urquan.0 as ::core::ffi::c_int
    {
        NPCPhrase_cb(C2Rust_Unnamed_2::URQUAN_LEFT.0 as ::core::ffi::c_int, None);
        stack = 1 as BYTE;
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_2::where_are_urquan.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_2::it_went_away.0 as ::core::ffi::c_int
    {
        NPCPhrase_cb(C2Rust_Unnamed_2::DEEP_TROUBLE.0 as ::core::ffi::c_int, None);
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_2::we_destroyed_it.0 as ::core::ffi::c_int
    {
        NPCPhrase_cb(C2Rust_Unnamed_2::GOOD_NEWS.0 as ::core::ffi::c_int, None);
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_2::what_probe.0 as ::core::ffi::c_int
    {
        NPCPhrase_cb(C2Rust_Unnamed_2::SURE_HOPE.0 as ::core::ffi::c_int, None);
    }
    i = 0 as BYTE;
    while (i as ::core::ffi::c_int) < 2 as ::core::ffi::c_int {
        if stack as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            if *GetStringAddress(
                SetAbsStringTableIndex(
                    CommData.ConversationPhrases,
                    (C2Rust_Unnamed_2::first_give_info.0 as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int) as COUNT,
                ),
            ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
            {
                DoResponsePhrase(
                    C2Rust_Unnamed_2::first_give_info.0 as ::core::ffi::c_int
                        as RESPONSE_REF,
                    Some(RevealSelf as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                    ::core::ptr::null_mut::<UNICODE>(),
                );
            } else if *GetStringAddress(
                SetAbsStringTableIndex(
                    CommData.ConversationPhrases,
                    (C2Rust_Unnamed_2::whats_this_starbase.0 as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int) as COUNT,
                ),
            ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
            {
                DoResponsePhrase(
                    C2Rust_Unnamed_2::whats_this_starbase.0 as ::core::ffi::c_int
                        as RESPONSE_REF,
                    Some(RevealSelf as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                    ::core::ptr::null_mut::<UNICODE>(),
                );
            } else if *GetStringAddress(
                SetAbsStringTableIndex(
                    CommData.ConversationPhrases,
                    (C2Rust_Unnamed_2::what_about_earth.0 as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int) as COUNT,
                ),
            ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
            {
                DoResponsePhrase(
                    C2Rust_Unnamed_2::what_about_earth.0 as ::core::ffi::c_int
                        as RESPONSE_REF,
                    Some(RevealSelf as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                    ::core::ptr::null_mut::<UNICODE>(),
                );
            } else if *GetStringAddress(
                SetAbsStringTableIndex(
                    CommData.ConversationPhrases,
                    (C2Rust_Unnamed_2::where_are_urquan.0 as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int) as COUNT,
                ),
            ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
            {
                DoResponsePhrase(
                    C2Rust_Unnamed_2::where_are_urquan.0 as ::core::ffi::c_int
                        as RESPONSE_REF,
                    Some(RevealSelf as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                    ::core::ptr::null_mut::<UNICODE>(),
                );
            } else if *GetStringAddress(
                SetAbsStringTableIndex(
                    CommData.ConversationPhrases,
                    (C2Rust_Unnamed_2::what_was_red_thing.0 as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int) as COUNT,
                ),
            ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
            {
                DoResponsePhrase(
                    C2Rust_Unnamed_2::what_was_red_thing.0 as ::core::ffi::c_int
                        as RESPONSE_REF,
                    Some(TellProbe as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                    ::core::ptr::null_mut::<UNICODE>(),
                );
            }
        } else if *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_2::we_are_vindicator0.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        {
            construct_response(
                &raw mut shared_phrase_buf as *mut UNICODE,
                C2Rust_Unnamed_2::we_are_vindicator0.0 as ::core::ffi::c_int,
                &raw mut GlobData.SIS_state.CommanderName as *mut UNICODE,
                C2Rust_Unnamed_2::we_are_vindicator1.0 as ::core::ffi::c_int,
                &raw mut GlobData.SIS_state.ShipName as *mut UNICODE,
                C2Rust_Unnamed_2::we_are_vindicator2.0 as ::core::ffi::c_int,
                ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut UNICODE,
            );
            DoResponsePhrase(
                C2Rust_Unnamed_2::we_are_vindicator0.0 as ::core::ffi::c_int
                    as RESPONSE_REF,
                Some(RevealSelf as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                &raw mut shared_phrase_buf as *mut UNICODE,
            );
        } else if *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_2::our_mission_was_secret.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        {
            DoResponsePhrase(
                C2Rust_Unnamed_2::our_mission_was_secret.0 as ::core::ffi::c_int
                    as RESPONSE_REF,
                Some(RevealSelf as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            );
        } else if getGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_0::MOONBASE_DESTROYED.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_0::END_MOONBASE_DESTROYED.0 as ::core::ffi::c_int,
        ) as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        {
            DoResponsePhrase(
                C2Rust_Unnamed_2::we_are_here_to_help.0 as ::core::ffi::c_int
                    as RESPONSE_REF,
                Some(TellMoonBase as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            );
        } else {
            DoResponsePhrase(
                C2Rust_Unnamed_2::we_are_here_to_help.0 as ::core::ffi::c_int
                    as RESPONSE_REF,
                Some(BaseDestroyed as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            );
        }
        i = i.wrapping_add(1);
        stack = (stack as ::core::ffi::c_int ^ 1 as ::core::ffi::c_int) as BYTE;
    }
}
unsafe extern "C" fn GiveRadios(mut R: RESPONSE_REF) {
    if R as ::core::ffi::c_int
        == C2Rust_Unnamed_2::we_will_transfer_now.0 as ::core::ffi::c_int
    {
        setGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_0::RADIOACTIVES_PROVIDED.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_0::END_RADIOACTIVES_PROVIDED.0 as ::core::ffi::c_int,
            1 as BYTE,
        );
        NPCPhrase_cb(C2Rust_Unnamed_2::FUEL_UP0.0 as ::core::ffi::c_int, None);
        NPCPhrase_cb(C2Rust_Unnamed_2::FUEL_UP1.0 as ::core::ffi::c_int, None);
        AlienTalkSegue(1 as COUNT);
        CommData.AlienAmbientArray[2usize].AnimFlags = (CommData
            .AlienAmbientArray[2usize]
            .AnimFlags as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as BYTE;
        XFormColorMap(
            GetColorMapAddress(
                SetAbsStringTableIndex(CommData.AlienColorMap, 0 as COUNT),
            ),
            (840 as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as SIZE,
        );
        AlienTalkSegue(!(0 as ::core::ffi::c_int) as COUNT);
        RevealSelf(0 as RESPONSE_REF);
    } else {
        if R as ::core::ffi::c_int
            == C2Rust_Unnamed_2::what_will_you_give_us.0 as ::core::ffi::c_int
        {
            NPCPhrase_cb(
                C2Rust_Unnamed_2::MESSAGE_GARBLED_1.0 as ::core::ffi::c_int,
                None,
            );
        } else if R as ::core::ffi::c_int
            == C2Rust_Unnamed_2::before_radios_we_need_info.0 as ::core::ffi::c_int
        {
            NPCPhrase_cb(
                C2Rust_Unnamed_2::MESSAGE_GARBLED_2.0 as ::core::ffi::c_int,
                None,
            );
        }
        DoResponsePhrase(
            C2Rust_Unnamed_2::we_will_transfer_now.0 as ::core::ffi::c_int
                as RESPONSE_REF,
            Some(GiveRadios as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        );
        DoResponsePhrase(
            C2Rust_Unnamed_2::what_will_you_give_us.0 as ::core::ffi::c_int
                as RESPONSE_REF,
            Some(GiveRadios as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        );
        DoResponsePhrase(
            C2Rust_Unnamed_2::before_radios_we_need_info.0 as ::core::ffi::c_int
                as RESPONSE_REF,
            Some(GiveRadios as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        );
    };
}
unsafe extern "C" fn Intro() {
    if getGameState(
        &raw mut GlobData.Game_state.GameState as *mut BYTE,
        C2Rust_Unnamed_0::PROBE_ILWRATH_ENCOUNTER.0 as ::core::ffi::c_int,
        C2Rust_Unnamed_0::END_PROBE_ILWRATH_ENCOUNTER.0 as ::core::ffi::c_int,
    ) != 0
    {
        NPCPhrase_cb(C2Rust_Unnamed_2::VERY_IMPRESSIVE.0 as ::core::ffi::c_int, None);
        DoResponsePhrase(
            C2Rust_Unnamed_2::cook_their_butts.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(ByeBye as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        );
        DoResponsePhrase(
            C2Rust_Unnamed_2::overthrow_evil_aliens.0 as ::core::ffi::c_int
                as RESPONSE_REF,
            Some(ByeBye as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        );
        DoResponsePhrase(
            C2Rust_Unnamed_2::annihilate_those_monsters.0 as ::core::ffi::c_int
                as RESPONSE_REF,
            Some(ByeBye as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        );
    } else if getGameState(
        &raw mut GlobData.Game_state.GameState as *mut BYTE,
        C2Rust_Unnamed_0::STARBASE_VISITED.0 as ::core::ffi::c_int,
        C2Rust_Unnamed_0::END_STARBASE_VISITED.0 as ::core::ffi::c_int,
    ) != 0
    {
        if getGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_0::RADIOACTIVES_PROVIDED.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_0::END_RADIOACTIVES_PROVIDED.0 as ::core::ffi::c_int,
        ) != 0
        {
            if getGameState(
                &raw mut GlobData.Game_state.GameState as *mut BYTE,
                C2Rust_Unnamed_0::MOONBASE_DESTROYED.0 as ::core::ffi::c_int,
                C2Rust_Unnamed_0::END_MOONBASE_DESTROYED.0 as ::core::ffi::c_int,
            ) as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            {
                TellMoonBase(0 as RESPONSE_REF);
            } else {
                BaseDestroyed(0 as RESPONSE_REF);
            }
        } else {
            CommData.AlienColorMap = SetAbsStringTableIndex(
                CommData.AlienColorMap,
                1 as COUNT,
            ) as COLORMAP;
            NPCPhrase_cb(
                C2Rust_Unnamed_2::DO_YOU_HAVE_RADIO_THIS_TIME.0 as ::core::ffi::c_int,
                None,
            );
            if GlobData
                .SIS_state
                .ElementAmounts[C2Rust_Unnamed::RADIOACTIVE.0 as ::core::ffi::c_int
                as usize] != 0
            {
                GiveRadios(0 as RESPONSE_REF);
            } else {
                AskAfterRadios(0 as RESPONSE_REF);
            }
        }
    } else {
        CommData.AlienColorMap = SetAbsStringTableIndex(
            CommData.AlienColorMap,
            1 as COUNT,
        ) as COLORMAP;
        setGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_0::STARBASE_VISITED.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_0::END_STARBASE_VISITED.0 as ::core::ffi::c_int,
            1 as BYTE,
        );
        NPCPhrase_cb(
            C2Rust_Unnamed_2::ARE_YOU_SUPPLY_SHIP.0 as ::core::ffi::c_int,
            None,
        );
        construct_response(
            &raw mut shared_phrase_buf as *mut UNICODE,
            C2Rust_Unnamed_2::no_but_well_help0.0 as ::core::ffi::c_int,
            &raw mut GlobData.SIS_state.ShipName as *mut UNICODE,
            C2Rust_Unnamed_2::no_but_well_help1.0 as ::core::ffi::c_int,
            ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut UNICODE,
        );
        DoResponsePhrase(
            C2Rust_Unnamed_2::no_but_well_help0.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(NoRadioactives as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            &raw mut shared_phrase_buf as *mut UNICODE,
        );
        DoResponsePhrase(
            C2Rust_Unnamed_2::yes_this_is_supply_ship.0 as ::core::ffi::c_int
                as RESPONSE_REF,
            Some(NoRadioactives as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        );
        DoResponsePhrase(
            C2Rust_Unnamed_2::what_slave_planet.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(NoRadioactives as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        );
    };
}
unsafe extern "C" fn uninit_commander() -> COUNT {
    return 0 as COUNT;
}
unsafe extern "C" fn post_commander_enc() {}
#[no_mangle]
pub unsafe extern "C" fn init_commander_comm() -> *mut LOCDATA {
    let mut retval: *mut LOCDATA = ::core::ptr::null_mut::<LOCDATA>();
    commander_desc.init_encounter_func = Some(Intro as unsafe extern "C" fn() -> ())
        as Option<unsafe extern "C" fn() -> ()>;
    commander_desc.post_encounter_func = Some(
        post_commander_enc as unsafe extern "C" fn() -> (),
    ) as Option<unsafe extern "C" fn() -> ()>;
    commander_desc.uninit_encounter_func = Some(
        uninit_commander as unsafe extern "C" fn() -> COUNT,
    ) as Option<unsafe extern "C" fn() -> COUNT>;
    if getGameState(
        &raw mut GlobData.Game_state.GameState as *mut BYTE,
        C2Rust_Unnamed_0::RADIOACTIVES_PROVIDED.0 as ::core::ffi::c_int,
        C2Rust_Unnamed_0::END_RADIOACTIVES_PROVIDED.0 as ::core::ffi::c_int,
    ) != 0
    {
        commander_desc.AlienAmbientArray[2usize].AnimFlags = (commander_desc
            .AlienAmbientArray[2usize]
            .AnimFlags as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as BYTE;
        commander_desc.AlienSongFlags
            &= !(((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as LDAS_FLAGS);
    } else {
        commander_desc.AlienAmbientArray[2usize].AnimFlags = (commander_desc
            .AlienAmbientArray[2usize]
            .AnimFlags as ::core::ffi::c_int
            & !((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int)) as BYTE;
        commander_desc.AlienAltSongRes = b"comm.commander.lowpower.music\0".as_ptr()
            as *const ::core::ffi::c_char as RESOURCE;
        commander_desc.AlienSongFlags
            |= ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as LDAS_FLAGS;
    }
    commander_desc.AlienTextWidth = 143 as COUNT;
    commander_desc.AlienTextBaseline.x = 164 as COORD;
    commander_desc.AlienTextBaseline.y = 20 as COORD;
    setSegue(Segue::Segue_peace);
    retval = &raw mut commander_desc;
    return retval;
}
