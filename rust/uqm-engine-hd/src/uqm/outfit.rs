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

// Rust translation of src/uqm/outfit.c.
extern "C" {
    pub type string_table_entry;
    pub type context_desc;
    pub type frame_desc;
    pub type font_desc;
    pub type drawable_desc;
    pub type tfb_soundsample;
    pub type FlashContext;
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
    fn SetAbsStringTableIndex(String: STRING, StringTableIndex: COUNT) -> STRING;
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
    fn SetContext(Context: CONTEXT) -> CONTEXT;
    fn SetContextForeGroundColor(Color: Color) -> Color;
    fn GetContextClipRect(pRect: *mut RECT) -> BOOLEAN;
    fn DrawStamp(pStamp: *mut STAMP);
    fn DrawFilledStamp(pStamp: *mut STAMP);
    fn DrawFilledRectangle(pRect: *mut RECT);
    fn font_DrawText(pText: *mut TEXT);
    fn font_DrawShadowedText(
        pText: *mut TEXT,
        direction: BYTE,
        text_color: Color,
        shadow_color: Color,
    );
    fn BatchGraphics();
    fn UnbatchGraphics();
    fn DestroyDrawable(Drawable: DRAWABLE) -> BOOLEAN;
    fn GetFrameRect(Frame: FRAME, pRect: *mut RECT) -> BOOLEAN;
    fn LoadGraphicInstance(res: RESOURCE) -> *mut ::core::ffi::c_void;
    fn SetContextFont(Font: FONT) -> FONT;
    fn DestroyFont(FontRef: FONT) -> BOOLEAN;
    fn GetContextFontLeading(pheight: *mut SIZE) -> BOOLEAN;
    fn SetAbsFrameIndex(Frame: FRAME, FrameIndex: COUNT) -> FRAME;
    fn IncFrameIndex(Frame: FRAME) -> FRAME;
    fn CaptureDrawable(Drawable: DRAWABLE) -> FRAME;
    fn ReleaseDrawable(Frame: FRAME) -> DRAWABLE;
    fn memset(
        _: *mut ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strtok(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(_: *const ::core::ffi::c_char) -> size_t;
    static mut PulsedInputState: CONTROLLER_INPUT_STATE;
    static mut optWhichMenu: ::core::ffi::c_int;
    static mut optWhichFonts: ::core::ffi::c_int;
    static mut resolutionFactor: ::core::ffi::c_uint;
    static mut optInfiniteRU: OPT_ENABLABLE;
    static mut optInfiniteFuel: OPT_ENABLABLE;
    static mut optCustomBorder: OPT_ENABLABLE;
    static mut optScrTrans: ::core::ffi::c_int;
    static mut optFlagshipColor: ::core::ffi::c_int;
    static mut optWindowType: DWORD;
    static mut optDosMenus: OPT_ENABLABLE;
    fn LoadMusicInstance(res: RESOURCE) -> MUSIC_REF;
    fn SetMusicPosition();
    fn DoMenuChooser(pMS: *mut MENU_STATE, BaseState: BYTE) -> BOOLEAN;
    fn DrawMenuStateStrings(beg_index: BYTE, NewState: SWORD);
    fn DrawBorder(Visible: BYTE);
    fn GameOptions() -> BOOLEAN;
    fn SetNamingCallback(_: Option<NamingCallback>);
    static mut OutfitOrShipyard: BYTE;
    static mut CanvasWidth: ::core::ffi::c_int;
    static mut CanvasHeight: ::core::ffi::c_int;
    fn DrawSISFrame();
    fn ClearSISRect(ClearFlags: BYTE);
    fn SetFlashRect(pRect: *const RECT, purple: BOOLEAN);
    fn SetAdditionalRect(pRect: *const RECT, number: COUNT);
    fn DumpAdditionalRect();
    fn PreUpdateFlashRect();
    fn PostUpdateFlashRect();
    fn DrawSISTitle(pStr: *mut UNICODE);
    fn DrawSISMessage(pStr: *const UNICODE);
    fn GetGaugeRect(pRect: *mut RECT, IsCrewRect: BOOLEAN);
    fn DrawFlagshipStats();
    fn DeltaSISGauges(
        crew_delta: SIZE,
        fuel_delta: SDWORD,
        resunit_delta: ::core::ffi::c_int,
    );
    fn GetFuelTankCapacity() -> DWORD;
    fn CountSISPieces(piece_type: BYTE) -> COUNT;
    fn DrawFlagshipName(InStatusArea: BOOLEAN, NewGame: bool);
    fn DrawFuelInFTanks(isOutfit: BOOLEAN);
    static mut GameStrings: STRING;
    fn PlayMusicResume(hMusic: MUSIC_REF, Volume: BYTE);
    fn DrawShipPiece(
        ModuleFrame: FRAME,
        which_piece: COUNT,
        which_slot: COUNT,
        DrawBluePrint: BOOLEAN,
    );
    fn SleepThread(timePeriod: TimePeriod);
    fn getGameStateUint(name: *const ::core::ffi::c_char) -> DWORD;
    static mut GlobData: GLOBDATA;
    fn isPC(optWhich: ::core::ffi::c_int) -> OPT_CONSOLETYPE;
    fn is3DO(optWhich: ::core::ffi::c_int) -> OPT_CONSOLETYPE;
    static mut SpaceContext: CONTEXT;
    static mut StatusContext: CONTEXT;
    static mut StarConFont: FONT;
    static mut TinyFont: FONT;
    static mut TinyFontBold: FONT;
    static mut SquareFont: FONT;
    fn SetMenuSounds(sound_0: MENU_SOUND_FLAGS, sound_1: MENU_SOUND_FLAGS);
    fn PlayMenuSound(S: MENU_SOUND_EFFECT);
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
}
pub type size_t = usize;
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
pub type UniChar = uint32;
pub type STRING_TABLE_ENTRY_DESC = string_table_entry;
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
pub struct C2Rust_Unnamed_1(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_1 {
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
pub type NamingCallback = unsafe extern "C" fn() -> ();
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GLOBDATA {
    pub SIS_state: SIS_STATE,
    pub Game_state: GAME_STATE,
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
pub type ACTIVITY = UWORD;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MODULES_STATE {
    pub list: [BYTE; 14],
    pub count: COUNT,
    pub topIndex: COUNT,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_6(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_6 {
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
pub struct C2Rust_Unnamed_7(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_7 {
    pub const BIOLOGICAL_DISASTER: Self = Self(0);
    pub const EARTHQUAKE_DISASTER: Self = Self(1);
    pub const LIGHTNING_DISASTER: Self = Self(2);
    pub const LAVASPOT_DISASTER: Self = Self(3);
    pub const LANDER_INJURED: Self = Self(4);
    pub const LANDER_SHOOTS: Self = Self(5);
    pub const LANDER_HITS: Self = Self(6);
    pub const LIFEFORM_CANNED: Self = Self(7);
    pub const LANDER_PICKUP: Self = Self(8);
    pub const LANDER_FULL: Self = Self(9);
    pub const LANDER_DEPARTS: Self = Self(10);
    pub const LANDER_RETURNS: Self = Self(11);
    pub const LANDER_DESTROYED: Self = Self(12);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_8(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_8 {
    pub const OUTFIT_FUEL: Self = Self(0);
    pub const OUTFIT_MODULES: Self = Self(1);
    pub const OUTFIT_SAVELOAD: Self = Self(2);
    pub const OUTFIT_EXIT: Self = Self(3);
    pub const OUTFIT_DOFUEL: Self = Self(4);
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
#[no_mangle]
pub static mut lander_pos: [POINT; 10] = [POINT { x: 0, y: 0 }; 10];
#[no_mangle]
pub static mut ModuleFont: FONT = ::core::ptr::null_mut::<FONT_DESC>();
unsafe extern "C" fn InitializeDOSLanderPos() {
    let mut i: ::core::ffi::c_int = 0;
    let mut temp: [POINT; 10] = [
        point {
            x: 206 as COORD,
            y: 59 as COORD,
        },
        point {
            x: (206 as ::core::ffi::c_int + 14 as ::core::ffi::c_int) as COORD,
            y: (59 as ::core::ffi::c_int - 6 as ::core::ffi::c_int) as COORD,
        },
        point {
            x: (206 as ::core::ffi::c_int + 14 as ::core::ffi::c_int) as COORD,
            y: (59 as ::core::ffi::c_int + 9 as ::core::ffi::c_int) as COORD,
        },
        point {
            x: 206 as COORD,
            y: (59 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as COORD,
        },
        point {
            x: 206 as COORD,
            y: (59 as ::core::ffi::c_int + 15 as ::core::ffi::c_int) as COORD,
        },
        point {
            x: (206 as ::core::ffi::c_int - 14 as ::core::ffi::c_int) as COORD,
            y: (59 as ::core::ffi::c_int - 9 as ::core::ffi::c_int) as COORD,
        },
        point {
            x: (206 as ::core::ffi::c_int - 14 as ::core::ffi::c_int) as COORD,
            y: (59 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as COORD,
        },
        point {
            x: (206 as ::core::ffi::c_int + 14 as ::core::ffi::c_int) as COORD,
            y: (59 as ::core::ffi::c_int - 21 as ::core::ffi::c_int) as COORD,
        },
        point {
            x: (206 as ::core::ffi::c_int + 14 as ::core::ffi::c_int) as COORD,
            y: (59 as ::core::ffi::c_int + 24 as ::core::ffi::c_int) as COORD,
        },
        point {
            x: (206 as ::core::ffi::c_int - 28 as ::core::ffi::c_int) as COORD,
            y: (59 as ::core::ffi::c_int - 3 as ::core::ffi::c_int) as COORD,
        },
    ];
    if if optWindowType == 0 as DWORD {
        BOOLEAN::TRUE.0 as ::core::ffi::c_int
    } else {
        BOOLEAN::FALSE.0 as ::core::ffi::c_int
    } == 0
    {
        return;
    }
    if lander_pos[0usize].x as ::core::ffi::c_int
        != (temp[0usize].x as ::core::ffi::c_int) << resolutionFactor
    {
        i = 0 as ::core::ffi::c_int;
        while i < 10 as ::core::ffi::c_int {
            lander_pos[i as usize].x = ((temp[i as usize].x as ::core::ffi::c_int)
                << resolutionFactor) as COORD;
            lander_pos[i as usize].y = ((temp[i as usize].y as ::core::ffi::c_int)
                << resolutionFactor) as COORD;
            i += 1;
        }
    }
}
#[no_mangle]
pub static mut ModuleState: MODULES_STATE = MODULES_STATE {
    list: [0; 14],
    count: 0,
    topIndex: 0,
};
unsafe extern "C" fn DrawModuleStatus(
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
    if GlobData.Game_state.ModuleCost[index as usize] != 0 {
        SetContextForeGroundColor(
            if selected as ::core::ffi::c_int != 0 {
                buildColorRgba(0x55 as BYTE, 0xff as BYTE, 0xff as BYTE, 0xff as BYTE)
            } else {
                buildColorRgba(0 as BYTE, 0xaa as BYTE, 0xaa as BYTE, 0xff as BYTE)
            },
        );
        t.baseline.y = (r.corner.y as ::core::ffi::c_int
            + ((6 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        t.pStr = GetStringAddress(
            SetAbsStringTableIndex(
                GameStrings,
                (index as ::core::ffi::c_int
                    + (C2Rust_Unnamed_5::STARBASE_STRING_BASE.0 as ::core::ffi::c_int
                        + 12 as ::core::ffi::c_int)) as COUNT,
            ),
        );
        t.CharCount = utf8StringPos(t.pStr, ' ' as UniChar) as COUNT;
        uqm_lua_checked!(font_DrawText(&raw mut t), ());
        t.baseline.y = (t.baseline.y as ::core::ffi::c_int
            + ((7 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        t.pStr = skipUTF8Chars(
            t.pStr,
            (t.CharCount as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as size_t,
        );
        t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
        uqm_lua_checked!(font_DrawText(&raw mut t), ());
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
        t.baseline.y = (t.baseline.y as ::core::ffi::c_int
            - ((3 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        snprintf(
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[UNICODE; 10]>(),
            b"%u\0".as_ptr() as *const ::core::ffi::c_char,
            GlobData.Game_state.ModuleCost[index as usize] as ::core::ffi::c_int
                * 50 as ::core::ffi::c_int,
        );
        t.pStr = &raw mut buf as *mut UNICODE;
        t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
        uqm_lua_checked!(font_DrawText(&raw mut t), ());
    } else {
        SetContextForeGroundColor(
            buildColorRgba(0x55 as BYTE, 0x55 as BYTE, 0xff as BYTE, 0xff as BYTE),
        );
        r.corner.x = (r.corner.x as ::core::ffi::c_int
            + ((21 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        r.corner.y = (r.corner.y as ::core::ffi::c_int
            + ((6 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        r.extent.width = (r.extent.width as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int) as COORD;
        r.extent.height = ((2 as ::core::ffi::c_int) << resolutionFactor) as COORD;
        uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
    };
}
unsafe extern "C" fn DrawModuleDisplay(mut modState: *mut MODULES_STATE) {
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
            (C2Rust_Unnamed_5::STARBASE_STRING_BASE.0 as ::core::ffi::c_int
                + 11 as ::core::ffi::c_int) as COUNT,
        ),
    );
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    SetContextForeGroundColor(
        buildColorRgba(0x55 as BYTE, 0xff as BYTE, 0xff as BYTE, 0xff as BYTE),
    );
    uqm_lua_checked!(font_DrawText(&raw mut t), ());
    i = 0 as COUNT;
    while (i as ::core::ffi::c_int) < 5 as ::core::ffi::c_int {
        let mut modIndex: COUNT = ((*modState).topIndex as ::core::ffi::c_int
            + i as ::core::ffi::c_int) as COUNT;
        if modIndex as ::core::ffi::c_int >= (*modState).count as ::core::ffi::c_int {
            break;
        }
        uqm_lua_checked!(DrawModuleStatus((*modState).list[modIndex as usize] as COUNT, i, false), ());
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn DrawModules(mut modState: *mut MODULES_STATE, mut NewItem: COUNT) {
    let mut OldContext: CONTEXT = SetContext(StatusContext);
    BatchGraphics();
    uqm_lua_checked!(DrawModuleDisplay(modState), ());
    uqm_lua_checked!(DrawModuleStatus(
        (*modState).list[NewItem as usize] as COUNT,
        (NewItem as ::core::ffi::c_int - (*modState).topIndex as ::core::ffi::c_int)
            as COUNT,
        true,
    ), ());
    UnbatchGraphics();
    SetContext(OldContext);
}
unsafe extern "C" fn ManipulateModules(mut NewState: SIZE) {
    let mut modState: *mut MODULES_STATE = ::core::ptr::null_mut::<MODULES_STATE>();
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
    modState = &raw mut ModuleState;
    NewTop = (*modState).topIndex as SIZE;
    if NewState as ::core::ffi::c_int
        > C2Rust_Unnamed_2::NUM_PURCHASE_MODULES.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(DrawModules(modState, NewState as COUNT), ());
        return;
    }
    if (NewState as ::core::ffi::c_int) < NewTop as ::core::ffi::c_int
        || NewState as ::core::ffi::c_int
            >= NewTop as ::core::ffi::c_int + 5 as ::core::ffi::c_int
    {
        (*modState).topIndex = (NewState as ::core::ffi::c_int
            - NewState as ::core::ffi::c_int % 5 as ::core::ffi::c_int) as COUNT;
    }
    uqm_lua_checked!(DrawModules(modState, NewState as COUNT), ());
}
#[no_mangle]
pub unsafe extern "C" fn InventoryModules(
    mut pModuleMap: *mut BYTE,
    mut Size: COUNT,
) -> SIZE {
    let mut i: BYTE = 0;
    let mut ModulesOnBoard: SIZE = 0;
    ModulesOnBoard = 0 as SIZE;
    i = 0 as BYTE;
    while (i as ::core::ffi::c_int)
        < C2Rust_Unnamed_2::NUM_PURCHASE_MODULES.0 as ::core::ffi::c_int
        && Size as ::core::ffi::c_int > 0 as ::core::ffi::c_int
    {
        let c2rust_fresh4 = pModuleMap;
        pModuleMap = pModuleMap.offset(1);
        *c2rust_fresh4 = i;
        ModulesOnBoard += 1;
        Size = Size.wrapping_sub(1);
        i = i.wrapping_add(1);
    }
    return ModulesOnBoard;
}
unsafe extern "C" fn DrawModuleMenuText(
    mut r: *mut RECT,
    mut Index: ::core::ffi::c_int,
) {
    let mut text: TEXT = TEXT {
        baseline: POINT { x: 0, y: 0 },
        pStr: ::core::ptr::null::<UNICODE>(),
        align: TEXT_ALIGN::ALIGN_LEFT,
        CharCount: 0,
    };
    let mut leading: SIZE = 0;
    let mut block: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut buf: [UNICODE; 256] = [0; 256];
    let mut og_baseline_x: COORD = 0;
    if if optWindowType == 0 as DWORD {
        BOOLEAN::TRUE.0 as ::core::ffi::c_int
    } else {
        BOOLEAN::FALSE.0 as ::core::ffi::c_int
    } != 0
        || strlen(
            GetStringAddress(
                SetAbsStringTableIndex(
                    GameStrings,
                    (C2Rust_Unnamed_5::STARBASE_STRING_BASE.0 as ::core::ffi::c_int
                        + 12 as ::core::ffi::c_int
                        + C2Rust_Unnamed_2::NUM_PURCHASE_MODULES.0 as ::core::ffi::c_int
                        + Index) as COUNT,
                ),
            ),
        ) == 0
    {
        return;
    }
    SetContextFont(ModuleFont);
    GetContextFontLeading(&raw mut leading);
    SetContextForeGroundColor(colorFromRgb15(0xa as BYTE, 0xa as BYTE, 0xa as BYTE));
    if optCustomBorder.0 == 0 {
        block = *r;
        block.extent.height = (((leading as ::core::ffi::c_int)
            << 1 as ::core::ffi::c_int)
            - ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        uqm_lua_checked!(DrawFilledRectangle(&raw mut block), ());
    } else {
        uqm_lua_checked!(DrawBorder(C2Rust_Unnamed_3::TEXT_LABEL_FRAME.0 as ::core::ffi::c_int as BYTE), ());
    }
    text.baseline.x = ((*r).corner.x as ::core::ffi::c_int
        + ((*r).extent.width as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)) as COORD;
    text.baseline.y = ((*r).corner.y as ::core::ffi::c_int
        + leading as ::core::ffi::c_int
        - ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    og_baseline_x = text.baseline.x;
    utf8StringCopy(
        &raw mut buf as *mut UNICODE,
        ::core::mem::size_of::<[UNICODE; 256]>(),
        GetStringAddress(
            SetAbsStringTableIndex(
                GameStrings,
                (C2Rust_Unnamed_5::STARBASE_STRING_BASE.0 as ::core::ffi::c_int
                    + 12 as ::core::ffi::c_int
                    + C2Rust_Unnamed_2::NUM_PURCHASE_MODULES.0 as ::core::ffi::c_int
                    + Index) as COUNT,
            ),
        ),
    );
    text.align = TEXT_ALIGN::ALIGN_CENTER;
    text.pStr = strtok(
        &raw mut buf as *mut ::core::ffi::c_char,
        b" \0".as_ptr() as *const ::core::ffi::c_char,
    );
    text.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    while !text.pStr.is_null() {
        text.pStr = AlignText(text.pStr, &raw mut text.baseline.x);
        text.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
        uqm_lua_checked!(font_DrawShadowedText(
            &raw mut text,
            C2Rust_Unnamed::WEST_SHADOW.0 as ::core::ffi::c_int as BYTE,
            buildColorRgba(0x74 as BYTE, 0x74 as BYTE, 0x74 as BYTE, 0xff as BYTE),
            buildColorRgba(0x2e as BYTE, 0x2e as BYTE, 0x2e as BYTE, 0xff as BYTE),
        ), ());
        text.pStr = strtok(
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
            b" \0".as_ptr() as *const ::core::ffi::c_char,
        );
        text.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
        text.baseline.y = (text.baseline.y as ::core::ffi::c_int
            + leading as ::core::ffi::c_int) as COORD;
        text.baseline.x = og_baseline_x;
    }
}
unsafe extern "C" fn DrawModuleStrings(mut pMS: *mut MENU_STATE, mut NewModule: BYTE) {
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
    BatchGraphics();
    if if optWindowType == 0 as DWORD {
        BOOLEAN::TRUE.0 as ::core::ffi::c_int
    } else {
        BOOLEAN::FALSE.0 as ::core::ffi::c_int
    } == 0
    {
        r.corner.x = (s.origin.x as ::core::ffi::c_int
            - ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        r.corner.y = (s.origin.y as ::core::ffi::c_int
            - ((11 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        r.extent.width = (((64 as ::core::ffi::c_int) << resolutionFactor)
            - ((8 as ::core::ffi::c_int) << resolutionFactor)
            + ((2 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        r.extent.height = ((11 as ::core::ffi::c_int) << resolutionFactor) as COORD;
        SetContextForeGroundColor(colorFromRgb15(0xa as BYTE, 0xa as BYTE, 0xa as BYTE));
        if optCustomBorder.0 == 0 {
            uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
        }
    }
    uqm_lua_checked!(DrawBorder(C2Rust_Unnamed_3::SIS_RADAR_FRAME.0 as ::core::ffi::c_int as BYTE), ());
    if if optWindowType == 0 as DWORD {
        BOOLEAN::TRUE.0 as ::core::ffi::c_int
    } else {
        BOOLEAN::FALSE.0 as ::core::ffi::c_int
    } != 0
    {
        let mut dosRect: RECT = RECT {
            corner: POINT { x: 0, y: 0 },
            extent: EXTENT { width: 0, height: 0 },
        };
        dosRect.corner.x = ((2 as ::core::ffi::c_int) << resolutionFactor) as COORD;
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
            - ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        dosRect.extent.width = (((64 as ::core::ffi::c_int) << resolutionFactor)
            - ((8 as ::core::ffi::c_int) << resolutionFactor)
            + ((4 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        dosRect.extent.height = (((53 as ::core::ffi::c_int) << resolutionFactor)
            + ((2 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        if optCustomBorder.0 != 0 {
            dosRect.corner.x = (dosRect.corner.x as ::core::ffi::c_int
                + ((2 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
            dosRect.corner.y = (dosRect.corner.y as ::core::ffi::c_int
                + ((2 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
            dosRect.extent.width = (dosRect.extent.width as ::core::ffi::c_int
                - ((4 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
            dosRect.extent.height = (dosRect.extent.height as ::core::ffi::c_int
                - ((4 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
            SetContextForeGroundColor(colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE));
            uqm_lua_checked!(DrawFilledRectangle(&raw mut dosRect), ());
        } else if if resolutionFactor != 2 as ::core::ffi::c_uint {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } == 0
        {
            uqm_lua_checked!(DrawStarConBox(
                &raw mut dosRect,
                1 as SIZE,
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
    if NewModule as ::core::ffi::c_int
        >= C2Rust_Unnamed_2::NUM_MODULES.0 as ::core::ffi::c_int
    {
        r.corner = s.origin;
        r.extent.width = (((64 as ::core::ffi::c_int) << resolutionFactor)
            - ((8 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        r.extent.height = ((53 as ::core::ffi::c_int) << resolutionFactor) as COORD;
        SetContextForeGroundColor(colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE));
        uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
    } else if !(*pMS).CurFrame.is_null() {
        let mut t: TEXT = TEXT {
            baseline: POINT { x: 0, y: 0 },
            pStr: ::core::ptr::null::<UNICODE>(),
            align: TEXT_ALIGN::ALIGN_LEFT,
            CharCount: 0,
        };
        let mut buf: [UNICODE; 40] = [0; 40];
        s.frame = SetAbsFrameIndex((*pMS).CurFrame, NewModule as COUNT);
        uqm_lua_checked!(DrawStamp(&raw mut s), ());
        uqm_lua_checked!(DrawModuleMenuText(&raw mut r, NewModule as ::core::ffi::c_int), ());
        t.baseline.x = (s.origin.x as ::core::ffi::c_int
            + (((64 as ::core::ffi::c_int) << resolutionFactor)
                - ((8 as ::core::ffi::c_int) << resolutionFactor))
            - ((2 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        t.baseline.y = (s.origin.y as ::core::ffi::c_int
            + ((53 as ::core::ffi::c_int) << resolutionFactor)
            - ((2 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        t.align = TEXT_ALIGN::ALIGN_RIGHT;
        t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
        t.pStr = &raw mut buf as *mut UNICODE;
        sprintf(
            &raw mut buf as *mut ::core::ffi::c_char,
            b"%u\0".as_ptr() as *const ::core::ffi::c_char,
            GlobData.Game_state.ModuleCost[NewModule as usize] as ::core::ffi::c_int
                * 50 as ::core::ffi::c_int,
        );
        if isPC(optWhichFonts).0 != 0 {
            SetContextFont(TinyFont);
        } else {
            SetContextFont(TinyFontBold);
        }
        if GlobData.SIS_state.ResUnits
            >= (GlobData.Game_state.ModuleCost[NewModule as usize] as ::core::ffi::c_int
                * 50 as ::core::ffi::c_int) as DWORD
        {
            SetContextForeGroundColor(
                colorFromRgb15(0 as BYTE, 0x1f as BYTE, 0 as BYTE),
            );
        } else {
            SetContextForeGroundColor(
                colorFromRgb15(0x1f as BYTE, 0 as BYTE, 0 as BYTE),
            );
        }
        if if optWindowType == 0 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } == 0
        {
            uqm_lua_checked!(font_DrawText(&raw mut t), ());
        }
    }
    UnbatchGraphics();
    SetContext(OldContext);
}
unsafe extern "C" fn RedistributeFuel() {
    let OldContext: CONTEXT = SetContext(SpaceContext);
    BatchGraphics();
    uqm_lua_checked!(DrawFuelInFTanks(BOOLEAN::TRUE), ());
    UnbatchGraphics();
    SetContext(OldContext);
}
unsafe extern "C" fn DrawEscapePodText(mut rect: RECT) {
    let mut text: TEXT = TEXT {
        baseline: POINT { x: 0, y: 0 },
        pStr: ::core::ptr::null::<UNICODE>(),
        align: TEXT_ALIGN::ALIGN_LEFT,
        CharCount: 0,
    };
    let mut OldFont: FONT = ::core::ptr::null_mut::<FONT_DESC>();
    let mut OldColor: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    let mut leading: SIZE = 0;
    let mut block: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut buf: [UNICODE; 256] = [0; 256];
    let mut og_baseline_x: COORD = 0;
    if strlen(
        GetStringAddress(
            SetAbsStringTableIndex(
                GameStrings,
                (C2Rust_Unnamed_5::STARBASE_STRING_BASE.0 as ::core::ffi::c_int
                    + 12 as ::core::ffi::c_int
                    + C2Rust_Unnamed_2::NUM_PURCHASE_MODULES.0 as ::core::ffi::c_int
                    + C2Rust_Unnamed_2::NUM_PURCHASE_MODULES.0 as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int) as COUNT,
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
    block = rect;
    block.corner.x = (block.corner.x as ::core::ffi::c_int
        + (if if optWindowType == 0 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            9 as ::core::ffi::c_int
        } else {
            171 as ::core::ffi::c_int
        } << resolutionFactor)) as COORD;
    block.corner.y = (block.corner.y as ::core::ffi::c_int
        + ((38 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    block.extent.width = ((36 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    block.extent.height = ((6 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    uqm_lua_checked!(DrawFilledRectangle(&raw mut block), ());
    block.corner.x = (block.corner.x as ::core::ffi::c_int
        + ((9 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    block.corner.y = (block.corner.y as ::core::ffi::c_int
        + block.extent.height as ::core::ffi::c_int) as COORD;
    block.extent.width = ((19 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    uqm_lua_checked!(DrawFilledRectangle(&raw mut block), ());
    GetContextFontLeading(&raw mut leading);
    text.baseline = rect.corner;
    text.baseline.x = (text.baseline.x as ::core::ffi::c_int
        + (if if optWindowType == 0 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            27 as ::core::ffi::c_int
        } else {
            189 as ::core::ffi::c_int
        } << resolutionFactor)) as COORD;
    text.baseline.y = (text.baseline.y as ::core::ffi::c_int
        + ((39 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    text.align = TEXT_ALIGN::ALIGN_CENTER;
    og_baseline_x = text.baseline.x;
    utf8StringCopy(
        &raw mut buf as *mut UNICODE,
        ::core::mem::size_of::<[UNICODE; 256]>(),
        GetStringAddress(
            SetAbsStringTableIndex(
                GameStrings,
                (C2Rust_Unnamed_5::STARBASE_STRING_BASE.0 as ::core::ffi::c_int
                    + 12 as ::core::ffi::c_int
                    + C2Rust_Unnamed_2::NUM_PURCHASE_MODULES.0 as ::core::ffi::c_int
                    + C2Rust_Unnamed_2::NUM_PURCHASE_MODULES.0 as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int) as COUNT,
            ),
        ),
    );
    text.align = TEXT_ALIGN::ALIGN_CENTER;
    text.pStr = strtok(
        &raw mut buf as *mut ::core::ffi::c_char,
        b" \0".as_ptr() as *const ::core::ffi::c_char,
    );
    text.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    SetContextForeGroundColor(
        buildColorRgba(0 as BYTE, 0xf8 as BYTE, 0 as BYTE, 0xff as BYTE),
    );
    while !text.pStr.is_null() {
        text.pStr = AlignText(text.pStr, &raw mut text.baseline.x);
        text.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
        uqm_lua_checked!(font_DrawText(&raw mut text), ());
        text.pStr = strtok(
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
            b" \0".as_ptr() as *const ::core::ffi::c_char,
        );
        text.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
        text.baseline.y = (text.baseline.y as ::core::ffi::c_int
            + leading as ::core::ffi::c_int) as COORD;
        text.baseline.x = og_baseline_x;
    }
    SetContextFont(OldFont);
    SetContextForeGroundColor(OldColor);
}
unsafe extern "C" fn DrawNoLandersText(mut rect: RECT) {
    let mut text: TEXT = TEXT {
        baseline: POINT { x: 0, y: 0 },
        pStr: ::core::ptr::null::<UNICODE>(),
        align: TEXT_ALIGN::ALIGN_LEFT,
        CharCount: 0,
    };
    let mut OldFont: FONT = ::core::ptr::null_mut::<FONT_DESC>();
    let mut OldColor: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    let mut block: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    if if optWindowType == 0 as DWORD {
        BOOLEAN::TRUE.0 as ::core::ffi::c_int
    } else {
        BOOLEAN::FALSE.0 as ::core::ffi::c_int
    } != 0
        || strlen(
            GetStringAddress(
                SetAbsStringTableIndex(
                    GameStrings,
                    (C2Rust_Unnamed_5::STARBASE_STRING_BASE.0 as ::core::ffi::c_int
                        + 12 as ::core::ffi::c_int
                        + C2Rust_Unnamed_2::NUM_PURCHASE_MODULES.0 as ::core::ffi::c_int
                        + C2Rust_Unnamed_2::NUM_PURCHASE_MODULES.0 as ::core::ffi::c_int)
                        as COUNT,
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
    block = rect;
    block.corner.y = (block.corner.y as ::core::ffi::c_int
        + ((9 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    block.extent.width = (((162 as ::core::ffi::c_int) << resolutionFactor)
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
    block.extent.height = ((20 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    uqm_lua_checked!(DrawFilledRectangle(&raw mut block), ());
    text.baseline = rect.corner;
    text.baseline.x = (text.baseline.x as ::core::ffi::c_int
        + ((81 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    text.baseline.y = (text.baseline.y as ::core::ffi::c_int
        + ((17 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    text.align = TEXT_ALIGN::ALIGN_CENTER;
    text.pStr = AlignText(
        GetStringAddress(
            SetAbsStringTableIndex(
                GameStrings,
                (C2Rust_Unnamed_5::STARBASE_STRING_BASE.0 as ::core::ffi::c_int
                    + 12 as ::core::ffi::c_int
                    + C2Rust_Unnamed_2::NUM_PURCHASE_MODULES.0 as ::core::ffi::c_int
                    + C2Rust_Unnamed_2::NUM_PURCHASE_MODULES.0 as ::core::ffi::c_int)
                    as COUNT,
            ),
        ) as *const UNICODE,
        &raw mut text.baseline.x,
    );
    text.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    SetContextForeGroundColor(
        buildColorRgba(0 as BYTE, 0xf8 as BYTE, 0 as BYTE, 0xff as BYTE),
    );
    uqm_lua_checked!(font_DrawText(&raw mut text), ());
    SetContextFont(OldFont);
    SetContextForeGroundColor(OldColor);
}
unsafe extern "C" fn DisplayLanders(mut pMS: *mut MENU_STATE) {
    let mut s: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    s.frame = (*pMS).ModuleFrame;
    if uqm_lua_checked!(getGameStateUint(b"CHMMR_BOMB_STATE\0".as_ptr() as *const ::core::ffi::c_char), ())
        == 3 as DWORD
    {
        let mut rect: RECT = RECT {
            corner: POINT { x: 0, y: 0 },
            extent: EXTENT { width: 0, height: 0 },
        };
        s.origin.x = -(if if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            16 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        } << resolutionFactor) as COORD;
        s.origin.y = 0 as COORD;
        s.frame = SetAbsFrameIndex(
            (*pMS).ModuleFrame,
            (23 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int
                    * C2Rust_Unnamed_2::NUM_PURCHASE_MODULES.0 as ::core::ffi::c_int
                + 4 as ::core::ffi::c_int) as COUNT,
        );
        uqm_lua_checked!(DrawStamp(&raw mut s), ());
        GetFrameRect(s.frame, &raw mut rect);
        rect.corner.x = (rect.corner.x as ::core::ffi::c_int
            + s.origin.x as ::core::ffi::c_int) as COORD;
        uqm_lua_checked!(DrawNoLandersText(rect), ());
        uqm_lua_checked!(DrawEscapePodText(rect), ());
    } else {
        let mut i: COUNT = 0;
        if if optWindowType == 0 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } == 0
        {
            s.origin.x = (((24 as ::core::ffi::c_int) << resolutionFactor)
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
            s.origin.y = ((67 as ::core::ffi::c_int) << resolutionFactor) as COORD;
            i = 0 as COUNT;
            while (i as ::core::ffi::c_int)
                < GlobData.SIS_state.NumLanders as ::core::ffi::c_int
            {
                uqm_lua_checked!(DrawStamp(&raw mut s), ());
                s.origin.x = (s.origin.x as ::core::ffi::c_int
                    + ((15 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
                i = i.wrapping_add(1);
            }
            SetContextForeGroundColor(colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE));
            while (i as ::core::ffi::c_int) < 10 as ::core::ffi::c_int {
                uqm_lua_checked!(DrawFilledStamp(&raw mut s), ());
                s.origin.x = (s.origin.x as ::core::ffi::c_int
                    + ((15 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
                i = i.wrapping_add(1);
            }
        } else {
            i = 0 as COUNT;
            while (i as ::core::ffi::c_int)
                < GlobData.SIS_state.NumLanders as ::core::ffi::c_int
            {
                s.origin = lander_pos[i as usize];
                uqm_lua_checked!(DrawStamp(&raw mut s), ());
                i = i.wrapping_add(1);
            }
            SetContextForeGroundColor(colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE));
            while (i as ::core::ffi::c_int) < 10 as ::core::ffi::c_int {
                s.origin = lander_pos[i as usize];
                uqm_lua_checked!(DrawFilledStamp(&raw mut s), ());
                i = i.wrapping_add(1);
            }
        }
    };
}
unsafe extern "C" fn DoInstallModule(mut pMS: *mut MENU_STATE) -> BOOLEAN {
    let mut NewItem: SIZE = 0;
    let mut w: SIZE = 0;
    let mut NewState: BYTE = 0;
    let mut new_slot_piece: BYTE = 0;
    let mut old_slot_piece: BYTE = 0;
    let mut FirstItem: SIZE = 0;
    let mut LastItem: SIZE = 0;
    let mut select: BOOLEAN = BOOLEAN::FALSE;
    let mut cancel: BOOLEAN = BOOLEAN::FALSE;
    let mut motion: BOOLEAN = BOOLEAN::FALSE;
    if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
        & (C2Rust_Unnamed_6::CHECK_ABORT.0 as ::core::ffi::c_int
            | C2Rust_Unnamed_6::CHECK_LOAD.0 as ::core::ffi::c_int) != 0
    {
        (*pMS).InputFunc = Some(
            DoOutfit as unsafe extern "C" fn(*mut MENU_STATE) -> BOOLEAN,
        ) as Option<unsafe extern "C" fn(*mut menu_state) -> BOOLEAN>;
        return BOOLEAN::TRUE;
    }
    select = BOOLEAN(
        PulsedInputState
            .menu[C2Rust_Unnamed_0::KEY_MENU_SELECT.0 as ::core::ffi::c_int as usize]
            as ::core::ffi::c_uint,
    );
    cancel = BOOLEAN(
        PulsedInputState
            .menu[C2Rust_Unnamed_0::KEY_MENU_CANCEL.0 as ::core::ffi::c_int as usize]
            as ::core::ffi::c_uint,
    );
    motion = BOOLEAN(
        (PulsedInputState
            .menu[C2Rust_Unnamed_0::KEY_MENU_LEFT.0 as ::core::ffi::c_int as usize] != 0
            || PulsedInputState
                .menu[C2Rust_Unnamed_0::KEY_MENU_RIGHT.0 as ::core::ffi::c_int as usize]
                != 0
            || PulsedInputState
                .menu[C2Rust_Unnamed_0::KEY_MENU_UP.0 as ::core::ffi::c_int as usize]
                != 0
            || PulsedInputState
                .menu[C2Rust_Unnamed_0::KEY_MENU_DOWN.0 as ::core::ffi::c_int as usize]
                != 0) as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
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
    FirstItem = 0 as SIZE;
    NewState = (*pMS).CurState;
    match NewState as ::core::ffi::c_int {
        0 | 23 => {
            old_slot_piece = (if ((*pMS).delta_item as ::core::ffi::c_int)
                < GlobData.SIS_state.NumLanders as ::core::ffi::c_int
            {
                C2Rust_Unnamed_2::PLANET_LANDER.0 as ::core::ffi::c_int
            } else {
                C2Rust_Unnamed_2::NUM_MODULES.0 as ::core::ffi::c_int
                    + 3 as ::core::ffi::c_int
            }) as BYTE;
            LastItem = (10 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as SIZE;
        }
        1 | 20 => {
            old_slot_piece = GlobData.SIS_state.DriveSlots[(*pMS).delta_item as usize];
            LastItem = (11 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as SIZE;
        }
        2 | 21 => {
            old_slot_piece = GlobData.SIS_state.JetSlots[(*pMS).delta_item as usize];
            LastItem = (8 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as SIZE;
        }
        _ => {
            old_slot_piece = GlobData.SIS_state.ModuleSlots[(*pMS).delta_item as usize];
            if uqm_lua_checked!(getGameStateUint(
                b"CHMMR_BOMB_STATE\0".as_ptr() as *const ::core::ffi::c_char,
            ), BOOLEAN(0)) == 3 as DWORD
            {
                FirstItem = 10 as SIZE;
            }
            LastItem = (16 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as SIZE;
        }
    }
    if (NewState as ::core::ffi::c_int)
        < C2Rust_Unnamed_2::CREW_POD.0 as ::core::ffi::c_int
    {
        LastItem = NewState as SIZE;
        FirstItem = LastItem;
    } else if (NewState as ::core::ffi::c_int)
        < C2Rust_Unnamed_2::NUM_MODULES.0 as ::core::ffi::c_int
    {
        FirstItem = C2Rust_Unnamed_2::CREW_POD.0 as ::core::ffi::c_int as SIZE;
        LastItem = (C2Rust_Unnamed_2::NUM_PURCHASE_MODULES.0 as ::core::ffi::c_int
            - 1 as ::core::ffi::c_int) as SIZE;
    }
    's_1003: {
        's_907: {
            if (*pMS).Initialized == 0 {
                new_slot_piece = old_slot_piece;
                (*pMS).Initialized = BOOLEAN::TRUE.0 as ::core::ffi::c_int as SIZE;
                (*pMS).InputFunc = Some(
                    DoInstallModule as unsafe extern "C" fn(*mut MENU_STATE) -> BOOLEAN,
                ) as Option<unsafe extern "C" fn(*mut menu_state) -> BOOLEAN>;
                SetContext(SpaceContext);
                uqm_lua_checked!(ClearSISRect(
                    ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as BYTE,
                ), BOOLEAN(0));
                uqm_lua_checked!(SetFlashRect(::core::ptr::null::<RECT>(), BOOLEAN::FALSE), BOOLEAN(0));
            } else if select.0 != 0 || cancel.0 != 0 {
                new_slot_piece = (*pMS).CurState;
                if select.0 != 0 {
                    if (new_slot_piece as ::core::ffi::c_int)
                        < C2Rust_Unnamed_2::NUM_MODULES.0 as ::core::ffi::c_int
                    {
                        if GlobData.SIS_state.ResUnits
                            < (GlobData.Game_state.ModuleCost[new_slot_piece as usize]
                                as ::core::ffi::c_int * 50 as ::core::ffi::c_int) as DWORD
                        {
                            uqm_lua_checked!(PlayMenuSound(MENU_SOUND_EFFECT::MENU_SOUND_FAILURE), BOOLEAN(0));
                            return BOOLEAN::TRUE;
                        }
                    } else if new_slot_piece as ::core::ffi::c_int
                        == C2Rust_Unnamed_2::NUM_MODULES.0 as ::core::ffi::c_int
                            + 2 as ::core::ffi::c_int
                    {
                        if old_slot_piece as ::core::ffi::c_int
                            == C2Rust_Unnamed_2::CREW_POD.0 as ::core::ffi::c_int
                        {
                            if GlobData.SIS_state.CrewEnlisted as ::core::ffi::c_int
                                > 50 as ::core::ffi::c_int
                                    * (CountSISPieces(
                                        C2Rust_Unnamed_2::CREW_POD.0 as ::core::ffi::c_int as BYTE,
                                    ) as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                            {
                                uqm_lua_checked!(PlayMenuSound(MENU_SOUND_EFFECT::MENU_SOUND_FAILURE), BOOLEAN(0));
                                return BOOLEAN::TRUE;
                            }
                        } else if old_slot_piece as ::core::ffi::c_int
                            == C2Rust_Unnamed_2::FUEL_TANK.0 as ::core::ffi::c_int
                            || old_slot_piece as ::core::ffi::c_int
                                == C2Rust_Unnamed_2::HIGHEFF_FUELSYS.0 as ::core::ffi::c_int
                        {
                            let mut volume: DWORD = 0;
                            volume = (CountSISPieces(
                                C2Rust_Unnamed_2::FUEL_TANK.0 as ::core::ffi::c_int as BYTE,
                            ) as DWORD)
                                .wrapping_mul(
                                    (50 as ::core::ffi::c_int * 100 as ::core::ffi::c_int)
                                        as DWORD,
                                )
                                .wrapping_add(
                                    (CountSISPieces(
                                        C2Rust_Unnamed_2::HIGHEFF_FUELSYS.0 as ::core::ffi::c_int
                                            as BYTE,
                                    ) as DWORD)
                                        .wrapping_mul(
                                            (100 as ::core::ffi::c_int * 100 as ::core::ffi::c_int)
                                                as DWORD,
                                        ),
                                );
                            volume = volume
                                .wrapping_sub(
                                    (if old_slot_piece as ::core::ffi::c_int
                                        == C2Rust_Unnamed_2::FUEL_TANK.0 as ::core::ffi::c_int
                                    {
                                        50 as ::core::ffi::c_int * 100 as ::core::ffi::c_int
                                    } else {
                                        100 as ::core::ffi::c_int * 100 as ::core::ffi::c_int
                                    }) as DWORD,
                                );
                            if GlobData.SIS_state.FuelOnBoard
                                > volume
                                    .wrapping_add(
                                        (10 as ::core::ffi::c_int * 100 as ::core::ffi::c_int)
                                            as DWORD,
                                    )
                            {
                                if optInfiniteFuel.0 == 0 {
                                    uqm_lua_checked!(PlayMenuSound(MENU_SOUND_EFFECT::MENU_SOUND_FAILURE), BOOLEAN(0));
                                    return BOOLEAN::TRUE;
                                } else if old_slot_piece as ::core::ffi::c_int
                                    == C2Rust_Unnamed_2::FUEL_TANK.0 as ::core::ffi::c_int
                                {
                                    uqm_lua_checked!(DeltaSISGauges(
                                        0 as SIZE,
                                        -(50 as SDWORD * 100 as SDWORD),
                                        0 as ::core::ffi::c_int,
                                    ), BOOLEAN(0));
                                } else {
                                    uqm_lua_checked!(DeltaSISGauges(
                                        0 as SIZE,
                                        -(100 as SDWORD * 100 as SDWORD),
                                        0 as ::core::ffi::c_int,
                                    ), BOOLEAN(0));
                                }
                            }
                        } else if old_slot_piece as ::core::ffi::c_int
                            == C2Rust_Unnamed_2::STORAGE_BAY.0 as ::core::ffi::c_int
                        {
                            if GlobData.SIS_state.TotalElementMass as ::core::ffi::c_int
                                > 500 as ::core::ffi::c_int
                                    * (CountSISPieces(
                                        C2Rust_Unnamed_2::STORAGE_BAY.0 as ::core::ffi::c_int
                                            as BYTE,
                                    ) as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                            {
                                uqm_lua_checked!(PlayMenuSound(MENU_SOUND_EFFECT::MENU_SOUND_FAILURE), BOOLEAN(0));
                                return BOOLEAN::TRUE;
                            }
                        }
                    }
                }
                SetContext(SpaceContext);
                uqm_lua_checked!(SetFlashRect(::core::ptr::null::<RECT>(), BOOLEAN::FALSE), BOOLEAN(0));
                if select.0 != 0 {
                    if new_slot_piece as ::core::ffi::c_int
                        >= C2Rust_Unnamed_2::NUM_MODULES.0 as ::core::ffi::c_int
                        && old_slot_piece as ::core::ffi::c_int
                            >= C2Rust_Unnamed_2::NUM_MODULES.0 as ::core::ffi::c_int
                    {
                        new_slot_piece = (new_slot_piece as ::core::ffi::c_int
                            - (C2Rust_Unnamed_2::NUM_MODULES.0 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int)) as BYTE;
                        if new_slot_piece as ::core::ffi::c_int
                            > C2Rust_Unnamed_2::CREW_POD.0 as ::core::ffi::c_int
                        {
                            new_slot_piece = C2Rust_Unnamed_2::PLANET_LANDER.0
                                as ::core::ffi::c_int as BYTE;
                        }
                    } else {
                        match (*pMS).CurState as ::core::ffi::c_int {
                            0 => {
                                GlobData.SIS_state.NumLanders = GlobData
                                    .SIS_state
                                    .NumLanders
                                    .wrapping_add(1);
                            }
                            23 => {
                                GlobData.SIS_state.NumLanders = GlobData
                                    .SIS_state
                                    .NumLanders
                                    .wrapping_sub(1);
                            }
                            1 | 20 => {
                                GlobData.SIS_state.DriveSlots[(*pMS).delta_item as usize] = new_slot_piece;
                            }
                            2 | 21 => {
                                GlobData.SIS_state.JetSlots[(*pMS).delta_item as usize] = new_slot_piece;
                            }
                            _ => {
                                GlobData
                                    .SIS_state
                                    .ModuleSlots[(*pMS).delta_item as usize] = new_slot_piece;
                            }
                        }
                        if (new_slot_piece as ::core::ffi::c_int)
                            < C2Rust_Unnamed_2::NUM_MODULES.0 as ::core::ffi::c_int
                        {
                            uqm_lua_checked!(DeltaSISGauges(
                                0x7fff as SIZE,
                                0x7fff as SDWORD,
                                -(GlobData.Game_state.ModuleCost[new_slot_piece as usize]
                                    as ::core::ffi::c_int * 50 as ::core::ffi::c_int),
                            ), BOOLEAN(0));
                        } else {
                            uqm_lua_checked!(DeltaSISGauges(
                                0x7fff as SIZE,
                                0x7fff as SDWORD,
                                GlobData.Game_state.ModuleCost[old_slot_piece as usize]
                                    as ::core::ffi::c_int * 50 as ::core::ffi::c_int,
                            ), BOOLEAN(0));
                        }
                        if (*pMS).CurState as ::core::ffi::c_int
                            == C2Rust_Unnamed_2::PLANET_LANDER.0 as ::core::ffi::c_int
                            || (*pMS).CurState as ::core::ffi::c_int
                                == C2Rust_Unnamed_2::NUM_MODULES.0 as ::core::ffi::c_int
                                    + 3 as ::core::ffi::c_int
                        {
                            uqm_lua_checked!(DisplayLanders(pMS), BOOLEAN(0));
                        } else {
                            uqm_lua_checked!(DrawShipPiece(
                                (*pMS).ModuleFrame,
                                new_slot_piece as COUNT,
                                (*pMS).delta_item as COUNT,
                                BOOLEAN::FALSE,
                            ), BOOLEAN(0));
                            if new_slot_piece as ::core::ffi::c_int
                                > C2Rust_Unnamed_2::TURNING_JETS.0 as ::core::ffi::c_int
                                && old_slot_piece as ::core::ffi::c_int
                                    > C2Rust_Unnamed_2::TURNING_JETS.0 as ::core::ffi::c_int
                            {
                                uqm_lua_checked!(RedistributeFuel(), BOOLEAN(0));
                            }
                            uqm_lua_checked!(DrawFlagshipStats(), BOOLEAN(0));
                        }
                    }
                    cancel = BOOLEAN::FALSE;
                }
                if ((*pMS).CurState as ::core::ffi::c_int)
                    < C2Rust_Unnamed_2::NUM_MODULES.0 as ::core::ffi::c_int
                {
                    (*pMS).CurState = ((*pMS).CurState as ::core::ffi::c_int
                        + (C2Rust_Unnamed_2::NUM_MODULES.0 as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int)) as BYTE;
                    if (*pMS).CurState as ::core::ffi::c_int
                        == C2Rust_Unnamed_2::NUM_MODULES.0 as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int
                        && (*pMS).delta_item as ::core::ffi::c_int
                            + 1 as ::core::ffi::c_int
                            > GlobData.SIS_state.NumLanders as ::core::ffi::c_int
                    {
                        new_slot_piece = old_slot_piece;
                    }
                    if ((*pMS).CurState as ::core::ffi::c_int)
                        < C2Rust_Unnamed_2::NUM_MODULES.0 as ::core::ffi::c_int
                    {
                        (*pMS).CurState = (C2Rust_Unnamed_2::NUM_MODULES.0
                            as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as BYTE;
                    } else if (*pMS).CurState as ::core::ffi::c_int
                        > C2Rust_Unnamed_2::NUM_MODULES.0 as ::core::ffi::c_int
                            + 2 as ::core::ffi::c_int
                    {
                        (*pMS).CurState = (C2Rust_Unnamed_2::NUM_MODULES.0
                            as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as BYTE;
                    }
                    if cancel.0 != 0 {
                        new_slot_piece = (*pMS).CurState;
                    }
                } else if cancel.0 == 0 {
                    if new_slot_piece as ::core::ffi::c_int
                        == C2Rust_Unnamed_2::NUM_MODULES.0 as ::core::ffi::c_int
                            + 3 as ::core::ffi::c_int
                        && old_slot_piece as ::core::ffi::c_int
                            == C2Rust_Unnamed_2::PLANET_LANDER.0 as ::core::ffi::c_int
                        && ((*pMS).delta_item as ::core::ffi::c_int)
                            < GlobData.SIS_state.NumLanders as ::core::ffi::c_int
                    {
                        new_slot_piece = old_slot_piece;
                    } else {
                        (*pMS).CurState = new_slot_piece;
                    }
                } else {
                    SetContext(StatusContext);
                    (*pMS).CurState = C2Rust_Unnamed_8::OUTFIT_MODULES.0
                        as ::core::ffi::c_int as BYTE;
                    uqm_lua_checked!(DrawMenuStateStrings(
                        C2Rust_Unnamed_1::PM_FUEL.0 as ::core::ffi::c_int as BYTE,
                        (*pMS).CurState as SWORD,
                    ), BOOLEAN(0));
                    uqm_lua_checked!(SetFlashRect(
                        ::core::ptr::with_exposed_provenance_mut::<
                            RECT,
                        >(!(0 as ::core::ffi::c_long) as usize),
                        BOOLEAN::FALSE,
                    ), BOOLEAN(0));
                    (*pMS).InputFunc = Some(
                        DoOutfit as unsafe extern "C" fn(*mut MENU_STATE) -> BOOLEAN,
                    ) as Option<unsafe extern "C" fn(*mut menu_state) -> BOOLEAN>;
                    uqm_lua_checked!(ClearSISRect(
                        ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as BYTE,
                    ), BOOLEAN(0));
                    break 's_1003;
                }
            } else if motion.0 != 0 {
                NewItem = 0;
                NewItem = (if (NewState as ::core::ffi::c_int)
                    < C2Rust_Unnamed_2::NUM_MODULES.0 as ::core::ffi::c_int
                {
                    (*pMS).CurState as ::core::ffi::c_int
                } else {
                    (*pMS).delta_item as ::core::ffi::c_int
                }) as SIZE;
                loop {
                    if NewState as ::core::ffi::c_int
                        >= C2Rust_Unnamed_2::NUM_MODULES.0 as ::core::ffi::c_int
                        && (PulsedInputState
                            .menu[C2Rust_Unnamed_0::KEY_MENU_UP.0 as ::core::ffi::c_int
                            as usize] != 0
                            || PulsedInputState
                                .menu[C2Rust_Unnamed_0::KEY_MENU_DOWN.0
                                as ::core::ffi::c_int as usize] != 0)
                    {
                        if PulsedInputState
                            .menu[C2Rust_Unnamed_0::KEY_MENU_UP.0 as ::core::ffi::c_int
                            as usize] != 0
                        {
                            let c2rust_fresh0 = NewState;
                            NewState = NewState.wrapping_sub(1);
                            if c2rust_fresh0 as ::core::ffi::c_int
                                == C2Rust_Unnamed_2::NUM_MODULES.0 as ::core::ffi::c_int
                            {
                                NewState = (C2Rust_Unnamed_2::NUM_MODULES.0
                                    as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as BYTE;
                            }
                        } else {
                            let c2rust_fresh1 = NewState;
                            NewState = NewState.wrapping_add(1);
                            if c2rust_fresh1 as ::core::ffi::c_int
                                == C2Rust_Unnamed_2::NUM_MODULES.0 as ::core::ffi::c_int
                                    + 3 as ::core::ffi::c_int
                            {
                                NewState = C2Rust_Unnamed_2::NUM_MODULES.0
                                    as ::core::ffi::c_int as BYTE;
                            }
                        }
                        NewItem = 0 as SIZE;
                        if uqm_lua_checked!(getGameStateUint(
                            b"CHMMR_BOMB_STATE\0".as_ptr() as *const ::core::ffi::c_char,
                        ), BOOLEAN(0)) == 3 as DWORD
                        {
                            if NewState as ::core::ffi::c_int
                                == C2Rust_Unnamed_2::NUM_MODULES.0 as ::core::ffi::c_int
                                    + 3 as ::core::ffi::c_int
                            {
                                NewState = (if PulsedInputState
                                    .menu[C2Rust_Unnamed_0::KEY_MENU_UP.0 as ::core::ffi::c_int
                                    as usize] != 0
                                {
                                    C2Rust_Unnamed_2::NUM_MODULES.0 as ::core::ffi::c_int
                                        + 2 as ::core::ffi::c_int
                                } else {
                                    C2Rust_Unnamed_2::NUM_MODULES.0 as ::core::ffi::c_int
                                }) as BYTE;
                            }
                            if NewState as ::core::ffi::c_int
                                == C2Rust_Unnamed_2::NUM_MODULES.0 as ::core::ffi::c_int
                                    + 2 as ::core::ffi::c_int
                            {
                                NewItem = 10 as SIZE;
                            }
                        }
                        (*pMS).delta_item = NewItem;
                    } else if PulsedInputState
                        .menu[C2Rust_Unnamed_0::KEY_MENU_LEFT.0 as ::core::ffi::c_int
                        as usize] != 0
                        || PulsedInputState
                            .menu[C2Rust_Unnamed_0::KEY_MENU_UP.0 as ::core::ffi::c_int
                            as usize] != 0
                    {
                        let c2rust_fresh2 = NewItem;
                        NewItem -= 1;
                        if c2rust_fresh2 as ::core::ffi::c_int
                            == FirstItem as ::core::ffi::c_int
                        {
                            NewItem = LastItem;
                        }
                    } else if PulsedInputState
                        .menu[C2Rust_Unnamed_0::KEY_MENU_RIGHT.0 as ::core::ffi::c_int
                        as usize] != 0
                        || PulsedInputState
                            .menu[C2Rust_Unnamed_0::KEY_MENU_DOWN.0 as ::core::ffi::c_int
                            as usize] != 0
                    {
                        let c2rust_fresh3 = NewItem;
                        NewItem += 1;
                        if c2rust_fresh3 as ::core::ffi::c_int
                            == LastItem as ::core::ffi::c_int
                        {
                            NewItem = FirstItem;
                        }
                    }
                    if !((NewState as ::core::ffi::c_int)
                        < C2Rust_Unnamed_2::NUM_MODULES.0 as ::core::ffi::c_int
                        && (GlobData.Game_state.ModuleCost[NewItem as usize]
                            as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                            || NewItem as ::core::ffi::c_int
                                >= C2Rust_Unnamed_2::GUN_WEAPON.0 as ::core::ffi::c_int
                                && NewItem as ::core::ffi::c_int
                                    <= C2Rust_Unnamed_2::CANNON_WEAPON.0 as ::core::ffi::c_int
                                && (*pMS).delta_item as ::core::ffi::c_int
                                    > 0 as ::core::ffi::c_int
                                && ((*pMS).delta_item as ::core::ffi::c_int)
                                    < 13 as ::core::ffi::c_int))
                    {
                        break;
                    }
                }
                if (NewState as ::core::ffi::c_int)
                    < C2Rust_Unnamed_2::NUM_MODULES.0 as ::core::ffi::c_int
                {
                    if NewItem as ::core::ffi::c_int
                        != (*pMS).CurState as ::core::ffi::c_int
                    {
                        (*pMS).CurState = NewItem as BYTE;
                        uqm_lua_checked!(PreUpdateFlashRect(), BOOLEAN(0));
                        uqm_lua_checked!(DrawModuleStrings(pMS, NewItem as BYTE), BOOLEAN(0));
                        uqm_lua_checked!(ManipulateModules(NewItem), BOOLEAN(0));
                        uqm_lua_checked!(PostUpdateFlashRect(), BOOLEAN(0));
                    }
                    break 's_1003;
                } else if NewItem as ::core::ffi::c_int
                    != (*pMS).delta_item as ::core::ffi::c_int
                    || NewState as ::core::ffi::c_int
                        != (*pMS).CurState as ::core::ffi::c_int
                {
                    w = 0;
                    match NewState as ::core::ffi::c_int {
                        0 | 23 => {
                            new_slot_piece = (if (NewItem as ::core::ffi::c_int)
                                < GlobData.SIS_state.NumLanders as ::core::ffi::c_int
                            {
                                C2Rust_Unnamed_2::PLANET_LANDER.0 as ::core::ffi::c_int
                            } else {
                                C2Rust_Unnamed_2::NUM_MODULES.0 as ::core::ffi::c_int
                                    + 3 as ::core::ffi::c_int
                            }) as BYTE;
                        }
                        1 | 20 => {
                            new_slot_piece = GlobData
                                .SIS_state
                                .DriveSlots[NewItem as usize];
                        }
                        2 | 21 => {
                            new_slot_piece = GlobData
                                .SIS_state
                                .JetSlots[NewItem as usize];
                        }
                        _ => {
                            new_slot_piece = GlobData
                                .SIS_state
                                .ModuleSlots[NewItem as usize];
                        }
                    }
                    SetContext(SpaceContext);
                    if NewState as ::core::ffi::c_int
                        == (*pMS).CurState as ::core::ffi::c_int
                    {
                        if NewState as ::core::ffi::c_int
                            == C2Rust_Unnamed_2::PLANET_LANDER.0 as ::core::ffi::c_int
                            || NewState as ::core::ffi::c_int
                                == C2Rust_Unnamed_2::NUM_MODULES.0 as ::core::ffi::c_int
                                    + 3 as ::core::ffi::c_int
                        {
                            w = ((15 as ::core::ffi::c_int) << resolutionFactor) as SIZE;
                        } else {
                            w = ((12 as ::core::ffi::c_int) << resolutionFactor) as SIZE;
                        }
                        w = (w as ::core::ffi::c_int
                            * (NewItem as ::core::ffi::c_int
                                - (*pMS).delta_item as ::core::ffi::c_int)) as SIZE;
                        if if optWindowType == 0 as DWORD {
                            BOOLEAN::TRUE.0 as ::core::ffi::c_int
                        } else {
                            BOOLEAN::FALSE.0 as ::core::ffi::c_int
                        } != 0
                            && (NewState as ::core::ffi::c_int
                                == C2Rust_Unnamed_2::PLANET_LANDER.0 as ::core::ffi::c_int
                                || NewState as ::core::ffi::c_int
                                    == C2Rust_Unnamed_2::NUM_MODULES.0 as ::core::ffi::c_int
                                        + 3 as ::core::ffi::c_int)
                        {
                            (*pMS).flash_rect0.corner.x = (lander_pos[NewItem as usize].x
                                as ::core::ffi::c_int
                                - ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
                            (*pMS).flash_rect0.corner.y = (lander_pos[NewItem as usize].y
                                as ::core::ffi::c_int
                                - ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
                        } else {
                            (*pMS).flash_rect0.corner.x = ((*pMS).flash_rect0.corner.x
                                as ::core::ffi::c_int + w as ::core::ffi::c_int) as COORD;
                        }
                        (*pMS).flash_rect1.corner.x = ((*pMS).flash_rect1.corner.x
                            as ::core::ffi::c_int + w as ::core::ffi::c_int) as COORD;
                        (*pMS).delta_item = NewItem;
                        break 's_907;
                    } else {
                        (*pMS).CurState = NewState;
                    }
                } else {
                    break 's_1003;
                }
            } else {
                break 's_1003;
            }
            w = ((12 as ::core::ffi::c_int) << resolutionFactor) as SIZE;
            match (*pMS).CurState as ::core::ffi::c_int {
                0 | 23 => {
                    (*pMS).flash_rect0.corner.x = (if if optWindowType == 0 as DWORD {
                        BOOLEAN::TRUE.0 as ::core::ffi::c_int
                    } else {
                        BOOLEAN::FALSE.0 as ::core::ffi::c_int
                    } != 0
                    {
                        (206 as ::core::ffi::c_int) << resolutionFactor
                    } else {
                        ((24 as ::core::ffi::c_int) << resolutionFactor)
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
                            }
                    } - ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
                    (*pMS).flash_rect0.corner.y = (if if optWindowType == 0 as DWORD {
                        BOOLEAN::TRUE.0 as ::core::ffi::c_int
                    } else {
                        BOOLEAN::FALSE.0 as ::core::ffi::c_int
                    } != 0
                    {
                        (59 as ::core::ffi::c_int) << resolutionFactor
                    } else {
                        (67 as ::core::ffi::c_int) << resolutionFactor
                    } - ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
                    (*pMS).flash_rect0.extent.width = ((11 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int) << resolutionFactor) as COORD;
                    (*pMS).flash_rect0.extent.height = ((13 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int) << resolutionFactor) as COORD;
                    w = ((15 as ::core::ffi::c_int) << resolutionFactor) as SIZE;
                }
                1 | 20 => {
                    (*pMS).flash_rect0.corner.x = (((33 as ::core::ffi::c_int)
                        << resolutionFactor)
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
                        } - ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
                    (*pMS).flash_rect0.corner.y = (((86 as ::core::ffi::c_int)
                        << resolutionFactor)
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
                        - ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
                    (*pMS).flash_rect0.extent.width = ((8 as ::core::ffi::c_int)
                        << resolutionFactor) as COORD;
                    (*pMS).flash_rect0.extent.height = ((6 as ::core::ffi::c_int)
                        << resolutionFactor) as COORD;
                    (*pMS).flash_rect1.corner.x = (((31 as ::core::ffi::c_int)
                        << resolutionFactor)
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
                        } - ((1 as ::core::ffi::c_int) << resolutionFactor)
                        + if if resolutionFactor != 2 as ::core::ffi::c_uint {
                            BOOLEAN::FALSE.0 as ::core::ffi::c_int
                        } else {
                            BOOLEAN::TRUE.0 as ::core::ffi::c_int
                        } == 0
                        {
                            0 as ::core::ffi::c_int
                        } else {
                            5 as ::core::ffi::c_int
                        }) as COORD;
                    (*pMS).flash_rect1.corner.y = (((56 as ::core::ffi::c_int)
                        << resolutionFactor)
                        - ((1 as ::core::ffi::c_int) << resolutionFactor)
                        - if if resolutionFactor != 2 as ::core::ffi::c_uint {
                            BOOLEAN::FALSE.0 as ::core::ffi::c_int
                        } else {
                            BOOLEAN::TRUE.0 as ::core::ffi::c_int
                        } == 0
                        {
                            0 as ::core::ffi::c_int
                        } else {
                            10 as ::core::ffi::c_int
                        }) as COORD;
                    (*pMS).flash_rect1.extent.width = (((12 as ::core::ffi::c_int)
                        << resolutionFactor)
                        - if if resolutionFactor != 2 as ::core::ffi::c_uint {
                            BOOLEAN::FALSE.0 as ::core::ffi::c_int
                        } else {
                            BOOLEAN::TRUE.0 as ::core::ffi::c_int
                        } == 0
                        {
                            0 as ::core::ffi::c_int
                        } else {
                            10 as ::core::ffi::c_int
                        }) as COORD;
                    (*pMS).flash_rect1.extent.height = (((7 as ::core::ffi::c_int)
                        << resolutionFactor)
                        + if if resolutionFactor != 2 as ::core::ffi::c_uint {
                            BOOLEAN::FALSE.0 as ::core::ffi::c_int
                        } else {
                            BOOLEAN::TRUE.0 as ::core::ffi::c_int
                        } == 0
                        {
                            0 as ::core::ffi::c_int
                        } else {
                            4 as ::core::ffi::c_int
                        }) as COORD;
                }
                2 | 21 => {
                    (*pMS).flash_rect0.corner.x = (((70 as ::core::ffi::c_int)
                        << resolutionFactor)
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
                        } - ((1 as ::core::ffi::c_int) << resolutionFactor)
                        - if if resolutionFactor != 2 as ::core::ffi::c_uint {
                            BOOLEAN::FALSE.0 as ::core::ffi::c_int
                        } else {
                            BOOLEAN::TRUE.0 as ::core::ffi::c_int
                        } == 0
                        {
                            0 as ::core::ffi::c_int
                        } else {
                            6 as ::core::ffi::c_int
                        }) as COORD;
                    (*pMS).flash_rect0.corner.y = (((94 as ::core::ffi::c_int)
                        << resolutionFactor)
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
                        - ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
                    (*pMS).flash_rect0.extent.width = ((9 as ::core::ffi::c_int)
                        << resolutionFactor) as COORD;
                    (*pMS).flash_rect0.extent.height = ((10 as ::core::ffi::c_int)
                        << resolutionFactor) as COORD;
                    (*pMS).flash_rect1.corner.x = (((71 as ::core::ffi::c_int)
                        << resolutionFactor)
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
                        } - ((1 as ::core::ffi::c_int) << resolutionFactor)
                        - if if resolutionFactor != 2 as ::core::ffi::c_uint {
                            BOOLEAN::FALSE.0 as ::core::ffi::c_int
                        } else {
                            BOOLEAN::TRUE.0 as ::core::ffi::c_int
                        } == 0
                        {
                            0 as ::core::ffi::c_int
                        } else {
                            6 as ::core::ffi::c_int
                        }) as COORD;
                    (*pMS).flash_rect1.corner.y = (((48 as ::core::ffi::c_int)
                        << resolutionFactor)
                        - ((1 as ::core::ffi::c_int) << resolutionFactor)
                        + if if resolutionFactor != 2 as ::core::ffi::c_uint {
                            BOOLEAN::FALSE.0 as ::core::ffi::c_int
                        } else {
                            BOOLEAN::TRUE.0 as ::core::ffi::c_int
                        } == 0
                        {
                            0 as ::core::ffi::c_int
                        } else {
                            4 as ::core::ffi::c_int
                        }) as COORD;
                    (*pMS).flash_rect1.extent.width = ((7 as ::core::ffi::c_int)
                        << resolutionFactor) as COORD;
                    (*pMS).flash_rect1.extent.height = ((4 as ::core::ffi::c_int)
                        << resolutionFactor) as COORD;
                }
                _ => {
                    (*pMS).flash_rect0.corner.x = (((17 as ::core::ffi::c_int)
                        << resolutionFactor)
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
                        } - ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
                    (*pMS).flash_rect0.corner.y = (((117 as ::core::ffi::c_int)
                        << resolutionFactor)
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
                        - ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
                    (*pMS).flash_rect0.extent.width = (((12 as ::core::ffi::c_int)
                        << resolutionFactor)
                        + ((2 as ::core::ffi::c_int) << resolutionFactor)
                        + (((optWhichMenu == 0x2 as ::core::ffi::c_int)
                            as ::core::ffi::c_int) << resolutionFactor)) as COORD;
                    (*pMS).flash_rect0.extent.height = ((34 as ::core::ffi::c_int)
                        << resolutionFactor) as COORD;
                    (*pMS).flash_rect1.corner.x = (((17 as ::core::ffi::c_int)
                        << resolutionFactor)
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
                        } - ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
                    (*pMS).flash_rect1.corner.y = (((14 as ::core::ffi::c_int)
                        << resolutionFactor)
                        - ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
                    (*pMS).flash_rect1.extent.width = (((12 as ::core::ffi::c_int)
                        << resolutionFactor)
                        + ((2 as ::core::ffi::c_int) << resolutionFactor)
                        + (((optWhichMenu == 0x2 as ::core::ffi::c_int)
                            as ::core::ffi::c_int) << resolutionFactor)) as COORD;
                    (*pMS).flash_rect1.extent.height = ((25 as ::core::ffi::c_int)
                        << resolutionFactor) as COORD;
                }
            }
            w = (w as ::core::ffi::c_int * (*pMS).delta_item as ::core::ffi::c_int)
                as SIZE;
            if if optWindowType == 0 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
                && (NewState as ::core::ffi::c_int
                    == C2Rust_Unnamed_2::PLANET_LANDER.0 as ::core::ffi::c_int
                    || NewState as ::core::ffi::c_int
                        == C2Rust_Unnamed_2::NUM_MODULES.0 as ::core::ffi::c_int
                            + 3 as ::core::ffi::c_int)
            {
                (*pMS).flash_rect0.corner.x = (lander_pos[(*pMS).delta_item as usize].x
                    as ::core::ffi::c_int
                    - ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
                (*pMS).flash_rect0.corner.y = (lander_pos[(*pMS).delta_item as usize].y
                    as ::core::ffi::c_int
                    - ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
            } else {
                (*pMS).flash_rect0.corner.x = ((*pMS).flash_rect0.corner.x
                    as ::core::ffi::c_int + w as ::core::ffi::c_int) as COORD;
            }
            (*pMS).flash_rect1.corner.x = ((*pMS).flash_rect1.corner.x
                as ::core::ffi::c_int + w as ::core::ffi::c_int) as COORD;
        }
        uqm_lua_checked!(DrawModuleStrings(pMS, new_slot_piece), BOOLEAN(0));
        uqm_lua_checked!(ManipulateModules(new_slot_piece as SIZE), BOOLEAN(0));
        if ((*pMS).CurState as ::core::ffi::c_int)
            < C2Rust_Unnamed_2::NUM_MODULES.0 as ::core::ffi::c_int
        {
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
                    >(!(1 as ::core::ffi::c_long) as usize)
                },
                BOOLEAN::FALSE,
            ), BOOLEAN(0));
        } else {
            if optWhichMenu == 0x2 as ::core::ffi::c_int {
                match (*pMS).CurState as ::core::ffi::c_int {
                    23 => {
                        uqm_lua_checked!(DumpAdditionalRect(), BOOLEAN(0));
                    }
                    20 | 21 => {
                        SetAdditionalRect(
                            &raw mut (*pMS).flash_rect1,
                            BOOLEAN::TRUE.0 as ::core::ffi::c_int as COUNT,
                        );
                    }
                    _ => {
                        uqm_lua_checked!(DumpAdditionalRect(), BOOLEAN(0));
                        SetAdditionalRect(
                            &raw mut (*pMS).flash_rect1,
                            BOOLEAN::TRUE.0 as ::core::ffi::c_int as COUNT,
                        );
                    }
                }
            }
            if if optWindowType == 0 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
                && ((*pMS).CurState as ::core::ffi::c_int
                    == C2Rust_Unnamed_2::NUM_MODULES.0 as ::core::ffi::c_int
                        + 3 as ::core::ffi::c_int
                    || (*pMS).CurState as ::core::ffi::c_int
                        == C2Rust_Unnamed_2::PLANET_LANDER.0 as ::core::ffi::c_int)
                && is3DO(optWhichMenu).0 != 0
            {
                uqm_lua_checked!(SetFlashRect(&raw mut (*pMS).flash_rect0, BOOLEAN::TRUE), BOOLEAN(0));
            } else {
                uqm_lua_checked!(SetFlashRect(
                    &raw mut (*pMS).flash_rect0,
                    BOOLEAN(
                        (optWhichMenu == 0x2 as ::core::ffi::c_int) as ::core::ffi::c_int
                            as ::core::ffi::c_uint,
                    ),
                ), BOOLEAN(0));
            }
        }
    }
    return BOOLEAN::TRUE;
}
unsafe extern "C" fn ChangeFuelQuantity() {
    let mut incr: SDWORD = 0 as SDWORD;
    let maxFit: SDWORD = GetFuelTankCapacity()
        .wrapping_sub(GlobData.SIS_state.FuelOnBoard as SDWORD as DWORD) as SDWORD;
    let minFit: SDWORD = -(GlobData.SIS_state.FuelOnBoard as SDWORD);
    if PulsedInputState
        .menu[C2Rust_Unnamed_0::KEY_MENU_UP.0 as ::core::ffi::c_int as usize] != 0
    {
        incr = 100 as ::core::ffi::c_int as SDWORD;
    } else if PulsedInputState
        .menu[C2Rust_Unnamed_0::KEY_MENU_DOWN.0 as ::core::ffi::c_int as usize] != 0
    {
        incr = -100 as ::core::ffi::c_int as SDWORD;
    } else if PulsedInputState
        .menu[C2Rust_Unnamed_0::KEY_MENU_RIGHT.0 as ::core::ffi::c_int as usize] != 0
    {
        incr = (100 as ::core::ffi::c_int * 10 as ::core::ffi::c_int) as SDWORD;
    } else if PulsedInputState
        .menu[C2Rust_Unnamed_0::KEY_MENU_LEFT.0 as ::core::ffi::c_int as usize] != 0
    {
        incr = -(100 as ::core::ffi::c_int * 10 as ::core::ffi::c_int) as SDWORD;
    } else if PulsedInputState
        .menu[C2Rust_Unnamed_0::KEY_MENU_ZOOM_IN.0 as ::core::ffi::c_int as usize] != 0
    {
        incr = maxFit;
    } else if PulsedInputState
        .menu[C2Rust_Unnamed_0::KEY_MENU_ZOOM_OUT.0 as ::core::ffi::c_int as usize] != 0
    {
        incr = minFit;
    } else {
        return
    }
    if PulsedInputState
        .menu[C2Rust_Unnamed_0::KEY_MENU_ZOOM_IN.0 as ::core::ffi::c_int as usize] != 0
        || PulsedInputState
            .menu[C2Rust_Unnamed_0::KEY_MENU_ZOOM_OUT.0 as ::core::ffi::c_int as usize]
            != 0
    {
        uqm_lua_checked!(PlayMenuSound(MENU_SOUND_EFFECT::MENU_SOUND_INVOKED), ());
    }
    let maxAfford: SDWORD = GlobData
        .SIS_state
        .ResUnits
        .wrapping_div(GlobData.Game_state.FuelCost as DWORD) as SDWORD;
    if incr > maxFit {
        incr = maxFit;
    }
    if incr > maxAfford * 100 as ::core::ffi::c_int {
        incr = (maxAfford * 100 as ::core::ffi::c_int) as SDWORD;
    }
    if incr < minFit {
        incr = minFit;
    }
    if incr == 0 {
        uqm_lua_checked!(PlayMenuSound(MENU_SOUND_EFFECT::MENU_SOUND_FAILURE), ());
    } else {
        let cost: ::core::ffi::c_int = incr / 100 as ::core::ffi::c_int
            * GlobData.Game_state.FuelCost as ::core::ffi::c_int;
        uqm_lua_checked!(PreUpdateFlashRect(), ());
        uqm_lua_checked!(DeltaSISGauges(0 as SIZE, incr, -cost), ());
        uqm_lua_checked!(PostUpdateFlashRect(), ());
        uqm_lua_checked!(RedistributeFuel(), ());
    }
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut oldContext: CONTEXT = SetContext(StatusContext);
    GetGaugeRect(&raw mut r, BOOLEAN::FALSE);
    uqm_lua_checked!(SetFlashRect(&raw mut r, BOOLEAN::FALSE), ());
    SetContext(oldContext);
}
unsafe extern "C" fn onNamingDone() {
    uqm_lua_checked!(DrawFlagshipName(BOOLEAN::FALSE, BOOLEAN::FALSE.0 as ::core::ffi::c_int != 0), ());
}
#[no_mangle]
pub unsafe extern "C" fn DoOutfit(mut pMS: *mut MENU_STATE) -> BOOLEAN {
    '_ExitOutfit: {
        if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
            & C2Rust_Unnamed_6::CHECK_ABORT.0 as ::core::ffi::c_int == 0
        {
            OutfitOrShipyard = 2 as BYTE;
            if (*pMS).Initialized == 0 {
                (*pMS).InputFunc = Some(
                    DoOutfit as unsafe extern "C" fn(*mut MENU_STATE) -> BOOLEAN,
                ) as Option<unsafe extern "C" fn(*mut menu_state) -> BOOLEAN>;
                (*pMS).Initialized = BOOLEAN::TRUE.0 as ::core::ffi::c_int as SIZE;
                InitializeDOSLanderPos();
                SetNamingCallback(Some(onNamingDone as unsafe extern "C" fn() -> ()));
                let mut num_frames: COUNT = 0;
                let mut s: STAMP = STAMP {
                    origin: POINT { x: 0, y: 0 },
                    frame: ::core::ptr::null_mut::<FRAME_DESC>(),
                };
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
                (*pMS).CurFrame = CaptureDrawable(
                    uqm_lua_checked!(LoadGraphicInstance(b"graphics.modulesmenu\0".as_ptr() as RESOURCE), BOOLEAN(0))
                        as DRAWABLE,
                );
                (*pMS).hMusic = uqm_lua_checked!(LoadMusicInstance(
                    b"music.outfit\0".as_ptr() as RESOURCE,
                ), BOOLEAN(0));
                (*pMS).CurState = C2Rust_Unnamed_8::OUTFIT_FUEL.0 as ::core::ffi::c_int
                    as BYTE;
                (*pMS).ModuleFrame = CaptureDrawable(
                    uqm_lua_checked!(LoadGraphicInstance(b"graphics.outfitmodules\0".as_ptr() as RESOURCE), BOOLEAN(0))
                        as DRAWABLE,
                );
                s.origin.x = (if if optWindowType == 1 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    -(if if optWindowType == 1 as DWORD {
                        BOOLEAN::TRUE.0 as ::core::ffi::c_int
                    } else {
                        BOOLEAN::FALSE.0 as ::core::ffi::c_int
                    } != 0
                    {
                        16 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    } << resolutionFactor)
                        + ((3 as ::core::ffi::c_int) << resolutionFactor)
                } else {
                    0 as ::core::ffi::c_int
                }) as COORD;
                s.origin.y = 0 as COORD;
                s.frame = CaptureDrawable(
                    uqm_lua_checked!(LoadGraphicInstance(b"graphics.outfit\0".as_ptr() as RESOURCE), BOOLEAN(0))
                        as DRAWABLE,
                );
                if optFlagshipColor == 0x1 as ::core::ffi::c_int {
                    s.frame = SetAbsFrameIndex(s.frame, 1 as COUNT);
                } else {
                    s.frame = SetAbsFrameIndex(s.frame, 0 as COUNT);
                }
                SetTransitionSource(::core::ptr::null::<RECT>());
                BatchGraphics();
                uqm_lua_checked!(DrawSISFrame(), BOOLEAN(0));
                uqm_lua_checked!(DrawSISMessage(
                    GetStringAddress(
                        SetAbsStringTableIndex(
                            GameStrings,
                            (C2Rust_Unnamed_5::STARBASE_STRING_BASE.0
                                as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as COUNT,
                        ),
                    ),
                ), BOOLEAN(0));
                uqm_lua_checked!(DrawSISTitle(
                    GetStringAddress(
                        SetAbsStringTableIndex(
                            GameStrings,
                            C2Rust_Unnamed_5::STARBASE_STRING_BASE.0
                                as ::core::ffi::c_int as COUNT,
                        ),
                    ),
                ), BOOLEAN(0));
                SetContext(SpaceContext);
                uqm_lua_checked!(DrawStamp(&raw mut s), BOOLEAN(0));
                DestroyDrawable(ReleaseDrawable(s.frame));
                num_frames = 0 as COUNT;
                while (num_frames as ::core::ffi::c_int) < 11 as ::core::ffi::c_int {
                    let mut which_piece: BYTE = 0;
                    which_piece = GlobData.SIS_state.DriveSlots[num_frames as usize];
                    if (which_piece as ::core::ffi::c_int)
                        < C2Rust_Unnamed_2::NUM_MODULES.0 as ::core::ffi::c_int
                    {
                        uqm_lua_checked!(DrawShipPiece(
                            (*pMS).ModuleFrame,
                            which_piece as COUNT,
                            num_frames,
                            BOOLEAN::FALSE,
                        ), BOOLEAN(0));
                    }
                    num_frames = num_frames.wrapping_add(1);
                }
                num_frames = 0 as COUNT;
                while (num_frames as ::core::ffi::c_int) < 8 as ::core::ffi::c_int {
                    let mut which_piece_0: BYTE = 0;
                    which_piece_0 = GlobData.SIS_state.JetSlots[num_frames as usize];
                    if (which_piece_0 as ::core::ffi::c_int)
                        < C2Rust_Unnamed_2::NUM_MODULES.0 as ::core::ffi::c_int
                    {
                        uqm_lua_checked!(DrawShipPiece(
                            (*pMS).ModuleFrame,
                            which_piece_0 as COUNT,
                            num_frames,
                            BOOLEAN::FALSE,
                        ), BOOLEAN(0));
                    }
                    num_frames = num_frames.wrapping_add(1);
                }
                num_frames = 0 as COUNT;
                while (num_frames as ::core::ffi::c_int) < 16 as ::core::ffi::c_int {
                    let mut which_piece_1: BYTE = 0;
                    which_piece_1 = GlobData.SIS_state.ModuleSlots[num_frames as usize];
                    if (which_piece_1 as ::core::ffi::c_int)
                        < C2Rust_Unnamed_2::NUM_MODULES.0 as ::core::ffi::c_int
                    {
                        uqm_lua_checked!(DrawShipPiece(
                            (*pMS).ModuleFrame,
                            which_piece_1 as COUNT,
                            num_frames,
                            BOOLEAN::FALSE,
                        ), BOOLEAN(0));
                    }
                    num_frames = num_frames.wrapping_add(1);
                }
                uqm_lua_checked!(RedistributeFuel(), BOOLEAN(0));
                uqm_lua_checked!(DisplayLanders(pMS), BOOLEAN(0));
                if uqm_lua_checked!(getGameStateUint(
                    b"CHMMR_BOMB_STATE\0".as_ptr() as *const ::core::ffi::c_char,
                ), BOOLEAN(0)) < 3 as DWORD
                {
                    let mut ShieldFlags: BYTE = 0;
                    ShieldFlags = uqm_lua_checked!(getGameStateUint(
                        b"LANDER_SHIELDS\0".as_ptr() as *const ::core::ffi::c_char,
                    ), BOOLEAN(0)) as BYTE;
                    s.frame = SetAbsFrameIndex(
                        (*pMS).ModuleFrame,
                        (23 as ::core::ffi::c_int
                            + 2 as ::core::ffi::c_int
                                * C2Rust_Unnamed_2::NUM_PURCHASE_MODULES.0
                                    as ::core::ffi::c_int) as COUNT,
                    );
                    if ShieldFlags as ::core::ffi::c_int
                        & (1 as ::core::ffi::c_int)
                            << C2Rust_Unnamed_7::EARTHQUAKE_DISASTER.0
                                as ::core::ffi::c_int != 0
                    {
                        uqm_lua_checked!(DrawStamp(&raw mut s), BOOLEAN(0));
                    }
                    s.frame = IncFrameIndex(s.frame);
                    if ShieldFlags as ::core::ffi::c_int
                        & (1 as ::core::ffi::c_int)
                            << C2Rust_Unnamed_7::BIOLOGICAL_DISASTER.0
                                as ::core::ffi::c_int != 0
                    {
                        uqm_lua_checked!(DrawStamp(&raw mut s), BOOLEAN(0));
                    }
                    s.frame = IncFrameIndex(s.frame);
                    if ShieldFlags as ::core::ffi::c_int
                        & (1 as ::core::ffi::c_int)
                            << C2Rust_Unnamed_7::LIGHTNING_DISASTER.0
                                as ::core::ffi::c_int != 0
                    {
                        uqm_lua_checked!(DrawStamp(&raw mut s), BOOLEAN(0));
                    }
                    s.frame = IncFrameIndex(s.frame);
                    if ShieldFlags as ::core::ffi::c_int
                        & (1 as ::core::ffi::c_int)
                            << C2Rust_Unnamed_7::LAVASPOT_DISASTER.0
                                as ::core::ffi::c_int != 0
                    {
                        uqm_lua_checked!(DrawStamp(&raw mut s), BOOLEAN(0));
                    }
                }
                uqm_lua_checked!(DrawMenuStateStrings(
                    C2Rust_Unnamed_1::PM_FUEL.0 as ::core::ffi::c_int as BYTE,
                    (*pMS).CurState as SWORD,
                ), BOOLEAN(0));
                uqm_lua_checked!(DrawFlagshipName(
                    BOOLEAN::FALSE,
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int != 0,
                ), BOOLEAN(0));
                uqm_lua_checked!(DrawFlagshipStats(), BOOLEAN(0));
                uqm_lua_checked!(ScreenTransition(optScrTrans, ::core::ptr::null::<RECT>()), BOOLEAN(0));
                uqm_lua_checked!(PlayMusicResume((*pMS).hMusic, 160 as BYTE), BOOLEAN(0));
                UnbatchGraphics();
                uqm_lua_checked!(SetFlashRect(
                    ::core::ptr::with_exposed_provenance_mut::<
                        RECT,
                    >(!(0 as ::core::ffi::c_long) as usize),
                    BOOLEAN::FALSE,
                ), BOOLEAN(0));
                GlobData.SIS_state.FuelOnBoard = GlobData
                    .SIS_state
                    .FuelOnBoard
                    .wrapping_add(
                        (100 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as DWORD,
                    )
                    .wrapping_div(100 as DWORD);
                GlobData.SIS_state.FuelOnBoard = GlobData
                    .SIS_state
                    .FuelOnBoard
                    .wrapping_mul(100 as DWORD);
                SetContext(StatusContext);
            } else if PulsedInputState
                .menu[C2Rust_Unnamed_0::KEY_MENU_CANCEL.0 as ::core::ffi::c_int as usize]
                != 0
                || PulsedInputState
                    .menu[C2Rust_Unnamed_0::KEY_MENU_SELECT.0 as ::core::ffi::c_int
                    as usize] != 0
                    && (*pMS).CurState as ::core::ffi::c_int
                        == C2Rust_Unnamed_8::OUTFIT_EXIT.0 as ::core::ffi::c_int
            {
                if (*pMS).CurState as ::core::ffi::c_int
                    == C2Rust_Unnamed_8::OUTFIT_DOFUEL.0 as ::core::ffi::c_int
                {
                    (*pMS).CurState = C2Rust_Unnamed_8::OUTFIT_FUEL.0
                        as ::core::ffi::c_int as BYTE;
                    uqm_lua_checked!(DrawMenuStateStrings(
                        C2Rust_Unnamed_1::PM_FUEL.0 as ::core::ffi::c_int as BYTE,
                        (*pMS).CurState as SWORD,
                    ), BOOLEAN(0));
                    uqm_lua_checked!(SetFlashRect(
                        ::core::ptr::with_exposed_provenance_mut::<
                            RECT,
                        >(!(0 as ::core::ffi::c_long) as usize),
                        BOOLEAN::FALSE,
                    ), BOOLEAN(0));
                } else {
                    break '_ExitOutfit;
                }
            } else if PulsedInputState
                .menu[C2Rust_Unnamed_0::KEY_MENU_SELECT.0 as ::core::ffi::c_int as usize]
                != 0
            {
                match (*pMS).CurState as ::core::ffi::c_int {
                    0 => {
                        let mut r: RECT = RECT {
                            corner: POINT { x: 0, y: 0 },
                            extent: EXTENT { width: 0, height: 0 },
                        };
                        uqm_lua_checked!(DrawMenuStateStrings(
                            C2Rust_Unnamed_1::PM_FUEL.0 as ::core::ffi::c_int as BYTE,
                            (*pMS).CurState as SWORD,
                        ), BOOLEAN(0));
                        (*pMS).CurState = C2Rust_Unnamed_8::OUTFIT_DOFUEL.0
                            as ::core::ffi::c_int as BYTE;
                        SetContext(StatusContext);
                        GetGaugeRect(&raw mut r, BOOLEAN::FALSE);
                        uqm_lua_checked!(SetFlashRect(&raw mut r, BOOLEAN::FALSE), BOOLEAN(0));
                    }
                    4 => {
                        (*pMS).CurState = C2Rust_Unnamed_8::OUTFIT_FUEL.0
                            as ::core::ffi::c_int as BYTE;
                        uqm_lua_checked!(DrawMenuStateStrings(
                            C2Rust_Unnamed_1::PM_FUEL.0 as ::core::ffi::c_int as BYTE,
                            (*pMS).CurState as SWORD,
                        ), BOOLEAN(0));
                        uqm_lua_checked!(SetFlashRect(
                            ::core::ptr::with_exposed_provenance_mut::<
                                RECT,
                            >(!(0 as ::core::ffi::c_long) as usize),
                            BOOLEAN::FALSE,
                        ), BOOLEAN(0));
                    }
                    1 => {
                        if optDosMenus.0 != 0
                            || if optWindowType == 0 as DWORD {
                                BOOLEAN::TRUE.0 as ::core::ffi::c_int
                            } else {
                                BOOLEAN::FALSE.0 as ::core::ffi::c_int
                            } != 0
                        {
                            memset(
                                &raw mut ModuleState as *mut ::core::ffi::c_void,
                                0 as ::core::ffi::c_int,
                                ::core::mem::size_of::<MODULES_STATE>(),
                            );
                            ModuleState.count = InventoryModules(
                                &raw mut ModuleState.list as *mut BYTE,
                                C2Rust_Unnamed_2::NUM_PURCHASE_MODULES.0
                                    as ::core::ffi::c_int as COUNT,
                            ) as COUNT;
                        }
                        uqm_lua_checked!(DrawMenuStateStrings(
                            C2Rust_Unnamed_1::PM_FUEL.0 as ::core::ffi::c_int as BYTE,
                            (*pMS).CurState as SWORD,
                        ), BOOLEAN(0));
                        (*pMS).CurState = (C2Rust_Unnamed_2::NUM_MODULES.0
                            as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as BYTE;
                        if uqm_lua_checked!(getGameStateUint(
                            b"CHMMR_BOMB_STATE\0".as_ptr() as *const ::core::ffi::c_char,
                        ), BOOLEAN(0)) != 3 as DWORD
                        {
                            (*pMS).delta_item = 0 as SIZE;
                        } else {
                            (*pMS).delta_item = 10 as SIZE;
                        }
                        (*pMS).first_item.y = 0 as COORD;
                        (*pMS).Initialized = 0 as SIZE;
                        uqm_lua_checked!(DoInstallModule(pMS), BOOLEAN(0));
                    }
                    2 => {
                        if uqm_lua_checked!(GameOptions(), BOOLEAN(0)).0 == 0 {
                            break '_ExitOutfit;
                        } else {
                            uqm_lua_checked!(DrawMenuStateStrings(
                                C2Rust_Unnamed_1::PM_FUEL.0 as ::core::ffi::c_int as BYTE,
                                (*pMS).CurState as SWORD,
                            ), BOOLEAN(0));
                            uqm_lua_checked!(SetFlashRect(
                                ::core::ptr::with_exposed_provenance_mut::<
                                    RECT,
                                >(!(0 as ::core::ffi::c_long) as usize),
                                BOOLEAN::FALSE,
                            ), BOOLEAN(0));
                        }
                    }
                    _ => {}
                }
            } else if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
                & C2Rust_Unnamed_6::CHECK_LOAD.0 as ::core::ffi::c_int != 0
            {
                break '_ExitOutfit;
            } else {
                match (*pMS).CurState as ::core::ffi::c_int {
                    4 => {
                        SetMenuSounds(
                            (((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int)
                                as MENU_SOUND_FLAGS as ::core::ffi::c_int
                                | ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
                                    as MENU_SOUND_FLAGS as ::core::ffi::c_int
                                | ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                                    as MENU_SOUND_FLAGS as ::core::ffi::c_int
                                | ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                                    as MENU_SOUND_FLAGS as ::core::ffi::c_int)
                                as MENU_SOUND_FLAGS,
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
                    }
                    _ => {
                        SetMenuSounds(
                            (((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int)
                                as MENU_SOUND_FLAGS as ::core::ffi::c_int
                                | ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
                                    as MENU_SOUND_FLAGS as ::core::ffi::c_int
                                | ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                                    as MENU_SOUND_FLAGS as ::core::ffi::c_int
                                | ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                                    as MENU_SOUND_FLAGS as ::core::ffi::c_int)
                                as MENU_SOUND_FLAGS,
                            ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                                as MENU_SOUND_FLAGS,
                        );
                    }
                }
                if (*pMS).CurState as ::core::ffi::c_int
                    == C2Rust_Unnamed_8::OUTFIT_DOFUEL.0 as ::core::ffi::c_int
                {
                    if optInfiniteFuel.0 == 0 {
                        uqm_lua_checked!(ChangeFuelQuantity(), BOOLEAN(0));
                    }
                    uqm_lua_checked!(SleepThread(
                        (840 as ::core::ffi::c_int / 30 as ::core::ffi::c_int)
                            as TimePeriod,
                    ), BOOLEAN(0));
                } else {
                    uqm_lua_checked!(DoMenuChooser(
                        pMS,
                        C2Rust_Unnamed_1::PM_FUEL.0 as ::core::ffi::c_int as BYTE,
                    ), BOOLEAN(0));
                }
            }
            if optInfiniteFuel.0 != 0 {
                uqm_lua_checked!(DeltaSISGauges(
                    0 as SIZE,
                    GetFuelTankCapacity() as SDWORD,
                    0 as ::core::ffi::c_int,
                ), BOOLEAN(0));
                uqm_lua_checked!(RedistributeFuel(), BOOLEAN(0));
            }
            if optInfiniteRU.0 != 0 {
                GlobData.SIS_state.ResUnits = 1000000 as ::core::ffi::c_long as DWORD;
            }
            return BOOLEAN::TRUE;
        }
    }
    if ((*pMS).CurState as ::core::ffi::c_int)
        < C2Rust_Unnamed_8::OUTFIT_EXIT.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(DrawMenuStateStrings(
            C2Rust_Unnamed_1::PM_FUEL.0 as ::core::ffi::c_int as BYTE,
            C2Rust_Unnamed_8::OUTFIT_EXIT.0 as ::core::ffi::c_int as SWORD,
        ), BOOLEAN(0));
    }
    DestroyDrawable(ReleaseDrawable((*pMS).CurFrame));
    (*pMS).CurFrame = ::core::ptr::null_mut::<FRAME_DESC>();
    DestroyDrawable(ReleaseDrawable((*pMS).ModuleFrame));
    (*pMS).ModuleFrame = ::core::ptr::null_mut::<FRAME_DESC>();
    if if optWindowType == 0 as DWORD {
        BOOLEAN::TRUE.0 as ::core::ffi::c_int
    } else {
        BOOLEAN::FALSE.0 as ::core::ffi::c_int
    } == 0
    {
        DestroyFont(ModuleFont);
    }
    uqm_lua_checked!(SetMusicPosition(), BOOLEAN(0));
    SetNamingCallback(None);
    return BOOLEAN::FALSE;
}
