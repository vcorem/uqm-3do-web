/** The Afterlight Accord campaign rules. No DOM, timers, I/O, or random state. */
import { SHIPS } from './combat.mjs';
export const SAVE_SCHEMA_VERSION = 1;
const MAX_SAVE_BYTES = 4 * 1024 * 1024;
const MAX_JOURNAL = 6000;
const MAX_VALUE = 1000000;
const STATUS = new Set(['active', 'completed', 'failed']);
const ID = /^[a-z][a-z0-9_-]{0,79}$/;
const BAD_KEYS = new Set(['__proto__', 'constructor', 'prototype']);
const own = (o, key) => Object.prototype.hasOwnProperty.call(o, key);
const plain = o => o !== null && typeof o === 'object' && !Array.isArray(o)
  && [Object.prototype, null].includes(Object.getPrototypeOf(o));
const integer = n => Number.isSafeInteger(n);
const clone = value => structuredClone(value);
const unique = a => Array.isArray(a) && new Set(a).size === a.length;
const lookup = (data, type, id) => (data[type] ?? []).find(entry => entry.id === id);
const addUnique = (a, value) => { if (!a.includes(value)) a.push(value); };

/** Installed equipment is represented by ordinary validated campaign flags, preserving v1 saves. */
export function getCapabilities(data, state) {
  const benefits = { fuelCapacity: data.economy?.fuelCapacity ?? 100, travelEfficiency: 0,
    surveyShielding: 0, surveyBattery: 0 };
  for (const upgrade of data.upgrades ?? []) if (state?.flags?.[`upgrade_${upgrade.id}`]) {
    for (const [key, value] of Object.entries(upgrade.benefits ?? {})) benefits[key] += value;
  }
  benefits.travelEfficiency = Math.min(75, benefits.travelEfficiency);
  benefits.surveyShielding = Math.min(75, benefits.surveyShielding);
  benefits.surveyBattery = Math.min(300, benefits.surveyBattery);
  return benefits;
}

function economy(data, shipOrState = data.start?.ship ?? 5) {
  const state = plain(shipOrState) ? shipOrState : null;
  const ship = state?.ship ?? shipOrState;
  return { fuelCapacity: 100, refuelPrice: 2, repairPrice: 3,
    travelFuelScale: 8, travelDayScale: 12, distressDays: 7, distressFee: 50,
    ...(data.economy ?? {}), ...(state ? { fuelCapacity: getCapabilities(data, state).fuelCapacity } : {}),
    crewCapacity: SHIPS[ship]?.maxCrew ?? 18 };
}

const ownsShip = (data, state, ship) => ship === (data.start.ship ?? 5) || ship === state.ship || state.flags[`ship_${ship}`] === true;

/** First matching variant wins; the base remains a safe fallback for old playthroughs. */
export function resolveScene(data, state, sceneOrId) {
  const scene = typeof sceneOrId === 'string' ? lookup(data, 'scenes', sceneOrId) : sceneOrId;
  if (!scene) return null;
  const variant = (scene.variants ?? []).find(v => meets(state, v.requires));
  return variant ? { ...scene, paragraphs: variant.paragraphs } : scene;
}

function unsafeTree(value, depth = 0) {
  if (depth > 64) return true;
  if (value === null || typeof value !== 'object') return false;
  if (!Array.isArray(value) && !plain(value)) return true;
  return Object.keys(value).some(key => BAD_KEYS.has(key) || unsafeTree(value[key], depth + 1));
}

