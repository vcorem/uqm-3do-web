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

// Rust translation of src/libs/threads/thrcommon.c.
extern "C" {
    fn exit(_: ::core::ffi::c_int) -> !;
    fn emscripten_sleep(ms: ::core::ffi::c_uint);
    fn GetTimeCounter() -> TimeCount;
    fn log_add(_: log_Level, fmt: *const ::core::ffi::c_char, ...);
    fn Async_process();
    fn Async_timeBeforeNextMs() -> uint32;
    fn HMalloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn HFree(p: *mut ::core::ffi::c_void);
    fn InitThreadSystem_PT();
    fn UnInitThreadSystem_PT();
    fn CreateThread_PT(
        func: ThreadFunction,
        data: *mut ::core::ffi::c_void,
        stackSize: SDWORD,
        name: *const ::core::ffi::c_char,
    ) -> Thread;
    fn CreateMutex_PT(name: *const ::core::ffi::c_char, syncClass: DWORD) -> Mutex;
    fn CreateSemaphore_PT(
        initial: DWORD,
        name: *const ::core::ffi::c_char,
        syncClass: DWORD,
    ) -> Semaphore;
    fn CreateRecursiveMutex_PT(
        name: *const ::core::ffi::c_char,
        syncClass: DWORD,
    ) -> RecursiveMutex;
    fn CreateCondVar_PT(name: *const ::core::ffi::c_char, syncClass: DWORD) -> CondVar;
    fn GetMyThreadLocal_PT() -> *mut ThreadLocal;
    fn SleepThread_PT(sleepTime: TimeCount);
    fn SleepThreadUntil_PT(wakeTime: TimeCount);
    fn TaskSwitch_PT();
    fn WaitThread_PT(thread: Thread, status: *mut ::core::ffi::c_int);
    fn DestroyThread_PT(thread: Thread);
    fn DestroyMutex_PT(m: Mutex);
    fn LockMutex_PT(m: Mutex);
    fn UnlockMutex_PT(m: Mutex);
    fn DestroySemaphore_PT(sem: Semaphore);
    fn SetSemaphore_PT(sem: Semaphore);
    fn ClearSemaphore_PT(sem: Semaphore);
    fn DestroyCondVar_PT(c: CondVar);
    fn WaitCondVar_PT(c: CondVar);
    fn SignalCondVar_PT(c: CondVar);
    fn BroadcastCondVar_PT(c: CondVar);
    fn DestroyRecursiveMutex_PT(m: RecursiveMutex);
    fn LockRecursiveMutex_PT(m: RecursiveMutex);
    fn UnlockRecursiveMutex_PT(m: RecursiveMutex);
    fn GetRecursiveMutexDepth_PT(m: RecursiveMutex) -> ::core::ffi::c_int;
}
pub type size_t = usize;
pub type uint32 = ::core::ffi::c_uint;
pub type sint32 = ::core::ffi::c_int;
pub type DWORD = uint32;
pub type SDWORD = sint32;
pub type TimeCount = DWORD;
pub type TimePeriod = DWORD;
pub type Mutex = *mut ::core::ffi::c_void;
pub type Thread = *mut ::core::ffi::c_void;
pub type SpawnRequest = *mut SpawnRequest_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SpawnRequest_struct {
    pub func: ThreadFunction,
    pub data: *mut ::core::ffi::c_void,
    pub stackSize: SDWORD,
    pub sem: Semaphore,
    pub value: Thread,
    pub name: *const ::core::ffi::c_char,
}
pub type Semaphore = *mut ::core::ffi::c_void;
pub type ThreadFunction = Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
>;
pub type RecursiveMutex = *mut ::core::ffi::c_void;
pub type CondVar = *mut ::core::ffi::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _threadLocal {
    pub flushSem: Semaphore,
}
pub type ThreadLocal = _threadLocal;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed {
    pub const SYNC_CLASS_TOPLEVEL: Self = Self(1);
    pub const SYNC_CLASS_AUDIO: Self = Self(2);
    pub const SYNC_CLASS_VIDEO: Self = Self(4);
    pub const SYNC_CLASS_RESOURCE: Self = Self(8);
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
static mut lifecycleMutex: Mutex = ::core::ptr::null_mut::<::core::ffi::c_void>();
static mut pendingBirth: [SpawnRequest; 8] = [::core::ptr::null_mut::<
    SpawnRequest_struct,
>(); 8];
static mut pendingDeath: [Thread; 8] = [::core::ptr::null_mut::<
    ::core::ffi::c_void,
>(); 8];
#[no_mangle]
pub unsafe extern "C" fn InitThreadSystem() {
    let mut i: ::core::ffi::c_int = 0;
    InitThreadSystem_PT();
    i = 0 as ::core::ffi::c_int;
    while i < 8 as ::core::ffi::c_int {
        pendingBirth[i as usize] = ::core::ptr::null_mut::<SpawnRequest_struct>();
        pendingDeath[i as usize] = ::core::ptr::null_mut::<::core::ffi::c_void>()
            as Thread;
        i += 1;
    }
    lifecycleMutex = CreateMutex_Core(
        b"Thread Lifecycle Mutex\0".as_ptr() as *const ::core::ffi::c_char,
        C2Rust_Unnamed::SYNC_CLASS_RESOURCE.0 as ::core::ffi::c_int as DWORD,
    );
}
#[no_mangle]
pub unsafe extern "C" fn UnInitThreadSystem() {
    UnInitThreadSystem_PT();
    DestroyMutex(lifecycleMutex);
}
unsafe extern "C" fn FlagStartThread(mut s: SpawnRequest) -> Thread {
    let mut i: ::core::ffi::c_int = 0;
    LockMutex(lifecycleMutex);
    i = 0 as ::core::ffi::c_int;
    while i < 8 as ::core::ffi::c_int {
        if pendingBirth[i as usize].is_null() {
            pendingBirth[i as usize] = s;
            UnlockMutex(lifecycleMutex);
            if !(*s).sem.is_null() {
                let mut result: Thread = ::core::ptr::null_mut::<::core::ffi::c_void>();
                SetSemaphore((*s).sem);
                DestroySemaphore((*s).sem);
                result = (*s).value;
                HFree(s as *mut ::core::ffi::c_void);
                return result;
            }
            return ::core::ptr::null_mut::<::core::ffi::c_void>();
        }
        i += 1;
    }
    log_add(
        log_Level::log_Fatal,
        b"Thread Lifecycle array filled.  This is a fatal error!  Make LIFECYCLE_SIZE something larger than %d.\0"
            .as_ptr() as *const ::core::ffi::c_char,
        8 as ::core::ffi::c_int,
    );
    exit(1 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn FinishThread(mut thread: Thread) {
    let mut i: ::core::ffi::c_int = 0;
    LockMutex(lifecycleMutex);
    i = 0 as ::core::ffi::c_int;
    while i < 8 as ::core::ffi::c_int {
        if pendingDeath[i as usize].is_null() {
            pendingDeath[i as usize] = thread;
            UnlockMutex(lifecycleMutex);
            return;
        }
        i += 1;
    }
    log_add(
        log_Level::log_Fatal,
        b"Thread Lifecycle array filled.  This is a fatal error!  Make LIFECYCLE_SIZE something larger than %d.\0"
            .as_ptr() as *const ::core::ffi::c_char,
        8 as ::core::ffi::c_int,
    );
    exit(1 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn ProcessThreadLifecycles() {
    let mut i: ::core::ffi::c_int = 0;
    LockMutex(lifecycleMutex);
    i = 0 as ::core::ffi::c_int;
    while i < 8 as ::core::ffi::c_int {
        let mut s: SpawnRequest = pendingBirth[i as usize];
        if !s.is_null() {
            (*s).value = CreateThread_PT(
                (*s).func,
                (*s).data,
                (*s).stackSize,
                (*s).name,
            );
            if !(*s).sem.is_null() {
                ClearSemaphore((*s).sem);
            } else {
                HFree(s as *mut ::core::ffi::c_void);
            }
            pendingBirth[i as usize] = ::core::ptr::null_mut::<SpawnRequest_struct>();
        }
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < 8 as ::core::ffi::c_int {
        let mut t: Thread = pendingDeath[i as usize];
        if !t.is_null() {
            WaitThread(t, ::core::ptr::null_mut::<::core::ffi::c_int>());
            pendingDeath[i as usize] = ::core::ptr::null_mut::<::core::ffi::c_void>()
                as Thread;
            DestroyThread(t);
        }
        i += 1;
    }
    UnlockMutex(lifecycleMutex);
}
#[no_mangle]
pub unsafe extern "C" fn CreateThread_Core(
    mut func: ThreadFunction,
    mut data: *mut ::core::ffi::c_void,
    mut stackSize: SDWORD,
    mut name: *const ::core::ffi::c_char,
) -> Thread {
    let mut s: SpawnRequest = HMalloc(::core::mem::size_of::<SpawnRequest_struct>())
        as SpawnRequest;
    (*s).func = func;
    (*s).data = data;
    (*s).stackSize = stackSize;
    (*s).name = name;
    (*s).sem = CreateSemaphore_Core(
        0 as DWORD,
        b"SpawnRequest semaphore\0".as_ptr() as *const ::core::ffi::c_char,
        C2Rust_Unnamed::SYNC_CLASS_RESOURCE.0 as ::core::ffi::c_int as DWORD,
    );
    return FlagStartThread(s);
}
#[no_mangle]
pub unsafe extern "C" fn StartThread_Core(
    mut func: ThreadFunction,
    mut data: *mut ::core::ffi::c_void,
    mut stackSize: SDWORD,
    mut name: *const ::core::ffi::c_char,
) {
    let mut s: SpawnRequest = HMalloc(::core::mem::size_of::<SpawnRequest_struct>())
        as SpawnRequest;
    (*s).func = func;
    (*s).data = data;
    (*s).stackSize = stackSize;
    (*s).name = name;
    (*s).sem = ::core::ptr::null_mut::<::core::ffi::c_void>() as Semaphore;
    FlagStartThread(s);
}
#[no_mangle]
pub unsafe extern "C" fn CreateMutex_Core(
    mut name: *const ::core::ffi::c_char,
    mut syncClass: DWORD,
) -> Mutex {
    return CreateMutex_PT(name, syncClass);
}
#[no_mangle]
pub unsafe extern "C" fn CreateSemaphore_Core(
    mut initial: DWORD,
    mut name: *const ::core::ffi::c_char,
    mut syncClass: DWORD,
) -> Semaphore {
    return CreateSemaphore_PT(initial, name, syncClass);
}
#[no_mangle]
pub unsafe extern "C" fn CreateRecursiveMutex_Core(
    mut name: *const ::core::ffi::c_char,
    mut syncClass: DWORD,
) -> RecursiveMutex {
    return CreateRecursiveMutex_PT(name, syncClass);
}
#[no_mangle]
pub unsafe extern "C" fn CreateCondVar_Core(
    mut name: *const ::core::ffi::c_char,
    mut syncClass: DWORD,
) -> CondVar {
    return CreateCondVar_PT(name, syncClass);
}
#[no_mangle]
pub unsafe extern "C" fn DestroyThread(mut t: Thread) {
    DestroyThread_PT(t);
}
#[no_mangle]
pub unsafe extern "C" fn CreateThreadLocal() -> *mut ThreadLocal {
    let mut tl: *mut ThreadLocal = HMalloc(::core::mem::size_of::<ThreadLocal>())
        as *mut ThreadLocal;
    (*tl).flushSem = CreateSemaphore_Core(
        0 as DWORD,
        b"FlushGraphics\0".as_ptr() as *const ::core::ffi::c_char,
        C2Rust_Unnamed::SYNC_CLASS_VIDEO.0 as ::core::ffi::c_int as DWORD,
    );
    return tl;
}
#[no_mangle]
pub unsafe extern "C" fn DestroyThreadLocal(mut tl: *mut ThreadLocal) {
    DestroySemaphore((*tl).flushSem);
    HFree(tl as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn GetMyThreadLocal() -> *mut ThreadLocal {
    return GetMyThreadLocal_PT();
}
#[no_mangle]
pub unsafe extern "C" fn WaitThread(
    mut thread: Thread,
    mut status: *mut ::core::ffi::c_int,
) {
    WaitThread_PT(thread, status);
}
#[no_mangle]
pub unsafe extern "C" fn HibernateThread(mut timePeriod: TimePeriod) {
    SleepThread_PT(timePeriod);
}
#[no_mangle]
pub unsafe extern "C" fn HibernateThreadUntil(mut wakeTime: TimeCount) {
    SleepThreadUntil_PT(wakeTime);
}
#[no_mangle]
pub unsafe extern "C" fn SleepThread(mut timePeriod: TimePeriod) {
    let mut now: TimeCount = 0;
    now = GetTimeCounter();
    uqm_lua_checked!(SleepThreadUntil(now.wrapping_add(timePeriod)), ());
}
#[no_mangle]
pub unsafe extern "C" fn SleepThreadUntil(mut wakeTime: TimeCount) {
    loop {
        let mut nextTimeMs: uint32 = 0;
        let mut nextTime: TimeCount = 0;
        let mut now: TimeCount = 0;
        uqm_lua_checked!(Async_process(), ());
        now = GetTimeCounter();
        if wakeTime <= now {
            return;
        }
        nextTimeMs = Async_timeBeforeNextMs();
        nextTime = nextTimeMs
            .wrapping_div(1000 as uint32)
            .wrapping_mul(840 as uint32)
            .wrapping_add(
                nextTimeMs
                    .wrapping_rem(1000 as uint32)
                    .wrapping_mul(840 as uint32)
                    .wrapping_div(1000 as uint32),
            ) as TimeCount;
        if wakeTime < nextTime {
            nextTime = wakeTime;
        }
        SleepThreadUntil_PT(nextTime);
    };
}
#[no_mangle]
pub unsafe extern "C" fn TaskSwitch() {
    TaskSwitch_PT();
}
#[no_mangle]
pub unsafe extern "C" fn DestroyMutex(mut sem: Mutex) {
    DestroyMutex_PT(sem);
}
#[no_mangle]
pub unsafe extern "C" fn LockMutex(mut sem: Mutex) {
    LockMutex_PT(sem);
}
#[no_mangle]
pub unsafe extern "C" fn UnlockMutex(mut sem: Mutex) {
    UnlockMutex_PT(sem);
}
#[no_mangle]
pub unsafe extern "C" fn DestroySemaphore(mut sem: Semaphore) {
    DestroySemaphore_PT(sem);
}
#[no_mangle]
pub unsafe extern "C" fn SetSemaphore(mut sem: Semaphore) {
    SetSemaphore_PT(sem);
}
#[no_mangle]
pub unsafe extern "C" fn ClearSemaphore(mut sem: Semaphore) {
    ClearSemaphore_PT(sem);
}
#[no_mangle]
pub unsafe extern "C" fn DestroyCondVar(mut cv: CondVar) {
    DestroyCondVar_PT(cv);
}
#[no_mangle]
pub unsafe extern "C" fn WaitCondVar(mut cv: CondVar) {
    WaitCondVar_PT(cv);
}
#[no_mangle]
pub unsafe extern "C" fn SignalCondVar(mut cv: CondVar) {
    SignalCondVar_PT(cv);
}
#[no_mangle]
pub unsafe extern "C" fn BroadcastCondVar(mut cv: CondVar) {
    BroadcastCondVar_PT(cv);
}
#[no_mangle]
pub unsafe extern "C" fn DestroyRecursiveMutex(mut mutex: RecursiveMutex) {
    DestroyRecursiveMutex_PT(mutex);
}
#[no_mangle]
pub unsafe extern "C" fn LockRecursiveMutex(mut mutex: RecursiveMutex) {
    LockRecursiveMutex_PT(mutex);
}
#[no_mangle]
pub unsafe extern "C" fn UnlockRecursiveMutex(mut mutex: RecursiveMutex) {
    UnlockRecursiveMutex_PT(mutex);
}
#[no_mangle]
pub unsafe extern "C" fn GetRecursiveMutexDepth(
    mut mutex: RecursiveMutex,
) -> ::core::ffi::c_int {
    return GetRecursiveMutexDepth_PT(mutex);
}
#[no_mangle]
pub unsafe extern "C" fn TaskSwitch_Emscripten() {
    emscripten_sleep(1 as ::core::ffi::c_uint);
}
