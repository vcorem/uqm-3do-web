// Real keyboard/pointer input: all three objective modes, pause and recovery.
import assert from 'node:assert/strict';
import {mkdir,writeFile,copyFile} from 'node:fs/promises';
import path from 'node:path';
import {chromium} from 'playwright';
import {createNetplayServer} from '../../server/netplay.mjs';
import {surfaceWaypoints} from './surface-driver.mjs';

const root=path.resolve(new URL('../..',import.meta.url).pathname);
const output=path.resolve(process.env.AFTERLIGHT_LANDER_RESULTS||path.join(root,'test-results/afterlight-lander'));
const site=path.join(output,'site');await mkdir(site,{recursive:true});
await copyFile(path.join(root,'campaign/src/lander.mjs'),path.join(site,'lander.mjs'));
await writeFile(path.join(site,'index.html'),`<!doctype html><html><head><meta name="viewport" content="width=device-width, initial-scale=1"><style>body{margin:0;padding:10px;background:#030b13}#surface{max-width:1150px;margin:auto}</style></head><body><div id="surface"></div><script type="module">import{SurfaceExpedition,surfaceObjective}from'./lander.mjs';window.surfaceObjective=surfaceObjective;window.results=[];window.startSurface=(config={})=>{window.expedition?.destroy();window.expedition=new SurfaceExpedition({container:document.querySelector('#surface'),seed:77,id:'browser-'+results.length,biome:'crystal',...config,onComplete:receipt=>results.push(receipt)}).start();};startSurface();</script></body></html>`);
const report={passed:false,errors:[],results:[],modes:[]};let browser,page,server;const held=new Set();
const sleep=ms=>new Promise(resolve=>setTimeout(resolve,ms));
async function setKeys(keys){for(const key of [...held])if(!keys.includes(key)){await page.keyboard.up(key);held.delete(key);}for(const key of keys)if(!held.has(key)){await page.keyboard.down(key);held.add(key);}}
async function steer(target,stopRadius=13){
    const deadline=Date.now()+25000;
    while(Date.now()<deadline){
        const s=await page.evaluate(()=>({x:expedition.state.x,y:expedition.state.y,vx:expedition.state.vx,vy:expedition.state.vy,outcome:expedition.state.outcome,paused:expedition.paused}));
        assert.equal(s.paused,false,'test window must remain active');
        if(s.outcome||Math.hypot(target.x-s.x,target.y-s.y)<stopRadius){await setKeys([]);return;}
        const dx=target.x-s.x-s.vx*.23,dy=target.y-s.y-s.vy*.23;
        const keys=[];if(dx>3)keys.push('ArrowRight');if(dx< -3)keys.push('ArrowLeft');if(dy>3)keys.push('ArrowDown');if(dy< -3)keys.push('ArrowUp');
        await setKeys(keys);await sleep(45);
    }
    throw new Error('Keyboard navigation failed to reach target');
}
async function reach(target){const s=await page.evaluate(()=>expedition.state);for(const point of surfaceWaypoints(s,target))await steer(point);}
async function completeMode(mode){
    const labels=mode==='rescue'?['Beacon locker','Survival capsule','Long-range battery']:mode==='relay'?['Carrier dish','Clock gate','Return channel']:['Thermal fissure','Receiver ridge','Transit marker'];
    await page.evaluate(({mode,labels})=>startSurface({mode,objectiveLabels:labels,seed:mode==='survey'?77:mode==='rescue'?11:14,biome:mode==='survey'?'crystal':mode==='rescue'?'ice':'ocean'}),{mode,labels});
    assert.deepEqual(await page.locator('.surface-targets li').allTextContents(),labels);
    assert.ok((await page.locator('canvas').getAttribute('aria-label')).includes(labels[0]));
    if(mode==='rescue')assert.equal(await page.getByText(/stranded field teams/i).count(),0);
    const count=await page.evaluate(()=>results.length),initial=await page.evaluate(()=>expedition.state.ticks);
    await sleep(250);assert.equal(await page.evaluate(()=>expedition.state.ticks),initial,'briefing must not consume battery');
    await page.screenshot({path:path.join(output,`${mode}-briefing.png`)});
    await page.getByRole('button',{name:'Deploy lander',exact:true}).click();
    const started=Date.now();
    if(mode==='survey'){
        const ore=await page.evaluate(()=>expedition.state.ore.find(o=>expedition.state.hazards.every(h=>Math.hypot(o.x-h.baseX,o.y-h.baseY)>h.radius+h.amplitude+32)));
        assert.ok(ore,'at least one optional deposit has a clear route');await reach(ore);assert.equal(await page.evaluate(id=>expedition.state.ore[id].collected,ore.id),true);
        await page.getByRole('button',{name:'Pause expedition',exact:true}).click();const ticks=await page.evaluate(()=>expedition.state.ticks);await sleep(300);assert.equal(await page.evaluate(()=>expedition.state.ticks),ticks,'manual pause freezes battery/physics');
        await page.getByRole('button',{name:'Resume expedition',exact:true}).click();
        await page.evaluate(()=>window.dispatchEvent(new Event('blur')));assert.equal(await page.evaluate(()=>expedition.paused),true);
        const blurred=await page.evaluate(()=>expedition.state.ticks);await sleep(300);assert.equal(await page.evaluate(()=>expedition.state.ticks),blurred,'blur remains paused until explicit resume');
        await page.getByRole('button',{name:'Resume expedition',exact:true}).click();
    }
    let legs=0;
    while(await page.evaluate(()=>!expedition.state.outcome)){
        assert.ok(++legs<12,'bounded number of genuine objective legs');
        const goal=await page.evaluate(()=>({...surfaceObjective(expedition.state),cargo:expedition.state.cargo}));
        await reach(goal.target);
        await page.waitForFunction(goal=>{
            const s=expedition.state;if(s.outcome)return true;
            return goal.kind==='return'?false:goal.kind==='deliver'?s.beacons[goal.cargo].visited:goal.kind==='pickup'?s.cargo===goal.target.id:s.beacons[goal.target.id].visited;
        },goal,{timeout:13000});
    }
    await page.waitForFunction(count=>results.length===count+1,count);
    const receipt=await page.evaluate(()=>results.at(-1));assert.equal(receipt.outcome,'surveyed');assert.equal(receipt.beacons,3);assert.ok(receipt.hullRemaining>0);
    if(mode==='rescue')assert.deepEqual(receipt.completedTasks,[0,1,2]);
    report.results.push(receipt);report.modes.push({mode,legs,wallSeconds:Math.round((Date.now()-started)/1000),simulationSeconds:await page.evaluate(()=>expedition.state.ticks/60)});
    await page.screenshot({path:path.join(output,`${mode}-complete.png`)});await page.keyboard.press('Escape');await sleep(30);assert.equal(await page.evaluate(()=>results.length),count+1,'completion cannot fire twice');
    console.log(`Passed ${mode}: ${legs} physical objective legs`);
}
try{
    server=createNetplayServer({distDir:site});const address=await server.listen(0,'127.0.0.1');
    browser=await chromium.launch({headless:true,...(process.env.CHROME_BIN?{executablePath:process.env.CHROME_BIN}:{}),args:['--disable-renderer-backgrounding','--disable-backgrounding-occluded-windows','--disable-background-timer-throttling']});
    report.browser=browser.version();page=await browser.newPage({viewport:{width:1120,height:1050}});page.on('pageerror',error=>report.errors.push(String(error)));
    await page.goto(`http://127.0.0.1:${address.port}/`);await page.locator('canvas').waitFor();
    for(const mode of ['survey','rescue','relay'])await completeMode(mode);
    await page.evaluate(()=>startSurface({seed:88,mode:'rescue',completedTasks:[1]}));await page.setViewportSize({width:390,height:844});await page.screenshot({path:path.join(output,'surface-mobile.png')});
    assert.equal(await page.evaluate(()=>document.documentElement.scrollWidth<=innerWidth),true,'mobile controls may not overflow');
    await page.getByRole('button',{name:'Deploy lander',exact:true}).click();const before=await page.evaluate(()=>expedition.state.x);
    const right=page.getByRole('button',{name:'Steer right',exact:true});await right.scrollIntoViewIfNeeded();const box=await right.boundingBox();await page.mouse.move(box.x+box.width/2,box.y+box.height/2);await page.mouse.down();await sleep(600);await page.mouse.up();
    assert.ok(await page.evaluate(()=>expedition.state.x)>before+30,'pointer button physically moves the lander');
    await page.getByRole('button',{name:'Return to ship · Esc',exact:true}).click();await page.waitForFunction(()=>results.length===4);const retreat=await page.evaluate(()=>results[3]);assert.equal(retreat.outcome,'retreated');assert.deepEqual(retreat.completedTasks,[1]);report.results.push(retreat);
    await page.evaluate(()=>startSurface({seed:99}));await page.keyboard.press('Escape');await page.waitForFunction(()=>results.length===5);assert.equal(await page.evaluate(()=>results[4].outcome),'retreated');report.results.push(await page.evaluate(()=>results[4]));
    await page.evaluate(()=>expedition.destroy());assert.equal(await page.locator('canvas').count(),0);await page.keyboard.press('Escape');assert.equal(await page.evaluate(()=>results.length),5);
    assert.deepEqual(report.errors,[]);report.passed=true;console.log(JSON.stringify(report));
}catch(error){report.failure=error.stack||String(error);await page?.screenshot({path:path.join(output,'failure.png')}).catch(()=>{});throw error;}
finally{await setKeys([]).catch(()=>{});await writeFile(path.join(output,'results.json'),JSON.stringify(report,null,2)+'\n');await browser?.close();await server?.close();}
