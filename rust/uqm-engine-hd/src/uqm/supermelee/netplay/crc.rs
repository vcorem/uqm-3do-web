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

// Rust translation of src/uqm/supermelee/netplay/crc.c.
#[derive(Copy, Clone)]
#[repr(C)]
pub struct crc_State {
    pub crc: uint32,
}
pub type uint32 = ::core::ffi::c_uint;
pub type uint8 = ::core::ffi::c_uchar;
pub type uint16 = ::core::ffi::c_ushort;
pub type size_t = usize;
#[no_mangle]
pub static mut crcTable: [uint32; 256] = [
    0 as uint32,
    0x77073096 as ::core::ffi::c_int as uint32,
    0xee0e612c as uint32,
    0x990951ba as uint32,
    0x76dc419 as ::core::ffi::c_int as uint32,
    0x706af48f as ::core::ffi::c_int as uint32,
    0xe963a535 as uint32,
    0x9e6495a3 as uint32,
    0xedb8832 as ::core::ffi::c_int as uint32,
    0x79dcb8a4 as ::core::ffi::c_int as uint32,
    0xe0d5e91e as uint32,
    0x97d2d988 as uint32,
    0x9b64c2b as ::core::ffi::c_int as uint32,
    0x7eb17cbd as ::core::ffi::c_int as uint32,
    0xe7b82d07 as uint32,
    0x90bf1d91 as uint32,
    0x1db71064 as ::core::ffi::c_int as uint32,
    0x6ab020f2 as ::core::ffi::c_int as uint32,
    0xf3b97148 as uint32,
    0x84be41de as uint32,
    0x1adad47d as ::core::ffi::c_int as uint32,
    0x6ddde4eb as ::core::ffi::c_int as uint32,
    0xf4d4b551 as uint32,
    0x83d385c7 as uint32,
    0x136c9856 as ::core::ffi::c_int as uint32,
    0x646ba8c0 as ::core::ffi::c_int as uint32,
    0xfd62f97a as uint32,
    0x8a65c9ec as uint32,
    0x14015c4f as ::core::ffi::c_int as uint32,
    0x63066cd9 as ::core::ffi::c_int as uint32,
    0xfa0f3d63 as uint32,
    0x8d080df5 as uint32,
    0x3b6e20c8 as ::core::ffi::c_int as uint32,
    0x4c69105e as ::core::ffi::c_int as uint32,
    0xd56041e4 as uint32,
    0xa2677172 as uint32,
    0x3c03e4d1 as ::core::ffi::c_int as uint32,
    0x4b04d447 as ::core::ffi::c_int as uint32,
    0xd20d85fd as uint32,
    0xa50ab56b as uint32,
    0x35b5a8fa as ::core::ffi::c_int as uint32,
    0x42b2986c as ::core::ffi::c_int as uint32,
    0xdbbbc9d6 as uint32,
    0xacbcf940 as uint32,
    0x32d86ce3 as ::core::ffi::c_int as uint32,
    0x45df5c75 as ::core::ffi::c_int as uint32,
    0xdcd60dcf as uint32,
    0xabd13d59 as uint32,
    0x26d930ac as ::core::ffi::c_int as uint32,
    0x51de003a as ::core::ffi::c_int as uint32,
    0xc8d75180 as uint32,
    0xbfd06116 as uint32,
    0x21b4f4b5 as ::core::ffi::c_int as uint32,
    0x56b3c423 as ::core::ffi::c_int as uint32,
    0xcfba9599 as uint32,
    0xb8bda50f as uint32,
    0x2802b89e as ::core::ffi::c_int as uint32,
    0x5f058808 as ::core::ffi::c_int as uint32,
    0xc60cd9b2 as uint32,
    0xb10be924 as uint32,
    0x2f6f7c87 as ::core::ffi::c_int as uint32,
    0x58684c11 as ::core::ffi::c_int as uint32,
    0xc1611dab as uint32,
    0xb6662d3d as uint32,
    0x76dc4190 as ::core::ffi::c_int as uint32,
    0x1db7106 as ::core::ffi::c_int as uint32,
    0x98d220bc as uint32,
    0xefd5102a as uint32,
    0x71b18589 as ::core::ffi::c_int as uint32,
    0x6b6b51f as ::core::ffi::c_int as uint32,
    0x9fbfe4a5 as uint32,
    0xe8b8d433 as uint32,
    0x7807c9a2 as ::core::ffi::c_int as uint32,
    0xf00f934 as ::core::ffi::c_int as uint32,
    0x9609a88e as uint32,
    0xe10e9818 as uint32,
    0x7f6a0dbb as ::core::ffi::c_int as uint32,
    0x86d3d2d as ::core::ffi::c_int as uint32,
    0x91646c97 as uint32,
    0xe6635c01 as uint32,
    0x6b6b51f4 as ::core::ffi::c_int as uint32,
    0x1c6c6162 as ::core::ffi::c_int as uint32,
    0x856530d8 as uint32,
    0xf262004e as uint32,
    0x6c0695ed as ::core::ffi::c_int as uint32,
    0x1b01a57b as ::core::ffi::c_int as uint32,
    0x8208f4c1 as uint32,
    0xf50fc457 as uint32,
    0x65b0d9c6 as ::core::ffi::c_int as uint32,
    0x12b7e950 as ::core::ffi::c_int as uint32,
    0x8bbeb8ea as uint32,
    0xfcb9887c as uint32,
    0x62dd1ddf as ::core::ffi::c_int as uint32,
    0x15da2d49 as ::core::ffi::c_int as uint32,
    0x8cd37cf3 as uint32,
    0xfbd44c65 as uint32,
    0x4db26158 as ::core::ffi::c_int as uint32,
    0x3ab551ce as ::core::ffi::c_int as uint32,
    0xa3bc0074 as uint32,
    0xd4bb30e2 as uint32,
    0x4adfa541 as ::core::ffi::c_int as uint32,
    0x3dd895d7 as ::core::ffi::c_int as uint32,
    0xa4d1c46d as uint32,
    0xd3d6f4fb as uint32,
    0x4369e96a as ::core::ffi::c_int as uint32,
    0x346ed9fc as ::core::ffi::c_int as uint32,
    0xad678846 as uint32,
    0xda60b8d0 as uint32,
    0x44042d73 as ::core::ffi::c_int as uint32,
    0x33031de5 as ::core::ffi::c_int as uint32,
    0xaa0a4c5f as uint32,
    0xdd0d7cc9 as uint32,
    0x5005713c as ::core::ffi::c_int as uint32,
    0x270241aa as ::core::ffi::c_int as uint32,
    0xbe0b1010 as uint32,
    0xc90c2086 as uint32,
    0x5768b525 as ::core::ffi::c_int as uint32,
    0x206f85b3 as ::core::ffi::c_int as uint32,
    0xb966d409 as uint32,
    0xce61e49f as uint32,
    0x5edef90e as ::core::ffi::c_int as uint32,
    0x29d9c998 as ::core::ffi::c_int as uint32,
    0xb0d09822 as uint32,
    0xc7d7a8b4 as uint32,
    0x59b33d17 as ::core::ffi::c_int as uint32,
    0x2eb40d81 as ::core::ffi::c_int as uint32,
    0xb7bd5c3b as uint32,
    0xc0ba6cad as uint32,
    0xedb88320 as uint32,
    0x9abfb3b6 as uint32,
    0x3b6e20c as ::core::ffi::c_int as uint32,
    0x74b1d29a as ::core::ffi::c_int as uint32,
    0xead54739 as uint32,
    0x9dd277af as uint32,
    0x4db2615 as ::core::ffi::c_int as uint32,
    0x73dc1683 as ::core::ffi::c_int as uint32,
    0xe3630b12 as uint32,
    0x94643b84 as uint32,
    0xd6d6a3e as ::core::ffi::c_int as uint32,
    0x7a6a5aa8 as ::core::ffi::c_int as uint32,
    0xe40ecf0b as uint32,
    0x9309ff9d as uint32,
    0xa00ae27 as ::core::ffi::c_int as uint32,
    0x7d079eb1 as ::core::ffi::c_int as uint32,
    0xf00f9344 as uint32,
    0x8708a3d2 as uint32,
    0x1e01f268 as ::core::ffi::c_int as uint32,
    0x6906c2fe as ::core::ffi::c_int as uint32,
    0xf762575d as uint32,
    0x806567cb as uint32,
    0x196c3671 as ::core::ffi::c_int as uint32,
    0x6e6b06e7 as ::core::ffi::c_int as uint32,
    0xfed41b76 as uint32,
    0x89d32be0 as uint32,
    0x10da7a5a as ::core::ffi::c_int as uint32,
    0x67dd4acc as ::core::ffi::c_int as uint32,
    0xf9b9df6f as uint32,
    0x8ebeeff9 as uint32,
    0x17b7be43 as ::core::ffi::c_int as uint32,
    0x60b08ed5 as ::core::ffi::c_int as uint32,
    0xd6d6a3e8 as uint32,
    0xa1d1937e as uint32,
    0x38d8c2c4 as ::core::ffi::c_int as uint32,
    0x4fdff252 as ::core::ffi::c_int as uint32,
    0xd1bb67f1 as uint32,
    0xa6bc5767 as uint32,
    0x3fb506dd as ::core::ffi::c_int as uint32,
    0x48b2364b as ::core::ffi::c_int as uint32,
    0xd80d2bda as uint32,
    0xaf0a1b4c as uint32,
    0x36034af6 as ::core::ffi::c_int as uint32,
    0x41047a60 as ::core::ffi::c_int as uint32,
    0xdf60efc3 as uint32,
    0xa867df55 as uint32,
    0x316e8eef as ::core::ffi::c_int as uint32,
    0x4669be79 as ::core::ffi::c_int as uint32,
    0xcb61b38c as uint32,
    0xbc66831a as uint32,
    0x256fd2a0 as ::core::ffi::c_int as uint32,
    0x5268e236 as ::core::ffi::c_int as uint32,
    0xcc0c7795 as uint32,
    0xbb0b4703 as uint32,
    0x220216b9 as ::core::ffi::c_int as uint32,
    0x5505262f as ::core::ffi::c_int as uint32,
    0xc5ba3bbe as uint32,
    0xb2bd0b28 as uint32,
    0x2bb45a92 as ::core::ffi::c_int as uint32,
    0x5cb36a04 as ::core::ffi::c_int as uint32,
    0xc2d7ffa7 as uint32,
    0xb5d0cf31 as uint32,
    0x2cd99e8b as ::core::ffi::c_int as uint32,
    0x5bdeae1d as ::core::ffi::c_int as uint32,
    0x9b64c2b0 as uint32,
    0xec63f226 as uint32,
    0x756aa39c as ::core::ffi::c_int as uint32,
    0x26d930a as ::core::ffi::c_int as uint32,
    0x9c0906a9 as uint32,
    0xeb0e363f as uint32,
    0x72076785 as ::core::ffi::c_int as uint32,
    0x5005713 as ::core::ffi::c_int as uint32,
    0x95bf4a82 as uint32,
    0xe2b87a14 as uint32,
    0x7bb12bae as ::core::ffi::c_int as uint32,
    0xcb61b38 as ::core::ffi::c_int as uint32,
    0x92d28e9b as uint32,
    0xe5d5be0d as uint32,
    0x7cdcefb7 as ::core::ffi::c_int as uint32,
    0xbdbdf21 as ::core::ffi::c_int as uint32,
    0x86d3d2d4 as uint32,
    0xf1d4e242 as uint32,
    0x68ddb3f8 as ::core::ffi::c_int as uint32,
    0x1fda836e as ::core::ffi::c_int as uint32,
    0x81be16cd as uint32,
    0xf6b9265b as uint32,
    0x6fb077e1 as ::core::ffi::c_int as uint32,
    0x18b74777 as ::core::ffi::c_int as uint32,
    0x88085ae6 as uint32,
    0xff0f6a70 as uint32,
    0x66063bca as ::core::ffi::c_int as uint32,
    0x11010b5c as ::core::ffi::c_int as uint32,
    0x8f659eff as uint32,
    0xf862ae69 as uint32,
    0x616bffd3 as ::core::ffi::c_int as uint32,
    0x166ccf45 as ::core::ffi::c_int as uint32,
    0xa00ae278 as uint32,
    0xd70dd2ee as uint32,
    0x4e048354 as ::core::ffi::c_int as uint32,
    0x3903b3c2 as ::core::ffi::c_int as uint32,
    0xa7672661 as uint32,
    0xd06016f7 as uint32,
    0x4969474d as ::core::ffi::c_int as uint32,
    0x3e6e77db as ::core::ffi::c_int as uint32,
    0xaed16a4a as uint32,
    0xd9d65adc as uint32,
    0x40df0b66 as ::core::ffi::c_int as uint32,
    0x37d83bf0 as ::core::ffi::c_int as uint32,
    0xa9bcae53 as uint32,
    0xdebb9ec5 as uint32,
    0x47b2cf7f as ::core::ffi::c_int as uint32,
    0x30b5ffe9 as ::core::ffi::c_int as uint32,
    0xbdbdf21c as uint32,
    0xcabac28a as uint32,
    0x53b39330 as ::core::ffi::c_int as uint32,
    0x24b4a3a6 as ::core::ffi::c_int as uint32,
    0xbad03605 as uint32,
    0xcdd70693 as uint32,
    0x54de5729 as ::core::ffi::c_int as uint32,
    0x23d967bf as ::core::ffi::c_int as uint32,
    0xb3667a2e as uint32,
    0xc4614ab8 as uint32,
    0x5d681b02 as ::core::ffi::c_int as uint32,
    0x2a6f2b94 as ::core::ffi::c_int as uint32,
    0xb40bbe37 as uint32,
    0xc30c8ea1 as uint32,
    0x5a05df1b as ::core::ffi::c_int as uint32,
    0x2d02ef8d as ::core::ffi::c_int as uint32,
];
#[no_mangle]
pub unsafe extern "C" fn crc_init(mut state: *mut crc_State) {
    (*state).crc = 0xffffffff as ::core::ffi::c_uint as uint32;
}
#[no_mangle]
pub unsafe extern "C" fn crc_processBytes(
    mut state: *mut crc_State,
    mut buf: *mut uint8,
    mut bufLen: size_t,
) {
    let mut end: *mut uint8 = buf.offset(bufLen as isize);
    let mut newCrc: uint32 = (*state).crc;
    while buf < end {
        newCrc = newCrc >> 8 as ::core::ffi::c_int
            ^ crcTable[((newCrc ^ *buf as uint32) & 0xff as uint32) as usize];
    }
    (*state).crc = newCrc;
}
#[no_mangle]
pub unsafe extern "C" fn crc_processUint8(mut state: *mut crc_State, mut val: uint8) {
    let mut newCrc: uint32 = (*state).crc;
    newCrc = newCrc >> 8 as ::core::ffi::c_int
        ^ crcTable[((newCrc ^ val as uint32) & 0xff as uint32) as usize];
    (*state).crc = newCrc;
}
#[no_mangle]
pub unsafe extern "C" fn crc_processUint16(mut state: *mut crc_State, mut val: uint16) {
    let mut newCrc: uint32 = (*state).crc;
    newCrc = newCrc >> 8 as ::core::ffi::c_int
        ^ crcTable[((newCrc
            ^ (val as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as uint32)
            & 0xff as uint32) as usize];
    newCrc = newCrc >> 8 as ::core::ffi::c_int
        ^ crcTable[((newCrc
            ^ (val as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as uint32)
            & 0xff as uint32) as usize];
    (*state).crc = newCrc;
}
#[no_mangle]
pub unsafe extern "C" fn crc_processUint32(mut state: *mut crc_State, mut val: uint32) {
    let mut newCrc: uint32 = (*state).crc;
    newCrc = newCrc >> 8 as ::core::ffi::c_int
        ^ crcTable[((newCrc ^ val & 0xff as uint32) & 0xff as uint32) as usize];
    newCrc = newCrc >> 8 as ::core::ffi::c_int
        ^ crcTable[((newCrc ^ val >> 8 as ::core::ffi::c_int & 0xff as uint32)
            & 0xff as uint32) as usize];
    newCrc = newCrc >> 8 as ::core::ffi::c_int
        ^ crcTable[((newCrc ^ val >> 16 as ::core::ffi::c_int & 0xff as uint32)
            & 0xff as uint32) as usize];
    newCrc = newCrc >> 8 as ::core::ffi::c_int
        ^ crcTable[((newCrc ^ val >> 24 as ::core::ffi::c_int) & 0xff as uint32)
            as usize];
    (*state).crc = newCrc;
}
#[no_mangle]
pub unsafe extern "C" fn crc_finish(mut state: *const crc_State) -> uint32 {
    return !(*state).crc;
}
