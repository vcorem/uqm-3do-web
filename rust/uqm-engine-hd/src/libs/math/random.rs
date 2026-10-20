///Copyright Paul Reiche, Fred Ford. 1992-2002

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

 /****************************************************************************
* FILE: random.c
* DESC: a library of random number generators for general purpose use.
*
* References:
* "Random Number Generators: Good ones are hard to find" S.K.Park & K.W.Miller
* Communications of the ACM, Vol31 Number 10, October 1988, Pp 1192-1201
*
* HISTORY: Created 1/23/1989
* LAST CHANGED:
*
* Copyright (c) 1989, Robert Leyland and Fred Ford
****************************************************************************/

/* ----------------------------INCLUDES----------------------------------- */

// Rust translation of src/libs/math/random.c.
extern "C" {
    pub type frame_desc;
    fn puts(_: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn printf(_: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
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
static mut seed: DWORD = 12345 as DWORD;
#[no_mangle]
pub unsafe extern "C" fn TFB_Random() -> DWORD {
    seed = ((if optSeedType == C2Rust_Unnamed::OPTVAL_PLANET.0 as ::core::ffi::c_int
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
            seed
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
                    seed
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
    if seed > (0xffffffff as ::core::ffi::c_uint).wrapping_div(2 as ::core::ffi::c_uint)
    {
        seed = seed
            .wrapping_sub(
                (0xffffffff as ::core::ffi::c_uint)
                    .wrapping_div(2 as ::core::ffi::c_uint),
            );
        return seed;
    } else if seed != 0 {
        return seed
    } else {
        seed = 1 as DWORD;
        return seed;
    };
}
#[no_mangle]
pub unsafe extern "C" fn TFB_SeedRandom(mut new_seed: DWORD) -> DWORD {
    let mut old_seed: DWORD = 0;
    if new_seed as ::core::ffi::c_ulong == 0 as ::core::ffi::c_ulong {
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
    old_seed = seed;
    seed = new_seed;
    return old_seed;
}
#[no_mangle]
pub unsafe extern "C" fn printBits(size: size_t, ptr: *const ::core::ffi::c_void) {
    let mut b: *mut ::core::ffi::c_uchar = ptr as *mut ::core::ffi::c_uchar;
    let mut byte: ::core::ffi::c_uchar = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    i = size.wrapping_sub(1 as size_t) as ::core::ffi::c_int;
    while i >= 0 as ::core::ffi::c_int {
        j = 7 as ::core::ffi::c_int;
        while j >= 0 as ::core::ffi::c_int {
            byte = (*b.offset(i as isize) as ::core::ffi::c_int >> j
                & 1 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
            printf(
                b"%u\0".as_ptr() as *const ::core::ffi::c_char,
                byte as ::core::ffi::c_int,
            );
            j -= 1;
        }
        i -= 1;
    }
    puts(b"\0".as_ptr() as *const ::core::ffi::c_char);
}
