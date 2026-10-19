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

// Rust translation of src/libs/log/uqmlog.c.
extern "C" {
    pub type _IO_FILE;
    static stdout: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn vsnprintf(
        _: *mut ::core::ffi::c_char,
        _: size_t,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::VaList,
    ) -> ::core::ffi::c_int;
    fn log_displayBox(
        title: *const ::core::ffi::c_char,
        isError: ::core::ffi::c_int,
        msg: *const ::core::ffi::c_char,
    );
    fn atexit(_: Option<unsafe extern "C" fn() -> ()>) -> ::core::ffi::c_int;
    fn CreateMutex_Core(name: *const ::core::ffi::c_char, syncClass: DWORD) -> Mutex;
    fn DestroyMutex(sem: Mutex);
    fn LockMutex(sem: Mutex);
    fn UnlockMutex(sem: Mutex);
    fn memcpy(
        _: *mut ::core::ffi::c_void,
        _: *const ::core::ffi::c_void,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strcpy(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(_: *const ::core::ffi::c_char) -> size_t;
}
pub type __builtin_va_list = *mut ::core::ffi::c_void;
pub type size_t = usize;
pub type uint32 = ::core::ffi::c_uint;
pub type va_list = __builtin_va_list;
pub type __isoc_va_list = __builtin_va_list;
pub type FILE = _IO_FILE;
pub type Mutex = *mut ::core::ffi::c_void;
pub type log_Entry = [::core::ffi::c_char; 256];
pub type DWORD = uint32;
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
pub struct C2Rust_Unnamed(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed {
    pub const SYNC_CLASS_TOPLEVEL: Self = Self(1);
    pub const SYNC_CLASS_AUDIO: Self = Self(2);
    pub const SYNC_CLASS_VIDEO: Self = Self(4);
    pub const SYNC_CLASS_RESOURCE: Self = Self(8);
}
static mut queue: [log_Entry; 128] = [[0; 256]; 128];
static mut msgNoThread: log_Entry = [0; 256];
static mut msgBuf: [::core::ffi::c_char; 16384] = [0; 16384];
static mut maxLevel: ::core::ffi::c_int = log_Level::log_Error.0 as ::core::ffi::c_int;
static mut maxStreamLevel: ::core::ffi::c_int = log_Level::log_Debug.0
    as ::core::ffi::c_int;
static mut maxDisp: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
static mut qtotal: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut qhead: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut qtail: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut noThreadReady: bool = false;
static mut showBox: bool = true;
static mut errorBox: bool = true;
#[no_mangle]
pub static mut streamOut: *mut FILE = ::core::ptr::null_mut::<FILE>();
static mut qlock: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut qmutex: Mutex = ::core::ptr::null_mut::<::core::ffi::c_void>();
unsafe extern "C" fn lockQueue() {
    if ::core::ptr::read_volatile::<::core::ffi::c_int>(&raw const qlock) == 0 {
        return;
    }
    LockMutex(qmutex);
}
unsafe extern "C" fn unlockQueue() {
    if ::core::ptr::read_volatile::<::core::ffi::c_int>(&raw const qlock) == 0 {
        return;
    }
    UnlockMutex(qmutex);
}
unsafe extern "C" fn removeExcess(mut room: ::core::ffi::c_int) {
    room = maxDisp - room;
    if room < 0 as ::core::ffi::c_int {
        room = 0 as ::core::ffi::c_int;
    }
    while qtotal > room {
        qtotal -= 1;
        qtail += 1;
    }
    qtail %= 128 as ::core::ffi::c_int;
}
unsafe extern "C" fn acquireSlot() -> ::core::ffi::c_int {
    let mut slot: ::core::ffi::c_int = 0;
    lockQueue();
    removeExcess(1 as ::core::ffi::c_int);
    slot = qhead;
    qhead = (qhead + 1 as ::core::ffi::c_int) % 128 as ::core::ffi::c_int;
    qtotal += 1;
    unlockQueue();
    return slot;
}
unsafe extern "C" fn queueNonThreaded() {
    let mut slot: ::core::ffi::c_int = 0;
    if !::core::ptr::read_volatile::<bool>(&raw const noThreadReady) {
        return;
    }
    ::core::ptr::write_volatile(&raw mut noThreadReady, false);
    slot = acquireSlot();
    memcpy(
        &raw mut *(&raw mut queue as *mut log_Entry).offset(slot as isize)
            as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        &raw mut msgNoThread as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
        ::core::mem::size_of::<log_Entry>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn log_init(mut max_lines: ::core::ffi::c_int) {
    let mut i: ::core::ffi::c_int = 0;
    maxDisp = max_lines;
    streamOut = stdout;
    i = 0 as ::core::ffi::c_int;
    while i < 128 as ::core::ffi::c_int {
        queue[i
            as usize][(256 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as usize] = '\0'
            as ::core::ffi::c_char;
        i += 1;
    }
    msgBuf[::core::mem::size_of::<[::core::ffi::c_char; 16384]>()
        .wrapping_sub(1usize)] = '\0' as ::core::ffi::c_char;
    msgNoThread[::core::mem::size_of::<log_Entry>().wrapping_sub(1usize)] = '\0'
        as ::core::ffi::c_char;
    atexit(Some(exitCallback as unsafe extern "C" fn() -> ()));
}
#[no_mangle]
pub unsafe extern "C" fn log_initThreads() {
    qmutex = CreateMutex_Core(
        b"Logging Lock\0".as_ptr() as *const ::core::ffi::c_char,
        C2Rust_Unnamed::SYNC_CLASS_RESOURCE.0 as ::core::ffi::c_int as DWORD,
    );
    ::core::ptr::write_volatile(&raw mut qlock, 1 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn log_exit(mut code: ::core::ffi::c_int) -> ::core::ffi::c_int {
    showBox = false;
    if ::core::ptr::read_volatile::<::core::ffi::c_int>(&raw const qlock) != 0 {
        ::core::ptr::write_volatile(&raw mut qlock, 0 as ::core::ffi::c_int);
        DestroyMutex(qmutex);
        qmutex = ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    return code;
}
#[no_mangle]
pub unsafe extern "C" fn log_setLevel(mut level: ::core::ffi::c_int) {
    maxLevel = level;
}
#[no_mangle]
pub unsafe extern "C" fn log_setOutput(mut out: *mut FILE) -> *mut FILE {
    let mut old: *mut FILE = streamOut;
    streamOut = out;
    return old;
}
#[no_mangle]
pub unsafe extern "C" fn log_addV(
    mut level: log_Level,
    mut fmt: *const ::core::ffi::c_char,
    mut list: ::core::ffi::VaList,
) {
    let mut full_msg: log_Entry = [0; 256];
    vsnprintf(
        &raw mut full_msg as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<log_Entry>().wrapping_sub(1 as size_t),
        fmt,
        list.as_va_list(),
    );
    full_msg[::core::mem::size_of::<log_Entry>().wrapping_sub(1usize)] = '\0'
        as ::core::ffi::c_char;
    if level.0 as ::core::ffi::c_int <= maxStreamLevel {
        fprintf(
            streamOut,
            b"%s\n\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut full_msg as *mut ::core::ffi::c_char,
        );
    }
    if level.0 as ::core::ffi::c_int <= maxLevel {
        let mut slot: ::core::ffi::c_int = 0;
        queueNonThreaded();
        slot = acquireSlot();
        memcpy(
            &raw mut *(&raw mut queue as *mut log_Entry).offset(slot as isize)
                as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            &raw mut full_msg as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
            ::core::mem::size_of::<log_Entry>(),
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn log_add(
    mut level: log_Level,
    mut fmt: *const ::core::ffi::c_char,
    mut c2rust_args: ...
) {
    let mut list: ::core::ffi::VaListImpl;
    list = c2rust_args.clone();
    log_addV(level, fmt, list.as_va_list());
}
#[no_mangle]
pub unsafe extern "C" fn log_add_nothreadV(
    mut level: log_Level,
    mut fmt: *const ::core::ffi::c_char,
    mut list: ::core::ffi::VaList,
) {
    let mut full_msg: log_Entry = [0; 256];
    vsnprintf(
        &raw mut full_msg as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<log_Entry>().wrapping_sub(1 as size_t),
        fmt,
        list.as_va_list(),
    );
    full_msg[::core::mem::size_of::<log_Entry>().wrapping_sub(1usize)] = '\0'
        as ::core::ffi::c_char;
    if level.0 as ::core::ffi::c_int <= maxStreamLevel {
        fprintf(
            streamOut,
            b"%s\n\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut full_msg as *mut ::core::ffi::c_char,
        );
    }
    if level.0 as ::core::ffi::c_int <= maxLevel {
        memcpy(
            &raw mut msgNoThread as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            &raw mut full_msg as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
            ::core::mem::size_of::<log_Entry>(),
        );
        ::core::ptr::write_volatile(&raw mut noThreadReady, true);
    }
}
#[no_mangle]
pub unsafe extern "C" fn log_add_nothread(
    mut level: log_Level,
    mut fmt: *const ::core::ffi::c_char,
    mut c2rust_args: ...
) {
    let mut list: ::core::ffi::VaListImpl;
    list = c2rust_args.clone();
    log_add_nothreadV(level, fmt, list.as_va_list());
}
#[no_mangle]
pub unsafe extern "C" fn log_showBox(mut show: bool, mut err: bool) {
    showBox = show;
    errorBox = err;
}
#[no_mangle]
pub unsafe extern "C" fn log_captureLines(mut num: ::core::ffi::c_int) {
    if num > 128 as ::core::ffi::c_int {
        num = 128 as ::core::ffi::c_int;
    }
    if num < 1 as ::core::ffi::c_int {
        num = 1 as ::core::ffi::c_int;
    }
    maxDisp = num;
    lockQueue();
    removeExcess(0 as ::core::ffi::c_int);
    unlockQueue();
}
unsafe extern "C" fn exitCallback() {
    if showBox {
        displayLog(errorBox);
    }
    log_exit(0 as ::core::ffi::c_int);
}
unsafe extern "C" fn displayLog(mut isError: bool) {
    let mut p: *mut ::core::ffi::c_char = &raw mut msgBuf as *mut ::core::ffi::c_char;
    let mut left: ::core::ffi::c_int = ::core::mem::size_of::<
        [::core::ffi::c_char; 16384],
    >()
        .wrapping_sub(1usize) as ::core::ffi::c_int;
    let mut len: ::core::ffi::c_int = 0;
    let mut ptr: ::core::ffi::c_int = 0;
    if isError {
        strcpy(
            p,
            b"The Ur-Quan Masters encountered a fatal error.\nPart of the log follows:\n\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        len = strlen(p) as ::core::ffi::c_int;
        p = p.offset(len as isize);
        left -= len;
    }
    ptr = qtail;
    while ptr != qhead && left > 0 as ::core::ffi::c_int {
        len = strlen(
                &raw mut *(&raw mut queue as *mut log_Entry).offset(ptr as isize)
                    as *mut ::core::ffi::c_char,
            )
            .wrapping_add(1 as size_t) as ::core::ffi::c_int;
        if len > left {
            len = left;
        }
        memcpy(
            p as *mut ::core::ffi::c_void,
            &raw mut *(&raw mut queue as *mut log_Entry).offset(ptr as isize)
                as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
            len as size_t,
        );
        *p.offset((len - 1 as ::core::ffi::c_int) as isize) = '\n'
            as ::core::ffi::c_char;
        p = p.offset(len as isize);
        left -= len;
        ptr = (ptr + 1 as ::core::ffi::c_int) % 128 as ::core::ffi::c_int;
    }
    if ::core::ptr::read_volatile::<bool>(&raw const noThreadReady) {
        ::core::ptr::write_volatile(&raw mut noThreadReady, false);
        len = strlen(&raw mut msgNoThread as *mut ::core::ffi::c_char)
            as ::core::ffi::c_int;
        if len > left {
            len = left;
        }
        memcpy(
            p as *mut ::core::ffi::c_void,
            &raw mut msgNoThread as *mut ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            len as size_t,
        );
        p = p.offset(len as isize);
        left -= len;
    }
    *p = '\0' as ::core::ffi::c_char;
    log_displayBox(
        b"The Ur-Quan Masters\0".as_ptr() as *const ::core::ffi::c_char,
        isError as ::core::ffi::c_int,
        &raw mut msgBuf as *mut ::core::ffi::c_char,
    );
}
