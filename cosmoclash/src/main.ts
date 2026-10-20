import { installAssetBase } from '@/base';
import { createRenderer } from '@/game/Scene';
import { GameLoop } from '@/engine/GameLoop';
import { InputManager, shouldIgnoreGameKeyDown } from '@/input/InputManager';
import { Viewport } from '@/ui/Viewport';
import { LogoSplash } from '@/ui/LogoSplash';
import { MainMenu, type MenuChoice } from '@/ui/MainMenu';
import { DifficultyUI } from '@/ui/DifficultyUI';
import { SelectionScene, SelectionState } from '@/game/SelectionScene';
import { SelectionUI } from '@/ui/SelectionUI';
import { CHARACTER_REGISTRY } from '@/game/CharacterRegistry';
import { HUD } from '@/ui/HUD';
import { MatchHud, type SideInfo } from '@/ui/MatchHud';
import { Panel, PANEL_LOCK_MS } from '@/ui/Panel';
import { attachLiveRegion, announce } from '@/ui/live';
import { PLAYER_COLORS } from '@/ui/markers';
import { SoundManager } from '@/audio/SoundManager';
import { SoundEffect } from '@/audio/SoundEffect';
import { RustEngine, SHIP_IDS, inputMask, maskInput, snapshotHash, EMPTY_INPUT, type Snapshot } from '@/rust/Engine';
import { BattleView } from '@/rust/BattleView';
import { CosmoNetplay, createRoomId, invitationURL, readInvite, type MatchStart, type Frame } from '@/netplay/CosmoNetplay';
import { watchLobby } from '@/netplay/CosmoLobby';
import { OnlineController } from '@/netplay/OnlineController';
import { OnlineView } from '@/netplay/OnlineView';
import { FleetBadges } from '@/netplay/FleetBadges';
import '@/ui/hud.css';
import '@/ui/main-menu.css';
import '@/ui/cosmo-ui.css';

// The upstream scene code loads its files from the site root; the game is at /cosmoclash/.
installAssetBase();

const DT=1000/60;
const viewport=new Viewport();
const container=viewport.container;
attachLiveRegion(container);
const renderer=createRenderer(container);
const battle=new BattleView();
const input=new InputManager();
const sound=SoundManager.getInstance();
sound.preload();
type AppState='loading'|'logo'|'menu'|'difficulty'|'selection'|'online'|'playing'|'roundOver'|'gameOver'|'error';
let state:AppState='loading';
let mode:'ai'|'local'|'online'='ai';
let engine:RustEngine;
let snapshot:Snapshot|null=null;
let menu:MainMenu|null=null;
let difficultyUI:DifficultyUI|null=null;
let selection:SelectionScene|null=null;
let selectionUI:SelectionUI|null=null;
let badges:FleetBadges|null=null;
let hud:HUD|null=null;
let matchHud:MatchHud|null=null;
let panel:Panel|null=null;
let message:HTMLDivElement|null=null;
let difficulty:'easy'|'medium'|'hard'='medium';
let selected:[number,number]=[0,1];
let selectingPlayer:0|1=0;
// Play vs Computer: the computer's three ships one after the other; the player picks a new ship after each loss.
let aiRemaining:number[]=[];
let aiDefeated:number[]=[];
let playerLost:number[]=[];
let fleetReselect=false;
let lastInputs:[number,number]=[0,0];
let accumulator=0;
let roundDelay=0;
let winner:1|2=1;
let paused=false;
let onlineMatch:MatchStart|null=null;
let fleetIndex:[number,number]=[0,0];
let battleIndex=0;
let nextOnlineBattle=false;
let roundSeed=1;
let frames=0;
let lastHash='';
let netSession:CosmoNetplay|null=null;
let fullscreenAsked=false;
const seed=()=>crypto.getRandomValues(new Uint32Array(1))[0]||1;
const shipName=(index:number)=>CHARACTER_REGISTRY[index]?.name??'?';
const DIFFICULTY_NAMES={easy:'Easy',medium:'Medium',hard:'Hard'};
/** Two Players on one computer: Space is the fire key of Player 2, so only Enter chooses in the menus. */
const chooseKey=(key:string)=>key==='Enter'||(key===' '&&mode!=='local');

