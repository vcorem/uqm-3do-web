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

// Rust translation of src/uqm/planets/pstarmap.c.
extern "C" {
    pub type string_table_entry;
    pub type context_desc;
    pub type frame_desc;
    pub type font_desc;
    pub type drawable_desc;
    pub type tfb_soundsample;
    pub type joy_char;
    pub type FlashContext;
    fn atan2(
        _: ::core::ffi::c_double,
        _: ::core::ffi::c_double,
    ) -> ::core::ffi::c_double;
    fn cos(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn pow(_: ::core::ffi::c_double, _: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn sin(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn sqrt(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn HMalloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn HFree(p: *mut ::core::ffi::c_void);
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
    fn SetAbsStringTableIndex(String: STRING, StringTableIndex: COUNT) -> STRING;
    fn GetStringLength(String: STRING) -> COUNT;
    fn GetStringAddress(String: STRING) -> STRINGPTR;
    fn getCharFromString(ptr: *mut *const ::core::ffi::c_char) -> UniChar;
    fn utf8StringCount(start: *const ::core::ffi::c_char) -> size_t;
    fn utf8StringCountN(
        start: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
    ) -> size_t;
    fn utf8StringCopy(
        dst: *mut ::core::ffi::c_char,
        size: size_t,
        src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_uchar;
    fn utf8StringCompare(
        str1: *const ::core::ffi::c_char,
        str2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn skipUTF8Chars(
        ptr: *const ::core::ffi::c_char,
        num: size_t,
    ) -> *mut ::core::ffi::c_char;
    fn UniChar_toUpper(ch: UniChar) -> UniChar;
    fn GetTimeCounter() -> TimeCount;
    fn SleepThread(timePeriod: TimePeriod);
    fn SleepThreadUntil(wakeTime: TimeCount);
    fn BoxUnion(pr1: *mut RECT, pr2: *mut RECT, punion: *mut RECT);
    fn SetContext(Context: CONTEXT) -> CONTEXT;
    fn SetContextForeGroundColor(Color: Color) -> Color;
    fn SetContextBackGroundColor(Color: Color) -> Color;
    fn SetContextClipRect(pRect: *mut RECT) -> BOOLEAN;
    fn GetContextClipRect(pRect: *mut RECT) -> BOOLEAN;
    fn SetContextOrigin(orgOffset: POINT) -> POINT;
    fn DrawStamp(pStamp: *mut STAMP);
    fn DrawFilledStamp(pStamp: *mut STAMP);
    fn DrawPoint(pPoint: *mut POINT);
    fn DrawFilledRectangle(pRect: *mut RECT);
    fn DrawLine(pLine: *mut LINE, thickness: BYTE);
    fn font_GetTextRect(pText: *mut TEXT) -> RECT;
    fn font_DrawText(pText: *mut TEXT);
    fn BatchGraphics();
    fn UnbatchGraphics();
    fn ClearDrawable();
    fn DestroyDrawable(Drawable: DRAWABLE) -> BOOLEAN;
    fn GetFrameRect(Frame: FRAME, pRect: *mut RECT) -> BOOLEAN;
    fn LoadGraphicInstance(res: RESOURCE) -> *mut ::core::ffi::c_void;
    fn SetContextFont(Font: FONT) -> FONT;
    fn TextRect(pText: *mut TEXT, pRect: *mut RECT, pdelta: *mut BYTE) -> BOOLEAN;
    fn GetContextFontLeading(pheight: *mut SIZE) -> BOOLEAN;
    fn GetContextFontDispHeight(pheight: *mut SIZE) -> BOOLEAN;
    fn GetContextFontVertAlign(valign: *mut SBYTE) -> SBYTE;
    fn SetAbsFrameIndex(Frame: FRAME, FrameIndex: COUNT) -> FRAME;
    fn SetRelFrameIndex(Frame: FRAME, FrameOffs: SIZE) -> FRAME;
    fn IncFrameIndex(Frame: FRAME) -> FRAME;
    fn DecFrameIndex(Frame: FRAME) -> FRAME;
    fn CaptureDrawable(Drawable: DRAWABLE) -> FRAME;
    fn ReleaseDrawable(Frame: FRAME) -> DRAWABLE;
    fn atof(_: *const ::core::ffi::c_char) -> ::core::ffi::c_double;
    fn qsort(
        _: *mut ::core::ffi::c_void,
        _: size_t,
        _: size_t,
        _: Option<
            unsafe extern "C" fn(
                *const ::core::ffi::c_void,
                *const ::core::ffi::c_void,
            ) -> ::core::ffi::c_int,
        >,
    );
    fn abs(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn memset(
        _: *mut ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strcpy(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strcmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strtok(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(_: *const ::core::ffi::c_char) -> size_t;
    fn isdigit(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn ControlAtlas(menu_index: ::core::ffi::c_int, atlas_array: *mut FRAME) -> FRAME;
    static mut CurrentInputState: CONTROLLER_INPUT_STATE;
    static mut PulsedInputState: CONTROLLER_INPUT_STATE;
    fn FlushInput();
    fn SetMenuRepeatDelay(min: DWORD, max: DWORD, step: DWORD, gestalt: BOOLEAN);
    fn SetDefaultMenuRepeatDelay();
    fn DoInput(pInputState: *mut ::core::ffi::c_void, resetInput: BOOLEAN);
    fn DoTextEntry(pTES: *mut TEXTENTRY_STATE) -> BOOLEAN;
    static mut optWhichMenu: ::core::ffi::c_int;
    static mut optWhichFonts: ::core::ffi::c_int;
    static mut resolutionFactor: ::core::ffi::c_uint;
    static mut optBubbleWarp: OPT_ENABLABLE;
    static mut optInfiniteFuel: OPT_ENABLABLE;
    static mut optSubmenu: OPT_ENABLABLE;
    static mut optSeedType: ::core::ffi::c_int;
    static mut optSphereColors: ::core::ffi::c_int;
    static mut optScrTrans: ::core::ffi::c_int;
    static mut optFuelRange: ::core::ffi::c_int;
    static mut optSmartAutoPilot: OPT_ENABLABLE;
    static mut optShowVisitedStars: OPT_ENABLABLE;
    static mut optAdvancedAutoPilot: OPT_ENABLABLE;
    static mut optWindowType: DWORD;
    static mut CanvasWidth: ::core::ffi::c_int;
    static mut CanvasHeight: ::core::ffi::c_int;
    fn DrawHyperCoords(puniverse: POINT);
    fn DrawSISMessageEx(
        pStr: *const UNICODE,
        CurPos: SIZE,
        ExPos: SIZE,
        flags: COUNT,
    ) -> BOOLEAN;
    fn DrawSISMessage(pStr: *const UNICODE);
    fn DrawStatusMessage(pStr: *const UNICODE);
    fn DrawAutoPilotMessage(Reset: BOOLEAN);
    fn DeltaSISGauges(
        crew_delta: SIZE,
        fuel_delta: SDWORD,
        resunit_delta: ::core::ffi::c_int,
    );
    fn setGameStateUint(name: *const ::core::ffi::c_char, val: DWORD);
    fn getGameStateUint(name: *const ::core::ffi::c_char) -> DWORD;
    static mut GlobData: GLOBDATA;
    fn inHQSpace() -> BOOLEAN;
    fn inHyperSpace() -> BOOLEAN;
    fn inQuasiSpace() -> BOOLEAN;
    fn isPC(optWhich: ::core::ffi::c_int) -> OPT_CONSOLETYPE;
    fn is3DO(optWhich: ::core::ffi::c_int) -> OPT_CONSOLETYPE;
    fn square_root(value: DWORD) -> COUNT;
    fn playerInSolarSystem() -> bool;
    fn playerInPlanetOrbit() -> bool;
    fn DrawOval(pRect: *mut DRECT, num_off_pixels: BYTE, scaled: BOOLEAN);
    fn DrawFilledOval(pRect: *mut DRECT);
    fn DrawMenuStateStrings(beg_index: BYTE, NewState: SWORD);
    static mut CurStarDescPtr: *mut STAR_DESC;
    static mut star_array: [STAR_DESC; 0];
    static mut constel_array: *mut POINT;
    static mut plot_map: [PLOT_LOCATION; 70];
    static mut portal_map: [PORTAL_LOCATION; 16];
    fn FindStar(
        pLastStar: *mut STAR_DESC,
        puniverse: *mut POINT,
        xbounds: SIZE,
        ybounds: SIZE,
    ) -> *mut STAR_DESC;
    fn GetClusterName(pSD: *const STAR_DESC, buf: *mut UNICODE);
    static mut GameStrings: STRING;
    fn FlushCursorRect();
    static mut TextEntry3DO: BOOLEAN;
    fn InvokeSpawner() -> BOOLEAN;
    fn RepairBackRect(pRect: *mut RECT);
    static mut MiscDataFrame: FRAME;
    static mut visitedStarsFrame: FRAME;
    static mut OffScreenContext: CONTEXT;
    static mut SpaceContext: CONTEXT;
    static mut StatusContext: CONTEXT;
    static mut TinyFont: FONT;
    static mut TinyFontBold: FONT;
    static mut TinyFontCond: FONT;
    static mut stars_in_space: FRAME;
    static mut debugHook: Option<unsafe extern "C" fn() -> ()>;
    fn doInstantMove();
    fn AnyButtonPress(DetectSpecial: BOOLEAN) -> BOOLEAN;
    fn DirKeysPress() -> BOOLEAN;
    fn LoadIntoExtraScreen(r: *mut RECT);
    fn SetTransitionSource(pRect: *const RECT);
    fn ScreenTransition(transition: ::core::ffi::c_int, pRect: *const RECT);
    fn DrawFlagStatDisplay(str: *mut UNICODE);
    fn WholeFuelValue() -> *mut UNICODE;
    fn CheckAlliance(race: RACE_ID) -> COUNT;
    fn WarEraStrength(SpeciesID: SPECIES_ID) -> COUNT;
    fn SeedFleetLocation(
        FleetPtr: *mut FLEET_INFO,
        plotmap: *mut PLOT_LOCATION,
        visit: COUNT,
    ) -> POINT;
    fn SetMenuSounds(sound_0: MENU_SOUND_FLAGS, sound_1: MENU_SOUND_FLAGS);
    fn PlayMenuSound(S: MENU_SOUND_EFFECT);
}
pub type COUNT = UWORD;
pub type UWORD = uint16;
pub type uint16 = ::core::ffi::c_ushort;
pub type POINT = point;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct point {
    pub x: COORD,
    pub y: COORD,
}
pub type COORD = SWORD;
pub type SWORD = sint16;
pub type sint16 = ::core::ffi::c_short;
pub type uint8 = ::core::ffi::c_uchar;
pub type sint8 = ::core::ffi::c_schar;
pub type uint32 = ::core::ffi::c_uint;
pub type sint32 = ::core::ffi::c_int;
pub type BYTE = uint8;
pub type SBYTE = sint8;
pub type DWORD = uint32;
pub type SDWORD = sint32;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct extent {
    pub width: COORD,
    pub height: COORD,
}
pub type EXTENT = extent;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dextent {
    pub width: SDWORD,
    pub height: SDWORD,
}
pub type DEXTENT = dextent;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dpoint {
    pub x: SDWORD,
    pub y: SDWORD,
}
pub type DPOINT = dpoint;
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
pub struct drect {
    pub corner: DPOINT,
    pub extent: DEXTENT,
}
pub type DRECT = drect;
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
pub type joy_char_t = joy_char;
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
pub type TEXTENTRY_STATE = textentry_state;
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_0 {
    pub const OPTVAL_PRIME: Self = Self(0);
    pub const OPTVAL_PLANET: Self = Self(1);
    pub const OPTVAL_MRQ: Self = Self(2);
    pub const OPTVAL_STAR: Self = Self(3);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_1(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_1 {
    pub const OPTVAL_DEFAULT_COLORS: Self = Self(0);
    pub const OPTVAL_STARSEED_COLORS: Self = Self(1);
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
pub struct star_desc {
    pub star_pt: POINT,
    pub Type: BYTE,
    pub Index: BYTE,
    pub Prefix: BYTE,
    pub Postfix: BYTE,
}
pub type STAR_DESC = star_desc;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct plot_location {
    pub star_pt: POINT,
    pub star: *mut STAR_DESC,
    pub dist_sq: [DWORD; 70],
}
pub type PLOT_LOCATION = plot_location;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct portal_location {
    pub star_pt: POINT,
    pub quasi_pt: POINT,
    pub nearest_star: *mut STAR_DESC,
}
pub type PORTAL_LOCATION = portal_location;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_3(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_3 {
    pub const DWARF_STAR: Self = Self(0);
    pub const GIANT_STAR: Self = Self(1);
    pub const SUPER_GIANT_STAR: Self = Self(2);
    pub const NUM_STAR_TYPES: Self = Self(3);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_4(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_4 {
    pub const BLUE_BODY: Self = Self(0);
    pub const GREEN_BODY: Self = Self(1);
    pub const ORANGE_BODY: Self = Self(2);
    pub const RED_BODY: Self = Self(3);
    pub const WHITE_BODY: Self = Self(4);
    pub const GRAY_BODY: Self = Self(4);
    pub const YELLOW_BODY: Self = Self(5);
    pub const NUM_STAR_COLORS: Self = Self(6);
    pub const CYAN_BODY: Self = Self(6);
    pub const PURPLE_BODY: Self = Self(7);
    pub const VIOLET_BODY: Self = Self(8);
    pub const NUM_PLANET_COLORS: Self = Self(9);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_5(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_5 {
    pub const ARILOU_DEFINED: Self = Self(0);
    pub const SOL_DEFINED: Self = Self(1);
    pub const SHOFIXTI_DEFINED: Self = Self(2);
    pub const MAIDENS_DEFINED: Self = Self(3);
    pub const START_COLONY_DEFINED: Self = Self(4);
    pub const SPATHI_DEFINED: Self = Self(5);
    pub const ZOQFOT_DEFINED: Self = Self(6);
    pub const MELNORME0_DEFINED: Self = Self(7);
    pub const MELNORME1_DEFINED: Self = Self(8);
    pub const MELNORME2_DEFINED: Self = Self(9);
    pub const MELNORME3_DEFINED: Self = Self(10);
    pub const MELNORME4_DEFINED: Self = Self(11);
    pub const MELNORME5_DEFINED: Self = Self(12);
    pub const MELNORME6_DEFINED: Self = Self(13);
    pub const MELNORME7_DEFINED: Self = Self(14);
    pub const MELNORME8_DEFINED: Self = Self(15);
    pub const TALKING_PET_DEFINED: Self = Self(16);
    pub const CHMMR_DEFINED: Self = Self(17);
    pub const SYREEN_DEFINED: Self = Self(18);
    pub const BURVIXESE_DEFINED: Self = Self(19);
    pub const SLYLANDRO_DEFINED: Self = Self(20);
    pub const DRUUGE_DEFINED: Self = Self(21);
    pub const BOMB_DEFINED: Self = Self(22);
    pub const AQUA_HELIX_DEFINED: Self = Self(23);
    pub const SUN_DEVICE_DEFINED: Self = Self(24);
    pub const TAALO_PROTECTOR_DEFINED: Self = Self(25);
    pub const SHIP_VAULT_DEFINED: Self = Self(26);
    pub const URQUAN_WRECK_DEFINED: Self = Self(27);
    pub const VUX_BEAST_DEFINED: Self = Self(28);
    pub const SAMATRA_DEFINED: Self = Self(29);
    pub const ZOQ_SCOUT_DEFINED: Self = Self(30);
    pub const MYCON_DEFINED: Self = Self(31);
    pub const EGG_CASE0_DEFINED: Self = Self(32);
    pub const EGG_CASE1_DEFINED: Self = Self(33);
    pub const EGG_CASE2_DEFINED: Self = Self(34);
    pub const PKUNK_DEFINED: Self = Self(35);
    pub const UTWIG_DEFINED: Self = Self(36);
    pub const SUPOX_DEFINED: Self = Self(37);
    pub const YEHAT_DEFINED: Self = Self(38);
    pub const VUX_DEFINED: Self = Self(39);
    pub const ORZ_DEFINED: Self = Self(40);
    pub const THRADD_DEFINED: Self = Self(41);
    pub const RAINBOW0_DEFINED: Self = Self(42);
    pub const RAINBOW1_DEFINED: Self = Self(43);
    pub const RAINBOW2_DEFINED: Self = Self(44);
    pub const RAINBOW3_DEFINED: Self = Self(45);
    pub const RAINBOW4_DEFINED: Self = Self(46);
    pub const RAINBOW5_DEFINED: Self = Self(47);
    pub const RAINBOW6_DEFINED: Self = Self(48);
    pub const RAINBOW7_DEFINED: Self = Self(49);
    pub const RAINBOW8_DEFINED: Self = Self(50);
    pub const RAINBOW9_DEFINED: Self = Self(51);
    pub const ILWRATH_DEFINED: Self = Self(52);
    pub const ANDROSYNTH_DEFINED: Self = Self(53);
    pub const MYCON_TRAP_DEFINED: Self = Self(54);
    pub const URQUAN0_DEFINED: Self = Self(55);
    pub const URQUAN1_DEFINED: Self = Self(56);
    pub const URQUAN2_DEFINED: Self = Self(57);
    pub const KOHRAH0_DEFINED: Self = Self(58);
    pub const KOHRAH1_DEFINED: Self = Self(59);
    pub const KOHRAH2_DEFINED: Self = Self(60);
    pub const DESTROYED_STARBASE_DEFINED: Self = Self(61);
    pub const MOTHER_ARK_DEFINED: Self = Self(62);
    pub const ZOQ_COLONY0_DEFINED: Self = Self(63);
    pub const ZOQ_COLONY1_DEFINED: Self = Self(64);
    pub const ZOQ_COLONY2_DEFINED: Self = Self(65);
    pub const ZOQ_COLONY3_DEFINED: Self = Self(66);
    pub const ALGOLITES_DEFINED: Self = Self(67);
    pub const SPATHI_MONUMENT_DEFINED: Self = Self(68);
    pub const EXCAVATION_SITE_DEFINED: Self = Self(69);
    pub const NUM_PLOTS: Self = Self(70);
    pub const WAR_ERA: Self = Self(70);
    pub const SUCCESS: Self = Self(70);
    pub const HOME: Self = Self(71);
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
pub struct C2Rust_Unnamed_7(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_7 {
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_8(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_8 {
    pub const CHMMR_HOME: Self = Self(0);
    pub const ORZ_HOME: Self = Self(1);
    pub const PKUNK_HOME: Self = Self(2);
    pub const SHOFIXTI_HOME: Self = Self(3);
    pub const SPATHI_HOME: Self = Self(4);
    pub const SUPOX_HOME: Self = Self(5);
    pub const THRADDASH_HOME: Self = Self(6);
    pub const UTWIG_HOME: Self = Self(7);
    pub const VUX_HOME: Self = Self(8);
    pub const YEHAT_HOME: Self = Self(9);
    pub const DRUUGE_HOME: Self = Self(10);
    pub const ILWRATH_HOME: Self = Self(11);
    pub const MYCON_HOME: Self = Self(12);
    pub const SLYLANDRO_HOME: Self = Self(13);
    pub const UMGAH_HOME: Self = Self(14);
    pub const ZOQFOTPIK_HOME: Self = Self(15);
    pub const SYREEN_HOME: Self = Self(16);
    pub const ANDROSYNTH_HOME: Self = Self(17);
    pub const NUM_AVAILABLE_HOMES: Self = Self(18);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_9(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_9 {
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
pub type MENU_SOUND_FLAGS = UWORD;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct CURRENT_STARMAP_SHOWN(pub ::core::ffi::c_uint);
impl CURRENT_STARMAP_SHOWN {
    pub const NORMAL_STARMAP: Self = Self(0);
    pub const WAR_ERA_STARMAP: Self = Self(1);
    pub const CONSTELLATION_MAP: Self = Self(2);
    pub const HOMEWORLDS_MAP: Self = Self(3);
    pub const RAINBOW_MAP: Self = Self(4);
    pub const NUM_STARMAPS: Self = Self(5);
}
pub type NAMEPLATE = namePlate;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct namePlate {
    pub rect: RECT,
    pub text: TEXT,
    pub index: BYTE,
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
pub type STAR_SEARCH_STATE = starsearch_state;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct starsearch_state {
    pub pMS: *mut MENU_STATE,
    pub Text: [UNICODE; 256],
    pub LastText: [UNICODE; 256],
    pub LastChangeTime: DWORD,
    pub FirstIndex: ::core::ffi::c_int,
    pub CurIndex: ::core::ffi::c_int,
    pub LastIndex: ::core::ffi::c_int,
    pub SingleClust: BOOLEAN,
    pub SingleMatch: BOOLEAN,
    pub Buffer: [UNICODE; 256],
    pub Prefix: *const UNICODE,
    pub Cluster: *const UNICODE,
    pub PrefixLen: ::core::ffi::c_int,
    pub ClusterLen: ::core::ffi::c_int,
    pub ClusterPos: ::core::ffi::c_int,
    pub SortedStars: [::core::ffi::c_int; 502],
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
                    b"src/libs/math/../../uqm/displist.h\0".as_ptr()
                        as *const ::core::ffi::c_char,
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
                    b"src/libs/math/../../uqm/displist.h\0".as_ptr()
                        as *const ::core::ffi::c_char,
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
unsafe extern "C" fn RECT_TO_DRECT(mut r: RECT) -> DRECT {
    let mut dr: DRECT = drect {
        corner: dpoint {
            x: r.corner.x as SDWORD,
            y: r.corner.y as SDWORD,
        },
        extent: dextent {
            width: r.extent.width as SDWORD,
            height: r.extent.height as SDWORD,
        },
    };
    return dr;
}
#[inline]
unsafe extern "C" fn MAKE_LINE(
    mut line: *mut LINE,
    mut first: POINT,
    mut second: POINT,
) {
    (*line).first = first;
    (*line).second = second;
}
#[inline]
unsafe extern "C" fn pointsEqual(mut p1: POINT, mut p2: POINT) -> bool {
    return p1.x as ::core::ffi::c_int == p2.x as ::core::ffi::c_int
        && p1.y as ::core::ffi::c_int == p2.y as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn ptDistance(mut p1: POINT, mut p2: POINT) -> ::core::ffi::c_double {
    return sqrt(
        pow(
            p2.x as ::core::ffi::c_double - p1.x as ::core::ffi::c_double,
            2 as ::core::ffi::c_int as ::core::ffi::c_double,
        )
            + pow(
                p2.y as ::core::ffi::c_double - p1.y as ::core::ffi::c_double,
                2 as ::core::ffi::c_int as ::core::ffi::c_double,
            ),
    );
}
#[inline]
unsafe extern "C" fn ValidPoint(mut pt: POINT) -> BOOLEAN {
    return BOOLEAN(
        (pt.x as ::core::ffi::c_int != !(0 as ::core::ffi::c_int)
            && pt.y as ::core::ffi::c_int != !(0 as ::core::ffi::c_int))
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
unsafe extern "C" fn SaveLastLoc(mut pt: POINT) {
    uqm_lua_checked!(setGameStateUint(
        b"LAST_LOCATION_X\0".as_ptr() as *const ::core::ffi::c_char,
        pt.x as DWORD,
    ), ());
    uqm_lua_checked!(setGameStateUint(
        b"LAST_LOCATION_Y\0".as_ptr() as *const ::core::ffi::c_char,
        pt.y as DWORD,
    ), ());
}
#[inline]
unsafe extern "C" fn SaveAdvancedQuasiPilot(mut pt: POINT) {
    uqm_lua_checked!(setGameStateUint(
        b"ADV_AUTOPILOT_QUASI_X\0".as_ptr() as *const ::core::ffi::c_char,
        pt.x as DWORD,
    ), ());
    uqm_lua_checked!(setGameStateUint(
        b"ADV_AUTOPILOT_QUASI_Y\0".as_ptr() as *const ::core::ffi::c_char,
        pt.y as DWORD,
    ), ());
}
#[inline]
unsafe extern "C" fn SaveAdvancedAutoPilot(mut pt: POINT) {
    uqm_lua_checked!(setGameStateUint(
        b"ADV_AUTOPILOT_SAVE_X\0".as_ptr() as *const ::core::ffi::c_char,
        pt.x as DWORD,
    ), ());
    uqm_lua_checked!(setGameStateUint(
        b"ADV_AUTOPILOT_SAVE_Y\0".as_ptr() as *const ::core::ffi::c_char,
        pt.y as DWORD,
    ), ());
}
#[inline]
unsafe extern "C" fn IsHomeworldKnown(mut homeworld: DWORD) -> BOOLEAN {
    if homeworld > 18 as DWORD {
        return BOOLEAN::FALSE;
    }
    return BOOLEAN(
        uqm_lua_checked!(getGameStateUint(b"KNOW_HOMEWORLD\0".as_ptr() as *const ::core::ffi::c_char), BOOLEAN(0))
            & ((1 as ::core::ffi::c_int) << homeworld) as DWORD,
    );
}
unsafe extern "C" fn SwapPlates(mut n1: *mut NAMEPLATE, mut n2: *mut NAMEPLATE) {
    let mut temp: NAMEPLATE = *n1;
    *n1 = *n2;
    *n2 = temp;
}
static mut cursorLoc: POINT = POINT { x: 0, y: 0 };
static mut mapOrigin: POINT = POINT { x: 0, y: 0 };
static mut zoomLevel: ::core::ffi::c_int = 0;
static mut StarMapFrame: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
static mut which_starmap: CURRENT_STARMAP_SHOWN = CURRENT_STARMAP_SHOWN::NORMAL_STARMAP;
#[inline]
unsafe extern "C" fn signedDivWithError(
    mut val: ::core::ffi::c_long,
    mut divisor: ::core::ffi::c_long,
) -> ::core::ffi::c_long {
    let mut invert: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if val < 0 as ::core::ffi::c_long {
        invert = 1 as ::core::ffi::c_int;
        val = -val;
    }
    val = (val + (divisor >> 1 as ::core::ffi::c_int)) / divisor;
    return if invert != 0 { -val } else { val };
}
#[inline]
unsafe extern "C" fn universeToDispx(mut ux: ::core::ffi::c_long) -> COORD {
    return (signedDivWithError(
        ((ux - mapOrigin.x as ::core::ffi::c_long) << zoomLevel)
            * (CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
                    } << resolutionFactor)) >> resolutionFactor) as ::core::ffi::c_long,
        (9999 as ::core::ffi::c_int
            + ((9999 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                / (CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
                + 1 as ::core::ffi::c_int)) as ::core::ffi::c_long,
    )
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
                } << resolutionFactor)) >> resolutionFactor) - 1 as ::core::ffi::c_int
            >> 1 as ::core::ffi::c_int) as ::core::ffi::c_long) as COORD;
}
#[inline]
unsafe extern "C" fn universeToDispy(mut uy: ::core::ffi::c_long) -> COORD {
    return (signedDivWithError(
        (mapOrigin.y as ::core::ffi::c_long - uy << zoomLevel)
            * (CanvasHeight
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
                - ((13 as ::core::ffi::c_int) << resolutionFactor) >> resolutionFactor)
                as ::core::ffi::c_long,
        (9999 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as ::core::ffi::c_long,
    )
        + ((CanvasHeight
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
            - ((13 as ::core::ffi::c_int) << resolutionFactor) >> resolutionFactor)
            - 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as ::core::ffi::c_long)
        as COORD;
}
#[inline]
unsafe extern "C" fn dispxToUniverse(mut dx: COORD) -> COORD {
    return (((dx as ::core::ffi::c_int
        - ((CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
                } << resolutionFactor)) >> resolutionFactor) - 1 as ::core::ffi::c_int
            >> 1 as ::core::ffi::c_int)) as ::core::ffi::c_long
        * (9999 as ::core::ffi::c_int
            + ((9999 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                / (CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
                + 1 as ::core::ffi::c_int)) as ::core::ffi::c_long >> zoomLevel)
        / (CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
                } << resolutionFactor)) >> resolutionFactor) as ::core::ffi::c_long
        + mapOrigin.x as ::core::ffi::c_long) as COORD;
}
#[inline]
unsafe extern "C" fn dispyToUniverse(mut dy: COORD) -> COORD {
    return (((((CanvasHeight
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
        - ((13 as ::core::ffi::c_int) << resolutionFactor) >> resolutionFactor)
        - 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) - dy as ::core::ffi::c_int)
        as ::core::ffi::c_long
        * (9999 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as ::core::ffi::c_long
        >> zoomLevel)
        / (CanvasHeight
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
            - ((13 as ::core::ffi::c_int) << resolutionFactor) >> resolutionFactor)
            as ::core::ffi::c_long + mapOrigin.y as ::core::ffi::c_long) as COORD;
}
#[inline]
unsafe extern "C" fn universeToDispx2(mut ux: COORD) -> COORD {
    let mut v: ::core::ffi::c_long = signedDivWithError(
        ((ux as ::core::ffi::c_long - mapOrigin.x as ::core::ffi::c_long) << zoomLevel)
            * (CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
                    } << resolutionFactor))) as ::core::ffi::c_long,
        (9999 as ::core::ffi::c_int
            + ((9999 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                / (CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
                        } << resolutionFactor))) + 1 as ::core::ffi::c_int))
            as ::core::ffi::c_long,
    )
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
                } << resolutionFactor)) - 1 as ::core::ffi::c_int
            >> 1 as ::core::ffi::c_int) as ::core::ffi::c_long;
    if v > 32767 as ::core::ffi::c_long {
        return 32767 as COORD;
    }
    if v < -32768 as ::core::ffi::c_long {
        return -32768 as COORD;
    }
    return v as COORD;
}
#[inline]
unsafe extern "C" fn universeToDispy2(mut uy: COORD) -> COORD {
    let mut v: ::core::ffi::c_long = signedDivWithError(
        ((mapOrigin.y as ::core::ffi::c_long - uy as ::core::ffi::c_long) << zoomLevel)
            * (CanvasHeight
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
                as ::core::ffi::c_long,
        (9999 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as ::core::ffi::c_long,
    )
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
            - ((13 as ::core::ffi::c_int) << resolutionFactor) - 1 as ::core::ffi::c_int
            >> 1 as ::core::ffi::c_int) as ::core::ffi::c_long;
    if v > 32767 as ::core::ffi::c_long {
        return 32767 as COORD;
    }
    if v < -32768 as ::core::ffi::c_long {
        return -32768 as COORD;
    }
    return v as COORD;
}
static mut transition_pending: BOOLEAN = BOOLEAN::FALSE;
unsafe extern "C" fn flashCurrentLocation(mut r#where: *mut POINT, mut force: BOOLEAN) {
    static mut redraw: BOOLEAN = BOOLEAN::FALSE;
    static mut c: BYTE = 0 as BYTE;
    static mut val: ::core::ffi::c_int = -2 as ::core::ffi::c_int;
    static mut universe: POINT = POINT { x: 0, y: 0 };
    static mut NextTime: TimeCount = 0 as TimeCount;
    if !r#where.is_null() {
        universe = *r#where;
    }
    if GetTimeCounter() >= NextTime {
        NextTime = GetTimeCounter()
            .wrapping_add(
                (840 as ::core::ffi::c_int / 16 as ::core::ffi::c_int) as TimeCount,
            );
        if c as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            || c as ::core::ffi::c_int == 0x1a as ::core::ffi::c_int
        {
            val = -val;
        }
        c = (c as ::core::ffi::c_int + val) as BYTE;
        redraw = BOOLEAN::TRUE;
    }
    if force.0 != 0 || redraw.0 != 0 {
        let mut OldColor: Color = Color { r: 0, g: 0, b: 0, a: 0 };
        let mut OldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
        let mut s: STAMP = STAMP {
            origin: POINT { x: 0, y: 0 },
            frame: ::core::ptr::null_mut::<FRAME_DESC>(),
        };
        OldContext = SetContext(SpaceContext);
        OldColor = SetContextForeGroundColor(colorFromRgb15(c, c, c));
        s.origin.x = ((universeToDispx(universe.x as ::core::ffi::c_long)
            as ::core::ffi::c_int) << resolutionFactor) as COORD;
        s.origin.y = ((universeToDispy(universe.y as ::core::ffi::c_long)
            as ::core::ffi::c_int) << resolutionFactor) as COORD;
        s.frame = IncFrameIndex(StarMapFrame);
        uqm_lua_checked!(DrawFilledStamp(&raw mut s), ());
        SetContextForeGroundColor(OldColor);
        SetContext(OldContext);
        redraw = BOOLEAN::FALSE;
    }
}
unsafe extern "C" fn DrawCursor(mut curs_x: COORD, mut curs_y: COORD) {
    let mut s: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    s.origin.x = curs_x;
    s.origin.y = curs_y;
    s.frame = StarMapFrame;
    uqm_lua_checked!(DrawStamp(&raw mut s), ());
}
unsafe extern "C" fn DrawMarker(mut dest: POINT, mut r#type: BYTE) {
    let mut s: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    if r#type as ::core::ffi::c_int > 2 as ::core::ffi::c_int {
        return;
    }
    s.origin = MAKE_POINT(
        ((universeToDispx(dest.x as ::core::ffi::c_long) as ::core::ffi::c_int)
            << resolutionFactor) as COORD,
        ((universeToDispy(dest.y as ::core::ffi::c_long) as ::core::ffi::c_int)
            << resolutionFactor) as COORD,
    );
    s.frame = SetAbsFrameIndex(
        MiscDataFrame,
        (106 as ::core::ffi::c_int + r#type as ::core::ffi::c_int) as COUNT,
    );
    uqm_lua_checked!(DrawStamp(&raw mut s), ());
}
unsafe extern "C" fn DrawAutoPilot(mut pDstPt: *mut POINT) {
    let mut dx: SIZE = 0;
    let mut dy: SIZE = 0;
    let mut xincr: SIZE = 0;
    let mut yincr: SIZE = 0;
    let mut xerror: SIZE = 0;
    let mut yerror: SIZE = 0;
    let mut cycle: SIZE = 0;
    let mut delta: SIZE = 0;
    let mut pt: POINT = POINT { x: 0, y: 0 };
    let mut s: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    if inHQSpace().0 == 0 {
        pt = (*CurStarDescPtr).star_pt;
    } else {
        pt.x = logxToUniverse(GlobData.SIS_state.log_x) as COORD;
        pt.y = logyToUniverse(GlobData.SIS_state.log_y) as COORD;
    }
    if if resolutionFactor != 2 as ::core::ffi::c_uint {
        BOOLEAN::FALSE.0 as ::core::ffi::c_int
    } else {
        BOOLEAN::TRUE.0 as ::core::ffi::c_int
    } != 0
    {
        s.frame = DecFrameIndex(stars_in_space);
    }
    pt.x = ((universeToDispx(pt.x as ::core::ffi::c_long) as ::core::ffi::c_int)
        << resolutionFactor) as COORD;
    pt.y = ((universeToDispy(pt.y as ::core::ffi::c_long) as ::core::ffi::c_int)
        << resolutionFactor) as COORD;
    dx = (((universeToDispx((*pDstPt).x as ::core::ffi::c_long) as ::core::ffi::c_int)
        << resolutionFactor) - pt.x as ::core::ffi::c_int) as SIZE;
    if dx as ::core::ffi::c_int >= 0 as ::core::ffi::c_int {
        xincr = 1 as SIZE;
    } else {
        xincr = -1 as SIZE;
        dx = -(dx as ::core::ffi::c_int) as SIZE;
    }
    dx = ((dx as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as SIZE;
    dy = (((universeToDispy((*pDstPt).y as ::core::ffi::c_long) as ::core::ffi::c_int)
        << resolutionFactor) - pt.y as ::core::ffi::c_int) as SIZE;
    if dy as ::core::ffi::c_int >= 0 as ::core::ffi::c_int {
        yincr = 1 as SIZE;
    } else {
        yincr = -1 as SIZE;
        dy = -(dy as ::core::ffi::c_int) as SIZE;
    }
    dy = ((dy as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as SIZE;
    if dx as ::core::ffi::c_int >= dy as ::core::ffi::c_int {
        cycle = dx;
    } else {
        cycle = dy;
    }
    yerror = (cycle as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as SIZE;
    xerror = yerror;
    delta = xerror;
    SetContextForeGroundColor(colorFromRgb15(0x4 as BYTE, 0x4 as BYTE, 0x1f as BYTE));
    delta = (delta as ::core::ffi::c_int & !(1 as ::core::ffi::c_int)) as SIZE;
    loop {
        let c2rust_fresh0 = delta;
        delta -= 1;
        if c2rust_fresh0 == 0 {
            break;
        }
        if if resolutionFactor != 2 as ::core::ffi::c_uint {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } != 0
        {
            if delta as ::core::ffi::c_int % 8 as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
                && delta as ::core::ffi::c_int != 0 as ::core::ffi::c_int
            {
                s.origin.x = pt.x;
                s.origin.y = pt.y;
                uqm_lua_checked!(DrawFilledStamp(&raw mut s), ());
            }
        } else if delta as ::core::ffi::c_int & 1 as ::core::ffi::c_int == 0 {
            uqm_lua_checked!(DrawPoint(&raw mut pt), ());
        }
        xerror = (xerror as ::core::ffi::c_int - dx as ::core::ffi::c_int) as SIZE;
        if xerror as ::core::ffi::c_int <= 0 as ::core::ffi::c_int {
            pt.x = (pt.x as ::core::ffi::c_int + xincr as ::core::ffi::c_int) as COORD;
            xerror = (xerror as ::core::ffi::c_int + cycle as ::core::ffi::c_int)
                as SIZE;
        }
        yerror = (yerror as ::core::ffi::c_int - dy as ::core::ffi::c_int) as SIZE;
        if yerror as ::core::ffi::c_int <= 0 as ::core::ffi::c_int {
            pt.y = (pt.y as ::core::ffi::c_int + yincr as ::core::ffi::c_int) as COORD;
            yerror = (yerror as ::core::ffi::c_int + cycle as ::core::ffi::c_int)
                as SIZE;
        }
    };
}
unsafe extern "C" fn GetSphereRect(
    mut FleetPtr: *mut FLEET_INFO,
    mut pRect: *mut RECT,
    mut pRepairRect: *mut RECT,
) {
    let mut diameter: ::core::ffi::c_long = 0;
    diameter = ((*FleetPtr).known_strength as ::core::ffi::c_int
        * 11 as ::core::ffi::c_int) as ::core::ffi::c_long;
    (*pRect).extent.width = (((universeToDispx(diameter) as ::core::ffi::c_int)
        << resolutionFactor)
        - ((universeToDispx(0 as ::core::ffi::c_long) as ::core::ffi::c_int)
            << resolutionFactor)) as COORD;
    if ((*pRect).extent.width as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
        (*pRect).extent.width = -((*pRect).extent.width as ::core::ffi::c_int) as COORD;
    } else if (*pRect).extent.width as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        (*pRect).extent.width = ((1 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    }
    (*pRect).extent.height = (((universeToDispy(diameter) as ::core::ffi::c_int)
        << resolutionFactor)
        - ((universeToDispy(0 as ::core::ffi::c_long) as ::core::ffi::c_int)
            << resolutionFactor)) as COORD;
    if ((*pRect).extent.height as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
        (*pRect).extent.height = -((*pRect).extent.height as ::core::ffi::c_int)
            as COORD;
    } else if (*pRect).extent.height as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        (*pRect).extent.height = ((1 as ::core::ffi::c_int) << resolutionFactor)
            as COORD;
    }
    (*pRect).corner.x = ((universeToDispx((*FleetPtr).known_loc.x as ::core::ffi::c_long)
        as ::core::ffi::c_int) << resolutionFactor) as COORD;
    (*pRect).corner.y = ((universeToDispy((*FleetPtr).known_loc.y as ::core::ffi::c_long)
        as ::core::ffi::c_int) << resolutionFactor) as COORD;
    (*pRect).corner.x = ((*pRect).corner.x as ::core::ffi::c_int
        - ((*pRect).extent.width as ::core::ffi::c_int >> 1 as ::core::ffi::c_int))
        as COORD;
    (*pRect).corner.y = ((*pRect).corner.y as ::core::ffi::c_int
        - ((*pRect).extent.height as ::core::ffi::c_int >> 1 as ::core::ffi::c_int))
        as COORD;
    let mut t: TEXT = TEXT {
        baseline: POINT { x: 0, y: 0 },
        pStr: ::core::ptr::null::<UNICODE>(),
        align: TEXT_ALIGN::ALIGN_LEFT,
        CharCount: 0,
    };
    let mut locString: STRING = ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>();
    t.baseline.x = ((*pRect).corner.x as ::core::ffi::c_int
        + ((*pRect).extent.width as ::core::ffi::c_int >> 1 as ::core::ffi::c_int))
        as COORD;
    t.baseline.y = ((*pRect).corner.y as ::core::ffi::c_int
        + ((*pRect).extent.height as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
        - ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    t.align = TEXT_ALIGN::ALIGN_CENTER;
    locString = SetAbsStringTableIndex((*FleetPtr).race_strings, 1 as COUNT);
    t.CharCount = GetStringLength(locString);
    t.pStr = GetStringAddress(locString);
    TextRect(&raw mut t, pRepairRect, ::core::ptr::null_mut::<BYTE>());
    if (*pRepairRect).corner.x as ::core::ffi::c_int <= 0 as ::core::ffi::c_int {
        (*pRepairRect).corner.x = ((1 as ::core::ffi::c_int) << resolutionFactor)
            as COORD;
    } else if (*pRepairRect).corner.x as ::core::ffi::c_int
        + (*pRepairRect).extent.width as ::core::ffi::c_int
        >= CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
    {
        (*pRepairRect).corner.x = (CanvasWidth
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
                } << resolutionFactor))
            - (*pRepairRect).extent.width as ::core::ffi::c_int
            - ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    }
    if (*pRepairRect).corner.y as ::core::ffi::c_int <= 0 as ::core::ffi::c_int {
        (*pRepairRect).corner.y = ((1 as ::core::ffi::c_int) << resolutionFactor)
            as COORD;
    } else if (*pRepairRect).corner.y as ::core::ffi::c_int
        + (*pRepairRect).extent.height as ::core::ffi::c_int
        >= CanvasHeight
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
        (*pRepairRect).corner.y = (CanvasHeight
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
            - (*pRepairRect).extent.height as ::core::ffi::c_int
            - ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    }
    BoxUnion(pRepairRect, pRect, pRepairRect);
    (*pRepairRect).extent.width = ((*pRepairRect).extent.width as ::core::ffi::c_int
        + ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    (*pRepairRect).extent.height = ((*pRepairRect).extent.height as ::core::ffi::c_int
        + ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
}
unsafe extern "C" fn GetWarEraSphereRect(
    mut FleetPtr: *mut FLEET_INFO,
    mut pRect: *mut RECT,
    mut pRepairRect: *mut RECT,
) {
    let mut diameter: ::core::ffi::c_long = (WarEraStrength((*FleetPtr).SpeciesID)
        as ::core::ffi::c_int * 11 as ::core::ffi::c_int) as ::core::ffi::c_long;
    let mut loc: POINT = SeedFleetLocation(
        FleetPtr,
        &raw mut plot_map as *mut PLOT_LOCATION,
        C2Rust_Unnamed_5::WAR_ERA.0 as ::core::ffi::c_int as COUNT,
    );
    (*pRect).extent.width = (((universeToDispx(diameter) as ::core::ffi::c_int)
        << resolutionFactor)
        - ((universeToDispx(0 as ::core::ffi::c_long) as ::core::ffi::c_int)
            << resolutionFactor)) as COORD;
    if ((*pRect).extent.width as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
        (*pRect).extent.width = -((*pRect).extent.width as ::core::ffi::c_int) as COORD;
    } else if (*pRect).extent.width as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        (*pRect).extent.width = ((1 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    }
    (*pRect).extent.height = (((universeToDispy(diameter) as ::core::ffi::c_int)
        << resolutionFactor)
        - ((universeToDispy(0 as ::core::ffi::c_long) as ::core::ffi::c_int)
            << resolutionFactor)) as COORD;
    if ((*pRect).extent.height as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
        (*pRect).extent.height = -((*pRect).extent.height as ::core::ffi::c_int)
            as COORD;
    } else if (*pRect).extent.height as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        (*pRect).extent.height = ((1 as ::core::ffi::c_int) << resolutionFactor)
            as COORD;
    }
    (*pRect).corner.x = ((universeToDispx(loc.x as ::core::ffi::c_long)
        as ::core::ffi::c_int) << resolutionFactor) as COORD;
    (*pRect).corner.y = ((universeToDispy(loc.y as ::core::ffi::c_long)
        as ::core::ffi::c_int) << resolutionFactor) as COORD;
    (*pRect).corner.x = ((*pRect).corner.x as ::core::ffi::c_int
        - ((*pRect).extent.width as ::core::ffi::c_int >> 1 as ::core::ffi::c_int))
        as COORD;
    (*pRect).corner.y = ((*pRect).corner.y as ::core::ffi::c_int
        - ((*pRect).extent.height as ::core::ffi::c_int >> 1 as ::core::ffi::c_int))
        as COORD;
    if (*pRepairRect).corner.x as ::core::ffi::c_int <= 0 as ::core::ffi::c_int {
        (*pRepairRect).corner.x = ((1 as ::core::ffi::c_int) << resolutionFactor)
            as COORD;
    } else if (*pRepairRect).corner.x as ::core::ffi::c_int
        + (*pRepairRect).extent.width as ::core::ffi::c_int
        >= CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
    {
        (*pRepairRect).corner.x = (CanvasWidth
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
                } << resolutionFactor))
            - (*pRepairRect).extent.width as ::core::ffi::c_int
            - ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    }
    if (*pRepairRect).corner.y as ::core::ffi::c_int <= 0 as ::core::ffi::c_int {
        (*pRepairRect).corner.y = ((1 as ::core::ffi::c_int) << resolutionFactor)
            as COORD;
    } else if (*pRepairRect).corner.y as ::core::ffi::c_int
        + (*pRepairRect).extent.height as ::core::ffi::c_int
        >= CanvasHeight
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
        (*pRepairRect).corner.y = (CanvasHeight
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
            - (*pRepairRect).extent.height as ::core::ffi::c_int
            - ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    }
    BoxUnion(pRepairRect, pRect, pRepairRect);
    (*pRepairRect).extent.width = ((*pRepairRect).extent.width as ::core::ffi::c_int
        + ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    (*pRepairRect).extent.height = ((*pRepairRect).extent.height as ::core::ffi::c_int
        + ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
}
unsafe extern "C" fn FuelRequiredTo(mut dest: POINT) -> ::core::ffi::c_uint {
    let mut fuel_required: COUNT = 0;
    let mut f: DWORD = 0;
    let mut pt: POINT = POINT { x: 0, y: 0 };
    if inHQSpace().0 == 0 {
        pt = (*CurStarDescPtr).star_pt;
    } else {
        pt.x = logxToUniverse(GlobData.SIS_state.log_x) as COORD;
        pt.y = logyToUniverse(GlobData.SIS_state.log_y) as COORD;
    }
    pt.x = (pt.x as ::core::ffi::c_int - dest.x as ::core::ffi::c_int) as COORD;
    pt.y = (pt.y as ::core::ffi::c_int - dest.y as ::core::ffi::c_int) as COORD;
    f = (pt.x as ::core::ffi::c_long * pt.x as ::core::ffi::c_long
        + pt.y as ::core::ffi::c_long * pt.y as ::core::ffi::c_long) as DWORD;
    if f == 0 as DWORD
        || uqm_lua_checked!(getGameStateUint(
            b"ARILOU_SPACE_SIDE\0".as_ptr() as *const ::core::ffi::c_char,
        ), 0) > 1 as DWORD
    {
        fuel_required = 0 as COUNT;
    } else {
        fuel_required = (square_root(f) as ::core::ffi::c_int
            + 100 as ::core::ffi::c_int / 20 as ::core::ffi::c_int) as COUNT;
    }
    return fuel_required as ::core::ffi::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn GetPointOfEllipse(
    mut a: ::core::ffi::c_double,
    mut b: ::core::ffi::c_double,
    mut radian: ::core::ffi::c_double,
) -> POINT {
    let mut t: [::core::ffi::c_double; 2] = [a * cos(radian), b * sin(radian)];
    return point {
        x: (t[0usize]
            + (if (t[0usize] + 0.5f64) as ::core::ffi::c_int as ::core::ffi::c_double
                > t[0usize]
            {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as ::core::ffi::c_double) as COORD,
        y: (t[1usize]
            + (if (t[1usize] + 0.5f64) as ::core::ffi::c_int as ::core::ffi::c_double
                > t[1usize]
            {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as ::core::ffi::c_double) as COORD,
    };
}
#[no_mangle]
pub unsafe extern "C" fn ShiftPoint(mut p: POINT, mut S: POINT) -> POINT {
    return point {
        x: (p.x as ::core::ffi::c_int + S.x as ::core::ffi::c_int) as COORD,
        y: (p.y as ::core::ffi::c_int + S.y as ::core::ffi::c_int) as COORD,
    };
}
#[no_mangle]
pub unsafe extern "C" fn RotatePoint(
    mut p: POINT,
    mut Pivot: POINT,
    mut radian: ::core::ffi::c_double,
) -> POINT {
    let mut d: [::core::ffi::c_double; 2] = [
        (p.x as ::core::ffi::c_int - Pivot.x as ::core::ffi::c_int)
            as ::core::ffi::c_double,
        (p.y as ::core::ffi::c_int - Pivot.y as ::core::ffi::c_int)
            as ::core::ffi::c_double,
    ];
    let mut cosine: [::core::ffi::c_double; 2] = [cos(radian), sin(radian)];
    let mut x: ::core::ffi::c_double = Pivot.x as ::core::ffi::c_int
        as ::core::ffi::c_double
        + (d[0usize] * cosine[0usize] - d[1usize] * cosine[1usize]);
    let mut y: ::core::ffi::c_double = Pivot.y as ::core::ffi::c_int
        as ::core::ffi::c_double
        + (d[0usize] * cosine[1usize] + d[1usize] * cosine[0usize]);
    return point {
        x: (x
            + (if (x + 0.5f64) as ::core::ffi::c_int as ::core::ffi::c_double > x {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as ::core::ffi::c_double) as COORD,
        y: (y
            + (if (y + 0.5f64) as ::core::ffi::c_int as ::core::ffi::c_double > y {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as ::core::ffi::c_double) as COORD,
    };
}
#[no_mangle]
pub unsafe extern "C" fn CalcEllipsePoint(
    mut a: ::core::ffi::c_double,
    mut b: ::core::ffi::c_double,
    mut rad_one: ::core::ffi::c_double,
    mut rad_two: ::core::ffi::c_double,
    mut Pivot: POINT,
) -> POINT {
    let mut q: [::core::ffi::c_double; 2] = [a * cos(rad_one), b * sin(rad_one)];
    let mut w: [::core::ffi::c_double; 2] = [
        q[0usize]
            + (if (q[0usize] + 0.5f64) as ::core::ffi::c_int as ::core::ffi::c_double
                > q[0usize]
            {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as ::core::ffi::c_double
            + Pivot.x as ::core::ffi::c_int as ::core::ffi::c_double,
        q[1usize]
            + (if (q[1usize] + 0.5f64) as ::core::ffi::c_int as ::core::ffi::c_double
                > q[1usize]
            {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as ::core::ffi::c_double
            + Pivot.y as ::core::ffi::c_int as ::core::ffi::c_double,
    ];
    let mut d: [::core::ffi::c_double; 2] = [
        w[0usize] - Pivot.x as ::core::ffi::c_int as ::core::ffi::c_double,
        w[1usize] - Pivot.y as ::core::ffi::c_int as ::core::ffi::c_double,
    ];
    let mut cosine: [::core::ffi::c_double; 2] = [cos(rad_two), sin(rad_two)];
    let mut x: ::core::ffi::c_double = Pivot.x as ::core::ffi::c_int
        as ::core::ffi::c_double
        + (d[0usize] * cosine[0usize] - d[1usize] * cosine[1usize]);
    let mut y: ::core::ffi::c_double = Pivot.y as ::core::ffi::c_int
        as ::core::ffi::c_double
        + (d[0usize] * cosine[1usize] + d[1usize] * cosine[0usize]);
    x = x
        + (if (x + 0.5f64) as ::core::ffi::c_int as ::core::ffi::c_double > x {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as ::core::ffi::c_double;
    if x > 32767.0f64 {
        x = 32767.0f64;
    }
    if x < -32768.0f64 {
        x = -32768.0f64;
    }
    y = y
        + (if (y + 0.5f64) as ::core::ffi::c_int as ::core::ffi::c_double > y {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as ::core::ffi::c_double;
    if y > 32767.0f64 {
        y = 32767.0f64;
    }
    if y < -32768.0f64 {
        y = -32768.0f64;
    }
    return point {
        x: (if if resolutionFactor != 2 as ::core::ffi::c_uint {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } != 0
        {
            universeToDispx2(x as COORD) as ::core::ffi::c_int
        } else {
            universeToDispx(x as ::core::ffi::c_long) as ::core::ffi::c_int
        }) as COORD,
        y: (if if resolutionFactor != 2 as ::core::ffi::c_uint {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } != 0
        {
            universeToDispy2(y as COORD) as ::core::ffi::c_int
        } else {
            universeToDispy(y as ::core::ffi::c_long) as ::core::ffi::c_int
        }) as COORD,
    };
}
#[no_mangle]
pub unsafe extern "C" fn onScreen(
    mut l: *mut LINE,
    mut ignoreX: BOOLEAN,
    mut ignoreY: BOOLEAN,
) -> BOOLEAN {
    return BOOLEAN(
        !(((*l).first.x as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
            && ((*l).second.x as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
            && ignoreX.0 == 0
            || (*l).first.x as ::core::ffi::c_int
                >= CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
                && (*l).second.x as ::core::ffi::c_int
                    >= CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
                            } << resolutionFactor)) && ignoreX.0 == 0
            || ((*l).first.y as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
                && ((*l).second.y as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
                && ignoreY.0 == 0
            || (*l).first.y as ::core::ffi::c_int
                >= CanvasHeight
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
                && (*l).second.y as ::core::ffi::c_int
                    >= CanvasHeight
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
                && ignoreY.0 == 0) as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
}
unsafe extern "C" fn DrawNoReturnZone() {
    let mut dist: ::core::ffi::c_double = 0.;
    let mut sol: POINT = POINT { x: 0, y: 0 };
    let mut sis: POINT = POINT { x: 0, y: 0 };
    let mut halfFuel: ::core::ffi::c_double = GlobData
        .SIS_state
        .FuelOnBoard
        .wrapping_div(2 as DWORD) as ::core::ffi::c_double;
    sol = plot_map[C2Rust_Unnamed_5::SOL_DEFINED.0 as ::core::ffi::c_int as usize]
        .star_pt;
    sis = point {
        x: logxToUniverse(GlobData.SIS_state.log_x) as COORD,
        y: logyToUniverse(GlobData.SIS_state.log_y) as COORD,
    };
    dist = uqm_lua_checked!(FuelRequiredTo(sol), ()) as ::core::ffi::c_double
        / 2 as ::core::ffi::c_int as ::core::ffi::c_double;
    if dist <= halfFuel {
        let mut curr: POINT = POINT { x: 0, y: 0 };
        let mut center: POINT = POINT { x: 0, y: 0 };
        let mut rmax_y: POINT = POINT { x: 0, y: 0 };
        let mut rmin_y: POINT = POINT { x: 0, y: 0 };
        let mut i: ::core::ffi::c_double = 0.;
        let mut Step: ::core::ffi::c_double = 0.;
        let mut ry: ::core::ffi::c_double = 0.;
        let mut rotation: ::core::ffi::c_double = 0.;
        ry = sqrt(
            pow(halfFuel, 2 as ::core::ffi::c_int as ::core::ffi::c_double)
                - pow(dist, 2 as ::core::ffi::c_int as ::core::ffi::c_double),
        );
        Step = 3.14159265358979323846f64
            / (180.0f32
                / (if zoomLevel > 2 as ::core::ffi::c_int {
                    2 as ::core::ffi::c_int
                } else {
                    1 as ::core::ffi::c_int
                }) as ::core::ffi::c_float) as ::core::ffi::c_double;
        center = MAKE_POINT(
            ((sis.x as ::core::ffi::c_int + sol.x as ::core::ffi::c_int)
                / 2 as ::core::ffi::c_int) as COORD,
            ((sis.y as ::core::ffi::c_int + sol.y as ::core::ffi::c_int)
                / 2 as ::core::ffi::c_int) as COORD,
        );
        rotation = atan2(
            (sol.y as ::core::ffi::c_int - sis.y as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            (sol.x as ::core::ffi::c_int - sis.x as ::core::ffi::c_int)
                as ::core::ffi::c_double,
        );
        rmax_y = point {
            x: -1 as COORD,
            y: -1 as COORD,
        };
        rmin_y = point {
            x: (CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
            y: (CanvasHeight
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
                - ((13 as ::core::ffi::c_int) << resolutionFactor)) as COORD,
        };
        i = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
        while i
            < 3.14159265358979323846f64
                * 2 as ::core::ffi::c_int as ::core::ffi::c_double
        {
            curr = CalcEllipsePoint(halfFuel, ry, i, rotation, center);
            if curr.y as ::core::ffi::c_int > rmax_y.y as ::core::ffi::c_int {
                rmax_y = curr;
            }
            if (curr.y as ::core::ffi::c_int) < rmin_y.y as ::core::ffi::c_int {
                rmin_y = curr;
            }
            i += Step;
        }
        if rmax_y.y as ::core::ffi::c_int >= 0 as ::core::ffi::c_int
            || (rmin_y.y as ::core::ffi::c_int)
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
            let mut L: LINE = LINE {
                first: POINT { x: 0, y: 0 },
                second: POINT { x: 0, y: 0 },
            };
            let mut tempLine: LINE = LINE {
                first: POINT { x: 0, y: 0 },
                second: POINT { x: 0, y: 0 },
            };
            let mut prev: POINT = CalcEllipsePoint(
                halfFuel,
                ry,
                i - Step,
                rotation,
                center,
            );
            let mut dy: COORD = 0;
            let mut err: ::core::ffi::c_double = (rmax_y.x as ::core::ffi::c_double
                - rmin_y.x as ::core::ffi::c_double)
                / (rmax_y.y as ::core::ffi::c_double
                    - rmin_y.y as ::core::ffi::c_double);
            i = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            while i
                < 3.14159265358979323846f64
                    * 2 as ::core::ffi::c_int as ::core::ffi::c_double
            {
                L.first = CalcEllipsePoint(halfFuel, ry, i, rotation, center);
                L.second.x = (rmax_y.x as ::core::ffi::c_int
                    - (err
                        * (rmax_y.y as ::core::ffi::c_int
                            - L.first.y as ::core::ffi::c_int) as ::core::ffi::c_double)
                        as COORD as ::core::ffi::c_int) as COORD;
                L.second.y = L.first.y;
                if onScreen(&raw mut L, BOOLEAN::FALSE, BOOLEAN::FALSE).0 != 0 {
                    uqm_lua_checked!(DrawLine(&raw mut L, 1 as BYTE), ());
                }
                dy = (L.first.y as ::core::ffi::c_int - prev.y as ::core::ffi::c_int)
                    as COORD;
                MAKE_LINE(&raw mut tempLine, L.first, prev);
                if abs(dy as ::core::ffi::c_int) > 1 as ::core::ffi::c_int
                    && onScreen(&raw mut tempLine, BOOLEAN::TRUE, BOOLEAN::FALSE).0 != 0
                {
                    let mut L2: LINE = LINE {
                        first: POINT { x: 0, y: 0 },
                        second: POINT { x: 0, y: 0 },
                    };
                    let mut iter: COORD = 0;
                    let mut y_err: ::core::ffi::c_double = (L.first.x
                        as ::core::ffi::c_double - prev.x as ::core::ffi::c_double)
                        / (L.first.y as ::core::ffi::c_double
                            - prev.y as ::core::ffi::c_double);
                    if (dy as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                        iter = -1 as COORD;
                    } else {
                        iter = 1 as COORD;
                    }
                    while abs(dy as ::core::ffi::c_int) > 1 as ::core::ffi::c_int {
                        L2.second.y = (prev.y as ::core::ffi::c_int
                            + dy as ::core::ffi::c_int - iter as ::core::ffi::c_int)
                            as COORD;
                        L2.first.y = L2.second.y;
                        L2.first.x = (L.first.x as ::core::ffi::c_int
                            - (y_err
                                * (L.first.y as ::core::ffi::c_int
                                    - L2.first.y as ::core::ffi::c_int)
                                    as ::core::ffi::c_double) as COORD as ::core::ffi::c_int)
                            as COORD;
                        L2.second.x = (rmax_y.x as ::core::ffi::c_int
                            - (err
                                * (rmax_y.y as ::core::ffi::c_int
                                    - L2.first.y as ::core::ffi::c_int)
                                    as ::core::ffi::c_double) as COORD as ::core::ffi::c_int)
                            as COORD;
                        if onScreen(&raw mut L2, BOOLEAN::FALSE, BOOLEAN::FALSE).0 != 0 {
                            uqm_lua_checked!(DrawLine(&raw mut L2, 1 as BYTE), ());
                        }
                        dy = (dy as ::core::ffi::c_int - iter as ::core::ffi::c_int)
                            as COORD;
                    }
                }
                prev = L.first;
                i += Step;
            }
        }
    }
}
unsafe extern "C" fn GetFuelRect(
    mut r: *mut DRECT,
    mut diameter: SDWORD,
    mut corner: POINT,
) {
    let mut x: SDWORD = 0;
    let mut y: SDWORD = 0;
    let mut width: SDWORD = 0;
    let mut height: SDWORD = 0;
    if diameter < 0 as ::core::ffi::c_int {
        diameter = 0 as ::core::ffi::c_int as SDWORD;
    }
    if diameter > 9999 as ::core::ffi::c_int * 4 as ::core::ffi::c_int {
        diameter = (9999 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as SDWORD;
    }
    width = (((signedDivWithError(
        (((diameter - mapOrigin.x as ::core::ffi::c_int) << zoomLevel)
            * (CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
                    } << resolutionFactor)) >> resolutionFactor)) as ::core::ffi::c_long,
        (9999 as ::core::ffi::c_int
            + ((9999 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                / (CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
                + 1 as ::core::ffi::c_int)) as ::core::ffi::c_long,
    )
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
                } << resolutionFactor)) >> resolutionFactor) - 1 as ::core::ffi::c_int
            >> 1 as ::core::ffi::c_int) as ::core::ffi::c_long) << resolutionFactor)
        - ((universeToDispx(0 as ::core::ffi::c_long) as ::core::ffi::c_int)
            << resolutionFactor) as ::core::ffi::c_long) as SDWORD;
    if width < 0 as ::core::ffi::c_int {
        width = -width;
    }
    height = (((signedDivWithError(
        ((mapOrigin.y as ::core::ffi::c_int - diameter << zoomLevel)
            * (CanvasHeight
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
                - ((13 as ::core::ffi::c_int) << resolutionFactor) >> resolutionFactor))
            as ::core::ffi::c_long,
        (9999 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as ::core::ffi::c_long,
    )
        + ((CanvasHeight
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
            - ((13 as ::core::ffi::c_int) << resolutionFactor) >> resolutionFactor)
            - 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as ::core::ffi::c_long)
        << resolutionFactor)
        - ((universeToDispy(0 as ::core::ffi::c_long) as ::core::ffi::c_int)
            << resolutionFactor) as ::core::ffi::c_long) as SDWORD;
    if height < 0 as ::core::ffi::c_int {
        height = -height;
    }
    x = ((universeToDispx(corner.x as ::core::ffi::c_long) as SDWORD)
        << resolutionFactor) - (width >> 1 as ::core::ffi::c_int);
    y = ((universeToDispy(corner.y as ::core::ffi::c_long) as SDWORD)
        << resolutionFactor) - (height >> 1 as ::core::ffi::c_int);
    (*r).corner.x = x;
    (*r).corner.y = y;
    (*r).extent.width = width;
    (*r).extent.height = height;
}
unsafe extern "C" fn DrawFuelCircle(mut secondary: BOOLEAN) {
    let mut r: DRECT = DRECT {
        corner: DPOINT { x: 0, y: 0 },
        extent: DEXTENT { width: 0, height: 0 },
    };
    let mut corner: POINT = POINT { x: 0, y: 0 };
    let mut OldColor: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    let mut OnBoardFuel: DWORD = GlobData.SIS_state.FuelOnBoard;
    if secondary.0 != 0 {
        OnBoardFuel = OnBoardFuel
            .wrapping_sub(uqm_lua_checked!(FuelRequiredTo(GlobData.Game_state.autopilot), ()));
        corner = GlobData.Game_state.autopilot;
    } else if inHQSpace().0 == 0 {
        corner = (*CurStarDescPtr).star_pt;
    } else {
        corner.x = logxToUniverse(GlobData.SIS_state.log_x) as COORD;
        corner.y = logyToUniverse(GlobData.SIS_state.log_y) as COORD;
    }
    GetFuelRect(&raw mut r, (OnBoardFuel << 1 as ::core::ffi::c_int) as SDWORD, corner);
    if secondary.0 != 0 {
        OldColor = SetContextForeGroundColor(
            colorFromRgb15(0xa as BYTE, 0xa as BYTE, 0xa as BYTE),
        );
        uqm_lua_checked!(DrawOval(
            &raw mut r,
            (if if resolutionFactor != 2 as ::core::ffi::c_uint {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } == 0
            {
                1 as ::core::ffi::c_int
            } else {
                6 as ::core::ffi::c_int
            }) as BYTE,
            BOOLEAN::FALSE,
        ), ());
        SetContextForeGroundColor(OldColor);
    } else {
        OldColor = SetContextForeGroundColor(
            colorFromRgb15(0x4 as BYTE, 0x4 as BYTE, 0x4 as BYTE),
        );
        uqm_lua_checked!(DrawFilledOval(&raw mut r), ());
        SetContextForeGroundColor(OldColor);
        if optFuelRange > 1 as ::core::ffi::c_int {
            OldColor = SetContextForeGroundColor(
                colorFromRgb15(0x5 as BYTE, 0x5 as BYTE, 0x5 as BYTE),
            );
            if pointsEqual(
                corner,
                plot_map[C2Rust_Unnamed_5::SOL_DEFINED.0 as ::core::ffi::c_int as usize]
                    .star_pt,
            ) {
                GetFuelRect(&raw mut r, OnBoardFuel as SDWORD, corner);
                uqm_lua_checked!(DrawFilledOval(&raw mut r), ());
            } else {
                uqm_lua_checked!(DrawNoReturnZone(), ());
            }
            SetContextForeGroundColor(OldColor);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn isHomeworld(mut Index: BYTE) -> BOOLEAN {
    let mut raceBool: BOOLEAN = BOOLEAN::FALSE;
    match Index as ::core::ffi::c_int {
        17 => {
            if uqm_lua_checked!(IsHomeworldKnown(
                    C2Rust_Unnamed_8::CHMMR_HOME.0 as ::core::ffi::c_int as DWORD,
                ), BOOLEAN(0))
                .0 != 0
                && (CheckAlliance(RACE_ID::CHMMR_SHIP) as ::core::ffi::c_int
                    != C2Rust_Unnamed_7::DEAD_GUY.0 as ::core::ffi::c_int
                    || optSeedType
                        > C2Rust_Unnamed_0::OPTVAL_PLANET.0 as ::core::ffi::c_int)
            {
                raceBool = BOOLEAN::TRUE;
            }
        }
        40 => {
            if uqm_lua_checked!(IsHomeworldKnown(
                    C2Rust_Unnamed_8::ORZ_HOME.0 as ::core::ffi::c_int as DWORD,
                ), BOOLEAN(0))
                .0 != 0
                && (CheckAlliance(RACE_ID::ORZ_SHIP) as ::core::ffi::c_int
                    != C2Rust_Unnamed_7::DEAD_GUY.0 as ::core::ffi::c_int
                    || optSeedType
                        > C2Rust_Unnamed_0::OPTVAL_PLANET.0 as ::core::ffi::c_int)
            {
                raceBool = BOOLEAN::TRUE;
            }
        }
        35 => {
            if uqm_lua_checked!(IsHomeworldKnown(
                    C2Rust_Unnamed_8::PKUNK_HOME.0 as ::core::ffi::c_int as DWORD,
                ), BOOLEAN(0))
                .0 != 0
                && (CheckAlliance(RACE_ID::PKUNK_SHIP) as ::core::ffi::c_int
                    != C2Rust_Unnamed_7::DEAD_GUY.0 as ::core::ffi::c_int
                    || optSeedType
                        > C2Rust_Unnamed_0::OPTVAL_PLANET.0 as ::core::ffi::c_int)
            {
                raceBool = BOOLEAN::TRUE;
            }
        }
        2 => {
            if uqm_lua_checked!(IsHomeworldKnown(
                    C2Rust_Unnamed_8::SHOFIXTI_HOME.0 as ::core::ffi::c_int as DWORD,
                ), BOOLEAN(0))
                .0 != 0
                && (CheckAlliance(RACE_ID::SHOFIXTI_SHIP) as ::core::ffi::c_int
                    != C2Rust_Unnamed_7::DEAD_GUY.0 as ::core::ffi::c_int
                    || optSeedType
                        > C2Rust_Unnamed_0::OPTVAL_PLANET.0 as ::core::ffi::c_int)
            {
                raceBool = BOOLEAN::TRUE;
            }
        }
        5 => {
            if uqm_lua_checked!(IsHomeworldKnown(
                    C2Rust_Unnamed_8::SPATHI_HOME.0 as ::core::ffi::c_int as DWORD,
                ), BOOLEAN(0))
                .0 != 0
                && (CheckAlliance(RACE_ID::SPATHI_SHIP) as ::core::ffi::c_int
                    != C2Rust_Unnamed_7::DEAD_GUY.0 as ::core::ffi::c_int
                    || optSeedType
                        > C2Rust_Unnamed_0::OPTVAL_PLANET.0 as ::core::ffi::c_int)
            {
                raceBool = BOOLEAN::TRUE;
            }
        }
        37 => {
            if uqm_lua_checked!(IsHomeworldKnown(
                    C2Rust_Unnamed_8::SUPOX_HOME.0 as ::core::ffi::c_int as DWORD,
                ), BOOLEAN(0))
                .0 != 0
                && (CheckAlliance(RACE_ID::SUPOX_SHIP) as ::core::ffi::c_int
                    != C2Rust_Unnamed_7::DEAD_GUY.0 as ::core::ffi::c_int
                    || optSeedType
                        > C2Rust_Unnamed_0::OPTVAL_PLANET.0 as ::core::ffi::c_int)
            {
                raceBool = BOOLEAN::TRUE;
            }
        }
        41 => {
            if uqm_lua_checked!(IsHomeworldKnown(
                    C2Rust_Unnamed_8::THRADDASH_HOME.0 as ::core::ffi::c_int as DWORD,
                ), BOOLEAN(0))
                .0 != 0
                && (CheckAlliance(RACE_ID::THRADDASH_SHIP) as ::core::ffi::c_int
                    != C2Rust_Unnamed_7::DEAD_GUY.0 as ::core::ffi::c_int
                    || optSeedType
                        > C2Rust_Unnamed_0::OPTVAL_PLANET.0 as ::core::ffi::c_int)
            {
                raceBool = BOOLEAN::TRUE;
            }
        }
        36 => {
            if uqm_lua_checked!(IsHomeworldKnown(
                    C2Rust_Unnamed_8::UTWIG_HOME.0 as ::core::ffi::c_int as DWORD,
                ), BOOLEAN(0))
                .0 != 0
                && (CheckAlliance(RACE_ID::UTWIG_SHIP) as ::core::ffi::c_int
                    != C2Rust_Unnamed_7::DEAD_GUY.0 as ::core::ffi::c_int
                    || optSeedType
                        > C2Rust_Unnamed_0::OPTVAL_PLANET.0 as ::core::ffi::c_int)
            {
                raceBool = BOOLEAN::TRUE;
            }
        }
        39 => {
            if uqm_lua_checked!(IsHomeworldKnown(
                    C2Rust_Unnamed_8::VUX_HOME.0 as ::core::ffi::c_int as DWORD,
                ), BOOLEAN(0))
                .0 != 0
                && (CheckAlliance(RACE_ID::VUX_SHIP) as ::core::ffi::c_int
                    != C2Rust_Unnamed_7::DEAD_GUY.0 as ::core::ffi::c_int
                    || optSeedType
                        > C2Rust_Unnamed_0::OPTVAL_PLANET.0 as ::core::ffi::c_int)
            {
                raceBool = BOOLEAN::TRUE;
            }
        }
        38 => {
            if uqm_lua_checked!(IsHomeworldKnown(
                    C2Rust_Unnamed_8::YEHAT_HOME.0 as ::core::ffi::c_int as DWORD,
                ), BOOLEAN(0))
                .0 != 0
                && (CheckAlliance(RACE_ID::YEHAT_SHIP) as ::core::ffi::c_int
                    != C2Rust_Unnamed_7::DEAD_GUY.0 as ::core::ffi::c_int
                    || optSeedType
                        > C2Rust_Unnamed_0::OPTVAL_PLANET.0 as ::core::ffi::c_int)
            {
                raceBool = BOOLEAN::TRUE;
            }
        }
        21 => {
            if uqm_lua_checked!(IsHomeworldKnown(
                    C2Rust_Unnamed_8::DRUUGE_HOME.0 as ::core::ffi::c_int as DWORD,
                ), BOOLEAN(0))
                .0 != 0
                && (CheckAlliance(RACE_ID::DRUUGE_SHIP) as ::core::ffi::c_int
                    != C2Rust_Unnamed_7::DEAD_GUY.0 as ::core::ffi::c_int
                    || optSeedType
                        > C2Rust_Unnamed_0::OPTVAL_PLANET.0 as ::core::ffi::c_int)
            {
                raceBool = BOOLEAN::TRUE;
            }
        }
        52 => {
            if uqm_lua_checked!(IsHomeworldKnown(
                    C2Rust_Unnamed_8::ILWRATH_HOME.0 as ::core::ffi::c_int as DWORD,
                ), BOOLEAN(0))
                .0 != 0
                && (CheckAlliance(RACE_ID::ILWRATH_SHIP) as ::core::ffi::c_int
                    != C2Rust_Unnamed_7::DEAD_GUY.0 as ::core::ffi::c_int
                    || optSeedType
                        > C2Rust_Unnamed_0::OPTVAL_PLANET.0 as ::core::ffi::c_int)
            {
                raceBool = BOOLEAN::TRUE;
            }
        }
        31 => {
            if uqm_lua_checked!(IsHomeworldKnown(
                    C2Rust_Unnamed_8::MYCON_HOME.0 as ::core::ffi::c_int as DWORD,
                ), BOOLEAN(0))
                .0 != 0
                && (CheckAlliance(RACE_ID::MYCON_SHIP) as ::core::ffi::c_int
                    != C2Rust_Unnamed_7::DEAD_GUY.0 as ::core::ffi::c_int
                    || optSeedType
                        > C2Rust_Unnamed_0::OPTVAL_PLANET.0 as ::core::ffi::c_int)
            {
                raceBool = BOOLEAN::TRUE;
            }
        }
        20 => {
            if uqm_lua_checked!(IsHomeworldKnown(
                    C2Rust_Unnamed_8::SLYLANDRO_HOME.0 as ::core::ffi::c_int as DWORD,
                ), BOOLEAN(0))
                .0 != 0
            {
                raceBool = BOOLEAN::TRUE;
            }
        }
        16 => {
            if uqm_lua_checked!(IsHomeworldKnown(
                    C2Rust_Unnamed_8::UMGAH_HOME.0 as ::core::ffi::c_int as DWORD,
                ), BOOLEAN(0))
                .0 != 0
                && (CheckAlliance(RACE_ID::UMGAH_SHIP) as ::core::ffi::c_int
                    != C2Rust_Unnamed_7::DEAD_GUY.0 as ::core::ffi::c_int
                    || optSeedType
                        > C2Rust_Unnamed_0::OPTVAL_PLANET.0 as ::core::ffi::c_int)
            {
                raceBool = BOOLEAN::TRUE;
            }
        }
        6 => {
            if uqm_lua_checked!(IsHomeworldKnown(
                    C2Rust_Unnamed_8::ZOQFOTPIK_HOME.0 as ::core::ffi::c_int as DWORD,
                ), BOOLEAN(0))
                .0 != 0
                && (CheckAlliance(RACE_ID::ZOQFOTPIK_SHIP) as ::core::ffi::c_int
                    != C2Rust_Unnamed_7::DEAD_GUY.0 as ::core::ffi::c_int
                    || optSeedType
                        > C2Rust_Unnamed_0::OPTVAL_PLANET.0 as ::core::ffi::c_int)
            {
                raceBool = BOOLEAN::TRUE;
            }
        }
        18 => {
            if uqm_lua_checked!(IsHomeworldKnown(
                    C2Rust_Unnamed_8::SYREEN_HOME.0 as ::core::ffi::c_int as DWORD,
                ), BOOLEAN(0))
                .0 != 0
                && (CheckAlliance(RACE_ID::SYREEN_SHIP) as ::core::ffi::c_int
                    != C2Rust_Unnamed_7::DEAD_GUY.0 as ::core::ffi::c_int
                    || optSeedType
                        > C2Rust_Unnamed_0::OPTVAL_PLANET.0 as ::core::ffi::c_int)
            {
                raceBool = BOOLEAN::TRUE;
            }
        }
        53 => {
            if uqm_lua_checked!(IsHomeworldKnown(
                    C2Rust_Unnamed_8::ANDROSYNTH_HOME.0 as ::core::ffi::c_int as DWORD,
                ), BOOLEAN(0))
                .0 != 0
                && (CheckAlliance(RACE_ID::ANDROSYNTH_SHIP) as ::core::ffi::c_int
                    != C2Rust_Unnamed_7::DEAD_GUY.0 as ::core::ffi::c_int
                    || optSeedType
                        > C2Rust_Unnamed_0::OPTVAL_PLANET.0 as ::core::ffi::c_int)
            {
                raceBool = BOOLEAN::TRUE;
            }
        }
        1 | 4 => {
            raceBool = BOOLEAN::TRUE;
        }
        _ => {}
    }
    return raceBool;
}
#[no_mangle]
pub unsafe extern "C" fn markerBuf(
    star_index: ::core::ffi::c_int,
    mut marker_state: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    static mut buf: [::core::ffi::c_char; 255] = [0; 255];
    snprintf(
        &raw mut buf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 255]>(),
        b"SYS_%s_%02u\0".as_ptr() as *const ::core::ffi::c_char,
        marker_state,
        star_index / 32 as ::core::ffi::c_int,
    );
    return &raw mut buf as *mut ::core::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn isStarMarked(
    star_index: ::core::ffi::c_int,
    mut marker_state: *const ::core::ffi::c_char,
) -> BOOLEAN {
    let mut starIndex_0: ::core::ffi::c_int = star_index;
    let mut starData: DWORD = 0;
    if star_index == -1 as ::core::ffi::c_int {
        starIndex_0 = CurStarDescPtr.offset_from(&raw mut star_array as *mut STAR_DESC)
            as COUNT as ::core::ffi::c_int;
    }
    starData = uqm_lua_checked!(getGameStateUint(markerBuf(starIndex_0, marker_state)), BOOLEAN(0));
    return BOOLEAN(starData >> starIndex_0 % 32 as ::core::ffi::c_int & 1 as DWORD);
}
#[no_mangle]
pub unsafe extern "C" fn setStarMarked(
    star_index: ::core::ffi::c_int,
    mut marker_state: *const ::core::ffi::c_char,
) {
    let mut starIndex_0: ::core::ffi::c_int = star_index;
    let mut starData: DWORD = 0;
    if starIndex_0 == -1 as ::core::ffi::c_int {
        starIndex_0 = CurStarDescPtr.offset_from(&raw mut star_array as *mut STAR_DESC)
            as COUNT as ::core::ffi::c_int;
    }
    starData = uqm_lua_checked!(getGameStateUint(markerBuf(starIndex_0, marker_state)), ());
    starData
        ^= ((1 as ::core::ffi::c_int) << starIndex_0 % 32 as ::core::ffi::c_int)
            as DWORD;
    uqm_lua_checked!(setGameStateUint(markerBuf(starIndex_0, marker_state), starData), ());
}
unsafe extern "C" fn CheckTextsIntersect(
    mut curr: *mut RECT,
    mut prev: *mut RECT,
) -> COORD {
    if (*curr).corner.x as ::core::ffi::c_int
        + (*curr).extent.width as ::core::ffi::c_int
        <= (*prev).corner.x as ::core::ffi::c_int
        || (*prev).corner.x as ::core::ffi::c_int
            + (*prev).extent.width as ::core::ffi::c_int
            <= (*curr).corner.x as ::core::ffi::c_int
    {
        return 0 as COORD;
    }
    if (*curr).corner.y as ::core::ffi::c_int
        + (*curr).extent.height as ::core::ffi::c_int
        <= (*prev).corner.y as ::core::ffi::c_int
        || (*prev).corner.y as ::core::ffi::c_int
            + (*prev).extent.height as ::core::ffi::c_int
            <= (*curr).corner.y as ::core::ffi::c_int
    {
        return 0 as COORD;
    }
    return ((*prev).extent.height as ::core::ffi::c_int
        + ((1 as ::core::ffi::c_int) << resolutionFactor)
        - abs(
            (*curr).corner.y as ::core::ffi::c_int
                - (*prev).corner.y as ::core::ffi::c_int,
        )) as COORD;
}
unsafe extern "C" fn AdjustTextRect(mut r: *mut RECT, mut t: *mut TEXT) {
    let mut offs: COORD = 0;
    if (*r).corner.x as ::core::ffi::c_int <= 0 as ::core::ffi::c_int {
        offs = ((*r).corner.x as ::core::ffi::c_int
            - ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        (*t).baseline.x = ((*t).baseline.x as ::core::ffi::c_int
            - offs as ::core::ffi::c_int) as COORD;
        (*r).corner.x = ((*r).corner.x as ::core::ffi::c_int
            - offs as ::core::ffi::c_int) as COORD;
    } else if (*r).corner.x as ::core::ffi::c_int
        + (*r).extent.width as ::core::ffi::c_int
        >= CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
    {
        offs = ((*r).corner.x as ::core::ffi::c_int
            + (*r).extent.width as ::core::ffi::c_int
            - (CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
                    } << resolutionFactor)))
            + ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        (*t).baseline.x = ((*t).baseline.x as ::core::ffi::c_int
            - offs as ::core::ffi::c_int) as COORD;
        (*r).corner.x = ((*r).corner.x as ::core::ffi::c_int
            - offs as ::core::ffi::c_int) as COORD;
    }
    if (*r).corner.y as ::core::ffi::c_int <= 0 as ::core::ffi::c_int {
        offs = ((*r).corner.y as ::core::ffi::c_int
            - ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        (*t).baseline.y = ((*t).baseline.y as ::core::ffi::c_int
            - offs as ::core::ffi::c_int) as COORD;
        (*r).corner.y = ((*r).corner.y as ::core::ffi::c_int
            - offs as ::core::ffi::c_int) as COORD;
    } else if (*r).corner.y as ::core::ffi::c_int
        + (*r).extent.height as ::core::ffi::c_int
        >= CanvasHeight
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
        offs = ((*r).corner.y as ::core::ffi::c_int
            + (*r).extent.height as ::core::ffi::c_int
            - (CanvasHeight
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
            + ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        (*t).baseline.y = ((*t).baseline.y as ::core::ffi::c_int
            - offs as ::core::ffi::c_int) as COORD;
        (*r).corner.y = ((*r).corner.y as ::core::ffi::c_int
            - offs as ::core::ffi::c_int) as COORD;
    }
}
unsafe extern "C" fn DrawRaceName(mut t: *mut TEXT, mut c: *mut Color) {
    (*c).r = (if (*c).r as ::core::ffi::c_int
        >= 0xff as ::core::ffi::c_int
            - ((0x3 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x3 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
    {
        0xff as ::core::ffi::c_int
    } else {
        (*c).r as ::core::ffi::c_int
            + ((0x3 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x3 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
    }) as BYTE;
    (*c).g = (if (*c).g as ::core::ffi::c_int
        >= 0xff as ::core::ffi::c_int
            - ((0x3 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x3 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
    {
        0xff as ::core::ffi::c_int
    } else {
        (*c).g as ::core::ffi::c_int
            + ((0x3 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x3 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
    }) as BYTE;
    (*c).b = (if (*c).b as ::core::ffi::c_int
        >= 0xff as ::core::ffi::c_int
            - ((0x3 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x3 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
    {
        0xff as ::core::ffi::c_int
    } else {
        (*c).b as ::core::ffi::c_int
            + ((0x3 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x3 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
    }) as BYTE;
    SetContextForeGroundColor(*c);
    uqm_lua_checked!(font_DrawText(t), ());
}
#[no_mangle]
pub unsafe extern "C" fn RaceColor(mut index: COUNT) -> Color {
    static mut race_colors: [Color; 26] = [
        Color {
            r: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0x10 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x10 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
        Color {
            r: 0xb5 as BYTE,
            g: 0xb5 as BYTE,
            b: 0xb5 as BYTE,
            a: 0xff as BYTE,
        },
        Color {
            r: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0x1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x1 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0x1f as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x1f as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
        Color {
            r: ((0xe as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0xe as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0xe as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0xe as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
        Color {
            r: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0x6 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x6 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0x8 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x8 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
        Color {
            r: ((0xa as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0xa as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0x6 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x6 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
        Color {
            r: ((0xc as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0xc as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0x5 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x5 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
        Color {
            r: ((0xc as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0xc as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0x5 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x5 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
        Color {
            r: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0x6 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x6 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0x8 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x8 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
        Color {
            r: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0x6 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x6 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0x8 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x8 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
        Color {
            r: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0x10 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x10 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
        Color {
            r: ((0xa as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0xa as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0x11 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x11 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
        Color {
            r: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0x6 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x6 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0x8 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x8 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
        Color {
            r: ((0xf as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0xf as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
        Color {
            r: ((0xe as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0xe as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0xe as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0xe as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
        Color {
            r: ((0xe as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0xe as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0xe as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0xe as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
        Color {
            r: ((0xc as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0xc as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0x5 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x5 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
        Color {
            r: ((0xa as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0xa as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0x11 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x11 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
        Color {
            r: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0x8 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x8 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
        Color {
            r: ((0xf as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0xf as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
        Color {
            r: 0x51 as BYTE,
            g: 0x18 as BYTE,
            b: 0x8b as BYTE,
            a: 0xff as BYTE,
        },
        Color {
            r: ((0x6 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x6 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0x6 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x6 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0x6 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x6 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
        Color {
            r: ((0x12 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x12 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0x10 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x10 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0x1f as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x1f as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
        Color {
            r: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0x15 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x15 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0x1c as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x1c as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
        Color {
            r: ((0x18 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x18 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0x18 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x18 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0x18 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x18 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
        Color {
            r: ((0x14 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x14 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            g: ((0x7 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x7 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            b: ((0x1f as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | 0x1f as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
            a: 0xff as BYTE,
        },
    ];
    if optSphereColors == C2Rust_Unnamed_1::OPTVAL_DEFAULT_COLORS.0 as ::core::ffi::c_int
    {
        return race_colors[index as usize];
    }
    match index as ::core::ffi::c_int {
        0 => {
            return Color {
                r: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                g: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                b: ((0x17 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x17 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                a: 0xff as BYTE,
            };
        }
        1 => {
            return Color {
                r: ((0x10 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x10 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                g: ((0x15 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x15 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                b: ((0x19 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x19 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                a: 0xff as BYTE,
            };
        }
        2 => {
            return Color {
                r: ((0x9 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x9 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                g: ((0x9 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x9 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                b: ((0x1b as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x1b as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                a: 0xff as BYTE,
            };
        }
        3 => {
            return Color {
                r: ((0xd as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0xd as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                g: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                b: ((0x5 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x5 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                a: 0xff as BYTE,
            };
        }
        4 => {
            return Color {
                r: ((0x1c as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x1c as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                g: ((0x7 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x7 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                b: ((0x19 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x19 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                a: 0xff as BYTE,
            };
        }
        5 => {
            return Color {
                r: ((0xa as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0xa as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                g: ((0x6 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x6 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                a: 0xff as BYTE,
            };
        }
        6 => {
            return Color {
                r: ((0x15 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x15 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                g: ((0x12 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x12 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                b: ((0x3 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x3 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                a: 0xff as BYTE,
            };
        }
        7 => {
            return Color {
                r: ((0x5 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x5 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                g: ((0x14 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x14 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                b: ((0x1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x1 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                a: 0xff as BYTE,
            };
        }
        8 => {
            return Color {
                r: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                g: ((0x6 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x6 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                b: ((0x8 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x8 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                a: 0xff as BYTE,
            };
        }
        9 => {
            return Color {
                r: ((0x1a as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x1a as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                g: ((0x16 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x16 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                b: ((0x12 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x12 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                a: 0xff as BYTE,
            };
        }
        10 => {
            return Color {
                r: ((0x6 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x6 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                g: ((0x16 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x16 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                b: ((0xf as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0xf as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                a: 0xff as BYTE,
            };
        }
        11 => {
            return Color {
                r: ((0xa as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0xa as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                g: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                b: ((0x11 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x11 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                a: 0xff as BYTE,
            };
        }
        12 => {
            return Color {
                r: ((0x1a as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x1a as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                g: ((0x16 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x16 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                b: ((0x8 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x8 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                a: 0xff as BYTE,
            };
        }
        13 => {
            return Color {
                r: ((0xf as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0xf as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                g: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                a: 0xff as BYTE,
            };
        }
        14 => {
            return Color {
                r: ((0xe as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0xe as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                g: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                b: ((0xe as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0xe as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                a: 0xff as BYTE,
            };
        }
        15 => {
            return Color {
                r: ((0x14 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x14 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                g: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                b: ((0xc as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0xc as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                a: 0xff as BYTE,
            };
        }
        16 => {
            return Color {
                r: ((0x1d as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x1d as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                g: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                b: ((0x7 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x7 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                a: 0xff as BYTE,
            };
        }
        17 => {
            return Color {
                r: ((0xb as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0xb as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                g: ((0xc as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0xc as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                a: 0xff as BYTE,
            };
        }
        18 => {
            return Color {
                r: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                g: ((0x8 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x8 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                a: 0xff as BYTE,
            };
        }
        19 => {
            return Color {
                r: ((0x16 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x16 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                g: ((0xd as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0xd as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                b: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                a: 0xff as BYTE,
            };
        }
        20 => {
            return Color {
                r: ((0x15 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x15 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                g: ((0x7 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x7 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                b: ((0x3 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x3 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                a: 0xff as BYTE,
            };
        }
        21 => {
            return Color {
                r: ((0x6 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x6 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                g: ((0x6 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x6 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                b: ((0x6 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x6 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                a: 0xff as BYTE,
            };
        }
        22 => {
            return Color {
                r: ((0x9 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x9 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                g: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                b: ((0x15 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x15 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                a: 0xff as BYTE,
            };
        }
        23 => {
            return Color {
                r: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                g: ((0x15 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x15 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                b: ((0x1c as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x1c as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                a: 0xff as BYTE,
            };
        }
        24 => {
            return Color {
                r: ((0x10 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x10 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                g: ((0x10 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x10 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                b: ((0x10 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x10 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                a: 0xff as BYTE,
            };
        }
        25 => {
            return Color {
                r: ((0x19 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x19 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                g: ((0x10 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x10 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                b: ((0x1f as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x1f as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                a: 0xff as BYTE,
            };
        }
        _ => {
            return Color {
                r: ((0x1c as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0x1c as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                g: ((0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                b: ((0xc as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | 0xc as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as BYTE,
                a: 0xff as BYTE,
            };
        }
    };
}
unsafe extern "C" fn DrawStarMap(mut race_update: COUNT, mut pClipRect: *mut RECT) {
    let mut i: SIZE = 0;
    let mut which_space: COUNT = 0;
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut old_r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut oldOrigin: POINT = point {
        x: 0 as COORD,
        y: 0 as COORD,
    };
    let mut s: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    let mut star_frame: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
    let mut SDPtr: *mut STAR_DESC = ::core::ptr::null_mut::<STAR_DESC>();
    let mut draw_cursor: BOOLEAN = BOOLEAN::FALSE;
    if pClipRect
        == ::core::ptr::with_exposed_provenance_mut::<RECT>(-1 as ::core::ffi::c_int as usize)
    {
        pClipRect = ::core::ptr::null_mut::<RECT>();
        draw_cursor = BOOLEAN::FALSE;
    } else {
        draw_cursor = BOOLEAN::TRUE;
    }
    SetContext(SpaceContext);
    if !pClipRect.is_null() {
        GetContextClipRect(&raw mut old_r);
        (*pClipRect).corner.x = ((*pClipRect).corner.x as ::core::ffi::c_int
            + old_r.corner.x as ::core::ffi::c_int) as COORD;
        (*pClipRect).corner.y = ((*pClipRect).corner.y as ::core::ffi::c_int
            + old_r.corner.y as ::core::ffi::c_int) as COORD;
        SetContextClipRect(pClipRect);
        (*pClipRect).corner.x = ((*pClipRect).corner.x as ::core::ffi::c_int
            - old_r.corner.x as ::core::ffi::c_int) as COORD;
        (*pClipRect).corner.y = ((*pClipRect).corner.y as ::core::ffi::c_int
            - old_r.corner.y as ::core::ffi::c_int) as COORD;
        oldOrigin = SetContextOrigin(
            MAKE_POINT(
                -((*pClipRect).corner.x as ::core::ffi::c_int) as COORD,
                -((*pClipRect).corner.y as ::core::ffi::c_int) as COORD,
            ),
        );
    }
    if transition_pending.0 != 0 {
        SetTransitionSource(::core::ptr::null::<RECT>());
    }
    BatchGraphics();
    which_space = uqm_lua_checked!(getGameStateUint(
        b"ARILOU_SPACE_SIDE\0".as_ptr() as *const ::core::ffi::c_char,
    ), ()) as COUNT;
    if which_space as ::core::ffi::c_int <= 1 as ::core::ffi::c_int {
        SDPtr = (&raw mut star_array as *mut STAR_DESC).offset(0isize);
        SetContextForeGroundColor(colorFromRgb15(0 as BYTE, 0 as BYTE, 0x7 as BYTE));
        SetContextBackGroundColor(colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE));
    } else {
        SDPtr = (&raw mut star_array as *mut STAR_DESC)
            .offset((502 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize);
        SetContextForeGroundColor(colorFromRgb15(0 as BYTE, 0xb as BYTE, 0 as BYTE));
        SetContextBackGroundColor(colorFromRgb15(0 as BYTE, 0x8 as BYTE, 0 as BYTE));
    }
    uqm_lua_checked!(ClearDrawable(), ());
    if which_starmap.0 != CURRENT_STARMAP_SHOWN::CONSTELLATION_MAP.0
        && (race_update as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            && (which_space as ::core::ffi::c_int) < 2 as ::core::ffi::c_int)
        && !(optInfiniteFuel.0 != 0 || GlobData.SIS_state.FuelOnBoard == 0 as DWORD)
    {
        uqm_lua_checked!(DrawFuelCircle(BOOLEAN::FALSE), ());
    }
    r.corner.x = ((universeToDispx(0 as ::core::ffi::c_long) as ::core::ffi::c_int)
        << resolutionFactor) as COORD;
    r.extent.width = ((CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
            } << resolutionFactor)) << zoomLevel)
        - ((1 as ::core::ffi::c_int) << resolutionFactor << zoomLevel)) as COORD;
    r.extent.height = ((1 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    i = 9999 as SIZE;
    while (i as ::core::ffi::c_int) >= 0 as ::core::ffi::c_int {
        r.corner.y = ((universeToDispy(i as ::core::ffi::c_long) as ::core::ffi::c_int)
            << resolutionFactor) as COORD;
        uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
        i = (i as ::core::ffi::c_int - 500 as ::core::ffi::c_int) as SIZE;
    }
    r.corner.y = ((universeToDispy(0 as ::core::ffi::c_long) as ::core::ffi::c_int)
        << resolutionFactor) as COORD;
    uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
    r.corner.y = (((universeToDispy(9999 as ::core::ffi::c_long) as ::core::ffi::c_int)
        << resolutionFactor) + ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    r.extent.width = ((1 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    r.extent.height = ((CanvasHeight
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
        - ((13 as ::core::ffi::c_int) << resolutionFactor) << zoomLevel)
        - ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    i = 0 as SIZE;
    while (i as ::core::ffi::c_int) < 9999 as ::core::ffi::c_int {
        r.corner.x = ((universeToDispx(i as ::core::ffi::c_long) as ::core::ffi::c_int)
            << resolutionFactor) as COORD;
        uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
        i = (i as ::core::ffi::c_int + 500 as ::core::ffi::c_int) as SIZE;
    }
    r.corner.x = ((universeToDispx(9999 as ::core::ffi::c_long) as ::core::ffi::c_int)
        << resolutionFactor) as COORD;
    r.corner.y = ((universeToDispy(9999 as ::core::ffi::c_long) as ::core::ffi::c_int)
        << resolutionFactor) as COORD;
    r.extent.height = ((CanvasHeight
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
        - ((13 as ::core::ffi::c_int) << resolutionFactor) << zoomLevel)
        + ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    if r.extent.height as ::core::ffi::c_int
        - ((1 as ::core::ffi::c_int) << resolutionFactor)
        > -(((universeToDispy(9999 as ::core::ffi::c_long) as ::core::ffi::c_int)
            << resolutionFactor)
            - ((universeToDispy(0 as ::core::ffi::c_long) as ::core::ffi::c_int)
                << resolutionFactor))
    {
        r.extent.height = (r.extent.height as ::core::ffi::c_int
            - ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    }
    uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
    if which_starmap.0 != CURRENT_STARMAP_SHOWN::CONSTELLATION_MAP.0
        && (race_update as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            && (which_space as ::core::ffi::c_int) < 2 as ::core::ffi::c_int)
        && !(optInfiniteFuel.0 != 0 || GlobData.SIS_state.FuelOnBoard == 0 as DWORD)
        && (optFuelRange == 1 as ::core::ffi::c_int
            || optFuelRange == 3 as ::core::ffi::c_int)
        && (GlobData.Game_state.autopilot.x as ::core::ffi::c_int
            != !(0 as ::core::ffi::c_int)
            && GlobData.Game_state.autopilot.y as ::core::ffi::c_int
                != !(0 as ::core::ffi::c_int))
    {
        uqm_lua_checked!(DrawFuelCircle(BOOLEAN::TRUE), ());
    }
    star_frame = SetRelFrameIndex(StarMapFrame, 2 as SIZE);
    if which_space as ::core::ffi::c_int <= 1 as ::core::ffi::c_int
        && which_starmap.0 != CURRENT_STARMAP_SHOWN::CONSTELLATION_MAP.0
    {
        let mut index: COUNT = 0;
        let mut race_index: COUNT = ((race_update as ::core::ffi::c_int
            & 0x1f as ::core::ffi::c_int) - 1 as ::core::ffi::c_int) as COUNT;
        let mut hStarShip: HFLEETINFO = ::core::ptr::null_mut::<::core::ffi::c_void>();
        let mut hNextShip: HFLEETINFO = ::core::ptr::null_mut::<::core::ffi::c_void>();
        let mut nameplate: [NAMEPLATE; 26] = [NAMEPLATE {
            rect: RECT {
                corner: POINT { x: 0, y: 0 },
                extent: EXTENT { width: 0, height: 0 },
            },
            text: TEXT {
                baseline: POINT { x: 0, y: 0 },
                pStr: ::core::ptr::null::<UNICODE>(),
                align: TEXT_ALIGN::ALIGN_LEFT,
                CharCount: 0,
            },
            index: 0,
        }; 26];
        let mut currMax: BYTE = 0 as BYTE;
        index = 0 as COUNT;
        hStarShip = GlobData.Game_state.avail_race_q.head as HFLEETINFO;
        while !hStarShip.is_null() {
            let mut FleetPtr: *mut FLEET_INFO = ::core::ptr::null_mut::<FLEET_INFO>();
            FleetPtr = LockFleetInfo(
                &raw mut GlobData.Game_state.avail_race_q,
                hStarShip,
            );
            hNextShip = (*FleetPtr).succ;
            if (*FleetPtr).known_strength as ::core::ffi::c_int != 0
                && which_starmap.0 != CURRENT_STARMAP_SHOWN::WAR_ERA_STARMAP.0
                || which_starmap.0 == CURRENT_STARMAP_SHOWN::WAR_ERA_STARMAP.0
                    && WarEraStrength((*FleetPtr).SpeciesID) as ::core::ffi::c_int != 0
                    && (index as ::core::ffi::c_int)
                        < RACE_ID::NUM_BUILDABLE_SHIPS.0 as ::core::ffi::c_int
            {
                let mut repair_r: RECT = RECT {
                    corner: POINT { x: 0, y: 0 },
                    extent: EXTENT { width: 0, height: 0 },
                };
                if which_starmap.0 == CURRENT_STARMAP_SHOWN::WAR_ERA_STARMAP.0 {
                    GetWarEraSphereRect(FleetPtr, &raw mut r, &raw mut repair_r);
                } else {
                    GetSphereRect(FleetPtr, &raw mut r, &raw mut repair_r);
                }
                if (r.corner.x as ::core::ffi::c_int)
                    < CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
                    && (r.corner.y as ::core::ffi::c_int)
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
                    && r.corner.x as ::core::ffi::c_int
                        + r.extent.width as ::core::ffi::c_int > 0 as ::core::ffi::c_int
                    && r.corner.y as ::core::ffi::c_int
                        + r.extent.height as ::core::ffi::c_int > 0 as ::core::ffi::c_int
                    && (pClipRect.is_null()
                        || (repair_r.corner.x as ::core::ffi::c_int)
                            < (*pClipRect).corner.x as ::core::ffi::c_int
                                + (*pClipRect).extent.width as ::core::ffi::c_int
                            && (repair_r.corner.y as ::core::ffi::c_int)
                                < (*pClipRect).corner.y as ::core::ffi::c_int
                                    + (*pClipRect).extent.height as ::core::ffi::c_int
                            && repair_r.corner.x as ::core::ffi::c_int
                                + repair_r.extent.width as ::core::ffi::c_int
                                > (*pClipRect).corner.x as ::core::ffi::c_int
                            && repair_r.corner.y as ::core::ffi::c_int
                                + repair_r.extent.height as ::core::ffi::c_int
                                > (*pClipRect).corner.y as ::core::ffi::c_int)
                {
                    let mut c: Color = Color { r: 0, g: 0, b: 0, a: 0 };
                    let mut t: TEXT = TEXT {
                        baseline: POINT { x: 0, y: 0 },
                        pStr: ::core::ptr::null::<UNICODE>(),
                        align: TEXT_ALIGN::ALIGN_LEFT,
                        CharCount: 0,
                    };
                    let mut locString: STRING = ::core::ptr::null_mut::<
                        STRING_TABLE_ENTRY_DESC,
                    >();
                    let mut yehat_rebels: *mut UNICODE = b"+Yehat+\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut UNICODE;
                    c = RaceColor(index);
                    if index as ::core::ffi::c_int == race_index as ::core::ffi::c_int {
                        SetContextForeGroundColor(
                            colorFromRgb15(0x1f as BYTE, 0x1f as BYTE, 0x1f as BYTE),
                        );
                    } else {
                        SetContextForeGroundColor(c);
                    }
                    let mut dr: DRECT = RECT_TO_DRECT(r);
                    uqm_lua_checked!(DrawOval(
                        &raw mut dr,
                        0 as BYTE,
                        BOOLEAN(
                            (if resolutionFactor != 2 as ::core::ffi::c_uint {
                                BOOLEAN::FALSE.0 as ::core::ffi::c_int
                            } else {
                                BOOLEAN::TRUE.0 as ::core::ffi::c_int
                            }) as ::core::ffi::c_uint,
                        ),
                    ), ());
                    if isPC(optWhichFonts).0 != 0 {
                        SetContextFont(TinyFont);
                    } else {
                        SetContextFont(TinyFontBold);
                    }
                    t.baseline.x = (r.corner.x as ::core::ffi::c_int
                        + (r.extent.width as ::core::ffi::c_int
                            >> 1 as ::core::ffi::c_int)) as COORD;
                    t.baseline.y = (r.corner.y as ::core::ffi::c_int
                        + (r.extent.height as ::core::ffi::c_int
                            >> 1 as ::core::ffi::c_int)
                        - ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
                    t.align = TEXT_ALIGN::ALIGN_CENTER;
                    if which_starmap.0 == CURRENT_STARMAP_SHOWN::WAR_ERA_STARMAP.0 {
                        match index as ::core::ffi::c_int {
                            4 | 8 | 13 => {
                                t.pStr = GetStringAddress(
                                    SetAbsStringTableIndex(
                                        GameStrings,
                                        (C2Rust_Unnamed_9::STAR_STRING_BASE.0 as ::core::ffi::c_int
                                            + 132 as ::core::ffi::c_int) as COUNT,
                                    ),
                                );
                                t.CharCount = strlen(t.pStr) as COUNT;
                            }
                            22 => {
                                locString = SetAbsStringTableIndex(
                                    (*FleetPtr).race_strings,
                                    0 as COUNT,
                                );
                                t.pStr = GetStringAddress(locString);
                                t.CharCount = GetStringLength(locString);
                            }
                            _ => {
                                locString = SetAbsStringTableIndex(
                                    (*FleetPtr).race_strings,
                                    1 as COUNT,
                                );
                                t.CharCount = GetStringLength(locString);
                                t.pStr = GetStringAddress(locString);
                            }
                        }
                    } else if index as ::core::ffi::c_int
                        == RACE_ID::YEHAT_REBEL_SHIP.0 as ::core::ffi::c_int
                        && optSphereColors == 1 as ::core::ffi::c_int
                    {
                        t.CharCount = 7 as COUNT;
                        t.pStr = yehat_rebels;
                    } else {
                        locString = SetAbsStringTableIndex(
                            (*FleetPtr).race_strings,
                            1 as COUNT,
                        );
                        t.CharCount = GetStringLength(locString);
                        t.pStr = GetStringAddress(locString);
                    }
                    TextRect(&raw mut t, &raw mut r, ::core::ptr::null_mut::<BYTE>());
                    if index as ::core::ffi::c_int == race_index as ::core::ffi::c_int
                        && race_update as ::core::ffi::c_int
                            & (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int != 0
                    {
                        AdjustTextRect(&raw mut r, &raw mut t);
                        uqm_lua_checked!(DrawRaceName(&raw mut t, &raw mut c), ());
                    } else {
                        nameplate[currMax as usize].rect = r;
                        nameplate[currMax as usize].text = t;
                        nameplate[currMax as usize].index = index as BYTE;
                        currMax = currMax.wrapping_add(1);
                    }
                }
            } else if index as ::core::ffi::c_int == race_index as ::core::ffi::c_int
                && race_update as ::core::ffi::c_int
                    & ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                        | (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) != 0
            {
                let mut t_0: TEXT = TEXT {
                    baseline: POINT { x: 0, y: 0 },
                    pStr: ::core::ptr::null::<UNICODE>(),
                    align: TEXT_ALIGN::ALIGN_LEFT,
                    CharCount: 0,
                };
                let mut locString_0: STRING = ::core::ptr::null_mut::<
                    STRING_TABLE_ENTRY_DESC,
                >();
                let mut repair_r_0: RECT = RECT {
                    corner: POINT { x: 0, y: 0 },
                    extent: EXTENT { width: 0, height: 0 },
                };
                locString_0 = SetAbsStringTableIndex(
                    (*FleetPtr).race_strings,
                    1 as COUNT,
                );
                t_0.CharCount = GetStringLength(locString_0);
                t_0.pStr = GetStringAddress(locString_0);
                GetSphereRect(FleetPtr, &raw mut r, &raw mut repair_r_0);
                t_0.baseline.x = (r.corner.x as ::core::ffi::c_int
                    + (r.extent.width as ::core::ffi::c_int >> 1 as ::core::ffi::c_int))
                    as COORD;
                t_0.baseline.y = (r.corner.y as ::core::ffi::c_int
                    + (r.extent.height as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
                    - ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
                t_0.align = TEXT_ALIGN::ALIGN_CENTER;
                TextRect(&raw mut t_0, &raw mut r, ::core::ptr::null_mut::<BYTE>());
                nameplate[currMax as usize].rect = r;
                nameplate[currMax as usize].text = t_0;
                nameplate[currMax as usize].index = (race_update as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as BYTE;
                currMax = currMax.wrapping_add(1);
            }
            UnlockLink(&raw mut GlobData.Game_state.avail_race_q, hStarShip);
            index = index.wrapping_add(1);
            hStarShip = hNextShip;
        }
        if currMax as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
            let mut j: BYTE = 0;
            let mut k: BYTE = 0;
            let mut swapped: BOOLEAN = BOOLEAN::FALSE;
            let mut offs: COORD = 0;
            let mut t_1: TEXT = TEXT {
                baseline: POINT { x: 0, y: 0 },
                pStr: ::core::ptr::null::<UNICODE>(),
                align: TEXT_ALIGN::ALIGN_LEFT,
                CharCount: 0,
            };
            let mut c_0: Color = Color { r: 0, g: 0, b: 0, a: 0 };
            let mut mid: BYTE = currMax;
            j = 0 as BYTE;
            while (j as ::core::ffi::c_int)
                < currMax as ::core::ffi::c_int - 1 as ::core::ffi::c_int
            {
                swapped = BOOLEAN::FALSE;
                k = 0 as BYTE;
                while (k as ::core::ffi::c_int)
                    < currMax as ::core::ffi::c_int - j as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int
                {
                    if nameplate[k as usize].rect.corner.y as ::core::ffi::c_int
                        > nameplate[(k as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                as usize]
                            .rect
                            .corner
                            .y as ::core::ffi::c_int
                    {
                        SwapPlates(
                            (&raw mut nameplate as *mut NAMEPLATE).offset(k as isize),
                            (&raw mut nameplate as *mut NAMEPLATE)
                                .offset(
                                    (k as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                                ),
                        );
                        swapped = BOOLEAN::TRUE;
                    }
                    k = k.wrapping_add(1);
                }
                if swapped.0 == BOOLEAN::FALSE.0 {
                    break;
                }
                j = j.wrapping_add(1);
            }
            j = 0 as BYTE;
            while (j as ::core::ffi::c_int) < currMax as ::core::ffi::c_int {
                if nameplate[j as usize].index as ::core::ffi::c_int
                    & (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int != 0
                {
                    c_0 = colorFromRgb15(0x1f as BYTE, 0x1f as BYTE, 0x1f as BYTE);
                } else if nameplate[j as usize].index as ::core::ffi::c_int
                    & (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int != 0
                {
                    c_0 = buildColorRgba(
                        0x80 as BYTE,
                        0x80 as BYTE,
                        0x80 as BYTE,
                        0xff as BYTE,
                    );
                } else {
                    c_0 = RaceColor(
                        (nameplate[j as usize].index as ::core::ffi::c_int
                            & 0x1f as ::core::ffi::c_int) as COUNT,
                    );
                }
                r = nameplate[j as usize].rect;
                t_1 = nameplate[j as usize].text;
                AdjustTextRect(&raw mut r, &raw mut t_1);
                r.corner.y = (r.corner.y as ::core::ffi::c_int
                    + ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
                r.extent.height = (r.extent.height as ::core::ffi::c_int
                    - ((2 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
                if r.corner.y as ::core::ffi::c_int
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
                        >> 1 as ::core::ffi::c_int
                {
                    break;
                }
                k = 0 as BYTE;
                while (k as ::core::ffi::c_int) < j as ::core::ffi::c_int {
                    offs = CheckTextsIntersect(
                        &raw mut r,
                        &raw mut (*(&raw mut nameplate as *mut NAMEPLATE)
                            .offset(k as isize))
                            .rect,
                    );
                    if offs as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                        r.corner.y = (r.corner.y as ::core::ffi::c_int
                            + offs as ::core::ffi::c_int) as COORD;
                        t_1.baseline.y = (t_1.baseline.y as ::core::ffi::c_int
                            + offs as ::core::ffi::c_int) as COORD;
                    }
                    k = k.wrapping_add(1);
                }
                nameplate[j as usize].rect = r;
                uqm_lua_checked!(DrawRaceName(&raw mut t_1, &raw mut c_0), ());
                j = j.wrapping_add(1);
                mid = mid.wrapping_sub(1);
            }
            j = 1 as BYTE;
            while j as ::core::ffi::c_int <= mid as ::core::ffi::c_int {
                if nameplate[(currMax as ::core::ffi::c_int - j as ::core::ffi::c_int)
                        as usize]
                    .index as ::core::ffi::c_int
                    & (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int != 0
                {
                    c_0 = colorFromRgb15(0x1f as BYTE, 0x1f as BYTE, 0x1f as BYTE);
                } else if nameplate[(currMax as ::core::ffi::c_int
                        - j as ::core::ffi::c_int) as usize]
                    .index as ::core::ffi::c_int
                    & (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int != 0
                {
                    c_0 = buildColorRgba(
                        0x80 as BYTE,
                        0x80 as BYTE,
                        0x80 as BYTE,
                        0xff as BYTE,
                    );
                } else {
                    c_0 = RaceColor(
                        (nameplate[(currMax as ::core::ffi::c_int
                                - j as ::core::ffi::c_int) as usize]
                            .index as ::core::ffi::c_int & 0x1f as ::core::ffi::c_int)
                            as COUNT,
                    );
                }
                r = nameplate[(currMax as ::core::ffi::c_int - j as ::core::ffi::c_int)
                        as usize]
                    .rect;
                t_1 = nameplate[(currMax as ::core::ffi::c_int - j as ::core::ffi::c_int)
                        as usize]
                    .text;
                AdjustTextRect(&raw mut r, &raw mut t_1);
                r.corner.y = (r.corner.y as ::core::ffi::c_int
                    + ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
                r.extent.height = (r.extent.height as ::core::ffi::c_int
                    - ((2 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
                k = 1 as BYTE;
                while (k as ::core::ffi::c_int) < j as ::core::ffi::c_int {
                    offs = CheckTextsIntersect(
                        &raw mut r,
                        &raw mut (*(&raw mut nameplate as *mut NAMEPLATE)
                            .offset(
                                (currMax as ::core::ffi::c_int - k as ::core::ffi::c_int)
                                    as isize,
                            ))
                            .rect,
                    );
                    if offs as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                        r.corner.y = (r.corner.y as ::core::ffi::c_int
                            - offs as ::core::ffi::c_int) as COORD;
                        t_1.baseline.y = (t_1.baseline.y as ::core::ffi::c_int
                            - offs as ::core::ffi::c_int) as COORD;
                    }
                    k = k.wrapping_add(1);
                }
                nameplate[(currMax as ::core::ffi::c_int - j as ::core::ffi::c_int)
                        as usize]
                    .rect = r;
                uqm_lua_checked!(DrawRaceName(&raw mut t_1, &raw mut c_0), ());
                j = j.wrapping_add(1);
            }
        }
    }
    if which_space as ::core::ffi::c_int <= 1 as ::core::ffi::c_int
        && which_starmap.0 == CURRENT_STARMAP_SHOWN::CONSTELLATION_MAP.0
    {
        let mut oldColor: Color = Color { r: 0, g: 0, b: 0, a: 0 };
        let mut CNPtr: *mut POINT = ::core::ptr::null_mut::<POINT>();
        let mut l: LINE = LINE {
            first: POINT { x: 0, y: 0 },
            second: POINT { x: 0, y: 0 },
        };
        let mut c_1: BYTE = (0x3f as ::core::ffi::c_int
            + if if resolutionFactor != 2 as ::core::ffi::c_uint {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } == 0
            {
                0 as ::core::ffi::c_int
            } else {
                0x11 as ::core::ffi::c_int
            }) as BYTE;
        CNPtr = constel_array.offset(0isize);
        oldColor = SetContextForeGroundColor(
            buildColorRgba(c_1, c_1, c_1, 0xff as BYTE),
        );
        while ((*CNPtr).x as ::core::ffi::c_int) < 9999 as ::core::ffi::c_int
            && ((*CNPtr).y as ::core::ffi::c_int) < 9999 as ::core::ffi::c_int
        {
            l.first.x = (((universeToDispx((*CNPtr).x as ::core::ffi::c_long)
                as ::core::ffi::c_int) << resolutionFactor)
                + if if resolutionFactor != 2 as ::core::ffi::c_uint {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } == 0
                {
                    0 as ::core::ffi::c_int
                } else {
                    2 as ::core::ffi::c_int
                }) as COORD;
            l.first.y = (((universeToDispy((*CNPtr).y as ::core::ffi::c_long)
                as ::core::ffi::c_int) << resolutionFactor)
                + if if resolutionFactor != 2 as ::core::ffi::c_uint {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } == 0
                {
                    0 as ::core::ffi::c_int
                } else {
                    2 as ::core::ffi::c_int
                }) as COORD;
            CNPtr = CNPtr.offset(1);
            l.second.x = (((universeToDispx((*CNPtr).x as ::core::ffi::c_long)
                as ::core::ffi::c_int) << resolutionFactor)
                + if if resolutionFactor != 2 as ::core::ffi::c_uint {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } == 0
                {
                    0 as ::core::ffi::c_int
                } else {
                    2 as ::core::ffi::c_int
                }) as COORD;
            l.second.y = (((universeToDispy((*CNPtr).y as ::core::ffi::c_long)
                as ::core::ffi::c_int) << resolutionFactor)
                + if if resolutionFactor != 2 as ::core::ffi::c_uint {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } == 0
                {
                    0 as ::core::ffi::c_int
                } else {
                    2 as ::core::ffi::c_int
                }) as COORD;
            CNPtr = CNPtr.offset(1);
            uqm_lua_checked!(DrawLine(&raw mut l, 1 as BYTE), ());
        }
        SetContextForeGroundColor(oldColor);
    }
    if which_space as ::core::ffi::c_int <= 1 as ::core::ffi::c_int
        && which_starmap.0 == CURRENT_STARMAP_SHOWN::HOMEWORLDS_MAP.0
    {
        let mut i_0: COUNT = 0;
        i_0 = 0 as COUNT;
        while (i_0 as ::core::ffi::c_int)
            < 502 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
        {
            let mut Index: BYTE = (*(&raw mut star_array as *mut STAR_DESC)
                .offset(i_0 as isize))
                .Index;
            if uqm_lua_checked!(isHomeworld(Index), ()).0 != 0 {
                uqm_lua_checked!(DrawMarker(
                    (*(&raw mut star_array as *mut STAR_DESC).offset(i_0 as isize))
                        .star_pt,
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int as BYTE,
                ), ());
            }
            i_0 = i_0.wrapping_add(1);
        }
        i_0 = 0 as COUNT;
        while (i_0 as ::core::ffi::c_int) < 15 as ::core::ffi::c_int {
            if uqm_lua_checked!(getGameStateUint(
                b"KNOW_QS_PORTAL\0".as_ptr() as *const ::core::ffi::c_char,
            ), ()) & ((1 as ::core::ffi::c_int) << i_0 as ::core::ffi::c_int) as DWORD != 0
            {
                let mut t_2: TEXT = TEXT {
                    baseline: POINT { x: 0, y: 0 },
                    pStr: ::core::ptr::null::<UNICODE>(),
                    align: TEXT_ALIGN::ALIGN_LEFT,
                    CharCount: 0,
                };
                let mut pStr: [UNICODE; 2] = [0; 2];
                pStr[0usize] = ('A' as ::core::ffi::c_int + i_0 as ::core::ffi::c_int)
                    as UNICODE;
                pStr[1usize] = 0 as UNICODE;
                SetContextFont(TinyFontBold);
                SetContextForeGroundColor(
                    colorFromRgb15(0 as BYTE, 0x1a as BYTE, 0 as BYTE),
                );
                t_2.baseline.x = ((universeToDispx(
                    portal_map[i_0 as usize].star_pt.x as ::core::ffi::c_long,
                ) as ::core::ffi::c_int) << resolutionFactor) as COORD;
                t_2.baseline.y = (((universeToDispy(
                    portal_map[i_0 as usize].star_pt.y as ::core::ffi::c_long,
                ) as ::core::ffi::c_int) << resolutionFactor) + 3 as ::core::ffi::c_int)
                    as COORD;
                t_2.align = TEXT_ALIGN::ALIGN_CENTER;
                t_2.pStr = &raw mut pStr as *mut UNICODE;
                t_2.CharCount = 2 as COUNT;
                uqm_lua_checked!(font_DrawText(&raw mut t_2), ());
            }
            i_0 = i_0.wrapping_add(1);
        }
    }
    if which_space as ::core::ffi::c_int <= 1 as ::core::ffi::c_int
        && which_starmap.0 == CURRENT_STARMAP_SHOWN::RAINBOW_MAP.0
    {
        let mut rainbow_mask: UWORD = 0;
        rainbow_mask = (((uqm_lua_checked!(getGameStateUint(
            b"RAINBOW_WORLD1\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) as BYTE as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as UWORD
            as ::core::ffi::c_int
            | uqm_lua_checked!(getGameStateUint(
                b"RAINBOW_WORLD0\0".as_ptr() as *const ::core::ffi::c_char,
            ), ()) as BYTE as ::core::ffi::c_int) as UWORD;
        if rainbow_mask as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            which_starmap = CURRENT_STARMAP_SHOWN::NORMAL_STARMAP;
        } else {
            let mut i_1: COUNT = 0;
            i_1 = C2Rust_Unnamed_5::RAINBOW0_DEFINED.0 as ::core::ffi::c_int as COUNT;
            while i_1 as ::core::ffi::c_int
                <= C2Rust_Unnamed_5::RAINBOW9_DEFINED.0 as ::core::ffi::c_int
            {
                if rainbow_mask as ::core::ffi::c_int
                    & (1 as ::core::ffi::c_int)
                        << i_1 as ::core::ffi::c_int
                            - C2Rust_Unnamed_5::RAINBOW0_DEFINED.0 as ::core::ffi::c_int
                    != 0
                {
                    uqm_lua_checked!(DrawMarker(
                        plot_map[i_1 as usize].star_pt,
                        BOOLEAN::TRUE.0 as ::core::ffi::c_int as BYTE,
                    ), ());
                }
                i_1 = i_1.wrapping_add(1);
            }
        }
    }
    loop {
        let mut star_type: BYTE = 0;
        static mut i_2: COUNT = 0 as COUNT;
        i_2 = (if i_2 as ::core::ffi::c_int >= 502 as ::core::ffi::c_int {
            0 as ::core::ffi::c_int
        } else {
            i_2 as ::core::ffi::c_int
        }) as COUNT;
        star_type = (*SDPtr).Type;
        s.origin.x = ((universeToDispx((*SDPtr).star_pt.x as ::core::ffi::c_long)
            as ::core::ffi::c_int) << resolutionFactor) as COORD;
        s.origin.y = ((universeToDispy((*SDPtr).star_pt.y as ::core::ffi::c_long)
            as ::core::ffi::c_int) << resolutionFactor) as COORD;
        if which_space as ::core::ffi::c_int <= 1 as ::core::ffi::c_int {
            if which_starmap.0 == CURRENT_STARMAP_SHOWN::NORMAL_STARMAP.0
                && uqm_lua_checked!(isStarMarked(
                        i_2 as ::core::ffi::c_int,
                        b"PLYR_MARKER\0".as_ptr() as *const ::core::ffi::c_char,
                    ), ())
                    .0 != 0
            {
                uqm_lua_checked!(DrawMarker((*SDPtr).star_pt, 2 as BYTE), ());
            }
            if optShowVisitedStars.0 != 0
                && uqm_lua_checked!(isStarMarked(
                        i_2 as ::core::ffi::c_int,
                        b"VISITED\0".as_ptr() as *const ::core::ffi::c_char,
                    ), ())
                    .0 != 0 && which_starmap.0 == CURRENT_STARMAP_SHOWN::NORMAL_STARMAP.0
                && (*SDPtr).Index as ::core::ffi::c_int
                    != C2Rust_Unnamed_5::SOL_DEFINED.0 as ::core::ffi::c_int
            {
                s.frame = SetRelFrameIndex(
                    visitedStarsFrame,
                    (((star_type as ::core::ffi::c_int
                        & ((0xff as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                            & !(((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int)
                                as BYTE as ::core::ffi::c_int)) as BYTE
                            as ::core::ffi::c_int) >> 3 as ::core::ffi::c_int) as BYTE
                        as ::core::ffi::c_int
                        * C2Rust_Unnamed_4::NUM_STAR_COLORS.0 as ::core::ffi::c_int
                        + ((star_type as ::core::ffi::c_int
                            & ((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int)
                                as BYTE as ::core::ffi::c_int) >> 5 as ::core::ffi::c_int)
                            as BYTE as ::core::ffi::c_int) as SIZE,
                );
            } else {
                s.frame = SetRelFrameIndex(
                    star_frame,
                    (((star_type as ::core::ffi::c_int
                        & ((0xff as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                            & !(((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int)
                                as BYTE as ::core::ffi::c_int)) as BYTE
                            as ::core::ffi::c_int) >> 3 as ::core::ffi::c_int) as BYTE
                        as ::core::ffi::c_int
                        * C2Rust_Unnamed_4::NUM_STAR_COLORS.0 as ::core::ffi::c_int
                        + ((star_type as ::core::ffi::c_int
                            & ((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int)
                                as BYTE as ::core::ffi::c_int) >> 5 as ::core::ffi::c_int)
                            as BYTE as ::core::ffi::c_int) as SIZE,
                );
            }
            i_2 = i_2.wrapping_add(1);
        } else if (*SDPtr).star_pt.x as ::core::ffi::c_int == 6134 as ::core::ffi::c_int
            && (*SDPtr).star_pt.y as ::core::ffi::c_int == 5900 as ::core::ffi::c_int
        {
            s.frame = SetRelFrameIndex(
                star_frame,
                (C2Rust_Unnamed_3::SUPER_GIANT_STAR.0 as ::core::ffi::c_int
                    * C2Rust_Unnamed_4::NUM_STAR_COLORS.0 as ::core::ffi::c_int
                    + C2Rust_Unnamed_4::GREEN_BODY.0 as ::core::ffi::c_int) as SIZE,
            );
        } else {
            s.frame = SetRelFrameIndex(
                star_frame,
                (C2Rust_Unnamed_3::GIANT_STAR.0 as ::core::ffi::c_int
                    * C2Rust_Unnamed_4::NUM_STAR_COLORS.0 as ::core::ffi::c_int
                    + C2Rust_Unnamed_4::GREEN_BODY.0 as ::core::ffi::c_int) as SIZE,
            );
        }
        uqm_lua_checked!(DrawStamp(&raw mut s), ());
        SDPtr = SDPtr.offset(1);
        if !((*SDPtr).star_pt.x as ::core::ffi::c_int <= 9999 as ::core::ffi::c_int
            && (*SDPtr).star_pt.y as ::core::ffi::c_int <= 9999 as ::core::ffi::c_int)
        {
            break;
        }
    }
    if uqm_lua_checked!(getGameStateUint(b"ARILOU_SPACE\0".as_ptr() as *const ::core::ffi::c_char), ()) != 0 {
        if which_space as ::core::ffi::c_int <= 1 as ::core::ffi::c_int {
            s.origin.x = ((universeToDispx(
                plot_map[C2Rust_Unnamed_5::ARILOU_DEFINED.0 as ::core::ffi::c_int
                        as usize]
                    .star_pt
                    .x as ::core::ffi::c_long,
            ) as ::core::ffi::c_int) << resolutionFactor) as COORD;
            s.origin.y = ((universeToDispy(
                plot_map[C2Rust_Unnamed_5::ARILOU_DEFINED.0 as ::core::ffi::c_int
                        as usize]
                    .star_pt
                    .y as ::core::ffi::c_long,
            ) as ::core::ffi::c_int) << resolutionFactor) as COORD;
        } else {
            s.origin.x = ((universeToDispx(5000 as ::core::ffi::c_long)
                as ::core::ffi::c_int) << resolutionFactor) as COORD;
            s.origin.y = ((universeToDispy(5000 as ::core::ffi::c_long)
                as ::core::ffi::c_int) << resolutionFactor) as COORD;
        }
        s.frame = SetRelFrameIndex(
            star_frame,
            (C2Rust_Unnamed_3::GIANT_STAR.0 as ::core::ffi::c_int
                * C2Rust_Unnamed_4::NUM_STAR_COLORS.0 as ::core::ffi::c_int
                + C2Rust_Unnamed_4::GREEN_BODY.0 as ::core::ffi::c_int) as SIZE,
        );
        uqm_lua_checked!(DrawStamp(&raw mut s), ());
    }
    if race_update as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        && GlobData.Game_state.autopilot.x as ::core::ffi::c_int
            != !(0 as ::core::ffi::c_int)
        && GlobData.Game_state.autopilot.y as ::core::ffi::c_int
            != !(0 as ::core::ffi::c_int)
    {
        uqm_lua_checked!(DrawAutoPilot(&raw mut GlobData.Game_state.autopilot), ());
        if if resolutionFactor != 2 as ::core::ffi::c_uint {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } != 0
        {
            uqm_lua_checked!(DrawMarker(
                GlobData.Game_state.autopilot,
                BOOLEAN::FALSE.0 as ::core::ffi::c_int as BYTE,
            ), ());
        }
    }
    if transition_pending.0 != 0 {
        GetContextClipRect(&raw mut r);
        uqm_lua_checked!(ScreenTransition(optScrTrans, &raw mut r), ());
        transition_pending = BOOLEAN::FALSE;
    }
    if !pClipRect.is_null() {
        SetContextClipRect(&raw mut old_r);
        SetContextOrigin(oldOrigin);
    }
    if race_update as ::core::ffi::c_int == 0 as ::core::ffi::c_int && draw_cursor.0 != 0
    {
        GetContextClipRect(&raw mut r);
        LoadIntoExtraScreen(&raw mut r);
        uqm_lua_checked!(DrawCursor(
            ((universeToDispx(cursorLoc.x as ::core::ffi::c_long) as ::core::ffi::c_int)
                << resolutionFactor) as COORD,
            ((universeToDispy(cursorLoc.y as ::core::ffi::c_long) as ::core::ffi::c_int)
                << resolutionFactor) as COORD,
        ), ());
        uqm_lua_checked!(flashCurrentLocation(::core::ptr::null_mut::<POINT>(), BOOLEAN::TRUE), ());
    }
    UnbatchGraphics();
}
unsafe extern "C" fn EraseCursor(mut curs_x: COORD, mut curs_y: COORD) {
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    GetFrameRect(StarMapFrame, &raw mut r);
    r.corner.x = (r.corner.x as ::core::ffi::c_int + curs_x as ::core::ffi::c_int)
        as COORD;
    if (r.corner.x as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
        r.extent.width = (r.extent.width as ::core::ffi::c_int
            + r.corner.x as ::core::ffi::c_int) as COORD;
        r.corner.x = 0 as COORD;
    } else if r.corner.x as ::core::ffi::c_int + r.extent.width as ::core::ffi::c_int
        >= CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
    {
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
                } << resolutionFactor)) - r.corner.x as ::core::ffi::c_int) as COORD;
    }
    r.corner.y = (r.corner.y as ::core::ffi::c_int + curs_y as ::core::ffi::c_int)
        as COORD;
    if (r.corner.y as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
        r.extent.height = (r.extent.height as ::core::ffi::c_int
            + r.corner.y as ::core::ffi::c_int) as COORD;
        r.corner.y = 0 as COORD;
    } else if r.corner.y as ::core::ffi::c_int + r.extent.height as ::core::ffi::c_int
        >= CanvasHeight
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
    }
    RepairBackRect(&raw mut r);
}
unsafe extern "C" fn ZoomStarMap(mut dir: SIZE) {
    if dir as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
        if zoomLevel < 4 as ::core::ffi::c_int {
            zoomLevel += 1;
            mapOrigin = cursorLoc;
            uqm_lua_checked!(DrawStarMap(0 as COUNT, ::core::ptr::null_mut::<RECT>()), ());
            uqm_lua_checked!(SleepThread(
                (840 as ::core::ffi::c_int / 8 as ::core::ffi::c_int) as TimePeriod,
            ), ());
        }
    } else if (dir as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
        if zoomLevel > 0 as ::core::ffi::c_int {
            if zoomLevel > 1 as ::core::ffi::c_int {
                mapOrigin = cursorLoc;
            } else {
                mapOrigin.x = (9999 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
                    as COORD;
                mapOrigin.y = (9999 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
                    as COORD;
            }
            zoomLevel -= 1;
            uqm_lua_checked!(DrawStarMap(0 as COUNT, ::core::ptr::null_mut::<RECT>()), ());
            uqm_lua_checked!(SleepThread(
                (840 as ::core::ffi::c_int / 8 as ::core::ffi::c_int) as TimePeriod,
            ), ());
        }
    }
}
unsafe extern "C" fn UpdateCursorLocation(
    mut sx: ::core::ffi::c_int,
    mut sy: ::core::ffi::c_int,
    mut newpt: *const POINT,
) {
    let mut s: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    let mut pt: POINT = POINT { x: 0, y: 0 };
    pt.x = universeToDispx(cursorLoc.x as ::core::ffi::c_long);
    pt.y = universeToDispy(cursorLoc.y as ::core::ffi::c_long);
    if !newpt.is_null() {
        sy = 0 as ::core::ffi::c_int;
        sx = sy;
        s.origin.x = universeToDispx((*newpt).x as ::core::ffi::c_long);
        s.origin.y = universeToDispy((*newpt).y as ::core::ffi::c_long);
        cursorLoc = *newpt;
    } else {
        s.origin.x = (pt.x as ::core::ffi::c_int + sx) as COORD;
        s.origin.y = (pt.y as ::core::ffi::c_int + sy) as COORD;
    }
    if sx != 0 {
        cursorLoc.x = (dispxToUniverse(s.origin.x) as ::core::ffi::c_int - sx) as COORD;
        while universeToDispx(cursorLoc.x as ::core::ffi::c_long) as ::core::ffi::c_int
            == pt.x as ::core::ffi::c_int
        {
            cursorLoc.x = (cursorLoc.x as ::core::ffi::c_int + sx) as COORD;
        }
        if (cursorLoc.x as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
            cursorLoc.x = 0 as COORD;
        } else if cursorLoc.x as ::core::ffi::c_int > 9999 as ::core::ffi::c_int {
            cursorLoc.x = 9999 as COORD;
        }
        s.origin.x = universeToDispx(cursorLoc.x as ::core::ffi::c_long);
    }
    if sy != 0 {
        cursorLoc.y = (dispyToUniverse(s.origin.y) as ::core::ffi::c_int + sy) as COORD;
        while universeToDispy(cursorLoc.y as ::core::ffi::c_long) as ::core::ffi::c_int
            == pt.y as ::core::ffi::c_int
        {
            cursorLoc.y = (cursorLoc.y as ::core::ffi::c_int - sy) as COORD;
        }
        if (cursorLoc.y as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
            cursorLoc.y = 0 as COORD;
        } else if cursorLoc.y as ::core::ffi::c_int > 9999 as ::core::ffi::c_int {
            cursorLoc.y = 9999 as COORD;
        }
        s.origin.y = universeToDispy(cursorLoc.y as ::core::ffi::c_long);
    }
    if (s.origin.x as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
        || (s.origin.y as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
        || s.origin.x as ::core::ffi::c_int
            >= CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
        || s.origin.y as ::core::ffi::c_int
            >= CanvasHeight
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
                - ((13 as ::core::ffi::c_int) << resolutionFactor) >> resolutionFactor
    {
        mapOrigin = cursorLoc;
        uqm_lua_checked!(DrawStarMap(0 as COUNT, ::core::ptr::null_mut::<RECT>()), ());
        s.origin.x = universeToDispx(cursorLoc.x as ::core::ffi::c_long);
        s.origin.y = universeToDispy(cursorLoc.y as ::core::ffi::c_long);
    } else {
        BatchGraphics();
        EraseCursor(
            ((pt.x as ::core::ffi::c_int) << resolutionFactor) as COORD,
            ((pt.y as ::core::ffi::c_int) << resolutionFactor) as COORD,
        );
        uqm_lua_checked!(DrawCursor(
            ((s.origin.x as ::core::ffi::c_int) << resolutionFactor) as COORD,
            ((s.origin.y as ::core::ffi::c_int) << resolutionFactor) as COORD,
        ), ());
        uqm_lua_checked!(flashCurrentLocation(::core::ptr::null_mut::<POINT>(), BOOLEAN::TRUE), ());
        UnbatchGraphics();
    };
}
#[no_mangle]
pub unsafe extern "C" fn starIndex(mut starPt: POINT) -> ::core::ffi::c_int {
    let mut i: COUNT = 0;
    i = 0 as COUNT;
    while i as ::core::ffi::c_int <= 502 as ::core::ffi::c_int {
        if (*(&raw mut star_array as *mut STAR_DESC).offset(i as isize)).star_pt.x
            as ::core::ffi::c_int == starPt.x as ::core::ffi::c_int
            && (*(&raw mut star_array as *mut STAR_DESC).offset(i as isize)).star_pt.y
                as ::core::ffi::c_int == starPt.y as ::core::ffi::c_int
        {
            break;
        }
        i = i.wrapping_add(1);
    }
    return i as ::core::ffi::c_int;
}
unsafe extern "C" fn UpdateCursorInfo(mut prevbuf: *mut UNICODE) {
    let mut buf: [UNICODE; 256] = ::core::mem::transmute::<
        [u8; 256],
        [UNICODE; 256],
    >(
        *b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    );
    let mut pt: POINT = POINT { x: 0, y: 0 };
    let mut SDPtr: *mut STAR_DESC = ::core::ptr::null_mut::<STAR_DESC>();
    let mut BestSDPtr: *mut STAR_DESC = ::core::ptr::null_mut::<STAR_DESC>();
    if which_starmap.0 == CURRENT_STARMAP_SHOWN::NORMAL_STARMAP.0 {
        utf8StringCopy(
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[UNICODE; 256]>(),
            GetStringAddress(
                SetAbsStringTableIndex(
                    GameStrings,
                    (C2Rust_Unnamed_9::FEEDBACK_STRING_BASE.0 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int
                        + (is3DO(optWhichFonts).0 != 0
                            || if optWindowType == 1 as DWORD {
                                BOOLEAN::TRUE.0 as ::core::ffi::c_int
                            } else {
                                BOOLEAN::FALSE.0 as ::core::ffi::c_int
                            } != 0) as ::core::ffi::c_int) as COUNT,
                ),
            ),
        );
    } else {
        utf8StringCopy(
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[UNICODE; 256]>(),
            GetStringAddress(
                SetAbsStringTableIndex(
                    GameStrings,
                    ((C2Rust_Unnamed_9::FEEDBACK_STRING_BASE.0 as ::core::ffi::c_int
                        + 3 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                        .wrapping_add(which_starmap.0) as COUNT,
                ),
            ),
        );
    }
    pt.x = ((universeToDispx(cursorLoc.x as ::core::ffi::c_long) as ::core::ffi::c_int)
        << resolutionFactor) as COORD;
    pt.y = ((universeToDispy(cursorLoc.y as ::core::ffi::c_long) as ::core::ffi::c_int)
        << resolutionFactor) as COORD;
    BestSDPtr = ::core::ptr::null_mut::<STAR_DESC>();
    SDPtr = BestSDPtr;
    loop {
        SDPtr = uqm_lua_checked!(FindStar(SDPtr, &raw mut cursorLoc, 75 as SIZE, 75 as SIZE), ());
        if SDPtr.is_null() {
            break;
        }
        if (universeToDispx((*SDPtr).star_pt.x as ::core::ffi::c_long)
            as ::core::ffi::c_int) << resolutionFactor == pt.x as ::core::ffi::c_int
            && (universeToDispy((*SDPtr).star_pt.y as ::core::ffi::c_long)
                as ::core::ffi::c_int) << resolutionFactor == pt.y as ::core::ffi::c_int
            && (BestSDPtr.is_null()
                || (((*SDPtr).Type as ::core::ffi::c_int
                    & ((0xff as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                        & !(((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int)
                            as BYTE as ::core::ffi::c_int)) as BYTE
                        as ::core::ffi::c_int) >> 3 as ::core::ffi::c_int) as BYTE
                    as ::core::ffi::c_int
                    >= (((*BestSDPtr).Type as ::core::ffi::c_int
                        & ((0xff as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                            & !(((0xff as ::core::ffi::c_int) << 5 as ::core::ffi::c_int)
                                as BYTE as ::core::ffi::c_int)) as BYTE
                            as ::core::ffi::c_int) >> 3 as ::core::ffi::c_int) as BYTE
                        as ::core::ffi::c_int)
        {
            BestSDPtr = SDPtr;
        }
    }
    if !BestSDPtr.is_null() {
        let mut whichPortal: BYTE = ((*BestSDPtr).Postfix as ::core::ffi::c_int
            - 133 as ::core::ffi::c_int) as BYTE;
        cursorLoc = (*BestSDPtr).star_pt;
        if uqm_lua_checked!(getGameStateUint(
            b"ARILOU_SPACE_SIDE\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) >= 2 as DWORD
            && uqm_lua_checked!(getGameStateUint(
                b"KNOW_QS_PORTAL\0".as_ptr() as *const ::core::ffi::c_char,
            ), ()) & ((1 as ::core::ffi::c_int) << whichPortal as ::core::ffi::c_int) as DWORD
                == 0
        {
            utf8StringCopy(
                &raw mut buf as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[UNICODE; 256]>(),
                GetStringAddress(
                    SetAbsStringTableIndex(
                        GameStrings,
                        (C2Rust_Unnamed_9::STAR_STRING_BASE.0 as ::core::ffi::c_int
                            + 132 as ::core::ffi::c_int) as COUNT,
                    ),
                ),
            );
        } else if uqm_lua_checked!(getGameStateUint(
            b"ARILOU_SPACE_SIDE\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) < 2 as DWORD || whichPortal as ::core::ffi::c_int == 15 as ::core::ffi::c_int
        {
            GetClusterName(BestSDPtr, &raw mut buf as *mut UNICODE);
        } else {
            let mut starnameBuf: [UNICODE; 256] = ::core::mem::transmute::<
                [u8; 256],
                [UNICODE; 256],
            >(
                *b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            );
            utf8StringCopy(
                &raw mut starnameBuf as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[UNICODE; 256]>(),
                GetStringAddress(
                    SetAbsStringTableIndex(
                        GameStrings,
                        (*portal_map[whichPortal as usize].nearest_star).Postfix as COUNT,
                    ),
                ),
            );
            if strlen(&raw mut starnameBuf as *mut UNICODE) > 11 as size_t {
                let mut i: COUNT = 0;
                i = 9 as COUNT;
                while i as ::core::ffi::c_int > 5 as ::core::ffi::c_int {
                    if starnameBuf[i as usize] as ::core::ffi::c_int
                        != 'a' as ::core::ffi::c_int
                        && starnameBuf[i as usize] as ::core::ffi::c_int
                            != 'e' as ::core::ffi::c_int
                        && starnameBuf[i as usize] as ::core::ffi::c_int
                            != 'i' as ::core::ffi::c_int
                        && starnameBuf[i as usize] as ::core::ffi::c_int
                            != 'o' as ::core::ffi::c_int
                        && starnameBuf[i as usize] as ::core::ffi::c_int
                            != 'u' as ::core::ffi::c_int
                        && starnameBuf[i as usize] as ::core::ffi::c_int
                            != 'y' as ::core::ffi::c_int
                    {
                        break;
                    }
                    i = i.wrapping_sub(1);
                }
                starnameBuf[(i as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                    as usize] = '.' as UNICODE;
                starnameBuf[(i as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
                    as usize] = '\0' as UNICODE;
            }
            snprintf(
                &raw mut buf as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[UNICODE; 256]>(),
                b"[%c] To %05.1f : %05.1f (Near %.12s)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                'A' as ::core::ffi::c_int + whichPortal as ::core::ffi::c_int,
                (portal_map[whichPortal as usize].star_pt.x as ::core::ffi::c_float
                    / 10 as ::core::ffi::c_int as ::core::ffi::c_float)
                    as ::core::ffi::c_double,
                (portal_map[whichPortal as usize].star_pt.y as ::core::ffi::c_float
                    / 10 as ::core::ffi::c_int as ::core::ffi::c_float)
                    as ::core::ffi::c_double,
                &raw mut starnameBuf as *mut UNICODE,
            );
        }
    } else {
        let mut bucket: COORD = 0;
        bucket = dispxToUniverse(
            (pt.x as ::core::ffi::c_int >> resolutionFactor) as COORD,
        );
        if (bucket as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
            cursorLoc.x = 0 as COORD;
        } else if bucket as ::core::ffi::c_int > 9999 as ::core::ffi::c_int {
            cursorLoc.x = 9999 as COORD;
        }
        bucket = dispyToUniverse(
            (pt.y as ::core::ffi::c_int >> resolutionFactor) as COORD,
        );
        if (bucket as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
            cursorLoc.y = 0 as COORD;
        } else if bucket as ::core::ffi::c_int > 9999 as ::core::ffi::c_int {
            cursorLoc.y = 9999 as COORD;
        }
    }
    if uqm_lua_checked!(getGameStateUint(b"ARILOU_SPACE\0".as_ptr() as *const ::core::ffi::c_char), ()) != 0 {
        let mut ari_pt: POINT = POINT { x: 0, y: 0 };
        if uqm_lua_checked!(getGameStateUint(
            b"ARILOU_SPACE_SIDE\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) <= 1 as DWORD
        {
            ari_pt = plot_map[C2Rust_Unnamed_5::ARILOU_DEFINED.0 as ::core::ffi::c_int
                    as usize]
                .star_pt;
        } else {
            ari_pt.x = 5000 as COORD;
            ari_pt.y = 5000 as COORD;
        }
        if (universeToDispx(ari_pt.x as ::core::ffi::c_long) as ::core::ffi::c_int)
            << resolutionFactor == pt.x as ::core::ffi::c_int
            && (universeToDispy(ari_pt.y as ::core::ffi::c_long) as ::core::ffi::c_int)
                << resolutionFactor == pt.y as ::core::ffi::c_int
        {
            cursorLoc = ari_pt;
            utf8StringCopy(
                &raw mut buf as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[UNICODE; 256]>(),
                GetStringAddress(
                    SetAbsStringTableIndex(
                        GameStrings,
                        (C2Rust_Unnamed_9::STAR_STRING_BASE.0 as ::core::ffi::c_int
                            + 132 as ::core::ffi::c_int) as COUNT,
                    ),
                ),
            );
        }
    }
    uqm_lua_checked!(DrawHyperCoords(cursorLoc), ());
    if strcmp(&raw mut buf as *mut UNICODE, prevbuf) != 0 as ::core::ffi::c_int {
        strcpy(prevbuf, &raw mut buf as *mut UNICODE);
        if !BestSDPtr.is_null() {
            if optShowVisitedStars.0 != 0
                && uqm_lua_checked!(isStarMarked(
                        starIndex((*BestSDPtr).star_pt),
                        b"VISITED\0".as_ptr() as *const ::core::ffi::c_char,
                    ), ())
                    .0 != 0
            {
                let mut visBuf: [UNICODE; 256] = ::core::mem::transmute::<
                    [u8; 256],
                    [UNICODE; 256],
                >(
                    *b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                );
                utf8StringCopy(
                    &raw mut visBuf as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[UNICODE; 256]>(),
                    &raw mut buf as *mut UNICODE,
                );
                snprintf(
                    &raw mut buf as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[UNICODE; 256]>(),
                    b"%c %.251s %c\0".as_ptr() as *const ::core::ffi::c_char,
                    '(' as ::core::ffi::c_int,
                    &raw mut visBuf as *mut UNICODE,
                    ')' as ::core::ffi::c_int,
                );
            }
            uqm_lua_checked!(DrawSISMessage(&raw mut buf as *mut UNICODE), ());
        } else if uqm_lua_checked!(getGameStateUint(
            b"ARILOU_SPACE_SIDE\0".as_ptr() as *const ::core::ffi::c_char,
        ), ()) <= 1 as DWORD
        {
            let mut OldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
            OldContext = SetContext(OffScreenContext);
            if which_starmap.0 == CURRENT_STARMAP_SHOWN::WAR_ERA_STARMAP.0 {
                SetContextForeGroundColor(
                    colorFromRgb15(0x18 as BYTE, 0 as BYTE, 0 as BYTE),
                );
            } else {
                SetContextForeGroundColor(
                    colorFromRgb15(0xe as BYTE, 0xa7 as BYTE, 0xd9 as BYTE),
                );
            }
            uqm_lua_checked!(DrawSISMessageEx(
                &raw mut buf as *mut UNICODE,
                -1 as SIZE,
                -1 as SIZE,
                ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as COUNT,
            ), ());
            SetContext(OldContext);
        } else {
            strcpy(
                &raw mut buf as *mut ::core::ffi::c_char,
                GetStringAddress(
                    SetAbsStringTableIndex(
                        GameStrings,
                        (C2Rust_Unnamed_9::NAVIGATION_STRING_BASE.0 as ::core::ffi::c_int
                            + 1 as ::core::ffi::c_int) as COUNT,
                    ),
                ),
            );
            uqm_lua_checked!(DrawSISMessage(&raw mut buf as *mut UNICODE), ());
        }
    }
}
unsafe extern "C" fn FuelRequired() -> ::core::ffi::c_uint {
    return uqm_lua_checked!(FuelRequiredTo(cursorLoc), 0);
}
unsafe extern "C" fn UpdateFuelRequirement() {
    let mut buf: [UNICODE; 80] = [0; 80];
    let mut fuel_required: COUNT = uqm_lua_checked!(FuelRequired(), ()) as COUNT;
    sprintf(
        &raw mut buf as *mut ::core::ffi::c_char,
        b"%s %u.%u\0".as_ptr() as *const ::core::ffi::c_char,
        GetStringAddress(
            SetAbsStringTableIndex(
                GameStrings,
                (C2Rust_Unnamed_9::NAVIGATION_STRING_BASE.0 as ::core::ffi::c_int
                    + 4 as ::core::ffi::c_int) as COUNT,
            ),
        ),
        fuel_required as ::core::ffi::c_int / 100 as ::core::ffi::c_int,
        fuel_required as ::core::ffi::c_int % 100 as ::core::ffi::c_int
            / 10 as ::core::ffi::c_int,
    );
    uqm_lua_checked!(DrawStatusMessage(&raw mut buf as *mut UNICODE), ());
}
unsafe extern "C" fn compStarName(
    mut ptr1: *const ::core::ffi::c_void,
    mut ptr2: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut index1: ::core::ffi::c_int = 0;
    let mut index2: ::core::ffi::c_int = 0;
    index1 = *(ptr1 as *const ::core::ffi::c_int);
    index2 = *(ptr2 as *const ::core::ffi::c_int);
    if (*(&raw mut star_array as *mut STAR_DESC).offset(index1 as isize)).Postfix
        as ::core::ffi::c_int
        != (*(&raw mut star_array as *mut STAR_DESC).offset(index2 as isize)).Postfix
            as ::core::ffi::c_int
    {
        return utf8StringCompare(
            GetStringAddress(
                SetAbsStringTableIndex(
                    GameStrings,
                    (*(&raw mut star_array as *mut STAR_DESC).offset(index1 as isize))
                        .Postfix as COUNT,
                ),
            ),
            GetStringAddress(
                SetAbsStringTableIndex(
                    GameStrings,
                    (*(&raw mut star_array as *mut STAR_DESC).offset(index2 as isize))
                        .Postfix as COUNT,
                ),
            ),
        );
    }
    if ((*(&raw mut star_array as *mut STAR_DESC).offset(index1 as isize)).Prefix
        as ::core::ffi::c_int)
        < (*(&raw mut star_array as *mut STAR_DESC).offset(index2 as isize)).Prefix
            as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    if (*(&raw mut star_array as *mut STAR_DESC).offset(index1 as isize)).Prefix
        as ::core::ffi::c_int
        > (*(&raw mut star_array as *mut STAR_DESC).offset(index2 as isize)).Prefix
            as ::core::ffi::c_int
    {
        return 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn SortStarsOnName(mut pSS: *mut STAR_SEARCH_STATE) {
    let mut i: ::core::ffi::c_int = 0;
    let mut sorted: *mut ::core::ffi::c_int = &raw mut (*pSS).SortedStars
        as *mut ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < 502 as ::core::ffi::c_int {
        *sorted.offset(i as isize) = i;
        i += 1;
    }
    qsort(
        sorted as *mut ::core::ffi::c_void,
        502 as size_t,
        ::core::mem::size_of::<::core::ffi::c_int>(),
        Some(
            compStarName
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_void,
                    *const ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
    );
}
unsafe extern "C" fn SplitStarName(mut pSS: *mut STAR_SEARCH_STATE) {
    let mut buf: *mut UNICODE = &raw mut (*pSS).Buffer as *mut UNICODE;
    let mut next: *mut UNICODE = ::core::ptr::null_mut::<UNICODE>();
    let mut sep: *mut UNICODE = ::core::ptr::null_mut::<UNICODE>();
    (*pSS).Prefix = ::core::ptr::null::<UNICODE>();
    (*pSS).PrefixLen = 0 as ::core::ffi::c_int;
    (*pSS).Cluster = ::core::ptr::null::<UNICODE>();
    (*pSS).ClusterLen = 0 as ::core::ffi::c_int;
    (*pSS).ClusterPos = 0 as ::core::ffi::c_int;
    next = buf;
    while *next as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        && getCharFromString(&raw mut next as *mut *const ::core::ffi::c_char)
            == (if isPC(optWhichFonts).0 != 0 {
                0x20 as ::core::ffi::c_int
            } else {
                0x9 as ::core::ffi::c_int
            }) as UniChar
    {
        buf = next;
    }
    if *buf as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
        return;
    }
    (*pSS).Prefix = buf;
    buf = next;
    while *next as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        && getCharFromString(&raw mut next as *mut *const ::core::ffi::c_char)
            != (if isPC(optWhichFonts).0 != 0 {
                0x20 as ::core::ffi::c_int
            } else {
                0x9 as ::core::ffi::c_int
            }) as UniChar
    {
        buf = next;
    }
    if *buf as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        sep = buf;
        buf = next;
        while *next as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
            && getCharFromString(&raw mut next as *mut *const ::core::ffi::c_char)
                == (if isPC(optWhichFonts).0 != 0 {
                    0x20 as ::core::ffi::c_int
                } else {
                    0x9 as ::core::ffi::c_int
                }) as UniChar
        {
            buf = next;
        }
    }
    if *buf as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
        (*pSS).Cluster = (*pSS).Prefix;
        (*pSS).ClusterLen = utf8StringCount((*pSS).Cluster) as ::core::ffi::c_int;
        (*pSS).ClusterPos = utf8StringCountN(
            &raw mut (*pSS).Buffer as *mut UNICODE,
            (*pSS).Cluster,
        ) as ::core::ffi::c_int;
        (*pSS).Prefix = ::core::ptr::null::<UNICODE>();
        return;
    }
    (*pSS).Cluster = buf;
    (*pSS).ClusterLen = utf8StringCount((*pSS).Cluster) as ::core::ffi::c_int;
    (*pSS).ClusterPos = utf8StringCountN(
        &raw mut (*pSS).Buffer as *mut UNICODE,
        (*pSS).Cluster,
    ) as ::core::ffi::c_int;
    *sep = '\0' as UNICODE;
    (*pSS).PrefixLen = utf8StringCount((*pSS).Prefix) as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn SkipStarCluster(
    mut sortedStars: *mut ::core::ffi::c_int,
    mut istar: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut Postfix: ::core::ffi::c_int = (*(&raw mut star_array as *mut STAR_DESC)
        .offset(*sortedStars.offset(istar as isize) as isize))
        .Postfix as ::core::ffi::c_int;
    istar += 1;
    while istar < 502 as ::core::ffi::c_int
        && (*(&raw mut star_array as *mut STAR_DESC)
            .offset(*sortedStars.offset(istar as isize) as isize))
            .Postfix as ::core::ffi::c_int == Postfix
    {
        istar += 1;
    }
    return istar;
}
unsafe extern "C" fn FindNextStarIndex(
    mut pSS: *mut STAR_SEARCH_STATE,
    mut from: ::core::ffi::c_int,
    mut WithinClust: BOOLEAN,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    if (*pSS).Cluster.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    i = from;
    while i < 502 as ::core::ffi::c_int {
        let mut SDPtr: *mut STAR_DESC = (&raw mut star_array as *mut STAR_DESC)
            .offset((*pSS).SortedStars[i as usize] as isize);
        let mut FullName: [UNICODE; 256] = [0; 256];
        let mut ClusterName: *mut UNICODE = GetStringAddress(
            SetAbsStringTableIndex(GameStrings, (*SDPtr).Postfix as COUNT),
        );
        let mut sptr: *const UNICODE = ::core::ptr::null::<UNICODE>();
        let mut dptr: *const UNICODE = ::core::ptr::null::<UNICODE>();
        let mut dlen: ::core::ffi::c_int = 0;
        let mut c: ::core::ffi::c_int = 0;
        dlen = utf8StringCount(ClusterName) as ::core::ffi::c_int;
        if (*pSS).ClusterLen > dlen {
            i = SkipStarCluster(
                &raw mut (*pSS).SortedStars as *mut ::core::ffi::c_int,
                i,
            ) - 1 as ::core::ffi::c_int;
        } else {
            c = 0 as ::core::ffi::c_int;
            sptr = (*pSS).Cluster;
            dptr = ClusterName;
            while c < (*pSS).ClusterLen {
                let mut sc: UniChar = getCharFromString(&raw mut sptr);
                let mut dc: UniChar = getCharFromString(&raw mut dptr);
                if UniChar_toUpper(sc) != UniChar_toUpper(dc) {
                    break;
                }
                c += 1;
            }
            if c < (*pSS).ClusterLen {
                i = SkipStarCluster(
                    &raw mut (*pSS).SortedStars as *mut ::core::ffi::c_int,
                    i,
                ) - 1 as ::core::ffi::c_int;
            } else if !(!(*pSS).Prefix.is_null() && (*SDPtr).Prefix == 0) {
                if WithinClust.0 != 0 {
                    break;
                }
                if (*pSS).Prefix.is_null() {
                    if i == 0 as ::core::ffi::c_int
                        || (*SDPtr).Postfix as ::core::ffi::c_int
                            != (*(&raw mut star_array as *mut STAR_DESC)
                                .offset(
                                    (*pSS).SortedStars[(i - 1 as ::core::ffi::c_int) as usize]
                                        as isize,
                                ))
                                .Postfix as ::core::ffi::c_int
                    {
                        break;
                    }
                    i = SkipStarCluster(
                        &raw mut (*pSS).SortedStars as *mut ::core::ffi::c_int,
                        i,
                    ) - 1 as ::core::ffi::c_int;
                } else {
                    GetClusterName(SDPtr, &raw mut FullName as *mut UNICODE);
                    dlen = utf8StringCount(&raw mut FullName as *mut UNICODE)
                        as ::core::ffi::c_int;
                    if (*pSS).PrefixLen <= dlen {
                        c = 0 as ::core::ffi::c_int;
                        sptr = (*pSS).Prefix;
                        dptr = &raw mut FullName as *mut UNICODE;
                        while c < (*pSS).PrefixLen {
                            let mut sc_0: UniChar = getCharFromString(&raw mut sptr);
                            let mut dc_0: UniChar = getCharFromString(&raw mut dptr);
                            if UniChar_toUpper(sc_0) != UniChar_toUpper(dc_0) {
                                break;
                            }
                            c += 1;
                        }
                        if c >= (*pSS).PrefixLen {
                            break;
                        }
                    }
                }
            }
        }
        i += 1;
    }
    return if i < 502 as ::core::ffi::c_int { i } else { -1 as ::core::ffi::c_int };
}
unsafe extern "C" fn DrawMatchedStarName(mut pTES: *mut TEXTENTRY_STATE) {
    let mut pSS: *mut STAR_SEARCH_STATE = (*pTES).CbParam as *mut STAR_SEARCH_STATE;
    let mut buf: [UNICODE; 256] = ::core::mem::transmute::<
        [u8; 256],
        [UNICODE; 256],
    >(
        *b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    );
    let mut ExPos: SIZE = 0 as SIZE;
    let mut CurPos: SIZE = -1 as SIZE;
    let mut SDPtr: *mut STAR_DESC = (&raw mut star_array as *mut STAR_DESC)
        .offset((*pSS).SortedStars[(*pSS).CurIndex as usize] as isize);
    let mut flags: COUNT = 0;
    if (*pSS).SingleClust.0 != 0 || (*pSS).SingleMatch.0 != 0 {
        GetClusterName(SDPtr, &raw mut buf as *mut UNICODE);
        ExPos = -1 as SIZE;
        flags = ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as COUNT;
    } else {
        let mut pstr: *mut UNICODE = &raw mut buf as *mut UNICODE;
        strcpy(pstr, &raw mut (*pSS).Text as *mut UNICODE);
        ExPos = (*pSS).ClusterPos as SIZE;
        pstr = skipUTF8Chars(pstr, (*pSS).ClusterPos as size_t) as *mut UNICODE;
        strcpy(
            pstr,
            GetStringAddress(
                SetAbsStringTableIndex(GameStrings, (*SDPtr).Postfix as COUNT),
            ),
        );
        ExPos = (ExPos as ::core::ffi::c_int + (*pSS).ClusterLen) as SIZE;
        CurPos = (*pTES).CursorPos as SIZE;
        flags = ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as COUNT;
        if (*pTES).JoystickMode.0 != 0 {
            flags = (flags as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as COUNT;
        }
    }
    uqm_lua_checked!(DrawSISMessageEx(&raw mut buf as *mut UNICODE, CurPos, ExPos, flags), ());
    uqm_lua_checked!(DrawHyperCoords(cursorLoc), ());
}
unsafe extern "C" fn MatchNextStar(mut pSS: *mut STAR_SEARCH_STATE, mut Reset: BOOLEAN) {
    if Reset.0 != 0 {
        (*pSS).FirstIndex = -1 as ::core::ffi::c_int;
    }
    if (*pSS).FirstIndex < 0 as ::core::ffi::c_int {
        (*pSS).CurIndex = -1 as ::core::ffi::c_int;
        (*pSS).LastIndex = -1 as ::core::ffi::c_int;
        (*pSS).SingleClust = BOOLEAN::FALSE;
        (*pSS).SingleMatch = BOOLEAN::FALSE;
        strcpy(
            &raw mut (*pSS).Buffer as *mut ::core::ffi::c_char,
            &raw mut (*pSS).Text as *mut UNICODE,
        );
        SplitStarName(pSS);
    }
    (*pSS).CurIndex = FindNextStarIndex(
        pSS,
        (*pSS).CurIndex + 1 as ::core::ffi::c_int,
        (*pSS).SingleClust,
    );
    if (*pSS).FirstIndex < 0 as ::core::ffi::c_int {
        (*pSS).FirstIndex = (*pSS).CurIndex;
    }
    if (*pSS).CurIndex >= 0 as ::core::ffi::c_int {
        (*pSS).LastIndex = (*pSS).CurIndex;
    } else {
        (*pSS).CurIndex = (*pSS).FirstIndex;
        if (*pSS).FirstIndex == (*pSS).LastIndex
            && (*pSS).FirstIndex != -1 as ::core::ffi::c_int
        {
            if (*pSS).Prefix.is_null() {
                (*pSS).SingleClust = BOOLEAN::TRUE;
            } else {
                (*pSS).SingleMatch = BOOLEAN::TRUE;
            }
        }
    };
}
unsafe extern "C" fn OnStarNameChange(mut pTES: *mut TEXTENTRY_STATE) -> BOOLEAN {
    let mut pSS: *mut STAR_SEARCH_STATE = (*pTES).CbParam as *mut STAR_SEARCH_STATE;
    let mut flags: COUNT = 0;
    let mut ret: BOOLEAN = BOOLEAN::TRUE;
    if strcmp(
        &raw mut (*pSS).Text as *mut UNICODE,
        &raw mut (*pSS).LastText as *mut UNICODE,
    ) != 0 as ::core::ffi::c_int
    {
        (*pSS).LastChangeTime = GetTimeCounter() as DWORD;
        strcpy(
            &raw mut (*pSS).LastText as *mut ::core::ffi::c_char,
            &raw mut (*pSS).Text as *mut UNICODE,
        );
        MatchNextStar(pSS, BOOLEAN::TRUE);
    }
    if (*pSS).CurIndex < 0 as ::core::ffi::c_int {
        if (*pSS).Text[0usize] as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
            flags = ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as COUNT;
        } else {
            flags = ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as COUNT;
        }
        if (*pTES).JoystickMode.0 != 0 {
            flags = (flags as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as COUNT;
        }
        ret = uqm_lua_checked!(DrawSISMessageEx(
            &raw mut (*pSS).Text as *mut UNICODE,
            (*pTES).CursorPos as SIZE,
            -1 as SIZE,
            flags,
        ), BOOLEAN(0));
    } else {
        let mut SDPtr: *mut STAR_DESC = ::core::ptr::null_mut::<STAR_DESC>();
        SDPtr = (&raw mut star_array as *mut STAR_DESC)
            .offset((*pSS).SortedStars[(*pSS).CurIndex as usize] as isize);
        uqm_lua_checked!(UpdateCursorLocation(
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            &raw mut (*SDPtr).star_pt,
        ), BOOLEAN(0));
        uqm_lua_checked!(DrawMatchedStarName(pTES), BOOLEAN(0));
        uqm_lua_checked!(UpdateFuelRequirement(), BOOLEAN(0));
    }
    return ret;
}
unsafe extern "C" fn OnStarNameFrame(mut pTES: *mut TEXTENTRY_STATE) -> BOOLEAN {
    let mut pSS: *mut STAR_SEARCH_STATE = (*pTES).CbParam as *mut STAR_SEARCH_STATE;
    if PulsedInputState
        .menu[C2Rust_Unnamed::KEY_MENU_NEXT.0 as ::core::ffi::c_int as usize] != 0
    {
        let mut SDPtr: *mut STAR_DESC = ::core::ptr::null_mut::<STAR_DESC>();
        MatchNextStar(pSS, BOOLEAN::FALSE);
        if (*pSS).CurIndex < 0 as ::core::ffi::c_int {
            if PulsedInputState
                .menu[C2Rust_Unnamed::KEY_MENU_NEXT.0 as ::core::ffi::c_int as usize]
                != 0
            {
                uqm_lua_checked!(PlayMenuSound(MENU_SOUND_EFFECT::MENU_SOUND_FAILURE), BOOLEAN(0));
            }
            return BOOLEAN::TRUE;
        }
        SDPtr = (&raw mut star_array as *mut STAR_DESC)
            .offset((*pSS).SortedStars[(*pSS).CurIndex as usize] as isize);
        uqm_lua_checked!(UpdateCursorLocation(
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            &raw mut (*SDPtr).star_pt,
        ), BOOLEAN(0));
        uqm_lua_checked!(DrawMatchedStarName(pTES), BOOLEAN(0));
        uqm_lua_checked!(UpdateFuelRequirement(), BOOLEAN(0));
    }
    uqm_lua_checked!(flashCurrentLocation(::core::ptr::null_mut::<POINT>(), BOOLEAN::FALSE), BOOLEAN(0));
    uqm_lua_checked!(SleepThread((840 as ::core::ffi::c_int / 30 as ::core::ffi::c_int) as TimePeriod), BOOLEAN(0));
    return BOOLEAN::TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn coords_only(mut s: *mut UNICODE) -> BOOLEAN {
    let mut i: BYTE = 0;
    let mut count: BYTE = 0 as BYTE;
    let mut countD: BYTE = 0 as BYTE;
    let mut countC: BYTE = 0 as BYTE;
    let mut j: BYTE = strlen(s) as BYTE;
    i = 0 as BYTE;
    while (i as ::core::ffi::c_int) < j as ::core::ffi::c_int {
        if *s.offset(i as isize) as ::core::ffi::c_int == '.' as ::core::ffi::c_int {
            count = count.wrapping_add(1);
            countD = countD.wrapping_add(1);
        } else if *s.offset(i as isize) as ::core::ffi::c_int
            == ':' as ::core::ffi::c_int
        {
            count = count.wrapping_add(1);
            countC = countC.wrapping_add(1);
        } else if isdigit(*s.offset(i as isize) as ::core::ffi::c_int)
            == 0 as ::core::ffi::c_int
        {
            return BOOLEAN::FALSE
        } else {
            count = count.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    return BOOLEAN(
        (i as ::core::ffi::c_int == j as ::core::ffi::c_int
            && countD as ::core::ffi::c_int <= 2 as ::core::ffi::c_int
            && countC as ::core::ffi::c_int == 1 as ::core::ffi::c_int)
            as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
}
unsafe extern "C" fn DoStarSearch(mut pMS: *mut MENU_STATE) -> BOOLEAN {
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
    let mut pss: *mut STAR_SEARCH_STATE = ::core::ptr::null_mut::<STAR_SEARCH_STATE>();
    let mut success: BOOLEAN = BOOLEAN::FALSE;
    pss = HMalloc(::core::mem::size_of::<STAR_SEARCH_STATE>()) as *mut STAR_SEARCH_STATE;
    if pss.is_null() {
        return BOOLEAN::FALSE;
    }
    uqm_lua_checked!(DrawSISMessageEx(
        b"\0".as_ptr() as *const UNICODE,
        0 as SIZE,
        0 as SIZE,
        ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as COUNT,
    ), BOOLEAN(0));
    TextEntry3DO = BOOLEAN(is3DO(optWhichFonts).0);
    (*pss).pMS = pMS;
    (*pss).LastChangeTime = 0 as DWORD;
    (*pss).Text[0usize] = '\0' as UNICODE;
    (*pss).LastText[0usize] = '\0' as UNICODE;
    (*pss).FirstIndex = -1 as ::core::ffi::c_int;
    SortStarsOnName(pss);
    tes.Initialized = BOOLEAN::FALSE;
    tes.BaseStr = &raw mut (*pss).Text as *mut UNICODE;
    tes.MaxSize = ::core::mem::size_of::<[UNICODE; 256]>() as ::core::ffi::c_int;
    tes.CursorPos = 0 as ::core::ffi::c_int;
    tes.CbParam = pss as *mut ::core::ffi::c_void;
    tes.ChangeCallback = Some(
        OnStarNameChange as unsafe extern "C" fn(*mut TEXTENTRY_STATE) -> BOOLEAN,
    ) as Option<unsafe extern "C" fn(*mut textentry_state) -> BOOLEAN>;
    tes.FrameCallback = Some(
        OnStarNameFrame as unsafe extern "C" fn(*mut TEXTENTRY_STATE) -> BOOLEAN,
    ) as Option<unsafe extern "C" fn(*mut textentry_state) -> BOOLEAN>;
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
    SetDefaultMenuRepeatDelay();
    success = uqm_lua_checked!(DoTextEntry(&raw mut tes), BOOLEAN(0));
    if coords_only(tes.BaseStr).0 != 0 {
        let mut coord: POINT = POINT { x: 0, y: 0 };
        coord.x = (atof(
            strtok(tes.BaseStr, b":\0".as_ptr() as *const ::core::ffi::c_char),
        ) * 10 as ::core::ffi::c_int as ::core::ffi::c_double) as COORD;
        coord.y = (atof(
            strtok(
                ::core::ptr::null_mut::<::core::ffi::c_char>(),
                b":\0".as_ptr() as *const ::core::ffi::c_char,
            ),
        ) * 10 as ::core::ffi::c_int as ::core::ffi::c_double) as COORD;
        if coord.x as ::core::ffi::c_int > 9999 as ::core::ffi::c_int
            || coord.y as ::core::ffi::c_int > 9999 as ::core::ffi::c_int
            || (coord.x as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
            || (coord.y as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
        {
            success = BOOLEAN::FALSE;
        } else {
            uqm_lua_checked!(UpdateCursorLocation(
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                &raw mut coord,
            ), BOOLEAN(0));
        }
        success = BOOLEAN::TRUE;
    }
    uqm_lua_checked!(DrawSISMessageEx(
        &raw mut (*pss).Text as *mut UNICODE,
        -1 as SIZE,
        -1 as SIZE,
        ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as COUNT,
    ), BOOLEAN(0));
    HFree(pss as *mut ::core::ffi::c_void);
    TextEntry3DO = BOOLEAN::FALSE;
    return success;
}
#[no_mangle]
pub unsafe extern "C" fn DoBubbleWarp(mut UseFuel: BOOLEAN) {
    uqm_lua_checked!(PlayMenuSound(MENU_SOUND_EFFECT::MENU_SOUND_BUBBLEWARP), ());
    if UseFuel.0 != 0 {
        uqm_lua_checked!(DeltaSISGauges(0 as SIZE, -(uqm_lua_checked!(FuelRequired(), ()) as SDWORD), 0 as ::core::ffi::c_int), ());
    }
    if GlobData.Game_state.CurrentActivity as BYTE as ::core::ffi::c_int
        == C2Rust_Unnamed_2::IN_INTERPLANETARY.0 as ::core::ffi::c_int
    {
        GlobData.Game_state.CurrentActivity = (GlobData.Game_state.CurrentActivity
            as ::core::ffi::c_int
            | C2Rust_Unnamed_2::START_INTERPLANETARY.0 as ::core::ffi::c_int)
            as ACTIVITY;
        ::core::ptr::write_volatile(
            &raw mut debugHook,
            Some(doInstantMove as unsafe extern "C" fn() -> ())
                as Option<unsafe extern "C" fn() -> ()>,
        );
    } else {
        uqm_lua_checked!(doInstantMove(), ());
    };
}
unsafe extern "C" fn AdvancedAutoPilot() {
    let mut current_position: POINT = POINT { x: 0, y: 0 };
    let mut destination: POINT = GlobData.Game_state.autopilot;
    let mut portal_coordinates: POINT = POINT { x: 0, y: 0 };
    let mut distance: ::core::ffi::c_double = 0.;
    let mut fuel_no_portal: ::core::ffi::c_double = 0.;
    let mut fuel_with_portal: ::core::ffi::c_double = 0.;
    let mut minimum: ::core::ffi::c_double = 0.0f64;
    let mut i: BYTE = 0;
    let mut index: BYTE = 0 as BYTE;
    let mut KnownQSPortals: UWORD = uqm_lua_checked!(getGameStateUint(
        b"KNOW_QS_PORTAL\0".as_ptr() as *const ::core::ffi::c_char,
    ), ()) as UWORD;
    current_position.x = logxToUniverse(GlobData.SIS_state.log_x) as COORD;
    current_position.y = logyToUniverse(GlobData.SIS_state.log_y) as COORD;
    if pointsEqual(current_position, destination) {
        return;
    }
    i = 0 as BYTE;
    while (i as ::core::ffi::c_int) < 15 as ::core::ffi::c_int {
        distance = ptDistance(destination, portal_map[i as usize].star_pt);
        if if if GlobData.SIS_state.Difficulty as ::core::ffi::c_int != 0 {
            GlobData.SIS_state.Difficulty as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        } == 1 as ::core::ffi::c_int
        {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        } == 0
            && KnownQSPortals as ::core::ffi::c_int
                & (1 as ::core::ffi::c_int) << i as ::core::ffi::c_int == 0
        {
            distance = (9999 as ::core::ffi::c_int * 9999 as ::core::ffi::c_int)
                as ::core::ffi::c_double;
        }
        if i as ::core::ffi::c_int == 0 as ::core::ffi::c_int || distance < minimum {
            minimum = distance;
            index = (i as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as BYTE;
        }
        i = i.wrapping_add(1);
    }
    portal_coordinates = (*(&raw mut star_array as *mut STAR_DESC)
        .offset((502 as ::core::ffi::c_int + index as ::core::ffi::c_int) as isize))
        .star_pt;
    fuel_no_portal = ptDistance(current_position, destination)
        / 100 as ::core::ffi::c_int as ::core::ffi::c_double;
    fuel_with_portal = minimum / 100 as ::core::ffi::c_int as ::core::ffi::c_double
        + (if if GlobData.SIS_state.Difficulty as ::core::ffi::c_int != 0 {
            GlobData.SIS_state.Difficulty as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        } == 0 as ::core::ffi::c_int
        {
            10 as ::core::ffi::c_int
        } else if if GlobData.SIS_state.Difficulty as ::core::ffi::c_int != 0 {
            GlobData.SIS_state.Difficulty as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        } == 1 as ::core::ffi::c_int
        {
            5 as ::core::ffi::c_int
        } else {
            20 as ::core::ffi::c_int
        }) as ::core::ffi::c_double;
    if fuel_no_portal < fuel_with_portal {
        return;
    }
    uqm_lua_checked!(SaveAdvancedAutoPilot(destination), ());
    uqm_lua_checked!(SaveAdvancedQuasiPilot(portal_coordinates), ());
    if playerInSolarSystem() {
        GlobData.Game_state.autopilot = current_position;
    }
    if uqm_lua_checked!(inHyperSpace(), ()).0 != 0 {
        uqm_lua_checked!(InvokeSpawner(), ());
    }
}
unsafe extern "C" fn DoMoveCursor(mut pMS: *mut MENU_STATE) -> BOOLEAN {
    static mut last_buf: [UNICODE; 256] = [0; 256];
    let mut TimeIn: DWORD = GetTimeCounter();
    static mut moveRepeats: COUNT = 0;
    let mut isMove: BOOLEAN = BOOLEAN::FALSE;
    if (*pMS).Initialized == 0 {
        let mut universe: POINT = POINT { x: 0, y: 0 };
        (*pMS).Initialized = BOOLEAN::TRUE.0 as ::core::ffi::c_int as SIZE;
        (*pMS).InputFunc = Some(
            DoMoveCursor as unsafe extern "C" fn(*mut MENU_STATE) -> BOOLEAN,
        ) as Option<unsafe extern "C" fn(*mut menu_state) -> BOOLEAN>;
        if inHQSpace().0 == 0 {
            universe = (*CurStarDescPtr).star_pt;
        } else {
            universe.x = logxToUniverse(GlobData.SIS_state.log_x) as COORD;
            universe.y = logyToUniverse(GlobData.SIS_state.log_y) as COORD;
        }
        uqm_lua_checked!(flashCurrentLocation(&raw mut universe, BOOLEAN::FALSE), BOOLEAN(0));
        last_buf[0usize] = '\0' as UNICODE;
        uqm_lua_checked!(UpdateCursorInfo(&raw mut last_buf as *mut UNICODE), BOOLEAN(0));
        uqm_lua_checked!(UpdateFuelRequirement(), BOOLEAN(0));
        return BOOLEAN::TRUE;
    } else if PulsedInputState
        .menu[C2Rust_Unnamed::KEY_MENU_CANCEL.0 as ::core::ffi::c_int as usize] != 0
    {
        FlushInput();
        if optBubbleWarp.0 != 0 && optInfiniteFuel.0 == 0 && uqm_lua_checked!(inQuasiSpace(), BOOLEAN(0)).0 == 0
            && GlobData.Game_state.autopilot.x as ::core::ffi::c_int
                != !(0 as ::core::ffi::c_int)
            && GlobData.Game_state.autopilot.y as ::core::ffi::c_int
                != !(0 as ::core::ffi::c_int)
            && GlobData.SIS_state.FuelOnBoard >= uqm_lua_checked!(FuelRequired(), BOOLEAN(0))
        {
            uqm_lua_checked!(DoBubbleWarp(BOOLEAN::TRUE), BOOLEAN(0));
        }
        if uqm_lua_checked!(inQuasiSpace(), BOOLEAN(0)).0 == 0 && ValidPoint(GlobData.Game_state.autopilot).0 != 0 {
            if optSmartAutoPilot.0 != 0 {
                uqm_lua_checked!(SaveLastLoc(
                    MAKE_POINT(
                        logxToUniverse(GlobData.SIS_state.log_x) as COORD,
                        logyToUniverse(GlobData.SIS_state.log_y) as COORD,
                    ),
                ), BOOLEAN(0));
            }
            if optAdvancedAutoPilot.0 != 0
                && uqm_lua_checked!(getGameStateUint(
                    b"PORTAL_SPAWNER_ON_SHIP\0".as_ptr() as *const ::core::ffi::c_char,
                ), BOOLEAN(0)) != 0
            {
                uqm_lua_checked!(AdvancedAutoPilot(), BOOLEAN(0));
            }
        }
        return BOOLEAN::FALSE;
    } else if PulsedInputState
        .menu[C2Rust_Unnamed::KEY_MENU_SELECT.0 as ::core::ffi::c_int as usize] != 0
    {
        FlushInput();
        if optBubbleWarp.0 != 0 && (optInfiniteFuel.0 != 0 || uqm_lua_checked!(inQuasiSpace(), BOOLEAN(0)).0 != 0) {
            GlobData.Game_state.autopilot = cursorLoc;
            uqm_lua_checked!(DoBubbleWarp(BOOLEAN::FALSE), BOOLEAN(0));
            return BOOLEAN::FALSE;
        } else {
            if GlobData.Game_state.autopilot.x as ::core::ffi::c_int
                == cursorLoc.x as ::core::ffi::c_int
                && GlobData.Game_state.autopilot.y as ::core::ffi::c_int
                    == cursorLoc.y as ::core::ffi::c_int
            {
                GlobData.Game_state.autopilot.y = !(0 as ::core::ffi::c_int) as COORD;
                GlobData.Game_state.autopilot.x = GlobData.Game_state.autopilot.y;
            } else {
                GlobData.Game_state.autopilot = cursorLoc;
            }
            uqm_lua_checked!(DrawStarMap(0 as COUNT, ::core::ptr::null_mut::<RECT>()), BOOLEAN(0));
        }
    } else if PulsedInputState
        .menu[C2Rust_Unnamed::KEY_MENU_SEARCH.0 as ::core::ffi::c_int as usize] != 0
    {
        FlushInput();
        if uqm_lua_checked!(getGameStateUint(
            b"ARILOU_SPACE_SIDE\0".as_ptr() as *const ::core::ffi::c_char,
        ), BOOLEAN(0)) <= 1 as DWORD
        {
            let mut oldpt: POINT = cursorLoc;
            if uqm_lua_checked!(DoStarSearch(pMS), BOOLEAN(0)).0 == 0 {
                uqm_lua_checked!(UpdateCursorLocation(
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    &raw mut oldpt,
                ), BOOLEAN(0));
            }
            FlushCursorRect();
            strcpy(
                &raw mut last_buf as *mut ::core::ffi::c_char,
                b"  <random garbage>  \0".as_ptr() as *const ::core::ffi::c_char,
            );
            uqm_lua_checked!(UpdateCursorInfo(&raw mut last_buf as *mut UNICODE), BOOLEAN(0));
            uqm_lua_checked!(UpdateFuelRequirement(), BOOLEAN(0));
            SetMenuRepeatDelay(
                (840 as ::core::ffi::c_int / 60 as ::core::ffi::c_int) as DWORD,
                (840 as ::core::ffi::c_int / 8 as ::core::ffi::c_int) as DWORD,
                (840 as ::core::ffi::c_int / 120 as ::core::ffi::c_int) as DWORD,
                BOOLEAN::TRUE,
            );
            SetMenuSounds(
                0 as ::core::ffi::c_int as MENU_SOUND_FLAGS,
                0 as ::core::ffi::c_int as MENU_SOUND_FLAGS,
            );
        } else {
            uqm_lua_checked!(PlayMenuSound(MENU_SOUND_EFFECT::MENU_SOUND_FAILURE), BOOLEAN(0));
        }
    } else if PulsedInputState
        .menu[C2Rust_Unnamed::KEY_MENU_TOGGLEMAP.0 as ::core::ffi::c_int as usize] != 0
    {
        FlushInput();
        if uqm_lua_checked!(getGameStateUint(
            b"ARILOU_SPACE_SIDE\0".as_ptr() as *const ::core::ffi::c_char,
        ), BOOLEAN(0)) <= 1 as DWORD
        {
            let mut NewState: BYTE = 0;
            NewState = which_starmap.0 as BYTE;
            if NewState as ::core::ffi::c_int
                == CURRENT_STARMAP_SHOWN::RAINBOW_MAP.0 as ::core::ffi::c_int
            {
                NewState = CURRENT_STARMAP_SHOWN::NORMAL_STARMAP.0 as ::core::ffi::c_int
                    as BYTE;
            } else {
                NewState = NewState.wrapping_add(1);
            }
            if NewState as ::core::ffi::c_uint != which_starmap.0 {
                which_starmap = CURRENT_STARMAP_SHOWN(NewState as ::core::ffi::c_uint);
            }
            uqm_lua_checked!(PlayMenuSound(MENU_SOUND_EFFECT::MENU_SOUND_MOVE), BOOLEAN(0));
            uqm_lua_checked!(DrawStarMap(0 as COUNT, ::core::ptr::null_mut::<RECT>()), BOOLEAN(0));
            last_buf[0usize] = '\0' as UNICODE;
            uqm_lua_checked!(UpdateCursorInfo(&raw mut last_buf as *mut UNICODE), BOOLEAN(0));
        } else {
            uqm_lua_checked!(PlayMenuSound(MENU_SOUND_EFFECT::MENU_SOUND_FAILURE), BOOLEAN(0));
        }
    } else if PulsedInputState
        .menu[C2Rust_Unnamed::KEY_MENU_SPECIAL.0 as ::core::ffi::c_int as usize] != 0
    {
        FlushInput();
        if uqm_lua_checked!(getGameStateUint(
            b"ARILOU_SPACE_SIDE\0".as_ptr() as *const ::core::ffi::c_char,
        ), BOOLEAN(0)) <= 1 as DWORD
        {
            uqm_lua_checked!(setStarMarked(
                starIndex(cursorLoc),
                b"PLYR_MARKER\0".as_ptr() as *const ::core::ffi::c_char,
            ), BOOLEAN(0));
            uqm_lua_checked!(DrawStarMap(0 as COUNT, ::core::ptr::null_mut::<RECT>()), BOOLEAN(0));
        } else {
            uqm_lua_checked!(PlayMenuSound(MENU_SOUND_EFFECT::MENU_SOUND_FAILURE), BOOLEAN(0));
        }
    } else if PulsedInputState
        .menu[C2Rust_Unnamed::KEY_MENU_DELETE.0 as ::core::ffi::c_int as usize] != 0
    {
        FlushInput();
        if uqm_lua_checked!(getGameStateUint(
            b"ARILOU_SPACE_SIDE\0".as_ptr() as *const ::core::ffi::c_char,
        ), BOOLEAN(0)) <= 1 as DWORD
        {
            let mut i: COUNT = 0;
            i = 0 as COUNT;
            while i as ::core::ffi::c_int <= 502 as ::core::ffi::c_int {
                if uqm_lua_checked!(isStarMarked(
                        i as ::core::ffi::c_int,
                        b"PLYR_MARKER\0".as_ptr() as *const ::core::ffi::c_char,
                    ), BOOLEAN(0))
                    .0 != 0
                {
                    uqm_lua_checked!(setStarMarked(
                        i as ::core::ffi::c_int,
                        b"PLYR_MARKER\0".as_ptr() as *const ::core::ffi::c_char,
                    ), BOOLEAN(0));
                    uqm_lua_checked!(DrawStarMap(0 as COUNT, ::core::ptr::null_mut::<RECT>()), BOOLEAN(0));
                    uqm_lua_checked!(SleepThread(
                        (840 as ::core::ffi::c_int / 8 as ::core::ffi::c_int)
                            as TimePeriod,
                    ), BOOLEAN(0));
                }
                i = i.wrapping_add(1);
            }
        } else {
            uqm_lua_checked!(PlayMenuSound(MENU_SOUND_EFFECT::MENU_SOUND_FAILURE), BOOLEAN(0));
        }
    } else {
        let mut sx: SBYTE = 0;
        let mut sy: SBYTE = 0;
        let mut ZoomIn: SIZE = 0;
        let mut ZoomOut: SIZE = 0;
        ZoomOut = 0 as SIZE;
        ZoomIn = ZoomOut;
        if PulsedInputState
            .menu[C2Rust_Unnamed::KEY_MENU_ZOOM_IN.0 as ::core::ffi::c_int as usize] != 0
        {
            ZoomIn = 1 as SIZE;
        } else if PulsedInputState
            .menu[C2Rust_Unnamed::KEY_MENU_ZOOM_OUT.0 as ::core::ffi::c_int as usize]
            != 0
        {
            ZoomOut = 1 as SIZE;
        }
        uqm_lua_checked!(ZoomStarMap(
            (ZoomIn as ::core::ffi::c_int - ZoomOut as ::core::ffi::c_int) as SIZE,
        ), BOOLEAN(0));
        sy = 0 as SBYTE;
        sx = sy;
        if PulsedInputState
            .menu[C2Rust_Unnamed::KEY_MENU_LEFT.0 as ::core::ffi::c_int as usize] != 0
        {
            sx = -1 as SBYTE;
        }
        if PulsedInputState
            .menu[C2Rust_Unnamed::KEY_MENU_RIGHT.0 as ::core::ffi::c_int as usize] != 0
        {
            sx = 1 as SBYTE;
        }
        if PulsedInputState
            .menu[C2Rust_Unnamed::KEY_MENU_UP.0 as ::core::ffi::c_int as usize] != 0
        {
            sy = -1 as SBYTE;
        }
        if PulsedInputState
            .menu[C2Rust_Unnamed::KEY_MENU_DOWN.0 as ::core::ffi::c_int as usize] != 0
        {
            sy = 1 as SBYTE;
        }
        if DirKeysPress().0 != 0
            && CurrentInputState
                .menu[C2Rust_Unnamed::KEY_MENU_NEXT.0 as ::core::ffi::c_int as usize]
                != 0
        {
            sx = (sx as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as SBYTE;
            sy = (sy as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as SBYTE;
        }
        if sx as ::core::ffi::c_int != 0 as ::core::ffi::c_int
            || sy as ::core::ffi::c_int != 0 as ::core::ffi::c_int
        {
            uqm_lua_checked!(UpdateCursorLocation(
                sx as ::core::ffi::c_int,
                sy as ::core::ffi::c_int,
                ::core::ptr::null::<POINT>(),
            ), BOOLEAN(0));
            uqm_lua_checked!(UpdateCursorInfo(&raw mut last_buf as *mut UNICODE), BOOLEAN(0));
            uqm_lua_checked!(UpdateFuelRequirement(), BOOLEAN(0));
            isMove = BOOLEAN::TRUE;
        }
        uqm_lua_checked!(SleepThreadUntil(
            TimeIn
                .wrapping_add(
                    (840 as ::core::ffi::c_int / 60 as ::core::ffi::c_int) as TimeCount,
                ),
        ), BOOLEAN(0));
    }
    if isMove.0 != 0 {
        moveRepeats = moveRepeats.wrapping_add(1);
    } else {
        moveRepeats = 0 as COUNT;
    }
    uqm_lua_checked!(flashCurrentLocation(::core::ptr::null_mut::<POINT>(), BOOLEAN::FALSE), BOOLEAN(0));
    return BOOLEAN(
        (GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
            & (C2Rust_Unnamed_2::CHECK_ABORT.0 as ::core::ffi::c_int
                | C2Rust_Unnamed_2::CHECK_LOAD.0 as ::core::ffi::c_int) == 0)
            as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
}
unsafe extern "C" fn RepairMap(
    mut update_race: COUNT,
    mut pLastRect: *mut RECT,
    mut pNextRect: *mut RECT,
) {
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    r.extent.width = 50 as COORD;
    r.corner.x = ((*pNextRect).corner.x as ::core::ffi::c_int
        + ((*pNextRect).extent.width as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
        - (r.extent.width as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)) as COORD;
    if (r.corner.x as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
        r.corner.x = 0 as COORD;
    } else if r.corner.x as ::core::ffi::c_int + r.extent.width as ::core::ffi::c_int
        >= CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
    {
        r.corner.x = (CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
                } << resolutionFactor)) - r.extent.width as ::core::ffi::c_int) as COORD;
    }
    r.extent.height = 9 as COORD;
    r.corner.y = ((*pNextRect).corner.y as ::core::ffi::c_int
        + ((*pNextRect).extent.height as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
        - r.extent.height as ::core::ffi::c_int) as COORD;
    if (r.corner.y as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
        r.corner.y = 0 as COORD;
    } else if r.corner.y as ::core::ffi::c_int + r.extent.height as ::core::ffi::c_int
        >= CanvasHeight
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
        r.corner.y = (CanvasHeight
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
            - r.extent.height as ::core::ffi::c_int) as COORD;
    }
    BoxUnion(pLastRect, &raw mut r, &raw mut r);
    BoxUnion(pNextRect, &raw mut r, &raw mut r);
    *pLastRect = *pNextRect;
    if (r.corner.x as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
        r.extent.width = (r.extent.width as ::core::ffi::c_int
            + r.corner.x as ::core::ffi::c_int) as COORD;
        r.corner.x = 0 as COORD;
    }
    if r.corner.x as ::core::ffi::c_int + r.extent.width as ::core::ffi::c_int
        > CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
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
    {
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
                } << resolutionFactor)) - r.corner.x as ::core::ffi::c_int) as COORD;
    }
    if (r.corner.y as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
        r.extent.height = (r.extent.height as ::core::ffi::c_int
            + r.corner.y as ::core::ffi::c_int) as COORD;
        r.corner.y = 0 as COORD;
    }
    if r.corner.y as ::core::ffi::c_int + r.extent.height as ::core::ffi::c_int
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
    }
    r.extent.height = (r.extent.height as ::core::ffi::c_int
        + (r.corner.y as ::core::ffi::c_int & 1 as ::core::ffi::c_int)) as COORD;
    r.corner.y = (r.corner.y as ::core::ffi::c_int & !(1 as ::core::ffi::c_int))
        as COORD;
    uqm_lua_checked!(DrawStarMap(update_race, &raw mut r), ());
}
unsafe extern "C" fn UpdateMap() {
    let mut ButtonState: BYTE = 0;
    let mut VisibleChange: BYTE = 0;
    let mut MapDrawn: BOOLEAN = BOOLEAN::FALSE;
    let mut Interrupted: BOOLEAN = BOOLEAN::FALSE;
    let mut index: COUNT = 0;
    let mut hStarShip: HFLEETINFO = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut hNextShip: HFLEETINFO = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut GrowthFactor: COUNT = 0;
    FlushInput();
    ButtonState = 1 as BYTE;
    Interrupted = BOOLEAN::FALSE;
    MapDrawn = Interrupted;
    index = 1 as COUNT;
    hStarShip = GlobData.Game_state.avail_race_q.head as HFLEETINFO;
    while !hStarShip.is_null() {
        let mut FleetPtr: *mut FLEET_INFO = ::core::ptr::null_mut::<FLEET_INFO>();
        FleetPtr = LockFleetInfo(&raw mut GlobData.Game_state.avail_race_q, hStarShip);
        hNextShip = (*FleetPtr).succ;
        if ButtonState != 0 {
            if uqm_lua_checked!(AnyButtonPress(BOOLEAN::TRUE), ()).0 == 0 {
                ButtonState = 0 as BYTE;
            }
        } else {
            Interrupted = BOOLEAN(
                (Interrupted.0 != 0 || uqm_lua_checked!(AnyButtonPress(BOOLEAN::TRUE), ()).0 != 0)
                    as ::core::ffi::c_int as ::core::ffi::c_uint,
            );
            if Interrupted.0 != 0 {
                MapDrawn = BOOLEAN::TRUE;
            }
        }
        if (*FleetPtr).known_strength != 0 {
            let mut dx: SIZE = 0;
            let mut dy: SIZE = 0;
            let mut delta: SIZE = 0;
            let mut r: RECT = RECT {
                corner: POINT { x: 0, y: 0 },
                extent: EXTENT { width: 0, height: 0 },
            };
            let mut last_r: RECT = RECT {
                corner: POINT { x: 0, y: 0 },
                extent: EXTENT { width: 0, height: 0 },
            };
            let mut temp_r0: RECT = RECT {
                corner: POINT { x: 0, y: 0 },
                extent: EXTENT { width: 0, height: 0 },
            };
            let mut temp_r1: RECT = RECT {
                corner: POINT { x: 0, y: 0 },
                extent: EXTENT { width: 0, height: 0 },
            };
            let mut str: COUNT = 0;
            dx = ((*FleetPtr).loc.x as ::core::ffi::c_int
                - (*FleetPtr).known_loc.x as ::core::ffi::c_int) as SIZE;
            dy = ((*FleetPtr).loc.y as ::core::ffi::c_int
                - (*FleetPtr).known_loc.y as ::core::ffi::c_int) as SIZE;
            if dx as ::core::ffi::c_int != 0 || dy as ::core::ffi::c_int != 0 {
                let mut xincr: SIZE = 0;
                let mut yincr: SIZE = 0;
                let mut xerror: SIZE = 0;
                let mut yerror: SIZE = 0;
                let mut cycle: SIZE = 0;
                if dx as ::core::ffi::c_int >= 0 as ::core::ffi::c_int {
                    xincr = 1 as SIZE;
                } else {
                    xincr = -1 as SIZE;
                    dx = -(dx as ::core::ffi::c_int) as SIZE;
                }
                dx = ((dx as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as SIZE;
                if dy as ::core::ffi::c_int >= 0 as ::core::ffi::c_int {
                    yincr = 1 as SIZE;
                } else {
                    yincr = -1 as SIZE;
                    dy = -(dy as ::core::ffi::c_int) as SIZE;
                }
                dy = ((dy as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as SIZE;
                if dx as ::core::ffi::c_int >= dy as ::core::ffi::c_int {
                    cycle = dx;
                } else {
                    cycle = dy;
                }
                yerror = (cycle as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
                    as SIZE;
                xerror = yerror;
                delta = xerror;
                if MapDrawn.0 == 0 {
                    uqm_lua_checked!(DrawStarMap(
                        !(0 as ::core::ffi::c_int) as COUNT,
                        ::core::ptr::null_mut::<RECT>(),
                    ), ());
                    MapDrawn = BOOLEAN::TRUE;
                }
                GetSphereRect(FleetPtr, &raw mut temp_r0, &raw mut last_r);
                last_r.extent.width = (last_r.extent.width as ::core::ffi::c_int
                    + (((1 as ::core::ffi::c_int) << resolutionFactor)
                        + if if resolutionFactor != 2 as ::core::ffi::c_uint {
                            BOOLEAN::FALSE.0 as ::core::ffi::c_int
                        } else {
                            BOOLEAN::TRUE.0 as ::core::ffi::c_int
                        } == 0
                        {
                            0 as ::core::ffi::c_int
                        } else {
                            1 as ::core::ffi::c_int
                        })) as COORD;
                last_r.extent.height = (last_r.extent.height as ::core::ffi::c_int
                    + (((1 as ::core::ffi::c_int) << resolutionFactor)
                        + if if resolutionFactor != 2 as ::core::ffi::c_uint {
                            BOOLEAN::FALSE.0 as ::core::ffi::c_int
                        } else {
                            BOOLEAN::TRUE.0 as ::core::ffi::c_int
                        } == 0
                        {
                            0 as ::core::ffi::c_int
                        } else {
                            1 as ::core::ffi::c_int
                        })) as COORD;
                VisibleChange = BOOLEAN::FALSE.0 as ::core::ffi::c_int as BYTE;
                '_DoneSphereMove: {
                    loop {
                        loop {
                            xerror = (xerror as ::core::ffi::c_int
                                - dx as ::core::ffi::c_int) as SIZE;
                            if xerror as ::core::ffi::c_int <= 0 as ::core::ffi::c_int {
                                (*FleetPtr).known_loc.x = ((*FleetPtr).known_loc.x
                                    as ::core::ffi::c_int + xincr as ::core::ffi::c_int)
                                    as COORD;
                                xerror = (xerror as ::core::ffi::c_int
                                    + cycle as ::core::ffi::c_int) as SIZE;
                            }
                            yerror = (yerror as ::core::ffi::c_int
                                - dy as ::core::ffi::c_int) as SIZE;
                            if yerror as ::core::ffi::c_int <= 0 as ::core::ffi::c_int {
                                (*FleetPtr).known_loc.y = ((*FleetPtr).known_loc.y
                                    as ::core::ffi::c_int + yincr as ::core::ffi::c_int)
                                    as COORD;
                                yerror = (yerror as ::core::ffi::c_int
                                    + cycle as ::core::ffi::c_int) as SIZE;
                            }
                            GetSphereRect(FleetPtr, &raw mut temp_r1, &raw mut r);
                            let c2rust_fresh1 = delta;
                            delta -= 1;
                            if !(c2rust_fresh1 as ::core::ffi::c_int != 0
                                && (delta as ::core::ffi::c_int & 0x1f as ::core::ffi::c_int
                                    != 0
                                    || temp_r0.corner.x as ::core::ffi::c_int
                                        == temp_r1.corner.x as ::core::ffi::c_int
                                        && temp_r0.corner.y as ::core::ffi::c_int
                                            == temp_r1.corner.y as ::core::ffi::c_int))
                            {
                                break;
                            }
                        }
                        if ButtonState != 0 {
                            if uqm_lua_checked!(AnyButtonPress(BOOLEAN::TRUE), ()).0 == 0 {
                                ButtonState = 0 as BYTE;
                            }
                        } else {
                            Interrupted = BOOLEAN(
                                (Interrupted.0 != 0 || uqm_lua_checked!(AnyButtonPress(BOOLEAN::TRUE), ()).0 != 0)
                                    as ::core::ffi::c_int as ::core::ffi::c_uint,
                            );
                            if Interrupted.0 != 0 {
                                MapDrawn = BOOLEAN::TRUE;
                                break '_DoneSphereMove;
                            }
                        }
                        r.extent.width = (r.extent.width as ::core::ffi::c_int
                            + (((1 as ::core::ffi::c_int) << resolutionFactor)
                                + if if resolutionFactor != 2 as ::core::ffi::c_uint {
                                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                                } else {
                                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                                } == 0
                                {
                                    0 as ::core::ffi::c_int
                                } else {
                                    1 as ::core::ffi::c_int
                                })) as COORD;
                        r.extent.height = (r.extent.height as ::core::ffi::c_int
                            + (((1 as ::core::ffi::c_int) << resolutionFactor)
                                + if if resolutionFactor != 2 as ::core::ffi::c_uint {
                                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                                } else {
                                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                                } == 0
                                {
                                    0 as ::core::ffi::c_int
                                } else {
                                    1 as ::core::ffi::c_int
                                })) as COORD;
                        if temp_r0.corner.x as ::core::ffi::c_int
                            != temp_r1.corner.x as ::core::ffi::c_int
                            || temp_r0.corner.y as ::core::ffi::c_int
                                != temp_r1.corner.y as ::core::ffi::c_int
                        {
                            VisibleChange = BOOLEAN::TRUE.0 as ::core::ffi::c_int
                                as BYTE;
                            uqm_lua_checked!(RepairMap(
                                (index as ::core::ffi::c_int
                                    | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int)
                                    as COUNT,
                                &raw mut last_r,
                                &raw mut r,
                            ), ());
                            uqm_lua_checked!(SleepThread(
                                (840 as ::core::ffi::c_int / 24 as ::core::ffi::c_int)
                                    as TimePeriod,
                            ), ());
                        }
                        if (delta as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                            break;
                        }
                    }
                    if VisibleChange != 0 {
                        uqm_lua_checked!(RepairMap(
                            !(0 as ::core::ffi::c_int) as COUNT,
                            &raw mut last_r,
                            &raw mut r,
                        ), ());
                    }
                }
                (*FleetPtr).known_loc = (*FleetPtr).loc;
            }
            delta = ((*FleetPtr).actual_strength as ::core::ffi::c_int
                - (*FleetPtr).known_strength as ::core::ffi::c_int) as SIZE;
            if delta != 0 {
                if MapDrawn.0 == 0 {
                    uqm_lua_checked!(DrawStarMap(
                        !(0 as ::core::ffi::c_int) as COUNT,
                        ::core::ptr::null_mut::<RECT>(),
                    ), ());
                    MapDrawn = BOOLEAN::TRUE;
                }
                if delta as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                    dx = 1 as SIZE;
                } else {
                    delta = -(delta as ::core::ffi::c_int) as SIZE;
                    dx = -1 as SIZE;
                }
                delta -= 1;
                GetSphereRect(FleetPtr, &raw mut temp_r0, &raw mut last_r);
                last_r.extent.width = (last_r.extent.width as ::core::ffi::c_int
                    + ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
                last_r.extent.height = (last_r.extent.height as ::core::ffi::c_int
                    + ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
                last_r.extent.height = (if last_r.extent.height as ::core::ffi::c_int
                    > (14 as ::core::ffi::c_int) << resolutionFactor
                {
                    last_r.extent.height as ::core::ffi::c_int
                } else {
                    (14 as ::core::ffi::c_int) << resolutionFactor
                }) as COORD;
                VisibleChange = BOOLEAN::FALSE.0 as ::core::ffi::c_int as BYTE;
                str = (*FleetPtr).known_strength;
                GrowthFactor = (if delta as ::core::ffi::c_int > 0 as ::core::ffi::c_int
                {
                    (*FleetPtr).actual_strength as ::core::ffi::c_int
                } else {
                    (*FleetPtr).known_strength as ::core::ffi::c_int
                }) as COUNT;
                '_DoneSphereGrowth: {
                    loop {
                        loop {
                            (*FleetPtr).known_strength = ((*FleetPtr).known_strength
                                as ::core::ffi::c_int + dx as ::core::ffi::c_int) as COUNT;
                            GetSphereRect(FleetPtr, &raw mut temp_r1, &raw mut r);
                            let c2rust_fresh2 = delta;
                            delta -= 1;
                            if !(c2rust_fresh2 as ::core::ffi::c_int != 0
                                && (delta as ::core::ffi::c_int & 0xf as ::core::ffi::c_int
                                    != 0
                                    || temp_r0.extent.height as ::core::ffi::c_int
                                        == temp_r1.extent.height as ::core::ffi::c_int))
                            {
                                break;
                            }
                        }
                        if ButtonState != 0 {
                            if uqm_lua_checked!(AnyButtonPress(BOOLEAN::TRUE), ()).0 == 0 {
                                ButtonState = 0 as BYTE;
                            }
                        } else {
                            Interrupted = BOOLEAN(
                                (Interrupted.0 != 0 || uqm_lua_checked!(AnyButtonPress(BOOLEAN::TRUE), ()).0 != 0)
                                    as ::core::ffi::c_int as ::core::ffi::c_uint,
                            );
                            if Interrupted.0 != 0 {
                                MapDrawn = BOOLEAN::TRUE;
                                break '_DoneSphereGrowth;
                            }
                        }
                        r.extent.width = (r.extent.width as ::core::ffi::c_int
                            + ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
                        r.extent.height = (r.extent.height as ::core::ffi::c_int
                            + ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
                        if temp_r0.extent.height as ::core::ffi::c_int
                            != temp_r1.extent.height as ::core::ffi::c_int
                            && !(str as ::core::ffi::c_int > 0 as ::core::ffi::c_int
                                && (*FleetPtr).known_strength as ::core::ffi::c_int
                                    == 0 as ::core::ffi::c_int)
                        {
                            VisibleChange = BOOLEAN::TRUE.0 as ::core::ffi::c_int
                                as BYTE;
                            uqm_lua_checked!(RepairMap(index, &raw mut last_r, &raw mut r), ());
                            uqm_lua_checked!(SleepThread(
                                (840 as ::core::ffi::c_int
                                    / (12 as ::core::ffi::c_int
                                        + GrowthFactor as ::core::ffi::c_int
                                            / 44 as ::core::ffi::c_int)) as TimePeriod,
                            ), ());
                        } else if str as ::core::ffi::c_int > 0 as ::core::ffi::c_int
                            && (*FleetPtr).known_strength as ::core::ffi::c_int
                                == 0 as ::core::ffi::c_int
                        {
                            VisibleChange = BOOLEAN::TRUE.0 as ::core::ffi::c_int
                                as BYTE;
                            uqm_lua_checked!(RepairMap(
                                (index as ::core::ffi::c_int
                                    | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                                    as COUNT,
                                &raw mut last_r,
                                &raw mut r,
                            ), ());
                            uqm_lua_checked!(SleepThread(
                                (840 as ::core::ffi::c_int / 12 as ::core::ffi::c_int)
                                    as TimePeriod,
                            ), ());
                            uqm_lua_checked!(RepairMap(
                                (index as ::core::ffi::c_int
                                    | (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int)
                                    as COUNT,
                                &raw mut last_r,
                                &raw mut r,
                            ), ());
                            uqm_lua_checked!(SleepThread(
                                (840 as ::core::ffi::c_int / 12 as ::core::ffi::c_int)
                                    as TimePeriod,
                            ), ());
                        }
                        if (delta as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                            break;
                        }
                    }
                    if VisibleChange as ::core::ffi::c_int != 0
                        || temp_r0.extent.width as ::core::ffi::c_int
                            != temp_r1.extent.width as ::core::ffi::c_int
                    {
                        uqm_lua_checked!(RepairMap(
                            !(0 as ::core::ffi::c_int) as COUNT,
                            &raw mut last_r,
                            &raw mut r,
                        ), ());
                    }
                }
                (*FleetPtr).known_strength = (*FleetPtr).actual_strength;
            }
        }
        UnlockLink(&raw mut GlobData.Game_state.avail_race_q, hStarShip);
        index = index.wrapping_add(1);
        hStarShip = hNextShip;
    }
}
unsafe extern "C" fn DrawStarmapKeyAtlas() {
    let mut OldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
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
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut origin: POINT = POINT { x: 0, y: 0 };
    let mut dheight: SIZE = 0;
    let mut spacing: SIZE = 0;
    let mut leading: SIZE = 0;
    let mut valign: SBYTE = 0;
    let mut sbottom: COORD = 0;
    let mut atlas_array: [FRAME; 3] = [::core::ptr::null_mut::<FRAME_DESC>(); 3];
    OldContext = SetContext(StatusContext);
    BatchGraphics();
    uqm_lua_checked!(DrawFlagStatDisplay(
        GetStringAddress(
            SetAbsStringTableIndex(
                GameStrings,
                (C2Rust_Unnamed_9::STATUS_STRING_BASE.0 as ::core::ffi::c_int
                    + 7 as ::core::ffi::c_int) as COUNT,
            ),
        ),
    ), ());
    SetContextFont(TinyFontCond);
    GetContextFontDispHeight(&raw mut dheight);
    GetContextFontVertAlign(&raw mut valign);
    SetContextForeGroundColor(
        buildColorRgba(0 as BYTE, 0xaa as BYTE, 0xaa as BYTE, 0xff as BYTE),
    );
    origin.x = ((4 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    origin.y = ((34 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    spacing = ((1 as ::core::ffi::c_int) << resolutionFactor) as SIZE;
    atlas_array[0usize] = CaptureDrawable(
        uqm_lua_checked!(LoadGraphicInstance(b"graphics.keyatlas\0".as_ptr() as RESOURCE), ()) as DRAWABLE,
    );
    atlas_array[1usize] = CaptureDrawable(
        uqm_lua_checked!(LoadGraphicInstance(b"graphics.axisatlas\0".as_ptr() as RESOURCE), ()) as DRAWABLE,
    );
    atlas_array[2usize] = CaptureDrawable(
        uqm_lua_checked!(LoadGraphicInstance(b"graphics.buttonatlas\0".as_ptr() as RESOURCE), ()) as DRAWABLE,
    );
    s.frame = ControlAtlas(
        C2Rust_Unnamed::KEY_MENU_TOGGLEMAP.0 as ::core::ffi::c_int,
        &raw mut atlas_array as *mut FRAME,
    );
    s.origin = origin;
    uqm_lua_checked!(DrawStamp(&raw mut s), ());
    GetFrameRect(s.frame, &raw mut r);
    sbottom = (s.origin.y as ::core::ffi::c_int + r.extent.height as ::core::ffi::c_int)
        as COORD;
    t.align = TEXT_ALIGN::ALIGN_LEFT;
    t.baseline.x = (s.origin.x as ::core::ffi::c_int
        + r.extent.width as ::core::ffi::c_int
        + ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    t.baseline.y = (sbottom as ::core::ffi::c_int
        - (r.extent.height as ::core::ffi::c_int - dheight as ::core::ffi::c_int)
            / 2 as ::core::ffi::c_int - valign as ::core::ffi::c_int) as COORD;
    t.pStr = GetStringAddress(
        SetAbsStringTableIndex(
            GameStrings,
            (C2Rust_Unnamed_9::STATUS_STRING_BASE.0 as ::core::ffi::c_int
                + 12 as ::core::ffi::c_int) as COUNT,
        ),
    );
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    uqm_lua_checked!(font_DrawText(&raw mut t), ());
    s.origin.y = (sbottom as ::core::ffi::c_int + spacing as ::core::ffi::c_int)
        as COORD;
    s.frame = ControlAtlas(
        C2Rust_Unnamed::KEY_MENU_SPECIAL.0 as ::core::ffi::c_int,
        &raw mut atlas_array as *mut FRAME,
    );
    uqm_lua_checked!(DrawStamp(&raw mut s), ());
    GetFrameRect(s.frame, &raw mut r);
    sbottom = (s.origin.y as ::core::ffi::c_int + r.extent.height as ::core::ffi::c_int)
        as COORD;
    t.baseline.x = (s.origin.x as ::core::ffi::c_int
        + r.extent.width as ::core::ffi::c_int
        + ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    t.baseline.y = (sbottom as ::core::ffi::c_int
        - (r.extent.height as ::core::ffi::c_int - dheight as ::core::ffi::c_int)
            / 2 as ::core::ffi::c_int - valign as ::core::ffi::c_int) as COORD;
    t.pStr = GetStringAddress(
        SetAbsStringTableIndex(
            GameStrings,
            (C2Rust_Unnamed_9::STATUS_STRING_BASE.0 as ::core::ffi::c_int
                + 13 as ::core::ffi::c_int) as COUNT,
        ),
    );
    uqm_lua_checked!(font_DrawText(&raw mut t), ());
    r = uqm_lua_checked!(font_GetTextRect(&raw mut t), ());
    s.origin.x = (t.baseline.x as ::core::ffi::c_int
        + r.extent.width as ::core::ffi::c_int
        + ((3 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    s.origin.y = (t.baseline.y as ::core::ffi::c_int
        - ((3 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    s.frame = SetAbsFrameIndex(MiscDataFrame, 108 as COUNT);
    uqm_lua_checked!(DrawStamp(&raw mut s), ());
    s.origin.y = (sbottom as ::core::ffi::c_int + spacing as ::core::ffi::c_int)
        as COORD;
    s.origin.x = origin.x;
    s.frame = ControlAtlas(
        C2Rust_Unnamed::KEY_MENU_NEXT.0 as ::core::ffi::c_int,
        &raw mut atlas_array as *mut FRAME,
    );
    uqm_lua_checked!(DrawStamp(&raw mut s), ());
    GetFrameRect(s.frame, &raw mut r);
    sbottom = (s.origin.y as ::core::ffi::c_int + r.extent.height as ::core::ffi::c_int)
        as COORD;
    t.baseline.x = (s.origin.x as ::core::ffi::c_int
        + r.extent.width as ::core::ffi::c_int
        + ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    t.baseline.y = (sbottom as ::core::ffi::c_int
        - (r.extent.height as ::core::ffi::c_int - dheight as ::core::ffi::c_int)
            / 2 as ::core::ffi::c_int - valign as ::core::ffi::c_int) as COORD;
    t.pStr = b":\0".as_ptr() as *const ::core::ffi::c_char as *const UNICODE;
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    uqm_lua_checked!(font_DrawText(&raw mut t), ());
    s.origin.x = (t.baseline.x as ::core::ffi::c_int
        + ((7 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    s.origin.y = (t.baseline.y as ::core::ffi::c_int
        - ((3 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    s.frame = SetAbsFrameIndex(MiscDataFrame, 48 as COUNT);
    uqm_lua_checked!(DrawStamp(&raw mut s), ());
    t.baseline.x = (t.baseline.x as ::core::ffi::c_int
        + ((13 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    t.pStr = GetStringAddress(
        SetAbsStringTableIndex(
            GameStrings,
            (C2Rust_Unnamed_9::STATUS_STRING_BASE.0 as ::core::ffi::c_int
                + 14 as ::core::ffi::c_int) as COUNT,
        ),
    );
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    uqm_lua_checked!(font_DrawText(&raw mut t), ());
    s.frame = ControlAtlas(
        C2Rust_Unnamed::KEY_MENU_SEARCH.0 as ::core::ffi::c_int,
        &raw mut atlas_array as *mut FRAME,
    );
    s.origin.y = (sbottom as ::core::ffi::c_int + spacing as ::core::ffi::c_int)
        as COORD;
    s.origin.x = origin.x;
    uqm_lua_checked!(DrawStamp(&raw mut s), ());
    GetFrameRect(s.frame, &raw mut r);
    sbottom = (s.origin.y as ::core::ffi::c_int + r.extent.height as ::core::ffi::c_int)
        as COORD;
    t.baseline.x = (s.origin.x as ::core::ffi::c_int
        + r.extent.width as ::core::ffi::c_int
        + ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    t.baseline.y = (sbottom as ::core::ffi::c_int
        - (r.extent.height as ::core::ffi::c_int - dheight as ::core::ffi::c_int)
            / 2 as ::core::ffi::c_int - valign as ::core::ffi::c_int) as COORD;
    t.pStr = GetStringAddress(
        SetAbsStringTableIndex(
            GameStrings,
            (C2Rust_Unnamed_9::STATUS_STRING_BASE.0 as ::core::ffi::c_int
                + 15 as ::core::ffi::c_int) as COUNT,
        ),
    );
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    uqm_lua_checked!(font_DrawText(&raw mut t), ());
    s.frame = ControlAtlas(
        C2Rust_Unnamed::KEY_MENU_ZOOM_OUT.0 as ::core::ffi::c_int,
        &raw mut atlas_array as *mut FRAME,
    );
    s.origin.y = (sbottom as ::core::ffi::c_int + spacing as ::core::ffi::c_int)
        as COORD;
    uqm_lua_checked!(DrawStamp(&raw mut s), ());
    GetFrameRect(s.frame, &raw mut r);
    s.frame = ControlAtlas(
        C2Rust_Unnamed::KEY_MENU_ZOOM_IN.0 as ::core::ffi::c_int,
        &raw mut atlas_array as *mut FRAME,
    );
    s.origin.x = (s.origin.x as ::core::ffi::c_int + r.extent.width as ::core::ffi::c_int
        + ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    uqm_lua_checked!(DrawStamp(&raw mut s), ());
    GetFrameRect(s.frame, &raw mut r);
    sbottom = (s.origin.y as ::core::ffi::c_int + r.extent.height as ::core::ffi::c_int)
        as COORD;
    t.baseline.x = (s.origin.x as ::core::ffi::c_int
        + r.extent.width as ::core::ffi::c_int
        + ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    t.baseline.y = (sbottom as ::core::ffi::c_int
        - (r.extent.height as ::core::ffi::c_int - dheight as ::core::ffi::c_int)
            / 2 as ::core::ffi::c_int - valign as ::core::ffi::c_int) as COORD;
    t.pStr = GetStringAddress(
        SetAbsStringTableIndex(
            GameStrings,
            (C2Rust_Unnamed_9::STATUS_STRING_BASE.0 as ::core::ffi::c_int
                + 16 as ::core::ffi::c_int) as COUNT,
        ),
    );
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    uqm_lua_checked!(font_DrawText(&raw mut t), ());
    GetContextFontLeading(&raw mut leading);
    r.corner.x = origin.x;
    r.corner.y = ((118 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    r.extent.width = (((64 as ::core::ffi::c_int) << resolutionFactor)
        - ((5 as ::core::ffi::c_int) << resolutionFactor)
        - ((3 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    r.extent.height = ((1 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    SetContextForeGroundColor(colorFromRgb15(0xa as BYTE, 0xa as BYTE, 0x1f as BYTE));
    uqm_lua_checked!(DrawFilledRectangle(&raw mut r), ());
    SetContextForeGroundColor(
        buildColorRgba(0 as BYTE, 0xaa as BYTE, 0xaa as BYTE, 0xff as BYTE),
    );
    t.align = TEXT_ALIGN::ALIGN_LEFT;
    t.baseline.x = (r.corner.x as ::core::ffi::c_int
        + ((2 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    t.baseline.y = (r.corner.y as ::core::ffi::c_int + leading as ::core::ffi::c_int
        + ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    t.pStr = GetStringAddress(
        SetAbsStringTableIndex(
            GameStrings,
            (C2Rust_Unnamed_9::STATUS_STRING_BASE.0 as ::core::ffi::c_int
                + 8 as ::core::ffi::c_int) as COUNT,
        ),
    );
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    uqm_lua_checked!(font_DrawText(&raw mut t), ());
    SetContextForeGroundColor(
        buildColorRgba(0x55 as BYTE, 0x55 as BYTE, 0xff as BYTE, 0xff as BYTE),
    );
    t.align = TEXT_ALIGN::ALIGN_RIGHT;
    t.baseline.x = (r.extent.width as ::core::ffi::c_int
        + ((2 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    t.baseline.y = (r.corner.y as ::core::ffi::c_int + leading as ::core::ffi::c_int
        + ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    t.pStr = WholeFuelValue();
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    uqm_lua_checked!(font_DrawText(&raw mut t), ());
    UnbatchGraphics();
    SetContext(OldContext);
    DestroyDrawable(ReleaseDrawable(atlas_array[0usize]));
    DestroyDrawable(ReleaseDrawable(atlas_array[1usize]));
    DestroyDrawable(ReleaseDrawable(atlas_array[2usize]));
}
#[no_mangle]
pub unsafe extern "C" fn StarMap() -> BOOLEAN {
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
    let mut universe: POINT = POINT { x: 0, y: 0 };
    let mut clip_r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut OldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    memset(
        &raw mut MenuState as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<MENU_STATE>(),
    );
    which_starmap = CURRENT_STARMAP_SHOWN::NORMAL_STARMAP;
    zoomLevel = 0 as ::core::ffi::c_int;
    mapOrigin.x = (9999 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as COORD;
    mapOrigin.y = (9999 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as COORD;
    StarMapFrame = SetAbsFrameIndex(MiscDataFrame, 48 as COUNT);
    if inHQSpace().0 == 0 {
        universe = (*CurStarDescPtr).star_pt;
    } else {
        universe.x = logxToUniverse(GlobData.SIS_state.log_x) as COORD;
        universe.y = logyToUniverse(GlobData.SIS_state.log_y) as COORD;
    }
    cursorLoc = GlobData.Game_state.autopilot;
    if cursorLoc.x as ::core::ffi::c_int == !(0 as ::core::ffi::c_int)
        && cursorLoc.y as ::core::ffi::c_int == !(0 as ::core::ffi::c_int)
    {
        cursorLoc = universe;
    }
    if optWhichMenu == 0x2 as ::core::ffi::c_int {
        if playerInPlanetOrbit() {
            uqm_lua_checked!(DrawMenuStateStrings(
                C2Rust_Unnamed_6::PM_ALT_SCAN.0 as ::core::ffi::c_int as BYTE,
                1 as SWORD,
            ), BOOLEAN(0));
        } else {
            uqm_lua_checked!(DrawMenuStateStrings(
                C2Rust_Unnamed_6::PM_ALT_STARMAP.0 as ::core::ffi::c_int as BYTE,
                0 as SWORD,
            ), BOOLEAN(0));
        }
    }
    MenuState.InputFunc = Some(
        DoMoveCursor as unsafe extern "C" fn(*mut MENU_STATE) -> BOOLEAN,
    ) as Option<unsafe extern "C" fn(*mut menu_state) -> BOOLEAN>;
    MenuState.Initialized = BOOLEAN::FALSE.0 as ::core::ffi::c_int as SIZE;
    transition_pending = BOOLEAN::TRUE;
    if uqm_lua_checked!(getGameStateUint(b"ARILOU_SPACE_SIDE\0".as_ptr() as *const ::core::ffi::c_char), BOOLEAN(0))
        <= 1 as DWORD
    {
        uqm_lua_checked!(UpdateMap(), BOOLEAN(0));
    } else if universe.x as ::core::ffi::c_int <= 6134 as ::core::ffi::c_int
        && universe.y as ::core::ffi::c_int <= 5900 as ::core::ffi::c_int
        && (universe.x as ::core::ffi::c_int >= 4480 as ::core::ffi::c_int
            && universe.y as ::core::ffi::c_int >= 4580 as ::core::ffi::c_int)
    {
        zoomLevel = 2 as ::core::ffi::c_int;
    }
    if optSubmenu.0 != 0 {
        uqm_lua_checked!(DrawStarmapKeyAtlas(), BOOLEAN(0));
    }
    uqm_lua_checked!(DrawStarMap(
        0 as COUNT,
        ::core::ptr::with_exposed_provenance_mut::<RECT>(-1 as ::core::ffi::c_int as usize),
    ), BOOLEAN(0));
    transition_pending = BOOLEAN::FALSE;
    BatchGraphics();
    OldContext = SetContext(SpaceContext);
    GetContextClipRect(&raw mut clip_r);
    SetContext(OldContext);
    LoadIntoExtraScreen(&raw mut clip_r);
    uqm_lua_checked!(DrawCursor(
        ((universeToDispx(cursorLoc.x as ::core::ffi::c_long) as ::core::ffi::c_int)
            << resolutionFactor) as COORD,
        ((universeToDispy(cursorLoc.y as ::core::ffi::c_long) as ::core::ffi::c_int)
            << resolutionFactor) as COORD,
    ), BOOLEAN(0));
    UnbatchGraphics();
    SetMenuSounds(
        0 as ::core::ffi::c_int as MENU_SOUND_FLAGS,
        0 as ::core::ffi::c_int as MENU_SOUND_FLAGS,
    );
    SetMenuRepeatDelay(
        (840 as ::core::ffi::c_int / 60 as ::core::ffi::c_int) as DWORD,
        (840 as ::core::ffi::c_int / 8 as ::core::ffi::c_int) as DWORD,
        (840 as ::core::ffi::c_int / 120 as ::core::ffi::c_int) as DWORD,
        BOOLEAN::TRUE,
    );
    uqm_lua_checked!(DoInput(&raw mut MenuState as *mut ::core::ffi::c_void, BOOLEAN::FALSE), BOOLEAN(0));
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
    SetDefaultMenuRepeatDelay();
    uqm_lua_checked!(DrawHyperCoords(universe), BOOLEAN(0));
    if GlobData.Game_state.autopilot.x as ::core::ffi::c_int
        != !(0 as ::core::ffi::c_int)
        && GlobData.Game_state.autopilot.y as ::core::ffi::c_int
            != !(0 as ::core::ffi::c_int)
    {
        uqm_lua_checked!(DrawAutoPilotMessage(BOOLEAN::FALSE), BOOLEAN(0));
    } else {
        uqm_lua_checked!(DrawSISMessage(::core::ptr::null::<UNICODE>()), BOOLEAN(0));
    }
    uqm_lua_checked!(DrawStatusMessage(::core::ptr::null::<UNICODE>()), BOOLEAN(0));
    if optSubmenu.0 != 0 {
        uqm_lua_checked!(DeltaSISGauges(0x7fff as SIZE, 0x7fff as SDWORD, 0x7fff as ::core::ffi::c_int), BOOLEAN(0));
    }
    return BOOLEAN(
        (GlobData.Game_state.autopilot.x as ::core::ffi::c_int
            != !(0 as ::core::ffi::c_int)
            && GlobData.Game_state.autopilot.y as ::core::ffi::c_int
                != !(0 as ::core::ffi::c_int)) as ::core::ffi::c_int
            as ::core::ffi::c_uint,
    );
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
                b"src/uqm/planets/../races.h\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                410 as ::core::ffi::c_int,
                b"LockFleetInfo\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    return LockLink(pq, h) as *mut FLEET_INFO;
}
