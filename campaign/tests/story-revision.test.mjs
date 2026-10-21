// Tests of authored consequences and solvable narrative operations. These use
// ordinary actions; they do not claim that an automated route measures fun.
import test from 'node:test';
import assert from 'node:assert/strict';
import fs from 'node:fs';
import {createGame,applyAction,getView,getEnding,resolveScene,validateCampaign,validateState,meets,loadSave} from '../src/state.mjs';
import {OPENING_CHOICES,LEAD_CHOICES,INVESTIGATION_CHOICES,ENDING_ROUTES,thirdActRoute,planItinerary} from './routes.mjs';
const data=JSON.parse(fs.readFileSync(new URL('../content/campaign.json',import.meta.url)));
const route=(choices,state)=>planItinerary(data,choices,state).state;
const perform=(state,action)=>{const r=applyAction(data,state,action);assert.ok(r.ok,r.error);return r.state;};
const untilFinale=()=>route([...thirdActRoute(),...INVESTIGATION_CHOICES]);
const prose=(state,id)=>resolveScene(data,state,id).paragraphs.join('\n');

test('authored content validates and every surface carries a practical discovery',()=>{
 assert.deepEqual(validateCampaign(data),{errors:[],warnings:[]});
 const surfaces=data.systems.filter(s=>s.survey).map(s=>s.survey);
 assert.equal(surfaces.length,20);
 assert.equal(new Set(surfaces.map(s=>s.mode)).size,3);
 assert.equal(new Set(surfaces.map(s=>s.biome)).size,5);
 for(const s of surfaces){
  assert.equal(s.objectiveLabels.length,3);
  assert.ok(s.rewards.evidence?.length||s.rewards.giveItems?.length||s.rewards.flags);
 }
 assert.equal(data.scenes.flatMap(s=>s.choices).filter(c=>c.combat).length,5);
});

test('opening equipment decision changes usable resources and the first investigation preserves its chosen method',()=>{
 const rescue=perform(createGame(data),{type:'choose',choiceId:'a1_wake_duty'});
 const research=perform(createGame(data),{type:'choose',choiceId:'a1_wake_chair'});
 assert.equal(rescue.resources.fuel-research.resources.fuel,15);
 assert.equal(rescue.resources.credits-research.resources.credits,20);
 assert.ok(research.items.includes('archive_shunt'));
 const measured=route(OPENING_CHOICES.map(x=>x==='a1_cinder_listen'?'a1_cinder_measure':x));
 assert.ok(measured.flags.signal_measurement&&measured.flags.signal_tested&&measured.flags.upgrade_survey_navigation);
 const interrupted=route(OPENING_CHOICES.map(x=>x==='a1_signal_callback'?'a1_signal_power':x));
 assert.match(prose(interrupted,'a1_signal_answer'),/request screams/);
 assert.doesNotMatch(prose(measured,'a1_signal_answer'),/request screams/);
});

test('publication gets an immediate breach response and cannot complete through private-delivery prose',()=>{
 const start=route(thirdActRoute());
 const published=route(['a2_return_public'],start);
 assert.equal(published.scene,'a2_return_exposed');
 assert.match(getView(data,published).scene.paragraphs.join(' '),/TAKE MY FATHER OFF YOUR SCREEN/);
 const result=applyAction(data,published,{type:'choose',choiceId:'a2_return_finish'});
 assert.equal(result.ok,false);
 const repaired=route(['a2_return_hearing'],published);
 assert.equal(repaired.quests.return,'completed');
 assert.ok(repaired.flags.letters_public&&repaired.flags.letters_redacted);
 const privateRoute=route(['a2_return_private','a2_return_finish'],start);
 assert.ok(privateRoute.flags.letters_private&&!privateRoute.flags.letters_public);
});

test('ordering Shear requires a specific withdrawal or physical agreement before reconciliation',()=>{
 const ordered=route([...OPENING_CHOICES,'a2_splinter_hear','a2_shear_order']);
 assert.equal(ordered.scene,'a2_shear_refusal');
 assert.equal(applyAction(data,ordered,{type:'choose',choiceId:'a2_dissent_complete'}).ok,false);
 const repaired=route(['a2_shear_repair','a2_dissent_complete'],ordered);
 assert.ok(repaired.flags.shear_pressured&&repaired.flags.shear_apology&&repaired.flags.shear_truce);
 assert.equal(repaired.day,ordered.day+2);
});

