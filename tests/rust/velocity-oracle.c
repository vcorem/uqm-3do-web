/* Actual-source differential oracle for UQM fixed-point movement and angles.
 * SPDX-License-Identifier: GPL-2.0-or-later */
#include <inttypes.h>
#include <limits.h>
#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "uqm/units.h"
#include "uqm/velocity.h"

#ifdef UQM_TEST_HD
typedef SDWORD input_t;
unsigned int resolutionFactor = 2;
extern SDWORD oracle_sinetab[];
void oracle_GetCurrentVelocityComponentsSdword(VELOCITY_DESC *, SDWORD *, SDWORD *);
void oracle_GetNextVelocityComponentsSdword(VELOCITY_DESC *, SDWORD *, SDWORD *, DWORD);
uint32_t uqm_hd_seed_a(void) { return 16807; }
#else
typedef SIZE input_t;
extern SIZE oracle_sinetab[];
#endif
COUNT oracle_ARCTAN(input_t x, input_t y);
void oracle_GetCurrentVelocityComponents(VELOCITY_DESC *, SIZE *, SIZE *);
void oracle_GetNextVelocityComponents(VELOCITY_DESC *, SIZE *, SIZE *, COUNT);
void oracle_SetVelocityVector(VELOCITY_DESC *, input_t, COUNT);
void oracle_SetVelocityComponents(VELOCITY_DESC *, input_t, input_t);
void oracle_DeltaVelocityComponents(VELOCITY_DESC *, input_t, input_t);

_Static_assert(sizeof(VELOCITY_DESC) == 18, "velocity size");
_Static_assert(_Alignof(VELOCITY_DESC) == 2, "velocity alignment");
_Static_assert(sizeof(EXTENT) == 4, "extent size");
_Static_assert(offsetof(VELOCITY_DESC, TravelAngle) == 0, "angle offset");
_Static_assert(offsetof(VELOCITY_DESC, vector) == 2, "vector offset");
_Static_assert(offsetof(VELOCITY_DESC, fract) == 6, "fraction offset");
_Static_assert(offsetof(VELOCITY_DESC, error) == 10, "error offset");
_Static_assert(offsetof(VELOCITY_DESC, incr) == 14, "increment offset");
_Static_assert(offsetof(EXTENT, width) == 0, "width offset");
_Static_assert(offsetof(EXTENT, height) == 2, "height offset");

void *HMalloc(size_t size) { return malloc(size); }
void HFree(void *pointer) { free(pointer); }
void rust_eh_personality(void) { abort(); }

static uint64_t checks;
static uint32_t corpus = UINT32_C(0x917bc425);
static uint32_t random_input(void)
{
    corpus ^= corpus << 13;
    corpus ^= corpus >> 17;
    corpus ^= corpus << 5;
    return corpus;
}

static void equal(const char *label, uint32_t input, uint32_t expected, uint32_t actual)
{
    ++checks;
    if (expected != actual) {
        fprintf(stderr, "FAIL %s input=0x%08" PRIx32
                " C=0x%08" PRIx32 " Rust=0x%08" PRIx32 "\n",
                label, input, expected, actual);
        exit(1);
    }
}

struct guarded_velocity {
    uint16_t before[4];
    VELOCITY_DESC value;
    uint16_t after[4];
};

static void reset_guard(struct guarded_velocity *state)
{
    memset(state, 0xa5, sizeof(*state));
}

static void equal_state(const char *label, uint32_t input,
        const VELOCITY_DESC *c, const struct guarded_velocity *rust)
{
    size_t i;
    uint16_t c_words[9], rust_words[9];
    memcpy(c_words, c, sizeof(c_words));
    memcpy(rust_words, &rust->value, sizeof(rust_words));
    for (i = 0; i < 9; ++i)
        equal(label, input, c_words[i], rust_words[i]);
    for (i = 0; i < 4; ++i) {
        equal("velocity prefix canary", input, 0xa5a5, rust->before[i]);
        equal("velocity suffix canary", input, 0xa5a5, rust->after[i]);
    }
}

static int valid_angle_pair(input_t x, input_t y)
{
#ifdef UQM_TEST_HD
    const input_t minimum = INT32_MIN;
    return x != minimum && y != minimum;
#else
    const input_t minimum = INT16_MIN;
    /* abs(MIN) is not representable in the C local type. The mixed cases can
     * index outside atantab or divide by zero. Both MIN values are the one
     * reproducible wrapped case, and are checked separately. */
    return (x != minimum && y != minimum) || (x == minimum && y == minimum);
#endif
}

