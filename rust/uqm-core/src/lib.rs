// SPDX-License-Identifier: GPL-2.0-or-later
// Derived from UQM: Copyright Paul Reiche, Fred Ford, Robert Leyland,
// Scott Anderson and Serge van den Boom. See README.md for source provenance.

//! Deterministic engine routines used by the C game through its existing ABI.
//!
//! This crate deliberately has no allocator, system dependencies or floating
//! point operations. The C engine owns contexts and calls remain synchronous.
//! Integer widths, seed coercion and CRC byte order match the original engine.

#![cfg_attr(not(test), no_std)]
#![deny(unsafe_op_in_unsafe_fn)]
#![allow(non_snake_case)]

use core::ffi::c_void;
use core::sync::atomic::{AtomicU32, Ordering};

pub mod bbox;
pub mod geometry;
pub mod queue;
pub mod trig;
pub mod velocity;

const INITIAL_SEED: u32 = 12_345;
const RNG_A: u32 = 16_807;
const RNG_M: u32 = 2_147_483_647;
const RNG_Q: u32 = 127_773;
const RNG_R: u32 = 2_836;

// The C callers serialize game state. Atomic access also prevents a Rust data
// race if separate pthreads reach the global RNG. Relaxed order is sufficient:
// this number is not used to publish any other memory. A successful CAS defines
// one RNG step, preserving the original sequence for serialized callers.
static GLOBAL_SEED: AtomicU32 = AtomicU32::new(INITIAL_SEED);

/// Apply the C engine's single-subtraction seed normalization, including its
/// historical `u32::MAX -> 2_147_483_648` edge case. Do not replace with modulo.
pub const fn normalize_seed(seed: u32) -> u32 {
    if seed == 0 {
        1
    } else if seed > RNG_M {
        seed - RNG_M
    } else {
        seed
    }
}

/// Advance UQM's Park-Miller variant using the C DWORD arithmetic. Its unsigned
/// subtraction/correction differs from the textbook signed implementation.
pub const fn rng_next(seed: u32) -> u32 {
    let next = (RNG_A * (seed % RNG_Q)).wrapping_sub(RNG_R * (seed / RNG_Q));
    if next > RNG_M {
        next - RNG_M
    } else if next == 0 {
        1
    } else {
        next
    }
}

/// HD's configurable generator. `multiplier` is the validated `SeedA` value
/// from MegaMod (3..=2_147_483_644), or its default 16_807.
///
/// # Panics
/// Panics if the multiplier is outside the engine's defined valid range.
pub const fn rng_next_with_multiplier(seed: u32, multiplier: u32) -> u32 {
    assert!(multiplier >= 3 && multiplier <= RNG_M - 3);
    let quotient = RNG_M / multiplier;
    let remainder = RNG_M % multiplier;
    let next = multiplier
        .wrapping_mul(seed % quotient)
        .wrapping_sub(remainder.wrapping_mul(seed / quotient));
    normalize_seed(next)
}

#[cfg(feature = "hd")]
extern "C" {
    // A C adapter reads MegaMod's SeedA macro. Its configuration is serialized
    // with simulation updates and always yields a multiplier in the range above.
    fn uqm_hd_seed_a() -> u32;
}

fn configured_rng_next(seed: u32) -> u32 {
    #[cfg(feature = "hd")]
    {
        // SAFETY: The adapter reads the engine's current validated multiplier.
        rng_next_with_multiplier(seed, unsafe { uqm_hd_seed_a() })
    }
    #[cfg(not(feature = "hd"))]
    rng_next(seed)
}

#[no_mangle]
pub extern "C" fn TFB_Random() -> u32 {
    let mut old = GLOBAL_SEED.load(Ordering::Relaxed);
    loop {
        let next = configured_rng_next(old);
        match GLOBAL_SEED.compare_exchange_weak(old, next, Ordering::Relaxed, Ordering::Relaxed) {
            Ok(_) => return next,
            Err(current) => old = current,
        }
    }
}

