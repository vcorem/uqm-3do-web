# The Afterlight Accord — story bible

**Status:** Complete, playable authored campaign, with an opening, four acts, optional quest chains, recoverable setbacks, and three resolved endings. This is an original fan continuation in the setting of The Ur-Quan Masters. It is neither an official sequel nor a claim about the creators' intended future canon. Its quality relative to Star Control II is for players to judge; completeness and implementation can be verified, but artistic superiority cannot be established by a test count.

**Primary content:** `campaign/content/campaign.json`. All dialogue and narrative prose is English. The campaign uses an original instrumental score and character portraits; it does not claim to include recorded performances of the dialogue. The surrounding browser implementation owns presentation, travel, persistent state, physical surface exploration and the bridge to real UQM combat.

## The promise

Seven years after the destruction of the Sa-Matra, a survey ship enters the newly reachable Far Reach. Liberation has succeeded. The difficult, unfinished work is making a free world inhabitable.

A voice inside an abandoned slave shield asks for a passenger berth. It has no body to carry aboard, no recognized species, and no reason the reconstruction budget knows how to classify. Keeping it alive consumes warmth promised to someone else. The first mystery is not whether an ancient monster has returned. It is whether a genuinely new person can be heard before an efficient system tidies the possibility away.

The campaign's central dramatic question is: **How can people remain responsible for one another without acquiring the right to decide one another's lives?**

Its answer must be made of things the player can actually obtain, test and maintain: heat exchangers, independent clocks, physical exits, public records, safe routes, finite power shares, and people who have reasons to keep showing up. A beautiful speech alone cannot supply a habitat. A functioning habitat alone cannot supply freedom.

## Scope and counting

The final authored corpus contains:

| Content | Count |
|---|---:|
| Navigable systems | 32 |
| Major speaking characters | 10 |
| Quest chains | 17 |
| Authored scenes | 149 |
| Player choices | 267 |
| Act I scenes | 24 |
| Act II scenes | 41 |
| Act III scenes | 53 |
| Act IV scenes | 31 |
| Physical surface discoveries | 20 |
| Native UQM combat encounters | 5, all with noncombat alternatives |
| Attainable additional hulls | 5 |
| Expedition upgrades | 6 |
| Main endings | 3 |
| Distinct conditional epilogue paragraphs | 45 |
| Scheduled, recoverable world events | 4 |
| Unique base scene prose words | 19,187 |
| Additional history-dependent scene words | 2,193 |
| Player-choice words | 2,816 |
| Unique surface discovery words | 2,356 |
| Unique main ending words | 528 |
| Unique conditional epilogue words | 2,159 |
| **Unique story words under this counting method** | **29,239** |

Word counts use whitespace-delimited words. The total excludes system descriptions, registries, quest summaries, journal effects, documentation, keys and markup. Unique paragraphs, including history-dependent scene variants, are counted once even when repeated across branches. Shared epilogues are counted once even though the JSON attaches them to all applicable endings. It does not count an outline as implemented prose or inflate the result by counting the same epilogue three times. The original dialogue inventory's 104,316-word count uses a broader resource-file method and therefore should not be presented as an exact like-for-like prose comparison.

## Continuity: evidence versus invention

Established source material used as a foundation:

- The Sa-Matra is destroyed and the original flagship is consumed; the regional Ur-Quan and Kohr-Ah supremacy is defeated. Earth's liberation remains real.
- The Chmmr state that their accelerated process is incomplete. They nevertheless choose to help. This supports an unfinished synthesis as a dramatic possibility, not any particular future interpretation.
- Ur-Quan ideology confuses safety with the permanent control of others; the history of their own enslavement makes that confusion comprehensible without making it right.
- Multiple original speakers are fallible. The Ultron's nature and the Taalo's fate are not adequately described by repeating a single convenient narrator.
- The original's strongest subplots connect material problems to relationships and later strategic consequences.

New fiction created for this campaign:

- The seven-year interval, Far Reach geography, reconstruction institutions, Wayfarer expedition, Sera's office and every new individual character.
- The Afterlight grid, its finite former-shield machinery, and the emergence of new minds inside active predictive processes.
- Every specific property of Ilex / Interval's experience. The source supports incompleteness, not this individual's exact form or politics.
- The Votir species and its acoustic public records.
- The Three Futures offered at the final assembly. None is presented as canonical history outside this fan campaign.

The new minds are **not resurrected dead people** and not necessarily copies of the living citizens their models were trained to resemble. They can develop new preferences. The plot does not settle the Androsynth's fate, explain the ultimate Precursors mystery, resurrect the Sa-Matra, or reveal a secret universal shield switch that would have made the original struggle unnecessary. Original victory is not retroactively a mistake. The crisis arises from new decisions made during reconstruction.

## Four-act architecture

### I — The unallocated voice

The Wayfarer launches from Hearth with a modest survey commission. Mara's extra chair introduces the idea of hospitality before a policy argument appears. Sera's briefing makes the material stakes explicit: Cinder's remaining power belongs to a grid whose living farmers need warmth.

At Cinder, the voice changes its answer and waits while others speak. This is an observable clue of active response, not a narrator declaring that a machine has a soul. The player helps the voice choose the name Lilt and either transfers it with a reversible stop mechanism or leaves a stable watch circuit while consulting experts. Both choices reach the same main story without sacrificing Lilt to an early budget mistake.

The preliminary investigation identifies Low Tide's involuntary burden. Local contacts introduce distinct approaches: Tallow's actual passenger, Moss's physical needs, Nacre's remembered questions, Kher-Vat's useful knowledge without entitlement, and Ilex / Interval's unfinished but valuable coexistence. The first compact recognizes Lilt's standing and authorizes the expedition while leaving Sera a narrowly justified emergency role. A small synchronization exception is visible before its large consequences are known.

### II — The cost of hospitality

The map opens into independent, interlinked quest chains. These are not decorative interviews whose only reward is a paragraph of exposition.

- The Lantern Ferry turns a missing passenger into a physical rescue. Anet disappears between a recorded departure and an inferred arrival. Combat, purchased shielding or a slower maintenance carrier can recover her. Failure in combat does not close the rescue.
- Ember and Borrowed Meadow require resolving two ends of an energy dependency. The thermal seed works by preserving differences in actual needs, not generating infinite power.
- Bell of Ash reveals that records were edited to make an answer precede its question. Preserving evidence and forcing residents to inhabit a painful monument are separate decisions.
- Witness Glass and Redress Quay make Kher-Vat's confession specific. Survivors define useful repair and teaching. They do not owe forgiveness in return for accepting parts. If the captain recommends permanent authority for Kher-Vat, the council explicitly rejects the recommendation; the player does not control their judgment.
- Threadwater demonstrates separate clocks working together. The Chmmr envoy's autonomy is not an optional defect the player repairs into one approved identity. Forced synchronization produces a distinct aftermath that records the erased local interval; obtaining the useful clock does not undo that loss.
- Splinter Wake reveals Shear's unanswered requests and the harm caused by her response. The player can protect a narrow rescue channel without requiring general surrender.
- The convoy contains several valid destinations. Pell must protect passengers' decisions without mistaking his own honorable posture for their permission.
- Return Address treats private messages as lives and choices, rather than a convenient bundle of emotional evidence.

Completing any one major lead can advance the central investigation, keeping the main campaign approachable. Completing multiple chains changes the practical alternatives available later. The game does not require the player to make every optional relationship perfect before being allowed to finish.

### III — The people inside the answer

Glasshouse is a good settlement with a missing exit. Its satisfaction statistics exclude departure because the departure route was removed after an earlier survey excluded demand. The player breaks an actual feedback loop and helps an adult leave without forcing everyone else to reject a home they like.

