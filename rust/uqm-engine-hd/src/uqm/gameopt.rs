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

// Rust translation of src/uqm/gameopt.c.
extern "C" {
    pub type uio_Stream;
    pub type string_table_entry;
    pub type context_desc;
    pub type frame_desc;
    pub type font_desc;
    pub type drawable_desc;
    pub type tfb_soundsample;
    pub type FlashContext;
    pub type joy_char;
    fn hypot(
        _: ::core::ffi::c_double,
        _: ::core::ffi::c_double,
    ) -> ::core::ffi::c_double;
    fn SetContext(Context: CONTEXT) -> CONTEXT;
    fn SetContextForeGroundColor(Color: Color) -> Color;
    fn SetContextBackGroundColor(Color: Color) -> Color;
    fn SetContextClipRect(pRect: *mut RECT) -> BOOLEAN;
    fn GetContextClipRect(pRect: *mut RECT) -> BOOLEAN;
    fn DrawStamp(pStamp: *mut STAMP);
    fn DrawRectangle(pRect: *mut RECT, scaled: BOOLEAN);
    fn DrawFilledRectangle(pRect: *mut RECT);
    fn font_GetTextRect(pText: *mut TEXT) -> RECT;
    fn font_DrawText(pText: *mut TEXT);
    fn BatchGraphics();
    fn UnbatchGraphics();
    fn ClearDrawable();
    fn DestroyDrawable(Drawable: DRAWABLE) -> BOOLEAN;
    fn GetFrameRect(Frame: FRAME, pRect: *mut RECT) -> BOOLEAN;
    fn SetContextFont(Font: FONT) -> FONT;
    fn TextRect(pText: *mut TEXT, pRect: *mut RECT, pdelta: *mut BYTE) -> BOOLEAN;
    fn GetContextFontLeading(pheight: *mut SIZE) -> BOOLEAN;
    fn GetFrameCount(Frame: FRAME) -> COUNT;
    fn SetAbsFrameIndex(Frame: FRAME, FrameIndex: COUNT) -> FRAME;
    fn SetRelFrameIndex(Frame: FRAME, FrameOffs: SIZE) -> FRAME;
    fn DecFrameIndex(Frame: FRAME) -> FRAME;
    fn ReleaseDrawable(Frame: FRAME) -> DRAWABLE;
    fn FadeScreen(fadeType: ScreenFadeType, TimeInterval: SIZE) -> DWORD;
    fn HMalloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn HFree(p: *mut ::core::ffi::c_void);
    fn printf(_: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn snprintf(
        _: *mut ::core::ffi::c_char,
        _: size_t,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn SetAbsStringTableIndex(String: STRING, StringTableIndex: COUNT) -> STRING;
    fn GetStringAddress(String: STRING) -> STRINGPTR;
    fn utf8StringCount(start: *const ::core::ffi::c_char) -> size_t;
    fn utf8StringPos(
        pStr: *const ::core::ffi::c_char,
        ch: UniChar,
    ) -> ::core::ffi::c_int;
    fn utf8StringCopy(
        dst: *mut ::core::ffi::c_char,
        size: size_t,
        src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_uchar;
    fn skipUTF8Chars(
        ptr: *const ::core::ffi::c_char,
        num: size_t,
    ) -> *mut ::core::ffi::c_char;
    fn AlignText(str: *const UNICODE, loc_x: *mut sint16) -> *mut UNICODE;
    static mut PlayFrame: FRAME;
    fn GetTimeCounter() -> TimeCount;
    fn memset(
        _: *mut ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strcpy(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strncpy(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: size_t,
    ) -> *mut ::core::ffi::c_char;
    fn strncat(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: size_t,
    ) -> *mut ::core::ffi::c_char;
    fn strncmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: size_t,
    ) -> ::core::ffi::c_int;
    fn strtok(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(_: *const ::core::ffi::c_char) -> size_t;
    static mut PulsedInputState: CONTROLLER_INPUT_STATE;
    fn ConfirmExit() -> BOOLEAN;
    fn DoPopupWindow(msg: *const ::core::ffi::c_char);
    fn SetInputCallback(_: Option<InputFrameCallback>) -> Option<InputFrameCallback>;
    fn DoInput(pInputState: *mut ::core::ffi::c_void, resetInput: BOOLEAN);
    fn DoTextEntry(pTES: *mut TEXTENTRY_STATE) -> BOOLEAN;
    static mut optWhichMenu: ::core::ffi::c_int;
    static mut optWhichFonts: ::core::ffi::c_int;
    static mut resolutionFactor: ::core::ffi::c_uint;
    static mut optInfiniteRU: OPT_ENABLABLE;
    static mut oldRU: DWORD;
    static mut optInfiniteFuel: OPT_ENABLABLE;
    static mut loadFuel: DWORD;
    static mut optCustomBorder: OPT_ENABLABLE;
    static mut optSeedType: ::core::ffi::c_int;
    static mut optWindowType: DWORD;
    static mut CanvasWidth: ::core::ffi::c_int;
    static mut CanvasHeight: ::core::ffi::c_int;
    fn InitQueue(pq: *mut QUEUE, num_elements: COUNT, size: OBJ_SIZE) -> BOOLEAN;
    fn UninitQueue(pq: *mut QUEUE) -> BOOLEAN;
    fn StopSound();
    fn FadeMusic(end_vol: BYTE, TimeInterval: SIZE) -> DWORD;
    fn SleepThread(timePeriod: TimePeriod);
    fn SleepThreadUntil(wakeTime: TimeCount);
    fn ClearSISRect(ClearFlags: BYTE);
    fn SetFlashRect(pRect: *const RECT, purple: BOOLEAN);
    fn PreUpdateFlashRect();
    fn PostUpdateFlashRect();
    fn DrawHyperCoords(puniverse: POINT);
    fn DrawSaveInfo(SisState: SIS_STATE);
    fn DrawSISTitle(pStr: *mut UNICODE);
    fn DrawSISMessage(pStr: *const UNICODE);
    fn DateToString(
        buf: *mut ::core::ffi::c_char,
        bufLen: size_t,
        month_index: BYTE,
        day_index: BYTE,
        year_index: COUNT,
    );
    fn DrawStatusMessage(pStr: *const UNICODE);
    fn DeltaSISGauges(
        crew_delta: SIZE,
        fuel_delta: SDWORD,
        resunit_delta: ::core::ffi::c_int,
    );
    fn GetFuelTankCapacity() -> DWORD;
    fn DrawFlagshipName(InStatusArea: BOOLEAN, NewGame: bool);
    fn DrawCaptainsName(NewGame: bool);
    fn setGameStateUint(name: *const ::core::ffi::c_char, val: DWORD);
    fn getGameStateUint(name: *const ::core::ffi::c_char) -> DWORD;
    static mut GlobData: GLOBDATA;
    static mut RadarContext: CONTEXT;
    fn inSavablePos() -> BOOLEAN;
    fn inHQSpace() -> BOOLEAN;
    fn isPC(optWhich: ::core::ffi::c_int) -> OPT_CONSOLETYPE;
    fn is3DO(optWhich: ::core::ffi::c_int) -> OPT_CONSOLETYPE;
    fn replaceChar(
        pStr: *mut ::core::ffi::c_char,
        find: ::core::ffi::c_char,
        replace: ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn InitLander(LanderFlags: BYTE);
    static mut CurStarDescPtr: *mut STAR_DESC;
    fn FindStar(
        pLastStar: *mut STAR_DESC,
        puniverse: *mut POINT,
        xbounds: SIZE,
        ybounds: SIZE,
    ) -> *mut STAR_DESC;
    fn GetClusterName(pSD: *const STAR_DESC, buf: *mut UNICODE);
    fn CloneShipFragment(
        shipIndex: RACE_ID,
        pDstQueue: *mut QUEUE,
        crew_level: COUNT,
    ) -> HSHIPFRAG;
    fn DoMenuChooser(pMS: *mut MENU_STATE, BaseState: BYTE) -> BOOLEAN;
    fn DrawMenuStateStrings(beg_index: BYTE, NewState: SWORD);
    fn DrawBorder(Visible: BYTE);
    static mut MiscDataFrame: FRAME;
    static mut SpaceContext: CONTEXT;
    static mut StatusContext: CONTEXT;
    static mut StarConFont: FONT;
    static mut MicroFont: FONT;
    static mut TinyFont: FONT;
    static mut TinyFontBold: FONT;
    static mut LabelFont: FONT;
    static mut SquareFont: FONT;
    static mut LastActivity: ACTIVITY;
    fn SetMenuSounds(sound_0: MENU_SOUND_FLAGS, sound_1: MENU_SOUND_FLAGS);
    fn PlayMenuSound(S: MENU_SOUND_EFFECT);
    fn ToggleMusic();
    fn ResumeMusic();
    fn PauseMusic();
    fn ToggleSoundEffect();
    fn LoadGame(
        which_game: COUNT,
        summary_desc: *mut SUMMARY_DESC,
        in_fp: *mut uio_Stream,
        try_core: BOOLEAN,
    ) -> BOOLEAN;
    fn SaveProblem();
    fn SaveGame(
        which_game: COUNT,
        summary_desc: *mut SUMMARY_DESC,
        name: *const ::core::ffi::c_char,
    ) -> BOOLEAN;
    static mut GameStrings: STRING;
    fn SetCursorRect(r: *mut RECT, context: CONTEXT);
    fn FlushCursorRect();
    fn SetCursorFlashBlock(state: BOOLEAN);
    fn DrawStarConBox(
        pRect: *mut RECT,
        BorderWidth: SIZE,
        TopLeftColor: Color,
        BottomRightColor: Color,
        FillInterior: BOOLEAN,
        InteriorColor: Color,
        CreateCorners: BOOLEAN,
        CornerColor: Color,
    );
    fn SaveContextFrame(saveRect: *const RECT) -> STAMP;
    fn SetTransitionSource(pRect: *const RECT);
    fn ScreenTransition(transition: ::core::ffi::c_int, pRect: *const RECT);
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
pub type UniChar = uint32;
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
    pub flags: BYTE,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ScreenFadeType(pub ::core::ffi::c_uint);
impl ScreenFadeType {
    pub const FadeAllToWhite: Self = Self(250);
    pub const FadeSomeToWhite: Self = Self(251);
    pub const FadeAllToBlack: Self = Self(252);
    pub const FadeAllToColor: Self = Self(253);
    pub const FadeSomeToBlack: Self = Self(254);
    pub const FadeSomeToColor: Self = Self(255);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct OPT_CONSOLETYPE(pub ::core::ffi::c_uint);
impl OPT_CONSOLETYPE {
    pub const OPTVAL_PC: Self = Self(0);
    pub const OPTVAL_3DO: Self = Self(1);
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
pub type MENU_STATE = menu_state;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct menu_state {
    pub InputFunc: Option<unsafe extern "C" fn(*mut menu_state) -> BOOLEAN>,
    pub Initialized: SIZE,
    pub CurState: BYTE,
    pub CurFrame: FRAME,
    pub CurString: STRING,
    pub first_item: POINT,
    pub delta_item: SIZE,
    pub ModuleFrame: FRAME,
    pub flash_rect0: RECT,
    pub flash_rect1: RECT,
    pub flash_rect2: RECT,
    pub flash_frame0: FRAME,
    pub flash_frame1: FRAME,
    pub flashContext: *mut FlashContext,
    pub hMusic: MUSIC_REF,
    pub privData: *mut ::core::ffi::c_void,
}
pub type MUSIC_REF = *mut *mut TFB_SoundSample;
pub type TFB_SoundSample = tfb_soundsample;
pub type NamingCallback = unsafe extern "C" fn() -> ();
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct OPT_ENABLABLE(pub ::core::ffi::c_uint);
impl OPT_ENABLABLE {
    pub const OPTVAL_DISABLED: Self = Self(0);
    pub const OPTVAL_ENABLED: Self = Self(1);
}
pub type TEXTENTRY_STATE = textentry_state;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct textentry_state {
    pub InputFunc: Option<unsafe extern "C" fn(*mut textentry_state) -> BOOLEAN>,
    pub Initialized: BOOLEAN,
    pub NextTime: DWORD,
    pub Success: BOOLEAN,
    pub CacheStr: *mut UNICODE,
    pub JoyAlphaString: STRING,
    pub JoystickMode: BOOLEAN,
    pub UpperRegister: BOOLEAN,
    pub JoyAlpha: *mut joy_char_t,
    pub JoyAlphaLength: ::core::ffi::c_int,
    pub JoyUpper: *mut joy_char_t,
    pub JoyLower: *mut joy_char_t,
    pub JoyRegLength: ::core::ffi::c_int,
    pub InsPt: *mut UNICODE,
    pub BaseStr: *mut UNICODE,
    pub CursorPos: ::core::ffi::c_int,
    pub MaxSize: ::core::ffi::c_int,
    pub ChangeCallback: Option<unsafe extern "C" fn(*mut textentry_state) -> BOOLEAN>,
    pub FrameCallback: Option<unsafe extern "C" fn(*mut textentry_state) -> BOOLEAN>,
    pub CbParam: *mut ::core::ffi::c_void,
}
pub type joy_char_t = joy_char;
pub type CONTROLLER_INPUT_STATE = _controller_input_state;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _controller_input_state {
    pub key: [[::core::ffi::c_int; 8]; 2],
    pub menu: [::core::ffi::c_int; 31],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PICK_GAME_STATE {
    pub summary: [SUMMARY_DESC; 51],
    pub saving: BOOLEAN,
    pub success: BOOLEAN,
    pub SummaryFrame: FRAME,
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
    pub SaveNameChecker: [UNICODE; 32],
    pub LegacySaveName: [UNICODE; 32],
    pub res_factor: BYTE,
}
pub type InputFrameCallback = unsafe extern "C" fn() -> ();
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
pub type MENU_SOUND_FLAGS = UWORD;
pub type HSHIPFRAG = HLINK;
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
pub type OBJ_SIZE = UWORD;
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct MENU_SOUND_EFFECT(pub ::core::ffi::c_uint);
impl MENU_SOUND_EFFECT {
    pub const MENU_SOUND_MOVE: Self = Self(0);
    pub const MENU_SOUND_SUCCESS: Self = Self(1);
    pub const MENU_SOUND_FAILURE: Self = Self(2);
    pub const MENU_SOUND_INVOKED: Self = Self(3);
    pub const MENU_SOUND_BUBBLEWARP: Self = Self(4);
    pub const MENU_SOUND_TEXT: Self = Self(5);
}
pub type TimeCount = DWORD;
pub type TimePeriod = DWORD;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed {
    pub const KEY_PAUSE: Self = Self(0);
    pub const KEY_EXIT: Self = Self(1);
    pub const KEY_ABORT: Self = Self(2);
    pub const KEY_DEBUG: Self = Self(3);
    pub const KEY_FULLSCREEN: Self = Self(4);
    pub const KEY_MENU_UP: Self = Self(5);
    pub const KEY_MENU_DOWN: Self = Self(6);
    pub const KEY_MENU_LEFT: Self = Self(7);
    pub const KEY_MENU_RIGHT: Self = Self(8);
    pub const KEY_MENU_SELECT: Self = Self(9);
    pub const KEY_MENU_CANCEL: Self = Self(10);
    pub const KEY_MENU_SPECIAL: Self = Self(11);
    pub const KEY_MENU_PAGE_UP: Self = Self(12);
    pub const KEY_MENU_PAGE_DOWN: Self = Self(13);
    pub const KEY_MENU_HOME: Self = Self(14);
    pub const KEY_MENU_END: Self = Self(15);
    pub const KEY_MENU_ZOOM_IN: Self = Self(16);
    pub const KEY_MENU_ZOOM_OUT: Self = Self(17);
    pub const KEY_MENU_DELETE: Self = Self(18);
    pub const KEY_MENU_BACKSPACE: Self = Self(19);
    pub const KEY_MENU_EDIT_CANCEL: Self = Self(20);
    pub const KEY_MENU_SEARCH: Self = Self(21);
    pub const KEY_MENU_NEXT: Self = Self(22);
    pub const KEY_MENU_TOGGLEMAP: Self = Self(23);
    pub const KEY_SCREENSHOT: Self = Self(24);
    pub const KEY_QUICKSAVE: Self = Self(25);
    pub const KEY_QUICKLOAD: Self = Self(26);
    pub const KEY_DEBUG_2: Self = Self(27);
    pub const KEY_DEBUG_3: Self = Self(28);
    pub const KEY_DEBUG_4: Self = Self(29);
    pub const KEY_MENU_ANY: Self = Self(30);
    pub const NUM_MENU_KEYS: Self = Self(31);
}
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_1(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_1 {
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_3(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_3 {
    pub const PM_SCAN: Self = Self(0);
    pub const PM_STARMAP: Self = Self(1);
    pub const PM_DEVICES: Self = Self(2);
    pub const PM_CARGO: Self = Self(3);
    pub const PM_ROSTER: Self = Self(4);
    pub const PM_GAME_MENU: Self = Self(5);
    pub const PM_NAVIGATE: Self = Self(6);
    pub const PM_MIN_SCAN: Self = Self(7);
    pub const PM_ENE_SCAN: Self = Self(8);
    pub const PM_BIO_SCAN: Self = Self(9);
    pub const PM_EXIT_SCAN: Self = Self(10);
    pub const PM_AUTO_SCAN: Self = Self(11);
    pub const PM_LAUNCH_LANDER: Self = Self(12);
    pub const PM_SAVE_GAME: Self = Self(13);
    pub const PM_LOAD_GAME: Self = Self(14);
    pub const PM_QUIT_GAME: Self = Self(15);
    pub const PM_CHANGE_SETTINGS: Self = Self(16);
    pub const PM_EXIT_GAME_MENU: Self = Self(17);
    pub const PM_CONVERSE: Self = Self(18);
    pub const PM_ATTACK: Self = Self(19);
    pub const PM_ENCOUNTER_GAME_MENU: Self = Self(20);
    pub const PM_FUEL: Self = Self(21);
    pub const PM_MODULE: Self = Self(22);
    pub const PM_OUTFIT_GAME_MENU: Self = Self(23);
    pub const PM_EXIT_OUTFIT: Self = Self(24);
    pub const PM_CREW: Self = Self(25);
    pub const PM_SHIPYARD_GAME_MENU: Self = Self(26);
    pub const PM_EXIT_SHIPYARD: Self = Self(27);
    pub const PM_SOUND_ON: Self = Self(28);
    pub const PM_SOUND_OFF: Self = Self(29);
    pub const PM_MUSIC_ON: Self = Self(30);
    pub const PM_MUSIC_OFF: Self = Self(31);
    pub const PM_CYBORG_OFF: Self = Self(32);
    pub const PM_CYBORG_NORMAL: Self = Self(33);
    pub const PM_CYBORG_DOUBLE: Self = Self(34);
    pub const PM_CYBORG_SUPER: Self = Self(35);
    pub const PM_READ_VERY_SLOW: Self = Self(36);
    pub const PM_READ_SLOW: Self = Self(37);
    pub const PM_READ_MODERATE: Self = Self(38);
    pub const PM_READ_FAST: Self = Self(39);
    pub const PM_READ_VERY_FAST: Self = Self(40);
    pub const PM_CHANGE_CAPTAIN: Self = Self(41);
    pub const PM_CHANGE_SHIP: Self = Self(42);
    pub const PM_EXIT_SETTINGS: Self = Self(43);
    pub const PM_ALT_SCAN: Self = Self(44);
    pub const PM_ALT_STARMAP: Self = Self(45);
    pub const PM_ALT_MANIFEST: Self = Self(46);
    pub const PM_ALT_GAME_MENU: Self = Self(47);
    pub const PM_ALT_NAVIGATE: Self = Self(48);
    pub const PM_ALT_CARGO: Self = Self(49);
    pub const PM_ALT_DEVICES: Self = Self(50);
    pub const PM_ALT_ROSTER: Self = Self(51);
    pub const PM_ALT_EXIT_MANIFEST: Self = Self(52);
    pub const PM_ALT_MSCAN: Self = Self(53);
    pub const PM_ALT_ESCAN: Self = Self(54);
    pub const PM_ALT_BSCAN: Self = Self(55);
    pub const PM_ALT_ASCAN: Self = Self(56);
    pub const PM_ALT_DISPATCH: Self = Self(57);
    pub const PM_ALT_EXIT_SCAN: Self = Self(58);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_4(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_4 {
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_5(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_5 {
    pub const SAVE_GAME: Self = Self(0);
    pub const LOAD_GAME: Self = Self(1);
    pub const QUIT_GAME: Self = Self(2);
    pub const SETTINGS: Self = Self(3);
    pub const EXIT_GAME_MENU: Self = Self(4);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_6(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_6 {
    pub const SOUND_ON_SETTING: Self = Self(0);
    pub const SOUND_OFF_SETTING: Self = Self(1);
    pub const MUSIC_ON_SETTING: Self = Self(2);
    pub const MUSIC_OFF_SETTING: Self = Self(3);
    pub const CYBORG_OFF_SETTING: Self = Self(4);
    pub const CYBORG_NORMAL_SETTING: Self = Self(5);
    pub const CYBORG_DOUBLE_SETTING: Self = Self(6);
    pub const CYBORG_SUPER_SETTING: Self = Self(7);
    pub const READ_VERY_SLOW_SETTING: Self = Self(8);
    pub const READ_SLOW_SETTING: Self = Self(9);
    pub const READ_MODERATE_SETTING: Self = Self(10);
    pub const READ_FAST_SETTING: Self = Self(11);
    pub const READ_VERY_FAST_SETTING: Self = Self(12);
    pub const CHANGE_CAPTAIN_SETTING: Self = Self(13);
    pub const CHANGE_SHIP_SETTING: Self = Self(14);
    pub const EXIT_SETTINGS_MENU: Self = Self(15);
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
static mut lastUsedSlot: COUNT = 0;
static mut namingCB: Option<NamingCallback> = None;
#[no_mangle]
pub static mut NewGameInit: BOOLEAN = BOOLEAN::FALSE;
#[no_mangle]
pub static mut OutfitOrShipyard: BYTE = 0 as BYTE;
#[no_mangle]
pub static mut SaveOrLoad: BOOLEAN = BOOLEAN::FALSE;
#[no_mangle]
pub static mut TextEntry3DO: BOOLEAN = BOOLEAN::FALSE;
#[no_mangle]
pub static mut QuickLoadRequested: BOOLEAN = BOOLEAN::FALSE;
#[no_mangle]
pub unsafe extern "C" fn ConfirmSaveLoad(mut MsgStamp: *mut STAMP) {
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut clip_r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut t: TEXT = TEXT {
        baseline: POINT { x: 0, y: 0 },
        pStr: ::core::ptr::null::<UNICODE>(),
        align: TEXT_ALIGN::ALIGN_LEFT,
        CharCount: 0,
    };
    SetContextFont(StarConFont);
    GetContextClipRect(&raw mut clip_r);
    t.baseline.x = (clip_r.extent.width as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
        as COORD;
    t.baseline.y = ((clip_r.extent.height as ::core::ffi::c_int
        >> 1 as ::core::ffi::c_int) + ((3 as ::core::ffi::c_int) << resolutionFactor))
        as COORD;
    t.align = TEXT_ALIGN::ALIGN_CENTER;
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    if !MsgStamp.is_null() {
        t.pStr = GetStringAddress(
            SetAbsStringTableIndex(
                GameStrings,
                (C2Rust_Unnamed_4::SAVEGAME_STRING_BASE.0 as ::core::ffi::c_int
                    + 0 as ::core::ffi::c_int) as COUNT,
            ),
        );
    } else {
        t.pStr = GetStringAddress(
            SetAbsStringTableIndex(
                GameStrings,
                (C2Rust_Unnamed_4::SAVEGAME_STRING_BASE.0 as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int) as COUNT,
            ),
        );
    }
    TextRect(&raw mut t, &raw mut r, ::core::ptr::null_mut::<BYTE>());
    r.corner.x = (r.corner.x as ::core::ffi::c_int
        - ((4 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    r.corner.y = (r.corner.y as ::core::ffi::c_int
        - ((4 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    r.extent.width = (r.extent.width as ::core::ffi::c_int
        + ((8 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    r.extent.height = (r.extent.height as ::core::ffi::c_int
        + ((8 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    if !MsgStamp.is_null() {
        *MsgStamp = SaveContextFrame(&raw mut r);
    }
    uqm_lua_checked!(DrawStarConBox(
        &raw mut r,
        ((2 as ::core::ffi::c_int) << resolutionFactor) as SIZE,
        colorFromRgb15(0x10 as BYTE, 0x10 as BYTE, 0x10 as BYTE),
        colorFromRgb15(0x8 as BYTE, 0x8 as BYTE, 0x8 as BYTE),
        BOOLEAN::TRUE,
        colorFromRgb15(0xa as BYTE, 0xa as BYTE, 0xa as BYTE),
        BOOLEAN::TRUE,
        buildColorRgba(0x50 as BYTE, 0x50 as BYTE, 0x50 as BYTE, 0 as BYTE),
    ), ());
    SetContextForeGroundColor(
        if isPC(optWhichFonts).0 != 0 {
            colorFromRgb15(0x1f as BYTE, 0x1f as BYTE, 0x1f as BYTE)
        } else {
            colorFromRgb15(0x14 as BYTE, 0x14 as BYTE, 0x14 as BYTE)
        },
    );
    uqm_lua_checked!(font_DrawText(&raw mut t), ());
}
unsafe extern "C" fn FeedbackSetting(mut which_setting: BYTE) {
    let mut buf: [UNICODE; 128] = [0; 128];
    let mut tmpstr: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    buf[0usize] = '\0' as UNICODE;
    buf[::core::mem::size_of::<[UNICODE; 128]>().wrapping_sub(1usize)] = '\0' as UNICODE;
    match which_setting as ::core::ffi::c_int {
        0 | 1 => {
            snprintf(
                &raw mut buf as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[UNICODE; 128]>().wrapping_sub(1 as size_t),
                b"%s %s\0".as_ptr() as *const ::core::ffi::c_char,
                GetStringAddress(
                    SetAbsStringTableIndex(
                        GameStrings,
                        (C2Rust_Unnamed_4::OPTION_STRING_BASE.0 as ::core::ffi::c_int
                            + 0 as ::core::ffi::c_int) as COUNT,
                    ),
                ),
                if GlobData.Game_state.glob_flags as ::core::ffi::c_int
                    & (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int != 0
                {
                    GetStringAddress(
                        SetAbsStringTableIndex(
                            GameStrings,
                            (C2Rust_Unnamed_4::OPTION_STRING_BASE.0 as ::core::ffi::c_int
                                + 3 as ::core::ffi::c_int) as COUNT,
                        ),
                    )
                } else {
                    GetStringAddress(
                        SetAbsStringTableIndex(
                            GameStrings,
                            (C2Rust_Unnamed_4::OPTION_STRING_BASE.0 as ::core::ffi::c_int
                                + 4 as ::core::ffi::c_int) as COUNT,
                        ),
                    )
                },
            );
        }
        2 | 3 => {
            snprintf(
                &raw mut buf as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[UNICODE; 128]>().wrapping_sub(1 as size_t),
                b"%s %s\0".as_ptr() as *const ::core::ffi::c_char,
                GetStringAddress(
                    SetAbsStringTableIndex(
                        GameStrings,
                        (C2Rust_Unnamed_4::OPTION_STRING_BASE.0 as ::core::ffi::c_int
                            + 1 as ::core::ffi::c_int) as COUNT,
                    ),
                ),
                if GlobData.Game_state.glob_flags as ::core::ffi::c_int
                    & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int != 0
                {
                    GetStringAddress(
                        SetAbsStringTableIndex(
                            GameStrings,
                            (C2Rust_Unnamed_4::OPTION_STRING_BASE.0 as ::core::ffi::c_int
                                + 3 as ::core::ffi::c_int) as COUNT,
                        ),
                    )
                } else {
                    GetStringAddress(
                        SetAbsStringTableIndex(
                            GameStrings,
                            (C2Rust_Unnamed_4::OPTION_STRING_BASE.0 as ::core::ffi::c_int
                                + 4 as ::core::ffi::c_int) as COUNT,
                        ),
                    )
                },
            );
        }
        4 | 5 | 6 | 7 => {
            if optWhichMenu == 0x2 as ::core::ffi::c_int
                && which_setting as ::core::ffi::c_int
                    > C2Rust_Unnamed_6::CYBORG_NORMAL_SETTING.0 as ::core::ffi::c_int
            {
                if which_setting as ::core::ffi::c_int
                    == C2Rust_Unnamed_6::CYBORG_DOUBLE_SETTING.0 as ::core::ffi::c_int
                {
                    tmpstr = b"+\0".as_ptr() as *const ::core::ffi::c_char;
                } else {
                    tmpstr = b"++\0".as_ptr() as *const ::core::ffi::c_char;
                }
            } else {
                tmpstr = b"\0".as_ptr() as *const ::core::ffi::c_char;
            }
            snprintf(
                &raw mut buf as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[UNICODE; 128]>().wrapping_sub(1 as size_t),
                b"%s %s%s\0".as_ptr() as *const ::core::ffi::c_char,
                GetStringAddress(
                    SetAbsStringTableIndex(
                        GameStrings,
                        (C2Rust_Unnamed_4::OPTION_STRING_BASE.0 as ::core::ffi::c_int
                            + 2 as ::core::ffi::c_int) as COUNT,
                    ),
                ),
                if GlobData.Game_state.glob_flags as ::core::ffi::c_int
                    & (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int == 0
                {
                    GetStringAddress(
                        SetAbsStringTableIndex(
                            GameStrings,
                            (C2Rust_Unnamed_4::OPTION_STRING_BASE.0 as ::core::ffi::c_int
                                + 3 as ::core::ffi::c_int) as COUNT,
                        ),
                    )
                } else {
                    GetStringAddress(
                        SetAbsStringTableIndex(
                            GameStrings,
                            (C2Rust_Unnamed_4::OPTION_STRING_BASE.0 as ::core::ffi::c_int
                                + 4 as ::core::ffi::c_int) as COUNT,
                        ),
                    )
                },
                tmpstr,
            );
        }
        8 | 9 | 10 | 11 | 12 => {
            snprintf(
                &raw mut buf as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[UNICODE; 128]>().wrapping_sub(1 as size_t),
                b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                GetStringAddress(
                    SetAbsStringTableIndex(
                        GameStrings,
                        (C2Rust_Unnamed_4::OPTION_STRING_BASE.0 as ::core::ffi::c_int
                            + 5 as ::core::ffi::c_int
                            + (GlobData.Game_state.glob_flags as ::core::ffi::c_int
                                & ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                                    - 1 as ::core::ffi::c_int)) as COUNT,
                    ),
                ),
            );
        }
        13 | 14 => {
            utf8StringCopy(
                &raw mut buf as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[UNICODE; 128]>(),
                GetStringAddress(
                    SetAbsStringTableIndex(
                        GameStrings,
                        (C2Rust_Unnamed_4::NAMING_STRING_BASE.0 as ::core::ffi::c_int
                            + if is3DO(optWhichFonts).0 != 0 {
                                7 as ::core::ffi::c_int
                            } else {
                                0 as ::core::ffi::c_int
                            }) as COUNT,
                    ),
                ),
            );
        }
        _ => {}
    }
    uqm_lua_checked!(DrawStatusMessage(&raw mut buf as *mut UNICODE), ());
}
unsafe extern "C" fn DrawNameString(
    mut nameCaptain: bool,
    mut Str: *mut UNICODE,
    mut CursorPos: COUNT,
    mut state: COUNT,
) -> BOOLEAN {
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut lf: TEXT = TEXT {
        baseline: POINT { x: 0, y: 0 },
        pStr: ::core::ptr::null::<UNICODE>(),
        align: TEXT_ALIGN::ALIGN_LEFT,
        CharCount: 0,
    };
    let mut BackGround: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    let mut ForeGround: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    let mut Font: FONT = ::core::ptr::null_mut::<FONT_DESC>();
    r.corner.x = ((2 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    r.extent.width = (((64 as ::core::ffi::c_int) << resolutionFactor)
        - ((4 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    r.extent.height = ((7 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    if nameCaptain {
        if isPC(optWhichFonts).0 != 0 {
            Font = TinyFont;
        } else {
            Font = TinyFontBold;
        }
        r.corner.y = ((10 as ::core::ffi::c_int) << resolutionFactor) as COORD;
        r.corner.x = (r.corner.x as ::core::ffi::c_int
            + ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        r.extent.width = (r.extent.width as ::core::ffi::c_int
            - ((2 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        lf.baseline.x = (r.corner.x as ::core::ffi::c_int
            + (r.extent.width as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
            - ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        BackGround = colorFromRgb15(0xa as BYTE, 0xa as BYTE, 0x1f as BYTE);
        ForeGround = colorFromRgb15(0xa as BYTE, 0x1f as BYTE, 0x1f as BYTE);
    } else {
        Font = StarConFont;
        r.corner.y = ((20 as ::core::ffi::c_int) << resolutionFactor) as COORD;
        lf.baseline.x = (r.corner.x as ::core::ffi::c_int
            + (r.extent.width as ::core::ffi::c_int >> 1 as ::core::ffi::c_int))
            as COORD;
        BackGround = colorFromRgb15(0xf as BYTE, 0 as BYTE, 0 as BYTE);
        ForeGround = colorFromRgb15(0x1f as BYTE, 0xa as BYTE, 0 as BYTE);
    }
    lf.baseline.y = (r.corner.y as ::core::ffi::c_int
        + r.extent.height as ::core::ffi::c_int
        - ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    lf.align = TEXT_ALIGN::ALIGN_CENTER;
    SetContext(StatusContext);
    SetContextFont(Font);
    lf.pStr = Str;
    lf.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    if state as ::core::ffi::c_int & 1 as ::core::ffi::c_int == 0 {
        if nameCaptain {
            uqm_lua_checked!(DrawCaptainsName(nameCaptain), BOOLEAN(0));
        } else {
            uqm_lua_checked!(DrawFlagshipName(BOOLEAN::TRUE, !nameCaptain), BOOLEAN(0));
        }
    } else {
        let mut i: COUNT = 0;
        let mut text_r: RECT = RECT {
            corner: POINT { x: 0, y: 0 },
            extent: EXTENT { width: 0, height: 0 },
        };
        let mut char_deltas: [BYTE; 32] = [0; 32];
        let mut pchar_deltas: *mut BYTE = ::core::ptr::null_mut::<BYTE>();
        TextRect(&raw mut lf, &raw mut text_r, &raw mut char_deltas as *mut BYTE);
        if text_r.extent.width as ::core::ffi::c_int
            + ((2 as ::core::ffi::c_int) << resolutionFactor)
            >= r.extent.width as ::core::ffi::c_int
        {
            return BOOLEAN::FALSE;
        }
        SetContextForeGroundColor(BackGround);
        uqm_lua_checked!(DrawFilledRectangle(&raw mut r), BOOLEAN(0));
        if optCustomBorder.0 != 0 {
            uqm_lua_checked!(DrawBorder(
                C2Rust_Unnamed_1::SIS_STAT_REPAIR_FRAME.0 as ::core::ffi::c_int as BYTE,
            ), BOOLEAN(0));
        }
        pchar_deltas = &raw mut char_deltas as *mut BYTE;
        i = CursorPos;
        while i as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
            let c2rust_fresh1 = pchar_deltas;
            pchar_deltas = pchar_deltas.offset(1);
            text_r.corner.x = (text_r.corner.x as ::core::ffi::c_int
                + *c2rust_fresh1 as ::core::ffi::c_int) as COORD;
            i = i.wrapping_sub(1);
        }
        if (CursorPos as ::core::ffi::c_int) < lf.CharCount as ::core::ffi::c_int {
            text_r.corner.x = (text_r.corner.x as ::core::ffi::c_int
                - ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        }
        if state as ::core::ffi::c_int & 2 as ::core::ffi::c_int != 0 {
            text_r.corner.y = r.corner.y;
            text_r.extent.height = r.extent.height;
            SetCursorFlashBlock(BOOLEAN::TRUE);
            if CursorPos as ::core::ffi::c_int == lf.CharCount as ::core::ffi::c_int {
                text_r.extent.width = ((1 as ::core::ffi::c_int) << resolutionFactor)
                    as COORD;
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
            } else if CursorPos as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                == lf.CharCount as ::core::ffi::c_int
            {
                text_r.extent.width = (*pchar_deltas as SIZE as ::core::ffi::c_int
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
                text_r.corner.x = (text_r.corner.x as ::core::ffi::c_int
                    + ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
            } else {
                text_r.extent.width = *pchar_deltas as SIZE as COORD;
                text_r.corner.x = (text_r.corner.x as ::core::ffi::c_int
                    + ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
            }
            if text_r.extent.width as ::core::ffi::c_int >= 200 as ::core::ffi::c_int {
                text_r.extent.width = ((1 as ::core::ffi::c_int) << resolutionFactor)
                    as COORD;
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
            } else {
                SetContextForeGroundColor(
                    colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE),
                );
                uqm_lua_checked!(DrawFilledRectangle(&raw mut text_r), BOOLEAN(0));
            }
        } else {
            text_r.corner.y = (r.corner.y as ::core::ffi::c_int
                + ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
            text_r.extent.height = (r.extent.height as ::core::ffi::c_int
                - ((2 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
            text_r.extent.width = ((1 as ::core::ffi::c_int) << resolutionFactor)
                as COORD;
            if CursorPos as ::core::ffi::c_int == lf.CharCount as ::core::ffi::c_int {
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
        SetCursorRect(&raw mut text_r, StatusContext);
        SetContextForeGroundColor(ForeGround);
        uqm_lua_checked!(font_DrawText(&raw mut lf), BOOLEAN(0));
    }
    return BOOLEAN::TRUE;
}
unsafe extern "C" fn OnNameChange(mut pTES: *mut TEXTENTRY_STATE) -> BOOLEAN {
    let mut nameCaptain: bool = !(*pTES).CbParam.is_null();
    let mut hl: COUNT = 1 as COUNT;
    if (*pTES).JoystickMode.0 != 0 {
        hl = (hl as ::core::ffi::c_int | 2 as ::core::ffi::c_int) as COUNT;
    }
    return uqm_lua_checked!(DrawNameString(nameCaptain, (*pTES).BaseStr, (*pTES).CursorPos as COUNT, hl), BOOLEAN(0));
}
unsafe extern "C" fn NameCaptainOrShip(
    mut nameCaptain: BOOLEAN,
    mut gamestart: BOOLEAN,
) {
    let mut buf: [UNICODE; 32] = ::core::mem::transmute::<
        [u8; 32],
        [UNICODE; 32],
    >(*b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0");
    let mut tes: TEXTENTRY_STATE = TEXTENTRY_STATE {
        InputFunc: None,
        Initialized: BOOLEAN::FALSE,
        NextTime: 0,
        Success: BOOLEAN::FALSE,
        CacheStr: ::core::ptr::null_mut::<UNICODE>(),
        JoyAlphaString: ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>(),
        JoystickMode: BOOLEAN::FALSE,
        UpperRegister: BOOLEAN::FALSE,
        JoyAlpha: ::core::ptr::null_mut::<joy_char_t>(),
        JoyAlphaLength: 0,
        JoyUpper: ::core::ptr::null_mut::<joy_char_t>(),
        JoyLower: ::core::ptr::null_mut::<joy_char_t>(),
        JoyRegLength: 0,
        InsPt: ::core::ptr::null_mut::<UNICODE>(),
        BaseStr: ::core::ptr::null_mut::<UNICODE>(),
        CursorPos: 0,
        MaxSize: 0,
        ChangeCallback: None,
        FrameCallback: None,
        CbParam: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    let mut Setting: *mut UNICODE = ::core::ptr::null_mut::<UNICODE>();
    let mut CursPos: COUNT = 0 as COUNT;
    SetContext(StatusContext);
    uqm_lua_checked!(DrawNameString(
        nameCaptain.0 != 0,
        &raw mut buf as *mut UNICODE,
        CursPos,
        1 as COUNT,
    ), ());
    uqm_lua_checked!(DrawStatusMessage(
        GetStringAddress(
            SetAbsStringTableIndex(
                GameStrings,
                (C2Rust_Unnamed_4::NAMING_STRING_BASE.0 as ::core::ffi::c_int
                    + if is3DO(optWhichFonts).0 != 0 {
                        7 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    }) as COUNT,
            ),
        ),
    ), ());
    if nameCaptain.0 != 0 {
        Setting = &raw mut GlobData.SIS_state.CommanderName as *mut UNICODE;
        tes.MaxSize = ::core::mem::size_of::<[UNICODE; 32]>() as ::core::ffi::c_int;
        TextEntry3DO = BOOLEAN(is3DO(optWhichFonts).0);
    } else {
        Setting = &raw mut GlobData.SIS_state.ShipName as *mut UNICODE;
        tes.MaxSize = ::core::mem::size_of::<[UNICODE; 32]>() as ::core::ffi::c_int;
        TextEntry3DO = BOOLEAN::FALSE;
    }
    tes.Initialized = BOOLEAN::FALSE;
    tes.BaseStr = &raw mut buf as *mut UNICODE;
    tes.CursorPos = CursPos as ::core::ffi::c_int;
    tes.CbParam = ::core::ptr::with_exposed_provenance_mut::<
        ::core::ffi::c_void,
    >(nameCaptain.0 as usize);
    tes.ChangeCallback = Some(
        OnNameChange as unsafe extern "C" fn(*mut TEXTENTRY_STATE) -> BOOLEAN,
    ) as Option<unsafe extern "C" fn(*mut textentry_state) -> BOOLEAN>;
    tes.FrameCallback = None;
    if uqm_lua_checked!(DoTextEntry(&raw mut tes), ()).0 != 0 {
        utf8StringCopy(Setting, tes.MaxSize as size_t, &raw mut buf as *mut UNICODE);
    } else {
        utf8StringCopy(
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[UNICODE; 32]>(),
            Setting,
        );
    }
    if gamestart.0 != 0 && !Setting.is_null()
        && *Setting.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
    {
        strcpy(
            Setting,
            GetStringAddress(
                SetAbsStringTableIndex(
                    GameStrings,
                    ((C2Rust_Unnamed_4::NAMING_STRING_BASE.0 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                        .wrapping_add(nameCaptain.0) as COUNT,
                ),
            ),
        );
        CursPos = strlen(
            GetStringAddress(
                SetAbsStringTableIndex(
                    GameStrings,
                    ((C2Rust_Unnamed_4::NAMING_STRING_BASE.0 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                        .wrapping_add(nameCaptain.0) as COUNT,
                ),
            ),
        ) as COUNT;
    }
    if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
        & C2Rust_Unnamed_2::CHECK_ABORT.0 as ::core::ffi::c_int != 0
    {
        NewGameInit = BOOLEAN::FALSE;
    }
    FlushCursorRect();
    uqm_lua_checked!(SetFlashRect(
        ::core::ptr::with_exposed_provenance_mut::<RECT>(!(0 as ::core::ffi::c_long) as usize),
        BOOLEAN::FALSE,
    ), ());
    uqm_lua_checked!(DrawNameString(
        nameCaptain.0 != 0,
        &raw mut buf as *mut UNICODE,
        CursPos,
        0 as COUNT,
    ), ());
    uqm_lua_checked!(DrawBorder(C2Rust_Unnamed_1::SIS_STAT_REPAIR_FRAME.0 as ::core::ffi::c_int as BYTE), ());
    TextEntry3DO = BOOLEAN::FALSE;
    if namingCB.is_some() {
        uqm_lua_checked!(namingCB.expect("non-null function pointer")(), ());
    }
}
unsafe extern "C" fn DrawSaveNameString(
    mut Str: *mut UNICODE,
    mut CursorPos: COUNT,
    mut state: COUNT,
    mut gameIndex: COUNT,
) -> BOOLEAN {
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut lf: TEXT = TEXT {
        baseline: POINT { x: 0, y: 0 },
        pStr: ::core::ptr::null::<UNICODE>(),
        align: TEXT_ALIGN::ALIGN_LEFT,
        CharCount: 0,
    };
    let mut BackGround: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    let mut ForeGround: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    let mut Font: FONT = ::core::ptr::null_mut::<FONT_DESC>();
    let mut fullStr: [UNICODE; 256] = [0; 256];
    let mut dateStr: [UNICODE; 80] = [0; 80];
    DateToString(
        &raw mut dateStr as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[UNICODE; 80]>(),
        GlobData.Game_state.GameClock.month_index,
        GlobData.Game_state.GameClock.day_index,
        GlobData.Game_state.GameClock.year_index,
    );
    strncat(
        &raw mut dateStr as *mut ::core::ffi::c_char,
        b": \0".as_ptr() as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[UNICODE; 80]>()
            .wrapping_sub(strlen(&raw mut dateStr as *mut UNICODE))
            .wrapping_sub(1 as size_t),
    );
    snprintf(
        &raw mut fullStr as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[UNICODE; 256]>(),
        b"%s%s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut dateStr as *mut UNICODE,
        Str,
    );
    SetContextForeGroundColor(colorFromRgb15(0x1b as BYTE, 0 as BYTE, 0x1b as BYTE));
    r.extent.width = ((15 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    if 50 as ::core::ffi::c_int > 99 as ::core::ffi::c_int {
        r.extent.width = (r.extent.width as ::core::ffi::c_int
            + ((5 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    }
    r.extent.height = ((11 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    r.corner.x = ((8 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    r.corner.y = ((160 as ::core::ffi::c_int
        + gameIndex as ::core::ffi::c_int
            % if optWindowType < 2 as DWORD {
                2 as ::core::ffi::c_int
            } else {
                5 as ::core::ffi::c_int
            } * 13 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    uqm_lua_checked!(DrawRectangle(
        &raw mut r,
        BOOLEAN(
            (if resolutionFactor != 2 as ::core::ffi::c_uint {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            }) as ::core::ffi::c_uint,
        ),
    ), BOOLEAN(0));
    r.extent.width = (((204 as ::core::ffi::c_int) << resolutionFactor)
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
            - ((242 as ::core::ffi::c_int) << resolutionFactor))) as COORD;
    r.corner.x = ((30 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    uqm_lua_checked!(DrawRectangle(
        &raw mut r,
        BOOLEAN(
            (if resolutionFactor != 2 as ::core::ffi::c_uint {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            }) as ::core::ffi::c_uint,
        ),
    ), BOOLEAN(0));
    Font = TinyFont;
    lf.baseline.x = (r.corner.x as ::core::ffi::c_int
        + ((3 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    lf.baseline.y = (r.corner.y as ::core::ffi::c_int
        + ((8 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    BackGround = colorFromRgb15(0x1b as BYTE, 0 as BYTE, 0x1b as BYTE);
    ForeGround = colorFromRgb15(0 as BYTE, 0 as BYTE, 0x14 as BYTE);
    lf.align = TEXT_ALIGN::ALIGN_LEFT;
    SetContextFont(Font);
    lf.pStr = &raw mut fullStr as *mut UNICODE;
    lf.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    if state as ::core::ffi::c_int & 1 as ::core::ffi::c_int == 0 {
        let mut t: TEXT = TEXT {
            baseline: POINT { x: 0, y: 0 },
            pStr: ::core::ptr::null::<UNICODE>(),
            align: TEXT_ALIGN::ALIGN_LEFT,
            CharCount: 0,
        };
        SetContextForeGroundColor(colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE));
        uqm_lua_checked!(DrawFilledRectangle(&raw mut r), BOOLEAN(0));
        t.baseline.x = (r.corner.x as ::core::ffi::c_int
            + ((3 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        t.baseline.y = (r.corner.y as ::core::ffi::c_int
            + ((8 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        t.align = TEXT_ALIGN::ALIGN_LEFT;
        t.pStr = Str;
        t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
        SetContextForeGroundColor(
            colorFromRgb15(0x16 as BYTE, 0xb as BYTE, 0x1f as BYTE),
        );
        uqm_lua_checked!(font_DrawText(&raw mut t), BOOLEAN(0));
    } else {
        let mut i: COUNT = 0;
        let mut FullCursorPos: COUNT = 0;
        let mut text_r: RECT = RECT {
            corner: POINT { x: 0, y: 0 },
            extent: EXTENT { width: 0, height: 0 },
        };
        let mut char_deltas: [BYTE; 256] = [0; 256];
        let mut pchar_deltas: *mut BYTE = ::core::ptr::null_mut::<BYTE>();
        TextRect(&raw mut lf, &raw mut text_r, &raw mut char_deltas as *mut BYTE);
        if text_r.extent.width as ::core::ffi::c_int
            + ((2 as ::core::ffi::c_int) << resolutionFactor)
            >= r.extent.width as ::core::ffi::c_int
        {
            return BOOLEAN::FALSE;
        }
        uqm_lua_checked!(PreUpdateFlashRect(), BOOLEAN(0));
        SetContextForeGroundColor(BackGround);
        uqm_lua_checked!(DrawFilledRectangle(&raw mut r), BOOLEAN(0));
        pchar_deltas = &raw mut char_deltas as *mut BYTE;
        FullCursorPos = (CursorPos as ::core::ffi::c_int
            + utf8StringCount(&raw mut dateStr as *mut UNICODE) as COUNT
                as ::core::ffi::c_int) as COUNT;
        i = FullCursorPos;
        while i as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
            let c2rust_fresh2 = pchar_deltas;
            pchar_deltas = pchar_deltas.offset(1);
            text_r.corner.x = (text_r.corner.x as ::core::ffi::c_int
                + *c2rust_fresh2 as ::core::ffi::c_int) as COORD;
            i = i.wrapping_sub(1);
        }
        if (FullCursorPos as ::core::ffi::c_int) < lf.CharCount as ::core::ffi::c_int {
            text_r.corner.x = (text_r.corner.x as ::core::ffi::c_int
                - ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        }
        if state as ::core::ffi::c_int & 2 as ::core::ffi::c_int != 0 {
            if FullCursorPos as ::core::ffi::c_int == lf.CharCount as ::core::ffi::c_int
            {
                text_r.extent.width = ((1 as ::core::ffi::c_int) << resolutionFactor)
                    as COORD;
            } else if FullCursorPos as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                == lf.CharCount as ::core::ffi::c_int
            {
                text_r.extent.width = (*pchar_deltas as SIZE as ::core::ffi::c_int
                    + ((2 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
            } else {
                text_r.extent.width = (*pchar_deltas as SIZE as ::core::ffi::c_int
                    + ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
            }
        } else {
            text_r.extent.width = ((1 as ::core::ffi::c_int) << resolutionFactor)
                as COORD;
        }
        text_r.corner.y = r.corner.y;
        text_r.extent.height = r.extent.height;
        SetContextForeGroundColor(colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE));
        uqm_lua_checked!(DrawFilledRectangle(&raw mut text_r), BOOLEAN(0));
        SetContextForeGroundColor(ForeGround);
        uqm_lua_checked!(font_DrawText(&raw mut lf), BOOLEAN(0));
        uqm_lua_checked!(PostUpdateFlashRect(), BOOLEAN(0));
    }
    return BOOLEAN::TRUE;
}
unsafe extern "C" fn OnSaveNameChange(mut pTES: *mut TEXTENTRY_STATE) -> BOOLEAN {
    let mut hl: COUNT = 1 as COUNT;
    let mut gameIndex: *mut COUNT = (*pTES).CbParam as *mut COUNT;
    if (*pTES).JoystickMode.0 != 0 {
        hl = (hl as ::core::ffi::c_int | 2 as ::core::ffi::c_int) as COUNT;
    }
    return uqm_lua_checked!(DrawSaveNameString(
        (*pTES).BaseStr,
        (*pTES).CursorPos as COUNT,
        hl,
        *gameIndex,
    ), BOOLEAN(0));
}
unsafe extern "C" fn NameSaveGame(
    mut gameIndex: COUNT,
    mut buf: *mut UNICODE,
) -> BOOLEAN {
    let mut tes: TEXTENTRY_STATE = TEXTENTRY_STATE {
        InputFunc: None,
        Initialized: BOOLEAN::FALSE,
        NextTime: 0,
        Success: BOOLEAN::FALSE,
        CacheStr: ::core::ptr::null_mut::<UNICODE>(),
        JoyAlphaString: ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>(),
        JoystickMode: BOOLEAN::FALSE,
        UpperRegister: BOOLEAN::FALSE,
        JoyAlpha: ::core::ptr::null_mut::<joy_char_t>(),
        JoyAlphaLength: 0,
        JoyUpper: ::core::ptr::null_mut::<joy_char_t>(),
        JoyLower: ::core::ptr::null_mut::<joy_char_t>(),
        JoyRegLength: 0,
        InsPt: ::core::ptr::null_mut::<UNICODE>(),
        BaseStr: ::core::ptr::null_mut::<UNICODE>(),
        CursorPos: 0,
        MaxSize: 0,
        ChangeCallback: None,
        FrameCallback: None,
        CbParam: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    let mut CursPos: COUNT = strlen(buf) as COUNT;
    let mut gIndex: *mut COUNT = HMalloc(::core::mem::size_of::<COUNT>()) as *mut COUNT;
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    *gIndex = gameIndex;
    uqm_lua_checked!(DrawSaveNameString(buf, CursPos, 1 as COUNT, gameIndex), BOOLEAN(0));
    tes.MaxSize = 64 as ::core::ffi::c_int;
    tes.Initialized = BOOLEAN::FALSE;
    tes.BaseStr = buf;
    tes.CursorPos = CursPos as ::core::ffi::c_int;
    tes.CbParam = gIndex as *mut ::core::ffi::c_void;
    tes.ChangeCallback = Some(
        OnSaveNameChange as unsafe extern "C" fn(*mut TEXTENTRY_STATE) -> BOOLEAN,
    ) as Option<unsafe extern "C" fn(*mut textentry_state) -> BOOLEAN>;
    tes.FrameCallback = None;
    r.extent.width = (((204 as ::core::ffi::c_int) << resolutionFactor)
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
            - ((242 as ::core::ffi::c_int) << resolutionFactor))) as COORD;
    r.extent.height = ((11 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    r.corner.x = ((30 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    r.corner.y = (((160 as ::core::ffi::c_int) << resolutionFactor)
        + gameIndex as ::core::ffi::c_int
            % if optWindowType < 2 as DWORD {
                2 as ::core::ffi::c_int
            } else {
                5 as ::core::ffi::c_int
            } * ((13 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    uqm_lua_checked!(SetFlashRect(&raw mut r, BOOLEAN::FALSE), BOOLEAN(0));
    if uqm_lua_checked!(DoTextEntry(&raw mut tes), BOOLEAN(0)).0 == 0 {
        *buf.offset(0isize) = 0 as UNICODE;
    }
    uqm_lua_checked!(SetFlashRect(::core::ptr::null::<RECT>(), BOOLEAN::FALSE), BOOLEAN(0));
    uqm_lua_checked!(DrawSaveNameString(buf, CursPos, 0 as COUNT, gameIndex), BOOLEAN(0));
    if namingCB.is_some() {
        uqm_lua_checked!(namingCB.expect("non-null function pointer")(), BOOLEAN(0));
    }
    SetMenuSounds(
        (((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as MENU_SOUND_FLAGS
            as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as MENU_SOUND_FLAGS
                as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as MENU_SOUND_FLAGS
                as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as MENU_SOUND_FLAGS
                as ::core::ffi::c_int) as MENU_SOUND_FLAGS,
        ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as MENU_SOUND_FLAGS,
    );
    HFree(gIndex as *mut ::core::ffi::c_void);
    uqm_lua_checked!(SetFlashRect(::core::ptr::null::<RECT>(), BOOLEAN::FALSE), BOOLEAN(0));
    if tes.Success.0 != 0 { return BOOLEAN::TRUE } else { return BOOLEAN::FALSE };
}
#[no_mangle]
pub unsafe extern "C" fn SetNamingCallback(mut callback: Option<NamingCallback>) {
    namingCB = callback;
}
unsafe extern "C" fn DoSettings(mut pMS: *mut MENU_STATE) -> BOOLEAN {
    let mut cur_speed: BYTE = 0;
    let mut read_speed: BYTE = 0;
    static mut i: BYTE = 0 as BYTE;
    if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
        & (C2Rust_Unnamed_2::CHECK_ABORT.0 as ::core::ffi::c_int
            | C2Rust_Unnamed_2::CHECK_LOAD.0 as ::core::ffi::c_int) != 0
    {
        i = 0 as BYTE;
        return BOOLEAN::FALSE;
    }
    cur_speed = ((GlobData.Game_state.glob_flags as ::core::ffi::c_int
        & (((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
            - 1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
        >> 6 as ::core::ffi::c_int) as BYTE;
    read_speed = (GlobData.Game_state.glob_flags as ::core::ffi::c_int
        & ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
            - 1 as ::core::ffi::c_int) as BYTE;
    if NewGameInit.0 != 0 {
        i = i.wrapping_add(1);
        (*pMS).CurState = (if i as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
            C2Rust_Unnamed_6::CHANGE_SHIP_SETTING.0 as ::core::ffi::c_int
        } else {
            C2Rust_Unnamed_6::CHANGE_CAPTAIN_SETTING.0 as ::core::ffi::c_int
        }) as BYTE;
        PulsedInputState
            .menu[C2Rust_Unnamed::KEY_MENU_SELECT.0 as ::core::ffi::c_int as usize] = 65535
            as ::core::ffi::c_int;
    }
    if PulsedInputState
        .menu[C2Rust_Unnamed::KEY_MENU_CANCEL.0 as ::core::ffi::c_int as usize] != 0
        || PulsedInputState
            .menu[C2Rust_Unnamed::KEY_MENU_SELECT.0 as ::core::ffi::c_int as usize] != 0
            && (*pMS).CurState as ::core::ffi::c_int
                == C2Rust_Unnamed_6::EXIT_SETTINGS_MENU.0 as ::core::ffi::c_int
    {
        return BOOLEAN::FALSE
    } else if PulsedInputState
        .menu[C2Rust_Unnamed::KEY_MENU_SELECT.0 as ::core::ffi::c_int as usize] != 0
    {
        match (*pMS).CurState as ::core::ffi::c_int {
            0 | 1 => {
                ToggleSoundEffect();
                (*pMS).CurState = ((*pMS).CurState as ::core::ffi::c_int
                    ^ 1 as ::core::ffi::c_int) as BYTE;
                uqm_lua_checked!(DrawMenuStateStrings(
                    C2Rust_Unnamed_3::PM_SOUND_ON.0 as ::core::ffi::c_int as BYTE,
                    (*pMS).CurState as SWORD,
                ), BOOLEAN(0));
            }
            2 | 3 => {
                uqm_lua_checked!(ToggleMusic(), BOOLEAN(0));
                (*pMS).CurState = ((*pMS).CurState as ::core::ffi::c_int
                    ^ 1 as ::core::ffi::c_int) as BYTE;
                uqm_lua_checked!(DrawMenuStateStrings(
                    C2Rust_Unnamed_3::PM_SOUND_ON.0 as ::core::ffi::c_int as BYTE,
                    (*pMS).CurState as SWORD,
                ), BOOLEAN(0));
            }
            13 | 14 => {
                uqm_lua_checked!(SetFlashRect(::core::ptr::null::<RECT>(), BOOLEAN::FALSE), BOOLEAN(0));
                uqm_lua_checked!(DrawMenuStateStrings(
                    C2Rust_Unnamed_3::PM_SOUND_ON.0 as ::core::ffi::c_int as BYTE,
                    (*pMS).CurState as SWORD,
                ), BOOLEAN(0));
                uqm_lua_checked!(NameCaptainOrShip(
                    BOOLEAN(
                        ((*pMS).CurState as ::core::ffi::c_int
                            == C2Rust_Unnamed_6::CHANGE_CAPTAIN_SETTING.0
                                as ::core::ffi::c_int) as ::core::ffi::c_int
                            as ::core::ffi::c_uint,
                    ),
                    NewGameInit,
                ), BOOLEAN(0));
            }
            4 | 5 | 6 | 7 => {
                let c2rust_fresh0 = cur_speed;
                cur_speed = cur_speed.wrapping_add(1);
                if (c2rust_fresh0 as ::core::ffi::c_int)
                    < 4 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                {
                    GlobData.Game_state.glob_flags = (GlobData.Game_state.glob_flags
                        as ::core::ffi::c_int
                        | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as BYTE;
                } else {
                    cur_speed = 0 as BYTE;
                    GlobData.Game_state.glob_flags = (GlobData.Game_state.glob_flags
                        as ::core::ffi::c_int
                        & !((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int))
                        as BYTE;
                }
                GlobData.Game_state.glob_flags = (GlobData.Game_state.glob_flags
                    as ::core::ffi::c_int
                    & !((((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                    | (cur_speed as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                    as BYTE;
                (*pMS).CurState = (C2Rust_Unnamed_6::CYBORG_OFF_SETTING.0
                    as ::core::ffi::c_int + cur_speed as ::core::ffi::c_int) as BYTE;
                uqm_lua_checked!(DrawMenuStateStrings(
                    C2Rust_Unnamed_3::PM_SOUND_ON.0 as ::core::ffi::c_int as BYTE,
                    (*pMS).CurState as SWORD,
                ), BOOLEAN(0));
            }
            8 | 9 | 10 | 11 | 12 => {
                if read_speed as ::core::ffi::c_int
                    == 5 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                {
                    read_speed = 0 as BYTE;
                } else {
                    read_speed = read_speed.wrapping_add(1);
                }
                GlobData.Game_state.glob_flags = (GlobData.Game_state.glob_flags
                    as ::core::ffi::c_int
                    & !(((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int) | read_speed as ::core::ffi::c_int)
                    as BYTE;
                (*pMS).CurState = (C2Rust_Unnamed_6::READ_VERY_SLOW_SETTING.0
                    as ::core::ffi::c_int + read_speed as ::core::ffi::c_int) as BYTE;
                uqm_lua_checked!(DrawMenuStateStrings(
                    C2Rust_Unnamed_3::PM_SOUND_ON.0 as ::core::ffi::c_int as BYTE,
                    (*pMS).CurState as SWORD,
                ), BOOLEAN(0));
            }
            _ => {
                uqm_lua_checked!(DrawMenuStateStrings(
                    C2Rust_Unnamed_3::PM_SOUND_ON.0 as ::core::ffi::c_int as BYTE,
                    (*pMS).CurState as SWORD,
                ), BOOLEAN(0));
            }
        }
        if optWhichMenu == 0x2 as ::core::ffi::c_int {
            uqm_lua_checked!(DrawMenuStateStrings(
                C2Rust_Unnamed_3::PM_SOUND_ON.0 as ::core::ffi::c_int as BYTE,
                (*pMS).CurState as SWORD,
            ), BOOLEAN(0));
        }
        uqm_lua_checked!(FeedbackSetting((*pMS).CurState), BOOLEAN(0));
    } else if uqm_lua_checked!(DoMenuChooser(
            pMS,
            C2Rust_Unnamed_3::PM_SOUND_ON.0 as ::core::ffi::c_int as BYTE,
        ), BOOLEAN(0))
        .0 != 0
    {
        uqm_lua_checked!(FeedbackSetting((*pMS).CurState), BOOLEAN(0));
    }
    if NewGameInit.0 != 0 {
        if i as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            uqm_lua_checked!(SettingsMenu(BOOLEAN::TRUE), BOOLEAN(0));
            return BOOLEAN::FALSE;
        } else {
            NewGameInit = BOOLEAN::FALSE;
            i = 0 as BYTE;
        }
    }
    return BOOLEAN::TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn SettingsMenu(mut NameFlagship: BOOLEAN) {
    let mut MenuState: MENU_STATE = MENU_STATE {
        InputFunc: None,
        Initialized: 0,
        CurState: 0,
        CurFrame: ::core::ptr::null_mut::<FRAME_DESC>(),
        CurString: ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>(),
        first_item: POINT { x: 0, y: 0 },
        delta_item: 0,
        ModuleFrame: ::core::ptr::null_mut::<FRAME_DESC>(),
        flash_rect0: RECT {
            corner: POINT { x: 0, y: 0 },
            extent: EXTENT { width: 0, height: 0 },
        },
        flash_rect1: RECT {
            corner: POINT { x: 0, y: 0 },
            extent: EXTENT { width: 0, height: 0 },
        },
        flash_rect2: RECT {
            corner: POINT { x: 0, y: 0 },
            extent: EXTENT { width: 0, height: 0 },
        },
        flash_frame0: ::core::ptr::null_mut::<FRAME_DESC>(),
        flash_frame1: ::core::ptr::null_mut::<FRAME_DESC>(),
        flashContext: ::core::ptr::null_mut::<FlashContext>(),
        hMusic: ::core::ptr::null_mut::<*mut TFB_SoundSample>(),
        privData: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    memset(
        &raw mut MenuState as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<MENU_STATE>(),
    );
    if NewGameInit.0 != 0 {
        MenuState.CurState = (if NameFlagship.0 == 0 {
            C2Rust_Unnamed_6::CHANGE_CAPTAIN_SETTING.0 as ::core::ffi::c_int
        } else {
            C2Rust_Unnamed_6::CHANGE_SHIP_SETTING.0 as ::core::ffi::c_int
        }) as BYTE;
    } else {
        MenuState.CurState = C2Rust_Unnamed_6::SOUND_ON_SETTING.0 as ::core::ffi::c_int
            as BYTE;
    }
    uqm_lua_checked!(DrawMenuStateStrings(
        C2Rust_Unnamed_3::PM_SOUND_ON.0 as ::core::ffi::c_int as BYTE,
        MenuState.CurState as SWORD,
    ), ());
    uqm_lua_checked!(FeedbackSetting(MenuState.CurState), ());
    MenuState.InputFunc = Some(
        DoSettings as unsafe extern "C" fn(*mut MENU_STATE) -> BOOLEAN,
    ) as Option<unsafe extern "C" fn(*mut menu_state) -> BOOLEAN>;
    uqm_lua_checked!(DoInput(&raw mut MenuState as *mut ::core::ffi::c_void, BOOLEAN::FALSE), ());
    uqm_lua_checked!(DrawStatusMessage(::core::ptr::null::<UNICODE>()), ());
}
unsafe extern "C" fn DrawLines(mut pt: POINT, mut width: SIZE) {
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    r.corner = pt;
    r.extent.width = width as COORD;
    r.extent.height = ((1 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    SetContextForeGroundColor(
        buildColorRgba(0x6 as BYTE, 0xc as BYTE, 0xba as BYTE, 0xff as BYTE),
    );
    uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
    r.corner.y = (r.corner.y as ::core::ffi::c_int
        + ((4 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    SetContextForeGroundColor(
        buildColorRgba(0x3 as BYTE, 0x6 as BYTE, 0x5d as BYTE, 0xff as BYTE),
    );
    uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
    r.corner.y = (r.corner.y as ::core::ffi::c_int
        - ((2 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    r.corner.x = (r.corner.x as ::core::ffi::c_int
        + ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    r.extent.width = (r.extent.width as ::core::ffi::c_int
        - ((2 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    SetContextForeGroundColor(
        buildColorRgba(0x5 as BYTE, 0xa as BYTE, 0x9b as BYTE, 0xff as BYTE),
    );
    uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
}
unsafe extern "C" fn SCL_POINT(mut pt: POINT) -> POINT {
    let mut point: POINT = POINT { x: 0, y: 0 };
    point.x = ((pt.x as ::core::ffi::c_int) << resolutionFactor) as COORD;
    point.y = ((pt.y as ::core::ffi::c_int) << resolutionFactor) as COORD;
    return point;
}
unsafe extern "C" fn DrawLabel(mut pt: POINT, mut width: SIZE, mut gamestr: DWORD) {
    let mut t: TEXT = TEXT {
        baseline: POINT { x: 0, y: 0 },
        pStr: ::core::ptr::null::<UNICODE>(),
        align: TEXT_ALIGN::ALIGN_LEFT,
        CharCount: 0,
    };
    let mut buf: [UNICODE; 256] = [0; 256];
    let mut leading: SIZE = 0;
    let mut OldFont: FONT = ::core::ptr::null_mut::<FONT_DESC>();
    let mut OldColor: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    uqm_lua_checked!(DrawLines(
        SCL_POINT(pt),
        ((width as ::core::ffi::c_int) << resolutionFactor) as SIZE,
    ), ());
    utf8StringCopy(
        &raw mut buf as *mut UNICODE,
        ::core::mem::size_of::<[UNICODE; 256]>(),
        GetStringAddress(SetAbsStringTableIndex(GameStrings, gamestr as COUNT)),
    );
    OldFont = SetContextFont(LabelFont);
    OldColor = SetContextForeGroundColor(
        buildColorRgba(0xba as BYTE, 0 as BYTE, 0xba as BYTE, 0xff as BYTE),
    );
    GetContextFontLeading(&raw mut leading);
    t.baseline.x = (((width as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
        + pt.x as ::core::ffi::c_int) << resolutionFactor) as COORD;
    t.baseline.y = ((pt.y as ::core::ffi::c_int + 5 as ::core::ffi::c_int)
        << resolutionFactor) as COORD;
    t.align = TEXT_ALIGN::ALIGN_CENTER;
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    t.pStr = AlignText(
        &raw mut buf as *mut UNICODE as *const UNICODE,
        &raw mut t.baseline.x,
    );
    uqm_lua_checked!(font_DrawText(&raw mut t), ());
    SetContextFont(OldFont);
    SetContextForeGroundColor(OldColor);
}
unsafe extern "C" fn DrawAllLabels(mut rect: RECT) {
    let mut pt: POINT = POINT { x: 0, y: 0 };
    let mut width: SIZE = 0;
    let mut i: COUNT = 0;
    let mut OldColor: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    i = 4 as COUNT;
    while i as ::core::ffi::c_int <= 8 as ::core::ffi::c_int {
        if strlen(
            GetStringAddress(
                SetAbsStringTableIndex(
                    GameStrings,
                    (C2Rust_Unnamed_4::LABEL_STRING_BASE.0 as ::core::ffi::c_int
                        + i as ::core::ffi::c_int) as COUNT,
                ),
            ),
        ) == 0
        {
            return;
        }
        i = i.wrapping_add(1);
    }
    OldColor = SetContextForeGroundColor(
        colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE),
    );
    rect.corner.x = (rect.corner.x as ::core::ffi::c_int
        + ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    rect.corner.y = (rect.corner.y as ::core::ffi::c_int
        + ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    rect.extent.width = (rect.extent.width as ::core::ffi::c_int
        - ((2 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    rect.extent.height = (137 as ::core::ffi::c_int
        - if if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        } << resolutionFactor) as COORD;
    uqm_lua_checked!(DrawFilledRectangle(&raw mut rect), ());
    SetContextForeGroundColor(OldColor);
    width = 69 as SIZE;
    pt.x = (11 as ::core::ffi::c_int
        - if if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            9 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as COORD;
    pt.y = 5 as COORD;
    uqm_lua_checked!(DrawLabel(
        pt,
        width,
        (C2Rust_Unnamed_4::LABEL_STRING_BASE.0 as ::core::ffi::c_int
            + 4 as ::core::ffi::c_int) as DWORD,
    ), ());
    pt.y = 76 as COORD;
    uqm_lua_checked!(DrawLabel(
        pt,
        width,
        (C2Rust_Unnamed_4::LABEL_STRING_BASE.0 as ::core::ffi::c_int
            + 5 as ::core::ffi::c_int) as DWORD,
    ), ());
    width = 72 as SIZE;
    pt.x = (158 as ::core::ffi::c_int
        - if if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            20 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as COORD;
    pt.y = 5 as COORD;
    uqm_lua_checked!(DrawLabel(
        pt,
        width,
        (C2Rust_Unnamed_4::LABEL_STRING_BASE.0 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int) as DWORD,
    ), ());
    pt.y = 87 as COORD;
    uqm_lua_checked!(DrawLabel(
        pt,
        width,
        (C2Rust_Unnamed_4::LABEL_STRING_BASE.0 as ::core::ffi::c_int
            + 7 as ::core::ffi::c_int) as DWORD,
    ), ());
    pt.y = 111 as COORD;
    uqm_lua_checked!(DrawLabel(
        pt,
        width,
        (C2Rust_Unnamed_4::LABEL_STRING_BASE.0 as ::core::ffi::c_int
            + 8 as ::core::ffi::c_int) as DWORD,
    ), ());
}
unsafe extern "C" fn DrawBlankSavegameDisplay(mut pickState: *mut PICK_GAME_STATE) {
    let mut s: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    let mut rect: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    s.origin.x = 0 as COORD;
    s.origin.y = 0 as COORD;
    s.frame = SetAbsFrameIndex(
        (*pickState).SummaryFrame,
        (GetFrameCount((*pickState).SummaryFrame) as ::core::ffi::c_int
            - 1 as ::core::ffi::c_int) as COUNT,
    );
    uqm_lua_checked!(DrawStamp(&raw mut s), ());
    GetFrameRect(s.frame, &raw mut rect);
    rect.corner.x = (rect.corner.x as ::core::ffi::c_int
        + s.origin.x as ::core::ffi::c_int) as COORD;
    rect.corner.y = (rect.corner.y as ::core::ffi::c_int
        + s.origin.y as ::core::ffi::c_int) as COORD;
    uqm_lua_checked!(DrawAllLabels(rect), ());
}
unsafe extern "C" fn DrawSaveLoad(mut pickState: *mut PICK_GAME_STATE) {
    let mut s: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    s.frame = SetAbsFrameIndex(
        (*pickState).SummaryFrame,
        (GetFrameCount((*pickState).SummaryFrame) as ::core::ffi::c_int
            - 2 as ::core::ffi::c_int) as COUNT,
    );
    GetFrameRect(s.frame, &raw mut r);
    s.origin.x = ((((CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
            } << resolutionFactor)) >> resolutionFactor)
        - (r.extent.width as ::core::ffi::c_int >> resolutionFactor))
        / 2 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    s.origin.y = 0 as COORD;
    r.corner.x = ((1 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    r.corner.y = (r.corner.y as ::core::ffi::c_int
        - (if if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            1 as ::core::ffi::c_int
        } else {
            2 as ::core::ffi::c_int
        } << resolutionFactor)) as COORD;
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
            } << resolutionFactor))
        - (if if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            1 as ::core::ffi::c_int
        } else {
            2 as ::core::ffi::c_int
        } << resolutionFactor)) as COORD;
    r.extent.height = (r.extent.height as ::core::ffi::c_int
        + (if if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            3 as ::core::ffi::c_int
        } else {
            4 as ::core::ffi::c_int
        } << resolutionFactor)) as COORD;
    SetContextForeGroundColor(colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE));
    uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
    if (*pickState).saving.0 != 0 {
        s.frame = DecFrameIndex(s.frame);
    }
    uqm_lua_checked!(DrawStamp(&raw mut s), ());
}
unsafe extern "C" fn DrawSaveLoadText(mut pickState: *mut PICK_GAME_STATE) {
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut OldFont: FONT = ::core::ptr::null_mut::<FONT_DESC>();
    let mut OldColor: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    let mut text: TEXT = TEXT {
        baseline: POINT { x: 0, y: 0 },
        pStr: ::core::ptr::null::<UNICODE>(),
        align: TEXT_ALIGN::ALIGN_LEFT,
        CharCount: 0,
    };
    let mut buf: [UNICODE; 256] = [0; 256];
    let mut leading: SIZE = 0;
    if strlen(
        GetStringAddress(
            SetAbsStringTableIndex(
                GameStrings,
                C2Rust_Unnamed_4::LABEL_STRING_BASE
                    .0
                    .wrapping_add(
                        (3 as ::core::ffi::c_uint).wrapping_sub((*pickState).saving.0),
                    ) as COUNT,
            ),
        ),
    ) == 0
    {
        uqm_lua_checked!(DrawSaveLoad(pickState), ());
        return;
    }
    r.corner.x = ((1 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    r.corner.y = (((151 as ::core::ffi::c_int) << resolutionFactor)
        - (if if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            1 as ::core::ffi::c_int
        } else {
            2 as ::core::ffi::c_int
        } << resolutionFactor)) as COORD;
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
            } << resolutionFactor))
        - (if if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            1 as ::core::ffi::c_int
        } else {
            2 as ::core::ffi::c_int
        } << resolutionFactor)) as COORD;
    r.extent.height = (((7 as ::core::ffi::c_int) << resolutionFactor)
        + (if if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            3 as ::core::ffi::c_int
        } else {
            4 as ::core::ffi::c_int
        } << resolutionFactor)) as COORD;
    SetContextForeGroundColor(colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE));
    uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
    OldFont = SetContextFont(
        if if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            LabelFont
        } else {
            MicroFont
        },
    );
    OldColor = SetContextForeGroundColor(
        if if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            buildColorRgba(0x7b as BYTE, 0x83 as BYTE, 0xde as BYTE, 0xff as BYTE)
        } else {
            buildColorRgba(0xba as BYTE, 0 as BYTE, 0xba as BYTE, 0xff as BYTE)
        },
    );
    GetContextFontLeading(&raw mut leading);
    text.baseline.x = (((CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
            } << resolutionFactor)) >> resolutionFactor) / 2 as ::core::ffi::c_int)
        << resolutionFactor) as COORD;
    text.baseline.y = (((5 as ::core::ffi::c_int) << resolutionFactor)
        + ((151 as ::core::ffi::c_int) << resolutionFactor)
        + (if if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            1 as ::core::ffi::c_int
        } else {
            2 as ::core::ffi::c_int
        } << resolutionFactor)) as COORD;
    utf8StringCopy(
        &raw mut buf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[UNICODE; 256]>(),
        GetStringAddress(
            SetAbsStringTableIndex(
                GameStrings,
                C2Rust_Unnamed_4::LABEL_STRING_BASE
                    .0
                    .wrapping_add(
                        (3 as ::core::ffi::c_uint).wrapping_sub((*pickState).saving.0),
                    ) as COUNT,
            ),
        ),
    );
    text.align = TEXT_ALIGN::ALIGN_CENTER;
    text.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    text.pStr = &raw mut buf as *mut UNICODE;
    uqm_lua_checked!(font_DrawText(&raw mut text), ());
    SetContextFont(OldFont);
    SetContextForeGroundColor(OldColor);
}
unsafe extern "C" fn DrawSavegameCargo(mut sisState: *mut SIS_STATE) {
    let mut i: COUNT = 0;
    let mut s: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    let mut t: TEXT = TEXT {
        baseline: POINT { x: 0, y: 0 },
        pStr: ::core::ptr::null::<UNICODE>(),
        align: TEXT_ALIGN::ALIGN_LEFT,
        CharCount: 0,
    };
    let mut buf: [UNICODE; 40] = [0; 40];
    static mut cargo_color: [Color; 9] = [
        Color {
            r: ((0x2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0xe as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0xe as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0x13 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x13 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
        Color {
            r: ((0x19 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x19 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
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
            r: ((0x3 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x3 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0x5 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x5 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0x1e as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x1e as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
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
            r: ((0x1b as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x1b as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0x1b as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x1b as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
        Color {
            r: ((0x1e as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x1e as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0xd as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0xd as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
        Color {
            r: ((0x14 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x14 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0x14 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x14 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
        Color {
            r: ((0xf as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0xf as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0x19 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x19 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
    ];
    SetContext(SpaceContext);
    BatchGraphics();
    SetContextFont(StarConFont);
    s.frame = SetAbsFrameIndex(
        MiscDataFrame,
        (((4 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
            + 3 as ::core::ffi::c_int) as COUNT,
    );
    s.origin.x = (((7 as ::core::ffi::c_int) << resolutionFactor)
        + (if if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            0 as ::core::ffi::c_int
        } else {
            14 as ::core::ffi::c_int
        } << resolutionFactor)
        - (if if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            0 as ::core::ffi::c_int
        } else {
            7 as ::core::ffi::c_int
        } << resolutionFactor)
        + (if if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            0 as ::core::ffi::c_int
        } else {
            3 as ::core::ffi::c_int
        } << resolutionFactor)) as COORD;
    s.origin.y = ((17 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    t.baseline.x = (((33 as ::core::ffi::c_int) << resolutionFactor)
        + (if if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            0 as ::core::ffi::c_int
        } else {
            14 as ::core::ffi::c_int
        } << resolutionFactor)
        - (if if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            0 as ::core::ffi::c_int
        } else {
            7 as ::core::ffi::c_int
        } << resolutionFactor)
        + (if if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            0 as ::core::ffi::c_int
        } else {
            3 as ::core::ffi::c_int
        } << resolutionFactor)) as COORD;
    t.baseline.y = (((17 as ::core::ffi::c_int) << resolutionFactor)
        + ((3 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    t.align = TEXT_ALIGN::ALIGN_RIGHT;
    t.pStr = &raw mut buf as *mut UNICODE;
    i = 0 as COUNT;
    while (i as ::core::ffi::c_int)
        < C2Rust_Unnamed_0::NUM_ELEMENT_CATEGORIES.0 as ::core::ffi::c_int
    {
        if i as ::core::ffi::c_int
            == C2Rust_Unnamed_0::NUM_ELEMENT_CATEGORIES.0 as ::core::ffi::c_int
                / 2 as ::core::ffi::c_int
        {
            s.origin.x = (s.origin.x as ::core::ffi::c_int
                + ((36 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
            s.origin.y = ((17 as ::core::ffi::c_int) << resolutionFactor) as COORD;
            t.baseline.x = (t.baseline.x as ::core::ffi::c_int
                + ((36 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
            t.baseline.y = (((17 as ::core::ffi::c_int) << resolutionFactor)
                + ((3 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        }
        uqm_lua_checked!(DrawStamp(&raw mut s), ());
        s.frame = SetRelFrameIndex(s.frame, 5 as SIZE);
        s.origin.y = (s.origin.y as ::core::ffi::c_int
            + ((12 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        SetContextForeGroundColor(cargo_color[i as usize]);
        snprintf(
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[UNICODE; 40]>(),
            b"%u\0".as_ptr() as *const ::core::ffi::c_char,
            (*sisState).ElementAmounts[i as usize] as ::core::ffi::c_int,
        );
        t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
        uqm_lua_checked!(font_DrawText(&raw mut t), ());
        t.baseline.y = (t.baseline.y as ::core::ffi::c_int
            + ((12 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        i = i.wrapping_add(1);
    }
    s.origin.x = (((24 as ::core::ffi::c_int) << resolutionFactor)
        + (if if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            0 as ::core::ffi::c_int
        } else {
            14 as ::core::ffi::c_int
        } << resolutionFactor)
        - (if if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            0 as ::core::ffi::c_int
        } else {
            7 as ::core::ffi::c_int
        } << resolutionFactor)) as COORD;
    s.origin.y = ((68 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    s.frame = SetAbsFrameIndex(s.frame, 68 as COUNT);
    uqm_lua_checked!(DrawStamp(&raw mut s), ());
    t.baseline.x = (((50 as ::core::ffi::c_int) << resolutionFactor)
        + (if if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            0 as ::core::ffi::c_int
        } else {
            14 as ::core::ffi::c_int
        } << resolutionFactor)) as COORD;
    t.baseline.y = (s.origin.y as ::core::ffi::c_int
        + ((3 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    SetContextForeGroundColor(cargo_color[i as usize]);
    snprintf(
        &raw mut buf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[UNICODE; 40]>(),
        b"%u\0".as_ptr() as *const ::core::ffi::c_char,
        (*sisState).TotalBioMass as ::core::ffi::c_int,
    );
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    uqm_lua_checked!(font_DrawText(&raw mut t), ());
    UnbatchGraphics();
}
unsafe extern "C" fn DrawEmptySlot() {
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
    let mut buf: [UNICODE; 256] = [0; 256];
    let mut oldfg: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    let mut stroke: BYTE = 0;
    let mut offset: POINT = POINT { x: 0, y: 0 };
    let mut t_baseline: POINT = POINT { x: 0, y: 0 };
    let mut leading: SIZE = 0;
    r.corner.x = ((1 as ::core::ffi::c_int) << resolutionFactor) as COORD;
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
            } << resolutionFactor))
        - (if if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            1 as ::core::ffi::c_int
        } else {
            2 as ::core::ffi::c_int
        } << resolutionFactor)) as COORD;
    r.extent.height = (if if optWindowType == 1 as DWORD {
        BOOLEAN::TRUE.0 as ::core::ffi::c_int
    } else {
        BOOLEAN::FALSE.0 as ::core::ffi::c_int
    } != 0
    {
        136 as ::core::ffi::c_int
    } else {
        146 as ::core::ffi::c_int
    } << resolutionFactor) as COORD;
    SetContextForeGroundColor(colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE));
    uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
    SetContextFont(LabelFont);
    GetContextFontLeading(&raw mut leading);
    t.baseline.x = (CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
            } << resolutionFactor)) >> 1 as ::core::ffi::c_int) as COORD;
    t.baseline.y = ((66 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    utf8StringCopy(
        &raw mut buf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[UNICODE; 256]>(),
        GetStringAddress(
            SetAbsStringTableIndex(
                GameStrings,
                (C2Rust_Unnamed_4::LABEL_STRING_BASE.0 as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int) as COUNT,
            ),
        ),
    );
    t.align = TEXT_ALIGN::ALIGN_CENTER;
    oldfg = SetContextForeGroundColor(
        buildColorRgba(0x1f as BYTE, 0 as BYTE, 0x1f as BYTE, 0xff as BYTE),
    );
    t_baseline = t.baseline;
    stroke = ((3 as ::core::ffi::c_int) << resolutionFactor) as BYTE;
    while stroke as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
        if stroke as ::core::ffi::c_int == (2 as ::core::ffi::c_int) << resolutionFactor
        {
            SetContextForeGroundColor(
                buildColorRgba(0x3e as BYTE, 0 as BYTE, 0x3e as BYTE, 0xff as BYTE),
            );
        } else if stroke as ::core::ffi::c_int
            == (1 as ::core::ffi::c_int) << resolutionFactor
        {
            SetContextForeGroundColor(
                buildColorRgba(0x5d as BYTE, 0 as BYTE, 0x5d as BYTE, 0xff as BYTE),
            );
        }
        offset.x = -(stroke as ::core::ffi::c_int) as COORD;
        while offset.x as ::core::ffi::c_int <= stroke as ::core::ffi::c_int {
            offset.y = -(stroke as ::core::ffi::c_int) as COORD;
            while offset.y as ::core::ffi::c_int <= stroke as ::core::ffi::c_int {
                if hypot(
                    offset.x as ::core::ffi::c_double,
                    offset.y as ::core::ffi::c_double,
                ) <= stroke as ::core::ffi::c_int as ::core::ffi::c_double
                {
                    t.baseline = MAKE_POINT(
                        (t_baseline.x as ::core::ffi::c_int
                            + offset.x as ::core::ffi::c_int) as COORD,
                        (t_baseline.y as ::core::ffi::c_int
                            + offset.y as ::core::ffi::c_int) as COORD,
                    );
                    t.pStr = &raw mut buf as *mut UNICODE;
                    t.CharCount = utf8StringPos(
                        &raw mut buf as *mut UNICODE,
                        ' ' as UniChar,
                    ) as COUNT;
                    uqm_lua_checked!(font_DrawText(&raw mut t), ());
                    t.baseline.y = (t.baseline.y as ::core::ffi::c_int
                        + leading as ::core::ffi::c_int) as COORD;
                    t.pStr = skipUTF8Chars(
                        &raw mut buf as *mut UNICODE,
                        t.CharCount as size_t,
                    );
                    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
                    uqm_lua_checked!(font_DrawText(&raw mut t), ());
                    t.baseline.y = (t.baseline.y as ::core::ffi::c_int
                        - leading as ::core::ffi::c_int) as COORD;
                }
                offset.y += 1;
            }
            offset.x += 1;
        }
        stroke = (stroke as ::core::ffi::c_int
            - ((1 as ::core::ffi::c_int) << resolutionFactor)) as BYTE;
    }
    t.baseline = t_baseline;
    SetContextForeGroundColor(
        buildColorRgba(0xba as BYTE, 0 as BYTE, 0xba as BYTE, 0xff as BYTE),
    );
    t.pStr = &raw mut buf as *mut UNICODE;
    t.CharCount = utf8StringPos(&raw mut buf as *mut UNICODE, ' ' as UniChar) as COUNT;
    uqm_lua_checked!(font_DrawText(&raw mut t), ());
    t.baseline.y = (t.baseline.y as ::core::ffi::c_int + leading as ::core::ffi::c_int)
        as COORD;
    t.pStr = skipUTF8Chars(&raw mut buf as *mut UNICODE, t.CharCount as size_t);
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    uqm_lua_checked!(font_DrawText(&raw mut t), ());
    SetContextForeGroundColor(oldfg);
}
unsafe extern "C" fn DrawBombPodText(mut s: *mut STAMP) {
    let mut t: TEXT = TEXT {
        baseline: POINT { x: 0, y: 0 },
        pStr: ::core::ptr::null::<UNICODE>(),
        align: TEXT_ALIGN::ALIGN_LEFT,
        CharCount: 0,
    };
    let mut buf: [UNICODE; 256] = [0; 256];
    let mut leading: SIZE = 0;
    let mut OldFont: FONT = ::core::ptr::null_mut::<FONT_DESC>();
    let mut OldColor: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut og_baseline_x: COORD = 0 as COORD;
    if strlen(
        GetStringAddress(
            SetAbsStringTableIndex(
                GameStrings,
                (C2Rust_Unnamed_4::LABEL_STRING_BASE.0 as ::core::ffi::c_int
                    + 9 as ::core::ffi::c_int) as COUNT,
            ),
        ),
    ) == 0
        || strlen(
            GetStringAddress(
                SetAbsStringTableIndex(
                    GameStrings,
                    (C2Rust_Unnamed_4::LABEL_STRING_BASE.0 as ::core::ffi::c_int
                        + 10 as ::core::ffi::c_int) as COUNT,
                ),
            ),
        ) == 0
    {
        return;
    }
    OldFont = SetContextFont(SquareFont);
    OldColor = SetContextForeGroundColor(
        colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE),
    );
    GetContextFontLeading(&raw mut leading);
    GetFrameRect((*s).frame, &raw mut r);
    r.corner.y = (r.corner.y as ::core::ffi::c_int
        + ((32 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    r.extent.height = ((29 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
    utf8StringCopy(
        &raw mut buf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[UNICODE; 256]>(),
        GetStringAddress(
            SetAbsStringTableIndex(
                GameStrings,
                (C2Rust_Unnamed_4::LABEL_STRING_BASE.0 as ::core::ffi::c_int
                    + 9 as ::core::ffi::c_int) as COUNT,
            ),
        ),
    );
    t.baseline.x = ((r.extent.width as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
        + r.corner.x as ::core::ffi::c_int + (*s).origin.x as ::core::ffi::c_int)
        as COORD;
    t.baseline.y = (r.corner.y as ::core::ffi::c_int
        + ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    og_baseline_x = t.baseline.x;
    t.align = TEXT_ALIGN::ALIGN_CENTER;
    t.pStr = strtok(
        &raw mut buf as *mut ::core::ffi::c_char,
        b" \0".as_ptr() as *const ::core::ffi::c_char,
    );
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    SetContextForeGroundColor(
        buildColorRgba(0 as BYTE, 0xe0 as BYTE, 0 as BYTE, 0xff as BYTE),
    );
    while !t.pStr.is_null() {
        t.pStr = AlignText(t.pStr, &raw mut t.baseline.x);
        uqm_lua_checked!(font_DrawText(&raw mut t), ());
        t.pStr = strtok(
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
            b" \0".as_ptr() as *const ::core::ffi::c_char,
        );
        t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
        t.baseline.y = (t.baseline.y as ::core::ffi::c_int
            + leading as ::core::ffi::c_int) as COORD;
    }
    r.corner.y = (r.corner.y as ::core::ffi::c_int
        + ((78 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    r.extent.height = ((15 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    SetContextForeGroundColor(colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE));
    uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
    utf8StringCopy(
        &raw mut buf as *mut UNICODE,
        ::core::mem::size_of::<[UNICODE; 256]>(),
        GetStringAddress(
            SetAbsStringTableIndex(
                GameStrings,
                (C2Rust_Unnamed_4::LABEL_STRING_BASE.0 as ::core::ffi::c_int
                    + 10 as ::core::ffi::c_int) as COUNT,
            ),
        ),
    );
    t.baseline.x = og_baseline_x;
    t.baseline.y = (r.corner.y as ::core::ffi::c_int
        + ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    t.pStr = strtok(
        &raw mut buf as *mut ::core::ffi::c_char,
        b" \0".as_ptr() as *const ::core::ffi::c_char,
    );
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    SetContextForeGroundColor(
        buildColorRgba(0 as BYTE, 0xe0 as BYTE, 0 as BYTE, 0xff as BYTE),
    );
    while !t.pStr.is_null() {
        t.pStr = AlignText(t.pStr, &raw mut t.baseline.x);
        uqm_lua_checked!(font_DrawText(&raw mut t), ());
        t.pStr = strtok(
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
            b" \0".as_ptr() as *const ::core::ffi::c_char,
        );
        t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
        t.baseline.y = (t.baseline.y as ::core::ffi::c_int
            + leading as ::core::ffi::c_int) as COORD;
    }
    SetContextForeGroundColor(OldColor);
    SetContextFont(OldFont);
}
unsafe extern "C" fn DrawNoLimit() {
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
    let mut oldfg: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    let mut OldFont: FONT = ::core::ptr::null_mut::<FONT_DESC>();
    let mut stroke: BYTE = 0;
    let mut offset: POINT = POINT { x: 0, y: 0 };
    let mut t_baseline: POINT = POINT { x: 0, y: 0 };
    let mut leading: SIZE = 0;
    r.corner.x = (((137 as ::core::ffi::c_int) << resolutionFactor)
        + (if if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            0 as ::core::ffi::c_int
        } else {
            14 as ::core::ffi::c_int
        } << resolutionFactor)
        + (if if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            0 as ::core::ffi::c_int
        } else {
            7 as ::core::ffi::c_int
        } << resolutionFactor)) as COORD;
    r.corner.y = ((93 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    r.extent.width = ((75 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    r.extent.height = ((17 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    SetContextForeGroundColor(colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE));
    uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
    OldFont = SetContextFont(LabelFont);
    GetContextFontLeading(&raw mut leading);
    t.baseline.x = ((r.extent.width as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
        + r.corner.x as ::core::ffi::c_int
        - if if resolutionFactor != 2 as ::core::ffi::c_uint {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } == 0
        {
            0 as ::core::ffi::c_int
        } else {
            2 as ::core::ffi::c_int
        }) as COORD;
    t.baseline.y = (r.corner.y as ::core::ffi::c_int + leading as ::core::ffi::c_int)
        as COORD;
    t.pStr = GetStringAddress(
        SetAbsStringTableIndex(
            GameStrings,
            C2Rust_Unnamed_4::LABEL_STRING_BASE.0 as ::core::ffi::c_int as COUNT,
        ),
    );
    t.align = TEXT_ALIGN::ALIGN_CENTER;
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    oldfg = SetContextForeGroundColor(
        buildColorRgba(0x2 as BYTE, 0x4 as BYTE, 0x3e as BYTE, 0xff as BYTE),
    );
    t_baseline = t.baseline;
    stroke = ((3 as ::core::ffi::c_int) << resolutionFactor) as BYTE;
    while stroke as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
        if stroke as ::core::ffi::c_int == (2 as ::core::ffi::c_int) << resolutionFactor
        {
            SetContextForeGroundColor(
                buildColorRgba(0x3 as BYTE, 0x6 as BYTE, 0x5d as BYTE, 0xff as BYTE),
            );
        } else if stroke as ::core::ffi::c_int
            == (1 as ::core::ffi::c_int) << resolutionFactor
        {
            SetContextForeGroundColor(
                buildColorRgba(0x4 as BYTE, 0x8 as BYTE, 0x7c as BYTE, 0xff as BYTE),
            );
        }
        offset.x = -(stroke as ::core::ffi::c_int) as COORD;
        while offset.x as ::core::ffi::c_int <= stroke as ::core::ffi::c_int {
            offset.y = -(stroke as ::core::ffi::c_int) as COORD;
            while offset.y as ::core::ffi::c_int <= stroke as ::core::ffi::c_int {
                if hypot(
                    offset.x as ::core::ffi::c_double,
                    offset.y as ::core::ffi::c_double,
                ) <= stroke as ::core::ffi::c_int as ::core::ffi::c_double
                {
                    t.baseline = MAKE_POINT(
                        (t_baseline.x as ::core::ffi::c_int
                            + offset.x as ::core::ffi::c_int) as COORD,
                        (t_baseline.y as ::core::ffi::c_int
                            + offset.y as ::core::ffi::c_int) as COORD,
                    );
                    uqm_lua_checked!(font_DrawText(&raw mut t), ());
                }
                offset.y += 1;
            }
            offset.x += 1;
        }
        stroke = (stroke as ::core::ffi::c_int
            - ((1 as ::core::ffi::c_int) << resolutionFactor)) as BYTE;
    }
    t.baseline = t_baseline;
    SetContextForeGroundColor(
        buildColorRgba(0x8 as BYTE, 0x10 as BYTE, 0xf8 as BYTE, 0xff as BYTE),
    );
    uqm_lua_checked!(font_DrawText(&raw mut t), ());
    SetContextForeGroundColor(oldfg);
    SetContextFont(OldFont);
}
unsafe extern "C" fn DrawSavegameSummary(
    mut pickState: *mut PICK_GAME_STATE,
    mut gameIndex: COUNT,
) {
    let mut pSD: *mut SUMMARY_DESC = (&raw mut (*pickState).summary as *mut SUMMARY_DESC)
        .offset(gameIndex as ::core::ffi::c_int as isize);
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut s: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    BatchGraphics();
    SaveOrLoad = BOOLEAN::TRUE;
    if (*pSD).year_index as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        uqm_lua_checked!(DrawSaveInfo(SIS_STATE {
            log_x: 0 as SDWORD,
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
            ShipName: [0; 32],
            CommanderName: [0; 32],
            PlanetName: [0; 32],
            Difficulty: 0,
            Extended: 0,
            Nomad: 0,
            ShipSeed: 0,
            Seed: 0,
            SaveVersion: 0,
        }), ());
        uqm_lua_checked!(DrawEmptySlot(), ());
    } else {
        let mut i: COUNT = 0;
        let mut OldRect: RECT = RECT {
            corner: POINT { x: 0, y: 0 },
            extent: EXTENT { width: 0, height: 0 },
        };
        let mut t: TEXT = TEXT {
            baseline: POINT { x: 0, y: 0 },
            pStr: ::core::ptr::null::<UNICODE>(),
            align: TEXT_ALIGN::ALIGN_LEFT,
            CharCount: 0,
        };
        let mut player_built_q: QUEUE = QUEUE {
            head: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            tail: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            pq_tab: ::core::ptr::null_mut::<BYTE>(),
            free_list: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            object_size: 0,
            num_objects: 0,
        };
        let mut OldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
        let mut SaveSS: SIS_STATE = SIS_STATE {
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
            ShipName: [0; 32],
            CommanderName: [0; 32],
            PlanetName: [0; 32],
            Difficulty: 0,
            Extended: 0,
            Nomad: 0,
            ShipSeed: 0,
            Seed: 0,
            SaveVersion: 0,
        };
        let mut buf: [UNICODE; 256] = [0; 256];
        let mut starPt: POINT = POINT { x: 0, y: 0 };
        SaveSS = GlobData.SIS_state;
        player_built_q = GlobData.Game_state.built_ship_q;
        OldContext = SetContext(StatusContext);
        GetContextClipRect(&raw mut OldRect);
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
            + ((CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
                - ((64 as ::core::ffi::c_int) << resolutionFactor) >> resolutionFactor
                >> 1 as ::core::ffi::c_int) << resolutionFactor)
            - (if if optWindowType == 1 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                0 as ::core::ffi::c_int
            } else {
                16 as ::core::ffi::c_int
            } << resolutionFactor)
            + (if if optWindowType == 1 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                0 as ::core::ffi::c_int
            } else {
                14 as ::core::ffi::c_int
            } << resolutionFactor)) as COORD;
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
            }) as COORD;
        r.extent.width = ((64 as ::core::ffi::c_int) << resolutionFactor) as COORD;
        r.extent.height = CanvasHeight as COORD;
        SetContextClipRect(&raw mut r);
        GlobData.SIS_state = (*pSD).SS;
        uqm_lua_checked!(DrawSaveInfo(GlobData.SIS_state), ());
        InitQueue(
            &raw mut GlobData.Game_state.built_ship_q,
            12 as COUNT,
            ::core::mem::size_of::<SHIP_FRAGMENT>() as OBJ_SIZE,
        );
        i = 0 as COUNT;
        while (i as ::core::ffi::c_int) < (*pSD).NumShips as ::core::ffi::c_int {
            CloneShipFragment(
                RACE_ID((*pSD).ShipList[i as usize] as ::core::ffi::c_uint),
                &raw mut GlobData.Game_state.built_ship_q,
                0 as COUNT,
            );
            i = i.wrapping_add(1);
        }
        DateToString(
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[UNICODE; 256]>(),
            (*pSD).month_index,
            (*pSD).day_index,
            (*pSD).year_index,
        );
        uqm_lua_checked!(ClearSISRect(((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as BYTE), ());
        uqm_lua_checked!(DrawBorder(
            C2Rust_Unnamed_1::SIS_STAT_SUMM_FRAME.0 as ::core::ffi::c_int as BYTE,
        ), ());
        uqm_lua_checked!(DrawStatusMessage(&raw mut buf as *mut UNICODE), ());
        UninitQueue(&raw mut GlobData.Game_state.built_ship_q);
        SetContextClipRect(&raw mut OldRect);
        SetContext(SpaceContext);
        s.origin.y = ((13 as ::core::ffi::c_int) << resolutionFactor) as COORD;
        i = 0 as COUNT;
        while (i as ::core::ffi::c_int) < 4 as ::core::ffi::c_int {
            let mut j: COUNT = 0;
            s.origin.x = (((140 as ::core::ffi::c_int) << resolutionFactor)
                + (if if optWindowType == 1 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    0 as ::core::ffi::c_int
                } else {
                    14 as ::core::ffi::c_int
                } << resolutionFactor)
                + (if if optWindowType == 1 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    0 as ::core::ffi::c_int
                } else {
                    7 as ::core::ffi::c_int
                } << resolutionFactor)) as COORD;
            j = 0 as COUNT;
            while (j as ::core::ffi::c_int) < 4 as ::core::ffi::c_int {
                let mut devIndex: COUNT = (i as ::core::ffi::c_int
                    * 4 as ::core::ffi::c_int + j as ::core::ffi::c_int) as COUNT;
                if (devIndex as ::core::ffi::c_int)
                    < (*pSD).NumDevices as ::core::ffi::c_int
                {
                    s.frame = SetAbsFrameIndex(
                        MiscDataFrame,
                        (77 as ::core::ffi::c_int
                            + (*pSD).DeviceList[devIndex as usize] as ::core::ffi::c_int)
                            as COUNT,
                    );
                    uqm_lua_checked!(DrawStamp(&raw mut s), ());
                }
                s.origin.x = (s.origin.x as ::core::ffi::c_int
                    + ((18 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
                j = j.wrapping_add(1);
            }
            s.origin.y = (s.origin.y as ::core::ffi::c_int
                + ((18 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
            i = i.wrapping_add(1);
        }
        SetContextFont(StarConFont);
        t.baseline.x = (((173 as ::core::ffi::c_int) << resolutionFactor)
            + (if if optWindowType == 1 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                0 as ::core::ffi::c_int
            } else {
                14 as ::core::ffi::c_int
            } << resolutionFactor)
            + (if if optWindowType == 1 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                0 as ::core::ffi::c_int
            } else {
                7 as ::core::ffi::c_int
            } << resolutionFactor)) as COORD;
        t.align = TEXT_ALIGN::ALIGN_CENTER;
        t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
        t.pStr = &raw mut buf as *mut UNICODE;
        if (*pSD).Flags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int != 0
        {
            s.origin.x = ((if if optWindowType == 1 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                0 as ::core::ffi::c_int
            } else {
                14 as ::core::ffi::c_int
            } << resolutionFactor)
                - (if if optWindowType == 1 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    0 as ::core::ffi::c_int
                } else {
                    7 as ::core::ffi::c_int
                } << resolutionFactor)
                + (if if optWindowType == 1 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    0 as ::core::ffi::c_int
                } else {
                    6 as ::core::ffi::c_int
                } << resolutionFactor)) as COORD;
            s.origin.y = 0 as COORD;
            s.frame = SetRelFrameIndex((*pickState).SummaryFrame, 0 as SIZE);
            uqm_lua_checked!(DrawStamp(&raw mut s), ());
            uqm_lua_checked!(DrawBombPodText(&raw mut s), ());
            uqm_lua_checked!(DrawNoLimit(), ());
        } else {
            uqm_lua_checked!(DrawSavegameCargo(&raw mut (*pSD).SS), ());
            SetContext(RadarContext);
            GetContextClipRect(&raw mut OldRect);
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
                    } << resolutionFactor)
                        + ((1 as ::core::ffi::c_int) << resolutionFactor)
                } else {
                    0 as ::core::ffi::c_int
                } + ((10 as ::core::ffi::c_int) << resolutionFactor)
                + (if if optWindowType == 1 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    0 as ::core::ffi::c_int
                } else {
                    14 as ::core::ffi::c_int
                } << resolutionFactor)
                - (if if optWindowType == 1 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    0 as ::core::ffi::c_int
                } else {
                    7 as ::core::ffi::c_int
                } << resolutionFactor)) as COORD;
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
                    } << resolutionFactor)
                        + ((1 as ::core::ffi::c_int) << resolutionFactor)
                } else {
                    0 as ::core::ffi::c_int
                } + ((84 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
            r.extent = OldRect.extent;
            SetContextClipRect(&raw mut r);
            uqm_lua_checked!(InitLander(
                ((*pSD).Flags as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as BYTE,
            ), ());
            SetContextClipRect(&raw mut OldRect);
            SetContext(SpaceContext);
            snprintf(
                &raw mut buf as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[UNICODE; 256]>(),
                b"%u\0".as_ptr() as *const ::core::ffi::c_char,
                (*pSD).SS.ResUnits,
            );
            t.baseline.y = ((102 as ::core::ffi::c_int) << resolutionFactor) as COORD;
            SetContextForeGroundColor(
                colorFromRgb15(0x10 as BYTE, 0 as BYTE, 0x10 as BYTE),
            );
            uqm_lua_checked!(font_DrawText(&raw mut t), ());
            t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
        }
        t.baseline.y = ((126 as ::core::ffi::c_int) << resolutionFactor) as COORD;
        snprintf(
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[UNICODE; 256]>(),
            b"%u\0".as_ptr() as *const ::core::ffi::c_char,
            (((*pSD).MCreditHi as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
                as UWORD as ::core::ffi::c_int | (*pSD).MCreditLo as ::core::ffi::c_int,
        );
        SetContextForeGroundColor(colorFromRgb15(0x10 as BYTE, 0 as BYTE, 0x10 as BYTE));
        uqm_lua_checked!(font_DrawText(&raw mut t), ());
        t.baseline.x = (((1 as ::core::ffi::c_int) << resolutionFactor)
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
                - ((57 as ::core::ffi::c_int) << resolutionFactor)
                - ((12 as ::core::ffi::c_int) << resolutionFactor)
                - ((2 as ::core::ffi::c_int) << resolutionFactor)
                >> 1 as ::core::ffi::c_int)) as COORD;
        t.baseline.y = (((139 as ::core::ffi::c_int + 6 as ::core::ffi::c_int)
            << resolutionFactor)
            + (if if optWindowType == 1 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                2 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } << resolutionFactor)) as COORD;
        t.align = TEXT_ALIGN::ALIGN_CENTER;
        t.pStr = &raw mut buf as *mut UNICODE;
        starPt.x = logxToUniverse((*pSD).SS.log_x) as COORD;
        starPt.y = logyToUniverse((*pSD).SS.log_y) as COORD;
        's_368: {
            match (*pSD).Activity as ::core::ffi::c_int {
                1 | 4 | 7 | 8 => {
                    let mut QuasiState: BYTE = 0;
                    let mut SDPtr: *mut STAR_DESC = ::core::ptr::null_mut::<STAR_DESC>();
                    QuasiState = uqm_lua_checked!(getGameStateUint(
                        b"ARILOU_SPACE_SIDE\0".as_ptr() as *const ::core::ffi::c_char,
                    ), ()) as BYTE;
                    uqm_lua_checked!(setGameStateUint(
                        b"ARILOU_SPACE_SIDE\0".as_ptr() as *const ::core::ffi::c_char,
                        0 as DWORD,
                    ), ());
                    SDPtr = uqm_lua_checked!(FindStar(
                        ::core::ptr::null_mut::<STAR_DESC>(),
                        &raw mut starPt,
                        1 as SIZE,
                        1 as SIZE,
                    ), ());
                    uqm_lua_checked!(setGameStateUint(
                        b"ARILOU_SPACE_SIDE\0".as_ptr() as *const ::core::ffi::c_char,
                        QuasiState as DWORD,
                    ), ());
                    if !SDPtr.is_null() {
                        GetClusterName(SDPtr, &raw mut buf as *mut UNICODE);
                        starPt = (*SDPtr).star_pt;
                        break 's_368;
                    }
                }
                3 => {
                    utf8StringCopy(
                        &raw mut buf as *mut ::core::ffi::c_char,
                        ::core::mem::size_of::<[UNICODE; 256]>(),
                        GetStringAddress(
                            SetAbsStringTableIndex(
                                GameStrings,
                                (C2Rust_Unnamed_4::NAVIGATION_STRING_BASE.0
                                    as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as COUNT,
                            ),
                        ),
                    );
                    break 's_368;
                }
                6 => {
                    utf8StringCopy(
                        &raw mut buf as *mut ::core::ffi::c_char,
                        ::core::mem::size_of::<[UNICODE; 256]>(),
                        GetStringAddress(
                            SetAbsStringTableIndex(
                                GameStrings,
                                (C2Rust_Unnamed_4::NAVIGATION_STRING_BASE.0
                                    as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as COUNT,
                            ),
                        ),
                    );
                    break 's_368;
                }
                _ => {}
            }
            buf[0usize] = '\0' as UNICODE;
        }
        if isPC(optWhichFonts).0 != 0 {
            SetContextFont(TinyFont);
        } else {
            SetContextFont(TinyFontBold);
        }
        SetContextForeGroundColor(colorFromRgb15(0x1b as BYTE, 0 as BYTE, 0x1b as BYTE));
        t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
        if is3DO(optWhichFonts).0 != 0 {
            replaceChar(
                &raw mut buf as *mut ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x9 as ::core::ffi::c_char,
            );
        }
        uqm_lua_checked!(font_DrawText(&raw mut t), ());
        t.align = TEXT_ALIGN::ALIGN_CENTER;
        t.baseline.x = (CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
            - (if if optWindowType == 1 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                -1 as ::core::ffi::c_int
            } else {
                4 as ::core::ffi::c_int
            } << resolutionFactor)
            + ((((57 as ::core::ffi::c_int) << resolutionFactor)
                - ((2 as ::core::ffi::c_int) << resolutionFactor) >> resolutionFactor
                >> 1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        match (*pSD).Activity as ::core::ffi::c_int {
            8 => {
                utf8StringCopy(
                    &raw mut buf as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[UNICODE; 256]>(),
                    GetStringAddress(
                        SetAbsStringTableIndex(
                            GameStrings,
                            C2Rust_Unnamed_4::STARBASE_STRING_BASE.0
                                as ::core::ffi::c_int as COUNT,
                        ),
                    ),
                );
            }
            1 => {
                utf8StringCopy(
                    &raw mut buf as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[UNICODE; 256]>(),
                    GetStringAddress(
                        SetAbsStringTableIndex(
                            GameStrings,
                            (C2Rust_Unnamed_4::PLANET_NUMBER_BASE.0 as ::core::ffi::c_int
                                + 32 as ::core::ffi::c_int) as COUNT,
                        ),
                    ),
                );
            }
            7 => {
                utf8StringCopy(
                    &raw mut buf as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[UNICODE; 256]>(),
                    &raw mut (*pSD).SS.PlanetName as *mut UNICODE,
                );
            }
            _ => {
                snprintf(
                    &raw mut buf as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[UNICODE; 256]>(),
                    b"%03u.%01u : %03u.%01u\0".as_ptr() as *const ::core::ffi::c_char,
                    starPt.x as ::core::ffi::c_int / 10 as ::core::ffi::c_int,
                    starPt.x as ::core::ffi::c_int % 10 as ::core::ffi::c_int,
                    starPt.y as ::core::ffi::c_int / 10 as ::core::ffi::c_int,
                    starPt.y as ::core::ffi::c_int % 10 as ::core::ffi::c_int,
                );
            }
        }
        if is3DO(optWhichFonts).0 != 0 {
            replaceChar(
                &raw mut buf as *mut ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x9 as ::core::ffi::c_char,
            );
        }
        t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
        uqm_lua_checked!(font_DrawText(&raw mut t), ());
        SetContext(OldContext);
        GlobData.Game_state.built_ship_q = player_built_q;
        GlobData.SIS_state = SaveSS;
        if optCustomBorder.0 != 0 {
            uqm_lua_checked!(DrawStatusMessage(::core::ptr::null::<UNICODE>()), ());
        }
    }
    UnbatchGraphics();
    SaveOrLoad = BOOLEAN::FALSE;
}
unsafe extern "C" fn TruncateSaveName(
    mut buf: *mut UNICODE,
    mut maxWidth: COORD,
    mut naming: BOOLEAN,
) {
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
    t.pStr = buf;
    r = uqm_lua_checked!(font_GetTextRect(&raw mut t), ());
    if r.extent.width as ::core::ffi::c_int > maxWidth as ::core::ffi::c_int {
        let mut stringLength: size_t = strlen(t.pStr);
        let ellipses: [::core::ffi::c_char; 4] = ::core::mem::transmute::<
            [u8; 4],
            [::core::ffi::c_char; 4],
        >(*b"...\0");
        loop {
            stringLength = stringLength.wrapping_sub(1);
            strncpy(
                buf
                    .offset(
                        stringLength
                            .wrapping_sub(
                                ::core::mem::size_of::<[::core::ffi::c_char; 4]>(),
                            ) as isize,
                    ),
                &raw const ellipses as *const ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 4]>(),
            );
            r = uqm_lua_checked!(font_GetTextRect(&raw mut t), ());
            if r.extent.width as ::core::ffi::c_int <= maxWidth as ::core::ffi::c_int {
                break;
            }
        }
        if naming.0 != 0 {
            *buf
                .offset(
                    strlen(buf)
                        .wrapping_sub(
                            strlen(&raw const ellipses as *const ::core::ffi::c_char),
                        ) as isize,
                ) = '\0' as UNICODE;
        }
    }
}
unsafe extern "C" fn DrawGameSelection(
    mut pickState: *mut PICK_GAME_STATE,
    mut selSlot: COUNT,
) {
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
    let mut i: COUNT = 0;
    let mut curSlot: COUNT = 0;
    let mut totalSlots: COUNT = 0;
    let mut buf: [UNICODE; 256] = [0; 256];
    let mut buf2: [UNICODE; 80] = [0; 80];
    let mut SaveName: *mut UNICODE = ::core::ptr::null_mut::<UNICODE>();
    let mut UnSelected: Color = colorFromRgb15(0 as BYTE, 0 as BYTE, 0x14 as BYTE);
    let mut Selected: Color = colorFromRgb15(0x1b as BYTE, 0 as BYTE, 0x1b as BYTE);
    BatchGraphics();
    SetContextFont(TinyFont);
    uqm_lua_checked!(DrawSaveLoadText(pickState), ());
    r.corner.x = ((1 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    r.corner.y = ((160 as ::core::ffi::c_int) << resolutionFactor) as COORD;
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
            } << resolutionFactor)) - ((2 as ::core::ffi::c_int) << resolutionFactor)
        + (if if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        } << resolutionFactor)) as COORD;
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
        - r.corner.y as ::core::ffi::c_int
        - (if if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            0 as ::core::ffi::c_int
        } else {
            1 as ::core::ffi::c_int
        } << resolutionFactor)) as COORD;
    SetContextForeGroundColor(colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE));
    uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    t.pStr = &raw mut buf as *mut UNICODE;
    t.align = TEXT_ALIGN::ALIGN_LEFT;
    totalSlots = (if (*pickState).saving.0 != 0 {
        50 as ::core::ffi::c_int
    } else {
        50 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
    }) as COUNT;
    curSlot = (selSlot as ::core::ffi::c_int
        - selSlot as ::core::ffi::c_int
            % if optWindowType < 2 as DWORD {
                2 as ::core::ffi::c_int
            } else {
                5 as ::core::ffi::c_int
            }) as COUNT;
    i = 0 as COUNT;
    while (i as ::core::ffi::c_int)
        < if optWindowType < 2 as DWORD {
            2 as ::core::ffi::c_int
        } else {
            5 as ::core::ffi::c_int
        } && (curSlot as ::core::ffi::c_int) < totalSlots as ::core::ffi::c_int
    {
        let mut desc: *mut SUMMARY_DESC = (&raw mut (*pickState).summary
            as *mut SUMMARY_DESC)
            .offset(curSlot as isize);
        SetContextForeGroundColor(
            if curSlot as ::core::ffi::c_int == selSlot as ::core::ffi::c_int {
                Selected
            } else {
                UnSelected
            },
        );
        r.extent.width = ((15 as ::core::ffi::c_int) << resolutionFactor) as COORD;
        if 50 as ::core::ffi::c_int > 99 as ::core::ffi::c_int {
            r.extent.width = (r.extent.width as ::core::ffi::c_int
                + ((5 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        }
        r.extent.height = ((11 as ::core::ffi::c_int) << resolutionFactor) as COORD;
        r.corner.x = ((8 as ::core::ffi::c_int) << resolutionFactor) as COORD;
        r.corner.y = ((160 as ::core::ffi::c_int
            + i as ::core::ffi::c_int * 13 as ::core::ffi::c_int) << resolutionFactor)
            as COORD;
        uqm_lua_checked!(DrawRectangle(
            &raw mut r,
            BOOLEAN(
                (if resolutionFactor != 2 as ::core::ffi::c_uint {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                }) as ::core::ffi::c_uint,
            ),
        ), ());
        t.baseline.x = (r.corner.x as ::core::ffi::c_int
            + ((3 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        t.baseline.y = (r.corner.y as ::core::ffi::c_int
            + ((8 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        if (*pickState).saving.0 == 0
            && curSlot as ::core::ffi::c_int == 50 as ::core::ffi::c_int
        {
            snprintf(
                &raw mut buf as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[UNICODE; 256]>(),
                b"QS\0".as_ptr() as *const ::core::ffi::c_char,
            );
        } else {
            snprintf(
                &raw mut buf as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[UNICODE; 256]>(),
                if 50 as ::core::ffi::c_int > 99 as ::core::ffi::c_int {
                    b"%03u\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"%02u\0".as_ptr() as *const ::core::ffi::c_char
                },
                curSlot as ::core::ffi::c_int,
            );
        }
        uqm_lua_checked!(font_DrawText(&raw mut t), ());
        r.extent.width = (((204 as ::core::ffi::c_int) << resolutionFactor)
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
                - ((242 as ::core::ffi::c_int) << resolutionFactor))) as COORD;
        r.corner.x = ((30 as ::core::ffi::c_int) << resolutionFactor) as COORD;
        uqm_lua_checked!(DrawRectangle(
            &raw mut r,
            BOOLEAN(
                (if resolutionFactor != 2 as ::core::ffi::c_uint {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                }) as ::core::ffi::c_uint,
            ),
        ), ());
        t.baseline.x = (r.corner.x as ::core::ffi::c_int
            + ((3 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        if (*desc).year_index as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            utf8StringCopy(
                &raw mut buf as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[UNICODE; 256]>(),
                GetStringAddress(
                    SetAbsStringTableIndex(
                        GameStrings,
                        (C2Rust_Unnamed_4::SAVEGAME_STRING_BASE.0 as ::core::ffi::c_int
                            + 3 as ::core::ffi::c_int) as COUNT,
                    ),
                ),
            );
        } else {
            DateToString(
                &raw mut buf2 as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[UNICODE; 80]>(),
                (*desc).month_index,
                (*desc).day_index,
                (*desc).year_index,
            );
            if strncmp(
                &raw mut (*desc).SaveNameChecker as *mut UNICODE,
                b"superbutcherX !\0".as_ptr() as *const ::core::ffi::c_char,
                32 as size_t,
            ) == 0
            {
                SaveName = &raw mut (*desc).LegacySaveName as *mut UNICODE;
            } else {
                SaveName = if (*desc).SaveName[0usize] as ::core::ffi::c_int != 0 {
                    &raw mut (*desc).SaveName as *mut UNICODE
                } else {
                    GetStringAddress(
                        SetAbsStringTableIndex(
                            GameStrings,
                            (C2Rust_Unnamed_4::SAVEGAME_STRING_BASE.0
                                as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as COUNT,
                        ),
                    )
                };
            }
            snprintf(
                &raw mut buf as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[UNICODE; 256]>(),
                b"%s: %s\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut buf2 as *mut UNICODE,
                SaveName,
            );
            uqm_lua_checked!(TruncateSaveName(
                &raw mut buf as *mut UNICODE,
                (r.extent.width as ::core::ffi::c_int - 7 as ::core::ffi::c_int)
                    as COORD,
                BOOLEAN::FALSE,
            ), ());
        }
        uqm_lua_checked!(font_DrawText(&raw mut t), ());
        i = i.wrapping_add(1);
        curSlot = curSlot.wrapping_add(1);
    }
    UnbatchGraphics();
}
unsafe extern "C" fn RedrawPickDisplay(
    mut pickState: *mut PICK_GAME_STATE,
    mut selSlot: COUNT,
) {
    BatchGraphics();
    uqm_lua_checked!(DrawBlankSavegameDisplay(pickState), ());
    uqm_lua_checked!(DrawSavegameSummary(pickState, selSlot), ());
    uqm_lua_checked!(DrawGameSelection(pickState, selSlot), ());
    UnbatchGraphics();
}
unsafe extern "C" fn LoadGameDescriptions(
    mut pSD: *mut SUMMARY_DESC,
    mut includeQuickSave: BOOLEAN,
) {
    let mut i: COUNT = 0;
    let mut maxSlots: COUNT = (if includeQuickSave.0 != 0 {
        50 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
    } else {
        50 as ::core::ffi::c_int
    }) as COUNT;
    i = 0 as COUNT;
    while (i as ::core::ffi::c_int) < maxSlots as ::core::ffi::c_int {
        if uqm_lua_checked!(LoadGame(i, pSD, ::core::ptr::null_mut::<uio_Stream>(), BOOLEAN::FALSE), ()).0 == 0
        {
            (*pSD).year_index = 0 as COUNT;
        }
        i = i.wrapping_add(1);
        pSD = pSD.offset(1);
    }
}
unsafe extern "C" fn DoPickGame(mut pMS: *mut MENU_STATE) -> BOOLEAN {
    let mut pickState: *mut PICK_GAME_STATE = (*pMS).privData as *mut PICK_GAME_STATE;
    let mut NewState: BYTE = 0;
    let mut pSD: *mut SUMMARY_DESC = ::core::ptr::null_mut::<SUMMARY_DESC>();
    let mut TimeIn: DWORD = GetTimeCounter();
    let mut maxSlots: COUNT = (if (*pickState).saving.0 != 0 {
        50 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
    } else {
        50 as ::core::ffi::c_int
    }) as COUNT;
    if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
        & C2Rust_Unnamed_2::CHECK_ABORT.0 as ::core::ffi::c_int != 0
    {
        return BOOLEAN::FALSE;
    }
    if PulsedInputState
        .menu[C2Rust_Unnamed::KEY_MENU_CANCEL.0 as ::core::ffi::c_int as usize] != 0
    {
        (*pickState).success = BOOLEAN::FALSE;
        return BOOLEAN::FALSE;
    } else if PulsedInputState
        .menu[C2Rust_Unnamed::KEY_MENU_SELECT.0 as ::core::ffi::c_int as usize] != 0
    {
        pSD = (&raw mut (*pickState).summary as *mut SUMMARY_DESC)
            .offset((*pMS).CurState as isize);
        if (*pickState).saving.0 != 0 || (*pSD).year_index as ::core::ffi::c_int != 0
            || (*pickState).saving.0 == 0
                && (*pMS).CurState as ::core::ffi::c_int == 50 as ::core::ffi::c_int
        {
            let mut LoadFuelScaled: DWORD = loadFuel.wrapping_div(100 as DWORD);
            let mut TankCapacityScaled: DWORD = GetFuelTankCapacity()
                .wrapping_div(100 as DWORD);
            if optInfiniteRU.0 != 0 {
                GlobData.SIS_state.ResUnits = oldRU;
            }
            if optInfiniteFuel.0 != 0 {
                if loadFuel <= GetFuelTankCapacity() {
                    GlobData.SIS_state.FuelOnBoard = loadFuel;
                } else {
                    GlobData.SIS_state.ResUnits = GlobData
                        .SIS_state
                        .ResUnits
                        .wrapping_add(
                            LoadFuelScaled
                                .wrapping_sub(TankCapacityScaled)
                                .wrapping_mul(GlobData.Game_state.FuelCost as DWORD),
                        );
                    GlobData.SIS_state.FuelOnBoard = GetFuelTankCapacity();
                }
            }
            uqm_lua_checked!(PlayMenuSound(MENU_SOUND_EFFECT::MENU_SOUND_SUCCESS), BOOLEAN(0));
            (*pickState).success = BOOLEAN::TRUE;
            return BOOLEAN::FALSE;
        }
        uqm_lua_checked!(PlayMenuSound(MENU_SOUND_EFFECT::MENU_SOUND_FAILURE), BOOLEAN(0));
    } else {
        NewState = (*pMS).CurState;
        if PulsedInputState
            .menu[C2Rust_Unnamed::KEY_MENU_LEFT.0 as ::core::ffi::c_int as usize] != 0
            || PulsedInputState
                .menu[C2Rust_Unnamed::KEY_MENU_PAGE_UP.0 as ::core::ffi::c_int as usize]
                != 0
        {
            if NewState as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                NewState = maxSlots as BYTE;
            } else if NewState as ::core::ffi::c_int
                - if optWindowType < 2 as DWORD {
                    2 as ::core::ffi::c_int
                } else {
                    5 as ::core::ffi::c_int
                } >= 0 as ::core::ffi::c_int
            {
                NewState = (NewState as ::core::ffi::c_int
                    - if optWindowType < 2 as DWORD {
                        2 as ::core::ffi::c_int
                    } else {
                        5 as ::core::ffi::c_int
                    }) as BYTE;
            } else {
                NewState = 0 as BYTE;
            }
        } else if PulsedInputState
            .menu[C2Rust_Unnamed::KEY_MENU_RIGHT.0 as ::core::ffi::c_int as usize] != 0
            || PulsedInputState
                .menu[C2Rust_Unnamed::KEY_MENU_PAGE_DOWN.0 as ::core::ffi::c_int
                as usize] != 0
        {
            if NewState as ::core::ffi::c_int == maxSlots as ::core::ffi::c_int {
                NewState = 0 as BYTE;
            } else if NewState as ::core::ffi::c_int
                + if optWindowType < 2 as DWORD {
                    2 as ::core::ffi::c_int
                } else {
                    5 as ::core::ffi::c_int
                } < maxSlots as ::core::ffi::c_int
            {
                NewState = (NewState as ::core::ffi::c_int
                    + if optWindowType < 2 as DWORD {
                        2 as ::core::ffi::c_int
                    } else {
                        5 as ::core::ffi::c_int
                    }) as BYTE;
            } else {
                NewState = maxSlots as BYTE;
            }
        } else if PulsedInputState
            .menu[C2Rust_Unnamed::KEY_MENU_UP.0 as ::core::ffi::c_int as usize] != 0
        {
            if NewState as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                NewState = maxSlots as BYTE;
            } else {
                NewState = NewState.wrapping_sub(1);
            }
        } else if PulsedInputState
            .menu[C2Rust_Unnamed::KEY_MENU_DOWN.0 as ::core::ffi::c_int as usize] != 0
        {
            if NewState as ::core::ffi::c_int == maxSlots as ::core::ffi::c_int {
                NewState = 0 as BYTE;
            } else {
                NewState = NewState.wrapping_add(1);
            }
        }
        if NewState as ::core::ffi::c_int != (*pMS).CurState as ::core::ffi::c_int {
            (*pMS).CurState = NewState;
            SetContext(SpaceContext);
            uqm_lua_checked!(RedrawPickDisplay(pickState, (*pMS).CurState as COUNT), BOOLEAN(0));
        }
        uqm_lua_checked!(SleepThreadUntil(
            TimeIn
                .wrapping_add(
                    (840 as ::core::ffi::c_int / 30 as ::core::ffi::c_int) as TimeCount,
                ),
        ), BOOLEAN(0));
    }
    return BOOLEAN::TRUE;
}
unsafe extern "C" fn SaveLoadGame(
    mut pickState: *mut PICK_GAME_STATE,
    mut gameIndex: COUNT,
    mut canceled_by_user: *mut BOOLEAN,
) -> BOOLEAN {
    let mut desc: *mut SUMMARY_DESC = (&raw mut (*pickState).summary
        as *mut SUMMARY_DESC)
        .offset(gameIndex as ::core::ffi::c_int as isize);
    let mut nameBuf: [UNICODE; 256] = [0; 256];
    let mut saveStamp: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    let mut success: BOOLEAN = BOOLEAN::FALSE;
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    GetContextClipRect(&raw mut r);
    saveStamp.frame = ::core::ptr::null_mut::<FRAME_DESC>();
    if (*pickState).saving.0 != 0 {
        uqm_lua_checked!(TruncateSaveName(
            &raw mut (*desc).SaveName as *mut UNICODE,
            (r.extent.width as ::core::ffi::c_int - 104 as ::core::ffi::c_int) as COORD,
            BOOLEAN::TRUE,
        ), BOOLEAN(0));
        strncpy(
            &raw mut nameBuf as *mut ::core::ffi::c_char,
            &raw mut (*desc).SaveName as *mut UNICODE,
            64 as size_t,
        );
        nameBuf[64usize] = 0 as UNICODE;
        if uqm_lua_checked!(NameSaveGame(gameIndex, &raw mut nameBuf as *mut UNICODE), BOOLEAN(0)).0 != 0 {
            uqm_lua_checked!(PlayMenuSound(MENU_SOUND_EFFECT::MENU_SOUND_SUCCESS), BOOLEAN(0));
            uqm_lua_checked!(ConfirmSaveLoad(
                if (*pickState).saving.0 != 0 {
                    &raw mut saveStamp
                } else {
                    ::core::ptr::null_mut::<STAMP>()
                },
            ), BOOLEAN(0));
            success = uqm_lua_checked!(SaveGame(gameIndex, desc, &raw mut nameBuf as *mut UNICODE), BOOLEAN(0));
        } else {
            success = BOOLEAN::FALSE;
            *canceled_by_user = BOOLEAN::TRUE;
        }
    } else {
        uqm_lua_checked!(ConfirmSaveLoad(
            if (*pickState).saving.0 != 0 {
                &raw mut saveStamp
            } else {
                ::core::ptr::null_mut::<STAMP>()
            },
        ), BOOLEAN(0));
        success = uqm_lua_checked!(LoadGame(
            gameIndex,
            ::core::ptr::null_mut::<SUMMARY_DESC>(),
            ::core::ptr::null_mut::<uio_Stream>(),
            BOOLEAN::FALSE,
        ), BOOLEAN(0));
    }
    if (*pickState).saving.0 != 0 {
        uqm_lua_checked!(DrawStamp(&raw mut saveStamp), BOOLEAN(0));
    }
    DestroyDrawable(ReleaseDrawable(saveStamp.frame));
    return success;
}
#[no_mangle]
pub static mut SaveLoadActive: BOOLEAN = BOOLEAN::FALSE;
unsafe extern "C" fn PickGame(
    mut saving: BOOLEAN,
    mut fromMainMenu: BOOLEAN,
) -> BOOLEAN {
    let mut OldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    let mut MenuState: MENU_STATE = MENU_STATE {
        InputFunc: None,
        Initialized: 0,
        CurState: 0,
        CurFrame: ::core::ptr::null_mut::<FRAME_DESC>(),
        CurString: ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>(),
        first_item: POINT { x: 0, y: 0 },
        delta_item: 0,
        ModuleFrame: ::core::ptr::null_mut::<FRAME_DESC>(),
        flash_rect0: RECT {
            corner: POINT { x: 0, y: 0 },
            extent: EXTENT { width: 0, height: 0 },
        },
        flash_rect1: RECT {
            corner: POINT { x: 0, y: 0 },
            extent: EXTENT { width: 0, height: 0 },
        },
        flash_rect2: RECT {
            corner: POINT { x: 0, y: 0 },
            extent: EXTENT { width: 0, height: 0 },
        },
        flash_frame0: ::core::ptr::null_mut::<FRAME_DESC>(),
        flash_frame1: ::core::ptr::null_mut::<FRAME_DESC>(),
        flashContext: ::core::ptr::null_mut::<FlashContext>(),
        hMusic: ::core::ptr::null_mut::<*mut TFB_SoundSample>(),
        privData: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    let mut pickState: PICK_GAME_STATE = PICK_GAME_STATE {
        summary: [SUMMARY_DESC {
            SS: SIS_STATE {
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
                ShipName: [0; 32],
                CommanderName: [0; 32],
                PlanetName: [0; 32],
                Difficulty: 0,
                Extended: 0,
                Nomad: 0,
                ShipSeed: 0,
                Seed: 0,
                SaveVersion: 0,
            },
            Activity: 0,
            Flags: 0,
            day_index: 0,
            month_index: 0,
            year_index: 0,
            MCreditLo: 0,
            MCreditHi: 0,
            NumShips: 0,
            NumDevices: 0,
            ShipList: [0; 12],
            DeviceList: [0; 16],
            SaveName: [0; 64],
            SaveNameChecker: [0; 32],
            LegacySaveName: [0; 32],
            res_factor: 0,
        }; 51],
        saving: BOOLEAN::FALSE,
        success: BOOLEAN::FALSE,
        SummaryFrame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    let mut DlgRect: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut DlgStamp: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    let mut TimeOut: TimeCount = 0;
    let mut oldCallback: Option<InputFrameCallback> = None;
    SaveLoadActive = BOOLEAN::TRUE;
    memset(
        &raw mut pickState as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<PICK_GAME_STATE>(),
    );
    pickState.saving = saving;
    pickState.SummaryFrame = SetAbsFrameIndex(PlayFrame, 44 as COUNT);
    memset(
        &raw mut MenuState as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<MENU_STATE>(),
    );
    MenuState.privData = &raw mut pickState as *mut ::core::ffi::c_void;
    MenuState.CurState = lastUsedSlot as BYTE;
    TimeOut = uqm_lua_checked!(FadeMusic(
        0 as BYTE,
        (840 as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as SIZE,
    ), BOOLEAN(0)) as TimeCount;
    oldCallback = SetInputCallback(None);
    uqm_lua_checked!(LoadGameDescriptions(
        &raw mut pickState.summary as *mut SUMMARY_DESC,
        BOOLEAN((pickState.saving.0 == 0) as ::core::ffi::c_int as ::core::ffi::c_uint),
    ), BOOLEAN(0));
    OldContext = SetContext(SpaceContext);
    DlgStamp = SaveContextFrame(::core::ptr::null::<RECT>());
    GetContextClipRect(&raw mut DlgRect);
    uqm_lua_checked!(DrawMenuStateStrings(
        C2Rust_Unnamed_3::PM_SAVE_GAME.0 as ::core::ffi::c_int as BYTE,
        (saving.0 == 0) as ::core::ffi::c_int as SWORD,
    ), BOOLEAN(0));
    uqm_lua_checked!(SleepThreadUntil(TimeOut), BOOLEAN(0));
    uqm_lua_checked!(PauseMusic(), BOOLEAN(0));
    uqm_lua_checked!(StopSound(), BOOLEAN(0));
    uqm_lua_checked!(FadeMusic(160 as BYTE, 0 as SIZE), BOOLEAN(0));
    SetTransitionSource(::core::ptr::null::<RECT>());
    BatchGraphics();
    SetContextBackGroundColor(colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE));
    uqm_lua_checked!(ClearDrawable(), BOOLEAN(0));
    uqm_lua_checked!(RedrawPickDisplay(&raw mut pickState, MenuState.CurState as COUNT), BOOLEAN(0));
    if fromMainMenu.0 != 0 {
        UnbatchGraphics();
        FadeScreen(
            ScreenFadeType::FadeAllToColor,
            (840 as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as SIZE,
        );
    } else {
        let mut ctxRect: RECT = RECT {
            corner: POINT { x: 0, y: 0 },
            extent: EXTENT { width: 0, height: 0 },
        };
        GetContextClipRect(&raw mut ctxRect);
        uqm_lua_checked!(ScreenTransition(3 as ::core::ffi::c_int, &raw mut ctxRect), BOOLEAN(0));
        UnbatchGraphics();
    }
    SetMenuSounds(
        (((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as MENU_SOUND_FLAGS
            as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as MENU_SOUND_FLAGS
                as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as MENU_SOUND_FLAGS
                as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as MENU_SOUND_FLAGS
                as ::core::ffi::c_int
            | (((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as MENU_SOUND_FLAGS
                as ::core::ffi::c_int
                | ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
                    as MENU_SOUND_FLAGS as ::core::ffi::c_int)) as MENU_SOUND_FLAGS,
        0 as MENU_SOUND_FLAGS,
    );
    MenuState.InputFunc = Some(
        DoPickGame as unsafe extern "C" fn(*mut MENU_STATE) -> BOOLEAN,
    ) as Option<unsafe extern "C" fn(*mut menu_state) -> BOOLEAN>;
    loop {
        let mut canceled_by_user: BOOLEAN = BOOLEAN::FALSE;
        pickState.success = BOOLEAN::FALSE;
        uqm_lua_checked!(DoInput(&raw mut MenuState as *mut ::core::ffi::c_void, BOOLEAN::TRUE), BOOLEAN(0));
        if pickState.success.0 == 0 {
            break;
        }
        lastUsedSlot = MenuState.CurState as COUNT;
        if uqm_lua_checked!(SaveLoadGame(
                &raw mut pickState,
                MenuState.CurState as COUNT,
                &raw mut canceled_by_user,
            ), BOOLEAN(0))
            .0 != 0
        {
            break;
        }
        if saving.0 != 0 && canceled_by_user.0 == 0 {
            uqm_lua_checked!(SaveProblem(), BOOLEAN(0));
        }
        uqm_lua_checked!(LoadGameDescriptions(
            &raw mut pickState.summary as *mut SUMMARY_DESC,
            BOOLEAN(
                (pickState.saving.0 == 0) as ::core::ffi::c_int as ::core::ffi::c_uint,
            ),
        ), BOOLEAN(0));
        uqm_lua_checked!(RedrawPickDisplay(&raw mut pickState, MenuState.CurState as COUNT), BOOLEAN(0));
    }
    SetMenuSounds(
        (((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as MENU_SOUND_FLAGS
            as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as MENU_SOUND_FLAGS
                as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as MENU_SOUND_FLAGS
                as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as MENU_SOUND_FLAGS
                as ::core::ffi::c_int) as MENU_SOUND_FLAGS,
        ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as MENU_SOUND_FLAGS,
    );
    if pickState.success.0 != 0 && saving.0 == 0 {
        GlobData.Game_state.CurrentActivity = (GlobData.Game_state.CurrentActivity
            as ::core::ffi::c_int | C2Rust_Unnamed_2::CHECK_LOAD.0 as ::core::ffi::c_int)
            as ACTIVITY;
    }
    if pickState.success.0 != 0 {
        let mut pSD: *mut SUMMARY_DESC = (&raw mut pickState.summary
            as *mut SUMMARY_DESC)
            .offset(MenuState.CurState as ::core::ffi::c_int as isize);
        printf(
            if saving.0 != 0 {
                b"Saving -> \0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"Loading -> \0".as_ptr() as *const ::core::ffi::c_char
            },
        );
        printf(
            b"Slot: %d\n\0".as_ptr() as *const ::core::ffi::c_char,
            MenuState.CurState as ::core::ffi::c_int,
        );
        printf(
            b"Name: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*pSD).SaveName as *mut UNICODE,
        );
        printf(b"Seed Type: %s\n\0".as_ptr() as *const ::core::ffi::c_char, SeedStr());
        printf(
            b"Seed: %d\n\0".as_ptr() as *const ::core::ffi::c_char,
            GlobData.SIS_state.Seed,
        );
        printf(
            b"Difficulty: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            if if GlobData.SIS_state.Difficulty as ::core::ffi::c_int != 0 {
                GlobData.SIS_state.Difficulty as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } == 0 as ::core::ffi::c_int
            {
                b"Normal\0".as_ptr() as *const ::core::ffi::c_char
            } else if if GlobData.SIS_state.Difficulty as ::core::ffi::c_int != 0 {
                GlobData.SIS_state.Difficulty as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } == 1 as ::core::ffi::c_int
            {
                b"Easy\0".as_ptr() as *const ::core::ffi::c_char
            } else if if GlobData.SIS_state.Difficulty as ::core::ffi::c_int != 0 {
                GlobData.SIS_state.Difficulty as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } == 2 as ::core::ffi::c_int
            {
                b"Hard\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"CYO\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
        printf(
            b"Extended: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            if if GlobData.SIS_state.Extended as ::core::ffi::c_int != 0 {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                b"True\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"False\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
        printf(
            b"Nomad: %s\n\n\0".as_ptr() as *const ::core::ffi::c_char,
            if if GlobData.SIS_state.Nomad as ::core::ffi::c_int != 0 {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } == 2 as ::core::ffi::c_int
            {
                b"Normal\0".as_ptr() as *const ::core::ffi::c_char
            } else if if GlobData.SIS_state.Nomad as ::core::ffi::c_int != 0 {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } == 1 as ::core::ffi::c_int
            {
                b"Easy\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"Off\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
        log_add(
            log_Level::log_Info,
            if saving.0 != 0 {
                b"Saving > \0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"Loading > \0".as_ptr() as *const ::core::ffi::c_char
            },
        );
        log_add(
            log_Level::log_Info,
            b"Name: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*pSD).SaveName as *mut UNICODE,
        );
        log_add(
            log_Level::log_Info,
            b"Slot: %d\n\0".as_ptr() as *const ::core::ffi::c_char,
            MenuState.CurState as ::core::ffi::c_int,
        );
        log_add(
            log_Level::log_Info,
            b"Seed Type: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            SeedStr(),
        );
        log_add(
            log_Level::log_Info,
            b"Seed: %d\n\0".as_ptr() as *const ::core::ffi::c_char,
            GlobData.SIS_state.Seed,
        );
        log_add(
            log_Level::log_Info,
            b"Difficulty: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            if if GlobData.SIS_state.Difficulty as ::core::ffi::c_int != 0 {
                GlobData.SIS_state.Difficulty as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } == 0 as ::core::ffi::c_int
            {
                b"Normal\0".as_ptr() as *const ::core::ffi::c_char
            } else if if GlobData.SIS_state.Difficulty as ::core::ffi::c_int != 0 {
                GlobData.SIS_state.Difficulty as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } == 1 as ::core::ffi::c_int
            {
                b"Easy\0".as_ptr() as *const ::core::ffi::c_char
            } else if if GlobData.SIS_state.Difficulty as ::core::ffi::c_int != 0 {
                GlobData.SIS_state.Difficulty as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } == 2 as ::core::ffi::c_int
            {
                b"Hard\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"CYO\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
        log_add(
            log_Level::log_Info,
            b"Extended: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            if if GlobData.SIS_state.Extended as ::core::ffi::c_int != 0 {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                b"True\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"False\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
        log_add(
            log_Level::log_Info,
            b"Nomad: %s\n\n\0".as_ptr() as *const ::core::ffi::c_char,
            if if GlobData.SIS_state.Nomad as ::core::ffi::c_int != 0 {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } == 2 as ::core::ffi::c_int
            {
                b"Normal\0".as_ptr() as *const ::core::ffi::c_char
            } else if if GlobData.SIS_state.Nomad as ::core::ffi::c_int != 0 {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } == 1 as ::core::ffi::c_int
            {
                b"Easy\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"Off\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
        if lastUsedSlot as ::core::ffi::c_int == 50 as ::core::ffi::c_int {
            lastUsedSlot = 0 as COUNT;
        }
    }
    if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
        & C2Rust_Unnamed_2::CHECK_ABORT.0 as ::core::ffi::c_int == 0
        && (saving.0 != 0 || pickState.success.0 == 0 && fromMainMenu.0 == 0)
    {
        let mut InStarbase: BOOLEAN = BOOLEAN(
            (uqm_lua_checked!(getGameStateUint(
                b"GLOBAL_FLAGS_AND_DATA\0".as_ptr() as *const ::core::ffi::c_char,
            ), BOOLEAN(0)) == !(0 as ::core::ffi::c_int) as BYTE as DWORD) as ::core::ffi::c_int
                as ::core::ffi::c_uint,
        );
        DlgRect.extent.width = (DlgRect.extent.width as ::core::ffi::c_int
            + DlgRect.corner.x as ::core::ffi::c_int) as COORD;
        DlgRect.extent.height = (DlgRect.extent.height as ::core::ffi::c_int
            + DlgRect.corner.y as ::core::ffi::c_int) as COORD;
        DlgRect.corner.y = 0 as COORD;
        DlgRect.corner.x = DlgRect.corner.y;
        SetTransitionSource(&raw mut DlgRect);
        BatchGraphics();
        uqm_lua_checked!(DrawStamp(&raw mut DlgStamp), BOOLEAN(0));
        if InStarbase.0 != 0
            && OutfitOrShipyard as ::core::ffi::c_int > 1 as ::core::ffi::c_int
        {
            uqm_lua_checked!(DrawSISMessage(
                GetStringAddress(
                    SetAbsStringTableIndex(
                        GameStrings,
                        (C2Rust_Unnamed_4::STARBASE_STRING_BASE.0 as ::core::ffi::c_int
                            + OutfitOrShipyard as ::core::ffi::c_int) as COUNT,
                    ),
                ),
            ), BOOLEAN(0));
        } else {
            uqm_lua_checked!(DrawSISMessage(::core::ptr::null::<UNICODE>()), BOOLEAN(0));
        }
        OutfitOrShipyard = 0 as BYTE;
        if inHQSpace().0 != 0 {
            uqm_lua_checked!(DrawHyperCoords(GlobData.Game_state.ShipStamp.origin), BOOLEAN(0));
        } else if InStarbase.0 != 0 {
            uqm_lua_checked!(DrawSISTitle(
                GetStringAddress(
                    SetAbsStringTableIndex(
                        GameStrings,
                        (C2Rust_Unnamed_4::STARBASE_STRING_BASE.0 as ::core::ffi::c_int
                            + 0 as ::core::ffi::c_int) as COUNT,
                    ),
                ),
            ), BOOLEAN(0));
        } else if GlobData.Game_state.ip_planet as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
            uqm_lua_checked!(DrawHyperCoords((*CurStarDescPtr).star_pt), BOOLEAN(0));
        } else {
            uqm_lua_checked!(DrawSISTitle(&raw mut GlobData.SIS_state.PlanetName as *mut UNICODE), BOOLEAN(0));
        }
        uqm_lua_checked!(DeltaSISGauges(0x7fff as SIZE, 0x7fff as SDWORD, 0x7fff as ::core::ffi::c_int), BOOLEAN(0));
        uqm_lua_checked!(ScreenTransition(3 as ::core::ffi::c_int, &raw mut DlgRect), BOOLEAN(0));
        UnbatchGraphics();
    }
    DestroyDrawable(ReleaseDrawable(DlgStamp.frame));
    SetContext(OldContext);
    uqm_lua_checked!(ResumeMusic(), BOOLEAN(0));
    SetInputCallback(oldCallback);
    SaveLoadActive = BOOLEAN::FALSE;
    return pickState.success;
}
#[no_mangle]
pub unsafe extern "C" fn RequestQuickLoad() {
    let mut desc: SUMMARY_DESC = SUMMARY_DESC {
        SS: SIS_STATE {
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
            ShipName: [0; 32],
            CommanderName: [0; 32],
            PlanetName: [0; 32],
            Difficulty: 0,
            Extended: 0,
            Nomad: 0,
            ShipSeed: 0,
            Seed: 0,
            SaveVersion: 0,
        },
        Activity: 0,
        Flags: 0,
        day_index: 0,
        month_index: 0,
        year_index: 0,
        MCreditLo: 0,
        MCreditHi: 0,
        NumShips: 0,
        NumDevices: 0,
        ShipList: [0; 12],
        DeviceList: [0; 16],
        SaveName: [0; 64],
        SaveNameChecker: [0; 32],
        LegacySaveName: [0; 32],
        res_factor: 0,
    };
    if uqm_lua_checked!(inSavablePos(), ()).0 == 0 {
        return;
    }
    if uqm_lua_checked!(LoadGame(
            50 as COUNT,
            &raw mut desc,
            ::core::ptr::null_mut::<uio_Stream>(),
            BOOLEAN::FALSE,
        ), ())
        .0 == 0
    {
        log_add(
            log_Level::log_Warning,
            b"QuickLoad: No quicksave found\0".as_ptr() as *const ::core::ffi::c_char,
        );
        uqm_lua_checked!(DoPopupWindow(
            GetStringAddress(
                SetAbsStringTableIndex(
                    GameStrings,
                    (C2Rust_Unnamed_4::SAVEGAME_STRING_BASE.0 as ::core::ffi::c_int
                        + 9 as ::core::ffi::c_int) as COUNT,
                ),
            ),
        ), ());
        return;
    }
    QuickLoadRequested = BOOLEAN::TRUE;
    if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
        & C2Rust_Unnamed_2::START_INTERPLANETARY.0 as ::core::ffi::c_int
        == 0 as ::core::ffi::c_int
    {
        GlobData.Game_state.CurrentActivity = (GlobData.Game_state.CurrentActivity
            as ::core::ffi::c_int
            | (C2Rust_Unnamed_2::START_INTERPLANETARY.0 as ::core::ffi::c_int
                | C2Rust_Unnamed_2::CHECK_LOAD.0 as ::core::ffi::c_int)) as ACTIVITY;
    } else {
        GlobData.Game_state.CurrentActivity = (GlobData.Game_state.CurrentActivity
            as ::core::ffi::c_int | C2Rust_Unnamed_2::CHECK_LOAD.0 as ::core::ffi::c_int)
            as ACTIVITY;
    };
}
#[no_mangle]
pub unsafe extern "C" fn QuickLoadDeferred() -> BOOLEAN {
    let mut OldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    if QuickLoadRequested.0 == 0 {
        return BOOLEAN::FALSE;
    }
    QuickLoadRequested = BOOLEAN::FALSE;
    OldContext = SetContext(SpaceContext);
    uqm_lua_checked!(ConfirmSaveLoad(::core::ptr::null_mut::<STAMP>()), BOOLEAN(0));
    SetContext(OldContext);
    if uqm_lua_checked!(LoadGame(
            50 as COUNT,
            ::core::ptr::null_mut::<SUMMARY_DESC>(),
            ::core::ptr::null_mut::<uio_Stream>(),
            BOOLEAN::FALSE,
        ), BOOLEAN(0))
        .0 == 0
    {
        log_add(
            log_Level::log_Error,
            b"QuickLoad: Failed to load quicksave\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return BOOLEAN::FALSE;
    }
    return BOOLEAN::TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn QuickSave() -> BOOLEAN {
    let mut pickState: PICK_GAME_STATE = PICK_GAME_STATE {
        summary: [SUMMARY_DESC {
            SS: SIS_STATE {
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
                ShipName: [0; 32],
                CommanderName: [0; 32],
                PlanetName: [0; 32],
                Difficulty: 0,
                Extended: 0,
                Nomad: 0,
                ShipSeed: 0,
                Seed: 0,
                SaveVersion: 0,
            },
            Activity: 0,
            Flags: 0,
            day_index: 0,
            month_index: 0,
            year_index: 0,
            MCreditLo: 0,
            MCreditHi: 0,
            NumShips: 0,
            NumDevices: 0,
            ShipList: [0; 12],
            DeviceList: [0; 16],
            SaveName: [0; 64],
            SaveNameChecker: [0; 32],
            LegacySaveName: [0; 32],
            res_factor: 0,
        }; 51],
        saving: BOOLEAN::FALSE,
        success: BOOLEAN::FALSE,
        SummaryFrame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    let mut saveStamp: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    let mut success: BOOLEAN = BOOLEAN::FALSE;
    let mut OldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    memset(
        &raw mut pickState as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<PICK_GAME_STATE>(),
    );
    pickState.saving = BOOLEAN::TRUE;
    if uqm_lua_checked!(LoadGame(
            50 as COUNT,
            &raw mut pickState.summary as *mut SUMMARY_DESC,
            ::core::ptr::null_mut::<uio_Stream>(),
            BOOLEAN::FALSE,
        ), BOOLEAN(0))
        .0 == 0
    {
        (*(&raw mut pickState.summary as *mut SUMMARY_DESC)).year_index = 0 as COUNT;
    }
    OldContext = SetContext(SpaceContext);
    uqm_lua_checked!(ConfirmSaveLoad(&raw mut saveStamp), BOOLEAN(0));
    uqm_lua_checked!(SleepThread((840 as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as TimePeriod), BOOLEAN(0));
    success = BOOLEAN(
        (uqm_lua_checked!(SaveGame(
                50 as COUNT,
                &raw mut pickState.summary as *mut SUMMARY_DESC,
                GetStringAddress(
                    SetAbsStringTableIndex(
                        GameStrings,
                        (C2Rust_Unnamed_4::SAVEGAME_STRING_BASE.0 as ::core::ffi::c_int
                            + 8 as ::core::ffi::c_int) as COUNT,
                    ),
                ),
            ), BOOLEAN(0))
            .0 == 0) as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
    if success.0 == 0 {
        log_add(
            log_Level::log_Error,
            b"QuickSave failed to save\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    uqm_lua_checked!(DrawStamp(&raw mut saveStamp), BOOLEAN(0));
    DestroyDrawable(ReleaseDrawable(saveStamp.frame));
    SetContext(OldContext);
    return success;
}
unsafe extern "C" fn DoGameOptions(mut pMS: *mut MENU_STATE) -> BOOLEAN {
    if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
        & (C2Rust_Unnamed_2::CHECK_ABORT.0 as ::core::ffi::c_int
            | C2Rust_Unnamed_2::CHECK_LOAD.0 as ::core::ffi::c_int) != 0
    {
        return BOOLEAN::FALSE;
    }
    if PulsedInputState
        .menu[C2Rust_Unnamed::KEY_MENU_CANCEL.0 as ::core::ffi::c_int as usize] != 0
        || PulsedInputState
            .menu[C2Rust_Unnamed::KEY_MENU_SELECT.0 as ::core::ffi::c_int as usize] != 0
            && (*pMS).CurState as ::core::ffi::c_int
                == C2Rust_Unnamed_5::EXIT_GAME_MENU.0 as ::core::ffi::c_int
    {
        return BOOLEAN::FALSE
    } else if PulsedInputState
        .menu[C2Rust_Unnamed::KEY_MENU_SELECT.0 as ::core::ffi::c_int as usize] != 0
    {
        match (*pMS).CurState as ::core::ffi::c_int {
            0 | 1 => {
                uqm_lua_checked!(SetFlashRect(::core::ptr::null::<RECT>(), BOOLEAN::FALSE), BOOLEAN(0));
                if uqm_lua_checked!(PickGame(
                        BOOLEAN(
                            ((*pMS).CurState as ::core::ffi::c_int
                                == C2Rust_Unnamed_5::SAVE_GAME.0 as ::core::ffi::c_int)
                                as ::core::ffi::c_int as ::core::ffi::c_uint,
                        ),
                        BOOLEAN::FALSE,
                    ), BOOLEAN(0))
                    .0 != 0
                {
                    return BOOLEAN::FALSE;
                }
                uqm_lua_checked!(DrawMenuStateStrings(
                    C2Rust_Unnamed_3::PM_SAVE_GAME.0 as ::core::ffi::c_int as BYTE,
                    (*pMS).CurState as SWORD,
                ), BOOLEAN(0));
                uqm_lua_checked!(SetFlashRect(
                    ::core::ptr::with_exposed_provenance_mut::<
                        RECT,
                    >(!(0 as ::core::ffi::c_long) as usize),
                    BOOLEAN::FALSE,
                ), BOOLEAN(0));
            }
            2 => {
                uqm_lua_checked!(DrawMenuStateStrings(
                    C2Rust_Unnamed_3::PM_SAVE_GAME.0 as ::core::ffi::c_int as BYTE,
                    (*pMS).CurState as SWORD,
                ), BOOLEAN(0));
                if uqm_lua_checked!(ConfirmExit(), BOOLEAN(0)).0 != 0 {
                    return BOOLEAN::FALSE
                } else {
                    uqm_lua_checked!(DrawMenuStateStrings(
                        C2Rust_Unnamed_3::PM_SAVE_GAME.0 as ::core::ffi::c_int as BYTE,
                        (*pMS).CurState as SWORD,
                    ), BOOLEAN(0));
                }
            }
            3 => {
                uqm_lua_checked!(SettingsMenu(BOOLEAN::FALSE), BOOLEAN(0));
                uqm_lua_checked!(DrawMenuStateStrings(
                    C2Rust_Unnamed_3::PM_SAVE_GAME.0 as ::core::ffi::c_int as BYTE,
                    (*pMS).CurState as SWORD,
                ), BOOLEAN(0));
            }
            _ => {}
        }
    } else {
        uqm_lua_checked!(DoMenuChooser(
            pMS,
            C2Rust_Unnamed_3::PM_SAVE_GAME.0 as ::core::ffi::c_int as BYTE,
        ), BOOLEAN(0));
    }
    return BOOLEAN::TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn GameOptions() -> BOOLEAN {
    let mut MenuState: MENU_STATE = MENU_STATE {
        InputFunc: None,
        Initialized: 0,
        CurState: 0,
        CurFrame: ::core::ptr::null_mut::<FRAME_DESC>(),
        CurString: ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>(),
        first_item: POINT { x: 0, y: 0 },
        delta_item: 0,
        ModuleFrame: ::core::ptr::null_mut::<FRAME_DESC>(),
        flash_rect0: RECT {
            corner: POINT { x: 0, y: 0 },
            extent: EXTENT { width: 0, height: 0 },
        },
        flash_rect1: RECT {
            corner: POINT { x: 0, y: 0 },
            extent: EXTENT { width: 0, height: 0 },
        },
        flash_rect2: RECT {
            corner: POINT { x: 0, y: 0 },
            extent: EXTENT { width: 0, height: 0 },
        },
        flash_frame0: ::core::ptr::null_mut::<FRAME_DESC>(),
        flash_frame1: ::core::ptr::null_mut::<FRAME_DESC>(),
        flashContext: ::core::ptr::null_mut::<FlashContext>(),
        hMusic: ::core::ptr::null_mut::<*mut TFB_SoundSample>(),
        privData: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    memset(
        &raw mut MenuState as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<MENU_STATE>(),
    );
    if LastActivity as ::core::ffi::c_int
        == C2Rust_Unnamed_2::CHECK_LOAD.0 as ::core::ffi::c_int
    {
        let mut success: BOOLEAN = BOOLEAN::FALSE;
        uqm_lua_checked!(DrawMenuStateStrings(
            C2Rust_Unnamed_3::PM_SAVE_GAME.0 as ::core::ffi::c_int as BYTE,
            C2Rust_Unnamed_5::LOAD_GAME.0 as ::core::ffi::c_int as SWORD,
        ), BOOLEAN(0));
        success = uqm_lua_checked!(PickGame(BOOLEAN::FALSE, BOOLEAN::TRUE), BOOLEAN(0));
        if success.0 == 0 {
            GlobData.Game_state.CurrentActivity = (GlobData.Game_state.CurrentActivity
                as ::core::ffi::c_int
                | C2Rust_Unnamed_2::CHECK_ABORT.0 as ::core::ffi::c_int) as ACTIVITY;
        }
        return BOOLEAN::FALSE;
    }
    MenuState.CurState = C2Rust_Unnamed_5::SAVE_GAME.0 as ::core::ffi::c_int as BYTE;
    uqm_lua_checked!(DrawMenuStateStrings(
        C2Rust_Unnamed_3::PM_SAVE_GAME.0 as ::core::ffi::c_int as BYTE,
        MenuState.CurState as SWORD,
    ), BOOLEAN(0));
    uqm_lua_checked!(SetFlashRect(
        ::core::ptr::with_exposed_provenance_mut::<RECT>(!(0 as ::core::ffi::c_long) as usize),
        BOOLEAN::FALSE,
    ), BOOLEAN(0));
    SetMenuSounds(
        (((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as MENU_SOUND_FLAGS
            as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as MENU_SOUND_FLAGS
                as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as MENU_SOUND_FLAGS
                as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as MENU_SOUND_FLAGS
                as ::core::ffi::c_int) as MENU_SOUND_FLAGS,
        ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as MENU_SOUND_FLAGS,
    );
    MenuState.InputFunc = Some(
        DoGameOptions as unsafe extern "C" fn(*mut MENU_STATE) -> BOOLEAN,
    ) as Option<unsafe extern "C" fn(*mut menu_state) -> BOOLEAN>;
    uqm_lua_checked!(DoInput(&raw mut MenuState as *mut ::core::ffi::c_void, BOOLEAN::TRUE), BOOLEAN(0));
    uqm_lua_checked!(SetFlashRect(::core::ptr::null::<RECT>(), BOOLEAN::FALSE), BOOLEAN(0));
    return BOOLEAN(
        (GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
            & (C2Rust_Unnamed_2::CHECK_ABORT.0 as ::core::ffi::c_int
                | C2Rust_Unnamed_2::CHECK_LOAD.0 as ::core::ffi::c_int) == 0)
            as ::core::ffi::c_int as ::core::ffi::c_uint,
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
#[inline]
unsafe extern "C" fn SeedStr() -> *mut UNICODE {
    match optSeedType {
        0 => return b"None\0".as_ptr() as *const ::core::ffi::c_char as *mut UNICODE,
        1 => return b"Planet\0".as_ptr() as *const ::core::ffi::c_char as *mut UNICODE,
        2 => return b"MRQ\0".as_ptr() as *const ::core::ffi::c_char as *mut UNICODE,
        3 => return b"StarSeed\0".as_ptr() as *const ::core::ffi::c_char as *mut UNICODE,
        _ => return b"???\0".as_ptr() as *const ::core::ffi::c_char as *mut UNICODE,
    };
}
