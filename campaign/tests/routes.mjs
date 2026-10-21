/** Shared real-action itineraries for state tests and UI playthroughs.
 * No state injection, debug flags, or reward shortcuts. A browser can replay
 * the returned actions through the visible controls and compare the result.
 */
import { createGame, applyAction, previewTravel } from '../src/state.mjs';

export const OPENING_CHOICES = Object.freeze(['a1_wake_duty', 'a1_brief_question', 'a1_terms_depart',
  'a1_cinder_listen', 'a1_name_welcome', 'a1_container_build', 'a1_report_publish',
  'a1_mara_promise', 'a1_interval_origin', 'a1_forum_councils', 'a1_compact_launch']);

export const LEAD_CHOICES = Object.freeze({
  roots: ['a2_ember_wait', 'a2_ember_depart', 'a2_meadow_consent', 'a2_roots_complete'],
  promises: ['a2_bell_work', 'a2_bell_take', 'a2_promises_complete'],
  smallcourage: ['a2_ferry_manual', 'a2_ferry_complete'],
  redress: ['a2_witness_copy', 'a2_redress_council', 'a2_redress_complete'],
  synthesis: ['a2_thread_slow', 'a2_thread_take', 'a2_synthesis_complete'],
  dissent: ['a2_splinter_hear', 'a2_shear_truce', 'a2_dissent_complete'],
});

export function opening() { return [...OPENING_CHOICES]; }
export function thirdActRoute(lead = 'promises') {
  if (!LEAD_CHOICES[lead]) throw new Error(`Unknown lead ${lead}.`);
  return [...OPENING_CHOICES, ...LEAD_CHOICES[lead], 'a2_quiet_enter'];
}

export const INVESTIGATION_CHOICES = Object.freeze([
  'a3_glass_listen', 'a3_glass_central', 'a3_glass_complete',
  'a3_unwritten_preserve', 'a3_drafts_people', 'a3_archive_close',
  'a3_sera_account', 'a3_sera_next', 'a3_crisis_depart',
]);

export const ACCORD_PREPARATION_CHOICES = Object.freeze([
  'a3_undertow_publish', 'a3_undertow_labor', 'a3_grid_finish',
  'a3_vigil_pledge', 'a3_vigil_transfer', 'a3_anchor_take',
  'a3_fault_clock', 'a3_fault_record', 'a3_fault_depart',
  'a3_harbor_full', 'a3_harbor_complete',
]);

/** The three complete routes use actual dialogue, travel, costs and recoveries.
 * They deliberately avoid combat/survey shortcuts: native battles and physical
 * landings are exercised separately, with their genuine completion receipts.
 */
export const ENDING_ROUTES = Object.freeze({
  accord: Object.freeze([...OPENING_CHOICES, ...LEAD_CHOICES.roots,
    ...LEAD_CHOICES.promises, ...LEAD_CHOICES.synthesis, 'a2_quiet_enter',
    ...INVESTIGATION_CHOICES, ...ACCORD_PREPARATION_CHOICES,
    'a4_table_listen', 'a4_testimony_costs', 'a4_choose_accord',
    'a4_junction_bypass', 'a4_junction_finish',
    'a4_accord_review', 'a4_accord_accept', 'a4_accord_end']),
  mandate: Object.freeze([...thirdActRoute(), ...INVESTIGATION_CHOICES,
    'a4_table_listen', 'a4_testimony_costs', 'a4_choose_mandate',
    'a4_junction_bypass', 'a4_junction_finish',
    'a4_mandate_expiry', 'a4_mandate_watch', 'a4_mandate_end']),
  partition: Object.freeze([...thirdActRoute(), ...INVESTIGATION_CHOICES,
    'a4_table_listen', 'a4_testimony_costs', 'a4_choose_partition',
    'a4_junction_bypass', 'a4_junction_finish',
    'a4_partition_corridor', 'a4_partition_witness', 'a4_partition_end']),
});

/** Optional quests and character visits, including every principal relationship.
 * This deliberately circuitous expedition also exercises the rescue economy.
 */
