// SPDX-License-Identifier: GPL-2.0-or-later
// Derived from sc2/src/uqm/displist.c and displist.h, also used by MegaMod.
// Copyright Paul Reiche and Fred Ford, 1992-2002.

//! The engine's bounded intrusive queues, including their reusable slot pool.
//!
//! Queue capacity is gameplay state: encounter counts depend on allocation
//! exhaustion. Slots therefore retain the original allocation/reuse order and
//! stale fields. This is not an unbounded Rust collection. All ownership stays
//! with the C engine's HMalloc/HFree allocator, and accesses are serialized by
//! the engine. Raw accesses preserve callback mutation and C execution order.

use core::ffi::c_void;
use core::ptr;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Link {
    pub pred: *mut Link,
    pub succ: *mut Link,
}

/// `QUEUE_TABLE` is mandatory in both upstream headers. Pointer fields must
/// remain in this order; the following count fields are u16 and u8, not usize.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Queue {
    pub head: *mut Link,
    pub tail: *mut Link,
    pub pq_tab: *mut u8,
    pub free_list: *mut Link,
    pub object_size: u16,
    pub num_objects: u8,
}

#[cfg(not(test))]
extern "C" {
    fn log_add(level: core::ffi::c_int, format: *const core::ffi::c_char, ...);
}

fn log_exhausted() {
    #[cfg(not(test))]
    // SAFETY: log_Debug is 5 in uqmlog.h. The static format contains no format
    // specifiers, is NUL-terminated, and the C logger owns its synchronization.
    unsafe {
        log_add(5, c"AllocLink(): No more elements".as_ptr());
    }
}

/// # Safety
/// `queue` must point to writable Queue storage without a live owned pool.
/// Capacity must be 1..=255 and slot size a multiple of Link alignment, at
/// least the size of Link. As in C, every slot begins with a Link. Calls and
/// subsequent access to this queue/pool must be serialized.
#[no_mangle]
pub unsafe extern "C" fn InitQueue(queue: *mut Queue, capacity: u16, size: u16) -> i32 {
    // The C do/while corrupts memory for capacity 0 or >255, and undersized or
    // unaligned slots cannot hold LINK. Reject those invalid contracts before
    // allocating; all engine call sites supply valid capacities and C sizes.
    assert!((1..=255).contains(&capacity));
    assert!(usize::from(size) >= core::mem::size_of::<Link>());
    assert_eq!(usize::from(size) % core::mem::align_of::<Link>(), 0);
    // SAFETY: The caller supplies writable storage and valid slot geometry.
    // Only succ is initialized, exactly as FreeLink does in the C source.
    unsafe {
        (*queue).head = ptr::null_mut();
        (*queue).tail = ptr::null_mut();
        (*queue).object_size = size;
        (*queue).free_list = ptr::null_mut();
        (*queue).num_objects = capacity as u8;
        (*queue).pq_tab = crate::HMalloc(usize::from(size) * usize::from(capacity)).cast();
        if (*queue).pq_tab.is_null() {
            return 0;
        }
        for index in (0..usize::from(capacity)).rev() {
            let link = (*queue).pq_tab.add(usize::from(size) * index).cast();
            FreeLink(queue, link);
        }
        1
    }
}

/// # Safety
/// `queue` must be initialized with either a live owned pool or a null pool.
/// No pool pointers may be used after this call. Access must be serialized.
#[no_mangle]
pub unsafe extern "C" fn UninitQueue(queue: *mut Queue) -> i32 {
    // SAFETY: The queue owns its C allocation; HFree accepts a null pointer.
    unsafe {
        (*queue).head = ptr::null_mut();
        (*queue).tail = ptr::null_mut();
        (*queue).free_list = ptr::null_mut();
        crate::HFree((*queue).pq_tab.cast());
        (*queue).pq_tab = ptr::null_mut();
        1
    }
}

