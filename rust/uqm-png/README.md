# UQM PNG orchestration in Rust

Both engine crates include `png.rs` through their existing `png2sdl` module.
It exports the original `TFB_png_to_sdl`, `TFB_sdl_to_png` and
`SDL_PNGFormatAlpha` C ABI. Rust owns PNG transform selection, SDL surfaces and
formats, transparency, palettes, row allocation, stream ownership and cleanup.
It uses SDL allocation and works without the Rust standard library.

`png-guard.c` is a binding to external libpng and stdio. Each libpng operation
has its own C `setjmp` scope. The C I/O trampoline waits for the Rust callback
to return before reporting a short transfer with `png_error`. Thus a libpng
`longjmp` never crosses a live Rust frame. Do not replace these guards with a
single C callback that wraps the Rust decoder, or call `png_error` inside the
Rust I/O callback. The guard requires libpng headers and libc; it does not
depend on SDL or contain image-format policy.

The hand port retains the Original decoder and HD writer behavior, including
palette indices, color keys, packed and interlaced input, 16-bit stripping,
stream rewind on failure and HD's rule that a failed write releases its input
surface. Three previously unsafe cases receive defined behavior:

- Palette alpha, which C left uninitialized, is opaque (255). Palette RGB,
  indices and nonpalette pixel alpha retain their original values.
- Short reads and writes fail through the C guard after the Rust callback
  returns. C previously ignored the transfer count.
- A null output stream is not closed when `freedst` is set.

The independent oracle compiles the unchanged Original and HD C sources beside
this module and uses real SDL/libpng headers to check structure sizes and
offsets. Native and shared-memory Wasm builds, each in debug and optimized
profiles, pass 1,292,704 comparisons per run: 152 valid fixtures, 304 malformed
inputs, 636 truncation cases and 14 HD writer cases. Comparisons include pixels,
formats, palette RGB, color keys, stream positions, ownership and byte-for-byte
encoded PNG output. Only the legacy uninitialized palette alpha is normalized
for comparisons. Native C oracle and guard code are ASan/UBSan instrumented.
See `tests/rust/test-png.sh` for the reproducible check.

The port is GPL-2.0-or-later, as are the UQM sources it adapts. Decoder ancestry
includes SDL2_image, copyright 1997–2019 Sam Lantinga, originally authored by
Philippe Lavoie, and libpng's example. The HD writer derives from driedfruit's
SDL_SavePNG. The complete GPL is in `../uqm-engine-original/COPYING`.
