import test from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { createGame, validateCampaign, validateState, applyAction, getView, previewTravel,
  serializeSave, loadSave, resolveBattle, resolveSurvey, getEnding } from '../src/state.mjs';
import { OPENING_CHOICES, LEAD_CHOICES, INVESTIGATION_CHOICES, ENDING_ROUTES,
  GRAND_TOUR_CHOICES, AUTHORITY_TOUR_CHOICES, thirdActRoute, planItinerary } from './routes.mjs';

const data = JSON.parse(readFileSync(new URL('../content/campaign.json', import.meta.url), 'utf8'));
const firstAct = OPENING_CHOICES;

/** These are normal campaign actions; no flag/resource assignment is permitted. */
function perform(state, action) {
  const result = applyAction(data, state, action);
  assert.equal(result.ok, true, `${JSON.stringify(action)}: ${result.error}`);
  assert.deepEqual(validateState(data, result.state).errors, []);
  return result.state;
}

function route(ids, state = createGame(data)) {
  return planItinerary(data, ids, state).state;
}

function travel(state, systemId) {
  if (state.scene) state = perform(state, { type: 'leave' });
  if (state.location === systemId) return state;
  if (!previewTravel(data, state, systemId).allowed) state = perform(state, { type: 'distress' });
  return state.location === systemId ? state : perform(state, { type: 'travel', systemId });
}

function waitUntil(state, day) {
  while (state.day < day) state = perform(state, { type: 'distress' });
  return state;
}

/** Receipt fixtures test campaign routing; real combat and lander input are
 * exercised independently. They never assign flags, rewards, or resources. */
function battleResult(state, outcome) {
  assert.ok(state.pendingBattle, 'the authored choice launched a battle');
  const result = resolveBattle(data, state, { id: state.pendingBattle.battleId, outcome,
    crew: outcome === 'defeat' ? 0 : 1,
    enemyRemaining: outcome === 'victory' ? 0 : state.pendingBattle.enemies.length, frames: 500 });
  assert.equal(result.ok, true, result.error);
  assert.deepEqual(validateState(data, result.state).errors, []);
  return result.state;
}

function engagementScenarios() {
  const mandate = ENDING_ROUTES.mandate, junction = mandate.indexOf('a4_junction_bypass');
  return [
    { opening: OPENING_CHOICES, fight: 'a2_ferry_battle', regroup: 'a2_ferry_setback_leave',
      alternative: ['a2_ferry_labor', 'a2_ferry_complete'], victory: ['a2_ferry_complete'], flag: 'anet_saved' },
    { opening: [...thirdActRoute(), ...INVESTIGATION_CHOICES, 'a2_shoal_records', 'a2_pell_depart'],
      fight: 'a3_convoy_fight', regroup: 'a3_convoy_regroup',
      alternative: ['a3_convoy_record', 'a3_convoy_finish'], victory: ['a3_convoy_finish'], flag: 'pell_supports' },
    { opening: mandate.slice(0, junction), fight: 'a4_junction_battle', regroup: 'a4_junction_regroup',
      alternative: mandate.slice(junction), victory: mandate.slice(junction + 1), flag: 'junction_ready' },
    { opening: OPENING_CHOICES, fight: 'a2_breakwater_fight', regroup: 'a2_breakwater_regroup',
      alternative: ['a2_breakwater_tug', 'a2_breakwater_complete'],
      victory: ['a2_breakwater_salvage', 'a2_breakwater_complete'], flag: 'breakwater_open' },
    { opening: [...thirdActRoute(), ...INVESTIGATION_CHOICES, 'a3_crossing_stagger', 'a3_crossing_complete'],
      fight: 'a3_crossing_guard_fight', regroup: 'a3_crossing_guard_regroup',
      alternative: ['a3_crossing_guard_decoy', 'a3_crossing_guard_finish'],
      victory: ['a3_crossing_guard_salvage', 'a3_crossing_guard_finish'], flag: 'crossing_secured' },
  ];
}

