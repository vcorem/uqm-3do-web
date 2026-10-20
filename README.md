# The Ur-Quan Masters — 3DO Web Edition

This branch also includes [Cosmo Clash](cosmoclash/README.md), with a Rust
WebAssembly game engine and private online fleet battles. Its source, assets,
build, and validation are kept in the `cosmoclash/` directory.

A local WebAssembly port of UQM 0.8.0, based on
[intgr/uqm-wasm](https://github.com/intgr/uqm-wasm), upstream commit
`daadbb540a8c46f09dcdb0080b4212fb33e6cb94` (2023-12-10).
The reference target is **Star Control II for the 3DO**.

**Rust port branch (`codex/rust-port`).** Both Original and HD game engines
build from Rust implementations, alongside a native Rust network relay.
The engine translation retains substantial `unsafe` code and the original C ABI;
SDL, codecs and the HD Lua interpreter remain external dependencies. JavaScript
handles browser APIs and the launcher. This is not a memory-safe rewrite.
See [Rust scope, build requirements and validation](docs/rust-port.md).

**Status: private alpha.** The campaign, local Melee and private two-player
Online Melee run in the browser. Original and HD online combat have been exercised in two
independent browser processes on one computer, including full fleet matches,
rematches, simulated delay and disconnect recovery, with matching simulation
checksums and no observed mismatch. Two separate computers over the internet
and the complete 3DO fidelity checklist are still unverified. No public
release or deployment of this Rust branch has been made.

The original ships, physics and artwork remain in place. On this branch,
the game implementations compile from Rust to WebAssembly.
The browser launcher selects 3DO menus, fonts, scan icons, shields, scrolling
and melee zoom. The checkout includes 3DO music and speech. Original 3DO
videos are included in the repository content: intro, ending and all 25 ship
presentations. Their package source and attribution are recorded separately.
The launcher automatically enables the original videos when all required files
are installed; see [3DO movies](docs/3do-videos.md).
See [compatibility scope and remaining checks](docs/3do-compatibility.md).

## Choose Original or Redrawn HD

An optional combined build opens with an edition picker: **Original 3DO** or
**Redrawn HD**, using actual 1280×960 artwork from UQM MegaMod 0.8.5. Only the
selected edition downloads. Both editions support campaign, local Melee and
Online Melee with another player using the same edition. HD has separate saves
and rooms; HD and Original cannot play one another.
See [HD build, storage and attribution](docs/hd-edition.md).

Build the selection screen and both editions with `./scripts/build-editions.sh`.
The first HD download is about 640 MB; the original artwork and HD artwork
use separate engines and save databases.

The browser keeps each edition's game data after the first visit
([`web/engine-loader`](web/engine-loader/engine-loader.js)). A later visit asks
the server only for the data file's version (its ETag) and starts from the
stored copy; the C and Rust engines of an edition share it. A new version
replaces the old copy. `scripts/serve.py` and the Cloudflare Worker send the
ETag. Private windows, browsers without Cache Storage and servers without an
ETag download the data on every visit, as before. To free the space (about
300 MB for Original, 640 MB for HD) and keep the saved games, remove only the
site's Cache Storage (in the browser's developer tools, Application or Storage,
Cache Storage, `uqm-game-data-original` / `uqm-game-data-hd`), or run
`UQMEngineLoader.removeStoredData()` in the console of a game page. Clearing
all site data also deletes the saved games.

## Play the installed local game

On macOS, double-click **Play UQM.command**. Keep the terminal window open,
then choose **Original 3DO** or **Redrawn HD** and launch the selected game.
Stop the server with Ctrl-C.
The installed local build includes the full campaign, original music and
speech with dialogue subtitles, and the 3DO intro, ending and 25 ship videos.
Saves stay in this browser.
In a ship selection menu, press **Alt / Option** for the ship video.

## Play Online Melee locally

Use Node.js 22 or newer. After building the game, start the game and room
server on a separate port so an existing campaign preview can stay open:

```sh
npm ci
npm start -- --port 10000
```

`npm start` runs the Node server, which also has the room lists of Online
Melee. `npm run start:rust` runs the Rust relay with the pinned Rust toolchain
described below; it has private rooms only, and its first start compiles it.

Open **http://127.0.0.1:10000/** and choose an edition if using the combined
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

Original rooms connect to `/netplay/<room>` and HD rooms connect to
`/netplay/hd/<room>`. One relay serves both, with separate room identities and
shared capacity limits. A matching room code cannot connect different editions.

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
in the list. A
localhost invite works only on that computer. Two-computer play requires a
shared HTTPS server; see [relay setup and limits](docs/netplay-server.md).
The Python preview and legacy nginx Docker image serve local play only;
they do not run the room relay.

