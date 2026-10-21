# Original campaign study: evidence and design consequences

This is the research record for **The Afterlight Accord**, an original fan continuation of The Ur-Quan Masters. It is not a claim to know every line from memory, an official continuation, or a substitute for testing the new campaign. Dialogue references beginning `comm/` are relative to `sc2/content/base/`; abbreviated dialogue filenames are in their species' subdirectory. The C files `gameev.c`, `gameev.h` and `globdata.h` are under `sc2/src/uqm/`. Named dialogue labels provide stable anchors even if line numbers change.

## Corpus and actual coverage

The local original dialogue corpus contains **27 text files, 16,604 lines, and 104,316 whitespace-delimited words** under `sc2/content/base/comm/`. That count includes player responses, resource labels, alternate greetings, outtakes, and other markup; it is not a count of unique spoken prose. The directories include multiple conversations for some species, rather than 27 distinct species. Dialogue labels, companion C state machines, planetary generation scripts, and the scheduled-event implementation were inspected.

The close reading covered the full Orz and Mycon conversations; the Ur-Quan and Chmmr principal conversations; the Arilou material on humans, dimensions, the Talking Pet and portals; the Kohr-Ah and Melnorme accounts of the Sentient Milieu and the Doctrines; the Dnyarri's lies, captivity and final-battle instructions; the Syreen revenge chain; the Yehat revolution and rebel dialogues; the Slylandro programming problem; the Ultron's contradictory accounts and scientific analyses; the Androsynth ruins reports; and the PC ending's narrated story. Representative material was studied for the remaining speakers, alongside their quest and event hooks. **Not every optional greeting, outtake, and branch in the entire 104,316-word corpus received equal close reading.** The count describes the inventory, not a claim that a word-count script constitutes reading.

The local source incorporates UQM's combined PC/3DO resources. In particular, `base/cutscene/ending/final.txt` explicitly identifies itself as the PC ending script. It is useful corroboration of the resolution, but should not be presented as a transcription of the 3DO video.

Useful repository entry points for checking the central claims:

| Claim or mechanic | Exact source location |
|---|---|
| Chmmr process remains incomplete | `sc2/content/base/comm/chmmr/chmmr.txt:145`, `WE_ARE_FREE`; help despite unreadiness at line 180 |
| Ultron is a functioning mental amplifier | `sc2/content/base/comm/starbase/starbase.txt:989`, `ABOUT_ULTRON_3` |
| Conflicting Taalo accounts | `sc2/content/base/comm/melnorme/melnorme.txt:967`, `OK_BUY_HISTORY_4`; `sc2/content/base/comm/orz/orz.txt:526`, `FRIENDLY_PLACE` |
| Arilou's dimensional warnings | `sc2/content/base/comm/arilou/arilou.txt:299`, `IDF_PARASITES`; `NOT_NOW` at line 311 |
| Yehat revolution and its eventual payoff | `sc2/content/base/comm/yehat/yehat.txt:270`, `WE_REVOLT`; `sc2/content/base/comm/yehatrebels/yehatrebels.txt:187`, `YEHAT_CAVALRY` |
| Slylandro's programming mistake | `sc2/content/base/comm/slylandro/slylandro.txt:564`, `UH_OH` |
| Dynamic campaign events | `sc2/src/uqm/gameev.h`, `sc2/src/uqm/gameev.c`; base victory timetable in `sc2/src/uqm/globdata.h:173` |
| Androsynth ruins investigation hooks | `sc2/src/uqm/planets/generate/genand.c` and its referenced report resources |

## What the original actually does

### The enemy's rationale changes the meaning of the war

`comm/urquan/urquan.txt`, `URQUAN_STORY`, describes solitary predators who learned cooperation, joined the Sentient Milieu, suffered Dnyarri control, and converted a determination never to be enslaved into a universal mandate to enslave others. The phrase that obtains the account is a moral question, not a password presented as a mystical key. The Kohr-Ah explain the same trauma through another doctrine in `comm/kohrah/kohrah.txt`, `YOU_DIE` and `THEN_DIE`. Their offer of rebirth does not soften their extermination; it shows the self-sealing logic that makes ordinary persuasion fail.

