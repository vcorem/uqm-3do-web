/* Independent fixtures + unchanged actual UQM/MegaMod C decoder/writer oracle.
 * SPDX-License-Identifier: GPL-2.0-or-later */
#include <SDL.h>
#include <png.h>
#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "png-guard.h"

extern SDL_Surface *TFB_png_to_sdl(SDL_RWops *);
extern int TFB_sdl_to_png(SDL_Surface *, SDL_RWops *, int);
extern SDL_Surface *SDL_PNGFormatAlpha(SDL_Surface *);
extern SDL_Surface *oracle_original_decode(SDL_RWops *);
extern SDL_Surface *oracle_hd_decode(SDL_RWops *);
extern int oracle_hd_write(SDL_Surface *, SDL_RWops *, int);
extern SDL_Surface *oracle_hd_format(SDL_Surface *);
extern size_t uqm_png_test_layout(size_t);
void rust_eh_personality(void) { abort(); }
static uint64_t checks;
static unsigned fixture_number, malformed, truncated, writers;
static const char *phase = "ABI";

static void equal(const char *field, uint64_t expected, uint64_t actual)
{
    ++checks;
    if (expected != actual) {
        fprintf(stderr, "\nFAIL %s fixture=%u field=%s C=%llu Rust=%llu\n", phase,
                fixture_number, field, (unsigned long long)expected, (unsigned long long)actual);
        exit(1);
    }
}
static void compare_bytes(const char *field, const void *a, const void *b, size_t length)
{
    size_t i;
    for (i = 0; i < length; ++i) equal(field, ((const uint8_t *)a)[i], ((const uint8_t *)b)[i]);
}
static void check_abi(void)
{
    size_t i, values[] = {
        sizeof(SDL_Surface), _Alignof(SDL_Surface),
        offsetof(SDL_Surface, format), offsetof(SDL_Surface, w),
        offsetof(SDL_Surface, pitch), offsetof(SDL_Surface, pixels),
        offsetof(SDL_Surface, locked), offsetof(SDL_Surface, clip_rect),
        offsetof(SDL_Surface, refcount), sizeof(SDL_PixelFormat),
        _Alignof(SDL_PixelFormat), offsetof(SDL_PixelFormat, palette),
        offsetof(SDL_PixelFormat, BitsPerPixel), offsetof(SDL_PixelFormat, Rmask),
        offsetof(SDL_PixelFormat, Rloss), offsetof(SDL_PixelFormat, Rshift),
        offsetof(SDL_PixelFormat, next), sizeof(SDL_Palette),
        offsetof(SDL_Palette, colors), offsetof(SDL_Palette, refcount),
        sizeof(SDL_Color), sizeof(SDL_Rect), sizeof(uqm_png_header),
        _Alignof(uqm_png_header), offsetof(uqm_png_header, channels), sizeof(uqm_png_io),
        sizeof(png_color_16), _Alignof(png_color_16), offsetof(png_color_16, gray),
        sizeof(uqm_png_transparency), offsetof(uqm_png_transparency, count),
        sizeof(uqm_png_palette), offsetof(uqm_png_palette, count), sizeof(png_color),
    };
    for (i = 0; i < sizeof(values) / sizeof(values[0]); ++i) equal("layout", values[i], uqm_png_test_layout(i));
}

