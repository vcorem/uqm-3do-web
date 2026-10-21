// Genuine production UQM simulation and actual keyboard play. No health/win injection.
import assert from 'node:assert/strict';
import {mkdir,readFile,writeFile,readdir,copyFile,link,rm} from 'node:fs/promises';
import path from 'node:path';
import {createHash} from 'node:crypto';
import {chromium} from 'playwright';
import {createNetplayServer} from '../../server/netplay.mjs';
import {SHIPS} from '../src/combat.mjs';

const root=path.resolve(new URL('../..',import.meta.url).pathname);
const dist=path.resolve(process.env.AFTERLIGHT_COMBAT_DIST || path.join(root,'.tools/afterlight-combat'));
const output=path.resolve(process.env.AFTERLIGHT_COMBAT_RESULTS || path.join(root,'test-results/afterlight-combat'));
const site=path.join(output,'site');
await mkdir(path.join(site,'combat'),{recursive:true});
for(const name of await readdir(dist)) {
    const dest=path.join(site,'combat',name);await rm(dest,{force:true});
    try{await link(path.join(dist,name),dest);}catch{await copyFile(path.join(dist,name),dest);}
}
await copyFile(path.join(root,'campaign/src/combat.mjs'),path.join(site,'combat.js'));
await writeFile(path.join(site,'index.html'),`<!doctype html><html><head><style>body{margin:0;background:#050914;color:white}[hidden]{display:none!important}#combat{width:100vw;height:100vh}</style></head><body><div id="combat"></div><script type="module">
import{UQMCombat}from'./combat.js';window.events=[];window.results=[];window.combat=new UQMCombat({container:document.querySelector('#combat'),url:'./combat/index.html'});for(const type of ['ready','visible','started','result','error'])combat.addEventListener(type,e=>events.push({type,detail:e.detail}));combat.preload().catch(e=>events.push({type:'loadError',message:e.message}));window.run=request=>{window.pending=combat.start(request).then(result=>{results.push(result);return result;}).catch(error=>{events.push({type:'promiseError',message:error.message});return null;});};</script></body></html>`);
const report={passed:false,results:[],errors:[],console:[],steps:[],build:{}};
for(const name of ['uqm-debug.wasm','uqm-debug.js','host.js'])report.build[name]=createHash('sha256').update(await readFile(path.join(dist,name))).digest('hex');
let server,browser,page;
const held=new Set();
const sleep=ms=>new Promise(resolve=>setTimeout(resolve,ms));
function mark(step){report.steps.push(step);console.log(step);}
async function start(request,first=false){
    await page.evaluate(request=>window.run(request),request);
    if(first)await page.frameLocator('iframe').getByRole('button',{name:'Enter combat',exact:true}).click();
    await page.waitForFunction(id=>events.some(e=>(e.type==='started'&&e.detail.id===id)||e.type==='error'),request.id,{timeout:60000});
    assert.equal(await page.evaluate(()=>events.find(e=>e.type==='error')),undefined);
    mark('Native encounter started: '+request.id);
}
async function release(){for(const key of held){await page.keyboard.up(key);held.delete(key);}}
async function waitResult(id,timeout=180000){
    const deadline=Date.now()+timeout;let nextProgress=Date.now();
    while(Date.now()<deadline){
        const result=await page.evaluate(id=>results.find(result=>result.id===id),id);
        if(result){report.results.push(result);mark('Native result: '+JSON.stringify(result));return result;}
        const error=await page.evaluate(()=>events.find(e=>['error','promiseError','loadError'].includes(e.type)));
        assert.equal(error,undefined,JSON.stringify(error));
        if(Date.now()>nextProgress){const state=await page.frames()[1].evaluate(()=>window.__afterlightCombat);console.log(id+': '+JSON.stringify({frames:state.frames,crew:state.crew,phase:state.phase}));nextProgress=Date.now()+10000;}
        await sleep(250);
    }
    throw new Error('Native encounter timed out: '+id);
}
try{
    server=createNetplayServer({distDir:site});const address=await server.listen(0,'127.0.0.1');
    browser=await chromium.launch({headless:process.env.HEADED!=='1',...(process.env.CHROME_BIN?{executablePath:process.env.CHROME_BIN}:{}),args:['--disable-renderer-backgrounding','--disable-backgrounding-occluded-windows','--disable-background-timer-throttling']});
    report.browser=browser.version();
    page=await browser.newPage({viewport:{width:1200,height:1000}});
    page.on('pageerror',error=>report.errors.push(String(error)));
    page.on('console',message=>{if(message.type()==='error')report.console.push(message.text());});
    await page.goto(`http://127.0.0.1:${address.port}/`);
    await page.waitForFunction(()=>window.combat?.ready,null,{timeout:240000});
    assert.equal(await page.locator('iframe').isVisible(),false,'preloaded combat must remain hidden');
    const frame=page.frames()[1];
    assert.equal(await frame.evaluate(()=>typeof Module._uqm_campaign_test_ai),'undefined','production must exclude the test AI control export');
    mark('Production engine loaded behind the real parent/iframe adapter');
    await start({id:'victory',player:{ship:3,crew:42},enemies:[6],seed:12345,difficulty:'standard'},true);
    await page.frameLocator('iframe').locator('#canvas').focus();
    for(const key of ['ArrowUp','ArrowLeft','Enter']){await page.keyboard.down(key);held.add(key);}
    await sleep(4000);await page.screenshot({path:path.join(output,'combat-keyboard.png')});
    report.audio=await frame.evaluate(()=>{
        const ctx=typeof AL==='object'?AL.currentCtx:null;
        let peak=0,samples=0;
        const sources=Object.values(ctx?.sources||{}).filter(Boolean).sort((a,b)=>a.id-b.id);
        for(const source of sources.slice(0,5))for(const node of source.audioQueue||[]){
            const pcm=node.buffer?.getChannelData(0);if(pcm)for(const value of pcm){peak=Math.max(peak,Math.abs(value));samples++;}
        }
        return{context:ctx?.audioCtx.state,peak,samples,musicGain:sources[5]?.gain.gain.value};
    });
    assert.equal(report.audio.context,'running','The real user activation must unlock combat audio');
    assert.equal(report.audio.musicGain,0,'Only the new campaign score may play over combat');
    assert.ok(report.audio.peak>0&&report.audio.samples>0,'Native weapon sound PCM must be present');
    const won=await waitResult('victory');await release();
    assert.equal(won.outcome,'victory');assert.ok(won.frames>20);assert.ok(won.crew>0&&won.crew<=42);assert.equal(won.enemyRemaining,0);
    await start({id:'defeat',player:{ship:5,crew:1},enemies:[3],seed:4321,difficulty:'awesome'});
    await page.frameLocator('iframe').locator('#canvas').focus();
    await page.keyboard.down('Enter');held.add('Enter');
    const lost=await waitResult('defeat');await release();
    assert.equal(lost.outcome,'defeat');assert.equal(lost.crew,0);assert.equal(lost.enemyRemaining,1);assert.ok(lost.frames>10);
    await start({id:'retreat',player:{ship:5,crew:17},enemies:[15,24,19],seed:9876,difficulty:'standard'});
    await page.waitForFunction(()=>document.querySelector('iframe').contentWindow.__afterlightCombat.frames>=20,null,{timeout:30000});
    await page.frameLocator('iframe').getByRole('button',{name:'Retreat',exact:true}).click();
    const withdrew=await waitResult('retreat',30000);assert.equal(withdrew.outcome,'retreat');assert.ok(withdrew.crew>0&&withdrew.crew<=17);assert.ok(withdrew.enemyRemaining>0);
    await start({id:'reused',player:{ship:3,crew:42},enemies:[6,6],seed:5555,difficulty:'standard'});
    await page.frameLocator('iframe').locator('#canvas').focus();
    for(const key of ['ArrowUp','ArrowLeft','Enter']){await page.keyboard.down(key);held.add(key);}
    const fleet=await waitResult('reused');await release();assert.ok(['victory','defeat'].includes(fleet.outcome));assert.ok(fleet.enemyRemaining<2,'The genuine fleet must advance beyond its first opponent');
    if(process.env.AFTERLIGHT_SHIP_MATRIX==='1') {
        report.shipMatrix=[];
        for(const ship of SHIPS) {
            const id='hull-'+ship.id;
            await start({id,player:{ship:ship.id,crew:ship.maxCrew},enemies:[15],seed:22222+ship.id,difficulty:'standard'});
            await page.waitForFunction(()=>document.querySelector('iframe').contentWindow.__afterlightCombat.frames>=3,null,{timeout:30000});
            const observed=await frame.evaluate(()=>window.__afterlightCombat.crew);
            assert.equal(observed,ship.maxCrew,'Requested native starting crew for '+ship.name);
            await page.evaluate(()=>combat.retreat());const result=await waitResult(id,30000);
            assert.equal(result.outcome,'retreat');assert.equal(result.enemyRemaining,1);
            report.shipMatrix.push({id:ship.id,name:ship.name,startingCrew:observed,...result});
        }
        mark('All 25 native hulls accepted exact crew and clean retreat/reuse');
    }
    assert.deepEqual(report.errors,[]);assert.equal(await page.locator('iframe').isVisible(),false);
    report.events=await page.evaluate(()=>events);report.passed=true;mark('Victory, defeat, real retreat, two-ship fleet progression and repeated engine reuse passed');
}catch(error){report.failure=error.stack||String(error);if(page){await page.screenshot({path:path.join(output,'failure.png')}).catch(()=>{});report.events=await page.evaluate(()=>window.events).catch(()=>null);report.engine=await page.frames()[1]?.evaluate(()=>window.__afterlightCombat).catch(()=>null);}throw error;}
finally{await release().catch(()=>{});await writeFile(path.join(output,'results.json'),JSON.stringify(report,null,2)+'\n');await browser?.close();await server?.close();}
