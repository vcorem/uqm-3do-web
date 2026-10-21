// SPDX-License-Identifier: GPL-2.0-or-later
// Afterlight's original, deterministic surface expeditions.
export const SURFACE = Object.freeze({width:1024,height:720,seconds:150,tick:1/60,radius:11,maxSalvage:24,acceleration:680,drag:4.3,speed:170});
export const SURFACE_MODES = Object.freeze(['survey','rescue','relay']);
const clamp=(value,min,max)=>Math.max(min,Math.min(max,value));
const distance=(a,b)=>Math.hypot(a.x-b.x,a.y-b.y);
const MODES={
    survey:{title:'Field mapping',seconds:150,verb:'MAP',past:'MAPPED',work:3.2,brief:'Map three research sites in any order. Stop inside each gold ring while the scan fills, then bring the findings back to the landing pad.',help:'Release the controls to brake inside a ring. Scan progress is retained when you leave. Red vents pulse: the dashed warning ring gives you time to move.'},
    rescue:{title:'Recovery shuttle',seconds:180,verb:'RECOVER',past:'DELIVERED',work:.9,brief:'Recover the three marked objectives. The lander can make one recovery at a time: stop at a gold marker to pick it up, then return to the landing pad to unload.',help:'A loaded lander moves more slowly. Each delivery is secured immediately. Moving red storms are shown on the scanner; the pad and pickup sites are sheltered.'},
    relay:{title:'Signal relay',seconds:165,verb:'LINK',past:'LINKED',work:1.8,brief:'Restore the three relay stations in numbered order. Stop inside the active gold ring while its signal window is open. Finished links stay connected. Return home after station three.',help:'A striped ring shows a closed signal window; wait safely inside it. The countdown tells you when it opens. Red electrical fields pulse and move; completed links never reset.'}
};
function seedNumber(seed){if(Number.isInteger(seed))return seed>>>0;let hash=2166136261;for(const char of String(seed))hash=Math.imul(hash^char.charCodeAt(0),16777619);return hash>>>0;}
function randomSequence(seed){let value=seed>>>0;return()=>{value=(value+0x6D2B79F5)|0;let n=Math.imul(value^(value>>>15),1|value);n^=n+Math.imul(n^(n>>>7),61|n);return((n^(n>>>14))>>>0)/4294967296;};}
function difficultyLevel(value){return value==='gentle'||value===1?1:value==='hazardous'||value===3?3:2;}
const boundedNumber=(value,min,max,fallback=0)=>Number.isFinite(value)?clamp(value,min,max):fallback;

/** Reproducible serializable world. Upgrades improve safety, never gate a mode.
 * shielding is a percentage reduction (0..75); batteryBonus is additive seconds.
 * All modes return the same bounded campaign receipt; beacons means secured tasks.
 */