struct buffer { uint8_t *data; size_t length, capacity; };
static void fixture_write(png_structp png, png_bytep data, png_size_t length)
{
    struct buffer *b = png_get_io_ptr(png);
    if (b->length + length > b->capacity) {
        b->capacity = (b->length + length) * 2;
        b->data = realloc(b->data, b->capacity);
        if (!b->data) abort();
    }
    memcpy(b->data + b->length, data, length);
    b->length += length;
}
static unsigned sample(unsigned x, unsigned y, unsigned channel, unsigned depth)
{ return (x * 137 + y * 83 + channel * 53) & ((1u << depth) - 1); }
static struct buffer make_fixture(int color, int depth, int interlace, int transparency, int width, int height)
{
    struct buffer b = {0};
    png_structp png = png_create_write_struct(PNG_LIBPNG_VER_STRING, NULL, NULL, NULL);
    png_infop info = png_create_info_struct(png);
    png_color colors[256];
    png_byte alpha[256], **rows;
    png_color_16 transparent = {0};
    int channels = color == PNG_COLOR_TYPE_RGBA ? 4 : color == PNG_COLOR_TYPE_RGB ? 3 : color == PNG_COLOR_TYPE_GRAY_ALPHA ? 2 : 1;
    size_t rowbytes = ((size_t)width * channels * depth + 7) / 8;
    unsigned x, y, c, count = color == PNG_COLOR_TYPE_PALETTE ? 1u << depth : 0;
    if (setjmp(png_jmpbuf(png))) abort();
    png_set_write_fn(png, &b, fixture_write, NULL);
    png_set_IHDR(png, info, width, height, depth, color, interlace, 0, 0);
    if (count) {
        for (x = 0; x < count; ++x) {
            colors[x].red = x * 17; colors[x].green = x * 71; colors[x].blue = x * 137;
            alpha[x] = 255;
        }
        png_set_PLTE(png, info, colors, count);
        if (transparency) {
            if (transparency != 4) alpha[1] = transparency == 3 ? 91 : 0;
            if (transparency == 2) alpha[0] = 0;
            png_set_tRNS(png, info, alpha, count, NULL);
        }
    } else if (transparency) {
        transparent.red = sample(0, 0, 0, depth);
        transparent.green = sample(0, 0, 1, depth);
        transparent.blue = sample(0, 0, 2, depth);
        transparent.gray = sample(0, 0, 0, depth);
        png_set_tRNS(png, info, NULL, 0, &transparent);
    }
    rows = calloc(height, sizeof(*rows));
    for (y = 0; y < (unsigned)height; ++y) {
        rows[y] = calloc(rowbytes, 1);
        for (x = 0; x < (unsigned)width; ++x) for (c = 0; c < (unsigned)channels; ++c) {
            unsigned value = sample(x, y, c, depth);
            size_t index = x * channels + c;
            if (depth < 8) rows[y][index * depth / 8] |= value << (8 - depth - index * depth % 8);
            else if (depth == 8) rows[y][index] = value;
            else { rows[y][index * 2] = value >> 8; rows[y][index * 2 + 1] = value; }
        }
    }
    png_write_info(png, info);
    png_write_image(png, rows);
    png_write_end(png, info);
    for (y = 0; y < (unsigned)height; ++y) free(rows[y]);
    free(rows);
    png_destroy_write_struct(&png, &info);
    return b;
}

static void compare_surface(SDL_Surface *a, SDL_Surface *b)
{
    int x, y;
    uint32_t keya = 0, keyb = 0;
    equal("surface presence", a != NULL, b != NULL);
    if (!a || !b) return;
    equal("width", a->w, b->w); equal("height", a->h, b->h);
    equal("pitch", a->pitch, b->pitch); equal("flags", a->flags, b->flags);
    equal("bits", a->format->BitsPerPixel, b->format->BitsPerPixel);
    equal("bytes", a->format->BytesPerPixel, b->format->BytesPerPixel);
    equal("Rmask", a->format->Rmask, b->format->Rmask); equal("Gmask", a->format->Gmask, b->format->Gmask);
    equal("Bmask", a->format->Bmask, b->format->Bmask); equal("Amask", a->format->Amask, b->format->Amask);
    equal("colorkey present", SDL_GetColorKey(a, &keya), SDL_GetColorKey(b, &keyb));
    equal("colorkey", keya, keyb);
    equal("palette presence", a->format->palette != NULL, b->format->palette != NULL);
    if (a->format->palette) {
        equal("palette count", a->format->palette->ncolors, b->format->palette->ncolors);
        for (x = 0; x < a->format->palette->ncolors; ++x) {
            compare_bytes("palette RGB", &a->format->palette->colors[x], &b->format->palette->colors[x], 3);
            /* Upstream copies uninitialized palette alpha. Test the intended
             * opaque palette semantics; do not turn indeterminate bytes into
             * an equivalence requirement. Pixel alpha in RGBA remains exact. */
            equal("defined palette alpha", 255, b->format->palette->colors[x].a);
            a->format->palette->colors[x].a = 255;
        }
    }
    for (y = 0; y < a->h; ++y) {
        const uint8_t *pa = (const uint8_t *)a->pixels + y * a->pitch;
        const uint8_t *pb = (const uint8_t *)b->pixels + y * b->pitch;
        compare_bytes("pixel bytes", pa, pb, (size_t)a->w * a->format->BytesPerPixel);
        for (x = 0; x < a->w; ++x) {
            uint32_t va = 0, vb = 0;
            uint8_t ca[4], cb[4];
            memcpy(&va, pa + x * a->format->BytesPerPixel, a->format->BytesPerPixel);
            memcpy(&vb, pb + x * b->format->BytesPerPixel, b->format->BytesPerPixel);
            SDL_GetRGBA(va, a->format, ca, ca + 1, ca + 2, ca + 3);
            SDL_GetRGBA(vb, b->format, cb, cb + 1, cb + 2, cb + 3);
            compare_bytes("mapped pixel RGBA", ca, cb, sizeof(ca));
        }
    }
}
static size_t compare_decode(const uint8_t *data, size_t length, int expect_valid)
{
    unsigned edition;
    size_t consumed = 0;
    uint8_t *with_prefix = malloc(length + 9);
    memset(with_prefix, 0x96, 9); memcpy(with_prefix + 9, data, length);
    for (edition = 0; edition < 2; ++edition) {
        SDL_RWops *a = SDL_RWFromConstMem(with_prefix, length + 9), *b = SDL_RWFromConstMem(with_prefix, length + 9);
        SDL_Surface *ca, *rb;
        SDL_RWseek(a, 9, RW_SEEK_SET); SDL_RWseek(b, 9, RW_SEEK_SET);
        ca = edition ? oracle_hd_decode(a) : oracle_original_decode(a);
        rb = TFB_png_to_sdl(b);
        equal("expected decode status", expect_valid, ca != NULL);
        compare_surface(ca, rb);
        equal("stream position", SDL_RWtell(a), SDL_RWtell(b));
        consumed = SDL_RWtell(b) - 9;
        SDL_FreeSurface(ca); SDL_FreeSurface(rb);
        SDL_RWclose(a); SDL_RWclose(b);
    }
    free(with_prefix);
    return consumed;
}
static void reject_truncated(const uint8_t *data, size_t length)
{
    SDL_RWops *stream = SDL_RWFromConstMem(data, length);
    SDL_Surface *surface = TFB_png_to_sdl(stream);
    equal("truncated rejected", 1, surface == NULL);
    equal("truncated position restored", 0, SDL_RWtell(stream));
    SDL_FreeSurface(surface); SDL_RWclose(stream);
    ++truncated;
}
static void fixture_case(int color, int depth, int interlace, int transparency, int width, int height)
{
    struct buffer b = make_fixture(color, depth, interlace, transparency, width, height);
    size_t consumed, i;
    ++fixture_number;
    phase = "valid decode";
    consumed = compare_decode(b.data, b.length, 1);
    phase = "malformed decode";
    b.data[0] ^= 1;
    compare_decode(b.data, b.length, 0);
    b.data[0] ^= 1;
    b.data[29] ^= 1; /* IHDR CRC: deterministic malformed input, no short read. */
    compare_decode(b.data, b.length, 0);
    b.data[29] ^= 1;
    malformed += 2;
    phase = "truncated decode";
    reject_truncated(b.data, 1);
    reject_truncated(b.data, consumed / 2);
    reject_truncated(b.data, consumed - 1);
    if (fixture_number <= 3) for (i = 1; i < consumed; ++i) reject_truncated(b.data, i);
    /* Post-IDAT data is deliberately ignored by both legacy decoders. */
    phase = "omitted post-IDAT data";
    compare_decode(b.data, consumed, 1);
    free(b.data);
}

