# The Ur-Quan Masters — 3DO Web Edition

A local WebAssembly port of UQM 0.8.0, based on
[intgr/uqm-wasm](https://github.com/intgr/uqm-wasm), upstream commit
`daadbb540a8c46f09dcdb0080b4212fb33e6cb94` (2023-12-10).
The reference target is **Star Control II for the 3DO**.

The original C engine, ships, physics and game artwork remain in place.
The browser launcher selects 3DO menus, fonts, scan icons, shields, scrolling
and melee zoom. The checkout includes 3DO music and speech. Original 3DO
videos are included in the repository content: intro, ending and all 25 ship
presentations. Their package source and attribution are recorded separately.
The launcher automatically enables the original videos when all required files
are installed; see [3DO movies](docs/3do-videos.md).
See [compatibility scope and remaining checks](docs/3do-compatibility.md).

## Play the installed local game

On macOS, double-click **Play UQM.command**. Keep the terminal window open,
then click **Launch game** in the browser. Stop the server with Ctrl-C.
The installed local build includes the full campaign, original music and
speech with dialogue subtitles, and the 3DO intro, ending and 25 ship videos.
Saves stay in this browser.
In a ship selection menu, press **Alt / Option** for the ship video.

## Build locally

Requirements: Git, Python 3.10 or newer for the current SDK installer, a POSIX
shell and Make. macOS needs the Xcode command-line tools. The compiler and
its caches live inside `.tools/emsdk`; no shell startup changes are needed.

```sh
mkdir -p .tools
git clone https://github.com/emscripten-core/emsdk.git .tools/emsdk
.tools/emsdk/emsdk install 3.1.50
.tools/emsdk/emsdk activate 3.1.50
./scripts/build-web.sh
python3 scripts/serve.py
```

If the default Python is older, set `EMSDK_PYTHON` to a Python 3.10+ executable
when running the SDK installer. The SDK supplies its own runtime afterward.
Use `JOBS=8 ./scripts/build-web.sh` to change build concurrency.
Emscripten **3.1.50** is pinned in `.emscripten-version`; the build script
rejects other versions. The first build downloads and compiles SDL2,
libpng, Ogg/Vorbis and zlib.

Open **http://127.0.0.1:9999** and wait for **Launch game**. Clicking it enables
the game and browser audio. Select Super Melee in the original menu.
The preview server binds to loopback only. Stop it with Ctrl-C.
Use `python3 scripts/serve.py --port 10000` for another port.

The generated `dist/` contains the HTML launcher, JavaScript, Wasm, worker,
content bundle and license notices. It can be hosted as static files with:

```text
Cross-Origin-Opener-Policy: same-origin
Cross-Origin-Embedder-Policy: require-corp
```

Serve `.wasm` as `application/wasm`. Use HTTPS outside localhost. Opening the
HTML as a file, or using a server without the isolation headers, will not
support the shared memory used by the game. The initial content bundle is
large because it includes the complete campaign voice recordings.

## Saving and controls

Saves and preferences use IndexedDB on the current browser origin. Keep the
same hostname and port to access the same saves. Clearing site data removes
them. Browser persistence is requested on the first write; storage is still
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
node --test sc2/wasm/tests/*.test.cjs
bash -n scripts/build-web.sh
python3 -m py_compile scripts/serve.py
```

Browser smoke checks and their limitations are recorded separately in
[the validation record](docs/validation.md). A passing build or one playable
battle does not establish exact 3DO fidelity or complete campaign coverage.
The upstream fork lists missing Firefox audio samples and unsupported SDL
mixing/threading paths; this build retains OpenAL and pthreads.

Network SuperMelee is disabled in the browser build. A later milestone will
adapt the existing UQM network simulation to browser transport and two-player
rooms. No multiplayer service or public deployment is included yet.

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