test('authored campaign has valid identifiers, data types, references, and complete dialogue', () => {
  const check = validateCampaign(data);
  assert.deepEqual(check.errors, []);
  assert.deepEqual(check.warnings, []);
  assert.equal(data.systems.length, 32);
  assert.ok(data.characters.length >= 10);
  assert.ok(data.scenes.length >= 149, 'the expanded multi-act campaign retains all authored encounters');
  assert.ok(data.scenes.reduce((count, scene) => count + scene.choices.length, 0) >= 267,
    'the expanded campaign retains its decision branches');
  assert.equal(data.quests.length, 17);
  const targets = new Set(data.scenes.flatMap(s => s.choices.flatMap(c =>
    [c.next, c.combat?.winSceneId, c.combat?.loseSceneId, c.combat?.retreatSceneId].filter(Boolean))));
  for (const scene of data.scenes) if (scene.entry === false && scene.id !== data.start.sceneId)
    assert.ok(targets.has(scene.id), `Continuation ${scene.id} has an authored incoming link`);
  const publicContinuations = data.scenes.filter(s => targets.has(s.id) && s.entry !== false);
  assert.deepEqual(publicContinuations.map(s => s.id).sort(), ['a2_ember', 'a4_partition_terms'],
    'only authored recovery hubs are also public entries');
  for (const scene of publicContinuations) assert.ok(scene.requires, `${scene.id} retains its prerequisite gate`);
});

test('public recovery hubs respect their prerequisites before and after the winter deadline', () => {
  const early = travel(route(OPENING_CHOICES), 'ember');
  assert.equal(applyAction(data, early, { type: 'enter', sceneId: 'a2_ember_cold' }).ok, false);
  assert.equal(applyAction(data, early, { type: 'enter', sceneId: 'a2_ember_return_repair' }).ok, false);
  let late = travel(waitUntil(early, 45), 'ember');
  assert.equal(late.flags.winter_warning, true);
  assert.equal(applyAction(data, late, { type: 'enter', sceneId: 'a2_ember' }).ok, false,
    'an incoming recovery link cannot bypass the frozen greenhouse');
  assert.ok(!getView(data, late).availableScenes.some(s => s.id === 'a2_ember'));
  late = route(['a2_ember_restart_work'], late);
  assert.equal(late.scene, 'a2_ember', 'the real repair reopens the public seed encounter');
  assert.equal(late.flags.winter_repaired, true);
  late = route(['a2_ember_wait', 'a2_ember_depart'], late);
  assert.ok(late.items.includes('thermal_seed'));
});

test('the public partition recovery hub waits for a real separation and repairs a missing corridor', () => {
  const choices = ENDING_ROUTES.partition, selection = choices.indexOf('a4_choose_partition');
  const proposal = perform(route(choices.slice(0, selection)), { type: 'leave' });
  assert.equal(applyAction(data, proposal, { type: 'enter', sceneId: 'a4_partition_terms' }).ok, false);
  const selected = route(['a4_choose_partition'], proposal);
  assert.equal(applyAction(data, selected, { type: 'enter', sceneId: 'a4_partition_terms' }).ok, false,
    'choosing partition does not substitute for physically separating its circuits');
  const ratification = choices.indexOf('a4_partition_corridor');
  let ready = route(choices.slice(selection + 1, ratification)
    .map(id => id === 'a4_partition_bridge' ? 'a4_partition_store' : id), selected);
  ready = travel(ready, 'assembly');
  ready = perform(ready, { type: 'enter', sceneId: 'a4_partition_terms' });
  assert.equal(applyAction(data, ready, { type: 'choose', choiceId: 'a4_partition_corridor' }).ok, false,
    'radio-only separation cannot promise an absent physical corridor');
  const beforeRepair = ready.day;
  ready = route(['a4_partition_rebuild_corridor'], ready);
  assert.equal(ready.day, beforeRepair + 2);
  assert.equal(ready.scene, 'a4_partition_terms');
  ready = route(choices.slice(ratification), ready);
  assert.equal(ready.ending, 'partition');
  assert.equal(ready.flags.partition_bridge_rebuilt, true);
});

