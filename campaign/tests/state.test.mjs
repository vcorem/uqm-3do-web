import test from 'node:test';
import assert from 'node:assert/strict';
import { createGame, validateCampaign, validateState, applyAction, getView, previewTravel,
  serializeSave, loadSave, resolveBattle, getBattleRequest, resolveSurvey, meets } from '../src/state.mjs';

function fixture() {
  const scene = (id, systemId, choices, extra = {}) => ({ id, systemId, title: id,
    characterId: 'keeper', act: 1, paragraphs: [`An authored ${id} encounter.`], choices, ...extra });
  return {
    id: 'test-campaign', version: 1, title: 'State fixture',
    start: { systemId: 'home', sceneId: 'arrival', credits: 100, fuel: 100, crew: 18 },
    systems: [
      { id: 'home', name: 'Home', x: 0, y: 0, services: { refuel: true, repair: true },
        survey: { title: 'Old hull', text: 'A hull identifies a lost arrival.', days: 2,
          rewards: { credits: 15, evidence: ['hull_record'] } } },
      { id: 'field', name: 'Field', x: 24, y: 0, survey: { title: 'Field survey', text: 'Tracks lead away.', days: 1, rewards: { credits: 10 } } },
      { id: 'hidden', name: 'Hidden', x: 80, y: 60, known: false },
    ],
    characters: [{ id: 'keeper', name: 'Keeper' }], quests: [{ id: 'inquiry', title: 'Inquiry' }],
    items: [{ id: 'key', name: 'Key' }, { id: 'part', name: 'Part' }],
    evidence: [{ id: 'hull_record', name: 'Hull record' }],
    endings: [{ id: 'accord', title: 'Accord', paragraphs: ['The agreement holds.'] }],
    scenes: [
      scene('arrival', 'home', [{ id: 'begin', text: 'Begin.', effects: { flags: { begun: true }, quests: { inquiry: 'active' }, giveItems: ['key'] } }]),
      scene('trade', 'home', [{ id: 'purchase', text: 'Purchase the part.', requires: { resource: 'credits', min: 40 }, effects: { credits: -40, giveItems: ['part'], flags: { bought: true } } }], { requires: { flag: 'begun' } }),
      scene('gate', 'home', [{ id: 'open_gate', text: 'Open.', next: 'revelation', effects: { takeItems: ['key'], flags: { opened: true }, unlockSystems: ['hidden'], reputation: { keeper: 2 } } }], { requires: { item: 'key' } }),
      scene('revelation', 'home', [{ id: 'understand', text: 'Understand.', effects: { flags: { informed: true }, journal: 'The keeper has a reason.' } }], { entry: false, requires: { flag: 'opened' } }),
      scene('interception', 'field', [{ id: 'engage', text: 'Intercept.', combat: { id: 'test_fight', enemies: [15], winSceneId: 'won', loseSceneId: 'lost', retreatSceneId: 'retreated' } }]),
      scene('won', 'field', [{ id: 'claim_victory', text: 'Continue.', effects: { flags: { won: true }, ship: 3 } }], { entry: false }),
      scene('lost', 'field', [{ id: 'accept_loss', text: 'Rebuild.', effects: { flags: { rebuilt: true } } }], { entry: false }),
      scene('retreated', 'field', [{ id: 'accept_retreat', text: 'Reconsider.', effects: { flags: { withdrew: true } } }], { entry: false }),
      scene('conclusion', 'hidden', [{ id: 'sign', text: 'Sign.', effects: { quests: { inquiry: 'completed' }, ending: 'accord' } }], { requires: { flag: 'informed' } }),
    ], events: [],
  };
}

