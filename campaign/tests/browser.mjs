/** Real DOM playthroughs; the state engine computes expectations, never browser progress. */
import assert from 'node:assert/strict';
import {readFile,mkdir,writeFile} from 'node:fs/promises';
import path from 'node:path';
import {chromium,firefox,webkit} from 'playwright';
import {createNetplayServer} from '../../server/netplay.mjs';
import {createGame,serializeSave,applyAction} from '../src/state.mjs';
import * as routes from './routes.mjs';
const root=path.resolve(new URL('../..',import.meta.url).pathname);
const data=JSON.parse(await readFile(path.join(root,'campaign/content/campaign.json'),'utf8'));
const browserName=process.env.AFTERLIGHT_BROWSER||'chromium';
if(!['chromium','firefox','webkit'].includes(browserName))throw Error('Unsupported browser '+browserName);
const output=path.join(root,'test-results/afterlight-browser'+(browserName==='chromium'?'':'-'+browserName));await mkdir(output,{recursive:true});
const report={passed:false,errors:[],routes:[],screenshots:[]};
let browser,server;
const saveKey='uqm.afterlight.save.v1';
const screenshot=async(page,name)=>{await page.screenshot({path:path.join(output,name+'.png'),fullPage:true});report.screenshots.push(name+'.png');};
async function click(page,selector){await page.locator(selector).first().click();}
async function confirm(page){const dialog=page.locator('#confirm-dialog[open]');if(await dialog.count())await dialog.getByRole('button',{name:'Confirm',exact:true}).click();}
async function replay(page,actions){
  for(const action of actions){
    const revision=await page.evaluate(()=>__afterlight.state.revision);
    if(action.type==='leave')await click(page,'[data-action="leave"]');
    else if(action.type==='travel'){await page.locator('#destination').selectOption(action.systemId);await click(page,`[data-action="travel"][data-id="${action.systemId}"]`);}
    else if(action.type==='enter'){let target=page.locator(`[data-action="enter"][data-id="${action.sceneId}"]`);if(!await target.count()){await click(page,'[data-action="tab"][data-page="contacts"]');target=page.locator(`[data-action="enter"][data-id="${action.sceneId}"]`);}await target.first().click();}
    else if(action.type==='choose'){
      while(await page.locator('[data-action="continue"]').count())await click(page,'[data-action="continue"]');
      await click(page,`[data-action="choose"][data-id="${action.choiceId}"]`);await confirm(page);
    }else if(action.type==='distress'){
      await click(page,'[data-action="tab"][data-page="ship"]');await click(page,'[data-action="distress"]');await confirm(page);await click(page,'[data-action="tab"][data-page="chart"]');
    }else throw Error(`No visible driver for ${action.type}`);
    await page.waitForFunction(previous=>__afterlight.state.revision>previous,revision);
  }
}
try{
  server=createNetplayServer({distDir:path.join(root,'.tools/afterlight-dist')});const address=await server.listen(0,'127.0.0.1');const url=`http://127.0.0.1:${address.port}/`;
  browser=await ({chromium,firefox,webkit}[browserName]).launch({headless:true,...(browserName==='chromium'&&process.env.CHROME_BIN?{executablePath:process.env.CHROME_BIN}:{}),...(browserName==='chromium'?{args:['--disable-background-timer-throttling']}:{})});report.browser={name:browserName,version:browser.version()};
  const page=await browser.newPage({viewport:{width:1440,height:1000}});
  page.on('pageerror',e=>report.errors.push(e.message));page.on('response',r=>{if(r.status()>=400&&!r.url().endsWith('/favicon.ico'))report.errors.push(`${r.status()} ${r.url()}`);});
  await page.addInitScript(()=>{const Native=window.AudioContext;window.AudioContext=class extends Native{constructor(...args){super(...args);window.__audioTestContext=this;window.__audioTestGains=[];}createGain(){const gain=super.createGain();window.__audioTestGains.push(gain);return gain;}};});
  await page.goto(url);await page.getByRole('button',{name:'Begin expedition',exact:true}).waitFor();await screenshot(page,'title');
  await click(page,'[data-action="begin"]');await page.locator('[data-scene="a1_wake"]').waitFor();await screenshot(page,'opening');
  const opening=routes.planItinerary(data,routes.OPENING_CHOICES);await replay(page,opening.actions);
  assert.deepEqual(await page.evaluate(()=>__afterlight.state),opening.state,'opening agrees with ordinary state transitions');
  await screenshot(page,'act-two-chart');
  await page.waitForFunction(()=>__afterlight.music.context==='running');await page.waitForTimeout(1800);
  const sample=()=>{const a=__audioTestContext.createAnalyser();__audioTestGains[0].connect(a);return new Promise(resolve=>setTimeout(()=>{const f=new Float32Array(a.fftSize);a.getFloatTimeDomainData(f);a.disconnect();__audioTestGains[0].disconnect(a);resolve(Math.max(...f.map(Math.abs)));},150));};
  assert.ok(await page.evaluate(sample)>.0001,'original score generates audible PCM');await click(page,'[data-action="music"]');await page.waitForTimeout(1800);assert.ok(await page.evaluate(sample)<.0001,'music toggle silences the score');await click(page,'[data-action="music"]');
  const assetResults=await page.evaluate(async content=>{
    const ctx=new AudioContext();const portraits=[];for(const c of content.characters){const image=new Image();image.src=c.portrait;await image.decode();portraits.push(image.naturalWidth);}
    const manifest=await(await fetch('./assets/music/manifest.json')).json();const cues=Array.isArray(manifest)?manifest:manifest.tracks;
    const decoded=[];for(const cue of cues){const id=cue.id;const r=await fetch(`./assets/music/${id}.ogg`);if(!r.ok)throw Error(id);const b=await ctx.decodeAudioData(await r.arrayBuffer());decoded.push({id,duration:b.duration,channels:b.numberOfChannels});}await ctx.close();return{portraits,decoded};
  },data);assert.equal(assetResults.portraits.length,10);assert.equal(assetResults.decoded.length,15);assert.ok(assetResults.decoded.every(t=>t.duration>50&&t.channels===2));report.assets=assetResults;
  const beforeReload=await page.evaluate(()=>__afterlight.state);await page.reload();await click(page,'[data-action="resume"]');assert.deepEqual(await page.evaluate(()=>__afterlight.state),beforeReload);
  await click(page,'[data-action="menu"]');await screenshot(page,'expedition-menu');
  const downloadPromise=page.waitForEvent('download');await click(page,'[data-action="export"]');const download=await downloadPromise;const exported=path.join(output,'exported-save.json');await download.saveAs(exported);assert.deepEqual(JSON.parse(await readFile(exported,'utf8')).state??JSON.parse(await readFile(exported,'utf8')),JSON.parse(serializeSave(data,beforeReload)).state??JSON.parse(serializeSave(data,beforeReload)));
  await page.locator('#import-file').setInputFiles({name:'broken.json',mimeType:'application/json',buffer:Buffer.from('{broken')});assert.deepEqual(await page.evaluate(()=>__afterlight.state),beforeReload,'bad import preserves expedition');
  const previous=applyAction(data,beforeReload,{type:'leave'});const validBackup=serializeSave(data,previous.ok?previous.state:beforeReload);
  await page.evaluate(({key,backup})=>{localStorage.setItem(key,'{broken');localStorage.setItem(key+'.previous',backup);},{key:saveKey,backup:validBackup});await page.reload();await click(page,'[data-action="resume"]');assert.deepEqual(await page.evaluate(()=>__afterlight.state),JSON.parse(validBackup).state??JSON.parse(validBackup));
  report.saveRecovery=true;
  if(!routes.ENDING_ROUTES)throw Error('All three final-route itineraries are required.');
  for(const [id,choices]of Object.entries(routes.ENDING_ROUTES)){
    const context=await browser.newContext({viewport:{width:1440,height:1000}});const routePage=await context.newPage();routePage.on('pageerror',e=>report.errors.push(e.message));await routePage.goto(url);await click(routePage,'[data-action="begin"]');
    const plan=routes.planItinerary(data,choices);await replay(routePage,plan.actions);const actual=await routePage.evaluate(()=>__afterlight.state);assert.equal(actual.ending,id);assert.equal(JSON.stringify(actual),JSON.stringify(plan.state),`Full state mismatch on ${id}`);await routePage.locator('.ending').waitFor();await screenshot(routePage,`ending-${id}`);report.routes.push({ending:id,actions:plan.actions.length,day:actual.day,systems:actual.visitedSystems.length,decisions:actual.usedChoices.length});await context.close();
  }
  const mobile=await browser.newPage({viewport:{width:390,height:844}});mobile.on('pageerror',e=>report.errors.push(e.message));await mobile.goto(url);await click(mobile,'[data-action="begin"]');await screenshot(mobile,'mobile-dialogue');assert.ok(await mobile.evaluate(()=>document.documentElement.scrollWidth<=innerWidth));await click(mobile,'[data-action="leave"]');await screenshot(mobile,'mobile-chart');assert.ok(await mobile.evaluate(()=>document.documentElement.scrollWidth<=innerWidth));
  await click(mobile,'[data-action="menu"]');await click(mobile,'[data-action="text-size"]');assert.equal(await mobile.evaluate(()=>getComputedStyle(document.documentElement).fontSize),'18px');assert.ok(await mobile.evaluate(()=>document.documentElement.scrollWidth<=innerWidth));report.mobile=true;
  assert.deepEqual(report.errors,[]);report.passed=true;console.log(JSON.stringify(report,null,2));
}catch(error){report.failure=error.stack||String(error);throw error;}
finally{await writeFile(path.join(output,'results.json'),JSON.stringify(report,null,2)+'\n');await browser?.close();await server?.close();}
