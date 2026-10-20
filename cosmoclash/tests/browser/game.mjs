import assert from 'node:assert/strict';
import {chromium} from 'playwright';
import {createRustRelay} from '../../../tests/helpers/rust-relay.mjs';
import fs from 'node:fs/promises';
import path from 'node:path';
import {createHash} from 'node:crypto';
import {fileURLToPath} from 'node:url';
const root=path.resolve(path.dirname(fileURLToPath(import.meta.url)),'../..');
const output=path.join(root,'test-results/browser');await fs.mkdir(output,{recursive:true});
const executablePath=process.env.CHROME_BIN|| (process.platform==='darwin'?'/Applications/Google Chrome.app/Contents/MacOS/Google Chrome':undefined);
const expectedHash=createHash('sha256').update(await fs.readFile(path.join(root,'dist/cosmoclash.wasm'))).digest('hex');
const scenarios=[];
async function launch(){return chromium.launch({executablePath,headless:true,args:['--use-angle=swiftshader','--enable-unsafe-swiftshader','--disable-background-timer-throttling','--disable-backgrounding-occluded-windows','--disable-renderer-backgrounding']});}
async function poll(fn,timeout=20000){const end=Date.now()+timeout;let last;while(Date.now()<end){last=await fn();if(last)return last;await new Promise(r=>setTimeout(r,100));}throw new Error(`Timed out after ${timeout}ms`);}
async function read(page){return page.evaluate(()=>({state:window.__cosmo?.state,net:window.__cosmo?.netplay,snapshot:window.__cosmo?.snapshot,frames:window.__cosmo?.frames,fleet:window.__cosmo?.fleet,engine:window.__cosmo?.engine}));}
async function setKeys(page,keys,held){for(const key of held)if(!keys.has(key)){await page.keyboard.up(key);held.delete(key);}for(const key of keys)if(!held.has(key)){await page.keyboard.down(key);held.add(key);}}
function botKeys(snapshot,player){
 const self=snapshot.players[player],enemy=snapshot.players[1-player];
 const dx=enemy.position.x-self.position.x,dy=enemy.position.y-self.position.y;
 const desired=(Math.atan2(-dx,dy)*180/Math.PI+360)%360;
 const diff=((desired-self.angle+540)%360)-180;
 const keys=new Set(['/']);if(diff>4)keys.add('ArrowRight');else if(diff< -4)keys.add('ArrowLeft');
 if(Math.abs(diff)<45)keys.add('ArrowUp');
 if(self.shipId==='sholev'&&!self.frostShieldActive&&self.energy>.5)keys.add('.');
 return keys;
}
async function onlineScenario(name,latency,fullMatch){
 const relay=createRustRelay({distDir:path.join(root,'dist'),latencyPatternMs:latency});
 const address=await relay.listen(0);const url=`http://127.0.0.1:${address.port}/`;
 let browsers=[],pages=[];const errors=[],requests=[],socketPaths=[];const held=[new Set(),new Set()];
 const result={name,passed:false,latency,fullMatch,wasmSha256:expectedHash};
 try{
  browsers=await Promise.all([launch(),launch()]);pages=await Promise.all(browsers.map(b=>b.newPage({viewport:{width:960,height:720}})));
  for(const [index,page] of pages.entries()){
   page.on('pageerror',error=>errors.push({index,message:error.message}));
   page.on('response',response=>{if(response.status()>=400&&!response.url().endsWith('favicon.ico'))requests.push({url:response.url(),status:response.status()});});
   page.on('websocket',socket=>socketPaths.push(socket.url()));
  }
  await Promise.all(pages.map(p=>p.goto(url)));
  await Promise.all(pages.map(p=>p.waitForFunction(()=>window.__cosmo?.state==='menu',{},{timeout:60000})));
  for(const page of pages){assert.equal((await read(page)).engine,expectedHash);await page.getByRole('button',{name:'Online Melee',exact:true}).click();}
  await pages[0].getByRole('button',{name:'Create room',exact:true}).click();
  const invitation=await poll(()=>pages[0].locator('[data-invitation]').inputValue());
  const roomInput=pages[1].locator('[data-room]');await roomInput.click();await roomInput.press('ControlOrMeta+A');await roomInput.pressSequentially(invitation);assert.equal(await roomInput.inputValue(),invitation,'Room invitation must remain typeable with gameplay shortcuts installed');await pages[1].getByRole('button',{name:'Join room',exact:true}).click();
  await Promise.all(pages.map(p=>p.waitForFunction(()=>window.__cosmo?.netplay?.phase==='lobby',{},{timeout:15000})));
  // Gondi starts first for contact/weapon match. Full normal run retains all3 ships;
  // jitter run uses one ship for a focused second match + disconnect.
  for(const page of pages){for(const ship of [0,2])await page.locator(`[data-ship="${ship}"]`).uncheck();if(fullMatch){await page.locator('[data-ship="2"]').check();await page.locator('[data-ship="0"]').check();}}
  await Promise.all(pages.map(p=>p.getByRole('button',{name:'Ready',exact:true}).click()));
  await Promise.all(pages.map(p=>p.waitForFunction(()=>window.__cosmo?.netplay?.phase==='playing',{},{timeout:20000})));
  const initial=await Promise.all(pages.map(read));
  const initialPositions=initial.map(s=>s.snapshot.players.map(p=>({...p.position})));
  const started=Date.now();let progressAt=started;const seenShips=new Set();let states;let shots=0,motion=false,maxChecks=0,maxBattle=0;
  while(Date.now()-started<(fullMatch?(process.env.CI?480000:240000):30000)){
   states=await Promise.all(pages.map(read));
   for(const [index,s] of states.entries()){assert.ok(!['error','closed'].includes(s.net.phase),JSON.stringify(s.net));assert.ok(s.snapshot);shots=Math.max(shots,s.snapshot.bullets.length);maxChecks=Math.max(maxChecks,s.net.verifiedChecksums);maxBattle=Math.max(maxBattle,s.fleet.battle);s.snapshot.players.forEach(p=>seenShips.add(p.shipId));motion||=s.snapshot.players.some((p,i)=>Math.hypot(p.position.x-initialPositions[index][i].x,p.position.y-initialPositions[index][i].y)>10);}
   if(Date.now()-progressAt>10000){progressAt=Date.now();console.log(JSON.stringify({scenario:name,elapsedMs:progressAt-started,ticks:states.map(s=>s.net.tick),battles:states.map(s=>s.fleet.battle),checksums:states.map(s=>s.net.verifiedChecksums)}));}
   if(states.every(s=>s.net.phase==='ended'))break;
   await Promise.all(pages.map((p,i)=>setKeys(p,states[i].net.phase==='playing'?botKeys(states[i].snapshot,i):new Set(),held[i])));
   if(!fullMatch&&maxChecks>=8)break;
   await new Promise(r=>setTimeout(r,150));
  }
  states=await Promise.all(pages.map(read));assert.ok(motion,'Actual ships must move');assert.ok(shots>0,'Actual weapons must spawn');assert.ok(states.every(s=>s.net.verifiedChecksums>=5),'Both engines must verify state repeatedly');
  await Promise.all(pages.map((p,i)=>p.screenshot({path:path.join(output,`${name}-${i}.png`)})));
  Object.assign(result,{elapsedMs:Date.now()-started,seenShips:[...seenShips].sort(),frames:states.map(s=>s.net.tick),verifiedChecksums:states.map(s=>s.net.verifiedChecksums),maxShots:shots,motion,battleTransitions:maxBattle});
  if(fullMatch){
   assert.ok(states.every(s=>s.net.phase==='ended'),'Full fleet match must finish naturally');assert.ok(maxBattle>=2,'Losing fleet must advance through all3ships');assert.deepEqual([...seenShips].sort(),['gondi','michiGichi','sholev']);
   await Promise.all(pages.map((p,i)=>setKeys(p,new Set(),held[i])));
   await pages[0].getByRole('button',{name:'Rematch',exact:true}).click();await pages[1].getByRole('button',{name:/^(Accept rematch|Rematch)$/}).click();
   await Promise.all(pages.map(p=>p.waitForFunction(()=>window.__cosmo?.netplay?.phase==='lobby'&&window.__cosmo.netplay.round===1,{},{timeout:15000})));
   await Promise.all(pages.map(p=>p.getByRole('button',{name:'Ready',exact:true}).click()));
   await Promise.all(pages.map(p=>p.waitForFunction(()=>window.__cosmo?.netplay?.phase==='playing'&&window.__cosmo.netplay.round===1,{},{timeout:15000})));
   await poll(async()=>{const s=await Promise.all(pages.map(read));return s.every(s=>s.net.verifiedChecksums>=3);},20000);result.rematch=true;
  }
  await pages[1].getByRole('button',{name:'Leave match',exact:true}).click();
  await pages[0].waitForFunction(()=>['closed','error'].includes(window.__cosmo?.netplay?.phase),{},{timeout:10000});
  await pages[0].getByRole('button',{name:'Back to menu',exact:true}).click();
  await Promise.all(pages.map(p=>p.waitForFunction(()=>window.__cosmo?.state==='menu',{},{timeout:10000})));
  result.disconnect=true;assert.deepEqual(errors,[]);assert.deepEqual(requests,[]);assert.equal(socketPaths.length,2);assert.ok(socketPaths.every(p=>/^\/netplay\/cosmoclash\/[0-9a-f]{32}$/.test(new URL(p).pathname)));result.socketPaths=socketPaths;result.passed=true;
 }catch(error){
  result.error=String(error);result.errors=errors;result.requests=requests;
  result.state=await Promise.all(pages.map(p=>read(p).catch(()=>null)));
  await fs.writeFile(path.join(output,`${name}-failure.json`),JSON.stringify(result,null,2));
  throw error;
 }finally{
  scenarios.push(result);await fs.writeFile(path.join(output,'results.json'),JSON.stringify({passed:scenarios.every(s=>s.passed),scenarios},null,2));
  await Promise.all(browsers.map(b=>b.close()));await relay.close();
 }
 console.log(JSON.stringify(result));
}
await onlineScenario('normal',[0],!process.env.COSMO_QUICK);
if(!process.env.COSMO_QUICK)await onlineScenario('jitter',[0,30,5,70,15],false);
