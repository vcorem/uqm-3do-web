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

// Rust translation of src/uqm/supermelee/netplay/packethandlers.c.
use ::c2rust_bitfields;
extern "C" {
    pub type NetDescriptor;
    pub type string_table_entry;
    pub type tfb_soundsample;
    pub type RandomContext;
    pub type MeleeTeam;
    pub type MeleeSetup;
    pub type getmelee_struct;
    static mut packetTypeData: [PacketTypeData; 0];
    fn __assert_fail(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    ) -> !;
    fn memcpy(
        _: *mut ::core::ffi::c_void,
        _: *const ::core::ffi::c_void,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    static mut battleFrameCount: BattleFrameCounter;
    fn getBattleInputDelay() -> size_t;
    fn BattleInputBuffer_push(
        bib: *mut BattleInputBuffer,
        input: BATTLE_INPUT_STATE,
    ) -> bool;
    fn getBattleInputBuffer(player: size_t) -> *mut BattleInputBuffer;
    fn updateRandomSeed(pMS: *mut MELEE_STATE, side: COUNT, seed: DWORD);
    fn confirmationCancelled(pMS: *mut MELEE_STATE, side: COUNT);
    fn abortFeedback(conn: *mut NetConnection, reason: NetplayAbortReason);
    fn Melee_RemoteChange_ship(
        pMS: *mut MELEE_STATE,
        conn: *mut NetConnection,
        side: COUNT,
        index: FleetShipIndex,
        ship: MeleeShip,
    );
    fn Melee_RemoteChange_teamName(
        pMS: *mut MELEE_STATE,
        conn: *mut NetConnection,
        side: COUNT,
        name: *const ::core::ffi::c_char,
    );
    fn addRemoteChecksum(
        conn: *mut NetConnection,
        frameNr: BattleFrameCounter,
        checksum: Checksum,
    );
    fn NetConnection_getStateData(
        conn: *const NetConnection,
    ) -> *mut NetConnectionStateData;
    fn NetConnection_setState(conn: *mut NetConnection, state: NetState);
    fn NetConnection_getChecksumInterval(conn: *const NetConnection) -> size_t;
    fn sendAck(conn: *mut NetConnection, id: uint32);
    fn sendHandshake0(conn: *mut NetConnection);
    fn sendHandshake1(conn: *mut NetConnection);
    fn sendHandshakeCancelAck(conn: *mut NetConnection);
    fn sendAbort(conn: *mut NetConnection, reason: NetplayAbortReason);
    fn Netplay_cancelConfirmation(conn: *mut NetConnection) -> ::core::ffi::c_int;
    fn Netplay_remoteReady(conn: *mut NetConnection) -> bool;
    fn Netplay_remoteReset(conn: *mut NetConnection, reason: NetplayResetReason);
    fn log_add(_: log_Level, fmt: *const ::core::ffi::c_char, ...);
    fn updateMeleeSelection(
        gms: *mut GETMELEE_STATE,
        player: COUNT,
        ship: COUNT,
    ) -> bool;
    fn __errno_location() -> *mut ::core::ffi::c_int;
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
pub type BYTE = uint8;
pub type UBYTE = uint8;
pub type UWORD = uint16;
pub type DWORD = uint32;
pub type COUNT = UWORD;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct BOOLEAN(pub ::core::ffi::c_uint);
impl BOOLEAN {
    pub const FALSE: Self = Self(0);
    pub const TRUE: Self = Self(1);
}
pub type STRING_TABLE_ENTRY_DESC = string_table_entry;
pub type STRING = *mut STRING_TABLE_ENTRY_DESC;
pub type DIRENTRY = STRING;
pub type TimeCount = DWORD;
pub type BATTLE_INPUT_STATE = UBYTE;
pub type BattleFrameCounter = DWORD;
pub type BattleFrameCallback = unsafe extern "C" fn() -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct battlestate_struct {
    pub InputFunc: Option<unsafe extern "C" fn(*mut battlestate_struct) -> BOOLEAN>,
    pub first_time: BOOLEAN,
    pub NextTime: DWORD,
    pub frame_cb: Option<BattleFrameCallback>,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Packet_Init {
    pub header: PacketHeader,
    pub protoVersion: C2Rust_Unnamed_1,
    pub padding0: uint16,
    pub uqmVersion: C2Rust_Unnamed_0,
    pub padding1: uint8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_0 {
    pub major: uint8,
    pub minor: uint8,
    pub patch: uint8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_1 {
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct melee_state {
    pub InputFunc: Option<unsafe extern "C" fn(*mut melee_state) -> BOOLEAN>,
    pub Initialized: BOOLEAN,
    pub meleeStarted: BOOLEAN,
    pub MeleeOption: MELEE_OPTIONS,
    pub side: COUNT,
    pub row: COUNT,
    pub col: COUNT,
    pub meleeSetup: *mut MeleeSetup,
    pub load: melee_load_state,
    pub currentShip: MeleeShip,
    pub CurIndex: COUNT,
    pub buildPickConfirmed: BOOLEAN,
    pub randomContext: *mut RandomContext,
    pub LastInputTime: TimeCount,
    pub hMusic: MUSIC_REF,
}
pub type MUSIC_REF = *mut *mut TFB_SoundSample;
pub type TFB_SoundSample = tfb_soundsample;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct melee_load_state {
    pub preBuiltList: *mut *mut MeleeTeam,
    pub preBuiltCount: COUNT,
    pub dirEntries: DIRENTRY,
    pub entryIndices: *mut COUNT,
    pub numIndices: COUNT,
    pub view: [*mut MeleeTeam; 5],
    pub top: COUNT,
    pub bot: COUNT,
    pub cur: COUNT,
    pub viewSize: COUNT,
}
pub type MELEE_OPTIONS = BYTE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BattleStateData {
    pub releaseFunction: NetConnectionStateData_ReleaseFunction,
    pub meleeState: *mut melee_state,
    pub battleState: *mut battlestate_struct,
    pub getMeleeState: *mut getmelee_struct,
    pub endFrameCount: BattleFrameCounter,
}
pub type MELEE_STATE = melee_state;
pub type GETMELEE_STATE = getmelee_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BattleInputBuffer {
    pub buf: *mut BATTLE_INPUT_STATE,
    pub maxSize: size_t,
    pub first: size_t,
    pub size: size_t,
}
#[inline]
unsafe extern "C" fn packetLength(mut packet: *const Packet) -> size_t {
    return ntoh16((*packet).header.len) as size_t;
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
unsafe extern "C" fn ntoh16(mut x: uint16) -> uint16 {
    return swapBytes16(x);
}
#[inline]
unsafe extern "C" fn ntoh32(mut x: uint32) -> uint32 {
    return swapBytes32(x);
}
unsafe extern "C" fn testNetState(mut condition: bool, mut r#type: PacketType) -> bool {
    if !condition {
        log_add(
            log_Level::log_Error,
            b"Packet of type '%s' received from wrong state.\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            (*(&raw mut packetTypeData as *mut PacketTypeData).offset(r#type.0 as isize))
                .name,
        );
        *__errno_location() = 9 as ::core::ffi::c_int;
    }
    return condition;
}
unsafe extern "C" fn versionCompare(
    mut major1: ::core::ffi::c_int,
    mut minor1: ::core::ffi::c_int,
    mut patch1: ::core::ffi::c_float,
    mut major2: ::core::ffi::c_int,
    mut minor2: ::core::ffi::c_int,
    mut patch2: ::core::ffi::c_float,
) -> ::core::ffi::c_int {
    if major1 < major2 {
        return -1 as ::core::ffi::c_int;
    }
    if major1 > major2 {
        return 1 as ::core::ffi::c_int;
    }
    if minor1 < minor2 {
        return -1 as ::core::ffi::c_int;
    }
    if minor1 > minor2 {
        return 1 as ::core::ffi::c_int;
    }
    if patch1 < patch2 {
        return -1 as ::core::ffi::c_int;
    }
    if patch1 > patch2 {
        return 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn PacketHandler_Init(
    mut conn: *mut NetConnection,
    mut packet: *const Packet_Init,
) -> ::core::ffi::c_int {
    if (*conn).stateFlags.reset.localReset() {
        return 0 as ::core::ffi::c_int;
    }
    if (*conn).stateFlags.reset.remoteReset() {
        *__errno_location() = 9 as ::core::ffi::c_int;
        return -1 as ::core::ffi::c_int;
    }
    if !testNetState(
        (*conn).state.0 == NetState::NetState_init.0
            && !(*conn).stateFlags.ready.remoteReady(),
        PacketType::PACKET_INIT,
    ) {
        return -1 as ::core::ffi::c_int;
    }
    if (*packet).protoVersion.major as ::core::ffi::c_int != 0 as ::core::ffi::c_int
        || (*packet).protoVersion.minor as ::core::ffi::c_int != 4 as ::core::ffi::c_int
    {
        sendAbort(conn, NetplayAbortReason::AbortReason_versionMismatch);
        uqm_lua_checked!(abortFeedback(conn, NetplayAbortReason::AbortReason_versionMismatch), 0);
        log_add(
            log_Level::log_Error,
            b"Protocol version %d.%d not supported.\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            (*packet).protoVersion.major as ::core::ffi::c_int,
            (*packet).protoVersion.minor as ::core::ffi::c_int,
        );
        *__errno_location() = 52 as ::core::ffi::c_int;
        return -1 as ::core::ffi::c_int;
    }
    if versionCompare(
        (*packet).uqmVersion.major as ::core::ffi::c_int,
        (*packet).uqmVersion.minor as ::core::ffi::c_int,
        (*packet).uqmVersion.patch as ::core::ffi::c_float,
        0 as ::core::ffi::c_int,
        8 as ::core::ffi::c_int,
        5 as ::core::ffi::c_int as ::core::ffi::c_float,
    ) < 0 as ::core::ffi::c_int
    {
        sendAbort(conn, NetplayAbortReason::AbortReason_versionMismatch);
        uqm_lua_checked!(abortFeedback(conn, NetplayAbortReason::AbortReason_versionMismatch), 0);
        log_add(
            log_Level::log_Error,
            b"Remote side is running a version of UQM that is too old (%d.%d.%d; %d.%d.%d is required).\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
            (*packet).uqmVersion.major as ::core::ffi::c_int,
            (*packet).uqmVersion.minor as ::core::ffi::c_int,
            (*packet).uqmVersion.patch as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            8 as ::core::ffi::c_int,
            5 as ::core::ffi::c_int,
        );
        *__errno_location() = 52 as ::core::ffi::c_int;
        return -1 as ::core::ffi::c_int;
    }
    uqm_lua_checked!(Netplay_remoteReady(conn), 0);
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn PacketHandler_Ping(
    mut conn: *mut NetConnection,
    mut packet: *const Packet_Ping,
) -> ::core::ffi::c_int {
    if !testNetState(
        (*conn).state.0 > NetState::NetState_init.0,
        PacketType::PACKET_PING,
    ) {
        return -1 as ::core::ffi::c_int;
    }
    sendAck(conn, (*packet).id);
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn PacketHandler_Ack(
    mut conn: *mut NetConnection,
    mut packet: *const Packet_Ack,
) -> ::core::ffi::c_int {
    if !testNetState(
        (*conn).state.0 > NetState::NetState_init.0,
        PacketType::PACKET_ACK,
    ) {
        return -1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn localSide(
    mut conn: *mut NetConnection,
    mut side: NetplaySide,
) -> ::core::ffi::c_int {
    if side.0 == NetplaySide::NetplaySide_local.0 {
        return (*conn).player;
    }
    return 1 as ::core::ffi::c_int - (*conn).player;
}
#[no_mangle]
pub unsafe extern "C" fn PacketHandler_Ready(
    mut conn: *mut NetConnection,
    mut packet: *const Packet_Ready,
) -> ::core::ffi::c_int {
    if (*conn).stateFlags.reset.localReset() {
        return 0 as ::core::ffi::c_int;
    }
    if (*conn).stateFlags.reset.remoteReset() {
        *__errno_location() = 9 as ::core::ffi::c_int;
        return -1 as ::core::ffi::c_int;
    }
    if !testNetState(
        readyFlagsMeaningful((*conn).state) as ::core::ffi::c_int != 0
            && !(*conn).stateFlags.ready.remoteReady(),
        PacketType::PACKET_READY,
    ) {
        return -1 as ::core::ffi::c_int;
    }
    uqm_lua_checked!(Netplay_remoteReady(conn), 0);
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn PacketHandler_Fleet(
    mut conn: *mut NetConnection,
    mut packet: *const Packet_Fleet,
) -> ::core::ffi::c_int {
    let mut numShips: uint16 = ntoh16((*packet).numShips);
    let mut i: size_t = 0;
    let mut len: size_t = 0;
    let mut player: ::core::ffi::c_int = 0;
    let mut battleStateData: *mut BattleStateData = ::core::ptr::null_mut::<
        BattleStateData,
    >();
    if (*conn).stateFlags.reset.localReset() {
        return 0 as ::core::ffi::c_int;
    }
    if (*conn).stateFlags.reset.remoteReset() {
        *__errno_location() = 9 as ::core::ffi::c_int;
        return -1 as ::core::ffi::c_int;
    }
    if !testNetState(
        (*conn).state.0 == NetState::NetState_inSetup.0,
        PacketType::PACKET_FLEET,
    ) {
        return -1 as ::core::ffi::c_int;
    }
    player = localSide(conn, NetplaySide((*packet).side as ::core::ffi::c_uint));
    len = packetLength(packet as *const Packet);
    if (*packet).side as ::core::ffi::c_int
        > NetplaySide::NetplaySide_remote.0 as ::core::ffi::c_int
        || numShips as ::core::ffi::c_int > 14 as ::core::ffi::c_int
        || ::core::mem::size_of::<Packet_Fleet>()
            .wrapping_add(
                (numShips as usize).wrapping_mul(::core::mem::size_of::<FleetEntry>()),
            ) > len
    {
        log_add(
            log_Level::log_Warning,
            b"Invalid fleet size. Specified size is %d, actual size = %d\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            numShips as ::core::ffi::c_int,
            len
                .wrapping_sub(::core::mem::size_of::<Packet_Fleet>())
                .wrapping_div(::core::mem::size_of::<FleetEntry>()) as ::core::ffi::c_int,
        );
        *__errno_location() = 9 as ::core::ffi::c_int;
        return -1 as ::core::ffi::c_int;
    }
    battleStateData = NetConnection_getStateData(conn) as *mut BattleStateData;
    if (*conn).stateFlags.handshake.localOk() {
        Netplay_cancelConfirmation(conn);
        uqm_lua_checked!(confirmationCancelled((*battleStateData).meleeState, (*conn).player as COUNT), 0);
    }
    i = 0 as size_t;
    while i < numShips as size_t {
        let mut ship: MeleeShip = MeleeShip(
            (*(&raw const (*packet).ships as *const FleetEntry).offset(i as isize)).ship
                as ::core::ffi::c_uint,
        );
        let mut index: FleetShipIndex = (*(&raw const (*packet).ships
            as *const FleetEntry)
            .offset(i as isize))
            .index as FleetShipIndex;
        if !MeleeShip_valid(ship) {
            log_add(
                log_Level::log_Warning,
                b"Invalid ship type number %d (max = %d).\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                ship.0,
                MeleeShip::MELEE_ZOQFOTPIK.0 as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int - 1 as ::core::ffi::c_int,
            );
            *__errno_location() = 9 as ::core::ffi::c_int;
            return -1 as ::core::ffi::c_int;
        }
        if index as ::core::ffi::c_int >= 14 as ::core::ffi::c_int {
            log_add(
                log_Level::log_Warning,
                b"Invalid ship position number %d (max = %d).\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                index as ::core::ffi::c_int,
                14 as ::core::ffi::c_int - 1 as ::core::ffi::c_int,
            );
            *__errno_location() = 9 as ::core::ffi::c_int;
            return -1 as ::core::ffi::c_int;
        }
        uqm_lua_checked!(Melee_RemoteChange_ship(
            (*battleStateData).meleeState,
            conn,
            player as COUNT,
            index,
            ship,
        ), 0);
        i = i.wrapping_add(1);
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn PacketHandler_TeamName(
    mut conn: *mut NetConnection,
    mut packet: *const Packet_TeamName,
) -> ::core::ffi::c_int {
    let mut nameLen: size_t = 0;
    let mut side: ::core::ffi::c_int = 0;
    let mut battleStateData: *mut BattleStateData = ::core::ptr::null_mut::<
        BattleStateData,
    >();
    if (*conn).stateFlags.reset.localReset() {
        return 0 as ::core::ffi::c_int;
    }
    if (*conn).stateFlags.reset.remoteReset() {
        *__errno_location() = 9 as ::core::ffi::c_int;
        return -1 as ::core::ffi::c_int;
    }
    if !testNetState(
        (*conn).state.0 == NetState::NetState_inSetup.0,
        PacketType::PACKET_FLEET,
    ) {
        return -1 as ::core::ffi::c_int;
    }
    if (*packet).side as ::core::ffi::c_int
        > NetplaySide::NetplaySide_remote.0 as ::core::ffi::c_int
        || packetLength(packet as *const Packet)
            <= ::core::mem::size_of::<Packet_TeamName>()
    {
        *__errno_location() = 9 as ::core::ffi::c_int;
        return -1 as ::core::ffi::c_int;
    }
    battleStateData = NetConnection_getStateData(conn) as *mut BattleStateData;
    if (*conn).stateFlags.handshake.localOk() {
        Netplay_cancelConfirmation(conn);
        uqm_lua_checked!(confirmationCancelled((*battleStateData).meleeState, (*conn).player as COUNT), 0);
    }
    side = localSide(conn, NetplaySide((*packet).side as ::core::ffi::c_uint));
    nameLen = packetLength(packet as *const Packet)
        .wrapping_sub(::core::mem::size_of::<Packet_TeamName>())
        .wrapping_sub(1 as size_t);
    let mut buf: [::core::ffi::c_char; 31] = [0; 31];
    if nameLen > 30 as size_t {
        nameLen = 30 as size_t;
    }
    memcpy(
        &raw mut buf as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        &raw const (*packet).name as *const uint8 as *const ::core::ffi::c_char
            as *const ::core::ffi::c_void,
        nameLen,
    );
    buf[nameLen] = '\0' as ::core::ffi::c_char;
    uqm_lua_checked!(Melee_RemoteChange_teamName(
        (*battleStateData).meleeState,
        conn,
        side as COUNT,
        &raw mut buf as *mut ::core::ffi::c_char,
    ), 0);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn handshakeComplete(mut conn: *mut NetConnection) {
    (!(*conn).stateFlags.handshake.localOk()
        || {
            __assert_fail(
                b"!conn->stateFlags.handshake.localOk\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/uqm/supermelee/netplay/packethandlers.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                296 as ::core::ffi::c_int,
                b"handshakeComplete\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    (!(*conn).stateFlags.handshake.remoteOk()
        || {
            __assert_fail(
                b"!conn->stateFlags.handshake.remoteOk\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/uqm/supermelee/netplay/packethandlers.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                297 as ::core::ffi::c_int,
                b"handshakeComplete\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    (!(*conn).stateFlags.handshake.canceling()
        || {
            __assert_fail(
                b"!conn->stateFlags.handshake.canceling\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/uqm/supermelee/netplay/packethandlers.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                298 as ::core::ffi::c_int,
                b"handshakeComplete\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    ((*conn).state.0 == NetState::NetState_inSetup.0
        || {
            __assert_fail(
                b"conn->state == NetState_inSetup\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/uqm/supermelee/netplay/packethandlers.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                300 as ::core::ffi::c_int,
                b"handshakeComplete\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    NetConnection_setState(conn, NetState::NetState_preBattle);
}
#[no_mangle]
pub unsafe extern "C" fn PacketHandler_Handshake0(
    mut conn: *mut NetConnection,
    mut packet: *const Packet_Handshake0,
) -> ::core::ffi::c_int {
    if (*conn).stateFlags.reset.localReset() {
        return 0 as ::core::ffi::c_int;
    }
    if (*conn).stateFlags.reset.remoteReset() {
        *__errno_location() = 9 as ::core::ffi::c_int;
        return -1 as ::core::ffi::c_int;
    }
    if !testNetState(
        handshakeMeaningful((*conn).state) as ::core::ffi::c_int != 0
            && !(*conn).stateFlags.handshake.remoteOk(),
        PacketType::PACKET_HANDSHAKE0,
    ) {
        return -1 as ::core::ffi::c_int;
    }
    (*conn).stateFlags.handshake.set_remoteOk(true as bool);
    if (*conn).stateFlags.handshake.localOk() as ::core::ffi::c_int != 0
        && !(*conn).stateFlags.handshake.canceling()
    {
        sendHandshake1(conn);
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn PacketHandler_Handshake1(
    mut conn: *mut NetConnection,
    mut packet: *const Packet_Handshake1,
) -> ::core::ffi::c_int {
    if (*conn).stateFlags.reset.localReset() {
        return 0 as ::core::ffi::c_int;
    }
    if (*conn).stateFlags.reset.remoteReset() {
        *__errno_location() = 9 as ::core::ffi::c_int;
        return -1 as ::core::ffi::c_int;
    }
    if !testNetState(
        handshakeMeaningful((*conn).state) as ::core::ffi::c_int != 0
            && ((*conn).stateFlags.handshake.localOk() as ::core::ffi::c_int != 0
                || (*conn).stateFlags.handshake.canceling() as ::core::ffi::c_int != 0),
        PacketType::PACKET_HANDSHAKE1,
    ) {
        return -1 as ::core::ffi::c_int;
    }
    if (*conn).stateFlags.handshake.canceling() {
        (*conn).stateFlags.handshake.set_remoteOk(true as bool);
    } else {
        let mut remoteWasOk: bool = (*conn).stateFlags.handshake.remoteOk();
        (*conn).stateFlags.handshake.set_localOk(false as bool);
        (*conn).stateFlags.handshake.set_remoteOk(false as bool);
        if !remoteWasOk {
            sendHandshake1(conn);
        }
        handshakeComplete(conn);
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn PacketHandler_HandshakeCancel(
    mut conn: *mut NetConnection,
    mut packet: *const Packet_HandshakeCancel,
) -> ::core::ffi::c_int {
    if (*conn).stateFlags.reset.localReset() {
        return 0 as ::core::ffi::c_int;
    }
    if (*conn).stateFlags.reset.remoteReset() {
        *__errno_location() = 9 as ::core::ffi::c_int;
        return -1 as ::core::ffi::c_int;
    }
    if !testNetState(
        handshakeMeaningful((*conn).state) as ::core::ffi::c_int != 0
            && (*conn).stateFlags.handshake.remoteOk() as ::core::ffi::c_int != 0,
        PacketType::PACKET_HANDSHAKECANCEL,
    ) {
        return -1 as ::core::ffi::c_int;
    }
    (*conn).stateFlags.handshake.set_remoteOk(false as bool);
    sendHandshakeCancelAck(conn);
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn PacketHandler_HandshakeCancelAck(
    mut conn: *mut NetConnection,
    mut packet: *const Packet_HandshakeCancelAck,
) -> ::core::ffi::c_int {
    if (*conn).stateFlags.reset.localReset() {
        return 0 as ::core::ffi::c_int;
    }
    if (*conn).stateFlags.reset.remoteReset() {
        *__errno_location() = 9 as ::core::ffi::c_int;
        return -1 as ::core::ffi::c_int;
    }
    if !testNetState(
        handshakeMeaningful((*conn).state) as ::core::ffi::c_int != 0
            && (*conn).stateFlags.handshake.canceling() as ::core::ffi::c_int != 0,
        PacketType::PACKET_HANDSHAKECANCELACK,
    ) {
        return -1 as ::core::ffi::c_int;
    }
    (*conn).stateFlags.handshake.set_canceling(false as bool);
    if (*conn).stateFlags.handshake.localOk() {
        if (*conn).stateFlags.handshake.remoteOk() {
            sendHandshake1(conn);
        } else {
            sendHandshake0(conn);
        }
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn PacketHandler_SeedRandom(
    mut conn: *mut NetConnection,
    mut packet: *const Packet_SeedRandom,
) -> ::core::ffi::c_int {
    let mut battleStateData: *mut BattleStateData = ::core::ptr::null_mut::<
        BattleStateData,
    >();
    if (*conn).stateFlags.reset.localReset() {
        return 0 as ::core::ffi::c_int;
    }
    if (*conn).stateFlags.reset.remoteReset() {
        *__errno_location() = 9 as ::core::ffi::c_int;
        return -1 as ::core::ffi::c_int;
    }
    if !testNetState(
        (*conn).state.0 == NetState::NetState_preBattle.0
            && !(*conn).stateFlags.discriminant,
        PacketType::PACKET_SEEDRANDOM,
    ) {
        return -1 as ::core::ffi::c_int;
    }
    battleStateData = NetConnection_getStateData(conn) as *mut BattleStateData;
    updateRandomSeed(
        (*battleStateData).meleeState,
        (*conn).player as COUNT,
        ntoh32((*packet).seed),
    );
    (*conn).stateFlags.agreement.set_randomSeed(true as bool);
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn PacketHandler_InputDelay(
    mut conn: *mut NetConnection,
    mut packet: *const Packet_InputDelay,
) -> ::core::ffi::c_int {
    let mut delay: uint32 = 0;
    if (*conn).stateFlags.reset.localReset() {
        return 0 as ::core::ffi::c_int;
    }
    if (*conn).stateFlags.reset.remoteReset() {
        *__errno_location() = 9 as ::core::ffi::c_int;
        return -1 as ::core::ffi::c_int;
    }
    if !testNetState(
        (*conn).state.0 == NetState::NetState_preBattle.0,
        PacketType::PACKET_INPUTDELAY,
    ) {
        return -1 as ::core::ffi::c_int;
    }
    delay = ntoh32((*packet).delay);
    if delay > (840 as ::core::ffi::c_int / 24 as ::core::ffi::c_int) as uint32 {
        log_add(
            log_Level::log_Error,
            b"NETPLAY: [%d]     Received absurdly large input delay value (%d).\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
            (*conn).player,
            delay,
        );
        return -1 as ::core::ffi::c_int;
    }
    (*conn).stateFlags.inputDelay = delay as size_t;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn PacketHandler_SelectShip(
    mut conn: *mut NetConnection,
    mut packet: *const Packet_SelectShip,
) -> ::core::ffi::c_int {
    let mut updateResult: bool = false;
    let mut battleStateData: *mut BattleStateData = ::core::ptr::null_mut::<
        BattleStateData,
    >();
    if (*conn).stateFlags.reset.localReset() {
        return 0 as ::core::ffi::c_int;
    }
    if (*conn).stateFlags.reset.remoteReset() {
        *__errno_location() = 9 as ::core::ffi::c_int;
        return -1 as ::core::ffi::c_int;
    }
    if !testNetState(
        (*conn).state.0 == NetState::NetState_selectShip.0,
        PacketType::PACKET_SELECTSHIP,
    ) {
        return -1 as ::core::ffi::c_int;
    }
    battleStateData = NetConnection_getStateData(conn) as *mut BattleStateData;
    updateResult = uqm_lua_checked!(updateMeleeSelection(
        (*battleStateData).getMeleeState,
        (*conn).player as COUNT,
        ntoh16((*packet).ship),
    ), 0);
    if !updateResult {
        *__errno_location() = 9 as ::core::ffi::c_int;
        return -1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn PacketHandler_BattleInput(
    mut conn: *mut NetConnection,
    mut packet: *const Packet_BattleInput,
) -> ::core::ffi::c_int {
    let mut input: BATTLE_INPUT_STATE = 0;
    let mut bib: *mut BattleInputBuffer = ::core::ptr::null_mut::<BattleInputBuffer>();
    if (*conn).stateFlags.reset.localReset() {
        return 0 as ::core::ffi::c_int;
    }
    if (*conn).stateFlags.reset.remoteReset() {
        *__errno_location() = 9 as ::core::ffi::c_int;
        return -1 as ::core::ffi::c_int;
    }
    if !testNetState(
        (*conn).state.0 == NetState::NetState_inBattle.0
            || (*conn).state.0 == NetState::NetState_endingBattle.0
            || (*conn).state.0 == NetState::NetState_endingBattle2.0,
        PacketType::PACKET_BATTLEINPUT,
    ) {
        return -1 as ::core::ffi::c_int;
    }
    input = (*packet).state;
    bib = getBattleInputBuffer((*conn).player as size_t);
    if !BattleInputBuffer_push(bib, input) {
        return -1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn PacketHandler_FrameCount(
    mut conn: *mut NetConnection,
    mut packet: *const Packet_FrameCount,
) -> ::core::ffi::c_int {
    let mut battleStateData: *mut BattleStateData = ::core::ptr::null_mut::<
        BattleStateData,
    >();
    let mut frameCount: BattleFrameCounter = 0;
    if (*conn).stateFlags.reset.localReset() {
        return 0 as ::core::ffi::c_int;
    }
    if (*conn).stateFlags.reset.remoteReset() {
        *__errno_location() = 9 as ::core::ffi::c_int;
        return -1 as ::core::ffi::c_int;
    }
    if !testNetState(
        (*conn).state.0 == NetState::NetState_endingBattle.0,
        PacketType::PACKET_FRAMECOUNT,
    ) {
        return -1 as ::core::ffi::c_int;
    }
    frameCount = ntoh32((*packet).frameCount);
    battleStateData = NetConnection_getStateData(conn) as *mut BattleStateData;
    if frameCount > (*battleStateData).endFrameCount {
        (*battleStateData).endFrameCount = frameCount;
    }
    uqm_lua_checked!(Netplay_remoteReady(conn), 0);
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn PacketHandler_Checksum(
    mut conn: *mut NetConnection,
    mut packet: *const Packet_Checksum,
) -> ::core::ffi::c_int {
    let mut frameNr: uint32 = 0;
    let mut checksum: uint32 = 0;
    let mut delay: size_t = 0;
    let mut interval: size_t = 0;
    if (*conn).stateFlags.reset.localReset() {
        return 0 as ::core::ffi::c_int;
    }
    if (*conn).stateFlags.reset.remoteReset() {
        *__errno_location() = 9 as ::core::ffi::c_int;
        return -1 as ::core::ffi::c_int;
    }
    if !testNetState(NetState_battleActive((*conn).state), PacketType::PACKET_CHECKSUM) {
        return -1 as ::core::ffi::c_int;
    }
    frameNr = ntoh32((*packet).frameNr);
    checksum = ntoh32((*packet).checksum);
    interval = NetConnection_getChecksumInterval(conn);
    delay = getBattleInputDelay();
    if (frameNr as size_t).wrapping_rem(interval) != 0 as size_t {
        log_add(
            log_Level::log_Warning,
            b"NETPLAY: [%d] <== Received checksum for frame %u, while we only expect checksums on frames divisable by %u -- discarding.\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
            (*conn).player,
            frameNr,
            interval as ::core::ffi::c_uint,
        );
        return 0 as ::core::ffi::c_int;
    }
    if frameNr as size_t
        > (battleFrameCount as size_t).wrapping_add(delay).wrapping_add(1 as size_t)
    {
        log_add(
            log_Level::log_Warning,
            b"NETPLAY: [%d] <== Received checksum for a frame too far in the future (frame %u, current is %u, input delay is %u) -- discarding.\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
            (*conn).player,
            frameNr,
            battleFrameCount,
            delay as ::core::ffi::c_uint,
        );
        return 0 as ::core::ffi::c_int;
    }
    if (frameNr as size_t).wrapping_add(delay) < battleFrameCount as size_t {
        log_add(
            log_Level::log_Warning,
            b"NETPLAY: [%d] <== Received checksum for a frame too far in the past (frame %u, current is %u, input delay is %u) -- discarding.\0"
                .as_ptr() as *const ::core::ffi::c_char,
            (*conn).player,
            frameNr,
            battleFrameCount,
            delay as ::core::ffi::c_uint,
        );
        return 0 as ::core::ffi::c_int;
    }
    addRemoteChecksum(conn, frameNr, checksum);
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn PacketHandler_Abort(
    mut conn: *mut NetConnection,
    mut packet: *const Packet_Abort,
) -> ::core::ffi::c_int {
    uqm_lua_checked!(abortFeedback(conn, NetplayAbortReason((*packet).reason as ::core::ffi::c_uint)), 0);
    return -1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn PacketHandler_Reset(
    mut conn: *mut NetConnection,
    mut packet: *const Packet_Reset,
) -> ::core::ffi::c_int {
    let mut reason: NetplayResetReason = NetplayResetReason::ResetReason_unspecified;
    if !testNetState(!(*conn).stateFlags.reset.remoteReset(), PacketType::PACKET_RESET) {
        return -1 as ::core::ffi::c_int;
    }
    reason = NetplayResetReason(ntoh16((*packet).reason) as ::core::ffi::c_uint);
    uqm_lua_checked!(Netplay_remoteReset(conn, reason), 0);
    return 0 as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn handshakeMeaningful(mut state: NetState) -> bool {
    return state.0 == NetState::NetState_inSetup.0;
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
#[inline]
unsafe extern "C" fn MeleeShip_valid(mut ship: MeleeShip) -> bool {
    return ship.0
        < (MeleeShip::MELEE_ZOQFOTPIK.0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
            as ::core::ffi::c_uint || ship.0 == MeleeShip::MELEE_NONE.0;
}
#[inline]
unsafe extern "C" fn NetState_battleActive(mut state: NetState) -> bool {
    return state.0 == NetState::NetState_inBattle.0
        || state.0 == NetState::NetState_endingBattle.0
        || state.0 == NetState::NetState_endingBattle2.0;
}
