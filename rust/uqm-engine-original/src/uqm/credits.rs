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

// Rust translation of src/uqm/credits.c.
extern "C" {
    pub type font_desc;
    pub type string_table_entry;
    pub type string_table;
    pub type tfb_soundsample;
    pub type context_desc;
    fn sscanf(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn LoadStringTableInstance(res: RESOURCE) -> STRING_TABLE;
    fn DestroyStringTable(StringTable: STRING_TABLE) -> BOOLEAN;
    fn CaptureStringTable(StringTable: STRING_TABLE) -> STRING;
    fn ReleaseStringTable(String: STRING) -> STRING_TABLE;
    fn GetStringTableCount(String: STRING) -> COUNT;
    fn SetAbsStringTableIndex(String: STRING, StringTableIndex: COUNT) -> STRING;
    fn GetStringAddress(String: STRING) -> STRINGPTR;
    fn utf8StringCopy(
        dst: *mut ::core::ffi::c_char,
        size: size_t,
        src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_uchar;
    fn SetContext(Context: CONTEXT) -> CONTEXT;
    fn SetContextForeGroundColor(Color: Color) -> Color;
    fn SetContextBackGroundColor(Color: Color) -> Color;
    fn SetContextFGFrame(Frame: FRAME) -> FRAME;
    fn GetContextFGFrame() -> FRAME;
    fn SetContextClipRect(pRect: *mut RECT) -> BOOLEAN;
    fn GetContextClipRect(pRect: *mut RECT) -> BOOLEAN;
    fn DrawStamp(pStamp: *mut STAMP);
    fn DrawFilledRectangle(pRect: *mut RECT);
    fn font_DrawText(pText: *mut TEXT);
    fn FlushGraphics();
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
    fn SetFrameTransparentColor(_: FRAME, _: Color);
    fn CaptureDrawable(Drawable: DRAWABLE) -> FRAME;
    fn ReleaseDrawable(Frame: FRAME) -> DRAWABLE;
    fn FadeScreen(fadeType: ScreenFadeType, TimeInterval: SIZE) -> DWORD;
    fn FlushColorXForms();
    static mut PulsedInputState: CONTROLLER_INPUT_STATE;
    fn SetInputCallback(_: Option<InputFrameCallback>) -> Option<InputFrameCallback>;
    fn DoInput(pInputState: *mut ::core::ffi::c_void, resetInput: BOOLEAN);
    fn GetTimeCounter() -> TimeCount;
    fn SleepThread(timePeriod: TimePeriod);
    fn SleepThreadUntil(wakeTime: TimeCount);
    fn LoadMusicInstance(res: RESOURCE) -> MUSIC_REF;
    fn DestroyMusic(MusicRef: MUSIC_REF) -> BOOLEAN;
    fn FadeMusic(end_vol: BYTE, TimeInterval: SIZE) -> DWORD;
    fn memset(
        _: *mut ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strchr(
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn strtok(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(_: *const ::core::ffi::c_char) -> size_t;
    static mut GlobData: GLOBDATA;
    fn InitCommunication(which_comm: CONVERSATION) -> COUNT;
    static mut CommWndRect: RECT;
    fn SetCommIntroMode(_: CommIntroMode, howLong: TimeCount);
    fn StopMusic();
    fn PlayMusic(MusicRef: MUSIC_REF, Continuous: BOOLEAN, Priority: BYTE);
    fn SetMenuSounds(sound_0: MENU_SOUND_FLAGS, sound_1: MENU_SOUND_FLAGS);
    static mut ScreenContext: CONTEXT;
    fn abs(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    static mut sliderDisabled: BOOLEAN;
    static mut oscillDisabled: BOOLEAN;
    static mut optSubtitles: BOOLEAN;
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
pub type FONT = *mut FONT_DESC;
pub type FONT_DESC = font_desc;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FONT_SIZE_DEF {
    pub size: ::core::ffi::c_int,
    pub res: RESOURCE,
    pub font: FONT,
}
pub type RESOURCE = *const ::core::ffi::c_char;
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
pub type EXTENT = extent;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct extent {
    pub width: COORD,
    pub height: COORD,
}
pub type COORD = SWORD;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Color {
    pub r: BYTE,
    pub g: BYTE,
    pub b: BYTE,
    pub a: BYTE,
}
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
pub type DRAWABLE = *mut DRAWABLE_DESC;
pub type DRAWABLE_DESC = drawable_desc;
pub type STRING = *mut STRING_TABLE_ENTRY_DESC;
pub type STRING_TABLE_ENTRY_DESC = string_table_entry;
pub type STRING_TABLE = *mut STRING_TABLE_DESC;
pub type STRING_TABLE_DESC = string_table;
pub type MUSIC_REF = *mut *mut TFB_SoundSample;
pub type TFB_SoundSample = tfb_soundsample;
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
pub type CONTEXT = *mut CONTEXT_DESC;
pub type CONTEXT_DESC = context_desc;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CreditTextFrame {
    pub frame: FRAME,
    pub strIndex: ::core::ffi::c_int,
}
pub type InputFrameCallback = unsafe extern "C" fn() -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CREDITS_INPUT_STATE {
    pub InputFunc: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> BOOLEAN>,
    pub AllowCancel: BOOLEAN,
    pub AllowSpeedChange: BOOLEAN,
    pub CloseWhenDone: BOOLEAN,
    pub CloseTimeOut: DWORD,
}
pub type MENU_SOUND_FLAGS = UWORD;
pub type TimePeriod = DWORD;
pub type CONTROLLER_INPUT_STATE = _controller_input_state;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _controller_input_state {
    pub key: [[::core::ffi::c_int; 7]; 6],
    pub menu: [::core::ffi::c_int; 24],
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
pub type STAMP = stamp;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stamp {
    pub origin: POINT,
    pub frame: FRAME,
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
pub struct PRIMITIVE {
    pub Links: PRIM_LINKS,
    pub Type: GRAPHICS_PRIM,
    pub color: Color,
    pub Object: PRIM_DESC,
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
    pub ShipName: [UNICODE; 16],
    pub CommanderName: [UNICODE; 16],
    pub PlanetName: [UNICODE; 16],
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct col_format_t {
    pub align: TEXT_ALIGN,
    pub basex: COORD,
}
pub type size_t = usize;
pub type STRINGPTR = *mut ::core::ffi::c_char;
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
static mut CreditsRate: ::core::ffi::c_int = 0;
static mut OutTakesRunning: BOOLEAN = BOOLEAN::FALSE;
static mut CreditsRunning: BOOLEAN = BOOLEAN::FALSE;
static mut CreditsTab: STRING = ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>();
static mut CreditsBack: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
static mut DrawContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
static mut LocalContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
static mut CreditsFrame: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
static mut CreditsExtent: EXTENT = EXTENT { width: 0, height: 0 };
static mut textFrames: [CreditTextFrame; 32] = [CreditTextFrame {
    frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    strIndex: 0,
}; 32];
static mut firstFrame: ::core::ffi::c_int = 0;
static mut lastFrame: ::core::ffi::c_int = 0;
static mut totalHeight: ::core::ffi::c_int = 0;
static mut curFrameOfs: ::core::ffi::c_int = 0;
static mut CreditsFont: [FONT_SIZE_DEF; 4] = [
    FONT_SIZE_DEF {
        size: 13 as ::core::ffi::c_int,
        res: b"credits.font.pt13\0".as_ptr() as RESOURCE,
        font: ::core::ptr::null_mut::<FONT_DESC>(),
    },
    FONT_SIZE_DEF {
        size: 17 as ::core::ffi::c_int,
        res: b"credits.font.pt17\0".as_ptr() as RESOURCE,
        font: ::core::ptr::null_mut::<FONT_DESC>(),
    },
    FONT_SIZE_DEF {
        size: 45 as ::core::ffi::c_int,
        res: b"credits.font.pt45\0".as_ptr() as RESOURCE,
        font: ::core::ptr::null_mut::<FONT_DESC>(),
    },
    FONT_SIZE_DEF {
        size: 0 as ::core::ffi::c_int,
        res: ::core::ptr::null::<::core::ffi::c_char>(),
        font: ::core::ptr::null_mut::<FONT_DESC>(),
    },
];
unsafe extern "C" fn Credits_MakeTransFrame(
    mut w: ::core::ffi::c_int,
    mut h: ::core::ffi::c_int,
    mut TransColor: Color,
) -> FRAME {
    let mut OldFrame: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
    let mut f: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
    f = CaptureDrawable(
        CreateDrawable(
            ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as CREATE_FLAGS,
            w as SIZE,
            h as SIZE,
            1 as COUNT,
        ),
    );
    SetFrameTransparentColor(f, TransColor);
    OldFrame = SetContextFGFrame(f);
    SetContextBackGroundColor(TransColor);
    ClearDrawable();
    SetContextFGFrame(OldFrame);
    return f;
}
unsafe extern "C" fn ParseTextLines(
    mut Lines: *mut TEXT,
    mut MaxLines: ::core::ffi::c_int,
    mut Buffer: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut pEnd: *const ::core::ffi::c_char = Buffer.offset(strlen(Buffer) as isize);
    i = 0 as ::core::ffi::c_int;
    while i < MaxLines && Buffer < pEnd as *mut ::core::ffi::c_char {
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
        i += 1;
        Lines = Lines.offset(1);
    }
    return i;
}
unsafe extern "C" fn Credits_RenderTextFrame(
    mut TempContext: CONTEXT,
    mut istr: *mut ::core::ffi::c_int,
    mut dir: ::core::ffi::c_int,
    mut BackColor: Color,
    mut ForeColor: Color,
) -> FRAME {
    let mut f: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
    let mut OldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    let mut OldFrame: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
    let mut TextLines: [TEXT; 50] = [TEXT {
        baseline: POINT { x: 0, y: 0 },
        pStr: ::core::ptr::null::<UNICODE>(),
        align: TEXT_ALIGN::ALIGN_LEFT,
        CharCount: 0,
    }; 50];
    let mut pStr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut size: ::core::ffi::c_int = 0;
    let mut salign: [::core::ffi::c_char; 32] = [0; 32];
    let mut scol: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut scaned: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut rows: ::core::ffi::c_int = 0;
    let mut cnt: ::core::ffi::c_int = 0;
    let mut buf: [::core::ffi::c_char; 2048] = [0; 2048];
    let mut fdef: *mut FONT_SIZE_DEF = ::core::ptr::null_mut::<FONT_SIZE_DEF>();
    let mut leading: SIZE = 0;
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
    let mut colfmt: [col_format_t; 5] = [col_format_t {
        align: TEXT_ALIGN::ALIGN_LEFT,
        basex: 0,
    }; 5];
    if *istr < 0 as ::core::ffi::c_int
        || *istr >= GetStringTableCount(CreditsTab) as ::core::ffi::c_int
    {
        let mut next_s: ::core::ffi::c_int = *istr + dir;
        if next_s < 0 as ::core::ffi::c_int
            || next_s >= GetStringTableCount(CreditsTab) as ::core::ffi::c_int
        {
            return ::core::ptr::null_mut::<FRAME_DESC>();
        }
        *istr = next_s;
    }
    while *istr >= 0 as ::core::ffi::c_int
        && *istr < GetStringTableCount(CreditsTab) as ::core::ffi::c_int
    {
        pStr = GetStringAddress(SetAbsStringTableIndex(CreditsTab, *istr as COUNT))
            as *mut ::core::ffi::c_char;
        *istr += dir;
        if !pStr.is_null() && *pStr as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
            break;
        }
    }
    if pStr.is_null() || *pStr as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<FRAME_DESC>();
    }
    if 2 as ::core::ffi::c_int
        != sscanf(
            pStr,
            b"%d %31s %n\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut size,
            &raw mut salign as *mut ::core::ffi::c_char,
            &raw mut scaned,
        ) || size <= 0 as ::core::ffi::c_int
    {
        return ::core::ptr::null_mut::<FRAME_DESC>();
    }
    pStr = pStr.offset(scaned as isize);
    utf8StringCopy(
        &raw mut buf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 2048]>(),
        pStr,
    );
    rows = ParseTextLines(
        &raw mut TextLines as *mut TEXT,
        50 as ::core::ffi::c_int,
        &raw mut buf as *mut ::core::ffi::c_char,
    );
    if rows == 0 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<FRAME_DESC>();
    }
    i = 0 as ::core::ffi::c_int;
    cnt = rows;
    while i < rows {
        let mut nextcol: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut icol: ::core::ffi::c_int = 0;
        TextLines[i as usize].baseline.x = 0 as COORD;
        TextLines[i as usize].baseline.y = (i + 1 as ::core::ffi::c_int) as COORD;
        icol = 1 as ::core::ffi::c_int;
        nextcol = strchr(TextLines[i as usize].pStr, '\t' as ::core::ffi::c_int);
        while icol < 5 as ::core::ffi::c_int && !nextcol.is_null() {
            *nextcol = '\0' as ::core::ffi::c_char;
            nextcol = nextcol.offset(1);
            if cnt < 50 as ::core::ffi::c_int {
                TextLines[cnt as usize].pStr = nextcol;
                TextLines[cnt as usize].CharCount = !(0 as ::core::ffi::c_int) as COUNT;
                TextLines[cnt as usize].baseline.x = icol as COORD;
                TextLines[cnt as usize].baseline.y = (i + 1 as ::core::ffi::c_int)
                    as COORD;
                cnt += 1;
            }
            icol += 1;
            nextcol = strchr(nextcol, '\t' as ::core::ffi::c_int);
        }
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < 5 as ::core::ffi::c_int {
        colfmt[i as usize].align = TEXT_ALIGN::ALIGN_LEFT;
        colfmt[i as usize].basex = (CreditsExtent.width as ::core::ffi::c_int
            / 64 as ::core::ffi::c_int) as COORD;
        i += 1;
    }
    fdef = &raw mut CreditsFont as *mut FONT_SIZE_DEF;
    while (*fdef).size != 0 && size > (*fdef).size {
        fdef = fdef.offset(1);
    }
    if (*fdef).size == 0 {
        return ::core::ptr::null_mut::<FRAME_DESC>();
    }
    t.align = TEXT_ALIGN::ALIGN_LEFT;
    t.baseline.x = 100 as COORD;
    t.baseline.y = 100 as COORD;
    t.pStr = b" \0".as_ptr() as *const ::core::ffi::c_char as *const UNICODE;
    t.CharCount = 1 as COUNT;
    OldContext = SetContext(TempContext);
    SetContextFont((*fdef).font);
    GetContextFontLeading(&raw mut leading);
    TextRect(&raw mut t, &raw mut r, ::core::ptr::null_mut::<BYTE>());
    i = 0 as ::core::ffi::c_int;
    scol = strtok(
        &raw mut salign as *mut ::core::ffi::c_char,
        b",\0".as_ptr() as *const ::core::ffi::c_char,
    );
    while !scol.is_null() && i < 5 as ::core::ffi::c_int {
        let mut c: ::core::ffi::c_char = 0;
        let mut x: ::core::ffi::c_int = 0;
        let mut n: ::core::ffi::c_int = 0;
        colfmt[i as usize].align = TEXT_ALIGN::ALIGN_LEFT;
        colfmt[i as usize].basex = r.extent.width;
        n = sscanf(
            scol,
            b"%c/%d\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut c,
            &raw mut x,
        );
        if n >= 1 as ::core::ffi::c_int {
            match c as ::core::ffi::c_int {
                76 => {
                    colfmt[i as usize].align = TEXT_ALIGN::ALIGN_LEFT;
                    if n >= 2 as ::core::ffi::c_int {
                        colfmt[i as usize].basex = x as COORD;
                    }
                }
                67 => {
                    colfmt[i as usize].align = TEXT_ALIGN::ALIGN_CENTER;
                    if n >= 2 as ::core::ffi::c_int {
                        colfmt[i as usize].basex = x as COORD;
                    } else {
                        colfmt[i as usize].basex = (CreditsExtent.width
                            as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as COORD;
                    }
                }
                82 => {
                    colfmt[i as usize].align = TEXT_ALIGN::ALIGN_RIGHT;
                    if n >= 2 as ::core::ffi::c_int {
                        colfmt[i as usize].basex = x as COORD;
                    } else {
                        colfmt[i as usize].basex = (CreditsExtent.width
                            as ::core::ffi::c_int - r.extent.width as ::core::ffi::c_int)
                            as COORD;
                    }
                }
                _ => {}
            }
        }
        i += 1;
        scol = strtok(
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
            b",\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    i = 0 as ::core::ffi::c_int;
    while i < cnt {
        let mut fmt: *mut col_format_t = (&raw mut colfmt as *mut col_format_t)
            .offset(TextLines[i as usize].baseline.x as ::core::ffi::c_int as isize);
        TextLines[i as usize].align = (*fmt).align;
        TextLines[i as usize].baseline.x = (*fmt).basex;
        TextLines[i as usize].baseline.y = (TextLines[i as usize].baseline.y
            as ::core::ffi::c_int * leading as ::core::ffi::c_int) as COORD;
        i += 1;
    }
    f = Credits_MakeTransFrame(
        CreditsExtent.width as ::core::ffi::c_int,
        leading as ::core::ffi::c_int * rows
            + (leading as ::core::ffi::c_int >> 1 as ::core::ffi::c_int),
        BackColor,
    );
    OldFrame = SetContextFGFrame(f);
    SetContextForeGroundColor(ForeColor);
    i = 0 as ::core::ffi::c_int;
    while i < cnt {
        font_DrawText((&raw mut TextLines as *mut TEXT).offset(i as isize));
        i += 1;
    }
    SetContextFGFrame(OldFrame);
    SetContext(OldContext);
    return f;
}
#[inline]
unsafe extern "C" fn frameIndex(mut index: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return (index + 32 as ::core::ffi::c_int) % 32 as ::core::ffi::c_int;
}
unsafe extern "C" fn RenderCreditsScreen(mut targetContext: CONTEXT) {
    let mut oldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    let mut s: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    let mut i: ::core::ffi::c_int = 0;
    oldContext = SetContext(targetContext);
    s.origin.x = 0 as COORD;
    s.origin.y = 0 as COORD;
    s.frame = CreditsBack;
    DrawStamp(&raw mut s);
    s.origin.y = -curFrameOfs as COORD;
    i = firstFrame;
    while i != lastFrame {
        let mut fr: RECT = RECT {
            corner: POINT { x: 0, y: 0 },
            extent: EXTENT { width: 0, height: 0 },
        };
        s.frame = textFrames[i as usize].frame;
        DrawStamp(&raw mut s);
        GetFrameRect(s.frame, &raw mut fr);
        s.origin.y = (s.origin.y as ::core::ffi::c_int
            + fr.extent.height as ::core::ffi::c_int) as COORD;
        i = frameIndex(i + 1 as ::core::ffi::c_int);
    }
    if OutTakesRunning.0 != 0 {
        SetContextForeGroundColor(colorFromRgb15(0 as BYTE, 0 as BYTE, 0x1f as BYTE));
        DrawFilledRectangle(&raw mut CommWndRect);
    }
    SetContext(oldContext);
}
unsafe extern "C" fn InitCredits() {
    let mut ctxRect: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut oldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    let mut targetFrame: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
    memset(
        &raw mut textFrames as *mut CreditTextFrame as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[CreditTextFrame; 32]>(),
    );
    LocalContext = CreateContextAux(
        b"Credits.LocalContext\0".as_ptr() as *const ::core::ffi::c_char,
    );
    DrawContext = CreateContextAux(
        b"Credits.DrawContext\0".as_ptr() as *const ::core::ffi::c_char,
    );
    targetFrame = GetContextFGFrame();
    GetContextClipRect(&raw mut ctxRect);
    CreditsExtent = ctxRect.extent;
    oldContext = SetContext(LocalContext);
    CreditsFrame = Credits_MakeTransFrame(
        CreditsExtent.width as ::core::ffi::c_int,
        CreditsExtent.height as ::core::ffi::c_int,
        colorFromRgb15(0 as BYTE, 0 as BYTE, 0x1f as BYTE),
    );
    SetContextFGFrame(CreditsFrame);
    textFrames[0usize].frame = Credits_MakeTransFrame(
        1 as ::core::ffi::c_int,
        CreditsExtent.height as ::core::ffi::c_int,
        colorFromRgb15(0 as BYTE, 0 as BYTE, 0x1f as BYTE),
    );
    textFrames[0usize].strIndex = -1 as ::core::ffi::c_int;
    firstFrame = 0 as ::core::ffi::c_int;
    lastFrame = firstFrame + 1 as ::core::ffi::c_int;
    totalHeight = (*textFrames[0usize].frame).Bounds.height as ::core::ffi::c_int;
    curFrameOfs = 0 as ::core::ffi::c_int;
    SetContext(DrawContext);
    SetContextFGFrame(targetFrame);
    SetContext(oldContext);
    RenderCreditsScreen(LocalContext);
    CreditsRate = 9 as ::core::ffi::c_int;
    CreditsRunning = BOOLEAN::TRUE;
}
unsafe extern "C" fn freeCreditTextFrame(mut tf: *mut CreditTextFrame) {
    DestroyDrawable(ReleaseDrawable((*tf).frame));
    (*tf).frame = ::core::ptr::null_mut::<FRAME_DESC>();
}
unsafe extern "C" fn UninitCredits() {
    DestroyContext(DrawContext);
    DrawContext = ::core::ptr::null_mut::<CONTEXT_DESC>();
    DestroyContext(LocalContext);
    LocalContext = ::core::ptr::null_mut::<CONTEXT_DESC>();
    DestroyDrawable(ReleaseDrawable(CreditsFrame));
    CreditsFrame = ::core::ptr::null_mut::<FRAME_DESC>();
    while firstFrame != lastFrame {
        freeCreditTextFrame(
            (&raw mut textFrames as *mut CreditTextFrame).offset(firstFrame as isize),
        );
        firstFrame = frameIndex(firstFrame + 1 as ::core::ffi::c_int);
    }
}
unsafe extern "C" fn calcDeficitHeight() -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut maxPos: ::core::ffi::c_int = 0;
    maxPos = -curFrameOfs;
    i = firstFrame;
    while i != lastFrame {
        let mut fr: RECT = RECT {
            corner: POINT { x: 0, y: 0 },
            extent: EXTENT { width: 0, height: 0 },
        };
        GetFrameRect(textFrames[i as usize].frame, &raw mut fr);
        maxPos += fr.extent.height as ::core::ffi::c_int;
        i = frameIndex(i + 1 as ::core::ffi::c_int);
    }
    return CreditsExtent.height as ::core::ffi::c_int - maxPos;
}
unsafe extern "C" fn processCreditsFrame() {
    static mut NextTime: TimeCount = 0;
    let mut Now: TimeCount = GetTimeCounter();
    if Now >= NextTime {
        let mut fr: RECT = RECT {
            corner: POINT { x: 0, y: 0 },
            extent: EXTENT { width: 0, height: 0 },
        };
        let mut OldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
        let mut rate: ::core::ffi::c_int = 0;
        let mut direction: ::core::ffi::c_int = 0;
        let mut dirstep: ::core::ffi::c_int = 0;
        let mut deficitHeight: ::core::ffi::c_int = 0;
        let mut s: STAMP = STAMP {
            origin: POINT { x: 0, y: 0 },
            frame: ::core::ptr::null_mut::<FRAME_DESC>(),
        };
        rate = abs(CreditsRate);
        if rate != 0 as ::core::ffi::c_int {
            direction = CreditsRate / rate;
            dirstep = (rate + 36 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                / 36 as ::core::ffi::c_int;
            rate = 840 as ::core::ffi::c_int * dirstep / rate;
            dirstep *= direction;
        } else {
            direction = 0 as ::core::ffi::c_int;
            dirstep = 0 as ::core::ffi::c_int;
            rate = 840 as ::core::ffi::c_int;
        }
        NextTime = GetTimeCounter().wrapping_add(rate as TimeCount);
        s.origin.x = 0 as COORD;
        s.origin.y = 0 as COORD;
        s.frame = CreditsFrame;
        OldContext = SetContext(DrawContext);
        DrawStamp(&raw mut s);
        SetContext(OldContext);
        FlushGraphics();
        deficitHeight = calcDeficitHeight();
        curFrameOfs += dirstep;
        if curFrameOfs
            < -(CreditsExtent.height as ::core::ffi::c_int / 20 as ::core::ffi::c_int)
        {
            if CreditsRate < 0 as ::core::ffi::c_int {
                CreditsRate
                    -= CreditsRate / 10 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
            }
        } else if deficitHeight
            > CreditsExtent.height as ::core::ffi::c_int / 25 as ::core::ffi::c_int
        {
            if CreditsRate > 0 as ::core::ffi::c_int {
                CreditsRate
                    -= CreditsRate / 10 as ::core::ffi::c_int + 1 as ::core::ffi::c_int;
            }
            CreditsRunning = BOOLEAN(
                (CreditsRate != 0 as ::core::ffi::c_int) as ::core::ffi::c_int
                    as ::core::ffi::c_uint,
            );
        } else if CreditsRunning.0 == 0 {
            CreditsRunning = BOOLEAN::TRUE;
        }
        if firstFrame != lastFrame {
            if direction > 0 as ::core::ffi::c_int {
                GetFrameRect(textFrames[firstFrame as usize].frame, &raw mut fr);
                if curFrameOfs >= fr.extent.height as ::core::ffi::c_int {
                    totalHeight -= fr.extent.height as ::core::ffi::c_int;
                    freeCreditTextFrame(
                        (&raw mut textFrames as *mut CreditTextFrame)
                            .offset(firstFrame as isize),
                    );
                    firstFrame = frameIndex(firstFrame + 1 as ::core::ffi::c_int);
                    curFrameOfs -= fr.extent.height as ::core::ffi::c_int;
                }
            } else if direction < 0 as ::core::ffi::c_int {
                let mut index: ::core::ffi::c_int = frameIndex(
                    lastFrame - 1 as ::core::ffi::c_int,
                );
                let mut framePos: ::core::ffi::c_int = 0;
                GetFrameRect(textFrames[index as usize].frame, &raw mut fr);
                framePos = totalHeight - curFrameOfs
                    - fr.extent.height as ::core::ffi::c_int;
                if framePos >= CreditsExtent.height as ::core::ffi::c_int {
                    lastFrame = index;
                    totalHeight -= fr.extent.height as ::core::ffi::c_int;
                    freeCreditTextFrame(
                        (&raw mut textFrames as *mut CreditTextFrame)
                            .offset(lastFrame as isize),
                    );
                }
            }
        }
        if direction > 0 as ::core::ffi::c_int {
            let mut next_s: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            if firstFrame != lastFrame {
                next_s = textFrames[frameIndex(lastFrame - 1 as ::core::ffi::c_int)
                        as usize]
                    .strIndex + 1 as ::core::ffi::c_int;
            }
            while totalHeight - curFrameOfs < CreditsExtent.height as ::core::ffi::c_int
                && next_s < GetStringTableCount(CreditsTab) as ::core::ffi::c_int
            {
                let mut tf: *mut CreditTextFrame = (&raw mut textFrames
                    as *mut CreditTextFrame)
                    .offset(lastFrame as isize);
                (*tf).frame = Credits_RenderTextFrame(
                    LocalContext,
                    &raw mut next_s,
                    direction,
                    colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE),
                    colorFromRgb15(0x1f as BYTE, 0x1f as BYTE, 0x1f as BYTE),
                );
                (*tf).strIndex = next_s - 1 as ::core::ffi::c_int;
                if !(*tf).frame.is_null() {
                    GetFrameRect((*tf).frame, &raw mut fr);
                    totalHeight += fr.extent.height as ::core::ffi::c_int;
                    lastFrame = frameIndex(lastFrame + 1 as ::core::ffi::c_int);
                }
            }
        } else if direction < 0 as ::core::ffi::c_int {
            let mut next_s_0: ::core::ffi::c_int = GetStringTableCount(CreditsTab)
                as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
            if firstFrame != lastFrame {
                next_s_0 = textFrames[firstFrame as usize].strIndex
                    - 1 as ::core::ffi::c_int;
            }
            while curFrameOfs < 0 as ::core::ffi::c_int
                && next_s_0 >= 0 as ::core::ffi::c_int
            {
                let mut index_0: ::core::ffi::c_int = frameIndex(
                    firstFrame - 1 as ::core::ffi::c_int,
                );
                let mut tf_0: *mut CreditTextFrame = (&raw mut textFrames
                    as *mut CreditTextFrame)
                    .offset(index_0 as isize);
                (*tf_0).frame = Credits_RenderTextFrame(
                    LocalContext,
                    &raw mut next_s_0,
                    direction,
                    colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE),
                    colorFromRgb15(0x1f as BYTE, 0x1f as BYTE, 0x1f as BYTE),
                );
                (*tf_0).strIndex = next_s_0 + 1 as ::core::ffi::c_int;
                if !(*tf_0).frame.is_null() {
                    GetFrameRect((*tf_0).frame, &raw mut fr);
                    totalHeight += fr.extent.height as ::core::ffi::c_int;
                    firstFrame = index_0;
                    curFrameOfs += fr.extent.height as ::core::ffi::c_int;
                }
            }
        }
        RenderCreditsScreen(LocalContext);
    }
}
unsafe extern "C" fn LoadCredits() -> BOOLEAN {
    let mut fdef: *mut FONT_SIZE_DEF = ::core::ptr::null_mut::<FONT_SIZE_DEF>();
    CreditsTab = CaptureStringTable(
        LoadStringTableInstance(b"credits.credits\0".as_ptr() as RESOURCE),
    );
    if CreditsTab.is_null() {
        return BOOLEAN::FALSE;
    }
    CreditsBack = CaptureDrawable(
        LoadGraphicInstance(b"credits.background\0".as_ptr() as RESOURCE) as DRAWABLE,
    );
    fdef = &raw mut CreditsFont as *mut FONT_SIZE_DEF;
    while (*fdef).size != 0 {
        (*fdef).font = LoadGraphicInstance((*fdef).res) as FONT;
        fdef = fdef.offset(1);
    }
    return BOOLEAN::TRUE;
}
unsafe extern "C" fn FreeCredits() {
    let mut fdef: *mut FONT_SIZE_DEF = ::core::ptr::null_mut::<FONT_SIZE_DEF>();
    DestroyStringTable(ReleaseStringTable(CreditsTab));
    CreditsTab = ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>();
    DestroyDrawable(ReleaseDrawable(CreditsBack));
    CreditsBack = ::core::ptr::null_mut::<FRAME_DESC>();
    fdef = &raw mut CreditsFont as *mut FONT_SIZE_DEF;
    while (*fdef).size != 0 {
        DestroyFont((*fdef).font);
        (*fdef).font = ::core::ptr::null_mut::<FONT_DESC>();
        fdef = fdef.offset(1);
    }
}
unsafe extern "C" fn OutTakes() {
    static mut outtake_list: [CONVERSATION; 15] = [
        CONVERSATION::ZOQFOTPIK_CONVERSATION,
        CONVERSATION::TALKING_PET_CONVERSATION,
        CONVERSATION::ORZ_CONVERSATION,
        CONVERSATION::UTWIG_CONVERSATION,
        CONVERSATION::THRADD_CONVERSATION,
        CONVERSATION::SUPOX_CONVERSATION,
        CONVERSATION::SYREEN_CONVERSATION,
        CONVERSATION::SHOFIXTI_CONVERSATION,
        CONVERSATION::PKUNK_CONVERSATION,
        CONVERSATION::YEHAT_CONVERSATION,
        CONVERSATION::DRUUGE_CONVERSATION,
        CONVERSATION::URQUAN_CONVERSATION,
        CONVERSATION::VUX_CONVERSATION,
        CONVERSATION::BLACKURQ_CONVERSATION,
        CONVERSATION::ARILOU_CONVERSATION,
    ];
    let mut oldsubtitles: BOOLEAN = optSubtitles;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    optSubtitles = BOOLEAN::TRUE;
    sliderDisabled = BOOLEAN::TRUE;
    oscillDisabled = BOOLEAN::TRUE;
    i = 0 as ::core::ffi::c_int;
    while i < 15 as ::core::ffi::c_int
        && GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
            & C2Rust_Unnamed_0::CHECK_ABORT.0 as ::core::ffi::c_int == 0
    {
        SetCommIntroMode(CommIntroMode::CIM_CROSSFADE_WINDOW, 0 as TimeCount);
        InitCommunication(outtake_list[i as usize]);
        i += 1;
    }
    optSubtitles = oldsubtitles;
    sliderDisabled = BOOLEAN::FALSE;
    oscillDisabled = BOOLEAN::FALSE;
}
unsafe extern "C" fn DoCreditsInput(mut pIS: *mut ::core::ffi::c_void) -> BOOLEAN {
    let mut pCIS: *mut CREDITS_INPUT_STATE = pIS as *mut CREDITS_INPUT_STATE;
    if CreditsRunning.0 != 0 {
        (*pCIS).CloseTimeOut = 0 as DWORD;
    }
    if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
        & C2Rust_Unnamed_0::CHECK_ABORT.0 as ::core::ffi::c_int != 0
        || (*pCIS).AllowCancel.0 != 0
            && (PulsedInputState
                .menu[C2Rust_Unnamed::KEY_MENU_SELECT.0 as ::core::ffi::c_int as usize]
                != 0
                || PulsedInputState
                    .menu[C2Rust_Unnamed::KEY_MENU_CANCEL.0 as ::core::ffi::c_int
                    as usize] != 0)
    {
        return BOOLEAN::FALSE;
    }
    if (*pCIS).AllowSpeedChange.0 != 0
        && (PulsedInputState
            .menu[C2Rust_Unnamed::KEY_MENU_UP.0 as ::core::ffi::c_int as usize] != 0
            || PulsedInputState
                .menu[C2Rust_Unnamed::KEY_MENU_DOWN.0 as ::core::ffi::c_int as usize]
                != 0)
    {
        let mut newrate: ::core::ffi::c_int = CreditsRate;
        let mut step: ::core::ffi::c_int = abs(CreditsRate) / 5 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int;
        if PulsedInputState
            .menu[C2Rust_Unnamed::KEY_MENU_DOWN.0 as ::core::ffi::c_int as usize] != 0
        {
            newrate += step;
        } else if PulsedInputState
            .menu[C2Rust_Unnamed::KEY_MENU_UP.0 as ::core::ffi::c_int as usize] != 0
        {
            newrate -= step;
        }
        if newrate < -130 as ::core::ffi::c_int {
            newrate = -130 as ::core::ffi::c_int;
        } else if newrate > 130 as ::core::ffi::c_int {
            newrate = 130 as ::core::ffi::c_int;
        }
        CreditsRate = newrate;
    }
    if CreditsRunning.0 == 0 {
        (*pCIS).AllowCancel = BOOLEAN::TRUE;
    }
    if CreditsRunning.0 == 0 && (*pCIS).CloseWhenDone.0 != 0 {
        if (*pCIS).CloseTimeOut == 0 as DWORD {
            (*pCIS).CloseTimeOut = GetTimeCounter()
                .wrapping_add(
                    (840 as ::core::ffi::c_int * 5 as ::core::ffi::c_int) as TimeCount,
                ) as DWORD;
        } else if GetTimeCounter() > (*pCIS).CloseTimeOut {
            return BOOLEAN::FALSE
        }
    }
    if CreditsRunning.0 == 0
        && (PulsedInputState
            .menu[C2Rust_Unnamed::KEY_MENU_SELECT.0 as ::core::ffi::c_int as usize] != 0
            || PulsedInputState
                .menu[C2Rust_Unnamed::KEY_MENU_CANCEL.0 as ::core::ffi::c_int as usize]
                != 0)
    {
        return BOOLEAN::FALSE;
    }
    SleepThread((840 as ::core::ffi::c_int / 36 as ::core::ffi::c_int) as TimePeriod);
    return BOOLEAN::TRUE;
}
unsafe extern "C" fn on_input_frame() {
    processCreditsFrame();
}
#[no_mangle]
pub unsafe extern "C" fn Credits(mut WithOuttakes: BOOLEAN) {
    let mut hMusic: MUSIC_REF = ::core::ptr::null_mut::<*mut TFB_SoundSample>();
    let mut cis: CREDITS_INPUT_STATE = CREDITS_INPUT_STATE {
        InputFunc: None,
        AllowCancel: BOOLEAN::FALSE,
        AllowSpeedChange: BOOLEAN::FALSE,
        CloseWhenDone: BOOLEAN::FALSE,
        CloseTimeOut: 0,
    };
    let mut screenRect: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut s: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    hMusic = LoadMusicInstance(b"music.credits\0".as_ptr() as RESOURCE);
    SetContext(ScreenContext);
    SetContextClipRect(::core::ptr::null_mut::<RECT>());
    GetContextClipRect(&raw mut screenRect);
    SetContextBackGroundColor(colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE));
    ClearDrawable();
    if LoadCredits().0 == 0 {
        return;
    }
    s.origin.x = 0 as COORD;
    s.origin.y = 0 as COORD;
    s.frame = CreditsBack;
    DrawStamp(&raw mut s);
    FadeScreen(
        ScreenFadeType::FadeAllToColor,
        (840 as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as SIZE,
    );
    CommWndRect.corner.x = ((screenRect.extent.width as ::core::ffi::c_int
        - CommWndRect.extent.width as ::core::ffi::c_int) / 2 as ::core::ffi::c_int)
        as COORD;
    CommWndRect.corner.y = 5 as COORD;
    InitCredits();
    SetInputCallback(Some(on_input_frame as unsafe extern "C" fn() -> ()));
    if WithOuttakes.0 != 0 {
        OutTakesRunning = BOOLEAN::TRUE;
        OutTakes();
        OutTakesRunning = BOOLEAN::FALSE;
    }
    if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
        & C2Rust_Unnamed_0::CHECK_ABORT.0 as ::core::ffi::c_int == 0
    {
        if !hMusic.is_null() {
            PlayMusic(hMusic, BOOLEAN::TRUE, 1 as BYTE);
        }
        cis.InputFunc = Some(
            DoCreditsInput as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> BOOLEAN,
        ) as Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> BOOLEAN>;
        cis.AllowCancel = BOOLEAN(
            (WithOuttakes.0 == 0) as ::core::ffi::c_int as ::core::ffi::c_uint,
        );
        cis.CloseWhenDone = BOOLEAN(
            (WithOuttakes.0 == 0) as ::core::ffi::c_int as ::core::ffi::c_uint,
        );
        cis.AllowSpeedChange = BOOLEAN::TRUE;
        SetMenuSounds(
            0 as ::core::ffi::c_int as MENU_SOUND_FLAGS,
            0 as ::core::ffi::c_int as MENU_SOUND_FLAGS,
        );
        DoInput(&raw mut cis as *mut ::core::ffi::c_void, BOOLEAN::TRUE);
    }
    SetInputCallback(None);
    FadeMusic(0 as BYTE, (840 as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as SIZE);
    UninitCredits();
    SetContext(ScreenContext);
    SleepThreadUntil(
        FadeScreen(
            ScreenFadeType::FadeAllToBlack,
            (840 as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as SIZE,
        ),
    );
    FlushColorXForms();
    if !hMusic.is_null() {
        StopMusic();
        DestroyMusic(hMusic);
    }
    FadeMusic(160 as BYTE, 0 as SIZE);
    FreeCredits();
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