const ok = result => { assert.equal(result.ok, true, result.error); return result.state; };
const act = (data, state, type, rest = {}) => ok(applyAction(data, state, { type, ...rest }));
function completeSurvey(data, state) {
  state = act(data, state, 'launchSurvey');
  return ok(resolveSurvey(data, state, { id: state.pendingSurvey.id, outcome: 'surveyed', salvage: 0, hullRemaining: 100, beacons: 3 }));
}
const clone = x => structuredClone(x);
function freeze(o) { if (o && typeof o === 'object') { Object.freeze(o); Object.values(o).forEach(freeze); } return o; }
function begun(data = fixture()) { return act(data, createGame(data), 'choose', { choiceId: 'begin' }); }
function battle(data = fixture()) {
  let state = begun(data);
  state = act(data, state, 'travel', { systemId: 'field' });
  state = act(data, state, 'enter', { sceneId: 'interception' });
  return act(data, state, 'choose', { choiceId: 'engage' });
}
function receipt(state, outcome = 'victory') {
  return { id: state.pendingBattle.battleId, outcome, crew: outcome === 'defeat' ? 0 : 9,
    enemyRemaining: outcome === 'victory' ? 0 : 1, frames: 1000 };
}

test('valid data starts a real conversation with a preserved immutable source', () => {
  const data = freeze(fixture());
  assert.deepEqual(validateCampaign(data), { errors: [], warnings: [] });
  const state = createGame(data, { captainName: 'Mara' });
  assert.equal(state.scene, 'arrival');
  assert.equal(state.ship, 5);
  assert.equal(state.resources.crew, 18);
  assert.equal(state.journal[0].text, data.scenes[0].paragraphs.join('\n\n'));
  assert.deepEqual(validateState(data, state).errors, []);
  assert.equal(state.captainName, 'Mara');
});

test('all predicate types compose without executing expressions', () => {
  const data = fixture(); let state = begun(data);
  state = completeSurvey(data, state);
  state = act(data, state, 'enter', { sceneId: 'gate' });
  state = act(data, state, 'choose', { choiceId: 'open_gate' });
  assert.equal(meets(state, { all: [
    { flag: 'begun' }, { not: { flag: 'unknown' } }, { evidence: 'hull_record' },
    { quest: 'inquiry', status: 'active' }, { reputation: 'keeper', min: 2 },
    { resource: 'fuel', min: 100 }, { visited: 'revelation' }, { day: { min: 2, max: 2 } },
    { any: [{ item: 'missing' }, { flag: 'opened' }] },
  ] }), true);
  assert.equal(meets(state, { flag: 'begun', is: false }), false);
  assert.equal(meets(state, { invented: 'code' }), false);
});

test('failed choices roll back every field including item, funds, flags, time, and logs', () => {
  const data = fixture();
  data.scenes.find(s => s.id === 'gate').choices[0].effects = { takeItems: ['key'], credits: -12, days: 5, flags: { opened: false }, unlockSystems: ['hidden'] };
  let state = begun(data); state = act(data, state, 'enter', { sceneId: 'gate' });
  freeze(state); const before = JSON.stringify(state);
  const result = applyAction(data, state, { type: 'choose', choiceId: 'open_gate' });
  assert.equal(result.ok, false);
  assert.match(result.error, /discoveries/);
  assert.equal(result.state, state);
  assert.equal(JSON.stringify(state), before);
});

test('a choice must belong to the active scene and meet exact costs', () => {
  const data = fixture(); let state = begun(data);
  assert.equal(applyAction(data, state, { type: 'choose', choiceId: 'purchase' }).ok, false);
  state.resources.credits = 39;
  state = act(data, state, 'enter', { sceneId: 'trade' });
  assert.equal(getView(data, state).choices[0].enabled, false);
  const failed = applyAction(data, state, { type: 'choose', choiceId: 'purchase' });
  assert.equal(failed.state, state);
  assert.equal(failed.ok, false);
  assert.equal(state.resources.credits, 39);
});

test('dialogue, survey, and transaction rewards cannot be collected twice', () => {
  const data = fixture(); let state = begun(data);
  state = completeSurvey(data, state); const credit = state.resources.credits;
  assert.equal(applyAction(data, state, { type: 'survey' }).ok, false);
  assert.equal(state.resources.credits, credit);
  assert.equal(applyAction(data, state, { type: 'enter', sceneId: 'arrival' }).ok, false);
  state = act(data, state, 'enter', { sceneId: 'trade' });
  state = act(data, state, 'choose', { choiceId: 'purchase' });
  assert.deepEqual(state.items, ['key', 'part']);
  assert.equal(applyAction(data, state, { type: 'enter', sceneId: 'trade' }).ok, false);
});