export function createSurface({seed=1,biome='basalt',mode='survey',difficulty='standard',shielding=0,batteryBonus=0,scanner=0,completedTasks=[],objectiveLabels=[],id}={}){
    mode=SURFACE_MODES.includes(mode)?mode:'survey';
    const numericSeed=seedNumber(seed),random=randomSequence(numericSeed),level=difficultyLevel(difficulty);
    const mirror=random()>.5,variant=Math.floor(random()*3);
    const layouts=[
        {pad:{x:160,y:600},sites:[{x:190,y:145},{x:830,y:170},{x:740,y:555}]},
        {pad:{x:510,y:615},sites:[{x:150,y:190},{x:830,y:170},{x:495,y:340}]},
        {pad:{x:170,y:125},sites:[{x:830,y:130},{x:810,y:555},{x:200,y:560}]}
    ];
    const layout=layouts[variant],flip=point=>({x:mirror?1024-point.x:point.x,y:point.y});
    const pad={...flip(layout.pad),radius:45};
    const labels=Array.from({length:3},(_,index)=>Array.isArray(objectiveLabels)&&typeof objectiveLabels[index]==='string'&&objectiveLabels[index].trim()?objectiveLabels[index].trim().slice(0,80):`${mode==='rescue'?'Recovery target':mode==='relay'?'Relay station':'Research site'} ${index+1}`);
    const beacons=layout.sites.map((point,index)=>({...flip({x:point.x+(random()-.5)*66,y:point.y+(random()-.5)*52}),id:index,label:labels[index],radius:35,visited:false,carried:false,progress:0,windowPhase:random()*8}));
    if(mode==='rescue'&&Array.isArray(completedTasks))for(const task of new Set(completedTasks))if(Number.isInteger(task)&&task>=0&&task<3)beacons[task].visited=true;
    const hazards=[],hazardCount=level===1?7:level===3?12:9;
    for(let attempt=0;hazards.length<hazardCount&&attempt<2000;attempt++){
        const amplitude=mode==='survey'?0:16+random()*(mode==='rescue'?20:15);
        const point={x:70+random()*884,y:75+random()*570,radius:22+random()*(level===3?20:12),phase:random()*Math.PI*2,amplitude,axis:random()>.5?'x':'y',period:7+random()*5,kind:mode==='rescue'?'storm':mode==='relay'?'arc':'vent'};
        const margin=point.radius+amplitude;
        if(point.x>margin+60&&point.x<1024-margin-60&&point.y>margin+60&&point.y<720-margin-60&&distance(point,pad)>margin+100&&beacons.every(b=>distance(point,b)>margin+85)&&hazards.every(h=>distance(point,h)>margin+h.radius+h.amplitude+70))hazards.push({...point,baseX:point.x,baseY:point.y,active:true,warning:false});
    }
    const ore=[];
    for(let attempt=0;ore.length<8&&attempt<1000;attempt++){
        const point={id:ore.length,x:65+random()*894,y:75+random()*565,value:3,collected:false};
        if(distance(point,pad)>85&&beacons.every(b=>distance(point,b)>55)&&hazards.every(h=>distance(point,h)>h.radius+22)&&ore.every(o=>distance(point,o)>45))ore.push(point);
    }
    const terrain=Array.from({length:110},()=>({x:random()*1024,y:random()*720,radius:3+random()*22,angle:random()*Math.PI*2,shade:random()}));
    const batteryCapacity=MODES[mode].seconds+(level===1?25:level===3?-10:0)+boundedNumber(batteryBonus,0,300);
    const state={id,seed:numericSeed,biome:String(biome),mode,objectiveLabels:labels,difficulty:level,shielding:boundedNumber(shielding,0,75),scanner:boundedNumber(scanner,0,2),pad,beacons,hazards,ore,terrain,x:pad.x,y:pad.y,vx:0,vy:0,heading:-Math.PI/2,hull:100,salvage:0,ticks:0,accumulator:0,batteryCapacity,battery:batteryCapacity,invulnerable:0,cargo:null,docking:0,outcome:null};
    updateHazards(state);return state;
}

export function surfaceReceipt(state){
    if(!state.outcome)return null;
    const receipt={outcome:state.outcome,salvage:clamp(Math.floor(state.salvage),0,SURFACE.maxSalvage),hullRemaining:clamp(Math.ceil(state.hull),0,100),beacons:clamp(state.beacons.filter(beacon=>beacon.visited).length,0,3)};
    if(state.mode==='rescue')receipt.completedTasks=Object.freeze(state.beacons.filter(beacon=>beacon.visited).map(beacon=>beacon.id));
    if(state.id!==undefined)receipt.id=state.id;return Object.freeze(receipt);
}
function relayWindow(state,beacon){const phase=(state.ticks/60+beacon.windowPhase)%8;return {open:phase<4.8,remaining:phase<4.8?4.8-phase:8-phase};}