/** Validate authored registries, references, costs, predicates, and dialogue links. */
export function validateCampaign(data) {
  const errors = [], warnings = [];
  const err = (path, message) => errors.push(`${path}: ${message}`);
  if (!plain(data) || unsafeTree(data)) return { errors: ['Campaign must be safe plain JSON.'], warnings };
  if (!ID.test(data.id ?? '')) err('id', 'must be a stable lowercase identifier');
  if (!integer(data.version) || data.version < 1) err('version', 'must be a positive integer');
  if (typeof data.title !== 'string' || !data.title.trim()) err('title', 'is required');
  const registries = ['systems', 'characters', 'scenes', 'quests', 'endings', 'items', 'evidence'];
  const maps = {};
  for (const name of registries) {
    maps[name] = new Map();
    if (!Array.isArray(data[name])) { err(name, 'must be an array'); continue; }
    for (const [index, entry] of data[name].entries()) {
      if (!plain(entry)) { err(`${name}[${index}]`, 'must be an object'); continue; }
      if (!ID.test(entry.id ?? '')) err(`${name}[${index}].id`, 'invalid identifier');
      else if (maps[name].has(entry.id)) err(name, `duplicate id ${entry.id}`);
      else maps[name].set(entry.id, entry);
    }
  }
  const ref = (type, id, path) => { if (!maps[type].has(id)) err(path, `unknown ${type} id ${String(id)}`); };
  if (data.factions !== undefined && !Array.isArray(data.factions)) err('factions', 'must be an array');
  const factions = new Set(maps.characters.keys());
  for (const faction of Array.isArray(data.factions) ? data.factions : []) {
    if (!plain(faction) || !ID.test(faction.id ?? '') || factions.has(faction.id)) err('factions', 'requires unique valid faction objects');
    else factions.add(faction.id);
  }
  const flagsSet = new Set(Object.keys(data.start?.flags ?? {}));
  const flagsRequired = new Set();
  const condition = (c, path, depth = 0) => {
    if (!plain(c) || depth > 24) { err(path, 'invalid or excessively nested condition'); return; }
    const operators = ['all', 'any', 'not', 'flag', 'item', 'evidence', 'quest', 'resource', 'reputation', 'visited', 'day'];
    const present = operators.filter(key => own(c, key));
    if (present.length !== 1) { err(path, 'requires exactly one condition operator'); return; }
    const op = present[0];
    const allowedKeys = new Set([op, ...(op === 'flag' ? ['is'] : []),
      ...(['resource', 'reputation'].includes(op) ? ['min'] : []), ...(op === 'quest' ? ['status'] : [])]);
    for (const key of Object.keys(c)) if (!allowedKeys.has(key)) err(path, `unknown condition property ${key}`);
    if (op === 'all' || op === 'any') {
      if (!Array.isArray(c[op]) || c[op].length === 0) err(path, 'condition group must not be empty');
      else c[op].forEach((child, i) => condition(child, `${path}.${op}[${i}]`, depth + 1));
    } else if (op === 'not') condition(c.not, `${path}.not`, depth + 1);
    else if (op === 'flag') {
      if (!ID.test(c.flag ?? '')) err(path, 'invalid flag id');
      if (own(c, 'is') && typeof c.is !== 'boolean') err(path, 'flag is must be boolean');
      flagsRequired.add(c.flag);
    } else if (op === 'item') ref('items', c.item, path);
    else if (op === 'evidence') ref('evidence', c.evidence, path);
    else if (op === 'visited') ref('scenes', c.visited, path);
    else if (op === 'quest') { ref('quests', c.quest, path); if (!STATUS.has(c.status)) err(path, 'invalid quest status'); }
    else if (op === 'day') {
      if (!plain(c.day) || !Object.keys(c.day).length || Object.keys(c.day).some(k => !['min', 'max'].includes(k))
        || Object.values(c.day).some(v => !integer(v) || v < 0 || v > MAX_VALUE)
        || ((c.day.min ?? 0) > (c.day.max ?? MAX_VALUE))) err(path, 'invalid day range');
    }
    else if (op === 'resource') {
      if (!['credits', 'fuel', 'crew'].includes(c.resource)) err(path, 'unknown resource');
      if (!integer(c.min) || c.min < 0 || c.min > MAX_VALUE) err(path, 'invalid resource minimum');
    } else if (op === 'reputation') {
      if (!factions.has(c.reputation)) err(path, 'unknown reputation faction');
      if (!integer(c.min) || Math.abs(c.min) > 100) err(path, 'invalid reputation minimum');
    }
  };
  const effects = (e, path) => {
    if (!plain(e)) { err(path, 'effects must be an object'); return; }
    const allowed = new Set(['flags', 'credits', 'fuel', 'crew', 'days', 'reputation', 'giveItems',
      'takeItems', 'evidence', 'quests', 'unlockSystems', 'ending', 'journal', 'ship']);
    for (const key of Object.keys(e)) if (!allowed.has(key)) err(path, `unknown effect ${key}`);
    for (const key of ['credits', 'fuel', 'crew', 'days']) if (own(e, key)
      && (!integer(e[key]) || Math.abs(e[key]) > MAX_VALUE || (key === 'days' && e[key] < 0))) err(path, `invalid ${key} delta`);
    if (e.flags !== undefined) {
      if (!plain(e.flags)) err(path, 'flags must be an object');
      else for (const [key, value] of Object.entries(e.flags)) {
        if (!ID.test(key) || typeof value !== 'boolean') err(path, `invalid flag ${key}`);
        flagsSet.add(key);
      }
    }
    for (const [key, type] of [['giveItems', 'items'], ['takeItems', 'items'], ['evidence', 'evidence'], ['unlockSystems', 'systems']]) {
      if (e[key] !== undefined) {
        if (!unique(e[key])) err(path, `${key} must be an array of unique ids`);
        else e[key].forEach(id => ref(type, id, path));
      }
    }
    if (e.quests !== undefined) {
      if (!plain(e.quests)) err(path, 'quests must be an object');
      else for (const [id, status] of Object.entries(e.quests)) { ref('quests', id, path); if (!STATUS.has(status)) err(path, 'invalid quest status'); }
    }
    if (e.reputation !== undefined) {
      if (!plain(e.reputation)) err(path, 'reputation must be an object');
      else for (const [id, delta] of Object.entries(e.reputation)) {
        if (!factions.has(id) || !integer(delta) || Math.abs(delta) > 100) err(path, 'invalid reputation change');
      }
    }
    if (e.ending !== undefined) ref('endings', e.ending, path);
    if (e.ship !== undefined && (!integer(e.ship) || !SHIPS[e.ship])) err(path, 'ship must be a native Melee id in 0..24');
    if (e.journal !== undefined && (typeof e.journal !== 'string' || !e.journal.trim())) err(path, 'journal must be nonempty text');
  };
  const offerIds = new Set(), offeredShips = new Set();
  for (let ship = 0; ship < SHIPS.length; ship++) flagsSet.add(`ship_${ship}`);
  for (const kind of ['upgrades', 'shipyard']) {
    if (data[kind] !== undefined && !Array.isArray(data[kind])) { err(kind, 'must be an array'); continue; }
    for (const offer of data[kind] ?? []) {
      if (!plain(offer) || !ID.test(offer.id ?? '') || !ID.test(`upgrade_${offer.id}`) || offerIds.has(offer.id)) {
        err(kind, 'offers require unique valid ids'); continue;
      }
      offerIds.add(offer.id);
      const path = `${kind}.${offer.id}`;
      for (const key of ['name', 'description']) if (typeof offer[key] !== 'string' || !offer[key].trim()) err(path, `${key} is required`);
      if (!integer(offer.price) || offer.price < 0 || offer.price > MAX_VALUE) err(path, 'price must be a nonnegative integer');
      if (offer.requires) condition(offer.requires, `${path}.requires`);
      if (offer.systemIds !== undefined) {
        if (!unique(offer.systemIds) || !offer.systemIds.length) err(path, 'systemIds must be a nonempty array of unique systems');
        else offer.systemIds.forEach(id => ref('systems', id, path));
      }
      if (kind === 'upgrades') {
        flagsSet.add(`upgrade_${offer.id}`);
        const limits = { fuelCapacity: 250, travelEfficiency: 75, surveyShielding: 75, surveyBattery: 180 };
        if (!plain(offer.benefits) || !Object.keys(offer.benefits).length) err(path, 'equipment requires benefits');
        else for (const [key, value] of Object.entries(offer.benefits)) if (!own(limits, key) || !integer(value) || value <= 0 || value > limits[key]) err(path, `invalid equipment benefit ${key}`);
      } else {
        if (!integer(offer.ship) || !SHIPS[offer.ship]) err(path, 'ship must be a native Melee id in 0..24');
        else if (offeredShips.has(offer.ship)) err(path, 'each hull must have one shipyard offer');
        offeredShips.add(offer.ship);
      }
    }
  }
  for (const [id, system] of maps.systems) {
    if (typeof system.name !== 'string' || !system.name.trim()) err(id, 'system name is required');
    if (![system.x, system.y].every(n => typeof n === 'number' && Number.isFinite(n) && n >= 0 && n <= 100)) err(id, 'coordinates must be in 0..100');
    if (own(system, 'known') && typeof system.known !== 'boolean') err(id, 'known must be boolean');
    if (system.requires) condition(system.requires, `${id}.requires`);
    if (system.services !== undefined && (!plain(system.services)
      || Object.entries(system.services).some(([key, value]) => !['refuel', 'repair'].includes(key) || typeof value !== 'boolean')))
      err(id, 'services must contain boolean refuel/repair availability');
    if (system.survey !== undefined && !plain(system.survey)) err(id, 'survey must be an authored object');
    else if (system.survey) {
      if (typeof system.survey.text !== 'string' || !system.survey.text.trim()) err(id, 'survey requires authored text');
      if (own(system.survey, 'days') && (!integer(system.survey.days) || system.survey.days < 0)) err(id, 'invalid survey days');
      if (own(system.survey, 'mode') && !['survey', 'rescue', 'relay'].includes(system.survey.mode)) err(id, 'invalid surface mission mode');
      if (own(system.survey, 'biome') && !['ice', 'ocean', 'desert', 'basalt', 'crystal'].includes(system.survey.biome)) err(id, 'invalid surface biome');
      if (own(system.survey, 'difficulty') && !['gentle', 'standard', 'hazardous', 1, 2, 3].includes(system.survey.difficulty)) err(id, 'invalid surface difficulty');
      if (system.survey.rewards) effects(system.survey.rewards, `${id}.survey.rewards`);
    }
  }
  const choiceIds = new Set(), combatIds = new Set(), links = [];
  for (const [id, scene] of maps.scenes) {
    ref('systems', scene.systemId, id);
    ref('characters', scene.characterId, id);
    if (!integer(scene.act) || scene.act < 1 || scene.act > 4) err(id, 'act must be 1..4');
    if (typeof scene.title !== 'string' || !scene.title.trim()) err(id, 'title is required');
    if (!Array.isArray(scene.paragraphs) || scene.paragraphs.length === 0
      || scene.paragraphs.some(s => typeof s !== 'string' || !s.trim())) err(id, 'requires authored paragraphs');
    if (scene.requires) condition(scene.requires, `${id}.requires`);
    if (scene.variants !== undefined && !Array.isArray(scene.variants)) err(id, 'variants must be an array');
    for (const [index, variant] of (Array.isArray(scene.variants) ? scene.variants : []).entries()) {
      const path = `${id}.variants[${index}]`;
      if (!plain(variant) || !variant.requires || !Array.isArray(variant.paragraphs) || !variant.paragraphs.length
        || variant.paragraphs.some(p => typeof p !== 'string' || !p.trim())) { err(path, 'variant requires a condition and authored paragraphs'); continue; }
      condition(variant.requires, `${path}.requires`);
    }
    for (const key of ['repeatable', 'entry']) if (own(scene, key) && typeof scene[key] !== 'boolean') err(id, `${key} must be boolean`);
    if (!Array.isArray(scene.choices) || scene.choices.length === 0) { err(id, 'requires choices'); continue; }
    for (const choice of scene.choices) {
      const path = `${id}.${choice?.id}`;
      if (!plain(choice)) { err(id, 'invalid choice'); continue; }
      if (!ID.test(choice.id ?? '') || choiceIds.has(choice.id)) err(path, 'choice ids must be valid and globally unique');
      choiceIds.add(choice.id);
      if (typeof choice.text !== 'string' || !choice.text.trim()) err(path, 'choice text is required');
      if (choice.requires) condition(choice.requires, `${path}.requires`);
      if (choice.effects) effects(choice.effects, `${path}.effects`);
      for (const key of ['once', 'hidden']) if (own(choice, key) && typeof choice[key] !== 'boolean') err(path, `${key} must be boolean`);
      if (choice.once === false && choice.effects) err(path, 'repeatable choices must not grant or spend state effects');
      if (choice.next !== undefined && choice.next !== null) { ref('scenes', choice.next, path); links.push([id, choice.next, path]); }
      if (choice.combat) {
        const c = choice.combat;
        if (!plain(c) || !ID.test(c.id ?? '') || !Array.isArray(c.enemies) || c.enemies.length < 1 || c.enemies.length > 14
          || c.enemies.some(ship => !integer(ship) || !SHIPS[ship])) err(path, 'invalid combat specification');
        if (combatIds.has(c.id)) err(path, 'combat ids must be globally unique');
        combatIds.add(c.id);
        if (choice.once === false) err(path, 'authored combat encounters cannot be repeatable');
        for (const key of ['winSceneId', 'loseSceneId']) { ref('scenes', c[key], path); links.push([id, c[key], path]); }
        if (c.retreatSceneId !== undefined) { ref('scenes', c.retreatSceneId, path); links.push([id, c.retreatSceneId, path]); }
        if (choice.next) err(path, 'combat must use outcome scenes instead of next');
        if (choice.effects?.ending) err(path, 'combat launch cannot grant an ending');
      }
    }
  }
  for (const [from, to, path] of links) if (maps.scenes.has(to)
    && maps.scenes.get(from).systemId !== maps.scenes.get(to).systemId) err(path, 'dialogue link must stay in the same system; use travel between systems');
  for (const [id, ending] of maps.endings) {
    if (!Array.isArray(ending.paragraphs) || !ending.paragraphs.length
      || ending.paragraphs.some(p => typeof p !== 'string' || !p.trim())) err(id, 'ending requires authored paragraphs');
    if (ending.epilogues !== undefined && !Array.isArray(ending.epilogues)) err(id, 'epilogues must be an array');
    for (const epilogue of Array.isArray(ending.epilogues) ? ending.epilogues : []) {
      if (!plain(epilogue) || typeof epilogue.title !== 'string' || !epilogue.title.trim()
        || !Array.isArray(epilogue.paragraphs) || !epilogue.paragraphs.length
        || epilogue.paragraphs.some(p => typeof p !== 'string' || !p.trim())) { err(id, 'invalid authored epilogue'); continue; }
      if (epilogue.requires) condition(epilogue.requires, `${id}.epilogue`);
    }
  }
  if (data.events !== undefined && !Array.isArray(data.events)) err('events', 'must be an array');
  const eventIds = new Set();
  for (const event of Array.isArray(data.events) ? data.events : []) {
    if (!plain(event) || !ID.test(event.id ?? '') || eventIds.has(event.id)) { err('events', 'requires unique valid ids'); continue; }
    eventIds.add(event.id);
    if (!integer(event.day) || event.day < 0 || event.day > MAX_VALUE) err(event.id, 'event day must be nonnegative integer');
    if (typeof event.title !== 'string' || !event.title.trim() || typeof event.text !== 'string' || !event.text.trim()) err(event.id, 'event requires authored title and text');
    if (event.requires) condition(event.requires, `${event.id}.requires`);
    if (event.effects) {
      effects(event.effects, `${event.id}.effects`);
      if (event.effects.days) err(event.id, 'scheduled events cannot themselves advance time');
      if (event.effects.ending) err(event.id, 'scheduled events cannot silently end the main campaign');
    }
  }
  if (!plain(data.start)) err('start', 'is required');
  else {
    ref('systems', data.start.systemId, 'start');
    ref('scenes', data.start.sceneId, 'start');
    if (maps.scenes.get(data.start.sceneId)?.systemId !== data.start.systemId) err('start', 'scene must be at starting system');
    for (const key of ['credits', 'fuel', 'crew']) if (!integer(data.start[key]) || data.start[key] < 0 || data.start[key] > MAX_VALUE) err('start', `invalid ${key}`);
    if (data.start.flags) effects({ flags: data.start.flags }, 'start.flags');
    if (data.start.ship !== undefined && (!integer(data.start.ship) || !SHIPS[data.start.ship])) err('start', 'invalid ship');
  }
  const eco = economy(data);
  for (const [key, value] of Object.entries(eco)) if (!Number.isFinite(value) || value <= 0 || value > MAX_VALUE) err('economy', `invalid ${key}`);
  for (const key of ['fuelCapacity', 'crewCapacity', 'refuelPrice', 'repairPrice', 'distressDays', 'distressFee']) if (!integer(eco[key])) err('economy', `${key} must be integer`);
  if (data.start?.fuel > eco.fuelCapacity || data.start?.crew > eco.crewCapacity) err('start', 'starting resources exceed capacity');
  for (const flag of flagsRequired) if (!flagsSet.has(flag)) warnings.push(`Flag ${flag} is tested but never set by authored content.`);
  return { errors, warnings };
}

