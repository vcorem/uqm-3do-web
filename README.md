# The Ur-Quan Masters in the browser

[![Build and test](https://github.com/vcorem/uqm-3do-web/actions/workflows/build.yml/badge.svg?branch=main)](https://github.com/vcorem/uqm-3do-web/actions/workflows/build.yml)
[![Deploy](https://github.com/vcorem/uqm-3do-web/actions/workflows/deploy.yml/badge.svg?branch=deploy%2Fcloudflare)](https://github.com/vcorem/uqm-3do-web/actions/workflows/deploy.yml)

**Play now: [urquanmasters.org](https://urquanmasters.org/).** There is nothing to
install; a desktop browser with a keyboard works best.

The Ur-Quan Masters (UQM) is the free, open-source version of Star Control II by
Toys for Bob. This repository compiles it to WebAssembly (it started from
[intgr/uqm-wasm](https://github.com/intgr/uqm-wasm), upstream commit
`daadbb540a8c46f09dcdb0080b4212fb33e6cb94`), with **Star Control II for the 3DO**
as its reference, and it runs the site urquanmasters.org: four games behind one
picker, with online battles through room lists inside the games.

## Play

| Game | Path | Ways to play |
| --- | --- | --- |
| **The Ur-Quan Masters, Original 3DO** | `/original/` (C), `/original-rust/` (Rust) | Campaign, local Melee, Online Melee with other Original players |
| **The Ur-Quan Masters, Redrawn HD** | `/hd/` (C), `/hd-rust/` (Rust) | Campaign, local Melee, Online Melee with other HD players |
| **[Melee Remastered](docs/melee-remastered.md)** | `/melee/` (C), `/melee-rust/` (Rust) | Super-Melee only, with new ship, crew and arena art and two new scores: local battles, Network Melee in the HD rooms |
| **[Cosmo Clash](cosmoclash/README.md)** by Ori Hanegby, Guy Corem and Orr Kowarsky | `/cosmoclash/` | Against the computer (Easy, Medium, Hard), two players on one keyboard, online duels |

- **Engines.** Each UQM game has the C engine (the reference) and a Rust engine:
  a C2Rust translation, published as a test version with its own saves. Online,
  C and Rust players of the same edition play each other.
- **Online.** Each game has its own room list inside the game: create a named
  room or pick one from the list. **Copy link** gives a direct link to a room
  that waits; the room stays in the list. The rooms of one game never pair with
  another game's, with one exception: Melee Remastered runs the HD engine and
  rules, so it uses the HD rooms, and HD and Melee Remastered players can fight
  each other (each sees their own art).
- **First visit.** The game data downloads once: about 300 MB for Original and
  660 MB for HD and Melee Remastered (they share one copy). The browser keeps it.
- **Saves.** Each game and engine keeps its own saves and preferences in the browser.
- **Keys.** Press **?** in a UQM game (**K** in Cosmo Clash) to show the keys.

**Melee Remastered** is the HD build published a second time with a Melee-only
page: two choices, **Melee** and **Network Melee**, without the logo, intro or
campaign menu. It has painted art for all 25 ships and crews, a dark nebula
arena, navy/cyan panels and two original combat scores; **Picture & sound**
switches between Remastered and Classic HD art at any time. The HD page keeps
its own art in Super-Melee.

**Cosmo Clash** is a separate 2D/3D space duel game with a Rust WebAssembly
gameplay engine; its source, assets, build and validation are in `cosmoclash/`,
and its hosting is in [the Cosmo Clash port notes](docs/cosmoclash-port.md#hosting).

## Status

The site has been live since 19 October 2026. CI (`.github/workflows/build.yml`)
builds the engines and runs the unit, network and real-browser tests on every
change, and the deploy repeats them before it publishes. Online matches between
two computers on different networks passed on the site on 19 and 20 October 2026
for Original and HD, with the C and Rust engines. Still open: the complete 3DO
fidelity checklist, the native Safari application, mobile and touch devices,
and gamepads.

The original ships, physics and collision artwork remain in place. The Rust
engines keep substantial `unsafe` code and the original C ABI; SDL, the codecs
and the HD Lua interpreter remain external dependencies. They are a translation,
not a memory-safe rewrite: see [Rust scope, build requirements and validation](docs/rust-port.md).
The browser launcher selects 3DO menus, fonts, scan icons, shields, scrolling
and melee zoom. The content includes 3DO music and speech, and the original 3DO
videos (intro, ending and all 25 ship presentations); see [3DO movies](docs/3do-videos.md)
and [compatibility scope and remaining checks](docs/3do-compatibility.md).

## Repository layout

| Folder | Contents |
| --- | --- |
| `sc2/` | UQM 0.8.0 C engine and its content; the browser page and room client in `sc2/wasm/` |
| `hd/` | UQM MegaMod HD build: patches, page (`hd/wasm/`), content lock, and the Melee Remastered art and music (`hd/remastered/`) |
| `rust/` | Rust engines (C2Rust translations) and the crates `uqm-core`, `uqm-png`, `uqm-lua`, `uqm-relay` |
| `cosmoclash/` | Cosmo Clash (TypeScript, Three.js and a Rust Wasm gameplay engine) |
| `web/` | site picker, in-game online lobby, game data loader, keys help |
| `server/` | Node game server and relay with room lists, for local play |
| `deploy/` | Cloudflare Worker and its Durable Object relay (the site) |
| `scripts/` | build, package and maintenance scripts |
| `tests/` | unit, server, Worker, browser and C/Rust comparison tests |
| `docs/` | documentation |
| `tools/`, `misc/`, `symbian/` | upstream UQM folders; the web build does not use them |

## Build locally

Requirements: Git, a POSIX shell, Make, Python 3.10 or newer for the SDK
installer, Node.js 22.12 or newer, and Rustup. macOS needs the Xcode
command-line tools. The HD build downloads its media (about 660 MB) on its first
run. The compiler and its caches live inside `.tools/emsdk`; no shell startup
changes are needed.

```sh
mkdir -p .tools
git clone https://github.com/emscripten-core/emsdk.git .tools/emsdk
.tools/emsdk/emsdk install 3.1.50
.tools/emsdk/emsdk activate 3.1.50
rustup toolchain install 1.85.1 --profile minimal --component rustfmt --component clippy
rustup toolchain install "$(cat .rust-wasm-toolchain)" --profile minimal --component rust-src --target wasm32-unknown-emscripten
rustup target add wasm32-unknown-unknown --toolchain 1.85.1   # Cosmo Clash
./scripts/build-web.sh
python3 scripts/serve.py
```

If the default Python is older, set `EMSDK_PYTHON` to a Python 3.10+ executable
when running the SDK installer. The SDK supplies its own runtime afterward.
The script configures browser dependencies, compiles the Rust engine and
relinks the page on every package build so launcher or storage edits cannot
leave stale output. Compiler outputs and downloaded ports are cached.
Use `UQM_CORE=c` for the C reference, or `UQM_CORE=hybrid` for the intermediate
core migration; local builds use the whole Rust game engine by default
(`UQM_CORE=rust`), while the site's main pages use C.
Use an absolute output path for a separate package, for example
`BUILD_OUTPUT="$PWD/.tools/netplay-dist" ./scripts/build-web.sh`.
Emscripten **3.1.50** is pinned in `.emscripten-version`; the build script
rejects other versions. The first build downloads and compiles SDL2,
libpng, Ogg/Vorbis and zlib.

Open **http://127.0.0.1:9999** and choose **Play** on the start screen. It works
while the game downloads and starts it when it is ready, with browser audio. Select Super Melee in the original menu.
The preview server binds to loopback only. Stop it with Ctrl-C.
Use `python3 scripts/serve.py --port 10000` for another port.

The generated `dist/` contains the HTML launcher, JavaScript, Wasm, worker,
content bundle, room client and license notices. Local play can be hosted as
static files with:

```text
Cross-Origin-Opener-Policy: same-origin
Cross-Origin-Embedder-Policy: require-corp
```

Serve `.wasm` as `application/wasm`. Use HTTPS outside localhost. Opening the
HTML as a file, or using a server without the isolation headers, will not
support the shared memory used by the game. The initial content bundle is
large because it includes the complete campaign voice recordings and installed
3DO videos. Online Melee additionally needs the same-origin Rust or Node relay at
`/netplay/`, including its `/netplay/hd/` and `/netplay/cosmoclash/` subtrees;
a static host alone is insufficient. The room lists need the Node relay (or
the Cloudflare relay); the Rust relay has private rooms only.

### The whole site

`./scripts/build-editions.sh` builds the whole site: the picker with its four
cards, the C and Rust engines of both editions, Melee Remastered (the HD builds
again, with the Melee page) and Cosmo Clash (`UQM_COSMO=0` leaves it out). Only
the selected game downloads. See [HD build, storage and attribution](docs/hd-edition.md).

The browser keeps each edition's game data after the first visit
([`web/engine-loader`](web/engine-loader/engine-loader.js)). A later visit asks
the server only for the data file's version (its ETag) and starts from the
stored copy; the C and Rust engines of an edition share it, and the HD and
Melee Remastered pages share the HD copy. A new version
replaces the old copy. `scripts/serve.py` and the Cloudflare Worker send the
ETag. Private windows, browsers without Cache Storage and servers without an
ETag download the data on every visit, as before. To free the space (about
300 MB for Original, 660 MB for HD and Melee) and keep the saved games, remove only the
site's Cache Storage (in the browser's developer tools, Application or Storage,
Cache Storage, `uqm-game-data-original` / `uqm-game-data-hd`), or run
`UQMEngineLoader.removeStoredData()` in the console of a game page. Clearing
all site data also deletes the saved games.

On macOS, after a build, double-click **Play UQM.command**. It serves the built
site; choose a card and **Play**. Keep the terminal window open and stop the
server with Ctrl-C. In a ship selection menu, press **Alt / Option** for the
ship video.

### Online Melee locally

Start the game and room server:

```sh
npm ci
npm start -- --port 10000
```

`npm start` runs the Node server, which also has the room lists of Online
Melee. `npm run start:rust` runs the Rust relay with the pinned Rust toolchain
described above; it has private rooms only, and its first start compiles it.

Open **http://127.0.0.1:10000/** and choose a game if using the combined
package. On the start screen inside the game frame, choose **Online Melee**
(the frame goes full screen). Choose **Create room**, give the room a name and
wait. The other player chooses **Online Melee** in a second browser page, sees
the room in the live list and picks it with the arrows and Enter (or a click).
After a short countdown the Melee screen opens on both sides, still in full
screen. Configure the fleets, confirm Start Melee on both sides and choose
ships. The host controls the bottom fleet; the guest controls the top fleet.
Both use the player-one keyboard template; HD uses the arrow-key controls.
**Copy link** on the waiting screen copies a direct link to the same room: it
opens the lobby and joins after one click. The room stays in the public list
too, and the first player who joins gets it. A room opens, and a picked room
is joined, only when that page's game has loaded, so nobody waits for the
other player's download. From the game's main menu, the **Online Melee**
corner button or the **O** key opens the lobby again.
See [the in-frame start screen and lobby](docs/online-ui.md).

Original rooms connect to `/netplay/<room>`, HD and Melee Remastered rooms to
`/netplay/hd/<room>` and Cosmo Clash rooms to `/netplay/cosmoclash/<room>`. One
relay serves all three, with separate room identities, separate room lists and
shared capacity limits. A matching room code cannot connect different editions
or games.

Rooms use the original UQM netplay protocol and simulation over an ordered
WebSocket relay, with a three-frame default input delay. Browser callbacks
queue network bytes; the game reads them during its normal input polling.
The Diagnostics panel shows the connection, engine state and simulation
checksum counts. Leaving or losing either connection ends the room. With an
engine that has the main-menu hook, the lobby comes back by itself for the
next match; an engine without it offers a reload, which opens the lobby
again. **Play** on the start screen opens the campaign and local Melee
normally.

The invite contains the room code in its URL fragment. Anyone with it can
take the guest slot while the room waits, as can anyone who picks the room
in the list. A localhost invite works only on that computer. Two-computer play
requires a shared HTTPS server; see [relay setup and limits](docs/netplay-server.md).
The Python preview and legacy nginx Docker image serve local play only;
they do not run the room relay.

## Saving and controls

Saves and preferences use IndexedDB on the current browser origin. Keep the
same hostname and port to access the same saves. Clearing site data removes
them. Each page mounts its own folder: Original `/home/web_user/.uqm`, HD
`/home/web_user/.uqm-hd`, the Rust pages `.uqm-rust` and `.uqm-hd-rust`, and
Melee Remastered `.uqm-melee` and `.uqm-melee-rust`. Switching editions or
starting an online session does not convert campaign saves between engines.
Browser persistence is requested on the first write; storage is still subject
to the browser's policy. A storage failure is shown in the launcher; if initial
loading fails, the game can run in memory without overwriting existing database
saves. Writes are serialized to avoid overlapping flushes.

Menus: arrow keys, Enter, Esc. Default player-one combat: Up for thrust,
Left/Right to turn, Right Ctrl or Enter to fire, Right Shift for the special.
The in-game input settings allow other keyboard templates and controllers.
Gamepad behavior still requires browser testing. Full screen is available
from the launcher; click the game canvas to return keyboard focus.

## Tests and CI

```sh
npm ci
npm test
# Build first: the harness reads the generated platform configuration.
./scripts/build-web.sh
bash sc2/src/uqm/supermelee/netplay/tests/run.sh
npx playwright install chromium
npm run test:browser
npm run test:subtitles
bash -n scripts/build-web.sh
python3 -m py_compile scripts/serve.py
# Optional redrawn HD engine, online, campaign and edition-switch checks:
./scripts/build-hd-web.sh
UQM_EDITION=hd UQM_RELAY=rust UQM_DIST=.tools/hd-dist \
  UQM_TEST_OUTPUT=test-results/hd-netplay npm run test:browser
UQM_DIST=.tools/hd-dist npm run test:hd-campaign
UQM_DIST=.tools/hd-dist npm run test:hd-stability
python3 scripts/package-editions.py --original dist --hd .tools/hd-dist --output .tools/editions-dist
# Optional Cosmo Clash at /cosmoclash/ behind the picker (test:cosmoclash-site needs no UQM build):
npm ci --prefix cosmoclash && npm run build --prefix cosmoclash -- --base /cosmoclash/
npm run test:cosmoclash-site
python3 scripts/package-editions.py --original dist --hd .tools/hd-dist --cosmoclash cosmoclash/dist --output .tools/editions-dist
npm run test:worker
UQM_DIST=.tools/editions-dist npm run test:editions
```

Browser smoke checks and their limitations are recorded in
[the first validation record](docs/validation.md) and
[the Rust validation record](docs/rust-validation.md). A passing build or one
playable battle does not establish exact 3DO fidelity or complete campaign
coverage. The upstream fork lists missing Firefox audio samples and unsupported
SDL mixing/threading paths; this build retains OpenAL and pthreads.

`npm test` covers browser storage/video setup, room client and launch behavior,
and the real WebSocket relay. The separate Wasm harness compiles the actual
transport, packet handlers and disconnect waits; it exercises direct and
pthread transport access, fragmented/coalesced protocol data, malformed setup
packets and disconnects during setup/handshake/reset/battle transitions.

`npm run test:browser` starts its own loopback relay and two independent
browser processes against the built game (Chromium by default). It exercises
full fleet matches,
ship transitions, simulated delay/jitter, disconnects and preservation of local
fleet settings. It writes diagnostics and screenshots to the ignored
`test-results/browser/` directory. Use `UQM_DIST=/absolute/path/to/package` to
test a separate build. Set `UQM_EDITION=hd` for an HD package; the default is
`original`. Both peers must use the same edition. `UQM_PEER_DIST` can supply an
independent C build of that edition for comparison with Rust; see
[Rust verification](docs/rust-port.md#verification). `UQM_MELEE_ONLY=1` opens
the Melee Remastered page, and `UQM_GUEST_MELEE=1` plays an HD host against a
Melee Remastered guest.

To exercise other engines or a mixed pair, install the matching Playwright
browsers and choose the host and guest separately:

```sh
npx playwright install firefox webkit
UQM_BROWSER=firefox UQM_TEST_OUTPUT=test-results/firefox npm run test:browser
UQM_BROWSER=chromium UQM_PEER_BROWSER=webkit UQM_TEST_OUTPUT=test-results/mixed npm run test:browser
```

`UQM_PEER_BROWSER` defaults to `UQM_BROWSER`. `CHROME_BIN` can select an
installed Chrome executable for Chromium peers. Each result records both
engines and versions. Playwright WebKit tests the engine used by Safari; it
does not test the installed Safari application.

`npm run test:subtitles` loads a normal campaign save near the Earth starbase
in fresh Chromium sessions. It checks actual voiced dialogue and changing
subtitle pixels with the launcher override, then removes the override to verify
that the saved disabled preference hides text while speech keeps playing.
Use `UQM_DIST`, `UQM_TEST_OUTPUT` and `CHROME_BIN` as above.

GitHub Actions runs four workflows, all with read-only repository permissions:

- `build.yml`: the unit and network checks, both pinned-toolchain engine builds
  in C and Rust, the real-browser Melee scenarios on the HD and Melee pages, HD
  campaign travel, five-minute stability and edition/save-isolation checks.
- `cosmoclash.yml`: the Cosmo Clash build, its tests and the Worker tests.
- `deploy.yml`: builds and tests the whole site, then publishes it (see below).
- `remote-peer.yml`: one side of a match between two networks, for live checks.

## Deployment

The site runs on Cloudflare Workers: `deploy/worker.js` serves the static files
(it joins the large data files from parts) and `deploy/netplay.js` is the relay
with its room lists (Durable Objects). A push to the branch `deploy/cloudflare`
runs `deploy.yml`, which builds every engine, runs the tests, splits the data
files (`deploy/split-data.py`), deploys with Wrangler and checks the live site.
It needs the repository secrets `CLOUDFLARE_API_TOKEN` and
`CLOUDFLARE_ACCOUNT_ID`. A fork must change the routes in
`deploy/wrangler.jsonc` and the allowed origins (`NETPLAY_ORIGINS`).

## Documentation

- [Melee Remastered](docs/melee-remastered.md), [HD edition](docs/hd-edition.md),
  [3DO compatibility](docs/3do-compatibility.md), [3DO movies](docs/3do-videos.md)
- [Relay and room lists](docs/netplay-server.md), [in-game start screen and lobby](docs/online-ui.md),
  [engine netplay hook](docs/engine-netplay-hook.md)
- [Rust port](docs/rust-port.md), [Rust validation](docs/rust-validation.md),
  [first validation record](docs/validation.md)
- [Cosmo Clash port and hosting](docs/cosmoclash-port.md)
- [Licenses](docs/licenses.md), [README of intgr/uqm-wasm](docs/upstream-readme.md)

## Contributing

Issues and pull requests are welcome. Run `npm test` and the checks of the part
you change (see [Tests and CI](#tests-and-ci)). Change the C engine in `sc2/src`
or in `hd/patches`, then regenerate the Rust translation with the pinned C2Rust
pipeline ([Rust port](docs/rust-port.md)); the translation locks in
`scripts/rust-engine/` must match. The imported Cosmo Clash sources and assets
are hash-locked by `npm run test:provenance --prefix cosmoclash`.

To report a security problem, use the private vulnerability reporting of this
repository (the **Security** tab) instead of a public issue.

## Credits

- Star Control II: Fred Ford, Paul Reiche III and Toys for Bob. The Ur-Quan
  Masters: the UQM team ([full credits](sc2/AUTHORS)).
- The first WebAssembly port: Marti Raudsepp ([intgr/uqm-wasm](https://github.com/intgr/uqm-wasm)).
- UQM HD MegaMod: Serosis and contributors ([JHGuitarFreak/UQM-MegaMod](https://github.com/JHGuitarFreak/UQM-MegaMod)),
  and the HD artists listed in its `ATTRIBUTION.txt` (`content-licenses.txt` on the HD pages).
- 3DO videos: Crystal Dynamics. 3DO music and speech: Toys for Bob.
- Rust translation: [C2Rust](https://github.com/immunant/c2rust).
  WebAssembly toolchain: [Emscripten](https://emscripten.org/).
- Melee Remastered art and music: made for this project with AI assistance
  ([credits](hd/remastered/credits.md)).
- Cosmo Clash: Ori Hanegby, Guy Corem and Orr Kowarsky.
- This web edition, its online play and the site: Guy Corem.

## License

The UQM code is GPL-2.0-or-later, and each compiled engine as a whole is
GPL-2.0-only, because some bundled files are GPL-2.0-only ([`COPYING`](COPYING)
is the GPL text). The project's own code is GPL-2.0-or-later. The UQM content
is CC BY-NC-SA 2.5, with the UQM distribution clause in
[`sc2/COPYING`](sc2/COPYING). The HD content, the 3DO videos, the Melee
Remastered art and music, and Cosmo Clash have their own terms: see
[Licenses](docs/licenses.md) for each part, and `THIRD-PARTY-NOTICES.txt` on the
site for the libraries in the WebAssembly builds.

This project is not affiliated with or endorsed by Toys for Bob or the owner of
the Star Control trademark.
