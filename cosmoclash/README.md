# Cosmo Clash — Rust WebAssembly and Online Melee

A separate game inside the UQM repository: the third game of the site, at
`urquanmasters.org/cosmoclash/`. The original Cosmo Clash models, textures,
sounds, three ships, AI difficulties and local gauntlet are retained. Online
fleet battles use the site's relay with a separate room namespace and room list.

The simulation runs in Rust compiled to WebAssembly: ship movement, weapons,
morphing, frost shields, slime, collisions, gravity, wrap-around, camera tracking,
entity lifetime, and AI. Three.js renders the original artwork; TypeScript hosts
browser input/audio, menus, fleet transitions, and the WebSocket connection.
The original TypeScript gameplay classes remain as independent test references;
the production battle loop does not invoke them.

## Build and play locally

Requirements: Node.js 22.12+, Python 3, Bash, and Rust 1.85.1. From the repository root:

```bash
npm ci
npm ci --prefix cosmoclash
rustup target add wasm32-unknown-unknown --toolchain 1.85.1
npm run build --prefix cosmoclash
npm start --prefix cosmoclash
```

Open `http://127.0.0.1:10000/cosmoclash/` (the address `/` goes there too).
`npm start` serves the build at `/cosmoclash/`, as on the site, and online play at
`/netplay/` on the same host: the site's Node relay (`server/netplay.mjs`) with the
Cosmo Clash room list. Options: `-- --port 10000 --host 127.0.0.1 --relay node|rust`.
`--relay rust` uses the Rust relay, which has no room list: the game then offers
private rooms with an invite link. To play online on one computer, open the
address in two browser windows.

The main menu has **Play vs Computer**, **Two Players · Same Computer** and
**Online Melee**. Arrows choose, Enter or a click starts; a choice puts the game
in full screen, and `F` (or the corner button of the main menu) turns full screen
on or off. Fly with the arrow keys, `/` fires, `.` is the special ability, `M`
mutes. The second local player uses W/A/D, Space and Shift (so in Two Players only
Enter chooses in the menus). Both online players use the first player's keys on
their own computer. Esc pauses a local battle and asks before it leaves an online
match or a game against the computer.

`K` (or the **Keys (K)** button) shows or hides the keys in every screen, also
during a battle; nothing pauses. When the window is wider than the 4:3 game
(for example 1280x720, 1920x1080, also in full screen), the keys show at first
in the dark margins beside the game, never over it: the players' keys and the
ships on the left, the menus and the game keys on the right; the game remembers
when the player hides them. On a 4:3 window or a phone they open over the
game, and `K`, Esc or Close closes them. Two Players shows the keys of both
players; vs Computer and Online show the first player's keys. The UQM pages use
`?` for their keys, but here `?` is Shift + `/`, two fire keys of a Two Players
battle.

Every game is a fleet match: the first ship of a fleet opens the match, and
after each battle the loser chooses the next ship from the ships left (the
winner's ship stays). The last ship comes by itself, and a fleet without ships
loses the match. Against the computer both sides have all three ships; in **Two
Players** Player 2 and then Player 1 choose a fleet of one to three ships.

**Online Melee** shows the open rooms. Pick one to play, or **Create room**, give it
a name and wait: the match starts when someone joins (**Copy invite link** is there
too). Each player then picks one to three ships in the 3D ship choice; the first
pick opens the match. **Ready** when the fleet is complete (**Change fleet**
takes it back while the opponent still chooses). After a loss with two ships
left the loser has 20 seconds to choose the next ship (then the first ship left
comes) while the opponent waits; a destroyed fleet ends the match. **Rematch**
needs both players. All screens are in the game; see [docs/online.md](docs/online.md).

For development, run `npm start --prefix cosmoclash` (the relay and the room
list on port 10000) and `npm run dev --prefix cosmoclash`; open
`http://127.0.0.1:5173/cosmoclash/`. Vite sends `/netplay/` to port 10000.

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
upstream test suite and compares real TypeScript gameplay and AI with Rust. It
also tests the room-name rule (the shared UQM cases), the room list client, the
room client, the online screens' state machine and view, and the room list
through the real Node and Rust relays.
Wasm tests compare independent Wasm instances and native Rust, and reject invalid
ABI requests. Browser tests use independent browser processes and real relays,
with real keyboard and mouse input, state checksums and asset loading:
`tests/browser/solo.mjs` (local play and the local fleet matches), `tests/browser/game.mjs`
(full fleet match with chosen next ships, rematch, jitter and a disconnect during
a choice through the Rust relay with an invite link) and
`tests/browser/online-ui.mjs` (`npm run test:online-ui`: the in-game lobby with
the room list, saving every screen at 1280x720, 1024x768 and 1920x1080 in
`test-results/online-ui/`; `solo.mjs` saves the local screens in
`test-results/local-ui/`; the screens of the fleet matches and the choice of
the next ship are also in `test-results/next-ship/`) and
`tests/browser/keys-help.mjs` (`npm run test:keys-help`: the keys beside and
over the game in the menu, the local and online battles, the lobby and the fleet
choice, also in full screen, in `test-results/keys-help/`).
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
`CLAUDE.md` and `docs/` (except `docs/online.md`) are historical upstream
documentation and can describe its previous TypeScript implementation. The
executable source and comparison tests, rather than older prose values, define
the behavior of this port.
