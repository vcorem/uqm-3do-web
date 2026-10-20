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

// Rust translation of src/uqm/supermelee/netplay/proto/ready.c.
use ::c2rust_bitfields;
extern "C" {
    pub type NetDescriptor;
    fn __assert_fail(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    ) -> !;
    fn NetConnection_getState(conn: *const NetConnection) -> NetState;
    fn NetConnection_setReadyCallback(
        conn: *mut NetConnection,
        callback: NetConnection_ReadyCallback,
        arg: *mut ::core::ffi::c_void,
    );
    fn sendReady(conn: *mut NetConnection);
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
unsafe extern "C" fn Netplay_bothReady(mut conn: *mut NetConnection) {
    let mut callback: NetConnection_ReadyCallback = None;
    let mut readyArg: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
        ::core::ffi::c_void,
    >();
    ((*conn).readyCallback.is_some()
        || {
            __assert_fail(
                b"conn->readyCallback != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/uqm/supermelee/netplay/proto/ready.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                34 as ::core::ffi::c_int,
                b"Netplay_bothReady\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    callback = (*conn).readyCallback;
    readyArg = (*conn).readyCallbackArg;
    NetConnection_setReadyCallback(
        conn,
        None,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    );
    uqm_lua_checked!(callback.expect("non-null function pointer")(conn, readyArg), ());
}
#[no_mangle]
pub unsafe extern "C" fn Netplay_localReady(
    mut conn: *mut NetConnection,
    mut callback: NetConnection_ReadyCallback,
    mut readyArg: *mut ::core::ffi::c_void,
    mut notifyRemote: bool,
) -> bool {
    (readyFlagsMeaningful(NetConnection_getState(conn)) as ::core::ffi::c_int != 0
        || {
            __assert_fail(
                b"readyFlagsMeaningful(NetConnection_getState(conn))\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/uqm/supermelee/netplay/proto/ready.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                54 as ::core::ffi::c_int,
                b"Netplay_localReady\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    (!(*conn).stateFlags.ready.localReady()
        || {
            __assert_fail(
                b"!conn->stateFlags.ready.localReady\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/uqm/supermelee/netplay/proto/ready.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                55 as ::core::ffi::c_int,
                b"Netplay_localReady\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    (callback.is_some()
        || {
            __assert_fail(
                b"callback != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/uqm/supermelee/netplay/proto/ready.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                56 as ::core::ffi::c_int,
                b"Netplay_localReady\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    NetConnection_setReadyCallback(conn, callback, readyArg);
    if notifyRemote {
        sendReady(conn);
    }
    if !(*conn).stateFlags.ready.remoteReady() {
        (*conn).stateFlags.ready.set_localReady(true as bool);
        return false;
    }
    (*conn).stateFlags.ready.set_remoteReady(false as bool);
    uqm_lua_checked!(Netplay_bothReady(conn), false);
    return true;
}
#[no_mangle]
pub unsafe extern "C" fn Netplay_remoteReady(mut conn: *mut NetConnection) -> bool {
    (readyFlagsMeaningful(NetConnection_getState(conn)) as ::core::ffi::c_int != 0
        || {
            __assert_fail(
                b"readyFlagsMeaningful(NetConnection_getState(conn))\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/uqm/supermelee/netplay/proto/ready.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                78 as ::core::ffi::c_int,
                b"Netplay_remoteReady\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    (!(*conn).stateFlags.ready.remoteReady()
        || {
            __assert_fail(
                b"!conn->stateFlags.ready.remoteReady\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/uqm/supermelee/netplay/proto/ready.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                81 as ::core::ffi::c_int,
                b"Netplay_remoteReady\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    if !(*conn).stateFlags.ready.localReady() {
        (*conn).stateFlags.ready.set_remoteReady(true as bool);
        return false;
    }
    (*conn).stateFlags.ready.set_localReady(false as bool);
    uqm_lua_checked!(Netplay_bothReady(conn), false);
    return true;
}
#[no_mangle]
pub unsafe extern "C" fn Netplay_isLocalReady(mut conn: *const NetConnection) -> bool {
    return (*conn).stateFlags.ready.localReady();
}
#[no_mangle]
pub unsafe extern "C" fn Netplay_isRemoteReady(mut conn: *const NetConnection) -> bool {
    return (*conn).stateFlags.ready.remoteReady();
}
#[inline]
unsafe extern "C" fn readyFlagsMeaningful(mut state: NetState) -> bool {
    return state.0 == NetState::NetState_init.0
        || state.0 == NetState::NetState_preBattle.0
        || state.0 == NetState::NetState_selectShip.0
        || state.0 == NetState::NetState_interBattle.0
        || state.0 == NetState::NetState_inBattle.0
        || state.0 == NetState::NetState_endingBattle.0
        || state.0 == NetState::NetState_endingBattle2.0;
}
