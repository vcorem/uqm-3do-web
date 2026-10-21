import test from 'node:test';
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { SHIPS } from '../src/combat.mjs';
import { createGame, applyAction, getView, getCapabilities, previewTravel, validateCampaign,
  validateState, serializeSave, loadSave, resolveBattle, resolveSurvey } from '../src/state.mjs';

function fixture() {
  const scene = (id, choices, extra = {}) => ({ id, title: id, systemId: 'home', characterId: 'engineer',
    act: 1, paragraphs: [`The ${id} encounter.`], choices, ...extra });
  return {
    id: 'progression-test', version: 1, title: 'Progression fixture',
    start: { systemId: 'home', sceneId: 'arrival', credits: 200, fuel: 100, crew: 18, ship: 5 },
    systems: [{ id: 'home', name: 'Home', x: 0, y: 0, services: { refuel: true, repair: true } },
      { id: 'field', name: 'Field', x: 80, y: 0 }],
    characters: [{ id: 'engineer', name: 'Engineer' }], quests: [], items: [], evidence: [], endings: [], events: [],
    scenes: [
      scene('arrival', [{ id: 'begin', text: 'Begin.', effects: { reputation: { engineer: 2 }, flags: { met: true } } }]),
      scene('award', [{ id: 'accept', text: 'Accept equipment.', effects: { flags: { upgrade_tank: true }, fuel: 40 } }]),
      scene('encounter', [{ id: 'fight', text: 'Fight.', effects: { credits: -5 },
        combat: { id: 'patrol', enemies: [14], winSceneId: 'won', loseSceneId: 'setback', retreatSceneId: 'setback' } }]),
      scene('setback', [{ id: 'regroup', text: 'Regroup.', next: 'debrief', effects: { credits: 7, flags: { regrouped: true } } }], { entry: false }),
      scene('debrief', [{ id: 'continue', text: 'Continue.', effects: { credits: -3 } }], { entry: false }),
      scene('won', [{ id: 'reward', text: 'Collect the salvage.', effects: { credits: 30 } }], { entry: false }),
      scene('callback', [{ id: 'close', text: 'Close.' }], { variants: [
        { requires: { flag: 'met' }, paragraphs: ['The engineer remembers your meeting.'] },
        { requires: { reputation: 'engineer', min: 0 }, paragraphs: ['The engineer has not met you.'] },
      ] }),
    ],
    upgrades: [
      { id: 'tank', name: 'Fuel tank', description: 'Carries forty additional fuel.', price: 40,
        systemIds: ['home'], benefits: { fuelCapacity: 40 } },
      { id: 'sail', name: 'Sail', description: 'Reduces transit fuel by a quarter.', price: 30,
        requires: { reputation: 'engineer', min: 2 }, benefits: { travelEfficiency: 25 } },
      { id: 'lander', name: 'Lander package', description: 'Improves shielding and battery.', price: 10,
        benefits: { surveyShielding: 30, surveyBattery: 40 } },
    ],
    shipyard: [
      { id: 'skiff', ship: 1, name: 'Arilou Skiff', description: 'A nimble short-range escort.', price: 20,
        systemIds: ['home'], requires: { reputation: 'engineer', min: 2 } },
      { id: 'avatar', ship: 3, name: 'Chmmr Avatar', description: 'A heavy escort.', price: 80 },
    ],
  };
}

const ok = result => { assert.equal(result.ok, true, result.error); return result.state; };
const act = (data, state, type, rest = {}) => ok(applyAction(data, state, { type, ...rest }));
const begun = data => act(data, createGame(data), 'choose', { choiceId: 'begin' });
const choose = (data, state, choiceId) => act(data, state, 'choose', { choiceId });
const enter = (data, state, sceneId) => act(data, state, 'enter', { sceneId });
function endBattle(data, state, outcome) {
  return ok(resolveBattle(data, state, { id: state.pendingBattle.battleId, outcome,
    crew: outcome === 'defeat' ? 0 : 9, enemyRemaining: outcome === 'victory' ? 0 : 1, frames: 900 }));
}

