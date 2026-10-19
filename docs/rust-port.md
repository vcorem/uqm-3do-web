# Rust migration

Both game engines now have Rust implementations: Original UQM and the separately
pinned HD MegaMod engine. The network relay is also implemented in Rust.
Default browser builds link these Rust engines; they do not link the old C game
object files. The work is isolated on `codex/rust-port`, starting from
`783d8d3ce`. The previous playable checkout and its saves are unchanged.

This is a source port, not a memory-safe or idiomatic rewrite. C2Rust translated
340 configured Original units and 350 HD units into `unsafe` Rust, preserving
the original ABI and algorithms. Ten units in each engine are superseded by
independently written, differentially tested Rust modules. The PNG integration
is also written in Rust, with a narrow C guard for libpng's `setjmp` errors.
Preserved C sources provide provenance and independent reference builds.

The HD Online Melee work from main commit
`5d7810ba5302803abe537f1b669d696a3af073ad` is included in this branch. Enabling
its WebSocket transport, protocol handlers and synchronization code expands the
configured HD engine from 330 to 350 translated units. The HD launcher, shared
browser client and Rust relay support the same edition-specific room contract.
This port does not import main's deployment setup or deploy a public service.

## Implementation boundaries

| Component | Implementation |
| --- | --- |
| Ships, combat, campaign, dialogue, saves, menus, resources, rendering, audio/video orchestration and threading | `rust/uqm-engine-original` and `rust/uqm-engine-hd` |
| RNG, square root, CRC, fixed-point movement, angles, queues, clipping and dirty regions | Shared `rust/uqm-core`, with edition-specific behavior |
| PNG decoding, HD writing, SDL conversion and stream ownership | Shared `rust/uqm-png/png.rs` |
| Private two-player relay and static host | `rust/uqm-relay`; native Rust executable using the existing browser protocol |
| Browser platform | Existing JavaScript launcher, persistence, WebSocket glue and generated Emscripten runtime |
| Native dependencies | SDL2, OpenAL, libpng, zlib, Ogg/Vorbis; HD also retains third-party Lua |
| Small C adapters | Emscripten main entry/proxy compatibility, JavaScript string storage, protected libpng and Lua error boundaries |

Lua and media libraries are dependencies, not Rust implementations. The C guard
contains no game logic; it prevents libpng's error jumps from crossing live Rust
frames. The JavaScript-string adapter preserves original byte strings in the
Wasm data section Emscripten requires, since Rust's custom section is unsuitable.
Rust startup explicitly initializes translated global tables once before the
first game call. Allocation uses the same Emscripten heap as the libraries.

`UQM_CORE=rust` is the default complete game-engine build. `UQM_CORE=c` builds
the reference C engine; `UQM_CORE=hybrid` retains the intermediate ten-module
migration for diagnosis. Edition-specific `engine-language.json` records the
actual implementation and dependencies. Media and save databases are unchanged.
See [translation provenance and regeneration](../scripts/rust-engine/README.md).

Both editions support Online Melee with another player using the same edition.
Original connects to `/netplay/<room>` and HD connects to `/netplay/hd/<room>`,
with the `role=host` or `role=guest` query. Both relays keep active and retired
rooms separate by edition and token while sharing resource limits. The engines
are incompatible across editions. The HD invitation retains the HD page path;
both players join and then click **Launch online Melee**. Campaign persistence
also stays separate: Original mounts `/home/web_user/.uqm`, while HD mounts
`/home/web_user/.uqm-hd`. No save conversion is performed.

## Toolchains and build

Rust 1.85.1 is pinned for native tools and the relay. WebAssembly uses the pinned
nightly in `.rust-wasm-toolchain` for C-variadic ABI support and to rebuild `core`, `alloc` and `compiler_builtins`
with atomics, bulk memory and mutable globals. The prebuilt Rust core does not
support the game's shared-memory link even when those flags are added to the
application crate. Both debug and release Wasm checks therefore use `build-std`.
Emscripten remains pinned to 3.1.50. The game library uses `no_std` and the
existing engine allocator; it does not introduce a second libc or allocator.
The current Wasm pin is `nightly-2025-02-01` (LLVM 19). A tested LLVM 20 nightly
emitted `bulk-memory-opt` and `call-indirect-overlong` feature metadata that the
pinned Emscripten optimizer cannot accept. Toolchain upgrades therefore require
both optimized differential executables and full game links; creating a Rust
static archive alone does not verify compatibility.

With Rustup and the existing Emscripten prerequisites installed:

```sh
rustup toolchain install 1.85.1 --profile minimal --component rustfmt --component clippy
rustup toolchain install "$(cat .rust-wasm-toolchain)" --profile minimal \
  --component rust-src --target wasm32-unknown-emscripten
npm ci
cargo build --manifest-path rust/Cargo.toml -p uqm-relay --release --locked
./scripts/build-web.sh
./scripts/build-hd-web.sh
./scripts/build-editions.sh
npm start -- --port 10000
```

For the isolated local toolchain under `.tools`, source `scripts/rust-env.sh`
from Bash before direct Cargo commands. Build scripts do that automatically.
The relay executable is `rust/target/release/uqm-relay`. Build it for the server's
actual Linux architecture before deploying; a local macOS binary is not portable
to Linux. `npm start` builds and runs the Rust relay; `npm run start:node`
runs the retained Node reference server.

```sh
# Example runtime arguments; no server deployment is performed by this branch.
./uqm-relay --host 127.0.0.1 --port 10010 \
  --origin https://playuqm.com --dist /srv/uqm/web
```

Preserve the HTTPS/WebSocket proxy and isolation headers described in
[the server guide](netplay-server.md). The Rust relay retains the same private
two-peer rooms, limits, exact origin checking and process-lifetime retired IDs.
Forwarded IP headers are not trusted; proxy-aware admission and long-term room-ID
retirement remain separate production work. No public deployment is implied.

## Verification

See [the dated local validation record](rust-validation.md) for the tested
Wasm hashes, completed checks and limits.

```sh
cargo fmt --manifest-path rust/Cargo.toml --all -- --check
cargo test --manifest-path rust/Cargo.toml --workspace --all-features --locked
cargo clippy --manifest-path rust/Cargo.toml --workspace --all-targets --all-features --locked -- -D warnings
npm test
./scripts/test-rust-core.sh --all
./scripts/test-rust-core.sh --all --hd
./tests/rust/test-velocity.sh --all
./tests/rust/test-velocity.sh --all --hd
./tests/rust/test-queue-geometry.sh --all
./tests/rust/test-queue-geometry.sh --all --hd
./tests/rust/test-bbox.sh --all
./tests/rust/test-bbox.sh --all --hd
./tests/rust/test-png.sh --wasm
python3 tests/rust/test-solar-sort.py
UQM_RELAY=rust UQM_DIST=dist npm run test:browser

# Build a genuine C reference and put one C client against one Rust client.
UQM_CORE=c BUILD_OUTPUT="$PWD/.tools/c-reference" ./scripts/build-web.sh
UQM_RELAY=rust UQM_DIST=dist UQM_PEER_DIST=.tools/c-reference \
  UQM_TEST_OUTPUT=test-results/rust-c npm run test:browser
UQM_RELAY=rust UQM_DIST=dist UQM_PEER_DIST=.tools/c-reference \
  UQM_SHIP_SMOKE=all UQM_TEST_OUTPUT=test-results/rust-ships npm run test:browser

# HD uses its own package, room namespace, save mount and C reference.
HD_BUILD_OUTPUT="$PWD/.tools/hd-dist" ./scripts/build-hd-web.sh
UQM_EDITION=hd UQM_RELAY=rust UQM_DIST=.tools/hd-dist \
  UQM_TEST_OUTPUT=test-results/hd-rust npm run test:browser
UQM_CORE=c HD_BUILD_OUTPUT="$PWD/.tools/hd-c-reference" ./scripts/build-hd-web.sh
UQM_EDITION=hd UQM_RELAY=rust UQM_DIST=.tools/hd-dist \
  UQM_PEER_DIST=.tools/hd-c-reference \
  UQM_TEST_OUTPUT=test-results/hd-rust-c npm run test:browser
```

The oracle compiles the unchanged C sources with renamed symbols and compares
them directly with Rust in one executable. Native C checks use ASan/UBSan;
the Wasm oracle uses the actual shared-memory link. Tests include ABI sizes,
context allocation/copy/free, seed boundaries and long sequences, integer square
roots, scalar CRC ordering and byte buffers. HD adds the real custom multiplier
formula, high-bit string bytes and defined integer range cases. The movement
oracle verifies the real 18-byte descriptor layout, exact sine table and angle
rounding, every 16-bit magnitude with all 16 facings, all raw facing values,
acceleration/frame sequences, arbitrary state fields and HD 32-bit durations.
Every `--all` invocation runs native and pthread Wasm, in debug and release;
`--hd` selects the independently pinned HD source and Rust feature. The HD
checkout must be available first, normally through `build-hd-web.sh`. CI runs
all these oracle suites for both editions. The queue/geometry oracle verifies
all valid pool capacities, allocation failure, exhaustion and reuse, linked
order and callbacks that remove their current slot. It compares rectangle
output aliases and complete line state, including rejected clips and untouched
fields, against the unchanged C sources through their actual public headers. See
[oracle details](../tests/rust/README.md).