#[no_mangle]
pub extern "C" fn TFB_SeedRandom(seed: u32) -> u32 {
    GLOBAL_SEED.swap(normalize_seed(seed), Ordering::Relaxed)
}

/// Layout of the opaque C `RandomContext` in `libs/math/random.h`.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RandomContext {
    pub seed: u32,
}

extern "C" {
    // UQM's HMalloc terminates on allocation failure. Using the same allocator
    // and deallocator keeps ownership entirely in the existing engine domain.
    fn HMalloc(size: usize) -> *mut c_void;
    fn HFree(pointer: *mut c_void);
}

#[no_mangle]
pub extern "C" fn RandomContext_New() -> *mut RandomContext {
    // SAFETY: HMalloc's engine contract supplies an aligned, non-null allocation
    // of the requested size or terminates the process. This initializes it once.
    unsafe {
        let context = HMalloc(core::mem::size_of::<RandomContext>()).cast::<RandomContext>();
        context.write(RandomContext { seed: INITIAL_SEED });
        context
    }
}

/// Allocate an HD context with the exact supplied bits, without normalization.
#[cfg(feature = "hd")]
#[no_mangle]
pub extern "C" fn RandomContext_Set(seed: u32) -> *mut RandomContext {
    // SAFETY: HMalloc returns aligned writable storage or terminates. HD's Set
    // operation intentionally differs from SeedRandom by accepting raw seeds.
    unsafe {
        let context = HMalloc(core::mem::size_of::<RandomContext>()).cast::<RandomContext>();
        context.write(RandomContext { seed });
        context
    }
}

#[cfg(feature = "hd")]
#[no_mangle]
pub extern "C" fn RandomContext_FastRandom(seed: u32) -> u32 {
    configured_rng_next(seed)
}

/// HD's inclusive range mapping. Widened arithmetic also defines the overflow
/// cases that the old C expression cannot represent, including the full i32
/// range. Defined C inputs produce the original result.
#[cfg(feature = "hd")]
#[no_mangle]
pub extern "C" fn RangeMinMax(minimum: i32, maximum: i32, random: u32) -> i32 {
    if minimum > maximum {
        return maximum;
    }
    let width = (i64::from(maximum) - i64::from(minimum) + 1) as u64;
    (i64::from(minimum) + (u64::from(random) % width) as i64) as i32
}

/// # Safety
/// `context` must be null or an allocation returned by `RandomContext_New` or
/// `RandomContext_Copy`, no longer in use and not already freed.
#[no_mangle]
pub unsafe extern "C" fn RandomContext_Delete(context: *mut RandomContext) {
    // SAFETY: Ownership is transferred back to the matching engine allocator.
    unsafe { HFree(context.cast()) };
}

/// # Safety
/// `source` must point to a live, initialized context readable for this call.
/// It must not be concurrently modified.
#[no_mangle]
pub unsafe extern "C" fn RandomContext_Copy(source: *const RandomContext) -> *mut RandomContext {
    // SAFETY: The source meets the caller contract; HMalloc has the same
    // allocation contract as in RandomContext_New. The new context is separate.
    unsafe {
        let copy = HMalloc(core::mem::size_of::<RandomContext>()).cast::<RandomContext>();
        copy.write(source.read());
        copy
    }
}

/// # Safety
/// `context` must point to a live, initialized, writable context. Calls touching
/// that context must be serialized, as for the original C API.
#[no_mangle]
pub unsafe extern "C" fn RandomContext_Random(context: *mut RandomContext) -> u32 {
    // SAFETY: The caller supplies an exclusively accessible context.
    unsafe {
        let next = configured_rng_next((*context).seed);
        (*context).seed = next;
        next
    }
}

/// # Safety
/// `context` must point to a live, initialized, writable context. Calls touching
/// that context must be serialized, as for the original C API.
#[no_mangle]
pub unsafe extern "C" fn RandomContext_SeedRandom(context: *mut RandomContext, seed: u32) -> u32 {
    // SAFETY: The caller supplies an exclusively accessible context.
    unsafe {
        let old = (*context).seed;
        (*context).seed = normalize_seed(seed);
        old
    }
}

