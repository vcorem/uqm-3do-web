#![no_std]
#[path = "../../rust/uqm-png/png.rs"]
mod png;
use core::mem::{align_of, offset_of, size_of};
use png::*;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo<'_>) -> ! {
    unsafe extern "C" {
        fn abort() -> !;
    }
    unsafe { abort() }
}

#[no_mangle]
pub extern "C" fn uqm_png_test_layout(index: usize) -> usize {
    let values = [
        size_of::<SDL_Surface>(),
        align_of::<SDL_Surface>(),
        offset_of!(SDL_Surface, format),
        offset_of!(SDL_Surface, w),
        offset_of!(SDL_Surface, pitch),
        offset_of!(SDL_Surface, pixels),
        offset_of!(SDL_Surface, locked),
        offset_of!(SDL_Surface, clip_rect),
        offset_of!(SDL_Surface, refcount),
        size_of::<SDL_PixelFormat>(),
        align_of::<SDL_PixelFormat>(),
        offset_of!(SDL_PixelFormat, palette),
        offset_of!(SDL_PixelFormat, BitsPerPixel),
        offset_of!(SDL_PixelFormat, Rmask),
        offset_of!(SDL_PixelFormat, Rloss),
        offset_of!(SDL_PixelFormat, Rshift),
        offset_of!(SDL_PixelFormat, next),
        size_of::<SDL_Palette>(),
        offset_of!(SDL_Palette, colors),
        offset_of!(SDL_Palette, refcount),
        size_of::<SDL_Color>(),
        size_of::<SDL_Rect>(),
        size_of::<PngHeader>(),
        align_of::<PngHeader>(),
        offset_of!(PngHeader, channels),
        size_of::<PngIo>(),
        size_of::<PngColor16>(),
        align_of::<PngColor16>(),
        offset_of!(PngColor16, gray),
        size_of::<PngTransparency>(),
        offset_of!(PngTransparency, count),
        size_of::<PngPalette>(),
        offset_of!(PngPalette, count),
        size_of::<PngColor>(),
    ];
    values.get(index).copied().unwrap_or(usize::MAX)
}
