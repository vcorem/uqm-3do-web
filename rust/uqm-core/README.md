# UQM deterministic Rust core

This crate replaces a defined set of routines in the real C engine. It is an
incremental engine port, not a complete Rust rewrite of Original or HD.

The production build is `no_std`, contains no Rust allocator, uses no external
crates, and exports the original C ABI as a static library. Rust unit tests use
the normal host test harness. Workspace development and release profiles must
set `panic = "abort"`; panics cannot unwind through the C ABI.

## Implemented ABI

| Original sources | Exported functions |
| --- | --- |
| `sc2/src/libs/math/random.c` | `TFB_Random`, `TFB_SeedRandom` |
| `sc2/src/libs/math/random2.c` | `RandomContext_New`, `RandomContext_Delete`, `RandomContext_Copy`, `RandomContext_Random`, `RandomContext_SeedRandom`, `RandomContext_GetSeed` |
| `sc2/src/libs/math/sqrt.c` | `square_root` |
| `sc2/src/uqm/supermelee/netplay/crc.c` | `crc_init`, `crc_processBytes`, `crc_processUint8`, `crc_processUint16`, `crc_processUint32`, `crc_finish` |
| `sc2/src/uqm/trans.c` | `ARCTAN`, writable `sinetab` object |
| `sc2/src/uqm/velocity.c` | `GetCurrentVelocityComponents`, `GetNextVelocityComponents`, `SetVelocityVector`, `SetVelocityComponents`, `DeltaVelocityComponents` |
| `sc2/src/libs/graphics/bbox.c` | Six `TFB_BBox_*` operations, `TFB_BBox`, `maxWidth`, `maxHeight` |
| `sc2/src/uqm/displist.c` | `InitQueue`, `UninitQueue`, `ReinitQueue`, `AllocLink`, `FreeLink`, `PutQueue`, `InsertQueue`, `RemoveQueue`, `CountLinks`, `ForAllLinks` |
| `sc2/src/libs/graphics/boxint.c` | `BoxIntersect`, `BoxUnion` |
| `sc2/src/libs/graphics/clipline.c` | `_clip_line` |
| `sc2/src/libs/graphics/bbox.c` | `TFB_BBox_Init`, `TFB_BBox_Reset`, `TFB_BBox_SetClipRect`, `TFB_BBox_RegisterPoint`, `TFB_BBox_RegisterRect`, `TFB_BBox_RegisterCanvas`; globals `TFB_BBox`, `maxWidth`, `maxHeight` |

`DWORD`/`uint32` are `u32`; `COUNT`/`uint16` are `u16`; `uint8` is `u8` and
`size_t` is `usize`. Both context structures contain one `u32` and use `repr(C)`.
Pointers retain the original C validity and serialization preconditions.
Random contexts are allocated and freed through the engine's `HMalloc`/`HFree`,
so they never cross allocator domains. The global seed uses atomic operations;
its sequence is identical for serialized calls. Its ordering does not publish
unrelated data or make the remainder of the C simulation thread-safe.

`SIZE` and `COORD` remain signed 16-bit values; `SDWORD` is signed 32-bit.
`VelocityDesc` is the exact `VELOCITY_DESC` layout: 18 bytes, alignment 2,
with `TravelAngle` at offset 0 and the four 4-byte `EXTENT` pairs at offsets
2, 6, 10 and 14. Each pair contains signed 16-bit width/height values, including
in HD. Movement preserves fixed-point remainders, arithmetic right shifts,
narrowing conversions and the C order of output/error writes. Legal output
aliasing retains that order. The sine table contains the exact original
truncated integer constants; it is not regenerated from floating-point sine.
Table mutation must be serialized with reads, as with the C implementation.

The intrusive queue ABI retains the mandatory `QUEUE_TABLE` pool used by both
engines. Capacity is fixed: gameplay encounter limits depend on allocation
exhaustion. `Queue` contains four pointers (`head`, `tail`, `pq_tab`,
`free_list`), followed by a `u16` slot size and `u8` capacity. It is 20 bytes
with alignment 4 on Wasm32 and 40 bytes with alignment 8 on the native 64-bit
test host. Each slot starts with a two-pointer `Link`. Pool allocation order,
LIFO reuse, stale predecessor/payload fields and cached-successor callback
iteration match C. Callbacks may remove/free their current slot. No exclusive
Rust references are held across C callbacks. Allocation exhaustion preserves
the existing `log_add(log_Debug, ...)` diagnostic through the C logger.