/// # Safety
/// `queue` must own a valid initialized pool whose slots are not concurrently
/// used. Existing payloads/pred fields remain, but all links become available.
#[no_mangle]
pub unsafe extern "C" fn ReinitQueue(queue: *mut Queue) {
    // SAFETY: Every computed slot lies in the caller's live C-owned pool.
    unsafe {
        (*queue).head = ptr::null_mut();
        (*queue).tail = ptr::null_mut();
        (*queue).free_list = ptr::null_mut();
        for index in (0..usize::from((*queue).num_objects)).rev() {
            let link = (*queue)
                .pq_tab
                .add(usize::from((*queue).object_size) * index)
                .cast();
            FreeLink(queue, link);
        }
    }
}

/// # Safety
/// `queue` and its free list must be valid and accessed serially. The returned
/// slot remains owned by this queue; null means its fixed capacity is exhausted.
#[no_mangle]
pub unsafe extern "C" fn AllocLink(queue: *mut Queue) -> *mut Link {
    // SAFETY: A non-null free-list head is a valid initialized Link slot.
    unsafe {
        let link = (*queue).free_list;
        if link.is_null() {
            log_exhausted();
        } else {
            (*queue).free_list = (*link).succ;
        }
        link
    }
}

/// # Safety
/// `link` must be a non-null allocated slot of `queue`, removed from the active
/// list and absent from the free list. Access must be serialized. Only the succ
/// field is changed; pred and payload deliberately retain their old values.
#[no_mangle]
pub unsafe extern "C" fn FreeLink(queue: *mut Queue, link: *mut Link) {
    // SAFETY: Both raw pointers satisfy the queue's intrusive ownership contract.
    unsafe {
        (*link).succ = (*queue).free_list;
        (*queue).free_list = link;
    }
}

/// # Safety
/// `queue` must have a valid list. `link` must be an allocated slot of the same
/// pool, not currently in either list. All access must be serialized.
#[no_mangle]
pub unsafe extern "C" fn PutQueue(queue: *mut Queue, link: *mut Link) {
    // SAFETY: The caller maintains valid intrusive links. Write order follows C.
    unsafe {
        if (*queue).head.is_null() {
            (*queue).head = link;
        } else {
            (*(*queue).tail).succ = link;
        }
        (*link).pred = (*queue).tail;
        (*link).succ = ptr::null_mut();
        (*queue).tail = link;
    }
}

/// # Safety
/// `queue` must have a valid list. `link` is an unlinked allocated slot of this
/// pool; `reference` is either a current list member or null to append. Access
/// must be serialized, with no simultaneously held exclusive Rust references.
#[no_mangle]
pub unsafe extern "C" fn InsertQueue(queue: *mut Queue, link: *mut Link, reference: *mut Link) {
    // SAFETY: Intrusive ownership/list membership are required of the caller.
    unsafe {
        if reference.is_null() {
            PutQueue(queue, link);
        } else {
            (*link).pred = (*reference).pred;
            (*reference).pred = link;
            (*link).succ = reference;
            if (*queue).head == reference {
                (*queue).head = link;
            } else {
                (*(*link).pred).succ = link;
            }
        }
    }
}

/// # Safety
/// `link` must be a current member of `queue`'s valid active list, accessed
/// serially. Removal does not free the slot or clear its stale link fields.
#[no_mangle]
pub unsafe extern "C" fn RemoveQueue(queue: *mut Queue, link: *mut Link) {
    // SAFETY: Existing neighbors and queue state are valid by caller contract.
    unsafe {
        if (*queue).head == link {
            (*queue).head = (*link).succ;
        } else {
            (*(*link).pred).succ = (*link).succ;
        }
        if (*queue).tail == link {
            (*queue).tail = (*link).pred;
        } else {
            (*(*link).succ).pred = (*link).pred;
        }
    }
}

