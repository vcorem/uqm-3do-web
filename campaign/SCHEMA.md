# Afterlight authored campaign contract

The campaign is JSON data interpreted by `src/state.mjs`. It contains no executable expressions. Dialogue and story content are authored, not assembled from random templates. Original UQM combat runs separately through `src/combat.mjs`; this state engine does not simulate or invent combat victories.

## Top-level document

Required fields: `id` (stable lowercase identifier), `version` (positive integer), `title`, `start`, and the arrays `systems`, `characters`, `scenes`, `quests`, `endings`, `items`, `evidence`. Optional arrays: `factions`, `events`, `upgrades`, `shipyard`. Identifiers use lowercase letters, digits, underscores, or hyphens, start with a letter, and are at most 80 characters. Choice IDs are globally unique.

`start` contains `systemId`, `sceneId`, `credits`, `fuel`, `crew`, optional `ship` (default 5, Earthling Cruiser), and optional boolean `flags`. The starting scene must be at the starting system. The start date is day zero. The start scene opens automatically even when its `entry` is false.

`economy` may set positive `fuelCapacity` (100), `refuelPrice` (2), `repairPrice` (3), `travelFuelScale` (8), `travelDayScale` (12), `distressDays` (7), and `distressFee` (50). Crew capacity comes from the active original UQM ship; it cannot be overridden by campaign economy. The Syreen ship can hold captured crew up to its native maximum.

## Registries

- System: `{id,name,x,y,description,known?,requires?,services?,survey?}`. Coordinates are 0–100. `known` defaults to true. `services` may contain `refuel:true` and `repair:true`. `survey` is `{title,text,brief?,days?,rewards?,mode?,biome?,difficulty?,music?,objectiveLabels?}`; the default duration is one day, and rewards use the effects schema. `objectiveLabels` names the three actual objectives in the briefing, HUD and accessible description; cargo missions must not invent people to rescue. A survey can be completed once.
- Character: `{id,name,species,role,description,portrait?,music?}`. Character IDs can also identify reputation. A `factions` registry may provide additional reputation IDs.
- Item/evidence: `{id,name,description}`. Evidence persists independently of tradeable items.
- Quest: `{id,title,description}`. Active state is separate: `active`, `completed`, or `failed`.
- Ending: `{id,title,paragraphs:[string,...],epilogues?:[{title,requires?,paragraphs}]}`. Ending selection is explicit. `getEnding` and `getView().ending` include only the epilogues whose conditions match the actual final state. The ending journal entry includes those selected epilogues. Scheduled events cannot silently end the main campaign.

## Scenes and choices

Scene: `{id,title,systemId,characterId,act,paragraphs,variants?,music?,requires?,entry?,repeatable?,choices}`. `act` is 1–4. Paragraphs are nonempty strings. Scenes default to once-only and manually enterable. Mark response-only or continuation scenes `entry:false`; these remain available through authored `next` links or combat outcomes. Reviewing the journal is separate from replaying the live conversation.

A scene may define `variants:[{requires,paragraphs}]`. The first matching condition replaces its base paragraphs. `resolveScene` supplies the same text to the interface and journal. If revisiting changes the selected text, the new transcript is also logged. The base paragraphs must remain coherent for old saves and unanticipated combinations.

Choice: `{id,text,next?,requires?,effects?,once?,hidden?,combat?}`. An omitted/null `next` closes the conversation. A non-null next scene must be at the same system and satisfy its requirements **after** the effects. Decisions default to once-only. `once:false` may be used for repeatable transitions without state effects. `hidden:true` hides an unavailable response; otherwise the UI receives a disabled response with a player-facing reason.

The entire choice, including its destination, is validated before committing any state. An impossible target rolls back costs, rewards, flags, history, and time. Leaving an unfinished scene does not mark it complete. An already-visited unfinished continuation can be resumed locally even if its original `entry` is false; this prevents closing a conversation from stranding its quest. A chosen response completes a nonrepeatable source scene.

