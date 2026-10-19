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

// This files contains functions that notify the other side of local
// changes.

// Rust translation of src/uqm/supermelee/netplay/notify.c.
use ::c2rust_bitfields;
extern "C" {
    pub type NetDescriptor;
    fn __assert_fail(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    );
    fn NetConnection_getState(conn: *const NetConnection) -> NetState;
    fn sendTeamName(
        conn: *mut NetConnection,
        side: NetplaySide,
        name: *const ::core::ffi::c_char,
        len: size_t,
    );
    fn sendFleet(
        conn: *mut NetConnection,
        side: NetplaySide,
        ships: *const MeleeShip,
        numShips: size_t,
    );
    fn sendFleetShip(
        conn: *mut NetConnection,
        player: NetplaySide,
        shipIndex: FleetShipIndex,
        ship: MeleeShip,
    );
    fn sendSeedRandom(conn: *mut NetConnection, seed: uint32);
    fn sendInputDelay(conn: *mut NetConnection, delay: uint32);
    fn sendSelectShip(conn: *mut NetConnection, index: FleetShipIndex);
    fn sendBattleInput(conn: *mut NetConnection, input: BATTLE_INPUT_STATE);
    fn sendFrameCount(conn: *mut NetConnection, frameCount: BattleFrameCounter);
    fn sendChecksum(
        conn: *mut NetConnection,
        frameNr: BattleFrameCounter,
        checksum: Checksum,
    );
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
pub type UBYTE = uint8;
pub type UWORD = uint16;
pub type DWORD = uint32;
pub type COUNT = UWORD;
pub type BattleFrameCounter = DWORD;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct NetplaySide(pub ::core::ffi::c_uint);
impl NetplaySide {
    pub const NetplaySide_local: Self = Self(0);
    pub const NetplaySide_remote: Self = Self(1);
}
pub type BATTLE_INPUT_STATE = UBYTE;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct MeleeShip(pub ::core::ffi::c_uint);
impl MeleeShip {
    pub const MELEE_ANDROSYNTH: Self = Self(0);
    pub const MELEE_ARILOU: Self = Self(1);
    pub const MELEE_CHENJESU: Self = Self(2);
    pub const MELEE_CHMMR: Self = Self(3);
    pub const MELEE_DRUUGE: Self = Self(4);
    pub const MELEE_EARTHLING: Self = Self(5);
    pub const MELEE_ILWRATH: Self = Self(6);
    pub const MELEE_KOHR_AH: Self = Self(7);
    pub const MELEE_MELNORME: Self = Self(8);
    pub const MELEE_MMRNMHRM: Self = Self(9);
    pub const MELEE_MYCON: Self = Self(10);
    pub const MELEE_ORZ: Self = Self(11);
    pub const MELEE_PKUNK: Self = Self(12);
    pub const MELEE_SHOFIXTI: Self = Self(13);
    pub const MELEE_SLYLANDRO: Self = Self(14);
    pub const MELEE_SPATHI: Self = Self(15);
    pub const MELEE_SUPOX: Self = Self(16);
    pub const MELEE_SYREEN: Self = Self(17);
    pub const MELEE_THRADDASH: Self = Self(18);
    pub const MELEE_UMGAH: Self = Self(19);
    pub const MELEE_URQUAN: Self = Self(20);
    pub const MELEE_UTWIG: Self = Self(21);
    pub const MELEE_VUX: Self = Self(22);
    pub const MELEE_YEHAT: Self = Self(23);
    pub const MELEE_ZOQFOTPIK: Self = Self(24);
    pub const MELEE_UNSET: Self = Self(254);
    pub const MELEE_NONE: Self = Self(255);
}
pub type FleetShipIndex = COUNT;
#[inline]
unsafe extern "C" fn NetState_battleActive(mut state: NetState) -> bool {
    return state.0 == NetState::NetState_inBattle.0
        || state.0 == NetState::NetState_endingBattle.0
        || state.0 == NetState::NetState_endingBattle2.0;
}
#[inline]
unsafe extern "C" fn netSide(
    mut conn: *mut NetConnection,
    mut side: ::core::ffi::c_int,
) -> NetplaySide {
    if side == (*conn).player {
        return NetplaySide::NetplaySide_remote;
    }
    return NetplaySide::NetplaySide_local;
}
#[no_mangle]
pub unsafe extern "C" fn Netplay_Notify_shipSelected(
    mut conn: *mut NetConnection,
    mut index: FleetShipIndex,
) {
    (NetConnection_getState(conn).0 == NetState::NetState_selectShip.0
        || {
            __assert_fail(
                b"NetConnection_getState(conn) == NetState_selectShip\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/uqm/supermelee/netplay/notify.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                41 as ::core::ffi::c_int,
                b"Netplay_Notify_shipSelected\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    sendSelectShip(conn, index);
}
#[no_mangle]
pub unsafe extern "C" fn Netplay_Notify_battleInput(
    mut conn: *mut NetConnection,
    mut input: BATTLE_INPUT_STATE,
) {
    (NetConnection_getState(conn).0 == NetState::NetState_inBattle.0
        || NetConnection_getState(conn).0 == NetState::NetState_endingBattle.0
        || NetConnection_getState(conn).0 == NetState::NetState_endingBattle2.0
        || {
            __assert_fail(
                b"NetConnection_getState(conn) == NetState_inBattle || NetConnection_getState(conn) == NetState_endingBattle || NetConnection_getState(conn) == NetState_endingBattle2\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"src/uqm/supermelee/netplay/notify.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                50 as ::core::ffi::c_int,
                b"Netplay_Notify_battleInput\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    sendBattleInput(conn, input);
}
#[no_mangle]
pub unsafe extern "C" fn Netplay_Notify_setTeamName(
    mut conn: *mut NetConnection,
    mut player: ::core::ffi::c_int,
    mut name: *const ::core::ffi::c_char,
    mut len: size_t,
) {
    (NetConnection_getState(conn).0 == NetState::NetState_inSetup.0
        || {
            __assert_fail(
                b"NetConnection_getState(conn) == NetState_inSetup\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/uqm/supermelee/netplay/notify.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                58 as ::core::ffi::c_int,
                b"Netplay_Notify_setTeamName\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    (!(*conn).stateFlags.handshake.localOk()
        || {
            __assert_fail(
                b"!conn->stateFlags.handshake.localOk\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/uqm/supermelee/netplay/notify.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                59 as ::core::ffi::c_int,
                b"Netplay_Notify_setTeamName\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    sendTeamName(conn, netSide(conn, player), name, len);
}
#[no_mangle]
pub unsafe extern "C" fn Netplay_Notify_setFleet(
    mut conn: *mut NetConnection,
    mut player: ::core::ffi::c_int,
    mut fleet: *const MeleeShip,
    mut fleetSize: size_t,
) {
    (NetConnection_getState(conn).0 == NetState::NetState_inSetup.0
        || {
            __assert_fail(
                b"NetConnection_getState(conn) == NetState_inSetup\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/uqm/supermelee/netplay/notify.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                68 as ::core::ffi::c_int,
                b"Netplay_Notify_setFleet\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    (!(*conn).stateFlags.handshake.localOk()
        || {
            __assert_fail(
                b"!conn->stateFlags.handshake.localOk\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/uqm/supermelee/netplay/notify.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                69 as ::core::ffi::c_int,
                b"Netplay_Notify_setFleet\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    sendFleet(conn, netSide(conn, player), fleet, fleetSize);
}
#[no_mangle]
pub unsafe extern "C" fn Netplay_Notify_setShip(
    mut conn: *mut NetConnection,
    mut player: ::core::ffi::c_int,
    mut index: FleetShipIndex,
    mut ship: MeleeShip,
) {
    (NetConnection_getState(conn).0 == NetState::NetState_inSetup.0
        || {
            __assert_fail(
                b"NetConnection_getState(conn) == NetState_inSetup\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/uqm/supermelee/netplay/notify.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                77 as ::core::ffi::c_int,
                b"Netplay_Notify_setShip\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    (!(*conn).stateFlags.handshake.localOk()
        || {
            __assert_fail(
                b"!conn->stateFlags.handshake.localOk\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/uqm/supermelee/netplay/notify.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                78 as ::core::ffi::c_int,
                b"Netplay_Notify_setShip\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    sendFleetShip(conn, netSide(conn, player), index, ship);
}
#[no_mangle]
pub unsafe extern "C" fn Netplay_Notify_seedRandom(
    mut conn: *mut NetConnection,
    mut seed: uint32,
) {
    (NetConnection_getState(conn).0 == NetState::NetState_preBattle.0
        || {
            __assert_fail(
                b"NetConnection_getState(conn) == NetState_preBattle\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/uqm/supermelee/netplay/notify.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                85 as ::core::ffi::c_int,
                b"Netplay_Notify_seedRandom\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    sendSeedRandom(conn, seed);
    (*conn).stateFlags.agreement.set_randomSeed(true as bool);
}
#[no_mangle]
pub unsafe extern "C" fn Netplay_Notify_inputDelay(
    mut conn: *mut NetConnection,
    mut delay: uint32,
) {
    (NetConnection_getState(conn).0 == NetState::NetState_preBattle.0
        || {
            __assert_fail(
                b"NetConnection_getState(conn) == NetState_preBattle\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/uqm/supermelee/netplay/notify.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                93 as ::core::ffi::c_int,
                b"Netplay_Notify_inputDelay\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    sendInputDelay(conn, delay);
}
#[no_mangle]
pub unsafe extern "C" fn Netplay_Notify_frameCount(
    mut conn: *mut NetConnection,
    mut frameCount: BattleFrameCounter,
) {
    (NetConnection_getState(conn).0 == NetState::NetState_endingBattle.0
        || {
            __assert_fail(
                b"NetConnection_getState(conn) == NetState_endingBattle\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/uqm/supermelee/netplay/notify.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                101 as ::core::ffi::c_int,
                b"Netplay_Notify_frameCount\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    sendFrameCount(conn, frameCount);
}
#[no_mangle]
pub unsafe extern "C" fn Netplay_Notify_checksum(
    mut conn: *mut NetConnection,
    mut frameNr: BattleFrameCounter,
    mut checksum: Checksum,
) {
    (NetState_battleActive(NetConnection_getState(conn)) as ::core::ffi::c_int != 0
        || {
            __assert_fail(
                b"NetState_battleActive(NetConnection_getState(conn))\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/uqm/supermelee/netplay/notify.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                110 as ::core::ffi::c_int,
                b"Netplay_Notify_checksum\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    sendChecksum(conn, frameNr, checksum);
}