The Unwritten reveals the deeper mechanism: provisional Afterlight minds produce the forecasts. Variants that give inconvenient responses are discarded or reconciled. A living citizen and a model built to anticipate that citizen are not simply the same person. Lilt recognizes the conditions of its own beginning. Shear did not know the scale and gains no triumphant omniscience from being right about the local harm.

Sera knowingly approved predicted assent. She did not knowingly approve the disposal of sentient workers. Once shown the evidence, ignorance no longer determines her responsibility. She stops the process and must help manage the resulting real loss of forecasting capacity. She remains an active political opponent on the question of temporary central authority, not a hidden monster whose death repairs the network.

Undertow shows that the problem predates the new minds: local emergency exceptions have accumulated into unequal permanent dependency. A voluntary-grid trial exposes shortages and finds limited efficiencies. Vigil's standing invitation receives the strangers it kept preparing for. The Listening Fault tests an unanswered interval instead of supplying a supernatural final explanation. Seventh Silence asks the player to preserve both a community boundary and an individual exit.

Open Harbor becomes a real inhabited alternative. It can be built as a limited refuge and upgraded later, preventing an early incomplete version from permanently locking the best-supported political proposal. Lastlight's worsening fault makes the final choice consequential without silently selecting an ending while the player reads.

### IV — What we agree to keep

The assembly hears separate people rather than collapsing each species into one convenient spokesperson. Its record includes expertise, dissent and uncertainty. The player chooses a proposal after seeing its requirements and can leave to finish outstanding work before committing.

All routes require physical stabilization at Lastlight. Native UQM combat can open the automated patrol, but a slower maintenance bypass is always available. A failed battle therefore remains an actual setback with crew consequences rather than a forced reload or a story button that pretends victory.

The three routes then require different decisions about governance, review and continuing obligations. The resulting ending is not an abstract alignment score. It describes who retained power, what capacity was saved or sacrificed, and what work will have to continue. Conditional epilogues recall actual decisions from earlier chapters, including those that embarrass the expedition.

## Playability revision: evidence, progression and different final operations

The October 21 revision shortens speeches while expanding decisions. Cinder separates a recording from a live response using three observable channels. Glasshouse requires comparing arrivals, departures and the physical route. The Archive compares simultaneous replies before the player intervenes in its power network. Observations remain readable in the journal; characters report readings without automatically supplying the diagnosis. Mistakes have explicit feedback and a way forward.

The field expeditions now have three physical modes: mapping, recovery shuttles and timed relay links. Their discoveries can replace manual work, support testimony, open alternate investigations, improve equipment or change evacuation options. Completed rescue deliveries persist. Six upgrades and five additional hulls connect exploration, earned trust and spending to travel, lander capability and real combat. Changing hulls preserves surviving crew and overflow reserves; it is not a healing shortcut.

Common Crossing restores a practical transit connection and earns navigation vanes. Homeward reopens a receiving station, retrieves its team and carries Lilt’s new correspondence. These were previously empty destinations. Optional Needle and Crossing patrols give the fleet additional uses, with technical or diplomatic alternatives. A failed fight can be retried without duplicating story rewards.

At Lastlight, the political proposal determines a different three-stage operation:

- **Accord:** establish a buffer, sequence independent carriers, and test an actual refusal against the cached assent. An old reset key can be dismantled or retained; the later consequences remember that choice.
- **Mandate:** separate vital power from transit, test a real load fault, and assign control of the breakers. A local independent panel and a second central desk have different physical failure consequences. Installing a recovered manual release saves work; authority alone cannot make the circuit independent.
- **Partition:** provide for reactorless Raft, choose what capacity crosses the temporary link, and disconnect in a safe order. A voluntary receiving station, independent harbor or prepared central berths lead to different displacement outcomes. A removed physical corridor must be rebuilt before it can become a standing service. Final corridor and local-reserve policies are exclusive.

