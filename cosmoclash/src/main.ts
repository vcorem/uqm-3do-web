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
import { CosmoNetplay, createRoomId, invitationURL, readInvite, type MatchStart, type Frame, type NextBattle } from '@/netplay/CosmoNetplay';
import { watchLobby } from '@/netplay/CosmoLobby';
import { OnlineController } from '@/netplay/OnlineController';
import { OnlineView } from '@/netplay/OnlineView';
import { FleetBadges } from '@/netplay/FleetBadges';
import { FleetMatch, battleSeed, type FleetView } from '@/match/FleetMatch';
import { FleetPicker, type PickResult } from '@/match/FleetPicker';
import { FleetPickView } from '@/ui/FleetPickView';
import { KeysHelp } from '@/ui/KeysHelp';
import { fleetLine } from '@/ui/fleetChips';
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
/** The keys beside the game (wide windows, full screen) or over it: K or the Keys (K) button, in every screen. */
const keys=new KeysHelp({frame:viewport.frame,container});
viewport.onResize(()=>keys.layout());
const battle=new BattleView();
const input=new InputManager();
const sound=SoundManager.getInstance();
sound.preload();
type AppState='loading'|'logo'|'menu'|'difficulty'|'fleet'|'selection'|'online'|'playing'|'roundOver'|'gameOver'|'error';
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
/**
 * The fleets of this game, in all three modes (src/match/FleetMatch.ts): Play
 * vs Computer (all three ships each; the computer takes its next ship at
 * random), Two Players (the fleets of the fleet choice) and Online Melee. The
 * loser of a battle with two or more ships left chooses the next one.
 */
let match:FleetMatch|null=null;
/** Local play: the player who chooses the next ship on the ship choice (null: the first ship vs the computer). */
let nextChooser:0|1|null=null;
/** Two Players: the fleet choice of one player (Player 2 first), and the fleets so far. */
let picker:FleetPicker|null=null;
let pickView:FleetPickView|null=null;
let picking:0|1=1;
let localFleets:[number[],number[]]=[[],[]];
let lastInputs:[number,number]=[0,0];
let accumulator=0;
let roundDelay=0;
let winner:1|2=1;
let paused=false;
let onlineMatch:MatchStart|null=null;
let battleIndex=0;
let nextOnlineBattle=false;
let roundSeed=1;
let frames=0;
let lastHash='';
let netSession:CosmoNetplay|null=null;
const seed=()=>crypto.getRandomValues(new Uint32Array(1))[0]||1;
const shipName=(index:number|null|undefined)=>CHARACTER_REGISTRY[index??-1]?.name??'?';
const plural=(n:number,word:string)=>`${n} ${word}${n===1?'':'s'}`;
const EMPTY_FLEET:FleetView={ships:[],lost:0,current:-1};
/** The ship of this player that was destroyed last. */
const lastLost=(player:0|1)=>match?.lost(player).at(-1)??-1;
const DIFFICULTY_NAMES={easy:'Easy',medium:'Medium',hard:'Hard'};

/**
 * A menu choice is a user gesture: the game frame goes full screen then, also
 * again after Esc or the browser left full screen. F (or the menu button) turns
 * it off for good, and on again.
 */
const fullscreenGesture=():Promise<boolean>=>viewport.fullscreenGesture();
const fullscreenToggle={active:()=>viewport.fullscreen,toggle:()=>{void viewport.toggleFullscreen();}};
/** Two Players on one computer: Space is the fire key of Player 2, so only Enter chooses in the menus. */
const chooseKey=(key:string)=>key==='Enter'||(key===' '&&mode!=='local');
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
function clearSelection():void {badges?.dispose();badges=null;selection?.dispose();selection=null;selectionUI?.destroy();selectionUI=null;pickView?.destroy();pickView=null;picker=null;}
function clearRound():void {battle.clear();hud?.destroy();hud=null;matchHud?.destroy();matchHud=null;snapshot=null;clearPanel();}
// The online fleet screen has its picks panel under the ship: the ship shows a little
// higher there (a fraction of the frame height), in the space between the name box and the panel.
const FLEET_LIFT=0.05;
let selectionLift=0;
function resizeCameras():void {
  renderer.setSize(viewport.width,viewport.height);battle.camera.aspect=viewport.width/viewport.height;battle.camera.updateProjectionMatrix();
  if(selection){
    selection.camera.aspect=viewport.width/viewport.height;
    if(selectionLift)selection.camera.setViewOffset(viewport.width,viewport.height,0,Math.round(viewport.height*selectionLift),viewport.width,viewport.height);
    else selection.camera.clearViewOffset();
    selection.camera.updateProjectionMatrix();
  }
}
viewport.onResize(resizeCameras);
/** The 3D ship choice turns to this ship (with the fly animation when it is next to the current one). */
function turnSelection(index:number):void {
  if(!selection||selection.currentIndex===index)return;
  const delta=(index-selection.currentIndex+3)%3;
  if(delta===1)selection.selectNext();else selection.selectPrev();
  if(selection.currentIndex!==index)selection.jumpToIndex(index);
}

