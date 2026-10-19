// SPDX-License-Identifier: GPL-2.0-or-later
// Derived from sc2/src/libs/graphics/boxint.c and clipline.c and MegaMod's
// variants. Copyright Paul Reiche and Fred Ford, 1992-2002.

//! Integer rectangle/line clipping behind the existing graphics C ABI.
//!
//! COORD/SIZE narrowing, inclusive line edges and partially modified rejected
//! lines retain the original behavior. Raw pointer accesses preserve aliased
//! rectangle output order, including the legacy BoxUnion aliasing behavior.

use crate::velocity::Extent;
use core::ffi::c_long;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Point {
    pub x: i16,
    pub y: i16,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Rect {
    pub corner: Point,
    pub extent: Extent,
}

#[cfg(feature = "hd")]
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DPoint {
    pub x: i32,
    pub y: i32,
}

#[cfg(feature = "hd")]
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DExtent {
    pub width: i32,
    pub height: i32,
}

#[cfg(feature = "hd")]
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DRect {
    pub corner: DPoint,
    pub extent: DExtent,
}

#[cfg(not(feature = "hd"))]
pub type ClipRect = Rect;
#[cfg(feature = "hd")]
pub type ClipRect = DRect;

/// BOOLEAN is a C enum (int-sized), not a C99 bool. The trailing padding is
/// deliberately left to repr(C); callers compare fields, not padding bytes.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct BresenhamLine {
    pub first: Point,
    pub second: Point,
    pub abs_delta_x: i16,
    pub abs_delta_y: i16,
    pub error_term: i16,
    pub end_points_exchanged: i32,
    pub intersect_code: u8,
}

const LEFT: u8 = 1;
const TOP: u8 = 2;
const RIGHT: u8 = 4;
const BOTTOM: u8 = 8;
const ALL_SIDES: u8 = LEFT | TOP | RIGHT | BOTTOM;
const NOCLIP: u8 = 128;

/// # Safety
/// All inputs must point to initialized rectangles and output to writable Rect
/// storage, with serialized access. Output may alias either input (or both).
/// A rejected intersection leaves the output completely unchanged.
#[no_mangle]
pub unsafe extern "C" fn BoxIntersect(
    first: *mut Rect,
    second: *mut Rect,
    output: *mut Rect,
) -> u8 {
    // SAFETY: All raw accesses satisfy the caller's validity contract. Output
    // writes retain C order and occur only after both positive extents exist.
    unsafe {
        let mut code = NOCLIP;
        let mut x = (*first).corner.x.wrapping_sub((*second).corner.x);
        let mut width = (*first).extent.width;
        let delta = (*second).extent.width.wrapping_sub(x);
        if delta <= width {
            if delta != width {
                width = delta;
                code &= !NOCLIP;
            }
            code |= RIGHT;
        }
        if x <= 0 {
            if x < 0 {
                width = width.wrapping_add(x);
                x = 0;
                code &= !NOCLIP;
            }
            code |= LEFT;
        }
        if width <= 0 {
            return 0;
        }
        let mut y = (*first).corner.y.wrapping_sub((*second).corner.y);
        let mut height = (*first).extent.height;
        let delta = (*second).extent.height.wrapping_sub(y);
        if delta <= height {
            if delta != height {
                height = delta;
                code &= !NOCLIP;
            }
            code |= BOTTOM;
        }
        if y <= 0 {
            if y < 0 {
                height = height.wrapping_add(y);
                y = 0;
                code &= !NOCLIP;
            }
            code |= TOP;
        }
        if height <= 0 {
            return 0;
        }
        (*output).corner.x = x.wrapping_add((*second).corner.x);
        (*output).corner.y = y.wrapping_add((*second).corner.y);
        (*output).extent.width = width;
        (*output).extent.height = height;
        code
    }
}