/** Evaluate only the small authored predicate language; never evaluates code. */
export function meets(state, condition) {
  if (condition === undefined || condition === null) return true;
  if (condition.all) return condition.all.every(c => meets(state, c));
  if (condition.any) return condition.any.some(c => meets(state, c));
  if (condition.not) return !meets(state, condition.not);
  if (condition.flag) return (state.flags[condition.flag] === true) === (condition.is ?? true);
  if (condition.item) return state.items.includes(condition.item);
  if (condition.evidence) return state.evidence.includes(condition.evidence);
  if (condition.quest) return state.quests[condition.quest] === condition.status;
  if (condition.resource) return state.resources[condition.resource] >= condition.min;
  if (condition.reputation) return (state.reputation[condition.reputation] ?? 0) >= condition.min;
  if (condition.visited) return state.visitedScenes.includes(condition.visited);
  if (condition.day) return state.day >= (condition.day.min ?? 0) && state.day <= (condition.day.max ?? MAX_VALUE);
  return false;
}

function conditionReason(state, condition) {
  if (meets(state, condition)) return null;
  if (condition?.all) return condition.all.map(c => conditionReason(state, c)).find(Boolean);
  if (condition?.resource) return `Requires ${condition.min} ${condition.resource}.`;
  if (condition?.item) return 'Requires an item you have not recovered.';
  if (condition?.evidence) return 'Requires evidence you have not recovered.';
  if (condition?.reputation) return 'This contact does not yet trust your proposal.';
  return 'Further discoveries or decisions are needed.';
}

function log(state, type, title, text, extra = {}) {
  // Navigation cannot exhaust save capacity; conversations and decisions are retained.
  if (['travel', 'service', 'recovery'].includes(type) && state.journal.filter(e => e.type === type).length >= 1500) return;
  if (type === 'decision' && extra.choiceId && state.journal.some(e => e.choiceId === extra.choiceId)) return;
  if (state.journal.length >= MAX_JOURNAL) throw new Error('The journal is full. Export a save before continuing.');
  state.journal.push({ id: state.journal.length + 1, day: state.day, type, title, text, ...extra });
}

function enterScene(data, state, id, automatic = false) {
  const scene = lookup(data, 'scenes', id);
  if (!scene || scene.systemId !== state.location) throw new Error('That conversation is not at this system.');
  if (!meets(state, scene.requires)) throw new Error(conditionReason(state, scene.requires));
  if (!automatic && scene.entry === false && !state.visitedScenes.includes(id)) throw new Error('This conversation follows from another encounter.');
  if (!automatic && !outcomeSceneAllowed(data, state, id)) throw new Error('This report is available only after the corresponding engagement or its authored alternative.');
  if (!scene.repeatable && state.completedScenes.includes(id)) {
    if (automatic && failurePathScene(data, state, id)) state.completedScenes = state.completedScenes.filter(done => done !== id);
    else throw new Error('That conversation is already complete. Read it again in the journal.');
  }
  state.scene = id;
  const firstVisit = !state.visitedScenes.includes(id);
  addUnique(state.visitedScenes, id);
  const transcript = resolveScene(data, state, scene).paragraphs.join('\n\n');
  const previous = state.journal.findLast(entry => entry.type === 'conversation' && entry.sceneId === id);
  if (firstVisit || previous?.text !== transcript) log(state, 'conversation', scene.title, transcript, { sceneId: id });
  // Automatic links also reveal their destination system for consistent saves.
  if (automatic) addUnique(state.knownSystems, scene.systemId);
}