/// # Safety
/// `context` must point to a live, initialized context readable for this call.
/// It must not be concurrently modified.
#[no_mangle]
pub unsafe extern "C" fn RandomContext_GetSeed(context: *const RandomContext) -> u32 {
    // SAFETY: The caller supplies a readable context with no concurrent writes.
    unsafe { (*context).seed }
}

/// Floor of the unsigned 32-bit square root, computed entirely with integers.
/// The restoring algorithm uses at most sixteen rounds and fits in `u32`.
pub const fn integer_square_root(mut value: u32) -> u16 {
    let mut result = 0;
    let mut bit = 1_u32 << 30;
    while bit > value {
        bit >>= 2;
    }
    while bit != 0 {
        if value >= result + bit {
            value -= result + bit;
            result = (result >> 1) + bit;
        } else {
            result >>= 1;
        }
        bit >>= 2;
    }
    result as u16
}

#[no_mangle]
pub extern "C" fn square_root(value: u32) -> u16 {
    integer_square_root(value)
}

/// HD's NUL-terminated text checksum, retaining the C implementation's signed
/// `char` promotion on platforms where char is signed. Non-ASCII bytes can
/// therefore differ from the ordinary byte CRC API; this is intentional.
///
/// # Safety
/// `text` must identify a readable, NUL-terminated array of C chars that remains
/// live and unchanged throughout this call. Null is not a valid string pointer.
#[cfg(feature = "hd")]
#[no_mangle]
pub unsafe extern "C" fn crc32b(text: *const core::ffi::c_char) -> u32 {
    let mut pointer = text;
    let mut crc = u32::MAX;
    loop {
        // SAFETY: The caller supplies a valid NUL-terminated string.
        let character = unsafe { pointer.read() };
        if character == 0 {
            return !crc;
        }
        // C promotes char to int, then converts it to unsigned int. Preserve
        // sign extension before the XOR instead of treating char as uint8.
        crc ^= character as i32 as u32;
        for _ in 0..8 {
            crc = (crc >> 1) ^ (0xedb8_8320 & 0_u32.wrapping_sub(crc & 1));
        }
        // SAFETY: A non-NUL character is followed by at least one readable
        // character in the caller's NUL-terminated allocation.
        pointer = unsafe { pointer.add(1) };
    }
}

const fn make_crc_table() -> [u32; 256] {
    let mut table = [0; 256];
    let mut index = 0;
    while index < table.len() {
        let mut crc = index as u32;
        let mut bit = 0;
        while bit < 8 {
            crc = if crc & 1 != 0 {
                (crc >> 1) ^ 0xedb8_8320
            } else {
                crc >> 1
            };
            bit += 1;
        }
        table[index] = crc;
        index += 1;
    }
    table
}

const CRC_TABLE: [u32; 256] = make_crc_table();

#[inline]
fn crc_byte(crc: u32, value: u8) -> u32 {
    (crc >> 8) ^ CRC_TABLE[((crc ^ u32::from(value)) & 0xff) as usize]
}

/// Update an unfinalized CRC-32/ISO-HDLC state (`0xffffffff` initially).
pub fn crc_update(mut crc: u32, bytes: &[u8]) -> u32 {
    for &byte in bytes {
        crc = crc_byte(crc, byte);
    }
    crc
}

/// Layout of C `crc_State` from `uqm/supermelee/netplay/crc.h`.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CrcState {
    pub crc: u32,
}

/// # Safety
/// `state` must be aligned and writable for one `CrcState`, with no concurrent
/// accesses. Initialization of its previous contents is not required.
#[no_mangle]
pub unsafe extern "C" fn crc_init(state: *mut CrcState) {
    // SAFETY: The caller supplies storage for one state.
    unsafe { state.write(CrcState { crc: u32::MAX }) };
}

