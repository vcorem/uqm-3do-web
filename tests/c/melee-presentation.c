/* Exercise the production C registry with a small owned-image backend. */
#include <assert.h>
#include <stdlib.h>
#include <string.h>
#include "libs/graphics/melee_presentation.h"
#include "libs/uio.h"
#include "uqm/globdata.h"

GLOBDATA GlobData;
uio_DirHandle *contentDir;
static unsigned allocated, deleted;
void *HMalloc (size_t size) { return malloc (size); }
void HFree (void *pointer) { free (pointer); }
uio_Stream *uio_fopen (uio_DirHandle *dir, const char *path, const char *mode)
{
    (void) dir; (void) mode;
    return strstr (path, "arilou/portrait-cap-005.png") ? NULL : (uio_Stream *) 1;
}
int uio_fclose (uio_Stream *stream) { (void) stream; return 0; }
TFB_Canvas TFB_DrawCanvas_LoadFromFile (void *dir, const char *path)
{
    EXTENT *size = malloc (sizeof *size);
    (void) dir;
    size->width = strstr (path, "cap-000.png") ? 220 : 24;
    size->height = strstr (path, "cap-000.png") ? 120 : 16;
    if (strstr (path, "chenjesu/portrait-cap-004.png")) ++size->width;
    return size;
}
TFB_Image *TFB_DrawImage_New (TFB_Canvas canvas)
{
    TFB_Image *image = calloc (1, sizeof *image);
    image->extent = *(EXTENT *) canvas;
    image->NormalImg = canvas;
    ++allocated;
    return image;
}
void TFB_DrawImage_Delete (TFB_Image *image)
{
    UQM_MeleePresentationForget (image);
    free (image->NormalImg);
    free (image);
    ++deleted;
}
void TFB_DrawScreen_SetMipmap (TFB_Image *image, TFB_Image *mipmap, int x, int y)
{ UQM_MeleePresentationMipmap (image, mipmap, x, y); }
void TFB_DrawScreen_Image (TFB_Image *image, int x, int y, int scale,
    int mode, TFB_ColorMap *cmap, DrawMode draw, SCREEN screen)
{ (void) image; (void) x; (void) y; (void) scale; (void) mode; (void) cmap; (void) draw; (void) screen; }

static void loadCaptain (const char *race, TFB_Image **frames, unsigned count)
{
    char source[160], resource[160];
    unsigned drawable = UQM_MeleePresentationNextDrawable (), row;
    snprintf (resource, sizeof resource, "addons/mm-hd/ships/%s/portrait-cap.ani", race);
    for (row = count; row--;) {
        EXTENT *size = malloc (sizeof *size);
        size->width = row ? 24 : 220; size->height = row ? 16 : 120;
        frames[row] = TFB_DrawImage_New (size);
        snprintf (source, sizeof source, "addons/mm-hd/ships/%s/portrait-cap-%03u.png", race, row == 2 ? 1 : row);
        UQM_MeleePresentationRegister (frames[row], source, resource, -3 * row, -2 * row, drawable, row);
    }
    UQM_MeleePresentationFinalizeCaptain (drawable, count);
}

