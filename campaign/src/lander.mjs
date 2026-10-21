// SPDX-License-Identifier: GPL-2.0-or-later
// Afterlight's original surface expedition; independent of the classic UQM lander.
export const SURFACE = Object.freeze({width:1024,height:720,seconds:120,tick:1/60,radius:11,maxSalvage:24,acceleration:680,drag:4.3,speed:170});
const clamp=(value,min,max)=>Math.max(min,Math.min(max,value));
const distance=(a,b)=>Math.hypot(a.x-b.x,a.y-b.y);
function seedNumber(seed) {
    if(Number.isInteger(seed))return seed>>>0;
    let hash=2166136261;for(const char of String(seed))hash=Math.imul(hash^char.charCodeAt(0),16777619);return hash>>>0;
}
function randomSequence(seed) {
    let value=seed>>>0;return()=>{value=(value+0x6D2B79F5)|0;let n=Math.imul(value^(value>>>15),1|value);n^=n+Math.imul(n^(n>>>7),61|n);return((n^(n>>>14))>>>0)/4294967296;};
}

/** Reproducible serializable world. No clock, DOM, random globals or side effects. */
export function createSurface({seed=1,biome='basalt',id}={}) {
    const numericSeed=seedNumber(seed),random=randomSequence(numericSeed),pad={x:512,y:620,radius:42};
    const beacons=[{x:215,y:200},{x:800,y:215},{x:515,y:360}].map((point,index)=>({id:index,x:point.x+(random()-.5)*90,y:point.y+(random()-.5)*65,radius:29,visited:false}));
    const hazards=[];
    for(let attempt=0;hazards.length<9 && attempt<300;attempt++) {
        const point={x:65+random()*894,y:75+random()*575,radius:20+random()*14,phase:random()*Math.PI*2};
        if(distance(point,pad)>115 && beacons.every(b=>distance(point,b)>95) && hazards.every(h=>distance(point,h)>90))hazards.push(point);
    }
    const ore=[];
    for(let attempt=0;ore.length<8 && attempt<300;attempt++) {
        const point={id:ore.length,x:65+random()*894,y:75+random()*565,value:3,collected:false};
        if(distance(point,pad)>70 && hazards.every(h=>distance(point,h)>h.radius+38) && ore.every(o=>distance(point,o)>50))ore.push(point);
    }
    const terrain=Array.from({length:95},()=>({x:random()*1024,y:random()*720,radius:3+random()*22,angle:random()*Math.PI*2,shade:random()}));
    return {id,seed:numericSeed,biome:String(biome),pad,beacons,hazards,ore,terrain,x:pad.x,y:pad.y,vx:0,vy:0,heading:-Math.PI/2,hull:100,salvage:0,ticks:0,accumulator:0,battery:SURFACE.seconds,invulnerable:0,outcome:null};
}

export function surfaceReceipt(state) {
    if(!state.outcome)return null;
    const receipt={outcome:state.outcome,salvage:clamp(Math.floor(state.salvage),0,SURFACE.maxSalvage),hullRemaining:clamp(Math.ceil(state.hull),0,100),beacons:state.beacons.filter(beacon=>beacon.visited).length};
    if(state.id!==undefined)receipt.id=state.id;
    return Object.freeze(receipt);
}

/** Pure fixed-step physics; returns a new state and never changes its input.
 * input.x/y are -1..1; input.retreat always evacuates immediately. Time is active
 * simulation time: a hidden tab pauses, so a backgrounded browser cannot kill crew.
 */
