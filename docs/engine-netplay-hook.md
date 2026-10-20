# Online Melee from the main menu: the engine side

All four engines (Original C, Original Rust, HD C, HD Rust) let the page start
an online Super Melee while the game shows its main menu, so that a lobby in
the page can start any number of sessions without a reload. Code:
`sc2/src/uqm/restart.c` and `credits.c` (Original),
`hd/patches/0009-web-netplay-menu-hook.patch` (HD). The Rust engines are
regenerated from these sources (`scripts/rust-engine/README.md`).

## What the page provides

| `Module` member | Set by | Meaning |
| --- | --- | --- |
| `takePendingNetplayRole()` | page | Returns `0` (host), `1` (guest) or `-1` (nothing). The engine calls it in every main-menu frame. Return a role once; before that, set `Module.webNet`. |
| `webNet` | page | The room connection (`UQMNetplay.connect(...)`) of the session. The engine closes it when the session ends. |
| `holdMainMenu` | page | Optional. While it is `true`, the main menu does not time out. Set it while the online lobby is open (room list, name entry, waiting for an opponent, countdown) and clear it when the engine takes the role or the lobby closes. |
| `supportsNetplayHook` | engine | `true` from the first time the main menu shows. |
| `gameScreen` | engine | `'mainMenu'` only while the engine polls the hook. `'transition'` in the main menu's sub-screens (Setup, the HD difficulty chooser, popups), the intro, the credits and the campaign; `'melee'` in Super Melee. |

Without `takePendingNetplayRole` the engine works as before.

## What the engine does

- In each main-menu frame the engine makes one call to the page (each call
  waits for the browser's main thread). The call takes the role and reads the
  hold: `Module.holdMainMenu`, or `Module.webNet` in the state `waiting` or
  `ready`.
- On a role the engine starts the session as `--web-netplay=host|guest` does
  (input delay 3, `gameScreen` `'melee'`), plays it in the Super-Melee screen
  and then returns to the main menu. When the room closes, the engine shows
  the reason in a popup; one key returns to the main menu.
- Original only: the main menu times out after 20 s without input (2 minutes
  with its music, which this build does not have) into the title and the
  credits. The hold stops this. If no hold was set, the credits take a role
  too: they end, and the session starts after a fade. During the title
  picture (3 s) the role waits. The HD web build is a DEBUG build and has no
  main menu time-out.
- HD only: the session turns off the ship seed and reloads the ship list, as
  Super Melee from the menu does, so both players have the same ships.
- The Super-Melee NET... button says "For Online Melee, press O in the main
  menu." (the page's lobby key); in a session it says that the room assigns
  the sides.

## Test

`npm run test:netplay-hook` (`UQM_EDITION=hd` for HD; `UQM_DIST` and
`UQM_PEER_DIST` for the two players' builds) launches both pages with the
shell's normal start path and plays two sessions from the main menu. Session
1 leaves the main menu idle for 25 s without a hold and sets `Module.webNet`
only with the role, as the lobby does. Session 2 holds the menu with
`Module.holdMainMenu` and then with a ready room. Both sessions play a full
fleet match with checksums, close the room and return to the main menu.
