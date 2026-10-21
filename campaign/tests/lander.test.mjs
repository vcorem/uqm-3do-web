import test from 'node:test';
import assert from 'node:assert/strict';
import {createSurface,stepSurface,surfaceReceipt,surfaceObjective,SURFACE,SURFACE_MODES} from '../src/lander.mjs';
import {surfaceWaypoints,steerSurface} from './surface-driver.mjs';

// Feedback steering: directional input only. It brakes at objectives and lets
// the actual simulation decide when scanning, boarding and unloading complete.
function steer(state,target){
    const dx=target.x-state.x,dy=target.y-state.y,d=Math.hypot(dx,dy),speed=Math.min(140,d*3),vx=d?dx/d*speed:0,vy=d?dy/d*speed:0;
    return {x:(vx*4.3+(vx-state.vx)*8)/680,y:(vy*4.3+(vy-state.vy)*8)/680};
}
function advanceTo(state,target,predicate,seconds=30){
    let s=state;for(let frame=0;frame<seconds*60&&!s.outcome&&!predicate(s);frame++)s=stepSurface(s,steer(s,target));return s;
}
function expedition(state){
    let s=state,key=null,path=[];
    for(let frame=0;frame<state.batteryCapacity*60+1&&!s.outcome;frame++){
        const goal=surfaceObjective(s),nextKey=goal.kind+':'+(goal.target.id??'pad');
        if(nextKey!==key){key=nextKey;path=surfaceWaypoints(s,goal.target);}
        while(path.length>1&&Math.hypot(path[0].x-s.x,path[0].y-s.y)<14)path.shift();
        s=stepSurface(s,steerSurface(s,path[0]));
    }return s;
}

test('surface generation is seeded, bounded and protects objectives from every moving hazard',()=>{
    assert.deepEqual(createSurface({seed:'vesper'}),createSurface({seed:'vesper'}));
    assert.notDeepEqual(createSurface({seed:0}).hazards,createSurface({seed:1}).hazards);
    const layouts=new Set();
    for(const mode of SURFACE_MODES)for(let seed=0;seed<150;seed++){
        const s=createSurface({seed,mode});assert.equal(s.beacons.length,3);assert.equal(s.ore.length,8);assert.ok(s.hazards.length>=4);
        layouts.add(`${s.pad.x},${s.pad.y}`);
        for(const h of s.hazards){assert.ok(Math.hypot(h.baseX-s.pad.x,h.baseY-s.pad.y)>h.radius+h.amplitude+100);assert.ok(s.beacons.every(b=>Math.hypot(h.baseX-b.x,h.baseY-b.y)>h.radius+h.amplitude+85));}
        for(const point of [...s.beacons,...s.hazards,...s.ore]){assert.ok(point.x>0&&point.x<SURFACE.width);assert.ok(point.y>0&&point.y<SURFACE.height);}
    }
    assert.ok(layouts.size>=5,'the landing geometry must genuinely vary');
});

test('fixed-step steering and hazard motion are deterministic across frame rates and do not mutate input',()=>{
    const initial=createSurface({seed:42,mode:'relay'}),saved=structuredClone(initial);
    const coarse=stepSurface(initial,{x:1,y:-1},1);
    let fine=initial;for(let i=0;i<60;i++)fine=stepSurface(fine,{x:1,y:-1},1/60);
    for(const key of ['x','y','vx','vy','hull','battery','ticks'])assert.equal(coarse[key],fine[key]);assert.deepEqual(coarse.hazards,fine.hazards);
    assert.deepEqual(initial,saved);assert.ok(coarse.x>initial.x&&coarse.y<initial.y);
    const coast=stepSurface(coarse,{},1);assert.ok(Math.hypot(coast.vx,coast.vy)<Math.hypot(coarse.vx,coarse.vy));
    const horizontal=stepSurface(initial,{x:1},1);assert.ok(Math.abs(Math.hypot(coarse.vx,coarse.vy)-Math.hypot(horizontal.vx,horizontal.vy))<1e-8);
    assert.throws(()=>stepSurface(initial,{},NaN),RangeError);assert.throws(()=>stepSurface(initial,{},-1),RangeError);
});

test('map edges stop movement without trapping or damaging the lander',()=>{
    const initial={...createSurface({seed:22}),hazards:[]};
    const edge=stepSurface(initial,{x:-1,y:1},20);
    assert.equal(edge.x,SURFACE.radius);assert.equal(edge.y,SURFACE.height-SURFACE.radius);assert.equal(edge.hull,100);
    const away=stepSurface(edge,{x:1,y:-1},1);assert.ok(away.x>edge.x&&away.y<edge.y);
});