test('reading, closing, and reopening incomplete dialogue does not advance time or duplicate text', () => {
  const data = fixture(); let state = createGame(data);
  for (let i = 0; i < 50; i++) {
    state = act(data, state, 'leave');
    state = act(data, state, 'enter', { sceneId: 'arrival' });
    getView(data, state);
  }
  assert.equal(state.day, 0);
  assert.equal(state.journal.length, 1);
  assert.equal(state.resources.fuel, 100);
  assert.deepEqual(state.completedScenes, []);
});

test('manual entry cannot bypass automatic dialogue or combat result gates', () => {
  const data = fixture(); let state = begun(data);
  assert.equal(applyAction(data, state, { type: 'enter', sceneId: 'revelation' }).ok, false);
  state = act(data, state, 'travel', { systemId: 'field' });
  // Even forgetting entry:false must never expose a native victory node.
  data.scenes.find(s => s.id === 'won').entry = true;
  assert.equal(getView(data, state).availableScenes.some(s => s.id === 'won'), false);
  assert.equal(applyAction(data, state, { type: 'enter', sceneId: 'won' }).ok, false);
  assert.equal(state.ship, 5);
});

test('an unlocked automatic continuation can be resumed after closing or travelling away', () => {
  const data = fixture(); let state = begun(data);
  state = act(data, state, 'enter', { sceneId: 'gate' });
  state = act(data, state, 'choose', { choiceId: 'open_gate' });
  assert.equal(state.scene, 'revelation');
  state = act(data, state, 'travel', { systemId: 'field' });
  state = act(data, state, 'travel', { systemId: 'home' });
  assert.ok(getView(data, state).availableScenes.some(s => s.id === 'revelation'));
  state = act(data, state, 'enter', { sceneId: 'revelation' });
  state = act(data, state, 'choose', { choiceId: 'understand' });
  assert.equal(state.flags.informed, true);
  assert.equal(state.journal.filter(e => e.sceneId === 'revelation' && e.type === 'conversation').length, 1);
});

test('an authored noncombat alternative may share a resolution without fabricating a battle victory', () => {
  const data = fixture();
  data.scenes.find(s => s.id === 'interception').choices.push({ id: 'negotiate', text: 'Negotiate a passage.',
    effects: { credits: -20 }, next: 'won' });
  let state = begun(data);
  state = act(data, state, 'travel', { systemId: 'field' });
  state = act(data, state, 'enter', { sceneId: 'interception' });
  state = act(data, state, 'choose', { choiceId: 'negotiate' });
  assert.equal(state.scene, 'won'); assert.equal(state.pendingBattle, null);
  assert.deepEqual(state.battleOutcomes, {});
  state = act(data, state, 'leave');
  assert.ok(getView(data, state).availableScenes.some(s => s.id === 'won'));
  state = act(data, state, 'enter', { sceneId: 'won' });
  assert.equal(state.resources.credits, 80);
});

test('travel preview matches applied time/fuel and refuses unknown or unaffordable systems', () => {
  const data = fixture(); let state = begun(data);
  assert.equal(previewTravel(data, state, 'home').allowed, false);
  assert.equal(previewTravel(data, state, 'hidden').allowed, false);
  const preview = previewTravel(data, state, 'field');
  assert.equal(preview.fuel, 3); assert.equal(preview.days, 2); assert.equal(preview.distance, 24);
  state = act(data, state, 'travel', { systemId: 'field' });
  assert.equal(state.day, 2); assert.equal(state.resources.fuel, 97);
  state.resources.fuel = 0;
  const result = applyAction(data, state, { type: 'travel', systemId: 'home' });
  assert.equal(result.ok, false); assert.equal(result.state, state);
});

test('services enforce capacity, positive integer amounts, availability, and money', () => {
  const data = fixture(); let state = begun(data);
  state.resources.fuel = 80; state.resources.crew = 8;
  state = act(data, state, 'refuel', { amount: 20 });
  assert.equal(state.resources.credits, 60);
  assert.equal(state.resources.fuel, 100);
  assert.equal(applyAction(data, state, { type: 'refuel', amount: 1 }).ok, false);
  assert.equal(applyAction(data, state, { type: 'repair', amount: -10 }).ok, false);
  assert.equal(applyAction(data, state, { type: 'repair', amount: 1.5 }).ok, false);
  state = act(data, state, 'repair', { amount: 10 });
  assert.equal(state.resources.credits, 30);
  assert.equal(state.resources.crew, 18);
  state = act(data, state, 'travel', { systemId: 'field' });
  assert.equal(applyAction(data, state, { type: 'refuel', amount: 1 }).ok, false);
});

