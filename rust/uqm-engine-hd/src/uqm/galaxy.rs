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

/* background starfield - used to generate agalaxy.asm */

// Rust translation of src/uqm/galaxy.c.
extern "C" {
    pub type frame_desc;
    fn HFree(p: *mut ::core::ffi::c_void);
    fn HCalloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn fmin(_: ::core::ffi::c_double, _: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn ApplyMask(layer: FRAME, base: FRAME, mode: DrawMode, fill: *mut Color);
    fn SetAbsFrameIndex(Frame: FRAME, FrameIndex: COUNT) -> FRAME;
    fn DecFrameIndex(Frame: FRAME) -> FRAME;
    static mut optMeleeScale: ::core::ffi::c_int;
    static mut resolutionFactor: ::core::ffi::c_uint;
    static mut optWindowType: DWORD;
    static mut CanvasWidth: ::core::ffi::c_int;
    static mut CanvasHeight: ::core::ffi::c_int;
    static mut DisplayFreeList: COUNT;
    static mut DisplayArray: [PRIMITIVE; 330];
    static mut zoom_out: COUNT;
    static mut DisplayLinks: PRIM_LINKS;
    fn getGameStateUint(name: *const ::core::ffi::c_char) -> DWORD;
    static mut GlobData: GLOBDATA;
    fn inHQSpace() -> BOOLEAN;
    fn inQuasiSpace() -> BOOLEAN;
    static mut stars_in_space: FRAME;
    static mut scenery: FRAME;
    fn InsertPrim(pLinks: *mut PRIM_LINKS, primIndex: COUNT, iPI: COUNT);
    fn TFB_Random() -> DWORD;
    fn log_add(_: log_Level, fmt: *const ::core::ffi::c_char, ...);
    fn GetColorMapColor(ColorMapIndex: COUNT, ColorIndex: COUNT) -> Color;
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
pub struct Color {
    pub r: BYTE,
    pub g: BYTE,
    pub b: BYTE,
    pub a: BYTE,
}
pub type FRAME_DESC = frame_desc;
pub type FRAME = *mut FRAME_DESC;
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct gfx_object(pub ::core::ffi::c_uint);
impl gfx_object {
    pub const POINT_PRIM: Self = Self(0);
    pub const STAMP_PRIM: Self = Self(1);
    pub const STAMPFILL_PRIM: Self = Self(2);
    pub const LINE_PRIM: Self = Self(3);
    pub const TEXT_PRIM: Self = Self(4);
    pub const RECT_PRIM: Self = Self(5);
    pub const RECTFILL_PRIM: Self = Self(6);
    pub const POINT_PRIM_HD: Self = Self(7);
    pub const NUM_PRIMS: Self = Self(8);
}
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
pub struct DrawKind(pub ::core::ffi::c_uint);
impl DrawKind {
    pub const DRAW_REPLACE: Self = Self(0);
    pub const DRAW_ADDITIVE: Self = Self(1);
    pub const DRAW_ALPHA: Self = Self(2);
    pub const DRAW_MULTIPLY: Self = Self(3);
    pub const DRAW_OVERLAY: Self = Self(4);
    pub const DRAW_SCREEN: Self = Self(5);
    pub const DRAW_GRAYSCALE: Self = Self(6);
    pub const DRAW_LINEARBURN: Self = Self(7);
    pub const DRAW_HYPTOQUAS: Self = Self(8);
    pub const DRAW_DESATURATE: Self = Self(9);
    pub const DRAW_DEFAULT: Self = Self(0);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DrawMode {
    pub kind: BYTE,
    pub factor: SWORD,
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
pub struct VIEW_STATE(pub ::core::ffi::c_uint);
impl VIEW_STATE {
    pub const VIEW_STABLE: Self = Self(0);
    pub const VIEW_SCROLL: Self = Self(1);
    pub const VIEW_CHANGE: Self = Self(2);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct STAR_BLOCK {
    pub min_star_index: COUNT,
    pub num_stars: COUNT,
    pub star_array: *mut DPOINT,
    pub pmin_star: *mut DPOINT,
    pub plast_star: *mut DPOINT,
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed {
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
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_0 {
    pub const TFB_SCALE_STEP: Self = Self(0);
    pub const TFB_SCALE_NEAREST: Self = Self(1);
    pub const TFB_SCALE_BILINEAR: Self = Self(2);
    pub const TFB_SCALE_TRILINEAR: Self = Self(3);
}
#[inline]
unsafe extern "C" fn MAKE_DRAW_MODE(mut kind: DrawKind, mut factor: SWORD) -> DrawMode {
    let mut mode: DrawMode = DrawMode { kind: 0, factor: 0 };
    mode.kind = kind.0 as BYTE;
    mode.factor = factor;
    return mode;
}
static mut sceneries: BOOLEAN = BOOLEAN(0 as ::core::ffi::c_int as ::core::ffi::c_uint);
static mut HSOffsets: *mut BYTE = ::core::ptr::null_mut::<BYTE>();
#[no_mangle]
pub static mut SpaceOrg: DPOINT = DPOINT { x: 0, y: 0 };
static mut log_star_array: [DPOINT; 180] = [DPOINT { x: 0, y: 0 }; 180];
#[no_mangle]
pub static mut StarBlock: [STAR_BLOCK; 3] = [STAR_BLOCK {
    min_star_index: 0,
    num_stars: 0,
    star_array: ::core::ptr::null_mut::<DPOINT>(),
    pmin_star: ::core::ptr::null_mut::<DPOINT>(),
    plast_star: ::core::ptr::null_mut::<DPOINT>(),
}; 3];
unsafe extern "C" fn SortStarBlock(mut pStarBlock: *mut STAR_BLOCK) {
    let mut i: COUNT = 0;
    i = 0 as COUNT;
    while (i as ::core::ffi::c_int) < (*pStarBlock).num_stars as ::core::ffi::c_int {
        let mut j: COUNT = 0;
        j = ((*pStarBlock).num_stars as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
            as COUNT;
        while j as ::core::ffi::c_int > i as ::core::ffi::c_int {
            if (*(*pStarBlock).star_array.offset(i as isize)).y
                > (*(*pStarBlock).star_array.offset(j as isize)).y
            {
                let mut temp: DPOINT = DPOINT { x: 0, y: 0 };
                temp = *(*pStarBlock).star_array.offset(i as isize);
                *(*pStarBlock).star_array.offset(i as isize) = *(*pStarBlock)
                    .star_array
                    .offset(j as isize);
                *(*pStarBlock).star_array.offset(j as isize) = temp;
            }
            j = j.wrapping_sub(1);
        }
        i = i.wrapping_add(1);
    }
    (*pStarBlock).min_star_index = 0 as COUNT;
    (*pStarBlock).pmin_star = (*pStarBlock).star_array.offset(0isize);
    (*pStarBlock).plast_star = (*pStarBlock)
        .star_array
        .offset(
            ((*pStarBlock).num_stars as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                as isize,
        );
}
unsafe extern "C" fn WrapStarBlock(mut plane: SIZE, mut dx: SDWORD, mut dy: SDWORD) {
    let mut i: COUNT = 0;
    let mut ppt: *mut DPOINT = ::core::ptr::null_mut::<DPOINT>();
    let mut offs_y: SDWORD = 0;
    let mut num_stars: COUNT = 0;
    let mut pStarBlock: *mut STAR_BLOCK = ::core::ptr::null_mut::<STAR_BLOCK>();
    pStarBlock = (&raw mut StarBlock as *mut STAR_BLOCK).offset(plane as isize);
    i = (*pStarBlock).min_star_index;
    ppt = (*pStarBlock).pmin_star;
    num_stars = (*pStarBlock).num_stars;
    if dy < 0 as ::core::ffi::c_int {
        let mut first: COUNT = 0;
        first = i;
        dy = -dy;
        offs_y = ((((CanvasHeight
            - (if if optWindowType == 1 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                16 as SDWORD
            } else {
                0 as SDWORD
            } << resolutionFactor) * 2 as SDWORD) << 2 as ::core::ffi::c_int)
            << 3 as ::core::ffi::c_int) << plane as ::core::ffi::c_int) - dy;
        while (*ppt).y < dy {
            (*ppt).y += offs_y;
            (*ppt).x += dx;
            i = i.wrapping_add(1);
            if (i as ::core::ffi::c_int) < num_stars as ::core::ffi::c_int {
                ppt = ppt.offset(1);
            } else {
                i = 0 as COUNT;
                ppt = (*pStarBlock).star_array.offset(0isize);
            }
            if i as ::core::ffi::c_int == first as ::core::ffi::c_int {
                return;
            }
        }
        (*pStarBlock).min_star_index = i;
        (*pStarBlock).pmin_star = ppt;
        if first as ::core::ffi::c_int <= i as ::core::ffi::c_int {
            i = (num_stars as ::core::ffi::c_int - i as ::core::ffi::c_int) as COUNT;
            loop {
                (*ppt).y -= dy;
                (*ppt).x += dx;
                ppt = ppt.offset(1);
                i = i.wrapping_sub(1);
                if i == 0 {
                    break;
                }
            }
            ppt = (*pStarBlock).star_array.offset(0isize);
        }
        if first as ::core::ffi::c_int > i as ::core::ffi::c_int {
            i = (first as ::core::ffi::c_int - i as ::core::ffi::c_int) as COUNT;
            loop {
                (*ppt).y -= dy;
                (*ppt).x += dx;
                ppt = ppt.offset(1);
                i = i.wrapping_sub(1);
                if i == 0 {
                    break;
                }
            }
        }
    } else {
        let mut last: COUNT = 0;
        ppt = ppt.offset(-1);
        let c2rust_fresh0 = i;
        i = i.wrapping_sub(1);
        if c2rust_fresh0 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            i = (num_stars as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as COUNT;
            ppt = (*pStarBlock).plast_star;
        }
        last = i;
        if dy > 0 as ::core::ffi::c_int {
            offs_y = ((((CanvasHeight
                - (if if optWindowType == 1 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    16 as SDWORD
                } else {
                    0 as SDWORD
                } << resolutionFactor) * 2 as SDWORD) << 2 as ::core::ffi::c_int)
                << 3 as ::core::ffi::c_int) << plane as ::core::ffi::c_int) - dy;
            while (*ppt).y >= offs_y {
                (*ppt).y -= offs_y;
                (*ppt).x += dx;
                let c2rust_fresh1 = i;
                i = i.wrapping_sub(1);
                if c2rust_fresh1 as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                    ppt = ppt.offset(-1);
                } else {
                    i = (num_stars as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                        as COUNT;
                    ppt = (*pStarBlock).plast_star;
                }
                if i as ::core::ffi::c_int == last as ::core::ffi::c_int {
                    return;
                }
            }
            (*pStarBlock).pmin_star = ppt.offset(1 as ::core::ffi::c_int as isize);
            (*pStarBlock).min_star_index = (i as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int) as COUNT;
            if (*pStarBlock).min_star_index as ::core::ffi::c_int
                == num_stars as ::core::ffi::c_int
            {
                (*pStarBlock).min_star_index = 0 as COUNT;
                (*pStarBlock).pmin_star = (*pStarBlock).star_array.offset(0isize);
            }
        }
        if last as ::core::ffi::c_int >= i as ::core::ffi::c_int {
            i = i.wrapping_add(1);
            loop {
                (*ppt).y += dy;
                (*ppt).x += dx;
                ppt = ppt.offset(-1);
                i = i.wrapping_sub(1);
                if i == 0 {
                    break;
                }
            }
            i = (num_stars as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as COUNT;
            ppt = (*pStarBlock).plast_star;
        }
        if (last as ::core::ffi::c_int) < i as ::core::ffi::c_int {
            i = (i as ::core::ffi::c_int - last as ::core::ffi::c_int) as COUNT;
            loop {
                (*ppt).y += dy;
                (*ppt).x += dx;
                ppt = ppt.offset(-1);
                i = i.wrapping_sub(1);
                if i == 0 {
                    break;
                }
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn InitStarOffsets() {
    let mut i: COUNT = 0;
    let mut offsets: *mut BYTE = ::core::ptr::null_mut::<BYTE>();
    HSOffsets = HCalloc(
        ::core::mem::size_of::<BYTE>()
            .wrapping_mul(
                (30 as ::core::ffi::c_int + 60 as ::core::ffi::c_int) as size_t,
            ),
    ) as *mut BYTE;
    i = (30 as ::core::ffi::c_int + 60 as ::core::ffi::c_int) as COUNT;
    offsets = HSOffsets;
    while i as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
        *offsets = (TFB_Random() & 0x1f as DWORD) as BYTE;
        i = i.wrapping_sub(1);
        offsets = offsets.offset(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn UninitStarOffsets() {
    HFree(HSOffsets as *mut ::core::ffi::c_void);
    HSOffsets = ::core::ptr::null_mut::<BYTE>();
}
#[no_mangle]
pub unsafe extern "C" fn InitGalaxy() {
    let mut i: COUNT = 0;
    let mut factor: COUNT = 0;
    let mut ppt: *mut DPOINT = ::core::ptr::null_mut::<DPOINT>();
    let mut Links: PRIM_LINKS = 0;
    let mut medStarColor: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    let mut smallStarColor: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    sceneries = BOOLEAN(
        (if GlobData.SIS_state.Extended as ::core::ffi::c_int != 0 {
            BOOLEAN::TRUE.0 as ::core::ffi::c_int
        } else {
            BOOLEAN::FALSE.0 as ::core::ffi::c_int
        } != 0
            && uqm_lua_checked!(getGameStateUint(
                b"URQUAN_PROTECTING_SAMATRA\0".as_ptr() as *const ::core::ffi::c_char,
            ), ()) != 0
            && GlobData.Game_state.CurrentActivity as BYTE as ::core::ffi::c_int
                != C2Rust_Unnamed::IN_HYPERSPACE.0 as ::core::ffi::c_int
            && GlobData.Game_state.CurrentActivity as BYTE as ::core::ffi::c_int
                != C2Rust_Unnamed::IN_LAST_BATTLE.0 as ::core::ffi::c_int)
            as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
    log_add(
        log_Level::log_Debug,
        b"InitGalaxy(): transition_width = %d, transition_height = %d\0".as_ptr()
            as *const ::core::ffi::c_char,
        ((CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
            - (if if optWindowType == 1 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                16 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } << resolutionFactor) * 2 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
            << 2 as ::core::ffi::c_int,
        ((CanvasHeight
            - (if if optWindowType == 1 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                16 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } << resolutionFactor) * 2 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
            << 2 as ::core::ffi::c_int,
    );
    Links = ((0xffff as ::core::ffi::c_int as COUNT as DWORD) << 16 as ::core::ffi::c_int
        | 0xffff as ::core::ffi::c_int as COUNT as DWORD) as PRIM_LINKS;
    factor = (2 as ::core::ffi::c_int + 3 as ::core::ffi::c_int
        + (3 as ::core::ffi::c_int - 3 as ::core::ffi::c_int)) as COUNT;
    medStarColor = GetColorMapColor(0x1 as COUNT, 0x9 as COUNT);
    smallStarColor = GetColorMapColor(
        (if inHQSpace().0 != 0 {
            0x35 as ::core::ffi::c_int
        } else {
            0x1 as ::core::ffi::c_int
        }) as COUNT,
        (if inHQSpace().0 != 0 {
            0xfe as ::core::ffi::c_int
        } else {
            0x7 as ::core::ffi::c_int
        }) as COUNT,
    );
    if if resolutionFactor != 2 as ::core::ffi::c_uint {
        BOOLEAN::FALSE.0 as ::core::ffi::c_int
    } else {
        BOOLEAN::TRUE.0 as ::core::ffi::c_int
    } != 0
    {
        uqm_lua_checked!(ApplyMask(
            ::core::ptr::null_mut::<FRAME_DESC>(),
            DecFrameIndex(DecFrameIndex(stars_in_space)),
            MAKE_DRAW_MODE(DrawKind::DRAW_REPLACE, 0xff as SWORD),
            &raw mut smallStarColor,
        ), ());
    }
    i = 0 as COUNT;
    ppt = &raw mut log_star_array as *mut DPOINT;
    while (i as ::core::ffi::c_int)
        < 30 as ::core::ffi::c_int + 60 as ::core::ffi::c_int + 90 as ::core::ffi::c_int
    {
        let mut p: COUNT = 0;
        p = DisplayFreeList;
        DisplayFreeList = (DisplayArray[DisplayFreeList as usize].Links
            >> 16 as ::core::ffi::c_int) as UWORD as COUNT;
        DisplayArray[p as usize].flags = 0 as BYTE;
        if i as ::core::ffi::c_int == 30 as ::core::ffi::c_int
            || i as ::core::ffi::c_int
                == 30 as ::core::ffi::c_int + 60 as ::core::ffi::c_int
        {
            factor = factor.wrapping_add(1);
        }
        (*ppt).x = TFB_Random() as UWORD as ::core::ffi::c_int
            % (CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
                - (if if optWindowType == 1 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    16 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                } << resolutionFactor) * 2 as ::core::ffi::c_int)
            << factor as ::core::ffi::c_int;
        (*ppt).y = TFB_Random() as UWORD as ::core::ffi::c_int
            % (CanvasHeight
                - (if if optWindowType == 1 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    16 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                } << resolutionFactor) * 2 as ::core::ffi::c_int)
            << factor as ::core::ffi::c_int;
        if (i as ::core::ffi::c_int)
            < 30 as ::core::ffi::c_int + 60 as ::core::ffi::c_int
        {
            DisplayArray[p as usize].Type = gfx_object::STAMP_PRIM.0
                as ::core::ffi::c_int as GRAPHICS_PRIM;
            DisplayArray[p as usize].color = medStarColor;
            DisplayArray[p as usize].Object.Stamp.frame = if sceneries.0 != 0
                && i as ::core::ffi::c_int
                    == 30 as ::core::ffi::c_int + 60 as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int
            {
                scenery
            } else {
                stars_in_space
            };
            if if resolutionFactor != 2 as ::core::ffi::c_uint {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } != 0 && uqm_lua_checked!(inQuasiSpace(), ()).0 != 0
            {
                DisplayArray[p as usize].flags = ((1 as ::core::ffi::c_int)
                    << 1 as ::core::ffi::c_int) as BYTE;
            }
        } else {
            DisplayArray[p as usize].Type = (if if resolutionFactor
                != 2 as ::core::ffi::c_uint
            {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } != 0
            {
                gfx_object::STAMP_PRIM.0 as ::core::ffi::c_int
            } else {
                gfx_object::POINT_PRIM.0 as ::core::ffi::c_int
            }) as GRAPHICS_PRIM;
            DisplayArray[p as usize].color = smallStarColor;
            DisplayArray[p as usize].flags = ((1 as ::core::ffi::c_int)
                << 0 as ::core::ffi::c_int) as BYTE;
            DisplayArray[p as usize].Object.Stamp.frame = DecFrameIndex(
                DecFrameIndex(stars_in_space),
            );
        }
        InsertPrim(&raw mut Links, p, Links as COUNT);
        i = i.wrapping_add(1);
        ppt = ppt.offset(1);
    }
    SortStarBlock((&raw mut StarBlock as *mut STAR_BLOCK).offset(0isize));
    SortStarBlock((&raw mut StarBlock as *mut STAR_BLOCK).offset(1isize));
    SortStarBlock((&raw mut StarBlock as *mut STAR_BLOCK).offset(2isize));
}
unsafe extern "C" fn CmpMovePoints(
    mut pt1: *const POINT,
    mut pt2: *const DPOINT,
    mut dx: SDWORD,
    mut dy: SDWORD,
    mut reduction: SIZE,
) -> BOOLEAN {
    if optMeleeScale == C2Rust_Unnamed_0::TFB_SCALE_STEP.0 as ::core::ffi::c_int {
        return BOOLEAN(
            ((*pt1).x as ::core::ffi::c_int
                != (*pt2).x - dx >> reduction as ::core::ffi::c_int
                || (*pt1).y as ::core::ffi::c_int
                    != (*pt2).y - dy >> reduction as ::core::ffi::c_int)
                as ::core::ffi::c_int as ::core::ffi::c_uint,
        )
    } else {
        return BOOLEAN(
            ((*pt1).x as ::core::ffi::c_int
                != ((*pt2).x - dx << 8 as ::core::ffi::c_int)
                    / reduction as ::core::ffi::c_int
                || (*pt1).y as ::core::ffi::c_int
                    != ((*pt2).y - dy << 8 as ::core::ffi::c_int)
                        / reduction as ::core::ffi::c_int) as ::core::ffi::c_int
                as ::core::ffi::c_uint,
        )
    };
}
#[no_mangle]
pub unsafe extern "C" fn MoveGalaxy(
    mut view_state: VIEW_STATE,
    mut dx: SDWORD,
    mut dy: SDWORD,
) {
    if view_state.0 != VIEW_STATE::VIEW_STABLE.0 {
        let mut pprim: *mut PRIMITIVE = ::core::ptr::null_mut::<PRIMITIVE>();
        let mut offsets: *mut BYTE = ::core::ptr::null_mut::<BYTE>();
        let mut reduction: COUNT = zoom_out;
        let mut iss: COUNT = 0;
        let mut i: COUNT = 0;
        let mut wrap_around: ::core::ffi::c_int = 0;
        let mut ppt: *mut DPOINT = ::core::ptr::null_mut::<DPOINT>();
        static mut star_counts: [COUNT; 3] = [30 as COUNT, 60 as COUNT, 90 as COUNT];
        if inHQSpace().0 != 0 {
            let mut i_0: COUNT = 0;
            pprim = &raw mut DisplayArray as *mut PRIMITIVE;
            offsets = HSOffsets;
            i_0 = 30 as COUNT;
            while i_0 as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                *offsets = (*offsets as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                    & 0x1f as ::core::ffi::c_int) as BYTE;
                (*pprim).Object.Stamp.frame = SetAbsFrameIndex(
                    (*pprim).Object.Stamp.frame,
                    (if *offsets as ::core::ffi::c_int > 10 as ::core::ffi::c_int {
                        48 as ::core::ffi::c_int
                    } else {
                        *offsets as ::core::ffi::c_int + 37 as ::core::ffi::c_int
                    }) as COUNT,
                );
                i_0 = i_0.wrapping_sub(1);
                pprim = pprim.offset(1);
                offsets = offsets.offset(1);
            }
            i_0 = 60 as COUNT;
            while i_0 as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                *offsets = (*offsets as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                    & 0xf as ::core::ffi::c_int) as BYTE;
                (*pprim).Object.Stamp.frame = SetAbsFrameIndex(
                    (*pprim).Object.Stamp.frame,
                    (if *offsets as ::core::ffi::c_int > 10 as ::core::ffi::c_int {
                        48 as ::core::ffi::c_int
                    } else {
                        *offsets as ::core::ffi::c_int + 26 as ::core::ffi::c_int
                    }) as COUNT,
                );
                i_0 = i_0.wrapping_sub(1);
                pprim = pprim.offset(1);
                offsets = offsets.offset(1);
            }
            dx <<= 3 as ::core::ffi::c_int;
            dy <<= 3 as ::core::ffi::c_int;
            WrapStarBlock(2 as SIZE, dx, dy);
            WrapStarBlock(1 as SIZE, dx, dy);
            WrapStarBlock(0 as SIZE, dx, dy);
            dx = ((((CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
                - (if if optWindowType == 1 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    16 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                } << resolutionFactor) * 2 as ::core::ffi::c_int)
                << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int) as COORD as ::core::ffi::c_int
                - (((CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
                    - (if if optWindowType == 1 as DWORD {
                        BOOLEAN::TRUE.0 as ::core::ffi::c_int
                    } else {
                        BOOLEAN::FALSE.0 as ::core::ffi::c_int
                    } != 0
                    {
                        16 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    } << resolutionFactor) * 2 as ::core::ffi::c_int)
                    << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    >> 3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                        - 2 as ::core::ffi::c_int)) as SDWORD;
            dy = ((((CanvasHeight
                - (if if optWindowType == 1 as DWORD {
                    BOOLEAN::TRUE.0 as ::core::ffi::c_int
                } else {
                    BOOLEAN::FALSE.0 as ::core::ffi::c_int
                } != 0
                {
                    16 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                } << resolutionFactor) * 2 as ::core::ffi::c_int)
                << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int) as COORD as ::core::ffi::c_int
                - (((CanvasHeight
                    - (if if optWindowType == 1 as DWORD {
                        BOOLEAN::TRUE.0 as ::core::ffi::c_int
                    } else {
                        BOOLEAN::FALSE.0 as ::core::ffi::c_int
                    } != 0
                    {
                        16 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    } << resolutionFactor) * 2 as ::core::ffi::c_int)
                    << 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    >> 3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                        - 2 as ::core::ffi::c_int)) as SDWORD;
            reduction = (if optMeleeScale != 0 {
                ((1 as ::core::ffi::c_int)
                    << 8 as ::core::ffi::c_int + 3 as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
            } else {
                2 as ::core::ffi::c_int + 2 as ::core::ffi::c_int
            }) as COUNT;
        } else {
            let mut scale: COUNT = (if optMeleeScale != 0 {
                reduction as ::core::ffi::c_int >> 9 as ::core::ffi::c_int
            } else {
                reduction as ::core::ffi::c_int
            }) as COUNT;
            if view_state.0 == VIEW_STATE::VIEW_CHANGE.0 {
                let mut star_object: [GRAPHICS_PRIM; 2] = [
                    (if if resolutionFactor != 2 as ::core::ffi::c_uint {
                        BOOLEAN::FALSE.0 as ::core::ffi::c_int
                    } else {
                        BOOLEAN::TRUE.0 as ::core::ffi::c_int
                    } == 0 && optMeleeScale != 0
                        && reduction as ::core::ffi::c_int > 256 as ::core::ffi::c_int
                    {
                        gfx_object::POINT_PRIM.0 as ::core::ffi::c_int
                    } else {
                        gfx_object::STAMP_PRIM.0 as ::core::ffi::c_int
                    }) as GRAPHICS_PRIM,
                    (if if resolutionFactor != 2 as ::core::ffi::c_uint {
                        BOOLEAN::FALSE.0 as ::core::ffi::c_int
                    } else {
                        BOOLEAN::TRUE.0 as ::core::ffi::c_int
                    } == 0 && optMeleeScale != 0
                    {
                        gfx_object::POINT_PRIM.0 as ::core::ffi::c_int
                    } else if if resolutionFactor != 2 as ::core::ffi::c_uint {
                        BOOLEAN::FALSE.0 as ::core::ffi::c_int
                    } else {
                        BOOLEAN::TRUE.0 as ::core::ffi::c_int
                    } != 0 && scale as ::core::ffi::c_int == 1 as ::core::ffi::c_int
                    {
                        ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int)
                            << 7 as ::core::ffi::c_int
                            | gfx_object::STAMP_PRIM.0 as ::core::ffi::c_int
                    } else {
                        gfx_object::STAMP_PRIM.0 as ::core::ffi::c_int
                    }) as GRAPHICS_PRIM,
                ];
                let mut star_frame: [FRAME; 2] = [
                    SetAbsFrameIndex(
                        stars_in_space,
                        (if optMeleeScale != 0 {
                            scale as ::core::ffi::c_int
                        } else {
                            fmin(
                                scale as ::core::ffi::c_double,
                                1 as ::core::ffi::c_int as ::core::ffi::c_double,
                            ) as COUNT as ::core::ffi::c_int
                        }) as COUNT,
                    ),
                    SetAbsFrameIndex(
                        stars_in_space,
                        fmin(
                            (scale as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                as ::core::ffi::c_double,
                            2 as ::core::ffi::c_int as ::core::ffi::c_double,
                        ) as COUNT,
                    ),
                ];
                iss = 0 as COUNT;
                pprim = &raw mut DisplayArray as *mut PRIMITIVE;
                while (iss as ::core::ffi::c_int) < 2 as ::core::ffi::c_int {
                    i = star_counts[iss as usize];
                    while i as ::core::ffi::c_uint
                        > sceneries.0 & iss as ::core::ffi::c_uint
                    {
                        (*pprim).Type = (star_object[iss as usize] as ::core::ffi::c_int
                            & 0xf as ::core::ffi::c_int) as GRAPHICS_PRIM;
                        (*pprim).flags = (star_object[iss as usize] as ::core::ffi::c_int
                            >> 7 as ::core::ffi::c_int) as BYTE;
                        (*pprim).Object.Stamp.frame = star_frame[iss as usize];
                        i = i.wrapping_sub(1);
                        pprim = pprim.offset(1);
                    }
                    iss = iss.wrapping_add(1);
                }
                if i as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                    (*pprim).Object.Stamp.frame = SetAbsFrameIndex(scenery, scale);
                }
            }
            WrapStarBlock(2 as SIZE, dx, dy);
            WrapStarBlock(1 as SIZE, dx, dy);
            WrapStarBlock(0 as SIZE, dx, dy);
            dx = SpaceOrg.x;
            dy = SpaceOrg.y;
            reduction = (if optMeleeScale != 0 {
                (reduction as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
            } else {
                reduction as ::core::ffi::c_int + 2 as ::core::ffi::c_int
            }) as COUNT;
        }
        ppt = &raw mut log_star_array as *mut DPOINT;
        iss = 0 as COUNT;
        pprim = &raw mut DisplayArray as *mut PRIMITIVE;
        wrap_around = ((CanvasWidth - ((64 as ::core::ffi::c_int) << resolutionFactor)
            - (if if optWindowType == 1 as DWORD {
                BOOLEAN::TRUE.0 as ::core::ffi::c_int
            } else {
                BOOLEAN::FALSE.0 as ::core::ffi::c_int
            } != 0
            {
                16 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } << resolutionFactor) * 2 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
            << 3 as ::core::ffi::c_int;
        while (iss as ::core::ffi::c_int) < 3 as ::core::ffi::c_int
            && (view_state.0 == VIEW_STATE::VIEW_CHANGE.0
                || CmpMovePoints(
                        &raw mut (*pprim).Object.Point,
                        ppt,
                        dx,
                        dy,
                        reduction as SIZE,
                    )
                    .0 != 0)
        {
            i = star_counts[iss as usize];
            while i as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                (*ppt).x = (if (*ppt).x < 0 as ::core::ffi::c_int {
                    (*ppt).x + wrap_around
                } else if (*ppt).x >= wrap_around {
                    (*ppt).x - wrap_around
                } else {
                    (*ppt).x
                }) as DWORD as SDWORD;
                if optMeleeScale
                    == C2Rust_Unnamed_0::TFB_SCALE_STEP.0 as ::core::ffi::c_int
                {
                    (*pprim).Object.Point.x = ((*ppt).x - dx
                        >> reduction as ::core::ffi::c_int) as COORD;
                    (*pprim).Object.Point.y = ((*ppt).y - dy
                        >> reduction as ::core::ffi::c_int) as COORD;
                } else {
                    (*pprim).Object.Point.x = (((*ppt).x - dx << 8 as ::core::ffi::c_int)
                        / reduction as ::core::ffi::c_int) as COORD;
                    (*pprim).Object.Point.y = (((*ppt).y - dy << 8 as ::core::ffi::c_int)
                        / reduction as ::core::ffi::c_int) as COORD;
                }
                i = i.wrapping_sub(1);
                pprim = pprim.offset(1);
                ppt = ppt.offset(1);
            }
            if optMeleeScale == C2Rust_Unnamed_0::TFB_SCALE_STEP.0 as ::core::ffi::c_int
            {
                reduction = reduction.wrapping_add(1);
            } else {
                reduction = ((reduction as ::core::ffi::c_int)
                    << 1 as ::core::ffi::c_int) as COUNT;
            }
            iss = iss.wrapping_add(1);
            wrap_around <<= 1 as ::core::ffi::c_int;
            dx <<= 1 as ::core::ffi::c_int;
            dy <<= 1 as ::core::ffi::c_int;
        }
    }
    DisplayLinks = ((0 as ::core::ffi::c_int as DWORD) << 16 as ::core::ffi::c_int
        | (30 as ::core::ffi::c_int + 60 as ::core::ffi::c_int + 90 as ::core::ffi::c_int
            - 1 as ::core::ffi::c_int) as UWORD as DWORD) as PRIM_LINKS;
}
#[no_mangle]
pub unsafe extern "C" fn SetStarPoint(mut pt: POINT, mut i: COUNT) {
    log_star_array[i as usize].x = pt.x as SDWORD;
    log_star_array[i as usize].y = pt.y as SDWORD;
}
#[export_name = "uqm_c2rust_init_cc8c9db2fa0eecb9"]
unsafe extern "C" fn c2rust_run_static_initializers() {
    StarBlock = [
        STAR_BLOCK {
            min_star_index: 0 as COUNT,
            num_stars: 30 as COUNT,
            star_array: (&raw mut log_star_array as *mut DPOINT).offset(0isize),
            pmin_star: ::core::ptr::null_mut::<DPOINT>(),
            plast_star: ::core::ptr::null_mut::<DPOINT>(),
        },
        STAR_BLOCK {
            min_star_index: 0 as COUNT,
            num_stars: 60 as COUNT,
            star_array: (&raw mut log_star_array as *mut DPOINT).offset(30isize),
            pmin_star: ::core::ptr::null_mut::<DPOINT>(),
            plast_star: ::core::ptr::null_mut::<DPOINT>(),
        },
        STAR_BLOCK {
            min_star_index: 0 as COUNT,
            num_stars: 90 as COUNT,
            star_array: (&raw mut log_star_array as *mut DPOINT)
                .offset((30 as ::core::ffi::c_int + 60 as ::core::ffi::c_int) as isize),
            pmin_star: ::core::ptr::null_mut::<DPOINT>(),
            plast_star: ::core::ptr::null_mut::<DPOINT>(),
        },
    ];
}
// Explicit engine initialization replaces platform constructor tables.
