# Cosmo Clash: menus and Online Melee in the game

All menus and the Online Melee lobby are screens inside the game frame, in the
look of the original menus (the main-menu art, the glowing cyan title of the
logo, dark blue glass panels, uppercase buttons). They behave like the UQM online
lobby (`web/online/online-ui.js`, `docs/online-ui.md`); nobody has to copy a link
to play.

Code: `src/netplay/OnlineController.ts` (the state machine, no DOM),
`src/netplay/OnlineView.ts` and `online.css` (the screens), `src/netplay/CosmoLobby.ts`
(the room list client), `src/netplay/CosmoNetplay.ts` (the room client and the
lockstep), `src/netplay/roomName.ts`, `src/match/FleetMatch.ts` (the fleet match
of all three modes) and `src/match/FleetPicker.ts` (the fleet choice of Two
Players), `src/ui/*` (main menu, difficulty, ship choice, the fleet choice of Two
Players, pause and game over, battle text), `src/main.ts` (scenes and battle loop).

## Fleet matches

All three modes are fleet matches with one model (`FleetMatch`): the first ship
of a fleet opens the match. After each battle the loser's ship is destroyed and
the winner's ship stays (both ships reset for the next battle). A loser with two
or more ships left chooses the next ship from them; with one ship left it comes
by itself (no choice screen); with none the match is over.

- Play vs Computer: both sides have all three ships. "Choose Your Next Ship"
  comes with two ships left; with one, the message says "Your last ship:
  Sholev" and the next battle starts. The computer takes one of its ships left
  at random.
- Two Players · Same Computer: Player 2 and then Player 1 choose a fleet of 1 to
  3 ships (the first pick opens), Enter only. After a loss: "Player 2: Choose
  Your Next Ship" with his ships left; the game over shows the winner and both
  fleets.
- Online Melee: each player chooses 1 to 3 ships (the first pick opens; the
  order of the others does not matter). After a loss with two ships left the
  loser chooses within 20 s (room protocol 3).

## Screens