static void test_angles(void)
{
    int32_t x, y;
    size_t i;
    for (i = 0; i < FULL_CIRCLE; ++i)
        equal("sine table exact integer entry", (uint32_t)i,
                (uint32_t)oracle_sinetab[i], (uint32_t)sinetab[i]);
    for (x = -256; x <= 256; ++x)
        for (y = -256; y <= 256; ++y)
            equal("angle exhaustive small vectors", (uint32_t)x,
                    oracle_ARCTAN((input_t)x, (input_t)y), ARCTAN((input_t)x, (input_t)y));
    for (x = -32767; x <= 32767; ++x) {
        static const input_t axis[] = {-32767, -1, 0, 1, 32767};
        for (i = 0; i < sizeof(axis) / sizeof(axis[0]); ++i)
            equal("angle full signed16 range/axis", (uint32_t)x,
                    oracle_ARCTAN((input_t)x, axis[i]), ARCTAN((input_t)x, axis[i]));
    }
    for (i = 0; i < 1000000; ++i) {
        input_t a = (input_t)random_input(), b = (input_t)random_input();
        if (valid_angle_pair(a, b))
            equal("angle random target-width vectors", (uint32_t)a,
                    oracle_ARCTAN(a, b), ARCTAN(a, b));
    }
#ifndef UQM_TEST_HD
    equal("angle both signed16 minimum", 0, oracle_ARCTAN(INT16_MIN, INT16_MIN),
            ARCTAN(INT16_MIN, INT16_MIN));
#endif
    puts("PASS trig: exact sine table, exhaustive small vectors, signed16 axes, 1000000 vectors");
}

static void current(VELOCITY_DESC *c, struct guarded_velocity *rust, uint32_t key)
{
    SIZE cx = 0, cy = 0, rx = 0, ry = 0;
    oracle_GetCurrentVelocityComponents(c, &cx, &cy);
    GetCurrentVelocityComponents(&rust->value, &rx, &ry);
    equal("current velocity x", key, (uint16_t)cx, (uint16_t)rx);
    equal("current velocity y", key, (uint16_t)cy, (uint16_t)ry);
    equal_state("current velocity preserves state", key, c, rust);
#ifdef UQM_TEST_HD
    {
        SDWORD cx32 = 0, cy32 = 0, rx32 = 0, ry32 = 0;
        oracle_GetCurrentVelocityComponentsSdword(c, &cx32, &cy32);
        GetCurrentVelocityComponentsSdword(&rust->value, &rx32, &ry32);
        equal("HD current signed32 x", key, (uint32_t)cx32, (uint32_t)rx32);
        equal("HD current signed32 y", key, (uint32_t)cy32, (uint32_t)ry32);
        equal_state("HD current preserves state", key, c, rust);
    }
#endif
}

static void next(VELOCITY_DESC *c, struct guarded_velocity *rust,
        COUNT frames, uint32_t key)
{
    SIZE cx = 0, cy = 0, rx = 0, ry = 0;
    oracle_GetNextVelocityComponents(c, &cx, &cy, frames);
    GetNextVelocityComponents(&rust->value, &rx, &ry, frames);
    equal("next velocity x", key, (uint16_t)cx, (uint16_t)rx);
    equal("next velocity y", key, (uint16_t)cy, (uint16_t)ry);
    equal_state("next velocity state", key, c, rust);
}

static void vector_case(input_t magnitude, COUNT facing)
{
    VELOCITY_DESC c;
    struct guarded_velocity rust;
    memset(&c, 0xa5, sizeof(c));
    reset_guard(&rust);
    oracle_SetVelocityVector(&c, magnitude, facing);
    SetVelocityVector(&rust.value, magnitude, facing);
    equal_state("set vector state", (uint32_t)magnitude, &c, &rust);
    current(&c, &rust, facing);
    next(&c, &rust, (COUNT)(facing ^ 0xffff), facing);
    next(&c, &rust, 1, facing);
}

static void test_vectors(void)
{
    int32_t magnitude;
    uint32_t facing;
    const int32_t first = -32768, last = 32767;
    for (magnitude = first; magnitude <= last; ++magnitude)
        for (facing = 0; facing < 16; ++facing)
            vector_case((input_t)magnitude, (COUNT)facing);
    for (facing = 0; facing <= 65535; ++facing)
        vector_case((input_t)((int32_t)(facing % 8191) - 4095), (COUNT)facing);
#ifdef UQM_TEST_HD
    {
        static const int32_t edges[] = {INT32_MIN, INT32_MIN + 1, -134217729,
                -134217728, -67108865, -67108864, -65536, -32769,
                32768, 65535, 65536, 67108863, 67108864, 134217727, INT32_MAX};
        size_t i;
        for (i = 0; i < sizeof(edges) / sizeof(edges[0]); ++i)
            for (facing = 0; facing < 16; ++facing)
                vector_case(edges[i], (COUNT)facing);
    }
#endif
    puts("PASS velocity vectors: complete magnitude/facing grids and all 65536 facing inputs");
}

