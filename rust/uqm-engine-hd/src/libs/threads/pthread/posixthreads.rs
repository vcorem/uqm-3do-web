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

// Rust translation of src/libs/threads/pthread/posixthreads.c.
extern "C" {
    pub type __pthread;
    pub type _IO_FILE;
    fn malloc(_: size_t) -> *mut ::core::ffi::c_void;
    fn free(_: *mut ::core::ffi::c_void);
    fn abort() -> !;
    fn exit(_: ::core::ffi::c_int) -> !;
    fn usleep(_: ::core::ffi::c_uint) -> ::core::ffi::c_int;
    fn HMalloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn HFree(p: *mut ::core::ffi::c_void);
    fn GetTimeCounter() -> TimeCount;
    fn CreateThreadLocal() -> *mut ThreadLocal;
    fn DestroyThreadLocal(tl: *mut ThreadLocal);
    fn FinishThread(_: Thread);
    fn TaskSwitch_Emscripten();
    fn pthread_create(
        _: *mut pthread_t,
        _: *const pthread_attr_t,
        _: Option<
            unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
        >,
        _: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn pthread_join(
        _: pthread_t,
        _: *mut *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn pthread_self() -> pthread_t;
    fn pthread_mutex_init(
        _: *mut pthread_mutex_t,
        _: *const pthread_mutexattr_t,
    ) -> ::core::ffi::c_int;
    fn pthread_mutex_lock(_: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    fn pthread_mutex_unlock(_: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    fn pthread_mutex_destroy(_: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    fn pthread_cond_init(
        _: *mut pthread_cond_t,
        _: *const pthread_condattr_t,
    ) -> ::core::ffi::c_int;
    fn pthread_cond_destroy(_: *mut pthread_cond_t) -> ::core::ffi::c_int;
    fn pthread_cond_wait(
        _: *mut pthread_cond_t,
        _: *mut pthread_mutex_t,
    ) -> ::core::ffi::c_int;
    fn pthread_cond_broadcast(_: *mut pthread_cond_t) -> ::core::ffi::c_int;
    fn pthread_cond_signal(_: *mut pthread_cond_t) -> ::core::ffi::c_int;
    fn pthread_attr_init(_: *mut pthread_attr_t) -> ::core::ffi::c_int;
    fn pthread_attr_setstacksize(
        _: *mut pthread_attr_t,
        _: size_t,
    ) -> ::core::ffi::c_int;
    fn pthread_mutexattr_destroy(_: *mut pthread_mutexattr_t) -> ::core::ffi::c_int;
    fn pthread_mutexattr_init(_: *mut pthread_mutexattr_t) -> ::core::ffi::c_int;
    fn pthread_mutexattr_settype(
        _: *mut pthread_mutexattr_t,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    static stderr: *mut FILE;
    fn fflush(_: *mut FILE) -> ::core::ffi::c_int;
    fn sem_destroy(_: *mut sem_t) -> ::core::ffi::c_int;
    fn sem_getvalue(_: *mut sem_t, _: *mut ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn sem_init(
        _: *mut sem_t,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
    fn sem_post(_: *mut sem_t) -> ::core::ffi::c_int;
    fn sem_wait(_: *mut sem_t) -> ::core::ffi::c_int;
    fn log_add(_: log_Level, fmt: *const ::core::ffi::c_char, ...);
    fn log_add_nothread(_: log_Level, fmt: *const ::core::ffi::c_char, ...);
}
pub type size_t = ::core::ffi::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pthread_attr_t {
    pub __u: C2Rust_Unnamed,
    pub _a_transferredcanvases: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed {
    pub __i: [::core::ffi::c_int; 10],
    pub __vi: [::core::ffi::c_int; 10],
    pub __s: [::core::ffi::c_ulong; 10],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pthread_mutex_t {
    pub __u: C2Rust_Unnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_0 {
    pub __i: [::core::ffi::c_int; 6],
    pub __vi: [::core::ffi::c_int; 6],
    pub __p: [*mut ::core::ffi::c_void; 6],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pthread_cond_t {
    pub __u: C2Rust_Unnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_1 {
    pub __i: [::core::ffi::c_int; 12],
    pub __vi: [::core::ffi::c_int; 12],
    pub __p: [*mut ::core::ffi::c_void; 12],
}
pub type pthread_t = *mut __pthread;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pthread_mutexattr_t {
    pub __attr: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pthread_condattr_t {
    pub __attr: ::core::ffi::c_uint,
}
pub type uint32 = ::core::ffi::c_uint;
pub type sint32 = ::core::ffi::c_int;
pub type DWORD = uint32;
pub type SDWORD = sint32;
pub type TimeCount = DWORD;
pub type ThreadFunction = Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
>;
pub type Thread = *mut ::core::ffi::c_void;
pub type Mutex = *mut ::core::ffi::c_void;
pub type Semaphore = *mut ::core::ffi::c_void;
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
pub struct C2Rust_Unnamed_2(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_2 {
    pub const SYNC_CLASS_TOPLEVEL: Self = Self(1);
    pub const SYNC_CLASS_AUDIO: Self = Self(2);
    pub const SYNC_CLASS_VIDEO: Self = Self(4);
    pub const SYNC_CLASS_RESOURCE: Self = Self(8);
}
pub type TrueThread = *mut _thread;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _thread {
    pub native: pthread_t,
    pub name: *const ::core::ffi::c_char,
    pub localData: *mut ThreadLocal,
    pub next: *mut _thread,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sem_t {
    pub __val: [::core::ffi::c_int; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ThreadStartInfo {
    pub func: ThreadFunction,
    pub data: *mut ::core::ffi::c_void,
    pub sem: sem_t,
    pub thread: TrueThread,
}
pub type FILE = _IO_FILE;
pub type Mut = _mutex;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _mutex {
    pub mutex: pthread_mutex_t,
    pub owner: pthread_t,
    pub name: *const ::core::ffi::c_char,
    pub syncClass: DWORD,
}
pub type Sem = _sem;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _sem {
    pub sem: sem_t,
    pub name: *const ::core::ffi::c_char,
    pub syncClass: DWORD,
}
pub type RecM = _recm;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _recm {
    pub mutex: pthread_mutex_t,
    pub thread_id: pthread_t,
    pub locks: ::core::ffi::c_uint,
    pub name: *const ::core::ffi::c_char,
    pub syncClass: DWORD,
}
pub type cvar = _cond;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cond {
    pub cond: pthread_cond_t,
    pub mutex: pthread_mutex_t,
    pub name: *const ::core::ffi::c_char,
    pub syncClass: DWORD,
}
#[inline]
unsafe extern "C" fn explode() -> ! {
    abort();
}
static mut threadQueue: TrueThread = ::core::ptr::null_mut::<_thread>();
static mut threadQueueMutex: pthread_mutex_t = pthread_mutex_t {
    __u: C2Rust_Unnamed_0 { __i: [0; 6] },
};
#[no_mangle]
pub unsafe extern "C" fn InitThreadSystem_PT() {
    pthread_mutex_init(
        &raw mut threadQueueMutex,
        ::core::ptr::null::<pthread_mutexattr_t>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn UnInitThreadSystem_PT() {
    pthread_mutex_destroy(&raw mut threadQueueMutex);
}
unsafe extern "C" fn QueueThread(mut thread: TrueThread) {
    pthread_mutex_lock(&raw mut threadQueueMutex);
    (*thread).next = ::core::ptr::read_volatile::<TrueThread>(&raw const threadQueue)
        as *mut _thread;
    ::core::ptr::write_volatile(&raw mut threadQueue, thread);
    pthread_mutex_unlock(&raw mut threadQueueMutex);
}
unsafe extern "C" fn UnQueueThread(mut thread: TrueThread) {
    let mut ptr: *mut TrueThread = ::core::ptr::null_mut::<TrueThread>();
    pthread_mutex_lock(&raw mut threadQueueMutex);
    ptr = &raw mut threadQueue;
    while ::core::ptr::read_volatile::<TrueThread>(ptr) != thread {
        if ::core::ptr::read_volatile::<TrueThread>(ptr).is_null() {
            log_add(
                log_Level::log_Debug,
                b"Error: Trying to remove non-present thread from thread queue.\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
            fflush(stderr);
            explode();
        }
        ptr = &raw mut (*::core::ptr::read_volatile::<TrueThread>(ptr)).next
            as *mut TrueThread;
    }
    ::core::ptr::write_volatile(
        ptr,
        (*::core::ptr::read_volatile::<TrueThread>(ptr)).next as TrueThread,
    );
    pthread_mutex_unlock(&raw mut threadQueueMutex);
}
unsafe extern "C" fn FindThreadInfo(mut threadID: pthread_t) -> TrueThread {
    let mut ptr: TrueThread = ::core::ptr::null_mut::<_thread>();
    pthread_mutex_lock(&raw mut threadQueueMutex);
    ptr = ::core::ptr::read_volatile::<TrueThread>(&raw const threadQueue);
    while !ptr.is_null() {
        if (*ptr).native == threadID {
            pthread_mutex_unlock(&raw mut threadQueueMutex);
            return ptr;
        }
        ptr = (*ptr).next as TrueThread;
    }
    pthread_mutex_unlock(&raw mut threadQueueMutex);
    return ::core::ptr::null_mut::<_thread>();
}
unsafe extern "C" fn MyThreadName() -> *const ::core::ffi::c_char {
    let mut t: TrueThread = FindThreadInfo(pthread_self());
    return if !t.is_null() {
        (*t).name
    } else {
        b"Unknown (probably renderer)\0".as_ptr() as *const ::core::ffi::c_char
    };
}
unsafe extern "C" fn ThreadHelper(
    mut startInfo: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    let mut func: ThreadFunction = None;
    let mut data: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
        ::core::ffi::c_void,
    >();
    let mut sem: *mut sem_t = ::core::ptr::null_mut::<sem_t>();
    let mut thread: TrueThread = ::core::ptr::null_mut::<_thread>();
    let mut result: ::core::ffi::c_int = 0;
    func = (*(startInfo as *mut ThreadStartInfo)).func;
    data = (*(startInfo as *mut ThreadStartInfo)).data;
    sem = &raw mut (*(startInfo as *mut ThreadStartInfo)).sem;
    if sem_wait(sem) != 0 {
        log_add(
            log_Level::log_Fatal,
            b"ThreadHelper sem_wait fail\0".as_ptr() as *const ::core::ffi::c_char,
        );
        exit(1 as ::core::ffi::c_int);
    }
    if sem_destroy(sem) != 0 {
        log_add(
            log_Level::log_Fatal,
            b"ThreadHelper sem_destroy fail\0".as_ptr() as *const ::core::ffi::c_char,
        );
        exit(1 as ::core::ffi::c_int);
    }
    thread = (*(startInfo as *mut ThreadStartInfo)).thread;
    HFree(startInfo);
    result = uqm_lua_checked!(Some(func.expect("non-null function pointer"))
        .expect("non-null function pointer")(data), ::core::ptr::null_mut());
    log_add(
        log_Level::log_Debug,
        b"Thread '%s' done (returned %d).\0".as_ptr() as *const ::core::ffi::c_char,
        (*thread).name,
        result,
    );
    fflush(stderr);
    UnQueueThread(thread);
    DestroyThreadLocal((*thread).localData);
    FinishThread(thread as Thread);
    return ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(result as usize);
}
#[no_mangle]
pub unsafe extern "C" fn DestroyThread_PT(mut t: Thread) {
    HFree(t);
}
#[no_mangle]
pub unsafe extern "C" fn CreateThread_PT(
    mut func: ThreadFunction,
    mut data: *mut ::core::ffi::c_void,
    mut stackSize: SDWORD,
    mut name: *const ::core::ffi::c_char,
) -> Thread {
    let mut thread: TrueThread = ::core::ptr::null_mut::<_thread>();
    let mut startInfo: *mut ThreadStartInfo = ::core::ptr::null_mut::<ThreadStartInfo>();
    let mut attr: pthread_attr_t = pthread_attr_t {
        __u: C2Rust_Unnamed { __i: [0; 10] },
        _a_transferredcanvases: ::core::ptr::null::<::core::ffi::c_char>(),
    };
    thread = HMalloc(::core::mem::size_of::<_thread>() as size_t) as *mut _thread
        as TrueThread;
    (*thread).name = name;
    (*thread).localData = CreateThreadLocal();
    startInfo = HMalloc(::core::mem::size_of::<ThreadStartInfo>() as size_t)
        as *mut ThreadStartInfo;
    (*startInfo).func = func;
    (*startInfo).data = data;
    if sem_init(
        &raw mut (*startInfo).sem,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_uint,
    ) < 0 as ::core::ffi::c_int
    {
        log_add(
            log_Level::log_Fatal,
            b"createthread seminit fail\0".as_ptr() as *const ::core::ffi::c_char,
        );
        exit(1 as ::core::ffi::c_int);
    }
    (*startInfo).thread = thread;
    pthread_attr_init(&raw mut attr);
    if pthread_attr_setstacksize(&raw mut attr, 75000 as size_t) != 0 {
        log_add(
            log_Level::log_Debug,
            b"pthread stacksize fail\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if pthread_create(
        &raw mut (*thread).native,
        &raw mut attr,
        Some(
            ThreadHelper
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                ) -> *mut ::core::ffi::c_void,
        ),
        startInfo as *mut ::core::ffi::c_void,
    ) != 0
    {
        log_add(
            log_Level::log_Debug,
            b"pthread create fail\0".as_ptr() as *const ::core::ffi::c_char,
        );
        DestroyThreadLocal((*thread).localData);
        HFree(startInfo as *mut ::core::ffi::c_void);
        HFree(thread as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    QueueThread(thread);
    log_add(
        log_Level::log_Debug,
        b"Thread '%s' created.\0".as_ptr() as *const ::core::ffi::c_char,
        (*thread).name,
    );
    fflush(stderr);
    if sem_post(&raw mut (*startInfo).sem) != 0 {
        log_add(
            log_Level::log_Fatal,
            b"CreateThread sem_post fail\0".as_ptr() as *const ::core::ffi::c_char,
        );
        exit(1 as ::core::ffi::c_int);
    }
    return thread as Thread;
}
#[no_mangle]
pub unsafe extern "C" fn SleepThread_PT(mut sleepTime: TimeCount) {
    usleep(
        sleepTime
            .wrapping_mul(1000000 as ::core::ffi::c_int as ::core::ffi::c_uint)
            .wrapping_div(840 as ::core::ffi::c_uint),
    );
}
#[no_mangle]
pub unsafe extern "C" fn SleepThreadUntil_PT(mut wakeTime: TimeCount) {
    let mut now: TimeCount = 0;
    now = GetTimeCounter();
    if wakeTime <= now {
        TaskSwitch_PT();
    } else {
        usleep(
            wakeTime
                .wrapping_sub(now)
                .wrapping_mul(1000000 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_div(840 as ::core::ffi::c_uint),
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn TaskSwitch_PT() {
    TaskSwitch_Emscripten();
}
#[no_mangle]
pub unsafe extern "C" fn WaitThread_PT(
    mut thread: Thread,
    mut status: *mut ::core::ffi::c_int,
) {
    pthread_join(
        (*(thread as TrueThread)).native,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_void>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn GetMyThreadLocal_PT() -> *mut ThreadLocal {
    let mut t: TrueThread = FindThreadInfo(pthread_self());
    return if !t.is_null() {
        (*t).localData
    } else {
        ::core::ptr::null_mut::<ThreadLocal>()
    };
}
#[no_mangle]
pub unsafe extern "C" fn CreateMutex_PT(
    mut name: *const ::core::ffi::c_char,
    mut syncClass: DWORD,
) -> Mutex {
    let mut mutex: *mut Mut = malloc(::core::mem::size_of::<Mut>() as size_t)
        as *mut Mut;
    if !mutex.is_null() {
        let mut attr: pthread_mutexattr_t = pthread_mutexattr_t { __attr: 0 };
        pthread_mutexattr_init(&raw mut attr);
        pthread_mutexattr_settype(&raw mut attr, 1 as ::core::ffi::c_int);
        if pthread_mutex_init(&raw mut (*mutex).mutex, &raw mut attr) != 0 {
            log_add_nothread(
                log_Level::log_Fatal,
                b"Could not initialize mutex '%s':aborting.\0".as_ptr()
                    as *const ::core::ffi::c_char,
                name,
            );
            exit(1 as ::core::ffi::c_int);
        }
        pthread_mutexattr_destroy(&raw mut attr);
        (*mutex).owner = ::core::ptr::null_mut::<__pthread>();
        (*mutex).name = name;
        (*mutex).syncClass = syncClass;
    }
    return mutex as Mutex;
}
#[no_mangle]
pub unsafe extern "C" fn DestroyMutex_PT(mut m: Mutex) {
    let mut mutex: *mut Mut = m as *mut Mut;
    pthread_mutex_destroy(&raw mut (*mutex).mutex);
    free(mutex as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn LockMutex_PT(mut m: Mutex) {
    let mut mutex: *mut Mut = m as *mut Mut;
    if !(*mutex).owner.is_null()
        && (*mutex).syncClass
            & C2Rust_Unnamed_2::SYNC_CLASS_TOPLEVEL.0 as ::core::ffi::c_int as DWORD != 0
    {
        log_add_nothread(
            log_Level::log_Debug,
            b"Thread '%s' blocking on mutex '%s'\0".as_ptr()
                as *const ::core::ffi::c_char,
            MyThreadName(),
            (*mutex).name,
        );
    }
    while pthread_mutex_lock(&raw mut (*mutex).mutex) != 0 as ::core::ffi::c_int {
        TaskSwitch_PT();
    }
    (*mutex).owner = pthread_self();
}
#[no_mangle]
pub unsafe extern "C" fn UnlockMutex_PT(mut m: Mutex) {
    let mut mutex: *mut Mut = m as *mut Mut;
    (*mutex).owner = ::core::ptr::null_mut::<__pthread>();
    while pthread_mutex_unlock(&raw mut (*mutex).mutex) != 0 as ::core::ffi::c_int {
        TaskSwitch_PT();
    }
}
#[no_mangle]
pub unsafe extern "C" fn CreateSemaphore_PT(
    mut initial: DWORD,
    mut name: *const ::core::ffi::c_char,
    mut syncClass: DWORD,
) -> Semaphore {
    let mut sem: *mut Sem = HMalloc(::core::mem::size_of::<_sem>() as size_t)
        as *mut Sem;
    (*sem).name = name;
    (*sem).syncClass = syncClass;
    if sem_init(&raw mut (*sem).sem, 0 as ::core::ffi::c_int, initial)
        < 0 as ::core::ffi::c_int
    {
        log_add(
            log_Level::log_Fatal,
            b"Could not initialize semaphore '%s': aborting.\0".as_ptr()
                as *const ::core::ffi::c_char,
            name,
        );
        exit(1 as ::core::ffi::c_int);
    }
    return sem as Semaphore;
}
#[no_mangle]
pub unsafe extern "C" fn DestroySemaphore_PT(mut s: Semaphore) {
    let mut sem: *mut Sem = s as *mut Sem;
    if sem_destroy(&raw mut (*sem).sem) != 0 {
        log_add(
            log_Level::log_Debug,
            b"Destroying semaphore '%s' failed\0".as_ptr() as *const ::core::ffi::c_char,
            (*sem).name,
        );
    }
    HFree(sem as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn SetSemaphore_PT(mut s: Semaphore) {
    let mut sem: *mut Sem = s as *mut Sem;
    let mut contention: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    sem_getvalue(&raw mut (*sem).sem, &raw mut contention);
    contention = (contention == 0) as ::core::ffi::c_int;
    if contention != 0
        && (*sem).syncClass
            & C2Rust_Unnamed_2::SYNC_CLASS_TOPLEVEL.0 as ::core::ffi::c_int as DWORD != 0
    {
        log_add(
            log_Level::log_Debug,
            b"Thread '%s' blocking on semaphore '%s'\0".as_ptr()
                as *const ::core::ffi::c_char,
            MyThreadName(),
            (*sem).name,
        );
    }
    while sem_wait(&raw mut (*sem).sem) == -1 as ::core::ffi::c_int {
        TaskSwitch_PT();
    }
    if contention != 0
        && (*sem).syncClass
            & C2Rust_Unnamed_2::SYNC_CLASS_TOPLEVEL.0 as ::core::ffi::c_int as DWORD != 0
    {
        log_add(
            log_Level::log_Debug,
            b"Thread '%s' awakens, released from semaphore '%s'\0".as_ptr()
                as *const ::core::ffi::c_char,
            MyThreadName(),
            (*sem).name,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn ClearSemaphore_PT(mut s: Semaphore) {
    let mut sem: *mut Sem = s as *mut Sem;
    while sem_post(&raw mut (*sem).sem) == -1 as ::core::ffi::c_int {
        TaskSwitch_PT();
    }
}
#[no_mangle]
pub unsafe extern "C" fn CreateRecursiveMutex_PT(
    mut name: *const ::core::ffi::c_char,
    mut syncClass: DWORD,
) -> RecursiveMutex {
    let mut mtx: *mut RecM = HMalloc(::core::mem::size_of::<_recm>() as size_t)
        as *mut RecM;
    (*mtx).thread_id = ::core::ptr::null_mut::<__pthread>();
    if pthread_mutex_init(
        &raw mut (*mtx).mutex,
        ::core::ptr::null::<pthread_mutexattr_t>(),
    ) != 0
    {
        log_add(
            log_Level::log_Fatal,
            b"Could not initialize recursive mutex '%s': aborting.\0".as_ptr()
                as *const ::core::ffi::c_char,
            name,
        );
        exit(1 as ::core::ffi::c_int);
    }
    (*mtx).name = name;
    (*mtx).syncClass = syncClass;
    (*mtx).locks = 0 as ::core::ffi::c_uint;
    return mtx as RecursiveMutex;
}
#[no_mangle]
pub unsafe extern "C" fn DestroyRecursiveMutex_PT(mut val: RecursiveMutex) {
    let mut mtx: *mut RecM = val as *mut RecM;
    pthread_mutex_destroy(&raw mut (*mtx).mutex);
    HFree(mtx as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn LockRecursiveMutex_PT(mut val: RecursiveMutex) {
    let mut mtx: *mut RecM = val as *mut RecM;
    let mut thread_id: pthread_t = pthread_self();
    if (*mtx).locks == 0 || (*mtx).thread_id != thread_id {
        if !(*mtx).thread_id.is_null()
            && (*mtx).syncClass
                & C2Rust_Unnamed_2::SYNC_CLASS_TOPLEVEL.0 as ::core::ffi::c_int as DWORD
                != 0
        {
            log_add(
                log_Level::log_Debug,
                b"Thread '%s' blocking on '%s'\0".as_ptr() as *const ::core::ffi::c_char,
                MyThreadName(),
                (*mtx).name,
            );
        }
        while pthread_mutex_lock(&raw mut (*mtx).mutex) != 0 {
            TaskSwitch_PT();
        }
        (*mtx).thread_id = thread_id;
    }
    (*mtx).locks = (*mtx).locks.wrapping_add(1);
}
#[no_mangle]
pub unsafe extern "C" fn UnlockRecursiveMutex_PT(mut val: RecursiveMutex) {
    let mut mtx: *mut RecM = val as *mut RecM;
    let mut thread_id: pthread_t = pthread_self();
    if (*mtx).locks == 0 || (*mtx).thread_id != thread_id {
        log_add(
            log_Level::log_Debug,
            b"'%s' attempted to unlock %s when it didn't hold it\0".as_ptr()
                as *const ::core::ffi::c_char,
            MyThreadName(),
            (*mtx).name,
        );
    } else {
        (*mtx).locks = (*mtx).locks.wrapping_sub(1);
        if (*mtx).locks == 0 {
            (*mtx).thread_id = ::core::ptr::null_mut::<__pthread>();
            pthread_mutex_unlock(&raw mut (*mtx).mutex);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn GetRecursiveMutexDepth_PT(
    mut val: RecursiveMutex,
) -> ::core::ffi::c_int {
    let mut mtx: *mut RecM = val as *mut RecM;
    return (*mtx).locks as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn CreateCondVar_PT(
    mut name: *const ::core::ffi::c_char,
    mut syncClass: DWORD,
) -> CondVar {
    let mut err1: ::core::ffi::c_int = 0;
    let mut err2: ::core::ffi::c_int = 0;
    let mut cv: *mut cvar = HMalloc(::core::mem::size_of::<cvar>() as size_t)
        as *mut cvar;
    err1 = pthread_cond_init(
        &raw mut (*cv).cond,
        ::core::ptr::null::<pthread_condattr_t>(),
    );
    err2 = pthread_mutex_init(
        &raw mut (*cv).mutex,
        ::core::ptr::null::<pthread_mutexattr_t>(),
    );
    if err1 != 0 || err2 != 0 {
        log_add(
            log_Level::log_Fatal,
            b"Could not initialize condition variable '%s': aborting.\0".as_ptr()
                as *const ::core::ffi::c_char,
            name,
        );
        exit(1 as ::core::ffi::c_int);
    }
    (*cv).name = name;
    (*cv).syncClass = syncClass;
    return cv as CondVar;
}
#[no_mangle]
pub unsafe extern "C" fn DestroyCondVar_PT(mut c: CondVar) {
    let mut cv: *mut cvar = c as *mut cvar;
    pthread_cond_destroy(&raw mut (*cv).cond);
    pthread_mutex_destroy(&raw mut (*cv).mutex);
    HFree(cv as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn WaitCondVar_PT(mut c: CondVar) {
    let mut cv: *mut cvar = c as *mut cvar;
    pthread_mutex_lock(&raw mut (*cv).mutex);
    if (*cv).syncClass
        & C2Rust_Unnamed_2::SYNC_CLASS_TOPLEVEL.0 as ::core::ffi::c_int as DWORD != 0
    {
        log_add(
            log_Level::log_Debug,
            b"Thread '%s' waiting for signal from '%s'\0".as_ptr()
                as *const ::core::ffi::c_char,
            MyThreadName(),
            (*cv).name,
        );
    }
    while pthread_cond_wait(&raw mut (*cv).cond, &raw mut (*cv).mutex)
        != 0 as ::core::ffi::c_int
    {
        TaskSwitch_PT();
    }
    if (*cv).syncClass
        & C2Rust_Unnamed_2::SYNC_CLASS_TOPLEVEL.0 as ::core::ffi::c_int as DWORD != 0
    {
        log_add(
            log_Level::log_Debug,
            b"Thread '%s' received signal from '%s', awakening.\0".as_ptr()
                as *const ::core::ffi::c_char,
            MyThreadName(),
            (*cv).name,
        );
    }
    pthread_mutex_unlock(&raw mut (*cv).mutex);
}
#[no_mangle]
pub unsafe extern "C" fn SignalCondVar_PT(mut c: CondVar) {
    let mut cv: *mut cvar = c as *mut cvar;
    pthread_cond_signal(&raw mut (*cv).cond);
}
#[no_mangle]
pub unsafe extern "C" fn BroadcastCondVar_PT(mut c: CondVar) {
    let mut cv: *mut cvar = c as *mut cvar;
    pthread_cond_broadcast(&raw mut (*cv).cond);
}