export function createGame(data, options = {}) {
  const { errors } = validateCampaign(data);
  if (errors.length) throw new Error(`Invalid campaign:\n${errors.join('\n')}`);
  const state = {
    schemaVersion: SAVE_SCHEMA_VERSION, campaignId: data.id, campaignVersion: data.version,
    captainName: 'Captain', day: 0, location: data.start.systemId, scene: null,
    ship: data.start.ship ?? 5, reserveCrew: 0,
    resources: { credits: data.start.credits, fuel: data.start.fuel, crew: data.start.crew },
    flags: { ...(data.start.flags ?? {}) }, items: [], evidence: [], quests: {}, reputation: {},
    knownSystems: data.systems.filter(s => s.known !== false).map(s => s.id),
    visitedSystems: [data.start.systemId], visitedScenes: [], completedScenes: [],
    usedChoices: [], surveyedSystems: [], surveySalvage: {}, surveyProgress: {}, journal: [], ending: null, revision: 0,
    pendingBattle: null, pendingSurvey: null, battleOutcomes: {}, triggeredEvents: [],
  };
  addUnique(state.knownSystems, data.start.systemId);
  if (options.captainName !== undefined) state.captainName = cleanName(options.captainName);
  enterScene(data, state, data.start.sceneId, true);
  processDueEvents(data, state);
  return state;
}

function cleanName(name) {
  if (typeof name !== 'string') throw new Error('Captain name must be text.');
  const result = name.replace(/[\u0000-\u001f\u007f]/g, '').trim();
  if (result.length < 1 || result.length > 40) throw new Error('Use a captain name between 1 and 40 characters.');
  return result;
}

export function previewTravel(data, state, systemId) {
  const from = lookup(data, 'systems', state.location), to = lookup(data, 'systems', systemId), eco = economy(data, state);
  const result = { systemId, days: 0, fuel: 0, distance: 0, allowed: false, reason: null };
  if (!from || !to || !state.knownSystems.includes(systemId)) return { ...result, reason: 'This system has not been charted.' };
  if (systemId === state.location) return { ...result, reason: 'You are already here.' };
  result.distance = Math.hypot(to.x - from.x, to.y - from.y);
  result.days = Math.max(1, Math.ceil(result.distance / eco.travelDayScale));
  result.fuel = Math.max(1, Math.ceil(result.distance / eco.travelFuelScale * (1 - getCapabilities(data, state).travelEfficiency / 100)));
  if (state.ending) result.reason = 'This expedition has concluded.';
  else if (state.pendingBattle) result.reason = 'Resolve the current engagement first.';
  else if (state.pendingSurvey) result.reason = 'Return the surface expedition to the ship first.';
  else if (!meets(state, to.requires)) result.reason = conditionReason(state, to.requires);
  else if (state.resources.fuel < result.fuel) result.reason = `Requires ${result.fuel} fuel. Request a distress tow if stranded.`;
  else result.allowed = true;
  return result;
}

function effectCostReason(state, effects = {}) {
  for (const key of ['credits', 'fuel', 'crew']) {
    const available = key === 'crew' && effects.ship !== undefined ? SHIPS[effects.ship]?.crew : state.resources[key];
    if ((effects[key] ?? 0) < 0 && available + effects[key] < 0) return `Requires ${-effects[key]} ${key}.`;
  }
  for (const id of effects.takeItems ?? []) if (!state.items.includes(id)) return 'The required item is not aboard.';
  return null;
}

function failurePathScene(data, state, sceneId) {
  const choices = data.scenes.flatMap(s => s.choices), battles = choices.filter(c => c.combat).map(c => c.combat);
  const victories = new Set(battles.map(b => b.winSceneId));
  const pending = battles.filter(b => ['defeat', 'retreat'].includes(state.battleOutcomes[b.id]))
    .flatMap(b => [b.loseSceneId, b.retreatSceneId].filter(Boolean));
  const visited = new Set();
  while (pending.length) {
    const id = pending.pop();
    if (visited.has(id) || victories.has(id)) continue;
    if (id === sceneId) return true;
    visited.add(id);
    const scene = lookup(data, 'scenes', id);
    for (const choice of scene?.choices ?? []) if (choice.next && !choice.combat) pending.push(choice.next);
  }
  return false;
}

function choiceReason(data, state, choice) {
  const used = choice.once !== false && state.usedChoices.includes(choice.id);
  const retry = choice.combat && ['defeat', 'retreat'].includes(state.battleOutcomes[choice.combat.id]);
  if (choice.combat && state.battleOutcomes[choice.combat.id] === 'victory') return 'This engagement is already won.';
  if (used && !retry) return failurePathScene(data, state, state.scene) ? null : 'This decision has already been made.';
  if (choice.combat && state.resources.crew < 1) return 'Recover the crew before entering combat.';
  return conditionReason(state, choice.requires) || (used ? null : effectCostReason(state, choice.effects));
}

function outcomeSceneAllowed(data, state, sceneId) {
  if (state.visitedScenes.includes(sceneId)) return true;
  const encounters = data.scenes.flatMap(s => s.choices).filter(c => c.combat).map(c => c.combat);
  const relevant = encounters.filter(c => [c.winSceneId, c.loseSceneId, c.retreatSceneId].includes(sceneId));
  if (!relevant.length) return true;
  return relevant.some(c => {
    const outcome = state.battleOutcomes[c.id];
    return (outcome === 'victory' && c.winSceneId === sceneId)
      || (outcome === 'defeat' && c.loseSceneId === sceneId)
      || (outcome === 'retreat' && (c.retreatSceneId ?? c.loseSceneId) === sceneId);
  });
}

function deterministicSeed(id) {
  let seed = 2166136261;
  for (const char of id) seed = Math.imul(seed ^ char.charCodeAt(0), 16777619) >>> 0;
  return seed;
}

/** Concrete request for UQMCombat.start(); its id is also required in the receipt. */
export function getBattleRequest(data, state) {
  if (!state.pendingBattle) return null;
  return { id: state.pendingBattle.battleId, player: { ship: state.ship, crew: state.resources.crew },
    enemies: [...state.pendingBattle.enemies], seed: deterministicSeed(state.pendingBattle.battleId), difficulty: 'standard' };
}

function offerReason(state, offer, owned) {
  if (owned) return 'Already acquired.';
  if (state.ending) return 'This expedition has concluded.';
  if (state.pendingBattle) return 'Resolve the current engagement first.';
  if (state.pendingSurvey) return 'Return the surface expedition to the ship first.';
  if (offer.systemIds?.length && !offer.systemIds.includes(state.location)) return 'Visit its listed shipyard to acquire this equipment.';
  return conditionReason(state, offer.requires) || (state.resources.credits < offer.price ? `Requires ${offer.price} credits.` : null);
}

function fleetView(data, state) {
  const hulls = new Set([data.start.ship ?? 5, state.ship, ...(data.shipyard ?? []).map(o => o.ship)]);
  for (let ship = 0; ship < SHIPS.length; ship++) if (state.flags[`ship_${ship}`]) hulls.add(ship);
  return [...hulls].map(ship => {
    const offer = (data.shipyard ?? []).find(o => o.ship === ship), owned = ownsShip(data, state, ship), active = ship === state.ship;
    const reason = active ? 'Currently deployed.' : owned
      ? state.pendingBattle ? 'Resolve the current engagement first.' : state.pendingSurvey ? 'Return the surface expedition first.'
        : state.ending ? 'This expedition has concluded.' : !hasCrewTransferPort(data, state) ? 'Visit a fuel depot or repair port to transfer crews and change ships.' : null
      : offerReason(state, offer, false);
    return { ...(offer ?? {}), ship, name: offer?.name ?? SHIPS[ship].name, hull: SHIPS[ship],
      offerId: offer?.id ?? null, owned, active, enabled: !reason, reason,
      description: offer?.description ?? 'An available escort hull. The existing crew transfers when you change ships.' };
  });
}

