/* Actual-source queue and clipping oracle. SPDX-License-Identifier: GPL-2.0-or-later */
#include <inttypes.h>
#include <limits.h>
#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "uqm/displist.h"
#include "libs/graphics/gfxintrn.h"
#include "libs/log.h"

#ifdef UQM_TEST_HD
typedef DRECT clip_rect;
unsigned int resolutionFactor = 2;
uint32_t uqm_hd_seed_a(void) { return 16807; }
_Static_assert(sizeof(DRECT) == 16 && _Alignof(DRECT) == 4, "HD rectangle ABI");
#else
typedef RECT clip_rect;
#endif
BOOLEAN oracle_InitQueue(QUEUE *, COUNT, OBJ_SIZE);
BOOLEAN oracle_UninitQueue(QUEUE *);
void oracle_ReinitQueue(QUEUE *);
HLINK oracle_AllocLink(QUEUE *);
void oracle_FreeLink(QUEUE *, HLINK);
void oracle_PutQueue(QUEUE *, HLINK);
void oracle_InsertQueue(QUEUE *, HLINK, HLINK);
void oracle_RemoveQueue(QUEUE *, HLINK);
COUNT oracle_CountLinks(QUEUE *);
void oracle_ForAllLinks(QUEUE *, void (*)(LINK *, void *), void *);
INTERSECT_CODE oracle_BoxIntersect(RECT *, RECT *, RECT *);
void oracle_BoxUnion(RECT *, RECT *, RECT *);
INTERSECT_CODE oracle_clip_line(const clip_rect *, BRESENHAM_LINE *);

_Static_assert(sizeof(BOOLEAN) == 4 && _Alignof(BOOLEAN) == 4, "BOOLEAN ABI");
_Static_assert(sizeof(RECT) == 8 && _Alignof(RECT) == 2, "RECT ABI");
_Static_assert(sizeof(LINK) == 2 * sizeof(void *), "LINK ABI");
_Static_assert(offsetof(LINK, pred) == 0 && offsetof(LINK, succ) == sizeof(void *), "LINK fields");
_Static_assert(sizeof(QUEUE) == (sizeof(void *) == 8 ? 40 : 20), "QUEUE size");
_Static_assert(_Alignof(QUEUE) == _Alignof(void *), "QUEUE alignment");
_Static_assert(offsetof(QUEUE, head) == 0 && offsetof(QUEUE, tail) == sizeof(void *), "queue ends");
_Static_assert(offsetof(QUEUE, pq_tab) == 2 * sizeof(void *), "queue pool");
_Static_assert(offsetof(QUEUE, free_list) == 3 * sizeof(void *), "queue free list");
_Static_assert(offsetof(QUEUE, object_size) == 4 * sizeof(void *), "queue stride");
_Static_assert(offsetof(QUEUE, num_objects) == 4 * sizeof(void *) + 2, "queue capacity");
_Static_assert(sizeof(BRESENHAM_LINE) == 24 && _Alignof(BRESENHAM_LINE) == 4, "line ABI");
_Static_assert(offsetof(BRESENHAM_LINE, first) == 0 && offsetof(BRESENHAM_LINE, second) == 4, "line points");
_Static_assert(offsetof(BRESENHAM_LINE, abs_delta_x) == 8 && offsetof(BRESENHAM_LINE, abs_delta_y) == 10, "line deltas");
_Static_assert(offsetof(BRESENHAM_LINE, error_term) == 12, "line error");
_Static_assert(offsetof(BRESENHAM_LINE, end_points_exchanged) == 16, "line exchange flag");
_Static_assert(offsetof(BRESENHAM_LINE, intersect_code) == 20, "line outcode");

static uint64_t checks;
static uint32_t input_number, corpus = UINT32_C(0x5de19c47);
static const char *phase = "startup";
static unsigned log_calls, allocations, releases;
static int fail_allocation;
static size_t expected_allocation_size;
struct allocation { unsigned char *base; size_t length; };
static struct allocation live[8];