test('hazard damage has a recovery interval; shielding and battery upgrades have real effects',()=>{
    const initial=createSurface({seed:9});initial.hazards=[{x:initial.x,y:initial.y,radius:30,active:true}];
    const hit=stepSurface(initial,{},1/60);assert.equal(hit.hull,87);
    assert.equal(stepSurface(hit,{},.5).hull,87);assert.equal(stepSurface(hit,{},1.7).hull,74);
    const shielded=stepSurface({...initial,shielding:50},{},1/60);assert.equal(shielded.hull,93.5);
    const wreck=stepSurface(initial,{},20);assert.equal(wreck.outcome,'damaged');assert.equal(surfaceReceipt(wreck).hullRemaining,0);
    assert.equal(stepSurface(wreck,{retreat:true},1),wreck,'a finished result cannot change');
    assert.equal(createSurface({mode:'rescue',batteryBonus:45}).batteryCapacity,225);
    assert.equal(createSurface({shielding:1000}).shielding,75);
    assert.equal(createSurface({mode:'unknown',difficulty:'unknown'}).mode,'survey');
});

test('ore must be physically collected and each deposit pays once within a hard cap',()=>{
    const initial=createSurface({seed:33});initial.hazards=[];
    initial.ore=[{x:initial.x+100,y:initial.y,value:3,collected:false}];
    assert.equal(stepSurface(initial,{},1).salvage,0);
    const reached=stepSurface(initial,{x:1},1);assert.equal(reached.salvage,3);assert.equal(reached.ore[0].collected,true);
    assert.equal(stepSurface(reached,{},1).salvage,3);
    const full={...initial,salvage:23,ore:[{x:initial.x,y:initial.y,value:3,collected:false}]};assert.equal(stepSurface(full,{},1/60).salvage,SURFACE.maxSalvage);
});

test('mapping requires stopping for a scan and retains completed work when leaving',()=>{
    let s=createSurface({seed:77,mode:'survey'});const target=s.beacons[0];
    s=advanceTo(s,target,state=>Math.hypot(state.x-target.x,state.y-target.y)<15);
    assert.equal(s.beacons[0].visited,false,'simply touching the marker is insufficient');
    s=advanceTo(s,target,state=>state.beacons[0].progress>=1);
    assert.ok(s.beacons[0].progress>=1&&s.beacons[0].progress<3.2);
    const progress=s.beacons[0].progress;s=stepSurface(s,{x:1},1);assert.ok(s.beacons[0].progress>=progress);
    s=advanceTo(s,target,state=>state.beacons[0].visited);assert.equal(s.beacons[0].visited,true);assert.equal(s.outcome,null);
});

test('recovery has one-slot cargo, a slower loaded lander and secured deliveries only at home',()=>{
    let s=createSurface({seed:11,mode:'rescue'});const first=s.beacons[0],second=s.beacons[1];
    s=advanceTo(s,first,state=>state.cargo===0);assert.equal(s.cargo,0);assert.equal(s.beacons[0].visited,false);
    assert.equal(surfaceObjective(s).kind,'deliver');
    s=advanceTo(s,second,state=>Math.hypot(state.x-second.x,state.y-second.y)<10);s=stepSurface(s,{},2);
    assert.equal(s.cargo,0);assert.equal(s.beacons[1].carried,false,'cannot collect a second group before unloading');
    const empty=createSurface({seed:11,mode:'rescue'}),loaded={...empty,cargo:0};
    assert.ok(stepSurface(loaded,{x:1},1).vx<stepSurface(empty,{x:1},1).vx*.85);
    s=advanceTo(s,s.pad,state=>state.beacons[0].visited);assert.equal(s.beacons[0].visited,true);assert.equal(s.cargo,null);assert.equal(s.outcome,null);
});

test('relay stations enforce sequence and open windows, while completed connections stay live',()=>{
    let s=createSurface({seed:14,mode:'relay'});const second=s.beacons[1];
    s=advanceTo(s,second,state=>Math.hypot(state.x-second.x,state.y-second.y)<10);s=stepSurface(s,{},6);
    assert.equal(s.beacons[1].progress,0,'station two cannot be activated before one');
    s=advanceTo(s,s.beacons[0],state=>state.beacons[0].visited,25);assert.equal(s.beacons[0].visited,true);
    assert.equal(surfaceObjective(s).target.id,1);s=stepSurface(s,{},10);assert.equal(s.beacons[0].visited,true,'links do not expire');
    const blocked=createSurface({seed:14,mode:'relay'});blocked.x=blocked.beacons[0].x;blocked.y=blocked.beacons[0].y;blocked.beacons[0].windowPhase=5;
    assert.equal(stepSurface(blocked,{},1).beacons[0].progress,0,'closed window does not charge');
    assert.ok(stepSurface(blocked,{},4).beacons[0].progress>0,'staying safely inside will catch the next window');
});

