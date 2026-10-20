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

// Rust translation of src/uqm/planets/pl_stuff.c.
extern "C" {
    pub type font_desc;
    pub type string_table_entry;
    pub type context_desc;
    fn TFB_Random() -> DWORD;
    static mut pSolarSysState: *mut SOLARSYS_STATE;
    static mut PlanetContext: CONTEXT;
    fn RenderPlanetSphere(Frame: FRAME, offset: ::core::ffi::c_int, doThrob: BOOLEAN);
    fn SetShieldThrobEffect(
        FromFrame: FRAME,
        offset: ::core::ffi::c_int,
        ToFrame: FRAME,
    );
    fn GetTimeCounter() -> TimeCount;
    static mut ScreenWidth: ::core::ffi::c_int;
    static mut ScreenHeight: ::core::ffi::c_int;
    fn SetContext(Context: CONTEXT) -> CONTEXT;
    fn SetContextForeGroundColor(Color: Color) -> Color;
    fn SetContextFGFrame(Frame: FRAME) -> FRAME;
    fn SetContextClipRect(pRect: *mut RECT) -> BOOLEAN;
    fn GetContextClipRect(pRect: *mut RECT) -> BOOLEAN;
    fn SetContextDrawMode(_: DrawMode) -> DrawMode;
    fn DrawStamp(pStamp: *mut STAMP);
    fn DrawFilledRectangle(pRect: *mut RECT);
    fn BatchGraphics();
    fn UnbatchGraphics();
    fn CreateDrawable(
        CreateFlags: CREATE_FLAGS,
        width: SIZE,
        height: SIZE,
        num_frames: COUNT,
    ) -> DRAWABLE;
    fn DestroyDrawable(Drawable: DRAWABLE) -> BOOLEAN;
    fn GetFrameRect(Frame: FRAME, pRect: *mut RECT) -> BOOLEAN;
    fn SetAbsFrameIndex(Frame: FRAME, FrameIndex: COUNT) -> FRAME;
    fn CaptureDrawable(Drawable: DRAWABLE) -> FRAME;
    fn ReleaseDrawable(Frame: FRAME) -> DRAWABLE;
    static mut OffScreenContext: CONTEXT;
    fn SetGraphicScale(scale: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn SetGraphicScaleMode(mode: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn RepairBackRect(pRect: *mut RECT);
    static mut optWhichShield: ::core::ffi::c_int;
    fn exp(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn SleepThreadUntil(wakeTime: TimeCount);
}
pub type uint8 = ::core::ffi::c_uchar;
pub type sint8 = ::core::ffi::c_schar;
pub type uint16 = ::core::ffi::c_ushort;
pub type sint16 = ::core::ffi::c_short;
pub type uint32 = ::core::ffi::c_uint;
pub type BYTE = uint8;
pub type SBYTE = sint8;
pub type UWORD = uint16;
pub type SWORD = sint16;
pub type DWORD = uint32;
pub type COUNT = UWORD;
pub type SIZE = SWORD;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct BOOLEAN(pub ::core::ffi::c_uint);
impl BOOLEAN {
    pub const FALSE: Self = Self(0);
    pub const TRUE: Self = Self(1);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct planet_desc {
    pub rand_seed: DWORD,
    pub data_index: BYTE,
    pub NumPlanets: BYTE,
    pub radius: SIZE,
    pub location: POINT,
    pub temp_color: Color,
    pub NextIndex: COUNT,
    pub image: STAMP,
    pub pPrevDesc: *mut PLANET_DESC,
}
pub type PLANET_DESC = planet_desc;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct node_info {
    pub loc_pt: POINT,
    pub density: COUNT,
    pub r#type: COUNT,
}
pub type NODE_INFO = node_info;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct planet_orbit {
    pub TopoZoomFrame: FRAME,
    pub lpTopoData: *mut SBYTE,
    pub SphereFrame: FRAME,
    pub ObjectFrame: FRAME,
    pub TintFrame: FRAME,
    pub TintColor: Color,
    pub TopoColors: *mut Color,
    pub ScratchArray: *mut Color,
    pub WorkFrame: FRAME,
}
pub type PLANET_ORBIT = planet_orbit;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct solarsys_state {
    pub InputFunc: Option<unsafe extern "C" fn(*mut solarsys_state) -> BOOLEAN>,
    pub InIpFlight: BOOLEAN,
    pub WaitIntersect: COUNT,
    pub SunDesc: [PLANET_DESC; 1],
    pub PlanetDesc: [PLANET_DESC; 16],
    pub MoonDesc: [PLANET_DESC; 4],
    pub pBaseDesc: *mut PLANET_DESC,
    pub pOrbitalDesc: *mut PLANET_DESC,
    pub FirstPlanetIndex: SIZE,
    pub LastPlanetIndex: SIZE,
    pub turn_counter: BYTE,
    pub turn_wait: BYTE,
    pub thrust_counter: BYTE,
    pub max_ship_speed: BYTE,
    pub XlatRef: STRING,
    pub XlatPtr: *const ::core::ffi::c_void,
    pub OrbitalCMap: COLORMAP,
    pub SysInfo: SYSTEM_INFO,
    pub genFuncs: *const GenerateFunctions,
    pub PlanetSideFrame: [FRAME; 6],
    pub TopoFrame: FRAME,
    pub Orbit: PLANET_ORBIT,
    pub InOrbit: BOOLEAN,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GenerateFunctions {
    pub initNpcs: InitNpcsFunction,
    pub reinitNpcs: ReinitNpcsFunction,
    pub uninitNpcs: UninitNpcsFunction,
    pub generatePlanets: GeneratePlanetsFunction,
    pub generateMoons: GenerateMoonsFunction,
    pub generateName: GenerateNameFunction,
    pub generateOrbital: GenerateOrbitalFunction,
    pub generateMinerals: GenerateMineralsFunction,
    pub generateEnergy: GenerateEnergyFunction,
    pub generateLife: GenerateLifeFunction,
    pub pickupMinerals: PickupMineralsFunction,
    pub pickupEnergy: PickupEnergyFunction,
    pub pickupLife: PickupLifeFunction,
}
pub type PickupLifeFunction = Option<
    unsafe extern "C" fn(*mut SOLARSYS_STATE, *mut PLANET_DESC, COUNT) -> bool,
>;
pub type SOLARSYS_STATE = solarsys_state;
pub type PickupEnergyFunction = Option<
    unsafe extern "C" fn(*mut SOLARSYS_STATE, *mut PLANET_DESC, COUNT) -> bool,
>;
pub type PickupMineralsFunction = Option<
    unsafe extern "C" fn(*mut SOLARSYS_STATE, *mut PLANET_DESC, COUNT) -> bool,
>;
pub type GenerateLifeFunction = Option<
    unsafe extern "C" fn(
        *const SOLARSYS_STATE,
        *const PLANET_DESC,
        COUNT,
        *mut NODE_INFO,
    ) -> COUNT,
>;
pub type GenerateEnergyFunction = Option<
    unsafe extern "C" fn(
        *const SOLARSYS_STATE,
        *const PLANET_DESC,
        COUNT,
        *mut NODE_INFO,
    ) -> COUNT,
>;
pub type GenerateMineralsFunction = Option<
    unsafe extern "C" fn(
        *const SOLARSYS_STATE,
        *const PLANET_DESC,
        COUNT,
        *mut NODE_INFO,
    ) -> COUNT,
>;
pub type GenerateOrbitalFunction = Option<
    unsafe extern "C" fn(*mut SOLARSYS_STATE, *mut PLANET_DESC) -> bool,
>;
pub type GenerateNameFunction = Option<
    unsafe extern "C" fn(*const SOLARSYS_STATE, *const PLANET_DESC) -> bool,
>;
pub type GenerateMoonsFunction = Option<
    unsafe extern "C" fn(*mut SOLARSYS_STATE, *mut PLANET_DESC) -> bool,
>;
pub type GeneratePlanetsFunction = Option<
    unsafe extern "C" fn(*mut SOLARSYS_STATE) -> bool,
>;
pub type UninitNpcsFunction = Option<unsafe extern "C" fn(*mut SOLARSYS_STATE) -> bool>;
pub type ReinitNpcsFunction = Option<unsafe extern "C" fn(*mut SOLARSYS_STATE) -> bool>;
pub type InitNpcsFunction = Option<unsafe extern "C" fn(*mut SOLARSYS_STATE) -> bool>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SYSTEM_INFO {
    pub StarSize: BYTE,
    pub StarIntensity: BYTE,
    pub StarEnergy: UWORD,
    pub PlanetInfo: PLANET_INFO,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PLANET_INFO {
    pub AxialTilt: SIZE,
    pub Tectonics: UWORD,
    pub Weather: UWORD,
    pub PlanetDensity: UWORD,
    pub PlanetRadius: UWORD,
    pub SurfaceGravity: UWORD,
    pub SurfaceTemperature: SIZE,
    pub RotationPeriod: UWORD,
    pub AtmoDensity: UWORD,
    pub LifeChance: SIZE,
    pub PlanetToSunDist: UWORD,
    pub PlanDataPtr: *const PlanetFrame,
    pub ScanSeed: [DWORD; 3],
    pub ScanRetrieveMask: [DWORD; 3],
    pub DiscoveryString: STRING,
    pub LanderFont: FONT,
    pub LanderFontEff: FRAME,
}
pub type FONT = *mut FONT_DESC;
pub type FONT_DESC = font_desc;
pub type STRING = *mut STRING_TABLE_ENTRY_DESC;
pub type STRING_TABLE_ENTRY_DESC = string_table_entry;
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
pub type COLORMAP = STRING;
pub type CONTEXT_DESC = context_desc;
pub type DRAWABLE_DESC = drawable_desc;
pub type CONTEXT = *mut CONTEXT_DESC;
pub type DRAWABLE = *mut DRAWABLE_DESC;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rect {
    pub corner: POINT,
    pub extent: EXTENT,
}
pub type RECT = rect;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct DrawKind(pub ::core::ffi::c_uint);
impl DrawKind {
    pub const DRAW_REPLACE: Self = Self(0);
    pub const DRAW_ADDITIVE: Self = Self(1);
    pub const DRAW_ALPHA: Self = Self(2);
    pub const DRAW_DEFAULT: Self = Self(0);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DrawMode {
    pub kind: BYTE,
    pub factor: SWORD,
}
pub type TimeCount = DWORD;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed {
    pub const TFB_SCALE_STEP: Self = Self(0);
    pub const TFB_SCALE_NEAREST: Self = Self(1);
    pub const TFB_SCALE_BILINEAR: Self = Self(2);
    pub const TFB_SCALE_TRILINEAR: Self = Self(3);
}
static mut rotFrameIndex: ::core::ffi::c_int = 0;
static mut rotDirection: ::core::ffi::c_int = 0;
static mut throbShield: bool = false;
static mut rotPointIndex: ::core::ffi::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn DrawPlanetSphere(
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
) {
    let mut s: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    let mut Orbit: *mut PLANET_ORBIT = &raw mut (*pSolarSysState).Orbit;
    s.origin.x = x as COORD;
    s.origin.y = y as COORD;
    BatchGraphics();
    s.frame = (*Orbit).SphereFrame;
    DrawStamp(&raw mut s);
    if !(*Orbit).ObjectFrame.is_null() {
        s.frame = (*Orbit).ObjectFrame;
        DrawStamp(&raw mut s);
    }
    UnbatchGraphics();
}
#[no_mangle]
pub unsafe extern "C" fn DrawDefaultPlanetSphere() {
    let mut oldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    oldContext = SetContext(PlanetContext);
    DrawPlanetSphere(
        (ScreenWidth - 64 as ::core::ffi::c_int
            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
            - 14 as ::core::ffi::c_int) / 2 as ::core::ffi::c_int,
        (ScreenHeight - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
            - 13 as ::core::ffi::c_int
            - (75 as ::core::ffi::c_int - 0 as ::core::ffi::c_int)
            - 5 as ::core::ffi::c_int - 9 as ::core::ffi::c_int)
            / 2 as ::core::ffi::c_int,
    );
    SetContext(oldContext);
}
#[no_mangle]
pub unsafe extern "C" fn InitSphereRotation(
    mut direction: ::core::ffi::c_int,
    mut shielded: BOOLEAN,
) {
    let mut Orbit: *mut PLANET_ORBIT = &raw mut (*pSolarSysState).Orbit;
    rotDirection = direction;
    rotPointIndex = 0 as ::core::ffi::c_int;
    throbShield = shielded.0 != 0 && optWhichShield == 0x1 as ::core::ffi::c_int;
    if throbShield {
        (*Orbit).WorkFrame = (*Orbit).ObjectFrame;
        (*Orbit).ObjectFrame = CaptureDrawable(
            CreateDrawable(
                (((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as CREATE_FLAGS
                    as ::core::ffi::c_int
                    | ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                        as CREATE_FLAGS as ::core::ffi::c_int) as CREATE_FLAGS,
                (*(*Orbit).ObjectFrame).Bounds.width,
                (*(*Orbit).ObjectFrame).Bounds.height,
                2 as COUNT,
            ),
        );
    }
    rotFrameIndex = 1 as ::core::ffi::c_int;
    PrepareNextRotationFrame();
}
#[no_mangle]
pub unsafe extern "C" fn UninitSphereRotation() {
    let mut Orbit: *mut PLANET_ORBIT = &raw mut (*pSolarSysState).Orbit;
    if !(*Orbit).WorkFrame.is_null() {
        DestroyDrawable(ReleaseDrawable((*Orbit).ObjectFrame));
        (*Orbit).ObjectFrame = (*Orbit).WorkFrame;
        (*Orbit).WorkFrame = ::core::ptr::null_mut::<FRAME_DESC>();
    }
}
#[no_mangle]
pub unsafe extern "C" fn PrepareNextRotationFrame() {
    let mut Orbit: *mut PLANET_ORBIT = &raw mut (*pSolarSysState).Orbit;
    rotFrameIndex ^= 1 as ::core::ffi::c_int;
    rotPointIndex += rotDirection;
    if rotPointIndex < 0 as ::core::ffi::c_int {
        rotPointIndex = ScreenWidth - 64 as ::core::ffi::c_int
            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
            - 14 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    } else if rotPointIndex
        >= ScreenWidth - 64 as ::core::ffi::c_int
            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
            - 14 as ::core::ffi::c_int
    {
        rotPointIndex = 0 as ::core::ffi::c_int;
    }
    (*Orbit).SphereFrame = SetAbsFrameIndex(
        (*Orbit).SphereFrame,
        rotFrameIndex as COUNT,
    );
    RenderPlanetSphere(
        (*Orbit).SphereFrame,
        rotPointIndex,
        BOOLEAN(throbShield as ::core::ffi::c_uint),
    );
    if throbShield {
        (*Orbit).ObjectFrame = SetAbsFrameIndex(
            (*Orbit).ObjectFrame,
            rotFrameIndex as COUNT,
        );
        SetShieldThrobEffect((*Orbit).WorkFrame, rotPointIndex, (*Orbit).ObjectFrame);
    }
}
#[no_mangle]
pub unsafe extern "C" fn ZoomInPlanetSphere() {
    let mut Orbit: *mut PLANET_ORBIT = &raw mut (*pSolarSysState).Orbit;
    let base: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
    let mut dx: ::core::ffi::c_int = 0;
    let mut dy: ::core::ffi::c_int = 0;
    let mut oldScale: ::core::ffi::c_int = 0;
    let mut oldMode: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut frameCount: ::core::ffi::c_int = 0;
    let mut zoomCorner: ::core::ffi::c_int = 0;
    let mut frameRect: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut repairRect: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    let mut NextTime: TimeCount = 0;
    frameCount = 840 as ::core::ffi::c_int * 6 as ::core::ffi::c_int
        / 5 as ::core::ffi::c_int
        / (840 as ::core::ffi::c_int / 24 as ::core::ffi::c_int);
    zoomCorner = TFB_Random() as ::core::ffi::c_int;
    dx = 1 as ::core::ffi::c_int
        - (zoomCorner & 1 as ::core::ffi::c_int) * 2 as ::core::ffi::c_int;
    dy = 1 as ::core::ffi::c_int - (zoomCorner & 2 as ::core::ffi::c_int);
    if !(*Orbit).ObjectFrame.is_null() {
        GetFrameRect((*Orbit).ObjectFrame, &raw mut frameRect);
    } else {
        GetFrameRect((*Orbit).SphereFrame, &raw mut frameRect);
    }
    repairRect = frameRect;
    i = 0 as ::core::ffi::c_int;
    while i <= frameCount {
        let mut scale: ::core::ffi::c_double = 0.;
        let mut pt: POINT = POINT { x: 0, y: 0 };
        NextTime = GetTimeCounter()
            .wrapping_add(
                (840 as ::core::ffi::c_int / 24 as ::core::ffi::c_int) as TimeCount,
            );
        if i < frameCount {
            scale = 1.134f64
                - exp(
                    -2.0f64 * i as ::core::ffi::c_double
                        / frameCount as ::core::ffi::c_double,
                );
        } else {
            scale = 1.0f64;
        }
        pt.x = ((ScreenWidth - 64 as ::core::ffi::c_int
            - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
            - 14 as ::core::ffi::c_int) / 2 as ::core::ffi::c_int
            + (dx as ::core::ffi::c_double * (1.0f64 - scale)
                * ((ScreenWidth - 64 as ::core::ffi::c_int
                    - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    - 14 as ::core::ffi::c_int) * 6 as ::core::ffi::c_int
                    / 10 as ::core::ffi::c_int) as ::core::ffi::c_double + 0.5f64)
                as ::core::ffi::c_int) as COORD;
        pt.y = ((ScreenHeight - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
            - 13 as ::core::ffi::c_int
            - (75 as ::core::ffi::c_int - 0 as ::core::ffi::c_int)
            - 5 as ::core::ffi::c_int - 9 as ::core::ffi::c_int)
            / 2 as ::core::ffi::c_int
            + (dy as ::core::ffi::c_double * (1.0f64 - scale)
                * ((ScreenHeight - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    - 13 as ::core::ffi::c_int
                    - (75 as ::core::ffi::c_int - 0 as ::core::ffi::c_int)
                    - 5 as ::core::ffi::c_int) * 6 as ::core::ffi::c_int
                    / 10 as ::core::ffi::c_int) as ::core::ffi::c_double + 0.5f64)
                as ::core::ffi::c_int) as COORD;
        SetContext(PlanetContext);
        BatchGraphics();
        if i > 0 as ::core::ffi::c_int {
            RepairBackRect(&raw mut repairRect);
        }
        oldMode = SetGraphicScaleMode(
            C2Rust_Unnamed::TFB_SCALE_BILINEAR.0 as ::core::ffi::c_int,
        );
        oldScale = SetGraphicScale(
            (base as ::core::ffi::c_double * scale + 0.5f64) as ::core::ffi::c_int,
        );
        DrawPlanetSphere(pt.x as ::core::ffi::c_int, pt.y as ::core::ffi::c_int);
        SetGraphicScale(oldScale);
        SetGraphicScaleMode(oldMode);
        UnbatchGraphics();
        repairRect.corner.x = (pt.x as ::core::ffi::c_int
            + frameRect.corner.x as ::core::ffi::c_int) as COORD;
        repairRect.corner.y = (pt.y as ::core::ffi::c_int
            + frameRect.corner.y as ::core::ffi::c_int) as COORD;
        PrepareNextRotationFrame();
        SleepThreadUntil(NextTime);
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn RotatePlanetSphere(mut keepRate: BOOLEAN) {
    static mut NextTime: TimeCount = 0;
    let mut Now: TimeCount = GetTimeCounter();
    if keepRate.0 != 0 && Now < NextTime {
        return;
    }
    NextTime = Now
        .wrapping_add(
            (840 as ::core::ffi::c_int * 12 as ::core::ffi::c_int
                / (ScreenWidth - 64 as ::core::ffi::c_int
                    - 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    - 14 as ::core::ffi::c_int)) as TimeCount,
        );
    DrawDefaultPlanetSphere();
    PrepareNextRotationFrame();
}
unsafe extern "C" fn renderTintFrame(mut tintColor: Color) {
    let mut Orbit: *mut PLANET_ORBIT = &raw mut (*pSolarSysState).Orbit;
    let mut oldContext: CONTEXT = ::core::ptr::null_mut::<CONTEXT_DESC>();
    let mut mode: DrawMode = DrawMode { kind: 0, factor: 0 };
    let mut oldMode: DrawMode = DrawMode { kind: 0, factor: 0 };
    let mut s: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    let mut r: RECT = RECT {
        corner: POINT { x: 0, y: 0 },
        extent: EXTENT { width: 0, height: 0 },
    };
    oldContext = SetContext(OffScreenContext);
    SetContextFGFrame((*Orbit).TintFrame);
    SetContextClipRect(::core::ptr::null_mut::<RECT>());
    GetContextClipRect(&raw mut r);
    s.origin.x = 0 as COORD;
    s.origin.y = 0 as COORD;
    s.frame = (*pSolarSysState).TopoFrame;
    DrawStamp(&raw mut s);
    mode = MAKE_DRAW_MODE(
        DrawKind::DRAW_ADDITIVE,
        (0xff as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as SWORD,
    );
    oldMode = SetContextDrawMode(mode);
    SetContextForeGroundColor(tintColor);
    DrawFilledRectangle(&raw mut r);
    SetContextDrawMode(oldMode);
    SetContext(oldContext);
}
#[no_mangle]
pub unsafe extern "C" fn DrawPlanet(
    mut tintY: ::core::ffi::c_int,
    mut tintColor: Color,
) {
    let mut s: STAMP = STAMP {
        origin: POINT { x: 0, y: 0 },
        frame: ::core::ptr::null_mut::<FRAME_DESC>(),
    };
    let mut Orbit: *mut PLANET_ORBIT = &raw mut (*pSolarSysState).Orbit;
    s.origin.x = 0 as COORD;
    s.origin.y = 0 as COORD;
    BatchGraphics();
    if sameColor(tintColor, colorFromRgb15(0 as BYTE, 0 as BYTE, 0 as BYTE)) {
        s.frame = (*pSolarSysState).TopoFrame;
        DrawStamp(&raw mut s);
    } else {
        let mut tintFrame: FRAME = (*Orbit).TintFrame;
        let mut height: ::core::ffi::c_int = (*tintFrame).Bounds.height
            as ::core::ffi::c_int;
        if !sameColor(tintColor, (*Orbit).TintColor) {
            renderTintFrame(tintColor);
            (*Orbit).TintColor = tintColor;
        }
        if tintY < height - 1 as ::core::ffi::c_int {
            s.frame = (*pSolarSysState).TopoFrame;
            DrawStamp(&raw mut s);
        }
        if tintY >= 0 as ::core::ffi::c_int {
            let mut oldClipRect: RECT = RECT {
                corner: POINT { x: 0, y: 0 },
                extent: EXTENT { width: 0, height: 0 },
            };
            let mut clipRect: RECT = RECT {
                corner: POINT { x: 0, y: 0 },
                extent: EXTENT { width: 0, height: 0 },
            };
            GetContextClipRect(&raw mut oldClipRect);
            clipRect = oldClipRect;
            clipRect.extent.height = (tintY + 1 as ::core::ffi::c_int) as COORD;
            SetContextClipRect(&raw mut clipRect);
            s.frame = tintFrame;
            DrawStamp(&raw mut s);
            SetContextClipRect(&raw mut oldClipRect);
        }
    }
    UnbatchGraphics();
}
#[inline]
unsafe extern "C" fn sameColor(mut c1: Color, mut c2: Color) -> bool {
    return c1.r as ::core::ffi::c_int == c2.r as ::core::ffi::c_int
        && c1.g as ::core::ffi::c_int == c2.g as ::core::ffi::c_int
        && c1.b as ::core::ffi::c_int == c2.b as ::core::ffi::c_int
        && c1.a as ::core::ffi::c_int == c2.a as ::core::ffi::c_int;
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
unsafe extern "C" fn MAKE_DRAW_MODE(mut kind: DrawKind, mut factor: SWORD) -> DrawMode {
    let mut mode: DrawMode = DrawMode { kind: 0, factor: 0 };
    mode.kind = kind.0 as BYTE;
    mode.factor = factor;
    return mode;
}
