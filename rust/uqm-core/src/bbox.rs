// SPDX-License-Identifier: GPL-2.0-or-later
// Derived from sc2/src/libs/graphics/bbox.c and bbox.h (also used by MegaMod).

//! Dirty-region accounting for the original renderer. The C header requires
//! every access to occur on the screen-owning thread; this API preserves that
//! contract rather than introducing a lock around externally visible fields.

#![allow(non_upper_case_globals)]

use crate::geometry::{Point, Rect};
use crate::velocity::Extent;
use core::ffi::{c_int, c_void};
use core::ptr;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct BoundingBox {
    pub valid: c_int,
    pub region: Rect,
    pub clip: Rect,
}

const EMPTY_RECT: Rect = Rect {
    corner: Point { x: 0, y: 0 },
    extent: Extent {
        width: 0,
        height: 0,
    },
};

#[no_mangle]
pub static mut TFB_BBox: BoundingBox = BoundingBox {
    valid: 0,
    region: EMPTY_RECT,
    clip: EMPTY_RECT,
};
#[no_mangle]
pub static mut maxWidth: c_int = 0;
#[no_mangle]
pub static mut maxHeight: c_int = 0;

extern "C" {
    fn TFB_DrawCanvas_GetExtent(canvas: *mut c_void, extent: *mut Extent);
}

/// # Safety
/// All bounding-box globals and calls must be serialized on the screen thread.
#[no_mangle]
pub unsafe extern "C" fn TFB_BBox_Init(width: c_int, height: c_int) {
    // SAFETY: The screen thread exclusively owns these C-visible globals.
    unsafe {
        maxWidth = width;
        maxHeight = height;
        TFB_BBox.clip.extent.width = width as i16;
        TFB_BBox.clip.extent.height = height as i16;
    }
}

/// # Safety
/// All bounding-box globals and calls must be serialized on the screen thread.
#[no_mangle]
pub unsafe extern "C" fn TFB_BBox_Reset() {
    // SAFETY: The caller serializes the renderer's state.
    unsafe { TFB_BBox.valid = 0 };
}

/// # Safety
/// `rect` is null or readable as an initialized `RECT`; it may alias the global
/// clip or region. Calls and all global accesses belong to the screen thread.
#[no_mangle]
pub unsafe extern "C" fn TFB_BBox_SetClipRect(rect: *const Rect) {
    // SAFETY: No references to globals are formed; copying the input before
    // writes preserves the C assignment when rect aliases either global field.
    unsafe {
        let mut clip = if rect.is_null() {
            Rect {
                corner: Point { x: 0, y: 0 },
                extent: Extent {
                    width: maxWidth as i16,
                    height: maxHeight as i16,
                },
            }
        } else {
            rect.read()
        };
        if !rect.is_null() {
            clip.corner.x = clip.corner.x.max(0);
            clip.corner.y = clip.corner.y.max(0);
            if i32::from(clip.corner.x) + i32::from(clip.extent.width) > maxWidth {
                clip.extent.width = maxWidth.wrapping_sub(i32::from(clip.corner.x)) as i16;
            }
            if i32::from(clip.corner.y) + i32::from(clip.extent.height) > maxHeight {
                clip.extent.height = maxHeight.wrapping_sub(i32::from(clip.corner.y)) as i16;
            }
        }
        ptr::addr_of_mut!(TFB_BBox.clip).write(clip);
    }
}