static uint32_t random_input(void)
{
    corpus ^= corpus << 13;
    corpus ^= corpus >> 17;
    corpus ^= corpus << 5;
    return corpus;
}
static void equal(const char *field, uint64_t expected, uint64_t actual)
{
    ++checks;
    if (expected != actual) {
        fprintf(stderr, "FAIL %s case=%" PRIu32 " %s C=%" PRIu64 " Rust=%" PRIu64 "\n",
                phase, input_number, field, expected, actual);
        exit(1);
    }
}
static void bytes_equal(const char *field, const void *expected, const void *actual, size_t length)
{
    const unsigned char *a = expected, *b = actual;
    size_t i;
    for (i = 0; i < length; ++i) equal(field, a[i], b[i]);
}
static void allocation_guards(const struct allocation *a)
{
    size_t i;
    for (i = 0; i < 32; ++i) {
        equal("allocation prefix", 0xa5, a->base[i]);
        equal("allocation suffix", 0xa5, a->base[32 + a->length + i]);
    }
}
void *HMalloc(size_t size)
{
    size_t i;
    equal("queue allocation size", expected_allocation_size, size);
    if (fail_allocation) return NULL;
    for (i = 0; i < 8 && live[i].base; ++i) {}
    equal("allocation tracker capacity", 1, i < 8);
    live[i].base = malloc(size + 64);
    if (!live[i].base) abort();
    live[i].length = size;
    memset(live[i].base, 0xa5, size + 64);
    memset(live[i].base + 32, 0, size);
    ++allocations;
    return live[i].base + 32;
}
void HFree(void *pointer)
{
    size_t i;
    if (!pointer) return;
    for (i = 0; i < 8 && (!live[i].base || live[i].base + 32 != pointer); ++i) {}
    equal("owned allocation freed", 1, i < 8);
    allocation_guards(&live[i]);
    free(live[i].base);
    live[i].base = NULL;
    ++releases;
}
void log_add(log_Level level, const char *format, ...)
{
    equal("queue diagnostic level", log_Debug, level);
    equal("queue diagnostic text", 0, strcmp(format, "AllocLink(): No more elements"));
    ++log_calls;
}
/* Other exports in a staticlib codegen unit may retain these imports. They are
 * not part of this oracle; calling either is a hard test failure. */
void rust_eh_personality(void) { abort(); }
void TFB_DrawCanvas_GetExtent(TFB_Canvas canvas, EXTENT *extent)
{ (void)canvas; (void)extent; abort(); }

static unsigned slot(const QUEUE *q, const void *pointer)
{
    uintptr_t value = (uintptr_t)pointer, base = (uintptr_t)q->pq_tab;
    if (!pointer) return 0;
    equal("pointer within queue pool", 1,
            value >= base && value < base + q->object_size * q->num_objects);
    equal("pointer at slot boundary", 0, (value - base) % q->object_size);
    return (unsigned)((value - base) / q->object_size) + 1;
}
static LINK *link_at(QUEUE *q, unsigned index)
{ return index ? (LINK *)(q->pq_tab + (index - 1) * q->object_size) : NULL; }

