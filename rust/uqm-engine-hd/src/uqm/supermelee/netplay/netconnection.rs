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

// Rust translation of src/uqm/supermelee/netplay/netconnection.c.
use ::c2rust_bitfields;
extern "C" {
    pub type NetDescriptor;
    static mut netStateData: [NetStateData; 0];
    fn NetConnectionStateData_release(stateData: *mut NetConnectionStateData);
    fn malloc(_: size_t) -> *mut ::core::ffi::c_void;
    fn free(_: *mut ::core::ffi::c_void);
    fn webDataReady(conn: *mut NetConnection);
    fn emscripten_asm_const_int_sync_on_main_thread(
        code: *const ::core::ffi::c_char,
        arg_sigs: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn log_add(_: log_Level, fmt: *const ::core::ffi::c_char, ...);
    fn PacketQueue_init(queue: *mut PacketQueue);
    fn PacketQueue_uninit(queue: *mut PacketQueue);
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NetStateData {
    pub name: *const ::core::ffi::c_char,
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
#[no_mangle]
pub static mut Agreement_nothingAgreed: Agreement = Agreement { randomSeed: [0; 1] };
#[no_mangle]
pub unsafe extern "C" fn NetConnection_open(
    mut player: ::core::ffi::c_int,
    mut options: *const NetplayPeerOptions,
    mut connectCallback: NetConnection_ConnectCallback,
    mut closeCallback: NetConnection_CloseCallback,
    mut errorCallback: NetConnection_ErrorCallback,
    mut deleteCallback: NetConnection_DeleteCallback,
    mut extra: *mut ::core::ffi::c_void,
) -> *mut NetConnection {
    let mut conn: *mut NetConnection = ::core::ptr::null_mut::<NetConnection>();
    conn = malloc(::core::mem::size_of::<NetConnection>()) as *mut NetConnection;
    (*conn).nd = ::core::ptr::null_mut::<NetDescriptor>();
    (*conn).player = player;
    (*conn).state = NetState::NetState_unconnected;
    (*conn).options = options;
    (*conn).extra = extra;
    PacketQueue_init(&raw mut (*conn).queue);
    (*conn).connectCallback = connectCallback;
    (*conn).closeCallback = closeCallback;
    (*conn).errorCallback = errorCallback;
    (*conn).deleteCallback = deleteCallback;
    (*conn).readyCallback = None;
    (*conn).readyCallbackArg = ::core::ptr::null_mut::<::core::ffi::c_void>();
    (*conn).resetCallback = None;
    (*conn).resetCallbackArg = ::core::ptr::null_mut::<::core::ffi::c_void>();
    (*conn).readBuf = malloc(2048 as size_t) as *mut uint8;
    (*conn).readEnd = (*conn).readBuf;
    (*conn).stateData = ::core::ptr::null_mut::<NetConnectionStateData>();
    (*conn).stateFlags.connected = false;
    (*conn).stateFlags.disconnected = false;
    (*conn).stateFlags.discriminant = false;
    (*conn).stateFlags.handshake.set_localOk(false as bool);
    (*conn).stateFlags.handshake.set_remoteOk(false as bool);
    (*conn).stateFlags.handshake.set_canceling(false as bool);
    (*conn).stateFlags.ready.set_localReady(false as bool);
    (*conn).stateFlags.ready.set_remoteReady(false as bool);
    (*conn).stateFlags.reset.set_localReset(false as bool);
    (*conn).stateFlags.reset.set_remoteReset(false as bool);
    (*conn).stateFlags.agreement = Agreement_nothingAgreed;
    (*conn).stateFlags.inputDelay = 0 as size_t;
    (*conn).stateFlags.checksumInterval = 1 as size_t;
    let mut i: size_t = 0;
    (*conn).statistics.packetsReceived = 0 as size_t;
    (*conn).statistics.packetsSent = 0 as size_t;
    i = 0 as size_t;
    while i < PacketType::PACKET_NUM.0 as ::core::ffi::c_int as size_t {
        (*conn).statistics.packetTypeReceived[i] = 0 as size_t;
        (*conn).statistics.packetTypeSent[i] = 0 as size_t;
        i = i.wrapping_add(1);
    }
    NetConnection_setState(conn, NetState::NetState_connecting);
    return conn;
}
unsafe extern "C" fn NetConnection_doDeleteCallback(mut conn: *mut NetConnection) {
    if (*conn).deleteCallback.is_some() {
        uqm_lua_checked!((*conn).deleteCallback.expect("non-null function pointer")(conn), ());
    }
}
unsafe extern "C" fn NetConnection_delete(mut conn: *mut NetConnection) {
    uqm_lua_checked!(NetConnection_doDeleteCallback(conn), ());
    if !(*conn).stateData.is_null() {
        uqm_lua_checked!(NetConnectionStateData_release((*conn).stateData), ());
        (*conn).stateData = ::core::ptr::null_mut::<NetConnectionStateData>();
    }
    free((*conn).readBuf as *mut ::core::ffi::c_void);
    PacketQueue_uninit(&raw mut (*conn).queue);
    free(conn as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn Netplay_doCloseCallback(mut conn: *mut NetConnection) {
    if (*conn).closeCallback.is_some() {
        uqm_lua_checked!((*conn).closeCallback.expect("non-null function pointer")(conn), ());
    }
}
unsafe extern "C" fn NetConnection_doClose(mut conn: *mut NetConnection) {
    (*conn).stateFlags.connected = false;
    (*conn).stateFlags.disconnected = true;
    uqm_lua_checked!(Netplay_doCloseCallback(conn), ());
    NetConnection_setState(conn, NetState::NetState_unconnected);
}
#[no_mangle]
pub unsafe extern "C" fn NetConnection_webDisconnect(
    mut conn: *mut NetConnection,
    mut err: ::core::ffi::c_int,
) {
    if (*conn).stateFlags.disconnected {
        return;
    }
    WebTransport_close();
    if err != 0 as ::core::ffi::c_int {
        log_add(
            log_Level::log_Error,
            b"Web netplay transport failed (error %d).\0".as_ptr()
                as *const ::core::ffi::c_char,
            err,
        );
    }
    uqm_lua_checked!(NetConnection_doClose(conn), ());
}
#[no_mangle]
pub unsafe extern "C" fn NetConnection_webPoll(mut conn: *mut NetConnection) {
    let mut status: ::core::ffi::c_int = 0;
    if (*conn).stateFlags.disconnected {
        return;
    }
    status = WebTransport_status();
    if status < 0 as ::core::ffi::c_int {
        uqm_lua_checked!(NetConnection_webDisconnect(conn, 0 as ::core::ffi::c_int), ());
        return;
    }
    if status == 0 as ::core::ffi::c_int {
        return;
    }
    if !(*conn).stateFlags.connected {
        (*conn).stateFlags.discriminant = status == 1 as ::core::ffi::c_int;
        (*conn).stateFlags.connected = true;
        NetConnection_setState(conn, NetState::NetState_init);
        if (*conn).connectCallback.is_some() {
            uqm_lua_checked!((*conn).connectCallback.expect("non-null function pointer")(conn), ());
        }
    }
    uqm_lua_checked!(webDataReady(conn), ());
}
#[no_mangle]
pub unsafe extern "C" fn NetConnection_close(mut conn: *mut NetConnection) {
    WebTransport_close();
    if !(*conn).stateFlags.disconnected {
        uqm_lua_checked!(NetConnection_doClose(conn), ());
    }
    uqm_lua_checked!(NetConnection_delete(conn), ());
}
#[no_mangle]
pub unsafe extern "C" fn NetConnection_doErrorCallback(
    mut nd: *mut NetConnection,
    mut err: ::core::ffi::c_int,
) {
    let mut error: NetConnectionError = NetConnectionError {
        state: NetState::NetState_unconnected,
        err: 0,
        extra: C2Rust_Unnamed {
            listenError: ::core::ptr::null::<ListenError>(),
        },
    };
    if (*nd).errorCallback.is_some() {
        error.state = (*nd).state;
        error.err = err;
        error.extra.connectError = ::core::ptr::null::<ConnectError>();
        uqm_lua_checked!(Some((*nd).errorCallback.expect("non-null function pointer"))
            .expect("non-null function pointer")(nd, &raw mut error), ());
    }
}
#[no_mangle]
pub unsafe extern "C" fn NetConnection_setStateData(
    mut conn: *mut NetConnection,
    mut stateData: *mut NetConnectionStateData,
) {
    (*conn).stateData = stateData;
}
#[no_mangle]
pub unsafe extern "C" fn NetConnection_getStateData(
    mut conn: *const NetConnection,
) -> *mut NetConnectionStateData {
    return (*conn).stateData;
}
#[no_mangle]
pub unsafe extern "C" fn NetConnection_setExtra(
    mut conn: *mut NetConnection,
    mut extra: *mut ::core::ffi::c_void,
) {
    (*conn).extra = extra;
}
#[no_mangle]
pub unsafe extern "C" fn NetConnection_getExtra(
    mut conn: *const NetConnection,
) -> *mut ::core::ffi::c_void {
    return (*conn).extra;
}
#[no_mangle]
pub unsafe extern "C" fn NetConnection_setReadyCallback(
    mut conn: *mut NetConnection,
    mut callback: NetConnection_ReadyCallback,
    mut arg: *mut ::core::ffi::c_void,
) {
    (*conn).readyCallback = callback;
    (*conn).readyCallbackArg = arg;
}
#[no_mangle]
pub unsafe extern "C" fn NetConnection_getReadyCallback(
    mut conn: *const NetConnection,
) -> NetConnection_ReadyCallback {
    return (*conn).readyCallback;
}
#[no_mangle]
pub unsafe extern "C" fn NetConnection_getReadyCallbackArg(
    mut conn: *const NetConnection,
) -> *mut ::core::ffi::c_void {
    return (*conn).readyCallbackArg;
}
#[no_mangle]
pub unsafe extern "C" fn NetConnection_setResetCallback(
    mut conn: *mut NetConnection,
    mut callback: NetConnection_ResetCallback,
    mut arg: *mut ::core::ffi::c_void,
) {
    (*conn).resetCallback = callback;
    (*conn).resetCallbackArg = arg;
}
#[no_mangle]
pub unsafe extern "C" fn NetConnection_getResetCallback(
    mut conn: *const NetConnection,
) -> NetConnection_ResetCallback {
    return (*conn).resetCallback;
}
#[no_mangle]
pub unsafe extern "C" fn NetConnection_getResetCallbackArg(
    mut conn: *const NetConnection,
) -> *mut ::core::ffi::c_void {
    return (*conn).resetCallbackArg;
}
#[no_mangle]
pub unsafe extern "C" fn NetConnection_setState(
    mut conn: *mut NetConnection,
    mut state: NetState,
) {
    if state.0 == (*conn).state.0 {
        log_add(
            log_Level::log_Warning,
            b"NETPLAY: [%d]     Connection state set to %s while already in that state.\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
            (*conn).player,
            (*(&raw mut netStateData as *mut NetStateData).offset(state.0 as isize)).name,
        );
    }
    (*conn).state = state;
    WebTransport_engineState(
        (*(&raw mut netStateData as *mut NetStateData).offset(state.0 as isize)).name,
        (*conn).player,
    );
}
#[no_mangle]
pub unsafe extern "C" fn NetConnection_getState(
    mut conn: *const NetConnection,
) -> NetState {
    return (*conn).state;
}
#[no_mangle]
pub unsafe extern "C" fn NetConnection_getDiscriminant(
    mut conn: *const NetConnection,
) -> bool {
    return (*conn).stateFlags.discriminant;
}
#[no_mangle]
pub unsafe extern "C" fn NetConnection_getPeerOptions(
    mut conn: *const NetConnection,
) -> *const NetplayPeerOptions {
    return (*conn).options;
}
#[no_mangle]
pub unsafe extern "C" fn NetConnection_isConnected(
    mut conn: *const NetConnection,
) -> bool {
    return (*conn).stateFlags.connected;
}
#[no_mangle]
pub unsafe extern "C" fn NetConnection_getPlayerNr(
    mut conn: *const NetConnection,
) -> ::core::ffi::c_int {
    return (*conn).player;
}
#[no_mangle]
pub unsafe extern "C" fn NetConnection_getInputDelay(
    mut conn: *const NetConnection,
) -> size_t {
    return (*conn).stateFlags.inputDelay;
}
#[no_mangle]
pub unsafe extern "C" fn NetConnection_getChecksumBuffer(
    mut conn: *mut NetConnection,
) -> *mut ChecksumBuffer {
    return &raw mut (*conn).checksumBuffer;
}
#[no_mangle]
pub unsafe extern "C" fn NetConnection_getChecksumInterval(
    mut conn: *const NetConnection,
) -> size_t {
    return (*conn).stateFlags.checksumInterval;
}
#[no_mangle]
pub unsafe extern "C" fn NetConnection_getStatistics(
    mut conn: *mut NetConnection,
) -> *mut NetStatistics {
    return &raw mut (*conn).statistics;
}
#[inline]
unsafe extern "C" fn WebTransport_status() -> ::core::ffi::c_int {
    let mut c2rust_lvalue: [::core::ffi::c_char; 1] = ['\0' as ::core::ffi::c_char];
    return emscripten_asm_const_int_sync_on_main_thread(
        ({
            extern "C" { static uqm_em_asm_5: [::core::ffi::c_char; 191]; }
            &raw const uqm_em_asm_5 as *const ::core::ffi::c_char
        }),
        &raw mut c2rust_lvalue as *mut ::core::ffi::c_char,
    );
}
#[inline]
unsafe extern "C" fn WebTransport_close() {
    let mut c2rust_lvalue: [::core::ffi::c_char; 1] = ['\0' as ::core::ffi::c_char];
    emscripten_asm_const_int_sync_on_main_thread(
        ({
            extern "C" { static uqm_em_asm_6: [::core::ffi::c_char; 46]; }
            &raw const uqm_em_asm_6 as *const ::core::ffi::c_char
        }),
        &raw mut c2rust_lvalue as *mut ::core::ffi::c_char,
    );
}
#[inline]
unsafe extern "C" fn WebTransport_engineState(
    mut state: *const ::core::ffi::c_char,
    mut player: ::core::ffi::c_int,
) {
    let mut c2rust_lvalue: [::core::ffi::c_char; 3] = [
        'p' as ::core::ffi::c_char,
        'i' as ::core::ffi::c_char,
        '\0' as ::core::ffi::c_char,
    ];
    emscripten_asm_const_int_sync_on_main_thread(
        ({
            extern "C" { static uqm_em_asm_7: [::core::ffi::c_char; 118]; }
            &raw const uqm_em_asm_7 as *const ::core::ffi::c_char
        }),
        &raw mut c2rust_lvalue as *mut ::core::ffi::c_char,
        state,
        player,
    );
}
