/*
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
 *  Foundation, Inc., 59 Temple Place - Suite 330, Boston, MA 02111-1307, USA.
 */

// Rust translation of src/uqm/supermelee/netplay/notifyall.c.
extern "C" {
    pub type string_table_entry;
    pub type tfb_soundsample;
    pub type MeleeTeam;
    pub type MeleeSetup;
    pub type RandomContext;
    pub type NetConnection;
    fn NetConnection_isConnected(conn: *const NetConnection) -> bool;
    fn NetConnection_getState(conn: *const NetConnection) -> NetState;
    fn strlen(_: *const ::core::ffi::c_char) -> size_t;
    static mut netConnections: [*mut NetConnection; 0];
    fn forEachConnectedPlayer(
        callback: ForEachConnectionCallback,
        arg: *mut ::core::ffi::c_void,
    ) -> bool;
    fn Netplay_Notify_battleInput(conn: *mut NetConnection, input: BATTLE_INPUT_STATE);
    fn Netplay_Notify_setTeamName(
        conn: *mut NetConnection,
        player: ::core::ffi::c_int,
        name: *const ::core::ffi::c_char,
        len: size_t,
    );
    fn Netplay_Notify_setFleet(
        conn: *mut NetConnection,
        player: ::core::ffi::c_int,
        fleet: *const MeleeShip,
        fleetSize: size_t,
    );
    fn Netplay_Notify_setShip(
        conn: *mut NetConnection,
        player: ::core::ffi::c_int,
        index: FleetShipIndex,
        ship: MeleeShip,
    );
    fn Netplay_Notify_inputDelay(conn: *mut NetConnection, delay: uint32);
    fn Netplay_Notify_checksum(
        conn: *mut NetConnection,
        frameCount: BattleFrameCounter,
        checksum: Checksum,
    );
    fn MeleeSetup_getShip(
        setup: *const MeleeSetup,
        teamNr: size_t,
        slotNr: FleetShipIndex,
    ) -> MeleeShip;
    fn MeleeSetup_getFleet(setup: *const MeleeSetup, teamNr: size_t) -> *const MeleeShip;
    fn MeleeSetup_getTeamName(
        setup: *const MeleeSetup,
        teamNr: size_t,
    ) -> *const ::core::ffi::c_char;
}
pub type size_t = usize;
pub type uint8 = ::core::ffi::c_uchar;
pub type uint16 = ::core::ffi::c_ushort;
pub type uint32 = ::core::ffi::c_uint;
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
pub type BattleFrameCounter = DWORD;
pub type STRING_TABLE_ENTRY_DESC = string_table_entry;
pub type STRING = *mut STRING_TABLE_ENTRY_DESC;
pub type DIRENTRY = STRING;
pub type MUSIC_REF = *mut *mut TFB_SoundSample;
pub type TFB_SoundSample = tfb_soundsample;
pub type BATTLE_INPUT_STATE = UBYTE;
pub type TimeCount = DWORD;
pub type FleetShipIndex = COUNT;
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
pub type Checksum = uint32;
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
pub type MELEE_OPTIONS = BYTE;
pub type MELEE_STATE = melee_state;
pub type ForEachConnectionCallback = Option<
    unsafe extern "C" fn(*mut NetConnection, *mut ::core::ffi::c_void) -> bool,