test('forced welfare contact causes an isolation repair and retains its breach record',()=>{
 const start=route(LEAD_CHOICES.dissent,untilFinale());
 const breach=route(['a3_silence_force'],start);
 assert.equal(breach.scene,'a3_silence_breach');
 assert.match(getView(data,breach).scene.paragraphs.join(' '),/ACCESS FORCED/);
 const done=route(['a3_silence_breach_work'],breach);
 assert.ok(done.flags.silence_forced&&done.flags.silence_repaired);
 assert.ok(!done.flags.silence_respected);
 assert.equal(done.day,breach.day+2);
});

test('minimal finale does not invent optional supper or personal memories',()=>{
 const state=untilFinale();
 assert.ok(!state.flags.thermal_trial&&!state.flags.keeper_accountable&&!state.flags.lilt_chair);
 const supper=prose(state,'a4_supper');
 assert.doesNotMatch(supper,/fruit from the thermal trial|apprentice who corrected/);
 assert.doesNotMatch(prose(state,'a3_lilt_truth'),/Low Tide arguing|first empty plate/);
 assert.doesNotMatch(prose(state,'a3_convoy_arrive'),/Anet/);
});

test('investigations allow understandable mistakes and recover through evidence without softlocking',()=>{
 const opening=route(OPENING_CHOICES.map(x=>x==='a1_signal_callback'?'a1_signal_power':x));
 let state=route([...LEAD_CHOICES.promises,'a2_quiet_enter'],opening);
 state=route(['a3_glass_test','a3_glass_private_files','a3_glass_back_to_records','a3_glass_diagnose_staff','a3_glass_diagnose_retry','a3_glass_central','a3_glass_complete',
  'a3_unwritten_verify','a3_archive_playback','a3_archive_live_test','a3_archive_diagnose_merge','a3_archive_merge_abort','a3_drafts_precaution','a3_archive_central','a3_archive_close'],state);
 assert.ok(state.flags.glass_misdiagnosed&&state.flags.glass_diagnosed&&state.flags.archive_merge_considered&&state.flags.archive_diagnosed);
 assert.ok(state.evidence.includes('closed_exit')&&state.evidence.includes('unwritten_answers'));
 assert.deepEqual(validateState(data,state).errors,[]);
});

test('earlier harvest earns a hull, a late recovery has a lasting purchase cost',()=>{
 const early=route([...OPENING_CHOICES,...LEAD_CHOICES.roots,'a2_growers_accept_blade']);
 assert.ok(early.flags.ship_16&&!early.flags.winter_warning);
 let late=route(OPENING_CHOICES);
 for(let i=0;i<7;i++)late=perform(late,{type:'distress'});
 assert.ok(late.flags.winter_warning);
 late=route(['a2_ember_restart_work',...LEAD_CHOICES.roots],late);
 assert.ok(late.flags.winter_repaired&&!late.flags.ship_16);
 const offer=data.shipyard.find(o=>o.ship===16);
 assert.equal(offer.price,90);
 assert.ok(meets(late,offer.requires));
 assert.ok(late.resources.credits<offer.price,'the lost grant has a genuine affordability consequence');
 assert.equal(applyAction(data,late,{type:'enter',sceneId:'a2_growers_grant'}).ok,false);
});

test('forced clocks affect a real ship loan until the named repair is completed',()=>{
 let state=route([...OPENING_CHOICES,'a2_thread_force','a2_thread_forced_take','a2_synthesis_complete']);
 assert.ok(state.reputation.ilex<2);
 assert.ok(!getView(data,state).fleet.find(o=>o.ship===9).enabled);
 state=route(['a2_thread_repair_trace'],state);
 assert.ok(state.reputation.ilex>=2&&state.flags.thread_reconciled);
 assert.ok(getView(data,state).fleet.find(o=>o.ship===9).enabled);
});

test('three endings require three different performed engineering operations',()=>{
 const outcomes={};
 for(const [name,choices] of Object.entries(ENDING_ROUTES)){
  const state=route(choices);outcomes[name]=state;
  assert.equal(state.ending,name);
  assert.ok(state.flags.junction_ready);
  assert.deepEqual(validateState(data,state).errors,[]);
 }
 assert.ok(outcomes.accord.flags.junction_live_refusal);
 assert.ok(outcomes.mandate.flags.mandate_failover_tested&&outcomes.mandate.flags.mandate_external_key);
 assert.ok(outcomes.partition.flags.partition_witnessed&&outcomes.partition.flags.raft_evacuated);
 assert.ok(!outcomes.accord.flags.raft_evacuated&&!outcomes.partition.flags.junction_live_refusal);
 assert.notDeepEqual(getEnding(data,outcomes.accord).paragraphs,getEnding(data,outcomes.mandate).paragraphs);
});

