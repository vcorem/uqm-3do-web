// Real interface checks for acquired hulls, equipment and persistent rescue work.
// Imported starting saves come only from ordinary campaign actions; no flags,
// resources, physics or native results are injected into the browser.
import assert from 'node:assert/strict';
import {readFile,mkdir,writeFile} from 'node:fs/promises';
import {fileURLToPath} from 'node:url';
import {chromium,firefox,webkit} from 'playwright';
import {createNetplayServer} from '../../server/netplay.mjs';
import {applyAction,getView,serializeSave,resolveBattle} from '../src/state.mjs';
import {planItinerary,OPENING_CHOICES,LEAD_CHOICES} from './routes.mjs';
import {surfaceWaypoints} from './surface-driver.mjs';

const root=fileURLToPath(new URL('../../',import.meta.url));
const data=JSON.parse(await readFile(new URL('../content/campaign.json',import.meta.url),'utf8'));
const browserName=process.env.AFTERLIGHT_BROWSER||'chromium';
if(!['chromium','firefox','webkit'].includes(browserName))throw Error('Unsupported browser '+browserName);
const out=new URL('../../test-results/afterlight-progression'+(browserName==='chromium'?'':'-'+browserName)+'/',import.meta.url);await mkdir(out,{recursive:true});
const report={passed:false,errors:[],checks:[]},held=new Set();let browser,page,server;
const sleep=ms=>new Promise(r=>setTimeout(r,ms));
const state=()=>page.evaluate(()=>__afterlight.state);
const act=(s,a)=>{const r=applyAction(data,s,a);assert.equal(r.ok,true,r.error);return r.state;};
async function importSave(s){
 const replace=await page.evaluate(()=>Boolean(__afterlight.state||localStorage.getItem('uqm.afterlight.save.v1')));
 await page.locator('#import-file').setInputFiles({name:'earned-expedition.json',mimeType:'application/json',buffer:Buffer.from(serializeSave(data,s))});
 if(replace)await page.locator('#confirm-dialog button[value=confirm]').click();
 await page.waitForFunction(rev=>__afterlight.state?.revision===rev,s.revision);assert.deepEqual(await state(),s);
}
async function uiAction(selector,request){const expected=act(await state(),request);await page.locator(selector).first().click();await page.waitForFunction(rev=>__afterlight.state.revision===rev,expected.revision);assert.deepEqual(await state(),expected);return expected;}
async function keys(next){for(const k of [...held])if(!next.includes(k)){await page.keyboard.up(k);held.delete(k);}for(const k of next)if(!held.has(k)){await page.keyboard.down(k);held.add(k);}}
async function navigate(target){
 const initial=await page.evaluate(()=>__afterlight.surface);
 for(const point of surfaceWaypoints(initial,target)){
  let reached=false;const end=Date.now()+25000;
  while(Date.now()<end){const s=await page.evaluate(()=>__afterlight.surface);assert.ok(s,'surface stays active');
   if(Math.hypot(point.x-s.x,point.y-s.y)<11){await keys([]);reached=true;break;}
   const dx=point.x-s.x-s.vx*.23,dy=point.y-s.y-s.vy*.23,k=[];
   if(dx>3)k.push('ArrowRight');if(dx< -3)k.push('ArrowLeft');if(dy>3)k.push('ArrowDown');if(dy< -3)k.push('ArrowUp');await keys(k);await sleep(45);
  }assert.ok(reached,'physical navigation reaches target');
 }
}
try{
 server=createNetplayServer({distDir:root+'.tools/afterlight-dist'});const address=await server.listen(0,'127.0.0.1');
 browser=await ({chromium,firefox,webkit}[browserName]).launch({headless:true,...(browserName==='chromium'&&process.env.CHROME_BIN?{executablePath:process.env.CHROME_BIN}:{}),...(browserName==='chromium'?{args:['--disable-background-timer-throttling']}:{})});report.browser={name:browserName,version:browser.version()};
 page=await browser.newPage({viewport:{width:1360,height:1000}});page.on('pageerror',e=>report.errors.push(e.message));
 await page.addInitScript(()=>{window.observedReceipts=[];window.nativeRequests=[];window.addEventListener('message',e=>{if(e.origin===location.origin&&e.source===window.parent&&e.data?.type==='uqm-campaign:start')nativeRequests.push(structuredClone(e.data.request));if(e.origin===location.origin&&e.source===document.querySelector('#combat-frame iframe')?.contentWindow&&e.data?.type==='uqm-campaign:result')observedReceipts.push(structuredClone(e.data));});});
 await page.goto(`http://127.0.0.1:${address.port}/`);await page.locator('[data-action=begin]').waitFor();
 const earned=planItinerary(data,[...OPENING_CHOICES,...LEAD_CHOICES.roots,'a2_growers_accept_blade']).state;
 assert.ok(earned.flags.ship_16);assert.ok(earned.day<45);await importSave(earned);
 await page.locator('nav [data-page=ship]').click();
 await uiAction('[data-action=switchShip][data-ship="16"]',{type:'switchShip',ship:16});
 let current=await state();assert.equal(current.resources.crew,12);assert.equal(current.reserveCrew,6);
 await page.reload();await page.locator('[data-action=resume]').click();assert.deepEqual(await state(),current);await page.locator('nav [data-page=ship]').click();
 await uiAction('[data-action=switchShip][data-ship="5"]',{type:'switchShip',ship:5});assert.equal((await state()).resources.crew,18);
 await page.locator('nav [data-page=chart]').click();await page.locator('#destination').selectOption('garden');await uiAction('[data-action=travel]',{type:'travel',systemId:'garden'});await page.locator('nav [data-page=ship]').click();
 await uiAction('[data-action=purchaseUpgrade][data-id=surface_cells]',{type:'purchaseUpgrade',upgradeId:'surface_cells'});
 assert.equal(getView(data,await state()).capabilities.surveyBattery,40);
 report.checks.push('An earned Supox hull transfers twelve crew and preserves six reserves through reload; switching back neither heals nor loses crew.');
 await page.locator('nav [data-page=chart]').click();await page.locator('#destination').selectOption('needle');
 await uiAction('[data-action=travel]',{type:'travel',systemId:'needle'});await page.locator('nav [data-page=ship]').click();
 const before=await state();await uiAction('[data-action=purchaseUpgrade][data-id=range_tanks]',{type:'purchaseUpgrade',upgradeId:'range_tanks'});
 await uiAction('[data-action=purchaseShip][data-id=market_eluder]',{type:'purchaseShip',offerId:'market_eluder'});
 current=await state();assert.equal(current.resources.fuel,before.resources.fuel);assert.equal(getView(data,current).capabilities.fuelCapacity,130);assert.equal(current.ship,5);
 await uiAction('[data-action=switchShip][data-ship="15"]',{type:'switchShip',ship:15});assert.equal((await state()).resources.crew,18);
 await page.screenshot({path:fileURLToPath(new URL('outfitting-desktop.png',out)),fullPage:true});await page.setViewportSize({width:390,height:844});
 assert.ok(await page.evaluate(()=>document.documentElement.scrollWidth<=innerWidth));await page.screenshot({path:fileURLToPath(new URL('outfitting-mobile.png',out)),fullPage:true});await page.setViewportSize({width:1360,height:1000});
 report.checks.push('Visible equipment and hull purchases charge their actual prices, increase capacity without free fuel or crew, and fit a narrow screen.');

 // The campaign must launch the acquired hull, not the starter, in real UQM.
 await page.locator('nav [data-page=chart]').click();await page.locator('#destination').selectOption('ferry');await uiAction('[data-action=travel]',{type:'travel',systemId:'ferry'});
 await uiAction('[data-action=enter][data-id=a2_ferry]',{type:'enter',sceneId:'a2_ferry'});await uiAction('[data-action=choose][data-id=a2_ferry_battle]',{type:'choose',choiceId:'a2_ferry_battle'});
 const pending=await state();assert.equal(pending.ship,15);
 const frame=page.frameLocator('#combat-frame iframe');await frame.getByRole('button',{name:'Enter combat',exact:true}).click({timeout:240000});
 await page.waitForFunction(()=>document.querySelector('#combat-frame iframe')?.contentWindow.__afterlightCombat?.phase===3,null,{timeout:90000});
 await page.waitForFunction(()=>document.querySelector('#combat-frame iframe').contentWindow.__afterlightCombat.frames>=20,null,{timeout:30000});
 const native=await page.evaluate(()=>document.querySelector('#combat-frame iframe').contentWindow.__afterlightCombat);
 const request=await page.locator('#combat-frame iframe').evaluate(el=>el.contentWindow.nativeRequests.at(-1));assert.equal(request.player.ship,15);assert.equal(request.player.crew,18);assert.ok(native.crew>0&&native.crew<=18);
 await page.locator('#retreat').click();await page.waitForFunction(()=>__afterlight.state.pendingBattle===null,null,{timeout:30000});
 const receipt=await page.evaluate(()=>observedReceipts.at(-1));assert.equal(receipt.outcome,'retreat');assert.ok(receipt.frames>=20);assert.deepEqual(await state(),resolveBattle(data,pending,receipt).state);
 report.native={ship:15,receipt};report.checks.push('The purchased Spathi hull actually launches in native UQM with the surviving crew; a real withdrawal returns through the campaign rules.');

 // A real delivery survives recall and reload. Undelivered objectives remain.
 let rescue=act(earned,{type:'travel',systemId:'garden'});rescue=act(rescue,{type:'purchaseUpgrade',upgradeId:'surface_cells'});await importSave(rescue);
 await page.locator('[data-action=survey]').click();await page.waitForFunction(()=>__afterlight.surface!==null);
 const terrain=await page.evaluate(()=>__afterlight.surface);assert.equal(terrain.mode,'rescue');assert.ok(terrain.battery>=210,'equipment reaches the actual surface simulation');
 await page.getByRole('button',{name:'Deploy lander',exact:true}).click();const target=terrain.beacons[0];await navigate(target);await page.waitForFunction(()=>__afterlight.surface?.cargo===0);
 await navigate(terrain.pad);await page.waitForFunction(()=>__afterlight.surface?.beacons[0].visited);
 await page.getByRole('button',{name:'Return to ship · Esc',exact:true}).click();await page.waitForFunction(()=>__afterlight.state.pendingSurvey===null);
 const partial=await state();assert.deepEqual(partial.surveyProgress.garden,[0]);assert.equal(partial.surveyedSystems.includes('garden'),false);
 await page.reload();await page.locator('[data-action=resume]').click();assert.deepEqual(await state(),partial);await page.locator('[data-action=survey]').click();await page.waitForFunction(()=>__afterlight.surface!==null);
 assert.deepEqual(await page.evaluate(()=>__afterlight.surface.beacons.filter(b=>b.visited).map(b=>b.id)),[0]);
 await page.getByRole('button',{name:'Return to ship · Esc',exact:true}).click();report.checks.push('A keyboard-controlled rescue delivery survives recall, save/load and a new launch; the unfinished mission remains available.');
 assert.deepEqual(report.errors,[]);report.passed=true;console.log(JSON.stringify(report,null,2));
}catch(e){report.failure=e.stack;await page?.screenshot({path:fileURLToPath(new URL('failure.png',out)),fullPage:true}).catch(()=>{});throw e;}
finally{await keys([]).catch(()=>{});await writeFile(new URL('results.json',out),JSON.stringify(report,null,2)+'\n');await browser?.close();await server?.close();}
