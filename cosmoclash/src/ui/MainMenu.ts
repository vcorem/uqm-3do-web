import './main-menu.css';

export class MainMenu {
  private root:HTMLDivElement;
  private keyHandler:(e:KeyboardEvent)=>void;
  constructor(parent:HTMLElement,onPlay:()=>void,onLocal?:()=>void,onOnline?:()=>void) {
    this.root=document.createElement('div');this.root.id='main-menu';
    this.root.style.backgroundImage='url(/textures/main-bk.jpg)';
    const add=(text:string,action:()=>void)=>{const button=document.createElement('button');button.className='menu-btn';button.textContent=text;button.addEventListener('click',action);this.root.appendChild(button);};
    add('Play vs Computer',onPlay);
    if(onLocal)add('Two Players · Same Computer',onLocal);
    if(onOnline)add('Online Melee',onOnline);
    const controls=document.createElement('p');controls.className='menu-controls';controls.textContent='↑ Thrust · ← → Turn · / Fire · . Special · M Mute';this.root.appendChild(controls);
    const credits=document.createElement('div');credits.className='menu-credits';credits.textContent='Made by Ori Hanegby, Guy Corem, Orr Kowarsky';this.root.appendChild(credits);parent.appendChild(this.root);
    this.keyHandler=e=>{if((e.key==='Enter'||e.key===' ')&&!(e.target instanceof HTMLButtonElement)){e.preventDefault();onPlay();}};
    window.addEventListener('keydown',this.keyHandler);
  }
  destroy():void {window.removeEventListener('keydown',this.keyHandler);this.root.remove();}
}
