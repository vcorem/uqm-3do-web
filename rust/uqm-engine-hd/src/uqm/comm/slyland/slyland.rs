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

// Rust translation of src/uqm/comm/slyland/slyland.c.
extern "C" {
    pub type string_table_entry;
    pub type frame_desc;
    pub type font_desc;
    pub type tfb_soundsample;
    fn abs(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    static mut resolutionFactor: ::core::ffi::c_uint;
    static mut optWindowType: DWORD;
    static mut CanvasWidth: ::core::ffi::c_int;
    fn setGameStateUint(name: *const ::core::ffi::c_char, val: DWORD);
    fn getGameStateUint(name: *const ::core::ffi::c_char) -> DWORD;
    static mut GlobData: GLOBDATA;
    fn DoResponsePhrase(
        R: RESPONSE_REF,
        response_func: RESPONSE_FUNC,
        ContstructStr: *mut UNICODE,
    );
    fn NPCPhrase_cb(index: ::core::ffi::c_int, cb: CallbackFunction);
    fn NPCPhrase_splice(index: ::core::ffi::c_int);
    fn NPCNumber(number: ::core::ffi::c_int, fmt: *const ::core::ffi::c_char);
    fn setSegue(segue: Segue);
    static mut plot_map: [PLOT_LOCATION; 70];
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
pub struct C2Rust_Unnamed(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed {
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
    pub flags: BYTE,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ANIMATION_DESC {
    pub StartIndex: COUNT,
    pub NumFrames: BYTE,
    pub AnimFlags: COUNT,
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct CONVERSATION(pub ::core::ffi::c_uint);
impl CONVERSATION {
    pub const ARILOU_CONVERSATION: Self = Self(0);
    pub const CHMMR_CONVERSATION: Self = Self(1);
    pub const COMMANDER_CONVERSATION: Self = Self(2);
    pub const ORZ_CONVERSATION: Self = Self(3);
    pub const PKUNK_CONVERSATION: Self = Self(4);
    pub const SHOFIXTI_CONVERSATION: Self = Self(5);
    pub const SPATHI_CONVERSATION: Self = Self(6);
    pub const SUPOX_CONVERSATION: Self = Self(7);
    pub const THRADD_CONVERSATION: Self = Self(8);
    pub const UTWIG_CONVERSATION: Self = Self(9);
    pub const VUX_CONVERSATION: Self = Self(10);
    pub const YEHAT_CONVERSATION: Self = Self(11);
    pub const MELNORME_CONVERSATION: Self = Self(12);
    pub const DRUUGE_CONVERSATION: Self = Self(13);
    pub const ILWRATH_CONVERSATION: Self = Self(14);
    pub const MYCON_CONVERSATION: Self = Self(15);
    pub const SLYLANDRO_CONVERSATION: Self = Self(16);
    pub const UMGAH_CONVERSATION: Self = Self(17);
    pub const URQUAN_CONVERSATION: Self = Self(18);
    pub const ZOQFOTPIK_CONVERSATION: Self = Self(19);
    pub const SYREEN_CONVERSATION: Self = Self(20);
    pub const BLACKURQ_CONVERSATION: Self = Self(21);
    pub const TALKING_PET_CONVERSATION: Self = Self(22);
    pub const SLYLANDRO_HOME_CONVERSATION: Self = Self(23);
    pub const URQUAN_DRONE_CONVERSATION: Self = Self(24);
    pub const YEHAT_REBEL_CONVERSATION: Self = Self(25);
    pub const INVALID_CONVERSATION: Self = Self(26);
    pub const NUM_CONVERSATIONS: Self = Self(27);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ALT_RESOURCE {
    pub AlienFrameRes: RESOURCE,
    pub AlienColorMapRes: RESOURCE,
    pub AlienSongRes: RESOURCE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LOCDATA {
    pub AlienConv: CONVERSATION,
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
    pub AltRes: ALT_RESOURCE,
    pub ConversationPhrasesRes: RESOURCE,
    pub NumAnimations: COUNT,
    pub AlienAmbientArray: [ANIMATION_DESC; 30],
    pub AlienTransitionDesc: ANIMATION_DESC,
    pub AlienTalkDesc: ANIMATION_DESC,
    pub AlienNumberSpeech: NUMBER_SPEECH,
    pub AlienFrame: FRAME,
    pub AlienFont: FONT,
    pub AlienColorMap: COLORMAP,
    pub AlienSong: MUSIC_REF,
    pub ConversationPhrases: STRING,
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
pub type PLOT_LOCATION = plot_location;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct plot_location {
    pub star_pt: POINT,
    pub star: *mut STAR_DESC,
    pub dist_sq: [DWORD; 70],
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_0 {
    pub const NULL_PHRASE: Self = Self(0);
    pub const WE_COME_IN_PEACE_1: Self = Self(1);
    pub const WE_COME_IN_PEACE_2: Self = Self(2);
    pub const WE_COME_IN_PEACE_3: Self = Self(3);
    pub const WE_COME_IN_PEACE_4: Self = Self(4);
    pub const WE_COME_IN_PEACE_5: Self = Self(5);
    pub const WE_COME_IN_PEACE_6: Self = Self(6);
    pub const WE_COME_IN_PEACE_7: Self = Self(7);
    pub const WE_COME_IN_PEACE_8: Self = Self(8);
    pub const threat_1: Self = Self(9);
    pub const threat_2: Self = Self(10);
    pub const threat_3: Self = Self(11);
    pub const threat_4: Self = Self(12);
    pub const PROGRAMMED_TO_DEFEND_1: Self = Self(13);
    pub const PROGRAMMED_TO_DEFEND_2: Self = Self(14);
    pub const PROGRAMMED_TO_DEFEND_3: Self = Self(15);
    pub const PROGRAMMED_TO_DEFEND_4: Self = Self(16);
    pub const something_wrong_1: Self = Self(17);
    pub const something_wrong_2: Self = Self(18);
    pub const something_wrong_3: Self = Self(19);
    pub const something_wrong_4: Self = Self(20);
    pub const NOMINAL_FUNCTION_1: Self = Self(21);
    pub const NOMINAL_FUNCTION_2: Self = Self(22);
    pub const NOMINAL_FUNCTION_3: Self = Self(23);
    pub const NOMINAL_FUNCTION_4: Self = Self(24);
    pub const we_are_us_1: Self = Self(25);
    pub const we_are_us_2: Self = Self(26);
    pub const we_are_us_3: Self = Self(27);
    pub const we_are_us_4: Self = Self(28);
    pub const THIS_IS_PROBE_1: Self = Self(29);
    pub const THIS_IS_PROBE_2: Self = Self(30);
    pub const THIS_IS_PROBE_3: Self = Self(31);
    pub const THIS_IS_PROBE_40: Self = Self(32);
    pub const THIS_IS_PROBE_41: Self = Self(33);
    pub const THIS_IS_PROBE_42: Self = Self(34);
    pub const why_attack_1: Self = Self(35);
    pub const why_attack_2: Self = Self(36);
    pub const why_attack_3: Self = Self(37);
    pub const why_attack_4: Self = Self(38);
    pub const PEACEFUL_MISSION_1: Self = Self(39);
    pub const PEACEFUL_MISSION_2: Self = Self(40);
    pub const PEACEFUL_MISSION_3: Self = Self(41);
    pub const PEACEFUL_MISSION_4: Self = Self(42);
    pub const bye_1: Self = Self(43);
    pub const bye_2: Self = Self(44);
    pub const bye_3: Self = Self(45);
    pub const bye_4: Self = Self(46);
    pub const GOODBYE_1: Self = Self(47);
    pub const GOODBYE_2: Self = Self(48);
    pub const GOODBYE_3: Self = Self(49);
    pub const GOODBYE_4: Self = Self(50);
    pub const HOSTILE: Self = Self(51);
    pub const DESTRUCT_SEQUENCE: Self = Self(52);
    pub const destruct_code: Self = Self(53);
    pub const COORD_PLUS: Self = Self(54);
    pub const COORD_MINUS: Self = Self(55);
    pub const COORD_POINT: Self = Self(56);
    pub const ENUMERATE_HUNDRED: Self = Self(57);
    pub const ENUMERATE_THOUSAND: Self = Self(58);
    pub const ENUMERATE_ZERO: Self = Self(59);
    pub const ENUMERATE_ONE: Self = Self(60);
    pub const ENUMERATE_TWO: Self = Self(61);
    pub const ENUMERATE_THREE: Self = Self(62);
    pub const ENUMERATE_FOUR: Self = Self(63);
    pub const ENUMERATE_FIVE: Self = Self(64);
    pub const ENUMERATE_SIX: Self = Self(65);
    pub const ENUMERATE_SEVEN: Self = Self(66);
    pub const ENUMERATE_EIGHT: Self = Self(67);
    pub const ENUMERATE_NINE: Self = Self(68);
    pub const ENUMERATE_TEN: Self = Self(69);
    pub const ENUMERATE_ELEVEN: Self = Self(70);
    pub const ENUMERATE_TWELVE: Self = Self(71);
    pub const ENUMERATE_THIRTEEN: Self = Self(72);
    pub const ENUMERATE_FOURTEEN: Self = Self(73);
    pub const ENUMERATE_FIFTEEN: Self = Self(74);
    pub const ENUMERATE_SIXTEEN: Self = Self(75);
    pub const ENUMERATE_SEVENTEEN: Self = Self(76);
    pub const ENUMERATE_EIGHTEEN: Self = Self(77);
    pub const ENUMERATE_NINETEEN: Self = Self(78);
    pub const ENUMERATE_TWENTY: Self = Self(79);
    pub const ENUMERATE_THIRTY: Self = Self(80);
    pub const ENUMERATE_FOURTY: Self = Self(81);
    pub const ENUMERATE_FIFTY: Self = Self(82);
    pub const ENUMERATE_SIXTY: Self = Self(83);
    pub const ENUMERATE_SEVENTY: Self = Self(84);
    pub const ENUMERATE_EIGHTY: Self = Self(85);
    pub const ENUMERATE_NINETY: Self = Self(86);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_1(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_1 {
    pub const ARILOU_DEFINED: Self = Self(0);
    pub const SOL_DEFINED: Self = Self(1);
    pub const SHOFIXTI_DEFINED: Self = Self(2);
    pub const MAIDENS_DEFINED: Self = Self(3);
    pub const START_COLONY_DEFINED: Self = Self(4);
    pub const SPATHI_DEFINED: Self = Self(5);
    pub const ZOQFOT_DEFINED: Self = Self(6);
    pub const MELNORME0_DEFINED: Self = Self(7);
    pub const MELNORME1_DEFINED: Self = Self(8);
    pub const MELNORME2_DEFINED: Self = Self(9);
    pub const MELNORME3_DEFINED: Self = Self(10);
    pub const MELNORME4_DEFINED: Self = Self(11);
    pub const MELNORME5_DEFINED: Self = Self(12);
    pub const MELNORME6_DEFINED: Self = Self(13);
    pub const MELNORME7_DEFINED: Self = Self(14);
    pub const MELNORME8_DEFINED: Self = Self(15);
    pub const TALKING_PET_DEFINED: Self = Self(16);
    pub const CHMMR_DEFINED: Self = Self(17);
    pub const SYREEN_DEFINED: Self = Self(18);
    pub const BURVIXESE_DEFINED: Self = Self(19);
    pub const SLYLANDRO_DEFINED: Self = Self(20);
    pub const DRUUGE_DEFINED: Self = Self(21);
    pub const BOMB_DEFINED: Self = Self(22);
    pub const AQUA_HELIX_DEFINED: Self = Self(23);
    pub const SUN_DEVICE_DEFINED: Self = Self(24);
    pub const TAALO_PROTECTOR_DEFINED: Self = Self(25);
    pub const SHIP_VAULT_DEFINED: Self = Self(26);
    pub const URQUAN_WRECK_DEFINED: Self = Self(27);
    pub const VUX_BEAST_DEFINED: Self = Self(28);
    pub const SAMATRA_DEFINED: Self = Self(29);
    pub const ZOQ_SCOUT_DEFINED: Self = Self(30);
    pub const MYCON_DEFINED: Self = Self(31);
    pub const EGG_CASE0_DEFINED: Self = Self(32);
    pub const EGG_CASE1_DEFINED: Self = Self(33);
    pub const EGG_CASE2_DEFINED: Self = Self(34);
    pub const PKUNK_DEFINED: Self = Self(35);
    pub const UTWIG_DEFINED: Self = Self(36);
    pub const SUPOX_DEFINED: Self = Self(37);
    pub const YEHAT_DEFINED: Self = Self(38);
    pub const VUX_DEFINED: Self = Self(39);
    pub const ORZ_DEFINED: Self = Self(40);
    pub const THRADD_DEFINED: Self = Self(41);
    pub const RAINBOW0_DEFINED: Self = Self(42);
    pub const RAINBOW1_DEFINED: Self = Self(43);
    pub const RAINBOW2_DEFINED: Self = Self(44);
    pub const RAINBOW3_DEFINED: Self = Self(45);
    pub const RAINBOW4_DEFINED: Self = Self(46);
    pub const RAINBOW5_DEFINED: Self = Self(47);
    pub const RAINBOW6_DEFINED: Self = Self(48);
    pub const RAINBOW7_DEFINED: Self = Self(49);
    pub const RAINBOW8_DEFINED: Self = Self(50);
    pub const RAINBOW9_DEFINED: Self = Self(51);
    pub const ILWRATH_DEFINED: Self = Self(52);
    pub const ANDROSYNTH_DEFINED: Self = Self(53);
    pub const MYCON_TRAP_DEFINED: Self = Self(54);
    pub const URQUAN0_DEFINED: Self = Self(55);
    pub const URQUAN1_DEFINED: Self = Self(56);
    pub const URQUAN2_DEFINED: Self = Self(57);
    pub const KOHRAH0_DEFINED: Self = Self(58);
    pub const KOHRAH1_DEFINED: Self = Self(59);
    pub const KOHRAH2_DEFINED: Self = Self(60);
    pub const DESTROYED_STARBASE_DEFINED: Self = Self(61);
    pub const MOTHER_ARK_DEFINED: Self = Self(62);
    pub const ZOQ_COLONY0_DEFINED: Self = Self(63);
    pub const ZOQ_COLONY1_DEFINED: Self = Self(64);
    pub const ZOQ_COLONY2_DEFINED: Self = Self(65);
    pub const ZOQ_COLONY3_DEFINED: Self = Self(66);
    pub const ALGOLITES_DEFINED: Self = Self(67);
    pub const SPATHI_MONUMENT_DEFINED: Self = Self(68);
    pub const EXCAVATION_SITE_DEFINED: Self = Self(69);
    pub const NUM_PLOTS: Self = Self(70);
    pub const WAR_ERA: Self = Self(70);
    pub const SUCCESS: Self = Self(70);
    pub const HOME: Self = Self(71);
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
static mut slylandro_desc: LOCDATA = unsafe {
    LOCDATA {
        AlienConv: CONVERSATION::SLYLANDRO_CONVERSATION,
        init_encounter_func: None,
        post_encounter_func: None,
        uninit_encounter_func: None,
        AlienFrameRes: b"comm.probe.graphics\0".as_ptr() as RESOURCE,
        AlienFontRes: b"comm.probe.font\0".as_ptr() as RESOURCE,
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
        AlienColorMapRes: b"comm.probe.colortable\0".as_ptr() as RESOURCE,
        AlienSongRes: b"comm.probe.music\0".as_ptr() as RESOURCE,
        AltRes: ALT_RESOURCE {
            AlienFrameRes: ::core::ptr::null::<::core::ffi::c_char>(),
            AlienColorMapRes: ::core::ptr::null::<::core::ffi::c_char>(),
            AlienSongRes: ::core::ptr::null::<::core::ffi::c_char>(),
        },
        ConversationPhrasesRes: b"comm.probe.dialogue\0".as_ptr() as RESOURCE,
        NumAnimations: 6 as COUNT,
        AlienAmbientArray: [
            ANIMATION_DESC {
                StartIndex: 1 as COUNT,
                NumFrames: 9 as BYTE,
                AnimFlags: ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
                    as COUNT,
                BaseFrameRate: (840 as ::core::ffi::c_int / 40 as ::core::ffi::c_int)
                    as COUNT,
                RandomFrameRate: 0 as COUNT,
                BaseRestartRate: 0 as COUNT,
                RandomRestartRate: (840 as ::core::ffi::c_int * 3 as ::core::ffi::c_int)
                    as COUNT,
                BlockMask: ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                    as DWORD,
            },
            ANIMATION_DESC {
                StartIndex: 10 as COUNT,
                NumFrames: 8 as BYTE,
                AnimFlags: ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
                    as COUNT,
                BaseFrameRate: (840 as ::core::ffi::c_int / 40 as ::core::ffi::c_int)
                    as COUNT,
                RandomFrameRate: 0 as COUNT,
                BaseRestartRate: 0 as COUNT,
                RandomRestartRate: (840 as ::core::ffi::c_int * 3 as ::core::ffi::c_int)
                    as COUNT,
                BlockMask: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                    as DWORD,
            },
            ANIMATION_DESC {
                StartIndex: 18 as COUNT,
                NumFrames: 8 as BYTE,
                AnimFlags: ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
                    as COUNT,
                BaseFrameRate: (840 as ::core::ffi::c_int / 40 as ::core::ffi::c_int)
                    as COUNT,
                RandomFrameRate: 0 as COUNT,
                BaseRestartRate: 0 as COUNT,
                RandomRestartRate: (840 as ::core::ffi::c_int * 3 as ::core::ffi::c_int)
                    as COUNT,
                BlockMask: ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int)
                    as DWORD,
            },
            ANIMATION_DESC {
                StartIndex: 26 as COUNT,
                NumFrames: 8 as BYTE,
                AnimFlags: ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
                    as COUNT,
                BaseFrameRate: (840 as ::core::ffi::c_int / 40 as ::core::ffi::c_int)
                    as COUNT,
                RandomFrameRate: 0 as COUNT,
                BaseRestartRate: 0 as COUNT,
                RandomRestartRate: (840 as ::core::ffi::c_int * 3 as ::core::ffi::c_int)
                    as COUNT,
                BlockMask: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int)
                    as DWORD,
            },
            ANIMATION_DESC {
                StartIndex: 34 as COUNT,
                NumFrames: 8 as BYTE,
                AnimFlags: ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
                    as COUNT,
                BaseFrameRate: (840 as ::core::ffi::c_int / 40 as ::core::ffi::c_int)
                    as COUNT,
                RandomFrameRate: 0 as COUNT,
                BaseRestartRate: 0 as COUNT,
                RandomRestartRate: (840 as ::core::ffi::c_int * 3 as ::core::ffi::c_int)
                    as COUNT,
                BlockMask: ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
                    as DWORD,
            },
            ANIMATION_DESC {
                StartIndex: 42 as COUNT,
                NumFrames: 9 as BYTE,
                AnimFlags: ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
                    as COUNT,
                BaseFrameRate: (840 as ::core::ffi::c_int / 40 as ::core::ffi::c_int)
                    as COUNT,
                RandomFrameRate: 0 as COUNT,
                BaseRestartRate: 0 as COUNT,
                RandomRestartRate: (840 as ::core::ffi::c_int * 3 as ::core::ffi::c_int)
                    as COUNT,
                BlockMask: ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                    as DWORD,
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
            AnimFlags: 0 as COUNT,
            BaseFrameRate: 0 as COUNT,
            RandomFrameRate: 0 as COUNT,
            BaseRestartRate: 0 as COUNT,
            RandomRestartRate: 0 as COUNT,
            BlockMask: 0 as DWORD,
        },
        AlienTalkDesc: ANIMATION_DESC {
            StartIndex: 0 as COUNT,
            NumFrames: 0 as BYTE,
            AnimFlags: 0 as COUNT,
            BaseFrameRate: 0 as COUNT,
            RandomFrameRate: 0 as COUNT,
            BaseRestartRate: 0 as COUNT,
            RandomRestartRate: 0 as COUNT,
            BlockMask: 0 as DWORD,
        },
        AlienNumberSpeech: &raw const probe_numbers_english,
        AlienFrame: ::core::ptr::null_mut::<FRAME_DESC>(),
        AlienFont: ::core::ptr::null_mut::<FONT_DESC>(),
        AlienColorMap: ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>(),
        AlienSong: ::core::ptr::null_mut::<*mut TFB_SoundSample>(),
        ConversationPhrases: ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>(),
    }
};
static mut probe_digit_names: [COUNT; 10] = [
    C2Rust_Unnamed_0::ENUMERATE_ZERO.0 as ::core::ffi::c_int as COUNT,
    C2Rust_Unnamed_0::ENUMERATE_ONE.0 as ::core::ffi::c_int as COUNT,
    C2Rust_Unnamed_0::ENUMERATE_TWO.0 as ::core::ffi::c_int as COUNT,
    C2Rust_Unnamed_0::ENUMERATE_THREE.0 as ::core::ffi::c_int as COUNT,
    C2Rust_Unnamed_0::ENUMERATE_FOUR.0 as ::core::ffi::c_int as COUNT,
    C2Rust_Unnamed_0::ENUMERATE_FIVE.0 as ::core::ffi::c_int as COUNT,
    C2Rust_Unnamed_0::ENUMERATE_SIX.0 as ::core::ffi::c_int as COUNT,
    C2Rust_Unnamed_0::ENUMERATE_SEVEN.0 as ::core::ffi::c_int as COUNT,
    C2Rust_Unnamed_0::ENUMERATE_EIGHT.0 as ::core::ffi::c_int as COUNT,
    C2Rust_Unnamed_0::ENUMERATE_NINE.0 as ::core::ffi::c_int as COUNT,
];
static mut probe_teen_names: [COUNT; 10] = [
    C2Rust_Unnamed_0::ENUMERATE_TEN.0 as ::core::ffi::c_int as COUNT,
    C2Rust_Unnamed_0::ENUMERATE_ELEVEN.0 as ::core::ffi::c_int as COUNT,
    C2Rust_Unnamed_0::ENUMERATE_TWELVE.0 as ::core::ffi::c_int as COUNT,
    C2Rust_Unnamed_0::ENUMERATE_THIRTEEN.0 as ::core::ffi::c_int as COUNT,
    C2Rust_Unnamed_0::ENUMERATE_FOURTEEN.0 as ::core::ffi::c_int as COUNT,
    C2Rust_Unnamed_0::ENUMERATE_FIFTEEN.0 as ::core::ffi::c_int as COUNT,
    C2Rust_Unnamed_0::ENUMERATE_SIXTEEN.0 as ::core::ffi::c_int as COUNT,
    C2Rust_Unnamed_0::ENUMERATE_SEVENTEEN.0 as ::core::ffi::c_int as COUNT,
    C2Rust_Unnamed_0::ENUMERATE_EIGHTEEN.0 as ::core::ffi::c_int as COUNT,
    C2Rust_Unnamed_0::ENUMERATE_NINETEEN.0 as ::core::ffi::c_int as COUNT,
];
static mut probe_tens_names: [COUNT; 10] = [
    0 as COUNT,
    0 as COUNT,
    C2Rust_Unnamed_0::ENUMERATE_TWENTY.0 as ::core::ffi::c_int as COUNT,
    C2Rust_Unnamed_0::ENUMERATE_THIRTY.0 as ::core::ffi::c_int as COUNT,
    C2Rust_Unnamed_0::ENUMERATE_FOURTY.0 as ::core::ffi::c_int as COUNT,
    C2Rust_Unnamed_0::ENUMERATE_FIFTY.0 as ::core::ffi::c_int as COUNT,
    C2Rust_Unnamed_0::ENUMERATE_SIXTY.0 as ::core::ffi::c_int as COUNT,
    C2Rust_Unnamed_0::ENUMERATE_SEVENTY.0 as ::core::ffi::c_int as COUNT,
    C2Rust_Unnamed_0::ENUMERATE_EIGHTY.0 as ::core::ffi::c_int as COUNT,
    C2Rust_Unnamed_0::ENUMERATE_NINETY.0 as ::core::ffi::c_int as COUNT,
];
static mut random_ramble: [DWORD; 17] = [
    1 as DWORD,
    15 as DWORD,
    48 as DWORD,
    312 as DWORD,
    7 as DWORD,
    4000 as DWORD,
    668 as DWORD,
    46 as DWORD,
    4 as DWORD,
    8 as DWORD,
    15 as DWORD,
    16 as DWORD,
    23 as DWORD,
    42 as DWORD,
    998 as DWORD,
    80556 as ::core::ffi::c_int as DWORD,
    1337 as DWORD,
];
static mut probe_numbers_english: NUMBER_SPEECH_DESC = unsafe {
    NUMBER_SPEECH_DESC {
        NumDigits: 5 as COUNT,
        Digits: [
            SPEECH_DIGIT {
                Divider: 1000 as ::core::ffi::c_int,
                Subtrahend: 0 as ::core::ffi::c_int,
                StrDigits: ::core::ptr::null_mut::<COUNT>(),
                Names: ::core::ptr::null_mut::<SPEECH_DIGITNAME>(),
                CommonNameIndex: C2Rust_Unnamed_0::ENUMERATE_THOUSAND.0
                    as ::core::ffi::c_int as COUNT,
            },
            SPEECH_DIGIT {
                Divider: 100 as ::core::ffi::c_int,
                Subtrahend: 0 as ::core::ffi::c_int,
                StrDigits: &raw const probe_digit_names as *mut COUNT,
                Names: ::core::ptr::null_mut::<SPEECH_DIGITNAME>(),
                CommonNameIndex: C2Rust_Unnamed_0::ENUMERATE_HUNDRED.0
                    as ::core::ffi::c_int as COUNT,
            },
            SPEECH_DIGIT {
                Divider: 10 as ::core::ffi::c_int,
                Subtrahend: 0 as ::core::ffi::c_int,
                StrDigits: &raw const probe_tens_names as *mut COUNT,
                Names: ::core::ptr::null_mut::<SPEECH_DIGITNAME>(),
                CommonNameIndex: 0 as COUNT,
            },
            SPEECH_DIGIT {
                Divider: 1 as ::core::ffi::c_int,
                Subtrahend: 10 as ::core::ffi::c_int,
                StrDigits: &raw const probe_teen_names as *mut COUNT,
                Names: ::core::ptr::null_mut::<SPEECH_DIGITNAME>(),
                CommonNameIndex: 0 as COUNT,
            },
            SPEECH_DIGIT {
                Divider: 1 as ::core::ffi::c_int,
                Subtrahend: 0 as ::core::ffi::c_int,
                StrDigits: &raw const probe_digit_names as *mut COUNT,
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
static mut bye: RESPONSE_REF = 0;
static mut why_attack: RESPONSE_REF = 0;
static mut we_are_us: RESPONSE_REF = 0;
static mut something_wrong: RESPONSE_REF = 0;
static mut threat: RESPONSE_REF = 0;
unsafe extern "C" fn sayCoord(mut coord: ::core::ffi::c_int) {
    let mut ac: ::core::ffi::c_int = 0;
    ac = abs(coord);
    uqm_lua_checked!(NPCPhrase_splice(
        if coord < 0 as ::core::ffi::c_int {
            C2Rust_Unnamed_0::COORD_MINUS.0 as ::core::ffi::c_int
        } else {
            C2Rust_Unnamed_0::COORD_PLUS.0 as ::core::ffi::c_int
        },
    ), ());
    uqm_lua_checked!(NPCNumber(
        ac / 10 as ::core::ffi::c_int,
        b"%03d\0".as_ptr() as *const ::core::ffi::c_char,
    ), ());
    uqm_lua_checked!(NPCPhrase_splice(C2Rust_Unnamed_0::COORD_POINT.0 as ::core::ffi::c_int), ());
    uqm_lua_checked!(NPCNumber(ac % 10 as ::core::ffi::c_int, ::core::ptr::null::<::core::ffi::c_char>()), ());
}
unsafe extern "C" fn CombatIsInevitable(mut R: RESPONSE_REF) {
    if R as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        if uqm_lua_checked!(getGameStateUint(
            b"DESTRUCT_CODE_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) != 0
        {
            uqm_lua_checked!(DoResponsePhrase(
                C2Rust_Unnamed_0::destruct_code.0 as ::core::ffi::c_int as RESPONSE_REF,
                Some(CombatIsInevitable as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            ), ());
        }
        match uqm_lua_checked!(getGameStateUint(
            b"SLYLANDRO_PROBE_THREAT\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) {
            0 => {
                threat = C2Rust_Unnamed_0::threat_1.0 as ::core::ffi::c_int
                    as RESPONSE_REF;
            }
            1 => {
                threat = C2Rust_Unnamed_0::threat_2.0 as ::core::ffi::c_int
                    as RESPONSE_REF;
            }
            2 => {
                threat = C2Rust_Unnamed_0::threat_3.0 as ::core::ffi::c_int
                    as RESPONSE_REF;
            }
            _ => {
                threat = C2Rust_Unnamed_0::threat_4.0 as ::core::ffi::c_int
                    as RESPONSE_REF;
            }
        }
        uqm_lua_checked!(DoResponsePhrase(
            threat,
            Some(CombatIsInevitable as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
        match uqm_lua_checked!(getGameStateUint(
            b"SLYLANDRO_PROBE_WRONG\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) {
            0 => {
                something_wrong = C2Rust_Unnamed_0::something_wrong_1.0
                    as ::core::ffi::c_int as RESPONSE_REF;
            }
            1 => {
                something_wrong = C2Rust_Unnamed_0::something_wrong_2.0
                    as ::core::ffi::c_int as RESPONSE_REF;
            }
            2 => {
                something_wrong = C2Rust_Unnamed_0::something_wrong_3.0
                    as ::core::ffi::c_int as RESPONSE_REF;
            }
            _ => {
                something_wrong = C2Rust_Unnamed_0::something_wrong_4.0
                    as ::core::ffi::c_int as RESPONSE_REF;
            }
        }
        uqm_lua_checked!(DoResponsePhrase(
            something_wrong,
            Some(CombatIsInevitable as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
        match uqm_lua_checked!(getGameStateUint(
            b"SLYLANDRO_PROBE_ID\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) {
            0 => {
                we_are_us = C2Rust_Unnamed_0::we_are_us_1.0 as ::core::ffi::c_int
                    as RESPONSE_REF;
            }
            1 => {
                we_are_us = C2Rust_Unnamed_0::we_are_us_2.0 as ::core::ffi::c_int
                    as RESPONSE_REF;
            }
            2 => {
                we_are_us = C2Rust_Unnamed_0::we_are_us_3.0 as ::core::ffi::c_int
                    as RESPONSE_REF;
            }
            _ => {
                we_are_us = C2Rust_Unnamed_0::we_are_us_4.0 as ::core::ffi::c_int
                    as RESPONSE_REF;
            }
        }
        uqm_lua_checked!(DoResponsePhrase(
            we_are_us,
            Some(CombatIsInevitable as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
        match uqm_lua_checked!(getGameStateUint(
            b"SLYLANDRO_PROBE_INFO\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) {
            0 => {
                why_attack = C2Rust_Unnamed_0::why_attack_1.0 as ::core::ffi::c_int
                    as RESPONSE_REF;
            }
            1 => {
                why_attack = C2Rust_Unnamed_0::why_attack_2.0 as ::core::ffi::c_int
                    as RESPONSE_REF;
            }
            2 => {
                why_attack = C2Rust_Unnamed_0::why_attack_3.0 as ::core::ffi::c_int
                    as RESPONSE_REF;
            }
            _ => {
                why_attack = C2Rust_Unnamed_0::why_attack_4.0 as ::core::ffi::c_int
                    as RESPONSE_REF;
            }
        }
        uqm_lua_checked!(DoResponsePhrase(
            why_attack,
            Some(CombatIsInevitable as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
        match uqm_lua_checked!(getGameStateUint(
            b"SLYLANDRO_PROBE_EXIT\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) {
            0 => {
                bye = C2Rust_Unnamed_0::bye_1.0 as ::core::ffi::c_int as RESPONSE_REF;
            }
            1 => {
                bye = C2Rust_Unnamed_0::bye_2.0 as ::core::ffi::c_int as RESPONSE_REF;
            }
            2 => {
                bye = C2Rust_Unnamed_0::bye_3.0 as ::core::ffi::c_int as RESPONSE_REF;
            }
            _ => {
                bye = C2Rust_Unnamed_0::bye_4.0 as ::core::ffi::c_int as RESPONSE_REF;
            }
        }
        uqm_lua_checked!(DoResponsePhrase(
            bye,
            Some(CombatIsInevitable as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::destruct_code.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::DESTRUCT_SEQUENCE.0 as ::core::ffi::c_int, None), ());
        uqm_lua_checked!(setSegue(Segue::Segue_victory), ());
    } else {
        let mut NumVisits: BYTE = 0;
        if R as ::core::ffi::c_int == threat as ::core::ffi::c_int {
            NumVisits = uqm_lua_checked!(getGameStateUint(
                b"SLYLANDRO_PROBE_THREAT\0".as_ptr() as *const ::core::ffi::c_char,
            ), ()) as BYTE;
            let c2rust_fresh1 = NumVisits;
            NumVisits = NumVisits.wrapping_add(1);
            match c2rust_fresh1 as ::core::ffi::c_int {
                0 => {
                    uqm_lua_checked!(NPCPhrase_cb(
                        C2Rust_Unnamed_0::PROGRAMMED_TO_DEFEND_1.0 as ::core::ffi::c_int,
                        None,
                    ), ());
                }
                1 => {
                    uqm_lua_checked!(NPCPhrase_cb(
                        C2Rust_Unnamed_0::PROGRAMMED_TO_DEFEND_2.0 as ::core::ffi::c_int,
                        None,
                    ), ());
                }
                2 => {
                    uqm_lua_checked!(NPCPhrase_cb(
                        C2Rust_Unnamed_0::PROGRAMMED_TO_DEFEND_3.0 as ::core::ffi::c_int,
                        None,
                    ), ());
                }
                3 => {
                    uqm_lua_checked!(NPCPhrase_cb(
                        C2Rust_Unnamed_0::PROGRAMMED_TO_DEFEND_4.0 as ::core::ffi::c_int,
                        None,
                    ), ());
                    NumVisits = NumVisits.wrapping_sub(1);
                }
                _ => {}
            }
            uqm_lua_checked!(setGameStateUint(
                b"SLYLANDRO_PROBE_THREAT\0".as_ptr() as *const ::core::ffi::c_char,
                NumVisits as DWORD,
            ), ());
        } else if R as ::core::ffi::c_int == something_wrong as ::core::ffi::c_int {
            NumVisits = uqm_lua_checked!(getGameStateUint(
                b"SLYLANDRO_PROBE_WRONG\0".as_ptr() as *const ::core::ffi::c_char,
            ), ()) as BYTE;
            let c2rust_fresh2 = NumVisits;
            NumVisits = NumVisits.wrapping_add(1);
            match c2rust_fresh2 as ::core::ffi::c_int {
                0 => {
                    uqm_lua_checked!(NPCPhrase_cb(
                        C2Rust_Unnamed_0::NOMINAL_FUNCTION_1.0 as ::core::ffi::c_int,
                        None,
                    ), ());
                }
                1 => {
                    uqm_lua_checked!(NPCPhrase_cb(
                        C2Rust_Unnamed_0::NOMINAL_FUNCTION_2.0 as ::core::ffi::c_int,
                        None,
                    ), ());
                }
                2 => {
                    uqm_lua_checked!(NPCPhrase_cb(
                        C2Rust_Unnamed_0::NOMINAL_FUNCTION_3.0 as ::core::ffi::c_int,
                        None,
                    ), ());
                }
                3 => {
                    uqm_lua_checked!(NPCPhrase_cb(
                        C2Rust_Unnamed_0::NOMINAL_FUNCTION_4.0 as ::core::ffi::c_int,
                        None,
                    ), ());
                    NumVisits = NumVisits.wrapping_sub(1);
                }
                _ => {}
            }
            uqm_lua_checked!(setGameStateUint(
                b"SLYLANDRO_PROBE_WRONG\0".as_ptr() as *const ::core::ffi::c_char,
                NumVisits as DWORD,
            ), ());
        } else if R as ::core::ffi::c_int == we_are_us as ::core::ffi::c_int {
            NumVisits = uqm_lua_checked!(getGameStateUint(
                b"SLYLANDRO_PROBE_ID\0".as_ptr() as *const ::core::ffi::c_char,
            ), ()) as BYTE;
            let c2rust_fresh3 = NumVisits;
            NumVisits = NumVisits.wrapping_add(1);
            match c2rust_fresh3 as ::core::ffi::c_int {
                0 => {
                    uqm_lua_checked!(NPCPhrase_cb(
                        C2Rust_Unnamed_0::THIS_IS_PROBE_1.0 as ::core::ffi::c_int,
                        None,
                    ), ());
                }
                1 => {
                    uqm_lua_checked!(NPCPhrase_cb(
                        C2Rust_Unnamed_0::THIS_IS_PROBE_2.0 as ::core::ffi::c_int,
                        None,
                    ), ());
                }
                2 => {
                    uqm_lua_checked!(NPCPhrase_cb(
                        C2Rust_Unnamed_0::THIS_IS_PROBE_3.0 as ::core::ffi::c_int,
                        None,
                    ), ());
                }
                3 => {
                    let mut dx: SIZE = 0;
                    let mut dy: SIZE = 0;
                    dx = ((plot_map[C2Rust_Unnamed_1::SLYLANDRO_DEFINED.0
                            as ::core::ffi::c_int as usize]
                        .star_pt
                        .x as ::core::ffi::c_int
                        - logxToUniverse(GlobData.SIS_state.log_x))
                        * if plot_map[C2Rust_Unnamed_1::SLYLANDRO_DEFINED.0
                                as ::core::ffi::c_int as usize]
                            .star_pt
                            .x as ::core::ffi::c_int > 5000 as ::core::ffi::c_int
                        {
                            1 as ::core::ffi::c_int
                        } else {
                            -1 as ::core::ffi::c_int
                        }) as SIZE;
                    dy = ((plot_map[C2Rust_Unnamed_1::SLYLANDRO_DEFINED.0
                            as ::core::ffi::c_int as usize]
                        .star_pt
                        .y as ::core::ffi::c_int
                        - logyToUniverse(GlobData.SIS_state.log_y))
                        * if plot_map[C2Rust_Unnamed_1::SLYLANDRO_DEFINED.0
                                as ::core::ffi::c_int as usize]
                            .star_pt
                            .y as ::core::ffi::c_int > 5000 as ::core::ffi::c_int
                        {
                            1 as ::core::ffi::c_int
                        } else {
                            -1 as ::core::ffi::c_int
                        }) as SIZE;
                    uqm_lua_checked!(NPCPhrase_cb(
                        C2Rust_Unnamed_0::THIS_IS_PROBE_40.0 as ::core::ffi::c_int,
                        None,
                    ), ());
                    uqm_lua_checked!(sayCoord(dy as ::core::ffi::c_int), ());
                    uqm_lua_checked!(NPCPhrase_splice(
                        C2Rust_Unnamed_0::THIS_IS_PROBE_41.0 as ::core::ffi::c_int,
                    ), ());
                    uqm_lua_checked!(sayCoord(dx as ::core::ffi::c_int), ());
                    uqm_lua_checked!(NPCPhrase_cb(
                        C2Rust_Unnamed_0::THIS_IS_PROBE_42.0 as ::core::ffi::c_int,
                        None,
                    ), ());
                    NumVisits = NumVisits.wrapping_sub(1);
                }
                _ => {}
            }
            uqm_lua_checked!(setGameStateUint(
                b"SLYLANDRO_PROBE_ID\0".as_ptr() as *const ::core::ffi::c_char,
                NumVisits as DWORD,
            ), ());
        } else if R as ::core::ffi::c_int == why_attack as ::core::ffi::c_int {
            NumVisits = uqm_lua_checked!(getGameStateUint(
                b"SLYLANDRO_PROBE_INFO\0".as_ptr() as *const ::core::ffi::c_char,
            ), ()) as BYTE;
            let c2rust_fresh4 = NumVisits;
            NumVisits = NumVisits.wrapping_add(1);
            match c2rust_fresh4 as ::core::ffi::c_int {
                0 => {
                    uqm_lua_checked!(NPCPhrase_cb(
                        C2Rust_Unnamed_0::PEACEFUL_MISSION_1.0 as ::core::ffi::c_int,
                        None,
                    ), ());
                }
                1 => {
                    uqm_lua_checked!(NPCPhrase_cb(
                        C2Rust_Unnamed_0::PEACEFUL_MISSION_2.0 as ::core::ffi::c_int,
                        None,
                    ), ());
                }
                2 => {
                    uqm_lua_checked!(NPCPhrase_cb(
                        C2Rust_Unnamed_0::PEACEFUL_MISSION_3.0 as ::core::ffi::c_int,
                        None,
                    ), ());
                }
                3 => {
                    uqm_lua_checked!(NPCPhrase_cb(
                        C2Rust_Unnamed_0::PEACEFUL_MISSION_4.0 as ::core::ffi::c_int,
                        None,
                    ), ());
                    NumVisits = NumVisits.wrapping_sub(1);
                }
                _ => {}
            }
            uqm_lua_checked!(setGameStateUint(
                b"SLYLANDRO_PROBE_INFO\0".as_ptr() as *const ::core::ffi::c_char,
                NumVisits as DWORD,
            ), ());
        } else if R as ::core::ffi::c_int == bye as ::core::ffi::c_int {
            NumVisits = uqm_lua_checked!(getGameStateUint(
                b"SLYLANDRO_PROBE_EXIT\0".as_ptr() as *const ::core::ffi::c_char,
            ), ()) as BYTE;
            let c2rust_fresh5 = NumVisits;
            NumVisits = NumVisits.wrapping_add(1);
            match c2rust_fresh5 as ::core::ffi::c_int {
                0 => {
                    uqm_lua_checked!(NPCPhrase_cb(
                        C2Rust_Unnamed_0::GOODBYE_1.0 as ::core::ffi::c_int,
                        None,
                    ), ());
                }
                1 => {
                    uqm_lua_checked!(NPCPhrase_cb(
                        C2Rust_Unnamed_0::GOODBYE_2.0 as ::core::ffi::c_int,
                        None,
                    ), ());
                }
                2 => {
                    uqm_lua_checked!(NPCPhrase_cb(
                        C2Rust_Unnamed_0::GOODBYE_3.0 as ::core::ffi::c_int,
                        None,
                    ), ());
                }
                3 => {
                    uqm_lua_checked!(NPCPhrase_cb(
                        C2Rust_Unnamed_0::GOODBYE_4.0 as ::core::ffi::c_int,
                        None,
                    ), ());
                    NumVisits = NumVisits.wrapping_sub(1);
                }
                _ => {}
            }
            uqm_lua_checked!(setGameStateUint(
                b"SLYLANDRO_PROBE_EXIT\0".as_ptr() as *const ::core::ffi::c_char,
                NumVisits as DWORD,
            ), ());
        }
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::HOSTILE.0 as ::core::ffi::c_int, None), ());
        uqm_lua_checked!(setGameStateUint(
            b"PROBE_EXHIBITED_BUG\0".as_ptr() as *const ::core::ffi::c_char,
            1 as DWORD,
        ), ());
        uqm_lua_checked!(setSegue(Segue::Segue_hostile), ());
    };
}
unsafe extern "C" fn Intro() {
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
        && uqm_lua_checked!(getGameStateUint(
            b"GLOBAL_FLAGS_AND_DATA\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as DWORD != 0
    {
        let mut i: COUNT = 0;
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::COORD_POINT.0 as ::core::ffi::c_int, None), ());
        i = 0 as COUNT;
        while (i as ::core::ffi::c_int) < 16 as ::core::ffi::c_int {
            uqm_lua_checked!(NPCNumber(
                random_ramble[i as usize] as ::core::ffi::c_int,
                b"%d \0".as_ptr() as *const ::core::ffi::c_char,
            ), ());
            i = i.wrapping_add(1);
        }
        uqm_lua_checked!(NPCNumber(
            random_ramble[16usize] as ::core::ffi::c_int,
            b"%d\0".as_ptr() as *const ::core::ffi::c_char,
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::COORD_POINT.0 as ::core::ffi::c_int, None), ());
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::HOSTILE.0 as ::core::ffi::c_int, None), ());
        uqm_lua_checked!(setGameStateUint(
            b"PROBE_EXHIBITED_BUG\0".as_ptr() as *const ::core::ffi::c_char,
            1 as DWORD,
        ), ());
        uqm_lua_checked!(setSegue(Segue::Segue_hostile), ());
    } else {
        let mut NumVisits: BYTE = 0;
        NumVisits = uqm_lua_checked!(getGameStateUint(
            b"SLYLANDRO_PROBE_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) as BYTE;
        let c2rust_fresh0 = NumVisits;
        NumVisits = NumVisits.wrapping_add(1);
        match c2rust_fresh0 as ::core::ffi::c_int {
            0 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::WE_COME_IN_PEACE_1.0 as ::core::ffi::c_int,
                    None,
                ), ());
            }
            1 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::WE_COME_IN_PEACE_2.0 as ::core::ffi::c_int,
                    None,
                ), ());
            }
            2 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::WE_COME_IN_PEACE_3.0 as ::core::ffi::c_int,
                    None,
                ), ());
            }
            3 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::WE_COME_IN_PEACE_4.0 as ::core::ffi::c_int,
                    None,
                ), ());
            }
            4 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::WE_COME_IN_PEACE_5.0 as ::core::ffi::c_int,
                    None,
                ), ());
            }
            5 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::WE_COME_IN_PEACE_6.0 as ::core::ffi::c_int,
                    None,
                ), ());
            }
            6 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::WE_COME_IN_PEACE_7.0 as ::core::ffi::c_int,
                    None,
                ), ());
            }
            7 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::WE_COME_IN_PEACE_8.0 as ::core::ffi::c_int,
                    None,
                ), ());
                NumVisits = NumVisits.wrapping_sub(1);
            }
            _ => {}
        }
        uqm_lua_checked!(setGameStateUint(
            b"SLYLANDRO_PROBE_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
            NumVisits as DWORD,
        ), ());
        uqm_lua_checked!(CombatIsInevitable(0 as ::core::ffi::c_int as RESPONSE_REF), ());
    };
}
unsafe extern "C" fn uninit_slyland() -> COUNT {
    return 0 as COUNT;
}
unsafe extern "C" fn post_slyland_enc() {}
#[no_mangle]
pub unsafe extern "C" fn init_slyland_comm() -> *mut LOCDATA {
    let mut retval: *mut LOCDATA = ::core::ptr::null_mut::<LOCDATA>();
    slylandro_desc.init_encounter_func = Some(Intro as unsafe extern "C" fn() -> ())
        as Option<unsafe extern "C" fn() -> ()>;
    slylandro_desc.post_encounter_func = Some(
        post_slyland_enc as unsafe extern "C" fn() -> (),
    ) as Option<unsafe extern "C" fn() -> ()>;
    slylandro_desc.uninit_encounter_func = Some(
        uninit_slyland as unsafe extern "C" fn() -> COUNT,
    ) as Option<unsafe extern "C" fn() -> COUNT>;
    slylandro_desc.AlienTextBaseline.x = (((1 as ::core::ffi::c_int) << resolutionFactor)
        + (CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
            - (if if optWindowType == 1 as DWORD {
                C2Rust_Unnamed::TRUE.0 as ::core::ffi::c_int
            } else {
                C2Rust_Unnamed::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                16 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } << resolutionFactor) * 2 as ::core::ffi::c_int
            - (((13 as ::core::ffi::c_int) << resolutionFactor)
                + (if if optWindowType == 1 as DWORD {
                    C2Rust_Unnamed::TRUE.0 as ::core::ffi::c_int
                } else {
                    C2Rust_Unnamed::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    1 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                } << resolutionFactor))
            - ((1 as ::core::ffi::c_int) << resolutionFactor << 1 as ::core::ffi::c_int)
            >> 1 as ::core::ffi::c_int)) as COORD;
    slylandro_desc.AlienTextBaseline.y = 0 as COORD;
    slylandro_desc.AlienTextWidth = (CanvasWidth
        - ((64 as ::core::ffi::c_int) << resolutionFactor)
        - (if if optWindowType == 1 as DWORD {
            C2Rust_Unnamed::TRUE.0 as ::core::ffi::c_int
        } else {
            C2Rust_Unnamed::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            16 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        } << resolutionFactor) * 2 as ::core::ffi::c_int
        - (((13 as ::core::ffi::c_int) << resolutionFactor)
            + (if if optWindowType == 1 as DWORD {
                C2Rust_Unnamed::TRUE.0 as ::core::ffi::c_int
            } else {
                C2Rust_Unnamed::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } << resolutionFactor))
        - ((1 as ::core::ffi::c_int) << resolutionFactor << 1 as ::core::ffi::c_int)
        - ((16 as ::core::ffi::c_int) << resolutionFactor)) as COUNT;
    uqm_lua_checked!(setSegue(Segue::Segue_hostile), ::core::ptr::null_mut());
    retval = &raw mut slylandro_desc;
    return retval;
}