## Conditions

Exactly one operator per object:

```json
{"all":[{"flag":"heard_lilt"},{"not":{"flag":"sealed"}}]}
{"any":[{"evidence":"echo"},{"item":"receiver"}]}
{"flag":"heard_lilt","is":true}
{"item":"receiver"}
{"evidence":"echo"}
{"quest":"witnesses","status":"completed"}
{"resource":"credits","min":80}
{"reputation":"mara","min":2}
{"visited":"first_contact"}
{"day":{"min":10,"max":30}}
```

`flag.is` defaults to true; absent flags are false. Resources are `credits`, `fuel`, and `crew`. Day bounds are inclusive and either bound can be omitted. Reputation is bounded to −100…100. `all`, `any`, and `not` can be nested.

## Effects

```json
{
  "flags":{"heard_lilt":true},
  "credits":40,
  "fuel":-5,
  "crew":-2,
  "days":1,
  "reputation":{"mara":1},
  "giveItems":["receiver"],
  "takeItems":["battery"],
  "evidence":["echo"],
  "quests":{"witnesses":"active"},
  "unlockSystems":["quiet_star"],
  "journal":"An authored observation recorded exactly as written."
}
```

Resource values are integer deltas; costs cannot take resources below zero. Fuel and crew rewards stop at capacity. Credits stop at one million. `days` cannot be negative. `ending:"ending-id"` concludes the expedition. `ship:0` equips that original Melee ship and its full normal crew complement; subsequent `crew` in the same effects object is an explicit delta. Native ship IDs 0–24 and crew limits are exported as `SHIPS` by `combat.mjs`.

## Equipment and fleet

`upgrades` entries are `{id,name,description,price,systemIds?,requires?,benefits}`. Supported positive integer benefits are `fuelCapacity` (additive units), `travelEfficiency` (percentage fuel reduction), `surveyShielding` (percentage damage reduction), and `surveyBattery` (additional seconds). Combined percentage reductions cap at 75. Ownership uses `flags.upgrade_<id>`. Authored rewards may grant that flag directly. Capacity is applied before simultaneous fuel rewards.

`shipyard` entries are `{id,ship,name,description,price,systemIds?,requires?}`. Purchases are atomic, once-only and respect location, funds and prerequisites. Ship ownership uses `flags.ship_<nativeId>`. A newly purchased hull does not manufacture crew or automatically replace the active vessel. At an equipped port, switching transfers the total available crew into the new hull up to its capacity; the remainder stays on the support tender in `reserveCrew`. Switching back restores that reserve without creating or deleting people. Only deployed crew are exposed to battle losses.

`getView` exposes offer ownership, availability and reasons, `capabilities`, `fleet`, `reserveCrew`, and `totalCrew`. The interface must show which yard offers locked equipment and what it changes. Active fuel capacity and travel previews already incorporate installed benefits.

## Combat

```json
{
  "id":"choose_interception",
  "text":"Intercept the armed tender.",
  "combat":{
    "id":"tender_interception",
    "enemies":[15,18],
    "winSceneId":"interception_won",
    "loseSceneId":"interception_lost",
    "retreatSceneId":"interception_withdrawn"
  }
}
```

Each encounter has 1–14 native enemy ship IDs. Outcome scenes stay in the same system. Retreat defaults to the defeat scene if omitted. Outcome scenes cannot be manually entered before the matching engagement outcome, even if an author forgets `entry:false`. A specifically authored noncombat `next` link can reach the same resolution, allowing genuine diplomatic or technical alternatives to combat. Put result-specific story effects in those outcome scenes. Choice effects are launch costs; they cannot award an ending while launching combat. Mark an encounter's source scene `repeatable:true` if the player must return after a setback or a temporary supply delivery; its individual rewards remain once-only. Defeat and retreat permit another combat attempt, including revisiting setback dialogue. A victory prevents further combat attempts. Reused setback choices do not apply their state effects again; every launched attempt receives a new receipt ID.

