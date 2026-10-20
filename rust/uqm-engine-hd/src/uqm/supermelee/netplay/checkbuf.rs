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

// Rust translation of src/uqm/supermelee/netplay/checkbuf.c.
extern "C" {
    fn __assert_fail(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    ) -> !;
    fn malloc(_: size_t) -> *mut ::core::ffi::c_void;
    fn free(_: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ChecksumEntry {
    pub checksum: Checksum,
}
pub type Checksum = uint32;
pub type uint32 = ::core::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ChecksumBuffer {
    pub checksums: *mut ChecksumEntry,
    pub maxSize: size_t,
    pub interval: size_t,
}
pub type DWORD = uint32;
pub type BattleFrameCounter = DWORD;
#[no_mangle]
pub unsafe extern "C" fn ChecksumBuffer_init(
    mut cb: *mut ChecksumBuffer,
    mut delay: size_t,
    mut interval: size_t,
) {
    let mut bufSize: size_t = (2 as size_t)
        .wrapping_mul(delay)
        .wrapping_add(2 as size_t)
        .wrapping_add(interval.wrapping_sub(1 as size_t))
        .wrapping_div(interval);
    (*cb).checksums = malloc(
        bufSize.wrapping_mul(::core::mem::size_of::<ChecksumEntry>()),
    ) as *mut ChecksumEntry;
    (*cb).maxSize = bufSize;
    (*cb).interval = interval;
}
#[no_mangle]
pub unsafe extern "C" fn ChecksumBuffer_uninit(mut cb: *mut ChecksumBuffer) {
    if !(*cb).checksums.is_null() {
        free((*cb).checksums as *mut ::core::ffi::c_void);
        (*cb).checksums = ::core::ptr::null_mut::<ChecksumEntry>();
    }
}
unsafe extern "C" fn ChecksumBuffer_getChecksumEntry(
    mut cb: *mut ChecksumBuffer,
    mut frameNr: BattleFrameCounter,
) -> *mut ChecksumEntry {
    let mut index: size_t = 0;
    let mut entry: *mut ChecksumEntry = ::core::ptr::null_mut::<ChecksumEntry>();
    ((frameNr as size_t).wrapping_rem((*cb).interval) == 0 as size_t
        || {
            __assert_fail(
                b"frameNr % cb->interval == 0\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/uqm/supermelee/netplay/checkbuf.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                100 as ::core::ffi::c_int,
                b"ChecksumBuffer_getChecksumEntry\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    index = (frameNr as size_t).wrapping_div((*cb).interval).wrapping_rem((*cb).maxSize);
    entry = (*cb).checksums.offset(index as isize);
    return entry;
}
#[no_mangle]
pub unsafe extern "C" fn ChecksumBuffer_addChecksum(
    mut cb: *mut ChecksumBuffer,
    mut frameNr: BattleFrameCounter,
    mut checksum: Checksum,
) -> bool {
    let mut entry: *mut ChecksumEntry = ::core::ptr::null_mut::<ChecksumEntry>();
    ((frameNr as size_t).wrapping_rem((*cb).interval) == 0 as size_t
        || {
            __assert_fail(
                b"frameNr % cb->interval == 0\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/uqm/supermelee/netplay/checkbuf.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                114 as ::core::ffi::c_int,
                b"ChecksumBuffer_addChecksum\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    entry = ChecksumBuffer_getChecksumEntry(cb, frameNr);
    (*entry).checksum = checksum;
    return true;
}
#[no_mangle]
pub unsafe extern "C" fn ChecksumBuffer_getChecksum(
    mut cb: *mut ChecksumBuffer,
    mut frameNr: BattleFrameCounter,
    mut result: *mut Checksum,
) -> bool {
    let mut entry: *mut ChecksumEntry = ::core::ptr::null_mut::<ChecksumEntry>();
    entry = ChecksumBuffer_getChecksumEntry(cb, frameNr);
    *result = (*entry).checksum;
    return true;
}
