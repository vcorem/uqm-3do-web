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

// Rust translation of src/libs/heap/heap.c.
extern "C" {
    fn malloc(_: size_t) -> *mut ::core::ffi::c_void;
    fn realloc(_: *mut ::core::ffi::c_void, _: size_t) -> *mut ::core::ffi::c_void;
    fn free(_: *mut ::core::ffi::c_void);
    fn __assert_fail(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    ) -> !;
    fn ceil(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
}
pub type size_t = ::core::ffi::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Heap {
    pub comparator: HeapValue_Comparator,
    pub minSize: size_t,
    pub minFillQuotient: ::core::ffi::c_double,
    pub entries: *mut *mut HeapValue,
    pub numEntries: size_t,
    pub size: size_t,
    pub minFill: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HeapValue {
    pub index: size_t,
}
pub type HeapValue_Comparator = Option<
    unsafe extern "C" fn(*mut HeapValue, *mut HeapValue) -> ::core::ffi::c_int,
>;
unsafe extern "C" fn Heap_resize(mut heap: *mut Heap, mut size: size_t) {
    (*heap).entries = realloc(
        (*heap).entries as *mut ::core::ffi::c_void,
        size.wrapping_mul(::core::mem::size_of::<*mut HeapValue>() as size_t),
    ) as *mut *mut HeapValue;
    (*heap).size = size as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn Heap_new(
    mut comparator: HeapValue_Comparator,
    mut initialSize: size_t,
    mut minSize: size_t,
    mut minFillQuotient: ::core::ffi::c_double,
) -> *mut Heap {
    let mut heap: *mut Heap = ::core::ptr::null_mut::<Heap>();
    (minFillQuotient >= 0.0f64
        || {
            __assert_fail(
                b"minFillQuotient >= 0.0\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/libs/heap/heap.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                41 as ::core::ffi::c_int,
                b"Heap_new\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    heap = malloc(::core::mem::size_of::<Heap>() as size_t) as *mut Heap;
    if initialSize < minSize {
        initialSize = minSize;
    }
    (*heap).comparator = comparator;
    (*heap).minSize = minSize as size_t;
    (*heap).minFillQuotient = minFillQuotient;
    (*heap).size = nextPower2(initialSize) as size_t;
    (*heap).minFill = ceil(
        ((*heap).size >> 1 as ::core::ffi::c_int) as ::core::ffi::c_double
            * (*heap).minFillQuotient,
    ) as size_t;
    (*heap).entries = malloc(
        (*heap).size.wrapping_mul(::core::mem::size_of::<*mut HeapValue>() as size_t),
    ) as *mut *mut HeapValue;
    (*heap).numEntries = 0 as size_t;
    return heap;
}
#[no_mangle]
pub unsafe extern "C" fn Heap_delete(mut heap: *mut Heap) {
    free((*heap).entries as *mut ::core::ffi::c_void);
    free(heap as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn Heap_add(mut heap: *mut Heap, mut value: *mut HeapValue) {
    let mut i: size_t = 0;
    if (*heap).numEntries >= (*heap).size {
        Heap_resize(heap, (*heap).size.wrapping_mul(2 as size_t));
    }
    i = (*heap).numEntries as size_t;
    (*heap).numEntries = (*heap).numEntries.wrapping_add(1);
    while i > 0 as size_t {
        let mut parentI: size_t = i.wrapping_sub(1 as size_t).wrapping_div(2 as size_t);
        if uqm_lua_checked!((*heap)
            .comparator
            .expect(
                "non-null function pointer",
            )(*(*heap).entries.offset(parentI as isize), value), ())
            <= 0 as ::core::ffi::c_int
        {
            break;
        }
        *(*heap).entries.offset(i as isize) = *(*heap).entries.offset(parentI as isize);
        (**(*heap).entries.offset(i as isize)).index = i as size_t;
        i = parentI;
    }
    *(*heap).entries.offset(i as isize) = value;
    (**(*heap).entries.offset(i as isize)).index = i as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn Heap_first(mut heap: *const Heap) -> *mut HeapValue {
    ((*heap).numEntries > 0 as size_t
        || {
            __assert_fail(
                b"heap->numEntries > 0\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/libs/heap/heap.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                91 as ::core::ffi::c_int,
                b"Heap_first\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    return *(*heap).entries.offset(0isize);
}
unsafe extern "C" fn Heap_removeByIndex(mut heap: *mut Heap, mut i: size_t) {
    ((*heap).numEntries > i
        || {
            __assert_fail(
                b"heap->numEntries > i\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/libs/heap/heap.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                98 as ::core::ffi::c_int,
                b"Heap_removeByIndex\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    (*heap).numEntries = (*heap).numEntries.wrapping_sub(1);
    if (*heap).numEntries != 0 as size_t {
        let mut lastEntry: *mut HeapValue = *(*heap)
            .entries
            .offset((*heap).numEntries as isize);
        loop {
            let mut childI: size_t = i
                .wrapping_mul(2 as size_t)
                .wrapping_add(1 as size_t);
            if childI.wrapping_add(1 as size_t) >= (*heap).numEntries {
                if childI >= (*heap).numEntries {
                    break;
                }
            } else if uqm_lua_checked!((*heap)
                .comparator
                .expect(
                    "non-null function pointer",
                )(
                *(*heap).entries.offset(childI.wrapping_add(1 as size_t) as isize),
                *(*heap).entries.offset(childI as isize),
            ), ()) < 0 as ::core::ffi::c_int
            {
                childI = childI.wrapping_add(1);
            }
            if uqm_lua_checked!((*heap)
                .comparator
                .expect(
                    "non-null function pointer",
                )(lastEntry, *(*heap).entries.offset(childI as isize)), ())
                <= 0 as ::core::ffi::c_int
            {
                break;
            }
            *(*heap).entries.offset(i as isize) = *(*heap)
                .entries
                .offset(childI as isize);
            (**(*heap).entries.offset(i as isize)).index = i as size_t;
            i = childI;
        }
        *(*heap).entries.offset(i as isize) = lastEntry;
        (**(*heap).entries.offset(i as isize)).index = i as size_t;
    }
    if (*heap).numEntries < (*heap).minFill && (*heap).numEntries > (*heap).minSize {
        Heap_resize(heap, (*heap).size.wrapping_div(2 as size_t));
    }
}
#[no_mangle]
pub unsafe extern "C" fn Heap_pop(mut heap: *mut Heap) -> *mut HeapValue {
    let mut result: *mut HeapValue = ::core::ptr::null_mut::<HeapValue>();
    ((*heap).numEntries > 0 as size_t
        || {
            __assert_fail(
                b"heap->numEntries > 0\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/libs/heap/heap.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                156 as ::core::ffi::c_int,
                b"Heap_pop\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    result = *(*heap).entries.offset(0isize);
    uqm_lua_checked!(Heap_removeByIndex(heap, 0 as size_t), ::core::ptr::null_mut());
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn Heap_count(mut heap: *const Heap) -> size_t {
    return (*heap).numEntries;
}
#[no_mangle]
pub unsafe extern "C" fn Heap_hasMore(mut heap: *const Heap) -> bool {
    return (*heap).numEntries > 0 as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn Heap_remove(mut heap: *mut Heap, mut value: *mut HeapValue) {
    uqm_lua_checked!(Heap_removeByIndex(heap, (*value).index), ());
}
#[inline]
unsafe extern "C" fn nextPower2(mut x: size_t) -> size_t {
    x = x.wrapping_sub(1);
    x |= x >> 1 as ::core::ffi::c_int;
    x |= x >> 2 as ::core::ffi::c_int;
    x |= x >> 4 as ::core::ffi::c_int;
    x |= x >> 8 as ::core::ffi::c_int;
    x |= x >> 16 as ::core::ffi::c_int;
    return x.wrapping_add(1 as size_t);
}
