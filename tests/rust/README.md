# Rust differential and boundary tests

Run `./scripts/test-rust-core.sh --all` from the repository. Use `--native` (the
default) or `--wasm` to run one target. Both targets exercise debug and release
Rust static libraries through the original C ABI. The pinned Rust toolchain and
Emscripten version are required; the script does not install either toolchain.
The Wasm toolchain pin is in `.rust-wasm-toolchain` and requires its `rust-src`
component: both test and production builds use `-Zbuild-std=core,compiler_builtins`
to rebuild Rust's core libraries with the required atomics/bulk-memory features.
Run `./scripts/test-rust-core.sh --all --hd` for the separate HD feature and its
pinned upstream source checkout at `.tools/hd-source` (or `HD_SOURCE_DIR`).
Run these commands sequentially, as CI does. Original and HD feature builds
share the default Cargo target directory: Cargo's build lock ends before the
external C linker reads the archive. Parallel edition runs must set separate
`CARGO_TARGET_DIR` paths to prevent one feature build replacing the other's
library between build and link.

The oracle compiles these **unchanged original source files**, renaming only
their exported symbols with the preprocessor:

- `sc2/src/libs/math/random.c`
- `sc2/src/libs/math/random2.c`
- `sc2/src/libs/math/sqrt.c`
- `sc2/src/uqm/supermelee/netplay/crc.c`

`core-oracle.c` uses the real UQM public headers for every call. Its only header
fixture supplies `HAVE__BOOL` to the generated Unix build configuration; it does
not replace types, function declarations, constants, or algorithms. Compile-time
assertions verify C size, alignment and member offsets; runtime field accesses
and guarded context allocations verify that Rust honors those definitions.
Native C sources and the harness run with AddressSanitizer and UndefinedBehavior
Sanitizer. Rust itself is not sanitizer-instrumented by this script. Both Wasm
profiles use production's pthread/shared-memory flags. The test-only native
unwind-personality symbol aborts immediately if ever invoked; Rust is built
with `panic=abort`.
Unexercised renderer/logger imports that share a release staticlib codegen unit
are resolved by `unused-engine-imports.c`; these stubs abort if called. The
queue and renderer oracles supply tracked implementations for their own imports.

Coverage includes reproducible long RNG sequences and seed coercion at `0`,
`2^31-1`, `2^31`, and `2^32-1`; every 16-bit seed; context copying, independent
mutation, cross-language allocation ownership, and release; every 16-bit square
root input and square boundary, plus one million 32-bit inputs; all 8/16-bit CRC
values, one million mixed scalar CRC sequences, arbitrary CRC starting states,
known CRC-32 vectors, streaming, unaligned buffers, and empty input.

The old C `crc_processBytes` function does not advance its input pointer, so it
never returns for nonempty input. Tests deliberately never invoke that case.
The corrected Rust bytes API is instead compared with the original C
`crc_processUint8` called for each byte, as well as independent published CRC-32
known vectors. The original empty-buffer case is still exercised directly.

The HD suite compiles the corresponding unchanged MegaMod C sources, adds its
runtime multiplier, raw context constructor, stateless RNG, inclusive range,
and string-CRC functions, and retains the real HD RNG header and macros.
`hd-environment.h` replaces only two unrelated game-state inputs and their broad
engine header dependencies. It supplies the current seed mode and SIS seed;
the upstream `SeedA`, `SeedQ`, `SeedR`, boundary checks, types and algorithms
remain the oracle. Cases cover all four modes, 15 valid/invalid multiplier
boundaries, runtime mode changes, signed-character text hashing, and defined
integer ranges. The old C range function has signed-overflow undefined behavior
for `INT_MAX` and very wide ranges; those cases are intentionally not used as a
reference for Rust's defined widened-arithmetic extension.

Artifacts are isolated under ignored `.tools/rust-core-oracle/`. These core
tests prove equivalence only for the modules listed above. The whole-engine
Rust port is validated separately by source/export audits, codec and Lua
boundary tests, actual-translator regressions, and real-engine browser tests.

