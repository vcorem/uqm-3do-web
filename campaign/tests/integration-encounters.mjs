// Parent campaign -> real surface/native combat -> authenticated campaign receipts.
// Fixtures are produced exclusively through valid campaign actions, then imported
// with the visible save UI. No campaign state, physics, crew or winners are patched.
import assert from 'node:assert/strict';
import {readFile,writeFile,mkdir} from 'node:fs/promises';
import path from 'node:path';
import {chromium} from 'playwright';
import {createNetplayServer} from '../../server/netplay.mjs';
import {applyAction,getView,previewTravel,serializeSave,loadSave,resolveBattle,resolveSurvey} from '../src/state.mjs';
import {planItinerary,OPENING_CHOICES} from './routes.mjs';
import {surfaceObjective} from '../src/lander.mjs';
import {surfaceWaypoints} from './surface-driver.mjs';

const root=path.resolve(new URL('../..',import.meta.url).pathname);
const dist=path.resolve(process.env.AFTERLIGHT_DIST||path.join(root,'.tools/afterlight-dist'));
const output=path.resolve(process.env.AFTERLIGHT_INTEGRATION_RESULTS||path.join(root,'test-results/afterlight-integration'));
await mkdir(output,{recursive:true});
const data=JSON.parse(await readFile(path.join(dist,'content/campaign.json'),'utf8'));
const report={passed:false,errors:[],steps:[],surface:null,battle:null};let browser,page,server;
const held=new Set(),sleep=ms=>new Promise(resolve=>setTimeout(resolve,ms));
function mark(text){report.steps.push(text);console.log(text);}
function action(state,request){const result=applyAction(data,state,request);assert.equal(result.ok,true,result.error);return result.state;}
function makeFixture(state){const text=serializeSave(data,state);assert.equal(loadSave(data,text).ok,true);return text;}
async function snapshot(){return page.evaluate(()=>window.__afterlight.state);}
async function importFixture(state){
    const hasSave=await page.evaluate(()=>Boolean(localStorage.getItem('uqm.afterlight.save.v1')));
    await page.locator('#import-file').setInputFiles({name:'verified-expedition.json',mimeType:'application/json',buffer:Buffer.from(makeFixture(state))});
    if(hasSave){await page.locator('#confirm-dialog').waitFor({state:'visible'});await page.locator('#confirm-dialog button[value=confirm]').click();}
    await page.waitForFunction(revision=>window.__afterlight.state?.revision===revision,state.revision);
    assert.deepEqual(await snapshot(),state,'imported fixture must be exactly the legitimate action-generated state');
}
async function setKeys(keys){for(const key of [...held])if(!keys.includes(key)){await page.keyboard.up(key);held.delete(key);}for(const key of keys)if(!held.has(key)){await page.keyboard.down(key);held.add(key);}}
async function steer(target,radius=26){
    const deadline=Date.now()+30000;
    while(Date.now()<deadline){
        const surface=await page.evaluate(()=>window.__afterlight.surface);if(!surface){await setKeys([]);return;}
        if(Math.hypot(target.x-surface.x,target.y-surface.y)<radius){await setKeys([]);return;}
        const dx=target.x-surface.x-surface.vx*.23,dy=target.y-surface.y-surface.vy*.23,keys=[];
        if(dx>6)keys.push('ArrowRight');if(dx< -6)keys.push('ArrowLeft');if(dy>6)keys.push('ArrowDown');if(dy< -6)keys.push('ArrowUp');
        await setKeys(keys);await sleep(65);
    }
    throw new Error('Parent surface navigation timed out');
}
async function navigateSurface(target,radius=10){
    const state=await page.evaluate(()=>window.__afterlight.surface);assert.ok(state);
    const route=surfaceWaypoints(state,target);for(let i=0;i<route.length;i++)await steer(route[i],i===route.length-1?radius:12);
    await setKeys([]);
}
async function completeSurfaceTasks(){
    const deadline=Date.now()+180000;
    while(Date.now()<deadline){
        const state=await page.evaluate(()=>window.__afterlight.surface);
        if(!state||state.beacons.every(b=>b.visited))return;
        const objective=surfaceObjective(state),signature=`${objective.kind}:${objective.target.id??'pad'}`;
        await navigateSurface(objective.target);
        await page.waitForFunction(({kind,id})=>{
            const s=window.__afterlight.surface;if(!s)return true;
            if(kind==='deliver')return s.cargo===null;
            if(kind==='pickup')return s.cargo===id;
            return s.beacons[id].visited;
        },{kind:objective.kind,id:objective.target.id},{timeout:15000});
    }
    throw new Error('Parent surface objectives timed out');
}
async function resumeAfterReload(){await page.reload();await page.locator('[data-action=resume]').click();await page.waitForFunction(()=>window.__afterlight.state!==null);}