static void bounded_chain(const QUEUE *q, HLINK first, const char *label)
{
    unsigned count = 0;
    while (first) {
        (void)slot(q, first);
        equal(label, 1, ++count <= q->num_objects);
        first = ((LINK *)first)->succ;
    }
}
struct guarded_queue { uint64_t before[2]; QUEUE value; uint64_t after[2]; };
static void compare_queue(QUEUE *c, struct guarded_queue *r)
{
    unsigned i;
    QUEUE *q = &r->value;
    equal("head", slot(c, c->head), slot(q, q->head));
    equal("tail", slot(c, c->tail), slot(q, q->tail));
    equal("free list", slot(c, c->free_list), slot(q, q->free_list));
    equal("pool existence", c->pq_tab != NULL, q->pq_tab != NULL);
    equal("stride", c->object_size, q->object_size);
    equal("capacity", c->num_objects, q->num_objects);
    bytes_equal("queue trailing padding", (unsigned char *)c + offsetof(QUEUE, num_objects) + 1,
            (unsigned char *)q + offsetof(QUEUE, num_objects) + 1,
            sizeof(QUEUE) - offsetof(QUEUE, num_objects) - 1);
    for (i = 0; i < 2; ++i) {
        equal("queue prefix", UINT64_C(0xa5a5a5a5a5a5a5a5), r->before[i]);
        equal("queue suffix", UINT64_C(0xa5a5a5a5a5a5a5a5), r->after[i]);
    }
    if (!q->pq_tab) return;
    for (i = 1; i <= q->num_objects; ++i) {
        LINK *a = link_at(c, i), *b = link_at(q, i);
        equal("slot predecessor", slot(c, a->pred), slot(q, b->pred));
        equal("slot successor", slot(c, a->succ), slot(q, b->succ));
        bytes_equal("slot payload", a + 1, b + 1, q->object_size - sizeof(LINK));
    }
    bounded_chain(q, q->head, "active chain terminates");
    bounded_chain(q, q->free_list, "free chain terminates");
    equal("link count", oracle_CountLinks(c), CountLinks(q));
    for (i = 0; i < 8; ++i) if (live[i].base) allocation_guards(&live[i]);
}
struct callback_state { QUEUE *q; unsigned visited[255], count; int oracle, remove; };
static void visit_link(LINK *link, void *opaque)
{
    struct callback_state *state = opaque;
    equal("callback limit", 1, state->count < 255);
    state->visited[state->count++] = slot(state->q, link);
    if (state->q->object_size > sizeof(LINK)) ((unsigned char *)(link + 1))[0] ^= 0x7b;
    if (state->remove) {
        if (state->oracle) {
            oracle_RemoveQueue(state->q, link);
            oracle_FreeLink(state->q, link);
        } else {
            RemoveQueue(state->q, link);
            FreeLink(state->q, link);
        }
    }
}
static void visit_pair(QUEUE *c, QUEUE *r, int remove)
{
    struct callback_state a = {.q = c, .oracle = 1, .remove = remove};
    struct callback_state b = {.q = r, .oracle = 0, .remove = remove};
    oracle_ForAllLinks(c, visit_link, &a);
    ForAllLinks(r, visit_link, &b);
    equal("callback count", a.count, b.count);
    bytes_equal("callback order", a.visited, b.visited, a.count * sizeof(unsigned));
}
static void queue_case(unsigned capacity, unsigned stride)
{
    QUEUE c;
    struct guarded_queue r;
    unsigned state[256] = {0}, i, step;
    memset(&c, 0xa5, sizeof(c));
    memset(&r, 0xa5, sizeof(r));
    expected_allocation_size = capacity * stride;
    equal("queue init", oracle_InitQueue(&c, capacity, stride), InitQueue(&r.value, capacity, stride));
    compare_queue(&c, &r);
    /* Fill the whole pool, alternating head/middle/tail insertions, then verify
     * exhaustion, callback removal, cached-next traversal, and reuse. */
    for (i = 1; i <= capacity; ++i) {
        HLINK a = oracle_AllocLink(&c), b = AllocLink(&r.value);
        unsigned reference = i % 3 == 0 ? slot(&c, c.head) : 0;
        if (i % 3 == 1 && c.head) reference = slot(&c, ((LINK *)c.head)->succ);
        equal("allocation order", slot(&c, a), slot(&r.value, b));
        equal("initial slot order", i, slot(&r.value, b));
        oracle_InsertQueue(&c, a, link_at(&c, reference));
        InsertQueue(&r.value, b, link_at(&r.value, reference));
        compare_queue(&c, &r);
    }
    i = log_calls;
    equal("C exhausted", 1, oracle_AllocLink(&c) == NULL);
    equal("Rust exhausted", 1, AllocLink(&r.value) == NULL);
    equal("exhaustion diagnostics", i + 2, log_calls);
    visit_pair(&c, &r.value, 0);
    compare_queue(&c, &r);
    visit_pair(&c, &r.value, 1);
    compare_queue(&c, &r);
    for (step = 0; step < 192; ++step) {
        unsigned choice = random_input() % 12, index = random_input() % capacity + 1;
        if (choice == 0) {
            oracle_ReinitQueue(&c); ReinitQueue(&r.value);
            memset(state, 0, sizeof(state));
        } else if (choice == 1) {
            visit_pair(&c, &r.value, step & 1);
            if (step & 1) for (i = 1; i <= capacity; ++i) if (state[i] == 2) state[i] = 0;
        } else if (state[index] == 0) {
            HLINK a = oracle_AllocLink(&c), b = AllocLink(&r.value);
            equal("reused allocation order", slot(&c, a), slot(&r.value, b));
            if (b) state[slot(&r.value, b)] = 1;
        } else if (state[index] == 1) {
            if (choice & 1) {
                unsigned ref = slot(&c, c.head);
                if (choice & 2) ref = slot(&c, c.tail);
                oracle_InsertQueue(&c, link_at(&c, index), link_at(&c, ref));
                InsertQueue(&r.value, link_at(&r.value, index), link_at(&r.value, ref));
                state[index] = 2;
            } else {
                oracle_FreeLink(&c, link_at(&c, index)); FreeLink(&r.value, link_at(&r.value, index));
                state[index] = 0;
            }
        } else {
            oracle_RemoveQueue(&c, link_at(&c, index)); RemoveQueue(&r.value, link_at(&r.value, index));
            state[index] = 1;
            if (choice & 1) {
                oracle_PutQueue(&c, link_at(&c, index)); PutQueue(&r.value, link_at(&r.value, index));
                state[index] = 2;
            }
        }
        compare_queue(&c, &r);
    }
    equal("uninit result", oracle_UninitQueue(&c), UninitQueue(&r.value));
    compare_queue(&c, &r);
    equal("repeat uninit", oracle_UninitQueue(&c), UninitQueue(&r.value));
    compare_queue(&c, &r);
}
static void test_queues(void)
{
    unsigned capacity, extra;
    phase = "queue";
    for (capacity = 1; capacity <= 255; ++capacity) {
        for (extra = 0; extra <= 16; extra += 16) {
            ++input_number;
            queue_case(capacity, sizeof(LINK) + extra);
        }
    }
    queue_case(1, 65528);
    for (capacity = 1; capacity <= 255; ++capacity) {
        QUEUE c;
        struct guarded_queue r;
        memset(&c, 0xa5, sizeof(c)); memset(&r, 0xa5, sizeof(r));
        expected_allocation_size = capacity * sizeof(LINK);
        fail_allocation = 1;
        equal("failed init", oracle_InitQueue(&c, capacity, sizeof(LINK)),
                InitQueue(&r.value, capacity, sizeof(LINK)));
        compare_queue(&c, &r);
        fail_allocation = 0;
        oracle_UninitQueue(&c); UninitQueue(&r.value);
        compare_queue(&c, &r);
    }
    equal("allocation lifecycle", allocations, releases);
}

