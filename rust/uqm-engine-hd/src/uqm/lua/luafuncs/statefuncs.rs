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
 *  Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA  02111-1307  USA
 */

// Rust translation of src/uqm/lua/luafuncs/statefuncs.c.
extern "C" {
    pub type lua_State;
    pub type frame_desc;
    #[link_name = "uqm_lua_guard_lua_type"]
    fn lua_type(L: *mut lua_State, idx: ::core::ffi::c_int) -> ::core::ffi::c_int;
    #[link_name = "uqm_lua_guard_lua_toboolean"]
    fn lua_toboolean(L: *mut lua_State, idx: ::core::ffi::c_int) -> ::core::ffi::c_int;
    #[link_name = "uqm_lua_guard_lua_pushinteger"]
    fn lua_pushinteger(L: *mut lua_State, n: lua_Integer);
    #[link_name = "uqm_lua_guard_lua_pushstring"]
    fn lua_pushstring(
        L: *mut lua_State,
        s: *const ::core::ffi::c_char,
    ) -> *const ::core::ffi::c_char;
    #[link_name = "uqm_lua_guard_lua_pushboolean"]
    fn lua_pushboolean(L: *mut lua_State, b: ::core::ffi::c_int);
    #[link_name = "uqm_lua_guard_lua_createtable"]
    fn lua_createtable(
        L: *mut lua_State,
        narr: ::core::ffi::c_int,
        nrec: ::core::ffi::c_int,
    );
    #[link_name = "uqm_lua_guard_lua_setfield"]
    fn lua_setfield(
        L: *mut lua_State,
        idx: ::core::ffi::c_int,
        k: *const ::core::ffi::c_char,
    );
    #[link_name = "uqm_lua_guard_luaL_checklstring"]
    fn luaL_checklstring(
        L: *mut lua_State,
        numArg: ::core::ffi::c_int,
        l: *mut size_t,
    ) -> *const ::core::ffi::c_char;
    #[link_name = "uqm_lua_guard_luaL_checkinteger"]
    fn luaL_checkinteger(L: *mut lua_State, numArg: ::core::ffi::c_int) -> lua_Integer;
    #[link_name = "uqm_lua_guard_luaL_checktype"]
    fn luaL_checktype(
        L: *mut lua_State,
        narg: ::core::ffi::c_int,
        t: ::core::ffi::c_int,
    );
    #[link_name = "uqm_lua_guard_luaL_setfuncs"]
    fn luaL_setfuncs(L: *mut lua_State, l: *const luaL_Reg, nup: ::core::ffi::c_int);
    fn log_add(_: log_Level, fmt: *const ::core::ffi::c_char, ...);
    fn DrawLanders();
    fn DeltaSISGauges(
        crew_delta: SIZE,
        fuel_delta: SDWORD,
        resunit_delta: ::core::ffi::c_int,
    );
    static mut GlobData: GLOBDATA;
    static mut luaUqm_delta: DELTA_TYPES;
    fn luaUqm_getProp(luaState: *mut lua_State, nameIndex: ::core::ffi::c_int);
    fn luaUqm_setProp(
        luaState: *mut lua_State,
        nameIndex: ::core::ffi::c_int,
        valueIndex: ::core::ffi::c_int,
    );
    fn luaUqm_checkPropValueType(
        luaState: *mut lua_State,
        funName: *const ::core::ffi::c_char,
        nameIndex: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn ShipIdStrToIndex(shipIdStr: *const ::core::ffi::c_char) -> SPECIES_ID;
    fn RaceIdStrToIndex(raceIdStr: *const ::core::ffi::c_char) -> RACE_ID;
    fn AddEscortShips(race: RACE_ID, count: SIZE) -> COUNT;
    fn CalculateEscortsWorth() -> COUNT;
    fn SetRaceAllied(race: RACE_ID, flag: BOOLEAN) -> BOOLEAN;
    fn StartSphereTracking(race: RACE_ID) -> COUNT;
    fn CheckSphereTracking(race: RACE_ID) -> BOOLEAN;
    fn KillRace(race: RACE_ID) -> BOOLEAN;
    fn CountEscortShips(race: RACE_ID) -> COUNT;
    fn EscortFeasibilityStudy(race: RACE_ID) -> COUNT;
    fn CheckAlliance(race: RACE_ID) -> COUNT;
    fn RemoveSomeEscortShips(race: RACE_ID, count: COUNT) -> COUNT;
    fn RemoveEscortShips(race: RACE_ID) -> COUNT;
}
pub type size_t = ::core::ffi::c_ulong;
pub type ptrdiff_t = isize;
pub type lua_CFunction = Option<
    unsafe extern "C" fn(*mut lua_State) -> ::core::ffi::c_int,
>;
pub type lua_Integer = ptrdiff_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct luaL_Reg {
    pub name: *const ::core::ffi::c_char,
    pub func: lua_CFunction,
}
pub type uint8 = ::core::ffi::c_uchar;
pub type uint16 = ::core::ffi::c_ushort;
pub type sint16 = ::core::ffi::c_short;
pub type uint32 = ::core::ffi::c_uint;
pub type sint32 = ::core::ffi::c_int;
pub type BYTE = uint8;
pub type UWORD = uint16;
pub type SWORD = sint16;
pub type DWORD = uint32;
pub type SDWORD = sint32;
pub type COUNT = UWORD;
pub type SIZE = SWORD;
pub type UNICODE = ::core::ffi::c_char;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct BOOLEAN(pub ::core::ffi::c_uint);
impl BOOLEAN {
    pub const FALSE: Self = Self(0);
    pub const TRUE: Self = Self(1);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Color {
    pub r: BYTE,
    pub g: BYTE,
    pub b: BYTE,
    pub a: BYTE,
}
pub type FRAME_DESC = frame_desc;
pub type FRAME = *mut FRAME_DESC;
pub type COORD = SWORD;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct extent {
    pub width: COORD,
    pub height: COORD,
}
pub type EXTENT = extent;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct point {
    pub x: COORD,
    pub y: COORD,
}
pub type POINT = point;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stamp {
    pub origin: POINT,
    pub frame: FRAME,
}
pub type STAMP = stamp;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rect {
    pub corner: POINT,
    pub extent: EXTENT,
}
pub type RECT = rect;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct line {
    pub first: POINT,
    pub second: POINT,
}
pub type LINE = line;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct TEXT_ALIGN(pub ::core::ffi::c_uint);
impl TEXT_ALIGN {
    pub const ALIGN_LEFT: Self = Self(0);
    pub const ALIGN_CENTER: Self = Self(1);
    pub const ALIGN_RIGHT: Self = Self(2);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct text {
    pub baseline: POINT,
    pub pStr: *const UNICODE,
    pub align: TEXT_ALIGN,
    pub CharCount: COUNT,
}
pub type TEXT = text;
pub type GRAPHICS_PRIM = BYTE;
#[derive(Copy, Clone)]
#[repr(C)]
pub union PRIM_DESC {
    pub Point: POINT,
    pub Stamp: STAMP,
    pub Line: LINE,
    pub Text: TEXT,
    pub Rect: RECT,
}
pub type PRIM_LINKS = DWORD;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PRIMITIVE {
    pub Links: PRIM_LINKS,
    pub Type: GRAPHICS_PRIM,
    pub color: Color,
    pub Object: PRIM_DESC,
    pub flags: BYTE,
}
pub type DELTA_TYPES = delta_types;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct delta_types {
    pub _int: ::core::ffi::c_int,
    pub _bool: BOOLEAN,
    pub _str: *mut UNICODE,
    pub _char: ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SIS_STATE {
    pub log_x: SDWORD,
    pub log_y: SDWORD,
    pub ResUnits: DWORD,
    pub FuelOnBoard: DWORD,
    pub CrewEnlisted: COUNT,
    pub TotalElementMass: COUNT,
    pub TotalBioMass: COUNT,
    pub ModuleSlots: [BYTE; 16],
    pub DriveSlots: [BYTE; 11],
    pub JetSlots: [BYTE; 8],
    pub NumLanders: BYTE,
    pub ElementAmounts: [COUNT; 8],
    pub ShipName: [UNICODE; 32],
    pub CommanderName: [UNICODE; 32],
    pub PlanetName: [UNICODE; 32],
    pub Difficulty: BYTE,
    pub Extended: BYTE,
    pub Nomad: BYTE,
    pub ShipSeed: BYTE,
    pub Seed: SDWORD,
    pub SaveVersion: BYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GLOBDATA {
    pub SIS_state: SIS_STATE,
    pub Game_state: GAME_STATE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GAME_STATE {
    pub glob_flags: BYTE,
    pub CrewCost: BYTE,
    pub FuelCost: BYTE,
    pub ModuleCost: [BYTE; 20],
    pub ElementWorth: [BYTE; 8],
    pub DisplayArray: *mut PRIMITIVE,
    pub CurrentActivity: ACTIVITY,
    pub GameClock: CLOCK_STATE,
    pub autopilot: POINT,
    pub ip_location: POINT,
    pub ShipStamp: STAMP,
    pub ShipFacing: UWORD,
    pub ip_planet: BYTE,
    pub in_orbit: BYTE,
    pub velocity: VELOCITY_DESC,
    pub BattleGroupRef: DWORD,
    pub avail_race_q: QUEUE,
    pub npc_built_ship_q: QUEUE,
    pub ip_group_q: QUEUE,
    pub encounter_q: QUEUE,
    pub built_ship_q: QUEUE,
    pub stowed_ship_q: QUEUE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct QUEUE {
    pub head: HLINK,
    pub tail: HLINK,
    pub pq_tab: *mut BYTE,
    pub free_list: HLINK,
    pub object_size: COUNT,
    pub num_objects: BYTE,
}
pub type HLINK = QUEUE_HANDLE;
pub type QUEUE_HANDLE = *mut ::core::ffi::c_void;
pub type VELOCITY_DESC = velocity_desc;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct velocity_desc {
    pub TravelAngle: COUNT,
    pub vector: EXTENT,
    pub fract: EXTENT,
    pub error: EXTENT,
    pub incr: EXTENT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CLOCK_STATE {
    pub day_index: BYTE,
    pub month_index: BYTE,
    pub year_index: COUNT,
    pub tick_count: SIZE,
    pub day_in_ticks: SIZE,
    pub event_q: QUEUE,
}
pub type ACTIVITY = UWORD;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct RACE_ID(pub ::core::ffi::c_uint);
impl RACE_ID {
    pub const ARILOU_SHIP: Self = Self(0);
    pub const CHMMR_SHIP: Self = Self(1);
    pub const HUMAN_SHIP: Self = Self(2);
    pub const ORZ_SHIP: Self = Self(3);
    pub const PKUNK_SHIP: Self = Self(4);
    pub const SHOFIXTI_SHIP: Self = Self(5);
    pub const SPATHI_SHIP: Self = Self(6);
    pub const SUPOX_SHIP: Self = Self(7);
    pub const THRADDASH_SHIP: Self = Self(8);
    pub const UTWIG_SHIP: Self = Self(9);
    pub const VUX_SHIP: Self = Self(10);
    pub const YEHAT_SHIP: Self = Self(11);
    pub const MELNORME_SHIP: Self = Self(12);
    pub const DRUUGE_SHIP: Self = Self(13);
    pub const ILWRATH_SHIP: Self = Self(14);
    pub const MYCON_SHIP: Self = Self(15);
    pub const SLYLANDRO_SHIP: Self = Self(16);
    pub const UMGAH_SHIP: Self = Self(17);
    pub const URQUAN_SHIP: Self = Self(18);
    pub const ZOQFOTPIK_SHIP: Self = Self(19);
    pub const SYREEN_SHIP: Self = Self(20);
    pub const BLACK_URQUAN_SHIP: Self = Self(21);
    pub const ANDROSYNTH_SHIP: Self = Self(22);
    pub const CHENJESU_SHIP: Self = Self(23);
    pub const MMRNMHRM_SHIP: Self = Self(24);
    pub const NUM_BUILDABLE_SHIPS: Self = Self(25);
    pub const YEHAT_REBEL_SHIP: Self = Self(25);
    pub const URQUAN_DRONE_SHIP: Self = Self(26);
    pub const SAMATRA_SHIP: Self = Self(26);
    pub const NUM_AVAILABLE_RACES: Self = Self(27);
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
pub struct SPECIES_ID(pub ::core::ffi::c_uint);
impl SPECIES_ID {
    pub const NO_ID: Self = Self(0);
    pub const ARILOU_ID: Self = Self(1);
    pub const CHMMR_ID: Self = Self(2);
    pub const EARTHLING_ID: Self = Self(3);
    pub const ORZ_ID: Self = Self(4);
    pub const PKUNK_ID: Self = Self(5);
    pub const SHOFIXTI_ID: Self = Self(6);
    pub const SPATHI_ID: Self = Self(7);
    pub const SUPOX_ID: Self = Self(8);
    pub const THRADDASH_ID: Self = Self(9);
    pub const UTWIG_ID: Self = Self(10);
    pub const VUX_ID: Self = Self(11);
    pub const YEHAT_ID: Self = Self(12);
    pub const MELNORME_ID: Self = Self(13);
    pub const DRUUGE_ID: Self = Self(14);
    pub const ILWRATH_ID: Self = Self(15);
    pub const MYCON_ID: Self = Self(16);
    pub const SLYLANDRO_ID: Self = Self(17);
    pub const UMGAH_ID: Self = Self(18);
    pub const UR_QUAN_ID: Self = Self(19);
    pub const ZOQFOTPIK_ID: Self = Self(20);
    pub const SYREEN_ID: Self = Self(21);
    pub const KOHR_AH_ID: Self = Self(22);
    pub const ANDROSYNTH_ID: Self = Self(23);
    pub const CHENJESU_ID: Self = Self(24);
    pub const MMRNMHRM_ID: Self = Self(25);
    pub const LAST_MELEE_ID: Self = Self(25);
    pub const SIS_SHIP_ID: Self = Self(26);
    pub const SA_MATRA_ID: Self = Self(27);
    pub const UR_QUAN_PROBE_ID: Self = Self(28);
    pub const NUM_SPECIES_ID: Self = Self(29);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed {
    pub const DEAD_GUY: Self = Self(0);
    pub const GOOD_GUY: Self = Self(1);
    pub const BAD_GUY: Self = Self(2);
}
static mut stateClockFuncs: [luaL_Reg; 2] = [
    luaL_Reg {
        name: b"getDate\0".as_ptr() as *const ::core::ffi::c_char,
        func: Some(
            luaUqm_state_clock_getDate
                as unsafe extern "C" fn(*mut lua_State) -> ::core::ffi::c_int,
        ),
    },
    luaL_Reg {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        func: None,
    },
];
static mut stateEscortFuncs: [luaL_Reg; 6] = [
    luaL_Reg {
        name: b"addShips\0".as_ptr() as *const ::core::ffi::c_char,
        func: Some(
            luaUqm_state_escort_addShips
                as unsafe extern "C" fn(*mut lua_State) -> ::core::ffi::c_int,
        ),
    },
    luaL_Reg {
        name: b"canAddShips\0".as_ptr() as *const ::core::ffi::c_char,
        func: Some(
            luaUqm_state_escort_canAddShips
                as unsafe extern "C" fn(*mut lua_State) -> ::core::ffi::c_int,
        ),
    },
    luaL_Reg {
        name: b"removeShips\0".as_ptr() as *const ::core::ffi::c_char,
        func: Some(
            luaUqm_state_escort_removeShips
                as unsafe extern "C" fn(*mut lua_State) -> ::core::ffi::c_int,
        ),
    },
    luaL_Reg {
        name: b"shipCount\0".as_ptr() as *const ::core::ffi::c_char,
        func: Some(
            luaUqm_state_escort_shipCount
                as unsafe extern "C" fn(*mut lua_State) -> ::core::ffi::c_int,
        ),
    },
    luaL_Reg {
        name: b"totalValue\0".as_ptr() as *const ::core::ffi::c_char,
        func: Some(
            luaUqm_state_escort_totalValue
                as unsafe extern "C" fn(*mut lua_State) -> ::core::ffi::c_int,
        ),
    },
    luaL_Reg {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        func: None,
    },
];
static mut statePropFuncs: [luaL_Reg; 3] = [
    luaL_Reg {
        name: b"get\0".as_ptr() as *const ::core::ffi::c_char,
        func: Some(
            luaUqm_state_prop_get
                as unsafe extern "C" fn(*mut lua_State) -> ::core::ffi::c_int,
        ),
    },
    luaL_Reg {
        name: b"set\0".as_ptr() as *const ::core::ffi::c_char,
        func: Some(
            luaUqm_state_prop_set
                as unsafe extern "C" fn(*mut lua_State) -> ::core::ffi::c_int,
        ),
    },
    luaL_Reg {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        func: None,
    },
];
static mut stateRaceFuncs: [luaL_Reg; 7] = [
    luaL_Reg {
        name: b"isAlive\0".as_ptr() as *const ::core::ffi::c_char,
        func: Some(
            luaUqm_state_race_isAlive
                as unsafe extern "C" fn(*mut lua_State) -> ::core::ffi::c_int,
        ),
    },
    luaL_Reg {
        name: b"isAllied\0".as_ptr() as *const ::core::ffi::c_char,
        func: Some(
            luaUqm_state_race_isAllied
                as unsafe extern "C" fn(*mut lua_State) -> ::core::ffi::c_int,
        ),
    },
    luaL_Reg {
        name: b"isKnown\0".as_ptr() as *const ::core::ffi::c_char,
        func: Some(
            luaUqm_state_race_isKnown
                as unsafe extern "C" fn(*mut lua_State) -> ::core::ffi::c_int,
        ),
    },
    luaL_Reg {
        name: b"setAlive\0".as_ptr() as *const ::core::ffi::c_char,
        func: Some(
            luaUqm_state_race_setAlive
                as unsafe extern "C" fn(*mut lua_State) -> ::core::ffi::c_int,
        ),
    },
    luaL_Reg {
        name: b"setAllied\0".as_ptr() as *const ::core::ffi::c_char,
        func: Some(
            luaUqm_state_race_setAllied
                as unsafe extern "C" fn(*mut lua_State) -> ::core::ffi::c_int,
        ),
    },
    luaL_Reg {
        name: b"setKnown\0".as_ptr() as *const ::core::ffi::c_char,
        func: Some(
            luaUqm_state_race_setKnown
                as unsafe extern "C" fn(*mut lua_State) -> ::core::ffi::c_int,
        ),
    },
    luaL_Reg {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        func: None,
    },
];
static mut stateSisFuncs: [luaL_Reg; 11] = [
    luaL_Reg {
        name: b"addCrew\0".as_ptr() as *const ::core::ffi::c_char,
        func: Some(
            luaUqm_state_sis_addCrew
                as unsafe extern "C" fn(*mut lua_State) -> ::core::ffi::c_int,
        ),
    },
    luaL_Reg {
        name: b"addFuel\0".as_ptr() as *const ::core::ffi::c_char,
        func: Some(
            luaUqm_state_sis_addFuel
                as unsafe extern "C" fn(*mut lua_State) -> ::core::ffi::c_int,
        ),
    },
    luaL_Reg {
        name: b"addLanders\0".as_ptr() as *const ::core::ffi::c_char,
        func: Some(
            luaUqm_state_sis_addLanders
                as unsafe extern "C" fn(*mut lua_State) -> ::core::ffi::c_int,
        ),
    },
    luaL_Reg {
        name: b"addResUnits\0".as_ptr() as *const ::core::ffi::c_char,
        func: Some(
            luaUqm_state_sis_addResUnits
                as unsafe extern "C" fn(*mut lua_State) -> ::core::ffi::c_int,
        ),
    },
    luaL_Reg {
        name: b"getCaptainName\0".as_ptr() as *const ::core::ffi::c_char,
        func: Some(
            luaUqm_state_sis_getCaptainName
                as unsafe extern "C" fn(*mut lua_State) -> ::core::ffi::c_int,
        ),
    },
    luaL_Reg {
        name: b"getCrew\0".as_ptr() as *const ::core::ffi::c_char,
        func: Some(
            luaUqm_state_sis_getCrew
                as unsafe extern "C" fn(*mut lua_State) -> ::core::ffi::c_int,
        ),
    },
    luaL_Reg {
        name: b"getFuel\0".as_ptr() as *const ::core::ffi::c_char,
        func: Some(
            luaUqm_state_sis_getFuel
                as unsafe extern "C" fn(*mut lua_State) -> ::core::ffi::c_int,
        ),
    },
    luaL_Reg {
        name: b"getLanders\0".as_ptr() as *const ::core::ffi::c_char,
        func: Some(
            luaUqm_state_sis_getLanders
                as unsafe extern "C" fn(*mut lua_State) -> ::core::ffi::c_int,
        ),
    },
    luaL_Reg {
        name: b"getResUnits\0".as_ptr() as *const ::core::ffi::c_char,
        func: Some(
            luaUqm_state_sis_getResUnits
                as unsafe extern "C" fn(*mut lua_State) -> ::core::ffi::c_int,
        ),
    },
    luaL_Reg {
        name: b"getShipName\0".as_ptr() as *const ::core::ffi::c_char,
        func: Some(
            luaUqm_state_sis_getShipName
                as unsafe extern "C" fn(*mut lua_State) -> ::core::ffi::c_int,
        ),
    },
    luaL_Reg {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        func: None,
    },
];
static mut stateMiscFuncs: [luaL_Reg; 2] = [
    luaL_Reg {
        name: b"alignText\0".as_ptr() as *const ::core::ffi::c_char,
        func: Some(
            luaUqm_state_misc_alignText
                as unsafe extern "C" fn(*mut lua_State) -> ::core::ffi::c_int,
        ),
    },
    luaL_Reg {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        func: None,
    },
];
#[no_mangle]
pub unsafe extern "C" fn luaUqm_state_open(
    mut luaState: *mut lua_State,
) -> ::core::ffi::c_int {
    uqm_lua_checked!(lua_createtable(luaState, 0 as ::core::ffi::c_int, 5 as ::core::ffi::c_int), 0);
    uqm_lua_checked!(lua_createtable(
        luaState,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[luaL_Reg; 2]>()
            .wrapping_div(::core::mem::size_of::<luaL_Reg>())
            .wrapping_sub(1usize) as ::core::ffi::c_int,
    ), 0);
    uqm_lua_checked!(luaL_setfuncs(
        luaState,
        &raw const stateClockFuncs as *const luaL_Reg,
        0 as ::core::ffi::c_int,
    ), 0);
    uqm_lua_checked!(lua_setfield(
        luaState,
        -2 as ::core::ffi::c_int,
        b"clock\0".as_ptr() as *const ::core::ffi::c_char,
    ), 0);
    uqm_lua_checked!(lua_createtable(
        luaState,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[luaL_Reg; 6]>()
            .wrapping_div(::core::mem::size_of::<luaL_Reg>())
            .wrapping_sub(1usize) as ::core::ffi::c_int,
    ), 0);
    uqm_lua_checked!(luaL_setfuncs(
        luaState,
        &raw const stateEscortFuncs as *const luaL_Reg,
        0 as ::core::ffi::c_int,
    ), 0);
    uqm_lua_checked!(lua_setfield(
        luaState,
        -2 as ::core::ffi::c_int,
        b"escort\0".as_ptr() as *const ::core::ffi::c_char,
    ), 0);
    uqm_lua_checked!(lua_createtable(
        luaState,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[luaL_Reg; 3]>()
            .wrapping_div(::core::mem::size_of::<luaL_Reg>())
            .wrapping_sub(1usize) as ::core::ffi::c_int,
    ), 0);
    uqm_lua_checked!(luaL_setfuncs(
        luaState,
        &raw const statePropFuncs as *const luaL_Reg,
        0 as ::core::ffi::c_int,
    ), 0);
    uqm_lua_checked!(lua_setfield(
        luaState,
        -2 as ::core::ffi::c_int,
        b"prop\0".as_ptr() as *const ::core::ffi::c_char,
    ), 0);
    uqm_lua_checked!(lua_createtable(
        luaState,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[luaL_Reg; 7]>()
            .wrapping_div(::core::mem::size_of::<luaL_Reg>())
            .wrapping_sub(1usize) as ::core::ffi::c_int,
    ), 0);
    uqm_lua_checked!(luaL_setfuncs(
        luaState,
        &raw const stateRaceFuncs as *const luaL_Reg,
        0 as ::core::ffi::c_int,
    ), 0);
    uqm_lua_checked!(lua_setfield(
        luaState,
        -2 as ::core::ffi::c_int,
        b"race\0".as_ptr() as *const ::core::ffi::c_char,
    ), 0);
    uqm_lua_checked!(lua_createtable(
        luaState,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[luaL_Reg; 11]>()
            .wrapping_div(::core::mem::size_of::<luaL_Reg>())
            .wrapping_sub(1usize) as ::core::ffi::c_int,
    ), 0);
    uqm_lua_checked!(luaL_setfuncs(
        luaState,
        &raw const stateSisFuncs as *const luaL_Reg,
        0 as ::core::ffi::c_int,
    ), 0);
    uqm_lua_checked!(lua_setfield(
        luaState,
        -2 as ::core::ffi::c_int,
        b"sis\0".as_ptr() as *const ::core::ffi::c_char,
    ), 0);
    uqm_lua_checked!(lua_createtable(
        luaState,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[luaL_Reg; 2]>()
            .wrapping_div(::core::mem::size_of::<luaL_Reg>())
            .wrapping_sub(1usize) as ::core::ffi::c_int,
    ), 0);
    uqm_lua_checked!(luaL_setfuncs(
        luaState,
        &raw const stateMiscFuncs as *const luaL_Reg,
        0 as ::core::ffi::c_int,
    ), 0);
    uqm_lua_checked!(lua_setfield(
        luaState,
        -2 as ::core::ffi::c_int,
        b"misc\0".as_ptr() as *const ::core::ffi::c_char,
    ), 0);
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn testRaceId(
    mut luaState: *mut lua_State,
    mut argn: ::core::ffi::c_int,
) -> COUNT {
    let mut raceIdStr: *const ::core::ffi::c_char = uqm_lua_checked!(luaL_checklstring(
        luaState,
        argn,
        ::core::ptr::null_mut::<size_t>(),
    ), 0);
    let mut raceId: COUNT = RaceIdStrToIndex(raceIdStr).0 as COUNT;
    if raceId as ::core::ffi::c_int
        == -1 as ::core::ffi::c_int as COUNT as ::core::ffi::c_int
    {
        log_add(
            log_Level::log_Error,
            b"[script] Warning: testRaceId(): No race exists with id '%s'.\0".as_ptr()
                as *const ::core::ffi::c_char,
            raceIdStr,
        );
        return -1 as ::core::ffi::c_int as COUNT;
    }
    return raceId;
}
unsafe extern "C" fn testShipId(
    mut luaState: *mut lua_State,
    mut argn: ::core::ffi::c_int,
) -> COUNT {
    let mut shipIdStr: *const ::core::ffi::c_char = uqm_lua_checked!(luaL_checklstring(
        luaState,
        argn,
        ::core::ptr::null_mut::<size_t>(),
    ), 0);
    let mut shipId: COUNT = ShipIdStrToIndex(shipIdStr).0 as COUNT;
    if shipId as ::core::ffi::c_int
        == -1 as ::core::ffi::c_int as COUNT as ::core::ffi::c_int
    {
        log_add(
            log_Level::log_Error,
            b"[script] Warning: testShipId(): No ship exists with id '%s'.\0".as_ptr()
                as *const ::core::ffi::c_char,
            shipIdStr,
        );
        return -1 as ::core::ffi::c_int as COUNT;
    }
    return shipId;
}
unsafe extern "C" fn luaUqm_state_clock_getDate(
    mut luaState: *mut lua_State,
) -> ::core::ffi::c_int {
    uqm_lua_checked!(lua_createtable(luaState, 0 as ::core::ffi::c_int, 3 as ::core::ffi::c_int), 0);
    uqm_lua_checked!(lua_pushinteger(luaState, GlobData.Game_state.GameClock.year_index as lua_Integer), 0);
    uqm_lua_checked!(lua_setfield(
        luaState,
        -2 as ::core::ffi::c_int,
        b"year\0".as_ptr() as *const ::core::ffi::c_char,
    ), 0);
    uqm_lua_checked!(lua_pushinteger(luaState, GlobData.Game_state.GameClock.month_index as lua_Integer), 0);
    uqm_lua_checked!(lua_setfield(
        luaState,
        -2 as ::core::ffi::c_int,
        b"month\0".as_ptr() as *const ::core::ffi::c_char,
    ), 0);
    uqm_lua_checked!(lua_pushinteger(luaState, GlobData.Game_state.GameClock.day_index as lua_Integer), 0);
    uqm_lua_checked!(lua_setfield(
        luaState,
        -2 as ::core::ffi::c_int,
        b"day\0".as_ptr() as *const ::core::ffi::c_char,
    ), 0);
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn luaUqm_state_escort_addShips(
    mut luaState: *mut lua_State,
) -> ::core::ffi::c_int {
    let mut shipId: COUNT = 0;
    let mut count: ::core::ffi::c_int = 0;
    let mut numAdded: ::core::ffi::c_int = 0;
    shipId = uqm_lua_checked!(testShipId(luaState, 1 as ::core::ffi::c_int), 0);
    if shipId as ::core::ffi::c_int
        == -1 as ::core::ffi::c_int as COUNT as ::core::ffi::c_int
    {
        uqm_lua_checked!(lua_pushboolean(luaState, BOOLEAN::FALSE.0 as ::core::ffi::c_int), 0);
        return 1 as ::core::ffi::c_int;
    }
    count = uqm_lua_checked!(luaL_checkinteger(luaState, 2 as ::core::ffi::c_int), 0) as ::core::ffi::c_int;
    numAdded = uqm_lua_checked!(AddEscortShips(
        RACE_ID(
            (shipId as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_uint,
        ),
        count as SIZE,
    ), 0) as ::core::ffi::c_int;
    uqm_lua_checked!(lua_pushinteger(luaState, numAdded as lua_Integer), 0);
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn luaUqm_state_escort_canAddShips(
    mut luaState: *mut lua_State,
) -> ::core::ffi::c_int {
    let mut shipId: COUNT = 0;
    let mut result: ::core::ffi::c_int = 0;
    shipId = uqm_lua_checked!(testShipId(luaState, 1 as ::core::ffi::c_int), 0);
    if shipId as ::core::ffi::c_int
        == -1 as ::core::ffi::c_int as COUNT as ::core::ffi::c_int
    {
        uqm_lua_checked!(lua_pushboolean(luaState, BOOLEAN::FALSE.0 as ::core::ffi::c_int), 0);
        return 1 as ::core::ffi::c_int;
    }
    result = EscortFeasibilityStudy(
        RACE_ID(
            (shipId as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_uint,
        ),
    ) as ::core::ffi::c_int;
    uqm_lua_checked!(lua_pushinteger(luaState, result as lua_Integer), 0);
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn luaUqm_state_escort_removeShips(
    mut luaState: *mut lua_State,
) -> ::core::ffi::c_int {
    let mut shipId: COUNT = 0;
    let mut numRemoved: ::core::ffi::c_int = 0;
    shipId = uqm_lua_checked!(testShipId(luaState, 1 as ::core::ffi::c_int), 0);
    if shipId as ::core::ffi::c_int
        == -1 as ::core::ffi::c_int as COUNT as ::core::ffi::c_int
    {
        uqm_lua_checked!(lua_pushboolean(luaState, BOOLEAN::FALSE.0 as ::core::ffi::c_int), 0);
        return 1 as ::core::ffi::c_int;
    }
    if uqm_lua_checked!(lua_type(luaState, 2 as ::core::ffi::c_int), 0) == 0 as ::core::ffi::c_int {
        numRemoved = uqm_lua_checked!(RemoveEscortShips(
            RACE_ID(
                (shipId as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                    as ::core::ffi::c_uint,
            ),
        ), 0) as ::core::ffi::c_int;
    } else {
        let mut count: ::core::ffi::c_int = uqm_lua_checked!(luaL_checkinteger(
            luaState,
            2 as ::core::ffi::c_int,
        ), 0) as ::core::ffi::c_int;
        numRemoved = uqm_lua_checked!(RemoveSomeEscortShips(
            RACE_ID(
                (shipId as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                    as ::core::ffi::c_uint,
            ),
            count as COUNT,
        ), 0) as ::core::ffi::c_int;
    }
    uqm_lua_checked!(lua_pushinteger(luaState, numRemoved as lua_Integer), 0);
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn luaUqm_state_escort_shipCount(
    mut luaState: *mut lua_State,
) -> ::core::ffi::c_int {
    let mut shipId: COUNT = 0;
    let mut result: ::core::ffi::c_int = 0;
    shipId = uqm_lua_checked!(testShipId(luaState, 1 as ::core::ffi::c_int), 0);
    if shipId as ::core::ffi::c_int
        == -1 as ::core::ffi::c_int as COUNT as ::core::ffi::c_int
    {
        uqm_lua_checked!(lua_pushboolean(luaState, BOOLEAN::FALSE.0 as ::core::ffi::c_int), 0);
        return 1 as ::core::ffi::c_int;
    }
    result = CountEscortShips(
        RACE_ID(
            (shipId as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_uint,
        ),
    ) as ::core::ffi::c_int;
    uqm_lua_checked!(lua_pushinteger(luaState, result as lua_Integer), 0);
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn luaUqm_state_escort_totalValue(
    mut luaState: *mut lua_State,
) -> ::core::ffi::c_int {
    let mut result: COUNT = CalculateEscortsWorth();
    uqm_lua_checked!(lua_pushinteger(luaState, result as lua_Integer), 0);
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn luaUqm_state_race_isAlive(
    mut luaState: *mut lua_State,
) -> ::core::ffi::c_int {
    let mut raceId: COUNT = 0;
    let mut result: BOOLEAN = BOOLEAN::FALSE;
    raceId = uqm_lua_checked!(testRaceId(luaState, 1 as ::core::ffi::c_int), 0);
    if raceId as ::core::ffi::c_int
        == -1 as ::core::ffi::c_int as COUNT as ::core::ffi::c_int
    {
        uqm_lua_checked!(lua_pushboolean(luaState, BOOLEAN::FALSE.0 as ::core::ffi::c_int), 0);
        return 1 as ::core::ffi::c_int;
    }
    result = BOOLEAN(
        (CheckAlliance(RACE_ID(raceId as ::core::ffi::c_uint)) as ::core::ffi::c_int
            != C2Rust_Unnamed::DEAD_GUY.0 as ::core::ffi::c_int) as ::core::ffi::c_int
            as ::core::ffi::c_uint,
    );
    uqm_lua_checked!(lua_pushboolean(luaState, result.0 as ::core::ffi::c_int), 0);
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn luaUqm_state_race_isAllied(
    mut luaState: *mut lua_State,
) -> ::core::ffi::c_int {
    let mut raceId: COUNT = 0;
    let mut result: BOOLEAN = BOOLEAN::FALSE;
    raceId = uqm_lua_checked!(testRaceId(luaState, 1 as ::core::ffi::c_int), 0);
    if raceId as ::core::ffi::c_int
        == -1 as ::core::ffi::c_int as COUNT as ::core::ffi::c_int
    {
        uqm_lua_checked!(lua_pushboolean(luaState, BOOLEAN::FALSE.0 as ::core::ffi::c_int), 0);
        return 1 as ::core::ffi::c_int;
    }
    result = BOOLEAN(
        (CheckAlliance(RACE_ID(raceId as ::core::ffi::c_uint)) as ::core::ffi::c_int
            == C2Rust_Unnamed::GOOD_GUY.0 as ::core::ffi::c_int) as ::core::ffi::c_int
            as ::core::ffi::c_uint,
    );
    uqm_lua_checked!(lua_pushboolean(luaState, result.0 as ::core::ffi::c_int), 0);
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn luaUqm_state_race_isKnown(
    mut luaState: *mut lua_State,
) -> ::core::ffi::c_int {
    let mut raceId: COUNT = 0;
    let mut result: BOOLEAN = BOOLEAN::FALSE;
    raceId = uqm_lua_checked!(testRaceId(luaState, 1 as ::core::ffi::c_int), 0);
    if raceId as ::core::ffi::c_int
        == -1 as ::core::ffi::c_int as COUNT as ::core::ffi::c_int
    {
        uqm_lua_checked!(lua_pushboolean(luaState, BOOLEAN::FALSE.0 as ::core::ffi::c_int), 0);
        return 1 as ::core::ffi::c_int;
    }
    result = CheckSphereTracking(RACE_ID(raceId as ::core::ffi::c_uint));
    uqm_lua_checked!(lua_pushboolean(luaState, result.0 as ::core::ffi::c_int), 0);
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn luaUqm_state_race_setAlive(
    mut luaState: *mut lua_State,
) -> ::core::ffi::c_int {
    let mut raceId: COUNT = 0;
    let mut flag: ::core::ffi::c_int = 0;
    let mut result: BOOLEAN = BOOLEAN::FALSE;
    raceId = uqm_lua_checked!(testRaceId(luaState, 1 as ::core::ffi::c_int), 0);
    if raceId as ::core::ffi::c_int
        == -1 as ::core::ffi::c_int as COUNT as ::core::ffi::c_int
    {
        uqm_lua_checked!(lua_pushboolean(luaState, BOOLEAN::FALSE.0 as ::core::ffi::c_int), 0);
        return 1 as ::core::ffi::c_int;
    }
    flag = uqm_lua_checked!(lua_toboolean(luaState, 2 as ::core::ffi::c_int), 0);
    if flag != 0 as ::core::ffi::c_int {
        log_add(
            log_Level::log_Error,
            b"[script] Warning: luaUqm_state_race_setAlive(): setAlive(true) is not implemented.\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        uqm_lua_checked!(lua_pushboolean(luaState, BOOLEAN::FALSE.0 as ::core::ffi::c_int), 0);
        return 1 as ::core::ffi::c_int;
    }
    result = KillRace(RACE_ID(raceId as ::core::ffi::c_uint));
    uqm_lua_checked!(lua_pushboolean(luaState, result.0 as ::core::ffi::c_int), 0);
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn luaUqm_state_race_setAllied(
    mut luaState: *mut lua_State,
) -> ::core::ffi::c_int {
    let mut raceId: COUNT = 0;
    let mut flag: ::core::ffi::c_int = 0;
    let mut result: BOOLEAN = BOOLEAN::FALSE;
    raceId = uqm_lua_checked!(testRaceId(luaState, 1 as ::core::ffi::c_int), 0);
    if raceId as ::core::ffi::c_int
        == -1 as ::core::ffi::c_int as COUNT as ::core::ffi::c_int
    {
        uqm_lua_checked!(lua_pushboolean(luaState, BOOLEAN::FALSE.0 as ::core::ffi::c_int), 0);
        return 1 as ::core::ffi::c_int;
    }
    flag = uqm_lua_checked!(lua_toboolean(luaState, 2 as ::core::ffi::c_int), 0);
    result = SetRaceAllied(
        RACE_ID(raceId as ::core::ffi::c_uint),
        BOOLEAN(flag as ::core::ffi::c_uint),
    );
    uqm_lua_checked!(lua_pushboolean(luaState, result.0 as ::core::ffi::c_int), 0);
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn luaUqm_state_race_setKnown(
    mut luaState: *mut lua_State,
) -> ::core::ffi::c_int {
    let mut raceId: COUNT = 0;
    let mut flag: ::core::ffi::c_int = 0;
    let mut result: BOOLEAN = BOOLEAN::FALSE;
    raceId = uqm_lua_checked!(testRaceId(luaState, 1 as ::core::ffi::c_int), 0);
    if raceId as ::core::ffi::c_int
        == -1 as ::core::ffi::c_int as COUNT as ::core::ffi::c_int
    {
        uqm_lua_checked!(lua_pushboolean(luaState, BOOLEAN::FALSE.0 as ::core::ffi::c_int), 0);
        return 1 as ::core::ffi::c_int;
    }
    flag = uqm_lua_checked!(lua_toboolean(luaState, 2 as ::core::ffi::c_int), 0);
    if flag == 0 as ::core::ffi::c_int {
        log_add(
            log_Level::log_Error,
            b"[script] Warning: luaUqm_state_race_setKnown(): setKnown(false) is not implemented.\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        uqm_lua_checked!(lua_pushboolean(luaState, BOOLEAN::FALSE.0 as ::core::ffi::c_int), 0);
        return 1 as ::core::ffi::c_int;
    }
    result = BOOLEAN(
        (StartSphereTracking(RACE_ID(raceId as ::core::ffi::c_uint))
            as ::core::ffi::c_int != 0 as ::core::ffi::c_int) as ::core::ffi::c_int
            as ::core::ffi::c_uint,
    );
    uqm_lua_checked!(lua_pushboolean(luaState, result.0 as ::core::ffi::c_int), 0);
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn luaUqm_state_sis_addCrew(
    mut luaState: *mut lua_State,
) -> ::core::ffi::c_int {
    let mut delta: ::core::ffi::c_int = 0;
    let mut oldCrew: COUNT = 0;
    let mut newCrew: COUNT = 0;
    delta = uqm_lua_checked!(luaL_checkinteger(luaState, 1 as ::core::ffi::c_int), 0) as ::core::ffi::c_int;
    oldCrew = GlobData.SIS_state.CrewEnlisted;
    uqm_lua_checked!(DeltaSISGauges(delta as SIZE, 0 as SDWORD, 0 as ::core::ffi::c_int), 0);
    newCrew = GlobData.SIS_state.CrewEnlisted;
    uqm_lua_checked!(lua_pushinteger(
        luaState,
        (newCrew as ::core::ffi::c_int - oldCrew as ::core::ffi::c_int) as lua_Integer,
    ), 0);
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn luaUqm_state_sis_addFuel(
    mut luaState: *mut lua_State,
) -> ::core::ffi::c_int {
    let mut delta: ::core::ffi::c_int = 0;
    let mut oldFuel: COUNT = 0;
    let mut newFuel: COUNT = 0;
    delta = uqm_lua_checked!(luaL_checkinteger(luaState, 1 as ::core::ffi::c_int), 0) as ::core::ffi::c_int;
    oldFuel = GlobData.SIS_state.FuelOnBoard as COUNT;
    uqm_lua_checked!(DeltaSISGauges(0 as SIZE, delta, 0 as ::core::ffi::c_int), 0);
    newFuel = GlobData.SIS_state.FuelOnBoard as COUNT;
    uqm_lua_checked!(lua_pushinteger(
        luaState,
        (newFuel as ::core::ffi::c_int - oldFuel as ::core::ffi::c_int) as lua_Integer,
    ), 0);
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn luaUqm_state_sis_addLanders(
    mut luaState: *mut lua_State,
) -> ::core::ffi::c_int {
    let mut delta: ::core::ffi::c_int = 0;
    let mut oldCount: ::core::ffi::c_int = 0;
    let mut newCount: ::core::ffi::c_int = 0;
    delta = uqm_lua_checked!(luaL_checkinteger(luaState, 1 as ::core::ffi::c_int), 0) as ::core::ffi::c_int;
    oldCount = GlobData.SIS_state.NumLanders as ::core::ffi::c_int;
    newCount = oldCount + delta;
    if newCount < 0 as ::core::ffi::c_int {
        newCount = 0 as ::core::ffi::c_int;
    } else if newCount > 10 as ::core::ffi::c_int {
        newCount = 10 as ::core::ffi::c_int;
    }
    if newCount != oldCount {
        GlobData.SIS_state.NumLanders = newCount as BYTE;
        uqm_lua_checked!(DrawLanders(), 0);
    }
    uqm_lua_checked!(lua_pushinteger(luaState, (newCount - oldCount) as lua_Integer), 0);
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn luaUqm_state_sis_addResUnits(
    mut luaState: *mut lua_State,
) -> ::core::ffi::c_int {
    let mut delta: ::core::ffi::c_int = 0;
    let mut oldResUnits: COUNT = 0;
    let mut newResUnits: COUNT = 0;
    delta = uqm_lua_checked!(luaL_checkinteger(luaState, 1 as ::core::ffi::c_int), 0) as ::core::ffi::c_int;
    oldResUnits = GlobData.SIS_state.ResUnits as COUNT;
    uqm_lua_checked!(DeltaSISGauges(0 as SIZE, delta, 0 as ::core::ffi::c_int), 0);
    newResUnits = GlobData.SIS_state.ResUnits as COUNT;
    uqm_lua_checked!(lua_pushinteger(
        luaState,
        (newResUnits as ::core::ffi::c_int - oldResUnits as ::core::ffi::c_int)
            as lua_Integer,
    ), 0);
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn luaUqm_state_sis_getCaptainName(
    mut luaState: *mut lua_State,
) -> ::core::ffi::c_int {
    uqm_lua_checked!(lua_pushstring(luaState, &raw mut GlobData.SIS_state.CommanderName as *mut UNICODE), 0);
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn luaUqm_state_sis_getCrew(
    mut luaState: *mut lua_State,
) -> ::core::ffi::c_int {
    uqm_lua_checked!(lua_pushinteger(luaState, GlobData.SIS_state.CrewEnlisted as lua_Integer), 0);
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn luaUqm_state_sis_getFuel(
    mut luaState: *mut lua_State,
) -> ::core::ffi::c_int {
    uqm_lua_checked!(lua_pushinteger(luaState, GlobData.SIS_state.FuelOnBoard as lua_Integer), 0);
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn luaUqm_state_sis_getResUnits(
    mut luaState: *mut lua_State,
) -> ::core::ffi::c_int {
    uqm_lua_checked!(lua_pushinteger(luaState, GlobData.SIS_state.ResUnits as lua_Integer), 0);
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn luaUqm_state_sis_getLanders(
    mut luaState: *mut lua_State,
) -> ::core::ffi::c_int {
    uqm_lua_checked!(lua_pushinteger(luaState, GlobData.SIS_state.NumLanders as lua_Integer), 0);
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn luaUqm_state_sis_getShipName(
    mut luaState: *mut lua_State,
) -> ::core::ffi::c_int {
    uqm_lua_checked!(lua_pushstring(luaState, &raw mut GlobData.SIS_state.ShipName as *mut UNICODE), 0);
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn luaUqm_state_prop_get(
    mut luaState: *mut lua_State,
) -> ::core::ffi::c_int {
    uqm_lua_checked!(luaL_checktype(luaState, 1 as ::core::ffi::c_int, 4 as ::core::ffi::c_int), 0);
    uqm_lua_checked!(luaUqm_getProp(luaState, 1 as ::core::ffi::c_int), 0);
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn luaUqm_state_prop_set(
    mut luaState: *mut lua_State,
) -> ::core::ffi::c_int {
    uqm_lua_checked!(luaL_checktype(luaState, 1 as ::core::ffi::c_int, 4 as ::core::ffi::c_int), 0);
    uqm_lua_checked!(luaUqm_checkPropValueType(
        luaState,
        b"state.prop.set\0".as_ptr() as *const ::core::ffi::c_char,
        2 as ::core::ffi::c_int,
    ), 0);
    uqm_lua_checked!(luaUqm_setProp(luaState, 1 as ::core::ffi::c_int, 2 as ::core::ffi::c_int), 0);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn luaUqm_state_misc_alignText(
    mut luaState: *mut lua_State,
) -> ::core::ffi::c_int {
    luaUqm_delta._int = uqm_lua_checked!(luaL_checkinteger(luaState, 1 as ::core::ffi::c_int), 0)
        as ::core::ffi::c_int;
    uqm_lua_checked!(lua_pushstring(luaState, b"\0".as_ptr() as *const ::core::ffi::c_char), 0);
    return 1 as ::core::ffi::c_int;
}
