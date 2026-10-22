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

// Rust translation of src/uqm/supermelee/melee.c.
extern "C" {
    pub type uio_Handle;
    pub type uio_DirHandle;
    pub type uio_Stream;
    pub type string_table_entry;
    pub type string_table;
    pub type context_desc;
    pub type frame_desc;
    pub type font_desc;
    pub type drawable_desc;
    pub type joy_char;
    pub type tfb_soundsample;
    pub type RandomContext;
    pub type NetConnection;
    pub type MeleeTeam;
    pub type MeleeSetup;
    fn SetContext(Context: CONTEXT) -> CONTEXT;
    fn SetContextForeGroundColor(Color: Color) -> Color;
    fn GetContextForeGroundColor() -> Color;
    fn SetContextBackGroundColor(Color: Color) -> Color;
    fn SetContextFGFrame(Frame: FRAME) -> FRAME;
    fn SetContextClipRect(pRect: *mut RECT) -> BOOLEAN;
    fn GetContextClipRect(pRect: *mut RECT) -> BOOLEAN;
    fn SetContextOrigin(orgOffset: POINT) -> POINT;
    fn DrawStamp(pStamp: *mut STAMP);
    fn DrawRectangle(pRect: *mut RECT, scaled: BOOLEAN);
    fn DrawFilledRectangle(pRect: *mut RECT);
    fn font_GetTextRect(pText: *mut TEXT) -> RECT;
    fn font_DrawText(pText: *mut TEXT);
    fn font_DrawTracedText(pText: *mut TEXT, text: Color, trace: Color);
    fn BatchGraphics();
    fn UnbatchGraphics();
    fn ClearDrawable();
    fn DestroyDrawable(Drawable: DRAWABLE) -> BOOLEAN;
    fn GetFrameRect(Frame: FRAME, pRect: *mut RECT) -> BOOLEAN;
    fn LoadGraphicInstance(res: RESOURCE) -> *mut ::core::ffi::c_void;
    fn SetContextFontEffect(EffectFrame: FRAME) -> FRAME;
    fn SetContextFont(Font: FONT) -> FONT;
    fn DestroyFont(FontRef: FONT) -> BOOLEAN;
    fn TextRect(pText: *mut TEXT, pRect: *mut RECT, pdelta: *mut BYTE) -> BOOLEAN;
    fn GetContextFontLeading(pheight: *mut SIZE) -> BOOLEAN;
    fn GetFrameCount(Frame: FRAME) -> COUNT;
    fn SetAbsFrameIndex(Frame: FRAME, FrameIndex: COUNT) -> FRAME;
    fn CaptureDrawable(Drawable: DRAWABLE) -> FRAME;
    fn ReleaseDrawable(Frame: FRAME) -> DRAWABLE;
    fn FadeScreen(fadeType: ScreenFadeType, TimeInterval: SIZE) -> DWORD;
    fn FlushColorXForms();
    fn __assert_fail(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    ) -> !;
    fn uio_fstat(handle: *mut uio_Handle, statBuf: *mut stat) -> ::core::ffi::c_int;
    fn uio_fopen(
        dir: *mut uio_DirHandle,
        path: *const ::core::ffi::c_char,
        mode: *const ::core::ffi::c_char,
    ) -> *mut uio_Stream;
    fn uio_fclose(stream: *mut uio_Stream) -> ::core::ffi::c_int;
    fn uio_fgetc(stream: *mut uio_Stream) -> ::core::ffi::c_int;
    fn uio_fputc(c: ::core::ffi::c_int, stream: *mut uio_Stream) -> ::core::ffi::c_int;
    fn uio_streamHandle(stream: *mut uio_Stream) -> *mut uio_Handle;
    fn sprintf(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn res_OpenResFile(
        dir: *mut uio_DirHandle,
        filename: *const ::core::ffi::c_char,
        mode: *const ::core::ffi::c_char,
    ) -> *mut uio_Stream;
    fn res_CloseResFile(fp: *mut uio_Stream) -> BOOLEAN;
    fn DeleteResFile(
        dir: *mut uio_DirHandle,
        filename: *const ::core::ffi::c_char,
    ) -> BOOLEAN;
    fn DestroyStringTable(StringTable: STRING_TABLE) -> BOOLEAN;
    fn CaptureStringTable(StringTable: STRING_TABLE) -> STRING;
    fn ReleaseStringTable(String: STRING) -> STRING_TABLE;
    fn SetAbsStringTableIndex(String: STRING, StringTableIndex: COUNT) -> STRING;
    fn GetStringAddress(String: STRING) -> STRINGPTR;
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
    fn AddPadd(str: *const UNICODE, padding: *mut sint16) -> *mut UNICODE;
    fn GetTimeCounter() -> TimeCount;
    fn memcpy(
        _: *mut ::core::ffi::c_void,
        _: *const ::core::ffi::c_void,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        _: *mut ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strncpy(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: size_t,
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
    fn ControlAtlas(menu_index: ::core::ffi::c_int, atlas_array: *mut FRAME) -> FRAME;
    static mut PulsedInputState: CONTROLLER_INPUT_STATE;
    static mut PlayerControls: [CONTROL_TEMPLATE; 0];
    fn FlushInput();
    fn DoPopupWindow(msg: *const ::core::ffi::c_char);
    fn DoInput(pInputState: *mut ::core::ffi::c_void, resetInput: BOOLEAN);
    static mut GamePaused: BOOLEAN;
    fn DoTextEntry(pTES: *mut TEXTENTRY_STATE) -> BOOLEAN;
    static mut optWhichMenu: ::core::ffi::c_int;
    static mut resolutionFactor: ::core::ffi::c_uint;
    static mut optMainMenuMusic: OPT_ENABLABLE;
    static mut optControllerType: ::core::ffi::c_int;
    static mut optMeleeToolTips: OPT_ENABLABLE;
    static mut optWindowType: DWORD;
    static mut configDir: *mut uio_DirHandle;
    static mut CanvasWidth: ::core::ffi::c_int;
    static mut CanvasHeight: ::core::ffi::c_int;
    fn InitSpace() -> BOOLEAN;
    fn UninitSpace();
    fn SleepThread(timePeriod: TimePeriod);
    fn SleepThreadUntil(wakeTime: TimeCount);
    fn LoadSoundInstance(res: RESOURCE) -> SOUND_REF;
    fn LoadMusicInstance(res: RESOURCE) -> MUSIC_REF;
    fn DestroySound(SoundRef: SOUND_REF) -> BOOLEAN;
    fn DestroyMusic(MusicRef: MUSIC_REF) -> BOOLEAN;
    fn WaitForSoundEnd(Channel: COUNT);
    fn FadeMusic(end_vol: BYTE, TimeInterval: SIZE) -> DWORD;
    fn SetMusicPosition();
    static mut GlobData: GLOBDATA;
    fn InitGlobData();
    fn is3DO(optWhich: ::core::ffi::c_int) -> OPT_CONSOLETYPE;
    fn TFB_SeedRandom(seed: DWORD) -> DWORD;
    fn TFB_Random() -> DWORD;
    fn RandomContext_New() -> *mut RandomContext;
    fn RandomContext_Delete(context: *mut RandomContext);
    fn RandomContext_SeedRandom(context: *mut RandomContext, new_seed: DWORD) -> DWORD;
    fn NetConnection_isConnected(conn: *const NetConnection) -> bool;
    fn NetConnection_getState(conn: *const NetConnection) -> NetState;
    fn NetConnection_getDiscriminant(conn: *const NetConnection) -> bool;
    fn NetConnection_getPeerOptions(
        conn: *const NetConnection,
    ) -> *const NetplayPeerOptions;
    fn NetConnection_getPlayerNr(conn: *const NetConnection) -> ::core::ffi::c_int;
    static mut netplayOptions: NetplayOptions;
    static mut webNetplayRole: ::core::ffi::c_int;
    fn Battle(_: Option<BattleFrameCallback>) -> BOOLEAN;
    static MeleeTeam_serialSize: size_t;
    fn MeleeTeam_getFleet(team: *const MeleeTeam) -> *const MeleeShip;
    fn MeleeTeam_getTeamName(team: *const MeleeTeam) -> *const ::core::ffi::c_char;
    fn MeleeSetup_getSentShip(
        setup: *const MeleeSetup,
        teamNr: size_t,
        slotNr: FleetShipIndex,
    ) -> MeleeShip;
    fn MeleeSetup_getSentTeamName(
        setup: *const MeleeSetup,
        teamNr: size_t,
    ) -> *const ::core::ffi::c_char;
    fn MeleeSetup_setSentShip(
        setup: *mut MeleeSetup,
        teamNr: size_t,
        slotNr: FleetShipIndex,
        ship: MeleeShip,
    ) -> bool;
    fn MeleeSetup_setSentTeamName(
        setup: *mut MeleeSetup,
        teamNr: size_t,
        name: *const ::core::ffi::c_char,
    ) -> bool;
    fn MeleeSetup_new() -> *mut MeleeSetup;
    fn MeleeSetup_delete(setup: *mut MeleeSetup);
    fn MeleeSetup_setShip(
        setup: *mut MeleeSetup,
        teamNr: size_t,
        slotNr: FleetShipIndex,
        ship: MeleeShip,
    ) -> bool;
    fn MeleeSetup_getShip(
        setup: *const MeleeSetup,
        teamNr: size_t,
        slotNr: FleetShipIndex,
    ) -> MeleeShip;
    fn MeleeSetup_setTeamName(
        setup: *mut MeleeSetup,
        teamNr: size_t,
        name: *const ::core::ffi::c_char,
    ) -> bool;
    fn MeleeSetup_getTeamName(
        setup: *const MeleeSetup,
        teamNr: size_t,
    ) -> *const ::core::ffi::c_char;
    fn MeleeSetup_getFleetValue(setup: *const MeleeSetup, teamNr: size_t) -> COUNT;
    fn MeleeSetup_deserializeTeam(
        setup: *mut MeleeSetup,
        teamNr: size_t,
        stream: *mut uio_Stream,
    ) -> ::core::ffi::c_int;
    fn MeleeSetup_serializeTeam(
        setup: *const MeleeSetup,
        teamNr: size_t,
        stream: *mut uio_Stream,
    ) -> ::core::ffi::c_int;
    fn InitMeleeLoadState(pMS: *mut MELEE_STATE);
    fn UninitMeleeLoadState(pMS: *mut MELEE_STATE);
    fn DoLoadTeam(pMS: *mut MELEE_STATE) -> BOOLEAN;
    fn DoSaveTeam(pMS: *mut MELEE_STATE) -> BOOLEAN;
    fn LoadTeamList(pMS: *mut MELEE_STATE);
    fn BuildBuildPickFrame();
    fn DestroyBuildPickFrame();
    fn BuildPickShip(pMS: *mut MELEE_STATE) -> bool;
    fn GetBuildPickFrameRect(r: *mut RECT);
    fn GetToolTipFrameRect(r: *mut RECT);
    fn DrawPickFrame(pMS: *mut MELEE_STATE);
    fn DrawPickIcon(ship: MeleeShip, DrawErase: bool);
    fn DrawTooltip(SIPtr: *mut SHIP_INFO);
    fn ClearShipStatus(y: COORD);
    fn InitShipStatus(
        ShipInfoPtr: *mut SHIP_INFO,
        StarShipPtr: *mut STARSHIP,
        pClipRect: *mut RECT,
        inMeleeMenu: BOOLEAN,
    );
    fn load_gravity_well(selector: BYTE);
    fn free_gravity_well();
    fn getLineWithinWidth(
        pText: *mut TEXT,
        startNext: *mut *const ::core::ffi::c_char,
        maxWidth: SIZE,
        maxChars: COUNT,
    ) -> BOOLEAN;
    static mut GameStrings: STRING;
    fn SetCursorRect(r: *mut RECT, context: CONTEXT);
    fn SetCursorFlashBlock(state: BOOLEAN);
    fn BuildPickMeleeFrame();
    fn DestroyPickMeleeFrame();
    fn FillPickMeleeFrame(setup: *mut MeleeSetup);
    static mut Screen: FRAME;
    static mut FontGradFrame: FRAME;
    static mut SpaceContext: CONTEXT;
    static mut StatusContext: CONTEXT;
    static mut StarConFont: FONT;
    static mut MicroFont: FONT;
    static mut LabelFont: FONT;
    static mut PlayerControl: [BYTE; 0];
    fn SetPlayerInputAll() -> bool;
    fn ClearPlayerInputAll();
    static mut master_q: QUEUE;
    fn GetShipCostFromIndex(Index: ::core::ffi::c_uint) -> COUNT;
    fn GetShipMeleeIconsFromIndex(Index: ::core::ffi::c_uint) -> FRAME;
    static mut netConnections: [*mut NetConnection; 0];
    fn closeAllConnections();
    fn closeDisconnectedConnections();
    fn NetMelee_getBattleState(conn: *mut NetConnection) -> *mut battlestate_struct;
    fn netInput();
    fn flushPacketQueues();
    fn confirmConnections();
    fn cancelConfirmations();
    fn openPlayerNetworkConnection(
        player: COUNT,
        extra: *mut ::core::ffi::c_void,
    ) -> *mut NetConnection;
    fn closePlayerNetworkConnection(player: COUNT);
    fn setupInputDelay(localInputDelay: size_t) -> bool;
    fn negotiateReadyConnections(notifyRemote: bool, nextState: NetState) -> bool;
    fn Netplay_Notify_seedRandom(conn: *mut NetConnection, seed: uint32);
    fn Netplay_NotifyAll_setTeamName(pMS: *mut MELEE_STATE, playerNr: size_t);
    fn Netplay_NotifyAll_setFleet(pMS: *mut MELEE_STATE, playerNr: size_t);
    fn Netplay_NotifyAll_setShip(pMS: *mut MELEE_STATE, playerNr: size_t, index: size_t);
    fn Netplay_NotifyAll_inputDelay(delay: size_t) -> bool;
    fn MeleeConnectDialog(side: ::core::ffi::c_int) -> BOOLEAN;
    fn DrawShadowedBox(r: *mut RECT, bg: Color, dark: Color, medium: Color);
    fn StopMusic();
    fn PlayMusicResume(hMusic: MUSIC_REF, Volume: BYTE);
    static mut GameSounds: SOUND;
    fn SetMenuSounds(sound_0: MENU_SOUND_FLAGS, sound_1: MENU_SOUND_FLAGS);
    fn PlayMenuSound(S: MENU_SOUND_EFFECT);
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
    fn DrawBorderPadding(videoWidth: DWORD);
    fn SeedRandomNumbers() -> DWORD;
    fn log_add(_: log_Level, fmt: *const ::core::ffi::c_char, ...);
    fn GetStarShipFromIndex(pShipQ: *mut QUEUE, Index: COUNT) -> HLINK;
}
pub type size_t = usize;
pub type ssize_t = isize;
pub type off_t = ::core::ffi::c_longlong;
pub type uid_t = ::core::ffi::c_uint;
pub type gid_t = ::core::ffi::c_uint;
pub type time_t = ::core::ffi::c_longlong;
pub type mode_t = ::core::ffi::c_uint;
pub type nlink_t = ::core::ffi::c_ulong;
pub type ino_t = ::core::ffi::c_ulonglong;
pub type dev_t = ::core::ffi::c_uint;
pub type blksize_t = ::core::ffi::c_int;
pub type blkcnt_t = ::core::ffi::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: time_t,
    pub tv_nsec: ::core::ffi::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: dev_t,
    pub st_mode: mode_t,
    pub st_nlink: nlink_t,
    pub st_uid: uid_t,
    pub st_gid: gid_t,
    pub st_rdev: dev_t,
    pub st_size: off_t,
    pub st_blksize: blksize_t,
    pub st_blocks: blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub st_ino: ino_t,
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
pub type DIRENTRY = STRING;
pub type CONTEXT_DESC = context_desc;
pub type FRAME_DESC = frame_desc;
pub type FONT_DESC = font_desc;
pub type DRAWABLE_DESC = drawable_desc;
pub type CONTEXT = *mut CONTEXT_DESC;
pub type FRAME = *mut FRAME_DESC;
pub type FONT = *mut FONT_DESC;
pub type DRAWABLE = *mut DRAWABLE_DESC;
pub type TIME_VALUE = UWORD;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct INTERSECT_CONTROL {
    pub last_time_val: TIME_VALUE,
    pub EndPoint: POINT,
    pub IntersectStamp: STAMP,
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct CONTROL_TEMPLATE(pub ::core::ffi::c_uint);
impl CONTROL_TEMPLATE {
    pub const CONTROL_TEMPLATE_PL_1: Self = Self(0);
    pub const CONTROL_TEMPLATE_PL_2: Self = Self(1);
    pub const NUM_TEMPLATES: Self = Self(2);
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
pub type SOUND_REF = STRING_TABLE;
pub type SOUND = STRING;
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
pub struct NetplayAbortReason(pub ::core::ffi::c_uint);
impl NetplayAbortReason {
    pub const AbortReason_unspecified: Self = Self(0);
    pub const AbortReason_versionMismatch: Self = Self(1);
    pub const AbortReason_invalidHash: Self = Self(2);
    pub const AbortReason_protocolError: Self = Self(3);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct NetplayResetReason(pub ::core::ffi::c_uint);
impl NetplayResetReason {
    pub const ResetReason_unspecified: Self = Self(0);
    pub const ResetReason_syncLoss: Self = Self(1);
    pub const ResetReason_manualReset: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct NetState(pub ::core::ffi::c_uint);
impl NetState {
    pub const NetState_unconnected: Self = Self(0);
    pub const NetState_connecting: Self = Self(1);
    pub const NetState_init: Self = Self(2);
    pub const NetState_inSetup: Self = Self(3);
    pub const NetState_preBattle: Self = Self(4);
    pub const NetState_interBattle: Self = Self(5);
    pub const NetState_selectShip: Self = Self(6);
    pub const NetState_inBattle: Self = Self(7);
    pub const NetState_endingBattle: Self = Self(8);
    pub const NetState_endingBattle2: Self = Self(9);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NetplayPeerOptions {
    pub isServer: bool,
    pub host: *const ::core::ffi::c_char,
    pub port: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NetplayOptions {
    pub metaServer: *const ::core::ffi::c_char,
    pub metaPort: *const ::core::ffi::c_char,
    pub peer: [NetplayPeerOptions; 2],
    pub inputDelay: size_t,
}
pub type BattleFrameCallback = unsafe extern "C" fn() -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct battlestate_struct {
    pub InputFunc: Option<unsafe extern "C" fn(*mut battlestate_struct) -> BOOLEAN>,
    pub first_time: BOOLEAN,
    pub NextTime: DWORD,
    pub frame_cb: Option<BattleFrameCallback>,
}
pub type HELEMENT = HLINK;
pub type ELEMENT_FLAGS = UWORD;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct state {
    pub location: POINT,
    pub image: C2Rust_Unnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_1 {
    pub frame: FRAME,
    pub farray: *mut FRAME,
}
pub type STATE = state;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct element {
    pub pred: HELEMENT,
    pub succ: HELEMENT,
    pub preprocess_func: Option<ElementProcessFunc>,
    pub postprocess_func: Option<ElementProcessFunc>,
    pub collision_func: Option<ElementCollisionFunc>,
    pub death_func: Option<ElementProcessFunc>,
    pub playerNr: SIZE,
    pub state_flags: ELEMENT_FLAGS,
    pub c2rust_unnamed: C2Rust_Unnamed_6,
    pub c2rust_unnamed_0: C2Rust_Unnamed_5,
    pub c2rust_unnamed_1: C2Rust_Unnamed_4,
    pub c2rust_unnamed_2: C2Rust_Unnamed_3,
    pub c2rust_unnamed_3: C2Rust_Unnamed_2,
    pub colorCycleIndex: BYTE,
    pub velocity: VELOCITY_DESC,
    pub IntersectControl: INTERSECT_CONTROL,
    pub PrimIndex: COUNT,
    pub current: STATE,
    pub next: STATE,
    pub pParent: *mut ::core::ffi::c_void,
    pub hTarget: HELEMENT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_2 {
    pub thrust_wait: BYTE,
    pub blast_offset: BYTE,
    pub next_turn: BYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_3 {
    pub turn_wait: BYTE,
    pub sys_loc: BYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_4 {
    pub mass_points: BYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_5 {
    pub crew_level: COUNT,
    pub hit_points: COUNT,
    pub facing: COUNT,
    pub cycle: COUNT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_6 {
    pub life_span: COUNT,
    pub scan_node: COUNT,
}
pub type ElementProcessFunc = unsafe extern "C" fn(*mut ELEMENT) -> ();
pub type ELEMENT = element;
pub type ElementCollisionFunc = unsafe extern "C" fn(
    *mut ELEMENT,
    *mut POINT,
    *mut ELEMENT,
    *mut POINT,
) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct melee_state {
    pub InputFunc: Option<unsafe extern "C" fn(*mut melee_state) -> BOOLEAN>,
    pub Initialized: BOOLEAN,
    pub meleeStarted: BOOLEAN,
    pub MeleeOption: MELEE_OPTIONS,
    pub side: COUNT,
    pub row: COUNT,
    pub col: COUNT,
    pub meleeSetup: *mut MeleeSetup,
    pub load: melee_load_state,
    pub currentShip: MeleeShip,
    pub CurIndex: COUNT,
    pub buildPickConfirmed: BOOLEAN,
    pub randomContext: *mut RandomContext,
    pub LastInputTime: TimeCount,
    pub hMusic: MUSIC_REF,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct melee_load_state {
    pub preBuiltList: *mut *mut MeleeTeam,
    pub preBuiltCount: COUNT,
    pub dirEntries: DIRENTRY,
    pub entryIndices: *mut COUNT,
    pub numIndices: COUNT,
    pub view: [*mut MeleeTeam; 5],
    pub top: COUNT,
    pub bot: COUNT,
    pub cur: COUNT,
    pub viewSize: COUNT,
}
pub type MELEE_OPTIONS = BYTE;
pub type MELEE_STATE = melee_state;
pub type FleetShipIndex = COUNT;
pub type MENU_SOUND_FLAGS = UWORD;
pub type HMASTERSHIP = HLINK;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MASTER_SHIP_INFO {
    pub pred: HMASTERSHIP,
    pub succ: HMASTERSHIP,
    pub SpeciesID: SPECIES_ID,
    pub ShipInfo: SHIP_INFO,
    pub Fleet: FLEET_STUFF,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FLEET_STUFF {
    pub strength: COUNT,
    pub known_loc: POINT,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct STARSHIP {
    pub pred: HLINK,
    pub succ: HLINK,
    pub SpeciesID: SPECIES_ID,
    pub captains_name_index: BYTE,
    pub RaceDescPtr: *mut RACE_DESC,
    pub crew_level: COUNT,
    pub max_crew: COUNT,
    pub ship_cost: BYTE,
    pub index: COUNT,
    pub race_strings: STRING,
    pub icons: FRAME,
    pub weapon_counter: BYTE,
    pub special_counter: BYTE,
    pub energy_counter: BYTE,
    pub ship_input_state: BYTE,
    pub cur_status_flags: STATUS_FLAGS,
    pub old_status_flags: STATUS_FLAGS,
    pub hShip: HELEMENT,
    pub ShipFacing: COUNT,
    pub playerNr: SIZE,
    pub control: BYTE,
}
pub type STATUS_FLAGS = UWORD;
pub type RACE_DESC = race_desc;
#[derive(Copy, Clone)]
#[repr(C, align(16))]
pub struct race_desc {
    pub ship_info: SHIP_INFO,
    pub c2rust_padding: [u8; 8],
    pub fleet: FLEET_STUFF,
    pub c2rust_padding_0: [u8; 10],
    pub characteristics: CHARACTERISTIC_STUFF,
    pub c2rust_padding_1: [u8; 2],
    pub ship_data: DATA_STUFF,
    pub c2rust_padding_2: [u8; 8],
    pub cyborg_control: INTEL_STUFF,
    pub uninit_func: Option<UNINIT_FUNC>,
    pub preprocess_func: Option<PREPROCESS_FUNC>,
    pub postprocess_func: Option<POSTPROCESS_FUNC>,
    pub init_weapon_func: Option<INIT_WEAPON_FUNC>,
    pub data: *mut ::core::ffi::c_void,
    pub CodeRef: *mut ::core::ffi::c_void,
}
pub type INIT_WEAPON_FUNC = unsafe extern "C" fn(*mut ELEMENT, *mut HELEMENT) -> COUNT;
pub type POSTPROCESS_FUNC = unsafe extern "C" fn(*mut ELEMENT) -> ();
pub type PREPROCESS_FUNC = unsafe extern "C" fn(*mut ELEMENT) -> ();
pub type UNINIT_FUNC = unsafe extern "C" fn(*mut RACE_DESC) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct INTEL_STUFF {
    pub ManeuverabilityIndex: COUNT,
    pub WeaponRange: COUNT,
    pub intelligence_func: Option<IntelligenceFunc>,
}
pub type IntelligenceFunc = unsafe extern "C" fn(
    *mut ELEMENT,
    *mut EVALUATE_DESC,
    COUNT,
) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EVALUATE_DESC {
    pub ObjectPtr: *mut ELEMENT,
    pub facing: COUNT,
    pub which_turn: COUNT,
    pub MoveState: MOVEMENT_STATE,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct MOVEMENT_STATE(pub ::core::ffi::c_uint);
impl MOVEMENT_STATE {
    pub const PURSUE: Self = Self(0);
    pub const AVOID: Self = Self(1);
    pub const ENTICE: Self = Self(2);
    pub const NO_MOVEMENT: Self = Self(3);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DATA_STUFF {
    pub ship_rsc: [RESOURCE; 3],
    pub weapon_rsc: [RESOURCE; 3],
    pub special_rsc: [RESOURCE; 3],
    pub captain_control: CAPTAIN_STUFF,
    pub victory_ditty_rsc: RESOURCE,
    pub ship_sounds_rsc: RESOURCE,
    pub ship: [FRAME; 3],
    pub weapon: [FRAME; 3],
    pub special: [FRAME; 3],
    pub victory_ditty: MUSIC_REF,
    pub ship_sounds: SOUND,
}
pub type CAPTAIN_STUFF = captain_stuff;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct captain_stuff {
    pub captain_rsc: RESOURCE,
    pub background: FRAME,
    pub turn: FRAME,
    pub thrust: FRAME,
    pub weapon: FRAME,
    pub special: FRAME,
    pub tl_offset: BYTE,
    pub tr_offset: BYTE,
    pub thrust_offset: BYTE,
    pub weapon_offset: BYTE,
    pub special_offset: BYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CHARACTERISTIC_STUFF {
    pub max_thrust: COUNT,
    pub thrust_increment: COUNT,
    pub energy_regeneration: BYTE,
    pub weapon_energy_cost: BYTE,
    pub special_energy_cost: BYTE,
    pub energy_wait: BYTE,
    pub turn_wait: BYTE,
    pub thrust_wait: BYTE,
    pub weapon_wait: BYTE,
    pub special_wait: BYTE,
    pub ship_mass: BYTE,
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
pub struct C2Rust_Unnamed_7(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_7 {
    pub const MELEE_BACKGROUND: Self = Self(0);
    pub const HUMAN_CON_TOP: Self = Self(1);
    pub const WEAK_CYBORG_TOP: Self = Self(2);
    pub const GOOD_CYBORG_TOP: Self = Self(3);
    pub const AWES_CYBORG_TOP: Self = Self(4);
    pub const HUMAN_CON_TOP_HL: Self = Self(5);
    pub const WEAK_CYBORG_TOP_HL: Self = Self(6);
    pub const GOOD_CYBORG_TOP_HL: Self = Self(7);
    pub const AWES_CYBORG_TOP_HL: Self = Self(8);
    pub const HUMAN_CON_BOTT: Self = Self(9);
    pub const WEAK_CYBORG_BOTT: Self = Self(10);
    pub const GOOD_CYBORG_BOTT: Self = Self(11);
    pub const AWES_CYBORG_BOTT: Self = Self(12);
    pub const HUMAN_CON_BOTT_HL: Self = Self(13);
    pub const WEAK_CYBORG_BOTT_HL: Self = Self(14);
    pub const GOOD_CYBORG_BOTT_HL: Self = Self(15);
    pub const AWES_CYBORG_BOTT_HL: Self = Self(16);
    pub const LOAD_BUTTON_TOP: Self = Self(17);
    pub const SAVE_BUTTON_TOP: Self = Self(18);
    pub const LOAD_BUTTON_TOP_HL: Self = Self(19);
    pub const SAVE_BUTTON_TOP_HL: Self = Self(20);
    pub const SAVE_BUTTON_BOTT: Self = Self(21);
    pub const LOAD_BUTTON_BOTT: Self = Self(22);
    pub const SAVE_BUTTON_BOTT_HL: Self = Self(23);
    pub const LOAD_BUTTON_BOTT_HL: Self = Self(24);
    pub const BATTLE_BUTTON: Self = Self(25);
    pub const BATTLE_BUTTON_HL: Self = Self(26);
    pub const SHIP_PICKER: Self = Self(27);
    pub const TEAM_PICKER: Self = Self(28);
    pub const QUIT_BUTTON: Self = Self(29);
    pub const QUIT_BUTTON_HL: Self = Self(30);
    pub const NETWORK_CON_TOP: Self = Self(31);
    pub const NETWORK_CON_TOP_HL: Self = Self(32);
    pub const NETWORK_CON_BOTT: Self = Self(33);
    pub const NETWORK_CON_BOTT_HL: Self = Self(34);
    pub const NET_BUTTON_TOP: Self = Self(35);
    pub const NET_BUTTON_TOP_HL: Self = Self(36);
    pub const NET_BUTTON_BOTT: Self = Self(37);
    pub const NET_BUTTON_BOTT_HL: Self = Self(38);
    pub const SHIP_DESC: Self = Self(39);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_8(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_8 {
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_10(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_10 {
    pub const NET_TOP: Self = Self(0);
    pub const CONTROLS_TOP: Self = Self(1);
    pub const SAVE_TOP: Self = Self(2);
    pub const LOAD_TOP: Self = Self(3);
    pub const START_MELEE: Self = Self(4);
    pub const LOAD_BOT: Self = Self(5);
    pub const SAVE_BOT: Self = Self(6);
    pub const CONTROLS_BOT: Self = Self(7);
    pub const QUIT_BOT: Self = Self(8);
    pub const EDIT_MELEE: Self = Self(9);
    pub const BUILD_PICK: Self = Self(10);
}
// Presentation palette only. Keep collision, control and gauge semantics intact.
unsafe fn melee_ui_active() -> bool {
    extern "C" { fn UQM_GetMeleeRemastered() -> ::core::ffi::c_int; }
    UQM_GetMeleeRemastered() != 0 &&
        crate::src::uqm::globdata::GlobData.Game_state.CurrentActivity & 0xff == 0
}
unsafe fn melee_ui_color(mut c: Color) -> Color {
    if !melee_ui_active() || c.a != 255 { return c; }
    let next = match (c.r, c.g, c.b) {
        (0, 0, 74) => (38, 62, 77),
        (0, 0, 115) => (49, 79, 96),
        (0, 0, 99) => (20, 37, 47),
        (57, 0, 99) => (141, 225, 223),
        (99, 0, 165) => (71, 125, 136),
        (82, 0, 140) => (28, 57, 73),
        (0, 0, 165) => (18, 35, 45),
        (0, 82, 165) => (87, 123, 140),
        (165, 0, 165) => (33, 63, 80),
        (2, 4, 62) => (18, 38, 49),
        (4, 8, 124) => (30, 65, 83),
        (35, 140, 210) => (157, 189, 207),
        (40, 160, 240) => (201, 242, 245),
        (40, 40, 124) => (25, 48, 59),
        (60, 60, 186) => (49, 89, 103),
        (31, 0, 31) => (18, 35, 45),
        (62, 0, 62) => (49, 89, 103),
        (66, 66, 66) => (38, 60, 73),
        (87, 87, 87) => (26, 42, 54),
        (85, 85, 85) => (26, 42, 54),
        (82, 82, 82) => (20, 34, 45),
        (132, 132, 132) => (74, 107, 123),
        (116, 116, 116) => (166, 191, 204),
        (138, 138, 138) => (175, 201, 215),
        (69, 69, 69) => (35, 59, 73),
        (248, 224, 0) => (167, 240, 227),
        (123, 132, 222) => (155, 182, 200),
        (189, 198, 239) => (222, 240, 246),
        (50, 50, 155) => (135, 182, 200),
        _ => return c,
    };
    c.r=next.0; c.g=next.1; c.b=next.2;
    c
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
    return melee_ui_color(c);
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
    return melee_ui_color(c);
}
#[inline]
unsafe extern "C" fn MAKE_POINT(mut x: COORD, mut y: COORD) -> POINT {
    let mut pt: POINT = point { x: x, y: y };
    return pt;
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
#[no_mangle]
pub static mut MeleeFrame: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
#[no_mangle]
pub static mut AtlasArray: [FRAME; 3] = [::core::ptr::null_mut::<FRAME_DESC>(); 3];
#[no_mangle]
pub static mut SpecialAtlas: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
#[no_mangle]
pub static mut pMeleeState: *mut MELEE_STATE = ::core::ptr::null_mut::<MELEE_STATE>();
#[no_mangle]
pub static mut MicroThinFont: FONT = ::core::ptr::null_mut::<FONT_DESC>();
#[no_mangle]
pub static mut ButtonFont: FONT = ::core::ptr::null_mut::<FONT_DESC>();
unsafe extern "C" fn ButtonText(mut which_icon: COUNT) -> ::core::ffi::c_int {
    match which_icon as ::core::ffi::c_int {
        1 | 5 | 9 | 13 => return 10 as ::core::ffi::c_int,
        2 | 6 | 10 | 14 => return 11 as ::core::ffi::c_int,
        3 | 7 | 11 | 15 => return 12 as ::core::ffi::c_int,
        4 | 8 | 12 | 16 => return 13 as ::core::ffi::c_int,
        17 | 19 | 22 | 24 => return 14 as ::core::ffi::c_int,
        18 | 20 | 21 | 23 => return 15 as ::core::ffi::c_int,
        25 | 26 => return 16 as ::core::ffi::c_int,
        29 | 30 => return 17 as ::core::ffi::c_int,
        31 | 32 | 33 | 34 => return 18 as ::core::ffi::c_int,
        35 | 36 | 37 | 38 => return 19 as ::core::ffi::c_int,
        _ => return 0 as ::core::ffi::c_int,
    };
}
unsafe extern "C" fn DrawControlText(
    mut stamp: STAMP,
    mut which_icon: COUNT,
    mut HiLite: BOOLEAN,
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
    let mut OldFont: FONT = ::core::ptr::null_mut::<FONT_DESC>();
    let mut leading: SIZE = 0;
    let mut buf: [UNICODE; 256] = [0; 256];
    if ButtonText(which_icon) == 0 {
        return;
    }
    OldFont = SetContextFont(MicroThinFont);
    GetFrameRect(stamp.frame, &raw mut r);
    GetContextFontLeading(&raw mut leading);
    utf8StringCopy(
        &raw mut buf as *mut UNICODE,
        ::core::mem::size_of::<[UNICODE; 256]>(),
        GetStringAddress(
            SetAbsStringTableIndex(
                GameStrings,
                (C2Rust_Unnamed_9::MELEE_STRING_BASE.0 as ::core::ffi::c_int
                    + ButtonText(which_icon)) as COUNT,
            ),
        ),
    );
    t.align = TEXT_ALIGN::ALIGN_CENTER;
    t.pStr = strtok(
        &raw mut buf as *mut ::core::ffi::c_char,
        b" \0".as_ptr() as *const ::core::ffi::c_char,
    );
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    t.baseline.x = (r.corner.x as ::core::ffi::c_int
        + (r.extent.width as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)) as COORD;
    t.baseline.y = (r.corner.y as ::core::ffi::c_int + leading as ::core::ffi::c_int
        + ((4 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    while !t.pStr.is_null() {
        t.pStr = AlignText(t.pStr, &raw mut t.baseline.x);
        t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
        uqm_lua_checked!(font_DrawTracedText(
            &raw mut t,
            if HiLite.0 != 0 {
                buildColorRgba(0x28 as BYTE, 0xa0 as BYTE, 0xf0 as BYTE, 0xff as BYTE)
            } else {
                buildColorRgba(0x23 as BYTE, 0x8c as BYTE, 0xd2 as BYTE, 0xff as BYTE)
            },
            if HiLite.0 != 0 {
                buildColorRgba(0x3c as BYTE, 0x3c as BYTE, 0xba as BYTE, 0xff as BYTE)
            } else {
                buildColorRgba(0x28 as BYTE, 0x28 as BYTE, 0x7c as BYTE, 0xff as BYTE)
            },
        ), ());
        t.pStr = strtok(
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
            b" \0".as_ptr() as *const ::core::ffi::c_char,
        );
        t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
        t.baseline.y = (t.baseline.y as ::core::ffi::c_int
            + leading as ::core::ffi::c_int) as COORD;
    }
    SetContextFont(OldFont);
}
unsafe extern "C" fn DrawBattleText(
    mut stamp: STAMP,
    mut which_icon: COUNT,
    mut HiLite: BOOLEAN,
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
    let mut OldFont: FONT = ::core::ptr::null_mut::<FONT_DESC>();
    let mut OldFontEffect: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
    let mut leading: SIZE = 0;
    let mut buf: [UNICODE; 256] = [0; 256];
    if ButtonText(which_icon) == 0 {
        return;
    }
    OldFont = SetContextFont(LabelFont);
    GetFrameRect(stamp.frame, &raw mut r);
    GetContextFontLeading(&raw mut leading);
    utf8StringCopy(
        &raw mut buf as *mut UNICODE,
        ::core::mem::size_of::<[UNICODE; 256]>(),
        GetStringAddress(
            SetAbsStringTableIndex(
                GameStrings,
                (C2Rust_Unnamed_9::MELEE_STRING_BASE.0 as ::core::ffi::c_int
                    + ButtonText(which_icon)) as COUNT,
            ),
        ),
    );
    t.baseline.x = (r.corner.x as ::core::ffi::c_int
        + (r.extent.width as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)) as COORD;
    t.baseline.y = (r.corner.y as ::core::ffi::c_int
        + r.extent.height as ::core::ffi::c_int
        - ((leading as ::core::ffi::c_int >> resolutionFactor >> 1 as ::core::ffi::c_int)
            << resolutionFactor) - ((1 as ::core::ffi::c_int) << resolutionFactor))
        as COORD;
    t.align = TEXT_ALIGN::ALIGN_CENTER;
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    t.pStr = AlignText(
        &raw mut buf as *mut UNICODE as *const UNICODE,
        &raw mut t.baseline.x,
    );
    uqm_lua_checked!(font_DrawTracedText(
        &raw mut t,
        buildColorRgba(0x50 as BYTE, 0x50 as BYTE, 0x50 as BYTE, 0 as BYTE),
        if HiLite.0 != 0 {
            buildColorRgba(0x3e as BYTE, 0 as BYTE, 0x3e as BYTE, 0xff as BYTE)
        } else {
            buildColorRgba(0x1f as BYTE, 0 as BYTE, 0x1f as BYTE, 0xff as BYTE)
        },
    ), ());
    OldFontEffect = SetContextFontEffect(
        SetAbsFrameIndex(
            FontGradFrame,
            (8 as ::core::ffi::c_uint).wrapping_add(HiLite.0) as COUNT,
        ),
    );
    uqm_lua_checked!(font_DrawText(&raw mut t), ());
    SetContextFontEffect(OldFontEffect);
    SetContextFont(OldFont);
}
unsafe extern "C" fn DrawButtonText(
    mut stamp: STAMP,
    mut which_icon: COUNT,
    mut HiLite: BOOLEAN,
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
    let mut OldFont: FONT = ::core::ptr::null_mut::<FONT_DESC>();
    let mut OldColor: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    let mut buf: [UNICODE; 256] = [0; 256];
    if ButtonText(which_icon) == 0 {
        return;
    }
    OldFont = SetContextFont(ButtonFont);
    OldColor = SetContextForeGroundColor(
        if HiLite.0 != 0 {
            buildColorRgba(0xf8 as BYTE, 0xe0 as BYTE, 0 as BYTE, 0xff as BYTE)
        } else {
            buildColorRgba(0x8a as BYTE, 0x8a as BYTE, 0x8a as BYTE, 0xff as BYTE)
        },
    );
    GetFrameRect(stamp.frame, &raw mut r);
    utf8StringCopy(
        &raw mut buf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[UNICODE; 256]>(),
        GetStringAddress(
            SetAbsStringTableIndex(
                GameStrings,
                (C2Rust_Unnamed_9::MELEE_STRING_BASE.0 as ::core::ffi::c_int
                    + ButtonText(which_icon)) as COUNT,
            ),
        ),
    );
    t.baseline.x = (r.corner.x as ::core::ffi::c_int
        + (r.extent.width as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)) as COORD;
    t.baseline.y = r.corner.y;
    t.align = TEXT_ALIGN::ALIGN_CENTER;
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    t.pStr = AlignText(
        &raw mut buf as *mut UNICODE as *const UNICODE,
        &raw mut t.baseline.x,
    );
    uqm_lua_checked!(font_DrawText(&raw mut t), ());
    SetContextFont(OldFont);
    SetContextForeGroundColor(OldColor);
}
unsafe extern "C" fn DrawVerticalText(mut str: *mut UNICODE, mut point: POINT) {
    let mut t: TEXT = TEXT {
        baseline: POINT { x: 0, y: 0 },
        pStr: ::core::ptr::null::<UNICODE>(),
        align: TEXT_ALIGN::ALIGN_LEFT,
        CharCount: 0,
    };
    let mut i: COUNT = 0;
    let mut leading: SIZE = 0;
    let mut str_len: size_t = 0;
    let mut buf: [UNICODE; 256] = [0; 256];
    let mut OldColor: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    GetContextFontLeading(&raw mut leading);
    t.baseline = point;
    utf8StringCopy(
        &raw mut buf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[UNICODE; 256]>(),
        AlignText(
            AddPadd(str as *const UNICODE, &raw mut leading) as *const UNICODE,
            &raw mut t.baseline.x,
        ),
    );
    str_len = strlen(&raw mut buf as *mut UNICODE);
    t.align = TEXT_ALIGN::ALIGN_CENTER;
    t.CharCount = 1 as COUNT;
    t.pStr = &raw mut buf as *mut UNICODE;
    OldColor = GetContextForeGroundColor();
    i = 0 as COUNT;
    while (i as size_t) < str_len {
        SetContextForeGroundColor(
            buildColorRgba(0x45 as BYTE, 0x45 as BYTE, 0x45 as BYTE, 0xff as BYTE),
        );
        t.baseline.x = (t.baseline.x as ::core::ffi::c_int
            - ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        uqm_lua_checked!(font_DrawText(&raw mut t), ());
        SetContextForeGroundColor(
            buildColorRgba(0x74 as BYTE, 0x74 as BYTE, 0x74 as BYTE, 0xff as BYTE),
        );
        t.baseline.x = (t.baseline.x as ::core::ffi::c_int
            + ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        uqm_lua_checked!(font_DrawText(&raw mut t), ());
        t.pStr = skipUTF8Chars(t.pStr, 1 as size_t);
        t.baseline.y = (t.baseline.y as ::core::ffi::c_int
            + leading as ::core::ffi::c_int) as COORD;
        i = i.wrapping_add(1);
    }
    SetContextForeGroundColor(OldColor);
}
#[no_mangle]
pub unsafe extern "C" fn DrawShipPickerText(mut stamp: STAMP) {
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut OldFont: FONT = ::core::ptr::null_mut::<FONT_DESC>();
    let mut i: COUNT = 0;
    let mut s: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    let mut pt: POINT = POINT { x: 0, y: 0 };
    i = 0 as COUNT;
    while (i as ::core::ffi::c_int) < 2 as ::core::ffi::c_int {
        if strlen(
            GetStringAddress(
                SetAbsStringTableIndex(
                    GameStrings,
                    (C2Rust_Unnamed_9::MELEE_STRING_BASE.0 as ::core::ffi::c_int
                        + 23 as ::core::ffi::c_int + i as ::core::ffi::c_int) as COUNT,
                ),
            ),
        ) == 0
        {
            return;
        }
        i = i.wrapping_add(1);
    }
    OldFont = SetContextFont(LabelFont);
    GetFrameRect(stamp.frame, &raw mut r);
    pt.x = (r.corner.x as ::core::ffi::c_int
        + ((2 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    pt.y = (r.corner.y as ::core::ffi::c_int
        + ((3 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    s.frame = ControlAtlas(
        C2Rust_Unnamed::KEY_MENU_SELECT.0 as ::core::ffi::c_int,
        &raw mut AtlasArray as *mut FRAME,
    );
    s.origin = pt;
    uqm_lua_checked!(DrawStamp(&raw mut s), ());
    GetFrameRect(s.frame, &raw mut r);
    pt.x = (s.origin.x as ::core::ffi::c_int
        + (r.extent.width as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)) as COORD;
    pt.y = (s.origin.y as ::core::ffi::c_int + r.extent.height as ::core::ffi::c_int
        + ((9 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    uqm_lua_checked!(DrawVerticalText(
        GetStringAddress(
            SetAbsStringTableIndex(
                GameStrings,
                (C2Rust_Unnamed_9::MELEE_STRING_BASE.0 as ::core::ffi::c_int
                    + 23 as ::core::ffi::c_int) as COUNT,
            ),
        ),
        pt,
    ), ());
    GetFrameRect(stamp.frame, &raw mut r);
    pt.x = (r.corner.x as ::core::ffi::c_int + r.extent.width as ::core::ffi::c_int
        - ((2 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    pt.y = (r.corner.y as ::core::ffi::c_int
        + ((3 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    s.frame = ControlAtlas(
        C2Rust_Unnamed::KEY_MENU_SPECIAL.0 as ::core::ffi::c_int,
        &raw mut AtlasArray as *mut FRAME,
    );
    GetFrameRect(s.frame, &raw mut r);
    pt.x = (pt.x as ::core::ffi::c_int - r.extent.width as ::core::ffi::c_int) as COORD;
    s.origin = pt;
    uqm_lua_checked!(DrawStamp(&raw mut s), ());
    pt.x = (s.origin.x as ::core::ffi::c_int
        + (r.extent.width as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)) as COORD;
    pt.y = (s.origin.y as ::core::ffi::c_int + r.extent.height as ::core::ffi::c_int
        + ((9 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    uqm_lua_checked!(DrawVerticalText(
        GetStringAddress(
            SetAbsStringTableIndex(
                GameStrings,
                (C2Rust_Unnamed_9::MELEE_STRING_BASE.0 as ::core::ffi::c_int
                    + 24 as ::core::ffi::c_int) as COUNT,
            ),
        ),
        pt,
    ), ());
    SetContextFont(OldFont);
}
unsafe extern "C" fn DrawTeamPickerText(mut stamp: STAMP) {
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut text_r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut pt: POINT = POINT { x: 0, y: 0 };
    let mut t: TEXT = TEXT {
        baseline: POINT { x: 0, y: 0 },
        pStr: ::core::ptr::null::<UNICODE>(),
        align: TEXT_ALIGN::ALIGN_LEFT,
        CharCount: 0,
    };
    let mut OldFont: FONT = ::core::ptr::null_mut::<FONT_DESC>();
    let mut OldColor: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    let mut buf: [UNICODE; 256] = [0; 256];
    let mut s: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    let mut i: COUNT = 0;
    let mut leading: SIZE = 0;
    i = 0 as COUNT;
    while (i as ::core::ffi::c_int) < 3 as ::core::ffi::c_int {
        utf8StringCopy(
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[UNICODE; 256]>(),
            GetStringAddress(
                SetAbsStringTableIndex(
                    GameStrings,
                    (C2Rust_Unnamed_9::MELEE_STRING_BASE.0 as ::core::ffi::c_int
                        + 20 as ::core::ffi::c_int + i as ::core::ffi::c_int) as COUNT,
                ),
            ),
        );
        if strlen(&raw mut buf as *mut UNICODE) == 0 {
            return;
        }
        i = i.wrapping_add(1);
    }
    OldFont = SetContextFont(LabelFont);
    OldColor = SetContextForeGroundColor(
        buildColorRgba(0x32 as BYTE, 0x32 as BYTE, 0x9b as BYTE, 0xff as BYTE),
    );
    GetFrameRect(stamp.frame, &raw mut r);
    GetContextFontLeading(&raw mut leading);
    pt.x = (r.corner.x as ::core::ffi::c_int
        + ((3 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    pt.y = (r.corner.y as ::core::ffi::c_int + r.extent.height as ::core::ffi::c_int
        - ((3 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    s.frame = ControlAtlas(
        C2Rust_Unnamed::KEY_MENU_SELECT.0 as ::core::ffi::c_int,
        &raw mut AtlasArray as *mut FRAME,
    );
    GetFrameRect(s.frame, &raw mut r);
    pt.y = (pt.y as ::core::ffi::c_int - r.extent.height as ::core::ffi::c_int) as COORD;
    s.origin = pt;
    uqm_lua_checked!(DrawStamp(&raw mut s), ());
    utf8StringCopy(
        &raw mut buf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[UNICODE; 256]>(),
        GetStringAddress(
            SetAbsStringTableIndex(
                GameStrings,
                (C2Rust_Unnamed_9::MELEE_STRING_BASE.0 as ::core::ffi::c_int
                    + 20 as ::core::ffi::c_int) as COUNT,
            ),
        ),
    );
    t.baseline.x = (r.extent.width as ::core::ffi::c_int + pt.x as ::core::ffi::c_int
        + ((3 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    t.baseline.y = (pt.y as ::core::ffi::c_int + leading as ::core::ffi::c_int
        - ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    t.align = TEXT_ALIGN::ALIGN_LEFT;
    t.pStr = &raw mut buf as *mut UNICODE;
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    uqm_lua_checked!(font_DrawText(&raw mut t), ());
    text_r = uqm_lua_checked!(font_GetTextRect(&raw mut t), ());
    s.frame = ControlAtlas(
        C2Rust_Unnamed::KEY_MENU_CANCEL.0 as ::core::ffi::c_int,
        &raw mut AtlasArray as *mut FRAME,
    );
    s.origin.x = (text_r.extent.width as ::core::ffi::c_int
        + t.baseline.x as ::core::ffi::c_int
        + ((3 as ::core::ffi::c_int) << resolutionFactor)
        + ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    uqm_lua_checked!(DrawStamp(&raw mut s), ());
    utf8StringCopy(
        &raw mut buf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[UNICODE; 256]>(),
        GetStringAddress(
            SetAbsStringTableIndex(
                GameStrings,
                (C2Rust_Unnamed_9::MELEE_STRING_BASE.0 as ::core::ffi::c_int
                    + 21 as ::core::ffi::c_int) as COUNT,
            ),
        ),
    );
    t.baseline.x = (s.origin.x as ::core::ffi::c_int
        + r.extent.width as ::core::ffi::c_int
        + ((3 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    uqm_lua_checked!(font_DrawText(&raw mut t), ());
    text_r = uqm_lua_checked!(font_GetTextRect(&raw mut t), ());
    s.frame = SetAbsFrameIndex(SpecialAtlas, optControllerType as COUNT);
    s.origin.x = (text_r.extent.width as ::core::ffi::c_int
        + t.baseline.x as ::core::ffi::c_int
        + ((3 as ::core::ffi::c_int) << resolutionFactor)
        + ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    uqm_lua_checked!(DrawStamp(&raw mut s), ());
    utf8StringCopy(
        &raw mut buf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[UNICODE; 256]>(),
        GetStringAddress(
            SetAbsStringTableIndex(
                GameStrings,
                (C2Rust_Unnamed_9::MELEE_STRING_BASE.0 as ::core::ffi::c_int
                    + 22 as ::core::ffi::c_int) as COUNT,
            ),
        ),
    );
    t.baseline.x = (s.origin.x as ::core::ffi::c_int
        + r.extent.width as ::core::ffi::c_int
        + ((3 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    uqm_lua_checked!(font_DrawText(&raw mut t), ());
    text_r = uqm_lua_checked!(font_GetTextRect(&raw mut t), ());
    SetContextFont(OldFont);
    SetContextForeGroundColor(OldColor);
}
unsafe extern "C" fn WhichText(mut which_icon: COUNT) -> ::core::ffi::c_int {
    match which_icon as ::core::ffi::c_int {
        1 | 5 | 9 | 13 | 2 | 6 | 10 | 14 | 3 | 7 | 11 | 15 | 4 | 8 | 12 | 16 | 31 | 32
        | 33 | 34 => return 1 as ::core::ffi::c_int,
        17 | 19 | 22 | 24 | 18 | 20 | 21 | 23 | 29 | 30 | 35 | 36 | 37 | 38 => {
            return 3 as ::core::ffi::c_int;
        }
        25 | 26 => return 2 as ::core::ffi::c_int,
        28 => return 4 as ::core::ffi::c_int,
        _ => return 0 as ::core::ffi::c_int,
    };
}
#[no_mangle]
pub unsafe extern "C" fn DrawMeleeIcon(mut which_icon: COUNT, mut HiLite: BOOLEAN) {
    let mut s: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    let mut which_text: ::core::ffi::c_int = WhichText(which_icon);
    let mut NeedBatch: BOOLEAN = BOOLEAN(
        (which_text != 0 && which_text < 3 as ::core::ffi::c_int) as ::core::ffi::c_int
            as ::core::ffi::c_uint,
    );
    if NeedBatch.0 != 0 {
        BatchGraphics();
    }
    s.origin.x = 0 as COORD;
    s.origin.y = 0 as COORD;
    s.frame = SetAbsFrameIndex(MeleeFrame, which_icon);
    if resolutionFactor == 2 && melee_ui_active() && which_text > 0 && which_text < 4 {
        let mut panel: RECT = RECT { corner: POINT { x:0, y:0 }, extent: EXTENT { width:0, height:0 } };
        GetFrameRect(s.frame, &raw mut panel);
        panel.corner.x += s.origin.x;
        panel.corner.y += s.origin.y;
        uqm_lua_checked!(DrawStarConBox(&raw mut panel, 1,
            if HiLite.0 != 0 { Color { r:141,g:225,b:223,a:255 } } else { Color { r:49,g:79,b:96,a:255 } },
            Color { r:28,g:48,b:61,a:255 }, BOOLEAN::TRUE,
            if HiLite.0 != 0 { Color { r:28,g:57,b:73,a:255 } } else { Color { r:20,g:37,b:47,a:255 } },
            BOOLEAN::FALSE, Color { r:80,g:80,b:80,a:0 }), ());
    } else {
        uqm_lua_checked!(DrawStamp(&raw mut s), ());
    }
    if which_text == 0 {
        return;
    }
    match which_text {
        1 => {
            uqm_lua_checked!(DrawControlText(s, which_icon, HiLite), ());
        }
        2 => {
            uqm_lua_checked!(DrawBattleText(s, which_icon, HiLite), ());
        }
        3 => {
            uqm_lua_checked!(DrawButtonText(s, which_icon, HiLite), ());
        }
        4 => {
            uqm_lua_checked!(DrawTeamPickerText(s), ());
        }
        _ => {}
    }
    if NeedBatch.0 != 0 {
        UnbatchGraphics();
    }
}
unsafe extern "C" fn GetShipIndex(mut row: BYTE, mut col: BYTE) -> FleetShipIndex {
    return (row as ::core::ffi::c_int
        * if if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            6 as ::core::ffi::c_int
        } else {
            7 as ::core::ffi::c_int
        } + col as ::core::ffi::c_int) as FleetShipIndex;
}
unsafe extern "C" fn GetShipRow(mut index: FleetShipIndex) -> BYTE {
    return (index as ::core::ffi::c_int
        / if if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            6 as ::core::ffi::c_int
        } else {
            7 as ::core::ffi::c_int
        }) as BYTE;
}
unsafe extern "C" fn GetShipColumn(mut index: ::core::ffi::c_int) -> BYTE {
    return (index
        % if if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            6 as ::core::ffi::c_int
        } else {
            7 as ::core::ffi::c_int
        }) as BYTE;
}
#[no_mangle]
pub unsafe extern "C" fn GetShipBox(
    mut pRect: *mut RECT,
    mut side: COUNT,
    mut row: COUNT,
    mut col: COUNT,
) {
    (*pRect).corner.x = ((2 as ::core::ffi::c_int
        + if if optWindowType == 0 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            2 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        } << resolutionFactor)
        + col as ::core::ffi::c_int
            * (((34 as ::core::ffi::c_int) << resolutionFactor)
                + ((1 as ::core::ffi::c_int) << resolutionFactor))) as COORD;
    (*pRect).corner.y = ((22 as ::core::ffi::c_int
        - if if optWindowType == 0 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        } << resolutionFactor)
        + side as ::core::ffi::c_int
            * ((22 as ::core::ffi::c_int
                - if if optWindowType == 0 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    1 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                } << resolutionFactor) + ((1 as ::core::ffi::c_int) << resolutionFactor)
                + 2 as ::core::ffi::c_int
                    * (((34 as ::core::ffi::c_int) << resolutionFactor)
                        + ((1 as ::core::ffi::c_int) << resolutionFactor)))
        + row as ::core::ffi::c_int
            * (((34 as ::core::ffi::c_int) << resolutionFactor)
                + ((1 as ::core::ffi::c_int) << resolutionFactor))) as COORD;
    (*pRect).extent.width = ((34 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    (*pRect).extent.height = ((34 as ::core::ffi::c_int) << resolutionFactor) as COORD;
}
unsafe extern "C" fn DrawShipBox(
    mut side: COUNT,
    mut index: FleetShipIndex,
    mut ship: MeleeShip,
    mut HiLite: BOOLEAN,
) {
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut row: BYTE = GetShipRow(index);
    let mut col: BYTE = GetShipColumn(index as ::core::ffi::c_int);
    let mut FilledSlot: BOOLEAN = BOOLEAN(
        (ship.0 != MeleeShip::MELEE_NONE.0) as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
    GetShipBox(&raw mut r, side, row as COUNT, col as COUNT);
    BatchGraphics();
    if if resolutionFactor != 2 as ::core::ffi::c_uint {
        BOOLEAN::FALSE.0 as ::core::ffi::c_int
    } else {
        BOOLEAN::TRUE.0 as ::core::ffi::c_int
    } != 0 && !melee_ui_active()
    {
        let mut s: STAMP = STAMP {
            origin: POINT { x: 0, y: 0 },
            frame: ::core::ptr::null_mut::<FRAME_DESC>(),
        };
        s.origin = r.corner;
        s.frame = SetAbsFrameIndex(
            MeleeFrame,
            ((GetFrameCount(MeleeFrame) as ::core::ffi::c_int - 4 as ::core::ffi::c_int)
                as ::core::ffi::c_uint)
                .wrapping_add(FilledSlot.0)
                .wrapping_add(HiLite.0 << 1 as ::core::ffi::c_int) as COUNT,
        );
        uqm_lua_checked!(DrawStamp(&raw mut s), ());
    } else if HiLite.0 != 0 {
        uqm_lua_checked!(DrawStarConBox(
            &raw mut r,
            1 as SIZE,
            colorFromRgb15(0x7 as BYTE, 0 as BYTE, 0xc as BYTE),
            colorFromRgb15(0xc as BYTE, 0 as BYTE, 0x14 as BYTE),
            FilledSlot,
            colorFromRgb15(0xa as BYTE, 0 as BYTE, 0x11 as BYTE),
            BOOLEAN::FALSE,
            buildColorRgba(0x50 as BYTE, 0x50 as BYTE, 0x50 as BYTE, 0 as BYTE),
        ), ());
    } else {
        uqm_lua_checked!(DrawStarConBox(
            &raw mut r,
            1 as SIZE,
            colorFromRgb15(0 as BYTE, 0 as BYTE, 0x9 as BYTE),
            colorFromRgb15(0 as BYTE, 0 as BYTE, 0xe as BYTE),
            FilledSlot,
            colorFromRgb15(0 as BYTE, 0 as BYTE, 0xc as BYTE),
            BOOLEAN::FALSE,
            buildColorRgba(0x50 as BYTE, 0x50 as BYTE, 0x50 as BYTE, 0 as BYTE),
        ), ());
    }
    if FilledSlot.0 != 0 {
        let mut s_0: STAMP = STAMP {
            origin: POINT { x: 0, y: 0 },
            frame: ::core::ptr::null_mut::<FRAME_DESC>(),
        };
        s_0.origin.x = (r.corner.x as ::core::ffi::c_int
            + (r.extent.width as ::core::ffi::c_int >> 1 as ::core::ffi::c_int))
            as COORD;
        s_0.origin.y = (r.corner.y as ::core::ffi::c_int
            + (r.extent.height as ::core::ffi::c_int >> 1 as ::core::ffi::c_int))
            as COORD;
        s_0.frame = GetShipMeleeIconsFromIndex(ship.0);
        uqm_lua_checked!(DrawStamp(&raw mut s_0), ());
    }
    UnbatchGraphics();
}
unsafe extern "C" fn ClearShipBox(mut side: COUNT, mut index: FleetShipIndex) {
    let mut rect: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut row: BYTE = GetShipRow(index);
    let mut col: BYTE = GetShipColumn(index as ::core::ffi::c_int);
    GetShipBox(&raw mut rect, side, row as COUNT, col as COUNT);
    uqm_lua_checked!(RepairMeleeFrame(&raw mut rect), ());
}
unsafe extern "C" fn DrawShipBoxCurrent(mut pMS: *mut MELEE_STATE, mut HiLite: BOOLEAN) {
    let mut slotI: FleetShipIndex = GetShipIndex((*pMS).row as BYTE, (*pMS).col as BYTE);
    let mut ship: MeleeShip = MeleeSetup_getShip(
        (*pMS).meleeSetup,
        (*pMS).side as size_t,
        slotI,
    );
    uqm_lua_checked!(DrawShipBox((*pMS).side, slotI, ship, HiLite), ());
}
unsafe extern "C" fn DrawControls(mut which_side: COUNT, mut HiLite: BOOLEAN) {
    let mut which_icon: COUNT = 0;
    if *(&raw mut PlayerControl as *mut BYTE).offset(which_side as isize)
        as ::core::ffi::c_int
        & ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as BYTE
            as ::core::ffi::c_int != 0
    {
        uqm_lua_checked!(DrawMeleeIcon(
            (C2Rust_Unnamed_7::NETWORK_CON_TOP.0 as ::core::ffi::c_int
                + if HiLite.0 != 0 {
                    1 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                }
                + 2 as ::core::ffi::c_int
                    * (1 as ::core::ffi::c_int - which_side as ::core::ffi::c_int))
                as COUNT,
            HiLite,
        ), ());
        return;
    }
    if *(&raw mut PlayerControl as *mut BYTE).offset(which_side as isize)
        as ::core::ffi::c_int
        & ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as BYTE
            as ::core::ffi::c_int != 0
    {
        which_icon = 0 as COUNT;
    } else {
        match *(&raw mut PlayerControl as *mut BYTE).offset(which_side as isize)
            as ::core::ffi::c_int
            & (((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as BYTE
                as ::core::ffi::c_int
                | ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as BYTE
                    as ::core::ffi::c_int
                | ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as BYTE
                    as ::core::ffi::c_int)
        {
            16 => {
                which_icon = 1 as COUNT;
            }
            32 => {
                which_icon = 2 as COUNT;
            }
            64 => {
                which_icon = 3 as COUNT;
            }
            _ => {
                which_icon = 0 as COUNT;
            }
        }
    }
    uqm_lua_checked!(DrawMeleeIcon(
        (1 as ::core::ffi::c_int
            + 8 as ::core::ffi::c_int
                * (1 as ::core::ffi::c_int - which_side as ::core::ffi::c_int)
            + if HiLite.0 != 0 {
                4 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } + which_icon as ::core::ffi::c_int) as COUNT,
        HiLite,
    ), ());
}
unsafe extern "C" fn DrawTeams() {
    let mut side: COUNT = 0;
    side = 0 as COUNT;
    while (side as ::core::ffi::c_int) < 2 as ::core::ffi::c_int {
        let mut index: FleetShipIndex = 0;
        uqm_lua_checked!(DrawControls(side, BOOLEAN::FALSE), ());
        index = 0 as FleetShipIndex;
        while (index as ::core::ffi::c_int) < 14 as ::core::ffi::c_int {
            let mut ship: MeleeShip = MeleeSetup_getShip(
                (*pMeleeState).meleeSetup,
                side as size_t,
                index,
            );
            if index as ::core::ffi::c_int
                == 2 as ::core::ffi::c_int
                    * if if optWindowType == 1 as DWORD {
                        BOOLEAN::TRUE.0 as ::core::ffi::c_int
                    } else {
                        BOOLEAN::FALSE.0 as ::core::ffi::c_int
                    } != 0
                    {
                        6 as ::core::ffi::c_int
                    } else {
                        7 as ::core::ffi::c_int
                    }
            {
                break;
            }
            uqm_lua_checked!(DrawShipBox(side, index, ship, BOOLEAN::FALSE), ());
            index = index.wrapping_add(1);
        }
        uqm_lua_checked!(DrawTeamString(
            pMeleeState,
            side,
            0 as COUNT,
            ::core::ptr::null::<::core::ffi::c_char>(),
        ), ());
        uqm_lua_checked!(DrawFleetValue(pMeleeState, side, 0 as COUNT), ());
        side = side.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn QuickRepair(mut whichFrame: COUNT, mut pRect: *mut RECT) {
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut OldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    let mut OldRect: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut oldOrigin: POINT = POINT { x: 0, y: 0 };
    r.corner.x = (*pRect).corner.x;
    r.corner.y = (*pRect).corner.y;
    r.extent = (*pRect).extent;
    OldContext = SetContext(SpaceContext);
    GetContextClipRect(&raw mut OldRect);
    SetContextClipRect(&raw mut r);
    if if optWindowType == 1 as DWORD {
        BOOLEAN::TRUE.0 as ::core::ffi::c_int
    } else {
        BOOLEAN::FALSE.0 as ::core::ffi::c_int
    } != 0
    {
        oldOrigin = SetContextOrigin(MAKE_POINT(r.corner.x, r.corner.y));
    } else {
        oldOrigin = SetContextOrigin(
            MAKE_POINT(
                -(r.corner.x as ::core::ffi::c_int) as COORD,
                -(r.corner.y as ::core::ffi::c_int) as COORD,
            ),
        );
    }
    uqm_lua_checked!(DrawMeleeIcon(whichFrame, BOOLEAN::FALSE), ());
    SetContextOrigin(oldOrigin);
    SetContextClipRect(&raw mut OldRect);
    SetContext(OldContext);
}
unsafe extern "C" fn DrawSuperMeleeTitle() {
    let mut t: TEXT = TEXT {
        baseline: POINT { x: 0, y: 0 },
        pStr: ::core::ptr::null::<UNICODE>(),
        align: TEXT_ALIGN::ALIGN_LEFT,
        CharCount: 0,
    };
    let mut OldFont: FONT = ::core::ptr::null_mut::<FONT_DESC>();
    let mut OldFontEffect: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
    let mut buf: *mut UNICODE = GetStringAddress(
        SetAbsStringTableIndex(
            GameStrings,
            (C2Rust_Unnamed_9::MELEE_STRING_BASE.0 as ::core::ffi::c_int
                + 9 as ::core::ffi::c_int) as COUNT,
        ),
    );
    if strlen(buf) == 0 as size_t {
        return;
    }
    OldFont = SetContextFont(
        uqm_lua_checked!(LoadGraphicInstance(b"font.melee.title\0".as_ptr() as RESOURCE), ()) as FONT,
    );
    let remastered = melee_ui_active();
    OldFontEffect = SetContextFontEffect(if remastered { ::core::ptr::null_mut() } else { SetAbsFrameIndex(FontGradFrame, 7 as COUNT) });
    let old_text_color = if remastered { Some(SetContextForeGroundColor(Color { r:214,g:232,b:240,a:255 })) } else { None };
    t.baseline.x = ((CanvasWidth >> 1 as ::core::ffi::c_int)
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
            } << resolutionFactor) - ((1 as ::core::ffi::c_int) << resolutionFactor)
        } else {
            0 as ::core::ffi::c_int
        }) as COORD;
    t.baseline.y = ((4 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    t.align = TEXT_ALIGN::ALIGN_CENTER;
    t.pStr = AlignText(buf as *const UNICODE, &raw mut t.baseline.x);
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    uqm_lua_checked!(font_DrawText(&raw mut t), ());
    SetContextFont(OldFont);
    SetContextFontEffect(OldFontEffect);
    if let Some(color) = old_text_color { SetContextForeGroundColor(color); }
}
unsafe extern "C" fn RepairMeleeArea(mut pRect: *const RECT) {
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut OldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    let mut OldRect: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut oldOrigin: POINT = POINT { x: 0, y: 0 };
    r.corner.x = ((*pRect).corner.x as ::core::ffi::c_int
        + (if if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            16 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        } << resolutionFactor)) as COORD;
    r.corner.y = ((*pRect).corner.y as ::core::ffi::c_int
        + (if if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            16 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        } << resolutionFactor)) as COORD;
    r.extent = (*pRect).extent;
    if r.corner.y as ::core::ffi::c_int & 1 as ::core::ffi::c_int != 0 {
        r.corner.y = (r.corner.y as ::core::ffi::c_int
            - ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        r.extent.height = (r.extent.height as ::core::ffi::c_int
            + ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    }
    OldContext = SetContext(SpaceContext);
    GetContextClipRect(&raw mut OldRect);
    SetContextClipRect(&raw mut r);
    oldOrigin = SetContextOrigin(
        MAKE_POINT(
            (-(r.corner.x as ::core::ffi::c_int)
                + (if if optWindowType == 1 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    16 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                } << resolutionFactor)) as COORD,
            (-(r.corner.y as ::core::ffi::c_int)
                + (if if optWindowType == 1 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    16 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                } << resolutionFactor)) as COORD,
        ),
    );
    BatchGraphics();
    uqm_lua_checked!(DrawMeleeIcon(
        C2Rust_Unnamed_7::MELEE_BACKGROUND.0 as ::core::ffi::c_int as COUNT,
        BOOLEAN::FALSE,
    ), ());
    uqm_lua_checked!(DrawTeams(), ());
    if (*pRect).corner.x as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        && (*pRect).corner.y as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        && (*pRect).extent.width as ::core::ffi::c_int == CanvasWidth
        && (*pRect).extent.height as ::core::ffi::c_int == CanvasHeight
    {
        uqm_lua_checked!(DrawSuperMeleeTitle(), ());
        uqm_lua_checked!(DrawMeleeIcon(
            C2Rust_Unnamed_7::LOAD_BUTTON_TOP.0 as ::core::ffi::c_int as COUNT,
            BOOLEAN::FALSE,
        ), ());
        uqm_lua_checked!(DrawMeleeIcon(
            C2Rust_Unnamed_7::SAVE_BUTTON_TOP.0 as ::core::ffi::c_int as COUNT,
            BOOLEAN::FALSE,
        ), ());
        uqm_lua_checked!(DrawMeleeIcon(
            C2Rust_Unnamed_7::LOAD_BUTTON_BOTT.0 as ::core::ffi::c_int as COUNT,
            BOOLEAN::FALSE,
        ), ());
        uqm_lua_checked!(DrawMeleeIcon(
            C2Rust_Unnamed_7::SAVE_BUTTON_BOTT.0 as ::core::ffi::c_int as COUNT,
            BOOLEAN::FALSE,
        ), ());
        uqm_lua_checked!(DrawMeleeIcon(
            C2Rust_Unnamed_7::QUIT_BUTTON.0 as ::core::ffi::c_int as COUNT,
            BOOLEAN::FALSE,
        ), ());
        uqm_lua_checked!(DrawMeleeIcon(
            C2Rust_Unnamed_7::NET_BUTTON_TOP.0 as ::core::ffi::c_int as COUNT,
            BOOLEAN::FALSE,
        ), ());
        uqm_lua_checked!(DrawMeleeIcon(
            C2Rust_Unnamed_7::BATTLE_BUTTON_HL.0 as ::core::ffi::c_int as COUNT,
            BOOLEAN::TRUE,
        ), ());
    }
    if (*pMeleeState).MeleeOption as ::core::ffi::c_int
        == C2Rust_Unnamed_10::BUILD_PICK.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(DrawPickFrame(pMeleeState), ());
    }
    UnbatchGraphics();
    SetContextOrigin(oldOrigin);
    SetContextClipRect(&raw mut OldRect);
    SetContext(OldContext);
}
#[no_mangle]
pub unsafe extern "C" fn RepairMeleeFrame(mut pRect: *const RECT) {
    BatchGraphics();
    uqm_lua_checked!(RepairMeleeArea(pRect), ());
    uqm_lua_checked!(RepairMeleeStatus(pRect), ());
    UnbatchGraphics();
}
unsafe extern "C" fn RedrawMeleeFrame() {
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    r.corner.x = 0 as COORD;
    r.corner.y = 0 as COORD;
    r.extent.width = CanvasWidth as COORD;
    r.extent.height = CanvasHeight as COORD;
    uqm_lua_checked!(RepairMeleeFrame(&raw mut r), ());
}
unsafe extern "C" fn GetTeamStringRect(mut side: COUNT, mut r: *mut RECT) {
    (*r).corner.x = ((2 as ::core::ffi::c_int
        + if if optWindowType == 0 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            2 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        } << resolutionFactor) - ((1 as ::core::ffi::c_int) << resolutionFactor))
        as COORD;
    (*r).corner.y = ((side as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
        * ((22 as ::core::ffi::c_int
            - if if optWindowType == 0 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } << resolutionFactor)
            + ((((34 as ::core::ffi::c_int) << resolutionFactor)
                + ((1 as ::core::ffi::c_int) << resolutionFactor))
                * 2 as ::core::ffi::c_int
                + ((2 as ::core::ffi::c_int) << resolutionFactor)))) as COORD;
    (*r).extent.width = (if if optWindowType == 1 as DWORD {
        BOOLEAN::TRUE.0 as ::core::ffi::c_int
    } else {
        BOOLEAN::FALSE.0 as ::core::ffi::c_int
    } != 0
    {
        6 as ::core::ffi::c_int
    } else {
        7 as ::core::ffi::c_int
    }
        * (((34 as ::core::ffi::c_int) << resolutionFactor)
            + ((1 as ::core::ffi::c_int) << resolutionFactor))
        - ((29 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    (*r).extent.height = ((13 as ::core::ffi::c_int) << resolutionFactor) as COORD;
}
unsafe extern "C" fn GetFleetValueRect(mut side: COUNT, mut r: *mut RECT) {
    (*r).corner.x = ((2 as ::core::ffi::c_int
        + if if optWindowType == 0 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            2 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        } << resolutionFactor)
        + if if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            6 as ::core::ffi::c_int
        } else {
            7 as ::core::ffi::c_int
        }
            * (((34 as ::core::ffi::c_int) << resolutionFactor)
                + ((1 as ::core::ffi::c_int) << resolutionFactor))
        - ((30 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    (*r).corner.y = ((side as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
        * ((22 as ::core::ffi::c_int
            - if if optWindowType == 0 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } << resolutionFactor)
            + ((((34 as ::core::ffi::c_int) << resolutionFactor)
                + ((1 as ::core::ffi::c_int) << resolutionFactor))
                * 2 as ::core::ffi::c_int
                + ((2 as ::core::ffi::c_int) << resolutionFactor)))) as COORD;
    (*r).extent.width = ((29 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    (*r).extent.height = ((13 as ::core::ffi::c_int) << resolutionFactor) as COORD;
}
unsafe extern "C" fn GetFullStringRect(mut side: COUNT, mut r: *mut RECT) {
    (*r).extent.width = ((245 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    (*r).extent.height = ((13 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    (*r).corner.x = ((1 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    (*r).corner.y = ((94 as ::core::ffi::c_int) << resolutionFactor
        << side as ::core::ffi::c_int) as COORD;
}
unsafe extern "C" fn DrawTeamStringsBackGround(mut side: COUNT) {
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    GetFullStringRect(side, &raw mut r);
    uqm_lua_checked!(QuickRepair(0 as COUNT, &raw mut r), ());
}
unsafe extern "C" fn DrawFleetValue(
    mut pMS: *mut MELEE_STATE,
    mut side: COUNT,
    mut HiLiteState: COUNT,
) {
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut rtText: TEXT = TEXT {
        baseline: POINT { x: 0, y: 0 },
        pStr: ::core::ptr::null::<UNICODE>(),
        align: TEXT_ALIGN::ALIGN_LEFT,
        CharCount: 0,
    };
    let mut buf: [UNICODE; 30] = [0; 30];
    let mut fleetValue: COUNT = 0;
    GetFleetValueRect(side, &raw mut r);
    if HiLiteState as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
        uqm_lua_checked!(RepairMeleeFrame(&raw mut r), ());
        return;
    }
    SetContextFont(MicroFont);
    fleetValue = MeleeSetup_getFleetValue((*pMS).meleeSetup, side as size_t);
    sprintf(
        &raw mut buf as *mut ::core::ffi::c_char,
        b"%u\0".as_ptr() as *const ::core::ffi::c_char,
        fleetValue as ::core::ffi::c_int,
    );
    rtText.pStr = &raw mut buf as *mut UNICODE;
    rtText.align = TEXT_ALIGN::ALIGN_RIGHT;
    rtText.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    rtText.baseline.y = (r.corner.y as ::core::ffi::c_int
        + r.extent.height as ::core::ffi::c_int
        - ((3 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    rtText.baseline.x = (r.corner.x as ::core::ffi::c_int
        + r.extent.width as ::core::ffi::c_int) as COORD;
    SetContextForeGroundColor(
        if HiLiteState as ::core::ffi::c_int & 2 as ::core::ffi::c_int == 0 {
            if if optWindowType == 0 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                buildColorRgba(0xaa as BYTE, 0 as BYTE, 0 as BYTE, 0xff as BYTE)
            } else {
                colorFromRgb15(0xf as BYTE, 0x10 as BYTE, 0x1b as BYTE)
            }
        } else if if optWindowType == 0 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            buildColorRgba(0xff as BYTE, 0x55 as BYTE, 0xff as BYTE, 0xff as BYTE)
        } else {
            colorFromRgb15(0x17 as BYTE, 0x18 as BYTE, 0x1d as BYTE)
        },
    );
    uqm_lua_checked!(font_DrawText(&raw mut rtText), ());
}
unsafe extern "C" fn DrawTeamString(
    mut pMS: *mut MELEE_STATE,
    mut side: COUNT,
    mut HiLiteState: COUNT,
    mut teamName: *const ::core::ffi::c_char,
) -> BOOLEAN {
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut lfText: TEXT = TEXT {
        baseline: POINT { x: 0, y: 0 },
        pStr: ::core::ptr::null::<UNICODE>(),
        align: TEXT_ALIGN::ALIGN_LEFT,
        CharCount: 0,
    };
    GetTeamStringRect(side, &raw mut r);
    if HiLiteState as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
        uqm_lua_checked!(RepairMeleeFrame(&raw mut r), BOOLEAN(0));
        return BOOLEAN::TRUE;
    }
    SetContextFont(MicroFont);
    lfText.pStr = (if !teamName.is_null() {
        teamName
    } else {
        MeleeSetup_getTeamName((*pMS).meleeSetup, side as size_t)
    }) as *const UNICODE;
    lfText.baseline.y = (r.corner.y as ::core::ffi::c_int
        + r.extent.height as ::core::ffi::c_int
        - ((3 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    lfText.baseline.x = (r.corner.x as ::core::ffi::c_int
        + ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    lfText.align = TEXT_ALIGN::ALIGN_LEFT;
    lfText.CharCount = strlen(lfText.pStr) as COUNT;
    BatchGraphics();
    if HiLiteState as ::core::ffi::c_int & 1 as ::core::ffi::c_int == 0 {
        SetContextForeGroundColor(
            if HiLiteState as ::core::ffi::c_int & 2 as ::core::ffi::c_int == 0 {
                if if optWindowType == 0 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    buildColorRgba(0xaa as BYTE, 0 as BYTE, 0 as BYTE, 0xff as BYTE)
                } else {
                    colorFromRgb15(0xf as BYTE, 0x10 as BYTE, 0x1b as BYTE)
                }
            } else if if optWindowType == 0 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                buildColorRgba(0xff as BYTE, 0x55 as BYTE, 0xff as BYTE, 0xff as BYTE)
            } else {
                colorFromRgb15(0x17 as BYTE, 0x18 as BYTE, 0x1d as BYTE)
            },
        );
        uqm_lua_checked!(font_DrawText(&raw mut lfText), BOOLEAN(0));
    } else {
        let mut i: COUNT = 0;
        let mut text_r: RECT = RECT {
            corner: POINT { x: 0, y: 0 },
            extent: EXTENT { width: 0, height: 0 },
        };
        let mut char_deltas: [BYTE; 30] = [0; 30];
        let mut pchar_deltas: *mut BYTE = ::core::ptr::null_mut::<BYTE>();
        TextRect(&raw mut lfText, &raw mut text_r, &raw mut char_deltas as *mut BYTE);
        text_r = r;
        SetContextForeGroundColor(colorFromRgb15(0x14 as BYTE, 0 as BYTE, 0x14 as BYTE));
        uqm_lua_checked!(DrawFilledRectangle(&raw mut text_r), BOOLEAN(0));
        pchar_deltas = &raw mut char_deltas as *mut BYTE;
        i = (*pMS).CurIndex;
        while i as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
            let c2rust_fresh0 = pchar_deltas;
            pchar_deltas = pchar_deltas.offset(1);
            text_r.corner.x = (text_r.corner.x as ::core::ffi::c_int
                + *c2rust_fresh0 as SIZE as ::core::ffi::c_int) as COORD;
            i = i.wrapping_sub(1);
        }
        if ((*pMS).CurIndex as ::core::ffi::c_int)
            < lfText.CharCount as ::core::ffi::c_int
        {
            text_r.corner.x = (text_r.corner.x as ::core::ffi::c_int
                - ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        }
        if HiLiteState as ::core::ffi::c_int & 8 as ::core::ffi::c_int != 0 {
            text_r.corner.y = (r.corner.y as ::core::ffi::c_int
                + ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
            text_r.extent.height = (r.extent.height as ::core::ffi::c_int
                - ((2 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
            SetCursorFlashBlock(BOOLEAN::TRUE);
            if (*pMS).CurIndex as ::core::ffi::c_int
                == lfText.CharCount as ::core::ffi::c_int
            {
                text_r.extent.width = ((1 as ::core::ffi::c_int) << resolutionFactor)
                    as COORD;
                text_r.corner.x = (text_r.corner.x as ::core::ffi::c_int
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
            } else if (*pMS).CurIndex as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                == lfText.CharCount as ::core::ffi::c_int
            {
                text_r.extent.width = (*pchar_deltas as SIZE as ::core::ffi::c_int
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
                text_r.corner.x = (text_r.corner.x as ::core::ffi::c_int
                    + ((2 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
            } else {
                text_r.extent.width = *pchar_deltas as SIZE as COORD;
                text_r.corner.x = (text_r.corner.x as ::core::ffi::c_int
                    + ((2 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
            }
            if text_r.extent.width as ::core::ffi::c_int >= 200 as ::core::ffi::c_int {
                text_r.extent.width = ((1 as ::core::ffi::c_int) << resolutionFactor)
                    as COORD;
                text_r.corner.x = (text_r.corner.x as ::core::ffi::c_int
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
            } else {
                SetContextForeGroundColor(
                    colorFromRgb15(0x1f as BYTE, 0x1f as BYTE, 0x1f as BYTE),
                );
                uqm_lua_checked!(DrawFilledRectangle(&raw mut text_r), BOOLEAN(0));
            }
        } else {
            text_r.corner.y = (r.corner.y as ::core::ffi::c_int
                + ((3 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
            text_r.extent.height = (r.extent.height as ::core::ffi::c_int
                - ((6 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
            text_r.extent.width = ((1 as ::core::ffi::c_int) << resolutionFactor)
                as COORD;
            if (*pMS).CurIndex as ::core::ffi::c_int
                == lfText.CharCount as ::core::ffi::c_int
            {
                text_r.corner.x = (text_r.corner.x as ::core::ffi::c_int
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
            }
            SetCursorFlashBlock(BOOLEAN::FALSE);
        }
        text_r.corner.x = (text_r.corner.x as ::core::ffi::c_int
            + ((1 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        SetCursorRect(&raw mut text_r, SpaceContext);
        SetContextForeGroundColor(
            if if optWindowType == 0 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                buildColorRgba(0xff as BYTE, 0x55 as BYTE, 0xff as BYTE, 0xff as BYTE)
            } else {
                colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE)
            },
        );
        uqm_lua_checked!(font_DrawText(&raw mut lfText), BOOLEAN(0));
    }
    UnbatchGraphics();
    return BOOLEAN::TRUE;
}
static mut meleeStatusText: [::core::ffi::c_char; 256] = [0; 256];
unsafe extern "C" fn multiLineDrawText(mut textIn: *mut TEXT, mut clipRect: *mut RECT) {
    let mut oldRect: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut leading: SIZE = 0;
    let mut text: TEXT = TEXT {
        baseline: POINT { x: 0, y: 0 },
        pStr: ::core::ptr::null::<UNICODE>(),
        align: TEXT_ALIGN::ALIGN_LEFT,
        CharCount: 0,
    };
    let mut lineWidth: SIZE = 0;
    GetContextClipRect(&raw mut oldRect);
    SetContextClipRect(clipRect);
    GetContextFontLeading(&raw mut leading);
    text = *textIn;
    text.baseline.x = ((1 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    text.baseline.y = 0 as COORD;
    if (*clipRect).extent.width as ::core::ffi::c_int
        > text.baseline.x as ::core::ffi::c_int
    {
        lineWidth = ((*clipRect).extent.width as ::core::ffi::c_int
            - text.baseline.x as ::core::ffi::c_int) as SIZE;
        while *text.pStr as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
            let mut nextLine: *const ::core::ffi::c_char = ::core::ptr::null::<
                ::core::ffi::c_char,
            >();
            text.baseline.y = (text.baseline.y as ::core::ffi::c_int
                + leading as ::core::ffi::c_int) as COORD;
            text.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
            getLineWithinWidth(
                &raw mut text,
                &raw mut nextLine,
                lineWidth,
                text.CharCount,
            );
            uqm_lua_checked!(font_DrawText(&raw mut text), ());
            text.pStr = nextLine as *const UNICODE;
        }
    }
    SetContextClipRect(&raw mut oldRect);
}
unsafe extern "C" fn GetMeleeStatusRect(mut r: *mut RECT) {
    let mut bottom: COORD = (CanvasHeight
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
    (*r).corner.x = ((1 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    (*r).corner.y = ((201 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    if (*r).corner.y as ::core::ffi::c_int
        > bottom as ::core::ffi::c_int - ((25 as ::core::ffi::c_int) << resolutionFactor)
    {
        (*r).corner.y = (bottom as ::core::ffi::c_int
            - ((25 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    }
    (*r).extent.width = (if if optWindowType == 1 as DWORD {
        BOOLEAN::TRUE.0 as ::core::ffi::c_int
    } else {
        BOOLEAN::FALSE.0 as ::core::ffi::c_int
    } != 0
    {
        6 as ::core::ffi::c_int
    } else {
        7 as ::core::ffi::c_int
    }
        * (((34 as ::core::ffi::c_int) << resolutionFactor)
            + ((1 as ::core::ffi::c_int) << resolutionFactor))) as COORD;
    (*r).extent.height = ((38 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    if (*r).corner.y as ::core::ffi::c_int + (*r).extent.height as ::core::ffi::c_int
        > bottom as ::core::ffi::c_int
    {
        (*r).extent.height = (bottom as ::core::ffi::c_int
            - (*r).corner.y as ::core::ffi::c_int) as COORD;
    }
}
unsafe extern "C" fn DrawMeleeStatusText() {
    let mut oldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut lfText: TEXT = TEXT {
        baseline: POINT { x: 0, y: 0 },
        pStr: ::core::ptr::null::<UNICODE>(),
        align: TEXT_ALIGN::ALIGN_LEFT,
        CharCount: 0,
    };
    if meleeStatusText[0usize] as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
        return;
    }
    oldContext = SetContext(SpaceContext);
    GetMeleeStatusRect(&raw mut r);
    r.corner.x = (r.corner.x as ::core::ffi::c_int
        + (if if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            16 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        } << resolutionFactor)) as COORD;
    r.corner.y = (r.corner.y as ::core::ffi::c_int
        + (if if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            16 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        } << resolutionFactor)) as COORD;
    lfText.pStr = &raw mut meleeStatusText as *mut ::core::ffi::c_char;
    lfText.align = TEXT_ALIGN::ALIGN_LEFT;
    lfText.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    SetContextFont(MicroFont);
    SetContextForeGroundColor(colorFromRgb15(0 as BYTE, 0x14 as BYTE, 0 as BYTE));
    BatchGraphics();
    uqm_lua_checked!(multiLineDrawText(&raw mut lfText, &raw mut r), ());
    UnbatchGraphics();
    SetContext(oldContext);
}
unsafe extern "C" fn RepairMeleeStatus(mut pRect: *const RECT) {
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    if (*pRect).corner.x as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        && (*pRect).corner.y as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        && (*pRect).extent.width as ::core::ffi::c_int == CanvasWidth
        && (*pRect).extent.height as ::core::ffi::c_int == CanvasHeight
    {
        meleeStatusText[0usize] = '\0' as ::core::ffi::c_char;
    }
    GetMeleeStatusRect(&raw mut r);
    if meleeStatusText[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        && ((*pRect).corner.x as ::core::ffi::c_int != r.corner.x as ::core::ffi::c_int
            || (*pRect).corner.y as ::core::ffi::c_int
                != r.corner.y as ::core::ffi::c_int
            || (*pRect).extent.width as ::core::ffi::c_int
                != r.extent.width as ::core::ffi::c_int
            || (*pRect).extent.height as ::core::ffi::c_int
                != r.extent.height as ::core::ffi::c_int)
    {
        uqm_lua_checked!(RepairMeleeArea(&raw mut r), ());
    }
    uqm_lua_checked!(DrawBorderPadding(0 as DWORD), ());
    uqm_lua_checked!(DrawMeleeStatusText(), ());
}
unsafe extern "C" fn DrawMeleeStatusMessage(mut message: *const ::core::ffi::c_char) {
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    utf8StringCopy(
        &raw mut meleeStatusText as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
        message,
    );
    GetMeleeStatusRect(&raw mut r);
    uqm_lua_checked!(RepairMeleeFrame(&raw mut r), ());
}
unsafe extern "C" fn UpdateMeleeStatusMessage(mut player: ssize_t) {
    let mut conn: *mut NetConnection = ::core::ptr::null_mut::<NetConnection>();
    (player == -1 as ssize_t || player >= 0 as ssize_t && player < 2 as ssize_t
        || {
            __assert_fail(
                b"player == -1 || (player >= 0 && player < NUM_PLAYERS)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/uqm/supermelee/melee.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                1358 as ::core::ffi::c_int,
                b"UpdateMeleeStatusMessage\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    if player == -1 as ssize_t {
        uqm_lua_checked!(DrawMeleeStatusMessage(b"\0".as_ptr() as *const ::core::ffi::c_char), ());
        return;
    }
    conn = *(&raw mut netConnections as *mut *mut NetConnection).offset(player as isize)
        as *mut NetConnection;
    if conn.is_null() {
        uqm_lua_checked!(DrawMeleeStatusMessage(
            GetStringAddress(
                SetAbsStringTableIndex(
                    GameStrings,
                    (C2Rust_Unnamed_9::NETMELEE_STRING_BASE.0 as ::core::ffi::c_int
                        + 0 as ::core::ffi::c_int) as COUNT,
                ),
            ),
        ), ());
        return;
    }
    match NetConnection_getState(conn) {
        NetState::NetState_unconnected => {
            uqm_lua_checked!(DrawMeleeStatusMessage(
                GetStringAddress(
                    SetAbsStringTableIndex(
                        GameStrings,
                        (C2Rust_Unnamed_9::NETMELEE_STRING_BASE.0 as ::core::ffi::c_int
                            + 0 as ::core::ffi::c_int) as COUNT,
                    ),
                ),
            ), ());
        }
        NetState::NetState_connecting => {
            if (*NetConnection_getPeerOptions(conn)).isServer {
                uqm_lua_checked!(DrawMeleeStatusMessage(
                    GetStringAddress(
                        SetAbsStringTableIndex(
                            GameStrings,
                            (C2Rust_Unnamed_9::NETMELEE_STRING_BASE.0
                                as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as COUNT,
                        ),
                    ),
                ), ());
            } else {
                uqm_lua_checked!(DrawMeleeStatusMessage(
                    GetStringAddress(
                        SetAbsStringTableIndex(
                            GameStrings,
                            (C2Rust_Unnamed_9::NETMELEE_STRING_BASE.0
                                as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as COUNT,
                        ),
                    ),
                ), ());
            }
        }
        NetState::NetState_init | NetState::NetState_inSetup => {
            uqm_lua_checked!(DrawMeleeStatusMessage(
                GetStringAddress(
                    SetAbsStringTableIndex(
                        GameStrings,
                        (C2Rust_Unnamed_9::NETMELEE_STRING_BASE.0 as ::core::ffi::c_int
                            + 3 as ::core::ffi::c_int) as COUNT,
                    ),
                ),
            ), ());
        }
        _ => {
            uqm_lua_checked!(DrawMeleeStatusMessage(b"\0".as_ptr() as *const ::core::ffi::c_char), ());
        }
    };
}
unsafe extern "C" fn Deselect(mut opt: BYTE) {
    match opt as ::core::ffi::c_int {
        4 => {
            uqm_lua_checked!(DrawMeleeIcon(
                C2Rust_Unnamed_7::BATTLE_BUTTON.0 as ::core::ffi::c_int as COUNT,
                BOOLEAN::FALSE,
            ), ());
        }
        3 => {
            uqm_lua_checked!(DrawMeleeIcon(
                C2Rust_Unnamed_7::LOAD_BUTTON_TOP.0 as ::core::ffi::c_int as COUNT,
                BOOLEAN::FALSE,
            ), ());
        }
        5 => {
            uqm_lua_checked!(DrawMeleeIcon(
                C2Rust_Unnamed_7::LOAD_BUTTON_BOTT.0 as ::core::ffi::c_int as COUNT,
                BOOLEAN::FALSE,
            ), ());
        }
        2 => {
            uqm_lua_checked!(DrawMeleeIcon(
                C2Rust_Unnamed_7::SAVE_BUTTON_TOP.0 as ::core::ffi::c_int as COUNT,
                BOOLEAN::FALSE,
            ), ());
        }
        6 => {
            uqm_lua_checked!(DrawMeleeIcon(
                C2Rust_Unnamed_7::SAVE_BUTTON_BOTT.0 as ::core::ffi::c_int as COUNT,
                BOOLEAN::FALSE,
            ), ());
        }
        0 => {
            uqm_lua_checked!(DrawMeleeIcon(
                C2Rust_Unnamed_7::NET_BUTTON_TOP.0 as ::core::ffi::c_int as COUNT,
                BOOLEAN::FALSE,
            ), ());
        }
        8 => {
            uqm_lua_checked!(DrawMeleeIcon(
                C2Rust_Unnamed_7::QUIT_BUTTON.0 as ::core::ffi::c_int as COUNT,
                BOOLEAN::FALSE,
            ), ());
        }
        1 | 7 => {
            let mut which_side: COUNT = 0;
            which_side = (if opt as ::core::ffi::c_int
                == C2Rust_Unnamed_10::CONTROLS_TOP.0 as ::core::ffi::c_int
            {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as COUNT;
            uqm_lua_checked!(DrawControls(which_side, BOOLEAN::FALSE), ());
        }
        9 => {
            if (*pMeleeState).InputFunc
                == Some(DoEdit as unsafe extern "C" fn(*mut MELEE_STATE) -> BOOLEAN)
            {
                if ((*pMeleeState).row as ::core::ffi::c_int) < 2 as ::core::ffi::c_int {
                    uqm_lua_checked!(DrawShipBoxCurrent(pMeleeState, BOOLEAN::FALSE), ());
                } else if (*pMeleeState).CurIndex as ::core::ffi::c_int
                    == -1 as ::core::ffi::c_int as COUNT as ::core::ffi::c_int
                {
                    if if resolutionFactor != 2 as ::core::ffi::c_uint {
                        BOOLEAN::FALSE.0 as ::core::ffi::c_int
                    } else {
                        BOOLEAN::TRUE.0 as ::core::ffi::c_int
                    } != 0
                    {
                        uqm_lua_checked!(DrawTeamStringsBackGround((*pMeleeState).side), ());
                    }
                    uqm_lua_checked!(DrawTeamString(
                        pMeleeState,
                        (*pMeleeState).side,
                        0 as COUNT,
                        ::core::ptr::null::<::core::ffi::c_char>(),
                    ), ());
                    uqm_lua_checked!(DrawFleetValue(pMeleeState, (*pMeleeState).side, 0 as COUNT), ());
                }
            }
        }
        10 => {
            uqm_lua_checked!(DrawPickIcon((*pMeleeState).currentShip, false), ());
        }
        _ => {}
    };
}
unsafe extern "C" fn Select(mut opt: BYTE) {
    match opt as ::core::ffi::c_int {
        4 => {
            uqm_lua_checked!(DrawMeleeIcon(
                C2Rust_Unnamed_7::BATTLE_BUTTON_HL.0 as ::core::ffi::c_int as COUNT,
                BOOLEAN::TRUE,
            ), ());
        }
        3 => {
            uqm_lua_checked!(DrawMeleeIcon(
                C2Rust_Unnamed_7::LOAD_BUTTON_TOP_HL.0 as ::core::ffi::c_int as COUNT,
                BOOLEAN::TRUE,
            ), ());
        }
        5 => {
            uqm_lua_checked!(DrawMeleeIcon(
                C2Rust_Unnamed_7::LOAD_BUTTON_BOTT_HL.0 as ::core::ffi::c_int as COUNT,
                BOOLEAN::TRUE,
            ), ());
        }
        2 => {
            uqm_lua_checked!(DrawMeleeIcon(
                C2Rust_Unnamed_7::SAVE_BUTTON_TOP_HL.0 as ::core::ffi::c_int as COUNT,
                BOOLEAN::TRUE,
            ), ());
        }
        6 => {
            uqm_lua_checked!(DrawMeleeIcon(
                C2Rust_Unnamed_7::SAVE_BUTTON_BOTT_HL.0 as ::core::ffi::c_int as COUNT,
                BOOLEAN::TRUE,
            ), ());
        }
        0 => {
            uqm_lua_checked!(DrawMeleeIcon(
                C2Rust_Unnamed_7::NET_BUTTON_TOP_HL.0 as ::core::ffi::c_int as COUNT,
                BOOLEAN::TRUE,
            ), ());
        }
        8 => {
            uqm_lua_checked!(DrawMeleeIcon(
                C2Rust_Unnamed_7::QUIT_BUTTON_HL.0 as ::core::ffi::c_int as COUNT,
                BOOLEAN::TRUE,
            ), ());
        }
        1 | 7 => {
            let mut which_side: COUNT = 0;
            which_side = (if opt as ::core::ffi::c_int
                == C2Rust_Unnamed_10::CONTROLS_TOP.0 as ::core::ffi::c_int
            {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as COUNT;
            uqm_lua_checked!(DrawControls(which_side, BOOLEAN::TRUE), ());
        }
        9 => {
            if (*pMeleeState).InputFunc
                == Some(DoEdit as unsafe extern "C" fn(*mut MELEE_STATE) -> BOOLEAN)
            {
                if ((*pMeleeState).row as ::core::ffi::c_int) < 2 as ::core::ffi::c_int {
                    uqm_lua_checked!(DrawShipBoxCurrent(pMeleeState, BOOLEAN::TRUE), ());
                } else if (*pMeleeState).CurIndex as ::core::ffi::c_int
                    == -1 as ::core::ffi::c_int as COUNT as ::core::ffi::c_int
                {
                    if if resolutionFactor != 2 as ::core::ffi::c_uint {
                        BOOLEAN::FALSE.0 as ::core::ffi::c_int
                    } else {
                        BOOLEAN::TRUE.0 as ::core::ffi::c_int
                    } != 0
                    {
                        uqm_lua_checked!(DrawTeamStringsBackGround((*pMeleeState).side), ());
                    }
                    uqm_lua_checked!(DrawTeamString(
                        pMeleeState,
                        (*pMeleeState).side,
                        2 as COUNT,
                        ::core::ptr::null::<::core::ffi::c_char>(),
                    ), ());
                    uqm_lua_checked!(DrawFleetValue(pMeleeState, (*pMeleeState).side, 2 as COUNT), ());
                }
            }
        }
        10 => {
            uqm_lua_checked!(DrawPickIcon(
                (*pMeleeState).currentShip,
                1 as ::core::ffi::c_uint & is3DO(optWhichMenu).0 != 0,
            ), ());
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn Melee_flashSelection(mut pMS: *mut MELEE_STATE) {
    static mut NextTime: TimeCount = 0 as TimeCount;
    static mut select: bool = false;
    let mut Now: TimeCount = GetTimeCounter();
    if Now >= NextTime {
        let mut OldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
        NextTime = Now
            .wrapping_add(
                (if (*pMS).MeleeOption as ::core::ffi::c_int
                    != C2Rust_Unnamed_10::BUILD_PICK.0 as ::core::ffi::c_int
                    || is3DO(optWhichMenu).0 != 0
                {
                    840 as ::core::ffi::c_int / 9 as ::core::ffi::c_int
                } else {
                    840 as ::core::ffi::c_int / 16 as ::core::ffi::c_int
                }) as TimeCount,
            );
        select = !select;
        OldContext = SetContext(SpaceContext);
        if select {
            uqm_lua_checked!(Select((*pMS).MeleeOption), ());
        } else {
            uqm_lua_checked!(Deselect((*pMS).MeleeOption), ());
        }
        SetContext(OldContext);
    }
}
unsafe extern "C" fn InitMelee(mut pMS: *mut MELEE_STATE) {
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    SetContext(SpaceContext);
    SetContextFGFrame(Screen);
    SetContextClipRect(::core::ptr::null_mut::<RECT>());
    SetContextBackGroundColor(colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE));
    uqm_lua_checked!(ClearDrawable(), ());
    r.corner.x = (if if optWindowType == 1 as DWORD {
        BOOLEAN::TRUE.0 as ::core::ffi::c_int
    } else {
        BOOLEAN::FALSE.0 as ::core::ffi::c_int
    } != 0
    {
        16 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    } << resolutionFactor) as COORD;
    r.corner.y = (if if optWindowType == 1 as DWORD {
        BOOLEAN::TRUE.0 as ::core::ffi::c_int
    } else {
        BOOLEAN::FALSE.0 as ::core::ffi::c_int
    } != 0
    {
        16 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    } << resolutionFactor) as COORD;
    r.extent.width = (CanvasWidth
        - (if if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            16 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        } << resolutionFactor << 1 as ::core::ffi::c_int)) as COORD;
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
        } << resolutionFactor << 1 as ::core::ffi::c_int)) as COORD;
    SetContextClipRect(&raw mut r);
    r.corner.y = 0 as COORD;
    r.corner.x = r.corner.y;
    uqm_lua_checked!(RedrawMeleeFrame(), ());
}
#[no_mangle]
pub unsafe extern "C" fn DrawMeleeShipStrings(
    mut pMS: *mut MELEE_STATE,
    mut NewStarShip: MeleeShip,
) {
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut OldRect: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut OldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    OldContext = SetContext(StatusContext);
    GetContextClipRect(&raw mut OldRect);
    r = OldRect;
    r.corner.x = (r.corner.x as ::core::ffi::c_int
        - (if if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            0 as ::core::ffi::c_int
        } else {
            3 as ::core::ffi::c_int
        } << resolutionFactor)) as COORD;
    r.corner.y = (r.corner.y as ::core::ffi::c_int
        + ((76 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    r.extent.height = ((65 as ::core::ffi::c_int) << resolutionFactor) as COORD;
    SetContextClipRect(&raw mut r);
    BatchGraphics();
    if NewStarShip.0 == MeleeShip::MELEE_NONE.0 {
        let mut r_0: RECT = RECT {
            corner: POINT { x: 0, y: 0 },
            extent: EXTENT { width: 0, height: 0 },
        };
        let mut t: TEXT = TEXT {
            baseline: POINT { x: 0, y: 0 },
            pStr: ::core::ptr::null::<UNICODE>(),
            align: TEXT_ALIGN::ALIGN_LEFT,
            CharCount: 0,
        };
        uqm_lua_checked!(ClearShipStatus(0 as COORD), ());
        SetContextFont(StarConFont);
        r_0.corner.x = ((3 as ::core::ffi::c_int) << resolutionFactor) as COORD;
        r_0.corner.y = ((4 as ::core::ffi::c_int) << resolutionFactor) as COORD;
        r_0.extent.width = ((57 as ::core::ffi::c_int) << resolutionFactor) as COORD;
        r_0.extent.height = ((60 as ::core::ffi::c_int) << resolutionFactor) as COORD;
        SetContextForeGroundColor(colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE));
        uqm_lua_checked!(DrawRectangle(
            &raw mut r_0,
            BOOLEAN(
                (if resolutionFactor != 2 as ::core::ffi::c_uint {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                }) as ::core::ffi::c_uint,
            ),
        ), ());
        t.baseline.x = ((64 as ::core::ffi::c_int) << resolutionFactor
            >> 1 as ::core::ffi::c_int) as COORD;
        t.baseline.y = ((32 as ::core::ffi::c_int) << resolutionFactor) as COORD;
        t.align = TEXT_ALIGN::ALIGN_CENTER;
        if ((*pMS).row as ::core::ffi::c_int) < 2 as ::core::ffi::c_int {
            t.pStr = GetStringAddress(
                SetAbsStringTableIndex(
                    GameStrings,
                    (C2Rust_Unnamed_9::MELEE_STRING_BASE.0 as ::core::ffi::c_int
                        + 0 as ::core::ffi::c_int) as COUNT,
                ),
            );
            t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
            uqm_lua_checked!(font_DrawText(&raw mut t), ());
            t.pStr = GetStringAddress(
                SetAbsStringTableIndex(
                    GameStrings,
                    (C2Rust_Unnamed_9::MELEE_STRING_BASE.0 as ::core::ffi::c_int
                        + 1 as ::core::ffi::c_int) as COUNT,
                ),
            );
        } else {
            t.pStr = GetStringAddress(
                SetAbsStringTableIndex(
                    GameStrings,
                    (C2Rust_Unnamed_9::MELEE_STRING_BASE.0 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int) as COUNT,
                ),
            );
            t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
            uqm_lua_checked!(font_DrawText(&raw mut t), ());
            t.pStr = GetStringAddress(
                SetAbsStringTableIndex(
                    GameStrings,
                    (C2Rust_Unnamed_9::MELEE_STRING_BASE.0 as ::core::ffi::c_int
                        + 3 as ::core::ffi::c_int) as COUNT,
                ),
            );
        }
        t.baseline.y = (t.baseline.y as ::core::ffi::c_int
            + ((9 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
        uqm_lua_checked!(font_DrawText(&raw mut t), ());
    } else {
        let mut hMasterShip: HMASTERSHIP = ::core::ptr::null_mut::<
            ::core::ffi::c_void,
        >();
        let mut MasterPtr: *mut MASTER_SHIP_INFO = ::core::ptr::null_mut::<
            MASTER_SHIP_INFO,
        >();
        hMasterShip = GetStarShipFromIndex(&raw mut master_q, NewStarShip.0 as COUNT)
            as HMASTERSHIP;
        MasterPtr = LockMasterShip(&raw mut master_q, hMasterShip);
        uqm_lua_checked!(InitShipStatus(
            &raw mut (*MasterPtr).ShipInfo,
            ::core::ptr::null_mut::<STARSHIP>(),
            ::core::ptr::null_mut::<RECT>(),
            BOOLEAN::TRUE,
        ), ());
        if optMeleeToolTips.0 != 0
            && (*pMS).MeleeOption as ::core::ffi::c_int
                == C2Rust_Unnamed_10::BUILD_PICK.0 as ::core::ffi::c_int
        {
            uqm_lua_checked!(DrawTooltip(&raw mut (*MasterPtr).ShipInfo), ());
        }
        UnlockLink(&raw mut master_q, hMasterShip);
    }
    UnbatchGraphics();
    SetContextClipRect(&raw mut OldRect);
    SetContext(OldContext);
}
unsafe extern "C" fn UpdateCurrentShip(mut pMS: *mut MELEE_STATE) {
    if (*pMS).row as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
        (*pMS).currentShip = MeleeShip::MELEE_NONE;
    } else {
        let mut slotNr: FleetShipIndex = GetShipIndex(
            (*pMS).row as BYTE,
            (*pMS).col as BYTE,
        );
        (*pMS).currentShip = MeleeSetup_getShip(
            (*pMS).meleeSetup,
            (*pMS).side as size_t,
            slotNr,
        );
    }
    uqm_lua_checked!(DrawMeleeShipStrings(pMS, (*pMS).currentShip), ());
}
#[no_mangle]
pub unsafe extern "C" fn GetShipValue(mut StarShip: MeleeShip) -> COUNT {
    let mut val: COUNT = 0;
    if StarShip.0 == MeleeShip::MELEE_NONE.0 {
        return 0 as COUNT;
    }
    val = GetShipCostFromIndex(StarShip.0);
    if val as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        val = !(0 as ::core::ffi::c_int) as COUNT;
    }
    return val;
}
unsafe extern "C" fn DeleteCurrentShip(mut pMS: *mut MELEE_STATE) {
    let mut slotI: FleetShipIndex = GetShipIndex((*pMS).row as BYTE, (*pMS).col as BYTE);
    uqm_lua_checked!(Melee_LocalChange_ship(pMS, (*pMS).side, slotI, MeleeShip::MELEE_NONE), ());
}
unsafe extern "C" fn isShipSlotSelected(
    mut pMS: *mut MELEE_STATE,
    mut side: COUNT,
    mut index: FleetShipIndex,
) -> bool {
    if (*pMS).MeleeOption as ::core::ffi::c_int
        != C2Rust_Unnamed_10::EDIT_MELEE.0 as ::core::ffi::c_int
    {
        return false;
    }
    if (*pMS).side as ::core::ffi::c_int != side as ::core::ffi::c_int {
        return false;
    }
    return index as ::core::ffi::c_int
        == GetShipIndex((*pMS).row as BYTE, (*pMS).col as BYTE) as ::core::ffi::c_int;
}
unsafe extern "C" fn AdvanceCursor(mut pMS: *mut MELEE_STATE) {
    (*pMS).col = (*pMS).col.wrapping_add(1);
    if (*pMS).col as ::core::ffi::c_int
        == if if optWindowType == 1 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            6 as ::core::ffi::c_int
        } else {
            7 as ::core::ffi::c_int
        }
    {
        (*pMS).row = (*pMS).row.wrapping_add(1);
        if ((*pMS).row as ::core::ffi::c_int) < 2 as ::core::ffi::c_int {
            (*pMS).col = 0 as COUNT;
        } else {
            (*pMS).col = (if if optWindowType == 1 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                6 as ::core::ffi::c_int
            } else {
                7 as ::core::ffi::c_int
            } - 1 as ::core::ffi::c_int) as COUNT;
            (*pMS).row = (2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as COUNT;
        }
    }
}
unsafe extern "C" fn OnTeamNameChange(mut pTES: *mut TEXTENTRY_STATE) -> BOOLEAN {
    let mut pMS: *mut MELEE_STATE = (*pTES).CbParam as *mut MELEE_STATE;
    let mut ret: BOOLEAN = BOOLEAN::FALSE;
    let mut hl: COUNT = 1 as COUNT;
    (*pMS).CurIndex = (*pTES).CursorPos as COUNT;
    if (*pTES).JoystickMode.0 != 0 {
        hl = (hl as ::core::ffi::c_int | 8 as ::core::ffi::c_int) as COUNT;
    }
    ret = uqm_lua_checked!(DrawTeamString(pMS, (*pMS).side, hl, (*pTES).BaseStr), BOOLEAN(0));
    return ret;
}
unsafe extern "C" fn TeamNameFrameCallback(mut pTES: *mut TEXTENTRY_STATE) -> BOOLEAN {
    uqm_lua_checked!(netInput(), BOOLEAN(0));
    if webNetplayRole >= 0 as ::core::ffi::c_int
        && GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
            & C2Rust_Unnamed_0::CHECK_ABORT.0 as ::core::ffi::c_int != 0
    {
        return BOOLEAN::FALSE;
    }
    return BOOLEAN::TRUE;
}
unsafe extern "C" fn BuildPickShipPopup(mut pMS: *mut MELEE_STATE) {
    let mut buildOk: bool = false;
    (*pMS).MeleeOption = C2Rust_Unnamed_10::BUILD_PICK.0 as ::core::ffi::c_int
        as MELEE_OPTIONS;
    buildOk = uqm_lua_checked!(BuildPickShip(pMS), ());
    if buildOk {
        let mut index: FleetShipIndex = GetShipIndex(
            (*pMS).row as BYTE,
            (*pMS).col as BYTE,
        );
        uqm_lua_checked!(Melee_LocalChange_ship(pMS, (*pMS).side, index, (*pMS).currentShip), ());
        AdvanceCursor(pMS);
    }
    (*pMS).MeleeOption = C2Rust_Unnamed_10::EDIT_MELEE.0 as ::core::ffi::c_int
        as MELEE_OPTIONS;
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    GetBuildPickFrameRect(&raw mut r);
    uqm_lua_checked!(RepairMeleeFrame(&raw mut r), ());
    if optMeleeToolTips.0 != 0 {
        GetToolTipFrameRect(&raw mut r);
        uqm_lua_checked!(RepairMeleeFrame(&raw mut r), ());
    }
    uqm_lua_checked!(UpdateCurrentShip(pMS), ());
    (*pMS).InputFunc = Some(DoEdit as unsafe extern "C" fn(*mut MELEE_STATE) -> BOOLEAN)
        as Option<unsafe extern "C" fn(*mut melee_state) -> BOOLEAN>;
}
unsafe extern "C" fn DoEdit(mut pMS: *mut MELEE_STATE) -> BOOLEAN {
    let mut TimeIn: DWORD = GetTimeCounter();
    ::core::ptr::write_volatile(&raw mut GamePaused, BOOLEAN::FALSE);
    if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
        & C2Rust_Unnamed_0::CHECK_ABORT.0 as ::core::ffi::c_int != 0
    {
        return BOOLEAN::FALSE;
    }
    SetMenuSounds(
        (((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as MENU_SOUND_FLAGS
            as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as MENU_SOUND_FLAGS
                as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as MENU_SOUND_FLAGS
                as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as MENU_SOUND_FLAGS
                as ::core::ffi::c_int) as MENU_SOUND_FLAGS,
        (((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as MENU_SOUND_FLAGS
            as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int) as MENU_SOUND_FLAGS
                as ::core::ffi::c_int) as MENU_SOUND_FLAGS,
    );
    if (*pMS).Initialized.0 == 0 {
        uqm_lua_checked!(UpdateCurrentShip(pMS), BOOLEAN(0));
        (*pMS).Initialized = BOOLEAN::TRUE;
        (*pMS).InputFunc = Some(
            DoEdit as unsafe extern "C" fn(*mut MELEE_STATE) -> BOOLEAN,
        ) as Option<unsafe extern "C" fn(*mut melee_state) -> BOOLEAN>;
        return BOOLEAN::TRUE;
    }
    uqm_lua_checked!(netInput(), BOOLEAN(0));
    if webNetplayRole >= 0 as ::core::ffi::c_int
        && GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
            & C2Rust_Unnamed_0::CHECK_ABORT.0 as ::core::ffi::c_int != 0
    {
        return BOOLEAN::FALSE;
    }
    if (((*pMS).row as ::core::ffi::c_int) < 2 as ::core::ffi::c_int
        || (*pMS).currentShip.0 == MeleeShip::MELEE_NONE.0)
        && (PulsedInputState
            .menu[C2Rust_Unnamed::KEY_MENU_CANCEL.0 as ::core::ffi::c_int as usize] != 0
            || PulsedInputState
                .menu[C2Rust_Unnamed::KEY_MENU_RIGHT.0 as ::core::ffi::c_int as usize]
                != 0
                && ((*pMS).col as ::core::ffi::c_int
                    == if if optWindowType == 1 as DWORD {
                        BOOLEAN::TRUE.0 as ::core::ffi::c_int
                    } else {
                        BOOLEAN::FALSE.0 as ::core::ffi::c_int
                    } != 0
                    {
                        6 as ::core::ffi::c_int
                    } else {
                        7 as ::core::ffi::c_int
                    } - 1 as ::core::ffi::c_int
                    || (*pMS).row as ::core::ffi::c_int == 2 as ::core::ffi::c_int))
    {
        uqm_lua_checked!(Deselect(C2Rust_Unnamed_10::EDIT_MELEE.0 as ::core::ffi::c_int as BYTE), BOOLEAN(0));
        (*pMS).currentShip = MeleeShip::MELEE_NONE;
        (*pMS).MeleeOption = C2Rust_Unnamed_10::START_MELEE.0 as ::core::ffi::c_int
            as MELEE_OPTIONS;
        (*pMS).InputFunc = Some(
            DoMelee as unsafe extern "C" fn(*mut MELEE_STATE) -> BOOLEAN,
        ) as Option<unsafe extern "C" fn(*mut melee_state) -> BOOLEAN>;
        (*pMS).LastInputTime = GetTimeCounter();
    } else if ((*pMS).row as ::core::ffi::c_int) < 2 as ::core::ffi::c_int
        && PulsedInputState
            .menu[C2Rust_Unnamed::KEY_MENU_SELECT.0 as ::core::ffi::c_int as usize] != 0
    {
        uqm_lua_checked!(Select(C2Rust_Unnamed_10::EDIT_MELEE.0 as ::core::ffi::c_int as BYTE), BOOLEAN(0));
        uqm_lua_checked!(BuildPickShipPopup(pMS), BOOLEAN(0));
    } else if ((*pMS).row as ::core::ffi::c_int) < 2 as ::core::ffi::c_int
        && PulsedInputState
            .menu[C2Rust_Unnamed::KEY_MENU_SPECIAL.0 as ::core::ffi::c_int as usize] != 0
    {
        uqm_lua_checked!(Deselect(C2Rust_Unnamed_10::EDIT_MELEE.0 as ::core::ffi::c_int as BYTE), BOOLEAN(0));
        (*pMS).currentShip.0 != MeleeShip::MELEE_NONE.0;
    } else if ((*pMS).row as ::core::ffi::c_int) < 2 as ::core::ffi::c_int
        && PulsedInputState
            .menu[C2Rust_Unnamed::KEY_MENU_DELETE.0 as ::core::ffi::c_int as usize] != 0
    {
        uqm_lua_checked!(Deselect(C2Rust_Unnamed_10::EDIT_MELEE.0 as ::core::ffi::c_int as BYTE), BOOLEAN(0));
        uqm_lua_checked!(DeleteCurrentShip(pMS), BOOLEAN(0));
        AdvanceCursor(pMS);
        uqm_lua_checked!(UpdateCurrentShip(pMS), BOOLEAN(0));
    } else {
        let mut side: COUNT = (*pMS).side;
        let mut row: COUNT = (*pMS).row;
        let mut col: COUNT = (*pMS).col;
        if row as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            if PulsedInputState
                .menu[C2Rust_Unnamed::KEY_MENU_SELECT.0 as ::core::ffi::c_int as usize]
                != 0
            {
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
                let mut buf: [::core::ffi::c_char; 31] = [0; 31];
                (*pMS).CurIndex = 0 as COUNT;
                uqm_lua_checked!(DrawTeamString(
                    pMS,
                    (*pMS).side,
                    1 as COUNT,
                    ::core::ptr::null::<::core::ffi::c_char>(),
                ), BOOLEAN(0));
                strncpy(
                    &raw mut buf as *mut ::core::ffi::c_char,
                    MeleeSetup_getTeamName((*pMS).meleeSetup, (*pMS).side as size_t),
                    30 as size_t,
                );
                buf[30usize] = '\0' as ::core::ffi::c_char;
                tes.Initialized = BOOLEAN::FALSE;
                tes.BaseStr = &raw mut buf as *mut ::core::ffi::c_char as *mut UNICODE;
                tes.CursorPos = 0 as ::core::ffi::c_int;
                tes.MaxSize = 30 as ::core::ffi::c_int + 1 as ::core::ffi::c_int;
                tes.CbParam = pMS as *mut ::core::ffi::c_void;
                tes.ChangeCallback = Some(
                    OnTeamNameChange
                        as unsafe extern "C" fn(*mut TEXTENTRY_STATE) -> BOOLEAN,
                ) as Option<unsafe extern "C" fn(*mut textentry_state) -> BOOLEAN>;
                tes.FrameCallback = Some(
                    TeamNameFrameCallback
                        as unsafe extern "C" fn(*mut TEXTENTRY_STATE) -> BOOLEAN,
                ) as Option<unsafe extern "C" fn(*mut textentry_state) -> BOOLEAN>;
                uqm_lua_checked!(DoTextEntry(&raw mut tes), BOOLEAN(0));
                (*pMS).CurIndex = -1 as ::core::ffi::c_int as COUNT;
                if tes.Success.0 == 0
                    || !uqm_lua_checked!(Melee_LocalChange_teamName(
                        pMS,
                        (*pMS).side,
                        &raw mut buf as *mut ::core::ffi::c_char,
                    ), BOOLEAN(0))
                {
                    uqm_lua_checked!(Melee_UpdateView_teamName(pMS, (*pMS).side), BOOLEAN(0));
                }
                return BOOLEAN::TRUE;
            }
        }
        if PulsedInputState
            .menu[C2Rust_Unnamed::KEY_MENU_LEFT.0 as ::core::ffi::c_int as usize] != 0
        {
            if col as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                col = col.wrapping_sub(1);
            }
        } else if PulsedInputState
            .menu[C2Rust_Unnamed::KEY_MENU_RIGHT.0 as ::core::ffi::c_int as usize] != 0
        {
            if (col as ::core::ffi::c_int)
                < if if optWindowType == 1 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    6 as ::core::ffi::c_int
                } else {
                    7 as ::core::ffi::c_int
                } - 1 as ::core::ffi::c_int
            {
                col = col.wrapping_add(1);
            }
        }
        if PulsedInputState
            .menu[C2Rust_Unnamed::KEY_MENU_UP.0 as ::core::ffi::c_int as usize] != 0
        {
            let c2rust_fresh1 = row;
            row = row.wrapping_sub(1);
            if c2rust_fresh1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                if side as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    row = 0 as COUNT;
                } else {
                    row = 2 as COUNT;
                    side = (side == 0) as ::core::ffi::c_int as COUNT;
                }
            }
        } else if PulsedInputState
            .menu[C2Rust_Unnamed::KEY_MENU_DOWN.0 as ::core::ffi::c_int as usize] != 0
        {
            let c2rust_fresh2 = row;
            row = row.wrapping_add(1);
            if c2rust_fresh2 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
                if side as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
                    row = 2 as COUNT;
                } else {
                    row = 0 as COUNT;
                    side = (side == 0) as ::core::ffi::c_int as COUNT;
                }
            }
        }
        if col as ::core::ffi::c_int != (*pMS).col as ::core::ffi::c_int
            || row as ::core::ffi::c_int != (*pMS).row as ::core::ffi::c_int
            || side as ::core::ffi::c_int != (*pMS).side as ::core::ffi::c_int
        {
            uqm_lua_checked!(Deselect(C2Rust_Unnamed_10::EDIT_MELEE.0 as ::core::ffi::c_int as BYTE), BOOLEAN(0));
            (*pMS).side = side;
            (*pMS).row = row;
            (*pMS).col = col;
            uqm_lua_checked!(UpdateCurrentShip(pMS), BOOLEAN(0));
        }
    }
    uqm_lua_checked!(flushPacketQueues(), BOOLEAN(0));
    uqm_lua_checked!(Melee_flashSelection(pMS), BOOLEAN(0));
    uqm_lua_checked!(SleepThreadUntil(
        TimeIn
            .wrapping_add(
                (840 as ::core::ffi::c_int / 30 as ::core::ffi::c_int) as TimeCount,
            ),
    ), BOOLEAN(0));
    return BOOLEAN::TRUE;
}
unsafe extern "C" fn numPlayersReady() -> ssize_t {
    let mut player: size_t = 0;
    let mut numDone: size_t = 0;
    numDone = 0 as size_t;
    player = 0 as size_t;
    while player < 2 as size_t {
        if *(&raw mut PlayerControl as *mut BYTE).offset(player as isize)
            as ::core::ffi::c_int
            & ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as BYTE
                as ::core::ffi::c_int == 0
        {
            numDone = numDone.wrapping_add(1);
        } else {
            let mut conn: *mut NetConnection = ::core::ptr::null_mut::<NetConnection>();
            conn = *(&raw mut netConnections as *mut *mut NetConnection)
                .offset(player as isize) as *mut NetConnection;
            if conn.is_null() || !NetConnection_isConnected(conn) {
                return -1 as ssize_t;
            }
            if NetConnection_getState(conn).0 > NetState::NetState_inSetup.0 {
                numDone = numDone.wrapping_add(1);
            }
        }
        player = player.wrapping_add(1);
    }
    return numDone as ssize_t;
}
unsafe extern "C" fn DoConfirmSettings(mut pMS: *mut MELEE_STATE) -> BOOLEAN {
    let mut numDone: ssize_t = 0;
    ::core::ptr::write_volatile(&raw mut GamePaused, BOOLEAN::FALSE);
    if PulsedInputState
        .menu[C2Rust_Unnamed::KEY_MENU_CANCEL.0 as ::core::ffi::c_int as usize] != 0
    {
        (*pMS).InputFunc = Some(
            DoMelee as unsafe extern "C" fn(*mut MELEE_STATE) -> BOOLEAN,
        ) as Option<unsafe extern "C" fn(*mut melee_state) -> BOOLEAN>;
        cancelConfirmations();
        uqm_lua_checked!(DrawMeleeStatusMessage(
            GetStringAddress(
                SetAbsStringTableIndex(
                    GameStrings,
                    (C2Rust_Unnamed_9::NETMELEE_STRING_BASE.0 as ::core::ffi::c_int
                        + 4 as ::core::ffi::c_int) as COUNT,
                ),
            ),
        ), BOOLEAN(0));
        return BOOLEAN::TRUE;
    }
    if PulsedInputState
        .menu[C2Rust_Unnamed::KEY_MENU_LEFT.0 as ::core::ffi::c_int as usize] != 0
        || PulsedInputState
            .menu[C2Rust_Unnamed::KEY_MENU_UP.0 as ::core::ffi::c_int as usize] != 0
        || PulsedInputState
            .menu[C2Rust_Unnamed::KEY_MENU_DOWN.0 as ::core::ffi::c_int as usize] != 0
    {
        (*pMS).InputFunc = Some(
            DoMelee as unsafe extern "C" fn(*mut MELEE_STATE) -> BOOLEAN,
        ) as Option<unsafe extern "C" fn(*mut melee_state) -> BOOLEAN>;
        cancelConfirmations();
        uqm_lua_checked!(DrawMeleeStatusMessage(b"\0".as_ptr() as *const ::core::ffi::c_char), BOOLEAN(0));
        return uqm_lua_checked!(DoMelee(pMS), BOOLEAN(0));
    }
    uqm_lua_checked!(closeDisconnectedConnections(), BOOLEAN(0));
    uqm_lua_checked!(netInput(), BOOLEAN(0));
    if webNetplayRole >= 0 as ::core::ffi::c_int
        && GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
            & C2Rust_Unnamed_0::CHECK_ABORT.0 as ::core::ffi::c_int != 0
    {
        return BOOLEAN::FALSE;
    }
    uqm_lua_checked!(SleepThread((840 as ::core::ffi::c_int / 120 as ::core::ffi::c_int) as TimePeriod), BOOLEAN(0));
    numDone = numPlayersReady();
    if numDone == -1 as ssize_t {
        cancelConfirmations();
        uqm_lua_checked!(flushPacketQueues(), BOOLEAN(0));
        (*pMS).InputFunc = Some(
            DoMelee as unsafe extern "C" fn(*mut MELEE_STATE) -> BOOLEAN,
        ) as Option<unsafe extern "C" fn(*mut melee_state) -> BOOLEAN>;
        return BOOLEAN::TRUE;
    } else if numDone != 2 as ssize_t {
        return BOOLEAN::TRUE
    }
    uqm_lua_checked!(Netplay_NotifyAll_inputDelay(netplayOptions.inputDelay), BOOLEAN(0));
    let mut player: COUNT = 0;
    player = 0 as COUNT;
    while (player as ::core::ffi::c_int) < 2 as ::core::ffi::c_int {
        let mut conn: *mut NetConnection = ::core::ptr::null_mut::<NetConnection>();
        if *(&raw mut PlayerControl as *mut BYTE).offset(player as isize)
            as ::core::ffi::c_int
            & ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as BYTE
                as ::core::ffi::c_int != 0
        {
            conn = *(&raw mut netConnections as *mut *mut NetConnection)
                .offset(player as isize) as *mut NetConnection;
            (!conn.is_null()
                || {
                    __assert_fail(
                        b"conn != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                        b"src/uqm/supermelee/melee.c\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        2114 as ::core::ffi::c_int,
                        b"DoConfirmSettings\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    0 as ::core::ffi::c_int != 0
                }) as ::core::ffi::c_int;
            if NetConnection_isConnected(conn) {
                if NetConnection_getDiscriminant(conn) {
                    Netplay_Notify_seedRandom(conn, SeedRandomNumbers());
                }
            }
        }
        player = player.wrapping_add(1);
    }
    uqm_lua_checked!(flushPacketQueues(), BOOLEAN(0));
    let mut allOk: bool = uqm_lua_checked!(negotiateReadyConnections(
        true,
        NetState::NetState_interBattle,
    ), BOOLEAN(0));
    if !allOk {
        return BOOLEAN::FALSE;
    }
    let mut ok: bool = setupInputDelay(netplayOptions.inputDelay);
    if !ok {
        return BOOLEAN::FALSE;
    }
    (*pMS).InputFunc = Some(DoMelee as unsafe extern "C" fn(*mut MELEE_STATE) -> BOOLEAN)
        as Option<unsafe extern "C" fn(*mut melee_state) -> BOOLEAN>;
    uqm_lua_checked!(StartMelee(pMS), BOOLEAN(0));
    if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
        & C2Rust_Unnamed_0::CHECK_ABORT.0 as ::core::ffi::c_int != 0
    {
        return BOOLEAN::FALSE;
    }
    return BOOLEAN::TRUE;
}
unsafe extern "C" fn LoadMeleeInfo(mut pMS: *mut MELEE_STATE) {
    uqm_lua_checked!(BuildPickMeleeFrame(), ());
    MeleeFrame = CaptureDrawable(
        uqm_lua_checked!(LoadGraphicInstance(b"graphics.meleemenu\0".as_ptr() as RESOURCE), ()) as DRAWABLE,
    );
    AtlasArray[0usize] = CaptureDrawable(
        uqm_lua_checked!(LoadGraphicInstance(b"graphics.keyatlas\0".as_ptr() as RESOURCE), ()) as DRAWABLE,
    );
    AtlasArray[1usize] = CaptureDrawable(
        uqm_lua_checked!(LoadGraphicInstance(b"graphics.axisatlas\0".as_ptr() as RESOURCE), ()) as DRAWABLE,
    );
    AtlasArray[2usize] = CaptureDrawable(
        uqm_lua_checked!(LoadGraphicInstance(b"graphics.buttonatlas\0".as_ptr() as RESOURCE), ()) as DRAWABLE,
    );
    SpecialAtlas = CaptureDrawable(
        uqm_lua_checked!(LoadGraphicInstance(b"graphics.specialatlas\0".as_ptr() as RESOURCE), ()) as DRAWABLE,
    );
    uqm_lua_checked!(BuildBuildPickFrame(), ());
    MicroThinFont = uqm_lua_checked!(LoadGraphicInstance(b"font.micro.thin\0".as_ptr() as RESOURCE), ())
        as FONT;
    ButtonFont = uqm_lua_checked!(LoadGraphicInstance(b"font.button\0".as_ptr() as RESOURCE), ()) as FONT;
    FontGradFrame = CaptureDrawable(
        uqm_lua_checked!(LoadGraphicInstance(b"graphics.fontgradient\0".as_ptr() as RESOURCE), ()) as DRAWABLE,
    );
    uqm_lua_checked!(InitSpace(), ());
    uqm_lua_checked!(LoadTeamList(pMS), ());
}
unsafe extern "C" fn FreeMeleeInfo(mut pMS: *mut MELEE_STATE) {
    let mut i: ::core::ffi::c_int = 0;
    DestroyStringTable(ReleaseStringTable((*pMS).load.dirEntries));
    (*pMS).load.dirEntries = ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>();
    if !(*pMS).hMusic.is_null() {
        uqm_lua_checked!(DestroyMusic((*pMS).hMusic), ());
        (*pMS).hMusic = ::core::ptr::null_mut::<*mut TFB_SoundSample>();
    }
    UninitSpace();
    DestroyPickMeleeFrame();
    DestroyDrawable(ReleaseDrawable(MeleeFrame));
    MeleeFrame = ::core::ptr::null_mut::<FRAME_DESC>();
    i = 0 as ::core::ffi::c_int;
    while i < 3 as ::core::ffi::c_int {
        DestroyDrawable(ReleaseDrawable(AtlasArray[i as usize]));
        AtlasArray[i as usize] = ::core::ptr::null_mut::<FRAME_DESC>();
        i += 1;
    }
    DestroyBuildPickFrame();
    DestroyFont(MicroThinFont);
    DestroyFont(ButtonFont);
    DestroyDrawable(ReleaseDrawable(FontGradFrame));
    FontGradFrame = ::core::ptr::null_mut::<FRAME_DESC>();
    uqm_lua_checked!(closeAllConnections(), ());
    setupInputDelay(0 as size_t);
}
unsafe extern "C" fn BuildAndDrawShipList(mut pMS: *mut MELEE_STATE) {
    uqm_lua_checked!(FillPickMeleeFrame((*pMS).meleeSetup), ());
}
unsafe extern "C" fn StartMelee(mut pMS: *mut MELEE_STATE) {
    uqm_lua_checked!(FadeMusic(0 as BYTE, (840 as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as SIZE), ());
    uqm_lua_checked!(SleepThreadUntil(
        FadeScreen(
                ScreenFadeType::FadeAllToBlack,
                (840 as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as SIZE,
            )
            .wrapping_add(
                (840 as ::core::ffi::c_int / 60 as ::core::ffi::c_int) as TimeCount,
            ),
    ), ());
    FlushColorXForms();
    uqm_lua_checked!(SetMusicPosition(), ());
    uqm_lua_checked!(StopMusic(), ());
    uqm_lua_checked!(FadeMusic(160 as BYTE, 0 as SIZE), ());
    if !(*pMS).hMusic.is_null() {
        uqm_lua_checked!(DestroyMusic((*pMS).hMusic), ());
        (*pMS).hMusic = ::core::ptr::null_mut::<*mut TFB_SoundSample>();
    }
    if SetPlayerInputAll() {
        uqm_lua_checked!(BuildAndDrawShipList(pMS), ());
        uqm_lua_checked!(WaitForSoundEnd(!(0 as ::core::ffi::c_int) as COUNT), ());
        uqm_lua_checked!(load_gravity_well(
            (TFB_Random() as COUNT as ::core::ffi::c_int
                % C2Rust_Unnamed_8::NUMBER_OF_PLANET_TYPES.0 as ::core::ffi::c_int)
                as BYTE,
        ), ());
        uqm_lua_checked!(Battle(None), ());
        free_gravity_well();
        uqm_lua_checked!(ClearPlayerInputAll(), ());
        if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
            & C2Rust_Unnamed_0::CHECK_ABORT.0 as ::core::ffi::c_int != 0
        {
            return;
        }
        uqm_lua_checked!(SleepThreadUntil(
            FadeScreen(
                    ScreenFadeType::FadeAllToBlack,
                    (840 as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as SIZE,
                )
                .wrapping_add(
                    (840 as ::core::ffi::c_int / 60 as ::core::ffi::c_int) as TimeCount,
                ),
        ), ());
        FlushColorXForms();
    }
    GlobData.Game_state.CurrentActivity = C2Rust_Unnamed_0::SUPER_MELEE.0
        as ::core::ffi::c_int as ACTIVITY;
    (*pMS).Initialized = BOOLEAN::FALSE;
}
unsafe extern "C" fn StartMeleeButtonPressed(mut pMS: *mut MELEE_STATE) {
    if MeleeSetup_getFleetValue((*pMS).meleeSetup, 0 as size_t) as ::core::ffi::c_int
        == 0 as ::core::ffi::c_int
        || MeleeSetup_getFleetValue((*pMS).meleeSetup, 1 as size_t) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(PlayMenuSound(MENU_SOUND_EFFECT::MENU_SOUND_FAILURE), ());
        return;
    }
    if *(&raw mut PlayerControl as *mut BYTE).offset(0isize) as ::core::ffi::c_int
        & ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as BYTE
            as ::core::ffi::c_int != 0
        && *(&raw mut PlayerControl as *mut BYTE).offset(1isize) as ::core::ffi::c_int
            & ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as BYTE
                as ::core::ffi::c_int != 0
    {
        uqm_lua_checked!(DrawMeleeStatusMessage(
            GetStringAddress(
                SetAbsStringTableIndex(
                    GameStrings,
                    (C2Rust_Unnamed_9::NETMELEE_STRING_BASE.0 as ::core::ffi::c_int
                        + 32 as ::core::ffi::c_int) as COUNT,
                ),
            ),
        ), ());
        return;
    }
    if *(&raw mut PlayerControl as *mut BYTE).offset(0isize) as ::core::ffi::c_int
        & ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as BYTE
            as ::core::ffi::c_int != 0
        && *(&raw mut PlayerControl as *mut BYTE).offset(1isize) as ::core::ffi::c_int
            & (((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as BYTE
                as ::core::ffi::c_int
                | ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as BYTE
                    as ::core::ffi::c_int) != 0
        || *(&raw mut PlayerControl as *mut BYTE).offset(0isize) as ::core::ffi::c_int
            & (((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as BYTE
                as ::core::ffi::c_int
                | ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as BYTE
                    as ::core::ffi::c_int) != 0
            && *(&raw mut PlayerControl as *mut BYTE).offset(1isize)
                as ::core::ffi::c_int
                & ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as BYTE
                    as ::core::ffi::c_int != 0
    {
        uqm_lua_checked!(DrawMeleeStatusMessage(
            GetStringAddress(
                SetAbsStringTableIndex(
                    GameStrings,
                    (C2Rust_Unnamed_9::NETMELEE_STRING_BASE.0 as ::core::ffi::c_int
                        + 33 as ::core::ffi::c_int) as COUNT,
                ),
            ),
        ), ());
        return;
    }
    let mut player: COUNT = 0;
    let mut netReady: bool = true;
    player = 0 as COUNT;
    while (player as ::core::ffi::c_int) < 2 as ::core::ffi::c_int {
        let mut conn: *mut NetConnection = ::core::ptr::null_mut::<NetConnection>();
        if *(&raw mut PlayerControl as *mut BYTE).offset(player as isize)
            as ::core::ffi::c_int
            & ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as BYTE
                as ::core::ffi::c_int != 0
        {
            conn = *(&raw mut netConnections as *mut *mut NetConnection)
                .offset(player as isize) as *mut NetConnection;
            if conn.is_null() || !NetConnection_isConnected(conn) {
                netReady = false;
                if player as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    uqm_lua_checked!(DrawMeleeStatusMessage(
                        GetStringAddress(
                            SetAbsStringTableIndex(
                                GameStrings,
                                (C2Rust_Unnamed_9::NETMELEE_STRING_BASE.0
                                    as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as COUNT,
                            ),
                        ),
                    ), ());
                } else {
                    uqm_lua_checked!(DrawMeleeStatusMessage(
                        GetStringAddress(
                            SetAbsStringTableIndex(
                                GameStrings,
                                (C2Rust_Unnamed_9::NETMELEE_STRING_BASE.0
                                    as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as COUNT,
                            ),
                        ),
                    ), ());
                }
            } else if NetConnection_getState(conn).0 != NetState::NetState_inSetup.0 {
                netReady = false;
                if player as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    uqm_lua_checked!(DrawMeleeStatusMessage(
                        GetStringAddress(
                            SetAbsStringTableIndex(
                                GameStrings,
                                (C2Rust_Unnamed_9::NETMELEE_STRING_BASE.0
                                    as ::core::ffi::c_int + 14 as ::core::ffi::c_int) as COUNT,
                            ),
                        ),
                    ), ());
                } else {
                    uqm_lua_checked!(DrawMeleeStatusMessage(
                        GetStringAddress(
                            SetAbsStringTableIndex(
                                GameStrings,
                                (C2Rust_Unnamed_9::NETMELEE_STRING_BASE.0
                                    as ::core::ffi::c_int + 15 as ::core::ffi::c_int) as COUNT,
                            ),
                        ),
                    ), ());
                }
            }
        }
        player = player.wrapping_add(1);
    }
    if !netReady {
        uqm_lua_checked!(PlayMenuSound(MENU_SOUND_EFFECT::MENU_SOUND_FAILURE), ());
        return;
    }
    if numPlayersReady() != 2 as ssize_t {
        uqm_lua_checked!(DrawMeleeStatusMessage(
            GetStringAddress(
                SetAbsStringTableIndex(
                    GameStrings,
                    (C2Rust_Unnamed_9::NETMELEE_STRING_BASE.0 as ::core::ffi::c_int
                        + 7 as ::core::ffi::c_int) as COUNT,
                ),
            ),
        ), ());
    }
    confirmConnections();
    (*pMS).InputFunc = Some(
        DoConfirmSettings as unsafe extern "C" fn(*mut MELEE_STATE) -> BOOLEAN,
    ) as Option<unsafe extern "C" fn(*mut melee_state) -> BOOLEAN>;
}
unsafe extern "C" fn DoConnectingDialog(mut pMS: *mut MELEE_STATE) -> BOOLEAN {
    let mut TimeIn: DWORD = GetTimeCounter();
    let mut which_side: COUNT = (if (*pMS).MeleeOption as ::core::ffi::c_int
        == C2Rust_Unnamed_10::NET_TOP.0 as ::core::ffi::c_int
    {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    }) as COUNT;
    let mut conn: *mut NetConnection = ::core::ptr::null_mut::<NetConnection>();
    ::core::ptr::write_volatile(&raw mut GamePaused, BOOLEAN::FALSE);
    if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
        & C2Rust_Unnamed_0::CHECK_ABORT.0 as ::core::ffi::c_int != 0
    {
        return BOOLEAN::FALSE;
    }
    SetMenuSounds(
        0 as ::core::ffi::c_int as MENU_SOUND_FLAGS,
        0 as ::core::ffi::c_int as MENU_SOUND_FLAGS,
    );
    if (*pMS).Initialized.0 == 0 {
        let mut r: RECT = RECT {
            corner: POINT { x: 0, y: 0 },
            extent: EXTENT { width: 0, height: 0 },
        };
        let mut oldfont: FONT = ::core::ptr::null_mut::<FONT_DESC>();
        let mut oldcolor: Color = Color { r: 0, g: 0, b: 0, a: 0 };
        let mut t: TEXT = TEXT {
            baseline: POINT { x: 0, y: 0 },
            pStr: ::core::ptr::null::<UNICODE>(),
            align: TEXT_ALIGN::ALIGN_LEFT,
            CharCount: 0,
        };
        if !(*(&raw mut netConnections as *mut *mut NetConnection)
            .offset(which_side as isize))
            .is_null()
        {
            uqm_lua_checked!(closePlayerNetworkConnection(which_side), BOOLEAN(0));
        }
        (*pMS).Initialized = BOOLEAN::TRUE;
        conn = openPlayerNetworkConnection(which_side, pMS as *mut ::core::ffi::c_void);
        (*pMS).InputFunc = Some(
            DoConnectingDialog as unsafe extern "C" fn(*mut MELEE_STATE) -> BOOLEAN,
        ) as Option<unsafe extern "C" fn(*mut melee_state) -> BOOLEAN>;
        oldfont = SetContextFont(StarConFont);
        oldcolor = SetContextForeGroundColor(
            colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE),
        );
        BatchGraphics();
        r.extent.width = ((200 as ::core::ffi::c_int) << resolutionFactor) as COORD;
        r.extent.height = ((30 as ::core::ffi::c_int) << resolutionFactor) as COORD;
        r.corner.x = (CanvasWidth - r.extent.width as ::core::ffi::c_int
            >> 1 as ::core::ffi::c_int) as COORD;
        r.corner.y = (CanvasHeight - r.extent.height as ::core::ffi::c_int
            >> 1 as ::core::ffi::c_int) as COORD;
        uqm_lua_checked!(DrawShadowedBox(
            &raw mut r,
            colorFromRgb15(0x14 as BYTE, 0x14 as BYTE, 0x14 as BYTE),
            colorFromRgb15(0x8 as BYTE, 0x8 as BYTE, 0x8 as BYTE),
            colorFromRgb15(0x10 as BYTE, 0x10 as BYTE, 0x10 as BYTE),
        ), BOOLEAN(0));
        if (*NetConnection_getPeerOptions(conn)).isServer {
            t.pStr = GetStringAddress(
                SetAbsStringTableIndex(
                    GameStrings,
                    (C2Rust_Unnamed_9::NETMELEE_STRING_BASE.0 as ::core::ffi::c_int
                        + 1 as ::core::ffi::c_int) as COUNT,
                ),
            );
        } else {
            t.pStr = GetStringAddress(
                SetAbsStringTableIndex(
                    GameStrings,
                    (C2Rust_Unnamed_9::NETMELEE_STRING_BASE.0 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int) as COUNT,
                ),
            );
        }
        t.baseline.y = (r.corner.y as ::core::ffi::c_int
            + ((10 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        t.baseline.x = (CanvasWidth >> 1 as ::core::ffi::c_int) as COORD;
        t.align = TEXT_ALIGN::ALIGN_CENTER;
        t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
        uqm_lua_checked!(font_DrawText(&raw mut t), BOOLEAN(0));
        t.pStr = GetStringAddress(
            SetAbsStringTableIndex(
                GameStrings,
                (C2Rust_Unnamed_9::NETMELEE_STRING_BASE.0 as ::core::ffi::c_int
                    + 18 as ::core::ffi::c_int) as COUNT,
            ),
        );
        t.baseline.y = (t.baseline.y as ::core::ffi::c_int
            + ((16 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
        uqm_lua_checked!(font_DrawText(&raw mut t), BOOLEAN(0));
        SetContextFont(oldfont);
        SetContextForeGroundColor(oldcolor);
        UnbatchGraphics();
    }
    uqm_lua_checked!(netInput(), BOOLEAN(0));
    if PulsedInputState
        .menu[C2Rust_Unnamed::KEY_MENU_CANCEL.0 as ::core::ffi::c_int as usize] != 0
    {
        if !(*(&raw mut netConnections as *mut *mut NetConnection)
            .offset(which_side as isize))
            .is_null()
        {
            uqm_lua_checked!(closePlayerNetworkConnection(which_side), BOOLEAN(0));
            uqm_lua_checked!(UpdateMeleeStatusMessage(which_side as ssize_t), BOOLEAN(0));
        }
        uqm_lua_checked!(RedrawMeleeFrame(), BOOLEAN(0));
        (*pMS).InputFunc = Some(
            DoMelee as unsafe extern "C" fn(*mut MELEE_STATE) -> BOOLEAN,
        ) as Option<unsafe extern "C" fn(*mut melee_state) -> BOOLEAN>;
        (*pMS).LastInputTime = GetTimeCounter();
        uqm_lua_checked!(flushPacketQueues(), BOOLEAN(0));
        return BOOLEAN::TRUE;
    }
    conn = *(&raw mut netConnections as *mut *mut NetConnection)
        .offset(which_side as isize) as *mut NetConnection;
    if !conn.is_null() {
        let mut status: NetState = NetConnection_getState(conn);
        if status.0 == NetState::NetState_init.0
            || status.0 == NetState::NetState_inSetup.0
        {
            *(&raw mut PlayerControl as *mut BYTE).offset(which_side as isize) = (((1
                as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as BYTE
                as ::core::ffi::c_int
                | ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as BYTE
                    as ::core::ffi::c_int) as BYTE;
            uqm_lua_checked!(DrawControls(which_side, BOOLEAN::TRUE), BOOLEAN(0));
            uqm_lua_checked!(RedrawMeleeFrame(), BOOLEAN(0));
            uqm_lua_checked!(UpdateMeleeStatusMessage(which_side as ssize_t), BOOLEAN(0));
            (*pMS).InputFunc = Some(
                DoMelee as unsafe extern "C" fn(*mut MELEE_STATE) -> BOOLEAN,
            ) as Option<unsafe extern "C" fn(*mut melee_state) -> BOOLEAN>;
            (*pMS).LastInputTime = GetTimeCounter();
            uqm_lua_checked!(Deselect((*pMS).MeleeOption), BOOLEAN(0));
            (*pMS).MeleeOption = C2Rust_Unnamed_10::START_MELEE.0 as ::core::ffi::c_int
                as MELEE_OPTIONS;
        }
    }
    uqm_lua_checked!(flushPacketQueues(), BOOLEAN(0));
    uqm_lua_checked!(SleepThreadUntil(
        TimeIn
            .wrapping_add(
                (840 as ::core::ffi::c_int / 30 as ::core::ffi::c_int) as TimeCount,
            ),
    ), BOOLEAN(0));
    return BOOLEAN::TRUE;
}
unsafe extern "C" fn check_for_disconnects(mut pMS: *mut MELEE_STATE) {
    let mut player: COUNT = 0;
    player = 0 as COUNT;
    while (player as ::core::ffi::c_int) < 2 as ::core::ffi::c_int {
        let mut conn: *mut NetConnection = ::core::ptr::null_mut::<NetConnection>();
        if *(&raw mut PlayerControl as *mut BYTE).offset(player as isize)
            as ::core::ffi::c_int
            & ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as BYTE
                as ::core::ffi::c_int != 0
        {
            conn = *(&raw mut netConnections as *mut *mut NetConnection)
                .offset(player as isize) as *mut NetConnection;
            if conn.is_null() || !NetConnection_isConnected(conn) {
                if webNetplayRole >= 0 as ::core::ffi::c_int {
                    GlobData.Game_state.CurrentActivity = (GlobData
                        .Game_state
                        .CurrentActivity as ::core::ffi::c_int
                        | C2Rust_Unnamed_0::CHECK_ABORT.0 as ::core::ffi::c_int)
                        as ACTIVITY;
                    return;
                }
                *(&raw mut PlayerControl as *mut BYTE).offset(player as isize) = (((1
                    as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as BYTE
                    as ::core::ffi::c_int
                    | ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as BYTE
                        as ::core::ffi::c_int) as BYTE;
                uqm_lua_checked!(DrawControls(player, BOOLEAN::FALSE), ());
                log_add(
                    log_Level::log_User,
                    b"Player %d has disconnected; shifting controls\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    player as ::core::ffi::c_int,
                );
            }
        }
        player = player.wrapping_add(1);
    }
}
unsafe extern "C" fn nextControlType(mut which_side: COUNT) {
    match *(&raw mut PlayerControl as *mut BYTE).offset(which_side as isize)
        as ::core::ffi::c_int
    {
        17 => {
            *(&raw mut PlayerControl as *mut BYTE).offset(which_side as isize) = (((1
                as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as BYTE
                as ::core::ffi::c_int
                | ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as BYTE
                    as ::core::ffi::c_int
                | ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as BYTE
                    as ::core::ffi::c_int) as BYTE;
        }
        22 => {
            *(&raw mut PlayerControl as *mut BYTE).offset(which_side as isize) = (((1
                as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as BYTE
                as ::core::ffi::c_int
                | ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as BYTE
                    as ::core::ffi::c_int
                | ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as BYTE
                    as ::core::ffi::c_int) as BYTE;
        }
        38 => {
            *(&raw mut PlayerControl as *mut BYTE).offset(which_side as isize) = (((1
                as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as BYTE
                as ::core::ffi::c_int
                | ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as BYTE
                    as ::core::ffi::c_int
                | ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as BYTE
                    as ::core::ffi::c_int) as BYTE;
        }
        70 => {
            *(&raw mut PlayerControl as *mut BYTE).offset(which_side as isize) = (((1
                as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as BYTE
                as ::core::ffi::c_int
                | ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as BYTE
                    as ::core::ffi::c_int) as BYTE;
        }
        24 => {
            if !(*(&raw mut netConnections as *mut *mut NetConnection)
                .offset(which_side as isize))
                .is_null()
            {
                uqm_lua_checked!(closePlayerNetworkConnection(which_side), ());
            }
            uqm_lua_checked!(UpdateMeleeStatusMessage(-1 as ssize_t), ());
            *(&raw mut PlayerControl as *mut BYTE).offset(which_side as isize) = (((1
                as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as BYTE
                as ::core::ffi::c_int
                | ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as BYTE
                    as ::core::ffi::c_int) as BYTE;
        }
        _ => {
            log_add(
                log_Level::log_Error,
                b"Error: Bad control type (%d) in nextControlType().\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                *(&raw mut PlayerControl as *mut BYTE).offset(which_side as isize)
                    as ::core::ffi::c_int,
            );
            *(&raw mut PlayerControl as *mut BYTE).offset(which_side as isize) = (((1
                as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as BYTE
                as ::core::ffi::c_int
                | ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as BYTE
                    as ::core::ffi::c_int) as BYTE;
        }
    }
    uqm_lua_checked!(DrawControls(which_side, BOOLEAN::TRUE), ());
}
unsafe extern "C" fn MeleeOptionDown(mut current: MELEE_OPTIONS) -> MELEE_OPTIONS {
    if current as ::core::ffi::c_int
        == C2Rust_Unnamed_10::QUIT_BOT.0 as ::core::ffi::c_int
    {
        return C2Rust_Unnamed_10::NET_TOP.0 as ::core::ffi::c_int as MELEE_OPTIONS;
    }
    return (current as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as MELEE_OPTIONS;
}
unsafe extern "C" fn MeleeOptionUp(mut current: MELEE_OPTIONS) -> MELEE_OPTIONS {
    if current as ::core::ffi::c_int
        == C2Rust_Unnamed_10::NET_TOP.0 as ::core::ffi::c_int
    {
        return C2Rust_Unnamed_10::QUIT_BOT.0 as ::core::ffi::c_int as MELEE_OPTIONS;
    }
    return (current as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as MELEE_OPTIONS;
}
unsafe extern "C" fn MeleeOptionSelect(mut pMS: *mut MELEE_STATE) {
    match (*pMS).MeleeOption as ::core::ffi::c_int {
        4 => {
            uqm_lua_checked!(StartMeleeButtonPressed(pMS), ());
        }
        3 | 5 => {
            (*pMS).Initialized = BOOLEAN::FALSE;
            (*pMS).side = (if (*pMS).MeleeOption as ::core::ffi::c_int
                == C2Rust_Unnamed_10::LOAD_TOP.0 as ::core::ffi::c_int
            {
                0 as ::core::ffi::c_int
            } else {
                1 as ::core::ffi::c_int
            }) as COUNT;
            uqm_lua_checked!(DoLoadTeam(pMS), ());
        }
        2 | 6 => {
            (*pMS).side = (if (*pMS).MeleeOption as ::core::ffi::c_int
                == C2Rust_Unnamed_10::SAVE_TOP.0 as ::core::ffi::c_int
            {
                0 as ::core::ffi::c_int
            } else {
                1 as ::core::ffi::c_int
            }) as COUNT;
            if MeleeSetup_getFleetValue((*pMS).meleeSetup, (*pMS).side as size_t)
                as ::core::ffi::c_int > 0 as ::core::ffi::c_int
            {
                uqm_lua_checked!(DoSaveTeam(pMS), ());
            } else {
                uqm_lua_checked!(PlayMenuSound(MENU_SOUND_EFFECT::MENU_SOUND_FAILURE), ());
            }
        }
        8 => {
            GlobData.Game_state.CurrentActivity = (GlobData.Game_state.CurrentActivity
                as ::core::ffi::c_int
                | C2Rust_Unnamed_0::CHECK_ABORT.0 as ::core::ffi::c_int) as ACTIVITY;
        }
        0 => {
            let mut which_side: COUNT = 0;
            let mut confirmed: BOOLEAN = BOOLEAN::FALSE;
            if webNetplayRole >= 0 as ::core::ffi::c_int {
                uqm_lua_checked!(DrawMeleeStatusMessage(
                    b"Player sides are assigned by the online room.\0".as_ptr()
                        as *const ::core::ffi::c_char,
                ), ());
            } else {
                uqm_lua_checked!(DrawMeleeStatusMessage(
                    (if crate::src::uqm::restart::UQM_MeleeOnlyProfile().0 != 0 {
                        b"Choose EXIT, then Network Melee.\0".as_ptr()
                    } else {
                        b"For Online Melee, press O in the main menu.\0".as_ptr()
                    }) as *const ::core::ffi::c_char,
                ), ());
            }
        }
        1 | 7 => {
            let mut which_side_0: COUNT = (if (*pMS).MeleeOption as ::core::ffi::c_int
                == C2Rust_Unnamed_10::CONTROLS_TOP.0 as ::core::ffi::c_int
            {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as COUNT;
            if webNetplayRole >= 0 as ::core::ffi::c_int {
                uqm_lua_checked!(DrawMeleeStatusMessage(
                    b"Player sides are assigned by the online room.\0".as_ptr()
                        as *const ::core::ffi::c_char,
                ), ());
            } else {
                uqm_lua_checked!(nextControlType(which_side_0), ());
            }
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn DoMelee(mut pMS: *mut MELEE_STATE) -> BOOLEAN {
    let mut TimeIn: DWORD = GetTimeCounter();
    let mut force_select: BOOLEAN = BOOLEAN::FALSE;
    ::core::ptr::write_volatile(&raw mut GamePaused, BOOLEAN::FALSE);
    if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
        & C2Rust_Unnamed_0::CHECK_ABORT.0 as ::core::ffi::c_int != 0
    {
        return BOOLEAN::FALSE;
    }
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
    if (*pMS).Initialized.0 == 0 {
        if !(*pMS).hMusic.is_null() {
            uqm_lua_checked!(StopMusic(), BOOLEAN(0));
            uqm_lua_checked!(DestroyMusic((*pMS).hMusic), BOOLEAN(0));
            (*pMS).hMusic = ::core::ptr::null_mut::<*mut TFB_SoundSample>();
        }
        (*pMS).hMusic = ::core::ptr::null_mut::<*mut TFB_SoundSample>();
        (*pMS).Initialized = BOOLEAN::TRUE;
        (*pMS).MeleeOption = C2Rust_Unnamed_10::START_MELEE.0 as ::core::ffi::c_int
            as MELEE_OPTIONS;
        if optMainMenuMusic.0 != 0 {
            (*pMS).hMusic = uqm_lua_checked!(LoadMusicInstance(b"music.meleemenu\0".as_ptr() as RESOURCE), BOOLEAN(0));
            uqm_lua_checked!(PlayMusicResume((*pMS).hMusic, 160 as BYTE), BOOLEAN(0));
        }
        uqm_lua_checked!(InitMelee(pMS), BOOLEAN(0));
        if webNetplayRole >= 0 as ::core::ffi::c_int
            && (*(&raw mut netConnections as *mut *mut NetConnection)
                .offset((1 as ::core::ffi::c_int - webNetplayRole) as isize))
                .is_null()
        {
            openPlayerNetworkConnection(
                (1 as ::core::ffi::c_int - webNetplayRole) as COUNT,
                pMS as *mut ::core::ffi::c_void,
            );
        }
        FadeScreen(
            ScreenFadeType::FadeAllToColor,
            (840 as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as SIZE,
        );
        (*pMS).LastInputTime = GetTimeCounter();
        return BOOLEAN::TRUE;
    }
    uqm_lua_checked!(netInput(), BOOLEAN(0));
    if webNetplayRole >= 0 as ::core::ffi::c_int
        && GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
            & C2Rust_Unnamed_0::CHECK_ABORT.0 as ::core::ffi::c_int != 0
    {
        return BOOLEAN::FALSE;
    }
    if PulsedInputState
        .menu[C2Rust_Unnamed::KEY_MENU_CANCEL.0 as ::core::ffi::c_int as usize] != 0
        || PulsedInputState
            .menu[C2Rust_Unnamed::KEY_MENU_LEFT.0 as ::core::ffi::c_int as usize] != 0
    {
        (*pMS).LastInputTime = GetTimeCounter();
        uqm_lua_checked!(Deselect((*pMS).MeleeOption), BOOLEAN(0));
        (*pMS).MeleeOption = C2Rust_Unnamed_10::EDIT_MELEE.0 as ::core::ffi::c_int
            as MELEE_OPTIONS;
        (*pMS).Initialized = BOOLEAN::FALSE;
        if PulsedInputState
            .menu[C2Rust_Unnamed::KEY_MENU_CANCEL.0 as ::core::ffi::c_int as usize] != 0
        {
            (*pMS).side = 0 as COUNT;
            (*pMS).row = 0 as COUNT;
            (*pMS).col = 0 as COUNT;
        } else {
            (*pMS).side = 0 as COUNT;
            (*pMS).row = (2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as COUNT;
            (*pMS).col = (if if optWindowType == 1 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                6 as ::core::ffi::c_int
            } else {
                7 as ::core::ffi::c_int
            } - 1 as ::core::ffi::c_int) as COUNT;
        }
        uqm_lua_checked!(DoEdit(pMS), BOOLEAN(0));
    } else {
        let mut NewMeleeOption: MELEE_OPTIONS = 0;
        NewMeleeOption = (*pMS).MeleeOption;
        if PulsedInputState
            .menu[C2Rust_Unnamed::KEY_MENU_UP.0 as ::core::ffi::c_int as usize] != 0
        {
            (*pMS).LastInputTime = GetTimeCounter();
            NewMeleeOption = MeleeOptionUp((*pMS).MeleeOption);
        } else if PulsedInputState
            .menu[C2Rust_Unnamed::KEY_MENU_DOWN.0 as ::core::ffi::c_int as usize] != 0
        {
            (*pMS).LastInputTime = GetTimeCounter();
            NewMeleeOption = MeleeOptionDown((*pMS).MeleeOption);
        }
        if *(&raw mut PlayerControl as *mut BYTE).offset(0isize) as ::core::ffi::c_int
            & *(&raw mut PlayerControl as *mut BYTE).offset(1isize) as ::core::ffi::c_int
            & ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as BYTE
                as ::core::ffi::c_int != 0
            && GetTimeCounter().wrapping_sub((*pMS).LastInputTime)
                > (840 as ::core::ffi::c_int * 10 as ::core::ffi::c_int) as TimeCount
        {
            force_select = BOOLEAN::TRUE;
            NewMeleeOption = C2Rust_Unnamed_10::START_MELEE.0 as ::core::ffi::c_int
                as MELEE_OPTIONS;
        }
        if NewMeleeOption as ::core::ffi::c_int
            != (*pMS).MeleeOption as ::core::ffi::c_int
        {
            if (*pMS).MeleeOption as ::core::ffi::c_int
                == C2Rust_Unnamed_10::CONTROLS_TOP.0 as ::core::ffi::c_int
                || (*pMS).MeleeOption as ::core::ffi::c_int
                    == C2Rust_Unnamed_10::CONTROLS_BOT.0 as ::core::ffi::c_int
            {
                uqm_lua_checked!(UpdateMeleeStatusMessage(-1 as ssize_t), BOOLEAN(0));
            }
            uqm_lua_checked!(Deselect((*pMS).MeleeOption), BOOLEAN(0));
            (*pMS).MeleeOption = NewMeleeOption;
            uqm_lua_checked!(Select((*pMS).MeleeOption), BOOLEAN(0));
            if NewMeleeOption as ::core::ffi::c_int
                == C2Rust_Unnamed_10::CONTROLS_TOP.0 as ::core::ffi::c_int
                || NewMeleeOption as ::core::ffi::c_int
                    == C2Rust_Unnamed_10::CONTROLS_BOT.0 as ::core::ffi::c_int
            {
                let mut side: COUNT = (if NewMeleeOption as ::core::ffi::c_int
                    == C2Rust_Unnamed_10::CONTROLS_TOP.0 as ::core::ffi::c_int
                {
                    1 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                }) as COUNT;
                if *(&raw mut PlayerControl as *mut BYTE).offset(side as isize)
                    as ::core::ffi::c_int
                    & ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as BYTE
                        as ::core::ffi::c_int != 0
                {
                    uqm_lua_checked!(UpdateMeleeStatusMessage(side as ssize_t), BOOLEAN(0));
                } else {
                    uqm_lua_checked!(UpdateMeleeStatusMessage(-1 as ssize_t), BOOLEAN(0));
                }
            }
        }
        if PulsedInputState
            .menu[C2Rust_Unnamed::KEY_MENU_SELECT.0 as ::core::ffi::c_int as usize] != 0
            || force_select.0 != 0
        {
            uqm_lua_checked!(MeleeOptionSelect(pMS), BOOLEAN(0));
            if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
                & C2Rust_Unnamed_0::CHECK_ABORT.0 as ::core::ffi::c_int != 0
            {
                return BOOLEAN::FALSE;
            }
        }
    }
    uqm_lua_checked!(flushPacketQueues(), BOOLEAN(0));
    uqm_lua_checked!(check_for_disconnects(pMS), BOOLEAN(0));
    uqm_lua_checked!(Melee_flashSelection(pMS), BOOLEAN(0));
    uqm_lua_checked!(SleepThreadUntil(
        TimeIn
            .wrapping_add(
                (840 as ::core::ffi::c_int / 30 as ::core::ffi::c_int) as TimeCount,
            ),
    ), BOOLEAN(0));
    return BOOLEAN::TRUE;
}
unsafe extern "C" fn LoadMeleeConfig(mut pMS: *mut MELEE_STATE) -> ::core::ffi::c_int {
    let mut stream: *mut uio_Stream = ::core::ptr::null_mut::<uio_Stream>();
    let mut status: ::core::ffi::c_int = 0;
    let mut side: COUNT = 0;
    stream = uqm_lua_checked!(uio_fopen(
        configDir,
        b"melee.cfg\0".as_ptr() as *const ::core::ffi::c_char,
        b"rb\0".as_ptr() as *const ::core::ffi::c_char,
    ), 0);
    '_err: {
        if !stream.is_null() {
            let mut sb: stat = stat {
                st_dev: 0,
                st_mode: 0,
                st_nlink: 0,
                st_uid: 0,
                st_gid: 0,
                st_rdev: 0,
                st_size: 0,
                st_blksize: 0,
                st_blocks: 0,
                st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
                st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
                st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
                st_ino: 0,
            };
            if uqm_lua_checked!(uio_fstat(uio_streamHandle(stream), &raw mut sb), 0)
                != -1 as ::core::ffi::c_int
            {
                if sb.st_size as size_t
                    == (1 as size_t)
                        .wrapping_add(MeleeTeam_serialSize)
                        .wrapping_mul(2 as size_t)
                {
                    side = 0 as COUNT;
                    while (side as ::core::ffi::c_int) < 2 as ::core::ffi::c_int {
                        status = uqm_lua_checked!(uio_fgetc(stream), 0);
                        if status == -1 as ::core::ffi::c_int {
                            break '_err;
                        }
                        *(&raw mut PlayerControl as *mut BYTE).offset(side as isize) = status
                            as BYTE;
                        if uqm_lua_checked!(MeleeSetup_deserializeTeam(
                            (*pMS).meleeSetup,
                            side as size_t,
                            stream,
                        ), 0) == -1 as ::core::ffi::c_int
                        {
                            break '_err;
                        }
                        if *(&raw mut PlayerControl as *mut BYTE).offset(side as isize)
                            as ::core::ffi::c_int
                            & ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                                as BYTE as ::core::ffi::c_int != 0
                        {
                            *(&raw mut PlayerControl as *mut BYTE)
                                .offset(side as isize) = (((1 as ::core::ffi::c_int)
                                << 0 as ::core::ffi::c_int) as BYTE as ::core::ffi::c_int
                                | ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                                    as BYTE as ::core::ffi::c_int) as BYTE;
                        }
                        side = side.wrapping_add(1);
                    }
                    uqm_lua_checked!(uio_fclose(stream), 0);
                    return 0 as ::core::ffi::c_int;
                }
            }
        }
    }
    if !stream.is_null() {
        uqm_lua_checked!(uio_fclose(stream), 0);
    }
    return -1 as ::core::ffi::c_int;
}
unsafe extern "C" fn WriteMeleeConfig(mut pMS: *mut MELEE_STATE) -> ::core::ffi::c_int {
    let mut stream: *mut uio_Stream = ::core::ptr::null_mut::<uio_Stream>();
    let mut side: COUNT = 0;
    stream = uqm_lua_checked!(res_OpenResFile(
        configDir,
        b"melee.cfg\0".as_ptr() as *const ::core::ffi::c_char,
        b"wb\0".as_ptr() as *const ::core::ffi::c_char,
    ), 0);
    '_err: {
        if !stream.is_null() {
            side = 0 as COUNT;
            while (side as ::core::ffi::c_int) < 2 as ::core::ffi::c_int {
                if uqm_lua_checked!(uio_fputc(
                    *(&raw mut PlayerControl as *mut BYTE).offset(side as isize)
                        as ::core::ffi::c_int,
                    stream,
                ), 0) == -1 as ::core::ffi::c_int
                {
                    break '_err;
                }
                if uqm_lua_checked!(MeleeSetup_serializeTeam((*pMS).meleeSetup, side as size_t, stream), 0)
                    == -1 as ::core::ffi::c_int
                {
                    break '_err;
                }
                side = side.wrapping_add(1);
            }
            if uqm_lua_checked!(res_CloseResFile(stream), 0).0 != 0 {
                return 0 as ::core::ffi::c_int;
            }
        }
    }
    if !stream.is_null() {
        uqm_lua_checked!(res_CloseResFile(stream), 0);
        uqm_lua_checked!(DeleteResFile(configDir, b"melee.cfg\0".as_ptr() as *const ::core::ffi::c_char), 0);
    }
    return -1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Melee() {
    let mut savedControl: [BYTE; 2] = [0; 2];
    let mut savedTemplates: [CONTROL_TEMPLATE; 2] = [CONTROL_TEMPLATE::CONTROL_TEMPLATE_PL_1; 2];
    let mut webSession: BOOLEAN = BOOLEAN(
        (webNetplayRole >= 0 as ::core::ffi::c_int) as ::core::ffi::c_int
            as ::core::ffi::c_uint,
    );
    InitGlobData();
    let mut MenuState: MELEE_STATE = MELEE_STATE {
        InputFunc: None,
        Initialized: BOOLEAN::FALSE,
        meleeStarted: BOOLEAN::FALSE,
        MeleeOption: 0,
        side: 0,
        row: 0,
        col: 0,
        meleeSetup: ::core::ptr::null_mut::<MeleeSetup>(),
        load: melee_load_state {
            preBuiltList: ::core::ptr::null_mut::<*mut MeleeTeam>(),
            preBuiltCount: 0,
            dirEntries: ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>(),
            entryIndices: ::core::ptr::null_mut::<COUNT>(),
            numIndices: 0,
            view: [::core::ptr::null_mut::<MeleeTeam>(); 5],
            top: 0,
            bot: 0,
            cur: 0,
            viewSize: 0,
        },
        currentShip: MeleeShip::MELEE_ANDROSYNTH,
        CurIndex: 0,
        buildPickConfirmed: BOOLEAN::FALSE,
        randomContext: ::core::ptr::null_mut::<RandomContext>(),
        LastInputTime: 0,
        hMusic: ::core::ptr::null_mut::<*mut TFB_SoundSample>(),
    };
    pMeleeState = &raw mut MenuState;
    memset(
        pMeleeState as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<MELEE_STATE>(),
    );
    MenuState.InputFunc = Some(
        DoMelee as unsafe extern "C" fn(*mut MELEE_STATE) -> BOOLEAN,
    ) as Option<unsafe extern "C" fn(*mut melee_state) -> BOOLEAN>;
    MenuState.Initialized = BOOLEAN::FALSE;
    MenuState.meleeSetup = MeleeSetup_new();
    MenuState.randomContext = RandomContext_New();
    RandomContext_SeedRandom(MenuState.randomContext, GetTimeCounter());
    let mut player: COUNT = 0;
    player = 0 as COUNT;
    while (player as ::core::ffi::c_int) < 2 as ::core::ffi::c_int {
        *(&raw mut netConnections as *mut *mut NetConnection).offset(player as isize) = ::core::ptr::null_mut::<
            NetConnection,
        >();
        player = player.wrapping_add(1);
    }
    MenuState.currentShip = MeleeShip::MELEE_NONE;
    MenuState.CurIndex = -1 as ::core::ffi::c_int as COUNT;
    InitMeleeLoadState(&raw mut MenuState);
    GlobData.Game_state.CurrentActivity = C2Rust_Unnamed_0::SUPER_MELEE.0
        as ::core::ffi::c_int as ACTIVITY;
    GameSounds = CaptureStringTable(
        uqm_lua_checked!(LoadSoundInstance(b"sounds.battle\0".as_ptr() as RESOURCE), ()),
    ) as SOUND;
    uqm_lua_checked!(LoadMeleeInfo(&raw mut MenuState), ());
    if uqm_lua_checked!(LoadMeleeConfig(&raw mut MenuState), ()) == -1 as ::core::ffi::c_int {
        *(&raw mut PlayerControl as *mut BYTE).offset(0isize) = (((1
            as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as BYTE
            as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as BYTE
                as ::core::ffi::c_int) as BYTE;
        uqm_lua_checked!(Melee_LocalChange_team(
            &raw mut MenuState,
            0 as size_t,
            *MenuState.load.preBuiltList.offset(0isize),
        ), ());
        *(&raw mut PlayerControl as *mut BYTE).offset(1isize) = (((1
            as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as BYTE
            as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as BYTE
                as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as BYTE
                as ::core::ffi::c_int) as BYTE;
        uqm_lua_checked!(Melee_LocalChange_team(
            &raw mut MenuState,
            1 as size_t,
            *MenuState.load.preBuiltList.offset(1isize),
        ), ());
    }
    memcpy(
        &raw mut savedControl as *mut BYTE as *mut ::core::ffi::c_void,
        &raw mut PlayerControl as *mut BYTE as *const ::core::ffi::c_void,
        ::core::mem::size_of::<[BYTE; 2]>(),
    );
    memcpy(
        &raw mut savedTemplates as *mut CONTROL_TEMPLATE as *mut ::core::ffi::c_void,
        &raw mut PlayerControls as *mut CONTROL_TEMPLATE as *const ::core::ffi::c_void,
        ::core::mem::size_of::<[CONTROL_TEMPLATE; 2]>(),
    );
    if webSession.0 != 0 {
        *(&raw mut PlayerControl as *mut BYTE).offset(webNetplayRole as isize) = (((1
            as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as BYTE
            as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as BYTE
                as ::core::ffi::c_int) as BYTE;
        *(&raw mut PlayerControl as *mut BYTE)
            .offset((1 as ::core::ffi::c_int - webNetplayRole) as isize) = (((1
            as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as BYTE
            as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as BYTE
                as ::core::ffi::c_int) as BYTE;
        *(&raw mut PlayerControls as *mut CONTROL_TEMPLATE)
            .offset(webNetplayRole as isize) = CONTROL_TEMPLATE::CONTROL_TEMPLATE_PL_1;
    }
    MenuState.side = 0 as COUNT;
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
    uqm_lua_checked!(DoInput(&raw mut MenuState as *mut ::core::ffi::c_void, BOOLEAN::TRUE), ());
    uqm_lua_checked!(StopMusic(), ());
    uqm_lua_checked!(WaitForSoundEnd(!(0 as ::core::ffi::c_int) as COUNT), ());
    if webSession.0 == 0 {
        uqm_lua_checked!(WriteMeleeConfig(&raw mut MenuState), ());
    }
    uqm_lua_checked!(FreeMeleeInfo(&raw mut MenuState), ());
    if webSession.0 != 0 {
        memcpy(
            &raw mut PlayerControl as *mut BYTE as *mut ::core::ffi::c_void,
            &raw mut savedControl as *mut BYTE as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[BYTE; 2]>(),
        );
        memcpy(
            &raw mut PlayerControls as *mut CONTROL_TEMPLATE as *mut ::core::ffi::c_void,
            &raw mut savedTemplates as *mut CONTROL_TEMPLATE
                as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[CONTROL_TEMPLATE; 2]>(),
        );
    }
    uqm_lua_checked!(DestroySound(ReleaseStringTable(GameSounds)), ());
    GameSounds = ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>();
    UninitMeleeLoadState(&raw mut MenuState);
    RandomContext_Delete(MenuState.randomContext);
    MeleeSetup_delete(MenuState.meleeSetup);
    FlushInput();
}
#[no_mangle]
pub unsafe extern "C" fn updateRandomSeed(
    mut pMS: *mut MELEE_STATE,
    mut side: COUNT,
    mut seed: DWORD,
) {
    TFB_SeedRandom(seed);
}
#[no_mangle]
pub unsafe extern "C" fn confirmationCancelled(
    mut pMS: *mut MELEE_STATE,
    mut side: COUNT,
) {
    if side as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        uqm_lua_checked!(DrawMeleeStatusMessage(
            GetStringAddress(
                SetAbsStringTableIndex(
                    GameStrings,
                    (C2Rust_Unnamed_9::NETMELEE_STRING_BASE.0 as ::core::ffi::c_int
                        + 16 as ::core::ffi::c_int) as COUNT,
                ),
            ),
        ), ());
    } else {
        uqm_lua_checked!(DrawMeleeStatusMessage(
            GetStringAddress(
                SetAbsStringTableIndex(
                    GameStrings,
                    (C2Rust_Unnamed_9::NETMELEE_STRING_BASE.0 as ::core::ffi::c_int
                        + 17 as ::core::ffi::c_int) as COUNT,
                ),
            ),
        ), ());
    }
    if (*pMS).InputFunc
        == Some(DoConfirmSettings as unsafe extern "C" fn(*mut MELEE_STATE) -> BOOLEAN)
    {
        (*pMS).InputFunc = Some(
            DoMelee as unsafe extern "C" fn(*mut MELEE_STATE) -> BOOLEAN,
        ) as Option<unsafe extern "C" fn(*mut melee_state) -> BOOLEAN>;
    }
}
unsafe extern "C" fn connectionFeedback(
    mut conn: *mut NetConnection,
    mut str: *const ::core::ffi::c_char,
    mut forcePopup: bool,
) {
    let mut bs: *mut battlestate_struct = NetMelee_getBattleState(conn);
    if bs.is_null() && !forcePopup {
        uqm_lua_checked!(DrawMeleeStatusMessage(str), ());
    } else {
        uqm_lua_checked!(DoPopupWindow(str), ());
    };
}
#[no_mangle]
pub unsafe extern "C" fn connectedFeedback(mut conn: *mut NetConnection) {
    if NetConnection_getPlayerNr(conn) == 0 as ::core::ffi::c_int {
        uqm_lua_checked!(connectionFeedback(
            conn,
            GetStringAddress(
                SetAbsStringTableIndex(
                    GameStrings,
                    (C2Rust_Unnamed_9::NETMELEE_STRING_BASE.0 as ::core::ffi::c_int
                        + 8 as ::core::ffi::c_int) as COUNT,
                ),
            ),
            false,
        ), ());
    } else {
        uqm_lua_checked!(connectionFeedback(
            conn,
            GetStringAddress(
                SetAbsStringTableIndex(
                    GameStrings,
                    (C2Rust_Unnamed_9::NETMELEE_STRING_BASE.0 as ::core::ffi::c_int
                        + 9 as ::core::ffi::c_int) as COUNT,
                ),
            ),
            false,
        ), ());
    }
    uqm_lua_checked!(PlayMenuSound(MENU_SOUND_EFFECT::MENU_SOUND_INVOKED), ());
}
unsafe extern "C" fn abortReasonString(
    mut reason: NetplayAbortReason,
) -> *const ::core::ffi::c_char {
    match reason {
        NetplayAbortReason::AbortReason_unspecified => {
            return GetStringAddress(
                SetAbsStringTableIndex(
                    GameStrings,
                    (C2Rust_Unnamed_9::NETMELEE_STRING_BASE.0 as ::core::ffi::c_int
                        + 25 as ::core::ffi::c_int) as COUNT,
                ),
            );
        }
        NetplayAbortReason::AbortReason_versionMismatch => {
            return GetStringAddress(
                SetAbsStringTableIndex(
                    GameStrings,
                    (C2Rust_Unnamed_9::NETMELEE_STRING_BASE.0 as ::core::ffi::c_int
                        + 26 as ::core::ffi::c_int) as COUNT,
                ),
            );
        }
        NetplayAbortReason::AbortReason_invalidHash => {
            return GetStringAddress(
                SetAbsStringTableIndex(
                    GameStrings,
                    (C2Rust_Unnamed_9::NETMELEE_STRING_BASE.0 as ::core::ffi::c_int
                        + 27 as ::core::ffi::c_int) as COUNT,
                ),
            );
        }
        NetplayAbortReason::AbortReason_protocolError => {
            return GetStringAddress(
                SetAbsStringTableIndex(
                    GameStrings,
                    (C2Rust_Unnamed_9::NETMELEE_STRING_BASE.0 as ::core::ffi::c_int
                        + 28 as ::core::ffi::c_int) as COUNT,
                ),
            );
        }
        _ => {}
    }
    return ::core::ptr::null::<::core::ffi::c_char>();
}
#[no_mangle]
pub unsafe extern "C" fn abortFeedback(
    mut conn: *mut NetConnection,
    mut reason: NetplayAbortReason,
) {
    let mut msg: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    msg = abortReasonString(reason);
    if !msg.is_null() {
        uqm_lua_checked!(connectionFeedback(conn, msg, true), ());
    }
}
unsafe extern "C" fn resetReasonString(
    mut reason: NetplayResetReason,
) -> *const ::core::ffi::c_char {
    match reason {
        NetplayResetReason::ResetReason_unspecified => {
            return GetStringAddress(
                SetAbsStringTableIndex(
                    GameStrings,
                    (C2Rust_Unnamed_9::NETMELEE_STRING_BASE.0 as ::core::ffi::c_int
                        + 29 as ::core::ffi::c_int) as COUNT,
                ),
            );
        }
        NetplayResetReason::ResetReason_syncLoss => {
            return GetStringAddress(
                SetAbsStringTableIndex(
                    GameStrings,
                    (C2Rust_Unnamed_9::NETMELEE_STRING_BASE.0 as ::core::ffi::c_int
                        + 30 as ::core::ffi::c_int) as COUNT,
                ),
            );
        }
        NetplayResetReason::ResetReason_manualReset => {
            return GetStringAddress(
                SetAbsStringTableIndex(
                    GameStrings,
                    (C2Rust_Unnamed_9::NETMELEE_STRING_BASE.0 as ::core::ffi::c_int
                        + 31 as ::core::ffi::c_int) as COUNT,
                ),
            );
        }
        _ => {}
    }
    return ::core::ptr::null::<::core::ffi::c_char>();
}
#[no_mangle]
pub unsafe extern "C" fn resetFeedback(
    mut conn: *mut NetConnection,
    mut reason: NetplayResetReason,
    mut byRemote: bool,
) {
    let mut msg: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    uqm_lua_checked!(flushPacketQueues(), ());
    if reason.0 == NetplayResetReason::ResetReason_manualReset.0 && !byRemote {
        return;
    }
    msg = resetReasonString(reason);
    if !msg.is_null() {
        uqm_lua_checked!(connectionFeedback(conn, msg, false), ());
    }
    GlobData.Game_state.CurrentActivity = (GlobData.Game_state.CurrentActivity
        as ::core::ffi::c_int | C2Rust_Unnamed_0::CHECK_ABORT.0 as ::core::ffi::c_int)
        as ACTIVITY;
}
#[no_mangle]
pub unsafe extern "C" fn errorFeedback(mut conn: *mut NetConnection) {
    if NetConnection_getPlayerNr(conn) == 0 as ::core::ffi::c_int {
        uqm_lua_checked!(connectionFeedback(
            conn,
            GetStringAddress(
                SetAbsStringTableIndex(
                    GameStrings,
                    (C2Rust_Unnamed_9::NETMELEE_STRING_BASE.0 as ::core::ffi::c_int
                        + 10 as ::core::ffi::c_int) as COUNT,
                ),
            ),
            false,
        ), ());
    } else {
        uqm_lua_checked!(connectionFeedback(
            conn,
            GetStringAddress(
                SetAbsStringTableIndex(
                    GameStrings,
                    (C2Rust_Unnamed_9::NETMELEE_STRING_BASE.0 as ::core::ffi::c_int
                        + 11 as ::core::ffi::c_int) as COUNT,
                ),
            ),
            false,
        ), ());
    };
}
unsafe extern "C" fn closeWebMeleeRoom(mut message: *const ::core::ffi::c_char) -> bool {
    if webNetplayRole < 0 as ::core::ffi::c_int {
        return false;
    }
    if GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
        & C2Rust_Unnamed_0::CHECK_ABORT.0 as ::core::ffi::c_int == 0
    {
        uqm_lua_checked!(DoPopupWindow(message), false);
        GlobData.Game_state.CurrentActivity = (GlobData.Game_state.CurrentActivity
            as ::core::ffi::c_int
            | C2Rust_Unnamed_0::CHECK_ABORT.0 as ::core::ffi::c_int) as ACTIVITY;
    }
    return true;
}
#[no_mangle]
pub unsafe extern "C" fn closeFeedback(mut conn: *mut NetConnection) {
    let mut message: *const ::core::ffi::c_char = GetStringAddress(
        SetAbsStringTableIndex(
            GameStrings,
            (C2Rust_Unnamed_9::NETMELEE_STRING_BASE.0 as ::core::ffi::c_int
                + if NetConnection_getPlayerNr(conn) == 0 as ::core::ffi::c_int {
                    12 as ::core::ffi::c_int
                } else {
                    13 as ::core::ffi::c_int
                }) as COUNT,
        ),
    );
    if uqm_lua_checked!(closeWebMeleeRoom(message), ()) {
        return;
    }
    uqm_lua_checked!(connectionFeedback(conn, message, false), ());
}
unsafe extern "C" fn Melee_UpdateView_fleetValue(
    mut pMS: *mut MELEE_STATE,
    mut side: COUNT,
) {
    if (*pMS).meleeStarted.0 != 0 {
        return;
    }
    uqm_lua_checked!(DrawFleetValue(pMS, side, 4 as COUNT), ());
}
unsafe extern "C" fn Melee_UpdateView_ship(
    mut pMS: *mut MELEE_STATE,
    mut side: COUNT,
    mut index: FleetShipIndex,
) {
    let mut ship: MeleeShip = MeleeShip::MELEE_ANDROSYNTH;
    if (*pMS).meleeStarted.0 != 0 {
        return;
    }
    ship = MeleeSetup_getShip((*pMS).meleeSetup, side as size_t, index);
    if ship.0 == MeleeShip::MELEE_NONE.0 {
        uqm_lua_checked!(ClearShipBox(side, index), ());
    } else {
        uqm_lua_checked!(DrawShipBox(side, index, ship, BOOLEAN::FALSE), ());
    };
}
unsafe extern "C" fn Melee_UpdateView_teamName(
    mut pMS: *mut MELEE_STATE,
    mut side: COUNT,
) {
    if (*pMS).meleeStarted.0 != 0 {
        return;
    }
    uqm_lua_checked!(DrawTeamString(pMS, side, 4 as COUNT, ::core::ptr::null::<::core::ffi::c_char>()), ());
}
unsafe extern "C" fn Melee_Change_ship(
    mut pMS: *mut MELEE_STATE,
    mut side: COUNT,
    mut index: FleetShipIndex,
    mut ship: MeleeShip,
) -> bool {
    if !MeleeSetup_setShip((*pMS).meleeSetup, side as size_t, index, ship) {
        uqm_lua_checked!(Melee_UpdateView_ship(pMS, side, index), false);
        return false;
    }
    uqm_lua_checked!(Melee_UpdateView_ship(pMS, side, index), false);
    uqm_lua_checked!(Melee_UpdateView_fleetValue(pMS, side), false);
    if isShipSlotSelected(pMS, side, index) {
        (*pMS).currentShip = ship;
        uqm_lua_checked!(DrawMeleeShipStrings(pMS, ship), false);
    }
    return true;
}
unsafe extern "C" fn Melee_Change_teamName(
    mut pMS: *mut MELEE_STATE,
    mut side: COUNT,
    mut name: *const ::core::ffi::c_char,
) -> bool {
    let mut setup: *mut MeleeSetup = (*pMS).meleeSetup;
    if !MeleeSetup_setTeamName(setup, side as size_t, name) {
        return false;
    }
    if (*pMS).row as ::core::ffi::c_int != 2 as ::core::ffi::c_int
        || (*pMS).side as ::core::ffi::c_int != side as ::core::ffi::c_int
        || (*pMeleeState).CurIndex as ::core::ffi::c_int
            == -1 as ::core::ffi::c_int as COUNT as ::core::ffi::c_int
    {
        uqm_lua_checked!(Melee_UpdateView_teamName(pMS, side), false);
    }
    return true;
}
#[no_mangle]
pub unsafe extern "C" fn Melee_LocalChange_ship(
    mut pMS: *mut MELEE_STATE,
    mut side: COUNT,
    mut index: FleetShipIndex,
    mut ship: MeleeShip,
) -> bool {
    if !uqm_lua_checked!(Melee_Change_ship(pMS, side, index, ship), false) {
        return false;
    }
    let mut setup: *mut MeleeSetup = (*pMS).meleeSetup;
    let mut sentShip: MeleeShip = MeleeSetup_getSentShip(setup, side as size_t, index);
    if sentShip.0 == MeleeShip::MELEE_UNSET.0 {
        Netplay_NotifyAll_setShip(pMS, side as size_t, index as size_t);
        MeleeSetup_setSentShip(setup, side as size_t, index, ship);
    }
    return true;
}
#[no_mangle]
pub unsafe extern "C" fn Melee_LocalChange_teamName(
    mut pMS: *mut MELEE_STATE,
    mut side: COUNT,
    mut name: *const ::core::ffi::c_char,
) -> bool {
    if !uqm_lua_checked!(Melee_Change_teamName(pMS, side, name), false) {
        return false;
    }
    let mut setup: *mut MeleeSetup = (*pMS).meleeSetup;
    let mut sentName: *const ::core::ffi::c_char = MeleeSetup_getSentTeamName(
        setup,
        side as size_t,
    );
    if sentName.is_null() {
        Netplay_NotifyAll_setTeamName(pMS, side as size_t);
        MeleeSetup_setSentTeamName(setup, side as size_t, name);
    }
    return true;
}
#[no_mangle]
pub unsafe extern "C" fn Melee_LocalChange_fleet(
    mut pMS: *mut MELEE_STATE,
    mut teamNr: size_t,
    mut fleet: *const MeleeShip,
) -> bool {
    let mut slotI: FleetShipIndex = 0;
    let mut changed: bool = false;
    slotI = 0 as FleetShipIndex;
    while (slotI as ::core::ffi::c_int) < 14 as ::core::ffi::c_int {
        if uqm_lua_checked!(Melee_LocalChange_ship(
            pMS,
            teamNr as COUNT,
            slotI,
            *fleet.offset(slotI as isize),
        ), false) {
            changed = true;
        }
        slotI = slotI.wrapping_add(1);
    }
    return changed;
}
#[no_mangle]
pub unsafe extern "C" fn Melee_LocalChange_team(
    mut pMS: *mut MELEE_STATE,
    mut teamNr: size_t,
    mut team: *const MeleeTeam,
) -> bool {
    let mut fleet: *const MeleeShip = MeleeTeam_getFleet(team);
    let mut name: *const ::core::ffi::c_char = MeleeTeam_getTeamName(team);
    let mut changed: bool = false;
    if uqm_lua_checked!(Melee_LocalChange_fleet(pMS, teamNr, fleet), false) {
        changed = true;
    }
    if uqm_lua_checked!(Melee_LocalChange_teamName(pMS, teamNr as COUNT, name), false) {
        changed = true;
    }
    return changed;
}
#[no_mangle]
pub unsafe extern "C" fn Melee_bootstrapSyncTeam(
    mut meleeState: *mut MELEE_STATE,
    mut teamNr: size_t,
) {
    let mut setup: *mut MeleeSetup = (*meleeState).meleeSetup;
    let mut slotI: FleetShipIndex = 0;
    let mut teamName: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    Netplay_NotifyAll_setFleet(meleeState, teamNr);
    slotI = 0 as FleetShipIndex;
    while (slotI as ::core::ffi::c_int) < 14 as ::core::ffi::c_int {
        let mut ship: MeleeShip = MeleeSetup_getShip(setup, teamNr, slotI);
        (MeleeSetup_getSentShip(setup, teamNr, slotI).0 == MeleeShip::MELEE_UNSET.0
            || {
                __assert_fail(
                    b"MeleeSetup_getSentShip (setup, teamNr, slotI) == MELEE_UNSET\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    b"src/uqm/supermelee/melee.c\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    3333 as ::core::ffi::c_int,
                    b"Melee_bootstrapSyncTeam\0".as_ptr() as *const ::core::ffi::c_char,
                );
                0 as ::core::ffi::c_int != 0
            }) as ::core::ffi::c_int;
        MeleeSetup_setSentShip(setup, teamNr, slotI, ship);
        slotI = slotI.wrapping_add(1);
    }
    Netplay_NotifyAll_setTeamName(meleeState, teamNr);
    teamName = MeleeSetup_getTeamName(setup, teamNr);
    MeleeSetup_setSentTeamName(setup, teamNr, teamName);
}
#[no_mangle]
pub unsafe extern "C" fn Melee_RemoteChange_ship(
    mut pMS: *mut MELEE_STATE,
    mut conn: *mut NetConnection,
    mut side: COUNT,
    mut index: FleetShipIndex,
    mut ship: MeleeShip,
) {
    let mut setup: *mut MeleeSetup = (*pMS).meleeSetup;
    let mut sentShip: MeleeShip = MeleeSetup_getSentShip(setup, side as size_t, index);
    let mut currentShip: MeleeShip = MeleeShip::MELEE_ANDROSYNTH;
    if sentShip.0 == MeleeShip::MELEE_UNSET.0 {
        uqm_lua_checked!(Melee_Change_ship(pMS, side, index, ship), ());
        Netplay_NotifyAll_setShip(pMS, side as size_t, index as size_t);
        return;
    }
    MeleeSetup_setSentShip(setup, side as size_t, index, MeleeShip::MELEE_UNSET);
    if ship.0 != sentShip.0 {
        if NetConnection_getPlayerNr(conn) != side as ::core::ffi::c_int {} else {
            uqm_lua_checked!(Melee_Change_ship(pMS, side, index, ship), ());
            return;
        }
    }
    currentShip = MeleeSetup_getShip(setup, side as size_t, index);
    if currentShip.0 != sentShip.0 {
        Netplay_NotifyAll_setShip(pMS, side as size_t, index as size_t);
        MeleeSetup_setSentShip(setup, side as size_t, index, ship);
    }
}
#[no_mangle]
pub unsafe extern "C" fn Melee_RemoteChange_teamName(
    mut pMS: *mut MELEE_STATE,
    mut conn: *mut NetConnection,
    mut side: COUNT,
    mut newName: *const ::core::ffi::c_char,
) {
    let mut setup: *mut MeleeSetup = (*pMS).meleeSetup;
    let mut sentName: *const ::core::ffi::c_char = MeleeSetup_getSentTeamName(
        setup,
        side as size_t,
    );
    let mut currentName: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    if sentName.is_null() {
        uqm_lua_checked!(Melee_Change_teamName(pMS, side, newName), ());
        Netplay_NotifyAll_setTeamName(pMS, side as size_t);
        return;
    }
    if strcmp(newName, sentName) == 0 as ::core::ffi::c_int {
        if NetConnection_getPlayerNr(conn) != side as ::core::ffi::c_int {} else {
            uqm_lua_checked!(Melee_Change_teamName(pMS, side, newName), ());
            MeleeSetup_setSentTeamName(
                setup,
                side as size_t,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
            return;
        }
    }
    currentName = MeleeSetup_getTeamName(setup, side as size_t);
    if strcmp(currentName, sentName) != 0 as ::core::ffi::c_int {
        Netplay_NotifyAll_setTeamName(pMS, side as size_t);
        MeleeSetup_setSentTeamName(setup, side as size_t, newName);
    } else {
        MeleeSetup_setSentTeamName(
            setup,
            side as size_t,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
    };
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
                b"src/uqm/supermelee/../master.h\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                50 as ::core::ffi::c_int,
                b"LockMasterShip\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    return LockLink(pq, h) as *mut MASTER_SHIP_INFO;
}
