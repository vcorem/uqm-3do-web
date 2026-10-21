// Input-only test navigation. This uses public scanner positions, never moves
// the lander or grants progress directly. Swept hazard envelopes are avoided.
const distance=(a,b)=>Math.hypot(a.x-b.x,a.y-b.y);
const GRID=24;
const WIDTH=43,HEIGHT=30;
function safePoint(state,p){return state.hazards.every(h=>distance(p,{x:h.baseX??h.x,y:h.baseY??h.y})>h.radius+(h.amplitude||0)+28);}
function safeLine(state,a,b){const steps=Math.ceil(distance(a,b)/8);for(let i=0;i<=steps;i++){const t=i/(steps||1);if(!safePoint(state,{x:a.x+(b.x-a.x)*t,y:a.y+(b.y-a.y)*t}))return false;}return true;}
export function surfaceWaypoints(state,target){
    const start={x:state.x,y:state.y};if(safeLine(state,start,target))return [target];
    const point=index=>({x:12+(index%WIDTH)*GRID,y:12+Math.floor(index/WIDTH)*GRID});
    const index=p=>Math.max(0,Math.min(HEIGHT-1,Math.round((p.y-12)/GRID)))*WIDTH+Math.max(0,Math.min(WIDTH-1,Math.round((p.x-12)/GRID)));
    const first=index(start),last=index(target),open=new Set([first]),cost=new Map([[first,0]]),previous=new Map();
    const valid=new Map();const safe=id=>{if(!valid.has(id))valid.set(id,safePoint(state,point(id)));return valid.get(id);};
    let found=false;
    while(open.size){
        let current,lowest=Infinity;
        for(const id of open){const score=cost.get(id)+distance(point(id),target);if(score<lowest){lowest=score;current=id;}}
        if(current===last){found=true;break;}open.delete(current);
        const cx=current%WIDTH,cy=Math.floor(current/WIDTH);
        for(let dy=-1;dy<=1;dy++)for(let dx=-1;dx<=1;dx++){
            if((!dx&&!dy)||cx+dx<0||cx+dx>=WIDTH||cy+dy<0||cy+dy>=HEIGHT)continue;
            const next=(cy+dy)*WIDTH+cx+dx;if(!safe(next)||!safeLine(state,point(current),point(next)))continue;
            const score=cost.get(current)+GRID*Math.hypot(dx,dy);
            if(score<(cost.get(next)??Infinity)){cost.set(next,score);previous.set(next,current);open.add(next);}
        }
    }
    if(!found)throw new Error(`No safe surface path for ${state.mode}/${state.seed} to ${target.id??'pad'}`);
    const path=[target];let at=last;while(at!==first){path.unshift(point(at));at=previous.get(at);}path.unshift(start);
    const simplified=[];let cursor=0;
    while(cursor<path.length-1){let next=path.length-1;while(next>cursor+1&&!safeLine(state,path[cursor],path[next]))next--;simplified.push(path[next]);cursor=next;}
    return simplified;
}
export function steerSurface(state,target){
    const dx=target.x-state.x,dy=target.y-state.y,d=Math.hypot(dx,dy),speed=Math.min(140,d*3),vx=d?dx/d*speed:0,vy=d?dy/d*speed:0;
    return {x:(vx*4.3+(vx-state.vx)*8)/680,y:(vy*4.3+(vy-state.vy)*8)/680};
}
