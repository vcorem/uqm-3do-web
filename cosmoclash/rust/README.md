# Cosmo Clash Rust gameplay engine

This standalone Cargo workspace ports the gameplay in `src/engine`, `src/game`,
`src/ai` and the battle orchestration in the pinned upstream `main.ts`. Rendering,
DOM controls, audio playback and room transport stay in the browser host. The
browser does not run the retained TypeScript physics or AI.

Upstream: `ohanegby/cosmoclash-web`, commit
`966ad7b7ac90c44d3751dc014fbdc1ae03359e07` (ISC). See `../reference/upstream.json`
and the repository attribution for the original authors and assets.

## State and timing

All gameplay numbers use `f64`. The world owns two players, all projectiles,
asteroids, the sun, explosions, the camera's simulation position, AI state and a
seeded xorshift32 generator. The browser uses a fixed 60 Hz step. Collision
checks run before input and movement, matching the source game. Camera movement
is simulated here because its center affects wrapping; browser viewport sizes
cannot alter the battle state.

The Gondi secondary weapon deliberately charges its **primary** energy cost
(0.12), matching `main.ts`, rather than the unused `SECONDARY_ENERGY_COST` constant.
Sholev fires eight world-aligned ice shards. Shield toggling uses an input edge
and a two-second cooldown. MichiGichi keeps the current form's weapon during a
transition until its 40 animation steps complete. These details are covered by
both unit tests and actual upstream TypeScript differential tests.

Rendering-pool preallocation no longer consumes random numbers. Each new live
entity receives a monotonically increasing ID; reclaimed IDs are not reused.
Spawn randomness is seed-controlled, including asteroid placement, snot orbits
and AI decisions. The returned state includes RNG and AI state for reproducible
validation.

## WebAssembly ABI, version 1

The artifact is `cosmoclash_core.wasm`. Exports:

- `cosmo_alloc(len) -> pointer`: allocates a tracked zeroed input buffer; returns
  zero for invalid sizes or exhausted allocation slots.
- `cosmo_dealloc(pointer, len)`: frees only a tracked matching allocation.
- `cosmo_request(pointer, len) -> pointer`: reads a tracked input and returns a
  pointer to the engine-owned JSON result.
- `cosmo_result_len() -> len`: result length. Copy the result before making another
  request; never deallocate the engine-owned result.
- `cosmo_abi_version() -> 1`.

Requests are UTF-8 JSON and are limited to 64 KiB, with at most 16 live input
allocations. Arbitrary or freed pointers are rejected without dereferencing them.
Unknown request/input fields and invalid ship IDs, AI levels, and timesteps are
rejected. Timesteps must be finite and in `[0.001, 100]` milliseconds. Rejected
requests preserve the prior world.

```json
{"op":"init","seed":123,"ships":["gondi","michiGichi"],"aiDifficulty":"medium"}
{"op":"step","dt":16.666666666666668,"inputs":[{"thrust":true,"firePrimary":true},{}]}
{"op":"snapshot"}
```

Ship IDs are `gondi`, `michiGichi`, `sholev`. Omit `aiDifficulty` for two humans;
`easy`, `medium`, `hard` select source AI levels 1–3 for player two. Missing input
booleans default to false. Inputs are `thrust`, `rotateLeft`, `rotateRight`,
`firePrimary`, `fireSecondary`.

Responses are `{ "ok": true, "snapshot": ... }` or
`{ "ok": false, "error": "..." }`. Snapshots use camelCase, stable numeric IDs,
string entity type names and bullet kinds. `winner` is null, 1, or 2. Events last
for one step. A finished battle continues death animations while physics stops.
The host handles fleet advancement, menu navigation, and rematch selection.

## Validation

From the repository root, source `scripts/rust-env.sh` to use the cached toolchain.

```sh
cargo test --offline --manifest-path cosmoclash/rust/Cargo.toml
cargo clippy --offline --manifest-path cosmoclash/rust/Cargo.toml --all-targets -- -D warnings
cargo build --offline --manifest-path cosmoclash/rust/Cargo.toml --bin cosmo-jsonl
```

Run `tests/gameplay-parity.test.ts` and `tests/ai-parity.test.ts` through the
Cosmo Clash Vitest suite. Gameplay parity executes the actual preserved upstream
input and wrap functions, imports its untouched entity/collision/weapon classes,
and compares every simulated frame. AI parity compares the source controller,
learning state, tactic selection, random draw order and decisions. Small floating
point tolerances apply to derived geometry across native and JavaScript math;
decisions, IDs, arrays, booleans, and integer counters compare exactly.

The native `cosmo-jsonl` driver accepts the same requests as WebAssembly. It also
provides `testLoad` and `ai_oracle` solely for differential fixtures. These commands
are **not** part of the production WebAssembly API.
