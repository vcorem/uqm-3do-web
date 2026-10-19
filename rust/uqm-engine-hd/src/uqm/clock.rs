//Copyright Paul Reiche, Fred Ford. 1992-2002

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

// Rust translation of src/uqm/clock.c.
extern "C" {
    pub type frame_desc;
    fn __assert_fail(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    ) -> !;
    fn EventHandler(selector: BYTE);
    fn CreateMutex_Core(name: *const ::core::ffi::c_char, syncClass: DWORD) -> Mutex;
    fn DestroyMutex(sem: Mutex);
    fn LockMutex(sem: Mutex);
    fn UnlockMutex(sem: Mutex);
    fn AllocLink(pq: *mut QUEUE) -> HLINK;
    fn FreeLink(pq: *mut QUEUE, hLink: HLINK);
    fn InitQueue(pq: *mut QUEUE, num_elements: COUNT, size: OBJ_SIZE) -> BOOLEAN;
    fn UninitQueue(pq: *mut QUEUE) -> BOOLEAN;
    fn InsertQueue(pq: *mut QUEUE, hLink: HLINK, hRefLink: HLINK);
    fn RemoveQueue(pq: *mut QUEUE, hLink: HLINK);
    static mut timeDilationScale: ::core::ffi::c_int;
    fn DrawStatusMessage(pStr: *const UNICODE);
    static mut GlobData: GLOBDATA;
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed {
    pub const ARILOU_ENTRANCE_EVENT: Self = Self(0);
    pub const ARILOU_EXIT_EVENT: Self = Self(1);
    pub const HYPERSPACE_ENCOUNTER_EVENT: Self = Self(2);
    pub const KOHR_AH_VICTORIOUS_EVENT: Self = Self(3);
    pub const ADVANCE_PKUNK_MISSION: Self = Self(4);
    pub const ADVANCE_THRADD_MISSION: Self = Self(5);
    pub const ZOQFOT_DISTRESS_EVENT: Self = Self(6);
    pub const ZOQFOT_DEATH_EVENT: Self = Self(7);
    pub const SHOFIXTI_RETURN_EVENT: Self = Self(8);
    pub const ADVANCE_UTWIG_SUPOX_MISSION: Self = Self(9);
    pub const KOHR_AH_GENOCIDE_EVENT: Self = Self(10);
    pub const SPATHI_SHIELD_EVENT: Self = Self(11);
    pub const ADVANCE_ILWRATH_MISSION: Self = Self(12);
    pub const ADVANCE_MYCON_MISSION: Self = Self(13);
    pub const ARILOU_UMGAH_CHECK: Self = Self(14);
    pub const YEHAT_REBEL_EVENT: Self = Self(15);
    pub const SLYLANDRO_RAMP_UP: Self = Self(16);
    pub const SLYLANDRO_RAMP_DOWN: Self = Self(17);
    pub const NUM_EVENTS: Self = Self(18);
}
pub type Mutex = *mut ::core::ffi::c_void;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_0 {
    pub const SYNC_CLASS_TOPLEVEL: Self = Self(1);
    pub const SYNC_CLASS_AUDIO: Self = Self(2);
    pub const SYNC_CLASS_VIDEO: Self = Self(4);
    pub const SYNC_CLASS_RESOURCE: Self = Self(8);
}
pub type QUEUE_HANDLE = *mut ::core::ffi::c_void;
pub type OBJ_SIZE = UWORD;
pub type HLINK = QUEUE_HANDLE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct link {
    pub pred: HLINK,
    pub succ: HLINK,
}
pub type LINK = link;
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
pub type HEVENT = HLINK;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event {
    pub pred: HEVENT,
    pub succ: HEVENT,
    pub day_index: BYTE,
    pub month_index: BYTE,
    pub year_index: COUNT,
    pub func_index: BYTE,
}
pub type EVENT = event;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct EVENT_TYPE(pub ::core::ffi::c_uint);
impl EVENT_TYPE {
    pub const ABSOLUTE_EVENT: Self = Self(0);
    pub const RELATIVE_EVENT: Self = Self(1);
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
pub type ACTIVITY = UWORD;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GLOBDATA {
    pub SIS_state: SIS_STATE,
    pub Game_state: GAME_STATE,
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
#[inline]
unsafe extern "C" fn LockLink(mut pq: *const QUEUE, mut h: HLINK) -> *mut LINK {
    if !h.is_null() {
        (!(*pq).pq_tab.is_null() && h as *mut BYTE >= (*pq).pq_tab
            && (h as *mut BYTE)
                < (*pq)
                    .pq_tab
                    .offset(
                        ((*pq).object_size as ::core::ffi::c_int
                            * (*pq).num_objects as ::core::ffi::c_int) as isize,
                    )
            || {
                __assert_fail(
                    b"pq->pq_tab && (BYTE*)h >= pq->pq_tab && (BYTE*)h < pq->pq_tab + pq->object_size * pq->num_objects\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    b"src/uqm/displist.h\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    77 as ::core::ffi::c_int,
                    b"LockLink\0".as_ptr() as *const ::core::ffi::c_char,
                );
                0 as ::core::ffi::c_int != 0
            }) as ::core::ffi::c_int;
    }
    return h as *mut LINK;
}
#[inline]
unsafe extern "C" fn UnlockLink(mut pq: *const QUEUE, mut h: HLINK) {
    if !h.is_null() {
        (!(*pq).pq_tab.is_null() && h as *mut BYTE >= (*pq).pq_tab
            && (h as *mut BYTE)
                < (*pq)
                    .pq_tab
                    .offset(
                        ((*pq).object_size as ::core::ffi::c_int
                            * (*pq).num_objects as ::core::ffi::c_int) as isize,
                    )
            || {
                __assert_fail(
                    b"pq->pq_tab && (BYTE*)h >= pq->pq_tab && (BYTE*)h < pq->pq_tab + pq->object_size * pq->num_objects\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    b"src/uqm/displist.h\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    88 as ::core::ffi::c_int,
                    b"UnlockLink\0".as_ptr() as *const ::core::ffi::c_char,
                );
                0 as ::core::ffi::c_int != 0
            }) as ::core::ffi::c_int;
    }
}
static mut clock_mutex: Mutex = ::core::ptr::null_mut::<::core::ffi::c_void>();
unsafe extern "C" fn IsLeapYear(mut year: COUNT) -> BOOLEAN {
    return BOOLEAN(
        (year as ::core::ffi::c_int & 3 as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            && (year as ::core::ffi::c_int % 100 as ::core::ffi::c_int
                != 0 as ::core::ffi::c_int
                || year as ::core::ffi::c_int % 400 as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int)) as ::core::ffi::c_int
            as ::core::ffi::c_uint,
    );
}
unsafe extern "C" fn DaysInMonth(mut month: COUNT, mut year: COUNT) -> BYTE {
    static mut days_in_month: [BYTE; 12] = [
        31 as BYTE,
        28 as BYTE,
        31 as BYTE,
        30 as BYTE,
        31 as BYTE,
        30 as BYTE,
        31 as BYTE,
        31 as BYTE,
        30 as BYTE,
        31 as BYTE,
        30 as BYTE,
        31 as BYTE,
    ];
    if month as ::core::ffi::c_int == 2 as ::core::ffi::c_int && IsLeapYear(year).0 != 0
    {
        return 29 as BYTE;
    }
    return days_in_month[(month as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
        as usize];
}
unsafe extern "C" fn nextClockDay() {
    GlobData.Game_state.GameClock.day_index = GlobData
        .Game_state
        .GameClock
        .day_index
        .wrapping_add(1);
    if GlobData.Game_state.GameClock.day_index as ::core::ffi::c_int
        > DaysInMonth(
            GlobData.Game_state.GameClock.month_index as COUNT,
            GlobData.Game_state.GameClock.year_index,
        ) as ::core::ffi::c_int
    {
        GlobData.Game_state.GameClock.day_index = 1 as BYTE;
        GlobData.Game_state.GameClock.month_index = GlobData
            .Game_state
            .GameClock
            .month_index
            .wrapping_add(1);
        if GlobData.Game_state.GameClock.month_index as ::core::ffi::c_int
            > 12 as ::core::ffi::c_int
        {
            GlobData.Game_state.GameClock.month_index = 1 as BYTE;
            GlobData.Game_state.GameClock.year_index = GlobData
                .Game_state
                .GameClock
                .year_index
                .wrapping_add(1);
        }
    }
    uqm_lua_checked!(DrawStatusMessage(::core::ptr::null::<UNICODE>()), ());
}
#[no_mangle]
pub unsafe extern "C" fn daysElapsed() -> ::core::ffi::c_float {
    let mut days: ::core::ffi::c_float = 0 as ::core::ffi::c_int as ::core::ffi::c_float;
    let mut index: COUNT = 0;
    index = 2155 as COUNT;
    while (index as ::core::ffi::c_int)
        < GlobData.Game_state.GameClock.year_index as ::core::ffi::c_int
    {
        days += 365 as ::core::ffi::c_int as ::core::ffi::c_float;
        if IsLeapYear(index).0 != 0 {
            days += 1.;
        }
        index = index.wrapping_add(1);
    }
    if GlobData.Game_state.GameClock.month_index as ::core::ffi::c_int
        == 1 as ::core::ffi::c_int
    {
        days = days - 31 as ::core::ffi::c_int as ::core::ffi::c_float;
    }
    index = 2 as COUNT;
    while (index as ::core::ffi::c_int)
        < GlobData.Game_state.GameClock.month_index as ::core::ffi::c_int
    {
        days
            += DaysInMonth(index, GlobData.Game_state.GameClock.year_index)
                as ::core::ffi::c_int as ::core::ffi::c_float;
        index = index.wrapping_add(1);
    }
    days = days
        + GlobData.Game_state.GameClock.day_index as ::core::ffi::c_int
            as ::core::ffi::c_float - 17 as ::core::ffi::c_int as ::core::ffi::c_float;
    days = days
        + (GlobData.Game_state.GameClock.day_in_ticks as ::core::ffi::c_int
            - GlobData.Game_state.GameClock.tick_count as ::core::ffi::c_int)
            as ::core::ffi::c_float
            / GlobData.Game_state.GameClock.day_in_ticks as ::core::ffi::c_float;
    return days;
}
unsafe extern "C" fn processClockDayEvents() {
    let mut hEvent: HEVENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
    loop {
        hEvent = GlobData.Game_state.GameClock.event_q.head as HEVENT;
        if hEvent.is_null() {
            break;
        }
        let mut EventPtr: *mut EVENT = ::core::ptr::null_mut::<EVENT>();
        EventPtr = LockLink(&raw mut GlobData.Game_state.GameClock.event_q, hEvent)
            as *mut EVENT;
        if GlobData.Game_state.GameClock.day_index as ::core::ffi::c_int
            != (*EventPtr).day_index as ::core::ffi::c_int
            || GlobData.Game_state.GameClock.month_index as ::core::ffi::c_int
                != (*EventPtr).month_index as ::core::ffi::c_int
            || GlobData.Game_state.GameClock.year_index as ::core::ffi::c_int
                != (*EventPtr).year_index as ::core::ffi::c_int
        {
            UnlockLink(&raw mut GlobData.Game_state.GameClock.event_q, hEvent);
            break;
        } else {
            RemoveQueue(&raw mut GlobData.Game_state.GameClock.event_q, hEvent);
            uqm_lua_checked!(EventHandler((*EventPtr).func_index), ());
            UnlockLink(&raw mut GlobData.Game_state.GameClock.event_q, hEvent);
            FreeLink(&raw mut GlobData.Game_state.GameClock.event_q, hEvent);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn InitGameClock() -> BOOLEAN {
    if InitQueue(
            &raw mut GlobData.Game_state.GameClock.event_q,
            C2Rust_Unnamed::NUM_EVENTS.0 as ::core::ffi::c_int as COUNT,
            ::core::mem::size_of::<EVENT>() as OBJ_SIZE,
        )
        .0 == 0
    {
        return BOOLEAN::FALSE;
    }
    clock_mutex = CreateMutex_Core(
        b"Clock Mutex\0".as_ptr() as *const ::core::ffi::c_char,
        C2Rust_Unnamed_0::SYNC_CLASS_TOPLEVEL.0 as ::core::ffi::c_int as DWORD,
    );
    GlobData.Game_state.GameClock.month_index = 2 as BYTE;
    GlobData.Game_state.GameClock.day_index = 17 as BYTE;
    GlobData.Game_state.GameClock.year_index = 2155 as COUNT;
    GlobData.Game_state.GameClock.tick_count = 0 as SIZE;
    GlobData.Game_state.GameClock.day_in_ticks = 0 as SIZE;
    return BOOLEAN::TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn UninitGameClock() -> BOOLEAN {
    DestroyMutex(clock_mutex);
    clock_mutex = ::core::ptr::null_mut::<::core::ffi::c_void>() as Mutex;
    UninitQueue(&raw mut GlobData.Game_state.GameClock.event_q);
    return BOOLEAN::TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn LockGameClock() {
    if !clock_mutex.is_null() {
        LockMutex(clock_mutex);
    }
}
#[no_mangle]
pub unsafe extern "C" fn UnlockGameClock() {
    if !clock_mutex.is_null() {
        UnlockMutex(clock_mutex);
    }
}
#[no_mangle]
pub unsafe extern "C" fn GameClockRunning() -> BOOLEAN {
    let mut day_in_ticks: SIZE = 0;
    if clock_mutex.is_null() {
        return BOOLEAN::FALSE;
    }
    LockMutex(clock_mutex);
    day_in_ticks = GlobData.Game_state.GameClock.day_in_ticks;
    UnlockMutex(clock_mutex);
    return BOOLEAN(
        (day_in_ticks as ::core::ffi::c_int != 0 as ::core::ffi::c_int)
            as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
}
#[no_mangle]
pub unsafe extern "C" fn SetGameClockRate(mut seconds_per_day: COUNT) {
    let mut new_day_in_ticks: SIZE = 0;
    let mut new_tick_count: SIZE = 0;
    new_day_in_ticks = (seconds_per_day as ::core::ffi::c_int * 24 as ::core::ffi::c_int)
        as SIZE;
    match timeDilationScale {
        1 => {
            new_day_in_ticks = (new_day_in_ticks as ::core::ffi::c_int
                * 6 as ::core::ffi::c_int) as SIZE;
        }
        2 => {
            new_day_in_ticks = (new_day_in_ticks as ::core::ffi::c_int
                / 5 as ::core::ffi::c_int) as SIZE;
        }
        0 | _ => {}
    }
    if GlobData.Game_state.GameClock.day_in_ticks as ::core::ffi::c_int
        == 0 as ::core::ffi::c_int
    {
        new_tick_count = new_day_in_ticks;
    } else if GlobData.Game_state.GameClock.tick_count as ::core::ffi::c_int
        <= 0 as ::core::ffi::c_int
    {
        new_tick_count = 0 as SIZE;
    } else {
        new_tick_count = (GlobData.Game_state.GameClock.tick_count as DWORD)
            .wrapping_mul(new_day_in_ticks as DWORD)
            .wrapping_div(GlobData.Game_state.GameClock.day_in_ticks as DWORD) as SIZE;
        if new_tick_count as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            new_tick_count = 1 as SIZE;
        }
    }
    GlobData.Game_state.GameClock.day_in_ticks = new_day_in_ticks;
    GlobData.Game_state.GameClock.tick_count = new_tick_count;
}
#[no_mangle]
pub unsafe extern "C" fn ValidateEvent(
    mut r#type: EVENT_TYPE,
    mut pmonth_index: *mut COUNT,
    mut pday_index: *mut COUNT,
    mut pyear_index: *mut COUNT,
) -> BOOLEAN {
    let mut month_index: COUNT = 0;
    let mut day_index: COUNT = 0;
    let mut year_index: COUNT = 0;
    month_index = *pmonth_index;
    day_index = *pday_index;
    year_index = *pyear_index;
    if r#type.0 == EVENT_TYPE::RELATIVE_EVENT.0 {
        month_index = (month_index as ::core::ffi::c_int
            + (GlobData.Game_state.GameClock.month_index as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int)) as COUNT;
        year_index = (year_index as ::core::ffi::c_int
            + (GlobData.Game_state.GameClock.year_index as ::core::ffi::c_int
                + month_index as ::core::ffi::c_int / 12 as ::core::ffi::c_int))
            as COUNT;
        month_index = (month_index as ::core::ffi::c_int % 12 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int) as COUNT;
        day_index = (day_index as ::core::ffi::c_int
            + GlobData.Game_state.GameClock.day_index as ::core::ffi::c_int) as COUNT;
        while day_index as ::core::ffi::c_int
            > DaysInMonth(month_index, year_index) as ::core::ffi::c_int
        {
            day_index = (day_index as ::core::ffi::c_int
                - DaysInMonth(month_index, year_index) as ::core::ffi::c_int) as COUNT;
            month_index = month_index.wrapping_add(1);
            if month_index as ::core::ffi::c_int > 12 as ::core::ffi::c_int {
                month_index = 1 as COUNT;
                year_index = year_index.wrapping_add(1);
            }
        }
        *pmonth_index = month_index;
        *pday_index = day_index;
        *pyear_index = year_index;
    }
    return BOOLEAN(
        !((year_index as ::core::ffi::c_int)
            < GlobData.Game_state.GameClock.year_index as ::core::ffi::c_int
            || year_index as ::core::ffi::c_int
                == GlobData.Game_state.GameClock.year_index as ::core::ffi::c_int
                && ((month_index as ::core::ffi::c_int)
                    < GlobData.Game_state.GameClock.month_index as ::core::ffi::c_int
                    || month_index as ::core::ffi::c_int
                        == GlobData.Game_state.GameClock.month_index
                            as ::core::ffi::c_int
                        && (day_index as ::core::ffi::c_int)
                            < GlobData.Game_state.GameClock.day_index
                                as ::core::ffi::c_int)) as ::core::ffi::c_int
            as ::core::ffi::c_uint,
    );
}
#[no_mangle]
pub unsafe extern "C" fn AddEvent(
    mut r#type: EVENT_TYPE,
    mut month_index: COUNT,
    mut day_index: COUNT,
    mut year_index: COUNT,
    mut func_index: BYTE,
) -> HEVENT {
    let mut hNewEvent: HEVENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if r#type.0 == EVENT_TYPE::RELATIVE_EVENT.0
        && month_index as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        && day_index as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        && year_index as ::core::ffi::c_int == 0 as ::core::ffi::c_int
    {
        uqm_lua_checked!(EventHandler(func_index), ::core::ptr::null_mut());
    } else if ValidateEvent(
            r#type,
            &raw mut month_index,
            &raw mut day_index,
            &raw mut year_index,
        )
        .0 != 0
        && {
            hNewEvent = AllocLink(&raw mut GlobData.Game_state.GameClock.event_q)
                as HEVENT;
            !hNewEvent.is_null()
        }
    {
        let mut EventPtr: *mut EVENT = ::core::ptr::null_mut::<EVENT>();
        EventPtr = LockLink(&raw mut GlobData.Game_state.GameClock.event_q, hNewEvent)
            as *mut EVENT;
        (*EventPtr).day_index = day_index as BYTE;
        (*EventPtr).month_index = month_index as BYTE;
        (*EventPtr).year_index = year_index;
        (*EventPtr).func_index = func_index;
        UnlockLink(&raw mut GlobData.Game_state.GameClock.event_q, hNewEvent);
        let mut hEvent: HEVENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
        let mut hSuccEvent: HEVENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
        hEvent = GlobData.Game_state.GameClock.event_q.head as HEVENT;
        while !hEvent.is_null() {
            EventPtr = LockLink(&raw mut GlobData.Game_state.GameClock.event_q, hEvent)
                as *mut EVENT;
            if (year_index as ::core::ffi::c_int)
                < (*EventPtr).year_index as ::core::ffi::c_int
                || year_index as ::core::ffi::c_int
                    == (*EventPtr).year_index as ::core::ffi::c_int
                    && ((month_index as ::core::ffi::c_int)
                        < (*EventPtr).month_index as ::core::ffi::c_int
                        || month_index as ::core::ffi::c_int
                            == (*EventPtr).month_index as ::core::ffi::c_int
                            && (day_index as ::core::ffi::c_int)
                                < (*EventPtr).day_index as ::core::ffi::c_int)
            {
                UnlockLink(&raw mut GlobData.Game_state.GameClock.event_q, hEvent);
                break;
            } else {
                hSuccEvent = (*EventPtr).succ;
                UnlockLink(&raw mut GlobData.Game_state.GameClock.event_q, hEvent);
                hEvent = hSuccEvent;
            }
        }
        InsertQueue(&raw mut GlobData.Game_state.GameClock.event_q, hNewEvent, hEvent);
        return hNewEvent;
    }
    return ::core::ptr::null_mut::<::core::ffi::c_void>();
}
#[no_mangle]
pub unsafe extern "C" fn GameClockTick() {
    LockMutex(clock_mutex);
    GlobData.Game_state.GameClock.tick_count -= 1;
    if GlobData.Game_state.GameClock.tick_count as ::core::ffi::c_int
        <= 0 as ::core::ffi::c_int
    {
        GlobData.Game_state.GameClock.tick_count = GlobData
            .Game_state
            .GameClock
            .day_in_ticks;
        if GlobData.Game_state.GameClock.day_in_ticks as ::core::ffi::c_int
            > 0 as ::core::ffi::c_int
        {
            uqm_lua_checked!(nextClockDay(), ());
            uqm_lua_checked!(processClockDayEvents(), ());
        }
    }
    UnlockMutex(clock_mutex);
}
#[no_mangle]
pub unsafe extern "C" fn MoveGameClockDays(mut days: COUNT) {
    while days as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
        uqm_lua_checked!(nextClockDay(), ());
        uqm_lua_checked!(processClockDayEvents(), ());
        days = days.wrapping_sub(1);
    }
    GlobData.Game_state.GameClock.tick_count = GlobData
        .Game_state
        .GameClock
        .day_in_ticks;
}