/** Read-only objective metadata for HUDs, accessibility and input-only playtests. */
export function surfaceObjective(state){
    const done=state.beacons.filter(b=>b.visited).length;
    if(done===3)return {kind:'return',target:state.pad,label:'Return to the landing pad',progress:1};
    if(state.mode==='rescue'&&state.cargo!==null)return {kind:'deliver',target:state.pad,label:`Deliver ${state.beacons[state.cargo].label} to the landing pad`,progress:state.docking};
    const target=state.mode==='relay'?state.beacons.find(b=>!b.visited):state.beacons.filter(b=>!b.visited&&!b.carried).sort((a,b)=>distance(a,state)-distance(b,state))[0];
    const kind=state.mode==='rescue'?'pickup':state.mode==='relay'?'relay':'scan';
    return {kind,target,label:kind==='pickup'?`Recover ${target.label}`:kind==='relay'?`Connect ${target.label}`:`Map ${target.label}`,progress:target.progress/MODES[state.mode].work,...(kind==='relay'?{window:relayWindow(state,target)}:{})};
}
function updateHazards(state){
    const time=state.ticks/60;
    for(const h of state.hazards){
        if(h.baseX===undefined)continue; // Hand-authored static hazards are supported.
        const motion=Math.sin(time*2*Math.PI/h.period+h.phase)*h.amplitude;
        h.x=h.baseX+(h.axis==='x'?motion:0);h.y=h.baseY+(h.axis==='y'?motion:0);
        const cycle=(time+h.phase)%7;
        h.active=h.kind==='storm'||cycle>=3.4;h.warning=h.kind!=='storm'&&cycle>=2.2&&cycle<3.4;
    }
}
function updateTasks(state,speed){
    const quiet=speed<35,dt=SURFACE.tick;
    if(state.mode==='rescue'&&state.cargo!==null){
        if(distance(state,state.pad)<state.pad.radius&&quiet){
            state.docking+=dt;
            if(state.docking>=1){const recovered=state.beacons[state.cargo];recovered.visited=true;recovered.carried=false;state.cargo=null;state.docking=0;}
        }else state.docking=0;
    }else{
        const active=state.mode==='relay'?state.beacons.find(b=>!b.visited):null;
        for(const beacon of state.beacons){
            if(beacon.visited||beacon.carried||(active&&beacon!==active))continue;
            if(distance(state,beacon)<beacon.radius&&quiet&&(state.mode!=='relay'||relayWindow(state,beacon).open)){
                beacon.progress=Math.min(MODES[state.mode].work,beacon.progress+dt*(1+state.scanner*.15));
                if(beacon.progress+1e-9>=MODES[state.mode].work){
                    if(state.mode==='rescue'){beacon.carried=true;state.cargo=beacon.id;state.docking=0;break;}
                    beacon.visited=true;
                }
            }
        }
    }
    if(state.beacons.every(beacon=>beacon.visited)&&distance(state,state.pad)<state.pad.radius&&quiet)state.outcome='surveyed';
}

/** Pure fixed-step physics. input.x/y are -1..1; retreat evacuates immediately.
 * Time is active simulation time. The browser adapter pauses hidden/blurred tabs.
 */
export function stepSurface(state,input={},elapsed=SURFACE.tick){
    if(!Number.isFinite(elapsed)||elapsed<0||elapsed>600)throw new RangeError('Surface elapsed time must be 0–600 seconds.');
    if(state.outcome)return state;
    const next={...state,beacons:state.beacons.map(value=>({...value})),ore:state.ore.map(value=>({...value})),hazards:state.hazards.map(value=>({...value}))};
    if(input.retreat){next.outcome=next.hull>0?'retreated':'damaged';return next;}
    let x=Number.isFinite(input.x)?clamp(input.x,-1,1):0,y=Number.isFinite(input.y)?clamp(input.y,-1,1):0;
    const length=Math.hypot(x,y);if(length>1){x/=length;y/=length;}
    next.accumulator+=elapsed;
    while(next.accumulator+1e-10>=SURFACE.tick&&!next.outcome){
        next.accumulator=Math.max(0,next.accumulator-SURFACE.tick);next.ticks++;
        next.battery=Math.max(0,next.batteryCapacity-next.ticks/60);next.invulnerable=Math.max(0,next.invulnerable-SURFACE.tick);updateHazards(next);
        const damping=Math.exp(-SURFACE.drag*SURFACE.tick),loaded=next.cargo!==null;
        next.vx=(next.vx+x*SURFACE.acceleration*SURFACE.tick*(loaded?.76:1))*damping;
        next.vy=(next.vy+y*SURFACE.acceleration*SURFACE.tick*(loaded?.76:1))*damping;
        let speed=Math.hypot(next.vx,next.vy),limit=SURFACE.speed*(loaded?.76:1);
        if(speed>limit){next.vx*=limit/speed;next.vy*=limit/speed;speed=limit;}
        next.x+=next.vx*SURFACE.tick;next.y+=next.vy*SURFACE.tick;
        if(speed>5)next.heading=Math.atan2(next.vy,next.vx);
        if(next.x<SURFACE.radius||next.x>SURFACE.width-SURFACE.radius){next.x=clamp(next.x,SURFACE.radius,SURFACE.width-SURFACE.radius);next.vx=0;}
        if(next.y<SURFACE.radius||next.y>SURFACE.height-SURFACE.radius){next.y=clamp(next.y,SURFACE.radius,SURFACE.height-SURFACE.radius);next.vy=0;}
        for(const hazard of next.hazards)if(hazard.active!==false&&next.invulnerable===0&&distance(next,hazard)<hazard.radius+SURFACE.radius){
            const damage=(next.difficulty===1?10:next.difficulty===3?16:13)*(1-next.shielding/100);
            next.hull=Math.max(0,next.hull-damage);next.invulnerable=1.6;
        }
        if(next.hull===0){next.outcome='damaged';break;}
        updateTasks(next,speed);
        for(const ore of next.ore)if(!ore.collected&&distance(next,ore)<22){ore.collected=true;next.salvage=Math.min(SURFACE.maxSalvage,next.salvage+ore.value);}
        if(!next.outcome&&next.battery===0)next.outcome='retreated';
    }
    return next;
}

