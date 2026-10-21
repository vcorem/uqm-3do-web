# Original campaign study: systems, structure, and fairness

This study is based on the repository's actual UQM C source and English dialogue, not a recollection of a walkthrough. Paths below are repository-relative. The original communication corpus contains 27 dialogue files, 3,451 tagged blocks, approximately 98,143 dialogue words, and 16,604 lines. Those counts include repeated greetings, player responses, optional branches, and outtakes. They are a scope reference, not a claim that word count determines quality.

## A changing world

`sc2/src/uqm/gameev.c` initializes daily race updates, the monthly Arilou opening, Kohr-Ah victory, and Slylandro growth. `clock.c` begins on February 17, 2155; `globdata.h` sets four years to initial Kohr-Ah victory. Starting the Utwig/Supox mission adds a year before genocide. Moving spheres, timed ambushes, migration, and changes to alliance status make politics part of navigation.

Specific examples worth preserving as design principles:

- Arilou access opens on the seventeenth for three days each month (`gameev.c`, `arilou_entrance_event` and `arilou_exit_event`).
- Zoq-Fot-Pik distress has a six-month response window; an unanswered distress call changes the world (`ZOQFOT_DISTRESS_EVENT`, `ZOQFOT_DEATH_EVENT`).
- Spathi shield themselves and leave the alliance (`SPATHI_SHIELD_EVENT`).
- Shofixti restoration changes ship availability and crew cost (`SHOFIXTI_RETURN_EVENT`).
- Slylandro pressure increases every 182 days until its cause is solved, then declines on a shorter cadence (`SLYLANDRO_RAMP_UP`, `SLYLANDRO_RAMP_DOWN`).
- The Mycon travel to Organon, suffer losses, and return, rather than simply flipping a quest flag (`mycon_mission`).

For Afterlight, time should produce a small number of visible, authored changes and optional fail-forward consequences. Reading and checking the journal should never consume time. Every serious deadline needs advance explanation and a persistent record; the main ending should not silently become impossible because one optional warning was missed.

## Evidence, personality, and geography

The Syreen chain combines a mistaken interpretation of catastrophe, material evidence, political motivation, a spatial deduction, fleet recovery, and an ambush shaped by the enemy's psychology. Talana needs proof concerning the Deep Children. The vault clue limits the search by star color and travel distance. Mycon protection of their offspring explains the ambush's success. Their departure enables acquisition of another important device.

Sources: `sc2/content/base/comm/syreen/syreen.txt` (`WHAT_PROOF`, `HORRIBLE_TRUTH`, `DONT_KNOW_WHERE`, `OK_FOUND_VAULT`, `HELLO_AFTER_AMBUSH_1`); `sc2/src/uqm/comm/mycon/myconc.c` (`TrickMycon`); `gameev.c` (`mycon_mission`).

Afterlight's major chains should similarly contain a discovery that reinterprets an earlier encounter, a decision arising from a character's motives, and a lasting world change. Evidence should have provenance. The journal should distinguish direct observations from another character's claim. A clue is stronger when it teaches the player something about a people as well as a destination.

## Optional does not mean disposable

Restoring the Shofixti adds a ship, affects recruitment costs, and permits an encounter with the Yehat that can start a revolt. The player brings a living escort captain rather than an abstract completion token. Sources: `sc2/src/uqm/comm/shofixt/shofixt.c`, `sc2/src/uqm/comm/yehat/yehatc.c`, and the Shofixti/Yehat events in `gameev.c`.

Optional Afterlight chains should affect at least two of relationships, supply, travel, combat options, political choices, or epilogues. A failed rescue can create displaced survivors and a harder negotiation instead of deleting every route forward.

## Antagonism and uncertainty

Ur-Quan and Kohr-Ah dialogue explains historical trauma, the Dnyarri, the Taalo, and incompatible doctrines without excusing enslavement or extermination. Their interpretation of security is the conflict, not merely an excuse to deliver a final battle. Sources: `sc2/content/base/comm/urquan/urquan.txt` and `kohrah/kohrah.txt`.

Orz dialogue uses repeated translation substitutions and the distinction between one entity and its apparent bodies. This supports inference while preserving uncertainty (`orz.txt`, especially `GENERAL_INFO_ALLY_1` and `ABOUT_US_1`–`ABOUT_US_3`). The central campaign mystery should resolve, but not every curiosity in the setting needs an omniscient explanation.

Afterlight therefore preserves the original liberation. Its central problem comes from reconstruction, incomplete knowledge, and the autonomy of minds emerging during isolation. It should avoid invalidating the original victory through an unexplained stronger master of the old villains.

## Economy, morality, and recovery

Melnorme trade makes exploration economically useful: biological information and rainbow worlds can fund technology and fuel. Their rescue code can take equipment but ultimately provides charity when there is nothing to surrender. See `sc2/src/uqm/comm/melnorm/melnorm.c`, `StripShip`, and `sc2/content/base/comm/melnorme/melnorme.txt`, `CHARITY`.

Druuge transactions record crew sold, alter future crew prices, and provoke specific Hayes reactions. See `sc2/src/uqm/comm/druuge/druugec.c`, `ExitConversation`, and `sc2/content/base/comm/starbase/starbase.txt`, bulletins 27–30. These are concrete consequences rather than a generic morality score.

Afterlight should track who was promised what, whether consent was obtained, and what evidence was shared or withheld. Different characters can evaluate the same decision differently. Fuel exhaustion and battlefield defeat must have costly recovery paths, not permanent softlocks. Essential information must not be available only through a vendor the player can no longer afford.

## Presentation and music

The original uses alien-specific themes, navigation themes, and quieter location music. The resource map `sc2/content/uqm.rmp` routes these through `MUSICRES`; `sc2/content/addons/3domusic/3domusic.rmp` demonstrates Ogg replacement tracks. The actual decoder registry in `sc2/src/libs/sound/decoders/decoder.c` supports WAV, MOD, and Ogg. `music.c` streams looped music separately from speech.

The available local tools include Python with NumPy and ffmpeg with PCM, FLAC, and Vorbis encoding. Original compositions can be authored as deterministic note/chord/rhythm data, rendered with original synthesized instruments, and encoded as reusable Ogg assets. Preserve composition sources, avoid importing original melodies, validate clipping and loop boundaries, and listen to the rendered result. Technical audio checks alone cannot establish musical quality.

## Validation standard

Mechanical checks should prove reference integrity, valid conditions, atomic choices, one-time rewards, save/load correctness, deterministic time ordering, recovery from zero resources, native battle outcome matching, and reachable endings. Campaign traversal should cover direct, exploratory, distrustful, failed-rescue, low-resource, and different-ending routes. Major revelations need more than one prior clue. A successful automated traversal does not establish literary quality; a fresh-reader playtest remains necessary to find assumptions only the author understands.

The implemented Afterlight systems are a new authored campaign framework around genuine UQM combat. The study's description of the original game's continuously moving spheres should not be mistaken for a claim that every original strategic system has been recreated in the new campaign.