/// # Safety
/// `queue` must contain a valid finite active list with serialized access.
#[no_mangle]
pub unsafe extern "C" fn CountLinks(queue: *mut Queue) -> u16 {
    // SAFETY: Each non-null link belongs to the caller's finite valid list.
    unsafe {
        let mut count = 0u16;
        let mut link = (*queue).head;
        while !link.is_null() {
            count = count.wrapping_add(1);
            link = (*link).succ;
        }
        count
    }
}

/// # Safety
/// The queue/list and callback must be valid and accessed serially. The callback
/// may remove/free the current slot; it must leave the already-cached successor
/// valid and keep the queue's pool alive. Its argument has the callback's own
/// validity requirements. No Rust reference is retained across the callback.
#[no_mangle]
pub unsafe extern "C" fn ForAllLinks(
    queue: *mut Queue,
    callback: unsafe extern "C" fn(*mut Link, *mut c_void),
    argument: *mut c_void,
) {
    // SAFETY: The caller/callback maintain validity of the cached successor.
    unsafe {
        let mut link = (*queue).head;
        while !link.is_null() {
            let next = (*link).succ;
            callback(link, argument);
            link = next;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn empty_queue() -> Queue {
        Queue {
            head: ptr::null_mut(),
            tail: ptr::null_mut(),
            pq_tab: ptr::null_mut(),
            free_list: ptr::null_mut(),
            object_size: 0,
            num_objects: 0,
        }
    }

    #[test]
    fn pool_capacity_reuse_and_reinitialization_are_deterministic() {
        let mut queue = empty_queue();
        // SAFETY: Each allocated slot is used serially and the pool freed once.
        unsafe {
            assert_eq!(
                InitQueue(&mut queue, 3, core::mem::size_of::<Link>() as u16),
                1
            );
            let a = AllocLink(&mut queue);
            let b = AllocLink(&mut queue);
            let c = AllocLink(&mut queue);
            assert_eq!(a.cast::<u8>(), queue.pq_tab);
            assert_eq!(b, a.add(1));
            assert_eq!(c, a.add(2));
            assert!(AllocLink(&mut queue).is_null());
            PutQueue(&mut queue, a);
            PutQueue(&mut queue, c);
            InsertQueue(&mut queue, b, c);
            assert_eq!(CountLinks(&mut queue), 3);
            assert_eq!((*a).succ, b);
            assert_eq!((*c).pred, b);
            RemoveQueue(&mut queue, b);
            assert_eq!((*b).pred, a);
            assert_eq!((*b).succ, c);
            FreeLink(&mut queue, b);
            assert_eq!(AllocLink(&mut queue), b);
            ReinitQueue(&mut queue);
            assert_eq!(CountLinks(&mut queue), 0);
            assert_eq!(AllocLink(&mut queue), a);
            assert_eq!(AllocLink(&mut queue), b);
            assert_eq!(AllocLink(&mut queue), c);
            assert_eq!(UninitQueue(&mut queue), 1);
            assert!(queue.pq_tab.is_null());
            assert_eq!(queue.num_objects, 3);
        }
    }

    unsafe extern "C" fn remove_current(link: *mut Link, argument: *mut c_void) {
        // SAFETY: The test passes its live queue; the iterator caches next first.
        unsafe {
            let queue = argument.cast::<Queue>();
            RemoveQueue(queue, link);
            FreeLink(queue, link);
        }
    }

    #[test]
    fn callback_can_remove_and_free_current_link() {
        let mut queue = empty_queue();
        // SAFETY: The callback mutates only the current link; successors survive.
        unsafe {
            InitQueue(&mut queue, 3, core::mem::size_of::<Link>() as u16);
            for _ in 0..3 {
                let link = AllocLink(&mut queue);
                PutQueue(&mut queue, link);
            }
            let old_tail = queue.tail;
            let queue_pointer = &mut queue as *mut Queue;
            ForAllLinks(queue_pointer, remove_current, queue_pointer.cast());
            assert_eq!(CountLinks(&mut queue), 0);
            assert_eq!(AllocLink(&mut queue), old_tail);
            UninitQueue(&mut queue);
        }
    }
}
