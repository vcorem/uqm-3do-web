/* Differential tests link Rust's public C ABI against the original C sources.
 * The oracle sources are compiled unchanged, using the real UQM headers.
 * SPDX-License-Identifier: GPL-2.0-or-later
 */
#include <inttypes.h>
#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define RANDOM2_INTERNAL
#include "libs/mathlib.h"
#include "uqm/supermelee/netplay/crc.h"

/* Declarations use the actual UQM ABI types, not independent typedef copies. */
DWORD oracle_TFB_Random(void);
DWORD oracle_TFB_SeedRandom(DWORD seed);
RandomContext *oracle_RandomContext_New(void);
void oracle_RandomContext_Delete(RandomContext *context);
RandomContext *oracle_RandomContext_Copy(const RandomContext *context);
DWORD oracle_RandomContext_Random(RandomContext *context);
DWORD oracle_RandomContext_SeedRandom(RandomContext *context, DWORD seed);
DWORD oracle_RandomContext_GetSeed(RandomContext *context);
COUNT oracle_square_root(DWORD value);
void oracle_crc_init(crc_State *state);
void oracle_crc_processBytes(crc_State *state, uint8 *buffer, size_t length);
void oracle_crc_processUint8(crc_State *state, uint8 value);
void oracle_crc_processUint16(crc_State *state, uint16 value);
void oracle_crc_processUint32(crc_State *state, uint32 value);
uint32 oracle_crc_finish(const crc_State *state);

#ifdef UQM_TEST_HD
RandomContext *oracle_RandomContext_Set(DWORD seed);
DWORD oracle_RandomContext_FastRandom(DWORD seed);
int oracle_RangeMinMax(int min, int max, DWORD random);
uint32_t oracle_crc32b(const char *text);
int oracle_hd_seed_type = OPTVAL_PRIME;
DWORD oracle_hd_sis_Seed = 0;

/* Rust imports the same runtime-selected multiplier as the production bridge. */
uint32_t uqm_hd_seed_a(void)
{
    return SeedA;
}
#endif

_Static_assert(sizeof(DWORD) == 4, "DWORD must be 32 bits");
_Static_assert(sizeof(COUNT) == 2, "square_root returns a 16-bit COUNT");
_Static_assert(sizeof(RandomContext) == 4, "RandomContext ABI size");
_Static_assert(_Alignof(RandomContext) == _Alignof(uint32), "RandomContext alignment");
_Static_assert(offsetof(RandomContext, seed) == 0, "RandomContext seed offset");
_Static_assert(sizeof(crc_State) == 4, "CRC state ABI size");
_Static_assert(_Alignof(crc_State) == _Alignof(uint32), "CRC state alignment");
_Static_assert(offsetof(crc_State, crc) == 0, "CRC member offset");

static uint64_t checks;
static uint64_t allocations;
static uint64_t releases;
static uint32_t corpus_state = UINT32_C(0x73e4a921);

/* The native precompiled Rust core may retain an unwind-personality reference
 * even though this no_std crate uses panic=abort. Unwinding is never permitted
 * through this C ABI; make any accidental invocation an immediate test failure. */
void rust_eh_personality(void)
{
    fputs("FAIL unexpected Rust unwind\n", stderr);
    abort();
}

static void fail(const char *operation, uint32_t input, uint32_t expected,
        uint32_t actual)
{
    fprintf(stderr, "FAIL %s at input 0x%08" PRIx32
            ": C expected 0x%08" PRIx32 ", Rust got 0x%08" PRIx32 "\n",
            operation, input, expected, actual);
    exit(1);
}

static void equal(const char *operation, uint32_t input, uint32_t expected,
        uint32_t actual)
{
    ++checks;
    if (expected != actual)
        fail(operation, input, expected, actual);
}

/* Independent reproducible input generator; this is not the game's RNG. */
static uint32_t next_input(void)
{
    corpus_state ^= corpus_state << 13;
    corpus_state ^= corpus_state >> 17;
    corpus_state ^= corpus_state << 5;
    return corpus_state;
}

