/** Browser regressions for the two independently reproduced save-loss failures. */
import assert from 'node:assert/strict';
import {readFile,writeFile,mkdir} from 'node:fs/promises';
import {fileURLToPath} from 'node:url';
import {chromium} from 'playwright';
import {createNetplayServer} from '../../server/netplay.mjs';
import {createGame,serializeSave,applyAction} from '../src/state.mjs';
const data=JSON.parse(await readFile(new URL('../content/campaign.json',import.meta.url),'utf8'));
const output=new URL('../../test-results/afterlight-recovery/',import.meta.url);await mkdir(output,{recursive:true});
const server=createNetplayServer({distDir:fileURLToPath(new URL('../../.tools/afterlight-dist',import.meta.url))});
const report={passed:false,checks:[],errors:[]};const key='uqm.afterlight.save.v1';let browser;
try{
 const address=await server.listen(0,'127.0.0.1'),url=`http://127.0.0.1:${address.port}/`;
 browser=await chromium.launch({headless:true,...(process.env.CHROME_BIN?{executablePath:process.env.CHROME_BIN}:{})});report.browser=browser.version();
 const existing=applyAction(data,createGame(data,{captainName:'Existing Captain'}),{type:'choose',choiceId:'a1_wake_duty'}).state;
 const context=await browser.newContext(),page=await context.newPage();page.on('pageerror',e=>report.errors.push(e.message));
 await page.goto(url);await page.locator('[data-action=begin]').waitFor();
 await page.evaluate(({key,save})=>{localStorage.setItem(key,save);localStorage.setItem('uqm.afterlight.preferences.v1','{broken');},{key,save:serializeSave(data,existing)});
 await page.reload();await page.locator('[data-action=resume]').waitFor();await page.locator('[data-action=begin]').click();
 assert.equal(await page.locator('#confirm-dialog[open]').count(),1,'healthy save must trigger replacement confirmation despite broken preferences');
 await page.getByRole('button',{name:'Go back',exact:true}).click();await page.locator('[data-action=resume]').click();
 assert.deepEqual(await page.evaluate(()=>__afterlight.state),existing);report.checks.push('Malformed preferences preserve Continue, overwrite confirmation and the exact expedition.');
 // Recover an otherwise healthy backup even when the primary key is absent.
 await page.evaluate(({key,save})=>{localStorage.removeItem(key);localStorage.setItem(key+'.previous',save);},{key,save:serializeSave(data,existing)});
 await page.reload();await page.locator('[data-action=resume]').click();assert.deepEqual(await page.evaluate(()=>__afterlight.state),existing);
 report.checks.push('A missing primary save can recover its valid backup.');await context.close();

 const quota=await browser.newContext(),q=await quota.newPage();q.on('pageerror',e=>report.errors.push(e.message));await q.goto(url);await q.locator('[data-action=begin]').click();
 await q.evaluate(key=>{const original=Storage.prototype.setItem;window.__restoreWrites=()=>{Storage.prototype.setItem=original;};Storage.prototype.setItem=function(k,v){if(k===key)throw new DOMException('Quota exceeded','QuotaExceededError');return original.call(this,k,v);};},key);
 await q.locator('[data-id=a1_wake_duty]').click();const unsaved=await q.evaluate(()=>__afterlight.state);
 await q.locator('[data-action=menu]').click();await q.locator('[data-action=title]').click();
 assert.deepEqual(await q.evaluate(()=>__afterlight.state),unsaved,'failed saving must retain the live expedition');assert.equal(await q.locator('[data-action=export]').count(),1);
 const dl=q.waitForEvent('download');await q.locator('[data-action=export]').click();const download=await dl;await download.saveAs(fileURLToPath(new URL('unsaved-export.json',output)));
 assert.deepEqual(JSON.parse(await readFile(new URL('unsaved-export.json',output),'utf8')).state,unsaved);
 await q.evaluate(()=>__restoreWrites());await q.locator('[data-action=title]').click();await q.locator('[data-action=resume]').click();assert.deepEqual(await q.evaluate(()=>__afterlight.state),unsaved);
 report.checks.push('Write failure keeps live state and export available; restoring storage saves the exact latest revision.');
 // UI navigation must match the named destination, with the unfinished scene resumable.
 await q.locator('[data-action=tab][data-page=chart]').click();assert.equal(await q.locator('.chart-layout').count(),1);
 await q.locator('[data-action=enter][data-id=a1_brief]').click();assert.equal(await q.locator('[data-scene=a1_brief]').count(),1);
 assert.equal(await q.locator('[data-action=continue]').count(),0,'continuous reading is the default');
 await q.locator('[data-action=menu]').click();await q.locator('[data-action=reading]').click();await q.locator('[data-action=menu]').click();
 assert.equal(await q.locator('[data-action=continue]').count(),1,'short-passage preference is still supported');report.checks.push('Chart opens the map; an unfinished conversation resumes; both reading modes work.');
 await q.locator('[data-action=tab][data-page=chart]').click();await q.locator('#destination').selectOption('cinder');await q.locator('[data-action=travel]').click();await q.locator('#destination').selectOption('hearth');await q.locator('[data-action=travel]').click();await q.locator('nav [data-action=tab][data-page=ship]').click();
 const before=await q.evaluate(()=>__afterlight.state.resources);await q.locator('#refuel-amount').fill('1');await q.locator('[data-action=refuel]').click();const after=await q.evaluate(()=>__afterlight.state.resources);
 assert.equal(after.fuel,before.fuel+1);assert.equal(after.credits,before.credits-(data.economy?.refuelPrice??2));report.checks.push('Partial refuel purchases preserve remaining money.');
 await q.screenshot({path:fileURLToPath(new URL('resupply.png',output)),fullPage:true});await quota.close();
 const freshFailure=await browser.newContext(),f=await freshFailure.newPage();f.on('pageerror',e=>report.errors.push(e.message));await f.goto(url);await f.locator('[data-action=begin]').waitFor();
 await f.evaluate(key=>{const original=Storage.prototype.setItem;Storage.prototype.setItem=function(k,v){if(k===key)throw new DOMException('Quota exceeded','QuotaExceededError');return original.call(this,k,v);};},key);
 await f.locator('[data-action=begin]').click();await f.locator('[data-id=a1_wake_duty]').click();const live=await f.evaluate(()=>__afterlight.state);
 await f.locator('#import-file').setInputFiles({name:'other.json',mimeType:'application/json',buffer:Buffer.from(serializeSave(data,existing))});await f.locator('#confirm-dialog[open]').waitFor();
 assert.match(await f.locator('#confirm-text').innerText(),/not saved/);await f.getByRole('button',{name:'Go back',exact:true}).click();assert.deepEqual(await f.evaluate(()=>__afterlight.state),live);report.checks.push('Import cannot silently replace a live expedition when no autosave has ever succeeded.');await freshFailure.close();
 assert.deepEqual(report.errors,[]);report.passed=true;console.log(JSON.stringify(report,null,2));
}catch(error){report.failure=String(error.stack||error);throw error;}
finally{await writeFile(new URL('results.json',output),JSON.stringify(report,null,2)+'\n');await browser?.close();await server.close();}
