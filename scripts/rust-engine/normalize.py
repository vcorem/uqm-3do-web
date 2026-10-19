"""Mechanical compatibility fixes for generated Wasm Rust, not game logic.

Run after init-fix.py. Output is written only to the explicitly selected generated tree.
"""
import json
from pathlib import Path
import re
import sys

root = Path(sys.argv[1]).resolve()
changes = {}
for path in sorted(root.rglob('*.rs')):
    if 'target' in path.relative_to(root).parts:
        continue
    old = path.read_text()
    new = old.replace('::core::ptr::from_exposed_addr_mut', '::core::ptr::with_exposed_provenance_mut')
    new = new.replace('::core::ptr::from_exposed_addr', '::core::ptr::with_exposed_provenance')
    new = new.replace('.expose_addr()', '.expose_provenance()')
    new = new.replace('::std::vec::from_elem', '::alloc::vec::from_elem')
    if re.search(r'\bVec[<:]', new) and 'use alloc::vec::Vec;' not in new:
        new = 'use alloc::vec::Vec;\n' + new
    if path.name == 'lib.rs':
        if '#![no_std]' not in new:
            new = '#![no_std]\n' + new
        new = re.sub(r'#!\[feature\((?:stdsimd|raw_ref_op|strict_provenance)\)\]\n', '', new)
        new = re.sub(r'pub mod (\d\w*);', lambda m: '#[path = "' + m[1] + '.rs"]\npub mod module_' + m[1] + ';', new)
        if 'extern crate alloc;' not in new:
            pos = new.index('pub mod src')
            new = new[:pos] + 'extern crate alloc;\nmod uqm_allocator;\nmod uqm_engine_init;\n\n' + new[pos:]
        if '#[panic_handler]' not in new:
            new += '\n#[panic_handler]\nfn panic(_: &core::panic::PanicInfo) -> ! { core::arch::wasm32::unreachable() }\n'
    if new != old:
        path.write_text(new)
        changes[str(path.relative_to(root))] = {'before': len(old), 'after': len(new)}
manifest = root / 'Cargo.toml'
text = manifest.read_text().replace('c2rust-bitfields = "0.22.1"', 'c2rust-bitfields = { version = "=0.22.1", features = ["no_std"] }')
manifest.write_text(text)
(root / 'uqm_allocator.rs').write_text('''// Rust allocation uses the same Emscripten heap as the original engine.
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
''')
(root / 'normalization-report.json').write_text(json.dumps(changes, indent=2)+'\n')
print('Normalized', len(changes), 'Rust source files in', root)