const PALETTES={ice:['#102d3d','#244959','#719da5'],ocean:['#082a38','#123d47','#39686e'],desert:['#312c29','#4a4031','#6d5a3f'],basalt:['#101d2b','#1d2c3b','#3a4854'],crystal:['#231e39','#352d4a','#6b5879']};
function palette(biome){const match=Object.keys(PALETTES).find(key=>biome.toLowerCase().includes(key));return PALETTES[match||'basalt'];}
const STYLE=`.surface-expedition{color:#eaf2ee;background:#07121e;padding:16px;border:1px solid #b69e63;border-radius:12px;font:14px system-ui,sans-serif;max-width:1120px;margin:auto}.surface-expedition *{box-sizing:border-box}.surface-top{display:flex;align-items:center;justify-content:space-between;gap:12px;margin-bottom:10px}.surface-top h2{font-size:21px;margin:0;color:#e5ce91}.surface-top p{margin:5px 0;color:#b9ced0}.surface-expedition button{color:#eaf2ee;background:#172d3c;border:1px solid #728987;border-radius:6px;padding:11px 15px;cursor:pointer;font:inherit;min-height:44px}.surface-expedition button:hover{background:#234351}.surface-expedition button:focus-visible,.surface-expedition canvas:focus-visible{outline:3px solid #edd59a;outline-offset:3px}.surface-actions{display:flex;gap:8px;flex-wrap:wrap;justify-content:flex-end}.surface-canvas{display:block;width:100%;height:auto;max-height:65dvh;aspect-ratio:1024/720;object-fit:contain;background:#091523;border:1px solid #36545b;touch-action:none}.surface-status{display:flex;justify-content:space-between;gap:10px;flex-wrap:wrap;padding:11px 0;color:#bdd9d7;font-variant-numeric:tabular-nums}.surface-footer{display:flex;align-items:center;justify-content:space-between;gap:12px}.surface-instructions{max-width:700px;line-height:1.5;color:#bbccc9;margin:4px 0}.surface-controls{display:grid;grid-template-columns:repeat(3,45px);grid-template-rows:repeat(2,44px);gap:4px;flex-shrink:0}.surface-controls button{padding:5px;touch-action:none;user-select:none;-webkit-user-select:none}.surface-controls [data-direction=up]{grid-column:2}.surface-controls [data-direction=left]{grid-column:1;grid-row:2}.surface-controls [data-direction=down]{grid-column:2;grid-row:2}.surface-controls [data-direction=right]{grid-column:3;grid-row:2}.surface-message{color:#e6d099;font-weight:600;min-height:42px;line-height:1.4}.surface-help{font-size:12px;line-height:1.5;color:#a7bdc0}.surface-briefing{background:#152f3b;border:1px solid #688884;border-radius:8px;padding:14px;margin:10px 0;line-height:1.55}.surface-briefing p{margin:0 0 10px}.surface-expedition [hidden]{display:none!important}.surface-briefing button{background:#31534b;border-color:#b7d9bb}.surface-targets{margin:4px 0 12px;padding-left:25px;color:#dfdfc5}.surface-upgrades{color:#a6e0bc;font-size:12px}.surface-pause-note{margin:0 0 8px;color:#ead5a0}@media(max-width:600px){.surface-expedition{padding:10px}.surface-top h2{font-size:18px}.surface-top p{font-size:12px}.surface-instructions{font-size:12px}.surface-canvas{max-height:none}.surface-status{font-size:12px;gap:6px}.surface-top{align-items:flex-start}.surface-actions{max-width:150px}.surface-actions button{padding:8px;font-size:12px}.surface-footer{align-items:flex-start}.surface-briefing{font-size:13px}}`;

