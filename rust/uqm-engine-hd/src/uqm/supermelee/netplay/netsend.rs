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

// Rust translation of src/uqm/supermelee/netplay/netsend.c.
use ::c2rust_bitfields;
extern "C" {
    pub type NetDescriptor;
    fn __assert_fail(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    ) -> !;
    fn emscripten_asm_const_int_sync_on_main_thread(
        code: *const ::core::ffi::c_char,
        arg_sigs: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn NetConnection_isConnected(conn: *const NetConnection) -> bool;
    fn NetConnection_getStatistics(conn: *mut NetConnection) -> *mut NetStatistics;
}
pub type size_t = usize;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NetConnection {
    pub nd: *mut NetDescriptor,
    pub player: ::core::ffi::c_int,
    pub state: NetState,
    pub stateFlags: NetStateFlags,
    pub readyCallback: NetConnection_ReadyCallback,
    pub readyCallbackArg: *mut ::core::ffi::c_void,
    pub resetCallback: NetConnection_ResetCallback,
    pub resetCallbackArg: *mut ::core::ffi::c_void,
    pub options: *const NetplayPeerOptions,
    pub queue: PacketQueue,
    pub statistics: NetStatistics,
    pub checksumBuffer: ChecksumBuffer,
    pub connectCallback: NetConnection_ConnectCallback,
    pub closeCallback: NetConnection_CloseCallback,
    pub errorCallback: NetConnection_ErrorCallback,
    pub deleteCallback: NetConnection_DeleteCallback,
    pub readBuf: *mut uint8,
    pub readEnd: *mut uint8,
    pub stateData: *mut NetConnectionStateData,
    pub extra: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NetConnectionStateData {
    pub releaseFunction: NetConnectionStateData_ReleaseFunction,
}
pub type NetConnectionStateData_ReleaseFunction = Option<
    unsafe extern "C" fn(*mut NetConnectionStateData) -> (),
>;
pub type uint8 = ::core::ffi::c_uchar;
pub type NetConnection_DeleteCallback = Option<
    unsafe extern "C" fn(*mut NetConnection) -> (),
>;
pub type NetConnection_ErrorCallback = Option<
    unsafe extern "C" fn(*mut NetConnection, *const NetConnectionError) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NetConnectionError {
    pub state: NetState,
    pub err: ::core::ffi::c_int,
    pub extra: C2Rust_Unnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed {
    pub listenError: *const ListenError,
    pub connectError: *const ConnectError,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ConnectError {
    pub state: ConnectStateState,
    pub err: ::core::ffi::c_int,
    pub resolveError: *const ResolveError,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ResolveError {
    pub gaiRes: ::core::ffi::c_int,
    pub err: ::core::ffi::c_int,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ConnectStateState(pub ::core::ffi::c_uint);
impl ConnectStateState {
    pub const Connect_closed: Self = Self(0);
    pub const Connect_resolving: Self = Self(1);
    pub const Connect_connecting: Self = Self(2);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ListenError {
    pub state: ListenStateState,
    pub err: ::core::ffi::c_int,
    pub resolveError: *const ResolveError,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ListenStateState(pub ::core::ffi::c_uint);
impl ListenStateState {
    pub const Listen_closed: Self = Self(0);
    pub const Listen_resolving: Self = Self(1);
    pub const Listen_listening: Self = Self(2);
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
pub type NetConnection_CloseCallback = Option<
    unsafe extern "C" fn(*mut NetConnection) -> (),
>;
pub type NetConnection_ConnectCallback = Option<
    unsafe extern "C" fn(*mut NetConnection) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ChecksumBuffer {
    pub checksums: *mut ChecksumEntry,
    pub maxSize: size_t,
    pub interval: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ChecksumEntry {
    pub checksum: Checksum,
}
pub type Checksum = uint32;
pub type uint32 = ::core::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NetStatistics {
    pub packetsReceived: size_t,
    pub packetTypeReceived: [size_t; 18],
    pub packetsSent: size_t,
    pub packetTypeSent: [size_t; 18],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PacketQueue {
    pub size: size_t,
    pub first: *mut PacketQueueLink,
    pub end: *mut *mut PacketQueueLink,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PacketQueueLink {
    pub next: *mut PacketQueueLink,
    pub packet: *mut Packet,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NetplayPeerOptions {
    pub isServer: bool,
    pub host: *const ::core::ffi::c_char,
    pub port: *const ::core::ffi::c_char,
}
pub type NetConnection_ResetCallback = Option<
    unsafe extern "C" fn(*mut NetConnection, *mut ::core::ffi::c_void) -> (),
>;
pub type NetConnection_ReadyCallback = Option<
    unsafe extern "C" fn(*mut NetConnection, *mut ::core::ffi::c_void) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NetStateFlags {
    pub connected: bool,
    pub disconnected: bool,
    pub discriminant: bool,
    pub handshake: HandShakeFlags,
    pub ready: ReadyFlags,
    pub reset: ResetFlags,
    pub agreement: Agreement,
    pub inputDelay: size_t,
    pub checksumInterval: size_t,
}
#[derive(Copy, Clone, ::c2rust_bitfields::BitfieldStruct)]
#[repr(C)]
pub struct Agreement {
    #[bitfield(name = "randomSeed", ty = "bool", bits = "0..=0")]
    pub randomSeed: [u8; 1],
}
#[derive(Copy, Clone, ::c2rust_bitfields::BitfieldStruct)]
#[repr(C)]
pub struct ResetFlags {
    #[bitfield(name = "localReset", ty = "bool", bits = "0..=0")]
    #[bitfield(name = "remoteReset", ty = "bool", bits = "1..=1")]
    pub localReset_remoteReset: [u8; 1],
}
#[derive(Copy, Clone, ::c2rust_bitfields::BitfieldStruct)]
#[repr(C)]
pub struct ReadyFlags {
    #[bitfield(name = "localReady", ty = "bool", bits = "0..=0")]
    #[bitfield(name = "remoteReady", ty = "bool", bits = "1..=1")]
    pub localReady_remoteReady: [u8; 1],
}
#[derive(Copy, Clone, ::c2rust_bitfields::BitfieldStruct)]
#[repr(C)]
pub struct HandShakeFlags {
    #[bitfield(name = "localOk", ty = "bool", bits = "0..=0")]
    #[bitfield(name = "remoteOk", ty = "bool", bits = "1..=1")]
    #[bitfield(name = "canceling", ty = "bool", bits = "2..=2")]
    pub localOk_remoteOk_canceling: [u8; 1],
}
#[inline]
unsafe extern "C" fn packetLength(mut packet: *const Packet) -> size_t {
    return ntoh16((*packet).header.len) as size_t;
}
#[inline]
unsafe extern "C" fn packetType(mut packet: *const Packet) -> PacketType {
    return PacketType(
        ntoh16((*packet).header.r#type) as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
}
#[inline]
unsafe extern "C" fn swapBytes16(mut x: uint16) -> uint16 {
    return ((x as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
        | x as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as uint16;
}
#[inline]
unsafe extern "C" fn ntoh16(mut x: uint16) -> uint16 {
    return swapBytes16(x);
}
#[no_mangle]
pub unsafe extern "C" fn sendPacket(
    mut conn: *mut NetConnection,
    mut packet: *mut Packet,
) -> ::core::ffi::c_int {
    (NetConnection_isConnected(conn) as ::core::ffi::c_int != 0
        || {
            __assert_fail(
                b"NetConnection_isConnected(conn)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/uqm/supermelee/netplay/netsend.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                44 as ::core::ffi::c_int,
                b"sendPacket\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    let mut result: ::core::ffi::c_int = WebTransport_send(
        packet as *const ::core::ffi::c_void,
        packetLength(packet),
    );
    if result != 1 as ::core::ffi::c_int {
        *__errno_location() = if result == 0 as ::core::ffi::c_int {
            6 as ::core::ffi::c_int
        } else {
            15 as ::core::ffi::c_int
        };
        return -1 as ::core::ffi::c_int;
    }
    let c2rust_lvalue_ptr = &raw mut (*NetConnection_getStatistics(conn)).packetsSent;
    *c2rust_lvalue_ptr = (*c2rust_lvalue_ptr).wrapping_add(1);
    let c2rust_lvalue_ptr_0 = &raw mut (*NetConnection_getStatistics(conn))
        .packetTypeSent[packetType(packet).0 as usize];
    *c2rust_lvalue_ptr_0 = (*c2rust_lvalue_ptr_0).wrapping_add(1);
    return 0 as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn WebTransport_send(
    mut data: *const ::core::ffi::c_void,
    mut size: size_t,
) -> ::core::ffi::c_int {
    let mut c2rust_lvalue: [::core::ffi::c_char; 3] = [
        'p' as ::core::ffi::c_char,
        'i' as ::core::ffi::c_char,
        '\0' as ::core::ffi::c_char,
    ];
    return emscripten_asm_const_int_sync_on_main_thread(
        ({
            extern "C" { static uqm_em_asm_7: [::core::ffi::c_char; 202]; }
            &raw const uqm_em_asm_7 as *const ::core::ffi::c_char
        }),
        &raw mut c2rust_lvalue as *mut ::core::ffi::c_char,
        data,
        size,
    );
}