// ------------------------------------------------------------------ menus
function showMenu(focus:MenuChoice=mode):void {
  online.reset();
  menu?.destroy();menu=null;difficultyUI?.destroy();difficultyUI=null;
  clearSelection();clearRound();clearMessage();accumulator=0;state='menu';onlineMatch=null;netSession=null;
  match=null;nextChooser=null;keys.setMode('menu');
  menu=new MainMenu(container,choice=>{
    void fullscreenGesture();
    menu?.destroy();menu=null;mode=choice;keys.setMode(choice);
    if(choice==='ai')showDifficulty();
    else if(choice==='local')showFleetPick(1);
    else {state='online';online.openLobby();}
  },focus,fullscreenToggle);
  render();
  sound.playMusic('intro');
}
function showDifficulty():void {
  state='difficulty';
  difficultyUI=new DifficultyUI(container,level=>{
    difficulty=(['easy','medium','hard'] as const)[level-1];difficultyUI?.destroy();difficultyUI=null;
    match=null;nextChooser=null;showSelection(0);
  },()=>showMenu('ai'));
}
/**
 * The 3D ship choice of local play: the first ship against the computer, or
 * "Choose Your Next Ship" (nextChooser) after a loss with two or more ships
 * left, with the destroyed ships and the ships that are not in the fleet hidden.
 */
