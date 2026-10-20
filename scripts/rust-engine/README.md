# Reproducing the Rust engines

The checked-in `rust/uqm-engine-original` and `rust/uqm-engine-hd` crates contain
generated **unsafe Rust implementations**, plus the independently tested
`uqm-core` implementation of math, CRC, velocity, queues and geometry. C2Rust is
not needed for ordinary builds. Each engine is a separate Cargo workspace so
native relay/core tests do not accidentally compile browser-only engine code.

Build the engine archives from the repository root:

```sh
scripts/rust-engine/build.sh original
scripts/rust-engine/build.sh hd
```

These use the pinned Rust nightly in `.rust-wasm-toolchain`, rebuild
`core`, `alloc` and `compiler_builtins` for shared-memory Emscripten, and emit
`libuqm_engine_original.a` / `libuqm_engine_hd.a` under
`.tools/rust-engine-*/wasm32-unknown-emscripten/release/`.
The final Emscripten link additionally needs the normal platform libraries,
`uqm_rust_entry.c` and `uqm_asm_consts.c`. HD retains its external Lua runtime.
PNG handling uses the shared Rust source in `rust/uqm-png/png.rs` and its C
guards around individual libpng operations. Link `png-guard.c` once. Do not
restore the generated `setjmp` implementation, which would jump across Rust
frames.

The C `crcTable` data export is intentionally absent in both editions: only the old
CRC implementation used it, and `uqm-core` owns all CRC operations. All game
function exports are preserved. Generated modules superseded by `uqm-core`
remain in the source mapping but are excluded by `lib.rs`.

## Translator and source pins

`provenance.json` records the translator commit, LLVM, Rust, Emscripten and HD
source revisions, plus hashes of the translator patch, source locks and
postprocessing tools. `verify-provenance.py` rejects altered or nonportable
reproduction inputs; deliberate pipeline changes require a reviewed hash update.
`compile-db.py` also rejects mismatched Clang and Emscripten versions.
HD browser netplay incorporates upstream web commit
`5d7810ba5302803abe537f1b669d696a3af073ad` from this project's main branch.
The entire HD engine is regenerated with `NETPLAY=NETPLAY_FULL`, including the
20 network/protocol units and every conditional caller and structure. Browser
transport operations run through the same Emscripten main-thread JavaScript
bindings as the C reference. HD rooms use a separate relay namespace from
Original rooms; the two game editions are not simulation-compatible.

`original-inputs.json` and `hd-inputs.json` record exact
compiler flags, active source units, and hashes of C sources and headers.
They also fix the historical `__DATE__` / `__TIME__` build banner strings, so
regenerating on another day does not change engine sources.
`compile-db.py` rejects changed inputs and resolves Emscripten's real Clang
invocation rather than treating driver `-s` flags as Clang options.

Use LLVM 18.1.8 with its Clang development libraries and CMake. The translator
setup script installs nothing globally; it requires the following environment
variables to point to the prepared tools:

```sh
export LLVM_CONFIG_PATH=/absolute/llvm18/bin/llvm-config
export CLANG_PATH=/absolute/llvm18/bin/clang
export LIBCLANG_PATH=/absolute/llvm18/lib
scripts/rust-engine/setup-translator.sh
```

`c2rust-wasm.patch` disables Darwin header injection and C2Rust's erroneous
classification of MikMod `_mm_*` function names as x86 SIMD intrinsics when the
explicit Wasm experiment flags are enabled. Neither patch changes game logic.
The HD prelude supplies an existing file-scope declaration so repeated
block-scope `extern starmap_array` declarations share the correct identity.

## Regeneration workflow

Use configured, pinned Original/HD source trees with the repository's browser
patches already applied. Regeneration must go into an empty staging directory:

```sh
python3 scripts/rust-engine/regenerate.py \
  --edition original --source sc2 --output .tools/regenerated-original \
  --emcc .tools/emsdk/upstream/emscripten/emcc \
  --clang /absolute/llvm18/bin/clang \
  --c2rust .tools/c2rust-pinned-target/release/c2rust-transpile
```

