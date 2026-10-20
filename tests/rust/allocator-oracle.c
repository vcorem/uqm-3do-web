/* Exercise the actual Rust GlobalAlloc against Emscripten's shared C heap. */
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
extern int rust_allocator_case(size_t alignment, size_t size);
int main(void) {
    const size_t alignments[] = {1, 2, 4, 8, 16, 32, 64, 128, 256, 4096};
    const size_t sizes[] = {1, 3, 7, 8, 9, 15, 16, 17, 31, 32, 33, 65, 127, 513, 4097};
    size_t cases = 0;
    for (unsigned round = 0; round < 64; ++round) {
        void *fragment[23];
        for (unsigned i = 0; i < 23; ++i) {
            size_t size = 1 + (round * 19 + i * 9) % 257;
            fragment[i] = malloc(size);
            if (!fragment[i]) return 20;
            memset(fragment[i], 0xa5, size);
        }
        for (unsigned i = 0; i < 23; i += 2) { free(fragment[i]); fragment[i] = NULL; }
        for (unsigned a = 0; a < sizeof alignments / sizeof *alignments; ++a) {
            for (unsigned s = 0; s < sizeof sizes / sizeof *sizes; ++s) {
                int status = rust_allocator_case(alignments[a], sizes[s]);
                if (status) {
                    fprintf(stderr, "allocator failure=%d align=%zu size=%zu round=%u\n", status, alignments[a], sizes[s], round);
                    return status;
                }
                ++cases;
            }
        }
        for (unsigned i = 0; i < 23; ++i) free(fragment[i]);
    }
    printf("Emscripten allocator: %zu allocation/zero/grow/shrink cases passed\n", cases);
    return 0;
}