## Movement and angle oracle

Run `tests/rust/test-velocity.sh --all` and
`tests/rust/test-velocity.sh --all --hd`. The optional `--native` and `--wasm`
modes work like the core script. This independent executable compiles the real
`uqm/trans.c` and `uqm/velocity.c` into renamed oracle symbols, then calls Rust
through the real `units.h`, `velocity.h` and `gfxlib.h` declarations. The fixture
only declares host capabilities and the unrelated HD resolution option; no
movement types, table entries, angle constants or arithmetic macros are copied.

The suite verifies the exact 18-byte, 2-byte-aligned velocity descriptor and all
member offsets. It compares the full sine table, an exhaustive small-vector
angle grid, signed 16-bit axes, and one million reproducible angle inputs. It
tests every signed 16-bit magnitude with all 16 facings, all 65,536 raw facing
inputs, HD signed 32-bit magnitude boundaries, component initialization,
successive acceleration and frame steps, every 16-bit frame duration, and 65,536
arbitrary descriptor states. HD also exercises 32-bit outputs and frame counts
through `UINT32_MAX`. Every descriptor comparison includes memory canaries.

The original C movement code deliberately shifts negative coordinates. Its
oracle is compiled with `-fwrapv` for signed arithmetic and disables the shift
portion of UBSan (`-fno-sanitize=shift`); AddressSanitizer and other UBSan checks
remain enabled in native runs. The actual Emscripten debug/release executables
verify the production 32-bit arithmetic separately. Native `long` is 64-bit on
macOS while Wasm `long` is 32-bit; Rust preserves each target's C width.

The C angle routine cannot safely negate a single minimum signed input; the
original 16-bit variant can subsequently index outside its table or divide by
zero, and the HD 32-bit negation overflows. These unsafe cases are excluded from
C equivalence claims. The original variant's reproducible pair of minimum
16-bit inputs is explicitly checked. Rust's defined behavior for the remaining
formerly undefined inputs is covered by its unit tests, not treated as a valid
legacy C oracle.

Movement artifacts are isolated under `.tools/rust-velocity-oracle/`.

## Queue and geometry oracle

Run `tests/rust/test-queue-geometry.sh --all` and
`tests/rust/test-queue-geometry.sh --all --hd`. The same `--native`/`--wasm`
selectors and debug/release matrix apply. This suite compiles the unchanged
`uqm/displist.c`, `libs/graphics/boxint.c` and `libs/graphics/clipline.c`, with
renamed exports. It includes their actual queue and graphics public headers;
the shared environment fixture supplies only host capabilities and the HD
resolution option. No queue, rectangle or line declarations are copied.

Compile-time checks verify every queue and clipping field offset, pointer-width
dependent queue layouts, the four-byte `BOOLEAN` enum, and the 24-byte
`BRESENHAM_LINE`. Runtime checks compare pool pointers by slot index, payloads,
links, guarded allocations and outputs. Every capacity from 1 through 255 is
tested with two slot sizes, plus a near-maximum 16-bit slot size. Traces exhaust
the pool, insert/remove at different positions, allocate/free/reuse slots, reset
the queue, preserve stale fields, iterate callbacks, remove/free the current
slot from a callback, and inject allocation failures. Exhaustion diagnostics
and allocation ownership are checked as well.

Rectangle tests combine signed 16-bit boundaries with 200,000 reproducible
input pairs. Six input/output alias layouts exercise each union and
intersection, including identical inputs and output overwriting either input.
The line corpus exhausts a short-vector grid around six clip rectangles and
adds 500,000 reproducible large-coordinate and local-intersection cases, each
in both endpoint orders. It compares every state byte and guard, including
the original partial mutations on rejection, all side flags, rounding and
error terms. HD adds extreme 32-bit rectangles and their legacy 16-bit internal
narrowing. Pointer size and C `long` width are checked on both native and Wasm.

