# Afterlight combat adapter

Afterlight uses the **actual Original UQM Rust engine** for combat. The campaign
shell owns travel, dialogue, quests, resources and saves. Native UQM owns ship
physics, weapons, AI, collision, damage, special abilities, crew and victory.
There is no second combat simulator and no JavaScript method that awards a win.

## Build and hosting

```sh
bash scripts/build-afterlight-combat.sh
```

Output: `.tools/afterlight-combat/` (override `BUILD_OUTPUT`). Package this folder
as `combat/` next to the campaign page. The build links the existing complete
Rust engine with its normal SDL/OpenAL/Emscripten dependencies. It preloads the
base asset pack only; original voice, music and FMV add-ons are omitted. The
source media tree is never changed. Native music volume is zero so the new
campaign score can continue; native weapon and collision effects remain active.
Retain `COPYING.txt` and the project's content notices with the package.

Serve over localhost or HTTPS with the existing relay's cross-origin isolation
headers. Both included Node and Rust servers recognize `.mjs` modules. A modern
desktop browser and keyboard are required. No network match or relay room is
created by a campaign encounter.

The browser shows a first **Enter combat** button to obtain real audio/input
activation. It keeps the native engine alive between encounters. The combat
process uses `/tmp/afterlight-uqm` for temporary settings and never mounts the
Original or HD IndexedDB save directories.

## Parent API

`campaign/src/combat.mjs` exports `SHIPS`, `validateEncounter` and `UQMCombat`:

```js
const combat = new UQMCombat({ container, url: './combat/index.html' });
await combat.preload();
const result = await combat.start({
  id: 'encounter-42',
  player: { ship: 5, crew: 18 },
  enemies: [19, 24],
  seed: 12345,
  difficulty: 'standard',
});
```

`SHIPS` follows the original 25-entry `MeleeShip` enum. Each entry includes
`id`, `name`, ordinary `crew` and `maxCrew`; Syreen can carry more than its
ordinary starting crew. Enemy fleets accept 1–14 ships in authored order.
Difficulty accepts `standard`, `good` or `awesome`. IDs are nonempty strings
up to 128 characters; seeds are unsigned 32-bit integers.

The returned result is `{id, outcome, crew, enemyRemaining, frames}`. Outcome is
`victory`, `defeat` or `retreat`. Crew and surviving enemy counts come from
the native simulation and native cleanup. Simultaneous death is a campaign
defeat. Native resurrection mechanics remain native: starting crew is applied
only on the initial player spawn, never on later resurrection.

`retreat()` requests the native battle-abort/cleanup path. This is an explicit
campaign withdrawal, not a claim that stock Melee supports the original
campaign's hyperspace escape animation. The parent applies its authored
withdrawal cost. If cancelled before the first launch, the promise rejects
with `AbortError`; no battle result or victory is manufactured.

`retry()` reloads a failed combat process after the parent preserves its pending
encounter. `destroy()` closes it. Do not retry or overwrite saves while a battle
is running. Events: `ready`, `visible`, `started`, `result`, `cancelled`, `error`.
Errors reject; they do not advance the campaign. The wrapper and child validate
both exact origin and message source, and a request cannot replace an active
encounter. An `error` event or rejected promise must leave campaign progress
recoverable in the parent UI.

## Native hook boundary and regeneration

`rust/uqm-engine-original/campaign.rs` is a small hand-written atomic mailbox.
The browser writes a validated encounter and the game thread consumes it at
the existing Melee input loop. The mailbox's pending/running/result phases
prevent cross-thread partial configurations. The game sets real fleets and
controls, seeds its normal RNG, enters `StartMelee`, auto-selects the next
surviving fleet slot, and reports actual native survivors before cleanup.

The opt-in hooks are explicitly marked `AFTERLIGHT` in `restart.rs`, `melee.rs`,
`pickmele.rs`, `ship.rs`, `battle.rs` and `init.rs`. **Setup team 1 maps to battle
player 0**, an intentional legacy UQM reversal. The campaign player is battle
player 0 with the normal first keyboard template. The original game shells do
not export or enable the mailbox and preserve their original flow.