test('late Lastlight can be repaired through ordinary actions and cannot skip its operation',()=>{
 let state=untilFinale();
 for(let i=0;i<15;i++)state=perform(state,{type:'distress'});
 assert.ok(state.flags.lastlight_strained);
 state=route(['a4_table_listen','a4_testimony_costs','a4_choose_mandate','a4_junction_bypass'],state);
 assert.equal(applyAction(data,state,{type:'choose',choiceId:'a4_junction_finish'}).ok,false);
 state=route(['a4_junction_cool_work','a4_junction_begin_mandate','a4_mandate_build_panel','a4_mandate_loadtest','a4_mandate_set_breakers','a4_junction_finish'],state);
 assert.ok(state.flags.lastlight_cooled&&state.flags.junction_ready);
});

test('formerly empty destinations produce capability and a distinctive partition rescue',()=>{
 let state=untilFinale();
 state=route(['a3_crossing_double','a3_crossing_retry','a3_crossing_complete','a3_home_work','a3_home_finish'],state);
 assert.ok(state.flags.upgrade_crossing_vanes&&state.flags.home_open);
 assert.equal(state.quests.crossing,'completed');assert.equal(state.quests.homeward,'completed');
 state=route(['a4_table_listen','a4_testimony_costs','a4_choose_partition','a4_junction_bypass','a4_junction_begin_partition','a4_partition_home','a4_partition_bridge','a4_partition_separate','a4_junction_finish'],state);
 assert.ok(state.flags.raft_home&&!state.flags.raft_evacuated);
});

test('pump deadline crossed during the operation still requires coolant before ratification',()=>{
 const prefix=ENDING_ROUTES.mandate.slice(0,ENDING_ROUTES.mandate.indexOf('a4_junction_begin_mandate'));
 let state=route(prefix);
 for(let i=0;i<7;i++){
  state=perform(state,{type:'distress'});
  state=perform(state,{type:'travel',systemId:'lastlight'});
 }
 assert.equal(state.day,119);
 state=perform(state,{type:'enter',sceneId:'a4_junction_clear'});
 assert.ok(!state.flags.lastlight_strained);
 state=route(['a4_junction_begin_mandate','a4_mandate_build_panel','a4_mandate_loadtest','a4_mandate_set_breakers'],state);
 assert.ok(state.flags.lastlight_strained&&!state.flags.lastlight_cooled);
 assert.match(getView(data,state).scene.paragraphs.join(' '),/holding pump has passed day 120/);
 assert.equal(applyAction(data,state,{type:'choose',choiceId:'a4_junction_finish'}).ok,false);
 state=route(['a4_junction_cool_work','a4_junction_finish'],state);
 assert.ok(state.flags.lastlight_cooled&&state.flags.junction_ready);
});

test('seed taken before the cold cycle can still restore its mother beds afterward',()=>{
 let state=route([...OPENING_CHOICES,'a2_ember_wait','a2_ember_depart']);
 assert.ok(state.flags.ember_resolved&&!state.flags.winter_warning);
 while(state.day<45)state=perform(state,{type:'distress'});
 state=route(['a2_meadow_consent','a2_roots_complete'],state);
 assert.ok(state.flags.winter_warning&&!state.flags.winter_repaired);
 state=route(['a2_ember_return_work'],state);
 assert.ok(state.flags.winter_repaired);
 assert.ok(meets(state,data.shipyard.find(o=>o.ship===16).requires));
 assert.deepEqual(validateState(data,state).errors,[]);
});

test('an early convoy records actual accepted fallback berths before its witnessed arrival',()=>{
 const state=route([...thirdActRoute(),'a2_shoal_hold','a2_pell_depart','a3_convoy_chime','a3_convoy_finish']);
 assert.ok(!state.flags.glass_investigated&&!state.flags.harbor_ready);
 assert.ok(state.flags.convoy_hearth_berths&&state.flags.pell_supports);
 assert.match(prose(state,'a3_convoy_arrive'),/Hearth’s confirmed temporary cabins/);
 assert.doesNotMatch(prose(state,'a3_convoy_arrive'),/Open Harbor receives/);
});

test('chair and archive hearing do not invent optional contacts or private adventures',()=>{
 const opening=route([...OPENING_CHOICES,'a1_lilt_friend']);
 assert.ok(!opening.flags.lowtide_heard&&!opening.flags.lowtide_supported);
 assert.match(prose(opening,'a1_lilt_chair'),/no personal reply/);
 assert.doesNotMatch(prose(opening,'a1_lilt_chair'),/Hessa’s actual reply/);
 const minimal=route(['a3_nacre_privacy'],untilFinale());
 assert.ok(!minimal.flags.lilt_individual&&!minimal.flags.letters_resolved&&!minimal.flags.shear_plural);
 assert.doesNotMatch(prose(minimal,'a3_nacre_hearing'),/Lilt’s chosen emergency contacts|Return Address decision|Shear’s meeting/);
});