function hasCrewTransferPort(data, state) {
  const services = lookup(data, 'systems', state.location)?.services;
  return !!(services?.refuel || services?.repair);
}

/** UI projection: labels remain authored; predicates never leak internal flag names. */
export function getView(data, state) {
  const system = lookup(data, 'systems', state.location);
  const scene = resolveScene(data, state, state.scene);
  const choices = (scene?.choices ?? []).map(choice => {
    const reason = state.pendingBattle ? 'Resolve the current engagement first.'
      : state.pendingSurvey ? 'Return the surface expedition to the ship first.' : choiceReason(data, state, choice);
    return { ...choice, enabled: !reason && !state.ending, reason };
  }).filter(choice => !choice.hidden || choice.enabled);
  return {
    system, scene, choices, economy: economy(data, state), ship: SHIPS[state.ship], pendingBattle: state.pendingBattle, pendingSurvey: state.pendingSurvey,
    capabilities: getCapabilities(data, state), fleet: fleetView(data, state),
    reserveCrew: state.reserveCrew ?? 0, totalCrew: state.resources.crew + (state.reserveCrew ?? 0),
    upgrades: (data.upgrades ?? []).map(offer => {
      const owned = state.flags[`upgrade_${offer.id}`] === true, reason = offerReason(state, offer, owned);
      return { ...offer, owned, enabled: !reason, reason };
    }),
    ending: getEnding(data, state),
    availableScenes: data.scenes.filter(s => (s.entry !== false || state.visitedScenes.includes(s.id)) && s.systemId === state.location && s.id !== state.scene
      && (s.repeatable || !state.completedScenes.includes(s.id)) && meets(state, s.requires) && outcomeSceneAllowed(data, state, s.id)
      && s.choices.some(c => !choiceReason(data, { ...state, scene: s.id }, c))),
    travelOptions: data.systems.filter(s => state.knownSystems.includes(s.id))
      .map(s => ({ ...s, ...previewTravel(data, state, s.id) })),
    quests: data.quests.filter(q => state.quests[q.id]).map(q => ({ ...q, status: state.quests[q.id] })),
    evidence: data.evidence.filter(e => state.evidence.includes(e.id)),
    items: data.items.filter(i => state.items.includes(i.id)),
    canSurvey: !!system?.survey && state.resources.crew > 0 && !state.surveyedSystems.includes(system.id) && !state.pendingBattle && !state.pendingSurvey && !state.ending,
  };
}

/** Include only epilogues earned by the actual completed playthrough. */
export function getEnding(data, state) {
  const ending = lookup(data, 'endings', state.ending);
  return ending ? { ...ending, epilogues: (ending.epilogues ?? []).filter(e => meets(state, e.requires)) } : null;
}

function applyEffects(data, state, effects = {}, schedule = true) {
  const reason = effectCostReason(state, effects);
  if (reason) throw new Error(reason);
  if (effects.ship !== undefined) {
    state.flags[`ship_${state.ship}`] = true;
    state.ship = effects.ship;
    state.flags[`ship_${effects.ship}`] = true;
    state.resources.crew = SHIPS[effects.ship].crew;
  }
  Object.assign(state.flags, effects.flags ?? {});
  const eco = economy(data, state);
  for (const key of ['credits', 'fuel', 'crew']) {
    const maximum = key === 'credits' ? MAX_VALUE : eco[`${key}Capacity`];
    state.resources[key] = Math.min(maximum, state.resources[key] + (effects[key] ?? 0));
  }
  for (const id of effects.takeItems ?? []) state.items = state.items.filter(item => item !== id);
  for (const id of effects.giveItems ?? []) addUnique(state.items, id);
  for (const id of effects.evidence ?? []) addUnique(state.evidence, id);
  for (const id of effects.unlockSystems ?? []) addUnique(state.knownSystems, id);
  Object.assign(state.quests, effects.quests ?? {});
  for (const [id, delta] of Object.entries(effects.reputation ?? {})) state.reputation[id] = Math.max(-100, Math.min(100, (state.reputation[id] ?? 0) + delta));
  if (schedule) advanceDays(data, state, effects.days ?? 0);
  if (effects.journal) log(state, 'discovery', 'Captain’s log', effects.journal);
  if (effects.ending) {
    state.ending = effects.ending;
    state.scene = null;
    const ending = getEnding(data, state);
    const text = [...ending.paragraphs, ...ending.epilogues.flatMap(e => [e.title, ...e.paragraphs])].join('\n\n');
    log(state, 'ending', ending.title, text, { endingId: ending.id });
  }
}

function processDueEvents(data, state) {
  const events = [...(data.events ?? [])].sort((a, b) => a.day - b.day || a.id.localeCompare(b.id));
  let changed = true;
  while (changed) {
    changed = false;
    for (const event of events) if (!state.triggeredEvents.includes(event.id) && event.day <= state.day && meets(state, event.requires)) {
      const eventEffects = clone(event.effects ?? {});
      // World losses are consequences, never unaffordable transaction gates.
      for (const resource of ['credits', 'fuel', 'crew']) if ((eventEffects[resource] ?? 0) < 0)
        eventEffects[resource] = Math.max(eventEffects[resource], -(resource === 'crew' && eventEffects.ship !== undefined
          ? SHIPS[eventEffects.ship].crew : state.resources[resource]));
      if ((eventEffects.takeItems ?? []).some(id => !state.items.includes(id))) continue;
      addUnique(state.triggeredEvents, event.id);
      applyEffects(data, state, eventEffects, false);
      log(state, 'world', event.title, event.text, { eventId: event.id });
      changed = true;
    }
  }
}

function dayBoundaries(condition, into) {
  if (!condition) return;
  if (condition.day) {
    if (condition.day.min !== undefined) into.add(condition.day.min);
    if (condition.day.max !== undefined) into.add(condition.day.max + 1);
  }
  if (condition.not) dayBoundaries(condition.not, into);
  for (const child of [...(condition.all ?? []), ...(condition.any ?? [])]) dayBoundaries(child, into);
}

/** Visit every relevant time boundary, making bulk and one-day advancement equivalent. */
function advanceDays(data, state, days) {
  const target = state.day + days, boundaries = new Set([target]);
  for (const event of data.events ?? []) { boundaries.add(event.day); dayBoundaries(event.requires, boundaries); }
  processDueEvents(data, state);
  for (const day of [...boundaries].filter(n => n > state.day && n <= target).sort((a, b) => a - b)) {
    state.day = day;
    processDueEvents(data, state);
  }
}

function failure(state, error, code = 'invalid_action') {
  return { ok: false, state, events: [], error: error instanceof Error ? error.message : String(error), code };
}