try{
    // The normal opening grants access to Act II; no synthetic flags or resources.
    const opening=planItinerary(data,OPENING_CHOICES).state;
    const surveySystem=data.systems.find(system=>system.survey?.mode==='survey' && opening.knownSystems.includes(system.id) && previewTravel(data,opening,system.id).allowed);
    assert.ok(surveySystem,'an actual accessible survey must exist');
    const surveyFixture=opening.location===surveySystem.id?opening:action(opening,{type:'travel',systemId:surveySystem.id});
    let battleFixture=opening;if(battleFixture.scene)battleFixture=action(battleFixture,{type:'leave'});
    if(battleFixture.location!=='ferry')battleFixture=action(battleFixture,{type:'travel',systemId:'ferry'});
    battleFixture=action(battleFixture,{type:'enter',sceneId:'a2_ferry'});
    await writeFile(path.join(output,'surface-fixture.json'),makeFixture(surveyFixture));await writeFile(path.join(output,'combat-fixture.json'),makeFixture(battleFixture));
    report.fixture={openingRevision:opening.revision,surveySystem:surveySystem.id,surveyRevision:surveyFixture.revision,battleRevision:battleFixture.revision};
    server=createNetplayServer({distDir:dist});const address=await server.listen(0,'127.0.0.1');
    browser=await chromium.launch({headless:true,...(process.env.CHROME_BIN?{executablePath:process.env.CHROME_BIN}:{}),args:['--disable-renderer-backgrounding','--disable-backgrounding-occluded-windows','--disable-background-timer-throttling']});
    report.browser=browser.version();page=await browser.newPage({viewport:{width:1280,height:1100}});
    page.on('pageerror',error=>report.errors.push(String(error)));
    await page.addInitScript(()=>{window.__observedBattleReceipts=[];window.addEventListener('message',event=>{if(event.origin===location.origin&&event.source===document.querySelector('#combat-frame iframe')?.contentWindow&&event.data?.type==='uqm-campaign:result')window.__observedBattleReceipts.push(structuredClone(event.data));});});
    await page.goto(`http://127.0.0.1:${address.port}/`);await page.locator('[data-action=begin]').waitFor();
    await importFixture(surveyFixture);await page.locator('[data-action=survey]').click();
    await page.waitForFunction(()=>window.__afterlight.surface!==null);assert.equal(await page.locator('#surface-layer').isVisible(),true);assert.equal(await page.locator('#app').evaluate(element=>element.inert),true);
    const pending=await snapshot();assert.ok(pending.pendingSurvey);assert.equal(pending.resources.credits,surveyFixture.resources.credits,'launch alone must not award survey resources');assert.equal(pending.surveyedSystems.includes(surveySystem.id),false);
    const terrain=await page.evaluate(()=>window.__afterlight.surface);assert.equal(terrain.seed,pending.pendingSurvey.seed);
    mark('Visible campaign survey launch creates only a pending receipt');
    await page.getByRole('button',{name:'Deploy lander',exact:true}).click();
    await completeSurfaceTasks();
    assert.ok((await snapshot()).pendingSurvey,'all three scans alone must not resolve campaign rewards');
    await page.screenshot({path:path.join(output,'parent-surface-survey.png')});
    const beforeReturn=await page.evaluate(()=>window.__afterlight.surface);await navigateSurface(terrain.pad,18);
    await page.waitForFunction(()=>window.__afterlight.state.pendingSurvey===null);const surveyed=await snapshot();
    assert.equal(surveyed.surveyedSystems.includes(surveySystem.id),true);assert.equal(surveyed.journal.filter(entry=>entry.type==='survey'&&entry.systemId===surveySystem.id).length,1);
    const surveyOracle=resolveSurvey(data,pending,{id:pending.pendingSurvey.id,outcome:'surveyed',beacons:3,hullRemaining:Math.ceil(beforeReturn.hull),salvage:surveyed.surveySalvage[surveySystem.id]});assert.equal(surveyOracle.ok,true,surveyOracle.error);assert.deepEqual(surveyed,surveyOracle.state,'actual surface return must apply exactly the authored reward/event rules');
    assert.equal(await page.locator('#surface-layer').isVisible(),false);assert.equal(await page.locator('#app').evaluate(element=>element.inert),false);
    assert.equal(await page.locator('[data-action=survey]').count(),0,'completed survey is no longer offered');
    report.surface={system:surveySystem.id,dayBefore:surveyFixture.day,dayAfter:surveyed.day,creditsBefore:surveyFixture.resources.credits,creditsAfter:surveyed.resources.credits,beacons:3,hullBeforeReturn:beforeReturn.hull};mark('Physical beacon visits and return award the authored survey exactly once');
    await resumeAfterReload();assert.deepEqual(await snapshot(),surveyed,'completed survey persists exactly across reload');

    // Import the legitimate pre-survey save again to exercise interrupted flights.
    await importFixture(surveyFixture);await page.locator('[data-action=survey]').click();await page.waitForFunction(()=>window.__afterlight.surface!==null);
    const interrupted=await snapshot();await page.getByRole('button',{name:'Deploy lander',exact:true}).click();await page.keyboard.down('ArrowUp');await sleep(250);await page.keyboard.up('ArrowUp');
    await resumeAfterReload();await page.waitForFunction(()=>window.__afterlight.surface!==null);
    assert.deepEqual((await snapshot()).pendingSurvey,interrupted.pendingSurvey,'pending survey identity and seed survive reload');
    const resumed=await page.evaluate(()=>window.__afterlight.surface);assert.equal(resumed.beacons.filter(b=>b.visited).length,0);assert.equal(resumed.salvage,0,'unfinished local collection is never rewarded across reload');
    await page.getByRole('button',{name:'Return to ship · Esc',exact:true}).click();await page.waitForFunction(()=>window.__afterlight.state.pendingSurvey===null);
    const retreated=await snapshot();assert.equal(retreated.surveyedSystems.includes(surveySystem.id),false);assert.equal(retreated.resources.credits,surveyFixture.resources.credits);assert.equal(retreated.day,surveyFixture.day+1);assert.equal(await page.locator('#app').evaluate(element=>element.inert),false);
    mark('Pending survey reload resumes safely; early withdrawal grants no survey reward');

    await importFixture(battleFixture);
    while(await page.locator('[data-action=continue]').count())await page.locator('[data-action=continue]').click();
    const battleChoice=getView(data,battleFixture).choices.find(choice=>choice.id==='a2_ferry_battle');assert.equal(battleChoice.enabled,true);
    await page.locator('[data-id=a2_ferry_battle]').click();await page.locator('#combat-layer').waitFor({state:'visible'});const pendingBattle=await snapshot();assert.ok(pendingBattle.pendingBattle);assert.equal(pendingBattle.battleOutcomes['lantern-drone'],undefined);
    await resumeAfterReload();await page.locator('#combat-layer').waitFor({state:'visible'});assert.deepEqual((await snapshot()).pendingBattle,pendingBattle.pendingBattle);
    const frame=page.frameLocator('#combat-frame iframe');await frame.getByRole('button',{name:'Enter combat',exact:true}).click({timeout:240000});
    await page.waitForFunction(()=>document.querySelector('#combat-frame iframe')?.contentWindow.__afterlightCombat?.phase===3,null,{timeout:90000});
    await frame.locator('#canvas').focus();await setKeys(['ArrowUp','ArrowLeft','Enter']);await sleep(4000);await page.screenshot({path:path.join(output,'parent-native-combat.png')});
    await page.waitForFunction(()=>window.__afterlight.state.pendingBattle===null,null,{timeout:180000});await setKeys([]);
    const resolved=await snapshot(),receipts=await page.evaluate(()=>window.__observedBattleReceipts);assert.equal(receipts.length,1);const receipt=receipts[0];
    assert.equal(receipt.id,pendingBattle.pendingBattle.battleId);assert.ok(receipt.frames>20);assert.ok(['victory','defeat'].includes(receipt.outcome));assert.equal(resolved.battleOutcomes['lantern-drone'],receipt.outcome);
    const battleOracle=resolveBattle(data,pendingBattle,receipt);assert.equal(battleOracle.ok,true,battleOracle.error);assert.deepEqual(resolved,battleOracle.state,'the native receipt must drive the exact campaign transition');
    assert.equal(resolved.scene,receipt.outcome==='victory'?'a2_ferry_rescue':'a2_ferry_setback');assert.equal(resolved.resources.crew,receipt.outcome==='victory'?receipt.crew:9);
    assert.equal(await page.locator('#combat-layer').isVisible(),false);assert.equal(await page.locator('#app').evaluate(element=>element.inert),false);
    report.battle={receipt,crewAfter:resolved.resources.crew,scene:resolved.scene,dayBefore:battleFixture.day,dayAfter:resolved.day};mark('Actual native battle result advances the campaign and preserves its crew/recovery rules');
    await resumeAfterReload();assert.deepEqual(await snapshot(),resolved,'native battle outcome persists exactly across reload');
    assert.deepEqual(report.errors,[]);report.passed=true;console.log(JSON.stringify(report));
}catch(error){report.failure=error.stack||String(error);if(page){await page.screenshot({path:path.join(output,'failure.png')}).catch(()=>{});report.finalState=await snapshot().catch(()=>null);report.surfaceAtFailure=await page.evaluate(()=>window.__afterlight?.surface).catch(()=>null);}throw error;}
finally{await setKeys([]).catch(()=>{});await writeFile(path.join(output,'results.json'),JSON.stringify(report,null,2)+'\n');await browser?.close();await server?.close();}
