// Copyright Michael Martin, 2004.

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

// Rust translation of src/uqm/setupmenu.c.
extern "C" {
    pub type uio_DirHandle;
    pub type uio_Stream;
    pub type string_table_entry;
    pub type string_table;
    pub type frame_desc;
    pub type font_desc;
    pub type drawable_desc;
    pub type joy_char;
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
    fn sscanf(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn uio_fopen(
        dir: *mut uio_DirHandle,
        path: *const ::core::ffi::c_char,
        mode: *const ::core::ffi::c_char,
    ) -> *mut uio_Stream;
    fn uio_fclose(stream: *mut uio_Stream) -> ::core::ffi::c_int;
    fn uio_fprintf(
        stream: *mut uio_Stream,
        format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn LoadStringTableInstance(res: RESOURCE) -> STRING_TABLE;
    fn DestroyStringTable(StringTable: STRING_TABLE) -> BOOLEAN;
    fn CaptureStringTable(StringTable: STRING_TABLE) -> STRING;
    fn ReleaseStringTable(String: STRING) -> STRING_TABLE;
    fn GetStringTableCount(String: STRING) -> COUNT;
    fn SetAbsStringTableIndex(String: STRING, StringTableIndex: COUNT) -> STRING;
    fn GetStringAddress(String: STRING) -> STRINGPTR;
    fn HMalloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn HFree(p: *mut ::core::ffi::c_void);
    fn utf8StringCount(start: *const ::core::ffi::c_char) -> size_t;
    fn utf8StringCopy(
        dst: *mut ::core::ffi::c_char,
        size: size_t,
        src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_uchar;
    fn SaveResourceIndex(
        dir: *mut uio_DirHandle,
        rmpfile: *const ::core::ffi::c_char,
        root: *const ::core::ffi::c_char,
        strip_root: BOOLEAN,
    );
    fn res_PutString(key: *const ::core::ffi::c_char, value: *const ::core::ffi::c_char);
    fn res_PutInteger(key: *const ::core::ffi::c_char, value: ::core::ffi::c_int);
    fn res_GetBoolean(key: *const ::core::ffi::c_char) -> BOOLEAN;
    fn res_PutBoolean(key: *const ::core::ffi::c_char, value: BOOLEAN);
    fn exp(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn fabs(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn log(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn font_DrawText(pText: *mut TEXT);
    fn BatchGraphics();
    fn UnbatchGraphics();
    fn FlushGraphics();
    fn DestroyDrawable(Drawable: DRAWABLE) -> BOOLEAN;
    fn LoadGraphicInstance(res: RESOURCE) -> *mut ::core::ffi::c_void;
    fn SetAbsFrameIndex(Frame: FRAME, FrameIndex: COUNT) -> FRAME;
    fn CaptureDrawable(Drawable: DRAWABLE) -> FRAME;
    fn ReleaseDrawable(Frame: FRAME) -> DRAWABLE;
    fn FadeScreen(fadeType: ScreenFadeType, TimeInterval: SIZE) -> DWORD;
    fn FlushColorXForms();
    fn atoi(_: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn exit(_: ::core::ffi::c_int) -> !;
    fn memcpy(
        _: *mut ::core::ffi::c_void,
        _: *const ::core::ffi::c_void,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strncpy(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: size_t,
    ) -> *mut ::core::ffi::c_char;
    fn GetTimeCounter() -> TimeCount;
    fn VControl_AddGestureBinding(
        g: *mut VCONTROL_GESTURE,
        target: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn VControl_RemoveGestureBinding(
        g: *mut VCONTROL_GESTURE,
        target: *mut ::core::ffi::c_int,
    );
    fn VControl_DumpGesture(
        buf: *mut ::core::ffi::c_char,
        n: ::core::ffi::c_int,
        g: *mut VCONTROL_GESTURE,
    ) -> ::core::ffi::c_int;
    fn VControl_ClearGesture();
    fn VControl_GetLastGesture(g: *mut VCONTROL_GESTURE) -> ::core::ffi::c_int;
    static mut curr_bindings: [MENU_BINDINGS; 31];
    static mut def_bindings: [MENU_BINDINGS; 31];
    static mut PulsedInputState: CONTROLLER_INPUT_STATE;
    static mut PlayerControls: [CONTROL_TEMPLATE; 0];
    fn FlushInput();
    fn SetDefaultMenuRepeatDelay();
    fn DoPopupWindow(msg: *const ::core::ffi::c_char);
    fn DoPopupWindowFont(msg: *const ::core::ffi::c_char, font: FONT);
    fn DoInput(pInputState: *mut ::core::ffi::c_void, resetInput: BOOLEAN);
    static mut GamePaused: BOOLEAN;
    fn DoTextEntry(pTES: *mut TEXTENTRY_STATE) -> BOOLEAN;
    fn TestSpeechSound(snd: STRING);
    static mut optWhichCoarseScan: ::core::ffi::c_int;
    static mut optWhichMenu: ::core::ffi::c_int;
    static mut optWhichFonts: ::core::ffi::c_int;
    static mut optWhichIntro: ::core::ffi::c_int;
    static mut optWhichShield: ::core::ffi::c_int;
    static mut optSmoothScroll: ::core::ffi::c_int;
    static mut optMeleeScale: ::core::ffi::c_int;
    static mut loresBlowupScale: ::core::ffi::c_uint;
    static mut resolutionFactor: ::core::ffi::c_uint;
    static mut audioDriver: ::core::ffi::c_uint;
    static mut audioQuality: ::core::ffi::c_uint;
    static mut optRequiresReload: BOOLEAN;
    static mut optRequiresRestart: BOOLEAN;
    static mut optCheatMode: OPT_ENABLABLE;
    static mut optGodModes: ::core::ffi::c_int;
    static mut timeDilationScale: ::core::ffi::c_int;
    static mut optBubbleWarp: OPT_ENABLABLE;
    static mut optUnlockShips: OPT_ENABLABLE;
    static mut optHeadStart: OPT_ENABLABLE;
    static mut optAutoButtons: OPT_ENABLABLE;
    static mut optInfiniteRU: OPT_ENABLABLE;
    static mut optSkipIntro: OPT_ENABLABLE;
    static mut optMainMenuMusic: OPT_ENABLABLE;
    static mut optNebulae: OPT_ENABLABLE;
    static mut optOrbitingPlanets: OPT_ENABLABLE;
    static mut optTexturedPlanets: OPT_ENABLABLE;
    static mut optDateFormat: ::core::ffi::c_int;
    static mut optInfiniteFuel: OPT_ENABLABLE;
    static mut optPartialPickup: OPT_ENABLABLE;
    static mut optSubmenu: OPT_ENABLABLE;
    static mut optInfiniteCredits: OPT_ENABLABLE;
    static mut optCustomBorder: OPT_ENABLABLE;
    static mut optSeedType: ::core::ffi::c_int;
    static mut optCustomSeed: ::core::ffi::c_int;
    static mut optShipSeed: OPT_ENABLABLE;
    static mut optSphereColors: ::core::ffi::c_int;
    static mut optSpaceMusic: ::core::ffi::c_int;
    static mut optVolasMusic: OPT_ENABLABLE;
    static mut optWholeFuel: OPT_ENABLABLE;
    static mut optDirJoy: [::core::ffi::c_int; 2];
    static mut optLanderHold: ::core::ffi::c_int;
    static mut optScrTrans: ::core::ffi::c_int;
    static mut optDifficulty: ::core::ffi::c_int;
    static mut optDiffChooser: ::core::ffi::c_int;
    static mut optFuelRange: ::core::ffi::c_int;
    static mut optExtended: OPT_ENABLABLE;
    static mut optNomad: ::core::ffi::c_int;
    static mut optGameOver: OPT_ENABLABLE;
    static mut optShipDirectionIP: OPT_ENABLABLE;
    static mut optHazardColors: OPT_ENABLABLE;
    static mut optOrzCompFont: OPT_ENABLABLE;
    static mut optControllerType: ::core::ffi::c_int;
    static mut optSmartAutoPilot: OPT_ENABLABLE;
    static mut optTintPlanSphere: ::core::ffi::c_int;
    static mut optPlanetStyle: ::core::ffi::c_int;
    static mut optStarBackground: ::core::ffi::c_int;
    static mut optScanStyle: ::core::ffi::c_int;
    static mut optNonStopOscill: OPT_ENABLABLE;
    static mut optScopeStyle: ::core::ffi::c_int;
    static mut optSuperPC: ::core::ffi::c_int;
    static mut optHyperStars: OPT_ENABLABLE;
    static mut optPlanetTexture: OPT_ENABLABLE;
    static mut optFlagshipColor: ::core::ffi::c_int;
    static mut optNoHQEncounters: OPT_ENABLABLE;
    static mut optDeCleansing: OPT_ENABLABLE;
    static mut optMeleeObstacles: OPT_ENABLABLE;
    static mut optShowVisitedStars: OPT_ENABLABLE;
    static mut optUnscaledStarSystem: OPT_ENABLABLE;
    static mut optScanSphere: ::core::ffi::c_int;
    static mut optNebulaeVolume: ::core::ffi::c_int;
    static mut optSlaughterMode: OPT_ENABLABLE;
    static mut optAdvancedAutoPilot: OPT_ENABLABLE;
    static mut optMeleeToolTips: OPT_ENABLABLE;
    static mut optMusicResume: ::core::ffi::c_int;
    static mut optWindowType: DWORD;
    static mut optScatterElements: OPT_ENABLABLE;
    static mut optShowUpgrades: OPT_ENABLABLE;
    static mut optFleetPointSys: OPT_ENABLABLE;
    static mut optShipStore: OPT_ENABLABLE;
    static mut optCaptainNames: OPT_ENABLABLE;
    static mut optDosMenus: OPT_ENABLABLE;
    static mut optDeviceArray: [OPT_ADD_REMOVE; 28];
    static mut optUpgradeArray: [OPT_ADD_REMOVE; 13];
    static mut optHyperSpaceColor: ::core::ffi::c_int;
    static mut DeadZoneLeftStick: [::core::ffi::c_int; 2];
    static mut DeadZoneRightStick: [::core::ffi::c_int; 2];
    static mut opt3doMusic: OPT_ENABLABLE;
    static mut optRemixMusic: OPT_ENABLABLE;
    static mut optSpeech: OPT_ENABLABLE;
    static mut optSubtitles: OPT_ENABLABLE;
    static mut optStereoSFX: OPT_ENABLABLE;
    static mut optKeepAspectRatio: OPT_ENABLABLE;
    static mut optGamma: ::core::ffi::c_float;
    static mut configDir: *mut uio_DirHandle;
    fn isAddonAvailable(addon_name: *const ::core::ffi::c_char) -> BOOLEAN;
    fn setGammaCorrection(gamma: ::core::ffi::c_float) -> bool;
    fn SleepThreadUntil(wakeTime: TimeCount);
    fn TaskSwitch();
    fn LoadSoundInstance(res: RESOURCE) -> SOUND_REF;
    fn DestroySound(SoundRef: SOUND_REF) -> BOOLEAN;
    fn SetMusicVolume(Volume: COUNT);
    static mut CanvasWidth: ::core::ffi::c_int;
    static mut CanvasHeight: ::core::ffi::c_int;
    static mut GlobData: GLOBDATA;
    fn is3DO(optWhich: ::core::ffi::c_int) -> OPT_CONSOLETYPE;
    static mut MicroFont: FONT;
    static mut PlyrFont: FONT;
    fn SetMenuSounds(sound_0: MENU_SOUND_FLAGS, sound_1: MENU_SOUND_FLAGS);
    fn Reload();
    fn InitMenuMusic();
    fn UninitMenuMusic();
    fn DrawLabelAsWindow(label: *mut WIDGET_LABEL, windowRect: *mut RECT);
    fn Widget_SetWindowColors(bg: Color, dark: Color, medium: Color);
    fn Widget_SetFont(newFont: FONT) -> FONT;
    fn Widget_Event(event: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn Widget_ReceiveFocusMenuScreen(
        _self: *mut WIDGET,
        event: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn Widget_ReceiveFocusChoice(
        _self: *mut WIDGET,
        event: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn Widget_ReceiveFocusSimple(
        _self: *mut WIDGET,
        event: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn Widget_ReceiveFocusControlEntry(
        _self: *mut WIDGET,
        event: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn Widget_ReceiveFocusRefuseFocus(
        _self: *mut WIDGET,
        event: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn Widget_ReceiveFocusMenuControlEntry(
        _self: *mut WIDGET,
        event: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn Widget_HandleEventMenuScreen(
        _self: *mut WIDGET,
        event: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn Widget_HandleEventChoice(
        _self: *mut WIDGET,
        event: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn Widget_HandleEventSlider(
        _self: *mut WIDGET,
        event: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn Widget_HandleEventTextEntry(
        _self: *mut WIDGET,
        event: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn Widget_HandleEventControlEntry(
        _self: *mut WIDGET,
        event: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn Widget_HandleEventMenuControlEntry(
        _self: *mut WIDGET,
        event: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn Widget_HandleEventIgnoreAll(
        _self: *mut WIDGET,
        event: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn Widget_HeightChoice(_self: *mut WIDGET) -> ::core::ffi::c_int;
    fn Widget_HeightFullScreen(_self: *mut WIDGET) -> ::core::ffi::c_int;
    fn Widget_HeightOneLine(_self: *mut WIDGET) -> ::core::ffi::c_int;
    fn Widget_HeightLabel(_self: *mut WIDGET) -> ::core::ffi::c_int;
    fn Widget_WidthFullScreen(_self: *mut WIDGET) -> ::core::ffi::c_int;
    fn Widget_DrawMenuScreen(
        _self: *mut WIDGET,
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
    );
    fn Widget_DrawChoice(
        _self: *mut WIDGET,
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
    );
    fn Widget_DrawButton(
        _self: *mut WIDGET,
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
    );
    fn Widget_DrawLabel(
        _self: *mut WIDGET,
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
    );
    fn Widget_DrawSlider(
        _self: *mut WIDGET,
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
    );
    fn Widget_DrawTextEntry(
        _self: *mut WIDGET,
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
    );
    fn Widget_DrawControlEntry(
        _self: *mut WIDGET,
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
    );
    fn Widget_DrawMenuControlEntry(
        _self: *mut WIDGET,
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
    );
    fn Widget_Slider_DrawValue(
        self_0: *mut WIDGET_SLIDER,
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
    );
    fn ResetOffset();
    fn LoadArrows();
    fn ReleaseArrows();
    static mut GfxFlags: ::core::ffi::c_int;
    fn TFB_SetGamma(gamma: ::core::ffi::c_float) -> bool;
    fn SetTransitionSource(pRect: *const RECT);
    fn ScreenTransition(transition: ::core::ffi::c_int, pRect: *const RECT);
    static mut WindowWidth: ::core::ffi::c_int;
    static mut WindowHeight: ::core::ffi::c_int;
    static mut SavedWidth: ::core::ffi::c_int;
    static mut SavedHeight: ::core::ffi::c_int;
    static mut GraphicsDriver: ::core::ffi::c_int;
    fn TFB_DrawScreen_ReinitVideo(
        driver: ::core::ffi::c_int,
        flags: ::core::ffi::c_int,
        width: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
    );
    fn InterrogateInputState(
        templat: ::core::ffi::c_int,
        control: ::core::ffi::c_int,
        index: ::core::ffi::c_int,
        buffer: *mut ::core::ffi::c_char,
        maxlen: ::core::ffi::c_int,
        r#override: *mut VCONTROL_GESTURE,
    );
    fn RemoveInputState(
        templat: ::core::ffi::c_int,
        control: ::core::ffi::c_int,
        index: ::core::ffi::c_int,
    );
    fn RebindInputState(
        templat: ::core::ffi::c_int,
        control: ::core::ffi::c_int,
        index: ::core::ffi::c_int,
    );
    fn SaveKeyConfiguration(path: *mut uio_DirHandle, fname: *const ::core::ffi::c_char);
    static mut menu_res_names: [*const ::core::ffi::c_char; 0];
    static mut menu_vec: *mut ::core::ffi::c_int;
    fn InitVideoPlayer(UseCDROM: BOOLEAN) -> BOOLEAN;
    fn UninitVideoPlayer();
    static mut soundflags: ::core::ffi::c_int;
    static mut snddriver: ::core::ffi::c_int;
    static mut musicVolume: ::core::ffi::c_int;
    static mut musicVolumeScale: ::core::ffi::c_float;
    static mut sfxVolumeScale: ::core::ffi::c_float;
    static mut speechVolumeScale: ::core::ffi::c_float;
    fn SetSpeechVolume(volume: ::core::ffi::c_float);
    fn StringBank_Create() -> *mut stringbank;
    fn StringBank_Free(bank_0: *mut stringbank);
    fn StringBank_AddOrFindString(
        bank_0: *mut stringbank,
        str: *const ::core::ffi::c_char,
    ) -> *const ::core::ffi::c_char;
    fn SplitString(
        s: *const ::core::ffi::c_char,
        splitchar: ::core::ffi::c_char,
        n: ::core::ffi::c_int,
        result: *mut *const ::core::ffi::c_char,
        bank_0: *mut stringbank,
    ) -> ::core::ffi::c_int;
    fn log_add(_: log_Level, fmt: *const ::core::ffi::c_char, ...);
    fn TFB_BBox_Init(width: ::core::ffi::c_int, height: ::core::ffi::c_int);
    fn TFB_BBox_Reset();
    static mut GameStrings: STRING;
}
pub type int32_t = ::core::ffi::c_int;
pub type uint8_t = ::core::ffi::c_uchar;
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
pub type FRAME_DESC = frame_desc;
pub type FONT_DESC = font_desc;
pub type DRAWABLE_DESC = drawable_desc;
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
pub type Uint8 = uint8_t;
pub type Sint32 = int32_t;
pub type SDL_Keycode = Sint32;
pub type sdl_key_t = SDL_Keycode;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct VCONTROL_GESTURE_TYPE(pub ::core::ffi::c_uint);
impl VCONTROL_GESTURE_TYPE {
    pub const VCONTROL_NONE: Self = Self(0);
    pub const VCONTROL_KEY: Self = Self(1);
    pub const VCONTROL_JOYAXIS: Self = Self(2);
    pub const VCONTROL_JOYBUTTON: Self = Self(3);
    pub const VCONTROL_JOYHAT: Self = Self(4);
    pub const NUM_VCONTROL_GESTURES: Self = Self(5);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VCONTROL_GESTURE {
    pub r#type: VCONTROL_GESTURE_TYPE,
    pub gesture: C2Rust_Unnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed {
    pub key: sdl_key_t,
    pub axis: C2Rust_Unnamed_2,
    pub button: C2Rust_Unnamed_1,
    pub hat: C2Rust_Unnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_0 {
    pub port: ::core::ffi::c_int,
    pub index: ::core::ffi::c_int,
    pub dir: Uint8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_1 {
    pub port: ::core::ffi::c_int,
    pub index: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_2 {
    pub port: ::core::ffi::c_int,
    pub index: ::core::ffi::c_int,
    pub polarity: ::core::ffi::c_int,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_3(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_3 {
    pub const KEY_UP: Self = Self(0);
    pub const KEY_DOWN: Self = Self(1);
    pub const KEY_LEFT: Self = Self(2);
    pub const KEY_RIGHT: Self = Self(3);
    pub const KEY_WEAPON: Self = Self(4);
    pub const KEY_SPECIAL: Self = Self(5);
    pub const KEY_ESCAPE: Self = Self(6);
    pub const KEY_THRUST: Self = Self(7);
    pub const NUM_KEYS: Self = Self(8);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_4(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_4 {
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
pub struct menu_bindings {
    pub action: [::core::ffi::c_char; 40],
    pub binding: [VCONTROL_GESTURE; 6],
}
pub type MENU_BINDINGS = menu_bindings;
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct OPT_SCALETYPE(pub ::core::ffi::c_uint);
impl OPT_SCALETYPE {
    pub const OPTVAL_NO_SCALE: Self = Self(0);
    pub const OPTVAL_BILINEAR_SCALE: Self = Self(1);
    pub const OPTVAL_BIADAPT_SCALE: Self = Self(2);
    pub const OPTVAL_BIADV_SCALE: Self = Self(3);
    pub const OPTVAL_TRISCAN_SCALE: Self = Self(4);
    pub const OPTVAL_HQXX_SCALE: Self = Self(5);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct OPT_RESTYPE(pub ::core::ffi::c_uint);
impl OPT_RESTYPE {
    pub const OPTVAL_320_240: Self = Self(0);
    pub const OPTVAL_REAL_1280_960: Self = Self(1);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct OPT_RESSCALER(pub ::core::ffi::c_uint);
impl OPT_RESSCALER {
    pub const NO_BLOWUP: Self = Self(0);
    pub const OPTVAL_SCALE_640_480: Self = Self(1);
    pub const OPTVAL_SCALE_960_720: Self = Self(2);
    pub const OPTVAL_SCALE_1280_960: Self = Self(3);
    pub const OPTVAL_SCALE_1600_1200: Self = Self(4);
    pub const OPTVAL_SCALE_1920_1440: Self = Self(5);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct OPT_DRIVERTYPE(pub ::core::ffi::c_uint);
impl OPT_DRIVERTYPE {
    pub const OPTVAL_PURE_IF_POSSIBLE: Self = Self(0);
    pub const OPTVAL_ALWAYS_GL: Self = Self(1);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct OPT_ADRIVERTYPE(pub ::core::ffi::c_uint);
impl OPT_ADRIVERTYPE {
    pub const OPTVAL_SILENCE: Self = Self(0);
    pub const OPTVAL_MIXSDL: Self = Self(1);
    pub const OPTVAL_OPENAL: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct OPT_AQUALITYTYPE(pub ::core::ffi::c_uint);
impl OPT_AQUALITYTYPE {
    pub const OPTVAL_LOW: Self = Self(0);
    pub const OPTVAL_MEDIUM: Self = Self(1);
    pub const OPTVAL_HIGH: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct OPT_GODTYPE(pub ::core::ffi::c_uint);
impl OPT_GODTYPE {
    pub const OPTVAL_NO_GOD: Self = Self(0);
    pub const OPTVAL_INF_ENERGY: Self = Self(1);
    pub const OPTVAL_INF_HEALTH: Self = Self(2);
    pub const OPTVAL_FULL_GOD: Self = Self(3);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct OPT_TDTYPE(pub ::core::ffi::c_uint);
impl OPT_TDTYPE {
    pub const OPTVAL_NORMAL: Self = Self(0);
    pub const OPTVAL_SLOW: Self = Self(1);
    pub const OPTVAL_FAST: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct OPT_DATETYPE(pub ::core::ffi::c_uint);
impl OPT_DATETYPE {
    pub const OPTVAL_MMMDDYYYY: Self = Self(0);
    pub const OPTVAL_MMDDYYYY: Self = Self(1);
    pub const OPTVAL_DDMMMYYYY: Self = Self(2);
    pub const OPTVAL_DDMMYYYY: Self = Self(3);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct OPT_DIFFICULTY(pub ::core::ffi::c_uint);
impl OPT_DIFFICULTY {
    pub const OPTVAL_NORM: Self = Self(0);
    pub const OPTVAL_EASY: Self = Self(1);
    pub const OPTVAL_HARD: Self = Self(2);
    pub const OPTVAL_IMPO: Self = Self(3);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct OPT_NOMAD(pub ::core::ffi::c_uint);
impl OPT_NOMAD {
    pub const OPTVAL_NOM_OFF: Self = Self(0);
    pub const OPTVAL_NOM_EASY: Self = Self(1);
    pub const OPTVAL_NOM_NORM: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct OPT_CONTROLLER(pub ::core::ffi::c_uint);
impl OPT_CONTROLLER {
    pub const OPTVAL_KBM: Self = Self(0);
    pub const OPTVAL_XBX: Self = Self(1);
    pub const OPTVAL_PS4: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct OPT_FUELRANGE(pub ::core::ffi::c_uint);
impl OPT_FUELRANGE {
    pub const OPTVAL_FUEL_NORM: Self = Self(0);
    pub const OPTVAL_FUEL_AT_DEST: Self = Self(1);
    pub const OPTVAL_FUEL_TO_SOL: Self = Self(2);
    pub const OPTVAL_FUEL_ALL: Self = Self(3);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct OPT_MELEEZOOM(pub ::core::ffi::c_uint);
impl OPT_MELEEZOOM {
    pub const OPTVAL_STEP: Self = Self(0);
    pub const OPTVAL_NEAREST: Self = Self(1);
    pub const OPTVAL_BILINEAR: Self = Self(2);
    pub const OPTVAL_TRILINEAR: Self = Self(3);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct OPT_SPHERETYPE(pub ::core::ffi::c_uint);
impl OPT_SPHERETYPE {
    pub const OPTVAL_PC_SPHERE: Self = Self(0);
    pub const OPTVAL_3DO_SPHERE: Self = Self(1);
    pub const OPTVAL_UQM_SPHERE: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct OPT_WINDOWTYPE(pub ::core::ffi::c_uint);
impl OPT_WINDOWTYPE {
    pub const OPTVAL_PC_WINDOW: Self = Self(0);
    pub const OPTVAL_3DO_WINDOW: Self = Self(1);
    pub const OPTVAL_UQM_WINDOW: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct OPT_MUSICRESUME(pub ::core::ffi::c_uint);
impl OPT_MUSICRESUME {
    pub const OPTVAL_RESUME_DISABLED: Self = Self(0);
    pub const OPTVAL_RESUME_5MIN: Self = Self(1);
    pub const OPTVAL_RESUME_INDEFINITE: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct OPT_SEED(pub ::core::ffi::c_uint);
impl OPT_SEED {
    pub const OPTVAL_PRIME: Self = Self(0);
    pub const OPTVAL_PLANET: Self = Self(1);
    pub const OPTVAL_MRQ: Self = Self(2);
    pub const OPTVAL_STAR: Self = Self(3);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct OPT_SPHERECOLORS(pub ::core::ffi::c_uint);
impl OPT_SPHERECOLORS {
    pub const OPTVAL_DEFAULT_COLORS: Self = Self(0);
    pub const OPTVAL_STARSEED_COLORS: Self = Self(1);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct OPT_SPACEMUSIC(pub ::core::ffi::c_uint);
impl OPT_SPACEMUSIC {
    pub const OPTVAL_NO_SPACEMUSIC: Self = Self(0);
    pub const OPTVAL_SM_NO_SPOILERS: Self = Self(1);
    pub const OPTVAL_SM_SPOILERS: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct OPT_ADD_REMOVE(pub ::core::ffi::c_uint);
impl OPT_ADD_REMOVE {
    pub const OPTVAL_DO_NOTHING: Self = Self(0);
    pub const OPTVAL_REM_THING: Self = Self(1);
    pub const OPTVAL_ADD_THING: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct OPT_DIRECTJOY(pub ::core::ffi::c_uint);
impl OPT_DIRECTJOY {
    pub const OPTVAL_NORMAL_CONTROL: Self = Self(0);
    pub const OPTVAL_DIRECTIONAL_LS_CONTROL: Self = Self(1);
    pub const OPTVAL_DIRECTIONAL_RS_CONTROL: Self = Self(2);
    pub const OPTVAL_DIRECTIONAL_LS_THRUST: Self = Self(3);
    pub const OPTVAL_DIRECTIONAL_RS_THRUST: Self = Self(4);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_5(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_5 {
    pub const CHOICE_GRAPHICS: Self = Self(0);
    pub const CHOICE_FRBUFFER: Self = Self(1);
    pub const CHOICE_SCALER: Self = Self(2);
    pub const CHOICE_SCANLINE: Self = Self(3);
    pub const CHOICE_MENUSTYLE: Self = Self(4);
    pub const CHOICE_FONTSTYLE: Self = Self(5);
    pub const CHOICE_SCANMENU: Self = Self(6);
    pub const CHOICE_SCROLLSTYLE: Self = Self(7);
    pub const CHOICE_SUBTITLES: Self = Self(8);
    pub const CHOICE_REMIXES1: Self = Self(9);
    pub const CHOICE_DISPLAY: Self = Self(10);
    pub const CHOICE_CUTSCENE: Self = Self(11);
    pub const CHOICE_SHOWFPS: Self = Self(12);
    pub const CHOICE_MELEEZOOM: Self = Self(13);
    pub const CHOICE_POSAUDIO: Self = Self(14);
    pub const CHOICE_SNDDRIVER: Self = Self(15);
    pub const CHOICE_SNDQUALITY: Self = Self(16);
    pub const CHOICE_SLVSHIELD: Self = Self(17);
    pub const CHOICE_BTMPLAYER: Self = Self(18);
    pub const CHOICE_TOPPLAYER: Self = Self(19);
    pub const CHOICE_KBLAYOUT: Self = Self(20);
    pub const CHOICE_REMIXES2: Self = Self(21);
    pub const CHOICE_SPEECH: Self = Self(22);
    pub const CHOICE_ASPRATIO: Self = Self(23);
    pub const CHOICE_CHEATING: Self = Self(24);
    pub const CHOICE_CHGODMODE: Self = Self(25);
    pub const CHOICE_CHTIME: Self = Self(26);
    pub const CHOICE_CHWARP: Self = Self(27);
    pub const CHOICE_CHSHIPS: Self = Self(28);
    pub const CHOICE_CHHEADSTART: Self = Self(29);
    pub const CHOICE_AUTOBUTT: Self = Self(30);
    pub const CHOICE_CHINFRU: Self = Self(31);
    pub const CHOICE_SKIPINTRO: Self = Self(32);
    pub const CHOICE_FUELCIRCLE: Self = Self(33);
    pub const CHOICE_MMENUMUSIC: Self = Self(34);
    pub const CHOICE_NEBULAE: Self = Self(35);
    pub const CHOICE_ORBPLANETS: Self = Self(36);
    pub const CHOICE_TEXPLANETS: Self = Self(37);
    pub const CHOICE_DATESTRING: Self = Self(38);
    pub const CHOICE_CHINFFUEL: Self = Self(39);
    pub const CHOICE_PARTPICKUP: Self = Self(40);
    pub const CHOICE_SUBMENU: Self = Self(41);
    pub const CHOICE_RESOLUTION: Self = Self(42);
    pub const CHOICE_CHINFCRD: Self = Self(43);
    pub const CHOICE_HAZARDCLR: Self = Self(44);
    pub const CHOICE_CUSTBORDER: Self = Self(45);
    pub const CHOICE_IPMUSIC: Self = Self(46);
    pub const CHOICE_REMIXES3: Self = Self(47);
    pub const CHOICE_FUELDECIM: Self = Self(48);
    pub const CHOICE_DIRJOYP1: Self = Self(49);
    pub const CHOICE_ANDRZOOM: Self = Self(50);
    pub const CHOICE_LANDERHOLD: Self = Self(51);
    pub const CHOICE_SCRMELT: Self = Self(52);
    pub const CHOICE_SKILLLVL: Self = Self(53);
    pub const CHOICE_EXTENDED: Self = Self(54);
    pub const CHOICE_NOMAD: Self = Self(55);
    pub const CHOICE_GAMEOVER: Self = Self(56);
    pub const CHOICE_IPSHIPDIR: Self = Self(57);
    pub const CHOICE_ORZFONT: Self = Self(58);
    pub const CHOICE_INPDEVICE: Self = Self(59);
    pub const CHOICE_SMARTAUTO: Self = Self(60);
    pub const CHOICE_SCANTINT: Self = Self(61);
    pub const CHOICE_IPSTYLE: Self = Self(62);
    pub const CHOICE_IPBACKGROUND: Self = Self(63);
    pub const CHOICE_SCANSTYLE: Self = Self(64);
    pub const CHOICE_NOSTOSCILL: Self = Self(65);
    pub const CHOICE_OSCILLSTYLE: Self = Self(66);
    pub const CHOICE_ANIMHYPER: Self = Self(67);
    pub const CHOICE_LANDERSTYLE: Self = Self(68);
    pub const CHOICE_PLNTEXTURE: Self = Self(69);
    pub const CHOICE_FLAGSHIP: Self = Self(70);
    pub const CHOICE_CHCLEANHYPER: Self = Self(71);
    pub const CHOICE_CHDECLEAN: Self = Self(72);
    pub const CHOICE_CHNOPLANET: Self = Self(73);
    pub const CHOICE_VISITED: Self = Self(74);
    pub const CHOICE_HDM_IP: Self = Self(75);
    pub const CHOICE_SCANSPHERE: Self = Self(76);
    pub const CHOICE_SLAUGHTER: Self = Self(77);
    pub const CHOICE_ADVAUTO: Self = Self(78);
    pub const CHOICE_MLTOOLTIP: Self = Self(79);
    pub const CHOICE_MUSRESUME: Self = Self(80);
    pub const CHOICE_WINDOWTYPE: Self = Self(81);
    pub const CHOICE_GAMESEED: Self = Self(82);
    pub const CHOICE_SOICOLOR: Self = Self(83);
    pub const CHOICE_SCATTERCARGO: Self = Self(84);
    pub const CHOICE_LANDERUPGMASK: Self = Self(85);
    pub const CHOICE_FLEETPOINT: Self = Self(86);
    pub const CHOICE_DEVSPAWNER: Self = Self(87);
    pub const CHOICE_DEVPET: Self = Self(88);
    pub const CHOICE_DEVBOMB: Self = Self(89);
    pub const CHOICE_DEVSUN: Self = Self(90);
    pub const CHOICE_DEVSPHERE: Self = Self(91);
    pub const CHOICE_DEVHELIX: Self = Self(92);
    pub const CHOICE_DEVSPINDLE: Self = Self(93);
    pub const CHOICE_DEVULTRON0: Self = Self(94);
    pub const CHOICE_DEVULTRON1: Self = Self(95);
    pub const CHOICE_DEVULTRON2: Self = Self(96);
    pub const CHOICE_DEVULTRON3: Self = Self(97);
    pub const CHOICE_DEVMAIDENS: Self = Self(98);
    pub const CHOICE_DEVCASTER0: Self = Self(99);
    pub const CHOICE_DEVCASTER1: Self = Self(100);
    pub const CHOICE_DEVSHIELD: Self = Self(101);
    pub const CHOICE_DEVEGGCS0: Self = Self(102);
    pub const CHOICE_DEVEGGCS1: Self = Self(103);
    pub const CHOICE_DEVEGGCS2: Self = Self(104);
    pub const CHOICE_DEVSHUTTLE: Self = Self(105);
    pub const CHOICE_DEVBEAST: Self = Self(106);
    pub const CHOICE_DEVSLYCODE: Self = Self(107);
    pub const CHOICE_DEVWARPPOD: Self = Self(108);
    pub const CHOICE_DEVTRIDENT: Self = Self(109);
    pub const CHOICE_DEVROD: Self = Self(110);
    pub const CHOICE_DEVLUNBASE: Self = Self(111);
    pub const CHOICE_UPGLANDERSPD: Self = Self(112);
    pub const CHOICE_UPGLANDERCRG: Self = Self(113);
    pub const CHOICE_UPGLANDERFRE: Self = Self(114);
    pub const CHOICE_UPGLANDERBSH: Self = Self(115);
    pub const CHOICE_UPGLANDERQSH: Self = Self(116);
    pub const CHOICE_UPGLANDERLSH: Self = Self(117);
    pub const CHOICE_UPGLANDERHSH: Self = Self(118);
    pub const CHOICE_MODPOINTDEF: Self = Self(119);
    pub const CHOICE_MODFUSBLAST: Self = Self(120);
    pub const CHOICE_MODFUELTANK: Self = Self(121);
    pub const CHOICE_MODTRACKING: Self = Self(122);
    pub const CHOICE_MODHELLBORE: Self = Self(123);
    pub const CHOICE_MODFURNACE: Self = Self(124);
    pub const CHOICE_SHIPSEED: Self = Self(125);
    pub const CHOICE_SHIPSTORE: Self = Self(126);
    pub const CHOICE_CAPTNAMES: Self = Self(127);
    pub const CHOICE_DOSMENUS: Self = Self(128);
    pub const CHOICE_HSCOLOR: Self = Self(129);
    pub const CHOICE_DIRJOYP2: Self = Self(130);
    pub const CHOICE_COUNT: Self = Self(131);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_6(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_6 {
    pub const MENU_QUITSUB: Self = Self(0);
    pub const MENU_GRAPHICS: Self = Self(1);
    pub const MENU_AUDIO: Self = Self(2);
    pub const MENU_ENGINE: Self = Self(3);
    pub const MENU_CHEATS: Self = Self(4);
    pub const MENY_KEYCONF: Self = Self(5);
    pub const MENU_ADVANCED: Self = Self(6);
    pub const MENU_EDITKEYS: Self = Self(7);
    pub const MENU_MUSIC: Self = Self(8);
    pub const MENU_VISUAL: Self = Self(9);
    pub const MENU_QOL: Self = Self(10);
    pub const MENU_DEVICES: Self = Self(11);
    pub const MENU_UPGRADES: Self = Self(12);
    pub const MENU_EDITMENUKEYS: Self = Self(13);
    pub const MENU_DEADZONES: Self = Self(14);
    pub const MENU_COUNT: Self = Self(15);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_7(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_7 {
    pub const SLIDER_MUSVOLUME: Self = Self(0);
    pub const SLIDER_SFXVOLUME: Self = Self(1);
    pub const SLIDER_SPCHVOLUME: Self = Self(2);
    pub const SLIDER_GAMMA: Self = Self(3);
    pub const SLIDER_NEBULA: Self = Self(4);
    pub const SLIDER_DEADZONE_00: Self = Self(5);
    pub const SLIDER_DEADZONE_01: Self = Self(6);
    pub const SLIDER_DEADZONE_02: Self = Self(7);
    pub const SLIDER_DEADZONE_03: Self = Self(8);
    pub const SLIDER_COUNT: Self = Self(9);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_8(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_8 {
    pub const BTN_QUITSETUP: Self = Self(0);
    pub const BTN_QUITSUBMENU: Self = Self(1);
    pub const BTN_GFXMENU: Self = Self(2);
    pub const BTN_ENGMENU: Self = Self(3);
    pub const BTN_SNDMENU: Self = Self(4);
    pub const BTN_CHTMENU: Self = Self(5);
    pub const BTN_KEYMENU: Self = Self(6);
    pub const BTN_ADVMENU: Self = Self(7);
    pub const BTN_EDITKEYS: Self = Self(8);
    pub const BTN_PREVMENU: Self = Self(9);
    pub const BTN_MUSMENU: Self = Self(10);
    pub const BTN_VISMENU: Self = Self(11);
    pub const BTN_QOLMENU: Self = Self(12);
    pub const BTN_DEVMENU: Self = Self(13);
    pub const BTN_UPGMENU: Self = Self(14);
    pub const BTN_CHTPREV: Self = Self(15);
    pub const BTN_EDITMENUKEYS: Self = Self(16);
    pub const BTN_SAVEMENUBINDS: Self = Self(17);
    pub const BTN_CANCELMENUBINDS: Self = Self(18);
    pub const BTN_LOADDEFMENUBINDS: Self = Self(19);
    pub const BTN_EDIT_DEADZONES: Self = Self(20);
    pub const BUTTON_COUNT: Self = Self(21);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_9(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_9 {
    pub const LABEL_UNUSED: Self = Self(0);
    pub const LABEL_KEYSTOOLTIP: Self = Self(1);
    pub const LABEL_TAPTOOLTIP: Self = Self(2);
    pub const LABEL_PRESSTOEDIT: Self = Self(3);
    pub const LABEL_SPACER: Self = Self(4);
    pub const LABEL_UI: Self = Self(5);
    pub const LABEL_COMM: Self = Self(6);
    pub const LABEL_IP: Self = Self(7);
    pub const LABEL_SCAN: Self = Self(8);
    pub const LABEL_COUNT: Self = Self(9);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_10(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_10 {
    pub const TEXT_GAMESEED: Self = Self(0);
    pub const TEXT_CUSTMRES: Self = Self(1);
    pub const TEXTENTRY_COUNT: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_11(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_11 {
    pub const CONTROL_UP: Self = Self(0);
    pub const CONTROL_DOWN: Self = Self(1);
    pub const CONTROL_LEFT: Self = Self(2);
    pub const CONTROL_RGHT: Self = Self(3);
    pub const CONTROL_WEAP: Self = Self(4);
    pub const CONTROL_SPEC: Self = Self(5);
    pub const CONTROL_ESC: Self = Self(6);
    pub const CONTROL_THRU: Self = Self(7);
    pub const CONTROLENTRY_COUNT: Self = Self(8);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_12(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_12 {
    pub const MENUCONTROL_PAUSE: Self = Self(0);
    pub const MENUCONTROL_EXIT: Self = Self(1);
    pub const MENUCONTROL_ABORT: Self = Self(2);
    pub const MENUCONTROL_DEBUG: Self = Self(3);
    pub const MENUCONTROL_FULLSCREEN: Self = Self(4);
    pub const MENUCONTROL_UP: Self = Self(5);
    pub const MENUCONTROL_DOWN: Self = Self(6);
    pub const MENUCONTROL_LEFT: Self = Self(7);
    pub const MENUCONTROL_RIGHT: Self = Self(8);
    pub const MENUCONTROL_SELECT: Self = Self(9);
    pub const MENUCONTROL_CANCEL: Self = Self(10);
    pub const MENUCONTROL_SPECIAL: Self = Self(11);
    pub const MENUCONTROL_PAGEUP: Self = Self(12);
    pub const MENUCONTROL_PAGEDOWN: Self = Self(13);
    pub const MENUCONTROL_HOME: Self = Self(14);
    pub const MENUCONTROL_END: Self = Self(15);
    pub const MENUCONTROL_ZOOMIN: Self = Self(16);
    pub const MENUCONTROL_ZOOMOUT: Self = Self(17);
    pub const MENUCONTROL_DELETE: Self = Self(18);
    pub const MENUCONTROL_BACKSPACE: Self = Self(19);
    pub const MENUCONTROL_EDITCANCEL: Self = Self(20);
    pub const MENUCONTROL_SEARCH: Self = Self(21);
    pub const MENUCONTROL_NEXT: Self = Self(22);
    pub const MENUCONTROL_TOGGLEMAP: Self = Self(23);
    pub const MENUCONTROL_SCREENSHOT: Self = Self(24);
    pub const MENUCONTROL_QUICKSAVE: Self = Self(25);
    pub const MENUCONTROL_QUICKLOAD: Self = Self(26);
    pub const MENUCONTROL_DEBUG2: Self = Self(27);
    pub const MENUCONTROL_DEBUG3: Self = Self(28);
    pub const MENUCONTROL_DEBUG4: Self = Self(29);
    pub const MENUCONTROL_COUNT: Self = Self(30);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct globalopts_struct {
    pub scaler: OPT_SCALETYPE,
    pub screenResolution: OPT_RESTYPE,
    pub loresBlowup: OPT_RESSCALER,
    pub driver: OPT_DRIVERTYPE,
    pub adriver: OPT_ADRIVERTYPE,
    pub aquality: OPT_AQUALITYTYPE,
    pub godModes: OPT_GODTYPE,
    pub tdType: OPT_TDTYPE,
    pub dateType: OPT_DATETYPE,
    pub difficulty: OPT_DIFFICULTY,
    pub nomad: OPT_NOMAD,
    pub meleezoom: OPT_MELEEZOOM,
    pub controllerType: OPT_CONTROLLER,
    pub fuelRange: OPT_FUELRANGE,
    pub sphereType: OPT_SPHERETYPE,
    pub windowType: OPT_WINDOWTYPE,
    pub musicResume: OPT_MUSICRESUME,
    pub seedType: OPT_SEED,
    pub sphereColors: OPT_SPHERECOLORS,
    pub spaceMusic: OPT_SPACEMUSIC,
    pub deviceArray: [OPT_ADD_REMOVE; 28],
    pub upgradeArray: [OPT_ADD_REMOVE; 13],
    pub dirJoy: [OPT_DIRECTJOY; 2],
    pub fullscreen: OPT_ENABLABLE,
    pub subtitles: OPT_ENABLABLE,
    pub scanlines: OPT_ENABLABLE,
    pub fps: OPT_ENABLABLE,
    pub stereo: OPT_ENABLABLE,
    pub music3do: OPT_ENABLABLE,
    pub musicremix: OPT_ENABLABLE,
    pub speech: OPT_ENABLABLE,
    pub keepaspect: OPT_ENABLABLE,
    pub cheatMode: OPT_ENABLABLE,
    pub mainMenuMusic: OPT_ENABLABLE,
    pub nebulae: OPT_ENABLABLE,
    pub orbitingPlanets: OPT_ENABLABLE,
    pub texturedPlanets: OPT_ENABLABLE,
    pub godMode: OPT_ENABLABLE,
    pub bubbleWarp: OPT_ENABLABLE,
    pub unlockShips: OPT_ENABLABLE,
    pub headStart: OPT_ENABLABLE,
    pub autoButtons: OPT_ENABLABLE,
    pub infiniteRU: OPT_ENABLABLE,
    pub skipIntro: OPT_ENABLABLE,
    pub infiniteFuel: OPT_ENABLABLE,
    pub partialPickup: OPT_ENABLABLE,
    pub submenu: OPT_ENABLABLE,
    pub infiniteCredits: OPT_ENABLABLE,
    pub customBorder: OPT_ENABLABLE,
    pub volasMusic: OPT_ENABLABLE,
    pub wholeFuel: OPT_ENABLABLE,
    pub extended: OPT_ENABLABLE,
    pub gameOver: OPT_ENABLABLE,
    pub shipDirectionIP: OPT_ENABLABLE,
    pub hazardColors: OPT_ENABLABLE,
    pub orzCompFont: OPT_ENABLABLE,
    pub smartAutoPilot: OPT_ENABLABLE,
    pub nonStopOscill: OPT_ENABLABLE,
    pub hyperStars: OPT_ENABLABLE,
    pub planetTexture: OPT_ENABLABLE,
    pub noHQEncounters: OPT_ENABLABLE,
    pub deCleansing: OPT_ENABLABLE,
    pub meleeObstacles: OPT_ENABLABLE,
    pub showVisitedStars: OPT_ENABLABLE,
    pub unscaledStarSystem: OPT_ENABLABLE,
    pub slaughterMode: OPT_ENABLABLE,
    pub advancedAutoPilot: OPT_ENABLABLE,
    pub meleeToolTips: OPT_ENABLABLE,
    pub scatterElements: OPT_ENABLABLE,
    pub showUpgrades: OPT_ENABLABLE,
    pub fleetPointSys: OPT_ENABLABLE,
    pub shipSeed: OPT_ENABLABLE,
    pub shipStore: OPT_ENABLABLE,
    pub captainNames: OPT_ENABLABLE,
    pub dosMenus: OPT_ENABLABLE,
    pub menu: OPT_CONSOLETYPE,
    pub text: OPT_CONSOLETYPE,
    pub scroll: OPT_CONSOLETYPE,
    pub intro: OPT_CONSOLETYPE,
    pub shield: OPT_CONSOLETYPE,
    pub scrTrans: OPT_CONSOLETYPE,
    pub landerHold: OPT_CONSOLETYPE,
    pub tintPlanSphere: OPT_CONSOLETYPE,
    pub planetStyle: OPT_CONSOLETYPE,
    pub scanStyle: OPT_CONSOLETYPE,
    pub scopeStyle: OPT_CONSOLETYPE,
    pub landerStyle: OPT_CONSOLETYPE,
    pub flagshipColor: OPT_CONSOLETYPE,
    pub hyperSpaceColor: OPT_CONSOLETYPE,
    pub player1: CONTROL_TEMPLATE,
    pub player2: CONTROL_TEMPLATE,
    pub speechvol: ::core::ffi::c_int,
    pub musicvol: ::core::ffi::c_int,
    pub sfxvol: ::core::ffi::c_int,
    pub nebulaevol: ::core::ffi::c_int,
    pub cscan: ::core::ffi::c_int,
    pub gamma: ::core::ffi::c_int,
    pub starBackground: ::core::ffi::c_int,
    pub deadZoneLeftStick: [::core::ffi::c_int; 2],
    pub deadZoneRightStick: [::core::ffi::c_int; 2],
}
pub type GLOBALOPTS = globalopts_struct;
pub type MENU_SOUND_FLAGS = UWORD;
pub type SOUND = STRING;
pub type SOUND_REF = STRING_TABLE;
pub type stringbank = _stringbank_chunk;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _stringbank_chunk {
    pub data: [::core::ffi::c_char; 1016],
    pub len: ::core::ffi::c_int,
    pub next: *mut _stringbank_chunk,
}
pub type WIDGET_LABEL = _widget_label;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _widget_label {
    pub tag: WIDGET_TYPE,
    pub parent: *mut _widget,
    pub handleEvent: Option<
        unsafe extern "C" fn(*mut _widget, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub receiveFocus: Option<
        unsafe extern "C" fn(*mut _widget, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub draw: Option<
        unsafe extern "C" fn(*mut _widget, ::core::ffi::c_int, ::core::ffi::c_int) -> (),
    >,
    pub height: Option<unsafe extern "C" fn(*mut _widget) -> ::core::ffi::c_int>,
    pub width: Option<unsafe extern "C" fn(*mut _widget) -> ::core::ffi::c_int>,
    pub line_count: ::core::ffi::c_int,
    pub lines: *mut *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _widget {
    pub tag: WIDGET_TYPE,
    pub parent: *mut _widget,
    pub handleEvent: Option<
        unsafe extern "C" fn(*mut _widget, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub receiveFocus: Option<
        unsafe extern "C" fn(*mut _widget, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub draw: Option<
        unsafe extern "C" fn(*mut _widget, ::core::ffi::c_int, ::core::ffi::c_int) -> (),
    >,
    pub height: Option<unsafe extern "C" fn(*mut _widget) -> ::core::ffi::c_int>,
    pub width: Option<unsafe extern "C" fn(*mut _widget) -> ::core::ffi::c_int>,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct WIDGET_TYPE(pub ::core::ffi::c_uint);
impl WIDGET_TYPE {
    pub const WIDGET_TYPE_MENU_SCREEN: Self = Self(0);
    pub const WIDGET_TYPE_CHOICE: Self = Self(1);
    pub const WIDGET_TYPE_BUTTON: Self = Self(2);
    pub const WIDGET_TYPE_LABEL: Self = Self(3);
    pub const WIDGET_TYPE_SLIDER: Self = Self(4);
    pub const WIDGET_TYPE_TEXTENTRY: Self = Self(5);
    pub const WIDGET_TYPE_CONTROLENTRY: Self = Self(6);
    pub const WIDGET_TYPE_MENUCONTROLENTRY: Self = Self(7);
    pub const NUM_WIDGET_TYPES: Self = Self(8);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CHOICE_OPTION {
    pub optname: *const ::core::ffi::c_char,
    pub tooltip: [*const ::core::ffi::c_char; 3],
}
pub type WIDGET_CHOICE = _widget_choice;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _widget_choice {
    pub tag: WIDGET_TYPE,
    pub parent: *mut _widget,
    pub handleEvent: Option<
        unsafe extern "C" fn(*mut _widget, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub receiveFocus: Option<
        unsafe extern "C" fn(*mut _widget, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub draw: Option<
        unsafe extern "C" fn(*mut _widget, ::core::ffi::c_int, ::core::ffi::c_int) -> (),
    >,
    pub height: Option<unsafe extern "C" fn(*mut _widget) -> ::core::ffi::c_int>,
    pub width: Option<unsafe extern "C" fn(*mut _widget) -> ::core::ffi::c_int>,
    pub category: *const ::core::ffi::c_char,
    pub numopts: ::core::ffi::c_int,
    pub options: *mut CHOICE_OPTION,
    pub selected: ::core::ffi::c_int,
    pub highlighted: ::core::ffi::c_int,
    pub choice_num: ::core::ffi::c_int,
    pub onChange: Option<
        unsafe extern "C" fn(*mut _widget_choice, ::core::ffi::c_int) -> (),
    >,
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
pub type WIDGET_TEXTENTRY = _widget_textentry;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _widget_textentry {
    pub tag: WIDGET_TYPE,
    pub parent: *mut _widget,
    pub handleEvent: Option<
        unsafe extern "C" fn(*mut _widget, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub receiveFocus: Option<
        unsafe extern "C" fn(*mut _widget, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub draw: Option<
        unsafe extern "C" fn(*mut _widget, ::core::ffi::c_int, ::core::ffi::c_int) -> (),
    >,
    pub height: Option<unsafe extern "C" fn(*mut _widget) -> ::core::ffi::c_int>,
    pub width: Option<unsafe extern "C" fn(*mut _widget) -> ::core::ffi::c_int>,
    pub handleEventSelect: Option<
        unsafe extern "C" fn(*mut _widget_textentry) -> ::core::ffi::c_int,
    >,
    pub onChange: Option<unsafe extern "C" fn(*mut _widget_textentry) -> ()>,
    pub category: *const ::core::ffi::c_char,
    pub value: [::core::ffi::c_char; 31],
    pub maxlen: ::core::ffi::c_int,
    pub state: WIDGET_TEXTENTRY_STATE,
    pub cursor_pos: ::core::ffi::c_int,
    pub tooltip: [*const ::core::ffi::c_char; 3],
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct WIDGET_TEXTENTRY_STATE(pub ::core::ffi::c_uint);
impl WIDGET_TEXTENTRY_STATE {
    pub const WTE_NORMAL: Self = Self(0);
    pub const WTE_EDITING: Self = Self(1);
    pub const WTE_BLOCKCUR: Self = Self(2);
}
pub type WIDGET_SLIDER = _widget_slider;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _widget_slider {
    pub tag: WIDGET_TYPE,
    pub parent: *mut _widget,
    pub handleEvent: Option<
        unsafe extern "C" fn(*mut _widget, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub receiveFocus: Option<
        unsafe extern "C" fn(*mut _widget, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub draw: Option<
        unsafe extern "C" fn(*mut _widget, ::core::ffi::c_int, ::core::ffi::c_int) -> (),
    >,
    pub height: Option<unsafe extern "C" fn(*mut _widget) -> ::core::ffi::c_int>,
    pub width: Option<unsafe extern "C" fn(*mut _widget) -> ::core::ffi::c_int>,
    pub draw_value: Option<
        unsafe extern "C" fn(
            *mut _widget_slider,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub min: ::core::ffi::c_int,
    pub max: ::core::ffi::c_int,
    pub step: ::core::ffi::c_int,
    pub value: ::core::ffi::c_int,
    pub category: *const ::core::ffi::c_char,
    pub tooltip: [*const ::core::ffi::c_char; 3],
    pub onChange: Option<unsafe extern "C" fn(*mut _widget_slider) -> ()>,
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
pub type SETUP_MENU_STATE = setup_menu_state;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct setup_menu_state {
    pub InputFunc: Option<unsafe extern "C" fn(*mut setup_menu_state) -> BOOLEAN>,
    pub initialized: BOOLEAN,
    pub anim_frame_count: ::core::ffi::c_int,
    pub NextTime: DWORD,
}
pub type WIDGET_MENUCONTROLENTRY = _widget_menucontrolentry;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _widget_menucontrolentry {
    pub tag: WIDGET_TYPE,
    pub parent: *mut _widget,
    pub handleEvent: Option<
        unsafe extern "C" fn(*mut _widget, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub receiveFocus: Option<
        unsafe extern "C" fn(*mut _widget, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub draw: Option<
        unsafe extern "C" fn(*mut _widget, ::core::ffi::c_int, ::core::ffi::c_int) -> (),
    >,
    pub height: Option<unsafe extern "C" fn(*mut _widget) -> ::core::ffi::c_int>,
    pub width: Option<unsafe extern "C" fn(*mut _widget) -> ::core::ffi::c_int>,
    pub onChange: Option<unsafe extern "C" fn(*mut _widget_menucontrolentry) -> ()>,
    pub onDelete: Option<unsafe extern "C" fn(*mut _widget_menucontrolentry) -> ()>,
    pub category: *const ::core::ffi::c_char,
    pub controlindex: ::core::ffi::c_int,
    pub highlighted: ::core::ffi::c_int,
    pub controlname: [[::core::ffi::c_char; 16]; 6],
    pub current_page: ::core::ffi::c_int,
    pub num_pages: ::core::ffi::c_int,
}
pub type WIDGET = _widget;
pub type WIDGET_CONTROLENTRY = _widget_controlentry;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _widget_controlentry {
    pub tag: WIDGET_TYPE,
    pub parent: *mut _widget,
    pub handleEvent: Option<
        unsafe extern "C" fn(*mut _widget, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub receiveFocus: Option<
        unsafe extern "C" fn(*mut _widget, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub draw: Option<
        unsafe extern "C" fn(*mut _widget, ::core::ffi::c_int, ::core::ffi::c_int) -> (),
    >,
    pub height: Option<unsafe extern "C" fn(*mut _widget) -> ::core::ffi::c_int>,
    pub width: Option<unsafe extern "C" fn(*mut _widget) -> ::core::ffi::c_int>,
    pub onChange: Option<unsafe extern "C" fn(*mut _widget_controlentry) -> ()>,
    pub onDelete: Option<unsafe extern "C" fn(*mut _widget_controlentry) -> ()>,
    pub category: *const ::core::ffi::c_char,
    pub controlindex: ::core::ffi::c_int,
    pub highlighted: ::core::ffi::c_int,
    pub controlname: [[::core::ffi::c_char; 16]; 4],
    pub current_page: ::core::ffi::c_int,
    pub num_pages: ::core::ffi::c_int,
}
pub type WIDGET_BUTTON = _widget_button;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _widget_button {
    pub tag: WIDGET_TYPE,
    pub parent: *mut _widget,
    pub handleEvent: Option<
        unsafe extern "C" fn(*mut _widget, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub receiveFocus: Option<
        unsafe extern "C" fn(*mut _widget, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub draw: Option<
        unsafe extern "C" fn(*mut _widget, ::core::ffi::c_int, ::core::ffi::c_int) -> (),
    >,
    pub height: Option<unsafe extern "C" fn(*mut _widget) -> ::core::ffi::c_int>,
    pub width: Option<unsafe extern "C" fn(*mut _widget) -> ::core::ffi::c_int>,
    pub name: *const ::core::ffi::c_char,
    pub tooltip: [*const ::core::ffi::c_char; 3],
}
pub type HANDLER = Option<
    unsafe extern "C" fn(*mut WIDGET, ::core::ffi::c_int) -> ::core::ffi::c_int,
>;
pub type WIDGET_MENU_SCREEN = _widget_menu_screen;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _widget_menu_screen {
    pub tag: WIDGET_TYPE,
    pub parent: *mut _widget,
    pub handleEvent: Option<
        unsafe extern "C" fn(*mut _widget, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub receiveFocus: Option<
        unsafe extern "C" fn(*mut _widget, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub draw: Option<
        unsafe extern "C" fn(*mut _widget, ::core::ffi::c_int, ::core::ffi::c_int) -> (),
    >,
    pub height: Option<unsafe extern "C" fn(*mut _widget) -> ::core::ffi::c_int>,
    pub width: Option<unsafe extern "C" fn(*mut _widget) -> ::core::ffi::c_int>,
    pub title: *const ::core::ffi::c_char,
    pub subtitle: *const ::core::ffi::c_char,
    pub bgStamp: STAMP,
    pub num_children: ::core::ffi::c_int,
    pub child: *mut *mut _widget,
    pub highlighted: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option_list_value {
    pub str: *const ::core::ffi::c_char,
    pub value: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_13 {
    pub widgets: *mut *mut WIDGET,
    pub bgIndex: ::core::ffi::c_int,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_14(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_14 {
    pub const PORTAL_SPAWNER_DEVICE: Self = Self(0);
    pub const TALKING_PET_DEVICE: Self = Self(1);
    pub const UTWIG_BOMB_DEVICE: Self = Self(2);
    pub const SUN_EFFICIENCY_DEVICE: Self = Self(3);
    pub const ROSY_SPHERE_DEVICE: Self = Self(4);
    pub const AQUA_HELIX_DEVICE: Self = Self(5);
    pub const CLEAR_SPINDLE_DEVICE: Self = Self(6);
    pub const ULTRON_0_DEVICE: Self = Self(7);
    pub const ULTRON_1_DEVICE: Self = Self(8);
    pub const ULTRON_2_DEVICE: Self = Self(9);
    pub const ULTRON_3_DEVICE: Self = Self(10);
    pub const MAIDENS_DEVICE: Self = Self(11);
    pub const UMGAH_HYPERWAVE_DEVICE: Self = Self(12);
    pub const BURVIX_HYPERWAVE_DEVICE: Self = Self(13);
    pub const DATA_PLATE_1_DEVICE: Self = Self(14);
    pub const DATA_PLATE_2_DEVICE: Self = Self(15);
    pub const DATA_PLATE_3_DEVICE: Self = Self(16);
    pub const TAALO_PROTECTOR_DEVICE: Self = Self(17);
    pub const EGG_CASING0_DEVICE: Self = Self(18);
    pub const EGG_CASING1_DEVICE: Self = Self(19);
    pub const EGG_CASING2_DEVICE: Self = Self(20);
    pub const SYREEN_SHUTTLE_DEVICE: Self = Self(21);
    pub const VUX_BEAST_DEVICE: Self = Self(22);
    pub const DESTRUCT_CODE_DEVICE: Self = Self(23);
    pub const URQUAN_WARP_DEVICE: Self = Self(24);
    pub const WIMBLIS_TRIDENT_DEVICE: Self = Self(25);
    pub const GLOWING_ROD_DEVICE: Self = Self(26);
    pub const LUNAR_BASE_DEVICE: Self = Self(27);
    pub const NUM_DEVICES: Self = Self(28);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_15(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_15 {
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
pub struct C2Rust_Unnamed_16(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_16 {
    pub const TFB_GFXDRIVER_SDL_OPENGL: Self = Self(0);
    pub const TFB_GFXDRIVER_SDL_PURE: Self = Self(1);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_17(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_17 {
    pub const TFB_SCALE_STEP: Self = Self(0);
    pub const TFB_SCALE_NEAREST: Self = Self(1);
    pub const TFB_SCALE_BILINEAR: Self = Self(2);
    pub const TFB_SCALE_TRILINEAR: Self = Self(3);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_18(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_18 {
    pub const WIDGET_EVENT_UP: Self = Self(0);
    pub const WIDGET_EVENT_DOWN: Self = Self(1);
    pub const WIDGET_EVENT_LEFT: Self = Self(2);
    pub const WIDGET_EVENT_RIGHT: Self = Self(3);
    pub const WIDGET_EVENT_SELECT: Self = Self(4);
    pub const WIDGET_EVENT_CANCEL: Self = Self(5);
    pub const WIDGET_EVENT_DELETE: Self = Self(6);
    pub const NUM_WIDGET_EVENTS: Self = Self(7);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_19(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_19 {
    pub const audio_DRIVER_MIXSDL: Self = Self(0);
    pub const audio_DRIVER_NOSOUND: Self = Self(1);
    pub const audio_DRIVER_OPENAL: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_20(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_20 {
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
static mut SetupTab: STRING = ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>();
#[no_mangle]
pub static mut scalerList: [option_list_value; 6] = [
    option_list_value {
        str: b"no\0".as_ptr() as *const ::core::ffi::c_char,
        value: 0 as ::core::ffi::c_int,
    },
    option_list_value {
        str: b"bilinear\0".as_ptr() as *const ::core::ffi::c_char,
        value: (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int,
    },
    option_list_value {
        str: b"biadapt\0".as_ptr() as *const ::core::ffi::c_char,
        value: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int,
    },
    option_list_value {
        str: b"biadv\0".as_ptr() as *const ::core::ffi::c_char,
        value: (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int,
    },
    option_list_value {
        str: b"triscan\0".as_ptr() as *const ::core::ffi::c_char,
        value: (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int,
    },
    option_list_value {
        str: b"hq\0".as_ptr() as *const ::core::ffi::c_char,
        value: (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int,
    },
];
static mut saved_menu_bindings: [MENU_BINDINGS; 31] = [MENU_BINDINGS {
    action: [0; 40],
    binding: [VCONTROL_GESTURE {
        r#type: VCONTROL_GESTURE_TYPE::VCONTROL_NONE,
        gesture: C2Rust_Unnamed { key: 0 },
    }; 6],
}; 31];
static mut menu_bindings_dirty: BOOLEAN = BOOLEAN::FALSE;
static mut SfxVol: ::core::ffi::c_int = 0;
static mut MusVol: ::core::ffi::c_int = 0;
static mut SpcVol: ::core::ffi::c_int = 0;
static mut optMScale: ::core::ffi::c_int = 0;
static mut testSounds: SOUND = ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>();
#[no_mangle]
pub static mut InSetupMenu: BOOLEAN = BOOLEAN::FALSE;
unsafe extern "C" fn whichPlatformRef(mut opt: OPT_CONSOLETYPE) -> ::core::ffi::c_int {
    return if opt.0 != 0 {
        0x1 as ::core::ffi::c_int
    } else {
        0x2 as ::core::ffi::c_int
    };
}
unsafe extern "C" fn PutBoolOpt(
    mut glob: *mut OPT_ENABLABLE,
    mut set: *mut OPT_ENABLABLE,
    mut key: *const ::core::ffi::c_char,
    mut reload: BOOLEAN,
) -> BOOLEAN {
    if (*glob).0 != (*set).0 {
        *glob = *set;
        uqm_lua_checked!(res_PutBoolean(key, BOOLEAN((*set).0)), BOOLEAN(0));
        if reload.0 != 0 {
            optRequiresReload = BOOLEAN::TRUE;
        }
        return BOOLEAN::TRUE;
    }
    return BOOLEAN::FALSE;
}
unsafe extern "C" fn PutIntOpt(
    mut glob: *mut ::core::ffi::c_int,
    mut set: *mut ::core::ffi::c_int,
    mut key: *const ::core::ffi::c_char,
    mut reload: BOOLEAN,
) -> BOOLEAN {
    if *glob != *set {
        *glob = *set;
        uqm_lua_checked!(res_PutInteger(key, *set), BOOLEAN(0));
        if reload.0 != 0 {
            optRequiresReload = BOOLEAN::TRUE;
        }
        return BOOLEAN::TRUE;
    }
    return BOOLEAN::FALSE;
}
unsafe extern "C" fn PutConsOpt(
    mut glob: *mut ::core::ffi::c_int,
    mut set: *mut OPT_CONSOLETYPE,
    mut key: *const ::core::ffi::c_char,
    mut reload: BOOLEAN,
) -> BOOLEAN {
    if *glob != whichPlatformRef(*set) {
        *glob = whichPlatformRef(*set);
        uqm_lua_checked!(res_PutBoolean(key, BOOLEAN((*set).0)), BOOLEAN(0));
        if reload.0 != 0 {
            optRequiresReload = BOOLEAN::TRUE;
        }
        return BOOLEAN::TRUE;
    }
    return BOOLEAN::FALSE;
}
static mut done: BOOLEAN = BOOLEAN::FALSE;
static mut next: *mut WIDGET = ::core::ptr::null_mut::<WIDGET>();
static mut current: *mut WIDGET = ::core::ptr::null_mut::<WIDGET>();
static mut menus: [WIDGET_MENU_SCREEN; 15] = [WIDGET_MENU_SCREEN {
    tag: WIDGET_TYPE::WIDGET_TYPE_MENU_SCREEN,
    parent: ::core::ptr::null_mut::<_widget>(),
    handleEvent: None,
    receiveFocus: None,
    draw: None,
    height: None,
    width: None,
    title: ::core::ptr::null::<::core::ffi::c_char>(),
    subtitle: ::core::ptr::null::<::core::ffi::c_char>(),
    bgStamp: STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    },
    num_children: 0,
    child: ::core::ptr::null_mut::<*mut _widget>(),
    highlighted: 0,
}; 15];
static mut choices: [WIDGET_CHOICE; 131] = [WIDGET_CHOICE {
    tag: WIDGET_TYPE::WIDGET_TYPE_MENU_SCREEN,
    parent: ::core::ptr::null_mut::<_widget>(),
    handleEvent: None,
    receiveFocus: None,
    draw: None,
    height: None,
    width: None,
    category: ::core::ptr::null::<::core::ffi::c_char>(),
    numopts: 0,
    options: ::core::ptr::null_mut::<CHOICE_OPTION>(),
    selected: 0,
    highlighted: 0,
    choice_num: 0,
    onChange: None,
}; 131];
static mut sliders: [WIDGET_SLIDER; 9] = [WIDGET_SLIDER {
    tag: WIDGET_TYPE::WIDGET_TYPE_MENU_SCREEN,
    parent: ::core::ptr::null_mut::<_widget>(),
    handleEvent: None,
    receiveFocus: None,
    draw: None,
    height: None,
    width: None,
    draw_value: None,
    min: 0,
    max: 0,
    step: 0,
    value: 0,
    category: ::core::ptr::null::<::core::ffi::c_char>(),
    tooltip: [::core::ptr::null::<::core::ffi::c_char>(); 3],
    onChange: None,
}; 9];
static mut buttons: [WIDGET_BUTTON; 21] = [WIDGET_BUTTON {
    tag: WIDGET_TYPE::WIDGET_TYPE_MENU_SCREEN,
    parent: ::core::ptr::null_mut::<_widget>(),
    handleEvent: None,
    receiveFocus: None,
    draw: None,
    height: None,
    width: None,
    name: ::core::ptr::null::<::core::ffi::c_char>(),
    tooltip: [::core::ptr::null::<::core::ffi::c_char>(); 3],
}; 21];
static mut labels: [WIDGET_LABEL; 9] = [WIDGET_LABEL {
    tag: WIDGET_TYPE::WIDGET_TYPE_MENU_SCREEN,
    parent: ::core::ptr::null_mut::<_widget>(),
    handleEvent: None,
    receiveFocus: None,
    draw: None,
    height: None,
    width: None,
    line_count: 0,
    lines: ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
}; 9];
static mut textentries: [WIDGET_TEXTENTRY; 2] = [WIDGET_TEXTENTRY {
    tag: WIDGET_TYPE::WIDGET_TYPE_MENU_SCREEN,
    parent: ::core::ptr::null_mut::<_widget>(),
    handleEvent: None,
    receiveFocus: None,
    draw: None,
    height: None,
    width: None,
    handleEventSelect: None,
    onChange: None,
    category: ::core::ptr::null::<::core::ffi::c_char>(),
    value: [0; 31],
    maxlen: 0,
    state: WIDGET_TEXTENTRY_STATE::WTE_NORMAL,
    cursor_pos: 0,
    tooltip: [::core::ptr::null::<::core::ffi::c_char>(); 3],
}; 2];
static mut controlentries: [WIDGET_CONTROLENTRY; 8] = [WIDGET_CONTROLENTRY {
    tag: WIDGET_TYPE::WIDGET_TYPE_MENU_SCREEN,
    parent: ::core::ptr::null_mut::<_widget>(),
    handleEvent: None,
    receiveFocus: None,
    draw: None,
    height: None,
    width: None,
    onChange: None,
    onDelete: None,
    category: ::core::ptr::null::<::core::ffi::c_char>(),
    controlindex: 0,
    highlighted: 0,
    controlname: [[0; 16]; 4],
    current_page: 0,
    num_pages: 0,
}; 8];
static mut menucontrols: [WIDGET_MENUCONTROLENTRY; 30] = [WIDGET_MENUCONTROLENTRY {
    tag: WIDGET_TYPE::WIDGET_TYPE_MENU_SCREEN,
    parent: ::core::ptr::null_mut::<_widget>(),
    handleEvent: None,
    receiveFocus: None,
    draw: None,
    height: None,
    width: None,
    onChange: None,
    onDelete: None,
    category: ::core::ptr::null::<::core::ffi::c_char>(),
    controlindex: 0,
    highlighted: 0,
    controlname: [[0; 16]; 6],
    current_page: 0,
    num_pages: 0,
}; 30];
static mut button_handlers: [HANDLER; 21] = [
    Some(
        quit_main_menu
            as unsafe extern "C" fn(
                *mut WIDGET,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
    ),
    Some(
        quit_sub_menu
            as unsafe extern "C" fn(
                *mut WIDGET,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
    ),
    Some(
        do_graphics
            as unsafe extern "C" fn(
                *mut WIDGET,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
    ),
    Some(
        do_engine
            as unsafe extern "C" fn(
                *mut WIDGET,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
    ),
    Some(
        do_audio
            as unsafe extern "C" fn(
                *mut WIDGET,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
    ),
    Some(
        do_cheats
            as unsafe extern "C" fn(
                *mut WIDGET,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
    ),
    Some(
        do_keyconfig
            as unsafe extern "C" fn(
                *mut WIDGET,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
    ),
    Some(
        do_advanced
            as unsafe extern "C" fn(
                *mut WIDGET,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
    ),
    Some(
        do_editkeys
            as unsafe extern "C" fn(
                *mut WIDGET,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
    ),
    Some(
        do_keyconfig
            as unsafe extern "C" fn(
                *mut WIDGET,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
    ),
    Some(
        do_music
            as unsafe extern "C" fn(
                *mut WIDGET,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
    ),
    Some(
        do_visual
            as unsafe extern "C" fn(
                *mut WIDGET,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
    ),
    Some(
        do_qol
            as unsafe extern "C" fn(
                *mut WIDGET,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
    ),
    Some(
        do_devices
            as unsafe extern "C" fn(
                *mut WIDGET,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
    ),
    Some(
        do_upgrades
            as unsafe extern "C" fn(
                *mut WIDGET,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
    ),
    Some(
        do_cheats
            as unsafe extern "C" fn(
                *mut WIDGET,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
    ),
    Some(
        do_editmenukeys
            as unsafe extern "C" fn(
                *mut WIDGET,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
    ),
    Some(
        do_savemenubinds
            as unsafe extern "C" fn(
                *mut WIDGET,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
    ),
    Some(
        do_cancelmenubinds
            as unsafe extern "C" fn(
                *mut WIDGET,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
    ),
    Some(
        do_loaddefmenubinds
            as unsafe extern "C" fn(
                *mut WIDGET,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
    ),
    Some(
        do_deadzones
            as unsafe extern "C" fn(
                *mut WIDGET,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
    ),
];
static mut main_widgets: [*mut WIDGET; 12] = [::core::ptr::null_mut::<WIDGET>(); 12];
static mut graphics_widgets: [*mut WIDGET; 12] = [::core::ptr::null_mut::<WIDGET>(); 12];
static mut engine_widgets: [*mut WIDGET; 31] = [::core::ptr::null_mut::<WIDGET>(); 31];
static mut audio_widgets: [*mut WIDGET; 10] = [::core::ptr::null_mut::<WIDGET>(); 10];
static mut music_widgets: [*mut WIDGET; 10] = [::core::ptr::null_mut::<WIDGET>(); 10];
static mut cheat_widgets: [*mut WIDGET; 16] = [::core::ptr::null_mut::<WIDGET>(); 16];
static mut keyconfig_widgets: [*mut WIDGET; 12] = [::core::ptr::null_mut::<
    WIDGET,
>(); 12];
static mut advanced_widgets: [*mut WIDGET; 12] = [::core::ptr::null_mut::<WIDGET>(); 12];
static mut visual_widgets: [*mut WIDGET; 28] = [::core::ptr::null_mut::<WIDGET>(); 28];
static mut qol_widgets: [*mut WIDGET; 12] = [::core::ptr::null_mut::<WIDGET>(); 12];
static mut editkeys_widgets: [*mut WIDGET; 15] = [::core::ptr::null_mut::<WIDGET>(); 15];
static mut devices_widgets: [*mut WIDGET; 28] = [::core::ptr::null_mut::<WIDGET>(); 28];
static mut upgrades_widgets: [*mut WIDGET; 16] = [::core::ptr::null_mut::<WIDGET>(); 16];
static mut editmenukeys_widgets: [*mut WIDGET; 38] = [::core::ptr::null_mut::<
    WIDGET,
>(); 38];
static mut editdeadzone_widgets: [*mut WIDGET; 8] = [::core::ptr::null_mut::<
    WIDGET,
>(); 8];
static mut menu_defs: [C2Rust_Unnamed_13; 16] = unsafe {
    [
        C2Rust_Unnamed_13 {
            widgets: &raw const main_widgets as *mut *mut WIDGET,
            bgIndex: 0 as ::core::ffi::c_int,
        },
        C2Rust_Unnamed_13 {
            widgets: &raw const graphics_widgets as *mut *mut WIDGET,
            bgIndex: 1 as ::core::ffi::c_int,
        },
        C2Rust_Unnamed_13 {
            widgets: &raw const audio_widgets as *mut *mut WIDGET,
            bgIndex: 2 as ::core::ffi::c_int,
        },
        C2Rust_Unnamed_13 {
            widgets: &raw const engine_widgets as *mut *mut WIDGET,
            bgIndex: 3 as ::core::ffi::c_int,
        },
        C2Rust_Unnamed_13 {
            widgets: &raw const cheat_widgets as *mut *mut WIDGET,
            bgIndex: 4 as ::core::ffi::c_int,
        },
        C2Rust_Unnamed_13 {
            widgets: &raw const keyconfig_widgets as *mut *mut WIDGET,
            bgIndex: 5 as ::core::ffi::c_int,
        },
        C2Rust_Unnamed_13 {
            widgets: &raw const advanced_widgets as *mut *mut WIDGET,
            bgIndex: 6 as ::core::ffi::c_int,
        },
        C2Rust_Unnamed_13 {
            widgets: &raw const editkeys_widgets as *mut *mut WIDGET,
            bgIndex: 7 as ::core::ffi::c_int,
        },
        C2Rust_Unnamed_13 {
            widgets: &raw const music_widgets as *mut *mut WIDGET,
            bgIndex: 8 as ::core::ffi::c_int,
        },
        C2Rust_Unnamed_13 {
            widgets: &raw const visual_widgets as *mut *mut WIDGET,
            bgIndex: 9 as ::core::ffi::c_int,
        },
        C2Rust_Unnamed_13 {
            widgets: &raw const qol_widgets as *mut *mut WIDGET,
            bgIndex: 10 as ::core::ffi::c_int,
        },
        C2Rust_Unnamed_13 {
            widgets: &raw const devices_widgets as *mut *mut WIDGET,
            bgIndex: 11 as ::core::ffi::c_int,
        },
        C2Rust_Unnamed_13 {
            widgets: &raw const upgrades_widgets as *mut *mut WIDGET,
            bgIndex: 12 as ::core::ffi::c_int,
        },
        C2Rust_Unnamed_13 {
            widgets: &raw const editmenukeys_widgets as *mut *mut WIDGET,
            bgIndex: 13 as ::core::ffi::c_int,
        },
        C2Rust_Unnamed_13 {
            widgets: &raw const editdeadzone_widgets as *mut *mut WIDGET,
            bgIndex: 14 as ::core::ffi::c_int,
        },
        C2Rust_Unnamed_13 {
            widgets: ::core::ptr::null_mut::<*mut WIDGET>(),
            bgIndex: 0 as ::core::ffi::c_int,
        },
    ]
};
static mut minGamma: ::core::ffi::c_float = 0.4f32;
static mut maxGamma: ::core::ffi::c_float = 2.5f32;
static mut minGammaX: ::core::ffi::c_float = 0.;
static mut maxGammaX: ::core::ffi::c_float = 0.;
unsafe extern "C" fn quit_main_menu(
    mut self_0: *mut WIDGET,
    mut event: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if event == C2Rust_Unnamed_18::WIDGET_EVENT_SELECT.0 as ::core::ffi::c_int {
        next = ::core::ptr::null_mut::<WIDGET>();
        return BOOLEAN::TRUE.0 as ::core::ffi::c_int;
    }
    return BOOLEAN::FALSE.0 as ::core::ffi::c_int;
}
unsafe extern "C" fn quit_sub_menu(
    mut self_0: *mut WIDGET,
    mut event: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if event == C2Rust_Unnamed_18::WIDGET_EVENT_SELECT.0 as ::core::ffi::c_int {
        ResetOffset();
        next = (&raw mut menus as *mut WIDGET_MENU_SCREEN)
            .offset(C2Rust_Unnamed_6::MENU_QUITSUB.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET;
        uqm_lua_checked!(Some((*next).receiveFocus.expect("non-null function pointer"))
            .expect(
                "non-null function pointer",
            )(next, C2Rust_Unnamed_18::WIDGET_EVENT_SELECT.0 as ::core::ffi::c_int), 0);
        return BOOLEAN::TRUE.0 as ::core::ffi::c_int;
    }
    return BOOLEAN::FALSE.0 as ::core::ffi::c_int;
}
unsafe extern "C" fn populate_res() {
    sprintf(
        &raw mut (*(&raw mut textentries as *mut WIDGET_TEXTENTRY)
            .offset(C2Rust_Unnamed_10::TEXT_CUSTMRES.0 as ::core::ffi::c_int as isize))
            .value as *mut ::core::ffi::c_char,
        b"%dx%d\0".as_ptr() as *const ::core::ffi::c_char,
        SavedWidth,
        SavedHeight,
    );
}
unsafe extern "C" fn do_graphics(
    mut self_0: *mut WIDGET,
    mut event: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if event == C2Rust_Unnamed_18::WIDGET_EVENT_SELECT.0 as ::core::ffi::c_int {
        next = (&raw mut menus as *mut WIDGET_MENU_SCREEN)
            .offset(C2Rust_Unnamed_6::MENU_GRAPHICS.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET;
        populate_res();
        uqm_lua_checked!(Some((*next).receiveFocus.expect("non-null function pointer"))
            .expect(
                "non-null function pointer",
            )(next, C2Rust_Unnamed_18::WIDGET_EVENT_DOWN.0 as ::core::ffi::c_int), 0);
        return BOOLEAN::TRUE.0 as ::core::ffi::c_int;
    }
    return BOOLEAN::FALSE.0 as ::core::ffi::c_int;
}
unsafe extern "C" fn do_audio(
    mut self_0: *mut WIDGET,
    mut event: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if event == C2Rust_Unnamed_18::WIDGET_EVENT_SELECT.0 as ::core::ffi::c_int {
        next = (&raw mut menus as *mut WIDGET_MENU_SCREEN)
            .offset(C2Rust_Unnamed_6::MENU_AUDIO.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET;
        uqm_lua_checked!(Some((*next).receiveFocus.expect("non-null function pointer"))
            .expect(
                "non-null function pointer",
            )(next, C2Rust_Unnamed_18::WIDGET_EVENT_DOWN.0 as ::core::ffi::c_int), 0);
        return BOOLEAN::TRUE.0 as ::core::ffi::c_int;
    }
    return BOOLEAN::FALSE.0 as ::core::ffi::c_int;
}
unsafe extern "C" fn do_engine(
    mut self_0: *mut WIDGET,
    mut event: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if event == C2Rust_Unnamed_18::WIDGET_EVENT_SELECT.0 as ::core::ffi::c_int {
        next = (&raw mut menus as *mut WIDGET_MENU_SCREEN)
            .offset(C2Rust_Unnamed_6::MENU_ENGINE.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET;
        uqm_lua_checked!(Some((*next).receiveFocus.expect("non-null function pointer"))
            .expect(
                "non-null function pointer",
            )(next, C2Rust_Unnamed_18::WIDGET_EVENT_DOWN.0 as ::core::ffi::c_int), 0);
        return BOOLEAN::TRUE.0 as ::core::ffi::c_int;
    }
    return BOOLEAN::FALSE.0 as ::core::ffi::c_int;
}
unsafe extern "C" fn do_cheats(
    mut self_0: *mut WIDGET,
    mut event: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if event == C2Rust_Unnamed_18::WIDGET_EVENT_SELECT.0 as ::core::ffi::c_int {
        next = (&raw mut menus as *mut WIDGET_MENU_SCREEN)
            .offset(C2Rust_Unnamed_6::MENU_CHEATS.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET;
        uqm_lua_checked!(Some((*next).receiveFocus.expect("non-null function pointer"))
            .expect(
                "non-null function pointer",
            )(next, C2Rust_Unnamed_18::WIDGET_EVENT_DOWN.0 as ::core::ffi::c_int), 0);
        return BOOLEAN::TRUE.0 as ::core::ffi::c_int;
    }
    return BOOLEAN::FALSE.0 as ::core::ffi::c_int;
}
unsafe extern "C" fn do_keyconfig(
    mut self_0: *mut WIDGET,
    mut event: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if event == C2Rust_Unnamed_18::WIDGET_EVENT_SELECT.0 as ::core::ffi::c_int {
        next = (&raw mut menus as *mut WIDGET_MENU_SCREEN)
            .offset(C2Rust_Unnamed_6::MENY_KEYCONF.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET;
        uqm_lua_checked!(Some((*next).receiveFocus.expect("non-null function pointer"))
            .expect(
                "non-null function pointer",
            )(next, C2Rust_Unnamed_18::WIDGET_EVENT_DOWN.0 as ::core::ffi::c_int), 0);
        return BOOLEAN::TRUE.0 as ::core::ffi::c_int;
    }
    return BOOLEAN::FALSE.0 as ::core::ffi::c_int;
}
unsafe extern "C" fn populate_seed() {
    if choices[C2Rust_Unnamed_5::CHOICE_GAMESEED.0 as ::core::ffi::c_int as usize]
        .selected == OPT_SEED::OPTVAL_PRIME.0 as ::core::ffi::c_int
        || if optCustomSeed < 3 as ::core::ffi::c_int
            || optCustomSeed > 2147483644 as ::core::ffi::c_int
        {
            0 as ::core::ffi::c_int
        } else {
            1 as ::core::ffi::c_int
        } == 0
    {
        optCustomSeed = 16807 as ::core::ffi::c_int;
    }
    snprintf(
        &raw mut (*(&raw mut textentries as *mut WIDGET_TEXTENTRY)
            .offset(C2Rust_Unnamed_10::TEXT_GAMESEED.0 as ::core::ffi::c_int as isize))
            .value as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 31]>(),
        b"%d\0".as_ptr() as *const ::core::ffi::c_char,
        optCustomSeed,
    );
}
unsafe extern "C" fn do_advanced(
    mut self_0: *mut WIDGET,
    mut event: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if event == C2Rust_Unnamed_18::WIDGET_EVENT_SELECT.0 as ::core::ffi::c_int {
        next = (&raw mut menus as *mut WIDGET_MENU_SCREEN)
            .offset(C2Rust_Unnamed_6::MENU_ADVANCED.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET;
        populate_seed();
        uqm_lua_checked!(Some((*next).receiveFocus.expect("non-null function pointer"))
            .expect(
                "non-null function pointer",
            )(next, C2Rust_Unnamed_18::WIDGET_EVENT_DOWN.0 as ::core::ffi::c_int), 0);
        return BOOLEAN::TRUE.0 as ::core::ffi::c_int;
    }
    return BOOLEAN::FALSE.0 as ::core::ffi::c_int;
}
unsafe extern "C" fn do_music(
    mut self_0: *mut WIDGET,
    mut event: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if event == C2Rust_Unnamed_18::WIDGET_EVENT_SELECT.0 as ::core::ffi::c_int {
        next = (&raw mut menus as *mut WIDGET_MENU_SCREEN)
            .offset(C2Rust_Unnamed_6::MENU_MUSIC.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET;
        uqm_lua_checked!(Some((*next).receiveFocus.expect("non-null function pointer"))
            .expect(
                "non-null function pointer",
            )(next, C2Rust_Unnamed_18::WIDGET_EVENT_DOWN.0 as ::core::ffi::c_int), 0);
        return BOOLEAN::TRUE.0 as ::core::ffi::c_int;
    }
    return BOOLEAN::FALSE.0 as ::core::ffi::c_int;
}
unsafe extern "C" fn do_visual(
    mut self_0: *mut WIDGET,
    mut event: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if event == C2Rust_Unnamed_18::WIDGET_EVENT_SELECT.0 as ::core::ffi::c_int {
        next = (&raw mut menus as *mut WIDGET_MENU_SCREEN)
            .offset(C2Rust_Unnamed_6::MENU_VISUAL.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET;
        uqm_lua_checked!(Some((*next).receiveFocus.expect("non-null function pointer"))
            .expect(
                "non-null function pointer",
            )(next, C2Rust_Unnamed_18::WIDGET_EVENT_DOWN.0 as ::core::ffi::c_int), 0);
        return BOOLEAN::TRUE.0 as ::core::ffi::c_int;
    }
    return BOOLEAN::FALSE.0 as ::core::ffi::c_int;
}
unsafe extern "C" fn do_qol(
    mut self_0: *mut WIDGET,
    mut event: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if event == C2Rust_Unnamed_18::WIDGET_EVENT_SELECT.0 as ::core::ffi::c_int {
        next = (&raw mut menus as *mut WIDGET_MENU_SCREEN)
            .offset(C2Rust_Unnamed_6::MENU_QOL.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET;
        uqm_lua_checked!(Some((*next).receiveFocus.expect("non-null function pointer"))
            .expect(
                "non-null function pointer",
            )(next, C2Rust_Unnamed_18::WIDGET_EVENT_DOWN.0 as ::core::ffi::c_int), 0);
        return BOOLEAN::TRUE.0 as ::core::ffi::c_int;
    }
    return BOOLEAN::FALSE.0 as ::core::ffi::c_int;
}
unsafe extern "C" fn do_devices(
    mut self_0: *mut WIDGET,
    mut event: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if event == C2Rust_Unnamed_18::WIDGET_EVENT_SELECT.0 as ::core::ffi::c_int {
        next = (&raw mut menus as *mut WIDGET_MENU_SCREEN)
            .offset(C2Rust_Unnamed_6::MENU_DEVICES.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET;
        uqm_lua_checked!(Some((*next).receiveFocus.expect("non-null function pointer"))
            .expect(
                "non-null function pointer",
            )(next, C2Rust_Unnamed_18::WIDGET_EVENT_DOWN.0 as ::core::ffi::c_int), 0);
        return BOOLEAN::TRUE.0 as ::core::ffi::c_int;
    }
    return BOOLEAN::FALSE.0 as ::core::ffi::c_int;
}
unsafe extern "C" fn do_upgrades(
    mut self_0: *mut WIDGET,
    mut event: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if event == C2Rust_Unnamed_18::WIDGET_EVENT_SELECT.0 as ::core::ffi::c_int {
        next = (&raw mut menus as *mut WIDGET_MENU_SCREEN)
            .offset(C2Rust_Unnamed_6::MENU_UPGRADES.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET;
        uqm_lua_checked!(Some((*next).receiveFocus.expect("non-null function pointer"))
            .expect(
                "non-null function pointer",
            )(next, C2Rust_Unnamed_18::WIDGET_EVENT_DOWN.0 as ::core::ffi::c_int), 0);
        return BOOLEAN::TRUE.0 as ::core::ffi::c_int;
    }
    return BOOLEAN::FALSE.0 as ::core::ffi::c_int;
}
unsafe extern "C" fn populate_editkeys(mut templat: ::core::ffi::c_int) {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < C2Rust_Unnamed_3::NUM_KEYS.0 as ::core::ffi::c_int {
        j = 0 as ::core::ffi::c_int;
        while j < 4 as ::core::ffi::c_int {
            InterrogateInputState(
                templat,
                i,
                j,
                &raw mut *(&raw mut (*(&raw mut controlentries
                    as *mut WIDGET_CONTROLENTRY)
                    .offset(i as isize))
                    .controlname as *mut [::core::ffi::c_char; 16])
                    .offset(j as isize) as *mut ::core::ffi::c_char,
                16 as ::core::ffi::c_int,
                ::core::ptr::null_mut::<VCONTROL_GESTURE>(),
            );
            j += 1;
        }
        i += 1;
    }
}
unsafe extern "C" fn do_editkeys(
    mut self_0: *mut WIDGET,
    mut event: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if event == C2Rust_Unnamed_18::WIDGET_EVENT_SELECT.0 as ::core::ffi::c_int {
        next = (&raw mut menus as *mut WIDGET_MENU_SCREEN)
            .offset(C2Rust_Unnamed_6::MENU_EDITKEYS.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET;
        choices[C2Rust_Unnamed_5::CHOICE_KBLAYOUT.0 as ::core::ffi::c_int as usize]
            .selected = 0 as ::core::ffi::c_int;
        populate_editkeys(0 as ::core::ffi::c_int);
        uqm_lua_checked!(Some((*next).receiveFocus.expect("non-null function pointer"))
            .expect(
                "non-null function pointer",
            )(next, C2Rust_Unnamed_18::WIDGET_EVENT_DOWN.0 as ::core::ffi::c_int), 0);
        return BOOLEAN::TRUE.0 as ::core::ffi::c_int;
    }
    return BOOLEAN::FALSE.0 as ::core::ffi::c_int;
}
unsafe extern "C" fn do_deadzones(
    mut self_0: *mut WIDGET,
    mut event: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if event == C2Rust_Unnamed_18::WIDGET_EVENT_SELECT.0 as ::core::ffi::c_int {
        next = (&raw mut menus as *mut WIDGET_MENU_SCREEN)
            .offset(C2Rust_Unnamed_6::MENU_DEADZONES.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET;
        uqm_lua_checked!(Some((*next).receiveFocus.expect("non-null function pointer"))
            .expect(
                "non-null function pointer",
            )(next, C2Rust_Unnamed_18::WIDGET_EVENT_DOWN.0 as ::core::ffi::c_int), 0);
        return BOOLEAN::TRUE.0 as ::core::ffi::c_int;
    }
    return BOOLEAN::FALSE.0 as ::core::ffi::c_int;
}
unsafe extern "C" fn change_template(
    mut self_0: *mut WIDGET_CHOICE,
    mut oldval: ::core::ffi::c_int,
) {
    populate_editkeys((*self_0).selected);
}
unsafe extern "C" fn SaveMenuBindings() {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut f: *mut uio_Stream = ::core::ptr::null_mut::<uio_Stream>();
    let mut g: *mut VCONTROL_GESTURE = ::core::ptr::null_mut::<VCONTROL_GESTURE>();
    let mut g_str: [::core::ffi::c_char; 128] = [0; 128];
    f = uqm_lua_checked!(uio_fopen(
        configDir,
        b"override.cfg\0".as_ptr() as *const ::core::ffi::c_char,
        b"w\0".as_ptr() as *const ::core::ffi::c_char,
    ), ());
    if f.is_null() {
        log_add(
            log_Level::log_Error,
            b"Failed to open override.cfg for writing\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    uqm_lua_checked!(uio_fprintf(
        f,
        b"# Custom menu control bindings generated by Setup Menu\n\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    ), ());
    i = 0 as ::core::ffi::c_int;
    while i < C2Rust_Unnamed_12::MENUCONTROL_COUNT.0 as ::core::ffi::c_int {
        if !(*(&raw mut menu_res_names as *mut *const ::core::ffi::c_char)
            .offset(i as isize))
            .is_null()
        {
            j = 0 as ::core::ffi::c_int;
            while j < 6 as ::core::ffi::c_int {
                g = (&raw mut (*(&raw mut curr_bindings as *mut MENU_BINDINGS)
                    .offset(i as isize))
                    .binding as *mut VCONTROL_GESTURE)
                    .offset(j as isize);
                if (*g).r#type.0 != VCONTROL_GESTURE_TYPE::VCONTROL_NONE.0 {
                    VControl_DumpGesture(
                        &raw mut g_str as *mut ::core::ffi::c_char,
                        ::core::mem::size_of::<[::core::ffi::c_char; 128]>()
                            as ::core::ffi::c_int,
                        g,
                    );
                    uqm_lua_checked!(uio_fprintf(
                        f,
                        b"%s.%d = STRING:%s\n\0".as_ptr() as *const ::core::ffi::c_char,
                        *(&raw mut menu_res_names as *mut *const ::core::ffi::c_char)
                            .offset(i as isize),
                        j + 1 as ::core::ffi::c_int,
                        &raw mut g_str as *mut ::core::ffi::c_char,
                    ), ());
                } else {
                    uqm_lua_checked!(uio_fprintf(
                        f,
                        b"%s.%d =\n\0".as_ptr() as *const ::core::ffi::c_char,
                        *(&raw mut menu_res_names as *mut *const ::core::ffi::c_char)
                            .offset(i as isize),
                        j + 1 as ::core::ffi::c_int,
                    ), ());
                }
                j += 1;
            }
        }
        i += 1;
    }
    uqm_lua_checked!(uio_fclose(f), ());
    menu_bindings_dirty = BOOLEAN::FALSE;
}
unsafe extern "C" fn LoadDefaultMenuBindings() {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut g: *mut VCONTROL_GESTURE = ::core::ptr::null_mut::<VCONTROL_GESTURE>();
    i = 0 as ::core::ffi::c_int;
    while i < C2Rust_Unnamed_12::MENUCONTROL_COUNT.0 as ::core::ffi::c_int {
        if !(*(&raw mut menu_res_names as *mut *const ::core::ffi::c_char)
            .offset(i as isize))
            .is_null()
        {
            j = 0 as ::core::ffi::c_int;
            while j < 6 as ::core::ffi::c_int {
                g = (&raw mut (*(&raw mut curr_bindings as *mut MENU_BINDINGS)
                    .offset(i as isize))
                    .binding as *mut VCONTROL_GESTURE)
                    .offset(j as isize);
                VControl_RemoveGestureBinding(
                    g,
                    menu_vec.offset(i as isize) as *mut ::core::ffi::c_int,
                );
                j += 1;
            }
            memcpy(
                (&raw mut curr_bindings as *mut MENU_BINDINGS).offset(i as isize)
                    as *mut ::core::ffi::c_void,
                (&raw mut def_bindings as *mut MENU_BINDINGS).offset(i as isize)
                    as *const ::core::ffi::c_void,
                ::core::mem::size_of::<MENU_BINDINGS>(),
            );
            j = 0 as ::core::ffi::c_int;
            while j < 6 as ::core::ffi::c_int {
                g = (&raw mut (*(&raw mut curr_bindings as *mut MENU_BINDINGS)
                    .offset(i as isize))
                    .binding as *mut VCONTROL_GESTURE)
                    .offset(j as isize);
                VControl_AddGestureBinding(
                    g,
                    menu_vec.offset(i as isize) as *mut ::core::ffi::c_int,
                );
                j += 1;
            }
        }
        i += 1;
    }
    menu_bindings_dirty = BOOLEAN::TRUE;
}
unsafe extern "C" fn RestoreMenuBindings() {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut g: *mut VCONTROL_GESTURE = ::core::ptr::null_mut::<VCONTROL_GESTURE>();
    i = 0 as ::core::ffi::c_int;
    while i < C2Rust_Unnamed_12::MENUCONTROL_COUNT.0 as ::core::ffi::c_int {
        if !(*(&raw mut menu_res_names as *mut *const ::core::ffi::c_char)
            .offset(i as isize))
            .is_null()
        {
            j = 0 as ::core::ffi::c_int;
            while j < 6 as ::core::ffi::c_int {
                g = (&raw mut (*(&raw mut curr_bindings as *mut MENU_BINDINGS)
                    .offset(i as isize))
                    .binding as *mut VCONTROL_GESTURE)
                    .offset(j as isize);
                VControl_RemoveGestureBinding(
                    g,
                    menu_vec.offset(i as isize) as *mut ::core::ffi::c_int,
                );
                j += 1;
            }
            memcpy(
                (&raw mut curr_bindings as *mut MENU_BINDINGS).offset(i as isize)
                    as *mut ::core::ffi::c_void,
                (&raw mut saved_menu_bindings as *mut MENU_BINDINGS).offset(i as isize)
                    as *const ::core::ffi::c_void,
                ::core::mem::size_of::<MENU_BINDINGS>(),
            );
            j = 0 as ::core::ffi::c_int;
            while j < 6 as ::core::ffi::c_int {
                g = (&raw mut (*(&raw mut curr_bindings as *mut MENU_BINDINGS)
                    .offset(i as isize))
                    .binding as *mut VCONTROL_GESTURE)
                    .offset(j as isize);
                VControl_AddGestureBinding(
                    g,
                    menu_vec.offset(i as isize) as *mut ::core::ffi::c_int,
                );
                j += 1;
            }
        }
        i += 1;
    }
    menu_bindings_dirty = BOOLEAN::FALSE;
}
unsafe extern "C" fn populate_menukeys() {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut g: *mut VCONTROL_GESTURE = ::core::ptr::null_mut::<VCONTROL_GESTURE>();
    let mut size: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < C2Rust_Unnamed_12::MENUCONTROL_COUNT.0 as ::core::ffi::c_int {
        j = 0 as ::core::ffi::c_int;
        while j < 6 as ::core::ffi::c_int {
            if curr_bindings[i as usize].binding[j as usize].r#type.0
                != VCONTROL_GESTURE_TYPE::VCONTROL_NONE.0
            {
                let mut name: *mut ::core::ffi::c_char = &raw mut *(&raw mut (*(&raw mut menucontrols
                    as *mut WIDGET_MENUCONTROLENTRY)
                    .offset(i as isize))
                    .controlname as *mut [::core::ffi::c_char; 16])
                    .offset(j as isize) as *mut ::core::ffi::c_char;
                g = (&raw mut (*(&raw mut curr_bindings as *mut MENU_BINDINGS)
                    .offset(i as isize))
                    .binding as *mut VCONTROL_GESTURE)
                    .offset(j as isize);
                InterrogateInputState(
                    -1 as ::core::ffi::c_int,
                    -1 as ::core::ffi::c_int,
                    -1 as ::core::ffi::c_int,
                    name,
                    size,
                    g,
                );
            } else {
                menucontrols[i as usize].controlname[j as usize][0usize] = '\0'
                    as ::core::ffi::c_char;
            }
            j += 1;
        }
        i += 1;
    }
}
unsafe extern "C" fn do_editmenukeys(
    mut self_0: *mut WIDGET,
    mut event: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if event == C2Rust_Unnamed_18::WIDGET_EVENT_SELECT.0 as ::core::ffi::c_int {
        next = (&raw mut menus as *mut WIDGET_MENU_SCREEN)
            .offset(C2Rust_Unnamed_6::MENU_EDITMENUKEYS.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET;
        populate_menukeys();
        uqm_lua_checked!(Some((*next).receiveFocus.expect("non-null function pointer"))
            .expect(
                "non-null function pointer",
            )(next, C2Rust_Unnamed_18::WIDGET_EVENT_DOWN.0 as ::core::ffi::c_int), 0);
        return BOOLEAN::TRUE.0 as ::core::ffi::c_int;
    }
    return BOOLEAN::FALSE.0 as ::core::ffi::c_int;
}
unsafe extern "C" fn do_savemenubinds(
    mut self_0: *mut WIDGET,
    mut event: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if event == C2Rust_Unnamed_18::WIDGET_EVENT_SELECT.0 as ::core::ffi::c_int {
        uqm_lua_checked!(SaveMenuBindings(), 0);
        return BOOLEAN::TRUE.0 as ::core::ffi::c_int;
    }
    return BOOLEAN::FALSE.0 as ::core::ffi::c_int;
}
unsafe extern "C" fn do_cancelmenubinds(
    mut self_0: *mut WIDGET,
    mut event: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if event == C2Rust_Unnamed_18::WIDGET_EVENT_SELECT.0 as ::core::ffi::c_int {
        RestoreMenuBindings();
        populate_menukeys();
        return BOOLEAN::TRUE.0 as ::core::ffi::c_int;
    }
    return BOOLEAN::FALSE.0 as ::core::ffi::c_int;
}
unsafe extern "C" fn do_loaddefmenubinds(
    mut self_0: *mut WIDGET,
    mut event: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if event == C2Rust_Unnamed_18::WIDGET_EVENT_SELECT.0 as ::core::ffi::c_int {
        LoadDefaultMenuBindings();
        populate_menukeys();
        return BOOLEAN::TRUE.0 as ::core::ffi::c_int;
    }
    return BOOLEAN::FALSE.0 as ::core::ffi::c_int;
}
unsafe extern "C" fn addon_unavailable(
    mut self_0: *mut WIDGET_CHOICE,
    mut oldval: ::core::ffi::c_int,
) {
    (*self_0).selected = oldval;
    uqm_lua_checked!(DoPopupWindow(
        GetStringAddress(
            SetAbsStringTableIndex(
                GameStrings,
                (C2Rust_Unnamed_20::MAINMENU_STRING_BASE.0 as ::core::ffi::c_int
                    + 36 as ::core::ffi::c_int) as COUNT,
            ),
        ),
    ), ());
    Widget_SetFont(PlyrFont);
}
unsafe extern "C" fn check_for_hd(
    mut self_0: *mut WIDGET_CHOICE,
    mut oldval: ::core::ffi::c_int,
) {
    if (*self_0).selected != OPT_RESTYPE::OPTVAL_REAL_1280_960.0 as ::core::ffi::c_int {
        return;
    }
    if isAddonAvailable(b"mm-hd\0".as_ptr() as *const ::core::ffi::c_char).0 == 0 {
        oldval = OPT_RESTYPE::OPTVAL_320_240.0 as ::core::ffi::c_int;
        uqm_lua_checked!(addon_unavailable(self_0, OPT_RESTYPE::OPTVAL_320_240.0 as ::core::ffi::c_int), ());
    }
}
unsafe extern "C" fn check_dos_3do_modes(
    mut self_0: *mut WIDGET_CHOICE,
    mut oldval: ::core::ffi::c_int,
) -> BOOLEAN {
    let mut selected: bool = choices[C2Rust_Unnamed_5::CHOICE_GRAPHICS.0
            as ::core::ffi::c_int as usize]
        .selected != 0;
    let mut shmagoigle: ::core::ffi::c_int = if selected as ::core::ffi::c_int != 0
        && if resolutionFactor != 2 as ::core::ffi::c_uint {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } == 0
    {
        2 as ::core::ffi::c_int
    } else if resolutionFactor != 2 as ::core::ffi::c_uint {
        BOOLEAN::FALSE.0 as ::core::ffi::c_int
    } else {
        BOOLEAN::TRUE.0 as ::core::ffi::c_int
    };
    match (*self_0).selected {
        0 => {
            if isAddonAvailable(
                    if shmagoigle != 0 {
                        b"dos-mode-hd\0".as_ptr() as *const ::core::ffi::c_char
                    } else {
                        b"dos-mode-sd\0".as_ptr() as *const ::core::ffi::c_char
                    },
                )
                .0 == 0
            {
                oldval = OPT_WINDOWTYPE::OPTVAL_UQM_WINDOW.0 as ::core::ffi::c_int;
                uqm_lua_checked!(addon_unavailable(self_0, oldval), BOOLEAN(0));
                return BOOLEAN::FALSE;
            }
        }
        1 => {
            if isAddonAvailable(
                    if shmagoigle != 0 {
                        b"3do-mode-hd\0".as_ptr() as *const ::core::ffi::c_char
                    } else {
                        b"3do-mode-sd\0".as_ptr() as *const ::core::ffi::c_char
                    },
                )
                .0 == 0
            {
                oldval = OPT_WINDOWTYPE::OPTVAL_UQM_WINDOW.0 as ::core::ffi::c_int;
                uqm_lua_checked!(addon_unavailable(
                    self_0,
                    OPT_WINDOWTYPE::OPTVAL_UQM_WINDOW.0 as ::core::ffi::c_int,
                ), BOOLEAN(0));
                return BOOLEAN::FALSE;
            }
        }
        _ => {}
    }
    return BOOLEAN::TRUE;
}
unsafe extern "C" fn check_remixes(
    mut self_0: *mut WIDGET_CHOICE,
    mut oldval: ::core::ffi::c_int,
) -> BOOLEAN {
    let mut addon_available: BOOLEAN = BOOLEAN::FALSE;
    match (*self_0).choice_num {
        9 => {
            addon_available = isAddonAvailable(
                b"3domusic\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        21 => {
            addon_available = isAddonAvailable(
                b"remix\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        47 => {
            addon_available = isAddonAvailable(
                b"volasaurus-remix-pack\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        46 => {
            addon_available = BOOLEAN(
                (isAddonAvailable(
                        b"volasaurus-remix-pack\0".as_ptr() as *const ::core::ffi::c_char,
                    )
                    .0 != 0
                    || isAddonAvailable(
                            b"SpaceMusic\0".as_ptr() as *const ::core::ffi::c_char,
                        )
                        .0 != 0) as ::core::ffi::c_int as ::core::ffi::c_uint,
            );
        }
        _ => {
            log_add(
                log_Level::log_Error,
                b"invalid choice_num in check_remixes()\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    }
    if addon_available.0 == 0 {
        oldval = OPT_ENABLABLE::OPTVAL_DISABLED.0 as ::core::ffi::c_int;
        choices[(*self_0).choice_num as usize].selected = oldval;
        uqm_lua_checked!(addon_unavailable(self_0, oldval), BOOLEAN(0));
        return BOOLEAN::FALSE;
    }
    return BOOLEAN::TRUE;
}
unsafe extern "C" fn check_availability(
    mut self_0: *mut WIDGET_CHOICE,
    mut oldval: ::core::ffi::c_int,
) {
    if (*self_0).choice_num == C2Rust_Unnamed_5::CHOICE_GRAPHICS.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(check_for_hd(self_0, oldval), ());
    }
    if (*self_0).choice_num == C2Rust_Unnamed_5::CHOICE_REMIXES1.0 as ::core::ffi::c_int
        || (*self_0).choice_num
            == C2Rust_Unnamed_5::CHOICE_REMIXES2.0 as ::core::ffi::c_int
        || (*self_0).choice_num
            == C2Rust_Unnamed_5::CHOICE_REMIXES3.0 as ::core::ffi::c_int
        || (*self_0).choice_num
            == C2Rust_Unnamed_5::CHOICE_IPMUSIC.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(check_remixes(self_0, oldval), ());
    }
    if (*self_0).choice_num
        == C2Rust_Unnamed_5::CHOICE_WINDOWTYPE.0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(check_dos_3do_modes(self_0, oldval), ());
    }
}
unsafe extern "C" fn change_seedtype(
    mut self_0: *mut WIDGET_CHOICE,
    mut oldval: ::core::ffi::c_int,
) {
    if (*self_0).selected == OPT_SEED::OPTVAL_PRIME.0 as ::core::ffi::c_int {
        optCustomSeed = 16807 as ::core::ffi::c_int;
        snprintf(
            &raw mut (*(&raw mut textentries as *mut WIDGET_TEXTENTRY)
                .offset(
                    C2Rust_Unnamed_10::TEXT_GAMESEED.0 as ::core::ffi::c_int as isize,
                ))
                .value as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 31]>(),
            b"%d\0".as_ptr() as *const ::core::ffi::c_char,
            optCustomSeed,
        );
    }
}
unsafe extern "C" fn change_seed(mut self_0: *mut WIDGET_TEXTENTRY) {
    let mut customSeed: ::core::ffi::c_int = atoi(
        &raw mut (*self_0).value as *mut ::core::ffi::c_char,
    );
    if choices[C2Rust_Unnamed_5::CHOICE_GAMESEED.0 as ::core::ffi::c_int as usize]
        .selected == OPT_SEED::OPTVAL_PRIME.0 as ::core::ffi::c_int
        || if optCustomSeed < 3 as ::core::ffi::c_int
            || optCustomSeed > 2147483644 as ::core::ffi::c_int
        {
            0 as ::core::ffi::c_int
        } else {
            1 as ::core::ffi::c_int
        } == 0
    {
        customSeed = 16807 as ::core::ffi::c_int;
        snprintf(
            &raw mut (*self_0).value as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 31]>(),
            b"%d\0".as_ptr() as *const ::core::ffi::c_char,
            customSeed,
        );
    }
    optCustomSeed = customSeed;
}
unsafe extern "C" fn adjustMusic(mut self_0: *mut WIDGET_SLIDER) {
    musicVolumeScale = (*self_0).value as ::core::ffi::c_float / 100.0f32;
    uqm_lua_checked!(SetMusicVolume(musicVolume as COUNT), ());
}
unsafe extern "C" fn adjustSFX(mut self_0: *mut WIDGET_SLIDER) {
    sfxVolumeScale = (*self_0).value as ::core::ffi::c_float / 100.0f32;
}
unsafe extern "C" fn adjustSpeech(mut self_0: *mut WIDGET_SLIDER) {
    speechVolumeScale = (*self_0).value as ::core::ffi::c_float / 100.0f32;
    uqm_lua_checked!(SetSpeechVolume(speechVolumeScale), ());
    uqm_lua_checked!(TestSpeechSound(
        SetAbsStringTableIndex(
            testSounds,
            ((*self_0).value == 100 as ::core::ffi::c_int) as ::core::ffi::c_int as COUNT,
        ),
    ), ());
}
unsafe extern "C" fn toggle_scanlines(
    mut self_0: *mut WIDGET_CHOICE,
    mut NewGfxFlags: *mut ::core::ffi::c_int,
) {
    if (*self_0).selected == 1 as ::core::ffi::c_int {
        *NewGfxFlags |= (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int;
    } else {
        *NewGfxFlags &= !((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int);
    }
    uqm_lua_checked!(res_PutBoolean(
        b"config.scanlines\0".as_ptr() as *const ::core::ffi::c_char,
        BOOLEAN((*self_0).selected as ::core::ffi::c_uint),
    ), ());
}
unsafe extern "C" fn change_scaling(
    mut self_0: *mut WIDGET_CHOICE,
    mut NewWidth: *mut ::core::ffi::c_int,
    mut NewHeight: *mut ::core::ffi::c_int,
) {
    if (*self_0).selected < 6 as ::core::ffi::c_int {
        if (*self_0).selected == 0 {
            *NewWidth = (320 as ::core::ffi::c_int) << resolutionFactor;
            *NewHeight = if if optWindowType == 0 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                200 as ::core::ffi::c_int
            } else {
                240 as ::core::ffi::c_int
            } << resolutionFactor;
        } else {
            *NewWidth = 320 as ::core::ffi::c_int
                * (1 as ::core::ffi::c_int + (*self_0).selected);
            *NewHeight = if if optWindowType == 0 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                200 as ::core::ffi::c_int
            } else {
                240 as ::core::ffi::c_int
            } * (1 as ::core::ffi::c_int + (*self_0).selected);
        }
    }
    SavedWidth = inBounds(*NewWidth, 320 as SDWORD, 1920 as SDWORD)
        as ::core::ffi::c_int;
    SavedHeight = inBounds(*NewHeight, 200 as SDWORD, 1400 as SDWORD)
        as ::core::ffi::c_int;
    uqm_lua_checked!(PutIntOpt(
        &raw mut loresBlowupScale as *mut ::core::ffi::c_int,
        &raw mut (*self_0).selected,
        b"config.loresBlowupScale\0".as_ptr() as *const ::core::ffi::c_char,
        BOOLEAN::FALSE,
    ), ());
    uqm_lua_checked!(res_PutInteger(
        b"config.reswidth\0".as_ptr() as *const ::core::ffi::c_char,
        *NewWidth,
    ), ());
    uqm_lua_checked!(res_PutInteger(
        b"config.resheight\0".as_ptr() as *const ::core::ffi::c_char,
        *NewHeight,
    ), ());
}
unsafe extern "C" fn toggle_fullscreen(
    mut self_0: *mut WIDGET_CHOICE,
    mut NewGfxFlags: *mut ::core::ffi::c_int,
) {
    if (*self_0).selected == 1 as ::core::ffi::c_int {
        *NewGfxFlags &= !((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int);
        *NewGfxFlags |= (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
    } else if (*self_0).selected == 2 as ::core::ffi::c_int {
        *NewGfxFlags &= !((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int);
        *NewGfxFlags |= (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int;
    } else {
        *NewGfxFlags &= !((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int);
        *NewGfxFlags &= !((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int);
    }
    uqm_lua_checked!(res_PutInteger(
        b"config.fullscreen\0".as_ptr() as *const ::core::ffi::c_char,
        (*self_0).selected,
    ), ());
}
unsafe extern "C" fn change_scaler(
    mut self_0: *mut WIDGET_CHOICE,
    mut OldVal: ::core::ffi::c_int,
    mut NewGfxFlags: *mut ::core::ffi::c_int,
) {
    *NewGfxFlags &= !scalerList[OldVal as usize].value;
    *NewGfxFlags |= scalerList[(*self_0).selected as usize].value;
    uqm_lua_checked!(res_PutString(
        b"config.scaler\0".as_ptr() as *const ::core::ffi::c_char,
        scalerList[(*self_0).selected as usize].str,
    ), ());
}
unsafe extern "C" fn toggle_showfps(
    mut self_0: *mut WIDGET_CHOICE,
    mut NewGfxFlags: *mut ::core::ffi::c_int,
) {
    if (*self_0).selected == 1 as ::core::ffi::c_int {
        *NewGfxFlags |= (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int;
    } else {
        *NewGfxFlags &= !((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int);
    }
    uqm_lua_checked!(res_PutBoolean(
        b"config.showfps\0".as_ptr() as *const ::core::ffi::c_char,
        BOOLEAN((*self_0).selected as ::core::ffi::c_uint),
    ), ());
}
unsafe extern "C" fn change_gfxdriver(
    mut self_0: *mut WIDGET_CHOICE,
    mut NewGfxDriver: *mut ::core::ffi::c_int,
) {
    *NewGfxDriver = C2Rust_Unnamed_16::TFB_GFXDRIVER_SDL_PURE.0 as ::core::ffi::c_int;
    if GraphicsDriver != *NewGfxDriver {
        uqm_lua_checked!(res_PutBoolean(
            b"config.alwaysgl\0".as_ptr() as *const ::core::ffi::c_char,
            BOOLEAN((*self_0).selected as ::core::ffi::c_uint),
        ), ());
        uqm_lua_checked!(res_PutBoolean(
            b"config.usegl\0".as_ptr() as *const ::core::ffi::c_char,
            BOOLEAN(
                (*NewGfxDriver
                    == C2Rust_Unnamed_16::TFB_GFXDRIVER_SDL_OPENGL.0
                        as ::core::ffi::c_int) as ::core::ffi::c_int
                    as ::core::ffi::c_uint,
            ),
        ), ());
    }
}
#[no_mangle]
pub unsafe extern "C" fn process_graphics_options(
    mut self_0: *mut WIDGET_CHOICE,
    mut OldVal: ::core::ffi::c_int,
) {
    let mut NewGfxFlags: ::core::ffi::c_int = GfxFlags;
    let mut NewGfxDriver: ::core::ffi::c_int = GraphicsDriver;
    let mut NewWidth: ::core::ffi::c_int = SavedWidth;
    let mut NewHeight: ::core::ffi::c_int = SavedHeight;
    let mut isExclusive: BOOLEAN = BOOLEAN::FALSE;
    if OldVal == (*self_0).selected {
        return;
    }
    match (*self_0).choice_num {
        1 => {
            uqm_lua_checked!(change_gfxdriver(self_0, &raw mut NewGfxDriver), ());
        }
        2 => {
            uqm_lua_checked!(change_scaler(self_0, OldVal, &raw mut NewGfxFlags), ());
        }
        3 => {
            uqm_lua_checked!(toggle_scanlines(self_0, &raw mut NewGfxFlags), ());
        }
        10 => {
            uqm_lua_checked!(toggle_fullscreen(self_0, &raw mut NewGfxFlags), ());
        }
        12 => {
            uqm_lua_checked!(toggle_showfps(self_0, &raw mut NewGfxFlags), ());
        }
        23 => {
            optKeepAspectRatio = OPT_ENABLABLE(
                (*self_0).selected as ::core::ffi::c_uint,
            );
            uqm_lua_checked!(res_PutBoolean(
                b"config.keepaspectratio\0".as_ptr() as *const ::core::ffi::c_char,
                BOOLEAN((*self_0).selected as ::core::ffi::c_uint),
            ), ());
        }
        42 => {
            uqm_lua_checked!(change_scaling(self_0, &raw mut NewWidth, &raw mut NewHeight), ());
            isExclusive = BOOLEAN(
                (NewGfxFlags & (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
                    as ::core::ffi::c_uint,
            );
        }
        _ => return,
    }
    if optKeepAspectRatio.0 != 0 {
        let mut threshold: ::core::ffi::c_float = 0.75f32;
        let mut ratio: ::core::ffi::c_float = NewHeight as ::core::ffi::c_float
            / NewWidth as ::core::ffi::c_float;
        if ratio > threshold {
            NewWidth = (NewHeight as ::core::ffi::c_float / threshold)
                as ::core::ffi::c_int;
        } else if ratio < threshold {
            NewHeight = (NewWidth as ::core::ffi::c_float * threshold)
                as ::core::ffi::c_int;
        }
    }
    if NewWidth != WindowWidth || NewHeight != WindowHeight || NewGfxFlags != GfxFlags
        || NewGfxDriver != GraphicsDriver
    {
        if isExclusive.0 != 0 {
            NewGfxFlags &= !((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int);
        }
        TFB_DrawScreen_ReinitVideo(NewGfxDriver, NewGfxFlags, NewWidth, NewHeight);
    }
    FlushInput();
    if isExclusive.0 != 0 {
        GfxFlags |= (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
        TFB_DrawScreen_ReinitVideo(GraphicsDriver, GfxFlags, WindowWidth, WindowHeight);
    }
    populate_res();
}
unsafe extern "C" fn change_res(mut self_0: *mut WIDGET_TEXTENTRY) {
    let mut NewWidth: ::core::ffi::c_int = SavedWidth;
    let mut NewHeight: ::core::ffi::c_int = SavedHeight;
    let mut NewGfxFlags: ::core::ffi::c_int = GfxFlags;
    let mut isExclusive: BOOLEAN = BOOLEAN(
        (NewGfxFlags & (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
            as ::core::ffi::c_uint,
    );
    if sscanf(
        &raw mut (*self_0).value as *mut ::core::ffi::c_char,
        b"%dx%d\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut NewWidth,
        &raw mut NewHeight,
    ) != 2 as ::core::ffi::c_int
    {
        populate_res();
        return;
    }
    NewWidth = inBounds(NewWidth, 320 as SDWORD, 1920 as SDWORD) as ::core::ffi::c_int;
    NewHeight = inBounds(NewHeight, 200 as SDWORD, 1440 as SDWORD) as ::core::ffi::c_int;
    SavedWidth = NewWidth;
    SavedHeight = NewHeight;
    if optKeepAspectRatio.0 != 0 {
        let mut threshold: ::core::ffi::c_float = 0.75f32;
        let mut ratio: ::core::ffi::c_float = NewHeight as ::core::ffi::c_float
            / NewWidth as ::core::ffi::c_float;
        if ratio > threshold {
            NewWidth = (NewHeight as ::core::ffi::c_float / threshold)
                as ::core::ffi::c_int;
        } else if ratio < threshold {
            NewHeight = (NewWidth as ::core::ffi::c_float * threshold)
                as ::core::ffi::c_int;
        }
    }
    if NewWidth != WindowWidth || NewHeight != WindowHeight {
        if isExclusive.0 != 0 {
            NewGfxFlags &= !((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int);
        }
        TFB_DrawScreen_ReinitVideo(GraphicsDriver, GfxFlags, NewWidth, NewHeight);
    } else {
        return
    }
    if NewGfxFlags != GfxFlags {
        GfxFlags = NewGfxFlags;
    }
    FlushInput();
    if isExclusive.0 != 0 {
        GfxFlags |= (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
        TFB_DrawScreen_ReinitVideo(GraphicsDriver, GfxFlags, WindowWidth, WindowHeight);
    }
    populate_res();
    uqm_lua_checked!(PutIntOpt(
        &raw mut loresBlowupScale as *mut ::core::ffi::c_int,
        &raw mut (*(&raw mut choices as *mut WIDGET_CHOICE)
            .offset(
                C2Rust_Unnamed_5::CHOICE_RESOLUTION.0 as ::core::ffi::c_int as isize,
            ))
            .selected,
        b"config.loresBlowupScale\0".as_ptr() as *const ::core::ffi::c_char,
        BOOLEAN::FALSE,
    ), ());
    uqm_lua_checked!(res_PutInteger(
        b"config.reswidth\0".as_ptr() as *const ::core::ffi::c_char,
        SavedWidth,
    ), ());
    uqm_lua_checked!(res_PutInteger(
        b"config.resheight\0".as_ptr() as *const ::core::ffi::c_char,
        SavedHeight,
    ), ());
}
unsafe extern "C" fn SetDefaults() {
    let mut opts: GLOBALOPTS = GLOBALOPTS {
        scaler: OPT_SCALETYPE::OPTVAL_NO_SCALE,
        screenResolution: OPT_RESTYPE::OPTVAL_320_240,
        loresBlowup: OPT_RESSCALER::NO_BLOWUP,
        driver: OPT_DRIVERTYPE::OPTVAL_PURE_IF_POSSIBLE,
        adriver: OPT_ADRIVERTYPE::OPTVAL_SILENCE,
        aquality: OPT_AQUALITYTYPE::OPTVAL_LOW,
        godModes: OPT_GODTYPE::OPTVAL_NO_GOD,
        tdType: OPT_TDTYPE::OPTVAL_NORMAL,
        dateType: OPT_DATETYPE::OPTVAL_MMMDDYYYY,
        difficulty: OPT_DIFFICULTY::OPTVAL_NORM,
        nomad: OPT_NOMAD::OPTVAL_NOM_OFF,
        meleezoom: OPT_MELEEZOOM::OPTVAL_STEP,
        controllerType: OPT_CONTROLLER::OPTVAL_KBM,
        fuelRange: OPT_FUELRANGE::OPTVAL_FUEL_NORM,
        sphereType: OPT_SPHERETYPE::OPTVAL_PC_SPHERE,
        windowType: OPT_WINDOWTYPE::OPTVAL_PC_WINDOW,
        musicResume: OPT_MUSICRESUME::OPTVAL_RESUME_DISABLED,
        seedType: OPT_SEED::OPTVAL_PRIME,
        sphereColors: OPT_SPHERECOLORS::OPTVAL_DEFAULT_COLORS,
        spaceMusic: OPT_SPACEMUSIC::OPTVAL_NO_SPACEMUSIC,
        deviceArray: [OPT_ADD_REMOVE::OPTVAL_DO_NOTHING; 28],
        upgradeArray: [OPT_ADD_REMOVE::OPTVAL_DO_NOTHING; 13],
        dirJoy: [OPT_DIRECTJOY::OPTVAL_NORMAL_CONTROL; 2],
        fullscreen: OPT_ENABLABLE::OPTVAL_DISABLED,
        subtitles: OPT_ENABLABLE::OPTVAL_DISABLED,
        scanlines: OPT_ENABLABLE::OPTVAL_DISABLED,
        fps: OPT_ENABLABLE::OPTVAL_DISABLED,
        stereo: OPT_ENABLABLE::OPTVAL_DISABLED,
        music3do: OPT_ENABLABLE::OPTVAL_DISABLED,
        musicremix: OPT_ENABLABLE::OPTVAL_DISABLED,
        speech: OPT_ENABLABLE::OPTVAL_DISABLED,
        keepaspect: OPT_ENABLABLE::OPTVAL_DISABLED,
        cheatMode: OPT_ENABLABLE::OPTVAL_DISABLED,
        mainMenuMusic: OPT_ENABLABLE::OPTVAL_DISABLED,
        nebulae: OPT_ENABLABLE::OPTVAL_DISABLED,
        orbitingPlanets: OPT_ENABLABLE::OPTVAL_DISABLED,
        texturedPlanets: OPT_ENABLABLE::OPTVAL_DISABLED,
        godMode: OPT_ENABLABLE::OPTVAL_DISABLED,
        bubbleWarp: OPT_ENABLABLE::OPTVAL_DISABLED,
        unlockShips: OPT_ENABLABLE::OPTVAL_DISABLED,
        headStart: OPT_ENABLABLE::OPTVAL_DISABLED,
        autoButtons: OPT_ENABLABLE::OPTVAL_DISABLED,
        infiniteRU: OPT_ENABLABLE::OPTVAL_DISABLED,
        skipIntro: OPT_ENABLABLE::OPTVAL_DISABLED,
        infiniteFuel: OPT_ENABLABLE::OPTVAL_DISABLED,
        partialPickup: OPT_ENABLABLE::OPTVAL_DISABLED,
        submenu: OPT_ENABLABLE::OPTVAL_DISABLED,
        infiniteCredits: OPT_ENABLABLE::OPTVAL_DISABLED,
        customBorder: OPT_ENABLABLE::OPTVAL_DISABLED,
        volasMusic: OPT_ENABLABLE::OPTVAL_DISABLED,
        wholeFuel: OPT_ENABLABLE::OPTVAL_DISABLED,
        extended: OPT_ENABLABLE::OPTVAL_DISABLED,
        gameOver: OPT_ENABLABLE::OPTVAL_DISABLED,
        shipDirectionIP: OPT_ENABLABLE::OPTVAL_DISABLED,
        hazardColors: OPT_ENABLABLE::OPTVAL_DISABLED,
        orzCompFont: OPT_ENABLABLE::OPTVAL_DISABLED,
        smartAutoPilot: OPT_ENABLABLE::OPTVAL_DISABLED,
        nonStopOscill: OPT_ENABLABLE::OPTVAL_DISABLED,
        hyperStars: OPT_ENABLABLE::OPTVAL_DISABLED,
        planetTexture: OPT_ENABLABLE::OPTVAL_DISABLED,
        noHQEncounters: OPT_ENABLABLE::OPTVAL_DISABLED,
        deCleansing: OPT_ENABLABLE::OPTVAL_DISABLED,
        meleeObstacles: OPT_ENABLABLE::OPTVAL_DISABLED,
        showVisitedStars: OPT_ENABLABLE::OPTVAL_DISABLED,
        unscaledStarSystem: OPT_ENABLABLE::OPTVAL_DISABLED,
        slaughterMode: OPT_ENABLABLE::OPTVAL_DISABLED,
        advancedAutoPilot: OPT_ENABLABLE::OPTVAL_DISABLED,
        meleeToolTips: OPT_ENABLABLE::OPTVAL_DISABLED,
        scatterElements: OPT_ENABLABLE::OPTVAL_DISABLED,
        showUpgrades: OPT_ENABLABLE::OPTVAL_DISABLED,
        fleetPointSys: OPT_ENABLABLE::OPTVAL_DISABLED,
        shipSeed: OPT_ENABLABLE::OPTVAL_DISABLED,
        shipStore: OPT_ENABLABLE::OPTVAL_DISABLED,
        captainNames: OPT_ENABLABLE::OPTVAL_DISABLED,
        dosMenus: OPT_ENABLABLE::OPTVAL_DISABLED,
        menu: OPT_CONSOLETYPE::OPTVAL_PC,
        text: OPT_CONSOLETYPE::OPTVAL_PC,
        scroll: OPT_CONSOLETYPE::OPTVAL_PC,
        intro: OPT_CONSOLETYPE::OPTVAL_PC,
        shield: OPT_CONSOLETYPE::OPTVAL_PC,
        scrTrans: OPT_CONSOLETYPE::OPTVAL_PC,
        landerHold: OPT_CONSOLETYPE::OPTVAL_PC,
        tintPlanSphere: OPT_CONSOLETYPE::OPTVAL_PC,
        planetStyle: OPT_CONSOLETYPE::OPTVAL_PC,
        scanStyle: OPT_CONSOLETYPE::OPTVAL_PC,
        scopeStyle: OPT_CONSOLETYPE::OPTVAL_PC,
        landerStyle: OPT_CONSOLETYPE::OPTVAL_PC,
        flagshipColor: OPT_CONSOLETYPE::OPTVAL_PC,
        hyperSpaceColor: OPT_CONSOLETYPE::OPTVAL_PC,
        player1: CONTROL_TEMPLATE::CONTROL_TEMPLATE_PL_1,
        player2: CONTROL_TEMPLATE::CONTROL_TEMPLATE_PL_1,
        speechvol: 0,
        musicvol: 0,
        sfxvol: 0,
        nebulaevol: 0,
        cscan: 0,
        gamma: 0,
        starBackground: 0,
        deadZoneLeftStick: [0; 2],
        deadZoneRightStick: [0; 2],
    };
    let mut i: BYTE = 0;
    GetGlobalOptions(&raw mut opts);
    choices[C2Rust_Unnamed_5::CHOICE_GRAPHICS.0 as ::core::ffi::c_int as usize]
        .selected = opts.screenResolution.0 as ::core::ffi::c_int;
    choices[C2Rust_Unnamed_5::CHOICE_FRBUFFER.0 as ::core::ffi::c_int as usize]
        .selected = opts.driver.0 as ::core::ffi::c_int;
    choices[C2Rust_Unnamed_5::CHOICE_SCALER.0 as ::core::ffi::c_int as usize].selected = opts
        .scaler
        .0 as ::core::ffi::c_int;
    choices[C2Rust_Unnamed_5::CHOICE_SCANLINE.0 as ::core::ffi::c_int as usize]
        .selected = opts.scanlines.0 as ::core::ffi::c_int;
    choices[C2Rust_Unnamed_5::CHOICE_MENUSTYLE.0 as ::core::ffi::c_int as usize]
        .selected = opts.menu.0 as ::core::ffi::c_int;
    choices[C2Rust_Unnamed_5::CHOICE_FONTSTYLE.0 as ::core::ffi::c_int as usize]
        .selected = opts.text.0 as ::core::ffi::c_int;
    choices[C2Rust_Unnamed_5::CHOICE_SCANMENU.0 as ::core::ffi::c_int as usize]
        .selected = opts.cscan;
    choices[C2Rust_Unnamed_5::CHOICE_SCROLLSTYLE.0 as ::core::ffi::c_int as usize]
        .selected = opts.scroll.0 as ::core::ffi::c_int;
    choices[C2Rust_Unnamed_5::CHOICE_SUBTITLES.0 as ::core::ffi::c_int as usize]
        .selected = opts.subtitles.0 as ::core::ffi::c_int;
    choices[C2Rust_Unnamed_5::CHOICE_REMIXES1.0 as ::core::ffi::c_int as usize]
        .selected = opts.music3do.0 as ::core::ffi::c_int;
    choices[C2Rust_Unnamed_5::CHOICE_DISPLAY.0 as ::core::ffi::c_int as usize]
        .selected = opts.fullscreen.0 as ::core::ffi::c_int;
    choices[C2Rust_Unnamed_5::CHOICE_CUTSCENE.0 as ::core::ffi::c_int as usize]
        .selected = opts.intro.0 as ::core::ffi::c_int;
    choices[C2Rust_Unnamed_5::CHOICE_SHOWFPS.0 as ::core::ffi::c_int as usize]
        .selected = opts.fps.0 as ::core::ffi::c_int;
    choices[C2Rust_Unnamed_5::CHOICE_MELEEZOOM.0 as ::core::ffi::c_int as usize]
        .selected = opts.meleezoom.0 as ::core::ffi::c_int;
    choices[C2Rust_Unnamed_5::CHOICE_POSAUDIO.0 as ::core::ffi::c_int as usize]
        .selected = opts.stereo.0 as ::core::ffi::c_int;
    choices[C2Rust_Unnamed_5::CHOICE_SNDDRIVER.0 as ::core::ffi::c_int as usize]
        .selected = opts.adriver.0 as ::core::ffi::c_int;
    choices[C2Rust_Unnamed_5::CHOICE_SNDQUALITY.0 as ::core::ffi::c_int as usize]
        .selected = opts.aquality.0 as ::core::ffi::c_int;
    choices[C2Rust_Unnamed_5::CHOICE_SLVSHIELD.0 as ::core::ffi::c_int as usize]
        .selected = opts.shield.0 as ::core::ffi::c_int;
    choices[C2Rust_Unnamed_5::CHOICE_KBLAYOUT.0 as ::core::ffi::c_int as usize]
        .selected = 0 as ::core::ffi::c_int;
    choices[C2Rust_Unnamed_5::CHOICE_REMIXES2.0 as ::core::ffi::c_int as usize]
        .selected = opts.musicremix.0 as ::core::ffi::c_int;
    choices[C2Rust_Unnamed_5::CHOICE_SPEECH.0 as ::core::ffi::c_int as usize].selected = opts
        .speech
        .0 as ::core::ffi::c_int;
    choices[C2Rust_Unnamed_5::CHOICE_ASPRATIO.0 as ::core::ffi::c_int as usize]
        .selected = opts.keepaspect.0 as ::core::ffi::c_int;
    choices[C2Rust_Unnamed_5::CHOICE_CHEATING.0 as ::core::ffi::c_int as usize]
        .selected = opts.cheatMode.0 as ::core::ffi::c_int;
    choices[C2Rust_Unnamed_5::CHOICE_CHGODMODE.0 as ::core::ffi::c_int as usize]
        .selected = opts.godModes.0 as ::core::ffi::c_int;
    choices[C2Rust_Unnamed_5::CHOICE_CHTIME.0 as ::core::ffi::c_int as usize].selected = opts
        .tdType
        .0 as ::core::ffi::c_int;
    choices[C2Rust_Unnamed_5::CHOICE_CHWARP.0 as ::core::ffi::c_int as usize].selected = opts
        .bubbleWarp
        .0 as ::core::ffi::c_int;
    choices[C2Rust_Unnamed_5::CHOICE_CHSHIPS.0 as ::core::ffi::c_int as usize]
        .selected = opts.unlockShips.0 as ::core::ffi::c_int;
    choices[C2Rust_Unnamed_5::CHOICE_CHHEADSTART.0 as ::core::ffi::c_int as usize]
        .selected = opts.headStart.0 as ::core::ffi::c_int;
    choices[C2Rust_Unnamed_5::CHOICE_AUTOBUTT.0 as ::core::ffi::c_int as usize]
        .selected = opts.autoButtons.0 as ::core::ffi::c_int;
    choices[C2Rust_Unnamed_5::CHOICE_CHINFRU.0 as ::core::ffi::c_int as usize]
        .selected = opts.infiniteRU.0 as ::core::ffi::c_int;
    choices[C2Rust_Unnamed_5::CHOICE_SKIPINTRO.0 as ::core::ffi::c_int as usize]
        .selected = opts.skipIntro.0 as ::core::ffi::c_int;
    choices[C2Rust_Unnamed_5::CHOICE_FUELCIRCLE.0 as ::core::ffi::c_int as usize]
        .selected = opts.fuelRange.0 as ::core::ffi::c_int;
    choices[C2Rust_Unnamed_5::CHOICE_MMENUMUSIC.0 as ::core::ffi::c_int as usize]
        .selected = opts.mainMenuMusic.0 as ::core::ffi::c_int;
    choices[C2Rust_Unnamed_5::CHOICE_NEBULAE.0 as ::core::ffi::c_int as usize]
        .selected = opts.nebulae.0 as ::core::ffi::c_int;
    choices[C2Rust_Unnamed_5::CHOICE_ORBPLANETS.0 as ::core::ffi::c_int as usize]
        .selected = opts.orbitingPlanets.0 as ::core::ffi::c_int;
    choices[C2Rust_Unnamed_5::CHOICE_TEXPLANETS.0 as ::core::ffi::c_int as usize]
        .selected = opts.texturedPlanets.0 as ::core::ffi::c_int;
    choices[C2Rust_Unnamed_5::CHOICE_DATESTRING.0 as ::core::ffi::c_int as usize]
        .selected = opts.dateType.0 as ::core::ffi::c_int;
    choices[C2Rust_Unnamed_5::CHOICE_CHINFFUEL.0 as ::core::ffi::c_int as usize]
        .selected = opts.infiniteFuel.0 as ::core::ffi::c_int;
    choices[C2Rust_Unnamed_5::CHOICE_PARTPICKUP.0 as ::core::ffi::c_int as usize]
        .selected = opts.partialPickup.0 as ::core::ffi::c_int;
    choices[C2Rust_Unnamed_5::CHOICE_SUBMENU.0 as ::core::ffi::c_int as usize]
        .selected = opts.submenu.0 as ::core::ffi::c_int;
    choices[C2Rust_Unnamed_5::CHOICE_RESOLUTION.0 as ::core::ffi::c_int as usize]
        .selected = opts.loresBlowup.0 as ::core::ffi::c_int;
    choices[C2Rust_Unnamed_5::CHOICE_CHINFCRD.0 as ::core::ffi::c_int as usize]
        .selected = opts.infiniteCredits.0 as ::core::ffi::c_int;
    choices[C2Rust_Unnamed_5::CHOICE_HAZARDCLR.0 as ::core::ffi::c_int as usize]
        .selected = opts.hazardColors.0 as ::core::ffi::c_int;
    choices[C2Rust_Unnamed_5::CHOICE_CUSTBORDER.0 as ::core::ffi::c_int as usize]
        .selected = opts.customBorder.0 as ::core::ffi::c_int;
    choices[C2Rust_Unnamed_5::CHOICE_IPMUSIC.0 as ::core::ffi::c_int as usize]
        .selected = opts.spaceMusic.0 as ::core::ffi::c_int;
    choices[C2Rust_Unnamed_5::CHOICE_REMIXES3.0 as ::core::ffi::c_int as usize]
        .selected = opts.volasMusic.0 as ::core::ffi::c_int;
    choices[C2Rust_Unnamed_5::CHOICE_FUELDECIM.0 as ::core::ffi::c_int as usize]
        .selected = opts.wholeFuel.0 as ::core::ffi::c_int;
    choices[C2Rust_Unnamed_5::CHOICE_DIRJOYP1.0 as ::core::ffi::c_int as usize]
        .selected = opts.dirJoy[0usize].0 as ::core::ffi::c_int;
    choices[C2Rust_Unnamed_5::CHOICE_LANDERHOLD.0 as ::core::ffi::c_int as usize]
        .selected = opts.landerHold.0 as ::core::ffi::c_int;
    choices[C2Rust_Unnamed_5::CHOICE_SCRMELT.0 as ::core::ffi::c_int as usize]
        .selected = opts.scrTrans.0 as ::core::ffi::c_int;
    choices[C2Rust_Unnamed_5::CHOICE_SKILLLVL.0 as ::core::ffi::c_int as usize]
        .selected = opts.difficulty.0 as ::core::ffi::c_int;
    choices[C2Rust_Unnamed_5::CHOICE_EXTENDED.0 as ::core::ffi::c_int as usize]
        .selected = opts.extended.0 as ::core::ffi::c_int;
    choices[C2Rust_Unnamed_5::CHOICE_NOMAD.0 as ::core::ffi::c_int as usize].selected = opts
        .nomad
        .0 as ::core::ffi::c_int;
    choices[C2Rust_Unnamed_5::CHOICE_GAMEOVER.0 as ::core::ffi::c_int as usize]
        .selected = opts.gameOver.0 as ::core::ffi::c_int;
    choices[C2Rust_Unnamed_5::CHOICE_IPSHIPDIR.0 as ::core::ffi::c_int as usize]
        .selected = opts.shipDirectionIP.0 as ::core::ffi::c_int;
    choices[C2Rust_Unnamed_5::CHOICE_ORZFONT.0 as ::core::ffi::c_int as usize]
        .selected = opts.orzCompFont.0 as ::core::ffi::c_int;
    choices[C2Rust_Unnamed_5::CHOICE_INPDEVICE.0 as ::core::ffi::c_int as usize]
        .selected = opts.controllerType.0 as ::core::ffi::c_int;
    choices[C2Rust_Unnamed_5::CHOICE_SMARTAUTO.0 as ::core::ffi::c_int as usize]
        .selected = opts.smartAutoPilot.0 as ::core::ffi::c_int;
    choices[C2Rust_Unnamed_5::CHOICE_SCANTINT.0 as ::core::ffi::c_int as usize]
        .selected = opts.tintPlanSphere.0 as ::core::ffi::c_int;
    choices[C2Rust_Unnamed_5::CHOICE_IPSTYLE.0 as ::core::ffi::c_int as usize]
        .selected = opts.planetStyle.0 as ::core::ffi::c_int;
    choices[C2Rust_Unnamed_5::CHOICE_IPBACKGROUND.0 as ::core::ffi::c_int as usize]
        .selected = opts.starBackground;
    choices[C2Rust_Unnamed_5::CHOICE_SCANSTYLE.0 as ::core::ffi::c_int as usize]
        .selected = opts.scanStyle.0 as ::core::ffi::c_int;
    choices[C2Rust_Unnamed_5::CHOICE_NOSTOSCILL.0 as ::core::ffi::c_int as usize]
        .selected = opts.nonStopOscill.0 as ::core::ffi::c_int;
    choices[C2Rust_Unnamed_5::CHOICE_OSCILLSTYLE.0 as ::core::ffi::c_int as usize]
        .selected = opts.scopeStyle.0 as ::core::ffi::c_int;
    choices[C2Rust_Unnamed_5::CHOICE_ANIMHYPER.0 as ::core::ffi::c_int as usize]
        .selected = opts.hyperStars.0 as ::core::ffi::c_int;
    choices[C2Rust_Unnamed_5::CHOICE_LANDERSTYLE.0 as ::core::ffi::c_int as usize]
        .selected = opts.landerStyle.0 as ::core::ffi::c_int;
    choices[C2Rust_Unnamed_5::CHOICE_PLNTEXTURE.0 as ::core::ffi::c_int as usize]
        .selected = opts.planetTexture.0 as ::core::ffi::c_int;
    choices[C2Rust_Unnamed_5::CHOICE_FLAGSHIP.0 as ::core::ffi::c_int as usize]
        .selected = opts.flagshipColor.0 as ::core::ffi::c_int;
    choices[C2Rust_Unnamed_5::CHOICE_CHCLEANHYPER.0 as ::core::ffi::c_int as usize]
        .selected = opts.noHQEncounters.0 as ::core::ffi::c_int;
    choices[C2Rust_Unnamed_5::CHOICE_CHDECLEAN.0 as ::core::ffi::c_int as usize]
        .selected = opts.deCleansing.0 as ::core::ffi::c_int;
    choices[C2Rust_Unnamed_5::CHOICE_CHNOPLANET.0 as ::core::ffi::c_int as usize]
        .selected = opts.meleeObstacles.0 as ::core::ffi::c_int;
    choices[C2Rust_Unnamed_5::CHOICE_VISITED.0 as ::core::ffi::c_int as usize]
        .selected = opts.showVisitedStars.0 as ::core::ffi::c_int;
    choices[C2Rust_Unnamed_5::CHOICE_HDM_IP.0 as ::core::ffi::c_int as usize].selected = opts
        .unscaledStarSystem
        .0 as ::core::ffi::c_int;
    choices[C2Rust_Unnamed_5::CHOICE_SCANSPHERE.0 as ::core::ffi::c_int as usize]
        .selected = opts.sphereType.0 as ::core::ffi::c_int;
    choices[C2Rust_Unnamed_5::CHOICE_SLAUGHTER.0 as ::core::ffi::c_int as usize]
        .selected = opts.slaughterMode.0 as ::core::ffi::c_int;
    choices[C2Rust_Unnamed_5::CHOICE_ADVAUTO.0 as ::core::ffi::c_int as usize]
        .selected = opts.advancedAutoPilot.0 as ::core::ffi::c_int;
    choices[C2Rust_Unnamed_5::CHOICE_MLTOOLTIP.0 as ::core::ffi::c_int as usize]
        .selected = opts.meleeToolTips.0 as ::core::ffi::c_int;
    choices[C2Rust_Unnamed_5::CHOICE_MUSRESUME.0 as ::core::ffi::c_int as usize]
        .selected = opts.musicResume.0 as ::core::ffi::c_int;
    choices[C2Rust_Unnamed_5::CHOICE_WINDOWTYPE.0 as ::core::ffi::c_int as usize]
        .selected = opts.windowType.0 as ::core::ffi::c_int;
    choices[C2Rust_Unnamed_5::CHOICE_GAMESEED.0 as ::core::ffi::c_int as usize]
        .selected = opts.seedType.0 as ::core::ffi::c_int;
    choices[C2Rust_Unnamed_5::CHOICE_SOICOLOR.0 as ::core::ffi::c_int as usize]
        .selected = opts.sphereColors.0 as ::core::ffi::c_int;
    choices[C2Rust_Unnamed_5::CHOICE_SCATTERCARGO.0 as ::core::ffi::c_int as usize]
        .selected = opts.scatterElements.0 as ::core::ffi::c_int;
    choices[C2Rust_Unnamed_5::CHOICE_LANDERUPGMASK.0 as ::core::ffi::c_int as usize]
        .selected = opts.showUpgrades.0 as ::core::ffi::c_int;
    choices[C2Rust_Unnamed_5::CHOICE_FLEETPOINT.0 as ::core::ffi::c_int as usize]
        .selected = opts.fleetPointSys.0 as ::core::ffi::c_int;
    choices[C2Rust_Unnamed_5::CHOICE_HSCOLOR.0 as ::core::ffi::c_int as usize]
        .selected = opts.hyperSpaceColor.0 as ::core::ffi::c_int;
    choices[C2Rust_Unnamed_5::CHOICE_DIRJOYP2.0 as ::core::ffi::c_int as usize]
        .selected = opts.dirJoy[1usize].0 as ::core::ffi::c_int;
    i = 87 as BYTE;
    while (i as usize)
        < 87usize
            .wrapping_add(
                ::core::mem::size_of::<[OPT_ADD_REMOVE; 28]>()
                    .wrapping_div(::core::mem::size_of::<OPT_ADD_REMOVE>()),
            )
    {
        choices[i as usize].selected = opts
            .deviceArray[(i as ::core::ffi::c_int - 87 as ::core::ffi::c_int) as usize]
            .0 as ::core::ffi::c_int;
        i = i.wrapping_add(1);
    }
    i = 112 as BYTE;
    while (i as ::core::ffi::c_int)
        < 112 as ::core::ffi::c_int + 13 as ::core::ffi::c_int
    {
        choices[i as usize].selected = opts
            .upgradeArray[(i as ::core::ffi::c_int - 112 as ::core::ffi::c_int) as usize]
            .0 as ::core::ffi::c_int;
        i = i.wrapping_add(1);
    }
    choices[C2Rust_Unnamed_5::CHOICE_SHIPSEED.0 as ::core::ffi::c_int as usize]
        .selected = opts.shipSeed.0 as ::core::ffi::c_int;
    choices[C2Rust_Unnamed_5::CHOICE_SHIPSTORE.0 as ::core::ffi::c_int as usize]
        .selected = opts.shipStore.0 as ::core::ffi::c_int;
    choices[C2Rust_Unnamed_5::CHOICE_CAPTNAMES.0 as ::core::ffi::c_int as usize]
        .selected = opts.captainNames.0 as ::core::ffi::c_int;
    choices[C2Rust_Unnamed_5::CHOICE_DOSMENUS.0 as ::core::ffi::c_int as usize]
        .selected = opts.dosMenus.0 as ::core::ffi::c_int;
    sliders[C2Rust_Unnamed_7::SLIDER_MUSVOLUME.0 as ::core::ffi::c_int as usize].value = opts
        .musicvol;
    sliders[C2Rust_Unnamed_7::SLIDER_SFXVOLUME.0 as ::core::ffi::c_int as usize].value = opts
        .sfxvol;
    sliders[C2Rust_Unnamed_7::SLIDER_SPCHVOLUME.0 as ::core::ffi::c_int as usize]
        .value = opts.speechvol;
    sliders[C2Rust_Unnamed_7::SLIDER_GAMMA.0 as ::core::ffi::c_int as usize].value = opts
        .gamma;
    sliders[C2Rust_Unnamed_7::SLIDER_NEBULA.0 as ::core::ffi::c_int as usize].value = opts
        .nebulaevol;
    sliders[C2Rust_Unnamed_7::SLIDER_DEADZONE_00.0 as ::core::ffi::c_int as usize]
        .value = opts.deadZoneLeftStick[0usize];
    sliders[C2Rust_Unnamed_7::SLIDER_DEADZONE_01.0 as ::core::ffi::c_int as usize]
        .value = opts.deadZoneRightStick[0usize];
    sliders[C2Rust_Unnamed_7::SLIDER_DEADZONE_02.0 as ::core::ffi::c_int as usize]
        .value = opts.deadZoneLeftStick[1usize];
    sliders[C2Rust_Unnamed_7::SLIDER_DEADZONE_03.0 as ::core::ffi::c_int as usize]
        .value = opts.deadZoneRightStick[1usize];
}
unsafe extern "C" fn PropagateResults() {
    let mut opts: GLOBALOPTS = GLOBALOPTS {
        scaler: OPT_SCALETYPE::OPTVAL_NO_SCALE,
        screenResolution: OPT_RESTYPE::OPTVAL_320_240,
        loresBlowup: OPT_RESSCALER::NO_BLOWUP,
        driver: OPT_DRIVERTYPE::OPTVAL_PURE_IF_POSSIBLE,
        adriver: OPT_ADRIVERTYPE::OPTVAL_SILENCE,
        aquality: OPT_AQUALITYTYPE::OPTVAL_LOW,
        godModes: OPT_GODTYPE::OPTVAL_NO_GOD,
        tdType: OPT_TDTYPE::OPTVAL_NORMAL,
        dateType: OPT_DATETYPE::OPTVAL_MMMDDYYYY,
        difficulty: OPT_DIFFICULTY::OPTVAL_NORM,
        nomad: OPT_NOMAD::OPTVAL_NOM_OFF,
        meleezoom: OPT_MELEEZOOM::OPTVAL_STEP,
        controllerType: OPT_CONTROLLER::OPTVAL_KBM,
        fuelRange: OPT_FUELRANGE::OPTVAL_FUEL_NORM,
        sphereType: OPT_SPHERETYPE::OPTVAL_PC_SPHERE,
        windowType: OPT_WINDOWTYPE::OPTVAL_PC_WINDOW,
        musicResume: OPT_MUSICRESUME::OPTVAL_RESUME_DISABLED,
        seedType: OPT_SEED::OPTVAL_PRIME,
        sphereColors: OPT_SPHERECOLORS::OPTVAL_DEFAULT_COLORS,
        spaceMusic: OPT_SPACEMUSIC::OPTVAL_NO_SPACEMUSIC,
        deviceArray: [OPT_ADD_REMOVE::OPTVAL_DO_NOTHING; 28],
        upgradeArray: [OPT_ADD_REMOVE::OPTVAL_DO_NOTHING; 13],
        dirJoy: [OPT_DIRECTJOY::OPTVAL_NORMAL_CONTROL; 2],
        fullscreen: OPT_ENABLABLE::OPTVAL_DISABLED,
        subtitles: OPT_ENABLABLE::OPTVAL_DISABLED,
        scanlines: OPT_ENABLABLE::OPTVAL_DISABLED,
        fps: OPT_ENABLABLE::OPTVAL_DISABLED,
        stereo: OPT_ENABLABLE::OPTVAL_DISABLED,
        music3do: OPT_ENABLABLE::OPTVAL_DISABLED,
        musicremix: OPT_ENABLABLE::OPTVAL_DISABLED,
        speech: OPT_ENABLABLE::OPTVAL_DISABLED,
        keepaspect: OPT_ENABLABLE::OPTVAL_DISABLED,
        cheatMode: OPT_ENABLABLE::OPTVAL_DISABLED,
        mainMenuMusic: OPT_ENABLABLE::OPTVAL_DISABLED,
        nebulae: OPT_ENABLABLE::OPTVAL_DISABLED,
        orbitingPlanets: OPT_ENABLABLE::OPTVAL_DISABLED,
        texturedPlanets: OPT_ENABLABLE::OPTVAL_DISABLED,
        godMode: OPT_ENABLABLE::OPTVAL_DISABLED,
        bubbleWarp: OPT_ENABLABLE::OPTVAL_DISABLED,
        unlockShips: OPT_ENABLABLE::OPTVAL_DISABLED,
        headStart: OPT_ENABLABLE::OPTVAL_DISABLED,
        autoButtons: OPT_ENABLABLE::OPTVAL_DISABLED,
        infiniteRU: OPT_ENABLABLE::OPTVAL_DISABLED,
        skipIntro: OPT_ENABLABLE::OPTVAL_DISABLED,
        infiniteFuel: OPT_ENABLABLE::OPTVAL_DISABLED,
        partialPickup: OPT_ENABLABLE::OPTVAL_DISABLED,
        submenu: OPT_ENABLABLE::OPTVAL_DISABLED,
        infiniteCredits: OPT_ENABLABLE::OPTVAL_DISABLED,
        customBorder: OPT_ENABLABLE::OPTVAL_DISABLED,
        volasMusic: OPT_ENABLABLE::OPTVAL_DISABLED,
        wholeFuel: OPT_ENABLABLE::OPTVAL_DISABLED,
        extended: OPT_ENABLABLE::OPTVAL_DISABLED,
        gameOver: OPT_ENABLABLE::OPTVAL_DISABLED,
        shipDirectionIP: OPT_ENABLABLE::OPTVAL_DISABLED,
        hazardColors: OPT_ENABLABLE::OPTVAL_DISABLED,
        orzCompFont: OPT_ENABLABLE::OPTVAL_DISABLED,
        smartAutoPilot: OPT_ENABLABLE::OPTVAL_DISABLED,
        nonStopOscill: OPT_ENABLABLE::OPTVAL_DISABLED,
        hyperStars: OPT_ENABLABLE::OPTVAL_DISABLED,
        planetTexture: OPT_ENABLABLE::OPTVAL_DISABLED,
        noHQEncounters: OPT_ENABLABLE::OPTVAL_DISABLED,
        deCleansing: OPT_ENABLABLE::OPTVAL_DISABLED,
        meleeObstacles: OPT_ENABLABLE::OPTVAL_DISABLED,
        showVisitedStars: OPT_ENABLABLE::OPTVAL_DISABLED,
        unscaledStarSystem: OPT_ENABLABLE::OPTVAL_DISABLED,
        slaughterMode: OPT_ENABLABLE::OPTVAL_DISABLED,
        advancedAutoPilot: OPT_ENABLABLE::OPTVAL_DISABLED,
        meleeToolTips: OPT_ENABLABLE::OPTVAL_DISABLED,
        scatterElements: OPT_ENABLABLE::OPTVAL_DISABLED,
        showUpgrades: OPT_ENABLABLE::OPTVAL_DISABLED,
        fleetPointSys: OPT_ENABLABLE::OPTVAL_DISABLED,
        shipSeed: OPT_ENABLABLE::OPTVAL_DISABLED,
        shipStore: OPT_ENABLABLE::OPTVAL_DISABLED,
        captainNames: OPT_ENABLABLE::OPTVAL_DISABLED,
        dosMenus: OPT_ENABLABLE::OPTVAL_DISABLED,
        menu: OPT_CONSOLETYPE::OPTVAL_PC,
        text: OPT_CONSOLETYPE::OPTVAL_PC,
        scroll: OPT_CONSOLETYPE::OPTVAL_PC,
        intro: OPT_CONSOLETYPE::OPTVAL_PC,
        shield: OPT_CONSOLETYPE::OPTVAL_PC,
        scrTrans: OPT_CONSOLETYPE::OPTVAL_PC,
        landerHold: OPT_CONSOLETYPE::OPTVAL_PC,
        tintPlanSphere: OPT_CONSOLETYPE::OPTVAL_PC,
        planetStyle: OPT_CONSOLETYPE::OPTVAL_PC,
        scanStyle: OPT_CONSOLETYPE::OPTVAL_PC,
        scopeStyle: OPT_CONSOLETYPE::OPTVAL_PC,
        landerStyle: OPT_CONSOLETYPE::OPTVAL_PC,
        flagshipColor: OPT_CONSOLETYPE::OPTVAL_PC,
        hyperSpaceColor: OPT_CONSOLETYPE::OPTVAL_PC,
        player1: CONTROL_TEMPLATE::CONTROL_TEMPLATE_PL_1,
        player2: CONTROL_TEMPLATE::CONTROL_TEMPLATE_PL_1,
        speechvol: 0,
        musicvol: 0,
        sfxvol: 0,
        nebulaevol: 0,
        cscan: 0,
        gamma: 0,
        starBackground: 0,
        deadZoneLeftStick: [0; 2],
        deadZoneRightStick: [0; 2],
    };
    let mut i: BYTE = 0;
    opts.screenResolution = OPT_RESTYPE(
        choices[C2Rust_Unnamed_5::CHOICE_GRAPHICS.0 as ::core::ffi::c_int as usize]
            .selected as ::core::ffi::c_uint,
    );
    opts.driver = OPT_DRIVERTYPE(
        choices[C2Rust_Unnamed_5::CHOICE_FRBUFFER.0 as ::core::ffi::c_int as usize]
            .selected as ::core::ffi::c_uint,
    );
    opts.scaler = OPT_SCALETYPE(
        choices[C2Rust_Unnamed_5::CHOICE_SCALER.0 as ::core::ffi::c_int as usize]
            .selected as ::core::ffi::c_uint,
    );
    opts.scanlines = OPT_ENABLABLE(
        choices[C2Rust_Unnamed_5::CHOICE_SCANLINE.0 as ::core::ffi::c_int as usize]
            .selected as ::core::ffi::c_uint,
    );
    opts.menu = OPT_CONSOLETYPE(
        choices[C2Rust_Unnamed_5::CHOICE_MENUSTYLE.0 as ::core::ffi::c_int as usize]
            .selected as ::core::ffi::c_uint,
    );
    opts.text = OPT_CONSOLETYPE(
        choices[C2Rust_Unnamed_5::CHOICE_FONTSTYLE.0 as ::core::ffi::c_int as usize]
            .selected as ::core::ffi::c_uint,
    );
    opts.cscan = choices[C2Rust_Unnamed_5::CHOICE_SCANMENU.0 as ::core::ffi::c_int
            as usize]
        .selected;
    opts.scroll = OPT_CONSOLETYPE(
        choices[C2Rust_Unnamed_5::CHOICE_SCROLLSTYLE.0 as ::core::ffi::c_int as usize]
            .selected as ::core::ffi::c_uint,
    );
    opts.subtitles = OPT_ENABLABLE(
        choices[C2Rust_Unnamed_5::CHOICE_SUBTITLES.0 as ::core::ffi::c_int as usize]
            .selected as ::core::ffi::c_uint,
    );
    opts.music3do = OPT_ENABLABLE(
        choices[C2Rust_Unnamed_5::CHOICE_REMIXES1.0 as ::core::ffi::c_int as usize]
            .selected as ::core::ffi::c_uint,
    );
    opts.fullscreen = OPT_ENABLABLE(
        choices[C2Rust_Unnamed_5::CHOICE_DISPLAY.0 as ::core::ffi::c_int as usize]
            .selected as ::core::ffi::c_uint,
    );
    opts.intro = OPT_CONSOLETYPE(
        choices[C2Rust_Unnamed_5::CHOICE_CUTSCENE.0 as ::core::ffi::c_int as usize]
            .selected as ::core::ffi::c_uint,
    );
    opts.fps = OPT_ENABLABLE(
        choices[C2Rust_Unnamed_5::CHOICE_SHOWFPS.0 as ::core::ffi::c_int as usize]
            .selected as ::core::ffi::c_uint,
    );
    opts.meleezoom = OPT_MELEEZOOM(
        choices[C2Rust_Unnamed_5::CHOICE_MELEEZOOM.0 as ::core::ffi::c_int as usize]
            .selected as ::core::ffi::c_uint,
    );
    opts.stereo = OPT_ENABLABLE(
        choices[C2Rust_Unnamed_5::CHOICE_POSAUDIO.0 as ::core::ffi::c_int as usize]
            .selected as ::core::ffi::c_uint,
    );
    opts.adriver = OPT_ADRIVERTYPE(
        choices[C2Rust_Unnamed_5::CHOICE_SNDDRIVER.0 as ::core::ffi::c_int as usize]
            .selected as ::core::ffi::c_uint,
    );
    opts.aquality = OPT_AQUALITYTYPE(
        choices[C2Rust_Unnamed_5::CHOICE_SNDQUALITY.0 as ::core::ffi::c_int as usize]
            .selected as ::core::ffi::c_uint,
    );
    opts.shield = OPT_CONSOLETYPE(
        choices[C2Rust_Unnamed_5::CHOICE_SLVSHIELD.0 as ::core::ffi::c_int as usize]
            .selected as ::core::ffi::c_uint,
    );
    opts.musicremix = OPT_ENABLABLE(
        choices[C2Rust_Unnamed_5::CHOICE_REMIXES2.0 as ::core::ffi::c_int as usize]
            .selected as ::core::ffi::c_uint,
    );
    opts.speech = OPT_ENABLABLE(
        choices[C2Rust_Unnamed_5::CHOICE_SPEECH.0 as ::core::ffi::c_int as usize]
            .selected as ::core::ffi::c_uint,
    );
    opts.keepaspect = OPT_ENABLABLE(
        choices[C2Rust_Unnamed_5::CHOICE_ASPRATIO.0 as ::core::ffi::c_int as usize]
            .selected as ::core::ffi::c_uint,
    );
    opts.cheatMode = OPT_ENABLABLE(
        choices[C2Rust_Unnamed_5::CHOICE_CHEATING.0 as ::core::ffi::c_int as usize]
            .selected as ::core::ffi::c_uint,
    );
    opts.godModes = OPT_GODTYPE(
        choices[C2Rust_Unnamed_5::CHOICE_CHGODMODE.0 as ::core::ffi::c_int as usize]
            .selected as ::core::ffi::c_uint,
    );
    opts.tdType = OPT_TDTYPE(
        choices[C2Rust_Unnamed_5::CHOICE_CHTIME.0 as ::core::ffi::c_int as usize]
            .selected as ::core::ffi::c_uint,
    );
    opts.bubbleWarp = OPT_ENABLABLE(
        choices[C2Rust_Unnamed_5::CHOICE_CHWARP.0 as ::core::ffi::c_int as usize]
            .selected as ::core::ffi::c_uint,
    );
    opts.unlockShips = OPT_ENABLABLE(
        choices[C2Rust_Unnamed_5::CHOICE_CHSHIPS.0 as ::core::ffi::c_int as usize]
            .selected as ::core::ffi::c_uint,
    );
    opts.headStart = OPT_ENABLABLE(
        choices[C2Rust_Unnamed_5::CHOICE_CHHEADSTART.0 as ::core::ffi::c_int as usize]
            .selected as ::core::ffi::c_uint,
    );
    opts.autoButtons = OPT_ENABLABLE(
        choices[C2Rust_Unnamed_5::CHOICE_AUTOBUTT.0 as ::core::ffi::c_int as usize]
            .selected as ::core::ffi::c_uint,
    );
    opts.infiniteRU = OPT_ENABLABLE(
        choices[C2Rust_Unnamed_5::CHOICE_CHINFRU.0 as ::core::ffi::c_int as usize]
            .selected as ::core::ffi::c_uint,
    );
    opts.skipIntro = OPT_ENABLABLE(
        choices[C2Rust_Unnamed_5::CHOICE_SKIPINTRO.0 as ::core::ffi::c_int as usize]
            .selected as ::core::ffi::c_uint,
    );
    opts.fuelRange = OPT_FUELRANGE(
        choices[C2Rust_Unnamed_5::CHOICE_FUELCIRCLE.0 as ::core::ffi::c_int as usize]
            .selected as ::core::ffi::c_uint,
    );
    opts.mainMenuMusic = OPT_ENABLABLE(
        choices[C2Rust_Unnamed_5::CHOICE_MMENUMUSIC.0 as ::core::ffi::c_int as usize]
            .selected as ::core::ffi::c_uint,
    );
    opts.nebulae = OPT_ENABLABLE(
        choices[C2Rust_Unnamed_5::CHOICE_NEBULAE.0 as ::core::ffi::c_int as usize]
            .selected as ::core::ffi::c_uint,
    );
    opts.orbitingPlanets = OPT_ENABLABLE(
        choices[C2Rust_Unnamed_5::CHOICE_ORBPLANETS.0 as ::core::ffi::c_int as usize]
            .selected as ::core::ffi::c_uint,
    );
    opts.texturedPlanets = OPT_ENABLABLE(
        choices[C2Rust_Unnamed_5::CHOICE_TEXPLANETS.0 as ::core::ffi::c_int as usize]
            .selected as ::core::ffi::c_uint,
    );
    opts.dateType = OPT_DATETYPE(
        choices[C2Rust_Unnamed_5::CHOICE_DATESTRING.0 as ::core::ffi::c_int as usize]
            .selected as ::core::ffi::c_uint,
    );
    opts.infiniteFuel = OPT_ENABLABLE(
        choices[C2Rust_Unnamed_5::CHOICE_CHINFFUEL.0 as ::core::ffi::c_int as usize]
            .selected as ::core::ffi::c_uint,
    );
    opts.partialPickup = OPT_ENABLABLE(
        choices[C2Rust_Unnamed_5::CHOICE_PARTPICKUP.0 as ::core::ffi::c_int as usize]
            .selected as ::core::ffi::c_uint,
    );
    opts.submenu = OPT_ENABLABLE(
        choices[C2Rust_Unnamed_5::CHOICE_SUBMENU.0 as ::core::ffi::c_int as usize]
            .selected as ::core::ffi::c_uint,
    );
    opts.loresBlowup = OPT_RESSCALER(
        choices[C2Rust_Unnamed_5::CHOICE_RESOLUTION.0 as ::core::ffi::c_int as usize]
            .selected as ::core::ffi::c_uint,
    );
    opts.infiniteCredits = OPT_ENABLABLE(
        choices[C2Rust_Unnamed_5::CHOICE_CHINFCRD.0 as ::core::ffi::c_int as usize]
            .selected as ::core::ffi::c_uint,
    );
    opts.hazardColors = OPT_ENABLABLE(
        choices[C2Rust_Unnamed_5::CHOICE_HAZARDCLR.0 as ::core::ffi::c_int as usize]
            .selected as ::core::ffi::c_uint,
    );
    opts.customBorder = OPT_ENABLABLE(
        choices[C2Rust_Unnamed_5::CHOICE_CUSTBORDER.0 as ::core::ffi::c_int as usize]
            .selected as ::core::ffi::c_uint,
    );
    opts.spaceMusic = OPT_SPACEMUSIC(
        choices[C2Rust_Unnamed_5::CHOICE_IPMUSIC.0 as ::core::ffi::c_int as usize]
            .selected as ::core::ffi::c_uint,
    );
    opts.volasMusic = OPT_ENABLABLE(
        choices[C2Rust_Unnamed_5::CHOICE_REMIXES3.0 as ::core::ffi::c_int as usize]
            .selected as ::core::ffi::c_uint,
    );
    opts.wholeFuel = OPT_ENABLABLE(
        choices[C2Rust_Unnamed_5::CHOICE_FUELDECIM.0 as ::core::ffi::c_int as usize]
            .selected as ::core::ffi::c_uint,
    );
    opts.dirJoy[0usize] = OPT_DIRECTJOY(
        choices[C2Rust_Unnamed_5::CHOICE_DIRJOYP1.0 as ::core::ffi::c_int as usize]
            .selected as ::core::ffi::c_uint,
    );
    opts.landerHold = OPT_CONSOLETYPE(
        choices[C2Rust_Unnamed_5::CHOICE_LANDERHOLD.0 as ::core::ffi::c_int as usize]
            .selected as ::core::ffi::c_uint,
    );
    opts.scrTrans = OPT_CONSOLETYPE(
        choices[C2Rust_Unnamed_5::CHOICE_SCRMELT.0 as ::core::ffi::c_int as usize]
            .selected as ::core::ffi::c_uint,
    );
    opts.difficulty = OPT_DIFFICULTY(
        choices[C2Rust_Unnamed_5::CHOICE_SKILLLVL.0 as ::core::ffi::c_int as usize]
            .selected as ::core::ffi::c_uint,
    );
    opts.extended = OPT_ENABLABLE(
        choices[C2Rust_Unnamed_5::CHOICE_EXTENDED.0 as ::core::ffi::c_int as usize]
            .selected as ::core::ffi::c_uint,
    );
    opts.nomad = OPT_NOMAD(
        choices[C2Rust_Unnamed_5::CHOICE_NOMAD.0 as ::core::ffi::c_int as usize].selected
            as ::core::ffi::c_uint,
    );
    opts.gameOver = OPT_ENABLABLE(
        choices[C2Rust_Unnamed_5::CHOICE_GAMEOVER.0 as ::core::ffi::c_int as usize]
            .selected as ::core::ffi::c_uint,
    );
    opts.shipDirectionIP = OPT_ENABLABLE(
        choices[C2Rust_Unnamed_5::CHOICE_IPSHIPDIR.0 as ::core::ffi::c_int as usize]
            .selected as ::core::ffi::c_uint,
    );
    opts.orzCompFont = OPT_ENABLABLE(
        choices[C2Rust_Unnamed_5::CHOICE_ORZFONT.0 as ::core::ffi::c_int as usize]
            .selected as ::core::ffi::c_uint,
    );
    opts.controllerType = OPT_CONTROLLER(
        choices[C2Rust_Unnamed_5::CHOICE_INPDEVICE.0 as ::core::ffi::c_int as usize]
            .selected as ::core::ffi::c_uint,
    );
    opts.smartAutoPilot = OPT_ENABLABLE(
        choices[C2Rust_Unnamed_5::CHOICE_SMARTAUTO.0 as ::core::ffi::c_int as usize]
            .selected as ::core::ffi::c_uint,
    );
    opts.tintPlanSphere = OPT_CONSOLETYPE(
        choices[C2Rust_Unnamed_5::CHOICE_SCANTINT.0 as ::core::ffi::c_int as usize]
            .selected as ::core::ffi::c_uint,
    );
    opts.planetStyle = OPT_CONSOLETYPE(
        choices[C2Rust_Unnamed_5::CHOICE_IPSTYLE.0 as ::core::ffi::c_int as usize]
            .selected as ::core::ffi::c_uint,
    );
    opts.starBackground = choices[C2Rust_Unnamed_5::CHOICE_IPBACKGROUND.0
            as ::core::ffi::c_int as usize]
        .selected;
    opts.scanStyle = OPT_CONSOLETYPE(
        choices[C2Rust_Unnamed_5::CHOICE_SCANSTYLE.0 as ::core::ffi::c_int as usize]
            .selected as ::core::ffi::c_uint,
    );
    opts.nonStopOscill = OPT_ENABLABLE(
        choices[C2Rust_Unnamed_5::CHOICE_NOSTOSCILL.0 as ::core::ffi::c_int as usize]
            .selected as ::core::ffi::c_uint,
    );
    opts.scopeStyle = OPT_CONSOLETYPE(
        choices[C2Rust_Unnamed_5::CHOICE_OSCILLSTYLE.0 as ::core::ffi::c_int as usize]
            .selected as ::core::ffi::c_uint,
    );
    opts.hyperStars = OPT_ENABLABLE(
        choices[C2Rust_Unnamed_5::CHOICE_ANIMHYPER.0 as ::core::ffi::c_int as usize]
            .selected as ::core::ffi::c_uint,
    );
    opts.landerStyle = OPT_CONSOLETYPE(
        choices[C2Rust_Unnamed_5::CHOICE_LANDERSTYLE.0 as ::core::ffi::c_int as usize]
            .selected as ::core::ffi::c_uint,
    );
    opts.planetTexture = OPT_ENABLABLE(
        choices[C2Rust_Unnamed_5::CHOICE_PLNTEXTURE.0 as ::core::ffi::c_int as usize]
            .selected as ::core::ffi::c_uint,
    );
    opts.flagshipColor = OPT_CONSOLETYPE(
        choices[C2Rust_Unnamed_5::CHOICE_FLAGSHIP.0 as ::core::ffi::c_int as usize]
            .selected as ::core::ffi::c_uint,
    );
    opts.noHQEncounters = OPT_ENABLABLE(
        choices[C2Rust_Unnamed_5::CHOICE_CHCLEANHYPER.0 as ::core::ffi::c_int as usize]
            .selected as ::core::ffi::c_uint,
    );
    opts.deCleansing = OPT_ENABLABLE(
        choices[C2Rust_Unnamed_5::CHOICE_CHDECLEAN.0 as ::core::ffi::c_int as usize]
            .selected as ::core::ffi::c_uint,
    );
    opts.meleeObstacles = OPT_ENABLABLE(
        choices[C2Rust_Unnamed_5::CHOICE_CHNOPLANET.0 as ::core::ffi::c_int as usize]
            .selected as ::core::ffi::c_uint,
    );
    opts.showVisitedStars = OPT_ENABLABLE(
        choices[C2Rust_Unnamed_5::CHOICE_VISITED.0 as ::core::ffi::c_int as usize]
            .selected as ::core::ffi::c_uint,
    );
    opts.unscaledStarSystem = OPT_ENABLABLE(
        choices[C2Rust_Unnamed_5::CHOICE_HDM_IP.0 as ::core::ffi::c_int as usize]
            .selected as ::core::ffi::c_uint,
    );
    opts.sphereType = OPT_SPHERETYPE(
        choices[C2Rust_Unnamed_5::CHOICE_SCANSPHERE.0 as ::core::ffi::c_int as usize]
            .selected as ::core::ffi::c_uint,
    );
    opts.slaughterMode = OPT_ENABLABLE(
        choices[C2Rust_Unnamed_5::CHOICE_SLAUGHTER.0 as ::core::ffi::c_int as usize]
            .selected as ::core::ffi::c_uint,
    );
    opts.advancedAutoPilot = OPT_ENABLABLE(
        choices[C2Rust_Unnamed_5::CHOICE_ADVAUTO.0 as ::core::ffi::c_int as usize]
            .selected as ::core::ffi::c_uint,
    );
    opts.meleeToolTips = OPT_ENABLABLE(
        choices[C2Rust_Unnamed_5::CHOICE_MLTOOLTIP.0 as ::core::ffi::c_int as usize]
            .selected as ::core::ffi::c_uint,
    );
    opts.musicResume = OPT_MUSICRESUME(
        choices[C2Rust_Unnamed_5::CHOICE_MUSRESUME.0 as ::core::ffi::c_int as usize]
            .selected as ::core::ffi::c_uint,
    );
    opts.windowType = OPT_WINDOWTYPE(
        choices[C2Rust_Unnamed_5::CHOICE_WINDOWTYPE.0 as ::core::ffi::c_int as usize]
            .selected as ::core::ffi::c_uint,
    );
    opts.seedType = OPT_SEED(
        choices[C2Rust_Unnamed_5::CHOICE_GAMESEED.0 as ::core::ffi::c_int as usize]
            .selected as ::core::ffi::c_uint,
    );
    opts.sphereColors = OPT_SPHERECOLORS(
        choices[C2Rust_Unnamed_5::CHOICE_SOICOLOR.0 as ::core::ffi::c_int as usize]
            .selected as ::core::ffi::c_uint,
    );
    opts.scatterElements = OPT_ENABLABLE(
        choices[C2Rust_Unnamed_5::CHOICE_SCATTERCARGO.0 as ::core::ffi::c_int as usize]
            .selected as ::core::ffi::c_uint,
    );
    opts.showUpgrades = OPT_ENABLABLE(
        choices[C2Rust_Unnamed_5::CHOICE_LANDERUPGMASK.0 as ::core::ffi::c_int as usize]
            .selected as ::core::ffi::c_uint,
    );
    opts.fleetPointSys = OPT_ENABLABLE(
        choices[C2Rust_Unnamed_5::CHOICE_FLEETPOINT.0 as ::core::ffi::c_int as usize]
            .selected as ::core::ffi::c_uint,
    );
    opts.hyperSpaceColor = OPT_CONSOLETYPE(
        choices[C2Rust_Unnamed_5::CHOICE_HSCOLOR.0 as ::core::ffi::c_int as usize]
            .selected as ::core::ffi::c_uint,
    );
    opts.dirJoy[1usize] = OPT_DIRECTJOY(
        choices[C2Rust_Unnamed_5::CHOICE_DIRJOYP2.0 as ::core::ffi::c_int as usize]
            .selected as ::core::ffi::c_uint,
    );
    i = 87 as BYTE;
    while (i as usize)
        < 87usize
            .wrapping_add(
                ::core::mem::size_of::<[OPT_ADD_REMOVE; 28]>()
                    .wrapping_div(::core::mem::size_of::<OPT_ADD_REMOVE>()),
            )
    {
        opts
            .deviceArray[(i as ::core::ffi::c_int - 87 as ::core::ffi::c_int)
            as usize] = OPT_ADD_REMOVE(
            choices[i as usize].selected as ::core::ffi::c_uint,
        );
        i = i.wrapping_add(1);
    }
    i = 112 as BYTE;
    while (i as ::core::ffi::c_int)
        < 112 as ::core::ffi::c_int + 13 as ::core::ffi::c_int
    {
        opts
            .upgradeArray[(i as ::core::ffi::c_int - 112 as ::core::ffi::c_int)
            as usize] = OPT_ADD_REMOVE(
            choices[i as usize].selected as ::core::ffi::c_uint,
        );
        i = i.wrapping_add(1);
    }
    opts.shipSeed = OPT_ENABLABLE(
        choices[C2Rust_Unnamed_5::CHOICE_SHIPSEED.0 as ::core::ffi::c_int as usize]
            .selected as ::core::ffi::c_uint,
    );
    opts.shipStore = OPT_ENABLABLE(
        choices[C2Rust_Unnamed_5::CHOICE_SHIPSTORE.0 as ::core::ffi::c_int as usize]
            .selected as ::core::ffi::c_uint,
    );
    opts.captainNames = OPT_ENABLABLE(
        choices[C2Rust_Unnamed_5::CHOICE_CAPTNAMES.0 as ::core::ffi::c_int as usize]
            .selected as ::core::ffi::c_uint,
    );
    opts.dosMenus = OPT_ENABLABLE(
        choices[C2Rust_Unnamed_5::CHOICE_DOSMENUS.0 as ::core::ffi::c_int as usize]
            .selected as ::core::ffi::c_uint,
    );
    opts.musicvol = sliders[C2Rust_Unnamed_7::SLIDER_MUSVOLUME.0 as ::core::ffi::c_int
            as usize]
        .value;
    opts.sfxvol = sliders[C2Rust_Unnamed_7::SLIDER_SFXVOLUME.0 as ::core::ffi::c_int
            as usize]
        .value;
    opts.speechvol = sliders[C2Rust_Unnamed_7::SLIDER_SPCHVOLUME.0 as ::core::ffi::c_int
            as usize]
        .value;
    opts.gamma = sliders[C2Rust_Unnamed_7::SLIDER_GAMMA.0 as ::core::ffi::c_int as usize]
        .value;
    opts.nebulaevol = sliders[C2Rust_Unnamed_7::SLIDER_NEBULA.0 as ::core::ffi::c_int
            as usize]
        .value;
    opts.deadZoneLeftStick[0usize] = sliders[C2Rust_Unnamed_7::SLIDER_DEADZONE_00.0
            as ::core::ffi::c_int as usize]
        .value;
    opts.deadZoneRightStick[0usize] = sliders[C2Rust_Unnamed_7::SLIDER_DEADZONE_01.0
            as ::core::ffi::c_int as usize]
        .value;
    opts.deadZoneLeftStick[1usize] = sliders[C2Rust_Unnamed_7::SLIDER_DEADZONE_02.0
            as ::core::ffi::c_int as usize]
        .value;
    opts.deadZoneRightStick[1usize] = sliders[C2Rust_Unnamed_7::SLIDER_DEADZONE_03.0
            as ::core::ffi::c_int as usize]
        .value;
    uqm_lua_checked!(SetGlobalOptions(&raw mut opts), ());
}
unsafe extern "C" fn DoSetupMenu(mut pInputState: *mut SETUP_MENU_STATE) -> BOOLEAN {
    ::core::ptr::write_volatile(&raw mut GamePaused, BOOLEAN::FALSE);
    if (*pInputState).initialized.0 == 0 {
        SetDefaultMenuRepeatDelay();
        (*pInputState).NextTime = GetTimeCounter() as DWORD;
        SetDefaults();
        Widget_SetFont(PlyrFont);
        Widget_SetWindowColors(
            colorFromRgb15(0x14 as BYTE, 0x14 as BYTE, 0x14 as BYTE),
            colorFromRgb15(0x8 as BYTE, 0x8 as BYTE, 0x8 as BYTE),
            colorFromRgb15(0x10 as BYTE, 0x10 as BYTE, 0x10 as BYTE),
        );
        current = ::core::ptr::null_mut::<WIDGET>();
        next = (&raw mut menus as *mut WIDGET_MENU_SCREEN).offset(0isize) as *mut WIDGET;
        uqm_lua_checked!(Some((*next).receiveFocus.expect("non-null function pointer"))
            .expect(
                "non-null function pointer",
            )(next, C2Rust_Unnamed_18::WIDGET_EVENT_DOWN.0 as ::core::ffi::c_int), BOOLEAN(0));
        (*pInputState).initialized = BOOLEAN::TRUE;
    }
    if current != next {
        SetTransitionSource(::core::ptr::null::<RECT>());
    }
    BatchGraphics();
    uqm_lua_checked!(Some((*next).draw.expect("non-null function pointer"))
        .expect(
            "non-null function pointer",
        )(next, 0 as ::core::ffi::c_int, 0 as ::core::ffi::c_int), BOOLEAN(0));
    if current != next {
        uqm_lua_checked!(ScreenTransition(3 as ::core::ffi::c_int, ::core::ptr::null::<RECT>()), BOOLEAN(0));
        current = next;
    }
    UnbatchGraphics();
    if PulsedInputState
        .menu[C2Rust_Unnamed_4::KEY_MENU_UP.0 as ::core::ffi::c_int as usize] != 0
    {
        uqm_lua_checked!(Widget_Event(C2Rust_Unnamed_18::WIDGET_EVENT_UP.0 as ::core::ffi::c_int), BOOLEAN(0));
    } else if PulsedInputState
        .menu[C2Rust_Unnamed_4::KEY_MENU_DOWN.0 as ::core::ffi::c_int as usize] != 0
    {
        uqm_lua_checked!(Widget_Event(C2Rust_Unnamed_18::WIDGET_EVENT_DOWN.0 as ::core::ffi::c_int), BOOLEAN(0));
    } else if PulsedInputState
        .menu[C2Rust_Unnamed_4::KEY_MENU_LEFT.0 as ::core::ffi::c_int as usize] != 0
    {
        uqm_lua_checked!(Widget_Event(C2Rust_Unnamed_18::WIDGET_EVENT_LEFT.0 as ::core::ffi::c_int), BOOLEAN(0));
    } else if PulsedInputState
        .menu[C2Rust_Unnamed_4::KEY_MENU_RIGHT.0 as ::core::ffi::c_int as usize] != 0
    {
        uqm_lua_checked!(Widget_Event(C2Rust_Unnamed_18::WIDGET_EVENT_RIGHT.0 as ::core::ffi::c_int), BOOLEAN(0));
    }
    if PulsedInputState
        .menu[C2Rust_Unnamed_4::KEY_MENU_SELECT.0 as ::core::ffi::c_int as usize] != 0
    {
        uqm_lua_checked!(Widget_Event(C2Rust_Unnamed_18::WIDGET_EVENT_SELECT.0 as ::core::ffi::c_int), BOOLEAN(0));
    }
    if PulsedInputState
        .menu[C2Rust_Unnamed_4::KEY_MENU_CANCEL.0 as ::core::ffi::c_int as usize] != 0
    {
        uqm_lua_checked!(Widget_Event(C2Rust_Unnamed_18::WIDGET_EVENT_CANCEL.0 as ::core::ffi::c_int), BOOLEAN(0));
    }
    if PulsedInputState
        .menu[C2Rust_Unnamed_4::KEY_MENU_DELETE.0 as ::core::ffi::c_int as usize] != 0
    {
        uqm_lua_checked!(Widget_Event(C2Rust_Unnamed_18::WIDGET_EVENT_DELETE.0 as ::core::ffi::c_int), BOOLEAN(0));
    }
    uqm_lua_checked!(SleepThreadUntil(
        (*pInputState)
            .NextTime
            .wrapping_add(
                (840 as ::core::ffi::c_int / 20 as ::core::ffi::c_int) as TimeCount,
            ),
    ), BOOLEAN(0));
    (*pInputState).NextTime = GetTimeCounter() as DWORD;
    return BOOLEAN(
        !(GlobData.Game_state.CurrentActivity as ::core::ffi::c_int
            & C2Rust_Unnamed_15::CHECK_ABORT.0 as ::core::ffi::c_int != 0
            || next.is_null()) as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
}
unsafe extern "C" fn redraw_menu() {
    BatchGraphics();
    uqm_lua_checked!(Some((*next).draw.expect("non-null function pointer"))
        .expect(
            "non-null function pointer",
        )(next, 0 as ::core::ffi::c_int, 0 as ::core::ffi::c_int), ());
    UnbatchGraphics();
}
unsafe extern "C" fn OnTextEntryChange(mut pTES: *mut TEXTENTRY_STATE) -> BOOLEAN {
    let mut widget: *mut WIDGET_TEXTENTRY = (*pTES).CbParam as *mut WIDGET_TEXTENTRY;
    (*widget).cursor_pos = (*pTES).CursorPos;
    if (*pTES).JoystickMode.0 != 0 {
        (*widget).state = WIDGET_TEXTENTRY_STATE(
            (*widget).state.0 | WIDGET_TEXTENTRY_STATE::WTE_BLOCKCUR.0,
        );
    } else {
        (*widget).state = WIDGET_TEXTENTRY_STATE(
            (*widget).state.0
                & !(WIDGET_TEXTENTRY_STATE::WTE_BLOCKCUR.0 as ::core::ffi::c_int)
                    as ::core::ffi::c_uint,
        );
    }
    return BOOLEAN::TRUE;
}
unsafe extern "C" fn OnTextEntryFrame(mut pTES: *mut TEXTENTRY_STATE) -> BOOLEAN {
    uqm_lua_checked!(redraw_menu(), BOOLEAN(0));
    uqm_lua_checked!(SleepThreadUntil((*pTES).NextTime), BOOLEAN(0));
    (*pTES).NextTime = GetTimeCounter()
        .wrapping_add(
            (840 as ::core::ffi::c_int / 20 as ::core::ffi::c_int) as TimeCount,
        ) as DWORD;
    return BOOLEAN::TRUE;
}
unsafe extern "C" fn OnTextEntryEvent(
    mut widget: *mut WIDGET_TEXTENTRY,
) -> ::core::ffi::c_int {
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
    let mut revert_buf: [UNICODE; 256] = [0; 256];
    (*widget).cursor_pos = utf8StringCount(
        &raw mut (*widget).value as *mut ::core::ffi::c_char,
    ) as ::core::ffi::c_int;
    (*widget).state = WIDGET_TEXTENTRY_STATE::WTE_EDITING;
    uqm_lua_checked!(redraw_menu(), 0);
    utf8StringCopy(
        &raw mut revert_buf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[UNICODE; 256]>(),
        &raw mut (*widget).value as *mut ::core::ffi::c_char,
    );
    tes.Initialized = BOOLEAN::FALSE;
    tes.NextTime = GetTimeCounter()
        .wrapping_add(
            (840 as ::core::ffi::c_int / 20 as ::core::ffi::c_int) as TimeCount,
        ) as DWORD;
    tes.BaseStr = &raw mut (*widget).value as *mut ::core::ffi::c_char as *mut UNICODE;
    tes.MaxSize = (*widget).maxlen;
    tes.CursorPos = (*widget).cursor_pos;
    tes.CbParam = widget as *mut ::core::ffi::c_void;
    tes.ChangeCallback = Some(
        OnTextEntryChange as unsafe extern "C" fn(*mut TEXTENTRY_STATE) -> BOOLEAN,
    ) as Option<unsafe extern "C" fn(*mut textentry_state) -> BOOLEAN>;
    tes.FrameCallback = Some(
        OnTextEntryFrame as unsafe extern "C" fn(*mut TEXTENTRY_STATE) -> BOOLEAN,
    ) as Option<unsafe extern "C" fn(*mut textentry_state) -> BOOLEAN>;
    SetMenuSounds(
        0 as ::core::ffi::c_int as MENU_SOUND_FLAGS,
        ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as MENU_SOUND_FLAGS,
    );
    if uqm_lua_checked!(DoTextEntry(&raw mut tes), 0).0 == 0 {
        utf8StringCopy(
            &raw mut (*widget).value as *mut ::core::ffi::c_char,
            (*widget).maxlen as size_t,
            &raw mut revert_buf as *mut UNICODE,
        );
    } else if (*widget).onChange.is_some() {
        uqm_lua_checked!(Some((*widget).onChange.expect("non-null function pointer"))
            .expect("non-null function pointer")(widget), 0);
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
    (*widget).state = WIDGET_TEXTENTRY_STATE::WTE_NORMAL;
    uqm_lua_checked!(redraw_menu(), 0);
    return BOOLEAN::TRUE.0 as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn gammaCurve(mut x: ::core::ffi::c_float) -> ::core::ffi::c_float {
    return exp(
        ((x - 1 as ::core::ffi::c_int as ::core::ffi::c_float) * 2.1972f32)
            as ::core::ffi::c_double,
    ) as ::core::ffi::c_float;
}
#[inline]
unsafe extern "C" fn solveGammaCurve(
    mut y: ::core::ffi::c_float,
) -> ::core::ffi::c_float {
    return (log(y as ::core::ffi::c_double) / 2.1972f64
        + 1 as ::core::ffi::c_int as ::core::ffi::c_double) as ::core::ffi::c_float;
}
unsafe extern "C" fn gammaToSlider(
    mut gamma: ::core::ffi::c_float,
) -> ::core::ffi::c_int {
    let x: ::core::ffi::c_float = solveGammaCurve(gamma);
    let step: ::core::ffi::c_float = (maxGammaX - minGammaX)
        / 100 as ::core::ffi::c_int as ::core::ffi::c_float;
    return (((x - minGammaX) / step) as ::core::ffi::c_double + 0.5f64)
        as ::core::ffi::c_int;
}
unsafe extern "C" fn sliderToGamma(
    mut value: ::core::ffi::c_int,
) -> ::core::ffi::c_float {
    let step: ::core::ffi::c_float = (maxGammaX - minGammaX)
        / 100 as ::core::ffi::c_int as ::core::ffi::c_float;
    let x: ::core::ffi::c_float = minGammaX + step * value as ::core::ffi::c_float;
    let g: ::core::ffi::c_float = gammaCurve(x);
    return if fabs((g - 1.0f32) as ::core::ffi::c_double) < 0.001f64 {
        1.0f32
    } else {
        g
    };
}
unsafe extern "C" fn updateGammaBounds(mut useUpper: bool) {
    let mut g: ::core::ffi::c_float = 0.;
    let mut x: ::core::ffi::c_float = 0.;
    let mut slider: ::core::ffi::c_int = 0;
    minGammaX = solveGammaCurve(minGamma);
    maxGammaX = solveGammaCurve(maxGamma);
    slider = gammaToSlider(1.0f32);
    g = sliderToGamma(slider);
    if g == 1.0f32 {
        return;
    }
    x = solveGammaCurve(g);
    if useUpper {
        let d: ::core::ffi::c_float = (x - 1.0f32)
            * 100 as ::core::ffi::c_int as ::core::ffi::c_float
            / slider as ::core::ffi::c_float;
        maxGammaX -= d;
        maxGamma = gammaCurve(maxGammaX);
    } else {
        let d_0: ::core::ffi::c_float = (x - 1.0f32)
            * 100 as ::core::ffi::c_int as ::core::ffi::c_float
            / (100 as ::core::ffi::c_int - slider) as ::core::ffi::c_float;
        minGammaX -= d_0;
        minGamma = gammaCurve(minGammaX);
    };
}
unsafe extern "C" fn gamma_HandleEventSlider(
    mut _self: *mut WIDGET,
    mut event: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut self_0: *mut WIDGET_SLIDER = _self as *mut WIDGET_SLIDER;
    let mut prevValue: ::core::ffi::c_int = (*self_0).value;
    let mut gamma: ::core::ffi::c_float = 0.;
    let mut set: bool = false;
    match event {
        2 => {
            (*self_0).value -= (*self_0).step;
        }
        3 => {
            (*self_0).value += (*self_0).step;
        }
        _ => return BOOLEAN::FALSE.0 as ::core::ffi::c_int,
    }
    gamma = sliderToGamma((*self_0).value);
    set = TFB_SetGamma(gamma);
    if !set {
        (*self_0).value = prevValue;
        gamma = sliderToGamma((*self_0).value);
    }
    if gamma < minGamma
        || !set && event == C2Rust_Unnamed_18::WIDGET_EVENT_LEFT.0 as ::core::ffi::c_int
    {
        gamma = minGamma;
        updateGammaBounds(true);
        (*self_0).value = 0 as ::core::ffi::c_int;
    } else if gamma > maxGamma
        || !set && event == C2Rust_Unnamed_18::WIDGET_EVENT_RIGHT.0 as ::core::ffi::c_int
    {
        gamma = maxGamma;
        updateGammaBounds(false);
        (*self_0).value = 100 as ::core::ffi::c_int;
    }
    return BOOLEAN::TRUE.0 as ::core::ffi::c_int;
}
unsafe extern "C" fn gamma_DrawValue(
    mut self_0: *mut WIDGET_SLIDER,
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
) {
    let mut t: TEXT = TEXT {
        baseline: POINT { x: 0, y: 0 },
        pStr: ::core::ptr::null::<UNICODE>(),
        align: TEXT_ALIGN::ALIGN_LEFT,
        CharCount: 0,
    };
    let mut buf: [::core::ffi::c_char; 16] = [0; 16];
    let mut gamma: ::core::ffi::c_float = sliderToGamma((*self_0).value);
    snprintf(
        &raw mut buf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
        b"%.4f\0".as_ptr() as *const ::core::ffi::c_char,
        gamma as ::core::ffi::c_double,
    );
    t.baseline.x = (x + ((6 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    t.baseline.y = y as COORD;
    t.align = TEXT_ALIGN::ALIGN_LEFT;
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    t.pStr = &raw mut buf as *mut ::core::ffi::c_char;
    uqm_lua_checked!(font_DrawText(&raw mut t), ());
}
#[inline]
unsafe extern "C" fn deadzoneToSlider(
    mut deadzone: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return deadzone * 100 as ::core::ffi::c_int / 32767 as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn sliderToDeadzone(
    mut value: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return value * 32767 as ::core::ffi::c_int / 100 as ::core::ffi::c_int;
}
unsafe extern "C" fn deadzone_DrawValue(
    mut self_0: *mut WIDGET_SLIDER,
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
) {
    let mut t: TEXT = TEXT {
        baseline: POINT { x: 0, y: 0 },
        pStr: ::core::ptr::null::<UNICODE>(),
        align: TEXT_ALIGN::ALIGN_LEFT,
        CharCount: 0,
    };
    let mut buf: [::core::ffi::c_char; 16] = [0; 16];
    snprintf(
        &raw mut buf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
        b"%d%%\0".as_ptr() as *const ::core::ffi::c_char,
        (*self_0).value,
    );
    t.baseline.x = (x + ((6 as ::core::ffi::c_int) << resolutionFactor)) as COORD;
    t.baseline.y = y as COORD;
    t.align = TEXT_ALIGN::ALIGN_LEFT;
    t.CharCount = !(0 as ::core::ffi::c_int) as COUNT;
    t.pStr = &raw mut buf as *mut ::core::ffi::c_char;
    uqm_lua_checked!(font_DrawText(&raw mut t), ());
}
unsafe extern "C" fn adjustDeadzone(mut self_0: *mut WIDGET_SLIDER) {
    let mut deadzone: ::core::ffi::c_int = sliderToDeadzone((*self_0).value);
    match self_0.offset_from(&raw mut sliders as *mut WIDGET_SLIDER) {
        5 => {
            DeadZoneLeftStick[0usize] = deadzone;
        }
        6 => {
            DeadZoneRightStick[0usize] = deadzone;
        }
        7 => {
            DeadZoneLeftStick[1usize] = deadzone;
        }
        8 => {
            DeadZoneRightStick[1usize] = deadzone;
        }
        _ => {}
    };
}
unsafe extern "C" fn rebind_control(mut widget: *mut WIDGET_CONTROLENTRY) {
    let mut templat: ::core::ffi::c_int = choices[C2Rust_Unnamed_5::CHOICE_KBLAYOUT.0
            as ::core::ffi::c_int as usize]
        .selected;
    let mut control: ::core::ffi::c_int = (*widget).controlindex;
    let mut index: ::core::ffi::c_int = (*widget).highlighted;
    FlushInput();
    uqm_lua_checked!(DrawLabelAsWindow(
        (&raw mut labels as *mut WIDGET_LABEL)
            .offset(
                C2Rust_Unnamed_9::LABEL_PRESSTOEDIT.0 as ::core::ffi::c_int as isize,
            ),
        ::core::ptr::null_mut::<RECT>(),
    ), ());
    uqm_lua_checked!(RebindInputState(templat, control, index), ());
    populate_editkeys(templat);
    FlushInput();
}
unsafe extern "C" fn clear_control(mut widget: *mut WIDGET_CONTROLENTRY) {
    let mut templat: ::core::ffi::c_int = choices[C2Rust_Unnamed_5::CHOICE_KBLAYOUT.0
            as ::core::ffi::c_int as usize]
        .selected;
    let mut control: ::core::ffi::c_int = (*widget).controlindex;
    let mut index: ::core::ffi::c_int = (*widget).highlighted;
    uqm_lua_checked!(RemoveInputState(templat, control, index), ());
    populate_editkeys(templat);
}
unsafe extern "C" fn VControl_GestureEqual(
    mut a: *mut VCONTROL_GESTURE,
    mut b: *mut VCONTROL_GESTURE,
) -> BOOLEAN {
    if (*a).r#type.0 != (*b).r#type.0 {
        return BOOLEAN::FALSE;
    }
    match (*a).r#type {
        VCONTROL_GESTURE_TYPE::VCONTROL_KEY => {
            return BOOLEAN(
                ((*a).gesture.key == (*b).gesture.key) as ::core::ffi::c_int
                    as ::core::ffi::c_uint,
            );
        }
        VCONTROL_GESTURE_TYPE::VCONTROL_JOYBUTTON => {
            return BOOLEAN(
                ((*a).gesture.button.port == (*b).gesture.button.port
                    && (*a).gesture.button.index == (*b).gesture.button.index)
                    as ::core::ffi::c_int as ::core::ffi::c_uint,
            );
        }
        VCONTROL_GESTURE_TYPE::VCONTROL_JOYAXIS => {
            return BOOLEAN(
                ((*a).gesture.axis.port == (*b).gesture.axis.port
                    && (*a).gesture.axis.index == (*b).gesture.axis.index
                    && (*a).gesture.axis.polarity == (*b).gesture.axis.polarity)
                    as ::core::ffi::c_int as ::core::ffi::c_uint,
            );
        }
        _ => return BOOLEAN::FALSE,
    };
}
unsafe extern "C" fn BlacklistedBindings(
    mut action_index: ::core::ffi::c_int,
) -> BOOLEAN {
    match action_index {
        24 | 1 | 2 | 3 | 4 | 27 | 28 | 29 => return BOOLEAN::TRUE,
        _ => return BOOLEAN::FALSE,
    };
}
unsafe extern "C" fn CheckRebindConflict(
    mut action: ::core::ffi::c_int,
    mut g_compare: *mut VCONTROL_GESTURE,
    mut conflict: *mut ::core::ffi::c_char,
    mut size: size_t,
) -> BOOLEAN {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut g: *mut VCONTROL_GESTURE = ::core::ptr::null_mut::<VCONTROL_GESTURE>();
    let mut binding_special: BOOLEAN = BlacklistedBindings(action);
    i = 0 as ::core::ffi::c_int;
    while i < C2Rust_Unnamed_12::MENUCONTROL_COUNT.0 as ::core::ffi::c_int {
        let mut key_special: BOOLEAN = BOOLEAN::FALSE;
        if !((*(&raw mut menu_res_names as *mut *const ::core::ffi::c_char)
            .offset(i as isize))
            .is_null() || i == action)
        {
            key_special = BlacklistedBindings(i);
            if binding_special.0 != 0 || key_special.0 != 0 {
                j = 0 as ::core::ffi::c_int;
                while j < 6 as ::core::ffi::c_int {
                    g = (&raw mut (*(&raw mut curr_bindings as *mut MENU_BINDINGS)
                        .offset(i as isize))
                        .binding as *mut VCONTROL_GESTURE)
                        .offset(j as isize);
                    if (*g).r#type.0 != VCONTROL_GESTURE_TYPE::VCONTROL_NONE.0 {
                        if VControl_GestureEqual(g, g_compare).0 != 0 {
                            snprintf(
                                conflict,
                                size,
                                b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                                *(&raw mut menu_res_names
                                    as *mut *const ::core::ffi::c_char)
                                    .offset(i as isize),
                            );
                            return BOOLEAN::TRUE;
                        }
                    }
                    j += 1;
                }
            }
        }
        i += 1;
    }
    j = 0 as ::core::ffi::c_int;
    while j < 6 as ::core::ffi::c_int {
        if j != action {
            g = (&raw mut (*(&raw mut curr_bindings as *mut MENU_BINDINGS)
                .offset(action as isize))
                .binding as *mut VCONTROL_GESTURE)
                .offset(j as isize);
            if (*g).r#type.0 != VCONTROL_GESTURE_TYPE::VCONTROL_NONE.0 {
                if VControl_GestureEqual(g, g_compare).0 != 0 {
                    let mut category: *const ::core::ffi::c_char = menucontrols[action
                            as usize]
                        .category;
                    snprintf(
                        conflict,
                        size,
                        b"same %s\0".as_ptr() as *const ::core::ffi::c_char,
                        category,
                    );
                    return BOOLEAN::TRUE;
                }
            }
        }
        j += 1;
    }
    return BOOLEAN::FALSE;
}
unsafe extern "C" fn rebind_menu_control(mut widget: *mut WIDGET_MENUCONTROLENTRY) {
    let mut g: VCONTROL_GESTURE = VCONTROL_GESTURE {
        r#type: VCONTROL_GESTURE_TYPE::VCONTROL_NONE,
        gesture: C2Rust_Unnamed { key: 0 },
    };
    let mut existing: *mut VCONTROL_GESTURE = ::core::ptr::null_mut::<
        VCONTROL_GESTURE,
    >();
    let mut buff: [::core::ffi::c_char; 4096] = [0; 4096];
    let mut conflict: [::core::ffi::c_char; 64] = [0; 64];
    let mut bind: [::core::ffi::c_char; 32] = [0; 32];
    let mut index: ::core::ffi::c_int = (*widget).controlindex;
    let mut active: ::core::ffi::c_int = (*widget).highlighted;
    let mut page: ::core::ffi::c_int = (*widget).current_page;
    let mut slot: ::core::ffi::c_int = page * 2 as ::core::ffi::c_int + active;
    let mut vec: *mut ::core::ffi::c_int = menu_vec.offset(index as isize)
        as *mut ::core::ffi::c_int;
    FlushInput();
    uqm_lua_checked!(DrawLabelAsWindow(
        (&raw mut labels as *mut WIDGET_LABEL)
            .offset(
                C2Rust_Unnamed_9::LABEL_PRESSTOEDIT.0 as ::core::ffi::c_int as isize,
            ),
        ::core::ptr::null_mut::<RECT>(),
    ), ());
    VControl_ClearGesture();
    while VControl_GetLastGesture(&raw mut g) == 0 {
        TaskSwitch();
    }
    if g.r#type.0 != VCONTROL_GESTURE_TYPE::VCONTROL_NONE.0 {
        if CheckRebindConflict(
                index,
                &raw mut g,
                &raw mut conflict as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
            )
            .0 != 0
        {
            let mut category: *const ::core::ffi::c_char = menucontrols[index as usize]
                .category;
            InterrogateInputState(
                -1 as ::core::ffi::c_int,
                -1 as ::core::ffi::c_int,
                -1 as ::core::ffi::c_int,
                &raw mut bind as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 32]>()
                    as ::core::ffi::c_int,
                &raw mut g,
            );
            snprintf(
                &raw mut buff as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 4096]>(),
                b"Cannot bind to `%s'\n\nUsing the `%s' (%s) binding\0".as_ptr()
                    as *const ::core::ffi::c_char,
                category,
                &raw mut conflict as *mut ::core::ffi::c_char,
                &raw mut bind as *mut ::core::ffi::c_char,
            );
            uqm_lua_checked!(DoPopupWindowFont(&raw mut buff as *mut ::core::ffi::c_char, MicroFont), ());
            populate_menukeys();
            FlushInput();
            return;
        }
        if curr_bindings[index as usize].binding[slot as usize].r#type.0
            != VCONTROL_GESTURE_TYPE::VCONTROL_NONE.0
        {
            existing = (&raw mut (*(&raw mut curr_bindings as *mut MENU_BINDINGS)
                .offset(index as isize))
                .binding as *mut VCONTROL_GESTURE)
                .offset(slot as isize);
            VControl_RemoveGestureBinding(existing, vec);
        }
        curr_bindings[index as usize].binding[slot as usize] = g;
        VControl_AddGestureBinding(&raw mut g, vec);
        menu_bindings_dirty = BOOLEAN::TRUE;
    }
    populate_menukeys();
    FlushInput();
}
unsafe extern "C" fn clear_menu_control(mut widget: *mut WIDGET_MENUCONTROLENTRY) {
    let mut index: ::core::ffi::c_int = (*widget).controlindex;
    let mut active: ::core::ffi::c_int = (*widget).highlighted;
    let mut page: ::core::ffi::c_int = (*widget).current_page;
    let mut slot: ::core::ffi::c_int = page * 2 as ::core::ffi::c_int + active;
    let mut target: *mut ::core::ffi::c_int = menu_vec.offset(index as isize)
        as *mut ::core::ffi::c_int;
    let mut g: *mut VCONTROL_GESTURE = (&raw mut (*(&raw mut curr_bindings
        as *mut MENU_BINDINGS)
        .offset(index as isize))
        .binding as *mut VCONTROL_GESTURE)
        .offset(slot as isize);
    if (*g).r#type.0 != VCONTROL_GESTURE_TYPE::VCONTROL_NONE.0 {
        VControl_RemoveGestureBinding(g, target);
    }
    (*g).r#type = VCONTROL_GESTURE_TYPE::VCONTROL_NONE;
    menu_bindings_dirty = BOOLEAN::TRUE;
    populate_menukeys();
}
unsafe extern "C" fn count_widgets(mut widgets: *mut *mut WIDGET) -> ::core::ffi::c_int {
    let mut count: ::core::ffi::c_int = 0;
    count = 0 as ::core::ffi::c_int;
    while !(*widgets).is_null() {
        widgets = widgets.offset(1);
        count += 1;
    }
    return count;
}
static mut bank: *mut stringbank = ::core::ptr::null_mut::<stringbank>();
static mut setup_frame: FRAME = ::core::ptr::null_mut::<FRAME_DESC>();
unsafe extern "C" fn init_widgets() {
    let mut buffer: [*const ::core::ffi::c_char; 225] = [::core::ptr::null::<
        ::core::ffi::c_char,
    >(); 225];
    let mut str: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut title: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    let mut count: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut index: ::core::ffi::c_int = 0;
    if bank.is_null() {
        bank = StringBank_Create();
    }
    if setup_frame.is_null() || optRequiresReload.0 != 0 {
        setup_frame = CaptureDrawable(
            uqm_lua_checked!(LoadGraphicInstance(b"graphics.setupmenu\0".as_ptr() as RESOURCE), ()) as DRAWABLE,
        );
        uqm_lua_checked!(LoadArrows(), ());
    }
    count = GetStringTableCount(SetupTab) as ::core::ffi::c_int;
    if count < 3 as ::core::ffi::c_int {
        log_add(
            log_Level::log_Fatal,
            b"PANIC: Setup string table too short to even hold all indices!\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        exit(1 as ::core::ffi::c_int);
    }
    title = StringBank_AddOrFindString(
        bank,
        GetStringAddress(SetAbsStringTableIndex(SetupTab, 0 as COUNT))
            as *const ::core::ffi::c_char,
    );
    if SplitString(
        GetStringAddress(SetAbsStringTableIndex(SetupTab, 1 as COUNT))
            as *const ::core::ffi::c_char,
        '\n' as ::core::ffi::c_char,
        C2Rust_Unnamed_6::MENU_COUNT.0 as ::core::ffi::c_int
            + C2Rust_Unnamed_5::CHOICE_COUNT.0 as ::core::ffi::c_int
            + C2Rust_Unnamed_7::SLIDER_COUNT.0 as ::core::ffi::c_int
            + C2Rust_Unnamed_8::BUTTON_COUNT.0 as ::core::ffi::c_int
            + C2Rust_Unnamed_9::LABEL_COUNT.0 as ::core::ffi::c_int
            + C2Rust_Unnamed_10::TEXTENTRY_COUNT.0 as ::core::ffi::c_int
            + C2Rust_Unnamed_11::CONTROLENTRY_COUNT.0 as ::core::ffi::c_int
            + C2Rust_Unnamed_12::MENUCONTROL_COUNT.0 as ::core::ffi::c_int,
        &raw mut buffer as *mut *const ::core::ffi::c_char,
        bank,
    ) != C2Rust_Unnamed_6::MENU_COUNT.0 as ::core::ffi::c_int
    {
        log_add(
            log_Level::log_Fatal,
            b"PANIC: Incorrect number of Menu Subtitles\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        exit(1 as ::core::ffi::c_int);
    }
    i = 0 as ::core::ffi::c_int;
    while i < C2Rust_Unnamed_6::MENU_COUNT.0 as ::core::ffi::c_int {
        menus[i as usize].tag = WIDGET_TYPE::WIDGET_TYPE_MENU_SCREEN;
        menus[i as usize].parent = ::core::ptr::null_mut::<_widget>();
        menus[i as usize].handleEvent = Some(
            Widget_HandleEventMenuScreen
                as unsafe extern "C" fn(
                    *mut WIDGET,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut _widget,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
            >;
        menus[i as usize].receiveFocus = Some(
            Widget_ReceiveFocusMenuScreen
                as unsafe extern "C" fn(
                    *mut WIDGET,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut _widget,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
            >;
        menus[i as usize].draw = Some(
            Widget_DrawMenuScreen
                as unsafe extern "C" fn(
                    *mut WIDGET,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C" fn(
                    *mut _widget,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> (),
            >;
        menus[i as usize].height = Some(
            Widget_HeightFullScreen
                as unsafe extern "C" fn(*mut WIDGET) -> ::core::ffi::c_int,
        ) as Option<unsafe extern "C" fn(*mut _widget) -> ::core::ffi::c_int>;
        menus[i as usize].width = Some(
            Widget_WidthFullScreen
                as unsafe extern "C" fn(*mut WIDGET) -> ::core::ffi::c_int,
        ) as Option<unsafe extern "C" fn(*mut _widget) -> ::core::ffi::c_int>;
        menus[i as usize].title = title;
        menus[i as usize].subtitle = buffer[i as usize];
        menus[i as usize].bgStamp.origin.x = 0 as COORD;
        menus[i as usize].bgStamp.origin.y = 0 as COORD;
        menus[i as usize].bgStamp.frame = SetAbsFrameIndex(
            setup_frame,
            menu_defs[i as usize].bgIndex as COUNT,
        );
        menus[i as usize].num_children = count_widgets(menu_defs[i as usize].widgets);
        menus[i as usize].child = menu_defs[i as usize].widgets as *mut *mut _widget;
        menus[i as usize].highlighted = 0 as ::core::ffi::c_int;
        i += 1;
    }
    if !menu_defs[i as usize].widgets.is_null() {
        log_add(
            log_Level::log_Error,
            b"Menu definition array has more items!\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if SplitString(
        GetStringAddress(SetAbsStringTableIndex(SetupTab, 2 as COUNT))
            as *const ::core::ffi::c_char,
        '\n' as ::core::ffi::c_char,
        C2Rust_Unnamed_6::MENU_COUNT.0 as ::core::ffi::c_int
            + C2Rust_Unnamed_5::CHOICE_COUNT.0 as ::core::ffi::c_int
            + C2Rust_Unnamed_7::SLIDER_COUNT.0 as ::core::ffi::c_int
            + C2Rust_Unnamed_8::BUTTON_COUNT.0 as ::core::ffi::c_int
            + C2Rust_Unnamed_9::LABEL_COUNT.0 as ::core::ffi::c_int
            + C2Rust_Unnamed_10::TEXTENTRY_COUNT.0 as ::core::ffi::c_int
            + C2Rust_Unnamed_11::CONTROLENTRY_COUNT.0 as ::core::ffi::c_int
            + C2Rust_Unnamed_12::MENUCONTROL_COUNT.0 as ::core::ffi::c_int,
        &raw mut buffer as *mut *const ::core::ffi::c_char,
        bank,
    ) != C2Rust_Unnamed_5::CHOICE_COUNT.0 as ::core::ffi::c_int
    {
        log_add(
            log_Level::log_Fatal,
            b"PANIC: Incorrect number of Choice Options: %d. Should be %d\0".as_ptr()
                as *const ::core::ffi::c_char,
            C2Rust_Unnamed_5::CHOICE_COUNT.0 as ::core::ffi::c_int,
            SplitString(
                GetStringAddress(SetAbsStringTableIndex(SetupTab, 2 as COUNT))
                    as *const ::core::ffi::c_char,
                '\n' as ::core::ffi::c_char,
                C2Rust_Unnamed_6::MENU_COUNT.0 as ::core::ffi::c_int
                    + C2Rust_Unnamed_5::CHOICE_COUNT.0 as ::core::ffi::c_int
                    + C2Rust_Unnamed_7::SLIDER_COUNT.0 as ::core::ffi::c_int
                    + C2Rust_Unnamed_8::BUTTON_COUNT.0 as ::core::ffi::c_int
                    + C2Rust_Unnamed_9::LABEL_COUNT.0 as ::core::ffi::c_int
                    + C2Rust_Unnamed_10::TEXTENTRY_COUNT.0 as ::core::ffi::c_int
                    + C2Rust_Unnamed_11::CONTROLENTRY_COUNT.0 as ::core::ffi::c_int
                    + C2Rust_Unnamed_12::MENUCONTROL_COUNT.0 as ::core::ffi::c_int,
                &raw mut buffer as *mut *const ::core::ffi::c_char,
                bank,
            ),
        );
        exit(1 as ::core::ffi::c_int);
    }
    i = 0 as ::core::ffi::c_int;
    while i < C2Rust_Unnamed_5::CHOICE_COUNT.0 as ::core::ffi::c_int {
        choices[i as usize].tag = WIDGET_TYPE::WIDGET_TYPE_CHOICE;
        choices[i as usize].parent = ::core::ptr::null_mut::<_widget>();
        choices[i as usize].handleEvent = Some(
            Widget_HandleEventChoice
                as unsafe extern "C" fn(
                    *mut WIDGET,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut _widget,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
            >;
        choices[i as usize].receiveFocus = Some(
            Widget_ReceiveFocusChoice
                as unsafe extern "C" fn(
                    *mut WIDGET,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut _widget,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
            >;
        choices[i as usize].draw = Some(
            Widget_DrawChoice
                as unsafe extern "C" fn(
                    *mut WIDGET,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C" fn(
                    *mut _widget,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> (),
            >;
        choices[i as usize].height = Some(
            Widget_HeightChoice
                as unsafe extern "C" fn(*mut WIDGET) -> ::core::ffi::c_int,
        ) as Option<unsafe extern "C" fn(*mut _widget) -> ::core::ffi::c_int>;
        choices[i as usize].width = Some(
            Widget_WidthFullScreen
                as unsafe extern "C" fn(*mut WIDGET) -> ::core::ffi::c_int,
        ) as Option<unsafe extern "C" fn(*mut _widget) -> ::core::ffi::c_int>;
        choices[i as usize].category = buffer[i as usize];
        choices[i as usize].numopts = 0 as ::core::ffi::c_int;
        choices[i as usize].options = ::core::ptr::null_mut::<CHOICE_OPTION>();
        choices[i as usize].selected = 0 as ::core::ffi::c_int;
        choices[i as usize].highlighted = 0 as ::core::ffi::c_int;
        choices[i as usize].onChange = None;
        i += 1;
    }
    index = 3 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < C2Rust_Unnamed_5::CHOICE_COUNT.0 as ::core::ffi::c_int {
        let mut j: ::core::ffi::c_int = 0;
        let mut optcount: ::core::ffi::c_int = 0;
        if index >= count {
            log_add(
                log_Level::log_Fatal,
                b"PANIC: String table cut short while reading choices\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            exit(1 as ::core::ffi::c_int);
        }
        let c2rust_fresh0 = index;
        index += 1;
        str = GetStringAddress(SetAbsStringTableIndex(SetupTab, c2rust_fresh0 as COUNT))
            as *const ::core::ffi::c_char;
        optcount = SplitString(
            str,
            '\n' as ::core::ffi::c_char,
            C2Rust_Unnamed_6::MENU_COUNT.0 as ::core::ffi::c_int
                + C2Rust_Unnamed_5::CHOICE_COUNT.0 as ::core::ffi::c_int
                + C2Rust_Unnamed_7::SLIDER_COUNT.0 as ::core::ffi::c_int
                + C2Rust_Unnamed_8::BUTTON_COUNT.0 as ::core::ffi::c_int
                + C2Rust_Unnamed_9::LABEL_COUNT.0 as ::core::ffi::c_int
                + C2Rust_Unnamed_10::TEXTENTRY_COUNT.0 as ::core::ffi::c_int
                + C2Rust_Unnamed_11::CONTROLENTRY_COUNT.0 as ::core::ffi::c_int
                + C2Rust_Unnamed_12::MENUCONTROL_COUNT.0 as ::core::ffi::c_int,
            &raw mut buffer as *mut *const ::core::ffi::c_char,
            bank,
        );
        choices[i as usize].numopts = optcount;
        choices[i as usize].options = HMalloc(
            (optcount as size_t).wrapping_mul(::core::mem::size_of::<CHOICE_OPTION>()),
        ) as *mut CHOICE_OPTION;
        choices[i as usize].choice_num = i;
        j = 0 as ::core::ffi::c_int;
        while j < optcount {
            (*choices[i as usize].options.offset(j as isize)).optname = buffer[j
                as usize];
            (*choices[i as usize].options.offset(j as isize)).tooltip[0usize] = b"\0"
                .as_ptr() as *const ::core::ffi::c_char;
            (*choices[i as usize].options.offset(j as isize)).tooltip[1usize] = b"\0"
                .as_ptr() as *const ::core::ffi::c_char;
            (*choices[i as usize].options.offset(j as isize)).tooltip[2usize] = b"\0"
                .as_ptr() as *const ::core::ffi::c_char;
            j += 1;
        }
        j = 0 as ::core::ffi::c_int;
        while j < optcount {
            let mut k: ::core::ffi::c_int = 0;
            let mut tipcount: ::core::ffi::c_int = 0;
            if index >= count {
                log_add(
                    log_Level::log_Fatal,
                    b"PANIC: String table cut short while reading choices\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                exit(1 as ::core::ffi::c_int);
            }
            let c2rust_fresh1 = index;
            index += 1;
            str = GetStringAddress(
                SetAbsStringTableIndex(SetupTab, c2rust_fresh1 as COUNT),
            ) as *const ::core::ffi::c_char;
            tipcount = SplitString(
                str,
                '\n' as ::core::ffi::c_char,
                C2Rust_Unnamed_6::MENU_COUNT.0 as ::core::ffi::c_int
                    + C2Rust_Unnamed_5::CHOICE_COUNT.0 as ::core::ffi::c_int
                    + C2Rust_Unnamed_7::SLIDER_COUNT.0 as ::core::ffi::c_int
                    + C2Rust_Unnamed_8::BUTTON_COUNT.0 as ::core::ffi::c_int
                    + C2Rust_Unnamed_9::LABEL_COUNT.0 as ::core::ffi::c_int
                    + C2Rust_Unnamed_10::TEXTENTRY_COUNT.0 as ::core::ffi::c_int
                    + C2Rust_Unnamed_11::CONTROLENTRY_COUNT.0 as ::core::ffi::c_int
                    + C2Rust_Unnamed_12::MENUCONTROL_COUNT.0 as ::core::ffi::c_int,
                &raw mut buffer as *mut *const ::core::ffi::c_char,
                bank,
            );
            if tipcount > 3 as ::core::ffi::c_int {
                tipcount = 3 as ::core::ffi::c_int;
            }
            k = 0 as ::core::ffi::c_int;
            while k < tipcount {
                (*choices[i as usize].options.offset(j as isize)).tooltip[k as usize] = buffer[k
                    as usize];
                k += 1;
            }
            j += 1;
        }
        i += 1;
    }
    choices[C2Rust_Unnamed_5::CHOICE_KBLAYOUT.0 as ::core::ffi::c_int as usize]
        .onChange = Some(
        change_template
            as unsafe extern "C" fn(*mut WIDGET_CHOICE, ::core::ffi::c_int) -> (),
    ) as Option<unsafe extern "C" fn(*mut _widget_choice, ::core::ffi::c_int) -> ()>;
    choices[C2Rust_Unnamed_5::CHOICE_GAMESEED.0 as ::core::ffi::c_int as usize]
        .onChange = Some(
        change_seedtype
            as unsafe extern "C" fn(*mut WIDGET_CHOICE, ::core::ffi::c_int) -> (),
    ) as Option<unsafe extern "C" fn(*mut _widget_choice, ::core::ffi::c_int) -> ()>;
    choices[C2Rust_Unnamed_5::CHOICE_GRAPHICS.0 as ::core::ffi::c_int as usize]
        .onChange = Some(
        check_availability
            as unsafe extern "C" fn(*mut WIDGET_CHOICE, ::core::ffi::c_int) -> (),
    ) as Option<unsafe extern "C" fn(*mut _widget_choice, ::core::ffi::c_int) -> ()>;
    choices[C2Rust_Unnamed_5::CHOICE_REMIXES1.0 as ::core::ffi::c_int as usize]
        .onChange = Some(
        check_availability
            as unsafe extern "C" fn(*mut WIDGET_CHOICE, ::core::ffi::c_int) -> (),
    ) as Option<unsafe extern "C" fn(*mut _widget_choice, ::core::ffi::c_int) -> ()>;
    choices[C2Rust_Unnamed_5::CHOICE_REMIXES2.0 as ::core::ffi::c_int as usize]
        .onChange = Some(
        check_availability
            as unsafe extern "C" fn(*mut WIDGET_CHOICE, ::core::ffi::c_int) -> (),
    ) as Option<unsafe extern "C" fn(*mut _widget_choice, ::core::ffi::c_int) -> ()>;
    choices[C2Rust_Unnamed_5::CHOICE_IPMUSIC.0 as ::core::ffi::c_int as usize]
        .onChange = Some(
        check_availability
            as unsafe extern "C" fn(*mut WIDGET_CHOICE, ::core::ffi::c_int) -> (),
    ) as Option<unsafe extern "C" fn(*mut _widget_choice, ::core::ffi::c_int) -> ()>;
    choices[C2Rust_Unnamed_5::CHOICE_REMIXES3.0 as ::core::ffi::c_int as usize]
        .onChange = Some(
        check_availability
            as unsafe extern "C" fn(*mut WIDGET_CHOICE, ::core::ffi::c_int) -> (),
    ) as Option<unsafe extern "C" fn(*mut _widget_choice, ::core::ffi::c_int) -> ()>;
    choices[C2Rust_Unnamed_5::CHOICE_WINDOWTYPE.0 as ::core::ffi::c_int as usize]
        .onChange = Some(
        check_availability
            as unsafe extern "C" fn(*mut WIDGET_CHOICE, ::core::ffi::c_int) -> (),
    ) as Option<unsafe extern "C" fn(*mut _widget_choice, ::core::ffi::c_int) -> ()>;
    choices[C2Rust_Unnamed_5::CHOICE_FRBUFFER.0 as ::core::ffi::c_int as usize]
        .onChange = Some(
        process_graphics_options
            as unsafe extern "C" fn(*mut WIDGET_CHOICE, ::core::ffi::c_int) -> (),
    ) as Option<unsafe extern "C" fn(*mut _widget_choice, ::core::ffi::c_int) -> ()>;
    choices[C2Rust_Unnamed_5::CHOICE_SCALER.0 as ::core::ffi::c_int as usize].onChange = Some(
        process_graphics_options
            as unsafe extern "C" fn(*mut WIDGET_CHOICE, ::core::ffi::c_int) -> (),
    ) as Option<unsafe extern "C" fn(*mut _widget_choice, ::core::ffi::c_int) -> ()>;
    choices[C2Rust_Unnamed_5::CHOICE_SCANLINE.0 as ::core::ffi::c_int as usize]
        .onChange = Some(
        process_graphics_options
            as unsafe extern "C" fn(*mut WIDGET_CHOICE, ::core::ffi::c_int) -> (),
    ) as Option<unsafe extern "C" fn(*mut _widget_choice, ::core::ffi::c_int) -> ()>;
    choices[C2Rust_Unnamed_5::CHOICE_DISPLAY.0 as ::core::ffi::c_int as usize]
        .onChange = Some(
        process_graphics_options
            as unsafe extern "C" fn(*mut WIDGET_CHOICE, ::core::ffi::c_int) -> (),
    ) as Option<unsafe extern "C" fn(*mut _widget_choice, ::core::ffi::c_int) -> ()>;
    choices[C2Rust_Unnamed_5::CHOICE_SHOWFPS.0 as ::core::ffi::c_int as usize]
        .onChange = Some(
        process_graphics_options
            as unsafe extern "C" fn(*mut WIDGET_CHOICE, ::core::ffi::c_int) -> (),
    ) as Option<unsafe extern "C" fn(*mut _widget_choice, ::core::ffi::c_int) -> ()>;
    choices[C2Rust_Unnamed_5::CHOICE_ASPRATIO.0 as ::core::ffi::c_int as usize]
        .onChange = Some(
        process_graphics_options
            as unsafe extern "C" fn(*mut WIDGET_CHOICE, ::core::ffi::c_int) -> (),
    ) as Option<unsafe extern "C" fn(*mut _widget_choice, ::core::ffi::c_int) -> ()>;
    choices[C2Rust_Unnamed_5::CHOICE_RESOLUTION.0 as ::core::ffi::c_int as usize]
        .onChange = Some(
        process_graphics_options
            as unsafe extern "C" fn(*mut WIDGET_CHOICE, ::core::ffi::c_int) -> (),
    ) as Option<unsafe extern "C" fn(*mut _widget_choice, ::core::ffi::c_int) -> ()>;
    if index >= count {
        log_add(
            log_Level::log_Fatal,
            b"PANIC: String table cut short while reading sliders\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        exit(1 as ::core::ffi::c_int);
    }
    let c2rust_fresh2 = index;
    index += 1;
    if SplitString(
        GetStringAddress(SetAbsStringTableIndex(SetupTab, c2rust_fresh2 as COUNT))
            as *const ::core::ffi::c_char,
        '\n' as ::core::ffi::c_char,
        C2Rust_Unnamed_6::MENU_COUNT.0 as ::core::ffi::c_int
            + C2Rust_Unnamed_5::CHOICE_COUNT.0 as ::core::ffi::c_int
            + C2Rust_Unnamed_7::SLIDER_COUNT.0 as ::core::ffi::c_int
            + C2Rust_Unnamed_8::BUTTON_COUNT.0 as ::core::ffi::c_int
            + C2Rust_Unnamed_9::LABEL_COUNT.0 as ::core::ffi::c_int
            + C2Rust_Unnamed_10::TEXTENTRY_COUNT.0 as ::core::ffi::c_int
            + C2Rust_Unnamed_11::CONTROLENTRY_COUNT.0 as ::core::ffi::c_int
            + C2Rust_Unnamed_12::MENUCONTROL_COUNT.0 as ::core::ffi::c_int,
        &raw mut buffer as *mut *const ::core::ffi::c_char,
        bank,
    ) != C2Rust_Unnamed_7::SLIDER_COUNT.0 as ::core::ffi::c_int
    {
        log_add(
            log_Level::log_Fatal,
            b"PANIC: Incorrect number of Slider Options\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        exit(1 as ::core::ffi::c_int);
    }
    i = 0 as ::core::ffi::c_int;
    while i < C2Rust_Unnamed_7::SLIDER_COUNT.0 as ::core::ffi::c_int {
        sliders[i as usize].tag = WIDGET_TYPE::WIDGET_TYPE_SLIDER;
        sliders[i as usize].parent = ::core::ptr::null_mut::<_widget>();
        sliders[i as usize].handleEvent = Some(
            Widget_HandleEventSlider
                as unsafe extern "C" fn(
                    *mut WIDGET,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut _widget,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
            >;
        sliders[i as usize].receiveFocus = Some(
            Widget_ReceiveFocusSimple
                as unsafe extern "C" fn(
                    *mut WIDGET,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut _widget,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
            >;
        sliders[i as usize].draw = Some(
            Widget_DrawSlider
                as unsafe extern "C" fn(
                    *mut WIDGET,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C" fn(
                    *mut _widget,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> (),
            >;
        sliders[i as usize].height = Some(
            Widget_HeightOneLine
                as unsafe extern "C" fn(*mut WIDGET) -> ::core::ffi::c_int,
        ) as Option<unsafe extern "C" fn(*mut _widget) -> ::core::ffi::c_int>;
        sliders[i as usize].width = Some(
            Widget_WidthFullScreen
                as unsafe extern "C" fn(*mut WIDGET) -> ::core::ffi::c_int,
        ) as Option<unsafe extern "C" fn(*mut _widget) -> ::core::ffi::c_int>;
        sliders[i as usize].draw_value = Some(
            Widget_Slider_DrawValue
                as unsafe extern "C" fn(
                    *mut WIDGET_SLIDER,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C" fn(
                    *mut _widget_slider,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> (),
            >;
        sliders[i as usize].min = 0 as ::core::ffi::c_int;
        sliders[i as usize].max = 100 as ::core::ffi::c_int;
        sliders[i as usize].step = 5 as ::core::ffi::c_int;
        sliders[i as usize].value = 75 as ::core::ffi::c_int;
        sliders[i as usize].category = buffer[i as usize];
        sliders[i as usize].tooltip[0usize] = b"\0".as_ptr()
            as *const ::core::ffi::c_char;
        sliders[i as usize].tooltip[1usize] = b"\0".as_ptr()
            as *const ::core::ffi::c_char;
        sliders[i as usize].tooltip[2usize] = b"\0".as_ptr()
            as *const ::core::ffi::c_char;
        sliders[i as usize].onChange = None;
        i += 1;
    }
    sliders[C2Rust_Unnamed_7::SLIDER_MUSVOLUME.0 as ::core::ffi::c_int as usize]
        .onChange = Some(adjustMusic as unsafe extern "C" fn(*mut WIDGET_SLIDER) -> ())
        as Option<unsafe extern "C" fn(*mut _widget_slider) -> ()>;
    sliders[C2Rust_Unnamed_7::SLIDER_SFXVOLUME.0 as ::core::ffi::c_int as usize]
        .onChange = Some(adjustSFX as unsafe extern "C" fn(*mut WIDGET_SLIDER) -> ())
        as Option<unsafe extern "C" fn(*mut _widget_slider) -> ()>;
    sliders[C2Rust_Unnamed_7::SLIDER_SPCHVOLUME.0 as ::core::ffi::c_int as usize]
        .onChange = Some(adjustSpeech as unsafe extern "C" fn(*mut WIDGET_SLIDER) -> ())
        as Option<unsafe extern "C" fn(*mut _widget_slider) -> ()>;
    sliders[C2Rust_Unnamed_7::SLIDER_GAMMA.0 as ::core::ffi::c_int as usize].step = 1
        as ::core::ffi::c_int;
    sliders[C2Rust_Unnamed_7::SLIDER_GAMMA.0 as ::core::ffi::c_int as usize]
        .handleEvent = Some(
        gamma_HandleEventSlider
            as unsafe extern "C" fn(
                *mut WIDGET,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
    )
        as Option<
            unsafe extern "C" fn(*mut _widget, ::core::ffi::c_int) -> ::core::ffi::c_int,
        >;
    sliders[C2Rust_Unnamed_7::SLIDER_GAMMA.0 as ::core::ffi::c_int as usize]
        .draw_value = Some(
        gamma_DrawValue
            as unsafe extern "C" fn(
                *mut WIDGET_SLIDER,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *mut _widget_slider,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
            ) -> (),
        >;
    sliders[C2Rust_Unnamed_7::SLIDER_NEBULA.0 as ::core::ffi::c_int as usize].step = 1
        as ::core::ffi::c_int;
    sliders[C2Rust_Unnamed_7::SLIDER_NEBULA.0 as ::core::ffi::c_int as usize].max = 50
        as ::core::ffi::c_int;
    let mut j_0: ::core::ffi::c_int = 0;
    j_0 = C2Rust_Unnamed_7::SLIDER_DEADZONE_00.0 as ::core::ffi::c_int;
    while j_0 <= C2Rust_Unnamed_7::SLIDER_DEADZONE_03.0 as ::core::ffi::c_int {
        sliders[j_0 as usize].step = 1 as ::core::ffi::c_int;
        sliders[j_0 as usize].max = 100 as ::core::ffi::c_int;
        sliders[j_0 as usize].draw_value = Some(
            deadzone_DrawValue
                as unsafe extern "C" fn(
                    *mut WIDGET_SLIDER,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C" fn(
                    *mut _widget_slider,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> (),
            >;
        sliders[j_0 as usize].onChange = Some(
            adjustDeadzone as unsafe extern "C" fn(*mut WIDGET_SLIDER) -> (),
        ) as Option<unsafe extern "C" fn(*mut _widget_slider) -> ()>;
        j_0 += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < C2Rust_Unnamed_7::SLIDER_COUNT.0 as ::core::ffi::c_int {
        let mut j_1: ::core::ffi::c_int = 0;
        let mut tipcount_0: ::core::ffi::c_int = 0;
        if index >= count {
            log_add(
                log_Level::log_Fatal,
                b"PANIC: String table cut short while reading sliders\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            exit(1 as ::core::ffi::c_int);
        }
        let c2rust_fresh3 = index;
        index += 1;
        str = GetStringAddress(SetAbsStringTableIndex(SetupTab, c2rust_fresh3 as COUNT))
            as *const ::core::ffi::c_char;
        tipcount_0 = SplitString(
            str,
            '\n' as ::core::ffi::c_char,
            C2Rust_Unnamed_6::MENU_COUNT.0 as ::core::ffi::c_int
                + C2Rust_Unnamed_5::CHOICE_COUNT.0 as ::core::ffi::c_int
                + C2Rust_Unnamed_7::SLIDER_COUNT.0 as ::core::ffi::c_int
                + C2Rust_Unnamed_8::BUTTON_COUNT.0 as ::core::ffi::c_int
                + C2Rust_Unnamed_9::LABEL_COUNT.0 as ::core::ffi::c_int
                + C2Rust_Unnamed_10::TEXTENTRY_COUNT.0 as ::core::ffi::c_int
                + C2Rust_Unnamed_11::CONTROLENTRY_COUNT.0 as ::core::ffi::c_int
                + C2Rust_Unnamed_12::MENUCONTROL_COUNT.0 as ::core::ffi::c_int,
            &raw mut buffer as *mut *const ::core::ffi::c_char,
            bank,
        );
        if tipcount_0 > 3 as ::core::ffi::c_int {
            tipcount_0 = 3 as ::core::ffi::c_int;
        }
        j_1 = 0 as ::core::ffi::c_int;
        while j_1 < tipcount_0 {
            sliders[i as usize].tooltip[j_1 as usize] = buffer[j_1 as usize];
            j_1 += 1;
        }
        i += 1;
    }
    if index >= count {
        log_add(
            log_Level::log_Fatal,
            b"PANIC: String table cut short while reading buttons\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        exit(1 as ::core::ffi::c_int);
    }
    let c2rust_fresh4 = index;
    index += 1;
    if SplitString(
        GetStringAddress(SetAbsStringTableIndex(SetupTab, c2rust_fresh4 as COUNT))
            as *const ::core::ffi::c_char,
        '\n' as ::core::ffi::c_char,
        C2Rust_Unnamed_6::MENU_COUNT.0 as ::core::ffi::c_int
            + C2Rust_Unnamed_5::CHOICE_COUNT.0 as ::core::ffi::c_int
            + C2Rust_Unnamed_7::SLIDER_COUNT.0 as ::core::ffi::c_int
            + C2Rust_Unnamed_8::BUTTON_COUNT.0 as ::core::ffi::c_int
            + C2Rust_Unnamed_9::LABEL_COUNT.0 as ::core::ffi::c_int
            + C2Rust_Unnamed_10::TEXTENTRY_COUNT.0 as ::core::ffi::c_int
            + C2Rust_Unnamed_11::CONTROLENTRY_COUNT.0 as ::core::ffi::c_int
            + C2Rust_Unnamed_12::MENUCONTROL_COUNT.0 as ::core::ffi::c_int,
        &raw mut buffer as *mut *const ::core::ffi::c_char,
        bank,
    ) != C2Rust_Unnamed_8::BUTTON_COUNT.0 as ::core::ffi::c_int
    {
        log_add(
            log_Level::log_Fatal,
            b"PANIC: Incorrect number of Button Options\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        exit(1 as ::core::ffi::c_int);
    }
    i = 0 as ::core::ffi::c_int;
    while i < C2Rust_Unnamed_8::BUTTON_COUNT.0 as ::core::ffi::c_int {
        buttons[i as usize].tag = WIDGET_TYPE::WIDGET_TYPE_BUTTON;
        buttons[i as usize].parent = ::core::ptr::null_mut::<_widget>();
        buttons[i as usize].handleEvent = button_handlers[i as usize]
            as Option<
                unsafe extern "C" fn(
                    *mut _widget,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
            >;
        buttons[i as usize].receiveFocus = Some(
            Widget_ReceiveFocusSimple
                as unsafe extern "C" fn(
                    *mut WIDGET,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut _widget,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
            >;
        buttons[i as usize].draw = Some(
            Widget_DrawButton
                as unsafe extern "C" fn(
                    *mut WIDGET,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C" fn(
                    *mut _widget,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> (),
            >;
        buttons[i as usize].height = Some(
            Widget_HeightOneLine
                as unsafe extern "C" fn(*mut WIDGET) -> ::core::ffi::c_int,
        ) as Option<unsafe extern "C" fn(*mut _widget) -> ::core::ffi::c_int>;
        buttons[i as usize].width = Some(
            Widget_WidthFullScreen
                as unsafe extern "C" fn(*mut WIDGET) -> ::core::ffi::c_int,
        ) as Option<unsafe extern "C" fn(*mut _widget) -> ::core::ffi::c_int>;
        buttons[i as usize].name = buffer[i as usize];
        buttons[i as usize].tooltip[0usize] = b"\0".as_ptr()
            as *const ::core::ffi::c_char;
        buttons[i as usize].tooltip[1usize] = b"\0".as_ptr()
            as *const ::core::ffi::c_char;
        buttons[i as usize].tooltip[2usize] = b"\0".as_ptr()
            as *const ::core::ffi::c_char;
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < C2Rust_Unnamed_8::BUTTON_COUNT.0 as ::core::ffi::c_int {
        let mut j_2: ::core::ffi::c_int = 0;
        let mut tipcount_1: ::core::ffi::c_int = 0;
        if index >= count {
            log_add(
                log_Level::log_Fatal,
                b"PANIC: String table cut short while reading buttons\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            exit(1 as ::core::ffi::c_int);
        }
        let c2rust_fresh5 = index;
        index += 1;
        str = GetStringAddress(SetAbsStringTableIndex(SetupTab, c2rust_fresh5 as COUNT))
            as *const ::core::ffi::c_char;
        tipcount_1 = SplitString(
            str,
            '\n' as ::core::ffi::c_char,
            C2Rust_Unnamed_6::MENU_COUNT.0 as ::core::ffi::c_int
                + C2Rust_Unnamed_5::CHOICE_COUNT.0 as ::core::ffi::c_int
                + C2Rust_Unnamed_7::SLIDER_COUNT.0 as ::core::ffi::c_int
                + C2Rust_Unnamed_8::BUTTON_COUNT.0 as ::core::ffi::c_int
                + C2Rust_Unnamed_9::LABEL_COUNT.0 as ::core::ffi::c_int
                + C2Rust_Unnamed_10::TEXTENTRY_COUNT.0 as ::core::ffi::c_int
                + C2Rust_Unnamed_11::CONTROLENTRY_COUNT.0 as ::core::ffi::c_int
                + C2Rust_Unnamed_12::MENUCONTROL_COUNT.0 as ::core::ffi::c_int,
            &raw mut buffer as *mut *const ::core::ffi::c_char,
            bank,
        );
        if tipcount_1 > 3 as ::core::ffi::c_int {
            tipcount_1 = 3 as ::core::ffi::c_int;
        }
        j_2 = 0 as ::core::ffi::c_int;
        while j_2 < tipcount_1 {
            buttons[i as usize].tooltip[j_2 as usize] = buffer[j_2 as usize];
            j_2 += 1;
        }
        i += 1;
    }
    if index >= count {
        log_add(
            log_Level::log_Fatal,
            b"PANIC: String table cut short while reading labels\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        exit(1 as ::core::ffi::c_int);
    }
    let c2rust_fresh6 = index;
    index += 1;
    if SplitString(
        GetStringAddress(SetAbsStringTableIndex(SetupTab, c2rust_fresh6 as COUNT))
            as *const ::core::ffi::c_char,
        '\n' as ::core::ffi::c_char,
        C2Rust_Unnamed_6::MENU_COUNT.0 as ::core::ffi::c_int
            + C2Rust_Unnamed_5::CHOICE_COUNT.0 as ::core::ffi::c_int
            + C2Rust_Unnamed_7::SLIDER_COUNT.0 as ::core::ffi::c_int
            + C2Rust_Unnamed_8::BUTTON_COUNT.0 as ::core::ffi::c_int
            + C2Rust_Unnamed_9::LABEL_COUNT.0 as ::core::ffi::c_int
            + C2Rust_Unnamed_10::TEXTENTRY_COUNT.0 as ::core::ffi::c_int
            + C2Rust_Unnamed_11::CONTROLENTRY_COUNT.0 as ::core::ffi::c_int
            + C2Rust_Unnamed_12::MENUCONTROL_COUNT.0 as ::core::ffi::c_int,
        &raw mut buffer as *mut *const ::core::ffi::c_char,
        bank,
    ) != C2Rust_Unnamed_9::LABEL_COUNT.0 as ::core::ffi::c_int
    {
        log_add(
            log_Level::log_Fatal,
            b"PANIC: Incorrect number of Label Options\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        exit(1 as ::core::ffi::c_int);
    }
    i = 0 as ::core::ffi::c_int;
    while i < C2Rust_Unnamed_9::LABEL_COUNT.0 as ::core::ffi::c_int {
        labels[i as usize].tag = WIDGET_TYPE::WIDGET_TYPE_LABEL;
        labels[i as usize].parent = ::core::ptr::null_mut::<_widget>();
        labels[i as usize].handleEvent = Some(
            Widget_HandleEventIgnoreAll
                as unsafe extern "C" fn(
                    *mut WIDGET,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut _widget,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
            >;
        labels[i as usize].receiveFocus = Some(
            Widget_ReceiveFocusRefuseFocus
                as unsafe extern "C" fn(
                    *mut WIDGET,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut _widget,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
            >;
        labels[i as usize].draw = Some(
            Widget_DrawLabel
                as unsafe extern "C" fn(
                    *mut WIDGET,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C" fn(
                    *mut _widget,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> (),
            >;
        labels[i as usize].height = Some(
            Widget_HeightLabel as unsafe extern "C" fn(*mut WIDGET) -> ::core::ffi::c_int,
        ) as Option<unsafe extern "C" fn(*mut _widget) -> ::core::ffi::c_int>;
        labels[i as usize].width = Some(
            Widget_WidthFullScreen
                as unsafe extern "C" fn(*mut WIDGET) -> ::core::ffi::c_int,
        ) as Option<unsafe extern "C" fn(*mut _widget) -> ::core::ffi::c_int>;
        labels[i as usize].line_count = 0 as ::core::ffi::c_int;
        labels[i as usize].lines = ::core::ptr::null_mut::<*const ::core::ffi::c_char>();
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < C2Rust_Unnamed_9::LABEL_COUNT.0 as ::core::ffi::c_int {
        let mut j_3: ::core::ffi::c_int = 0;
        let mut linecount: ::core::ffi::c_int = 0;
        if index >= count {
            log_add(
                log_Level::log_Fatal,
                b"PANIC: String table cut short while reading labels\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            exit(1 as ::core::ffi::c_int);
        }
        let c2rust_fresh7 = index;
        index += 1;
        str = GetStringAddress(SetAbsStringTableIndex(SetupTab, c2rust_fresh7 as COUNT))
            as *const ::core::ffi::c_char;
        linecount = SplitString(
            str,
            '\n' as ::core::ffi::c_char,
            C2Rust_Unnamed_6::MENU_COUNT.0 as ::core::ffi::c_int
                + C2Rust_Unnamed_5::CHOICE_COUNT.0 as ::core::ffi::c_int
                + C2Rust_Unnamed_7::SLIDER_COUNT.0 as ::core::ffi::c_int
                + C2Rust_Unnamed_8::BUTTON_COUNT.0 as ::core::ffi::c_int
                + C2Rust_Unnamed_9::LABEL_COUNT.0 as ::core::ffi::c_int
                + C2Rust_Unnamed_10::TEXTENTRY_COUNT.0 as ::core::ffi::c_int
                + C2Rust_Unnamed_11::CONTROLENTRY_COUNT.0 as ::core::ffi::c_int
                + C2Rust_Unnamed_12::MENUCONTROL_COUNT.0 as ::core::ffi::c_int,
            &raw mut buffer as *mut *const ::core::ffi::c_char,
            bank,
        );
        labels[i as usize].line_count = linecount;
        labels[i as usize].lines = HMalloc(
            (linecount as size_t)
                .wrapping_mul(::core::mem::size_of::<*const ::core::ffi::c_char>()),
        ) as *mut *const ::core::ffi::c_char;
        j_3 = 0 as ::core::ffi::c_int;
        while j_3 < linecount {
            *labels[i as usize].lines.offset(j_3 as isize) = buffer[j_3 as usize];
            j_3 += 1;
        }
        i += 1;
    }
    if index >= count {
        log_add(
            log_Level::log_Fatal,
            b"PANIC: String table cut short while reading text entries\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        exit(1 as ::core::ffi::c_int);
    }
    let c2rust_fresh8 = index;
    index += 1;
    if SplitString(
        GetStringAddress(SetAbsStringTableIndex(SetupTab, c2rust_fresh8 as COUNT))
            as *const ::core::ffi::c_char,
        '\n' as ::core::ffi::c_char,
        C2Rust_Unnamed_6::MENU_COUNT.0 as ::core::ffi::c_int
            + C2Rust_Unnamed_5::CHOICE_COUNT.0 as ::core::ffi::c_int
            + C2Rust_Unnamed_7::SLIDER_COUNT.0 as ::core::ffi::c_int
            + C2Rust_Unnamed_8::BUTTON_COUNT.0 as ::core::ffi::c_int
            + C2Rust_Unnamed_9::LABEL_COUNT.0 as ::core::ffi::c_int
            + C2Rust_Unnamed_10::TEXTENTRY_COUNT.0 as ::core::ffi::c_int
            + C2Rust_Unnamed_11::CONTROLENTRY_COUNT.0 as ::core::ffi::c_int
            + C2Rust_Unnamed_12::MENUCONTROL_COUNT.0 as ::core::ffi::c_int,
        &raw mut buffer as *mut *const ::core::ffi::c_char,
        bank,
    ) != C2Rust_Unnamed_10::TEXTENTRY_COUNT.0 as ::core::ffi::c_int
    {
        log_add(
            log_Level::log_Fatal,
            b"PANIC: Incorrect number of Text Entries\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        exit(1 as ::core::ffi::c_int);
    }
    i = 0 as ::core::ffi::c_int;
    while i < C2Rust_Unnamed_10::TEXTENTRY_COUNT.0 as ::core::ffi::c_int {
        textentries[i as usize].tag = WIDGET_TYPE::WIDGET_TYPE_TEXTENTRY;
        textentries[i as usize].parent = ::core::ptr::null_mut::<_widget>();
        textentries[i as usize].handleEvent = Some(
            Widget_HandleEventTextEntry
                as unsafe extern "C" fn(
                    *mut WIDGET,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut _widget,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
            >;
        textentries[i as usize].receiveFocus = Some(
            Widget_ReceiveFocusSimple
                as unsafe extern "C" fn(
                    *mut WIDGET,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut _widget,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
            >;
        textentries[i as usize].draw = Some(
            Widget_DrawTextEntry
                as unsafe extern "C" fn(
                    *mut WIDGET,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C" fn(
                    *mut _widget,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> (),
            >;
        textentries[i as usize].height = Some(
            Widget_HeightOneLine
                as unsafe extern "C" fn(*mut WIDGET) -> ::core::ffi::c_int,
        ) as Option<unsafe extern "C" fn(*mut _widget) -> ::core::ffi::c_int>;
        textentries[i as usize].width = Some(
            Widget_WidthFullScreen
                as unsafe extern "C" fn(*mut WIDGET) -> ::core::ffi::c_int,
        ) as Option<unsafe extern "C" fn(*mut _widget) -> ::core::ffi::c_int>;
        textentries[i as usize].handleEventSelect = Some(
            OnTextEntryEvent
                as unsafe extern "C" fn(*mut WIDGET_TEXTENTRY) -> ::core::ffi::c_int,
        ) as Option<unsafe extern "C" fn(*mut _widget_textentry) -> ::core::ffi::c_int>;
        textentries[i as usize].category = buffer[i as usize];
        textentries[i as usize].value[0usize] = 0 as ::core::ffi::c_char;
        textentries[i as usize].maxlen = 31 as ::core::ffi::c_int
            - 1 as ::core::ffi::c_int;
        textentries[i as usize].state = WIDGET_TEXTENTRY_STATE::WTE_NORMAL;
        textentries[i as usize].cursor_pos = 0 as ::core::ffi::c_int;
        textentries[i as usize].tooltip[0usize] = b"\0".as_ptr()
            as *const ::core::ffi::c_char;
        textentries[i as usize].tooltip[1usize] = b"\0".as_ptr()
            as *const ::core::ffi::c_char;
        textentries[i as usize].tooltip[2usize] = b"\0".as_ptr()
            as *const ::core::ffi::c_char;
        i += 1;
    }
    let c2rust_fresh9 = index;
    index += 1;
    if SplitString(
        GetStringAddress(SetAbsStringTableIndex(SetupTab, c2rust_fresh9 as COUNT))
            as *const ::core::ffi::c_char,
        '\n' as ::core::ffi::c_char,
        C2Rust_Unnamed_6::MENU_COUNT.0 as ::core::ffi::c_int
            + C2Rust_Unnamed_5::CHOICE_COUNT.0 as ::core::ffi::c_int
            + C2Rust_Unnamed_7::SLIDER_COUNT.0 as ::core::ffi::c_int
            + C2Rust_Unnamed_8::BUTTON_COUNT.0 as ::core::ffi::c_int
            + C2Rust_Unnamed_9::LABEL_COUNT.0 as ::core::ffi::c_int
            + C2Rust_Unnamed_10::TEXTENTRY_COUNT.0 as ::core::ffi::c_int
            + C2Rust_Unnamed_11::CONTROLENTRY_COUNT.0 as ::core::ffi::c_int
            + C2Rust_Unnamed_12::MENUCONTROL_COUNT.0 as ::core::ffi::c_int,
        &raw mut buffer as *mut *const ::core::ffi::c_char,
        bank,
    ) != C2Rust_Unnamed_10::TEXTENTRY_COUNT.0 as ::core::ffi::c_int
    {
        log_add(
            log_Level::log_Fatal,
            b"PANIC: Incorrect number of Text Entries\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        exit(1 as ::core::ffi::c_int);
    }
    i = 0 as ::core::ffi::c_int;
    while i < C2Rust_Unnamed_10::TEXTENTRY_COUNT.0 as ::core::ffi::c_int {
        let mut j_4: ::core::ffi::c_int = 0;
        let mut tipcount_2: ::core::ffi::c_int = 0;
        strncpy(
            &raw mut (*(&raw mut textentries as *mut WIDGET_TEXTENTRY)
                .offset(i as isize))
                .value as *mut ::core::ffi::c_char,
            buffer[i as usize],
            textentries[i as usize].maxlen as size_t,
        );
        textentries[i as usize].value[textentries[i as usize].maxlen as usize] = 0
            as ::core::ffi::c_char;
        if index >= count {
            log_add(
                log_Level::log_Fatal,
                b"PANIC: String table cut short while reading text entries\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            exit(1 as ::core::ffi::c_int);
        }
        let c2rust_fresh10 = index;
        index += 1;
        str = GetStringAddress(SetAbsStringTableIndex(SetupTab, c2rust_fresh10 as COUNT))
            as *const ::core::ffi::c_char;
        tipcount_2 = SplitString(
            str,
            '\n' as ::core::ffi::c_char,
            C2Rust_Unnamed_6::MENU_COUNT.0 as ::core::ffi::c_int
                + C2Rust_Unnamed_5::CHOICE_COUNT.0 as ::core::ffi::c_int
                + C2Rust_Unnamed_7::SLIDER_COUNT.0 as ::core::ffi::c_int
                + C2Rust_Unnamed_8::BUTTON_COUNT.0 as ::core::ffi::c_int
                + C2Rust_Unnamed_9::LABEL_COUNT.0 as ::core::ffi::c_int
                + C2Rust_Unnamed_10::TEXTENTRY_COUNT.0 as ::core::ffi::c_int
                + C2Rust_Unnamed_11::CONTROLENTRY_COUNT.0 as ::core::ffi::c_int
                + C2Rust_Unnamed_12::MENUCONTROL_COUNT.0 as ::core::ffi::c_int,
            &raw mut buffer as *mut *const ::core::ffi::c_char,
            bank,
        );
        if tipcount_2 > 3 as ::core::ffi::c_int {
            tipcount_2 = 3 as ::core::ffi::c_int;
        }
        j_4 = 0 as ::core::ffi::c_int;
        while j_4 < tipcount_2 {
            textentries[i as usize].tooltip[j_4 as usize] = buffer[j_4 as usize];
            j_4 += 1;
        }
        i += 1;
    }
    textentries[C2Rust_Unnamed_10::TEXT_GAMESEED.0 as ::core::ffi::c_int as usize]
        .onChange = Some(
        change_seed as unsafe extern "C" fn(*mut WIDGET_TEXTENTRY) -> (),
    ) as Option<unsafe extern "C" fn(*mut _widget_textentry) -> ()>;
    textentries[C2Rust_Unnamed_10::TEXT_CUSTMRES.0 as ::core::ffi::c_int as usize]
        .onChange = Some(change_res as unsafe extern "C" fn(*mut WIDGET_TEXTENTRY) -> ())
        as Option<unsafe extern "C" fn(*mut _widget_textentry) -> ()>;
    if index >= count {
        log_add(
            log_Level::log_Fatal,
            b"PANIC: String table cut short while reading control entries\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        exit(1 as ::core::ffi::c_int);
    }
    let c2rust_fresh11 = index;
    index += 1;
    if SplitString(
        GetStringAddress(SetAbsStringTableIndex(SetupTab, c2rust_fresh11 as COUNT))
            as *const ::core::ffi::c_char,
        '\n' as ::core::ffi::c_char,
        C2Rust_Unnamed_6::MENU_COUNT.0 as ::core::ffi::c_int
            + C2Rust_Unnamed_5::CHOICE_COUNT.0 as ::core::ffi::c_int
            + C2Rust_Unnamed_7::SLIDER_COUNT.0 as ::core::ffi::c_int
            + C2Rust_Unnamed_8::BUTTON_COUNT.0 as ::core::ffi::c_int
            + C2Rust_Unnamed_9::LABEL_COUNT.0 as ::core::ffi::c_int
            + C2Rust_Unnamed_10::TEXTENTRY_COUNT.0 as ::core::ffi::c_int
            + C2Rust_Unnamed_11::CONTROLENTRY_COUNT.0 as ::core::ffi::c_int
            + C2Rust_Unnamed_12::MENUCONTROL_COUNT.0 as ::core::ffi::c_int,
        &raw mut buffer as *mut *const ::core::ffi::c_char,
        bank,
    ) != C2Rust_Unnamed_11::CONTROLENTRY_COUNT.0 as ::core::ffi::c_int
    {
        log_add(
            log_Level::log_Fatal,
            b"PANIC: Incorrect number of Control Entries\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        exit(1 as ::core::ffi::c_int);
    }
    i = 0 as ::core::ffi::c_int;
    while i < C2Rust_Unnamed_11::CONTROLENTRY_COUNT.0 as ::core::ffi::c_int {
        controlentries[i as usize].tag = WIDGET_TYPE::WIDGET_TYPE_CONTROLENTRY;
        controlentries[i as usize].parent = ::core::ptr::null_mut::<_widget>();
        controlentries[i as usize].handleEvent = Some(
            Widget_HandleEventControlEntry
                as unsafe extern "C" fn(
                    *mut WIDGET,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut _widget,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
            >;
        controlentries[i as usize].receiveFocus = Some(
            Widget_ReceiveFocusControlEntry
                as unsafe extern "C" fn(
                    *mut WIDGET,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut _widget,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
            >;
        controlentries[i as usize].draw = Some(
            Widget_DrawControlEntry
                as unsafe extern "C" fn(
                    *mut WIDGET,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C" fn(
                    *mut _widget,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> (),
            >;
        controlentries[i as usize].height = Some(
            Widget_HeightOneLine
                as unsafe extern "C" fn(*mut WIDGET) -> ::core::ffi::c_int,
        ) as Option<unsafe extern "C" fn(*mut _widget) -> ::core::ffi::c_int>;
        controlentries[i as usize].width = Some(
            Widget_WidthFullScreen
                as unsafe extern "C" fn(*mut WIDGET) -> ::core::ffi::c_int,
        ) as Option<unsafe extern "C" fn(*mut _widget) -> ::core::ffi::c_int>;
        controlentries[i as usize].category = buffer[i as usize];
        controlentries[i as usize].highlighted = 0 as ::core::ffi::c_int;
        controlentries[i as usize].controlname[0usize][0usize] = 0
            as ::core::ffi::c_char;
        controlentries[i as usize].controlname[1usize][0usize] = 0
            as ::core::ffi::c_char;
        controlentries[i as usize].controlname[2usize][0usize] = 0
            as ::core::ffi::c_char;
        controlentries[i as usize].controlname[3usize][0usize] = 0
            as ::core::ffi::c_char;
        controlentries[i as usize].controlindex = i;
        controlentries[i as usize].onChange = Some(
            rebind_control as unsafe extern "C" fn(*mut WIDGET_CONTROLENTRY) -> (),
        ) as Option<unsafe extern "C" fn(*mut _widget_controlentry) -> ()>;
        controlentries[i as usize].onDelete = Some(
            clear_control as unsafe extern "C" fn(*mut WIDGET_CONTROLENTRY) -> (),
        ) as Option<unsafe extern "C" fn(*mut _widget_controlentry) -> ()>;
        controlentries[i as usize].current_page = 0 as ::core::ffi::c_int;
        controlentries[i as usize].num_pages = 2 as ::core::ffi::c_int;
        i += 1;
    }
    if index >= count {
        log_add(
            log_Level::log_Fatal,
            b"PANIC: String table cut short while reading menu control entries\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        exit(1 as ::core::ffi::c_int);
    }
    let c2rust_fresh12 = index;
    index += 1;
    if SplitString(
        GetStringAddress(SetAbsStringTableIndex(SetupTab, c2rust_fresh12 as COUNT))
            as *const ::core::ffi::c_char,
        '\n' as ::core::ffi::c_char,
        C2Rust_Unnamed_6::MENU_COUNT.0 as ::core::ffi::c_int
            + C2Rust_Unnamed_5::CHOICE_COUNT.0 as ::core::ffi::c_int
            + C2Rust_Unnamed_7::SLIDER_COUNT.0 as ::core::ffi::c_int
            + C2Rust_Unnamed_8::BUTTON_COUNT.0 as ::core::ffi::c_int
            + C2Rust_Unnamed_9::LABEL_COUNT.0 as ::core::ffi::c_int
            + C2Rust_Unnamed_10::TEXTENTRY_COUNT.0 as ::core::ffi::c_int
            + C2Rust_Unnamed_11::CONTROLENTRY_COUNT.0 as ::core::ffi::c_int
            + C2Rust_Unnamed_12::MENUCONTROL_COUNT.0 as ::core::ffi::c_int,
        &raw mut buffer as *mut *const ::core::ffi::c_char,
        bank,
    ) != C2Rust_Unnamed_12::MENUCONTROL_COUNT.0 as ::core::ffi::c_int
    {
        log_add(
            log_Level::log_Fatal,
            b"PANIC: Incorrect number of Menu Control Entries\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        exit(1 as ::core::ffi::c_int);
    }
    i = 0 as ::core::ffi::c_int;
    while i < C2Rust_Unnamed_12::MENUCONTROL_COUNT.0 as ::core::ffi::c_int {
        menucontrols[i as usize].tag = WIDGET_TYPE::WIDGET_TYPE_MENUCONTROLENTRY;
        menucontrols[i as usize].parent = ::core::ptr::null_mut::<_widget>();
        menucontrols[i as usize].handleEvent = Some(
            Widget_HandleEventMenuControlEntry
                as unsafe extern "C" fn(
                    *mut WIDGET,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut _widget,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
            >;
        menucontrols[i as usize].receiveFocus = Some(
            Widget_ReceiveFocusMenuControlEntry
                as unsafe extern "C" fn(
                    *mut WIDGET,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut _widget,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
            >;
        menucontrols[i as usize].draw = Some(
            Widget_DrawMenuControlEntry
                as unsafe extern "C" fn(
                    *mut WIDGET,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C" fn(
                    *mut _widget,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> (),
            >;
        menucontrols[i as usize].height = Some(
            Widget_HeightOneLine
                as unsafe extern "C" fn(*mut WIDGET) -> ::core::ffi::c_int,
        ) as Option<unsafe extern "C" fn(*mut _widget) -> ::core::ffi::c_int>;
        menucontrols[i as usize].width = Some(
            Widget_WidthFullScreen
                as unsafe extern "C" fn(*mut WIDGET) -> ::core::ffi::c_int,
        ) as Option<unsafe extern "C" fn(*mut _widget) -> ::core::ffi::c_int>;
        menucontrols[i as usize].category = buffer[i as usize];
        menucontrols[i as usize].highlighted = 0 as ::core::ffi::c_int;
        let mut j_5: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while j_5 < 6 as ::core::ffi::c_int {
            menucontrols[i as usize].controlname[j_5 as usize][0usize] = 0
                as ::core::ffi::c_char;
            j_5 += 1;
        }
        menucontrols[i as usize].controlindex = i;
        menucontrols[i as usize].onChange = Some(
            rebind_menu_control
                as unsafe extern "C" fn(*mut WIDGET_MENUCONTROLENTRY) -> (),
        ) as Option<unsafe extern "C" fn(*mut _widget_menucontrolentry) -> ()>;
        menucontrols[i as usize].onDelete = Some(
            clear_menu_control
                as unsafe extern "C" fn(*mut WIDGET_MENUCONTROLENTRY) -> (),
        ) as Option<unsafe extern "C" fn(*mut _widget_menucontrolentry) -> ()>;
        menucontrols[i as usize].current_page = 0 as ::core::ffi::c_int;
        menucontrols[i as usize].num_pages = 3 as ::core::ffi::c_int;
        memcpy(
            (&raw mut saved_menu_bindings as *mut MENU_BINDINGS).offset(i as isize)
                as *mut ::core::ffi::c_void,
            (&raw mut curr_bindings as *mut MENU_BINDINGS).offset(i as isize)
                as *const ::core::ffi::c_void,
            ::core::mem::size_of::<MENU_BINDINGS>(),
        );
        i += 1;
    }
    if index < count {
        log_add(
            log_Level::log_Warning,
            b"WARNING: Setup strings had %d garbage entries at the end.\0".as_ptr()
                as *const ::core::ffi::c_char,
            count - index,
        );
    }
    testSounds = CaptureStringTable(
        uqm_lua_checked!(LoadSoundInstance(b"sounds.test\0".as_ptr() as RESOURCE), ()),
    ) as SOUND;
}
unsafe extern "C" fn clean_up_widgets() {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < C2Rust_Unnamed_5::CHOICE_COUNT.0 as ::core::ffi::c_int {
        if !choices[i as usize].options.is_null() {
            HFree(choices[i as usize].options as *mut ::core::ffi::c_void);
        }
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < C2Rust_Unnamed_9::LABEL_COUNT.0 as ::core::ffi::c_int {
        if !labels[i as usize].lines.is_null() {
            HFree(labels[i as usize].lines as *mut ::core::ffi::c_void);
        }
        i += 1;
    }
    if !SetupTab.is_null() {
        DestroyStringTable(ReleaseStringTable(SetupTab));
        SetupTab = ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>();
    }
    if !bank.is_null() {
        StringBank_Free(bank);
        bank = ::core::ptr::null_mut::<stringbank>();
    }
    if !setup_frame.is_null() {
        DestroyDrawable(ReleaseDrawable(setup_frame));
        setup_frame = ::core::ptr::null_mut::<FRAME_DESC>();
        ReleaseArrows();
    }
    if !testSounds.is_null() {
        uqm_lua_checked!(DestroySound(ReleaseStringTable(testSounds)), ());
        testSounds = ::core::ptr::null_mut::<STRING_TABLE_ENTRY_DESC>();
    }
}
#[no_mangle]
pub unsafe extern "C" fn SetupMenu() {
    let mut s: SETUP_MENU_STATE = SETUP_MENU_STATE {
        InputFunc: None,
        initialized: BOOLEAN::FALSE,
        anim_frame_count: 0,
        NextTime: 0,
    };
    InSetupMenu = BOOLEAN::TRUE;
    s.InputFunc = Some(
        DoSetupMenu as unsafe extern "C" fn(*mut SETUP_MENU_STATE) -> BOOLEAN,
    ) as Option<unsafe extern "C" fn(*mut setup_menu_state) -> BOOLEAN>;
    s.initialized = BOOLEAN::FALSE;
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
    SetupTab = CaptureStringTable(
        uqm_lua_checked!(LoadStringTableInstance(b"text.setupmenu\0".as_ptr() as RESOURCE), ()),
    );
    if !SetupTab.is_null() {
        uqm_lua_checked!(init_widgets(), ());
    } else {
        log_add(
            log_Level::log_Fatal,
            b"PANIC: Could not find strings for the setup menu!\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        exit(1 as ::core::ffi::c_int);
    }
    done = BOOLEAN::FALSE;
    uqm_lua_checked!(DoInput(&raw mut s as *mut ::core::ffi::c_void, BOOLEAN::TRUE), ());
    GlobData.Game_state.CurrentActivity = (GlobData.Game_state.CurrentActivity
        as ::core::ffi::c_int
        & !(C2Rust_Unnamed_15::CHECK_ABORT.0 as ::core::ffi::c_int)) as ACTIVITY;
    uqm_lua_checked!(PropagateResults(), ());
    if !SetupTab.is_null() {
        uqm_lua_checked!(clean_up_widgets(), ());
    }
    InSetupMenu = BOOLEAN::FALSE;
    SetMenuSounds(
        (((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as MENU_SOUND_FLAGS
            as ::core::ffi::c_int
            | ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as MENU_SOUND_FLAGS
                as ::core::ffi::c_int) as MENU_SOUND_FLAGS,
        ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as MENU_SOUND_FLAGS,
    );
}
#[no_mangle]
pub unsafe extern "C" fn GetGlobalOptions(mut opts: *mut GLOBALOPTS) {
    let mut whichBound: bool = false;
    let mut flags: ::core::ffi::c_int = 0;
    let mut i: BYTE = 0;
    (*opts).screenResolution = OPT_RESTYPE(resolutionFactor >> 1 as ::core::ffi::c_int);
    if GfxFlags & (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int != 0 {
        (*opts).fullscreen = OPT_ENABLABLE(
            2 as ::core::ffi::c_int as ::core::ffi::c_uint,
        );
    } else if GfxFlags & (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int != 0 {
        (*opts).fullscreen = OPT_ENABLABLE(
            1 as ::core::ffi::c_int as ::core::ffi::c_uint,
        );
    } else {
        (*opts).fullscreen = OPT_ENABLABLE(
            0 as ::core::ffi::c_int as ::core::ffi::c_uint,
        );
    }
    (*opts).fps = OPT_ENABLABLE(
        (if GfxFlags & (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int != 0 {
            OPT_ENABLABLE::OPTVAL_ENABLED.0 as ::core::ffi::c_int
        } else {
            OPT_ENABLABLE::OPTVAL_DISABLED.0 as ::core::ffi::c_int
        }) as ::core::ffi::c_uint,
    );
    (*opts).scanlines = OPT_ENABLABLE(
        (if GfxFlags & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int != 0 {
            OPT_ENABLABLE::OPTVAL_ENABLED.0 as ::core::ffi::c_int
        } else {
            OPT_ENABLABLE::OPTVAL_DISABLED.0 as ::core::ffi::c_int
        }) as ::core::ffi::c_uint,
    );
    flags = GfxFlags & 248 as ::core::ffi::c_int;
    match flags {
        8 => {
            (*opts).scaler = OPT_SCALETYPE::OPTVAL_BILINEAR_SCALE;
        }
        16 => {
            (*opts).scaler = OPT_SCALETYPE::OPTVAL_BIADAPT_SCALE;
        }
        32 => {
            (*opts).scaler = OPT_SCALETYPE::OPTVAL_BIADV_SCALE;
        }
        64 => {
            (*opts).scaler = OPT_SCALETYPE::OPTVAL_TRISCAN_SCALE;
        }
        128 => {
            (*opts).scaler = OPT_SCALETYPE::OPTVAL_HQXX_SCALE;
        }
        _ => {
            (*opts).scaler = OPT_SCALETYPE::OPTVAL_NO_SCALE;
        }
    }
    (*opts).keepaspect = optKeepAspectRatio;
    whichBound = optGamma < maxGamma;
    if optGamma <= minGamma {
        minGamma = optGamma - 0.03f32;
    } else if optGamma >= maxGamma {
        maxGamma = optGamma + 0.3f32;
    }
    updateGammaBounds(whichBound);
    (*opts).gamma = gammaToSlider(optGamma);
    (*opts).loresBlowup = OPT_RESSCALER(loresBlowupScale);
    if if resolutionFactor != 2 as ::core::ffi::c_uint {
        BOOLEAN::FALSE.0 as ::core::ffi::c_int
    } else {
        BOOLEAN::TRUE.0 as ::core::ffi::c_int
    } == 0
        && GraphicsDriver
            != C2Rust_Unnamed_16::TFB_GFXDRIVER_SDL_PURE.0 as ::core::ffi::c_int
        && (WindowWidth == 320 as ::core::ffi::c_int
            || WindowWidth == 640 as ::core::ffi::c_int)
        || res_GetBoolean(b"config.alwaysgl\0".as_ptr() as *const ::core::ffi::c_char).0
            != 0
    {
        (*opts).driver = OPT_DRIVERTYPE::OPTVAL_ALWAYS_GL;
    } else {
        (*opts).driver = OPT_DRIVERTYPE::OPTVAL_PURE_IF_POSSIBLE;
    }
    (*opts).windowType = OPT_WINDOWTYPE(optWindowType);
    match (*opts).windowType {
        OPT_WINDOWTYPE::OPTVAL_PC_WINDOW => {
            if isAddonAvailable(
                    if if resolutionFactor != 2 as ::core::ffi::c_uint {
                        BOOLEAN::FALSE.0 as ::core::ffi::c_int
                    } else {
                        BOOLEAN::TRUE.0 as ::core::ffi::c_int
                    } != 0
                    {
                        b"dos-mode-hd\0".as_ptr() as *const ::core::ffi::c_char
                    } else {
                        b"dos-mode-sd\0".as_ptr() as *const ::core::ffi::c_char
                    },
                )
                .0 == 0
            {
                (*opts).windowType = OPT_WINDOWTYPE(
                    2 as ::core::ffi::c_int as ::core::ffi::c_uint,
                );
            }
        }
        OPT_WINDOWTYPE::OPTVAL_3DO_WINDOW => {
            if isAddonAvailable(
                    if if resolutionFactor != 2 as ::core::ffi::c_uint {
                        BOOLEAN::FALSE.0 as ::core::ffi::c_int
                    } else {
                        BOOLEAN::TRUE.0 as ::core::ffi::c_int
                    } != 0
                    {
                        b"3do-mode-hd\0".as_ptr() as *const ::core::ffi::c_char
                    } else {
                        b"3do-mode-sd\0".as_ptr() as *const ::core::ffi::c_char
                    },
                )
                .0 == 0
            {
                (*opts).windowType = OPT_WINDOWTYPE(
                    2 as ::core::ffi::c_int as ::core::ffi::c_uint,
                );
            }
        }
        _ => {}
    }
    (*opts).stereo = optStereoSFX;
    (*opts).music3do = opt3doMusic;
    (*opts).musicremix = optRemixMusic;
    (*opts).volasMusic = optVolasMusic;
    (*opts).spaceMusic = OPT_SPACEMUSIC(optSpaceMusic as ::core::ffi::c_uint);
    (*opts).mainMenuMusic = optMainMenuMusic;
    (*opts).musicResume = OPT_MUSICRESUME(optMusicResume as ::core::ffi::c_uint);
    (*opts).speech = optSpeech;
    match snddriver {
        2 => {
            (*opts).adriver = OPT_ADRIVERTYPE::OPTVAL_OPENAL;
        }
        0 => {
            (*opts).adriver = OPT_ADRIVERTYPE::OPTVAL_MIXSDL;
        }
        _ => {
            (*opts).adriver = OPT_ADRIVERTYPE::OPTVAL_SILENCE;
        }
    }
    audioDriver = (*opts).adriver.0;
    if soundflags & (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int != 0 {
        (*opts).aquality = OPT_AQUALITYTYPE::OPTVAL_HIGH;
    } else if soundflags & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int != 0 {
        (*opts).aquality = OPT_AQUALITYTYPE::OPTVAL_LOW;
    } else {
        (*opts).aquality = OPT_AQUALITYTYPE::OPTVAL_MEDIUM;
    }
    audioQuality = (*opts).aquality.0;
    (*opts).musicvol = ((musicVolumeScale * 100.0f32) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) / 5 as ::core::ffi::c_int * 5 as ::core::ffi::c_int;
    MusVol = (*opts).musicvol;
    (*opts).sfxvol = ((sfxVolumeScale * 100.0f32) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) / 5 as ::core::ffi::c_int * 5 as ::core::ffi::c_int;
    SfxVol = (*opts).sfxvol;
    (*opts).speechvol = ((speechVolumeScale * 100.0f32) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) / 5 as ::core::ffi::c_int * 5 as ::core::ffi::c_int;
    SpcVol = (*opts).speechvol;
    (*opts).subtitles = optSubtitles;
    (*opts).menu = is3DO(optWhichMenu);
    (*opts).submenu = optSubmenu;
    (*opts).text = is3DO(optWhichFonts);
    (*opts).scrTrans = is3DO(optScrTrans);
    (*opts).intro = is3DO(optWhichIntro);
    (*opts).skipIntro = optSkipIntro;
    (*opts).meleezoom = OPT_MELEEZOOM(
        (if optMeleeScale == C2Rust_Unnamed_17::TFB_SCALE_STEP.0 as ::core::ffi::c_int {
            OPT_CONSOLETYPE::OPTVAL_PC.0 as ::core::ffi::c_int
        } else {
            OPT_CONSOLETYPE::OPTVAL_3DO.0 as ::core::ffi::c_int
        }) as ::core::ffi::c_uint,
    );
    optMScale = (*opts).meleezoom.0 as ::core::ffi::c_int;
    (*opts).dateType = OPT_DATETYPE(optDateFormat as ::core::ffi::c_uint);
    (*opts).customBorder = optCustomBorder;
    (*opts).flagshipColor = is3DO(optFlagshipColor);
    (*opts).gameOver = optGameOver;
    (*opts).hyperStars = optHyperStars;
    (*opts).showVisitedStars = optShowVisitedStars;
    (*opts).fuelRange = OPT_FUELRANGE(optFuelRange as ::core::ffi::c_uint);
    (*opts).wholeFuel = optWholeFuel;
    (*opts).meleeToolTips = optMeleeToolTips;
    (*opts).sphereColors = OPT_SPHERECOLORS(optSphereColors as ::core::ffi::c_uint);
    (*opts).dosMenus = optDosMenus;
    (*opts).hyperSpaceColor = is3DO(optHyperSpaceColor);
    (*opts).nebulae = optNebulae;
    (*opts).nebulaevol = optNebulaeVolume;
    (*opts).starBackground = optStarBackground;
    (*opts).unscaledStarSystem = optUnscaledStarSystem;
    (*opts).planetStyle = is3DO(optPlanetStyle);
    (*opts).orbitingPlanets = optOrbitingPlanets;
    (*opts).texturedPlanets = optTexturedPlanets;
    (*opts).landerHold = is3DO(optLanderHold);
    (*opts).partialPickup = optPartialPickup;
    (*opts).cscan = optWhichCoarseScan;
    (*opts).hazardColors = optHazardColors;
    (*opts).scanStyle = is3DO(optScanStyle);
    (*opts).landerStyle = is3DO(optSuperPC);
    (*opts).planetTexture = optPlanetTexture;
    (*opts).sphereType = OPT_SPHERETYPE(optScanSphere as ::core::ffi::c_uint);
    (*opts).tintPlanSphere = is3DO(optTintPlanSphere);
    (*opts).shield = is3DO(optWhichShield);
    (*opts).difficulty = OPT_DIFFICULTY(optDiffChooser as ::core::ffi::c_uint);
    (*opts).extended = optExtended;
    (*opts).nomad = OPT_NOMAD(optNomad as ::core::ffi::c_uint);
    (*opts).slaughterMode = optSlaughterMode;
    (*opts).seedType = OPT_SEED(optSeedType as ::core::ffi::c_uint);
    (*opts).shipSeed = optShipSeed;
    (*opts).fleetPointSys = optFleetPointSys;
    (*opts).scroll = is3DO(optSmoothScroll);
    (*opts).orzCompFont = optOrzCompFont;
    (*opts).scopeStyle = is3DO(optScopeStyle);
    (*opts).nonStopOscill = optNonStopOscill;
    (*opts).smartAutoPilot = optSmartAutoPilot;
    (*opts).advancedAutoPilot = optAdvancedAutoPilot;
    (*opts).shipDirectionIP = optShipDirectionIP;
    (*opts).autoButtons = optAutoButtons;
    (*opts).controllerType = OPT_CONTROLLER(optControllerType as ::core::ffi::c_uint);
    (*opts).dirJoy[0usize] = OPT_DIRECTJOY(optDirJoy[0usize] as ::core::ffi::c_uint);
    (*opts).dirJoy[1usize] = OPT_DIRECTJOY(optDirJoy[1usize] as ::core::ffi::c_uint);
    (*opts).deadZoneLeftStick[0usize] = deadzoneToSlider(DeadZoneLeftStick[0usize]);
    (*opts).deadZoneRightStick[0usize] = deadzoneToSlider(DeadZoneRightStick[0usize]);
    (*opts).deadZoneLeftStick[1usize] = deadzoneToSlider(DeadZoneLeftStick[1usize]);
    (*opts).deadZoneRightStick[1usize] = deadzoneToSlider(DeadZoneRightStick[1usize]);
    (*opts).player1 = *(&raw mut PlayerControls as *mut CONTROL_TEMPLATE).offset(0isize);
    (*opts).player2 = *(&raw mut PlayerControls as *mut CONTROL_TEMPLATE).offset(1isize);
    (*opts).scatterElements = optScatterElements;
    (*opts).showUpgrades = optShowUpgrades;
    (*opts).shipStore = optShipStore;
    (*opts).captainNames = optCaptainNames;
    (*opts).cheatMode = optCheatMode;
    (*opts).godModes = OPT_GODTYPE(optGodModes as ::core::ffi::c_uint);
    (*opts).tdType = OPT_TDTYPE(timeDilationScale as ::core::ffi::c_uint);
    (*opts).bubbleWarp = optBubbleWarp;
    (*opts).unlockShips = optUnlockShips;
    (*opts).headStart = optHeadStart;
    (*opts).infiniteCredits = optInfiniteCredits;
    (*opts).infiniteRU = optInfiniteRU;
    (*opts).infiniteFuel = optInfiniteFuel;
    (*opts).noHQEncounters = optNoHQEncounters;
    (*opts).deCleansing = optDeCleansing;
    (*opts).meleeObstacles = optMeleeObstacles;
    i = 0 as BYTE;
    while (i as ::core::ffi::c_int)
        < C2Rust_Unnamed_14::NUM_DEVICES.0 as ::core::ffi::c_int
    {
        (*opts).deviceArray[i as usize] = optDeviceArray[i as usize];
        i = i.wrapping_add(1);
    }
    i = 0 as BYTE;
    while (i as ::core::ffi::c_int) < 13 as ::core::ffi::c_int {
        (*opts).upgradeArray[i as usize] = optUpgradeArray[i as usize];
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn SetGlobalOptions(mut opts: *mut GLOBALOPTS) {
    let mut NewSndFlags: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut resFactor: ::core::ffi::c_int = resolutionFactor as ::core::ffi::c_int;
    let mut newFactor: ::core::ffi::c_int = 0;
    let mut i: BYTE = 0;
    newFactor = ((*opts).screenResolution.0 << 1 as ::core::ffi::c_int)
        as ::core::ffi::c_int;
    uqm_lua_checked!(PutIntOpt(
        &raw mut resFactor,
        &raw mut newFactor,
        b"config.resolutionfactor\0".as_ptr() as *const ::core::ffi::c_char,
        BOOLEAN::TRUE,
    ), ());
    if resFactor != resolutionFactor as ::core::ffi::c_int {
        uqm_lua_checked!(SleepThreadUntil(
            FadeScreen(
                ScreenFadeType::FadeAllToBlack,
                (840 as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as SIZE,
            ),
        ), ());
        resolutionFactor = resFactor as ::core::ffi::c_uint;
        match (*opts).windowType {
            OPT_WINDOWTYPE::OPTVAL_PC_WINDOW => {
                if isAddonAvailable(
                        if resolutionFactor != 0 {
                            b"dos-mode-hd\0".as_ptr() as *const ::core::ffi::c_char
                        } else {
                            b"dos-mode-sd\0".as_ptr() as *const ::core::ffi::c_char
                        },
                    )
                    .0 == 0
                {
                    (*opts).windowType = OPT_WINDOWTYPE::OPTVAL_UQM_WINDOW;
                }
            }
            OPT_WINDOWTYPE::OPTVAL_3DO_WINDOW => {
                if isAddonAvailable(
                        if resolutionFactor != 0 {
                            b"3do-mode-hd\0".as_ptr() as *const ::core::ffi::c_char
                        } else {
                            b"3do-mode-sd\0".as_ptr() as *const ::core::ffi::c_char
                        },
                    )
                    .0 == 0
                {
                    (*opts).windowType = OPT_WINDOWTYPE::OPTVAL_UQM_WINDOW;
                }
            }
            _ => {}
        }
    }
    if optWindowType != (*opts).windowType.0 {
        uqm_lua_checked!(PutIntOpt(
            &raw mut optWindowType as *mut ::core::ffi::c_int,
            &raw mut (*opts).windowType as *mut ::core::ffi::c_int,
            b"mm.windowType\0".as_ptr() as *const ::core::ffi::c_char,
            BOOLEAN::TRUE,
        ), ());
    }
    if optGamma != 1.0f32 || sliderToGamma((*opts).gamma) != 1.0f32 {
        optGamma = sliderToGamma((*opts).gamma);
        setGammaCorrection(optGamma);
        uqm_lua_checked!(res_PutInteger(
            b"config.gamma\0".as_ptr() as *const ::core::ffi::c_char,
            ((optGamma * 1000 as ::core::ffi::c_int as ::core::ffi::c_float)
                as ::core::ffi::c_double + 0.5f64) as ::core::ffi::c_int,
        ), ());
    }
    uqm_lua_checked!(PutBoolOpt(
        &raw mut optStereoSFX,
        &raw mut (*opts).stereo,
        b"config.positionalsfx\0".as_ptr() as *const ::core::ffi::c_char,
        BOOLEAN::TRUE,
    ), ());
    uqm_lua_checked!(PutBoolOpt(
        &raw mut opt3doMusic,
        &raw mut (*opts).music3do,
        b"config.3domusic\0".as_ptr() as *const ::core::ffi::c_char,
        BOOLEAN::TRUE,
    ), ());
    uqm_lua_checked!(PutBoolOpt(
        &raw mut optRemixMusic,
        &raw mut (*opts).musicremix,
        b"config.remixmusic\0".as_ptr() as *const ::core::ffi::c_char,
        BOOLEAN::TRUE,
    ), ());
    uqm_lua_checked!(PutBoolOpt(
        &raw mut optVolasMusic,
        &raw mut (*opts).volasMusic,
        b"mm.volasMusic\0".as_ptr() as *const ::core::ffi::c_char,
        BOOLEAN::TRUE,
    ), ());
    uqm_lua_checked!(PutIntOpt(
        &raw mut optSpaceMusic,
        &raw mut (*opts).spaceMusic as *mut ::core::ffi::c_int,
        b"mm.spaceMusic\0".as_ptr() as *const ::core::ffi::c_char,
        BOOLEAN::TRUE,
    ), ());
    if uqm_lua_checked!(PutBoolOpt(
            &raw mut optMainMenuMusic,
            &raw mut (*opts).mainMenuMusic,
            b"mm.mainMenuMusic\0".as_ptr() as *const ::core::ffi::c_char,
            BOOLEAN::FALSE,
        ), ())
        .0 != 0
    {
        if optMainMenuMusic.0 != 0 {
            uqm_lua_checked!(InitMenuMusic(), ());
        } else {
            uqm_lua_checked!(UninitMenuMusic(), ());
        }
    }
    uqm_lua_checked!(PutIntOpt(
        &raw mut optMusicResume,
        &raw mut (*opts).musicResume as *mut ::core::ffi::c_int,
        b"mm.musicResume\0".as_ptr() as *const ::core::ffi::c_char,
        BOOLEAN::FALSE,
    ), ());
    uqm_lua_checked!(PutBoolOpt(
        &raw mut optSpeech,
        &raw mut (*opts).speech,
        b"config.speech\0".as_ptr() as *const ::core::ffi::c_char,
        BOOLEAN::TRUE,
    ), ());
    if audioDriver != (*opts).adriver.0 {
        audioDriver = (*opts).adriver.0;
        match (*opts).adriver {
            OPT_ADRIVERTYPE::OPTVAL_SILENCE => {
                snddriver = C2Rust_Unnamed_19::audio_DRIVER_NOSOUND.0
                    as ::core::ffi::c_int;
                uqm_lua_checked!(res_PutString(
                    b"config.audiodriver\0".as_ptr() as *const ::core::ffi::c_char,
                    b"none\0".as_ptr() as *const ::core::ffi::c_char,
                ), ());
            }
            OPT_ADRIVERTYPE::OPTVAL_MIXSDL => {
                snddriver = C2Rust_Unnamed_19::audio_DRIVER_MIXSDL.0
                    as ::core::ffi::c_int;
                uqm_lua_checked!(res_PutString(
                    b"config.audiodriver\0".as_ptr() as *const ::core::ffi::c_char,
                    b"mixsdl\0".as_ptr() as *const ::core::ffi::c_char,
                ), ());
            }
            OPT_ADRIVERTYPE::OPTVAL_OPENAL => {
                snddriver = C2Rust_Unnamed_19::audio_DRIVER_OPENAL.0
                    as ::core::ffi::c_int;
                uqm_lua_checked!(res_PutString(
                    b"config.audiodriver\0".as_ptr() as *const ::core::ffi::c_char,
                    b"openal\0".as_ptr() as *const ::core::ffi::c_char,
                ), ());
            }
            _ => {}
        }
        optRequiresRestart = BOOLEAN::TRUE;
    }
    if audioQuality != (*opts).aquality.0 {
        audioQuality = (*opts).aquality.0;
        match (*opts).aquality {
            OPT_AQUALITYTYPE::OPTVAL_LOW => {
                NewSndFlags |= (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int;
                uqm_lua_checked!(res_PutString(
                    b"config.audioquality\0".as_ptr() as *const ::core::ffi::c_char,
                    b"low\0".as_ptr() as *const ::core::ffi::c_char,
                ), ());
            }
            OPT_AQUALITYTYPE::OPTVAL_MEDIUM => {
                NewSndFlags |= (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int;
                uqm_lua_checked!(res_PutString(
                    b"config.audioquality\0".as_ptr() as *const ::core::ffi::c_char,
                    b"medium\0".as_ptr() as *const ::core::ffi::c_char,
                ), ());
            }
            OPT_AQUALITYTYPE::OPTVAL_HIGH => {
                NewSndFlags |= (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int;
                uqm_lua_checked!(res_PutString(
                    b"config.audioquality\0".as_ptr() as *const ::core::ffi::c_char,
                    b"high\0".as_ptr() as *const ::core::ffi::c_char,
                ), ());
            }
            _ => {}
        }
        soundflags = NewSndFlags;
        optRequiresRestart = BOOLEAN::TRUE;
    }
    uqm_lua_checked!(PutIntOpt(
        &raw mut SfxVol,
        &raw mut (*opts).sfxvol,
        b"config.sfxvol\0".as_ptr() as *const ::core::ffi::c_char,
        BOOLEAN::FALSE,
    ), ());
    uqm_lua_checked!(PutIntOpt(
        &raw mut MusVol,
        &raw mut (*opts).musicvol,
        b"config.musicvol\0".as_ptr() as *const ::core::ffi::c_char,
        BOOLEAN::FALSE,
    ), ());
    uqm_lua_checked!(PutIntOpt(
        &raw mut SpcVol,
        &raw mut (*opts).speechvol,
        b"config.speechvol\0".as_ptr() as *const ::core::ffi::c_char,
        BOOLEAN::FALSE,
    ), ());
    uqm_lua_checked!(PutBoolOpt(
        &raw mut optSubtitles,
        &raw mut (*opts).subtitles,
        b"config.subtitles\0".as_ptr() as *const ::core::ffi::c_char,
        BOOLEAN::FALSE,
    ), ());
    uqm_lua_checked!(PutConsOpt(
        &raw mut optWhichMenu,
        &raw mut (*opts).menu,
        b"config.textmenu\0".as_ptr() as *const ::core::ffi::c_char,
        BOOLEAN::FALSE,
    ), ());
    uqm_lua_checked!(PutBoolOpt(
        &raw mut optSubmenu,
        &raw mut (*opts).submenu,
        b"mm.submenu\0".as_ptr() as *const ::core::ffi::c_char,
        BOOLEAN::FALSE,
    ), ());
    uqm_lua_checked!(PutConsOpt(
        &raw mut optWhichFonts,
        &raw mut (*opts).text,
        b"config.textgradients\0".as_ptr() as *const ::core::ffi::c_char,
        BOOLEAN::FALSE,
    ), ());
    uqm_lua_checked!(PutConsOpt(
        &raw mut optScrTrans,
        &raw mut (*opts).scrTrans,
        b"mm.scrTransition\0".as_ptr() as *const ::core::ffi::c_char,
        BOOLEAN::FALSE,
    ), ());
    uqm_lua_checked!(PutConsOpt(
        &raw mut optWhichIntro,
        &raw mut (*opts).intro,
        b"config.3domovies\0".as_ptr() as *const ::core::ffi::c_char,
        BOOLEAN::TRUE,
    ), ());
    uqm_lua_checked!(PutBoolOpt(
        &raw mut optSkipIntro,
        &raw mut (*opts).skipIntro,
        b"mm.skipIntro\0".as_ptr() as *const ::core::ffi::c_char,
        BOOLEAN::FALSE,
    ), ());
    if optMScale != (*opts).meleezoom.0 as ::core::ffi::c_int {
        optMeleeScale = if (*opts).meleezoom.0 as ::core::ffi::c_int
            == OPT_CONSOLETYPE::OPTVAL_3DO.0 as ::core::ffi::c_int
        {
            C2Rust_Unnamed_17::TFB_SCALE_TRILINEAR.0 as ::core::ffi::c_int
        } else {
            C2Rust_Unnamed_17::TFB_SCALE_STEP.0 as ::core::ffi::c_int
        };
        uqm_lua_checked!(res_PutBoolean(
            b"config.smoothmelee\0".as_ptr() as *const ::core::ffi::c_char,
            BOOLEAN(
                ((*opts).meleezoom.0 as ::core::ffi::c_int
                    == OPT_CONSOLETYPE::OPTVAL_3DO.0 as ::core::ffi::c_int)
                    as ::core::ffi::c_int as ::core::ffi::c_uint,
            ),
        ), ());
    }
    uqm_lua_checked!(PutIntOpt(
        &raw mut optDateFormat,
        &raw mut (*opts).dateType as *mut ::core::ffi::c_int,
        b"mm.dateFormat\0".as_ptr() as *const ::core::ffi::c_char,
        BOOLEAN::FALSE,
    ), ());
    uqm_lua_checked!(PutBoolOpt(
        &raw mut optCustomBorder,
        &raw mut (*opts).customBorder,
        b"mm.customBorder\0".as_ptr() as *const ::core::ffi::c_char,
        BOOLEAN::FALSE,
    ), ());
    uqm_lua_checked!(PutConsOpt(
        &raw mut optFlagshipColor,
        &raw mut (*opts).flagshipColor,
        b"mm.flagshipColor\0".as_ptr() as *const ::core::ffi::c_char,
        BOOLEAN::FALSE,
    ), ());
    uqm_lua_checked!(PutBoolOpt(
        &raw mut optGameOver,
        &raw mut (*opts).gameOver,
        b"mm.gameOver\0".as_ptr() as *const ::core::ffi::c_char,
        BOOLEAN::FALSE,
    ), ());
    uqm_lua_checked!(PutBoolOpt(
        &raw mut optHyperStars,
        &raw mut (*opts).hyperStars,
        b"mm.hyperStars\0".as_ptr() as *const ::core::ffi::c_char,
        BOOLEAN::FALSE,
    ), ());
    uqm_lua_checked!(PutBoolOpt(
        &raw mut optShowVisitedStars,
        &raw mut (*opts).showVisitedStars,
        b"mm.showVisitedStars\0".as_ptr() as *const ::core::ffi::c_char,
        BOOLEAN::FALSE,
    ), ());
    uqm_lua_checked!(PutIntOpt(
        &raw mut optFuelRange,
        &raw mut (*opts).fuelRange as *mut ::core::ffi::c_int,
        b"mm.fuelRange\0".as_ptr() as *const ::core::ffi::c_char,
        BOOLEAN::FALSE,
    ), ());
    uqm_lua_checked!(PutBoolOpt(
        &raw mut optWholeFuel,
        &raw mut (*opts).wholeFuel,
        b"mm.wholeFuel\0".as_ptr() as *const ::core::ffi::c_char,
        BOOLEAN::FALSE,
    ), ());
    uqm_lua_checked!(PutBoolOpt(
        &raw mut optMeleeToolTips,
        &raw mut (*opts).meleeToolTips,
        b"mm.meleeToolTips\0".as_ptr() as *const ::core::ffi::c_char,
        BOOLEAN::FALSE,
    ), ());
    uqm_lua_checked!(PutIntOpt(
        &raw mut optSphereColors,
        &raw mut (*opts).sphereColors as *mut ::core::ffi::c_int,
        b"mm.sphereColors\0".as_ptr() as *const ::core::ffi::c_char,
        BOOLEAN::FALSE,
    ), ());
    uqm_lua_checked!(PutBoolOpt(
        &raw mut optScatterElements,
        &raw mut (*opts).scatterElements,
        b"mm.scatterElements\0".as_ptr() as *const ::core::ffi::c_char,
        BOOLEAN::FALSE,
    ), ());
    uqm_lua_checked!(PutBoolOpt(
        &raw mut optShipStore,
        &raw mut (*opts).shipStore,
        b"mm.shipStore\0".as_ptr() as *const ::core::ffi::c_char,
        BOOLEAN::FALSE,
    ), ());
    uqm_lua_checked!(PutBoolOpt(
        &raw mut optCaptainNames,
        &raw mut (*opts).captainNames,
        b"mm.captainNames\0".as_ptr() as *const ::core::ffi::c_char,
        BOOLEAN::FALSE,
    ), ());
    uqm_lua_checked!(PutBoolOpt(
        &raw mut optDosMenus,
        &raw mut (*opts).dosMenus,
        b"mm.dosMenus\0".as_ptr() as *const ::core::ffi::c_char,
        BOOLEAN::FALSE,
    ), ());
    uqm_lua_checked!(PutConsOpt(
        &raw mut optHyperSpaceColor,
        &raw mut (*opts).hyperSpaceColor,
        b"mm.hyperSpaceColor\0".as_ptr() as *const ::core::ffi::c_char,
        BOOLEAN::FALSE,
    ), ());
    uqm_lua_checked!(PutBoolOpt(
        &raw mut optNebulae,
        &raw mut (*opts).nebulae,
        b"mm.nebulae\0".as_ptr() as *const ::core::ffi::c_char,
        BOOLEAN::FALSE,
    ), ());
    uqm_lua_checked!(PutIntOpt(
        &raw mut optNebulaeVolume,
        &raw mut (*opts).nebulaevol,
        b"mm.nebulaevol\0".as_ptr() as *const ::core::ffi::c_char,
        BOOLEAN::FALSE,
    ), ());
    uqm_lua_checked!(PutIntOpt(
        &raw mut optStarBackground,
        &raw mut (*opts).starBackground,
        b"mm.starBackground\0".as_ptr() as *const ::core::ffi::c_char,
        BOOLEAN::FALSE,
    ), ());
    uqm_lua_checked!(PutBoolOpt(
        &raw mut optUnscaledStarSystem,
        &raw mut (*opts).unscaledStarSystem,
        b"mm.unscaledStarSystem\0".as_ptr() as *const ::core::ffi::c_char,
        BOOLEAN::FALSE,
    ), ());
    uqm_lua_checked!(PutConsOpt(
        &raw mut optPlanetStyle,
        &raw mut (*opts).planetStyle,
        b"mm.planetStyle\0".as_ptr() as *const ::core::ffi::c_char,
        BOOLEAN::FALSE,
    ), ());
    uqm_lua_checked!(PutBoolOpt(
        &raw mut optOrbitingPlanets,
        &raw mut (*opts).orbitingPlanets,
        b"mm.orbitingPlanets\0".as_ptr() as *const ::core::ffi::c_char,
        BOOLEAN::FALSE,
    ), ());
    uqm_lua_checked!(PutBoolOpt(
        &raw mut optTexturedPlanets,
        &raw mut (*opts).texturedPlanets,
        b"mm.texturedPlanets\0".as_ptr() as *const ::core::ffi::c_char,
        BOOLEAN::FALSE,
    ), ());
    uqm_lua_checked!(PutConsOpt(
        &raw mut optLanderHold,
        &raw mut (*opts).landerHold,
        b"mm.landerHold\0".as_ptr() as *const ::core::ffi::c_char,
        BOOLEAN::FALSE,
    ), ());
    uqm_lua_checked!(PutBoolOpt(
        &raw mut optPartialPickup,
        &raw mut (*opts).partialPickup,
        b"mm.partialPickup\0".as_ptr() as *const ::core::ffi::c_char,
        BOOLEAN::FALSE,
    ), ());
    uqm_lua_checked!(PutIntOpt(
        &raw mut optWhichCoarseScan,
        &raw mut (*opts).cscan,
        b"config.iconicscan\0".as_ptr() as *const ::core::ffi::c_char,
        BOOLEAN::FALSE,
    ), ());
    uqm_lua_checked!(PutBoolOpt(
        &raw mut optHazardColors,
        &raw mut (*opts).hazardColors,
        b"mm.hazardColors\0".as_ptr() as *const ::core::ffi::c_char,
        BOOLEAN::FALSE,
    ), ());
    uqm_lua_checked!(PutConsOpt(
        &raw mut optScanStyle,
        &raw mut (*opts).scanStyle,
        b"mm.scanStyle\0".as_ptr() as *const ::core::ffi::c_char,
        BOOLEAN::FALSE,
    ), ());
    uqm_lua_checked!(PutConsOpt(
        &raw mut optSuperPC,
        &raw mut (*opts).landerStyle,
        b"mm.landerStyle\0".as_ptr() as *const ::core::ffi::c_char,
        BOOLEAN::FALSE,
    ), ());
    uqm_lua_checked!(PutBoolOpt(
        &raw mut optPlanetTexture,
        &raw mut (*opts).planetTexture,
        b"mm.planetTexture\0".as_ptr() as *const ::core::ffi::c_char,
        BOOLEAN::FALSE,
    ), ());
    uqm_lua_checked!(PutIntOpt(
        &raw mut optScanSphere,
        &raw mut (*opts).sphereType as *mut ::core::ffi::c_int,
        b"mm.sphereType\0".as_ptr() as *const ::core::ffi::c_char,
        BOOLEAN::FALSE,
    ), ());
    uqm_lua_checked!(PutConsOpt(
        &raw mut optTintPlanSphere,
        &raw mut (*opts).tintPlanSphere,
        b"mm.tintPlanSphere\0".as_ptr() as *const ::core::ffi::c_char,
        BOOLEAN::FALSE,
    ), ());
    uqm_lua_checked!(PutConsOpt(
        &raw mut optWhichShield,
        &raw mut (*opts).shield,
        b"config.pulseshield\0".as_ptr() as *const ::core::ffi::c_char,
        BOOLEAN::FALSE,
    ), ());
    uqm_lua_checked!(PutBoolOpt(
        &raw mut optShowUpgrades,
        &raw mut (*opts).showUpgrades,
        b"mm.showUpgrades\0".as_ptr() as *const ::core::ffi::c_char,
        BOOLEAN::FALSE,
    ), ());
    uqm_lua_checked!(PutIntOpt(
        &raw mut optSeedType,
        &raw mut (*opts).seedType as *mut ::core::ffi::c_int,
        b"mm.seedType\0".as_ptr() as *const ::core::ffi::c_char,
        BOOLEAN::FALSE,
    ), ());
    let mut customSeed: ::core::ffi::c_int = atoi(
        &raw mut (*(&raw mut textentries as *mut WIDGET_TEXTENTRY)
            .offset(C2Rust_Unnamed_10::TEXT_GAMESEED.0 as ::core::ffi::c_int as isize))
            .value as *mut ::core::ffi::c_char,
    );
    if if customSeed < 3 as ::core::ffi::c_int
        || customSeed > 2147483644 as ::core::ffi::c_int
    {
        0 as ::core::ffi::c_int
    } else {
        1 as ::core::ffi::c_int
    } == 0 || optSeedType == OPT_SEED::OPTVAL_PRIME.0 as ::core::ffi::c_int
    {
        customSeed = 16807 as ::core::ffi::c_int;
    }
    uqm_lua_checked!(PutIntOpt(
        &raw mut optCustomSeed,
        &raw mut customSeed,
        b"mm.customSeed\0".as_ptr() as *const ::core::ffi::c_char,
        BOOLEAN::FALSE,
    ), ());
    uqm_lua_checked!(PutBoolOpt(
        &raw mut optShipSeed,
        &raw mut (*opts).shipSeed,
        b"mm.shipSeed\0".as_ptr() as *const ::core::ffi::c_char,
        BOOLEAN::FALSE,
    ), ());
    uqm_lua_checked!(PutIntOpt(
        &raw mut optDiffChooser,
        &raw mut (*opts).difficulty as *mut ::core::ffi::c_int,
        b"mm.difficulty\0".as_ptr() as *const ::core::ffi::c_char,
        BOOLEAN::FALSE,
    ), ());
    optDifficulty = (*opts).difficulty.0 as ::core::ffi::c_int;
    if optDifficulty == OPT_DIFFICULTY::OPTVAL_IMPO.0 as ::core::ffi::c_int {
        optDifficulty = OPT_DIFFICULTY::OPTVAL_NORM.0 as ::core::ffi::c_int;
    }
    uqm_lua_checked!(PutBoolOpt(
        &raw mut optExtended,
        &raw mut (*opts).extended,
        b"mm.extended\0".as_ptr() as *const ::core::ffi::c_char,
        BOOLEAN::FALSE,
    ), ());
    uqm_lua_checked!(PutIntOpt(
        &raw mut optNomad,
        &raw mut (*opts).nomad as *mut ::core::ffi::c_int,
        b"mm.nomad\0".as_ptr() as *const ::core::ffi::c_char,
        BOOLEAN::FALSE,
    ), ());
    uqm_lua_checked!(PutBoolOpt(
        &raw mut optSlaughterMode,
        &raw mut (*opts).slaughterMode,
        b"mm.slaughterMode\0".as_ptr() as *const ::core::ffi::c_char,
        BOOLEAN::FALSE,
    ), ());
    uqm_lua_checked!(PutBoolOpt(
        &raw mut optFleetPointSys,
        &raw mut (*opts).fleetPointSys,
        b"mm.fleetPointSys\0".as_ptr() as *const ::core::ffi::c_char,
        BOOLEAN::FALSE,
    ), ());
    uqm_lua_checked!(PutConsOpt(
        &raw mut optSmoothScroll,
        &raw mut (*opts).scroll,
        b"config.smoothscroll\0".as_ptr() as *const ::core::ffi::c_char,
        BOOLEAN::FALSE,
    ), ());
    uqm_lua_checked!(PutBoolOpt(
        &raw mut optOrzCompFont,
        &raw mut (*opts).orzCompFont,
        b"mm.orzCompFont\0".as_ptr() as *const ::core::ffi::c_char,
        BOOLEAN::FALSE,
    ), ());
    uqm_lua_checked!(PutConsOpt(
        &raw mut optScopeStyle,
        &raw mut (*opts).scopeStyle,
        b"mm.scopeStyle\0".as_ptr() as *const ::core::ffi::c_char,
        BOOLEAN::FALSE,
    ), ());
    uqm_lua_checked!(PutBoolOpt(
        &raw mut optNonStopOscill,
        &raw mut (*opts).nonStopOscill,
        b"mm.nonStopOscill\0".as_ptr() as *const ::core::ffi::c_char,
        BOOLEAN::FALSE,
    ), ());
    uqm_lua_checked!(PutBoolOpt(
        &raw mut optSmartAutoPilot,
        &raw mut (*opts).smartAutoPilot,
        b"mm.smartAutoPilot\0".as_ptr() as *const ::core::ffi::c_char,
        BOOLEAN::FALSE,
    ), ());
    uqm_lua_checked!(PutBoolOpt(
        &raw mut optAdvancedAutoPilot,
        &raw mut (*opts).advancedAutoPilot,
        b"mm.advancedAutoPilot\0".as_ptr() as *const ::core::ffi::c_char,
        BOOLEAN::FALSE,
    ), ());
    uqm_lua_checked!(PutBoolOpt(
        &raw mut optShipDirectionIP,
        &raw mut (*opts).shipDirectionIP,
        b"mm.shipDirectionIP\0".as_ptr() as *const ::core::ffi::c_char,
        BOOLEAN::FALSE,
    ), ());
    uqm_lua_checked!(PutBoolOpt(
        &raw mut optAutoButtons,
        &raw mut (*opts).autoButtons,
        b"mm.autoButtons\0".as_ptr() as *const ::core::ffi::c_char,
        BOOLEAN::FALSE,
    ), ());
    uqm_lua_checked!(PutIntOpt(
        &raw mut optControllerType,
        &raw mut (*opts).controllerType as *mut ::core::ffi::c_int,
        b"mm.controllerType\0".as_ptr() as *const ::core::ffi::c_char,
        BOOLEAN::FALSE,
    ), ());
    uqm_lua_checked!(PutIntOpt(
        (&raw mut optDirJoy as *mut ::core::ffi::c_int).offset(0isize),
        (&raw mut (*opts).dirJoy as *mut OPT_DIRECTJOY).offset(0isize)
            as *mut ::core::ffi::c_int,
        b"mm.dirJoyP1\0".as_ptr() as *const ::core::ffi::c_char,
        BOOLEAN::FALSE,
    ), ());
    uqm_lua_checked!(PutIntOpt(
        (&raw mut optDirJoy as *mut ::core::ffi::c_int).offset(1isize),
        (&raw mut (*opts).dirJoy as *mut OPT_DIRECTJOY).offset(1isize)
            as *mut ::core::ffi::c_int,
        b"mm.dirJoyP2\0".as_ptr() as *const ::core::ffi::c_char,
        BOOLEAN::FALSE,
    ), ());
    uqm_lua_checked!(res_PutInteger(
        b"mm.deadZoneLeftP1\0".as_ptr() as *const ::core::ffi::c_char,
        DeadZoneLeftStick[0usize],
    ), ());
    uqm_lua_checked!(res_PutInteger(
        b"mm.deadZoneRightP1\0".as_ptr() as *const ::core::ffi::c_char,
        DeadZoneRightStick[0usize],
    ), ());
    uqm_lua_checked!(res_PutInteger(
        b"mm.deadZoneLeftP2\0".as_ptr() as *const ::core::ffi::c_char,
        DeadZoneLeftStick[1usize],
    ), ());
    uqm_lua_checked!(res_PutInteger(
        b"mm.deadZoneRightP2\0".as_ptr() as *const ::core::ffi::c_char,
        DeadZoneRightStick[1usize],
    ), ());
    uqm_lua_checked!(res_PutString(
        b"keys.version\0".as_ptr() as *const ::core::ffi::c_char,
        b"0.8.5\0".as_ptr() as *const ::core::ffi::c_char,
    ), ());
    uqm_lua_checked!(PutBoolOpt(
        &raw mut optCheatMode,
        &raw mut (*opts).cheatMode,
        b"cheat.kohrStahp\0".as_ptr() as *const ::core::ffi::c_char,
        BOOLEAN::FALSE,
    ), ());
    uqm_lua_checked!(PutIntOpt(
        &raw mut optGodModes,
        &raw mut (*opts).godModes as *mut ::core::ffi::c_int,
        b"cheat.godModes\0".as_ptr() as *const ::core::ffi::c_char,
        BOOLEAN::FALSE,
    ), ());
    uqm_lua_checked!(PutIntOpt(
        &raw mut timeDilationScale,
        &raw mut (*opts).tdType as *mut ::core::ffi::c_int,
        b"cheat.timeDilation\0".as_ptr() as *const ::core::ffi::c_char,
        BOOLEAN::FALSE,
    ), ());
    uqm_lua_checked!(PutBoolOpt(
        &raw mut optBubbleWarp,
        &raw mut (*opts).bubbleWarp,
        b"cheat.bubbleWarp\0".as_ptr() as *const ::core::ffi::c_char,
        BOOLEAN::FALSE,
    ), ());
    uqm_lua_checked!(PutBoolOpt(
        &raw mut optUnlockShips,
        &raw mut (*opts).unlockShips,
        b"cheat.unlockShips\0".as_ptr() as *const ::core::ffi::c_char,
        BOOLEAN::FALSE,
    ), ());
    uqm_lua_checked!(PutBoolOpt(
        &raw mut optHeadStart,
        &raw mut (*opts).headStart,
        b"cheat.headStart\0".as_ptr() as *const ::core::ffi::c_char,
        BOOLEAN::FALSE,
    ), ());
    uqm_lua_checked!(PutBoolOpt(
        &raw mut optInfiniteCredits,
        &raw mut (*opts).infiniteCredits,
        b"cheat.infiniteCredits\0".as_ptr() as *const ::core::ffi::c_char,
        BOOLEAN::FALSE,
    ), ());
    uqm_lua_checked!(PutBoolOpt(
        &raw mut optInfiniteRU,
        &raw mut (*opts).infiniteRU,
        b"cheat.infiniteRU\0".as_ptr() as *const ::core::ffi::c_char,
        BOOLEAN::FALSE,
    ), ());
    uqm_lua_checked!(PutBoolOpt(
        &raw mut optInfiniteFuel,
        &raw mut (*opts).infiniteFuel,
        b"cheat.infiniteFuel\0".as_ptr() as *const ::core::ffi::c_char,
        BOOLEAN::FALSE,
    ), ());
    uqm_lua_checked!(PutBoolOpt(
        &raw mut optNoHQEncounters,
        &raw mut (*opts).noHQEncounters,
        b"cheat.noHQEncounters\0".as_ptr() as *const ::core::ffi::c_char,
        BOOLEAN::FALSE,
    ), ());
    uqm_lua_checked!(PutBoolOpt(
        &raw mut optDeCleansing,
        &raw mut (*opts).deCleansing,
        b"cheat.deCleansing\0".as_ptr() as *const ::core::ffi::c_char,
        BOOLEAN::FALSE,
    ), ());
    uqm_lua_checked!(PutBoolOpt(
        &raw mut optMeleeObstacles,
        &raw mut (*opts).meleeObstacles,
        b"cheat.meleeObstacles\0".as_ptr() as *const ::core::ffi::c_char,
        BOOLEAN::FALSE,
    ), ());
    i = 0 as BYTE;
    while (i as ::core::ffi::c_int)
        < C2Rust_Unnamed_14::NUM_DEVICES.0 as ::core::ffi::c_int
    {
        optDeviceArray[i as usize] = (*opts).deviceArray[i as usize];
        i = i.wrapping_add(1);
    }
    i = 0 as BYTE;
    while (i as ::core::ffi::c_int) < 13 as ::core::ffi::c_int {
        optUpgradeArray[i as usize] = (*opts).upgradeArray[i as usize];
        i = i.wrapping_add(1);
    }
    uqm_lua_checked!(SaveResourceIndex(
        configDir,
        b"uqm.cfg\0".as_ptr() as *const ::core::ffi::c_char,
        b"config.\0".as_ptr() as *const ::core::ffi::c_char,
        BOOLEAN::TRUE,
    ), ());
    uqm_lua_checked!(SaveKeyConfiguration(
        configDir,
        b"flight.cfg\0".as_ptr() as *const ::core::ffi::c_char,
    ), ());
    uqm_lua_checked!(SaveResourceIndex(
        configDir,
        b"megamod.cfg\0".as_ptr() as *const ::core::ffi::c_char,
        b"mm.\0".as_ptr() as *const ::core::ffi::c_char,
        BOOLEAN::TRUE,
    ), ());
    uqm_lua_checked!(SaveResourceIndex(
        configDir,
        b"cheats.cfg\0".as_ptr() as *const ::core::ffi::c_char,
        b"cheat.\0".as_ptr() as *const ::core::ffi::c_char,
        BOOLEAN::TRUE,
    ), ());
    if optRequiresReload.0 != 0 {
        let mut w: ::core::ffi::c_int = WindowWidth;
        let mut h: ::core::ffi::c_int = WindowHeight;
        uqm_lua_checked!(SleepThreadUntil(
            FadeScreen(
                ScreenFadeType::FadeAllToBlack,
                (840 as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as SIZE,
            ),
        ), ());
        FlushGraphics();
        uqm_lua_checked!(UninitVideoPlayer(), ());
        ResetOffset();
        resolutionFactor = resolutionFactor;
        CanvasWidth = (320 as ::core::ffi::c_int) << resolutionFactor;
        CanvasHeight = if if optWindowType == 0 as DWORD {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
        {
            200 as ::core::ffi::c_int
        } else {
            240 as ::core::ffi::c_int
        } << resolutionFactor;
        if choices[C2Rust_Unnamed_5::CHOICE_RESOLUTION.0 as ::core::ffi::c_int as usize]
            .selected != 6 as ::core::ffi::c_int
        {
            h = ((if if optWindowType == 0 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                200 as ::core::ffi::c_int
            } else {
                240 as ::core::ffi::c_int
            }) as ::core::ffi::c_uint)
                .wrapping_mul(loresBlowupScale.wrapping_add(1 as ::core::ffi::c_uint))
                as ::core::ffi::c_int;
            SavedHeight = h;
            uqm_lua_checked!(res_PutInteger(
                b"config.resheight\0".as_ptr() as *const ::core::ffi::c_char,
                SavedHeight,
            ), ());
        }
        if optKeepAspectRatio.0 != 0 {
            let mut threshold: ::core::ffi::c_float = 0.75f32;
            let mut ratio: ::core::ffi::c_float = h as ::core::ffi::c_float
                / w as ::core::ffi::c_float;
            if ratio > threshold {
                w = (h as ::core::ffi::c_float / threshold) as ::core::ffi::c_int;
            } else if ratio < threshold {
                h = (w as ::core::ffi::c_float * threshold) as ::core::ffi::c_int;
            }
        }
        log_add(
            log_Level::log_Debug,
            b"ScreenWidth:%d, ScreenHeight:%d, Wactual:%d, Hactual:%d\0".as_ptr()
                as *const ::core::ffi::c_char,
            CanvasWidth,
            CanvasHeight,
            w,
            h,
        );
        TFB_BBox_Reset();
        TFB_BBox_Init(CanvasWidth, CanvasHeight);
        FlushColorXForms();
        TFB_DrawScreen_ReinitVideo(GraphicsDriver, GfxFlags, w, h);
        uqm_lua_checked!(InitVideoPlayer(BOOLEAN::TRUE), ());
        uqm_lua_checked!(Reload(), ());
    }
}
#[inline]
unsafe extern "C" fn inBounds(
    mut val: SDWORD,
    mut min: SDWORD,
    mut max: SDWORD,
) -> SDWORD {
    if val < min {
        return min;
    }
    if val > max {
        return max;
    }
    return val;
}
#[export_name = "uqm_c2rust_init_ba407804bbae8b28"]
unsafe extern "C" fn c2rust_run_static_initializers() {
    editdeadzone_widgets = [
        (&raw mut sliders as *mut WIDGET_SLIDER)
            .offset(
                C2Rust_Unnamed_7::SLIDER_DEADZONE_00.0 as ::core::ffi::c_int as isize,
            ) as *mut WIDGET,
        (&raw mut sliders as *mut WIDGET_SLIDER)
            .offset(
                C2Rust_Unnamed_7::SLIDER_DEADZONE_01.0 as ::core::ffi::c_int as isize,
            ) as *mut WIDGET,
        (&raw mut labels as *mut WIDGET_LABEL)
            .offset(C2Rust_Unnamed_9::LABEL_SPACER.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut sliders as *mut WIDGET_SLIDER)
            .offset(
                C2Rust_Unnamed_7::SLIDER_DEADZONE_02.0 as ::core::ffi::c_int as isize,
            ) as *mut WIDGET,
        (&raw mut sliders as *mut WIDGET_SLIDER)
            .offset(
                C2Rust_Unnamed_7::SLIDER_DEADZONE_03.0 as ::core::ffi::c_int as isize,
            ) as *mut WIDGET,
        (&raw mut labels as *mut WIDGET_LABEL)
            .offset(C2Rust_Unnamed_9::LABEL_SPACER.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut buttons as *mut WIDGET_BUTTON)
            .offset(C2Rust_Unnamed_8::BTN_PREVMENU.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        ::core::ptr::null_mut::<WIDGET>(),
    ];
    editmenukeys_widgets = [
        (&raw mut labels as *mut WIDGET_LABEL)
            .offset(C2Rust_Unnamed_9::LABEL_TAPTOOLTIP.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut labels as *mut WIDGET_LABEL)
            .offset(C2Rust_Unnamed_9::LABEL_SPACER.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut menucontrols as *mut WIDGET_MENUCONTROLENTRY)
            .offset(
                C2Rust_Unnamed_12::MENUCONTROL_PAUSE.0 as ::core::ffi::c_int as isize,
            ) as *mut WIDGET,
        (&raw mut menucontrols as *mut WIDGET_MENUCONTROLENTRY)
            .offset(C2Rust_Unnamed_12::MENUCONTROL_EXIT.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut menucontrols as *mut WIDGET_MENUCONTROLENTRY)
            .offset(
                C2Rust_Unnamed_12::MENUCONTROL_ABORT.0 as ::core::ffi::c_int as isize,
            ) as *mut WIDGET,
        (&raw mut menucontrols as *mut WIDGET_MENUCONTROLENTRY)
            .offset(
                C2Rust_Unnamed_12::MENUCONTROL_DEBUG.0 as ::core::ffi::c_int as isize,
            ) as *mut WIDGET,
        (&raw mut menucontrols as *mut WIDGET_MENUCONTROLENTRY)
            .offset(
                C2Rust_Unnamed_12::MENUCONTROL_FULLSCREEN.0 as ::core::ffi::c_int
                    as isize,
            ) as *mut WIDGET,
        (&raw mut menucontrols as *mut WIDGET_MENUCONTROLENTRY)
            .offset(C2Rust_Unnamed_12::MENUCONTROL_UP.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut menucontrols as *mut WIDGET_MENUCONTROLENTRY)
            .offset(C2Rust_Unnamed_12::MENUCONTROL_DOWN.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut menucontrols as *mut WIDGET_MENUCONTROLENTRY)
            .offset(C2Rust_Unnamed_12::MENUCONTROL_LEFT.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut menucontrols as *mut WIDGET_MENUCONTROLENTRY)
            .offset(
                C2Rust_Unnamed_12::MENUCONTROL_RIGHT.0 as ::core::ffi::c_int as isize,
            ) as *mut WIDGET,
        (&raw mut menucontrols as *mut WIDGET_MENUCONTROLENTRY)
            .offset(
                C2Rust_Unnamed_12::MENUCONTROL_SELECT.0 as ::core::ffi::c_int as isize,
            ) as *mut WIDGET,
        (&raw mut menucontrols as *mut WIDGET_MENUCONTROLENTRY)
            .offset(
                C2Rust_Unnamed_12::MENUCONTROL_CANCEL.0 as ::core::ffi::c_int as isize,
            ) as *mut WIDGET,
        (&raw mut menucontrols as *mut WIDGET_MENUCONTROLENTRY)
            .offset(
                C2Rust_Unnamed_12::MENUCONTROL_SPECIAL.0 as ::core::ffi::c_int as isize,
            ) as *mut WIDGET,
        (&raw mut menucontrols as *mut WIDGET_MENUCONTROLENTRY)
            .offset(
                C2Rust_Unnamed_12::MENUCONTROL_PAGEUP.0 as ::core::ffi::c_int as isize,
            ) as *mut WIDGET,
        (&raw mut menucontrols as *mut WIDGET_MENUCONTROLENTRY)
            .offset(
                C2Rust_Unnamed_12::MENUCONTROL_PAGEDOWN.0 as ::core::ffi::c_int as isize,
            ) as *mut WIDGET,
        (&raw mut menucontrols as *mut WIDGET_MENUCONTROLENTRY)
            .offset(C2Rust_Unnamed_12::MENUCONTROL_HOME.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut menucontrols as *mut WIDGET_MENUCONTROLENTRY)
            .offset(C2Rust_Unnamed_12::MENUCONTROL_END.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut menucontrols as *mut WIDGET_MENUCONTROLENTRY)
            .offset(
                C2Rust_Unnamed_12::MENUCONTROL_ZOOMIN.0 as ::core::ffi::c_int as isize,
            ) as *mut WIDGET,
        (&raw mut menucontrols as *mut WIDGET_MENUCONTROLENTRY)
            .offset(
                C2Rust_Unnamed_12::MENUCONTROL_ZOOMOUT.0 as ::core::ffi::c_int as isize,
            ) as *mut WIDGET,
        (&raw mut menucontrols as *mut WIDGET_MENUCONTROLENTRY)
            .offset(
                C2Rust_Unnamed_12::MENUCONTROL_DELETE.0 as ::core::ffi::c_int as isize,
            ) as *mut WIDGET,
        (&raw mut menucontrols as *mut WIDGET_MENUCONTROLENTRY)
            .offset(
                C2Rust_Unnamed_12::MENUCONTROL_BACKSPACE.0 as ::core::ffi::c_int as isize,
            ) as *mut WIDGET,
        (&raw mut menucontrols as *mut WIDGET_MENUCONTROLENTRY)
            .offset(
                C2Rust_Unnamed_12::MENUCONTROL_EDITCANCEL.0 as ::core::ffi::c_int
                    as isize,
            ) as *mut WIDGET,
        (&raw mut menucontrols as *mut WIDGET_MENUCONTROLENTRY)
            .offset(
                C2Rust_Unnamed_12::MENUCONTROL_SEARCH.0 as ::core::ffi::c_int as isize,
            ) as *mut WIDGET,
        (&raw mut menucontrols as *mut WIDGET_MENUCONTROLENTRY)
            .offset(C2Rust_Unnamed_12::MENUCONTROL_NEXT.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut menucontrols as *mut WIDGET_MENUCONTROLENTRY)
            .offset(
                C2Rust_Unnamed_12::MENUCONTROL_TOGGLEMAP.0 as ::core::ffi::c_int as isize,
            ) as *mut WIDGET,
        (&raw mut menucontrols as *mut WIDGET_MENUCONTROLENTRY)
            .offset(
                C2Rust_Unnamed_12::MENUCONTROL_SCREENSHOT.0 as ::core::ffi::c_int
                    as isize,
            ) as *mut WIDGET,
        (&raw mut menucontrols as *mut WIDGET_MENUCONTROLENTRY)
            .offset(
                C2Rust_Unnamed_12::MENUCONTROL_QUICKSAVE.0 as ::core::ffi::c_int as isize,
            ) as *mut WIDGET,
        (&raw mut menucontrols as *mut WIDGET_MENUCONTROLENTRY)
            .offset(
                C2Rust_Unnamed_12::MENUCONTROL_QUICKLOAD.0 as ::core::ffi::c_int as isize,
            ) as *mut WIDGET,
        (&raw mut menucontrols as *mut WIDGET_MENUCONTROLENTRY)
            .offset(
                C2Rust_Unnamed_12::MENUCONTROL_DEBUG2.0 as ::core::ffi::c_int as isize,
            ) as *mut WIDGET,
        (&raw mut menucontrols as *mut WIDGET_MENUCONTROLENTRY)
            .offset(
                C2Rust_Unnamed_12::MENUCONTROL_DEBUG3.0 as ::core::ffi::c_int as isize,
            ) as *mut WIDGET,
        (&raw mut menucontrols as *mut WIDGET_MENUCONTROLENTRY)
            .offset(
                C2Rust_Unnamed_12::MENUCONTROL_DEBUG4.0 as ::core::ffi::c_int as isize,
            ) as *mut WIDGET,
        (&raw mut labels as *mut WIDGET_LABEL)
            .offset(C2Rust_Unnamed_9::LABEL_SPACER.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut buttons as *mut WIDGET_BUTTON)
            .offset(
                C2Rust_Unnamed_8::BTN_LOADDEFMENUBINDS.0 as ::core::ffi::c_int as isize,
            ) as *mut WIDGET,
        (&raw mut buttons as *mut WIDGET_BUTTON)
            .offset(C2Rust_Unnamed_8::BTN_SAVEMENUBINDS.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut buttons as *mut WIDGET_BUTTON)
            .offset(
                C2Rust_Unnamed_8::BTN_CANCELMENUBINDS.0 as ::core::ffi::c_int as isize,
            ) as *mut WIDGET,
        (&raw mut buttons as *mut WIDGET_BUTTON)
            .offset(C2Rust_Unnamed_8::BTN_PREVMENU.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        ::core::ptr::null_mut::<WIDGET>(),
    ];
    upgrades_widgets = [
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(
                C2Rust_Unnamed_5::CHOICE_UPGLANDERSPD.0 as ::core::ffi::c_int as isize,
            ) as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(
                C2Rust_Unnamed_5::CHOICE_UPGLANDERCRG.0 as ::core::ffi::c_int as isize,
            ) as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(
                C2Rust_Unnamed_5::CHOICE_UPGLANDERFRE.0 as ::core::ffi::c_int as isize,
            ) as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(
                C2Rust_Unnamed_5::CHOICE_UPGLANDERBSH.0 as ::core::ffi::c_int as isize,
            ) as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(
                C2Rust_Unnamed_5::CHOICE_UPGLANDERQSH.0 as ::core::ffi::c_int as isize,
            ) as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(
                C2Rust_Unnamed_5::CHOICE_UPGLANDERLSH.0 as ::core::ffi::c_int as isize,
            ) as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(
                C2Rust_Unnamed_5::CHOICE_UPGLANDERHSH.0 as ::core::ffi::c_int as isize,
            ) as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(
                C2Rust_Unnamed_5::CHOICE_MODPOINTDEF.0 as ::core::ffi::c_int as isize,
            ) as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(
                C2Rust_Unnamed_5::CHOICE_MODFUSBLAST.0 as ::core::ffi::c_int as isize,
            ) as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(
                C2Rust_Unnamed_5::CHOICE_MODFUELTANK.0 as ::core::ffi::c_int as isize,
            ) as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(
                C2Rust_Unnamed_5::CHOICE_MODTRACKING.0 as ::core::ffi::c_int as isize,
            ) as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(
                C2Rust_Unnamed_5::CHOICE_MODHELLBORE.0 as ::core::ffi::c_int as isize,
            ) as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(C2Rust_Unnamed_5::CHOICE_MODFURNACE.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut labels as *mut WIDGET_LABEL)
            .offset(C2Rust_Unnamed_9::LABEL_SPACER.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut buttons as *mut WIDGET_BUTTON)
            .offset(C2Rust_Unnamed_8::BTN_CHTPREV.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        ::core::ptr::null_mut::<WIDGET>(),
    ];
    devices_widgets = [
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(C2Rust_Unnamed_5::CHOICE_DEVSPAWNER.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(C2Rust_Unnamed_5::CHOICE_DEVPET.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(C2Rust_Unnamed_5::CHOICE_DEVBOMB.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(C2Rust_Unnamed_5::CHOICE_DEVSUN.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(C2Rust_Unnamed_5::CHOICE_DEVSPHERE.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(C2Rust_Unnamed_5::CHOICE_DEVHELIX.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(C2Rust_Unnamed_5::CHOICE_DEVSPINDLE.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(C2Rust_Unnamed_5::CHOICE_DEVULTRON0.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(C2Rust_Unnamed_5::CHOICE_DEVULTRON1.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(C2Rust_Unnamed_5::CHOICE_DEVULTRON2.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(C2Rust_Unnamed_5::CHOICE_DEVULTRON3.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(C2Rust_Unnamed_5::CHOICE_DEVMAIDENS.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(C2Rust_Unnamed_5::CHOICE_DEVCASTER0.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(C2Rust_Unnamed_5::CHOICE_DEVCASTER1.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(C2Rust_Unnamed_5::CHOICE_DEVSHIELD.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(C2Rust_Unnamed_5::CHOICE_DEVEGGCS0.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(C2Rust_Unnamed_5::CHOICE_DEVEGGCS1.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(C2Rust_Unnamed_5::CHOICE_DEVEGGCS2.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(C2Rust_Unnamed_5::CHOICE_DEVSHUTTLE.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(C2Rust_Unnamed_5::CHOICE_DEVBEAST.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(C2Rust_Unnamed_5::CHOICE_DEVSLYCODE.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(C2Rust_Unnamed_5::CHOICE_DEVWARPPOD.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(C2Rust_Unnamed_5::CHOICE_DEVTRIDENT.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(C2Rust_Unnamed_5::CHOICE_DEVROD.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(C2Rust_Unnamed_5::CHOICE_DEVLUNBASE.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut labels as *mut WIDGET_LABEL)
            .offset(C2Rust_Unnamed_9::LABEL_SPACER.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut buttons as *mut WIDGET_BUTTON)
            .offset(C2Rust_Unnamed_8::BTN_CHTPREV.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        ::core::ptr::null_mut::<WIDGET>(),
    ];
    qol_widgets = [
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(C2Rust_Unnamed_5::CHOICE_SKIPINTRO.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(C2Rust_Unnamed_5::CHOICE_PARTPICKUP.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(
                C2Rust_Unnamed_5::CHOICE_SCATTERCARGO.0 as ::core::ffi::c_int as isize,
            ) as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(C2Rust_Unnamed_5::CHOICE_SUBMENU.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(C2Rust_Unnamed_5::CHOICE_SMARTAUTO.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(C2Rust_Unnamed_5::CHOICE_ADVAUTO.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(C2Rust_Unnamed_5::CHOICE_VISITED.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(C2Rust_Unnamed_5::CHOICE_MLTOOLTIP.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(C2Rust_Unnamed_5::CHOICE_SHIPSTORE.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut labels as *mut WIDGET_LABEL)
            .offset(C2Rust_Unnamed_9::LABEL_SPACER.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut buttons as *mut WIDGET_BUTTON)
            .offset(C2Rust_Unnamed_8::BTN_QUITSUBMENU.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        ::core::ptr::null_mut::<WIDGET>(),
    ];
    visual_widgets = [
        (&raw mut labels as *mut WIDGET_LABEL)
            .offset(C2Rust_Unnamed_9::LABEL_UI.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(C2Rust_Unnamed_5::CHOICE_DATESTRING.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(C2Rust_Unnamed_5::CHOICE_CUSTBORDER.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(C2Rust_Unnamed_5::CHOICE_FUELDECIM.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(C2Rust_Unnamed_5::CHOICE_FUELCIRCLE.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(C2Rust_Unnamed_5::CHOICE_SOICOLOR.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(C2Rust_Unnamed_5::CHOICE_ANIMHYPER.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(C2Rust_Unnamed_5::CHOICE_CAPTNAMES.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(C2Rust_Unnamed_5::CHOICE_GAMEOVER.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut labels as *mut WIDGET_LABEL)
            .offset(C2Rust_Unnamed_9::LABEL_SPACER.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut labels as *mut WIDGET_LABEL)
            .offset(C2Rust_Unnamed_9::LABEL_COMM.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(C2Rust_Unnamed_5::CHOICE_ORZFONT.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(C2Rust_Unnamed_5::CHOICE_NOSTOSCILL.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut labels as *mut WIDGET_LABEL)
            .offset(C2Rust_Unnamed_9::LABEL_SPACER.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut labels as *mut WIDGET_LABEL)
            .offset(C2Rust_Unnamed_9::LABEL_IP.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(C2Rust_Unnamed_5::CHOICE_NEBULAE.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut sliders as *mut WIDGET_SLIDER)
            .offset(C2Rust_Unnamed_7::SLIDER_NEBULA.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(C2Rust_Unnamed_5::CHOICE_ORBPLANETS.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(C2Rust_Unnamed_5::CHOICE_TEXPLANETS.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(C2Rust_Unnamed_5::CHOICE_HDM_IP.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(C2Rust_Unnamed_5::CHOICE_IPSHIPDIR.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut labels as *mut WIDGET_LABEL)
            .offset(C2Rust_Unnamed_9::LABEL_SPACER.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut labels as *mut WIDGET_LABEL)
            .offset(C2Rust_Unnamed_9::LABEL_SCAN.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(C2Rust_Unnamed_5::CHOICE_HAZARDCLR.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(C2Rust_Unnamed_5::CHOICE_PLNTEXTURE.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(
                C2Rust_Unnamed_5::CHOICE_LANDERUPGMASK.0 as ::core::ffi::c_int as isize,
            ) as *mut WIDGET,
        (&raw mut buttons as *mut WIDGET_BUTTON)
            .offset(C2Rust_Unnamed_8::BTN_QUITSUBMENU.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        ::core::ptr::null_mut::<WIDGET>(),
    ];
    music_widgets = [
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(C2Rust_Unnamed_5::CHOICE_REMIXES1.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(C2Rust_Unnamed_5::CHOICE_REMIXES2.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(C2Rust_Unnamed_5::CHOICE_REMIXES3.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut labels as *mut WIDGET_LABEL)
            .offset(C2Rust_Unnamed_9::LABEL_SPACER.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(C2Rust_Unnamed_5::CHOICE_IPMUSIC.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(C2Rust_Unnamed_5::CHOICE_MMENUMUSIC.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(C2Rust_Unnamed_5::CHOICE_MUSRESUME.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut labels as *mut WIDGET_LABEL)
            .offset(C2Rust_Unnamed_9::LABEL_SPACER.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut buttons as *mut WIDGET_BUTTON)
            .offset(C2Rust_Unnamed_8::BTN_QUITSUBMENU.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        ::core::ptr::null_mut::<WIDGET>(),
    ];
    editkeys_widgets = [
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(C2Rust_Unnamed_5::CHOICE_KBLAYOUT.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut labels as *mut WIDGET_LABEL)
            .offset(C2Rust_Unnamed_9::LABEL_SPACER.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut labels as *mut WIDGET_LABEL)
            .offset(C2Rust_Unnamed_9::LABEL_TAPTOOLTIP.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut labels as *mut WIDGET_LABEL)
            .offset(C2Rust_Unnamed_9::LABEL_SPACER.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut controlentries as *mut WIDGET_CONTROLENTRY)
            .offset(C2Rust_Unnamed_11::CONTROL_UP.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut controlentries as *mut WIDGET_CONTROLENTRY)
            .offset(C2Rust_Unnamed_11::CONTROL_DOWN.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut controlentries as *mut WIDGET_CONTROLENTRY)
            .offset(C2Rust_Unnamed_11::CONTROL_LEFT.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut controlentries as *mut WIDGET_CONTROLENTRY)
            .offset(C2Rust_Unnamed_11::CONTROL_RGHT.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut controlentries as *mut WIDGET_CONTROLENTRY)
            .offset(C2Rust_Unnamed_11::CONTROL_WEAP.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut controlentries as *mut WIDGET_CONTROLENTRY)
            .offset(C2Rust_Unnamed_11::CONTROL_SPEC.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut controlentries as *mut WIDGET_CONTROLENTRY)
            .offset(C2Rust_Unnamed_11::CONTROL_ESC.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut controlentries as *mut WIDGET_CONTROLENTRY)
            .offset(C2Rust_Unnamed_11::CONTROL_THRU.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut labels as *mut WIDGET_LABEL)
            .offset(C2Rust_Unnamed_9::LABEL_SPACER.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut buttons as *mut WIDGET_BUTTON)
            .offset(C2Rust_Unnamed_8::BTN_PREVMENU.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        ::core::ptr::null_mut::<WIDGET>(),
    ];
    advanced_widgets = [
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(C2Rust_Unnamed_5::CHOICE_SKILLLVL.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(C2Rust_Unnamed_5::CHOICE_EXTENDED.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(C2Rust_Unnamed_5::CHOICE_NOMAD.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(C2Rust_Unnamed_5::CHOICE_SLAUGHTER.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(C2Rust_Unnamed_5::CHOICE_FLEETPOINT.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut labels as *mut WIDGET_LABEL)
            .offset(C2Rust_Unnamed_9::LABEL_SPACER.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(C2Rust_Unnamed_5::CHOICE_GAMESEED.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(C2Rust_Unnamed_5::CHOICE_SHIPSEED.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut textentries as *mut WIDGET_TEXTENTRY)
            .offset(C2Rust_Unnamed_10::TEXT_GAMESEED.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut labels as *mut WIDGET_LABEL)
            .offset(C2Rust_Unnamed_9::LABEL_SPACER.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut buttons as *mut WIDGET_BUTTON)
            .offset(C2Rust_Unnamed_8::BTN_QUITSUBMENU.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        ::core::ptr::null_mut::<WIDGET>(),
    ];
    keyconfig_widgets = [
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(C2Rust_Unnamed_5::CHOICE_AUTOBUTT.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(C2Rust_Unnamed_5::CHOICE_INPDEVICE.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(C2Rust_Unnamed_5::CHOICE_DIRJOYP1.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(C2Rust_Unnamed_5::CHOICE_DIRJOYP2.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut labels as *mut WIDGET_LABEL)
            .offset(C2Rust_Unnamed_9::LABEL_SPACER.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut labels as *mut WIDGET_LABEL)
            .offset(C2Rust_Unnamed_9::LABEL_KEYSTOOLTIP.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut buttons as *mut WIDGET_BUTTON)
            .offset(C2Rust_Unnamed_8::BTN_EDITKEYS.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut buttons as *mut WIDGET_BUTTON)
            .offset(C2Rust_Unnamed_8::BTN_EDITMENUKEYS.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut buttons as *mut WIDGET_BUTTON)
            .offset(
                C2Rust_Unnamed_8::BTN_EDIT_DEADZONES.0 as ::core::ffi::c_int as isize,
            ) as *mut WIDGET,
        (&raw mut labels as *mut WIDGET_LABEL)
            .offset(C2Rust_Unnamed_9::LABEL_SPACER.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut buttons as *mut WIDGET_BUTTON)
            .offset(C2Rust_Unnamed_8::BTN_QUITSUBMENU.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        ::core::ptr::null_mut::<WIDGET>(),
    ];
    cheat_widgets = [
        (&raw mut buttons as *mut WIDGET_BUTTON)
            .offset(C2Rust_Unnamed_8::BTN_DEVMENU.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut buttons as *mut WIDGET_BUTTON)
            .offset(C2Rust_Unnamed_8::BTN_UPGMENU.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(C2Rust_Unnamed_5::CHOICE_CHEATING.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(C2Rust_Unnamed_5::CHOICE_CHDECLEAN.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(C2Rust_Unnamed_5::CHOICE_CHGODMODE.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(C2Rust_Unnamed_5::CHOICE_CHTIME.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(C2Rust_Unnamed_5::CHOICE_CHWARP.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(
                C2Rust_Unnamed_5::CHOICE_CHHEADSTART.0 as ::core::ffi::c_int as isize,
            ) as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(C2Rust_Unnamed_5::CHOICE_CHSHIPS.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(C2Rust_Unnamed_5::CHOICE_CHINFRU.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(C2Rust_Unnamed_5::CHOICE_CHINFFUEL.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(C2Rust_Unnamed_5::CHOICE_CHINFCRD.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(
                C2Rust_Unnamed_5::CHOICE_CHCLEANHYPER.0 as ::core::ffi::c_int as isize,
            ) as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(C2Rust_Unnamed_5::CHOICE_CHNOPLANET.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut buttons as *mut WIDGET_BUTTON)
            .offset(C2Rust_Unnamed_8::BTN_QUITSUBMENU.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        ::core::ptr::null_mut::<WIDGET>(),
    ];
    engine_widgets = [
        (&raw mut labels as *mut WIDGET_LABEL)
            .offset(C2Rust_Unnamed_9::LABEL_UI.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(C2Rust_Unnamed_5::CHOICE_WINDOWTYPE.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut labels as *mut WIDGET_LABEL)
            .offset(C2Rust_Unnamed_9::LABEL_SPACER.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(C2Rust_Unnamed_5::CHOICE_MENUSTYLE.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(C2Rust_Unnamed_5::CHOICE_DOSMENUS.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(C2Rust_Unnamed_5::CHOICE_FONTSTYLE.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(C2Rust_Unnamed_5::CHOICE_CUTSCENE.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(C2Rust_Unnamed_5::CHOICE_MELEEZOOM.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(C2Rust_Unnamed_5::CHOICE_FLAGSHIP.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(C2Rust_Unnamed_5::CHOICE_SCRMELT.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(C2Rust_Unnamed_5::CHOICE_HSCOLOR.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut labels as *mut WIDGET_LABEL)
            .offset(C2Rust_Unnamed_9::LABEL_SPACER.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut labels as *mut WIDGET_LABEL)
            .offset(C2Rust_Unnamed_9::LABEL_COMM.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(
                C2Rust_Unnamed_5::CHOICE_SCROLLSTYLE.0 as ::core::ffi::c_int as isize,
            ) as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(C2Rust_Unnamed_5::CHOICE_SPEECH.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(C2Rust_Unnamed_5::CHOICE_SUBTITLES.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(
                C2Rust_Unnamed_5::CHOICE_OSCILLSTYLE.0 as ::core::ffi::c_int as isize,
            ) as *mut WIDGET,
        (&raw mut labels as *mut WIDGET_LABEL)
            .offset(C2Rust_Unnamed_9::LABEL_SPACER.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut labels as *mut WIDGET_LABEL)
            .offset(C2Rust_Unnamed_9::LABEL_IP.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(C2Rust_Unnamed_5::CHOICE_IPSTYLE.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(
                C2Rust_Unnamed_5::CHOICE_IPBACKGROUND.0 as ::core::ffi::c_int as isize,
            ) as *mut WIDGET,
        (&raw mut labels as *mut WIDGET_LABEL)
            .offset(C2Rust_Unnamed_9::LABEL_SPACER.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut labels as *mut WIDGET_LABEL)
            .offset(C2Rust_Unnamed_9::LABEL_SCAN.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(C2Rust_Unnamed_5::CHOICE_SCANMENU.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(C2Rust_Unnamed_5::CHOICE_SLVSHIELD.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(C2Rust_Unnamed_5::CHOICE_SCANSTYLE.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(C2Rust_Unnamed_5::CHOICE_SCANSPHERE.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(C2Rust_Unnamed_5::CHOICE_SCANTINT.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(
                C2Rust_Unnamed_5::CHOICE_LANDERSTYLE.0 as ::core::ffi::c_int as isize,
            ) as *mut WIDGET,
        (&raw mut buttons as *mut WIDGET_BUTTON)
            .offset(C2Rust_Unnamed_8::BTN_QUITSUBMENU.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        ::core::ptr::null_mut::<WIDGET>(),
    ];
    audio_widgets = [
        (&raw mut sliders as *mut WIDGET_SLIDER)
            .offset(C2Rust_Unnamed_7::SLIDER_MUSVOLUME.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut sliders as *mut WIDGET_SLIDER)
            .offset(C2Rust_Unnamed_7::SLIDER_SFXVOLUME.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut sliders as *mut WIDGET_SLIDER)
            .offset(C2Rust_Unnamed_7::SLIDER_SPCHVOLUME.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut labels as *mut WIDGET_LABEL)
            .offset(C2Rust_Unnamed_9::LABEL_SPACER.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(C2Rust_Unnamed_5::CHOICE_POSAUDIO.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(C2Rust_Unnamed_5::CHOICE_SNDDRIVER.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(C2Rust_Unnamed_5::CHOICE_SNDQUALITY.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut labels as *mut WIDGET_LABEL)
            .offset(C2Rust_Unnamed_9::LABEL_SPACER.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut buttons as *mut WIDGET_BUTTON)
            .offset(C2Rust_Unnamed_8::BTN_QUITSUBMENU.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        ::core::ptr::null_mut::<WIDGET>(),
    ];
    graphics_widgets = [
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(C2Rust_Unnamed_5::CHOICE_GRAPHICS.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(C2Rust_Unnamed_5::CHOICE_RESOLUTION.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut textentries as *mut WIDGET_TEXTENTRY)
            .offset(C2Rust_Unnamed_10::TEXT_CUSTMRES.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(C2Rust_Unnamed_5::CHOICE_ASPRATIO.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(C2Rust_Unnamed_5::CHOICE_DISPLAY.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut sliders as *mut WIDGET_SLIDER)
            .offset(C2Rust_Unnamed_7::SLIDER_GAMMA.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(C2Rust_Unnamed_5::CHOICE_SCALER.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(C2Rust_Unnamed_5::CHOICE_SCANLINE.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut choices as *mut WIDGET_CHOICE)
            .offset(C2Rust_Unnamed_5::CHOICE_SHOWFPS.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut labels as *mut WIDGET_LABEL)
            .offset(C2Rust_Unnamed_9::LABEL_SPACER.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut buttons as *mut WIDGET_BUTTON)
            .offset(C2Rust_Unnamed_8::BTN_QUITSUBMENU.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        ::core::ptr::null_mut::<WIDGET>(),
    ];
    main_widgets = [
        (&raw mut buttons as *mut WIDGET_BUTTON)
            .offset(C2Rust_Unnamed_8::BTN_GFXMENU.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut buttons as *mut WIDGET_BUTTON)
            .offset(C2Rust_Unnamed_8::BTN_ENGMENU.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut buttons as *mut WIDGET_BUTTON)
            .offset(C2Rust_Unnamed_8::BTN_VISMENU.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut buttons as *mut WIDGET_BUTTON)
            .offset(C2Rust_Unnamed_8::BTN_SNDMENU.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut buttons as *mut WIDGET_BUTTON)
            .offset(C2Rust_Unnamed_8::BTN_MUSMENU.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut buttons as *mut WIDGET_BUTTON)
            .offset(C2Rust_Unnamed_8::BTN_KEYMENU.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut buttons as *mut WIDGET_BUTTON)
            .offset(C2Rust_Unnamed_8::BTN_QOLMENU.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut buttons as *mut WIDGET_BUTTON)
            .offset(C2Rust_Unnamed_8::BTN_ADVMENU.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut buttons as *mut WIDGET_BUTTON)
            .offset(C2Rust_Unnamed_8::BTN_CHTMENU.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut labels as *mut WIDGET_LABEL)
            .offset(C2Rust_Unnamed_9::LABEL_SPACER.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        (&raw mut buttons as *mut WIDGET_BUTTON)
            .offset(C2Rust_Unnamed_8::BTN_QUITSETUP.0 as ::core::ffi::c_int as isize)
            as *mut WIDGET,
        ::core::ptr::null_mut::<WIDGET>(),
    ];
}
// Explicit engine initialization replaces platform constructor tables.