/// # Safety
/// Inputs must be initialized, output writable, and access serialized. Output
/// may alias inputs; subsequent reads intentionally observe previous writes,
/// exactly as the original C BoxUnion does. This is not a snapshot-based union.
#[no_mangle]
pub unsafe extern "C" fn BoxUnion(first: *mut Rect, second: *mut Rect, output: *mut Rect) {
    // SAFETY: Raw accesses preserve the C alias contract without &mut borrows.
    unsafe {
        (*output).corner.x = (*first).corner.x.min((*second).corner.x);
        (*output).corner.y = (*first).corner.y.min((*second).corner.y);
        (*output).extent.width = ((i32::from((*first).corner.x) + i32::from((*first).extent.width))
            .max(i32::from((*second).corner.x) + i32::from((*second).extent.width))
            - i32::from((*output).corner.x)) as i16;
        (*output).extent.height = ((i32::from((*first).corner.y)
            + i32::from((*first).extent.height))
        .max(i32::from((*second).corner.y) + i32::from((*second).extent.height))
            - i32::from((*output).corner.y)) as i16;
    }
}

/// # Safety
/// `clip` must point to the edition's initialized clipping rectangle and `line`
/// to initialized writable line storage. Access must be serialized. Partial
/// endpoint/delta writes on rejection intentionally match the C routine.
/// The engine uses endpoint differences that fit its signed doubled SIZE
/// arithmetic; explicit wrapping additionally defines those legacy overflows.
#[no_mangle]
// ClipRect coordinates are i16 in Original and i32 in HD; retain the explicit
// casts in the shared arithmetic. Manual swaps retain the audited C write order
// and keep this FFI routine entirely in raw-pointer accesses.
#[allow(clippy::unnecessary_cast, clippy::manual_swap)]
pub unsafe extern "C" fn _clip_line(clip: *const ClipRect, line: *mut BresenhamLine) -> u8 {
    // SAFETY: All input/output accesses use the caller's live initialized
    // structures. Values are read/written in the same sequence as the C code.
    unsafe {
        let mut xmin = (*clip).corner.x as i16;
        let mut ymin = (*clip).corner.y as i16;
        let mut xmax = ((*clip).corner.x as i32)
            .wrapping_add((*clip).extent.width as i32)
            .wrapping_sub(1) as i16;
        let mut ymax = ((*clip).corner.y as i32)
            .wrapping_add((*clip).extent.height as i32)
            .wrapping_sub(1) as i16;

        if (*line).first.x <= (*line).second.x {
            (*line).end_points_exchanged = 0;
        } else {
            let x = (*line).first.x;
            (*line).first.x = (*line).second.x;
            (*line).second.x = x;
            let y = (*line).first.y;
            (*line).first.y = (*line).second.y;
            (*line).second.y = y;
            (*line).end_points_exchanged = 1;
        }

        if (*line).first.x > xmax
            || (*line).second.x < xmin
            || ((*line).first.y > ymax && (*line).second.y > ymax)
            || ((*line).first.y < ymin && (*line).second.y < ymin)
        {
            return 0;
        }

        let mut code = NOCLIP;
        let mut x0 = 0i16;
        let mut y0 = 0i16;
        let dx = (*line)
            .second
            .x
            .wrapping_sub((*line).first.x)
            .wrapping_shl(1);
        let mut dy = (*line)
            .second
            .y
            .wrapping_sub((*line).first.y)
            .wrapping_shl(1);
        (*line).abs_delta_x = dx;
        (*line).abs_delta_y = dy;
        if dy == 0 {
            if (*line).first.x < xmin {
                (*line).first.x = xmin;
                code |= LEFT;
            }
            if (*line).second.x > xmax {
                (*line).second.x = xmax;
                code |= RIGHT;
            }
        } else if dx == 0 {
            if dy < 0 {
                let y = (*line).first.y;
                (*line).first.y = (*line).second.y;
                (*line).second.y = y;
                dy = dy.wrapping_neg();
                (*line).abs_delta_y = dy;
            }
            if (*line).first.y < ymin {
                (*line).first.y = ymin;
                code |= TOP;
            }
            if (*line).second.y > ymax {
                (*line).second.y = ymax;
                code |= BOTTOM;
            }
        } else {
            let x = (*line).first.x;
            let mut x1 = (*line).second.x.wrapping_sub(x);
            xmin = xmin.wrapping_sub(x);
            xmax = xmax.wrapping_sub(x);

            let y = (*line).first.y;
            let mut y1;
            if dy > 0 {
                y1 = (*line).second.y.wrapping_sub(y);
                ymin = ymin.wrapping_sub(y);
                ymax = ymax.wrapping_sub(y);
            } else {
                y1 = y.wrapping_sub((*line).second.y);
                let new_ymax = y.wrapping_sub(ymin);
                ymin = y.wrapping_sub(ymax);
                ymax = new_ymax;
                dy = dy.wrapping_neg();
            }

            if dx > dy {
                let half_dx = c_long::from(dx >> 1);
                if x0 < xmin {
                    x0 = xmin;
                    y0 =
                        ((c_long::from(dy) * c_long::from(x0) + half_dx) / c_long::from(dx)) as i16;
                    if y0 > ymax {
                        return 0;
                    }
                    code |= LEFT;
                }
                if x1 > xmax {
                    x1 = xmax;
                    y1 =
                        ((c_long::from(dy) * c_long::from(x1) + half_dx) / c_long::from(dx)) as i16;
                    if y1 < ymin {
                        return 0;
                    }
                    code |= RIGHT;
                }
                if y0 < ymin {
                    y0 = ymin;
                    x0 = ((c_long::from(dx) * c_long::from(y0) - half_dx + c_long::from(dy) - 1)
                        / c_long::from(dy)) as i16;
                    if x0 > xmax {
                        return 0;
                    }
                    code |= TOP;
                    code &= !LEFT;
                }
                if y1 > ymax {
                    y1 = ymax;
                    x1 = (((c_long::from(dx) * (c_long::from(y1) + 1) - half_dx + c_long::from(dy)
                        - 1)
                        / c_long::from(dy)) as i16)
                        .wrapping_sub(1);
                    if x1 < xmin {
                        return 0;
                    }
                    code |= BOTTOM;
                    code &= !RIGHT;
                }
            } else {
                let half_dy = c_long::from(dy >> 1);
                if y0 < ymin {
                    y0 = ymin;
                    x0 =
                        ((c_long::from(dx) * c_long::from(y0) + half_dy) / c_long::from(dy)) as i16;
                    if x0 > xmax {
                        return 0;
                    }
                    code |= TOP;
                }
                if y1 > ymax {
                    y1 = ymax;
                    x1 =
                        ((c_long::from(dx) * c_long::from(y1) + half_dy) / c_long::from(dy)) as i16;
                    if x1 < xmin {
                        return 0;
                    }
                    code |= BOTTOM;
                }
                if x0 < xmin {
                    x0 = xmin;
                    y0 = ((c_long::from(dy) * c_long::from(x0) - half_dy + c_long::from(dx) - 1)
                        / c_long::from(dx)) as i16;
                    if y0 > ymax {
                        return 0;
                    }
                    code |= LEFT;
                    code &= !TOP;
                }
                if x1 > xmax {
                    x1 = xmax;
                    y1 = (((c_long::from(dy) * (c_long::from(x1) + 1) - half_dy + c_long::from(dx)
                        - 1)
                        / c_long::from(dx)) as i16)
                        .wrapping_sub(1);
                    if y1 < ymin {
                        return 0;
                    }
                    code |= RIGHT;
                    code &= !BOTTOM;
                }
            }

            (*line).second.x = (*line).first.x.wrapping_add(x1);
            (*line).first.x = (*line).first.x.wrapping_add(x0);
            if (*line).abs_delta_y > 0 {
                (*line).second.y = (*line).first.y.wrapping_add(y1);
                (*line).first.y = (*line).first.y.wrapping_add(y0);
            } else {
                (*line).second.y = (*line).first.y.wrapping_sub(y1);
                (*line).first.y = (*line).first.y.wrapping_sub(y0);
                let y_code = code & (TOP | BOTTOM);
                if y_code != 0 && y_code != (TOP | BOTTOM) {
                    code ^= TOP | BOTTOM;
                }
            }
        }

        if code & ALL_SIDES == 0 {
            (*line).error_term = if dx > dy {
                (dx >> 1).wrapping_neg()
            } else {
                (dy >> 1).wrapping_neg()
            };
        } else {
            code &= !NOCLIP;
            (*line).error_term = if dx > dy {
                ((c_long::from(x0) * c_long::from(dy) - c_long::from(y0) * c_long::from(dx)) as i16)
                    .wrapping_sub(dx >> 1)
            } else {
                ((c_long::from(y0) * c_long::from(dx) - c_long::from(x0) * c_long::from(dy)) as i16)
                    .wrapping_sub(dy >> 1)
            };
        }
        (*line).intersect_code = code;
        code
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rect(x: i16, y: i16, width: i16, height: i16) -> Rect {
        Rect {
            corner: Point { x, y },
            extent: Extent { width, height },
        }
    }

    #[test]
    fn intersection_alias_and_rejection_keep_original_writes() {
        let mut a = rect(1, 2, 10, 20);
        let mut b = rect(5, 7, 3, 5);
        // SAFETY: All three pointers reference initialized writable rectangles.
        unsafe {
            let a_ptr = &mut a as *mut Rect;
            assert_eq!(BoxIntersect(a_ptr, &mut b, a_ptr), ALL_SIDES);
            assert_eq!(a, b);
            b.corner.x = 50;
            let before = a;
            assert_eq!(BoxIntersect(a_ptr, &mut b, a_ptr), 0);
            assert_eq!(a, before);
        }
    }

    #[test]
    fn union_preserves_legacy_aliasing_instead_of_snapshotting() {
        let mut a = rect(10, 20, 5, 5);
        let mut b = rect(0, 0, 2, 2);
        let mut separate = Rect::default();
        // SAFETY: Output storage is valid, including the documented input alias.
        unsafe {
            BoxUnion(&mut a, &mut b, &mut separate);
            assert_eq!(separate, rect(0, 0, 15, 25));
            let a_ptr = &mut a as *mut Rect;
            BoxUnion(a_ptr, &mut b, a_ptr);
            assert_eq!(a, rect(0, 0, 5, 5));
        }
    }

    fn clipping_rect() -> ClipRect {
        #[cfg(not(feature = "hd"))]
        {
            rect(0, 0, 10, 10)
        }
        #[cfg(feature = "hd")]
        {
            DRect {
                corner: DPoint { x: 0, y: 0 },
                extent: DExtent {
                    width: 10,
                    height: 10,
                },
            }
        }
    }

    #[test]
    fn clipping_swaps_endpoints_before_rejecting() {
        let mut line = BresenhamLine {
            first: Point { x: 20, y: 30 },
            second: Point { x: 11, y: 12 },
            abs_delta_x: 123,
            abs_delta_y: -456,
            error_term: 789,
            end_points_exchanged: 0,
            intersect_code: 91,
        };
        // SAFETY: Both clipping input and line storage are initialized and live.
        unsafe {
            assert_eq!(_clip_line(&clipping_rect(), &mut line), 0);
        }
        assert_eq!(line.first, Point { x: 11, y: 12 });
        assert_eq!(line.second, Point { x: 20, y: 30 });
        assert_eq!(line.end_points_exchanged, 1);
        assert_eq!(
            (
                line.abs_delta_x,
                line.abs_delta_y,
                line.error_term,
                line.intersect_code
            ),
            (123, -456, 789, 91)
        );
    }

    #[test]
    fn clipping_uses_inclusive_edges_and_retains_unclipped_deltas() {
        let mut line = BresenhamLine {
            first: Point { x: -2, y: 5 },
            second: Point { x: 12, y: 5 },
            ..BresenhamLine::default()
        };
        // SAFETY: Both clipping input and line storage are initialized and live.
        unsafe {
            assert_eq!(_clip_line(&clipping_rect(), &mut line), LEFT | RIGHT);
        }
        assert_eq!(line.first, Point { x: 0, y: 5 });
        assert_eq!(line.second, Point { x: 9, y: 5 });
        assert_eq!(line.abs_delta_x, 28);
        assert_eq!(line.error_term, -14);
    }

    #[test]
    fn graphics_layouts_match_c_headers() {
        assert_eq!(core::mem::size_of::<Rect>(), 8);
        assert_eq!(core::mem::align_of::<Rect>(), 2);
        assert_eq!(core::mem::size_of::<BresenhamLine>(), 24);
        assert_eq!(core::mem::align_of::<BresenhamLine>(), 4);
        assert_eq!(
            core::mem::offset_of!(BresenhamLine, end_points_exchanged),
            16
        );
        assert_eq!(core::mem::offset_of!(BresenhamLine, intersect_code), 20);
        #[cfg(feature = "hd")]
        {
            assert_eq!(core::mem::size_of::<DRect>(), 16);
            assert_eq!(core::mem::align_of::<DRect>(), 4);
        }
    }
}
