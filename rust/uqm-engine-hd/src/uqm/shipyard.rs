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

// Rust translation of src/uqm/shipyard.c.
extern "C" {
    pub type string_table_entry;
    pub type string_table;
    pub type context_desc;
    pub type font_desc;
    pub type tfb_soundsample;
    pub type FlashContext;
    fn SetContext(Context: CONTEXT) -> CONTEXT;
    fn SetContextForeGroundColor(Color: Color) -> Color;
    fn SetContextBackGroundColor(Color: Color) -> Color;
    fn SetContextClipRect(pRect: *mut RECT) -> BOOLEAN;
    fn GetContextClipRect(pRect: *mut RECT) -> BOOLEAN;
    fn DrawStamp(pStamp: *mut STAMP);
    fn DrawFilledStamp(pStamp: *mut STAMP);
    fn DrawFilledRectangle(pRect: *mut RECT);
    fn font_GetTextRect(pText: *mut TEXT) -> RECT;
    fn font_DrawText(pText: *mut TEXT);
    fn font_DrawShadowedText(
        pText: *mut TEXT,
        direction: BYTE,
        text_color: Color,
        shadow_color: Color,
    );
    fn BatchGraphics();
    fn UnbatchGraphics();
    fn ClearDrawable();
    fn DestroyDrawable(Drawable: DRAWABLE) -> BOOLEAN;
    fn LoadGraphicInstance(res: RESOURCE) -> *mut ::core::ffi::c_void;
    fn SetContextFont(Font: FONT) -> FONT;
    fn DestroyFont(FontRef: FONT) -> BOOLEAN;
    fn GetContextFontLeading(pheight: *mut SIZE) -> BOOLEAN;
    fn SetAbsFrameIndex(Frame: FRAME, FrameIndex: COUNT) -> FRAME;
    fn IncFrameIndex(Frame: FRAME) -> FRAME;
    fn DecFrameIndex(Frame: FRAME) -> FRAME;
    fn CaptureDrawable(Drawable: DRAWABLE) -> FRAME;
    fn ReleaseDrawable(Frame: FRAME) -> DRAWABLE;
    fn SetColorMap(ColorMapPtr: COLORMAPPTR) -> BOOLEAN;
    fn GetColorMapAddress(_: COLORMAP) -> COLORMAPPTR;
    fn __assert_fail(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    ) -> !;
    fn sprintf(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn snprintf(
        _: *mut ::core::ffi::c_char,
        _: size_t,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn LoadStringTableInstance(res: RESOURCE) -> STRING_TABLE;
    fn DestroyStringTable(StringTable: STRING_TABLE) -> BOOLEAN;
    fn CaptureStringTable(StringTable: STRING_TABLE) -> STRING;
    fn ReleaseStringTable(String: STRING) -> STRING_TABLE;
    fn GetStringTableCount(String: STRING) -> COUNT;
    fn SetAbsStringTableIndex(String: STRING, StringTableIndex: COUNT) -> STRING;
    fn SetRelStringTableIndex(String: STRING, StringTableOffs: SIZE) -> STRING;
    fn GetStringLength(String: STRING) -> COUNT;
    fn GetStringAddress(String: STRING) -> STRINGPTR;
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
    fn SleepThread(timePeriod: TimePeriod);
    fn SleepThreadUntil(wakeTime: TimeCount);
    fn GetTimeCounter() -> TimeCount;
    fn memset(
        _: *mut ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strlen(_: *const ::core::ffi::c_char) -> size_t;
    static mut PulsedInputState: CONTROLLER_INPUT_STATE;
    fn FlushInput();
    fn SetInputCallback(_: Option<InputFrameCallback>) -> Option<InputFrameCallback>;
    static mut optWhichMenu: ::core::ffi::c_int;
    static mut optWhichFonts: ::core::ffi::c_int;
    static mut optWhichIntro: ::core::ffi::c_int;
    static mut resolutionFactor: ::core::ffi::c_uint;
    static mut optInfiniteRU: OPT_ENABLABLE;
    static mut optCustomBorder: OPT_ENABLABLE;
    static mut optScrTrans: ::core::ffi::c_int;
    static mut optWindowType: DWORD;
    static mut optFleetPointSys: OPT_ENABLABLE;
    static mut optShipStore: OPT_ENABLABLE;
    static mut optCaptainNames: OPT_ENABLABLE;
    static mut optDosMenus: OPT_ENABLABLE;
    static mut CanvasWidth: ::core::ffi::c_int;
    static mut CanvasHeight: ::core::ffi::c_int;
    fn FreeLink(pq: *mut QUEUE, hLink: HLINK);
    fn InsertQueue(pq: *mut QUEUE, hLink: HLINK, hRefLink: HLINK);
    fn RemoveQueue(pq: *mut QUEUE, hLink: HLINK);
    fn CountLinks(pq: *mut QUEUE) -> COUNT;
    fn LoadMusicInstance(res: RESOURCE) -> MUSIC_REF;
    fn SetMusicPosition();
    fn DrawSISFrame();
    fn ClearSISRect(ClearFlags: BYTE);
    fn SetFlashRect(pRect: *const RECT, purple: BOOLEAN);
    fn PreUpdateFlashRect();
    fn PostUpdateFlashRect();
    fn DrawSISTitle(pStr: *mut UNICODE);
    fn DrawSISMessage(pStr: *const UNICODE);
    fn GetGaugeRect(pRect: *mut RECT, IsCrewRect: BOOLEAN);
    fn DeltaSISGauges(
        crew_delta: SIZE,
        fuel_delta: SDWORD,
        resunit_delta: ::core::ffi::c_int,
    );
    fn GetCrewCount() -> COUNT;
    fn GetCPodCapacity(ppt: *mut POINT) -> COUNT;
    fn GetSBayCapacity(ppt: *mut POINT) -> COUNT;
    fn DrawFuelInFTanks(isOutfit: BOOLEAN);
    fn setGameStateUint(name: *const ::core::ffi::c_char, val: DWORD);
    fn getGameStateUint(name: *const ::core::ffi::c_char) -> DWORD;
    static mut GlobData: GLOBDATA;
    fn isPC(optWhich: ::core::ffi::c_int) -> OPT_CONSOLETYPE;
    fn CloneShipFragment(
        shipIndex: RACE_ID,
        pDstQueue: *mut QUEUE,
        crew_level: COUNT,
    ) -> HSHIPFRAG;
    fn GetStarShipFromIndex(pShipQ: *mut QUEUE, Index: COUNT) -> HLINK;
    fn GetEscortByStarShipIndex(index: COUNT) -> HSHIPFRAG;
    fn GetIndexFromStarShip(pShipQ: *mut QUEUE, hStarShip: HLINK) -> COUNT;
    fn CheckAlliance(race: RACE_ID) -> COUNT;
    static mut GameStrings: STRING;
    fn GameOptions() -> BOOLEAN;
    static mut OutfitOrShipyard: BYTE;
    fn DoShipSpin(index: COUNT, hMusic: MUSIC_REF);
    fn DoMenuChooser(pMS: *mut MENU_STATE, BaseState: BYTE) -> BOOLEAN;
    fn DrawMenuStateStrings(beg_index: BYTE, NewState: SWORD);
    fn DrawBorder(Visible: BYTE);
    fn DrawShipPiece(
        ModuleFrame: FRAME,
        which_piece: COUNT,
        which_slot: COUNT,
        DrawBluePrint_0: BOOLEAN,
    );
    static mut ScreenContext: CONTEXT;
    static mut SpaceContext: CONTEXT;
    static mut StatusContext: CONTEXT;
    static mut StarConFont: FONT;
    static mut TinyFont: FONT;
    static mut TinyFontBold: FONT;
    static mut classicPackPresent: BOOLEAN;
    fn PlayMusicResume(hMusic: MUSIC_REF, Volume: BYTE);
    fn SetMenuSounds(sound_0: MENU_SOUND_FLAGS, sound_1: MENU_SOUND_FLAGS);
    fn PlayMenuSound(S: MENU_SOUND_EFFECT);
    static mut master_q: QUEUE;
    fn FindMasterShip(ship_ref: SPECIES_ID) -> HMASTERSHIP;
    fn FindMasterShipIndex(ship_ref: SPECIES_ID) -> ::core::ffi::c_int;
    fn GetShipCostFromIndex(Index: ::core::ffi::c_uint) -> COUNT;
    fn AnyButtonPress(DetectSpecial: BOOLEAN) -> BOOLEAN;
    fn SetTransitionSource(pRect: *const RECT);
    fn ScreenTransition(transition: ::core::ffi::c_int, pRect: *const RECT);
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
    fn DrawRenderedBox(
        r: *mut RECT,
        filled: BOOLEAN,
        fill_color: Color,
        r#type: ::core::ffi::c_int,
        custom: ::core::ffi::c_int,
    );
    static mut ModuleFont: FONT;
    fn abs(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
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
pub type size_t = usize;
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
    pub const NORTH_SHADOW: Self = Self(0);
    pub const NORTH_EAST_SHADOW: Self = Self(1);
    pub const EAST_SHADOW: Self = Self(2);
    pub const SOUTH_EAST_SHADOW: Self = Self(3);
    pub const SOUTH_SHADOW: Self = Self(4);
    pub const SOUTH_WEST_SHADOW: Self = Self(5);
    pub const WEST_SHADOW: Self = Self(6);
    pub const NORTH_WEST_SHADOW: Self = Self(7);
}
pub type TimeCount = DWORD;
pub type TimePeriod = DWORD;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_0 {
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
pub type InputFrameCallback = unsafe extern "C" fn() -> ();
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
pub type MUSIC_REF = *mut *mut TFB_SoundSample;
pub type TFB_SoundSample = tfb_soundsample;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FLEET_STUFF {
    pub strength: COUNT,
    pub known_loc: POINT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SHIP_INFO {
    pub idStr: *const ::core::ffi::c_char,
    pub ship_flags: UWORD,
    pub ship_cost: BYTE,
    pub crew_level: COUNT,
    pub max_crew: COUNT,
    pub energy_level: BYTE,
    pub max_energy: BYTE,
    pub race_strings_rsc: RESOURCE,
    pub icons_rsc: RESOURCE,
    pub melee_icon_rsc: RESOURCE,
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
pub type HFLEETINFO = HLINK;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FLEET_INFO {
    pub pred: HFLEETINFO,
    pub succ: HFLEETINFO,
    pub SpeciesID: SPECIES_ID,
    pub shipIdStr: *const ::core::ffi::c_char,
    pub allied_state: UWORD,
    pub days_left: BYTE,
    pub growth_fract: BYTE,
    pub crew_level: COUNT,
    pub max_crew: COUNT,
    pub growth: BYTE,
    pub max_energy: BYTE,
    pub loc: POINT,
    pub race_strings: STRING,
    pub icons: FRAME,
    pub melee_icon: FRAME,
    pub actual_strength: COUNT,
    pub known_strength: COUNT,
    pub known_loc: POINT,
    pub growth_err_term: BYTE,
    pub func_index: BYTE,
    pub dest_loc: POINT,
    pub can_build: BOOLEAN,
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
pub struct C2Rust_Unnamed_2(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_2 {
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_3(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_3 {
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
pub struct C2Rust_Unnamed_4(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_4 {
    pub const DOS_MENU_HILITE: Self = Self(0);
    pub const SPECIAL_BEVEL: Self = Self(1);
    pub const THIN_INNER_BEVEL: Self = Self(9);
    pub const THICK_INNER_BEVEL: Self = Self(17);
    pub const THIN_OUTER_BEVEL: Self = Self(25);
    pub const THICK_OUTER_BEVEL: Self = Self(33);
    pub const THICK_OUT_INV_BEVEL: Self = Self(41);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_5(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_5 {
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
pub type MENU_STATE = menu_state;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_6(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_6 {
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
pub struct C2Rust_Unnamed_7(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_7 {
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
pub struct MeleeShip(pub ::core::ffi::c_uint);
impl MeleeShip {
    pub const MELEE_ANDROSYNTH: Self = Self(0);
    pub const MELEE_ARILOU: Self = Self(1);
    pub const MELEE_CHENJESU: Self = Self(2);
    pub const MELEE_CHMMR: Self = Self(3);
    pub const MELEE_DRUUGE: Self = Self(4);
    pub const MELEE_EARTHLING: Self = Self(5);
    pub const MELEE_ILWRATH: Self = Self(6);
    pub const MELEE_KOHR_AH: Self = Self(7);
    pub const MELEE_MELNORME: Self = Self(8);
    pub const MELEE_MMRNMHRM: Self = Self(9);
    pub const MELEE_MYCON: Self = Self(10);
    pub const MELEE_ORZ: Self = Self(11);
    pub const MELEE_PKUNK: Self = Self(12);
    pub const MELEE_SHOFIXTI: Self = Self(13);
    pub const MELEE_SLYLANDRO: Self = Self(14);
    pub const MELEE_SPATHI: Self = Self(15);
    pub const MELEE_SUPOX: Self = Self(16);
    pub const MELEE_SYREEN: Self = Self(17);
    pub const MELEE_THRADDASH: Self = Self(18);
    pub const MELEE_UMGAH: Self = Self(19);
    pub const MELEE_URQUAN: Self = Self(20);
    pub const MELEE_UTWIG: Self = Self(21);
    pub const MELEE_VUX: Self = Self(22);
    pub const MELEE_YEHAT: Self = Self(23);
    pub const MELEE_ZOQFOTPIK: Self = Self(24);
    pub const MELEE_UNSET: Self = Self(254);
    pub const MELEE_NONE: Self = Self(255);
}
pub type HMASTERSHIP = HLINK;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MASTER_SHIP_INFO {
    pub pred: HMASTERSHIP,
    pub succ: HMASTERSHIP,
    pub SpeciesID: SPECIES_ID,
    pub ShipInfo: SHIP_INFO,
    pub Fleet: FLEET_STUFF,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_8(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_8 {
    pub const TALK_COMMANDER: Self = Self(0);
    pub const OUTFIT_STARSHIP: Self = Self(1);
    pub const SHIPYARD: Self = Self(2);
    pub const DEPART_BASE: Self = Self(3);
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct DMS_Mode(pub ::core::ffi::c_uint);
impl DMS_Mode {
    pub const DMS_Mode_navigate: Self = Self(0);
    pub const DMS_Mode_addEscort: Self = Self(1);
    pub const DMS_Mode_editCrew: Self = Self(2);
    pub const DMS_Mode_exit: Self = Self(3);
}
pub type MENU_SOUND_FLAGS = UWORD;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SHIPS_STATE {
    pub count: COUNT,
    pub topIndex: COUNT,
    pub ShipStats: [SHIP_STATS; 225],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SHIP_STATS {
    pub ShipName: [UNICODE; 30],
    pub ShipCost: COUNT,
    pub CrewLevel: BYTE,
    pub MaxCrew: BYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_9 {
    pub StarShipPtr: *mut SHIP_FRAGMENT,
    pub finished_s: POINT,
    pub ship_s: STAMP,
    pub lfdoor_s: STAMP,
    pub rtdoor_s: STAMP,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_10(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_10 {
    pub const SHIPYARD_CREW: Self = Self(0);
    pub const SHIPYARD_SAVELOAD: Self = Self(1);
    pub const SHIPYARD_EXIT: Self = Self(2);
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
#[inline]
unsafe extern "C" fn LockFleetInfo(
    mut pq: *const QUEUE,
    mut h: HFLEETINFO,
) -> *mut FLEET_INFO {
    ((*pq).object_size as usize == ::core::mem::size_of::<FLEET_INFO>()
        || {
            __assert_fail(
                b"GetLinkSize (pq) == sizeof (FLEET_INFO)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/uqm/races.h\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                410 as ::core::ffi::c_int,
                b"LockFleetInfo\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    return LockLink(pq, h) as *mut FLEET_INFO;
}
#[no_mangle]
pub static mut hangar_x_coords_3do: [COORD; 4] = [
    19 as COORD,
    60 as COORD,
    116 as COORD,
    157 as COORD,
];
#[no_mangle]
pub static mut hangar_x_coords_uqm: [COORD; 6] = [
    0 as COORD,
    38 as COORD,
    76 as COORD,
    131 as COORD,
    169 as COORD,
    207 as COORD,
];
#[no_mangle]
pub static mut hangar_x_coords_dos: [COORD; 6] = [
    0 as COORD,
    38 as COORD,
    76 as COORD,
    133 as COORD,
    171 as COORD,
    209 as COORD,
];
#[no_mangle]
pub static mut hangar_x_coords: [COORD; 6] = [0; 6];
static mut DoShipSpins: BOOLEAN = BOOLEAN::FALSE;
static mut stowed_q: BOOLEAN = BOOLEAN::FALSE;
static mut currentStowShip: BYTE = 0 as BYTE;
static mut stowCount: COUNT = 0 as COUNT;
static mut availableCount: COUNT = 0 as COUNT;
#[no_mangle]
pub static mut ShipState: SHIPS_STATE = SHIPS_STATE {
    count: 0,
    topIndex: 0,
    ShipStats: [SHIP_STATS {
        ShipName: [0; 30],
        ShipCost: 0,
        CrewLevel: 0,
        MaxCrew: 0,
    }; 225],
};
unsafe extern "C" fn GetStowedShipCount() -> COUNT {
    let mut Index: COUNT = 0;
    let mut hStarShip: HSHIPFRAG = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut hNextShip: HSHIPFRAG = ::core::ptr::null_mut::<::core::ffi::c_void>();
    Index = 0 as COUNT;
    hStarShip = GlobData.Game_state.stowed_ship_q.head as HSHIPFRAG;
    while !hStarShip.is_null() {
        let mut StarShipPtr: *mut SHIP_FRAGMENT = ::core::ptr::null_mut::<
            SHIP_FRAGMENT,
        >();
        StarShipPtr = LockShipFrag(
            &raw mut GlobData.Game_state.stowed_ship_q,
            hStarShip,
        );
        Index = Index.wrapping_add(1);
        hNextShip = (*StarShipPtr).succ as HSHIPFRAG;
        UnlockLink(&raw mut GlobData.Game_state.stowed_ship_q, hStarShip);
        hStarShip = hNextShip;
    }
    return Index;
}
unsafe extern "C" fn DrawShipsStatus(
    mut index: COUNT,
    mut pos: COUNT,
    mut selected: bool,
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
    let mut buf: [UNICODE; 10] = [0; 10];
    if stowed_q.0 != 0 {
        index = (index as ::core::ffi::c_int
            + RACE_ID::NUM_BUILDABLE_SHIPS.0 as ::core::ffi::c_int) as COUNT;
    }
    t.align = TEXT_ALIGN::ALIGN_LEFT;
    t.baseline.x = ((5 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    r.extent.width = (((64 as ::core::ffi::c_int) << resolutionFactor)
        - ((5 as ::core::ffi::c_int) << resolutionFactor)
        - ((3 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    r.extent.height = (((7 as ::core::ffi::c_int) << resolutionFactor)
        * 2 as ::core::ffi::c_int) as COORD;
    r.corner.x = (((5 as ::core::ffi::c_int) << resolutionFactor)
        - ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    r.corner.y = (((36 as ::core::ffi::c_int) << resolutionFactor)
        + pos as ::core::ffi::c_int * ((16 as ::core::ffi::c_int) << resolutionFactor)
        + ((2 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    SetContextForeGroundColor(
        if selected as ::core::ffi::c_int != 0 {
            buildColorRgba(0x55 as BYTE, 0x55 as BYTE, 0xff as BYTE, 0xff as BYTE)
        } else {
            buildColorRgba(0 as BYTE, 0 as BYTE, 0xaa as BYTE, 0xff as BYTE)
        },
    );
    uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
    SetContextFont(TinyFont);
    SetContextForeGroundColor(
        if selected as ::core::ffi::c_int != 0 {
            buildColorRgba(0x55 as BYTE, 0xff as BYTE, 0xff as BYTE, 0xff as BYTE)
        } else if stowed_q.0 != 0 {
            colorFromRgb15(0x16 as BYTE, 0xb as BYTE, 0x1f as BYTE)
        } else {
            buildColorRgba(0 as BYTE, 0xaa as BYTE, 0xaa as BYTE, 0xff as BYTE)
        },
    );
    t.baseline.y = (r.corner.y as ::core::ffi::c_int
        + ((6 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    t.pStr = &raw mut (*(&raw mut ShipState.ShipStats as *mut SHIP_STATS)
        .offset(index as isize))
        .ShipName as *mut UNICODE;
    t.CharCount = (if stowed_q.0 != 0 {
        !(0 as ::core::ffi::c_int) as COUNT as ::core::ffi::c_int
    } else {
        utf8StringPos(t.pStr, ' ' as UniChar)
    }) as COUNT;
    uqm_lua_checked!(font_DrawText(&raw mut t), ());
    if stowed_q.0 == 0 {
        t.baseline.y = (t.baseline.y as ::core::ffi::c_int
            + ((7 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        t.pStr = skipUTF8Chars(
            t.pStr,
            (t.CharCount as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as size_t,
        );
        t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
        uqm_lua_checked!(font_DrawText(&raw mut t), ());
    }
    SetContextForeGroundColor(
        if selected as ::core::ffi::c_int != 0 {
            buildColorRgba(0x55 as BYTE, 0xff as BYTE, 0xff as BYTE, 0xff as BYTE)
        } else {
            buildColorRgba(0x55 as BYTE, 0x55 as BYTE, 0xff as BYTE, 0xff as BYTE)
        },
    );
    t.align = TEXT_ALIGN::ALIGN_RIGHT;
    t.baseline.x = (((61 as ::core::ffi::c_int) << resolutionFactor)
        - ((2 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    if stowed_q.0 == 0 {
        t.baseline.y = (t.baseline.y as ::core::ffi::c_int
            - ((3 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    }
    snprintf(
        &raw mut buf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[UNICODE; 10]>(),
        b"%u\0".as_ptr() as *const ::core::ffi::c_char,
        ShipState.ShipStats[index as usize].ShipCost as ::core::ffi::c_int,
    );
    t.pStr = &raw mut buf as *mut UNICODE;
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    uqm_lua_checked!(font_DrawText(&raw mut t), ());
    if stowed_q.0 != 0 {
        t.baseline.y = (t.baseline.y as ::core::ffi::c_int
            + ((7 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        snprintf(
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[UNICODE; 10]>(),
            b"%u/%u\0".as_ptr() as *const ::core::ffi::c_char,
            ShipState.ShipStats[index as usize].CrewLevel as ::core::ffi::c_int,
            ShipState.ShipStats[index as usize].MaxCrew as ::core::ffi::c_int,
        );
        t.pStr = &raw mut buf as *mut UNICODE;
        t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
        uqm_lua_checked!(font_DrawText(&raw mut t), ());
    }
}
unsafe extern "C" fn DrawShipsDisplay(mut shipState: *mut SHIPS_STATE) {
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
    let mut i: COUNT = 0;
    r.corner.x = ((2 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    r.corner.y = ((20 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    r.extent.width = (((64 as ::core::ffi::c_int) << resolutionFactor)
        - ((5 as ::core::ffi::c_int) << resolutionFactor)
        + ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    r.extent.height = (((129 as ::core::ffi::c_int) << resolutionFactor)
        - r.corner.y as ::core::ffi::c_int) as COORD;
    if optCustomBorder.0 == 0
        && if resolutionFactor != 2 as ::core::ffi::c_uint {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } == 0
    {
        uqm_lua_checked!(DrawStarConBox(
            &raw mut r,
            ((1 as ::core::ffi::c_int) << resolutionFactor) as SIZE,
            colorFromRgb15(0x10 as BYTE, 0x10 as BYTE, 0x10 as BYTE),
            colorFromRgb15(0x8 as BYTE, 0x8 as BYTE, 0x8 as BYTE),
            BOOLEAN::TRUE,
            buildColorRgba(0 as BYTE, 0 as BYTE, 0xaa as BYTE, 0xff as BYTE),
            BOOLEAN::FALSE,
            buildColorRgba(0x50 as BYTE, 0x50 as BYTE, 0x50 as BYTE, 0 as BYTE),
        ), ());
    } else {
        uqm_lua_checked!(DrawBorder(C2Rust_Unnamed_3::DEVICE_CARGO_FRAME.0 as ::core::ffi::c_int as BYTE), ());
    }
    SetContextFont(StarConFont);
    t.baseline.x = (((64 as ::core::ffi::c_int) << resolutionFactor
        >> 1 as ::core::ffi::c_int) - ((1 as ::core::ffi::c_int) << resolutionFactor))
        as COORD;
    t.baseline.y = (r.corner.y as ::core::ffi::c_int
        + ((7 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    t.align = TEXT_ALIGN::ALIGN_CENTER;
    t.pStr = GetStringAddress(
        SetAbsStringTableIndex(
            GameStrings,
            (C2Rust_Unnamed_7::STARBASE_STRING_BASE.0 as ::core::ffi::c_int
                + 10 as ::core::ffi::c_int) as COUNT,
        ),
    );
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    SetContextForeGroundColor(
        buildColorRgba(0x55 as BYTE, 0xff as BYTE, 0xff as BYTE, 0xff as BYTE),
    );
    uqm_lua_checked!(font_DrawText(&raw mut t), ());
    t.baseline.y = (t.baseline.y as ::core::ffi::c_int
        + ((7 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    if stowed_q.0 != 0 {
        t.pStr = b"stored\0".as_ptr() as *const ::core::ffi::c_char as *const UNICODE;
    } else {
        t.pStr = b"new\0".as_ptr() as *const ::core::ffi::c_char as *const UNICODE;
    }
    uqm_lua_checked!(font_DrawText(&raw mut t), ());
    let mut count: COUNT = (if stowed_q.0 != 0 {
        stowCount as ::core::ffi::c_int
    } else {
        (*shipState).count as ::core::ffi::c_int
    }) as COUNT;
    i = 0 as COUNT;
    while (i as ::core::ffi::c_int) < 5 as ::core::ffi::c_int {
        let mut modIndex: COUNT = ((*shipState).topIndex as ::core::ffi::c_int
            + i as ::core::ffi::c_int) as COUNT;
        if modIndex as ::core::ffi::c_int >= count as ::core::ffi::c_int {
            break;
        }
        uqm_lua_checked!(DrawShipsStatus(modIndex, i, false), ());
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn DrawShips(mut shipState: *mut SHIPS_STATE, mut NewItem: COUNT) {
    let mut OldContext: CONTEXT = SetContext(StatusContext);
    let mut pos: COUNT = (NewItem as ::core::ffi::c_int
        - (*shipState).topIndex as ::core::ffi::c_int) as COUNT;
    BatchGraphics();
    uqm_lua_checked!(DrawShipsDisplay(shipState), ());
    uqm_lua_checked!(DrawShipsStatus(NewItem, pos, true), ());
    UnbatchGraphics();
    SetContext(OldContext);
}
unsafe extern "C" fn ManipulateShips(mut NewState: SIZE) {
    let mut shipState: *mut SHIPS_STATE = ::core::ptr::null_mut::<SHIPS_STATE>();
    let mut NewTop: SIZE = 0;
    if !(optDosMenus.0 != 0
        || if optWindowType == 0 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0)
    {
        return;
    }
    shipState = &raw mut ShipState;
    NewTop = (*shipState).topIndex as SIZE;
    if NewState as ::core::ffi::c_int
        > RACE_ID::NUM_BUILDABLE_SHIPS.0 as ::core::ffi::c_int
            + 200 as ::core::ffi::c_int
    {
        uqm_lua_checked!(DrawShips(&raw mut ShipState, NewState as COUNT), ());
        return;
    }
    if (NewState as ::core::ffi::c_int) < NewTop as ::core::ffi::c_int
        || NewState as ::core::ffi::c_int
            >= NewTop as ::core::ffi::c_int + 5 as ::core::ffi::c_int
    {
        (*shipState).topIndex = (NewState as ::core::ffi::c_int
            - NewState as ::core::ffi::c_int % 5 as ::core::ffi::c_int) as COUNT;
    }
    uqm_lua_checked!(DrawShips(shipState, NewState as COUNT), ());
}
unsafe extern "C" fn GetShipStats(
    mut ship_stats: *mut SHIP_STATS,
    mut species_id: SPECIES_ID,
) {
    let mut hMasterShip: HMASTERSHIP = FindMasterShip(species_id);
    let mut ShipPtr: *mut MASTER_SHIP_INFO = LockMasterShip(
        &raw mut master_q,
        hMasterShip,
    );
    snprintf(
        &raw mut (*ship_stats).ShipName as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[UNICODE; 30]>(),
        b"%s %s\0".as_ptr() as *const ::core::ffi::c_char,
        GetStringAddress(
            SetAbsStringTableIndex((*ShipPtr).ShipInfo.race_strings, 2 as COUNT),
        ),
        GetStringAddress(
            SetAbsStringTableIndex((*ShipPtr).ShipInfo.race_strings, 4 as COUNT),
        ),
    );
    (*ship_stats).ShipCost = ((*ShipPtr).ShipInfo.ship_cost as ::core::ffi::c_int
        * 100 as ::core::ffi::c_int) as COUNT;
    UnlockLink(&raw mut master_q, hMasterShip);
}
unsafe extern "C" fn GetStowedShipStats(
    mut ship_stats: *mut SHIP_STATS,
    mut hStowShip: HSHIPFRAG,
) {
    let mut StowShipPtr: *mut SHIP_FRAGMENT = LockShipFrag(
        &raw mut GlobData.Game_state.stowed_ship_q,
        hStowShip,
    );
    let mut hMasterShip: HMASTERSHIP = FindMasterShip((*StowShipPtr).SpeciesID);
    let mut ShipPtr: *mut MASTER_SHIP_INFO = LockMasterShip(
        &raw mut master_q,
        hMasterShip,
    );
    (*ship_stats).ShipCost = (*ShipPtr).ShipInfo.ship_cost as COUNT;
    UnlockLink(&raw mut master_q, hMasterShip);
    snprintf(
        &raw mut (*ship_stats).ShipName as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[UNICODE; 30]>(),
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        GetStringAddress(
            SetAbsStringTableIndex(
                (*StowShipPtr).race_strings,
                (*StowShipPtr).captains_name_index as COUNT,
            ),
        ),
    );
    (*ship_stats).CrewLevel = (*StowShipPtr).crew_level as BYTE;
    (*ship_stats).MaxCrew = (*StowShipPtr).max_crew as BYTE;
    UnlockLink(&raw mut GlobData.Game_state.stowed_ship_q, hStowShip);
}
unsafe extern "C" fn InventoryShips(mut ship_state: *mut SHIPS_STATE) {
    let mut i: COUNT = 0;
    let mut ShipsOnBoard: SIZE = 0 as SIZE;
    i = 0 as COUNT;
    while (i as ::core::ffi::c_int)
        < RACE_ID::NUM_BUILDABLE_SHIPS.0 as ::core::ffi::c_int
    {
        let mut hStarShip: HFLEETINFO = GetStarShipFromIndex(
            &raw mut GlobData.Game_state.avail_race_q,
            i,
        );
        let mut FleetPtr: *mut FLEET_INFO = LockFleetInfo(
            &raw mut GlobData.Game_state.avail_race_q,
            hStarShip,
        );
        if (*FleetPtr).allied_state as ::core::ffi::c_int
            == C2Rust_Unnamed_1::GOOD_GUY.0 as ::core::ffi::c_int
            || (*FleetPtr).can_build.0 != 0
        {
            GetShipStats(
                (&raw mut (*ship_state).ShipStats as *mut SHIP_STATS)
                    .offset(ShipsOnBoard as isize),
                (*FleetPtr).SpeciesID,
            );
            ShipsOnBoard += 1;
        }
        UnlockLink(&raw mut GlobData.Game_state.avail_race_q, hStarShip);
        i = i.wrapping_add(1);
    }
    (*ship_state).count = ShipsOnBoard as COUNT;
    stowCount = GetStowedShipCount();
    let mut hStowShip: HSHIPFRAG = GetStarShipFromIndex(
        &raw mut GlobData.Game_state.stowed_ship_q,
        0 as COUNT,
    );
    while (i as ::core::ffi::c_int)
        < stowCount as ::core::ffi::c_int
            + RACE_ID::NUM_BUILDABLE_SHIPS.0 as ::core::ffi::c_int
        && !hStowShip.is_null()
    {
        let mut StowShipPtr: *mut SHIP_FRAGMENT = LockShipFrag(
            &raw mut GlobData.Game_state.stowed_ship_q,
            hStowShip,
        );
        let mut hNext: HSHIPFRAG = (*StowShipPtr).succ;
        GetStowedShipStats(
            (&raw mut (*ship_state).ShipStats as *mut SHIP_STATS).offset(i as isize),
            hStowShip,
        );
        UnlockLink(&raw mut GlobData.Game_state.stowed_ship_q, hStowShip);
        hStowShip = hNext;
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn FillHangarX() {
    let mut i: BYTE = 0;
    i = 0 as BYTE;
    while (i as ::core::ffi::c_int)
        < if if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            4 as ::core::ffi::c_int
        } else {
            6 as ::core::ffi::c_int
        }
    {
        if if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            hangar_x_coords[i as usize] = hangar_x_coords_3do[i as usize];
        } else if if optWindowType == 0 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } == 0
        {
            hangar_x_coords[i as usize] = hangar_x_coords_uqm[i as usize];
        } else {
            hangar_x_coords[i as usize] = hangar_x_coords_dos[i as usize];
        }
        if if resolutionFactor != 2 as ::core::ffi::c_uint {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } != 0
        {
            hangar_x_coords[i as usize] = ((hangar_x_coords[i as usize]
                as ::core::ffi::c_int) << resolutionFactor) as COORD;
        }
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn showRemainingCrew() {
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
    let mut buf: [UNICODE; 30] = [0; 30];
    let mut remaining_crew: SIZE = 0;
    if if if GlobData.SIS_state.Difficulty as ::core::ffi::c_int != 0 {
        GlobData.SIS_state.Difficulty as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    } == 2 as ::core::ffi::c_int
    {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    } == 0
    {
        return;
    }
    BatchGraphics();
    remaining_crew = (2000 as ::core::ffi::c_int
        - (((uqm_lua_checked!(getGameStateUint(
            b"CREW_PURCHASED1\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) as BYTE as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as UWORD
            as ::core::ffi::c_int
            | uqm_lua_checked!(getGameStateUint(
                b"CREW_PURCHASED0\0".as_ptr() as *const ::core::ffi::c_char,
            ), ()) as BYTE as ::core::ffi::c_int) as SIZE as ::core::ffi::c_int) as SIZE;
    r.extent = MAKE_EXTENT(
        ((122 as ::core::ffi::c_int) << resolutionFactor) as COORD,
        ((7 as ::core::ffi::c_int) << resolutionFactor) as COORD,
    );
    r.corner = MAKE_POINT(
        (if optWindowType == 2 as DWORD {
            2 as ::core::ffi::c_int
        } else if optWindowType == 1 as DWORD {
            6 as ::core::ffi::c_int
        } else {
            19 as ::core::ffi::c_int
        } << resolutionFactor) as COORD,
        (((74 as ::core::ffi::c_int) << resolutionFactor)
            - (r.extent.height as ::core::ffi::c_int
                + ((2 as ::core::ffi::c_int) << resolutionFactor))) as COORD,
    );
    if optWindowType < 2 as DWORD {
        r.corner.y = ((1 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    }
    SetContextForeGroundColor(colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE));
    uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
    SetContextFont(TinyFont);
    SetContextForeGroundColor(colorFromRgb15(0 as BYTE, 0 as BYTE, 0x16 as BYTE));
    t.baseline.x = r.corner.x;
    t.baseline.y = (r.corner.y as ::core::ffi::c_int
        + r.extent.height as ::core::ffi::c_int
        - ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    t.align = TEXT_ALIGN::ALIGN_LEFT;
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    t.pStr = &raw mut buf as *mut UNICODE;
    utf8StringCopy(
        &raw mut buf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[UNICODE; 30]>(),
        GetStringAddress(
            SetAbsStringTableIndex(
                GameStrings,
                (C2Rust_Unnamed_7::STARBASE_STRING_BASE.0 as ::core::ffi::c_int
                    + 6 as ::core::ffi::c_int) as COUNT,
            ),
        ),
    );
    uqm_lua_checked!(font_DrawText(&raw mut t), ());
    t.baseline.x = (t.baseline.x as ::core::ffi::c_int
        + ((71 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    SetContextForeGroundColor(
        if remaining_crew as ::core::ffi::c_int > 1000 as ::core::ffi::c_int {
            colorFromRgb15(0 as BYTE, 0x16 as BYTE, 0 as BYTE)
        } else if (remaining_crew as ::core::ffi::c_int) < 250 as ::core::ffi::c_int {
            colorFromRgb15(0x16 as BYTE, 0 as BYTE, 0 as BYTE)
        } else {
            colorFromRgb15(0x16 as BYTE, 0x16 as BYTE, 0 as BYTE)
        },
    );
    if CheckAlliance(RACE_ID::SHOFIXTI_SHIP) as ::core::ffi::c_int
        == C2Rust_Unnamed_1::GOOD_GUY.0 as ::core::ffi::c_int
    {
        sprintf(
            &raw mut buf as *mut ::core::ffi::c_char,
            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
            b"\xE2\x88\x9E\0".as_ptr() as *const ::core::ffi::c_char,
        );
    } else if remaining_crew as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        utf8StringCopy(
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[UNICODE; 30]>(),
            GetStringAddress(
                SetAbsStringTableIndex(
                    GameStrings,
                    (C2Rust_Unnamed_7::STARBASE_STRING_BASE.0 as ::core::ffi::c_int
                        + 7 as ::core::ffi::c_int) as COUNT,
                ),
            ),
        );
    } else {
        sprintf(
            &raw mut buf as *mut ::core::ffi::c_char,
            b"%u\0".as_ptr() as *const ::core::ffi::c_char,
            remaining_crew as ::core::ffi::c_int,
        );
    }
    uqm_lua_checked!(font_DrawText(&raw mut t), ());
    UnbatchGraphics();
}
#[no_mangle]
pub unsafe extern "C" fn ShipPoints(mut hFleet: HFLEETINFO) -> COUNT {
    if hFleet.is_null() {
        return 0 as COUNT;
    }
    let mut FleetPtr: *mut FLEET_INFO = LockFleetInfo(
        &raw mut GlobData.Game_state.avail_race_q,
        hFleet,
    );
    let mut shipCost: COUNT = GetShipCostFromIndex(
        FindMasterShipIndex((*FleetPtr).SpeciesID) as ::core::ffi::c_uint,
    );
    UnlockLink(&raw mut GlobData.Game_state.avail_race_q, hFleet);
    return shipCost;
}
#[no_mangle]
pub unsafe extern "C" fn ShipCost(mut race_id: BYTE) -> COUNT {
    return (ShipPoints(
        GetStarShipFromIndex(&raw mut GlobData.Game_state.avail_race_q, race_id as COUNT),
    ) as ::core::ffi::c_int * 100 as ::core::ffi::c_int) as COUNT;
}
unsafe extern "C" fn CalculateAllyPoints() -> SIZE {
    let mut i: BYTE = 0;
    let mut hFleet: HFLEETINFO = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut FleetPtr: *mut FLEET_INFO = ::core::ptr::null_mut::<FLEET_INFO>();
    let mut MaxPoints: SIZE = (if if GlobData.SIS_state.Difficulty as ::core::ffi::c_int
        != 0
    {
        GlobData.SIS_state.Difficulty as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    } == 0 as ::core::ffi::c_int
    {
        60 as ::core::ffi::c_int
    } else if if GlobData.SIS_state.Difficulty as ::core::ffi::c_int != 0 {
        GlobData.SIS_state.Difficulty as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    } == 1 as ::core::ffi::c_int
    {
        90 as ::core::ffi::c_int
    } else {
        30 as ::core::ffi::c_int
    }) as SIZE;
    i = RACE_ID::ARILOU_SHIP.0 as ::core::ffi::c_int as BYTE;
    while i as ::core::ffi::c_int <= SPECIES_ID::LAST_MELEE_ID.0 as ::core::ffi::c_int {
        hFleet = GetStarShipFromIndex(
            &raw mut GlobData.Game_state.avail_race_q,
            i as COUNT,
        ) as HFLEETINFO;
        if !hFleet.is_null() {
            FleetPtr = LockFleetInfo(&raw mut GlobData.Game_state.avail_race_q, hFleet);
            if !FleetPtr.is_null() {
                if (*FleetPtr).allied_state as ::core::ffi::c_int
                    == C2Rust_Unnamed_1::GOOD_GUY.0 as ::core::ffi::c_int
                {
                    MaxPoints = (MaxPoints as ::core::ffi::c_int
                        + ShipPoints(hFleet) as ::core::ffi::c_int
                            * if if GlobData.SIS_state.Difficulty as ::core::ffi::c_int
                                != 0
                            {
                                GlobData.SIS_state.Difficulty as ::core::ffi::c_int
                            } else {
                                0 as ::core::ffi::c_int
                            } == 0 as ::core::ffi::c_int
                            {
                                2 as ::core::ffi::c_int
                            } else if if GlobData.SIS_state.Difficulty
                                as ::core::ffi::c_int != 0
                            {
                                GlobData.SIS_state.Difficulty as ::core::ffi::c_int
                            } else {
                                0 as ::core::ffi::c_int
                            } == 1 as ::core::ffi::c_int
                            {
                                3 as ::core::ffi::c_int
                            } else {
                                1 as ::core::ffi::c_int
                            }) as SIZE;
                }
                UnlockLink(&raw mut GlobData.Game_state.avail_race_q, hFleet);
            }
        }
        i = i.wrapping_add(1);
    }
    return MaxPoints;
}
#[no_mangle]
pub unsafe extern "C" fn CalculateEscortsPoints() -> COUNT {
    let mut total: COUNT = 0 as COUNT;
    let mut hStarShip: HSHIPFRAG = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut hNextShip: HSHIPFRAG = ::core::ptr::null_mut::<::core::ffi::c_void>();
    hStarShip = GlobData.Game_state.built_ship_q.head as HSHIPFRAG;
    while !hStarShip.is_null() {
        let mut StarShipPtr: *mut SHIP_FRAGMENT = LockShipFrag(
            &raw mut GlobData.Game_state.built_ship_q,
            hStarShip,
        );
        hNextShip = (*StarShipPtr).succ as HSHIPFRAG;
        total = (total as ::core::ffi::c_int
            + ShipPoints(
                GetStarShipFromIndex(
                    &raw mut GlobData.Game_state.avail_race_q,
                    (*StarShipPtr).race_id as COUNT,
                ),
            ) as ::core::ffi::c_int) as COUNT;
        UnlockLink(&raw mut GlobData.Game_state.built_ship_q, hStarShip);
        hStarShip = hNextShip;
    }
    return total;
}
#[no_mangle]
pub unsafe extern "C" fn CanBuyPoints(mut hFleet: HFLEETINFO) -> BOOLEAN {
    if optFleetPointSys.0 == 0
        && if if GlobData.SIS_state.Difficulty as ::core::ffi::c_int != 0 {
            GlobData.SIS_state.Difficulty as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        } == 2 as ::core::ffi::c_int
        {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        } == 0
        || uqm_lua_checked!(getGameStateUint(b"CHMMR_BOMB_STATE\0".as_ptr() as *const ::core::ffi::c_char), BOOLEAN(0))
            == 3 as DWORD
    {
        return BOOLEAN::TRUE;
    }
    if ShipPoints(hFleet) as ::core::ffi::c_int
        > CalculateAllyPoints() as ::core::ffi::c_int
            - CalculateEscortsPoints() as ::core::ffi::c_int
    {
        return BOOLEAN::FALSE;
    }
    return BOOLEAN::TRUE;
}
unsafe extern "C" fn showRemainingPoints(mut delta: ::core::ffi::c_int) {
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
    let mut buf: [UNICODE; 30] = [0; 30];
    let mut percentage_left: SBYTE = 0;
    let mut FP: SIZE = (CalculateAllyPoints() as ::core::ffi::c_int
        - CalculateEscortsPoints() as ::core::ffi::c_int + delta) as SIZE;
    if optFleetPointSys.0 == 0
        && if if GlobData.SIS_state.Difficulty as ::core::ffi::c_int != 0 {
            GlobData.SIS_state.Difficulty as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        } == 2 as ::core::ffi::c_int
        {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        } == 0
        || uqm_lua_checked!(getGameStateUint(b"CHMMR_BOMB_STATE\0".as_ptr() as *const ::core::ffi::c_char), ())
            == 3 as DWORD
    {
        return;
    }
    BatchGraphics();
    r.extent = MAKE_EXTENT(
        ((100 as ::core::ffi::c_int) << resolutionFactor) as COORD,
        ((7 as ::core::ffi::c_int) << resolutionFactor) as COORD,
    );
    r.corner.x = ((if optWindowType == 2 as DWORD || optWindowType == 0 as DWORD {
        243 as ::core::ffi::c_int
    } else {
        210 as ::core::ffi::c_int
    } - 100 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    r.corner.y = (((74 as ::core::ffi::c_int) << resolutionFactor)
        - (r.extent.height as ::core::ffi::c_int
            + ((2 as ::core::ffi::c_int) << resolutionFactor))) as COORD;
    if optWindowType < 2 as DWORD {
        r.corner.y = ((1 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    }
    SetContextForeGroundColor(colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE));
    uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
    SetContextFont(TinyFont);
    t.align = TEXT_ALIGN::ALIGN_RIGHT;
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    t.pStr = &raw mut buf as *mut UNICODE;
    sprintf(
        &raw mut buf as *mut ::core::ffi::c_char,
        b"%d\0".as_ptr() as *const ::core::ffi::c_char,
        FP as ::core::ffi::c_int,
    );
    r.corner.x = (if optWindowType == 2 as DWORD || optWindowType == 0 as DWORD {
        243 as ::core::ffi::c_int
    } else {
        210 as ::core::ffi::c_int
    }
        - if optWindowType == 2 as DWORD {
            2 as ::core::ffi::c_int
        } else if optWindowType == 1 as DWORD {
            6 as ::core::ffi::c_int
        } else {
            19 as ::core::ffi::c_int
        } << resolutionFactor) as COORD;
    t.baseline.x = r.corner.x;
    t.baseline.y = (r.corner.y as ::core::ffi::c_int
        + r.extent.height as ::core::ffi::c_int
        - ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    percentage_left = (if FP as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
        FP as ::core::ffi::c_float / CalculateAllyPoints() as ::core::ffi::c_float
            * 100 as ::core::ffi::c_int as ::core::ffi::c_float
    } else {
        0 as ::core::ffi::c_int as ::core::ffi::c_float
    }) as SBYTE;
    SetContextForeGroundColor(
        if percentage_left as ::core::ffi::c_int > 50 as ::core::ffi::c_int {
            colorFromRgb15(0 as BYTE, 0x16 as BYTE, 0 as BYTE)
        } else if (percentage_left as ::core::ffi::c_int) < 25 as ::core::ffi::c_int {
            colorFromRgb15(0x16 as BYTE, 0 as BYTE, 0 as BYTE)
        } else {
            colorFromRgb15(0x16 as BYTE, 0x16 as BYTE, 0 as BYTE)
        },
    );
    uqm_lua_checked!(font_DrawText(&raw mut t), ());
    r = uqm_lua_checked!(font_GetTextRect(&raw mut t), ());
    t.baseline.x = (t.baseline.x as ::core::ffi::c_int
        - r.extent.width as ::core::ffi::c_int) as COORD;
    utf8StringCopy(
        &raw mut buf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[UNICODE; 30]>(),
        GetStringAddress(
            SetAbsStringTableIndex(
                GameStrings,
                (C2Rust_Unnamed_7::STARBASE_STRING_BASE.0 as ::core::ffi::c_int
                    + 8 as ::core::ffi::c_int) as COUNT,
            ),
        ),
    );
    SetContextForeGroundColor(colorFromRgb15(0 as BYTE, 0 as BYTE, 0x16 as BYTE));
    uqm_lua_checked!(font_DrawText(&raw mut t), ());
    UnbatchGraphics();
}
unsafe extern "C" fn animatePowerLines(mut pMS: *mut MENU_STATE) {
    static mut s: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    static mut ColorMap: COLORMAP = ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>();
    static mut NextTime: TimeCount = 0 as TimeCount;
    let mut Now: TimeCount = GetTimeCounter();
    if if optWindowType == 1 as DWORD {
        BOOLEAN::TRUE.0 as ::core::ffi::c_int
    } else {
        BOOLEAN::FALSE.0 as ::core::ffi::c_int
    } != 0
        || GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
            & (C2Rust_Unnamed_5::CHECK_ABORT.0 as ::core::ffi::c_int
                | C2Rust_Unnamed_5::CHECK_LOAD.0 as ::core::ffi::c_int) != 0
    {
        return;
    }
    if !pMS.is_null() {
        s.origin.x = 0 as COORD;
        s.origin.y = 0 as COORD;
        s.frame = SetAbsFrameIndex((*pMS).ModuleFrame, 28 as COUNT);
        ColorMap = SetAbsStringTableIndex((*pMS).CurString, 0 as COUNT) as COLORMAP;
    }
    if Now >= NextTime || !pMS.is_null() {
        NextTime = Now
            .wrapping_add(
                (840 as ::core::ffi::c_int
                    / ((15 as ::core::ffi::c_int) << resolutionFactor)) as TimeCount,
            );
        SetColorMap(GetColorMapAddress(ColorMap));
        uqm_lua_checked!(DrawStamp(&raw mut s), ());
        ColorMap = SetRelStringTableIndex(ColorMap, 1 as SIZE) as COLORMAP;
    }
}
unsafe extern "C" fn SpinStarShip(mut pMS: *mut MENU_STATE, mut hStarShip: HFLEETINFO) {
    let mut Index: ::core::ffi::c_int = 0;
    let mut FleetPtr: *mut FLEET_INFO = ::core::ptr::null_mut::<FLEET_INFO>();
    FleetPtr = LockFleetInfo(&raw mut GlobData.Game_state.avail_race_q, hStarShip);
    Index = FindMasterShipIndex((*FleetPtr).SpeciesID);
    UnlockLink(&raw mut GlobData.Game_state.avail_race_q, hStarShip);
    if Index >= 0 as ::core::ffi::c_int
        && Index
            < MeleeShip::MELEE_ZOQFOTPIK.0 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int
    {
        if isPC(optWhichIntro).0 != 0 {
            uqm_lua_checked!(PlayMenuSound(MENU_SOUND_EFFECT::MENU_SOUND_SUCCESS), ());
        }
        DoShipSpins = BOOLEAN::TRUE;
        uqm_lua_checked!(DoShipSpin(Index as COUNT, (*pMS).hMusic), ());
    }
    DoShipSpins = BOOLEAN::FALSE;
}
unsafe extern "C" fn on_input_frame() {
    let mut oldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    oldContext = SetContext(SpaceContext);
    if DoShipSpins.0 == 0 {
        uqm_lua_checked!(animatePowerLines(::core::ptr::null_mut::<MENU_STATE>()), ());
    }
    SetContext(oldContext);
}
unsafe extern "C" fn GetAvailableRaceCount() -> COUNT {
    let mut Index: COUNT = 0;
    let mut hStarShip: HFLEETINFO = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut hNextShip: HFLEETINFO = ::core::ptr::null_mut::<::core::ffi::c_void>();
    Index = 0 as COUNT;
    hStarShip = GlobData.Game_state.avail_race_q.head as HFLEETINFO;
    while !hStarShip.is_null() {
        let mut FleetPtr: *mut FLEET_INFO = ::core::ptr::null_mut::<FLEET_INFO>();
        FleetPtr = LockFleetInfo(&raw mut GlobData.Game_state.avail_race_q, hStarShip);
        if (*FleetPtr).allied_state as ::core::ffi::c_int
            == C2Rust_Unnamed_1::GOOD_GUY.0 as ::core::ffi::c_int
            || (*FleetPtr).can_build.0 != 0
        {
            Index = Index.wrapping_add(1);
        }
        hNextShip = (*FleetPtr).succ;
        UnlockLink(&raw mut GlobData.Game_state.avail_race_q, hStarShip);
        hStarShip = hNextShip;
    }
    return Index;
}
unsafe extern "C" fn GetAvailableRaceFromIndex(mut Index: BYTE) -> HFLEETINFO {
    let mut hStarShip: HFLEETINFO = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut hNextShip: HFLEETINFO = ::core::ptr::null_mut::<::core::ffi::c_void>();
    hStarShip = GlobData.Game_state.avail_race_q.head as HFLEETINFO;
    while !hStarShip.is_null() {
        let mut FleetPtr: *mut FLEET_INFO = ::core::ptr::null_mut::<FLEET_INFO>();
        FleetPtr = LockFleetInfo(&raw mut GlobData.Game_state.avail_race_q, hStarShip);
        if ((*FleetPtr).allied_state as ::core::ffi::c_int
            == C2Rust_Unnamed_1::GOOD_GUY.0 as ::core::ffi::c_int
            || (*FleetPtr).can_build.0 != 0)
            && {
                let c2rust_fresh0 = Index;
                Index = Index.wrapping_sub(1);
                c2rust_fresh0 as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            }
        {
            UnlockLink(&raw mut GlobData.Game_state.avail_race_q, hStarShip);
            return hStarShip;
        }
        hNextShip = (*FleetPtr).succ;
        UnlockLink(&raw mut GlobData.Game_state.avail_race_q, hStarShip);
        hStarShip = hNextShip;
    }
    return ::core::ptr::null_mut::<::core::ffi::c_void>();
}
unsafe extern "C" fn DrawShipyardShipText(
    mut r: *mut RECT,
    mut Index: ::core::ffi::c_int,
) {
    let mut race_name: [UNICODE; 64] = [0; 64];
    let mut ship_name: [UNICODE; 64] = [0; 64];
    let mut OldFont: FONT = ::core::ptr::null_mut::<FONT_DESC>();
    let mut OldColor: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    let mut leading: SIZE = 0;
    let mut block: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut text: TEXT = TEXT {
        baseline: POINT { x: 0, y: 0 },
        pStr: ::core::ptr::null::<UNICODE>(),
        align: TEXT_ALIGN::ALIGN_LEFT,
        CharCount: 0,
    };
    let mut strip_align: COORD = 0;
    if if optWindowType == 0 as DWORD {
        BOOLEAN::TRUE.0 as ::core::ffi::c_int
    } else {
        BOOLEAN::FALSE.0 as ::core::ffi::c_int
    } != 0
    {
        return;
    }
    let mut hStarShip: HFLEETINFO = GetStarShipFromIndex(
        &raw mut GlobData.Game_state.avail_race_q,
        Index as COUNT,
    );
    let mut FleetPtr: *mut FLEET_INFO = LockFleetInfo(
        &raw mut GlobData.Game_state.avail_race_q,
        hStarShip,
    );
    let mut hMasterShip: HMASTERSHIP = FindMasterShip((*FleetPtr).SpeciesID);
    let mut ShipPtr: *mut MASTER_SHIP_INFO = LockMasterShip(
        &raw mut master_q,
        hMasterShip,
    );
    utf8StringCopy(
        &raw mut race_name as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[UNICODE; 64]>(),
        GetStringAddress(
            SetAbsStringTableIndex(
                (*ShipPtr).ShipInfo.race_strings,
                (GetStringTableCount((*ShipPtr).ShipInfo.race_strings)
                    as ::core::ffi::c_int - 3 as ::core::ffi::c_int) as COUNT,
            ),
        ),
    );
    utf8StringCopy(
        &raw mut ship_name as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[UNICODE; 64]>(),
        GetStringAddress(
            SetAbsStringTableIndex(
                (*ShipPtr).ShipInfo.race_strings,
                (GetStringTableCount((*ShipPtr).ShipInfo.race_strings)
                    as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as COUNT,
            ),
        ),
    );
    UnlockLink(&raw mut GlobData.Game_state.avail_race_q, hStarShip);
    UnlockLink(&raw mut master_q, hMasterShip);
    if strlen(&raw mut race_name as *mut ::core::ffi::c_char) == 0
        || strlen(&raw mut ship_name as *mut ::core::ffi::c_char) == 0
    {
        return;
    }
    OldFont = SetContextFont(ModuleFont);
    OldColor = SetContextForeGroundColor(
        colorFromRgb15(0xa as BYTE, 0xa as BYTE, 0xa as BYTE),
    );
    GetContextFontLeading(&raw mut leading);
    if optCustomBorder.0 == 0 {
        block = *r;
        block.extent.height = ((leading as ::core::ffi::c_int)
            << 1 as ::core::ffi::c_int) as COORD;
        uqm_lua_checked!(DrawFilledRectangle(&raw mut block), ());
    } else {
        uqm_lua_checked!(DrawBorder(C2Rust_Unnamed_3::TEXT_LABEL_FRAME.0 as ::core::ffi::c_int as BYTE), ());
    }
    text.align = TEXT_ALIGN::ALIGN_CENTER;
    text.baseline.x = ((*r).corner.x as ::core::ffi::c_int
        + ((*r).extent.width as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)) as COORD;
    text.baseline.y = ((*r).corner.y as ::core::ffi::c_int
        + leading as ::core::ffi::c_int) as COORD;
    strip_align = text.baseline.x;
    text.pStr = AlignText(&raw mut race_name as *const UNICODE, &raw mut strip_align);
    text.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    uqm_lua_checked!(font_DrawShadowedText(
        &raw mut text,
        C2Rust_Unnamed::WEST_SHADOW.0 as ::core::ffi::c_int as BYTE,
        buildColorRgba(0x78 as BYTE, 0x78 as BYTE, 0x78 as BYTE, 0xff as BYTE),
        buildColorRgba(0x4b as BYTE, 0x4b as BYTE, 0x4b as BYTE, 0xff as BYTE),
    ), ());
    text.baseline.y = (text.baseline.y as ::core::ffi::c_int
        + leading as ::core::ffi::c_int) as COORD;
    text.pStr = AlignText(&raw mut ship_name as *const UNICODE, &raw mut strip_align);
    text.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    uqm_lua_checked!(font_DrawShadowedText(
        &raw mut text,
        C2Rust_Unnamed::WEST_SHADOW.0 as ::core::ffi::c_int as BYTE,
        buildColorRgba(0x78 as BYTE, 0x78 as BYTE, 0x78 as BYTE, 0xff as BYTE),
        buildColorRgba(0x4b as BYTE, 0x4b as BYTE, 0x4b as BYTE, 0xff as BYTE),
    ), ());
    SetContextFont(OldFont);
    SetContextForeGroundColor(OldColor);
}
unsafe extern "C" fn DrawRaceStrings(mut NewRaceItem: BYTE) {
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut textRect: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut s: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    let mut OldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    OldContext = SetContext(StatusContext);
    GetContextClipRect(&raw mut r);
    s.origin.x = (((4 as ::core::ffi::c_int) << resolutionFactor)
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
            + (if if optWindowType == 1 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                16 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } << resolutionFactor)) - r.corner.x as ::core::ffi::c_int) as COORD;
    s.origin.y = (((9 as ::core::ffi::c_int) << resolutionFactor)
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
            - ((13 as ::core::ffi::c_int) << resolutionFactor))
        - ((53 as ::core::ffi::c_int) << resolutionFactor)
        - r.corner.y as ::core::ffi::c_int) as COORD;
    r.corner.x = (s.origin.x as ::core::ffi::c_int
        - ((2 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    r.corner.y = (s.origin.y as ::core::ffi::c_int
        - ((12 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    r.extent.width = (((64 as ::core::ffi::c_int) << resolutionFactor)
        - ((8 as ::core::ffi::c_int) << resolutionFactor)
        + ((4 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    r.extent.height = ((12 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    BatchGraphics();
    uqm_lua_checked!(ClearSISRect(((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as BYTE), ());
    SetContextForeGroundColor(colorFromRgb15(0xa as BYTE, 0xa as BYTE, 0xa as BYTE));
    if if optWindowType == 0 as DWORD {
        BOOLEAN::TRUE.0 as ::core::ffi::c_int
    } else {
        BOOLEAN::FALSE.0 as ::core::ffi::c_int
    } == 0
    {
        if optCustomBorder.0 == 0 {
            uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
        }
        textRect = r;
    }
    uqm_lua_checked!(DrawBorder(C2Rust_Unnamed_3::SIS_RADAR_FRAME.0 as ::core::ffi::c_int as BYTE), ());
    if if optWindowType == 0 as DWORD {
        BOOLEAN::TRUE.0 as ::core::ffi::c_int
    } else {
        BOOLEAN::FALSE.0 as ::core::ffi::c_int
    } == 0
    {
        r.corner = s.origin;
        r.extent.width = (((64 as ::core::ffi::c_int) << resolutionFactor)
            - ((8 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        r.extent.height = ((53 as ::core::ffi::c_int) << resolutionFactor) as COORD;
        SetContextForeGroundColor(colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE));
        uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
    } else {
        let mut dosRect: RECT = RECT {
            corner: POINT { x: 0, y: 0 },
            extent: EXTENT { width: 0, height: 0 },
        };
        dosRect.corner.x = ((3 as ::core::ffi::c_int) << resolutionFactor) as COORD;
        dosRect.corner.y = (((9 as ::core::ffi::c_int) << resolutionFactor)
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
                - ((13 as ::core::ffi::c_int) << resolutionFactor))
            - ((53 as ::core::ffi::c_int) << resolutionFactor)
            + ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        dosRect.extent.width = (((64 as ::core::ffi::c_int) << resolutionFactor)
            - ((8 as ::core::ffi::c_int) << resolutionFactor)
            + ((2 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        dosRect.extent.height = (((53 as ::core::ffi::c_int) << resolutionFactor)
            - ((2 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        if if resolutionFactor != 2 as ::core::ffi::c_uint {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } == 0
        {
            uqm_lua_checked!(DrawStarConBox(
                &raw mut dosRect,
                ((1 as ::core::ffi::c_int) << resolutionFactor) as SIZE,
                colorFromRgb15(0xf as BYTE, 0xf as BYTE, 0xf as BYTE),
                colorFromRgb15(0x6 as BYTE, 0x6 as BYTE, 0x6 as BYTE),
                BOOLEAN::TRUE,
                colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE),
                BOOLEAN::FALSE,
                buildColorRgba(0x50 as BYTE, 0x50 as BYTE, 0x50 as BYTE, 0 as BYTE),
            ), ());
        } else {
            uqm_lua_checked!(DrawRenderedBox(
                &raw mut dosRect,
                BOOLEAN::TRUE,
                colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE),
                C2Rust_Unnamed_4::THIN_INNER_BEVEL.0 as ::core::ffi::c_int,
                optCustomBorder.0 as ::core::ffi::c_int,
            ), ());
        }
    }
    if NewRaceItem as ::core::ffi::c_int
        != !(0 as ::core::ffi::c_int) as BYTE as ::core::ffi::c_int
    {
        let mut t: TEXT = TEXT {
            baseline: POINT { x: 0, y: 0 },
            pStr: ::core::ptr::null::<UNICODE>(),
            align: TEXT_ALIGN::ALIGN_LEFT,
            CharCount: 0,
        };
        let mut hStarShip: HFLEETINFO = ::core::ptr::null_mut::<::core::ffi::c_void>();
        let mut FleetPtr: *mut FLEET_INFO = ::core::ptr::null_mut::<FLEET_INFO>();
        let mut buf: [UNICODE; 30] = [0; 30];
        let mut shipCost: COUNT = 0;
        let mut shipPoints: COUNT = 0;
        let mut shipCrew: COUNT = 0;
        let mut maxCrew: COUNT = 0;
        let mut r_0: RECT = RECT {
            corner: POINT { x: 0, y: 0 },
            extent: EXTENT { width: 0, height: 0 },
        };
        let mut captain: STRING = ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>();
        uqm_lua_checked!(ManipulateShips(NewRaceItem as SIZE), ());
        if stowed_q.0 != 0 {
            let mut hStowShip: HSHIPFRAG = GetStarShipFromIndex(
                &raw mut GlobData.Game_state.stowed_ship_q,
                NewRaceItem as COUNT,
            );
            let mut StowShipPtr: *mut SHIP_FRAGMENT = LockShipFrag(
                &raw mut GlobData.Game_state.stowed_ship_q,
                hStowShip,
            );
            NewRaceItem = (*StowShipPtr).race_id;
            shipCrew = (*StowShipPtr).crew_level;
            captain = SetAbsStringTableIndex(
                (*StowShipPtr).race_strings,
                (*StowShipPtr).captains_name_index as COUNT,
            );
            UnlockLink(&raw mut GlobData.Game_state.stowed_ship_q, hStowShip);
            hStarShip = GetStarShipFromIndex(
                &raw mut GlobData.Game_state.avail_race_q,
                NewRaceItem as COUNT,
            ) as HFLEETINFO;
            FleetPtr = LockFleetInfo(
                &raw mut GlobData.Game_state.avail_race_q,
                hStarShip,
            );
            maxCrew = (*FleetPtr).crew_level;
            UnlockLink(&raw mut GlobData.Game_state.avail_race_q, hStarShip);
        } else {
            hStarShip = GetAvailableRaceFromIndex(NewRaceItem);
            NewRaceItem = GetIndexFromStarShip(
                &raw mut GlobData.Game_state.avail_race_q,
                hStarShip,
            ) as BYTE;
        }
        shipPoints = ShipPoints(hStarShip);
        shipCost = ShipCost(NewRaceItem);
        if if optWindowType == 0 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } == 0
        {
            if stowed_q.0 != 0 {
                t.baseline.x = (((4 as ::core::ffi::c_int) << resolutionFactor)
                    + (((64 as ::core::ffi::c_int) << resolutionFactor)
                        - ((8 as ::core::ffi::c_int) << resolutionFactor))
                        / 2 as ::core::ffi::c_int) as COORD;
                t.baseline.y = (((9 as ::core::ffi::c_int) << resolutionFactor)
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
                        - ((13 as ::core::ffi::c_int) << resolutionFactor))
                    - ((53 as ::core::ffi::c_int) << resolutionFactor)
                    - ((2 as ::core::ffi::c_int) << resolutionFactor)
                    - (if if optWindowType == 0 as DWORD {
                        BOOLEAN::TRUE.0 as ::core::ffi::c_int
                    } else {
                        BOOLEAN::FALSE.0 as ::core::ffi::c_int
                    } != 0
                    {
                        2 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    } << resolutionFactor)
                    - (if if optWindowType == 1 as DWORD {
                        BOOLEAN::TRUE.0 as ::core::ffi::c_int
                    } else {
                        BOOLEAN::FALSE.0 as ::core::ffi::c_int
                    } != 0
                    {
                        16 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    } << resolutionFactor)) as COORD;
                t.align = TEXT_ALIGN::ALIGN_CENTER;
                t.pStr = GetStringAddress(captain);
                t.CharCount = GetStringLength(captain);
                uqm_lua_checked!(font_DrawShadowedText(
                    &raw mut t,
                    C2Rust_Unnamed::WEST_SHADOW.0 as ::core::ffi::c_int as BYTE,
                    colorFromRgb15(0x16 as BYTE, 0xb as BYTE, 0x1f as BYTE),
                    buildColorRgba(
                        0x4b as BYTE,
                        0x4b as BYTE,
                        0x4b as BYTE,
                        0xff as BYTE,
                    ),
                ), ());
            } else {
                uqm_lua_checked!(DrawShipyardShipText(
                    &raw mut textRect,
                    NewRaceItem as ::core::ffi::c_int,
                ), ());
            }
        }
        FleetPtr = LockFleetInfo(&raw mut GlobData.Game_state.avail_race_q, hStarShip);
        s.frame = (*FleetPtr).melee_icon;
        UnlockLink(&raw mut GlobData.Game_state.avail_race_q, hStarShip);
        s.origin.x = (s.origin.x as ::core::ffi::c_int
            + (((64 as ::core::ffi::c_int) << resolutionFactor)
                - ((8 as ::core::ffi::c_int) << resolutionFactor)
                >> 1 as ::core::ffi::c_int)) as COORD;
        s.origin.y = (s.origin.y as ::core::ffi::c_int
            + ((53 as ::core::ffi::c_int) << resolutionFactor
                >> 1 as ::core::ffi::c_int)) as COORD;
        uqm_lua_checked!(DrawStamp(&raw mut s), ());
        if isPC(optWhichFonts).0 != 0 {
            SetContextFont(TinyFont);
        } else {
            SetContextFont(TinyFontBold);
        }
        t.baseline.x = (((4 as ::core::ffi::c_int) << resolutionFactor)
            + (((64 as ::core::ffi::c_int) << resolutionFactor)
                - ((8 as ::core::ffi::c_int) << resolutionFactor))
            - ((2 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        t.baseline.y = (((9 as ::core::ffi::c_int) << resolutionFactor)
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
                - ((13 as ::core::ffi::c_int) << resolutionFactor))
            - ((53 as ::core::ffi::c_int) << resolutionFactor)
            + ((53 as ::core::ffi::c_int) << resolutionFactor)
            - ((2 as ::core::ffi::c_int) << resolutionFactor)
            - (if if optWindowType == 0 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                2 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } << resolutionFactor)
            - (if if optWindowType == 1 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                16 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } << resolutionFactor)) as COORD;
        t.align = TEXT_ALIGN::ALIGN_RIGHT;
        t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
        t.pStr = &raw mut buf as *mut UNICODE;
        if stowed_q.0 == 0 {
            sprintf(
                &raw mut buf as *mut ::core::ffi::c_char,
                b"%u\0".as_ptr() as *const ::core::ffi::c_char,
                shipCost as ::core::ffi::c_int,
            );
            if shipCost as DWORD <= GlobData.SIS_state.ResUnits {
                SetContextForeGroundColor(
                    colorFromRgb15(0 as BYTE, 0x1f as BYTE, 0 as BYTE),
                );
            } else if shipCost as DWORD > GlobData.SIS_state.ResUnits {
                SetContextForeGroundColor(
                    colorFromRgb15(0x1f as BYTE, 0 as BYTE, 0 as BYTE),
                );
            }
        } else {
            sprintf(
                &raw mut buf as *mut ::core::ffi::c_char,
                b"%u/%u\0".as_ptr() as *const ::core::ffi::c_char,
                shipCrew as ::core::ffi::c_int,
                maxCrew as ::core::ffi::c_int,
            );
            if shipCrew as ::core::ffi::c_int == maxCrew as ::core::ffi::c_int {
                SetContextForeGroundColor(
                    colorFromRgb15(0 as BYTE, 0x1f as BYTE, 0 as BYTE),
                );
            } else if shipCrew as ::core::ffi::c_int
                > maxCrew as ::core::ffi::c_int / 2 as ::core::ffi::c_int
            {
                SetContextForeGroundColor(
                    colorFromRgb15(0x1f as BYTE, 0x1f as BYTE, 0 as BYTE),
                );
            } else {
                SetContextForeGroundColor(
                    colorFromRgb15(0x1f as BYTE, 0 as BYTE, 0 as BYTE),
                );
            }
        }
        uqm_lua_checked!(font_DrawText(&raw mut t), ());
        if (optFleetPointSys.0 != 0
            || if if GlobData.SIS_state.Difficulty as ::core::ffi::c_int != 0 {
                GlobData.SIS_state.Difficulty as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } == 2 as ::core::ffi::c_int
            {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } != 0)
            && uqm_lua_checked!(getGameStateUint(
                b"CHMMR_BOMB_STATE\0".as_ptr() as *const ::core::ffi::c_char,
            ), ()) < 3 as DWORD
        {
            t.baseline.y = (((9 as ::core::ffi::c_int) << resolutionFactor)
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
                    - ((13 as ::core::ffi::c_int) << resolutionFactor))
                - ((53 as ::core::ffi::c_int) << resolutionFactor)
                + ((7 as ::core::ffi::c_int) << resolutionFactor)
                + (if if optWindowType == 0 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    2 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                } << resolutionFactor)
                - (if if optWindowType == 1 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    16 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                } << resolutionFactor)) as COORD;
            sprintf(
                &raw mut buf as *mut ::core::ffi::c_char,
                b"%u\0".as_ptr() as *const ::core::ffi::c_char,
                shipPoints as ::core::ffi::c_int,
            );
            if (shipPoints as ::core::ffi::c_int)
                < (CalculateAllyPoints() as ::core::ffi::c_int
                    - CalculateEscortsPoints() as ::core::ffi::c_int)
                    / 2 as ::core::ffi::c_int
            {
                SetContextForeGroundColor(
                    colorFromRgb15(0 as BYTE, 0x1f as BYTE, 0 as BYTE),
                );
            } else if shipPoints as ::core::ffi::c_int
                <= CalculateAllyPoints() as ::core::ffi::c_int
                    - CalculateEscortsPoints() as ::core::ffi::c_int
            {
                SetContextForeGroundColor(
                    colorFromRgb15(0x1f as BYTE, 0x1f as BYTE, 0 as BYTE),
                );
            } else {
                SetContextForeGroundColor(
                    colorFromRgb15(0x1f as BYTE, 0 as BYTE, 0 as BYTE),
                );
            }
            uqm_lua_checked!(font_DrawText(&raw mut t), ());
            r_0 = uqm_lua_checked!(font_GetTextRect(&raw mut t), ());
            t.baseline.x = (t.baseline.x as ::core::ffi::c_int
                - r_0.extent.width as ::core::ffi::c_int) as COORD;
            utf8StringCopy(
                &raw mut buf as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[UNICODE; 30]>(),
                GetStringAddress(
                    SetAbsStringTableIndex(
                        GameStrings,
                        (C2Rust_Unnamed_7::STARBASE_STRING_BASE.0 as ::core::ffi::c_int
                            + 9 as ::core::ffi::c_int) as COUNT,
                    ),
                ),
            );
            SetContextForeGroundColor(
                colorFromRgb15(0 as BYTE, 0 as BYTE, 0x1f as BYTE),
            );
            uqm_lua_checked!(font_DrawText(&raw mut t), ());
        }
    }
    UnbatchGraphics();
    SetContext(OldContext);
}
unsafe extern "C" fn ShowShipCrew(
    mut StarShipPtr: *mut SHIP_FRAGMENT,
    mut pRect: *const RECT,
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
    let mut buf: [UNICODE; 80] = [0; 80];
    let mut hTemplate: HFLEETINFO = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut TemplatePtr: *mut FLEET_INFO = ::core::ptr::null_mut::<FLEET_INFO>();
    let mut maxCrewLevel: COUNT = 0;
    let mut captain: STRING = ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>();
    if isPC(optWhichFonts).0 != 0 {
        SetContextFont(TinyFont);
    } else {
        SetContextFont(TinyFontBold);
    }
    hTemplate = GetStarShipFromIndex(
        &raw mut GlobData.Game_state.avail_race_q,
        (*StarShipPtr).race_id as COUNT,
    ) as HFLEETINFO;
    TemplatePtr = LockFleetInfo(&raw mut GlobData.Game_state.avail_race_q, hTemplate);
    maxCrewLevel = (if if GlobData.SIS_state.Extended as ::core::ffi::c_int != 0 {
        BOOLEAN::TRUE.0 as ::core::ffi::c_int
    } else {
        BOOLEAN::FALSE.0 as ::core::ffi::c_int
    } != 0
    {
        (*TemplatePtr).max_crew as ::core::ffi::c_int
    } else {
        (*TemplatePtr).crew_level as ::core::ffi::c_int
    }) as COUNT;
    UnlockLink(&raw mut GlobData.Game_state.avail_race_q, hTemplate);
    captain = SetAbsStringTableIndex(
        (*StarShipPtr).race_strings,
        (*StarShipPtr).captains_name_index as COUNT,
    );
    if (*StarShipPtr).crew_level as ::core::ffi::c_int
        == maxCrewLevel as ::core::ffi::c_int
    {
        sprintf(
            &raw mut buf as *mut ::core::ffi::c_char,
            b"%u\0".as_ptr() as *const ::core::ffi::c_char,
            (*StarShipPtr).crew_level as ::core::ffi::c_int,
        );
    } else if (*StarShipPtr).crew_level as ::core::ffi::c_int
        > maxCrewLevel as ::core::ffi::c_int
    {
        sprintf(
            &raw mut buf as *mut ::core::ffi::c_char,
            b"[%u/%u]\0".as_ptr() as *const ::core::ffi::c_char,
            (*StarShipPtr).crew_level as ::core::ffi::c_int
                - maxCrewLevel as ::core::ffi::c_int,
            maxCrewLevel as ::core::ffi::c_int,
        );
    } else if (*StarShipPtr).crew_level as ::core::ffi::c_int == 0 as ::core::ffi::c_int
    {
        utf8StringCopy(
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[UNICODE; 80]>(),
            GetStringAddress(
                SetAbsStringTableIndex(
                    GameStrings,
                    (C2Rust_Unnamed_7::STARBASE_STRING_BASE.0 as ::core::ffi::c_int
                        + 5 as ::core::ffi::c_int) as COUNT,
                ),
            ),
        );
    } else {
        sprintf(
            &raw mut buf as *mut ::core::ffi::c_char,
            b"%u/%u\0".as_ptr() as *const ::core::ffi::c_char,
            (*StarShipPtr).crew_level as ::core::ffi::c_int,
            maxCrewLevel as ::core::ffi::c_int,
        );
    }
    r = *pRect;
    t.baseline.x = (r.corner.x as ::core::ffi::c_int
        + (r.extent.width as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)) as COORD;
    t.baseline.y = (r.corner.y as ::core::ffi::c_int
        + r.extent.height as ::core::ffi::c_int
        - ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    t.align = TEXT_ALIGN::ALIGN_CENTER;
    t.pStr = &raw mut buf as *mut UNICODE;
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    if r.corner.y != 0 {
        r.corner.y = (t.baseline.y as ::core::ffi::c_int
            - ((6 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        r.extent.width = ((34 as ::core::ffi::c_int) << resolutionFactor) as COORD;
        r.extent.height = ((6 as ::core::ffi::c_int) << resolutionFactor) as COORD;
        SetContextForeGroundColor(colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE));
        uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
    }
    SetContextForeGroundColor(
        if (*StarShipPtr).crew_level as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            colorFromRgb15(0 as BYTE, 0x14 as BYTE, 0 as BYTE)
        } else {
            colorFromRgb15(0x12 as BYTE, 0 as BYTE, 0 as BYTE)
        },
    );
    uqm_lua_checked!(font_DrawText(&raw mut t), ());
    if optCaptainNames.0 == 0 {
        return;
    }
    r = *pRect;
    t.baseline.y = (r.corner.y as ::core::ffi::c_int
        + ((7 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    t.pStr = GetStringAddress(captain);
    t.CharCount = GetStringLength(captain);
    if r.corner.y == 0 {
        r.corner.y = (t.baseline.y as ::core::ffi::c_int
            - ((6 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        r.extent.width = ((34 as ::core::ffi::c_int) << resolutionFactor) as COORD;
        r.extent.height = ((6 as ::core::ffi::c_int) << resolutionFactor) as COORD;
        let mut shady: Color = buildColorRgba(
            0 as BYTE,
            0 as BYTE,
            0 as BYTE,
            0x80 as BYTE,
        );
        SetContextForeGroundColor(shady);
        uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
        SetContextFont(TinyFont);
        SetContextForeGroundColor(
            colorFromRgb15(0x16 as BYTE, 0xb as BYTE, 0x1f as BYTE),
        );
        uqm_lua_checked!(font_DrawText(&raw mut t), ());
    }
}
unsafe extern "C" fn ShowCombatShip(
    mut pMS: *mut MENU_STATE,
    mut which_window: COUNT,
    mut YankedStarShipPtr: *mut SHIP_FRAGMENT,
) {
    let mut i: COUNT = 0;
    let mut num_ships: COUNT = 0;
    let mut hStarShip: HSHIPFRAG = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut hNextShip: HSHIPFRAG = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut StarShipPtr: *mut SHIP_FRAGMENT = ::core::ptr::null_mut::<SHIP_FRAGMENT>();
    let mut ship_win_info: [C2Rust_Unnamed_9; 12] = [C2Rust_Unnamed_9 {
        StarShipPtr: ::core::ptr::null_mut::<SHIP_FRAGMENT>(),
        finished_s: POINT { x: 0, y: 0 },
        ship_s: STAMP {
            origin: POINT { x: 0, y: 0 },
            frame: ::core::ptr::null_mut::<FRAME_DESC>(),
        },
        lfdoor_s: STAMP {
            origin: POINT { x: 0, y: 0 },
            frame: ::core::ptr::null_mut::<FRAME_DESC>(),
        },
        rtdoor_s: STAMP {
            origin: POINT { x: 0, y: 0 },
            frame: ::core::ptr::null_mut::<FRAME_DESC>(),
        },
    }; 12];
    let mut pship_win_info: *mut C2Rust_Unnamed_9 = ::core::ptr::null_mut::<
        C2Rust_Unnamed_9,
    >();
    num_ships = 1 as COUNT;
    pship_win_info = (&raw mut ship_win_info as *mut C2Rust_Unnamed_9).offset(0isize);
    if !YankedStarShipPtr.is_null() {
        (*pship_win_info).StarShipPtr = YankedStarShipPtr;
        (*pship_win_info).lfdoor_s.origin.x = -((34 as ::core::ffi::c_int)
            << resolutionFactor >> 1 as ::core::ffi::c_int) as COORD;
        (*pship_win_info).rtdoor_s.origin.x = ((34 as ::core::ffi::c_int)
            << resolutionFactor >> 1 as ::core::ffi::c_int) as COORD;
        (*pship_win_info).lfdoor_s.origin.y = 0 as COORD;
        (*pship_win_info).rtdoor_s.origin.y = 0 as COORD;
        (*pship_win_info).lfdoor_s.frame = IncFrameIndex((*pMS).ModuleFrame);
        (*pship_win_info).rtdoor_s.frame = IncFrameIndex(
            (*pship_win_info).lfdoor_s.frame,
        );
        (*pship_win_info).ship_s.origin.x = (((34 as ::core::ffi::c_int)
            << resolutionFactor >> 1 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int)
            as COORD;
        (*pship_win_info).ship_s.origin.y = ((34 as ::core::ffi::c_int)
            << resolutionFactor >> 1 as ::core::ffi::c_int) as COORD;
        (*pship_win_info).ship_s.frame = (*YankedStarShipPtr).melee_icon;
        (*pship_win_info).finished_s.x = hangar_x_coords[(which_window
            as ::core::ffi::c_int
            % if if optWindowType == 1 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                4 as ::core::ffi::c_int
            } else {
                6 as ::core::ffi::c_int
            }) as usize];
        (*pship_win_info).finished_s.y = ((if if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            64 as ::core::ffi::c_int
        } else if if optWindowType == 0 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            71 as ::core::ffi::c_int
        } else {
            88 as ::core::ffi::c_int
        } << resolutionFactor)
            + (if if optWindowType == 1 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                44 as ::core::ffi::c_int
            } else if if optWindowType == 0 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                76 as ::core::ffi::c_int
            } else {
                84 as ::core::ffi::c_int
            } << resolutionFactor)
                * (which_window as ::core::ffi::c_int
                    / if if optWindowType == 1 as DWORD {
                        BOOLEAN::TRUE.0 as ::core::ffi::c_int
                    } else {
                        BOOLEAN::FALSE.0 as ::core::ffi::c_int
                    } != 0
                    {
                        4 as ::core::ffi::c_int
                    } else {
                        6 as ::core::ffi::c_int
                    })) as COORD;
    } else {
        if which_window as ::core::ffi::c_int
            == !(0 as ::core::ffi::c_int) as COUNT as ::core::ffi::c_int
        {
            hStarShip = GlobData.Game_state.built_ship_q.head as HSHIPFRAG;
            num_ships = CountLinks(&raw mut GlobData.Game_state.built_ship_q);
        } else {
            let mut hTailShip: HSHIPFRAG = ::core::ptr::null_mut::<
                ::core::ffi::c_void,
            >();
            hTailShip = GlobData.Game_state.built_ship_q.tail as HSHIPFRAG;
            RemoveQueue(&raw mut GlobData.Game_state.built_ship_q, hTailShip);
            hStarShip = GlobData.Game_state.built_ship_q.head as HSHIPFRAG;
            while !hStarShip.is_null() {
                StarShipPtr = LockShipFrag(
                    &raw mut GlobData.Game_state.built_ship_q,
                    hStarShip,
                );
                if (*StarShipPtr).index as ::core::ffi::c_int
                    > which_window as ::core::ffi::c_int
                {
                    UnlockLink(&raw mut GlobData.Game_state.built_ship_q, hStarShip);
                    break;
                } else {
                    hNextShip = (*StarShipPtr).succ as HSHIPFRAG;
                    UnlockLink(&raw mut GlobData.Game_state.built_ship_q, hStarShip);
                    hStarShip = hNextShip;
                }
            }
            InsertQueue(&raw mut GlobData.Game_state.built_ship_q, hTailShip, hStarShip);
            hStarShip = hTailShip;
            StarShipPtr = LockShipFrag(
                &raw mut GlobData.Game_state.built_ship_q,
                hStarShip,
            );
            (*StarShipPtr).index = which_window as BYTE;
            UnlockLink(&raw mut GlobData.Game_state.built_ship_q, hStarShip);
        }
        i = 0 as COUNT;
        while (i as ::core::ffi::c_int) < num_ships as ::core::ffi::c_int {
            StarShipPtr = LockShipFrag(
                &raw mut GlobData.Game_state.built_ship_q,
                hStarShip,
            );
            hNextShip = (*StarShipPtr).succ as HSHIPFRAG;
            (*pship_win_info).StarShipPtr = StarShipPtr;
            (*pship_win_info).lfdoor_s.origin.x = -((1 as ::core::ffi::c_int)
                << resolutionFactor) as COORD;
            (*pship_win_info).rtdoor_s.origin.x = ((1 as ::core::ffi::c_int)
                << resolutionFactor) as COORD;
            (*pship_win_info).lfdoor_s.origin.y = 0 as COORD;
            (*pship_win_info).rtdoor_s.origin.y = 0 as COORD;
            (*pship_win_info).lfdoor_s.frame = IncFrameIndex((*pMS).ModuleFrame);
            (*pship_win_info).rtdoor_s.frame = IncFrameIndex(
                (*pship_win_info).lfdoor_s.frame,
            );
            (*pship_win_info).ship_s.origin.x = (((34 as ::core::ffi::c_int)
                << resolutionFactor >> 1 as ::core::ffi::c_int)
                + 1 as ::core::ffi::c_int) as COORD;
            (*pship_win_info).ship_s.origin.y = ((34 as ::core::ffi::c_int)
                << resolutionFactor >> 1 as ::core::ffi::c_int) as COORD;
            (*pship_win_info).ship_s.frame = (*StarShipPtr).melee_icon;
            which_window = (*StarShipPtr).index as COUNT;
            (*pship_win_info).finished_s.x = hangar_x_coords[(which_window
                as ::core::ffi::c_int
                % if if optWindowType == 1 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    4 as ::core::ffi::c_int
                } else {
                    6 as ::core::ffi::c_int
                }) as usize];
            (*pship_win_info).finished_s.y = ((if if optWindowType == 1 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                64 as ::core::ffi::c_int
            } else if if optWindowType == 0 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                71 as ::core::ffi::c_int
            } else {
                88 as ::core::ffi::c_int
            } << resolutionFactor)
                + (if if optWindowType == 1 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    44 as ::core::ffi::c_int
                } else if if optWindowType == 0 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    76 as ::core::ffi::c_int
                } else {
                    84 as ::core::ffi::c_int
                } << resolutionFactor)
                    * (which_window as ::core::ffi::c_int
                        / if if optWindowType == 1 as DWORD {
                            BOOLEAN::TRUE.0 as ::core::ffi::c_int
                        } else {
                            BOOLEAN::FALSE.0 as ::core::ffi::c_int
                        } != 0
                        {
                            4 as ::core::ffi::c_int
                        } else {
                            6 as ::core::ffi::c_int
                        })) as COORD;
            pship_win_info = pship_win_info.offset(1);
            UnlockLink(&raw mut GlobData.Game_state.built_ship_q, hStarShip);
            hStarShip = hNextShip;
            i = i.wrapping_add(1);
        }
    }
    if num_ships != 0 {
        let mut AllDoorsFinished: BOOLEAN = BOOLEAN::FALSE;
        let mut TimeIn: DWORD = 0;
        let mut r: RECT = RECT {
            corner: POINT { x: 0, y: 0 },
            extent: EXTENT { width: 0, height: 0 },
        };
        let mut OldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
        let mut OldClipRect: RECT = RECT {
            corner: POINT { x: 0, y: 0 },
            extent: EXTENT { width: 0, height: 0 },
        };
        let mut j: ::core::ffi::c_int = 0;
        AllDoorsFinished = BOOLEAN::FALSE;
        r.corner.y = 0 as COORD;
        r.corner.x = r.corner.y;
        r.extent.width = ((34 as ::core::ffi::c_int) << resolutionFactor) as COORD;
        r.extent.height = (((34 as ::core::ffi::c_int) << resolutionFactor)
            + ((6 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        FlushInput();
        TimeIn = GetTimeCounter() as DWORD;
        j = 0 as ::core::ffi::c_int;
        while j
            < ((34 as ::core::ffi::c_int) << resolutionFactor >> 1 as ::core::ffi::c_int)
                + 1 as ::core::ffi::c_int && AllDoorsFinished.0 == 0
        {
            uqm_lua_checked!(SleepThreadUntil(
                TimeIn
                    .wrapping_add(
                        (840 as ::core::ffi::c_int
                            / ((24 as ::core::ffi::c_int) << resolutionFactor))
                            as TimeCount,
                    ),
            ), ());
            TimeIn = GetTimeCounter() as DWORD;
            if uqm_lua_checked!(AnyButtonPress(BOOLEAN::FALSE), ()).0 != 0 {
                if !YankedStarShipPtr.is_null() {
                    ship_win_info[0usize].lfdoor_s.origin.x = 0 as COORD;
                    ship_win_info[0usize].rtdoor_s.origin.x = 0 as COORD;
                }
                AllDoorsFinished = BOOLEAN::TRUE;
            }
            OldContext = SetContext(SpaceContext);
            GetContextClipRect(&raw mut OldClipRect);
            SetContextBackGroundColor(colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE));
            BatchGraphics();
            pship_win_info = (&raw mut ship_win_info as *mut C2Rust_Unnamed_9)
                .offset(0isize);
            i = 0 as COUNT;
            while (i as ::core::ffi::c_int) < num_ships as ::core::ffi::c_int {
                let mut ClipRect: RECT = RECT {
                    corner: POINT { x: 0, y: 0 },
                    extent: EXTENT { width: 0, height: 0 },
                };
                ClipRect.corner.x = (((6 as ::core::ffi::c_int) << resolutionFactor)
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
                    } + (*pship_win_info).finished_s.x as ::core::ffi::c_int) as COORD;
                ClipRect.corner.y = (((9 as ::core::ffi::c_int) << resolutionFactor)
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
                    } + (*pship_win_info).finished_s.y as ::core::ffi::c_int) as COORD;
                ClipRect.extent.width = ((34 as ::core::ffi::c_int) << resolutionFactor)
                    as COORD;
                ClipRect.extent.height = (((34 as ::core::ffi::c_int)
                    << resolutionFactor)
                    + ((6 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
                SetContextClipRect(&raw mut ClipRect);
                uqm_lua_checked!(ClearDrawable(), ());
                uqm_lua_checked!(DrawStamp(&raw mut (*pship_win_info).ship_s), ());
                uqm_lua_checked!(ShowShipCrew((*pship_win_info).StarShipPtr, &raw mut r), ());
                if AllDoorsFinished.0 == 0 || !YankedStarShipPtr.is_null() {
                    uqm_lua_checked!(DrawStamp(&raw mut (*pship_win_info).lfdoor_s), ());
                    uqm_lua_checked!(DrawStamp(&raw mut (*pship_win_info).rtdoor_s), ());
                    if !YankedStarShipPtr.is_null() {
                        (*pship_win_info).lfdoor_s.origin.x += 1;
                        (*pship_win_info).rtdoor_s.origin.x -= 1;
                    } else {
                        (*pship_win_info).lfdoor_s.origin.x -= 1;
                        (*pship_win_info).rtdoor_s.origin.x += 1;
                    }
                }
                pship_win_info = pship_win_info.offset(1);
                i = i.wrapping_add(1);
            }
            SetContextClipRect(&raw mut OldClipRect);
            uqm_lua_checked!(animatePowerLines(::core::ptr::null_mut::<MENU_STATE>()), ());
            UnbatchGraphics();
            SetContext(OldContext);
            j += 1;
        }
    }
}
unsafe extern "C" fn CrewTransaction(mut crew_delta: SIZE) {
    if crew_delta != 0 {
        let mut crew_bought: SIZE = 0;
        crew_bought = ((((uqm_lua_checked!(getGameStateUint(
            b"CREW_PURCHASED1\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) as BYTE as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as UWORD
            as ::core::ffi::c_int
            | uqm_lua_checked!(getGameStateUint(
                b"CREW_PURCHASED0\0".as_ptr() as *const ::core::ffi::c_char,
            ), ()) as BYTE as ::core::ffi::c_int) as SIZE as ::core::ffi::c_int
            + crew_delta as ::core::ffi::c_int) as SIZE;
        if (crew_bought as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
            if (crew_delta as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                crew_bought = 0 as SIZE;
            } else {
                crew_bought = 0x7fff as SIZE;
            }
        } else if crew_delta as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
            if crew_bought as ::core::ffi::c_int >= 1000 as ::core::ffi::c_int
                && (crew_bought as ::core::ffi::c_int - crew_delta as ::core::ffi::c_int)
                    < 1000 as ::core::ffi::c_int
            {
                GlobData.Game_state.CrewCost = (GlobData.Game_state.CrewCost
                    as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as BYTE;
                uqm_lua_checked!(DrawMenuStateStrings(
                    C2Rust_Unnamed_6::PM_CREW.0 as ::core::ffi::c_int as BYTE,
                    C2Rust_Unnamed_10::SHIPYARD_CREW.0 as ::core::ffi::c_int as SWORD,
                ), ());
            }
        } else if (crew_bought as ::core::ffi::c_int) < 1000 as ::core::ffi::c_int
            && crew_bought as ::core::ffi::c_int - crew_delta as ::core::ffi::c_int
                >= 1000 as ::core::ffi::c_int
        {
            GlobData.Game_state.CrewCost = (GlobData.Game_state.CrewCost
                as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as BYTE;
            uqm_lua_checked!(DrawMenuStateStrings(
                C2Rust_Unnamed_6::PM_CREW.0 as ::core::ffi::c_int as BYTE,
                C2Rust_Unnamed_10::SHIPYARD_CREW.0 as ::core::ffi::c_int as SWORD,
            ), ());
        }
        if CheckAlliance(RACE_ID::SHOFIXTI_SHIP) as ::core::ffi::c_int
            != C2Rust_Unnamed_1::GOOD_GUY.0 as ::core::ffi::c_int
        {
            uqm_lua_checked!(setGameStateUint(
                b"CREW_PURCHASED0\0".as_ptr() as *const ::core::ffi::c_char,
                crew_bought as UWORD as BYTE as DWORD,
            ), ());
            uqm_lua_checked!(setGameStateUint(
                b"CREW_PURCHASED1\0".as_ptr() as *const ::core::ffi::c_char,
                (crew_bought as UWORD as ::core::ffi::c_int >> 8 as ::core::ffi::c_int)
                    as BYTE as DWORD,
            ), ());
            uqm_lua_checked!(showRemainingCrew(), ());
        }
    }
}
unsafe extern "C" fn DMS_FlashFlagShip() {
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
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
    if optWhichMenu != 0x2 as ::core::ffi::c_int {
        r.extent.height = (((63 as ::core::ffi::c_int) << resolutionFactor)
            - (if if optWindowType == 1 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                2 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } << resolutionFactor)) as COORD;
    } else {
        r.extent.height = (((74 as ::core::ffi::c_int) << resolutionFactor)
            - (if if optWindowType == 0 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                9 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } << resolutionFactor)
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
                } << resolutionFactor) - ((3 as ::core::ffi::c_int) << resolutionFactor)
            } else {
                0 as ::core::ffi::c_int
            }) as COORD;
    }
    uqm_lua_checked!(SetFlashRect(
        &raw mut r,
        BOOLEAN(
            (optWhichMenu == 0x2 as ::core::ffi::c_int) as ::core::ffi::c_int
                as ::core::ffi::c_uint,
        ),
    ), ());
}
unsafe extern "C" fn DMS_GetEscortShipRect(mut rOut: *mut RECT, mut slotNr: BYTE) {
    let mut row: BYTE = (slotNr as ::core::ffi::c_int
        / if if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            4 as ::core::ffi::c_int
        } else {
            6 as ::core::ffi::c_int
        }) as BYTE;
    let mut col: BYTE = (slotNr as ::core::ffi::c_int
        % if if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            4 as ::core::ffi::c_int
        } else {
            6 as ::core::ffi::c_int
        }) as BYTE;
    (*rOut).corner.x = hangar_x_coords[col as usize];
    (*rOut).corner.y = ((if if optWindowType == 1 as DWORD {
        BOOLEAN::TRUE.0 as ::core::ffi::c_int
    } else {
        BOOLEAN::FALSE.0 as ::core::ffi::c_int
    } != 0
    {
        64 as ::core::ffi::c_int
    } else if if optWindowType == 0 as DWORD {
        BOOLEAN::TRUE.0 as ::core::ffi::c_int
    } else {
        BOOLEAN::FALSE.0 as ::core::ffi::c_int
    } != 0
    {
        71 as ::core::ffi::c_int
    } else {
        88 as ::core::ffi::c_int
    } << resolutionFactor)
        + (if if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            44 as ::core::ffi::c_int
        } else if if optWindowType == 0 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            76 as ::core::ffi::c_int
        } else {
            84 as ::core::ffi::c_int
        } << resolutionFactor) * row as ::core::ffi::c_int) as COORD;
    (*rOut).extent.width = ((34 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    (*rOut).extent.height = (((34 as ::core::ffi::c_int) << resolutionFactor)
        + ((6 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
}
unsafe extern "C" fn DMS_FlashEscortShip(mut slotNr: BYTE) {
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    DMS_GetEscortShipRect(&raw mut r, slotNr);
    uqm_lua_checked!(SetFlashRect(
        &raw mut r,
        BOOLEAN(
            (optWhichMenu == 0x2 as ::core::ffi::c_int) as ::core::ffi::c_int
                as ::core::ffi::c_uint,
        ),
    ), ());
}
unsafe extern "C" fn DMS_FlashFlagShipCrewCount() {
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    SetContext(StatusContext);
    GetGaugeRect(&raw mut r, BOOLEAN::TRUE);
    uqm_lua_checked!(SetFlashRect(&raw mut r, BOOLEAN::FALSE), ());
    SetContext(SpaceContext);
}
unsafe extern "C" fn DMS_FlashEscortShipCrewCount(mut slotNr: BYTE) {
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut row: BYTE = (slotNr as ::core::ffi::c_int
        / if if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            4 as ::core::ffi::c_int
        } else {
            6 as ::core::ffi::c_int
        }) as BYTE;
    let mut col: BYTE = (slotNr as ::core::ffi::c_int
        % if if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            4 as ::core::ffi::c_int
        } else {
            6 as ::core::ffi::c_int
        }) as BYTE;
    r.corner.x = hangar_x_coords[col as usize];
    r.corner.y = ((if if optWindowType == 1 as DWORD {
        BOOLEAN::TRUE.0 as ::core::ffi::c_int
    } else {
        BOOLEAN::FALSE.0 as ::core::ffi::c_int
    } != 0
    {
        64 as ::core::ffi::c_int
    } else if if optWindowType == 0 as DWORD {
        BOOLEAN::TRUE.0 as ::core::ffi::c_int
    } else {
        BOOLEAN::FALSE.0 as ::core::ffi::c_int
    } != 0
    {
        71 as ::core::ffi::c_int
    } else {
        88 as ::core::ffi::c_int
    } << resolutionFactor)
        + (if if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            44 as ::core::ffi::c_int
        } else if if optWindowType == 0 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            76 as ::core::ffi::c_int
        } else {
            84 as ::core::ffi::c_int
        } << resolutionFactor) * row as ::core::ffi::c_int
        + (((34 as ::core::ffi::c_int) << resolutionFactor)
            + ((6 as ::core::ffi::c_int) << resolutionFactor)
            - ((6 as ::core::ffi::c_int) << resolutionFactor))) as COORD;
    r.extent.width = ((34 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    r.extent.height = ((5 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    SetContext(SpaceContext);
    uqm_lua_checked!(SetFlashRect(&raw mut r, BOOLEAN::FALSE), ());
}
unsafe extern "C" fn DMS_FlashActiveShip(mut pMS: *mut MENU_STATE) {
    if ((*pMS).CurState as ::core::ffi::c_int
        >> 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE != 0
    {
        uqm_lua_checked!(DMS_FlashFlagShip(), ());
    } else {
        uqm_lua_checked!(DMS_FlashEscortShip((*pMS).CurState), ());
    };
}
unsafe extern "C" fn DMS_SetMode(mut pMS: *mut MENU_STATE, mut mode: DMS_Mode) {
    match mode {
        DMS_Mode::DMS_Mode_navigate => {
            SetMenuSounds(
                (((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int)
                    as MENU_SOUND_FLAGS as ::core::ffi::c_int
                    | ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
                        as MENU_SOUND_FLAGS as ::core::ffi::c_int
                    | ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                        as MENU_SOUND_FLAGS as ::core::ffi::c_int
                    | ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                        as MENU_SOUND_FLAGS as ::core::ffi::c_int) as MENU_SOUND_FLAGS,
                ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                    as MENU_SOUND_FLAGS,
            );
            uqm_lua_checked!(DMS_FlashActiveShip(pMS), ());
        }
        DMS_Mode::DMS_Mode_addEscort => {
            SetMenuSounds(
                (((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int)
                    as MENU_SOUND_FLAGS as ::core::ffi::c_int
                    | ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
                        as MENU_SOUND_FLAGS as ::core::ffi::c_int
                    | ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                        as MENU_SOUND_FLAGS as ::core::ffi::c_int
                    | ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                        as MENU_SOUND_FLAGS as ::core::ffi::c_int) as MENU_SOUND_FLAGS,
                ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                    as MENU_SOUND_FLAGS,
            );
            uqm_lua_checked!(SetFlashRect(
                if if optWindowType == 0 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    ::core::ptr::with_exposed_provenance_mut::<
                        RECT,
                    >(!(2 as ::core::ffi::c_long) as usize)
                } else {
                    ::core::ptr::with_exposed_provenance_mut::<
                        RECT,
                    >(!(0 as ::core::ffi::c_long) as usize)
                },
                BOOLEAN::FALSE,
            ), ());
        }
        DMS_Mode::DMS_Mode_editCrew => {
            SetMenuSounds(
                (((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int)
                    as MENU_SOUND_FLAGS as ::core::ffi::c_int
                    | ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
                        as MENU_SOUND_FLAGS as ::core::ffi::c_int
                    | ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                        as MENU_SOUND_FLAGS as ::core::ffi::c_int
                    | ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                        as MENU_SOUND_FLAGS as ::core::ffi::c_int) as MENU_SOUND_FLAGS,
                (((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int)
                    as MENU_SOUND_FLAGS as ::core::ffi::c_int
                    | ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
                        as MENU_SOUND_FLAGS as ::core::ffi::c_int
                    | (((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                        as MENU_SOUND_FLAGS as ::core::ffi::c_int
                        | ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int)
                            as MENU_SOUND_FLAGS as ::core::ffi::c_int))
                    as MENU_SOUND_FLAGS,
            );
            if ((*pMS).CurState as ::core::ffi::c_int
                >> 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE != 0
            {
                uqm_lua_checked!(DMS_FlashFlagShipCrewCount(), ());
            } else {
                uqm_lua_checked!(DMS_FlashEscortShipCrewCount((*pMS).CurState), ());
            }
        }
        DMS_Mode::DMS_Mode_exit => {
            SetMenuSounds(
                (((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int)
                    as MENU_SOUND_FLAGS as ::core::ffi::c_int
                    | ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
                        as MENU_SOUND_FLAGS as ::core::ffi::c_int
                    | ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                        as MENU_SOUND_FLAGS as ::core::ffi::c_int
                    | ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                        as MENU_SOUND_FLAGS as ::core::ffi::c_int) as MENU_SOUND_FLAGS,
                ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                    as MENU_SOUND_FLAGS,
            );
            uqm_lua_checked!(SetFlashRect(
                ::core::ptr::with_exposed_provenance_mut::<
                    RECT,
                >(!(0 as ::core::ffi::c_long) as usize),
                BOOLEAN::FALSE,
            ), ());
        }
        _ => {}
    };
}
unsafe extern "C" fn DMS_SpinShip(
    mut pMS: *mut MENU_STATE,
    mut hStarShip: HSHIPFRAG,
) -> BOOLEAN {
    let mut hSpinShip: HFLEETINFO = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut OldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    let mut OldClipRect: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    if ((*pMS).CurState as ::core::ffi::c_int
        >> 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE
        as ::core::ffi::c_int != 0 as ::core::ffi::c_int
    {
        return BOOLEAN::FALSE;
    }
    if hStarShip.is_null() as ::core::ffi::c_int
        == ((*pMS).delta_item as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int == 0)
            as ::core::ffi::c_int
    {
        return BOOLEAN::FALSE;
    }
    if hStarShip.is_null() && stowed_q.0 == 0 {
        hSpinShip = GetAvailableRaceFromIndex((*pMS).delta_item as UWORD as BYTE);
        if hSpinShip.is_null() {
            return BOOLEAN::FALSE;
        }
    } else if hStarShip.is_null() && stowed_q.0 != 0 {
        let mut hStowedShip: HSHIPFRAG = GetStarShipFromIndex(
            &raw mut GlobData.Game_state.stowed_ship_q,
            (*pMS).delta_item as UWORD as BYTE as COUNT,
        );
        let mut FragPtr: *mut SHIP_FRAGMENT = LockShipFrag(
            &raw mut GlobData.Game_state.stowed_ship_q,
            hStowedShip,
        );
        hSpinShip = GetStarShipFromIndex(
            &raw mut GlobData.Game_state.avail_race_q,
            (*FragPtr).race_id as COUNT,
        ) as HFLEETINFO;
        UnlockLink(&raw mut GlobData.Game_state.stowed_ship_q, hStowedShip);
    } else {
        let mut FragPtr_0: *mut SHIP_FRAGMENT = LockShipFrag(
            &raw mut GlobData.Game_state.built_ship_q,
            hStarShip,
        );
        hSpinShip = GetStarShipFromIndex(
            &raw mut GlobData.Game_state.avail_race_q,
            (*FragPtr_0).race_id as COUNT,
        ) as HFLEETINFO;
        UnlockLink(&raw mut GlobData.Game_state.built_ship_q, hStarShip);
    }
    uqm_lua_checked!(SetFlashRect(::core::ptr::null::<RECT>(), BOOLEAN::FALSE), BOOLEAN(0));
    OldContext = SetContext(ScreenContext);
    GetContextClipRect(&raw mut OldClipRect);
    uqm_lua_checked!(SpinStarShip(pMS, hSpinShip), BOOLEAN(0));
    SetContextClipRect(&raw mut OldClipRect);
    SetContext(OldContext);
    return BOOLEAN::TRUE;
}
unsafe extern "C" fn DMS_HireFlagShipCrew() -> SIZE {
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut crew_bought: SIZE = 0;
    crew_bought = (((uqm_lua_checked!(getGameStateUint(
        b"CREW_PURCHASED1\0".as_ptr() as *const ::core::ffi::c_char,
    ), 0) as BYTE as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as UWORD
        as ::core::ffi::c_int
        | uqm_lua_checked!(getGameStateUint(b"CREW_PURCHASED0\0".as_ptr() as *const ::core::ffi::c_char), 0)
            as BYTE as ::core::ffi::c_int) as SIZE;
    if GetCPodCapacity(&raw mut r.corner) as ::core::ffi::c_int
        <= GetCrewCount() as ::core::ffi::c_int
    {
        return 0 as SIZE;
    }
    if GlobData.SIS_state.ResUnits < GlobData.Game_state.CrewCost as DWORD {
        return 0 as SIZE;
    }
    if if if GlobData.SIS_state.Difficulty as ::core::ffi::c_int != 0 {
        GlobData.SIS_state.Difficulty as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    } == 2 as ::core::ffi::c_int
    {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    } != 0 && crew_bought as ::core::ffi::c_int >= 2000 as ::core::ffi::c_int
        && CheckAlliance(RACE_ID::SHOFIXTI_SHIP) as ::core::ffi::c_int
            != C2Rust_Unnamed_1::GOOD_GUY.0 as ::core::ffi::c_int
    {
        return 0 as SIZE;
    }
    r.extent.width = ((1 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    r.extent.height = r.extent.width;
    uqm_lua_checked!(DrawFilledRectangle(&raw mut r), 0);
    uqm_lua_checked!(PreUpdateFlashRect(), 0);
    uqm_lua_checked!(DeltaSISGauges(
        1 as SIZE,
        0 as SDWORD,
        -(GlobData.Game_state.CrewCost as ::core::ffi::c_int),
    ), 0);
    uqm_lua_checked!(PostUpdateFlashRect(), 0);
    return 1 as SIZE;
}
unsafe extern "C" fn DMS_DismissFlagShipCrew() -> SIZE {
    let mut crew_bought: SIZE = 0;
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    if GetCrewCount() as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        return 0 as SIZE;
    }
    crew_bought = (((uqm_lua_checked!(getGameStateUint(
        b"CREW_PURCHASED1\0".as_ptr() as *const ::core::ffi::c_char,
    ), 0) as BYTE as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as UWORD
        as ::core::ffi::c_int
        | uqm_lua_checked!(getGameStateUint(b"CREW_PURCHASED0\0".as_ptr() as *const ::core::ffi::c_char), 0)
            as BYTE as ::core::ffi::c_int) as SIZE;
    uqm_lua_checked!(PreUpdateFlashRect(), 0);
    uqm_lua_checked!(DeltaSISGauges(
        -1 as SIZE,
        0 as SDWORD,
        GlobData.Game_state.CrewCost as ::core::ffi::c_int
            - if crew_bought as ::core::ffi::c_int == 1000 as ::core::ffi::c_int {
                2 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            },
    ), 0);
    uqm_lua_checked!(PostUpdateFlashRect(), 0);
    GetCPodCapacity(&raw mut r.corner);
    r.extent.width = ((1 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    r.extent.height = r.extent.width;
    SetContextForeGroundColor(colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE));
    uqm_lua_checked!(DrawFilledRectangle(&raw mut r), 0);
    return -1 as SIZE;
}
unsafe extern "C" fn DMS_HireEscortShipCrew(
    mut StarShipPtr: *mut SHIP_FRAGMENT,
) -> SIZE {
    let mut templateMaxCrew: COUNT = 0;
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut crew_bought: SIZE = 0;
    let mut crew_level: COUNT = (if (*StarShipPtr).crew_level as ::core::ffi::c_int
        > (*StarShipPtr).max_crew as ::core::ffi::c_int
    {
        (*StarShipPtr).crew_level as ::core::ffi::c_int
            - (*StarShipPtr).max_crew as ::core::ffi::c_int
    } else {
        (*StarShipPtr).crew_level as ::core::ffi::c_int
    }) as COUNT;
    crew_bought = (((uqm_lua_checked!(getGameStateUint(
        b"CREW_PURCHASED1\0".as_ptr() as *const ::core::ffi::c_char,
    ), 0) as BYTE as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as UWORD
        as ::core::ffi::c_int
        | uqm_lua_checked!(getGameStateUint(b"CREW_PURCHASED0\0".as_ptr() as *const ::core::ffi::c_char), 0)
            as BYTE as ::core::ffi::c_int) as SIZE;
    let mut hTemplate: HFLEETINFO = GetStarShipFromIndex(
        &raw mut GlobData.Game_state.avail_race_q,
        (*StarShipPtr).race_id as COUNT,
    );
    let mut TemplatePtr: *mut FLEET_INFO = LockFleetInfo(
        &raw mut GlobData.Game_state.avail_race_q,
        hTemplate,
    );
    templateMaxCrew = (if if GlobData.SIS_state.Extended as ::core::ffi::c_int != 0 {
        BOOLEAN::TRUE.0 as ::core::ffi::c_int
    } else {
        BOOLEAN::FALSE.0 as ::core::ffi::c_int
    } != 0
        && if if GlobData.SIS_state.Difficulty as ::core::ffi::c_int != 0 {
            GlobData.SIS_state.Difficulty as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        } == 2 as ::core::ffi::c_int
        {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        } == 0
    {
        (*TemplatePtr).max_crew as ::core::ffi::c_int
    } else {
        (*TemplatePtr).crew_level as ::core::ffi::c_int
    }) as COUNT;
    UnlockLink(&raw mut GlobData.Game_state.avail_race_q, hTemplate);
    if GlobData.SIS_state.ResUnits < GlobData.Game_state.CrewCost as DWORD {
        return 0 as SIZE;
    }
    if crew_level as ::core::ffi::c_int >= (*StarShipPtr).max_crew as ::core::ffi::c_int
    {
        return 0 as SIZE;
    }
    if crew_level as ::core::ffi::c_int >= templateMaxCrew as ::core::ffi::c_int {
        return 0 as SIZE;
    }
    if if if GlobData.SIS_state.Difficulty as ::core::ffi::c_int != 0 {
        GlobData.SIS_state.Difficulty as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    } == 2 as ::core::ffi::c_int
    {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    } != 0 && crew_bought as ::core::ffi::c_int >= 2000 as ::core::ffi::c_int
        && CheckAlliance(RACE_ID::SHOFIXTI_SHIP) as ::core::ffi::c_int
            != C2Rust_Unnamed_1::GOOD_GUY.0 as ::core::ffi::c_int
    {
        return 0 as SIZE;
    }
    if crew_level as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
        uqm_lua_checked!(DeltaSISGauges(
            0 as SIZE,
            0 as SDWORD,
            -(GlobData.Game_state.CrewCost as ::core::ffi::c_int),
        ), 0);
    } else {
        uqm_lua_checked!(DeltaSISGauges(
            0 as SIZE,
            0 as SDWORD,
            -(ShipCost((*StarShipPtr).race_id) as ::core::ffi::c_int),
        ), 0);
        uqm_lua_checked!(showRemainingPoints(0 as ::core::ffi::c_int), 0);
    }
    (*StarShipPtr).crew_level = (*StarShipPtr).crew_level.wrapping_add(1);
    uqm_lua_checked!(PreUpdateFlashRect(), 0);
    DMS_GetEscortShipRect(&raw mut r, (*StarShipPtr).index);
    uqm_lua_checked!(ShowShipCrew(StarShipPtr, &raw mut r), 0);
    uqm_lua_checked!(PostUpdateFlashRect(), 0);
    return 1 as SIZE;
}
unsafe extern "C" fn DMS_DismissEscortShipCrew(
    mut StarShipPtr: *mut SHIP_FRAGMENT,
) -> SIZE {
    let mut crew_delta: SIZE = 0 as SIZE;
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut crew_level: COUNT = (if (*StarShipPtr).crew_level as ::core::ffi::c_int
        > (*StarShipPtr).max_crew as ::core::ffi::c_int
    {
        (*StarShipPtr).crew_level as ::core::ffi::c_int
            - (*StarShipPtr).max_crew as ::core::ffi::c_int
    } else {
        (*StarShipPtr).crew_level as ::core::ffi::c_int
    }) as COUNT;
    if crew_level as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
        if crew_level as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
            let mut crew_bought: SIZE = (((uqm_lua_checked!(getGameStateUint(
                b"CREW_PURCHASED1\0".as_ptr() as *const ::core::ffi::c_char,
            ), 0) as BYTE as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as UWORD
                as ::core::ffi::c_int
                | uqm_lua_checked!(getGameStateUint(
                    b"CREW_PURCHASED0\0".as_ptr() as *const ::core::ffi::c_char,
                ), 0) as BYTE as ::core::ffi::c_int) as SIZE;
            uqm_lua_checked!(DeltaSISGauges(
                0 as SIZE,
                0 as SDWORD,
                GlobData.Game_state.CrewCost as ::core::ffi::c_int
                    - if crew_bought as ::core::ffi::c_int == 1000 as ::core::ffi::c_int
                    {
                        2 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    },
            ), 0);
        } else {
            let mut hTemplate: HFLEETINFO = GetStarShipFromIndex(
                &raw mut GlobData.Game_state.avail_race_q,
                (*StarShipPtr).race_id as COUNT,
            );
            uqm_lua_checked!(DeltaSISGauges(
                0 as SIZE,
                0 as SDWORD,
                ShipCost((*StarShipPtr).race_id) as ::core::ffi::c_int,
            ), 0);
            uqm_lua_checked!(showRemainingPoints(ShipPoints(hTemplate) as ::core::ffi::c_int), 0);
        }
        crew_delta = -1 as SIZE;
        (*StarShipPtr).crew_level = (*StarShipPtr).crew_level.wrapping_sub(1);
        if (*StarShipPtr).crew_level as ::core::ffi::c_int
            == (*StarShipPtr).max_crew as ::core::ffi::c_int
        {
            (*StarShipPtr).crew_level = 0 as COUNT;
        }
    } else {
        uqm_lua_checked!(PlayMenuSound(MENU_SOUND_EFFECT::MENU_SOUND_FAILURE), 0);
    }
    uqm_lua_checked!(PreUpdateFlashRect(), 0);
    DMS_GetEscortShipRect(&raw mut r, (*StarShipPtr).index);
    uqm_lua_checked!(ShowShipCrew(StarShipPtr, &raw mut r), 0);
    uqm_lua_checked!(PostUpdateFlashRect(), 0);
    return crew_delta;
}
unsafe extern "C" fn DMS_ModifyCrew(
    mut pMS: *mut MENU_STATE,
    mut hStarShip: HSHIPFRAG,
    mut dy: SBYTE,
) {
    let mut crew_delta: SIZE = 0 as SIZE;
    let mut StarShipPtr: *mut SHIP_FRAGMENT = ::core::ptr::null_mut::<SHIP_FRAGMENT>();
    let mut r#loop: COUNT = 0;
    let mut DoLoop: COUNT = 1 as COUNT;
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    DoLoop = abs(dy as ::core::ffi::c_int) as COUNT;
    if abs(dy as ::core::ffi::c_int) == 50 as ::core::ffi::c_int {
        uqm_lua_checked!(PlayMenuSound(MENU_SOUND_EFFECT::MENU_SOUND_INVOKED), ());
    }
    if !hStarShip.is_null() {
        StarShipPtr = LockShipFrag(&raw mut GlobData.Game_state.built_ship_q, hStarShip);
    }
    if hStarShip.is_null() {
        if dy as ::core::ffi::c_int == -50 as ::core::ffi::c_int {
            DoLoop = (GetCPodCapacity(&raw mut r.corner) as ::core::ffi::c_int
                - GetCrewCount() as ::core::ffi::c_int) as COUNT;
        } else if dy as ::core::ffi::c_int == 50 as ::core::ffi::c_int {
            DoLoop = GetCrewCount();
        }
        r#loop = 0 as COUNT;
        while (r#loop as ::core::ffi::c_int) < DoLoop as ::core::ffi::c_int {
            if (dy as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                crew_delta = uqm_lua_checked!(DMS_HireFlagShipCrew(), ());
            } else {
                crew_delta = uqm_lua_checked!(DMS_DismissFlagShipCrew(), ());
            }
            if crew_delta as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                uqm_lua_checked!(DMS_FlashFlagShipCrewCount(), ());
            }
            if crew_delta as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                break;
            }
            uqm_lua_checked!(CrewTransaction(crew_delta), ());
            uqm_lua_checked!(animatePowerLines(::core::ptr::null_mut::<MENU_STATE>()), ());
            r#loop = r#loop.wrapping_add(1);
        }
    } else {
        let mut crew_level: COUNT = (if (*StarShipPtr).crew_level as ::core::ffi::c_int
            > (*StarShipPtr).max_crew as ::core::ffi::c_int
        {
            (*StarShipPtr).crew_level as ::core::ffi::c_int
                - (*StarShipPtr).max_crew as ::core::ffi::c_int
        } else {
            (*StarShipPtr).crew_level as ::core::ffi::c_int
        }) as COUNT;
        if dy as ::core::ffi::c_int == -50 as ::core::ffi::c_int {
            DoLoop = ((*StarShipPtr).max_crew as ::core::ffi::c_int
                - crew_level as ::core::ffi::c_int) as COUNT;
        } else if dy as ::core::ffi::c_int == 50 as ::core::ffi::c_int {
            DoLoop = crew_level;
        }
        r#loop = 0 as COUNT;
        while (r#loop as ::core::ffi::c_int) < DoLoop as ::core::ffi::c_int {
            if (dy as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                crew_delta = uqm_lua_checked!(DMS_HireEscortShipCrew(StarShipPtr), ());
            } else {
                crew_delta = uqm_lua_checked!(DMS_DismissEscortShipCrew(StarShipPtr), ());
            }
            if crew_delta as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                uqm_lua_checked!(DMS_FlashEscortShipCrewCount((*StarShipPtr).index), ());
            }
            if crew_delta as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                break;
            }
            if if if GlobData.SIS_state.Difficulty as ::core::ffi::c_int != 0 {
                GlobData.SIS_state.Difficulty as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } == 2 as ::core::ffi::c_int
            {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } == 0
                || (dy as ::core::ffi::c_int > 0 as ::core::ffi::c_int
                    && (*StarShipPtr).crew_level as ::core::ffi::c_int
                        >= 1 as ::core::ffi::c_int
                    || (dy as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
                        && (*StarShipPtr).crew_level as ::core::ffi::c_int
                            != 1 as ::core::ffi::c_int)
            {
                uqm_lua_checked!(CrewTransaction(crew_delta), ());
                uqm_lua_checked!(animatePowerLines(::core::ptr::null_mut::<MENU_STATE>()), ());
            }
            r#loop = r#loop.wrapping_add(1);
        }
    }
    if crew_delta as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        uqm_lua_checked!(PlayMenuSound(MENU_SOUND_EFFECT::MENU_SOUND_FAILURE), ());
    }
    if !hStarShip.is_null() {
        UnlockLink(&raw mut GlobData.Game_state.built_ship_q, hStarShip);
        (*pMS).delta_item = ((*pMS).delta_item as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as SIZE;
    }
}
unsafe extern "C" fn DMS_TryAddEscortShip(mut pMS: *mut MENU_STATE) {
    let mut hFleet: HFLEETINFO = GetAvailableRaceFromIndex(
        (*pMS).delta_item as UWORD as BYTE,
    );
    let mut Index: COUNT = GetIndexFromStarShip(
        &raw mut GlobData.Game_state.avail_race_q,
        hFleet,
    );
    let mut shipCost: COUNT = ShipCost(Index as BYTE);
    if uqm_lua_checked!(CanBuyPoints(hFleet), ()).0 != 0 && GlobData.SIS_state.ResUnits >= shipCost as DWORD
        && !CloneShipFragment(
                RACE_ID(Index as ::core::ffi::c_uint),
                &raw mut GlobData.Game_state.built_ship_q,
                1 as COUNT,
            )
            .is_null()
    {
        uqm_lua_checked!(ShowCombatShip(
            pMS,
            (*pMS).CurState as COUNT,
            ::core::ptr::null_mut::<SHIP_FRAGMENT>(),
        ), ());
        uqm_lua_checked!(DrawMenuStateStrings(
            C2Rust_Unnamed_6::PM_CREW.0 as ::core::ffi::c_int as BYTE,
            C2Rust_Unnamed_10::SHIPYARD_CREW.0 as ::core::ffi::c_int as SWORD,
        ), ());
        uqm_lua_checked!(DrawMenuStateStrings(
            C2Rust_Unnamed_6::PM_CREW.0 as ::core::ffi::c_int as BYTE,
            C2Rust_Unnamed_10::SHIPYARD_CREW.0 as ::core::ffi::c_int as SWORD,
        ), ());
        uqm_lua_checked!(DeltaSISGauges(
            0x7fff as SIZE,
            0x7fff as SDWORD,
            -(shipCost as ::core::ffi::c_int),
        ), ());
        uqm_lua_checked!(DMS_SetMode(pMS, DMS_Mode::DMS_Mode_editCrew), ());
        uqm_lua_checked!(showRemainingPoints(0 as ::core::ffi::c_int), ());
    } else {
        uqm_lua_checked!(PlayMenuSound(MENU_SOUND_EFFECT::MENU_SOUND_FAILURE), ());
    };
}
unsafe extern "C" fn DMS_TryUnstowEscortShip(mut pMS: *mut MENU_STATE) {
    let mut hStarShip: HSHIPFRAG = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut hStowShip: HSHIPFRAG = GetStarShipFromIndex(
        &raw mut GlobData.Game_state.stowed_ship_q,
        (*pMS).delta_item as UWORD as BYTE as COUNT,
    );
    let mut StowShipPtr: *mut SHIP_FRAGMENT = LockShipFrag(
        &raw mut GlobData.Game_state.stowed_ship_q,
        hStowShip,
    );
    let mut Index: COUNT = (*StowShipPtr).race_id as COUNT;
    let mut hFleet: HFLEETINFO = GetStarShipFromIndex(
        &raw mut GlobData.Game_state.avail_race_q,
        Index,
    );
    if uqm_lua_checked!(CanBuyPoints(hFleet), ()).0 != 0
        && {
            hStarShip = CloneShipFragment(
                RACE_ID(Index as ::core::ffi::c_uint),
                &raw mut GlobData.Game_state.built_ship_q,
                (*StowShipPtr).crew_level,
            );
            !hStarShip.is_null()
        }
    {
        let mut StarShipPtr: *mut SHIP_FRAGMENT = LockShipFrag(
            &raw mut GlobData.Game_state.built_ship_q,
            hStarShip,
        );
        (*StarShipPtr).captains_name_index = (*StowShipPtr).captains_name_index;
        UnlockLink(&raw mut GlobData.Game_state.built_ship_q, hStarShip);
        UnlockLink(&raw mut GlobData.Game_state.stowed_ship_q, hStowShip);
        RemoveQueue(&raw mut GlobData.Game_state.stowed_ship_q, hStowShip);
        FreeLink(&raw mut GlobData.Game_state.stowed_ship_q, hStowShip);
        uqm_lua_checked!(ShowCombatShip(
            pMS,
            (*pMS).CurState as COUNT,
            ::core::ptr::null_mut::<SHIP_FRAGMENT>(),
        ), ());
        uqm_lua_checked!(DrawMenuStateStrings(
            C2Rust_Unnamed_6::PM_CREW.0 as ::core::ffi::c_int as BYTE,
            C2Rust_Unnamed_10::SHIPYARD_CREW.0 as ::core::ffi::c_int as SWORD,
        ), ());
        uqm_lua_checked!(DrawMenuStateStrings(
            C2Rust_Unnamed_6::PM_CREW.0 as ::core::ffi::c_int as BYTE,
            C2Rust_Unnamed_10::SHIPYARD_CREW.0 as ::core::ffi::c_int as SWORD,
        ), ());
        uqm_lua_checked!(DeltaSISGauges(0x7fff as SIZE, 0x7fff as SDWORD, 0x7fff as ::core::ffi::c_int), ());
        if optDosMenus.0 != 0
            || if optWindowType == 0 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
        {
            memset(
                &raw mut ShipState as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<SHIPS_STATE>(),
            );
            InventoryShips(&raw mut ShipState);
        }
        uqm_lua_checked!(DMS_SetMode(pMS, DMS_Mode::DMS_Mode_editCrew), ());
        uqm_lua_checked!(showRemainingPoints(0 as ::core::ffi::c_int), ());
        stowCount = GetStowedShipCount();
        currentStowShip = (*pMS).delta_item as UWORD as BYTE;
        stowed_q = BOOLEAN(
            stowed_q.0
                & (stowCount as ::core::ffi::c_int > 0 as ::core::ffi::c_int)
                    as ::core::ffi::c_int as ::core::ffi::c_uint,
        );
    } else {
        uqm_lua_checked!(PlayMenuSound(MENU_SOUND_EFFECT::MENU_SOUND_FAILURE), ());
        UnlockLink(&raw mut GlobData.Game_state.stowed_ship_q, hStowShip);
    };
}
unsafe extern "C" fn DMS_AddEscortShip(
    mut pMS: *mut MENU_STATE,
    mut special: BOOLEAN,
    mut select: BOOLEAN,
    mut cancel: BOOLEAN,
    mut dx: SBYTE,
    mut dy: SBYTE,
) {
    ((*pMS).delta_item as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int != 0
        || {
            __assert_fail(
                b"pMS->delta_item & MODIFY_CREW_FLAG\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/uqm/shipyard.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                1953 as ::core::ffi::c_int,
                b"DMS_AddEscortShip\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    let mut currentShip: BYTE = (*pMS).delta_item as UWORD as BYTE;
    if special.0 != 0 {
        if uqm_lua_checked!(DMS_SpinShip(pMS, ::core::ptr::null_mut::<::core::ffi::c_void>()), ()).0 != 0 {
            uqm_lua_checked!(DMS_SetMode(pMS, DMS_Mode::DMS_Mode_addEscort), ());
        }
        return;
    }
    if cancel.0 != 0 {
        (*pMS).delta_item = ((*pMS).delta_item as ::core::ffi::c_int
            & !((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)) as SIZE;
        uqm_lua_checked!(SetFlashRect(::core::ptr::null::<RECT>(), BOOLEAN::FALSE), ());
        uqm_lua_checked!(DrawMenuStateStrings(
            C2Rust_Unnamed_6::PM_CREW.0 as ::core::ffi::c_int as BYTE,
            C2Rust_Unnamed_10::SHIPYARD_CREW.0 as ::core::ffi::c_int as SWORD,
        ), ());
        uqm_lua_checked!(DrawMenuStateStrings(
            C2Rust_Unnamed_6::PM_CREW.0 as ::core::ffi::c_int as BYTE,
            C2Rust_Unnamed_10::SHIPYARD_CREW.0 as ::core::ffi::c_int as SWORD,
        ), ());
        uqm_lua_checked!(DMS_SetMode(pMS, DMS_Mode::DMS_Mode_navigate), ());
        if optDosMenus.0 != 0
            || if optWindowType == 0 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
        {
            uqm_lua_checked!(DeltaSISGauges(
                0x7fff as SIZE,
                0x7fff as SDWORD,
                0x7fff as ::core::ffi::c_int,
            ), ());
        }
        currentStowShip = currentShip;
    } else if select.0 != 0 {
        if stowed_q.0 == 0 {
            uqm_lua_checked!(DMS_TryAddEscortShip(pMS), ());
        } else {
            uqm_lua_checked!(DMS_TryUnstowEscortShip(pMS), ());
        }
    } else if dy as ::core::ffi::c_int != 0
        && (optShipStore.0 != 0
            || if if GlobData.SIS_state.Difficulty as ::core::ffi::c_int != 0 {
                GlobData.SIS_state.Difficulty as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } == 2 as ::core::ffi::c_int
            {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } != 0 || optFleetPointSys.0 != 0)
    {
        if stowCount as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            uqm_lua_checked!(PlayMenuSound(MENU_SOUND_EFFECT::MENU_SOUND_FAILURE), ());
        } else {
            stowed_q = BOOLEAN(
                (stowed_q.0 == 0) as ::core::ffi::c_int as ::core::ffi::c_uint,
            );
            currentShip = 0 as BYTE;
            uqm_lua_checked!(PreUpdateFlashRect(), ());
            uqm_lua_checked!(DrawRaceStrings(currentShip), ());
            uqm_lua_checked!(PostUpdateFlashRect(), ());
            (*pMS).delta_item = (currentShip as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as SIZE;
        }
    } else if dx != 0 {
        if (dx as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
            if (currentShip as ::core::ffi::c_int) < -(dx as ::core::ffi::c_int) {
                currentShip = (if stowed_q.0 != 0 {
                    stowCount as ::core::ffi::c_int
                } else {
                    availableCount as ::core::ffi::c_int
                } - 1 as ::core::ffi::c_int) as BYTE;
                if (optDosMenus.0 != 0
                    || if optWindowType == 0 as DWORD {
                        BOOLEAN::TRUE.0 as ::core::ffi::c_int
                    } else {
                        BOOLEAN::FALSE.0 as ::core::ffi::c_int
                    } != 0) && dx as ::core::ffi::c_int != -1 as ::core::ffi::c_int
                {
                    currentShip = (currentShip as ::core::ffi::c_int
                        - currentShip as ::core::ffi::c_int % 5 as ::core::ffi::c_int)
                        as BYTE;
                }
            } else {
                currentShip = (currentShip as ::core::ffi::c_int
                    + dx as ::core::ffi::c_int) as BYTE;
            }
        } else if dx as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
            if currentShip as ::core::ffi::c_int + dx as ::core::ffi::c_int
                >= if stowed_q.0 != 0 {
                    stowCount as ::core::ffi::c_int
                } else {
                    availableCount as ::core::ffi::c_int
                }
            {
                currentShip = 0 as BYTE;
            } else {
                currentShip = (currentShip as ::core::ffi::c_int
                    + dx as ::core::ffi::c_int) as BYTE;
            }
        }
        if currentShip as ::core::ffi::c_int
            != (*pMS).delta_item as UWORD as BYTE as ::core::ffi::c_int
        {
            uqm_lua_checked!(PreUpdateFlashRect(), ());
            uqm_lua_checked!(DrawRaceStrings(currentShip), ());
            uqm_lua_checked!(PostUpdateFlashRect(), ());
            (*pMS).delta_item = (currentShip as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as SIZE;
        }
    }
}
unsafe extern "C" fn DMS_ScrapEscortShip(
    mut pMS: *mut MENU_STATE,
    mut hStarShip: HSHIPFRAG,
) {
    let mut StarShipPtr: *mut SHIP_FRAGMENT = LockShipFrag(
        &raw mut GlobData.Game_state.built_ship_q,
        hStarShip,
    );
    uqm_lua_checked!(SetFlashRect(::core::ptr::null::<RECT>(), BOOLEAN::FALSE), ());
    uqm_lua_checked!(ShowCombatShip(pMS, (*pMS).CurState as COUNT, StarShipPtr), ());
    UnlockLink(&raw mut GlobData.Game_state.built_ship_q, hStarShip);
    RemoveQueue(&raw mut GlobData.Game_state.built_ship_q, hStarShip);
    FreeLink(&raw mut GlobData.Game_state.built_ship_q, hStarShip);
    uqm_lua_checked!(DeltaSISGauges(0x7fff as SIZE, 0x7fff as SDWORD, 0x7fff as ::core::ffi::c_int), ());
    uqm_lua_checked!(showRemainingPoints(0 as ::core::ffi::c_int), ());
    SetContext(SpaceContext);
    uqm_lua_checked!(DMS_SetMode(pMS, DMS_Mode::DMS_Mode_navigate), ());
}
unsafe extern "C" fn DMS_StowEscortShip(
    mut pMS: *mut MENU_STATE,
    mut hStarShip: HSHIPFRAG,
) {
    let mut hStowShip: HSHIPFRAG = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut StarShipPtr: *mut SHIP_FRAGMENT = LockShipFrag(
        &raw mut GlobData.Game_state.built_ship_q,
        hStarShip,
    );
    uqm_lua_checked!(SetFlashRect(::core::ptr::null::<RECT>(), BOOLEAN::FALSE), ());
    uqm_lua_checked!(ShowCombatShip(pMS, (*pMS).CurState as COUNT, StarShipPtr), ());
    let mut Index: COUNT = (*StarShipPtr).race_id as COUNT;
    hStowShip = CloneShipFragment(
        RACE_ID(Index as ::core::ffi::c_uint),
        &raw mut GlobData.Game_state.stowed_ship_q,
        (*StarShipPtr).crew_level,
    );
    if !hStowShip.is_null() {
        let mut StowShipPtr: *mut SHIP_FRAGMENT = LockShipFrag(
            &raw mut GlobData.Game_state.stowed_ship_q,
            hStowShip,
        );
        (*StowShipPtr).captains_name_index = (*StarShipPtr).captains_name_index;
        UnlockLink(&raw mut GlobData.Game_state.stowed_ship_q, hStowShip);
    }
    UnlockLink(&raw mut GlobData.Game_state.built_ship_q, hStarShip);
    RemoveQueue(&raw mut GlobData.Game_state.built_ship_q, hStarShip);
    FreeLink(&raw mut GlobData.Game_state.built_ship_q, hStarShip);
    uqm_lua_checked!(DeltaSISGauges(0x7fff as SIZE, 0x7fff as SDWORD, 0x7fff as ::core::ffi::c_int), ());
    uqm_lua_checked!(showRemainingPoints(0 as ::core::ffi::c_int), ());
    stowCount = GetStowedShipCount();
    SetContext(SpaceContext);
    uqm_lua_checked!(DMS_SetMode(pMS, DMS_Mode::DMS_Mode_navigate), ());
}
unsafe extern "C" fn DMS_MoveCursor(
    mut curState: BYTE,
    mut dx: SBYTE,
    mut dy: SBYTE,
) -> BYTE {
    let mut row: BYTE = ((curState as ::core::ffi::c_int
        & 0xf as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE
        as ::core::ffi::c_int
        / if if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            4 as ::core::ffi::c_int
        } else {
            6 as ::core::ffi::c_int
        }) as BYTE;
    let mut col: BYTE = ((curState as ::core::ffi::c_int
        & 0xf as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE
        as ::core::ffi::c_int
        % if if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            4 as ::core::ffi::c_int
        } else {
            6 as ::core::ffi::c_int
        }) as BYTE;
    let mut isFlagShipSelected: BOOLEAN = BOOLEAN(
        ((curState as ::core::ffi::c_int
            >> 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE
            as ::core::ffi::c_int != 0 as ::core::ffi::c_int) as ::core::ffi::c_int
            as ::core::ffi::c_uint,
    );
    if dy != 0 {
        if isFlagShipSelected.0 != 0 {
            row = (12 as ::core::ffi::c_int
                / if if optWindowType == 1 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    4 as ::core::ffi::c_int
                } else {
                    6 as ::core::ffi::c_int
                }) as BYTE;
        }
        row = ((row as ::core::ffi::c_int
            + (12 as ::core::ffi::c_int
                / if if optWindowType == 1 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    4 as ::core::ffi::c_int
                } else {
                    6 as ::core::ffi::c_int
                } + 1 as ::core::ffi::c_int) + dy as ::core::ffi::c_int)
            % (12 as ::core::ffi::c_int
                / if if optWindowType == 1 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    4 as ::core::ffi::c_int
                } else {
                    6 as ::core::ffi::c_int
                } + 1 as ::core::ffi::c_int)) as BYTE;
        isFlagShipSelected = BOOLEAN(
            (row as ::core::ffi::c_int
                == 12 as ::core::ffi::c_int
                    / if if optWindowType == 1 as DWORD {
                        BOOLEAN::TRUE.0 as ::core::ffi::c_int
                    } else {
                        BOOLEAN::FALSE.0 as ::core::ffi::c_int
                    } != 0
                    {
                        4 as ::core::ffi::c_int
                    } else {
                        6 as ::core::ffi::c_int
                    }) as ::core::ffi::c_int as ::core::ffi::c_uint,
        );
        if isFlagShipSelected.0 != 0 {
            row = 0 as BYTE;
        }
    } else if dx != 0 {
        if isFlagShipSelected.0 == 0 {
            col = ((col as ::core::ffi::c_int
                + if if optWindowType == 1 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    4 as ::core::ffi::c_int
                } else {
                    6 as ::core::ffi::c_int
                } + dx as ::core::ffi::c_int)
                % if if optWindowType == 1 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    4 as ::core::ffi::c_int
                } else {
                    6 as ::core::ffi::c_int
                }) as BYTE;
        }
    }
    return (((if isFlagShipSelected.0 != 0 {
        0xf as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    }) as BYTE as ::core::ffi::c_int)
        << 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int
        | (row as ::core::ffi::c_int
            * if if optWindowType == 1 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                4 as ::core::ffi::c_int
            } else {
                6 as ::core::ffi::c_int
            } + col as ::core::ffi::c_int) as BYTE as ::core::ffi::c_int) as BYTE;
}
unsafe extern "C" fn DMS_EditCrewMode(
    mut pMS: *mut MENU_STATE,
    mut hStarShip: HSHIPFRAG,
    mut select: BOOLEAN,
    mut cancel: BOOLEAN,
    mut dy: SBYTE,
) {
    if cancel.0 != 0 {
        if !hStarShip.is_null() {
            let mut StarShipPtr: *mut SHIP_FRAGMENT = LockShipFrag(
                &raw mut GlobData.Game_state.built_ship_q,
                hStarShip,
            );
            let mut crew_level: COUNT = (*StarShipPtr).crew_level;
            let mut max_crew: COUNT = (*StarShipPtr).max_crew;
            if stowCount as ::core::ffi::c_int == 200 as ::core::ffi::c_int
                && crew_level as ::core::ffi::c_int > max_crew as ::core::ffi::c_int
            {
                uqm_lua_checked!(PlayMenuSound(MENU_SOUND_EFFECT::MENU_SOUND_FAILURE), ());
                UnlockLink(&raw mut GlobData.Game_state.built_ship_q, hStarShip);
                return;
            }
            if crew_level as ::core::ffi::c_int > max_crew as ::core::ffi::c_int {
                (*StarShipPtr).crew_level = ((*StarShipPtr).crew_level
                    as ::core::ffi::c_int
                    - (*StarShipPtr).max_crew as ::core::ffi::c_int) as COUNT;
            }
            UnlockLink(&raw mut GlobData.Game_state.built_ship_q, hStarShip);
            if crew_level as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                uqm_lua_checked!(DMS_ScrapEscortShip(pMS, hStarShip), ());
            } else if crew_level as ::core::ffi::c_int > max_crew as ::core::ffi::c_int {
                uqm_lua_checked!(DMS_StowEscortShip(pMS, hStarShip), ());
                if optDosMenus.0 != 0
                    || if optWindowType == 0 as DWORD {
                        BOOLEAN::TRUE.0 as ::core::ffi::c_int
                    } else {
                        BOOLEAN::FALSE.0 as ::core::ffi::c_int
                    } != 0
                {
                    memset(
                        &raw mut ShipState as *mut ::core::ffi::c_void,
                        0 as ::core::ffi::c_int,
                        ::core::mem::size_of::<SHIPS_STATE>(),
                    );
                    InventoryShips(&raw mut ShipState);
                }
            }
        }
        (*pMS).delta_item = ((*pMS).delta_item as ::core::ffi::c_int
            & !((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)) as SIZE;
        uqm_lua_checked!(DMS_SetMode(pMS, DMS_Mode::DMS_Mode_navigate), ());
    } else if dy != 0 {
        uqm_lua_checked!(DMS_ModifyCrew(pMS, hStarShip, dy), ());
    } else if select.0 != 0 && !hStarShip.is_null() {
        let mut StarShipPtr_0: *mut SHIP_FRAGMENT = LockShipFrag(
            &raw mut GlobData.Game_state.built_ship_q,
            hStarShip,
        );
        let mut hTemplate: HFLEETINFO = GetStarShipFromIndex(
            &raw mut GlobData.Game_state.avail_race_q,
            (*StarShipPtr_0).race_id as COUNT,
        );
        if (*StarShipPtr_0).crew_level as ::core::ffi::c_int
            > (*StarShipPtr_0).max_crew as ::core::ffi::c_int
        {
            (*StarShipPtr_0).crew_level = ((*StarShipPtr_0).crew_level
                as ::core::ffi::c_int - (*StarShipPtr_0).max_crew as ::core::ffi::c_int)
                as COUNT;
            uqm_lua_checked!(showRemainingPoints(0 as ::core::ffi::c_int), ());
        } else if (*StarShipPtr_0).crew_level as ::core::ffi::c_int
            > 0 as ::core::ffi::c_int
        {
            (*StarShipPtr_0).crew_level = ((*StarShipPtr_0).crew_level
                as ::core::ffi::c_int + (*StarShipPtr_0).max_crew as ::core::ffi::c_int)
                as COUNT;
            uqm_lua_checked!(showRemainingPoints(ShipPoints(hTemplate) as ::core::ffi::c_int), ());
        }
        let mut r: RECT = RECT {
            corner: POINT { x: 0, y: 0 },
            extent: EXTENT { width: 0, height: 0 },
        };
        uqm_lua_checked!(PreUpdateFlashRect(), ());
        DMS_GetEscortShipRect(&raw mut r, (*StarShipPtr_0).index);
        uqm_lua_checked!(ShowShipCrew(StarShipPtr_0, &raw mut r), ());
        uqm_lua_checked!(PostUpdateFlashRect(), ());
        UnlockLink(&raw mut GlobData.Game_state.built_ship_q, hStarShip);
    }
}
unsafe extern "C" fn DMS_NavigateShipSlots(
    mut pMS: *mut MENU_STATE,
    mut special: BOOLEAN,
    mut select: BOOLEAN,
    mut cancel: BOOLEAN,
    mut dx: SBYTE,
    mut dy: SBYTE,
) {
    let mut hStarShip: HSHIPFRAG = GetEscortByStarShipIndex((*pMS).CurState as COUNT);
    if dx as ::core::ffi::c_int != 0 || dy as ::core::ffi::c_int != 0 {
        let mut NewState: BYTE = DMS_MoveCursor((*pMS).CurState, dx, dy);
        if NewState as ::core::ffi::c_int != (*pMS).CurState as ::core::ffi::c_int {
            (*pMS).CurState = NewState;
            uqm_lua_checked!(DMS_FlashActiveShip(pMS), ());
        }
    }
    if special.0 != 0 {
        if uqm_lua_checked!(DMS_SpinShip(pMS, hStarShip), ()).0 != 0 {
            uqm_lua_checked!(DMS_SetMode(pMS, DMS_Mode::DMS_Mode_navigate), ());
        }
    } else if select.0 != 0 {
        if hStarShip.is_null()
            && ((*pMS).CurState as ::core::ffi::c_int
                >> 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE
                as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        {
            if !(optShipStore.0 != 0
                || if if GlobData.SIS_state.Difficulty as ::core::ffi::c_int != 0 {
                    GlobData.SIS_state.Difficulty as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                } == 2 as ::core::ffi::c_int
                {
                    1 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                } != 0 || optFleetPointSys.0 != 0) || stowCount == 0
            {
                stowed_q = BOOLEAN(0 as ::core::ffi::c_int as ::core::ffi::c_uint);
            } else if currentStowShip as ::core::ffi::c_int
                >= stowCount as ::core::ffi::c_int
            {
                currentStowShip = (stowCount as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as BYTE;
            }
            (*pMS).delta_item = ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
                | if stowed_q.0 != 0 {
                    currentStowShip as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                }) as SIZE;
            uqm_lua_checked!(DrawRaceStrings((*pMS).delta_item as UWORD as BYTE), ());
            uqm_lua_checked!(DMS_SetMode(pMS, DMS_Mode::DMS_Mode_addEscort), ());
        } else {
            (*pMS).delta_item = ((*pMS).delta_item as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as SIZE;
            uqm_lua_checked!(DMS_SetMode(pMS, DMS_Mode::DMS_Mode_editCrew), ());
        }
    } else if cancel.0 != 0 {
        (*pMS).InputFunc = Some(
            DoShipyard as unsafe extern "C" fn(*mut MENU_STATE) -> BOOLEAN,
        ) as Option<unsafe extern "C" fn(*mut menu_state) -> BOOLEAN>;
        (*pMS).CurState = C2Rust_Unnamed_10::SHIPYARD_CREW.0 as ::core::ffi::c_int
            as BYTE;
        uqm_lua_checked!(DrawMenuStateStrings(
            C2Rust_Unnamed_6::PM_CREW.0 as ::core::ffi::c_int as BYTE,
            (*pMS).CurState as SWORD,
        ), ());
        uqm_lua_checked!(DMS_SetMode(pMS, DMS_Mode::DMS_Mode_exit), ());
    }
}
unsafe extern "C" fn DoModifyShips(mut pMS: *mut MENU_STATE) -> BOOLEAN {
    if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
        & (C2Rust_Unnamed_5::CHECK_ABORT.0 as ::core::ffi::c_int
            | C2Rust_Unnamed_5::CHECK_LOAD.0 as ::core::ffi::c_int) != 0
    {
        (*pMS).InputFunc = Some(
            DoShipyard as unsafe extern "C" fn(*mut MENU_STATE) -> BOOLEAN,
        ) as Option<unsafe extern "C" fn(*mut menu_state) -> BOOLEAN>;
        return BOOLEAN::TRUE;
    }
    if (*pMS).Initialized == 0 {
        (*pMS).InputFunc = Some(
            DoModifyShips as unsafe extern "C" fn(*mut MENU_STATE) -> BOOLEAN,
        ) as Option<unsafe extern "C" fn(*mut menu_state) -> BOOLEAN>;
        (*pMS).Initialized = BOOLEAN::TRUE.0 as ::core::ffi::c_int as SIZE;
        (*pMS).CurState = ((0xf as ::core::ffi::c_int as BYTE as ::core::ffi::c_int)
            << 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int
            | 0 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE;
        (*pMS).delta_item = 0 as SIZE;
        SetContext(SpaceContext);
        uqm_lua_checked!(DMS_SetMode(pMS, DMS_Mode::DMS_Mode_navigate), BOOLEAN(0));
        stowCount = GetStowedShipCount();
        availableCount = GetAvailableRaceCount();
    } else {
        let mut special: BOOLEAN = BOOLEAN(
            (PulsedInputState
                .menu[C2Rust_Unnamed_0::KEY_MENU_SPECIAL.0 as ::core::ffi::c_int
                as usize] != 0 as ::core::ffi::c_int) as ::core::ffi::c_int
                as ::core::ffi::c_uint,
        );
        let mut select: BOOLEAN = BOOLEAN(
            (PulsedInputState
                .menu[C2Rust_Unnamed_0::KEY_MENU_SELECT.0 as ::core::ffi::c_int as usize]
                != 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_uint,
        );
        let mut cancel: BOOLEAN = BOOLEAN(
            (PulsedInputState
                .menu[C2Rust_Unnamed_0::KEY_MENU_CANCEL.0 as ::core::ffi::c_int as usize]
                != 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_uint,
        );
        let mut dx: SBYTE = 0 as SBYTE;
        let mut dy: SBYTE = 0 as SBYTE;
        if (*pMS).delta_item as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int == 0
        {
            if PulsedInputState
                .menu[C2Rust_Unnamed_0::KEY_MENU_RIGHT.0 as ::core::ffi::c_int as usize]
                != 0
            {
                dx = 1 as SBYTE;
            }
            if PulsedInputState
                .menu[C2Rust_Unnamed_0::KEY_MENU_LEFT.0 as ::core::ffi::c_int as usize]
                != 0
            {
                dx = -1 as SBYTE;
            }
            if PulsedInputState
                .menu[C2Rust_Unnamed_0::KEY_MENU_UP.0 as ::core::ffi::c_int as usize]
                != 0
            {
                dy = -1 as SBYTE;
            }
            if PulsedInputState
                .menu[C2Rust_Unnamed_0::KEY_MENU_DOWN.0 as ::core::ffi::c_int as usize]
                != 0
            {
                dy = 1 as SBYTE;
            }
            uqm_lua_checked!(DMS_NavigateShipSlots(pMS, special, select, cancel, dx, dy), BOOLEAN(0));
        } else {
            let mut hStarShip: HSHIPFRAG = GetEscortByStarShipIndex(
                (*pMS).CurState as COUNT,
            );
            if hStarShip.is_null()
                && ((*pMS).CurState as ::core::ffi::c_int
                    >> 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE
                    as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            {
                if PulsedInputState
                    .menu[C2Rust_Unnamed_0::KEY_MENU_RIGHT.0 as ::core::ffi::c_int
                    as usize] != 0
                {
                    dx = 1 as SBYTE;
                }
                if PulsedInputState
                    .menu[C2Rust_Unnamed_0::KEY_MENU_LEFT.0 as ::core::ffi::c_int
                    as usize] != 0
                {
                    dx = -1 as SBYTE;
                }
                if PulsedInputState
                    .menu[C2Rust_Unnamed_0::KEY_MENU_UP.0 as ::core::ffi::c_int as usize]
                    != 0
                {
                    dx = -1 as SBYTE;
                }
                if PulsedInputState
                    .menu[C2Rust_Unnamed_0::KEY_MENU_DOWN.0 as ::core::ffi::c_int
                    as usize] != 0
                {
                    dx = 1 as SBYTE;
                }
                if PulsedInputState
                    .menu[C2Rust_Unnamed_0::KEY_MENU_ZOOM_IN.0 as ::core::ffi::c_int
                    as usize] != 0
                {
                    dx = 5 as SBYTE;
                }
                if PulsedInputState
                    .menu[C2Rust_Unnamed_0::KEY_MENU_ZOOM_OUT.0 as ::core::ffi::c_int
                    as usize] != 0
                {
                    dx = -5 as SBYTE;
                }
                if PulsedInputState
                    .menu[C2Rust_Unnamed_0::KEY_MENU_NEXT.0 as ::core::ffi::c_int
                    as usize] != 0
                {
                    dy = 1 as SBYTE;
                }
                uqm_lua_checked!(DMS_AddEscortShip(pMS, special, select, cancel, dx, dy), BOOLEAN(0));
            } else {
                if PulsedInputState
                    .menu[C2Rust_Unnamed_0::KEY_MENU_UP.0 as ::core::ffi::c_int as usize]
                    != 0
                {
                    dy = -1 as SBYTE;
                }
                if PulsedInputState
                    .menu[C2Rust_Unnamed_0::KEY_MENU_DOWN.0 as ::core::ffi::c_int
                    as usize] != 0
                {
                    dy = 1 as SBYTE;
                }
                if PulsedInputState
                    .menu[C2Rust_Unnamed_0::KEY_MENU_RIGHT.0 as ::core::ffi::c_int
                    as usize] != 0
                {
                    dy = -10 as SBYTE;
                }
                if PulsedInputState
                    .menu[C2Rust_Unnamed_0::KEY_MENU_LEFT.0 as ::core::ffi::c_int
                    as usize] != 0
                {
                    dy = 10 as SBYTE;
                }
                if PulsedInputState
                    .menu[C2Rust_Unnamed_0::KEY_MENU_ZOOM_IN.0 as ::core::ffi::c_int
                    as usize] != 0
                {
                    dy = -50 as SBYTE;
                }
                if PulsedInputState
                    .menu[C2Rust_Unnamed_0::KEY_MENU_ZOOM_OUT.0 as ::core::ffi::c_int
                    as usize] != 0
                {
                    dy = 50 as SBYTE;
                }
                special = BOOLEAN(0 as ::core::ffi::c_int as ::core::ffi::c_uint);
                if PulsedInputState
                    .menu[C2Rust_Unnamed_0::KEY_MENU_NEXT.0 as ::core::ffi::c_int
                    as usize] != 0
                {
                    special = BOOLEAN(1 as ::core::ffi::c_int as ::core::ffi::c_uint);
                }
                uqm_lua_checked!(DMS_EditCrewMode(
                    pMS,
                    hStarShip,
                    BOOLEAN(
                        (special.0 != 0
                            && (optShipStore.0 != 0
                                || if if GlobData.SIS_state.Difficulty as ::core::ffi::c_int
                                    != 0
                                {
                                    GlobData.SIS_state.Difficulty as ::core::ffi::c_int
                                } else {
                                    0 as ::core::ffi::c_int
                                } == 2 as ::core::ffi::c_int
                                {
                                    1 as ::core::ffi::c_int
                                } else {
                                    0 as ::core::ffi::c_int
                                } != 0 || optFleetPointSys.0 != 0)) as ::core::ffi::c_int
                            as ::core::ffi::c_uint,
                    ),
                    BOOLEAN(
                        (select.0 != 0 || cancel.0 != 0) as ::core::ffi::c_int
                            as ::core::ffi::c_uint,
                    ),
                    dy,
                ), BOOLEAN(0));
            }
        }
    }
    uqm_lua_checked!(SleepThread((840 as ::core::ffi::c_int / 60 as ::core::ffi::c_int) as TimePeriod), BOOLEAN(0));
    return BOOLEAN::TRUE;
}
unsafe extern "C" fn DrawBluePrint(mut pMS: *mut MENU_STATE) {
    let mut num_frames: COUNT = 0;
    let mut s: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    let mut ModuleFrame: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
    ModuleFrame = CaptureDrawable(
        uqm_lua_checked!(LoadGraphicInstance(b"graphics.blueprints\0".as_ptr() as RESOURCE), ()) as DRAWABLE,
    );
    s.origin.x = -if if optWindowType == 1 as DWORD {
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
        } << resolutionFactor) - ((3 as ::core::ffi::c_int) << resolutionFactor)
    } else {
        0 as ::core::ffi::c_int
    } as COORD;
    s.origin.y = 0 as COORD;
    s.frame = DecFrameIndex(ModuleFrame);
    SetContextForeGroundColor(colorFromRgb15(0 as BYTE, 0 as BYTE, 0x16 as BYTE));
    uqm_lua_checked!(DrawFilledStamp(&raw mut s), ());
    num_frames = 0 as COUNT;
    while (num_frames as ::core::ffi::c_int) < 11 as ::core::ffi::c_int {
        uqm_lua_checked!(DrawShipPiece(
            ModuleFrame,
            GlobData.SIS_state.DriveSlots[num_frames as usize] as COUNT,
            num_frames,
            BOOLEAN::TRUE,
        ), ());
        num_frames = num_frames.wrapping_add(1);
    }
    num_frames = 0 as COUNT;
    while (num_frames as ::core::ffi::c_int) < 8 as ::core::ffi::c_int {
        uqm_lua_checked!(DrawShipPiece(
            ModuleFrame,
            GlobData.SIS_state.JetSlots[num_frames as usize] as COUNT,
            num_frames,
            BOOLEAN::TRUE,
        ), ());
        num_frames = num_frames.wrapping_add(1);
    }
    num_frames = 0 as COUNT;
    while (num_frames as ::core::ffi::c_int) < 16 as ::core::ffi::c_int {
        let mut which_piece: BYTE = 0;
        which_piece = GlobData.SIS_state.ModuleSlots[num_frames as usize];
        if !((*pMS).CurState as ::core::ffi::c_int
            == C2Rust_Unnamed_8::SHIPYARD.0 as ::core::ffi::c_int
            && (which_piece as ::core::ffi::c_int
                == C2Rust_Unnamed_2::CREW_POD.0 as ::core::ffi::c_int
                || which_piece as ::core::ffi::c_int
                    == C2Rust_Unnamed_2::STORAGE_BAY.0 as ::core::ffi::c_int))
        {
            uqm_lua_checked!(DrawShipPiece(ModuleFrame, which_piece as COUNT, num_frames, BOOLEAN::TRUE), ());
        }
        num_frames = num_frames.wrapping_add(1);
    }
    SetContextForeGroundColor(colorFromRgb15(0xa as BYTE, 0xa as BYTE, 0x1f as BYTE));
    num_frames = 0 as COUNT;
    while (num_frames as ::core::ffi::c_int) < 16 as ::core::ffi::c_int {
        let mut which_piece_0: BYTE = 0;
        which_piece_0 = GlobData.SIS_state.ModuleSlots[num_frames as usize];
        if (*pMS).CurState as ::core::ffi::c_int
            == C2Rust_Unnamed_8::SHIPYARD.0 as ::core::ffi::c_int
            && (which_piece_0 as ::core::ffi::c_int
                == C2Rust_Unnamed_2::CREW_POD.0 as ::core::ffi::c_int
                || which_piece_0 as ::core::ffi::c_int
                    == C2Rust_Unnamed_2::STORAGE_BAY.0 as ::core::ffi::c_int)
        {
            uqm_lua_checked!(DrawShipPiece(
                ModuleFrame,
                which_piece_0 as COUNT,
                num_frames,
                BOOLEAN::TRUE,
            ), ());
        }
        num_frames = num_frames.wrapping_add(1);
    }
    num_frames = GlobData.SIS_state.CrewEnlisted;
    GlobData.SIS_state.CrewEnlisted = 0 as COUNT;
    loop {
        let c2rust_fresh1 = num_frames;
        num_frames = num_frames.wrapping_sub(1);
        if c2rust_fresh1 == 0 {
            break;
        }
        let mut r: RECT = RECT {
            corner: POINT { x: 0, y: 0 },
            extent: EXTENT { width: 0, height: 0 },
        };
        r.extent.width = ((1 as ::core::ffi::c_int) << resolutionFactor) as COORD;
        r.extent.height = r.extent.width;
        GetCPodCapacity(&raw mut r.corner);
        uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
        GlobData.SIS_state.CrewEnlisted = GlobData
            .SIS_state
            .CrewEnlisted
            .wrapping_add(1);
    }
    let mut r_0: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    num_frames = GlobData.SIS_state.TotalElementMass;
    GlobData.SIS_state.TotalElementMass = 0 as COUNT;
    r_0.extent.width = ((9 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    r_0.extent.height = ((1 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    while num_frames != 0 {
        let mut m: COUNT = 0;
        m = (if (num_frames as ::core::ffi::c_int) < 50 as ::core::ffi::c_int {
            num_frames as ::core::ffi::c_int
        } else {
            50 as ::core::ffi::c_int
        }) as COUNT;
        GlobData.SIS_state.TotalElementMass = (GlobData.SIS_state.TotalElementMass
            as ::core::ffi::c_int + m as ::core::ffi::c_int) as COUNT;
        GetSBayCapacity(&raw mut r_0.corner);
        uqm_lua_checked!(DrawFilledRectangle(&raw mut r_0), ());
        num_frames = (num_frames as ::core::ffi::c_int - m as ::core::ffi::c_int)
            as COUNT;
    }
    uqm_lua_checked!(DrawFuelInFTanks(BOOLEAN::FALSE), ());
    uqm_lua_checked!(showRemainingCrew(), ());
    uqm_lua_checked!(showRemainingPoints(0 as ::core::ffi::c_int), ());
    DestroyDrawable(ReleaseDrawable(ModuleFrame));
}
#[no_mangle]
pub unsafe extern "C" fn DoShipyard(mut pMS: *mut MENU_STATE) -> BOOLEAN {
    let mut select: BOOLEAN = BOOLEAN::FALSE;
    let mut cancel: BOOLEAN = BOOLEAN::FALSE;
    '_ExitShipyard: {
        if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
            & C2Rust_Unnamed_5::CHECK_ABORT.0 as ::core::ffi::c_int == 0
        {
            select = BOOLEAN(
                PulsedInputState
                    .menu[C2Rust_Unnamed_0::KEY_MENU_SELECT.0 as ::core::ffi::c_int
                    as usize] as ::core::ffi::c_uint,
            );
            cancel = BOOLEAN(
                PulsedInputState
                    .menu[C2Rust_Unnamed_0::KEY_MENU_CANCEL.0 as ::core::ffi::c_int
                    as usize] as ::core::ffi::c_uint,
            );
            OutfitOrShipyard = 3 as BYTE;
            FillHangarX();
            SetMenuSounds(
                (((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int)
                    as MENU_SOUND_FLAGS as ::core::ffi::c_int
                    | ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
                        as MENU_SOUND_FLAGS as ::core::ffi::c_int
                    | ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                        as MENU_SOUND_FLAGS as ::core::ffi::c_int
                    | ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                        as MENU_SOUND_FLAGS as ::core::ffi::c_int) as MENU_SOUND_FLAGS,
                ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                    as MENU_SOUND_FLAGS,
            );
            if (*pMS).Initialized == 0 {
                (*pMS).InputFunc = Some(
                    DoShipyard as unsafe extern "C" fn(*mut MENU_STATE) -> BOOLEAN,
                ) as Option<unsafe extern "C" fn(*mut menu_state) -> BOOLEAN>;
                stowCount = GetStowedShipCount();
                availableCount = GetAvailableRaceCount();
                if optDosMenus.0 != 0
                    || if optWindowType == 0 as DWORD {
                        BOOLEAN::TRUE.0 as ::core::ffi::c_int
                    } else {
                        BOOLEAN::FALSE.0 as ::core::ffi::c_int
                    } != 0
                {
                    memset(
                        &raw mut ShipState as *mut ::core::ffi::c_void,
                        0 as ::core::ffi::c_int,
                        ::core::mem::size_of::<SHIPS_STATE>(),
                    );
                    InventoryShips(&raw mut ShipState);
                }
                if if optWindowType == 0 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } == 0
                {
                    ModuleFont = uqm_lua_checked!(LoadGraphicInstance(
                        b"font.module\0".as_ptr() as RESOURCE,
                    ), BOOLEAN(0)) as FONT;
                }
                let mut s: STAMP = STAMP {
                    origin: POINT { x: 0, y: 0 },
                    frame: ::core::ptr::null_mut::<FRAME_DESC>(),
                };
                let mut r: RECT = RECT {
                    corner: POINT { x: 0, y: 0 },
                    extent: EXTENT { width: 0, height: 0 },
                };
                let mut old_r: RECT = RECT {
                    corner: POINT { x: 0, y: 0 },
                    extent: EXTENT { width: 0, height: 0 },
                };
                (*pMS).ModuleFrame = CaptureDrawable(
                    uqm_lua_checked!(LoadGraphicInstance(b"graphics.shipyard\0".as_ptr() as RESOURCE), BOOLEAN(0))
                        as DRAWABLE,
                );
                (*pMS).CurString = CaptureStringTable(
                    uqm_lua_checked!(LoadStringTableInstance(b"colortable.hangar\0".as_ptr() as RESOURCE), BOOLEAN(0)),
                );
                (*pMS).hMusic = uqm_lua_checked!(LoadMusicInstance(
                    b"music.shipyard\0".as_ptr() as RESOURCE,
                ), BOOLEAN(0));
                SetTransitionSource(::core::ptr::null::<RECT>());
                BatchGraphics();
                uqm_lua_checked!(DrawSISFrame(), BOOLEAN(0));
                uqm_lua_checked!(DrawSISMessage(
                    GetStringAddress(
                        SetAbsStringTableIndex(
                            GameStrings,
                            (C2Rust_Unnamed_7::STARBASE_STRING_BASE.0
                                as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as COUNT,
                        ),
                    ),
                ), BOOLEAN(0));
                uqm_lua_checked!(DrawSISTitle(
                    GetStringAddress(
                        SetAbsStringTableIndex(
                            GameStrings,
                            C2Rust_Unnamed_7::STARBASE_STRING_BASE.0
                                as ::core::ffi::c_int as COUNT,
                        ),
                    ),
                ), BOOLEAN(0));
                SetContext(SpaceContext);
                uqm_lua_checked!(DrawBluePrint(pMS), BOOLEAN(0));
                (*pMS).CurState = C2Rust_Unnamed_10::SHIPYARD_CREW.0
                    as ::core::ffi::c_int as BYTE;
                uqm_lua_checked!(DrawMenuStateStrings(
                    C2Rust_Unnamed_6::PM_CREW.0 as ::core::ffi::c_int as BYTE,
                    (*pMS).CurState as SWORD,
                ), BOOLEAN(0));
                SetContext(SpaceContext);
                s.origin.x = 0 as COORD;
                s.origin.y = 0 as COORD;
                s.frame = SetAbsFrameIndex((*pMS).ModuleFrame, 0 as COUNT);
                if !(if optWindowType == 1 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0 || classicPackPresent.0 != 0)
                {
                    GetContextClipRect(&raw mut old_r);
                    r = old_r;
                    r.corner.x = (r.corner.x as ::core::ffi::c_int
                        - ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
                    r.extent.width = (r.extent.width as ::core::ffi::c_int
                        + ((2 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
                    r.extent.height = (r.extent.height as ::core::ffi::c_int
                        + ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
                    SetContextClipRect(&raw mut r);
                    uqm_lua_checked!(DrawStamp(&raw mut s), BOOLEAN(0));
                    if optCustomBorder.0 != 0 {
                        uqm_lua_checked!(DrawBorder(
                            C2Rust_Unnamed_3::SIS_REPAIR_FRAME.0 as ::core::ffi::c_int
                                as BYTE,
                        ), BOOLEAN(0));
                    }
                    SetContextClipRect(&raw mut old_r);
                    uqm_lua_checked!(animatePowerLines(pMS), BOOLEAN(0));
                } else {
                    uqm_lua_checked!(DrawStamp(&raw mut s), BOOLEAN(0));
                    if classicPackPresent.0 != 0 {
                        uqm_lua_checked!(animatePowerLines(pMS), BOOLEAN(0));
                    }
                }
                if isPC(optWhichFonts).0 != 0 {
                    SetContextFont(TinyFont);
                } else {
                    SetContextFont(TinyFontBold);
                }
                uqm_lua_checked!(ScreenTransition(optScrTrans, ::core::ptr::null::<RECT>()), BOOLEAN(0));
                UnbatchGraphics();
                uqm_lua_checked!(PlayMusicResume((*pMS).hMusic, 160 as BYTE), BOOLEAN(0));
                uqm_lua_checked!(ShowCombatShip(
                    pMS,
                    !(0 as ::core::ffi::c_int) as COUNT,
                    ::core::ptr::null_mut::<SHIP_FRAGMENT>(),
                ), BOOLEAN(0));
                SetInputCallback(Some(on_input_frame as unsafe extern "C" fn() -> ()));
                uqm_lua_checked!(SetFlashRect(
                    ::core::ptr::with_exposed_provenance_mut::<
                        RECT,
                    >(!(0 as ::core::ffi::c_long) as usize),
                    BOOLEAN::FALSE,
                ), BOOLEAN(0));
                (*pMS).Initialized = BOOLEAN::TRUE.0 as ::core::ffi::c_int as SIZE;
            } else if cancel.0 != 0
                || select.0 != 0
                    && (*pMS).CurState as ::core::ffi::c_int
                        == C2Rust_Unnamed_10::SHIPYARD_EXIT.0 as ::core::ffi::c_int
                || GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
                    & C2Rust_Unnamed_5::CHECK_LOAD.0 as ::core::ffi::c_int != 0
            {
                break '_ExitShipyard;
            } else if select.0 != 0 {
                if (*pMS).CurState as ::core::ffi::c_int
                    != C2Rust_Unnamed_10::SHIPYARD_SAVELOAD.0 as ::core::ffi::c_int
                {
                    (*pMS).Initialized = BOOLEAN::FALSE.0 as ::core::ffi::c_int as SIZE;
                    uqm_lua_checked!(DrawMenuStateStrings(
                        C2Rust_Unnamed_6::PM_CREW.0 as ::core::ffi::c_int as BYTE,
                        (*pMS).CurState as SWORD,
                    ), BOOLEAN(0));
                    uqm_lua_checked!(DoModifyShips(pMS), BOOLEAN(0));
                } else if uqm_lua_checked!(GameOptions(), BOOLEAN(0)).0 == 0 {
                    break '_ExitShipyard;
                } else {
                    uqm_lua_checked!(DrawMenuStateStrings(
                        C2Rust_Unnamed_6::PM_CREW.0 as ::core::ffi::c_int as BYTE,
                        (*pMS).CurState as SWORD,
                    ), BOOLEAN(0));
                    uqm_lua_checked!(SetFlashRect(
                        ::core::ptr::with_exposed_provenance_mut::<
                            RECT,
                        >(!(0 as ::core::ffi::c_long) as usize),
                        BOOLEAN::FALSE,
                    ), BOOLEAN(0));
                }
            } else {
                uqm_lua_checked!(DoMenuChooser(
                    pMS,
                    C2Rust_Unnamed_6::PM_CREW.0 as ::core::ffi::c_int as BYTE,
                ), BOOLEAN(0));
            }
            if optInfiniteRU.0 != 0 {
                GlobData.SIS_state.ResUnits = 1000000 as ::core::ffi::c_long as DWORD;
            }
            return BOOLEAN::TRUE;
        }
    }
    SetInputCallback(None);
    if ((*pMS).CurState as ::core::ffi::c_int)
        < C2Rust_Unnamed_10::SHIPYARD_EXIT.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(DrawMenuStateStrings(
            C2Rust_Unnamed_6::PM_CREW.0 as ::core::ffi::c_int as BYTE,
            C2Rust_Unnamed_10::SHIPYARD_EXIT.0 as ::core::ffi::c_int as SWORD,
        ), BOOLEAN(0));
    }
    DestroyDrawable(ReleaseDrawable((*pMS).ModuleFrame));
    (*pMS).ModuleFrame = ::core::ptr::null_mut::<FRAME_DESC>();
    DestroyStringTable(ReleaseStringTable((*pMS).CurString));
    (*pMS).CurString = ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>();
    if if optWindowType == 0 as DWORD {
        BOOLEAN::TRUE.0 as ::core::ffi::c_int
    } else {
        BOOLEAN::FALSE.0 as ::core::ffi::c_int
    } == 0
    {
        DestroyFont(ModuleFont);
    }
    uqm_lua_checked!(SetMusicPosition(), BOOLEAN(0));
    return BOOLEAN::FALSE;
}
#[inline]
unsafe extern "C" fn LockMasterShip(
    mut pq: *const QUEUE,
    mut h: HMASTERSHIP,
) -> *mut MASTER_SHIP_INFO {
    ((*pq).object_size as usize == ::core::mem::size_of::<MASTER_SHIP_INFO>()
        || {
            __assert_fail(
                b"GetLinkSize (pq) == sizeof (MASTER_SHIP_INFO)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/uqm/master.h\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                50 as ::core::ffi::c_int,
                b"LockMasterShip\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    return LockLink(pq, h) as *mut MASTER_SHIP_INFO;
}
