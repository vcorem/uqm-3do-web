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
    );
    fn ReinitQueue(pq: *mut QUEUE);
    fn CountLinks(pq: *mut QUEUE) -> COUNT;
    fn SetContext(Context: CONTEXT) -> CONTEXT;
    fn SetContextForeGroundColor(Color: Color) -> Color;
    fn SetContextBackGroundColor(Color: Color) -> Color;
    fn SetContextFGFrame(Frame: FRAME) -> FRAME;
    fn SetContextClipRect(pRect: *mut RECT) -> BOOLEAN;
    fn DrawStamp(pStamp: *mut STAMP);
    fn DrawFilledRectangle(pRect: *mut RECT);
    fn font_DrawText(pText: *mut TEXT);
    fn font_DrawTracedText(pText: *mut TEXT, text: Color, trace: Color);
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
    fn snprintf(
        _: *mut ::core::ffi::c_char,
        _: size_t,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn getCharFromString(ptr: *mut *const ::core::ffi::c_char) -> UniChar;
    fn utf8StringCopy(
        dst: *mut ::core::ffi::c_char,
        size: size_t,
        src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_uchar;
    fn LoadStringTableInstance(res: RESOURCE) -> STRING_TABLE;
    fn DestroyStringTable(StringTable: STRING_TABLE) -> BOOLEAN;
    fn CaptureStringTable(StringTable: STRING_TABLE) -> STRING;
    fn ReleaseStringTable(String: STRING) -> STRING_TABLE;
    fn SetAbsStringTableIndex(String: STRING, StringTableIndex: COUNT) -> STRING;
    fn GetStringLength(String: STRING) -> COUNT;
    fn GetStringAddress(String: STRING) -> STRINGPTR;
    fn LoadMusicInstance(res: RESOURCE) -> MUSIC_REF;
    fn DestroyMusic(MusicRef: MUSIC_REF) -> BOOLEAN;
    fn SetMusicVolume(Volume: COUNT);
    fn StopSound();
    fn FadeMusic(end_vol: BYTE, TimeInterval: SIZE) -> DWORD;
    fn memset(
        _: *mut ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn RepairSISBorder();
    fn DrawSISFrame();
    fn ClearSISRect(ClearFlags: BYTE);
    fn DrawHyperCoords(puniverse: POINT);
    fn DrawSISTitle(pStr: *mut UNICODE);
    fn DrawSISMessage(pStr: *const UNICODE);
    fn DrawAlienFrame(pSeq: *mut SEQUENCE, Num: COUNT, fullRedraw: BOOLEAN) -> BOOLEAN;
    fn InitCommAnimations();
    fn ProcessCommAnimations(fullRedraw: BOOLEAN, paused: BOOLEAN) -> BOOLEAN;
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
    static mut RadarContext: CONTEXT;
    fn inHQSpace() -> BOOLEAN;
    fn init_race(comm_id: CONVERSATION) -> *mut LOCDATA;
    fn PlayTrack();
    fn StopTrack();
    fn JumpTrack();
    fn PlayingTrack() -> COUNT;
    fn FastReverse_Smooth();
    fn FastForward_Smooth();
    fn FastReverse_Page();
    fn FastForward_Page();
    fn GetFirstTrackSubtitle() -> SUBTITLE_REF;
    fn GetNextTrackSubtitle(LastRef: SUBTITLE_REF) -> SUBTITLE_REF;
    fn GetTrackSubtitleText(SubRef: SUBTITLE_REF) -> *const UNICODE;
    fn GetTrackSubtitle() -> *const UNICODE;
    static mut ScreenWidth: ::core::ffi::c_int;
    static mut ScreenHeight: ::core::ffi::c_int;
    fn CloneShipFragment(
        shipIndex: COUNT,
        pDstQueue: *mut QUEUE,
        crew_level: COUNT,
    ) -> HSHIPFRAG;
    fn GetStarShipFromIndex(pShipQ: *mut QUEUE, Index: COUNT) -> HLINK;
    fn StartSphereTracking(race: COUNT) -> COUNT;
    static mut CurrentInputState: CONTROLLER_INPUT_STATE;
    static mut PulsedInputState: CONTROLLER_INPUT_STATE;
    fn DoInput(pInputState: *mut ::core::ffi::c_void, resetInput: BOOLEAN);
    fn EncounterBattle();
    fn BuildBattle(which_player: COUNT);
    fn InitEncounter() -> COUNT;
    fn UninitEncounter() -> COUNT;
    static mut optSmoothScroll: ::core::ffi::c_int;
    static mut optSubtitles: BOOLEAN;
    fn InitOscilloscope(scopeBg: FRAME);
    fn DrawOscilloscope();
    fn InitSlider(
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
        width: ::core::ffi::c_int,
        sliderFrame: FRAME,
        buttonFrame: FRAME,
    );
    fn SetSliderImage(f: FRAME);
    fn DrawSlider();
    static mut NextActivity: ACTIVITY;
    fn StopMusic();
    fn PlayMusic(MusicRef: MUSIC_REF, Continuous: BOOLEAN, Priority: BYTE);
    static mut GameStrings: STRING;
    static mut CurStarDescPtr: *mut STAR_DESC;
    static mut Screen: FRAME;
    static mut ActivityFrame: FRAME;
    static mut SpaceContext: CONTEXT;
    static mut TinyFont: FONT;
    static mut LastActivity: ACTIVITY;
    static mut usingSpeech: BOOLEAN;
    fn SetMenuSounds(sound_0: MENU_SOUND_FLAGS, sound_1: MENU_SOUND_FLAGS);
    static mut disableInteractivity: BOOLEAN;
    fn SetTransitionSource(pRect: *const RECT);
    fn ScreenTransition(transition: ::core::ffi::c_int, pRect: *const RECT);
    fn log_add(_: log_Level, fmt: *const ::core::ffi::c_char, ...);
}
pub type size_t = ::core::ffi::c_ulong;
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
pub type SUBTITLE_REF = *mut tfb_soundchunk;
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
}
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
}
pub type CONTROLLER_INPUT_STATE = _controller_input_state;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _controller_input_state {
    pub key: [[::core::ffi::c_int; 7]; 6],
    pub menu: [::core::ffi::c_int; 24],
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
pub struct C2Rust_Unnamed_1(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_1 {
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
pub struct C2Rust_Unnamed_2(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_2 {
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
    pub const KEY_MENU_ANY: Self = Self(23);
    pub const NUM_MENU_KEYS: Self = Self(24);
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
    pub const DEVICE_STRING_BASE: Self = Self(133);
    pub const CARGO_STRING_BASE: Self = Self(162);
    pub const ELEMENTS_STRING_BASE: Self = Self(172);
    pub const SCAN_STRING_BASE: Self = Self(305);
    pub const STAR_NUMBER_BASE: Self = Self(361);
    pub const PLANET_NUMBER_BASE: Self = Self(375);
    pub const MONTHS_STRING_BASE: Self = Self(408);
    pub const FEEDBACK_STRING_BASE: Self = Self(420);
    pub const STARBASE_STRING_BASE: Self = Self(422);
    pub const ENCOUNTER_STRING_BASE: Self = Self(427);
    pub const NAVIGATION_STRING_BASE: Self = Self(435);
    pub const NAMING_STRING_BASE: Self = Self(441);
    pub const MELEE_STRING_BASE: Self = Self(445);
    pub const SAVEGAME_STRING_BASE: Self = Self(454);
    pub const OPTION_STRING_BASE: Self = Self(459);
    pub const QUITMENU_STRING_BASE: Self = Self(464);
    pub const STATUS_STRING_BASE: Self = Self(467);
    pub const FLAGSHIP_STRING_BASE: Self = Self(473);
    pub const ORBITSCAN_STRING_BASE: Self = Self(486);
    pub const MAINMENU_STRING_BASE: Self = Self(505);
    pub const NETMELEE_STRING_BASE: Self = Self(560);
    pub const GAMESTR_COUNT: Self = Self(579);
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
static mut AnimContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
#[no_mangle]
pub static mut CommData: LOCDATA = LOCDATA {
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
    AlienAltSongRes: ::core::ptr::null::<::core::ffi::c_char>(),
    AlienSongFlags: 0,
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
    }; 20],
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
static mut TalkingFinished: BOOLEAN = BOOLEAN::FALSE;
static mut curIntroMode: CommIntroMode = CommIntroMode::CIM_DEFAULT;
static mut fadeTime: TimeCount = 0;
static mut pCurInputState: *mut ENCOUNTER_STATE = ::core::ptr::null_mut::<
    ENCOUNTER_STATE,
>();
static mut clear_subtitles: BOOLEAN = BOOLEAN::FALSE;
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
        x: (7 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as COORD,
        y: (10 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as COORD,
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
    BatchGraphics();
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
            DrawStamp(&raw mut s);
            UnbatchGraphics();
            return last_baseline;
        } else {
            OldContext = SetContext(TextCacheContext);
            ClearDrawable();
            last_subtitle = (*pTextIn).pStr;
        }
        text_width = CommData.AlienTextWidth as SIZE;
        SetContextFont(CommData.AlienFont);
        GetContextFontLeading(&raw mut leading);
        pText = pTextIn;
    } else {
        GetContextFontLeading(&raw mut leading);
        if status <= -4 as ::core::ffi::c_int {
            text_width = (ScreenWidth - 64 as ::core::ffi::c_int
                - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                - 14 as ::core::ffi::c_int - 8 as ::core::ffi::c_int
                - ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)) as SIZE;
            pText = pTextIn;
        } else {
            text_width = (ScreenWidth - 64 as ::core::ffi::c_int
                - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                - 14 as ::core::ffi::c_int - 8 as ::core::ffi::c_int
                - ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)) as SIZE;
            match status {
                -3 => {
                    SetContextForeGroundColor(
                        colorFromRgb15(0 as BYTE, 0 as BYTE, 0x14 as BYTE),
                    );
                }
                -2 => {
                    SetContextForeGroundColor(
                        colorFromRgb15(0 as BYTE, 0x14 as BYTE, 0x14 as BYTE),
                    );
                }
                -1 => {
                    SetContextForeGroundColor(
                        colorFromRgb15(0x1a as BYTE, 0x1a as BYTE, 0x1a as BYTE),
                    );
                }
                _ => {}
            }
            maxchars = (*pTextIn).CharCount;
            locText = *pTextIn;
            locText.baseline.x = (locText.baseline.x as ::core::ffi::c_int
                - 8 as ::core::ffi::c_int) as COORD;
            locText.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
            locText.pStr = b"\xE2\x80\xA2\0".as_ptr() as *const ::core::ffi::c_char
                as *const UNICODE;
            font_DrawText(&raw mut locText);
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
                < ScreenHeight - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    - 13 as ::core::ffi::c_int
            {
                font_DrawText(pText);
            }
            if status < -4 as ::core::ffi::c_int
                && (*pText).baseline.y as ::core::ffi::c_int
                    >= -status - 10 as ::core::ffi::c_int
            {
                pStr = pStr.offset(1);
                break;
            }
        } else {
            font_DrawTracedText(
                pText,
                CommData.AlienTextFColor,
                CommData.AlienTextBColor,
            );
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
        DrawStamp(&raw mut s_0);
        last_baseline = (*pText).baseline.y;
    }
    UnbatchGraphics();
    return (*pText).baseline.y;
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
unsafe extern "C" fn DrawSISComWindow() {
    let mut OldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    if GlobData.Game_state.CurrentActivity as BYTE as ::core::ffi::c_int
        != C2Rust_Unnamed_0::WON_LAST_BATTLE.0 as ::core::ffi::c_int
    {
        let mut r: RECT = RECT {
            corner: POINT { x: 0, y: 0 },
            extent: EXTENT { width: 0, height: 0 },
        };
        OldContext = SetContext(SpaceContext);
        r.corner.x = 0 as COORD;
        r.corner.y = (107 as ::core::ffi::c_int + 15 as ::core::ffi::c_int) as COORD;
        r.extent.width = (ScreenWidth - 64 as ::core::ffi::c_int
            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
            - 14 as ::core::ffi::c_int) as COORD;
        r.extent.height = (ScreenHeight
            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
            - 13 as ::core::ffi::c_int - r.corner.y as ::core::ffi::c_int) as COORD;
        SetContextForeGroundColor(colorFromRgb15(0 as BYTE, 0 as BYTE, 0x14 as BYTE));
        DrawFilledRectangle(&raw mut r);
        SetContext(OldContext);
    }
}
#[no_mangle]
pub unsafe extern "C" fn init_communication() {}
#[no_mangle]
pub unsafe extern "C" fn uninit_communication() {}
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
    DrawSISComWindow();
    y = (107 as ::core::ffi::c_int + 15 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
        as COORD;
    response = (*pES).top_response;
    while (response as ::core::ffi::c_int) < (*pES).num_responses as ::core::ffi::c_int {
        (*pES).response_list[response as usize].response_text.baseline.x = (1
            as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as COORD;
        (*pES).response_list[response as usize].response_text.baseline.y = (y
            as ::core::ffi::c_int + leading as ::core::ffi::c_int) as COORD;
        (*pES).response_list[response as usize].response_text.align = TEXT_ALIGN::ALIGN_LEFT;
        if response as ::core::ffi::c_int == (*pES).cur_response as ::core::ffi::c_int {
            y = add_text(
                -1 as ::core::ffi::c_int,
                &raw mut (*(&raw mut (*pES).response_list as *mut RESPONSE_ENTRY)
                    .offset(response as isize))
                    .response_text,
            );
        } else {
            y = add_text(
                -2 as ::core::ffi::c_int,
                &raw mut (*(&raw mut (*pES).response_list as *mut RESPONSE_ENTRY)
                    .offset(response as isize))
                    .response_text,
            );
        }
        response = response.wrapping_add(1);
    }
    if (*pES).top_response != 0 {
        s.origin.y = (107 as ::core::ffi::c_int + 15 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int) as COORD;
        s.frame = SetAbsFrameIndex(ActivityFrame, 6 as COUNT);
    } else if y as ::core::ffi::c_int
        > ScreenHeight - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
            - 13 as ::core::ffi::c_int
    {
        s.origin.y = (ScreenHeight - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
            - 13 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as COORD;
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
        s.origin.x = (ScreenWidth - 64 as ::core::ffi::c_int
            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
            - 14 as ::core::ffi::c_int - r.extent.width as ::core::ffi::c_int
            - 1 as ::core::ffi::c_int) as COORD;
        DrawStamp(&raw mut s);
    }
    UnbatchGraphics();
}
unsafe extern "C" fn FeedbackPlayerPhrase(mut pStr: *mut UNICODE) {
    SetContext(SpaceContext);
    BatchGraphics();
    DrawSISComWindow();
    if *pStr.offset(0isize) != 0 {
        let mut ct: TEXT = TEXT {
            baseline: POINT { x: 0, y: 0 },
            pStr: ::core::ptr::null::<UNICODE>(),
            align: TEXT_ALIGN::ALIGN_LEFT,
            CharCount: 0,
        };
        ct.baseline.x = (ScreenWidth - 64 as ::core::ffi::c_int
            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
            - 14 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as COORD;
        ct.baseline.y = (107 as ::core::ffi::c_int + 15 as ::core::ffi::c_int
            + 13 as ::core::ffi::c_int) as COORD;
        ct.align = TEXT_ALIGN::ALIGN_CENTER;
        ct.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
        ct.pStr = GetStringAddress(
            SetAbsStringTableIndex(
                GameStrings,
                C2Rust_Unnamed_4::FEEDBACK_STRING_BASE.0 as ::core::ffi::c_int as COUNT,
            ),
        );
        SetContextForeGroundColor(
            colorFromRgb15(0xa as BYTE, 0xc as BYTE, 0x1f as BYTE),
        );
        font_DrawText(&raw mut ct);
        ct.baseline.y = (ct.baseline.y as ::core::ffi::c_int + 16 as ::core::ffi::c_int)
            as COORD;
        SetContextForeGroundColor(
            colorFromRgb15(0x12 as BYTE, 0x14 as BYTE, 0x4f as BYTE),
        );
        ct.pStr = pStr;
        add_text(-4 as ::core::ffi::c_int, &raw mut ct);
    }
    UnbatchGraphics();
}
unsafe extern "C" fn InitSpeechGraphics() {
    InitOscilloscope(SetAbsFrameIndex(ActivityFrame, 9 as COUNT));
    InitSlider(
        0 as ::core::ffi::c_int,
        107 as ::core::ffi::c_int,
        ScreenWidth - 64 as ::core::ffi::c_int
            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
            - 14 as ::core::ffi::c_int,
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
            (840 as ::core::ffi::c_int / 32 as ::core::ffi::c_int) as TimeCount,
        );
    OldContext = SetContext(RadarContext);
    DrawOscilloscope();
    SetContext(SpaceContext);
    DrawSlider();
    SetContext(OldContext);
}
unsafe extern "C" fn UpdateAnimations(mut paused: bool) {
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
    change = ProcessCommAnimations(
        clear_subtitles,
        BOOLEAN(paused as ::core::ffi::c_uint),
    );
    if change.0 != 0 || clear_subtitles.0 != 0 {
        RedrawSubtitles();
    }
    UnbatchGraphics();
    clear_subtitles = BOOLEAN::FALSE;
    SetContext(OldContext);
}
unsafe extern "C" fn UpdateCommGraphics() {
    UpdateAnimations(false);
    UpdateSpeechGraphics();
}
unsafe extern "C" fn DoTalkSegue(mut pTS: *mut TALKING_STATE) -> BOOLEAN {
    let mut left: bool = false;
    let mut right: bool = false;
    let mut curTrack: COUNT = 0;
    if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
        & C2Rust_Unnamed_0::CHECK_ABORT.0 as ::core::ffi::c_int != 0
    {
        (*pTS).ended = true;
        return BOOLEAN::FALSE;
    }
    if PulsedInputState
        .menu[C2Rust_Unnamed_2::KEY_MENU_CANCEL.0 as ::core::ffi::c_int as usize] != 0
    {
        JumpTrack();
        (*pTS).ended = true;
        return BOOLEAN::FALSE;
    }
    if optSmoothScroll == 0x2 as ::core::ffi::c_int {
        left = PulsedInputState
            .menu[C2Rust_Unnamed_2::KEY_MENU_LEFT.0 as ::core::ffi::c_int as usize]
            != 0 as ::core::ffi::c_int;
        right = PulsedInputState
            .menu[C2Rust_Unnamed_2::KEY_MENU_RIGHT.0 as ::core::ffi::c_int as usize]
            != 0 as ::core::ffi::c_int;
    } else if optSmoothScroll == 0x1 as ::core::ffi::c_int {
        left = CurrentInputState
            .menu[C2Rust_Unnamed_2::KEY_MENU_LEFT.0 as ::core::ffi::c_int as usize]
            != 0 as ::core::ffi::c_int;
        right = CurrentInputState
            .menu[C2Rust_Unnamed_2::KEY_MENU_RIGHT.0 as ::core::ffi::c_int as usize]
            != 0 as ::core::ffi::c_int;
    }
    if right {
        SetSliderImage(SetAbsFrameIndex(ActivityFrame, 3 as COUNT));
        if optSmoothScroll == 0x2 as ::core::ffi::c_int {
            FastForward_Page();
        } else if optSmoothScroll == 0x1 as ::core::ffi::c_int {
            FastForward_Smooth();
        }
        (*pTS).seeking = true;
    } else if left as ::core::ffi::c_int != 0 || (*pTS).rewind as ::core::ffi::c_int != 0
    {
        (*pTS).rewind = false;
        SetSliderImage(SetAbsFrameIndex(ActivityFrame, 4 as COUNT));
        if optSmoothScroll == 0x2 as ::core::ffi::c_int {
            FastReverse_Page();
        } else if optSmoothScroll == 0x1 as ::core::ffi::c_int {
            FastReverse_Smooth();
        }
        (*pTS).seeking = true;
    } else if (*pTS).seeking {
        (*pTS).seeking = false;
        SetSliderImage(SetAbsFrameIndex(ActivityFrame, 2 as COUNT));
    } else {
        CheckSubtitles();
    }
    UpdateAnimations((*pTS).seeking);
    UpdateSpeechGraphics();
    curTrack = PlayingTrack();
    (*pTS).ended = !(*pTS).seeking && curTrack == 0;
    SleepThreadUntil((*pTS).NextTime);
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
    UpdateCommGraphics();
    SleepThread((840 as ::core::ffi::c_int / 40 as ::core::ffi::c_int) as TimePeriod);
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
            runCommAnimFrame();
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
        PlayTrack();
        (PlayingTrack() as ::core::ffi::c_int != 0
            || {
                __assert_fail(
                    b"PlayingTrack ()\0".as_ptr() as *const ::core::ffi::c_char,
                    b"src/uqm/comm.c\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    686 as ::core::ffi::c_int,
                    b"TalkSegue\0".as_ptr() as *const ::core::ffi::c_char,
                );
                0 as ::core::ffi::c_int != 0
            }) as ::core::ffi::c_int;
    }
    SetMenuSounds(
        0 as ::core::ffi::c_int as MENU_SOUND_FLAGS,
        0 as ::core::ffi::c_int as MENU_SOUND_FLAGS,
    );
    talkingState.InputFunc = Some(
        DoTalkSegue as unsafe extern "C" fn(*mut TALKING_STATE) -> BOOLEAN,
    ) as Option<unsafe extern "C" fn(*mut talking_state) -> BOOLEAN>;
    talkingState.waitTrack = wait_track;
    DoInput(&raw mut talkingState as *mut ::core::ffi::c_void, BOOLEAN::FALSE);
    ClearSubtitles();
    if talkingState.ended {
        SetSliderImage(SetAbsFrameIndex(ActivityFrame, 8 as COUNT));
    }
    if runningTalkingAnim().0 != 0 {
        setStopTalkingAnim();
    }
    while runningTalkingAnim().0 != 0 {
        runCommAnimFrame();
    }
    return BOOLEAN(talkingState.ended as ::core::ffi::c_uint);
}
unsafe extern "C" fn CommIntroTransition() {
    if curIntroMode.0 == CommIntroMode::CIM_CROSSFADE_SCREEN.0 {
        ScreenTransition(3 as ::core::ffi::c_int, ::core::ptr::null::<RECT>());
        UnbatchGraphics();
    } else if curIntroMode.0 == CommIntroMode::CIM_CROSSFADE_SPACE.0 {
        let mut r: RECT = RECT {
            corner: POINT { x: 0, y: 0 },
            extent: EXTENT { width: 0, height: 0 },
        };
        r.corner.x = (7 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as COORD;
        r.corner.y = (10 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as COORD;
        r.extent.width = (ScreenWidth - 64 as ::core::ffi::c_int
            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
            - 14 as ::core::ffi::c_int) as COORD;
        r.extent.height = (ScreenHeight
            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
            - 13 as ::core::ffi::c_int) as COORD;
        ScreenTransition(3 as ::core::ffi::c_int, &raw mut r);
        UnbatchGraphics();
    } else if curIntroMode.0 == CommIntroMode::CIM_CROSSFADE_WINDOW.0 {
        ScreenTransition(3 as ::core::ffi::c_int, &raw mut CommWndRect);
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
                    745 as ::core::ffi::c_int,
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
        & C2Rust_Unnamed_0::CHECK_ABORT.0 as ::core::ffi::c_int != 0
        || TalkingFinished.0 != 0
    {
        return;
    }
    if (*pCurInputState).Initialized == 0 {
        InitSpeechGraphics();
        SetColorMap(GetColorMapAddress(CommData.AlienColorMap));
        SetContext(AnimContext);
        DrawAlienFrame(::core::ptr::null_mut::<SEQUENCE>(), 0 as COUNT, BOOLEAN::TRUE);
        UpdateSpeechGraphics();
        CommIntroTransition();
        (*pCurInputState).Initialized = BOOLEAN::TRUE.0 as ::core::ffi::c_int as COUNT;
        PlayMusic(CommData.AlienSong, BOOLEAN::TRUE, 1 as BYTE);
        SetMusicVolume(
            (if usingSpeech.0 != 0 {
                160 as ::core::ffi::c_int / 2 as ::core::ffi::c_int
            } else {
                160 as ::core::ffi::c_int
            }) as COUNT,
        );
        InitCommAnimations();
        LastActivity = (LastActivity as ::core::ffi::c_int
            & !(C2Rust_Unnamed_0::CHECK_LOAD.0 as ::core::ffi::c_int)) as ACTIVITY;
    }
    TalkingFinished = TalkSegue(wait_track);
    if TalkingFinished.0 != 0 {
        FadeMusic(160 as BYTE, 840 as SIZE);
    }
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
        DoInput(pSS as *mut ::core::ffi::c_void, BOOLEAN::FALSE);
    } else if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
        & C2Rust_Unnamed_0::CHECK_ABORT.0 as ::core::ffi::c_int != 0
    {
        return BOOLEAN::FALSE
    } else if PulsedInputState
        .menu[C2Rust_Unnamed_2::KEY_MENU_SELECT.0 as ::core::ffi::c_int as usize] != 0
        || PulsedInputState
            .menu[C2Rust_Unnamed_2::KEY_MENU_CANCEL.0 as ::core::ffi::c_int as usize]
            != 0
        || PulsedInputState
            .menu[C2Rust_Unnamed_2::KEY_MENU_RIGHT.0 as ::core::ffi::c_int as usize] != 0
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
        r.extent.width = (ScreenWidth - 64 as ::core::ffi::c_int
            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
            - 14 as ::core::ffi::c_int) as COORD;
        r.extent.height = (ScreenHeight
            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
            - 13 as ::core::ffi::c_int - 107 as ::core::ffi::c_int
            - 15 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as COORD;
        SetContext(AnimContext);
        SetContextForeGroundColor(colorFromRgb15(0 as BYTE, 0x5 as BYTE, 0 as BYTE));
        DrawFilledRectangle(&raw mut r);
        SetContextForeGroundColor(colorFromRgb15(0 as BYTE, 0x10 as BYTE, 0 as BYTE));
        r.extent.width = (r.extent.width as ::core::ffi::c_int
            - (2 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)) as COORD;
        t.baseline.x = 2 as COORD;
        t.align = TEXT_ALIGN::ALIGN_LEFT;
        t.baseline.y = 8 as COORD;
        SetContextFont(TinyFont);
        row = 0 as ::core::ffi::c_int;
        's_190: while row
            < (ScreenHeight - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                - 13 as ::core::ffi::c_int - 107 as ::core::ffi::c_int
                - 15 as ::core::ffi::c_int) / 8 as ::core::ffi::c_int
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
                    < (ScreenHeight - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                        - 13 as ::core::ffi::c_int - 107 as ::core::ffi::c_int
                        - 15 as ::core::ffi::c_int) / 8 as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int
                    && getLineWithinWidth(
                            &raw mut t,
                            &raw mut next,
                            r.extent.width,
                            !(0 as ::core::ffi::c_int) as COUNT,
                        )
                        .0 == 0
                {
                    font_DrawText(&raw mut t);
                    t.baseline.y = (t.baseline.y as ::core::ffi::c_int
                        + 8 as ::core::ffi::c_int) as COORD;
                    t.pStr = next as *const UNICODE;
                    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
                    row += 1;
                }
                if row
                    >= (ScreenHeight - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                        - 13 as ::core::ffi::c_int - 107 as ::core::ffi::c_int
                        - 15 as ::core::ffi::c_int) / 8 as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int
                {
                    (*pSS).LeftOver = next as *const UNICODE;
                    break 's_190;
                } else {
                    font_DrawText(&raw mut t);
                    t.baseline.y = (t.baseline.y as ::core::ffi::c_int
                        + 8 as ::core::ffi::c_int) as COORD;
                }
            }
            row += 1;
            (*pSS).NextSub = GetNextTrackSubtitle((*pSS).NextSub);
        }
        if row
            >= (ScreenHeight - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                - 13 as ::core::ffi::c_int - 107 as ::core::ffi::c_int
                - 15 as ::core::ffi::c_int) / 8 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int
            && (!(*pSS).NextSub.is_null() || !(*pSS).LeftOver.is_null())
        {
            let mut mt: TEXT = TEXT {
                baseline: POINT { x: 0, y: 0 },
                pStr: ::core::ptr::null::<UNICODE>(),
                align: TEXT_ALIGN::ALIGN_LEFT,
                CharCount: 0,
            };
            let mut buffer: [UNICODE; 80] = [0; 80];
            mt.baseline.x = (ScreenWidth - 64 as ::core::ffi::c_int
                - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                - 14 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as COORD;
            mt.baseline.y = t.baseline.y;
            mt.align = TEXT_ALIGN::ALIGN_CENTER;
            snprintf(
                &raw mut buffer as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[UNICODE; 80]>() as size_t,
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
            mt.pStr = &raw mut buffer as *mut UNICODE;
            SetContextForeGroundColor(
                colorFromRgb15(0 as BYTE, 0x17 as BYTE, 0 as BYTE),
            );
            font_DrawText(&raw mut mt);
        }
        (*pSS).PrintNext = BOOLEAN::FALSE;
    } else {
        SleepThread(
            (840 as ::core::ffi::c_int / 20 as ::core::ffi::c_int) as TimePeriod,
        );
    }
    return BOOLEAN::TRUE;
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
    FeedbackPlayerPhrase(&raw mut (*pES).phrase_buf as *mut UNICODE);
    StopTrack();
    ClearSubtitles();
    SetSliderImage(SetAbsFrameIndex(ActivityFrame, 2 as COUNT));
    FadeMusic(
        (if usingSpeech.0 != 0 {
            160 as ::core::ffi::c_int / 2 as ::core::ffi::c_int
        } else {
            160 as ::core::ffi::c_int
        }) as BYTE,
        840 as SIZE,
    );
    TalkingFinished = BOOLEAN::FALSE;
    (*pES).num_responses = 0 as BYTE;
    Some(
            (*pES)
                .response_list[(*pES).cur_response as usize]
                .response_func
                .expect("non-null function pointer"),
        )
        .expect(
            "non-null function pointer",
        )((*pES).response_list[(*pES).cur_response as usize].response_ref);
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
        FeedbackPlayerPhrase(&raw mut (*pES).phrase_buf as *mut UNICODE);
    }
    SummaryState.Initialized = BOOLEAN::FALSE;
    DoConvSummary(&raw mut SummaryState);
    if !pES.is_null() {
        RefreshResponses(pES);
    }
    clear_subtitles = BOOLEAN::TRUE;
}
unsafe extern "C" fn SelectReplay(mut pES: *mut ENCOUNTER_STATE) {
    FadeMusic(
        (if usingSpeech.0 != 0 {
            160 as ::core::ffi::c_int / 2 as ::core::ffi::c_int
        } else {
            160 as ::core::ffi::c_int
        }) as BYTE,
        840 as SIZE,
    );
    if !pES.is_null() {
        FeedbackPlayerPhrase(&raw mut (*pES).phrase_buf as *mut UNICODE);
    }
    TalkSegue(0 as COUNT);
}
unsafe extern "C" fn PlayerResponseInput(mut pES: *mut ENCOUNTER_STATE) {
    let mut response: BYTE = 0;
    if (*pES).top_response as ::core::ffi::c_int
        == !(0 as ::core::ffi::c_int) as BYTE as ::core::ffi::c_int
    {
        (*pES).top_response = 0 as BYTE;
        RefreshResponses(pES);
    }
    if PulsedInputState
        .menu[C2Rust_Unnamed_2::KEY_MENU_SELECT.0 as ::core::ffi::c_int as usize] != 0
    {
        SelectResponse(pES);
    } else if PulsedInputState
        .menu[C2Rust_Unnamed_2::KEY_MENU_CANCEL.0 as ::core::ffi::c_int as usize] != 0
        && GlobData.Game_state.CurrentActivity as BYTE as ::core::ffi::c_int
            != C2Rust_Unnamed_0::WON_LAST_BATTLE.0 as ::core::ffi::c_int
    {
        SelectConversationSummary(pES);
    } else {
        response = (*pES).cur_response;
        if PulsedInputState
            .menu[C2Rust_Unnamed_2::KEY_MENU_LEFT.0 as ::core::ffi::c_int as usize] != 0
        {
            SelectReplay(pES);
            if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
                & C2Rust_Unnamed_0::CHECK_ABORT.0 as ::core::ffi::c_int == 0
            {
                RefreshResponses(pES);
                FadeMusic(160 as BYTE, 840 as SIZE);
            }
        } else if PulsedInputState
            .menu[C2Rust_Unnamed_2::KEY_MENU_UP.0 as ::core::ffi::c_int as usize] != 0
        {
            response = ((response as ::core::ffi::c_int
                + ((*pES).num_responses as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                    as BYTE as ::core::ffi::c_int)
                % (*pES).num_responses as ::core::ffi::c_int) as BYTE;
        } else if PulsedInputState
            .menu[C2Rust_Unnamed_2::KEY_MENU_DOWN.0 as ::core::ffi::c_int as usize] != 0
        {
            response = ((response as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                as BYTE as ::core::ffi::c_int
                % (*pES).num_responses as ::core::ffi::c_int) as BYTE;
        }
        if response as ::core::ffi::c_int != (*pES).cur_response as ::core::ffi::c_int {
            let mut y: COORD = 0;
            BatchGraphics();
            add_text(
                -2 as ::core::ffi::c_int,
                &raw mut (*(&raw mut (*pES).response_list as *mut RESPONSE_ENTRY)
                    .offset((*pES).cur_response as isize))
                    .response_text,
            );
            (*pES).cur_response = response;
            y = add_text(
                -1 as ::core::ffi::c_int,
                &raw mut (*(&raw mut (*pES).response_list as *mut RESPONSE_ENTRY)
                    .offset((*pES).cur_response as isize))
                    .response_text,
            );
            if (response as ::core::ffi::c_int)
                < (*pES).top_response as ::core::ffi::c_int
            {
                (*pES).top_response = 0 as BYTE;
                RefreshResponses(pES);
            } else if y as ::core::ffi::c_int
                > ScreenHeight - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    - 13 as ::core::ffi::c_int
            {
                (*pES).top_response = response;
                RefreshResponses(pES);
            }
            UnbatchGraphics();
        }
        UpdateCommGraphics();
        SleepThreadUntil((*pES).NextTime);
        (*pES).NextTime = GetTimeCounter()
            .wrapping_add(
                (840 as ::core::ffi::c_int / 40 as ::core::ffi::c_int) as TimeCount,
            );
    };
}
unsafe extern "C" fn DoLastReplay(mut pLRS: *mut LAST_REPLAY_STATE) -> BOOLEAN {
    if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
        & C2Rust_Unnamed_0::CHECK_ABORT.0 as ::core::ffi::c_int != 0
    {
        return BOOLEAN::FALSE;
    }
    if GetTimeCounter() > (*pLRS).TimeOut {
        return BOOLEAN::FALSE;
    }
    if PulsedInputState
        .menu[C2Rust_Unnamed_2::KEY_MENU_CANCEL.0 as ::core::ffi::c_int as usize] != 0
        && GlobData.Game_state.CurrentActivity as BYTE as ::core::ffi::c_int
            != C2Rust_Unnamed_0::WON_LAST_BATTLE.0 as ::core::ffi::c_int
    {
        FadeMusic(
            (if usingSpeech.0 != 0 {
                160 as ::core::ffi::c_int / 2 as ::core::ffi::c_int
            } else {
                160 as ::core::ffi::c_int
            }) as BYTE,
            840 as SIZE,
        );
        SelectConversationSummary(::core::ptr::null_mut::<ENCOUNTER_STATE>());
        (*pLRS).TimeOut = FadeMusic(
                0 as BYTE,
                (840 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as SIZE,
            )
            .wrapping_add(
                (840 as ::core::ffi::c_int / 60 as ::core::ffi::c_int) as DWORD,
            ) as TimeCount;
    } else if PulsedInputState
        .menu[C2Rust_Unnamed_2::KEY_MENU_LEFT.0 as ::core::ffi::c_int as usize] != 0
    {
        SelectReplay(::core::ptr::null_mut::<ENCOUNTER_STATE>());
        (*pLRS).TimeOut = FadeMusic(
                0 as BYTE,
                (840 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as SIZE,
            )
            .wrapping_add(
                (840 as ::core::ffi::c_int / 60 as ::core::ffi::c_int) as DWORD,
            ) as TimeCount;
    }
    UpdateCommGraphics();
    SleepThreadUntil((*pLRS).NextTime);
    (*pLRS).NextTime = GetTimeCounter()
        .wrapping_add(
            (840 as ::core::ffi::c_int / 40 as ::core::ffi::c_int) as TimeCount,
        );
    return BOOLEAN::TRUE;
}
unsafe extern "C" fn DoCommunication(mut pES: *mut ENCOUNTER_STATE) -> BOOLEAN {
    SetMenuSounds(
        (((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as MENU_SOUND_FLAGS
            as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as MENU_SOUND_FLAGS
                as ::core::ffi::c_int) as MENU_SOUND_FLAGS,
        ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as MENU_SOUND_FLAGS,
    );
    if TalkingFinished.0 == 0 {
        AlienTalkSegue(!(0 as ::core::ffi::c_int) as COUNT);
        return BOOLEAN::TRUE;
    }
    if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
        & C2Rust_Unnamed_0::CHECK_ABORT.0 as ::core::ffi::c_int == 0
    {
        if (*pES).num_responses as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
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
            replayState.TimeOut = FadeMusic(
                    0 as BYTE,
                    (840 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as SIZE,
                )
                .wrapping_add(
                    (840 as ::core::ffi::c_int / 60 as ::core::ffi::c_int) as DWORD,
                ) as TimeCount;
            replayState.InputFunc = Some(
                DoLastReplay as unsafe extern "C" fn(*mut LAST_REPLAY_STATE) -> BOOLEAN,
            ) as Option<unsafe extern "C" fn(*mut last_replay_state) -> BOOLEAN>;
            DoInput(&raw mut replayState as *mut ::core::ffi::c_void, BOOLEAN::FALSE);
        } else {
            PlayerResponseInput(pES);
            return BOOLEAN::TRUE;
        }
    }
    SetContext(SpaceContext);
    DestroyContext(AnimContext);
    AnimContext = ::core::ptr::null_mut::<CONTEXT_DESC>();
    FlushColorXForms();
    ClearSubtitles();
    StopMusic();
    StopSound();
    StopTrack();
    SleepThreadUntil(
        FadeMusic(160 as BYTE, 0 as SIZE)
            .wrapping_add(
                (840 as ::core::ffi::c_int / 60 as ::core::ffi::c_int) as TimeCount,
            ),
    );
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
        & C2Rust_Unnamed_0::CHECK_ABORT.0 as ::core::ffi::c_int != 0
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
        (*pEntry).response_text.CharCount = GetStringLength(locString);
    }
    (*pEntry).response_func = response_func;
    (*pES).num_responses = (*pES).num_responses.wrapping_add(1);
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
        }; 8],
        phrase_buf: [0; 1024],
    };
    let mut PlayerFont: FONT = ::core::ptr::null_mut::<FONT_DESC>();
    let mut OldFont: FONT = ::core::ptr::null_mut::<FONT_DESC>();
    let mut SongRef: MUSIC_REF = ::core::ptr::null_mut::<*mut TFB_SoundSample>();
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
    PlayerFont = LoadGraphicInstance(b"font.player\0".as_ptr() as RESOURCE) as FONT;
    CommData.AlienFrame = CaptureDrawable(
        LoadGraphicInstance(CommData.AlienFrameRes) as DRAWABLE,
    );
    CommData.AlienFont = LoadGraphicInstance(CommData.AlienFontRes) as FONT;
    CommData.AlienColorMap = CaptureStringTable(
        LoadStringTableInstance(CommData.AlienColorMapRes),
    ) as COLORMAP;
    if CommData.AlienSongFlags
        & ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as LDAS_FLAGS != 0
        && !CommData.AlienAltSongRes.is_null()
    {
        SongRef = LoadMusicInstance(CommData.AlienAltSongRes);
    }
    if !SongRef.is_null() {
        CommData.AlienSong = SongRef;
    } else {
        CommData.AlienSong = LoadMusicInstance(CommData.AlienSongRes);
    }
    CommData.ConversationPhrases = CaptureStringTable(
        LoadStringTableInstance(CommData.ConversationPhrasesRes),
    );
    SubtitleText.baseline = CommData.AlienTextBaseline;
    SubtitleText.align = CommData.AlienTextAlign;
    TextCacheContext = CreateContextAux(
        b"TextCacheContext\0".as_ptr() as *const ::core::ffi::c_char,
    );
    TextCacheFrame = CaptureDrawable(
        CreateDrawable(
            ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as CREATE_FLAGS,
            (ScreenWidth - 64 as ::core::ffi::c_int
                - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                - 14 as ::core::ffi::c_int) as SIZE,
            (ScreenHeight - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                - 13 as ::core::ffi::c_int - 107 as ::core::ffi::c_int
                - 15 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as SIZE,
            1 as COUNT,
        ),
    );
    SetContext(TextCacheContext);
    SetContextFGFrame(TextCacheFrame);
    TextBack = colorFromRgb15(0 as BYTE, 0 as BYTE, 0x10 as BYTE);
    SetContextBackGroundColor(TextBack);
    ClearDrawable();
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
    r.extent.width = (ScreenWidth - 64 as ::core::ffi::c_int
        - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int - 14 as ::core::ffi::c_int)
        as COORD;
    CommWndRect.extent = r.extent;
    SetTransitionSource(::core::ptr::null::<RECT>());
    BatchGraphics();
    if GlobData.Game_state.CurrentActivity as BYTE as ::core::ffi::c_int
        == C2Rust_Unnamed_0::WON_LAST_BATTLE.0 as ::core::ffi::c_int
    {
        r.corner = CommWndRect.corner;
        SetContextClipRect(&raw mut r);
    } else {
        r.corner.x = (7 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as COORD;
        r.corner.y = (10 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as COORD;
        SetContextClipRect(&raw mut r);
        CommWndRect.corner = r.corner;
        DrawSISFrame();
        if getGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed::GLOBAL_FLAGS_AND_DATA.0 as ::core::ffi::c_int,
            C2Rust_Unnamed::END_GLOBAL_FLAGS_AND_DATA.0 as ::core::ffi::c_int,
        ) as ::core::ffi::c_int
            == !(0 as ::core::ffi::c_int) as BYTE as ::core::ffi::c_int
            && getGameState(
                &raw mut GlobData.Game_state.GameState as *mut BYTE,
                C2Rust_Unnamed::STARBASE_AVAILABLE.0 as ::core::ffi::c_int,
                C2Rust_Unnamed::END_STARBASE_AVAILABLE.0 as ::core::ffi::c_int,
            ) as ::core::ffi::c_int != 0
        {
            DrawSISMessage(
                GetStringAddress(
                    SetAbsStringTableIndex(
                        GameStrings,
                        (C2Rust_Unnamed_4::STARBASE_STRING_BASE.0 as ::core::ffi::c_int
                            + 1 as ::core::ffi::c_int) as COUNT,
                    ),
                ),
            );
            DrawSISTitle(
                GetStringAddress(
                    SetAbsStringTableIndex(
                        GameStrings,
                        (C2Rust_Unnamed_4::STARBASE_STRING_BASE.0 as ::core::ffi::c_int
                            + 0 as ::core::ffi::c_int) as COUNT,
                    ),
                ),
            );
        } else {
            DrawSISMessage(::core::ptr::null::<UNICODE>());
            DrawSISTitle(&raw mut GlobData.SIS_state.PlanetName as *mut UNICODE);
        }
    }
    DrawSISComWindow();
    LastActivity = (LastActivity as ::core::ffi::c_int
        | C2Rust_Unnamed_0::CHECK_LOAD.0 as ::core::ffi::c_int) as ACTIVITY;
    Some(CommData.init_encounter_func.expect("non-null function pointer"))
        .expect("non-null function pointer")();
    DoInput(&raw mut ES as *mut ::core::ffi::c_void, BOOLEAN::FALSE);
    if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
        & (C2Rust_Unnamed_0::CHECK_ABORT.0 as ::core::ffi::c_int
            | C2Rust_Unnamed_0::CHECK_LOAD.0 as ::core::ffi::c_int) == 0
    {
        Some(CommData.post_encounter_func.expect("non-null function pointer"))
            .expect("non-null function pointer")();
    }
    Some(CommData.uninit_encounter_func.expect("non-null function pointer"))
        .expect("non-null function pointer")();
    SetContext(SpaceContext);
    SetContextFont(OldFont);
    DestroyStringTable(ReleaseStringTable(CommData.ConversationPhrases));
    DestroyMusic(CommData.AlienSong);
    DestroyStringTable(ReleaseStringTable(CommData.AlienColorMap));
    DestroyFont(CommData.AlienFont);
    DestroyDrawable(ReleaseDrawable(CommData.AlienFrame));
    DestroyContext(TextCacheContext);
    DestroyDrawable(ReleaseDrawable(TextCacheFrame));
    DestroyFont(PlayerFont);
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
        & C2Rust_Unnamed_0::CHECK_LOAD.0 as ::core::ffi::c_int != 0
    {
        LastActivity = (LastActivity as ::core::ffi::c_int
            & !(C2Rust_Unnamed_0::CHECK_LOAD.0 as ::core::ffi::c_int)) as ACTIVITY;
        if which_comm.0 != CONVERSATION::COMMANDER_CONVERSATION.0 {
            if LastActivity as BYTE as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                DrawSISFrame();
            } else {
                ClearSISRect(
                    ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as BYTE,
                );
                RepairSISBorder();
            }
            DrawSISMessage(::core::ptr::null::<UNICODE>());
            if inHQSpace().0 != 0 {
                DrawHyperCoords(GlobData.Game_state.ShipStamp.origin);
            } else if GlobData.Game_state.ip_planet as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
            {
                DrawHyperCoords((*CurStarDescPtr).star_pt);
            } else {
                DrawSISTitle(&raw mut GlobData.SIS_state.PlanetName as *mut UNICODE);
            }
        }
    }
    if which_comm.0 == CONVERSATION::URQUAN_DRONE_CONVERSATION.0 {
        status = C2Rust_Unnamed_1::URQUAN_DRONE_SHIP.0 as ::core::ffi::c_int as COUNT;
        which_comm = CONVERSATION::URQUAN_CONVERSATION;
    } else {
        if which_comm.0 == CONVERSATION::YEHAT_REBEL_CONVERSATION.0 {
            status = C2Rust_Unnamed_1::YEHAT_REBEL_SHIP.0 as ::core::ffi::c_int as COUNT;
            which_comm = CONVERSATION::YEHAT_CONVERSATION;
        } else {
            let mut commToShip: [COUNT; 27] = [
                C2Rust_Unnamed_1::ARILOU_SHIP.0 as ::core::ffi::c_int as COUNT,
                C2Rust_Unnamed_1::CHMMR_SHIP.0 as ::core::ffi::c_int as COUNT,
                C2Rust_Unnamed_1::HUMAN_SHIP.0 as ::core::ffi::c_int as COUNT,
                C2Rust_Unnamed_1::ORZ_SHIP.0 as ::core::ffi::c_int as COUNT,
                C2Rust_Unnamed_1::PKUNK_SHIP.0 as ::core::ffi::c_int as COUNT,
                C2Rust_Unnamed_1::SHOFIXTI_SHIP.0 as ::core::ffi::c_int as COUNT,
                C2Rust_Unnamed_1::SPATHI_SHIP.0 as ::core::ffi::c_int as COUNT,
                C2Rust_Unnamed_1::SUPOX_SHIP.0 as ::core::ffi::c_int as COUNT,
                C2Rust_Unnamed_1::THRADDASH_SHIP.0 as ::core::ffi::c_int as COUNT,
                C2Rust_Unnamed_1::UTWIG_SHIP.0 as ::core::ffi::c_int as COUNT,
                C2Rust_Unnamed_1::VUX_SHIP.0 as ::core::ffi::c_int as COUNT,
                C2Rust_Unnamed_1::YEHAT_SHIP.0 as ::core::ffi::c_int as COUNT,
                C2Rust_Unnamed_1::MELNORME_SHIP.0 as ::core::ffi::c_int as COUNT,
                C2Rust_Unnamed_1::DRUUGE_SHIP.0 as ::core::ffi::c_int as COUNT,
                C2Rust_Unnamed_1::ILWRATH_SHIP.0 as ::core::ffi::c_int as COUNT,
                C2Rust_Unnamed_1::MYCON_SHIP.0 as ::core::ffi::c_int as COUNT,
                C2Rust_Unnamed_1::SLYLANDRO_SHIP.0 as ::core::ffi::c_int as COUNT,
                C2Rust_Unnamed_1::UMGAH_SHIP.0 as ::core::ffi::c_int as COUNT,
                C2Rust_Unnamed_1::URQUAN_SHIP.0 as ::core::ffi::c_int as COUNT,
                C2Rust_Unnamed_1::ZOQFOTPIK_SHIP.0 as ::core::ffi::c_int as COUNT,
                C2Rust_Unnamed_1::SYREEN_SHIP.0 as ::core::ffi::c_int as COUNT,
                C2Rust_Unnamed_1::BLACK_URQUAN_SHIP.0 as ::core::ffi::c_int as COUNT,
                C2Rust_Unnamed_1::UMGAH_SHIP.0 as ::core::ffi::c_int as COUNT,
                C2Rust_Unnamed_1::SLYLANDRO_SHIP.0 as ::core::ffi::c_int as COUNT,
                C2Rust_Unnamed_1::URQUAN_DRONE_SHIP.0 as ::core::ffi::c_int as COUNT,
                C2Rust_Unnamed_1::YEHAT_SHIP.0 as ::core::ffi::c_int as COUNT,
                C2Rust_Unnamed_1::HUMAN_SHIP.0 as ::core::ffi::c_int as COUNT,
            ];
            status = commToShip[which_comm.0 as usize];
            if status as ::core::ffi::c_int
                >= C2Rust_Unnamed_1::YEHAT_REBEL_SHIP.0 as ::core::ffi::c_int
            {
                status = C2Rust_Unnamed_1::HUMAN_SHIP.0 as ::core::ffi::c_int as COUNT;
            }
        }
        StartSphereTracking(status);
        if which_comm.0 == CONVERSATION::ORZ_CONVERSATION.0
            || which_comm.0 == CONVERSATION::TALKING_PET_CONVERSATION.0
                && (getGameState(
                    &raw mut GlobData.Game_state.GameState as *mut BYTE,
                    C2Rust_Unnamed::TALKING_PET_ON_SHIP.0 as ::core::ffi::c_int,
                    C2Rust_Unnamed::END_TALKING_PET_ON_SHIP.0 as ::core::ffi::c_int,
                ) == 0
                    || GlobData.Game_state.CurrentActivity as BYTE as ::core::ffi::c_int
                        == C2Rust_Unnamed_0::IN_LAST_BATTLE.0 as ::core::ffi::c_int)
            || which_comm.0 != CONVERSATION::CHMMR_CONVERSATION.0
                && which_comm.0 != CONVERSATION::SYREEN_CONVERSATION.0
        {
            BuildBattle(1 as COUNT);
        }
    }
    LocDataPtr = init_race(
        CONVERSATION(
            if status as ::core::ffi::c_int
                != C2Rust_Unnamed_1::YEHAT_REBEL_SHIP.0 as ::core::ffi::c_int
            {
                which_comm.0
            } else {
                CONVERSATION::YEHAT_REBEL_CONVERSATION.0
            },
        ),
    );
    if !LocDataPtr.is_null() {
        CommData = *LocDataPtr;
    }
    if getGameState(
        &raw mut GlobData.Game_state.GameState as *mut BYTE,
        C2Rust_Unnamed::BATTLE_SEGUE.0 as ::core::ffi::c_int,
        C2Rust_Unnamed::END_BATTLE_SEGUE.0 as ::core::ffi::c_int,
    ) as ::core::ffi::c_int == 0 as ::core::ffi::c_int
    {
        status = C2Rust_Unnamed_3::HAIL.0 as ::core::ffi::c_int as COUNT;
    } else {
        status = InitEncounter();
        if status as ::core::ffi::c_int == C2Rust_Unnamed_3::HAIL.0 as ::core::ffi::c_int
            && !LocDataPtr.is_null()
        {
            setGameState(
                &raw mut GlobData.Game_state.GameState as *mut BYTE,
                C2Rust_Unnamed::BATTLE_SEGUE.0 as ::core::ffi::c_int,
                C2Rust_Unnamed::END_BATTLE_SEGUE.0 as ::core::ffi::c_int,
                0 as BYTE,
            );
        } else {
            status = C2Rust_Unnamed_3::ATTACK.0 as ::core::ffi::c_int as COUNT;
            setGameState(
                &raw mut GlobData.Game_state.GameState as *mut BYTE,
                C2Rust_Unnamed::BATTLE_SEGUE.0 as ::core::ffi::c_int,
                C2Rust_Unnamed::END_BATTLE_SEGUE.0 as ::core::ffi::c_int,
                1 as BYTE,
            );
        }
    }
    if status as ::core::ffi::c_int == C2Rust_Unnamed_3::HAIL.0 as ::core::ffi::c_int {
        HailAlien();
    } else if !LocDataPtr.is_null() {
        if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
            & (C2Rust_Unnamed_0::CHECK_ABORT.0 as ::core::ffi::c_int
                | C2Rust_Unnamed_0::CHECK_LOAD.0 as ::core::ffi::c_int) == 0
        {
            Some(CommData.post_encounter_func.expect("non-null function pointer"))
                .expect("non-null function pointer")();
        }
        Some(CommData.uninit_encounter_func.expect("non-null function pointer"))
            .expect("non-null function pointer")();
    }
    status = 0 as COUNT;
    if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
        & (C2Rust_Unnamed_0::CHECK_ABORT.0 as ::core::ffi::c_int
            | C2Rust_Unnamed_0::CHECK_LOAD.0 as ::core::ffi::c_int) == 0
    {
        if GlobData.Game_state.CurrentActivity as BYTE as ::core::ffi::c_int
            == C2Rust_Unnamed_0::IN_LAST_BATTLE.0 as ::core::ffi::c_int
            && GlobData.Game_state.glob_flags as ::core::ffi::c_int
                & (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int != 0
        {
            ReinitQueue(&raw mut GlobData.Game_state.npc_built_ship_q);
        }
        setGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed::GLOBAL_FLAGS_AND_DATA.0 as ::core::ffi::c_int,
            C2Rust_Unnamed::END_GLOBAL_FLAGS_AND_DATA.0 as ::core::ffi::c_int,
            0 as BYTE,
        );
        status = (getGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed::BATTLE_SEGUE.0 as ::core::ffi::c_int,
            C2Rust_Unnamed::END_BATTLE_SEGUE.0 as ::core::ffi::c_int,
        ) as ::core::ffi::c_int != 0
            && !GlobData.Game_state.npc_built_ship_q.head.is_null())
            as ::core::ffi::c_int as COUNT;
        if status != 0 {
            BuildBattle(0 as COUNT);
            EncounterBattle();
        } else {
            setGameState(
                &raw mut GlobData.Game_state.GameState as *mut BYTE,
                C2Rust_Unnamed::BATTLE_SEGUE.0 as ::core::ffi::c_int,
                C2Rust_Unnamed::END_BATTLE_SEGUE.0 as ::core::ffi::c_int,
                0 as BYTE,
            );
        }
    }
    UninitEncounter();
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn RaceCommunication() {
    let mut i: COUNT = 0;
    let mut status: COUNT = 0;
    let mut hStarShip: HSHIPFRAG = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut FragPtr: *mut SHIP_FRAGMENT = ::core::ptr::null_mut::<SHIP_FRAGMENT>();
    let mut hEncounter: HENCOUNTER = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut RaceComm: [CONVERSATION; 24] = [
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
        CONVERSATION::YEHAT_REBEL_CONVERSATION,
        CONVERSATION::URQUAN_DRONE_CONVERSATION,
    ];
    if GlobData.Game_state.CurrentActivity as BYTE as ::core::ffi::c_int
        == C2Rust_Unnamed_0::IN_LAST_BATTLE.0 as ::core::ffi::c_int
    {
        ReinitQueue(&raw mut GlobData.Game_state.npc_built_ship_q);
        CloneShipFragment(
            C2Rust_Unnamed_1::SAMATRA_SHIP.0 as ::core::ffi::c_int as COUNT,
            &raw mut GlobData.Game_state.npc_built_ship_q,
            0 as COUNT,
        );
        InitCommunication(CONVERSATION::TALKING_PET_CONVERSATION);
        if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
            & (C2Rust_Unnamed_0::CHECK_ABORT.0 as ::core::ffi::c_int
                | C2Rust_Unnamed_0::CHECK_LOAD.0 as ::core::ffi::c_int) == 0
            && GlobData.SIS_state.CrewEnlisted as ::core::ffi::c_int
                != !(0 as ::core::ffi::c_int) as COUNT as ::core::ffi::c_int
        {
            GlobData.Game_state.CurrentActivity = C2Rust_Unnamed_0::WON_LAST_BATTLE.0
                as ::core::ffi::c_int as ACTIVITY;
        }
        return;
    } else if NextActivity as ::core::ffi::c_int
        & C2Rust_Unnamed_0::CHECK_LOAD.0 as ::core::ffi::c_int != 0
    {
        let mut ec: BYTE = 0;
        ec = getGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed::ESCAPE_COUNTER.0 as ::core::ffi::c_int,
            C2Rust_Unnamed::END_ESCAPE_COUNTER.0 as ::core::ffi::c_int,
        );
        if getGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed::FOUND_PLUTO_SPATHI.0 as ::core::ffi::c_int,
            C2Rust_Unnamed::END_FOUND_PLUTO_SPATHI.0 as ::core::ffi::c_int,
        ) as ::core::ffi::c_int == 1 as ::core::ffi::c_int
        {
            InitCommunication(CONVERSATION::SPATHI_CONVERSATION);
        } else if getGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed::GLOBAL_FLAGS_AND_DATA.0 as ::core::ffi::c_int,
            C2Rust_Unnamed::END_GLOBAL_FLAGS_AND_DATA.0 as ::core::ffi::c_int,
        ) as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        {
            InitCommunication(CONVERSATION::TALKING_PET_CONVERSATION);
        } else if getGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed::GLOBAL_FLAGS_AND_DATA.0 as ::core::ffi::c_int,
            C2Rust_Unnamed::END_GLOBAL_FLAGS_AND_DATA.0 as ::core::ffi::c_int,
        ) as ::core::ffi::c_int
            & ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) != 0
        {
            InitCommunication(CONVERSATION::ILWRATH_CONVERSATION);
        } else {
            InitCommunication(CONVERSATION::CHMMR_CONVERSATION);
        }
        if GlobData.SIS_state.CrewEnlisted as ::core::ffi::c_int
            != !(0 as ::core::ffi::c_int) as COUNT as ::core::ffi::c_int
        {
            NextActivity = (GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
                & !(C2Rust_Unnamed_0::START_ENCOUNTER.0 as ::core::ffi::c_int))
                as ACTIVITY;
            if NextActivity as BYTE as ::core::ffi::c_int
                == C2Rust_Unnamed_0::IN_INTERPLANETARY.0 as ::core::ffi::c_int
            {
                NextActivity = (NextActivity as ::core::ffi::c_int
                    | C2Rust_Unnamed_0::START_INTERPLANETARY.0 as ::core::ffi::c_int)
                    as ACTIVITY;
            }
            GlobData.Game_state.CurrentActivity = (GlobData.Game_state.CurrentActivity
                as ::core::ffi::c_int
                | C2Rust_Unnamed_0::CHECK_LOAD.0 as ::core::ffi::c_int) as ACTIVITY;
        }
        setGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed::ESCAPE_COUNTER.0 as ::core::ffi::c_int,
            C2Rust_Unnamed::END_ESCAPE_COUNTER.0 as ::core::ffi::c_int,
            ec,
        );
        return;
    } else if inHQSpace().0 != 0 {
        ReinitQueue(&raw mut GlobData.Game_state.npc_built_ship_q);
        if getGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed::ARILOU_SPACE_SIDE.0 as ::core::ffi::c_int,
            C2Rust_Unnamed::END_ARILOU_SPACE_SIDE.0 as ::core::ffi::c_int,
        ) as ::core::ffi::c_int >= 2 as ::core::ffi::c_int
        {
            InitCommunication(CONVERSATION::ARILOU_CONVERSATION);
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
                    (*EncounterPtr).race_id as COUNT,
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
    status = InitCommunication(RaceComm[i as usize]);
    if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
        & (C2Rust_Unnamed_0::CHECK_ABORT.0 as ::core::ffi::c_int
            | C2Rust_Unnamed_0::CHECK_LOAD.0 as ::core::ffi::c_int) != 0
    {
        return;
    }
    if i as ::core::ffi::c_int == C2Rust_Unnamed_1::CHMMR_SHIP.0 as ::core::ffi::c_int {
        ReinitQueue(&raw mut GlobData.Game_state.npc_built_ship_q);
    }
    if GlobData.Game_state.CurrentActivity as BYTE as ::core::ffi::c_int
        == C2Rust_Unnamed_0::IN_INTERPLANETARY.0 as ::core::ffi::c_int
    {
        if i as ::core::ffi::c_int
            == C2Rust_Unnamed_1::SLYLANDRO_SHIP.0 as ::core::ffi::c_int
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
        add_text(1 as ::core::ffi::c_int, &raw mut t);
    }
}
unsafe extern "C" fn ClearSubtitles() {
    clear_subtitles = BOOLEAN::TRUE;
    last_subtitle = ::core::ptr::null::<UNICODE>();
    SubtitleText.pStr = ::core::ptr::null::<UNICODE>();
    SubtitleText.CharCount = 0 as COUNT;
}
unsafe extern "C" fn CheckSubtitles() {
    let mut pStr: *const UNICODE = ::core::ptr::null::<UNICODE>();
    let mut baseline: POINT = POINT { x: 0, y: 0 };
    let mut align: TEXT_ALIGN = TEXT_ALIGN::ALIGN_LEFT;
    pStr = GetTrackSubtitle();
    baseline = CommData.AlienTextBaseline;
    align = CommData.AlienTextAlign;
    if pStr != SubtitleText.pStr
        || SubtitleText.baseline.x as ::core::ffi::c_int
            != baseline.x as ::core::ffi::c_int
        || SubtitleText.baseline.y as ::core::ffi::c_int
            != baseline.y as ::core::ffi::c_int || SubtitleText.align.0 != align.0
    {
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
            & !((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)) as BYTE;
    } else {
        CommData.AlienTalkDesc.AnimFlags = (CommData.AlienTalkDesc.AnimFlags
            as ::core::ffi::c_int | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
            as BYTE;
    };
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
        as BYTE;
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
        as BYTE;
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
        as BYTE;
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
                329 as ::core::ffi::c_int,
                b"LockShipFrag\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    return LockLink(pq, h) as *mut SHIP_FRAGMENT;
}