test('the authored first act is playable using ordinary choices and interstellar travel', () => {
  const state = route(firstAct);
  assert.equal(state.flags.act2, true);
  assert.equal(state.flags.lilt_aboard, true);
  assert.equal(state.quests.firstvoice, 'completed');
  assert.equal(state.quests.allocation, 'completed');
  assert.ok(state.items.includes('field_lattice'));
  assert.ok(state.evidence.includes('new_life'));
  assert.ok(state.day > 0);
  assert.deepEqual(loadSave(data, serializeSave(data, state)).state, state);
});

test('the authored technical ferry rescue needs no fabricated combat victory', () => {
  let state = route([...firstAct, 'a2_ferry_manual', 'a2_ferry_complete']);
  assert.equal(state.quests.smallcourage, 'completed');
  assert.equal(state.flags.anet_saved, true);
  assert.deepEqual(state.battleOutcomes, {});
  assert.ok(state.items.includes('ferry_tag'));
  assert.equal(state.pendingBattle, null);
});

test('the authored ferry waiting choice can be followed by the promised rescue', () => {
  const state = route([...firstAct, 'a2_ferry_wait', 'a2_ferry_manual', 'a2_ferry_complete']);
  assert.equal(state.flags.ferry_supplied, true);
  assert.equal(state.flags.anet_saved, true);
});

test('a native defeat receipt leaves the authored physical ferry rescue reachable', () => {
  let state = route([...firstAct, 'a2_ferry_battle']);
  assert.ok(state.pendingBattle);
  // This fixture checks campaign routing. Actual UQM combat is tested separately.
  const result = resolveBattle(data, state, { id: state.pendingBattle.battleId,
    outcome: 'defeat', crew: 0, enemyRemaining: 1, frames: 500 });
  assert.equal(result.ok, true, result.error);
  state = route(['a2_ferry_setback_leave', 'a2_ferry_manual', 'a2_ferry_complete'], result.state);
  assert.equal(state.battleOutcomes['lantern-drone'], 'defeat');
  assert.equal(state.flags.anet_saved, true);
});

test('six independent principal leads each open the authored third act', () => {
  const paths = Object.values(LEAD_CHOICES);
  for (const path of paths) {
    const state = route([...firstAct, ...path, 'a2_quiet_enter']);
    assert.equal(state.flags.act3, true, path.join(', '));
    assert.ok(state.evidence.includes('sera_signature'));
  }
});

for (const [endingId, choices] of Object.entries(ENDING_ROUTES)) {
  test(`the entire ${endingId} campaign reaches its authored ending through ordinary actions`, () => {
    const { state, actions } = planItinerary(data, choices);
    assert.equal(state.ending, endingId);
    assert.equal(state.flags.campaign_complete, true);
    assert.equal(state.quests.accord, 'completed');
    assert.equal(state.pendingBattle, null);
    assert.equal(state.pendingSurvey, null);
    assert.ok(actions.length > 40);
    assert.ok(state.day > 20);
    assert.ok(state.journal.filter(e => e.type === 'conversation').length > 25);
    assert.deepEqual(validateState(data, state).errors, []);
    assert.deepEqual(loadSave(data, serializeSave(data, state)).state, state);
    assert.equal(applyAction(data, state, { type: 'distress' }).ok, false, 'an ending remains final');
    const otherRoutes = ['accord', 'mandate', 'partition'].filter(id => id !== endingId);
    for (const id of otherRoutes) assert.notEqual(state.flags[`route_${id}`], true);
    const operations = { accord: 'finale_distributed', mandate: 'finale_mandate', partition: 'finale_partition' };
    assert.equal(state.flags[operations[endingId]], true, 'the selected settlement completes its physical operation');
    for (const id of otherRoutes) assert.notEqual(state.flags[operations[id]], true);
  });
}