The Melnorme's nine `OK_BUY_HISTORY_*` accounts supply connective evidence: Taalo patience first helped the Ur-Quan join a cooperative society; Kzer-Za discovered the Dnyarri's aversion to sharing extreme pain; the Excruciators enabled resistance; genetic debasement made surviving Dnyarri into Talking Pets; the Sa-Matra decided the first Doctrinal War. `gameev.c` turns the doctrine into a moving extermination fleet if time expires. The villain is therefore history, motive, character, mechanics, and deadline together.

**Consequence for Afterlight:** the opposition must have a coherent ethic whose damage the player can demonstrate. Revealing a difficult childhood cannot cancel accountability. A solution needs practical institutions and physical work, not a speech that makes everyone nice.

### Narrators have beliefs, interests, and blind spots

The Melnorme dismiss the Ultron as a fraud in `OK_BUY_EVENT_8`. The Druuge's account at the bomb planet, `NOT_GET_BOMB`, inadvertently describes an experience their salesmanship cannot fully explain. Hayes's successive `ABOUT_ULTRON_0` through `ABOUT_ULTRON_3` reports revise his initial contempt as repairs reveal measurable function; the final report identifies a Precursor mental amplifier incompatible with the minds his technicians tested. Calling the Ultron merely a placebo contradicts the full evidence.

The Melnorme call the Taalo eliminated in `OK_BUY_HISTORY_4`; Orz's `FRIENDLY_PLACE` says they spread into another space while the Dnyarri pursued them. The translation explicitly declares its accuracy unknown. The Androsynth ruins contain destroyed cities without bodies, fragmented dimensional research, and Bukowski's frightening injuries; they do not provide a neat diagram of what happened. `arilou.txt`, `IDF_PARASITES` and `NOT_NOW`, gives a warning from another interested speaker, not omniscient confirmation of every interpretation.

**Consequence:** distinguish verified physical evidence, a speaker's testimony, and inference in the journal. Do not explain away Orz, the Precursors, or the Arilou to make the new plot look larger. The new campaign may introduce its own answerable mystery while leaving those older ones genuinely open.

### Liberation has costs without being a mistake

The shielded Chenjesu/Mmrnmhrm explain the process and its intended duration in `chmmr.txt`, `HYBRID_PROCESS`, `LONG_TIME`, and `DANGER_TO_US`. After the Sun Device is used, `WE_ARE_FREE` says that the process is incomplete despite emergence. `WILL_HELP_ANALYZE_LOGS` says they were not ready but chooses to help. Their willing alliance, the Captain's destruction of the flagship, and the successful lifting of Earth's shield remain achievements.

**Consequence:** Ilex/Interval can have an unfinished negotiation between inherited modes of thought. They must not be revealed as a defective villain or proof the player should have left Earth enslaved. The new crisis arises from *new reconstruction choices applied to a remote former prison network*, not from secretly making the original victory evil.

### Relationships reshape the strategic map

`gameev.h` enumerates **18 scheduled event types**. `gameev.c` gives the Zoq-Fot-Pik six months between distress and destruction; grows the probe problem over time; removes the allied Spathi sphere when they voluntarily shield themselves; advances Pkunk migration; moves the Mycon fleet into and out of the Organon ambush; and simulates costly Utwig/Supox intervention. `YEARS_TO_KOHRAH_VICTORY` is four in `globdata.h`; the intervention grants an additional year in the victory event handler. Time is a strategic resource with authored consequences.

The Shofixti return is not merely another ship unlock. `yehat.txt`, `WE_REVOLT`, uses a living survivor to break the starship clans' rationalization of the Queen's command. `yehatrebels.txt`, `ABOUT_ROYALTY`, explains the institution behind their loyalty. `YEHAT_CAVALRY` pays off the apparently disastrous Pkunk absorption with a new queen and aid at the final battle. That reversal rewards concern for a minor-looking subplot.

