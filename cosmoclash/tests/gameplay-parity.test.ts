// @vitest-environment node
/** Differential oracle: unchanged upstream classes and extracted production
 * input/wrap functions versus Rust. Legacy JS never drives the shipped game. */
import { describe,it,expect,vi,afterAll } from 'vitest';
import { spawn } from 'node:child_process';
import { createInterface } from 'node:readline';
import { readFileSync,mkdirSync,writeFileSync } from 'node:fs';
import ts from 'typescript';
import { Entity } from '../src/engine/Entity';
import { CollisionDetector } from '../src/engine/CollisionDetector';
import { GameEventMgr } from '../src/engine/GameEventMgr';
import { GameEventType } from '../src/engine/GameEvent';
import { Player,GONDI_STATS } from '../src/game/Player';
import { SholevPlayer } from '../src/game/SholevPlayer';
import { MichiGichiPlayer,MorphState } from '../src/game/MichiGichiPlayer';
import { Bullet,PLASMA_BULLET_CONFIG,ICE_BULLET_CONFIG,FROST_SHIELD_CONFIG } from '../src/game/Bullet';
import { Snot,SNOT_CONFIG } from '../src/game/Snot';
import { Asteroid } from '../src/game/Asteroid';
import { Sun } from '../src/game/Sun';
import { Explosion } from '../src/game/Explosion';
import { CameraController,createCamera } from '../src/game/Camera';
import { CollisionHandler,enforceMinimumDistance } from '../src/game/CollisionHandler';
import { applySunGravity } from '../src/game/Gravity';
import { ARENA } from '../src/game/Arena';
import { SoundEffect } from '../src/audio/SoundEffect';
import { SoundEventListener } from '../src/audio/SoundEventListener';
import { SoundManager } from '../src/audio/SoundManager';
const evidence={frames:0,comparisons:0,maxNumericDifference:0,completedCases:0};
afterAll(()=>{vi.restoreAllMocks();mkdirSync('test-results/gameplay-parity',{recursive:true});writeFileSync('test-results/gameplay-parity/results.json',JSON.stringify({passed:evidence.completedCases===10,...evidence},null,2));});
const original=readFileSync(new URL('../reference/upstream-main.ts.txt',import.meta.url),'utf8');
const inputFn=original.slice(original.indexOf('function updatePlayerInput('),original.indexOf('// Sync mesh to player physics state'));
const wrapFn=original.slice(original.indexOf('function handlePlayerWrapAround('),original.indexOf('// Game loop'));
const source=ts.transpileModule(inputFn+'\n'+wrapFn,{compilerOptions:{target:ts.ScriptTarget.ES2022,module:ts.ModuleKind.None}}).outputText;
const noop=()=>({thrust:false,rotateLeft:false,rotateRight:false,firePrimary:false,fireSecondary:false});
class Native {
 child=spawn(new URL('../rust/target/debug/cosmo-jsonl',import.meta.url).pathname,[],{stdio:['pipe','pipe','inherit']});
 waiting:Array<{resolve:(v:any)=>void,reject:(e:Error)=>void}>=[];
 constructor(){createInterface({input:this.child.stdout}).on('line',line=>{const w=this.waiting.shift();if(!w)return;const v=JSON.parse(line);v.ok===false?w.reject(new Error(v.error)):w.resolve(v.snapshot??v);});this.child.on('error',e=>this.waiting.splice(0).forEach(w=>w.reject(e)));}
 request(v:any):Promise<any>{return new Promise((resolve,reject)=>{this.waiting.push({resolve,reject});this.child.stdin.write(JSON.stringify(v)+'\n');});}
 close(){this.child.stdin.end();}
}
function id(e:Entity,n:number){Object.defineProperty(e,'id',{value:n});return e;}
function base(e:any){return {id:e.id,position:{...e.position},previousPosition:{...e.previousPosition},accelVec:{...e.accelVec},angle:e.angle,health:e.health,alive:e.alive,collisionSpheres:e.collisionSpheres};}
function playerView(p:any){return {...base(p),rollAngle:p.rollAngle,energy:p.energy,primaryCooldown:p.primaryCooldown,secondaryCooldown:p.secondaryCooldown,snotCount:p.snotCount,morphState:p.morphState??0,currentAnimFrame:p.currentAnimFrame??1,frostShieldActive:p.frostShieldActive??false,frostShieldActiveTime:p.frostShieldActiveTime??0,stats:{speedMaxScaler:p.stats.speedMaxScaler,acceleration:p.stats.acceleration,rotationSpeed:p.stats.rotationSpeed,primaryEnergyCost:p.stats.primaryEnergyCost}};}
function bulletView(b:any){return {...base(b),ttl:b.ttl,animFrame:b.animFrame,animSpeed:b.animSpeed,localOffsetDistance:b.localOffsetDistance,attachedSpeed:b.attachedSpeed,damageCooldown:b.damageCooldown};}
function snotView(s:any){return {...base(s),ttl:s.ttl,attached:s.attached,currentFrame:s.currentFrame};}
function asteroidView(a:any){return {...base(a),asteroidType:a.asteroidType,curXAngle:a.curXAngle,curYAngle:a.curYAngle};}
function rustView(w:any){return {players:w.players.map(playerView),bullets:w.bullets.map(bulletView),snots:w.snots.map(snotView),asteroids:w.asteroids.map(asteroidView),explosions:w.explosions.map((e:any)=>({...base(e),currentFrame:e.currentFrame})),sun:{...base(w.sun),currentFrame:w.sun.currentFrame},camera:w.camera.position,winner:w.winner,sounds:w.events.filter((e:any)=>e.sound).map((e:any)=>e.sound)};}
function same(a:any,b:any,path='world'){
 evidence.comparisons++;
 if(typeof a==='number'&&typeof b==='number'){evidence.maxNumericDifference=Math.max(evidence.maxNumericDifference,Math.abs(a-b));if(!Number.isFinite(a)||!Number.isFinite(b)||Math.abs(a-b)>1e-10*Math.max(1,Math.abs(a),Math.abs(b)))throw new Error(`${path}: Rust ${a} != TS ${b}`);return;}
 if(a===null||b===null||typeof a!=='object'||typeof b!=='object'){expect(a,path).toBe(b);return;}
 expect(Object.keys(a),path).toEqual(Object.keys(b));for(const k of Object.keys(a))same(a[k],b[k],path+'.'+k);
}
class Reference {
 players:[Player,Player];sun=new Sun();bullets:Bullet[]=[];snots:Snot[]=[];asteroids:Asteroid[]=[];explosions:Explosion[]=[];camera=createCamera();cameraCtl=new CameraController();winner:number|null=null;nextId:number;rng:number;input:any;wrap:any;sounds:string[]=[];
 constructor(w:any){
  const make=(p:any)=>{const e=p.shipId==='michiGichi'?new MichiGichiPlayer():p.shipId==='sholev'?new SholevPlayer():new Player(GONDI_STATS);id(e,p.id);for(const k of ['position','previousPosition','accelVec','angle','health','energy','alive','rollAngle','primaryCooldown','secondaryCooldown','snotCount'])(e as any)[k]=structuredClone(p[k]);return e;};
  this.players=w.players.map(make) as [Player,Player];this.nextId=w.nextId;this.rng=w.rng.state;id(this.sun,w.sun.id);this.sun.position={...w.sun.position};this.sun.previousPosition={...w.sun.previousPosition};this.camera.position.set(w.camera.position.x,w.camera.position.y,w.camera.position.z);
  this.asteroids=w.asteroids.map((a:any)=>{const result=new Asteroid(a.asteroidType);id(result,a.id);for(const k of ['position','previousPosition','accelVec','angle','health','alive','curXAngle','curYAngle'])(result as any)[k]=structuredClone(a[k]);return result;});
  vi.spyOn(SoundManager,'getInstance').mockReturnValue({play:(sound:string)=>this.sounds.push(sound)} as any);
  GameEventMgr.reset();const mgr=GameEventMgr.getInstance();const handler=new CollisionHandler();for(const type of [GameEventType.BulletHit,GameEventType.Collision,GameEventType.SunCollision,GameEventType.SnotHit])mgr.addListener(handler,type);
  const sounds=new SoundEventListener();for(const type of [GameEventType.BulletHit,GameEventType.Explosion,GameEventType.SunCollision,GameEventType.SnotHit])mgr.addListener(sounds,type);
  mgr.addListener({onGameEvent:(event:any)=>{const e=new Explosion();id(e,this.nextId++);e.position={...event.entity.position};e.previousPosition={...event.entity.position};this.explosions.push(e);}},GameEventType.Explosion);
  const acquireBullet=(config:any,origin:any,target?:any)=>{const b=new Bullet(config,origin);id(b,this.nextId++);if(config.tracking&&target)b.trackingTarget=target;if(config===PLASMA_BULLET_CONFIG)b.animSpeed=.4;if(config===ICE_BULLET_CONFIG)b.animSpeed=.2;this.bullets.push(b);return b;};
  const acquireSnot=(origin:any,target:any)=>{const s=new Snot(origin,target);id(s,this.nextId++);this.snots.push(s);return s;};
  const factory=new Function('ARENA','MichiGichiPlayer','SholevPlayer','MorphState','FROST_SHIELD_CONFIG','SNOT_CONFIG','SoundManager','SoundEffect','acquireBullet','acquireSnot',`let p1PrevSecondary=false,p2PrevSecondary=false;${source};return {input:updatePlayerInput,wrap:handlePlayerWrapAround};`);
  const funcs=factory(ARENA,MichiGichiPlayer,SholevPlayer,MorphState,FROST_SHIELD_CONFIG,SNOT_CONFIG,SoundManager,SoundEffect,acquireBullet,acquireSnot);this.input=funcs.input;this.wrap=funcs.wrap;
 }
 random(){let x=this.rng;x^=x<<13;x^=x>>>17;x^=x<<5;this.rng=x>>>0;return this.rng/4294967296;}
 step(dt:number,inputs:any[]){this.sounds=[];evidence.frames++;const prior=Math.random;Math.random=()=>this.random();try{
  const cam={x:this.camera.position.x,y:this.camera.position.y,z:this.camera.position.z};let wrapped=false;
  if(this.winner===null){new CollisionDetector().checkCollisions([...this.players,this.sun,...this.bullets,...this.snots,...this.asteroids]);enforceMinimumDistance(this.players[0],this.players[1]);this.input(this.players[0],this.players[1],inputs[0],{update:()=>{}},dt,true);this.input(this.players[1],this.players[0],inputs[1],{update:()=>{}},dt,false);
   for(const p of this.players)if(p instanceof MichiGichiPlayer)p.updateMorph(dt);for(const p of this.players)if(p instanceof SholevPlayer)p.updateFrostShield(dt);
   for(const p of this.players)if(p.alive)p.update(dt);this.sun.update(dt);this.sun.handleWrapAround(cam);for(const b of this.bullets){b.update(dt);b.handleWrapAround(cam);}for(const s of this.snots){s.update(dt);s.handleWrapAround(cam);}for(const a of this.asteroids){a.update(dt);a.handleWrapAround(cam);}for(const p of this.players)applySunGravity(p,this.sun.position,dt);wrapped=this.wrap(...this.players);if(!this.players[0].alive||!this.players[1].alive)this.winner=this.players[0].alive?1:2;
  }
  this.sun.updateAnimation(dt);this.cameraCtl.update(this.camera,this.players[0].position.x,this.players[0].position.y,this.players[1].position.x,this.players[1].position.y,dt,wrapped);
  for(const b of this.bullets){if(b.config===PLASMA_BULLET_CONFIG){b.animFrame+=b.animSpeed*dt;if(b.animFrame>41)b.animSpeed=.03;if(b.animFrame>79)b.animFrame=41;}else if(b.config===ICE_BULLET_CONFIG){b.animFrame+=b.animSpeed*dt;if(b.animFrame>64)b.animFrame=0;}}
  for(const e of this.explosions){e.update(dt);e.handleWrapAround(cam);}this.bullets=this.bullets.filter(b=>b.alive);this.snots=this.snots.filter(b=>b.alive);this.asteroids=this.asteroids.filter(b=>b.alive);this.explosions=this.explosions.filter(b=>b.alive);
 }finally{Math.random=prior;}}
 view(){return {players:this.players.map(playerView),bullets:this.bullets.map(bulletView),snots:this.snots.map(snotView),asteroids:this.asteroids.map(asteroidView),explosions:this.explosions.map(e=>({...base(e),currentFrame:e.currentFrame})),sun:{...base(this.sun),currentFrame:this.sun.currentFrame},camera:{x:this.camera.position.x,y:this.camera.position.y,z:this.camera.position.z},winner:this.winner,sounds:this.sounds};}
}
const ships=['gondi','michiGichi','sholev'];
describe('actual upstream gameplay versus Rust',()=>{
 it('oracle rejects a changed movement result',()=>{expect(()=>same({position:{x:1,y:2}},{position:{x:1.01,y:2}})).toThrow();evidence.maxNumericDifference=0;});
 for(const p of ships)for(const q of ships)it(`${p} versus ${q}: every tick`,async()=>{
  const native=new Native();try{const init=await native.request({op:'init',seed:9274,ships:[p,q]});init.players[0].angle=90;init.players[1].angle=270;const reference=new Reference(init);await native.request({op:'testLoad',snapshot:init});
   for(let frame=0;frame<500;frame++){const inputs=[0,1].map(()=>({thrust:true,rotateLeft:frame>160&&frame%160<60,rotateRight:frame>160&&frame%160>=60&&frame%160<95,firePrimary:frame%250<190,fireSecondary:frame%180<25}));const dt=[1000/60,10,20,40][Math.floor(frame/125)];reference.step(dt,inputs);const actual=await native.request({op:'step',dt,inputs});same(rustView(actual),reference.view(),`${p}/${q} frame ${frame}`);}
  evidence.completedCases++;}finally{native.close();}
 },30000);
 it('seeded snot orbit and expiration, shield toggles, morph reversal and player wrap',async()=>{
  const native=new Native();try{
   for(const [p,q] of [['gondi','gondi'],['sholev','gondi'],['michiGichi','gondi']]){const init=await native.request({op:'init',seed:17,ships:[p,q]});init.asteroids=[];init.players[0].position={x:500,y:500,z:0};init.players[0].previousPosition={...init.players[0].position};init.players[0].angle=270;init.players[1].position={x:535,y:500,z:0};init.players[1].previousPosition={...init.players[1].position};const reference=new Reference(init);await native.request({op:'testLoad',snapshot:init});
    for(let frame=0;frame<650;frame++){const inputs=[{...noop(),fireSecondary:frame===0||frame===140||frame===200},noop()];const dt=20;reference.step(dt,inputs);const actual=await native.request({op:'step',dt,inputs});same(rustView(actual),reference.view(),`${p} special ${frame}`);}
   }
   const init=await native.request({op:'init',seed:17,ships:['gondi','gondi']});init.asteroids=[];init.players[1].position.x=-1600;init.players[1].previousPosition={...init.players[1].position};init.players[0].accelVec={x:1,y:1,z:0};const reference=new Reference(init);await native.request({op:'testLoad',snapshot:init});for(let frame=0;frame<100;frame++){reference.step(20,[noop(),noop()]);const actual=await native.request({op:'step',dt:20,inputs:[noop(),noop()]});same(rustView(actual),reference.view(),`wrap ${frame}`);}
  evidence.completedCases++;}finally{native.close();}
 },30000);
});