test('distress always recovers a zero-credit zero-fuel zero-crew ship without creating currency', () => {
  const data = fixture(); let state = begun(data);
  state = act(data, state, 'travel', { systemId: 'field' });
  state.resources = { credits: 0, fuel: 0, crew: 0 };
  const day = state.day;
  state = act(data, state, 'distress');
  assert.equal(state.location, 'home'); assert.equal(state.day, day + 7);
  assert.deepEqual(state.resources, { credits: 0, fuel: 40, crew: 9 });
  state = act(data, state, 'distress');
  assert.deepEqual(state.resources, { credits: 0, fuel: 40, crew: 9 });
  assert.equal(previewTravel(data, state, 'field').allowed, true);
});

test('launching a survey grants nothing; only a valid completed surface receipt grants the authored discovery', () => {
  const data = fixture(); let state = begun(data);
  state = act(data, state, 'survey');
  assert.ok(state.pendingSurvey); assert.equal(state.day, 0);
  assert.equal(state.resources.credits, 100); assert.deepEqual(state.evidence, []);
  assert.equal(getView(data, state).canSurvey, false);
  assert.equal(applyAction(data, state, { type: 'travel', systemId: 'field' }).ok, false);
  for (const patch of [{ id: 'stale' }, { beacons: 2 }, { hullRemaining: 0 }, { salvage: 61 }, { hullRemaining: 101 }]) {
    const result = resolveSurvey(data, state, { id: state.pendingSurvey.id, outcome: 'surveyed', salvage: 10, hullRemaining: 100, beacons: 3, ...patch });
    assert.equal(result.ok, false); assert.equal(result.state, state);
  }
  const report = { id: state.pendingSurvey.id, outcome: 'surveyed', salvage: 20, hullRemaining: 77, beacons: 3 };
  const finished = ok(resolveSurvey(data, state, report));
  assert.equal(finished.day, 2); assert.equal(finished.resources.credits, 135);
  assert.deepEqual(finished.evidence, ['hull_record']); assert.deepEqual(finished.surveyedSystems, ['home']);
  assert.equal(resolveSurvey(data, finished, report).ok, false);
  assert.equal(applyAction(data, finished, { type: 'launchSurvey' }).ok, false);
});

test('surface salvage is capped across retries and retreat cannot duplicate beacon rewards', () => {
  const data = fixture(); let state = begun(data);
  for (let i = 0; i < 3; i++) {
    state = act(data, state, 'launchSurvey');
    state = ok(resolveSurvey(data, state, { id: state.pendingSurvey.id, outcome: 'retreated', salvage: 40, hullRemaining: 90, beacons: 2 }));
  }
  assert.equal(state.surveySalvage.home, 60); assert.equal(state.resources.credits, 160);
  assert.equal(state.day, 3); assert.deepEqual(state.evidence, []); assert.deepEqual(state.surveyedSystems, []);
  state = completeSurvey(data, state);
  assert.equal(state.resources.credits, 175, 'authored reward is separate and paid only after success');
  assert.equal(state.surveySalvage.home, 60);
});

test('surface damage and save interruption permit recall, recovery, and retry', () => {
  const data = fixture(); let state = begun(data);
  state = act(data, state, 'launchSurvey');
  const pending = loadSave(data, serializeSave(data, state));
  assert.equal(pending.ok, true); assert.deepEqual(pending.state.pendingSurvey, state.pendingSurvey);
  state = act(data, pending.state, 'abandonSurvey');
  assert.equal(state.pendingSurvey, null); assert.equal(state.day, 1);
  state = act(data, state, 'launchSurvey');
  state = ok(resolveSurvey(data, state, { id: state.pendingSurvey.id, outcome: 'damaged', salvage: 0, hullRemaining: 0, beacons: 0 }));
  assert.equal(state.resources.crew, 15); assert.equal(state.day, 2);
  assert.equal(state.pendingSurvey, null); assert.deepEqual(state.surveyedSystems, []);
  state = completeSurvey(data, state);
  assert.deepEqual(state.surveyedSystems, ['home']);
});

