import test from 'node:test';
import assert from 'node:assert/strict';
import {readFile,stat} from 'node:fs/promises';
import path from 'node:path';
import {createHash} from 'node:crypto';
const root=path.resolve(import.meta.dirname,'..');
const load=async name=>JSON.parse(await readFile(path.join(root,name),'utf8'));
test('Every speaking character has its own usable portrait and original score cue',async()=>{
 const data=await load('content/campaign.json'),music=await load('assets/music/manifest.json');
 for(const character of data.characters){
  assert(character.portrait.startsWith('assets/portraits/'));assert(!character.portrait.includes('..'));
  const png=await readFile(path.join(root,character.portrait));
  assert.equal(png.subarray(1,4).toString(),'PNG');assert(png.readUInt32BE(16)>=1024);assert(png.readUInt32BE(20)>=1024);
  assert(music.tracks.some(track=>track.id===character.music),character.id+' music cue');
 }
});
test('All fifteen authored score cues have valid Ogg files, MIDI scores and measured headroom',async()=>{
 const manifest=await load('assets/music/manifest.json');assert.equal(manifest.tracks.length,15);
 assert.equal(manifest.channels,2);assert.equal(new Set(manifest.tracks.map(t=>t.id)).size,15);
 for(const track of manifest.tracks){
  const file=await readFile(path.join(root,track.file));assert.equal(file.subarray(0,4).toString(),'OggS');
  assert.equal(createHash('sha256').update(file).digest('hex'),track.sha256);
  assert(track.duration>=50&&track.duration<=150);assert(track.peak<.85&&track.peak>.5);assert(track.rms>.03&&track.rms<.3);assert(track.loopBoundaryJump<.08);
  assert.equal((await readFile(path.join(root,'assets/music',track.id+'.mid'))).subarray(0,4).toString(),'MThd');
 }
});
test('Title artwork is present and project references stay inside the resource package',async()=>{
 assert((await stat(path.join(root,'assets/scenes/afterlight.png'))).size>100000);
 const data=await load('content/campaign.json');
 assert(data.systems.length>=30);assert(data.characters.length>=10);
 for(const character of data.characters)assert(!/https?:|file:|^\//.test(character.portrait));
});
