/* SPDX-License-Identifier: GPL-2.0-or-later
 * libpng's non-local errors are contained entirely within C stack frames.
 * There is no decoder/writer image policy in this external-codec adapter. */
#include "png-guard.h"
#include <setjmp.h>
#include <stdio.h>
#include <stdlib.h>

void uqm_png_report_error(const char *message)
{ fprintf(stderr, "%s", message); }

struct uqm_png_context {
    png_structp png;
    png_infop info;
    uqm_png_io io;
    int writing;
};

static void read_trampoline(png_structp png, png_bytep output, png_size_t size)
{
    uqm_png_context *context = png_get_io_ptr(png);
    size_t count = context->io.read ? context->io.read(context->io.user, output, size) : 0;
    /* The Rust I/O callback has RETURNED before png_error can longjmp. */
    if (count != size) png_error(png, "Short read from PNG input");
}

static void write_trampoline(png_structp png, png_bytep input, png_size_t size)
{
    uqm_png_context *context = png_get_io_ptr(png);
    size_t count = context->io.write ? context->io.write(context->io.user, input, size) : 0;
    /* Never call png_error from a Rust callback. */
    if (count != size) png_error(png, "Short write to PNG output");
}

uqm_png_context *uqm_png_create(int writing)
{
    uqm_png_context *context = calloc(1, sizeof(*context));
    if (!context) return NULL;
    context->writing = writing != 0;
    if (writing)
        context->png = png_create_write_struct(PNG_LIBPNG_VER_STRING, NULL, NULL, NULL);
    else
        context->png = png_create_read_struct(PNG_LIBPNG_VER_STRING, NULL, NULL, NULL);
    if (!context->png) { free(context); return NULL; }
    if (setjmp(png_jmpbuf(context->png))) {
        uqm_png_destroy(context);
        return NULL;
    }
    context->info = png_create_info_struct(context->png);
    if (!context->info) { uqm_png_destroy(context); return NULL; }
    return context;
}

void uqm_png_destroy(uqm_png_context *context)
{
    if (!context) return;
    if (context->png && !setjmp(png_jmpbuf(context->png))) {
        if (context->writing)
            png_destroy_write_struct(&context->png, &context->info);
        else
            png_destroy_read_struct(&context->png, &context->info, NULL);
    }
    free(context);
}

int uqm_png_call(uqm_png_context *context, unsigned operation, void *argument)
{
    if (!context || !context->png) return 0;
    if (setjmp(png_jmpbuf(context->png))) return 0;
    switch (operation) {
    case UQM_PNG_SET_IO:
        context->io = *(const uqm_png_io *)argument;
        if (context->writing) png_set_write_fn(context->png, context, write_trampoline, NULL);
        else png_set_read_fn(context->png, context, read_trampoline);
        break;
    case UQM_PNG_READ_INFO: png_read_info(context->png, context->info); break;
    case UQM_PNG_GET_HEADER: {
        uqm_png_header *header = argument;
        if (!png_get_IHDR(context->png, context->info, &header->width, &header->height,
                &header->depth, &header->color, &header->interlace,
                &header->compression, &header->filter)) return 0;
        header->channels = png_get_channels(context->png, context->info);
        break;
    }
    case UQM_PNG_STRIP16: png_set_strip_16(context->png); break;
    case UQM_PNG_INTERLACE: (void)png_set_interlace_handling(context->png); break;
    case UQM_PNG_PACKING: png_set_packing(context->png); break;
    case UQM_PNG_EXPAND: png_set_expand(context->png); break;
    case UQM_PNG_GET_TRNS: {
        uqm_png_transparency *transparency = argument;
        png_bytep alpha = NULL;
        png_color_16p color = NULL;
        transparency->count = 0;
        transparency->valid = png_get_valid(context->png, context->info, PNG_INFO_tRNS) != 0;
        if (transparency->valid)
            png_get_tRNS(context->png, context->info, &alpha, &transparency->count, &color);
        transparency->alpha = alpha;
        transparency->color = color;
        break;
    }
    case UQM_PNG_GRAY_TO_RGB: png_set_gray_to_rgb(context->png); break;
    case UQM_PNG_UPDATE_INFO: png_read_update_info(context->png, context->info); break;
    case UQM_PNG_READ_IMAGE: png_read_image(context->png, argument); break;
    case UQM_PNG_GET_PALETTE: {
        uqm_png_palette *palette = argument;
        png_colorp colors = NULL;
        palette->count = 0;
        png_get_PLTE(context->png, context->info, &colors, &palette->count);
        palette->colors = colors;
        break;
    }
    case UQM_PNG_SET_PALETTE: {
        const uqm_png_palette *palette = argument;
        png_set_PLTE(context->png, context->info, palette->colors, palette->count);
        break;
    }
    case UQM_PNG_SET_HEADER: {
        const uqm_png_header *header = argument;
        png_set_IHDR(context->png, context->info, header->width, header->height,
                header->depth, header->color, header->interlace,
                header->compression, header->filter);
        break;
    }
    case UQM_PNG_SET_BGR: png_set_bgr(context->png); break;
    case UQM_PNG_WRITE_INFO: png_write_info(context->png, context->info); break;
    case UQM_PNG_WRITE_IMAGE: png_write_image(context->png, argument); break;
    case UQM_PNG_WRITE_END: png_write_end(context->png, context->info); break;
    default: return 0;
    }
    return 1;
}