/** The first menu choice is a user gesture: the game frame goes full screen then (once). */
function fullscreenGesture():Promise<boolean> {
  if(fullscreenAsked)return Promise.resolve(true);
  fullscreenAsked=true;
  return viewport.requestFullscreen().then(ok=>{if(!ok)fullscreenAsked=false;return ok;});
}
function clearMessage():void { message?.remove();message=null; }
function showMessage(text:string,button?:{label:string;run:()=>void},title=''):void {
  clearMessage();message=document.createElement('div');message.className='cosmo-message';message.setAttribute('role','alert');
  if(title){const h=document.createElement('h1');h.className='cosmo-message-title';h.textContent=title;message.appendChild(h);}
  const p=document.createElement('p');p.textContent=text;message.appendChild(p);
  if(button){const b=document.createElement('button');b.className='cc-btn is-focus';b.textContent=button.label;b.onclick=button.run;message.appendChild(b);setTimeout(()=>b.focus(),0);}
  container.appendChild(message);
}
function fail(error:unknown):void {
  console.error(error);state='error';online.reset();
  showMessage(error instanceof Error?error.message:'The game could not continue.',engine?{label:'Return to menu',run:()=>showMenu()}:{label:'Reload game',run:()=>location.reload()},'Game stopped');
}
function clearPanel():void { panel?.destroy();panel=null;paused=false; }
function clearSelection():void {badges?.dispose();badges=null;selection?.dispose();selection=null;selectionUI?.destroy();selectionUI=null;}
function clearRound():void {battle.clear();hud?.destroy();hud=null;matchHud?.destroy();matchHud=null;snapshot=null;clearPanel();}
function resizeCameras():void {
  renderer.setSize(viewport.width,viewport.height);battle.camera.aspect=viewport.width/viewport.height;battle.camera.updateProjectionMatrix();
  if(selection){selection.camera.aspect=viewport.width/viewport.height;selection.camera.updateProjectionMatrix();}
}
viewport.onResize(resizeCameras);

