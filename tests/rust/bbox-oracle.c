/* SPDX-License-Identifier: GPL-2.0-or-later
 * Compare original bbox.c with Rust using actual public graphics types.
 * Only opaque backend canvas extent retrieval is a fixture. */
#include <inttypes.h>
#include <limits.h>
#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "libs/graphics/bbox.h"

extern TFB_BoundingBox oracle_TFB_BBox;
extern int maxWidth, maxHeight, oracle_maxWidth, oracle_maxHeight;
void oracle_TFB_BBox_Init(int, int);
void oracle_TFB_BBox_Reset(void);
void oracle_TFB_BBox_SetClipRect(const RECT *);
void oracle_TFB_BBox_RegisterPoint(int, int);
void oracle_TFB_BBox_RegisterRect(const RECT *);
void oracle_TFB_BBox_RegisterCanvas(TFB_Canvas, int, int);

_Static_assert(sizeof(TFB_BoundingBox) == 20, "bounding box size");
_Static_assert(_Alignof(TFB_BoundingBox) == 4, "bounding box alignment");
_Static_assert(offsetof(TFB_BoundingBox, valid) == 0, "valid offset");
_Static_assert(offsetof(TFB_BoundingBox, region) == 4, "region offset");
_Static_assert(offsetof(TFB_BoundingBox, clip) == 12, "clip offset");

void *HMalloc(size_t size) { return malloc(size); }
void HFree(void *pointer) { free(pointer); }
void rust_eh_personality(void) { abort(); }
void log_add(int level, const char *format, ...) { (void)level; (void)format; }
#ifdef UQM_TEST_HD
unsigned int resolutionFactor = 2;
uint32_t uqm_hd_seed_a(void) { return 16807; }
#endif
static unsigned extent_calls;
void TFB_DrawCanvas_GetExtent(TFB_Canvas canvas, EXTENT *out)
{
    ++extent_calls;
    *out = *(const EXTENT *)canvas;
}

static uint64_t checks;
static uint32_t corpus = UINT32_C(0x431cc9e7);
static uint32_t rnd(void)
{
    corpus ^= corpus << 13;
    corpus ^= corpus >> 17;
    corpus ^= corpus << 5;
    return corpus;
}
static void equal(const char *label, int expected, int actual)
{
    ++checks;
    if (expected != actual) {
        fprintf(stderr, "FAIL %s at check %" PRIu64 ": C=%d Rust=%d\n",
                label, checks, expected, actual);
        exit(1);
    }
}
static void state(void)
{
    equal("valid", oracle_TFB_BBox.valid, TFB_BBox.valid);
    equal("region.x", oracle_TFB_BBox.region.corner.x, TFB_BBox.region.corner.x);
    equal("region.y", oracle_TFB_BBox.region.corner.y, TFB_BBox.region.corner.y);
    equal("region.w", oracle_TFB_BBox.region.extent.width, TFB_BBox.region.extent.width);
    equal("region.h", oracle_TFB_BBox.region.extent.height, TFB_BBox.region.extent.height);
    equal("clip.x", oracle_TFB_BBox.clip.corner.x, TFB_BBox.clip.corner.x);
    equal("clip.y", oracle_TFB_BBox.clip.corner.y, TFB_BBox.clip.corner.y);
    equal("clip.w", oracle_TFB_BBox.clip.extent.width, TFB_BBox.clip.extent.width);
    equal("clip.h", oracle_TFB_BBox.clip.extent.height, TFB_BBox.clip.extent.height);
    equal("width", oracle_maxWidth, maxWidth);
    equal("height", oracle_maxHeight, maxHeight);
}
static RECT random_rect(void)
{
    RECT rect = {{(COORD)rnd(), (COORD)rnd()}, {(SIZE)rnd(), (SIZE)rnd()}};
    return rect;
}
#define BOTH(name, ...) do { oracle_##name(__VA_ARGS__); name(__VA_ARGS__); state(); } while (0)

int main(void)
{
    static const int boundaries[] = {INT_MIN, -65536, -32768, -1, 0, 1,
        240, 320, 960, 1280, 32767, 65535, INT_MAX};
    uint32_t iteration;
    state();
    for (iteration = 0; iteration < 100000; ++iteration) {
        RECT input = random_rect();
        EXTENT canvas = {(SIZE)rnd(), (SIZE)rnd()};
        int x = (int32_t)rnd(), y = (int32_t)rnd();
        int width = iteration < 169 ? boundaries[iteration % 13] : (int32_t)rnd();
        int height = iteration < 169 ? boundaries[iteration / 13] : (int32_t)rnd();
        TFB_BoundingBox initial;
        initial.valid = (iteration % 5) ? (int32_t)rnd() : 0;
        initial.region = random_rect(); initial.clip = random_rect();
        TFB_BBox = oracle_TFB_BBox = initial;
        BOTH(TFB_BBox_Init, width, height);
        BOTH(TFB_BBox_SetClipRect, NULL);
        BOTH(TFB_BBox_Reset);
        BOTH(TFB_BBox_RegisterPoint, x, y);
        BOTH(TFB_BBox_SetClipRect, &input);
        BOTH(TFB_BBox_RegisterPoint, y, x);
        BOTH(TFB_BBox_RegisterRect, &input);
        /* Each side must use its own global pointer. These are valid aliased
         * calls, and RegisterRect must re-read after the first point write. */
        oracle_TFB_BBox_RegisterRect(&oracle_TFB_BBox.region);
        TFB_BBox_RegisterRect(&TFB_BBox.region); state();
        oracle_TFB_BBox_RegisterRect(&oracle_TFB_BBox.clip);
        TFB_BBox_RegisterRect(&TFB_BBox.clip); state();
        oracle_TFB_BBox_SetClipRect(&oracle_TFB_BBox.region);
        TFB_BBox_SetClipRect(&TFB_BBox.region); state();
        oracle_TFB_BBox_SetClipRect(&oracle_TFB_BBox.clip);
        TFB_BBox_SetClipRect(&TFB_BBox.clip); state();
        BOTH(TFB_BBox_Reset);
        extent_calls = 0;
        BOTH(TFB_BBox_RegisterCanvas, &canvas, x, y);
        equal("backend callback count", 2, (int)extent_calls);
        BOTH(TFB_BBox_Init, 1280, 960);
        BOTH(TFB_BBox_SetClipRect, NULL);
        BOTH(TFB_BBox_RegisterPoint, iteration % 1280, iteration % 960);
    }
    printf("Bounding-box actual-C differential passed: %" PRIu64 " comparisons.\n", checks);
    return 0;
}
