import test from 'node:test';
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {createGame,applyAction,getView,validateState} from '../src/state.mjs';
import {ENDING_ROUTES,GRAND_TOUR_CHOICES,AUTHORITY_TOUR_CHOICES,planItinerary} from './routes.mjs';

const data=JSON.parse(readFileSync(new URL('../content/campaign.json',import.meta.url),'utf8'));

test('every enabled response, local contact and purchase remains executable along contrasting complete voyages',()=>{
 let audited=0;
 for(const [route,choices] of Object.entries({...ENDING_ROUTES,grand:GRAND_TOUR_CHOICES,authority:AUTHORITY_TOUR_CHOICES})){
  const plan=planItinerary(data,choices);let state=createGame(data);
  for(const step of [null,...plan.actions]){
   if(step){const result=applyAction(data,state,step);assert.equal(result.ok,true,result.error);state=result.state;}
   if(state.ending||state.pendingBattle||state.pendingSurvey)continue;
   const view=getView(data,state),offered=[
    ...view.choices.filter(c=>c.enabled).map(c=>({type:'choose',choiceId:c.id})),
    ...view.availableScenes.map(s=>({type:'enter',sceneId:s.id})),
    ...view.upgrades.filter(o=>o.enabled).map(o=>({type:'purchaseUpgrade',upgradeId:o.id})),
    ...view.fleet.filter(o=>o.enabled).map(o=>o.owned?{type:'switchShip',ship:o.ship}:{type:'purchaseShip',offerId:o.id}),
   ];
   for(const action of offered){
    // Opening a different contact first closes the current transmission, as the UI does.
    const from=action.type==='enter'&&state.scene?applyAction(data,state,{type:'leave'}).state:state;
    const result=applyAction(data,from,action);
    assert.equal(result.ok,true,`${route} day ${state.day} ${state.scene}: enabled ${JSON.stringify(action)} failed: ${result.error}`);
    assert.deepEqual(validateState(data,result.state).errors,[]);audited++;
   }
  }
 }
 assert.ok(audited>1000,'audit includes alternate decisions and local opportunities, not only the chosen path');
});