/// Process bytes in their supplied order. This corrects the unused C routine's
/// missing pointer increment; the original loops forever for nonempty input.
///
/// # Safety
/// `state` must be a live, initialized, writable state with no concurrent
/// accesses. For nonzero `length`, `buffer` must identify `length` readable
/// bytes in a valid allocation. A null buffer is permitted when `length` is zero.
/// The buffer may overlap the state: the state is written only after reading.
#[no_mangle]
pub unsafe extern "C" fn crc_processBytes(state: *mut CrcState, buffer: *mut u8, length: usize) {
    // SAFETY: Raw accesses preserve the C API's possible state/buffer aliasing.
    // The caller guarantees both regions and serialized access. No reference or
    // slice is constructed, including for the permitted null/zero-length case.
    unsafe {
        let mut crc = (*state).crc;
        for index in 0..length {
            crc = crc_byte(crc, buffer.add(index).read());
        }
        (*state).crc = crc;
    }
}

/// # Safety
/// `state` must be a live, initialized, writable state; all its accesses must
/// be serialized for the duration of this call.
#[no_mangle]
pub unsafe extern "C" fn crc_processUint8(state: *mut CrcState, value: u8) {
    // SAFETY: The caller supplies an exclusively accessible initialized state.
    unsafe { (*state).crc = crc_byte((*state).crc, value) };
}

/// # Safety
/// `state` must be a live, initialized, writable state; all its accesses must
/// be serialized for the duration of this call.
#[no_mangle]
pub unsafe extern "C" fn crc_processUint16(state: *mut CrcState, value: u16) {
    // SAFETY: The caller supplies an exclusively accessible initialized state.
    unsafe { (*state).crc = crc_update((*state).crc, &value.to_le_bytes()) };
}

/// # Safety
/// `state` must be a live, initialized, writable state; all its accesses must
/// be serialized for the duration of this call.
#[no_mangle]
pub unsafe extern "C" fn crc_processUint32(state: *mut CrcState, value: u32) {
    // SAFETY: The caller supplies an exclusively accessible initialized state.
    unsafe { (*state).crc = crc_update((*state).crc, &value.to_le_bytes()) };
}