| Screen | What it shows | Keys |
| --- | --- | --- |
| Main menu | Play vs Computer, Two Players · Same Computer, Online Melee; Full screen (F) in the corner; the hint "↑ ↓ choose · Enter start · F full screen · K keys" | ↑ ↓, Enter/Space, F |
| Difficulty | Easy, Medium (focus), Hard, Back | ↑ ↓, Enter, Esc |
| Fleet choice (Two Players) | PLAYER 2 · CHOOSE YOUR FLEET (then Player 1), the 3D ships, the ship card (Enter adds it; the first pick opens the match), the slots (1 is the opener, a check mark for the others; badges in 3D too), Done · N ships, Undo, Back | ← → ships, Enter (not Space), ↓ ↑ buttons, Backspace undo, Esc main menu |
| Ship choice (local) | The 3D ships; vs Computer: the difficulty, later "Choose Your Next Ship" with the two ships left ("You have 2 ships left"); Two Players: "Player 2: Choose Your Next Ship". Destroyed ships are not shown; "Choose Your Next Ship" also shows both fleets as chips over Choose this ship, and its hint says "Esc Leave" | ← →, Enter (vs Computer also Space), Esc; on "Choose Your Next Ship" Esc asks "Leave this game?" (Keep playing, Main menu) |
| Battle (local) | HUD bars, P1/P2 and who (You, Computer, Player 1/2), both fleets as chips (destroyed struck through, the ship in battle bright; the computer's ships that did not fly are ?), rings in the player colours for 3 s, a message after each battle: "Your Gondi was destroyed" with "Choose your next ship.", "Your last ship: Sholev" or the end (a long one wraps to two lines above its second line) | Esc or Menu (Esc): Paused |
| Paused (local) | Resume, Main menu; the battle waits; the hint "Enter choose · Esc resume · K keys" | ← →, Enter, Esc resumes |
| Game over (local) | Victory or Defeat (vs Computer), Player N wins; both fleets; Play again, Main menu. Enter and Space do nothing for the first 600 ms (it opens by itself after the last battle) | Enter, Esc |
| Lobby | ONLINE MELEE, the live list of open rooms (name, waiting time, the newest has the focus), Create room, Back. States: connecting, no rooms, lost list (Try again; the game also tries again after 1, 2, 4, 8, 15 s), no room list on this server (Create private room) | ↑ ↓ list, ← → list and buttons, Home/End/PageUp/PageDown, Enter, Esc |
| Name | The room name (remembered; first "Pilot NN"; selected, so typing replaces it), the reason when a name is refused, Open room, Cancel. Before the room list answered, Open room shows "Checking the room list…" and waits up to 10 s | type, Enter, Esc, ↓ |
| Waiting | ROOM OPEN (or PRIVATE ROOM OPEN with the invite link), the name, "Open for m:ss · closes after 10 min (private: 2 min) without an opponent", Copy invite link (focus), Cancel | Esc closes the room |
| Joining | The room name, Connecting…, Cancel | Esc (Enter does nothing) |
| Invite | For `/cosmoclash/?cosmoRoom=<token>`: Join, Not now | Enter, Esc |
| Fleet | "Opponent found!", the 3D ships, the ship card (Enter adds it, or takes it out; the first pick "opens the match"), the slots (1 is the opener, a check mark for the others; badges in 3D too), "Ship 1 opens the match. After each loss you choose your next ship from the ships left.", Ready · N ships, Undo, Leave room, the opponent's progress (choosing · n of 3 picked; ready with n ships). A ready player has Change fleet while the opponent still chooses | ← → ships, Enter, ↓ ↑ buttons, Backspace undo, Esc: Leave the room? |
| Countdown | 3, 2, 1, which player you are (Player 1 cyan left, Player 2 orange right), both fleets as chips (the opening ships; after a choice "Next battle", the chosen ships and the destroyed ones struck through) | Esc: Leave the match? |
| Battle (online) | P1/P2 · You/Opponent in the player colours, fleet chips (destroyed, in battle, left), rings and YOU for 3 s per battle, "Your Gondi was destroyed · Choose your next ship." ("Your last ship: Sholev" when one is left; "Enemy Gondi destroyed! · Your opponent chooses the next ship.") | the ship keys; Esc or Leave (Esc): Leave the match? |
| Next ship (loser) | CHOOSE YOUR NEXT SHIP over the 3D ships (only the ships left; the ship above the panel; the battle bars hidden), the ship card (Enter launches it), the time (20 s, a bar; red for the last 5 s), "Choose in N seconds, or Sholev comes by itself.", both fleets, Leave match. When the time runs out the first ship left in fleet order comes | ← → ships, Enter or Space, ↓ ↑, Esc: Leave the match? (the time goes on) |
| Next ship (winner) | "Enemy Gondi destroyed!" over the last battle, "Your opponent is choosing the next ship", the same time, "Their choice: Sholev or MichiGichi", both fleets, "Your MichiGichi stays for the next battle.", Leave (Esc) for the mouse; no keyboard focus | only Esc: Leave the match? |
| Result | VICTORY, DEFEAT or DRAW, both fleets in the order the ships flew (destroyed ships struck through), Rematch (Accept rematch; both must accept), Back to lobby | Enter, Esc back to lobby |

After a rematch the fleet screen keeps the last fleet and Ready has the focus;
the fleet, next ship and result screens ignore Enter and Space for their first
600 ms (a double press of Rematch does not also say Ready, a fire key held at
the end of a battle does not choose), and Change fleet takes a Ready back. An
open "Leave the match?" stays open between the countdown, the battle and the
next ship. When the opponent leaves (also during the choice of the next ship),
the connection is lost, the relay ends the
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
- The keys (`src/ui/KeysHelp.ts`, `keys-help.css`): `K` or the Keys (K) button
  shows or hides them on every screen of this table, also in the battles, local
  and online; nothing pauses (the lockstep goes on) and the keyboard focus stays
  with the game. A held-key repeat does not toggle again; a K typed in the room
  name is a letter. With 140 px or more beside the 4:3 game (1280x720,
  1920x1080, also in full screen), an `aside` in each dark margin, never over the
  game: the mode, the players' keys and the ships (weapon and special from
  `CHARACTER_REGISTRY`) on the left; the menus and the game keys (Esc, M, F, K)
  on the right, under the button. They show at first; a hidden choice is
  remembered (`cosmoclash.keys.v1` in localStorage). Without that room (4:3
  windows, phones) the button is in the bottom right corner of the game and
  opens the same keys as a dialog over the game (it never opens by itself); `K`,
  Esc or Close (K) closes it, and that Esc does not also pause, leave or cancel
  the room name. Two Players and the main menu show both players; vs Computer and
  Online show the Player 1 keys and say so (online both players fly with the
  arrows on their own computer). `?` (the key of the UQM pages) is Shift + `/`
  here, two fire keys of a Two Players battle, so the key is `K`.
- One live region (`#cosmo-status`, `role=status`) says each screen and message
  (and each change of the keys).
  On the choice of the next ship it says the ship and the seconds left when the
  text changes (a new ship, the last 5 s, after a question), never each second.
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

## Room protocol 3

`PROTOCOL_VERSION` is 3 and the build id is `cosmo3:<Wasm SHA-256>`. Protocol 2
pages (fixed order of battle) and protocol 3 pages refuse each other in the
`hello`, before any battle ("The game versions differ.").

### The fleet choice (from protocol 2)

While the players choose, a `fleet` packet has 0 to 3 different ships (the
picks so far), so that each player sees the other's progress; Ready needs 1 to
3 ships, and a `ready` from a player without ships is refused. The first ship
opens the match. The opponent's ships show only at the countdown.

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

### The next ship (protocol 3)

A battle ends 90 ticks after a ship was destroyed. Both peers know at that tick
T (from the same simulation) what comes next: the end of the match (`finish`),
the last ship (the next battle starts at tick T+1, as before), or a choice:

