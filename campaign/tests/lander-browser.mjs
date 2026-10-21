// Real browser input smoke: collects ore, visits all beacons and returns home.
import assert from 'node:assert/strict';
import {mkdir,writeFile,copyFile} from 'node:fs/promises';
import path from 'node:path';
import {chromium} from 'playwright';
import {createNetplayServer} from '../../server/netplay.mjs';

const root=path.resolve(new URL('../..',import.meta.url).pathname);
const output=path.resolve(process.env.AFTERLIGHT_LANDER_RESULTS||path.join(root,'test-results/afterlight-lander'));
const site=path.join(output,'site');await mkdir(site,{recursive:true});
await copyFile(path.join(root,'campaign/src/lander.mjs'),path.join(site,'lander.mjs'));
await writeFile(path.join(site,'index.html'),`<!doctype html><html><head><meta name="viewport" content="width=device-width, initial-scale=1"><style>body{margin:0;padding:10px;background:#030b13}#surface{max-width:1150px;margin:auto}</style></head><body><div id="surface"></div><script type="module">import{SurfaceExpedition}from'./lander.mjs';window.results=[];window.startSurface=(seed=77)=>{window.expedition?.destroy();window.expedition=new SurfaceExpedition({container:document.querySelector('#surface'),seed,id:'browser-'+seed,biome:'crystal',onComplete:receipt=>results.push(receipt)}).start();};startSurface();</script></body></html>`);
const report={passed:false,errors:[],results:[]};let browser,page,server;const held=new Set();
const sleep=ms=>new Promise(resolve=>setTimeout(resolve,ms));
async function setKeys(keys){for(const key of [...held])if(!keys.includes(key)){await page.keyboard.up(key);held.delete(key);}for(const key of keys)if(!held.has(key)){await page.keyboard.down(key);held.add(key);}}
async function steer(target,stopRadius=27){
    const deadline=Date.now()+30000;
    while(Date.now()<deadline){
        const s=await page.evaluate(()=>({x:expedition.state.x,y:expedition.state.y,vx:expedition.state.vx,vy:expedition.state.vy,outcome:expedition.state.outcome}));
        if(s.outcome||Math.hypot(target.x-s.x,target.y-s.y)<stopRadius){await setKeys([]);return;}
        const dx=target.x-s.x-s.vx*.23,dy=target.y-s.y-s.vy*.23;
        const keys=[];if(dx>6)keys.push('ArrowRight');if(dx< -6)keys.push('ArrowLeft');if(dy>6)keys.push('ArrowDown');if(dy< -6)keys.push('ArrowUp');
        await setKeys(keys);await sleep(65);
    }
    throw new Error('Keyboard navigation failed to reach target');
}
try{
    server=createNetplayServer({distDir:site});const address=await server.listen(0,'127.0.0.1');
    browser=await chromium.launch({headless:true,...(process.env.CHROME_BIN?{executablePath:process.env.CHROME_BIN}:{}),args:['--disable-renderer-backgrounding','--disable-backgrounding-occluded-windows','--disable-background-timer-throttling']});
    report.browser=browser.version();page=await browser.newPage({viewport:{width:1120,height:1000}});page.on('pageerror',error=>report.errors.push(String(error)));
    await page.goto(`http://127.0.0.1:${address.port}/`);await page.locator('canvas').waitFor();await page.screenshot({path:path.join(output,'surface-desktop.png')});
    const objectives=await page.evaluate(()=>({ore:expedition.state.ore[0],beacons:expedition.state.beacons,pad:expedition.state.pad}));
    await steer(objectives.ore,17);assert.equal(await page.evaluate(()=>expedition.state.ore[0].collected),true);
    for(const beacon of objectives.beacons){await steer(beacon);assert.equal(await page.evaluate(id=>expedition.state.beacons[id].visited,beacon.id),true);}
    assert.equal(await page.evaluate(()=>results.length),0,'all scans still require returning physically');
    await page.screenshot({path:path.join(output,'surface-recorded.png')});await steer(objectives.pad,20);
    await page.waitForFunction(()=>results.length===1);const success=await page.evaluate(()=>results[0]);assert.equal(success.outcome,'surveyed');assert.equal(success.beacons,3);assert.ok(success.salvage>=3);assert.ok(success.hullRemaining>0);report.results.push(success);
    await page.keyboard.press('Escape');await sleep(50);assert.equal(await page.evaluate(()=>results.length),1,'completed listeners must not duplicate rewards');
    await page.evaluate(()=>startSurface(88));await page.setViewportSize({width:390,height:844});await page.screenshot({path:path.join(output,'surface-mobile.png')});
    assert.equal(await page.evaluate(()=>document.documentElement.scrollWidth<=innerWidth),true,'mobile controls may not overflow');
    const before=await page.evaluate(()=>expedition.state.x);
    const right=page.getByRole('button',{name:'Steer right',exact:true});await right.scrollIntoViewIfNeeded();const box=await right.boundingBox();await page.mouse.move(box.x+box.width/2,box.y+box.height/2);await page.mouse.down();await sleep(600);await page.mouse.up();
    assert.ok(await page.evaluate(()=>expedition.state.x)>before+30,'pointer button physically moves the lander');
    await page.getByRole('button',{name:'Return to ship · Esc',exact:true}).click();await page.waitForFunction(()=>results.length===2);report.results.push(await page.evaluate(()=>results[1]));assert.equal(report.results[1].outcome,'retreated');
    await page.evaluate(()=>startSurface(99));await page.keyboard.press('Escape');await page.waitForFunction(()=>results.length===3);assert.equal(await page.evaluate(()=>results[2].outcome),'retreated');report.results.push(await page.evaluate(()=>results[2]));
    await page.evaluate(()=>expedition.destroy());assert.equal(await page.locator('canvas').count(),0);await page.keyboard.press('Escape');assert.equal(await page.evaluate(()=>results.length),3);
    assert.deepEqual(report.errors,[]);report.passed=true;console.log(JSON.stringify(report));
}catch(error){report.failure=error.stack||String(error);await page?.screenshot({path:path.join(output,'failure.png')}).catch(()=>{});throw error;}
finally{await setKeys([]).catch(()=>{});await writeFile(path.join(output,'results.json'),JSON.stringify(report,null,2)+'\n');await browser?.close();await server?.close();}
