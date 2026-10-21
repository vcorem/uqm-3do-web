# The Afterlight Accord

An original, unofficial continuation of *The Ur-Quan Masters*, set seven years after the defeat of the Sa-Matra. You command the survey ship Wayfarer in the Far Reach. An abandoned shield asks for one passenger berth. The investigation becomes a story about the people reconstruction leaves out, the limits of a promise, and the difference between predicting an answer and hearing it.

This is a complete branching campaign with its own browser interface, star chart, resources, conversations, journal, saves, original illustrations and original instrumental score. Physical surface expeditions accompany the investigation. Space encounters use the actual Original UQM Rust/WebAssembly engine, its 25 ships, weapons, AI and sound effects.

The campaign is a new authored work, not an official sequel or a reconstruction of an unreleased game. The original campaign remains available through the repository's existing launchers.

## Play locally

Use Node 22 or newer and the pinned Rust/Emscripten setup documented in the repository README. On a checkout with its dependencies installed:

```sh
npm run build:afterlight
npm run start:afterlight
```

Open **http://127.0.0.1:10002/**. Do not open `campaign/index.html` directly with `file://`: loading the game data and threaded native combat requires a proper local web server.

For story, art, music or interface changes after combat has been built:

```sh
bash scripts/build-afterlight.sh --ui-only
```

The portable result is `.tools/afterlight-dist/`. Serve its contents at a site root or a subdirectory, preserving relative paths. It needs the same cross-origin isolation headers as the original UQM browser build:

```text
Cross-Origin-Opener-Policy: same-origin
Cross-Origin-Embedder-Policy: require-corp
```

Serve `.mjs` and `.js` as JavaScript, `.wasm` as `application/wasm`, and `.ogg` as `audio/ogg`. HTTPS is required outside localhost. The included Rust relay can serve the static package and headers, but this single-player campaign does not use multiplayer rooms or require a remote game service. No AI API, streaming service or external asset host is used at runtime.

For a standalone distributable download, first commit a clean revision, then run:

```sh
bash scripts/package-afterlight-release.sh
```

This rebuilds combat, creates a portable ZIP under `.tools/releases/`, and includes an exact corresponding source archive, revision, file hashes and hosting instructions. It refuses an uncommitted checkout. The ordinary development build is a preview; use the release packager when sharing a standalone binary download.

## Controls and recovery

- Read transmissions in passages; choose a response with the mouse or number keys. The clock does not run while reading. Costs appear before a decision.
- Select a known system in the star chart, inspect its fuel/day cost, then set course. The mission journal lists open signals and preserves evidence separately from the speaker's claims.
- Refuel and recover crew at equipped ports. A distress tow remains available with no money, fuel or crew; its time and supply cost are shown.
- On a surface expedition, use WASD, arrow keys or the direction buttons. Visit three survey rings and physically return to the landing pad. Blue ore is optional; red vents damage the lander. Escape recalls the team. An incomplete survey can be retried.
- In a native space battle, activate **Enter combat** once. Arrow keys steer/thrust, Right Ctrl or Enter fires, and Right Shift uses the special. Disengage withdraws. Defeat advances the story through a recovery scene; it is not secretly treated as victory.
- Every decision is saved in browser storage. Export a JSON save for a durable backup. Import rejects invalid files without replacing the current expedition. A previous valid autosave can be recovered from the menu.

The mobile layout supports reading, travel and touch surface controls. Native space combat still requires a keyboard; this release does not promise full phone-only play. All major story engagements have authored noncombat alternatives.

## Source guide

| Area | Source |
|---|---|
| All campaign writing, systems, quests, choices, surveys and endings | `content/campaign.json` |
| Deterministic rules, save validation and encounter receipts | `src/state.mjs`, `SCHEMA.md` |
| Browser interface | `index.html`, `style.css`, `src/app.mjs` |
| Original surface simulation | `src/lander.mjs` |
| Rust UQM bridge and reproducible hooks | `NATIVE-COMBAT.md`, `native-hooks.patch`, `combat-shell/`, `../rust/uqm-engine-original/campaign.rs` |
| Ten character portraits, cover and generation records | `assets/portraits/`, `assets/scenes/`, `assets/art-manifest.json` |
| Fifteen original Ogg cues and editable MIDI scores | `assets/music/`, `tools/compose.py` |
| Full plot and continuity notes — spoilers | `../docs/afterlight-story-bible.md` |
| Original campaign research | `../docs/afterlight-original-study-lore.md`, `../docs/afterlight-original-study-mechanics.md` |
| Developer handover | `../docs/afterlight-handover.html` |

Dialogue and campaign logic are authored JSON plus JavaScript. Space combat is the existing Rust translation compiled to WebAssembly. The new surface game is JavaScript/Canvas and deliberately does not claim to reproduce the original UQM lander. Native combat retains the original pixel-art ships; the illustrated portraits do not imply a redrawn ship set. There is no recorded voice acting.

## Verify

```sh
npm run test:afterlight
npm run test:afterlight:browser
node campaign/tests/integration-encounters.mjs
node campaign/tests/combat-browser.mjs
node campaign/tests/lander-browser.mjs
python3 campaign/tools/verify-audio.py
```

Install the locked Playwright Chromium with `npx playwright install chromium`, or select an installed Chrome using `CHROME_BIN`. Browser tests start their own isolated local servers. `test-results/` contains generated screenshots and reports and is intentionally not committed. `gameplay-validation.json` records the tested native build and real encounter receipts; the handover records final campaign validation.

The three ending playthroughs use ordinary UI actions all the way from the opening. The native tests physically fight, lose and retreat; they do not inject health, damage or winners. The integration test imports only saves produced by normal campaign actions, then steers and fights through the real interface. Unit tests additionally exercise malformed saves, duplicate receipts, resource exhaustion, deadlines, conditional epilogues and recovery paths.

CI: `.github/workflows/afterlight.yml` builds the actual Rust combat package and runs campaign and browser checks. A local pass is not a claim that hosted CI has already finished.

## Music and art production

The musical compositions use explicitly authored motifs, harmonic sequences, meters and arrangements rendered with a small synthesizer. They are delivered as playable Ogg files and editable MIDI, with score and measurement manifests. To regenerate them, install Python NumPy and FFmpeg, then run `python3 campaign/tools/compose.py`; `verify-audio.py` decodes every resulting file. Regeneration is optional for normal builds.

The ten portraits and cover are generated raster artwork. `assets/art-manifest.json` records the tool, art direction, individual briefs, dimensions and hashes. Art was generated from text descriptions without a reference-image edit. The surface maps, chart and ship schematic are code-rendered resources.

## Review boundaries

A completed automated route is evidence of playability, not proof of literary quality or equivalence to Star Control II. This campaign is substantially smaller than the original corpus. Independent human playthroughs, editorial feedback, music listening and longer accessibility/browser testing remain valuable before a public release. The authored endings are complete; publication, professional voice performances, localization, additional native ship art and a new multiplayer campaign are not included.

See `CREDITS.html` for attribution and licensing. New code and writing are GPL-2.0-or-later. New generated illustrations and synthesized music are CC BY-SA 4.0 to the extent rights are held; existing UQM assets retain their own notices.