test('native battle request is deterministic and campaign remains blocked until result', () => {
  const data = fixture(), state = battle(data), request = getBattleRequest(data, state);
  assert.deepEqual(request, getBattleRequest(data, clone(state)));
  assert.deepEqual(request.player, { ship: 5, crew: 18 });
  assert.deepEqual(request.enemies, [15]);
  assert.ok(Number.isInteger(request.seed) && request.seed >= 0);
  for (const action of [{ type: 'travel', systemId: 'home' }, { type: 'survey' }, { type: 'choose', choiceId: 'claim_victory' }, { type: 'distress' }]) {
    assert.equal(applyAction(data, state, action).ok, false);
  }
  assert.equal(state.flags.won, undefined);
});

test('only matching, internally consistent native results can resolve an encounter once', () => {
  const data = fixture(), state = battle(data), result = receipt(state);
  for (const patch of [{ id: 'wrong' }, { outcome: 'won' }, { crew: 19 }, { crew: 0 }, { enemyRemaining: 1 }, { frames: -1 }]) {
    const rejected = resolveBattle(data, state, { ...result, ...patch });
    assert.equal(rejected.ok, false, JSON.stringify(patch)); assert.equal(rejected.state, state);
  }
  let won = ok(resolveBattle(data, state, result));
  assert.equal(won.scene, 'won'); assert.equal(won.resources.crew, 9);
  assert.equal(won.battleOutcomes.test_fight, 'victory');
  assert.equal(resolveBattle(data, won, result).ok, false);
  assert.equal(won.flags.won, undefined, 'native battle routes the conversation but does not fabricate story decisions');
  won = act(data, won, 'choose', { choiceId: 'claim_victory' });
  assert.equal(won.ship, 3); assert.equal(won.resources.crew, 42);
  assert.equal(getView(data, won).economy.crewCapacity, 42);
});

test('defeat and retreat preserve fail-forward paths with authentic crew and stated cost', () => {
  const data = fixture(), state = battle(data);
  const defeated = ok(resolveBattle(data, state, receipt(state, 'defeat')));
  assert.equal(defeated.scene, 'lost'); assert.equal(defeated.day, state.day + 4);
  assert.equal(defeated.resources.credits, 75); assert.equal(defeated.resources.crew, 9);
  assert.equal(defeated.flags.won, undefined);
  const retreated = ok(resolveBattle(data, state, receipt(state, 'retreat')));
  assert.equal(retreated.scene, 'retreated'); assert.equal(retreated.resources.crew, 9);
  assert.equal(retreated.resources.credits, 100); assert.equal(retreated.day, state.day + 1);
  const abandoned = act(data, state, 'abandonBattle');
  assert.equal(abandoned.scene, 'retreated'); assert.equal(abandoned.battleOutcomes.test_fight, 'retreat');
});

test('crew adjustments on an acquired vessel apply to that vessel’s native complement', () => {
  const data = fixture();
  data.scenes.find(s => s.id === 'won').choices[0].effects.crew = -12;
  let state = battle(data);
  state = resolveBattle(data, state, receipt(state, 'victory')).state;
  assert.equal(state.resources.crew, 9);
  state = act(data, state, 'choose', { choiceId: 'claim_victory' });
  assert.equal(state.ship, 3);
  assert.equal(state.resources.crew, 30, 'new native crew 42 minus the explicit 12 adjustment');
});

test('a complete authored route reaches the ending without debug flags or fabricated rewards', () => {
  const data = fixture(); let state = begun(data);
  state = act(data, state, 'enter', { sceneId: 'gate' });
  state = act(data, state, 'choose', { choiceId: 'open_gate' });
  state = act(data, state, 'choose', { choiceId: 'understand' });
  state = act(data, state, 'travel', { systemId: 'hidden' });
  state = act(data, state, 'enter', { sceneId: 'conclusion' });
  state = act(data, state, 'choose', { choiceId: 'sign' });
  assert.equal(state.ending, 'accord'); assert.equal(state.quests.inquiry, 'completed');
  assert.equal(state.scene, null);
  assert.equal(getView(data, state).ending.title, 'Accord');
  assert.equal(applyAction(data, state, { type: 'travel', systemId: 'home' }).ok, false);
  assert.equal(loadSave(data, serializeSave(data, state)).ok, true);
});