// ------------------------------------------------------------------ menus
function showMenu(focus:MenuChoice=mode):void {
  online.reset();
  menu?.destroy();menu=null;difficultyUI?.destroy();difficultyUI=null;
  clearSelection();clearRound();clearMessage();accumulator=0;state='menu';onlineMatch=null;netSession=null;
  playerLost=[];aiDefeated=[];aiRemaining=[];fleetReselect=false;
  menu=new MainMenu(container,choice=>{
    void fullscreenGesture();
    menu?.destroy();menu=null;mode=choice;
    if(choice==='ai')showDifficulty();
    else if(choice==='local')showSelection(1);
    else {state='online';online.openLobby();}
  },focus);
  render();
  sound.playMusic('intro');
}
function showDifficulty():void {
  state='difficulty';
  difficultyUI=new DifficultyUI(container,level=>{
    difficulty=(['easy','medium','hard'] as const)[level-1];difficultyUI?.destroy();difficultyUI=null;
    playerLost=[];aiDefeated=[];fleetReselect=false;showSelection(0);
  },()=>showMenu('ai'));
}
function showSelection(player:0|1):void {
  clearSelection();clearMessage();clearPanel();state='selection';selectingPlayer=player;
  selection=new SelectionScene();selection.state=player===0?SelectionState.SELECTING_P1:SelectionState.SELECTING_P2;
  resizeCameras();
  if(fleetReselect){const used=new Set(playerLost.map(i=>CHARACTER_REGISTRY[i].id));selection.setDisabledShips(used);selection.jumpToIndex(CHARACTER_REGISTRY.findIndex(def=>!used.has(def.id)));}
  selectionUI=new SelectionUI(container,{prev:()=>browseSelection(-1),next:()=>browseSelection(1),choose:chooseSelection});
  if(mode==='ai'){
    selectionUI.setHeader(fleetReselect?'Choose Your Next Ship':'Choose Your Ship');
    selectionUI.setSubtitle(fleetReselect?`Your ${shipName(playerLost[playerLost.length-1])} was destroyed. The computer has ${aiRemaining.length+1} ship${aiRemaining.length?'s':''} left.`:`Play vs Computer · ${DIFFICULTY_NAMES[difficulty]}`);
  }else selectionUI.setSubtitle(player===1?'Player 2 keys: W A D to fly · Space fire · Shift special':'Player 1 keys: arrows to fly · / fire · . special');
  selectionUI.show(player+1,selection.getCurrentCharacter());
  if(fleetReselect)selectionUI.showRemaining(3-playerLost.length);
  sound.playMusic('intro');
}
function browseSelection(delta:1|-1):void {
  if(state!=='selection'||!selection||!selectionUI)return;
  if(delta<0)selection.selectPrev();else selection.selectNext();
  selectionUI.show(selectingPlayer+1,selection.getCurrentCharacter());
}
function chooseSelection():void {
  if(state!=='selection'||!selection)return;
  selected[selectingPlayer]=CHARACTER_REGISTRY.findIndex(def=>def.id===selection!.getCurrentCharacter().id);
  if(mode==='local'&&selectingPlayer===1){showSelection(0);return;}
  if(mode==='ai'&&!fleetReselect){const other=Math.floor(Math.random()*3);selected[1]=other;aiRemaining=[0,1,2].filter(n=>n!==other);playerLost=[];aiDefeated=[];}
  fleetReselect=false;startBattle(seed());
}
function startBattle(value:number):void {
  clearSelection();clearMessage();clearRound();roundSeed=value>>>0;
  snapshot=engine.init(roundSeed,[SHIP_IDS[selected[0]],SHIP_IDS[selected[1]]],mode==='ai'?difficulty:undefined);
  battle.start([SHIP_IDS[selected[0]],SHIP_IDS[selected[1]]]);hud=new HUD(container);
  const info=mode==='local'?'P1: arrows fly, / fire, . special   ·   P2: W A D fly, Space fire, Shift special':mode==='ai'?`vs Computer · ${DIFFICULTY_NAMES[difficulty]}`:'';
  matchHud=new MatchHud(container,{exitLabel:mode==='online'?'Leave (Esc)':'Menu (Esc)',info,onExit:()=>{if(mode==='online')online.activate('leave');else pauseLocal();}});
  // Who is who: rings in the player colours for the first three seconds.
  if(mode==='online')battle.mark(onlineMatch?.localPlayer??0,180);
  else if(mode==='ai')battle.mark(0,180);
  else battle.mark(null,180,['P1','P2']);
  state='playing';lastInputs=[0,0];accumulator=0;roundDelay=0;
  battle.update(snapshot,lastInputs,DT);updateHud();sound.playMusic('battle');
  announce(`${shipName(selected[0])} against ${shipName(selected[1])}.`);
}
function pauseLocal():void {
  if(mode==='online'||(state!=='playing'&&state!=='roundOver')||panel)return;
  paused=true;
  panel=new Panel(container,{title:'Paused',lines:[mode==='ai'?'Leaving ends this game against the computer.':'Leaving ends this battle.'],
    buttons:[{id:'resume',label:'Resume'},{id:'menu',label:'Main menu'}],escape:'resume',dialog:true,hint:'Enter choose · Esc resume',
    chooseKeys:mode==='local'?['Enter']:undefined,
    onChoose:id=>{if(id==='resume'){clearPanel();accumulator=0;}else showMenu();}});
}
/** Esc on "Choose Your Next Ship" (vs Computer): the game is not lost without a question. */
function askLeaveGame():void {
  if(panel)return;
  panel=new Panel(container,{title:'Leave this game?',lines:[`Leaving ends this game against the computer (${DIFFICULTY_NAMES[difficulty]}).`],
    buttons:[{id:'stay',label:'Keep playing'},{id:'menu',label:'Main menu'}],escape:'stay',dialog:true,hint:'Enter choose · Esc keep playing',
    onChoose:id=>{if(id==='stay')clearPanel();else showMenu('ai');}});
}
function localGameOver():void {
  state='gameOver';matchHud?.message('');
  const ai=mode==='ai';
  const won=winner===1;
  panel=new Panel(container,{
    title:ai?(won?'Victory':'Defeat'):`Player ${winner} wins`,titleClass:ai?(won?'won':'lost'):'won',
    lines:ai?[won?`You destroyed all three computer ships (${DIFFICULTY_NAMES[difficulty]}).`:'The computer destroyed all your ships.']:[`${shipName(selected[winner-1])} wins the battle.`],
    buttons:[{id:'again',label:'Play again'},{id:'menu',label:'Main menu'}],escape:'menu',hint:'Enter choose · Esc main menu',
    // It opens by itself: a fire key that the players still press does not choose.
    lockMs:PANEL_LOCK_MS,chooseKeys:ai?undefined:['Enter'],
    onChoose:id=>{if(id==='menu'){showMenu();return;}clearRound();if(ai){playerLost=[];aiDefeated=[];fleetReselect=false;showSelection(0);}else showSelection(1);},
  });
}

