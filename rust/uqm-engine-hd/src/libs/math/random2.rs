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

// This file contains variants of the random functions in random.c
// that store the state of the RNG in a context, allowing for multiple
// independant RNGs to be used simultaneously.
// The RNG behavior itself is the same.

// Rust translation of src/libs/math/random2.c.
extern "C" {
    pub type frame_desc;
    fn HMalloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn HFree(p: *mut ::core::ffi::c_void);
    static mut optSeedType: ::core::ffi::c_int;
    static mut GlobData: GLOBDATA;
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
pub type size_t = ::core::ffi::c_ulong;
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
    pub const OPTVAL_PRIME: Self = Self(0);
    pub const OPTVAL_PLANET: Self = Self(1);
    pub const OPTVAL_MRQ: Self = Self(2);
    pub const OPTVAL_STAR: Self = Self(3);
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
pub struct RandomContext {
    pub seed: DWORD,
}
#[no_mangle]
pub unsafe extern "C" fn RandomContext_New() -> *mut RandomContext {
    let mut result: *mut RandomContext = HMalloc(
        ::core::mem::size_of::<RandomContext>() as size_t,
    ) as *mut RandomContext;
    (*result).seed = 12345 as DWORD;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn RandomContext_Set(mut Context: DWORD) -> *mut RandomContext {
    let mut result: *mut RandomContext = HMalloc(
        ::core::mem::size_of::<RandomContext>() as size_t,
    ) as *mut RandomContext;
    (*result).seed = Context;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn RandomContext_Delete(mut context: *mut RandomContext) {
    HFree(context as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn RandomContext_Copy(
    mut source: *const RandomContext,
) -> *mut RandomContext {
    let mut result: *mut RandomContext = HMalloc(
        ::core::mem::size_of::<RandomContext>() as size_t,
    ) as *mut RandomContext;
    *result = *source;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn RandomContext_Random(mut context: *mut RandomContext) -> DWORD {
    (*context).seed = ((if optSeedType
        == C2Rust_Unnamed::OPTVAL_PLANET.0 as ::core::ffi::c_int
        && if GlobData.SIS_state.Seed < 3 as ::core::ffi::c_int
            || GlobData.SIS_state.Seed > 2147483644 as ::core::ffi::c_int
        {
            0 as ::core::ffi::c_int
        } else {
            1 as ::core::ffi::c_int
        } != 0
    {
        GlobData.SIS_state.Seed
    } else {
        16807 as ::core::ffi::c_int
    }) as ::core::ffi::c_uint)
        .wrapping_mul(
            (*context)
                .seed
                .wrapping_rem(
                    (0xffffffff as ::core::ffi::c_uint)
                        .wrapping_div(2 as ::core::ffi::c_uint)
                        .wrapping_div(
                            (if optSeedType
                                == C2Rust_Unnamed::OPTVAL_PLANET.0 as ::core::ffi::c_int
                                && if GlobData.SIS_state.Seed < 3 as ::core::ffi::c_int
                                    || GlobData.SIS_state.Seed
                                        > 2147483644 as ::core::ffi::c_int
                                {
                                    0 as ::core::ffi::c_int
                                } else {
                                    1 as ::core::ffi::c_int
                                } != 0
                            {
                                GlobData.SIS_state.Seed
                            } else {
                                16807 as ::core::ffi::c_int
                            }) as ::core::ffi::c_uint,
                        ),
                ),
        )
        .wrapping_sub(
            (0xffffffff as ::core::ffi::c_uint)
                .wrapping_div(2 as ::core::ffi::c_uint)
                .wrapping_rem(
                    (if optSeedType
                        == C2Rust_Unnamed::OPTVAL_PLANET.0 as ::core::ffi::c_int
                        && if GlobData.SIS_state.Seed < 3 as ::core::ffi::c_int
                            || GlobData.SIS_state.Seed > 2147483644 as ::core::ffi::c_int
                        {
                            0 as ::core::ffi::c_int
                        } else {
                            1 as ::core::ffi::c_int
                        } != 0
                    {
                        GlobData.SIS_state.Seed
                    } else {
                        16807 as ::core::ffi::c_int
                    }) as ::core::ffi::c_uint,
                )
                .wrapping_mul(
                    (*context)
                        .seed
                        .wrapping_div(
                            (0xffffffff as ::core::ffi::c_uint)
                                .wrapping_div(2 as ::core::ffi::c_uint)
                                .wrapping_div(
                                    (if optSeedType
                                        == C2Rust_Unnamed::OPTVAL_PLANET.0 as ::core::ffi::c_int
                                        && if GlobData.SIS_state.Seed < 3 as ::core::ffi::c_int
                                            || GlobData.SIS_state.Seed
                                                > 2147483644 as ::core::ffi::c_int
                                        {
                                            0 as ::core::ffi::c_int
                                        } else {
                                            1 as ::core::ffi::c_int
                                        } != 0
                                    {
                                        GlobData.SIS_state.Seed
                                    } else {
                                        16807 as ::core::ffi::c_int
                                    }) as ::core::ffi::c_uint,
                                ),
                        ),
                ),
        ) as DWORD;
    if (*context).seed
        > (0xffffffff as ::core::ffi::c_uint).wrapping_div(2 as ::core::ffi::c_uint)
    {
        (*context).seed = (*context)
            .seed
            .wrapping_sub(
                (0xffffffff as ::core::ffi::c_uint)
                    .wrapping_div(2 as ::core::ffi::c_uint),
            );
    } else if (*context).seed == 0 as DWORD {
        (*context).seed = 1 as DWORD;
    }
    return (*context).seed;
}
#[no_mangle]
pub unsafe extern "C" fn RandomContext_SeedRandom(
    mut context: *mut RandomContext,
    mut new_seed: DWORD,
) -> DWORD {
    let mut old_seed: DWORD = 0;
    if new_seed == 0 as DWORD {
        new_seed = 1 as DWORD;
    } else if new_seed
        > (0xffffffff as ::core::ffi::c_uint).wrapping_div(2 as ::core::ffi::c_uint)
    {
        new_seed = new_seed
            .wrapping_sub(
                (0xffffffff as ::core::ffi::c_uint)
                    .wrapping_div(2 as ::core::ffi::c_uint),
            );
    }
    old_seed = (*context).seed;
    (*context).seed = new_seed;
    return old_seed;
}
#[no_mangle]
pub unsafe extern "C" fn RandomContext_GetSeed(
    mut context: *mut RandomContext,
) -> DWORD {
    return (*context).seed;
}
#[no_mangle]
pub unsafe extern "C" fn RandomContext_FastRandom(mut seed: DWORD) -> DWORD {
    let mut res: DWORD = ((if optSeedType
        == C2Rust_Unnamed::OPTVAL_PLANET.0 as ::core::ffi::c_int
        && if GlobData.SIS_state.Seed < 3 as ::core::ffi::c_int
            || GlobData.SIS_state.Seed > 2147483644 as ::core::ffi::c_int
        {
            0 as ::core::ffi::c_int
        } else {
            1 as ::core::ffi::c_int
        } != 0
    {
        GlobData.SIS_state.Seed
    } else {
        16807 as ::core::ffi::c_int
    }) as DWORD)
        .wrapping_mul(
            seed
                .wrapping_rem(
                    (0xffffffff as DWORD)
                        .wrapping_div(2 as DWORD)
                        .wrapping_div(
                            (if optSeedType
                                == C2Rust_Unnamed::OPTVAL_PLANET.0 as ::core::ffi::c_int
                                && if GlobData.SIS_state.Seed < 3 as ::core::ffi::c_int
                                    || GlobData.SIS_state.Seed
                                        > 2147483644 as ::core::ffi::c_int
                                {
                                    0 as ::core::ffi::c_int
                                } else {
                                    1 as ::core::ffi::c_int
                                } != 0
                            {
                                GlobData.SIS_state.Seed
                            } else {
                                16807 as ::core::ffi::c_int
                            }) as DWORD,
                        ),
                ),
        )
        .wrapping_sub(
            (0xffffffff as DWORD)
                .wrapping_div(2 as DWORD)
                .wrapping_rem(
                    (if optSeedType
                        == C2Rust_Unnamed::OPTVAL_PLANET.0 as ::core::ffi::c_int
                        && if GlobData.SIS_state.Seed < 3 as ::core::ffi::c_int
                            || GlobData.SIS_state.Seed > 2147483644 as ::core::ffi::c_int
                        {
                            0 as ::core::ffi::c_int
                        } else {
                            1 as ::core::ffi::c_int
                        } != 0
                    {
                        GlobData.SIS_state.Seed
                    } else {
                        16807 as ::core::ffi::c_int
                    }) as DWORD,
                )
                .wrapping_mul(
                    seed
                        .wrapping_div(
                            (0xffffffff as DWORD)
                                .wrapping_div(2 as DWORD)
                                .wrapping_div(
                                    (if optSeedType
                                        == C2Rust_Unnamed::OPTVAL_PLANET.0 as ::core::ffi::c_int
                                        && if GlobData.SIS_state.Seed < 3 as ::core::ffi::c_int
                                            || GlobData.SIS_state.Seed
                                                > 2147483644 as ::core::ffi::c_int
                                        {
                                            0 as ::core::ffi::c_int
                                        } else {
                                            1 as ::core::ffi::c_int
                                        } != 0
                                    {
                                        GlobData.SIS_state.Seed
                                    } else {
                                        16807 as ::core::ffi::c_int
                                    }) as DWORD,
                                ),
                        ),
                ),
        );
    if res > (0xffffffff as ::core::ffi::c_uint).wrapping_div(2 as ::core::ffi::c_uint) {
        res = res
            .wrapping_sub(
                (0xffffffff as ::core::ffi::c_uint)
                    .wrapping_div(2 as ::core::ffi::c_uint),
            );
    } else if res == 0 as DWORD {
        res = 1 as DWORD;
    }
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn RangeMinMax(
    mut min: ::core::ffi::c_int,
    mut max: ::core::ffi::c_int,
    mut rand: DWORD,
) -> ::core::ffi::c_int {
    if min > max {
        return max;
    }
    return rand
        .wrapping_rem((max + 1 as ::core::ffi::c_int - min) as DWORD)
        .wrapping_add(min as DWORD) as ::core::ffi::c_int;
}