**Consequence:** Afterlight's optional rescues must matter in later negotiations, available routes, and epilogues. A completed quest is not only a checked box. The consequences should remain legible, with time-sensitive choices clearly signalled rather than invisible unwinnable traps.

### Objects connect otherwise separate stories

The Sun Device chain combines Mycon biology, evidence of Syra's destruction, Talana's grief and revenge, a recovered fleet, an ambush, and the Chmmr's energy requirement. The Ultron chain links Pkunk generosity, Thraddash culture, Druuge trade, Utwig belief, and the main military timetable. The Taalo device chain makes an uncertain ancient story into a concrete protection with dangerous limits.

The player discovers why an object matters in a human or alien relationship before it becomes an inventory solution. Hayes gives analyses that change with evidence; he is not only a shop menu.

**Consequence:** the new campaign's lattice, listening seed, thermal ferry, witness archive and independent clock must each have a practical use and a moral context. Several solutions should be possible. No finale should hinge on a single unmarked object that can be accidentally discarded hours earlier.

### Humor carries information

Fwiffo's opening confession gives coordinates, a cipher, a character, and an account of a violent misunderstanding through a spectacularly self-protective surrender. Spathi cowardice has consistent priorities; it is not random jokes. The Slylandro speak warmly and are horrified when the player walks them through their own program's consequences. The key revelation in `UH_OH` follows information the player has already been allowed to inspect: replication priority 999 is applied to the ship currently selected for contact. The satire targets a mundane interface mistake and an inadequate product, not innate alien stupidity.

The Yehat's language, cultural honor and partisan loyalties differ from their individual capacity for affection. Orz's friendliness is both funny and structurally unsettling. Mycon fragments sometimes sound like engineering instructions embedded in inherited religious memory. Druuge legal language makes exploitation explicit while its agents call it fair dealing. Umgah humor can harm people. The range of tone is part of the world, rather than permission to put the same quippy voice in every mouth.

**Consequence:** Tallow's risk vocabulary must sometimes discover a real danger. Nacre's seemingly absurd acoustical etiquette has a useful mechanism. Mara can be funny because she repairs physical things under impossible promises; she should not undercut every emotional scene. Sera is allowed warmth without that warmth being a disguise.

## Continuity boundaries for the new work

1. The Sa-Matra was destroyed; the original flagship was consumed. The Ur-Quan and Kohr-Ah lost their regional supremacy. Earth's liberation is real.
2. Seven years is a chosen interval for an original fan continuation, not a claim about an official sequel timeline.
3. The new expedition travels a newly reachable region. Original world names and characters can be remembered without silently relocating their homes.
4. Chmmr unfinished synthesis is a supported opening for new fiction, not evidence for a specific canonical future.
5. A new Kzer-Za individual may change. This does not mean all Kzer-Za have repented, all victims forgive them, or the Kohr-Ah have become harmless.
6. The Afterlight entities are *new minds*. They are not proof that dead loved ones have been resurrected, and they do not secretly contain every person who ever lived behind a shield.
7. The new network was assembled after the war from limited former shield machinery and new reconstruction equipment. It is not a hidden master switch whose destruction would have solved the original game.
8. The plot does not settle the Androsynth's fate, identify the Precursors' ultimate fate, or turn the Arilou and Orz into interchangeable villains.
9. Optional original-game states are not all forced into a single exhaustive history. Recollections use the established victory and selected broad consequences, not a claim that every original side quest necessarily happened.
10. A substantial new playable campaign can be tested for completeness, meaningful decisions and craft. Its quality relative to a beloved classic remains a judgment for players, not a metric established by word count.

## New creative direction

The original asks whether freedom can be won against those who abolish it in the name of safety. **The Afterlight Accord asks how people can keep that freedom while becoming responsible for lives that depend on them.** The answer is neither abandoning everyone to private risk nor allowing a well-intentioned authority to decide what every life should be. The player must build a workable third arrangement out of imperfect allies, limited energy, accountable promises, and the right to leave.

The final solution must therefore be earned through the same broad grammar that makes the original compelling: explore, listen, obtain evidence, help someone specific, accept a cost, and discover that the relationship has changed what is possible.