// ------------------------------------------------------------------ battles
/** Who is who and the fleets, for the battle text. After a kill the destroyed ship shows as lost at once. */
function sides():[SideInfo,SideInfo] {
  const over=state==='roundOver'||state==='gameOver';
  const loser=over?(winner===1?1:0):-1;
  if(mode==='online'&&onlineMatch){
    const local=onlineMatch.localPlayer;
    return [0,1].map(i=>({player:`P${i+1}`,tag:i===local?'You':'Opponent',fleet:onlineMatch!.fleets[i],current:fleetIndex[i]+(loser===i?1:0)})) as [SideInfo,SideInfo];
  }
  if(mode==='ai'){
    // The lost and defeated lists already have the ship of the last kill; the next ships are not known yet.
    const mine=[...playerLost,...(loser===0?[]:[selected[0]])];
    const theirs=[...aiDefeated,...(loser===1?[]:[selected[1]])];
    return [
      {player:'P1',tag:'You',fleet:[...mine,...Array(Math.max(0,3-mine.length)).fill(-1)],current:playerLost.length},
      {player:'P2',tag:'Computer',fleet:[...theirs,...aiRemaining.map(()=>-1)],current:aiDefeated.length},
    ];
  }
  return [0,1].map(i=>({player:`P${i+1}`,tag:`Player ${i+1}`,fleet:[selected[i]],current:loser===i?1:0})) as [SideInfo,SideInfo];
}
function updateHud():void {
  if(!snapshot)return;const [a,b]=snapshot.players;hud?.update(a.health,a.energy,b.health,b.energy);
  matchHud?.setSides(sides());
}
function hashState():string {
  if(!snapshot)return '';
  // Include the fleet controller so divergence cannot hide between battles.
  return snapshotHash({...snapshot, match:{fleetIndex,battleIndex,roundDelay,nextOnlineBattle,roundSeed}} as Snapshot);
}
function roundMessage():void {
  if(!matchHud)return;
  const loser=(winner===1?1:0) as 0|1;
  const lostShip=shipName(selected[loser]);
  if(mode==='online'&&onlineMatch){
    const next=onlineMatch.fleets[loser][fleetIndex[loser]+1];
    if(loser===onlineMatch.localPlayer)matchHud.message(`Your ${lostShip} was destroyed`,next!==undefined?`Next: ${shipName(next)}`:'Your fleet is destroyed.');
    else matchHud.message(`Enemy ${lostShip} destroyed!`,next!==undefined?`Next enemy ship: ${shipName(next)}`:'The enemy fleet is destroyed.');
  }else if(mode==='ai'){
    if(loser===1)matchHud.message(`Enemy ${lostShip} destroyed!`,aiRemaining.length?`The computer has ${aiRemaining.length} ship${aiRemaining.length===1?'':'s'} left.`:'All computer ships are destroyed.');
    else matchHud.message(`Your ${lostShip} was destroyed`,playerLost.length<3?'Choose your next ship.':'You have no ships left.');
  }else matchHud.message(`Player ${winner} wins!`,`${lostShip} was destroyed.`);
  announce(matchHud?`${winner===1?'Player 1':'Player 2'} wins the round.`:'');
}
function startOnline(start:MatchStart):void {
  mode='online';onlineMatch=start;fleetIndex=[0,0];battleIndex=0;nextOnlineBattle=false;
  selected=[start.fleets[0][0],start.fleets[1][0]];startBattle(start.seed);
}
function onlineFrame(frame:Frame):void {
  if(!onlineMatch||!snapshot||!netSession)return;
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
        lastHash=hashState();netSession.reportChecksum(frame.tick,lastHash);netSession.finish(winner);return;
      }
      fleetIndex[lost]++;nextOnlineBattle=true;
    }
  }else if(snapshot.winner){winner=snapshot.winner as 1|2;state='roundOver';roundDelay=0;roundMessage();}
  lastHash=hashState();if(frame.tick%30===0)netSession.reportChecksum(frame.tick,lastHash);
  battle.update(snapshot,lastInputs,DT);updateHud();
}
function localTick():void {
  if(!snapshot)return;
  const a=input.player(0),b=mode==='local'?input.player(1):EMPTY_INPUT;
  snapshot=engine.step([a,b],DT);frames++;
  lastInputs=[inputMask(a),inputMask(snapshot.aiController?.out??b)];
  if(state==='playing'&&snapshot.winner){
    winner=snapshot.winner as 1|2;state='roundOver';roundDelay=0;
    if(mode==='ai'&&winner===2)playerLost.push(selected[0]);
    if(mode==='ai'&&winner===1)aiDefeated.push(selected[1]);
  }
  else if(state==='roundOver'){
    roundDelay++;
    if(roundDelay===90)roundMessage();
    if(roundDelay>=390){
      if(mode==='ai'&&winner===1&&aiRemaining.length){const index=Math.floor(Math.random()*aiRemaining.length);selected[1]=aiRemaining.splice(index,1)[0];startBattle(seed());return;}
      if(mode==='ai'&&winner===2&&playerLost.length<3){clearRound();fleetReselect=true;showSelection(0);return;}
      localGameOver();
    }
  }
  battle.update(snapshot,lastInputs,DT);updateHud();lastHash=hashState();
}

