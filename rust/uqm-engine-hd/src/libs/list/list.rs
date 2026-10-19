/*
 * Copyright (C) 2005  Serge van den Boom
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of version 2 of the GNU General Public License as
 * published by the Free Software Foundation.
 * Nota bene: later versions of the GNU General Public License do not apply
 * to this program.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program; if not, write to the Free Software
 * Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA  02111-1307  USA
 *
 */

// Rust translation of src/libs/list/list.c.
extern "C" {
    fn HMalloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn HFree(p: *mut ::core::ffi::c_void);
    fn __assert_fail(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    ) -> !;
}
pub type size_t = usize;
pub type List_Entry = *mut ::core::ffi::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct List_List {
    pub first: *mut List_Link,
    pub end: *mut *mut List_Link,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct List_Link {
    pub entry: List_Entry,
    pub next: *mut List_Link,
}
#[no_mangle]
pub unsafe extern "C" fn List_newList() -> *mut List_List {
    let mut list: *mut List_List = ::core::ptr::null_mut::<List_List>();
    list = List_allocList();
    if list.is_null() {
        return ::core::ptr::null_mut::<List_List>();
    }
    (*list).first = ::core::ptr::null_mut::<List_Link>();
    (*list).end = &raw mut (*list).first;
    return list;
}
#[no_mangle]
pub unsafe extern "C" fn List_deleteList(mut list: *mut List_List) {
    let mut link: *mut List_Link = ::core::ptr::null_mut::<List_Link>();
    let mut next: *mut List_Link = ::core::ptr::null_mut::<List_Link>();
    link = (*list).first;
    while !link.is_null() {
        next = (*link).next;
        List_freeLink(link);
        link = next;
    }
    List_freeList(list);
}
#[no_mangle]
pub unsafe extern "C" fn List_add(mut list: *mut List_List, mut entry: List_Entry) {
    let mut link: *mut List_Link = ::core::ptr::null_mut::<List_Link>();
    link = List_allocLink();
    (*link).entry = entry;
    (*link).next = ::core::ptr::null_mut::<List_Link>();
    *(*list).end = link;
    (*list).end = &raw mut (*link).next;
}
#[inline]
unsafe extern "C" fn List_findLink(
    mut list: *mut List_List,
    mut entry: List_Entry,
) -> *mut *mut List_Link {
    let mut linkPtr: *mut *mut List_Link = ::core::ptr::null_mut::<*mut List_Link>();
    linkPtr = &raw mut (*list).first;
    while !(*linkPtr).is_null() {
        if (**linkPtr).entry == entry {
            return linkPtr;
        }
        linkPtr = &raw mut (**linkPtr).next;
    }
    return ::core::ptr::null_mut::<*mut List_Link>();
}
#[inline]
unsafe extern "C" fn List_removeLink(
    mut list: *mut List_List,
    mut linkPtr: *mut *mut List_Link,
) {
    let mut link: *mut List_Link = *linkPtr;
    *linkPtr = (*link).next;
    if &raw mut (*link).next == (*list).end {
        (*list).end = linkPtr;
    }
    List_freeLink(link);
}
#[no_mangle]
pub unsafe extern "C" fn List_remove(mut list: *mut List_List, mut entry: List_Entry) {
    let mut linkPtr: *mut *mut List_Link = ::core::ptr::null_mut::<*mut List_Link>();
    linkPtr = List_findLink(list, entry);
    (!linkPtr.is_null()
        || {
            __assert_fail(
                b"linkPtr != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/libs/list/list.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                107 as ::core::ffi::c_int,
                b"List_remove\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    List_removeLink(list, linkPtr);
}
#[inline]
unsafe extern "C" fn List_allocList() -> *mut List_List {
    return HMalloc(::core::mem::size_of::<List_List>()) as *mut List_List;
}
#[inline]
unsafe extern "C" fn List_freeList(mut list: *mut List_List) {
    HFree(list as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn List_allocLink() -> *mut List_Link {
    return HMalloc(::core::mem::size_of::<List_Link>()) as *mut List_Link;
}
#[inline]
unsafe extern "C" fn List_freeLink(mut link: *mut List_Link) {
    HFree(link as *mut ::core::ffi::c_void);
}