test('returning for more evidence preserves the final proposal, but selecting one settlement locks it', () => {
  const full = ENDING_ROUTES.mandate, selection = full.indexOf('a4_choose_mandate');
  let state = route([...full.slice(0, selection), 'a4_more_work']);
  state = perform(state, { type: 'travel', systemId: 'hearth' });
  state = route(['a4_supper_finish', 'a4_choose_mandate'], state);
  assert.equal(state.flags.route_mandate, true);
  assert.equal(applyAction(data, state, { type: 'enter', sceneId: 'a4_proposal' }).ok, false);
  state = route(full.slice(selection + 1), state);
  assert.equal(state.ending, 'mandate');
});

test('a limited harbor can be improved later and still qualify for the distributed accord', () => {
  const full = ENDING_ROUTES.accord, preparation = full.indexOf('a3_undertow_publish');
  let state = route([...full.slice(0, preparation), 'a3_harbor_small', 'a3_harbor_complete']);
  assert.equal(state.flags.harbor_limited, true);
  const rest = full.slice(preparation).filter(id => !['a3_harbor_full', 'a3_harbor_complete'].includes(id));
  const assembly = rest.indexOf('a4_table_listen');
  rest.splice(assembly, 0, 'a3_harbor_upgrade_finish');
  state = route(rest, state);
  assert.equal(state.ending, 'accord');
  assert.equal(state.flags.harbor_independent, true);
  assert.equal(state.flags.harbor_limited, false);
});

test('materially different settlement terms select their own epilogues without rewriting history', () => {
  const variants = [
    ['accord', 'a4_accord_review', 'a4_accord_continuity', 'Continuity with an objection channel', 'The first renewal'],
    ['mandate', 'a4_mandate_expiry', 'a4_mandate_board', 'The board and the long emergency', 'A date that remains a date'],
    ['partition', 'a4_partition_corridor', 'a4_partition_reserve', 'Local reserves, longer journeys', 'The narrow common corridor'],
  ];
  for (const [id, from, to, included, excluded] of variants) {
    const state = route(ENDING_ROUTES[id].map(c => c === from ? to : c));
    const ending = getEnding(data, state), titles = ending.epilogues.map(e => e.title);
    assert.ok(titles.includes(included), `${id} includes ${included}`);
    assert.ok(!titles.includes(excluded), `${id} excludes ${excluded}`);
    assert.ok(state.journal.find(e => e.type === 'ending').text.includes(included));
    assert.ok(!state.journal.find(e => e.type === 'ending').text.includes(excluded));
  }
});

test('the rescued person and private letters are remembered only in the playthrough that helped them', () => {
  const base = ENDING_ROUTES.mandate, insertion = base.indexOf('a2_quiet_enter') + 1;
  const rescued = route([...base.slice(0, insertion), 'a2_ferry_labor', 'a2_ferry_complete',
    'a2_return_private', 'a2_return_finish', ...base.slice(insertion)]);
  const ordinary = route(base);
  const rescuedTitles = getEnding(data, rescued).epilogues.map(e => e.title);
  const ordinaryTitles = getEnding(data, ordinary).epilogues.map(e => e.title);
  assert.ok(rescuedTitles.includes('Anet’s actual arrival'));
  assert.ok(rescuedTitles.includes('Letters and their owners'));
  assert.ok(!rescuedTitles.includes('The unanswered ferry case'));
  assert.ok(ordinaryTitles.includes('The unanswered ferry case'));
  assert.ok(!ordinaryTitles.includes('Anet’s actual arrival'));
  assert.ok(!ordinaryTitles.includes('Letters and their owners'));
});