/** Atomic state transition: on any error, returns the original unmodified state. */
export function applyAction(data, state, action) {
  const validity = validateState(data, state);
  if (validity.errors.length) return failure(state, 'The campaign state is invalid. Load a validated save.', 'invalid_state');
  if (!plain(action) || unsafeTree(action) || typeof action.type !== 'string') return failure(state, 'Invalid action.');
  if (state.ending && action.type !== 'rename') return failure(state, 'This expedition has concluded. Start a new expedition to make different choices.');
  if (state.pendingBattle && !['rename', 'abandonBattle'].includes(action.type)) return failure(state, 'Resolve the current engagement first.');
  if (state.pendingSurvey && !['rename', 'abandonSurvey'].includes(action.type)) return failure(state, 'Return the surface expedition to the ship first.');
  const next = clone(state), logStart = next.journal.length;
  try {
    switch (action.type) {
      case 'rename': next.captainName = cleanName(action.captainName); break;
      case 'leave': next.scene = null; break;
      case 'enter':
        if (next.scene) throw new Error('Close the current conversation first.');
        enterScene(data, next, action.sceneId); break;
      case 'travel': {
        const preview = previewTravel(data, next, action.systemId);
        if (!preview.allowed) throw new Error(preview.reason);
        next.resources.fuel -= preview.fuel; advanceDays(data, next, preview.days);
        next.location = action.systemId; next.scene = null;
        addUnique(next.visitedSystems, action.systemId);
        log(next, 'travel', `Arrived at ${lookup(data, 'systems', action.systemId).name}`,
          `The crossing took ${preview.days} days and ${preview.fuel} fuel.`);
        break;
      }
      case 'choose': {
        const scene = lookup(data, 'scenes', next.scene);
        if (!scene) throw new Error('There is no active conversation.');
        const choice = scene.choices.find(c => c.id === action.choiceId);
        if (!choice) throw new Error('That response is not part of this conversation.');
        const reason = choiceReason(data, next, choice);
        if (reason) throw new Error(reason);
        // A retry may revisit its setback dialogue, but earned effects never repeat.
        if (!next.usedChoices.includes(choice.id)) applyEffects(data, next, choice.effects);
        if (choice.once !== false) addUnique(next.usedChoices, choice.id);
        if (!scene.repeatable) addUnique(next.completedScenes, scene.id);
        log(next, 'decision', scene.title, choice.text, { sceneId: scene.id, choiceId: choice.id });
        next.scene = null;
        if (choice.combat) {
          next.pendingBattle = { ...choice.combat, battleId: `${data.id}:${next.revision + 1}:${choice.combat.id}`, choiceId: choice.id, systemId: next.location };
          log(next, 'battle', 'Engagement begins', `An encounter with ${choice.combat.enemies.map(id => SHIPS[id].name).join(', ')} has begun.`);
        } else if (!next.ending && choice.next) enterScene(data, next, choice.next, true);
        break;
      }
      case 'survey': case 'launchSurvey': {
        const system = lookup(data, 'systems', next.location);
        if (!system.survey) throw new Error('No survey is available here.');
        if (next.surveyedSystems.includes(system.id)) throw new Error('This survey has already been completed.');
        if (next.resources.crew < 1) throw new Error('Recover the crew before launching a surface expedition.');
        const id = `${data.id}:${next.revision + 1}:survey:${system.id}`;
        next.pendingSurvey = { id, systemId: system.id, seed: deterministicSeed(id) };
        next.scene = null;
        break;
      }
      case 'refuel': case 'repair': {
        const system = lookup(data, 'systems', next.location), eco = economy(data, next);
        if (!system.services?.[action.type]) throw new Error(`This system has no ${action.type === 'refuel' ? 'fuel depot' : 'crew recovery facility'}.`);
        const resource = action.type === 'refuel' ? 'fuel' : 'crew';
        if (!integer(action.amount) || action.amount <= 0) throw new Error('Choose a positive whole amount.');
        const remaining = eco[`${resource}Capacity`] - next.resources[resource];
        if (action.amount > remaining) throw new Error('That amount exceeds the ship’s capacity.');
        const cost = action.amount * eco[`${action.type}Price`];
        if (!integer(cost) || cost > next.resources.credits) throw new Error(`Requires ${cost} credits.`);
        next.resources.credits -= cost; next.resources[resource] += action.amount;
        log(next, 'service', action.type === 'refuel' ? 'Refuelled' : 'Crew recovered', `${action.amount} ${resource} restored for ${cost} credits.`);
        break;
      }
      case 'purchaseUpgrade': {
        const offer = lookup(data, 'upgrades', action.upgradeId);
        if (!offer) throw new Error('That equipment is not available.');
        const reason = offerReason(next, offer, next.flags[`upgrade_${offer.id}`] === true);
        if (reason) throw new Error(reason);
        next.resources.credits -= offer.price;
        next.flags[`upgrade_${offer.id}`] = true;
        log(next, 'service', `${offer.name} installed`, `${offer.description} Installation costs ${offer.price} credits.`);
        break;
      }
      case 'purchaseShip': {
        const offer = lookup(data, 'shipyard', action.offerId);
        if (!offer) throw new Error('That vessel is not available.');
        const reason = offerReason(next, offer, ownsShip(data, next, offer.ship));
        if (reason) throw new Error(reason);
        next.resources.credits -= offer.price;
        next.flags[`ship_${offer.ship}`] = true;
        log(next, 'service', `${offer.name} joins the fleet`, `${offer.description} Acquisition costs ${offer.price} credits. Select the vessel to transfer your existing crew.`);
        break;
      }
      case 'switchShip': {
        if (!integer(action.ship) || !SHIPS[action.ship] || !ownsShip(data, next, action.ship)) throw new Error('That hull has not joined your fleet.');
        if (next.ship === action.ship) throw new Error('That vessel is already deployed.');
        if (!hasCrewTransferPort(data, next)) throw new Error('Visit a fuel depot or repair port to transfer crews and change ships.');
        const total = next.resources.crew + (next.reserveCrew ?? 0);
        next.flags[`ship_${next.ship}`] = true;
        next.flags[`ship_${action.ship}`] = true;
        next.ship = action.ship;
        next.resources.crew = Math.min(total, SHIPS[action.ship].maxCrew);
        next.reserveCrew = total - next.resources.crew;
        log(next, 'service', `${SHIPS[action.ship].name} deployed`, `${next.resources.crew} crew transfer aboard; ${next.reserveCrew} remain aboard the tender. Changing vessels preserves the total available crew.`);
        break;
      }
      case 'distress': {
        const eco = economy(data, next), home = lookup(data, 'systems', data.start.systemId);
        const fee = Math.min(eco.distressFee, next.resources.credits);
        next.resources.credits -= fee; next.resources.fuel = Math.max(next.resources.fuel, Math.min(40, eco.fuelCapacity));
        next.resources.crew = Math.max(next.resources.crew, Math.ceil(SHIPS[next.ship].crew / 2));
        advanceDays(data, next, eco.distressDays); next.location = home.id; next.scene = null;
        addUnique(next.knownSystems, home.id); addUnique(next.visitedSystems, home.id);
        log(next, 'recovery', 'A light answers', `A reconstruction tender tows the ship to ${home.name}. Rescue takes ${eco.distressDays} days; the contribution is ${fee} credits. Nobody is abandoned for lack of funds.`);
        break;
      }
      case 'abandonBattle':
        if (!next.pendingBattle) throw new Error('There is no engagement to abandon.');
        return resolveBattle(data, state, { id: next.pendingBattle.battleId, outcome: 'retreat', crew: next.resources.crew,
          enemyRemaining: next.pendingBattle.enemies.length, frames: 0 });
      case 'abandonSurvey':
        if (!next.pendingSurvey) throw new Error('There is no surface expedition to recall.');
        return resolveSurvey(data, state, { id: next.pendingSurvey.id, outcome: 'retreated', salvage: 0, hullRemaining: 100, beacons: 0 });
      default: throw new Error('Unknown campaign action.');
    }
    processDueEvents(data, next);
    next.revision++;
    const check = validateState(data, next);
    if (check.errors.length) throw new Error(`Action would create invalid state: ${check.errors[0]}`);
    return { ok: true, state: next, events: next.journal.slice(logStart) };
  } catch (error) { return failure(state, error); }
}