The historical C translation inputs are unchanged: these are deliberate
post-translation Rust additions, not a claim that the locked C reference
already contains the new campaign. `campaign/native-hooks.patch` preserves the
exact additions to the generated crate and Cargo feature declaration. After
regenerating into a fresh staging crate using `scripts/rust-engine/README.md`,
apply the patch there with `patch -p1`, copy `campaign.rs` from the current
crate, rebuild and rerun the combat checks before adopting the regenerated
sources. Do not regenerate directly over the working crate.

An optional `AFTERLIGHT_COMBAT_TEST_DRIVER=1` build enables the existing native
AI to control the player's hull for automated simulation tests. It is isolated
under `.tools/rust-afterlight-test`; the production shell does not export that
entry point. It does not set health, damage or winners. The default browser
regression asserts the test export is absent and uses actual keyboard inputs.

## Validation commands

```sh
node --test campaign/tests/combat.test.mjs
bash -c 'source scripts/rust-env.sh; rustc --edition=2021 --test rust/uqm-engine-original/campaign.rs -o .tools/afterlight-mailbox-test && .tools/afterlight-mailbox-test'
CHROME_BIN='/Applications/Google Chrome.app/Contents/MacOS/Google Chrome' \
  node campaign/tests/combat-browser.mjs
```

The browser regression starts its own isolated local server and clean Chrome
process, then checks native victory, defeat, crew carry-over/withdrawal,
multi-ship fleets, repeat encounters, real audio activation and native sound
PCM. It captures screenshots and exact build hashes under
`test-results/afterlight-combat/`. No simulation state is patched in the test.
`AFTERLIGHT_COMBAT_RESULTS` selects another evidence directory.
Set `AFTERLIGHT_SHIP_MATRIX=1` to additionally launch all 25 native hulls at their
maximum supported crew and check actual crew, retreat and process reuse.

To verify the normal game remains unaffected, link the same Rust archive with
`scripts/rust-engine/link.py original` **without** `--campaign-combat`, use
`AFTERLIGHT_ORIGINAL_DIST` to select that output, and run
`campaign/tests/original-regression.mjs`. This opens the ordinary main menu and
Melee through real keyboard input and verifies the campaign API is unexported.

The default engine remains pixel-art UQM combat. The new campaign's illustration
style does not imply that all 25 native ship sprite sets have been redrawn.

## Original surface expeditions

`campaign/src/lander.mjs` implements a separate new surface game for Afterlight,
not a port of the original game's lander. It renders seeded terrain, nine
damaging vents, three physical survey sites, optional ore and a landing pad.
The player steers with acceleration and braking, records all three sites, then
returns physically to the pad. Returning early or exhausting the 120-second
battery evacuates; hull destruction performs an emergency retrieval. A visible
return button and Escape always allow withdrawal. Hidden tabs pause the active
simulation clock. Keyboard and pointer controls are included.

```js
const expedition = new SurfaceExpedition({
  container, seed: pendingSurvey.seed, biome: 'crystal', id: pendingSurvey.id,
  onComplete(receipt) { /* resolve against the campaign's pending survey */ },
}).start();
// expedition.retreat(); expedition.destroy();
```

Completion occurs once with `{id?, outcome, salvage, hullRemaining, beacons}`.
Outcomes are `surveyed`, `retreated`, `damaged`. Salvage is bounded at 24 per
attempt, hull at 0–100, beacons at 0–3. Successful surveys require all three
actual visits plus returning to the pad. The campaign state layer applies
authored rewards and its cumulative salvage limit; the surface module never
edits campaign saves. Destroying the UI does not manufacture a receipt.

`createSurface`, `stepSurface`, `surfaceReceipt` and `SURFACE` expose the pure
fixed-step simulation for deterministic tests. Inputs are direction components
in `[-1, 1]` and optional `retreat`; time is measured in seconds. Terrain
generation does not use global randomness. The input state is never mutated.

```sh
node --test campaign/tests/lander.test.mjs
CHROME_BIN='/Applications/Google Chrome.app/Contents/MacOS/Google Chrome' \
  node campaign/tests/lander-browser.mjs
```

The browser check uses actual keyboard movement to collect ore, visit all three
sites and return; it also checks pointer steering, button/Escape recall,
single-delivery completion, mobile overflow and cleanup. Evidence is written to
`test-results/afterlight-lander/` (override `AFTERLIGHT_LANDER_RESULTS`). The unit
suite covers 300 seeded maps, time partitioning, movement, collision boundaries,
invulnerability, salvage conservation, full surveys and failure/retreat routes.