test('late, penniless, fuel-stranded captains can still complete every main ending', () => {
  let state = createGame(data);
  // Ordinary distress contributions exhaust the budget and advance the world.
  while (state.resources.credits > 0) state = perform(state, { type: 'distress' });
  // Ordinary exploration exhausts fuel; no field in the state is assigned by the test.
  for (let i = 0; i < 100; i++) {
    const destination = state.location === 'hearth' ? 'cinder' : 'hearth';
    if (!previewTravel(data, state, destination).allowed) break;
    state = perform(state, { type: 'travel', systemId: destination });
  }
  for (let i = 0; i < 30; i++) state = perform(state, { type: 'distress' });
  assert.equal(state.resources.credits, 0);
  assert.ok(state.day > 200);
  for (const [endingId, choices] of Object.entries(ENDING_ROUTES)) {
    // The authored "watch" alternative preserves Lilt safely until the publicly
    // funded lattice is available. A poor captain must make that actual decision.
    const affordableChoices = choices.map(c => c === 'a1_container_build' ? 'a1_container_watch' : c);
    const final = planItinerary(data, affordableChoices, state).state;
    assert.equal(final.ending, endingId);
    assert.ok(final.journal.some(e => e.type === 'recovery'));
  }
});

test('all five native engagements preserve both a retry and a noncombat route after defeat or retreat', () => {
  const scenarios = engagementScenarios();
  assert.deepEqual(new Set(scenarios.map(s => s.fight)),
    new Set(data.scenes.flatMap(s => s.choices).filter(c => c.combat).map(c => c.id)),
    'every authored native engagement has a setback itinerary');
  for (const scenario of scenarios) for (const outcome of ['defeat', 'retreat']) {
    const before = route([...scenario.opening, scenario.fight]);
    const regrouped = route([scenario.regroup], battleResult(before, outcome));
    const peaceful = route(scenario.alternative, regrouped);
    assert.equal(peaceful.flags[scenario.flag], true, `${scenario.flag} after ${outcome}`);
    assert.equal(peaceful.battleOutcomes[before.pendingBattle.id], outcome,
      'the alternative never invents a native victory');
    const retry = route([scenario.fight], regrouped);
    assert.equal(retry.pendingBattle.id, before.pendingBattle.id, 'the same encounter can be retried');
    assert.notEqual(retry.pendingBattle.battleId, before.pendingBattle.battleId, 'the retry requires a fresh receipt');
    const victorious = route(scenario.victory, battleResult(retry, 'victory'));
    assert.equal(victorious.flags[scenario.flag], true, `${scenario.flag} after ${outcome} and a retry`);
    assert.equal(victorious.battleOutcomes[before.pendingBattle.id], 'victory');
  }
});

