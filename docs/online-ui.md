# Start screen and online lobby in the game frame

The page shows its start screen and the Online Melee lobby inside the game
frame (`#screen`, the element that goes full screen). They are drawn on a
canvas over the game canvas with the game's own bitmap fonts, the Super-Melee
title lettering, star field, colours and BATTLE! picture, so they look like a
part of the game and work in full screen. Code: `web/online/online-ui.js`
(shared by both editions); art: `web/online/build-art.py`.

## Screens

| Screen | What it shows | Keys |
| --- | --- | --- |
| Start | Title picture, **Play** and **Online Melee**, How to start steps, download progress | arrows, Enter |
| Lobby | **ONLINE MELEE**, live list of open rooms of this edition (name, C ENGINE / RUST ENGINE tag, waiting time), **Create room**, **Back** | arrows (Left/Right: list and buttons), Enter, Esc |
| Name | Room name field (remembered; default "Captain NN"), **Open room**, **Cancel**; a refused name shows the reason here | type, Enter, Esc |
| Waiting | "Room open", the name, waiting time, **Cancel**, **Copy link** (focused first); "Almost ready" with the download while this page loads | Esc closes the room |
| Joining | The picked room while the client connects, or the download while this page loads | Esc (Enter does nothing) |
| Opponent found | Countdown (3 s), which fleet the player commands, **Cancel** | Esc (Enter does nothing) |
| Invite | For an invite link (`#room=<token>`): **Join** / **Not now** | Enter, Esc |
| Reload needed | Engine without the main-menu hook: **Reload** (opens the lobby after the reload) / **Back to game** | Enter, Esc |
| Message | Load failure or engine exit, **Reload** | Enter |

Mouse and touch work on every item (a transparent button lies over each drawn
item; the drawn focus follows the keyboard focus, so screen readers read the
items). A live region (`#status`) says the screen and messages.

- **Play** works while the game downloads; the game starts when it is ready.
- A room can be created or picked while the game downloads, but it opens
  (and a guest joins) only when this page's game is ready, so the opponent
  never waits in the engine for a download.
- The first choice is a user gesture: the frame goes full screen then, and
  Chromium keeps Esc as a game key (`navigator.keyboard.lock(['Escape'])`).
  Esc gives no user activation, so a choice made with Esc does not ask; a
  refused request is asked again at the next choice.
- A held Enter or Space repeats; repeats are ignored, so one press makes one
  choice. Enter never cancels: after **Open room** the focus is on **Copy
  link**, and Joining and Opponent found ignore Enter (Esc or a click cancels).
- Room names follow the room client's rule (`UQMNetplay.roomName`: NFC, inner
  spaces collapsed, no hidden or format characters, at most 24 characters).
  The name is remembered only when it is accepted, and the waiting screen
  shows it as the list does.
- While a match runs no page text is over the game.
- After a session (opponent left, match over) the engine returns to its main
  menu and the lobby comes back by itself, still in full screen, with the
  reason. From the main menu the **ONLINE MELEE / PRESS O** corner button (top
  right, clear of the menu) or the **O** key opens the lobby. The main menu
  does not use O. The button is not shown in the campaign or in battles.
- The lobby never covers another game screen: if the game leaves its main
  menu under it (a key pressed while the main menu fades in), the lobby hides
  and closes an open room. The menu opens at once when it is chosen, so a key
  pressed right after O is the menu's; a game key held while the menu opens
  gets its release on the game canvas.

## Contracts used

