/** Original file-based score, decoded after a user gesture. No remote requests. */
export class Score {
  constructor() { this.enabled=true;this.volume=.38;this.context=null;this.master=null;this.current=null;this.wanted=null;this.cache=new Map();this.pending=new Map();this.errors=[]; }
  async unlock() {
    if(!this.context) {
      const Context=globalThis.AudioContext || globalThis.webkitAudioContext;
      if(!Context)return;
      this.context=new Context();this.master=this.context.createGain();this.master.gain.value=this.enabled?this.volume:0;this.master.connect(this.context.destination);
    }
    await this.context.resume();
    if(this.wanted)await this.play(this.wanted);
  }
  setEnabled(enabled) { this.enabled=!!enabled;if(this.master)this.master.gain.setTargetAtTime(this.enabled?this.volume:0,this.context.currentTime,.2);if(enabled)this.unlock().catch(e=>this.errors.push(e.message)); }
  async buffer(id) {
    if(this.cache.has(id))return this.cache.get(id);
    if(this.pending.has(id))return this.pending.get(id);
    const promise=(async()=>{
      const response=await fetch(new URL(`../assets/music/${id}.ogg`,import.meta.url));
      if(!response.ok)throw new Error(`Music unavailable: ${id}`);
      const buffer=await this.context.decodeAudioData(await response.arrayBuffer());
      this.cache.set(id,buffer);
      while(this.cache.size>3){const key=this.cache.keys().next().value;this.cache.delete(key);}
      return buffer;
    })();
    this.pending.set(id,promise);
    try{return await promise;}finally{this.pending.delete(id);}
  }
  async play(id) {
    this.wanted=id;
    if(!this.context || this.current?.id===id)return;
    try {
      const buffer=await this.buffer(id);
      if(this.wanted!==id || this.current?.id===id)return;
      const source=this.context.createBufferSource(),gain=this.context.createGain(),now=this.context.currentTime;
      source.onended=()=>{source.disconnect();gain.disconnect();};
      source.buffer=buffer;source.loop=true;source.connect(gain);gain.connect(this.master);
      gain.gain.setValueAtTime(0,now);gain.gain.linearRampToValueAtTime(1,now+1.4);source.start();
      if(this.current){this.current.gain.gain.cancelScheduledValues(now);this.current.gain.gain.setValueAtTime(this.current.gain.gain.value,now);this.current.gain.gain.linearRampToValueAtTime(0,now+1.3);this.current.source.stop(now+1.5);}
      this.current={id,source,gain};
    } catch(error) { this.errors.push(error.message); }
  }
}