test('every authored scene is reachable and an extensive expedition can resolve every quest', () => {
  const grandTour = route(GRAND_TOUR_CHOICES);
  assert.equal(grandTour.ending, 'accord');
  for (const quest of data.quests) assert.equal(grandTour.quests[quest.id], 'completed', `${quest.title} resolves`);
  const visited = new Set(grandTour.visitedScenes);
  for (const id of route(AUTHORITY_TOUR_CHOICES).visitedScenes) visited.add(id);
  for (const choices of Object.values(ENDING_ROUTES)) for (const id of route(choices).visitedScenes) visited.add(id);

  const full = ENDING_ROUTES.accord, preparation = full.indexOf('a3_undertow_publish');
  const rest = full.slice(preparation).filter(id => !['a3_harbor_full', 'a3_harbor_complete'].includes(id));
  rest.splice(rest.indexOf('a4_table_listen'), 0, 'a3_harbor_upgrade_finish');
  const upgrade = route([...full.slice(0, preparation), 'a3_harbor_small', 'a3_harbor_complete', ...rest]);
  for (const id of upgrade.visitedScenes) visited.add(id);

  const collect = state => { for (const id of state.visitedScenes) visited.add(id); return state; };
  for (const scenario of engagementScenarios()) {
    const before = route([...scenario.opening, scenario.fight]);
    collect(battleResult(before, 'defeat'));
    collect(route(scenario.victory, battleResult(before, 'victory')));
  }

  const mistakes = {
    a3_glass_compare: ['a3_glass_private_files', 'a3_glass_back_to_records'],
    a3_glass_diagnose_route: ['a3_glass_diagnose_staff', 'a3_glass_diagnose_retry'],
    a3_archive_compare: ['a3_archive_playback', 'a3_archive_live_test'],
    a3_archive_diagnose_fork: ['a3_archive_diagnose_merge', 'a3_archive_merge_abort'],
    a3_crossing_stagger: ['a3_crossing_double', 'a3_crossing_retry'],
    a4_accord_stagger: ['a4_accord_simultaneous', 'a4_accord_retry'],
    a4_accord_live_refusal: ['a4_accord_cached', 'a4_accord_clear_cache'],
    a4_mandate_loadtest: ['a4_mandate_trusttest', 'a4_mandate_test_return'],
    a4_partition_separate: ['a4_partition_estimate', 'a4_partition_receipt'],
  };
  for (const choices of [GRAND_TOUR_CHOICES, ENDING_ROUTES.mandate, ENDING_ROUTES.partition])
    collect(route(choices.flatMap(id => mistakes[id] ?? [id])));

  collect(route([...OPENING_CHOICES, 'a2_ember_order', 'a2_ember_depart',
    'a2_meadow_consent', 'a2_roots_complete', 'a2_growers_repair_work']));
  collect(route([...OPENING_CHOICES, 'a2_thread_force', 'a2_thread_forced_take',
    'a2_synthesis_complete', 'a2_thread_repair_trace']));
  collect(route([...thirdActRoute().map(id => id === 'a1_report_publish' ? 'a1_report_direct' : id),
    'a2_return_public', 'a2_return_defend', 'a3_mara_repair_work']));
  collect(route([...OPENING_CHOICES, ...LEAD_CHOICES.roots, 'a2_growers_accept_blade']));

  const frozen = waitUntil(route(OPENING_CHOICES), 45);
  collect(route(LEAD_CHOICES.roots, frozen));
  const seedTaken = waitUntil(route([...OPENING_CHOICES, 'a2_ember_wait', 'a2_ember_depart']), 45);
  collect(route(['a2_meadow_consent', 'a2_roots_complete', 'a2_ember_return_work'], seedTaken));
  assert.deepEqual(data.scenes.filter(s => !visited.has(s.id)).map(s => s.id), [], 'no authored scene is stranded');
});

test('coercive choices preserve their actual losses and an explicit council refusal in the ending', () => {
  const state = route(AUTHORITY_TOUR_CHOICES);
  assert.equal(state.ending, 'mandate');
  for (const flag of ['ember_taken', 'meadow_central', 'clock_forced', 'redress_dependent',
    'council_corrected_captain', 'letters_public', 'silence_forced', 'mandate_board', 'mandate_delayed_review'])
    assert.equal(state.flags[flag], true, flag);
  assert.notEqual(state.flags.dual_preserved, true);
  assert.ok(state.visitedScenes.includes('a2_thread_forced'));
  assert.ok(!state.visitedScenes.includes('a2_thread_crossing'), 'forced reset does not display the voluntary crossing aftermath');
  assert.ok(state.visitedScenes.includes('a2_redress_objection'));
  assert.ok(state.journal.some(e => e.choiceId === 'a2_redress_accept_council'));
  const titles = getEnding(data, state).epilogues.map(e => e.title);
  for (const title of ['The time the yard lost', 'A captain corrected by the governed',
    'The culture taken in a hurry', 'A summer promised from the center',
    'The archive made public', 'The rescue that crossed a boundary', 'The board and the long emergency'])
    assert.ok(titles.includes(title), title);
  const consensual = route(GRAND_TOUR_CHOICES);
  const consensualTitles = getEnding(data, consensual).epilogues.map(e => e.title);
  assert.ok(!consensualTitles.includes('The time the yard lost'));
  assert.ok(!consensualTitles.includes('A captain corrected by the governed'));
});

