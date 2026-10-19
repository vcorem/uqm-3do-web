// SPDX-License-Identifier: GPL-2.0-or-later
// Derived from sc2/src/uqm/velocity.c, velocity.h and the pinned MegaMod variant.
// Copyright Paul Reiche and Fred Ford, 1992-2002.

//! Exact fixed-point velocity representation behind the existing C API.
//!
//! Arithmetic retains the original casts to signed/unsigned 8/16/32-bit values
//! and the sequence of writes. Explicit wrapping defines historical C signed
//! overflow/negative-left-shift cases to match the deployed Wasm arithmetic.
//! Pointer APIs do not create Rust references, preserving legal C output aliasing.

#![allow(non_snake_case)]

use crate::trig::{self, FULL_CIRCLE, QUADRANT};

#[cfg(not(feature = "hd"))]
pub type VelocityInput = i16;
#[cfg(feature = "hd")]
pub type VelocityInput = i32;

const SHIFT: u32 = 5;
const REMAINDER: i32 = 31;

/// The engine's `EXTENT`, including in HD, contains signed 16-bit coordinates.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Extent {
    pub width: i16,
    pub height: i16,
}

/// Exact `VELOCITY_DESC` layout: 18 bytes, alignment 2, in both engines.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct VelocityDesc {
    pub TravelAngle: u16,
    pub vector: Extent,
    pub fract: Extent,
    pub error: Extent,
    pub incr: Extent,
}

fn current_component(vector: i16, fract: i16, incr: i16) -> i32 {
    (i32::from(vector) << SHIFT) + i32::from(fract) - i32::from((incr as u16) >> 8)
}

// The incoming negative magnitude has already undergone the appropriate C
// narrowing: SIZE for vector trig outputs, SIZE/SDWORD for edition components.
fn axis_parts(value: i32, negated: i32) -> (i16, i16, i16) {
    if value >= 0 {
        ((value >> SHIFT) as i16, (value & REMAINDER) as i16, 1)
    } else {
        (
            (-(negated >> SHIFT)) as i16,
            (negated & REMAINDER) as i16,
            (((negated & REMAINDER) << 9) | 0xff) as i16,
        )
    }
}

fn from_parts(angle: u16, x: (i16, i16, i16), y: (i16, i16, i16)) -> VelocityDesc {
    VelocityDesc {
        TravelAngle: angle,
        vector: Extent {
            width: x.0,
            height: y.0,
        },
        fract: Extent {
            width: x.1,
            height: y.1,
        },
        error: Extent::default(),
        incr: Extent {
            width: x.2,
            height: y.2,
        },
    }
}

/// # Safety
/// `velocity` must point to a live initialized descriptor, and `dx`/`dy` to
/// writable i16 values. All accesses must be serialized. Output pointers may
/// alias each other or descriptor fields; the original x-then-y order is kept.
#[no_mangle]
pub unsafe extern "C" fn GetCurrentVelocityComponents(
    velocity: *mut VelocityDesc,
    dx: *mut i16,
    dy: *mut i16,
) {
    // SAFETY: Raw accesses follow the caller's validity/serialization contract.
    unsafe {
        dx.write(current_component(
            (*velocity).vector.width,
            (*velocity).fract.width,
            (*velocity).incr.width,
        ) as i16);
        dy.write(current_component(
            (*velocity).vector.height,
            (*velocity).fract.height,
            (*velocity).incr.height,
        ) as i16);
    }
}

/// # Safety
/// `velocity` must point to a live initialized descriptor, and `dx`/`dy` to
/// writable i32 values. All accesses must be serialized; outputs can alias.
#[cfg(feature = "hd")]
#[no_mangle]
pub unsafe extern "C" fn GetCurrentVelocityComponentsSdword(
    velocity: *mut VelocityDesc,
    dx: *mut i32,
    dy: *mut i32,
) {
    // SAFETY: The caller provides valid storage and serialized raw access.
    unsafe {
        dx.write(current_component(
            (*velocity).vector.width,
            (*velocity).fract.width,
            (*velocity).incr.width,
        ));
        dy.write(current_component(
            (*velocity).vector.height,
            (*velocity).fract.height,
            (*velocity).incr.height,
        ));
    }
}

/// # Safety
/// `velocity` must be live, initialized and writable; `dx`/`dy` must be writable
/// i16 outputs. Accesses must be serialized. Output aliasing follows C's exact
/// order: write x, update x error, write y, update y error.
#[no_mangle]
pub unsafe extern "C" fn GetNextVelocityComponents(
    velocity: *mut VelocityDesc,
    dx: *mut i16,
    dy: *mut i16,
    frames: u16,
) {
    // SAFETY: The caller provides all valid pointers. Each y input is read only
    // after completing the x writes, preserving the C function's alias behavior.
    unsafe {
        let error = ((*velocity).error.width as u16)
            .wrapping_add(((*velocity).fract.width as u16).wrapping_mul(frames));
        let value = i32::from((*velocity).vector.width)
            .wrapping_mul(i32::from(frames))
            .wrapping_add(
                i32::from((*velocity).incr.width as u8 as i8) * i32::from(error >> SHIFT),
            );
        dx.write(value as i16);
        (*velocity).error.width = (error & 31) as i16;

        let error = ((*velocity).error.height as u16)
            .wrapping_add(((*velocity).fract.height as u16).wrapping_mul(frames));
        let value = i32::from((*velocity).vector.height)
            .wrapping_mul(i32::from(frames))
            .wrapping_add(
                i32::from((*velocity).incr.height as u8 as i8) * i32::from(error >> SHIFT),
            );
        dy.write(value as i16);
        (*velocity).error.height = (error & 31) as i16;
    }
}

