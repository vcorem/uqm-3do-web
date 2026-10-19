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

// Rust translation of src/uqm/supermelee/netplay/checksum.c.
extern "C" {
    pub type frame_desc;
    pub type NetConnection;
    fn __assert_fail(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    );
    static mut disp_q: QUEUE;
    fn NetConnection_getChecksumBuffer(conn: *mut NetConnection) -> *mut ChecksumBuffer;
    fn ChecksumBuffer_init(cb: *mut ChecksumBuffer, delay: size_t, interval: size_t);
    fn ChecksumBuffer_uninit(cb: *mut ChecksumBuffer);
    fn ChecksumBuffer_addChecksum(
        cb: *mut ChecksumBuffer,
        frameNr: BattleFrameCounter,
        checksum: Checksum,
    ) -> bool;
    fn ChecksumBuffer_getChecksum(
        cb: *mut ChecksumBuffer,
        frameNr: BattleFrameCounter,
        result: *mut Checksum,
    ) -> bool;
    static mut battleFrameCount: BattleFrameCounter;
    fn emscripten_asm_const_int_sync_on_main_thread(
        code: *const ::core::ffi::c_char,
        arg_sigs: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn crc_processUint8(state: *mut crc_State, val: uint8);
    fn crc_processUint16(state: *mut crc_State, val: uint16);
    fn crc_processUint32(state: *mut crc_State, val: uint32);
    fn getBattleInputDelay() -> size_t;
    fn TFB_SeedRandom(seed: DWORD) -> DWORD;
    static mut netConnections: [*mut NetConnection; 0];
    fn log_add(_: log_Level, fmt: *const ::core::ffi::c_char, ...);
}
pub type uint8 = ::core::ffi::c_uchar;
pub type uint16 = ::core::ffi::c_ushort;
pub type sint16 = ::core::ffi::c_short;
pub type uint32 = ::core::ffi::c_uint;
pub type Checksum = uint32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct crc_State {
    pub crc: uint32,
}
pub type size_t = usize;
pub type BYTE = uint8;
pub type UWORD = uint16;
pub type SWORD = sint16;
pub type DWORD = uint32;
pub type COUNT = UWORD;
pub type SIZE = SWORD;
pub type QUEUE_HANDLE = *mut ::core::ffi::c_void;
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
pub type FRAME_DESC = frame_desc;
pub type FRAME = *mut FRAME_DESC;
pub type TIME_VALUE = UWORD;
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
pub struct INTERSECT_CONTROL {
    pub last_time_val: TIME_VALUE,
    pub EndPoint: POINT,
    pub IntersectStamp: STAMP,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct velocity_desc {
    pub TravelAngle: COUNT,
    pub vector: EXTENT,
    pub fract: EXTENT,
    pub error: EXTENT,
    pub incr: EXTENT,
}
pub type VELOCITY_DESC = velocity_desc;
pub type HELEMENT = HLINK;
pub type ELEMENT_FLAGS = UWORD;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct state {
    pub location: POINT,
    pub image: C2Rust_Unnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed {
    pub frame: FRAME,
    pub farray: *mut FRAME,
}
pub type STATE = state;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct element {
    pub pred: HELEMENT,
    pub succ: HELEMENT,
    pub preprocess_func: Option<ElementProcessFunc>,
    pub postprocess_func: Option<ElementProcessFunc>,
    pub collision_func: Option<ElementCollisionFunc>,
    pub death_func: Option<ElementProcessFunc>,
    pub playerNr: SIZE,
    pub state_flags: ELEMENT_FLAGS,
    pub c2rust_unnamed: C2Rust_Unnamed_4,
    pub c2rust_unnamed_0: C2Rust_Unnamed_3,
    pub c2rust_unnamed_1: C2Rust_Unnamed_2,
    pub c2rust_unnamed_2: C2Rust_Unnamed_1,
    pub c2rust_unnamed_3: C2Rust_Unnamed_0,
    pub colorCycleIndex: BYTE,
    pub velocity: VELOCITY_DESC,
    pub IntersectControl: INTERSECT_CONTROL,
    pub PrimIndex: COUNT,
    pub current: STATE,
    pub next: STATE,
    pub pParent: *mut ::core::ffi::c_void,
    pub hTarget: HELEMENT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_0 {
    pub thrust_wait: BYTE,
    pub blast_offset: BYTE,
    pub next_turn: BYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_1 {
    pub turn_wait: BYTE,
    pub sys_loc: BYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_2 {
    pub mass_points: BYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_3 {
    pub crew_level: COUNT,
    pub hit_points: COUNT,
    pub facing: COUNT,
    pub cycle: COUNT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_4 {
    pub life_span: COUNT,
    pub scan_node: COUNT,
}
pub type ElementProcessFunc = unsafe extern "C" fn(*mut ELEMENT) -> ();
pub type ELEMENT = element;
pub type ElementCollisionFunc = unsafe extern "C" fn(
    *mut ELEMENT,
    *mut POINT,
    *mut ELEMENT,
    *mut POINT,
) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ChecksumEntry {
    pub checksum: Checksum,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ChecksumBuffer {
    pub checksums: *mut ChecksumEntry,
    pub maxSize: size_t,
    pub interval: size_t,
}
pub type BattleFrameCounter = DWORD;
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
#[inline]
unsafe extern "C" fn crc_processELEMENT_FLAGS(
    mut state: *mut crc_State,
    mut val: ELEMENT_FLAGS,
) {
    crc_processUint16(state, val);
}
#[inline]
unsafe extern "C" fn crc_processCOUNT(mut state: *mut crc_State, mut val: COUNT) {
    crc_processUint16(state, val);
}
#[inline]
unsafe extern "C" fn crc_processBYTE(mut state: *mut crc_State, mut val: BYTE) {
    crc_processUint8(state, val);
}
#[inline]
unsafe extern "C" fn crc_processDWORD(mut state: *mut crc_State, mut val: DWORD) {
    crc_processUint32(state, val);
}
#[inline]
unsafe extern "C" fn crc_processCOORD(mut state: *mut crc_State, mut val: COORD) {
    crc_processUint16(state, val as uint16);
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
                    b"src/uqm/supermelee/netplay/../../displist.h\0"
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
                    b"src/uqm/supermelee/netplay/../../displist.h\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    88 as ::core::ffi::c_int,
                    b"UnlockLink\0".as_ptr() as *const ::core::ffi::c_char,
                );
                0 as ::core::ffi::c_int != 0
            }) as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub static mut localChecksumBuffer: ChecksumBuffer = ChecksumBuffer {
    checksums: ::core::ptr::null_mut::<ChecksumEntry>(),
    maxSize: 0,
    interval: 0,
};
#[no_mangle]
pub unsafe extern "C" fn crc_processEXTENT(
    mut state: *mut crc_State,
    mut val: *const EXTENT,
) {
    crc_processCOORD(state, (*val).width);
    crc_processCOORD(state, (*val).height);
}
#[no_mangle]
pub unsafe extern "C" fn crc_processVELOCITY_DESC(
    mut state: *mut crc_State,
    mut val: *const VELOCITY_DESC,
) {
    crc_processCOUNT(state, (*val).TravelAngle);
    crc_processEXTENT(state, &raw const (*val).vector);
    crc_processEXTENT(state, &raw const (*val).fract);
    crc_processEXTENT(state, &raw const (*val).error);
    crc_processEXTENT(state, &raw const (*val).incr);
}
#[no_mangle]
pub unsafe extern "C" fn crc_processPOINT(
    mut state: *mut crc_State,
    mut val: *const POINT,
) {
    crc_processCOORD(state, (*val).x);
    crc_processCOORD(state, (*val).y);
}
#[no_mangle]
pub unsafe extern "C" fn crc_processSTATE(
    mut state: *mut crc_State,
    mut val: *const STATE,
) {
    crc_processPOINT(state, &raw const (*val).location);
}
#[no_mangle]
pub unsafe extern "C" fn crc_processELEMENT(
    mut state: *mut crc_State,
    mut val: *const ELEMENT,
) {
    if (*val).state_flags as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 15 as ::core::ffi::c_int == 0
    {
        crc_processELEMENT_FLAGS(state, (*val).state_flags);
        crc_processCOUNT(state, (*val).c2rust_unnamed.life_span);
        crc_processCOUNT(state, (*val).c2rust_unnamed_0.crew_level);
        crc_processBYTE(state, (*val).c2rust_unnamed_1.mass_points);
        crc_processBYTE(state, (*val).c2rust_unnamed_2.turn_wait);
        crc_processBYTE(state, (*val).c2rust_unnamed_3.thrust_wait);
        crc_processVELOCITY_DESC(state, &raw const (*val).velocity);
        crc_processSTATE(state, &raw const (*val).current);
        crc_processSTATE(state, &raw const (*val).next);
    }
}
#[no_mangle]
pub unsafe extern "C" fn crc_processDispQueue(mut state: *mut crc_State) {
    let mut element: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut nextElement: HELEMENT = ::core::ptr::null_mut::<::core::ffi::c_void>();
    element = disp_q.head as HELEMENT;
    while !element.is_null() {
        let mut elementPtr: *mut ELEMENT = ::core::ptr::null_mut::<ELEMENT>();
        elementPtr = LockLink(&raw mut disp_q, element) as *mut ELEMENT;
        crc_processELEMENT(state, elementPtr);
        nextElement = (*elementPtr).succ;
        UnlockLink(&raw mut disp_q, element);
        element = nextElement;
    }
}
#[no_mangle]
pub unsafe extern "C" fn crc_processRNG(mut state: *mut crc_State) {
    let mut seed: DWORD = 0;
    seed = TFB_SeedRandom(0 as DWORD);
    crc_processDWORD(state, seed);
    TFB_SeedRandom(seed);
}
#[no_mangle]
pub unsafe extern "C" fn crc_processState(mut state: *mut crc_State) {
    crc_processRNG(state);
    crc_processDispQueue(state);
}
#[no_mangle]
pub unsafe extern "C" fn initChecksumBuffers() {
    let mut player: size_t = 0;
    player = 0 as size_t;
    while player < 2 as size_t {
        let mut conn: *mut NetConnection = ::core::ptr::null_mut::<NetConnection>();
        let mut cb: *mut ChecksumBuffer = ::core::ptr::null_mut::<ChecksumBuffer>();
        conn = *(&raw mut netConnections as *mut *mut NetConnection)
            .offset(player as isize) as *mut NetConnection;
        if !conn.is_null() {
            cb = NetConnection_getChecksumBuffer(conn);
            ChecksumBuffer_init(cb, getBattleInputDelay(), 1 as size_t);
        }
        player = player.wrapping_add(1);
    }
    ChecksumBuffer_init(
        &raw mut localChecksumBuffer,
        getBattleInputDelay(),
        1 as size_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn uninitChecksumBuffers() {
    let mut player: size_t = 0;
    player = 0 as size_t;
    while player < 2 as size_t {
        let mut conn: *mut NetConnection = ::core::ptr::null_mut::<NetConnection>();
        let mut cb: *mut ChecksumBuffer = ::core::ptr::null_mut::<ChecksumBuffer>();
        conn = *(&raw mut netConnections as *mut *mut NetConnection)
            .offset(player as isize) as *mut NetConnection;
        if !conn.is_null() {
            cb = NetConnection_getChecksumBuffer(conn);
            ChecksumBuffer_uninit(cb);
        }
        player = player.wrapping_add(1);
    }
    ChecksumBuffer_uninit(&raw mut localChecksumBuffer);
}
#[no_mangle]
pub unsafe extern "C" fn addLocalChecksum(
    mut frameNr: BattleFrameCounter,
    mut checksum: Checksum,
) {
    (frameNr == battleFrameCount
        || {
            __assert_fail(
                b"frameNr == battleFrameCount\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/uqm/supermelee/netplay/checksum.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                245 as ::core::ffi::c_int,
                b"addLocalChecksum\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    ChecksumBuffer_addChecksum(&raw mut localChecksumBuffer, frameNr, checksum);
}
#[no_mangle]
pub unsafe extern "C" fn addRemoteChecksum(
    mut conn: *mut NetConnection,
    mut frameNr: BattleFrameCounter,
    mut checksum: Checksum,
) {
    let mut cb: *mut ChecksumBuffer = ::core::ptr::null_mut::<ChecksumBuffer>();
    (frameNr as size_t
        <= (battleFrameCount as size_t)
            .wrapping_add(getBattleInputDelay())
            .wrapping_add(1 as size_t)
        || {
            __assert_fail(
                b"frameNr <= battleFrameCount + getBattleInputDelay() + 1\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/uqm/supermelee/netplay/checksum.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                255 as ::core::ffi::c_int,
                b"addRemoteChecksum\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    ((frameNr as size_t).wrapping_add(getBattleInputDelay())
        >= battleFrameCount as size_t
        || {
            __assert_fail(
                b"frameNr + getBattleInputDelay() >= battleFrameCount\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"src/uqm/supermelee/netplay/checksum.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                256 as ::core::ffi::c_int,
                b"addRemoteChecksum\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    cb = NetConnection_getChecksumBuffer(conn);
    ChecksumBuffer_addChecksum(cb, frameNr, checksum);
}
#[no_mangle]
pub unsafe extern "C" fn verifyChecksums(mut frameNr: BattleFrameCounter) -> bool {
    let mut localChecksum: Checksum = 0;
    let mut player: size_t = 0;
    if !ChecksumBuffer_getChecksum(
        &raw mut localChecksumBuffer,
        frameNr,
        &raw mut localChecksum,
    ) {
        return false;
    }
    player = 0 as size_t;
    while player < 2 as size_t {
        let mut conn: *mut NetConnection = ::core::ptr::null_mut::<NetConnection>();
        let mut cb: *mut ChecksumBuffer = ::core::ptr::null_mut::<ChecksumBuffer>();
        let mut remoteChecksum: Checksum = 0;
        conn = *(&raw mut netConnections as *mut *mut NetConnection)
            .offset(player as isize) as *mut NetConnection;
        if !conn.is_null() {
            cb = NetConnection_getChecksumBuffer(conn);
            if !ChecksumBuffer_getChecksum(cb, frameNr, &raw mut remoteChecksum) {
                return false;
            }
            if localChecksum != remoteChecksum {
                log_add(
                    log_Level::log_Error,
                    b"Network connections have gone out of sync at frame %u (local %08x, remote %08x).\n\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    frameNr,
                    localChecksum,
                    remoteChecksum,
                );
                WebTransport_checksum(frameNr, 0 as ::core::ffi::c_int);
                return false;
            }
        }
        player = player.wrapping_add(1);
    }
    WebTransport_checksum(frameNr, 1 as ::core::ffi::c_int);
    return true;
}
#[inline]
unsafe extern "C" fn WebTransport_checksum(
    mut frame: ::core::ffi::c_uint,
    mut matches: ::core::ffi::c_int,
) {
    let mut c2rust_lvalue: [::core::ffi::c_char; 3] = [
        'i' as ::core::ffi::c_char,
        'i' as ::core::ffi::c_char,
        '\0' as ::core::ffi::c_char,
    ];
    emscripten_asm_const_int_sync_on_main_thread(
        ({
            extern "C" { static uqm_em_asm_2: [::core::ffi::c_char; 250]; }
            &raw const uqm_em_asm_2 as *const ::core::ffi::c_char
        }),
        &raw mut c2rust_lvalue as *mut ::core::ffi::c_char,
        frame,
        matches,
    );
}