>;
#[no_mangle]
pub unsafe extern "C" fn Netplay_NotifyAll_setTeamName(
    mut pMS: *mut MELEE_STATE,
    mut playerNr: size_t,
) {
    let mut name: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    let mut len: size_t = 0;
    let mut playerI: size_t = 0;
    name = MeleeSetup_getTeamName((*pMS).meleeSetup, playerNr);
    len = strlen(name);
    playerI = 0 as size_t;
    while playerI < 2 as size_t {
        let mut conn: *mut NetConnection = *(&raw mut netConnections
            as *mut *mut NetConnection)
            .offset(playerI as isize);
        if !conn.is_null() {
            if NetConnection_isConnected(conn) {
                if NetConnection_getState(conn).0 == NetState::NetState_inSetup.0 {
                    Netplay_Notify_setTeamName(
                        conn,
                        playerNr as ::core::ffi::c_int,
                        name,
                        len,
                    );
                }
            }
        }
        playerI = playerI.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn Netplay_NotifyAll_setFleet(
    mut pMS: *mut MELEE_STATE,
    mut playerNr: size_t,
) {
    let mut setup: *mut MeleeSetup = (*pMS).meleeSetup;
    let mut ships: *const MeleeShip = MeleeSetup_getFleet(setup, playerNr);
    let mut playerI: size_t = 0;
    playerI = 0 as size_t;
    while playerI < 2 as size_t {
        let mut conn: *mut NetConnection = *(&raw mut netConnections
            as *mut *mut NetConnection)
            .offset(playerI as isize);
        if !conn.is_null() {
            if NetConnection_isConnected(conn) {
                if NetConnection_getState(conn).0 == NetState::NetState_inSetup.0 {
                    Netplay_Notify_setFleet(
                        conn,
                        playerNr as ::core::ffi::c_int,
                        ships,
                        (2 as ::core::ffi::c_int * 7 as ::core::ffi::c_int) as size_t,
                    );
                }
            }
        }
        playerI = playerI.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn Netplay_NotifyAll_setShip(
    mut pMS: *mut MELEE_STATE,
    mut playerNr: size_t,
    mut index: size_t,
) {
    let mut setup: *mut MeleeSetup = (*pMS).meleeSetup;
    let mut ship: MeleeShip = MeleeSetup_getShip(
        setup,
        playerNr,
        index as FleetShipIndex,
    );
    let mut playerI: size_t = 0;
    playerI = 0 as size_t;
    while playerI < 2 as size_t {
        let mut conn: *mut NetConnection = *(&raw mut netConnections
            as *mut *mut NetConnection)
            .offset(playerI as isize);
        if !conn.is_null() {
            if NetConnection_isConnected(conn) {
                if NetConnection_getState(conn).0 == NetState::NetState_inSetup.0 {
                    Netplay_Notify_setShip(
                        conn,
                        playerNr as ::core::ffi::c_int,
                        index as FleetShipIndex,
                        ship,
                    );
                }
            }
        }
        playerI = playerI.wrapping_add(1);
    }
}
unsafe extern "C" fn Netplay_NotifyAll_inputDelayCallback(
    mut conn: *mut NetConnection,
    mut arg: *mut ::core::ffi::c_void,
) -> bool {
    let mut delay: *const size_t = arg as *mut size_t;
    Netplay_Notify_inputDelay(conn, *delay as uint32);
    return true;
}
#[no_mangle]
pub unsafe extern "C" fn Netplay_NotifyAll_inputDelay(mut delay: size_t) -> bool {
    return forEachConnectedPlayer(
        Some(
            Netplay_NotifyAll_inputDelayCallback
                as unsafe extern "C" fn(
                    *mut NetConnection,
                    *mut ::core::ffi::c_void,
                ) -> bool,
        ),
        &raw mut delay as *mut ::core::ffi::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn Netplay_NotifyAll_checksum(
    mut frameNr: BattleFrameCounter,
    mut checksum: Checksum,
) {
    let mut player: COUNT = 0;
    player = 0 as COUNT;
    while (player as ::core::ffi::c_int) < 2 as ::core::ffi::c_int {
        let mut conn: *mut NetConnection = *(&raw mut netConnections
            as *mut *mut NetConnection)
            .offset(player as isize);
        if !conn.is_null() {
            if NetConnection_isConnected(conn) {
                Netplay_Notify_checksum(conn, frameNr, checksum);
            }
        }
        player = player.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn Netplay_NotifyAll_battleInput(mut input: BATTLE_INPUT_STATE) {
    let mut player: COUNT = 0;
    player = 0 as COUNT;
    while (player as ::core::ffi::c_int) < 2 as ::core::ffi::c_int {
        let mut conn: *mut NetConnection = *(&raw mut netConnections
            as *mut *mut NetConnection)
            .offset(player as isize);
        if !conn.is_null() {
            if NetConnection_isConnected(conn) {
                Netplay_Notify_battleInput(conn, input);
            }
        }
        player = player.wrapping_add(1);
    }
}