struct guarded_rects { uint32_t before[2]; RECT value[3]; uint32_t after[2]; };
static unsigned box_codes[256], line_codes[256];
static void box_case(RECT a, RECT b)
{
    unsigned alias, operation;
    for (alias = 0; alias < 6; ++alias) {
        for (operation = 0; operation < 2; ++operation) {
            struct guarded_rects c, r;
            unsigned input2 = alias >= 3 ? 0 : 1;
            unsigned output = alias % 3 == 0 ? 2 : alias % 3 - 1;
            memset(&c, 0xa5, sizeof(c));
            c.value[0] = a; c.value[1] = b;
            memcpy(&r, &c, sizeof(c));
            if (operation == 0) {
                unsigned code = oracle_BoxIntersect(&c.value[0], &c.value[input2], &c.value[output]);
                equal("BoxIntersect code", code,
                        BoxIntersect(&r.value[0], &r.value[input2], &r.value[output]));
                ++box_codes[code];
            } else {
                oracle_BoxUnion(&c.value[0], &c.value[input2], &c.value[output]);
                BoxUnion(&r.value[0], &r.value[input2], &r.value[output]);
            }
            bytes_equal("box inputs/output/canaries", &c, &r, sizeof(c));
        }
    }
}
static void test_boxes(void)
{
    static const int16_t edge[] = {INT16_MIN, -32767, -16384, -2, -1, 0, 1, 2, 16383, 32766, INT16_MAX};
    unsigned i, j, k;
    phase = "rectangles";
    for (i = 0; i < sizeof(edge) / sizeof(edge[0]); ++i)
        for (j = 0; j < sizeof(edge) / sizeof(edge[0]); ++j)
            for (k = 0; k < sizeof(edge) / sizeof(edge[0]); ++k) {
                RECT a = {{edge[i], edge[j]}, {edge[k], edge[(i + j) % 11]}};
                RECT b = {{edge[k], edge[i]}, {edge[j], edge[(j + k) % 11]}};
                ++input_number;
                box_case(a, b);
            }
    for (i = 0; i < 200000; ++i) {
        RECT a = {{random_input(), random_input()}, {random_input(), random_input()}};
        RECT b = {{random_input(), random_input()}, {random_input(), random_input()}};
        ++input_number;
        box_case(a, b);
    }
    equal("box rejection coverage", 1, box_codes[0] != 0);
    equal("box exact-match coverage", 1, box_codes[INTERSECT_NOCLIP | INTERSECT_ALL_SIDES] != 0);
}
struct guarded_line { uint32_t before[2]; BRESENHAM_LINE value; uint32_t after[2]; };
static void line_case(clip_rect rectangle, POINT first, POINT second)
{
    unsigned direction;
    for (direction = 0; direction < 2; ++direction) {
        struct guarded_line c, r;
        clip_rect original = rectangle;
        unsigned code;
        memset(&c, 0xa5, sizeof(c));
        c.value.first = direction ? second : first;
        c.value.second = direction ? first : second;
        memcpy(&r, &c, sizeof(c));
        code = oracle_clip_line(&rectangle, &c.value);
        equal("clip outcode", code, _clip_line(&rectangle, &r.value));
        ++line_codes[code];
        bytes_equal("clip fields/padding/canaries", &c, &r, sizeof(c));
        bytes_equal("clip rectangle unchanged", &original, &rectangle, sizeof(rectangle));
    }
}
static void test_lines(void)
{
    int x0, y0, x1, y1;
    unsigned i, j;
    static const clip_rect rectangles[] = {
        {{-3, -2}, {7, 5}}, {{0, 0}, {1, 1}}, {{0, 0}, {0, 0}},
        {{-5, -5}, {10, 10}}, {{-2, -1}, {-2, 5}}, {{1, 1}, {5, -5}},
        {{-32768, -32768}, {32767, 32767}}, {{32760, 32760}, {7, 7}},
        {{-16384, -16384}, {32767, 32767}}, {{-2, -2}, {4, 4}},
#ifdef UQM_TEST_HD
        {{INT32_MAX, INT32_MIN}, {INT32_MIN, INT32_MAX}},
        {{65533, -65538}, {65543, 65541}}, {{-65536, 65536}, {65537, 65537}},
#endif
    };
    phase = "line clipping";
    /* Exhaust all short vectors in both directions around several clips. */
    for (x0 = -6; x0 <= 6; ++x0) for (y0 = -6; y0 <= 6; ++y0)
        for (x1 = -6; x1 <= 6; ++x1) for (y1 = -6; y1 <= 6; ++y1)
            for (i = 0; i < 6; ++i) {
                POINT first = {x0, y0}, second = {x1, y1};
                ++input_number;
                line_case(rectangles[i], first, second);
            }
    for (i = 0; i < 250000; ++i) {
        POINT first = {random_input(), random_input()}, second;
        int dx = (int)(random_input() % 32767) - 16383;
        int dy = (int)(random_input() % 32767) - 16383;
        int endx = first.x + dx, endy = first.y + dy;
        if (endx < INT16_MIN || endx > INT16_MAX) endx = first.x - dx;
        if (endy < INT16_MIN || endy > INT16_MAX) endy = first.y - dy;
        second.x = endx; second.y = endy;
        ++input_number;
        line_case(rectangles[i % (sizeof(rectangles) / sizeof(rectangles[0]))], first, second);
        /* Small vectors through random rectangles give successful clips across
         * the signed coordinate range without overflowing doubled deltas. */
        {
            int originx = (int)(random_input() % 62000) - 31000;
            int originy = (int)(random_input() % 62000) - 31000;
            clip_rect rectangle = {{originx, originy}, {random_input() % 512 + 1, random_input() % 512 + 1}};
            first.x = originx + (int)(random_input() % 2048) - 768;
            first.y = originy + (int)(random_input() % 2048) - 768;
            second.x = originx + (int)(random_input() % 2048) - 768;
            second.y = originy + (int)(random_input() % 2048) - 768;
            line_case(rectangle, first, second);
        }
    }
    equal("line rejection coverage", 1, line_codes[0] != 0);
    equal("line no-clip coverage", 1, line_codes[INTERSECT_NOCLIP] != 0);
    for (j = 1; j <= INTERSECT_BOTTOM; j <<= 1) {
        unsigned count = 0;
        for (i = 1; i < 256; ++i) if (i & j) count += line_codes[i];
        equal("line side coverage", 1, count != 0);
    }
}
int main(void)
{
    test_queues();
    test_boxes();
    test_lines();
    printf("PASS queue/geometry: %" PRIu64 " comparisons; pointers=%zu long=%zu; %u allocation lifecycles\n",
            checks, sizeof(void *), sizeof(long), allocations);
    return 0;
}