/// # Safety
/// `state` must point to a live, initialized, readable state and must not be
/// concurrently modified during this call.
#[no_mangle]
pub unsafe extern "C" fn crc_finish(state: *const CrcState) -> u32 {
    // SAFETY: The caller supplies an initialized state readable for this call.
    unsafe { !(*state).crc }
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_information: &core::panic::PanicInfo<'_>) -> ! {
    // These routines validate their own arithmetic and indexing invariants; a
    // panic is an internal programming error and must never unwind into C.
    #[cfg(target_arch = "wasm32")]
    core::arch::wasm32::unreachable();

    #[cfg(not(target_arch = "wasm32"))]
    {
        extern "C" {
            fn abort() -> !;
        }
        // SAFETY: C abort has no preconditions and cannot return.
        unsafe { abort() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test-only allocator bridge. Production always resolves the game's
    // HMalloc/HFree, retaining its fatal allocation-failure policy.
    mod allocator {
        use core::ffi::c_void;
        extern "C" {
            fn malloc(size: usize) -> *mut c_void;
            fn free(pointer: *mut c_void);
        }

        #[no_mangle]
        unsafe extern "C" fn HMalloc(size: usize) -> *mut c_void {
            // SAFETY: malloc accepts any size; tests allocate contexts or queues.
            let pointer = unsafe { malloc(size) };
            assert!(!pointer.is_null());
            pointer
        }

        #[no_mangle]
        unsafe extern "C" fn HFree(pointer: *mut c_void) {
            // SAFETY: The tests free only allocations from the stub above.
            unsafe { free(pointer) };
        }
    }

    #[cfg(feature = "hd")]
    #[no_mangle]
    extern "C" fn uqm_hd_seed_a() -> u32 {
        RNG_A
    }

    #[test]
    fn seed_normalization_retains_original_boundaries() {
        assert_eq!(normalize_seed(0), 1);
        assert_eq!(normalize_seed(1), 1);
        assert_eq!(normalize_seed(RNG_M), RNG_M);
        assert_eq!(normalize_seed(RNG_M + 1), 1);
        assert_eq!(normalize_seed(u32::MAX - 1), RNG_M);
        assert_eq!(normalize_seed(u32::MAX), RNG_M + 1);
    }

    #[test]
    fn generator_preserves_uqm_unsigned_park_miller_variant() {
        let mut seed = 1;
        for expected in [
            16_807,
            282_475_249,
            1_622_650_073,
            984_943_658,
            1_144_108_930,
        ] {
            seed = rng_next(seed);
            assert_eq!(seed, expected);
        }
        seed = 1;
        for _ in 0..10_000 {
            seed = rng_next(seed);
        }
        // The textbook signed implementation produces 1_043_618_065 here;
        // UQM's unsigned wrap followed by subtraction of M is intentionally
        // preserved, rather than "correcting" the existing game sequence.
        assert_eq!(seed, 1_044_743_993);
        assert_eq!(rng_next(0), 1);
        assert_eq!(rng_next(RNG_M), 1);
    }

    #[test]
    fn configurable_rng_matches_default_and_wraps_without_panics() {
        for seed in [0, 1, 12_345, RNG_M - 1, RNG_M, RNG_M + 1, u32::MAX] {
            assert_eq!(rng_next_with_multiplier(seed, RNG_A), rng_next(seed));
            for multiplier in [3, 4, 100, 16_807, 1_000_000, RNG_M - 4, RNG_M - 3] {
                let result = rng_next_with_multiplier(seed, multiplier);
                assert!(result > 0 && result <= RNG_M + 1);
            }
        }
    }

    #[cfg(feature = "hd")]
    #[test]
    fn hd_raw_context_and_fast_rng_keep_original_semantics() {
        // SAFETY: Each fresh context is read, advanced and freed serially once.
        unsafe {
            for seed in [0, 1, RNG_M, RNG_M + 1, u32::MAX] {
                let context = RandomContext_Set(seed);
                assert_eq!(RandomContext_GetSeed(context), seed);
                assert_eq!(
                    RandomContext_Random(context),
                    RandomContext_FastRandom(seed)
                );
                RandomContext_Delete(context);
            }
        }
    }

    #[cfg(feature = "hd")]
    #[test]
    fn hd_inclusive_range_mapping_handles_bounds() {
        assert_eq!(RangeMinMax(4, 3, u32::MAX), 3);
        assert_eq!(RangeMinMax(-3, -3, u32::MAX), -3);
        assert_eq!(RangeMinMax(-3, 3, 0), -3);
        assert_eq!(RangeMinMax(-3, 3, 6), 3);
        assert_eq!(RangeMinMax(-3, 3, 7), -3);
        assert_eq!(RangeMinMax(i32::MIN, i32::MAX, 0), i32::MIN);
        assert_eq!(RangeMinMax(i32::MIN, i32::MAX, u32::MAX), i32::MAX);
        assert_eq!(RangeMinMax(i32::MAX, i32::MAX, 1), i32::MAX);
    }

    #[cfg(feature = "hd")]
    #[test]
    fn hd_text_crc_preserves_char_promotion() {
        let ascii = b"123456789\0";
        let non_ascii = [0x80_u8, 0xff, b'A', 0];
        // SAFETY: Both arrays are readable NUL-terminated C character strings.
        unsafe {
            assert_eq!(crc32b(ascii.as_ptr().cast()), 0xcbf4_3926);
            let expected = if core::ffi::c_char::MIN < 0 {
                0x1faf_4c94
            } else {
                0x8c8b_4ee6
            };
            assert_eq!(crc32b(non_ascii.as_ptr().cast()), expected);
        }
    }

    #[test]
    fn global_seed_swap_and_restore_preserve_sequence() {
        let old = TFB_SeedRandom(1);
        assert_eq!(TFB_Random(), 16_807);
        assert_eq!(TFB_SeedRandom(0), 16_807);
        assert_eq!(TFB_Random(), 16_807);
        assert_eq!(TFB_SeedRandom(u32::MAX), 16_807);
        assert_eq!(TFB_Random(), rng_next(RNG_M + 1));
        TFB_SeedRandom(old);
    }

    #[test]
    fn allocated_contexts_copy_state_and_evolve_independently() {
        // SAFETY: All contexts come from this API, are serialized and freed once.
        unsafe {
            let original = RandomContext_New();
            assert_eq!(RandomContext_GetSeed(original), INITIAL_SEED);
            assert_eq!(RandomContext_SeedRandom(original, 1), INITIAL_SEED);
            assert_eq!(RandomContext_Random(original), 16_807);
            let copy = RandomContext_Copy(original);
            assert_ne!(copy, original);
            assert_eq!(RandomContext_GetSeed(copy), 16_807);
            let next = RandomContext_Random(copy);
            assert_eq!(RandomContext_GetSeed(original), 16_807);
            assert_eq!(RandomContext_Random(original), next);
            RandomContext_Delete(original);
            RandomContext_Delete(copy);
            RandomContext_Delete(core::ptr::null_mut());
        }
    }

    #[test]
    fn square_root_is_exact_at_every_square_boundary() {
        assert_eq!(square_root(0), 0);
        assert_eq!(square_root(u32::MAX), u16::MAX);
        for root in 1..=u16::MAX as u32 {
            let squared = root * root;
            assert_eq!(square_root(squared - 1), (root - 1) as u16);
            assert_eq!(square_root(squared), root as u16);
            assert_eq!(square_root(squared + 1), root as u16);
        }
    }

    #[test]
    fn crc_matches_standard_vector_and_chunking() {
        assert_eq!(!crc_update(u32::MAX, b""), 0);
        assert_eq!(!crc_update(u32::MAX, b"123456789"), 0xcbf4_3926);
        let crc = crc_update(u32::MAX, b"1234");
        assert_eq!(!crc_update(crc, b"56789"), 0xcbf4_3926);
    }

    #[test]
    fn scalar_crc_uses_original_little_endian_wire_order() {
        let mut state = CrcState { crc: 0 };
        // SAFETY: A local initialized state is accessed serially.
        unsafe {
            crc_init(&mut state);
            crc_processUint8(&mut state, b'1');
            crc_processUint16(&mut state, u16::from_le_bytes(*b"23"));
            crc_processUint32(&mut state, u32::from_le_bytes(*b"4567"));
            crc_processUint16(&mut state, u16::from_le_bytes(*b"89"));
            assert_eq!(crc_finish(&state), 0xcbf4_3926);
        }
    }

    #[test]
    fn bytes_crc_advances_and_accepts_null_empty_buffer() {
        let mut state = CrcState { crc: u32::MAX };
        let mut bytes = *b"123456789";
        // SAFETY: Valid local state and buffer; null is used only with length 0.
        unsafe {
            crc_processBytes(&mut state, core::ptr::null_mut(), 0);
            assert_eq!(state.crc, u32::MAX);
            crc_processBytes(&mut state, bytes.as_mut_ptr(), bytes.len());
            assert_eq!(crc_finish(&state), 0xcbf4_3926);
        }
    }

    #[test]
    fn bytes_crc_permits_state_buffer_overlap() {
        let mut state = CrcState { crc: 0x1234_5678 };
        let expected = crc_update(state.crc, &state.crc.to_ne_bytes());
        let pointer = &mut state as *mut CrcState;
        // SAFETY: Both regions are the same initialized four-byte allocation;
        // the API promises to defer the state write until the reads finish.
        unsafe { crc_processBytes(pointer, pointer.cast::<u8>(), 4) };
        assert_eq!(state.crc, expected);
    }

    #[test]
    fn ffi_layout_matches_original_scalar_structs() {
        assert_eq!(core::mem::size_of::<RandomContext>(), 4);
        assert_eq!(core::mem::align_of::<RandomContext>(), 4);
        assert_eq!(core::mem::size_of::<CrcState>(), 4);
        assert_eq!(core::mem::align_of::<CrcState>(), 4);
    }
}