1. Both peers report the checksum of tick T (the hashed match state has the
   fleet match, with the chosen ships) and call `awaitNext({battle, player,
   ships})` from that frame: the session is `choosing` and `advance()` makes no
   frame. Inputs and checksums of the last battle that are still on the way
   are accepted.
2. Only the loser chooses (the 3D ship choice, 20 s). When the time runs out,
   the loser's game takes the first ship left in fleet order; the winner never
   chooses. The loser sends `{type:'next',round,battle,ship}`.
3. The winner checks it (the loser's ship, a ship left, this battle), answers
   `{type:'next-ack',round,battle}` and goes on; the loser goes on when the
   `next-ack` comes. Each peer builds the next battle (the chosen ship and the
   seed of the battle number) before frame T+1, then a 3 s countdown with both
   fleets. The packets of each peer come in one ordered stream: the loser's
   inputs of the next battle come after its `next`, the winner's after its
   `next-ack`, so both start the next battle at the same tick with the same
   ship and seed.
4. A `next` that comes before this peer reached tick T (a slow or hidden page)
   is kept and used at `awaitNext`. A `next` from the winner, of another battle
   or with a ship that cannot fly next, a second early `next` and a `next-ack`
   without a choice stop the match ("Invalid or incompatible game data").
5. The choice has beats every 20 s (the battle limit of 20 s of silence does
   not apply), so a full 20 s choice never ends the room. It has its own
   limits, because the loser's game always sends `next` within 20 s
   (`NEXT_MS`): the waiting peer stops 40 s after its pause without `next`
   (`NEXT_WAIT_MS`), and the loser 20 s after its `next` without `next-ack`
   (`NEXT_ACK_MS`), "Your opponent stopped responding." (a connection that
   dropped without a close, for example a laptop that sleeps). While the loser
   chooses, its own limit is the two-minute silence limit. A Leave or a lost
   connection during the choice goes back to the lobby with the reason ("Your
   opponent left the room.").

The input lockstep, the checksums, the result agreement and the rematch do not
change. The build id is the Wasm hash only: a later change to these packets in
TypeScript alone must also raise `PROTOCOL_VERSION`, else an old open page and
a new page pass the check and then refuse each other's packets.

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
  `CosmoNetplay.test.ts` (with the next-ship packets through the real Node and
  Rust relays: the same frames and checksums on both peers across the choice),
  `OnlineController.test.ts` (the next ship screens, the 20 s limit, the first
  ship left at the end of the time), `OnlineView.test.ts`, `FleetMatch.test.ts`
  (the fleet match of all modes, the last ship by itself, the hashed state and
  the battle seeds), `FleetPickView.test.ts`, `MainMenu.test.ts`,
  `online-relay.test.ts` (the room list and rooms through the real Node relay
  with the adapter, and `unsupported` from the Rust relay and the Node relay
  without it), `base.test.ts`.
- `npm run test:browser --prefix cosmoclash`: `solo.mjs`, `game.mjs`,
  `online-ui.mjs`, `keys-help.mjs`. `keys-help.mjs` (`npm run test:keys-help`)
  saves the keys beside the game at 1280x720 and 1920x1080 and over it at
  1024x768 (the main menu, a battle against the computer and its pause, a Two
  Players battle, the online lobby, the fleet choice and an online battle; the
  battle against the computer and a menu also in full screen) in
  `test-results/keys-help/`, and measures that the side panels never overlap
  the game frame and that the game frame has the same size with and without them. `online-ui.mjs` saves every online screen in
  `test-results/online-ui/` (1280x720, 1024x768, 1920x1080) and `results.json`.
  In its fleet match one player flies and the other does not, so the losers
  come in a known order: the guest chooses the next ship with the keyboard
  while the host waits, the host's time runs out (the last seconds at all three
  sizes: the page clocks stop for those screenshots), and the guest's last ship
  comes by itself (no choice screen). The test holds the guest's room packets
  for a moment (as a slow network does), so that the 1.5 s battle message of
  both pages ("Choose your next ship." and "Your last ship: Gondi" / "Their
  last ship: Gondi") stays for the screenshots. `game.mjs` plays a full fleet match with
  chosen next ships (the same tick and state hash on both peers during each
  choice, checksums after each one) and leaves during a choice in the jitter
  run. `solo.mjs` saves the local screens (difficulty, the fleet choice of Two
  Players, ship choices, pause, game over with both fleets, the battle message
  after a lost ship, "Choose Your Next Ship", "Your last ship", "Leave this
  game?") in `test-results/local-ui/` and checks that no text leaves the game
  frame and that the battle message does not cover its second line; its waits
  for the computer count only the time in which the game runs. The new and
  changed screens of the fleet matches are also in `test-results/next-ship/`
  (`online-*`, `solo-*`). After each new viewport size the tests wait for two
  animation frames, and they take a screenshot again when its last rows repeat
  its first rows (an old frame of a busy computer).
