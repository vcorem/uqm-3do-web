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
 *  Foundation, Inc., 59 Temple Place - Suite 330, Boston, MA 02111-1307, USA.
 */

/* By Serge van den Boom
 */

// Rust translation of src/libs/time/sdl/sdltime.c.
extern "C" {
    fn SDL_GetTicks() -> Uint32;
}
pub type uint32_t = ::core::ffi::c_uint;
pub type Uint32 = uint32_t;
#[no_mangle]
pub unsafe extern "C" fn SDLWrapper_GetTimeCounter() -> Uint32 {
    let mut ticks: Uint32 = SDL_GetTicks();
    return ticks
        .wrapping_div(1000 as Uint32)
        .wrapping_mul(840 as Uint32)
        .wrapping_add(
            ticks
                .wrapping_rem(1000 as Uint32)
                .wrapping_mul(840 as Uint32)
                .wrapping_div(1000 as Uint32),
        );
}
