# Local validation — 2026-10-18

Environment: macOS arm64, Emscripten 3.1.50, SDL2 2.24.2 from the Emscripten
port cache. Browser: Codex in-app browser, served from `http://127.0.0.1:9999`.
The browser engine/version was not separately recorded. Debug build with
OpenAL, pthreads and Asyncify. This is an initial desktop browser smoke test.

## Passed

- Compile the original C game and package the Wasm, JavaScript, worker, complete
  content bundle and license file. The initial content bundle was 147,715,790 bytes; the local build with
  installed videos is now 298,763,132 bytes.
- HTTP 200 for the launcher and Wasm, `application/wasm` MIME type and both
  required cross-origin isolation headers.
- Browser loading completes, saved-data initialization finishes and the
  explicit Launch game action starts the engine without a captured console error.
- Original title and main menu render; keyboard input selects menu entries.
- Start the campaign, skip the interim PC intro and render the Sol system.
- Save a new campaign to previously empty slot 00, named `web`. Wait for the
  browser storage completion message, reload the entire page, select Load Game,
  see the same slot/name and restore the Sol game state. The test save remains
  on this browser origin; no pre-existing slot was replaced.
- Enter Super Melee, render both fleets and start local combat against the AI.
  Observe Syreen versus Zoq-Fot-Pik, return to ship selection after destruction,
  select a Pkunk and render a second combat. This verifies startup and transitions,
  not combat balance, frame accuracy or all ship abilities.
- Quit from the original menu, skip the credits through its confirmation,
  and display the browser launcher message that the game closed normally.
- Seven storage regression tests pass: startup success, asynchronous/synchronous
  failures, serialized/coalesced writes, repeated failures and recovery,
  console warning deduplication and a failing UI callback.
- Shell syntax, Python server syntax, package presence/Wasm signature and
  `git diff --check` pass.

## Evidence

![Campaign restored after page reload](screenshots/campaign-reloaded.jpg)

![Local Melee combat](screenshots/melee-combat.jpg)

## Still unverified

- Exact 3DO parity, all ships and the full campaign; see the
  [compatibility checklist](3do-compatibility.md).
- Audible output quality and latency, speech synchronization and complete ending
  playback. Opening/ship video smoke checks are recorded below; an audible
  comparison was not performed.
- Firefox, Safari, standalone Chrome, mobile browsers, gamepads, two humans on
  one keyboard, full-screen transitions and background-tab behavior.
- Browser-storage quota failure in a real browser (covered with unit harnesses),
  long-running stability and performance/latency measurements.
- Docker build; the local native SDK build is the validated path.
- Two-computer online play and public hosting remain unverified. Local online
  alpha checks are recorded below.

## Optional video activation update

The launcher detects all 134 required movie/audio files before engine startup.
Four additional regression tests cover metadata-only content, a complete set
derived from the actual resource map, each individually missing file, and
empty files/directories. The later local installation includes the original movie pack; details follow.

## Local original movie pack — 2026-10-18

- Downloaded the MegaMod 3DO Video Pack to the ignored local cache and checked
  ZIP CRC integrity and SHA-256 (see [media source record](3do-videos.md)).
- Installed 134 required media files, including intro, ending and all 25 ship
  presentations; retained the existing engine resource map.
- Structural validation passed for all 27 clips and 9,209 frames, including
  header dimensions, bounded frame offsets and sections, DUK audio headers,
  decoder scratch-buffer limits and 26 supported AIFF PCM tracks.
- Rebuilt the browser package successfully. The real browser detects the
  complete pack and reports that original videos are enabled.

This does not establish full campaign completion or audible synchronization.

- Original 3DO intro renders changing animated frames in the browser, including
  the opening spacecraft sequence. No browser warnings/errors captured during
  this smoke check. A local screenshot is retained in the ignored media cache.

- Super Melee ship-video test: open the fleet editor with Esc, open its ship
  picker with Enter, then press Alt. The original Androsynth movie renders
  correctly; Esc returns to the picker. No console warning/error captured.
- The ending and the other 24 ship movies are installed and passed structural
  checks; their full playback has not been individually observed.

## Dialogue subtitle fix — 2026-10-18

The launcher previously passed `--nosubtitles`, suppressing spoken dialogue
text even when the saved preference requested it. A positive `--subtitles`
startup flag now enables dialogue text and takes precedence over an older
disabled preference. The existing disabling flag remains available.

- WebAssembly rebuild and `git diff --check` passed.
- In a separate browser session, the rebuilt launcher starts successfully and
  Setup → PC/3do Compatibility Options shows Subtitles: Enabled.
- No browser warning/error captured. The test tab was closed without leaving
  Setup, avoiding a saved-preference write from the test.
- The user's active campaign was not refreshed or modified. They must save
  before reloading to use the updated launch setting.


## Online Melee private alpha — 2026-10-18

The browser build now includes UQM's existing network simulation and protocol,
connected through a private two-player WebSocket relay. This is a private
alpha; no public release or deployment has been performed.

### Observed browser result

