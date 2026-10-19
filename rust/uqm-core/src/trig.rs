// SPDX-License-Identifier: GPL-2.0-or-later
// Derived from sc2/src/uqm/trans.c and units.h.
// Copyright Paul Reiche and Fred Ford, 1992-2002.

//! UQM's discrete 64-angle trigonometry, including the exported sine table.
//!
//! Table entries are the exact integer results of the upstream decimal
//! constants multiplied by 16384 and truncated, not newly rounded samples.
//! HD changes the table/argument widths; its 32-bit ratio arithmetic is retained.

#![allow(non_snake_case)]

#[cfg(not(feature = "hd"))]
pub type TrigInput = i16;
#[cfg(feature = "hd")]
pub type TrigInput = i32;
#[cfg(not(feature = "hd"))]
pub type SinValue = i16;
#[cfg(feature = "hd")]
pub type SinValue = i32;

pub const FULL_CIRCLE: u16 = 64;
pub const QUADRANT: u16 = 16;
const HALF_CIRCLE: u16 = 32;
const SIN_SHIFT: u32 = 14;

pub(crate) fn widen_input(value: TrigInput) -> i32 {
    #[cfg(not(feature = "hd"))]
    {
        i32::from(value)
    }
    #[cfg(feature = "hd")]
    {
        value
    }
}

const SIN_TABLE: [SinValue; 64] = [
    -16384, -16305, -16069, -15678, -15136, -14449, -13622, -12664, -11585, -10393, -9102, -7723,
    -6269, -4756, -3196, -1605, 0, 1605, 3196, 4756, 6269, 7723, 9102, 10393, 11585, 12664, 13622,
    14449, 15136, 15678, 16069, 16305, 16384, 16305, 16069, 15678, 15136, 14449, 13622, 12664,
    11585, 10393, 9102, 7723, 6269, 4756, 3196, 1605, 0, -1605, -3196, -4756, -6269, -7723, -9102,
    -10393, -11585, -12664, -13622, -14449, -15136, -15678, -16069, -16305,
];

// Keep the original writable C object and its edition-specific element width.
// The engine never changes this table, but C callers can still address it using
// their unchanged extern declaration. Any mutation must be serialized with reads.
#[no_mangle]
pub static mut sinetab: [SinValue; 64] = SIN_TABLE;

const ATAN_TABLE: [u16; 33] = [
    0, 0, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 6, 6, 6, 6, 7, 7, 7, 7, 7, 7, 8, 8,
    8,
];

/// Quantize a direction with UQM's original lookup, quadrant and rounding rules.
/// Up is 0, right is 16, down is 32, left is 48; zero motion returns 64.
///
/// C's Original implementation cannot represent `abs(i16::MIN)`. A single MIN
/// input normally indexes outside its lookup or divides by zero. For those
/// undefined inputs this function uses the full unsigned magnitude. The pair
/// `(MIN, MIN)` retains the original compiler's defined unsigned-ratio result 0.
/// HD's signed-overflowing i32::MIN negation similarly receives a defined unsigned
/// magnitude. All defined HD ratios still use the original wrapping u32 shifts.
#[no_mangle]
pub extern "C" fn ARCTAN(delta_x: TrigInput, delta_y: TrigInput) -> u16 {
    #[cfg(not(feature = "hd"))]
    if delta_x == i16::MIN && delta_y == i16::MIN {
        return 0;
    }

    let x = widen_input(delta_x).unsigned_abs();
    let y = widen_input(delta_y).unsigned_abs();
    if x == 0 && y == 0 {
        return FULL_CIRCLE;
    }
    let mut angle = if x > y {
        let index = (y.wrapping_shl(5).wrapping_add(x >> 1) / x) as usize;
        QUADRANT - ATAN_TABLE[index]
    } else {
        let index = (x.wrapping_shl(5).wrapping_add(y >> 1) / y) as usize;
        ATAN_TABLE[index]
    } as i32;
    if delta_x < 0 {
        angle = i32::from(FULL_CIRCLE) - angle;
    }
    if delta_y > 0 {
        angle = i32::from(HALF_CIRCLE) - angle;
    }
    (angle as u32 & 63) as u16
}

/// Evaluate the upstream SINE macro before its final destination's narrowing.
/// The intermediate has C `long` width: 32 bits in Wasm, 64 on the macOS host.
/// Wrapping multiplication defines the existing target's overflow behavior;
/// defined C products are unchanged. No floating point is introduced.
///
/// # Safety
/// No other caller may modify `sinetab` while this function reads its entry.
pub unsafe fn scaled_sine(angle: u16, magnitude: i32) -> i32 {
    // SAFETY: Masking guarantees one of the 64 initialized table entries. The
    // caller guarantees no concurrent C writes; no reference to static mut forms.
    let value = unsafe {
        core::ptr::addr_of!(sinetab)
            .cast::<SinValue>()
            .add(usize::from(angle & 63))
            .read()
    };
    let product = (value as core::ffi::c_long).wrapping_mul(magnitude as core::ffi::c_long);
    (product >> SIN_SHIFT) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cardinal_diagonal_and_zero_angles_follow_original_coordinates() {
        for (x, y, angle) in [
            (0, 0, 64),
            (0, -1, 0),
            (1, -1, 8),
            (1, 0, 16),
            (1, 1, 24),
            (0, 1, 32),
            (-1, 1, 40),
            (-1, 0, 48),
            (-1, -1, 56),
        ] {
            assert_eq!(ARCTAN(x, y), angle);
        }
    }

    #[test]
    fn sine_table_preserves_exact_upstream_truncation() {
        assert_eq!(SIN_TABLE[0], -16_384);
        assert_eq!(SIN_TABLE[1], -16_305);
        assert_eq!(SIN_TABLE[16], 0);
        assert_eq!(SIN_TABLE[32], 16_384);
        for index in 0..32 {
            assert_eq!(SIN_TABLE[index], -SIN_TABLE[index + 32]);
        }
        // SAFETY: Unit tests do not mutate the exported sine table.
        unsafe {
            assert_eq!(scaled_sine(0, 32), -32);
            assert_eq!(scaled_sine(16, 32), 0);
            assert_eq!(scaled_sine(32, 32), 32);
            assert_eq!(scaled_sine(64, 32), -32);
            // Arithmetic right shift rounds negative fixed-point products down.
            assert_eq!(scaled_sine(1, 1), -1);
        }
    }

    #[test]
    fn minimum_inputs_are_bounded_without_lookup_memory_errors() {
        let minimum = TrigInput::MIN;
        for x in [minimum, minimum + 1, -1, 0, 1, TrigInput::MAX] {
            for y in [minimum, minimum + 1, -1, 0, 1, TrigInput::MAX] {
                assert!(ARCTAN(x, y) <= FULL_CIRCLE);
            }
        }
        #[cfg(not(feature = "hd"))]
        assert_eq!(ARCTAN(minimum, minimum), 0);
    }
}