test('equipment offers expose location, trust and price gates and purchases are atomic', () => {
  const data = fixture(); let state = createGame(data);
  assert.deepEqual(validateCampaign(data), { errors: [], warnings: [] });
  assert.match(getView(data, state).upgrades.find(u => u.id === 'sail').reason, /trust/);
  const denied = applyAction(data, state, { type: 'purchaseUpgrade', upgradeId: 'sail' });
  assert.equal(denied.ok, false); assert.equal(denied.state, state);
  state = choose(data, state, 'begin');
  state = act(data, state, 'purchaseUpgrade', { upgradeId: 'tank' });
  assert.equal(state.resources.credits, 160);
  assert.equal(state.resources.fuel, 100, 'capacity does not provide free fuel');
  assert.equal(getView(data, state).economy.fuelCapacity, 140);
  assert.equal(getView(data, state).upgrades[0].owned, true);
  assert.equal(applyAction(data, state, { type: 'purchaseUpgrade', upgradeId: 'tank' }).ok, false);
  state = act(data, state, 'refuel', { amount: 40 });
  assert.equal(state.resources.fuel, 140); assert.equal(state.resources.credits, 80);
  const loaded = loadSave(data, serializeSave(data, state));
  assert.equal(loaded.ok, true); assert.equal(loaded.state.resources.fuel, 140);
  state = act(data, state, 'travel', { systemId: 'field' });
  const other = structuredClone(state); delete other.flags.upgrade_tank; other.resources.fuel = 100;
  assert.match(getView(data, other).upgrades[0].reason, /shipyard/);
  assert.equal(applyAction(data, other, { type: 'purchaseUpgrade', upgradeId: 'tank' }).ok, false);
});

test('installed benefits change actual transit costs and surface capabilities without changing time', () => {
  const data = fixture(); let state = begun(data);
  const before = previewTravel(data, state, 'field');
  state = act(data, state, 'purchaseUpgrade', { upgradeId: 'sail' });
  state = act(data, state, 'purchaseUpgrade', { upgradeId: 'lander' });
  const after = previewTravel(data, state, 'field');
  assert.equal(before.fuel, 10); assert.equal(after.fuel, 8); assert.equal(after.days, before.days);
  assert.deepEqual(getCapabilities(data, state), { fuelCapacity: 100, travelEfficiency: 25, surveyShielding: 30, surveyBattery: 40 });
  const travelled = act(data, state, 'travel', { systemId: 'field' });
  assert.equal(travelled.resources.fuel, 92); assert.equal(travelled.day, after.days);
});

test('one-time equipment rewards apply capacity before fuel and do not require a save migration', () => {
  const data = fixture(); let state = begun(data);
  const old = serializeSave(data, state);
  assert.equal(loadSave(data, old).ok, true);
  state = enter(data, state, 'award'); state = choose(data, state, 'accept');
  assert.equal(state.resources.fuel, 140);
  assert.equal(state.resources.credits, 200);
  assert.equal(getView(data, state).upgrades[0].owned, true);
  assert.deepEqual(validateState(data, state).errors, []);
});

test('fleet purchases and repeated ship switches cannot heal or duplicate purchases', () => {
  const data = fixture(); let state = begun(data);
  state.resources.crew = 9;
  state = act(data, state, 'purchaseShip', { offerId: 'skiff' });
  assert.equal(state.ship, 5, 'buying a hull does not silently switch the player');
  assert.equal(state.resources.crew, 9); assert.equal(state.resources.credits, 180);
  state = act(data, state, 'switchShip', { ship: 1 });
  assert.equal(state.resources.crew, 6);
  assert.equal(state.reserveCrew, 3);
  state = act(data, state, 'switchShip', { ship: 5 });
  assert.equal(state.resources.crew, 9); assert.equal(state.reserveCrew, 0);
  assert.equal(applyAction(data, state, { type: 'purchaseShip', offerId: 'skiff' }).ok, false);
  assert.equal(applyAction(data, state, { type: 'switchShip', ship: 3 }).ok, false);
  state = act(data, state, 'purchaseShip', { offerId: 'avatar' });
  state = act(data, state, 'switchShip', { ship: 3 });
  assert.equal(state.resources.crew, 9); assert.equal(getView(data, state).economy.crewCapacity, 42);
  assert.equal(getView(data, state).fleet.filter(s => s.active).length, 1);
  assert.deepEqual(validateState(data, state).errors, []);
  assert.equal(loadSave(data, serializeSave(data, state)).ok, true);
});

test('a legacy save always owns its current hull and its starter after switching', () => {
  const data = fixture(); let state = begun(data);
  state.ship = 3; state.resources.crew = 31;
  assert.equal(Object.keys(state.flags).some(id => id.startsWith('ship_')), false);
  state = ok(loadSave(data, serializeSave(data, state)));
  assert.equal(getView(data, state).fleet.find(s => s.ship === 3).owned, true);
  state = act(data, state, 'switchShip', { ship: 5 });
  assert.equal(state.flags.ship_3, true);
  state = act(data, state, 'switchShip', { ship: 3 });
  assert.equal(state.resources.crew, 31);
});

