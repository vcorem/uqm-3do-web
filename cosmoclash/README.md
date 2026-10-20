# Cosmo Clash — Rust WebAssembly and Online Melee

A separate game inside the UQM repository, on branch `codex/cosmoclash-rust-netplay`.
The original Cosmo Clash models, textures, sounds, three ships, AI difficulties,
and local gauntlet are retained. Private online fleet battles use the existing
Rust relay with a separate room namespace.

The simulation runs in Rust compiled to WebAssembly: ship movement, weapons,
morphing, frost shields, slime, collisions, gravity, wrap-around, camera tracking,
entity lifetime, and AI. Three.js renders the original artwork; TypeScript hosts
browser input/audio, menus, fleet transitions, and the WebSocket connection.
The original TypeScript gameplay classes remain as independent test references;
the production battle loop does not invoke them.

## Build and play

Requirements: Node.js 22.12+, Python 3, Bash, and Rust 1.85.1. From the repository root:

```bash
npm ci
npm ci --prefix cosmoclash
rustup target add wasm32-unknown-unknown --toolchain 1.85.1
npm run build --prefix cosmoclash
npm start --prefix cosmoclash -- --port 10000
```

Open `http://127.0.0.1:10000`. This port keeps the existing UQM preview separate.
The game has **Play vs Computer**, **Two Players · Same Computer**, and
**Online Melee** modes. Use the arrow keys to fly, `/` to fire, `.` for the special
ability, and `M` to mute. The second local player uses W/A/D, Space, and Shift.
Both online players use the first player's controls on their own computer.

For online play, create a room, share its invitation, and have your friend click
**Join room**. Select one to three different ships; the order shown is the order
of battle. Both players click **Ready**. Losing a ship advances that player's
fleet. Ships reset for each battle, matching the local round reset behavior.
A fleet defeat offers a mutually accepted rematch. Leaving closes that room;
create a new room to reconnect.

For development, `npm run dev --prefix cosmoclash` provides Vite. The production
server above also serves the WebSocket relay; the Vite development server needs
its proxy configuration and a separate relay process.

## Verify

```bash
npm run test:provenance --prefix cosmoclash
npm run test:rust --prefix cosmoclash
npm test --prefix cosmoclash
npm run build --prefix cosmoclash
npm run test:wasm --prefix cosmoclash
npm test
npx playwright install chromium
npm run test:browser --prefix cosmoclash
```

`npm test --prefix cosmoclash` builds the native Rust JSONL test driver, runs the
upstream test suite and compares real TypeScript gameplay and AI with Rust.
Wasm tests compare independent Wasm instances and native Rust, and reject invalid
ABI requests. Browser tests use two independent browser processes and the actual
Rust relay, with real keyboard controls, state checksums and asset loading.
On macOS the browser runner defaults to installed Google Chrome; set `CHROME_BIN`
to select another executable. Linux uses Playwright Chromium.

Raw reports and screenshots are written under `test-results/` and ignored by Git.
See [the port and hosting handover](../docs/cosmoclash-port.md) for verified scope,
network requirements, and remaining validation.

## Upstream and attribution

Imported from [ohanegby/cosmoclash-web](https://github.com/ohanegby/cosmoclash-web),
commit `966ad7b7ac90c44d3751dc014fbdc1ae03359e07`.
Original game: **Ori Hanegby, Guy Corem, and Orr Kowarsky**.
The upstream package declares **ISC**. Existing UQM code and its relay retain
their own licenses; this import does not change them.

The original [README](reference/UPSTREAM-README.md), entry point, and a SHA-256
manifest of every imported source and asset are preserved in `reference/`.
`CLAUDE.md` and `docs/` are historical upstream documentation and can describe
its previous TypeScript implementation. The executable source and comparison
tests, rather than older prose values, define the behavior of this port.
