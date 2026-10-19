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

// Rust translation of src/uqm/intro.c.
extern "C" {
    pub type uio_DirHandle;
    pub type string_table_entry;
    pub type string_table;
    pub type context_desc;
    pub type font_desc;
    pub type tfb_soundsample;
    pub type legacy_video_desc;
    pub type legacy_video_ref;
    fn __assert_fail(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    ) -> !;
    fn snprintf(
        _: *mut ::core::ffi::c_char,
        _: size_t,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn sscanf(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn LoadStringTableInstance(res: RESOURCE) -> STRING_TABLE;
    fn LoadStringTableFile(
        dir: *mut uio_DirHandle,
        fileName: *const ::core::ffi::c_char,
    ) -> STRING_TABLE;
    fn DestroyStringTable(StringTable: STRING_TABLE) -> BOOLEAN;
    fn CaptureStringTable(StringTable: STRING_TABLE) -> STRING;
    fn ReleaseStringTable(String: STRING) -> STRING_TABLE;
    fn GetStringTableCount(String: STRING) -> COUNT;
    fn SetAbsStringTableIndex(String: STRING, StringTableIndex: COUNT) -> STRING;
    fn SetRelStringTableIndex(String: STRING, StringTableOffs: SIZE) -> STRING;
    fn GetStringAddress(String: STRING) -> STRINGPTR;
    fn utf8StringCount(start: *const ::core::ffi::c_char) -> size_t;
    fn utf8StringCopy(
        dst: *mut ::core::ffi::c_char,
        size: size_t,
        src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_uchar;
    fn res_GetResourceType(res: RESOURCE) -> *const ::core::ffi::c_char;
    fn SetContext(Context: CONTEXT) -> CONTEXT;
    fn SetContextForeGroundColor(Color: Color) -> Color;
    fn SetContextBackGroundColor(Color: Color) -> Color;
    fn SetContextFGFrame(Frame: FRAME) -> FRAME;
    fn SetContextClipRect(pRect: *mut RECT) -> BOOLEAN;
    fn GetContextClipRect(pRect: *mut RECT) -> BOOLEAN;
    fn DrawStamp(pStamp: *mut STAMP);
    fn DrawFilledStamp(pStamp: *mut STAMP);
    fn DrawPoint(pPoint: *mut POINT);
    fn DrawFilledRectangle(pRect: *mut RECT);
    fn DrawLine(pLine: *mut LINE, thickness: BYTE);
    fn font_DrawText(pText: *mut TEXT);
    fn font_DrawText_Fade(lpText: *mut TEXT, repair: FRAME, skip: *mut BOOLEAN);
    fn font_DrawTracedText(pText: *mut TEXT, text: Color, trace: Color);
    fn BatchGraphics();
    fn UnbatchGraphics();
    fn FlushGraphics();
    fn ClearDrawable();
    fn ClearScreen();
    fn CreateDrawable(
        CreateFlags: CREATE_FLAGS,
        width: SIZE,
        height: SIZE,
        num_frames: COUNT,
    ) -> DRAWABLE;
    fn DestroyDrawable(Drawable: DRAWABLE) -> BOOLEAN;
    fn GetFrameRect(Frame: FRAME, pRect: *mut RECT) -> BOOLEAN;
    fn SetFrameHot(Frame: FRAME, HotSpot: HOT_SPOT) -> HOT_SPOT;
    fn LoadGraphicFile(pStr: *const ::core::ffi::c_char) -> DRAWABLE;
    fn LoadFontFile(pStr: *const ::core::ffi::c_char) -> FONT;
    fn LoadGraphicInstance(res: RESOURCE) -> *mut ::core::ffi::c_void;
    fn SetContextFont(Font: FONT) -> FONT;
    fn DestroyFont(FontRef: FONT) -> BOOLEAN;
    fn GetContextFontLeading(pheight: *mut SIZE) -> BOOLEAN;
    fn SetAbsFrameIndex(Frame: FRAME, FrameIndex: COUNT) -> FRAME;
    fn RotateFrame(Frame: FRAME, angle_deg: ::core::ffi::c_int) -> DRAWABLE;
    fn CaptureDrawable(Drawable: DRAWABLE) -> FRAME;
    fn ReleaseDrawable(Frame: FRAME) -> DRAWABLE;
    fn FadeScreen(fadeType: ScreenFadeType, TimeInterval: SIZE) -> DWORD;
    fn SleepThread(timePeriod: TimePeriod);
    fn SleepThreadUntil(wakeTime: TimeCount);
    fn TaskSwitch();
    fn GetTimeCounter() -> TimeCount;
    fn memset(
        _: *mut ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strcmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strncmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: size_t,
    ) -> ::core::ffi::c_int;
    fn strchr(
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(_: *const ::core::ffi::c_char) -> size_t;
    fn TFB_DrawScreen_Copy(r: *const RECT, src: SCREEN, dest: SCREEN);
    fn strcasecmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn toupper(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    static mut CurrentInputState: CONTROLLER_INPUT_STATE;
    static mut PulsedInputState: CONTROLLER_INPUT_STATE;
    fn UpdateInputState();
    fn DoInput(pInputState: *mut ::core::ffi::c_void, resetInput: BOOLEAN);
    static mut resolutionFactor: ::core::ffi::c_uint;
    static mut optShipSeed: OPT_ENABLABLE;
    static mut optFlagshipColor: ::core::ffi::c_int;
    static mut optWindowType: DWORD;
    static mut contentDir: *mut uio_DirHandle;
    fn LoadMusicFile(pStr: *const ::core::ffi::c_char) -> MUSIC_REF;
    fn LoadMusicInstance(res: RESOURCE) -> MUSIC_REF;
    fn DestroyMusic(MusicRef: MUSIC_REF) -> BOOLEAN;
    fn FadeMusic(end_vol: BYTE, TimeInterval: SIZE) -> DWORD;
    fn StopMusic();
    fn PlayMusic(MusicRef: MUSIC_REF, Continuous: BOOLEAN, Priority: BYTE);
    static mut CanvasWidth: ::core::ffi::c_int;
    static mut CanvasHeight: ::core::ffi::c_int;
    fn strupr(_: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    static mut GlobData: GLOBDATA;
    fn is3DO(optWhich: ::core::ffi::c_int) -> OPT_CONSOLETYPE;
    static mut OffScreenContext: CONTEXT;
    static mut ScreenContext: CONTEXT;
    fn SetMenuSounds(sound_0: MENU_SOUND_FLAGS, sound_1: MENU_SOUND_FLAGS);
    fn PlayMenuSound(S: MENU_SOUND_EFFECT);
    fn SetGraphicScale(scale: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn SetGraphicScaleMode(mode: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn SetTransitionSource(pRect: *const RECT);
    fn ScreenTransition(transition: ::core::ffi::c_int, pRect: *const RECT);
    fn PlayingStream(source: uint32) -> BOOLEAN;
    fn VidProcessFrame() -> BOOLEAN;
    fn VidGetPosition() -> DWORD;
    fn VidSeek(pos: DWORD) -> BOOLEAN;
    fn LoadLegacyVideoInstance(res: RESOURCE) -> LEGACY_VIDEO;
    fn DestroyLegacyVideo(vid: LEGACY_VIDEO) -> BOOLEAN;
    fn PlayLegacyVideo(vid: LEGACY_VIDEO) -> LEGACY_VIDEO_REF;
    fn StopLegacyVideo(r#ref: LEGACY_VIDEO_REF);
    fn PlayingLegacyVideo(r#ref: LEGACY_VIDEO_REF) -> BOOLEAN;
    fn log_add(_: log_Level, fmt: *const ::core::ffi::c_char, ...);
    fn ActKeysPress() -> BOOLEAN;
    fn SeedShip(SpeciesID: SPECIES_ID, loadWindow: BOOLEAN) -> SPECIES_ID;
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
pub type STRING_TABLE_ENTRY_DESC = string_table_entry;
pub type STRING_TABLE_DESC = string_table;
pub type STRING_TABLE = *mut STRING_TABLE_DESC;
pub type STRING = *mut STRING_TABLE_ENTRY_DESC;
pub type STRINGPTR = *mut ::core::ffi::c_char;
pub type RESOURCE = *const ::core::ffi::c_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Color {
    pub r: BYTE,
    pub g: BYTE,
    pub b: BYTE,
    pub a: BYTE,
}
pub type CONTEXT_DESC = context_desc;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct frame_desc {
    pub Type: DRAWABLE_TYPE,
    pub Index: UWORD,
    pub HotSpot: HOT_SPOT,
    pub Bounds: EXTENT,
    pub image: *mut TFB_Image,
    pub parent: *mut drawable_desc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct drawable_desc {
    pub Flags: CREATE_FLAGS,
    pub MaxIndex: UWORD,
    pub Frame: *mut FRAME_DESC,
}
pub type FRAME_DESC = frame_desc;
pub type CREATE_FLAGS = BYTE;
pub type TFB_Image = tfb_image;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tfb_image {
    pub NormalImg: TFB_Canvas,
    pub ScaledImg: TFB_Canvas,
    pub MipmapImg: TFB_Canvas,
    pub FilledImg: TFB_Canvas,
    pub colormap_index: ::core::ffi::c_int,
    pub colormap_version: ::core::ffi::c_int,
    pub NormalHs: HOT_SPOT,
    pub MipmapHs: HOT_SPOT,
    pub last_scale_hs: HOT_SPOT,
    pub last_scale: ::core::ffi::c_int,
    pub last_scale_type: ::core::ffi::c_int,
    pub last_fill: Color,
    pub extent: EXTENT,
    pub mutex: Mutex,
    pub dirty: BOOLEAN,
}
pub type Mutex = *mut ::core::ffi::c_void;
pub type EXTENT = extent;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct extent {
    pub width: COORD,
    pub height: COORD,
}
pub type COORD = SWORD;
pub type HOT_SPOT = POINT;
pub type POINT = point;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct point {
    pub x: COORD,
    pub y: COORD,
}
pub type TFB_Canvas = *mut ::core::ffi::c_void;
pub type DRAWABLE_TYPE = UWORD;
pub type FONT_DESC = font_desc;
pub type DRAWABLE_DESC = drawable_desc;
pub type CONTEXT = *mut CONTEXT_DESC;
pub type FRAME = *mut FRAME_DESC;
pub type FONT = *mut FONT_DESC;
pub type DRAWABLE = *mut DRAWABLE_DESC;
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
pub type TFB_SoundSample = tfb_soundsample;
pub type MUSIC_REF = *mut *mut TFB_SoundSample;
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
pub type MENU_SOUND_FLAGS = UWORD;
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
pub type LEGACY_VIDEO = *mut legacy_video_desc;
pub type LEGACY_VIDEO_REF = *mut legacy_video_ref;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VIDEO_INPUT_STATE {
    pub InputFunc: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> BOOLEAN>,
    pub CurVideo: LEGACY_VIDEO_REF,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PRESENTATION_INPUT_STATE {
    pub InputFunc: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> BOOLEAN>,
    pub StartTime: TimeCount,
    pub LastSyncTime: TimeCount,
    pub TimeOut: TimeCount,
    pub TimeOutOnSkip: ::core::ffi::c_int,
    pub SlideShow: STRING,
    pub Fonts: [FONT; 5],
    pub Frame: FRAME,
    pub MusicRef: MUSIC_REF,
    pub Batched: BOOLEAN,
    pub SisFrame: FRAME,
    pub RotatedFrame: FRAME,
    pub LastDrawKind: ::core::ffi::c_int,
    pub LastAngle: ::core::ffi::c_int,
    pub OperIndex: COUNT,
    pub TextFadeColor: Color,
    pub TextColor: Color,
    pub TextBackColor: Color,
    pub TextVPos: ::core::ffi::c_int,
    pub TextEffect: ::core::ffi::c_int,
    pub clip_r: RECT,
    pub tfade_r: RECT,
    pub TextLines: [TEXT; 15],
    pub LinesCount: COUNT,
    pub Buffer: [::core::ffi::c_char; 512],
    pub MovieFrame: ::core::ffi::c_int,
    pub MovieEndFrame: ::core::ffi::c_int,
    pub InterframeDelay: ::core::ffi::c_int,
    pub StatBox: RECT,
    pub NumSpinStat: COUNT,
    pub GetRect: RECT,
    pub CurrentFrameIndex: COUNT,
    pub HaveFrame: BOOLEAN,
    pub Skip: BOOLEAN,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SHIPMAP {
    pub sID: SPECIES_ID,
    pub ditty: [::core::ffi::c_char; 256],
    pub race: [::core::ffi::c_char; 256],
    pub ship: [::core::ffi::c_char; 256],
    pub spinline: COUNT,
    pub width: COUNT,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct SCREEN(pub ::core::ffi::c_uint);
impl SCREEN {
    pub const TFB_SCREEN_MAIN: Self = Self(0);
    pub const TFB_SCREEN_EXTRA: Self = Self(1);
    pub const TFB_SCREEN_TRANSITION: Self = Self(2);
    pub const TFB_GFX_NUMSCREENS: Self = Self(3);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_2(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_2 {
    pub const TFB_SCALE_STEP: Self = Self(0);
    pub const TFB_SCALE_NEAREST: Self = Self(1);
    pub const TFB_SCALE_BILINEAR: Self = Self(2);
    pub const TFB_SCALE_TRILINEAR: Self = Self(3);
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
unsafe extern "C" fn ParseColorString(
    mut Src: *const ::core::ffi::c_char,
    mut pColor: *mut Color,
) -> BOOLEAN {
    let mut clr: ::core::ffi::c_uint = 0;
    if 1 as ::core::ffi::c_int
        != sscanf(Src, b"%x\0".as_ptr() as *const ::core::ffi::c_char, &raw mut clr)
    {
        return BOOLEAN::FALSE;
    }
    *pColor = buildColorRgba(
        (clr >> 16 as ::core::ffi::c_int & 0xff as ::core::ffi::c_uint) as BYTE,
        (clr >> 8 as ::core::ffi::c_int & 0xff as ::core::ffi::c_uint) as BYTE,
        (clr & 0xff as ::core::ffi::c_uint) as BYTE,
        0xff as BYTE,
    );
    return BOOLEAN::TRUE;
}
unsafe extern "C" fn DoFadeScreen(
    mut pPIS: *mut PRESENTATION_INPUT_STATE,
    mut Src: *const ::core::ffi::c_char,
    mut FadeType: BYTE,
) -> BOOLEAN {
    let mut msecs: ::core::ffi::c_int = 0;
    if 1 as ::core::ffi::c_int
        == sscanf(Src, b"%d\0".as_ptr() as *const ::core::ffi::c_char, &raw mut msecs)
    {
        (*pPIS).TimeOut = FadeScreen(
                ScreenFadeType(FadeType as ::core::ffi::c_uint),
                (msecs * 840 as ::core::ffi::c_int / 1000 as ::core::ffi::c_int) as SIZE,
            )
            .wrapping_add(
                (840 as ::core::ffi::c_int / 10 as ::core::ffi::c_int) as DWORD,
            ) as TimeCount;
        (*pPIS).TimeOutOnSkip = BOOLEAN::FALSE.0 as ::core::ffi::c_int;
    }
    return BOOLEAN::TRUE;
}
unsafe extern "C" fn DrawTextEffect(
    mut pText: *mut TEXT,
    mut Fore: Color,
    mut Back: Color,
    mut Effect: ::core::ffi::c_int,
) {
    if Effect == 'T' as ::core::ffi::c_int {
        uqm_lua_checked!(font_DrawTracedText(pText, Fore, Back), ());
    } else {
        SetContextForeGroundColor(Fore);
        uqm_lua_checked!(font_DrawText(pText), ());
    };
}
unsafe extern "C" fn ParseTextLines(
    mut Lines: *mut TEXT,
    mut MaxLines: COUNT,
    mut Buffer: *mut ::core::ffi::c_char,
) -> COUNT {
    let mut i: COUNT = 0;
    let mut pEnd: *const ::core::ffi::c_char = Buffer.offset(strlen(Buffer) as isize);
    i = 0 as COUNT;
    while (i as ::core::ffi::c_int) < MaxLines as ::core::ffi::c_int
        && Buffer < pEnd as *mut ::core::ffi::c_char
    {
        let mut pTerm: *mut ::core::ffi::c_char = strchr(
            Buffer,
            '\n' as ::core::ffi::c_int,
        );
        if pTerm.is_null() {
            pTerm = Buffer.offset(strlen(Buffer) as isize);
        }
        *pTerm = '\0' as ::core::ffi::c_char;
        (*Lines).pStr = Buffer;
        (*Lines).CharCount = !(0 as ::core::ffi::c_int) as COUNT;
        Buffer = pTerm.offset(1 as ::core::ffi::c_int as isize);
        i = i.wrapping_add(1);
        Lines = Lines.offset(1);
    }
    return i;
}
unsafe extern "C" fn Present_BatchGraphics(mut pPIS: *mut PRESENTATION_INPUT_STATE) {
    if (*pPIS).Batched.0 == 0 {
        (*pPIS).Batched = BOOLEAN::TRUE;
        BatchGraphics();
    }
}
unsafe extern "C" fn Present_UnbatchGraphics(
    mut pPIS: *mut PRESENTATION_INPUT_STATE,
    mut bYield: BOOLEAN,
) {
    if (*pPIS).Batched.0 != 0 {
        UnbatchGraphics();
        (*pPIS).Batched = BOOLEAN::FALSE;
        if bYield.0 != 0 {
            TaskSwitch();
        }
    }
}
unsafe extern "C" fn Present_GenerateSIS(mut pPIS: *mut PRESENTATION_INPUT_STATE) {
    let mut OldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    let mut SisFrame: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
    let mut ModuleFrame: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
    let mut SkelFrame: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
    let mut s: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut hs: HOT_SPOT = POINT { x: 0, y: 0 };
    let mut slot: ::core::ffi::c_int = 0;
    let mut piece: COUNT = 0;
    let mut SisBack: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    OldContext = SetContext(OffScreenContext);
    SkelFrame = CaptureDrawable(
        uqm_lua_checked!(LoadGraphicInstance(b"graphics.flagshipskeleton\0".as_ptr() as RESOURCE), ())
            as DRAWABLE,
    );
    ModuleFrame = CaptureDrawable(
        uqm_lua_checked!(LoadGraphicInstance(b"graphics.outfitmodules\0".as_ptr() as RESOURCE), ()) as DRAWABLE,
    );
    GetFrameRect(SkelFrame, &raw mut r);
    SisFrame = CaptureDrawable(
        CreateDrawable(
            (((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as CREATE_FLAGS
                as ::core::ffi::c_int
                | ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as CREATE_FLAGS
                    as ::core::ffi::c_int) as CREATE_FLAGS,
            r.extent.width,
            r.extent.height,
            1 as COUNT,
        ),
    );
    SetContextFGFrame(SisFrame);
    SetContextClipRect(::core::ptr::null_mut::<RECT>());
    SisBack = buildColorRgba(0 as BYTE, 0 as BYTE, 0 as BYTE, 0 as BYTE);
    SetContextBackGroundColor(SisBack);
    uqm_lua_checked!(ClearDrawable(), ());
    s.frame = SetAbsFrameIndex(SkelFrame, is3DO(optFlagshipColor).0 as COUNT);
    s.origin.x = 0 as COORD;
    s.origin.y = 0 as COORD;
    uqm_lua_checked!(DrawStamp(&raw mut s), ());
    slot = 0 as ::core::ffi::c_int;
    while slot < 11 as ::core::ffi::c_int {
        piece = GlobData.SIS_state.DriveSlots[slot as usize] as COUNT;
        if (piece as ::core::ffi::c_int)
            < C2Rust_Unnamed_0::NUM_MODULES.0 as ::core::ffi::c_int
        {
            s.origin.x = (((33 as ::core::ffi::c_int) << resolutionFactor)
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
                    } << resolutionFactor)
                        - ((3 as ::core::ffi::c_int) << resolutionFactor)
                } else {
                    0 as ::core::ffi::c_int
                }) as COORD;
            s.origin.y = (((86 as ::core::ffi::c_int) << resolutionFactor)
                - (if if optWindowType == 0 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    21 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                } << resolutionFactor)
                + -(((79 as ::core::ffi::c_int) << resolutionFactor)
                    + if if resolutionFactor != 2 as ::core::ffi::c_uint {
                        BOOLEAN::FALSE.0 as ::core::ffi::c_int
                    } else {
                        BOOLEAN::TRUE.0 as ::core::ffi::c_int
                    } == 0
                    {
                        0 as ::core::ffi::c_int
                    } else {
                        33 as ::core::ffi::c_int
                    })) as COORD;
            s.origin.x = (s.origin.x as ::core::ffi::c_int
                + slot * ((12 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
            s.frame = SetAbsFrameIndex(ModuleFrame, piece);
            uqm_lua_checked!(DrawStamp(&raw mut s), ());
        }
        slot += 1;
    }
    slot = 0 as ::core::ffi::c_int;
    while slot < 8 as ::core::ffi::c_int {
        piece = GlobData.SIS_state.JetSlots[slot as usize] as COUNT;
        if (piece as ::core::ffi::c_int)
            < C2Rust_Unnamed_0::NUM_MODULES.0 as ::core::ffi::c_int
        {
            s.origin.x = (((70 as ::core::ffi::c_int) << resolutionFactor)
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
                    } << resolutionFactor)
                        - ((3 as ::core::ffi::c_int) << resolutionFactor)
                } else {
                    0 as ::core::ffi::c_int
                }) as COORD;
            s.origin.y = (((94 as ::core::ffi::c_int) << resolutionFactor)
                - (if if optWindowType == 0 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    21 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                } << resolutionFactor)
                + -(((79 as ::core::ffi::c_int) << resolutionFactor)
                    + if if resolutionFactor != 2 as ::core::ffi::c_uint {
                        BOOLEAN::FALSE.0 as ::core::ffi::c_int
                    } else {
                        BOOLEAN::TRUE.0 as ::core::ffi::c_int
                    } == 0
                    {
                        0 as ::core::ffi::c_int
                    } else {
                        33 as ::core::ffi::c_int
                    })) as COORD;
            s.origin.x = (s.origin.x as ::core::ffi::c_int
                + slot * ((12 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
            s.frame = SetAbsFrameIndex(ModuleFrame, piece);
            uqm_lua_checked!(DrawStamp(&raw mut s), ());
        }
        slot += 1;
    }
    slot = 0 as ::core::ffi::c_int;
    while slot < 16 as ::core::ffi::c_int {
        piece = GlobData.SIS_state.ModuleSlots[slot as usize] as COUNT;
        if (piece as ::core::ffi::c_int)
            < C2Rust_Unnamed_0::NUM_MODULES.0 as ::core::ffi::c_int
        {
            s.origin.x = (((17 as ::core::ffi::c_int) << resolutionFactor)
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
                    } << resolutionFactor)
                        - ((3 as ::core::ffi::c_int) << resolutionFactor)
                } else {
                    0 as ::core::ffi::c_int
                }) as COORD;
            s.origin.y = (((117 as ::core::ffi::c_int) << resolutionFactor)
                - (if if optWindowType == 0 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    21 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                } << resolutionFactor)
                + -(((79 as ::core::ffi::c_int) << resolutionFactor)
                    + if if resolutionFactor != 2 as ::core::ffi::c_uint {
                        BOOLEAN::FALSE.0 as ::core::ffi::c_int
                    } else {
                        BOOLEAN::TRUE.0 as ::core::ffi::c_int
                    } == 0
                    {
                        0 as ::core::ffi::c_int
                    } else {
                        33 as ::core::ffi::c_int
                    })) as COORD;
            s.origin.x = (s.origin.x as ::core::ffi::c_int
                + slot * ((12 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
            s.frame = SetAbsFrameIndex(ModuleFrame, piece);
            if piece as ::core::ffi::c_int
                >= C2Rust_Unnamed_0::BOMB_MODULE_0.0 as ::core::ffi::c_int
            {
                let mut oldColor: Color = SetContextForeGroundColor(
                    buildColorRgba(0 as BYTE, 0 as BYTE, 0x5f as BYTE, 0xff as BYTE),
                );
                s.origin.y = (s.origin.y as ::core::ffi::c_int
                    + ((2 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
                uqm_lua_checked!(DrawFilledStamp(&raw mut s), ());
                s.origin.y = (s.origin.y as ::core::ffi::c_int
                    - ((4 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
                uqm_lua_checked!(DrawFilledStamp(&raw mut s), ());
                SetContextForeGroundColor(
                    buildColorRgba(0 as BYTE, 0x7 as BYTE, 0xff as BYTE, 0xff as BYTE),
                );
                s.origin.y = (s.origin.y as ::core::ffi::c_int
                    + ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
                uqm_lua_checked!(DrawFilledStamp(&raw mut s), ());
                s.origin.y = (s.origin.y as ::core::ffi::c_int
                    + ((2 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
                uqm_lua_checked!(DrawFilledStamp(&raw mut s), ());
                s.origin.y = (s.origin.y as ::core::ffi::c_int
                    - ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
                SetContextForeGroundColor(oldColor);
            }
            uqm_lua_checked!(DrawStamp(&raw mut s), ());
        }
        slot += 1;
    }
    DestroyDrawable(ReleaseDrawable(SkelFrame));
    DestroyDrawable(ReleaseDrawable(ModuleFrame));
    hs.x = (r.extent.width as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as COORD;
    hs.y = (r.extent.height as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as COORD;
    SetFrameHot(SisFrame, hs);
    SetContext(OldContext);
    FlushGraphics();
    (*pPIS).SisFrame = SisFrame;
}
unsafe extern "C" fn DoSpinText(
    mut buf: *mut UNICODE,
    mut x: COORD,
    mut y: COORD,
    mut repair: FRAME,
    mut skip: *mut BOOLEAN,
) {
    let mut Text: TEXT = TEXT {
        baseline: POINT { x: 0, y: 0 },
        pStr: ::core::ptr::null::<UNICODE>(),
        align: TEXT_ALIGN::ALIGN_LEFT,
        CharCount: 0,
    };
    Text.pStr = buf;
    Text.CharCount = utf8StringCount(buf) as COUNT;
    Text.align = TEXT_ALIGN::ALIGN_LEFT;
    Text.baseline.y = y;
    Text.baseline.x = x;
    uqm_lua_checked!(font_DrawText_Fade(&raw mut Text, repair, skip), ());
}
unsafe extern "C" fn DoSpinLine(
    mut l: *mut LINE,
    mut front: Color,
    mut back: Color,
    mut skip: *mut BOOLEAN,
) {
    if (*skip).0 == 0 {
        SetContextForeGroundColor(back);
        uqm_lua_checked!(DrawLine(l, ((1 as ::core::ffi::c_int) << resolutionFactor) as BYTE), ());
        uqm_lua_checked!(PlayMenuSound(MENU_SOUND_EFFECT::MENU_SOUND_TEXT), ());
        uqm_lua_checked!(SleepThread(
            (840 as ::core::ffi::c_int / 16 as ::core::ffi::c_int) as TimePeriod,
        ), ());
    }
    SetContextForeGroundColor(front);
    uqm_lua_checked!(DrawLine(l, ((1 as ::core::ffi::c_int) << resolutionFactor) as BYTE), ());
}
unsafe extern "C" fn DoSpinStatBox(
    mut r: *mut RECT,
    mut front: Color,
    mut back: Color,
    mut skip: *mut BOOLEAN,
) {
    if (*skip).0 == 0 {
        uqm_lua_checked!(DrawStarConBox(
            r,
            ((1 as ::core::ffi::c_int) << resolutionFactor) as SIZE,
            back,
            back,
            BOOLEAN::FALSE,
            colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE),
            BOOLEAN::FALSE,
            colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE),
        ), ());
        uqm_lua_checked!(PlayMenuSound(MENU_SOUND_EFFECT::MENU_SOUND_TEXT), ());
        uqm_lua_checked!(SleepThread(
            (840 as ::core::ffi::c_int / 16 as ::core::ffi::c_int) as TimePeriod,
        ), ());
    }
    uqm_lua_checked!(DrawStarConBox(
        r,
        ((1 as ::core::ffi::c_int) << resolutionFactor) as SIZE,
        front,
        front,
        BOOLEAN::FALSE,
        colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE),
        BOOLEAN::FALSE,
        colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE),
    ), ());
}
unsafe extern "C" fn DoSpinStat(
    mut buf: *mut UNICODE,
    mut x: COORD,
    mut y: COORD,
    mut filled: COUNT,
    mut empty: COUNT,
    mut front: Color,
    mut back: Color,
    mut skip: *mut BOOLEAN,
) {
    let mut Text: TEXT = TEXT {
        baseline: POINT { x: 0, y: 0 },
        pStr: ::core::ptr::null::<UNICODE>(),
        align: TEXT_ALIGN::ALIGN_LEFT,
        CharCount: 0,
    };
    let mut i: COUNT = 0;
    let mut sq: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut c: POINT = POINT { x: 0, y: 0 };
    let mut chd: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    sq.corner.x = (x as ::core::ffi::c_int
        + ((63 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    sq.corner.y = (y as ::core::ffi::c_int
        - ((5 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    sq.extent.height = ((5 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    sq.extent.width = sq.extent.height;
    chd.extent.height = 4 as COORD;
    chd.extent.width = chd.extent.height;
    Text.pStr = buf;
    Text.CharCount = utf8StringCount(buf) as COUNT;
    Text.align = TEXT_ALIGN::ALIGN_LEFT;
    Text.baseline.y = y;
    Text.baseline.x = x;
    if (*skip).0 == 0 {
        SetContextForeGroundColor(back);
        uqm_lua_checked!(font_DrawText(&raw mut Text), ());
        uqm_lua_checked!(PlayMenuSound(MENU_SOUND_EFFECT::MENU_SOUND_TEXT), ());
        uqm_lua_checked!(SleepThread(
            (840 as ::core::ffi::c_int / 16 as ::core::ffi::c_int) as TimePeriod,
        ), ());
    }
    SetContextForeGroundColor(front);
    uqm_lua_checked!(font_DrawText(&raw mut Text), ());
    i = 0 as COUNT;
    while (i as ::core::ffi::c_int) < filled as ::core::ffi::c_int {
        if (*skip).0 == 0 {
            SetContextForeGroundColor(back);
            uqm_lua_checked!(DrawFilledRectangle(&raw mut sq), ());
            uqm_lua_checked!(PlayMenuSound(MENU_SOUND_EFFECT::MENU_SOUND_TEXT), ());
            uqm_lua_checked!(SleepThread(
                (840 as ::core::ffi::c_int / 16 as ::core::ffi::c_int) as TimePeriod,
            ), ());
        }
        SetContextForeGroundColor(front);
        uqm_lua_checked!(DrawFilledRectangle(&raw mut sq), ());
        sq.corner.x = (sq.corner.x as ::core::ffi::c_int
            + ((6 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        uqm_lua_checked!(UpdateInputState(), ());
        if CurrentInputState
            .menu[C2Rust_Unnamed::KEY_MENU_CANCEL.0 as ::core::ffi::c_int as usize] != 0
            || GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
                & C2Rust_Unnamed_1::CHECK_ABORT.0 as ::core::ffi::c_int != 0
        {
            *skip = BOOLEAN::TRUE;
        }
        i = i.wrapping_add(1);
    }
    i = 0 as COUNT;
    while (i as ::core::ffi::c_int) < empty as ::core::ffi::c_int {
        c.x = (sq.corner.x as ::core::ffi::c_int
            + ((2 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        c.y = (sq.corner.y as ::core::ffi::c_int
            + ((2 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        if (*skip).0 == 0 {
            SetContextForeGroundColor(back);
            uqm_lua_checked!(DrawStarConBox(
                &raw mut sq,
                ((1 as ::core::ffi::c_int) << resolutionFactor) as SIZE,
                back,
                back,
                BOOLEAN::FALSE,
                colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE),
                BOOLEAN::FALSE,
                colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE),
            ), ());
            if if resolutionFactor != 2 as ::core::ffi::c_uint {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } != 0
            {
                chd.corner = c;
                uqm_lua_checked!(DrawFilledRectangle(&raw mut chd), ());
            } else {
                uqm_lua_checked!(DrawPoint(&raw mut c), ());
            }
            uqm_lua_checked!(PlayMenuSound(MENU_SOUND_EFFECT::MENU_SOUND_TEXT), ());
            uqm_lua_checked!(SleepThread(
                (840 as ::core::ffi::c_int / 16 as ::core::ffi::c_int) as TimePeriod,
            ), ());
        }
        SetContextForeGroundColor(front);
        uqm_lua_checked!(DrawStarConBox(
            &raw mut sq,
            ((1 as ::core::ffi::c_int) << resolutionFactor) as SIZE,
            front,
            front,
            BOOLEAN::FALSE,
            colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE),
            BOOLEAN::FALSE,
            colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE),
        ), ());
        if if resolutionFactor != 2 as ::core::ffi::c_uint {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } != 0
        {
            chd.corner = c;
            uqm_lua_checked!(DrawFilledRectangle(&raw mut chd), ());
        } else {
            uqm_lua_checked!(DrawPoint(&raw mut c), ());
        }
        sq.corner.x = (sq.corner.x as ::core::ffi::c_int
            + ((6 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        uqm_lua_checked!(UpdateInputState(), ());
        if CurrentInputState
            .menu[C2Rust_Unnamed::KEY_MENU_CANCEL.0 as ::core::ffi::c_int as usize] != 0
            || GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
                & C2Rust_Unnamed_1::CHECK_ABORT.0 as ::core::ffi::c_int != 0
        {
            *skip = BOOLEAN::TRUE;
        }
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn Present_DrawMovieFrame(mut pPIS: *mut PRESENTATION_INPUT_STATE) {
    let mut s: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    s.origin.x = 0 as COORD;
    s.origin.y = 0 as COORD;
    s.frame = SetAbsFrameIndex((*pPIS).Frame, (*pPIS).MovieFrame as COUNT);
    uqm_lua_checked!(DrawStamp(&raw mut s), ());
}
unsafe extern "C" fn ShowPresentationFile(
    mut name: *const ::core::ffi::c_char,
) -> BOOLEAN {
    let mut pres: STRING = CaptureStringTable(uqm_lua_checked!(LoadStringTableFile(contentDir, name), BOOLEAN(0)));
    let mut result: BOOLEAN = uqm_lua_checked!(ShowSlidePresentation(pres), BOOLEAN(0));
    DestroyStringTable(ReleaseStringTable(pres));
    return result;
}
static mut ship_map: [SHIPMAP; 25] = unsafe {
    [
        SHIPMAP {
            sID: SPECIES_ID::ARILOU_ID,
            ditty: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"arilou\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            race: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"ARILOULALEELAY\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            ship: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b" SKIFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            spinline: 2 as COUNT,
            width: 123 as COUNT,
        },
        SHIPMAP {
            sID: SPECIES_ID::CHMMR_ID,
            ditty: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"chmmr\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            race: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"CHMMR\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            ship: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b" AVATAR\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            spinline: 2 as COUNT,
            width: 50 as COUNT,
        },
        SHIPMAP {
            sID: SPECIES_ID::EARTHLING_ID,
            ditty: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"earthling\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            race: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"EARTHLING\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            ship: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b" CRUISER\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            spinline: 130 as COUNT,
            width: 79 as COUNT,
        },
        SHIPMAP {
            sID: SPECIES_ID::ORZ_ID,
            ditty: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"orz\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            race: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"ORZ\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            ship: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b" NEMESIS\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            spinline: 2 as COUNT,
            width: 25 as COUNT,
        },
        SHIPMAP {
            sID: SPECIES_ID::PKUNK_ID,
            ditty: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"pkunk\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            race: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"PKUNK\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            ship: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b" FURY\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            spinline: 2 as COUNT,
            width: 43 as COUNT,
        },
        SHIPMAP {
            sID: SPECIES_ID::SHOFIXTI_ID,
            ditty: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"shofixti\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            race: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"SHOFIXTI\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            ship: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b" SCOUT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            spinline: 4 as COUNT,
            width: 61 as COUNT,
        },
        SHIPMAP {
            sID: SPECIES_ID::SPATHI_ID,
            ditty: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"spathi\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            race: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"SPATHI\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            ship: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b" ELUDER\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            spinline: 2 as COUNT,
            width: 47 as COUNT,
        },
        SHIPMAP {
            sID: SPECIES_ID::SUPOX_ID,
            ditty: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"supox\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            race: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"SUPOX\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            ship: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b" BLADE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            spinline: 2 as COUNT,
            width: 43 as COUNT,
        },
        SHIPMAP {
            sID: SPECIES_ID::THRADDASH_ID,
            ditty: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"thraddash\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            race: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"THRADDASH\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            ship: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b" TORCH\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            spinline: 2 as COUNT,
            width: 87 as COUNT,
        },
        SHIPMAP {
            sID: SPECIES_ID::UTWIG_ID,
            ditty: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"utwig\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            race: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"UTWIG\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            ship: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b" JUGGER\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            spinline: 2 as COUNT,
            width: 43 as COUNT,
        },
        SHIPMAP {
            sID: SPECIES_ID::VUX_ID,
            ditty: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"vux\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            race: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"VUX\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            ship: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b" INTRUDER\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            spinline: 130 as COUNT,
            width: 28 as COUNT,
        },
        SHIPMAP {
            sID: SPECIES_ID::YEHAT_ID,
            ditty: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"yehat\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            race: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"YEHAT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            ship: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b" TERMINATOR\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            spinline: 130 as COUNT,
            width: 46 as COUNT,
        },
        SHIPMAP {
            sID: SPECIES_ID::MELNORME_ID,
            ditty: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"melnorme\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            race: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"MELNORME\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            ship: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b" TRADER\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            spinline: 2 as COUNT,
            width: 77 as COUNT,
        },
        SHIPMAP {
            sID: SPECIES_ID::DRUUGE_ID,
            ditty: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"druuge\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            race: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"DRUUGE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            ship: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b" MAULER\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            spinline: 2 as COUNT,
            width: 53 as COUNT,
        },
        SHIPMAP {
            sID: SPECIES_ID::ILWRATH_ID,
            ditty: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"ilwrath\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            race: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"ILWRATH\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            ship: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b" AVENGER\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            spinline: 2 as COUNT,
            width: 62 as COUNT,
        },
        SHIPMAP {
            sID: SPECIES_ID::MYCON_ID,
            ditty: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"mycon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            race: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"MYCON\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            ship: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b" PODSHIP\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            spinline: 127 as COUNT,
            width: 47 as COUNT,
        },
        SHIPMAP {
            sID: SPECIES_ID::SLYLANDRO_ID,
            ditty: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"slylandro\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            race: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"SLYLANDRO\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            ship: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b" PROBE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            spinline: 1 as COUNT,
            width: 80 as COUNT,
        },
        SHIPMAP {
            sID: SPECIES_ID::UMGAH_ID,
            ditty: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"umgah\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            race: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"UMGAH\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            ship: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b" DRONE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            spinline: 127 as COUNT,
            width: 50 as COUNT,
        },
        SHIPMAP {
            sID: SPECIES_ID::UR_QUAN_ID,
            ditty: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"urquan\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            race: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"UR-QUAN\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            ship: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"\nDREADNAUGHT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            spinline: 119 as COUNT,
            width: 66 as COUNT,
        },
        SHIPMAP {
            sID: SPECIES_ID::ZOQFOTPIK_ID,
            ditty: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"zoqfotpik\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            race: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"ZOQ-FOT-PIK\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            ship: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b" STINGER\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            spinline: 130 as COUNT,
            width: 92 as COUNT,
        },
        SHIPMAP {
            sID: SPECIES_ID::SYREEN_ID,
            ditty: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"syreen\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            race: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"SYREEN\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            ship: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b" PENETRATOR\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            spinline: 2 as COUNT,
            width: 53 as COUNT,
        },
        SHIPMAP {
            sID: SPECIES_ID::KOHR_AH_ID,
            ditty: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"kohrah\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            race: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"KOHR-AH\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            ship: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            spinline: 119 as COUNT,
            width: 64 as COUNT,
        },
        SHIPMAP {
            sID: SPECIES_ID::ANDROSYNTH_ID,
            ditty: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"androsynth\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            race: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"ANDROSYNTH\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            ship: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b" GUARDIAN\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            spinline: 2 as COUNT,
            width: 94 as COUNT,
        },
        SHIPMAP {
            sID: SPECIES_ID::CHENJESU_ID,
            ditty: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"chenjesu\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            race: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"CHENJESU\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            ship: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"    BROODHOME\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            spinline: 1 as COUNT,
            width: 71 as COUNT,
        },
        SHIPMAP {
            sID: SPECIES_ID::MMRNMHRM_ID,
            ditty: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"mmrnmhrm\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            race: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"MMRNMHRM\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            ship: ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b" TRANSFORMER\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            spinline: 128 as COUNT,
            width: 84 as COUNT,
        },
    ]
};
static mut shipID: COUNT = 0;
static mut raceID: COUNT = 0;
static mut linespun: BOOLEAN = BOOLEAN(0 as ::core::ffi::c_int as ::core::ffi::c_uint);
unsafe extern "C" fn SeedDitty(
    mut buf: *mut ::core::ffi::c_char,
    mut size: size_t,
    mut str: *mut ::core::ffi::c_char,
) {
    if optShipSeed.0 != 0 {
        shipID = 0 as COUNT;
        while (shipID as usize)
            < ::core::mem::size_of::<[SHIPMAP; 25]>()
                .wrapping_div(::core::mem::size_of::<SHIPMAP>())
        {
            if strcasecmp(
                str,
                &raw const (*(&raw const ship_map as *const SHIPMAP)
                    .offset(shipID as isize))
                    .ditty as *const ::core::ffi::c_char,
            ) == 0
            {
                break;
            }
            shipID = shipID.wrapping_add(1);
        }
        if (shipID as usize)
            < ::core::mem::size_of::<[SHIPMAP; 25]>()
                .wrapping_div(::core::mem::size_of::<SHIPMAP>())
        {
            raceID = 0 as COUNT;
            while (raceID as usize)
                < ::core::mem::size_of::<[SHIPMAP; 25]>()
                    .wrapping_div(::core::mem::size_of::<SHIPMAP>())
            {
                if SeedShip(
                        ship_map[raceID as usize].sID,
                        BOOLEAN(0 as ::core::ffi::c_int as ::core::ffi::c_uint),
                    )
                    .0 == ship_map[shipID as usize].sID.0
                {
                    break;
                }
                raceID = raceID.wrapping_add(1);
            }
            if (raceID as usize)
                < ::core::mem::size_of::<[SHIPMAP; 25]>()
                    .wrapping_div(::core::mem::size_of::<SHIPMAP>())
            {
                linespun = BOOLEAN(0 as ::core::ffi::c_int as ::core::ffi::c_uint);
                snprintf(
                    buf,
                    size,
                    b"ship.%s.ditty\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw const (*(&raw const ship_map as *const SHIPMAP)
                        .offset(raceID as isize))
                        .ditty as *const ::core::ffi::c_char,
                );
                return;
            }
        }
    }
    raceID = ::core::mem::size_of::<[SHIPMAP; 25]>()
        .wrapping_div(::core::mem::size_of::<SHIPMAP>()) as COUNT;
    shipID = raceID;
    snprintf(buf, size, b"ship.%s.ditty\0".as_ptr() as *const ::core::ffi::c_char, str);
}
unsafe extern "C" fn SeedLineSpin(
    mut x1: *mut ::core::ffi::c_int,
    mut y1: *mut ::core::ffi::c_int,
    mut x2: *mut ::core::ffi::c_int,
    mut y2: *mut ::core::ffi::c_int,
) {
    if !(optShipSeed.0 == 0 || x1.is_null() || x2.is_null() || y1.is_null()
        || y2.is_null() || shipID as ::core::ffi::c_int == raceID as ::core::ffi::c_int
        || shipID as usize
            >= ::core::mem::size_of::<[SHIPMAP; 25]>()
                .wrapping_div(::core::mem::size_of::<SHIPMAP>())
        || raceID as usize
            >= ::core::mem::size_of::<[SHIPMAP; 25]>()
                .wrapping_div(::core::mem::size_of::<SHIPMAP>()))
    {
        if ship_map[shipID as usize].sID.0 == SPECIES_ID::KOHR_AH_ID.0
            && ship_map[raceID as usize].sID.0 != SPECIES_ID::ARILOU_ID.0
            && ship_map[raceID as usize].width as ::core::ffi::c_int
                > 80 as ::core::ffi::c_int && *y2 == 122 as ::core::ffi::c_int
        {
            *x2
                -= ship_map[raceID as usize].width as ::core::ffi::c_int
                    - 80 as ::core::ffi::c_int;
        }
        if ship_map[shipID as usize].sID.0 == SPECIES_ID::DRUUGE_ID.0
            && if resolutionFactor != 2 as ::core::ffi::c_uint {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } == 0
            && ship_map[raceID as usize].width as ::core::ffi::c_int
                > 80 as ::core::ffi::c_int && *y1 == 21 as ::core::ffi::c_int
        {
            *y1 += 10 as ::core::ffi::c_int;
            *y2 += 10 as ::core::ffi::c_int;
        }
    }
}
unsafe extern "C" fn SeedTextSpin(
    mut buf: *mut ::core::ffi::c_char,
    mut size: size_t,
    mut str: *mut ::core::ffi::c_char,
    mut x: *mut ::core::ffi::c_int,
    mut y: *mut ::core::ffi::c_int,
) {
    '_SeedTextSpinPassThru: {
        if !(optShipSeed.0 == 0 || x.is_null() || y.is_null()
            || shipID as ::core::ffi::c_int == raceID as ::core::ffi::c_int
            || shipID as usize
                >= ::core::mem::size_of::<[SHIPMAP; 25]>()
                    .wrapping_div(::core::mem::size_of::<SHIPMAP>())
            || raceID as usize
                >= ::core::mem::size_of::<[SHIPMAP; 25]>()
                    .wrapping_div(::core::mem::size_of::<SHIPMAP>()))
        {
            if ship_map[shipID as usize].sID.0 == SPECIES_ID::SPATHI_ID.0
                && if resolutionFactor != 2 as ::core::ffi::c_uint {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } != 0 && ship_map[raceID as usize].sID.0 != SPECIES_ID::ARILOU_ID.0
                && ship_map[raceID as usize].width as ::core::ffi::c_int
                    > 80 as ::core::ffi::c_int
                && (*x == 25 as ::core::ffi::c_int || *x == 40 as ::core::ffi::c_int)
            {
                *x -= 15 as ::core::ffi::c_int;
            }
            if ship_map[shipID as usize].sID.0 == SPECIES_ID::DRUUGE_ID.0
                && if resolutionFactor != 2 as ::core::ffi::c_uint {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } == 0
                && ship_map[raceID as usize].width as ::core::ffi::c_int
                    > 80 as ::core::ffi::c_int && *x == 108 as ::core::ffi::c_int
            {
                *y += 10 as ::core::ffi::c_int;
            }
            if !(ship_map[shipID as usize].sID.0 == SPECIES_ID::SLYLANDRO_ID.0
                && *x == 18 as ::core::ffi::c_int)
            {
                if ship_map[shipID as usize].spinline as ::core::ffi::c_int != *y {
                    break '_SeedTextSpinPassThru;
                } else if ship_map[shipID as usize].sID.0 == SPECIES_ID::MELNORME_ID.0
                    && *x == 215 as ::core::ffi::c_int
                {
                    break '_SeedTextSpinPassThru;
                } else if ship_map[shipID as usize].sID.0 == SPECIES_ID::UR_QUAN_ID.0
                    && *x == 222 as ::core::ffi::c_int
                {
                    break '_SeedTextSpinPassThru;
                } else if linespun.0 == 0 {
                    linespun = BOOLEAN(1 as ::core::ffi::c_int as ::core::ffi::c_uint);
                    utf8StringCopy(
                        buf,
                        size,
                        &raw const (*(&raw const ship_map as *const SHIPMAP)
                            .offset(raceID as isize))
                            .race as *const ::core::ffi::c_char,
                    );
                    buf = buf
                        .offset(
                            strlen(
                                    &raw const (*(&raw const ship_map as *const SHIPMAP)
                                        .offset(raceID as isize))
                                        .race as *const ::core::ffi::c_char,
                                )
                                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>())
                                as isize,
                        );
                    match ship_map[shipID as usize].sID {
                        SPECIES_ID::CHENJESU_ID
                        | SPECIES_ID::EARTHLING_ID
                        | SPECIES_ID::KOHR_AH_ID
                        | SPECIES_ID::MYCON_ID
                        | SPECIES_ID::THRADDASH_ID => {
                            if (ship_map[shipID as usize].sID.0
                                == SPECIES_ID::EARTHLING_ID.0
                                || ship_map[shipID as usize].sID.0
                                    == SPECIES_ID::KOHR_AH_ID.0
                                    && if resolutionFactor != 2 as ::core::ffi::c_uint {
                                        BOOLEAN::FALSE.0 as ::core::ffi::c_int
                                    } else {
                                        BOOLEAN::TRUE.0 as ::core::ffi::c_int
                                    } != 0)
                                && ship_map[raceID as usize].sID.0
                                    == SPECIES_ID::ARILOU_ID.0
                            {
                                buf = buf
                                    .offset(
                                        -(8usize
                                            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>())
                                            as isize),
                                    );
                                *x
                                    += ship_map[shipID as usize].width as ::core::ffi::c_int
                                        - 50 as ::core::ffi::c_int;
                            } else {
                                if ship_map[shipID as usize].sID.0
                                    == SPECIES_ID::CHENJESU_ID.0
                                    && if resolutionFactor != 2 as ::core::ffi::c_uint {
                                        BOOLEAN::FALSE.0 as ::core::ffi::c_int
                                    } else {
                                        BOOLEAN::TRUE.0 as ::core::ffi::c_int
                                    } != 0
                                {
                                    *x -= 10 as ::core::ffi::c_int;
                                }
                                *x
                                    += ship_map[shipID as usize].width as ::core::ffi::c_int
                                        - ship_map[raceID as usize].width as ::core::ffi::c_int;
                            }
                        }
                        SPECIES_ID::SHOFIXTI_ID => {
                            *x
                                += (ship_map[shipID as usize].width as ::core::ffi::c_int
                                    - ship_map[raceID as usize].width as ::core::ffi::c_int)
                                    * 4 as ::core::ffi::c_int / 5 as ::core::ffi::c_int;
                        }
                        SPECIES_ID::VUX_ID => {
                            if ship_map[shipID as usize].sID.0 == SPECIES_ID::VUX_ID.0
                                && ship_map[raceID as usize].sID.0
                                    == SPECIES_ID::ARILOU_ID.0
                            {
                                buf = buf
                                    .offset(
                                        -(8usize
                                            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>())
                                            as isize),
                                    );
                                *x
                                    += (ship_map[shipID as usize].width as ::core::ffi::c_int
                                        - 50 as ::core::ffi::c_int) * 3 as ::core::ffi::c_int
                                        / 5 as ::core::ffi::c_int;
                            } else {
                                *x
                                    += (ship_map[shipID as usize].width as ::core::ffi::c_int
                                        - ship_map[raceID as usize].width as ::core::ffi::c_int)
                                        * 3 as ::core::ffi::c_int / 5 as ::core::ffi::c_int;
                            }
                        }
                        SPECIES_ID::ANDROSYNTH_ID
                        | SPECIES_ID::ARILOU_ID
                        | SPECIES_ID::PKUNK_ID
                        | SPECIES_ID::SPATHI_ID
                        | SPECIES_ID::SYREEN_ID
                        | SPECIES_ID::UTWIG_ID
                        | SPECIES_ID::MMRNMHRM_ID => {
                            if ship_map[shipID as usize].sID.0 == SPECIES_ID::SPATHI_ID.0
                                && ship_map[raceID as usize].sID.0
                                    == SPECIES_ID::ARILOU_ID.0
                            {
                                buf = buf
                                    .offset(
                                        -(8usize
                                            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>())
                                            as isize),
                                    );
                                *x
                                    += (ship_map[shipID as usize].width as ::core::ffi::c_int
                                        - 50 as ::core::ffi::c_int) / 2 as ::core::ffi::c_int;
                            } else {
                                if ship_map[shipID as usize].sID.0
                                    == SPECIES_ID::SPATHI_ID.0
                                {
                                    *x += 2 as ::core::ffi::c_int;
                                }
                                if ship_map[shipID as usize].sID.0
                                    == SPECIES_ID::ARILOU_ID.0
                                {
                                    *x += 4 as ::core::ffi::c_int;
                                }
                                *x
                                    += (ship_map[shipID as usize].width as ::core::ffi::c_int
                                        - ship_map[raceID as usize].width as ::core::ffi::c_int)
                                        / 2 as ::core::ffi::c_int;
                            }
                        }
                        SPECIES_ID::CHMMR_ID
                        | SPECIES_ID::ILWRATH_ID
                        | SPECIES_ID::MELNORME_ID
                        | SPECIES_ID::ORZ_ID
                        | SPECIES_ID::SLYLANDRO_ID
                        | SPECIES_ID::UMGAH_ID
                        | SPECIES_ID::UR_QUAN_ID
                        | SPECIES_ID::YEHAT_ID
                        | SPECIES_ID::ZOQFOTPIK_ID => {
                            if (ship_map[shipID as usize].sID.0 == SPECIES_ID::CHMMR_ID.0
                                || ship_map[shipID as usize].sID.0 == SPECIES_ID::ORZ_ID.0
                                || ship_map[shipID as usize].sID.0
                                    == SPECIES_ID::ZOQFOTPIK_ID.0)
                                && ship_map[raceID as usize].sID.0
                                    == SPECIES_ID::ARILOU_ID.0
                            {
                                buf = buf
                                    .offset(
                                        -(8usize
                                            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>())
                                            as isize),
                                    );
                            } else if ship_map[shipID as usize].sID.0
                                == SPECIES_ID::UMGAH_ID.0
                                && if resolutionFactor != 2 as ::core::ffi::c_uint {
                                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                                } else {
                                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                                } == 0
                            {
                                *x
                                    += (ship_map[shipID as usize].width as ::core::ffi::c_int
                                        - ship_map[raceID as usize].width as ::core::ffi::c_int)
                                        * 4 as ::core::ffi::c_int / 5 as ::core::ffi::c_int;
                            }
                        }
                        SPECIES_ID::DRUUGE_ID | SPECIES_ID::SUPOX_ID => {
                            if ship_map[raceID as usize].width as ::core::ffi::c_int
                                > 80 as ::core::ffi::c_int
                            {
                                let mut pad: *mut ::core::ffi::c_char = (if if resolutionFactor
                                    != 2 as ::core::ffi::c_uint
                                {
                                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                                } else {
                                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                                } != 0
                                {
                                    b"\n   \0".as_ptr() as *const ::core::ffi::c_char
                                } else {
                                    b"\n       \0".as_ptr() as *const ::core::ffi::c_char
                                }) as *mut ::core::ffi::c_char;
                                utf8StringCopy(buf, size, pad);
                                buf = buf
                                    .offset(
                                        strlen(pad)
                                            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>())
                                            as isize,
                                    );
                            }
                        }
                        _ => {}
                    }
                    utf8StringCopy(
                        buf,
                        size,
                        &raw const (*(&raw const ship_map as *const SHIPMAP)
                            .offset(shipID as isize))
                            .ship as *const ::core::ffi::c_char,
                    );
                    return;
                }
            }
            utf8StringCopy(buf, size, b"\0".as_ptr() as *const ::core::ffi::c_char);
            return;
        }
    }
    utf8StringCopy(buf, size, str);
}
unsafe extern "C" fn DoPresentation(mut pIS: *mut ::core::ffi::c_void) -> BOOLEAN {
    let mut pPIS: *mut PRESENTATION_INPUT_STATE = pIS as *mut PRESENTATION_INPUT_STATE;
    if PulsedInputState
        .menu[C2Rust_Unnamed::KEY_MENU_CANCEL.0 as ::core::ffi::c_int as usize] != 0
        || GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
            & C2Rust_Unnamed_1::CHECK_ABORT.0 as ::core::ffi::c_int != 0
    {
        return BOOLEAN::FALSE;
    }
    if (*pPIS).TimeOut != 0 {
        let mut Delay: TimeCount = (840 as ::core::ffi::c_int / 84 as ::core::ffi::c_int)
            as TimeCount;
        if GetTimeCounter() >= (*pPIS).TimeOut {
            if (*pPIS).MovieFrame >= 0 as ::core::ffi::c_int {
                uqm_lua_checked!(Present_DrawMovieFrame(pPIS), BOOLEAN(0));
                (*pPIS).MovieFrame += 1;
                if (*pPIS).MovieFrame > (*pPIS).MovieEndFrame {
                    (*pPIS).MovieFrame = -1 as ::core::ffi::c_int;
                }
                Delay = (*pPIS).InterframeDelay as TimeCount;
            } else {
                (*pPIS).TimeOut = 0 as TimeCount;
                return BOOLEAN::TRUE;
            }
        }
        if (*pPIS).TimeOutOnSkip != 0
            && (PulsedInputState
                .menu[C2Rust_Unnamed::KEY_MENU_SELECT.0 as ::core::ffi::c_int as usize]
                != 0
                || PulsedInputState
                    .menu[C2Rust_Unnamed::KEY_MENU_SPECIAL.0 as ::core::ffi::c_int
                    as usize] != 0
                || PulsedInputState
                    .menu[C2Rust_Unnamed::KEY_MENU_RIGHT.0 as ::core::ffi::c_int
                    as usize] != 0)
        {
            (*pPIS).TimeOut = 0 as TimeCount;
            (*pPIS).MovieFrame = -1 as ::core::ffi::c_int;
            return BOOLEAN::TRUE;
        }
        uqm_lua_checked!(SleepThread(Delay), BOOLEAN(0));
        return BOOLEAN::TRUE;
    }
    while ((*pPIS).OperIndex as ::core::ffi::c_int)
        < GetStringTableCount((*pPIS).SlideShow) as ::core::ffi::c_int
    {
        let mut Opcode: [::core::ffi::c_char; 16] = [0; 16];
        let mut pStr: *mut ::core::ffi::c_char = GetStringAddress((*pPIS).SlideShow);
        (*pPIS).OperIndex = (*pPIS).OperIndex.wrapping_add(1);
        (*pPIS).SlideShow = SetRelStringTableIndex((*pPIS).SlideShow, 1 as SIZE);
        if pStr.is_null() {
            continue;
        }
        if 1 as ::core::ffi::c_int
            != sscanf(
                pStr,
                b"%15s\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut Opcode as *mut ::core::ffi::c_char,
            )
        {
            continue;
        }
        pStr = pStr.offset(strlen(&raw mut Opcode as *mut ::core::ffi::c_char) as isize);
        if *pStr as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
            pStr = pStr.offset(1);
        }
        strupr(&raw mut Opcode as *mut ::core::ffi::c_char);
        if strcmp(
            &raw mut Opcode as *mut ::core::ffi::c_char,
            b"DIMS\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            let mut w: ::core::ffi::c_int = 0;
            let mut h: ::core::ffi::c_int = 0;
            if 2 as ::core::ffi::c_int
                == sscanf(
                    pStr,
                    b"%d %d\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut w,
                    &raw mut h,
                )
            {
                w <<= resolutionFactor;
                h <<= resolutionFactor;
                (*pPIS).clip_r.extent.width = w as COORD;
                (*pPIS).clip_r.extent.height = h as COORD;
                (*pPIS).clip_r.corner.x = ((CanvasWidth - w) / 2 as ::core::ffi::c_int)
                    as COORD;
                (*pPIS).clip_r.corner.y = ((CanvasHeight - h) / 2 as ::core::ffi::c_int)
                    as COORD;
                SetContextClipRect(&raw mut (*pPIS).clip_r);
            }
        } else if strcmp(
            &raw mut Opcode as *mut ::core::ffi::c_char,
            b"FONT\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            let mut index: ::core::ffi::c_int = 0;
            let mut pFont: *mut FONT = ::core::ptr::null_mut::<FONT>();
            (::core::mem::size_of::<[::core::ffi::c_char; 512]>() >= 256usize
                || {
                    __assert_fail(
                        b"sizeof (pPIS->Buffer) >= 256\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"src/uqm/intro.c\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        753 as ::core::ffi::c_int,
                        b"DoPresentation\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    0 as ::core::ffi::c_int != 0
                }) as ::core::ffi::c_int;
            (*pPIS).Buffer[0usize] = '\0' as ::core::ffi::c_char;
            if 1 as ::core::ffi::c_int
                > sscanf(
                    pStr,
                    b"%d %255[^\n]\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut index,
                    &raw mut (*pPIS).Buffer as *mut ::core::ffi::c_char,
                ) || index < 0 as ::core::ffi::c_int || index >= 5 as ::core::ffi::c_int
            {
                log_add(
                    log_Level::log_Warning,
                    b"Bad FONT command '%s'\0".as_ptr() as *const ::core::ffi::c_char,
                    pStr,
                );
            } else {
                pFont = (&raw mut (*pPIS).Fonts as *mut FONT).offset(index as isize);
                if (*pPIS).Buffer[0usize] != 0 {
                    if !(*pFont).is_null() {
                        DestroyFont(*pFont);
                    }
                    *pFont = uqm_lua_checked!(LoadFontFile(
                        &raw mut (*pPIS).Buffer as *mut ::core::ffi::c_char,
                    ), BOOLEAN(0));
                }
                SetContextFont(*pFont);
            }
        } else if strcmp(
            &raw mut Opcode as *mut ::core::ffi::c_char,
            b"FONT1X\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
            && if resolutionFactor != 2 as ::core::ffi::c_uint {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } == 0
        {
            let mut index_0: ::core::ffi::c_int = 0;
            let mut pFont_0: *mut FONT = ::core::ptr::null_mut::<FONT>();
            (::core::mem::size_of::<[::core::ffi::c_char; 512]>() >= 256usize
                || {
                    __assert_fail(
                        b"sizeof (pPIS->Buffer) >= 256\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"src/uqm/intro.c\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        778 as ::core::ffi::c_int,
                        b"DoPresentation\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    0 as ::core::ffi::c_int != 0
                }) as ::core::ffi::c_int;
            (*pPIS).Buffer[0usize] = '\0' as ::core::ffi::c_char;
            if 1 as ::core::ffi::c_int
                > sscanf(
                    pStr,
                    b"%d %255[^\n]\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut index_0,
                    &raw mut (*pPIS).Buffer as *mut ::core::ffi::c_char,
                ) || index_0 < 0 as ::core::ffi::c_int
                || index_0 >= 5 as ::core::ffi::c_int
            {
                log_add(
                    log_Level::log_Warning,
                    b"Bad FONT command '%s'\0".as_ptr() as *const ::core::ffi::c_char,
                    pStr,
                );
            } else {
                pFont_0 = (&raw mut (*pPIS).Fonts as *mut FONT).offset(index_0 as isize);
                if (*pPIS).Buffer[0usize] != 0 {
                    if !(*pFont_0).is_null() {
                        DestroyFont(*pFont_0);
                    }
                    *pFont_0 = uqm_lua_checked!(LoadFontFile(
                        &raw mut (*pPIS).Buffer as *mut ::core::ffi::c_char,
                    ), BOOLEAN(0));
                }
                SetContextFont(*pFont_0);
            }
        } else if strcmp(
            &raw mut Opcode as *mut ::core::ffi::c_char,
            b"FONT4X\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
            && if resolutionFactor != 2 as ::core::ffi::c_uint {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } != 0
        {
            let mut index_1: ::core::ffi::c_int = 0;
            let mut pFont_1: *mut FONT = ::core::ptr::null_mut::<FONT>();
            (::core::mem::size_of::<[::core::ffi::c_char; 512]>() >= 256usize
                || {
                    __assert_fail(
                        b"sizeof (pPIS->Buffer) >= 256\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"src/uqm/intro.c\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        802 as ::core::ffi::c_int,
                        b"DoPresentation\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    0 as ::core::ffi::c_int != 0
                }) as ::core::ffi::c_int;
            (*pPIS).Buffer[0usize] = '\0' as ::core::ffi::c_char;
            if 1 as ::core::ffi::c_int
                > sscanf(
                    pStr,
                    b"%d %255[^\n]\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut index_1,
                    &raw mut (*pPIS).Buffer as *mut ::core::ffi::c_char,
                ) || index_1 < 0 as ::core::ffi::c_int
                || index_1 >= 5 as ::core::ffi::c_int
            {
                log_add(
                    log_Level::log_Warning,
                    b"Bad FONT command '%s'\0".as_ptr() as *const ::core::ffi::c_char,
                    pStr,
                );
            } else {
                pFont_1 = (&raw mut (*pPIS).Fonts as *mut FONT).offset(index_1 as isize);
                if (*pPIS).Buffer[0usize] != 0 {
                    if !(*pFont_1).is_null() {
                        DestroyFont(*pFont_1);
                    }
                    *pFont_1 = uqm_lua_checked!(LoadFontFile(
                        &raw mut (*pPIS).Buffer as *mut ::core::ffi::c_char,
                    ), BOOLEAN(0));
                }
                SetContextFont(*pFont_1);
            }
        } else if strcmp(
            &raw mut Opcode as *mut ::core::ffi::c_char,
            b"ANI\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            utf8StringCopy(
                &raw mut (*pPIS).Buffer as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                pStr,
            );
            if !(*pPIS).Frame.is_null() {
                DestroyDrawable(ReleaseDrawable((*pPIS).Frame));
            }
            (*pPIS).Frame = CaptureDrawable(
                uqm_lua_checked!(LoadGraphicFile(&raw mut (*pPIS).Buffer as *mut ::core::ffi::c_char), BOOLEAN(0)),
            );
        } else if strcmp(
            &raw mut Opcode as *mut ::core::ffi::c_char,
            b"ANI1X\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
            && if resolutionFactor != 2 as ::core::ffi::c_uint {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } == 0
        {
            utf8StringCopy(
                &raw mut (*pPIS).Buffer as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                pStr,
            );
            if !(*pPIS).Frame.is_null() {
                DestroyDrawable(ReleaseDrawable((*pPIS).Frame));
            }
            (*pPIS).Frame = CaptureDrawable(
                uqm_lua_checked!(LoadGraphicFile(&raw mut (*pPIS).Buffer as *mut ::core::ffi::c_char), BOOLEAN(0)),
            );
        } else if strcmp(
            &raw mut Opcode as *mut ::core::ffi::c_char,
            b"ANI4X\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
            && if resolutionFactor != 2 as ::core::ffi::c_uint {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } != 0
        {
            utf8StringCopy(
                &raw mut (*pPIS).Buffer as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                pStr,
            );
            if !(*pPIS).Frame.is_null() {
                DestroyDrawable(ReleaseDrawable((*pPIS).Frame));
            }
            (*pPIS).Frame = CaptureDrawable(
                uqm_lua_checked!(LoadGraphicFile(&raw mut (*pPIS).Buffer as *mut ::core::ffi::c_char), BOOLEAN(0)),
            );
        } else if strcmp(
            &raw mut Opcode as *mut ::core::ffi::c_char,
            b"MUSIC\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            utf8StringCopy(
                &raw mut (*pPIS).Buffer as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                pStr,
            );
            if !(*pPIS).MusicRef.is_null() {
                uqm_lua_checked!(StopMusic(), BOOLEAN(0));
                uqm_lua_checked!(DestroyMusic((*pPIS).MusicRef), BOOLEAN(0));
            }
            (*pPIS).MusicRef = uqm_lua_checked!(LoadMusicFile(
                &raw mut (*pPIS).Buffer as *mut ::core::ffi::c_char,
            ), BOOLEAN(0));
            uqm_lua_checked!(PlayMusic((*pPIS).MusicRef, BOOLEAN::FALSE, 1 as BYTE), BOOLEAN(0));
        } else if strcmp(
            &raw mut Opcode as *mut ::core::ffi::c_char,
            b"DITTY\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            if optShipSeed.0 != 0 {
                SeedDitty(
                    &raw mut (*pPIS).Buffer as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                    pStr,
                );
            } else {
                snprintf(
                    &raw mut (*pPIS).Buffer as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                    b"ship.%s.ditty\0".as_ptr() as *const ::core::ffi::c_char,
                    pStr,
                );
            }
            if !(*pPIS).MusicRef.is_null() {
                uqm_lua_checked!(StopMusic(), BOOLEAN(0));
                uqm_lua_checked!(DestroyMusic((*pPIS).MusicRef), BOOLEAN(0));
            }
            (*pPIS).MusicRef = uqm_lua_checked!(LoadMusicInstance(
                &raw mut (*pPIS).Buffer as *mut ::core::ffi::c_char as RESOURCE,
            ), BOOLEAN(0));
            uqm_lua_checked!(PlayMusic((*pPIS).MusicRef, BOOLEAN::FALSE, 1 as BYTE), BOOLEAN(0));
        } else if strcmp(
            &raw mut Opcode as *mut ::core::ffi::c_char,
            b"WAIT\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            let mut msecs: ::core::ffi::c_int = 0;
            Present_UnbatchGraphics(pPIS, BOOLEAN::TRUE);
            if 1 as ::core::ffi::c_int
                == sscanf(
                    pStr,
                    b"%d\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut msecs,
                )
            {
                (*pPIS).TimeOut = GetTimeCounter()
                    .wrapping_add(
                        (msecs * 840 as ::core::ffi::c_int / 1000 as ::core::ffi::c_int)
                            as TimeCount,
                    );
                (*pPIS).TimeOutOnSkip = BOOLEAN::TRUE.0 as ::core::ffi::c_int;
                return BOOLEAN::TRUE;
            }
        } else if strcmp(
            &raw mut Opcode as *mut ::core::ffi::c_char,
            b"WAITDITTY\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            while PlayingStream(
                    (0 as ::core::ffi::c_int
                        + (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint32,
                )
                .0 != 0
            {
                if CurrentInputState
                    .menu[C2Rust_Unnamed::KEY_MENU_CANCEL.0 as ::core::ffi::c_int
                    as usize] != 0
                    || GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
                        & C2Rust_Unnamed_1::CHECK_ABORT.0 as ::core::ffi::c_int != 0
                {
                    uqm_lua_checked!(StopMusic(), BOOLEAN(0));
                    (*pPIS).Skip = BOOLEAN::TRUE;
                    return BOOLEAN::TRUE;
                }
                uqm_lua_checked!(SleepThread(
                    (840 as ::core::ffi::c_int / 10 as ::core::ffi::c_int) as TimePeriod,
                ), BOOLEAN(0));
                uqm_lua_checked!(UpdateInputState(), BOOLEAN(0));
            }
        } else if strcmp(
            &raw mut Opcode as *mut ::core::ffi::c_char,
            b"SPINWAIT\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            let mut msecs_0: ::core::ffi::c_int = 0;
            let mut TimeOut: TimeCount = 0;
            if 1 as ::core::ffi::c_int
                == sscanf(
                    pStr,
                    b"%d\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut msecs_0,
                ) && (*pPIS).Skip.0 == 0
            {
                TimeOut = GetTimeCounter()
                    .wrapping_add(
                        (msecs_0 * 840 as ::core::ffi::c_int
                            / 1000 as ::core::ffi::c_int) as TimeCount,
                    );
                while GetTimeCounter() < TimeOut {
                    if CurrentInputState
                        .menu[C2Rust_Unnamed::KEY_MENU_CANCEL.0 as ::core::ffi::c_int
                        as usize] != 0
                        || GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
                            & C2Rust_Unnamed_1::CHECK_ABORT.0 as ::core::ffi::c_int != 0
                    {
                        Present_BatchGraphics(pPIS);
                        (*pPIS).Skip = BOOLEAN::TRUE;
                        return BOOLEAN::TRUE;
                    }
                    uqm_lua_checked!(SleepThread(
                        (840 as ::core::ffi::c_int / 84 as ::core::ffi::c_int)
                            as TimePeriod,
                    ), BOOLEAN(0));
                    uqm_lua_checked!(UpdateInputState(), BOOLEAN(0));
                }
            }
        } else if strcmp(
            &raw mut Opcode as *mut ::core::ffi::c_char,
            b"SYNC\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            let mut msecs_1: ::core::ffi::c_int = 0;
            Present_UnbatchGraphics(pPIS, BOOLEAN::TRUE);
            if 1 as ::core::ffi::c_int
                == sscanf(
                    pStr,
                    b"%d\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut msecs_1,
                )
            {
                (*pPIS).LastSyncTime = (*pPIS)
                    .StartTime
                    .wrapping_add(
                        (msecs_1 * 840 as ::core::ffi::c_int
                            / 1000 as ::core::ffi::c_int) as TimeCount,
                    );
                (*pPIS).TimeOut = (*pPIS).LastSyncTime;
                (*pPIS).TimeOutOnSkip = BOOLEAN::FALSE.0 as ::core::ffi::c_int;
                return BOOLEAN::TRUE;
            }
        } else if strcmp(
            &raw mut Opcode as *mut ::core::ffi::c_char,
            b"RESYNC\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*pPIS).StartTime = GetTimeCounter();
            (*pPIS).LastSyncTime = (*pPIS).StartTime;
        } else if strcmp(
            &raw mut Opcode as *mut ::core::ffi::c_char,
            b"DSYNC\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            let mut msecs_2: ::core::ffi::c_int = 0;
            Present_UnbatchGraphics(pPIS, BOOLEAN::TRUE);
            if 1 as ::core::ffi::c_int
                == sscanf(
                    pStr,
                    b"%d\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut msecs_2,
                )
            {
                (*pPIS).TimeOut = (*pPIS)
                    .LastSyncTime
                    .wrapping_add(
                        (msecs_2 * 840 as ::core::ffi::c_int
                            / 1000 as ::core::ffi::c_int) as TimeCount,
                    );
                (*pPIS).TimeOutOnSkip = BOOLEAN::FALSE.0 as ::core::ffi::c_int;
                return BOOLEAN::TRUE;
            }
        } else if strcmp(
            &raw mut Opcode as *mut ::core::ffi::c_char,
            b"BGC\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            let mut temp: Color = Color { r: 0, g: 0, b: 0, a: 0 };
            ParseColorString(pStr, &raw mut temp);
            SetContextBackGroundColor(temp);
        } else if strcmp(
            &raw mut Opcode as *mut ::core::ffi::c_char,
            b"TC\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            ParseColorString(pStr, &raw mut (*pPIS).TextColor);
        } else if strcmp(
            &raw mut Opcode as *mut ::core::ffi::c_char,
            b"TBC\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            ParseColorString(pStr, &raw mut (*pPIS).TextBackColor);
        } else if strcmp(
            &raw mut Opcode as *mut ::core::ffi::c_char,
            b"TFC\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            ParseColorString(pStr, &raw mut (*pPIS).TextFadeColor);
        } else if strcmp(
            &raw mut Opcode as *mut ::core::ffi::c_char,
            b"TVA\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*pPIS).TextVPos = toupper(*pStr as ::core::ffi::c_int);
        } else if strcmp(
            &raw mut Opcode as *mut ::core::ffi::c_char,
            b"TE\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*pPIS).TextEffect = toupper(*pStr as ::core::ffi::c_int);
        } else if strcmp(
            &raw mut Opcode as *mut ::core::ffi::c_char,
            b"TEXT\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            let mut x: ::core::ffi::c_int = 0;
            let mut y: ::core::ffi::c_int = 0;
            (::core::mem::size_of::<[::core::ffi::c_char; 512]>() >= 256usize
                || {
                    __assert_fail(
                        b"sizeof (pPIS->Buffer) >= 256\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"src/uqm/intro.c\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        979 as ::core::ffi::c_int,
                        b"DoPresentation\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    0 as ::core::ffi::c_int != 0
                }) as ::core::ffi::c_int;
            if 3 as ::core::ffi::c_int
                == sscanf(
                    pStr,
                    b"%d %d %255[^\n]\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut x,
                    &raw mut y,
                    &raw mut (*pPIS).Buffer as *mut ::core::ffi::c_char,
                )
            {
                let mut t: TEXT = TEXT {
                    baseline: POINT { x: 0, y: 0 },
                    pStr: ::core::ptr::null::<UNICODE>(),
                    align: TEXT_ALIGN::ALIGN_LEFT,
                    CharCount: 0,
                };
                t.align = TEXT_ALIGN::ALIGN_CENTER;
                t.pStr = &raw mut (*pPIS).Buffer as *mut ::core::ffi::c_char;
                t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
                t.baseline.x = (x << resolutionFactor) as COORD;
                t.baseline.y = (y << resolutionFactor) as COORD;
                uqm_lua_checked!(DrawTextEffect(
                    &raw mut t,
                    (*pPIS).TextColor,
                    (*pPIS).TextBackColor,
                    (*pPIS).TextEffect,
                ), BOOLEAN(0));
            }
        } else if strcmp(
            &raw mut Opcode as *mut ::core::ffi::c_char,
            b"TEXTSPIN\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            let mut x_0: ::core::ffi::c_int = 0;
            let mut y_0: ::core::ffi::c_int = 0;
            let mut n: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            (::core::mem::size_of::<[::core::ffi::c_char; 512]>() >= 256usize
                || {
                    __assert_fail(
                        b"sizeof (pPIS->Buffer) >= 256\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"src/uqm/intro.c\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        999 as ::core::ffi::c_int,
                        b"DoPresentation\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    0 as ::core::ffi::c_int != 0
                }) as ::core::ffi::c_int;
            if 2 as ::core::ffi::c_int
                == sscanf(
                    pStr,
                    b"%d %d %n\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut x_0,
                    &raw mut y_0,
                    &raw mut n,
                )
            {
                if optShipSeed.0 != 0 {
                    SeedTextSpin(
                        &raw mut (*pPIS).Buffer as *mut ::core::ffi::c_char,
                        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                        pStr.offset(n as isize),
                        &raw mut x_0,
                        &raw mut y_0,
                    );
                } else {
                    utf8StringCopy(
                        &raw mut (*pPIS).Buffer as *mut ::core::ffi::c_char,
                        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                        pStr.offset(n as isize),
                    );
                }
                x_0 <<= resolutionFactor;
                y_0 <<= resolutionFactor;
                if (*pPIS).HaveFrame.0 != 0
                    && ((*pPIS).GetRect.extent.width as ::core::ffi::c_int
                        > 0 as ::core::ffi::c_int
                        && (*pPIS).GetRect.extent.height as ::core::ffi::c_int
                            > 0 as ::core::ffi::c_int)
                {
                    x_0 += (*pPIS).GetRect.corner.x as ::core::ffi::c_int;
                    y_0 += (*pPIS).GetRect.corner.y as ::core::ffi::c_int;
                }
                SetContextForeGroundColor((*pPIS).TextColor);
                SetContextBackGroundColor((*pPIS).TextBackColor);
                if optShipSeed.0 != 0
                    && strncmp(
                        &raw mut (*pPIS).Buffer as *mut ::core::ffi::c_char,
                        b"SPATHI\0".as_ptr() as *const ::core::ffi::c_char,
                        6 as size_t,
                    ) == 0
                {
                    uqm_lua_checked!(DoSpinText(
                        b"SP\0".as_ptr() as *const ::core::ffi::c_char as *mut UNICODE,
                        x_0 as COORD,
                        (y_0 + ((7 as ::core::ffi::c_int) << resolutionFactor)) as COORD,
                        SetAbsFrameIndex((*pPIS).Frame, 0 as COUNT),
                        &raw mut (*pPIS).Skip,
                    ), BOOLEAN(0));
                    uqm_lua_checked!(DoSpinText(
                        b"A\0".as_ptr() as *const ::core::ffi::c_char as *mut UNICODE,
                        (x_0 + ((16 as ::core::ffi::c_int) << resolutionFactor))
                            as COORD,
                        (y_0 + ((7 as ::core::ffi::c_int) << resolutionFactor)) as COORD,
                        SetAbsFrameIndex((*pPIS).Frame, 0 as COUNT),
                        &raw mut (*pPIS).Skip,
                    ), BOOLEAN(0));
                    uqm_lua_checked!(DoSpinText(
                        b"THI\0".as_ptr() as *const ::core::ffi::c_char as *mut UNICODE,
                        (x_0 + ((25 as ::core::ffi::c_int) << resolutionFactor))
                            as COORD,
                        (y_0 + ((7 as ::core::ffi::c_int) << resolutionFactor)) as COORD,
                        SetAbsFrameIndex((*pPIS).Frame, 0 as COUNT),
                        &raw mut (*pPIS).Skip,
                    ), BOOLEAN(0));
                    if ship_map[shipID as usize].sID.0 == SPECIES_ID::UR_QUAN_ID.0 {
                        x_0 -= (52 as ::core::ffi::c_int) << resolutionFactor;
                        y_0 += (11 as ::core::ffi::c_int) << resolutionFactor;
                    }
                    uqm_lua_checked!(DoSpinText(
                        (&raw mut (*pPIS).Buffer as *mut ::core::ffi::c_char)
                            .offset(6isize),
                        (x_0 + ((52 as ::core::ffi::c_int) << resolutionFactor))
                            as COORD,
                        (y_0 + ((7 as ::core::ffi::c_int) << resolutionFactor)) as COORD,
                        SetAbsFrameIndex((*pPIS).Frame, 0 as COUNT),
                        &raw mut (*pPIS).Skip,
                    ), BOOLEAN(0));
                } else {
                    uqm_lua_checked!(DoSpinText(
                        &raw mut (*pPIS).Buffer as *mut UNICODE,
                        x_0 as COORD,
                        (y_0 + ((7 as ::core::ffi::c_int) << resolutionFactor)) as COORD,
                        SetAbsFrameIndex((*pPIS).Frame, 0 as COUNT),
                        &raw mut (*pPIS).Skip,
                    ), BOOLEAN(0));
                }
                if (*pPIS).Skip.0 != 0 {
                    Present_BatchGraphics(pPIS);
                }
            }
        } else if strcmp(
            &raw mut Opcode as *mut ::core::ffi::c_char,
            b"SPINSTAT\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            let mut x_1: ::core::ffi::c_int = 0;
            let mut y_1: ::core::ffi::c_int = 0;
            let mut f: ::core::ffi::c_int = 0;
            let mut e: ::core::ffi::c_int = 0;
            let mut leading: SIZE = 0;
            (::core::mem::size_of::<[::core::ffi::c_char; 512]>() >= 256usize
                || {
                    __assert_fail(
                        b"sizeof (pPIS->Buffer) >= 256\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"src/uqm/intro.c\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        1057 as ::core::ffi::c_int,
                        b"DoPresentation\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    0 as ::core::ffi::c_int != 0
                }) as ::core::ffi::c_int;
            if 3 as ::core::ffi::c_int
                == sscanf(
                    pStr,
                    b"%d %d %255[^\n]\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut f,
                    &raw mut e,
                    &raw mut (*pPIS).Buffer as *mut ::core::ffi::c_char,
                )
            {
                GetContextFontLeading(&raw mut leading);
                (*pPIS).NumSpinStat = (*pPIS).NumSpinStat.wrapping_add(1);
                x_1 = (*pPIS).StatBox.corner.x as ::core::ffi::c_int
                    + ((3 as ::core::ffi::c_int) << resolutionFactor);
                y_1 = (*pPIS).StatBox.corner.y as ::core::ffi::c_int
                    + ((1 as ::core::ffi::c_int) << resolutionFactor)
                    + leading as ::core::ffi::c_int
                        * (*pPIS).NumSpinStat as ::core::ffi::c_int;
                if (*pPIS).NumSpinStat as ::core::ffi::c_int > 8 as ::core::ffi::c_int {
                    log_add(
                        log_Level::log_Warning,
                        b"SPINSTAT: Number of SPINSTAT entries exceeds max amount '%s'\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        pStr,
                    );
                    return BOOLEAN::FALSE;
                }
                if f > 9 as ::core::ffi::c_int || f + e > 9 as ::core::ffi::c_int {
                    let mut buf: [::core::ffi::c_char; 524] = [0; 524];
                    let mut t_0: TEXT = TEXT {
                        baseline: POINT { x: 0, y: 0 },
                        pStr: ::core::ptr::null::<UNICODE>(),
                        align: TEXT_ALIGN::ALIGN_LEFT,
                        CharCount: 0,
                    };
                    log_add(
                        log_Level::log_Warning,
                        b"SPINSTAT: Stats exceed max values '%s'\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        pStr,
                    );
                    snprintf(
                        &raw mut buf as *mut ::core::ffi::c_char,
                        ::core::mem::size_of::<[::core::ffi::c_char; 524]>(),
                        b"%s %s\0".as_ptr() as *const ::core::ffi::c_char,
                        &raw mut (*pPIS).Buffer as *mut ::core::ffi::c_char,
                        b"Exceed max!\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    t_0.align = TEXT_ALIGN::ALIGN_LEFT;
                    t_0.pStr = &raw mut buf as *mut ::core::ffi::c_char;
                    t_0.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
                    t_0.baseline = MAKE_POINT(x_1 as COORD, y_1 as COORD);
                    uqm_lua_checked!(DrawTextEffect(
                        &raw mut t_0,
                        buildColorRgba(
                            0xff as BYTE,
                            0x55 as BYTE,
                            0x55 as BYTE,
                            0xff as BYTE,
                        ),
                        (*pPIS).TextBackColor,
                        (*pPIS).TextEffect,
                    ), BOOLEAN(0));
                } else {
                    uqm_lua_checked!(DoSpinStat(
                        &raw mut (*pPIS).Buffer as *mut UNICODE,
                        x_1 as COORD,
                        y_1 as COORD,
                        f as COUNT,
                        e as COUNT,
                        (*pPIS).TextColor,
                        (*pPIS).TextBackColor,
                        &raw mut (*pPIS).Skip,
                    ), BOOLEAN(0));
                    if (*pPIS).Skip.0 != 0 {
                        Present_BatchGraphics(pPIS);
                    }
                }
            } else {
                log_add(
                    log_Level::log_Warning,
                    b"Bad SPINSTAT command '%s'\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    pStr,
                );
            }
        } else if strcmp(
            &raw mut Opcode as *mut ::core::ffi::c_char,
            b"TFI\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            let mut leading_0: SIZE = 0;
            let mut i: COUNT = 0;
            let mut y_2: COORD = 0;
            utf8StringCopy(
                &raw mut (*pPIS).Buffer as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                pStr,
            );
            (*pPIS).LinesCount = ParseTextLines(
                &raw mut (*pPIS).TextLines as *mut TEXT,
                15 as COUNT,
                &raw mut (*pPIS).Buffer as *mut ::core::ffi::c_char,
            );
            Present_UnbatchGraphics(pPIS, BOOLEAN::TRUE);
            GetContextFontLeading(&raw mut leading_0);
            match (*pPIS).TextVPos {
                84 => {
                    y_2 = (leading_0 as ::core::ffi::c_int
                        / (*pPIS).LinesCount as ::core::ffi::c_int
                        + leading_0 as ::core::ffi::c_int) as COORD;
                }
                77 => {
                    y_2 = (((*pPIS).clip_r.extent.height as ::core::ffi::c_int
                        - (*pPIS).LinesCount as ::core::ffi::c_int
                            * leading_0 as ::core::ffi::c_int) / 2 as ::core::ffi::c_int)
                        as COORD;
                }
                _ => {
                    y_2 = ((*pPIS).clip_r.extent.height as ::core::ffi::c_int
                        - (*pPIS).LinesCount as ::core::ffi::c_int
                            * leading_0 as ::core::ffi::c_int) as COORD;
                }
            }
            (*pPIS).tfade_r = (*pPIS).clip_r;
            (*pPIS).tfade_r.corner.y = 0 as COORD;
            (*pPIS).tfade_r.extent.height = CanvasHeight as COORD;
            i = 0 as COUNT;
            while (i as ::core::ffi::c_int) < (*pPIS).LinesCount as ::core::ffi::c_int {
                (*pPIS).TextLines[i as usize].align = TEXT_ALIGN::ALIGN_CENTER;
                (*pPIS).TextLines[i as usize].baseline.x = (CanvasWidth
                    / 2 as ::core::ffi::c_int) as COORD;
                (*pPIS).TextLines[i as usize].baseline.y = y_2;
                i = i.wrapping_add(1);
                y_2 = (y_2 as ::core::ffi::c_int + leading_0 as ::core::ffi::c_int)
                    as COORD;
            }
            i = 0 as COUNT;
            while (i as ::core::ffi::c_int) < (*pPIS).LinesCount as ::core::ffi::c_int {
                uqm_lua_checked!(DrawTextEffect(
                    (&raw mut (*pPIS).TextLines as *mut TEXT)
                        .offset(i as ::core::ffi::c_int as isize),
                    (*pPIS).TextFadeColor,
                    (*pPIS).TextFadeColor,
                    (*pPIS).TextEffect,
                ), BOOLEAN(0));
                i = i.wrapping_add(1);
            }
            SetTransitionSource(&raw mut (*pPIS).tfade_r);
            BatchGraphics();
            i = 0 as COUNT;
            while (i as ::core::ffi::c_int) < (*pPIS).LinesCount as ::core::ffi::c_int {
                uqm_lua_checked!(DrawTextEffect(
                    (&raw mut (*pPIS).TextLines as *mut TEXT)
                        .offset(i as ::core::ffi::c_int as isize),
                    (*pPIS).TextColor,
                    (*pPIS).TextBackColor,
                    (*pPIS).TextEffect,
                ), BOOLEAN(0));
                i = i.wrapping_add(1);
            }
            uqm_lua_checked!(ScreenTransition(3 as ::core::ffi::c_int, &raw mut (*pPIS).tfade_r), BOOLEAN(0));
            UnbatchGraphics();
        } else if strcmp(
            &raw mut Opcode as *mut ::core::ffi::c_char,
            b"TFO\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            let mut i_0: COUNT = 0;
            Present_UnbatchGraphics(pPIS, BOOLEAN::TRUE);
            SetTransitionSource(&raw mut (*pPIS).tfade_r);
            BatchGraphics();
            i_0 = 0 as COUNT;
            while (i_0 as ::core::ffi::c_int) < (*pPIS).LinesCount as ::core::ffi::c_int
            {
                uqm_lua_checked!(DrawTextEffect(
                    (&raw mut (*pPIS).TextLines as *mut TEXT)
                        .offset(i_0 as ::core::ffi::c_int as isize),
                    (*pPIS).TextFadeColor,
                    (*pPIS).TextFadeColor,
                    (*pPIS).TextEffect,
                ), BOOLEAN(0));
                i_0 = i_0.wrapping_add(1);
            }
            uqm_lua_checked!(ScreenTransition(3 as ::core::ffi::c_int, &raw mut (*pPIS).tfade_r), BOOLEAN(0));
            UnbatchGraphics();
        } else if strcmp(
            &raw mut Opcode as *mut ::core::ffi::c_char,
            b"SAVEBG\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            TFB_DrawScreen_Copy(
                &raw mut (*pPIS).clip_r,
                SCREEN::TFB_SCREEN_MAIN,
                SCREEN::TFB_SCREEN_EXTRA,
            );
        } else if strcmp(
            &raw mut Opcode as *mut ::core::ffi::c_char,
            b"RESTBG\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            TFB_DrawScreen_Copy(
                &raw mut (*pPIS).clip_r,
                SCREEN::TFB_SCREEN_EXTRA,
                SCREEN::TFB_SCREEN_MAIN,
            );
        } else if strcmp(
            &raw mut Opcode as *mut ::core::ffi::c_char,
            b"DRAW\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            let mut cargs: ::core::ffi::c_int = 0;
            let mut draw_what: ::core::ffi::c_int = 0;
            let mut index_2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut x_2: ::core::ffi::c_int = 0;
            let mut y_3: ::core::ffi::c_int = 0;
            let mut scale: ::core::ffi::c_int = 0;
            let mut angle: ::core::ffi::c_int = 0;
            let mut scale_mode: ::core::ffi::c_int = 0;
            let mut ImgName: [::core::ffi::c_char; 16] = [0; 16];
            let mut old_scale: ::core::ffi::c_int = 0;
            let mut old_mode: ::core::ffi::c_int = 0;
            let mut s: STAMP = STAMP {
                origin: POINT { x: 0, y: 0 },
                frame: ::core::ptr::null_mut::<FRAME_DESC>(),
            };
            if 1 as ::core::ffi::c_int
                == sscanf(
                    pStr,
                    b"%15s\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut ImgName as *mut ::core::ffi::c_char,
                )
                && strcmp(
                    strupr(&raw mut ImgName as *mut ::core::ffi::c_char),
                    b"SIS\0".as_ptr() as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
            {
                draw_what = 1 as ::core::ffi::c_int;
                scale_mode = C2Rust_Unnamed_2::TFB_SCALE_NEAREST.0 as ::core::ffi::c_int;
                cargs = sscanf(
                    pStr,
                    b"%*s %d %d %d %d\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut x_2,
                    &raw mut y_3,
                    &raw mut scale,
                    &raw mut angle,
                ) + 1 as ::core::ffi::c_int;
            } else {
                draw_what = 0 as ::core::ffi::c_int;
                scale_mode = C2Rust_Unnamed_2::TFB_SCALE_BILINEAR.0
                    as ::core::ffi::c_int;
                cargs = sscanf(
                    pStr,
                    b"%d %d %d %d %d\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut index_2,
                    &raw mut x_2,
                    &raw mut y_3,
                    &raw mut scale,
                    &raw mut angle,
                );
            }
            if cargs < 1 as ::core::ffi::c_int {
                log_add(
                    log_Level::log_Warning,
                    b"Bad DRAW command '%s'\0".as_ptr() as *const ::core::ffi::c_char,
                    pStr,
                );
                (*pPIS).HaveFrame = BOOLEAN::FALSE;
            } else {
                if cargs < 5 as ::core::ffi::c_int {
                    angle = 0 as ::core::ffi::c_int;
                }
                if cargs < 4 as ::core::ffi::c_int {
                    scale = 256 as ::core::ffi::c_int;
                }
                if cargs < 3 as ::core::ffi::c_int {
                    x_2 = 0 as ::core::ffi::c_int;
                    y_3 = 0 as ::core::ffi::c_int;
                }
                x_2 <<= resolutionFactor;
                y_3 <<= resolutionFactor;
                s.frame = ::core::ptr::null_mut::<FRAME_DESC>();
                if draw_what == 0 as ::core::ffi::c_int {
                    s.frame = SetAbsFrameIndex((*pPIS).Frame, index_2 as COUNT);
                    (*pPIS).CurrentFrameIndex = index_2 as COUNT;
                    (*pPIS).HaveFrame = BOOLEAN::TRUE;
                } else if draw_what == 1 as ::core::ffi::c_int {
                    if (*pPIS).SisFrame.is_null() {
                        uqm_lua_checked!(Present_GenerateSIS(pPIS), BOOLEAN(0));
                    }
                    s.frame = SetAbsFrameIndex((*pPIS).SisFrame, 0 as COUNT);
                }
                if angle != 0 as ::core::ffi::c_int {
                    if angle != (*pPIS).LastAngle || draw_what != (*pPIS).LastDrawKind {
                        DestroyDrawable(ReleaseDrawable((*pPIS).RotatedFrame));
                        (*pPIS).RotatedFrame = CaptureDrawable(
                            RotateFrame(s.frame, -angle),
                        );
                        (*pPIS).LastAngle = angle;
                        (*pPIS).LastDrawKind = draw_what;
                    }
                    s.frame = (*pPIS).RotatedFrame;
                }
                s.origin.x = x_2 as COORD;
                s.origin.y = y_3 as COORD;
                old_mode = SetGraphicScaleMode(scale_mode);
                old_scale = SetGraphicScale(scale);
                uqm_lua_checked!(DrawStamp(&raw mut s), BOOLEAN(0));
                SetGraphicScale(old_scale);
                SetGraphicScaleMode(old_mode);
            }
        } else if strcmp(
            &raw mut Opcode as *mut ::core::ffi::c_char,
            b"BATCH\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            Present_BatchGraphics(pPIS);
        } else if strcmp(
            &raw mut Opcode as *mut ::core::ffi::c_char,
            b"UNBATCH\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            Present_UnbatchGraphics(pPIS, BOOLEAN::FALSE);
        } else if strcmp(
            &raw mut Opcode as *mut ::core::ffi::c_char,
            b"FTC\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            Present_UnbatchGraphics(pPIS, BOOLEAN::TRUE);
            return DoFadeScreen(
                pPIS,
                pStr,
                ScreenFadeType::FadeAllToColor.0 as ::core::ffi::c_int as BYTE,
            );
        } else if strcmp(
            &raw mut Opcode as *mut ::core::ffi::c_char,
            b"FTB\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            Present_UnbatchGraphics(pPIS, BOOLEAN::TRUE);
            return DoFadeScreen(
                pPIS,
                pStr,
                ScreenFadeType::FadeAllToBlack.0 as ::core::ffi::c_int as BYTE,
            );
        } else if strcmp(
            &raw mut Opcode as *mut ::core::ffi::c_char,
            b"FTW\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            Present_UnbatchGraphics(pPIS, BOOLEAN::TRUE);
            return DoFadeScreen(
                pPIS,
                pStr,
                ScreenFadeType::FadeAllToWhite.0 as ::core::ffi::c_int as BYTE,
            );
        } else if strcmp(
            &raw mut Opcode as *mut ::core::ffi::c_char,
            b"CLS\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            Present_UnbatchGraphics(pPIS, BOOLEAN::TRUE);
            uqm_lua_checked!(ClearScreen(), BOOLEAN(0));
        } else if strcmp(
            &raw mut Opcode as *mut ::core::ffi::c_char,
            b"CALL\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            Present_UnbatchGraphics(pPIS, BOOLEAN::TRUE);
            utf8StringCopy(
                &raw mut (*pPIS).Buffer as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                pStr,
            );
            uqm_lua_checked!(ShowPresentationFile(&raw mut (*pPIS).Buffer as *mut ::core::ffi::c_char), BOOLEAN(0));
        } else if strcmp(
            &raw mut Opcode as *mut ::core::ffi::c_char,
            b"LINE\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            let mut x1: ::core::ffi::c_int = 0;
            let mut x2: ::core::ffi::c_int = 0;
            let mut y1: ::core::ffi::c_int = 0;
            let mut y2: ::core::ffi::c_int = 0;
            if 4 as ::core::ffi::c_int
                == sscanf(
                    pStr,
                    b"%d %d %d %d\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut x1,
                    &raw mut y1,
                    &raw mut x2,
                    &raw mut y2,
                )
            {
                let mut l: LINE = LINE {
                    first: POINT { x: 0, y: 0 },
                    second: POINT { x: 0, y: 0 },
                };
                l.first.x = (x1 << resolutionFactor) as COORD;
                l.first.y = (y1 << resolutionFactor) as COORD;
                l.second.x = (x2 << resolutionFactor) as COORD;
                l.second.y = (y2 << resolutionFactor) as COORD;
                SetContextForeGroundColor((*pPIS).TextColor);
                uqm_lua_checked!(DrawLine(&raw mut l, 1 as BYTE), BOOLEAN(0));
            } else {
                log_add(
                    log_Level::log_Warning,
                    b"Bad LINE command '%s'\0".as_ptr() as *const ::core::ffi::c_char,
                    pStr,
                );
            }
        } else if strcmp(
            &raw mut Opcode as *mut ::core::ffi::c_char,
            b"LINESPIN\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            let mut x1_0: ::core::ffi::c_int = 0;
            let mut x2_0: ::core::ffi::c_int = 0;
            let mut y1_0: ::core::ffi::c_int = 0;
            let mut y2_0: ::core::ffi::c_int = 0;
            if 4 as ::core::ffi::c_int
                == sscanf(
                    pStr,
                    b"%d %d %d %d\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut x1_0,
                    &raw mut y1_0,
                    &raw mut x2_0,
                    &raw mut y2_0,
                )
            {
                if optShipSeed.0 != 0 {
                    SeedLineSpin(
                        &raw mut x1_0,
                        &raw mut y1_0,
                        &raw mut x2_0,
                        &raw mut y2_0,
                    );
                }
                let mut l_0: LINE = LINE {
                    first: POINT { x: 0, y: 0 },
                    second: POINT { x: 0, y: 0 },
                };
                x1_0 <<= resolutionFactor;
                x2_0 <<= resolutionFactor;
                y1_0 <<= resolutionFactor;
                y2_0 <<= resolutionFactor;
                if (*pPIS).HaveFrame.0 != 0
                    && ((*pPIS).GetRect.extent.width as ::core::ffi::c_int
                        > 0 as ::core::ffi::c_int
                        && (*pPIS).GetRect.extent.height as ::core::ffi::c_int
                            > 0 as ::core::ffi::c_int)
                {
                    x1_0 += (*pPIS).GetRect.corner.x as ::core::ffi::c_int;
                    x2_0 += (*pPIS).GetRect.corner.x as ::core::ffi::c_int;
                    y1_0 += (*pPIS).GetRect.corner.y as ::core::ffi::c_int;
                    y2_0 += (*pPIS).GetRect.corner.y as ::core::ffi::c_int;
                }
                l_0.first.x = x1_0 as COORD;
                l_0.first.y = y1_0 as COORD;
                l_0.second.x = x2_0 as COORD;
                l_0.second.y = y2_0 as COORD;
                uqm_lua_checked!(DoSpinLine(
                    &raw mut l_0,
                    (*pPIS).TextColor,
                    (*pPIS).TextBackColor,
                    &raw mut (*pPIS).Skip,
                ), BOOLEAN(0));
                if (*pPIS).Skip.0 != 0 {
                    Present_BatchGraphics(pPIS);
                }
            } else {
                log_add(
                    log_Level::log_Warning,
                    b"Bad LINESPIN command '%s'\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    pStr,
                );
            }
        } else if strcmp(
            &raw mut Opcode as *mut ::core::ffi::c_char,
            b"GETRECT\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            if (*pPIS).HaveFrame.0 != 0 {
                GetFrameRect(
                    SetAbsFrameIndex((*pPIS).Frame, (*pPIS).CurrentFrameIndex),
                    &raw mut (*pPIS).GetRect,
                );
            } else {
                log_add(
                    log_Level::log_Warning,
                    b"Bad GETRECT command, can not use GETRECT without drawing a frame first '%s'\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    pStr,
                );
            }
        } else if strcmp(
            &raw mut Opcode as *mut ::core::ffi::c_char,
            b"STATBOX\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            let mut x_3: ::core::ffi::c_int = 0;
            let mut y_4: ::core::ffi::c_int = 0;
            if 2 as ::core::ffi::c_int
                == sscanf(
                    pStr,
                    b"%d %d\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut x_3,
                    &raw mut y_4,
                )
            {
                (*pPIS).NumSpinStat = 0 as COUNT;
                x_3 <<= resolutionFactor;
                y_4 <<= resolutionFactor;
                if (*pPIS).HaveFrame.0 != 0
                    && ((*pPIS).GetRect.extent.width as ::core::ffi::c_int
                        > 0 as ::core::ffi::c_int
                        && (*pPIS).GetRect.extent.height as ::core::ffi::c_int
                            > 0 as ::core::ffi::c_int)
                {
                    x_3 += (*pPIS).GetRect.corner.x as ::core::ffi::c_int;
                    y_4 += (*pPIS).GetRect.corner.y as ::core::ffi::c_int;
                }
                (*pPIS).StatBox.corner = MAKE_POINT(x_3 as COORD, y_4 as COORD);
                (*pPIS).StatBox.extent = MAKE_EXTENT(
                    ((122 as ::core::ffi::c_int) << resolutionFactor) as COORD,
                    ((60 as ::core::ffi::c_int) << resolutionFactor) as COORD,
                );
                uqm_lua_checked!(DoSpinStatBox(
                    &raw mut (*pPIS).StatBox,
                    (*pPIS).TextColor,
                    (*pPIS).TextBackColor,
                    &raw mut (*pPIS).Skip,
                ), BOOLEAN(0));
                if (*pPIS).Skip.0 != 0 {
                    Present_BatchGraphics(pPIS);
                }
            } else {
                log_add(
                    log_Level::log_Warning,
                    b"Bad STATBOX command '%s'\0".as_ptr() as *const ::core::ffi::c_char,
                    pStr,
                );
            }
        } else if strcmp(
            &raw mut Opcode as *mut ::core::ffi::c_char,
            b"MOVIE\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            let mut fps: ::core::ffi::c_int = 0;
            let mut from: ::core::ffi::c_int = 0;
            let mut to: ::core::ffi::c_int = 0;
            if 3 as ::core::ffi::c_int
                == sscanf(
                    pStr,
                    b"%d %d %d\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut fps,
                    &raw mut from,
                    &raw mut to,
                ) && fps > 0 as ::core::ffi::c_int && from >= 0 as ::core::ffi::c_int
                && to >= 0 as ::core::ffi::c_int && to >= from
            {
                Present_UnbatchGraphics(pPIS, BOOLEAN::TRUE);
                (*pPIS).MovieFrame = from;
                (*pPIS).MovieEndFrame = to;
                (*pPIS).InterframeDelay = 840 as ::core::ffi::c_int / fps;
                (*pPIS).TimeOut = GetTimeCounter();
                (*pPIS).TimeOutOnSkip = BOOLEAN::TRUE.0 as ::core::ffi::c_int;
                return BOOLEAN::TRUE;
            } else {
                log_add(
                    log_Level::log_Warning,
                    b"Bad MOVIE command '%s'\0".as_ptr() as *const ::core::ffi::c_char,
                    pStr,
                );
            }
        } else if strcmp(
            &raw mut Opcode as *mut ::core::ffi::c_char,
            b"ANIMATE\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            let mut first_frame: ::core::ffi::c_int = 0;
            let mut last_frame: ::core::ffi::c_int = 0;
            let mut num_loops: ::core::ffi::c_int = 0;
            let mut milliseconds: ::core::ffi::c_int = 0;
            let mut fps_0: ::core::ffi::c_int = 0;
            if 5 as ::core::ffi::c_int
                == sscanf(
                    pStr,
                    b"%d %d %d %d %d\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut first_frame,
                    &raw mut last_frame,
                    &raw mut num_loops,
                    &raw mut milliseconds,
                    &raw mut fps_0,
                )
            {
                let mut s_0: STAMP = STAMP {
                    origin: POINT { x: 0, y: 0 },
                    frame: ::core::ptr::null_mut::<FRAME_DESC>(),
                };
                let mut loops: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                let mut index_3: COUNT = 0 as COUNT;
                let mut Now: TimeCount = 0;
                let mut timeout: TimeCount = 0;
                let mut NextTime: TimeCount = 0;
                let mut animation_rate: ::core::ffi::c_int = 840 as ::core::ffi::c_int
                    / fps_0;
                s_0.origin.x = 0 as COORD;
                s_0.origin.y = 0 as COORD;
                timeout = GetTimeCounter().wrapping_add(milliseconds as TimeCount);
                NextTime = GetTimeCounter().wrapping_add(animation_rate as TimeCount);
                while num_loops != 0 || milliseconds != 0 {
                    Now = GetTimeCounter();
                    if uqm_lua_checked!(ActKeysPress(), BOOLEAN(0)).0 != 0 {
                        break;
                    }
                    if Now < NextTime {
                        continue;
                    }
                    s_0.frame = SetAbsFrameIndex((*pPIS).Frame, index_3);
                    uqm_lua_checked!(DrawStamp(&raw mut s_0), BOOLEAN(0));
                    index_3 = index_3.wrapping_add(1);
                    if index_3 as ::core::ffi::c_int == last_frame {
                        loops += 1;
                        index_3 = first_frame as COUNT;
                    }
                    if num_loops > 0 as ::core::ffi::c_int && loops == num_loops {
                        break;
                    }
                    if Now >= timeout {
                        break;
                    }
                    NextTime = Now.wrapping_add(animation_rate as TimeCount);
                }
                return BOOLEAN::TRUE;
            } else {
                log_add(
                    log_Level::log_Warning,
                    b"Bad ANIMATION command '%s'\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    pStr,
                );
            }
        } else {
            strcmp(
                &raw mut Opcode as *mut ::core::ffi::c_char,
                b"NOOP\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int;
        }
    }
    return BOOLEAN::FALSE;
}
unsafe extern "C" fn ShowSlidePresentation(mut PresStr: STRING) -> BOOLEAN {
    let mut OldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    let mut OldFont: FONT = ::core::ptr::null_mut::<FONT_DESC>();
    let mut OldRect: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut pis: PRESENTATION_INPUT_STATE = PRESENTATION_INPUT_STATE {
        InputFunc: None,
        StartTime: 0,
        LastSyncTime: 0,
        TimeOut: 0,
        TimeOutOnSkip: 0,
        SlideShow: ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>(),
        Fonts: [::core::ptr::null_mut::<FONT_DESC>(); 5],
        Frame: ::core::ptr::null_mut::<FRAME_DESC>(),
        MusicRef: ::core::ptr::null_mut::<*mut TFB_SoundSample>(),
        Batched: BOOLEAN::FALSE,
        SisFrame: ::core::ptr::null_mut::<FRAME_DESC>(),
        RotatedFrame: ::core::ptr::null_mut::<FRAME_DESC>(),
        LastDrawKind: 0,
        LastAngle: 0,
        OperIndex: 0,
        TextFadeColor: Color { r: 0, g: 0, b: 0, a: 0 },
        TextColor: Color { r: 0, g: 0, b: 0, a: 0 },
        TextBackColor: Color { r: 0, g: 0, b: 0, a: 0 },
        TextVPos: 0,
        TextEffect: 0,
        clip_r: RECT {
            corner: POINT { x: 0, y: 0 },
            extent: EXTENT { width: 0, height: 0 },
        },
        tfade_r: RECT {
            corner: POINT { x: 0, y: 0 },
            extent: EXTENT { width: 0, height: 0 },
        },
        TextLines: [TEXT {
            baseline: POINT { x: 0, y: 0 },
            pStr: ::core::ptr::null::<UNICODE>(),
            align: TEXT_ALIGN::ALIGN_LEFT,
            CharCount: 0,
        }; 15],
        LinesCount: 0,
        Buffer: [0; 512],
        MovieFrame: 0,
        MovieEndFrame: 0,
        InterframeDelay: 0,
        StatBox: RECT {
            corner: POINT { x: 0, y: 0 },
            extent: EXTENT { width: 0, height: 0 },
        },
        NumSpinStat: 0,
        GetRect: RECT {
            corner: POINT { x: 0, y: 0 },
            extent: EXTENT { width: 0, height: 0 },
        },
        CurrentFrameIndex: 0,
        HaveFrame: BOOLEAN::FALSE,
        Skip: BOOLEAN::FALSE,
    };
    let mut i: ::core::ffi::c_int = 0;
    memset(
        &raw mut pis as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<PRESENTATION_INPUT_STATE>(),
    );
    pis.SlideShow = PresStr;
    if pis.SlideShow.is_null() {
        return BOOLEAN::FALSE;
    }
    pis.SlideShow = SetAbsStringTableIndex(pis.SlideShow, 0 as COUNT);
    pis.OperIndex = 0 as COUNT;
    OldContext = SetContext(ScreenContext);
    GetContextClipRect(&raw mut OldRect);
    OldFont = SetContextFont(::core::ptr::null_mut::<FONT_DESC>());
    SetContextBackGroundColor(colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE));
    SetMenuSounds(
        0 as ::core::ffi::c_int as MENU_SOUND_FLAGS,
        0 as ::core::ffi::c_int as MENU_SOUND_FLAGS,
    );
    pis.InputFunc = Some(
        DoPresentation as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> BOOLEAN,
    ) as Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> BOOLEAN>;
    pis.LastDrawKind = -1 as ::core::ffi::c_int;
    pis.TextVPos = 'B' as ::core::ffi::c_int;
    pis.MovieFrame = -1 as ::core::ffi::c_int;
    pis.StartTime = GetTimeCounter();
    pis.LastSyncTime = pis.StartTime;
    uqm_lua_checked!(DoInput(&raw mut pis as *mut ::core::ffi::c_void, BOOLEAN::TRUE), BOOLEAN(0));
    if !pis.MusicRef.is_null()
        && PlayingStream(
                (0 as ::core::ffi::c_int
                    + (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint32,
            )
            .0 != 0
    {
        uqm_lua_checked!(SleepThreadUntil(uqm_lua_checked!(FadeMusic(0 as BYTE, 840 as SIZE), BOOLEAN(0))), BOOLEAN(0));
        uqm_lua_checked!(StopMusic(), BOOLEAN(0));
        uqm_lua_checked!(FadeMusic(160 as BYTE, 0 as SIZE), BOOLEAN(0));
    }
    uqm_lua_checked!(DestroyMusic(pis.MusicRef), BOOLEAN(0));
    DestroyDrawable(ReleaseDrawable(pis.RotatedFrame));
    DestroyDrawable(ReleaseDrawable(pis.Frame));
    i = 0 as ::core::ffi::c_int;
    while i < 5 as ::core::ffi::c_int {
        DestroyFont(pis.Fonts[i as usize]);
        i += 1;
    }
    SetContextFont(OldFont);
    SetContextClipRect(&raw mut OldRect);
    SetContext(OldContext);
    return BOOLEAN::TRUE;
}
unsafe extern "C" fn DoVideoInput(mut pIS: *mut ::core::ffi::c_void) -> BOOLEAN {
    let mut pVIS: *mut VIDEO_INPUT_STATE = pIS as *mut VIDEO_INPUT_STATE;
    if PlayingLegacyVideo((*pVIS).CurVideo).0 == 0 {
        return BOOLEAN::FALSE;
    }
    if PulsedInputState
        .menu[C2Rust_Unnamed::KEY_MENU_SELECT.0 as ::core::ffi::c_int as usize] != 0
        || PulsedInputState
            .menu[C2Rust_Unnamed::KEY_MENU_CANCEL.0 as ::core::ffi::c_int as usize] != 0
        || PulsedInputState
            .menu[C2Rust_Unnamed::KEY_MENU_SPECIAL.0 as ::core::ffi::c_int as usize] != 0
        || GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
            & C2Rust_Unnamed_1::CHECK_ABORT.0 as ::core::ffi::c_int != 0
    {
        return BOOLEAN::FALSE
    } else if PulsedInputState
        .menu[C2Rust_Unnamed::KEY_MENU_LEFT.0 as ::core::ffi::c_int as usize] != 0
        || PulsedInputState
            .menu[C2Rust_Unnamed::KEY_MENU_RIGHT.0 as ::core::ffi::c_int as usize] != 0
    {
        let mut newpos: SDWORD = VidGetPosition() as SDWORD;
        if PulsedInputState
            .menu[C2Rust_Unnamed::KEY_MENU_LEFT.0 as ::core::ffi::c_int as usize] != 0
        {
            newpos -= 2000 as ::core::ffi::c_int;
        } else if PulsedInputState
            .menu[C2Rust_Unnamed::KEY_MENU_RIGHT.0 as ::core::ffi::c_int as usize] != 0
        {
            newpos += 1000 as ::core::ffi::c_int;
        }
        if newpos < 0 as ::core::ffi::c_int {
            newpos = 0 as ::core::ffi::c_int as SDWORD;
        }
        uqm_lua_checked!(VidSeek(newpos as DWORD), BOOLEAN(0));
    } else {
        if uqm_lua_checked!(VidProcessFrame(), BOOLEAN(0)).0 == 0 {
            return BOOLEAN::FALSE;
        }
        uqm_lua_checked!(SleepThread(
            (840 as ::core::ffi::c_int / 40 as ::core::ffi::c_int) as TimePeriod,
        ), BOOLEAN(0));
    }
    return BOOLEAN::TRUE;
}
unsafe extern "C" fn FadeClearScreen() {
    uqm_lua_checked!(SleepThreadUntil(
        FadeScreen(
            ScreenFadeType::FadeAllToBlack,
            (840 as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as SIZE,
        ),
    ), ());
    SetContext(ScreenContext);
    SetContextBackGroundColor(colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE));
    uqm_lua_checked!(ClearDrawable(), ());
    FadeScreen(ScreenFadeType::FadeAllToColor, 0 as SIZE);
}
unsafe extern "C" fn ShowLegacyVideo(mut vid: LEGACY_VIDEO) -> BOOLEAN {
    let mut vis: VIDEO_INPUT_STATE = VIDEO_INPUT_STATE {
        InputFunc: None,
        CurVideo: ::core::ptr::null_mut::<legacy_video_ref>(),
    };
    let mut r#ref: LEGACY_VIDEO_REF = ::core::ptr::null_mut::<legacy_video_ref>();
    uqm_lua_checked!(FadeClearScreen(), BOOLEAN(0));
    r#ref = uqm_lua_checked!(PlayLegacyVideo(vid), BOOLEAN(0));
    if r#ref.is_null() {
        return BOOLEAN::FALSE;
    }
    vis.InputFunc = Some(
        DoVideoInput as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> BOOLEAN,
    ) as Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> BOOLEAN>;
    vis.CurVideo = r#ref;
    SetMenuSounds(
        0 as ::core::ffi::c_int as MENU_SOUND_FLAGS,
        0 as ::core::ffi::c_int as MENU_SOUND_FLAGS,
    );
    uqm_lua_checked!(DoInput(&raw mut vis as *mut ::core::ffi::c_void, BOOLEAN::TRUE), BOOLEAN(0));
    uqm_lua_checked!(StopLegacyVideo(r#ref), BOOLEAN(0));
    uqm_lua_checked!(FadeClearScreen(), BOOLEAN(0));
    return BOOLEAN::TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn ShowPresentation(mut res: RESOURCE) -> BOOLEAN {
    let mut resType: *const ::core::ffi::c_char = res_GetResourceType(res);
    if resType.is_null() {
        return BOOLEAN::FALSE;
    }
    if strcmp(resType, b"STRTAB\0".as_ptr() as *const ::core::ffi::c_char) == 0 {
        let mut pres: STRING = CaptureStringTable(uqm_lua_checked!(LoadStringTableInstance(res), BOOLEAN(0)));
        let mut result: BOOLEAN = uqm_lua_checked!(ShowSlidePresentation(pres), BOOLEAN(0));
        DestroyStringTable(ReleaseStringTable(pres));
        return result;
    } else if strcmp(resType, b"3DOVID\0".as_ptr() as *const ::core::ffi::c_char) == 0 {
        let mut vid: LEGACY_VIDEO = uqm_lua_checked!(LoadLegacyVideoInstance(res), BOOLEAN(0));
        let mut result_0: BOOLEAN = uqm_lua_checked!(ShowLegacyVideo(vid), BOOLEAN(0));
        DestroyLegacyVideo(vid);
        return result_0;
    }
    log_add(
        log_Level::log_Warning,
        b"Tried to present '%s', of non-presentable type '%s'\0".as_ptr()
            as *const ::core::ffi::c_char,
        res,
        resType,
    );
    return BOOLEAN::FALSE;
}
#[export_name = "uqm_c2rust_init_b155651a38e88e34"]
unsafe extern "C" fn c2rust_run_static_initializers() {
    raceID = ::core::mem::size_of::<[SHIPMAP; 25]>()
        .wrapping_div(::core::mem::size_of::<SHIPMAP>()) as COUNT;
    shipID = ::core::mem::size_of::<[SHIPMAP; 25]>()
        .wrapping_div(::core::mem::size_of::<SHIPMAP>()) as COUNT;
}
// Explicit engine initialization replaces platform constructor tables.
