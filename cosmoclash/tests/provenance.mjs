import fs from 'node:fs/promises';
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import path from 'node:path';
import {fileURLToPath} from 'node:url';
const root=path.resolve(path.dirname(fileURLToPath(import.meta.url)),'..');
const manifest=JSON.parse(await fs.readFile(path.join(root,'reference/upstream.json'),'utf8'));
const sha=bytes=>createHash('sha256').update(bytes).digest('hex');
// Vitest 4 requires constructable mocks. Preserve the exact upstream tests
// separately; active copies only replace constructor arrow mocks with functions.
const preservedTests = new Map([
 ['src/game/SelectionScene.test.ts', 'reference/upstream-tests/src/game/SelectionScene.test.ts'],
 ['src/game/ThrustEffect.test.ts', 'reference/upstream-tests/src/game/ThrustEffect.test.ts'],
]);
let assets=0,references=0;
for(const [file,expected] of Object.entries(manifest.sha256)){
 if(file.startsWith('public/')||/^src\/(game|engine|ai)\//.test(file)){
  assert.equal(sha(await fs.readFile(path.join(root,preservedTests.get(file) ?? file))),expected,`Upstream reference changed: ${file}`);
  if(file.startsWith('public/'))assets++;else references++;
 }
}
assert.equal(sha(await fs.readFile(path.join(root,'reference/upstream-main.ts.txt'))),manifest.sha256['src/main.ts']);
assert.equal(sha(await fs.readFile(path.join(root,'reference/UPSTREAM-README.md'))),manifest.sha256['README.md']);
console.log(JSON.stringify({passed:true,upstream:manifest.commit,assets,references,preservedEntryPoint:true,preservedUpstreamTests:preservedTests.size}));
