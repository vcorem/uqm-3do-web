import {createGame,validateCampaign,getView,applyAction,resolveBattle,resolveSurvey,serializeSave,loadSave,meets} from './state.mjs';
import {UQMCombat} from './combat.mjs';
import {Score} from './audio.mjs';
import {SurfaceExpedition} from './lander.mjs';

const app=document.querySelector('#app');
const SAVE_KEY='uqm.afterlight.save.v1',BACKUP_KEY=SAVE_KEY+'.previous',PREF_KEY='uqm.afterlight.preferences.v1';
const score=new Score();
let data,state=null,page='chart',selected=null,readingScene=null,paragraph=0,saved=null,storageError='',combat=null,battleRunning=false,surface=null,toastTimer;
let logQuery='',logType='all',lastNotice='',preferences={music:true,textSize:'normal',reading:'flow'};
const h=value=>String(value??'').replace(/[&<>"']/g,ch=>({'&':'&amp;','<':'&lt;','>':'&gt;','"':'&quot;',"'":'&#39;'}[ch]));
const find=(kind,id)=>data[kind]?.find(entry=>entry.id===id);
const text=value=>h(String(value??'').replaceAll('{captain}',state?.captainName??'Captain'));
const button=(label,action,attrs='',className='')=>`<button class="${className}" data-action="${action}" ${attrs}>${label}</button>`;
function toast(message){clearTimeout(toastTimer);const element=document.querySelector('#toast');element.textContent=message;element.classList.add('visible');toastTimer=setTimeout(()=>element.classList.remove('visible'),6500);}
function persist(){
  if(!state)return true;
  try{
    const serialized=serializeSave(data,state),previous=localStorage.getItem(SAVE_KEY);
    localStorage.setItem(SAVE_KEY,serialized);saved=serialized;storageError='';
    if(previous&&previous!==serialized&&loadSave(data,previous).ok){
      try{localStorage.setItem(BACKUP_KEY,previous);}catch{storageError='Your current expedition is saved, but the previous-save backup could not be updated. Export a separate copy.';}
    }
    return true;
  }catch(error){storageError='Automatic saving is unavailable. Export your expedition from the menu before leaving.';toast(storageError);return false;}
}
function preferenceSave(){try{localStorage.setItem(PREF_KEY,JSON.stringify(preferences));}catch{}}
function confirmation(title,description){return new Promise(resolve=>{const dialog=document.querySelector('#confirm-dialog');document.querySelector('#confirm-title').textContent=title;document.querySelector('#confirm-text').textContent=description;dialog.returnValue='cancel';dialog.addEventListener('close',()=>resolve(dialog.returnValue==='confirm'),{once:true});dialog.showModal();});}
function accept(result){
  if(!result.ok){toast(result.error);return false;}
  const previous=state;state=result.state;const changes=[];
  for(const id of state.items.filter(id=>!previous?.items.includes(id)))changes.push(`Recovered: ${find('items',id)?.name||id}`);
  for(const id of state.evidence.filter(id=>!previous?.evidence.includes(id)))changes.push(`Finding: ${find('evidence',id)?.title||find('evidence',id)?.name||id}`);
  for(const offer of data.upgrades??[])if(state.flags[`upgrade_${offer.id}`]&&!previous?.flags[`upgrade_${offer.id}`])changes.push(`Installed: ${offer.name}`);
  for(const offer of data.shipyard??[])if(state.flags[`ship_${offer.ship}`]&&!previous?.flags[`ship_${offer.ship}`])changes.push(`Available hull: ${offer.name}`);
  if(previous&&previous.ship!==state.ship)changes.push(`Now flying: ${getView(data,state).ship.name}`);
  for(const c of data.characters){const before=previous?.reputation[c.id]??0,after=state.reputation[c.id]??0;if(after!==before)changes.push(`${c.name}: ${after>before?'cooperation improved':'trust strained'}`);}
  lastNotice=changes.slice(0,4).join(' · ');persist();selected=state.location;readingScene=null;render();
  if(state.scene!==previous?.scene){const heading=document.querySelector('.dialogue-title')||document.querySelector('#main-content');heading?.focus({preventScroll:true});heading?.scrollIntoView({block:'start'});}
  const important=result.events.filter(e=>['discovery','survey','recovery','world','equipment'].includes(e.type));if(important.length)toast(important.at(-1).text);
  return true;
}
function act(action){return accept(applyAction(data,state,action));}
async function begin(){
  if(saved&&!await confirmation('Begin a new expedition?','Your current automatic save will be replaced. Export it first if you want to return to this voyage.'))return;
  const captainName=document.querySelector('#captain-name')?.value.trim()||'Captain';
  try{state=createGame(data,{captainName});selected=state.location;page='chart';paragraph=0;readingScene=null;score.unlock().catch(()=>{});lastNotice='';persist();render();}catch(error){toast(error.message);}
}
function resume(){const result=loadSave(data,saved);if(!result.ok)return toast(result.error);state=result.state;page='chart';selected=state.location;score.unlock().catch(()=>{});render();}
function exportSave(){if(!state&&!saved)return toast('Begin an expedition before exporting a save.');const content=state?serializeSave(data,state):saved;const url=URL.createObjectURL(new Blob([content],{type:'application/json'}));const link=document.createElement('a');link.href=url;link.download=`afterlight-day-${state?.day??0}.json`;link.click();setTimeout(()=>URL.revokeObjectURL(url),1000);}

function titleScreen(){
  score.play('afterlight');
  app.innerHTML=`<main class="hero"><div class="hero-top"><span>AN ORIGINAL UR-QUAN MASTERS FAN CAMPAIGN</span><span>THE FAR REACH · YEAR SEVEN</span></div><div class="hero-copy"><span class="eyebrow">THE WAR IS OVER. THE WORK IS NOT.</span><h1>The Afterlight<br><em>Accord</em></h1><p>A voice where no one should be. A borrowed ship. Thirty-two worlds whose future depends on what you discover.</p><p>Take the Wayfarer beyond the old frontier. Meet the people the maps left out. Decide what it means to bring someone home.</p><label class="name-field">YOUR CAPTAIN’S NAME<input id="captain-name" maxlength="40" value="Captain" autocomplete="off"></label><div class="hero-actions">${saved?button('Continue expedition','resume','','primary'):''}${button(saved?'New expedition':'Begin expedition','begin','',saved?'':'primary')}${button('Import save','import','','quiet')}</div></div><footer class="hero-foot"><span>Explore · Investigate · Build your expedition</span><span>Original story & score · Real UQM space combat · English text</span></footer></main>`;
}
function header(view){
  const resource=(label,value,low=false)=>`<div class="resource ${low?'low':''}"><small>${label}</small>${value}</div>`;
  const tabs=[['chart','Star chart'],['contacts','Transmissions'],['objectives','Mission journal'],['ship','Wayfarer'],['log','Ship’s log']];
  return `<a href="#main-content" class="skip-link">Skip to current view</a><header class="masthead"><div class="wordmark"><span class="eyebrow">THE FAR REACH / WAYFARER</span>The Afterlight Accord</div><div class="resources">${resource('Voyage',`Day ${state.day}`)}${resource('Fuel',`${state.resources.fuel}<span class="muted"> / ${view.economy.fuelCapacity}</span>`,state.resources.fuel<12)}${resource('Crew',`${state.resources.crew}<span class="muted"> / ${view.ship.maxCrew}</span>`,state.resources.crew<5)}${resource('Supply',`${state.resources.credits} <span class="muted">RU</span>`)}</div><div class="header-actions">${button(preferences.music?'♫ Music on':'♫ Music off','music',`aria-pressed="${preferences.music}"`,'quiet')}${button('Menu','menu','','quiet')}</div></header>${storageError?`<div class="storage-warning" role="alert">${h(storageError)}</div>`:''}<nav class="tabs" aria-label="Expedition views">${tabs.map(([id,label])=>button(label,'tab',`data-page="${id}" ${page===id?'aria-current="page"':''}`,page===id?'active':'')).join('')}<span class="location">${h(view.system.name)} · ${h(state.captainName)}</span></nav>`;
}
function stars(view){
  const current=find('systems',state.location),target=find('systems',selected);
  let backdrop='';let seed=192729;
  for(let i=0;i<310;i++){seed=(seed*1664525+1013904223)>>>0;const x=seed%10000/100;seed=(seed*1664525+1013904223)>>>0;const y=seed%10000/100;backdrop+=`<circle cx="${x}" cy="${y}" r="${i%4===0?.095:.045}" fill="#bacbd0" opacity="${i%3===0?.45:.2}"/>`;}
  const route=target&&target.id!==current.id?`<line x1="${current.x}" y1="${current.y}" x2="${target.x}" y2="${target.y}" stroke="#d7b574" stroke-width=".12" stroke-dasharray=".5 .7"/>`:'';
  return `<svg class="star-chart" viewBox="0 0 100 100" role="img" aria-label="Far Reach star chart. Select a star, or use the destination list."><defs><radialGradient id="star-glow"><stop stop-color="#d7b574" stop-opacity=".3"/><stop offset="1" stop-color="#d7b574" stop-opacity="0"/></radialGradient></defs>${backdrop}${route}${view.travelOptions.map(s=>{const here=s.id===current.id,chosen=s.id===selected,visited=state.visitedSystems.includes(s.id);return `<g class="star ${chosen?'selected':''}" data-star="${s.id}" role="button" tabindex="0" aria-label="${h(s.name)}${here?', current location':''}">${chosen?`<circle cx="${s.x}" cy="${s.y}" r="4" fill="url(#star-glow)"/>`:''}<circle cx="${s.x}" cy="${s.y}" r="2.4" fill="transparent"/><circle cx="${s.x}" cy="${s.y}" r="${here?.75:.38}" fill="${here?'#e4bd76':visited?'#83d7d0':'#b0c3ce'}"/>${here?`<circle cx="${s.x}" cy="${s.y}" r="1.3" fill="none" stroke="#e4bd76"/>`:''}<text x="${s.x+1.1}" y="${s.y-.9}">${h(s.name)}</text></g>`;}).join('')}</svg>`;
}
function commitments(){
  const upcoming=(data.events||[]).filter(e=>e.forecast&&!state.triggeredEvents.includes(e.id)&&meets(state,e.requires));
  if(!upcoming.length)return '';
  return `<aside class="commitments" aria-label="Upcoming commitments"><h3>Commitments & changing conditions</h3>${upcoming.map(e=>`<p><strong>${h(e.title)}</strong><small>${e.day>state.day?`${e.day-state.day} voyage days remaining · day ${e.day}`:'Review at the next voyage action'}</small>${h(e.forecast)}</p>`).join('')}<small>Reading and planning never spend days.</small></aside>`;
}
function surfaceBrief(system){const survey=system.survey;if(!survey)return '';const reward=survey.rewards||{},findings=[...(reward.giveItems||[]).map(id=>find('items',id)?.name),...(reward.evidence||[]).map(id=>find('evidence',id)?.title||find('evidence',id)?.name)].filter(Boolean);return `<div class="operation-brief"><span class="eyebrow">${h(({rescue:'RESCUE',relay:'RELAY',survey:'SURVEY'})[survey.mode]||'SURFACE')} / ${h(survey.biome||'BASALT')}</span><p>${h(survey.brief||survey.title||'Investigate the surface signal and return with the findings.')}</p>${findings.length?`<p class="small">Potential finding: ${h(findings.join(' · '))}</p>`:''}</div>`;}
function chart(view){
  const destination=view.travelOptions.find(s=>s.id===selected)||view.travelOptions.find(s=>s.id===state.location);selected=destination.id;
  const here=destination.id===state.location;
  const transmissions=here?view.availableScenes:[];
  return `<div class="chart-layout"><section class="map">${stars(view)}<div class="map-heading"><span class="eyebrow">EXPEDITION CHART / ${state.knownSystems.length} KNOWN SYSTEMS</span><h2>The Far Reach</h2></div><div class="map-legend"><span><i></i>Wayfarer</span><span><i class="teal"></i>Visited</span><span><i class="gray"></i>Unexplored</span></div></section><aside class="system-panel"><label class="eyebrow" for="destination">PLOT A COURSE</label><select id="destination" class="system-select">${view.travelOptions.toSorted((a,b)=>a.name.localeCompare(b.name)).map(s=>`<option value="${s.id}" ${s.id===selected?'selected':''}>${h(s.name)}${s.id===state.location?' · here':''}</option>`).join('')}</select><div class="planet" data-biome="${h(destination.survey?.biome||'ocean')}"><div class="planet-orb"></div></div><div><span class="eyebrow">${here?'CURRENT ORBIT':state.visitedSystems.includes(destination.id)?'CHARTED SYSTEM':'UNEXPLORED SYSTEM'}</span><h2>${h(destination.name)}</h2></div><p>${h(destination.description)}</p>${here&&view.canSurvey?surfaceBrief(destination):''}${here?`<div class="route-cost"><span>Signal contacts</span><span>${transmissions.length}</span></div><div class="stack">${view.scene?button('Resume transmission','resume-transmission','','primary'):''}${transmissions.map(s=>button(`${h(s.title)}<small>${h(find('characters',s.characterId)?.name)}</small>`,'enter',`data-id="${s.id}"`)).join('')}${!transmissions.length?'<p class="help-note">No new local transmissions. Review your mission journal or explore another system.</p>':''}${view.canSurvey?button(`Launch ${h(({survey:'surface survey',rescue:'rescue operation',relay:'relay expedition'})[destination.survey.mode]||'surface expedition')}<small>${destination.survey.days??1} day · ${h(destination.survey.difficulty||'standard')} conditions</small>`,'survey'):''}${destination.services?.refuel||destination.services?.repair?button('Dock & outfit','tab','data-page="ship"'):''}</div>`:`<div class="route-cost"><span>${destination.days} ${destination.days===1?'day':'days'}</span><span>${destination.fuel} fuel</span></div>${button('Set course →','travel',`data-id="${destination.id}" ${destination.allowed?'':'disabled'}`,'primary')}${!destination.allowed?`<p class="help-note">${h(destination.reason)}</p>`:''}<p class="help-note">Travel advances the expedition clock. Reading and reviewing your log never does.</p>`}${commitments()}</aside></div>`;
}
function conversation(view){
  const scene=view.scene,character=find('characters',scene.characterId);
  if(readingScene!==scene.id){readingScene=scene.id;paragraph=0;}
  const end=preferences.reading==='flow'?scene.paragraphs.length:Math.min(paragraph+2,scene.paragraphs.length),done=end>=scene.paragraphs.length;
  score.play(scene.music||character.music||'drift');
  return `<article class="conversation" data-scene="${scene.id}"><div class="portrait"><img src="${h(character.portrait)}" alt="${h(character.name)}"><div class="portrait-label"><span class="eyebrow">${h(character.species)} / COMMUNICATION LINK</span><h2>${h(character.name)}</h2><p>${h(character.role)}</p></div></div><section class="dialogue"><header class="transmission-heading"><span class="eyebrow">ACT ${scene.act} · ${h(view.system.name)}</span>${button('Return to chart','leave','','quiet')}</header><h3 class="dialogue-title" tabindex="-1">${h(scene.title)}</h3><div class="prose" aria-live="polite">${scene.paragraphs.slice(paragraph,end).map(p=>`<p>${text(p)}</p>`).join('')}</div><div class="reading-progress">${preferences.reading==='flow'?'TRANSMISSION COMPLETE':`TRANSMISSION ${paragraph+1}–${end} / ${scene.paragraphs.length}`}</div>${!done?button('Continue transmission →','continue','','primary continue'):`<div class="response-list" aria-label="Your response">${view.choices.map((c,i)=>button(`<span class="choice-number">${i+1}.</span>${text(c.text)}${c.reason?`<small>${h(reasonText(c))}</small>`:''}${choiceCosts(c)?`<small>${h(choiceCosts(c))}</small>`:''}`,'choose',`data-id="${c.id}" ${c.enabled?'':'disabled'}`)).join('')}</div>`}${preferences.reading!=='flow'&&paragraph>0?button('← Previous passage','previous','','quiet small continue'):''}</section></article>`;
}
function requirementText(condition){
  if(!condition||meets(state,condition))return '';
  if(condition.all)return condition.all.map(requirementText).filter(Boolean).join(' · ');
  if(condition.any)return `One of: ${condition.any.map(requirementText).filter(Boolean).join(' / ')}`;
  if(condition.item)return `Recover ${find('items',condition.item)?.name||'the required instrument'}`;
  if(condition.evidence)return `Investigate ${find('evidence',condition.evidence)?.title||find('evidence',condition.evidence)?.name||'the missing evidence'}`;
  if(condition.reputation)return `${(state.reputation[condition.reputation]??0)<0?'Rebuild':'Build'} cooperation with ${find('characters',condition.reputation)?.name||'this community'}`;
  if(condition.quest)return `Complete ${find('quests',condition.quest)?.title||find('quests',condition.quest)?.name||'the related expedition'}`;
  if(condition.visited)return `Follow the signal: ${find('scenes',condition.visited)?.title||'an earlier conversation'}`;
  if(condition.resource)return `Requires ${condition.min} ${condition.resource==='credits'?'RU':condition.resource}`;
  if(condition.flag){
    const clues={act2:'Complete the first compact at Interval Anchorage',act3:'Follow one of the six principal leads to the Quiet Frame',
      finale_unlocked:'Complete the Glasshouse and Archive investigations',met_khervat:'Meet Khervat at the Witness site',
      archive_recovered:'Recover the Archive investigation',thermal_trial:'Complete the growers’ thermal trial',
      meadow_consent:'Agree on the Meadow trial with its growers',winter_repaired:'Restore the frozen beds at Ember',
      ilex_autonomy:'Complete Ilex’s coexistence experiment at Threadwater',pell_supports:'Help Pell’s convoy reach its destinations',
      harbor_independent:'Prepare an independent Open Harbor',crossing_secured:'Secure the Common Crossing route',
      crossing_open:'Complete the relay crossing at Common Crossing',lastlight_cooled:'Repair the Lastlight cooling system'};
    if(condition.is!==false&&clues[condition.flag])return clues[condition.flag];
    if(condition.is===false)return 'An earlier decision or changing condition has closed this approach';
  }
  if(condition.day)return condition.day.max!==undefined?`Available through day ${condition.day.max}`:`Available from day ${condition.day.min}`;
  return 'Follow the related investigation or commitment first';
}
function reasonText(choice){if(!choice.reason)return '';return !meets(state,choice.requires)?choice.lockedText||requirementText(choice.requires):choice.reason;}
function choiceCosts(choice){const e=choice.effects??{},costs=[];if(e.days)costs.push(`${e.days} days`);for(const key of ['credits','fuel','crew'])if(e[key]<0)costs.push(`${-e[key]} ${key==='credits'?'RU':key}`);if(choice.combat)costs.push('Space combat');return costs.join(' · ');}
function relationship(id){const value=state.reputation[id]??0;return value<=-2?'Working relationship: strained':value>=3?'Working relationship: trusted':value>0?'Working relationship: cooperative':'Working relationship: still developing';}
function serviceControl(kind,label,max,price,available){return `<div class="service-control"><label for="${kind}-amount">${label}<span class="muted small">${price} RU / unit · up to ${max}</span></label><input id="${kind}-amount" type="number" min="1" max="${Math.max(1,max)}" value="${Math.min(max,kind==='refuel'?10:5)||1}" ${available&&max?'':'disabled'}><button data-action="${kind}" ${available&&max?'':'disabled'}>${label}</button><span class="service-cost small" data-cost-for="${kind}">${Math.min(max,kind==='refuel'?10:5)*price} RU</span></div>`;}
function benefitText(b={}){return [b.fuelCapacity?`+${b.fuelCapacity} fuel capacity`:null,b.travelEfficiency?`${b.travelEfficiency}% lower travel fuel use`:null,b.surveyShielding?`${b.surveyShielding}% surface damage reduction`:null,b.surveyBattery?`+${b.surveyBattery}s surface battery`:null].filter(Boolean).join(' · ');}
function outfitterLocations(offer){const ids=offer.systemIds||[];if(!ids.length||offer.owned)return '';return `<p class="help-note">Outfitter: ${ids.map(id=>h(find('systems',id)?.name||id)).join(' · ')}</p>`;}
function expeditionEquipment(view){return `<h3 class="section-subhead">Expedition improvements</h3><p class="muted small">Recovered technology and local cooperation open new possibilities. Equipment stays with the expedition when you change ships.</p><div class="equipment-grid">${(view.upgrades||[]).map(o=>`<article class="equipment-card ${o.owned?'owned':''}"><span class="tag">${o.owned?'INSTALLED':`${o.price} RU`}</span><h3>${h(o.name)}</h3><p>${h(o.description)}</p>${outfitterLocations(o)}<p class="equipment-benefits">${h(benefitText(o.benefits))}</p>${o.owned?'':button('Install improvement','purchaseUpgrade',`data-id="${o.id}" ${o.enabled?'':'disabled'}`)}${!o.owned&&o.reason?`<p class="help-note">${h(reasonText(o))}</p>`:''}</article>`).join('')||'<p class="muted">Your commission will introduce the outfitting teams.</p>'}</div><h3 class="section-subhead">Ships & alliances</h3><p class="muted small">Change hulls at an equipped port. Your surviving crew transfer; additional berths can be filled through resupply.</p><div class="equipment-grid">${(view.fleet||[]).map(o=>`<article class="equipment-card ${o.ship===state.ship?'owned':''}"><span class="tag">${o.ship===state.ship?'ACTIVE HULL':o.owned?'AVAILABLE TO YOUR EXPEDITION':`${o.price} RU`}</span><h3>${h(o.name)}</h3><p>${h(o.description||'The expedition’s original vessel.')}</p>${outfitterLocations(o)}${o.ship===state.ship?'':o.owned?button('Command this ship','switchShip',`data-ship="${o.ship}" ${o.enabled?'':'disabled'}`):button('Acquire this ship','purchaseShip',`data-id="${o.id}" ${o.enabled?'':'disabled'}`)}${o.reason?`<p class="help-note">${h(reasonText(o))}</p>`:''}</article>`).join('')}</div>`;}
function contacts(view){
  const encountered=new Set(state.visitedScenes.map(id=>find('scenes',id)?.characterId));view.availableScenes.forEach(s=>encountered.add(s.characterId));
  return `<section class="section"><header><div><span class="eyebrow">OPEN CHANNELS</span><h2>People, not destinations.</h2><p>Every account has a speaker. Every speaker has something to lose. New local transmissions appear here when circumstances change.</p></div>${view.scene?button('Resume transmission','resume-transmission','','primary'):''}</header><div class="contact-list">${data.characters.filter(c=>encountered.has(c.id)).map(c=>{const scenes=view.availableScenes.filter(s=>s.characterId===c.id);return `<article class="contact"><img src="${h(c.portrait)}" alt="${h(c.name)}" loading="lazy"><div class="contact-body"><span class="eyebrow">${h(c.species)}</span><h3>${h(c.name)}</h3><p>${h(c.role)}</p><p class="relationship">${relationship(c.id)}</p>${scenes.length?scenes.map(s=>button(h(s.title),'enter',`data-id="${s.id}"`)).join(''):'<p class="help-note">No new local signal.</p>'}</div></article>`;}).join('')}</div></section>`;
}
function objectives(view){
  const signals=state.ending?[]:state.knownSystems.map(id=>{const local=getView(data,{...state,location:id,scene:null});return{system:local.system,scenes:local.availableScenes};}).filter(entry=>entry.scenes.length);
  return `<section class="section"><header><div><span class="eyebrow">MISSION JOURNAL</span><h2>What we know. What we owe.</h2><p>Evidence is preserved with its source. An account can be sincere without being complete.</p></div></header>${commitments()}<div class="ship-layout"><div><h3>Open threads</h3>${view.quests.length?view.quests.toSorted((a,b)=>(a.status==='active'?-1:1)-(b.status==='active'?-1:1)).map(q=>`<article class="quest-row"><span class="tag ${h(q.status)}">${h(q.status)}</span><h3>${h(q.title||q.name)}</h3><p>${h(q.description)}</p></article>`).join(''):'<p class="empty">Your commission begins with Mara’s briefing.</p>'}</div><div><h3>Latest findings</h3>${state.journal.filter(e=>['discovery','world','survey'].includes(e.type)).slice(-7).reverse().map(e=>`<article class="log-entry"><span class="eyebrow">DAY ${e.day}</span><p>${text(e.text)}</p></article>`).join('')||'<p class="empty">Findings will appear here as you explore.</p>'}</div></div><h3 style="margin-top:3rem">Signals worth following</h3><p class="muted small">Known contacts with an available conversation. Their accounts may lead in different directions.</p><div class="inventory">${signals.map(({system,scenes})=>`<article><h3>${h(system.name)}</h3><p>${scenes.map(s=>h(s.title)).join(" · ")}</p>${button("View system","signal",`data-id="${system.id}"`)}</article>`).join("")||'<p class="muted small">No outstanding known signals.</p>'}</div><h3 style="margin-top:3rem">Evidence aboard</h3><div class="inventory">${view.evidence.map(e=>`<article><span class="eyebrow">${h(e.kind||e.type||'RECORDED OBSERVATION')}</span><h3>${h(e.title||e.name)}</h3><p>${text(e.description||e.text)}</p>${e.source?`<p class="help-note">Source: ${h(e.source)}</p>`:''}</article>`).join('')}</div></section>`;
}
function ship(view){
  const fuel=Math.min(view.economy.fuelCapacity-state.resources.fuel,Math.floor(state.resources.credits/view.economy.refuelPrice));
  const crew=Math.min(view.ship.maxCrew-state.resources.crew,Math.floor(state.resources.credits/view.economy.repairPrice));
  return `<section class="section"><header><div><span class="eyebrow">EXPEDITION VESSEL</span><h2>The Wayfarer</h2><p>A survey commission, a borrowed hull, and one more chair than the manifest requires.</p></div></header><div class="ship-layout"><div><div class="ship-blueprint" aria-label="Stylized expedition ship schematic"><svg viewBox="0 0 340 240" fill="none" stroke="#93c9c7" stroke-width="1.4"><path d="M170 20 188 70 190 151 222 170 222 190 184 178 181 216 159 216 156 178 118 190 118 170 150 151 152 70Z"/><path d="M110 80h25v115h-25zM205 80h25v115h-25zM135 121h17m38 0h15M160 82h20v40h-20zM160 140h20m-20 10h20m-20 10h20M170 20v196"/><circle cx="170" cy="91" r="4"/><path d="M35 30h60m-30 0v175m-10 0h20M245 207h70m-35 0V40m-10 0h20" stroke="#496672" stroke-dasharray="3 5"/></svg></div><div class="stat-row"><span>Combat hull</span><span>${h(view.ship.name)}</span></div><div class="stat-row"><span>Crew on support tender</span><span>${view.reserveCrew||0}</span></div><div class="stat-row"><span>Fuel capacity</span><span>${view.economy.fuelCapacity}</span></div><div class="stat-row"><span>Commanding officer</span><span>${h(state.captainName)}</span></div><div class="stat-row"><span>Systems visited</span><span>${state.visitedSystems.length} / ${state.knownSystems.length} charted</span></div><div class="stat-row"><span>Current berth</span><span>${h(view.system.name)}</span></div></div><div><h3>Resupply</h3><p class="muted small">Fuel costs ${view.economy.refuelPrice} RU per unit. Recovering crew costs ${view.economy.repairPrice} RU per berth. Service is available only at an equipped port.</p><div class="service-row">${serviceControl('refuel','Buy fuel',fuel,view.economy.refuelPrice,view.system.services?.refuel)}${serviceControl('repair','Recover crew',crew,view.economy.repairPrice,view.system.services?.repair)}</div><h3 style="margin-top:2rem">When the route goes wrong</h3><p class="muted small">A reconstruction tender can tow you to Hearth, restore emergency fuel and recover crew. It takes ${view.economy.distressDays} days and up to ${view.economy.distressFee} RU. Rescue is available even with no funds.</p>${button('Send a distress call','distress')}<p class="help-note">Space combat uses the original UQM ships and physics. Surviving crew return to this manifest. A lost battle changes the story; it does not erase your expedition.</p></div></div>${expeditionEquipment(view)}<h3 style="margin-top:2.5rem">Cargo & instruments</h3><div class="inventory">${view.items.map(i=>`<article><h3>${h(i.name||i.title)}</h3><p>${text(i.description)}</p></article>`).join('')||'<p class="muted small">No special cargo aboard.</p>'}</div></section>`;
}
function logs(){
  const entries=state.journal.filter(e=>(logType==='all'||e.type===logType)&&`${e.title} ${e.text}`.toLowerCase().includes(logQuery.toLowerCase())).toReversed();
  return `<section class="section"><header><div><span class="eyebrow">SHIP’S LOG / DAY ${state.day}</span><h2>Nothing heard is lost.</h2><p>Exact transmissions, decisions, discoveries and crossings. The clock stands still while you read.</p></div>${button('Export save','export')}</header><div class="filter-row"><input id="log-search" type="search" placeholder="Search a name, place, or phrase…" aria-label="Search ship’s log" value="${h(logQuery)}"><select id="log-type" aria-label="Filter log"><option value="all">Everything</option>${['conversation','decision','discovery','survey','battle','travel','world'].map(t=>`<option value="${t}" ${logType===t?'selected':''}>${t[0].toUpperCase()+t.slice(1)}</option>`).join('')}</select></div><div class="log-list">${entries.map(e=>`<article class="log-entry"><span class="eyebrow">DAY ${e.day} / ${h(e.type)}</span><h3>${h(e.title)}</h3>${e.text.length>650?`<details><summary>Read full transmission</summary><p>${text(e.text)}</p></details>`:`<p>${text(e.text)}</p>`}</article>`).join('')||'<p class="empty">No entries match this search.</p>'}</div></section>`;
}
function settings(){return `<section class="section"><header><div><span class="eyebrow">EXPEDITION MENU</span><h2>Keep your place among the stars.</h2></div></header><div class="settings-grid"><div><h3>Your expedition</h3><p>Every decision is saved automatically in this browser. Export a save to keep a separate copy or move to another computer.</p><div class="button-row">${button('Export save','export','','primary')}${button('Import save','import')}${button('Restore previous autosave','backup')}</div><h3>Sound & reading</h3><div class="button-row">${button(preferences.music?'Turn music off':'Turn music on','music',`aria-pressed="${preferences.music}"`)}${button(preferences.textSize==='large'?'Standard text':'Larger text','text-size')}${button(preferences.reading==='flow'?'Read in short passages':'Read complete transmissions','reading')}</div><h3>A new course</h3><p>Returning to the title keeps your saved expedition. Beginning a new one replaces the automatic save.</p>${button('Return to title','title')}</div><div><h3>About this voyage</h3><p>The Afterlight Accord is an original, unofficial continuation set seven years after the events of Star Control II. Its account of the Far Reach is new fiction, not a claim about official canon.</p><p>The branching campaign, art and synthesized score were created for this project. Real space engagements run in the Rust/WebAssembly UQM engine with its original ships and sound effects.</p><p>This campaign has English text and an original instrumental score; it does not include recorded dialogue performances.</p><p><a href="./CREDITS.html" target="_blank" rel="noopener">Credits, source and asset notes ↗</a></p><h3>Combat controls</h3><p>Arrow keys steer and thrust. Right Ctrl or Enter fires. Right Shift activates the special ability. Use the Disengage button to withdraw. Surface controls: Arrow keys or WASD move the lander. Read the landing briefing before deploying: surveys, rescues and relay operations have different objectives. Recovered findings can unlock equipment and new approaches; hazards damage the lander. Escape recalls the team. Reading, menus and log searches do not spend days.</p></div></div></section>`;}
function ending(view){const ending=view.ending;const additional=(ending.epilogues??[]).filter(e=>meets(state,e.requires));score.play(ending.music||({accord:'homecoming',mandate:'accord',partition:'drift'})[ending.id]||'homecoming');return `<article class="ending"><section class="section"><span class="eyebrow">THE END OF THIS VOYAGE / DAY ${state.day}</span><h1>${h(ending.title)}</h1>${ending.paragraphs.map(p=>`<p>${text(p)}</p>`).join('')}${additional.map(e=>`<hr>${e.title?`<h3>${h(e.title)}</h3>`:''}${e.paragraphs.map(p=>`<p>${text(p)}</p>`).join('')}`).join('')}<hr><span class="eyebrow">THE WAYFARER WILL KEEP YOUR ACCOUNT.</span><p class="muted">You visited ${state.visitedSystems.length} systems and recorded ${state.evidence.length} pieces of evidence. Your decisions remain in the ship’s log.</p><div class="button-row">${button('Read the ship’s log','tab','data-page="log"','primary')}${button('Export this ending','export')}${button('Return to title','title')}</div></section></article>`;}
function render(){
  document.documentElement.style.setProperty('font-size',preferences.textSize==='large'?'18px':'16px');
  if(!state){titleScreen();return;}
  const view=getView(data,state);
  let content;
  if(view.ending&&page==='chart')content=ending(view);
  else if(view.scene&&page==='chart')content=conversation(view);
  else{if(!state.pendingBattle)score.play('drift');content=page==='contacts'?contacts(view):page==='objectives'?objectives(view):page==='ship'?ship(view):page==='log'?logs():page==='menu'?settings():chart(view);}
  app.innerHTML=`<div class="shell">${header(view)}<main class="main" id="main-content" tabindex="-1">${lastNotice?`<div class="expedition-change" role="status">${h(lastNotice)}</div>`:''}${content}</main></div>`;
  if(state.pendingBattle&&!battleRunning)showPendingBattle();
  if(state.pendingSurvey&&!surface)showPendingSurvey();
}
function combatSeed(id){let result=2166136261;for(const c of id)result=Math.imul(result^c.charCodeAt(0),16777619);return result>>>0;}
async function showPendingBattle(){
  if(battleRunning||!state?.pendingBattle)return;
  battleRunning=true;app.inert=true;const battle=state.pendingBattle,layer=document.querySelector('#combat-layer');layer.hidden=false;document.querySelector('#combat-recovery').hidden=true;document.querySelector('#combat-status').textContent='Loading the UQM combat engine. The first engagement downloads its ship artwork and sound effects.';score.play('battle');
  if(!combat){combat=new UQMCombat({container:document.querySelector('#combat-frame')});combat.addEventListener('started',()=>document.querySelector('#combat-status').textContent='Engagement in progress. Your surviving crew return to the expedition.');combat.addEventListener('visible',()=>document.querySelector('#combat-status').textContent='The ship is ready. Activate combat below to take the controls.');}
  try{const result=await combat.start({id:battle.battleId,player:{ship:state.ship,crew:state.resources.crew},enemies:battle.enemies,seed:combatSeed(battle.battleId),difficulty:'standard'});battleRunning=false;layer.hidden=true;app.inert=false;accept(resolveBattle(data,state,result));}
  catch(error){battleRunning=false;document.querySelector('#combat-status').textContent=`Combat has not completed: ${error.message}`;document.querySelector('#combat-recovery').hidden=false;}
}

function showPendingSurvey(){
  const request=state.pendingSurvey;if(!request||surface)return;
  const layer=document.querySelector('#surface-layer');layer.hidden=false;app.inert=true;
  const system=find('systems',request.systemId),capabilities=getView(data,state).capabilities||{};
  document.querySelector('#surface-heading').textContent=`${system.name} / SURFACE EXPEDITION`;
  score.play(system.survey?.music||'drift');
  try{
    surface=new SurfaceExpedition({container:document.querySelector('#surface-frame'),id:request.id,seed:request.seed,biome:system.survey?.biome||'basalt',mode:system.survey?.mode||'survey',difficulty:system.survey?.difficulty||'standard',shielding:capabilities.surveyShielding||0,batteryBonus:capabilities.surveyBattery||0,completedTasks:state.surveyProgress?.[request.systemId]||[],objectiveLabels:system.survey?.objectiveLabels,onComplete:receipt=>{
      surface?.destroy();surface=null;layer.hidden=true;app.inert=false;
      const result=resolveSurvey(data,state,receipt);accept(result);
      (document.querySelector('[data-action="survey"]')||document.querySelector('#main-content'))?.focus();
    }}).start();
  }catch(error){surface?.destroy();surface=null;layer.hidden=true;app.inert=false;accept(applyAction(data,state,{type:'abandonSurvey'}));toast(`The surface team could not launch: ${error.message}`);}
}

app.addEventListener('click',async event=>{
  const star=event.target.closest('[data-star]');if(star){selected=star.dataset.star;render();return;}
  const target=event.target.closest('button[data-action]');if(!target||target.disabled)return;const action=target.dataset.action;
  switch(action){
    case 'begin':await begin();break;
    case 'resume':resume();break;
    case 'music':preferences.music=!preferences.music;score.setEnabled(preferences.music);preferenceSave();render();break;
    case 'tab':if(target.dataset.page==='chart'&&state.scene){page='chart';act({type:'leave'});}else{page=target.dataset.page;render();}break;
    case 'resume-transmission':page='chart';render();break;
    case 'menu':page=page==='menu'?'chart':'menu';render();break;
    case 'enter':if(state.scene)act({type:'leave'});page='chart';act({type:'enter',sceneId:target.dataset.id});break;
    case 'leave':act({type:'leave'});break;
    case 'continue':paragraph+=2;render();document.querySelector('.dialogue-title')?.scrollIntoView({block:'nearest'});break;
    case 'previous':paragraph=Math.max(0,paragraph-2);render();break;
    case 'choose':{const c=getView(data,state).choices.find(c=>c.id===target.dataset.id);if(!c?.enabled)break;if((c.confirm||c.effects?.ending)&&!await confirmation('Commit to this decision?',c.confirmText||c.text))break;act({type:'choose',choiceId:c.id});break;}
    case 'signal':if(state.scene&&!act({type:'leave'}))break;page='chart';selected=target.dataset.id;render();break;
    case 'travel':page='chart';act({type:'travel',systemId:target.dataset.id});break;
    case 'survey':act({type:'launchSurvey'});break;
    case 'refuel':case 'repair':{const input=document.querySelector(`#${action}-amount`);act({type:action,amount:input?Number(input.value):Number(target.dataset.amount)});break;}
    case 'purchaseUpgrade':act({type:'purchaseUpgrade',upgradeId:target.dataset.id});break;
    case 'purchaseShip':act({type:'purchaseShip',offerId:target.dataset.id});break;
    case 'switchShip':act({type:'switchShip',ship:Number(target.dataset.ship)});break;
    case 'distress':if(await confirmation('Send a distress call?','A tender will return you to Hearth. The rescue advances time and uses up to 50 RU; nobody is refused for lack of money.'))act({type:'distress'});break;
    case 'export':exportSave();break;
    case 'import':document.querySelector('#import-file').click();break;
    case 'backup':{let old;try{old=localStorage.getItem(BACKUP_KEY);}catch{}if(!old){toast('No previous automatic save is available.');break;}const result=loadSave(data,old);if(!result.ok){toast(result.error);break;}if(await confirmation('Restore the previous autosave?','This returns to the decision immediately before the current save.')){state=result.state;persist();page='chart';readingScene=null;render();}break;}
    case 'text-size':preferences.textSize=preferences.textSize==='large'?'normal':'large';preferenceSave();render();break;
    case 'reading':preferences.reading=preferences.reading==='flow'?'passages':'flow';paragraph=0;preferenceSave();render();break;
    case 'title':if(persist()){state=null;lastNotice='';titleScreen();}else{render();toast('Your unsaved expedition is still open. Export a save before returning to the title.');}break;
  }
});
app.addEventListener('change',event=>{if(event.target.id==='destination'){selected=event.target.value;render();}if(event.target.id==='log-type'){logType=event.target.value;render();}});
app.addEventListener('input',event=>{if(['refuel-amount','repair-amount'].includes(event.target.id)){const kind=event.target.id.split('-')[0],price=getView(data,state).economy[kind==='refuel'?'refuelPrice':'repairPrice'],amount=Number(event.target.value),valid=Number.isInteger(amount)&&amount>=1&&amount<=Number(event.target.max);document.querySelector(`[data-action="${kind}"]`).disabled=!valid;document.querySelector(`[data-cost-for="${kind}"]`).textContent=valid?`${amount*price} RU`:'Choose an available quantity.';}if(event.target.id==='log-search'){logQuery=event.target.value;const selection=event.target.selectionStart;render();const input=document.querySelector('#log-search');input.focus();try{input.setSelectionRange(selection,selection);}catch{}}});
document.addEventListener('keydown',event=>{
  if(event.target.closest('input,textarea,select,[contenteditable="true"]')||event.ctrlKey||event.metaKey||event.altKey||document.querySelector('dialog[open]')||!document.querySelector('#combat-layer').hidden||!document.querySelector('#surface-layer').hidden)return;
  if((event.key==='Enter'||event.key===' ')&&event.target.closest('[data-star]')){event.preventDefault();selected=event.target.closest('[data-star]').dataset.star;render();}
  if(/^[1-9]$/.test(event.key)&&state?.scene&&page==='chart'){const choices=[...document.querySelectorAll('.response-list button')];choices[Number(event.key)-1]?.click();}
});
document.querySelector('#retreat').addEventListener('click',()=>{if(battleRunning)combat?.retreat();else document.querySelector('#cancel-combat').click();});
document.querySelector('#retry-combat').addEventListener('click',async()=>{try{await combat.retry();await showPendingBattle();}catch(error){toast(error.message);}});
document.querySelector('#cancel-combat').addEventListener('click',async()=>{if(battleRunning)return;if(!await confirmation('Withdraw from this engagement?','The withdrawal is recorded in the story. It does not count as a victory.'))return;const result=applyAction(data,state,{type:'abandonBattle'});document.querySelector('#combat-layer').hidden=true;app.inert=false;accept(result);});
document.querySelector('#import-file').addEventListener('change',async event=>{const file=event.target.files?.[0];event.target.value='';if(!file)return;if(file.size>4*1024*1024)return toast('That save file is too large.');const result=loadSave(data,await file.text());if(!result.ok)return toast(result.error);if((state||saved)&&!await confirmation('Import this expedition?',state&&serializeSave(data,state)!==saved?'Your current expedition includes progress that is not saved. Go back and export it first if you want to keep it. Importing replaces this expedition.':'The imported save will replace the current automatic save.'))return;state=result.state;page='chart';selected=state.location;readingScene=null;persist();score.unlock().catch(()=>{});render();});

try{
  const response=await fetch('./content/campaign.json');if(!response.ok)throw new Error('The campaign file could not be loaded.');data=await response.json();
  const check=validateCampaign(data);if(check.errors.length)throw new Error(`Campaign validation failed: ${check.errors[0]}`);
  // Preferences are disposable; a damaged preference must never hide an expedition.
  try{const value=JSON.parse(localStorage.getItem(PREF_KEY)||'null');if(value&&typeof value.music==='boolean'){preferences.music=value.music;preferences.textSize=value.textSize==='large'?'large':'normal';preferences.reading=value.reading==='passages'?'passages':'flow';}}catch{/* Use defaults independently of save recovery. */}
  try{
    const current=localStorage.getItem(SAVE_KEY),backup=localStorage.getItem(BACKUP_KEY);
    if(current&&loadSave(data,current).ok)saved=current;
    else if(backup&&loadSave(data,backup).ok){saved=backup;storageError='The latest save could not be read. Continue will recover your previous valid autosave.';}
    else if(current||backup)storageError='The saved expedition could not be read. Import a backup or begin a new voyage.';
  }catch{storageError='Browser storage is unavailable. Export saves to keep your expedition.';}
  score.enabled=preferences.music;
  Object.defineProperty(window,'__afterlight',{value:Object.freeze({get state(){return state?structuredClone(state):null;},get surface(){return surface?structuredClone(surface.state):null;},get music(){return Object.freeze({current:score.current?.id,context:score.context?.state,errors:[...score.errors]});},get content(){return Object.freeze({id:data.id,systems:data.systems.length,scenes:data.scenes.length});}}),writable:false});
  render();if(storageError)toast(storageError);
}catch(error){app.innerHTML=`<main class="boot"><span class="eyebrow">SIGNAL INTERRUPTED</span><h1>We could not open the expedition.</h1><p>${h(error.message)}</p><p>Refresh after the local build has completed.</p></main>`;console.error(error);}
