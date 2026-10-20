import type { PlayerInput } from '@/input/InputManager';
import { assetURL } from '@/base';

export type ShipId = 'michiGichi' | 'gondi' | 'sholev';
export const SHIP_IDS: ShipId[] = ['michiGichi', 'gondi', 'sholev'];
export const EMPTY_INPUT: PlayerInput = Object.freeze({ thrust:false, rotateLeft:false, rotateRight:false, firePrimary:false, fireSecondary:false });
export function inputMask(input: PlayerInput): number {
  return +input.thrust | (+input.rotateLeft << 1) | (+input.rotateRight << 2) | (+input.firePrimary << 3) | (+input.fireSecondary << 4);
}
export function maskInput(mask: number): PlayerInput {
  return { thrust:!!(mask&1), rotateLeft:!!(mask&2), rotateRight:!!(mask&4), firePrimary:!!(mask&8), fireSecondary:!!(mask&16) };
}
export interface Vec { x:number; y:number; z:number }
export interface EntityView {
  id:number; position:Vec; previousPosition:Vec; accelVec:Vec; angle:number; alive:boolean; health:number;
}
export interface PlayerView extends EntityView {
  shipId:ShipId; rollAngle:number; energy:number; morphState:number; currentAnimFrame:number; frostShieldActive:boolean;
}
export interface BulletView extends EntityView {
  kind:string; ttl:number; animFrame:number; animSpeed:number;
}
export interface SnotView extends EntityView { currentFrame:number }
export interface AsteroidView extends EntityView { asteroidType:number; curXAngle:number; curYAngle:number }
export interface ExplosionView extends EntityView { currentFrame:number; size?:number; sourceType:string }
export interface Snapshot {
  tick:number; players:[PlayerView,PlayerView]; bullets:BulletView[]; snots:SnotView[];
  asteroids:AsteroidView[]; explosions:ExplosionView[]; sun:EntityView & {currentFrame:number};
  camera:Vec | {position:Vec}; winner:0|1|2|null; events:({type:string;sound?:string}|string)[];
  checksum?:string;
  aiController?:{out:PlayerInput}|null;
}
interface CosmoExports extends WebAssembly.Exports {
  memory:WebAssembly.Memory;
  cosmo_alloc:(len:number)=>number; cosmo_dealloc:(ptr:number,len:number)=>void;
  cosmo_request:(ptr:number,len:number)=>number; cosmo_result_len:()=>number;
}
/** Browser host only: every battle update executes in the Rust WebAssembly engine. */
export class RustEngine {
  private encoder=new TextEncoder();
  private decoder=new TextDecoder('utf-8',{fatal:true});
  private constructor(private wasm:CosmoExports, readonly buildId:string) {}
  static async load(url=new URL(assetURL('cosmoclash.wasm'),location.href).href):Promise<RustEngine> {
    const response=await fetch(url,{cache:'no-store'});
    if(!response.ok) throw new Error(`Game engine could not load (${response.status}).`);
    const bytes=await response.arrayBuffer();
    const digest=await crypto.subtle.digest('SHA-256',bytes);
    const hash=[...new Uint8Array(digest)].map(n=>n.toString(16).padStart(2,'0')).join('');
    const {instance}=await WebAssembly.instantiate(bytes,{});
    const wasm=instance.exports as CosmoExports;
    for(const key of ['cosmo_alloc','cosmo_dealloc','cosmo_request','cosmo_result_len']) {
      if(typeof wasm[key]!=='function') throw new Error('The installed game engine is incompatible.');
    }
    if(!(wasm.memory instanceof WebAssembly.Memory)) throw new Error('Game engine memory is missing.');
    return new RustEngine(wasm,hash);
  }
  request(command:unknown):Snapshot {
    const bytes=this.encoder.encode(JSON.stringify(command));
    if(bytes.length>64*1024) throw new Error('Game request exceeds the size limit.');
    const ptr=this.wasm.cosmo_alloc(bytes.length);
    if(!ptr) throw new Error('Game engine could not allocate memory.');
    try {
      new Uint8Array(this.wasm.memory.buffer,ptr,bytes.length).set(bytes);
      const output=this.wasm.cosmo_request(ptr,bytes.length);
      const len=this.wasm.cosmo_result_len();
      if(!output||len>4*1024*1024) throw new Error('Invalid game engine response.');
      const result=JSON.parse(this.decoder.decode(new Uint8Array(this.wasm.memory.buffer,output,len)));
      if(!result.ok) throw new Error(result.error || 'Game engine rejected the request.');
      return result.snapshot as Snapshot;
    } finally { this.wasm.cosmo_dealloc(ptr,bytes.length); }
  }
  init(seed:number,ships:[ShipId,ShipId],aiDifficulty?:'easy'|'medium'|'hard'):Snapshot {
    return this.request({op:'init',seed:seed>>>0,ships,aiDifficulty});
  }
  step(inputs:[PlayerInput,PlayerInput],dt=1000/60):Snapshot { return this.request({op:'step',dt,inputs}); }
  snapshot():Snapshot { return this.request({op:'snapshot'}); }
}
/** Hash the complete deterministic snapshot, not pixels or mutable presentation state. */
export function snapshotHash(snapshot:Snapshot):string {
  const text=JSON.stringify(snapshot);
  let hash=0x811c9dc5;
  for(let i=0;i<text.length;i++) hash=Math.imul(hash^text.charCodeAt(i),0x01000193)>>>0;
  return hash.toString(16).padStart(8,'0');
}