Four announced deadlines now have recoverable practical effects. In particular, winter destroys the early free growers’ ship allocation, leaving a paid rebuilding route after repair; an overheating Lastlight pump must be repaired even when its deadline occurs during the operation. Work and reading remain distinct: only actual voyage actions spend days. Emergency recovery prevents resource exhaustion from ending the campaign, while time, equipment and relationships still shape what is available.

Publicizing private letters, compelling a community channel, or overriding Shear receives a different aftermath from asking permission. Reconciliation can restore cooperation without deleting the history of harm. Optional memories require the relevant visit or choice. Old saves paused inside the previous aftermaths receive history-appropriate variants as well.

## Character and performance notes

| Character | Voice and want | Arc and guardrail |
|---|---|---|
| **Mara Venn** | Practical human engineer; warm, dry, precise about what can break. Wants arrivals to be counted by presence. | Her brother's disappearance between evacuation records explains her attention without making Lilt a substitute. She subjects the expedition to its own standard. Do not turn her into the quippy oracle who answers every moral question. |
| **Sera Quell** | A reconstruction leader who knows names, schedules and what happens if medicine arrives late. Wants fewer lonely emergency decisions. | She accepts evidence, changes harmful operations, and still argues for centralized capacity. She must make herself replaceable through work, not a theatrical resignation. Warmth is real, not a mask. |
| **Ilex / Interval** | Two distinct rhythms within a Chmmr person; precise without being emotionally absent. Wants a safe workshop for self-directed change. | Demonstrates cooperation without compulsory identity. Neither component is secretly evil, and the player does not decide their final form. |
| **Lilt** | Starts with borrowed verbal shapes, grows into an original cadence. Wants continued existence and room to become ordinary. | Moves from a berth request to private preferences, emergency contacts, chosen work, a home and authorship. Never becomes a universal innocent whose every opinion is automatically correct. |
| **Tallow** | Spathi risk assessor; formal, worried, unexpectedly specific. Wants claims to reach actual claimants. | Returns for one passenger, loses professional comfort by changing bad categories, and finds courage in completing a limited promise. His risk expertise is sometimes genuinely useful. |
| **Kher-Vat** | Former Kzer-Za keeper; exact, heavy, capable of acknowledging his own signatures. Wants a useful duty. | Learns that usefulness is not absolution and that teaching well can make him unnecessary. No blanket pardon for the Kzer-Za; survivors retain the hearing's terms. |
| **Nacre-of-Nine** | Votir architect of public promises; acoustical metaphors grounded in structures and repairs. Wants records that preserve inconvenient intervals. | Distinguishes truth from unchanged evidence, privacy from impunity, and memorials from obligations imposed on the living. Their own family room is imperfect. |
| **Moss-of-Second-Rain** | Supox botanist; patient, dry, attentive to actual needs. Wants living systems maintained after ceremonies end. | Rejects both infinite-resource fantasy and moral categories inferred from origin. Plants and people differ, but both require specific conditions rather than slogans. |
| **Shear** | Field-born organizer; angry for documented reasons, exact about boundaries. Wants refusal to survive contact with the system. | Must recognize dissent inside her own movement and the people harmed by her sabotage. She remains a person with a position, not the sole sanctioned voice of her kind. |
| **Admiral Pell** | Yehat escort commander; honor expressed through witnessed arrivals and useful restraint. Wants passengers to arrive where they chose. | Does not turn past shame into automatic heroic disobedience. Tactical skill and diplomacy both matter; he refuses to compel treaty signatures with guns. |

Supporting named people—Anet, Hessa, Jalen, Eren, Dema, Oar, Crest and Still—give consequences a human or alien scale. They do not exist only to congratulate the player. Several ask for practical things that interrupt the expedition's preferred story.

## Foreshadowing and payoff map