Choosing combat creates `state.pendingBattle` and blocks travel and dialogue decisions. `getBattleRequest(data,state)` returns the exact request for `UQMCombat.start()`, including a deterministic seed and unique encounter instance ID. Pass the bridge's result directly to `resolveBattle(data,state,result)`:

```json
{"id":"afterlight-accord:17:tender_interception","outcome":"victory","crew":12,"enemyRemaining":0,"frames":3912}
```

Allowed outcomes are `victory`, `defeat`, `retreat`. State rules validate ID, crew, remaining enemies, and frame count; stale and duplicate results fail. Victory requires surviving crew and zero enemies. Defeat requires zero crew. A defeat costs three recovery days in addition to the encounter day and up to 25 credits, and restores half the ship's normal crew (rounded up). Retreat retains native surviving crew; at least one is restored if the native retreat report has zero. No player-facing win button is supplied. These checks provide state integrity, not anti-cheat protection against someone editing their own local save.

## Scheduled world events

Event: `{id,day,requires?,effects?,title,text,forecast?}`. The optional `forecast` explains the impending practical consequence in the chart and mission journal while the event is relevant and pending. Events trigger once when their day threshold has been reached and their conditions are satisfied. An event whose conditions become true later can trigger at that later date; it is not retroactive. The engine visits event dates and condition day boundaries, so crossing several days at once matches one-day advancement. Events at the same date are ordered by ID. Effects can unlock another overdue event, which is processed in the same date's event pass.

Events cannot themselves advance time or directly conclude the campaign. Their resource losses clamp at zero rather than preventing time from advancing. An event that consumes an absent item remains pending until the item and all requirements are available. Event IDs persist in `state.triggeredEvents`. Announce deadlines in authored dialogue/journal in advance, and keep failures of optional rescues from blocking all main endings.

## Physical surface expeditions

`{type:'launchSurvey'}` creates `state.pendingSurvey:{id,systemId,seed}`. The older `survey` action is an alias for launching; it does **not** grant discoveries. Pass the pending ID and seed, authored `mode` (`survey`, `rescue`, or `relay`), biome and difficulty, and installed shielding/battery benefits to the surface adapter. Rescue mode also receives saved `surveyProgress[systemId]` as `completedTasks`. Preparation and pause do not consume the simulation battery. While it is active, travel and dialogue actions are blocked. The adapter returns `{id,outcome,salvage,hullRemaining,beacons,completedTasks?}` to `resolveSurvey(data,state,result)`.

Outcomes are `surveyed`, `retreated`, and `damaged`. Salvage is an integer from 0 to 60, hull is 0–100, and beacons are 0–3. Success requires all three physical beacons and a surviving lander. It advances at least one day (or the authored `survey.days`), marks that system surveyed, and applies its authored discovery and rewards once. All other outcomes advance one day and permit retry. Damage removes between one and three available crew. A retreat never grants full authored mission rewards. Rescue receipts preserve actually delivered group IDs (unique integers 0–2) in `surveyProgress`, including after a recall or damaged lander. Retrying restores those deliveries; the full story reward remains once-only.

Salvage adds credits, but the persistent `surveySalvage` total caps recovery from each system at 60 across **all** attempts. Repeating partial expeditions cannot create unlimited money. Success prevents further expeditions to the same survey. A saved interrupted attempt can be recalled using `{type:'abandonSurvey'}`; it yields a retreat with no invented salvage and permits a new launch. No outcome is fabricated as a victory by a normal dialogue choice.

## State and API

`createGame(data,{captainName?})` validates data and returns state. `getView(data,state)` returns the current system/scene, choices with `enabled`/`reason`, eligible local `availableScenes`, charted `travelOptions`, active quests, collected items/evidence, native ship data, economy, and ending. `meets(state,condition)` is a pure predicate helper. `previewTravel(data,state,systemId)` returns `days`, `fuel`, `distance`, `allowed`, `reason`.

