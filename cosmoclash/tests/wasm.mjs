import assert from 'node:assert/strict';
import fs from 'node:fs/promises';
import path from 'node:path';
import {createHash} from 'node:crypto';
import {spawn} from 'node:child_process';
import {createInterface} from 'node:readline';
import {fileURLToPath} from 'node:url';
const root=path.resolve(path.dirname(fileURLToPath(import.meta.url)),'..');
const bytes=await fs.readFile(path.join(root,'public/cosmoclash.wasm'));
const encoder=new TextEncoder(),decoder=new TextDecoder();
async function wasm(){const {instance}=await WebAssembly.instantiate(bytes,{});const e=instance.exports;return {e,request(command){const b=encoder.encode(JSON.stringify(command));const ptr=e.cosmo_alloc(b.length);assert.ok(ptr);new Uint8Array(e.memory.buffer,ptr,b.length).set(b);try{const out=e.cosmo_request(ptr,b.length);return JSON.parse(decoder.decode(new Uint8Array(e.memory.buffer,out,e.cosmo_result_len())));}finally{e.cosmo_dealloc(ptr,b.length);}}};}
const a=await wasm(),b=await wasm();
const binary=process.env.COSMO_ORACLE||path.join(root,'rust/target/debug/cosmo-jsonl');
const native=spawn(binary,[],{stdio:['pipe','pipe','pipe']});
let nativeError='';native.stderr.on('data',x=>{nativeError=(nativeError+x).slice(-4000)});
const pending=[];
const lines=createInterface({input:native.stdout});
lines.on('line',line=>{const next=pending.shift();if(!next)throw new Error('Unexpected oracle response');next.resolve(JSON.parse(line));});
native.on('error',error=>{for(const p of pending.splice(0))p.reject(error)});
native.on('exit',code=>{for(const p of pending.splice(0))p.reject(new Error(`Native oracle exited ${code}: ${nativeError}`));});
const request=command=>new Promise((resolve,reject)=>{pending.push({resolve,reject});native.stdin.write(JSON.stringify(command)+'\n');});
let comparisons=0,maxDifference=0,frames=0;
function compare(x,y,where='state'){
 if(typeof x==='number'&&typeof y==='number'){
  assert.ok(Number.isFinite(x)&&Number.isFinite(y),`${where} finite`);const diff=Math.abs(x-y);maxDifference=Math.max(maxDifference,diff);
  assert.ok(diff<=1e-8+Math.max(Math.abs(x),Math.abs(y))*1e-10,`${where}: ${x} != ${y}`);comparisons++;return;
 }
 if(x&&y&&typeof x==='object'&&typeof y==='object'){
  assert.deepEqual(Object.keys(x),Object.keys(y),`${where} keys`);for(const key of Object.keys(x))compare(x[key],y[key],`${where}.${key}`);return;
 }
 assert.equal(x,y,where);comparisons++;
}
const ships=['michiGichi','gondi','sholev'];
let random=0x12345678;function rand(){random^=random<<13;random^=random>>>17;random^=random<<5;return random>>>0;}
const flags=mask=>({thrust:!!(mask&1),rotateLeft:!!(mask&2),rotateRight:!!(mask&4),firePrimary:!!(mask&8),fireSecondary:!!(mask&16)});
const cases=[];
try{
 for(const p1 of ships)for(const p2 of ships)for(const aiDifficulty of [undefined,'easy','medium','hard']){
  const init={op:'init',seed:rand(),ships:[p1,p2],...(aiDifficulty?{aiDifficulty}:{})};
  const start=a.request(init);assert.equal(start.ok,true);assert.deepEqual(start,b.request(init));compare(start,await request(init));
  let tick=0;
  for(;tick<360;tick++){
   const command={op:'step',dt:1000/60,inputs:[flags((tick%90<70?1:0)|(tick%160<80?2:4)|8|(tick%120===0?16:0)),flags(rand()%32)]};
   const got=a.request(command);assert.equal(got.ok,true);assert.deepEqual(got,b.request(command),'Two independent Wasm instances diverged');compare(got,await request(command));frames++;
  }
  cases.push({ships:[p1,p2],aiDifficulty:aiDifficulty??null,frames:tick});
 }
 const initial=a.request({op:'snapshot'});
 for(const command of [null,{}, {op:'step',dt:0,inputs:[{},{}]},{op:'step',dt:-1,inputs:[{},{}]},{op:'step',dt:101,inputs:[{},{}]},{op:'step',dt:16,inputs:[{thrust:1},{}]},{op:'init',seed:1,ships:['invalid','gondi']},{op:'init',seed:-1,ships:['gondi','sholev']},{op:'init',seed:1,ships:['gondi','sholev'],aiDifficulty:'impossible'},{op:'unknown'},{op:'snapshot',unexpected:true}]){
  assert.equal(a.request(command).ok,false,`Invalid request accepted: ${JSON.stringify(command)}`);assert.deepEqual(a.request({op:'snapshot'}),initial,'Rejected request mutated state');
 }
 assert.equal(a.e.cosmo_alloc(0),0);assert.equal(a.e.cosmo_alloc(65537),0);assert.equal(a.e.cosmo_alloc(0xffffffff),0);
 const pointers=Array.from({length:16},()=>a.e.cosmo_alloc(16));assert.ok(pointers.every(Boolean));assert.equal(a.e.cosmo_alloc(16),0);
 for(const ptr of pointers){a.e.cosmo_dealloc(ptr,15);a.e.cosmo_dealloc(ptr,16);a.e.cosmo_dealloc(ptr,16);}
 for(const [ptr,len] of [[0,10],[0xfffffff0,32],[pointers[0],16]]){const out=a.e.cosmo_request(ptr,len);const result=JSON.parse(decoder.decode(new Uint8Array(a.e.memory.buffer,out,a.e.cosmo_result_len())));assert.equal(result.ok,false);}
 const memoryBefore=a.e.memory.buffer.byteLength;
 for(let i=0;i<1000;i++)assert.equal(a.request({op:'init',seed:i,ships:['gondi','sholev']}).ok,true);
 const memoryAfter=a.e.memory.buffer.byteLength;assert.ok(memoryAfter-memoryBefore<=2*65536,'Unbounded memory growth across resets');
 const result={passed:true,wasmSha256:createHash('sha256').update(bytes).digest('hex'),cases,frames,fieldComparisons:comparisons,maxNumericDifference:maxDifference,abiValidation:true,resets:1000,memoryBefore,memoryAfter};
 await fs.mkdir(path.join(root,'test-results'),{recursive:true});await fs.writeFile(path.join(root,'test-results/wasm.json'),JSON.stringify(result,null,2));
 console.log(JSON.stringify({...result,cases:cases.length},null,2));
}finally{native.stdin.end();lines.close();native.kill('SIGTERM');}