test('a dismantled corridor must be physically rebuilt before adopting the corridor ending',()=>{
 const prefix=ENDING_ROUTES.partition.slice(0,ENDING_ROUTES.partition.indexOf('a4_partition_bridge'));
 let state=route(prefix);
 state=route(['a4_partition_store','a4_partition_separate','a4_junction_finish'],state);
 state=perform(state,{type:'travel',systemId:'assembly'});
 state=perform(state,{type:'enter',sceneId:'a4_partition_terms'});
 assert.ok(state.flags.partition_stocked&&!state.flags.partition_bridge_held);
 assert.equal(applyAction(data,state,{type:'choose',choiceId:'a4_partition_corridor'}).ok,false);
 const day=state.day;
 state=route(['a4_partition_rebuild_corridor','a4_partition_corridor','a4_partition_witness','a4_partition_end'],state);
 assert.equal(state.day,day+2);
 const titles=getEnding(data,state).epilogues.map(e=>e.title);
 assert.ok(titles.includes('The corridor rebuilt by hand')&&titles.includes('The cargo already on the islands'));
});

test('central and physically independent mandate panels produce different recorded failure consequences',()=>{
 const local=route(ENDING_ROUTES.mandate);
 const central=route(ENDING_ROUTES.mandate.map(c=>c==='a4_mandate_build_panel'?'a4_mandate_central_panel':c));
 const localTitles=getEnding(data,local).epilogues.map(e=>e.title);
 const centralTitles=getEnding(data,central).epilogues.map(e=>e.title);
 assert.ok(localTitles.includes('The panel outside the short circuit'));
 assert.ok(!localTitles.includes('The bus both desks shared'));
 assert.ok(centralTitles.includes('The bus both desks shared'));
 assert.ok(!centralTitles.includes('The panel outside the short circuit'));
 assert.equal(local.day-central.day,2);
});

test('winter arriving during graft work offers a usable recovery and preserves the uncollected seed',()=>{
 let state=route(OPENING_CHOICES);
 for(let i=0;i<5;i++)state=perform(state,{type:'distress'});
 state=perform(state,{type:'travel',systemId:'ember'});
 assert.equal(state.day,44);
 state=perform(state,{type:'enter',sceneId:'a2_ember'});
 state=perform(state,{type:'choose',choiceId:'a2_ember_wait'});
 assert.equal(state.day,48);
 assert.ok(state.flags.winter_warning&&state.flags.ember_graft&&!state.flags.ember_resolved);
 state=perform(state,{type:'leave'});
 const available=getView(data,state).availableScenes.map(s=>s.id);
 assert.ok(!available.includes('a2_ember_cold')&&available.includes('a2_ember_return_repair'));
 state=route(['a2_ember_return_work','a2_ember_depart'],state);
 assert.ok(state.flags.winter_repaired&&state.flags.ember_resolved&&state.items.includes('thermal_seed'));
});

test('corridor rebuild hub cannot select both mutually exclusive standing policies',()=>{
 for(const policy of ['a4_partition_corridor','a4_partition_reserve']){
  const prefix=ENDING_ROUTES.partition.slice(0,ENDING_ROUTES.partition.indexOf('a4_partition_corridor'));
  let state=route([...prefix,policy]);
  state=perform(state,{type:'leave'});
  assert.ok(!getView(data,state).availableScenes.some(s=>s.id==='a4_partition_terms'));
  assert.equal(applyAction(data,state,{type:'enter',sceneId:'a4_partition_terms'}).ok,false);
  assert.ok(!(state.flags.partition_corridor&&state.flags.partition_local));
 }
});

test('authentic old saves inside previously converged scenes retain their public or coercive history',()=>{
 const fixtures=JSON.parse(fs.readFileSync(new URL('./fixtures/pre-revision-story-saves.json',import.meta.url)));
 const cases=[
  ['letters','a2_return_finish',/copies you published/,'letters_public'],
  ['shear','a2_dissent_complete',/Your order has reopened nothing/,'shear_pressured'],
  ['silence','a3_silence_finish',/boundary re-established after a breach/,'silence_forced'],
 ];
 for(const [name,choiceId,expected,flag]of cases){
  const loaded=loadSave(data,fixtures.saves[name].save);assert.ok(loaded.ok,loaded.error);
  assert.equal(loaded.state.scene,fixtures.saves[name].scene);
  assert.match(getView(data,loaded.state).scene.paragraphs.join(' '),expected);
  const state=perform(loaded.state,{type:'choose',choiceId});
  assert.ok(state.flags[flag]);assert.deepEqual(validateState(data,state).errors,[]);
 }
});
