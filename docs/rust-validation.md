# Rust port validation — 2026-10-19

Branch: `codex/rust-port`, based on `783d8d3ce10edbb9afe3e2e7848ac8ee9e5a7416`.
This is local macOS validation. The expanded Linux CI workflow has not been run
on GitHub as part of this work, and no public release or deployment was made.
The original `codex/web-netplay` checkout remains unchanged.

## Tested builds

These are the final complete Rust engine builds, regenerated with both C2Rust
macro reconstruction modes disabled. Earlier partial or faulty builds are not
used as evidence for these final game checks.

| Artifact | SHA-256 |
| --- | --- |
| Original Rust Wasm | `1955130068121e6c480bf30639257e74561d2c3d8b25d13befc36f537068e924` |
| HD Rust Wasm | `493af1f4f2e5ad7cfdc3a9972198443545fa8e9bab2e816a63d2c13d405dbfca` |
| Independent Original C reference Wasm | `ad1b2bb56eac5bf70387df0a5060a4c5e9424bb9165e9b6bffae0bf790615f97` |

Each edition's `link-inputs.json` records zero legacy C game objects. It permits
only the Rust archive, named C platform/error adapters and, for HD, the pinned
third-party Lua runtime. SDL2 and media libraries remain native dependencies.
Public-export checks cover 2,444 Original and 2,799 HD symbols; only the replaced
C entry point and Original's unused private CRC table are intentionally absent.

Full regeneration covers 340 Original and 330 HD translation units. Compared
with the previous translation, exported signatures and all parsed C-layout
structs/unions are unchanged. Six Original and seven HD explicit initialization
groups remain present. The regeneration verifier checks 17 pinned input/tool
hashes. HD's source audit validates 13,209 checked calls and 110 guarded Lua
declarations against the actual transitive call graph.
After final nonsemantic source-notice cleanup, both production builds were run
again; their Wasm files are byte-for-byte identical to the tested hashes above.

## Automated checks

| Check | Result and scope |
| --- | --- |
| Rust workspace | 35 unit tests passed; formatting and Clippy with warnings denied passed for the maintained core and relay workspace |
| Project tests | 99 passed, including actual Rust/Node WebSocket clients, bounds, disconnects, HTTP serving, persistence and browser helpers |
| Fresh relay startup | An empty custom Cargo target directory built automatically; all 99 tests passed, and the launcher executed that binary |
| Regeneration helpers | 13 tests passed; strict link policy, source transformations, literal-preserving notice cleanup and provenance checks |
| Core differential matrices | Original and HD RNG, CRC, square root, movement, angles, queues, geometry and dirty-region modules compared with actual C in native/Wasm debug/release |
| Solar ordering | 1,501,984 field comparisons passed against extracted actual C, including the sun sentinel, HD ties and all planet counts 0–16 |
| Emscripten allocation | 19,200 debug/release cases passed, covering alignment 1–4096, zeroing, fragmentation, growth and shrinkage |
| PNG | Four native/Wasm debug/release configurations each passed 1,292,704 comparisons, including both C decoders, malformed/truncated input and exact HD writer output |
| Lua error boundary | Four configurations passed 38,972 checks in 2,572 cases, including OOM, nested callbacks, readers, finalizers and error handlers; 10,340 Rust normal-return markers verified |
| Actual C2Rust macro regression | 650,816 production-policy comparisons passed across four configurations; the faulty default policy failed the negative control in all four |
| Visual negative control | The star-background check accepts a real good frame and rejects a real broken frame even though both pass the former nonblank test |
| HD campaign | Real Load Game, Sol flight, exit to Hyperspace and continued flight passed, including distributed ambient-star pixels; screenshots inspected |
| HD stability | Five minutes in the actual fleet menu, followed by at least 15 seconds of real combat controls and visible playfield motion; no recorded browser errors |
| Combined edition picker | Both final engines booted; HD rendered at 1280×960; audio and Melee worked; edition-specific saves remained isolated and persisted |
| Original voiced dialogue | Positive and negative controls passed: actual advancing speech PCM, visible changing subtitle text with the override, and no subtitles with the override removed |
| C versus Rust online matches | Normal and jittered full fleet matches passed, followed by rematches and disconnects in setup, selection and battle; both clients returned to menus and retained local settings after reload |
| All 25 Original ships | All ship indices 0–24 passed real movement, weapon and special-input smoke checks against the independent C client with matching simulation checksums |

Native differential C harnesses use ASan/UBSan where documented. Rust is not
sanitizer-instrumented by these scripts. Documented C undefined-behavior cases
are not presented as a valid equivalence oracle. The maintained shared modules
did not change during the final whole-engine macro regeneration; their four-way
matrix results remain applicable.

Browser evidence uses separate processes and checks the fetched C and Rust Wasm
hashes. Original integration tests use Chromium 151.0.7922.34; HD campaign and
stability checks use installed Chrome 155.0.8059.84. These are local tests on one
machine, not internet play between two physical computers.

## Reproduce and interpret

Use the commands in [the port guide](rust-port.md),
[core oracle guide](../tests/rust/README.md),
[PNG tests](../tests/rust/PNG.md), [Lua tests](../tests/rust/LUA.md) and
[actual-translator regression](../tests/rust/MACROS.md).
Build artifacts and raw reports remain ignored under `.tools/` and
`test-results/`. The combined playable package is in `dist/`.
Final browser reports are in `test-results/rust-release-c`,
`rust-final-ships`, `rust-final-subtitles`, `rust-final-hd-campaign`,
`rust-final-hd-stability` and `rust-final-editions`.

The translated engine deliberately retains unsafe pointers and the original
C data model. Compilation and these tests do not prove memory safety, every
dialogue/ability outcome, a completed campaign, every 3DO fidelity detail, native
standalone game support or public-release readiness. The per-ship check sends
real movement, weapon and special inputs and compares simulation checksums; some
ability effects require game situations that this short smoke check does not
exercise. Rendering is checked separately because matching simulation checksums
cannot detect missing artwork.
