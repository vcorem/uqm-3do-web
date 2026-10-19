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
 *  Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA  02111-1307  USA
 */

/*
 * Right now, the functions to call as a callback for responses are stored
 * in the Lua registry. This could however also be handled in Lua code
 * itself: one generic Lua function which is called after a response, which
 * then calls the appropriate function which was registered for the response
 * in Lua code.
 */

//#define DEBUG_STARSEED

// Rust translation of src/uqm/lua/luafuncs/commfuncs.c.
extern "C" {
    pub type lua_State;
    pub type _IO_FILE;
    pub type string_table_entry;
    pub type frame_desc;
    pub type font_desc;
    pub type tfb_soundsample;
    #[link_name = "uqm_lua_guard_lua_gettop"]
    fn lua_gettop(L: *mut lua_State) -> ::core::ffi::c_int;
    #[link_name = "uqm_lua_guard_lua_settop"]
    fn lua_settop(L: *mut lua_State, idx: ::core::ffi::c_int);
    #[link_name = "uqm_lua_guard_lua_pushvalue"]
    fn lua_pushvalue(L: *mut lua_State, idx: ::core::ffi::c_int);
    #[link_name = "uqm_lua_guard_lua_replace"]
    fn lua_replace(L: *mut lua_State, idx: ::core::ffi::c_int);
    #[link_name = "uqm_lua_guard_lua_tolstring"]
    fn lua_tolstring(
        L: *mut lua_State,
        idx: ::core::ffi::c_int,
        len: *mut size_t,
    ) -> *const ::core::ffi::c_char;
    #[link_name = "uqm_lua_guard_lua_pushnil"]
    fn lua_pushnil(L: *mut lua_State);
    #[link_name = "uqm_lua_guard_lua_pushinteger"]
    fn lua_pushinteger(L: *mut lua_State, n: lua_Integer);
    #[link_name = "uqm_lua_guard_lua_pushstring"]
    fn lua_pushstring(
        L: *mut lua_State,
        s: *const ::core::ffi::c_char,
    ) -> *const ::core::ffi::c_char;
    #[link_name = "uqm_lua_guard_lua_pushboolean"]
    fn lua_pushboolean(L: *mut lua_State, b: ::core::ffi::c_int);
    #[link_name = "uqm_lua_guard_lua_gettable"]
    fn lua_gettable(L: *mut lua_State, idx: ::core::ffi::c_int);
    #[link_name = "uqm_lua_guard_lua_getfield"]
    fn lua_getfield(
        L: *mut lua_State,
        idx: ::core::ffi::c_int,
        k: *const ::core::ffi::c_char,
    );
    #[link_name = "uqm_lua_guard_lua_createtable"]
    fn lua_createtable(
        L: *mut lua_State,
        narr: ::core::ffi::c_int,
        nrec: ::core::ffi::c_int,
    );
    #[link_name = "uqm_lua_guard_lua_settable"]
    fn lua_settable(L: *mut lua_State, idx: ::core::ffi::c_int);
    #[link_name = "uqm_lua_guard_lua_setfield"]
    fn lua_setfield(
        L: *mut lua_State,
        idx: ::core::ffi::c_int,
        k: *const ::core::ffi::c_char,
    );
    #[link_name = "uqm_lua_guard_lua_pcallk"]
    fn lua_pcallk(
        L: *mut lua_State,
        nargs: ::core::ffi::c_int,
        nresults: ::core::ffi::c_int,
        errfunc: ::core::ffi::c_int,
        ctx: ::core::ffi::c_int,
        k: lua_CFunction,
    ) -> ::core::ffi::c_int;
    static stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn snprintf(
        _: *mut ::core::ffi::c_char,
        _: size_t,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    #[link_name = "uqm_lua_guard_luaL_checklstring"]
    fn luaL_checklstring(
        L: *mut lua_State,
        numArg: ::core::ffi::c_int,
        l: *mut size_t,
    ) -> *const ::core::ffi::c_char;
    #[link_name = "uqm_lua_guard_luaL_checkinteger"]
    fn luaL_checkinteger(L: *mut lua_State, numArg: ::core::ffi::c_int) -> lua_Integer;
    #[link_name = "uqm_lua_guard_luaL_checktype"]
    fn luaL_checktype(
        L: *mut lua_State,
        narg: ::core::ffi::c_int,
        t: ::core::ffi::c_int,
    );
    #[link_name = "uqm_lua_guard_luaL_checkoption"]
    fn luaL_checkoption(
        L: *mut lua_State,
        narg: ::core::ffi::c_int,
        def: *const ::core::ffi::c_char,
        lst: *const *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    #[link_name = "uqm_lua_guard_luaL_setfuncs"]
    fn luaL_setfuncs(L: *mut lua_State, l: *const luaL_Reg, nup: ::core::ffi::c_int);
    fn luaUqm_makeEnum(luaState: *mut lua_State, enumVals: *const luaUqm_EnumValue);
    fn HFree(p: *mut ::core::ffi::c_void);
    fn SetAbsStringTableIndex(String: STRING, StringTableIndex: COUNT) -> STRING;
    fn GetStringAddress(String: STRING) -> STRINGPTR;
    static mut RoboTrack: [COUNT; 0];
    fn log_add(_: log_Level, fmt: *const ::core::ffi::c_char, ...);
    fn luaUqm_comm_stringNeedsInterpolate(str: *const ::core::ffi::c_char) -> BOOLEAN;
    fn luaUqm_comm_stringInterpolate(
        str: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    static mut luaUqm_commState: *mut lua_State;
    fn islower(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn isupper(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn toupper(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    static mut optSeedType: ::core::ffi::c_int;
    static mut GlobData: GLOBDATA;
    static mut CommData: LOCDATA;
    fn DoResponsePhrase(
        R: RESPONSE_REF,
        response_func: RESPONSE_FUNC,
        ContstructStr: *mut UNICODE,
    );
    fn NPCPhrase_cb(index: ::core::ffi::c_int, cb: CallbackFunction);
    fn setSegue(segue: Segue);
    fn getSegue() -> Segue;
    fn phraseIdStrToNum(phraseId: *const ::core::ffi::c_char) -> RESPONSE_REF;
    fn phraseIdNumToStr(response: RESPONSE_REF) -> *const ::core::ffi::c_char;
    fn SetCustomBaseLine(sentence: COUNT, bl: POINT, align: TEXT_ALIGN);
    static mut star_array: [STAR_DESC; 0];
    static mut plot_map: [PLOT_LOCATION; 70];
    fn GetClusterName(pSD: *const STAR_DESC, buf: *mut UNICODE);
    fn FindNearestConstellation(starmap: *mut STAR_DESC, p: POINT) -> *mut STAR_DESC;
    fn PlotIdStrToIndex(plotIdStr: *const ::core::ffi::c_char) -> COUNT;
    static mut GameStrings: STRING;
}
pub type size_t = ::core::ffi::c_ulong;
pub type ptrdiff_t = isize;
pub type lua_CFunction = Option<
    unsafe extern "C" fn(*mut lua_State) -> ::core::ffi::c_int,
>;
pub type lua_Integer = ptrdiff_t;
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct luaL_Reg {
    pub name: *const ::core::ffi::c_char,
    pub func: lua_CFunction,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct luaUqm_EnumValue {
    pub name: *const ::core::ffi::c_char,
    pub value: ::core::ffi::c_int,
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
pub type RESOURCE = *const ::core::ffi::c_char;
pub type STRING_TABLE_ENTRY_DESC = string_table_entry;
pub type STRING = *mut STRING_TABLE_ENTRY_DESC;
pub type STRINGPTR = *mut ::core::ffi::c_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Color {
    pub r: BYTE,
    pub g: BYTE,
    pub b: BYTE,
    pub a: BYTE,
}
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
pub type PLOT_LOCATION = plot_location;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct plot_location {
    pub star_pt: POINT,
    pub star: *mut STAR_DESC,
    pub dist_sq: [DWORD; 70],
}
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
pub type RESPONSE_REF = COUNT;
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
pub type MUSIC_REF = *mut *mut TFB_SoundSample;
pub type TFB_SoundSample = tfb_soundsample;
pub type NUMBER_SPEECH = *const NUMBER_SPEECH_DESC;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NUMBER_SPEECH_DESC {
    pub NumDigits: COUNT,
    pub Digits: [SPEECH_DIGIT; 7],
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
pub struct SPEECH_DIGITNAME {
    pub Divider: ::core::ffi::c_int,
    pub MaxRemainder: ::core::ffi::c_int,
    pub StrIndex: COUNT,
}
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
pub struct ALT_RESOURCE {
    pub AlienFrameRes: RESOURCE,
    pub AlienColorMapRes: RESOURCE,
    pub AlienSongRes: RESOURCE,
}
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
pub struct GLOBDATA {
    pub SIS_state: SIS_STATE,
    pub Game_state: GAME_STATE,
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
pub type CallbackFunction = Option<unsafe extern "C" fn(CallbackArg) -> ()>;
pub type CallbackArg = *mut ::core::ffi::c_void;
pub type RESPONSE_FUNC = Option<unsafe extern "C" fn(RESPONSE_REF) -> ()>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed {
    pub const OPTVAL_PRIME: Self = Self(0);
    pub const OPTVAL_PLANET: Self = Self(1);
    pub const OPTVAL_MRQ: Self = Self(2);
    pub const OPTVAL_STAR: Self = Self(3);
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_1(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_1 {
    pub const ROBOT_NULL_PHRASE: Self = Self(0);
    pub const ROBOT_DIGIT_0: Self = Self(1);
    pub const ROBOT_DIGIT_1: Self = Self(2);
    pub const ROBOT_DIGIT_2: Self = Self(3);
    pub const ROBOT_DIGIT_3: Self = Self(4);
    pub const ROBOT_DIGIT_4: Self = Self(5);
    pub const ROBOT_DIGIT_5: Self = Self(6);
    pub const ROBOT_DIGIT_6: Self = Self(7);
    pub const ROBOT_DIGIT_7: Self = Self(8);
    pub const ROBOT_DIGIT_8: Self = Self(9);
    pub const ROBOT_DIGIT_9: Self = Self(10);
    pub const ROBOT_POINT: Self = Self(11);
    pub const ROBOT_BY: Self = Self(12);
    pub const ROBOT_COLOR_RED: Self = Self(13);
    pub const ROBOT_COLOR_ORANGE: Self = Self(14);
    pub const ROBOT_COLOR_YELLOW: Self = Self(15);
    pub const ROBOT_COLOR_GREEN: Self = Self(16);
    pub const ROBOT_COLOR_BLUE: Self = Self(17);
    pub const ROBOT_COLOR_WHITE: Self = Self(18);
    pub const ILWRATH_COLOR_RED: Self = Self(19);
    pub const ILWRATH_COLOR_ORANGE: Self = Self(20);
    pub const ILWRATH_COLOR_YELLOW: Self = Self(21);
    pub const ILWRATH_COLOR_GREEN: Self = Self(22);
    pub const ILWRATH_COLOR_BLUE: Self = Self(23);
    pub const ILWRATH_COLOR_WHITE: Self = Self(24);
    pub const ROBOT_PREFIX_0: Self = Self(25);
    pub const ROBOT_ALPHA: Self = Self(26);
    pub const ROBOT_BETA: Self = Self(27);
    pub const ROBOT_GAMMA: Self = Self(28);
    pub const ROBOT_DELTA: Self = Self(29);
    pub const ROBOT_EPSILON: Self = Self(30);
    pub const ROBOT_ZETA: Self = Self(31);
    pub const ROBOT_ETA: Self = Self(32);
    pub const ROBOT_THETA: Self = Self(33);
    pub const ROBOT_IOTA: Self = Self(34);
    pub const ROBOT_KAPPA: Self = Self(35);
    pub const ROBOT_LAMBDA: Self = Self(36);
    pub const ROBOT_MU: Self = Self(37);
    pub const ROBOT_NU: Self = Self(38);
    pub const ROBOT_XI: Self = Self(39);
    pub const ROBOT_POSTFIX_0: Self = Self(40);
    pub const ROBOT_POSTFIX_1: Self = Self(41);
    pub const ROBOT_POSTFIX_2: Self = Self(42);
    pub const ROBOT_POSTFIX_3: Self = Self(43);
    pub const ROBOT_POSTFIX_4: Self = Self(44);
    pub const ROBOT_POSTFIX_5: Self = Self(45);
    pub const ROBOT_POSTFIX_6: Self = Self(46);
    pub const ROBOT_POSTFIX_7: Self = Self(47);
    pub const ROBOT_POSTFIX_8: Self = Self(48);
    pub const ROBOT_POSTFIX_9: Self = Self(49);
    pub const ROBOT_POSTFIX_10: Self = Self(50);
    pub const ROBOT_POSTFIX_11: Self = Self(51);
    pub const ROBOT_POSTFIX_12: Self = Self(52);
    pub const ROBOT_POSTFIX_13: Self = Self(53);
    pub const ROBOT_POSTFIX_14: Self = Self(54);
    pub const ROBOT_POSTFIX_15: Self = Self(55);
    pub const ROBOT_POSTFIX_16: Self = Self(56);
    pub const ROBOT_POSTFIX_17: Self = Self(57);
    pub const ROBOT_POSTFIX_18: Self = Self(58);
    pub const ROBOT_POSTFIX_19: Self = Self(59);
    pub const ROBOT_POSTFIX_20: Self = Self(60);
    pub const ROBOT_POSTFIX_21: Self = Self(61);
    pub const ROBOT_POSTFIX_22: Self = Self(62);
    pub const ROBOT_POSTFIX_23: Self = Self(63);
    pub const ROBOT_POSTFIX_24: Self = Self(64);
    pub const ROBOT_POSTFIX_25: Self = Self(65);
    pub const ROBOT_POSTFIX_26: Self = Self(66);
    pub const ROBOT_POSTFIX_27: Self = Self(67);
    pub const ROBOT_POSTFIX_28: Self = Self(68);
    pub const ROBOT_POSTFIX_29: Self = Self(69);
    pub const ROBOT_POSTFIX_30: Self = Self(70);
    pub const ROBOT_POSTFIX_31: Self = Self(71);
    pub const ROBOT_POSTFIX_32: Self = Self(72);
    pub const ROBOT_POSTFIX_33: Self = Self(73);
    pub const ROBOT_POSTFIX_34: Self = Self(74);
    pub const ROBOT_POSTFIX_35: Self = Self(75);
    pub const ROBOT_POSTFIX_36: Self = Self(76);
    pub const ROBOT_POSTFIX_37: Self = Self(77);
    pub const ROBOT_POSTFIX_38: Self = Self(78);
    pub const ROBOT_POSTFIX_39: Self = Self(79);
    pub const ROBOT_POSTFIX_40: Self = Self(80);
    pub const ROBOT_POSTFIX_41: Self = Self(81);
    pub const ROBOT_POSTFIX_42: Self = Self(82);
    pub const ROBOT_POSTFIX_43: Self = Self(83);
    pub const ROBOT_POSTFIX_44: Self = Self(84);
    pub const ROBOT_POSTFIX_45: Self = Self(85);
    pub const ROBOT_POSTFIX_46: Self = Self(86);
    pub const ROBOT_POSTFIX_47: Self = Self(87);
    pub const ROBOT_POSTFIX_48: Self = Self(88);
    pub const ROBOT_POSTFIX_49: Self = Self(89);
    pub const ROBOT_POSTFIX_50: Self = Self(90);
    pub const ROBOT_POSTFIX_51: Self = Self(91);
    pub const ROBOT_POSTFIX_52: Self = Self(92);
    pub const ROBOT_POSTFIX_53: Self = Self(93);
    pub const ROBOT_POSTFIX_54: Self = Self(94);
    pub const ROBOT_POSTFIX_55: Self = Self(95);
    pub const ROBOT_POSTFIX_56: Self = Self(96);
    pub const ROBOT_POSTFIX_57: Self = Self(97);
    pub const ROBOT_POSTFIX_58: Self = Self(98);
    pub const ROBOT_POSTFIX_59: Self = Self(99);
    pub const ROBOT_POSTFIX_60: Self = Self(100);
    pub const ROBOT_POSTFIX_61: Self = Self(101);
    pub const ROBOT_POSTFIX_62: Self = Self(102);
    pub const ROBOT_POSTFIX_63: Self = Self(103);
    pub const ROBOT_POSTFIX_64: Self = Self(104);
    pub const ROBOT_POSTFIX_65: Self = Self(105);
    pub const ROBOT_POSTFIX_66: Self = Self(106);
    pub const ROBOT_POSTFIX_67: Self = Self(107);
    pub const ROBOT_POSTFIX_68: Self = Self(108);
    pub const ROBOT_POSTFIX_69: Self = Self(109);
    pub const ROBOT_POSTFIX_70: Self = Self(110);
    pub const ROBOT_POSTFIX_71: Self = Self(111);
    pub const ROBOT_POSTFIX_72: Self = Self(112);
    pub const ROBOT_POSTFIX_73: Self = Self(113);
    pub const ROBOT_POSTFIX_74: Self = Self(114);
    pub const ROBOT_POSTFIX_75: Self = Self(115);
    pub const ROBOT_POSTFIX_76: Self = Self(116);
    pub const ROBOT_POSTFIX_77: Self = Self(117);
    pub const ROBOT_POSTFIX_78: Self = Self(118);
    pub const ROBOT_POSTFIX_79: Self = Self(119);
    pub const ROBOT_POSTFIX_80: Self = Self(120);
    pub const ROBOT_POSTFIX_81: Self = Self(121);
    pub const ROBOT_POSTFIX_82: Self = Self(122);
    pub const ROBOT_POSTFIX_83: Self = Self(123);
    pub const ROBOT_POSTFIX_84: Self = Self(124);
    pub const ROBOT_POSTFIX_85: Self = Self(125);
    pub const ROBOT_POSTFIX_86: Self = Self(126);
    pub const ROBOT_POSTFIX_87: Self = Self(127);
    pub const ROBOT_POSTFIX_88: Self = Self(128);
    pub const ROBOT_POSTFIX_89: Self = Self(129);
    pub const ROBOT_POSTFIX_90: Self = Self(130);
    pub const ROBOT_POSTFIX_91: Self = Self(131);
    pub const ROBOT_POSTFIX_92: Self = Self(132);
    pub const ROBOT_POSTFIX_93: Self = Self(133);
    pub const ROBOT_POSTFIX_94: Self = Self(134);
    pub const ROBOT_POSTFIX_95: Self = Self(135);
    pub const ROBOT_POSTFIX_96: Self = Self(136);
    pub const ROBOT_POSTFIX_97: Self = Self(137);
    pub const ROBOT_POSTFIX_98: Self = Self(138);
    pub const ROBOT_POSTFIX_99: Self = Self(139);
    pub const ROBOT_POSTFIX_100: Self = Self(140);
    pub const ROBOT_POSTFIX_101: Self = Self(141);
    pub const ROBOT_POSTFIX_102: Self = Self(142);
    pub const ROBOT_POSTFIX_103: Self = Self(143);
    pub const ROBOT_POSTFIX_104: Self = Self(144);
    pub const ROBOT_POSTFIX_105: Self = Self(145);
    pub const ROBOT_POSTFIX_106: Self = Self(146);
    pub const ROBOT_POSTFIX_107: Self = Self(147);
    pub const ROBOT_POSTFIX_108: Self = Self(148);
    pub const ROBOT_POSTFIX_109: Self = Self(149);
    pub const ROBOT_POSTFIX_110: Self = Self(150);
    pub const ROBOT_POSTFIX_111: Self = Self(151);
    pub const ROBOT_POSTFIX_112: Self = Self(152);
    pub const ROBOT_POSTFIX_113: Self = Self(153);
    pub const ROBOT_POSTFIX_114: Self = Self(154);
    pub const ROBOT_POSTFIX_115: Self = Self(155);
    pub const ROBOT_POSTFIX_116: Self = Self(156);
    pub const ROBOT_POSTFIX_117: Self = Self(157);
    pub const ROBOT_POSTFIX_118: Self = Self(158);
    pub const ROBOT_POSTFIX_119: Self = Self(159);
    pub const ROBOT_POSTFIX_120: Self = Self(160);
    pub const ROBOT_POSTFIX_121: Self = Self(161);
    pub const ROBOT_POSTFIX_122: Self = Self(162);
    pub const ROBOT_POSTFIX_123: Self = Self(163);
    pub const ROBOT_POSTFIX_124: Self = Self(164);
    pub const ROBOT_POSTFIX_125: Self = Self(165);
    pub const ROBOT_POSTFIX_126: Self = Self(166);
    pub const ROBOT_POSTFIX_127: Self = Self(167);
    pub const ROBOT_POSTFIX_128: Self = Self(168);
    pub const ROBOT_POSTFIX_129: Self = Self(169);
    pub const ROBOT_POSTFIX_130: Self = Self(170);
    pub const ROBOT_POSTFIX_131: Self = Self(171);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_2(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_2 {
    pub const BLUE_BODY: Self = Self(0);
    pub const GREEN_BODY: Self = Self(1);
    pub const ORANGE_BODY: Self = Self(2);
    pub const RED_BODY: Self = Self(3);
    pub const WHITE_BODY: Self = Self(4);
    pub const GRAY_BODY: Self = Self(4);
    pub const YELLOW_BODY: Self = Self(5);
    pub const NUM_STAR_COLORS: Self = Self(6);
    pub const CYAN_BODY: Self = Self(6);
    pub const PURPLE_BODY: Self = Self(7);
    pub const VIOLET_BODY: Self = Self(8);
    pub const NUM_PLANET_COLORS: Self = Self(9);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_3(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_3 {
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
static mut npcPhraseCallbackRegistryKey: [::core::ffi::c_char; 27] = unsafe {
    ::core::mem::transmute::<
        [u8; 27],
        [::core::ffi::c_char; 27],
    >(*b"uqm_comm_npcPhraseCallback\0")
};
static mut responseCallbackRegistryKey: [::core::ffi::c_char; 26] = unsafe {
    ::core::mem::transmute::<
        [u8; 26],
        [::core::ffi::c_char; 26],
    >(*b"uqm_comm_responseCallback\0")
};
static mut commFuncs: [luaL_Reg; 16] = [
    luaL_Reg {
        name: b"addResponse\0".as_ptr() as *const ::core::ffi::c_char,
        func: Some(
            luaUqm_comm_addResponse
                as unsafe extern "C" fn(*mut lua_State) -> ::core::ffi::c_int,
        ),
    },
    luaL_Reg {
        name: b"disablePhrase\0".as_ptr() as *const ::core::ffi::c_char,
        func: Some(
            luaUqm_comm_disablePhrase
                as unsafe extern "C" fn(*mut lua_State) -> ::core::ffi::c_int,
        ),
    },
    luaL_Reg {
        name: b"doNpcPhrase\0".as_ptr() as *const ::core::ffi::c_char,
        func: Some(
            luaUqm_comm_doNpcPhrase
                as unsafe extern "C" fn(*mut lua_State) -> ::core::ffi::c_int,
        ),
    },
    luaL_Reg {
        name: b"getPhrase\0".as_ptr() as *const ::core::ffi::c_char,
        func: Some(
            luaUqm_comm_getPhrase
                as unsafe extern "C" fn(*mut lua_State) -> ::core::ffi::c_int,
        ),
    },
    luaL_Reg {
        name: b"getSegue\0".as_ptr() as *const ::core::ffi::c_char,
        func: Some(
            luaUqm_comm_getSegue
                as unsafe extern "C" fn(*mut lua_State) -> ::core::ffi::c_int,
        ),
    },
    luaL_Reg {
        name: b"isInOuttakes\0".as_ptr() as *const ::core::ffi::c_char,
        func: Some(
            luaUqm_comm_isInOuttakes
                as unsafe extern "C" fn(*mut lua_State) -> ::core::ffi::c_int,
        ),
    },
    luaL_Reg {
        name: b"isPhraseEnabled\0".as_ptr() as *const ::core::ffi::c_char,
        func: Some(
            luaUqm_comm_isPhraseEnabled
                as unsafe extern "C" fn(*mut lua_State) -> ::core::ffi::c_int,
        ),
    },
    luaL_Reg {
        name: b"setSegue\0".as_ptr() as *const ::core::ffi::c_char,
        func: Some(
            luaUqm_comm_setSegue
                as unsafe extern "C" fn(*mut lua_State) -> ::core::ffi::c_int,
        ),
    },
    luaL_Reg {
        name: b"setCustomBaseline\0".as_ptr() as *const ::core::ffi::c_char,
        func: Some(
            luaUqm_comm_setCustomBaseline
                as unsafe extern "C" fn(*mut lua_State) -> ::core::ffi::c_int,
        ),
    },
    luaL_Reg {
        name: b"getPoint\0".as_ptr() as *const ::core::ffi::c_char,
        func: Some(
            luaUqm_comm_getPoint
                as unsafe extern "C" fn(*mut lua_State) -> ::core::ffi::c_int,
        ),
    },
    luaL_Reg {
        name: b"getStarName\0".as_ptr() as *const ::core::ffi::c_char,
        func: Some(
            luaUqm_comm_getStarName
                as unsafe extern "C" fn(*mut lua_State) -> ::core::ffi::c_int,
        ),
    },
    luaL_Reg {
        name: b"getConstellation\0".as_ptr() as *const ::core::ffi::c_char,
        func: Some(
            luaUqm_comm_getConstellation
                as unsafe extern "C" fn(*mut lua_State) -> ::core::ffi::c_int,
        ),
    },
    luaL_Reg {
        name: b"getColor\0".as_ptr() as *const ::core::ffi::c_char,
        func: Some(
            luaUqm_comm_getColor
                as unsafe extern "C" fn(*mut lua_State) -> ::core::ffi::c_int,
        ),
    },
    luaL_Reg {
        name: b"swapIfSeeded\0".as_ptr() as *const ::core::ffi::c_char,
        func: Some(
            luaUqm_comm_swapIfSeeded
                as unsafe extern "C" fn(*mut lua_State) -> ::core::ffi::c_int,
        ),
    },
    luaL_Reg {
        name: b"removeIfSeeded\0".as_ptr() as *const ::core::ffi::c_char,
        func: Some(
            luaUqm_comm_removeIfSeeded
                as unsafe extern "C" fn(*mut lua_State) -> ::core::ffi::c_int,
        ),
    },
    luaL_Reg {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        func: None,
    },
];
static mut segueEnum: [luaUqm_EnumValue; 5] = [
    luaUqm_EnumValue {
        name: b"peace\0".as_ptr() as *const ::core::ffi::c_char,
        value: Segue::Segue_peace.0 as ::core::ffi::c_int,
    },
    luaUqm_EnumValue {
        name: b"hostile\0".as_ptr() as *const ::core::ffi::c_char,
        value: Segue::Segue_hostile.0 as ::core::ffi::c_int,
    },
    luaUqm_EnumValue {
        name: b"victory\0".as_ptr() as *const ::core::ffi::c_char,
        value: Segue::Segue_victory.0 as ::core::ffi::c_int,
    },
    luaUqm_EnumValue {
        name: b"defeat\0".as_ptr() as *const ::core::ffi::c_char,
        value: Segue::Segue_defeat.0 as ::core::ffi::c_int,
    },
    luaUqm_EnumValue {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        value: 0 as ::core::ffi::c_int,
    },
];
#[no_mangle]
pub unsafe extern "C" fn luaUqm_comm_open(
    mut luaState: *mut lua_State,
) -> ::core::ffi::c_int {
    uqm_lua_checked!(lua_createtable(
        luaState,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[luaL_Reg; 16]>()
            .wrapping_div(::core::mem::size_of::<luaL_Reg>())
            .wrapping_sub(1usize) as ::core::ffi::c_int,
    ), 0);
    uqm_lua_checked!(luaL_setfuncs(
        luaState,
        &raw const commFuncs as *const luaL_Reg,
        0 as ::core::ffi::c_int,
    ), 0);
    uqm_lua_checked!(luaUqm_makeEnum(luaState, &raw const segueEnum as *const luaUqm_EnumValue), 0);
    uqm_lua_checked!(lua_setfield(
        luaState,
        -2 as ::core::ffi::c_int,
        b"segue\0".as_ptr() as *const ::core::ffi::c_char,
    ), 0);
    uqm_lua_checked!(lua_createtable(luaState, 0 as ::core::ffi::c_int, 0 as ::core::ffi::c_int), 0);
    uqm_lua_checked!(lua_setfield(
        luaState,
        -1000000 as ::core::ffi::c_int - 1000 as ::core::ffi::c_int,
        &raw const responseCallbackRegistryKey as *const ::core::ffi::c_char,
    ), 0);
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn testPhraseId(
    mut luaState: *mut lua_State,
    mut argn: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut phraseIdStr: *const ::core::ffi::c_char = uqm_lua_checked!(luaL_checklstring(
        luaState,
        argn,
        ::core::ptr::null_mut::<size_t>(),
    ), 0);
    let mut phraseId: RESPONSE_REF = phraseIdStrToNum(phraseIdStr);
    if phraseId as ::core::ffi::c_int
        == -1 as ::core::ffi::c_int as RESPONSE_REF as ::core::ffi::c_int
    {
        log_add(
            log_Level::log_Error,
            b"[script] Warning: testPhraseId(): No phrase exists with id '%s'.\0"
                .as_ptr() as *const ::core::ffi::c_char,
            phraseIdStr,
        );
        return -1 as ::core::ffi::c_int;
    }
    return phraseId as ::core::ffi::c_int;
}
unsafe extern "C" fn pushPhraseId(
    mut luaState: *mut lua_State,
    mut response: RESPONSE_REF,
) {
    let mut phraseIdStr: *const ::core::ffi::c_char = phraseIdNumToStr(response);
    if !phraseIdStr.is_null() {
        uqm_lua_checked!(lua_pushstring(luaState, phraseIdStr), ());
    } else {
        uqm_lua_checked!(lua_pushnil(luaState), ());
    };
}
unsafe extern "C" fn setNpcPhraseCallback(
    mut luaState: *mut lua_State,
    mut argn: ::core::ffi::c_int,
) {
    uqm_lua_checked!(lua_pushvalue(luaState, argn), ());
    uqm_lua_checked!(lua_setfield(
        luaState,
        -1000000 as ::core::ffi::c_int - 1000 as ::core::ffi::c_int,
        &raw const npcPhraseCallbackRegistryKey as *const ::core::ffi::c_char,
    ), ());
}
unsafe extern "C" fn pushNpcPhraseCallback(mut luaState: *mut lua_State) {
    uqm_lua_checked!(lua_getfield(
        luaState,
        -1000000 as ::core::ffi::c_int - 1000 as ::core::ffi::c_int,
        &raw const npcPhraseCallbackRegistryKey as *const ::core::ffi::c_char,
    ), ());
}
unsafe extern "C" fn pushResponseCallbackRegistry(mut luaState: *mut lua_State) {
    uqm_lua_checked!(lua_getfield(
        luaState,
        -1000000 as ::core::ffi::c_int - 1000 as ::core::ffi::c_int,
        &raw const responseCallbackRegistryKey as *const ::core::ffi::c_char,
    ), ());
}
unsafe extern "C" fn setResponseCallback(
    mut luaState: *mut lua_State,
    mut responseArgN: ::core::ffi::c_int,
    mut callbackArgN: ::core::ffi::c_int,
) {
    uqm_lua_checked!(pushResponseCallbackRegistry(luaState), ());
    uqm_lua_checked!(lua_pushvalue(luaState, responseArgN), ());
    uqm_lua_checked!(lua_pushvalue(luaState, callbackArgN), ());
    uqm_lua_checked!(lua_settable(luaState, -3 as ::core::ffi::c_int), ());
    uqm_lua_checked!(lua_settop(luaState, -1 as ::core::ffi::c_int - 1 as ::core::ffi::c_int), ());
}
unsafe extern "C" fn pushResponseCallback(
    mut luaState: *mut lua_State,
    mut response: RESPONSE_REF,
) {
    uqm_lua_checked!(pushResponseCallbackRegistry(luaState), ());
    uqm_lua_checked!(pushPhraseId(luaState, response), ());
    uqm_lua_checked!(lua_gettable(luaState, -2 as ::core::ffi::c_int), ());
    uqm_lua_checked!(lua_replace(luaState, -2 as ::core::ffi::c_int), ());
}
unsafe extern "C" fn npcPhraseCallback(mut extra: CallbackArg) {
    uqm_lua_checked!(pushNpcPhraseCallback(luaUqm_commState), ());
    if uqm_lua_checked!(lua_pcallk(
        luaUqm_commState,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        None,
    ), ()) != 0 as ::core::ffi::c_int
    {
        log_add(
            log_Level::log_Error,
            b"[script] An error occurred during a doNpcPhrase() callback: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            uqm_lua_checked!(lua_tolstring(
                luaUqm_commState,
                -1 as ::core::ffi::c_int,
                ::core::ptr::null_mut::<size_t>(),
            ), ()),
        );
        uqm_lua_checked!(lua_settop(luaUqm_commState, -1 as ::core::ffi::c_int - 1 as ::core::ffi::c_int), ());
    }
}
unsafe extern "C" fn responseCallback(mut response: RESPONSE_REF) {
    uqm_lua_checked!(pushResponseCallback(luaUqm_commState, response), ());
    uqm_lua_checked!(pushPhraseId(luaUqm_commState, response), ());
    if uqm_lua_checked!(lua_pcallk(
        luaUqm_commState,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        None,
    ), ()) != 0 as ::core::ffi::c_int
    {
        log_add(
            log_Level::log_Error,
            b"[script] An error occurred during an addResponse() callback: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            uqm_lua_checked!(lua_tolstring(
                luaUqm_commState,
                -1 as ::core::ffi::c_int,
                ::core::ptr::null_mut::<size_t>(),
            ), ()),
        );
        uqm_lua_checked!(lua_settop(luaUqm_commState, -1 as ::core::ffi::c_int - 1 as ::core::ffi::c_int), ());
    }
}
unsafe extern "C" fn luaUqm_comm_isPhraseEnabled(
    mut luaState: *mut lua_State,
) -> ::core::ffi::c_int {
    let mut phraseId: ::core::ffi::c_int = uqm_lua_checked!(testPhraseId(
        luaState,
        1 as ::core::ffi::c_int,
    ), 0);
    if phraseId == -1 as ::core::ffi::c_int {
        uqm_lua_checked!(lua_pushboolean(luaState, BOOLEAN::FALSE.0 as ::core::ffi::c_int), 0);
        return 1 as ::core::ffi::c_int;
    }
    uqm_lua_checked!(lua_pushboolean(
        luaState,
        (*GetStringAddress(
            SetAbsStringTableIndex(
                CommData.ConversationPhrases,
                (phraseId - 1 as ::core::ffi::c_int) as COUNT,
            ),
        ) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int) as ::core::ffi::c_int,
    ), 0);
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn luaUqm_comm_disablePhrase(
    mut luaState: *mut lua_State,
) -> ::core::ffi::c_int {
    let mut phraseId: ::core::ffi::c_int = uqm_lua_checked!(testPhraseId(
        luaState,
        1 as ::core::ffi::c_int,
    ), 0);
    if phraseId == -1 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    *GetStringAddress(
        SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (phraseId - 1 as ::core::ffi::c_int) as COUNT,
        ),
    ) = '\0' as UNICODE;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn luaUqm_comm_doNpcPhrase(
    mut luaState: *mut lua_State,
) -> ::core::ffi::c_int {
    let mut callback: CallbackFunction = None;
    let mut phraseId: ::core::ffi::c_int = uqm_lua_checked!(testPhraseId(
        luaState,
        1 as ::core::ffi::c_int,
    ), 0);
    if phraseId == -1 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    if uqm_lua_checked!(lua_gettop(luaState), 0) >= 2 as ::core::ffi::c_int {
        uqm_lua_checked!(setNpcPhraseCallback(luaState, 2 as ::core::ffi::c_int), 0);
        callback = Some(npcPhraseCallback as unsafe extern "C" fn(CallbackArg) -> ())
            as CallbackFunction;
    } else {
        callback = None;
    }
    uqm_lua_checked!(NPCPhrase_cb(phraseId, callback), 0);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn luaUqm_comm_addResponse(
    mut luaState: *mut lua_State,
) -> ::core::ffi::c_int {
    let mut phraseId: ::core::ffi::c_int = uqm_lua_checked!(testPhraseId(
        luaState,
        1 as ::core::ffi::c_int,
    ), 0);
    if phraseId == -1 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    uqm_lua_checked!(luaL_checktype(luaState, 2 as ::core::ffi::c_int, 6 as ::core::ffi::c_int), 0);
    uqm_lua_checked!(setResponseCallback(luaState, 1 as ::core::ffi::c_int, 2 as ::core::ffi::c_int), 0);
    uqm_lua_checked!(DoResponsePhrase(
        phraseId as RESPONSE_REF,
        Some(responseCallback as unsafe extern "C" fn(RESPONSE_REF) -> ()),
        ::core::ptr::null_mut::<UNICODE>(),
    ), 0);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn luaUqm_comm_getPhrase(
    mut luaState: *mut lua_State,
) -> ::core::ffi::c_int {
    let mut phraseId: ::core::ffi::c_int = 0;
    let mut str: STRING = ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>();
    let mut strBuf: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    phraseId = uqm_lua_checked!(testPhraseId(luaState, 1 as ::core::ffi::c_int), 0);
    if phraseId == -1 as ::core::ffi::c_int {
        uqm_lua_checked!(lua_pushnil(luaState), 0);
        return 1 as ::core::ffi::c_int;
    }
    str = SetAbsStringTableIndex(
        CommData.ConversationPhrases,
        (phraseId - 1 as ::core::ffi::c_int) as COUNT,
    );
    strBuf = GetStringAddress(str) as *const ::core::ffi::c_char;
    if luaUqm_comm_stringNeedsInterpolate(strBuf).0 != 0 {
        let mut interpolated: *mut ::core::ffi::c_char = uqm_lua_checked!(luaUqm_comm_stringInterpolate(
            strBuf,
        ), 0);
        uqm_lua_checked!(lua_pushstring(luaState, interpolated), 0);
        HFree(interpolated as *mut ::core::ffi::c_void);
    } else {
        uqm_lua_checked!(lua_pushstring(luaState, strBuf), 0);
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn luaUqm_comm_getSegue(
    mut luaState: *mut lua_State,
) -> ::core::ffi::c_int {
    let mut result: ::core::ffi::c_int = uqm_lua_checked!(getSegue(), 0).0 as ::core::ffi::c_int;
    uqm_lua_checked!(lua_pushinteger(luaState, result as lua_Integer), 0);
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn luaUqm_comm_setSegue(
    mut luaState: *mut lua_State,
) -> ::core::ffi::c_int {
    let mut what: ::core::ffi::c_int = uqm_lua_checked!(luaL_checkinteger(
        luaState,
        1 as ::core::ffi::c_int,
    ), 0) as ::core::ffi::c_int;
    match Segue(what as ::core::ffi::c_uint) {
        Segue::Segue_peace
        | Segue::Segue_hostile
        | Segue::Segue_victory
        | Segue::Segue_defeat => {}
        _ => {
            log_add(
                log_Level::log_Error,
                b"[script] Warning: setSegue(): Invalid parameter value (%d).\0".as_ptr()
                    as *const ::core::ffi::c_char,
                what,
            );
        }
    }
    uqm_lua_checked!(setSegue(Segue(what as ::core::ffi::c_uint)), 0);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn luaUqm_comm_isInOuttakes(
    mut luaState: *mut lua_State,
) -> ::core::ffi::c_int {
    let mut result: BOOLEAN = BOOLEAN(
        (GlobData.Game_state.CurrentActivity as BYTE as ::core::ffi::c_int
            == C2Rust_Unnamed_0::WON_LAST_BATTLE.0 as ::core::ffi::c_int)
            as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
    uqm_lua_checked!(lua_pushboolean(luaState, result.0 as ::core::ffi::c_int), 0);
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn luaUqm_comm_setCustomBaseline(
    mut luaState: *mut lua_State,
) -> ::core::ffi::c_int {
    static mut textAlign: [*const ::core::ffi::c_char; 4] = [
        b"ALIGN_LEFT\0".as_ptr() as *const ::core::ffi::c_char,
        b"ALIGN_CENTER\0".as_ptr() as *const ::core::ffi::c_char,
        b"ALIGN_RIGHT\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
    ];
    let mut lineNumber: COUNT = uqm_lua_checked!(luaL_checkinteger(luaState, 1 as ::core::ffi::c_int), 0)
        as ::core::ffi::c_int as COUNT;
    let mut baselineX: COORD = uqm_lua_checked!(luaL_checkinteger(luaState, 2 as ::core::ffi::c_int), 0)
        as ::core::ffi::c_int as COORD;
    let mut baselineY: COORD = uqm_lua_checked!(luaL_checkinteger(luaState, 3 as ::core::ffi::c_int), 0)
        as ::core::ffi::c_int as COORD;
    let mut alignment: ::core::ffi::c_int = uqm_lua_checked!(luaL_checkoption(
        luaState,
        4 as ::core::ffi::c_int,
        b"ALIGN_LEFT\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const textAlign as *const *const ::core::ffi::c_char,
    ), 0);
    SetCustomBaseLine(
        lineNumber,
        point {
            x: baselineX,
            y: baselineY,
        },
        TEXT_ALIGN(alignment as ::core::ffi::c_uint),
    );
    uqm_lua_checked!(lua_pushstring(luaState, b"\0".as_ptr() as *const ::core::ffi::c_char), 0);
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn luaUqm_comm_getPoint(
    mut luaState: *mut lua_State,
) -> ::core::ffi::c_int {
    let mut prime_text: *const ::core::ffi::c_char = uqm_lua_checked!(luaL_checklstring(
        luaState,
        1 as ::core::ffi::c_int,
        ::core::ptr::null_mut::<size_t>(),
    ), 0);
    if !(optSeedType > C2Rust_Unnamed::OPTVAL_PLANET.0 as ::core::ffi::c_int) {
        uqm_lua_checked!(lua_pushstring(luaState, prime_text), 0);
        return 1 as ::core::ffi::c_int;
    }
    let mut plot_name: *const ::core::ffi::c_char = uqm_lua_checked!(luaL_checklstring(
        luaState,
        2 as ::core::ffi::c_int,
        ::core::ptr::null_mut::<size_t>(),
    ), 0);
    let mut plot_id: COUNT = PlotIdStrToIndex(plot_name);
    if plot_id as ::core::ffi::c_int
        >= C2Rust_Unnamed_3::NUM_PLOTS.0 as ::core::ffi::c_int
    {
        fprintf(
            stderr,
            b"Plot not found for Point (%s %s).\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            prime_text,
            plot_name,
        );
        uqm_lua_checked!(lua_pushstring(luaState, prime_text), 0);
        return 1 as ::core::ffi::c_int;
    }
    let mut dialog: [::core::ffi::c_char; 256] = [0; 256];
    snprintf(
        &raw mut dialog as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>() as size_t,
        b"%05.1f : %05.1f\0".as_ptr() as *const ::core::ffi::c_char,
        (plot_map[plot_id as usize].star_pt.x as ::core::ffi::c_float
            / 10 as ::core::ffi::c_int as ::core::ffi::c_float) as ::core::ffi::c_double,
        (plot_map[plot_id as usize].star_pt.y as ::core::ffi::c_float
            / 10 as ::core::ffi::c_int as ::core::ffi::c_float) as ::core::ffi::c_double,
    );
    uqm_lua_checked!(lua_pushstring(luaState, &raw mut dialog as *mut ::core::ffi::c_char), 0);
    *(&raw mut RoboTrack as *mut COUNT).offset(0isize) = (C2Rust_Unnamed_1::ROBOT_DIGIT_0
        .0 as ::core::ffi::c_int
        + plot_map[plot_id as usize].star_pt.x as ::core::ffi::c_int
            / 1000 as ::core::ffi::c_int) as COUNT;
    *(&raw mut RoboTrack as *mut COUNT).offset(1isize) = (C2Rust_Unnamed_1::ROBOT_DIGIT_0
        .0 as ::core::ffi::c_int
        + plot_map[plot_id as usize].star_pt.x as ::core::ffi::c_int
            / 100 as ::core::ffi::c_int % 10 as ::core::ffi::c_int) as COUNT;
    *(&raw mut RoboTrack as *mut COUNT).offset(2isize) = (C2Rust_Unnamed_1::ROBOT_DIGIT_0
        .0 as ::core::ffi::c_int
        + plot_map[plot_id as usize].star_pt.x as ::core::ffi::c_int
            / 10 as ::core::ffi::c_int % 10 as ::core::ffi::c_int) as COUNT;
    *(&raw mut RoboTrack as *mut COUNT).offset(3isize) = C2Rust_Unnamed_1::ROBOT_POINT.0
        as ::core::ffi::c_int as COUNT;
    *(&raw mut RoboTrack as *mut COUNT).offset(4isize) = (C2Rust_Unnamed_1::ROBOT_DIGIT_0
        .0 as ::core::ffi::c_int
        + plot_map[plot_id as usize].star_pt.x as ::core::ffi::c_int
            % 10 as ::core::ffi::c_int) as COUNT;
    *(&raw mut RoboTrack as *mut COUNT).offset(5isize) = C2Rust_Unnamed_1::ROBOT_BY.0
        as ::core::ffi::c_int as COUNT;
    *(&raw mut RoboTrack as *mut COUNT).offset(6isize) = (C2Rust_Unnamed_1::ROBOT_DIGIT_0
        .0 as ::core::ffi::c_int
        + plot_map[plot_id as usize].star_pt.y as ::core::ffi::c_int
            / 1000 as ::core::ffi::c_int) as COUNT;
    *(&raw mut RoboTrack as *mut COUNT).offset(7isize) = (C2Rust_Unnamed_1::ROBOT_DIGIT_0
        .0 as ::core::ffi::c_int
        + plot_map[plot_id as usize].star_pt.y as ::core::ffi::c_int
            / 100 as ::core::ffi::c_int % 10 as ::core::ffi::c_int) as COUNT;
    *(&raw mut RoboTrack as *mut COUNT).offset(8isize) = (C2Rust_Unnamed_1::ROBOT_DIGIT_0
        .0 as ::core::ffi::c_int
        + plot_map[plot_id as usize].star_pt.y as ::core::ffi::c_int
            / 10 as ::core::ffi::c_int % 10 as ::core::ffi::c_int) as COUNT;
    *(&raw mut RoboTrack as *mut COUNT).offset(9isize) = C2Rust_Unnamed_1::ROBOT_POINT.0
        as ::core::ffi::c_int as COUNT;
    *(&raw mut RoboTrack as *mut COUNT).offset(10isize) = (C2Rust_Unnamed_1::ROBOT_DIGIT_0
        .0 as ::core::ffi::c_int
        + plot_map[plot_id as usize].star_pt.y as ::core::ffi::c_int
            % 10 as ::core::ffi::c_int) as COUNT;
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn CheckCase(
    mut key: *const ::core::ffi::c_char,
    mut dialog: *mut ::core::ffi::c_char,
) {
    let mut i: COUNT = 0 as COUNT;
    if isupper(*key.offset(0isize) as ::core::ffi::c_int) != 0
        && isupper(*key.offset(1isize) as ::core::ffi::c_int) != 0
    {
        while *dialog.offset(i as isize) as ::core::ffi::c_int
            != '\0' as ::core::ffi::c_int
        {
            *dialog.offset(i as isize) = toupper(
                *dialog.offset(i as isize) as ::core::ffi::c_int,
            ) as ::core::ffi::c_char;
            i = i.wrapping_add(1);
        }
    } else if isupper(*key.offset(0isize) as ::core::ffi::c_int) != 0
        && islower(*dialog.offset(0isize) as ::core::ffi::c_int) != 0
    {
        *dialog.offset(0isize) = toupper(*dialog.offset(0isize) as ::core::ffi::c_int)
            as ::core::ffi::c_char;
    }
}
unsafe extern "C" fn luaUqm_comm_getStarName(
    mut luaState: *mut lua_State,
) -> ::core::ffi::c_int {
    let mut prime_text: *const ::core::ffi::c_char = uqm_lua_checked!(luaL_checklstring(
        luaState,
        1 as ::core::ffi::c_int,
        ::core::ptr::null_mut::<size_t>(),
    ), 0);
    if !(optSeedType > C2Rust_Unnamed::OPTVAL_PLANET.0 as ::core::ffi::c_int) {
        uqm_lua_checked!(lua_pushstring(luaState, prime_text), 0);
        return 1 as ::core::ffi::c_int;
    }
    let mut plot_name: *const ::core::ffi::c_char = uqm_lua_checked!(luaL_checklstring(
        luaState,
        2 as ::core::ffi::c_int,
        ::core::ptr::null_mut::<size_t>(),
    ), 0);
    let mut plot_id: COUNT = PlotIdStrToIndex(plot_name);
    if plot_id as ::core::ffi::c_int
        >= C2Rust_Unnamed_3::NUM_PLOTS.0 as ::core::ffi::c_int
    {
        fprintf(
            stderr,
            b"Plot not found for Star Name (%s %s).\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            prime_text,
            plot_name,
        );
        uqm_lua_checked!(lua_pushstring(luaState, prime_text), 0);
        return 1 as ::core::ffi::c_int;
    }
    let mut dialog: [::core::ffi::c_char; 256] = [0; 256];
    GetClusterName(plot_map[plot_id as usize].star, &raw mut dialog as *mut UNICODE);
    CheckCase(prime_text, &raw mut dialog as *mut ::core::ffi::c_char);
    uqm_lua_checked!(lua_pushstring(luaState, &raw mut dialog as *mut ::core::ffi::c_char), 0);
    if (*plot_map[plot_id as usize].star).Prefix as ::core::ffi::c_int
        > 0 as ::core::ffi::c_int
    {
        *(&raw mut RoboTrack as *mut COUNT).offset(0isize) = (C2Rust_Unnamed_1::ROBOT_PREFIX_0
            .0 as ::core::ffi::c_int
            + (*plot_map[plot_id as usize].star).Prefix as ::core::ffi::c_int) as COUNT;
        *(&raw mut RoboTrack as *mut COUNT).offset(1isize) = (C2Rust_Unnamed_1::ROBOT_POSTFIX_0
            .0 as ::core::ffi::c_int
            + (*plot_map[plot_id as usize].star).Postfix as ::core::ffi::c_int) as COUNT;
    } else {
        *(&raw mut RoboTrack as *mut COUNT).offset(0isize) = (C2Rust_Unnamed_1::ROBOT_POSTFIX_0
            .0 as ::core::ffi::c_int
            + (*plot_map[plot_id as usize].star).Postfix as ::core::ffi::c_int) as COUNT;
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn luaUqm_comm_getConstellation(
    mut luaState: *mut lua_State,
) -> ::core::ffi::c_int {
    let mut prime_text: *const ::core::ffi::c_char = uqm_lua_checked!(luaL_checklstring(
        luaState,
        1 as ::core::ffi::c_int,
        ::core::ptr::null_mut::<size_t>(),
    ), 0);
    if !(optSeedType > C2Rust_Unnamed::OPTVAL_PLANET.0 as ::core::ffi::c_int) {
        uqm_lua_checked!(lua_pushstring(luaState, prime_text), 0);
        return 1 as ::core::ffi::c_int;
    }
    let mut plot_name: *const ::core::ffi::c_char = uqm_lua_checked!(luaL_checklstring(
        luaState,
        2 as ::core::ffi::c_int,
        ::core::ptr::null_mut::<size_t>(),
    ), 0);
    let mut plot_id: COUNT = PlotIdStrToIndex(plot_name);
    if plot_id as ::core::ffi::c_int
        >= C2Rust_Unnamed_3::NUM_PLOTS.0 as ::core::ffi::c_int
    {
        fprintf(
            stderr,
            b"Plot not found for Constellation (%s %s).\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            prime_text,
            plot_name,
        );
        uqm_lua_checked!(lua_pushstring(luaState, prime_text), 0);
        return 1 as ::core::ffi::c_int;
    }
    let mut dialog: [::core::ffi::c_char; 256] = [0; 256];
    let mut SDPtr: *mut STAR_DESC = FindNearestConstellation(
        &raw mut star_array as *mut STAR_DESC,
        plot_map[plot_id as usize].star_pt,
    );
    snprintf(
        &raw mut dialog as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>() as size_t,
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        GetStringAddress(SetAbsStringTableIndex(GameStrings, (*SDPtr).Postfix as COUNT)),
    );
    CheckCase(prime_text, &raw mut dialog as *mut ::core::ffi::c_char);
    uqm_lua_checked!(lua_pushstring(luaState, &raw mut dialog as *mut ::core::ffi::c_char), 0);
    *(&raw mut RoboTrack as *mut COUNT).offset(0isize) = (C2Rust_Unnamed_1::ROBOT_POSTFIX_0
        .0 as ::core::ffi::c_int + (*SDPtr).Postfix as ::core::ffi::c_int) as COUNT;
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn luaUqm_comm_getColor(
    mut luaState: *mut lua_State,
) -> ::core::ffi::c_int {
    let mut prime_text: *const ::core::ffi::c_char = uqm_lua_checked!(luaL_checklstring(
        luaState,
        1 as ::core::ffi::c_int,
        ::core::ptr::null_mut::<size_t>(),
    ), 0);
    if !(optSeedType > C2Rust_Unnamed::OPTVAL_PLANET.0 as ::core::ffi::c_int) {
        uqm_lua_checked!(lua_pushstring(luaState, prime_text), 0);
        return 1 as ::core::ffi::c_int;
    }
    let mut plot_name: *const ::core::ffi::c_char = uqm_lua_checked!(luaL_checklstring(
        luaState,
        2 as ::core::ffi::c_int,
        ::core::ptr::null_mut::<size_t>(),
    ), 0);
    let mut plot_id: COUNT = PlotIdStrToIndex(plot_name);
    if plot_id as ::core::ffi::c_int
        >= C2Rust_Unnamed_3::NUM_PLOTS.0 as ::core::ffi::c_int
    {
        fprintf(
            stderr,
            b"Plot not found for getColor (%s %s).\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            prime_text,
            plot_name,
        );
        uqm_lua_checked!(lua_pushstring(luaState, prime_text), 0);
        return 1 as ::core::ffi::c_int;
    }
    let mut dialog: [::core::ffi::c_char; 256] = [0; 256];
    match (((*plot_map[plot_id as usize].star).Type as ::core::ffi::c_int
        & ((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as BYTE
            as ::core::ffi::c_int) >> 5 as ::core::ffi::c_int) as BYTE
        as ::core::ffi::c_int
    {
        3 => {
            snprintf(
                &raw mut dialog as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 256]>() as size_t,
                b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                b"red\0".as_ptr() as *const ::core::ffi::c_char,
            );
            *(&raw mut RoboTrack as *mut COUNT).offset(0isize) = (if plot_id
                as ::core::ffi::c_int
                == C2Rust_Unnamed_3::ILWRATH_DEFINED.0 as ::core::ffi::c_int
            {
                C2Rust_Unnamed_1::ILWRATH_COLOR_RED.0 as ::core::ffi::c_int
            } else {
                C2Rust_Unnamed_1::ROBOT_COLOR_RED.0 as ::core::ffi::c_int
            }) as COUNT;
        }
        2 => {
            snprintf(
                &raw mut dialog as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 256]>() as size_t,
                b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                b"orange\0".as_ptr() as *const ::core::ffi::c_char,
            );
            *(&raw mut RoboTrack as *mut COUNT).offset(0isize) = (if plot_id
                as ::core::ffi::c_int
                == C2Rust_Unnamed_3::ILWRATH_DEFINED.0 as ::core::ffi::c_int
            {
                C2Rust_Unnamed_1::ILWRATH_COLOR_ORANGE.0 as ::core::ffi::c_int
            } else {
                C2Rust_Unnamed_1::ROBOT_COLOR_ORANGE.0 as ::core::ffi::c_int
            }) as COUNT;
        }
        5 => {
            snprintf(
                &raw mut dialog as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 256]>() as size_t,
                b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                b"yellow\0".as_ptr() as *const ::core::ffi::c_char,
            );
            *(&raw mut RoboTrack as *mut COUNT).offset(0isize) = (if plot_id
                as ::core::ffi::c_int
                == C2Rust_Unnamed_3::ILWRATH_DEFINED.0 as ::core::ffi::c_int
            {
                C2Rust_Unnamed_1::ILWRATH_COLOR_YELLOW.0 as ::core::ffi::c_int
            } else {
                C2Rust_Unnamed_1::ROBOT_COLOR_YELLOW.0 as ::core::ffi::c_int
            }) as COUNT;
        }
        1 => {
            snprintf(
                &raw mut dialog as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 256]>() as size_t,
                b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                b"green\0".as_ptr() as *const ::core::ffi::c_char,
            );
            *(&raw mut RoboTrack as *mut COUNT).offset(0isize) = (if plot_id
                as ::core::ffi::c_int
                == C2Rust_Unnamed_3::ILWRATH_DEFINED.0 as ::core::ffi::c_int
            {
                C2Rust_Unnamed_1::ILWRATH_COLOR_GREEN.0 as ::core::ffi::c_int
            } else {
                C2Rust_Unnamed_1::ROBOT_COLOR_GREEN.0 as ::core::ffi::c_int
            }) as COUNT;
        }
        0 => {
            snprintf(
                &raw mut dialog as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 256]>() as size_t,
                b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                b"blue\0".as_ptr() as *const ::core::ffi::c_char,
            );
            *(&raw mut RoboTrack as *mut COUNT).offset(0isize) = (if plot_id
                as ::core::ffi::c_int
                == C2Rust_Unnamed_3::ILWRATH_DEFINED.0 as ::core::ffi::c_int
            {
                C2Rust_Unnamed_1::ILWRATH_COLOR_BLUE.0 as ::core::ffi::c_int
            } else {
                C2Rust_Unnamed_1::ROBOT_COLOR_BLUE.0 as ::core::ffi::c_int
            }) as COUNT;
        }
        4 => {
            snprintf(
                &raw mut dialog as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 256]>() as size_t,
                b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                b"white\0".as_ptr() as *const ::core::ffi::c_char,
            );
            *(&raw mut RoboTrack as *mut COUNT).offset(0isize) = (if plot_id
                as ::core::ffi::c_int
                == C2Rust_Unnamed_3::ILWRATH_DEFINED.0 as ::core::ffi::c_int
            {
                C2Rust_Unnamed_1::ILWRATH_COLOR_WHITE.0 as ::core::ffi::c_int
            } else {
                C2Rust_Unnamed_1::ROBOT_COLOR_WHITE.0 as ::core::ffi::c_int
            }) as COUNT;
        }
        _ => {
            snprintf(
                &raw mut dialog as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 256]>() as size_t,
                b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                b"unknown\0".as_ptr() as *const ::core::ffi::c_char,
            );
            *(&raw mut RoboTrack as *mut COUNT).offset(0isize) = C2Rust_Unnamed_1::ROBOT_NULL_PHRASE
                .0 as ::core::ffi::c_int as COUNT;
        }
    }
    CheckCase(prime_text, &raw mut dialog as *mut ::core::ffi::c_char);
    uqm_lua_checked!(lua_pushstring(luaState, &raw mut dialog as *mut ::core::ffi::c_char), 0);
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn luaUqm_comm_swapIfSeeded(
    mut luaState: *mut lua_State,
) -> ::core::ffi::c_int {
    let mut prime_text: *const ::core::ffi::c_char = uqm_lua_checked!(luaL_checklstring(
        luaState,
        1 as ::core::ffi::c_int,
        ::core::ptr::null_mut::<size_t>(),
    ), 0);
    if !(optSeedType > C2Rust_Unnamed::OPTVAL_PLANET.0 as ::core::ffi::c_int) {
        uqm_lua_checked!(lua_pushstring(luaState, prime_text), 0);
        return 1 as ::core::ffi::c_int;
    }
    let mut seed_text: *const ::core::ffi::c_char = uqm_lua_checked!(luaL_checklstring(
        luaState,
        2 as ::core::ffi::c_int,
        ::core::ptr::null_mut::<size_t>(),
    ), 0);
    uqm_lua_checked!(lua_pushstring(luaState, seed_text), 0);
    *(&raw mut RoboTrack as *mut COUNT).offset(0isize) = !(0 as ::core::ffi::c_int)
        as COUNT;
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn luaUqm_comm_removeIfSeeded(
    mut luaState: *mut lua_State,
) -> ::core::ffi::c_int {
    let mut prime_text: *const ::core::ffi::c_char = uqm_lua_checked!(luaL_checklstring(
        luaState,
        1 as ::core::ffi::c_int,
        ::core::ptr::null_mut::<size_t>(),
    ), 0);
    if !(optSeedType > C2Rust_Unnamed::OPTVAL_PLANET.0 as ::core::ffi::c_int) {
        uqm_lua_checked!(lua_pushstring(luaState, prime_text), 0);
        return 1 as ::core::ffi::c_int;
    }
    uqm_lua_checked!(lua_pushstring(luaState, b"\0".as_ptr() as *const ::core::ffi::c_char), 0);
    return 1 as ::core::ffi::c_int;
}
