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

test('authored campaign has valid identifiers, data types, references, and complete dialogue', () => {
  const check = validateCampaign(data);
  assert.deepEqual(check.errors, []);
  assert.deepEqual(check.warnings, []);
  assert.equal(data.systems.length, 32);
  assert.ok(data.characters.length >= 10);
  assert.ok(data.scenes.length >= 100, 'complete, authored multi-act campaign');
  const targets = new Set(data.scenes.flatMap(s => s.choices.flatMap(c =>
    [c.next, c.combat?.winSceneId, c.combat?.loseSceneId, c.combat?.retreatSceneId].filter(Boolean))));
  for (const scene of data.scenes) if (scene.entry === false && scene.id !== data.start.sceneId)
    assert.ok(targets.has(scene.id), `Continuation ${scene.id} has an authored incoming link`);
  for (const id of targets) assert.equal(data.scenes.find(s => s.id === id).entry, false, `Continuation ${id} is not a premature local entry`);
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

test('all three native engagement setbacks preserve a noncombat route to their objective', () => {
  const scenarios = [
    { opening: [...OPENING_CHOICES, 'a2_ferry_battle'], after: ['a2_ferry_setback_leave', 'a2_ferry_labor', 'a2_ferry_complete'],
      flag: 'anet_saved' },
    { opening: [...thirdActRoute(), ...INVESTIGATION_CHOICES, 'a2_shoal_hold', 'a2_pell_depart', 'a3_convoy_fight'],
      after: ['a3_convoy_regroup', 'a3_convoy_record', 'a3_convoy_finish'], flag: 'pell_supports' },
    { opening: [...ENDING_ROUTES.mandate.slice(0, ENDING_ROUTES.mandate.indexOf('a4_junction_bypass')), 'a4_junction_battle'],
      after: ['a4_junction_regroup', 'a4_junction_bypass', 'a4_junction_finish', 'a4_mandate_expiry', 'a4_mandate_watch', 'a4_mandate_end'],
      flag: 'junction_ready' },
  ];
  for (const scenario of scenarios) for (const outcome of ['defeat', 'retreat']) {
    let state = route(scenario.opening);
    // Fixture verifies receipt routing only. Native combat has independent real-engine tests.
    const resolution = resolveBattle(data, state, { id: state.pendingBattle.battleId, outcome,
      crew: outcome === 'defeat' ? 0 : 1, enemyRemaining: state.pendingBattle.enemies.length, frames: 500 });
    assert.equal(resolution.ok, true, resolution.error);
    state = route(scenario.after, resolution.state);
    assert.equal(state.flags[scenario.flag], true, `${scenario.flag} after ${outcome}`);
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

  for (const choices of [
    [...OPENING_CHOICES, 'a2_ferry_battle'],
    [...thirdActRoute(), ...INVESTIGATION_CHOICES, 'a2_shoal_hold', 'a2_pell_depart', 'a3_convoy_fight'],
    [...ENDING_ROUTES.mandate.slice(0, ENDING_ROUTES.mandate.indexOf('a4_junction_bypass')), 'a4_junction_battle'],
  ]) {
    const before = route(choices);
    const result = resolveBattle(data, before, { id: before.pendingBattle.battleId,
      outcome: 'defeat', crew: 0, enemyRemaining: before.pendingBattle.enemies.length, frames: 500 });
    assert.equal(result.ok, true, result.error);
    for (const id of result.state.visitedScenes) visited.add(id);
  }
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

test('all eighteen authored surface discoveries are reachable and reward only a completed expedition once', () => {
  const sites = data.systems.filter(s => s.survey);
  assert.equal(sites.length, 18);
  assert.equal(new Set(sites.map(s => s.survey.text)).size, 18, 'every surface discovery is authored separately');
  let state = route(GRAND_TOUR_CHOICES.slice(0, GRAND_TOUR_CHOICES.indexOf('a4_table_listen')));
  if (state.scene) state = perform(state, { type: 'leave' });
  for (const system of sites) {
    if (state.location !== system.id) {
      if (!previewTravel(data, state, system.id).allowed) state = perform(state, { type: 'distress' });
      if (state.location !== system.id) state = perform(state, { type: 'travel', systemId: system.id });
    }
    const money = state.resources.credits;
    state = perform(state, { type: 'launchSurvey' });
    assert.equal(state.resources.credits, money);
    // Contract fixture only: the surface physics and keyboard completion have
    // separate lander tests; this checks each actual authored reward/reference.
    const completion = resolveSurvey(data, state, { id: state.pendingSurvey.id,
      outcome: 'surveyed', salvage: 0, hullRemaining: 80, beacons: 3 });
    assert.equal(completion.ok, true, completion.error);
    state = completion.state;
    assert.equal(state.resources.credits, money + system.survey.rewards.credits);
    assert.equal(applyAction(data, state, { type: 'launchSurvey' }).ok, false);
    assert.equal(state.journal.filter(e => e.type === 'survey' && e.systemId === system.id).length, 1);
  }
  assert.equal(state.surveyedSystems.length, 18);
  state = route(ENDING_ROUTES.accord.slice(ENDING_ROUTES.accord.indexOf('a4_table_listen')), state);
  assert.equal(state.ending, 'accord');
});
