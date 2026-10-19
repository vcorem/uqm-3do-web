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

// Rust translation of src/uqm/supermelee/netplay/netmisc.c.
extern "C" {
    pub type getmelee_struct;
    pub type tfb_soundsample;
    pub type RandomContext;
    pub type MeleeTeam;
    pub type string_table_entry;
    pub type MeleeSetup;
    pub type NetConnection;
    pub type ConnectError;
    pub type ListenError;
    fn __assert_fail(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    ) -> !;
    fn malloc(_: size_t) -> *mut ::core::ffi::c_void;
    fn free(_: *mut ::core::ffi::c_void);
    fn NetConnection_setStateData(
        conn: *mut NetConnection,
        stateData: *mut NetConnectionStateData,
    );
    fn NetConnection_getStateData(
        conn: *const NetConnection,
    ) -> *mut NetConnectionStateData;
    fn NetConnection_setExtra(conn: *mut NetConnection, extra: *mut ::core::ffi::c_void);
    fn NetConnection_getExtra(conn: *const NetConnection) -> *mut ::core::ffi::c_void;
    fn NetConnection_setState(conn: *mut NetConnection, state: NetState);
    fn NetConnection_getPlayerNr(conn: *const NetConnection) -> ::core::ffi::c_int;
    fn sendInit(conn: *mut NetConnection);
    fn flushPacketQueues();
    fn Netplay_localReady(
        conn: *mut NetConnection,
        callback: NetConnection_ReadyCallback,
        arg: *mut ::core::ffi::c_void,
        notifyRemote: bool,
    ) -> bool;
    fn connectedFeedback(conn: *mut NetConnection);
    fn errorFeedback(conn: *mut NetConnection);
    fn closeFeedback(conn: *mut NetConnection);
    fn Melee_bootstrapSyncTeam(pMS: *mut MELEE_STATE, teamNr: size_t);
    fn MeleeSetup_resetSentTeams(setup: *mut MeleeSetup);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BattleStateData {
    pub releaseFunction: NetConnectionStateData_ReleaseFunction,
    pub meleeState: *mut melee_state,
    pub battleState: *mut battlestate_struct,
    pub getMeleeState: *mut getmelee_struct,
    pub endFrameCount: BattleFrameCounter,
}
pub type BattleFrameCounter = DWORD;
pub type DWORD = uint32;
pub type uint32 = ::core::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct battlestate_struct {
    pub InputFunc: Option<unsafe extern "C" fn(*mut battlestate_struct) -> BOOLEAN>,
    pub first_time: BOOLEAN,
    pub NextTime: DWORD,
    pub frame_cb: Option<BattleFrameCallback>,
}
pub type BattleFrameCallback = unsafe extern "C" fn() -> ();
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct BOOLEAN(pub ::core::ffi::c_uint);
impl BOOLEAN {
    pub const FALSE: Self = Self(0);
    pub const TRUE: Self = Self(1);
}
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
pub type TimeCount = DWORD;
pub type COUNT = UWORD;
pub type UWORD = uint16;
pub type uint16 = ::core::ffi::c_ushort;
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
pub type DIRENTRY = STRING;
pub type STRING = *mut STRING_TABLE_ENTRY_DESC;
pub type STRING_TABLE_ENTRY_DESC = string_table_entry;
pub type MELEE_OPTIONS = BYTE;
pub type BYTE = uint8;
pub type uint8 = ::core::ffi::c_uchar;
pub type NetConnectionStateData_ReleaseFunction = Option<
    unsafe extern "C" fn(*mut NetConnectionStateData) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NetConnectionStateData {
    pub releaseFunction: NetConnectionStateData_ReleaseFunction,
}
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
pub type size_t = usize;
pub type NetConnection_ReadyCallback = Option<
    unsafe extern "C" fn(*mut NetConnection, *mut ::core::ffi::c_void) -> (),
>;
pub type MELEE_STATE = melee_state;
unsafe extern "C" fn BattleStateData_alloc() -> *mut BattleStateData {
    return malloc(::core::mem::size_of::<BattleStateData>()) as *mut BattleStateData;
}
unsafe extern "C" fn BattleStateData_free(mut battleStateData: *mut BattleStateData) {
    free(battleStateData as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn BattleStateData_new(
    mut meleeState: *mut melee_state,
    mut battleState: *mut battlestate_struct,
    mut getMeleeState: *mut getmelee_struct,
) -> *mut BattleStateData {
    let mut battleStateData: *mut BattleStateData = BattleStateData_alloc();
    (*battleStateData).releaseFunction = ::core::mem::transmute::<
        Option<unsafe extern "C" fn(*mut BattleStateData) -> ()>,
        NetConnectionStateData_ReleaseFunction,
    >(Some(BattleStateData_delete as unsafe extern "C" fn(*mut BattleStateData) -> ()));
    (*battleStateData).meleeState = meleeState as *mut melee_state;
    (*battleStateData).battleState = battleState;
    (*battleStateData).getMeleeState = getMeleeState as *mut getmelee_struct;
    return battleStateData;
}
unsafe extern "C" fn BattleStateData_delete(mut battleStateData: *mut BattleStateData) {
    BattleStateData_free(battleStateData);
}
#[no_mangle]
pub unsafe extern "C" fn NetMelee_connectCallback(mut conn: *mut NetConnection) {
    let mut battleStateData: *mut BattleStateData = ::core::ptr::null_mut::<
        BattleStateData,
    >();
    let mut meleeState: *mut melee_state = ::core::ptr::null_mut::<melee_state>();
    meleeState = NetConnection_getExtra(conn) as *mut melee_state;
    battleStateData = BattleStateData_new(
        meleeState,
        ::core::ptr::null_mut::<battlestate_struct>(),
        ::core::ptr::null_mut::<getmelee_struct>(),
    );
    NetConnection_setStateData(
        conn,
        battleStateData as *mut ::core::ffi::c_void as *mut NetConnectionStateData,
    );
    NetConnection_setExtra(conn, ::core::ptr::null_mut::<::core::ffi::c_void>());
    MeleeSetup_resetSentTeams((*meleeState).meleeSetup);
    sendInit(conn);
    uqm_lua_checked!(Netplay_localReady(
        conn,
        Some(
            NetMelee_enterState_inSetup
                as unsafe extern "C" fn(
                    *mut NetConnection,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
        false,
    ), ());
}
#[no_mangle]
pub unsafe extern "C" fn NetMelee_closeCallback(mut conn: *mut NetConnection) {
    uqm_lua_checked!(closeFeedback(conn), ());
}
#[no_mangle]
pub unsafe extern "C" fn NetMelee_errorCallback(
    mut conn: *mut NetConnection,
    mut error: *const NetConnectionError,
) {
    uqm_lua_checked!(errorFeedback(conn), ());
}
unsafe extern "C" fn NetMelee_enterState_inSetup(
    mut conn: *mut NetConnection,
    mut arg: *mut ::core::ffi::c_void,
) {
    let mut battleStateData: *mut BattleStateData = ::core::ptr::null_mut::<
        BattleStateData,
    >();
    let mut meleeState: *mut melee_state = ::core::ptr::null_mut::<melee_state>();
    let mut player: ::core::ffi::c_int = 0;
    NetConnection_setState(conn, NetState::NetState_inSetup);
    battleStateData = NetConnection_getStateData(conn) as *mut BattleStateData;
    meleeState = (*battleStateData).meleeState as *mut melee_state;
    player = NetConnection_getPlayerNr(conn);
    uqm_lua_checked!(connectedFeedback(conn), ());
    (2 as ::core::ffi::c_int == 2 as ::core::ffi::c_int
        || {
            __assert_fail(
                b"NUM_PLAYERS == 2\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/uqm/supermelee/netplay/netmisc.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                127 as ::core::ffi::c_int,
                b"NetMelee_enterState_inSetup\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    Melee_bootstrapSyncTeam(meleeState, player as size_t);
    uqm_lua_checked!(flushPacketQueues(), ());
}
