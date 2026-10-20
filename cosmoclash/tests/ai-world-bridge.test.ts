// @vitest-environment node
import {describe,it,expect,vi} from 'vitest';
import {spawnSync} from 'node:child_process';
import {resolve} from 'node:path';
import {mkdirSync,writeFileSync} from 'node:fs';
import {AIController} from '../src/ai/AIController';
import {buildPlayerTable,getAIEntityType} from '../src/ai/AIBridge';
import {CHARACTER_REGISTRY} from '../src/game/CharacterRegistry';
import {Player,GONDI_STATS} from '../src/game/Player';
import {MichiGichiPlayer,MICHI_STATS,GICHI_STATS} from '../src/game/MichiGichiPlayer';
import {SholevPlayer} from '../src/game/SholevPlayer';
import {Bullet,FLAME_BULLET_CONFIG,PLASMA_BULLET_CONFIG} from '../src/game/Bullet';
import {Snot} from '../src/game/Snot';
import {EntityPool} from '../src/engine/EntityPool';

const exe=process.env.COSMO_AI_ORACLE??resolve('rust/target/debug/cosmo-jsonl');
const plain=(v:any)=>JSON.parse(JSON.stringify(v));
function compare(a:any,b:any,path='root'):void {
  if(typeof b==='number') {if(typeof a!=='number'||Math.abs(a-b)>1e-11)throw new Error(`${path}: ${a} != ${b}`);}
  else if(Array.isArray(b)){expect(a.length,path).toBe(b.length);b.forEach((v,i)=>compare(a[i],v,`${path}[${i}]`));}
  else if(b!==null&&typeof b==='object'){expect(Object.keys(a).sort(),path).toEqual(Object.keys(b).sort());for(const k of Object.keys(b))compare(a[k],b[k],`${path}.${k}`);}
  else expect(a,path).toBe(b);
}
function normalizedTable(t:any) {
  const {player,...rest}=t;return plain({...rest,morphState:t.morphState??null,
    shots:t.shots.map((b:any)=>({position:{x:b.position.x,y:b.position.y},config:{tracking:!!b.config.tracking}})),
    snots:t.snots.map((s:any)=>({position:{x:s.position.x,y:s.position.y},attached:s.attached}))});
}
function normalizedController(c:any) {
  const {out,...ctx}=c.ctx;return plain({level:c.level,num:c.num,out:c.out,attackMode:c.attackMode,defenseMode:c.defenseMode,ctx,lastAILife:c.lastAILife,lastUserLife:c.lastUserLife});
}
function makePlayer(s:any) {
  const p=s.shipId==='michiGichi'?new MichiGichiPlayer():s.shipId==='sholev'?new SholevPlayer():new Player(GONDI_STATS);
  // Use the real source stats/effective-speed implementation, never copy the
  // Rust-computed speed into the TS reference.
  if(p instanceof MichiGichiPlayer){p.morphState=s.morphState;p.applyStats(s.morphState>=2?GICHI_STATS:MICHI_STATS);}
  if(p instanceof SholevPlayer&&s.frostShieldActive)p.activateFrostShield();
  p.position={...s.position};p.previousPosition={...s.previousPosition};p.angle=s.angle;p.accelVec={...s.accelVec};
  p.health=s.health;p.energy=s.energy;p.snotCount=s.snotCount;return p;
}
describe('production World AI bridge against upstream live Player and EntityPool',()=>{
  for(const level of [1,2,3] as const)it(`difficulty ${level}: owner, dead projectile, morph, shield and speed mapping`,async()=>{
    await new Promise(resolve=>setTimeout(resolve,0));
    const cases=[];
    for(const p1 of ['michiGichi','gondi','sholev'])for(const p2 of ['michiGichi','gondi','sholev'])for(let variant=0;variant<16;variant++)cases.push({ships:[p1,p2],variant,seed:994+variant});
    const child=spawnSync(exe,[],{input:JSON.stringify({op:'ai_oracle',kind:'worldBridge',level,cases})+'\n',encoding:'utf8',timeout:60000,maxBuffer:128*1024*1024});
    expect(child.status,child.stderr).toBe(0);const result=JSON.parse(child.stdout);expect(result.error).toBeUndefined();
    let deadBullets=0,deadSnots=0;
    for(const record of result.trace){
      const initial=record.before,players=initial.players.map(makePlayer);
      const definitions=initial.players.map((p:any)=>CHARACTER_REGISTRY.find(d=>d.id===p.shipId)!);
      const owners=new Map<number,Player>(initial.players.map((p:any,i:number)=>[p.id,players[i]]));owners.set(999,new Player(GONDI_STATS));
      const bullets=new EntityPool(()=>new Bullet(FLAME_BULLET_CONFIG),()=>{},0);
      const snots=new EntityPool(()=>new Snot(players[0],players[1]),()=>{},0);
      for(const b of initial.bullets){
        const entity=bullets.acquire();entity.config=b.kind==='plasma'?PLASMA_BULLET_CONFIG:FLAME_BULLET_CONFIG;
        entity.originEntity=owners.get(b.originEntity)??null;entity.position={...b.position};entity.alive=b.alive;
        if(!b.alive&&owners.has(b.originEntity)&&b.originEntity!==999)deadBullets++;
      }
      for(const s of initial.snots){const entity=snots.acquire();entity.originEntity=owners.get(s.originEntity)??null;
        entity.position={...s.position};entity.attached=s.attached;entity.alive=s.alive;
        if(!s.alive&&owners.has(s.originEntity)&&s.originEntity!==999)deadSnots++;
      }
      // Active pool membership deliberately includes objects killed earlier in
      // this frame. Adding an alive filter to World's bridge would fail here.
      for(let i=0;i<2;i++){
        const p=players[i],def=definitions[i],table=buildPlayerTable(p,players[1-i],def,bullets,snots),t=normalizedTable(table);
        compare(record.tables[i],t,`table ${i}`);
        compare(record.players[i],{position:{x:p.position.x,y:p.position.y},angle:p.angle,accelVec:{x:p.accelVec.x,y:p.accelVec.y},
          health:p.health,energy:p.energy,maxSpeed:p.getEffectiveMaxSpeed(),snotCount:p.snotCount,entityType:getAIEntityType(def),
          morphState:p instanceof MichiGichiPlayer?p.morphState:null,shots:t.shots,snots:t.snots},`player ${i}`);
      }
      let state=initial.rng.state>>>0;
      const mock=vi.spyOn(Math,'random').mockImplementation(()=>{state^=state<<13;state^=state>>>17;state^=state<<5;return(state>>>0)/4294967296;});
      try{
        const ai:any=new AIController(level,definitions[1],definitions[0]);
        for(let i=0;i<record.updates.length;i++){
          const output=ai.update(1000/60,players[1],players[0],bullets,snots);
          expect(record.updates[i].output).toEqual(output);
          expect(record.updates[i].controller).toEqual(normalizedController(ai));
          expect(record.updates[i].rng.state).toBe(state>>>0);
        }
      }finally{mock.mockRestore();}
    }
    expect(deadBullets).toBeGreaterThan(0);expect(deadSnots).toBeGreaterThan(0);
    const path=resolve(process.env.COSMO_AI_EVIDENCE_DIR??'test-results/ai-parity');mkdirSync(path,{recursive:true});
    writeFileSync(resolve(path,`world-bridge-${level}.json`),JSON.stringify({passed:true,cases:cases.length,controllerUpdates:cases.length*12,deadBullets,deadSnots,level,engine:exe},null,2)+'\n');
  },60000);
  it('World.step feeds the AI the first player’s newly fired shot in the same frame',()=>{
    const requests=(items:any[])=>{
      const r=spawnSync(exe,[],{input:items.map(x=>JSON.stringify(x)).join('\n')+'\n',encoding:'utf8',maxBuffer:8*1024*1024,timeout:10000});
      expect(r.status,r.stderr).toBe(0);return r.stdout.trim().split('\n').map(line=>JSON.parse(line));
    };
    const initial=requests([{op:'init',seed:8731,ships:['gondi','sholev'],aiDifficulty:'hard'}])[0].snapshot;
    initial.asteroids=[];initial.bullets=[];initial.snots=[];
    initial.players.forEach((p:any,i:number)=>{
      p.position={x:700,y:700+i*100,z:0};p.previousPosition={...p.position};p.accelVec={x:0,y:0,z:0};
      p.angle=i===0?0:180;p.health=1;p.energy=1;p.snotCount=0;p.primaryCooldown=0;
    });
    initial.aiController.num=5;initial.aiController.ctx.num=5;
    const [a,b]=initial.players.map(makePlayer),dt=1000/60;
    const defs=['gondi','sholev'].map(id=>CHARACTER_REGISTRY.find(d=>d.id===id)!);
    const c:any=new AIController(3,defs[1],defs[0]);c.num=5;c.ctx.num=5;
    // Real source first-player input calls energy refill and fire before AI.
    a.updateEnergy(dt);const shot=a.firePrimary()!;expect(shot).not.toBeNull();
    const bullets=new EntityPool(()=>shot,()=>{},0);bullets.acquire();
    const snots=new EntityPool(()=>new Snot(a,b),()=>{},0);
    let state=initial.rng.state>>>0;
    const mock=vi.spyOn(Math,'random').mockImplementation(()=>{state^=state<<13;state^=state>>>17;state^=state<<5;return(state>>>0)/4294967296;});
    try{c.update(dt,b,a,bullets,snots);}finally{mock.mockRestore();}
    // This guard specifically requires the newly fired shot. Calling the AI
    // before player-one input leaves disAdvantageCounter at zero instead.
    expect(c.ctx.disAdvantageCounter).toBe(1);
    const empty={thrust:false,rotateLeft:false,rotateRight:false,firePrimary:false,fireSecondary:false};
    const actual=requests([{op:'testLoad',snapshot:initial},{op:'step',dt,inputs:[{...empty,firePrimary:true},empty]}])[1];
    expect(actual.ok).toBe(true);expect(actual.snapshot.aiController).toEqual(normalizedController(c));
    expect(actual.snapshot.rng.state).toBe(state>>>0);
    const path=resolve(process.env.COSMO_AI_EVIDENCE_DIR??'test-results/ai-parity');mkdirSync(path,{recursive:true});
    writeFileSync(resolve(path,'world-input-order.json'),JSON.stringify({passed:true,firstPlayerShotVisibleSameFrame:true,engine:exe},null,2)+'\n');
  });
});
