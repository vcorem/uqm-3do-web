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

// Rust translation of src/uqm/displist.c.
extern "C" {
    fn __assert_fail(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    ) -> !;
    fn HMalloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn HFree(p: *mut ::core::ffi::c_void);
    fn log_add(_: log_Level, fmt: *const ::core::ffi::c_char, ...);
}
pub type size_t = usize;
pub type uint8 = ::core::ffi::c_uchar;
pub type uint16 = ::core::ffi::c_ushort;
pub type BYTE = uint8;
pub type UWORD = uint16;
pub type COUNT = UWORD;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct BOOLEAN(pub ::core::ffi::c_uint);
impl BOOLEAN {
    pub const FALSE: Self = Self(0);
    pub const TRUE: Self = Self(1);
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
#[no_mangle]
pub unsafe extern "C" fn InitQueue(
    mut pq: *mut QUEUE,
    mut num_elements: COUNT,
    mut size: OBJ_SIZE,
) -> BOOLEAN {
    (*pq).head = ::core::ptr::null_mut::<::core::ffi::c_void>() as HLINK;
    (*pq).tail = ::core::ptr::null_mut::<::core::ffi::c_void>() as HLINK;
    (*pq).object_size = size;
    (*pq).free_list = ::core::ptr::null_mut::<::core::ffi::c_void>() as HLINK;
    (*pq).num_objects = num_elements as BYTE;
    (*pq).pq_tab = HMalloc(
        ((*pq).object_size as ::core::ffi::c_int
            * (*pq).num_objects as COUNT as ::core::ffi::c_int) as size_t,
    ) as *mut BYTE;
    if !(*pq).pq_tab.is_null() {
        loop {
            FreeLink(
                pq,
                (*pq)
                    .pq_tab
                    .offset(
                        ((*pq).object_size as ::core::ffi::c_int
                            * (num_elements as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int)) as isize,
                    ) as HLINK,
            );
            num_elements = num_elements.wrapping_sub(1);
            if num_elements == 0 {
                break;
            }
        }
        return BOOLEAN::TRUE;
    }
    return BOOLEAN::FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn UninitQueue(mut pq: *mut QUEUE) -> BOOLEAN {
    (*pq).head = ::core::ptr::null_mut::<::core::ffi::c_void>() as HLINK;
    (*pq).tail = ::core::ptr::null_mut::<::core::ffi::c_void>() as HLINK;
    (*pq).free_list = ::core::ptr::null_mut::<::core::ffi::c_void>() as HLINK;
    HFree((*pq).pq_tab as *mut ::core::ffi::c_void);
    (*pq).pq_tab = ::core::ptr::null_mut::<BYTE>();
    return BOOLEAN::TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn ReinitQueue(mut pq: *mut QUEUE) {
    (*pq).head = ::core::ptr::null_mut::<::core::ffi::c_void>() as HLINK;
    (*pq).tail = ::core::ptr::null_mut::<::core::ffi::c_void>() as HLINK;
    let mut num_elements: COUNT = 0;
    (*pq).free_list = ::core::ptr::null_mut::<::core::ffi::c_void>() as HLINK;
    num_elements = (*pq).num_objects as COUNT;
    if num_elements != 0 {
        loop {
            FreeLink(
                pq,
                (*pq)
                    .pq_tab
                    .offset(
                        ((*pq).object_size as ::core::ffi::c_int
                            * (num_elements as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int)) as isize,
                    ) as HLINK,
            );
            num_elements = num_elements.wrapping_sub(1);
            if num_elements == 0 {
                break;
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn AllocLink(mut pq: *mut QUEUE) -> HLINK {
    let mut hLink: HLINK = ::core::ptr::null_mut::<::core::ffi::c_void>();
    hLink = (*pq).free_list;
    if !hLink.is_null() {
        let mut LinkPtr: *mut LINK = ::core::ptr::null_mut::<LINK>();
        LinkPtr = LockLink(pq, hLink);
        (*pq).free_list = (*LinkPtr).succ;
        UnlockLink(pq, hLink);
    } else {
        log_add(
            log_Level::log_Debug,
            b"AllocLink(): No more elements\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    return hLink;
}
#[no_mangle]
pub unsafe extern "C" fn FreeLink(mut pq: *mut QUEUE, mut hLink: HLINK) {
    let mut LinkPtr: *mut LINK = ::core::ptr::null_mut::<LINK>();
    LinkPtr = LockLink(pq, hLink);
    (*LinkPtr).succ = (*pq).free_list;
    UnlockLink(pq, hLink);
    (*pq).free_list = hLink;
}
#[no_mangle]
pub unsafe extern "C" fn PutQueue(mut pq: *mut QUEUE, mut hLink: HLINK) {
    let mut LinkPtr: *mut LINK = ::core::ptr::null_mut::<LINK>();
    if (*pq).head.is_null() {
        (*pq).head = hLink;
    } else {
        let mut hTail: HLINK = ::core::ptr::null_mut::<::core::ffi::c_void>();
        let mut lpTail: *mut LINK = ::core::ptr::null_mut::<LINK>();
        hTail = (*pq).tail;
        lpTail = LockLink(pq, hTail);
        (*lpTail).succ = hLink;
        UnlockLink(pq, hTail);
    }
    LinkPtr = LockLink(pq, hLink);
    (*LinkPtr).pred = (*pq).tail;
    (*LinkPtr).succ = ::core::ptr::null_mut::<::core::ffi::c_void>() as HLINK;
    UnlockLink(pq, hLink);
    (*pq).tail = hLink;
}
#[no_mangle]
pub unsafe extern "C" fn InsertQueue(
    mut pq: *mut QUEUE,
    mut hLink: HLINK,
    mut hRefLink: HLINK,
) {
    if hRefLink.is_null() {
        PutQueue(pq, hLink);
    } else {
        let mut LinkPtr: *mut LINK = ::core::ptr::null_mut::<LINK>();
        let mut RefLinkPtr: *mut LINK = ::core::ptr::null_mut::<LINK>();
        LinkPtr = LockLink(pq, hLink);
        RefLinkPtr = LockLink(pq, hRefLink);
        (*LinkPtr).pred = (*RefLinkPtr).pred;
        (*RefLinkPtr).pred = hLink;
        (*LinkPtr).succ = hRefLink;
        if (*pq).head == hRefLink {
            (*pq).head = hLink;
        } else {
            let mut hPredLink: HLINK = ::core::ptr::null_mut::<::core::ffi::c_void>();
            let mut PredLinkPtr: *mut LINK = ::core::ptr::null_mut::<LINK>();
            hPredLink = (*LinkPtr).pred;
            PredLinkPtr = LockLink(pq, hPredLink);
            (*PredLinkPtr).succ = hLink;
            UnlockLink(pq, hPredLink);
        }
        UnlockLink(pq, hRefLink);
        UnlockLink(pq, hLink);
    };
}
#[no_mangle]
pub unsafe extern "C" fn RemoveQueue(mut pq: *mut QUEUE, mut hLink: HLINK) {
    let mut LinkPtr: *mut LINK = ::core::ptr::null_mut::<LINK>();
    LinkPtr = LockLink(pq, hLink);
    if (*pq).head == hLink {
        (*pq).head = (*LinkPtr).succ;
    } else {
        let mut hPredLink: HLINK = ::core::ptr::null_mut::<::core::ffi::c_void>();
        let mut PredLinkPtr: *mut LINK = ::core::ptr::null_mut::<LINK>();
        hPredLink = (*LinkPtr).pred;
        PredLinkPtr = LockLink(pq, hPredLink);
        (*PredLinkPtr).succ = (*LinkPtr).succ;
        UnlockLink(pq, hPredLink);
    }
    if (*pq).tail == hLink {
        (*pq).tail = (*LinkPtr).pred;
    } else {
        let mut hSuccLink: HLINK = ::core::ptr::null_mut::<::core::ffi::c_void>();
        let mut hPredLink_0: HLINK = ::core::ptr::null_mut::<::core::ffi::c_void>();
        let mut SuccLinkPtr: *mut LINK = ::core::ptr::null_mut::<LINK>();
        hSuccLink = (*LinkPtr).succ;
        SuccLinkPtr = LockLink(pq, hSuccLink);
        hPredLink_0 = (*LinkPtr).pred;
        (*SuccLinkPtr).pred = hPredLink_0;
        UnlockLink(pq, hSuccLink);
    }
    UnlockLink(pq, hLink);
}
#[no_mangle]
pub unsafe extern "C" fn CountLinks(mut pq: *mut QUEUE) -> COUNT {
    let mut LinkCount: COUNT = 0;
    let mut hLink: HLINK = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut hNextLink: HLINK = ::core::ptr::null_mut::<::core::ffi::c_void>();
    LinkCount = 0 as COUNT;
    hLink = (*pq).head;
    while !hLink.is_null() {
        let mut LinkPtr: *mut LINK = ::core::ptr::null_mut::<LINK>();
        LinkCount = LinkCount.wrapping_add(1);
        LinkPtr = LockLink(pq, hLink);
        hNextLink = (*LinkPtr).succ;
        UnlockLink(pq, hLink);
        hLink = hNextLink;
    }
    return LinkCount;
}
#[no_mangle]
pub unsafe extern "C" fn ForAllLinks(
    mut pq: *mut QUEUE,
    mut callback: Option<
        unsafe extern "C" fn(*mut LINK, *mut ::core::ffi::c_void) -> (),
    >,
    mut arg: *mut ::core::ffi::c_void,
) {
    let mut hLink: HLINK = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut hNextLink: HLINK = ::core::ptr::null_mut::<::core::ffi::c_void>();
    hLink = (*pq).head;
    while !hLink.is_null() {
        let mut LinkPtr: *mut LINK = ::core::ptr::null_mut::<LINK>();
        LinkPtr = LockLink(pq, hLink);
        hNextLink = (*LinkPtr).succ;
        uqm_lua_checked!(Some(callback.expect("non-null function pointer"))
            .expect("non-null function pointer")(LinkPtr, arg), ());
        UnlockLink(pq, hLink);
        hLink = hNextLink;
    }
}