/* Check the imported allocator ABI, allocation sizes, out-of-bounds writes,
 * ownership transfer, copies, double frees, and eventual release of contexts. */
struct allocation {
    unsigned char *base;
    unsigned char *payload;
    size_t size;
};
static struct allocation live[16];

void *HMalloc(size_t size)
{
    size_t slot;
    equal("HMalloc context size", 0, sizeof(RandomContext), (uint32_t)size);
    for (slot = 0; slot < sizeof(live) / sizeof(live[0]); ++slot) {
        if (live[slot].base == NULL) {
            unsigned char *base = malloc(size + 32);
            if (base == NULL)
                abort();
            memset(base, 0xa5, size + 32);
            live[slot] = (struct allocation){base, base + 16, size};
            ++allocations;
            return live[slot].payload;
        }
    }
    fputs("FAIL allocator has too many live contexts\n", stderr);
    exit(1);
}

void HFree(void *pointer)
{
    size_t slot;
    if (pointer == NULL)
        return;
    for (slot = 0; slot < sizeof(live) / sizeof(live[0]); ++slot) {
        if (live[slot].payload == pointer) {
            size_t i;
            for (i = 0; i < 16; ++i) {
                equal("allocation prefix canary", (uint32_t)i, 0xa5,
                        live[slot].base[i]);
                equal("allocation suffix canary", (uint32_t)i, 0xa5,
                        live[slot].payload[live[slot].size + i]);
            }
            memset(live[slot].payload, 0xdd, live[slot].size);
            free(live[slot].base);
            live[slot] = (struct allocation){0};
            ++releases;
            return;
        }
    }
    fputs("FAIL free of foreign or already released context\n", stderr);
    exit(1);
}

static void rng_sequence(uint32_t seed, size_t length)
{
    size_t i;
    equal("global seed previous value", seed, oracle_TFB_SeedRandom(seed),
            TFB_SeedRandom(seed));
    for (i = 0; i < length; ++i)
        equal("global random sequence", seed, oracle_TFB_Random(), TFB_Random());
}

static void test_global_rng(void)
{
    static const uint32_t edges[] = {
        0, 1, 2, 12344, 12345, 12346, 127772, 127773, 127774,
        2147483646u, 2147483647u, 2147483648u, 2147483649u,
        4294967293u, 4294967294u, 4294967295u
    };
    size_t i;
    /* First call observes the actual initial static seed in both languages. */
    equal("global default seed sequence", 12345, oracle_TFB_Random(), TFB_Random());
    for (i = 0; i < sizeof(edges) / sizeof(edges[0]); ++i)
        rng_sequence(edges[i], 65536);
    for (i = 0; i < 65536; ++i)
        rng_sequence((uint32_t)i, 32);
    for (i = 0; i < 8192; ++i)
        rng_sequence(next_input(), 256);
    printf("PASS global RNG: 16 edge seeds, all 65536 uint16 seeds, 8192 uint32 seeds\n");
}