/// # Safety
/// `velocity` must be live, initialized and writable; `dx`/`dy` must be writable
/// i32 outputs. All accesses must be serialized. Writes retain the C order.
#[cfg(feature = "hd")]
#[no_mangle]
pub unsafe extern "C" fn GetNextVelocityComponentsSdword(
    velocity: *mut VelocityDesc,
    dx: *mut i32,
    dy: *mut i32,
    frames: u32,
) {
    // SAFETY: Raw pointer accesses meet the caller's validity contract. DWORD
    // promotions make the HD calculation unsigned before its final SDWORD cast.
    unsafe {
        let error = ((*velocity).error.width as u32)
            .wrapping_add(((*velocity).fract.width as u32).wrapping_mul(frames));
        let value = ((*velocity).vector.width as u32)
            .wrapping_mul(frames)
            .wrapping_add(((*velocity).incr.width as u8 as i8 as u32).wrapping_mul(error >> SHIFT));
        dx.write(value as i32);
        (*velocity).error.width = (error & 31) as i16;

        let error = ((*velocity).error.height as u32)
            .wrapping_add(((*velocity).fract.height as u32).wrapping_mul(frames));
        let value = ((*velocity).vector.height as u32)
            .wrapping_mul(frames)
            .wrapping_add(
                ((*velocity).incr.height as u8 as i8 as u32).wrapping_mul(error >> SHIFT),
            );
        dy.write(value as i32);
        (*velocity).error.height = (error & 31) as i16;
    }
}

/// # Safety
/// `velocity` must point to aligned writable storage for one descriptor, with
/// no concurrent accesses. No caller may mutate `sinetab` during this call.
#[no_mangle]
pub unsafe extern "C" fn SetVelocityVector(
    velocity: *mut VelocityDesc,
    magnitude: VelocityInput,
    facing: u16,
) {
    let angle = (facing & 15) << 2;
    // Original narrows magnitude to i16 immediately after shifting; HD retains
    // an i32 intermediate. Both then narrow each trig result to SIZE (i16).
    let magnitude = magnitude.wrapping_shl(SHIFT);
    // SAFETY: The caller serializes table reads and supplies writable storage.
    unsafe {
        let x = trig::scaled_sine(angle + QUADRANT, trig::widen_input(magnitude)) as i16;
        let y = trig::scaled_sine(angle, trig::widen_input(magnitude)) as i16;
        velocity.write(from_parts(
            angle,
            axis_parts(i32::from(x), i32::from(x.wrapping_neg())),
            axis_parts(i32::from(y), i32::from(y.wrapping_neg())),
        ));
    }
}

/// # Safety
/// `velocity` must identify aligned writable storage for one descriptor, with
/// no concurrent accesses. Previous initialization is not required.
#[no_mangle]
pub unsafe extern "C" fn SetVelocityComponents(
    velocity: *mut VelocityDesc,
    dx: VelocityInput,
    dy: VelocityInput,
) {
    let angle = trig::ARCTAN(dx, dy);
    let descriptor = if angle == FULL_CIRCLE {
        VelocityDesc {
            TravelAngle: angle,
            ..VelocityDesc::default()
        }
    } else {
        from_parts(
            angle,
            axis_parts(trig::widen_input(dx), trig::widen_input(dx.wrapping_neg())),
            axis_parts(trig::widen_input(dy), trig::widen_input(dy.wrapping_neg())),
        )
    };
    // SAFETY: The caller provides aligned storage for one descriptor.
    unsafe { velocity.write(descriptor) };
}