int main (void)
{
    TFB_Image *human[15], *arilou[15], *chenjesu[15], *shofixti[20], *rendered[15];
    TFB_Image baseline;
    unsigned scope, nested, row, count;
    GLOBAL (CurrentActivity) = SUPER_MELEE;
    loadCaptain ("human", human, 15);
    baseline = *human[0];
    assert (UQM_GetMeleeRemasteredCaptainImageCount (5) == 15);
    UQM_MeleePresentationBegin (TRUE);
    assert (UQM_MeleePresentationImage (human[0]) == human[0]);
    UQM_MeleePresentationEnd ();
    scope = UQM_MeleePresentationCaptainBegin ();
    assert (UQM_MeleePresentationCaptainNeedsRefresh (0, human[0]));
    assert (!UQM_MeleePresentationCaptainNeedsRefresh (0, human[0]));
    assert (UQM_MeleePresentationCaptainNeedsRefresh (1, human[0]));
    for (row = 0; row < 15; ++row) {
        rendered[row] = UQM_MeleePresentationImage (human[row]);
        assert (rendered[row] != human[row]);
        assert (rendered[row]->NormalHs.x == (COORD) (-3 * row));
    }
    assert (rendered[1] != rendered[2]);
    assert (UQM_GetMeleeRemasteredCaptainSeenMask (5) == (1u << 15) - 1);
    assert (UQM_GetMeleeRemasteredCaptainDrawCount (5, 1) == 1);
    assert (UQM_GetMeleeRemasteredCaptainDrawCount (5, 2) == 1);
    UQM_SetMeleeRemastered (0);
    nested = UQM_MeleePresentationCaptainBegin ();
    for (row = 0; row < 15; ++row) assert (UQM_MeleePresentationImage (human[row]) == rendered[row]);
    UQM_MeleePresentationCaptainEnd (nested);
    UQM_MeleePresentationCaptainEnd (scope);
    count = UQM_GetMeleeRemasteredCaptainDrawCount (5, -1);
    scope = UQM_MeleePresentationCaptainBegin ();
    assert (UQM_MeleePresentationCaptainNeedsRefresh (0, human[0]));
    for (row = 0; row < 15; ++row) assert (UQM_MeleePresentationImage (human[row]) == human[row]);
    UQM_MeleePresentationCaptainEnd (scope);
    assert (UQM_GetMeleeRemasteredCaptainDrawCount (5, -1) == count);
    UQM_SetMeleeRemastered (1);
    GLOBAL (CurrentActivity) = IN_ENCOUNTER;
    scope = UQM_MeleePresentationCaptainBegin ();
    assert (!UQM_MeleePresentationCaptainNeedsRefresh (0, human[0]));
    assert (UQM_MeleePresentationImage (human[0]) == human[0]);
    UQM_MeleePresentationCaptainEnd (scope);
    GLOBAL (CurrentActivity) = SUPER_MELEE;
    loadCaptain ("arilou", arilou, 15);
    loadCaptain ("chenjesu", chenjesu, 15);
    loadCaptain ("shofixti", shofixti, 20);
    assert (UQM_GetMeleeRemasteredCaptainImageCount (1) == 0);
    assert (UQM_GetMeleeRemasteredCaptainImageCount (2) == 0);
    assert (UQM_GetMeleeRemasteredCaptainImageCount (13) == 20);
    scope = UQM_MeleePresentationCaptainBegin ();
    for (row = 0; row < 15; ++row) {
        assert (UQM_MeleePresentationImage (arilou[row]) == arilou[row]);
        assert (UQM_MeleePresentationImage (chenjesu[row]) == chenjesu[row]);
    }
    assert (UQM_MeleePresentationImage (shofixti[19]) != shofixti[19]);
    UQM_MeleePresentationCaptainEnd (scope);
    assert (UQM_GetMeleeRemasteredCaptainDrawCount (13, 19) == 1);
    assert (UQM_GetMeleeRemasteredCaptainSeenMask (13) == 1u << 19);
    /* A toggle between the live pose and its separate partial death stamp
     * must preserve the base style, without reverting the requested setting. */
    for (row = 0; row < 2; ++row) {
        UQM_SetMeleeRemastered (row);
        scope = UQM_MeleePresentationCaptainBegin ();
        UQM_MeleePresentationCaptainNeedsRefresh (1, shofixti[0]);
        assert ((UQM_MeleePresentationImage (shofixti[0]) != shofixti[0]) == (row != 0));
        UQM_MeleePresentationCaptainEnd (scope);
        UQM_SetMeleeRemastered (1 - row);
        scope = UQM_MeleePresentationCaptainBeginFinalFrame (1, shofixti[0]);
        nested = UQM_MeleePresentationCaptainBegin ();
        assert ((UQM_MeleePresentationImage (shofixti[19]) != shofixti[19]) == (row != 0));
        assert (UQM_GetMeleeRemastered () == (int) (1 - row));
        UQM_MeleePresentationCaptainEnd (nested);
        UQM_MeleePresentationCaptainEnd (scope);
        scope = UQM_MeleePresentationCaptainBegin ();
        assert ((UQM_MeleePresentationImage (shofixti[19]) != shofixti[19]) == (row == 0));
        assert (UQM_MeleePresentationCaptainNeedsRefresh (1, shofixti[0]));
        UQM_MeleePresentationCaptainEnd (scope);
    }
    UQM_SetMeleeRemastered (1);
    scope = UQM_MeleePresentationCaptainBegin ();
    UQM_MeleePresentationCaptainNeedsRefresh (1, shofixti[0]);
    UQM_MeleePresentationCaptainEnd (scope);
    UQM_SetMeleeRemastered (0);
    scope = UQM_MeleePresentationCaptainBeginFinalFrame (1, shofixti[1]);
    assert (UQM_MeleePresentationImage (shofixti[19]) == shofixti[19]);
    UQM_MeleePresentationCaptainEnd (scope);
    UQM_SetMeleeRemastered (1);
    GLOBAL (CurrentActivity) = IN_ENCOUNTER;
    scope = UQM_MeleePresentationCaptainBeginFinalFrame (1, shofixti[0]);
    assert (UQM_MeleePresentationImage (shofixti[19]) == shofixti[19]);
    UQM_MeleePresentationCaptainEnd (scope);
    GLOBAL (CurrentActivity) = SUPER_MELEE;
    assert (!memcmp (&baseline, human[0], sizeof baseline));
    for (row = 0; row < 15; ++row) {
        TFB_DrawImage_Delete (human[row]); TFB_DrawImage_Delete (arilou[row]); TFB_DrawImage_Delete (chenjesu[row]);
    }
    for (row = 0; row < 20; ++row) TFB_DrawImage_Delete (shofixti[row]);
    assert (UQM_GetMeleeRemasteredImageCount () == 0);
    loadCaptain ("human", human, 15);
    assert (UQM_GetMeleeRemasteredCaptainImageCount (5) == 15);
    scope = UQM_MeleePresentationCaptainBegin ();
    for (row = 0; row < 15; ++row) assert (UQM_MeleePresentationImage (human[row]) != human[row]);
    UQM_MeleePresentationCaptainEnd (scope);
    for (row = 0; row < 15; ++row) TFB_DrawImage_Delete (human[row]);
    assert (UQM_GetMeleeRemasteredImageCount () == 0);
    assert (allocated == deleted);
    puts ("C captain registry: composition, toggle, aliases, fallback, diagnostics and reload passed.");
    return 0;
}
