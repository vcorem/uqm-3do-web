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

// Rust translation of src/uqm/supermelee/netplay/packet.c.
extern "C" {
    pub type NetConnection;
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
    fn __assert_fail(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    );
    fn malloc(_: size_t) -> *mut ::core::ffi::c_void;
    fn free(_: *mut ::core::ffi::c_void);
    fn PacketHandler_Init(
        conn: *mut NetConnection,
        packet: *const Packet_Init,
    ) -> ::core::ffi::c_int;
    fn PacketHandler_Ping(
        conn: *mut NetConnection,
        packet: *const Packet_Ping,
    ) -> ::core::ffi::c_int;
    fn PacketHandler_Ack(
        conn: *mut NetConnection,
        packet: *const Packet_Ack,
    ) -> ::core::ffi::c_int;
    fn PacketHandler_Ready(
        conn: *mut NetConnection,
        packet: *const Packet_Ready,
    ) -> ::core::ffi::c_int;
    fn PacketHandler_Fleet(
        conn: *mut NetConnection,
        packet: *const Packet_Fleet,
    ) -> ::core::ffi::c_int;
    fn PacketHandler_TeamName(
        conn: *mut NetConnection,
        packet: *const Packet_TeamName,
    ) -> ::core::ffi::c_int;
    fn PacketHandler_Handshake0(
        conn: *mut NetConnection,
        packet: *const Packet_Handshake0,
    ) -> ::core::ffi::c_int;
    fn PacketHandler_Handshake1(
        conn: *mut NetConnection,
        packet: *const Packet_Handshake1,
    ) -> ::core::ffi::c_int;
    fn PacketHandler_HandshakeCancel(
        conn: *mut NetConnection,
        packet: *const Packet_HandshakeCancel,
    ) -> ::core::ffi::c_int;
    fn PacketHandler_HandshakeCancelAck(
        conn: *mut NetConnection,
        packet: *const Packet_HandshakeCancelAck,
    ) -> ::core::ffi::c_int;
    fn PacketHandler_SeedRandom(
        conn: *mut NetConnection,
        packet: *const Packet_SeedRandom,
    ) -> ::core::ffi::c_int;
    fn PacketHandler_InputDelay(
        conn: *mut NetConnection,
        packet: *const Packet_InputDelay,
    ) -> ::core::ffi::c_int;
    fn PacketHandler_SelectShip(
        conn: *mut NetConnection,
        packet: *const Packet_SelectShip,
    ) -> ::core::ffi::c_int;
    fn PacketHandler_BattleInput(
        conn: *mut NetConnection,
        packet: *const Packet_BattleInput,
    ) -> ::core::ffi::c_int;
    fn PacketHandler_FrameCount(
        conn: *mut NetConnection,
        packet: *const Packet_FrameCount,
    ) -> ::core::ffi::c_int;
    fn PacketHandler_Checksum(
        conn: *mut NetConnection,
        packet: *const Packet_Checksum,
    ) -> ::core::ffi::c_int;
    fn PacketHandler_Abort(
        conn: *mut NetConnection,
        packet: *const Packet_Abort,
    ) -> ::core::ffi::c_int;
    fn PacketHandler_Reset(
        conn: *mut NetConnection,
        packet: *const Packet_Reset,
    ) -> ::core::ffi::c_int;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Packet {
    pub header: PacketHeader,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PacketHeader {
    pub len: uint16,
    pub r#type: uint16,
}
pub type uint16 = ::core::ffi::c_ushort;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct PacketType(pub ::core::ffi::c_uint);
impl PacketType {
    pub const PACKET_INIT: Self = Self(0);
    pub const PACKET_PING: Self = Self(1);
    pub const PACKET_ACK: Self = Self(2);
    pub const PACKET_READY: Self = Self(3);
    pub const PACKET_FLEET: Self = Self(4);
    pub const PACKET_TEAMNAME: Self = Self(5);
    pub const PACKET_HANDSHAKE0: Self = Self(6);
    pub const PACKET_HANDSHAKE1: Self = Self(7);
    pub const PACKET_HANDSHAKECANCEL: Self = Self(8);
    pub const PACKET_HANDSHAKECANCELACK: Self = Self(9);
    pub const PACKET_SEEDRANDOM: Self = Self(10);
    pub const PACKET_INPUTDELAY: Self = Self(11);
    pub const PACKET_SELECTSHIP: Self = Self(12);
    pub const PACKET_BATTLEINPUT: Self = Self(13);
    pub const PACKET_FRAMECOUNT: Self = Self(14);
    pub const PACKET_CHECKSUM: Self = Self(15);
    pub const PACKET_ABORT: Self = Self(16);
    pub const PACKET_RESET: Self = Self(17);
    pub const PACKET_NUM: Self = Self(18);
}
pub type size_t = usize;
pub type uint8 = ::core::ffi::c_uchar;
pub type uint32 = ::core::ffi::c_uint;
pub type PacketHandler = Option<
    unsafe extern "C" fn(
        *mut NetConnection,
        *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PacketTypeData {
    pub len: size_t,
    pub handler: PacketHandler,
    pub name: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Packet_Reset {
    pub header: PacketHeader,
    pub reason: uint16,
    pub padding0: uint16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Packet_Abort {
    pub header: PacketHeader,
    pub reason: uint16,
    pub padding0: uint16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Packet_Checksum {
    pub header: PacketHeader,
    pub frameNr: uint32,
    pub checksum: uint32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Packet_FrameCount {
    pub header: PacketHeader,
    pub frameCount: uint32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Packet_BattleInput {
    pub header: PacketHeader,
    pub state: uint8,
    pub padding0: uint8,
    pub padding1: uint16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Packet_SelectShip {
    pub header: PacketHeader,
    pub ship: uint16,
    pub padding: uint16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Packet_InputDelay {
    pub header: PacketHeader,
    pub delay: uint32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Packet_SeedRandom {
    pub header: PacketHeader,
    pub seed: uint32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Packet_HandshakeCancelAck {
    pub header: PacketHeader,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Packet_HandshakeCancel {
    pub header: PacketHeader,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Packet_Handshake1 {
    pub header: PacketHeader,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Packet_Handshake0 {
    pub header: PacketHeader,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Packet_TeamName {
    pub header: PacketHeader,
    pub side: uint8,
    pub padding: uint8,
    pub name: [uint8; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Packet_Fleet {
    pub header: PacketHeader,
    pub side: uint8,
    pub padding: uint8,
    pub numShips: uint16,
    pub ships: [FleetEntry; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FleetEntry {
    pub index: uint8,
    pub ship: uint8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Packet_Ready {
    pub header: PacketHeader,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Packet_Ack {
    pub header: PacketHeader,
    pub id: uint32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Packet_Ping {
    pub header: PacketHeader,
    pub id: uint32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Packet_Init {
    pub header: PacketHeader,
    pub protoVersion: C2Rust_Unnamed_0,
    pub padding0: uint16,
    pub uqmVersion: C2Rust_Unnamed,
    pub padding1: uint8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed {
    pub major: uint8,
    pub minor: uint8,
    pub patch: uint8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_0 {
    pub major: uint8,
    pub minor: uint8,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct NetplaySide(pub ::core::ffi::c_uint);
impl NetplaySide {
    pub const NetplaySide_local: Self = Self(0);
    pub const NetplaySide_remote: Self = Self(1);
}
#[inline]
unsafe extern "C" fn swapBytes16(mut x: uint16) -> uint16 {
    return ((x as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
        | x as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as uint16;
}
#[inline]
unsafe extern "C" fn swapBytes32(mut x: uint32) -> uint32 {
    return x << 24 as ::core::ffi::c_int
        | (x & 0xff00 as uint32) << 8 as ::core::ffi::c_int
        | (x & 0xff0000 as ::core::ffi::c_int as uint32) >> 8 as ::core::ffi::c_int
        | x >> 24 as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn hton16(mut x: uint16) -> uint16 {
    return swapBytes16(x);
}
#[inline]
unsafe extern "C" fn hton32(mut x: uint32) -> uint32 {
    return swapBytes32(x);
}
#[no_mangle]
pub static mut packetTypeData: [PacketTypeData; 18] = unsafe {
    [
        PacketTypeData {
            len: ::core::mem::size_of::<Packet_Init>(),
            handler: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut NetConnection,
                        *const Packet_Init,
                    ) -> ::core::ffi::c_int,
                >,
                PacketHandler,
            >(
                Some(
                    PacketHandler_Init
                        as unsafe extern "C" fn(
                            *mut NetConnection,
                            *const Packet_Init,
                        ) -> ::core::ffi::c_int,
                ),
            ),
            name: b"Init\0".as_ptr() as *const ::core::ffi::c_char,
        },
        PacketTypeData {
            len: ::core::mem::size_of::<Packet_Ping>(),
            handler: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut NetConnection,
                        *const Packet_Ping,
                    ) -> ::core::ffi::c_int,
                >,
                PacketHandler,
            >(
                Some(
                    PacketHandler_Ping
                        as unsafe extern "C" fn(
                            *mut NetConnection,
                            *const Packet_Ping,
                        ) -> ::core::ffi::c_int,
                ),
            ),
            name: b"Ping\0".as_ptr() as *const ::core::ffi::c_char,
        },
        PacketTypeData {
            len: ::core::mem::size_of::<Packet_Ack>(),
            handler: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut NetConnection,
                        *const Packet_Ack,
                    ) -> ::core::ffi::c_int,
                >,
                PacketHandler,
            >(
                Some(
                    PacketHandler_Ack
                        as unsafe extern "C" fn(
                            *mut NetConnection,
                            *const Packet_Ack,
                        ) -> ::core::ffi::c_int,
                ),
            ),
            name: b"Ack\0".as_ptr() as *const ::core::ffi::c_char,
        },
        PacketTypeData {
            len: ::core::mem::size_of::<Packet_Ready>(),
            handler: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut NetConnection,
                        *const Packet_Ready,
                    ) -> ::core::ffi::c_int,
                >,
                PacketHandler,
            >(
                Some(
                    PacketHandler_Ready
                        as unsafe extern "C" fn(
                            *mut NetConnection,
                            *const Packet_Ready,
                        ) -> ::core::ffi::c_int,
                ),
            ),
            name: b"Ready\0".as_ptr() as *const ::core::ffi::c_char,
        },
        PacketTypeData {
            len: ::core::mem::size_of::<Packet_Fleet>(),
            handler: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut NetConnection,
                        *const Packet_Fleet,
                    ) -> ::core::ffi::c_int,
                >,
                PacketHandler,
            >(
                Some(
                    PacketHandler_Fleet
                        as unsafe extern "C" fn(
                            *mut NetConnection,
                            *const Packet_Fleet,
                        ) -> ::core::ffi::c_int,
                ),
            ),
            name: b"Fleet\0".as_ptr() as *const ::core::ffi::c_char,
        },
        PacketTypeData {
            len: ::core::mem::size_of::<Packet_TeamName>(),
            handler: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut NetConnection,
                        *const Packet_TeamName,
                    ) -> ::core::ffi::c_int,
                >,
                PacketHandler,
            >(
                Some(
                    PacketHandler_TeamName
                        as unsafe extern "C" fn(
                            *mut NetConnection,
                            *const Packet_TeamName,
                        ) -> ::core::ffi::c_int,
                ),
            ),
            name: b"TeamName\0".as_ptr() as *const ::core::ffi::c_char,
        },
        PacketTypeData {
            len: ::core::mem::size_of::<Packet_Handshake0>(),
            handler: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut NetConnection,
                        *const Packet_Handshake0,
                    ) -> ::core::ffi::c_int,
                >,
                PacketHandler,
            >(
                Some(
                    PacketHandler_Handshake0
                        as unsafe extern "C" fn(
                            *mut NetConnection,
                            *const Packet_Handshake0,
                        ) -> ::core::ffi::c_int,
                ),
            ),
            name: b"Handshake0\0".as_ptr() as *const ::core::ffi::c_char,
        },
        PacketTypeData {
            len: ::core::mem::size_of::<Packet_Handshake1>(),
            handler: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut NetConnection,
                        *const Packet_Handshake1,
                    ) -> ::core::ffi::c_int,
                >,
                PacketHandler,
            >(
                Some(
                    PacketHandler_Handshake1
                        as unsafe extern "C" fn(
                            *mut NetConnection,
                            *const Packet_Handshake1,
                        ) -> ::core::ffi::c_int,
                ),
            ),
            name: b"Handshake1\0".as_ptr() as *const ::core::ffi::c_char,
        },
        PacketTypeData {
            len: ::core::mem::size_of::<Packet_HandshakeCancel>(),
            handler: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut NetConnection,
                        *const Packet_HandshakeCancel,
                    ) -> ::core::ffi::c_int,
                >,
                PacketHandler,
            >(
                Some(
                    PacketHandler_HandshakeCancel
                        as unsafe extern "C" fn(
                            *mut NetConnection,
                            *const Packet_HandshakeCancel,
                        ) -> ::core::ffi::c_int,
                ),
            ),
            name: b"HandshakeCancel\0".as_ptr() as *const ::core::ffi::c_char,
        },
        PacketTypeData {
            len: ::core::mem::size_of::<Packet_HandshakeCancelAck>(),
            handler: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut NetConnection,
                        *const Packet_HandshakeCancelAck,
                    ) -> ::core::ffi::c_int,
                >,
                PacketHandler,
            >(
                Some(
                    PacketHandler_HandshakeCancelAck
                        as unsafe extern "C" fn(
                            *mut NetConnection,
                            *const Packet_HandshakeCancelAck,
                        ) -> ::core::ffi::c_int,
                ),
            ),
            name: b"HandshakeCancelAck\0".as_ptr() as *const ::core::ffi::c_char,
        },
        PacketTypeData {
            len: ::core::mem::size_of::<Packet_SeedRandom>(),
            handler: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut NetConnection,
                        *const Packet_SeedRandom,
                    ) -> ::core::ffi::c_int,
                >,
                PacketHandler,
            >(
                Some(
                    PacketHandler_SeedRandom
                        as unsafe extern "C" fn(
                            *mut NetConnection,
                            *const Packet_SeedRandom,
                        ) -> ::core::ffi::c_int,
                ),
            ),
            name: b"SeedRandom\0".as_ptr() as *const ::core::ffi::c_char,
        },
        PacketTypeData {
            len: ::core::mem::size_of::<Packet_InputDelay>(),
            handler: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut NetConnection,
                        *const Packet_InputDelay,
                    ) -> ::core::ffi::c_int,
                >,
                PacketHandler,
            >(
                Some(
                    PacketHandler_InputDelay
                        as unsafe extern "C" fn(
                            *mut NetConnection,
                            *const Packet_InputDelay,
                        ) -> ::core::ffi::c_int,
                ),
            ),
            name: b"InputDelay\0".as_ptr() as *const ::core::ffi::c_char,
        },
        PacketTypeData {
            len: ::core::mem::size_of::<Packet_SelectShip>(),
            handler: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut NetConnection,
                        *const Packet_SelectShip,
                    ) -> ::core::ffi::c_int,
                >,
                PacketHandler,
            >(
                Some(
                    PacketHandler_SelectShip
                        as unsafe extern "C" fn(
                            *mut NetConnection,
                            *const Packet_SelectShip,
                        ) -> ::core::ffi::c_int,
                ),
            ),
            name: b"SelectShip\0".as_ptr() as *const ::core::ffi::c_char,
        },
        PacketTypeData {
            len: ::core::mem::size_of::<Packet_BattleInput>(),
            handler: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut NetConnection,
                        *const Packet_BattleInput,
                    ) -> ::core::ffi::c_int,
                >,
                PacketHandler,
            >(
                Some(
                    PacketHandler_BattleInput
                        as unsafe extern "C" fn(
                            *mut NetConnection,
                            *const Packet_BattleInput,
                        ) -> ::core::ffi::c_int,
                ),
            ),
            name: b"BattleInput\0".as_ptr() as *const ::core::ffi::c_char,
        },
        PacketTypeData {
            len: ::core::mem::size_of::<Packet_FrameCount>(),
            handler: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut NetConnection,
                        *const Packet_FrameCount,
                    ) -> ::core::ffi::c_int,
                >,
                PacketHandler,
            >(
                Some(
                    PacketHandler_FrameCount
                        as unsafe extern "C" fn(
                            *mut NetConnection,
                            *const Packet_FrameCount,
                        ) -> ::core::ffi::c_int,
                ),
            ),
            name: b"FrameCount\0".as_ptr() as *const ::core::ffi::c_char,
        },
        PacketTypeData {
            len: ::core::mem::size_of::<Packet_Checksum>(),
            handler: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut NetConnection,
                        *const Packet_Checksum,
                    ) -> ::core::ffi::c_int,
                >,
                PacketHandler,
            >(
                Some(
                    PacketHandler_Checksum
                        as unsafe extern "C" fn(
                            *mut NetConnection,
                            *const Packet_Checksum,
                        ) -> ::core::ffi::c_int,
                ),
            ),
            name: b"Checksum\0".as_ptr() as *const ::core::ffi::c_char,
        },
        PacketTypeData {
            len: ::core::mem::size_of::<Packet_Abort>(),
            handler: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut NetConnection,
                        *const Packet_Abort,
                    ) -> ::core::ffi::c_int,
                >,
                PacketHandler,
            >(
                Some(
                    PacketHandler_Abort
                        as unsafe extern "C" fn(
                            *mut NetConnection,
                            *const Packet_Abort,
                        ) -> ::core::ffi::c_int,
                ),
            ),
            name: b"Abort\0".as_ptr() as *const ::core::ffi::c_char,
        },
        PacketTypeData {
            len: ::core::mem::size_of::<Packet_Reset>(),
            handler: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut NetConnection,
                        *const Packet_Reset,
                    ) -> ::core::ffi::c_int,
                >,
                PacketHandler,
            >(
                Some(
                    PacketHandler_Reset
                        as unsafe extern "C" fn(
                            *mut NetConnection,
                            *const Packet_Reset,
                        ) -> ::core::ffi::c_int,
                ),
            ),
            name: b"Reset\0".as_ptr() as *const ::core::ffi::c_char,
        },
    ]
};
#[inline]
unsafe extern "C" fn Packet_alloc(mut size: size_t) -> *mut ::core::ffi::c_void {
    return malloc(size);
}
unsafe extern "C" fn Packet_create(
    mut r#type: PacketType,
    mut extraSize: size_t,
) -> *mut Packet {
    let mut result: *mut Packet = ::core::ptr::null_mut::<Packet>();
    let mut len: size_t = 0;
    (extraSize.wrapping_rem(4 as size_t) == 0 as size_t
        || {
            __assert_fail(
                b"extraSize % 4 == 0\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/uqm/supermelee/netplay/packet.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                70 as ::core::ffi::c_int,
                b"Packet_create\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    len = packetTypeData[r#type.0 as usize].len.wrapping_add(extraSize);
    result = Packet_alloc(len) as *mut Packet;
    (*result).header.len = hton16(len as uint16);
    (*result).header.r#type = hton16(r#type.0 as uint16);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn Packet_delete(mut packet: *mut Packet) {
    free(packet as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn Packet_Init_create() -> *mut Packet_Init {
    let mut packet: *mut Packet_Init = Packet_create(
        PacketType::PACKET_INIT,
        0 as size_t,
    ) as *mut Packet_Init;
    (*packet).protoVersion.major = 0 as uint8;
    (*packet).protoVersion.minor = 4 as uint8;
    (*packet).padding0 = 0 as uint16;
    (*packet).uqmVersion.major = 0 as uint8;
    (*packet).uqmVersion.minor = 8 as uint8;
    (*packet).uqmVersion.patch = 0 as uint8;
    (*packet).padding1 = 0 as uint8;
    return packet;
}
#[no_mangle]
pub unsafe extern "C" fn Packet_Ping_create(mut id: uint32) -> *mut Packet_Ping {
    let mut packet: *mut Packet_Ping = Packet_create(
        PacketType::PACKET_PING,
        0 as size_t,
    ) as *mut Packet_Ping;
    (*packet).id = hton32(id);
    return packet;
}
#[no_mangle]
pub unsafe extern "C" fn Packet_Ack_create(mut id: uint32) -> *mut Packet_Ack {
    let mut packet: *mut Packet_Ack = Packet_create(PacketType::PACKET_ACK, 0 as size_t)
        as *mut Packet_Ack;
    (*packet).id = hton32(id);
    return packet;
}
#[no_mangle]
pub unsafe extern "C" fn Packet_Ready_create() -> *mut Packet_Ready {
    let mut packet: *mut Packet_Ready = Packet_create(
        PacketType::PACKET_READY,
        0 as size_t,
    ) as *mut Packet_Ready;
    return packet;
}
#[no_mangle]
pub unsafe extern "C" fn Packet_Fleet_create(
    mut side: NetplaySide,
    mut numShips: size_t,
) -> *mut Packet_Fleet {
    let mut packet: *mut Packet_Fleet = ::core::ptr::null_mut::<Packet_Fleet>();
    let mut fleetSize: size_t = 0;
    let mut extraSize: size_t = 0;
    fleetSize = numShips.wrapping_mul(::core::mem::size_of::<FleetEntry>());
    extraSize = fleetSize.wrapping_add(3 as size_t)
        & !(0x3 as ::core::ffi::c_int) as size_t;
    packet = Packet_create(PacketType::PACKET_FLEET, extraSize) as *mut Packet_Fleet;
    (*packet).side = side.0 as uint8;
    (*packet).padding = 0 as uint8;
    (*packet).numShips = hton16(numShips as uint16);
    memset(
        (packet as *mut ::core::ffi::c_char)
            .offset(::core::mem::size_of::<Packet_Fleet>() as isize)
            .offset(fleetSize as isize) as *mut ::core::ffi::c_void,
        '\0' as ::core::ffi::c_int,
        extraSize.wrapping_sub(fleetSize),
    );
    return packet;
}
#[no_mangle]
pub unsafe extern "C" fn Packet_TeamName_create(
    mut side: NetplaySide,
    mut name: *const ::core::ffi::c_char,
    mut size: size_t,
) -> *mut Packet_TeamName {
    let mut packet: *mut Packet_TeamName = ::core::ptr::null_mut::<Packet_TeamName>();
    let mut extraSize: size_t = 0;
    extraSize = size.wrapping_add(1 as size_t).wrapping_add(3 as size_t)
        & !(0x3 as ::core::ffi::c_int) as size_t;
    packet = Packet_create(PacketType::PACKET_TEAMNAME, extraSize)
        as *mut Packet_TeamName;
    (*packet).side = side.0 as uint8;
    (*packet).padding = 0 as uint8;
    memcpy(
        &raw mut (*packet).name as *mut uint8 as *mut ::core::ffi::c_void,
        name as *const ::core::ffi::c_void,
        size,
    );
    memset(
        (packet as *mut ::core::ffi::c_char)
            .offset(::core::mem::size_of::<Packet_TeamName>() as isize)
            .offset(size as isize) as *mut ::core::ffi::c_void,
        '\0' as ::core::ffi::c_int,
        extraSize.wrapping_sub(size),
    );
    return packet;
}
#[no_mangle]
pub unsafe extern "C" fn Packet_Handshake0_create() -> *mut Packet_Handshake0 {
    let mut packet: *mut Packet_Handshake0 = Packet_create(
        PacketType::PACKET_HANDSHAKE0,
        0 as size_t,
    ) as *mut Packet_Handshake0;
    return packet;
}
#[no_mangle]
pub unsafe extern "C" fn Packet_Handshake1_create() -> *mut Packet_Handshake1 {
    let mut packet: *mut Packet_Handshake1 = Packet_create(
        PacketType::PACKET_HANDSHAKE1,
        0 as size_t,
    ) as *mut Packet_Handshake1;
    return packet;
}
#[no_mangle]
pub unsafe extern "C" fn Packet_HandshakeCancel_create() -> *mut Packet_HandshakeCancel {
    let mut packet: *mut Packet_HandshakeCancel = Packet_create(
        PacketType::PACKET_HANDSHAKECANCEL,
        0 as size_t,
    ) as *mut Packet_HandshakeCancel;
    return packet;
}
#[no_mangle]
pub unsafe extern "C" fn Packet_HandshakeCancelAck_create() -> *mut Packet_HandshakeCancelAck {
    let mut packet: *mut Packet_HandshakeCancelAck = Packet_create(
        PacketType::PACKET_HANDSHAKECANCELACK,
        0 as size_t,
    ) as *mut Packet_HandshakeCancelAck;
    return packet;
}
#[no_mangle]
pub unsafe extern "C" fn Packet_SeedRandom_create(
    mut seed: uint32,
) -> *mut Packet_SeedRandom {
    let mut packet: *mut Packet_SeedRandom = Packet_create(
        PacketType::PACKET_SEEDRANDOM,
        0 as size_t,
    ) as *mut Packet_SeedRandom;
    (*packet).seed = hton32(seed);
    return packet;
}
#[no_mangle]
pub unsafe extern "C" fn Packet_InputDelay_create(
    mut delay: uint32,
) -> *mut Packet_InputDelay {
    let mut packet: *mut Packet_InputDelay = Packet_create(
        PacketType::PACKET_INPUTDELAY,
        0 as size_t,
    ) as *mut Packet_InputDelay;
    (*packet).delay = hton32(delay);
    return packet;
}
#[no_mangle]
pub unsafe extern "C" fn Packet_SelectShip_create(
    mut ship: uint16,
) -> *mut Packet_SelectShip {
    let mut packet: *mut Packet_SelectShip = Packet_create(
        PacketType::PACKET_SELECTSHIP,
        0 as size_t,
    ) as *mut Packet_SelectShip;
    (*packet).ship = hton16(ship);
    (*packet).padding = 0 as uint16;
    return packet;
}
#[no_mangle]
pub unsafe extern "C" fn Packet_BattleInput_create(
    mut state: uint8,
) -> *mut Packet_BattleInput {
    let mut packet: *mut Packet_BattleInput = Packet_create(
        PacketType::PACKET_BATTLEINPUT,
        0 as size_t,
    ) as *mut Packet_BattleInput;
    (*packet).state = state;
    (*packet).padding0 = 0 as uint8;
    (*packet).padding1 = 0 as uint16;
    return packet;
}
#[no_mangle]
pub unsafe extern "C" fn Packet_FrameCount_create(
    mut frameCount: uint32,
) -> *mut Packet_FrameCount {
    let mut packet: *mut Packet_FrameCount = Packet_create(
        PacketType::PACKET_FRAMECOUNT,
        0 as size_t,
    ) as *mut Packet_FrameCount;
    (*packet).frameCount = hton32(frameCount);
    return packet;
}
#[no_mangle]
pub unsafe extern "C" fn Packet_Checksum_create(
    mut frameNr: uint32,
    mut checksum: uint32,
) -> *mut Packet_Checksum {
    let mut packet: *mut Packet_Checksum = Packet_create(
        PacketType::PACKET_CHECKSUM,
        0 as size_t,
    ) as *mut Packet_Checksum;
    (*packet).frameNr = hton32(frameNr);
    (*packet).checksum = hton32(checksum);
    return packet;
}
#[no_mangle]
pub unsafe extern "C" fn Packet_Abort_create(mut reason: uint16) -> *mut Packet_Abort {
    let mut packet: *mut Packet_Abort = Packet_create(
        PacketType::PACKET_ABORT,
        0 as size_t,
    ) as *mut Packet_Abort;
    (*packet).reason = hton16(reason);
    return packet;
}
#[no_mangle]
pub unsafe extern "C" fn Packet_Reset_create(mut reason: uint16) -> *mut Packet_Reset {
    let mut packet: *mut Packet_Reset = Packet_create(
        PacketType::PACKET_RESET,
        0 as size_t,
    ) as *mut Packet_Reset;
    (*packet).reason = hton16(reason);
    return packet;
}