/** Surface-adapter result. Authored discoveries require an actual completed expedition. */
export function resolveSurvey(data, state, receipt) {
  if (validateState(data, state).errors.length) return failure(state, 'Invalid campaign state.', 'invalid_state');
  const authoredSurvey = lookup(data, 'systems', state.pendingSurvey?.systemId)?.survey;
  if (!plain(receipt) || !state.pendingSurvey || receipt.id !== state.pendingSurvey.id
    || !['surveyed', 'retreated', 'damaged'].includes(receipt.outcome)
    || !integer(receipt.salvage) || receipt.salvage < 0 || receipt.salvage > 60
    || !integer(receipt.hullRemaining) || receipt.hullRemaining < 0 || receipt.hullRemaining > 100
    || !integer(receipt.beacons) || receipt.beacons < 0 || receipt.beacons > 3
    || (receipt.outcome === 'surveyed' && (receipt.beacons !== 3 || receipt.hullRemaining === 0))
    || (receipt.completedTasks !== undefined && (authoredSurvey?.mode !== 'rescue'
      || !unique(receipt.completedTasks) || receipt.completedTasks.length !== receipt.beacons
      || receipt.completedTasks.some(id => !integer(id) || id < 0 || id > 2))))
    return failure(state, 'Invalid, stale, or duplicate surface-expedition result.', 'invalid_survey_result');
  const next = clone(state), pending = next.pendingSurvey, logStart = next.journal.length;
  try {
    const system = lookup(data, 'systems', pending.systemId), survey = system.survey;
    next.pendingSurvey = null;
    if (survey.mode === 'rescue') {
      next.surveyProgress ??= {};
      const completed = new Set(next.surveyProgress[system.id] ?? []);
      for (const id of receipt.completedTasks ?? (receipt.outcome === 'surveyed' ? [0, 1, 2] : [])) completed.add(id);
      next.surveyProgress[system.id] = [...completed].sort();
    }
    const recovered = Math.min(receipt.salvage, 60 - (next.surveySalvage[system.id] ?? 0));
    next.surveySalvage[system.id] = (next.surveySalvage[system.id] ?? 0) + recovered;
    next.resources.credits = Math.min(MAX_VALUE, next.resources.credits + recovered);
    if (receipt.outcome === 'damaged') {
      const injured = Math.min(3, Math.max(1, Math.ceil((100 - receipt.hullRemaining) / 34)));
      next.resources.crew = Math.max(0, next.resources.crew - injured);
    }
    advanceDays(data, next, receipt.outcome === 'surveyed' ? Math.max(1, survey.days ?? 1) : 1);
    if (receipt.outcome === 'surveyed') {
      applyEffects(data, next, survey.rewards);
      addUnique(next.surveyedSystems, system.id);
      log(next, 'survey', survey.title ?? `${system.name} survey`, survey.text, { systemId: system.id });
    } else log(next, 'expedition', receipt.outcome === 'damaged' ? 'Surface team recovered' : 'Surface team recalled',
      (receipt.outcome === 'damaged' ? 'The damaged lander returned without completing the survey. Up to three crew require recovery; the expedition can be attempted again.'
        : 'The lander returned before completing the survey. Its discoveries remain incomplete; another expedition is possible.')
        + (survey.mode === 'rescue' && next.surveyProgress[system.id].length
          ? ` ${next.surveyProgress[system.id].length} recovery objectives are secured aboard and will remain safe on the next attempt.` : ''), { systemId: system.id });
    if (recovered) log(next, 'salvage', 'Recovered material', `${recovered} credits of usable material recovered from ${system.name}.`, { systemId: system.id });
    processDueEvents(data, next); next.revision++;
    const check = validateState(data, next);
    if (check.errors.length) throw new Error(check.errors[0]);
    return { ok: true, state: next, events: next.journal.slice(logStart) };
  } catch (error) { return failure(state, error, 'invalid_survey_result'); }
}

/** Called by the native UQM battle adapter; ordinary dialogue actions cannot claim a win. */
export function resolveBattle(data, state, receipt) {
  if (validateState(data, state).errors.length) return failure(state, 'Invalid campaign state.', 'invalid_state');
  if (!plain(receipt) || !state.pendingBattle || receipt.id !== state.pendingBattle.battleId
    || !['victory', 'defeat', 'retreat'].includes(receipt.outcome)
    || !integer(receipt.crew) || receipt.crew < 0 || receipt.crew > SHIPS[state.ship].maxCrew
    || !integer(receipt.enemyRemaining) || receipt.enemyRemaining < 0 || receipt.enemyRemaining > state.pendingBattle.enemies.length
    || !integer(receipt.frames) || receipt.frames < 0
    || (receipt.outcome === 'victory' && (receipt.enemyRemaining !== 0 || receipt.crew < 1))
    || (receipt.outcome === 'defeat' && receipt.crew !== 0)) return failure(state, 'Invalid, stale, or duplicate battle result.', 'invalid_battle_result');
  const next = clone(state), battle = next.pendingBattle, logStart = next.journal.length;
  try {
    next.pendingBattle = null; next.scene = null;
    next.battleOutcomes[battle.id] = receipt.outcome;
    next.resources.crew = receipt.crew;
    if (receipt.outcome === 'defeat') {
      next.resources.credits = Math.max(0, next.resources.credits - 25);
      next.resources.crew = Math.max(1, Math.ceil(SHIPS[next.ship].crew / 2));
      advanceDays(data, next, 3);
    } else if (receipt.outcome === 'retreat' && receipt.crew === 0) {
      next.resources.crew = 1;
    }
    advanceDays(data, next, 1);
    log(next, 'battle', receipt.outcome === 'victory' ? 'Engagement won' : 'Emergency withdrawal',
      receipt.outcome === 'victory' ? 'The native combat engine confirmed victory.'
        : receipt.outcome === 'defeat' ? 'A reconstruction tender recovers the survivors and a replacement vessel. Recovery costs three days and up to 25 credits; half the normal complement returns to duty.'
          : 'The ship disengages. Surviving crew return with it.',
      { battleId: battle.battleId, outcome: receipt.outcome });
    const destination = receipt.outcome === 'victory' ? battle.winSceneId
      : receipt.outcome === 'retreat' ? (battle.retreatSceneId ?? battle.loseSceneId) : battle.loseSceneId;
    if (receipt.outcome !== 'victory') {
      const source = data.scenes.find(s => s.choices.some(c => c.id === battle.choiceId));
      if (source) next.completedScenes = next.completedScenes.filter(id => id !== source.id);
    }
    enterScene(data, next, destination, true);
    next.revision++;
    const check = validateState(data, next);
    if (check.errors.length) throw new Error(check.errors[0]);
    return { ok: true, state: next, events: next.journal.slice(logStart) };
  } catch (error) { return failure(state, error, 'invalid_battle_result'); }
}

