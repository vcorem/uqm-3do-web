// SPDX-License-Identifier: GPL-2.0-or-later
export const SHIPS = Object.freeze([
    ['Androsynth Guardian',20],['Arilou Skiff',6],['Chenjesu Broodhome',36],['Chmmr Avatar',42],
    ['Druuge Mauler',14],['Earthling Cruiser',18],['Ilwrath Avenger',22],['Kohr-Ah Marauder',42],
    ['Melnorme Trader',20],['Mmrnmhrm X-Form',20],['Mycon Podship',20],['Orz Nemesis',16],
    ['Pkunk Fury',8],['Shofixti Scout',6],['Slylandro Probe',12],['Spathi Eluder',30],
    ['Supox Blade',12],['Syreen Penetrator',12,42],['Thraddash Torch',8],['Umgah Drone',10],
    ['Ur-Quan Dreadnought',42],['Utwig Jugger',20],['VUX Intruder',20],['Yehat Terminator',20],['Zoq-Fot-Pik Stinger',10]
].map(([name,crew,maxCrew=crew],id) => Object.freeze({id,name,crew,maxCrew})));

export function validateEncounter(request) {
    if (!request || typeof request.id !== 'string' || !request.id || request.id.length > 128) throw new Error('Encounter ID is required.');
    const ship = SHIPS[request.player?.ship];
    if (!ship || !Number.isInteger(request.player.ship) || !Number.isInteger(request.player.crew) ||
        request.player.crew < 1 || request.player.crew > ship.maxCrew) throw new Error('Invalid player ship or crew.');
    if (!Array.isArray(request.enemies) || request.enemies.length < 1 || request.enemies.length > 14 ||
        request.enemies.some(value => !Number.isInteger(value) || !SHIPS[value])) throw new Error('An encounter needs 1–14 valid enemy ships.');
    if (!Number.isInteger(request.seed) || request.seed < 0 || request.seed > 0xffffffff) throw new Error('Invalid encounter seed.');
    if (!['standard','good','awesome'].includes(request.difficulty || 'standard')) throw new Error('Invalid enemy difficulty.');
    return structuredClone(request);
}

/** Reusable genuine UQM combat. Caller owns the containing UI and campaign save.
 * preload() resolves once Wasm is ready. First start() shows an explicit sound
 * activation button in the iframe; later encounters reuse the running engine.
 * Errors never invent a battle result. retry() reloads the combat process only.
 */
export class UQMCombat extends EventTarget {
    constructor({container, url='./combat/index.html', loadTimeout=240000}={}) {
        super();
        if (!container) throw new Error('A combat container is required.');
        this.container=container;
        this.url=new URL(url,location.href);
        if (this.url.origin!==location.origin) throw new Error('Combat must use the same origin as the campaign.');
        this.loadTimeout=loadTimeout;
        this.frame=null; this.ready=false; this.active=null; this.loading=null;
        this.onMessage=event=>this.receive(event);
        window.addEventListener('message',this.onMessage);
    }
    notify(type,detail) { this.dispatchEvent(new CustomEvent(type,{detail})); }
    preload() {
        if(this.ready)return Promise.resolve();
        if(this.loading)return this.loading;
        this.loading=new Promise((resolve,reject)=>{
            this.resolveReady=resolve; this.rejectReady=reject;
            this.frame=document.createElement('iframe');
            this.frame.title='Afterlight space combat';
            this.frame.allow='autoplay; fullscreen';
            this.frame.src=this.url.href;
            this.frame.hidden=true;
            this.frame.style.cssText='width:100%;height:min(85dvh,850px);border:0';
            this.container.append(this.frame);
            this.timer=setTimeout(()=>this.failure(new Error('Combat took too long to load. Retry combat to try again.')),this.loadTimeout);
        });
        return this.loading;
    }
    async start(request) {
        request=validateEncounter(request);
        if(this.active)throw new Error('An encounter is already active.');
        // Reserve the slot before asynchronous preload so parallel starts fail.
        let encounter;
        const result=new Promise((resolve,reject)=>{encounter={request,resolve,reject};this.active=encounter;});
        result.catch(()=>{});
        this.preload().then(()=>{
            if(this.active!==encounter) return;
            this.frame.hidden=false;
            this.frame.contentWindow.postMessage({type:'uqm-campaign:start',request},this.url.origin);
            this.notify('visible',{id:request.id});
        }).catch(error=>{ if(this.active===encounter)this.failure(error); });
        return result;
    }
    retreat() {
        if(this.active && !this.ready) {
            const id=this.active.request.id;
            const error=new Error('Combat was cancelled before launch.');error.name='AbortError';
            this.active.reject(error);this.active=null;if(this.frame)this.frame.hidden=true;
            this.notify('cancelled',{id});return;
        }
        if(this.active && this.frame)this.frame.contentWindow.postMessage({type:'uqm-campaign:retreat',id:this.active.request.id},this.url.origin);
    }
    receive(event) {
        if(event.origin!==this.url.origin || event.source!==this.frame?.contentWindow || !event.data)return;
        const data=event.data;
        if(data.type==='uqm-campaign:ready') {
            if(data.version!==1)return this.failure(new Error('Incompatible combat adapter.'));
            clearTimeout(this.timer); this.ready=true; this.resolveReady?.(); this.notify('ready',{});
        } else if(data.type==='uqm-campaign:error') {
            if(data.id && this.active && data.id!==this.active.request.id)return;
            this.failure(new Error(data.message || 'Combat failed.'));
        } else if(data.id===this.active?.request.id) {
            if(data.type==='uqm-campaign:cancelled') {
                const error=new Error('Combat was cancelled before launch.');error.name='AbortError';
                this.active.reject(error);this.active=null;this.frame.hidden=true;this.notify('cancelled',{id:data.id});
                return;
            }
            if(data.type==='uqm-campaign:started')this.notify('started',{id:data.id});
            if(data.type==='uqm-campaign:result') {
                const max=SHIPS[this.active.request.player.ship].maxCrew;
                if(!['victory','defeat','retreat'].includes(data.outcome) || !Number.isInteger(data.crew) || data.crew<0 || data.crew>max ||
                    !Number.isInteger(data.enemyRemaining) || data.enemyRemaining<0 || data.enemyRemaining>this.active.request.enemies.length ||
                    !Number.isInteger(data.frames) || data.frames<0 || (data.outcome==='victory' && (data.enemyRemaining!==0 || data.crew<1)) ||
                    (data.outcome==='defeat' && data.crew!==0))return this.failure(new Error('Invalid native combat result.'));
                const active=this.active; this.active=null; this.frame.hidden=true;
                active.resolve(Object.freeze({id:data.id,outcome:data.outcome,crew:data.crew,enemyRemaining:data.enemyRemaining,frames:data.frames}));
                this.notify('result',data);
            }
        }
    }
    failure(error) {
        clearTimeout(this.timer); this.ready=false;
        this.rejectReady?.(error); this.rejectReady=null;
        if(this.active){this.active.reject(error);this.active=null;}
        this.notify('error',{message:error.message});
    }
    retry() {
        if(this.active)throw new Error('Finish or retreat from the active encounter before reloading.');
        clearTimeout(this.timer);this.frame?.remove();this.frame=null;this.ready=false;this.loading=null;
        return this.preload();
    }
    destroy() {
        this.failure(new Error('Combat closed.'));this.frame?.remove();window.removeEventListener('message',this.onMessage);
    }
}
