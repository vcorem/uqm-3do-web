import test from 'node:test';
import assert from 'node:assert/strict';
import {readFile} from 'node:fs/promises';
import {SHIPS,validateEncounter,UQMCombat} from '../src/combat.mjs';

const valid = () => ({id:'encounter-1',player:{ship:5,crew:18},enemies:[19,24],seed:0,difficulty:'standard'});
test('All 25 campaign hull IDs match the actual original Melee ordering',async()=>{
    const header=await readFile(new URL('../../sc2/src/uqm/supermelee/meleeship.h',import.meta.url),'utf8');
    const ids=[...header.matchAll(/^\s*MELEE_([A-Z_]+),/gm)].map(match=>match[1]);
    assert.equal(ids.length,25);assert.equal(SHIPS.length,25);
    for(const [index,name] of ids.entries()) {
        const actual=SHIPS[index].name.split(' ')[0].toUpperCase().replaceAll('-','_');
        const expected=name==='EARTHLING'?'EARTHLING':name;
        assert.equal(actual.replaceAll('_',''),expected.replaceAll('_',''));
    }
});

test('Parent combat handles cancellation, overlap, origin filtering and invalid native results',async()=>{
    const originals=Object.fromEntries(['window','document','location'].map(key=>[key,globalThis[key]]));
    const messages=[];
    globalThis.location=new URL('http://localhost:9000/campaign/');
    globalThis.window=new EventTarget();
    globalThis.document={createElement:()=>({style:{},hidden:false,contentWindow:{postMessage:(data,origin)=>messages.push({data,origin})},remove(){}})};
    const container={append(){}};
    const created=[];
    const make=()=>{const instance=new UQMCombat({container});created.push(instance);return instance;};
    const ready=instance=>instance.receive({origin:location.origin,source:instance.frame.contentWindow,data:{type:'uqm-campaign:ready',version:1}});
    const settle=()=>new Promise(resolve=>setTimeout(resolve,0));
    try{
        const loading=make(), cancelled=loading.start(valid());loading.retreat();
        await assert.rejects(cancelled,{name:'AbortError'});
        ready(loading);await settle();assert.equal(messages.length,0,'Cancelled preload must not launch later');
        const retry=make(), first=retry.start(valid());retry.retreat();await assert.rejects(first,{name:'AbortError'});
        const sameId=retry.start(valid());ready(retry);await settle();assert.equal(messages.length,1,'Reusing a cancelled ID must queue exactly once');
        retry.receive({origin:location.origin,source:retry.frame.contentWindow,data:{type:'uqm-campaign:cancelled',id:'encounter-1'}});
        await assert.rejects(sameId,{name:'AbortError'});messages.length=0;
        const active=make(), result=active.start(valid());
        await assert.rejects(active.start({...valid(),id:'overlap'}),/already active/);
        ready(active);await settle();assert.equal(messages.length,1);
        const payload={type:'uqm-campaign:result',id:'encounter-1',outcome:'victory',crew:12,enemyRemaining:0,frames:40};
        active.receive({origin:'https://foreign.example',source:active.frame.contentWindow,data:payload});
        active.receive({origin:location.origin,source:{},data:payload});assert.ok(active.active);
        active.receive({origin:location.origin,source:active.frame.contentWindow,data:payload});
        assert.deepEqual(await result,{id:'encounter-1',outcome:'victory',crew:12,enemyRemaining:0,frames:40});
        const invalid=active.start({...valid(),id:'next'});await settle();
        active.receive({origin:location.origin,source:active.frame.contentWindow,data:{...payload,id:'next',crew:0}});
        await assert.rejects(invalid,/Invalid native combat result/);
        assert.equal(active.active,null);assert.equal(active.ready,false);
        const beforeLaunch=make(), pending=beforeLaunch.start(valid());ready(beforeLaunch);await settle();
        beforeLaunch.receive({origin:location.origin,source:beforeLaunch.frame.contentWindow,data:{type:'uqm-campaign:cancelled',id:'encounter-1'}});
        await assert.rejects(pending,{name:'AbortError'});assert.equal(beforeLaunch.frame.hidden,true);
    }finally{
        for(const instance of created)instance.destroy();
        for(const [key,value] of Object.entries(originals)){if(value===undefined)delete globalThis[key];else globalThis[key]=value;}
    }
});
test('Encounter validation rejects impossible crew and malformed fleet/seed',()=>{
    assert.deepEqual(validateEncounter(valid()),valid());
    for(const request of [null,{}, {...valid(),id:''},{...valid(),player:{ship:25,crew:1}},
        {...valid(),player:{ship:5,crew:19}},{...valid(),player:{ship:5,crew:0}},
        {...valid(),enemies:[]},{...valid(),enemies:Array(15).fill(1)},
        {...valid(),enemies:[25]},{...valid(),seed:-1},{...valid(),seed:2**32},
        {...valid(),difficulty:'cheat'}])assert.throws(()=>validateEncounter(request));
    assert.equal(validateEncounter({...valid(),player:{ship:17,crew:42}}).player.crew,42);
    const request=valid(), copy=validateEncounter(request);request.enemies[0]=0;
    assert.equal(copy.enemies[0],19,'Requests must be detached from later caller mutation');
});
