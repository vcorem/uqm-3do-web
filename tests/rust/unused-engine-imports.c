/* A release staticlib codegen unit can include imports from unexercised engine
 * modules. Resolve them without silently implementing or testing those modules.
 * Their own oracles provide real tracked implementations instead.
 * SPDX-License-Identifier: GPL-2.0-or-later */
#include "velocity-environment.h"
#include <stdlib.h>
#include "libs/graphics/gfxintrn.h"
#include "libs/log.h"

void TFB_DrawCanvas_GetExtent(TFB_Canvas canvas, EXTENT *extent)
{
    (void)canvas;
    (void)extent;
    abort();
}

void log_add(log_Level level, const char *format, ...)
{
    (void)level;
    (void)format;
    abort();
}