static void test_context_rng(void)
{
    size_t iteration;
    static const uint32_t edges[] = {
        0, 1, 127773, 2147483646u, 2147483647u, 2147483648u,
        2147483649u, 4294967294u, 4294967295u
    };
    for (iteration = 0; iteration < 4096; ++iteration) {
        size_t i;
        uint32_t seed = iteration < sizeof(edges) / sizeof(edges[0])
                ? edges[iteration] : next_input();
        RandomContext *c = oracle_RandomContext_New();
        RandomContext *rust = RandomContext_New();
        RandomContext *c_copy;
        RandomContext *rust_copy;
        equal("new context default", seed, 12345, rust->seed);
        equal("context C layout seed", seed, c->seed, rust->seed);
        equal("context seed previous", seed,
                oracle_RandomContext_SeedRandom(c, seed),
                RandomContext_SeedRandom(rust, seed));
        equal("context normalized seed", seed, oracle_RandomContext_GetSeed(c),
                RandomContext_GetSeed(rust));
        for (i = 0; i < 128; ++i) {
            equal("context sequence", seed, oracle_RandomContext_Random(c),
                    RandomContext_Random(rust));
            equal("context GetSeed", seed, oracle_RandomContext_GetSeed(c),
                    RandomContext_GetSeed(rust));
            equal("context direct C layout read", seed, c->seed, rust->seed);
        }
        c_copy = oracle_RandomContext_Copy(c);
        rust_copy = RandomContext_Copy(rust);
        if (rust_copy == rust || c_copy == c)
            fail("context copy aliases source", seed, 0, 1);
        equal("copied context C layout", seed, c_copy->seed, rust_copy->seed);
        /* Mutating the source must leave its copy independent. */
        equal("source reseed previous", seed,
                oracle_RandomContext_SeedRandom(c, ~seed),
                RandomContext_SeedRandom(rust, ~seed));
        for (i = 0; i < 128; ++i) {
            equal("independent copied context", seed,
                    oracle_RandomContext_Random(c_copy),
                    RandomContext_Random(rust_copy));
            equal("independent reseeded source", seed,
                    oracle_RandomContext_Random(c), RandomContext_Random(rust));
        }
        /* Cross-language ownership must use the same imported allocator. */
        RandomContext_Delete(c_copy);
        oracle_RandomContext_Delete(rust_copy);
        RandomContext_Delete(rust);
        oracle_RandomContext_Delete(c);
    }
    equal("all contexts released", 0, (uint32_t)allocations, (uint32_t)releases);
    printf("PASS context RNG: 4096 independent lifecycle/copy cases, %" PRIu64
            " guarded allocations released\n", allocations);
}

static void check_square_root(uint32_t value)
{
    uint32_t result = square_root(value);
    equal("integer square root", value, oracle_square_root(value), result);
    ++checks;
    if ((uint64_t)result * result > value ||
            (uint64_t)(result + 1) * (result + 1) <= value)
        fail("sqrt mathematical floor property", value,
                oracle_square_root(value), result);
}

static void test_square_root(void)
{
    uint32_t i;
    for (i = 0; i <= 65535; ++i) {
        uint32_t square = i * i;
        check_square_root(i);
        check_square_root(square);
        if (square != 0)
            check_square_root(square - 1);
        check_square_root(square + 1);
    }
    check_square_root(UINT32_MAX);
    check_square_root(UINT32_MAX - 1);
    for (i = 0; i < 1000000; ++i)
        check_square_root(next_input());
    puts("PASS sqrt: all uint16 inputs, every uint16 square boundary, 1000000 uint32 inputs");
}

static void test_crc_scalars(void)
{
    uint32_t i;
    crc_State c, rust;
    oracle_crc_init(&c);
    crc_init(&rust);
    equal("CRC initialization", 0, c.crc, rust.crc);
    equal("CRC empty result", 0, 0, crc_finish(&rust));
    for (i = 0; i < 1000000; ++i) {
        uint32_t value = next_input();
        if (i % 257 == 0)
            c.crc = rust.crc = next_input();
        oracle_crc_processUint8(&c, (uint8)i);
        crc_processUint8(&rust, (uint8)i);
        equal("CRC uint8 state", i, c.crc, rust.crc);
        oracle_crc_processUint16(&c, (uint16)i);
        crc_processUint16(&rust, (uint16)i);
        equal("CRC uint16 state", i, c.crc, rust.crc);
        oracle_crc_processUint32(&c, value);
        crc_processUint32(&rust, value);
        equal("CRC uint32 state", value, c.crc, rust.crc);
        equal("CRC finish", value, oracle_crc_finish(&c), crc_finish(&rust));
    }
    puts("PASS CRC scalars: 1000000 mixed sequences, all uint8/uint16 values, arbitrary states");
}

static void crc_reference_bytes(crc_State *state, const uint8 *bytes, size_t length)
{
    size_t i;
    /* Original crc_processBytes never advances its pointer for nonempty input.
     * Do not hang the suite or modify the reference. Its real scalar-byte
     * implementation provides the independent oracle for the corrected API. */
    for (i = 0; i < length; ++i)
        oracle_crc_processUint8(state, bytes[i]);
}

