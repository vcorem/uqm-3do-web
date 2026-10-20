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

// Rust translation of src/uqm/supermelee/netplay/netrcv.c.
use ::c2rust_bitfields;
extern "C" {
    pub type NetDescriptor;
    fn NetConnection_webDisconnect(conn: *mut NetConnection, err: ::core::ffi::c_int);
    fn NetConnection_isConnected(conn: *const NetConnection) -> bool;
    fn NetConnection_getStatistics(conn: *mut NetConnection) -> *mut NetStatistics;
    static mut packetTypeData: [PacketTypeData; 0];
    fn memmove(
        _: *mut ::core::ffi::c_void,
        _: *const ::core::ffi::c_void,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn emscripten_asm_const_int_sync_on_main_thread(
        code: *const ::core::ffi::c_char,
        arg_sigs: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn log_add(_: log_Level, fmt: *const ::core::ffi::c_char, ...);
    fn __errno_location() -> *mut ::core::ffi::c_int;
}
pub type size_t = usize;
pub type ssize_t = isize;
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
unsafe extern "C" fn validPacketType(mut r#type: PacketType) -> bool {
    return r#type.0 < PacketType::PACKET_NUM.0;
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
unsafe extern "C" fn dataReceivedSingle(
    mut conn: *mut NetConnection,
    mut data: *const uint8,
    mut dataLen: size_t,
) -> ssize_t {
    let mut packetLen: uint32 = 0;
    let mut r#type: PacketType = PacketType::PACKET_INIT;
    let mut result: ::core::ffi::c_int = 0;
    if dataLen < ::core::mem::size_of::<PacketHeader>() {
        return 0 as ssize_t;
    }
    packetLen = packetLength(data as *const Packet) as uint32;
    r#type = packetType(data as *const Packet);
    if !validPacketType(r#type) {
        log_add(
            log_Level::log_Warning,
            b"Packet with invalid type %d received.\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            r#type.0,
        );
        *__errno_location() = 9 as ::core::ffi::c_int;
        return -1 as ssize_t;
    }
    if (packetLen as size_t)
        < (*(&raw mut packetTypeData as *mut PacketTypeData).offset(r#type.0 as isize))
            .len || packetLen > 2048 as uint32
    {
        log_add(
            log_Level::log_Warning,
            b"Packet with bad length field received (type=%s, lenfield=%d.\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            (*(&raw mut packetTypeData as *mut PacketTypeData).offset(r#type.0 as isize))
                .name,
            packetLen,
        );
        *__errno_location() = 9 as ::core::ffi::c_int;
        return -1 as ssize_t;
    }
    if dataLen < packetLen as size_t {
        return 0 as ssize_t;
    }
    let c2rust_lvalue_ptr = &raw mut (*NetConnection_getStatistics(conn))
        .packetsReceived;
    *c2rust_lvalue_ptr = (*c2rust_lvalue_ptr).wrapping_add(1);
    let c2rust_lvalue_ptr_0 = &raw mut (*NetConnection_getStatistics(conn))
        .packetTypeReceived[r#type.0 as usize];
    *c2rust_lvalue_ptr_0 = (*c2rust_lvalue_ptr_0).wrapping_add(1);
    result = (*(&raw mut packetTypeData as *mut PacketTypeData)
        .offset(r#type.0 as isize))
        .handler
        .expect("non-null function pointer")(conn, data as *const ::core::ffi::c_void);
    if result == -1 as ::core::ffi::c_int {
        return -1 as ssize_t;
    }
    return packetLen as ssize_t;
}
unsafe extern "C" fn dataReceivedMulti(
    mut conn: *mut NetConnection,
    mut data: *const uint8,
    mut len: size_t,
) -> ssize_t {
    let mut processed: size_t = 0;
    processed = 0 as size_t;
    while len > 0 as size_t {
        let mut packetLen: ssize_t = dataReceivedSingle(conn, data, len);
        if packetLen == -1 as ssize_t {
            return -1 as ssize_t;
        }
        if packetLen == 0 as ssize_t {
            break;
        }
        if !NetConnection_isConnected(conn) {
            return -1 as ssize_t;
        }
        processed = processed.wrapping_add(packetLen as size_t);
        data = data.offset(packetLen as isize);
        len = len.wrapping_sub(packetLen as size_t);
    }
    return processed as ssize_t;
}
#[no_mangle]
pub unsafe extern "C" fn webDataReady(mut conn: *mut NetConnection) {
    let mut iteration: ::core::ffi::c_uint = 0;
    iteration = 0 as ::core::ffi::c_uint;
    while iteration < 32 as ::core::ffi::c_uint {
        let mut remaining: size_t = (2048isize
            - (*conn).readEnd.offset_from((*conn).readBuf)) as size_t;
        let mut numRead: ::core::ffi::c_int = 0;
        let mut processed: ssize_t = 0;
        if remaining == 0 as size_t {
            NetConnection_webDisconnect(conn, 9 as ::core::ffi::c_int);
            return;
        }
        numRead = WebTransport_receive(
            (*conn).readEnd as *mut ::core::ffi::c_void,
            remaining,
        );
        if numRead < 0 as ::core::ffi::c_int {
            NetConnection_webDisconnect(conn, 15 as ::core::ffi::c_int);
            return;
        }
        if numRead == 0 as ::core::ffi::c_int {
            return;
        }
        (*conn).readEnd = (*conn).readEnd.offset(numRead as isize);
        processed = dataReceivedMulti(
            conn,
            (*conn).readBuf,
            (*conn).readEnd.offset_from((*conn).readBuf) as size_t,
        );
        if processed < 0 as ssize_t {
            NetConnection_webDisconnect(conn, *__errno_location());
            return;
        }
        if processed > 0 as ssize_t {
            memmove(
                (*conn).readBuf as *mut ::core::ffi::c_void,
                (*conn).readBuf.offset(processed as isize) as *const ::core::ffi::c_void,
                ((*conn).readEnd.offset_from((*conn).readBuf) - processed) as size_t,
            );
            (*conn).readEnd = (*conn).readEnd.offset(-(processed as isize));
        }
        iteration = iteration.wrapping_add(1);
    }
}
#[inline]
unsafe extern "C" fn WebTransport_receive(
    mut dest: *mut ::core::ffi::c_void,
    mut capacity: size_t,
) -> ::core::ffi::c_int {
    let mut c2rust_lvalue: [::core::ffi::c_char; 3] = [
        'p' as ::core::ffi::c_char,
        'i' as ::core::ffi::c_char,
        '\0' as ::core::ffi::c_char,
    ];
    return emscripten_asm_const_int_sync_on_main_thread(
        ({
            extern "C" { static uqm_em_asm_9: [::core::ffi::c_char; 484]; }
            &raw const uqm_em_asm_9 as *const ::core::ffi::c_char
        }),
        &raw mut c2rust_lvalue as *mut ::core::ffi::c_char,
        dest,
        capacity,
    );
}
