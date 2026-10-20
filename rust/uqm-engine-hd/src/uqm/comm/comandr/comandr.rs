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
    fn SetAbsFrameIndex(Frame: FRAME, FrameIndex: COUNT) -> FRAME;
    fn XFormColorMap(ColorMapPtr: COLORMAPPTR, TimeInterval: SIZE) -> DWORD;
    fn GetColorMapAddress(_: COLORMAP) -> COLORMAPPTR;
    fn SetAbsStringTableIndex(String: STRING, StringTableIndex: COUNT) -> STRING;
    fn GetStringAddress(String: STRING) -> STRINGPTR;
    fn LoadMusicInstance(res: RESOURCE) -> MUSIC_REF;
    fn SetMusicVolume(Volume: COUNT);
    fn PLRGetPos() -> DWORD;
    fn FadeMusic(end_vol: BYTE, TimeInterval: SIZE) -> DWORD;
    static mut resolutionFactor: ::core::ffi::c_uint;
    static mut optWindowType: DWORD;
    fn DrawLanders();
    fn DeltaSISGauges(
        crew_delta: SIZE,
        fuel_delta: SDWORD,
        resunit_delta: ::core::ffi::c_int,
    );
    static mut FilterData: FILTER_DESC;
    fn SwitchSequences(enableAll: BOOLEAN);
    fn EngageFilters(f_desc: *mut FILTER_DESC);
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
    fn EnableTalkingAnim(enable: BOOLEAN);
    static mut altResFlags: BYTE;
    fn luaUqm_comm_init(
        customFuncs: *const luaUqm_custom_Function,
        scriptRes: RESOURCE,
    ) -> BOOLEAN;
    fn luaUqm_comm_uninit();
    fn StopMusic();
    fn SeekMusic(pos: DWORD);
    fn PlayMusic(MusicRef: MUSIC_REF, Continuous: BOOLEAN, Priority: BYTE);
    fn TFB_Random() -> DWORD;
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
    pub flags: BYTE,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed {
    pub const DRAW_REPLACE: Self = Self(0);
    pub const DRAW_ADDITIVE: Self = Self(1);
    pub const DRAW_ALPHA: Self = Self(2);
    pub const DRAW_MULTIPLY: Self = Self(3);
    pub const DRAW_OVERLAY: Self = Self(4);
    pub const DRAW_SCREEN: Self = Self(5);
    pub const DRAW_GRAYSCALE: Self = Self(6);
    pub const DRAW_LINEARBURN: Self = Self(7);
    pub const DRAW_HYPTOQUAS: Self = Self(8);
    pub const DRAW_DESATURATE: Self = Self(9);
    pub const DRAW_DEFAULT: Self = Self(0);
}
pub type TFB_SoundSample = tfb_soundsample;
pub type MUSIC_REF = *mut *mut TFB_SoundSample;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_0 {
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
pub struct FILTER {
    pub ColorIndex: BYTE,
    pub OpacityIndex: BYTE,
    pub FrameIndex: SIZE,
    pub Kind: BYTE,
    pub Flags: BYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FILTER_DESC {
    pub NumFilters: BYTE,
    pub FilterArray: [FILTER; 3],
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct luaUqm_custom_Function {
    pub name: *const ::core::ffi::c_char,
    pub fun: Option<unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int>,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_1(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_1 {
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
    pub const OK_THE_NAME_IS_EMPIRE: Self = Self(16);
    pub const FUEL_UP0: Self = Self(17);
    pub const FUEL_UP1: Self = Self(18);
    pub const WHAT_KIND_OF_IDIOT: Self = Self(19);
    pub const DONT_KNOW_WHO_YOU_ARE: Self = Self(20);
    pub const THATS_IMPOSSIBLE: Self = Self(21);
    pub const ASK_AWAY: Self = Self(22);
    pub const RADIOS_ON_MERCURY: Self = Self(23);
    pub const THANKS_FOR_HELPING: Self = Self(24);
    pub const STARBASE_IS: Self = Self(25);
    pub const HAPPENED_TO_EARTH: Self = Self(26);
    pub const URQUAN_LEFT: Self = Self(27);
    pub const BASE_ON_MOON: Self = Self(28);
    pub const ACKNOWLEDGE_SECRET: Self = Self(29);
    pub const ABOUT_BASE: Self = Self(30);
    pub const GOOD_LUCK_WITH_BASE: Self = Self(31);
    pub const DEALT_WITH_BASE_YET: Self = Self(32);
    pub const HERE_COMES_ILWRATH: Self = Self(33);
    pub const VERY_IMPRESSIVE: Self = Self(34);
    pub const IT_WAS_ABANDONED: Self = Self(35);
    pub const YOU_REALLY_FOUGHT_BASE: Self = Self(36);
    pub const IM_GLAD_YOU_WON: Self = Self(37);
    pub const IM_SURE_IT_WAS_DIFFICULT: Self = Self(38);
    pub const THAT_WAS_PROBE: Self = Self(39);
    pub const DEEP_TROUBLE: Self = Self(40);
    pub const GOOD_NEWS: Self = Self(41);
    pub const SURE_HOPE: Self = Self(42);
    pub const ABOUT_BASE_AGAIN: Self = Self(43);
    pub const COOK_BUTTS: Self = Self(44);
    pub const OVERTHROW_ALIENS: Self = Self(45);
    pub const KILL_MONSTERS: Self = Self(46);
    pub const GOOD_LUCK_AGAIN: Self = Self(47);
    pub const STARBASE_WILL_BE_READY: Self = Self(48);
    pub const overthrow_evil_aliens: Self = Self(49);
    pub const annihilate_those_monsters: Self = Self(50);
    pub const cook_their_butts: Self = Self(51);
    pub const where_get_radios: Self = Self(52);
    pub const well_go_get_them_now: Self = Self(53);
    pub const we_will_transfer_now: Self = Self(54);
    pub const what_will_you_give_us: Self = Self(55);
    pub const before_radios_we_need_info: Self = Self(56);
    pub const no_but_well_help: Self = Self(57);
    pub const yes_this_is_supply_ship: Self = Self(58);
    pub const what_slave_planet: Self = Self(59);
    pub const i_lied: Self = Self(60);
    pub const plumb_out: Self = Self(61);
    pub const we_are_vindicator: Self = Self(62);
    pub const first_give_info: Self = Self(63);
    pub const we_must_go_now: Self = Self(64);
    pub const where_can_i_get_radios: Self = Self(65);
    pub const ok_i_will_get_radios: Self = Self(66);
    pub const whats_this_starbase: Self = Self(67);
    pub const what_about_earth: Self = Self(68);
    pub const where_are_urquan: Self = Self(69);
    pub const our_mission_was_secret: Self = Self(70);
    pub const we_are_here_to_help: Self = Self(71);
    pub const tell_me_about_base: Self = Self(72);
    pub const we_will_take_care_of_base: Self = Self(73);
    pub const tell_me_again: Self = Self(74);
    pub const base_was_abandoned: Self = Self(75);
    pub const we_fought_them: Self = Self(76);
    pub const oh_yes_big_fight: Self = Self(77);
    pub const i_lied_it_was_abandoned: Self = Self(78);
    pub const i_cant_talk_about_it: Self = Self(79);
    pub const name_1: Self = Self(80);
    pub const name_2: Self = Self(81);
    pub const name_3: Self = Self(82);
    pub const name_4: Self = Self(83);
    pub const i_lost_my_lander: Self = Self(84);
    pub const i_lost_another_lander: Self = Self(85);
    pub const need_fuel_mercury: Self = Self(86);
    pub const need_fuel_luna: Self = Self(87);
    pub const need_fuel_again: Self = Self(88);
    pub const what_was_red_thing: Self = Self(89);
    pub const it_went_away: Self = Self(90);
    pub const we_destroyed_it: Self = Self(91);
    pub const what_probe: Self = Self(92);
    pub const take_care_of_base_again: Self = Self(93);
    pub const goodbye_commander: Self = Self(94);
}
static mut commander_desc: LOCDATA = LOCDATA {
    AlienConv: CONVERSATION::COMMANDER_CONVERSATION,
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
    AltRes: ALT_RESOURCE {
        AlienFrameRes: b"comm.starbas.graphics\0".as_ptr() as RESOURCE,
        AlienColorMapRes: ::core::ptr::null::<::core::ffi::c_char>(),
        AlienSongRes: b"comm.commander.lowpower.music\0".as_ptr() as RESOURCE,
    },
    ConversationPhrasesRes: b"comm.commander.dialogue\0".as_ptr() as RESOURCE,
    NumAnimations: 4 as COUNT,
    AlienAmbientArray: [
        ANIMATION_DESC {
            StartIndex: 1 as COUNT,
            NumFrames: 3 as BYTE,
            AnimFlags: ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as COUNT,
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
            AnimFlags: ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as COUNT,
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
            AnimFlags: (((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                as ::core::ffi::c_long
                | (1 as ::core::ffi::c_long) << 8 as ::core::ffi::c_int) as COUNT,
            BaseFrameRate: 0 as COUNT,
            RandomFrameRate: (840 as ::core::ffi::c_int / 30 as ::core::ffi::c_int)
                as COUNT,
            BaseRestartRate: 0 as COUNT,
            RandomRestartRate: (840 as ::core::ffi::c_int / 15 as ::core::ffi::c_int)
                as COUNT,
            BlockMask: 0 as DWORD,
        },
        ANIMATION_DESC {
            StartIndex: 40 as COUNT,
            NumFrames: 16 as BYTE,
            AnimFlags: (((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int)
                as ::core::ffi::c_long
                | (1 as ::core::ffi::c_long) << 8 as ::core::ffi::c_int
                | ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                    as ::core::ffi::c_long) as COUNT,
            BaseFrameRate: 0 as COUNT,
            RandomFrameRate: (840 as ::core::ffi::c_int / 5 as ::core::ffi::c_int)
                as COUNT,
            BaseRestartRate: 0 as COUNT,
            RandomRestartRate: (840 as ::core::ffi::c_int / 4 as ::core::ffi::c_int)
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
        StartIndex: 4 as COUNT,
        NumFrames: 6 as BYTE,
        AnimFlags: 0 as COUNT,
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
static mut commander_filters: FILTER_DESC = FILTER_DESC {
    NumFilters: 2 as BYTE,
    FilterArray: [
        FILTER {
            ColorIndex: 0 as BYTE,
            OpacityIndex: 1 as BYTE,
            FrameIndex: 56 as SIZE,
            Kind: C2Rust_Unnamed::DRAW_MULTIPLY.0 as ::core::ffi::c_int as BYTE,
            Flags: ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as BYTE,
        },
        FILTER {
            ColorIndex: 2 as BYTE,
            OpacityIndex: 3 as BYTE,
            FrameIndex: 56 as SIZE,
            Kind: C2Rust_Unnamed::DRAW_OVERLAY.0 as ::core::ffi::c_int as BYTE,
            Flags: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as BYTE,
        },
        FILTER {
            ColorIndex: 0,
            OpacityIndex: 0,
            FrameIndex: 0,
            Kind: 0,
            Flags: 0,
        },
    ],
};
unsafe extern "C" fn RaveParty() {
    CommData.AlienColorMap = SetAbsStringTableIndex(
        CommData.AlienColorMap,
        TFB_Random().wrapping_rem(3 as DWORD).wrapping_add(1 as DWORD) as COUNT,
    ) as COLORMAP;
    if if resolutionFactor != 2 as ::core::ffi::c_uint {
        BOOLEAN::FALSE.0 as ::core::ffi::c_int
    } else {
        BOOLEAN::TRUE.0 as ::core::ffi::c_int
    } != 0
        && if GlobData.SIS_state.Extended as ::core::ffi::c_int != 0 {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
    {
        CommData.AlienFrame = SetAbsFrameIndex(
            CommData.AlienFrame,
            TFB_Random().wrapping_rem(15 as DWORD).wrapping_add(40 as DWORD) as COUNT,
        );
        SwitchSequences(BOOLEAN::FALSE);
        EnableTalkingAnim(BOOLEAN::FALSE);
        CommData.AlienAmbientArray[3usize].AnimFlags = (CommData
            .AlienAmbientArray[3usize]
            .AnimFlags as ::core::ffi::c_long
            & !((1 as ::core::ffi::c_long) << 8 as ::core::ffi::c_int)) as COUNT;
    } else {
        CommData.AlienAmbientArray[2usize].AnimFlags = (CommData
            .AlienAmbientArray[2usize]
            .AnimFlags as ::core::ffi::c_long
            & !((1 as ::core::ffi::c_long) << 8 as ::core::ffi::c_int)) as COUNT;
        if if resolutionFactor != 2 as ::core::ffi::c_uint {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } != 0
        {
            EngageFilters(&raw mut commander_filters);
        }
    };
}
unsafe extern "C" fn ByeBye(mut R: RESPONSE_REF) {
    if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::ok_i_will_get_radios.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::THANKS_FOR_HELPING.0 as ::core::ffi::c_int, None), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::well_go_get_them_now.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_1::GLAD_WHEN_YOU_COME_BACK.0 as ::core::ffi::c_int,
            None,
        ), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::we_will_take_care_of_base.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_1::GOOD_LUCK_WITH_BASE.0 as ::core::ffi::c_int,
            None,
        ), ());
        uqm_lua_checked!(setGameStateUint(
            b"WILL_DESTROY_BASE\0".as_ptr() as *const ::core::ffi::c_char,
            1 as DWORD,
        ), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::take_care_of_base_again.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::GOOD_LUCK_AGAIN.0 as ::core::ffi::c_int, None), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::base_was_abandoned.0 as ::core::ffi::c_int
        || R as ::core::ffi::c_int
            == C2Rust_Unnamed_1::i_lied_it_was_abandoned.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::IT_WAS_ABANDONED.0 as ::core::ffi::c_int, None), ());
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::HERE_COMES_ILWRATH.0 as ::core::ffi::c_int, None), ());
        uqm_lua_checked!(setGameStateUint(
            b"PROBE_ILWRATH_ENCOUNTER\0".as_ptr() as *const ::core::ffi::c_char,
            1 as DWORD,
        ), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::oh_yes_big_fight.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::IM_GLAD_YOU_WON.0 as ::core::ffi::c_int, None), ());
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::HERE_COMES_ILWRATH.0 as ::core::ffi::c_int, None), ());
        uqm_lua_checked!(setGameStateUint(
            b"PROBE_ILWRATH_ENCOUNTER\0".as_ptr() as *const ::core::ffi::c_char,
            1 as DWORD,
        ), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::i_cant_talk_about_it.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_1::IM_SURE_IT_WAS_DIFFICULT.0 as ::core::ffi::c_int,
            None,
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::HERE_COMES_ILWRATH.0 as ::core::ffi::c_int, None), ());
        uqm_lua_checked!(setGameStateUint(
            b"PROBE_ILWRATH_ENCOUNTER\0".as_ptr() as *const ::core::ffi::c_char,
            1 as DWORD,
        ), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::cook_their_butts.0 as ::core::ffi::c_int
        || R as ::core::ffi::c_int
            == C2Rust_Unnamed_1::overthrow_evil_aliens.0 as ::core::ffi::c_int
        || R as ::core::ffi::c_int
            == C2Rust_Unnamed_1::annihilate_those_monsters.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(setGameStateUint(
            b"PROBE_ILWRATH_ENCOUNTER\0".as_ptr() as *const ::core::ffi::c_char,
            0 as DWORD,
        ), ());
        if R as ::core::ffi::c_int
            == C2Rust_Unnamed_1::cook_their_butts.0 as ::core::ffi::c_int
        {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::COOK_BUTTS.0 as ::core::ffi::c_int, None), ());
        } else if R as ::core::ffi::c_int
            == C2Rust_Unnamed_1::overthrow_evil_aliens.0 as ::core::ffi::c_int
        {
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_1::OVERTHROW_ALIENS.0 as ::core::ffi::c_int,
                None,
            ), ());
        } else {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::KILL_MONSTERS.0 as ::core::ffi::c_int, None), ());
        }
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_1::THIS_MAY_SEEM_SILLY.0 as ::core::ffi::c_int,
            None,
        ), ());
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_1::name_1.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(ByeBye as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_1::name_2.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(ByeBye as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_1::name_3.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(ByeBye as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_1::name_4.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(ByeBye as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
        uqm_lua_checked!(setGameStateUint(
            b"STARBASE_AVAILABLE\0".as_ptr() as *const ::core::ffi::c_char,
            1 as DWORD,
        ), ());
    } else {
        if R as ::core::ffi::c_int == C2Rust_Unnamed_1::name_1.0 as ::core::ffi::c_int {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::OK_THE_NAFS.0 as ::core::ffi::c_int, None), ());
            uqm_lua_checked!(setGameStateUint(
                b"NEW_ALLIANCE_NAME\0".as_ptr() as *const ::core::ffi::c_char,
                0 as DWORD,
            ), ());
        } else if R as ::core::ffi::c_int
            == C2Rust_Unnamed_1::name_2.0 as ::core::ffi::c_int
        {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::OK_THE_CAN.0 as ::core::ffi::c_int, None), ());
            uqm_lua_checked!(setGameStateUint(
                b"NEW_ALLIANCE_NAME\0".as_ptr() as *const ::core::ffi::c_char,
                1 as DWORD,
            ), ());
        } else if R as ::core::ffi::c_int
            == C2Rust_Unnamed_1::name_3.0 as ::core::ffi::c_int
        {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::OK_THE_UFW.0 as ::core::ffi::c_int, None), ());
            uqm_lua_checked!(setGameStateUint(
                b"NEW_ALLIANCE_NAME\0".as_ptr() as *const ::core::ffi::c_char,
                2 as DWORD,
            ), ());
        } else {
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_1::OK_THE_NAME_IS_EMPIRE.0 as ::core::ffi::c_int,
                None,
            ), ());
            uqm_lua_checked!(setGameStateUint(
                b"NEW_ALLIANCE_NAME\0".as_ptr() as *const ::core::ffi::c_char,
                3 as DWORD,
            ), ());
        }
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_1::STARBASE_WILL_BE_READY.0 as ::core::ffi::c_int,
            None,
        ), ());
    };
}
unsafe extern "C" fn NoRadioactives(mut R: RESPONSE_REF) {
    if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::yes_this_is_supply_ship.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::ABOUT_TIME.0 as ::core::ffi::c_int, None), ());
        if GlobData
            .SIS_state
            .ElementAmounts[C2Rust_Unnamed_0::RADIOACTIVE.0 as ::core::ffi::c_int
            as usize] != 0
        {
            uqm_lua_checked!(GiveRadios(0 as RESPONSE_REF), ());
        } else {
            uqm_lua_checked!(DoResponsePhrase(
                C2Rust_Unnamed_1::i_lied.0 as ::core::ffi::c_int as RESPONSE_REF,
                Some(NoRadioactives as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            ), ());
            uqm_lua_checked!(DoResponsePhrase(
                C2Rust_Unnamed_1::plumb_out.0 as ::core::ffi::c_int as RESPONSE_REF,
                Some(NoRadioactives as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            ), ());
        }
    } else {
        if R as ::core::ffi::c_int
            == C2Rust_Unnamed_1::where_can_i_get_radios.0 as ::core::ffi::c_int
        {
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_1::RADIOS_ON_MERCURY.0 as ::core::ffi::c_int,
                None,
            ), ());
            *GetStringAddress(
                SetAbsStringTableIndex(
                    CommData.ConversationPhrases,
                    (C2Rust_Unnamed_1::where_can_i_get_radios.0 as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int) as COUNT,
                ),
            ) = '\0' as UNICODE;
        } else if R as ::core::ffi::c_int
            == C2Rust_Unnamed_1::no_but_well_help.0 as ::core::ffi::c_int
        {
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_1::THE_WHAT_FROM_WHERE.0 as ::core::ffi::c_int,
                None,
            ), ());
        } else if R as ::core::ffi::c_int
            == C2Rust_Unnamed_1::what_slave_planet.0 as ::core::ffi::c_int
            || R as ::core::ffi::c_int
                == C2Rust_Unnamed_1::i_lied.0 as ::core::ffi::c_int
        {
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_1::DONT_KNOW_WHO_YOU_ARE.0 as ::core::ffi::c_int,
                None,
            ), ());
        } else if R as ::core::ffi::c_int
            == C2Rust_Unnamed_1::plumb_out.0 as ::core::ffi::c_int
        {
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_1::WHAT_KIND_OF_IDIOT.0 as ::core::ffi::c_int,
                None,
            ), ());
        } else if R as ::core::ffi::c_int
            == C2Rust_Unnamed_1::i_lost_my_lander.0 as ::core::ffi::c_int
        {
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_1::HERE_IS_A_NEW_LANDER.0 as ::core::ffi::c_int,
                None,
            ), ());
            GlobData.SIS_state.NumLanders = GlobData
                .SIS_state
                .NumLanders
                .wrapping_add(1);
            uqm_lua_checked!(DrawLanders(), ());
            uqm_lua_checked!(DeltaSISGauges(4 as SIZE, 0 as SDWORD, 0 as ::core::ffi::c_int), ());
            uqm_lua_checked!(setGameStateUint(
                b"LANDERS_LOST\0".as_ptr() as *const ::core::ffi::c_char,
                1 as DWORD,
            ), ());
        } else if R as ::core::ffi::c_int
            == C2Rust_Unnamed_1::i_lost_another_lander.0 as ::core::ffi::c_int
        {
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_1::HERE_IS_ANOTHER_LANDER.0 as ::core::ffi::c_int,
                None,
            ), ());
            GlobData.SIS_state.NumLanders = GlobData
                .SIS_state
                .NumLanders
                .wrapping_add(1);
            uqm_lua_checked!(DrawLanders(), ());
            uqm_lua_checked!(DeltaSISGauges(4 as SIZE, 0 as SDWORD, 0 as ::core::ffi::c_int), ());
        } else if R as ::core::ffi::c_int
            == C2Rust_Unnamed_1::need_fuel_mercury.0 as ::core::ffi::c_int
            || R as ::core::ffi::c_int
                == C2Rust_Unnamed_1::need_fuel_luna.0 as ::core::ffi::c_int
        {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::GIVE_FUEL.0 as ::core::ffi::c_int, None), ());
            uqm_lua_checked!(DeltaSISGauges(
                0 as SIZE,
                5 as SDWORD * 100 as SDWORD,
                0 as ::core::ffi::c_int,
            ), ());
            uqm_lua_checked!(setGameStateUint(
                b"GIVEN_FUEL_BEFORE\0".as_ptr() as *const ::core::ffi::c_char,
                1 as DWORD,
            ), ());
        } else if R as ::core::ffi::c_int
            == C2Rust_Unnamed_1::need_fuel_again.0 as ::core::ffi::c_int
        {
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_1::GIVE_FUEL_AGAIN.0 as ::core::ffi::c_int,
                None,
            ), ());
            uqm_lua_checked!(DeltaSISGauges(
                0 as SIZE,
                5 as SDWORD * 100 as SDWORD,
                0 as ::core::ffi::c_int,
            ), ());
        }
        if GlobData
            .SIS_state
            .ElementAmounts[C2Rust_Unnamed_0::RADIOACTIVE.0 as ::core::ffi::c_int
            as usize] != 0
        {
            uqm_lua_checked!(GiveRadios(0 as RESPONSE_REF), ());
        } else {
            if GlobData.SIS_state.NumLanders as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
                && uqm_lua_checked!(getGameStateUint(
                    b"CHMMR_BOMB_STATE\0".as_ptr() as *const ::core::ffi::c_char,
                ), ()) < 2 as DWORD
            {
                if uqm_lua_checked!(getGameStateUint(
                    b"LANDERS_LOST\0".as_ptr() as *const ::core::ffi::c_char,
                ), ()) != 0
                {
                    uqm_lua_checked!(DoResponsePhrase(
                        C2Rust_Unnamed_1::i_lost_another_lander.0 as ::core::ffi::c_int
                            as RESPONSE_REF,
                        Some(NoRadioactives as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                        ::core::ptr::null_mut::<UNICODE>(),
                    ), ());
                } else {
                    uqm_lua_checked!(DoResponsePhrase(
                        C2Rust_Unnamed_1::i_lost_my_lander.0 as ::core::ffi::c_int
                            as RESPONSE_REF,
                        Some(NoRadioactives as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                        ::core::ptr::null_mut::<UNICODE>(),
                    ), ());
                }
            }
            if GlobData.SIS_state.FuelOnBoard
                < (2 as ::core::ffi::c_int * 100 as ::core::ffi::c_int) as DWORD
            {
                if uqm_lua_checked!(getGameStateUint(
                    b"GIVEN_FUEL_BEFORE\0".as_ptr() as *const ::core::ffi::c_char,
                ), ()) != 0
                {
                    uqm_lua_checked!(DoResponsePhrase(
                        C2Rust_Unnamed_1::need_fuel_again.0 as ::core::ffi::c_int
                            as RESPONSE_REF,
                        Some(NoRadioactives as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                        ::core::ptr::null_mut::<UNICODE>(),
                    ), ());
                } else {
                    uqm_lua_checked!(DoResponsePhrase(
                        C2Rust_Unnamed_1::need_fuel_mercury.0 as ::core::ffi::c_int
                            as RESPONSE_REF,
                        Some(NoRadioactives as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                        ::core::ptr::null_mut::<UNICODE>(),
                    ), ());
                }
            }
            uqm_lua_checked!(DoResponsePhrase(
                C2Rust_Unnamed_1::ok_i_will_get_radios.0 as ::core::ffi::c_int
                    as RESPONSE_REF,
                Some(ByeBye as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            ), ());
            if *GetStringAddress(
                SetAbsStringTableIndex(
                    CommData.ConversationPhrases,
                    (C2Rust_Unnamed_1::where_can_i_get_radios.0 as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int) as COUNT,
                ),
            ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
            {
                uqm_lua_checked!(DoResponsePhrase(
                    C2Rust_Unnamed_1::where_can_i_get_radios.0 as ::core::ffi::c_int
                        as RESPONSE_REF,
                    Some(NoRadioactives as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                    ::core::ptr::null_mut::<UNICODE>(),
                ), ());
            }
        }
    };
}
unsafe extern "C" fn AskAfterRadios(mut R: RESPONSE_REF) {
    if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::i_lost_my_lander.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_1::HERE_IS_A_NEW_LANDER.0 as ::core::ffi::c_int,
            None,
        ), ());
        GlobData.SIS_state.NumLanders = GlobData.SIS_state.NumLanders.wrapping_add(1);
        uqm_lua_checked!(DrawLanders(), ());
        uqm_lua_checked!(DeltaSISGauges(4 as SIZE, 0 as SDWORD, 0 as ::core::ffi::c_int), ());
        uqm_lua_checked!(setGameStateUint(
            b"LANDERS_LOST\0".as_ptr() as *const ::core::ffi::c_char,
            1 as DWORD,
        ), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::i_lost_another_lander.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_1::HERE_IS_ANOTHER_LANDER.0 as ::core::ffi::c_int,
            None,
        ), ());
        GlobData.SIS_state.NumLanders = GlobData.SIS_state.NumLanders.wrapping_add(1);
        uqm_lua_checked!(DrawLanders(), ());
        uqm_lua_checked!(DeltaSISGauges(4 as SIZE, 0 as SDWORD, 0 as ::core::ffi::c_int), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::need_fuel_mercury.0 as ::core::ffi::c_int
        || R as ::core::ffi::c_int
            == C2Rust_Unnamed_1::need_fuel_luna.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::GIVE_FUEL.0 as ::core::ffi::c_int, None), ());
        uqm_lua_checked!(DeltaSISGauges(0 as SIZE, 5 as SDWORD * 100 as SDWORD, 0 as ::core::ffi::c_int), ());
        uqm_lua_checked!(setGameStateUint(
            b"GIVEN_FUEL_BEFORE\0".as_ptr() as *const ::core::ffi::c_char,
            1 as DWORD,
        ), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::need_fuel_again.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::GIVE_FUEL_AGAIN.0 as ::core::ffi::c_int, None), ());
        uqm_lua_checked!(DeltaSISGauges(0 as SIZE, 5 as SDWORD * 100 as SDWORD, 0 as ::core::ffi::c_int), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::where_get_radios.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::RADIOS_ON_MERCURY.0 as ::core::ffi::c_int, None), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_1::where_get_radios.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    }
    if GlobData.SIS_state.NumLanders as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        && uqm_lua_checked!(getGameStateUint(b"CHMMR_BOMB_STATE\0".as_ptr() as *const ::core::ffi::c_char), ())
            < 2 as DWORD
    {
        if uqm_lua_checked!(getGameStateUint(b"LANDERS_LOST\0".as_ptr() as *const ::core::ffi::c_char), ())
            != 0
        {
            uqm_lua_checked!(DoResponsePhrase(
                C2Rust_Unnamed_1::i_lost_another_lander.0 as ::core::ffi::c_int
                    as RESPONSE_REF,
                Some(AskAfterRadios as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            ), ());
        } else {
            uqm_lua_checked!(DoResponsePhrase(
                C2Rust_Unnamed_1::i_lost_my_lander.0 as ::core::ffi::c_int
                    as RESPONSE_REF,
                Some(AskAfterRadios as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            ), ());
        }
    }
    if GlobData.SIS_state.FuelOnBoard
        < (2 as ::core::ffi::c_int * 100 as ::core::ffi::c_int) as DWORD
    {
        if uqm_lua_checked!(getGameStateUint(
            b"GIVEN_FUEL_BEFORE\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) != 0
        {
            uqm_lua_checked!(DoResponsePhrase(
                C2Rust_Unnamed_1::need_fuel_again.0 as ::core::ffi::c_int
                    as RESPONSE_REF,
                Some(AskAfterRadios as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            ), ());
        } else {
            uqm_lua_checked!(DoResponsePhrase(
                C2Rust_Unnamed_1::need_fuel_mercury.0 as ::core::ffi::c_int
                    as RESPONSE_REF,
                Some(AskAfterRadios as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            ), ());
        }
    }
    uqm_lua_checked!(DoResponsePhrase(
        C2Rust_Unnamed_1::well_go_get_them_now.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(ByeBye as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    ), ());
    if *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_1::where_get_radios.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_1::where_get_radios.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(AskAfterRadios as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
}
unsafe extern "C" fn BaseDestroyed(mut R: RESPONSE_REF) {
    if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::we_fought_them.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_1::YOU_REALLY_FOUGHT_BASE.0 as ::core::ffi::c_int,
            None,
        ), ());
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_1::oh_yes_big_fight.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(ByeBye as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_1::i_lied_it_was_abandoned.0 as ::core::ffi::c_int
                as RESPONSE_REF,
            Some(ByeBye as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_1::i_cant_talk_about_it.0 as ::core::ffi::c_int
                as RESPONSE_REF,
            Some(ByeBye as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    } else {
        if R as ::core::ffi::c_int
            == C2Rust_Unnamed_1::we_are_here_to_help.0 as ::core::ffi::c_int
        {
            uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::BASE_ON_MOON.0 as ::core::ffi::c_int, None), ());
        } else {
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_1::DEALT_WITH_BASE_YET.0 as ::core::ffi::c_int,
                None,
            ), ());
        }
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_1::base_was_abandoned.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(ByeBye as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_1::we_fought_them.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(BaseDestroyed as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    };
}
unsafe extern "C" fn TellMoonBase(mut R: RESPONSE_REF) {
    if R as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_1::DEALT_WITH_BASE_YET.0 as ::core::ffi::c_int,
            None,
        ), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::i_lost_my_lander.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_1::HERE_IS_A_NEW_LANDER.0 as ::core::ffi::c_int,
            None,
        ), ());
        GlobData.SIS_state.NumLanders = GlobData.SIS_state.NumLanders.wrapping_add(1);
        uqm_lua_checked!(DrawLanders(), ());
        uqm_lua_checked!(DeltaSISGauges(4 as SIZE, 0 as SDWORD, 0 as ::core::ffi::c_int), ());
        uqm_lua_checked!(setGameStateUint(
            b"LANDERS_LOST\0".as_ptr() as *const ::core::ffi::c_char,
            1 as DWORD,
        ), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::i_lost_another_lander.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_1::HERE_IS_ANOTHER_LANDER.0 as ::core::ffi::c_int,
            None,
        ), ());
        GlobData.SIS_state.NumLanders = GlobData.SIS_state.NumLanders.wrapping_add(1);
        uqm_lua_checked!(DrawLanders(), ());
        uqm_lua_checked!(DeltaSISGauges(4 as SIZE, 0 as SDWORD, 0 as ::core::ffi::c_int), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::need_fuel_mercury.0 as ::core::ffi::c_int
        || R as ::core::ffi::c_int
            == C2Rust_Unnamed_1::need_fuel_luna.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::GIVE_FUEL.0 as ::core::ffi::c_int, None), ());
        uqm_lua_checked!(DeltaSISGauges(0 as SIZE, 5 as SDWORD * 100 as SDWORD, 0 as ::core::ffi::c_int), ());
        uqm_lua_checked!(setGameStateUint(
            b"GIVEN_FUEL_BEFORE\0".as_ptr() as *const ::core::ffi::c_char,
            1 as DWORD,
        ), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::need_fuel_again.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::GIVE_FUEL_AGAIN.0 as ::core::ffi::c_int, None), ());
        uqm_lua_checked!(DeltaSISGauges(0 as SIZE, 5 as SDWORD * 100 as SDWORD, 0 as ::core::ffi::c_int), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::we_are_here_to_help.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::BASE_ON_MOON.0 as ::core::ffi::c_int, None), ());
    } else if uqm_lua_checked!(getGameStateUint(
        b"STARBASE_YACK_STACK1\0".as_ptr() as *const ::core::ffi::c_char,
    ), ()) == 0 as DWORD
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::ABOUT_BASE.0 as ::core::ffi::c_int, None), ());
        uqm_lua_checked!(setGameStateUint(
            b"STARBASE_YACK_STACK1\0".as_ptr() as *const ::core::ffi::c_char,
            1 as DWORD,
        ), ());
    } else {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::ABOUT_BASE_AGAIN.0 as ::core::ffi::c_int, None), ());
    }
    if GlobData.SIS_state.NumLanders as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        && uqm_lua_checked!(getGameStateUint(b"CHMMR_BOMB_STATE\0".as_ptr() as *const ::core::ffi::c_char), ())
            < 2 as DWORD
    {
        if uqm_lua_checked!(getGameStateUint(b"LANDERS_LOST\0".as_ptr() as *const ::core::ffi::c_char), ())
            != 0
        {
            uqm_lua_checked!(DoResponsePhrase(
                C2Rust_Unnamed_1::i_lost_another_lander.0 as ::core::ffi::c_int
                    as RESPONSE_REF,
                Some(TellMoonBase as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            ), ());
        } else {
            uqm_lua_checked!(DoResponsePhrase(
                C2Rust_Unnamed_1::i_lost_my_lander.0 as ::core::ffi::c_int
                    as RESPONSE_REF,
                Some(TellMoonBase as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            ), ());
        }
    }
    if GlobData.SIS_state.FuelOnBoard
        < (2 as ::core::ffi::c_int * 100 as ::core::ffi::c_int) as DWORD
    {
        if uqm_lua_checked!(getGameStateUint(
            b"GIVEN_FUEL_BEFORE\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) != 0
        {
            uqm_lua_checked!(DoResponsePhrase(
                C2Rust_Unnamed_1::need_fuel_again.0 as ::core::ffi::c_int
                    as RESPONSE_REF,
                Some(TellMoonBase as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            ), ());
        } else {
            uqm_lua_checked!(DoResponsePhrase(
                C2Rust_Unnamed_1::need_fuel_luna.0 as ::core::ffi::c_int as RESPONSE_REF,
                Some(TellMoonBase as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            ), ());
        }
    }
    if uqm_lua_checked!(getGameStateUint(b"WILL_DESTROY_BASE\0".as_ptr() as *const ::core::ffi::c_char), ())
        == 0 as DWORD
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_1::we_will_take_care_of_base.0 as ::core::ffi::c_int
                as RESPONSE_REF,
            Some(ByeBye as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    } else {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_1::take_care_of_base_again.0 as ::core::ffi::c_int
                as RESPONSE_REF,
            Some(ByeBye as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    }
    if uqm_lua_checked!(getGameStateUint(b"STARBASE_YACK_STACK1\0".as_ptr() as *const ::core::ffi::c_char), ())
        == 0 as DWORD
    {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_1::tell_me_about_base.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(TellMoonBase as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    } else {
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_1::tell_me_again.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(TellMoonBase as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    };
}
unsafe extern "C" fn TellProbe(mut R: RESPONSE_REF) {
    uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::THAT_WAS_PROBE.0 as ::core::ffi::c_int, None), ());
    *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (C2Rust_Unnamed_1::what_was_red_thing.0 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) = '\0' as UNICODE;
    uqm_lua_checked!(DoResponsePhrase(
        C2Rust_Unnamed_1::it_went_away.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(RevealSelf as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    ), ());
    uqm_lua_checked!(DoResponsePhrase(
        C2Rust_Unnamed_1::we_destroyed_it.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(RevealSelf as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    ), ());
    uqm_lua_checked!(DoResponsePhrase(
        C2Rust_Unnamed_1::what_probe.0 as ::core::ffi::c_int as RESPONSE_REF,
        Some(RevealSelf as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    ), ());
}
unsafe extern "C" fn RevealSelf(mut R: RESPONSE_REF) {
    let mut i: BYTE = 0;
    let mut stack: BYTE = 0;
    stack = 0 as BYTE;
    if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::we_are_vindicator.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::THATS_IMPOSSIBLE.0 as ::core::ffi::c_int, None), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_1::we_are_vindicator.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::our_mission_was_secret.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::ACKNOWLEDGE_SECRET.0 as ::core::ffi::c_int, None), ());
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_1::our_mission_was_secret.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::first_give_info.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::ASK_AWAY.0 as ::core::ffi::c_int, None), ());
        stack = 1 as BYTE;
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_1::first_give_info.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::whats_this_starbase.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::STARBASE_IS.0 as ::core::ffi::c_int, None), ());
        stack = 1 as BYTE;
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_1::whats_this_starbase.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::what_about_earth.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::HAPPENED_TO_EARTH.0 as ::core::ffi::c_int, None), ());
        stack = 1 as BYTE;
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_1::what_about_earth.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::where_are_urquan.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::URQUAN_LEFT.0 as ::core::ffi::c_int, None), ());
        stack = 1 as BYTE;
        *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_1::where_are_urquan.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) = '\0' as UNICODE;
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::it_went_away.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::DEEP_TROUBLE.0 as ::core::ffi::c_int, None), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::we_destroyed_it.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::GOOD_NEWS.0 as ::core::ffi::c_int, None), ());
    } else if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::what_probe.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::SURE_HOPE.0 as ::core::ffi::c_int, None), ());
    }
    i = 0 as BYTE;
    while (i as ::core::ffi::c_int) < 2 as ::core::ffi::c_int {
        if stack as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            if *GetStringAddress(
                SetAbsStringTableIndex(
                    CommData.ConversationPhrases,
                    (C2Rust_Unnamed_1::first_give_info.0 as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int) as COUNT,
                ),
            ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
            {
                uqm_lua_checked!(DoResponsePhrase(
                    C2Rust_Unnamed_1::first_give_info.0 as ::core::ffi::c_int
                        as RESPONSE_REF,
                    Some(RevealSelf as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                    ::core::ptr::null_mut::<UNICODE>(),
                ), ());
            } else if *GetStringAddress(
                SetAbsStringTableIndex(
                    CommData.ConversationPhrases,
                    (C2Rust_Unnamed_1::whats_this_starbase.0 as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int) as COUNT,
                ),
            ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
            {
                uqm_lua_checked!(DoResponsePhrase(
                    C2Rust_Unnamed_1::whats_this_starbase.0 as ::core::ffi::c_int
                        as RESPONSE_REF,
                    Some(RevealSelf as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                    ::core::ptr::null_mut::<UNICODE>(),
                ), ());
            } else if *GetStringAddress(
                SetAbsStringTableIndex(
                    CommData.ConversationPhrases,
                    (C2Rust_Unnamed_1::what_about_earth.0 as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int) as COUNT,
                ),
            ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
            {
                uqm_lua_checked!(DoResponsePhrase(
                    C2Rust_Unnamed_1::what_about_earth.0 as ::core::ffi::c_int
                        as RESPONSE_REF,
                    Some(RevealSelf as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                    ::core::ptr::null_mut::<UNICODE>(),
                ), ());
            } else if *GetStringAddress(
                SetAbsStringTableIndex(
                    CommData.ConversationPhrases,
                    (C2Rust_Unnamed_1::where_are_urquan.0 as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int) as COUNT,
                ),
            ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
            {
                uqm_lua_checked!(DoResponsePhrase(
                    C2Rust_Unnamed_1::where_are_urquan.0 as ::core::ffi::c_int
                        as RESPONSE_REF,
                    Some(RevealSelf as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                    ::core::ptr::null_mut::<UNICODE>(),
                ), ());
            } else if *GetStringAddress(
                SetAbsStringTableIndex(
                    CommData.ConversationPhrases,
                    (C2Rust_Unnamed_1::what_was_red_thing.0 as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int) as COUNT,
                ),
            ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
            {
                uqm_lua_checked!(DoResponsePhrase(
                    C2Rust_Unnamed_1::what_was_red_thing.0 as ::core::ffi::c_int
                        as RESPONSE_REF,
                    Some(TellProbe as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                    ::core::ptr::null_mut::<UNICODE>(),
                ), ());
            }
        } else if *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_1::we_are_vindicator.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        {
            uqm_lua_checked!(DoResponsePhrase(
                C2Rust_Unnamed_1::we_are_vindicator.0 as ::core::ffi::c_int
                    as RESPONSE_REF,
                Some(RevealSelf as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            ), ());
        } else if *GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (C2Rust_Unnamed_1::our_mission_was_secret.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        {
            uqm_lua_checked!(DoResponsePhrase(
                C2Rust_Unnamed_1::our_mission_was_secret.0 as ::core::ffi::c_int
                    as RESPONSE_REF,
                Some(RevealSelf as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            ), ());
        } else if uqm_lua_checked!(getGameStateUint(
            b"MOONBASE_DESTROYED\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) == 0 as DWORD
        {
            uqm_lua_checked!(DoResponsePhrase(
                C2Rust_Unnamed_1::we_are_here_to_help.0 as ::core::ffi::c_int
                    as RESPONSE_REF,
                Some(TellMoonBase as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            ), ());
        } else {
            uqm_lua_checked!(DoResponsePhrase(
                C2Rust_Unnamed_1::we_are_here_to_help.0 as ::core::ffi::c_int
                    as RESPONSE_REF,
                Some(BaseDestroyed as unsafe extern "C" fn(RESPONSE_REF) -> ()),
                ::core::ptr::null_mut::<UNICODE>(),
            ), ());
        }
        i = i.wrapping_add(1);
        stack = (stack as ::core::ffi::c_int ^ 1 as ::core::ffi::c_int) as BYTE;
    }
}
unsafe extern "C" fn GiveRadios(mut R: RESPONSE_REF) {
    if R as ::core::ffi::c_int
        == C2Rust_Unnamed_1::we_will_transfer_now.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(setGameStateUint(
            b"RADIOACTIVES_PROVIDED\0".as_ptr() as *const ::core::ffi::c_char,
            1 as DWORD,
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::FUEL_UP0.0 as ::core::ffi::c_int, None), ());
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::FUEL_UP1.0 as ::core::ffi::c_int, None), ());
        uqm_lua_checked!(AlienTalkSegue(1 as COUNT), ());
        CommData.AlienAmbientArray[2usize].AnimFlags = (CommData
            .AlienAmbientArray[2usize]
            .AnimFlags as ::core::ffi::c_long
            | (1 as ::core::ffi::c_long) << 8 as ::core::ffi::c_int) as COUNT;
        if if resolutionFactor != 2 as ::core::ffi::c_uint {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } != 0
            && if GlobData.SIS_state.Extended as ::core::ffi::c_int != 0 {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
        {
            CommData.AlienFrame = SetAbsFrameIndex(CommData.AlienFrame, 0 as COUNT);
            CommData.AlienAmbientArray[0usize].AnimFlags = (CommData
                .AlienAmbientArray[0usize]
                .AnimFlags as ::core::ffi::c_long
                & !((1 as ::core::ffi::c_long) << 8 as ::core::ffi::c_int)) as COUNT;
            CommData.AlienAmbientArray[1usize].AnimFlags = (CommData
                .AlienAmbientArray[1usize]
                .AnimFlags as ::core::ffi::c_long
                & !((1 as ::core::ffi::c_long) << 8 as ::core::ffi::c_int)) as COUNT;
            CommData.AlienAmbientArray[3usize].AnimFlags = (CommData
                .AlienAmbientArray[3usize]
                .AnimFlags as ::core::ffi::c_long
                | (1 as ::core::ffi::c_long) << 8 as ::core::ffi::c_int) as COUNT;
            EnableTalkingAnim(BOOLEAN::TRUE);
            EngageFilters(&raw mut commander_filters);
            FilterData.FilterArray[0usize].Flags = ((1 as ::core::ffi::c_int)
                << 0 as ::core::ffi::c_int) as BYTE;
            FilterData.FilterArray[1usize].Flags = (FilterData.FilterArray[1usize].Flags
                as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as BYTE;
        }
        if if resolutionFactor != 2 as ::core::ffi::c_uint {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } != 0
            && if GlobData.SIS_state.Extended as ::core::ffi::c_int != 0 {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } == 0
        {
            FilterData.FilterArray[0usize].Flags = ((1 as ::core::ffi::c_int)
                << 0 as ::core::ffi::c_int) as BYTE;
            FilterData.FilterArray[1usize].Flags = ((1 as ::core::ffi::c_int)
                << 3 as ::core::ffi::c_int) as BYTE;
        }
        XFormColorMap(
            GetColorMapAddress(
                SetAbsStringTableIndex(CommData.AlienColorMap, 0 as COUNT),
            ),
            (840 as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as SIZE,
        );
        if altResFlags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int != 0
        {
            let mut MusicPos: DWORD = uqm_lua_checked!(PLRGetPos(), ());
            uqm_lua_checked!(StopMusic(), ());
            CommData.AlienSong = uqm_lua_checked!(LoadMusicInstance(CommData.AlienSongRes), ());
            uqm_lua_checked!(SetMusicVolume(0 as COUNT), ());
            uqm_lua_checked!(PlayMusic(CommData.AlienSong, BOOLEAN::TRUE, 1 as BYTE), ());
            uqm_lua_checked!(SeekMusic(MusicPos), ());
            uqm_lua_checked!(FadeMusic(160 as BYTE, 840 as SIZE), ());
            altResFlags = (altResFlags as ::core::ffi::c_int
                & !((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)) as BYTE;
        }
        uqm_lua_checked!(AlienTalkSegue(!(0 as ::core::ffi::c_int) as COUNT), ());
        uqm_lua_checked!(RevealSelf(0 as RESPONSE_REF), ());
    } else {
        if R as ::core::ffi::c_int
            == C2Rust_Unnamed_1::what_will_you_give_us.0 as ::core::ffi::c_int
        {
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_1::MESSAGE_GARBLED_1.0 as ::core::ffi::c_int,
                None,
            ), ());
        } else if R as ::core::ffi::c_int
            == C2Rust_Unnamed_1::before_radios_we_need_info.0 as ::core::ffi::c_int
        {
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_1::MESSAGE_GARBLED_2.0 as ::core::ffi::c_int,
                None,
            ), ());
        }
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_1::we_will_transfer_now.0 as ::core::ffi::c_int
                as RESPONSE_REF,
            Some(GiveRadios as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_1::what_will_you_give_us.0 as ::core::ffi::c_int
                as RESPONSE_REF,
            Some(GiveRadios as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_1::before_radios_we_need_info.0 as ::core::ffi::c_int
                as RESPONSE_REF,
            Some(GiveRadios as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    };
}
unsafe extern "C" fn Intro() {
    if if resolutionFactor != 2 as ::core::ffi::c_uint {
        BOOLEAN::FALSE.0 as ::core::ffi::c_int
    } else {
        BOOLEAN::TRUE.0 as ::core::ffi::c_int
    } != 0
    {
        CommData.AlienAmbientArray[0usize].BaseFrameRate = (840 as ::core::ffi::c_int
            / 40 as ::core::ffi::c_int) as COUNT;
    }
    if uqm_lua_checked!(getGameStateUint(
        b"PROBE_ILWRATH_ENCOUNTER\0".as_ptr() as *const ::core::ffi::c_char,
    ), ()) != 0
    {
        uqm_lua_checked!(NPCPhrase_cb(C2Rust_Unnamed_1::VERY_IMPRESSIVE.0 as ::core::ffi::c_int, None), ());
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_1::cook_their_butts.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(ByeBye as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_1::overthrow_evil_aliens.0 as ::core::ffi::c_int
                as RESPONSE_REF,
            Some(ByeBye as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_1::annihilate_those_monsters.0 as ::core::ffi::c_int
                as RESPONSE_REF,
            Some(ByeBye as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    } else if uqm_lua_checked!(getGameStateUint(
        b"STARBASE_VISITED\0".as_ptr() as *const ::core::ffi::c_char,
    ), ()) != 0
    {
        if uqm_lua_checked!(getGameStateUint(
            b"RADIOACTIVES_PROVIDED\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) != 0
        {
            if uqm_lua_checked!(getGameStateUint(
                b"MOONBASE_DESTROYED\0".as_ptr() as *const ::core::ffi::c_char,
            ), ()) == 0 as DWORD
            {
                uqm_lua_checked!(TellMoonBase(0 as RESPONSE_REF), ());
            } else {
                uqm_lua_checked!(BaseDestroyed(0 as RESPONSE_REF), ());
            }
        } else {
            RaveParty();
            uqm_lua_checked!(NPCPhrase_cb(
                C2Rust_Unnamed_1::DO_YOU_HAVE_RADIO_THIS_TIME.0 as ::core::ffi::c_int,
                None,
            ), ());
            if GlobData
                .SIS_state
                .ElementAmounts[C2Rust_Unnamed_0::RADIOACTIVE.0 as ::core::ffi::c_int
                as usize] != 0
            {
                uqm_lua_checked!(GiveRadios(0 as RESPONSE_REF), ());
            } else {
                uqm_lua_checked!(AskAfterRadios(0 as RESPONSE_REF), ());
            }
        }
    } else {
        RaveParty();
        uqm_lua_checked!(setGameStateUint(
            b"STARBASE_VISITED\0".as_ptr() as *const ::core::ffi::c_char,
            1 as DWORD,
        ), ());
        uqm_lua_checked!(NPCPhrase_cb(
            C2Rust_Unnamed_1::ARE_YOU_SUPPLY_SHIP.0 as ::core::ffi::c_int,
            None,
        ), ());
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_1::no_but_well_help.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(NoRadioactives as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_1::yes_this_is_supply_ship.0 as ::core::ffi::c_int
                as RESPONSE_REF,
            Some(NoRadioactives as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
        uqm_lua_checked!(DoResponsePhrase(
            C2Rust_Unnamed_1::what_slave_planet.0 as ::core::ffi::c_int as RESPONSE_REF,
            Some(NoRadioactives as unsafe extern "C" fn(RESPONSE_REF) -> ()),
            ::core::ptr::null_mut::<UNICODE>(),
        ), ());
    };
}
unsafe extern "C" fn uninit_commander() -> COUNT {
    luaUqm_comm_uninit();
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
    uqm_lua_checked!(luaUqm_comm_init(
        ::core::ptr::null::<luaUqm_custom_Function>(),
        ::core::ptr::null::<::core::ffi::c_char>(),
    ), ::core::ptr::null_mut());
    if uqm_lua_checked!(getGameStateUint(
        b"RADIOACTIVES_PROVIDED\0".as_ptr() as *const ::core::ffi::c_char,
    ), ::core::ptr::null_mut()) == 0
    {
        altResFlags = (altResFlags as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as BYTE;
    }
    if if resolutionFactor != 2 as ::core::ffi::c_uint {
        BOOLEAN::FALSE.0 as ::core::ffi::c_int
    } else {
        BOOLEAN::TRUE.0 as ::core::ffi::c_int
    } != 0
    {
        altResFlags = (altResFlags as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as BYTE;
    }
    commander_desc.AlienTextWidth = ((143 as ::core::ffi::c_int) << resolutionFactor)
        as COUNT;
    commander_desc.AlienTextBaseline.x = (((164 as ::core::ffi::c_int)
        << resolutionFactor)
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
            } << resolutionFactor) - ((2 as ::core::ffi::c_int) << resolutionFactor)
        } else {
            0 as ::core::ffi::c_int
        } * 2 as ::core::ffi::c_int) as COORD;
    commander_desc.AlienTextBaseline.y = ((20 as ::core::ffi::c_int) << resolutionFactor)
        as COORD;
    uqm_lua_checked!(setSegue(Segue::Segue_peace), ::core::ptr::null_mut());
    retval = &raw mut commander_desc;
    return retval;
}