export function stepSurface(state,input={},elapsed=SURFACE.tick) {
    if(!Number.isFinite(elapsed)||elapsed<0||elapsed>600)throw new RangeError('Surface elapsed time must be 0–600 seconds.');
    if(state.outcome)return state;
    const next={...state,beacons:state.beacons.map(value=>({...value})),ore:state.ore.map(value=>({...value}))};
    if(input.retreat){next.outcome=next.hull>0?'retreated':'damaged';return next;}
    let x=Number.isFinite(input.x)?clamp(input.x,-1,1):0,y=Number.isFinite(input.y)?clamp(input.y,-1,1):0;
    const length=Math.hypot(x,y);if(length>1){x/=length;y/=length;}
    next.accumulator+=elapsed;
    while(next.accumulator+1e-10>=SURFACE.tick && !next.outcome) {
        next.accumulator=Math.max(0,next.accumulator-SURFACE.tick);next.ticks++;
        next.battery=Math.max(0,SURFACE.seconds-next.ticks/60);next.invulnerable=Math.max(0,next.invulnerable-SURFACE.tick);
        const damping=Math.exp(-SURFACE.drag*SURFACE.tick);
        next.vx=(next.vx+x*SURFACE.acceleration*SURFACE.tick)*damping;
        next.vy=(next.vy+y*SURFACE.acceleration*SURFACE.tick)*damping;
        const speed=Math.hypot(next.vx,next.vy);if(speed>SURFACE.speed){next.vx*=SURFACE.speed/speed;next.vy*=SURFACE.speed/speed;}
        next.x+=next.vx*SURFACE.tick;next.y+=next.vy*SURFACE.tick;
        if(speed>5)next.heading=Math.atan2(next.vy,next.vx);
        if(next.x<SURFACE.radius||next.x>SURFACE.width-SURFACE.radius){next.x=clamp(next.x,SURFACE.radius,SURFACE.width-SURFACE.radius);next.vx=0;}
        if(next.y<SURFACE.radius||next.y>SURFACE.height-SURFACE.radius){next.y=clamp(next.y,SURFACE.radius,SURFACE.height-SURFACE.radius);next.vy=0;}
        for(const hazard of next.hazards)if(next.invulnerable===0 && distance(next,hazard)<hazard.radius+SURFACE.radius) {
            next.hull=Math.max(0,next.hull-14);next.invulnerable=1.4;
        }
        if(next.hull===0){next.outcome='damaged';break;}
        for(const beacon of next.beacons)if(!beacon.visited && distance(next,beacon)<beacon.radius+SURFACE.radius)beacon.visited=true;
        for(const ore of next.ore)if(!ore.collected && distance(next,ore)<22){ore.collected=true;next.salvage=Math.min(SURFACE.maxSalvage,next.salvage+ore.value);}
        if(next.beacons.every(beacon=>beacon.visited) && distance(next,next.pad)<next.pad.radius){next.outcome='surveyed';break;}
        if(next.battery===0){next.outcome='retreated';break;}
    }
    return next;
}

const PALETTES={ice:['#102d3d','#244959','#719da5'],ocean:['#082a38','#123d47','#39686e'],desert:['#312c29','#4a4031','#6d5a3f'],basalt:['#101d2b','#1d2c3b','#3a4854'],crystal:['#231e39','#352d4a','#6b5879']};
function palette(biome){const match=Object.keys(PALETTES).find(key=>biome.toLowerCase().includes(key));return PALETTES[match||'basalt'];}
const STYLE=`.surface-expedition{color:#eaf2ee;background:#07121e;padding:16px;border:1px solid #b69e63;border-radius:12px;font:14px system-ui,sans-serif;max-width:1120px;margin:auto}.surface-expedition *{box-sizing:border-box}.surface-top{display:flex;align-items:center;justify-content:space-between;gap:12px;margin-bottom:10px}.surface-top h2{font-size:21px;margin:0;color:#e5ce91}.surface-top p{margin:5px 0;color:#b9ced0}.surface-expedition button{color:#eaf2ee;background:#172d3c;border:1px solid #728987;border-radius:6px;padding:11px 15px;cursor:pointer;font:inherit;min-height:44px}.surface-expedition button:hover{background:#234351}.surface-expedition button:focus-visible,.surface-expedition canvas:focus-visible{outline:3px solid #edd59a;outline-offset:3px}.surface-canvas{display:block;width:100%;height:auto;max-height:65dvh;aspect-ratio:1024/720;object-fit:contain;background:#091523;border:1px solid #36545b;touch-action:none}.surface-status{display:flex;justify-content:space-between;gap:10px;flex-wrap:wrap;padding:11px 0;color:#bdd9d7;font-variant-numeric:tabular-nums}.surface-footer{display:flex;align-items:center;justify-content:space-between;gap:12px}.surface-instructions{max-width:640px;line-height:1.5;color:#bbccc9;margin:4px 0}.surface-controls{display:grid;grid-template-columns:repeat(3,45px);grid-template-rows:repeat(2,44px);gap:4px;flex-shrink:0}.surface-controls button{padding:5px;touch-action:none;user-select:none;-webkit-user-select:none}.surface-controls [data-direction=up]{grid-column:2}.surface-controls [data-direction=left]{grid-column:1;grid-row:2}.surface-controls [data-direction=down]{grid-column:2;grid-row:2}.surface-controls [data-direction=right]{grid-column:3;grid-row:2}.surface-message{color:#e6d099;font-weight:600;min-height:22px}.surface-help{font-size:12px;color:#99adb0}@media(max-width:600px){.surface-expedition{padding:10px}.surface-top h2{font-size:18px}.surface-top p{font-size:12px}.surface-instructions{font-size:12px}.surface-canvas{max-height:none}.surface-status{font-size:12px;gap:6px}.surface-top{align-items:flex-start}}`;

