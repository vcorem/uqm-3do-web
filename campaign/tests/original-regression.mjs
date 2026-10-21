// Ordinary UQM remains available when the Afterlight adapter is disabled.
import {chromium} from 'playwright';
import {createNetplayServer} from '../../server/netplay.mjs';
import {mkdir,writeFile} from 'node:fs/promises';
import assert from 'node:assert/strict';
const out=process.env.AFTERLIGHT_ORIGINAL_RESULTS||'test-results/afterlight-original-regression';await mkdir(out,{recursive:true});
const server=createNetplayServer({distDir:process.env.AFTERLIGHT_ORIGINAL_DIST||'.tools/afterlight-original-regression'});const addr=await server.listen(0,'127.0.0.1');
let browser;const report={passed:false,errors:[]};
try{
 browser=await chromium.launch({headless:true,...(process.env.CHROME_BIN?{executablePath:process.env.CHROME_BIN}:{}),args:['--disable-renderer-backgrounding','--disable-backgrounding-occluded-windows','--disable-background-timer-throttling']});
 const page=await browser.newPage({viewport:{width:1200,height:1000}});page.on('pageerror',e=>report.errors.push(String(e)));
 await page.goto('http://127.0.0.1:'+addr.port+'/');await page.waitForFunction(()=>window.runtimeReady,null,{timeout:180000});
 assert.equal(await page.evaluate(()=>typeof Module._uqm_campaign_enable),'undefined');
 await page.getByRole('button',{name:'Launch game',exact:true}).click();
 await page.waitForFunction(()=>Module.gameScreen==='mainMenu',null,{timeout:90000});
 await page.locator('#canvas').focus();await page.keyboard.press('ArrowDown',{delay:200});await page.waitForTimeout(350);await page.keyboard.press('ArrowDown',{delay:200});await page.waitForTimeout(350);await page.keyboard.press('Enter',{delay:200});
 await page.waitForFunction(()=>Module.gameScreen==='melee',null,{timeout:30000});await page.waitForTimeout(1500);
 await page.locator('#canvas').screenshot({path:out+'/ordinary-melee.png'});assert.deepEqual(report.errors,[]);
 report.passed=true;report.screens=['mainMenu','melee'];report.campaignAPIExported=false;report.browser=browser.version();console.log(JSON.stringify(report));
}finally{await writeFile(out+'/results.json',JSON.stringify(report,null,2)+'\n');await browser?.close();await server.close();}
