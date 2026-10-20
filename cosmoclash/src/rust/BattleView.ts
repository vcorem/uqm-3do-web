import * as THREE from 'three';
import { GLTFLoader } from 'three/addons/loaders/GLTFLoader.js';
import { clone } from 'three/addons/utils/SkeletonUtils.js';
import { createScene, createStarField, scrollStarField } from '@/game/Scene';
import { createCamera } from '@/game/Camera';
import { CHARACTER_REGISTRY } from '@/game/CharacterRegistry';
import { ThrustEffect } from '@/game/ThrustEffect';
import { FrostTrailEffect } from '@/game/FrostTrailEffect';
import { SoundManager } from '@/audio/SoundManager';
import { SOUND_CONFIG, SoundEffect } from '@/audio/SoundEffect';
import { SHIP_IDS, type Snapshot, type ShipId, type EntityView } from './Engine';

type Visual = { object:THREE.Object3D; kind:string; ownedTexture?:THREE.Texture };
const RAD=Math.PI/180;
/** Original Three.js artwork and effects; this class never advances gameplay. */
export class BattleView {
  readonly scene=createScene();
  readonly camera=createCamera();
  private loader=new GLTFLoader();
  private textures=new Map<string,THREE.Texture>();
  private models=new Map<ShipId,{model:THREE.Group;animations:THREE.AnimationClip[]}>();
  private asteroidModels:THREE.Group[]=[];
  private visuals=new Map<number,Visual>();
  private players:THREE.Group[]=[];
  private mixers:(THREE.AnimationMixer|null)[]=[];
  private actions:(THREE.AnimationAction|null)[]=[];
  private thrust:ThrustEffect[]=[];
  private frost:FrostTrailEffect[]=[];
  private stars=createStarField();
  private sun:THREE.Mesh;
  private sunLight=new THREE.PointLight(0xff0000,2,1500);
  private glow:THREE.Sprite;
  private lastTick=-1;
  private ready:Promise<void>;
  private qHead=new THREE.Quaternion();
  private qRoll=new THREE.Quaternion();
  private zAxis=new THREE.Vector3(0,0,1);
  private forward=new THREE.Vector3();
  constructor() {
    this.scene.add(this.stars,this.sunLight);
    this.sun=new THREE.Mesh(new THREE.SphereGeometry(65,32,32),new THREE.MeshBasicMaterial({blending:THREE.AdditiveBlending,depthWrite:false}));
    this.sun.rotation.x=-Math.PI/2;
    this.scene.add(this.sun);
    const c=document.createElement('canvas');c.width=c.height=128;
    const ctx=c.getContext('2d')!;const g=ctx.createRadialGradient(64,64,0,64,64,64);
    g.addColorStop(0,'rgba(255,80,0,0.6)');g.addColorStop(.4,'rgba(255,30,0,0.2)');g.addColorStop(1,'rgba(255,0,0,0)');
    ctx.fillStyle=g;ctx.fillRect(0,0,128,128);
    this.glow=new THREE.Sprite(new THREE.SpriteMaterial({map:new THREE.CanvasTexture(c),blending:THREE.AdditiveBlending,depthWrite:false,transparent:true}));
    this.glow.scale.set(300,300,1);this.scene.add(this.glow);
    this.ready=this.load();
  }
  async prepare():Promise<void> { await this.ready; }
  private async load():Promise<void> {
    const textureFiles:Record<string,string>={flame:'GondiFire.jpg',plasma:'plasma_atlas.png',sun:'sun_shot.png',ice:'ice_shot_atlas.png',frost:'frost_shield.png',snot:'snot_atlas.png',explosion:'explosion_atlas.png',star:'star_test-02.jpg',sunAtlas:'sun_atlas.jpg',thrust:'thrust_test01.jpg'};
    const tl=new THREE.TextureLoader();
    await Promise.all([
      ...Object.entries(textureFiles).map(async([key,file])=>{this.textures.set(key,await tl.loadAsync(`/textures/${file}`));}),
      ...CHARACTER_REGISTRY.map(async(def,i)=>{
        const gltf=await this.loader.loadAsync(def.modelPath);
        gltf.scene.traverse(child=>{if((child as THREE.Mesh).isMesh){const mats=(child as THREE.Mesh).material;for(const mat of Array.isArray(mats)?mats:[mats])if((mat as THREE.MeshStandardMaterial).emissive)(mat as THREE.MeshStandardMaterial).emissive.set(0,0,0);}});
        this.models.set(SHIP_IDS[i],{model:gltf.scene,animations:gltf.animations});
      }),
      ...[0,1,2].map(async i=>{const gltf=await this.loader.loadAsync(`/models/asteroid0${i+1}.glb`);gltf.scene.scale.setScalar(10);gltf.scene.traverse(child=>{if((child as THREE.Mesh).isMesh){const mat=(child as THREE.Mesh).material as THREE.MeshStandardMaterial;if(mat.emissive)mat.emissive.set(0,0,0);}});this.asteroidModels[i]=gltf.scene;}),
    ]);
    // Preserve the source flame's luminance alpha and warm tint.
    const flame=this.textures.get('flame')!;const image=flame.image as HTMLImageElement;
    const canvas=document.createElement('canvas');canvas.width=image.width;canvas.height=image.height;
    const context=canvas.getContext('2d')!;context.drawImage(image,0,0);const data=context.getImageData(0,0,canvas.width,canvas.height);
    for(let i=0;i<data.data.length;i+=4){const d=data.data;d[i+3]=Math.min(255,(.299*d[i]+.587*d[i+1]+.114*d[i+2])*2);d[i]=Math.min(255,d[i]*1.1);d[i+1]=Math.floor(d[i+1]*.55);d[i+2]=Math.floor(d[i+2]*.15);}
    context.putImageData(data,0,0);flame.image=canvas;flame.needsUpdate=true;
    const sunTex=this.atlas('sunAtlas',8,8);(this.sun.material as THREE.MeshBasicMaterial).map=sunTex;(this.sun.material as THREE.Material).needsUpdate=true;
    for(let i=0;i<2;i++){
      const thrust=new ThrustEffect(this.textures.get('thrust')!);const frost=new FrostTrailEffect(this.textures.get('frost')!);
      this.thrust.push(thrust);this.frost.push(frost);this.scene.add(thrust.points,frost.points);
    }
  }
  start(ships:[ShipId,ShipId]):void {
    this.clear();
    for(let i=0;i<2;i++){
      const data=this.models.get(ships[i])!;const def=CHARACTER_REGISTRY[SHIP_IDS.indexOf(ships[i])];
      const group=new THREE.Group();const model=clone(data.model);
      model.scale.setScalar(def.modelScale);model.rotation.set(...def.modelRotation.map(v=>v*RAD) as [number,number,number]);group.add(model);this.players.push(group);this.scene.add(group);
      if(def.isMichiGichi && data.animations.length){const mixer=new THREE.AnimationMixer(model);const action=mixer.clipAction(data.animations[0]);action.play();action.paused=true;this.mixers.push(mixer);this.actions.push(action);}else{this.mixers.push(null);this.actions.push(null);}
      this.thrust[i].setColor(...(ships[i]==='sholev'?[.5,.8,1]:[1,1,1]) as [number,number,number]);
    }
    this.lastTick=-1;
  }
  clear():void {
    for(const mesh of this.players)this.scene.remove(mesh);
    for(const mixer of this.mixers)mixer?.stopAllAction();
    this.players=[];this.mixers=[];this.actions=[];
    for(const [id] of this.visuals)this.removeVisual(id);
  }
  private removeVisual(id:number):void {
    const visual=this.visuals.get(id);if(!visual)return;
    this.scene.remove(visual.object);
    if(visual.object instanceof THREE.Sprite)visual.object.material.dispose();
    visual.ownedTexture?.dispose();this.visuals.delete(id);
  }
  private atlas(name:string,cols:number,rows:number):THREE.Texture {
    const texture=this.textures.get(name)!.clone();texture.repeat.set(1/cols,1/rows);texture.wrapS=texture.wrapT=THREE.RepeatWrapping;texture.needsUpdate=true;
    if(name==='plasma'||name==='ice')texture.colorSpace=THREE.SRGBColorSpace;
    return texture;
  }
  private setFrame(texture:THREE.Texture,frame:number,cols:number,rows:number):void {const f=Math.max(0,Math.floor(frame));texture.offset.set((f%cols)/cols,1-(Math.floor(f/cols)+1)/rows);}
  private sprite(id:number,kind:string,size:number,cols=1,rows=1):THREE.Sprite {
    let entry=this.visuals.get(id);
    if(entry?.kind!==kind){this.removeVisual(id);entry=undefined;}
    if(!entry){
      const texture=cols>1?this.atlas(kind,cols,rows):this.textures.get(kind)!;
      const sprite=new THREE.Sprite(new THREE.SpriteMaterial({map:texture,blending:THREE.AdditiveBlending,depthWrite:false,transparent:true}));sprite.scale.set(size,size,1);
      entry={object:sprite,kind,ownedTexture:cols>1?texture:undefined};this.visuals.set(id,entry);this.scene.add(sprite);
    }
    return entry.object as THREE.Sprite;
  }
  private position(object:THREE.Object3D,e:EntityView):void {object.position.set(e.position.x,e.position.y,e.position.z);object.visible=e.alive;}
  update(snapshot:Snapshot,inputs:[number,number],visualDt:number):void {
    const changed=snapshot.tick!==this.lastTick;
    const dt=changed?visualDt:0;
    const camera='position' in snapshot.camera?snapshot.camera.position:snapshot.camera;
    const dx=camera.x-this.camera.position.x,dy=camera.y-this.camera.position.y;
    this.camera.position.set(camera.x,camera.y,camera.z);this.camera.lookAt(camera.x,camera.y,0);
    scrollStarField(this.stars,dx,dy);this.stars.position.set(camera.x,camera.y,250);
    this.position(this.sun,snapshot.sun);this.position(this.glow,snapshot.sun);this.position(this.sunLight,snapshot.sun);
    this.setFrame((this.sun.material as THREE.MeshBasicMaterial).map!,snapshot.sun.currentFrame,8,8);
    snapshot.players.forEach((p,i)=>{
      const mesh=this.players[i];if(!mesh)return;this.position(mesh,p);
      const heading=this.qHead.setFromAxisAngle(this.zAxis,p.angle*RAD);const forward=this.forward.set(0,1,0).applyQuaternion(heading);mesh.quaternion.copy(this.qRoll.setFromAxisAngle(forward,p.rollAngle*RAD).multiply(heading));
      const action=this.actions[i];if(action){action.time=(p.currentAnimFrame/42)*action.getClip().duration;this.mixers[i]!.update(0);}
      this.thrust[i].update(dt,p.alive&&!!(inputs[i]&1)&&!p.frostShieldActive,p.position.x,p.position.y,p.angle);
      this.frost[i].update(dt,p.alive&&p.frostShieldActive,p.position.x,p.position.y);
    });
    const active=new Set<number>();
    for(const b of snapshot.bullets){
      if(!b.alive)continue;active.add(b.id);
      const kind=b.kind==='frost-shield'||b.kind==='frostShield'?'frost':b.kind;
      const dimensions:Record<string,[number,number,number]>={flame:[35,1,1],plasma:[50,10,8],sun:[10,1,1],ice:[15,9,8],frost:[50,1,1]};
      const [size,cols,rows]=dimensions[kind]??dimensions.flame;
      const sprite=this.sprite(b.id,kind,size,cols,rows);this.position(sprite,b);
      const mat=sprite.material;
      if(kind==='flame'){const elapsed=700-b.ttl;mat.rotation=b.id*1.7+elapsed*.008;const scale=35*Math.min(elapsed/100,1)*Math.min(b.ttl/200,1);sprite.scale.set(scale,scale,1);}
      else if(kind==='plasma'){this.setFrame(mat.map!,Math.min(b.animFrame,79),10,8);const scale=Math.max(20,(10000-b.ttl)/60);sprite.scale.set(scale,scale,1);}
      else if(kind==='ice'){this.setFrame(mat.map!,b.animFrame%65,9,8);mat.rotation=b.angle*RAD+Math.PI;}
      else if(kind==='frost')mat.opacity=.6+.4*Math.sin((500000-b.ttl)*.005);
    }
    for(const s of snapshot.snots){if(!s.alive)continue;active.add(s.id);const sprite=this.sprite(s.id,'snot',50,11,11);this.position(sprite,s);this.setFrame(sprite.material.map!,s.currentFrame,11,11);}
    for(const e of snapshot.explosions){if(!e.alive)continue;active.add(e.id);const sprite=this.sprite(e.id,'explosion',e.size||(e.sourceType==='Asteroid'?150:300),9,9);this.position(sprite,e);sprite.scale.setScalar(e.size||(e.sourceType==='Asteroid'?150:300));this.setFrame(sprite.material.map!,e.currentFrame,9,9);}
    for(const a of snapshot.asteroids){if(!a.alive)continue;active.add(a.id);let entry=this.visuals.get(a.id);const kind=`asteroid-${a.asteroidType}`;if(entry?.kind!==kind){this.removeVisual(a.id);entry=undefined;}if(!entry){entry={object:this.asteroidModels[a.asteroidType].clone(),kind};this.visuals.set(a.id,entry);this.scene.add(entry.object);}this.position(entry.object,a);entry.object.rotation.set(a.curXAngle*RAD,a.curYAngle*RAD,0);}
    for(const id of this.visuals.keys())if(!active.has(id))this.removeVisual(id);
    if(changed)for(const event of snapshot.events){const sound=typeof event==='string'?event:event.sound;if(sound&&Object.prototype.hasOwnProperty.call(SOUND_CONFIG,sound))SoundManager.getInstance().play(sound as SoundEffect);}
    this.lastTick=snapshot.tick;
  }
}