export const GRAND_TOUR_CHOICES = Object.freeze([
  ...OPENING_CHOICES,
  'a1_tallow_help', 'a1_tallow_record', 'a1_moss_offer', 'a1_moss_depart',
  'a1_nacre_yes', 'a1_nacre_depart', 'a1_keeper_archive', 'a1_keeper_take',
  'a1_lowtide_honest', 'a1_lilt_friend', 'a1_sera_answer', 'a2_pilgrim_fix',
  ...LEAD_CHOICES.roots, ...LEAD_CHOICES.promises, ...LEAD_CHOICES.smallcourage,
  ...LEAD_CHOICES.redress, ...LEAD_CHOICES.synthesis, ...LEAD_CHOICES.dissent,
  'a2_quiet_enter', 'a2_shoal_records', 'a2_pell_depart', 'a2_nacre_repair',
  'a2_pell_listen', 'a2_return_private', 'a2_return_finish', 'a2_hush_rescue',
  ...INVESTIGATION_CHOICES.map(c => c === 'a3_glass_central' ? 'a3_glass_local' : c),
  'a3_lilt_contacts', 'a3_shear_independent', 'a3_moss_specific', 'a3_sera_workers',
  'a3_tallow_policy', 'a3_silence_request', 'a3_silence_finish', 'a3_keeper_witness',
  ...ACCORD_PREPARATION_CHOICES, 'a3_convoy_record', 'a3_convoy_finish',
  'a3_nacre_privacy', 'a4_review_self', 'a4_lilt_story', 'a4_supper_finish',
  ...ENDING_ROUTES.accord.slice(ENDING_ROUTES.accord.indexOf('a4_table_listen')),
]);

const authorityAlternatives = Object.freeze({
  a1_report_publish: 'a1_report_direct', a1_forum_councils: 'a1_forum_accept',
  a2_ember_wait: 'a2_ember_order', a2_meadow_consent: 'a2_meadow_priority',
  a2_bell_work: 'a2_bell_hold', a2_redress_council: 'a2_redress_expert',
  a2_thread_slow: 'a2_thread_force', a2_thread_take: 'a2_thread_forced_take',
  a2_shear_truce: 'a2_shear_order', a2_return_private: 'a2_return_public',
  a3_drafts_people: 'a3_drafts_precaution', a3_silence_request: 'a3_silence_force',
  a3_nacre_privacy: 'a3_nacre_open',
});
/** A contrasting expedition that records losses, intrusive choices, and an
 * explicit rejection of the captain's recommendation by the governed council.
 */
export const AUTHORITY_TOUR_CHOICES = Object.freeze([
  ...GRAND_TOUR_CHOICES.slice(0, GRAND_TOUR_CHOICES.indexOf('a4_table_listen'))
    .flatMap(c => c === 'a2_redress_council' ? ['a2_redress_expert', 'a2_redress_accept_council']
      : [authorityAlternatives[c] ?? c]),
  ...ENDING_ROUTES.mandate.slice(ENDING_ROUTES.mandate.indexOf('a4_table_listen'))
    .map(c => c === 'a4_mandate_expiry' ? 'a4_mandate_board'
      : c === 'a4_mandate_watch' ? 'a4_mandate_speed' : c),
]);

/** Produce and execute ordinary valid actions, preserving actual costs and days.
 * The returned state is an oracle only; a browser must replay actions in its UI.
 * Optional initialState supports continuing a prior genuine save or battle result.
 */
export function planItinerary(data, choiceIds, initialState = createGame(data)) {
  let state = initialState;
  const actions = [];
  const perform = action => {
    const result = applyAction(data, state, action);
    if (!result.ok) throw new Error(`${JSON.stringify(action)}: ${result.error}`);
    state = result.state; actions.push(action);
  };
  for (const choiceId of choiceIds) {
    const scene = data.scenes.find(s => s.choices.some(c => c.id === choiceId));
    if (!scene) throw new Error(`No authored choice ${choiceId}.`);
    if (state.scene !== scene.id) {
      if (state.scene) perform({ type: 'leave' });
      if (state.location !== scene.systemId) {
        const preview = previewTravel(data, state, scene.systemId);
        if (!preview.allowed && state.resources.fuel < preview.fuel) perform({ type: 'distress' });
        if (state.location !== scene.systemId) perform({ type: 'travel', systemId: scene.systemId });
      }
      perform({ type: 'enter', sceneId: scene.id });
    }
    perform({ type: 'choose', choiceId });
    if (state.pendingBattle || state.pendingSurvey) {
      if (choiceId !== choiceIds.at(-1)) throw new Error('Resolve the physical encounter before planning further dialogue.');
    }
  }
  return { actions, state };
}