// ------------------------------------------------------------------ online
function copyText(text:string):Promise<void> {
  // A hidden field and the copy command work in more places (also over plain HTTP).
  try {
    const field=document.createElement('textarea');field.value=text;field.setAttribute('readonly','');
    field.style.cssText='position:fixed;left:-9999px;top:0;opacity:0';
    (document.fullscreenElement??document.body).appendChild(field);field.select();
    const copied=document.execCommand?.('copy');field.remove();
    if(copied)return Promise.resolve();
  } catch { /* Try the clipboard API. */ }
  return navigator.clipboard?.writeText?navigator.clipboard.writeText(text):Promise.reject(new Error('No clipboard'));
}
const storage={
  get(key:string){try{return localStorage.getItem(key);}catch{return null;}},
  set(key:string,value:string){try{localStorage.setItem(key,value);}catch{/* Private mode. */}},
};
const online=new OnlineController({
  now:()=>Date.now(),setTimeout:(fn,ms)=>window.setTimeout(fn,ms),clearTimeout:id=>window.clearTimeout(id as number),
  random:Math.random,storage,origin:location.href,
  createToken:createRoomId,
  watchLobby:handlers=>watchLobby({origin:location.href,...handlers}),
  createSession:handlers=>{
    netSession=new CosmoNetplay({buildId:`cosmo2:${engine.buildId}`,onState:handlers.onState,onStart:handlers.onStart,onEnd:handlers.onEnd,onFrame:onlineFrame});
    return netSession;
  },
  inviteURL:token=>invitationURL(token),
  copy:copyText,
  fullscreen:fullscreenGesture,
  prepare:()=>battle.prepare(),
  sound:name=>{if(name==='found'){sound.play(SoundEffect.MorphOpen);if(document.hidden)document.title='Opponent found! · Cosmo Clash';}},
  clearInvite:()=>{try{const url=new URL(location.href);url.searchParams.delete('cosmoRoom');history.replaceState(null,'',url.href);}catch{/* Keep the address. */}},
  game:{
    menu:()=>showMenu('online'),
    start:startOnline,
    stop:()=>{clearSelection();clearRound();onlineMatch=null;},
    lost:()=>[fleetIndex[0]+(winner===2?1:0),fleetIndex[1]+(winner===1?1:0)],
  },
  changed:()=>render(),
});
const onlineView=new OnlineView(container,online);
document.addEventListener('visibilitychange',()=>{if(!document.hidden)document.title='CosmoClash';});

/** Online screens: the view, and the 3D ship choice of the fleet screen. */
function render():void {
  onlineView.render();
  const s=online.state;
  if(s.screen==='hidden')return;
  mode='online';
  if(s.screen==='fleet'){
    state='online';
    if(!selection){
      selection=new SelectionScene();selection.state=SelectionState.SELECTING_P1;resizeCameras();
      selection.jumpToIndex(s.fleet.carousel);badges=new FleetBadges(selection.scene);sound.playMusic('intro');
    }else{
      const delta=(s.fleet.carousel-selection.currentIndex+3)%3;
      if(delta===1)selection.selectNext();else if(delta===2)selection.selectPrev();
    }
    badges?.set(s.fleet.picks,PLAYER_COLORS[s.room?.role==='guest'?1:0]);
  }else if(selection)clearSelection();
  // The result screen covers the battle: its last message goes.
  if(s.screen==='result'){state='gameOver';matchHud?.message('');}
  else if(['lobby','name','waiting','joining','invite'].includes(s.screen)){state='online';if(snapshot)clearRound();}
}

