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

// Rust translation of src/uqm/supermelee/meleesetup.c.
extern "C" {
    pub type uio_Stream;
    fn uio_fread(
        buf: *mut ::core::ffi::c_void,
        size: size_t,
        nmemb: size_t,
        stream: *mut uio_Stream,
    ) -> size_t;
    fn uio_fgetc(stream: *mut uio_Stream) -> ::core::ffi::c_int;
    fn uio_fputc(c: ::core::ffi::c_int, stream: *mut uio_Stream) -> ::core::ffi::c_int;
    fn uio_fwrite(
        buf: *const ::core::ffi::c_void,
        size: size_t,
        nmemb: size_t,
        stream: *mut uio_Stream,
    ) -> size_t;
    fn HMalloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn HFree(p: *mut ::core::ffi::c_void);
    fn strncpy(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: size_t,
    ) -> *mut ::core::ffi::c_char;
    fn strcmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn log_add(_: log_Level, fmt: *const ::core::ffi::c_char, ...);
    fn GetShipCostFromIndex(Index: ::core::ffi::c_uint) -> COUNT;
    fn GetShipValue(StarShip: MeleeShip) -> COUNT;
}
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MeleeTeam {
    pub ships: [MeleeShip; 14],
    pub name: [::core::ffi::c_char; 55],
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
pub struct MeleeSetup {
    pub teams: [MeleeTeam; 2],
    pub fleetValue: [COUNT; 2],
    pub sentTeams: [MeleeTeam; 2],
    pub haveSentTeamName: [bool; 2],
}
pub type COUNT = UWORD;
pub type UWORD = uint16;
pub type uint16 = ::core::ffi::c_ushort;
pub type FleetShipIndex = COUNT;
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
pub static mut MeleeTeam_serialSize: size_t = 0;
#[no_mangle]
pub unsafe extern "C" fn MeleeTeam_init(mut team: *mut MeleeTeam) {
    let mut slotI: FleetShipIndex = 0;
    slotI = 0 as FleetShipIndex;
    while (slotI as ::core::ffi::c_int) < 14 as ::core::ffi::c_int {
        (*team).ships[slotI as usize] = MeleeShip::MELEE_NONE;
        slotI = slotI.wrapping_add(1);
    }
    (*team).name[0usize] = '\0' as ::core::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn MeleeTeam_uninit(mut team: *mut MeleeTeam) {}
#[no_mangle]
pub unsafe extern "C" fn MeleeTeam_new() -> *mut MeleeTeam {
    let mut result: *mut MeleeTeam = HMalloc(::core::mem::size_of::<MeleeTeam>())
        as *mut MeleeTeam;
    MeleeTeam_init(result);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn MeleeTeam_delete(mut team: *mut MeleeTeam) {
    MeleeTeam_uninit(team);
    HFree(team as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn MeleeTeam_serialize(
    mut team: *const MeleeTeam,
    mut stream: *mut uio_Stream,
) -> ::core::ffi::c_int {
    let mut slotI: FleetShipIndex = 0;
    slotI = 0 as FleetShipIndex;
    while (slotI as ::core::ffi::c_int) < 14 as ::core::ffi::c_int {
        if uqm_lua_checked!(uio_fputc((*team).ships[slotI as usize].0 as ::core::ffi::c_int, stream), 0)
            == -1 as ::core::ffi::c_int
        {
            return -1 as ::core::ffi::c_int;
        }
        slotI = slotI.wrapping_add(1);
    }
    if uqm_lua_checked!(uio_fwrite(
        &raw const (*team).name as *const ::core::ffi::c_char
            as *const ::core::ffi::c_void,
        ::core::mem::size_of::<[::core::ffi::c_char; 55]>(),
        1 as size_t,
        stream,
    ), 0) != 1 as size_t
    {
        return -1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn MeleeTeam_deserialize(
    mut team: *mut MeleeTeam,
    mut stream: *mut uio_Stream,
) -> ::core::ffi::c_int {
    let mut slotI: FleetShipIndex = 0;
    slotI = 0 as FleetShipIndex;
    '_err: {
        while (slotI as ::core::ffi::c_int) < 14 as ::core::ffi::c_int {
            let mut ship: ::core::ffi::c_int = uqm_lua_checked!(uio_fgetc(stream), 0);
            if ship == -1 as ::core::ffi::c_int {
                break '_err;
            }
            (*team).ships[slotI as usize] = MeleeShip(ship as ::core::ffi::c_uint);
            if (*team).ships[slotI as usize].0 != MeleeShip::MELEE_NONE.0 {
                if (*team).ships[slotI as usize].0
                    >= (MeleeShip::MELEE_ZOQFOTPIK.0 as ::core::ffi::c_int
                        + 1 as ::core::ffi::c_int) as ::core::ffi::c_uint
                {
                    log_add(
                        log_Level::log_Warning,
                        b"Invalid ship type in loaded team (index %d, ship type is %d, max valid is %d).\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        slotI as ::core::ffi::c_int,
                        (*team).ships[slotI as usize].0,
                        MeleeShip::MELEE_ZOQFOTPIK.0 as ::core::ffi::c_int
                            + 1 as ::core::ffi::c_int - 1 as ::core::ffi::c_int,
                    );
                    (*team).ships[slotI as usize] = MeleeShip::MELEE_NONE;
                }
            }
            slotI = slotI.wrapping_add(1);
        }
        if uqm_lua_checked!(uio_fread(
            &raw mut (*team).name as *mut ::core::ffi::c_char
                as *mut ::core::ffi::c_void,
            ::core::mem::size_of::<[::core::ffi::c_char; 55]>(),
            1 as size_t,
            stream,
        ), 0) == 1 as size_t
        {
            (*team).name[30usize] = '\0' as ::core::ffi::c_char;
            return 0 as ::core::ffi::c_int;
        }
    }
    MeleeTeam_delete(team);
    return -1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn MeleeTeam_getValue(mut team: *const MeleeTeam) -> COUNT {
    let mut total: COUNT = 0 as COUNT;
    let mut slotI: FleetShipIndex = 0;
    slotI = 0 as FleetShipIndex;
    while (slotI as ::core::ffi::c_int) < 14 as ::core::ffi::c_int {
        let mut ship: MeleeShip = (*team).ships[slotI as usize];
        let mut shipValue: COUNT = GetShipValue(ship);
        if shipValue as ::core::ffi::c_int
            != !(0 as ::core::ffi::c_int) as COUNT as ::core::ffi::c_int
        {
            total = (total as ::core::ffi::c_int + shipValue as ::core::ffi::c_int)
                as COUNT;
        }
        slotI = slotI.wrapping_add(1);
    }
    return total;
}
#[no_mangle]
pub unsafe extern "C" fn MeleeTeam_getShip(
    mut team: *const MeleeTeam,
    mut slotNr: FleetShipIndex,
) -> MeleeShip {
    return (*team).ships[slotNr as usize];
}
#[no_mangle]
pub unsafe extern "C" fn MeleeTeam_setShip(
    mut team: *mut MeleeTeam,
    mut slotNr: FleetShipIndex,
    mut ship: MeleeShip,
) {
    (*team).ships[slotNr as usize] = ship;
}
#[no_mangle]
pub unsafe extern "C" fn MeleeTeam_getFleet(
    mut team: *const MeleeTeam,
) -> *const MeleeShip {
    return &raw const (*team).ships as *const MeleeShip;
}
#[no_mangle]
pub unsafe extern "C" fn MeleeTeam_getTeamName(
    mut team: *const MeleeTeam,
) -> *const ::core::ffi::c_char {
    return &raw const (*team).name as *const ::core::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn MeleeTeam_setName(
    mut team: *mut MeleeTeam,
    mut name: *const ::core::ffi::c_char,
) {
    strncpy(
        &raw mut (*team).name as *mut ::core::ffi::c_char,
        name,
        ::core::mem::size_of::<[::core::ffi::c_char; 55]>().wrapping_sub(1 as size_t),
    );
    (*team)
        .name[::core::mem::size_of::<[::core::ffi::c_char; 55]>()
        .wrapping_sub(1usize)] = '\0' as ::core::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn MeleeTeam_copy(
    mut copy: *mut MeleeTeam,
    mut original: *const MeleeTeam,
) {
    *copy = *original;
}
unsafe extern "C" fn MeleeSetup_initSentTeam(
    mut setup: *mut MeleeSetup,
    mut teamNr: size_t,
) {
    let mut team: *mut MeleeTeam = (&raw mut (*setup).sentTeams as *mut MeleeTeam)
        .offset(teamNr as isize);
    let mut slotI: FleetShipIndex = 0;
    slotI = 0 as FleetShipIndex;
    while (slotI as ::core::ffi::c_int) < 14 as ::core::ffi::c_int {
        MeleeTeam_setShip(team, slotI, MeleeShip::MELEE_UNSET);
        slotI = slotI.wrapping_add(1);
    }
    (*setup).haveSentTeamName[teamNr] = false;
    MeleeTeam_setName(team, b"<INVALID>\0".as_ptr() as *const ::core::ffi::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn MeleeSetup_new() -> *mut MeleeSetup {
    let mut teamI: size_t = 0;
    let mut result: *mut MeleeSetup = HMalloc(::core::mem::size_of::<MeleeSetup>())
        as *mut MeleeSetup;
    if result.is_null() {
        return ::core::ptr::null_mut::<MeleeSetup>();
    }
    teamI = 0 as size_t;
    while teamI < 2 as size_t {
        MeleeTeam_init(
            (&raw mut (*result).teams as *mut MeleeTeam).offset(teamI as isize),
        );
        (*result).fleetValue[teamI] = 0 as COUNT;
        MeleeSetup_initSentTeam(result, teamI);
        teamI = teamI.wrapping_add(1);
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn MeleeSetup_delete(mut setup: *mut MeleeSetup) {
    HFree(setup as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn MeleeSetup_resetSentTeams(mut setup: *mut MeleeSetup) {
    let mut teamI: size_t = 0;
    teamI = 0 as size_t;
    while teamI < 2 as size_t {
        MeleeSetup_initSentTeam(setup, teamI);
        teamI = teamI.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn MeleeSetup_setShip(
    mut setup: *mut MeleeSetup,
    mut teamNr: size_t,
    mut slotNr: FleetShipIndex,
    mut ship: MeleeShip,
) -> bool {
    let mut team: *mut MeleeTeam = (&raw mut (*setup).teams as *mut MeleeTeam)
        .offset(teamNr as isize);
    let mut oldShip: MeleeShip = MeleeTeam_getShip(team, slotNr);
    if ship.0 == oldShip.0 {
        return false;
    }
    if oldShip.0 != MeleeShip::MELEE_NONE.0 {
        (*setup).fleetValue[teamNr] = ((*setup).fleetValue[teamNr] as ::core::ffi::c_int
            - GetShipCostFromIndex(oldShip.0) as ::core::ffi::c_int) as COUNT;
    }
    MeleeTeam_setShip(team, slotNr, ship);
    if ship.0 != MeleeShip::MELEE_NONE.0 {
        (*setup).fleetValue[teamNr] = ((*setup).fleetValue[teamNr] as ::core::ffi::c_int
            + GetShipCostFromIndex(ship.0) as ::core::ffi::c_int) as COUNT;
    }
    return true;
}
#[no_mangle]
pub unsafe extern "C" fn MeleeSetup_getShip(
    mut setup: *const MeleeSetup,
    mut teamNr: size_t,
    mut slotNr: FleetShipIndex,
) -> MeleeShip {
    return MeleeTeam_getShip(
        (&raw const (*setup).teams as *const MeleeTeam).offset(teamNr as isize),
        slotNr,
    );
}
#[no_mangle]
pub unsafe extern "C" fn MeleeSetup_getFleet(
    mut setup: *const MeleeSetup,
    mut teamNr: size_t,
) -> *const MeleeShip {
    return MeleeTeam_getFleet(
        (&raw const (*setup).teams as *const MeleeTeam).offset(teamNr as isize),
    );
}
#[no_mangle]
pub unsafe extern "C" fn MeleeSetup_setTeamName(
    mut setup: *mut MeleeSetup,
    mut teamNr: size_t,
    mut name: *const ::core::ffi::c_char,
) -> bool {
    let mut team: *mut MeleeTeam = (&raw mut (*setup).teams as *mut MeleeTeam)
        .offset(teamNr as isize);
    let mut oldName: *const ::core::ffi::c_char = MeleeTeam_getTeamName(team);
    if strcmp(oldName, name) == 0 as ::core::ffi::c_int {
        return false;
    }
    MeleeTeam_setName(team, name);
    return true;
}
#[no_mangle]
pub unsafe extern "C" fn MeleeSetup_getTeamName(
    mut setup: *const MeleeSetup,
    mut teamNr: size_t,
) -> *const ::core::ffi::c_char {
    return MeleeTeam_getTeamName(
        (&raw const (*setup).teams as *const MeleeTeam).offset(teamNr as isize),
    );
}
#[no_mangle]
pub unsafe extern "C" fn MeleeSetup_getFleetValue(
    mut setup: *const MeleeSetup,
    mut teamNr: size_t,
) -> COUNT {
    return (*setup).fleetValue[teamNr];
}
#[no_mangle]
pub unsafe extern "C" fn MeleeSetup_deserializeTeam(
    mut setup: *mut MeleeSetup,
    mut teamNr: size_t,
    mut stream: *mut uio_Stream,
) -> ::core::ffi::c_int {
    let mut team: *mut MeleeTeam = (&raw mut (*setup).teams as *mut MeleeTeam)
        .offset(teamNr as isize);
    let mut ret: ::core::ffi::c_int = uqm_lua_checked!(MeleeTeam_deserialize(team, stream), 0);
    if ret == 0 as ::core::ffi::c_int {
        (*setup).fleetValue[teamNr] = MeleeTeam_getValue(team);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn MeleeSetup_serializeTeam(
    mut setup: *const MeleeSetup,
    mut teamNr: size_t,
    mut stream: *mut uio_Stream,
) -> ::core::ffi::c_int {
    let mut team: *const MeleeTeam = (&raw const (*setup).teams as *const MeleeTeam)
        .offset(teamNr as isize);
    return uqm_lua_checked!(MeleeTeam_serialize(team, stream), 0);
}
#[no_mangle]
pub unsafe extern "C" fn MeleeSetup_getSentShip(
    mut setup: *const MeleeSetup,
    mut teamNr: size_t,
    mut slotNr: FleetShipIndex,
) -> MeleeShip {
    return MeleeTeam_getShip(
        (&raw const (*setup).sentTeams as *const MeleeTeam).offset(teamNr as isize),
        slotNr,
    );
}
#[no_mangle]
pub unsafe extern "C" fn MeleeSetup_getSentTeamName(
    mut setup: *const MeleeSetup,
    mut teamNr: size_t,
) -> *const ::core::ffi::c_char {
    if !(*setup).haveSentTeamName[teamNr] {
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    return MeleeTeam_getTeamName(
        (&raw const (*setup).sentTeams as *const MeleeTeam).offset(teamNr as isize),
    );
}
#[no_mangle]
pub unsafe extern "C" fn MeleeSetup_setSentShip(
    mut setup: *mut MeleeSetup,
    mut teamNr: size_t,
    mut slotNr: FleetShipIndex,
    mut ship: MeleeShip,
) -> bool {
    let mut team: *mut MeleeTeam = (&raw mut (*setup).sentTeams as *mut MeleeTeam)
        .offset(teamNr as isize);
    let mut oldShip: MeleeShip = MeleeTeam_getShip(team, slotNr);
    if ship.0 == oldShip.0 {
        return false;
    }
    MeleeTeam_setShip(team, slotNr, ship);
    return true;
}
#[no_mangle]
pub unsafe extern "C" fn MeleeSetup_setSentTeamName(
    mut setup: *mut MeleeSetup,
    mut teamNr: size_t,
    mut name: *const ::core::ffi::c_char,
) -> bool {
    let mut haveSentName: bool = (*setup).haveSentTeamName[teamNr];
    if name.is_null() {
        if !haveSentName {
            return false;
        }
        let mut team: *mut MeleeTeam = (&raw mut (*setup).sentTeams as *mut MeleeTeam)
            .offset(teamNr as isize);
        MeleeTeam_setName(team, b"<INVALID>\0".as_ptr() as *const ::core::ffi::c_char);
    } else {
        let mut team_0: *mut MeleeTeam = (&raw mut (*setup).sentTeams as *mut MeleeTeam)
            .offset(teamNr as isize);
        if haveSentName {
            let mut oldName: *const ::core::ffi::c_char = MeleeTeam_getTeamName(team_0);
            if strcmp(oldName, name) == 0 as ::core::ffi::c_int {
                return false;
            }
        }
        MeleeTeam_setName(team_0, name);
    }
    (*setup).haveSentTeamName[teamNr] = !name.is_null();
    return true;
}
#[export_name = "uqm_c2rust_init_f2b808cc7fbb65e1"]
unsafe extern "C" fn c2rust_run_static_initializers() {
    MeleeTeam_serialSize = (14 as size_t)
        .wrapping_add(::core::mem::size_of::<[::core::ffi::c_char; 55]>());
}
// Explicit engine initialization replaces platform constructor tables.
