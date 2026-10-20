# Rust port validation — 2026-10-19

Implementation commit: `3e8597d4bd25fc55f36cb1c6dba3ca37603f032b` on `codex/rust-port`,
based on `783d8d3ce10edbb9afe3e2e7848ac8ee9e5a7416`.
HD netplay incorporates main commit `5d7810ba5302803abe537f1b669d696a3af073ad`
(observed in main at `3ba129550`). This is local macOS validation. The expanded
Linux CI workflow has not been run on GitHub as part of this work, and this Rust
branch has not been publicly released or deployed. Main's separate hosting changes
were not imported.
The original `codex/web-netplay` checkout remains unchanged.

Delivery status reviewed on 2026-10-20: the initial Rust port is committed locally
as `84e963030d678aa6fedc7b05cd335ed67753f5b8`, followed by the HD netplay implementation
commit above. The HTML handover was refreshed to record this status. This is a
documentation-only update: the validation date and tested builds remain unchanged;
no new gameplay run is claimed. The branch has not been pushed by this work.
Linux CI and internet play between two physical computers remain unverified.

## Tested builds

These are the final complete Rust engine builds, regenerated with both C2Rust
macro reconstruction modes disabled. Earlier partial or faulty builds are not
used as evidence for these final game checks.

| Artifact | SHA-256 |
| --- | --- |
| Original Rust Wasm | `1955130068121e6c480bf30639257e74561d2c3d8b25d13befc36f537068e924` |
| HD Rust Wasm | `c706dc738c0eed78335a80eaff285e0e78b8c20814d0be550c71a9fda4c18fb3` |
| Independent HD C reference Wasm | `0adce7d85d0073952d0709d56593992688d73798df8c8c96b9a7afe9a3d0a53e` |
| Independent Original C reference Wasm | `ad1b2bb56eac5bf70387df0a5060a4c5e9424bb9165e9b6bffae0bf790615f97` |

Each edition's `link-inputs.json` records zero legacy C game objects. It permits
only the Rust archive, named C platform/error adapters and, for HD, the pinned
third-party Lua runtime. SDL2 and media libraries remain native dependencies.
Public-export checks cover 2,444 Original and 3,008 HD symbols; only the replaced
C entry point and each edition's unused private CRC table are intentionally absent.

Full regeneration covers 340 Original and 350 HD translation units. HD now has
`NETPLAY_FULL` enabled throughout the complete engine, including the 20 new
network/protocol units and all conditional structures and callers. Its CRC uses
the already tested shared Rust core. Six Original and eight HD explicit
initialization groups remain present. The verifier checks 19 pinned
reproduction inputs. HD's source audit validates 13,429 checked calls and 110
guarded Lua declarations against the actual transitive call graph.
The rebuilt Original Wasm is byte-identical to its prior validated artifact;
its launcher/client and the shared relay were updated for edition routing.

## Automated checks