test('all native hulls have valid ownership, capacity and save behavior', () => {
  const data = fixture(); let state = begun(data);
  for (const ship of SHIPS) state.flags[`ship_${ship.id}`] = true;
  for (const ship of SHIPS) {
    if (state.ship !== ship.id) state = act(data, state, 'switchShip', { ship: ship.id });
    assert.equal(getView(data, state).economy.crewCapacity, ship.maxCrew);
    assert.ok(state.resources.crew <= ship.maxCrew);
    assert.equal(getView(data, state).totalCrew, 18, 'transfers neither discard nor create crew');
    assert.equal(loadSave(data, serializeSave(data, state)).ok, true);
  }
});

test('crew reserves survive saves, remain vulnerable only when deployed, and transfer only in port', () => {
  const data = fixture(); let state = begun(data);
  state = act(data, state, 'purchaseShip', { offerId: 'skiff' });
  state = act(data, state, 'switchShip', { ship: 1 });
  assert.equal(state.reserveCrew, 12); assert.equal(state.resources.crew, 6);
  state = ok(loadSave(data, serializeSave(data, state)));
  state = enter(data, state, 'encounter'); state = choose(data, state, 'fight');
  state = ok(resolveBattle(data, state, { id: state.pendingBattle.battleId, outcome: 'retreat', crew: 3, enemyRemaining: 1, frames: 700 }));
  assert.equal(getView(data, state).totalCrew, 15);
  state = choose(data, state, 'regroup'); state = choose(data, state, 'continue');
  state = act(data, state, 'travel', { systemId: 'field' });
  const denied = applyAction(data, state, { type: 'switchShip', ship: 5 });
  assert.equal(denied.ok, false); assert.equal(denied.state, state);
  assert.match(getView(data, state).fleet.find(s => s.ship === 5).reason, /port/);
  state = act(data, state, 'travel', { systemId: 'home' });
  state = act(data, state, 'switchShip', { ship: 5 });
  assert.equal(state.resources.crew, 15); assert.equal(state.reserveCrew, 0);
  for (const invalid of [-1, 1.5, 1000001, '12']) {
    const corrupt = structuredClone(state); corrupt.reserveCrew = invalid;
    assert.ok(validateState(data, corrupt).errors.some(e => e.includes('reserve crew')));
  }
});

test('setback chains allow repeated retreats and defeat without duplicated effects; victory closes the encounter', () => {
  const data = fixture(); let state = begun(data);
  const ids = new Set();
  for (const outcome of ['retreat', 'retreat', 'defeat', 'retreat']) {
    state = enter(data, state, 'encounter'); state = choose(data, state, 'fight');
    assert.equal(ids.has(state.pendingBattle.battleId), false); ids.add(state.pendingBattle.battleId);
    const pending = state.pendingBattle.battleId;
    state = endBattle(data, state, outcome);
    assert.equal(state.scene, 'setback');
    assert.equal(resolveBattle(data, state, { id: pending, outcome: 'victory', crew: 9, enemyRemaining: 0, frames: 1 }).ok, false);
    state = choose(data, state, 'regroup'); assert.equal(state.scene, 'debrief');
    state = choose(data, state, 'continue');
    assert.equal(getView(data, state).availableScenes.some(s => s.id === 'encounter'), true);
    state = ok(loadSave(data, serializeSave(data, state)));
  }
  assert.equal(state.resources.credits, 174, '200 - first launch 5 + one recovery 7 - one debrief 3 - one defeat 25');
  assert.equal(state.usedChoices.filter(id => id === 'fight').length, 1);
  assert.equal(state.journal.filter(e => e.choiceId === 'regroup').length, 1);
  state = enter(data, state, 'encounter'); state = choose(data, state, 'fight');
  state = endBattle(data, state, 'victory'); state = choose(data, state, 'reward');
  assert.equal(state.resources.credits, 204);
  assert.equal(getView(data, state).availableScenes.some(s => s.id === 'encounter'), false);
  assert.equal(applyAction(data, state, { type: 'enter', sceneId: 'won' }).ok, false);
  assert.deepEqual(validateState(data, state).errors, []);
});

test('conversation variants use first matching history and the journal records exactly that text', () => {
  const data = fixture(); let state = begun(data);
  state = enter(data, state, 'callback');
  assert.deepEqual(getView(data, state).scene.paragraphs, ['The engineer remembers your meeting.']);
  assert.equal(state.journal.at(-1).text, getView(data, state).scene.paragraphs.join('\n\n'));
  state = act(data, state, 'leave');
  state = enter(data, state, 'callback');
  assert.equal(state.journal.filter(j => j.sceneId === 'callback').length, 1);
  const alternate = createGame(data); alternate.scene = null;
  const beforeMeeting = enter(data, alternate, 'callback');
  assert.deepEqual(getView(data, beforeMeeting).scene.paragraphs, ['The engineer has not met you.']);
  data.scenes.find(s => s.id === 'callback').variants = [{ requires: { flag: 'met' }, paragraphs: ['Only after meeting.'] }];
  assert.deepEqual(getView(data, beforeMeeting).scene.paragraphs, ['The callback encounter.']);
});