test('all three modes are completable through directional input across layouts, seeds and difficulty',()=>{
    const times={};
    for(const mode of SURFACE_MODES){
        times[mode]=[];
        for(const difficulty of ['gentle','standard','hazardous'])for(let seed=0;seed<40;seed++){
            const initial=createSurface({seed,mode,difficulty,id:`${mode}:${seed}`});assert.equal(surfaceReceipt(initial),null);
            const s=expedition(initial),receipt=surfaceReceipt(s);
            assert.equal(s.outcome,'surveyed',`${mode}/${difficulty}/${seed}: hull=${s.hull} battery=${s.battery}`);
            assert.equal(receipt.beacons,3);assert.equal(receipt.id,`${mode}:${seed}`);assert.ok(receipt.hullRemaining>0);assert.ok(receipt.salvage<=24);assert.ok(s.battery>20);assert.ok(Object.isFrozen(receipt));
            times[mode].push(s.ticks/60);
        }
    }
    // Recovery really requires multiple trips, rather than reskinning one route.
    assert.ok(times.rescue.reduce((a,b)=>a+b)/times.rescue.length>times.survey.reduce((a,b)=>a+b)/times.survey.length);
});

test('partial work never reports success; retreat and expiry preserve bounded collected ore',()=>{
    let s=createSurface({seed:77,id:'partial'});s=advanceTo(s,s.beacons[0],state=>state.beacons[0].visited);s=advanceTo(s,s.pad,state=>Math.hypot(state.x-state.pad.x,state.y-state.pad.y)<20);
    assert.equal(s.outcome,null);assert.equal(s.beacons.filter(b=>b.visited).length,1);
    const recalled=surfaceReceipt(stepSurface(s,{retreat:true},0));assert.equal(recalled.outcome,'retreated');assert.equal(recalled.beacons,1);assert.equal(recalled.id,'partial');
    const initial=createSurface({seed:1});const timed=stepSurface({...initial,salvage:6},{},initial.batteryCapacity);
    assert.deepEqual(surfaceReceipt(timed),{outcome:'retreated',salvage:6,hullRemaining:100,beacons:0});assert.equal(timed.battery,0);
    const partialRescue=createSurface({seed:1,mode:'rescue'});partialRescue.cargo=0;partialRescue.beacons[0].carried=true;
    assert.equal(surfaceReceipt(stepSurface(partialRescue,{retreat:true},0)).beacons,0,'onboard cargo is not falsely counted as delivered');
});


test('secured rescue deliveries survive recall and retry without duplicating or inventing tasks',()=>{
    let s=createSurface({seed:31,mode:'rescue'});s=advanceTo(s,s.beacons[1],state=>state.cargo===1);s=advanceTo(s,s.pad,state=>state.beacons[1].visited);
    const receipt=surfaceReceipt(stepSurface(s,{retreat:true},0));assert.deepEqual(receipt.completedTasks,[1]);assert.equal(receipt.beacons,1);assert.ok(Object.isFrozen(receipt.completedTasks));
    const resumed=createSurface({seed:31,mode:'rescue',completedTasks:receipt.completedTasks});assert.equal(resumed.beacons[1].visited,true);assert.equal(surfaceObjective(resumed).kind,'pickup');
    const done=expedition(resumed);assert.equal(done.outcome,'surveyed');assert.deepEqual(surfaceReceipt(done).completedTasks,[0,1,2]);
    const invalid=createSurface({mode:'rescue',completedTasks:[1,1,4,-1,'2',null]});assert.deepEqual(invalid.beacons.filter(b=>b.visited).map(b=>b.id),[1]);
    assert.equal(createSurface({mode:'survey',completedTasks:[0,1]}).beacons.some(b=>b.visited),false,'mapping data still requires returning home');
});


test('authored objective names identify cargo, scans and relays without inventing stranded people',()=>{
    const labels=['Beacon locker','Survival capsule','Long-range battery'];
    let rescue=createSurface({mode:'rescue',seed:31,objectiveLabels:labels});assert.deepEqual(rescue.objectiveLabels,labels);assert.notEqual(rescue.objectiveLabels,labels);
    assert.deepEqual(rescue.beacons.map(b=>b.label),labels);assert.equal(surfaceObjective(rescue).label,`Recover ${surfaceObjective(rescue).target.label}`);
    rescue=advanceTo(rescue,rescue.beacons[0],state=>state.cargo===0);assert.equal(surfaceObjective(rescue).label,'Deliver Beacon locker to the landing pad');
    assert.match(surfaceObjective(createSurface({mode:'survey',objectiveLabels:['Receiver','Transit marker','Dock light']})).label,/^Map (Receiver|Transit marker|Dock light)$/);
    assert.equal(surfaceObjective(createSurface({mode:'relay',objectiveLabels:['Carrier dish','Clock gate','Return channel']})).label,'Connect Carrier dish');
    const fallback=createSurface({mode:'rescue',objectiveLabels:['  Valid name  ',null,'']});assert.deepEqual(fallback.objectiveLabels,['Valid name','Recovery target 2','Recovery target 3']);
    assert.equal(createSurface({objectiveLabels:['a'.repeat(100)]}).beacons[0].label.length,80);
    assert.equal(createSurface({mode:'rescue',objectiveLabels:'abc'}).beacons[0].label,'Recovery target 1');
});
