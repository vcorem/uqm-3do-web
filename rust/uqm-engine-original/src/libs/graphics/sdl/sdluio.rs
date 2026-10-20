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

// Rust translation of src/libs/graphics/sdl/sdluio.c.
extern "C" {
    pub type uio_DirHandle;
    pub type _IO_FILE;
    pub type uio_Stream;
    pub type SDL_BlitMap;
    fn uio_fopen(
        dir: *mut uio_DirHandle,
        path: *const ::core::ffi::c_char,
        mode: *const ::core::ffi::c_char,
    ) -> *mut uio_Stream;
    fn uio_fclose(stream: *mut uio_Stream) -> ::core::ffi::c_int;
    fn uio_fread(
        buf: *mut ::core::ffi::c_void,
        size: size_t,
        nmemb: size_t,
        stream: *mut uio_Stream,
    ) -> size_t;
    fn uio_fseek(
        stream: *mut uio_Stream,
        offset: ::core::ffi::c_long,
        whence: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn uio_ftell(stream: *mut uio_Stream) -> ::core::ffi::c_long;
    fn uio_fwrite(
        buf: *const ::core::ffi::c_void,
        size: size_t,
        nmemb: size_t,
        stream: *mut uio_Stream,
    ) -> size_t;
    fn uio_ferror(stream: *mut uio_Stream) -> ::core::ffi::c_int;
    fn strerror(_: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    fn SDL_SetError(fmt: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn SDL_RWclose(context: *mut SDL_RWops) -> ::core::ffi::c_int;
    fn HMalloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn HFree(p: *mut ::core::ffi::c_void);
    fn TFB_png_to_sdl(src: *mut SDL_RWops) -> *mut SDL_Surface;
    fn __errno_location() -> *mut ::core::ffi::c_int;
}
pub type size_t = usize;
pub type int64_t = ::core::ffi::c_longlong;
pub type uint8_t = ::core::ffi::c_uchar;
pub type uint32_t = ::core::ffi::c_uint;
pub type FILE = _IO_FILE;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct SDL_bool(pub ::core::ffi::c_uint);
impl SDL_bool {
    pub const SDL_FALSE: Self = Self(0);
    pub const SDL_TRUE: Self = Self(1);
}
pub type Uint8 = uint8_t;
pub type Uint32 = uint32_t;
pub type Sint64 = int64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_RWops {
    pub size: Option<unsafe extern "C" fn(*mut SDL_RWops) -> Sint64>,
    pub seek: Option<
        unsafe extern "C" fn(*mut SDL_RWops, Sint64, ::core::ffi::c_int) -> Sint64,
    >,
    pub read: Option<
        unsafe extern "C" fn(
            *mut SDL_RWops,
            *mut ::core::ffi::c_void,
            size_t,
            size_t,
        ) -> size_t,
    >,
    pub write: Option<
        unsafe extern "C" fn(
            *mut SDL_RWops,
            *const ::core::ffi::c_void,
            size_t,
            size_t,
        ) -> size_t,
    >,
    pub close: Option<unsafe extern "C" fn(*mut SDL_RWops) -> ::core::ffi::c_int>,
    pub r#type: Uint32,
    pub hidden: C2Rust_Unnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed {
    pub stdio: C2Rust_Unnamed_2,
    pub mem: C2Rust_Unnamed_1,
    pub unknown: C2Rust_Unnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_0 {
    pub data1: *mut ::core::ffi::c_void,
    pub data2: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_1 {
    pub base: *mut Uint8,
    pub here: *mut Uint8,
    pub stop: *mut Uint8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_2 {
    pub autoclose: SDL_bool,
    pub fp: *mut FILE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_Color {
    pub r: Uint8,
    pub g: Uint8,
    pub b: Uint8,
    pub a: Uint8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_Palette {
    pub ncolors: ::core::ffi::c_int,
    pub colors: *mut SDL_Color,
    pub version: Uint32,
    pub refcount: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_PixelFormat {
    pub format: Uint32,
    pub palette: *mut SDL_Palette,
    pub BitsPerPixel: Uint8,
    pub BytesPerPixel: Uint8,
    pub padding: [Uint8; 2],
    pub Rmask: Uint32,
    pub Gmask: Uint32,
    pub Bmask: Uint32,
    pub Amask: Uint32,
    pub Rloss: Uint8,
    pub Gloss: Uint8,
    pub Bloss: Uint8,
    pub Aloss: Uint8,
    pub Rshift: Uint8,
    pub Gshift: Uint8,
    pub Bshift: Uint8,
    pub Ashift: Uint8,
    pub refcount: ::core::ffi::c_int,
    pub next: *mut SDL_PixelFormat,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_Rect {
    pub x: ::core::ffi::c_int,
    pub y: ::core::ffi::c_int,
    pub w: ::core::ffi::c_int,
    pub h: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_Surface {
    pub flags: Uint32,
    pub format: *mut SDL_PixelFormat,
    pub w: ::core::ffi::c_int,
    pub h: ::core::ffi::c_int,
    pub pitch: ::core::ffi::c_int,
    pub pixels: *mut ::core::ffi::c_void,
    pub userdata: *mut ::core::ffi::c_void,
    pub locked: ::core::ffi::c_int,
    pub list_blitmap: *mut ::core::ffi::c_void,
    pub clip_rect: SDL_Rect,
    pub map: *mut SDL_BlitMap,
    pub refcount: ::core::ffi::c_int,
}
#[no_mangle]
pub unsafe extern "C" fn sdluio_loadImage(
    mut dir: *mut uio_DirHandle,
    mut fileName: *const ::core::ffi::c_char,
) -> *mut SDL_Surface {
    let mut stream: *mut uio_Stream = ::core::ptr::null_mut::<uio_Stream>();
    let mut rwops: *mut SDL_RWops = ::core::ptr::null_mut::<SDL_RWops>();
    let mut result: *mut SDL_Surface = ::core::ptr::null_mut::<SDL_Surface>();
    stream = uio_fopen(dir, fileName, b"rb\0".as_ptr() as *const ::core::ffi::c_char);
    if stream.is_null() {
        SDL_SetError(
            b"Couldn't open '%s': %s\0".as_ptr() as *const ::core::ffi::c_char,
            fileName,
            strerror(*__errno_location()),
        );
        return ::core::ptr::null_mut::<SDL_Surface>();
    }
    rwops = sdluio_makeRWops(stream);
    if !rwops.is_null() {
        result = TFB_png_to_sdl(rwops);
        SDL_RWclose(rwops);
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn sdluio_seek(
    mut context: *mut SDL_RWops,
    mut offset: Sint64,
    mut whence: ::core::ffi::c_int,
) -> Sint64 {
    if uio_fseek(
        (*context).hidden.unknown.data1 as *mut uio_Stream,
        offset as ::core::ffi::c_long,
        whence,
    ) == -1 as ::core::ffi::c_int
    {
        SDL_SetError(
            b"Error seeking in uio_Stream: %s\0".as_ptr() as *const ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
        return -1 as Sint64;
    }
    return uio_ftell((*context).hidden.unknown.data1 as *mut uio_Stream) as Sint64;
}
#[no_mangle]
pub unsafe extern "C" fn sdluio_read(
    mut context: *mut SDL_RWops,
    mut ptr: *mut ::core::ffi::c_void,
    mut size: size_t,
    mut maxnum: size_t,
) -> size_t {
    let mut numRead: size_t = 0;
    numRead = uio_fread(
        ptr,
        size,
        maxnum,
        (*context).hidden.unknown.data1 as *mut uio_Stream,
    );
    if numRead == 0 as size_t
        && uio_ferror((*context).hidden.unknown.data1 as *mut uio_Stream) != 0
    {
        SDL_SetError(
            b"Error reading from uio_Stream: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
        return 0 as size_t;
    }
    return numRead as ::core::ffi::c_int as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn sdluio_write(
    mut context: *mut SDL_RWops,
    mut ptr: *const ::core::ffi::c_void,
    mut size: size_t,
    mut num: size_t,
) -> size_t {
    let mut numWritten: size_t = 0;
    numWritten = uio_fwrite(
        ptr,
        size,
        num,
        (*context).hidden.unknown.data1 as *mut uio_Stream,
    );
    if numWritten == 0 as size_t
        && uio_ferror((*context).hidden.unknown.data1 as *mut uio_Stream) != 0
    {
        SDL_SetError(
            b"Error writing to uio_Stream: %s\0".as_ptr() as *const ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
        return 0 as size_t;
    }
    return numWritten;
}
#[no_mangle]
pub unsafe extern "C" fn sdluio_close(
    mut context: *mut SDL_RWops,
) -> ::core::ffi::c_int {
    let mut result: ::core::ffi::c_int = 0;
    result = uio_fclose((*context).hidden.unknown.data1 as *mut uio_Stream);
    HFree(context as *mut ::core::ffi::c_void);
    return result;
}
unsafe extern "C" fn sdluio_makeRWops(mut stream: *mut uio_Stream) -> *mut SDL_RWops {
    let mut result: *mut SDL_RWops = ::core::ptr::null_mut::<SDL_RWops>();
    result = HMalloc(::core::mem::size_of::<SDL_RWops>()) as *mut SDL_RWops;
    (*result).seek = Some(
        sdluio_seek
            as unsafe extern "C" fn(*mut SDL_RWops, Sint64, ::core::ffi::c_int) -> Sint64,
    )
        as Option<
            unsafe extern "C" fn(*mut SDL_RWops, Sint64, ::core::ffi::c_int) -> Sint64,
        >;
    (*result).read = Some(
        sdluio_read
            as unsafe extern "C" fn(
                *mut SDL_RWops,
                *mut ::core::ffi::c_void,
                size_t,
                size_t,
            ) -> size_t,
    )
        as Option<
            unsafe extern "C" fn(
                *mut SDL_RWops,
                *mut ::core::ffi::c_void,
                size_t,
                size_t,
            ) -> size_t,
        >;
    (*result).write = Some(
        sdluio_write
            as unsafe extern "C" fn(
                *mut SDL_RWops,
                *const ::core::ffi::c_void,
                size_t,
                size_t,
            ) -> size_t,
    )
        as Option<
            unsafe extern "C" fn(
                *mut SDL_RWops,
                *const ::core::ffi::c_void,
                size_t,
                size_t,
            ) -> size_t,
        >;
    (*result).close = Some(
        sdluio_close as unsafe extern "C" fn(*mut SDL_RWops) -> ::core::ffi::c_int,
    ) as Option<unsafe extern "C" fn(*mut SDL_RWops) -> ::core::ffi::c_int>;
    (*result).hidden.unknown.data1 = stream as *mut ::core::ffi::c_void;
    return result;
}
