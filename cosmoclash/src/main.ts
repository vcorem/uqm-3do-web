import { createRenderer } from '@/game/Scene';
import { GameLoop } from '@/engine/GameLoop';
import { InputManager, shouldIgnoreGameKeyDown } from '@/input/InputManager';
import { Viewport } from '@/ui/Viewport';
import { LogoSplash } from '@/ui/LogoSplash';
import { MainMenu } from '@/ui/MainMenu';
import { DifficultyUI } from '@/ui/DifficultyUI';
import { SelectionScene, SelectionState } from '@/game/SelectionScene';
import { SelectionUI } from '@/ui/SelectionUI';
import { CHARACTER_REGISTRY } from '@/game/CharacterRegistry';
import { HUD } from '@/ui/HUD';
import { SoundManager } from '@/audio/SoundManager';
import { RustEngine, SHIP_IDS, inputMask, maskInput, snapshotHash, EMPTY_INPUT, type Snapshot } from '@/rust/Engine';
import { BattleView } from '@/rust/BattleView';
import { NetplayPanel } from '@/netplay/NetplayPanel';
import type { MatchStart, Frame } from '@/netplay/CosmoNetplay';
import '@/ui/hud.css';
import '@/ui/main-menu.css';

const DT=1000/60;
const viewport=new Viewport();
const container=viewport.container;
const renderer=createRenderer(container);
const battle=new BattleView();
const input=new InputManager();
const sound=SoundManager.getInstance();
sound.preload();
let state:'loading'|'logo'|'menu'|'difficulty'|'selection'|'online'|'playing'|'roundOver'|'gameOver'|'error'='loading';
let mode:'ai'|'local'|'online'='ai';
let engine:RustEngine;
let snapshot:Snapshot|null=null;
let menu:MainMenu|null=null;
let difficultyUI:DifficultyUI|null=null;
let selection:SelectionScene|null=null;
let selectionUI:SelectionUI|null=null;
let hud:HUD|null=null;
let panel:NetplayPanel|null=null;
let message:HTMLDivElement|null=null;
let battleInfo:HTMLDivElement|null=null;
let exitButton:HTMLButtonElement|null=null;
let difficulty:'easy'|'medium'|'hard'='medium';
let selected:[number,number]=[0,1];
let selectingPlayer:0|1=0;
let aiRemaining:number[]=[];
let playerUsed=new Set<string>();
let fleetReselect=false;
let lastInputs:[number,number]=[0,0];
let accumulator=0;
let roundDelay=0;
let winner:1|2=1;
let onlineMatch:MatchStart|null=null;
let fleetIndex:[number,number]=[0,0];
let battleIndex=0;
let nextOnlineBattle=false;
let roundSeed=1;
let frames=0;
let lastHash='';
const seed=()=>crypto.getRandomValues(new Uint32Array(1))[0]||1;

