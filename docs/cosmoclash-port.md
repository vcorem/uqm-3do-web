# Cosmo Clash: Rust port and online fleet battles

Branch: `codex/cosmoclash-rust-netplay`

Implementation, local validation and Linux CI: 20 October 2026

## Source and scope

The import is from `ohanegby/cosmoclash-web` at
`966ad7b7ac90c44d3751dc014fbdc1ae03359e07`. Original creators are Ori Hanegby,
Guy Corem and Orr Kowarsky. Its package declares ISC. The import preserves the
original artwork and credits, README, entry point, and a SHA-256 provenance
manifest. Existing UQM licensing is unchanged.

`cosmoclash/` is a separate, buildable game in this repository. Nothing replaces
UQM Original or HD. The existing relay gains an isolated Cosmo room namespace;
all three game namespaces share the relay's resource limits.

The Rust crate implements all gameplay simulation and AI: the three ships,
Michi/Gichi morph, primary and secondary weapons, Gondi slime, Sholev frost
shield, energy/cooldowns, movement, gravity, collisions, wrapping, camera,
projectiles/asteroids/explosions, winners, and all AI difficulties/tactics/learning.
It compiles to `wasm32-unknown-unknown` without Emscripten or a C gameplay bridge.

Three.js and TypeScript still provide rendering, browser input/audio, menus,
ship selection, fleet/round coordination and WebSocket transport. This is a Rust
simulation and AI port, not a rewrite of every browser component in Rust.
The old TypeScript simulation and AI classes are retained as independent test
references; the playable battle loop calls the Rust engine.

## Behavior and networking

- Solo retains the three-ship gauntlet and Easy/Medium/Hard AI. Same-computer
  two-player play is also available.
- Online Melee uses private two-player invitations. Each side chooses one to
  three unique ships and their order. Both must be ready before starting.
- The defeated side advances its fleet. Both ships reset for the next battle,
  as in the original local game. A fleet defeat ends the match. Rematch requires
  mutual acceptance and a fresh ready check; leaving retires the room.
- The simulation advances at a fixed 60 Hz, with seeded game randomness and a
  three-tick input delay. The peers exchange inputs in lockstep and compare
  full simulation and fleet state checksums. A detected disagreement stops play.
- The handshake checks the protocol and actual Wasm build hash. Final result
  agreement requires the same winner, final tick and checksum.
- The relay transports bounded packets; it does not simulate battles. There is
  no account system, matchmaking service, database, or competitive anti-cheat.
  A player can modify their own client, so this is intended for private matches.
- Cosmetic effects do not consume simulation randomness. Original gameplay
  constants and update ordering come from source, even where older upstream
  documentation describes different values.

The default controls are arrows, `/` for primary fire and `.` for the special
ability; `M` toggles audio. Player two on the same keyboard uses W/A/D, Space
and Shift. Online players both use the default controls locally.

## Reproduce locally

Run from the repository root with Node.js 22.12+, Bash, Python 3 and Rust 1.85.1:

```sh
npm ci
npm ci --prefix cosmoclash
rustup target add wasm32-unknown-unknown --toolchain 1.85.1
npm run build --prefix cosmoclash
npm start --prefix cosmoclash -- --port 10000
```

Open `http://127.0.0.1:10000`. The existing UQM preview on port 9999 is independent.
The production build writes `cosmoclash/dist/`; generated Wasm and build output
are deliberately not tracked. `npm run dev --prefix cosmoclash` uses Vite, whose
WebSocket proxy expects the separately running relay on port 10000.

## Server handover

Build the portable game assets above. Build the existing Rust relay on Linux
for the VPS architecture (or use a matching Linux cross-build); a macOS relay
binary cannot run on a Hetzner Linux VPS:

```sh
cargo build --locked --release --manifest-path rust/Cargo.toml -p uqm-relay
rust/target/release/uqm-relay --host 127.0.0.1 --port 10010 \
  --origin https://cosmo.example.com --dist /srv/cosmoclash/dist
```

Replace the example origin and path with the deployed values. Install the built
relay binary and static `dist/` directory on the VPS, run the relay as an
unprivileged system service, and use the existing HTTPS reverse proxy. It must
forward HTTP and WebSocket upgrades for `/netplay/cosmoclash/` to this process.
Prefer a dedicated hostname serving Cosmo at `/`; asset paths assume this root.
Use the same hostname and scheme for the game and its WebSocket endpoint.
Preserve the server's response headers and configure an idle timeout that allows
long-lived WebSockets. Public HTTPS is required for reliable browser crypto and
clipboard APIs. No Redis, database, STUN, TURN or separate game-authority process
is required. A domain registration and public deployment are separate work.

Room state is memory-only. Restarting the service disconnects every active room.
Default limits are 128 active rooms, 16 connections per observed source IP, and
10,000 distinct room identities over the process lifetime. Behind a reverse
proxy all clients share the proxy's observed source IP; forwarded IP headers are
not trusted. Account for this before opening to a large public audience. See
[`rust/uqm-relay/README.md`](../rust/uqm-relay/README.md) for the exact server
contract and [`docs/netplay-server.md`](netplay-server.md) for the existing UQM
hosting background. Test flags for artificial latency must not be used in
production.

## Verification

The following commands exercise the final source and build:

