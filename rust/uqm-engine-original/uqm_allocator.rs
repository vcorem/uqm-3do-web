// SPDX-License-Identifier: GPL-2.0-or-later
// Rust engine integration support.
// Rust allocation uses the same Emscripten heap as the original engine.
use core::alloc::{GlobalAlloc, Layout};
struct EmscriptenAllocator;
extern "C" {
    fn malloc(size: usize) -> *mut core::ffi::c_void;
    fn free(pointer: *mut core::ffi::c_void);
    fn posix_memalign(out: *mut *mut core::ffi::c_void, align: usize, size: usize) -> i32;
}
unsafe impl GlobalAlloc for EmscriptenAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // Emscripten 3.1.50 malloc guarantees only max_align_t (8 bytes).
        if layout.align() <= 8 {
            malloc(layout.size().max(1)).cast()
        } else {
            let mut pointer = core::ptr::null_mut();
            if posix_memalign(&mut pointer, layout.align(), layout.size().max(1)) == 0 {
                pointer.cast()
            } else {
                core::ptr::null_mut()
            }
        }
    }
    unsafe fn dealloc(&self, pointer: *mut u8, _: Layout) { free(pointer.cast()); }
}
#[global_allocator]
static ALLOCATOR: EmscriptenAllocator = EmscriptenAllocator;