function clearMessage():void { message?.remove();message=null; }
function showMessage(text:string,button?:{label:string;run:()=>void}):void {
  clearMessage();message=document.createElement('div');message.className='cosmo-message';message.setAttribute('role','status');
  const p=document.createElement('p');p.textContent=text;message.appendChild(p);
  if(button){const b=document.createElement('button');b.textContent=button.label;b.onclick=button.run;message.appendChild(b);}
  container.appendChild(message);
}
function fail(error:unknown):void { console.error(error);state='error';panel?.session?.close();showMessage(error instanceof Error?error.message:'The game could not continue.',engine?{label:'Return to menu',run:showMenu}:{label:'Reload game',run:()=>location.reload()}); }
function clearSelection():void {selection?.dispose();selection=null;selectionUI?.destroy();selectionUI=null;}
function clearRound():void {battle.clear();hud?.destroy();hud=null;battleInfo?.remove();battleInfo=null;exitButton?.remove();exitButton=null;snapshot=null;}
function showMenu():void {
  panel?.destroy();panel=null;menu?.destroy();menu=null;difficultyUI?.destroy();difficultyUI=null;
  clearSelection();clearRound();clearMessage();accumulator=0;state='menu';onlineMatch=null;playerUsed.clear();aiRemaining=[];fleetReselect=false;
  menu=new MainMenu(container,()=>{
    mode='ai';menu?.destroy();menu=null;state='difficulty';
    difficultyUI=new DifficultyUI(container,level=>{difficulty=(['easy','medium','hard'] as const)[level-1];difficultyUI?.destroy();difficultyUI=null;showSelection(0);});
  },()=>{mode='local';menu?.destroy();menu=null;showSelection(1);},()=>{menu?.destroy();menu=null;showOnline();});
  sound.playMusic('intro');
}
function showSelection(player:0|1):void {
  clearSelection();clearMessage();state='selection';selectingPlayer=player;
  selection=new SelectionScene();selection.state=player===0?SelectionState.SELECTING_P1:SelectionState.SELECTING_P2;
  if(fleetReselect){selection.setDisabledShips(playerUsed);const next=CHARACTER_REGISTRY.findIndex(def=>!playerUsed.has(def.id));selection.jumpToIndex(next);}
  selectionUI=new SelectionUI(container);selectionUI.show(player+1,selection.getCurrentCharacter());
  if(fleetReselect)selectionUI.showRemaining(3-playerUsed.size);
  sound.playMusic('intro');
}
function chooseSelection():void {
  if(!selection)return;selected[selectingPlayer]=CHARACTER_REGISTRY.findIndex(def=>def.id===selection!.getCurrentCharacter().id);
  if(mode==='local'&&selectingPlayer===1){showSelection(0);return;}
  if(mode==='ai'&&!fleetReselect){const other=Math.floor(Math.random()*3);selected[1]=other;aiRemaining=[0,1,2].filter(n=>n!==other);playerUsed.clear();}
  fleetReselect=false;startBattle(seed());
}
function startBattle(value:number):void {
  clearSelection();clearMessage();clearRound();roundSeed=value>>>0;
  snapshot=engine.init(roundSeed,[SHIP_IDS[selected[0]],SHIP_IDS[selected[1]]],mode==='ai'?difficulty:undefined);
  battle.start([SHIP_IDS[selected[0]],SHIP_IDS[selected[1]]]);hud=new HUD(container);
  battleInfo=document.createElement('div');battleInfo.className='cosmo-battle-info';container.appendChild(battleInfo);
  if(mode!=='online'){exitButton=document.createElement('button');exitButton.className='cosmo-exit';exitButton.textContent='Menu';exitButton.onclick=showMenu;container.appendChild(exitButton);}
  state='playing';lastInputs=[0,0];accumulator=0;roundDelay=0;
  battle.update(snapshot,lastInputs,DT);sound.playMusic('battle');
}
function startOnline(start:MatchStart):void {
  onlineMatch=start;fleetIndex=[0,0];battleIndex=0;nextOnlineBattle=false;
  selected=[start.fleets[0][0],start.fleets[1][0]];startBattle(start.seed);
}
function updateHud():void {
  if(!snapshot)return;const [a,b]=snapshot.players;hud?.update(a.health,a.energy,b.health,b.energy);
  if(battleInfo){
    const local=onlineMatch?.localPlayer;
    const names=`${CHARACTER_REGISTRY[selected[0]].name} vs ${CHARACTER_REGISTRY[selected[1]].name}`;
    battleInfo.textContent=mode==='online'?`You are Player ${(local??0)+1} · ${names} · Fleet ${fleetIndex[0]+1}/${onlineMatch!.fleets[0].length} – ${fleetIndex[1]+1}/${onlineMatch!.fleets[1].length}`:names+(mode==='local'?' · P2: W A D · Space · Shift':'');
  }
}
function hashState():string {
  if(!snapshot)return '';
  // Include the fleet controller so divergence cannot hide between battles.
  return snapshotHash({...snapshot, match:{fleetIndex,battleIndex,roundDelay,nextOnlineBattle,roundSeed}} as Snapshot);
}
function onlineFrame(frame:Frame):void {
  if(!onlineMatch||!snapshot)return;
  if(nextOnlineBattle){
    nextOnlineBattle=false;battleIndex++;
    selected=[onlineMatch.fleets[0][fleetIndex[0]],onlineMatch.fleets[1][fleetIndex[1]]];
    startBattle((onlineMatch.seed+Math.imul(battleIndex,0x9e3779b9))>>>0);
  }
  lastInputs=frame.inputs;
  snapshot=engine.step(frame.inputs.map(maskInput) as [ReturnType<typeof maskInput>,ReturnType<typeof maskInput>],DT);frames++;
  if(state==='roundOver'){
    roundDelay++;
    if(roundDelay>=90){
      const lost=(winner===1?1:0) as 0|1;
      if(fleetIndex[lost]+1>=onlineMatch.fleets[lost].length){
        lastHash=hashState();panel!.session!.reportChecksum(frame.tick,lastHash);panel!.session!.finish(winner);return;
      }
      fleetIndex[lost]++;nextOnlineBattle=true;
    }
  }else if(snapshot.winner){winner=snapshot.winner as 1|2;state='roundOver';roundDelay=0;hud?.showWinner(winner,false);}
  lastHash=hashState();if(frame.tick%30===0)panel!.session!.reportChecksum(frame.tick,lastHash);
  battle.update(snapshot,lastInputs,DT);updateHud();
}
function showOnline():void {
  clearRound();clearMessage();state='online';mode='online';
  panel=new NetplayPanel({container,buildId:`cosmo1:${engine.buildId}`,prepare:()=>battle.prepare(),
    onStart:startOnline,onFrame:onlineFrame,
    onEnd:result=>{state='gameOver';if(result)hud?.showWinner(result as 1|2,false);},
    onState:status=>{if(status.phase==='closed'||status.phase==='error'){state='online';}},
    onLeave:showMenu,
  });
  panel.open();
}
function localTick():void {
  if(!snapshot)return;
  const a=input.player(0),b=mode==='local'?input.player(1):EMPTY_INPUT;
  snapshot=engine.step([a,b],DT);frames++;
  lastInputs=[inputMask(a),inputMask(snapshot.aiController?.out??b)];
  if(state==='playing'&&snapshot.winner){winner=snapshot.winner as 1|2;state='roundOver';roundDelay=0;if(mode==='ai'&&winner===2)playerUsed.add(CHARACTER_REGISTRY[selected[0]].id);}
  else if(state==='roundOver'){
    roundDelay++;
    if(roundDelay>=90)hud?.showWinner(winner,false);
    if(roundDelay>=390){
      if(mode==='ai'&&winner===1&&aiRemaining.length){const index=Math.floor(Math.random()*aiRemaining.length);selected[1]=aiRemaining.splice(index,1)[0];startBattle(seed());return;}
      if(mode==='ai'&&winner===2&&playerUsed.size<3){clearRound();fleetReselect=true;showSelection(0);return;}
      state='gameOver';showMessage(`Player ${winner} wins!`,{label:'Return to menu',run:showMenu});
    }
  }
  battle.update(snapshot,lastInputs,DT);updateHud();lastHash=hashState();
}
window.addEventListener('resize',()=>{
  renderer.setSize(viewport.width,viewport.height);battle.camera.aspect=viewport.width/viewport.height;battle.camera.updateProjectionMatrix();
  if(selection){selection.camera.aspect=viewport.width/viewport.height;selection.camera.updateProjectionMatrix();}
});
window.addEventListener('keydown',e=>{
  if(shouldIgnoreGameKeyDown(e))return;
  if(e.key.toLowerCase()==='m')sound.toggleMute();
  if(state==='difficulty'){difficultyUI?.handleKey(e.key);return;}
  if(state==='selection'&&selection&&selectionUI){
    if(e.key==='ArrowLeft')selection.selectPrev();else if(e.key==='ArrowRight')selection.selectNext();else if(e.key==='Enter'){chooseSelection();return;}
    selectionUI.show(selectingPlayer+1,selection.getCurrentCharacter());
  }
  if(e.key==='Escape'&&mode!=='online'&&state!=='loading'&&state!=='logo')showMenu();
});
const gameLoop=new GameLoop(dt=>{
  if(state==='selection'){selection?.update(dt);return;}
  if(state!=='playing'&&state!=='roundOver')return;
  accumulator=Math.min(accumulator+dt,DT*6);
  try{
    for(let n=0;accumulator>=DT&&n<6;n++){
      if(mode==='online'){
        if(!panel?.session?.advance(inputMask(input.player(0)))){accumulator=Math.min(accumulator,DT);break;}
      }else localTick();
      accumulator=Math.max(0,accumulator-DT);
      if(state!=='playing'&&state!=='roundOver')break;
    }
  }catch(error){fail(error);}
},()=>{
  if(state==='selection'&&selection)renderer.render(selection.scene,selection.camera);
  else if(snapshot)renderer.render(battle.scene,battle.camera);
  else renderer.clear();
});
// Read-only diagnostics let tests verify actual rendered engine state without test cheats.
Object.defineProperty(window,'__cosmo',{value:Object.freeze({
  get state(){return state;},get mode(){return mode;},get engine(){return engine?.buildId;},
  get snapshot(){return snapshot?structuredClone(snapshot):null;},get hash(){return lastHash;},
  get frames(){return frames;},get netplay(){return panel?.session?.state??null;},
  get fleet(){return {indices:[...fleetIndex],battle:battleIndex};},
}),writable:false});
showMessage('Loading ships and game engine…');
Promise.all([RustEngine.load(),battle.prepare()]).then(([loaded])=>{
  engine=loaded;clearMessage();gameLoop.start();state='logo';new LogoSplash(container,()=>{
    if(new URL(location.href).searchParams.has('cosmoRoom'))showOnline();else showMenu();
  });
}).catch(fail);