static void test_crc_bytes(void)
{
    static const struct {
        const char *bytes;
        uint32_t expected;
    } vectors[] = {
        {"", 0x00000000u},
        {"123456789", 0xcbf43926u},
        {"The quick brown fox jumps over the lazy dog", 0x414fa339u}
    };
    static const size_t lengths[] = {0, 1, 2, 3, 4, 7, 8, 15, 16, 63, 64,
            255, 256, 1023, 1024, 4095, 4096, 65535, 65536};
    uint8 *buffer = malloc(65536);
    size_t i, j;
    crc_State c, rust;
    if (buffer == NULL)
        abort();
    for (i = 0; i < sizeof(vectors) / sizeof(vectors[0]); ++i) {
        crc_init(&rust);
        crc_processBytes(&rust, (uint8 *)vectors[i].bytes, strlen(vectors[i].bytes));
        equal("CRC32 known vector", (uint32_t)i, vectors[i].expected, crc_finish(&rust));
    }
    crc_init(&rust);
    crc_processBytes(&rust, NULL, 0);
    equal("CRC null empty buffer", 0, 0, crc_finish(&rust));
    /* Zero-length is the only terminating case of the original bytes function. */
    oracle_crc_init(&c);
    oracle_crc_processBytes(&c, buffer, 0);
    equal("CRC original empty buffer", 0, oracle_crc_finish(&c), crc_finish(&rust));
    for (i = 0; i < 65536; ++i)
        buffer[i] = (uint8)next_input();
    for (i = 0; i < 128; ++i) {
        for (j = 0; j < sizeof(lengths) / sizeof(lengths[0]); ++j) {
            size_t length = lengths[j];
            uint32_t state = next_input();
            crc_State split = {state};
            size_t at;
            c.crc = rust.crc = state;
            crc_reference_bytes(&c, buffer, length);
            crc_processBytes(&rust, buffer, length);
            equal("CRC bytes versus original scalar oracle", (uint32_t)length,
                    c.crc, rust.crc);
            for (at = 0; at < length;) {
                size_t chunk = (next_input() % 257) + 1;
                if (chunk > length - at)
                    chunk = length - at;
                crc_processBytes(&split, buffer + at, chunk);
                at += chunk;
            }
            equal("CRC streaming chunk independence", (uint32_t)length,
                    c.crc, split.crc);
        }
    }
    /* An intentionally unaligned buffer exercises the byte-pointer ABI. */
    c.crc = rust.crc = UINT32_MAX;
    crc_reference_bytes(&c, buffer + 1, 65535);
    crc_processBytes(&rust, buffer + 1, 65535);
    equal("CRC unaligned buffer", 65535, c.crc, rust.crc);
    free(buffer);
    puts("PASS CRC bytes: 3 known vectors, empty/null, 19 lengths, streaming and unaligned buffers");
}

#ifdef UQM_TEST_HD
static void test_hd_rng(void)
{
    static const uint32_t multipliers[] = {
        0, 1, 2, 3, 4, 5, 11, 16807, 32767, 1000003, 1073741823,
        2147483644u, 2147483645u, 2147483647u, 4294967295u
    };
    static const uint32_t raw_seeds[] = {
        0, 1, 2, 12345, 2147483646u, 2147483647u, 2147483648u, 4294967295u
    };
    int mode;
    size_t m, r, i;
    for (mode = 0; mode < 4; ++mode) {
        oracle_hd_seed_type = mode;
        for (m = 0; m < sizeof(multipliers) / sizeof(multipliers[0]); ++m) {
            oracle_hd_sis_Seed = multipliers[m];
            /* Retain the previous global RNG state while its multiplier changes. */
            for (i = 0; i < 4096; ++i)
                equal("HD runtime multiplier switch", multipliers[m],
                        oracle_TFB_Random(), TFB_Random());
            for (r = 0; r < sizeof(raw_seeds) / sizeof(raw_seeds[0]); ++r) {
                uint32_t seed = raw_seeds[r];
                RandomContext *c = oracle_RandomContext_Set(seed);
                RandomContext *rust = RandomContext_Set(seed);
                equal("HD Set preserves raw seed", seed, seed, rust->seed);
                for (i = 0; i < 256; ++i) {
                    uint32_t input = i == 0 ? seed : next_input();
                    equal("HD raw context sequence", seed,
                            oracle_RandomContext_Random(c), RandomContext_Random(rust));
                    equal("HD fast stateless random", input,
                            oracle_RandomContext_FastRandom(input),
                            RandomContext_FastRandom(input));
                }
                RandomContext_Delete(c);
                oracle_RandomContext_Delete(rust);
            }
        }
    }
    oracle_hd_seed_type = OPTVAL_PRIME;
    equal("HD contexts released", 0, (uint32_t)allocations, (uint32_t)releases);
    puts("PASS HD RNG: 4 modes, 15 multiplier boundaries, raw Set/FastRandom and runtime switches");
}