test('ending epilogues reflect actual decisions and are preserved in the final journal', () => {
  const data = fixture();
  data.endings[0].epilogues = [
    { title: 'The informed council', requires: { flag: 'informed' }, paragraphs: ['The council receives the complete account.'] },
    { title: 'The closed council', requires: { not: { flag: 'informed' } }, paragraphs: ['The council never receives the account.'] },
  ];
  let state = begun(data);
  state = act(data, state, 'enter', { sceneId: 'gate' });
  state = act(data, state, 'choose', { choiceId: 'open_gate' });
  state = act(data, state, 'choose', { choiceId: 'understand' });
  state = act(data, state, 'travel', { systemId: 'hidden' });
  state = act(data, state, 'enter', { sceneId: 'conclusion' });
  state = act(data, state, 'choose', { choiceId: 'sign' });
  assert.deepEqual(getView(data, state).ending.epilogues.map(e => e.title), ['The informed council']);
  const text = state.journal.find(e => e.type === 'ending').text;
  assert.match(text, /complete account/); assert.doesNotMatch(text, /never receives/);
});

test('save roundtrips complete state, including a pending native engagement', () => {
  const data = fixture(), state = battle(data);
  const loaded = loadSave(data, serializeSave(data, state));
  assert.equal(loaded.ok, true, loaded.error);
  assert.deepEqual(loaded.state, state);
  assert.notEqual(loaded.state, state);
  assert.deepEqual(getBattleRequest(data, loaded.state), getBattleRequest(data, state));
});

test('corrupt saves reject invalid versions, ids, money, battle specs, arrays, and object injection', () => {
  const data = fixture(), state = battle(data);
  const mutate = fn => { const next = clone(state); fn(next); return JSON.stringify({ format: 'afterlight-save', schemaVersion: 1, state: next }); };
  for (const fn of [
    s => { s.schemaVersion = 2; }, s => { s.campaignVersion = 999; }, s => { s.campaignId = 'other'; },
    s => { s.resources.fuel = -1; }, s => { s.resources.crew = 999; }, s => { s.resources.credits = Infinity; },
    s => { s.location = 'missing'; }, s => { s.items = ['missing']; }, s => { s.flags.invented = true; },
    s => { s.usedChoices = {}; }, s => { s.completedScenes = ['arrival', 'arrival']; },
    s => { s.pendingBattle.enemies = [3, 3]; }, s => { s.journal = [null]; },
    s => { s.battleOutcomes.invented = 'victory'; }, s => { s.triggeredEvents = ['missing']; },
  ]) assert.equal(loadSave(data, mutate(fn)).ok, false, fn.toString());
  assert.equal(loadSave(data, '{"__proto__":{"admin":true}}').ok, false);
  assert.equal(loadSave(data, 'x'.repeat(4 * 1024 * 1024 + 1)).ok, false);
  assert.equal(loadSave(data, '{').ok, false);
  assert.equal(({}).admin, undefined);
});

test('campaign validation catches broken references and cycles cannot create repeatable rewards', () => {
  for (const change of [
    d => { d.scenes[0].choices[0].next = 'absent'; },
    d => { d.scenes[0].choices[0].next = 'interception'; },
    d => { d.scenes[0].choices[0].effects.giveItems = ['absent']; },
    d => { d.scenes[1].choices[0].id = 'begin'; },
    d => { d.scenes[0].choices[0].once = false; },
    d => { d.scenes[0].requires = { flag: 'begun', item: 'key' }; },
    d => { d.scenes[0].requires = { resource: 'credits', min: -5 }; },
    d => { d.scenes[0].choices[0].effects.days = -3; },
    d => { d.scenes[4].choices[0].combat.enemies = [99]; },
    d => { d.start.crew = 40; },
    d => { d.factions = [null]; },
    d => { d.factions = [{ id: 'bad id' }]; },
    d => { d.systems[0].survey = 'missing survey description'; },
    d => { d.systems[0].services = { refuel: 'yes' }; },
  ]) { const data = fixture(); change(data); assert.ok(validateCampaign(data).errors.length > 0, change.toString()); }
});

