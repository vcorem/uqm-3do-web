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

// Rust translation of src/uqm/comm/supox/supoxc.c.
extern "C" {
    pub type string_table_entry;
    pub type frame_desc;
    pub type font_desc;
    pub type tfb_soundsample;
    fn SetAbsStringTableIndex(String: STRING, StringTableIndex: COUNT) -> STRING;
    fn GetStringAddress(String: STRING) -> STRINGPTR;
    static mut resolutionFactor: ::core::ffi::c_uint;
    static mut optWindowType: DWORD;
    static mut CanvasWidth: ::core::ffi::c_int;
    fn setGameStateUint(name: *const ::core::ffi::c_char, val: DWORD);
    fn getGameStateUint(name: *const ::core::ffi::c_char) -> DWORD;
    static mut GlobData: GLOBDATA;
    static mut CommData: LOCDATA;
    fn DoResponsePhrase(
        R: RESPONSE_REF,
        response_func: RESPONSE_FUNC,
        ContstructStr: *mut UNICODE,
    );
    fn NPCPhrase_cb(index: ::core::ffi::c_int, cb: CallbackFunction);
    fn setSegue(segue: Segue);
    fn AlienTalkSegue(wait_track: COUNT);
    fn luaUqm_comm_init(
        customFuncs: *const luaUqm_custom_Function,
        scriptRes: RESOURCE,
    ) -> BOOLEAN;
    fn luaUqm_comm_uninit();
    fn AddEscortShips(race: RACE_ID, count: SIZE) -> COUNT;
    fn ShipsReady(race: RACE_ID) -> BOOLEAN;
    fn PrepareShip(race: RACE_ID);
    fn StartSphereTracking(race: RACE_ID) -> COUNT;
    fn EscortFeasibilityStudy(race: RACE_ID) -> COUNT;
    fn CheckAlliance(race: RACE_ID) -> COUNT;
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed {
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct luaUqm_custom_Function {
    pub name: *const ::core::ffi::c_char,
    pub fun: Option<unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int>,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct RACE_ID(pub ::core::ffi::c_uint);
impl RACE_ID {
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
    pub const ANDROSYNTH_SHIP: Self = Self(22);
    pub const CHENJESU_SHIP: Self = Self(23);
    pub const MMRNMHRM_SHIP: Self = Self(24);
    pub const NUM_BUILDABLE_SHIPS: Self = Self(25);
    pub const YEHAT_REBEL_SHIP: Self = Self(25);
    pub const URQUAN_DRONE_SHIP: Self = Self(26);
    pub const SAMATRA_SHIP: Self = Self(26);
    pub const NUM_AVAILABLE_RACES: Self = Self(27);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_0 {
    pub const NULL_PHRASE: Self = Self(0);
    pub const NEUTRAL_SPACE_HELLO_1: Self = Self(1);
    pub const NEUTRAL_SPACE_HELLO_2: Self = Self(2);
    pub const NEUTRAL_HOMEWORLD_HELLO_1: Self = Self(3);
    pub const NEUTRAL_HOMEWORLD_HELLO_2: Self = Self(4);
    pub const HOSTILE_SPACE_HELLO_1: Self = Self(5);
    pub const HOSTILE_SPACE_HELLO_2: Self = Self(6);
    pub const ALLIED_HOMEWORLD_HELLO_1: Self = Self(7);
    pub const ALLIED_HOMEWORLD_HELLO_2: Self = Self(8);
    pub const ALLIED_HOMEWORLD_HELLO_3: Self = Self(9);
    pub const ALLIED_HOMEWORLD_HELLO_4: Self = Self(10);
    pub const i_am: Self = Self(11);
    pub const WE_ARE_SUPOX: Self = Self(12);
    pub const my_ship: Self = Self(13);
    pub const OUR_SHIP: Self = Self(14);
    pub const from_alliance: Self = Self(15);
    pub const FROM_SUPOX: Self = Self(16);
    pub const are_you_copying: Self = Self(17);
    pub const YEAH_SORRY: Self = Self(18);
    pub const why_copy: Self = Self(19);
    pub const SYMBIOTS: Self = Self(20);
    pub const tell_us_of_your_species: Self = Self(21);
    pub const OUR_SPECIES: Self = Self(22);
    pub const plants_arent_intelligent: Self = Self(23);
    pub const PROVES_WERE_SPECIAL: Self = Self(24);
    pub const anyone_around_here: Self = Self(25);
    pub const UTWIG_NEARBY: Self = Self(26);
    pub const what_relation_to_utwig: Self = Self(27);
    pub const UTWIG_ALLIES: Self = Self(28);
    pub const whats_wrong_with_utwig: Self = Self(29);
    pub const BROKE_ULTRON: Self = Self(30);
    pub const whats_ultron: Self = Self(31);
    pub const TAKE_ULTRON: Self = Self(32);
    pub const what_do_i_do_now: Self = Self(33);
    pub const FIX_IT: Self = Self(34);
    pub const thanks_now_we_eat_you: Self = Self(35);
    pub const HIDEOUS_MONSTERS: Self = Self(36);
    pub const got_fixed_ultron: Self = Self(37);
    pub const GOOD_GIVE_TO_UTWIG: Self = Self(38);
    pub const look_i_repaired_lots: Self = Self(39);
    pub const ALMOST_THERE: Self = Self(40);
    pub const look_i_slightly_repaired: Self = Self(41);
    pub const GREAT_DO_MORE: Self = Self(42);
    pub const where_get_repairs: Self = Self(43);
    pub const ANCIENT_RHYME: Self = Self(44);
    pub const bye_neutral: Self = Self(45);
    pub const GOODBYE_NEUTRAL: Self = Self(46);
    pub const ABOUT_BATTLE: Self = Self(47);
    pub const HELLO_BEFORE_KOHRAH_SPACE_1: Self = Self(48);
    pub const HELLO_BEFORE_KOHRAH_SPACE_2: Self = Self(49);
    pub const HELLO_DURING_KOHRAH_SPACE_1: Self = Self(50);
    pub const HELLO_DURING_KOHRAH_SPACE_2: Self = Self(51);
    pub const HELLO_AFTER_KOHRAH_SPACE_1: Self = Self(52);
    pub const HELLO_AFTER_KOHRAH_SPACE_2: Self = Self(53);
    pub const whats_up_after_space: Self = Self(54);
    pub const GENERAL_INFO_AFTER_SPACE_1: Self = Self(55);
    pub const GENERAL_INFO_AFTER_SPACE_2: Self = Self(56);
    pub const what_now_after_space: Self = Self(57);
    pub const DO_THIS_AFTER_SPACE: Self = Self(58);
    pub const bye_after_space: Self = Self(59);
    pub const GOODBYE_AFTER_SPACE: Self = Self(60);
    pub const whats_up_before_space: Self = Self(61);
    pub const GENERAL_INFO_BEFORE_SPACE_1: Self = Self(62);
    pub const GENERAL_INFO_BEFORE_SPACE_2: Self = Self(63);
    pub const what_now_before_space: Self = Self(64);
    pub const DO_THIS_BEFORE_SPACE: Self = Self(65);
    pub const bye_before_space: Self = Self(66);
    pub const GOODBYE_BEFORE_SPACE: Self = Self(67);
    pub const how_went_war: Self = Self(68);
    pub const how_goes_war: Self = Self(69);
    pub const BATTLE_HAPPENS_1: Self = Self(70);
    pub const BATTLE_HAPPENS_2: Self = Self(71);
    pub const FLEET_ON_WAY: Self = Self(72);
    pub const learn_new_info: Self = Self(73);
    pub const NO_NEW_INFO: Self = Self(74);
    pub const SAMATRA: Self = Self(75);
    pub const what_now_homeworld: Self = Self(76);
    pub const HOPE_KILL_EACH_OTHER: Self = Self(77);
    pub const UP_TO_YOU: Self = Self(78);
    pub const can_you_help: Self = Self(79);
    pub const HOW_HELP: Self = Self(80);
    pub const DONT_NEED: Self = Self(81);
    pub const HAVE_4_SHIPS: Self = Self(82);
    pub const give_info: Self = Self(83);
    pub const GOOD_HINTS: Self = Self(84);
    pub const how_is_ultron: Self = Self(85);
    pub const ULTRON_IS_GREAT: Self = Self(86);
    pub const bye_allied_homeworld: Self = Self(87);
    pub const GOODBYE_ALLIED_HOMEWORLD: Self = Self(88);
    pub const name_1: Self = Self(89);
    pub const name_2: Self = Self(90);
    pub const name_3: Self = Self(91);
    pub const name_4: Self = Self(92);
    pub const OUT_TAKES: Self = Self(93);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_1(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_1 {
    pub const DEAD_GUY: Self = Self(0);
    pub const GOOD_GUY: Self = Self(1);
    pub const BAD_GUY: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_2(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_2 {
    pub const CHMMR_HOME: Self = Self(0);
    pub const ORZ_HOME: Self = Self(1);
    pub const PKUNK_HOME: Self = Self(2);
    pub const SHOFIXTI_HOME: Self = Self(3);
    pub const SPATHI_HOME: Self = Self(4);
    pub const SUPOX_HOME: Self = Self(5);
    pub const THRADDASH_HOME: Self = Self(6);
    pub const UTWIG_HOME: Self = Self(7);
    pub const VUX_HOME: Self = Self(8);
    pub const YEHAT_HOME: Self = Self(9);
    pub const DRUUGE_HOME: Self = Self(10);
    pub const ILWRATH_HOME: Self = Self(11);
    pub const MYCON_HOME: Self = Self(12);
    pub const SLYLANDRO_HOME: Self = Self(13);
    pub const UMGAH_HOME: Self = Self(14);
    pub const ZOQFOTPIK_HOME: Self = Self(15);
    pub const SYREEN_HOME: Self = Self(16);
    pub const ANDROSYNTH_HOME: Self = Self(17);
    pub const NUM_AVAILABLE_HOMES: Self = Self(18);
}
#[inline]
unsafe extern "C" fn IsHomeworldKnown(mut homeworld: DWORD) -> BOOLEAN {
    if homeworld > 18 as DWORD {
        return BOOLEAN::FALSE;
    }
    return BOOLEAN(
        uqm_lua_checked!(getGameStateUint(b"KNOW_HOMEWORLD\0".as_ptr() as *const ::core::ffi::c_char), BOOLEAN(0))
            & ((1 as ::core::ffi::c_int) << homeworld) as DWORD,
    );
}
#[inline]
unsafe extern "C" fn SetHomeworldKnown(mut homeworld: DWORD) {
    let mut current: DWORD = 0;
    if uqm_lua_checked!(IsHomeworldKnown(homeworld), ()).0 != 0 {
        return;
    }
    current = uqm_lua_checked!(getGameStateUint(
        b"KNOW_HOMEWORLD\0".as_ptr() as *const ::core::ffi::c_char,
    ), ());
    current |= ((1 as ::core::ffi::c_int) << homeworld) as DWORD;
    uqm_lua_checked!(setGameStateUint(
        b"KNOW_HOMEWORLD\0".as_ptr() as *const ::core::ffi::c_char,
        current,
    ), ());
}
static mut supox_desc: LOCDATA = LOCDATA {
    AlienConv: CONVERSATION::SUPOX_CONVERSATION,
    init_encounter_func: None,
    post_encounter_func: None,
    uninit_encounter_func: None,
    AlienFrameRes: b"comm.supox.graphics\0".as_ptr() as RESOURCE,
    AlienFontRes: b"comm.supox.font\0".as_ptr() as RESOURCE,
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
    AlienColorMapRes: b"comm.supox.colortable\0".as_ptr() as RESOURCE,
    AlienSongRes: b"comm.supox.music\0".as_ptr() as RESOURCE,
    AltRes: ALT_RESOURCE {
        AlienFrameRes: ::core::ptr::null::<::core::ffi::c_char>(),
        AlienColorMapRes: ::core::ptr::null::<::core::ffi::c_char>(),
        AlienSongRes: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    ConversationPhrasesRes: b"comm.supox.dialogue\0".as_ptr() as RESOURCE,
    NumAnimations: 4 as COUNT,
    AlienAmbientArray: [
        ANIMATION_DESC {
            StartIndex: 4 as COUNT,
            NumFrames: 5 as BYTE,
            AnimFlags: ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as COUNT,
            BaseFrameRate: (840 as ::core::ffi::c_int / 10 as ::core::ffi::c_int)
                as COUNT,
            RandomFrameRate: (840 as ::core::ffi::c_int / 10 as ::core::ffi::c_int)
                as COUNT,
            BaseRestartRate: 840 as COUNT,
            RandomRestartRate: (840 as ::core::ffi::c_int * 3 as ::core::ffi::c_int)
                as COUNT,
            BlockMask: 0 as DWORD,
        },
        ANIMATION_DESC {
            StartIndex: 9 as COUNT,
            NumFrames: 10 as BYTE,
            AnimFlags: ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as COUNT,
            BaseFrameRate: (840 as ::core::ffi::c_int / 15 as ::core::ffi::c_int)
                as COUNT,
            RandomFrameRate: 0 as COUNT,
            BaseRestartRate: 840 as COUNT,
            RandomRestartRate: (840 as ::core::ffi::c_int * 3 as ::core::ffi::c_int)
                as COUNT,
            BlockMask: 0 as DWORD,
        },
        ANIMATION_DESC {
            StartIndex: 19 as COUNT,
            NumFrames: 10 as BYTE,
            AnimFlags: ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as COUNT,
            BaseFrameRate: (840 as ::core::ffi::c_int / 15 as ::core::ffi::c_int)
                as COUNT,
            RandomFrameRate: 0 as COUNT,
            BaseRestartRate: 840 as COUNT,
            RandomRestartRate: (840 as ::core::ffi::c_int * 3 as ::core::ffi::c_int)
                as COUNT,
            BlockMask: 0 as DWORD,
        },
        ANIMATION_DESC {
            StartIndex: 29 as COUNT,
            NumFrames: 13 as BYTE,
            AnimFlags: ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as COUNT,
            BaseFrameRate: (840 as ::core::ffi::c_int / 15 as ::core::ffi::c_int)
                as COUNT,
            RandomFrameRate: 0 as COUNT,
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
        StartIndex: 1 as COUNT,
        NumFrames: 3 as BYTE,
        AnimFlags: 0 as COUNT,
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
unsafe extern "C" fn ExitConversation(mut R: RESPONSE_REF) {
    uqm_lua_checked!(setSegue(Segue::Segue_peace), ());
    if R as ::core::ffi::c_int == C2Rust_Unnamed_0::bye_neutral.0 as ::core::ffi::c_int {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::GOODBYE_NEUTRAL.0 as ::core::ffi::c_int, None), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::what_do_i_do_now.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::FIX_IT.0 as ::core::ffi::c_int, None), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::thanks_now_we_eat_you.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::HIDEOUS_MONSTERS.0 as ::core::ffi::c_int, None), ());
        uqm_lua_checked!(setGameStateUint(
            b"SUPOX_HOSTILE\0".as_ptr() as *const ::core::ffi::c_char,
            1 as DWORD,
        ), ());
        uqm_lua_checked!(setGameStateUint(
            b"SUPOX_HOME_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
            0 as DWORD,
        ), ());
        uqm_lua_checked!(setGameStateUint(
            b"SUPOX_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
            0 as DWORD,
        ), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::bye_after_space.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::GOODBYE_AFTER_SPACE.0 as ::core::ffi::c_int,
            None,
        ), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::bye_before_space.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::GOODBYE_BEFORE_SPACE.0 as ::core::ffi::c_int,
            None,
        ), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::bye_allied_homeworld.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::GOODBYE_ALLIED_HOMEWORLD.0 as ::core::ffi::c_int,
            None,
        ), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::can_you_help.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::HOW_HELP.0 as ::core::ffi::c_int, None), ());
        if EscortFeasibilityStudy(RACE_ID::SUPOX_SHIP) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::DONT_NEED.0 as ::core::ffi::c_int, None), ());
        } else {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::HAVE_4_SHIPS.0 as ::core::ffi::c_int, None), ());
            uqm_lua_checked!(AlienTalkSegue(!(0 as ::core::ffi::c_int) as COUNT), ());
            uqm_lua_checked!(AddEscortShips(
                RACE_ID::SUPOX_SHIP,
                (if if GlobData.SIS_state.Difficulty as ::core::ffi::c_int != 0 {
                    GlobData.SIS_state.Difficulty as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                } == 0 as ::core::ffi::c_int
                {
                    4 as ::core::ffi::c_int
                } else if if GlobData.SIS_state.Difficulty as ::core::ffi::c_int != 0 {
                    GlobData.SIS_state.Difficulty as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                } == 1 as ::core::ffi::c_int
                {
                    4 as ::core::ffi::c_int
                } else {
                    2 as ::core::ffi::c_int
                }) as SIZE,
            ), ());
            uqm_lua_checked!(PrepareShip(RACE_ID::SUPOX_SHIP), ());
        }
    }
}
unsafe extern "C" fn AlliedHome(mut R: RESPONSE_REF) {
    let mut NumVisits: BYTE = 0;
    let mut News: BYTE = 0;
    News = uqm_lua_checked!(getGameStateUint(b"SUPOX_WAR_NEWS\0".as_ptr() as *const ::core::ffi::c_char), ())
        as BYTE;
    NumVisits = uqm_lua_checked!(getGameStateUint(
        b"UTWIG_SUPOX_MISSION\0".as_ptr() as *const ::core::ffi::c_char,
    ), ()) as BYTE;
    if R as ::core::ffi::c_int == C2Rust_Unnamed_0::how_went_war.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::ABOUT_BATTLE.0 as ::core::ffi::c_int, None), ());
        News = (News as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as BYTE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::how_goes_war.0 as ::core::ffi::c_int
    {
        if NumVisits as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::FLEET_ON_WAY.0 as ::core::ffi::c_int, None), ());
            uqm_lua_checked!(setGameStateUint(
                b"SUPOX_WAR_NEWS\0".as_ptr() as *const ::core::ffi::c_char,
                1 as DWORD,
            ), ());
        } else {
            match uqm_lua_checked!(getGameStateUint(
                b"SUPOX_WAR_NEWS\0".as_ptr() as *const ::core::ffi::c_char,
            ), ()) {
                0 => {
                    uqm_lua_checked!(NPCPhrase_cb(
                        C2Rust_Unnamed_0::BATTLE_HAPPENS_1.0 as ::core::ffi::c_int,
                        None,
                    ), ());
                    News = 1 as BYTE;
                }
                1 => {
                    uqm_lua_checked!(NPCPhrase_cb(
                        C2Rust_Unnamed_0::BATTLE_HAPPENS_2.0 as ::core::ffi::c_int,
                        None,
                    ), ());
                    News = 2 as BYTE;
                }
                _ => {}
            }
        }
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_0::how_goes_war.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::learn_new_info.0 as ::core::ffi::c_int
    {
        if (NumVisits as ::core::ffi::c_int) < 5 as ::core::ffi::c_int {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::NO_NEW_INFO.0 as ::core::ffi::c_int, None), ());
        } else {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::SAMATRA.0 as ::core::ffi::c_int, None), ());
            News = (News as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as BYTE;
        }
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_0::learn_new_info.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::what_now_homeworld.0 as ::core::ffi::c_int
    {
        if (NumVisits as ::core::ffi::c_int) < 5 as ::core::ffi::c_int {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::UP_TO_YOU.0 as ::core::ffi::c_int, None), ());
        } else {
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_0::HOPE_KILL_EACH_OTHER.0 as ::core::ffi::c_int,
                None,
            ), ());
        }
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_0::what_now_homeworld.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::how_is_ultron.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::ULTRON_IS_GREAT.0 as ::core::ffi::c_int, None), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_0::how_is_ultron.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if if GlobData.SIS_state.Extended as ::core::ffi::c_int != 0 {
        BOOLEAN::TRUE.0 as ::core::ffi::c_int
    } else {
        BOOLEAN::FALSE.0 as ::core::ffi::c_int
    } != 0
        && R as ::core::ffi::c_int == C2Rust_Unnamed_0::give_info.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::GOOD_HINTS.0 as ::core::ffi::c_int, None), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_0::give_info.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    }
    uqm_lua_checked!(setGameStateUint(
        b"SUPOX_WAR_NEWS\0".as_ptr() as *const ::core::ffi::c_char,
        News as DWORD,
    ), ());
    if NumVisits as ::core::ffi::c_int >= 5 as ::core::ffi::c_int {
        if News as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int == 0
        {
            uqm_lua_checked!(DoResponsePhrase(
                C2Rust_Unnamed_0::how_went_war.0 as ::core::ffi::c_int as RESPONSE_REF,
                Some(AlliedHome as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            ), ());
        }
    } else if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_0::how_goes_war.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        && (NumVisits as ::core::ffi::c_int == 1 as ::core::ffi::c_int
            && News as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            || NumVisits as ::core::ffi::c_int != 0
                && (News as ::core::ffi::c_int) < 2 as ::core::ffi::c_int)
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_0::how_goes_war.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(AlliedHome as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_0::learn_new_info.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_0::learn_new_info.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(AlliedHome as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_0::what_now_homeworld.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_0::what_now_homeworld.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(AlliedHome as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_0::how_is_ultron.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_0::how_is_ultron.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(AlliedHome as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    if if GlobData.SIS_state.Extended as ::core::ffi::c_int != 0 {
        BOOLEAN::TRUE.0 as ::core::ffi::c_int
    } else {
        BOOLEAN::FALSE.0 as ::core::ffi::c_int
    } != 0
        && *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_0::give_info.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_0::give_info.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(AlliedHome as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    if if GlobData.SIS_state.Extended as ::core::ffi::c_int != 0 {
        BOOLEAN::TRUE.0 as ::core::ffi::c_int
    } else {
        BOOLEAN::FALSE.0 as ::core::ffi::c_int
    } != 0 && uqm_lua_checked!(ShipsReady(RACE_ID::SUPOX_SHIP), ()).0 != 0
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_0::can_you_help.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(ExitConversation as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    uqm_lua_checked!(DoResponsePhrase(
        C2Rust_Unnamed_0::bye_allied_homeworld.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(ExitConversation as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    ), ());
}
unsafe extern "C" fn BeforeKohrAh(mut R: RESPONSE_REF) {
    let mut NumVisits: BYTE = 0;
    if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::whats_up_before_space.0 as ::core::ffi::c_int
    {
        NumVisits = uqm_lua_checked!(getGameStateUint(
            b"SUPOX_INFO\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) as BYTE;
        let c2rust_fresh8 = NumVisits;
        NumVisits = NumVisits.wrapping_add(1);
        match c2rust_fresh8 as ::core::ffi::c_int {
            0 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::GENERAL_INFO_BEFORE_SPACE_1.0
                        as ::core::ffi::c_int,
                    None,
                ), ());
            }
            1 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::GENERAL_INFO_BEFORE_SPACE_2.0
                        as ::core::ffi::c_int,
                    None,
                ), ());
                NumVisits = NumVisits.wrapping_sub(1);
            }
            _ => {}
        }
        uqm_lua_checked!(setGameStateUint(
            b"SUPOX_INFO\0".as_ptr() as *const ::core::ffi::c_char,
            NumVisits as DWORD,
        ), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_0::whats_up_before_space.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::what_now_before_space.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::DO_THIS_BEFORE_SPACE.0 as ::core::ffi::c_int,
            None,
        ), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_0::what_now_before_space.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    }
    if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_0::whats_up_before_space.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_0::whats_up_before_space.0 as ::core::ffi::c_int
                as RESPONSE_REF,
            Some(BeforeKohrAh as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_0::what_now_before_space.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_0::what_now_before_space.0 as ::core::ffi::c_int
                as RESPONSE_REF,
            Some(BeforeKohrAh as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    uqm_lua_checked!(DoResponsePhrase(
        C2Rust_Unnamed_0::bye_before_space.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(ExitConversation as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    ), ());
}
unsafe extern "C" fn AfterKohrAh(mut R: RESPONSE_REF) {
    let mut NumVisits: BYTE = 0;
    if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::whats_up_after_space.0 as ::core::ffi::c_int
    {
        NumVisits = uqm_lua_checked!(getGameStateUint(
            b"SUPOX_INFO\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) as BYTE;
        let c2rust_fresh7 = NumVisits;
        NumVisits = NumVisits.wrapping_add(1);
        match c2rust_fresh7 as ::core::ffi::c_int {
            0 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::GENERAL_INFO_AFTER_SPACE_1.0 as ::core::ffi::c_int,
                    None,
                ), ());
            }
            1 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::GENERAL_INFO_AFTER_SPACE_2.0 as ::core::ffi::c_int,
                    None,
                ), ());
                NumVisits = NumVisits.wrapping_sub(1);
            }
            _ => {}
        }
        uqm_lua_checked!(setGameStateUint(
            b"SUPOX_INFO\0".as_ptr() as *const ::core::ffi::c_char,
            NumVisits as DWORD,
        ), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_0::whats_up_after_space.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::what_now_after_space.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::DO_THIS_AFTER_SPACE.0 as ::core::ffi::c_int,
            None,
        ), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_0::what_now_after_space.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    }
    if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_0::whats_up_after_space.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_0::whats_up_after_space.0 as ::core::ffi::c_int
                as RESPONSE_REF,
            Some(AfterKohrAh as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_0::what_now_after_space.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_0::what_now_after_space.0 as ::core::ffi::c_int
                as RESPONSE_REF,
            Some(AfterKohrAh as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    uqm_lua_checked!(DoResponsePhrase(
        C2Rust_Unnamed_0::bye_after_space.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(ExitConversation as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    ), ());
}
unsafe extern "C" fn NeutralSupox(mut R: RESPONSE_REF) {
    let mut i: BYTE = 0;
    let mut LastStack: BYTE = 0;
    let mut NumVisits: BYTE = 0;
    let mut pStr: [RESPONSE_REF; 3] = [0; 3];
    LastStack = 0 as BYTE;
    pStr[2usize] = 0 as RESPONSE_REF;
    pStr[1usize] = pStr[2usize];
    pStr[0usize] = pStr[1usize];
    if R as ::core::ffi::c_int == C2Rust_Unnamed_0::i_am.0 as ::core::ffi::c_int {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::WE_ARE_SUPOX.0 as ::core::ffi::c_int, None), ());
        uqm_lua_checked!(setGameStateUint(
            b"SUPOX_STACK1\0".as_ptr() as *const ::core::ffi::c_char,
            1 as DWORD,
        ), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_0::i_am.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::my_ship.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::OUR_SHIP.0 as ::core::ffi::c_int, None), ());
        uqm_lua_checked!(setGameStateUint(
            b"SUPOX_STACK1\0".as_ptr() as *const ::core::ffi::c_char,
            2 as DWORD,
        ), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_0::my_ship.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::from_alliance.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::FROM_SUPOX.0 as ::core::ffi::c_int, None), ());
        uqm_lua_checked!(setGameStateUint(
            b"SUPOX_STACK1\0".as_ptr() as *const ::core::ffi::c_char,
            3 as DWORD,
        ), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_0::from_alliance.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::are_you_copying.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::YEAH_SORRY.0 as ::core::ffi::c_int, None), ());
        uqm_lua_checked!(setGameStateUint(
            b"SUPOX_STACK1\0".as_ptr() as *const ::core::ffi::c_char,
            4 as DWORD,
        ), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::why_copy.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::SYMBIOTS.0 as ::core::ffi::c_int, None), ());
        uqm_lua_checked!(setGameStateUint(
            b"SUPOX_STACK1\0".as_ptr() as *const ::core::ffi::c_char,
            5 as DWORD,
        ), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::tell_us_of_your_species.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::OUR_SPECIES.0 as ::core::ffi::c_int, None), ());
        LastStack = 1 as BYTE;
        uqm_lua_checked!(setGameStateUint(
            b"SUPOX_STACK2\0".as_ptr() as *const ::core::ffi::c_char,
            1 as DWORD,
        ), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::plants_arent_intelligent.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_0::PROVES_WERE_SPECIAL.0 as ::core::ffi::c_int,
            None,
        ), ());
        uqm_lua_checked!(setGameStateUint(
            b"SUPOX_STACK2\0".as_ptr() as *const ::core::ffi::c_char,
            2 as DWORD,
        ), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::anyone_around_here.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::UTWIG_NEARBY.0 as ::core::ffi::c_int, None), ());
        LastStack = 2 as BYTE;
        uqm_lua_checked!(setGameStateUint(
            b"SUPOX_WAR_NEWS\0".as_ptr() as *const ::core::ffi::c_char,
            1 as DWORD,
        ), ());
        StartSphereTracking(RACE_ID::UTWIG_SHIP);
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::what_relation_to_utwig.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::UTWIG_ALLIES.0 as ::core::ffi::c_int, None), ());
        LastStack = 2 as BYTE;
        uqm_lua_checked!(setGameStateUint(
            b"SUPOX_WAR_NEWS\0".as_ptr() as *const ::core::ffi::c_char,
            1 as DWORD,
        ), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::whats_wrong_with_utwig.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::BROKE_ULTRON.0 as ::core::ffi::c_int, None), ());
        LastStack = 2 as BYTE;
        uqm_lua_checked!(setGameStateUint(
            b"SUPOX_WAR_NEWS\0".as_ptr() as *const ::core::ffi::c_char,
            2 as DWORD,
        ), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::whats_ultron.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::TAKE_ULTRON.0 as ::core::ffi::c_int, None), ());
        uqm_lua_checked!(setGameStateUint(
            b"SUPOX_WAR_NEWS\0".as_ptr() as *const ::core::ffi::c_char,
            0 as DWORD,
        ), ());
        uqm_lua_checked!(setGameStateUint(
            b"ULTRON_CONDITION\0".as_ptr() as *const ::core::ffi::c_char,
            1 as DWORD,
        ), ());
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_0::what_do_i_do_now.0 as ::core::ffi::c_int as RESPONSE_REF,
            if if GlobData.SIS_state.Extended as ::core::ffi::c_int != 0 {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                Some(NeutralSupox as unsafe extern "C" fn(RESPONSE_REF) -> ())
            } else {
                Some(ExitConversation as unsafe extern "C" fn(RESPONSE_REF) -> ())
            },
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_0::thanks_now_we_eat_you.0 as ::core::ffi::c_int
                as RESPONSE_REF,
            Some(ExitConversation as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
        return;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::what_do_i_do_now.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::FIX_IT.0 as ::core::ffi::c_int, None), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::got_fixed_ultron.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::GOOD_GIVE_TO_UTWIG.0 as ::core::ffi::c_int, None), ());
        uqm_lua_checked!(setGameStateUint(
            b"SUPOX_ULTRON_HELP\0".as_ptr() as *const ::core::ffi::c_char,
            1 as DWORD,
        ), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::look_i_repaired_lots.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::ALMOST_THERE.0 as ::core::ffi::c_int, None), ());
        uqm_lua_checked!(setGameStateUint(
            b"SUPOX_ULTRON_HELP\0".as_ptr() as *const ::core::ffi::c_char,
            1 as DWORD,
        ), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::look_i_slightly_repaired.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::GREAT_DO_MORE.0 as ::core::ffi::c_int, None), ());
        uqm_lua_checked!(setGameStateUint(
            b"SUPOX_ULTRON_HELP\0".as_ptr() as *const ::core::ffi::c_char,
            1 as DWORD,
        ), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::where_get_repairs.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::ANCIENT_RHYME.0 as ::core::ffi::c_int, None), ());
        if if GlobData.SIS_state.Extended as ::core::ffi::c_int != 0 {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            StartSphereTracking(RACE_ID::DRUUGE_SHIP);
        }
        uqm_lua_checked!(setGameStateUint(
            b"SUPOX_ULTRON_HELP\0".as_ptr() as *const ::core::ffi::c_char,
            1 as DWORD,
        ), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_0::give_info.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::GOOD_HINTS.0 as ::core::ffi::c_int, None), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_0::give_info.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    }
    match uqm_lua_checked!(getGameStateUint(b"SUPOX_STACK2\0".as_ptr() as *const ::core::ffi::c_char), ()) {
        0 => {
            pStr[1usize] = C2Rust_Unnamed_0::tell_us_of_your_species.0
                as ::core::ffi::c_int as RESPONSE_REF;
        }
        1 => {
            pStr[1usize] = C2Rust_Unnamed_0::plants_arent_intelligent.0
                as ::core::ffi::c_int as RESPONSE_REF;
        }
        _ => {}
    }
    match uqm_lua_checked!(getGameStateUint(b"SUPOX_STACK1\0".as_ptr() as *const ::core::ffi::c_char), ()) {
        0 => {
            pStr[0usize] = C2Rust_Unnamed_0::i_am.0 as ::core::ffi::c_int
                as RESPONSE_REF;
            pStr[1usize] = 0 as RESPONSE_REF;
        }
        1 => {
            pStr[0usize] = C2Rust_Unnamed_0::my_ship.0 as ::core::ffi::c_int
                as RESPONSE_REF;
            pStr[1usize] = 0 as RESPONSE_REF;
        }
        2 => {
            pStr[0usize] = C2Rust_Unnamed_0::from_alliance.0 as ::core::ffi::c_int
                as RESPONSE_REF;
            pStr[1usize] = 0 as RESPONSE_REF;
        }
        3 => {
            pStr[0usize] = C2Rust_Unnamed_0::are_you_copying.0 as ::core::ffi::c_int
                as RESPONSE_REF;
            pStr[1usize] = 0 as RESPONSE_REF;
        }
        4 => {
            pStr[0usize] = C2Rust_Unnamed_0::why_copy.0 as ::core::ffi::c_int
                as RESPONSE_REF;
            pStr[1usize] = 0 as RESPONSE_REF;
        }
        _ => {}
    }
    NumVisits = uqm_lua_checked!(getGameStateUint(
        b"ULTRON_CONDITION\0".as_ptr() as *const ::core::ffi::c_char,
    ), ()) as BYTE;
    if NumVisits as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        match uqm_lua_checked!(getGameStateUint(
            b"SUPOX_WAR_NEWS\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) {
            0 => {
                if uqm_lua_checked!(getGameStateUint(
                    b"UTWIG_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
                ), ()) != 0
                    || uqm_lua_checked!(getGameStateUint(
                        b"UTWIG_HOME_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
                    ), ()) != 0
                    || uqm_lua_checked!(getGameStateUint(
                        b"BOMB_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
                    ), ()) != 0
                {
                    pStr[2usize] = C2Rust_Unnamed_0::what_relation_to_utwig.0
                        as ::core::ffi::c_int as RESPONSE_REF;
                } else {
                    pStr[2usize] = C2Rust_Unnamed_0::anyone_around_here.0
                        as ::core::ffi::c_int as RESPONSE_REF;
                }
            }
            1 => {
                pStr[2usize] = C2Rust_Unnamed_0::whats_wrong_with_utwig.0
                    as ::core::ffi::c_int as RESPONSE_REF;
            }
            2 => {
                pStr[2usize] = C2Rust_Unnamed_0::whats_ultron.0 as ::core::ffi::c_int
                    as RESPONSE_REF;
            }
            _ => {}
        }
    }
    if pStr[LastStack as usize] != 0 {
        uqm_lua_checked!(DoResponsePhrase(
            pStr[LastStack as usize],
            Some(NeutralSupox as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    i = 0 as BYTE;
    while (i as ::core::ffi::c_int) < 3 as ::core::ffi::c_int {
        if i as ::core::ffi::c_int != LastStack as ::core::ffi::c_int
            && pStr[i as usize] as ::core::ffi::c_int != 0
        {
            uqm_lua_checked!(DoResponsePhrase(
                pStr[i as usize],
                Some(NeutralSupox as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            ), ());
        }
        i = i.wrapping_add(1);
    }
    if uqm_lua_checked!(getGameStateUint(b"SUPOX_ULTRON_HELP\0".as_ptr() as *const ::core::ffi::c_char), ())
        == 0
    {
        match NumVisits as ::core::ffi::c_int {
            1 => {
                uqm_lua_checked!(DoResponsePhrase(
                    C2Rust_Unnamed_0::where_get_repairs.0 as ::core::ffi::c_int
                        as RESPONSE_REF,
                    Some(NeutralSupox as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                    ::core::ptr::null_mut::<UNICODE>(),
                ), ());
            }
            2 => {
                uqm_lua_checked!(DoResponsePhrase(
                    C2Rust_Unnamed_0::look_i_slightly_repaired.0 as ::core::ffi::c_int
                        as RESPONSE_REF,
                    Some(NeutralSupox as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                    ::core::ptr::null_mut::<UNICODE>(),
                ), ());
            }
            3 => {
                uqm_lua_checked!(DoResponsePhrase(
                    C2Rust_Unnamed_0::look_i_repaired_lots.0 as ::core::ffi::c_int
                        as RESPONSE_REF,
                    Some(NeutralSupox as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                    ::core::ptr::null_mut::<UNICODE>(),
                ), ());
            }
            4 => {
                uqm_lua_checked!(DoResponsePhrase(
                    C2Rust_Unnamed_0::got_fixed_ultron.0 as ::core::ffi::c_int
                        as RESPONSE_REF,
                    Some(NeutralSupox as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                    ::core::ptr::null_mut::<UNICODE>(),
                ), ());
            }
            _ => {}
        }
    }
    if if GlobData.SIS_state.Extended as ::core::ffi::c_int != 0 {
        BOOLEAN::TRUE.0 as ::core::ffi::c_int
    } else {
        BOOLEAN::FALSE.0 as ::core::ffi::c_int
    } != 0 && NumVisits as ::core::ffi::c_int != 0
        && *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_0::give_info.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_0::give_info.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(NeutralSupox as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    uqm_lua_checked!(DoResponsePhrase(
        C2Rust_Unnamed_0::bye_neutral.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(ExitConversation as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    ), ());
}
unsafe extern "C" fn Intro() {
    let mut NumVisits: BYTE = 0;
    if GlobData.Game_state.CurrentActivity as BYTE as ::core::ffi::c_int
        == C2Rust_Unnamed::WON_LAST_BATTLE.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_0::OUT_TAKES.0 as ::core::ffi::c_int, None), ());
        uqm_lua_checked!(setSegue(Segue::Segue_peace), ());
        return;
    }
    if uqm_lua_checked!(getGameStateUint(b"SUPOX_HOSTILE\0".as_ptr() as *const ::core::ffi::c_char), ()) != 0 {
        NumVisits = uqm_lua_checked!(getGameStateUint(
            b"SUPOX_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) as BYTE;
        let c2rust_fresh0 = NumVisits;
        NumVisits = NumVisits.wrapping_add(1);
        match c2rust_fresh0 as ::core::ffi::c_int {
            0 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::HOSTILE_SPACE_HELLO_1.0 as ::core::ffi::c_int,
                    None,
                ), ());
            }
            1 => {
                uqm_lua_checked!(NPCPhrase_cb(
                    C2Rust_Unnamed_0::HOSTILE_SPACE_HELLO_2.0 as ::core::ffi::c_int,
                    None,
                ), ());
                NumVisits = NumVisits.wrapping_sub(1);
            }
            _ => {}
        }
        uqm_lua_checked!(setGameStateUint(
            b"SUPOX_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
            NumVisits as DWORD,
        ), ());
        uqm_lua_checked!(setSegue(Segue::Segue_peace), ());
    } else if CheckAlliance(RACE_ID::SUPOX_SHIP) as ::core::ffi::c_int
        == C2Rust_Unnamed_1::GOOD_GUY.0 as ::core::ffi::c_int
    {
        if uqm_lua_checked!(getGameStateUint(
            b"GLOBAL_FLAGS_AND_DATA\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) & ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as DWORD != 0
        {
            NumVisits = uqm_lua_checked!(getGameStateUint(
                b"SUPOX_HOME_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
            ), ()) as BYTE;
            let c2rust_fresh1 = NumVisits;
            NumVisits = NumVisits.wrapping_add(1);
            match c2rust_fresh1 as ::core::ffi::c_int {
                0 => {
                    uqm_lua_checked!(NPCPhrase_cb(
                        C2Rust_Unnamed_0::ALLIED_HOMEWORLD_HELLO_1.0
                            as ::core::ffi::c_int,
                        None,
                    ), ());
                }
                1 => {
                    uqm_lua_checked!(NPCPhrase_cb(
                        C2Rust_Unnamed_0::ALLIED_HOMEWORLD_HELLO_2.0
                            as ::core::ffi::c_int,
                        None,
                    ), ());
                }
                2 => {
                    uqm_lua_checked!(NPCPhrase_cb(
                        C2Rust_Unnamed_0::ALLIED_HOMEWORLD_HELLO_3.0
                            as ::core::ffi::c_int,
                        None,
                    ), ());
                }
                3 => {
                    uqm_lua_checked!(NPCPhrase_cb(
                        C2Rust_Unnamed_0::ALLIED_HOMEWORLD_HELLO_4.0
                            as ::core::ffi::c_int,
                        None,
                    ), ());
                    NumVisits = NumVisits.wrapping_sub(1);
                }
                _ => {}
            }
            uqm_lua_checked!(setGameStateUint(
                b"SUPOX_HOME_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
                NumVisits as DWORD,
            ), ());
            uqm_lua_checked!(AlliedHome(0 as ::core::ffi::c_int as RESPONSE_REF), ());
        } else {
            NumVisits = uqm_lua_checked!(getGameStateUint(
                b"UTWIG_SUPOX_MISSION\0".as_ptr() as *const ::core::ffi::c_char,
            ), ()) as BYTE;
            if NumVisits as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
                NumVisits = uqm_lua_checked!(getGameStateUint(
                    b"SUPOX_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
                ), ()) as BYTE;
                let c2rust_fresh2 = NumVisits;
                NumVisits = NumVisits.wrapping_add(1);
                match c2rust_fresh2 as ::core::ffi::c_int {
                    0 => {
                        uqm_lua_checked!(NPCPhrase_cb(
                            C2Rust_Unnamed_0::HELLO_BEFORE_KOHRAH_SPACE_1.0
                                as ::core::ffi::c_int,
                            None,
                        ), ());
                    }
                    1 => {
                        uqm_lua_checked!(NPCPhrase_cb(
                            C2Rust_Unnamed_0::HELLO_BEFORE_KOHRAH_SPACE_2.0
                                as ::core::ffi::c_int,
                            None,
                        ), ());
                        NumVisits = NumVisits.wrapping_sub(1);
                    }
                    _ => {}
                }
                uqm_lua_checked!(setGameStateUint(
                    b"SUPOX_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
                    NumVisits as DWORD,
                ), ());
                uqm_lua_checked!(BeforeKohrAh(0 as ::core::ffi::c_int as RESPONSE_REF), ());
            } else if (NumVisits as ::core::ffi::c_int) < 5 as ::core::ffi::c_int {
                NumVisits = uqm_lua_checked!(getGameStateUint(
                    b"SUPOX_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
                ), ()) as BYTE;
                let c2rust_fresh3 = NumVisits;
                NumVisits = NumVisits.wrapping_add(1);
                match c2rust_fresh3 as ::core::ffi::c_int {
                    0 => {
                        uqm_lua_checked!(NPCPhrase_cb(
                            C2Rust_Unnamed_0::HELLO_DURING_KOHRAH_SPACE_1.0
                                as ::core::ffi::c_int,
                            None,
                        ), ());
                    }
                    1 => {
                        uqm_lua_checked!(NPCPhrase_cb(
                            C2Rust_Unnamed_0::HELLO_DURING_KOHRAH_SPACE_2.0
                                as ::core::ffi::c_int,
                            None,
                        ), ());
                        NumVisits = NumVisits.wrapping_sub(1);
                    }
                    _ => {}
                }
                uqm_lua_checked!(setGameStateUint(
                    b"SUPOX_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
                    NumVisits as DWORD,
                ), ());
            } else {
                NumVisits = uqm_lua_checked!(getGameStateUint(
                    b"SUPOX_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
                ), ()) as BYTE;
                let c2rust_fresh4 = NumVisits;
                NumVisits = NumVisits.wrapping_add(1);
                match c2rust_fresh4 as ::core::ffi::c_int {
                    0 => {
                        uqm_lua_checked!(NPCPhrase_cb(
                            C2Rust_Unnamed_0::HELLO_AFTER_KOHRAH_SPACE_1.0
                                as ::core::ffi::c_int,
                            None,
                        ), ());
                    }
                    1 => {
                        uqm_lua_checked!(NPCPhrase_cb(
                            C2Rust_Unnamed_0::HELLO_AFTER_KOHRAH_SPACE_2.0
                                as ::core::ffi::c_int,
                            None,
                        ), ());
                        NumVisits = NumVisits.wrapping_sub(1);
                    }
                    _ => {}
                }
                uqm_lua_checked!(setGameStateUint(
                    b"SUPOX_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
                    NumVisits as DWORD,
                ), ());
                uqm_lua_checked!(AfterKohrAh(0 as ::core::ffi::c_int as RESPONSE_REF), ());
            }
        }
    } else {
        if uqm_lua_checked!(getGameStateUint(
            b"GLOBAL_FLAGS_AND_DATA\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) & ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as DWORD != 0
        {
            NumVisits = uqm_lua_checked!(getGameStateUint(
                b"SUPOX_HOME_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
            ), ()) as BYTE;
            let c2rust_fresh5 = NumVisits;
            NumVisits = NumVisits.wrapping_add(1);
            match c2rust_fresh5 as ::core::ffi::c_int {
                0 => {
                    uqm_lua_checked!(NPCPhrase_cb(
                        C2Rust_Unnamed_0::NEUTRAL_HOMEWORLD_HELLO_1.0
                            as ::core::ffi::c_int,
                        None,
                    ), ());
                }
                1 => {
                    uqm_lua_checked!(NPCPhrase_cb(
                        C2Rust_Unnamed_0::NEUTRAL_HOMEWORLD_HELLO_2.0
                            as ::core::ffi::c_int,
                        None,
                    ), ());
                    NumVisits = NumVisits.wrapping_sub(1);
                }
                _ => {}
            }
            uqm_lua_checked!(setGameStateUint(
                b"SUPOX_HOME_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
                NumVisits as DWORD,
            ), ());
            uqm_lua_checked!(SetHomeworldKnown(
                C2Rust_Unnamed_2::SUPOX_HOME.0 as ::core::ffi::c_int as DWORD,
            ), ());
        } else {
            NumVisits = uqm_lua_checked!(getGameStateUint(
                b"SUPOX_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
            ), ()) as BYTE;
            let c2rust_fresh6 = NumVisits;
            NumVisits = NumVisits.wrapping_add(1);
            match c2rust_fresh6 as ::core::ffi::c_int {
                0 => {
                    uqm_lua_checked!(NPCPhrase_cb(
                        C2Rust_Unnamed_0::NEUTRAL_SPACE_HELLO_1.0 as ::core::ffi::c_int,
                        None,
                    ), ());
                }
                1 => {
                    uqm_lua_checked!(NPCPhrase_cb(
                        C2Rust_Unnamed_0::NEUTRAL_SPACE_HELLO_2.0 as ::core::ffi::c_int,
                        None,
                    ), ());
                    NumVisits = NumVisits.wrapping_sub(1);
                }
                _ => {}
            }
            uqm_lua_checked!(setGameStateUint(
                b"SUPOX_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
                NumVisits as DWORD,
            ), ());
        }
        uqm_lua_checked!(NeutralSupox(0 as ::core::ffi::c_int as RESPONSE_REF), ());
    };
}
unsafe extern "C" fn uninit_supox() -> COUNT {
    luaUqm_comm_uninit();
    return 0 as COUNT;
}
unsafe extern "C" fn post_supox_enc() {}
#[no_mangle]
pub unsafe extern "C" fn init_supox_comm() -> *mut LOCDATA {
    let mut retval: *mut LOCDATA = ::core::ptr::null_mut::<LOCDATA>();
    supox_desc.init_encounter_func = Some(Intro as unsafe extern "C" fn() -> ())
        as Option<unsafe extern "C" fn() -> ()>;
    supox_desc.post_encounter_func = Some(post_supox_enc as unsafe extern "C" fn() -> ())
        as Option<unsafe extern "C" fn() -> ()>;
    supox_desc.uninit_encounter_func = Some(
        uninit_supox as unsafe extern "C" fn() -> COUNT,
    ) as Option<unsafe extern "C" fn() -> COUNT>;
    uqm_lua_checked!(luaUqm_comm_init(
        ::core::ptr::null::<luaUqm_custom_Function>(),
        ::core::ptr::null::<::core::ffi::c_char>(),
    ), ::core::ptr::null_mut());
    supox_desc.AlienTextBaseline.x = (((1 as ::core::ffi::c_int) << resolutionFactor)
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
                } << resolutionFactor))
            - ((1 as ::core::ffi::c_int) << resolutionFactor << 1 as ::core::ffi::c_int)
            >> 1 as ::core::ffi::c_int)) as COORD;
    supox_desc.AlienTextBaseline.y = 0 as COORD;
    supox_desc.AlienTextWidth = (CanvasWidth
        - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
        - ((1 as ::core::ffi::c_int) << resolutionFactor << 1 as ::core::ffi::c_int)
        - ((16 as ::core::ffi::c_int) << resolutionFactor)) as COUNT;
    if uqm_lua_checked!(getGameStateUint(b"SUPOX_HOSTILE\0".as_ptr() as *const ::core::ffi::c_char), ::core::ptr::null_mut()) == 0
        || GlobData.Game_state.CurrentActivity as BYTE as ::core::ffi::c_int
            == C2Rust_Unnamed::WON_LAST_BATTLE.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(setSegue(Segue::Segue_peace), ::core::ptr::null_mut());
    } else {
        uqm_lua_checked!(setSegue(Segue::Segue_hostile), ::core::ptr::null_mut());
    }
    retval = &raw mut supox_desc;
    return retval;
}
