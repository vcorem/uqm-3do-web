// SPDX-License-Identifier: GPL-2.0-or-later
// Rust orchestration port of UQM/MegaMod libs/graphics/sdl/png2sdl.c.
// The source derives from SDL2_image (c) 1997-2019 Sam Lantinga, originally
// authored by Philippe Lavoie, and libpng's example.c. The HD writer derives
// from driedfruit's SDL_SavePNG (https://github.com/driedfruit/SDL_SavePNG/).
// This Rust port is an altered version; it retains the UQM GPL-2.0-or-later
// license. See ../uqm-engine-original/COPYING for the complete license.
// libpng setjmp/longjmp is contained in png-guard.c, never across Rust frames.

#![allow(non_snake_case, non_camel_case_types, dead_code)]

use core::ffi::{c_char, c_int, c_void};
use core::{mem, ptr};

pub type SDL_RWops = c_void;

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SDL_Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[repr(C)]
pub struct SDL_Palette {
    pub ncolors: c_int,
    pub colors: *mut SDL_Color,
    pub version: u32,
    pub refcount: c_int,
}

#[repr(C)]
pub struct SDL_PixelFormat {
    pub format: u32,
    pub palette: *mut SDL_Palette,
    pub BitsPerPixel: u8,
    pub BytesPerPixel: u8,
    pub padding: [u8; 2],
    pub Rmask: u32,
    pub Gmask: u32,
    pub Bmask: u32,
    pub Amask: u32,
    pub Rloss: u8,
    pub Gloss: u8,
    pub Bloss: u8,
    pub Aloss: u8,
    pub Rshift: u8,
    pub Gshift: u8,
    pub Bshift: u8,
    pub Ashift: u8,
    pub refcount: c_int,
    pub next: *mut SDL_PixelFormat,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SDL_Rect {
    pub x: c_int,
    pub y: c_int,
    pub w: c_int,
    pub h: c_int,
}

#[repr(C)]
pub struct SDL_Surface {
    pub flags: u32,
    pub format: *mut SDL_PixelFormat,
    pub w: c_int,
    pub h: c_int,
    pub pitch: c_int,
    pub pixels: *mut c_void,
    pub userdata: *mut c_void,
    pub locked: c_int,
    pub list_blitmap: *mut c_void,
    pub clip_rect: SDL_Rect,
    pub map: *mut c_void,
    pub refcount: c_int,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PngHeader {
    pub width: u32,
    pub height: u32,
    pub depth: c_int,
    pub color: c_int,
    pub interlace: c_int,
    pub compression: c_int,
    pub filter: c_int,
    pub channels: c_int,
}

#[repr(C)]
pub struct PngIo {
    pub user: *mut c_void,
    pub read: Option<unsafe extern "C" fn(*mut c_void, *mut u8, usize) -> usize>,
    pub write: Option<unsafe extern "C" fn(*mut c_void, *const u8, usize) -> usize>,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PngColor16 {
    pub index: u8,
    pub red: u16,
    pub green: u16,
    pub blue: u16,
    pub gray: u16,
}

#[repr(C)]
pub struct PngTransparency {
    pub alpha: *const u8,
    pub color: *const PngColor16,
    pub count: c_int,
    pub valid: c_int,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PngColor {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

#[repr(C)]
pub struct PngPalette {
    pub colors: *const PngColor,
    pub count: c_int,
}

extern "C" {
    fn uqm_png_create(writing: c_int) -> *mut c_void;
    fn uqm_png_destroy(context: *mut c_void);
    fn uqm_png_call(context: *mut c_void, operation: u32, argument: *mut c_void) -> c_int;
    fn uqm_png_report_error(message: *const c_char);
    fn SDL_malloc(size: usize) -> *mut c_void;
    fn SDL_free(pointer: *mut c_void);
    fn SDL_GetError() -> *const c_char;
    fn SDL_RWseek(context: *mut SDL_RWops, offset: i64, whence: c_int) -> i64;
    fn SDL_RWtell(context: *mut SDL_RWops) -> i64;
    fn SDL_RWread(context: *mut SDL_RWops, out: *mut c_void, size: usize, count: usize) -> usize;
    fn SDL_RWwrite(
        context: *mut SDL_RWops,
        data: *const c_void,
        size: usize,
        count: usize,
    ) -> usize;
    fn SDL_RWclose(context: *mut SDL_RWops) -> c_int;
    fn SDL_SetPaletteColors(
        palette: *mut SDL_Palette,
        colors: *const SDL_Color,
        first: c_int,
        count: c_int,
    ) -> c_int;
    fn SDL_MapRGB(format: *const SDL_PixelFormat, r: u8, g: u8, b: u8) -> u32;
    fn SDL_CreateRGBSurface(
        flags: u32,
        width: c_int,
        height: c_int,
        depth: c_int,
        r: u32,
        g: u32,
        b: u32,
        a: u32,
    ) -> *mut SDL_Surface;
    fn SDL_FreeSurface(surface: *mut SDL_Surface);
    fn SDL_LockSurface(surface: *mut SDL_Surface) -> c_int;
    fn SDL_UnlockSurface(surface: *mut SDL_Surface);
    fn SDL_SetColorKey(surface: *mut SDL_Surface, flag: c_int, key: u32) -> c_int;
    fn SDL_LowerBlit(
        source: *mut SDL_Surface,
        source_rect: *mut SDL_Rect,
        target: *mut SDL_Surface,
        target_rect: *mut SDL_Rect,
    ) -> c_int;
}

const SET_IO: u32 = 1;
const READ_INFO: u32 = 2;
const GET_HEADER: u32 = 3;
const STRIP_16: u32 = 4;
const INTERLACE: u32 = 5;
const PACKING: u32 = 6;
const EXPAND: u32 = 7;
const GET_TRNS: u32 = 8;
const GRAY_TO_RGB: u32 = 9;
const UPDATE_INFO: u32 = 10;
const READ_IMAGE: u32 = 11;
const GET_PALETTE: u32 = 12;
const SET_PALETTE: u32 = 13;
const SET_HEADER: u32 = 14;
const SET_BGR: u32 = 15;
const WRITE_INFO: u32 = 16;
const WRITE_IMAGE: u32 = 17;
const WRITE_END: u32 = 18;
const GRAY: c_int = 0;
const PALETTE: c_int = 3;
const GRAY_ALPHA: c_int = 4;

type DecodeResult = Result<(), *const c_char>;

unsafe fn call(context: *mut c_void, op: u32, argument: *mut c_void) -> DecodeResult {
    // Every libpng operation returns normally through a C error guard. Never
    // place setjmp in Rust or ask a C guard to wrap the entire Rust decoder.
    if unsafe { uqm_png_call(context, op, argument) } == 1 {
        Ok(())
    } else {
        Err(c"Error reading the PNG file.".as_ptr())
    }
}

unsafe fn simple(context: *mut c_void, op: u32) -> DecodeResult {
    unsafe { call(context, op, ptr::null_mut()) }
}

unsafe extern "C" fn read_bytes(user: *mut c_void, out: *mut u8, count: usize) -> usize {
    // The C trampoline checks the count and can call png_error only after this
    // Rust frame has returned. Neither this callback nor SDL invokes libpng.
    unsafe { SDL_RWread(user, out.cast(), 1, count) }
}

unsafe extern "C" fn write_bytes(user: *mut c_void, data: *const u8, count: usize) -> usize {
    unsafe { SDL_RWwrite(user, data.cast(), 1, count) }
}

struct Reader {
    context: *mut c_void,
    surface: *mut SDL_Surface,
    rows: *mut *mut u8,
}

unsafe fn decode(reader: &mut Reader, source: *mut SDL_RWops) -> DecodeResult {
    unsafe {
        let mut io = PngIo {
            user: source,
            read: Some(read_bytes),
            write: None,
        };
        call(reader.context, SET_IO, (&mut io as *mut PngIo).cast())?;
        simple(reader.context, READ_INFO)?;
        let mut header = PngHeader::default();
        call(
            reader.context,
            GET_HEADER,
            (&mut header as *mut PngHeader).cast(),
        )?;
        simple(reader.context, STRIP_16)?;
        simple(reader.context, INTERLACE)?;
        simple(reader.context, PACKING)?;
        if header.color == GRAY {
            simple(reader.context, EXPAND)?;
        }

        let mut transparent = PngTransparency {
            alpha: ptr::null(),
            color: ptr::null(),
            count: 0,
            valid: 0,
        };
        call(
            reader.context,
            GET_TRNS,
            (&mut transparent as *mut PngTransparency).cast(),
        )?;
        let mut ckey = -1i32;
        if transparent.valid != 0 {
            if header.color == PALETTE {
                let mut index = 0;
                let mut candidate = -1;
                while index < transparent.count {
                    let alpha = *transparent.alpha.add(index as usize);
                    if alpha == 0 {
                        if candidate >= 0 {
                            break;
                        }
                        candidate = index;
                    } else if alpha != 255 {
                        break;
                    }
                    index += 1;
                }
                if index == transparent.count {
                    ckey = candidate;
                } else {
                    simple(reader.context, EXPAND)?;
                }
            } else {
                ckey = 0;
            }
        }
        if header.color == GRAY_ALPHA {
            simple(reader.context, GRAY_TO_RGB)?;
        }
        simple(reader.context, UPDATE_INFO)?;
        call(
            reader.context,
            GET_HEADER,
            (&mut header as *mut PngHeader).cast(),
        )?;

        let (r, g, b, a) = if header.channels >= 3 {
            #[cfg(target_endian = "little")]
            {
                (
                    0xff,
                    0xff00,
                    0xff0000,
                    if header.channels == 4 { 0xff000000 } else { 0 },
                )
            }
            #[cfg(target_endian = "big")]
            {
                let shift = if header.channels == 4 { 0 } else { 8 };
                (
                    0xff000000 >> shift,
                    0xff0000 >> shift,
                    0xff00 >> shift,
                    0xff >> shift,
                )
            }
        } else {
            (0, 0, 0, 0)
        };
        reader.surface = SDL_CreateRGBSurface(
            0,
            header.width as c_int,
            header.height as c_int,
            header.depth * header.channels,
            r,
            g,
            b,
            a,
        );
        if reader.surface.is_null() {
            return Err(SDL_GetError());
        }
        if ckey != -1 {
            if header.color != PALETTE {
                if transparent.color.is_null() {
                    return Err(c"Missing PNG transparency color".as_ptr());
                }
                ckey = SDL_MapRGB(
                    (*reader.surface).format,
                    (*transparent.color).red as u8,
                    (*transparent.color).green as u8,
                    (*transparent.color).blue as u8,
                ) as i32;
            }
            SDL_SetColorKey(reader.surface, 1, ckey as u32);
        }

        SDL_LockSurface(reader.surface);
        let bytes = mem::size_of::<*mut u8>()
            .checked_mul(header.height as usize)
            .ok_or(c"Out of memory".as_ptr())?;
        reader.rows = SDL_malloc(bytes).cast();
        if reader.rows.is_null() {
            return Err(c"Out of memory".as_ptr());
        }
        for row in 0..header.height as usize {
            reader.rows.add(row).write(
                (*reader.surface)
                    .pixels
                    .cast::<u8>()
                    .add(row * (*reader.surface).pitch as usize),
            );
        }
        call(reader.context, READ_IMAGE, reader.rows.cast())?;
        SDL_UnlockSurface(reader.surface);

        if !(*(*reader.surface).format).palette.is_null() {
            let mut png_palette = PngPalette {
                colors: ptr::null(),
                count: 0,
            };
            call(
                reader.context,
                GET_PALETTE,
                (&mut png_palette as *mut PngPalette).cast(),
            )?;
            // Upstream leaves SDL_Color.a uninitialized. Define opaque alpha
            // instead of transferring uninitialized stack bytes to SDL2.
            let mut palette = [SDL_Color {
                r: 0,
                g: 0,
                b: 0,
                a: 255,
            }; 256];
            if header.color == GRAY {
                png_palette.count = 256;
                for (index, color) in palette.iter_mut().enumerate() {
                    color.r = index as u8;
                    color.g = index as u8;
                    color.b = index as u8;
                }
            } else if png_palette.count > 0 {
                if png_palette.count > 256 || png_palette.colors.is_null() {
                    return Err(c"Invalid PNG palette".as_ptr());
                }
                for (index, color) in palette
                    .iter_mut()
                    .enumerate()
                    .take(png_palette.count as usize)
                {
                    let source = png_palette.colors.add(index).read();
                    color.r = source.red;
                    color.g = source.green;
                    color.b = source.blue;
                }
            }
            SDL_SetPaletteColors(
                (*(*reader.surface).format).palette,
                palette.as_ptr(),
                0,
                png_palette.count,
            );
        }
        Ok(())
    }
}

/// # Safety
/// Source is null or a live SDL2 RWops stream. Its callbacks obey SDL's contract
/// and must not invoke libpng errors or unwind across this call.
#[no_mangle]
pub unsafe extern "C" fn TFB_png_to_sdl(source: *mut SDL_RWops) -> *mut SDL_Surface {
    unsafe {
        if source.is_null() {
            return ptr::null_mut();
        }
        let start = SDL_RWtell(source);
        let context = uqm_png_create(0);
        let mut reader = Reader {
            context,
            surface: ptr::null_mut(),
            rows: ptr::null_mut(),
        };
        let result = if context.is_null() {
            Err(c"Couldn't allocate memory for PNG file".as_ptr())
        } else {
            decode(&mut reader, source)
        };
        if !context.is_null() {
            uqm_png_destroy(context);
        }
        if !reader.rows.is_null() {
            SDL_free(reader.rows.cast());
        }
        if let Err(error) = result {
            SDL_RWseek(source, start, 0);
            if !reader.surface.is_null() {
                SDL_FreeSurface(reader.surface);
            }
            uqm_png_report_error(error);
            ptr::null_mut()
        } else {
            reader.surface
        }
    }
}

unsafe fn encode(
    context: *mut c_void,
    surface: *mut SDL_Surface,
    destination: *mut SDL_RWops,
    rows: &mut *mut *mut u8,
) -> DecodeResult {
    unsafe {
        let mut io = PngIo {
            user: destination,
            read: None,
            write: Some(write_bytes),
        };
        call(context, SET_IO, (&mut io as *mut PngIo).cast())?;
        let format = (*surface).format;
        let mut color_type = 2;
        if (*format).BytesPerPixel > 0
            && (*format).BytesPerPixel <= 8
            && !(*format).palette.is_null()
        {
            color_type |= 1;
            let palette = (*format).palette;
            if (*palette).ncolors < 0 || (*palette).ncolors > 256 {
                return Err(c"Invalid SDL palette".as_ptr());
            }
            let mut colors = [PngColor::default(); 256];
            for (index, color) in colors
                .iter_mut()
                .enumerate()
                .take((*palette).ncolors as usize)
            {
                let source = (*palette).colors.add(index).read();
                color.red = source.r;
                color.green = source.g;
                color.blue = source.b;
            }
            let mut palette = PngPalette {
                colors: colors.as_ptr(),
                count: (*palette).ncolors,
            };
            call(
                context,
                SET_PALETTE,
                (&mut palette as *mut PngPalette).cast(),
            )?;
        } else if (*format).BytesPerPixel > 3 || (*format).Amask != 0 {
            color_type |= 4;
        }
        let mut header = PngHeader {
            width: (*surface).w as u32,
            height: (*surface).h as u32,
            depth: 8,
            color: color_type,
            ..PngHeader::default()
        };
        call(context, SET_HEADER, (&mut header as *mut PngHeader).cast())?;
        simple(context, PACKING)?;
        #[cfg(target_endian = "little")]
        let (r_mask, g_mask, b_mask) = (0xff000000, 0xff0000, 0xff00);
        #[cfg(target_endian = "big")]
        let (r_mask, g_mask, b_mask) = (0xff, 0xff00, 0xff0000);
        if (*format).Rmask == b_mask && (*format).Gmask == g_mask && (*format).Bmask == r_mask {
            simple(context, SET_BGR)?;
        }
        simple(context, WRITE_INFO)?;
        let bytes = mem::size_of::<*mut u8>()
            .checked_mul((*surface).h as usize)
            .ok_or(c"Out of memory".as_ptr())?;
        *rows = SDL_malloc(bytes).cast();
        if (*rows).is_null() {
            return Err(c"Out of memory".as_ptr());
        }
        for row in 0..(*surface).h as usize {
            (*rows).add(row).write(
                (*surface)
                    .pixels
                    .cast::<u8>()
                    .add(row * (*surface).pitch as usize),
            );
        }
        call(context, WRITE_IMAGE, (*rows).cast())?;
        simple(context, WRITE_END)?;
        Ok(())
    }
}

/// # Safety
/// Surface/destination are null or live SDL2 values. On an error the original HD
/// API frees its surface; callers must honor that ownership rule. If freedst is
/// nonzero this closes the stream. Callbacks must return normally to their C
/// libpng trampoline before any PNG error is raised.
#[no_mangle]
pub unsafe extern "C" fn TFB_sdl_to_png(
    surface: *mut SDL_Surface,
    destination: *mut SDL_RWops,
    freedst: c_int,
) -> c_int {
    unsafe {
        let mut context = ptr::null_mut();
        let mut rows = ptr::null_mut();
        let result = if destination.is_null() {
            Err(c"Argument 2 to TFB_sdl_to_png can't be NULL\n".as_ptr())
        } else if surface.is_null() {
            Err(c"Argument 1 to TFB_sdl_to_png can't be NULL\n".as_ptr())
        } else {
            context = uqm_png_create(1);
            if context.is_null() {
                Err(c"Couldn't allocate memory for PNG file".as_ptr())
            } else {
                encode(context, surface, destination, &mut rows)
            }
        };
        if freedst != 0 && !destination.is_null() {
            SDL_RWclose(destination);
        }
        if !rows.is_null() {
            SDL_free(rows.cast());
        }
        if !context.is_null() {
            uqm_png_destroy(context);
        }
        if let Err(error) = result {
            if !surface.is_null() {
                SDL_FreeSurface(surface);
            }
            uqm_png_report_error(error);
            -1
        } else {
            0
        }
    }
}

/// # Safety
/// Source must be a live SDL2 surface. The returned surface either owns an
/// extra reference to the source or is a new 24-bit allocation, as in HD.
#[no_mangle]
pub unsafe extern "C" fn SDL_PNGFormatAlpha(source: *mut SDL_Surface) -> *mut SDL_Surface {
    unsafe {
        if source.is_null() {
            return ptr::null_mut();
        }
        let format = (*source).format;
        if (*format).BitsPerPixel <= 24 || (*format).Amask != 0 {
            (*source).refcount = (*source).refcount.wrapping_add(1);
            return source;
        }
        let mut rect = SDL_Rect {
            x: 0,
            y: 0,
            w: (*source).w,
            h: (*source).h,
        };
        let target = SDL_CreateRGBSurface(
            (*source).flags,
            (*source).w,
            (*source).h,
            24,
            (*format).Rmask,
            (*format).Gmask,
            (*format).Bmask,
            0,
        );
        if !target.is_null() {
            let rect_pointer = &mut rect as *mut SDL_Rect;
            SDL_LowerBlit(source, rect_pointer, target, rect_pointer);
        }
        target
    }
}
