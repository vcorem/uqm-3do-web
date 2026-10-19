// Appended to the unchanged production allocator by test-allocator.sh.
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! { core::arch::wasm32::unreachable() }

#[no_mangle]
pub unsafe extern "C" fn rust_allocator_case(align: usize, size: usize) -> i32 {
    let layout = Layout::from_size_align(size, align).unwrap();
    let pointer = ALLOCATOR.alloc(layout);
    if pointer.is_null() { return 1; }
    if pointer as usize % align != 0 { ALLOCATOR.dealloc(pointer, layout); return 2; }
    for i in 0..size { pointer.add(i).write((i as u8).wrapping_mul(37).wrapping_add(11)); }
    let larger_size = size * 3 + 17;
    let larger = ALLOCATOR.realloc(pointer, layout, larger_size);
    if larger.is_null() { ALLOCATOR.dealloc(pointer, layout); return 3; }
    let larger_layout = Layout::from_size_align(larger_size, align).unwrap();
    if larger as usize % align != 0 { ALLOCATOR.dealloc(larger, larger_layout); return 4; }
    for i in 0..size {
        if larger.add(i).read() != (i as u8).wrapping_mul(37).wrapping_add(11) {
            ALLOCATOR.dealloc(larger, larger_layout); return 5;
        }
    }
    let smaller_size = (size / 2).max(1);
    let smaller = ALLOCATOR.realloc(larger, larger_layout, smaller_size);
    if smaller.is_null() { ALLOCATOR.dealloc(larger, larger_layout); return 6; }
    let smaller_layout = Layout::from_size_align(smaller_size, align).unwrap();
    if smaller as usize % align != 0 { ALLOCATOR.dealloc(smaller, smaller_layout); return 7; }
    for i in 0..smaller_size {
        if smaller.add(i).read() != (i as u8).wrapping_mul(37).wrapping_add(11) {
            ALLOCATOR.dealloc(smaller, smaller_layout); return 8;
        }
    }
    ALLOCATOR.dealloc(smaller, smaller_layout);
    let zero = ALLOCATOR.alloc_zeroed(layout);
    if zero.is_null() { return 9; }
    if zero as usize % align != 0 { ALLOCATOR.dealloc(zero, layout); return 10; }
    for i in 0..size {
        if zero.add(i).read() != 0 { ALLOCATOR.dealloc(zero, layout); return 11; }
    }
    ALLOCATOR.dealloc(zero, layout);
    0
}