struct counted_stream { SDL_RWops *inner; unsigned closes; };
static Sint64 stream_size(SDL_RWops *rw) { return SDL_RWsize(((struct counted_stream *)rw->hidden.unknown.data1)->inner); }
static Sint64 stream_seek(SDL_RWops *rw, Sint64 offset, int whence)
{ return SDL_RWseek(((struct counted_stream *)rw->hidden.unknown.data1)->inner, offset, whence); }
static size_t stream_write(SDL_RWops *rw, const void *data, size_t size, size_t count)
{ return SDL_RWwrite(((struct counted_stream *)rw->hidden.unknown.data1)->inner, data, size, count); }
static int stream_close(SDL_RWops *rw)
{
    struct counted_stream *s = rw->hidden.unknown.data1;
    ++s->closes; SDL_RWclose(s->inner); SDL_FreeRW(rw); return 0;
}
static SDL_RWops *output_stream(struct counted_stream *state, void *data, size_t size)
{
    SDL_RWops *rw = SDL_AllocRW();
    memset(state, 0, sizeof(*state)); state->inner = SDL_RWFromMem(data, size);
    rw->size = stream_size; rw->seek = stream_seek; rw->write = stream_write; rw->close = stream_close;
    rw->hidden.unknown.data1 = state;
    return rw;
}
static SDL_Surface *source_surface(unsigned format, int width, int height)
{
    SDL_Surface *surface = SDL_CreateRGBSurfaceWithFormat(0, width, height, SDL_BITSPERPIXEL(format), format);
    int x, y;
    equal("source allocation", 1, surface != NULL);
    if (surface->format->palette) {
        SDL_Color palette[256];
        for (x = 0; x < 256; ++x) palette[x] = (SDL_Color){x * 17, x * 29, x * 73, 255};
        SDL_SetPaletteColors(surface->format->palette, palette, 0, 256);
    }
    for (y = 0; y < height; ++y) for (x = 0; x < width; ++x) {
        unsigned value = SDL_MapRGBA(surface->format, x * 41 + y, y * 17 + x, x * 17 + y * 19, x * 23 + y * 37);
        memcpy((uint8_t *)surface->pixels + y * surface->pitch + x * surface->format->BytesPerPixel,
                &value, surface->format->BytesPerPixel);
    }
    return surface;
}
static void writer_case(unsigned format, int width, int height, int close)
{
    uint8_t a[65536], b[65536];
    struct counted_stream sa, sb;
    SDL_Surface *ca = source_surface(format, width, height), *rb = source_surface(format, width, height);
    SDL_RWops *wa = output_stream(&sa, a, sizeof(a)), *wb = output_stream(&sb, b, sizeof(b));
    Sint64 lengtha, lengthb;
    int result_a, result_b;
    phase = "HD writer";
    /* With closed streams, compare complete deterministic output buffers. */
    memset(a, 0xa5, sizeof(a)); memset(b, 0xa5, sizeof(b));
    result_a = oracle_hd_write(ca, wa, close); result_b = TFB_sdl_to_png(rb, wb, close);
    equal("writer success", 0, result_a); equal("writer result", result_a, result_b);
    equal("close ownership", sa.closes, sb.closes); equal("requested close", close, sb.closes);
    compare_bytes("PNG encoded bytes", a, b, sizeof(a));
    if (!close) {
        lengtha = SDL_RWtell(wa); lengthb = SDL_RWtell(wb);
        equal("encoded length", lengtha, lengthb);
        compare_decode(a, lengtha, 1); compare_decode(b, lengthb, 1);
        SDL_RWclose(wa); SDL_RWclose(wb);
    }
    SDL_FreeSurface(ca); SDL_FreeSurface(rb);
    ++writers;
}
static void test_writers(void)
{
    static const unsigned formats[] = { SDL_PIXELFORMAT_INDEX8, SDL_PIXELFORMAT_RGB24,
        SDL_PIXELFORMAT_BGR24, SDL_PIXELFORMAT_RGBA32, SDL_PIXELFORMAT_BGRA32,
        SDL_PIXELFORMAT_ARGB8888, SDL_PIXELFORMAT_RGB888 };
    unsigned i;
    for (i = 0; i < sizeof(formats) / sizeof(formats[0]); ++i) {
        SDL_Surface *ca, *rb, *formatted_a, *formatted_b;
        uint8_t output[4];
        struct counted_stream state;
        SDL_RWops *rw;
        writer_case(formats[i], 7, 5, 0);
        writer_case(formats[i], 1, 1, 1);
        phase = "alpha format helper";
        ca = source_surface(formats[i], 7, 5); rb = source_surface(formats[i], 7, 5);
        formatted_a = oracle_hd_format(ca); formatted_b = SDL_PNGFormatAlpha(rb);
        equal("format reused source", ca == formatted_a, rb == formatted_b);
        equal("format source refcount", ca->refcount, rb->refcount);
        compare_surface(formatted_a, formatted_b);
        SDL_FreeSurface(formatted_a); SDL_FreeSurface(formatted_b); SDL_FreeSurface(ca); SDL_FreeSurface(rb);
        phase = "short output";
        rb = source_surface(formats[i], 7, 5);
        ++rb->refcount; /* Preserve one caller reference to observe error ownership. */
        rw = output_stream(&state, output, sizeof(output));
        equal("short write error", (uint64_t)-1, TFB_sdl_to_png(rb, rw, 1));
        equal("short write frees surface reference", 1, rb->refcount);
        equal("short write closes stream", 1, state.closes);
        SDL_FreeSurface(rb);
    }
    phase = "null argument handling";
    equal("null source", 1, TFB_png_to_sdl(NULL) == NULL);
    equal("null writer", (uint64_t)-1, TFB_sdl_to_png(NULL, NULL, 1));
}
int main(void)
{
    static const int colors[] = {0, 2, 3, 4, 6};
    static const int depths[] = {1, 2, 4, 8, 16};
    unsigned color, depth, interlace, trans, dimension;
    check_abi();
    for (color = 0; color < 5; ++color) for (depth = 0; depth < 5; ++depth) {
        if (colors[color] == 3 && depths[depth] == 16) continue;
        if (colors[color] != 0 && colors[color] != 3 && depths[depth] < 8) continue;
        for (interlace = 0; interlace < 2; ++interlace)
            for (trans = 0; trans < (colors[color] == 3 ? 5u : colors[color] == 0 || colors[color] == 2 ? 2u : 1u); ++trans)
                for (dimension = 0; dimension < 2; ++dimension)
                    fixture_case(colors[color], depths[depth], interlace, trans, dimension ? 7 : 1, dimension ? 5 : 1);
    }
    test_writers();
    printf("PASS PNG actual-C oracle: %llu checks, %u fixtures, %u malformed cases, %u truncated cases, %u writer cases; pointers=%zu\n",
            (unsigned long long)checks, fixture_number, malformed, truncated, writers, sizeof(void *));
    return 0;
}
