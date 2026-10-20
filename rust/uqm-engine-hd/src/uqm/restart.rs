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

// Rust translation of src/uqm/restart.c.
extern "C" {
    pub type tfb_soundsample;
    pub type FlashContext;
    pub type string_table_entry;
    fn snprintf(
        _: *mut ::core::ffi::c_char,
        _: size_t,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn SetAbsStringTableIndex(String: STRING, StringTableIndex: COUNT) -> STRING;
    fn GetStringAddress(String: STRING) -> STRINGPTR;
    fn utf8StringCount(start: *const ::core::ffi::c_char) -> size_t;
    fn SetContext(Context: CONTEXT) -> CONTEXT;
    fn SetContextForeGroundColor(Color: Color) -> Color;
    fn SetContextBackGroundColor(Color: Color) -> Color;
    fn SetContextFGFrame(Frame: FRAME) -> FRAME;
    fn SetContextClipRect(pRect: *mut RECT) -> BOOLEAN;
    fn GetContextClipRect(pRect: *mut RECT) -> BOOLEAN;
    fn DrawStamp(pStamp: *mut STAMP);
    fn font_DrawText(pText: *mut TEXT);
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
    fn LoadGraphicInstance(res: RESOURCE) -> *mut ::core::ffi::c_void;
    fn SetContextFont(Font: FONT) -> FONT;
    fn GetContextFontLeading(pheight: *mut SIZE) -> BOOLEAN;
    fn SetRelFrameIndex(Frame: FRAME, FrameOffs: SIZE) -> FRAME;
    fn IncFrameIndex(Frame: FRAME) -> FRAME;
    fn SetFrameTransparentColor(_: FRAME, _: Color);
    fn CaptureDrawable(Drawable: DRAWABLE) -> FRAME;
    fn ReleaseDrawable(Frame: FRAME) -> DRAWABLE;
    fn FadeScreen(fadeType: ScreenFadeType, TimeInterval: SIZE) -> DWORD;
    fn FlushColorXForms();
    fn memset(
        _: *mut ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn GetTimeCounter() -> TimeCount;
    fn SleepThread(timePeriod: TimePeriod);
    fn SleepThreadUntil(wakeTime: TimeCount);
    static mut CurrentInputState: CONTROLLER_INPUT_STATE;
    static mut PulsedInputState: CONTROLLER_INPUT_STATE;
    fn UpdateInputState();
    fn FlushInput();
    fn SetDefaultMenuRepeatDelay();
    fn DoPopupWindow(msg: *const ::core::ffi::c_char);
    fn DoInput(pInputState: *mut ::core::ffi::c_void, resetInput: BOOLEAN);
    static mut GamePaused: BOOLEAN;
    fn Credits(WithOuttakes: BOOLEAN);
    fn ReinitQueue(pq: *mut QUEUE);
    fn DestroyMusic(MusicRef: MUSIC_REF) -> BOOLEAN;
    fn FadeMusic(end_vol: BYTE, TimeInterval: SIZE) -> DWORD;
    fn SetMusicPosition();
    fn OkayToResume() -> BOOLEAN;
    fn GetMusicPosition() -> DWORD;
    fn ResetMusicResume();
    static mut Elements: *const BYTE;
    fn SetupMenu();
    static mut resolutionFactor: ::core::ffi::c_uint;
    static mut optRequiresReload: BOOLEAN;
    static mut optRequiresRestart: BOOLEAN;
    static mut optSkipIntro: OPT_ENABLABLE;
    static mut optMainMenuMusic: OPT_ENABLABLE;
    static mut optSuperMelee: BOOLEAN;
    static mut optLoadGame: BOOLEAN;
    static mut optCustomSeed: ::core::ffi::c_int;
    static mut optShipSeed: OPT_ENABLABLE;
    static mut optDifficulty: ::core::ffi::c_int;
    static mut optDiffChooser: ::core::ffi::c_int;
    static mut optGameOver: OPT_ENABLABLE;
    static mut optMeleeToolTips: OPT_ENABLABLE;
    static mut optWindowType: DWORD;
    static mut restartGame: BOOLEAN;
    static mut CanvasWidth: ::core::ffi::c_int;
    static mut CanvasHeight: ::core::ffi::c_int;
    fn SetFlashRect(pRect: *const RECT, purple: BOOLEAN);
    fn setGameStateUint(name: *const ::core::ffi::c_char, val: DWORD);
    fn getGameStateUint(name: *const ::core::ffi::c_char) -> DWORD;
    static mut GlobData: GLOBDATA;
    static mut PlanData: *const PlanetFrame;
    static mut star_array: [STAR_DESC; 0];
    static mut constel_array: *mut POINT;
    fn Introduction();
    fn Victory();
    fn GameOver(DeathType: BYTE);
    fn Reload();
    fn Flash_createOverlay(
        gfxContext: CONTEXT,
        origin: *const POINT,
        overlay: FRAME,
    ) -> *mut FlashContext;
    fn Flash_setState(
        context: *mut FlashContext,
        state: FlashState,
        timeSpentInState: TimeCount,
    );
    fn Flash_start(context: *mut FlashContext);
    fn Flash_terminate(context: *mut FlashContext);
    fn Flash_pause(context: *mut FlashContext);
    fn Flash_continue(context: *mut FlashContext);
    fn Flash_process(context: *mut FlashContext);
    fn Flash_setSpeed(
        context: *mut FlashContext,
        fadeInTime: TimeCount,
        onTime: TimeCount,
        fadeOutTime: TimeCount,
        offTime: TimeCount,
    );
    fn Flash_setMergeFactors(
        context: *mut FlashContext,
        startNumer: ::core::ffi::c_int,
        endNumer: ::core::ffi::c_int,
        denom: ::core::ffi::c_int,
    );
    fn Flash_setFrameTime(context: *mut FlashContext, frameTime: TimeCount);
    fn Flash_setPulseBox(context: *mut FlashContext, isPulsing: BOOLEAN);
    fn Flash_setOverlay(
        context: *mut FlashContext,
        origin: *const POINT,
        overlay: FRAME,
        cleanup: BOOLEAN,
    );
    fn Flash_UpdateOriginal(context: *mut FlashContext);
    static mut GameStrings: STRING;
    static mut netplayOptions: NetplayOptions;
    static mut webNetplayRole: ::core::ffi::c_int;
    static mut ScreenContext: CONTEXT;
    static mut MicroFont: FONT;
    static mut TinyFont: FONT;
    static mut SlabFont: FONT;
    static mut race_q: [QUEUE; 0];
    static mut Rando: BYTE;
    static mut DeathBySurrender: BOOLEAN;
    static mut DeathByMelee: BOOLEAN;
    static mut DeathBySuicide: BOOLEAN;
    static mut LastActivity: ACTIVITY;
    static mut PlayerControl: [BYTE; 0];
    fn StopMusic();
    fn SeekMusic(pos: DWORD);
    fn PlayMusic(MusicRef: MUSIC_REF, Continuous: BOOLEAN, Priority: BYTE);
    fn loadMainMenuMusic(selector: BYTE) -> MUSIC_REF;
    static mut NextActivity: ACTIVITY;
    fn SetMenuSounds(sound_0: MENU_SOUND_FLAGS, sound_1: MENU_SOUND_FLAGS);
    fn PlayMenuSound(S: MENU_SOUND_EFFECT);
    fn SeedRandomNumbers() -> DWORD;
    fn SaveContextFrame(saveRect: *const RECT) -> STAMP;
    fn Melee();
    fn SetTransitionSource(pRect: *const RECT);
    fn ScreenTransition(transition: ::core::ffi::c_int, pRect: *const RECT);
    fn FreeGameData();
    fn ReloadMasterShipList(YieldProcessing: Option<unsafe extern "C" fn() -> ()>);
    fn StringBank_Create() -> *mut stringbank;
    fn StringBank_Free(bank: *mut stringbank);
    fn SplitString(
        s: *const ::core::ffi::c_char,
        splitchar: ::core::ffi::c_char,
        n: ::core::ffi::c_int,
        result: *mut *const ::core::ffi::c_char,
        bank: *mut stringbank,
    ) -> ::core::ffi::c_int;
    fn emscripten_asm_const_int_sync_on_main_thread(
        code: *const ::core::ffi::c_char,
        arg_sigs: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
}
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
pub type POINT = point;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct point {
    pub x: COORD,
    pub y: COORD,
}
pub type COORD = SWORD;
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
pub type RESOURCE = *const ::core::ffi::c_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ELEMENT_ENTRY {
    pub ElementType: BYTE,
    pub Density: BYTE,
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
pub type EXTENT = extent;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct extent {
    pub width: COORD,
    pub height: COORD,
}
pub type STAMP = stamp;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stamp {
    pub origin: POINT,
    pub frame: FRAME,
}
pub type FRAME = *mut FRAME_DESC;
pub type FRAME_DESC = frame_desc;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Color {
    pub r: BYTE,
    pub g: BYTE,
    pub b: BYTE,
    pub a: BYTE,
}
pub type HOT_SPOT = POINT;
pub type TFB_Canvas = *mut ::core::ffi::c_void;
pub type DRAWABLE_TYPE = UWORD;
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
pub struct PRIMITIVE {
    pub Links: PRIM_LINKS,
    pub Type: GRAPHICS_PRIM,
    pub color: Color,
    pub Object: PRIM_DESC,
    pub flags: BYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union PRIM_DESC {
    pub Point: POINT,
    pub Stamp: STAMP,
    pub Line: LINE,
    pub Text: TEXT,
    pub Rect: RECT,
}
pub type RECT = rect;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rect {
    pub corner: POINT,
    pub extent: EXTENT,
}
pub type TEXT = text;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct text {
    pub baseline: POINT,
    pub pStr: *const UNICODE,
    pub align: TEXT_ALIGN,
    pub CharCount: COUNT,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct TEXT_ALIGN(pub ::core::ffi::c_uint);
impl TEXT_ALIGN {
    pub const ALIGN_LEFT: Self = Self(0);
    pub const ALIGN_CENTER: Self = Self(1);
    pub const ALIGN_RIGHT: Self = Self(2);
}
pub type LINE = line;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct line {
    pub first: POINT,
    pub second: POINT,
}
pub type GRAPHICS_PRIM = BYTE;
pub type PRIM_LINKS = DWORD;
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct OPT_ENABLABLE(pub ::core::ffi::c_uint);
impl OPT_ENABLABLE {
    pub const OPTVAL_DISABLED: Self = Self(0);
    pub const OPTVAL_ENABLED: Self = Self(1);
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
pub type STRING = *mut STRING_TABLE_ENTRY_DESC;
pub type STRING_TABLE_ENTRY_DESC = string_table_entry;
pub type DRAWABLE = *mut DRAWABLE_DESC;
pub type DRAWABLE_DESC = drawable_desc;
pub type MENU_SOUND_FLAGS = UWORD;
pub type STRINGPTR = *mut ::core::ffi::c_char;
pub type size_t = usize;
pub type FONT = *mut FONT_DESC;
pub type FONT_DESC = font_desc;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct font_desc {
    pub Leading: BYTE,
    pub fontPages: *mut FONT_PAGE,
    pub disp: EXTENT,
    pub filename: [::core::ffi::c_char; 4096],
    pub CharSpace: BYTE,
    pub KernAmount: BYTE,
    pub KernTab: [BYTE; 65535],
    pub VertAlign: SBYTE,
    pub HaveFntData: BOOLEAN,
}
pub type FONT_PAGE = FontPage;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FontPage {
    pub next: *mut FontPage,
    pub pageStart: UniChar,
    pub firstChar: UniChar,
    pub numChars: size_t,
    pub charDesc: *mut TFB_Char,
}
pub type TFB_Char = tfb_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tfb_char {
    pub extent: EXTENT,
    pub disp: EXTENT,
    pub HotSpot: HOT_SPOT,
    pub data: *mut BYTE,
    pub pitch: DWORD,
}
pub type UniChar = uint32;
pub type TimePeriod = DWORD;
pub type CONTEXT = *mut CONTEXT_DESC;
pub type CONTEXT_DESC = context_desc;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct context_desc {
    pub Flags: UWORD,
    pub ForeGroundColor: Color,
    pub BackGroundColor: Color,
    pub Mode: DrawMode,
    pub ForeGroundFrame: FRAME,
    pub Font: FONT,
    pub ClipRect: RECT,
    pub FontEffect: FRAME,
    pub FontBacking: *mut TFB_Image,
    pub BackingFlags: FBK_FLAGS,
    pub name: *const ::core::ffi::c_char,
    pub next: CONTEXT,
}
pub type FBK_FLAGS = UWORD;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DrawMode {
    pub kind: BYTE,
    pub factor: SWORD,
}
pub type CONTROLLER_INPUT_STATE = _controller_input_state;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _controller_input_state {
    pub key: [[::core::ffi::c_int; 8]; 2],
    pub menu: [::core::ffi::c_int; 31],
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct FlashState(pub ::core::ffi::c_uint);
impl FlashState {
    pub const FlashState_fadeIn: Self = Self(0);
    pub const FlashState_on: Self = Self(1);
    pub const FlashState_fadeOut: Self = Self(2);
    pub const FlashState_off: Self = Self(3);
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
pub type stringbank = _stringbank_chunk;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _stringbank_chunk {
    pub data: [::core::ffi::c_char; 1016],
    pub len: ::core::ffi::c_int,
    pub next: *mut _stringbank_chunk,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NetplayOptions {
    pub metaServer: *const ::core::ffi::c_char,
    pub metaPort: *const ::core::ffi::c_char,
    pub peer: [NetplayPeerOptions; 2],
    pub inputDelay: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NetplayPeerOptions {
    pub isServer: bool,
    pub host: *const ::core::ffi::c_char,
    pub port: *const ::core::ffi::c_char,
}
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
    pub const OPTVAL_NORMAL: Self = Self(0);
    pub const OPTVAL_SLOW: Self = Self(1);
    pub const OPTVAL_FAST: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_1(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_1 {
    pub const OPTVAL_NORM: Self = Self(0);
    pub const OPTVAL_EASY: Self = Self(1);
    pub const OPTVAL_HARD: Self = Self(2);
    pub const OPTVAL_IMPO: Self = Self(3);
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
    pub const DEATH_MARCH: Self = Self(0);
    pub const DIED_IN_BATTLE: Self = Self(1);
    pub const SUICIDE: Self = Self(2);
    pub const SURRENDERED: Self = Self(3);
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
    pub const START_NEW_GAME: Self = Self(0);
    pub const LOAD_SAVED_GAME: Self = Self(1);
    pub const PLAY_SUPER_MELEE: Self = Self(2);
    pub const SETUP_GAME: Self = Self(3);
    pub const QUIT_GAME: Self = Self(4);
    pub const NUM_MENU_ELEMENTS: Self = Self(5);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_6(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_6 {
    pub const EASY_DIFF: Self = Self(0);
    pub const ORIGINAL_DIFF: Self = Self(1);
    pub const HARD_DIFF: Self = Self(2);
}
unsafe extern "C" fn setBrowserGameScreen(mut screen: *const ::core::ffi::c_char) {
    let mut c2rust_lvalue: [::core::ffi::c_char; 2] = [
        'p' as ::core::ffi::c_char,
        '\0' as ::core::ffi::c_char,
    ];
    emscripten_asm_const_int_sync_on_main_thread(
        ({
            extern "C" { static uqm_em_asm_1: [::core::ffi::c_char; 42]; }
            &raw const uqm_em_asm_1 as *const ::core::ffi::c_char
        }),
        &raw mut c2rust_lvalue as *mut ::core::ffi::c_char,
        screen,
    );
}
unsafe extern "C" fn announceWebNetplayHook() {
    let mut c2rust_lvalue: [::core::ffi::c_char; 1] = ['\0' as ::core::ffi::c_char];
    emscripten_asm_const_int_sync_on_main_thread(
        ({
            extern "C" { static uqm_em_asm_2: [::core::ffi::c_char; 39]; }
            &raw const uqm_em_asm_2 as *const ::core::ffi::c_char
        }),
        &raw mut c2rust_lvalue as *mut ::core::ffi::c_char,
    );
}
unsafe extern "C" fn pollWebNetplayPage() -> ::core::ffi::c_int {
    let mut c2rust_lvalue: [::core::ffi::c_char; 1] = ['\0' as ::core::ffi::c_char];
    return emscripten_asm_const_int_sync_on_main_thread(
        ({
            extern "C" { static uqm_em_asm_3: [::core::ffi::c_char; 346]; }
            &raw const uqm_em_asm_3 as *const ::core::ffi::c_char
        }),
        &raw mut c2rust_lvalue as *mut ::core::ffi::c_char,
    );
}
unsafe extern "C" fn takeWebNetplayRole(mut page: ::core::ffi::c_int) -> BOOLEAN {
    let mut role: ::core::ffi::c_int = (page & 3 as ::core::ffi::c_int)
        - 1 as ::core::ffi::c_int;
    if role != 0 as ::core::ffi::c_int && role != 1 as ::core::ffi::c_int {
        return BOOLEAN::FALSE;
    }
    webNetplayRole = role;
    netplayOptions.inputDelay = 3 as size_t;
    return BOOLEAN::TRUE;
}
unsafe extern "C" fn playWebMelee() {
    optShipSeed = OPT_ENABLABLE::OPTVAL_DISABLED;
    uqm_lua_checked!(ReloadMasterShipList(None), ());
    uqm_lua_checked!(FreeGameData(), ());
    setBrowserGameScreen(b"melee\0".as_ptr() as *const ::core::ffi::c_char);
    uqm_lua_checked!(Melee(), ());
    webNetplayRole = -1 as ::core::ffi::c_int;
    GlobData.Game_state.CurrentActivity = 0 as ACTIVITY;
}
#[no_mangle]
pub static mut menuMusic: MUSIC_REF = ::core::ptr::null_mut::<*mut TFB_SoundSample>();
#[no_mangle]
pub unsafe extern "C" fn InitMenuMusic() {
    if optMainMenuMusic.0 != 0 && menuMusic.is_null() {
        uqm_lua_checked!(FadeMusic(0 as BYTE, 0 as SIZE), ());
        menuMusic = uqm_lua_checked!(loadMainMenuMusic(Rando), ());
        uqm_lua_checked!(PlayMusic(menuMusic, BOOLEAN::TRUE, 1 as BYTE), ());
        if OkayToResume().0 != 0 {
            uqm_lua_checked!(SeekMusic(GetMusicPosition()), ());
        }
        uqm_lua_checked!(FadeMusic(
            (160 as ::core::ffi::c_int + 70 as ::core::ffi::c_int) as BYTE,
            (840 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as SIZE,
        ), ());
    }
}
#[no_mangle]
pub unsafe extern "C" fn UninitMenuMusic() {
    if !menuMusic.is_null() {
        uqm_lua_checked!(SleepThreadUntil(uqm_lua_checked!(FadeMusic(0 as BYTE, 840 as SIZE), ())), ());
        uqm_lua_checked!(SetMusicPosition(), ());
        uqm_lua_checked!(StopMusic(), ());
        uqm_lua_checked!(DestroyMusic(menuMusic), ());
        menuMusic = ::core::ptr::null_mut::<*mut TFB_SoundSample>();
        uqm_lua_checked!(FadeMusic(160 as BYTE, 0 as SIZE), ());
    }
}
#[no_mangle]
pub unsafe extern "C" fn DrawToolTips(
    mut pMS: *mut MENU_STATE,
    mut answer: ::core::ffi::c_int,
) {
    let mut i: COUNT = 0;
    let mut t: TEXT = TEXT {
        baseline: POINT { x: 0, y: 0 },
        pStr: ::core::ptr::null::<UNICODE>(),
        align: TEXT_ALIGN::ALIGN_LEFT,
        CharCount: 0,
    };
    let mut bank: *mut stringbank = StringBank_Create();
    let mut lines: [*const ::core::ffi::c_char; 30] = [::core::ptr::null::<
        ::core::ffi::c_char,
    >(); 30];
    let mut line_count: ::core::ffi::c_int = 0;
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut s: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    SetContextFont(TinyFont);
    GetFrameRect(SetRelFrameIndex((*pMS).CurFrame, 2 as SIZE), &raw mut r);
    r.corner.y = (r.corner.y as ::core::ffi::c_int
        + ((CanvasHeight >> 1 as ::core::ffi::c_int)
            - ((12 as ::core::ffi::c_int) << resolutionFactor)
            + r.extent.height as ::core::ffi::c_int
            + ((1 as ::core::ffi::c_int) << resolutionFactor))) as COORD;
    s.frame = SetRelFrameIndex((*pMS).CurFrame, 3 as SIZE);
    r.extent = (*s.frame).Bounds;
    r.corner.x = (((CanvasWidth >> resolutionFactor)
        - (r.extent.width as ::core::ffi::c_int >> resolutionFactor)
        >> 1 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    s.origin = r.corner;
    uqm_lua_checked!(DrawStamp(&raw mut s), ());
    SetContextForeGroundColor(colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE));
    t.pStr = GetStringAddress(
        SetAbsStringTableIndex(
            GameStrings,
            (C2Rust_Unnamed_4::MAINMENU_STRING_BASE.0 as ::core::ffi::c_int
                + 66 as ::core::ffi::c_int + answer) as COUNT,
        ),
    );
    line_count = SplitString(
        t.pStr,
        '\n' as ::core::ffi::c_char,
        30 as ::core::ffi::c_int,
        &raw mut lines as *mut *const ::core::ffi::c_char,
        bank,
    );
    t.baseline.x = (r.corner.x as ::core::ffi::c_int
        + ((r.extent.width as ::core::ffi::c_int >> resolutionFactor
            >> 1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    t.baseline.y = (r.corner.y as ::core::ffi::c_int
        + ((10 as ::core::ffi::c_int) << resolutionFactor)
        + (if line_count < 2 as ::core::ffi::c_int {
            8 as ::core::ffi::c_int
        } else if line_count > 2 as ::core::ffi::c_int {
            0 as ::core::ffi::c_int
        } else {
            3 as ::core::ffi::c_int
        } << resolutionFactor)) as COORD;
    i = 0 as COUNT;
    while (i as ::core::ffi::c_int) < line_count {
        t.pStr = lines[i as usize] as *const UNICODE;
        t.align = TEXT_ALIGN::ALIGN_CENTER;
        t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
        uqm_lua_checked!(font_DrawText(&raw mut t), ());
        t.baseline.y = (t.baseline.y as ::core::ffi::c_int
            + ((8 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        i = i.wrapping_add(1);
    }
    StringBank_Free(bank);
}
unsafe extern "C" fn DrawDiffChooser(
    mut pMS: *mut MENU_STATE,
    mut answer: BYTE,
    mut confirm: BOOLEAN,
) {
    let mut s: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    let mut oldFont: FONT = ::core::ptr::null_mut::<FONT_DESC>();
    let mut t: TEXT = TEXT {
        baseline: POINT { x: 0, y: 0 },
        pStr: ::core::ptr::null::<UNICODE>(),
        align: TEXT_ALIGN::ALIGN_LEFT,
        CharCount: 0,
    };
    let mut i: COUNT = 0;
    s.origin = MAKE_POINT(
        (CanvasWidth >> 1 as ::core::ffi::c_int) as COORD,
        ((CanvasHeight >> 1 as ::core::ffi::c_int)
            - ((12 as ::core::ffi::c_int) << resolutionFactor)) as COORD,
    );
    s.frame = SetRelFrameIndex((*pMS).CurFrame, 2 as SIZE);
    uqm_lua_checked!(DrawStamp(&raw mut s), ());
    uqm_lua_checked!(DrawToolTips(pMS, answer as ::core::ffi::c_int), ());
    oldFont = SetContextFont(MicroFont);
    t.align = TEXT_ALIGN::ALIGN_CENTER;
    t.baseline.x = s.origin.x;
    t.baseline.y = (s.origin.y as ::core::ffi::c_int
        - ((20 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    i = 0 as COUNT;
    while i as ::core::ffi::c_int <= 2 as ::core::ffi::c_int {
        t.pStr = GetStringAddress(
            SetAbsStringTableIndex(
                GameStrings,
                (C2Rust_Unnamed_4::MAINMENU_STRING_BASE.0 as ::core::ffi::c_int
                    + 56 as ::core::ffi::c_int
                    + if i == 0 {
                        1 as ::core::ffi::c_int
                    } else if i as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                        2 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    }) as COUNT,
            ),
        );
        t.CharCount = utf8StringCount(t.pStr) as COUNT;
        SetContextForeGroundColor(
            if i as ::core::ffi::c_int == answer as ::core::ffi::c_int {
                if confirm.0 != 0 {
                    colorFromRgb15(0x14 as BYTE, 0x14 as BYTE, 0x14 as BYTE)
                } else {
                    colorFromRgb15(0x1f as BYTE, 0x1f as BYTE, 0 as BYTE)
                }
            } else {
                colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE)
            },
        );
        uqm_lua_checked!(font_DrawText(&raw mut t), ());
        t.baseline.y = (t.baseline.y as ::core::ffi::c_int
            + ((23 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        i = i.wrapping_add(1);
    }
    SetContextFont(oldFont);
}
unsafe extern "C" fn DoDiffChooser(mut pMS: *mut MENU_STATE) -> BOOLEAN {
    static mut LastInputTime: TimeCount = 0;
    static mut InactTimeOut: TimeCount = 0;
    let mut oldRect: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut s: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    let mut oldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    let mut response: BOOLEAN = BOOLEAN::FALSE;
    let mut done: BOOLEAN = BOOLEAN::FALSE;
    let mut a: BYTE = 1 as BYTE;
    InactTimeOut = (if optMainMenuMusic.0 != 0 {
        60 as ::core::ffi::c_int
    } else {
        20 as ::core::ffi::c_int
    } * 840 as ::core::ffi::c_int) as TimeCount;
    LastInputTime = GetTimeCounter();
    oldContext = SetContext(ScreenContext);
    GetContextClipRect(&raw mut oldRect);
    s = SaveContextFrame(::core::ptr::null::<RECT>());
    uqm_lua_checked!(DrawDiffChooser(pMS, a, BOOLEAN::FALSE), BOOLEAN(0));
    FlushGraphics();
    FlushInput();
    while done.0 == 0 {
        uqm_lua_checked!(UpdateInputState(), BOOLEAN(0));
        if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
            & C2Rust_Unnamed_2::CHECK_ABORT.0 as ::core::ffi::c_int != 0
        {
            return BOOLEAN::FALSE
        } else if PulsedInputState
            .menu[C2Rust_Unnamed::KEY_MENU_SELECT.0 as ::core::ffi::c_int as usize] != 0
        {
            done = BOOLEAN::TRUE;
            response = BOOLEAN::TRUE;
            uqm_lua_checked!(DrawDiffChooser(pMS, a, BOOLEAN::TRUE), BOOLEAN(0));
            uqm_lua_checked!(PlayMenuSound(MENU_SOUND_EFFECT::MENU_SOUND_SUCCESS), BOOLEAN(0));
        } else if PulsedInputState
            .menu[C2Rust_Unnamed::KEY_MENU_CANCEL.0 as ::core::ffi::c_int as usize] != 0
            || CurrentInputState
                .menu[C2Rust_Unnamed::KEY_EXIT.0 as ::core::ffi::c_int as usize] != 0
        {
            done = BOOLEAN::TRUE;
            response = BOOLEAN::FALSE;
            uqm_lua_checked!(DrawStamp(&raw mut s), BOOLEAN(0));
        } else if PulsedInputState
            .menu[C2Rust_Unnamed::KEY_MENU_UP.0 as ::core::ffi::c_int as usize] != 0
            || PulsedInputState
                .menu[C2Rust_Unnamed::KEY_MENU_DOWN.0 as ::core::ffi::c_int as usize]
                != 0
            || PulsedInputState
                .menu[C2Rust_Unnamed::KEY_MENU_LEFT.0 as ::core::ffi::c_int as usize]
                != 0
            || PulsedInputState
                .menu[C2Rust_Unnamed::KEY_MENU_RIGHT.0 as ::core::ffi::c_int as usize]
                != 0
        {
            let mut NewState: BYTE = 0;
            NewState = a;
            if PulsedInputState
                .menu[C2Rust_Unnamed::KEY_MENU_UP.0 as ::core::ffi::c_int as usize] != 0
                || PulsedInputState
                    .menu[C2Rust_Unnamed::KEY_MENU_LEFT.0 as ::core::ffi::c_int as usize]
                    != 0
            {
                if NewState as ::core::ffi::c_int
                    == C2Rust_Unnamed_6::EASY_DIFF.0 as ::core::ffi::c_int
                {
                    NewState = C2Rust_Unnamed_6::HARD_DIFF.0 as ::core::ffi::c_int
                        as BYTE;
                } else {
                    NewState = NewState.wrapping_sub(1);
                }
            } else if PulsedInputState
                .menu[C2Rust_Unnamed::KEY_MENU_DOWN.0 as ::core::ffi::c_int as usize]
                != 0
                || PulsedInputState
                    .menu[C2Rust_Unnamed::KEY_MENU_RIGHT.0 as ::core::ffi::c_int
                    as usize] != 0
            {
                if NewState as ::core::ffi::c_int
                    == C2Rust_Unnamed_6::HARD_DIFF.0 as ::core::ffi::c_int
                {
                    NewState = C2Rust_Unnamed_6::EASY_DIFF.0 as ::core::ffi::c_int
                        as BYTE;
                } else {
                    NewState = NewState.wrapping_add(1);
                }
            }
            if NewState as ::core::ffi::c_int != a as ::core::ffi::c_int {
                BatchGraphics();
                uqm_lua_checked!(DrawDiffChooser(pMS, NewState, BOOLEAN::FALSE), BOOLEAN(0));
                UnbatchGraphics();
                a = NewState;
            }
            uqm_lua_checked!(PlayMenuSound(MENU_SOUND_EFFECT::MENU_SOUND_MOVE), BOOLEAN(0));
            LastInputTime = GetTimeCounter();
        }
        uqm_lua_checked!(SleepThread(
            (840 as ::core::ffi::c_int / 30 as ::core::ffi::c_int) as TimePeriod,
        ), BOOLEAN(0));
    }
    if response.0 != 0 {
        optDifficulty = if a == 0 {
            C2Rust_Unnamed_1::OPTVAL_EASY.0 as ::core::ffi::c_int
        } else if a as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
            C2Rust_Unnamed_1::OPTVAL_HARD.0 as ::core::ffi::c_int
        } else {
            C2Rust_Unnamed_0::OPTVAL_NORMAL.0 as ::core::ffi::c_int
        };
    }
    DestroyDrawable(ReleaseDrawable(s.frame));
    FlushInput();
    SetContextClipRect(&raw mut oldRect);
    SetContext(oldContext);
    return response;
}
#[no_mangle]
pub static mut TextCache: [FRAME; 5] = [::core::ptr::null_mut::<FRAME_DESC>(); 5];
unsafe extern "C" fn InitPulseText() {
    let mut frame: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
    let mut OldFrame: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
    let mut leading: SIZE = 0;
    let mut t: TEXT = TEXT {
        baseline: POINT { x: 0, y: 0 },
        pStr: ::core::ptr::null::<UNICODE>(),
        align: TEXT_ALIGN::ALIGN_LEFT,
        CharCount: 0,
    };
    let mut i: COUNT = 0;
    if !TextCache[0usize].is_null() {
        return;
    }
    SetContextFont(SlabFont);
    SetContextBackGroundColor(colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE));
    SetContextForeGroundColor(colorFromRgb15(0x1f as BYTE, 0x1f as BYTE, 0x1f as BYTE));
    GetContextFontLeading(&raw mut leading);
    t.baseline.x = (CanvasWidth >> 1 as ::core::ffi::c_int) as COORD;
    t.baseline.y = (((42 as ::core::ffi::c_int) << resolutionFactor)
        - (if if optWindowType == 0 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            20 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        } << resolutionFactor)) as COORD;
    t.align = TEXT_ALIGN::ALIGN_CENTER;
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    i = C2Rust_Unnamed_5::START_NEW_GAME.0 as ::core::ffi::c_int as COUNT;
    while (i as ::core::ffi::c_int)
        < C2Rust_Unnamed_5::NUM_MENU_ELEMENTS.0 as ::core::ffi::c_int
    {
        t.pStr = GetStringAddress(
            SetAbsStringTableIndex(
                GameStrings,
                (C2Rust_Unnamed_4::MAINMENU_STRING_BASE.0 as ::core::ffi::c_int
                    + 69 as ::core::ffi::c_int + i as ::core::ffi::c_int) as COUNT,
            ),
        );
        frame = CaptureDrawable(
            CreateDrawable(
                ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as CREATE_FLAGS,
                CanvasWidth as SIZE,
                CanvasHeight as SIZE,
                1 as COUNT,
            ),
        );
        SetFrameTransparentColor(frame, colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE));
        OldFrame = SetContextFGFrame(frame);
        uqm_lua_checked!(ClearDrawable(), ());
        SetContextFGFrame(OldFrame);
        OldFrame = SetContextFGFrame(frame);
        uqm_lua_checked!(font_DrawText(&raw mut t), ());
        SetContextFGFrame(OldFrame);
        TextCache[i as usize] = frame;
        t.baseline.y = (t.baseline.y as ::core::ffi::c_int
            + leading as ::core::ffi::c_int) as COORD;
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn DrawRestartMenuGraphic(mut pMS: *mut MENU_STATE) {
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
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
    let mut buf: [UNICODE; 64] = [0; 64];
    let mut i: COUNT = 0;
    let mut leading: SIZE = 0;
    s.frame = (*pMS).CurFrame;
    GetFrameRect(s.frame, &raw mut r);
    s.origin.x = (CanvasWidth - r.extent.width as ::core::ffi::c_int
        >> 1 as ::core::ffi::c_int) as COORD;
    s.origin.y = (CanvasHeight - r.extent.height as ::core::ffi::c_int
        >> 1 as ::core::ffi::c_int) as COORD;
    SetContextBackGroundColor(colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE));
    BatchGraphics();
    uqm_lua_checked!(ClearDrawable(), ());
    FlushColorXForms();
    uqm_lua_checked!(DrawStamp(&raw mut s), ());
    s.frame = IncFrameIndex((*pMS).CurFrame);
    uqm_lua_checked!(DrawStamp(&raw mut s), ());
    SetContextFont(SlabFont);
    GetContextFontLeading(&raw mut leading);
    t.baseline.x = (CanvasWidth >> 1 as ::core::ffi::c_int) as COORD;
    t.baseline.y = (((42 as ::core::ffi::c_int) << resolutionFactor)
        - (if if optWindowType == 0 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            20 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        } << resolutionFactor)) as COORD;
    t.align = TEXT_ALIGN::ALIGN_CENTER;
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    SetContextForeGroundColor(
        buildColorRgba(0 as BYTE, 0x73 as BYTE, 0 as BYTE, 0xff as BYTE),
    );
    i = C2Rust_Unnamed_5::START_NEW_GAME.0 as ::core::ffi::c_int as COUNT;
    while (i as ::core::ffi::c_int)
        < C2Rust_Unnamed_5::NUM_MENU_ELEMENTS.0 as ::core::ffi::c_int
    {
        t.pStr = GetStringAddress(
            SetAbsStringTableIndex(
                GameStrings,
                (C2Rust_Unnamed_4::MAINMENU_STRING_BASE.0 as ::core::ffi::c_int
                    + 69 as ::core::ffi::c_int + i as ::core::ffi::c_int) as COUNT,
            ),
        );
        uqm_lua_checked!(font_DrawText(&raw mut t), ());
        t.baseline.y = (t.baseline.y as ::core::ffi::c_int
            + leading as ::core::ffi::c_int) as COORD;
        i = i.wrapping_add(1);
    }
    SetContextFont(TinyFont);
    SetContextForeGroundColor(colorFromRgb15(0x1f as BYTE, 0x1f as BYTE, 0x1f as BYTE));
    snprintf(
        &raw mut buf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[UNICODE; 64]>(),
        b"v%d.%d.%d %s\0".as_ptr() as *const ::core::ffi::c_char,
        0 as ::core::ffi::c_int,
        8 as ::core::ffi::c_int,
        5 as ::core::ffi::c_int,
        if if resolutionFactor != 2 as ::core::ffi::c_uint {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } == 0
        {
            b"MegaMod Debug\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"HD MegaMod Debug\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    t.pStr = &raw mut buf as *mut UNICODE;
    t.baseline.x = (CanvasWidth - ((2 as ::core::ffi::c_int) << resolutionFactor))
        as COORD;
    t.baseline.y = (CanvasHeight - ((2 as ::core::ffi::c_int) << resolutionFactor))
        as COORD;
    t.align = TEXT_ALIGN::ALIGN_RIGHT;
    uqm_lua_checked!(font_DrawText(&raw mut t), ());
    if optMainMenuMusic.0 != 0 {
        snprintf(
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[UNICODE; 64]>(),
            b"%s %s\0".as_ptr() as *const ::core::ffi::c_char,
            GetStringAddress(
                SetAbsStringTableIndex(
                    GameStrings,
                    (C2Rust_Unnamed_4::MAINMENU_STRING_BASE.0 as ::core::ffi::c_int
                        + 61 as ::core::ffi::c_int) as COUNT,
                ),
            ),
            GetStringAddress(
                SetAbsStringTableIndex(
                    GameStrings,
                    (C2Rust_Unnamed_4::MAINMENU_STRING_BASE.0 as ::core::ffi::c_int
                        + 62 as ::core::ffi::c_int + Rando as ::core::ffi::c_int)
                        as COUNT,
                ),
            ),
        );
        t.baseline.x = ((2 as ::core::ffi::c_int) << resolutionFactor) as COORD;
        t.baseline.y = (CanvasHeight - ((2 as ::core::ffi::c_int) << resolutionFactor))
            as COORD;
        t.align = TEXT_ALIGN::ALIGN_LEFT;
        uqm_lua_checked!(font_DrawText(&raw mut t), ());
    }
    UnbatchGraphics();
}
unsafe extern "C" fn DrawRestartMenu(
    mut pMS: *mut MENU_STATE,
    mut NewState: BYTE,
    mut f: FRAME,
) {
    let mut origin: POINT = POINT { x: 0, y: 0 };
    origin.x = 0 as COORD;
    origin.y = 0 as COORD;
    uqm_lua_checked!(Flash_setOverlay(
        (*pMS).flashContext,
        &raw mut origin,
        TextCache[NewState as usize],
        BOOLEAN::FALSE,
    ), ());
}
unsafe extern "C" fn RestartMessage() -> BOOLEAN {
    if optRequiresRestart.0 == 0 {
        return BOOLEAN::FALSE;
    }
    uqm_lua_checked!(SetFlashRect(::core::ptr::null::<RECT>(), BOOLEAN::FALSE), BOOLEAN(0));
    uqm_lua_checked!(DoPopupWindow(
        GetStringAddress(
            SetAbsStringTableIndex(
                GameStrings,
                (C2Rust_Unnamed_4::MAINMENU_STRING_BASE.0 as ::core::ffi::c_int
                    + 35 as ::core::ffi::c_int) as COUNT,
            ),
        ),
    ), BOOLEAN(0));
    SetMenuSounds(
        (((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as MENU_SOUND_FLAGS
            as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as MENU_SOUND_FLAGS
                as ::core::ffi::c_int) as MENU_SOUND_FLAGS,
        ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as MENU_SOUND_FLAGS,
    );
    SetTransitionSource(::core::ptr::null::<RECT>());
    uqm_lua_checked!(SleepThreadUntil(
        FadeScreen(
            ScreenFadeType::FadeAllToBlack,
            (840 as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as SIZE,
        ),
    ), BOOLEAN(0));
    GlobData.Game_state.CurrentActivity = C2Rust_Unnamed_2::CHECK_ABORT.0
        as ::core::ffi::c_int as ACTIVITY;
    restartGame = BOOLEAN::TRUE;
    return BOOLEAN::TRUE;
}
unsafe extern "C" fn DoRestart(mut pMS: *mut MENU_STATE) -> BOOLEAN {
    static mut LastInputTime: TimeCount = 0;
    static mut InactTimeOut: TimeCount = 0;
    let mut TimeIn: TimeCount = GetTimeCounter();
    let mut webPage: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    ::core::ptr::write_volatile(&raw mut GamePaused, BOOLEAN::FALSE);
    if optWindowType < 2 as DWORD {
        optMeleeToolTips = OPT_ENABLABLE(
            BOOLEAN::FALSE.0 as ::core::ffi::c_int as ::core::ffi::c_uint,
        );
    }
    if optSuperMelee.0 != 0 && optLoadGame.0 == 0 {
        (*pMS).CurState = C2Rust_Unnamed_5::PLAY_SUPER_MELEE.0 as ::core::ffi::c_int
            as BYTE;
        PulsedInputState
            .menu[C2Rust_Unnamed::KEY_MENU_SELECT.0 as ::core::ffi::c_int as usize] = 65535
            as ::core::ffi::c_int;
    } else if optLoadGame.0 != 0 && optSuperMelee.0 == 0 {
        (*pMS).CurState = C2Rust_Unnamed_5::LOAD_SAVED_GAME.0 as ::core::ffi::c_int
            as BYTE;
        PulsedInputState
            .menu[C2Rust_Unnamed::KEY_MENU_SELECT.0 as ::core::ffi::c_int as usize] = 65535
            as ::core::ffi::c_int;
    }
    if (*pMS).Initialized as ::core::ffi::c_int != 0
        && GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
            & C2Rust_Unnamed_2::CHECK_ABORT.0 as ::core::ffi::c_int == 0
    {
        uqm_lua_checked!(Flash_process((*pMS).flashContext), BOOLEAN(0));
    }
    if (*pMS).Initialized as ::core::ffi::c_int != 0
        && GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
            & C2Rust_Unnamed_2::CHECK_ABORT.0 as ::core::ffi::c_int == 0
    {
        webPage = pollWebNetplayPage();
    }
    if (*pMS).Initialized == 0 {
        (*pMS).hMusic = ::core::ptr::null_mut::<*mut TFB_SoundSample>();
        uqm_lua_checked!(InitMenuMusic(), BOOLEAN(0));
        uqm_lua_checked!(InitPulseText(), BOOLEAN(0));
        ResetMusicResume();
        InactTimeOut = (if optMainMenuMusic.0 != 0 {
            60 as ::core::ffi::c_int
        } else {
            20 as ::core::ffi::c_int
        } * 840 as ::core::ffi::c_int) as TimeCount;
        (*pMS).flashContext = uqm_lua_checked!(Flash_createOverlay(
            ScreenContext,
            ::core::ptr::null::<POINT>(),
            ::core::ptr::null_mut::<FRAME_DESC>(),
        ), BOOLEAN(0));
        uqm_lua_checked!(Flash_setMergeFactors(
            (*pMS).flashContext,
            -3 as ::core::ffi::c_int,
            3 as ::core::ffi::c_int,
            16 as ::core::ffi::c_int,
        ), BOOLEAN(0));
        Flash_setSpeed(
            (*pMS).flashContext,
            (6 as ::core::ffi::c_int * 840 as ::core::ffi::c_int
                / 14 as ::core::ffi::c_int) as TimeCount,
            0 as TimeCount,
            (6 as ::core::ffi::c_int * 840 as ::core::ffi::c_int
                / 14 as ::core::ffi::c_int) as TimeCount,
            0 as TimeCount,
        );
        Flash_setFrameTime(
            (*pMS).flashContext,
            (840 as ::core::ffi::c_int / 16 as ::core::ffi::c_int) as TimeCount,
        );
        uqm_lua_checked!(Flash_setState(
            (*pMS).flashContext,
            FlashState::FlashState_fadeIn,
            (3 as ::core::ffi::c_int * 840 as ::core::ffi::c_int
                / 16 as ::core::ffi::c_int) as TimeCount,
        ), BOOLEAN(0));
        Flash_setPulseBox((*pMS).flashContext, BOOLEAN::FALSE);
        uqm_lua_checked!(DrawRestartMenu(pMS, (*pMS).CurState, ::core::ptr::null_mut::<FRAME_DESC>()), BOOLEAN(0));
        uqm_lua_checked!(Flash_start((*pMS).flashContext), BOOLEAN(0));
        LastInputTime = GetTimeCounter();
        (*pMS).Initialized = BOOLEAN::TRUE.0 as ::core::ffi::c_int as SIZE;
        uqm_lua_checked!(SleepThreadUntil(
            FadeScreen(
                ScreenFadeType::FadeAllToColor,
                (840 as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as SIZE,
            ),
        ), BOOLEAN(0));
        setBrowserGameScreen(b"mainMenu\0".as_ptr() as *const ::core::ffi::c_char);
        announceWebNetplayHook();
    } else if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
        & C2Rust_Unnamed_2::CHECK_ABORT.0 as ::core::ffi::c_int != 0
    {
        uqm_lua_checked!(SleepThreadUntil(
            FadeScreen(
                ScreenFadeType::FadeAllToBlack,
                (840 as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as SIZE,
            ),
        ), BOOLEAN(0));
        return BOOLEAN::FALSE;
    } else if takeWebNetplayRole(webPage).0 != 0 {
        GlobData.Game_state.CurrentActivity = C2Rust_Unnamed_2::SUPER_MELEE.0
            as ::core::ffi::c_int as ACTIVITY;
        setBrowserGameScreen(b"transition\0".as_ptr() as *const ::core::ffi::c_char);
        Flash_pause((*pMS).flashContext);
        return BOOLEAN::FALSE;
    } else if PulsedInputState
        .menu[C2Rust_Unnamed::KEY_MENU_SELECT.0 as ::core::ffi::c_int as usize] != 0
    {
        setBrowserGameScreen(b"transition\0".as_ptr() as *const ::core::ffi::c_char);
        match (*pMS).CurState as ::core::ffi::c_int {
            0 => {
                if optCustomSeed == 404 as ::core::ffi::c_int {
                    uqm_lua_checked!(SetFlashRect(::core::ptr::null::<RECT>(), BOOLEAN::FALSE), BOOLEAN(0));
                    uqm_lua_checked!(DoPopupWindow(
                        GetStringAddress(
                            SetAbsStringTableIndex(
                                GameStrings,
                                (C2Rust_Unnamed_4::MAINMENU_STRING_BASE.0
                                    as ::core::ffi::c_int + 65 as ::core::ffi::c_int) as COUNT,
                            ),
                        ),
                    ), BOOLEAN(0));
                    SetMenuSounds(
                        (((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int)
                            as MENU_SOUND_FLAGS as ::core::ffi::c_int
                            | ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
                                as MENU_SOUND_FLAGS as ::core::ffi::c_int)
                            as MENU_SOUND_FLAGS,
                        ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                            as MENU_SOUND_FLAGS,
                    );
                    SetTransitionSource(::core::ptr::null::<RECT>());
                    uqm_lua_checked!(SleepThreadUntil(
                        FadeScreen(
                            ScreenFadeType::FadeAllToBlack,
                            (840 as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as SIZE,
                        ),
                    ), BOOLEAN(0));
                    GlobData.Game_state.CurrentActivity = C2Rust_Unnamed_2::CHECK_ABORT.0
                        as ::core::ffi::c_int as ACTIVITY;
                    restartGame = BOOLEAN::TRUE;
                } else {
                    if optDiffChooser
                        == C2Rust_Unnamed_1::OPTVAL_IMPO.0 as ::core::ffi::c_int
                    {
                        Flash_pause((*pMS).flashContext);
                        uqm_lua_checked!(Flash_setState(
                            (*pMS).flashContext,
                            FlashState::FlashState_fadeIn,
                            (3 as ::core::ffi::c_int * 840 as ::core::ffi::c_int
                                / 16 as ::core::ffi::c_int) as TimeCount,
                        ), BOOLEAN(0));
                        if uqm_lua_checked!(DoDiffChooser(pMS), BOOLEAN(0)).0 == 0 {
                            LastInputTime = GetTimeCounter();
                            if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
                                != !(0 as ::core::ffi::c_int) as ACTIVITY
                                    as ::core::ffi::c_int
                            {
                                Flash_continue((*pMS).flashContext);
                                setBrowserGameScreen(
                                    b"mainMenu\0".as_ptr() as *const ::core::ffi::c_char,
                                );
                            }
                            return BOOLEAN::TRUE;
                        }
                        Flash_continue((*pMS).flashContext);
                    }
                    LastActivity = (C2Rust_Unnamed_2::CHECK_LOAD.0 as ::core::ffi::c_int
                        | C2Rust_Unnamed_2::CHECK_RESTART.0 as ::core::ffi::c_int)
                        as ACTIVITY;
                    GlobData.Game_state.CurrentActivity = C2Rust_Unnamed_2::IN_INTERPLANETARY
                        .0 as ::core::ffi::c_int as ACTIVITY;
                }
            }
            1 => {
                LastActivity = C2Rust_Unnamed_2::CHECK_LOAD.0 as ::core::ffi::c_int
                    as ACTIVITY;
                GlobData.Game_state.CurrentActivity = C2Rust_Unnamed_2::IN_INTERPLANETARY
                    .0 as ::core::ffi::c_int as ACTIVITY;
                optLoadGame = BOOLEAN::FALSE;
            }
            2 => {
                optShipSeed = OPT_ENABLABLE::OPTVAL_DISABLED;
                uqm_lua_checked!(ReloadMasterShipList(None), BOOLEAN(0));
                GlobData.Game_state.CurrentActivity = C2Rust_Unnamed_2::SUPER_MELEE.0
                    as ::core::ffi::c_int as ACTIVITY;
                optSuperMelee = BOOLEAN::FALSE;
            }
            3 => {
                Flash_pause((*pMS).flashContext);
                uqm_lua_checked!(Flash_setState(
                    (*pMS).flashContext,
                    FlashState::FlashState_fadeIn,
                    (3 as ::core::ffi::c_int * 840 as ::core::ffi::c_int
                        / 16 as ::core::ffi::c_int) as TimeCount,
                ), BOOLEAN(0));
                uqm_lua_checked!(SetupMenu(), BOOLEAN(0));
                if optRequiresReload.0 != 0 {
                    return BOOLEAN::FALSE;
                }
                LastInputTime = GetTimeCounter();
                InactTimeOut = (if optMainMenuMusic.0 != 0 {
                    60 as ::core::ffi::c_int
                } else {
                    20 as ::core::ffi::c_int
                } * 840 as ::core::ffi::c_int) as TimeCount;
                SetTransitionSource(::core::ptr::null::<RECT>());
                BatchGraphics();
                uqm_lua_checked!(DrawRestartMenuGraphic(pMS), BOOLEAN(0));
                uqm_lua_checked!(ScreenTransition(3 as ::core::ffi::c_int, ::core::ptr::null::<RECT>()), BOOLEAN(0));
                Flash_UpdateOriginal((*pMS).flashContext);
                uqm_lua_checked!(DrawRestartMenu(
                    pMS,
                    (*pMS).CurState,
                    ::core::ptr::null_mut::<FRAME_DESC>(),
                ), BOOLEAN(0));
                Flash_continue((*pMS).flashContext);
                UnbatchGraphics();
                if uqm_lua_checked!(RestartMessage(), BOOLEAN(0)).0 == 0 {
                    setBrowserGameScreen(
                        b"mainMenu\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                }
                return BOOLEAN::TRUE;
            }
            4 => {
                uqm_lua_checked!(SleepThreadUntil(
                    FadeScreen(
                        ScreenFadeType::FadeAllToBlack,
                        (840 as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as SIZE,
                    ),
                ), BOOLEAN(0));
                GlobData.Game_state.CurrentActivity = C2Rust_Unnamed_2::CHECK_ABORT.0
                    as ::core::ffi::c_int as ACTIVITY;
            }
            _ => {}
        }
        Flash_pause((*pMS).flashContext);
        return BOOLEAN::FALSE;
    } else if PulsedInputState
        .menu[C2Rust_Unnamed::KEY_MENU_UP.0 as ::core::ffi::c_int as usize] != 0
        || PulsedInputState
            .menu[C2Rust_Unnamed::KEY_MENU_DOWN.0 as ::core::ffi::c_int as usize] != 0
    {
        let mut NewState: BYTE = 0;
        NewState = (*pMS).CurState;
        if PulsedInputState
            .menu[C2Rust_Unnamed::KEY_MENU_UP.0 as ::core::ffi::c_int as usize] != 0
        {
            if NewState as ::core::ffi::c_int
                == C2Rust_Unnamed_5::START_NEW_GAME.0 as ::core::ffi::c_int
            {
                NewState = C2Rust_Unnamed_5::QUIT_GAME.0 as ::core::ffi::c_int as BYTE;
            } else {
                NewState = NewState.wrapping_sub(1);
            }
        } else if PulsedInputState
            .menu[C2Rust_Unnamed::KEY_MENU_DOWN.0 as ::core::ffi::c_int as usize] != 0
        {
            if NewState as ::core::ffi::c_int
                == C2Rust_Unnamed_5::QUIT_GAME.0 as ::core::ffi::c_int
            {
                NewState = C2Rust_Unnamed_5::START_NEW_GAME.0 as ::core::ffi::c_int
                    as BYTE;
            } else {
                NewState = NewState.wrapping_add(1);
            }
        }
        if NewState as ::core::ffi::c_int != (*pMS).CurState as ::core::ffi::c_int {
            BatchGraphics();
            uqm_lua_checked!(DrawRestartMenu(pMS, NewState, ::core::ptr::null_mut::<FRAME_DESC>()), BOOLEAN(0));
            UnbatchGraphics();
            (*pMS).CurState = NewState;
        }
        LastInputTime = GetTimeCounter();
    } else if PulsedInputState
        .menu[C2Rust_Unnamed::KEY_MENU_LEFT.0 as ::core::ffi::c_int as usize] != 0
        || PulsedInputState
            .menu[C2Rust_Unnamed::KEY_MENU_RIGHT.0 as ::core::ffi::c_int as usize] != 0
    {
        LastInputTime = GetTimeCounter();
    }
    uqm_lua_checked!(SleepThreadUntil(
        TimeIn
            .wrapping_add(
                (840 as ::core::ffi::c_int / 30 as ::core::ffi::c_int) as TimeCount,
            ),
    ), BOOLEAN(0));
    return BOOLEAN::TRUE;
}
unsafe extern "C" fn RestartMenu(mut pMS: *mut MENU_STATE) -> BOOLEAN {
    let mut TimeOut: TimeCount = 0;
    let mut i: COUNT = 0;
    ReinitQueue((&raw mut race_q as *mut QUEUE).offset(0isize));
    ReinitQueue((&raw mut race_q as *mut QUEUE).offset(1isize));
    SetContext(ScreenContext);
    GlobData.Game_state.CurrentActivity = (GlobData.Game_state.CurrentActivity
        as ::core::ffi::c_int | C2Rust_Unnamed_2::CHECK_ABORT.0 as ::core::ffi::c_int)
        as ACTIVITY;
    if GlobData.SIS_state.CrewEnlisted as ::core::ffi::c_int
        == !(0 as ::core::ffi::c_int) as COUNT as ::core::ffi::c_int
        && uqm_lua_checked!(getGameStateUint(
            b"UTWIG_BOMB_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
        ), BOOLEAN(0)) != 0
        && uqm_lua_checked!(getGameStateUint(b"UTWIG_BOMB\0".as_ptr() as *const ::core::ffi::c_char), BOOLEAN(0)) == 0
        && DeathBySuicide.0 != 0
    {
        uqm_lua_checked!(setGameStateUint(
            b"UTWIG_BOMB_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
            0 as DWORD,
        ), BOOLEAN(0));
        uqm_lua_checked!(SleepThreadUntil(
            FadeScreen(
                    ScreenFadeType::FadeAllToWhite,
                    (840 as ::core::ffi::c_int / 8 as ::core::ffi::c_int) as SIZE,
                )
                .wrapping_add(
                    (840 as ::core::ffi::c_int / 60 as ::core::ffi::c_int) as TimeCount,
                ),
        ), BOOLEAN(0));
        SetContextBackGroundColor(
            colorFromRgb15(0x1f as BYTE, 0x1f as BYTE, 0x1f as BYTE),
        );
        uqm_lua_checked!(ClearDrawable(), BOOLEAN(0));
        FlushColorXForms();
        TimeOut = (840 as ::core::ffi::c_int / 8 as ::core::ffi::c_int) as TimeCount;
        GlobData.Game_state.CurrentActivity = C2Rust_Unnamed_2::IN_ENCOUNTER.0
            as ::core::ffi::c_int as ACTIVITY;
        if optGameOver.0 != 0 {
            uqm_lua_checked!(GameOver(C2Rust_Unnamed_3::SUICIDE.0 as ::core::ffi::c_int as BYTE), BOOLEAN(0));
        }
        DeathBySuicide = BOOLEAN::FALSE;
        uqm_lua_checked!(FreeGameData(), BOOLEAN(0));
        GlobData.Game_state.CurrentActivity = C2Rust_Unnamed_2::CHECK_ABORT.0
            as ::core::ffi::c_int as ACTIVITY;
    } else {
        TimeOut = (840 as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as TimeCount;
        if GlobData.SIS_state.CrewEnlisted as ::core::ffi::c_int
            == !(0 as ::core::ffi::c_int) as COUNT as ::core::ffi::c_int
        {
            GlobData.Game_state.CurrentActivity = C2Rust_Unnamed_2::IN_ENCOUNTER.0
                as ::core::ffi::c_int as ACTIVITY;
            if DeathByMelee.0 != 0 {
                if optGameOver.0 != 0 {
                    uqm_lua_checked!(GameOver(
                        C2Rust_Unnamed_3::DIED_IN_BATTLE.0 as ::core::ffi::c_int as BYTE,
                    ), BOOLEAN(0));
                }
                DeathByMelee = BOOLEAN::FALSE;
            } else if DeathBySurrender.0 != 0 {
                if optGameOver.0 != 0 {
                    uqm_lua_checked!(GameOver(
                        C2Rust_Unnamed_3::SURRENDERED.0 as ::core::ffi::c_int as BYTE,
                    ), BOOLEAN(0));
                }
                DeathBySurrender = BOOLEAN::FALSE;
            }
        }
        if LastActivity as BYTE as ::core::ffi::c_int
            == C2Rust_Unnamed_2::WON_LAST_BATTLE.0 as ::core::ffi::c_int
        {
            GlobData.Game_state.CurrentActivity = C2Rust_Unnamed_2::WON_LAST_BATTLE.0
                as ::core::ffi::c_int as ACTIVITY;
            uqm_lua_checked!(Victory(), BOOLEAN(0));
            uqm_lua_checked!(Credits(BOOLEAN::TRUE), BOOLEAN(0));
        }
        uqm_lua_checked!(FreeGameData(), BOOLEAN(0));
        GlobData.Game_state.CurrentActivity = C2Rust_Unnamed_2::CHECK_ABORT.0
            as ::core::ffi::c_int as ACTIVITY;
    }
    LastActivity = 0 as ACTIVITY;
    NextActivity = 0 as ACTIVITY;
    uqm_lua_checked!(SleepThreadUntil(FadeScreen(ScreenFadeType::FadeAllToBlack, TimeOut as SIZE)), BOOLEAN(0));
    if TimeOut == (840 as ::core::ffi::c_int / 8 as ::core::ffi::c_int) as TimeCount {
        uqm_lua_checked!(SleepThread((840 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as TimePeriod), BOOLEAN(0));
    }
    (*pMS).CurFrame = CaptureDrawable(
        uqm_lua_checked!(LoadGraphicInstance(b"graphics.newgame\0".as_ptr() as RESOURCE), BOOLEAN(0)) as DRAWABLE,
    );
    uqm_lua_checked!(DrawRestartMenuGraphic(pMS), BOOLEAN(0));
    GlobData.Game_state.CurrentActivity = (GlobData.Game_state.CurrentActivity
        as ::core::ffi::c_int & !(C2Rust_Unnamed_2::CHECK_ABORT.0 as ::core::ffi::c_int))
        as ACTIVITY;
    SetMenuSounds(
        (((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as MENU_SOUND_FLAGS
            as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as MENU_SOUND_FLAGS
                as ::core::ffi::c_int) as MENU_SOUND_FLAGS,
        ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as MENU_SOUND_FLAGS,
    );
    SetDefaultMenuRepeatDelay();
    uqm_lua_checked!(DoInput(pMS as *mut ::core::ffi::c_void, BOOLEAN::TRUE), BOOLEAN(0));
    if !(optRequiresRestart.0 != 0 || optRequiresReload.0 != 0) {
        uqm_lua_checked!(UninitMenuMusic(), BOOLEAN(0));
    }
    uqm_lua_checked!(Flash_terminate((*pMS).flashContext), BOOLEAN(0));
    (*pMS).flashContext = ::core::ptr::null_mut::<FlashContext>();
    DestroyDrawable(ReleaseDrawable((*pMS).CurFrame));
    (*pMS).CurFrame = ::core::ptr::null_mut::<FRAME_DESC>();
    i = C2Rust_Unnamed_5::START_NEW_GAME.0 as ::core::ffi::c_int as COUNT;
    while (i as ::core::ffi::c_int)
        < C2Rust_Unnamed_5::NUM_MENU_ELEMENTS.0 as ::core::ffi::c_int
    {
        DestroyDrawable(ReleaseDrawable(TextCache[i as usize]));
        TextCache[i as usize] = ::core::ptr::null_mut::<FRAME_DESC>();
        i = i.wrapping_add(1);
    }
    if optRequiresReload.0 != 0 {
        uqm_lua_checked!(Reload(), BOOLEAN(0));
    }
    if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
        == !(0 as ::core::ffi::c_int) as ACTIVITY as ::core::ffi::c_int
    {
        return BOOLEAN::FALSE;
    }
    if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
        & C2Rust_Unnamed_2::CHECK_ABORT.0 as ::core::ffi::c_int != 0
    {
        return BOOLEAN::FALSE;
    }
    TimeOut = FadeScreen(
        ScreenFadeType::FadeAllToBlack,
        (840 as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as SIZE,
    ) as TimeCount;
    uqm_lua_checked!(SleepThreadUntil(TimeOut), BOOLEAN(0));
    FlushColorXForms();
    SeedRandomNumbers();
    setBrowserGameScreen(b"transition\0".as_ptr() as *const ::core::ffi::c_char);
    return BOOLEAN(
        (GlobData.Game_state.CurrentActivity as BYTE as ::core::ffi::c_int
            != C2Rust_Unnamed_2::SUPER_MELEE.0 as ::core::ffi::c_int)
            as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
}
unsafe extern "C" fn TryStartGame() -> BOOLEAN {
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
    LastActivity = GlobData.Game_state.CurrentActivity;
    GlobData.Game_state.CurrentActivity = 0 as ACTIVITY;
    memset(
        &raw mut MenuState as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<MENU_STATE>(),
    );
    MenuState.InputFunc = Some(
        DoRestart as unsafe extern "C" fn(*mut MENU_STATE) -> BOOLEAN,
    ) as Option<unsafe extern "C" fn(*mut menu_state) -> BOOLEAN>;
    if webNetplayRole >= 0 as ::core::ffi::c_int {
        uqm_lua_checked!(playWebMelee(), BOOLEAN(0));
    }
    while uqm_lua_checked!(RestartMenu(&raw mut MenuState), BOOLEAN(0)).0 == 0 {
        if webNetplayRole >= 0 as ::core::ffi::c_int {
            uqm_lua_checked!(playWebMelee(), BOOLEAN(0));
            MenuState.Initialized = BOOLEAN::FALSE.0 as ::core::ffi::c_int as SIZE;
        } else if GlobData.Game_state.CurrentActivity as BYTE as ::core::ffi::c_int
            == C2Rust_Unnamed_2::SUPER_MELEE.0 as ::core::ffi::c_int
            && GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
                & C2Rust_Unnamed_2::CHECK_ABORT.0 as ::core::ffi::c_int == 0
        {
            uqm_lua_checked!(FreeGameData(), BOOLEAN(0));
            setBrowserGameScreen(b"melee\0".as_ptr() as *const ::core::ffi::c_char);
            uqm_lua_checked!(Melee(), BOOLEAN(0));
            MenuState.Initialized = BOOLEAN::FALSE.0 as ::core::ffi::c_int as SIZE;
        } else if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
            == !(0 as ::core::ffi::c_int) as ACTIVITY as ::core::ffi::c_int
        {
            uqm_lua_checked!(SleepThreadUntil(
                FadeScreen(
                    ScreenFadeType::FadeAllToBlack,
                    (840 as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as SIZE,
                ),
            ), BOOLEAN(0));
            return BOOLEAN::FALSE;
        } else if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
            & C2Rust_Unnamed_2::CHECK_ABORT.0 as ::core::ffi::c_int != 0
        {
            return BOOLEAN::FALSE
        }
    }
    return BOOLEAN::TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn StartGame() -> BOOLEAN {
    loop {
        while uqm_lua_checked!(TryStartGame(), BOOLEAN(0)).0 == 0 {
            if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
                == !(0 as ::core::ffi::c_int) as ACTIVITY as ::core::ffi::c_int
            {
                GlobData.Game_state.CurrentActivity = 0 as ACTIVITY;
                if optRequiresRestart.0 != 0 || optRequiresReload.0 != 0 {
                    optRequiresReload = BOOLEAN::FALSE;
                    optRequiresRestart = optRequiresReload;
                }
            }
            if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
                & C2Rust_Unnamed_2::CHECK_ABORT.0 as ::core::ffi::c_int != 0
            {
                return BOOLEAN::FALSE;
            }
        }
        if LastActivity as ::core::ffi::c_int
            & C2Rust_Unnamed_2::CHECK_RESTART.0 as ::core::ffi::c_int != 0
        {
            if optSkipIntro.0 == 0 {
                uqm_lua_checked!(Introduction(), BOOLEAN(0));
            }
        }
        if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
            & C2Rust_Unnamed_2::CHECK_ABORT.0 as ::core::ffi::c_int == 0
        {
            break;
        }
    }
    extern "C" {
        static mut starmap_array: [STAR_DESC; 0];
    }
    extern "C" {
        static element_array: [BYTE; 0];
    }
    extern "C" {
        static planet_array: [PlanetFrame; 0];
    }
    extern "C" {
        static mut constell_array: [POINT; 0];
    }
    let mut i: COUNT = 0;
    i = 0 as COUNT;
    while (i as ::core::ffi::c_int)
        < 502 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 15 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
    {
        *(&raw mut star_array as *mut STAR_DESC).offset(i as isize) = *(&raw mut starmap_array
            as *mut STAR_DESC)
            .offset(i as isize);
        i = i.wrapping_add(1);
    }
    Elements = &raw const element_array as *const BYTE;
    PlanData = &raw const planet_array as *const PlanetFrame;
    constel_array = &raw mut constell_array as *mut POINT;
    *(&raw mut PlayerControl as *mut BYTE).offset(0isize) = (((1 as ::core::ffi::c_int)
        << 0 as ::core::ffi::c_int) as BYTE as ::core::ffi::c_int
        | ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as BYTE
            as ::core::ffi::c_int) as BYTE;
    *(&raw mut PlayerControl as *mut BYTE).offset(1isize) = (((1 as ::core::ffi::c_int)
        << 1 as ::core::ffi::c_int) as BYTE as ::core::ffi::c_int
        | ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as BYTE
            as ::core::ffi::c_int
        | ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as BYTE
            as ::core::ffi::c_int) as BYTE;
    return BOOLEAN::TRUE;
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