Native C uses ASan/UBSan with `-fwrapv` and the shift-check exception already
described for movement. Queue capacity zero or above 255 and invalid slot
layouts are excluded because the original C writes outside its allocation.
The line corpus limits endpoint differences to 16,383 so doubled signed
16-bit deltas do not overflow; it does not treat overflow-induced division
failures as defined reference behavior. This does not constrain production
Rust inputs. Queue/geometry artifacts are isolated under
`.tools/rust-queue-geometry-oracle/`.

## PNG decoding and HD writing

Run `npm run test:rust:png` after the pinned HD source is available. This checks
the production Rust PNG module and its C libpng error boundary against both
unchanged C decoders and the HD writer in debug and optimized shared-memory
Wasm. Native ASan/UBSan checks are available through
`tests/rust/test-png.sh --native`. See [PNG oracle details](PNG.md) for native
dependencies, fixture coverage and the documented corrections to unsafe legacy
palette-alpha and I/O behavior.

## Renderer bounding-box oracle

Run `tests/rust/test-bbox.sh --all` and
`tests/rust/test-bbox.sh --all --hd`. This compiles unchanged `bbox.c` and its
actual `bbox.h`/graphics types, renaming only the C oracle's globals and exports.
The opaque canvas fixture supplies only `TFB_DrawCanvas_GetExtent`; the dirty
region algorithm is never replaced by a fixture. Both editions use identical
source, but each is checked with its own real headers and Rust feature set.

Each of the eight native/shared-Wasm debug/release runs compares 17,700,011
fields and callback counts. The corpus includes 100,000 reproducible operation
traces, every pair from 13 integer boundary values, arbitrary signed rectangle
fields, reset/reinitialization, null clips, clipping outside the screen, canvas
extent calls, and input pointers aliasing both global rectangles. It checks
partial writes and global dimensions after each step and verifies every ABI
offset. Native C has ASan/UBSan enabled; `-fwrapv` defines extreme signed integer
dimensions consistently with Wasm, rather than claiming these overflow cases
were portable defined C behavior. Outputs are under `.tools/rust-bbox-oracle/`.

## Lua error boundaries

Run `npm run test:rust:lua` after fetching the pinned HD source. This tests the
production C guard and shared Rust propagation macro with the actual unchanged
Lua 5.2 runtime in debug and optimized shared-memory Wasm.
`tests/rust/test-lua.sh --all` also checks native debug/release with ASan/UBSan.
Real Rust callback destructors must run before Lua catches an error. The oracle
compares native C callbacks, catches bad arguments/metamethod/parser/reader
errors, injects allocation failures, checks nested callbacks and upvalues, and
reuses each state after failure. See [Lua oracle details](LUA.md).

## HD network packet and transport oracle

Run `bash tests/rust/test-hd-netplay.sh --all` after the HD source is configured.
This imports the actual generated HD receive/send/packet-handler Rust modules
and compares them with independently compiled, renamed C implementations in
the same Wasm process. Each of four debug/release × direct/pthread modes checks
12,210 scenarios and 13 C/Rust ABI properties. Cases include packet lengths,
sides, ship positions/types, reset/confirmation flags, fragmented/coalesced
input, disconnects and send backpressure. The pthread runs use the real
main-thread JavaScript bridge. See [HD netplay oracle details](HD-NETPLAY.md)
for fixtures, limitations, exact source hashes and report locations.

## Actual translator macro expansion

Run `python3 tests/rust/test-macros.py --clang "$CLANG_PATH"` after installing
the pinned translator. Each of the four configurations compares 162,704 results
with zero differences for the production policy; the default-policy negative
control produces 10,169 differences. It translates a real C fixture, compares
native/shared Wasm debug/release results against compiled C, and verifies that the former
conservative macro policy fails the negative control. See [macro regression
details](MACROS.md). Ordinary browser CI separately checks actual good/broken
Hyperspace screenshots and fresh-game ambient stars without installing C2Rust.