test('authored scheduled consequences fire once and preserve a late expedition’s final choice', () => {
  const full = ENDING_ROUTES.mandate, assembly = full.indexOf('a4_table_listen');
  let state = route(full.slice(0, assembly));
  while (state.day + 7 < 85) state = perform(state, { type: 'distress' });
  const fuelBefore = state.resources.fuel;
  state = perform(state, { type: 'distress' });
  assert.equal(state.flags.reserve_tightened, true);
  assert.equal(state.resources.fuel, Math.max(fuelBefore, 40) - 10);
  while (state.day < 130) state = perform(state, { type: 'distress' });
  assert.deepEqual(new Set(state.triggeredEvents), new Set(data.events.map(e => e.id)));
  for (const event of data.events) assert.equal(state.journal.filter(e => e.eventId === event.id).length, 1);
  state = loadSave(data, serializeSave(data, state)).state;
  state = route(full.slice(assembly), state);
  assert.equal(state.ending, 'mandate');
  assert.equal(state.flags.assembly_waiting, true);
  assert.equal(state.flags.anet_saved, undefined, 'a survival extension never pretends to complete the rescue');
});

test('all twenty authored surface discoveries are reachable and reward only a completed expedition once', () => {
  const sites = data.systems.filter(s => s.survey);
  assert.equal(sites.length, 20);
  assert.equal(new Set(sites.map(s => s.survey.text)).size, 20, 'every surface discovery is authored separately');
  let state = route(GRAND_TOUR_CHOICES.slice(0, GRAND_TOUR_CHOICES.indexOf('a4_table_listen')));
  if (state.scene) state = perform(state, { type: 'leave' });
  for (const system of sites) {
    if (state.location !== system.id) {
      if (!previewTravel(data, state, system.id).allowed) state = perform(state, { type: 'distress' });
      if (state.location !== system.id) state = perform(state, { type: 'travel', systemId: system.id });
    }
    const before = state, money = state.resources.credits;
    state = perform(state, { type: 'launchSurvey' });
    assert.equal(state.resources.credits, money);
    assert.deepEqual(state.items, before.items, 'launching does not grant recovered equipment');
    assert.deepEqual(state.evidence, before.evidence, 'launching does not grant surface evidence');
    // Contract fixture only: the surface physics and keyboard completion have
    // separate lander tests; this checks each actual authored reward/reference.
    const completion = resolveSurvey(data, state, { id: state.pendingSurvey.id,
      outcome: 'surveyed', salvage: 0, hullRemaining: 80, beacons: 3 });
    assert.equal(completion.ok, true, completion.error);
    state = completion.state;
    assert.equal(state.resources.credits, money + system.survey.rewards.credits);
    for (const id of system.survey.rewards.giveItems ?? []) assert.ok(state.items.includes(id), `${system.id} rewards ${id}`);
    for (const id of system.survey.rewards.evidence ?? []) assert.ok(state.evidence.includes(id), `${system.id} discovers ${id}`);
    for (const [id, value] of Object.entries(system.survey.rewards.flags ?? {}))
      assert.equal(state.flags[id], value, `${system.id} activates ${id}`);
    assert.equal(applyAction(data, state, { type: 'launchSurvey' }).ok, false);
    assert.equal(state.journal.filter(e => e.type === 'survey' && e.systemId === system.id).length, 1);
  }
  assert.equal(state.surveyedSystems.length, 20);
  state = route(ENDING_ROUTES.accord.slice(ENDING_ROUTES.accord.indexOf('a4_table_listen')), state);
  assert.equal(state.ending, 'accord');
});