The browser comparison streams separate C and Rust asset packages to independent
browser processes while retaining one public origin and one relay. It records
both Wasm hashes and tests full fleet matches, jitter, rematches, checksum
agreement, disconnect recovery and persistence. Rust-versus-Rust alone cannot
detect a matching regression on both peers. Test-only relay options and aggregate
events are explicit CLI flags, never controlled by a client URL.

`UQM_EDITION` defaults to `original`; set it to `hd` for HD. Both package paths
must contain standalone builds of that same edition. The harness checks the
actual WebSocket paths as well as engine synchronization and preserves the
selected edition's local fleet settings. An HD Rust/C pair compares language
implementations of HD; it does not connect HD to Original. The commands above
are reproducible checks, not a claim that every scenario has completed; consult
the dated validation record for the actual results and artifact hashes.

The old unused C `crc_processBytes` loops forever for nonempty input. Its Rust
replacement is intentionally corrected, using the scalar C CRC as its oracle.
This is documented as a correction, not claimed to be equivalent to the bug.
The movement oracle makes the legacy signed-wrap contract explicit with
`-fwrapv`; native shift sanitization is excluded because the C code shifts
negative coordinates. Other native ASan/UBSan checks remain enabled. Unsafe
minimum-signed-input angle cases are excluded from C equivalence claims; Rust
defines bounded behavior for them. Target-specific C `long` width is preserved
and checked separately on native and Wasm builds.
Queue capacities outside 1..255 and invalid slot layouts corrupt memory in the
legacy implementation and are excluded from differential input. The clipping
corpus bounds endpoint differences to 16,383 so doubled signed 16-bit deltas
remain representable; it covers the full signed coordinate range, all clip
sides, reversed endpoints, zero/negative rectangle extents and HD widened clip
rectangles. This avoids using the legacy divider's overflow cases as a safe
oracle while retaining its historical narrowing behavior elsewhere.
The original solar display list deliberately addresses `PlanetDesc[-1]` to
reach the adjacent sun descriptor. Rust explicitly selects `SunDesc[0]` for this
sentinel while retaining bounds checks for planets. An unused final link is
set to zero rather than reading beyond the C sort buffer at maximum capacity.
An extracted actual-C oracle checks ordering, HD sun ties and counts 0–16.

Translation expands all C macros before generating Rust. C2Rust's conservative
macro reconstruction incorrectly reused one `MakeLinks` value for different
arguments, leaving ambient star display lists empty. Both macro reconstruction
modes are disabled across both engines. A visual Hyperspace regression checks
distributed background stars; combat checksums alone cannot detect this renderer
failure.

The PNG oracle covers both original decoders and HD writing with real SDL/libpng,
including malformed/truncated data and exact successful writer bytes. Its C
operations are protected individually, and short transfers propagate only after
Rust callbacks return. See [PNG details](../tests/rust/PNG.md).

The optional ship smoke mode sends thrust, turn, weapon and special keyboard
input for each selected Melee ship while the C and Rust clients compare frame
checksums. It is a short per-ship integration check, not exhaustive proof of
every ability state or AI behavior.

Optional C `DUMP_CRC_OPS` file logging is not implemented in Rust. See the
[core contract](../rust/uqm-core/README.md) for additional exact ABI details.

HD's Lua integration additionally protects each Lua operation and returns
normally through Rust on failure. C trampolines rethrow only after Rust callbacks
have returned, including nested calls and allocation errors. A generated call
propagation audit rejects unknown Lua imports. Lua itself remains the pinned
third-party C interpreter; this boundary is not a rewrite of that interpreter.

## Validation limits and future cleanup

The port deliberately preserves unsafe raw pointers and C data layouts. A future
idiomatic Rust refactor should proceed behind the same independent oracles and
C-versus-Rust browser checks. Generated modules emit expected cross-module C-ABI
and static-mutable-reference warnings; build logs retain them for inspection.
These warnings are not evidence of memory safety.

Local automated checks cannot establish full campaign completion, every ship
ability and dialogue branch, all 3DO fidelity cases, real internet play between
two computers, or readiness for public release. Native standalone game builds
are not supplied: the game target is the existing browser WebAssembly platform.
The relay has native tests, but deployment to the Hetzner server is separate.