function showSelection(player:0|1):void {
  clearSelection();clearMessage();clearPanel();state='selection';selectingPlayer=player;
  selectionLift=0;selection=new SelectionScene();selection.state=player===0?SelectionState.SELECTING_P1:SelectionState.SELECTING_P2;
  resizeCameras();
  const left=match&&nextChooser!==null?match.left(nextChooser):null;
  if(left){selection.setDisabledShips(new Set(CHARACTER_REGISTRY.filter((_,i)=>!left.includes(i)).map(def=>def.id)));selection.jumpToIndex(left[0]);}
  selectionUI=new SelectionUI(container,{prev:()=>browseSelection(-1),next:()=>browseSelection(1),choose:chooseSelection});
  if(mode==='ai'){
    selectionUI.setHeader(left?'Choose Your Next Ship':'Choose Your Ship');
    selectionUI.setSubtitle(left?`Your ${shipName(lastLost(0))} was destroyed. The computer has ${plural(match!.alive(1),'ship')} left.`:`Play vs Computer · ${DIFFICULTY_NAMES[difficulty]}`);
  }else if(left){
    const other=(1-player) as 0|1;
    selectionUI.setHeader(`Player ${player+1}: Choose Your Next Ship`);
    // Space is the fire key of Player 2: only Enter chooses (chooseKey), as the hint says.
    selectionUI.setSubtitle(`Your ${shipName(lastLost(player))} was destroyed. Player ${other+1} has ${plural(match!.alive(other),'ship')} left.`);
  }
  if(left){
    // Esc asks "Leave this game?" here (askLeaveGame); both fleets as chips, as on the online choice.
    selectionUI.setHint('← → Browse  |  Enter to Select  |  Esc Leave');
    selectionUI.setFleets(fleetLines());
  }
  selectionUI.show(player+1,selection.getCurrentCharacter());
  if(left)selectionUI.showRemaining(left.length);
  sound.playMusic('intro');
}
/** Two Players: the fleet choice of one player (Player 2 first, then Player 1), over the 3D ships. */
function showFleetPick(player:0|1):void {
  clearSelection();clearMessage();clearPanel();state='fleet';picking=player;match=null;nextChooser=null;
  if(player===1)localFleets=[[],[]];
  selectionLift=FLEET_LIFT;selection=new SelectionScene();selection.state=SelectionState.SELECTING_P1;resizeCameras();
  badges=new FleetBadges(selection.scene);
  picker=new FleetPicker(['Enter']);
  pickView=new FleetPickView(container,player,picker,{changed:syncFleetPick,result:fleetPickResult});
  syncFleetPick();
  sound.playMusic('intro');
}
function syncFleetPick():void {
  if(!picker||!selection)return;
  turnSelection(picker.carousel);
  badges?.set(picker.picks,PLAYER_COLORS[picking]);
  pickView?.render();
}
function fleetPickResult(result:PickResult):void {
  if(!picker||state!=='fleet')return;
  if(result==='back'){showMenu('local');return;}
  if(result!=='done')return;
  localFleets[picking]=[...picker.picks];
  if(picking===1){showFleetPick(0);return;}
  match=new FleetMatch(localFleets);battleIndex=0;
  startMatchBattle();
}
function browseSelection(delta:1|-1):void {
  if(state!=='selection'||!selection||!selectionUI)return;
  if(delta<0)selection.selectPrev();else selection.selectNext();
  selectionUI.show(selectingPlayer+1,selection.getCurrentCharacter());
}
function chooseSelection():void {
  if(state!=='selection'||!selection)return;
  const ship=CHARACTER_REGISTRY.findIndex(def=>def.id===selection!.getCurrentCharacter().id);
  if(match&&nextChooser!==null){match.choose(nextChooser,ship);nextChooser=null;battleIndex++;startMatchBattle();return;}
  if(mode!=='ai')return;
  // Play vs Computer: all three ships on both sides; the computer starts with one at random.
  const ai=Math.floor(Math.random()*3);
  match=new FleetMatch([[ship,...[0,1,2].filter(n=>n!==ship)],[ai,...[0,1,2].filter(n=>n!==ai)]]);battleIndex=0;
  startMatchBattle();
}
/** The next local battle with the ships in battle of the fleet match. */
function startMatchBattle():void {
  if(!match)return;
  selected=[match.current(0)!,match.current(1)!];
  startBattle(seed());
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
  panel=new Panel(container,{title:'Paused',lines:[mode==='ai'?'Leaving ends this game against the computer.':'Leaving ends this fleet match.'],
    buttons:[{id:'resume',label:'Resume'},{id:'menu',label:'Main menu'}],escape:'resume',dialog:true,hint:'Enter choose · Esc resume · K keys',
    chooseKeys:mode==='local'?['Enter']:undefined,
    onChoose:id=>{if(id==='resume'){clearPanel();accumulator=0;}else showMenu();}});
}
/** Esc on "Choose Your Next Ship": the game is not lost without a question. */
function askLeaveGame():void {
  if(panel)return;
  const ai=mode==='ai';
  panel=new Panel(container,{title:'Leave this game?',lines:[ai?`Leaving ends this game against the computer (${DIFFICULTY_NAMES[difficulty]}).`:'Leaving ends this fleet match of Player 1 and Player 2.'],
    buttons:[{id:'stay',label:'Keep playing'},{id:'menu',label:'Main menu'}],escape:'stay',dialog:true,hint:'Enter choose · Esc keep playing',
    chooseKeys:ai?undefined:['Enter'],
    onChoose:id=>{if(id==='stay')clearPanel();else showMenu(mode);}});
}
/** The fleets of the game for a panel: the ships in the order they flew, destroyed ones struck through. */
function fleetLines():HTMLElement[] {
  const [a,b]=sides();
  return [fleetLine(mode==='ai'?'You':'Player 1',0,{ships:a.fleet,lost:a.lost,current:a.current}),
    fleetLine(mode==='ai'?'Computer':'Player 2',1,{ships:b.fleet,lost:b.lost,current:b.current})];
}
function localGameOver():void {
  state='gameOver';matchHud?.message('');
  const ai=mode==='ai';
  const won=(match?.winner??winner)===1;
  const loser=won?2:1;
  panel=new Panel(container,{
    title:ai?(won?'Victory':'Defeat'):`Player ${won?1:2} wins`,titleClass:ai?(won?'won':'lost'):'won',
    lines:ai?[won?`You destroyed all three computer ships (${DIFFICULTY_NAMES[difficulty]}).`:'The computer destroyed all your ships.']:[`Player ${loser} has no ships left.`],
    content:fleetLines(),
    buttons:[{id:'again',label:'Play again'},{id:'menu',label:'Main menu'}],escape:'menu',hint:'Enter choose · Esc main menu',
    // It opens by itself: a fire key that the players still press does not choose.
    lockMs:PANEL_LOCK_MS,chooseKeys:ai?undefined:['Enter'],
    onChoose:id=>{if(id==='menu'){showMenu();return;}clearRound();match=null;nextChooser=null;if(ai)showSelection(0);else showFleetPick(1);},
  });
}

