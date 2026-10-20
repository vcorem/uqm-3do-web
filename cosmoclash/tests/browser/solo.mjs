import assert from 'node:assert/strict';
import {chromium} from 'playwright';
import {createRustRelay} from '../../../tests/helpers/rust-relay.mjs';
import fs from 'node:fs/promises';
import path from 'node:path';
import {createHash} from 'node:crypto';
import {fileURLToPath} from 'node:url';
const root=path.resolve(path.dirname(fileURLToPath(import.meta.url)),'../..');
const output=path.join(root,'test-results/browser');await fs.mkdir(output,{recursive:true});
const expectedHash=createHash('sha256').update(await fs.readFile(path.join(root,'dist/cosmoclash.wasm'))).digest('hex');
const relay=createRustRelay({distDir:path.join(root,'dist')});const address=await relay.listen(0);let browser,page;
const result={passed:false,wasmSha256:expectedHash};const errors=[],failed=[];
try{
 browser=await chromium.launch({executablePath:process.env.CHROME_BIN||(process.platform==='darwin'?'/Applications/Google Chrome.app/Contents/MacOS/Google Chrome':undefined),headless:true,args:['--use-angle=swiftshader','--enable-unsafe-swiftshader','--disable-background-timer-throttling','--disable-renderer-backgrounding']});
 page=await browser.newPage({viewport:{width:960,height:720}});
 // Observe the actual master output without replacing audio generation or game code.
 await page.addInitScript(()=>{
  const connect=AudioNode.prototype.connect;const probes=[];window.__audioProbe={read(){let peak=0;for(const analyser of probes){const pcm=new Float32Array(analyser.fftSize);analyser.getFloatTimeDomainData(pcm);for(const x of pcm)peak=Math.max(peak,Math.abs(x));}return peak;}};
  AudioNode.prototype.connect=function(destination,...args){const result=connect.call(this,destination,...args);if(destination===this.context.destination){const analyser=this.context.createAnalyser();analyser.fftSize=2048;connect.call(this,analyser);probes.push(analyser);}return result;};
 });
 page.on('pageerror',e=>errors.push(e.message));page.on('response',r=>{if(r.status()>=400&&!r.url().endsWith('favicon.ico'))failed.push({url:r.url(),status:r.status()})});
 await page.goto(`http://127.0.0.1:${address.port}`);
 await page.waitForFunction(()=>window.__cosmo?.state==='menu',{},{timeout:60000});
 assert.equal(await page.evaluate(()=>window.__cosmo.engine),expectedHash);
 await page.screenshot({path:path.join(output,'menu.png')});
 await page.getByRole('button',{name:'Play vs Computer',exact:true}).click();await page.getByText('Easy',{exact:true}).click();await page.keyboard.press('Enter');
 await page.waitForFunction(()=>window.__cosmo?.state==='playing',{},{timeout:10000});
 await page.keyboard.down('ArrowUp');await page.keyboard.down('/');await page.keyboard.down('.');
 await page.waitForFunction(()=>window.__cosmo.snapshot.players[0].morphState!==0,{},{timeout:10000});await page.keyboard.up('.');
 await page.waitForFunction(()=>window.__cosmo.snapshot.players[0].morphState===2&&window.__cosmo.snapshot.tick>70,{},{timeout:20000});
 await page.keyboard.up('ArrowUp');await page.keyboard.up('/');
 const solo=await page.evaluate(()=>({state:window.__cosmo.state,snapshot:window.__cosmo.snapshot,engine:window.__cosmo.engine}));
 assert.equal(solo.snapshot.players[0].shipId,'michiGichi');assert.ok(Math.abs(solo.snapshot.players[0].position.y+70)>10);assert.ok(solo.snapshot.aiController,'Rust AI must be active');
 await page.waitForFunction(()=>window.__audioProbe.read()>0.0001,{},{timeout:10000});
 result.audioPeak=await page.evaluate(()=>window.__audioProbe.read());assert.ok(result.audioPeak>0);
 await page.keyboard.press('m');await page.waitForTimeout(300);assert.equal(await page.evaluate(()=>window.__audioProbe.read()),0);await page.keyboard.press('m');
 await page.waitForFunction(()=>window.__audioProbe.read()>0.0001,{},{timeout:5000});result.muteRoundTrip=true;
 await page.screenshot({path:path.join(output,'solo-morph.png')});
 await page.getByRole('button',{name:'Menu',exact:true}).click();await page.getByRole('button',{name:'Two Players · Same Computer',exact:true}).click();
 await page.keyboard.press('ArrowRight');await page.keyboard.press('ArrowRight');await page.keyboard.press('Enter');
 await page.keyboard.press('ArrowRight');await page.keyboard.press('Enter');
 await page.waitForFunction(()=>window.__cosmo?.state==='playing',{},{timeout:10000});
 await page.keyboard.down('ArrowUp');await page.keyboard.down('/');await page.keyboard.down('.');await page.keyboard.down('w');await page.keyboard.down(' ');await page.keyboard.down('Shift');
 await page.waitForFunction(()=>window.__cosmo.snapshot.players[1].frostShieldActive,{},{timeout:10000});await page.keyboard.up('Shift');
 await page.waitForFunction(()=>window.__cosmo.snapshot.players[1].frostShieldActive&&window.__cosmo.snapshot.snots.length>0&&window.__cosmo.snapshot.tick>40,{},{timeout:15000});
 const local=await page.evaluate(()=>window.__cosmo.snapshot);
 assert.deepEqual(local.players.map(p=>p.shipId),['gondi','sholev']);assert.equal(local.aiController,null);assert.ok(local.bullets.some(b=>b.kind==='frost'));assert.ok(local.players.every(p=>p.position.y>-70));
 await page.screenshot({path:path.join(output,'local-shield-slime.png')});
 for(const key of ['ArrowUp','/','.','w',' '])await page.keyboard.up(key);
 await page.getByRole('button',{name:'Menu',exact:true}).click();
 // The unshipped debug instant-kill keys must have no effect in the playable build.
 await page.getByRole('button',{name:'Play vs Computer',exact:true}).click();await page.getByText('Hard',{exact:true}).click();await page.keyboard.press('Enter');
 await page.waitForFunction(()=>window.__cosmo?.state==='playing',{},{timeout:10000});await page.keyboard.press('1');await page.keyboard.press('2');
 assert.ok(await page.evaluate(()=>window.__cosmo.snapshot.players.every(p=>p.alive)));result.noDebugKills=true;
 await page.getByRole('button',{name:'Menu',exact:true}).click();
 await page.setViewportSize({width:390,height:844});await page.screenshot({path:path.join(output,'mobile-menu.png')});
 assert.equal(await page.evaluate(()=>document.documentElement.scrollWidth>innerWidth),false);
 assert.deepEqual(errors,[]);assert.deepEqual(failed,[]);
 Object.assign(result,{passed:true,soloMorph:true,localTwoPlayer:true,slime:true,frostShield:true,allShipsRendered:true,errors,failed});
 console.log(JSON.stringify(result));
}catch(error){result.error=String(error);result.errors=errors;result.failed=failed;result.state=await page?.evaluate(()=>({state:window.__cosmo?.state,snapshot:window.__cosmo?.snapshot})).catch(()=>null);await page?.screenshot({path:path.join(output,'solo-failure.png')}).catch(()=>{});throw error;}
finally{await fs.writeFile(path.join(output,'solo-results.json'),JSON.stringify(result,null,2));await browser?.close();await relay.close();}