| Check | Result and scope |
| --- | --- |
| Rust workspace | 35 unit tests passed; formatting and Clippy with warnings denied passed for the maintained core and relay workspace |
| Project tests | 111 passed, including actual Rust/Node WebSocket clients, bounds, disconnects, HTTP serving, persistence and browser helpers |
| Earlier fresh relay startup baseline | An empty custom Cargo target directory built automatically; the prior 99-test suite passed, and the launcher executed that binary |
| Regeneration helpers | 13 tests passed; strict link policy, source transformations, literal-preserving notice cleanup and all 19 provenance inputs |
| HD patch upgrades | Eight checks passed, including the actual pinned source, repeat application, legacy CRC-before-netplay caches, mode/content preservation, and unchanged source after patch failure |
| Edition isolation | Actual Rust/Node WebSocket clients verified that identical tokens cannot pair, forward bytes, disconnect or retire rooms across editions; shared capacity and strict paths also passed |
| HD packet/transport oracle | 48,840 matching C/Rust cases across debug/release × direct/pthread, plus 13 layout checks per mode; actual generated functions cover fragmentation, packet bounds, reset/ready flags, callbacks, disconnects and JS send/receive failure paths |
| Core differential matrices | Baseline: both editions compared with C in native/Wasm debug/release. HD core/CRC rerun after this port passed 15,671,769 comparisons per profile across all four profiles |
| Solar ordering | 1,501,984 field comparisons passed against extracted actual C, including the sun sentinel, HD ties and all planet counts 0–16 |
| Emscripten allocation | 19,200 debug/release cases passed, covering alignment 1–4096, zeroing, fragmentation, growth and shrinkage |
| PNG | Baseline: four native/Wasm configurations each passed 1,292,704 comparisons. Both Wasm profiles rerun after HD netplay passed the same counts, including malformed/truncated inputs and exact HD writer output |
| Lua error boundary | Baseline: four configurations passed 38,972 checks in 2,572 cases with 10,340 normal-return markers. Both Wasm profiles rerun after HD netplay each passed 9,743 checks in 643 cases, with 2,585 normal Rust-frame returns |
| Actual C2Rust macro regression (retained baseline) | 650,816 production-policy comparisons passed across four configurations; the faulty default policy failed the negative control in all four |
| Visual negative control | The star-background check accepts a real good frame and rejects a real broken frame even though both pass the former nonblank test |
| HD campaign | Real Load Game, Sol flight, exit to Hyperspace and continued flight passed, including distributed ambient-star pixels; screenshots inspected |
| HD stability | Five minutes in the actual fleet menu, followed by at least 15 seconds of real combat controls and visible playfield motion; no runtime/page errors (favicon 404s only) |
| Combined edition picker | Both final engines booted; HD rendered at 1280×960; audio and Melee worked; edition-specific saves remained isolated and persisted |
| Original voiced dialogue (unchanged-engine baseline) | Positive and negative controls passed: actual advancing speech PCM, visible changing subtitle text with the override, and no subtitles with the override removed |
| HD online matches | Rust/C, Rust/Rust and the independent C/C baseline each passed normal, jitter and setup scenarios with the Rust relay; full fleets, rematches, setup/selection/battle disconnects, menu return and persisted local fleets passed with zero checksum mismatches |
| Original online regression | Updated shared client/relay passed normal, jitter and setup scenarios again with two Original Rust clients; the engine Wasm remains byte-identical to baseline |
| Original C versus Rust online matches (retained baseline) | Normal and jittered full fleet matches passed, followed by rematches and disconnects in setup, selection and battle; both clients returned to menus and retained local settings after reload |
| All 25 HD ships | All indices 0–24 passed movement, weapon and special-input smoke checks with a Rust host and independent C guest through the Rust relay; fetched engine hashes and HD socket paths verified, with zero checksum mismatches |
| All 25 Original ships (unchanged-engine baseline) | All ship indices 0–24 passed real movement, weapon and special-input smoke checks against the independent C client with matching simulation checksums |

Native differential C harnesses use ASan/UBSan where documented. Rust is not
sanitizer-instrumented by these scripts. Documented C undefined-behavior cases
are not presented as a valid equivalence oracle. The maintained shared modules
did not change during the final whole-engine macro regeneration; their four-way
matrix results remain applicable.

Browser evidence uses separate processes and hashes the exact Wasm response
bytes forwarded to both clients. The earlier response-body inspector lookup
was replaced because HD's large media download could evict the Wasm response.
Successful new HD network and Original regression runs use installed Chrome
155.0.8059.84. Bundled Chromium 151.0.7922.34 timed out during screenshot capture
in initial C/C HD and Original runs; those are not counted as passing. The HD
C/C failure retained 309 matching checksums at ship selection, and a full
Chrome rerun passed. Failure diagnostics now save state/results before optional
screenshots and cleanup; the stalled Original test process tree was stopped.
Retained Original baseline integration evidence used Chromium 151. These are
local tests on one machine, not internet play between two physical computers.

## Reproduce and interpret

Use the commands in [the port guide](rust-port.md),
[core oracle guide](../tests/rust/README.md),
[PNG tests](../tests/rust/PNG.md), [Lua tests](../tests/rust/LUA.md) and
[actual-translator regression](../tests/rust/MACROS.md), and
[HD packet/transport oracle](../tests/rust/HD-NETPLAY.md).
Build artifacts and raw reports remain ignored under `.tools/` and
`test-results/`. The combined playable package is in `dist/`.
Current browser reports are in `test-results/hd-netplay-rust-c`,
`hd-netplay-rust-rust`, `hd-netplay-c-chrome155`, `hd-netplay-ships`,
`hd-netplay-original-chrome`, `hd-netplay-campaign`, `hd-netplay-stability`
and `hd-netplay-editions`. The checked aggregate is
`test-results/hd-netplay-validation.json`. Boundary results are in
`.tools/rust-hd-netplay-tests/results.json`; the new Wasm export report is
`test-results/hd-netplay-exports.json`. Retained Original baseline reports are
`test-results/rust-release-c`, `rust-final-ships` and `rust-final-subtitles`.

The translated engine deliberately retains unsafe pointers and the original
C data model. Compilation and these tests do not prove memory safety, every
dialogue/ability outcome, a completed campaign, every 3DO fidelity detail, native
standalone game support or public-release readiness. The per-ship check sends
real movement, weapon and special inputs and compares simulation checksums; some
ability effects require game situations that this short smoke check does not
exercise. Rendering is checked separately because matching simulation checksums
cannot detect missing artwork.