/// # Safety
/// `velocity` must point to one live initialized writable descriptor, accessed
/// exclusively for this call.
#[no_mangle]
pub unsafe extern "C" fn DeltaVelocityComponents(
    velocity: *mut VelocityDesc,
    dx: VelocityInput,
    dy: VelocityInput,
) {
    // SAFETY: The caller provides a valid descriptor. Reading both current
    // components before Set preserves C's local-value-then-write sequence.
    unsafe {
        let x = dx.wrapping_add(current_component(
            (*velocity).vector.width,
            (*velocity).fract.width,
            (*velocity).incr.width,
        ) as VelocityInput);
        let y = dy.wrapping_add(current_component(
            (*velocity).vector.height,
            (*velocity).fract.height,
            (*velocity).incr.height,
        ) as VelocityInput);
        SetVelocityComponents(velocity, x, y);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn layout_is_exact_for_both_editions() {
        assert_eq!(core::mem::size_of::<Extent>(), 4);
        assert_eq!(core::mem::align_of::<Extent>(), 2);
        assert_eq!(core::mem::size_of::<VelocityDesc>(), 18);
        assert_eq!(core::mem::align_of::<VelocityDesc>(), 2);
        assert_eq!(core::mem::offset_of!(VelocityDesc, TravelAngle), 0);
        assert_eq!(core::mem::offset_of!(VelocityDesc, vector), 2);
        assert_eq!(core::mem::offset_of!(VelocityDesc, fract), 6);
        assert_eq!(core::mem::offset_of!(VelocityDesc, error), 10);
        assert_eq!(core::mem::offset_of!(VelocityDesc, incr), 14);
    }

    #[test]
    fn fractional_motion_accumulates_and_preserves_signs() {
        let mut velocity = VelocityDesc::default();
        let mut x = 0;
        let mut y = 0;
        // SAFETY: Independent local objects are valid and accessed serially.
        unsafe {
            SetVelocityComponents(&mut velocity, 33, -33);
            GetCurrentVelocityComponents(&mut velocity, &mut x, &mut y);
            assert_eq!((x, y), (33, -33));
            let mut sum = (0_i32, 0_i32);
            for _ in 0..32 {
                GetNextVelocityComponents(&mut velocity, &mut x, &mut y, 1);
                sum.0 += i32::from(x);
                sum.1 += i32::from(y);
            }
            assert_eq!(sum, (33, -33));
            assert_eq!(velocity.error, Extent::default());
            DeltaVelocityComponents(&mut velocity, -33, 33);
            assert_eq!(
                velocity,
                VelocityDesc {
                    TravelAngle: FULL_CIRCLE,
                    ..VelocityDesc::default()
                }
            );
        }
    }

    #[test]
    fn cardinal_facing_and_normalization_match_game_conventions() {
        let mut velocity = VelocityDesc::default();
        let mut x = 0;
        let mut y = 0;
        // SAFETY: Local valid pointers; no test mutates the sine table.
        unsafe {
            for (facing, expected) in [
                (0, (0, -320)),
                (4, (320, 0)),
                (8, (0, 320)),
                (12, (-320, 0)),
                (16, (0, -320)),
            ] {
                SetVelocityVector(&mut velocity, 10, facing);
                GetCurrentVelocityComponents(&mut velocity, &mut x, &mut y);
                assert_eq!((x, y), expected);
                assert_eq!(velocity.TravelAngle, (facing & 15) * 4);
            }
        }
    }

    #[test]
    fn current_outputs_may_alias_each_other() {
        let mut velocity = VelocityDesc::default();
        let mut value = 0_i16;
        let output = &mut value as *mut i16;
        // SAFETY: Both output arguments may refer to the same live scalar.
        unsafe {
            SetVelocityComponents(&mut velocity, 20, -30);
            GetCurrentVelocityComponents(&mut velocity, output, output);
        }
        assert_eq!(value, -30);
    }

    #[test]
    fn outputs_aliasing_descriptor_fields_keep_c_write_order() {
        let initial = VelocityDesc {
            TravelAngle: 0,
            vector: Extent {
                width: 1,
                height: 2,
            },
            fract: Extent {
                width: 3,
                height: 4,
            },
            error: Extent::default(),
            incr: Extent {
                width: 1,
                height: 1,
            },
        };
        let mut velocity = initial;
        let mut y = 0;
        let pointer = &mut velocity as *mut VelocityDesc;
        // SAFETY: C permits x's output to alias the next axis's fraction.
        // All accesses are raw and sequential, and every field is initialized.
        unsafe {
            let x = core::ptr::addr_of_mut!((*pointer).fract.height);
            GetCurrentVelocityComponents(pointer, x, &mut y);
            assert_eq!(y, 99); // x writes 35 into y's fraction before its read.
            assert_eq!((*pointer).fract.height, 35);
            pointer.write(initial);
            GetNextVelocityComponents(pointer, x, &mut y, 32);
            assert_eq!(y, 99);
            assert_eq!((*pointer).fract.height, 35);
            assert_eq!((*pointer).error, Extent::default());
        }
    }

    #[cfg(feature = "hd")]
    #[test]
    fn hd_wide_getters_preserve_values_above_i16() {
        let mut velocity = VelocityDesc::default();
        let mut x = 0_i32;
        let mut y = 0_i32;
        // SAFETY: Valid local values and a descriptor, serially accessed.
        unsafe {
            SetVelocityComponents(&mut velocity, 100_001, -100_001);
            GetCurrentVelocityComponentsSdword(&mut velocity, &mut x, &mut y);
            assert_eq!((x, y), (100_001, -100_001));
            GetNextVelocityComponentsSdword(&mut velocity, &mut x, &mut y, 32);
            assert_eq!((x, y), (100_001, -100_001));
            assert_eq!(velocity.error, Extent::default());
        }
    }
}
