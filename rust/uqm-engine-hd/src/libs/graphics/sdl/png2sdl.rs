/*
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

/*
 * This code is adapted from IMG_png.c in the SDL2_image library, which is
 * available under the zlib license and is (c) 1997-2019 Sam Lantinga. The
 * code itself is credited to Philippe Lavoie as the original author. It
 * also shares some heritage with libpng's own example.c file, and
 * ultimately inherits the GPL2+ license from the rest of UQM.
 *
 * Differences from SDL2_image are:
 *  - It is PNG-only
 *  - It directly links the relevant version of libpng at compile time
 *  - It always uses libpng and will never forward to alternative
 *    libraries such as ImageIO.framework. This means that palette
 *    information will always be preserved, as UQM requires.
 *  - It locks the surface as the API demands rather than using
 *    volatility markers
 *  - Palette assignment is done through the API rather than by
 *    directly editing the format contents
 */

// Shared Rust PNG implementation; C contains only guarded library calls.
#[path = "../../../../../uqm-png/png.rs"]
mod implementation;
pub use implementation::*;