For HD use `--edition hd --source .tools/hd-source` and a different output.
The locked HD source list deliberately excludes the external Lua runtime.
The tool translates with `--fail-on-error`, `--translate-const-macros none`
and `--translate-fn-macros none`, preserving each Clang-expanded expression.
This avoids C2Rust reconstructing different function-macro invocations as a
single constant; that behavior previously erased the ambient star display
list in both editions. It installs explicit startup
initialization, and normalizes compatibility with the pinned Rust version.
Compile the staging crate for Emscripten to produce its Cargo.lock before
running `package.py`; packaging adds source notices/mappings, removes machine
paths from C diagnostics, selects `uqm-core` and the shared Rust PNG module,
and extracts original JS byte strings into the Emscripten data adapter. HD
packaging then invokes the pinned Rust syntax-aware Lua propagation helper,
which keeps Lua errors inside C guards and returns through Rust normally.
It records transformed calls for a fail-closed audit. Packaging also requires
an empty output directory so old transformation reports cannot mask new code. Keep
`uqm-core`, `uqm-png` and `uqm-lua` as siblings when compiling a packaged staging
copy.

```sh
python3 scripts/rust-engine/package.py \
  --edition original --generated .tools/regenerated-original/generated \
  --source sc2 --output /absolute/staging/uqm-engine-original
python3 scripts/rust-engine/test_tools.py
```

Review the resulting diff and repeat public-symbol, C/Rust differential,
browser startup/campaign, and network gameplay tests before adopting it.
Compilation and symbol coverage alone do not establish gameplay equivalence.

## Validation performed during this migration

The portable regeneration script was rerun over all 340 Original and 350 HD
units with both macro reconstruction modes disabled. Both packaged staging
crates compiled successfully for shared-memory Wasm, retained every expected
game export, and preserved the verified core, PNG, solar display list,
allocator and Lua boundary transformations.
ASM adapter regressions cover exact bytes, repeatability, corrupt data, invalid
lengths, and rejection of partially regenerated state without source mutation.
Runtime verification is recorded by the main project test reports.

The checked-in `original-exports.json` / `hd-exports.json` snapshots were captured
from all 340 / 350 real Emscripten C objects (2,444 / 3,008 public symbols). They
record each source hash and are checked against the translation input locks.
`capture-exports.py` reproduces a staged snapshot from the complete pinned C
reference object tree after verifying every locked source/header hash. Review
snapshot changes explicitly; the build never refreshes its own expected list.
Fresh builds can verify export coverage without old C object directories:

```sh
python3 scripts/rust-engine/check-exports.py original .tools/rust-engine-original/wasm32-unknown-emscripten/release/libuqm_engine_original.a
python3 scripts/rust-engine/check-exports.py hd .tools/rust-engine-hd/wasm32-unknown-emscripten/release/libuqm_engine_hd.a
bash tests/rust/test-allocator.sh
```

The only permitted missing archive symbols are the C platform entry and the
private, replaced CRC table. The audit also rejects archive members
that are not Rust codegen objects. `link-policy.py` rejects legacy C game input
files in the final link while allowing the named platform/codec guards, the
Emscripten compatibility shim and HD's pinned external Lua runtime. These checks
establish build composition and public-symbol coverage, not gameplay parity.
The allocator test uses the actual production allocator on the real shared
Emscripten heap, checking alignment, zero-fill, growth and shrinkage.

The solar display list uses an explicit SunDesc selection for the C index `-1`;
other planet indexes retain bounds checks. Its unused final NextIndex is set to
zero instead of reading uninitialized/out-of-bounds C data. The actual-C oracle
in `tests/rust/test-solar-sort.py` checks both editions, counts 0–16, ties and
extreme/random positions at native O0/O2. The regeneration packaging applies
this narrow correction with `solar-sort-fix.py`.

The aggregate engine license is GPL-2.0-only because the bundled uio code has
that restriction. Individual UQM/codec files retain their original notices.
Each generated source preserves its upstream leading copyright/license notices.
`SOURCE-MAP.json` associates every translated unit with its C path and hash.
`COPYING` and `notices/` retain upstream license and attribution documents.