Geometry uses the original 8-byte, alignment-2 `RECT`, with signed 16-bit
coordinates and extents. `BRESENHAM_LINE` is 24 bytes with alignment 4; its
`BOOLEAN` field is a four-byte C enum, not a one-byte boolean. Intersection
rejection preserves untouched rectangle output; line rejection preserves the
original partial writes. `BoxUnion` deliberately retains C's sequential reads
and writes when output aliases an input, including its legacy behavior that
differs from computing a union from snapshots of the two inputs.

Dirty-region accounting keeps the 20-byte `TFB_BoundingBox` layout and the
renderer-owned C globals. Its accesses must remain on the screen-owning thread,
as required by the original header. The existing `TFB_DrawCanvas_GetExtent`
backend supplies canvas dimensions. Global-region aliases passed to
`TFB_BBox_RegisterRect` retain the original two-step read/update sequence.

The Emscripten build needs the same ABI and shared-memory target features as the
game. `scripts/build-rust-core.sh` uses the pinned nightly toolchain recorded in
`.rust-wasm-toolchain` and rebuilds `core`/`compiler_builtins` with the game's
atomics, bulk-memory and mutable-globals flags. A normal precompiled `core` can
pull non-shared-memory objects into the archive through checked-arithmetic
panic paths; enabling atomics only on this crate is insufficient. Host tests
use the stable toolchain in `rust-toolchain.toml`.
The Wasm pin is currently `nightly-2025-02-01` with LLVM 19, paired with
Emscripten 3.1.50. A newer LLVM 20 nightly emits feature metadata unsupported by
that Emscripten optimizer. Validate final optimized links before changing the
pin; archive creation does not exercise the Wasm linker or optimizer.

The crate's own `no_std` panic handler is intended for the final C-linked
artifact. Its `rlib` is not intended to be linked into a separate Rust program
with another panic handler without adjusting that integration.

## Intentional bug correction

The original, unused `crc_processBytes` never increments its input pointer and
therefore hangs for any nonempty buffer. The Rust function consumes each byte
once and terminates. This is **not** behavioral equivalence to that faulty C
function. It produces the same result as calling the original scalar
`crc_processUint8` for each byte, with the same initial and final CRC state.
Empty input leaves the state untouched; its buffer may be null. All active
scalar CRC functions preserve their original little-endian ordering.

Compile-time `DUMP_CRC_OPS` diagnostic file logging from the old C CRC functions
is not exported by this crate. Checksums and their wire representation are
preserved; callers requiring that optional logging must keep the C backend or
add a separate diagnostic adapter.

## HD engine variant

The `hd` feature retains MegaMod's runtime planet-generation multiplier. Its C
configuration adapter `uint32_t uqm_hd_seed_a(void)` returns the current `SeedA`
macro. The Rust arithmetic preserves unsigned wrapping even for the large
configured multipliers. Configuration changes must be serialized with calls,
as in the existing engine. Original builds never read HD configuration.

The feature additionally exports `RandomContext_Set`, `RandomContext_FastRandom`,
`RangeMinMax` and `crc32b`. `Set` preserves raw seed bits instead of normalizing.
`crc32b` retains the source's C `char` promotion, so high-bit bytes differ from
ordinary CRC-32 on signed-char platforms. `RangeMinMax` preserves all defined C
inputs and uses widened arithmetic to define formerly overflowing ranges,
including `INT_MIN..INT_MAX`. The C byte-printing diagnostic `printBits` remains
an integration adapter, not a Rust simulation routine.

HD's `ARCTAN` and the three velocity setters use signed 32-bit arguments where
Original uses signed 16-bit arguments. Its sine-table entries are also widened
to signed 32-bit values. The HD feature
also exports `GetCurrentVelocityComponentsSdword` and
`GetNextVelocityComponentsSdword`, with signed 32-bit outputs and a 32-bit frame
count for the latter. The descriptor layout remains unchanged. Sine products
preserve target C `long` width (32-bit Wasm, 64-bit macOS), including wrapping
before final narrowing.

HD's `_clip_line` accepts a 16-byte, alignment-4 `DRECT` with signed 32-bit
coordinates/extents. Its internal clip bounds and line coordinates still
narrow to signed 16-bit values as in the upstream implementation. The other
geometry and queue interfaces are identical in Original and HD.