function eventFixture() {
  const data = fixture();
  data.events = [
    { id: 'first_warning', day: 2, title: 'Warning', text: 'The tender needs help.', effects: { flags: { warned: true } } },
    { id: 'rescue_deadline', day: 4, title: 'Departure', text: 'The tender relocates.', requires: { not: { flag: 'rescued' } }, effects: { flags: { relocated: true }, credits: -200 } },
    { id: 'delayed_report', day: 1, title: 'Report', text: 'A delayed report arrives.', requires: { flag: 'informed' }, effects: { flags: { report: true } } },
    { id: 'day_gate', day: 1, title: 'Aligned', text: 'The clocks align.', requires: { day: { min: 3, max: 3 } }, effects: { flags: { aligned: true } } },
  ];
  data.scenes[0].choices[0].effects.flags.rescued = false;
  return data;
}

test('time advances across all scheduled thresholds and triggers each event only once', () => {
  const data = eventFixture(); let state = begun(data);
  state = act(data, state, 'distress');
  assert.equal(state.day, 7);
  assert.deepEqual(state.triggeredEvents, ['first_warning', 'day_gate', 'rescue_deadline']);
  assert.deepEqual(state.journal.filter(e => e.type === 'world').map(e => e.day), [2, 3, 4]);
  assert.equal(state.resources.credits, 0, 'an unaffordable world consequence cannot stop time');
  state = act(data, state, 'distress');
  assert.equal(state.triggeredEvents.length, 3);
  assert.equal(state.journal.filter(e => e.type === 'world').length, 3);
});

test('bulk advancement and daily steps yield the same world-event ordering and resource consequences', () => {
  const data = eventFixture();
  const wait = days => ({ id: `wait_${days}`, text: 'Wait.', effects: { days } });
  data.scenes.push({ id: 'waiting', systemId: 'home', title: 'Observatory', characterId: 'keeper', act: 1,
    paragraphs: ['The observatory records departures.'], repeatable: true,
    choices: [wait(7), ...Array.from({ length: 7 }, (_, i) => ({ id: `step_${i}`, text: 'One day.', effects: { days: 1 } }))] });
  let bulk = begun(data), stepped = clone(bulk);
  bulk = act(data, bulk, 'enter', { sceneId: 'waiting' });
  bulk = act(data, bulk, 'choose', { choiceId: 'wait_7' });
  for (let i = 0; i < 7; i++) {
    stepped = act(data, stepped, 'enter', { sceneId: 'waiting' });
    stepped = act(data, stepped, 'choose', { choiceId: `step_${i}` });
  }
  for (const key of ['day', 'triggeredEvents', 'flags', 'resources']) assert.deepEqual(bulk[key], stepped[key], key);
  assert.deepEqual(bulk.journal.filter(e => e.type === 'world').map(({ day, title, text }) => ({ day, title, text })),
    stepped.journal.filter(e => e.type === 'world').map(({ day, title, text }) => ({ day, title, text })));
});

test('overdue conditional events activate when their evidence arrives, including after save/load', () => {
  const data = eventFixture(); let state = begun(data);
  state = act(data, state, 'distress');
  state = loadSave(data, serializeSave(data, state)).state;
  state = act(data, state, 'enter', { sceneId: 'gate' });
  state = act(data, state, 'choose', { choiceId: 'open_gate' });
  state = act(data, state, 'choose', { choiceId: 'understand' });
  assert.equal(state.day, 7);
  assert.equal(state.flags.report, true);
  assert.equal(state.journal.find(e => e.eventId === 'delayed_report').day, 7);
});

test('a deliberately broken authored route is detected by traversal, not disguised as success', () => {
  const data = fixture();
  data.scenes.find(s => s.id === 'revelation').requires = { flag: 'never_set' };
  assert.ok(validateCampaign(data).warnings.some(w => w.includes('never_set')));
  let state = begun(data); state = act(data, state, 'enter', { sceneId: 'gate' });
  const result = applyAction(data, state, { type: 'choose', choiceId: 'open_gate' });
  assert.equal(result.ok, false);
  assert.equal(result.state.ending, null);
  assert.ok(result.state.items.includes('key'), 'failed traversal preserves recoverability');
});
