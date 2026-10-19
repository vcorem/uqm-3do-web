/*
 *  Copyright 2006  Serge van den Boom <svdb@stack.nl>
 *
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
 *  Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA  02111-1307  USA
 */

// Rust translation of src/uqm/supermelee/netplay/netinput.c.
extern "C" {
    fn log_add(_: log_Level, fmt: *const ::core::ffi::c_char, ...);
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn malloc(_: size_t) -> *mut ::core::ffi::c_void;
    fn free(_: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
pub type uint8 = ::core::ffi::c_uchar;
pub type UBYTE = uint8;
pub type BATTLE_INPUT_STATE = UBYTE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BattleInputBuffer {
    pub buf: *mut BATTLE_INPUT_STATE,
    pub maxSize: size_t,
    pub first: size_t,
    pub size: size_t,
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
static mut battleInputBuffers: [BattleInputBuffer; 2] = [BattleInputBuffer {
    buf: ::core::ptr::null_mut::<BATTLE_INPUT_STATE>(),
    maxSize: 0,
    first: 0,
    size: 0,
}; 2];
static mut BattleInput_inputDelay: size_t = 0;
#[no_mangle]
pub unsafe extern "C" fn setBattleInputDelay(mut delay: size_t) {
    BattleInput_inputDelay = delay;
}
#[no_mangle]
pub unsafe extern "C" fn getBattleInputDelay() -> size_t {
    return BattleInput_inputDelay;
}
unsafe extern "C" fn BattleInputBuffer_init(
    mut bib: *mut BattleInputBuffer,
    mut bufSize: size_t,
) {
    (*bib).buf = malloc(
        bufSize.wrapping_mul(::core::mem::size_of::<BATTLE_INPUT_STATE>()),
    ) as *mut BATTLE_INPUT_STATE;
    (*bib).maxSize = bufSize;
    (*bib).first = 0 as size_t;
    (*bib).size = 0 as size_t;
}
unsafe extern "C" fn BattleInputBuffer_uninit(mut bib: *mut BattleInputBuffer) {
    if !(*bib).buf.is_null() {
        free((*bib).buf as *mut ::core::ffi::c_void);
        (*bib).buf = ::core::ptr::null_mut::<BATTLE_INPUT_STATE>();
    }
    (*bib).maxSize = 0 as size_t;
    (*bib).first = 0 as size_t;
    (*bib).size = 0 as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn initBattleInputBuffers() {
    let mut player: size_t = 0;
    let mut bufSize: ::core::ffi::c_int = BattleInput_inputDelay
        .wrapping_mul(2 as size_t)
        .wrapping_add(2 as size_t) as ::core::ffi::c_int;
    player = 0 as size_t;
    while player < 2 as size_t {
        let mut bib: *mut BattleInputBuffer = (&raw mut battleInputBuffers
            as *mut BattleInputBuffer)
            .offset(player as isize);
        BattleInputBuffer_init(bib, bufSize as size_t);
        let mut i: size_t = 0;
        i = 0 as size_t;
        while i < BattleInput_inputDelay {
            BattleInputBuffer_push(bib, 0 as ::core::ffi::c_int as BATTLE_INPUT_STATE);
            i = i.wrapping_add(1);
        }
        player = player.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn uninitBattleInputBuffers() {
    let mut player: size_t = 0;
    player = 0 as size_t;
    while player < 2 as size_t {
        let mut bib: *mut BattleInputBuffer = ::core::ptr::null_mut::<
            BattleInputBuffer,
        >();
        bib = (&raw mut battleInputBuffers as *mut BattleInputBuffer)
            .offset(player as isize);
        BattleInputBuffer_uninit(bib);
        player = player.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn BattleInputBuffer_push(
    mut bib: *mut BattleInputBuffer,
    mut input: BATTLE_INPUT_STATE,
) -> bool {
    let mut next: size_t = 0;
    if (*bib).size == (*bib).maxSize {
        log_add(
            log_Level::log_Error,
            b"NETPLAY:     battleInputBuffer full.\n\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        *__errno_location() = 42 as ::core::ffi::c_int;
        return false;
    }
    next = (*bib).first.wrapping_add((*bib).size).wrapping_rem((*bib).maxSize);
    *(*bib).buf.offset(next as isize) = input;
    (*bib).size = (*bib).size.wrapping_add(1);
    return true;
}
#[no_mangle]
pub unsafe extern "C" fn BattleInputBuffer_pop(
    mut bib: *mut BattleInputBuffer,
    mut input: *mut BATTLE_INPUT_STATE,
) -> bool {
    if (*bib).size == 0 as size_t {
        *__errno_location() = 6 as ::core::ffi::c_int;
        return false;
    }
    *input = *(*bib).buf.offset((*bib).first as isize);
    (*bib).first = (*bib).first.wrapping_add(1 as size_t).wrapping_rem((*bib).maxSize);
    (*bib).size = (*bib).size.wrapping_sub(1);
    return true;
}
#[no_mangle]
pub unsafe extern "C" fn getBattleInputBuffer(
    mut player: size_t,
) -> *mut BattleInputBuffer {
    return (&raw mut battleInputBuffers as *mut BattleInputBuffer)
        .offset(player as isize);
}