static void component_case(input_t dx, input_t dy)
{
    static const COUNT durations[] = {0, 1, 2, 3, 31, 32, 255, 256, 32767, 65535};
    VELOCITY_DESC c;
    struct guarded_velocity rust;
    size_t i;
    if (!valid_angle_pair(dx, dy))
        return;
    memset(&c, 0xa5, sizeof(c));
    reset_guard(&rust);
    oracle_SetVelocityComponents(&c, dx, dy);
    SetVelocityComponents(&rust.value, dx, dy);
    equal_state("set components state", (uint32_t)dx, &c, &rust);
    current(&c, &rust, (uint32_t)dy);
    for (i = 0; i < sizeof(durations) / sizeof(durations[0]); ++i)
        next(&c, &rust, durations[i], (uint32_t)dx);
    for (i = 0; i < 8; ++i) {
        input_t delta_x = (input_t)((int32_t)(random_input() % 2049) - 1024);
        input_t delta_y = (input_t)((int32_t)(random_input() % 2049) - 1024);
        input_t result_x, result_y;
#ifdef UQM_TEST_HD
        SDWORD cx, cy;
        oracle_GetCurrentVelocityComponentsSdword(&c, &cx, &cy);
        result_x = (input_t)((uint32_t)cx + (uint32_t)delta_x);
        result_y = (input_t)((uint32_t)cy + (uint32_t)delta_y);
#else
        SIZE cx, cy;
        oracle_GetCurrentVelocityComponents(&c, &cx, &cy);
        result_x = (input_t)((uint16_t)cx + (uint16_t)delta_x);
        result_y = (input_t)((uint16_t)cy + (uint16_t)delta_y);
#endif
        if (!valid_angle_pair(result_x, result_y))
            continue;
        oracle_DeltaVelocityComponents(&c, delta_x, delta_y);
        DeltaVelocityComponents(&rust.value, delta_x, delta_y);
        equal_state("delta velocity state", (uint32_t)delta_x, &c, &rust);
        next(&c, &rust, (COUNT)i, (uint32_t)delta_y);
    }
}

static void test_components(void)
{
    int32_t x;
    size_t i;
    component_case(0, 0);
    component_case(0, 1);
    component_case(1, 0);
    component_case(-1, 0);
    component_case(0, -1);
    for (x = -32767; x <= 32767; ++x)
        component_case((input_t)x, (input_t)(32767 - (x + 32767)));
    for (i = 0; i < 10000; ++i)
        component_case((input_t)random_input(), (input_t)random_input());
    puts("PASS velocity components: signed16 range, zero/cardinal, 10000 random, frame/delta sequences");
}

static void test_raw_states(void)
{
    uint32_t i;
    for (i = 0; i < 65536; ++i) {
        VELOCITY_DESC c;
        struct guarded_velocity rust;
        uint16_t words[9];
        size_t j;
        for (j = 0; j < 9; ++j)
            words[j] = (uint16_t)random_input();
        memcpy(&c, words, sizeof(c));
        reset_guard(&rust);
        rust.value = c;
        current(&c, &rust, i);
        next(&c, &rust, (COUNT)i, i);
        next(&c, &rust, (COUNT)(65535 - i), i);
#ifdef UQM_TEST_HD
        {
            static const DWORD durations[] = {0, 1, 65535, 65536,
                    2147483647u, 2147483648u, 4294967295u};
            SDWORD cx, cy, rx, ry;
            DWORD frames = durations[i % (sizeof(durations) / sizeof(durations[0]))];
            oracle_GetNextVelocityComponentsSdword(&c, &cx, &cy, frames);
            GetNextVelocityComponentsSdword(&rust.value, &rx, &ry, frames);
            equal("HD next signed32 x", frames, (uint32_t)cx, (uint32_t)rx);
            equal("HD next signed32 y", frames, (uint32_t)cy, (uint32_t)ry);
            equal_state("HD signed32 next state", frames, &c, &rust);
        }
#endif
    }
    puts("PASS raw velocity ABI: 65536 states/durations, signed narrowing, error carry and canaries");
}

int main(void)
{
    printf("ABI: velocity=%zu align=%zu extent=%zu input=%zu long=%zu\n",
            sizeof(VELOCITY_DESC), _Alignof(VELOCITY_DESC), sizeof(EXTENT),
            sizeof(input_t), sizeof(long));
    test_angles();
    test_vectors();
    test_components();
    test_raw_states();
    printf("PASS velocity/trig differential oracle: %" PRIu64 " comparisons\n", checks);
    return 0;
}