`applyAction(data,state,action)` and `resolveBattle` return `{ok,state,events,error?,code?}`. Successful state is a new object. Failures return the original state unchanged. Actions:

- `{type:'choose',choiceId}`
- `{type:'enter',sceneId}` (close the current conversation first)
- `{type:'leave'}`
- `{type:'travel',systemId}`
- `{type:'launchSurvey'}` (`survey` is a launch alias)
- `{type:'refuel',amount}` or `{type:'repair',amount}`
- `{type:'purchaseUpgrade',upgradeId}`
- `{type:'purchaseShip',offerId}` (acquire only; explicit switching follows)
- `{type:'switchShip',ship}` (requires a refuel or repair port)
- `{type:'distress'}`
- `{type:'rename',captainName}`
- `{type:'abandonBattle'}` (emergency recovery when a saved native session cannot resume; routes to retreat, never victory)
- `{type:'abandonSurvey'}` (recall an interrupted surface expedition without granting rewards)

Travel fuel is `max(1,ceil(distance/travelFuelScale * (1-travelEfficiency/100)))`; days use `travelDayScale`. Reading, leaving, and reopening text do not advance time. Services exchange resources at stated prices without time advancement. Distress is always available outside active combat or a completed expedition: return to the starting system, pay up to the configured fee, advance the stated days, receive at least 40 fuel (bounded by capacity), and restore at least half normal crew. Rescue works with zero credits and zero crew.

Persistent fields include `location`, `scene`, `ship`, `resources`, optional `reserveCrew` and `surveyProgress`, `flags`, `items`, `evidence`, `quests`, `reputation`, `knownSystems`, `visitedSystems`, `visitedScenes`, `completedScenes`, `usedChoices`, `surveyedSystems`, `surveySalvage`, `battleOutcomes`, `triggeredEvents`, `journal`, `ending`, `revision`, `day`, `pendingBattle`, and `pendingSurvey`. The journal preserves first-seen dialogue and selected decisions; it does not grant rewards when reread.

`validateCampaign(data)` returns `{errors,warnings}`. `validateState(data,state)` returns `{errors}`. `serializeSave(data,state)` returns a versioned JSON document; `loadSave(data,text)` returns `{ok,state,error?}` and rejects unknown IDs, malformed states, wrong versions, impossible resource bounds, dangerous object keys, and files exceeding 4 MiB. Old schema-1 saves may omit `reserveCrew` and `surveyProgress`; rules treat them as zero and empty. Original ownership includes the starting and currently deployed hull. The campaign version remains unchanged; existing authored choice IDs and their historical parents must be preserved to keep saved decisions valid.

## Rules and authored-route verification

Run `node --test campaign/tests/state.test.mjs campaign/tests/content.test.mjs` from the repository root. The first suite exercises transition invariants, atomic rollback, genuine native ship limits, pending-encounter saves, corrupted saves, timed-event ordering, and resource recovery. The second loads the actual campaign JSON and walks ordinary actions through every ending, every principal lead, all authored scenes, all quests, and all surface locations. It also checks long delays, empty budgets, optional sidequest epilogues, different settlement terms, and recovery after each combat setback.

`tests/routes.mjs` exports `ENDING_ROUTES`, `GRAND_TOUR_CHOICES`, `AUTHORITY_TOUR_CHOICES`, and `planItinerary`. These itineraries contain ordinary travel, dialogue, and rescue actions, so the browser integration suite can replay them through visible controls without injecting flags or resources. The grand tour deliberately takes a circuitous route; it is a coverage itinerary, not a recommended order of exploration. The authority tour exercises intrusive or centralized choices and verifies that concrete losses, a council's rejection of the captain's recommendation, and their epilogues remain in the actual history.

State tests use explicit combat/surface receipt fixtures to verify the **campaign consequences** of outcomes. They do not prove a human or simulated pilot won a battle or landed successfully. Those claims require the separate native-combat and physical-lander integration suites. Automated traversal also cannot establish literary quality or replace a fresh-reader playtest.