/** Strict save validation rejects unknown ids, impossible resources, and malformed state. */
export function validateState(data, state) {
  const errors = [], err = message => errors.push(message);
  if (!plain(state) || unsafeTree(state)) return { errors: ['State must be safe plain JSON.'] };
  if (state.schemaVersion !== SAVE_SCHEMA_VERSION) err('Unsupported save schema version.');
  if (state.campaignId !== data.id || state.campaignVersion !== data.version) err('Save belongs to a different campaign version.');
  const allowed = new Set(['schemaVersion', 'campaignId', 'campaignVersion', 'captainName', 'day', 'location', 'scene',
    'resources', 'flags', 'items', 'evidence', 'quests', 'reputation', 'knownSystems', 'visitedSystems', 'visitedScenes',
    'completedScenes', 'usedChoices', 'surveyedSystems', 'surveySalvage', 'surveyProgress', 'journal', 'ending', 'revision',
    'pendingBattle', 'pendingSurvey', 'ship', 'reserveCrew', 'battleOutcomes', 'triggeredEvents']);
  for (const key of Object.keys(state)) if (!allowed.has(key)) err(`Unknown state field ${key}.`);
  try { if (cleanName(state.captainName) !== state.captainName) err('Invalid captain name.'); } catch { err('Invalid captain name.'); }
  for (const key of ['day', 'revision']) if (!integer(state[key]) || state[key] < 0 || state[key] > MAX_VALUE) err(`Invalid ${key}.`);
  if (!lookup(data, 'systems', state.location)) err('Unknown current system.');
  const scene = state.scene === null ? null : lookup(data, 'scenes', state.scene);
  if (state.scene !== null && (!scene || scene.systemId !== state.location)) err('Invalid active conversation.');
  if (!integer(state.ship) || !SHIPS[state.ship]) err('Invalid combat ship.');
  if (state.reserveCrew !== undefined && (!integer(state.reserveCrew) || state.reserveCrew < 0
    || state.reserveCrew > MAX_VALUE || state.reserveCrew + (state.resources?.crew ?? 0) > MAX_VALUE)) err('Invalid reserve crew.');
  const eco = economy(data, state);
  if (!plain(state.resources) || Object.keys(state.resources).length !== 3) err('Invalid resources.');
  else for (const key of ['credits', 'fuel', 'crew']) {
    const max = key === 'credits' ? MAX_VALUE : eco[`${key}Capacity`];
    if (!integer(state.resources[key]) || state.resources[key] < 0 || state.resources[key] > max) err(`Invalid ${key} resource.`);
  }
  const allowedFlags = new Set(Object.keys(data.start?.flags ?? {}));
  for (const upgrade of data.upgrades ?? []) allowedFlags.add(`upgrade_${upgrade.id}`);
  for (let ship = 0; ship < SHIPS.length; ship++) allowedFlags.add(`ship_${ship}`);
  for (const s of data.systems) for (const key of Object.keys(s.survey?.rewards?.flags ?? {})) allowedFlags.add(key);
  for (const s of data.scenes) for (const c of s.choices) for (const key of Object.keys(c.effects?.flags ?? {})) allowedFlags.add(key);
  for (const e of data.events ?? []) for (const key of Object.keys(e.effects?.flags ?? {})) allowedFlags.add(key);
  if (!plain(state.flags) || Object.entries(state.flags).some(([key, value]) => !allowedFlags.has(key) || typeof value !== 'boolean')) err('Invalid campaign flags.');
  const choices = new Set(data.scenes.flatMap(s => s.choices.map(c => c.id)));
  for (const [key, registry] of [['items', 'items'], ['evidence', 'evidence'], ['knownSystems', 'systems'],
    ['visitedSystems', 'systems'], ['surveyedSystems', 'systems'], ['visitedScenes', 'scenes'], ['completedScenes', 'scenes']]) {
    if (!unique(state[key]) || state[key].some(id => !lookup(data, registry, id))) err(`Invalid ${key}.`);
  }
  if (!unique(state.usedChoices) || state.usedChoices.some(id => !choices.has(id))) err('Invalid used choices.');
  if (!plain(state.quests) || Object.entries(state.quests).some(([id, value]) => !lookup(data, 'quests', id) || !STATUS.has(value))) err('Invalid quest states.');
  const factions = new Set([...(data.factions ?? []).map(f => f.id), ...data.characters.map(c => c.id)]);
  if (!plain(state.reputation) || Object.entries(state.reputation).some(([id, value]) => !factions.has(id) || !integer(value) || Math.abs(value) > 100)) err('Invalid reputations.');
  const combats = new Set(data.scenes.flatMap(s => s.choices.filter(c => c.combat).map(c => c.combat.id)));
  if (!plain(state.battleOutcomes) || Object.entries(state.battleOutcomes).some(([id, outcome]) => !combats.has(id)
    || !['victory', 'defeat', 'retreat'].includes(outcome))) err('Invalid battle outcomes.');
  if (!unique(state.triggeredEvents) || state.triggeredEvents.some(id => !(data.events ?? []).some(e => e.id === id))) err('Invalid triggered events.');
  if (!plain(state.surveySalvage) || Object.entries(state.surveySalvage).some(([id, value]) => !lookup(data, 'systems', id)?.survey
    || !integer(value) || value < 0 || value > 60)) err('Invalid surface salvage totals.');
  if (state.surveyProgress !== undefined && (!plain(state.surveyProgress) || Object.entries(state.surveyProgress).some(([id, tasks]) =>
    lookup(data, 'systems', id)?.survey?.mode !== 'rescue' || !unique(tasks)
    || tasks.some(task => !integer(task) || task < 0 || task > 2)))) err('Invalid secured surface recovery groups.');
  if (Array.isArray(state.knownSystems) && !state.knownSystems.includes(state.location)) err('Current system must be charted.');
  if (Array.isArray(state.visitedSystems) && !state.visitedSystems.includes(state.location)) err('Current system must have been visited.');
  if (Array.isArray(state.completedScenes) && Array.isArray(state.visitedScenes)
    && state.completedScenes.some(id => !state.visitedScenes.includes(id))) err('Completed scenes must have been visited.');
  if (scene && Array.isArray(state.visitedScenes) && !state.visitedScenes.includes(scene.id)) err('Active scene must have been visited.');
  if (scene && !scene.repeatable && Array.isArray(state.completedScenes) && state.completedScenes.includes(scene.id)) err('A completed conversation cannot remain active.');
  if (Array.isArray(state.usedChoices) && Array.isArray(state.visitedScenes)) for (const id of state.usedChoices) {
    const parent = data.scenes.find(s => s.choices.some(c => c.id === id));
    if (parent && !state.visitedScenes.includes(parent.id)) err('A used choice must belong to a visited scene.');
  }
  if (state.ending !== null && !lookup(data, 'endings', state.ending)) err('Unknown ending.');
  if (state.ending && (state.scene || state.pendingBattle || state.pendingSurvey)) err('A concluded campaign cannot have an active scene or expedition.');
  if (!Array.isArray(state.journal) || state.journal.length > MAX_JOURNAL) err('Invalid journal.');
  else state.journal.forEach((entry, index) => {
    if (!plain(entry)) { err(`Invalid journal entry ${index}.`); return; }
    if (entry.id !== index + 1 || !integer(entry.day) || entry.day < 0 || entry.day > state.day
      || typeof entry.type !== 'string' || typeof entry.title !== 'string' || typeof entry.text !== 'string'
      || entry.text.length > 50000 || entry.title.length > 1000) err(`Invalid journal entry ${index}.`);
    if (entry.sceneId && !lookup(data, 'scenes', entry.sceneId)) err('Journal references an unknown scene.');
    if (entry.choiceId && !choices.has(entry.choiceId)) err('Journal references an unknown choice.');
  });
  if (state.pendingBattle !== null) {
    const b = state.pendingBattle;
    const choice = plain(b) ? data.scenes.flatMap(s => s.choices).find(c => c.id === b.choiceId) : null;
    if (!choice?.combat || b.systemId !== state.location || state.scene !== null
      || typeof b.battleId !== 'string' || !b.battleId.startsWith(`${data.id}:`)
      || state.battleOutcomes?.[choice?.combat?.id] === 'victory'
      || !Array.isArray(state.usedChoices) || !state.usedChoices.includes(b.choiceId)) err('Invalid pending battle.');
    else for (const key of ['id', 'enemies', 'winSceneId', 'loseSceneId', 'retreatSceneId']) if (JSON.stringify(b[key]) !== JSON.stringify(choice.combat[key])) err('Pending battle differs from authored encounter.');
  }
  if (state.pendingSurvey !== null) {
    const s = state.pendingSurvey;
    if (!plain(s) || !lookup(data, 'systems', s.systemId)?.survey || s.systemId !== state.location
      || state.scene !== null || state.pendingBattle !== null || typeof s.id !== 'string'
      || !s.id.startsWith(`${data.id}:`) || s.id.length > 200 || !integer(s.seed) || s.seed !== deterministicSeed(s.id)
      || (Array.isArray(state.surveyedSystems) && state.surveyedSystems.includes(s.systemId))) err('Invalid pending surface expedition.');
  }
  return { errors };
}

export function serializeSave(data, state) {
  const check = validateState(data, state);
  if (check.errors.length) throw new Error(`Cannot save invalid state: ${check.errors[0]}`);
  const text = JSON.stringify({ format: 'afterlight-save', schemaVersion: SAVE_SCHEMA_VERSION, state }, null, 2);
  if (new TextEncoder().encode(text).length > MAX_SAVE_BYTES) throw new Error('Save exceeds the 4 MiB limit.');
  return text;
}

export function loadSave(data, text) {
  if (typeof text !== 'string' || new TextEncoder().encode(text).length > MAX_SAVE_BYTES)
    return { ok: false, state: null, error: 'Save is not text or exceeds the 4 MiB limit.' };
  try {
    const parsed = JSON.parse(text);
    if (!plain(parsed) || unsafeTree(parsed) || parsed.format !== 'afterlight-save'
      || parsed.schemaVersion !== SAVE_SCHEMA_VERSION || Object.keys(parsed).some(k => !['format', 'schemaVersion', 'state'].includes(k))) throw new Error('Unsupported save format.');
    const check = validateState(data, parsed.state);
    if (check.errors.length) throw new Error(check.errors[0]);
    return { ok: true, state: parsed.state };
  } catch (error) { return { ok: false, state: null, error: `Could not load save: ${error.message}` }; }
}