Two independently launched browser pages on one computer connected to the
local Node relay, entered the original Melee flow and reached `inBattle`.
Combat continued with thousands of matching simulation CRC checks and zero
observed checksum mismatches. The launcher identifies the host's bottom fleet
and guest's top fleet, with separate user clicks required to join and launch.

This is evidence of local browser-to-browser synchronization. It does not
establish successful play between two physical computers, internet latency
handling, all ship matchups or full-match stability across browsers.

### Repeatable checks

- `npm test` covers storage/video initialization, browser room URLs and bounded
  queues, launch gating, role assignment, disconnects, relay pairing and room
  isolation, malformed/oversized data, rate limits and static file isolation.
- The relay tests use real WebSocket connections on ephemeral loopback ports.
  A 40,000-ping paused-reader regression verifies that control frames cannot
  bypass the server's outgoing memory bound. Ordinary ping replies and browser
  heartbeat pongs continue to work.
- `bash sc2/src/uqm/supermelee/netplay/tests/run.sh` compiles the actual transport
  and receive logic to Wasm in direct and pthread modes. It also compiles the
  actual variable-length setup packet handlers and blocking disconnect waits.
  These regressions cover fragmented/coalesced input, invalid packet lengths
  and types, truncated fleet/name data, invalid sides, send/receive disconnects
  and failure propagation during setup/handshake/reset/battle-end transitions.
- `npm run test:browser` runs the packaged game in two independent Chromium
  processes with real keyboard input. Its assertions cover full fleet matches,
  ship replacement, simulated delay/jitter, disconnects during setup/selection/
  battle, zero checksum mismatches and local fleet persistence after reload.
  Screenshots and a JSON result record are written to `test-results/browser/`.
  This describes the automated coverage; completed runs are recorded below
  when available.
- The normal build selects `CHOICE_netplay_VALUE='web'`, refreshes the source
  dependency list and relinks the launcher before packaging. The CI workflow
  runs the full build, source harness and real-browser scenarios rather than
  checking configuration text alone. The hosted workflow runs on the development branch and pull request; the
  linked run below records the clean Linux validation.

### Remaining online release checks

- Two physical computers through a shared HTTPS origin and WebSocket reverse
  proxy, followed by realistic latency/jitter and interrupted connections.
- Complete matches, rematches and ship transitions across representative
  matchups, with zero unexpected checksum mismatches.
- Foreground/background tab transitions, long sessions, focus loss and browser
  audio behavior during an online match.
- Independent current Chrome, Firefox and Safari testing.
- Public-host configuration and operational limits. The relay deliberately
  stops accepting fresh rooms after 10,000 distinct room IDs in one process;
  a reverse proxy also shares the per-IP limit unless deployment changes it.

### Completed automated browser run

The 2026-10-18 final local run used two independent Chrome 155.0.8059.84
processes on macOS with the rebuilt WebAssembly package:

- Baseline: a full two-ship fleet match, replacement ship and a rematch reaching
  ship selection; 547 matching simulation checks per side, zero mismatches.
- Injected one-way delay pattern 80/120/30/60 ms: a full two-ship fleet match
  and a rematch reaching combat; 549 matching checks per side at the end of
  the first match, zero mismatches. Each ship had to produce fresh matching
  frames; cumulative counters alone could not satisfy the check.
- Closing the opponent's real browser during setup, ship selection and combat
  displayed a disconnect and returned the surviving C engine to the drawn
  main menu after acknowledgement. No captured page crash or JavaScript error.
- The saved local fleet/control file remained byte-identical in memory and
  after reloading from IndexedDB in all three scenarios.
- All 43 JavaScript/real-WebSocket tests and five actual-source Wasm harness
  programs passed. The source harness includes direct and pthread execution.

The browser run records SHA-256 hashes of every tested runtime artifact in
`test-results/browser/build-hashes.json` alongside per-scenario results and
screenshots. These generated files stay local; the checked-in evidence below
shows the next-ship screen and successful return to the main menu. This test
uses a normal saved two-ship fleet fixture and genuine keyboard commands,
including Shofixti's self-destruct special; no simulation state is patched.

![Online replacement-ship selection](screenshots/online-next-ship.png)

![Return to main menu after opponent disconnect](screenshots/online-returned-menu.png)

### Local play regression on the network-enabled build

- Fresh isolated Chrome: local HUMAN versus WEAK CYBORG Melee, Chmmr/Kohr-Ah
  combat and a later Pkunk combat, then confirmed exit to fleets and main menu.
  No captured browser errors or Wasm aborts.
- A new campaign reached Sol, saved through the original menus into empty
  slot 00 as `netplay-test`, and reloaded the same save after a full page
  refresh. The save bytes were identical after IndexedDB restoration, and
  selecting the slot returned to Sol with the saved date and state.
- In an independent test, an online setup disconnect returned to the main
  menu, then New Game started the campaign in the same engine instance.
  Saving worked and a full page reload preserved all 3,333 save bytes.
- These checks used isolated test profiles at port 10000. The user's active
  campaign on port 9999 and its saved games were not modified.

![Campaign restored on the network-enabled build](screenshots/netplay-build-campaign-restored.png)

Clean Linux build and regression run:
[GitHub Actions](https://github.com/vcorem/uqm-3do-web/actions/runs/39115897414).
