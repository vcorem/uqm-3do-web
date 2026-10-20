# Cosmo Clash: menus and Online Melee in the game

All menus and the Online Melee lobby are screens inside the game frame, in the
look of the original menus (the main-menu art, the glowing cyan title of the
logo, dark blue glass panels, uppercase buttons). They behave like the UQM online
lobby (`web/online/online-ui.js`, `docs/online-ui.md`); nobody has to copy a link
to play.

Code: `src/netplay/OnlineController.ts` (the state machine, no DOM),
`src/netplay/OnlineView.ts` and `online.css` (the screens), `src/netplay/CosmoLobby.ts`
(the room list client), `src/netplay/CosmoNetplay.ts` (the room client and the
lockstep), `src/netplay/roomName.ts`, `src/ui/*` (main menu, difficulty, ship
choice, pause and game over, battle text), `src/main.ts` (scenes and battle loop).

## Screens

| Screen | What it shows | Keys |
| --- | --- | --- |
| Main menu | Play vs Computer, Two Players · Same Computer, Online Melee; Full screen (F) in the corner | ↑ ↓, Enter/Space, F |
| Difficulty | Easy, Medium (focus), Hard, Back | ↑ ↓, Enter, Esc |
| Ship choice (local) | The 3D ships; Player 2 chooses first in Two Players; vs Computer: the difficulty, later "Choose Your Next Ship" with the ships left on both sides | ← →, Enter (vs Computer also Space), Esc; on "Choose Your Next Ship" Esc asks "Leave this game?" (Keep playing, Main menu) |
| Battle (local) | HUD bars, P1/P2 and who (You, Computer, Player 1/2), fleet chips, rings in the player colours for 3 s, a message after each battle (a long one wraps to two lines above its second line) | Esc or Menu (Esc): Paused |
| Paused (local) | Resume, Main menu; the battle waits | ← →, Enter, Esc resumes |
| Game over (local) | Victory or Defeat (vs Computer), Player N wins; Play again, Main menu. Enter and Space do nothing for the first 600 ms (it opens by itself after the last battle) | Enter, Esc |
| Lobby | ONLINE MELEE, the live list of open rooms (name, waiting time, the newest has the focus), Create room, Back. States: connecting, no rooms, lost list (Try again; the game also tries again after 1, 2, 4, 8, 15 s), no room list on this server (Create private room) | ↑ ↓ list, ← → list and buttons, Home/End/PageUp/PageDown, Enter, Esc |
| Name | The room name (remembered; first "Pilot NN"; selected, so typing replaces it), the reason when a name is refused, Open room, Cancel. Before the room list answered, Open room shows "Checking the room list…" and waits up to 10 s | type, Enter, Esc, ↓ |
| Waiting | ROOM OPEN (or PRIVATE ROOM OPEN with the invite link), the name, "Open for m:ss · closes after 10 min (private: 2 min) without an opponent", Copy invite link (focus), Cancel | Esc closes the room |
| Joining | The room name, Connecting…, Cancel | Esc (Enter does nothing) |
| Invite | For `/cosmoclash/?cosmoRoom=<token>`: Join, Not now | Enter, Esc |
| Fleet | "Opponent found!", the 3D ships, the ship card (Enter adds it as ship N, or takes it out), the numbered slots 1 2 3 (badges in 3D too), Ready · N ships, Undo, Leave room, the opponent's progress (choosing · n of 3 picked; ready with n ships). A ready player has Change fleet while the opponent still chooses | ← → ships, Enter, ↓ ↑ buttons, Backspace undo, Esc: Leave the room? |
| Countdown | 3, 2, 1, which player you are (Player 1 cyan left, Player 2 orange right), both fleets | Esc: Leave the match? |
| Battle (online) | P1/P2 · You/Opponent in the player colours, fleet chips (current, destroyed), rings and YOU for 3 s per battle, "Your Gondi was destroyed · Next: Sholev" | the ship keys; Esc or Leave (Esc): Leave the match? |
| Result | VICTORY, DEFEAT or DRAW, both fleets with the destroyed ships, Rematch (Accept rematch; both must accept), Back to lobby | Enter, Esc back to lobby |

After a rematch the fleet screen keeps the last fleet and Ready has the focus;
the fleet and result screens ignore Enter and Space for their first 600 ms (a
double press of Rematch does not also say Ready), and Change fleet takes a
Ready back. When the opponent leaves, the connection is lost, the relay ends the
room ("The room closed. Server is stopping.", the room time limit, an idle room)
or the game stops (for example different checksums), the game goes back to the
lobby and says why.

The lobby shows only the rooms of the list of this visit: the rooms go when the
lobby closes and when the list is lost, so a room that closed meanwhile is not
shown or joined. A named room needs a server with room lists: it opens when the
list answered; on a server without room lists it is private (the waiting screen
says so).

- Every item is a real button: mouse, touch, Tab and screen readers work. The
  keyboard focus is the DOM focus and has a bright cyan ring.
- One action per key press: held-key repeats do nothing in the menus. The game's
  handlers make every Enter and Space choice; the browser's own action is
  prevented (else the key that skips the logo would also start a game).
- Enter never cancels: after Open room the focus is on Copy invite link; Joining
  ignores Enter; the Leave questions have Stay first.