```sh
npm run test:provenance --prefix cosmoclash
npm run test:rust --prefix cosmoclash
npm test --prefix cosmoclash
npm run build --prefix cosmoclash
npm run test:wasm --prefix cosmoclash
npm test
npx playwright install chromium
npm run test:browser --prefix cosmoclash
```

The source comparison suites execute the original TypeScript implementation and
Rust with identical inputs and seeded randomness, including an intentional
negative control. They compare gameplay state, audio-event ordering, AI flags,
timers, decisions, learning and RNG state. Floating-point geometry comparisons
allow a small tolerance. Independent Wasm instances must match exactly.

Browser tests launch actual independent browser processes against the actual
Rust relay, fetch the production build, and check its Wasm SHA-256. They use real
keyboard controls, including an automated aiming player; they do not inject
wins or replace simulation state. Coverage includes solo AI, morphing, local
weapons/shield/slime, actual audio output and mute, a complete three-ship online
fleet match, rematch, disconnect, and a separate connection with artificial
latency/jitter. Browser checks also inspect loading failures and script errors.

Raw reports and screenshots are generated under `cosmoclash/test-results/`
(ignored by Git). The dedicated GitHub workflow is
`.github/workflows/cosmoclash.yml`. Local results on macOS/Apple Silicon with
Chrome are recorded below. The complete dedicated Linux workflow also
[passed on GitHub](https://github.com/vcorem/uqm-3do-web/actions/runs/39367088430)
at commit `8d1148923aa1221b8322567c38a3b0c0302c09d9`, including the patched
dependencies, source comparisons, production Wasm, shared-relay regressions,
and actual two-browser fleet combat, rematch, latency/jitter and disconnect.
This does not establish behavior across separate public internet connections.

| Check | Local result |
| --- | --- |
| Original artwork/reference integrity | 44 assets and 62 source references match upstream; original entry point and README preserved |
| Browser/source unit and integration suite | 635 tests across 41 files passed |
| Rust engine | 27 tests passed; formatting and Clippy with warnings denied passed |
| TypeScript vs Rust gameplay | 6,550 frames and 4,270,031 field comparisons; maximum numeric difference 2.28e-13 |
| TypeScript vs Rust AI | 32,400 controller frames and 11,778,134 field comparisons; exact decisions/state/RNG, maximum geometry difference 5.69e-14 |
| Production AI adapter | 432 world scenarios, 5,184 updates; same-frame player-one shot visibility checked |
| Native vs Wasm / independent Wasm | 36 matchup/difficulty cases, 12,960 frames, 13,198,168 comparisons; maximum native/Wasm difference 1.82e-12; independent Wasm instances exact |
| Wasm ABI and repeated resets | Invalid inputs rejected without state changes; 1,000 resets with no linear-memory growth |
| Existing UQM and shared-relay regression suite | 115 tests passed |
| Patched development dependencies | Vite 7.3.7 / Vitest 4.1.11; 635 tests rerun successfully; full and runtime-only npm audits report zero findings (20 October 2026) |
| Real browser solo/local play | AI, morphing, all three ships, frost shield, slime, audio output/mute, responsive menu, and removed debug-kill keys passed |
| Real two-browser fleet match | 6,321 synchronized ticks, 212 verified checksums per peer, all three ships and four battle transitions; natural match finish, mutual rematch and disconnect passed |
| Real two-browser latency/jitter | Relay delays of 0/30/5/70/15 ms; both peers verified eight checksums; motion, firing and disconnect passed |

A compact machine-readable result record is in
[`cosmoclash/validation.json`](../cosmoclash/validation.json).

Validated gameplay Wasm SHA-256:
`de75c39465bbda999c89f4c8b0f750c3707024c28b4a629df5539b53aaeb714d`.

The fixed audio preload/decode race and editable-input keyboard handling have
regression coverage. Development dependencies are updated to patched Vite 7.3.7
and Vitest 4.1.11; Node types are an explicit locked dependency for clean builds.
Vitest 4 requires constructable function mocks. Two upstream test files receive
that mechanical adaptation; their exact originals remain under
`cosmoclash/reference/upstream-tests/`, with the original SHA-256 manifest
unchanged. Gameplay and AI source references remain unchanged. The production bundle builds successfully; Vite reports a
size advisory for the Three.js bundle (approximately 675 kB before gzip).

The existing general UQM workflow previously ran its C/Rust comparison tests
before the original build generated `config_unix.h`. The original build step is
now ordered before those comparisons. The four native oracle scripts also search
the source checkout root, where that header is generated, and their shared
fixture respects generated capability macros before supplying fallbacks. These
changes correct inherited clean-checkout prerequisites. The separate full UQM
workflow is not included in the Cosmo-specific validation totals above. The four
Original native oracle suites passed locally in both debug and release after
these fixes (1,576,637,346 comparisons across core, velocity/trigonometry,
queue/geometry and bounding boxes).

## Remaining release validation

Two local browser processes and simulated latency exercise the complete client
and relay, but do not establish behavior over a real public internet connection.
Before a public release, test the deployed HTTPS origin with two people on
separate networks, verify the reverse proxy's WebSocket timeout, and exercise
reconnection by creating a new room after a lost connection. Safari/Firefox,
touch-only play, mobile performance, and large public concurrency have not been
certified by the local Chrome run. This branch does not deploy or modify the live
UQM site.