// ------------------------------------------------------------------ keys
/** One handler for every menu: one action per key press (held-key repeats do nothing). */
function menuKey(e:KeyboardEvent):boolean {
  const key=e.key;
  if(state==='loading'||state==='logo')return false;
  if(panel)return e.repeat||panel.key(key);
  if(online.visible){
    const inBattle=(online.state.screen==='battle'||online.state.screen==='countdown')&&!online.state.confirm;
    if(e.repeat)return !inBattle||key==='Escape';
    return online.key(key);
  }
  if(state==='menu'&&menu)return e.repeat||menu.key(key);
  if(state==='difficulty'&&difficultyUI){
    if(e.repeat)return true;
    if(key==='Escape'){showMenu('ai');return true;}
    return difficultyUI.handleKey(key);
  }
  if(state==='selection'&&selection&&selectionUI){
    if(e.repeat)return ['ArrowLeft','ArrowRight','Enter','Escape'].includes(key)||chooseKey(key);
    if(key==='ArrowLeft'){browseSelection(-1);return true;}
    if(key==='ArrowRight'){browseSelection(1);return true;}
    if(chooseKey(key)){chooseSelection();return true;}
    if(key==='Escape'){if(mode==='ai'&&fleetReselect)askLeaveGame();else showMenu(mode);return true;}
    return false;
  }
  if((state==='playing'||state==='roundOver')&&mode!=='online'){
    if(key==='Escape'){if(!e.repeat)pauseLocal();return true;}
    return false;
  }
  if(state==='error'&&message&&(key==='Enter'||key===' ')&&!e.repeat){message.querySelector('button')?.click();return true;}
  return false;
}
window.addEventListener('keydown',e=>{
  if(shouldIgnoreGameKeyDown(e))return;
  // The game's handlers make every Enter and Space choice. The browser's own
  // action would click the button that has the focus by then (the key that
  // skips the logo would also start a game), so it never runs.
  if(e.key==='Enter'||e.key===' ')e.preventDefault();
  if(e.key.toLowerCase()==='m'&&!e.repeat&&state!=='loading')sound.toggleMute();
  if(menuKey(e)){e.preventDefault();e.stopPropagation();}
},true);
// A focused button also reacts to Space on key release: the menus already used the press.
window.addEventListener('keyup',e=>{if((e.key===' '||e.key==='Enter')&&(e.target as HTMLElement)?.closest?.('button'))e.preventDefault();},true);

const gameLoop=new GameLoop(dt=>{
  if(selection)selection.update(dt);
  if(paused)return;
  if(state!=='playing'&&state!=='roundOver'&&!(mode==='online'&&online.running))return;
  accumulator=Math.min(accumulator+dt,DT*6);
  try{
    for(let n=0;accumulator>=DT&&n<6;n++){
      if(mode==='online'){
        if(!online.running||!netSession)break;
        // The question "Leave the match?" is open: the ship gets no keys meanwhile.
        const mask=online.state.confirm?0:inputMask(input.player(0));
        if(!netSession.advance(mask)){accumulator=Math.min(accumulator,DT);break;}
      }else localTick();
      accumulator=Math.max(0,accumulator-DT);
      if(mode!=='online'&&state!=='playing'&&state!=='roundOver')break;
    }
  }catch(error){fail(error);}
},()=>{
  if(selection)renderer.render(selection.scene,selection.camera);
  else if(snapshot)renderer.render(battle.scene,battle.camera);
  else renderer.clear();
});
// Read-only diagnostics let tests verify actual rendered engine state without test cheats.
Object.defineProperty(window,'__cosmo',{value:Object.freeze({
  get state(){return state;},get mode(){return mode;},get engine(){return engine?.buildId;},
  get snapshot(){return snapshot?structuredClone(snapshot):null;},get hash(){return lastHash;},
  get frames(){return frames;},get netplay(){return netSession?.state??null;},
  get fleet(){return {indices:[...fleetIndex],battle:battleIndex};},
  get online(){const s=online.state;return {screen:s.screen,focus:s.focus,items:[...s.items],confirm:s.confirm,lobby:s.lobby.status,rooms:s.lobby.rooms.map(r=>r.name),notice:s.notice?.text??'',picks:[...s.fleet.picks],carousel:s.fleet.carousel,room:s.room?.name??null,opening:s.opening};},
  get fullscreen(){return document.fullscreenElement?.id??null;},
  get paused(){return paused;},
}),writable:false});
showMessage('Loading ships and game engine…');
Promise.all([RustEngine.load(),battle.prepare()]).then(([loaded])=>{
  engine=loaded;clearMessage();gameLoop.start();state='logo';new LogoSplash(container,()=>{
    const invite=readInvite(location.href);
    if(invite){mode='online';state='online';online.invite(invite);}else showMenu('ai');
  });
}).catch(fail);
