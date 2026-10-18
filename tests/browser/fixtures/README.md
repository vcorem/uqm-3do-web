# Campaign save fixture

`before-starbase.uqm` is a normal UQM save created through the game's keyboard
controls on 2026-10-18. A new campaign was flown to Earth, the Ur-Quan probe's
recorded message was encountered, and the game was saved through Game Menu →
Save near the starbase, before the first conversation with Commander Hayes.
No game variables, ship position, story flags, or save bytes were edited.

The subtitle regression loads this file using Load Game and flies into the
starbase with real keyboard input. It uses the installed game media; the
fixture contains campaign state, not voice or video assets.

The two fresh browser runs use the same legacy `subtitles = BOOLEAN:false`
preference. The positive run keeps the launcher's `--subtitles` override. The
negative run removes that override, so it also verifies that the saved
preference is parsed. Both runs must open and play the actual Hayes voice
clip; only the positive run may render subtitle pixels.

Run from the repository root with a packaged build:

```sh
UQM_DIST=.tools/netplay-dist CHROME_BIN="/Applications/Google Chrome.app/Contents/MacOS/Google Chrome" node tests/browser/subtitles.mjs
```

`UQM_TEST_OUTPUT` selects the evidence directory. `HEADED=1` shows the browser.
The script owns and closes its local server and fresh browser processes. It
writes screenshots, audio/pixel observations, game logs, and tested build
hashes. Navigation uses short wall-clock key presses and has been verified in
Chrome locally; it may need timing calibration on heavily loaded CI runners.
A missed station collision fails with a screenshot rather than bypassing
normal gameplay.