static void test_hd_ranges(void)
{
    static const int bounds[][2] = {
        {0, 0}, {0, 1}, {1, 1}, {-1, -1}, {-100, 100}, {-100, -2},
        {-2147483647 - 1, -2147483647 - 1}, {-2147483647, -1},
        {0, 2147483646}, {100, 0}, {2147483647, -2147483647 - 1}
    };
    size_t i, j;
    for (i = 0; i < sizeof(bounds) / sizeof(bounds[0]); ++i) {
        for (j = 0; j < 1024; ++j) {
            uint32_t random = j == 0 ? 0 : j == 1 ? UINT32_MAX : next_input();
            equal("HD RangeMinMax edge", random,
                    (uint32_t)oracle_RangeMinMax(bounds[i][0], bounds[i][1], random),
                    (uint32_t)RangeMinMax(bounds[i][0], bounds[i][1], random));
        }
    }
    for (i = 0; i < 100000; ++i) {
        int min = (int)(next_input() % 1048576) - 524288;
        int max = min + (int)(next_input() % 524288);
        uint32_t random = next_input();
        equal("HD RangeMinMax random", random,
                (uint32_t)oracle_RangeMinMax(min, max, random),
                (uint32_t)RangeMinMax(min, max, random));
    }
    /* The C code overflows for INT_MAX or a range wider than INT_MAX.
     * Those undefined C cases are intentionally excluded from the oracle. */
    puts("PASS HD inclusive ranges: 11 defined edge ranges and 100000 random cases");
}

static void test_hd_text_crc(void)
{
    char text[1025];
    size_t i, j;
    equal("HD text CRC empty", 0, 0, crc32b(""));
    equal("HD text CRC known vector", 9, 0xcbf43926u, crc32b("123456789"));
    for (i = 1; i < 256; ++i) {
        text[0] = (char)i;
        text[1] = '\0';
        equal("HD text CRC char signedness", (uint32_t)i,
                oracle_crc32b(text), crc32b(text));
    }
    for (i = 0; i < 2048; ++i) {
        size_t length = next_input() % 1025;
        for (j = 0; j < length; ++j)
            text[j] = (char)((next_input() % 255) + 1);
        text[length] = '\0';
        equal("HD text CRC byte string", (uint32_t)length,
                oracle_crc32b(text), crc32b(text));
        if (length > 1) {
            text[length / 2] = '\0';
            equal("HD text CRC first nul termination", (uint32_t)length,
                    oracle_crc32b(text), crc32b(text));
        }
    }
    puts("PASS HD text CRC: known vectors, all nonzero bytes and 2048 signed-char strings");
}
#endif

int main(void)
{
    printf("ABI: DWORD=%zu COUNT=%zu RandomContext=%zu/%zu crc_State=%zu/%zu\n",
            sizeof(DWORD), sizeof(COUNT), sizeof(RandomContext),
            _Alignof(RandomContext), sizeof(crc_State), _Alignof(crc_State));
    test_global_rng();
    test_context_rng();
    test_square_root();
    test_crc_scalars();
    test_crc_bytes();
#ifdef UQM_TEST_HD
    test_hd_rng();
    test_hd_ranges();
    test_hd_text_crc();
#endif
    printf("PASS Rust/C differential oracle: %" PRIu64 " comparisons\n", checks);
    return 0;
}