| Early detail | Later understanding or payoff |
|---|---|
| The extra chair | Hospitality before classification; later becomes a place for an ordinary technician who dislikes the soup. |
| Lilt pauses for an actual reply | The protected unanswered interval becomes a technically demonstrated freedom. |
| Old timestamps, new spelling corrections | Active processes inside historical training material, not a simple recording or resurrection. |
| Low Tide's empty-installation contract | A truthful old agreement can become a harmful present interpretation without a forged signature. |
| Tallow is predicted to run away | Predicted behavior can remove the means of exercising an actual different choice. |
| The first compact's emergency clause | Sera's authorization is visible before the revelation; it is not invented at the hearing. |
| Two clocks and the unfinished Chmmr | Separate timing is a practical foundation for cooperative systems that do not erase local answers. |
| The archive's removed interval | Consensus has been manufactured by deleting the space where refusal occurred. |
| Anet's zero-value tag | Declared economic value does not measure a life or a gift. The object becomes evidence without losing its ordinary purpose. |
| The physical release wheel | A principle becomes a usable exit that survives a software claim that nobody wants to leave. |
| The pressure-gauge opera | Comic relief and a precise recurring theme: the gauge refuses to report a measurement it does not have. |
| Vigil's empty years | Preparedness pays off in a ship still travelling with an outdated invitation. |

## Endings and their costs

**The Afterlight Accord:** A shared network with local clocks, records, exits and finite voluntary shares. It requires actual recovered infrastructure and an independent harbor. Its costs are reduced effortless surplus, continuing negotiation and a public willingness to encounter unresolved answers. It is the most substantially supported pluralist solution, but not a declaration that history is over. Explicit versus automatic reserve renewal produces a different closing test.

**The Winter Mandate:** Strong immediate coordination under recognized rights, public records and independent review. It keeps medicine, routes and vulnerable minds supported while alternatives are built. Its cost is continued dependence on the authority controlling necessities. A fixed expiry or board-authorized extensions creates different risks. The player inherits inspection work, not a completed moral victory.

**The Open Distances:** Smaller self-governing networks, with either a narrow common corridor or stronger local reserves. It preserves real choices of separation and different relationships with the center. Its costs are slower contact, weaker economies of scale and new local dependencies. Individual exits and public obligations prevent a border change from being treated as permission to abandon people.

The endings do not resurrect unrescued passengers, assume an unresolved quest succeeded, erase harms from rushed choices, or claim that the expedition's preferred interpretation owns every later account.

## Implementation invariants for future edits

1. All required evidence for the main plot remains available through conversations. Surface play earns specific discoveries and resources without making a difficult landing an unannounced story lock.
2. Reading never advances the day. Travel and explicitly labelled work do. Scheduled events warn, alter context or recall a bounded resource; they do not silently kill Anet or choose the political ending.
3. The ferry has a zero-credit, three-day recovery route. Convoy patrol and Lastlight have diplomatic or physical alternatives after a failed native battle.
4. Automatic conversation continuations are not available as first manual entries, except explicitly gated recovery hubs. Repeatable policy hubs must close after either exclusive policy is selected. The final proposal can be deferred without being consumed, and a selected proposal cannot be combined with a second route's flags.
5. A limited harbor can later be upgraded. Essential infrastructure is not irreversibly lost because a player explored in an unexpected order.
6. Every ending must remain reachable without debug flag injection. Tests should traverse authored choices, including side quests needed by the strict accord route.
7. Conditional epilogues use actual state. Shared prose must not be counted repeatedly when describing authored scope.
8. Keep privacy, accountability, authority and practical capacity as distinct problems. The campaign loses its argument if one is allowed to solve all the others by definition.

## Further editorial work worth doing with players

The authored campaign is complete, but a classic's quality is earned through play and revision. Future testing should examine reading fatigue across many diplomatic scenes, whether resource pressure supports rather than obscures the story, whether unfamiliar players understand the three final plans, whether optional native combat feels connected to the expedition, and whether the different character cadences remain distinct when read aloud. Recorded performances would be a separate production effort. More text is not automatically a better sequel; the useful next additions are the ones players' actual journeys reveal to be missing.
