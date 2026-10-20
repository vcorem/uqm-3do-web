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

/* By Michael Martin, 2002-09-21
 */

// Rust translation of src/libs/task/tasklib.c.
extern "C" {
    fn CreateThread_Core(
        func: ThreadFunction,
        data: *mut ::core::ffi::c_void,
        stackSize: SDWORD,
        name: *const ::core::ffi::c_char,
    ) -> Thread;
    fn CreateMutex_Core(name: *const ::core::ffi::c_char, syncClass: DWORD) -> Mutex;
    fn TaskSwitch();
    fn DestroyMutex(sem: Mutex);
    fn LockMutex(sem: Mutex);
    fn UnlockMutex(sem: Mutex);
    fn log_add(_: log_Level, fmt: *const ::core::ffi::c_char, ...);
}
pub type uint32 = ::core::ffi::c_uint;
pub type sint32 = ::core::ffi::c_int;
pub type DWORD = uint32;
pub type SDWORD = sint32;
pub type ThreadFunction = Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
>;
pub type Thread = *mut ::core::ffi::c_void;
pub type Mutex = *mut ::core::ffi::c_void;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed {
    pub const SYNC_CLASS_TOPLEVEL: Self = Self(1);
    pub const SYNC_CLASS_AUDIO: Self = Self(2);
    pub const SYNC_CLASS_VIDEO: Self = Self(4);
    pub const SYNC_CLASS_RESOURCE: Self = Self(8);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct taskstruct {
    pub state_mutex: Mutex,
    pub state: DWORD,
    pub name: *const ::core::ffi::c_char,
    pub thread: Thread,
}
pub type Task = *mut taskstruct;
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
static mut task_array: [taskstruct; 64] = [taskstruct {
    state_mutex: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    state: 0,
    name: ::core::ptr::null::<::core::ffi::c_char>(),
    thread: ::core::ptr::null_mut::<::core::ffi::c_void>(),
}; 64];
#[no_mangle]
pub unsafe extern "C" fn AssignTask(
    mut task_func: ThreadFunction,
    mut stackSize: SDWORD,
    mut name: *const ::core::ffi::c_char,
) -> Task {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < 64 as ::core::ffi::c_int {
        if Task_SetState(
            (&raw mut task_array as *mut taskstruct).offset(i as isize),
            1 as DWORD,
        ) == 0
        {
            Task_ClearState(
                (&raw mut task_array as *mut taskstruct).offset(i as isize),
                !(1 as ::core::ffi::c_int) as DWORD,
            );
            task_array[i as usize].name = name;
            ::core::ptr::write_volatile(
                &raw mut task_array[i as usize].thread,
                CreateThread_Core(
                    task_func,
                    (&raw mut task_array as *mut taskstruct).offset(i as isize)
                        as *mut ::core::ffi::c_void,
                    stackSize,
                    name,
                ),
            );
            return (&raw mut task_array as *mut taskstruct).offset(i as isize);
        }
        i += 1;
    }
    log_add(
        log_Level::log_Error,
        b"Task error!  Task array exhausted.  Check for thread leaks.\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    return ::core::ptr::null_mut::<taskstruct>();
}
#[no_mangle]
pub unsafe extern "C" fn FinishTask(mut task: Task) {
    ::core::ptr::write_volatile(
        &raw mut (*task).thread,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    );
    if Task_ClearState(task, 1 as DWORD) == 0 {
        log_add(
            log_Level::log_Debug,
            b"Task error!  Attempted to FinishTask '%s'... but it was already done!\0"
                .as_ptr() as *const ::core::ffi::c_char,
            (*task).name,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn ConcludeTask(mut task: Task) {
    let mut old: Thread = ::core::ptr::read_volatile::<
        Thread,
    >(&raw const (*task).thread);
    if !old.is_null() {
        Task_SetState(task, 2 as DWORD);
        while ::core::ptr::read_volatile::<Thread>(&raw const (*task).thread) == old {
            TaskSwitch();
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn Task_SetState(mut task: Task, mut state_mask: DWORD) -> DWORD {
    let mut old_state: DWORD = 0;
    LockMutex((*task).state_mutex);
    old_state = ::core::ptr::read_volatile::<DWORD>(&raw const (*task).state);
    ::core::ptr::write_volatile(
        &raw mut (*task).state,
        ::core::ptr::read_volatile::<DWORD>(&raw const (*task).state) | state_mask,
    );
    UnlockMutex((*task).state_mutex);
    old_state &= state_mask;
    return old_state;
}
#[no_mangle]
pub unsafe extern "C" fn Task_ClearState(
    mut task: Task,
    mut state_mask: DWORD,
) -> DWORD {
    let mut old_state: DWORD = 0;
    LockMutex((*task).state_mutex);
    old_state = ::core::ptr::read_volatile::<DWORD>(&raw const (*task).state);
    ::core::ptr::write_volatile(
        &raw mut (*task).state,
        ::core::ptr::read_volatile::<DWORD>(&raw const (*task).state) & !state_mask,
    );
    UnlockMutex((*task).state_mutex);
    old_state &= state_mask;
    return old_state;
}
#[no_mangle]
pub unsafe extern "C" fn Task_ToggleState(
    mut task: Task,
    mut state_mask: DWORD,
) -> DWORD {
    let mut old_state: DWORD = 0;
    LockMutex((*task).state_mutex);
    old_state = ::core::ptr::read_volatile::<DWORD>(&raw const (*task).state);
    ::core::ptr::write_volatile(
        &raw mut (*task).state,
        ::core::ptr::read_volatile::<DWORD>(&raw const (*task).state) ^ state_mask,
    );
    UnlockMutex((*task).state_mutex);
    old_state &= state_mask;
    return old_state;
}
#[no_mangle]
pub unsafe extern "C" fn Task_ReadState(mut task: Task, mut state_mask: DWORD) -> DWORD {
    return ::core::ptr::read_volatile::<DWORD>(&raw const (*task).state) & state_mask;
}
#[no_mangle]
pub unsafe extern "C" fn InitTaskSystem() {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < 64 as ::core::ffi::c_int {
        task_array[i as usize].state_mutex = CreateMutex_Core(
            b"task manager lock\0".as_ptr() as *const ::core::ffi::c_char,
            (C2Rust_Unnamed::SYNC_CLASS_TOPLEVEL.0 as ::core::ffi::c_int
                | C2Rust_Unnamed::SYNC_CLASS_RESOURCE.0 as ::core::ffi::c_int) as DWORD,
        );
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CleanupTaskSystem() {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < 64 as ::core::ffi::c_int {
        DestroyMutex(task_array[i as usize].state_mutex);
        task_array[i as usize].state_mutex = ::core::ptr::null_mut::<
            ::core::ffi::c_void,
        >();
        i += 1;
    }
}