// ------------------------------------------------------------------ battles
/**
 * Who is who and the fleets, for the battle text: the ships in the order they
 * flew, the destroyed ships first (after a kill the ship shows as lost at once).
 * Against the computer its ships that did not fly yet are '?'.
 */
function sides():[SideInfo,SideInfo] {
  const views=match?match.views():[EMPTY_FLEET,EMPTY_FLEET];
  const local=onlineMatch?.localPlayer??0;
  const tags=mode==='online'?(local===0?['You','Opponent']:['Opponent','You']):mode==='ai'?['You','Computer']:['Player 1','Player 2'];
  return views.map((view,i)=>{
    let fleet=view.ships;
    if(mode==='ai'&&i===1)fleet=fleet.map((ship,n)=>n<view.lost||ship===selected[1]?ship:-1);
    return {player:`P${i+1}`,tag:tags[i],fleet,lost:view.lost,current:view.current};
  }) as [SideInfo,SideInfo];
}
function updateHud():void {
  if(!snapshot)return;const [a,b]=snapshot.players;hud?.update(a.health,a.energy,b.health,b.energy);
  matchHud?.setSides(sides());
}
function hashState():string {
  if(!snapshot)return '';
  // Include the fleet match (the chosen ships too) so divergence cannot hide between battles.
  return snapshotHash({...snapshot, match:{fleets:match?.state()??null,battleIndex,roundDelay,nextOnlineBattle,roundSeed}} as Snapshot);
}
/** The message after a battle: the destroyed ship, and what comes next (a choice, the last ship, the end). */
function roundMessage():void {
  if(!matchHud||!match)return;
  const loser=(winner===1?1:0) as 0|1;
  const lostShip=shipName(lastLost(loser));
  const over=match.over, choose=match.chooser===loser, next=shipName(match.current(loser));
  let text:string, sub:string;
  if(mode==='online'&&onlineMatch){
    if(loser===onlineMatch.localPlayer){text=`Your ${lostShip} was destroyed`;sub=over?'Your fleet is destroyed.':choose?'Choose your next ship.':`Your last ship: ${next}`;}
    else{text=`Enemy ${lostShip} destroyed!`;sub=over?'The enemy fleet is destroyed.':choose?'Your opponent chooses the next ship.':`Their last ship: ${next}`;}
  }else if(mode==='ai'){
    const left=match.alive(1);
    if(loser===1){text=`Enemy ${lostShip} destroyed!`;sub=left?`The computer has ${plural(left,'ship')} left.`:'All computer ships are destroyed.';}
    else{text=`Your ${lostShip} was destroyed`;sub=over?'You have no ships left.':choose?'Choose your next ship.':`Your last ship: ${next}`;}
  }else if(over){text=`Player ${winner} wins!`;sub=`Player ${loser+1} has no ships left.`;}
  else{text=`Player ${winner} wins the battle`;sub=choose?`Player ${loser+1}: choose your next ship.`:`Player ${loser+1}'s last ship: ${next}`;}
  matchHud.message(text,sub);
  announce(`${text}. ${sub}`);
}
function startOnline(start:MatchStart):void {
  mode='online';onlineMatch=start;match=new FleetMatch(start.fleets);battleIndex=0;nextOnlineBattle=false;
  selected=[match.current(0)!,match.current(1)!];startBattle(battleSeed(start.seed,0));
}
/** The next online battle: the ships in battle of the fleet match and a seed from the match seed and the battle number. */
function nextOnlineBattleStart():void {
  if(!match||!onlineMatch)return;
  battleIndex++;
  selected=[match.current(0)!,match.current(1)!];
  startBattle(battleSeed(onlineMatch.seed,battleIndex));
}
/**
 * Both peers have the loser's choice (protocol 3), between the frame of the
 * pause and the next frame: the same ship and the same next battle on both.
 */
