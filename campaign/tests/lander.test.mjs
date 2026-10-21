import test from 'node:test';
import assert from 'node:assert/strict';
import {createSurface,stepSurface,surfaceReceipt,SURFACE} from '../src/lander.mjs';

test('surface generation is seeded, independent, bounded and leaves every objective accessible',()=>{
    assert.deepEqual(createSurface({seed:'vesper'}),createSurface({seed:'vesper'}));
    assert.notDeepEqual(createSurface({seed:0}).hazards,createSurface({seed:1}).hazards);
    for(let seed=0;seed<300;seed++) {
        const s=createSurface({seed});assert.equal(s.beacons.length,3);assert.equal(s.ore.length,8);assert.equal(s.hazards.length,9);
        for(const h of s.hazards){assert.ok(Math.hypot(h.x-s.pad.x,h.y-s.pad.y)>115);assert.ok(s.beacons.every(b=>Math.hypot(h.x-b.x,h.y-b.y)>95));}
        for(const point of [...s.beacons,...s.hazards,...s.ore]){assert.ok(point.x>0&&point.x<SURFACE.width);assert.ok(point.y>0&&point.y<SURFACE.height);}
    }
});

test('fixed-step steering is deterministic across frame rates without mutating the input',()=>{
    const initial=createSurface({seed:42}),saved=structuredClone(initial);
    const coarse=stepSurface(initial,{x:1,y:-1},1);
    let fine=initial;for(let i=0;i<60;i++)fine=stepSurface(fine,{x:1,y:-1},1/60);
    for(const key of ['x','y','vx','vy','hull','battery','ticks'])assert.equal(coarse[key],fine[key]);
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

test('hazard damage has a recovery interval and hull destruction evacuates',()=>{
    const initial=createSurface({seed:9});initial.hazards=[{x:initial.x,y:initial.y,radius:30}];
    const hit=stepSurface(initial,{},1/60);assert.equal(hit.hull,86);
    const protectedState=stepSurface(hit,{},.5);assert.equal(protectedState.hull,86);
    const nextHit=stepSurface(protectedState,{},1.1);assert.equal(nextHit.hull,72);
    const wreck=stepSurface(initial,{},20);assert.equal(wreck.outcome,'damaged');assert.equal(surfaceReceipt(wreck).hullRemaining,0);
    assert.equal(stepSurface(wreck,{retreat:true},1),wreck,'a finished result cannot change');
});

test('ore must be physically collected and each deposit pays once within a hard cap',()=>{
    const initial=createSurface({seed:33});initial.hazards=[];
    initial.ore=[{x:initial.x+100,y:initial.y,value:3,collected:false}];
    assert.equal(stepSurface(initial,{},1).salvage,0);
    const reached=stepSurface(initial,{x:1},1);assert.equal(reached.salvage,3);assert.equal(reached.ore[0].collected,true);
    assert.equal(stepSurface(reached,{},1).salvage,3);
    const full={...initial,salvage:23,ore:[{x:initial.x,y:initial.y,value:3,collected:false}]};
    assert.equal(stepSurface(full,{},1/60).salvage,SURFACE.maxSalvage);
});

function steerTo(state,target) {
    let s=state;
    for(let frame=0;frame<1800&&!s.outcome;frame++) {
        const dx=target.x-s.x,dy=target.y-s.y,distance=Math.hypot(dx,dy);
        if(distance<18)return s;
        const speed=Math.min(145,distance*3),vx=dx/distance*speed,vy=dy/distance*speed;
        s=stepSurface(s,{x:(vx*4.3+(vx-s.vx)*8)/680,y:(vy*4.3+(vy-s.vy)*8)/680},1/60);
    }
    return s;
}

test('an entire expedition can be completed through movement, with return required after all scans',()=>{
    let s=createSurface({seed:77,id:'survey:77'});assert.equal(surfaceReceipt(s),null);
    for(const beacon of s.beacons)s=steerTo(s,beacon);
    assert.equal(s.beacons.filter(b=>b.visited).length,3);assert.equal(s.outcome,null,'scanning the last beacon is not yet a return');
    s=steerTo(s,s.pad);assert.equal(s.outcome,'surveyed');
    const receipt=surfaceReceipt(s);assert.equal(receipt.id,'survey:77');assert.equal(receipt.beacons,3);assert.ok(receipt.hullRemaining>0);assert.ok(s.battery>30);
    assert.ok(Object.isFrozen(receipt));
});

test('partial surveys never succeed; retreat and battery exhaustion preserve collected ore',()=>{
    let s=createSurface({seed:77,id:'partial'});s=steerTo(s,s.beacons[0]);s=steerTo(s,s.pad);
    assert.equal(s.outcome,null);assert.equal(s.beacons.filter(b=>b.visited).length,1);
    const recalled=surfaceReceipt(stepSurface(s,{retreat:true},0));assert.equal(recalled.outcome,'retreated');assert.equal(recalled.beacons,1);assert.equal(recalled.id,'partial');
    const timed=stepSurface({...createSurface({seed:1}),salvage:6},{},120);
    assert.deepEqual(surfaceReceipt(timed),{outcome:'retreated',salvage:6,hullRemaining:100,beacons:0});assert.equal(timed.battery,0);
});
