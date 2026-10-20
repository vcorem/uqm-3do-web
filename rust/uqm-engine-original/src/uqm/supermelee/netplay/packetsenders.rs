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

// Rust translation of src/uqm/supermelee/netplay/packetsenders.c.
extern "C" {
    pub type NetConnection;
    fn Packet_Init_create() -> *mut Packet_Init;
    fn Packet_Ping_create(id: uint32) -> *mut Packet_Ping;
    fn Packet_Ack_create(id: uint32) -> *mut Packet_Ack;
    fn Packet_Ready_create() -> *mut Packet_Ready;
    fn Packet_Handshake0_create() -> *mut Packet_Handshake0;
    fn Packet_Handshake1_create() -> *mut Packet_Handshake1;
    fn Packet_HandshakeCancel_create() -> *mut Packet_HandshakeCancel;
    fn Packet_HandshakeCancelAck_create() -> *mut Packet_HandshakeCancelAck;
    fn Packet_SeedRandom_create(seed: uint32) -> *mut Packet_SeedRandom;
    fn Packet_InputDelay_create(delay: uint32) -> *mut Packet_InputDelay;
    fn Packet_Fleet_create(side: NetplaySide, numShips: size_t) -> *mut Packet_Fleet;
    fn Packet_TeamName_create(
        side: NetplaySide,
        name: *const ::core::ffi::c_char,
        size: size_t,
    ) -> *mut Packet_TeamName;
    fn Packet_SelectShip_create(ship: uint16) -> *mut Packet_SelectShip;
    fn Packet_BattleInput_create(state: uint8) -> *mut Packet_BattleInput;
    fn Packet_FrameCount_create(frameCount: uint32) -> *mut Packet_FrameCount;
    fn Packet_Checksum_create(frameNr: uint32, checksum: uint32) -> *mut Packet_Checksum;
    fn Packet_Abort_create(reason: uint16) -> *mut Packet_Abort;
    fn Packet_Reset_create(reason: uint16) -> *mut Packet_Reset;
    fn queuePacket(conn: *mut NetConnection, packet: *mut Packet);
}
pub type uint8 = ::core::ffi::c_uchar;
pub type uint16 = ::core::ffi::c_ushort;
pub type uint32 = ::core::ffi::c_uint;
pub type size_t = usize;
pub type Checksum = uint32;
pub type UBYTE = uint8;
pub type UWORD = uint16;
pub type DWORD = uint32;
pub type COUNT = UWORD;
pub type BattleFrameCounter = DWORD;
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct NetplayAbortReason(pub ::core::ffi::c_uint);
impl NetplayAbortReason {
    pub const AbortReason_unspecified: Self = Self(0);
    pub const AbortReason_versionMismatch: Self = Self(1);
    pub const AbortReason_invalidHash: Self = Self(2);
    pub const AbortReason_protocolError: Self = Self(3);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct NetplayResetReason(pub ::core::ffi::c_uint);
impl NetplayResetReason {
    pub const ResetReason_unspecified: Self = Self(0);
    pub const ResetReason_syncLoss: Self = Self(1);
    pub const ResetReason_manualReset: Self = Self(2);
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Packet_Ping {
    pub header: PacketHeader,
    pub id: uint32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Packet_Ack {
    pub header: PacketHeader,
    pub id: uint32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Packet_Ready {
    pub header: PacketHeader,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Packet_SeedRandom {
    pub header: PacketHeader,
    pub seed: uint32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Packet_InputDelay {
    pub header: PacketHeader,
    pub delay: uint32,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct NetplaySide(pub ::core::ffi::c_uint);
impl NetplaySide {
    pub const NetplaySide_local: Self = Self(0);
    pub const NetplaySide_remote: Self = Self(1);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FleetEntry {
    pub index: uint8,
    pub ship: uint8,
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
pub struct Packet_TeamName {
    pub header: PacketHeader,
    pub side: uint8,
    pub padding: uint8,
    pub name: [uint8; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Packet_Handshake0 {
    pub header: PacketHeader,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Packet_Handshake1 {
    pub header: PacketHeader,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Packet_HandshakeCancel {
    pub header: PacketHeader,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Packet_HandshakeCancelAck {
    pub header: PacketHeader,
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
pub struct Packet_BattleInput {
    pub header: PacketHeader,
    pub state: uint8,
    pub padding0: uint8,
    pub padding1: uint16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Packet_FrameCount {
    pub header: PacketHeader,
    pub frameCount: uint32,
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
pub struct Packet_Abort {
    pub header: PacketHeader,
    pub reason: uint16,
    pub padding0: uint16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Packet_Reset {
    pub header: PacketHeader,
    pub reason: uint16,
    pub padding0: uint16,
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
#[no_mangle]
pub unsafe extern "C" fn sendInit(mut conn: *mut NetConnection) {
    let mut packet: *mut Packet_Init = ::core::ptr::null_mut::<Packet_Init>();
    packet = Packet_Init_create();
    queuePacket(conn, packet as *mut Packet);
}
#[no_mangle]
pub unsafe extern "C" fn sendPing(mut conn: *mut NetConnection, mut id: uint32) {
    let mut packet: *mut Packet_Ping = ::core::ptr::null_mut::<Packet_Ping>();
    packet = Packet_Ping_create(id);
    queuePacket(conn, packet as *mut Packet);
}
#[no_mangle]
pub unsafe extern "C" fn sendAck(mut conn: *mut NetConnection, mut id: uint32) {
    let mut packet: *mut Packet_Ack = ::core::ptr::null_mut::<Packet_Ack>();
    packet = Packet_Ack_create(id);
    queuePacket(conn, packet as *mut Packet);
}
#[no_mangle]
pub unsafe extern "C" fn sendReady(mut conn: *mut NetConnection) {
    let mut packet: *mut Packet_Ready = ::core::ptr::null_mut::<Packet_Ready>();
    packet = Packet_Ready_create();
    queuePacket(conn, packet as *mut Packet);
}
#[no_mangle]
pub unsafe extern "C" fn sendHandshake0(mut conn: *mut NetConnection) {
    let mut packet: *mut Packet_Handshake0 = ::core::ptr::null_mut::<
        Packet_Handshake0,
    >();
    packet = Packet_Handshake0_create();
    queuePacket(conn, packet as *mut Packet);
}
#[no_mangle]
pub unsafe extern "C" fn sendHandshake1(mut conn: *mut NetConnection) {
    let mut packet: *mut Packet_Handshake1 = ::core::ptr::null_mut::<
        Packet_Handshake1,
    >();
    packet = Packet_Handshake1_create();
    queuePacket(conn, packet as *mut Packet);
}
#[no_mangle]
pub unsafe extern "C" fn sendHandshakeCancel(mut conn: *mut NetConnection) {
    let mut packet: *mut Packet_HandshakeCancel = ::core::ptr::null_mut::<
        Packet_HandshakeCancel,
    >();
    packet = Packet_HandshakeCancel_create();
    queuePacket(conn, packet as *mut Packet);
}
#[no_mangle]
pub unsafe extern "C" fn sendHandshakeCancelAck(mut conn: *mut NetConnection) {
    let mut packet: *mut Packet_HandshakeCancelAck = ::core::ptr::null_mut::<
        Packet_HandshakeCancelAck,
    >();
    packet = Packet_HandshakeCancelAck_create();
    queuePacket(conn, packet as *mut Packet);
}
#[no_mangle]
pub unsafe extern "C" fn sendTeamName(
    mut conn: *mut NetConnection,
    mut side: NetplaySide,
    mut name: *const ::core::ffi::c_char,
    mut len: size_t,
) {
    let mut packet: *mut Packet_TeamName = ::core::ptr::null_mut::<Packet_TeamName>();
    packet = Packet_TeamName_create(side, name, len);
    queuePacket(conn, packet as *mut Packet);
}
#[no_mangle]
pub unsafe extern "C" fn sendFleet(
    mut conn: *mut NetConnection,
    mut side: NetplaySide,
    mut ships: *const MeleeShip,
    mut shipCount: size_t,
) {
    let mut i: size_t = 0;
    let mut packet: *mut Packet_Fleet = ::core::ptr::null_mut::<Packet_Fleet>();
    packet = Packet_Fleet_create(side, shipCount);
    i = 0 as size_t;
    while i < shipCount {
        (*(&raw mut (*packet).ships as *mut FleetEntry).offset(i as isize)).index = i
            as uint8;
        (*(&raw mut (*packet).ships as *mut FleetEntry).offset(i as isize)).ship = (*ships
            .offset(i as isize))
            .0 as uint8;
        i = i.wrapping_add(1);
    }
    queuePacket(conn, packet as *mut Packet);
}
#[no_mangle]
pub unsafe extern "C" fn sendFleetShip(
    mut conn: *mut NetConnection,
    mut side: NetplaySide,
    mut shipIndex: FleetShipIndex,
    mut ship: MeleeShip,
) {
    let mut packet: *mut Packet_Fleet = ::core::ptr::null_mut::<Packet_Fleet>();
    packet = Packet_Fleet_create(side, 1 as size_t);
    (*(&raw mut (*packet).ships as *mut FleetEntry).offset(0isize)).index = shipIndex
        as uint8;
    (*(&raw mut (*packet).ships as *mut FleetEntry).offset(0isize)).ship = ship.0
        as uint8;
    queuePacket(conn, packet as *mut Packet);
}
#[no_mangle]
pub unsafe extern "C" fn sendSeedRandom(mut conn: *mut NetConnection, mut seed: uint32) {
    let mut packet: *mut Packet_SeedRandom = ::core::ptr::null_mut::<
        Packet_SeedRandom,
    >();
    packet = Packet_SeedRandom_create(seed);
    queuePacket(conn, packet as *mut Packet);
}
#[no_mangle]
pub unsafe extern "C" fn sendInputDelay(
    mut conn: *mut NetConnection,
    mut delay: uint32,
) {
    let mut packet: *mut Packet_InputDelay = ::core::ptr::null_mut::<
        Packet_InputDelay,
    >();
    packet = Packet_InputDelay_create(delay);
    queuePacket(conn, packet as *mut Packet);
}
#[no_mangle]
pub unsafe extern "C" fn sendSelectShip(
    mut conn: *mut NetConnection,
    mut index: FleetShipIndex,
) {
    let mut packet: *mut Packet_SelectShip = ::core::ptr::null_mut::<
        Packet_SelectShip,
    >();
    packet = Packet_SelectShip_create(index);
    queuePacket(conn, packet as *mut Packet);
}
#[no_mangle]
pub unsafe extern "C" fn sendBattleInput(
    mut conn: *mut NetConnection,
    mut input: BATTLE_INPUT_STATE,
) {
    let mut packet: *mut Packet_BattleInput = ::core::ptr::null_mut::<
        Packet_BattleInput,
    >();
    packet = Packet_BattleInput_create(input);
    queuePacket(conn, packet as *mut Packet);
}
#[no_mangle]
pub unsafe extern "C" fn sendFrameCount(
    mut conn: *mut NetConnection,
    mut frameCount: BattleFrameCounter,
) {
    let mut packet: *mut Packet_FrameCount = ::core::ptr::null_mut::<
        Packet_FrameCount,
    >();
    packet = Packet_FrameCount_create(frameCount);
    queuePacket(conn, packet as *mut Packet);
}
#[no_mangle]
pub unsafe extern "C" fn sendChecksum(
    mut conn: *mut NetConnection,
    mut frameNr: BattleFrameCounter,
    mut checksum: Checksum,
) {
    let mut packet: *mut Packet_Checksum = ::core::ptr::null_mut::<Packet_Checksum>();
    packet = Packet_Checksum_create(frameNr, checksum);
    queuePacket(conn, packet as *mut Packet);
}
#[no_mangle]
pub unsafe extern "C" fn sendAbort(
    mut conn: *mut NetConnection,
    mut reason: NetplayAbortReason,
) {
    let mut packet: *mut Packet_Abort = ::core::ptr::null_mut::<Packet_Abort>();
    packet = Packet_Abort_create(reason.0 as uint16);
    queuePacket(conn, packet as *mut Packet);
}
#[no_mangle]
pub unsafe extern "C" fn sendReset(
    mut conn: *mut NetConnection,
    mut reason: NetplayResetReason,
) {
    let mut packet: *mut Packet_Reset = ::core::ptr::null_mut::<Packet_Reset>();
    packet = Packet_Reset_create(reason.0 as uint16);
    queuePacket(conn, packet as *mut Packet);
}