/** Canvas surface game. start() is idempotent; completion fires exactly once.
 * destroy() releases every listener and animation frame without awarding rewards.
 */
export class SurfaceExpedition {
    constructor({container,seed=1,biome='basalt',id,onComplete=()=>{}}={}) {
        if(!container)throw new Error('A surface container is required.');
        this.container=container;this.state=createSurface({seed,biome,id});this.onComplete=onComplete;
        this.keys=new Set();this.pointers=new Map();this.started=false;this.finished=false;this.destroyed=false;this.frame=0;
    }
    start() {
        if(this.started||this.destroyed)return this;this.started=true;
        const document=this.container.ownerDocument;this.document=document;this.window=document.defaultView;
        this.element=document.createElement('section');this.element.className='surface-expedition';
        this.element.innerHTML=`<style>${STYLE}</style><div class="surface-top"><div><h2>Surface expedition</h2><p>Survey three sites. Bring the findings home.</p></div><button type="button" data-retreat>Return to ship · Esc</button></div><canvas class="surface-canvas" width="1024" height="720" tabindex="0" aria-label="Surface expedition. Use arrow keys or W A S D to steer. Reach the three numbered survey beacons, then return to the landing pad. Escape returns to your ship."></canvas><div class="surface-status"><span data-battery></span><span data-hull></span><span data-beacons></span><span data-salvage></span></div><div class="surface-message" role="status" aria-live="polite"></div><div class="surface-footer"><div><p class="surface-instructions">Steer with <strong>W A S D / arrows</strong> or the direction buttons. Visit the three gold rings, collect optional blue ore, then return to the landing pad. Red vents damage the lander.</p><p class="surface-help">Release to brake. Battery expiry evacuates automatically. Returning early keeps recovered ore.</p></div><div class="surface-controls" aria-label="Lander direction controls"><button type="button" data-direction="up" aria-label="Steer up">↑</button><button type="button" data-direction="left" aria-label="Steer left">←</button><button type="button" data-direction="down" aria-label="Steer down">↓</button><button type="button" data-direction="right" aria-label="Steer right">→</button></div></div>`;
        this.container.append(this.element);this.canvas=this.element.querySelector('canvas');this.context=this.canvas.getContext('2d');
        this.labels=Object.fromEntries(['battery','hull','beacons','salvage'].map(name=>[name,this.element.querySelector(`[data-${name}]`)]));
        this.message=this.element.querySelector('.surface-message');this.message.textContent='Touch down. The three survey rings are marked on your scanner.';
        this.abort=new AbortController();const signal=this.abort.signal;
        const directions={ArrowUp:'up',KeyW:'up',ArrowDown:'down',KeyS:'down',ArrowLeft:'left',KeyA:'left',ArrowRight:'right',KeyD:'right'};
        this.window.addEventListener('keydown',event=>{
            if(this.finished||this.destroyed)return;
            // Do not consume keys typed into unrelated text fields.
            if(/^(INPUT|TEXTAREA|SELECT)$/.test(event.target?.tagName))return;
            if(event.code==='Escape'){event.preventDefault();event.stopPropagation();this.retreat();return;}
            if(directions[event.code]){event.preventDefault();this.keys.add(event.code);}
        },{signal});
        this.window.addEventListener('keyup',event=>{if(directions[event.code]){event.preventDefault();this.keys.delete(event.code);}},{signal});
        const release=()=>{this.keys.clear();this.pointers.clear();this.lastTime=null;};
        this.window.addEventListener('blur',release,{signal});
        document.addEventListener('visibilitychange',release,{signal});
        this.element.querySelector('[data-retreat]').addEventListener('click',()=>this.retreat(),{signal});
        for(const button of this.element.querySelectorAll('[data-direction]')) {
            button.addEventListener('pointerdown',event=>{event.preventDefault();button.setPointerCapture(event.pointerId);this.pointers.set(event.pointerId,button.dataset.direction);},{signal});
            for(const type of ['pointerup','pointercancel','lostpointercapture'])button.addEventListener(type,event=>this.pointers.delete(event.pointerId),{signal});
            // Native button activation supports keyboard and assistive devices.
            button.addEventListener('click',event=>{if(event.detail===0){this.pointers.set('assistive',button.dataset.direction);this.window.setTimeout(()=>this.pointers.delete('assistive'),180);}},{signal});
        }
        this.tick=time=>{
            if(this.destroyed||this.finished)return;
            const dt=this.lastTime===null||this.lastTime===undefined||document.hidden?0:Math.min(.1,(time-this.lastTime)/1000);this.lastTime=time;
            const held=new Set([...this.keys].map(code=>directions[code]).concat([...this.pointers.values()]));
            this.state=stepSurface(this.state,{x:Number(held.has('right'))-Number(held.has('left')),y:Number(held.has('down'))-Number(held.has('up'))},dt);
            this.render();if(this.state.outcome){this.finish();return;}this.frame=this.window.requestAnimationFrame(this.tick);
        };
        this.render();this.canvas.focus({preventScroll:true});this.frame=this.window.requestAnimationFrame(this.tick);return this;
    }
    retreat(){if(this.destroyed||this.finished)return;this.state=stepSurface(this.state,{retreat:true},0);this.render();this.finish();}
    finish(){
        if(this.finished||this.destroyed)return;this.finished=true;this.window?.cancelAnimationFrame(this.frame);this.abort?.abort();this.keys.clear();this.pointers.clear();
        const receipt=surfaceReceipt(this.state);if(!receipt)return;
        if(this.message)this.message.textContent=receipt.outcome==='surveyed'?'Survey secured. Returning to your ship.':receipt.outcome==='damaged'?'Emergency retrieval. The lander has been recovered.':'Expedition recalled. Returning to your ship.';
        this.onComplete(receipt);
    }
    render(){
        const c=this.context;if(!c)return;const s=this.state,[base,rock,edge]=palette(s.biome);c.clearRect(0,0,1024,720);c.fillStyle=base;c.fillRect(0,0,1024,720);
        c.strokeStyle='#a1cbc30a';c.lineWidth=1;for(let x=0;x<1024;x+=64){c.beginPath();c.moveTo(x,0);c.lineTo(x,720);c.stroke();}for(let y=0;y<720;y+=64){c.beginPath();c.moveTo(0,y);c.lineTo(1024,y);c.stroke();}
        for(const stone of s.terrain){c.save();c.translate(stone.x,stone.y);c.rotate(stone.angle);c.fillStyle=stone.shade>.5?rock:base;c.strokeStyle=edge+'66';c.beginPath();c.ellipse(0,0,stone.radius,stone.radius*.5,0,0,Math.PI*2);c.fill();c.stroke();c.restore();}
        const circle=(x,y,r,color,width=2)=>{c.strokeStyle=color;c.lineWidth=width;c.beginPath();c.arc(x,y,r,0,Math.PI*2);c.stroke();};
        c.fillStyle='#4fc7ba12';c.beginPath();c.arc(s.pad.x,s.pad.y,s.pad.radius,0,Math.PI*2);c.fill();circle(s.pad.x,s.pad.y,s.pad.radius,'#7ce1cb',2);
        c.strokeStyle='#7ce1cb';c.lineWidth=3;c.strokeRect(s.pad.x-15,s.pad.y-15,30,30);c.beginPath();c.moveTo(s.pad.x-15,s.pad.y);c.lineTo(s.pad.x+15,s.pad.y);c.stroke();
        c.font='bold 13px system-ui';c.textAlign='center';c.fillStyle='#8adecd';c.fillText('LANDING PAD',s.pad.x,s.pad.y+s.pad.radius+22);
        for(const hazard of s.hazards){const pulse=1+Math.sin(s.ticks/18+hazard.phase)*.13;c.fillStyle='#d6554422';c.beginPath();c.arc(hazard.x,hazard.y,hazard.radius+8,0,Math.PI*2);c.fill();circle(hazard.x,hazard.y,hazard.radius*pulse,'#cf6653',2);c.fillStyle='#dc8464';c.beginPath();c.moveTo(hazard.x,hazard.y-9);c.lineTo(hazard.x+8,hazard.y+7);c.lineTo(hazard.x-8,hazard.y+7);c.closePath();c.fill();}
        for(const ore of s.ore)if(!ore.collected){c.save();c.translate(ore.x,ore.y);c.rotate(Math.PI/4);c.fillStyle='#7ecdf0';c.fillRect(-5,-5,10,10);c.strokeStyle='#80caf04d';c.strokeRect(-10,-10,20,20);c.restore();}
        for(const beacon of s.beacons){const color=beacon.visited?'#72e5b2':'#e7c678';circle(beacon.x,beacon.y,beacon.radius,color,3);circle(beacon.x,beacon.y,beacon.radius+6,color+'44',1);c.fillStyle=color;c.font='bold 21px system-ui';c.fillText(beacon.visited?'✓':String(beacon.id+1),beacon.x,beacon.y+7);c.font='11px system-ui';c.fillText(beacon.visited?'RECORDED':'SURVEY',beacon.x,beacon.y+beacon.radius+18);}
        const visited=s.beacons.filter(b=>b.visited).length;
        if(visited===3){c.setLineDash([6,8]);c.strokeStyle='#77dac267';c.beginPath();c.moveTo(s.x,s.y);c.lineTo(s.pad.x,s.pad.y);c.stroke();c.setLineDash([]);circle(s.pad.x,s.pad.y,s.pad.radius+8+Math.sin(s.ticks/15)*4,'#b9f2d8',2);}
        c.save();c.translate(s.x,s.y);c.rotate(s.heading);if(s.invulnerable>0){circle(0,0,19,'#ffba7b',2);c.globalAlpha=.7+Math.sin(s.ticks)*.3;}c.fillStyle='#dce9e1';c.strokeStyle='#081824';c.lineWidth=2;c.beginPath();c.moveTo(16,0);c.lineTo(-10,10);c.lineTo(-6,0);c.lineTo(-10,-10);c.closePath();c.fill();c.stroke();c.fillStyle='#5fd6d3';c.fillRect(-3,-3,5,6);c.restore();
        c.fillStyle='#d4e1d1';c.textAlign='left';c.font='12px monospace';c.fillText(`SURFACE SCANNER / ${s.biome.toUpperCase().slice(0,24)}`,18,26);c.fillStyle='#809f9e';c.fillText('N ↑',965,26);
        this.labels.battery.textContent=`Battery ${Math.ceil(s.battery)}s`;this.labels.battery.style.color=s.battery<25?'#f1a77d':'';this.labels.hull.textContent=`Hull ${s.hull}%`;this.labels.beacons.textContent=`Survey ${visited} / 3`;this.labels.salvage.textContent=`Ore ${s.salvage} / ${SURFACE.maxSalvage}`;
        const message=visited===3?'All three sites recorded. Return to the glowing landing pad.':s.battery<25?'Low battery. Evacuation is automatic at zero.':visited?`${visited} of 3 survey sites recorded. Keep moving.`:null;
        if(message && message!==this.message.textContent)this.message.textContent=message;
    }
    destroy(){if(this.destroyed)return;this.destroyed=true;this.window?.cancelAnimationFrame(this.frame);this.abort?.abort();this.keys.clear();this.pointers.clear();this.element?.remove();}
}
