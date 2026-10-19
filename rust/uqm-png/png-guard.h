#ifndef UQM_PNG_GUARD_H
#define UQM_PNG_GUARD_H
#include <stddef.h>
#include <stdint.h>
#include <png.h>

/* Binding-only error boundary. Rust owns image policy and SDL lifecycle. */
typedef struct uqm_png_context uqm_png_context;
typedef struct {
    uint32_t width, height;
    int depth, color, interlace, compression, filter, channels;
} uqm_png_header;
typedef struct {
    void *user;
    size_t (*read)(void *, unsigned char *, size_t);
    size_t (*write)(void *, const unsigned char *, size_t);
} uqm_png_io;
typedef struct {
    const unsigned char *alpha;
    const png_color_16 *color;
    int count, valid;
} uqm_png_transparency;
typedef struct { const png_color *colors; int count; } uqm_png_palette;
enum {
    UQM_PNG_SET_IO = 1, UQM_PNG_READ_INFO, UQM_PNG_GET_HEADER,
    UQM_PNG_STRIP16, UQM_PNG_INTERLACE, UQM_PNG_PACKING, UQM_PNG_EXPAND,
    UQM_PNG_GET_TRNS, UQM_PNG_GRAY_TO_RGB, UQM_PNG_UPDATE_INFO,
    UQM_PNG_READ_IMAGE, UQM_PNG_GET_PALETTE, UQM_PNG_SET_PALETTE,
    UQM_PNG_SET_HEADER, UQM_PNG_SET_BGR, UQM_PNG_WRITE_INFO,
    UQM_PNG_WRITE_IMAGE, UQM_PNG_WRITE_END
};
uqm_png_context *uqm_png_create(int writing);
void uqm_png_destroy(uqm_png_context *);
int uqm_png_call(uqm_png_context *, unsigned operation, void *argument);
void uqm_png_report_error(const char *message);
#endif