/// # Safety
/// All bounding-box globals and calls must be serialized on the screen thread.
#[no_mangle]
pub unsafe extern "C" fn TFB_BBox_RegisterPoint(mut x: c_int, mut y: c_int) {
    // SAFETY: The caller owns renderer state for this synchronous call. All
    // arithmetic on stored RECT fields is promoted to C int before narrowing.
    unsafe {
        let mut bbox = ptr::addr_of!(TFB_BBox).read();
        let x1 = i32::from(bbox.clip.corner.x);
        let y1 = i32::from(bbox.clip.corner.y);
        let x2 = x1 + i32::from(bbox.clip.extent.width) - 1;
        let y2 = y1 + i32::from(bbox.clip.extent.height) - 1;
        // These are ordered clamps, not clamp(min,max): malformed/empty clip
        // rectangles deliberately retain the original reversed-bound behavior.
        if x < x1 {
            x = x1;
        }
        if x >= x2 {
            x = x2;
        }
        if y < y1 {
            y = y1;
        }
        if y >= y2 {
            y = y2;
        }
        if bbox.valid == 0 {
            bbox.valid = 1;
            bbox.region = Rect {
                corner: Point {
                    x: x as i16,
                    y: y as i16,
                },
                extent: Extent {
                    width: 1,
                    height: 1,
                },
            };
        } else {
            let x1 = i32::from(bbox.region.corner.x);
            let y1 = i32::from(bbox.region.corner.y);
            let x2 = x1 + i32::from(bbox.region.extent.width) - 1;
            let y2 = y1 + i32::from(bbox.region.extent.height) - 1;
            if x < x1 {
                bbox.region.corner.x = x as i16;
                bbox.region.extent.width = (i32::from(bbox.region.extent.width) + x1 - x) as i16;
            }
            if y < y1 {
                bbox.region.corner.y = y as i16;
                bbox.region.extent.height = (i32::from(bbox.region.extent.height) + y1 - y) as i16;
            }
            if x > x2 {
                bbox.region.extent.width = (i32::from(bbox.region.extent.width) + x - x2) as i16;
            }
            if y > y2 {
                bbox.region.extent.height = (i32::from(bbox.region.extent.height) + y - y2) as i16;
            }
        }
        ptr::addr_of_mut!(TFB_BBox).write(bbox);
    }
}

/// # Safety
/// `rect` is readable as an initialized RECT and may alias either global RECT.
/// Global state and calls must be serialized on the screen thread.
#[no_mangle]
pub unsafe extern "C" fn TFB_BBox_RegisterRect(rect: *const Rect) {
    // SAFETY: Read the second point after RegisterPoint, as C does. Taking a
    // snapshot of both points first would change legal global-region aliasing.
    unsafe {
        TFB_BBox_RegisterPoint(i32::from((*rect).corner.x), i32::from((*rect).corner.y));
        TFB_BBox_RegisterPoint(
            i32::from((*rect).corner.x) + i32::from((*rect).extent.width) - 1,
            i32::from((*rect).corner.y) + i32::from((*rect).extent.height) - 1,
        );
    }
}

/// # Safety
/// `canvas` meets TFB_DrawCanvas_GetExtent's backend contract. All calls and
/// bounding-box global accesses must be serialized on the screen thread.
#[no_mangle]
pub unsafe extern "C" fn TFB_BBox_RegisterCanvas(canvas: *mut c_void, x: c_int, y: c_int) {
    let mut rect = Rect {
        corner: Point {
            x: x as i16,
            y: y as i16,
        },
        extent: Extent {
            width: 0,
            height: 0,
        },
    };
    // SAFETY: The backend writes the local initialized extent; canvas meets its
    // original C validity contract. The local rectangle lives through the call.
    unsafe {
        TFB_DrawCanvas_GetExtent(canvas, &mut rect.extent);
        TFB_BBox_RegisterRect(&rect);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[export_name = "TFB_DrawCanvas_GetExtent"]
    unsafe extern "C" fn test_extent(canvas: *mut c_void, out: *mut Extent) {
        // SAFETY: Unit tests pass a live Extent as the opaque test canvas.
        unsafe { out.write(canvas.cast::<Extent>().read()) };
    }

    #[test]
    fn dirty_region_retains_clipping_reset_and_backend_extent() {
        assert_eq!(core::mem::size_of::<BoundingBox>(), 20);
        assert_eq!(core::mem::offset_of!(BoundingBox, region), 4);
        assert_eq!(core::mem::offset_of!(BoundingBox, clip), 12);
        // SAFETY: This is the only unit test touching the renderer globals.
        unsafe {
            TFB_BBox_Init(320, 240);
            TFB_BBox_SetClipRect(ptr::null());
            TFB_BBox_Reset();
            TFB_BBox_RegisterPoint(-100, -100);
            TFB_BBox_RegisterPoint(500, 500);
            assert_eq!(
                ptr::addr_of!(TFB_BBox.region).read(),
                Rect {
                    corner: Point { x: 0, y: 0 },
                    extent: Extent {
                        width: 320,
                        height: 240
                    }
                }
            );
            TFB_BBox_Reset();
            let mut extent = Extent {
                width: 17,
                height: 13,
            };
            TFB_BBox_RegisterCanvas((&mut extent as *mut Extent).cast(), 10, 20);
            assert_eq!(
                ptr::addr_of!(TFB_BBox.region).read(),
                Rect {
                    corner: Point { x: 10, y: 20 },
                    extent
                }
            );
        }
    }
}