- One live region (`#cosmo-status`, `role=status`) says each screen and message.
- A menu choice (a user gesture, not Esc) asks for full screen of the game
  frame (`#cosmo-frame`, which holds the 4:3 game container) and, in Chromium,
  `navigator.keyboard.lock(['Escape'])`, so Esc stays a game key. When the game
  is not in full screen (refused, or Esc in Firefox and Safari left it), the
  next choice asks again. F (on the menu screens, not while a ship flies) and the
  corner button of the main menu turn full screen on or off; "off" stays until
  the player turns it on.
- Two Players on one computer: Space is the fire key of Player 2, so only Enter
  chooses in the ship choice, the pause and the game over panels.
- Sizes scale with the game height (`--u` = height / 720, set by `Viewport`):
  the screens read the same at 1024x768, 1280x720, 1920x1080 and in full screen.
- "Leave the match?" does not pause an online battle (the lockstep cannot wait):
  the dialog says so, and the ship gets no keys while it is open.

## Contract with the site (platform stream)

1. The game is at `/cosmoclash/` (Vite base). Invite link: `/cosmoclash/?cosmoRoom=<32 lowercase hex>`.
2. Edition `cosmoclash`. Room list: `/netplay/lobby?edition=cosmoclash`; the server
   sends `{"type":"rooms","rooms":[{"id","name","engine":"rust","created"}]}` at
   connect and on every change; the client sends `ping` every 30 s and gets `pong`.
3. Rooms: `/netplay/cosmoclash/<token>?role=host&name=<urlencoded>&engine=rust`
   (listed), `?role=host` (private), `?role=guest`. Room names follow the UQM rule
   (`roomName.ts`, tested with `sc2/wasm/tests/room-names.cjs`). A listed host
   sends `ping` every 30 s while its socket is open; the relay answers `pong` and
   closes a listed room whose host is silent for more than 150 s. Relay control
   messages are JSON text (waiting, ready, error, peer-left); game data is binary.
4. A relay without room lists answers the lobby URL with
   `{"type":"error","message":"Invalid room URL or role."}` (Rust relay) or
   "Invalid lobby URL." (Node relay, Worker) before any list: the client reports
   `unsupported`, does not retry, and the lobby offers a private room.
5. The engine tag of Cosmo rooms is `rust`.

## Room protocol 2

`PROTOCOL_VERSION` is 2 and the build id is `cosmo2:<Wasm SHA-256>`. While the
players choose, a `fleet` packet has 0 to 3 different ships (the picks so far),
so that each player sees the other's progress; Ready needs 1 to 3 ships, and a
`ready` from a player without ships is refused. The opponent's ships show only
at the countdown.

- Change fleet: `{type:'unready',round}`. Only the host starts the match, so the
  host decides. A host is not ready at once (the guest sees it). A guest asks
  and keeps its fleet until the host answers `{type:'unready-ok',round}`; when
  the host had already sent `start`, the host ignores the request and the match
  starts with the fleet that was ready.
- Beats: in the fleet choice and on the result screen each peer sends
  `{type:'beat',round}` every 20 s (a beat of the last round is accepted just
  after a rematch). Two minutes without any packet end the room ("Your opponent
  stopped responding."); in the battle the limit is 20 s.
- `peer-left` "The other player disconnected." is "Your opponent left the
  room."; other `peer-left` reasons show as "The room closed. <reason>".

The input lockstep, the checksums, the result agreement and the rematch do not
change. Protocol 1 and 2 pages do not play each other (the version check stops
them before any battle). The build id is the Wasm hash only: a later change to
these packets in TypeScript alone must also raise `PROTOCOL_VERSION`, else an
old open page and a new page pass the check and then refuse each other's packets.

## Local serving and the room list adapter

`npm start` (`scripts/serve.mjs`, `scripts/cosmo-server.mjs`) serves `dist/` at
`/cosmoclash/` with the site's isolation headers and sends `/netplay/` to the relay.
It asks the relay for the cosmoclash room list at start. If the relay has none
(the Node relay before the platform stream adds it), `scripts/cosmo-lobby.mjs`
adds it in front of the relay by the contract above: it lists named rooms, answers
the host's `ping`, ends silent listed rooms, and sends the room traffic (without
`name` and `engine`) to the relay, which pairs the players. With a relay that has
the list, the adapter is not used.

## Tests

- `npm test --prefix cosmoclash`: `roomName.test.ts`, `CosmoLobby.test.ts`,
  `CosmoNetplay.test.ts`, `OnlineController.test.ts`, `OnlineView.test.ts`,
  `MainMenu.test.ts`, `online-relay.test.ts` (the room list and rooms through the
  real Node relay with the adapter, and `unsupported` from the Rust relay and the
  Node relay without it), `base.test.ts`.
- `npm run test:browser --prefix cosmoclash`: `solo.mjs`, `game.mjs`,
  `online-ui.mjs`. `online-ui.mjs` saves every online screen in
  `test-results/online-ui/` (1280x720, 1024x768, 1920x1080) and `results.json`;
  `solo.mjs` saves the local screens (difficulty, ship choices, pause, game over,
  the battle message after a lost ship, "Choose Your Next Ship", "Leave this
  game?") in `test-results/local-ui/` and checks that no text leaves the game
  frame and that the battle message does not cover its second line.
