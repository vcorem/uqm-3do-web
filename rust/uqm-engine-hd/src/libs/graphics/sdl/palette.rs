/*
 *  Copyright 2009 Alex Volkov <codepro@usa.net>
 *
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

// Rust translation of src/libs/graphics/sdl/palette.c.
extern "C" {
    fn HFree(p: *mut ::core::ffi::c_void);
    fn HCalloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn __assert_fail(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    ) -> !;
}
pub type size_t = usize;
pub type uint8_t = ::core::ffi::c_uchar;
pub type Uint8 = uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_Color {
    pub r: Uint8,
    pub g: Uint8,
    pub b: Uint8,
    pub a: Uint8,
}
pub type uint8 = ::core::ffi::c_uchar;
pub type BYTE = uint8;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Color {
    pub r: BYTE,
    pub g: BYTE,
    pub b: BYTE,
    pub a: BYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NativePalette {
    pub colors: [SDL_Color; 256],
}
#[inline]
unsafe extern "C" fn NativeToColor(mut native: SDL_Color) -> Color {
    let mut color: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    color.r = native.r as BYTE;
    color.g = native.g as BYTE;
    color.b = native.b as BYTE;
    color.a = 0xff as BYTE;
    return color;
}
#[inline]
unsafe extern "C" fn ColorToNative(mut color: Color) -> SDL_Color {
    let mut native: SDL_Color = SDL_Color {
        r: 0,
        g: 0,
        b: 0,
        a: 0,
    };
    native.r = color.r as Uint8;
    native.g = color.g as Uint8;
    native.b = color.b as Uint8;
    native.a = color.a as Uint8;
    return native;
}
#[no_mangle]
pub unsafe extern "C" fn AllocNativePalette() -> *mut NativePalette {
    return HCalloc(::core::mem::size_of::<NativePalette>()) as *mut NativePalette;
}
#[no_mangle]
pub unsafe extern "C" fn FreeNativePalette(mut palette: *mut NativePalette) {
    HFree(palette as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn SetNativePaletteColor(
    mut palette: *mut NativePalette,
    mut index: ::core::ffi::c_int,
    mut color: Color,
) {
    (index < 256 as ::core::ffi::c_int
        || {
            __assert_fail(
                b"index < NUMBER_OF_PLUTVALS\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/libs/graphics/sdl/palette.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                38 as ::core::ffi::c_int,
                b"SetNativePaletteColor\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    (*palette).colors[index as usize] = ColorToNative(color);
}
#[no_mangle]
pub unsafe extern "C" fn GetNativePaletteColor(
    mut palette: *mut NativePalette,
    mut index: ::core::ffi::c_int,
) -> Color {
    (index < 256 as ::core::ffi::c_int
        || {
            __assert_fail(
                b"index < NUMBER_OF_PLUTVALS\0".as_ptr() as *const ::core::ffi::c_char,
                b"src/libs/graphics/sdl/palette.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                45 as ::core::ffi::c_int,
                b"GetNativePaletteColor\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0 as ::core::ffi::c_int != 0
        }) as ::core::ffi::c_int;
    return NativeToColor((*palette).colors[index as usize]);
}
