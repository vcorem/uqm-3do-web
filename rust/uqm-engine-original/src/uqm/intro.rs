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
    );
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
    fn DrawLine(pLine: *mut LINE);
    fn font_DrawText(pText: *mut TEXT);
    fn font_DrawTracedText(pText: *mut TEXT, text: Color, trace: Color);
    fn BatchGraphics();
    fn UnbatchGraphics();
    fn FlushGraphics();
    fn ClearDrawable();
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
    fn SetFrameTransparentColor(_: FRAME, _: Color);
    fn CaptureDrawable(Drawable: DRAWABLE) -> FRAME;
    fn ReleaseDrawable(Frame: FRAME) -> DRAWABLE;
    fn FadeScreen(fadeType: ScreenFadeType, TimeInterval: SIZE) -> DWORD;
    fn TFB_DrawScreen_Copy(r: *const RECT, src: SCREEN, dest: SCREEN);
    static mut PulsedInputState: CONTROLLER_INPUT_STATE;
    fn DoInput(pInputState: *mut ::core::ffi::c_void, resetInput: BOOLEAN);
    fn GetTimeCounter() -> TimeCount;
    fn SleepThread(timePeriod: TimePeriod);
    fn SleepThreadUntil(wakeTime: TimeCount);
    fn TaskSwitch();
    fn strupr(_: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn StopMusic();
    fn PlayMusic(MusicRef: MUSIC_REF, Continuous: BOOLEAN, Priority: BYTE);
    fn memset(
        _: *mut ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strcmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strchr(
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(_: *const ::core::ffi::c_char) -> size_t;
    static mut GlobData: GLOBDATA;
    static mut OffScreenContext: CONTEXT;
    static mut ScreenContext: CONTEXT;
    fn SetMenuSounds(sound_0: MENU_SOUND_FLAGS, sound_1: MENU_SOUND_FLAGS);
    static mut ScreenWidth: ::core::ffi::c_int;
    static mut ScreenHeight: ::core::ffi::c_int;
    fn SetGraphicScale(scale: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn SetGraphicScaleMode(mode: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn SetTransitionSource(pRect: *const RECT);
    fn ScreenTransition(transition: ::core::ffi::c_int, pRect: *const RECT);
    fn log_add(_: log_Level, fmt: *const ::core::ffi::c_char, ...);
    fn VidProcessFrame() -> BOOLEAN;
    fn VidGetPosition() -> DWORD;
    fn VidSeek(pos: DWORD) -> BOOLEAN;
    fn LoadLegacyVideoInstance(res: RESOURCE) -> LEGACY_VIDEO;
    fn DestroyLegacyVideo(vid: LEGACY_VIDEO) -> BOOLEAN;
    fn PlayLegacyVideo(vid: LEGACY_VIDEO) -> LEGACY_VIDEO_REF;
    fn StopLegacyVideo(r#ref: LEGACY_VIDEO_REF);
    fn PlayingLegacyVideo(r#ref: LEGACY_VIDEO_REF) -> BOOLEAN;
    fn toupper(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn LoadMusicFile(pStr: *const ::core::ffi::c_char) -> MUSIC_REF;
    fn DestroyMusic(MusicRef: MUSIC_REF) -> BOOLEAN;
    fn FadeMusic(end_vol: BYTE, TimeInterval: SIZE) -> DWORD;
    static mut contentDir: *mut uio_DirHandle;
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
    pub const KEY_MENU_ANY: Self = Self(23);
    pub const NUM_MENU_KEYS: Self = Self(24);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _controller_input_state {
    pub key: [[::core::ffi::c_int; 7]; 6],
    pub menu: [::core::ffi::c_int; 24],
}
pub type CONTROLLER_INPUT_STATE = _controller_input_state;
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
    pub GameState: [BYTE; 155],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GLOBDATA {
    pub SIS_state: SIS_STATE,
    pub Game_state: GAME_STATE,
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
        0 as BYTE,
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
        font_DrawTracedText(pText, Fore, Back);
    } else {
        SetContextForeGroundColor(Fore);
        font_DrawText(pText);
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
        origin: HOT_SPOT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    let mut r: RECT = RECT {
        corner: HOT_SPOT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut hs: HOT_SPOT = HOT_SPOT { x: 0, y: 0 };
    let mut slot: ::core::ffi::c_int = 0;
    let mut piece: COUNT = 0;
    let mut SisBack: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    OldContext = SetContext(OffScreenContext);
    SkelFrame = CaptureDrawable(
        LoadGraphicInstance(b"graphics.flagshipskeleton\0".as_ptr() as RESOURCE)
            as DRAWABLE,
    );
    ModuleFrame = CaptureDrawable(
        LoadGraphicInstance(b"graphics.outfitmodules\0".as_ptr() as RESOURCE) as DRAWABLE,
    );
    GetFrameRect(SkelFrame, &raw mut r);
    SisFrame = CaptureDrawable(
        CreateDrawable(
            ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as CREATE_FLAGS,
            r.extent.width,
            r.extent.height,
            1 as COUNT,
        ),
    );
    SetContextFGFrame(SisFrame);
    SetContextClipRect(::core::ptr::null_mut::<RECT>());
    SisBack = colorFromRgb15(0x1 as BYTE, 0x1 as BYTE, 0x1 as BYTE);
    SetContextBackGroundColor(SisBack);
    ClearDrawable();
    SetFrameTransparentColor(SisFrame, SisBack);
    s.frame = SetAbsFrameIndex(SkelFrame, 0 as COUNT);
    s.origin.x = 0 as COORD;
    s.origin.y = 0 as COORD;
    DrawStamp(&raw mut s);
    slot = 0 as ::core::ffi::c_int;
    while slot < 11 as ::core::ffi::c_int {
        piece = GlobData.SIS_state.DriveSlots[slot as usize] as COUNT;
        if (piece as ::core::ffi::c_int)
            < C2Rust_Unnamed_0::NUM_MODULES.0 as ::core::ffi::c_int
        {
            s.origin.x = 33 as COORD;
            s.origin.y = (65 as ::core::ffi::c_int + 21 as ::core::ffi::c_int
                + -79 as ::core::ffi::c_int) as COORD;
            s.origin.x = (s.origin.x as ::core::ffi::c_int
                + slot * 12 as ::core::ffi::c_int) as COORD;
            s.frame = SetAbsFrameIndex(ModuleFrame, piece);
            DrawStamp(&raw mut s);
        }
        slot += 1;
    }
    slot = 0 as ::core::ffi::c_int;
    while slot < 8 as ::core::ffi::c_int {
        piece = GlobData.SIS_state.JetSlots[slot as usize] as COUNT;
        if (piece as ::core::ffi::c_int)
            < C2Rust_Unnamed_0::NUM_MODULES.0 as ::core::ffi::c_int
        {
            s.origin.x = 70 as COORD;
            s.origin.y = (73 as ::core::ffi::c_int + 21 as ::core::ffi::c_int
                + -79 as ::core::ffi::c_int) as COORD;
            s.origin.x = (s.origin.x as ::core::ffi::c_int
                + slot * 12 as ::core::ffi::c_int) as COORD;
            s.frame = SetAbsFrameIndex(ModuleFrame, piece);
            DrawStamp(&raw mut s);
        }
        slot += 1;
    }
    slot = 0 as ::core::ffi::c_int;
    while slot < 16 as ::core::ffi::c_int {
        piece = GlobData.SIS_state.ModuleSlots[slot as usize] as COUNT;
        if (piece as ::core::ffi::c_int)
            < C2Rust_Unnamed_0::NUM_MODULES.0 as ::core::ffi::c_int
        {
            s.origin.x = 17 as COORD;
            s.origin.y = (96 as ::core::ffi::c_int + 21 as ::core::ffi::c_int
                + -79 as ::core::ffi::c_int) as COORD;
            s.origin.x = (s.origin.x as ::core::ffi::c_int
                + slot * 12 as ::core::ffi::c_int) as COORD;
            s.frame = SetAbsFrameIndex(ModuleFrame, piece);
            DrawStamp(&raw mut s);
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
unsafe extern "C" fn Present_DrawMovieFrame(mut pPIS: *mut PRESENTATION_INPUT_STATE) {
    let mut s: STAMP = STAMP {
        origin: HOT_SPOT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    s.origin.x = 0 as COORD;
    s.origin.y = 0 as COORD;
    s.frame = SetAbsFrameIndex((*pPIS).Frame, (*pPIS).MovieFrame as COUNT);
    DrawStamp(&raw mut s);
}
unsafe extern "C" fn ShowPresentationFile(
    mut name: *const ::core::ffi::c_char,
) -> BOOLEAN {
    let mut pres: STRING = CaptureStringTable(LoadStringTableFile(contentDir, name));
    let mut result: BOOLEAN = ShowSlidePresentation(pres);
    DestroyStringTable(ReleaseStringTable(pres));
    return result;
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
                Present_DrawMovieFrame(pPIS);
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
        SleepThread(Delay);
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
                (*pPIS).clip_r.extent.width = w as COORD;
                (*pPIS).clip_r.extent.height = h as COORD;
                (*pPIS).clip_r.corner.x = ((ScreenWidth - w) / 2 as ::core::ffi::c_int)
                    as COORD;
                (*pPIS).clip_r.corner.y = ((ScreenHeight - h) / 2 as ::core::ffi::c_int)
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
                        367 as ::core::ffi::c_int,
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
                    *pFont = LoadFontFile(
                        &raw mut (*pPIS).Buffer as *mut ::core::ffi::c_char,
                    );
                }
                SetContextFont(*pFont);
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
                LoadGraphicFile(&raw mut (*pPIS).Buffer as *mut ::core::ffi::c_char),
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
                StopMusic();
                DestroyMusic((*pPIS).MusicRef);
            }
            (*pPIS).MusicRef = LoadMusicFile(
                &raw mut (*pPIS).Buffer as *mut ::core::ffi::c_char,
            );
            PlayMusic((*pPIS).MusicRef, BOOLEAN::FALSE, 1 as BYTE);
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
            b"SYNC\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            let mut msecs_0: ::core::ffi::c_int = 0;
            Present_UnbatchGraphics(pPIS, BOOLEAN::TRUE);
            if 1 as ::core::ffi::c_int
                == sscanf(
                    pStr,
                    b"%d\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut msecs_0,
                )
            {
                (*pPIS).LastSyncTime = (*pPIS)
                    .StartTime
                    .wrapping_add(
                        (msecs_0 * 840 as ::core::ffi::c_int
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
            let mut msecs_1: ::core::ffi::c_int = 0;
            Present_UnbatchGraphics(pPIS, BOOLEAN::TRUE);
            if 1 as ::core::ffi::c_int
                == sscanf(
                    pStr,
                    b"%d\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut msecs_1,
                )
            {
                (*pPIS).TimeOut = (*pPIS)
                    .LastSyncTime
                    .wrapping_add(
                        (msecs_1 * 840 as ::core::ffi::c_int
                            / 1000 as ::core::ffi::c_int) as TimeCount,
                    );
                (*pPIS).TimeOutOnSkip = BOOLEAN::FALSE.0 as ::core::ffi::c_int;
                return BOOLEAN::TRUE;
            }
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
                        470 as ::core::ffi::c_int,
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
                    baseline: HOT_SPOT { x: 0, y: 0 },
                    pStr: ::core::ptr::null::<UNICODE>(),
                    align: TEXT_ALIGN::ALIGN_LEFT,
                    CharCount: 0,
                };
                t.align = TEXT_ALIGN::ALIGN_CENTER;
                t.pStr = &raw mut (*pPIS).Buffer as *mut ::core::ffi::c_char;
                t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
                t.baseline.x = x as COORD;
                t.baseline.y = y as COORD;
                DrawTextEffect(
                    &raw mut t,
                    (*pPIS).TextColor,
                    (*pPIS).TextBackColor,
                    (*pPIS).TextEffect,
                );
            }
        } else if strcmp(
            &raw mut Opcode as *mut ::core::ffi::c_char,
            b"TFI\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            let mut leading: SIZE = 0;
            let mut i: COUNT = 0;
            let mut y_0: COORD = 0;
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
            GetContextFontLeading(&raw mut leading);
            match (*pPIS).TextVPos {
                84 => {
                    y_0 = leading as COORD;
                }
                77 => {
                    y_0 = (((*pPIS).clip_r.extent.height as ::core::ffi::c_int
                        - (*pPIS).LinesCount as ::core::ffi::c_int
                            * leading as ::core::ffi::c_int) / 2 as ::core::ffi::c_int)
                        as COORD;
                }
                _ => {
                    y_0 = ((*pPIS).clip_r.extent.height as ::core::ffi::c_int
                        - (*pPIS).LinesCount as ::core::ffi::c_int
                            * leading as ::core::ffi::c_int) as COORD;
                }
            }
            (*pPIS).tfade_r = (*pPIS).clip_r;
            (*pPIS).tfade_r.corner.y = ((*pPIS).tfade_r.corner.y as ::core::ffi::c_int
                + (y_0 as ::core::ffi::c_int - leading as ::core::ffi::c_int)) as COORD;
            (*pPIS).tfade_r.extent.height = (((*pPIS).LinesCount as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int) * leading as ::core::ffi::c_int) as COORD;
            i = 0 as COUNT;
            while (i as ::core::ffi::c_int) < (*pPIS).LinesCount as ::core::ffi::c_int {
                (*pPIS).TextLines[i as usize].align = TEXT_ALIGN::ALIGN_CENTER;
                (*pPIS).TextLines[i as usize].baseline.x = (ScreenWidth
                    / 2 as ::core::ffi::c_int) as COORD;
                (*pPIS).TextLines[i as usize].baseline.y = y_0;
                i = i.wrapping_add(1);
                y_0 = (y_0 as ::core::ffi::c_int + leading as ::core::ffi::c_int)
                    as COORD;
            }
            i = 0 as COUNT;
            while (i as ::core::ffi::c_int) < (*pPIS).LinesCount as ::core::ffi::c_int {
                DrawTextEffect(
                    (&raw mut (*pPIS).TextLines as *mut TEXT)
                        .offset(i as ::core::ffi::c_int as isize),
                    (*pPIS).TextFadeColor,
                    (*pPIS).TextFadeColor,
                    (*pPIS).TextEffect,
                );
                i = i.wrapping_add(1);
            }
            SetTransitionSource(&raw mut (*pPIS).tfade_r);
            BatchGraphics();
            i = 0 as COUNT;
            while (i as ::core::ffi::c_int) < (*pPIS).LinesCount as ::core::ffi::c_int {
                DrawTextEffect(
                    (&raw mut (*pPIS).TextLines as *mut TEXT)
                        .offset(i as ::core::ffi::c_int as isize),
                    (*pPIS).TextColor,
                    (*pPIS).TextBackColor,
                    (*pPIS).TextEffect,
                );
                i = i.wrapping_add(1);
            }
            ScreenTransition(3 as ::core::ffi::c_int, &raw mut (*pPIS).tfade_r);
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
                DrawTextEffect(
                    (&raw mut (*pPIS).TextLines as *mut TEXT)
                        .offset(i_0 as ::core::ffi::c_int as isize),
                    (*pPIS).TextFadeColor,
                    (*pPIS).TextFadeColor,
                    (*pPIS).TextEffect,
                );
                i_0 = i_0.wrapping_add(1);
            }
            ScreenTransition(3 as ::core::ffi::c_int, &raw mut (*pPIS).tfade_r);
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
            let mut index_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut x_0: ::core::ffi::c_int = 0;
            let mut y_1: ::core::ffi::c_int = 0;
            let mut scale: ::core::ffi::c_int = 0;
            let mut angle: ::core::ffi::c_int = 0;
            let mut scale_mode: ::core::ffi::c_int = 0;
            let mut ImgName: [::core::ffi::c_char; 16] = [0; 16];
            let mut old_scale: ::core::ffi::c_int = 0;
            let mut old_mode: ::core::ffi::c_int = 0;
            let mut s: STAMP = STAMP {
                origin: HOT_SPOT { x: 0, y: 0 },
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
                    &raw mut x_0,
                    &raw mut y_1,
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
                    &raw mut index_0,
                    &raw mut x_0,
                    &raw mut y_1,
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
            } else {
                if cargs < 5 as ::core::ffi::c_int {
                    angle = 0 as ::core::ffi::c_int;
                }
                if cargs < 4 as ::core::ffi::c_int {
                    scale = 256 as ::core::ffi::c_int;
                }
                if cargs < 3 as ::core::ffi::c_int {
                    x_0 = 0 as ::core::ffi::c_int;
                    y_1 = 0 as ::core::ffi::c_int;
                }
                s.frame = ::core::ptr::null_mut::<FRAME_DESC>();
                if draw_what == 0 as ::core::ffi::c_int {
                    s.frame = SetAbsFrameIndex((*pPIS).Frame, index_0 as COUNT);
                } else if draw_what == 1 as ::core::ffi::c_int {
                    if (*pPIS).SisFrame.is_null() {
                        Present_GenerateSIS(pPIS);
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
                s.origin.x = x_0 as COORD;
                s.origin.y = y_1 as COORD;
                old_mode = SetGraphicScaleMode(scale_mode);
                old_scale = SetGraphicScale(scale);
                DrawStamp(&raw mut s);
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
            ClearDrawable();
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
            ShowPresentationFile(&raw mut (*pPIS).Buffer as *mut ::core::ffi::c_char);
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
                    first: HOT_SPOT { x: 0, y: 0 },
                    second: HOT_SPOT { x: 0, y: 0 },
                };
                l.first.x = x1 as COORD;
                l.first.y = y1 as COORD;
                l.second.x = x2 as COORD;
                l.second.y = y2 as COORD;
                SetContextForeGroundColor((*pPIS).TextColor);
                DrawLine(&raw mut l);
            } else {
                log_add(
                    log_Level::log_Warning,
                    b"Bad LINE command '%s'\0".as_ptr() as *const ::core::ffi::c_char,
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
        corner: HOT_SPOT { x: 0, y: 0 },
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
            corner: HOT_SPOT { x: 0, y: 0 },
            extent: EXTENT { width: 0, height: 0 },
        },
        tfade_r: RECT {
            corner: HOT_SPOT { x: 0, y: 0 },
            extent: EXTENT { width: 0, height: 0 },
        },
        TextLines: [TEXT {
            baseline: HOT_SPOT { x: 0, y: 0 },
            pStr: ::core::ptr::null::<UNICODE>(),
            align: TEXT_ALIGN::ALIGN_LEFT,
            CharCount: 0,
        }; 15],
        LinesCount: 0,
        Buffer: [0; 512],
        MovieFrame: 0,
        MovieEndFrame: 0,
        InterframeDelay: 0,
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
    DoInput(&raw mut pis as *mut ::core::ffi::c_void, BOOLEAN::TRUE);
    SleepThreadUntil(FadeMusic(0 as BYTE, 840 as SIZE));
    StopMusic();
    FadeMusic(160 as BYTE, 0 as SIZE);
    DestroyMusic(pis.MusicRef);
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
        VidSeek(newpos as DWORD);
    } else {
        if VidProcessFrame().0 == 0 {
            return BOOLEAN::FALSE;
        }
        SleepThread(
            (840 as ::core::ffi::c_int / 40 as ::core::ffi::c_int) as TimePeriod,
        );
    }
    return BOOLEAN::TRUE;
}
unsafe extern "C" fn FadeClearScreen() {
    SleepThreadUntil(
        FadeScreen(
            ScreenFadeType::FadeAllToBlack,
            (840 as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as SIZE,
        ),
    );
    SetContext(ScreenContext);
    SetContextBackGroundColor(colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE));
    ClearDrawable();
    FadeScreen(ScreenFadeType::FadeAllToColor, 0 as SIZE);
}
unsafe extern "C" fn ShowLegacyVideo(mut vid: LEGACY_VIDEO) -> BOOLEAN {
    let mut vis: VIDEO_INPUT_STATE = VIDEO_INPUT_STATE {
        InputFunc: None,
        CurVideo: ::core::ptr::null_mut::<legacy_video_ref>(),
    };
    let mut r#ref: LEGACY_VIDEO_REF = ::core::ptr::null_mut::<legacy_video_ref>();
    FadeClearScreen();
    r#ref = PlayLegacyVideo(vid);
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
    DoInput(&raw mut vis as *mut ::core::ffi::c_void, BOOLEAN::TRUE);
    StopLegacyVideo(r#ref);
    FadeClearScreen();
    return BOOLEAN::TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn ShowPresentation(mut res: RESOURCE) -> BOOLEAN {
    let mut resType: *const ::core::ffi::c_char = res_GetResourceType(res);
    if resType.is_null() {
        return BOOLEAN::FALSE;
    }
    if strcmp(resType, b"STRTAB\0".as_ptr() as *const ::core::ffi::c_char) == 0 {
        let mut pres: STRING = CaptureStringTable(LoadStringTableInstance(res));
        let mut result: BOOLEAN = ShowSlidePresentation(pres);
        DestroyStringTable(ReleaseStringTable(pres));
        return result;
    } else if strcmp(resType, b"3DOVID\0".as_ptr() as *const ::core::ffi::c_char) == 0 {
        let mut vid: LEGACY_VIDEO = LoadLegacyVideoInstance(res);
        let mut result_0: BOOLEAN = ShowLegacyVideo(vid);
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
