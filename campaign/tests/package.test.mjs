import test from 'node:test';
import assert from 'node:assert/strict';
import {mkdtemp,mkdir,writeFile,readFile,rm,access} from 'node:fs/promises';
import {tmpdir} from 'node:os';
import path from 'node:path';
import {spawnSync} from 'node:child_process';
const script=new URL('../tools/package.py',import.meta.url).pathname;
test('portable packaging removes stale build assets but refuses unrelated directories',async()=>{
  const root=await mkdtemp(path.join(tmpdir(),'afterlight-package-'));
  try{
    const combat=path.join(root,'combat');await mkdir(combat);await writeFile(path.join(combat,'index.html'),'native test fixture');
    const output=path.join(root,'output');const run=()=>spawnSync('python3',[script],{encoding:'utf8',env:{...process.env,AFTERLIGHT_OUTPUT:output,AFTERLIGHT_COMBAT_DIST:combat}});
    assert.equal(run().status,0);await writeFile(path.join(output,'assets','removed-portrait.png'),'stale');assert.equal(run().status,0);
    await assert.rejects(access(path.join(output,'assets','removed-portrait.png')));
    const manifest=JSON.parse(await readFile(path.join(output,'build-manifest.json'),'utf8'));assert.equal(manifest.campaign,'afterlight-accord');assert.ok(manifest.files['src/lander.mjs']);assert.ok(!manifest.files['assets/removed-portrait.png']);
    await rm(path.join(output,'build-manifest.json'));const result=run();assert.notEqual(result.status,0);assert.match(result.stderr,/Refusing to replace/);assert.equal(await readFile(path.join(output,'combat/index.html'),'utf8'),'native test fixture');
  }finally{await rm(root,{recursive:true,force:true});}
});