## Build locally

Requirements: Git, Python 3.10 or newer for the current SDK installer, a POSIX
shell, Make and Rustup. macOS needs the Xcode command-line tools. The compiler and
its caches live inside `.tools/emsdk`; no shell startup changes are needed.

```sh
mkdir -p .tools
git clone https://github.com/emscripten-core/emsdk.git .tools/emsdk
.tools/emsdk/emsdk install 3.1.50
.tools/emsdk/emsdk activate 3.1.50
rustup toolchain install 1.85.1 --profile minimal --component rustfmt --component clippy
rustup toolchain install "$(cat .rust-wasm-toolchain)" --profile minimal --component rust-src --target wasm32-unknown-emscripten
./scripts/build-web.sh
python3 scripts/serve.py
```

If the default Python is older, set `EMSDK_PYTHON` to a Python 3.10+ executable
when running the SDK installer. The SDK supplies its own runtime afterward.
The script configures browser dependencies, compiles the Rust engine and
relinks the page on every package build so launcher or storage edits cannot
leave stale output. Compiler outputs and downloaded ports are cached.
Use `UQM_CORE=c` for the C reference, or `UQM_CORE=hybrid` for the intermediate
core migration; default `UQM_CORE=rust` uses the whole Rust game engine.
To keep a separate preview package, use an absolute output path, for example
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
`/netplay/`, including its `/netplay/hd/` subtree; a static host alone is
insufficient.

## Saving and controls

Saves and preferences use IndexedDB on the current browser origin. Keep the
same hostname and port to access the same saves. Clearing site data removes
them. Original mounts `/home/web_user/.uqm`; HD mounts
`/home/web_user/.uqm-hd`. Switching editions or starting an online session does
not convert campaign saves between engines. Browser persistence is requested
on the first write; storage is still
subject to the browser's policy. A storage failure is shown in the launcher;
if initial loading fails, the game can run in memory without overwriting
existing database saves. Writes are serialized to avoid overlapping flushes.

Menus: arrow keys, Enter, Esc. Default player-one combat: Up for thrust,
Left/Right to turn, Right Ctrl or Enter to fire, Right Shift for the special.
The in-game input settings allow other keyboard templates and controllers.
Gamepad behavior still requires browser testing. Full screen is available
from the launcher; click the game canvas to return keyboard focus.

## Checks and current scope

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
UQM_DIST=.tools/editions-dist npm run test:editions
```

Browser smoke checks and their limitations are recorded separately in
[the original validation record](docs/validation.md) and
[the Rust validation record](docs/rust-validation.md). The latter records
which Rust Original and HD checks have completed. A passing build or one playable
battle does not establish exact 3DO fidelity or complete campaign coverage.
The upstream fork lists missing Firefox audio samples and unsupported SDL
mixing/threading paths; this build retains OpenAL and pthreads.

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
test a separate build. These are automated regression scenarios; the
validation record distinguishes completed runs from remaining checks.
Set `UQM_EDITION=hd` for an HD package; the default is `original`. Both peers
must use the same edition. `UQM_PEER_DIST` can supply an independent C build of
that edition for comparison with Rust; see [Rust verification](docs/rust-port.md#verification).

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
Use `UQM_DIST`, `UQM_TEST_OUTPUT` and `CHROME_BIN` as above. This local desktop
regression uses timed navigation and runs in CI alongside the Rust engine checks.

GitHub Actions runs the unit and network checks, including the real-browser
Melee scenarios, both pinned-toolchain engine builds, HD campaign travel,
five-minute HD stability and edition/save-isolation checks.
The workflow has read-only repository permissions and does not publish a
package, container or website. Remaining release checks are recorded in
[the validation record](docs/validation.md).

## Credits and licenses

Original game by Fred Ford, Paul Reiche III and Toys for Bob. Modern port by
the UQM team; original WebAssembly port by Marti Raudsepp. Preserve upstream
history and [full credits](sc2/AUTHORS).

Code is GPL-2.0-or-later. UQM base content, music and speech are CC BY-NC-SA
2.5, with additional UQM distribution language in [COPYING](sc2/COPYING).
Documentation has its own license there. The separately sourced 3DO movie
package has its [provenance and package attribution](docs/3do-video-source.json)
recorded separately; it is not assigned the base-content license by this project.

[Original fork README](docs/upstream-readme.md)