/** Canvas adapter. Preparation and explicit pause never consume simulation time.
 * Completion fires once; destroy removes listeners and grants no reward.
 */
export class SurfaceExpedition{
    constructor({container,onComplete=()=>{},...config}={}){
        if(!container)throw new Error('A surface container is required.');
        this.container=container;this.state=createSurface(config);this.onComplete=onComplete;
        this.keys=new Set();this.pointers=new Map();this.started=false;this.finished=false;this.destroyed=false;this.deployed=false;this.paused=true;this.frame=0;
    }
    start(){
        if(this.started||this.destroyed)return this;this.started=true;
        const document=this.container.ownerDocument;this.document=document;this.window=document.defaultView;
        this.element=document.createElement('section');this.element.className='surface-expedition';
        this.element.innerHTML=`<style>${STYLE}</style><div class="surface-top"><div><h2></h2><p data-subtitle></p></div><div class="surface-actions"><button type="button" data-pause hidden>Pause expedition</button><button type="button" data-retreat>Return to ship · Esc</button></div></div><div class="surface-briefing"><p data-brief></p><ol class="surface-targets" aria-label="Expedition objectives"></ol><p class="surface-upgrades" data-upgrades></p><button type="button" data-deploy>Deploy lander</button></div><p class="surface-pause-note" hidden>Expedition paused. Your battery is safe.</p><canvas class="surface-canvas" width="1024" height="720" tabindex="0"></canvas><div class="surface-status"><span data-battery></span><span data-hull></span><span data-beacons></span><span data-salvage></span></div><div class="surface-message" role="status" aria-live="polite"></div><div class="surface-footer"><div><p class="surface-instructions"></p><p class="surface-help"></p></div><div class="surface-controls" aria-label="Lander direction controls"><button type="button" data-direction="up" aria-label="Steer up">↑</button><button type="button" data-direction="left" aria-label="Steer left">←</button><button type="button" data-direction="down" aria-label="Steer down">↓</button><button type="button" data-direction="right" aria-label="Steer right">→</button></div></div>`;
        const mode=MODES[this.state.mode];
        this.element.querySelector('h2').textContent=mode.title;
        this.element.querySelector('[data-subtitle]').textContent=`${this.state.biome} · ${['','Gentle','Standard','Hazardous'][this.state.difficulty]} conditions`;
        this.element.querySelector('[data-brief]').textContent=mode.brief;
        for(const label of this.state.objectiveLabels){const item=document.createElement('li');item.textContent=label;this.element.querySelector('.surface-targets').append(item);}
        const upgrades=[];if(this.state.shielding)upgrades.push(`Reinforced shielding: ${this.state.shielding}% less damage`);if(this.state.batteryCapacity>mode.seconds)upgrades.push(`${Math.round(this.state.batteryCapacity)} seconds of active battery`);
        this.element.querySelector('[data-upgrades]').textContent=upgrades.join(' · ')||'Take a moment to plan. The timer starts only when you deploy.';
        this.element.querySelector('.surface-instructions').textContent=`Steer with W A S D / arrows or the direction buttons. ${mode.brief}`;
        this.element.querySelector('.surface-help').textContent=`${mode.help} Blue ore is optional. Battery expiry retrieves you automatically; recovered ore is kept.`;
        this.container.append(this.element);this.canvas=this.element.querySelector('canvas');this.context=this.canvas.getContext('2d');
        this.canvas.setAttribute('aria-label',`${mode.title}. ${mode.brief} Objectives: ${this.state.objectiveLabels.join('; ')}. Use arrow keys, W A S D or direction buttons to steer. P pauses; Escape returns to your ship.`);
        this.labels=Object.fromEntries(['battery','hull','beacons','salvage'].map(name=>[name,this.element.querySelector(`[data-${name}]`)]));
        this.message=this.element.querySelector('.surface-message');this.message.textContent='Review your route, then deploy. The battery is not running.';
        this.abort=new AbortController();const signal=this.abort.signal;
        const directions={ArrowUp:'up',KeyW:'up',ArrowDown:'down',KeyS:'down',ArrowLeft:'left',KeyA:'left',ArrowRight:'right',KeyD:'right'};
        this.window.addEventListener('keydown',event=>{
            if(this.finished||this.destroyed||/^(INPUT|TEXTAREA|SELECT)$/.test(event.target?.tagName))return;
            if(event.code==='Escape'){event.preventDefault();event.stopPropagation();this.retreat();return;}
            if(event.code==='KeyP'&&!event.repeat&&this.deployed){event.preventDefault();this.setPaused(!this.paused);return;}
            if(directions[event.code]&&this.deployed&&!this.paused){event.preventDefault();this.keys.add(event.code);}
        },{signal});
        this.window.addEventListener('keyup',event=>{if(directions[event.code]){if(this.deployed)event.preventDefault();this.keys.delete(event.code);}},{signal});
        this.window.addEventListener('blur',()=>{if(this.deployed)this.setPaused(true);},{signal});
        document.addEventListener('visibilitychange',()=>{if(document.hidden&&this.deployed)this.setPaused(true);},{signal});
        this.element.querySelector('[data-retreat]').addEventListener('click',()=>this.retreat(),{signal});
        this.element.querySelector('[data-deploy]').addEventListener('click',()=>{this.deployed=true;this.element.querySelector('.surface-briefing').hidden=true;this.element.querySelector('[data-pause]').hidden=false;this.setPaused(false);},{signal});
        this.element.querySelector('[data-pause]').addEventListener('click',()=>this.setPaused(!this.paused),{signal});
        for(const button of this.element.querySelectorAll('[data-direction]')){
            button.addEventListener('pointerdown',event=>{if(!this.deployed||this.paused)return;event.preventDefault();button.setPointerCapture(event.pointerId);this.pointers.set(event.pointerId,button.dataset.direction);},{signal});
            for(const type of ['pointerup','pointercancel','lostpointercapture'])button.addEventListener(type,event=>this.pointers.delete(event.pointerId),{signal});
            button.addEventListener('click',event=>{if(event.detail===0&&this.deployed&&!this.paused){this.pointers.set('assistive',button.dataset.direction);this.window.setTimeout(()=>this.pointers.delete('assistive'),180);}},{signal});
        }
        this.tick=time=>{
            if(this.destroyed||this.finished)return;
            const dt=this.lastTime===null||this.lastTime===undefined||document.hidden||this.paused?0:Math.min(.1,(time-this.lastTime)/1000);this.lastTime=time;
            const held=new Set([...this.keys].map(code=>directions[code]).concat([...this.pointers.values()]));
            this.state=stepSurface(this.state,{x:Number(held.has('right'))-Number(held.has('left')),y:Number(held.has('down'))-Number(held.has('up'))},dt);
            this.render();if(this.state.outcome){this.finish();return;}this.frame=this.window.requestAnimationFrame(this.tick);
        };
        this.render();this.element.querySelector('[data-deploy]').focus({preventScroll:true});this.frame=this.window.requestAnimationFrame(this.tick);return this;
    }
    setPaused(value){
        if(this.finished||this.destroyed||!this.deployed)return;
        this.paused=Boolean(value);this.keys.clear();this.pointers.clear();this.lastTime=null;
        this.element.querySelector('[data-pause]').textContent=this.paused?'Resume expedition':'Pause expedition';
        this.element.querySelector('.surface-pause-note').hidden=!this.paused;
        if(!this.paused)this.canvas.focus({preventScroll:true});this.render();
    }
    retreat(){if(this.destroyed||this.finished)return;this.state=stepSurface(this.state,{retreat:true},0);this.render();this.finish();}
    finish(){
        if(this.finished||this.destroyed)return;this.finished=true;this.window?.cancelAnimationFrame(this.frame);this.abort?.abort();this.keys.clear();this.pointers.clear();
        const receipt=surfaceReceipt(this.state);if(!receipt)return;
        if(this.message)this.message.textContent=receipt.outcome==='surveyed'?'Expedition secured. Returning to your ship.':receipt.outcome==='damaged'?'Emergency retrieval. The lander has been recovered.':'Expedition recalled. Returning to your ship.';
        this.onComplete(receipt);
    }
    render(){
        const c=this.context;if(!c)return;const s=this.state,mode=MODES[s.mode],[base,rock,edge]=palette(s.biome),objective=surfaceObjective(s);
        c.clearRect(0,0,1024,720);c.fillStyle=base;c.fillRect(0,0,1024,720);
        c.strokeStyle='#a1cbc30a';c.lineWidth=1;for(let x=0;x<1024;x+=64){c.beginPath();c.moveTo(x,0);c.lineTo(x,720);c.stroke();}for(let y=0;y<720;y+=64){c.beginPath();c.moveTo(0,y);c.lineTo(1024,y);c.stroke();}
        for(const stone of s.terrain){c.save();c.translate(stone.x,stone.y);c.rotate(stone.angle);c.fillStyle=stone.shade>.5?rock:base;c.strokeStyle=edge+'66';c.beginPath();if(s.biome==='crystal'){c.moveTo(0,-stone.radius);c.lineTo(stone.radius*.5,0);c.lineTo(0,stone.radius*.5);c.lineTo(-stone.radius*.5,0);c.closePath();}else c.ellipse(0,0,stone.radius,s.biome==='ocean'?stone.radius*.18:stone.radius*.5,0,0,Math.PI*2);c.fill();c.stroke();c.restore();}
        const circle=(x,y,r,color,width=2)=>{c.strokeStyle=color;c.lineWidth=width;c.beginPath();c.arc(x,y,r,0,Math.PI*2);c.stroke();};
        c.fillStyle='#4fc7ba12';c.beginPath();c.arc(s.pad.x,s.pad.y,s.pad.radius,0,Math.PI*2);c.fill();circle(s.pad.x,s.pad.y,s.pad.radius,'#7ce1cb',2);
        c.strokeStyle='#7ce1cb';c.lineWidth=3;c.strokeRect(s.pad.x-15,s.pad.y-15,30,30);c.beginPath();c.moveTo(s.pad.x-15,s.pad.y);c.lineTo(s.pad.x+15,s.pad.y);c.stroke();
        c.font='bold 13px system-ui';c.textAlign='center';c.fillStyle='#8adecd';c.fillText(s.mode==='rescue'?'UNLOAD / LANDING PAD':'LANDING PAD',s.pad.x,s.pad.y+s.pad.radius+22);
        if(s.mode==='relay'){
            c.setLineDash([5,9]);let last=s.pad;
            for(const b of s.beacons){c.strokeStyle=b.visited?'#72e5b28c':'#a3abb52d';c.lineWidth=b.visited?3:1;c.beginPath();c.moveTo(last.x,last.y);c.lineTo(b.x,b.y);c.stroke();last=b;}c.setLineDash([]);
        }
        for(const hazard of s.hazards){
            const color=hazard.active?'#eb8063':hazard.warning?'#f6c679':'#747d7e',pulse=1+Math.sin(s.ticks/18+(hazard.phase||0))*.08;
            if(hazard.amplitude){c.setLineDash([3,7]);c.strokeStyle='#bb705e66';c.lineWidth=1;c.beginPath();c.moveTo(hazard.baseX-(hazard.axis==='x'?hazard.amplitude:0),hazard.baseY-(hazard.axis==='y'?hazard.amplitude:0));c.lineTo(hazard.baseX+(hazard.axis==='x'?hazard.amplitude:0),hazard.baseY+(hazard.axis==='y'?hazard.amplitude:0));c.stroke();c.setLineDash([]);}
            c.fillStyle=hazard.active?'#d6554430':'#d6bd7710';c.beginPath();c.arc(hazard.x,hazard.y,hazard.radius,0,Math.PI*2);c.fill();if(!hazard.active)c.setLineDash([5,5]);circle(hazard.x,hazard.y,hazard.radius*pulse,color,hazard.warning?3:2);c.setLineDash([]);
            c.fillStyle=color;c.font='bold 19px system-ui';c.fillText(hazard.active?'!':hazard.warning?'!':'·',hazard.x,hazard.y+7);
        }
        for(const ore of s.ore)if(!ore.collected){c.save();c.translate(ore.x,ore.y);c.rotate(Math.PI/4);c.fillStyle='#7ecdf0';c.fillRect(-5,-5,10,10);c.strokeStyle='#80caf04d';c.strokeRect(-10,-10,20,20);c.restore();}
        for(const beacon of s.beacons){
            const locked=s.mode==='relay'&&!beacon.visited&&beacon!==objective.target,color=beacon.visited?'#72e5b2':locked||beacon.carried?'#7e8f95':'#e7c678',closed=s.mode==='relay'&&!beacon.visited&&!relayWindow(s,beacon).open;
            if(closed)c.setLineDash([5,5]);circle(beacon.x,beacon.y,beacon.radius,color,locked?1:3);c.setLineDash([]);circle(beacon.x,beacon.y,beacon.radius+6,color+'44',1);
            if(!beacon.visited&&beacon.progress>0){c.lineWidth=5;c.strokeStyle='#bcf0cf';c.beginPath();c.arc(beacon.x,beacon.y,beacon.radius+4,-Math.PI/2,-Math.PI/2+Math.PI*2*beacon.progress/mode.work);c.stroke();}
            c.fillStyle=color;c.font='bold 21px system-ui';c.fillText(beacon.visited?'✓':beacon.carried?'↑':String(beacon.id+1),beacon.x,beacon.y+7);c.font='11px system-ui';
            c.fillText(beacon.visited?mode.past:beacon.carried?'ON BOARD':locked?'WAITING':closed?`WINDOW IN ${Math.ceil(relayWindow(s,beacon).remaining)}s`:mode.verb,beacon.x,beacon.y+beacon.radius+18);
            c.fillStyle='#c3d2d0';c.font='12px system-ui';c.fillText(beacon.label,beacon.x,beacon.y+beacon.radius+34,180);
        }
        const visited=s.beacons.filter(b=>b.visited).length;
        if(objective.kind==='return'||objective.kind==='deliver'){
            c.setLineDash([6,8]);c.strokeStyle='#77dac267';c.beginPath();c.moveTo(s.x,s.y);c.lineTo(s.pad.x,s.pad.y);c.stroke();c.setLineDash([]);circle(s.pad.x,s.pad.y,s.pad.radius+8+Math.sin(s.ticks/15)*4,'#b9f2d8',2);
            if(s.docking){c.strokeStyle='#b9f2d8';c.lineWidth=5;c.beginPath();c.arc(s.pad.x,s.pad.y,s.pad.radius+4,-Math.PI/2,-Math.PI/2+Math.PI*2*s.docking);c.stroke();}
        }
        c.save();c.translate(s.x,s.y);c.rotate(s.heading);if(s.invulnerable>0)circle(0,0,19,'#ffba7b',2);if(s.cargo!==null){c.fillStyle='#e5c77f';c.fillRect(-23,-8,10,16);}c.fillStyle='#dce9e1';c.strokeStyle='#081824';c.lineWidth=2;c.beginPath();c.moveTo(16,0);c.lineTo(-10,10);c.lineTo(-6,0);c.lineTo(-10,-10);c.closePath();c.fill();c.stroke();c.fillStyle='#5fd6d3';c.fillRect(-3,-3,5,6);c.restore();
        c.fillStyle='#d4e1d1';c.textAlign='left';c.font='12px monospace';c.fillText(`${mode.title.toUpperCase()} / ${s.biome.toUpperCase().slice(0,24)}`,18,26);c.fillStyle='#809f9e';c.fillText('N ↑',965,26);
        if(this.paused){c.fillStyle='#06121a99';c.fillRect(0,0,1024,720);c.fillStyle='#e5ce91';c.font='bold 30px system-ui';c.textAlign='center';c.fillText(this.deployed?'EXPEDITION PAUSED':'PLAN YOUR ROUTE',512,330);c.font='17px system-ui';c.fillStyle='#d0e3de';c.fillText(this.deployed?'Resume when you are ready. Battery is safe.':'Three gold objectives. Blue optional ore. Red danger.',512,365);}
        this.labels.battery.textContent=`Battery ${Math.ceil(s.battery)}s`;this.labels.battery.style.color=s.battery<30?'#f1a77d':'';this.labels.hull.textContent=`Hull ${Math.ceil(s.hull)}%`;this.labels.beacons.textContent=`${s.mode==='rescue'?'Delivered':s.mode==='relay'?'Linked':'Mapped'} ${visited} / 3${s.cargo!==null?' · 1 on board':''}`;this.labels.salvage.textContent=`Ore ${s.salvage} / ${SURFACE.maxSalvage}`;
        let message=this.deployed?(this.paused?'Expedition paused. Resume when ready.':`${objective.label}.${objective.kind==='return'?' All tasks secured.':objective.kind==='deliver'?' Stop on the pad to unload.':objective.kind==='relay'&& !objective.window.open?' Wait in the ring for its signal window.':' Stop in the ring to work.'}`):'Review your route, then deploy. The battery is not running.';
        if(this.deployed&&!this.paused&&s.battery<30)message='Low battery: automatic retrieval in under 30 seconds. '+message;
        if(message!==this.message.textContent)this.message.textContent=message;
    }
    destroy(){if(this.destroyed)return;this.destroyed=true;this.window?.cancelAnimationFrame(this.frame);this.abort?.abort();this.keys.clear();this.pointers.clear();this.element?.remove();}
}
