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

// Rust translation of src/libs/callback/callback.c.
extern "C" {
    fn __assert_fail(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    ) -> !;
    fn malloc(_: size_t) -> *mut ::core::ffi::c_void;
    fn free(_: *mut ::core::ffi::c_void);
    fn CreateMutex_Core(name: *const ::core::ffi::c_char, syncClass: DWORD) -> Mutex;
    fn DestroyMutex(sem: Mutex);
    fn LockMutex(sem: Mutex);
    fn UnlockMutex(sem: Mutex);
}
pub type size_t = usize;
pub type uint32 = ::core::ffi::c_uint;
pub type DWORD = uint32;
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
pub struct CallbackLink {
    pub next: *mut CallbackLink,
    pub callback: CallbackFunction,
    pub arg: CallbackArg,
}
pub type CallbackArg = *mut ::core::ffi::c_void;
pub type CallbackFunction = Option<unsafe extern "C" fn(CallbackArg) -> ()>;
pub type CallbackID = *mut CallbackLink;
static mut callbacks: *mut CallbackLink = ::core::ptr::null_mut::<CallbackLink>();
static mut callbacksEnd: *mut *mut CallbackLink = ::core::ptr::null_mut::<
    *mut CallbackLink,
>();
static mut callbacksProcessEnd: *const *mut CallbackLink = ::core::ptr::null::<
    *mut CallbackLink,
>();
static mut callbackListLock: Mutex = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[inline]
unsafe extern "C" fn CallbackList_lock() {
    LockMutex(callbackListLock);
}
#[inline]
unsafe extern "C" fn CallbackList_unlock() {
    UnlockMutex(callbackListLock);
}
#[no_mangle]
pub unsafe extern "C" fn Callback_init() {
    callbacks = ::core::ptr::null_mut::<CallbackLink>();
    callbacksEnd = &raw mut callbacks;
    callbacksProcessEnd = &raw mut callbacks;
    callbackListLock = CreateMutex_Core(
        b"Callback List Lock\0".as_ptr() as *const ::core::ffi::c_char,
        C2Rust_Unnamed::SYNC_CLASS_TOPLEVEL.0 as ::core::ffi::c_int as DWORD,
    );
}
#[no_mangle]
pub unsafe extern "C" fn Callback_uninit() {
    DestroyMutex(callbackListLock);
    callbackListLock = ::core::ptr::null_mut::<::core::ffi::c_void>();
}
#[no_mangle]
pub unsafe extern "C" fn Callback_add(
    mut callback: CallbackFunction,
    mut arg: CallbackArg,
) -> CallbackID {
    let mut link: *mut CallbackLink = malloc(::core::mem::size_of::<CallbackLink>())
        as *mut CallbackLink;
    (*link).callback = callback;
    (*link).arg = arg;
    (*link).next = ::core::ptr::null_mut::<CallbackLink>();
    CallbackList_lock();
    *callbacksEnd = link;
    callbacksEnd = &raw mut (*link).next;
    CallbackList_unlock();
    return link;
}
unsafe extern "C" fn CallbackLink_delete(mut link: *mut CallbackLink) {
    free(link as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn CallbackLink_find(
    mut link: *mut CallbackLink,
) -> *mut *mut CallbackLink {
    let mut ptr: *mut *mut CallbackLink = ::core::ptr::null_mut::<*mut CallbackLink>();
    ptr = &raw mut callbacks;
    while !(*ptr).is_null() {
        if *ptr == link {
            return ptr;
        }
        ptr = &raw mut (**ptr).next;
    }
    return ::core::ptr::null_mut::<*mut CallbackLink>();
}
#[no_mangle]
pub unsafe extern "C" fn Callback_remove(mut id: CallbackID) -> bool {
    let mut link: *mut CallbackLink = id;
    let mut linkPtr: *mut *mut CallbackLink = ::core::ptr::null_mut::<
        *mut CallbackLink,
    >();
    CallbackList_lock();
    linkPtr = CallbackLink_find(link);
    if linkPtr.is_null() {
        CallbackList_unlock();
        return false;
    }
    if callbacksEnd == &raw mut (**linkPtr).next {
        callbacksEnd = linkPtr;
    }
    if callbacksProcessEnd == &raw mut (**linkPtr).next as *const *mut CallbackLink {
        callbacksProcessEnd = linkPtr;
    }
    *linkPtr = (**linkPtr).next;
    CallbackList_unlock();
    CallbackLink_delete(link);
    return true;
}
#[inline]
unsafe extern "C" fn CallbackLink_doCallback(mut link: *mut CallbackLink) {
    uqm_lua_checked!((*link).callback.expect("non-null function pointer")((*link).arg), ());
}
#[no_mangle]
pub unsafe extern "C" fn Callback_process() {
    let mut link: *mut CallbackLink = ::core::ptr::null_mut::<CallbackLink>();
    CallbackList_lock();
    callbacksProcessEnd = callbacksEnd;
    CallbackList_unlock();
    loop {
        CallbackList_lock();
        if callbacksProcessEnd == &raw mut callbacks as *const *mut CallbackLink {
            CallbackList_unlock();
            break;
        } else {
            (!callbacks.is_null()
                || {
                    __assert_fail(
                        b"callbacks != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                        b"src/libs/callback/callback.c\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        168 as ::core::ffi::c_int,
                        b"Callback_process\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    0 as ::core::ffi::c_int != 0
                }) as ::core::ffi::c_int;
            link = callbacks;
            callbacks = (*link).next;
            if callbacksEnd == &raw mut (*link).next {
                callbacksEnd = &raw mut callbacks;
            }
            if callbacksProcessEnd == &raw mut (*link).next as *const *mut CallbackLink {
                callbacksProcessEnd = &raw mut callbacks;
            }
            CallbackList_unlock();
            uqm_lua_checked!(CallbackLink_doCallback(link), ());
            CallbackLink_delete(link);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn Callback_haveMore() -> bool {
    let mut result: bool = false;
    CallbackList_lock();
    result = !callbacks.is_null();
    CallbackList_unlock();
    return result;
}
