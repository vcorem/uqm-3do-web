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
    pub type frame_desc;
    pub type font_desc;
    pub type drawable_desc;
    pub type tfb_soundsample;
    pub type FlashContext;
    fn SetContext(Context: CONTEXT) -> CONTEXT;
    fn SetContextForeGroundColor(Color: Color) -> Color;
    fn SetContextBackGroundColor(Color: Color) -> Color;
    fn SetContextClipRect(pRect: *mut RECT) -> BOOLEAN;
    fn GetContextClipRect(pRect: *mut RECT) -> BOOLEAN;
    fn DrawStamp(pStamp: *mut STAMP);
    fn DrawFilledStamp(pStamp: *mut STAMP);
    fn DrawPoint(pPoint: *mut POINT);
    fn DrawFilledRectangle(pRect: *mut RECT);
    fn font_DrawText(pText: *mut TEXT);
    fn BatchGraphics();
    fn UnbatchGraphics();
    fn ClearDrawable();
    fn DestroyDrawable(Drawable: DRAWABLE) -> BOOLEAN;
    fn LoadGraphicInstance(res: RESOURCE) -> *mut ::core::ffi::c_void;
    fn SetContextFont(Font: FONT) -> FONT;
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
    );
    fn sprintf(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
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
    fn SetRelStringTableIndex(String: STRING, StringTableOffs: SIZE) -> STRING;
    fn GetStringAddress(String: STRING) -> STRINGPTR;
    static mut ScreenWidth: ::core::ffi::c_int;
    static mut ScreenHeight: ::core::ffi::c_int;
    fn FreeLink(pq: *mut QUEUE, hLink: HLINK);
    fn InsertQueue(pq: *mut QUEUE, hLink: HLINK, hRefLink: HLINK);
    fn RemoveQueue(pq: *mut QUEUE, hLink: HLINK);
    fn CountLinks(pq: *mut QUEUE) -> COUNT;
    fn LoadMusicInstance(res: RESOURCE) -> MUSIC_REF;
    fn DoShipSpin(index: COUNT, hMusic: MUSIC_REF);
    static mut GameStrings: STRING;
    fn GameOptions() -> BOOLEAN;
    fn DoMenuChooser(pMS: *mut MENU_STATE, BaseState: BYTE) -> BOOLEAN;
    fn DrawMenuStateStrings(beg_index: BYTE, NewState: SWORD);
    static mut PulsedInputState: CONTROLLER_INPUT_STATE;
    fn FlushInput();
    fn SetInputCallback(_: Option<InputFrameCallback>) -> Option<InputFrameCallback>;
    fn GetTimeCounter() -> TimeCount;
    fn CloneShipFragment(
        shipIndex: COUNT,
        pDstQueue: *mut QUEUE,
        crew_level: COUNT,
    ) -> HSHIPFRAG;
    fn GetStarShipFromIndex(pShipQ: *mut QUEUE, Index: COUNT) -> HLINK;
    fn GetEscortByStarShipIndex(index: COUNT) -> HSHIPFRAG;
    fn GetIndexFromStarShip(pShipQ: *mut QUEUE, hStarShip: HLINK) -> COUNT;
    fn CheckAlliance(race: COUNT) -> COUNT;
    fn FindMasterShipIndex(ship_ref: SPECIES_ID) -> ::core::ffi::c_int;
    fn PlayMusic(MusicRef: MUSIC_REF, Continuous: BOOLEAN, Priority: BYTE);
    fn DrawShipPiece(
        ModuleFrame: FRAME,
        which_piece: COUNT,
        which_slot: COUNT,
        DrawBluePrint_0: BOOLEAN,
    );
    fn SleepThread(timePeriod: TimePeriod);
    fn SleepThreadUntil(wakeTime: TimeCount);
    fn DrawSISFrame();
    fn ClearSISRect(ClearFlags: BYTE);
    fn SetFlashRect(pRect: *const RECT);
    fn PreUpdateFlashRect();
    fn PostUpdateFlashRect();
    fn DrawSISTitle(pStr: *mut UNICODE);
    fn DrawSISMessage(pStr: *const UNICODE);
    fn GetGaugeRect(pRect: *mut RECT, IsCrewRect: BOOLEAN);
    fn DeltaSISGauges(
        crew_delta: SIZE,
        fuel_delta: SIZE,
        resunit_delta: ::core::ffi::c_int,
    );
    fn GetCrewCount() -> COUNT;
    fn GetCPodCapacity(ppt: *mut POINT) -> COUNT;
    fn GetSBayCapacity(ppt: *mut POINT) -> COUNT;
    fn GetFTankCapacity(ppt: *mut POINT) -> DWORD;
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
    fn SetMenuSounds(sound_0: MENU_SOUND_FLAGS, sound_1: MENU_SOUND_FLAGS);
    fn PlayMenuSound(S: MENU_SOUND_EFFECT);
    fn SetTransitionSource(pRect: *const RECT);
    fn ScreenTransition(transition: ::core::ffi::c_int, pRect: *const RECT);
    fn AnyButtonPress(DetectSpecial: BOOLEAN) -> BOOLEAN;
    static mut ScreenContext: CONTEXT;
    static mut SpaceContext: CONTEXT;
    static mut StatusContext: CONTEXT;
    static mut TinyFont: FONT;
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
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed {
    pub const DEAD_GUY: Self = Self(0);
    pub const GOOD_GUY: Self = Self(1);
    pub const BAD_GUY: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_0 {
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
pub type TimeCount = DWORD;
pub type TimePeriod = DWORD;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_1(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_1 {
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
pub type InputFrameCallback = unsafe extern "C" fn() -> ();
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
    pub flash_frame0: FRAME,
    pub flash_frame1: FRAME,
    pub flashContext: *mut FlashContext,
    pub hMusic: MUSIC_REF,
    pub privData: *mut ::core::ffi::c_void,
}
pub type MENU_STATE = menu_state;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_2(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_2 {
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
    pub const PM_CHANGE_CAPTAIN: Self = Self(36);
    pub const PM_CHANGE_SHIP: Self = Self(37);
    pub const PM_EXIT_SETTINGS: Self = Self(38);
    pub const PM_ALT_SCAN: Self = Self(39);
    pub const PM_ALT_STARMAP: Self = Self(40);
    pub const PM_ALT_MANIFEST: Self = Self(41);
    pub const PM_ALT_GAME_MENU: Self = Self(42);
    pub const PM_ALT_NAVIGATE: Self = Self(43);
    pub const PM_ALT_CARGO: Self = Self(44);
    pub const PM_ALT_DEVICES: Self = Self(45);
    pub const PM_ALT_ROSTER: Self = Self(46);
    pub const PM_ALT_EXIT_MANIFEST: Self = Self(47);
    pub const PM_ALT_MSCAN: Self = Self(48);
    pub const PM_ALT_ESCAN: Self = Self(49);
    pub const PM_ALT_BSCAN: Self = Self(50);
    pub const PM_ALT_ASCAN: Self = Self(51);
    pub const PM_ALT_DISPATCH: Self = Self(52);
    pub const PM_ALT_EXIT_SCAN: Self = Self(53);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_3(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_3 {
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_4(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_4 {
    pub const TALK_COMMANDER: Self = Self(0);
    pub const OUTFIT_STARSHIP: Self = Self(1);
    pub const SHIPYARD: Self = Self(2);
    pub const DEPART_BASE: Self = Self(3);
}
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
pub struct CLOCK_STATE {
    pub day_index: BYTE,
    pub month_index: BYTE,
    pub year_index: COUNT,
    pub tick_count: SIZE,
    pub day_in_ticks: SIZE,
    pub event_q: QUEUE,
}
pub type ACTIVITY = UWORD;
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
pub struct MENU_SOUND_EFFECT(pub ::core::ffi::c_uint);
impl MENU_SOUND_EFFECT {
    pub const MENU_SOUND_MOVE: Self = Self(0);
    pub const MENU_SOUND_SUCCESS: Self = Self(1);
    pub const MENU_SOUND_FAILURE: Self = Self(2);
    pub const MENU_SOUND_INVOKED: Self = Self(3);
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
pub struct C2Rust_Unnamed_5 {
    pub StarShipPtr: *mut SHIP_FRAGMENT,
    pub finished_s: POINT,
    pub ship_s: STAMP,
    pub lfdoor_s: STAMP,
    pub rtdoor_s: STAMP,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_6(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_6 {
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_7(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_7 {
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
pub struct C2Rust_Unnamed_8(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_8 {
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
pub struct C2Rust_Unnamed_9(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_9 {
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
                329 as ::core::ffi::c_int,
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
                395 as ::core::ffi::c_int,
                b"LockFleetInfo\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    return LockLink(pq, h) as *mut FLEET_INFO;
}
static mut hangar_x_coords: [COORD; 6] = [
    0 as COORD,
    38 as COORD,
    76 as COORD,
    131 as COORD,
    169 as COORD,
    207 as COORD,
];
static mut ShipCost: [COUNT; 23] = [
    1600 as COUNT,
    3000 as COUNT,
    1100 as COUNT,
    2300 as COUNT,
    2000 as COUNT,
    500 as COUNT,
    1800 as COUNT,
    1600 as COUNT,
    1000 as COUNT,
    2200 as COUNT,
    1200 as COUNT,
    2300 as COUNT,
    3600 as COUNT,
    1700 as COUNT,
    1000 as COUNT,
    2100 as COUNT,
    4400 as COUNT,
    700 as COUNT,
    3000 as COUNT,
    600 as COUNT,
    1300 as COUNT,
    3000 as COUNT,
    2300 as COUNT,
];
unsafe extern "C" fn animatePowerLines(mut pMS: *mut MENU_STATE) {
    static mut s: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    static mut ColorMap: COLORMAP = ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>();
    static mut NextTime: TimeCount = 0 as TimeCount;
    let mut Now: TimeCount = GetTimeCounter();
    if !pMS.is_null() {
        s.origin.x = 0 as COORD;
        s.origin.y = 0 as COORD;
        s.frame = SetAbsFrameIndex((*pMS).ModuleFrame, 24 as COUNT);
        ColorMap = SetAbsStringTableIndex((*pMS).CurString, 0 as COUNT) as COLORMAP;
    }
    if Now >= NextTime || !pMS.is_null() {
        NextTime = Now
            .wrapping_add(
                (840 as ::core::ffi::c_int / 15 as ::core::ffi::c_int) as TimeCount,
            );
        SetColorMap(GetColorMapAddress(ColorMap));
        DrawStamp(&raw mut s);
        ColorMap = SetRelStringTableIndex(ColorMap, 1 as SIZE) as COLORMAP;
    }
}
unsafe extern "C" fn on_input_frame() {
    let mut oldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    oldContext = SetContext(SpaceContext);
    animatePowerLines(::core::ptr::null_mut::<MENU_STATE>());
    SetContext(oldContext);
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
        DoShipSpin(Index as COUNT, (*pMS).hMusic);
    }
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
            == C2Rust_Unnamed::GOOD_GUY.0 as ::core::ffi::c_int
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
        if (*FleetPtr).allied_state as ::core::ffi::c_int
            == C2Rust_Unnamed::GOOD_GUY.0 as ::core::ffi::c_int
            && {
                let c2rust_fresh1 = Index;
                Index = Index.wrapping_sub(1);
                c2rust_fresh1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int
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
unsafe extern "C" fn DrawRaceStrings(mut pMS: *mut MENU_STATE, mut NewRaceItem: BYTE) {
    let mut r: RECT = RECT {
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
    s.origin.x = (4 as ::core::ffi::c_int
        + (ScreenWidth - 64 as ::core::ffi::c_int - 0 as ::core::ffi::c_int)
        - r.corner.x as ::core::ffi::c_int) as COORD;
    s.origin.y = (10 as ::core::ffi::c_int + 0 as ::core::ffi::c_int
        + (ScreenHeight - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
            - 13 as ::core::ffi::c_int) - 53 as ::core::ffi::c_int
        - r.corner.y as ::core::ffi::c_int) as COORD;
    r.corner.x = (s.origin.x as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as COORD;
    r.corner.y = (s.origin.y as ::core::ffi::c_int - 11 as ::core::ffi::c_int) as COORD;
    r.extent.width = (64 as ::core::ffi::c_int - 8 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as COORD;
    r.extent.height = 11 as COORD;
    BatchGraphics();
    ClearSISRect(((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as BYTE);
    SetContextForeGroundColor(colorFromRgb15(0xa as BYTE, 0xa as BYTE, 0xa as BYTE));
    DrawFilledRectangle(&raw mut r);
    r.corner = s.origin;
    r.extent.width = (64 as ::core::ffi::c_int - 8 as ::core::ffi::c_int) as COORD;
    r.extent.height = 53 as COORD;
    SetContextForeGroundColor(colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE));
    DrawFilledRectangle(&raw mut r);
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
        hStarShip = GetAvailableRaceFromIndex(NewRaceItem);
        NewRaceItem = GetIndexFromStarShip(
            &raw mut GlobData.Game_state.avail_race_q,
            hStarShip,
        ) as BYTE;
        s.frame = SetAbsFrameIndex(
            (*pMS).ModuleFrame,
            (3 as ::core::ffi::c_int + NewRaceItem as ::core::ffi::c_int) as COUNT,
        );
        DrawStamp(&raw mut s);
        FleetPtr = LockFleetInfo(&raw mut GlobData.Game_state.avail_race_q, hStarShip);
        s.frame = (*FleetPtr).melee_icon;
        UnlockLink(&raw mut GlobData.Game_state.avail_race_q, hStarShip);
        t.baseline.x = (s.origin.x as ::core::ffi::c_int
            + (64 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)
            - 2 as ::core::ffi::c_int) as COORD;
        t.baseline.y = (s.origin.y as ::core::ffi::c_int + 53 as ::core::ffi::c_int
            - 2 as ::core::ffi::c_int) as COORD;
        s.origin.x = (s.origin.x as ::core::ffi::c_int
            + (64 as ::core::ffi::c_int - 8 as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int)) as COORD;
        s.origin.y = (s.origin.y as ::core::ffi::c_int
            + (53 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)) as COORD;
        DrawStamp(&raw mut s);
        t.align = TEXT_ALIGN::ALIGN_RIGHT;
        t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
        t.pStr = &raw mut buf as *mut UNICODE;
        sprintf(
            &raw mut buf as *mut ::core::ffi::c_char,
            b"%u\0".as_ptr() as *const ::core::ffi::c_char,
            ShipCost[NewRaceItem as usize] as ::core::ffi::c_int,
        );
        SetContextFont(TinyFont);
        SetContextForeGroundColor(colorFromRgb15(0 as BYTE, 0x1f as BYTE, 0 as BYTE));
        font_DrawText(&raw mut t);
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
    hTemplate = GetStarShipFromIndex(
        &raw mut GlobData.Game_state.avail_race_q,
        (*StarShipPtr).race_id as COUNT,
    ) as HFLEETINFO;
    TemplatePtr = LockFleetInfo(&raw mut GlobData.Game_state.avail_race_q, hTemplate);
    maxCrewLevel = (*TemplatePtr).crew_level;
    UnlockLink(&raw mut GlobData.Game_state.avail_race_q, hTemplate);
    if (*StarShipPtr).crew_level as ::core::ffi::c_int
        >= maxCrewLevel as ::core::ffi::c_int
    {
        sprintf(
            &raw mut buf as *mut ::core::ffi::c_char,
            b"%u\0".as_ptr() as *const ::core::ffi::c_char,
            (*StarShipPtr).crew_level as ::core::ffi::c_int,
        );
    } else if (*StarShipPtr).crew_level as ::core::ffi::c_int == 0 as ::core::ffi::c_int
    {
        utf8StringCopy(
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[UNICODE; 80]>(),
            b"SCRAP\0".as_ptr() as *const ::core::ffi::c_char,
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
        + r.extent.height as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as COORD;
    t.align = TEXT_ALIGN::ALIGN_CENTER;
    t.pStr = &raw mut buf as *mut UNICODE;
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    if r.corner.y != 0 {
        r.corner.y = (t.baseline.y as ::core::ffi::c_int - 6 as ::core::ffi::c_int)
            as COORD;
        r.extent.width = 34 as COORD;
        r.extent.height = 6 as COORD;
        SetContextForeGroundColor(colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE));
        DrawFilledRectangle(&raw mut r);
    }
    SetContextForeGroundColor(
        if (*StarShipPtr).crew_level as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            colorFromRgb15(0 as BYTE, 0x14 as BYTE, 0 as BYTE)
        } else {
            colorFromRgb15(0x12 as BYTE, 0 as BYTE, 0 as BYTE)
        },
    );
    font_DrawText(&raw mut t);
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
    let mut ship_win_info: [C2Rust_Unnamed_5; 12] = [C2Rust_Unnamed_5 {
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
    let mut pship_win_info: *mut C2Rust_Unnamed_5 = ::core::ptr::null_mut::<
        C2Rust_Unnamed_5,
    >();
    num_ships = 1 as COUNT;
    pship_win_info = (&raw mut ship_win_info as *mut C2Rust_Unnamed_5).offset(0isize);
    if !YankedStarShipPtr.is_null() {
        (*pship_win_info).StarShipPtr = YankedStarShipPtr;
        (*pship_win_info).lfdoor_s.origin.x = -(34 as ::core::ffi::c_int
            >> 1 as ::core::ffi::c_int) as COORD;
        (*pship_win_info).rtdoor_s.origin.x = (34 as ::core::ffi::c_int
            >> 1 as ::core::ffi::c_int) as COORD;
        (*pship_win_info).lfdoor_s.origin.y = 0 as COORD;
        (*pship_win_info).rtdoor_s.origin.y = 0 as COORD;
        (*pship_win_info).lfdoor_s.frame = IncFrameIndex((*pMS).ModuleFrame);
        (*pship_win_info).rtdoor_s.frame = IncFrameIndex(
            (*pship_win_info).lfdoor_s.frame,
        );
        (*pship_win_info).ship_s.origin.x = ((34 as ::core::ffi::c_int
            >> 1 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int) as COORD;
        (*pship_win_info).ship_s.origin.y = (34 as ::core::ffi::c_int
            >> 1 as ::core::ffi::c_int) as COORD;
        (*pship_win_info).ship_s.frame = (*YankedStarShipPtr).melee_icon;
        (*pship_win_info).finished_s.x = hangar_x_coords[(which_window
            as ::core::ffi::c_int % 6 as ::core::ffi::c_int) as usize];
        (*pship_win_info).finished_s.y = (88 as ::core::ffi::c_int
            + 84 as ::core::ffi::c_int
                * (which_window as ::core::ffi::c_int / 6 as ::core::ffi::c_int))
            as COORD;
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
            (*pship_win_info).lfdoor_s.origin.x = -1 as COORD;
            (*pship_win_info).rtdoor_s.origin.x = 1 as COORD;
            (*pship_win_info).lfdoor_s.origin.y = 0 as COORD;
            (*pship_win_info).rtdoor_s.origin.y = 0 as COORD;
            (*pship_win_info).lfdoor_s.frame = IncFrameIndex((*pMS).ModuleFrame);
            (*pship_win_info).rtdoor_s.frame = IncFrameIndex(
                (*pship_win_info).lfdoor_s.frame,
            );
            (*pship_win_info).ship_s.origin.x = ((34 as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int) as COORD;
            (*pship_win_info).ship_s.origin.y = (34 as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int) as COORD;
            (*pship_win_info).ship_s.frame = (*StarShipPtr).melee_icon;
            which_window = (*StarShipPtr).index as COUNT;
            (*pship_win_info).finished_s.x = hangar_x_coords[(which_window
                as ::core::ffi::c_int % 6 as ::core::ffi::c_int) as usize];
            (*pship_win_info).finished_s.y = (88 as ::core::ffi::c_int
                + 84 as ::core::ffi::c_int
                    * (which_window as ::core::ffi::c_int / 6 as ::core::ffi::c_int))
                as COORD;
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
        r.extent.width = 34 as COORD;
        r.extent.height = (34 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as COORD;
        FlushInput();
        TimeIn = GetTimeCounter() as DWORD;
        j = 0 as ::core::ffi::c_int;
        while j
            < (34 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
                + 1 as ::core::ffi::c_int && AllDoorsFinished.0 == 0
        {
            SleepThreadUntil(
                TimeIn
                    .wrapping_add(
                        (840 as ::core::ffi::c_int / 24 as ::core::ffi::c_int)
                            as TimeCount,
                    ),
            );
            TimeIn = GetTimeCounter() as DWORD;
            if AnyButtonPress(BOOLEAN::FALSE).0 != 0 {
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
            pship_win_info = (&raw mut ship_win_info as *mut C2Rust_Unnamed_5)
                .offset(0isize);
            i = 0 as COUNT;
            while (i as ::core::ffi::c_int) < num_ships as ::core::ffi::c_int {
                let mut ClipRect: RECT = RECT {
                    corner: POINT { x: 0, y: 0 },
                    extent: EXTENT { width: 0, height: 0 },
                };
                ClipRect.corner.x = (7 as ::core::ffi::c_int + 0 as ::core::ffi::c_int
                    + (*pship_win_info).finished_s.x as ::core::ffi::c_int) as COORD;
                ClipRect.corner.y = (10 as ::core::ffi::c_int + 0 as ::core::ffi::c_int
                    + (*pship_win_info).finished_s.y as ::core::ffi::c_int) as COORD;
                ClipRect.extent.width = 34 as COORD;
                ClipRect.extent.height = (34 as ::core::ffi::c_int
                    + 6 as ::core::ffi::c_int) as COORD;
                SetContextClipRect(&raw mut ClipRect);
                ClearDrawable();
                DrawStamp(&raw mut (*pship_win_info).ship_s);
                ShowShipCrew((*pship_win_info).StarShipPtr, &raw mut r);
                if AllDoorsFinished.0 == 0 || !YankedStarShipPtr.is_null() {
                    DrawStamp(&raw mut (*pship_win_info).lfdoor_s);
                    DrawStamp(&raw mut (*pship_win_info).rtdoor_s);
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
            animatePowerLines(::core::ptr::null_mut::<MENU_STATE>());
            UnbatchGraphics();
            SetContext(OldContext);
            j += 1;
        }
    }
}
unsafe extern "C" fn CrewTransaction(mut crew_delta: SIZE) {
    if crew_delta != 0 {
        let mut crew_bought: SIZE = 0;
        crew_bought = ((((getGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_7::CREW_PURCHASED1.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_7::END_CREW_PURCHASED1.0 as ::core::ffi::c_int,
        ) as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as UWORD
            as ::core::ffi::c_int
            | getGameState(
                &raw mut GlobData.Game_state.GameState as *mut BYTE,
                C2Rust_Unnamed_7::CREW_PURCHASED0.0 as ::core::ffi::c_int,
                C2Rust_Unnamed_7::END_CREW_PURCHASED0.0 as ::core::ffi::c_int,
            ) as ::core::ffi::c_int) as SIZE as ::core::ffi::c_int
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
                DrawMenuStateStrings(
                    C2Rust_Unnamed_2::PM_CREW.0 as ::core::ffi::c_int as BYTE,
                    C2Rust_Unnamed_9::SHIPYARD_CREW.0 as ::core::ffi::c_int as SWORD,
                );
            }
        } else if (crew_bought as ::core::ffi::c_int) < 1000 as ::core::ffi::c_int
            && crew_bought as ::core::ffi::c_int - crew_delta as ::core::ffi::c_int
                >= 1000 as ::core::ffi::c_int
        {
            GlobData.Game_state.CrewCost = (GlobData.Game_state.CrewCost
                as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as BYTE;
            DrawMenuStateStrings(
                C2Rust_Unnamed_2::PM_CREW.0 as ::core::ffi::c_int as BYTE,
                C2Rust_Unnamed_9::SHIPYARD_CREW.0 as ::core::ffi::c_int as SWORD,
            );
        }
        if CheckAlliance(
            C2Rust_Unnamed_0::SHOFIXTI_SHIP.0 as ::core::ffi::c_int as COUNT,
        ) as ::core::ffi::c_int != C2Rust_Unnamed::GOOD_GUY.0 as ::core::ffi::c_int
        {
            setGameState(
                &raw mut GlobData.Game_state.GameState as *mut BYTE,
                C2Rust_Unnamed_7::CREW_PURCHASED0.0 as ::core::ffi::c_int,
                C2Rust_Unnamed_7::END_CREW_PURCHASED0.0 as ::core::ffi::c_int,
                crew_bought as UWORD as BYTE,
            );
            setGameState(
                &raw mut GlobData.Game_state.GameState as *mut BYTE,
                C2Rust_Unnamed_7::CREW_PURCHASED1.0 as ::core::ffi::c_int,
                C2Rust_Unnamed_7::END_CREW_PURCHASED1.0 as ::core::ffi::c_int,
                (crew_bought as UWORD as ::core::ffi::c_int >> 8 as ::core::ffi::c_int)
                    as BYTE,
            );
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
    r.extent.width = (ScreenWidth - 64 as ::core::ffi::c_int
        - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int - 14 as ::core::ffi::c_int)
        as COORD;
    r.extent.height = 61 as COORD;
    SetFlashRect(&raw mut r);
}
unsafe extern "C" fn DMS_GetEscortShipRect(mut rOut: *mut RECT, mut slotNr: BYTE) {
    let mut row: BYTE = (slotNr as ::core::ffi::c_int / 6 as ::core::ffi::c_int) as BYTE;
    let mut col: BYTE = (slotNr as ::core::ffi::c_int % 6 as ::core::ffi::c_int) as BYTE;
    (*rOut).corner.x = hangar_x_coords[col as usize];
    (*rOut).corner.y = (88 as ::core::ffi::c_int
        + 84 as ::core::ffi::c_int * row as ::core::ffi::c_int) as COORD;
    (*rOut).extent.width = 34 as COORD;
    (*rOut).extent.height = (34 as ::core::ffi::c_int + 6 as ::core::ffi::c_int)
        as COORD;
}
unsafe extern "C" fn DMS_FlashEscortShip(mut slotNr: BYTE) {
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    DMS_GetEscortShipRect(&raw mut r, slotNr);
    SetFlashRect(&raw mut r);
}
unsafe extern "C" fn DMS_FlashFlagShipCrewCount() {
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    SetContext(StatusContext);
    GetGaugeRect(&raw mut r, BOOLEAN::TRUE);
    SetFlashRect(&raw mut r);
    SetContext(SpaceContext);
}
unsafe extern "C" fn DMS_FlashEscortShipCrewCount(mut slotNr: BYTE) {
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut row: BYTE = (slotNr as ::core::ffi::c_int / 6 as ::core::ffi::c_int) as BYTE;
    let mut col: BYTE = (slotNr as ::core::ffi::c_int % 6 as ::core::ffi::c_int) as BYTE;
    r.corner.x = hangar_x_coords[col as usize];
    r.corner.y = (88 as ::core::ffi::c_int
        + 84 as ::core::ffi::c_int * row as ::core::ffi::c_int
        + (34 as ::core::ffi::c_int + 6 as ::core::ffi::c_int - 6 as ::core::ffi::c_int))
        as COORD;
    r.extent.width = 34 as COORD;
    r.extent.height = 5 as COORD;
    SetContext(SpaceContext);
    SetFlashRect(&raw mut r);
}
unsafe extern "C" fn DMS_FlashActiveShip(mut pMS: *mut MENU_STATE) {
    if ((*pMS).CurState as ::core::ffi::c_int
        >> 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE != 0
    {
        DMS_FlashFlagShip();
    } else {
        DMS_FlashEscortShip((*pMS).CurState);
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
            DMS_FlashActiveShip(pMS);
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
            SetFlashRect(
                ::core::ptr::with_exposed_provenance_mut::<
                    RECT,
                >(!(1 as ::core::ffi::c_long) as usize),
            );
        }
        DMS_Mode::DMS_Mode_editCrew => {
            SetMenuSounds(
                (((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int)
                    as MENU_SOUND_FLAGS as ::core::ffi::c_int
                    | ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
                        as MENU_SOUND_FLAGS as ::core::ffi::c_int) as MENU_SOUND_FLAGS,
                (((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                    as MENU_SOUND_FLAGS as ::core::ffi::c_int
                    | ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int)
                        as MENU_SOUND_FLAGS as ::core::ffi::c_int) as MENU_SOUND_FLAGS,
            );
            if ((*pMS).CurState as ::core::ffi::c_int
                >> 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE != 0
            {
                DMS_FlashFlagShipCrewCount();
            } else {
                DMS_FlashEscortShipCrewCount((*pMS).CurState);
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
            SetFlashRect(
                ::core::ptr::with_exposed_provenance_mut::<
                    RECT,
                >(!(0 as ::core::ffi::c_long) as usize),
            );
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
    if hStarShip.is_null() {
        hSpinShip = GetAvailableRaceFromIndex((*pMS).delta_item as UWORD as BYTE);
        if hSpinShip.is_null() {
            return BOOLEAN::FALSE;
        }
    } else {
        let mut FragPtr: *mut SHIP_FRAGMENT = LockShipFrag(
            &raw mut GlobData.Game_state.built_ship_q,
            hStarShip,
        );
        hSpinShip = GetStarShipFromIndex(
            &raw mut GlobData.Game_state.avail_race_q,
            (*FragPtr).race_id as COUNT,
        ) as HFLEETINFO;
        UnlockLink(&raw mut GlobData.Game_state.built_ship_q, hStarShip);
    }
    SetFlashRect(::core::ptr::null::<RECT>());
    OldContext = SetContext(ScreenContext);
    GetContextClipRect(&raw mut OldClipRect);
    SpinStarShip(pMS, hSpinShip);
    SetContextClipRect(&raw mut OldClipRect);
    SetContext(OldContext);
    return BOOLEAN::TRUE;
}
unsafe extern "C" fn DMS_HireFlagShipCrew() -> SIZE {
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    if GetCPodCapacity(&raw mut r.corner) as ::core::ffi::c_int
        <= GetCrewCount() as ::core::ffi::c_int
    {
        return 0 as SIZE;
    }
    if GlobData.SIS_state.ResUnits < GlobData.Game_state.CrewCost as DWORD {
        return 0 as SIZE;
    }
    DrawPoint(&raw mut r.corner);
    PreUpdateFlashRect();
    DeltaSISGauges(
        1 as SIZE,
        0 as SIZE,
        -(GlobData.Game_state.CrewCost as ::core::ffi::c_int),
    );
    PostUpdateFlashRect();
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
    crew_bought = (((getGameState(
        &raw mut GlobData.Game_state.GameState as *mut BYTE,
        C2Rust_Unnamed_7::CREW_PURCHASED1.0 as ::core::ffi::c_int,
        C2Rust_Unnamed_7::END_CREW_PURCHASED1.0 as ::core::ffi::c_int,
    ) as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as UWORD as ::core::ffi::c_int
        | getGameState(
            &raw mut GlobData.Game_state.GameState as *mut BYTE,
            C2Rust_Unnamed_7::CREW_PURCHASED0.0 as ::core::ffi::c_int,
            C2Rust_Unnamed_7::END_CREW_PURCHASED0.0 as ::core::ffi::c_int,
        ) as ::core::ffi::c_int) as SIZE;
    PreUpdateFlashRect();
    DeltaSISGauges(
        -1 as SIZE,
        0 as SIZE,
        GlobData.Game_state.CrewCost as ::core::ffi::c_int
            - if crew_bought as ::core::ffi::c_int == 1000 as ::core::ffi::c_int {
                2 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            },
    );
    PostUpdateFlashRect();
    GetCPodCapacity(&raw mut r.corner);
    SetContextForeGroundColor(colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE));
    DrawPoint(&raw mut r.corner);
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
    let mut hTemplate: HFLEETINFO = GetStarShipFromIndex(
        &raw mut GlobData.Game_state.avail_race_q,
        (*StarShipPtr).race_id as COUNT,
    );
    let mut TemplatePtr: *mut FLEET_INFO = LockFleetInfo(
        &raw mut GlobData.Game_state.avail_race_q,
        hTemplate,
    );
    templateMaxCrew = (*TemplatePtr).crew_level;
    UnlockLink(&raw mut GlobData.Game_state.avail_race_q, hTemplate);
    if GlobData.SIS_state.ResUnits < GlobData.Game_state.CrewCost as DWORD {
        return 0 as SIZE;
    }
    if (*StarShipPtr).crew_level as ::core::ffi::c_int
        >= (*StarShipPtr).max_crew as ::core::ffi::c_int
    {
        return 0 as SIZE;
    }
    if (*StarShipPtr).crew_level as ::core::ffi::c_int
        >= templateMaxCrew as ::core::ffi::c_int
    {
        return 0 as SIZE;
    }
    if (*StarShipPtr).crew_level as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
        DeltaSISGauges(
            0 as SIZE,
            0 as SIZE,
            -(GlobData.Game_state.CrewCost as ::core::ffi::c_int),
        );
    } else {
        DeltaSISGauges(
            0 as SIZE,
            0 as SIZE,
            -(ShipCost[(*StarShipPtr).race_id as usize] as ::core::ffi::c_int),
        );
    }
    (*StarShipPtr).crew_level = (*StarShipPtr).crew_level.wrapping_add(1);
    PreUpdateFlashRect();
    DMS_GetEscortShipRect(&raw mut r, (*StarShipPtr).index);
    ShowShipCrew(StarShipPtr, &raw mut r);
    PostUpdateFlashRect();
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
    if (*StarShipPtr).crew_level as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
        if (*StarShipPtr).crew_level as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
            let mut crew_bought: SIZE = (((getGameState(
                &raw mut GlobData.Game_state.GameState as *mut BYTE,
                C2Rust_Unnamed_7::CREW_PURCHASED1.0 as ::core::ffi::c_int,
                C2Rust_Unnamed_7::END_CREW_PURCHASED1.0 as ::core::ffi::c_int,
            ) as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as UWORD
                as ::core::ffi::c_int
                | getGameState(
                    &raw mut GlobData.Game_state.GameState as *mut BYTE,
                    C2Rust_Unnamed_7::CREW_PURCHASED0.0 as ::core::ffi::c_int,
                    C2Rust_Unnamed_7::END_CREW_PURCHASED0.0 as ::core::ffi::c_int,
                ) as ::core::ffi::c_int) as SIZE;
            DeltaSISGauges(
                0 as SIZE,
                0 as SIZE,
                GlobData.Game_state.CrewCost as ::core::ffi::c_int
                    - if crew_bought as ::core::ffi::c_int == 1000 as ::core::ffi::c_int
                    {
                        2 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    },
            );
        } else {
            DeltaSISGauges(
                0 as SIZE,
                0 as SIZE,
                ShipCost[(*StarShipPtr).race_id as usize] as ::core::ffi::c_int,
            );
        }
        crew_delta = -1 as SIZE;
        (*StarShipPtr).crew_level = (*StarShipPtr).crew_level.wrapping_sub(1);
    } else {
        PlayMenuSound(MENU_SOUND_EFFECT::MENU_SOUND_FAILURE);
    }
    PreUpdateFlashRect();
    DMS_GetEscortShipRect(&raw mut r, (*StarShipPtr).index);
    ShowShipCrew(StarShipPtr, &raw mut r);
    PostUpdateFlashRect();
    return crew_delta;
}
unsafe extern "C" fn DMS_ModifyCrew(
    mut pMS: *mut MENU_STATE,
    mut hStarShip: HSHIPFRAG,
    mut dy: SBYTE,
) {
    let mut crew_delta: SIZE = 0 as SIZE;
    let mut StarShipPtr: *mut SHIP_FRAGMENT = ::core::ptr::null_mut::<SHIP_FRAGMENT>();
    if !hStarShip.is_null() {
        StarShipPtr = LockShipFrag(&raw mut GlobData.Game_state.built_ship_q, hStarShip);
    }
    if hStarShip.is_null() {
        if (dy as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
            crew_delta = DMS_HireFlagShipCrew();
        } else {
            crew_delta = DMS_DismissFlagShipCrew();
        }
        if crew_delta as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            DMS_FlashFlagShipCrewCount();
        }
    } else {
        if (dy as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
            crew_delta = DMS_HireEscortShipCrew(StarShipPtr);
        } else {
            crew_delta = DMS_DismissEscortShipCrew(StarShipPtr);
        }
        if crew_delta as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            DMS_FlashEscortShipCrewCount((*StarShipPtr).index);
        }
    }
    if crew_delta as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        PlayMenuSound(MENU_SOUND_EFFECT::MENU_SOUND_FAILURE);
    }
    if !hStarShip.is_null() {
        UnlockLink(&raw mut GlobData.Game_state.built_ship_q, hStarShip);
        (*pMS).delta_item = ((*pMS).delta_item as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as SIZE;
    }
    CrewTransaction(crew_delta);
}
unsafe extern "C" fn DMS_TryAddEscortShip(mut pMS: *mut MENU_STATE) {
    let mut shipInfo: HFLEETINFO = GetAvailableRaceFromIndex(
        (*pMS).delta_item as UWORD as BYTE,
    );
    let mut Index: COUNT = GetIndexFromStarShip(
        &raw mut GlobData.Game_state.avail_race_q,
        shipInfo,
    );
    if GlobData.SIS_state.ResUnits >= ShipCost[Index as usize] as DWORD
        && !CloneShipFragment(
                Index,
                &raw mut GlobData.Game_state.built_ship_q,
                1 as COUNT,
            )
            .is_null()
    {
        ShowCombatShip(
            pMS,
            (*pMS).CurState as COUNT,
            ::core::ptr::null_mut::<SHIP_FRAGMENT>(),
        );
        DrawMenuStateStrings(
            C2Rust_Unnamed_2::PM_CREW.0 as ::core::ffi::c_int as BYTE,
            C2Rust_Unnamed_9::SHIPYARD_CREW.0 as ::core::ffi::c_int as SWORD,
        );
        DeltaSISGauges(
            0x7fff as SIZE,
            0x7fff as SIZE,
            -(ShipCost[Index as usize] as ::core::ffi::c_int),
        );
        DMS_SetMode(pMS, DMS_Mode::DMS_Mode_editCrew);
    } else {
        PlayMenuSound(MENU_SOUND_EFFECT::MENU_SOUND_FAILURE);
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
                986 as ::core::ffi::c_int,
                b"DMS_AddEscortShip\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    if special.0 != 0 {
        let mut hStarShip: HSHIPFRAG = GetEscortByStarShipIndex(
            (*pMS).delta_item as COUNT,
        );
        if DMS_SpinShip(pMS, hStarShip).0 != 0 {
            DMS_SetMode(pMS, DMS_Mode::DMS_Mode_addEscort);
        }
        return;
    }
    if cancel.0 != 0 {
        (*pMS).delta_item = ((*pMS).delta_item as ::core::ffi::c_int
            & !((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)) as SIZE;
        SetFlashRect(::core::ptr::null::<RECT>());
        DrawMenuStateStrings(
            C2Rust_Unnamed_2::PM_CREW.0 as ::core::ffi::c_int as BYTE,
            C2Rust_Unnamed_9::SHIPYARD_CREW.0 as ::core::ffi::c_int as SWORD,
        );
        DMS_SetMode(pMS, DMS_Mode::DMS_Mode_navigate);
    } else if select.0 != 0 {
        DMS_TryAddEscortShip(pMS);
    } else if dx as ::core::ffi::c_int != 0 || dy as ::core::ffi::c_int != 0 {
        let mut availableCount: COUNT = GetAvailableRaceCount();
        let mut currentShip: BYTE = (*pMS).delta_item as UWORD as BYTE;
        if (dx as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
            || (dy as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
        {
            let c2rust_fresh0 = currentShip;
            currentShip = currentShip.wrapping_sub(1);
            if c2rust_fresh0 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                currentShip = (availableCount as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as BYTE;
            }
        } else if dx as ::core::ffi::c_int > 0 as ::core::ffi::c_int
            || dy as ::core::ffi::c_int > 0 as ::core::ffi::c_int
        {
            currentShip = currentShip.wrapping_add(1);
            if currentShip as ::core::ffi::c_int == availableCount as ::core::ffi::c_int
            {
                currentShip = 0 as BYTE;
            }
        }
        if currentShip as ::core::ffi::c_int
            != (*pMS).delta_item as UWORD as BYTE as ::core::ffi::c_int
        {
            PreUpdateFlashRect();
            DrawRaceStrings(pMS, currentShip);
            PostUpdateFlashRect();
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
    let mut slotNr: BYTE = 0;
    SetFlashRect(::core::ptr::null::<RECT>());
    ShowCombatShip(pMS, (*pMS).CurState as COUNT, StarShipPtr);
    slotNr = (*StarShipPtr).index;
    UnlockLink(&raw mut GlobData.Game_state.built_ship_q, hStarShip);
    RemoveQueue(&raw mut GlobData.Game_state.built_ship_q, hStarShip);
    FreeLink(&raw mut GlobData.Game_state.built_ship_q, hStarShip);
    DeltaSISGauges(0x7fff as SIZE, 0x7fff as SIZE, 0x7fff as ::core::ffi::c_int);
    SetContext(SpaceContext);
    DMS_SetMode(pMS, DMS_Mode::DMS_Mode_navigate);
}
unsafe extern "C" fn DMS_MoveCursor(
    mut curState: BYTE,
    mut dx: SBYTE,
    mut dy: SBYTE,
) -> BYTE {
    let mut row: BYTE = ((curState as ::core::ffi::c_int
        & 0xf as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE
        as ::core::ffi::c_int / 6 as ::core::ffi::c_int) as BYTE;
    let mut col: BYTE = ((curState as ::core::ffi::c_int
        & 0xf as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE
        as ::core::ffi::c_int % 6 as ::core::ffi::c_int) as BYTE;
    let mut isFlagShipSelected: BOOLEAN = BOOLEAN(
        ((curState as ::core::ffi::c_int
            >> 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE
            as ::core::ffi::c_int != 0 as ::core::ffi::c_int) as ::core::ffi::c_int
            as ::core::ffi::c_uint,
    );
    if dy != 0 {
        if isFlagShipSelected.0 != 0 {
            row = (12 as ::core::ffi::c_int / 6 as ::core::ffi::c_int) as BYTE;
        }
        row = ((row as ::core::ffi::c_int
            + (12 as ::core::ffi::c_int / 6 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int) + dy as ::core::ffi::c_int)
            % (12 as ::core::ffi::c_int / 6 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int)) as BYTE;
        isFlagShipSelected = BOOLEAN(
            (row as ::core::ffi::c_int
                == 12 as ::core::ffi::c_int / 6 as ::core::ffi::c_int)
                as ::core::ffi::c_int as ::core::ffi::c_uint,
        );
        if isFlagShipSelected.0 != 0 {
            row = 0 as BYTE;
        }
    } else if dx != 0 {
        if isFlagShipSelected.0 == 0 {
            col = ((col as ::core::ffi::c_int + 6 as ::core::ffi::c_int
                + dx as ::core::ffi::c_int) % 6 as ::core::ffi::c_int) as BYTE;
        }
    }
    return (((if isFlagShipSelected.0 != 0 {
        0xf as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    }) as BYTE as ::core::ffi::c_int)
        << 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int
        | (row as ::core::ffi::c_int * 6 as ::core::ffi::c_int
            + col as ::core::ffi::c_int) as BYTE as ::core::ffi::c_int) as BYTE;
}
unsafe extern "C" fn DMS_EditCrewMode(
    mut pMS: *mut MENU_STATE,
    mut hStarShip: HSHIPFRAG,
    mut select: BOOLEAN,
    mut cancel: BOOLEAN,
    mut dy: SBYTE,
) {
    if select.0 != 0 || cancel.0 != 0 {
        if !hStarShip.is_null() {
            let mut StarShipPtr: *mut SHIP_FRAGMENT = LockShipFrag(
                &raw mut GlobData.Game_state.built_ship_q,
                hStarShip,
            );
            let mut crew_level: COUNT = (*StarShipPtr).crew_level;
            UnlockLink(&raw mut GlobData.Game_state.built_ship_q, hStarShip);
            if crew_level as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                DMS_ScrapEscortShip(pMS, hStarShip);
            }
        }
        (*pMS).delta_item = ((*pMS).delta_item as ::core::ffi::c_int
            & !((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)) as SIZE;
        DMS_SetMode(pMS, DMS_Mode::DMS_Mode_navigate);
    } else if dy != 0 {
        DMS_ModifyCrew(pMS, hStarShip, dy);
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
            DMS_FlashActiveShip(pMS);
        }
    }
    if special.0 != 0 {
        if DMS_SpinShip(pMS, hStarShip).0 != 0 {
            DMS_SetMode(pMS, DMS_Mode::DMS_Mode_navigate);
        }
    } else if select.0 != 0 {
        if hStarShip.is_null()
            && ((*pMS).CurState as ::core::ffi::c_int
                >> 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE
                as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        {
            (*pMS).delta_item = ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
                as SIZE;
            DrawRaceStrings(pMS, 0 as BYTE);
            DMS_SetMode(pMS, DMS_Mode::DMS_Mode_addEscort);
        } else {
            (*pMS).delta_item = ((*pMS).delta_item as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as SIZE;
            DMS_SetMode(pMS, DMS_Mode::DMS_Mode_editCrew);
        }
    } else if cancel.0 != 0 {
        (*pMS).InputFunc = Some(
            DoShipyard as unsafe extern "C" fn(*mut MENU_STATE) -> BOOLEAN,
        ) as Option<unsafe extern "C" fn(*mut menu_state) -> BOOLEAN>;
        (*pMS).CurState = C2Rust_Unnamed_9::SHIPYARD_CREW.0 as ::core::ffi::c_int
            as BYTE;
        DrawMenuStateStrings(
            C2Rust_Unnamed_2::PM_CREW.0 as ::core::ffi::c_int as BYTE,
            (*pMS).CurState as SWORD,
        );
        DMS_SetMode(pMS, DMS_Mode::DMS_Mode_exit);
    }
}
unsafe extern "C" fn DoModifyShips(mut pMS: *mut MENU_STATE) -> BOOLEAN {
    if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
        & C2Rust_Unnamed_8::CHECK_ABORT.0 as ::core::ffi::c_int != 0
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
        DMS_SetMode(pMS, DMS_Mode::DMS_Mode_navigate);
    } else {
        let mut special: BOOLEAN = BOOLEAN(
            (PulsedInputState
                .menu[C2Rust_Unnamed_1::KEY_MENU_SPECIAL.0 as ::core::ffi::c_int
                as usize] != 0 as ::core::ffi::c_int) as ::core::ffi::c_int
                as ::core::ffi::c_uint,
        );
        let mut select: BOOLEAN = BOOLEAN(
            (PulsedInputState
                .menu[C2Rust_Unnamed_1::KEY_MENU_SELECT.0 as ::core::ffi::c_int as usize]
                != 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_uint,
        );
        let mut cancel: BOOLEAN = BOOLEAN(
            (PulsedInputState
                .menu[C2Rust_Unnamed_1::KEY_MENU_CANCEL.0 as ::core::ffi::c_int as usize]
                != 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_uint,
        );
        let mut dx: SBYTE = 0 as SBYTE;
        let mut dy: SBYTE = 0 as SBYTE;
        if PulsedInputState
            .menu[C2Rust_Unnamed_1::KEY_MENU_RIGHT.0 as ::core::ffi::c_int as usize] != 0
        {
            dx = 1 as SBYTE;
        }
        if PulsedInputState
            .menu[C2Rust_Unnamed_1::KEY_MENU_LEFT.0 as ::core::ffi::c_int as usize] != 0
        {
            dx = -1 as SBYTE;
        }
        if PulsedInputState
            .menu[C2Rust_Unnamed_1::KEY_MENU_UP.0 as ::core::ffi::c_int as usize] != 0
        {
            dy = -1 as SBYTE;
        }
        if PulsedInputState
            .menu[C2Rust_Unnamed_1::KEY_MENU_DOWN.0 as ::core::ffi::c_int as usize] != 0
        {
            dy = 1 as SBYTE;
        }
        if (*pMS).delta_item as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int == 0
        {
            DMS_NavigateShipSlots(pMS, special, select, cancel, dx, dy);
        } else {
            let mut hStarShip: HSHIPFRAG = GetEscortByStarShipIndex(
                (*pMS).CurState as COUNT,
            );
            if hStarShip.is_null()
                && ((*pMS).CurState as ::core::ffi::c_int
                    >> 4 as ::core::ffi::c_int as BYTE as ::core::ffi::c_int) as BYTE
                    as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            {
                DMS_AddEscortShip(pMS, special, select, cancel, dx, dy);
            } else {
                DMS_EditCrewMode(pMS, hStarShip, select, cancel, dy);
            }
        }
    }
    SleepThread((840 as ::core::ffi::c_int / 30 as ::core::ffi::c_int) as TimePeriod);
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
        LoadGraphicInstance(b"graphics.blueprints\0".as_ptr() as RESOURCE) as DRAWABLE,
    );
    s.origin.x = 0 as COORD;
    s.origin.y = 0 as COORD;
    s.frame = DecFrameIndex(ModuleFrame);
    SetContextForeGroundColor(colorFromRgb15(0 as BYTE, 0 as BYTE, 0x16 as BYTE));
    DrawFilledStamp(&raw mut s);
    num_frames = 0 as COUNT;
    while (num_frames as ::core::ffi::c_int) < 11 as ::core::ffi::c_int {
        DrawShipPiece(
            ModuleFrame,
            GlobData.SIS_state.DriveSlots[num_frames as usize] as COUNT,
            num_frames,
            BOOLEAN::TRUE,
        );
        num_frames = num_frames.wrapping_add(1);
    }
    num_frames = 0 as COUNT;
    while (num_frames as ::core::ffi::c_int) < 8 as ::core::ffi::c_int {
        DrawShipPiece(
            ModuleFrame,
            GlobData.SIS_state.JetSlots[num_frames as usize] as COUNT,
            num_frames,
            BOOLEAN::TRUE,
        );
        num_frames = num_frames.wrapping_add(1);
    }
    num_frames = 0 as COUNT;
    while (num_frames as ::core::ffi::c_int) < 16 as ::core::ffi::c_int {
        let mut which_piece: BYTE = 0;
        which_piece = GlobData.SIS_state.ModuleSlots[num_frames as usize];
        if !((*pMS).CurState as ::core::ffi::c_int
            == C2Rust_Unnamed_4::SHIPYARD.0 as ::core::ffi::c_int
            && which_piece as ::core::ffi::c_int
                == C2Rust_Unnamed_6::CREW_POD.0 as ::core::ffi::c_int)
        {
            DrawShipPiece(ModuleFrame, which_piece as COUNT, num_frames, BOOLEAN::TRUE);
        }
        num_frames = num_frames.wrapping_add(1);
    }
    SetContextForeGroundColor(colorFromRgb15(0xa as BYTE, 0xa as BYTE, 0x1f as BYTE));
    num_frames = 0 as COUNT;
    while (num_frames as ::core::ffi::c_int) < 16 as ::core::ffi::c_int {
        let mut which_piece_0: BYTE = 0;
        which_piece_0 = GlobData.SIS_state.ModuleSlots[num_frames as usize];
        if (*pMS).CurState as ::core::ffi::c_int
            == C2Rust_Unnamed_4::SHIPYARD.0 as ::core::ffi::c_int
            && which_piece_0 as ::core::ffi::c_int
                == C2Rust_Unnamed_6::CREW_POD.0 as ::core::ffi::c_int
        {
            DrawShipPiece(
                ModuleFrame,
                which_piece_0 as COUNT,
                num_frames,
                BOOLEAN::TRUE,
            );
        }
        num_frames = num_frames.wrapping_add(1);
    }
    num_frames = GlobData.SIS_state.CrewEnlisted;
    GlobData.SIS_state.CrewEnlisted = 0 as COUNT;
    loop {
        let c2rust_fresh2 = num_frames;
        num_frames = num_frames.wrapping_sub(1);
        if c2rust_fresh2 == 0 {
            break;
        }
        let mut pt: POINT = POINT { x: 0, y: 0 };
        GetCPodCapacity(&raw mut pt);
        DrawPoint(&raw mut pt);
        GlobData.SIS_state.CrewEnlisted = GlobData
            .SIS_state
            .CrewEnlisted
            .wrapping_add(1);
    }
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    num_frames = GlobData.SIS_state.TotalElementMass;
    GlobData.SIS_state.TotalElementMass = 0 as COUNT;
    r.extent.width = 9 as COORD;
    r.extent.height = 1 as COORD;
    while num_frames != 0 {
        let mut m: COUNT = 0;
        m = (if (num_frames as ::core::ffi::c_int) < 50 as ::core::ffi::c_int {
            num_frames as ::core::ffi::c_int
        } else {
            50 as ::core::ffi::c_int
        }) as COUNT;
        GlobData.SIS_state.TotalElementMass = (GlobData.SIS_state.TotalElementMass
            as ::core::ffi::c_int + m as ::core::ffi::c_int) as COUNT;
        GetSBayCapacity(&raw mut r.corner);
        DrawFilledRectangle(&raw mut r);
        num_frames = (num_frames as ::core::ffi::c_int - m as ::core::ffi::c_int)
            as COUNT;
    }
    if GlobData.SIS_state.FuelOnBoard
        > (100 as ::core::ffi::c_int * 100 as ::core::ffi::c_int
            / 10 as ::core::ffi::c_int) as DWORD
    {
        let mut FuelVolume: DWORD = 0;
        let mut r_0: RECT = RECT {
            corner: POINT { x: 0, y: 0 },
            extent: EXTENT { width: 0, height: 0 },
        };
        FuelVolume = GlobData
            .SIS_state
            .FuelOnBoard
            .wrapping_sub(
                (100 as ::core::ffi::c_int * 100 as ::core::ffi::c_int
                    / 10 as ::core::ffi::c_int) as DWORD,
            );
        GlobData.SIS_state.FuelOnBoard = (100 as ::core::ffi::c_int
            * 100 as ::core::ffi::c_int / 10 as ::core::ffi::c_int) as DWORD;
        r_0.extent.width = 3 as COORD;
        r_0.extent.height = 1 as COORD;
        while FuelVolume != 0 {
            let mut m_0: COUNT = 0;
            GetFTankCapacity(&raw mut r_0.corner);
            DrawPoint(&raw mut r_0.corner);
            r_0.corner.x = (r_0.corner.x as ::core::ffi::c_int
                + (r_0.extent.width as ::core::ffi::c_int + 1 as ::core::ffi::c_int))
                as COORD;
            DrawPoint(&raw mut r_0.corner);
            r_0.corner.x = (r_0.corner.x as ::core::ffi::c_int
                - r_0.extent.width as ::core::ffi::c_int) as COORD;
            SetContextForeGroundColor(
                SetContextBackGroundColor(
                    colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE),
                ),
            );
            DrawFilledRectangle(&raw mut r_0);
            m_0 = (if FuelVolume
                < (100 as ::core::ffi::c_int * 100 as ::core::ffi::c_int
                    / 10 as ::core::ffi::c_int) as DWORD
            {
                FuelVolume as COUNT as ::core::ffi::c_int
            } else {
                100 as ::core::ffi::c_int * 100 as ::core::ffi::c_int
                    / 10 as ::core::ffi::c_int
            }) as COUNT;
            GlobData.SIS_state.FuelOnBoard = GlobData
                .SIS_state
                .FuelOnBoard
                .wrapping_add(m_0 as DWORD);
            FuelVolume = FuelVolume.wrapping_sub(m_0 as DWORD);
        }
    }
    DestroyDrawable(ReleaseDrawable(ModuleFrame));
}
#[no_mangle]
pub unsafe extern "C" fn DoShipyard(mut pMS: *mut MENU_STATE) -> BOOLEAN {
    let mut select: BOOLEAN = BOOLEAN::FALSE;
    let mut cancel: BOOLEAN = BOOLEAN::FALSE;
    '_ExitShipyard: {
        if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
            & C2Rust_Unnamed_8::CHECK_ABORT.0 as ::core::ffi::c_int == 0
        {
            select = BOOLEAN(
                PulsedInputState
                    .menu[C2Rust_Unnamed_1::KEY_MENU_SELECT.0 as ::core::ffi::c_int
                    as usize] as ::core::ffi::c_uint,
            );
            cancel = BOOLEAN(
                PulsedInputState
                    .menu[C2Rust_Unnamed_1::KEY_MENU_CANCEL.0 as ::core::ffi::c_int
                    as usize] as ::core::ffi::c_uint,
            );
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
                    LoadGraphicInstance(b"graphics.shipyard\0".as_ptr() as RESOURCE)
                        as DRAWABLE,
                );
                (*pMS).CurString = CaptureStringTable(
                    LoadStringTableInstance(b"colortable.hangar\0".as_ptr() as RESOURCE),
                );
                (*pMS).hMusic = LoadMusicInstance(
                    b"music.shipyard\0".as_ptr() as RESOURCE,
                );
                SetTransitionSource(::core::ptr::null::<RECT>());
                BatchGraphics();
                DrawSISFrame();
                DrawSISMessage(
                    GetStringAddress(
                        SetAbsStringTableIndex(
                            GameStrings,
                            (C2Rust_Unnamed_3::STARBASE_STRING_BASE.0
                                as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as COUNT,
                        ),
                    ),
                );
                DrawSISTitle(
                    GetStringAddress(
                        SetAbsStringTableIndex(
                            GameStrings,
                            C2Rust_Unnamed_3::STARBASE_STRING_BASE.0
                                as ::core::ffi::c_int as COUNT,
                        ),
                    ),
                );
                SetContext(SpaceContext);
                DrawBluePrint(pMS);
                (*pMS).CurState = C2Rust_Unnamed_9::SHIPYARD_CREW.0 as ::core::ffi::c_int
                    as BYTE;
                DrawMenuStateStrings(
                    C2Rust_Unnamed_2::PM_CREW.0 as ::core::ffi::c_int as BYTE,
                    (*pMS).CurState as SWORD,
                );
                SetContext(SpaceContext);
                s.origin.x = 0 as COORD;
                s.origin.y = 0 as COORD;
                s.frame = SetAbsFrameIndex((*pMS).ModuleFrame, 0 as COUNT);
                GetContextClipRect(&raw mut old_r);
                r = old_r;
                r.corner.x -= 1;
                r.extent.width = (r.extent.width as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int) as COORD;
                r.extent.height = (r.extent.height as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int) as COORD;
                SetContextClipRect(&raw mut r);
                DrawStamp(&raw mut s);
                SetContextClipRect(&raw mut old_r);
                animatePowerLines(pMS);
                SetContextFont(TinyFont);
                ScreenTransition(3 as ::core::ffi::c_int, ::core::ptr::null::<RECT>());
                UnbatchGraphics();
                PlayMusic((*pMS).hMusic, BOOLEAN::TRUE, 1 as BYTE);
                ShowCombatShip(
                    pMS,
                    !(0 as ::core::ffi::c_int) as COUNT,
                    ::core::ptr::null_mut::<SHIP_FRAGMENT>(),
                );
                SetInputCallback(Some(on_input_frame as unsafe extern "C" fn() -> ()));
                SetFlashRect(
                    ::core::ptr::with_exposed_provenance_mut::<
                        RECT,
                    >(!(0 as ::core::ffi::c_long) as usize),
                );
                (*pMS).Initialized = BOOLEAN::TRUE.0 as ::core::ffi::c_int as SIZE;
            } else if cancel.0 != 0
                || select.0 != 0
                    && (*pMS).CurState as ::core::ffi::c_int
                        == C2Rust_Unnamed_9::SHIPYARD_EXIT.0 as ::core::ffi::c_int
            {
                break '_ExitShipyard;
            } else if select.0 != 0 {
                if (*pMS).CurState as ::core::ffi::c_int
                    != C2Rust_Unnamed_9::SHIPYARD_SAVELOAD.0 as ::core::ffi::c_int
                {
                    (*pMS).Initialized = BOOLEAN::FALSE.0 as ::core::ffi::c_int as SIZE;
                    DoModifyShips(pMS);
                } else if GameOptions().0 == 0 {
                    break '_ExitShipyard;
                } else {
                    DrawMenuStateStrings(
                        C2Rust_Unnamed_2::PM_CREW.0 as ::core::ffi::c_int as BYTE,
                        (*pMS).CurState as SWORD,
                    );
                    SetFlashRect(
                        ::core::ptr::with_exposed_provenance_mut::<
                            RECT,
                        >(!(0 as ::core::ffi::c_long) as usize),
                    );
                }
            } else {
                DoMenuChooser(
                    pMS,
                    C2Rust_Unnamed_2::PM_CREW.0 as ::core::ffi::c_int as BYTE,
                );
            }
            return BOOLEAN::TRUE;
        }
    }
    SetInputCallback(None);
    DestroyDrawable(ReleaseDrawable((*pMS).ModuleFrame));
    (*pMS).ModuleFrame = ::core::ptr::null_mut::<FRAME_DESC>();
    DestroyStringTable(ReleaseStringTable((*pMS).CurString));
    (*pMS).CurString = ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>();
    return BOOLEAN::FALSE;
}