function onlineNext(next:NextBattle):void {
  if(!match||!onlineMatch)throw new Error('No online match.');
  match.choose(next.player,next.ship);
  nextOnlineBattleStart();
}
function onlineFrame(frame:Frame):void {
  if(!onlineMatch||!snapshot||!netSession||!match)return;
  // The last ship came by itself: the next battle starts at this frame.
  if(nextOnlineBattle){nextOnlineBattle=false;nextOnlineBattleStart();}
  lastInputs=frame.inputs;
  snapshot=engine.step(frame.inputs.map(maskInput) as [ReturnType<typeof maskInput>,ReturnType<typeof maskInput>],DT);frames++;
  let pause:{battle:number;player:0|1;ships:number[]}|null=null;
  if(state==='roundOver'){
    roundDelay++;
    if(roundDelay>=90){
      if(match.over){
        lastHash=hashState();netSession.reportChecksum(frame.tick,lastHash);netSession.finish(winner);return;
      }
      // Two or more ships left: the loser chooses (both peers stop here); one ship left: it comes by itself.
      const chooser=match.chooser;
      if(chooser!==null)pause={battle:battleIndex+1,player:chooser,ships:match.left(chooser)};
      else nextOnlineBattle=true;
    }
  }else if(snapshot.winner){winner=snapshot.winner as 1|2;state='roundOver';roundDelay=0;match.lose(winner===1?1:0);roundMessage();}
  lastHash=hashState();
  // The state at the pause is checked too, before the choice.
  if(frame.tick%30===0||pause)netSession.reportChecksum(frame.tick,lastHash);
  battle.update(snapshot,lastInputs,DT);updateHud();
  if(pause)netSession.awaitNext(pause);
}
function localTick():void {
  if(!snapshot||!match)return;
  const a=input.player(0),b=mode==='local'?input.player(1):EMPTY_INPUT;
  snapshot=engine.step([a,b],DT);frames++;
  lastInputs=[inputMask(a),inputMask(snapshot.aiController?.out??b)];
  if(state==='playing'&&snapshot.winner){
    winner=snapshot.winner as 1|2;state='roundOver';roundDelay=0;
    match.lose(winner===1?1:0);
  }
  else if(state==='roundOver'){
    roundDelay++;
    if(roundDelay===90)roundMessage();
    if(roundDelay>=390){
      if(match.over){localGameOver();}
      else{
        const chooser=match.chooser;
        // The computer takes one of its ships left at random.
        if(chooser===1&&mode==='ai'){const left=match.left(1);match.choose(1,left[Math.floor(Math.random()*left.length)]);}
        else if(chooser!==null){clearRound();nextChooser=chooser;showSelection(chooser);return;}
        battleIndex++;startMatchBattle();return;
      }
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
    netSession=new CosmoNetplay({buildId:`cosmo3:${engine.buildId}`,onState:handlers.onState,onStart:handlers.onStart,onEnd:handlers.onEnd,onNext:handlers.onNext,onFrame:onlineFrame});
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
    stop:()=>{clearSelection();clearRound();onlineMatch=null;match=null;},
    fleets:()=>match?match.views():[EMPTY_FLEET,EMPTY_FLEET],
    next:onlineNext,
  },
  changed:()=>render(),
});
const onlineView=new OnlineView(container,online);
document.addEventListener('visibilitychange',()=>{if(!document.hidden)document.title='CosmoClash';});

