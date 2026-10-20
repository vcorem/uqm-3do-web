// @vitest-environment node
// Actual upstream TypeScript is the oracle; no duplicate TypeScript AI is used.
import { describe, it, expect, vi } from 'vitest';
import { spawnSync } from 'node:child_process';
import { existsSync, mkdirSync, writeFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { AIController } from '../src/ai/AIController';
import { AIAttackMode } from '../src/ai/AIAttack';
import { AIDefenseMode } from '../src/ai/AIDefense';
import { AIOutput } from '../src/ai/AIOutput';
import { getDifficultyConstants, type DifficultyLevel } from '../src/ai/AIConstants';
import { AILearning } from '../src/ai/AILog';
import * as B from '../src/ai/AIBridge';
import * as M from '../src/ai/AICalc';
import * as T from '../src/ai/AITactics';
import { Player, GONDI_STATS } from '../src/game/Player';
import { MichiGichiPlayer } from '../src/game/MichiGichiPlayer';
import { SholevPlayer } from '../src/game/SholevPlayer';
import { CHARACTER_REGISTRY } from '../src/game/CharacterRegistry';

const executable = process.env.COSMO_AI_ORACLE ?? resolve('rust/target/debug/cosmo-jsonl');
const evidenceDir = resolve(process.env.COSMO_AI_EVIDENCE_DIR ?? 'test-results/ai-parity');
const counters = {controllerFrames:0,tacticCases:0,mathCases:0,modeCases:0,learningEvents:0,comparisons:0,maxNumericDifference:0};
function random(seed:number) {
  let state=seed>>>0, calls=0;
  return {next:()=>{state^=state<<13;state^=state>>>17;state^=state<<5;calls++;return (state>>>0)/4294967296;},
    state:()=>state>>>0,calls:()=>calls};
}
function native(request:any) {
  if (!existsSync(executable)) throw new Error(`Build Rust's cosmo-jsonl binary first: ${executable}`);
  const r=spawnSync(executable,[],{input:JSON.stringify({op:'ai_oracle',...request})+'\n',encoding:'utf8',maxBuffer:256*1024*1024,timeout:60000});
  if (r.status!==0 || r.error) throw new Error(`Rust oracle failed: ${r.error ?? r.stderr}`);
  const result=JSON.parse(r.stdout);if(result.error)throw new Error(result.error);return result;
}
function equal(actual:any,expected:any,path='root') {
  counters.comparisons++;
  if (typeof expected==='number') {
    // Decisions, timers, learning and RNG must match exactly. Only derived
    // geometry permits tiny native-libm/V8 acos rounding differences.
    const geometry=/\.(ai|user)\.|\.(distance|vectorSize|angle|velocityAngle|aim|speed|ttl|maxDistance)$/.test(path);
    const difference=Math.abs(actual-expected);
    counters.maxNumericDifference=Math.max(counters.maxNumericDifference,difference);
    const tolerance=geometry?1e-10+2e-12*Math.abs(expected):0;
    if (typeof actual!=='number' || difference>tolerance) {
      throw new Error(`${path}: Rust ${actual}, TypeScript ${expected}`);
    }
  }else if(Array.isArray(expected)) {
    if(!Array.isArray(actual)||actual.length!==expected.length)throw new Error(`${path}: array length differs`);
    expected.forEach((x,i)=>equal(actual[i],x,`${path}[${i}]`));
  }else if(expected!==null&&typeof expected==='object') {
    if(actual===null||typeof actual!=='object')throw new Error(`${path}: object missing`);
    const ak=Object.keys(actual).sort(),ek=Object.keys(expected).sort();
    if(JSON.stringify(ak)!==JSON.stringify(ek))throw new Error(`${path}: keys ${ak} != ${ek}`);
    for(const k of ek)equal(actual[k],expected[k],`${path}.${k}`);
  }else if(actual!==expected)throw new Error(`${path}: Rust ${actual}, TypeScript ${expected}`);
}
function plain(x:any){return JSON.parse(JSON.stringify(x));}
function ctxSnapshot(ctx:any){const {out,...rest}=ctx;return plain(rest);}
function controllerSnapshot(controller:any){return plain({level:controller.level,num:controller.num,out:controller.out,
  attackMode:controller.attackMode,defenseMode:controller.defenseMode,ctx:ctxSnapshot(controller.ctx),
  lastAILife:controller.lastAILife,lastUserLife:controller.lastUserLife});}
function tableSnapshot(t:any) {
  const {player,...rest}=t;
  return plain({...rest,morphState:rest.morphState??null,
    shots:rest.shots.map((s:any)=>({position:{x:s.position.x,y:s.position.y},config:{tracking:!!s.config.tracking}})),
    snots:rest.snots.map((s:any)=>({position:{x:s.position.x,y:s.position.y},attached:s.attached}))});
}
function charDef(type:number) {
  return CHARACTER_REGISTRY.find(d=>B.getAIEntityType(d)===type)!;
}
function player(s:any):Player {
  const p=s.entityType===1?new MichiGichiPlayer():s.entityType===8?new SholevPlayer():new Player(GONDI_STATS);
  p.position={...s.position,z:0};p.accelVec={...s.accelVec,z:0};p.angle=s.angle;p.health=s.health;p.energy=s.energy;p.snotCount=s.snotCount;
  // The bridge consumes effective speed, already computed by gameplay. Its
  // gameplay calculation is independently covered in the core engine parity.
  p.getEffectiveMaxSpeed=()=>s.maxSpeed;
  if(p instanceof MichiGichiPlayer)p.morphState=s.morphState;
  return p;
}
function pools(a:any,u:any,ap:Player,up:Player) {
  const shots=[...a.shots.map((b:any)=>({...b,originEntity:ap})),...u.shots.map((b:any)=>({...b,originEntity:up}))];
  const snots=[...a.snots.map((b:any)=>({...b,originEntity:ap})),...u.snots.map((b:any)=>({...b,originEntity:up}))];
  // Exercise owner filtering with unrelated live projectiles too.
  shots.unshift({position:{x:ap.position.x,y:ap.position.y},config:{tracking:true},originEntity:{}});
  return [{forEachActive:(fn:any)=>shots.forEach(fn)},{forEachActive:(fn:any)=>snots.forEach(fn)}] as any;
}
function sample(r:ReturnType<typeof random>,type:number,i:number) {
  const p={x:(r.next()-.5)*3000,y:(r.next()-.5)*2100};
  const speeds=[0,.001,.019,.02,.05,.06,.069,.07,.2,.5,1.2];
  const life=[1,1,.99,.75,.4,.05,0][i%7];
  return {position:p,angle:r.next()*360,accelVec:{x:(r.next()-.5)*3,y:(r.next()-.5)*3},
    health:life,energy:[0,.005,.01,.05,.1,.3,.4,.5,.75,.8,1][i%11],maxSpeed:speeds[i%speeds.length],
    snotCount:i%6,entityType:type,morphState:type===1?i%4:null,
    shots:Array.from({length:i%17},(_,j)=>({position:{x:p.x+(r.next()-.5)*1000,y:p.y+(r.next()-.5)*1000},config:{tracking:j%2===0}})),
    snots:Array.from({length:i%6},(_,j)=>({position:{x:p.x+(r.next()-.5)*300,y:p.y+(r.next()-.5)*300},attached:j%2===0}))};
}
function table(r:ReturnType<typeof random>,type:number,i:number) {
  const a=sample(r,type,i),u=sample(r,[0,1,4,8][i%4],i+3);
  const ap=player(a),up=player(u),[bp,sp]=pools(a,u,ap,up);
  // Default-type table uses the default bridge weapon, even though its Player
  // fixture derives from the available Gondi class.
  const def=charDef(type)??{...charDef(4),id:'neo',isMichiGichi:false};
  return tableSnapshot(B.buildPlayerTable(ap,up,def,bp,sp));
}
function context(level:DifficultyLevel,overrides:any={}) {
  const c:any=new AIController(level,charDef(4),charDef(1));
  return Object.assign(c.ctx,overrides);
}
function tsTable(a:any){return {...a,morphState:a.morphState??undefined,player:{}} as any;}
const tacticNames=['onceMethod','turnToAngle','turnAwayFromOp','playerThrustSpeed','fireWeapon2Once','playerThrustOnce',
 'fireWeapon1Energy','fireWeapon2Energy','fire','slowDown','stayAtADistanceFromSun','stayAtADistance','stayAtADistanceFromEntity',
 'flyToPoint','flyToPointNStop','flyInCircle','numOfShotsInAir','getClosestPlasma','getClosestSnot','isActiveIceShield',
 'getNumberOfSnotsStuck','facingAndShooting','avoidBluePlasma','avoidSnot','avoidIceSholev','shootWhenSnotsActive','snotsStuckOnAIAction',
 'superFastAndShoot','zigzag','turnAndShoot','zigZagShoot','uniqueTactic','cowardFlying','lowEnergyAttack','closeRangeAttack',
 'inDisadvantage','inAndOut','stationaryAttack','kamikazeAttack','sitOnTailAttack'];
function tactic(c:any,level:DifficultyLevel) {
  const a=tsTable(c.ai),u=tsTable(c.user),out=Object.assign(new AIOutput(),c.out),ctx=context(level,c.ctx);ctx.out=out;
  const p=c.parameter??150,fn=(T as any)[c.name];let result:any;
  switch(c.name) {
    case 'onceMethod':case 'fireWeapon2Once':case 'playerThrustOnce': result=fn(out,ctx);break;
    case 'turnToAngle':case 'turnAwayFromOp':case 'playerThrustSpeed':result=fn(a,p,out);break;
    case 'fireWeapon1Energy':case 'fireWeapon2Energy':result=fn(a,out,ctx);break;
    case 'fire':result=fn(a,u,c.weapon??1,c.shootTime??1000,c.shootInterval??25,c.timeToRest??10,c.aim??true,out,ctx);break;
    case 'slowDown':result=fn(a,p,out,ctx);break;
    case 'stayAtADistanceFromSun':result=fn(a,ctx.sunTable,p,out,ctx);break;
    case 'stayAtADistance':result=fn(a,u,p,out);break;
    case 'stayAtADistanceFromEntity':result=fn(a,u.x,u.y,p,out);break;
    case 'flyToPoint':result=fn(a,u.x,u.y,out);break;
    case 'flyToPointNStop':result=fn(a,u.x,u.y,out,ctx);break;
    case 'flyInCircle':result=fn(a,u,p,out,ctx);break;
    case 'numOfShotsInAir':case 'isActiveIceShield':case 'getNumberOfSnotsStuck':result=fn(a);break;
    case 'getClosestPlasma':case 'getClosestSnot':result=fn(a,u);break;
    case 'facingAndShooting':result=fn(a,u,ctx);break;
    case 'zigZagShoot':result=fn(a,u,c.stop??false,out,ctx);break;
    default:result=fn(a,u,out,ctx);
  }
  return {out:plain(out),ctx:ctxSnapshot(ctx),result:result??null};
}

describe('faithful Rust AI against unmodified upstream TypeScript',()=>{
  for(const level of [1,2,3] as const)for(const own of [1,4,8])for(const enemy of [1,4,8]){
    it(`controller difficulty ${level}, ship ${own} vs ${enemy}: state/learning/timers/bridge/RNG`,async()=>{
      await new Promise(resolve=>setTimeout(resolve,0));
      const inputs=random(0xa511+level*100+own*10+enemy),seed=0xafe123+level*111+own*17+enemy;
      const frames=Array.from({length:1200},(_,i)=>({ai:sample(inputs,own,i),user:sample(inputs,enemy,i+13),reset:i===800}));
      // Sustained safe state allows normal tactic dispatch; random states cover
      // priorities, hurt/snot/plasma/morph/energy cases and short-circuit draws.
      for(let i=100;i<650;i++) {
        frames[i].ai={...frames[i].ai,position:{x:1000,y:800},health:1,maxSpeed:.2,accelVec:{x:0,y:0},shots:[],snots:[]};
        frames[i].user={...frames[i].user,position:{x:1300,y:800},health:.6,maxSpeed:.2,accelVec:{x:0,y:0},shots:[],snots:[]};
      }
      const r=random(seed),mock=vi.spyOn(Math,'random').mockImplementation(r.next);
      let expected:any;
      try{
        const c:any=new AIController(level,charDef(own),charDef(enemy));
        const trace=frames.map(f=>{
          if(f.reset)c.reset();const a=player(f.ai),u=player(f.user),[bp,sp]=pools(f.ai,f.user,a,u);
          const at=B.buildPlayerTable(a,u,charDef(own),bp,sp),ut=B.buildPlayerTable(u,a,charDef(enemy),bp,sp);
          c.update(16.666666666666668,a,u,bp,sp);
          return {state:controllerSnapshot(c),ai:tableSnapshot(at),user:tableSnapshot(ut)};
        });expected={trace,randomState:r.state(),randomCalls:r.calls()};
      }finally{mock.mockRestore();}
      equal(native({kind:'controller',level,seed,frames}),expected);
      counters.controllerFrames+=frames.length;
    },60000);
  }
  for(const level of [1,2,3] as const){
    it(`attack/defense dispatch, learning selections and timer boundaries at difficulty ${level}`,async()=>{
      await new Promise(resolve=>setTimeout(resolve,0));
      const r=random(1399+level),cases:any[]=[];
      for(const attack of [false,true])for(const type of [0,1,4,8])for(let t=0;t<=21;t++)for(let i=0;i<8;i++) {
        const a=table(r,type,t+i),u=table(r,[0,1,4,8][i%4],t+i+2);
        Object.assign(a,{life:.8,speed:.8,maxSpeed:1,distance:600,x:1000,y:600});
        Object.assign(u,{shots:[],snots:[],maxSpeed:.2,life:.9,x:1500,y:600});
        if(i===5)a.life=.7;if(i===6)a.distance=100;if(i===7)u.shots=[{position:{x:1400,y:600},config:{tracking:false}}];
        const learning=new AILearning();learning.attackMaxTactic=19;learning.defenseMaxTactic=5;
        cases.push({attack,ai:a,user:u,mode:{frameCounter:[0,1,999,1000,1999,0,500,1000][i],whichTactic:t,lastFrameLife:.8,numOfTacticsPlayed:[1,3,4,5,6,10,11,12][i]},ctx:{num:i*500+1,tacticTime:1000,inDisAdvantage:i===7,learning}});
      }
      const seed=7721+level,rr=random(seed),mock=vi.spyOn(Math,'random').mockImplementation(rr.next);let expected:any;
      try{expected={trace:cases.map(c=>{
        const mode:any=Object.assign(c.attack?new AIAttackMode():new AIDefenseMode(),c.mode),ctx=context(level,c.ctx),out=new AIOutput();
        mode.update(tsTable(c.ai),tsTable(c.user),out,ctx);
        return {mode:plain(mode),out:plain(out),ctx:ctxSnapshot(ctx)};
      }),randomState:rr.state(),randomCalls:rr.calls()};}finally{mock.mockRestore();}
      equal(native({kind:'modes',level,seed,cases}),expected);counters.modeCases+=cases.length;
    },60000);
    it(`all 40 tactics/helpers and boundary states at difficulty ${level}`,async()=>{
      await new Promise(resolve=>setTimeout(resolve,0));
      const r=random(813103+level),cases:any[]=[];
      for(const name of tacticNames)for(const type of [0,1,4,8])for(let i=0;i<32;i++){
        const a=table(r,type,i),u=table(r,[0,1,4,8][i%4],i+1);
        if(i%4===0){u.x=a.x+25;u.y=a.y;a.distance=25;u.distance=25;}
        if(i%5===0){a.angle=a.angleToOp;u.angle=u.angleToOp;}
        if(i%7===0){a.x=50;a.y=0;a.speed=0;a.dxVector=0;a.dyVector=0;}
        if(i%8===0){a.maxSpeed=0.01;u.maxSpeed=0.01;}
        if(i%9===0){a.angle=M.goodAim(tsTable(a),tsTable(u),1);}
        // Deliberately nearby projectiles, with nearer nontracking/attached ones
        // first, verify filtering and short-circuit priority.
        u.shots.push({position:{x:a.x+1,y:a.y},config:{tracking:false}},{position:{x:a.x+2,y:a.y},config:{tracking:true}});
        u.snots.push({position:{x:a.x+1,y:a.y},attached:true},{position:{x:a.x+2,y:a.y},attached:false});
        cases.push({name,ai:a,user:u,parameter:[0,.01,.2,100,150,250,450,700][i%8],
          weapon:i%2+1,shootTime:i%2?500:1000,shootInterval:[0,1,25,150][i%4],timeToRest:i%3?10:0,aim:i%2===0,stop:i%2===0,
          out:{thrust:i%2===0,rotateLeft:i%3===0,rotateRight:i%3===1,firePrimary:i%2===1,fireSecondary:i%4===0},
          ctx:{num:[0,1,49,50,51,125,149,150,249,250,251,499,500,501,999,1000][i%16],
          fireWeapon2Flag:i%2===0,thrustOnceFlag:i%2===1,smallGitchiFlag:i%3===0,energyFillFlag:i%2===0,
          cowardReadyFlag:i%2===0,inLowEnergyAttack:i%3===0,slowDownSunFlag:i%5===0,inDisAdvantage:i%2===0,
          countShootTime:[0,1,25,500,1000,1001][i%6],readyToFire:i%3!==0,rotAngle:i*46,
          lastFrameCloseRange:i%2?0:999,closeRangeCounter:[0,199,200,201][i%4],disAdvantageCounter:[0,49,50,99][i%4],
          randomAngle:i*11,inAndOutdistance:i%3?400:100}});
      }
      const seed=121+level,rr=random(seed),mock=vi.spyOn(Math,'random').mockImplementation(rr.next);let expected:any;
      try{expected={trace:cases.map(c=>tactic(c,level)),randomState:rr.state(),randomCalls:rr.calls()};}finally{mock.mockRestore();}
      equal(native({kind:'tactics',level,seed,cases}),expected);counters.tacticCases+=cases.length;
    },60000);
    it(`geometry, predictive aim and range at difficulty ${level}`,async()=>{
      await new Promise(resolve=>setTimeout(resolve,0));
      const r=random(71311+level),cases=Array.from({length:3000},(_,i)=>({ai:table(r,[0,1,4,8][i%4],i),user:table(r,[8,4,1,0][i%4],i+2),n:[0,1,100,700,1000,5000][i%6]}));
      for(let i=0;i<cases.length;i+=9){cases[i].ai.x=cases[i].user.x;cases[i].ai.y=cases[i].user.y;cases[i].ai.dxVector=0;cases[i].ai.dyVector=0;}
      const seed=991+level,rr=random(seed),mock=vi.spyOn(Math,'random').mockImplementation(rr.next);let expected:any;
      try{expected={trace:cases.map(c=>{const a=tsTable(c.ai),b=tsTable(c.user);return {
        distance:M.getDistance(a,b),vectorSize:M.getVectorSize(a.x,a.y,b.x,b.y),angle:M.getAngleToXY(b.x,b.y,a.x,a.y),
        velocityAngle:M.getVVAngle(a.dxVector,a.dyVector),turn:M.turnToAngleDecision(a.angle,b.angle),
        aim:M.goodAim(a,b,getDifficultyConstants(level).goodAimProbability),range:M.realIsInRange(a,b),futureRange:M.isInFutureRange(a,b,c.n),
        speed:B.getSpeedOfShot(a.entityType,a.morphState),ttl:B.getMaxTtlOfShot(a.entityType,a.morphState),maxDistance:B.getMaxDistanceOfShot(a.entityType,a.morphState)};}),randomState:rr.state(),randomCalls:rr.calls()};}finally{mock.mockRestore();}
      equal(native({kind:'math',level,seed,cases}),expected);counters.mathCases+=cases.length;
    },60000);
  }
  it('learning records hit/loss history and preserves first-best ties',async()=>{
    await new Promise(resolve=>setTimeout(resolve,0));
    const r=random(931141),events=Array.from({length:4096},(_,i)=>({attack:i%2===0,tactic:Math.floor(r.next()*21),hit:r.next()>.3}));
    const learning=new AILearning(),trace=events.map(e=>{learning.updateTacticsTable(e.attack?'attack':'defense',e.tactic,e.hit);return plain(learning);});
    equal(native({kind:'learning',events}),{trace});counters.learningEvents=events.length;
  });
  it('reports exact exercised coverage and preserves a machine-readable result',()=>{
    expect(counters.controllerFrames).toBe(32400);expect(counters.tacticCases).toBe(15360);expect(counters.mathCases).toBe(9000);
    expect(counters.modeCases).toBe(4224);expect(counters.learningEvents).toBe(4096);
    mkdirSync(evidenceDir,{recursive:true});writeFileSync(resolve(evidenceDir,'results.json'),JSON.stringify({passed:true,...counters,levels:[1,2,3],ships:['michiGichi','gondi','sholev'],tactics:tacticNames,oracle:'unmodified upstream src/ai/*.ts',engine:executable},null,2)+'\n');
  });
});