test('secured rescue deliveries survive recall, damage, save/load and reward exactly once', () => {
  const data = fixture();
  data.systems[0].survey = { mode: 'rescue', text: 'All three recovery groups are safe.', rewards: { credits: 40 } };
  let state = begun(data);
  state = act(data, state, 'launchSurvey');
  let pending = state.pendingSurvey.id;
  for (const completedTasks of [[1, 1], [3], [0, 1], 'one']) {
    const denied = resolveSurvey(data, state, { id: pending, outcome: 'retreated', beacons: 1,
      completedTasks, hullRemaining: 100, salvage: 0 });
    assert.equal(denied.ok, false); assert.equal(denied.state, state);
  }
  state = ok(resolveSurvey(data, state, { id: pending, outcome: 'retreated', beacons: 1,
    completedTasks: [1], hullRemaining: 100, salvage: 0 }));
  assert.deepEqual(state.surveyProgress.home, [1]); assert.equal(state.resources.credits, 200);
  state = ok(loadSave(data, serializeSave(data, state)));
  state = act(data, state, 'launchSurvey');
  state = ok(resolveSurvey(data, state, { id: state.pendingSurvey.id, outcome: 'damaged', beacons: 2,
    completedTasks: [1, 2], hullRemaining: 0, salvage: 0 }));
  assert.deepEqual(state.surveyProgress.home, [1, 2]); assert.equal(state.resources.credits, 200);
  state = act(data, state, 'launchSurvey'); pending = state.pendingSurvey.id;
  state = ok(resolveSurvey(data, state, { id: pending, outcome: 'surveyed', beacons: 3,
    completedTasks: [0, 1, 2], hullRemaining: 90, salvage: 0 }));
  assert.deepEqual(state.surveyProgress.home, [0, 1, 2]); assert.equal(state.resources.credits, 240);
  assert.equal(applyAction(data, state, { type: 'launchSurvey' }).ok, false);
  assert.equal(resolveSurvey(data, state, { id: pending, outcome: 'surveyed', beacons: 3,
    completedTasks: [0, 1, 2], hullRemaining: 90, salvage: 0 }).ok, false);
  for (const progress of [{ home: [0, 0] }, { home: [3] }, { field: [0] }, { home: 'one' }]) {
    const corrupted = structuredClone(state); corrupted.surveyProgress = progress;
    assert.ok(validateState(data, corrupted).errors.length);
  }
});

test('invalid equipment, hull offers and history variants fail content validation', () => {
  for (const change of [
    d => { d.upgrades[0].price = -1; },
    d => { d.upgrades[0].benefits = { travelEfficiency: 100 }; },
    d => { d.upgrades[0].benefits = { crew: 42 }; },
    d => { d.upgrades[0].systemIds = ['missing']; },
    d => { d.upgrades.push(structuredClone(d.upgrades[0])); },
    d => { d.shipyard[0].ship = 25; },
    d => { d.shipyard[1].ship = d.shipyard[0].ship; },
    d => { d.systems[0].survey = { text: 'Survey.', mode: 'recue' }; },
    d => { d.systems[0].survey = { text: 'Survey.', biome: 'oceean' }; },
    d => { d.systems[0].survey = { text: 'Survey.', difficulty: 99 }; },
    d => { d.scenes.at(-1).variants[0].paragraphs = []; },
    d => { d.scenes.at(-1).variants[0].requires = { reputation: 'missing', min: 2 }; },
  ]) {
    const data = fixture(); change(data);
    assert.ok(validateCampaign(data).errors.length, change.toString());
    assert.throws(() => createGame(data), /Invalid campaign/);
  }
});

test('authentic pre-revision campaign saves remain readable by the revised engine', async () => {
  const data = JSON.parse(await readFile(new URL('../content/campaign.json', import.meta.url), 'utf8'));
  const saves = JSON.parse(await readFile(new URL('./fixtures/pre-revision-saves.json', import.meta.url), 'utf8'));
  for (const [name, save] of Object.entries(saves.saves)) {
    const loaded = loadSave(data, JSON.stringify(save));
    assert.equal(loaded.ok, true, `${name}: ${loaded.error}`);
    assert.deepEqual(validateState(data, loaded.state).errors, []);
    assert.equal(getView(data, loaded.state).fleet.find(h => h.ship === 5).owned, true);
  }
});
