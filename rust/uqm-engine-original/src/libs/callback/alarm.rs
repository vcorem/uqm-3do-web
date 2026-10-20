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

// Rust translation of src/libs/callback/alarm.c.
extern "C" {
    fn malloc(_: size_t) -> *mut ::core::ffi::c_void;
    fn free(_: *mut ::core::ffi::c_void);
    fn SDL_GetTicks() -> Uint32;
    fn Heap_new(
        comparator: HeapValue_Comparator,
        initialSize: size_t,
        minSize: size_t,
        minFillQuotient: ::core::ffi::c_double,
    ) -> *mut Heap;
    fn Heap_delete(heap: *mut Heap);
    fn Heap_add(heap: *mut Heap, value: *mut HeapValue);
    fn Heap_first(heap: *const Heap) -> *mut HeapValue;
    fn Heap_pop(heap: *mut Heap) -> *mut HeapValue;
    fn Heap_hasMore(heap: *const Heap) -> bool;
    fn Heap_remove(heap: *mut Heap, value: *mut HeapValue);
    fn __assert_fail(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    );
}
pub type size_t = usize;
pub type uint32_t = ::core::ffi::c_uint;
pub type uint32 = ::core::ffi::c_uint;
pub type AlarmTime = uint32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Alarm {
    pub index: size_t,
    pub time: AlarmTime,
    pub callback: AlarmCallback,
    pub arg: AlarmCallbackArg,
}
pub type AlarmCallbackArg = *mut ::core::ffi::c_void;
pub type AlarmCallback = Option<unsafe extern "C" fn(AlarmCallbackArg) -> ()>;
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
pub type Uint32 = uint32_t;
#[inline]
unsafe extern "C" fn alarmTimeToMsUint32(mut time: AlarmTime) -> uint32 {
    return time;
}
#[no_mangle]
pub static mut alarmHeap: *mut Heap = ::core::ptr::null_mut::<Heap>();
#[inline]
unsafe extern "C" fn Alarm_alloc() -> *mut Alarm {
    return malloc(::core::mem::size_of::<Alarm>()) as *mut Alarm;
}
#[inline]
unsafe extern "C" fn Alarm_free(mut alarm: *mut Alarm) {
    free(alarm as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn AlarmTime_compare(
    t1: AlarmTime,
    t2: AlarmTime,
) -> ::core::ffi::c_int {
    if t1 < t2 {
        return -1 as ::core::ffi::c_int;
    }
    if t1 > t2 {
        return 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn Alarm_compare(
    mut a1: *const Alarm,
    mut a2: *const Alarm,
) -> ::core::ffi::c_int {
    return AlarmTime_compare((*a1).time, (*a2).time);
}
#[no_mangle]
pub unsafe extern "C" fn Alarm_init() {
    (alarmHeap.is_null()
        || {
            __assert_fail(
                b"alarmHeap == NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/libs/callback/alarm.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                57 as ::core::ffi::c_int,
                b"Alarm_init\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    alarmHeap = Heap_new(
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(*const Alarm, *const Alarm) -> ::core::ffi::c_int,
            >,
            HeapValue_Comparator,
        >(
            Some(
                Alarm_compare
                    as unsafe extern "C" fn(
                        *const Alarm,
                        *const Alarm,
                    ) -> ::core::ffi::c_int,
            ),
        ),
        4 as size_t,
        4 as size_t,
        0.8f64,
    );
}
#[no_mangle]
pub unsafe extern "C" fn Alarm_uninit() {
    (!alarmHeap.is_null()
        || {
            __assert_fail(
                b"alarmHeap != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/libs/callback/alarm.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                64 as ::core::ffi::c_int,
                b"Alarm_uninit\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    while Heap_hasMore(alarmHeap) {
        let mut alarm: *mut Alarm = Heap_pop(alarmHeap) as *mut Alarm;
        Alarm_free(alarm);
    }
    Heap_delete(alarmHeap);
    alarmHeap = ::core::ptr::null_mut::<Heap>();
}
#[inline]
unsafe extern "C" fn AlarmTime_nowMs() -> AlarmTime {
    return SDL_GetTicks();
}
#[no_mangle]
pub unsafe extern "C" fn Alarm_addAbsoluteMs(
    mut ms: uint32,
    mut callback: AlarmCallback,
    mut arg: AlarmCallbackArg,
) -> *mut Alarm {
    let mut alarm: *mut Alarm = ::core::ptr::null_mut::<Alarm>();
    (!alarmHeap.is_null()
        || {
            __assert_fail(
                b"alarmHeap != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/libs/callback/alarm.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                84 as ::core::ffi::c_int,
                b"Alarm_addAbsoluteMs\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    alarm = Alarm_alloc();
    (*alarm).time = ms as AlarmTime;
    (*alarm).callback = callback;
    (*alarm).arg = arg;
    Heap_add(alarmHeap, alarm as *mut HeapValue);
    return alarm;
}
#[no_mangle]
pub unsafe extern "C" fn Alarm_addRelativeMs(
    mut ms: uint32,
    mut callback: AlarmCallback,
    mut arg: AlarmCallbackArg,
) -> *mut Alarm {
    let mut alarm: *mut Alarm = ::core::ptr::null_mut::<Alarm>();
    (!alarmHeap.is_null()
        || {
            __assert_fail(
                b"alarmHeap != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/libs/callback/alarm.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                101 as ::core::ffi::c_int,
                b"Alarm_addRelativeMs\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    alarm = Alarm_alloc();
    (*alarm).time = AlarmTime_nowMs().wrapping_add(ms) as AlarmTime;
    (*alarm).callback = callback;
    (*alarm).arg = arg;
    Heap_add(alarmHeap, alarm as *mut HeapValue);
    return alarm;
}
#[no_mangle]
pub unsafe extern "C" fn Alarm_remove(mut alarm: *mut Alarm) {
    (!alarmHeap.is_null()
        || {
            __assert_fail(
                b"alarmHeap != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/libs/callback/alarm.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                115 as ::core::ffi::c_int,
                b"Alarm_remove\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    Heap_remove(alarmHeap, alarm as *mut HeapValue);
    Alarm_free(alarm);
}
#[no_mangle]
pub unsafe extern "C" fn Alarm_processOne() -> bool {
    let mut now: AlarmTime = 0;
    let mut alarm: *mut Alarm = ::core::ptr::null_mut::<Alarm>();
    (!alarmHeap.is_null()
        || {
            __assert_fail(
                b"alarmHeap != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/libs/callback/alarm.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                129 as ::core::ffi::c_int,
                b"Alarm_processOne\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    if !Heap_hasMore(alarmHeap) {
        return false;
    }
    now = AlarmTime_nowMs();
    alarm = Heap_first(alarmHeap) as *mut Alarm;
    if now < (*alarm).time {
        return false;
    }
    Heap_pop(alarmHeap);
    (*alarm).callback.expect("non-null function pointer")((*alarm).arg);
    Alarm_free(alarm);
    return true;
}
#[no_mangle]
pub unsafe extern "C" fn Alarm_timeBeforeNextMs() -> uint32 {
    let mut alarm: *mut Alarm = ::core::ptr::null_mut::<Alarm>();
    if !Heap_hasMore(alarmHeap) {
        return 0xffffffff as uint32;
    }
    alarm = Heap_first(alarmHeap) as *mut Alarm;
    return alarmTimeToMsUint32((*alarm).time);
}