/** Online screens: the view, and the 3D ship choice of the fleet screen and of the next ship. */
function render():void {
  onlineView.render();
  const s=online.state;
  // The loser chooses the next ship over the 3D ships: the battle bars and chips are not shown meanwhile.
  const choosing=s.screen==='next'&&!!s.next?.mine;
  container.classList.toggle('cc-choosing',choosing);
  if(s.screen==='hidden')return;
  mode='online';keys.setMode('online');
  if(s.screen==='fleet'){
    state='online';
    if(!selection){
      selection=new SelectionScene();selection.state=SelectionState.SELECTING_P1;selectionLift=FLEET_LIFT;resizeCameras();
      selection.jumpToIndex(s.fleet.carousel);badges=new FleetBadges(selection.scene);sound.playMusic('intro');
    }else{
      const delta=(s.fleet.carousel-selection.currentIndex+3)%3;
      if(delta===1)selection.selectNext();else if(delta===2)selection.selectPrev();
    }
    badges?.set(s.fleet.picks,PLAYER_COLORS[s.room?.role==='guest'?1:0]);
  }else if(choosing){
    // Destroyed ships and ships that are not in the fleet are not shown (as on Choose Your Next Ship against the computer).
    const next=s.next!;
    if(!selection){
      selection=new SelectionScene();selection.state=SelectionState.SELECTING_P1;selectionLift=FLEET_LIFT;resizeCameras();
      selection.setDisabledShips(new Set(CHARACTER_REGISTRY.filter((_,i)=>!next.ships.includes(i)).map(def=>def.id)));
      selection.jumpToIndex(next.carousel);
    }else turnSelection(next.carousel);
  }else if(selection)clearSelection();
  // The result screen covers the battle: its last message goes; so do the choice of the next ship and its waiting screen.
  if(s.screen==='next')matchHud?.message('');
  if(s.screen==='result'){state='gameOver';matchHud?.message('');}
  else if(['lobby','name','waiting','joining','invite'].includes(s.screen)){state='online';if(snapshot)clearRound();}
}

// ------------------------------------------------------------------ keys
/** A ship flies now (local or online): the keys belong to the battle. */
function flying():boolean {
  if((state==='playing'||state==='roundOver')&&mode!=='online')return !panel;
  return online.visible&&['battle','countdown'].includes(online.state.screen)&&!online.state.confirm;
}
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
    if(key==='Escape'){if(match&&nextChooser!==null)askLeaveGame();else showMenu(mode);return true;}
    return false;
  }
  // Two Players: the fleet choice (Enter only: Space is the fire key of Player 2).
  if(state==='fleet'&&picker){
    if(e.repeat)return true;
    const {handled,result}=picker.key(key);
    if(handled){syncFleetPick();if(result)fleetPickResult(result);}
    return handled;
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
  // K shows or hides the keys in every screen (nothing pauses); Esc closes the pop-up of the keys only.
  if(keys.key(e)){e.preventDefault();e.stopPropagation();return;}
  // The game's handlers make every Enter and Space choice. The browser's own
  // action would click the button that has the focus by then (the key that
  // skips the logo would also start a game), so it never runs.
  if(e.key==='Enter'||e.key===' ')e.preventDefault();
  if(e.key.toLowerCase()==='m'&&!e.repeat&&state!=='loading')sound.toggleMute();
  // F: full screen on or off, on the menu screens (in a battle F is too near the keys of Player 2).
  if(e.key.toLowerCase()==='f'&&!e.shiftKey&&!flying()&&state!=='loading'){if(!e.repeat)void viewport.toggleFullscreen();e.preventDefault();return;}
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
  get fleet(){return {battle:battleIndex,lost:[match?.lost(0).length??0,match?.lost(1).length??0],current:[match?.current(0)??null,match?.current(1)??null],chooser:match?.chooser??null,sides:match?.views()??null,nextChooser};},
  get pick(){return picker?{player:picking,picks:[...picker.picks],carousel:picker.carousel,focus:picker.focus,notice:picker.notice}:null;},
  get online(){const s=online.state;return {screen:s.screen,focus:s.focus,items:[...s.items],confirm:s.confirm,lobby:s.lobby.status,rooms:s.lobby.rooms.map(r=>r.name),notice:s.notice?.text??'',picks:[...s.fleet.picks],carousel:s.fleet.carousel,room:s.room?.name??null,opening:s.opening,
    next:s.next?{...s.next,ships:[...s.next.ships],left:online.nextLeft()}:null,battle:s.battle};},
  get fullscreen(){return document.fullscreenElement?.id??null;},
  get keys(){return {side:keys.sideMode,visible:keys.visible,mode:keys.currentMode};},
  get paused(){return paused;},
}),writable:false});
showMessage('Loading ships and game engine…');
Promise.all([RustEngine.load(),battle.prepare()]).then(([loaded])=>{
  engine=loaded;clearMessage();gameLoop.start();state='logo';new LogoSplash(container,()=>{
    const invite=readInvite(location.href);
    if(invite){mode='online';state='online';keys.setMode('online');online.invite(invite);}else showMenu('ai');
  });
}).catch(fail);