## Legacy movement edge cases

Rust defines two's-complement wrapping for the C engine's historical signed
overflow and negative-left-shift operations. The differential oracle uses the
actual C source with `-fwrapv`, and checks Emscripten results directly. Its native
UBSan shift check is disabled for this known legacy pattern; other UBSan checks
and ASan remain enabled on the C harness.

Original `ARCTAN` cannot represent the absolute value of a single `i16::MIN`
input; these cases can index beyond the C table or divide by zero. Rust uses
the full unsigned magnitude instead. The reproducible Original pair
`(i16::MIN, i16::MIN)` preserves its legacy result 0. HD's overflowing
`i32::MIN` negation also receives defined unsigned-magnitude behavior. These
formerly unsafe inputs are unit-tested as extensions, not claimed as safe C
behavior that can be used indiscriminately as an oracle.

`InitQueue` requires a capacity of 1..255 and an aligned slot size large enough
for `LINK`. The old C allocator truncates capacity to a byte but initializes
slots using the original 16-bit count; zero or excessive capacities can write
outside the allocation. Rust asserts these invalid contracts before allocating
instead of reproducing memory corruption. Existing engine call sites satisfy
these requirements. As before, list operations require valid pool membership
and appropriate allocation/list state; the port does not make malformed raw
C pointers safe. All queue access remains serialized by the engine.

Clipping explicitly wraps historical signed doubling and narrowing operations.
The algorithm's integer division and inclusive-edge rounding are retained,
including the deltas and error state used by the subsequent renderer.

## Verification

Unit tests check UQM's unsigned Park-Miller sequence, exact normalization edge
cases, global state restoration, independent context copying, every 16-bit
square boundary, CRC-32's standard `123456789` vector, streaming/scalar order,
empty and overlapping byte buffers, and C-compatible type layouts.
Movement unit tests add zero/cardinal/diagonal angles, table values, exact
descriptor layout, fixed-point stepping, signed boundary behavior and aliased
outputs.
Queue and geometry unit tests cover capacity exhaustion, deterministic reuse,
reset, callback removal, rectangle aliasing, rejected-line mutation and
inclusive clipping edges. Independent C oracles check the complete structures
and operation traces on native and shared-memory Wasm targets.

Run the independent actual-source suites from the repository root:

```sh
./scripts/test-rust-core.sh --all
./scripts/test-rust-core.sh --all --hd
./tests/rust/test-velocity.sh --all
./tests/rust/test-velocity.sh --all --hd
./tests/rust/test-queue-geometry.sh --all
./tests/rust/test-queue-geometry.sh --all --hd
./tests/rust/test-bbox.sh --all
./tests/rust/test-bbox.sh --all --hd
```

Each command tests native debug/release and pthread Wasm debug/release. Native
C harnesses use ASan/UBSan; Rust itself is not sanitizer-instrumented by these
scripts. HD requires the source checkout created by `build-hd-web.sh`. See
[`tests/rust/README.md`](../../tests/rust/README.md) for the exact corpus,
configuration fixtures and documented C exclusions.

These tests complement, rather than replace, the independent differential C
oracle and real browser checks. Exact game fidelity requires running those
checks on the integrated artifact and comparing a C client with a Rust client.
Ships, collision, combat orchestration, campaign, most rendering, media,
resources, threading and save/load remain in the C engine; these passing
boundaries do not constitute a complete Rust port.

## Renderer dirty-region boundary

`bbox` retains the 20-byte `TFB_BoundingBox` with its 32-bit valid flag and
two 16-bit-coordinate rectangles. The writable exported globals and calls
remain exclusive to the screen-owning thread, as required by `bbox.h`; the
port does not claim synchronization of direct C global accesses. Initialization
and reset retain the source's partial writes. Ordered clamping preserves empty
and reversed bounds. Registering a rectangle re-reads its second point after
the first update, including when the pointer aliases the global region.
`TFB_DrawCanvas_GetExtent` remains a backend callback with the original ABI.

## Provenance and license

Derived from the UQM sources listed above, under GPL-2.0-or-later, preserving
the algorithms and public interfaces. Original copyright holders include
Paul Reiche, Fred Ford, Robert Leyland and Scott Anderson for the math sources,
and Serge van den Boom for the CRC sources. See the repository's `sc2/COPYING`
and the original source files for the full notices and license terms.
