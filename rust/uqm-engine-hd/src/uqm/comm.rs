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

// Rust translation of src/uqm/comm.c.
extern "C" {
    pub type string_table_entry;
    pub type string_table;
    pub type context_desc;
    pub type frame_desc;
    pub type font_desc;
    pub type drawable_desc;
    pub type tfb_soundsample;
    pub type tfb_soundchunk;
    fn GetTimeCounter() -> TimeCount;
    fn SleepThread(timePeriod: TimePeriod);
    fn SleepThreadUntil(wakeTime: TimeCount);
    fn __assert_fail(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    ) -> !;
    fn HFree(p: *mut ::core::ffi::c_void);
    fn HCalloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn ReinitQueue(pq: *mut QUEUE);
    fn CountLinks(pq: *mut QUEUE) -> COUNT;
    fn SetContext(Context: CONTEXT) -> CONTEXT;
    fn SetContextForeGroundColor(Color: Color) -> Color;
    fn SetContextBackGroundColor(Color: Color) -> Color;
    fn SetContextFGFrame(Frame: FRAME) -> FRAME;
    fn SetContextClipRect(pRect: *mut RECT) -> BOOLEAN;
    fn GetContextClipRect(pRect: *mut RECT) -> BOOLEAN;
    fn DrawStamp(pStamp: *mut STAMP);
    fn DrawFilledRectangle(pRect: *mut RECT);
    fn font_DrawText(pText: *mut TEXT);
    fn font_DrawTracedText(pText: *mut TEXT, text: Color, trace: Color);
    fn font_DrawTracedTextAlt(
        pText: *mut TEXT,
        text: Color,
        trace: Color,
        AltFontPtr: FONT,
        key: UniChar,
    );
    fn BatchGraphics();
    fn UnbatchGraphics();
    fn ClearDrawable();
    fn CreateContextAux(name: *const ::core::ffi::c_char) -> CONTEXT;
    fn DestroyContext(ContextRef: CONTEXT) -> BOOLEAN;
    fn CreateDrawable(
        CreateFlags: CREATE_FLAGS,
        width: SIZE,
        height: SIZE,
        num_frames: COUNT,
    ) -> DRAWABLE;
    fn DestroyDrawable(Drawable: DRAWABLE) -> BOOLEAN;
    fn GetFrameRect(Frame: FRAME, pRect: *mut RECT) -> BOOLEAN;
    fn LoadGraphicInstance(res: RESOURCE) -> *mut ::core::ffi::c_void;
    fn SetContextFont(Font: FONT) -> FONT;
    fn DestroyFont(FontRef: FONT) -> BOOLEAN;
    fn TextRect(pText: *mut TEXT, pRect: *mut RECT, pdelta: *mut BYTE) -> BOOLEAN;
    fn GetContextFontLeading(pheight: *mut SIZE) -> BOOLEAN;
    fn SetAbsFrameIndex(Frame: FRAME, FrameIndex: COUNT) -> FRAME;
    fn SetFrameTransparentColor(_: FRAME, _: Color);
    fn CaptureDrawable(Drawable: DRAWABLE) -> FRAME;
    fn ReleaseDrawable(Frame: FRAME) -> DRAWABLE;
    fn SetColorMap(ColorMapPtr: COLORMAPPTR) -> BOOLEAN;
    fn FadeScreen(fadeType: ScreenFadeType, TimeInterval: SIZE) -> DWORD;
    fn FlushColorXForms();
    fn GetColorMapAddress(_: COLORMAP) -> COLORMAPPTR;
    fn LoadStringTableInstance(res: RESOURCE) -> STRING_TABLE;
    fn DestroyStringTable(StringTable: STRING_TABLE) -> BOOLEAN;
    fn CaptureStringTable(StringTable: STRING_TABLE) -> STRING;
    fn ReleaseStringTable(String: STRING) -> STRING_TABLE;
    fn SetAbsStringTableIndex(String: STRING, StringTableIndex: COUNT) -> STRING;
    fn GetStringAddress(String: STRING) -> STRINGPTR;
    fn getCharFromString(ptr: *mut *const ::core::ffi::c_char) -> UniChar;
    fn utf8StringCopy(
        dst: *mut ::core::ffi::c_char,
        size: size_t,
        src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_uchar;
    fn LoadMusicInstance(res: RESOURCE) -> MUSIC_REF;
    fn DestroyMusic(MusicRef: MUSIC_REF) -> BOOLEAN;
    fn StopSound();
    fn FadeMusic(end_vol: BYTE, TimeInterval: SIZE) -> DWORD;
    fn SetMusicPosition();
    fn snprintf(
        _: *mut ::core::ffi::c_char,
        _: size_t,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn memset(
        _: *mut ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strcpy(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(_: *const ::core::ffi::c_char) -> size_t;
    static mut CurrentInputState: CONTROLLER_INPUT_STATE;
    static mut PulsedInputState: CONTROLLER_INPUT_STATE;
    fn UpdateInputState();
    fn FlushInput();
    fn DoInput(pInputState: *mut ::core::ffi::c_void, resetInput: BOOLEAN);
    static mut optWhichFonts: ::core::ffi::c_int;
    static mut optSmoothScroll: ::core::ffi::c_int;
    static mut resolutionFactor: ::core::ffi::c_uint;
    static mut optScrTrans: ::core::ffi::c_int;
    static mut optOrzCompFont: OPT_ENABLABLE;
    static mut optScopeStyle: ::core::ffi::c_int;
    static mut optWindowType: DWORD;
    static mut optSpeech: OPT_ENABLABLE;
    static mut optSubtitles: OPT_ENABLABLE;
    static mut CanvasWidth: ::core::ffi::c_int;
    static mut CanvasHeight: ::core::ffi::c_int;
    fn RepairSISBorder();
    fn DrawSISFrame();
    fn ClearSISRect(ClearFlags: BYTE);
    fn DrawHyperCoords(puniverse: POINT);
    fn DrawSISTitle(pStr: *mut UNICODE);
    fn DrawSISMessage(pStr: *const UNICODE);
    fn DrawAlienFrame(pSeq: *mut SEQUENCE, Num: COUNT, fullRedraw: BOOLEAN) -> BOOLEAN;
    fn InitCommAnimations();
    fn ProcessCommAnimations(fullRedraw: BOOLEAN, paused: BOOLEAN) -> BOOLEAN;
    fn ShutYourMouth();
    fn DisengageFilters();
    fn setGameStateUint(name: *const ::core::ffi::c_char, val: DWORD);
    fn getGameStateUint(name: *const ::core::ffi::c_char) -> DWORD;
    static mut GlobData: GLOBDATA;
    static mut RadarContext: CONTEXT;
    fn inHQSpace() -> BOOLEAN;
    fn isPC(optWhich: ::core::ffi::c_int) -> OPT_CONSOLETYPE;
    fn is3DO(optWhich: ::core::ffi::c_int) -> OPT_CONSOLETYPE;
    fn PlayTrack();
    fn StopTrack();
    fn JumpTrack();
    fn PauseTrack();
    fn ResumeTrack();
    fn PlayingTrack() -> COUNT;
    fn GetSubtitleNumber(sub: *const UNICODE) -> COUNT;
    fn FastReverse_Smooth();
    fn FastForward_Smooth();
    fn FastReverse_Page();
    fn FastForward_Page();
    fn GetFirstTrackSubtitle() -> SUBTITLE_REF;
    fn GetNextTrackSubtitle(LastRef: SUBTITLE_REF) -> SUBTITLE_REF;
    fn GetTrackSubtitleText(SubRef: SUBTITLE_REF) -> *const UNICODE;
    fn GetTrackSubtitle() -> *const UNICODE;
    fn GetSubtitleNumberByTrack(track: COUNT) -> COUNT;
    fn RecalculateDelay(numChars: DWORD, talk: BOOLEAN) -> DWORD;
    fn init_race(comm_id: CONVERSATION) -> *mut LOCDATA;
    static mut pSolarSysState: *mut SOLARSYS_STATE;
    fn worldIsMoon(solarSys: *const SOLARSYS_STATE, world: *const PLANET_DESC) -> bool;
    fn moonIndex(solarSys: *const SOLARSYS_STATE, moon: *const PLANET_DESC) -> COUNT;
    fn GetNamedPlanetaryBody() -> *mut UNICODE;
    static mut CurStarDescPtr: *mut STAR_DESC;
    fn CloneShipFragment(
        shipIndex: RACE_ID,
        pDstQueue: *mut QUEUE,
        crew_level: COUNT,
    ) -> HSHIPFRAG;
    fn GetStarShipFromIndex(pShipQ: *mut QUEUE, Index: COUNT) -> HLINK;
    fn StartSphereTracking(race: RACE_ID) -> COUNT;
    static mut sliderDisabled: BOOLEAN;
    static mut oscillDisabled: BOOLEAN;
    fn InitOscilloscope(scopeBg: FRAME);
    fn DrawOscilloscope();
    fn UninitOscilloscope();
    fn InitSlider(
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
        width: ::core::ffi::c_int,
        sliderFrame: FRAME,
        buttonFrame: FRAME,
    );
    fn SetSliderImage(f: FRAME);
    fn DrawSlider();
    fn StopMusic();
    fn PlayMusicResume(hMusic: MUSIC_REF, Volume: BYTE);
    static mut GameStrings: STRING;
    static mut NextActivity: ACTIVITY;
    fn EncounterBattle();
    fn BuildBattle(which_player: COUNT);
    fn InitEncounter() -> COUNT;
    fn UninitEncounter() -> COUNT;
    static mut Screen: FRAME;
    static mut ActivityFrame: FRAME;
    static mut ScreenContext: CONTEXT;
    static mut SpaceContext: CONTEXT;
    static mut TinyFont: FONT;
    static mut VolasPackPresent: BOOLEAN;
    static mut LastActivity: ACTIVITY;
    static mut usingSpeech: BOOLEAN;
    fn load_gravity_well(selector: BYTE);
    fn free_gravity_well();
    fn SetMenuSounds(sound_0: MENU_SOUND_FLAGS, sound_1: MENU_SOUND_FLAGS);
    static mut disableInteractivity: BOOLEAN;
    fn luaUqm_comm_stringNeedsInterpolate(str: *const ::core::ffi::c_char) -> BOOLEAN;
    fn luaUqm_comm_stringInterpolate(
        str: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn SetTransitionSource(pRect: *const RECT);
    fn ScreenTransition(transition: ::core::ffi::c_int, pRect: *const RECT);
    fn log_add(_: log_Level, fmt: *const ::core::ffi::c_char, ...);
    fn DrawBorder(Visible: BYTE);
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
pub type TimeCount = DWORD;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Color {
    pub r: BYTE,
    pub g: BYTE,
    pub b: BYTE,
    pub a: BYTE,
}
pub type RESOURCE = *const ::core::ffi::c_char;
pub type UniChar = uint32;
pub type STRING_TABLE_ENTRY_DESC = string_table_entry;
pub type STRING_TABLE_DESC = string_table;
pub type STRING_TABLE = *mut STRING_TABLE_DESC;
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
pub type CREATE_FLAGS = BYTE;
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
pub struct ScreenFadeType(pub ::core::ffi::c_uint);
impl ScreenFadeType {
    pub const FadeAllToWhite: Self = Self(250);
    pub const FadeSomeToWhite: Self = Self(251);
    pub const FadeAllToBlack: Self = Self(252);
    pub const FadeAllToColor: Self = Self(253);
    pub const FadeSomeToBlack: Self = Self(254);
    pub const FadeSomeToColor: Self = Self(255);
}
pub type TFB_SoundSample = tfb_soundsample;
pub type MUSIC_REF = *mut *mut TFB_SoundSample;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _controller_input_state {
    pub key: [[::core::ffi::c_int; 8]; 2],
    pub menu: [::core::ffi::c_int; 31],
}
pub type CONTROLLER_INPUT_STATE = _controller_input_state;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct OPT_ENABLABLE(pub ::core::ffi::c_uint);
impl OPT_ENABLABLE {
    pub const OPTVAL_DISABLED: Self = Self(0);
    pub const OPTVAL_ENABLED: Self = Self(1);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct OPT_CONSOLETYPE(pub ::core::ffi::c_uint);
impl OPT_CONSOLETYPE {
    pub const OPTVAL_PC: Self = Self(0);
    pub const OPTVAL_3DO: Self = Self(1);
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_0 {
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ANIM_DIR(pub ::core::ffi::c_int);
impl ANIM_DIR {
    pub const DOWN_DIR: Self = Self(-1);
    pub const NO_DIR: Self = Self(0);
    pub const UP_DIR: Self = Self(1);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ANIM_TYPE(pub ::core::ffi::c_uint);
impl ANIM_TYPE {
    pub const PICTURE_ANIM: Self = Self(0);
    pub const COLOR_ANIM: Self = Self(1);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SEQUENCE {
    pub ADPtr: *mut ANIMATION_DESC,
    pub Alarm: DWORD,
    pub Direction: ANIM_DIR,
    pub CurIndex: COUNT,
    pub NextIndex: COUNT,
    pub FramesLeft: COUNT,
    pub AnimType: ANIM_TYPE,
    pub Change: BOOLEAN,
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
pub struct C2Rust_Unnamed_1(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_1 {
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
pub type SUBTITLE_REF = *mut tfb_soundchunk;
pub type RESPONSE_REF = COUNT;
pub type RESPONSE_FUNC = Option<unsafe extern "C" fn(RESPONSE_REF) -> ()>;
pub type ENCOUNTER_STATE = encounter_state;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct encounter_state {
    pub InputFunc: Option<unsafe extern "C" fn(*mut encounter_state) -> BOOLEAN>,
    pub Initialized: COUNT,
    pub NextTime: TimeCount,
    pub num_responses: BYTE,
    pub cur_response: BYTE,
    pub top_response: BYTE,
    pub response_list: [RESPONSE_ENTRY; 8],
    pub phrase_buf: [UNICODE; 1024],
}
pub type RESPONSE_ENTRY = response_entry;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct response_entry {
    pub response_ref: RESPONSE_REF,
    pub response_text: TEXT,
    pub response_func: RESPONSE_FUNC,
    pub allocedResponse: *mut ::core::ffi::c_char,
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
pub type CUSTOM_BASELINE = CustomBaseline;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CustomBaseline {
    pub index: COUNT,
    pub baseline: POINT,
    pub align: TEXT_ALIGN,
    pub next: *mut CustomBaseline,
}
pub type TALKING_STATE = talking_state;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct talking_state {
    pub InputFunc: Option<unsafe extern "C" fn(*mut talking_state) -> BOOLEAN>,
    pub NextTime: TimeCount,
    pub waitTrack: COUNT,
    pub rewind: bool,
    pub seeking: bool,
    pub ended: bool,
}
pub type MENU_SOUND_FLAGS = UWORD;
pub type SUMMARY_STATE = summary_state;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct summary_state {
    pub InputFunc: Option<unsafe extern "C" fn(*mut summary_state) -> BOOLEAN>,
    pub Initialized: BOOLEAN,
    pub PrintNext: BOOLEAN,
    pub NextSub: SUBTITLE_REF,
    pub LeftOver: *const UNICODE,
}
pub type LAST_REPLAY_STATE = last_replay_state;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct last_replay_state {
    pub InputFunc: Option<unsafe extern "C" fn(*mut last_replay_state) -> BOOLEAN>,
    pub NextTime: TimeCount,
    pub TimeOut: TimeCount,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct CommIntroMode(pub ::core::ffi::c_uint);
impl CommIntroMode {
    pub const CIM_CROSSFADE_SPACE: Self = Self(0);
    pub const CIM_CROSSFADE_WINDOW: Self = Self(1);
    pub const CIM_CROSSFADE_SCREEN: Self = Self(2);
    pub const CIM_FADE_IN_SCREEN: Self = Self(3);
    pub const CIM_DEFAULT: Self = Self(0);
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
pub type PLANET_DESC = planet_desc;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct planet_desc {
    pub rand_seed: DWORD,
    pub data_index: BYTE,
    pub NumPlanets: BYTE,
    pub radius: SIZE,
    pub location: POINT,
    pub temp_color: Color,
    pub NextIndex: COUNT,
    pub image: STAMP,
    pub pPrevDesc: *mut PLANET_DESC,
    pub orbit: PLANET_ORBIT,
    pub size: COUNT,
    pub angle: COUNT,
    pub rotFrameIndex: ::core::ffi::c_int,
    pub rotPointIndex: ::core::ffi::c_int,
    pub rotwidth: ::core::ffi::c_int,
    pub rotheight: ::core::ffi::c_int,
    pub orb_speed: ::core::ffi::c_double,
    pub rot_speed: ::core::ffi::c_double,
    pub PlanetByte: BYTE,
    pub MoonByte: BYTE,
    pub frame_offset: BYTE,
}
pub type PLANET_ORBIT = planet_orbit;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct planet_orbit {
    pub TopoZoomFrame: FRAME,
    pub lpTopoData: *mut SBYTE,
    pub SphereFrame: FRAME,
    pub ObjectFrame: FRAME,
    pub TintFrame: FRAME,
    pub TintColor: Color,
    pub TopoColors: *mut Color,
    pub ScanColors: *mut *mut Color,
    pub ScratchArray: *mut Color,
    pub WorkFrame: FRAME,
    pub BackFrame: FRAME,
    pub light_diff: *mut *mut DWORD,
    pub map_rotate: *mut *mut MAP3D_POINT,
    pub TopoMask: FRAME,
    pub sphereBytes: *mut BYTE,
    pub sphereMap: COLORMAP,
    pub scanType: COUNT,
    pub Shade: FRAME,
    pub ShadeColors: *mut Color,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MAP3D_POINT {
    pub p: [POINT; 4],
    pub m: [DWORD; 4],
}
pub type SOLARSYS_STATE = solarsys_state;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct solarsys_state {
    pub InputFunc: Option<unsafe extern "C" fn(*mut solarsys_state) -> BOOLEAN>,
    pub InIpFlight: BOOLEAN,
    pub WaitIntersect: COUNT,
    pub SunDesc: [PLANET_DESC; 1],
    pub PlanetDesc: [PLANET_DESC; 16],
    pub MoonDesc: [PLANET_DESC; 4],
    pub pBaseDesc: *mut PLANET_DESC,
    pub pOrbitalDesc: *mut PLANET_DESC,
    pub FirstPlanetIndex: SIZE,
    pub LastPlanetIndex: SIZE,
    pub turn_counter: BYTE,
    pub turn_wait: BYTE,
    pub thrust_counter: BYTE,
    pub max_ship_speed: BYTE,
    pub XlatRef: STRING,
    pub XlatPtr: *const ::core::ffi::c_void,
    pub OrbitalCMap: COLORMAP,
    pub SysInfo: SYSTEM_INFO,
    pub genFuncs: *const GenerateFunctions,
    pub PlanetSideFrame: [FRAME; 6],
    pub TopoFrame: FRAME,
    pub Orbit: PLANET_ORBIT,
    pub InOrbit: BOOLEAN,
    pub ScanFrame: [FRAME; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GenerateFunctions {
    pub initNpcs: InitNpcsFunction,
    pub reinitNpcs: ReinitNpcsFunction,
    pub uninitNpcs: UninitNpcsFunction,
    pub generatePlanets: GeneratePlanetsFunction,
    pub generateMoons: GenerateMoonsFunction,
    pub generateName: GenerateNameFunction,
    pub generateOrbital: GenerateOrbitalFunction,
    pub generateMinerals: GenerateMineralsFunction,
    pub generateEnergy: GenerateEnergyFunction,
    pub generateLife: GenerateLifeFunction,
    pub pickupMinerals: PickupMineralsFunction,
    pub pickupEnergy: PickupEnergyFunction,
    pub pickupLife: PickupLifeFunction,
}
pub type PickupLifeFunction = Option<
    unsafe extern "C" fn(*mut SOLARSYS_STATE, *mut PLANET_DESC, COUNT) -> bool,
>;
pub type PickupEnergyFunction = Option<
    unsafe extern "C" fn(*mut SOLARSYS_STATE, *mut PLANET_DESC, COUNT) -> bool,
>;
pub type PickupMineralsFunction = Option<
    unsafe extern "C" fn(*mut SOLARSYS_STATE, *mut PLANET_DESC, COUNT) -> bool,
>;
pub type GenerateLifeFunction = Option<
    unsafe extern "C" fn(
        *const SOLARSYS_STATE,
        *const PLANET_DESC,
        COUNT,
        *mut NODE_INFO,
    ) -> COUNT,
>;
pub type NODE_INFO = node_info;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct node_info {
    pub loc_pt: POINT,
    pub density: COUNT,
    pub r#type: COUNT,
}
pub type GenerateEnergyFunction = Option<
    unsafe extern "C" fn(
        *const SOLARSYS_STATE,
        *const PLANET_DESC,
        COUNT,
        *mut NODE_INFO,
    ) -> COUNT,
>;
pub type GenerateMineralsFunction = Option<
    unsafe extern "C" fn(
        *const SOLARSYS_STATE,
        *const PLANET_DESC,
        COUNT,
        *mut NODE_INFO,
    ) -> COUNT,
>;
pub type GenerateOrbitalFunction = Option<
    unsafe extern "C" fn(*mut SOLARSYS_STATE, *mut PLANET_DESC) -> bool,
>;
pub type GenerateNameFunction = Option<
    unsafe extern "C" fn(*const SOLARSYS_STATE, *const PLANET_DESC) -> bool,
>;
pub type GenerateMoonsFunction = Option<
    unsafe extern "C" fn(*mut SOLARSYS_STATE, *mut PLANET_DESC) -> bool,
>;
pub type GeneratePlanetsFunction = Option<
    unsafe extern "C" fn(*mut SOLARSYS_STATE) -> bool,
>;
pub type UninitNpcsFunction = Option<unsafe extern "C" fn(*mut SOLARSYS_STATE) -> bool>;
pub type ReinitNpcsFunction = Option<unsafe extern "C" fn(*mut SOLARSYS_STATE) -> bool>;
pub type InitNpcsFunction = Option<unsafe extern "C" fn(*mut SOLARSYS_STATE) -> bool>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SYSTEM_INFO {
    pub StarSize: BYTE,
    pub StarIntensity: BYTE,
    pub StarEnergy: UWORD,
    pub PlanetInfo: PLANET_INFO,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PLANET_INFO {
    pub AxialTilt: SIZE,
    pub Tectonics: UWORD,
    pub Weather: UWORD,
    pub PlanetDensity: UWORD,
    pub PlanetRadius: UWORD,
    pub SurfaceGravity: UWORD,
    pub SurfaceTemperature: SIZE,
    pub RotationPeriod: UWORD,
    pub AtmoDensity: UWORD,
    pub LifeChance: SIZE,
    pub PlanetToSunDist: DWORD,
    pub PlanDataPtr: *const PlanetFrame,
    pub ScanSeed: [DWORD; 3],
    pub ScanRetrieveMask: [DWORD; 3],
    pub PartiallyScavengedList: [[BYTE; 32]; 3],
    pub DiscoveryString: STRING,
    pub LanderFont: FONT,
    pub LanderFontEff: FRAME,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PlanetFrame {
    pub Type: BYTE,
    pub BaseTectonics: BYTE,
    pub AtmoAndDensity: BYTE,
    pub UsefulElements: [ELEMENT_ENTRY; 8],
    pub CMapInstance: RESOURCE,
    pub XlatTabInstance: RESOURCE,
    pub num_faults: COUNT,
    pub fault_depth: SIZE,
    pub num_blemishes: COUNT,
    pub base_elevation: SIZE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ELEMENT_ENTRY {
    pub ElementType: BYTE,
    pub Density: BYTE,
}
pub type HENCOUNTER = HLINK;
pub type HSHIPFRAG = HLINK;
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
pub type BRIEF_SHIP_INFO = brief_ship_info;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct brief_ship_info {
    pub race_id: BYTE,
    pub crew_level: COUNT,
    pub max_crew: COUNT,
    pub max_energy: BYTE,
}
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
pub type HELEMENT = HLINK;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_2(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_2 {
    pub const FIRST_ROCKY_WORLD: Self = Self(0);
    pub const FIRST_SMALL_ROCKY_WORLD: Self = Self(0);
    pub const OOLITE_WORLD: Self = Self(0);
    pub const YTTRIC_WORLD: Self = Self(1);
    pub const QUASI_DEGENERATE_WORLD: Self = Self(2);
    pub const LANTHANIDE_WORLD: Self = Self(3);
    pub const TREASURE_WORLD: Self = Self(4);
    pub const UREA_WORLD: Self = Self(5);
    pub const METAL_WORLD: Self = Self(6);
    pub const RADIOACTIVE_WORLD: Self = Self(7);
    pub const OPALESCENT_WORLD: Self = Self(8);
    pub const CYANIC_WORLD: Self = Self(9);
    pub const ACID_WORLD: Self = Self(10);
    pub const ALKALI_WORLD: Self = Self(11);
    pub const HALIDE_WORLD: Self = Self(12);
    pub const GREEN_WORLD: Self = Self(13);
    pub const COPPER_WORLD: Self = Self(14);
    pub const CARBIDE_WORLD: Self = Self(15);
    pub const ULTRAMARINE_WORLD: Self = Self(16);
    pub const NOBLE_WORLD: Self = Self(17);
    pub const AZURE_WORLD: Self = Self(18);
    pub const CHONDRITE_WORLD: Self = Self(19);
    pub const PURPLE_WORLD: Self = Self(20);
    pub const SUPER_DENSE_WORLD: Self = Self(21);
    pub const PELLUCID_WORLD: Self = Self(22);
    pub const DUST_WORLD: Self = Self(23);
    pub const CRIMSON_WORLD: Self = Self(24);
    pub const CIMMERIAN_WORLD: Self = Self(25);
    pub const INFRARED_WORLD: Self = Self(26);
    pub const SELENIC_WORLD: Self = Self(27);
    pub const AURIC_WORLD: Self = Self(28);
    pub const LAST_SMALL_ROCKY_WORLD: Self = Self(28);
    pub const FIRST_LARGE_ROCKY_WORLD: Self = Self(29);
    pub const FLUORESCENT_WORLD: Self = Self(29);
    pub const ULTRAVIOLET_WORLD: Self = Self(30);
    pub const PLUTONIC_WORLD: Self = Self(31);
    pub const RAINBOW_WORLD: Self = Self(32);
    pub const SHATTERED_WORLD: Self = Self(33);
    pub const SAPPHIRE_WORLD: Self = Self(34);
    pub const ORGANIC_WORLD: Self = Self(35);
    pub const XENOLITHIC_WORLD: Self = Self(36);
    pub const REDUX_WORLD: Self = Self(37);
    pub const PRIMORDIAL_WORLD: Self = Self(38);
    pub const EMERALD_WORLD: Self = Self(39);
    pub const CHLORINE_WORLD: Self = Self(40);
    pub const MAGNETIC_WORLD: Self = Self(41);
    pub const WATER_WORLD: Self = Self(42);
    pub const TELLURIC_WORLD: Self = Self(43);
    pub const HYDROCARBON_WORLD: Self = Self(44);
    pub const IODINE_WORLD: Self = Self(45);
    pub const VINYLOGOUS_WORLD: Self = Self(46);
    pub const RUBY_WORLD: Self = Self(47);
    pub const MAGMA_WORLD: Self = Self(48);
    pub const MAROON_WORLD: Self = Self(49);
    pub const LAST_LARGE_ROCKY_WORLD: Self = Self(49);
    pub const LAST_ROCKY_WORLD: Self = Self(49);
    pub const FIRST_GAS_GIANT: Self = Self(50);
    pub const BLU_GAS_GIANT: Self = Self(50);
    pub const CYA_GAS_GIANT: Self = Self(51);
    pub const GRN_GAS_GIANT: Self = Self(52);
    pub const GRY_GAS_GIANT: Self = Self(53);
    pub const ORA_GAS_GIANT: Self = Self(54);
    pub const PUR_GAS_GIANT: Self = Self(55);
    pub const RED_GAS_GIANT: Self = Self(56);
    pub const VIO_GAS_GIANT: Self = Self(57);
    pub const YEL_GAS_GIANT: Self = Self(58);
    pub const LAST_GAS_GIANT: Self = Self(58);
    pub const NUMBER_OF_PLANET_TYPES: Self = Self(59);
    pub const WORLD_TYPE_SPECIAL: Self = Self(128);
    pub const PLANET_SHIELDED: Self = Self(128);
    pub const HIERARCHY_STARBASE: Self = Self(255);
    pub const SA_MATRA: Self = Self(254);
    pub const DESTROYED_STARBASE: Self = Self(253);
    pub const PRECURSOR_STARBASE: Self = Self(252);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_3(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_3 {
    pub const HAIL: Self = Self(0);
    pub const ATTACK: Self = Self(1);
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
#[inline]
unsafe extern "C" fn MAKE_EXTENT(mut width: COORD, mut height: COORD) -> EXTENT {
    let mut ext: EXTENT = extent {
        width: width,
        height: height,
    };
    return ext;
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
static mut speed_array: [BYTE; 5] = [
    0 as BYTE,
    12 as BYTE,
    16 as BYTE,
    24 as BYTE,
    32 as BYTE,
];
static mut AnimContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
#[no_mangle]
pub static mut CommData: LOCDATA = LOCDATA {
    AlienConv: CONVERSATION::ARILOU_CONVERSATION,
    init_encounter_func: None,
    post_encounter_func: None,
    uninit_encounter_func: None,
    AlienFrameRes: ::core::ptr::null::<::core::ffi::c_char>(),
    AlienFontRes: ::core::ptr::null::<::core::ffi::c_char>(),
    AlienTextFColor: Color { r: 0, g: 0, b: 0, a: 0 },
    AlienTextBColor: Color { r: 0, g: 0, b: 0, a: 0 },
    AlienTextBaseline: POINT { x: 0, y: 0 },
    AlienTextWidth: 0,
    AlienTextAlign: TEXT_ALIGN::ALIGN_LEFT,
    AlienTextValign: TEXT_VALIGN::VALIGN_TOP,
    AlienColorMapRes: ::core::ptr::null::<::core::ffi::c_char>(),
    AlienSongRes: ::core::ptr::null::<::core::ffi::c_char>(),
    AltRes: ALT_RESOURCE {
        AlienFrameRes: ::core::ptr::null::<::core::ffi::c_char>(),
        AlienColorMapRes: ::core::ptr::null::<::core::ffi::c_char>(),
        AlienSongRes: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    ConversationPhrasesRes: ::core::ptr::null::<::core::ffi::c_char>(),
    NumAnimations: 0,
    AlienAmbientArray: [ANIMATION_DESC {
        StartIndex: 0,
        NumFrames: 0,
        AnimFlags: 0,
        BaseFrameRate: 0,
        RandomFrameRate: 0,
        BaseRestartRate: 0,
        RandomRestartRate: 0,
        BlockMask: 0,
    }; 30],
    AlienTransitionDesc: ANIMATION_DESC {
        StartIndex: 0,
        NumFrames: 0,
        AnimFlags: 0,
        BaseFrameRate: 0,
        RandomFrameRate: 0,
        BaseRestartRate: 0,
        RandomRestartRate: 0,
        BlockMask: 0,
    },
    AlienTalkDesc: ANIMATION_DESC {
        StartIndex: 0,
        NumFrames: 0,
        AnimFlags: 0,
        BaseFrameRate: 0,
        RandomFrameRate: 0,
        BaseRestartRate: 0,
        RandomRestartRate: 0,
        BlockMask: 0,
    },
    AlienNumberSpeech: ::core::ptr::null::<NUMBER_SPEECH_DESC>(),
    AlienFrame: ::core::ptr::null_mut::<FRAME_DESC>(),
    AlienFont: ::core::ptr::null_mut::<FONT_DESC>(),
    AlienColorMap: ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>(),
    AlienSong: ::core::ptr::null_mut::<*mut TFB_SoundSample>(),
    ConversationPhrases: ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>(),
};
#[no_mangle]
pub static mut shared_phrase_buf: [UNICODE; 2048] = [0; 2048];
#[no_mangle]
pub static mut ComputerFont: FONT = ::core::ptr::null_mut::<FONT_DESC>();
static mut TalkingFinished: BOOLEAN = BOOLEAN::FALSE;
static mut curIntroMode: CommIntroMode = CommIntroMode::CIM_DEFAULT;
static mut fadeTime: TimeCount = 0;
#[no_mangle]
pub static mut IsDarkMode: BOOLEAN = BOOLEAN::FALSE;
#[no_mangle]
pub static mut cwLock: BOOLEAN = BOOLEAN::FALSE;
#[no_mangle]
pub static mut altResFlags: BYTE = 0 as BYTE;
static mut fadeIndex: COUNT = 0;
static mut head_node: *mut CUSTOM_BASELINE = ::core::ptr::null_mut::<CUSTOM_BASELINE>();
static mut cur_node: *mut CUSTOM_BASELINE = ::core::ptr::null_mut::<CUSTOM_BASELINE>();
static mut haveTalkingLock: BOOLEAN = BOOLEAN::FALSE;
static mut startSentence: COUNT = 0;
static mut endSentence: COUNT = 0;
static mut pCurInputState: *mut ENCOUNTER_STATE = ::core::ptr::null_mut::<
    ENCOUNTER_STATE,
>();
static mut clear_subtitles: BOOLEAN = BOOLEAN::FALSE;
static mut next_page: BOOLEAN = BOOLEAN::FALSE;
static mut SubtitleText: TEXT = TEXT {
    baseline: POINT { x: 0, y: 0 },
    pStr: ::core::ptr::null::<UNICODE>(),
    align: TEXT_ALIGN::ALIGN_LEFT,
    CharCount: 0,
};
static mut last_subtitle: *const UNICODE = ::core::ptr::null::<UNICODE>();
static mut TextCacheContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
static mut TextCacheFrame: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
#[no_mangle]
pub static mut CommWndRect: RECT = rect {
    corner: point {
        x: 0 as COORD,
        y: 0 as COORD,
    },
    extent: extent {
        width: 0 as COORD,
        height: 0 as COORD,
    },
};
unsafe extern "C" fn _count_lines(mut pText: *mut TEXT) -> ::core::ffi::c_int {
    let mut text_width: SIZE = 0;
    let mut pStr: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    let mut numLines: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut eol: BOOLEAN = BOOLEAN::FALSE;
    text_width = CommData.AlienTextWidth as SIZE;
    SetContextFont(CommData.AlienFont);
    pStr = (*pText).pStr as *const ::core::ffi::c_char;
    loop {
        numLines += 1;
        (*pText).pStr = pStr as *const UNICODE;
        eol = getLineWithinWidth(
            pText,
            &raw mut pStr,
            text_width,
            !(0 as ::core::ffi::c_int) as COUNT,
        );
        if eol.0 != 0 {
            break;
        }
    }
    (*pText).pStr = pStr as *const UNICODE;
    return numLines;
}
unsafe extern "C" fn add_text(
    mut status: ::core::ffi::c_int,
    mut pTextIn: *mut TEXT,
) -> COORD {
    let mut maxchars: COUNT = 0;
    let mut numchars: COUNT = 0;
    let mut locText: TEXT = TEXT {
        baseline: POINT { x: 0, y: 0 },
        pStr: ::core::ptr::null::<UNICODE>(),
        align: TEXT_ALIGN::ALIGN_LEFT,
        CharCount: 0,
    };
    let mut pText: *mut TEXT = ::core::ptr::null_mut::<TEXT>();
    let mut leading: SIZE = 0;
    let mut pStr: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    let mut text_width: SIZE = 0;
    let mut num_lines: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    static mut last_baseline: COORD = 0;
    let mut eol: BOOLEAN = BOOLEAN::FALSE;
    let mut OldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    let mut arrow: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    BatchGraphics();
    GetFrameRect(SetAbsFrameIndex(ActivityFrame, 6 as COUNT), &raw mut arrow);
    maxchars = !(0 as ::core::ffi::c_int) as COUNT;
    if status == 1 as ::core::ffi::c_int {
        if last_subtitle == (*pTextIn).pStr {
            let mut s: STAMP = STAMP {
                origin: POINT { x: 0, y: 0 },
                frame: ::core::ptr::null_mut::<FRAME_DESC>(),
            };
            s.origin.x = 0 as COORD;
            s.origin.y = 0 as COORD;
            s.frame = TextCacheFrame;
            uqm_lua_checked!(DrawStamp(&raw mut s), 0);
            UnbatchGraphics();
            return last_baseline;
        } else {
            OldContext = SetContext(TextCacheContext);
            uqm_lua_checked!(ClearDrawable(), 0);
            last_subtitle = (*pTextIn).pStr;
        }
        text_width = CommData.AlienTextWidth as SIZE;
        if CommData.AlienConv.0 == CONVERSATION::ORZ_CONVERSATION.0
            && optOrzCompFont.0 != 0
            && CommData.AlienTalkDesc.AnimFlags as ::core::ffi::c_int
                & (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int != 0
        {
            SetContextFont(ComputerFont);
        } else {
            SetContextFont(CommData.AlienFont);
        }
        GetContextFontLeading(&raw mut leading);
        pText = pTextIn;
    } else {
        GetContextFontLeading(&raw mut leading);
        if status <= -4 as ::core::ffi::c_int {
            text_width = (CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
                - ((8 as ::core::ffi::c_int) << resolutionFactor)
                - ((1 as ::core::ffi::c_int) << resolutionFactor
                    << 2 as ::core::ffi::c_int)
                - (arrow.extent.width as ::core::ffi::c_int
                    + ((2 as ::core::ffi::c_int) << resolutionFactor))) as SIZE;
            pText = pTextIn;
        } else {
            text_width = (CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
                - ((8 as ::core::ffi::c_int) << resolutionFactor)
                - ((1 as ::core::ffi::c_int) << resolutionFactor
                    << 2 as ::core::ffi::c_int)
                - (arrow.extent.width as ::core::ffi::c_int
                    + ((2 as ::core::ffi::c_int) << resolutionFactor))) as SIZE;
            match status {
                -3 => {
                    if IsDarkMode.0 == 0 {
                        SetContextForeGroundColor(
                            colorFromRgb15(0 as BYTE, 0 as BYTE, 0x14 as BYTE),
                        );
                    } else {
                        SetContextForeGroundColor(
                            colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE),
                        );
                    }
                }
                -2 => {
                    if IsDarkMode.0 == 0 {
                        SetContextForeGroundColor(
                            colorFromRgb15(0 as BYTE, 0x14 as BYTE, 0x14 as BYTE),
                        );
                    } else {
                        SetContextForeGroundColor(
                            buildColorRgba(
                                0 as BYTE,
                                0 as BYTE,
                                0xaa as BYTE,
                                0xff as BYTE,
                            ),
                        );
                    }
                }
                -1 => {
                    if IsDarkMode.0 == 0 {
                        SetContextForeGroundColor(
                            colorFromRgb15(0x1a as BYTE, 0x1a as BYTE, 0x1a as BYTE),
                        );
                    } else {
                        SetContextForeGroundColor(
                            buildColorRgba(
                                0x55 as BYTE,
                                0x55 as BYTE,
                                0xff as BYTE,
                                0xff as BYTE,
                            ),
                        );
                    }
                }
                _ => {}
            }
            maxchars = (*pTextIn).CharCount;
            locText = *pTextIn;
            locText.baseline.x = (locText.baseline.x as ::core::ffi::c_int
                - ((8 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
            locText.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
            locText.pStr = b"\xE2\x80\xA2\0".as_ptr() as *const ::core::ffi::c_char
                as *const UNICODE;
            uqm_lua_checked!(font_DrawText(&raw mut locText), 0);
            locText = *pTextIn;
            pText = &raw mut locText;
            (*pText).baseline.y = ((*pText).baseline.y as ::core::ffi::c_int
                - leading as ::core::ffi::c_int) as COORD;
        }
    }
    numchars = 0 as COUNT;
    pStr = (*pText).pStr as *const ::core::ffi::c_char;
    if status > 0 as ::core::ffi::c_int
        && CommData.AlienTextValign.0
            & (TEXT_VALIGN::VALIGN_MIDDLE.0 as ::core::ffi::c_int
                | TEXT_VALIGN::VALIGN_BOTTOM.0 as ::core::ffi::c_int)
                as ::core::ffi::c_uint != 0
    {
        num_lines = _count_lines(pText);
        if CommData.AlienTextValign.0 == TEXT_VALIGN::VALIGN_BOTTOM.0 {
            (*pText).baseline.y = ((*pText).baseline.y as ::core::ffi::c_int
                - leading as ::core::ffi::c_int * num_lines) as COORD;
        } else if CommData.AlienTextValign.0 == TEXT_VALIGN::VALIGN_MIDDLE.0 {
            (*pText).baseline.y = ((*pText).baseline.y as ::core::ffi::c_int
                - leading as ::core::ffi::c_int * num_lines / 2 as ::core::ffi::c_int)
                as COORD;
        }
        if ((*pText).baseline.y as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
            (*pText).baseline.y = 0 as COORD;
        }
    }
    loop {
        (*pText).pStr = pStr as *const UNICODE;
        (*pText).baseline.y = ((*pText).baseline.y as ::core::ffi::c_int
            + leading as ::core::ffi::c_int) as COORD;
        eol = getLineWithinWidth(pText, &raw mut pStr, text_width, maxchars);
        maxchars = (maxchars as ::core::ffi::c_int
            - (*pText).CharCount as ::core::ffi::c_int) as COUNT;
        if maxchars as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            maxchars = maxchars.wrapping_sub(1);
        }
        numchars = (numchars as ::core::ffi::c_int
            + (*pText).CharCount as ::core::ffi::c_int) as COUNT;
        if status <= 0 as ::core::ffi::c_int {
            if ((*pText).baseline.y as ::core::ffi::c_int)
                < CanvasHeight
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
            {
                uqm_lua_checked!(font_DrawText(pText), 0);
            }
            if status < -4 as ::core::ffi::c_int
                && (*pText).baseline.y as ::core::ffi::c_int
                    >= -status - 10 as ::core::ffi::c_int
            {
                pStr = pStr.offset(1);
                break;
            }
        } else if CommData.AlienConv.0 == CONVERSATION::ORZ_CONVERSATION.0
            && optOrzCompFont.0 != 0
        {
            uqm_lua_checked!(font_DrawTracedTextAlt(
                pText,
                CommData.AlienTextFColor,
                CommData.AlienTextBColor,
                ComputerFont,
                '*' as UniChar,
            ), 0);
        } else {
            uqm_lua_checked!(font_DrawTracedText(
                pText,
                CommData.AlienTextFColor,
                CommData.AlienTextBColor,
            ), 0);
        }
        if !(eol.0 == 0 && maxchars as ::core::ffi::c_int != 0) {
            break;
        }
    }
    (*pText).pStr = pStr as *const UNICODE;
    if status == 1 as ::core::ffi::c_int {
        let mut s_0: STAMP = STAMP {
            origin: POINT { x: 0, y: 0 },
            frame: ::core::ptr::null_mut::<FRAME_DESC>(),
        };
        SetContext(OldContext);
        s_0.origin.y = 0 as COORD;
        s_0.origin.x = s_0.origin.y;
        s_0.frame = TextCacheFrame;
        uqm_lua_checked!(DrawStamp(&raw mut s_0), 0);
        last_baseline = (*pText).baseline.y;
    }
    UnbatchGraphics();
    return (*pText).baseline.y;
}
#[no_mangle]
pub unsafe extern "C" fn GetCustomBaseline(mut i: COUNT) {
    if head_node.is_null() {
        return
    } else {
        cur_node = head_node;
        while !cur_node.is_null()
            && (*cur_node).index as ::core::ffi::c_int != i as ::core::ffi::c_int
        {
            cur_node = (*cur_node).next as *mut CUSTOM_BASELINE;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn CheckTalkingAnim(mut i: COUNT) {
    if haveTalkingLock.0 != 0 {
        if i as ::core::ffi::c_int >= startSentence as ::core::ffi::c_int
            && i as ::core::ffi::c_int <= endSentence as ::core::ffi::c_int
        {
            CommData.AlienTalkDesc.AnimFlags = (CommData.AlienTalkDesc.AnimFlags
                as ::core::ffi::c_int
                & !((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)) as COUNT;
            EnableTalkingAnim(BOOLEAN::FALSE);
            ShutYourMouth();
        } else {
            CommData.AlienTalkDesc.AnimFlags = (CommData.AlienTalkDesc.AnimFlags
                as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as COUNT;
            EnableTalkingAnim(BOOLEAN::TRUE);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn BlockTalkingAnim(mut trackStart: COUNT, mut trackEnd: COUNT) {
    if trackStart as ::core::ffi::c_int >= trackEnd as ::core::ffi::c_int {
        return;
    }
    haveTalkingLock = BOOLEAN::TRUE;
    startSentence = GetSubtitleNumberByTrack(trackStart);
    endSentence = (GetSubtitleNumberByTrack(trackEnd) as ::core::ffi::c_int
        - 1 as ::core::ffi::c_int) as COUNT;
}
#[no_mangle]
pub unsafe extern "C" fn ReleaseTalkingAnim() {
    if haveTalkingLock.0 != 0 {
        haveTalkingLock = BOOLEAN::FALSE;
        startSentence = !(0 as ::core::ffi::c_int) as COUNT;
        endSentence = !(0 as ::core::ffi::c_int) as COUNT;
        EnableTalkingAnim(BOOLEAN::TRUE);
    }
}
#[no_mangle]
pub unsafe extern "C" fn getLineWithinWidth(
    mut pText: *mut TEXT,
    mut startNext: *mut *const ::core::ffi::c_char,
    mut maxWidth: SIZE,
    mut maxChars: COUNT,
) -> BOOLEAN {
    let mut eol: BOOLEAN = BOOLEAN::FALSE;
    let mut done: BOOLEAN = BOOLEAN::FALSE;
    let mut rect: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut oldCount: COUNT = 0;
    let mut ptr: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut wordStart: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    let mut ch: UniChar = 0;
    let mut charCount: COUNT = 0;
    eol = BOOLEAN::FALSE;
    done = BOOLEAN::FALSE;
    oldCount = 1 as COUNT;
    charCount = 0 as COUNT;
    ch = '\0' as UniChar;
    ptr = (*pText).pStr as *const ::core::ffi::c_char;
    loop {
        wordStart = ptr;
        loop {
            if *ptr as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
                eol = BOOLEAN::TRUE;
                done = BOOLEAN::TRUE;
                break;
            } else {
                ch = getCharFromString(&raw mut ptr);
                eol = BOOLEAN(
                    (ch == '\0' as UniChar || ch == '\n' as UniChar
                        || ch == '\r' as UniChar) as ::core::ffi::c_int
                        as ::core::ffi::c_uint,
                );
                done = BOOLEAN(
                    (eol.0 != 0
                        || charCount as ::core::ffi::c_int
                            >= maxChars as ::core::ffi::c_int) as ::core::ffi::c_int
                        as ::core::ffi::c_uint,
                );
                if done.0 != 0 || ch == ' ' as UniChar {
                    break;
                }
                charCount = charCount.wrapping_add(1);
            }
        }
        oldCount = (*pText).CharCount;
        (*pText).CharCount = charCount;
        TextRect(pText, &raw mut rect, ::core::ptr::null_mut::<BYTE>());
        if rect.extent.width as ::core::ffi::c_int >= maxWidth as ::core::ffi::c_int {
            (*pText).CharCount = oldCount;
            *startNext = wordStart;
            return BOOLEAN::FALSE;
        }
        if done.0 != 0 {
            *startNext = ptr;
            return eol;
        }
        charCount = charCount.wrapping_add(1);
    };
}
#[no_mangle]
pub unsafe extern "C" fn DrawCommBorder(mut r: RECT) {
    let mut clipRect: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut oldClipRect: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    GetContextClipRect(&raw mut oldClipRect);
    clipRect = oldClipRect;
    clipRect.corner.x = (clipRect.corner.x as ::core::ffi::c_int
        - ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    clipRect.extent.width = (clipRect.extent.width as ::core::ffi::c_int
        + ((2 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    SetContextClipRect(&raw mut clipRect);
    SetContextForeGroundColor(colorFromRgb15(0xa as BYTE, 0xa as BYTE, 0xa as BYTE));
    r.corner.y = (r.corner.y as ::core::ffi::c_int
        - (if usingSpeech.0 == 0 && isPC(optSmoothScroll).0 != 0 {
            4 as ::core::ffi::c_int
        } else if if optWindowType == 0 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            4 as ::core::ffi::c_int
        } else {
            15 as ::core::ffi::c_int
        } << resolutionFactor)) as COORD;
    r.extent.width = clipRect.extent.width;
    r.extent.height = (if usingSpeech.0 == 0 && isPC(optSmoothScroll).0 != 0 {
        4 as ::core::ffi::c_int
    } else if if optWindowType == 0 as DWORD {
        BOOLEAN::TRUE.0 as ::core::ffi::c_int
    } else {
        BOOLEAN::FALSE.0 as ::core::ffi::c_int
    } != 0
    {
        4 as ::core::ffi::c_int
    } else {
        15 as ::core::ffi::c_int
    } << resolutionFactor) as COORD;
    uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
    SetContextForeGroundColor(colorFromRgb15(0x8 as BYTE, 0x8 as BYTE, 0x8 as BYTE));
    r.extent.height = ((1 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
    SetContextForeGroundColor(colorFromRgb15(0x10 as BYTE, 0x10 as BYTE, 0x10 as BYTE));
    r.corner.y = (r.corner.y as ::core::ffi::c_int
        + ((if usingSpeech.0 == 0 && isPC(optSmoothScroll).0 != 0 {
            4 as ::core::ffi::c_int
        } else if if optWindowType == 0 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            4 as ::core::ffi::c_int
        } else {
            15 as ::core::ffi::c_int
        } << resolutionFactor) - ((1 as ::core::ffi::c_int) << resolutionFactor)))
        as COORD;
    uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
    SetContextClipRect(&raw mut oldClipRect);
    uqm_lua_checked!(DrawBorder(C2Rust_Unnamed_0::SIS_COMM_FRAME.0 as ::core::ffi::c_int as BYTE), ());
}
unsafe extern "C" fn DrawSISComWindow() {
    let mut OldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    if GlobData.Game_state.CurrentActivity as BYTE as ::core::ffi::c_int
        != C2Rust_Unnamed_1::WON_LAST_BATTLE.0 as ::core::ffi::c_int
    {
        let mut r: RECT = RECT {
            corner: POINT { x: 0, y: 0 },
            extent: EXTENT { width: 0, height: 0 },
        };
        OldContext = SetContext(SpaceContext);
        r.corner.x = 0 as COORD;
        r.corner.y = (((107 as ::core::ffi::c_int) << resolutionFactor)
            + (if usingSpeech.0 == 0 && isPC(optSmoothScroll).0 != 0 {
                4 as ::core::ffi::c_int
            } else if if optWindowType == 0 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                4 as ::core::ffi::c_int
            } else {
                15 as ::core::ffi::c_int
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
                } << resolutionFactor))) as COORD;
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
            - r.corner.y as ::core::ffi::c_int) as COORD;
        if IsDarkMode.0 == 0 {
            SetContextForeGroundColor(
                colorFromRgb15(0 as BYTE, 0 as BYTE, 0x14 as BYTE),
            );
        } else {
            SetContextForeGroundColor(colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE));
        }
        uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
        if usingSpeech.0 == 0 && optSmoothScroll == 0x2 as ::core::ffi::c_int
            && IsDarkMode.0 == 0
        {
            uqm_lua_checked!(DrawCommBorder(r), ());
        }
        SetContext(OldContext);
    }
}
#[no_mangle]
pub unsafe extern "C" fn init_communication() {}
#[no_mangle]
pub unsafe extern "C" fn uninit_communication() {}
unsafe extern "C" fn RefreshResponsesSpecial(mut pES: *mut ENCOUNTER_STATE) {
    let mut y: COORD = 0;
    let mut response: BYTE = 0;
    let mut leading: SIZE = 0;
    SetContext(SpaceContext);
    GetContextFontLeading(&raw mut leading);
    BatchGraphics();
    uqm_lua_checked!(DrawSISComWindow(), ());
    y = ((((107 as ::core::ffi::c_int) << resolutionFactor)
        + (if usingSpeech.0 == 0 && isPC(optSmoothScroll).0 != 0 {
            4 as ::core::ffi::c_int
        } else if if optWindowType == 0 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            4 as ::core::ffi::c_int
        } else {
            15 as ::core::ffi::c_int
        } << resolutionFactor)) as ::core::ffi::c_uint)
        .wrapping_add(is3DO(optWhichFonts).0 << resolutionFactor) as COORD;
    response = (*pES).top_response;
    while (response as ::core::ffi::c_int) < (*pES).num_responses as ::core::ffi::c_int {
        (*pES).response_list[response as usize].response_text.baseline.x = (((1
            as ::core::ffi::c_int) << resolutionFactor)
            + ((8 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        (*pES).response_list[response as usize].response_text.baseline.y = (y
            as ::core::ffi::c_int + leading as ::core::ffi::c_int) as COORD;
        (*pES).response_list[response as usize].response_text.align = TEXT_ALIGN::ALIGN_LEFT;
        if response as ::core::ffi::c_int == (*pES).cur_response as ::core::ffi::c_int {
            y = uqm_lua_checked!(add_text(
                -1 as ::core::ffi::c_int,
                &raw mut (*(&raw mut (*pES).response_list as *mut RESPONSE_ENTRY)
                    .offset(response as isize))
                    .response_text,
            ), ());
        } else {
            y = uqm_lua_checked!(add_text(
                -3 as ::core::ffi::c_int,
                &raw mut (*(&raw mut (*pES).response_list as *mut RESPONSE_ENTRY)
                    .offset(response as isize))
                    .response_text,
            ), ());
        }
        response = response.wrapping_add(1);
    }
    UnbatchGraphics();
}
unsafe extern "C" fn RefreshResponses(mut pES: *mut ENCOUNTER_STATE) {
    let mut y: COORD = 0;
    let mut response: BYTE = 0;
    let mut leading: SIZE = 0;
    let mut s: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    SetContext(SpaceContext);
    GetContextFontLeading(&raw mut leading);
    BatchGraphics();
    uqm_lua_checked!(DrawSISComWindow(), ());
    y = ((((107 as ::core::ffi::c_int) << resolutionFactor)
        + (if usingSpeech.0 == 0 && isPC(optSmoothScroll).0 != 0 {
            4 as ::core::ffi::c_int
        } else if if optWindowType == 0 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            4 as ::core::ffi::c_int
        } else {
            15 as ::core::ffi::c_int
        } << resolutionFactor)) as ::core::ffi::c_uint)
        .wrapping_add(is3DO(optWhichFonts).0 << resolutionFactor) as COORD;
    response = (*pES).top_response;
    while (response as ::core::ffi::c_int) < (*pES).num_responses as ::core::ffi::c_int {
        (*pES).response_list[response as usize].response_text.baseline.x = (((1
            as ::core::ffi::c_int) << resolutionFactor)
            + ((8 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        (*pES).response_list[response as usize].response_text.baseline.y = (y
            as ::core::ffi::c_int + leading as ::core::ffi::c_int) as COORD;
        (*pES).response_list[response as usize].response_text.align = TEXT_ALIGN::ALIGN_LEFT;
        if response as ::core::ffi::c_int == (*pES).cur_response as ::core::ffi::c_int {
            y = uqm_lua_checked!(add_text(
                -1 as ::core::ffi::c_int,
                &raw mut (*(&raw mut (*pES).response_list as *mut RESPONSE_ENTRY)
                    .offset(response as isize))
                    .response_text,
            ), ());
        } else {
            y = uqm_lua_checked!(add_text(
                -2 as ::core::ffi::c_int,
                &raw mut (*(&raw mut (*pES).response_list as *mut RESPONSE_ENTRY)
                    .offset(response as isize))
                    .response_text,
            ), ());
        }
        response = response.wrapping_add(1);
    }
    if (*pES).top_response != 0 {
        s.origin.y = (((107 as ::core::ffi::c_int) << resolutionFactor)
            + (if usingSpeech.0 == 0 && isPC(optSmoothScroll).0 != 0 {
                4 as ::core::ffi::c_int
            } else if if optWindowType == 0 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                4 as ::core::ffi::c_int
            } else {
                15 as ::core::ffi::c_int
            } << resolutionFactor) + ((1 as ::core::ffi::c_int) << resolutionFactor))
            as COORD;
        s.frame = SetAbsFrameIndex(ActivityFrame, 6 as COUNT);
    } else if y as ::core::ffi::c_int
        > CanvasHeight
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
    {
        s.origin.y = (CanvasHeight
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
            - ((2 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        s.frame = SetAbsFrameIndex(ActivityFrame, 7 as COUNT);
    } else {
        s.frame = ::core::ptr::null_mut::<FRAME_DESC>();
    }
    if !s.frame.is_null() {
        let mut r: RECT = RECT {
            corner: POINT { x: 0, y: 0 },
            extent: EXTENT { width: 0, height: 0 },
        };
        GetFrameRect(s.frame, &raw mut r);
        s.origin.x = (CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
                } << resolutionFactor)) - r.extent.width as ::core::ffi::c_int
            - ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        uqm_lua_checked!(DrawStamp(&raw mut s), ());
    }
    UnbatchGraphics();
}
unsafe extern "C" fn FeedbackPlayerPhrase(mut pStr: *mut UNICODE) {
    SetContext(SpaceContext);
    BatchGraphics();
    uqm_lua_checked!(DrawSISComWindow(), ());
    if *pStr.offset(0isize) != 0 {
        let mut ct: TEXT = TEXT {
            baseline: POINT { x: 0, y: 0 },
            pStr: ::core::ptr::null::<UNICODE>(),
            align: TEXT_ALIGN::ALIGN_LEFT,
            CharCount: 0,
        };
        ct.baseline.x = ((CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
                } << resolutionFactor)) >> resolutionFactor >> 1 as ::core::ffi::c_int)
            << resolutionFactor) as COORD;
        ct.baseline.y = (((107 as ::core::ffi::c_int) << resolutionFactor)
            + (if usingSpeech.0 == 0 && isPC(optSmoothScroll).0 != 0 {
                4 as ::core::ffi::c_int
            } else if if optWindowType == 0 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                4 as ::core::ffi::c_int
            } else {
                15 as ::core::ffi::c_int
            } << resolutionFactor) + ((13 as ::core::ffi::c_int) << resolutionFactor))
            as COORD;
        ct.align = TEXT_ALIGN::ALIGN_CENTER;
        ct.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
        if IsDarkMode.0 == 0 {
            ct.pStr = GetStringAddress(
                SetAbsStringTableIndex(
                    GameStrings,
                    C2Rust_Unnamed_4::FEEDBACK_STRING_BASE.0 as ::core::ffi::c_int
                        as COUNT,
                ),
            );
            SetContextForeGroundColor(
                colorFromRgb15(0xa as BYTE, 0xc as BYTE, 0x1f as BYTE),
            );
            uqm_lua_checked!(font_DrawText(&raw mut ct), ());
            SetContextForeGroundColor(
                colorFromRgb15(0x12 as BYTE, 0x14 as BYTE, 0x4f as BYTE),
            );
        } else {
            SetContextForeGroundColor(
                buildColorRgba(0x55 as BYTE, 0x55 as BYTE, 0xff as BYTE, 0xff as BYTE),
            );
        }
        ct.baseline.y = (ct.baseline.y as ::core::ffi::c_int
            + ((16 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        ct.pStr = pStr;
        uqm_lua_checked!(add_text(-4 as ::core::ffi::c_int, &raw mut ct), ());
    }
    UnbatchGraphics();
}
unsafe extern "C" fn InitSpeechGraphics() {
    if optScopeStyle != 0x2 as ::core::ffi::c_int {
        InitOscilloscope(SetAbsFrameIndex(ActivityFrame, 9 as COUNT));
    } else {
        InitOscilloscope(SetAbsFrameIndex(ActivityFrame, 10 as COUNT));
    }
    InitSlider(
        0 as ::core::ffi::c_int,
        (107 as ::core::ffi::c_int) << resolutionFactor,
        CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
                } << resolutionFactor)),
        SetAbsFrameIndex(ActivityFrame, 5 as COUNT),
        SetAbsFrameIndex(ActivityFrame, 2 as COUNT),
    );
}
unsafe extern "C" fn UpdateSpeechGraphics() {
    static mut NextTime: TimeCount = 0;
    let mut OldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    if GetTimeCounter() < NextTime {
        return;
    }
    NextTime = GetTimeCounter()
        .wrapping_add(
            (840 as ::core::ffi::c_int
                / if if resolutionFactor != 2 as ::core::ffi::c_uint {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } == 0
                {
                    32 as ::core::ffi::c_int
                } else {
                    60 as ::core::ffi::c_int
                }) as TimeCount,
        );
    OldContext = SetContext(RadarContext);
    uqm_lua_checked!(DrawOscilloscope(), ());
    SetContext(SpaceContext);
    uqm_lua_checked!(DrawSlider(), ());
    SetContext(OldContext);
}
unsafe extern "C" fn UpdateAnimations(mut paused: BOOLEAN) {
    static mut NextTime: TimeCount = 0;
    let mut OldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    let mut change: BOOLEAN = BOOLEAN::FALSE;
    if GetTimeCounter() < NextTime {
        return;
    }
    NextTime = GetTimeCounter()
        .wrapping_add(
            (840 as ::core::ffi::c_int / 40 as ::core::ffi::c_int) as TimeCount,
        );
    OldContext = SetContext(AnimContext);
    BatchGraphics();
    change = uqm_lua_checked!(ProcessCommAnimations(clear_subtitles, paused), ());
    if change.0 != 0 || clear_subtitles.0 != 0 {
        uqm_lua_checked!(RedrawSubtitles(), ());
    }
    UnbatchGraphics();
    clear_subtitles = BOOLEAN::FALSE;
    SetContext(OldContext);
}
#[no_mangle]
pub unsafe extern "C" fn UpdateDuty(mut talk: BOOLEAN) {
    if talk.0 != 0 {
        if TalkingFinished.0 == 0 {
            setRunTalkingAnim();
        }
        CheckSubtitles(BOOLEAN::TRUE);
        uqm_lua_checked!(UpdateAnimations(BOOLEAN::FALSE), ());
    }
    uqm_lua_checked!(UpdateSpeechGraphics(), ());
}
#[no_mangle]
pub static mut DarkModeRect: [RECT; 6] = [RECT {
    corner: POINT { x: 0, y: 0 },
    extent: EXTENT { width: 0, height: 0 },
}; 6];
#[no_mangle]
pub unsafe extern "C" fn InitUIRects(mut state: BOOLEAN) {
    if state.0 != 0 {
        DarkModeRect[0usize].corner = MAKE_POINT(0 as COORD, 0 as COORD);
        DarkModeRect[0usize].extent = MAKE_EXTENT(
            (((6 as ::core::ffi::c_int) << resolutionFactor)
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
                }
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
                        } << resolutionFactor)))) as COORD,
            (((9 as ::core::ffi::c_int) << resolutionFactor)
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
                }) as COORD,
        );
        DarkModeRect[1usize].corner = MAKE_POINT(
            0 as COORD,
            (((9 as ::core::ffi::c_int) << resolutionFactor)
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
                }) as COORD,
        );
        DarkModeRect[1usize].extent = MAKE_EXTENT(
            (((6 as ::core::ffi::c_int) << resolutionFactor)
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
                }) as COORD,
            (CanvasHeight
                - (((9 as ::core::ffi::c_int) << resolutionFactor)
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
                    })) as COORD,
        );
        DarkModeRect[2usize].corner = MAKE_POINT(
            (((6 as ::core::ffi::c_int) << resolutionFactor)
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
                }
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
                        } << resolutionFactor)))) as COORD,
            0 as COORD,
        );
        DarkModeRect[2usize].extent = MAKE_EXTENT(
            (CanvasWidth - DarkModeRect[2usize].corner.x as ::core::ffi::c_int) as COORD,
            CanvasHeight as COORD,
        );
        DarkModeRect[3usize].corner = MAKE_POINT(
            (((6 as ::core::ffi::c_int) << resolutionFactor)
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
                }) as COORD,
            (((9 as ::core::ffi::c_int) << resolutionFactor)
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
                }
                + (CanvasHeight
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
                    - ((13 as ::core::ffi::c_int) << resolutionFactor))) as COORD,
        );
        DarkModeRect[3usize].extent = MAKE_EXTENT(
            (CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
                    } << resolutionFactor))) as COORD,
            (CanvasHeight
                - (((9 as ::core::ffi::c_int) << resolutionFactor)
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
                    })
                + (CanvasHeight
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
                    - ((13 as ::core::ffi::c_int) << resolutionFactor))) as COORD,
        );
        DarkModeRect[4usize].corner = MAKE_POINT(
            (((6 as ::core::ffi::c_int) << resolutionFactor)
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
                }) as COORD,
            (((9 as ::core::ffi::c_int) << resolutionFactor)
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
                } + ((107 as ::core::ffi::c_int) << resolutionFactor)) as COORD,
        );
        DarkModeRect[4usize].extent = MAKE_EXTENT(
            (CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
                    } << resolutionFactor))) as COORD,
            (if usingSpeech.0 == 0 && isPC(optSmoothScroll).0 != 0 {
                4 as ::core::ffi::c_int
            } else if if optWindowType == 0 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                4 as ::core::ffi::c_int
            } else {
                15 as ::core::ffi::c_int
            } << resolutionFactor) as COORD,
        );
        DarkModeRect[5usize].corner = MAKE_POINT(
            (((6 as ::core::ffi::c_int) << resolutionFactor)
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
                }) as COORD,
            (((9 as ::core::ffi::c_int) << resolutionFactor)
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
                } + ((107 as ::core::ffi::c_int) << resolutionFactor)
                + (if usingSpeech.0 == 0 && isPC(optSmoothScroll).0 != 0 {
                    4 as ::core::ffi::c_int
                } else if if optWindowType == 0 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    4 as ::core::ffi::c_int
                } else {
                    15 as ::core::ffi::c_int
                } << resolutionFactor)) as COORD,
        );
        DarkModeRect[5usize].extent = MAKE_EXTENT(
            (CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
                    } << resolutionFactor))) as COORD,
            (CanvasHeight
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
                - (((107 as ::core::ffi::c_int) << resolutionFactor)
                    + (if usingSpeech.0 == 0 && isPC(optSmoothScroll).0 != 0 {
                        4 as ::core::ffi::c_int
                    } else if if optWindowType == 0 as DWORD {
                        BOOLEAN::TRUE.0 as ::core::ffi::c_int
                    } else {
                        BOOLEAN::FALSE.0 as ::core::ffi::c_int
                    } != 0
                    {
                        4 as ::core::ffi::c_int
                    } else {
                        15 as ::core::ffi::c_int
                    } << resolutionFactor))) as COORD,
        );
    } else {
        memset(
            &raw mut DarkModeRect as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<[RECT; 6]>() as size_t,
        );
    };
}
unsafe extern "C" fn FadePlayerUI() {
    if IsDarkMode.0 == 0 {
        return
    } else {
        static mut NextTime: TimeCount = 0;
        let mut OldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
        let mut i: BYTE = 0;
        static mut DarkModeFGTextColor: [Color; 16] = [
            Color {
                r: 0 as BYTE,
                g: 0xff as BYTE,
                b: 0x10 as BYTE,
                a: 0 as BYTE,
            },
            Color {
                r: 0 as BYTE,
                g: 0xff as BYTE,
                b: 0x20 as BYTE,
                a: 0 as BYTE,
            },
            Color {
                r: 0 as BYTE,
                g: 0xff as BYTE,
                b: 0x30 as BYTE,
                a: 0 as BYTE,
            },
            Color {
                r: 0 as BYTE,
                g: 0xff as BYTE,
                b: 0x40 as BYTE,
                a: 0 as BYTE,
            },
            Color {
                r: 0 as BYTE,
                g: 0xff as BYTE,
                b: 0x50 as BYTE,
                a: 0 as BYTE,
            },
            Color {
                r: 0 as BYTE,
                g: 0xff as BYTE,
                b: 0x60 as BYTE,
                a: 0 as BYTE,
            },
            Color {
                r: 0x8 as BYTE,
                g: 0xff as BYTE,
                b: 0x70 as BYTE,
                a: 0 as BYTE,
            },
            Color {
                r: 0x10 as BYTE,
                g: 0xff as BYTE,
                b: 0x80 as BYTE,
                a: 0 as BYTE,
            },
            Color {
                r: 0x18 as BYTE,
                g: 0xff as BYTE,
                b: 0x90 as BYTE,
                a: 0 as BYTE,
            },
            Color {
                r: 0x20 as BYTE,
                g: 0xff as BYTE,
                b: 0xa0 as BYTE,
                a: 0 as BYTE,
            },
            Color {
                r: 0x28 as BYTE,
                g: 0xff as BYTE,
                b: 0xb0 as BYTE,
                a: 0 as BYTE,
            },
            Color {
                r: 0x30 as BYTE,
                g: 0xff as BYTE,
                b: 0xc0 as BYTE,
                a: 0 as BYTE,
            },
            Color {
                r: 0x38 as BYTE,
                g: 0xff as BYTE,
                b: 0xd0 as BYTE,
                a: 0 as BYTE,
            },
            Color {
                r: 0x40 as BYTE,
                g: 0xff as BYTE,
                b: 0xe0 as BYTE,
                a: 0 as BYTE,
            },
            Color {
                r: 0x48 as BYTE,
                g: 0xff as BYTE,
                b: 0xf0 as BYTE,
                a: 0 as BYTE,
            },
            Color {
                r: 0x55 as BYTE,
                g: 0xff as BYTE,
                b: 0xff as BYTE,
                a: 0 as BYTE,
            },
        ];
        static mut DarkModeBGTextColor: [Color; 16] = [
            Color {
                r: 0xe0 as BYTE,
                g: 0x9a as BYTE,
                b: 0x14 as BYTE,
                a: 0 as BYTE,
            },
            Color {
                r: 0xd2 as BYTE,
                g: 0x87 as BYTE,
                b: 0x14 as BYTE,
                a: 0 as BYTE,
            },
            Color {
                r: 0xc4 as BYTE,
                g: 0x7e as BYTE,
                b: 0x14 as BYTE,
                a: 0 as BYTE,
            },
            Color {
                r: 0xb6 as BYTE,
                g: 0x75 as BYTE,
                b: 0x14 as BYTE,
                a: 0 as BYTE,
            },
            Color {
                r: 0xa8 as BYTE,
                g: 0x6c as BYTE,
                b: 0x14 as BYTE,
                a: 0 as BYTE,
            },
            Color {
                r: 0x9a as BYTE,
                g: 0x63 as BYTE,
                b: 0x14 as BYTE,
                a: 0 as BYTE,
            },
            Color {
                r: 0x8c as BYTE,
                g: 0x5a as BYTE,
                b: 0x14 as BYTE,
                a: 0 as BYTE,
            },
            Color {
                r: 0x7e as BYTE,
                g: 0x51 as BYTE,
                b: 0x14 as BYTE,
                a: 0 as BYTE,
            },
            Color {
                r: 0x70 as BYTE,
                g: 0x48 as BYTE,
                b: 0x7 as BYTE,
                a: 0 as BYTE,
            },
            Color {
                r: 0x62 as BYTE,
                g: 0x3f as BYTE,
                b: 0x7 as BYTE,
                a: 0 as BYTE,
            },
            Color {
                r: 0x54 as BYTE,
                g: 0x36 as BYTE,
                b: 0x7 as BYTE,
                a: 0 as BYTE,
            },
            Color {
                r: 0x46 as BYTE,
                g: 0x2d as BYTE,
                b: 0 as BYTE,
                a: 0 as BYTE,
            },
            Color {
                r: 0x38 as BYTE,
                g: 0x24 as BYTE,
                b: 0 as BYTE,
                a: 0 as BYTE,
            },
            Color {
                r: 0x2a as BYTE,
                g: 0x1b as BYTE,
                b: 0 as BYTE,
                a: 0 as BYTE,
            },
            Color {
                r: 0x1c as BYTE,
                g: 0x12 as BYTE,
                b: 0 as BYTE,
                a: 0 as BYTE,
            },
            Color {
                r: 0 as BYTE,
                g: 0 as BYTE,
                b: 0 as BYTE,
                a: 0 as BYTE,
            },
        ];
        if fadeIndex as ::core::ffi::c_int > 50 as ::core::ffi::c_int {
            cwLock = BOOLEAN::FALSE;
            return;
        }
        if GetTimeCounter() < NextTime {
            return;
        }
        NextTime = GetTimeCounter()
            .wrapping_add(
                (840 as ::core::ffi::c_int / 15 as ::core::ffi::c_int) as TimeCount,
            );
        OldContext = SetContext(ScreenContext);
        SetContextForeGroundColor(
            buildColorRgba(
                0 as BYTE,
                0 as BYTE,
                0 as BYTE,
                (0xff as ::core::ffi::c_int as ::core::ffi::c_double * 0.02f64
                    * fadeIndex as ::core::ffi::c_int as ::core::ffi::c_double) as BYTE,
            ),
        );
        BatchGraphics();
        i = 0 as BYTE;
        while (i as usize)
            < ::core::mem::size_of::<[RECT; 6]>()
                .wrapping_div(::core::mem::size_of::<RECT>())
                .wrapping_sub(1usize)
        {
            uqm_lua_checked!(DrawFilledRectangle((&raw mut DarkModeRect as *mut RECT).offset(i as isize)), ());
            i = i.wrapping_add(1);
        }
        if (fadeIndex as ::core::ffi::c_int) < 16 as ::core::ffi::c_int {
            ClearSubtitles();
            CommData.AlienTextFColor = DarkModeFGTextColor[fadeIndex as usize];
            CommData.AlienTextBColor = DarkModeBGTextColor[fadeIndex as usize];
            uqm_lua_checked!(RedrawSubtitles(), ());
        }
        if cwLock.0 != 0 {
            uqm_lua_checked!(DrawFilledRectangle((&raw mut DarkModeRect as *mut RECT).offset(5isize)), ());
        }
        UnbatchGraphics();
        SetContext(OldContext);
        fadeIndex = fadeIndex.wrapping_add(1);
    };
}
unsafe extern "C" fn UpdateCommGraphics() {
    uqm_lua_checked!(UpdateAnimations(BOOLEAN::FALSE), ());
    uqm_lua_checked!(UpdateSpeechGraphics(), ());
    uqm_lua_checked!(FadePlayerUI(), ());
}
unsafe extern "C" fn DoTalkSegue(mut pTS: *mut TALKING_STATE) -> BOOLEAN {
    let mut left: bool = false;
    let mut right: bool = false;
    let mut curTrack: COUNT = 0;
    if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
        & C2Rust_Unnamed_1::CHECK_ABORT.0 as ::core::ffi::c_int != 0
    {
        (*pTS).ended = true;
        return BOOLEAN::FALSE;
    }
    if optSpeech.0 != 0 || optSmoothScroll == 0x1 as ::core::ffi::c_int
        || GlobData.Game_state.CurrentActivity as BYTE as ::core::ffi::c_int
            == C2Rust_Unnamed_1::WON_LAST_BATTLE.0 as ::core::ffi::c_int
    {
        if PulsedInputState
            .menu[C2Rust_Unnamed::KEY_MENU_CANCEL.0 as ::core::ffi::c_int as usize] != 0
        {
            uqm_lua_checked!(JumpTrack(), BOOLEAN(0));
            (*pTS).ended = true;
            return BOOLEAN::FALSE;
        }
        if optSmoothScroll == 0x2 as ::core::ffi::c_int || optSpeech.0 == 0 {
            left = PulsedInputState
                .menu[C2Rust_Unnamed::KEY_MENU_LEFT.0 as ::core::ffi::c_int as usize]
                != 0 as ::core::ffi::c_int;
            right = PulsedInputState
                .menu[C2Rust_Unnamed::KEY_MENU_RIGHT.0 as ::core::ffi::c_int as usize]
                != 0 as ::core::ffi::c_int;
        } else if optSmoothScroll == 0x1 as ::core::ffi::c_int {
            left = CurrentInputState
                .menu[C2Rust_Unnamed::KEY_MENU_LEFT.0 as ::core::ffi::c_int as usize]
                != 0 as ::core::ffi::c_int;
            right = CurrentInputState
                .menu[C2Rust_Unnamed::KEY_MENU_RIGHT.0 as ::core::ffi::c_int as usize]
                != 0 as ::core::ffi::c_int;
        }
        curTrack = PlayingTrack();
        if right {
            SetSliderImage(SetAbsFrameIndex(ActivityFrame, 3 as COUNT));
            if optSmoothScroll == 0x2 as ::core::ffi::c_int || optSpeech.0 == 0 {
                uqm_lua_checked!(FastForward_Page(), BOOLEAN(0));
            } else if optSmoothScroll == 0x1 as ::core::ffi::c_int {
                uqm_lua_checked!(FastForward_Smooth(), BOOLEAN(0));
            }
            (*pTS).seeking = true;
        } else if left as ::core::ffi::c_int != 0
            || (*pTS).rewind as ::core::ffi::c_int != 0
        {
            (*pTS).rewind = false;
            SetSliderImage(SetAbsFrameIndex(ActivityFrame, 4 as COUNT));
            if optSmoothScroll == 0x2 as ::core::ffi::c_int || optSpeech.0 == 0 {
                uqm_lua_checked!(FastReverse_Page(), BOOLEAN(0));
            } else if optSmoothScroll == 0x1 as ::core::ffi::c_int {
                uqm_lua_checked!(FastReverse_Smooth(), BOOLEAN(0));
            }
            (*pTS).seeking = true;
        } else if (*pTS).seeking {
            (*pTS).seeking = false;
            SetSliderImage(SetAbsFrameIndex(ActivityFrame, 2 as COUNT));
        } else {
            CheckSubtitles(
                BOOLEAN(
                    (curTrack as ::core::ffi::c_int != 0
                        && curTrack as ::core::ffi::c_int
                            <= (*pTS).waitTrack as ::core::ffi::c_int)
                        as ::core::ffi::c_int as ::core::ffi::c_uint,
                ),
            );
        }
    } else {
        curTrack = PlayingTrack();
        if PulsedInputState
            .menu[C2Rust_Unnamed::KEY_MENU_CANCEL.0 as ::core::ffi::c_int as usize] != 0
            || PulsedInputState
                .menu[C2Rust_Unnamed::KEY_MENU_RIGHT.0 as ::core::ffi::c_int as usize]
                != 0
        {
            uqm_lua_checked!(FastForward_Page(), BOOLEAN(0));
            FlushInput();
            next_page = BOOLEAN::TRUE;
            return BOOLEAN::TRUE;
        } else if PulsedInputState
            .menu[C2Rust_Unnamed::KEY_MENU_SPECIAL.0 as ::core::ffi::c_int as usize] != 0
        {
            uqm_lua_checked!(JumpTrack(), BOOLEAN(0));
            (*pTS).ended = true;
            return BOOLEAN::FALSE;
        } else if PauseSubtitles(next_page).0 != 0 {
            let mut delay: DWORD = RecalculateDelay(
                strlen(SubtitleText.pStr) as DWORD,
                BOOLEAN::FALSE,
            );
            let mut read_speed: BYTE = speed_array[(GlobData.Game_state.glob_flags
                as ::core::ffi::c_int
                & ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int) as usize];
            if delay > 0 as DWORD
                || read_speed as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            {
                let mut awake: BOOLEAN = BOOLEAN::FALSE;
                let mut block: BOOLEAN = BOOLEAN(
                    (wantTalkingAnim().0 == 0 || IsDarkMode.0 != 0
                        || haveTalkingAnim().0 == 0) as ::core::ffi::c_int
                        as ::core::ffi::c_uint,
                );
                let mut TimeOut: TimeCount = 0;
                uqm_lua_checked!(PauseTrack(), BOOLEAN(0));
                if block.0 == 0 {
                    freezeTalkingAnim();
                }
                TimeOut = GetTimeCounter().wrapping_add(delay) as TimeCount;
                while awake.0 == 0 {
                    if read_speed as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                        && GetTimeCounter() >= TimeOut
                    {
                        awake = BOOLEAN::TRUE;
                    }
                    uqm_lua_checked!(UpdateCommGraphics(), BOOLEAN(0));
                    uqm_lua_checked!(UpdateInputState(), BOOLEAN(0));
                    if PulsedInputState
                        .menu[C2Rust_Unnamed::KEY_MENU_CANCEL.0 as ::core::ffi::c_int
                        as usize] != 0
                        || PulsedInputState
                            .menu[C2Rust_Unnamed::KEY_MENU_RIGHT.0 as ::core::ffi::c_int
                            as usize] != 0
                        || GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
                            & C2Rust_Unnamed_1::CHECK_ABORT.0 as ::core::ffi::c_int != 0
                    {
                        awake = BOOLEAN::TRUE;
                    } else if PulsedInputState
                        .menu[C2Rust_Unnamed::KEY_MENU_SPECIAL.0 as ::core::ffi::c_int
                        as usize] != 0
                    {
                        uqm_lua_checked!(JumpTrack(), BOOLEAN(0));
                        (*pTS).ended = true;
                        if block.0 == 0 {
                            unFreezeTalkingAnim();
                        }
                        return BOOLEAN::FALSE;
                    }
                }
                if block.0 == 0 {
                    unFreezeTalkingAnim();
                }
                uqm_lua_checked!(ResumeTrack(), BOOLEAN(0));
            }
        }
        next_page = BOOLEAN::FALSE;
        CheckSubtitles(
            BOOLEAN(
                (curTrack as ::core::ffi::c_int != 0
                    && curTrack as ::core::ffi::c_int
                        <= (*pTS).waitTrack as ::core::ffi::c_int) as ::core::ffi::c_int
                    as ::core::ffi::c_uint,
            ),
        );
    }
    uqm_lua_checked!(UpdateAnimations(BOOLEAN((*pTS).seeking as ::core::ffi::c_uint)), BOOLEAN(0));
    uqm_lua_checked!(UpdateSpeechGraphics(), BOOLEAN(0));
    uqm_lua_checked!(FadePlayerUI(), BOOLEAN(0));
    (*pTS).ended = !(*pTS).seeking && curTrack == 0;
    uqm_lua_checked!(SleepThreadUntil((*pTS).NextTime), BOOLEAN(0));
    (*pTS).NextTime = GetTimeCounter()
        .wrapping_add(
            (840 as ::core::ffi::c_int / 60 as ::core::ffi::c_int) as TimeCount,
        );
    return BOOLEAN(
        ((*pTS).seeking as ::core::ffi::c_int != 0
            || curTrack as ::core::ffi::c_int != 0
                && curTrack as ::core::ffi::c_int
                    <= (*pTS).waitTrack as ::core::ffi::c_int) as ::core::ffi::c_int
            as ::core::ffi::c_uint,
    );
}
unsafe extern "C" fn runCommAnimFrame() {
    uqm_lua_checked!(UpdateCommGraphics(), ());
    uqm_lua_checked!(SleepThread((840 as ::core::ffi::c_int / 40 as ::core::ffi::c_int) as TimePeriod), ());
}
unsafe extern "C" fn TalkSegue(mut wait_track: COUNT) -> BOOLEAN {
    let mut talkingState: TALKING_STATE = TALKING_STATE {
        InputFunc: None,
        NextTime: 0,
        waitTrack: 0,
        rewind: false,
        seeking: false,
        ended: false,
    };
    if wantTalkingAnim().0 != 0 && haveTalkingAnim().0 != 0 {
        if haveTransitionAnim().0 != 0 {
            setRunIntroAnim();
        }
        setRunTalkingAnim();
        while runningIntroAnim().0 != 0 {
            uqm_lua_checked!(runCommAnimFrame(), BOOLEAN(0));
        }
    }
    memset(
        &raw mut talkingState as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<TALKING_STATE>() as size_t,
    );
    if wait_track as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        wait_track = !(0 as ::core::ffi::c_int) as COUNT;
        talkingState.rewind = true;
    } else if PlayingTrack() == 0 {
        uqm_lua_checked!(PlayTrack(), BOOLEAN(0));
    }
    SetMenuSounds(
        0 as ::core::ffi::c_int as MENU_SOUND_FLAGS,
        0 as ::core::ffi::c_int as MENU_SOUND_FLAGS,
    );
    talkingState.InputFunc = Some(
        DoTalkSegue as unsafe extern "C" fn(*mut TALKING_STATE) -> BOOLEAN,
    ) as Option<unsafe extern "C" fn(*mut talking_state) -> BOOLEAN>;
    talkingState.waitTrack = wait_track;
    uqm_lua_checked!(DoInput(&raw mut talkingState as *mut ::core::ffi::c_void, BOOLEAN::FALSE), BOOLEAN(0));
    if talkingState.ended {
        ClearSubtitles();
        SetSliderImage(SetAbsFrameIndex(ActivityFrame, 8 as COUNT));
        cwLock = BOOLEAN::FALSE;
    }
    if runningTalkingAnim().0 != 0 {
        setStopTalkingAnim();
    }
    if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
        & C2Rust_Unnamed_1::CHECK_ABORT.0 as ::core::ffi::c_int == 0
    {
        while runningTalkingAnim().0 != 0 {
            uqm_lua_checked!(runCommAnimFrame(), BOOLEAN(0));
        }
    }
    return BOOLEAN(talkingState.ended as ::core::ffi::c_uint);
}
unsafe extern "C" fn CommIntroTransition() {
    if curIntroMode.0 == CommIntroMode::CIM_CROSSFADE_SCREEN.0 {
        uqm_lua_checked!(ScreenTransition(optScrTrans, ::core::ptr::null::<RECT>()), ());
        UnbatchGraphics();
    } else if curIntroMode.0 == CommIntroMode::CIM_CROSSFADE_SPACE.0 {
        let mut r: RECT = RECT {
            corner: POINT { x: 0, y: 0 },
            extent: EXTENT { width: 0, height: 0 },
        };
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
            }) as COORD;
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
                } << resolutionFactor))) as COORD;
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
            - ((13 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        uqm_lua_checked!(ScreenTransition(optScrTrans, &raw mut r), ());
        UnbatchGraphics();
    } else if curIntroMode.0 == CommIntroMode::CIM_CROSSFADE_WINDOW.0 {
        uqm_lua_checked!(ScreenTransition(optScrTrans, &raw mut CommWndRect), ());
        UnbatchGraphics();
    } else if curIntroMode.0 == CommIntroMode::CIM_FADE_IN_SCREEN.0 {
        UnbatchGraphics();
        FadeScreen(ScreenFadeType::FadeAllToColor, fadeTime as SIZE);
    } else {
        UnbatchGraphics();
        (false
            && !(b"Unknown comm intro transition\0".as_ptr()
                as *const ::core::ffi::c_char)
                .is_null()
            || {
                __assert_fail(
                    b"0 && \"Unknown comm intro transition\"\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"src/uqm/comm.c\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    1162 as ::core::ffi::c_int,
                    b"CommIntroTransition\0".as_ptr() as *const ::core::ffi::c_char,
                );
                0 as ::core::ffi::c_int != 0
            }) as ::core::ffi::c_int;
    }
    curIntroMode = CommIntroMode::CIM_DEFAULT;
}
#[no_mangle]
pub unsafe extern "C" fn AlienTalkSegue(mut wait_track: COUNT) {
    if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
        & C2Rust_Unnamed_1::CHECK_ABORT.0 as ::core::ffi::c_int != 0
        || TalkingFinished.0 != 0
    {
        return;
    }
    if (*pCurInputState).Initialized == 0 {
        let mut AlienSong: MUSIC_REF = CommData.AlienSong;
        InitSpeechGraphics();
        SetColorMap(GetColorMapAddress(CommData.AlienColorMap));
        SetContext(AnimContext);
        uqm_lua_checked!(DrawAlienFrame(::core::ptr::null_mut::<SEQUENCE>(), 0 as COUNT, BOOLEAN::TRUE), ());
        uqm_lua_checked!(UpdateSpeechGraphics(), ());
        uqm_lua_checked!(CommIntroTransition(), ());
        (*pCurInputState).Initialized = BOOLEAN::TRUE.0 as ::core::ffi::c_int as COUNT;
        uqm_lua_checked!(PlayMusicResume(
            AlienSong,
            (if usingSpeech.0 != 0 && VolasPackPresent.0 == 0 {
                160 as ::core::ffi::c_int / 2 as ::core::ffi::c_int
            } else {
                160 as ::core::ffi::c_int
            }) as BYTE,
        ), ());
        InitCommAnimations();
        if optScrTrans != 0 && optSpeech.0 == 0 {
            let mut timeout: TimeCount = GetTimeCounter()
                .wrapping_add(
                    (840 as ::core::ffi::c_int / 4 as ::core::ffi::c_int) as TimeCount,
                );
            while GetTimeCounter() < timeout {
                uqm_lua_checked!(UpdateDuty(BOOLEAN::FALSE), ());
                uqm_lua_checked!(UpdateAnimations(BOOLEAN::FALSE), ());
            }
        }
        LastActivity = (LastActivity as ::core::ffi::c_int
            & !(C2Rust_Unnamed_1::CHECK_LOAD.0 as ::core::ffi::c_int)) as ACTIVITY;
    }
    TalkingFinished = uqm_lua_checked!(TalkSegue(wait_track), ());
    if TalkingFinished.0 != 0 && VolasPackPresent.0 == 0 {
        uqm_lua_checked!(FadeMusic(160 as BYTE, 840 as SIZE), ());
    }
}
unsafe extern "C" fn remove_char_from_string(mut str: *mut UNICODE, c: UNICODE) {
    let mut pr: *mut UNICODE = str;
    let mut pw: *mut UNICODE = str;
    while *pr != 0 {
        let c2rust_fresh0 = pr;
        pr = pr.offset(1);
        *pw = *c2rust_fresh0;
        pw = pw
            .offset(
                (*pw as ::core::ffi::c_int != c as ::core::ffi::c_int)
                    as ::core::ffi::c_int as isize,
            );
    }
    *pw = '\0' as UNICODE;
}
unsafe extern "C" fn DoConvSummary(mut pSS: *mut SUMMARY_STATE) -> BOOLEAN {
    if (*pSS).Initialized.0 == 0 {
        (*pSS).PrintNext = BOOLEAN::TRUE;
        (*pSS).NextSub = GetFirstTrackSubtitle();
        (*pSS).LeftOver = ::core::ptr::null::<UNICODE>();
        (*pSS).InputFunc = Some(
            DoConvSummary as unsafe extern "C" fn(*mut SUMMARY_STATE) -> BOOLEAN,
        ) as Option<unsafe extern "C" fn(*mut summary_state) -> BOOLEAN>;
        (*pSS).Initialized = BOOLEAN::TRUE;
        uqm_lua_checked!(DoInput(pSS as *mut ::core::ffi::c_void, BOOLEAN::FALSE), BOOLEAN(0));
    } else if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
        & C2Rust_Unnamed_1::CHECK_ABORT.0 as ::core::ffi::c_int != 0
    {
        return BOOLEAN::FALSE
    } else if PulsedInputState
        .menu[C2Rust_Unnamed::KEY_MENU_SELECT.0 as ::core::ffi::c_int as usize] != 0
        || PulsedInputState
            .menu[C2Rust_Unnamed::KEY_MENU_CANCEL.0 as ::core::ffi::c_int as usize] != 0
        || PulsedInputState
            .menu[C2Rust_Unnamed::KEY_MENU_RIGHT.0 as ::core::ffi::c_int as usize] != 0
    {
        if !(*pSS).NextSub.is_null() {
            (*pSS).PrintNext = BOOLEAN::TRUE;
        } else {
            return BOOLEAN::FALSE
        }
    } else if (*pSS).PrintNext.0 != 0 {
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
        let mut row: ::core::ffi::c_int = 0;
        r.corner.x = 0 as COORD;
        r.corner.y = 0 as COORD;
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
                } << resolutionFactor))) as COORD;
        r.extent.height = ((107 as ::core::ffi::c_int) << resolutionFactor) as COORD;
        SetContext(AnimContext);
        SetContextForeGroundColor(colorFromRgb15(0 as BYTE, 0x5 as BYTE, 0 as BYTE));
        uqm_lua_checked!(DrawFilledRectangle(&raw mut r), BOOLEAN(0));
        SetContextForeGroundColor(colorFromRgb15(0 as BYTE, 0x10 as BYTE, 0 as BYTE));
        r.extent.width = (r.extent.width as ::core::ffi::c_int
            - ((2 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) << resolutionFactor))
            as COORD;
        t.baseline.x = ((2 as ::core::ffi::c_int) << resolutionFactor) as COORD;
        t.align = TEXT_ALIGN::ALIGN_LEFT;
        t.baseline.y = ((8 as ::core::ffi::c_int) << resolutionFactor) as COORD;
        SetContextFont(TinyFont);
        row = 0 as ::core::ffi::c_int;
        's_238: while row
            < ((107 as ::core::ffi::c_int) << resolutionFactor)
                / ((8 as ::core::ffi::c_int) << resolutionFactor)
                - 1 as ::core::ffi::c_int && !(*pSS).NextSub.is_null()
        {
            let mut next: *const ::core::ffi::c_char = ::core::ptr::null::<
                ::core::ffi::c_char,
            >();
            's_111: {
                if !(*pSS).LeftOver.is_null() {
                    t.pStr = (*pSS).LeftOver;
                    (*pSS).LeftOver = ::core::ptr::null::<UNICODE>();
                } else {
                    t.pStr = GetTrackSubtitleText((*pSS).NextSub);
                    if t.pStr.is_null() {
                        break 's_111;
                    }
                }
                t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
                while row
                    < ((107 as ::core::ffi::c_int) << resolutionFactor)
                        / ((8 as ::core::ffi::c_int) << resolutionFactor)
                        - 1 as ::core::ffi::c_int
                    && getLineWithinWidth(
                            &raw mut t,
                            &raw mut next,
                            r.extent.width,
                            !(0 as ::core::ffi::c_int) as COUNT,
                        )
                        .0 == 0
                {
                    if CommData.AlienConv.0 == CONVERSATION::ORZ_CONVERSATION.0 {
                        let mut my_copy: [UNICODE; 128] = [0; 128];
                        strcpy(&raw mut my_copy as *mut ::core::ffi::c_char, t.pStr);
                        remove_char_from_string(
                            &raw mut my_copy as *mut UNICODE,
                            '$' as UNICODE,
                        );
                        t.pStr = &raw mut my_copy as *mut UNICODE;
                        uqm_lua_checked!(font_DrawText(&raw mut t), BOOLEAN(0));
                    } else {
                        uqm_lua_checked!(font_DrawText(&raw mut t), BOOLEAN(0));
                    }
                    t.baseline.y = (t.baseline.y as ::core::ffi::c_int
                        + ((8 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
                    t.pStr = next as *const UNICODE;
                    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
                    row += 1;
                }
                if row
                    >= ((107 as ::core::ffi::c_int) << resolutionFactor)
                        / ((8 as ::core::ffi::c_int) << resolutionFactor)
                        - 1 as ::core::ffi::c_int
                {
                    (*pSS).LeftOver = next as *const UNICODE;
                    break 's_238;
                } else {
                    if CommData.AlienConv.0 == CONVERSATION::ORZ_CONVERSATION.0 {
                        let mut my_copy_0: [UNICODE; 128] = [0; 128];
                        strcpy(&raw mut my_copy_0 as *mut ::core::ffi::c_char, t.pStr);
                        remove_char_from_string(
                            &raw mut my_copy_0 as *mut UNICODE,
                            '$' as UNICODE,
                        );
                        t.pStr = &raw mut my_copy_0 as *mut UNICODE;
                        uqm_lua_checked!(font_DrawText(&raw mut t), BOOLEAN(0));
                    } else {
                        uqm_lua_checked!(font_DrawText(&raw mut t), BOOLEAN(0));
                    }
                    t.baseline.y = (t.baseline.y as ::core::ffi::c_int
                        + ((8 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
                }
            }
            row += 1;
            (*pSS).NextSub = GetNextTrackSubtitle((*pSS).NextSub);
        }
        if row
            >= ((107 as ::core::ffi::c_int) << resolutionFactor)
                / ((8 as ::core::ffi::c_int) << resolutionFactor)
                - 1 as ::core::ffi::c_int
            && (!(*pSS).NextSub.is_null() || !(*pSS).LeftOver.is_null())
        {
            let mut mt: TEXT = TEXT {
                baseline: POINT { x: 0, y: 0 },
                pStr: ::core::ptr::null::<UNICODE>(),
                align: TEXT_ALIGN::ALIGN_LEFT,
                CharCount: 0,
            };
            let mut buffer: [UNICODE; 128] = [0; 128];
            mt.baseline.x = ((CanvasWidth
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
                    } << resolutionFactor)) >> resolutionFactor
                >> 1 as ::core::ffi::c_int) << resolutionFactor) as COORD;
            mt.baseline.y = t.baseline.y;
            mt.align = TEXT_ALIGN::ALIGN_CENTER;
            snprintf(
                &raw mut buffer as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[UNICODE; 128]>() as size_t,
                b"%s%s%s\0".as_ptr() as *const ::core::ffi::c_char,
                b"\xC2\xB7\0".as_ptr() as *const ::core::ffi::c_char,
                GetStringAddress(
                    SetAbsStringTableIndex(
                        GameStrings,
                        (C2Rust_Unnamed_4::FEEDBACK_STRING_BASE.0 as ::core::ffi::c_int
                            + 1 as ::core::ffi::c_int) as COUNT,
                    ),
                ),
                b"\xC2\xB7\0".as_ptr() as *const ::core::ffi::c_char,
            );
            if CommData.AlienConv.0 == CONVERSATION::ORZ_CONVERSATION.0 {
                remove_char_from_string(&raw mut buffer as *mut UNICODE, '$' as UNICODE);
            }
            mt.pStr = &raw mut buffer as *mut UNICODE;
            SetContextForeGroundColor(
                colorFromRgb15(0 as BYTE, 0x17 as BYTE, 0 as BYTE),
            );
            uqm_lua_checked!(font_DrawText(&raw mut mt), BOOLEAN(0));
        }
        (*pSS).PrintNext = BOOLEAN::FALSE;
    } else {
        FlushInput();
    }
    uqm_lua_checked!(UpdateSpeechGraphics(), BOOLEAN(0));
    return BOOLEAN::TRUE;
}
unsafe extern "C" fn ClearResponses(mut pES: *mut ENCOUNTER_STATE) {
    let mut responseI: size_t = 0;
    responseI = 0 as size_t;
    while responseI < 8 as size_t {
        let mut response: *mut RESPONSE_ENTRY = (&raw mut (*pES).response_list
            as *mut RESPONSE_ENTRY)
            .offset(responseI as isize);
        if !(*response).allocedResponse.is_null() {
            HFree((*response).allocedResponse as *mut ::core::ffi::c_void);
            (*response).allocedResponse = ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        responseI = responseI.wrapping_add(1);
    }
}
unsafe extern "C" fn SelectResponse(mut pES: *mut ENCOUNTER_STATE) {
    let mut response_text: *mut TEXT = &raw mut (*(&raw mut (*pES).response_list
        as *mut RESPONSE_ENTRY)
        .offset((*pES).cur_response as isize))
        .response_text;
    utf8StringCopy(
        &raw mut (*pES).phrase_buf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[UNICODE; 1024]>() as size_t,
        (*response_text).pStr,
    );
    if optSpeech.0 == 0 && optSmoothScroll == 0x2 as ::core::ffi::c_int {
        let mut timeout: TimeCount = GetTimeCounter()
            .wrapping_add(
                (840 as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as TimeCount,
            );
        uqm_lua_checked!(RefreshResponsesSpecial(pES), ());
        while GetTimeCounter() < timeout {
            uqm_lua_checked!(UpdateDuty(BOOLEAN::FALSE), ());
            uqm_lua_checked!(UpdateAnimations(BOOLEAN::FALSE), ());
        }
    } else {
        uqm_lua_checked!(FeedbackPlayerPhrase(&raw mut (*pES).phrase_buf as *mut UNICODE), ());
    }
    uqm_lua_checked!(StopTrack(), ());
    ClearSubtitles();
    SetSliderImage(SetAbsFrameIndex(ActivityFrame, 2 as COUNT));
    FlushCustomBaseLine();
    ReleaseTalkingAnim();
    uqm_lua_checked!(FadeMusic(
        (if usingSpeech.0 != 0 && VolasPackPresent.0 == 0 {
            160 as ::core::ffi::c_int / 2 as ::core::ffi::c_int
        } else {
            160 as ::core::ffi::c_int
        }) as BYTE,
        840 as SIZE,
    ), ());
    TalkingFinished = BOOLEAN::FALSE;
    (*pES).num_responses = 0 as BYTE;
    ClearResponses(pES);
    uqm_lua_checked!(Some(
            (*pES)
                .response_list[(*pES).cur_response as usize]
                .response_func
                .expect("non-null function pointer"),
        )
        .expect(
            "non-null function pointer",
        )((*pES).response_list[(*pES).cur_response as usize].response_ref), ());
}
unsafe extern "C" fn SelectConversationSummary(mut pES: *mut ENCOUNTER_STATE) {
    let mut SummaryState: SUMMARY_STATE = SUMMARY_STATE {
        InputFunc: None,
        Initialized: BOOLEAN::FALSE,
        PrintNext: BOOLEAN::FALSE,
        NextSub: ::core::ptr::null_mut::<tfb_soundchunk>(),
        LeftOver: ::core::ptr::null::<UNICODE>(),
    };
    if !pES.is_null() {
        uqm_lua_checked!(FeedbackPlayerPhrase(&raw mut (*pES).phrase_buf as *mut UNICODE), ());
    }
    SummaryState.Initialized = BOOLEAN::FALSE;
    uqm_lua_checked!(DoConvSummary(&raw mut SummaryState), ());
    if !pES.is_null() {
        uqm_lua_checked!(RefreshResponses(pES), ());
    }
    clear_subtitles = BOOLEAN::TRUE;
}
unsafe extern "C" fn SelectReplay(mut pES: *mut ENCOUNTER_STATE) {
    uqm_lua_checked!(FadeMusic(
        (if usingSpeech.0 != 0 && VolasPackPresent.0 == 0 {
            160 as ::core::ffi::c_int / 2 as ::core::ffi::c_int
        } else {
            160 as ::core::ffi::c_int
        }) as BYTE,
        840 as SIZE,
    ), ());
    if !pES.is_null() {
        uqm_lua_checked!(FeedbackPlayerPhrase(&raw mut (*pES).phrase_buf as *mut UNICODE), ());
    }
    uqm_lua_checked!(TalkSegue(0 as COUNT), ());
}
unsafe extern "C" fn PlayerResponseInput(mut pES: *mut ENCOUNTER_STATE) {
    let mut response: BYTE = 0;
    if (*pES).top_response as ::core::ffi::c_int
        == !(0 as ::core::ffi::c_int) as BYTE as ::core::ffi::c_int
    {
        (*pES).top_response = 0 as BYTE;
        uqm_lua_checked!(RefreshResponses(pES), ());
    }
    if PulsedInputState
        .menu[C2Rust_Unnamed::KEY_MENU_SELECT.0 as ::core::ffi::c_int as usize] != 0
    {
        uqm_lua_checked!(SelectResponse(pES), ());
    } else if PulsedInputState
        .menu[C2Rust_Unnamed::KEY_MENU_CANCEL.0 as ::core::ffi::c_int as usize] != 0
        && GlobData.Game_state.CurrentActivity as BYTE as ::core::ffi::c_int
            != C2Rust_Unnamed_1::WON_LAST_BATTLE.0 as ::core::ffi::c_int
        && IsDarkMode.0 == 0
    {
        uqm_lua_checked!(SelectConversationSummary(pES), ());
    } else {
        response = (*pES).cur_response;
        if PulsedInputState
            .menu[C2Rust_Unnamed::KEY_MENU_LEFT.0 as ::core::ffi::c_int as usize] != 0
            && (optSpeech.0 != 0 || optSmoothScroll == 0x1 as ::core::ffi::c_int)
        {
            uqm_lua_checked!(SelectReplay(pES), ());
            if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
                & C2Rust_Unnamed_1::CHECK_ABORT.0 as ::core::ffi::c_int == 0
            {
                uqm_lua_checked!(RefreshResponses(pES), ());
                uqm_lua_checked!(FadeMusic(160 as BYTE, 840 as SIZE), ());
            }
        } else if PulsedInputState
            .menu[C2Rust_Unnamed::KEY_MENU_UP.0 as ::core::ffi::c_int as usize] != 0
        {
            response = ((response as ::core::ffi::c_int
                + ((*pES).num_responses as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                    as BYTE as ::core::ffi::c_int)
                % (*pES).num_responses as ::core::ffi::c_int) as BYTE;
        } else if PulsedInputState
            .menu[C2Rust_Unnamed::KEY_MENU_DOWN.0 as ::core::ffi::c_int as usize] != 0
        {
            response = ((response as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                as BYTE as ::core::ffi::c_int
                % (*pES).num_responses as ::core::ffi::c_int) as BYTE;
        }
        if response as ::core::ffi::c_int != (*pES).cur_response as ::core::ffi::c_int {
            let mut y: COORD = 0;
            BatchGraphics();
            uqm_lua_checked!(add_text(
                -2 as ::core::ffi::c_int,
                &raw mut (*(&raw mut (*pES).response_list as *mut RESPONSE_ENTRY)
                    .offset((*pES).cur_response as isize))
                    .response_text,
            ), ());
            (*pES).cur_response = response;
            y = uqm_lua_checked!(add_text(
                -1 as ::core::ffi::c_int,
                &raw mut (*(&raw mut (*pES).response_list as *mut RESPONSE_ENTRY)
                    .offset((*pES).cur_response as isize))
                    .response_text,
            ), ());
            if (response as ::core::ffi::c_int)
                < (*pES).top_response as ::core::ffi::c_int
            {
                (*pES).top_response = 0 as BYTE;
                uqm_lua_checked!(RefreshResponses(pES), ());
            } else if y as ::core::ffi::c_int
                > CanvasHeight
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
            {
                (*pES).top_response = response;
                uqm_lua_checked!(RefreshResponses(pES), ());
            } else if if resolutionFactor != 2 as ::core::ffi::c_uint {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } != 0
            {
                uqm_lua_checked!(RefreshResponses(pES), ());
            }
            UnbatchGraphics();
        }
        uqm_lua_checked!(UpdateCommGraphics(), ());
    };
}
unsafe extern "C" fn DoLastReplay(mut pLRS: *mut LAST_REPLAY_STATE) -> BOOLEAN {
    if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
        & C2Rust_Unnamed_1::CHECK_ABORT.0 as ::core::ffi::c_int != 0
    {
        return BOOLEAN::FALSE;
    }
    if GetTimeCounter() > (*pLRS).TimeOut {
        return BOOLEAN::FALSE;
    }
    if PulsedInputState
        .menu[C2Rust_Unnamed::KEY_MENU_CANCEL.0 as ::core::ffi::c_int as usize] != 0
        && GlobData.Game_state.CurrentActivity as BYTE as ::core::ffi::c_int
            != C2Rust_Unnamed_1::WON_LAST_BATTLE.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(FadeMusic(
            (if usingSpeech.0 != 0 && VolasPackPresent.0 == 0 {
                160 as ::core::ffi::c_int / 2 as ::core::ffi::c_int
            } else {
                160 as ::core::ffi::c_int
            }) as BYTE,
            840 as SIZE,
        ), BOOLEAN(0));
        uqm_lua_checked!(SelectConversationSummary(::core::ptr::null_mut::<ENCOUNTER_STATE>()), BOOLEAN(0));
        (*pLRS).TimeOut = uqm_lua_checked!(FadeMusic(
                0 as BYTE,
                (840 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as SIZE,
            ), BOOLEAN(0))
            .wrapping_add(
                (840 as ::core::ffi::c_int / 60 as ::core::ffi::c_int) as DWORD,
            ) as TimeCount;
    } else if PulsedInputState
        .menu[C2Rust_Unnamed::KEY_MENU_LEFT.0 as ::core::ffi::c_int as usize] != 0
    {
        uqm_lua_checked!(SelectReplay(::core::ptr::null_mut::<ENCOUNTER_STATE>()), BOOLEAN(0));
        (*pLRS).TimeOut = uqm_lua_checked!(FadeMusic(
                0 as BYTE,
                (840 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as SIZE,
            ), BOOLEAN(0))
            .wrapping_add(
                (840 as ::core::ffi::c_int / 60 as ::core::ffi::c_int) as DWORD,
            ) as TimeCount;
    }
    while GetTimeCounter() < (*pLRS).NextTime {
        uqm_lua_checked!(UpdateCommGraphics(), BOOLEAN(0));
    }
    (*pLRS).NextTime = GetTimeCounter()
        .wrapping_add(
            (840 as ::core::ffi::c_int / 40 as ::core::ffi::c_int) as TimeCount,
        );
    return BOOLEAN::TRUE;
}
unsafe extern "C" fn DoCommunication(mut pES: *mut ENCOUNTER_STATE) -> BOOLEAN {
    if IsDarkMode.0 == 0 {
        SetMenuSounds(
            (((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as MENU_SOUND_FLAGS
                as ::core::ffi::c_int
                | ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
                    as MENU_SOUND_FLAGS as ::core::ffi::c_int) as MENU_SOUND_FLAGS,
            ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as MENU_SOUND_FLAGS,
        );
    }
    if TalkingFinished.0 == 0 {
        uqm_lua_checked!(AlienTalkSegue(!(0 as ::core::ffi::c_int) as COUNT), BOOLEAN(0));
        return BOOLEAN::TRUE;
    }
    if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
        & C2Rust_Unnamed_1::CHECK_ABORT.0 as ::core::ffi::c_int != 0
    {
        UninitOscilloscope();
    } else if (*pES).num_responses as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        let mut replayState: LAST_REPLAY_STATE = LAST_REPLAY_STATE {
            InputFunc: None,
            NextTime: 0,
            TimeOut: 0,
        };
        memset(
            &raw mut replayState as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<LAST_REPLAY_STATE>() as size_t,
        );
        replayState.TimeOut = uqm_lua_checked!(FadeMusic(
                0 as BYTE,
                (840 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as SIZE,
            ), BOOLEAN(0))
            .wrapping_add(
                (840 as ::core::ffi::c_int / 60 as ::core::ffi::c_int) as DWORD,
            ) as TimeCount;
        replayState.InputFunc = Some(
            DoLastReplay as unsafe extern "C" fn(*mut LAST_REPLAY_STATE) -> BOOLEAN,
        ) as Option<unsafe extern "C" fn(*mut last_replay_state) -> BOOLEAN>;
        uqm_lua_checked!(DoInput(&raw mut replayState as *mut ::core::ffi::c_void, BOOLEAN::FALSE), BOOLEAN(0));
    } else {
        uqm_lua_checked!(PlayerResponseInput(pES), BOOLEAN(0));
        return BOOLEAN::TRUE;
    }
    SetContext(SpaceContext);
    DestroyContext(AnimContext);
    AnimContext = ::core::ptr::null_mut::<CONTEXT_DESC>();
    FlushColorXForms();
    ClearSubtitles();
    if IsDarkMode.0 != 0 {
        SetCommDarkMode(BOOLEAN::FALSE);
    }
    uqm_lua_checked!(SetMusicPosition(), BOOLEAN(0));
    uqm_lua_checked!(StopMusic(), BOOLEAN(0));
    uqm_lua_checked!(StopSound(), BOOLEAN(0));
    uqm_lua_checked!(StopTrack(), BOOLEAN(0));
    uqm_lua_checked!(SleepThreadUntil(
        uqm_lua_checked!(FadeMusic(160 as BYTE, 0 as SIZE), BOOLEAN(0))
            .wrapping_add(
                (840 as ::core::ffi::c_int / 60 as ::core::ffi::c_int) as TimeCount,
            ),
    ), BOOLEAN(0));
    UninitOscilloscope();
    return BOOLEAN::FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn DoResponsePhrase(
    mut R: RESPONSE_REF,
    mut response_func: RESPONSE_FUNC,
    mut ConstructStr: *mut UNICODE,
) {
    let mut pES: *mut ENCOUNTER_STATE = pCurInputState;
    let mut pEntry: *mut RESPONSE_ENTRY = ::core::ptr::null_mut::<RESPONSE_ENTRY>();
    if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
        & C2Rust_Unnamed_1::CHECK_ABORT.0 as ::core::ffi::c_int != 0
    {
        return;
    }
    if (*pES).num_responses as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        (*pES).cur_response = 0 as BYTE;
        (*pES).top_response = !(0 as ::core::ffi::c_int) as BYTE;
    }
    pEntry = (&raw mut (*pES).response_list as *mut RESPONSE_ENTRY)
        .offset((*pES).num_responses as isize);
    (*pEntry).response_ref = R;
    (*pEntry).response_text.pStr = ConstructStr;
    if !(*pEntry).response_text.pStr.is_null() {
        (*pEntry).response_text.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    } else {
        let mut locString: STRING = ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>();
        locString = SetAbsStringTableIndex(
            CommData.ConversationPhrases,
            (R as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as COUNT,
        );
        (*pEntry).response_text.pStr = GetStringAddress(locString);
        if luaUqm_comm_stringNeedsInterpolate((*pEntry).response_text.pStr).0 != 0 {
            (*pEntry).allocedResponse = uqm_lua_checked!(luaUqm_comm_stringInterpolate(
                (*pEntry).response_text.pStr,
            ), ());
            (*pEntry).response_text.pStr = (*pEntry).allocedResponse;
        }
        (*pEntry).response_text.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    }
    (*pEntry).response_func = response_func;
    (*pES).num_responses = (*pES).num_responses.wrapping_add(1);
}
#[no_mangle]
pub unsafe extern "C" fn SetUpCommData() {
    if altResFlags as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int != 0
    {
        CommData.AlienFrame = CaptureDrawable(
            uqm_lua_checked!(LoadGraphicInstance(CommData.AltRes.AlienFrameRes), ()) as DRAWABLE,
        );
        if CommData.AlienFrame.is_null() {
            CommData.AlienFrame = CaptureDrawable(
                uqm_lua_checked!(LoadGraphicInstance(CommData.AlienFrameRes), ()) as DRAWABLE,
            );
            altResFlags = (altResFlags as ::core::ffi::c_int
                & !((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int)) as BYTE;
        }
    } else {
        CommData.AlienFrame = CaptureDrawable(
            uqm_lua_checked!(LoadGraphicInstance(CommData.AlienFrameRes), ()) as DRAWABLE,
        );
    }
    CommData.AlienFont = uqm_lua_checked!(LoadGraphicInstance(CommData.AlienFontRes), ()) as FONT;
    if altResFlags as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int != 0
    {
        CommData.AlienColorMap = CaptureStringTable(
            uqm_lua_checked!(LoadStringTableInstance(CommData.AltRes.AlienColorMapRes), ()),
        ) as COLORMAP;
        if CommData.AlienColorMap.is_null() {
            CommData.AlienColorMap = CaptureStringTable(
                uqm_lua_checked!(LoadStringTableInstance(CommData.AlienColorMapRes), ()),
            ) as COLORMAP;
            altResFlags = (altResFlags as ::core::ffi::c_int
                & !((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)) as BYTE;
        }
    } else {
        CommData.AlienColorMap = CaptureStringTable(
            uqm_lua_checked!(LoadStringTableInstance(CommData.AlienColorMapRes), ()),
        ) as COLORMAP;
    }
    if altResFlags as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int != 0
    {
        CommData.AlienSong = uqm_lua_checked!(LoadMusicInstance(CommData.AltRes.AlienSongRes), ());
        if CommData.AlienSong.is_null() {
            CommData.AlienSong = uqm_lua_checked!(LoadMusicInstance(CommData.AlienSongRes), ());
            altResFlags = (altResFlags as ::core::ffi::c_int
                & !((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)) as BYTE;
        }
    } else {
        CommData.AlienSong = uqm_lua_checked!(LoadMusicInstance(CommData.AlienSongRes), ());
    }
    CommData.ConversationPhrases = CaptureStringTable(
        uqm_lua_checked!(LoadStringTableInstance(CommData.ConversationPhrasesRes), ()),
    );
}
unsafe extern "C" fn HailAlien() {
    let mut ES: ENCOUNTER_STATE = ENCOUNTER_STATE {
        InputFunc: None,
        Initialized: 0,
        NextTime: 0,
        num_responses: 0,
        cur_response: 0,
        top_response: 0,
        response_list: [RESPONSE_ENTRY {
            response_ref: 0,
            response_text: TEXT {
                baseline: POINT { x: 0, y: 0 },
                pStr: ::core::ptr::null::<UNICODE>(),
                align: TEXT_ALIGN::ALIGN_LEFT,
                CharCount: 0,
            },
            response_func: None,
            allocedResponse: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        }; 8],
        phrase_buf: [0; 1024],
    };
    let mut PlayerFont: FONT = ::core::ptr::null_mut::<FONT_DESC>();
    let mut OldFont: FONT = ::core::ptr::null_mut::<FONT_DESC>();
    let mut TextBack: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    pCurInputState = &raw mut ES;
    memset(
        pCurInputState as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<ENCOUNTER_STATE>() as size_t,
    );
    TalkingFinished = BOOLEAN::FALSE;
    ES.InputFunc = Some(
        DoCommunication as unsafe extern "C" fn(*mut ENCOUNTER_STATE) -> BOOLEAN,
    ) as Option<unsafe extern "C" fn(*mut encounter_state) -> BOOLEAN>;
    if isPC(optWhichFonts).0 != 0 {
        PlayerFont = uqm_lua_checked!(LoadGraphicInstance(b"font.player\0".as_ptr() as RESOURCE), ()) as FONT;
    } else {
        PlayerFont = uqm_lua_checked!(LoadGraphicInstance(b"font.tiny.bold\0".as_ptr() as RESOURCE), ())
            as FONT;
    }
    if optOrzCompFont.0 != 0 {
        ComputerFont = uqm_lua_checked!(LoadGraphicInstance(b"comm.computer.font\0".as_ptr() as RESOURCE), ())
            as FONT;
        if ComputerFont.is_null() {
            log_add(
                log_Level::log_Warning,
                b"ComputerFont didn't load properly. Disabling Alternate Orz Font\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
            optOrzCompFont = OPT_ENABLABLE::OPTVAL_DISABLED;
        }
    }
    CommData.AlienFrame = CaptureDrawable(
        uqm_lua_checked!(LoadGraphicInstance(CommData.AlienFrameRes), ()) as DRAWABLE,
    );
    CommData.AlienFont = uqm_lua_checked!(LoadGraphicInstance(CommData.AlienFontRes), ()) as FONT;
    CommData.AlienColorMap = CaptureStringTable(
        uqm_lua_checked!(LoadStringTableInstance(CommData.AlienColorMapRes), ()),
    ) as COLORMAP;
    uqm_lua_checked!(SetUpCommData(), ());
    SubtitleText.baseline = CommData.AlienTextBaseline;
    SubtitleText.align = CommData.AlienTextAlign;
    TextCacheContext = CreateContextAux(
        b"TextCacheContext\0".as_ptr() as *const ::core::ffi::c_char,
    );
    TextCacheFrame = CaptureDrawable(
        CreateDrawable(
            ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as CREATE_FLAGS,
            (CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
                    } << resolutionFactor))) as SIZE,
            ((107 as ::core::ffi::c_int) << resolutionFactor) as SIZE,
            1 as COUNT,
        ),
    );
    SetContext(TextCacheContext);
    SetContextFGFrame(TextCacheFrame);
    TextBack = colorFromRgb15(0 as BYTE, 0 as BYTE, 0x1 as BYTE);
    SetContextBackGroundColor(TextBack);
    uqm_lua_checked!(ClearDrawable(), ());
    SetFrameTransparentColor(TextCacheFrame, TextBack);
    ES.phrase_buf[0usize] = '\0' as UNICODE;
    SetContext(SpaceContext);
    OldFont = SetContextFont(PlayerFont);
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    AnimContext = CreateContextAux(
        b"AnimContext\0".as_ptr() as *const ::core::ffi::c_char,
    );
    SetContext(AnimContext);
    SetContextFGFrame(Screen);
    GetFrameRect(CommData.AlienFrame, &raw mut r);
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
            } << resolutionFactor))) as COORD;
    CommWndRect.corner.x = (((6 as ::core::ffi::c_int) << resolutionFactor)
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
    CommWndRect.corner.y = (((9 as ::core::ffi::c_int) << resolutionFactor)
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
    CommWndRect.extent = r.extent;
    SetTransitionSource(::core::ptr::null::<RECT>());
    BatchGraphics();
    if GlobData.Game_state.CurrentActivity as BYTE as ::core::ffi::c_int
        == C2Rust_Unnamed_1::WON_LAST_BATTLE.0 as ::core::ffi::c_int
    {
        CommWndRect.corner.x = ((CanvasWidth
            - CommWndRect.extent.width as ::core::ffi::c_int) / 2 as ::core::ffi::c_int)
            as COORD;
        CommWndRect.corner.y = ((5 as ::core::ffi::c_int) << resolutionFactor) as COORD;
        r.corner = CommWndRect.corner;
        SetContextClipRect(&raw mut r);
    } else {
        let mut Log: POINT = MAKE_POINT(
            logxToUniverse(GlobData.SIS_state.log_x) as COORD,
            logyToUniverse(GlobData.SIS_state.log_y) as COORD,
        );
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
            }) as COORD;
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
        SetContextClipRect(&raw mut r);
        CommWndRect.corner = r.corner;
        uqm_lua_checked!(DrawSISFrame(), ());
        if uqm_lua_checked!(getGameStateUint(
            b"GLOBAL_FLAGS_AND_DATA\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) == !(0 as ::core::ffi::c_int) as BYTE as DWORD
            && uqm_lua_checked!(getGameStateUint(
                b"STARBASE_AVAILABLE\0".as_ptr() as *const ::core::ffi::c_char,
            ), ()) != 0
        {
            uqm_lua_checked!(DrawSISMessage(
                GetStringAddress(
                    SetAbsStringTableIndex(
                        GameStrings,
                        (C2Rust_Unnamed_4::STARBASE_STRING_BASE.0 as ::core::ffi::c_int
                            + 1 as ::core::ffi::c_int) as COUNT,
                    ),
                ),
            ), ());
            uqm_lua_checked!(DrawSISTitle(
                GetStringAddress(
                    SetAbsStringTableIndex(
                        GameStrings,
                        (C2Rust_Unnamed_4::STARBASE_STRING_BASE.0 as ::core::ffi::c_int
                            + 0 as ::core::ffi::c_int) as COUNT,
                    ),
                ),
            ), ());
        } else {
            uqm_lua_checked!(DrawSISMessage(::core::ptr::null::<UNICODE>()), ());
            if inHQSpace().0 != 0 {
                uqm_lua_checked!(DrawHyperCoords(GlobData.Game_state.ShipStamp.origin), ());
                if uqm_lua_checked!(getGameStateUint(
                    b"ARILOU_SPACE_SIDE\0".as_ptr() as *const ::core::ffi::c_char,
                ), ()) > 1 as DWORD
                    && uqm_lua_checked!(getGameStateUint(
                        b"ARILOU_HOME_VISITS\0".as_ptr() as *const ::core::ffi::c_char,
                    ), ()) != 0
                    && (Log.x as ::core::ffi::c_int == 6134 as ::core::ffi::c_int
                        && Log.y as ::core::ffi::c_int == 5900 as ::core::ffi::c_int)
                {
                    uqm_lua_checked!(DrawSISMessage(
                        &raw mut GlobData.SIS_state.PlanetName as *mut UNICODE,
                    ), ());
                }
            } else if GlobData.Game_state.ip_planet as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
            {
                uqm_lua_checked!(DrawHyperCoords((*CurStarDescPtr).star_pt), ());
            } else {
                uqm_lua_checked!(DrawSISTitle(&raw mut GlobData.SIS_state.PlanetName as *mut UNICODE), ());
            }
        }
    }
    uqm_lua_checked!(DrawSISComWindow(), ());
    LastActivity = (LastActivity as ::core::ffi::c_int
        | C2Rust_Unnamed_1::CHECK_LOAD.0 as ::core::ffi::c_int) as ACTIVITY;
    uqm_lua_checked!(Some(CommData.init_encounter_func.expect("non-null function pointer"))
        .expect("non-null function pointer")(), ());
    uqm_lua_checked!(DoInput(&raw mut ES as *mut ::core::ffi::c_void, BOOLEAN::FALSE), ());
    if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
        & (C2Rust_Unnamed_1::CHECK_ABORT.0 as ::core::ffi::c_int
            | C2Rust_Unnamed_1::CHECK_LOAD.0 as ::core::ffi::c_int) == 0
    {
        uqm_lua_checked!(Some(CommData.post_encounter_func.expect("non-null function pointer"))
            .expect("non-null function pointer")(), ());
    }
    uqm_lua_checked!(Some(CommData.uninit_encounter_func.expect("non-null function pointer"))
        .expect("non-null function pointer")(), ());
    ClearResponses(&raw mut ES);
    SetContext(SpaceContext);
    SetContextFont(OldFont);
    DestroyStringTable(ReleaseStringTable(CommData.ConversationPhrases));
    uqm_lua_checked!(DestroyMusic(CommData.AlienSong), ());
    DestroyStringTable(ReleaseStringTable(CommData.AlienColorMap));
    DestroyFont(CommData.AlienFont);
    DestroyDrawable(ReleaseDrawable(CommData.AlienFrame));
    DestroyContext(TextCacheContext);
    DestroyDrawable(ReleaseDrawable(TextCacheFrame));
    DestroyFont(PlayerFont);
    if optOrzCompFont.0 != 0 {
        DestroyFont(ComputerFont);
    }
    ReleaseTalkingAnim();
    DisengageFilters();
    altResFlags = (altResFlags as ::core::ffi::c_int
        & !((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)) as BYTE;
    CommData.ConversationPhrasesRes = ::core::ptr::null::<::core::ffi::c_char>();
    CommData.ConversationPhrases = ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>();
    pCurInputState = ::core::ptr::null_mut::<ENCOUNTER_STATE>();
}
#[no_mangle]
pub unsafe extern "C" fn SetCommIntroMode(
    mut newMode: CommIntroMode,
    mut howLong: TimeCount,
) {
    curIntroMode = newMode;
    fadeTime = howLong;
}
#[no_mangle]
pub unsafe extern "C" fn InitCommunication(mut which_comm: CONVERSATION) -> COUNT {
    let mut status: COUNT = 0;
    let mut LocDataPtr: *mut LOCDATA = ::core::ptr::null_mut::<LOCDATA>();
    if disableInteractivity.0 != 0 {
        return 0 as COUNT;
    }
    if LastActivity as ::core::ffi::c_int
        & C2Rust_Unnamed_1::CHECK_LOAD.0 as ::core::ffi::c_int != 0
    {
        LastActivity = (LastActivity as ::core::ffi::c_int
            & !(C2Rust_Unnamed_1::CHECK_LOAD.0 as ::core::ffi::c_int)) as ACTIVITY;
        if which_comm.0 != CONVERSATION::COMMANDER_CONVERSATION.0 {
            if LastActivity as BYTE as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                uqm_lua_checked!(DrawSISFrame(), 0);
            } else {
                uqm_lua_checked!(ClearSISRect(
                    ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as BYTE,
                ), 0);
                uqm_lua_checked!(RepairSISBorder(), 0);
            }
            uqm_lua_checked!(DrawSISMessage(::core::ptr::null::<UNICODE>()), 0);
            if inHQSpace().0 != 0 {
                uqm_lua_checked!(DrawHyperCoords(GlobData.Game_state.ShipStamp.origin), 0);
            } else if GlobData.Game_state.ip_planet as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
            {
                uqm_lua_checked!(DrawHyperCoords((*CurStarDescPtr).star_pt), 0);
            } else {
                if !pSolarSysState.is_null()
                    && GlobData.Game_state.CurrentActivity as BYTE as ::core::ffi::c_int
                        != C2Rust_Unnamed_1::IN_LAST_BATTLE.0 as ::core::ffi::c_int
                    && worldIsMoon(pSolarSysState, (*pSolarSysState).pOrbitalDesc)
                        as ::core::ffi::c_int != 0
                {
                    if uqm_lua_checked!(GetNamedPlanetaryBody(), 0).is_null() && isPC(optWhichFonts).0 != 0
                        && ((*(*pSolarSysState).pOrbitalDesc).data_index
                            as ::core::ffi::c_int
                            != C2Rust_Unnamed_2::HIERARCHY_STARBASE.0
                                as ::core::ffi::c_int
                            && (*(*pSolarSysState).pOrbitalDesc).data_index
                                as ::core::ffi::c_int
                                != C2Rust_Unnamed_2::DESTROYED_STARBASE.0
                                    as ::core::ffi::c_int
                            && (*(*pSolarSysState).pOrbitalDesc).data_index
                                as ::core::ffi::c_int
                                != C2Rust_Unnamed_2::PRECURSOR_STARBASE.0
                                    as ::core::ffi::c_int)
                    {
                        snprintf(
                            (&raw mut GlobData.SIS_state.PlanetName as *mut UNICODE)
                                .offset(
                                    strlen(
                                        &raw mut GlobData.SIS_state.PlanetName as *mut UNICODE,
                                    ) as isize,
                                ),
                            4 as size_t,
                            b"-%c%c\0".as_ptr() as *const ::core::ffi::c_char,
                            'A' as ::core::ffi::c_int
                                + moonIndex(pSolarSysState, (*pSolarSysState).pOrbitalDesc)
                                    as ::core::ffi::c_int,
                            '\0' as ::core::ffi::c_int,
                        );
                    }
                }
                uqm_lua_checked!(DrawSISTitle(&raw mut GlobData.SIS_state.PlanetName as *mut UNICODE), 0);
            }
        }
    }
    if which_comm.0 == CONVERSATION::URQUAN_DRONE_CONVERSATION.0 {
        status = RACE_ID::URQUAN_DRONE_SHIP.0 as ::core::ffi::c_int as COUNT;
        which_comm = CONVERSATION::URQUAN_CONVERSATION;
    } else {
        if which_comm.0 == CONVERSATION::YEHAT_REBEL_CONVERSATION.0 {
            status = RACE_ID::YEHAT_REBEL_SHIP.0 as ::core::ffi::c_int as COUNT;
            which_comm = CONVERSATION::YEHAT_CONVERSATION;
        } else {
            let mut commToShip: [COUNT; 30] = [
                RACE_ID::ARILOU_SHIP.0 as ::core::ffi::c_int as COUNT,
                RACE_ID::CHMMR_SHIP.0 as ::core::ffi::c_int as COUNT,
                RACE_ID::HUMAN_SHIP.0 as ::core::ffi::c_int as COUNT,
                RACE_ID::ORZ_SHIP.0 as ::core::ffi::c_int as COUNT,
                RACE_ID::PKUNK_SHIP.0 as ::core::ffi::c_int as COUNT,
                RACE_ID::SHOFIXTI_SHIP.0 as ::core::ffi::c_int as COUNT,
                RACE_ID::SPATHI_SHIP.0 as ::core::ffi::c_int as COUNT,
                RACE_ID::SUPOX_SHIP.0 as ::core::ffi::c_int as COUNT,
                RACE_ID::THRADDASH_SHIP.0 as ::core::ffi::c_int as COUNT,
                RACE_ID::UTWIG_SHIP.0 as ::core::ffi::c_int as COUNT,
                RACE_ID::VUX_SHIP.0 as ::core::ffi::c_int as COUNT,
                RACE_ID::YEHAT_SHIP.0 as ::core::ffi::c_int as COUNT,
                RACE_ID::MELNORME_SHIP.0 as ::core::ffi::c_int as COUNT,
                RACE_ID::DRUUGE_SHIP.0 as ::core::ffi::c_int as COUNT,
                RACE_ID::ILWRATH_SHIP.0 as ::core::ffi::c_int as COUNT,
                RACE_ID::MYCON_SHIP.0 as ::core::ffi::c_int as COUNT,
                RACE_ID::SLYLANDRO_SHIP.0 as ::core::ffi::c_int as COUNT,
                RACE_ID::UMGAH_SHIP.0 as ::core::ffi::c_int as COUNT,
                RACE_ID::URQUAN_SHIP.0 as ::core::ffi::c_int as COUNT,
                RACE_ID::ZOQFOTPIK_SHIP.0 as ::core::ffi::c_int as COUNT,
                RACE_ID::SYREEN_SHIP.0 as ::core::ffi::c_int as COUNT,
                RACE_ID::BLACK_URQUAN_SHIP.0 as ::core::ffi::c_int as COUNT,
                RACE_ID::ANDROSYNTH_SHIP.0 as ::core::ffi::c_int as COUNT,
                RACE_ID::CHENJESU_SHIP.0 as ::core::ffi::c_int as COUNT,
                RACE_ID::MMRNMHRM_SHIP.0 as ::core::ffi::c_int as COUNT,
                RACE_ID::UMGAH_SHIP.0 as ::core::ffi::c_int as COUNT,
                RACE_ID::SLYLANDRO_SHIP.0 as ::core::ffi::c_int as COUNT,
                RACE_ID::URQUAN_DRONE_SHIP.0 as ::core::ffi::c_int as COUNT,
                RACE_ID::YEHAT_SHIP.0 as ::core::ffi::c_int as COUNT,
                RACE_ID::HUMAN_SHIP.0 as ::core::ffi::c_int as COUNT,
            ];
            status = commToShip[which_comm.0 as usize];
            if status as ::core::ffi::c_int
                >= RACE_ID::YEHAT_REBEL_SHIP.0 as ::core::ffi::c_int
            {
                status = RACE_ID::HUMAN_SHIP.0 as ::core::ffi::c_int as COUNT;
            }
        }
        StartSphereTracking(RACE_ID(status as ::core::ffi::c_uint));
        if which_comm.0 == CONVERSATION::ORZ_CONVERSATION.0
            || which_comm.0 == CONVERSATION::TALKING_PET_CONVERSATION.0
                && (uqm_lua_checked!(getGameStateUint(
                    b"TALKING_PET_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
                ), 0) == 0
                    || GlobData.Game_state.CurrentActivity as BYTE as ::core::ffi::c_int
                        == C2Rust_Unnamed_1::IN_LAST_BATTLE.0 as ::core::ffi::c_int)
            || which_comm.0 != CONVERSATION::CHMMR_CONVERSATION.0
                && which_comm.0 != CONVERSATION::SYREEN_CONVERSATION.0
        {
            uqm_lua_checked!(BuildBattle(1 as COUNT), 0);
        }
    }
    LocDataPtr = uqm_lua_checked!(init_race(
        CONVERSATION(
            if status as ::core::ffi::c_int
                != RACE_ID::YEHAT_REBEL_SHIP.0 as ::core::ffi::c_int
            {
                which_comm.0
            } else {
                CONVERSATION::YEHAT_REBEL_CONVERSATION.0
            },
        ),
    ), 0);
    if !LocDataPtr.is_null() {
        CommData = *LocDataPtr;
    }
    if uqm_lua_checked!(getGameStateUint(b"BATTLE_SEGUE\0".as_ptr() as *const ::core::ffi::c_char), 0)
        == 0 as DWORD
    {
        status = C2Rust_Unnamed_3::HAIL.0 as ::core::ffi::c_int as COUNT;
    } else {
        status = uqm_lua_checked!(InitEncounter(), 0);
        if status as ::core::ffi::c_int == C2Rust_Unnamed_3::HAIL.0 as ::core::ffi::c_int
            && !LocDataPtr.is_null()
        {
            uqm_lua_checked!(setGameStateUint(
                b"BATTLE_SEGUE\0".as_ptr() as *const ::core::ffi::c_char,
                0 as DWORD,
            ), 0);
        } else {
            status = C2Rust_Unnamed_3::ATTACK.0 as ::core::ffi::c_int as COUNT;
            uqm_lua_checked!(setGameStateUint(
                b"BATTLE_SEGUE\0".as_ptr() as *const ::core::ffi::c_char,
                1 as DWORD,
            ), 0);
        }
    }
    if status as ::core::ffi::c_int == C2Rust_Unnamed_3::HAIL.0 as ::core::ffi::c_int {
        uqm_lua_checked!(HailAlien(), 0);
    } else if !LocDataPtr.is_null() {
        if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
            & (C2Rust_Unnamed_1::CHECK_ABORT.0 as ::core::ffi::c_int
                | C2Rust_Unnamed_1::CHECK_LOAD.0 as ::core::ffi::c_int) == 0
        {
            uqm_lua_checked!(Some(CommData.post_encounter_func.expect("non-null function pointer"))
                .expect("non-null function pointer")(), 0);
        }
        uqm_lua_checked!(Some(CommData.uninit_encounter_func.expect("non-null function pointer"))
            .expect("non-null function pointer")(), 0);
    }
    status = 0 as COUNT;
    if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
        & (C2Rust_Unnamed_1::CHECK_ABORT.0 as ::core::ffi::c_int
            | C2Rust_Unnamed_1::CHECK_LOAD.0 as ::core::ffi::c_int) == 0
    {
        if GlobData.Game_state.CurrentActivity as BYTE as ::core::ffi::c_int
            == C2Rust_Unnamed_1::IN_LAST_BATTLE.0 as ::core::ffi::c_int
            && GlobData.Game_state.glob_flags as ::core::ffi::c_int
                & (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int != 0
        {
            ReinitQueue(&raw mut GlobData.Game_state.npc_built_ship_q);
        }
        uqm_lua_checked!(setGameStateUint(
            b"GLOBAL_FLAGS_AND_DATA\0".as_ptr() as *const ::core::ffi::c_char,
            0 as DWORD,
        ), 0);
        status = (uqm_lua_checked!(getGameStateUint(
            b"BATTLE_SEGUE\0".as_ptr() as *const ::core::ffi::c_char,
        ), 0) != 0 && !GlobData.Game_state.npc_built_ship_q.head.is_null())
            as ::core::ffi::c_int as COUNT;
        if status != 0 {
            if if GlobData.SIS_state.Extended as ::core::ffi::c_int != 0 {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
                && uqm_lua_checked!(getGameStateUint(
                    b"URQUAN_PROTECTING_SAMATRA\0".as_ptr() as *const ::core::ffi::c_char,
                ), 0) != 0
            {
                free_gravity_well();
                uqm_lua_checked!(load_gravity_well(
                    uqm_lua_checked!(getGameStateUint(
                        b"BATTLE_PLANET\0".as_ptr() as *const ::core::ffi::c_char,
                    ), 0) as BYTE,
                ), 0);
            }
            uqm_lua_checked!(BuildBattle(0 as COUNT), 0);
            uqm_lua_checked!(EncounterBattle(), 0);
        } else {
            uqm_lua_checked!(setGameStateUint(
                b"BATTLE_SEGUE\0".as_ptr() as *const ::core::ffi::c_char,
                0 as DWORD,
            ), 0);
        }
    }
    uqm_lua_checked!(UninitEncounter(), 0);
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn RaceCommunication() {
    let mut i: COUNT = 0;
    let mut status: COUNT = 0;
    let mut hStarShip: HSHIPFRAG = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut FragPtr: *mut SHIP_FRAGMENT = ::core::ptr::null_mut::<SHIP_FRAGMENT>();
    let mut hEncounter: HENCOUNTER = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut RaceComm: [CONVERSATION; 27] = [
        CONVERSATION::ARILOU_CONVERSATION,
        CONVERSATION::CHMMR_CONVERSATION,
        CONVERSATION::INVALID_CONVERSATION,
        CONVERSATION::ORZ_CONVERSATION,
        CONVERSATION::PKUNK_CONVERSATION,
        CONVERSATION::SHOFIXTI_CONVERSATION,
        CONVERSATION::SPATHI_CONVERSATION,
        CONVERSATION::SUPOX_CONVERSATION,
        CONVERSATION::THRADD_CONVERSATION,
        CONVERSATION::UTWIG_CONVERSATION,
        CONVERSATION::VUX_CONVERSATION,
        CONVERSATION::YEHAT_CONVERSATION,
        CONVERSATION::MELNORME_CONVERSATION,
        CONVERSATION::DRUUGE_CONVERSATION,
        CONVERSATION::ILWRATH_CONVERSATION,
        CONVERSATION::MYCON_CONVERSATION,
        CONVERSATION::SLYLANDRO_CONVERSATION,
        CONVERSATION::UMGAH_CONVERSATION,
        CONVERSATION::URQUAN_CONVERSATION,
        CONVERSATION::ZOQFOTPIK_CONVERSATION,
        CONVERSATION::INVALID_CONVERSATION,
        CONVERSATION::BLACKURQ_CONVERSATION,
        CONVERSATION::INVALID_CONVERSATION,
        CONVERSATION::INVALID_CONVERSATION,
        CONVERSATION::INVALID_CONVERSATION,
        CONVERSATION::YEHAT_REBEL_CONVERSATION,
        CONVERSATION::URQUAN_DRONE_CONVERSATION,
    ];
    if GlobData.Game_state.CurrentActivity as BYTE as ::core::ffi::c_int
        == C2Rust_Unnamed_1::IN_LAST_BATTLE.0 as ::core::ffi::c_int
    {
        ReinitQueue(&raw mut GlobData.Game_state.npc_built_ship_q);
        CloneShipFragment(
            RACE_ID::SAMATRA_SHIP,
            &raw mut GlobData.Game_state.npc_built_ship_q,
            0 as COUNT,
        );
        uqm_lua_checked!(InitCommunication(CONVERSATION::TALKING_PET_CONVERSATION), ());
        if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
            & (C2Rust_Unnamed_1::CHECK_ABORT.0 as ::core::ffi::c_int
                | C2Rust_Unnamed_1::CHECK_LOAD.0 as ::core::ffi::c_int) == 0
            && GlobData.SIS_state.CrewEnlisted as ::core::ffi::c_int
                != !(0 as ::core::ffi::c_int) as COUNT as ::core::ffi::c_int
        {
            GlobData.Game_state.CurrentActivity = C2Rust_Unnamed_1::WON_LAST_BATTLE.0
                as ::core::ffi::c_int as ACTIVITY;
        }
        return;
    } else if NextActivity as ::core::ffi::c_int
        & C2Rust_Unnamed_1::CHECK_LOAD.0 as ::core::ffi::c_int != 0
    {
        let mut ec: BYTE = 0;
        ec = uqm_lua_checked!(getGameStateUint(b"ESCAPE_COUNTER\0".as_ptr() as *const ::core::ffi::c_char), ())
            as BYTE;
        if uqm_lua_checked!(getGameStateUint(
            b"FOUND_PLUTO_SPATHI\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) == 1 as DWORD
        {
            uqm_lua_checked!(InitCommunication(CONVERSATION::SPATHI_CONVERSATION), ());
        } else if uqm_lua_checked!(getGameStateUint(
            b"GLOBAL_FLAGS_AND_DATA\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) == 0 as DWORD
        {
            uqm_lua_checked!(InitCommunication(CONVERSATION::TALKING_PET_CONVERSATION), ());
        } else if uqm_lua_checked!(getGameStateUint(
            b"GLOBAL_FLAGS_AND_DATA\0".as_ptr() as *const ::core::ffi::c_char,
        ), ())
            & ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as DWORD != 0
        {
            uqm_lua_checked!(InitCommunication(CONVERSATION::ILWRATH_CONVERSATION), ());
        } else {
            uqm_lua_checked!(InitCommunication(CONVERSATION::CHMMR_CONVERSATION), ());
        }
        if GlobData.SIS_state.CrewEnlisted as ::core::ffi::c_int
            != !(0 as ::core::ffi::c_int) as COUNT as ::core::ffi::c_int
        {
            NextActivity = (GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
                & !(C2Rust_Unnamed_1::START_ENCOUNTER.0 as ::core::ffi::c_int))
                as ACTIVITY;
            if NextActivity as BYTE as ::core::ffi::c_int
                == C2Rust_Unnamed_1::IN_INTERPLANETARY.0 as ::core::ffi::c_int
            {
                NextActivity = (NextActivity as ::core::ffi::c_int
                    | C2Rust_Unnamed_1::START_INTERPLANETARY.0 as ::core::ffi::c_int)
                    as ACTIVITY;
            }
            GlobData.Game_state.CurrentActivity = (GlobData.Game_state.CurrentActivity
                as ::core::ffi::c_int
                | C2Rust_Unnamed_1::CHECK_LOAD.0 as ::core::ffi::c_int) as ACTIVITY;
        }
        uqm_lua_checked!(setGameStateUint(
            b"ESCAPE_COUNTER\0".as_ptr() as *const ::core::ffi::c_char,
            ec as DWORD,
        ), ());
        return;
    } else if inHQSpace().0 != 0 {
        ReinitQueue(&raw mut GlobData.Game_state.npc_built_ship_q);
        if uqm_lua_checked!(getGameStateUint(
            b"ARILOU_SPACE_SIDE\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) >= 2 as DWORD
        {
            uqm_lua_checked!(InitCommunication(CONVERSATION::ARILOU_CONVERSATION), ());
            return;
        } else {
            let mut EncounterPtr: *mut ENCOUNTER = ::core::ptr::null_mut::<ENCOUNTER>();
            hEncounter = GlobData.Game_state.encounter_q.head as HENCOUNTER;
            EncounterPtr = LockLink(&raw mut GlobData.Game_state.encounter_q, hEncounter)
                as *mut ENCOUNTER;
            i = 0 as COUNT;
            while (i as ::core::ffi::c_int)
                < (*EncounterPtr).num_ships as ::core::ffi::c_int
            {
                CloneShipFragment(
                    RACE_ID((*EncounterPtr).race_id as ::core::ffi::c_uint),
                    &raw mut GlobData.Game_state.npc_built_ship_q,
                    (*EncounterPtr).ShipList[i as usize].crew_level,
                );
                i = i.wrapping_add(1);
            }
            UnlockLink(&raw mut GlobData.Game_state.encounter_q, hEncounter);
        }
    }
    hStarShip = GlobData.Game_state.npc_built_ship_q.head as HSHIPFRAG;
    FragPtr = LockShipFrag(&raw mut GlobData.Game_state.npc_built_ship_q, hStarShip);
    i = (*FragPtr).race_id as COUNT;
    UnlockLink(&raw mut GlobData.Game_state.npc_built_ship_q, hStarShip);
    status = uqm_lua_checked!(InitCommunication(RaceComm[i as usize]), ());
    if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
        & (C2Rust_Unnamed_1::CHECK_ABORT.0 as ::core::ffi::c_int
            | C2Rust_Unnamed_1::CHECK_LOAD.0 as ::core::ffi::c_int) != 0
    {
        return;
    }
    if i as ::core::ffi::c_int == RACE_ID::CHMMR_SHIP.0 as ::core::ffi::c_int {
        ReinitQueue(&raw mut GlobData.Game_state.npc_built_ship_q);
    }
    if GlobData.Game_state.CurrentActivity as BYTE as ::core::ffi::c_int
        == C2Rust_Unnamed_1::IN_INTERPLANETARY.0 as ::core::ffi::c_int
    {
        if i as ::core::ffi::c_int == RACE_ID::SLYLANDRO_SHIP.0 as ::core::ffi::c_int
            && status as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        {
            ReinitQueue(&raw mut GlobData.Game_state.npc_built_ship_q);
        }
    } else if !hEncounter.is_null() {
        let mut i_0: BYTE = 0;
        let mut NumShips: BYTE = 0;
        let mut EncounterPtr_0: *mut ENCOUNTER = ::core::ptr::null_mut::<ENCOUNTER>();
        EncounterPtr_0 = LockLink(&raw mut GlobData.Game_state.encounter_q, hEncounter)
            as *mut ENCOUNTER;
        NumShips = CountLinks(&raw mut GlobData.Game_state.npc_built_ship_q) as BYTE;
        (*EncounterPtr_0).num_ships = NumShips;
        (*EncounterPtr_0).flags = ((*EncounterPtr_0).flags as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as BYTE;
        if status as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (*EncounterPtr_0).flags = ((*EncounterPtr_0).flags as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as BYTE;
        }
        i_0 = 0 as BYTE;
        while (i_0 as ::core::ffi::c_int) < NumShips as ::core::ffi::c_int {
            let mut hStarShip_0: HSHIPFRAG = ::core::ptr::null_mut::<
                ::core::ffi::c_void,
            >();
            let mut FragPtr_0: *mut SHIP_FRAGMENT = ::core::ptr::null_mut::<
                SHIP_FRAGMENT,
            >();
            let mut BSIPtr: *mut BRIEF_SHIP_INFO = ::core::ptr::null_mut::<
                BRIEF_SHIP_INFO,
            >();
            hStarShip_0 = GetStarShipFromIndex(
                &raw mut GlobData.Game_state.npc_built_ship_q,
                i_0 as COUNT,
            ) as HSHIPFRAG;
            FragPtr_0 = LockShipFrag(
                &raw mut GlobData.Game_state.npc_built_ship_q,
                hStarShip_0,
            );
            BSIPtr = (&raw mut (*EncounterPtr_0).ShipList as *mut BRIEF_SHIP_INFO)
                .offset(i_0 as isize);
            (*BSIPtr).race_id = (*FragPtr_0).race_id;
            (*BSIPtr).crew_level = (*FragPtr_0).crew_level;
            (*BSIPtr).max_crew = (*FragPtr_0).max_crew;
            (*BSIPtr).max_energy = (*FragPtr_0).max_energy;
            UnlockLink(&raw mut GlobData.Game_state.npc_built_ship_q, hStarShip_0);
            i_0 = i_0.wrapping_add(1);
        }
        UnlockLink(&raw mut GlobData.Game_state.encounter_q, hEncounter);
        ReinitQueue(&raw mut GlobData.Game_state.npc_built_ship_q);
    }
}
unsafe extern "C" fn RedrawSubtitles() {
    let mut t: TEXT = TEXT {
        baseline: POINT { x: 0, y: 0 },
        pStr: ::core::ptr::null::<UNICODE>(),
        align: TEXT_ALIGN::ALIGN_LEFT,
        CharCount: 0,
    };
    if optSubtitles.0 == 0 {
        return;
    }
    if !SubtitleText.pStr.is_null() {
        t = SubtitleText;
        if !cur_node.is_null() {
            t.baseline = (*cur_node).baseline;
            t.align = (*cur_node).align;
        }
        uqm_lua_checked!(add_text(1 as ::core::ffi::c_int, &raw mut t), ());
    }
}
unsafe extern "C" fn ClearSubtitles() {
    clear_subtitles = BOOLEAN::TRUE;
    last_subtitle = ::core::ptr::null::<UNICODE>();
    SubtitleText.pStr = ::core::ptr::null::<UNICODE>();
    SubtitleText.CharCount = 0 as COUNT;
}
unsafe extern "C" fn PauseSubtitles(mut force: BOOLEAN) -> BOOLEAN {
    let mut pStr: *const UNICODE = ::core::ptr::null::<UNICODE>();
    static mut num: COUNT = 0 as COUNT;
    pStr = GetTrackSubtitle();
    if GetSubtitleNumber(pStr) as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        || SubtitleText.pStr.is_null()
    {
        num = GetSubtitleNumber(pStr);
        return BOOLEAN::FALSE;
    }
    if num as ::core::ffi::c_int != GetSubtitleNumber(pStr) as ::core::ffi::c_int
        && force.0 == 0
    {
        num = GetSubtitleNumber(pStr);
        return BOOLEAN::TRUE;
    }
    num = GetSubtitleNumber(pStr);
    return BOOLEAN::FALSE;
}
unsafe extern "C" fn CheckSubtitles(mut really: BOOLEAN) {
    let mut pStr: *const UNICODE = ::core::ptr::null::<UNICODE>();
    let mut baseline: POINT = POINT { x: 0, y: 0 };
    let mut align: TEXT_ALIGN = TEXT_ALIGN::ALIGN_LEFT;
    let mut num: COUNT = 0;
    if really.0 == 0 {
        next_page = BOOLEAN::TRUE;
        return;
    }
    pStr = GetTrackSubtitle();
    baseline = CommData.AlienTextBaseline;
    align = CommData.AlienTextAlign;
    if pStr != SubtitleText.pStr
        || SubtitleText.baseline.x as ::core::ffi::c_int
            != baseline.x as ::core::ffi::c_int
        || SubtitleText.baseline.y as ::core::ffi::c_int
            != baseline.y as ::core::ffi::c_int || SubtitleText.align.0 != align.0
    {
        num = GetSubtitleNumber(pStr);
        GetCustomBaseline(num);
        CheckTalkingAnim(num);
        clear_subtitles = BOOLEAN::TRUE;
        SubtitleText.baseline = baseline;
        SubtitleText.align = align;
        if SubtitleText.pStr == pStr {
            log_add(
                log_Level::log_Warning,
                b"Dialog text and location changed out of sync\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        SubtitleText.pStr = pStr;
        if !pStr.is_null() {
            SubtitleText.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
        } else {
            SubtitleText.CharCount = 0 as COUNT;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn EnableTalkingAnim(mut enable: BOOLEAN) {
    if enable.0 != 0 {
        CommData.AlienTalkDesc.AnimFlags = (CommData.AlienTalkDesc.AnimFlags
            as ::core::ffi::c_int
            & !((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)) as COUNT;
    } else {
        CommData.AlienTalkDesc.AnimFlags = (CommData.AlienTalkDesc.AnimFlags
            as ::core::ffi::c_int | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
            as COUNT;
    };
}
#[no_mangle]
pub unsafe extern "C" fn SetCommDarkMode(mut state: BOOLEAN) {
    IsDarkMode = state;
    oscillDisabled = state;
    sliderDisabled = state;
    InitUIRects(state);
    fadeIndex = state.0 as COUNT;
    if state.0 == 0 {
        ReleaseTalkingAnim();
    }
}
#[no_mangle]
pub unsafe extern "C" fn RedrawSISComWindow() {
    uqm_lua_checked!(DrawSISComWindow(), ());
}
#[no_mangle]
pub unsafe extern "C" fn SetCustomBaseLine(
    mut sentence: COUNT,
    mut bl: POINT,
    mut align: TEXT_ALIGN,
) {
    let mut cur: *mut CUSTOM_BASELINE = ::core::ptr::null_mut::<CUSTOM_BASELINE>();
    let mut sPtr: *mut CUSTOM_BASELINE = ::core::ptr::null_mut::<CUSTOM_BASELINE>();
    cur = HCalloc(::core::mem::size_of::<CUSTOM_BASELINE>() as size_t)
        as *mut CUSTOM_BASELINE;
    (*cur).index = sentence;
    (*cur).baseline.x = ((bl.x as ::core::ffi::c_int) << resolutionFactor) as COORD;
    (*cur).baseline.y = ((bl.y as ::core::ffi::c_int) << resolutionFactor) as COORD;
    (*cur).align = align;
    if head_node.is_null() {
        head_node = cur;
    } else {
        sPtr = head_node;
        while !(*sPtr).next.is_null() {
            sPtr = (*sPtr).next as *mut CUSTOM_BASELINE;
        }
        (*sPtr).next = cur as *mut CustomBaseline;
    };
}
#[no_mangle]
pub unsafe extern "C" fn FlushCustomBaseLine() {
    let mut cur: *mut CUSTOM_BASELINE = ::core::ptr::null_mut::<CUSTOM_BASELINE>();
    let mut next: *mut CUSTOM_BASELINE = ::core::ptr::null_mut::<CUSTOM_BASELINE>();
    if head_node.is_null() {
        return;
    }
    cur = head_node;
    while !cur.is_null() {
        next = (*cur).next as *mut CUSTOM_BASELINE;
        HFree(cur as *mut ::core::ffi::c_void);
        cur = next;
    }
    head_node = ::core::ptr::null_mut::<CUSTOM_BASELINE>();
    cur_node = ::core::ptr::null_mut::<CUSTOM_BASELINE>();
}
#[inline]
unsafe extern "C" fn haveTalkingAnim() -> BOOLEAN {
    return BOOLEAN(
        (CommData.AlienTalkDesc.NumFrames as ::core::ffi::c_int
            > 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
}
#[inline]
unsafe extern "C" fn haveTransitionAnim() -> BOOLEAN {
    return BOOLEAN(
        (CommData.AlienTransitionDesc.NumFrames as ::core::ffi::c_int
            > 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
}
#[inline]
unsafe extern "C" fn wantTalkingAnim() -> BOOLEAN {
    return BOOLEAN(
        (CommData.AlienTalkDesc.AnimFlags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int == 0)
            as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
}
#[inline]
unsafe extern "C" fn setRunTalkingAnim() {
    CommData.AlienTalkDesc.AnimFlags = (CommData.AlienTalkDesc.AnimFlags
        as ::core::ffi::c_int | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
        as COUNT;
}
#[inline]
unsafe extern "C" fn runningTalkingAnim() -> BOOLEAN {
    return BOOLEAN(
        (CommData.AlienTalkDesc.AnimFlags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
            as ::core::ffi::c_uint,
    );
}
#[inline]
unsafe extern "C" fn setRunIntroAnim() {
    CommData.AlienTransitionDesc.AnimFlags = (CommData.AlienTransitionDesc.AnimFlags
        as ::core::ffi::c_int | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int)
        as COUNT;
}
#[inline]
unsafe extern "C" fn runningIntroAnim() -> BOOLEAN {
    return BOOLEAN(
        (CommData.AlienTransitionDesc.AnimFlags as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int)
            as ::core::ffi::c_uint,
    );
}
#[inline]
unsafe extern "C" fn setStopTalkingAnim() {
    CommData.AlienTalkDesc.AnimFlags = (CommData.AlienTalkDesc.AnimFlags
        as ::core::ffi::c_int | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
        as COUNT;
}
#[inline]
unsafe extern "C" fn freezeTalkingAnim() {
    CommData.AlienTalkDesc.AnimFlags = (CommData.AlienTalkDesc.AnimFlags
        as ::core::ffi::c_int | (1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int)
        as COUNT;
}
#[inline]
unsafe extern "C" fn unFreezeTalkingAnim() {
    CommData.AlienTalkDesc.AnimFlags = (CommData.AlienTalkDesc.AnimFlags
        as ::core::ffi::c_int & !((1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int))
        as COUNT;
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
                340 as ::core::ffi::c_int,
                b"LockShipFrag\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    return LockLink(pq, h) as *mut SHIP_FRAGMENT;
}