Engine hook (A): the page defines `Module.takePendingNetplayRole()` (0 host,
1 guest, -1 nothing). The first online session starts the engine with
`--web-netplay=host|guest` and `Module.webNet` set to the room client. Later
sessions, when `Module.gameScreen === 'mainMenu'` and
`Module.supportsNetplayHook === true`, set `Module.webNet` and then return
the role once (if the engine is away from its main menu at that moment, the
role waits until it is back there). `takePendingNetplayRole` tells the menu
when the engine took the role; a role not taken within 15 s is withdrawn and
the lobby shows an error. A session ends when `Module.gameScreen` goes from
`melee` back to `mainMenu`. The shell reports every `Module.gameScreen`
change as the engine makes it (a property setter), so a short session is not
missed between two polls. Without the hook the page offers a reload. The
Original main menu goes back to the intro after 20 s without input (2 minutes
with its music); while the menu covers the started game, the page sets
`Module.holdMainMenu = true` (and `false` when it hides), which the engine's
main menu reads (the ENGINE stream's hook build).

Lobby (B): `UQMNetplay.watchLobby({ edition, onRooms, onStatus })` gives the
room list (`{ id, name, engine, created }`, newest first); `onStatus` states
`closed` and `error` make the lobby show the problem and reconnect (2, 5, 10,
20 s). A host connects with `UQMNetplay.connect({ token, role: 'host', edition,
name, engine })`; `engine` is `rust` on Rust pages (`UQM_VARIANT.engine`),
else `c`. A guest joins with the listed id. If the client has no `watchLobby`,
the lobby says that the list is not available and invite links still work.

Scripted play (remote tests): `online.controller.host(name, token)` and
`online.controller.join(token)`. `host(null, token)` opens a private room
(no name: not in the list, so no other player can take it); the relay keeps
a private room 2 minutes, a listed one 10.

## Art

`build-art.py --edition original|hd --content <dir> --output <dist>` runs in
every build (`build-web.sh`, `build-hd-web.sh`, `rust-engine/link.py`) and
writes `online-art.json`, `online-art.png` (glyph atlas and title lettering),
`online-stars.png` and `online-title.png` next to the page. It uses only the
Python standard library.

- Fonts: `micro` (body text, like the Super-Melee team names), `starcon`
  (buttons and headings), `tiny` (small text) and the menu font (`slides`
  for Original, the HD main-menu `slab`). Metrics follow the engine
  (`gfxload.c`, `font.c`): advance = width + spacing, HD kerning from
  `kerndat.fnt`. "N/A" placeholder glyphs are left out. Characters that a
  font does not have (for example Hebrew room names) are drawn with a system
  font, right to left when needed.
- Title: **ONLINE MELEE** in the Super-Melee title lettering. Original takes
  the letters from `ui/meleemenu-000.png` and its row colours; HD uses
  `fonts/melee.title.fon` and gradient frame 7, as the engine does. O, N and
  I are not in the title, so they are made from U (mirrored bottom), the stem
  of L, and a diagonal of the same weight.
- Star field: Original removes the title and buttons from `meleemenu-000`;
  HD uses `ui/meleemenu/meleemenu-000.png`. Title picture: `ui/title.png`
  (HD: `ui/mainmenu/title.png`). BATTLE!: `meleemenu-025/026` (HD 004/005).
- Licences: the art is the game's content and keeps its licence. Original:
  UQM content, CC BY-NC-SA 2.5 (`COPYING.txt`). HD: UQM MegaMod HD content
  (`content-licenses.txt`, preserved attribution); the fonts and UI pictures
  used here have no third-party entries in its `ATTRIBUTION.txt`.

## Tests

- `node --test tests/web/online-ui.test.cjs`: the state machine (`npm test`).
- `node tests/browser/online-ui.mjs`: the real page shells with a test engine
  (`tests/browser/fixtures/fake-engine.js`) and a test lobby
  (`tests/helpers/online-lobby.mjs`) in front of the Node relay; keyboard and
  mouse flows, every screen saved for review at 1280x960 (full screen) and
  1024x768, both editions.
- `tests/browser/netplay.mjs`: real engines (C and Rust with
  `UQM_PEER_DIST`) meet in the test lobby and play a full match.
- Until the room client has `watchLobby`,
  `tests/browser/fixtures/lobby-client-shim.js` adds it in tests.

## Notes for the other parts of the page

- Loading progress: the start screen shows `Module.setStatus('… (done/total)')`
  byte progress and `Module.monitorRunDependencies` counts. A loader that
  fetches the game files itself (the CACHE stream's `engine-loader.js`) calls
  `online.controller.progress({ label, fraction })` instead, through the
  global `online` that the shell defines. Tests read the loading text from
  `online.state.loading.label`; `#status` is the menu's live region (it says
  "Unable to start. …" for a load failure), and the page has no `#progress`
  element.
- The page defines `Module.takePendingNetplayRole` in its `Module` object, so
  it exists before the engine starts.
- `tests/helpers/online-lobby.mjs` is a test lobby in front of any relay: it
  answers `/netplay/lobby`, lists rooms whose host adds `name=`, and forwards
  room connections without the lobby parameters, so the current relays pair
  the players. With a relay that has the lobby, tests can use it directly.
