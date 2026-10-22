#!/usr/bin/env python3
"""Original industrial-space combat score, rendered from an authored event score.

NumPy synthesis only: oversampled saturated string/reese voices, modeled drums,
filtered noise, FM metal and circular effects. No external audio is sampled.
Every release/reflection is accumulated modulo the exact64-bar loop length.
"""
from __future__ import annotations
import argparse
import hashlib
import json
import math
from pathlib import Path
import platform
import shutil
import subprocess
import tempfile
import wave
import numpy as np

RATE=44100
BARS=64
ROOT=Path(__file__).resolve().parent
SEED=0x554D5153
VERSION='martial-space-2'
TRACKS={
 'fleet-menu':{'title':'Fleet Mobilization','bpm':150,'key':'D Phrygian / D pedal','root':38,
  'sections':['Launch orders','War engines','Outer perimeter','Fleet formation','Tactical silence','Reactor charge','Launch corridor','Weapons ready'],
  'energy':[.78,.94,.87,1.,.52,.76,1.,.94]},
 'battle':{'title':'Hostile Vector','bpm':168,'key':'E Phrygian / E pedal','root':40,
  'sections':['Contact hostile','Weapons free','Flanking fire','Hull breach','Gravity trap','Target acquisition','Full assault','No retreat'],
  'energy':[.92,1.,.94,1.,.48,.78,1.,.98]},
}
# Authored16th-note riffs: (step, interval above tonic, gate in16th notes).
# Pedal notes, semitone friction and syncopated rests replace open major triads.
BATTLE_RIFFS=[
 [(0,0,1.5),(2,0,.65),(3,0,.65),(5,1,1.2),(7,0,.65),(8,0,1.5),(10,0,.65),(11,0,.65),(13,7,1.1),(15,1,.65)],
 [(0,0,1.5),(2,0,.65),(4,0,.65),(5,3,1.1),(7,1,.65),(8,0,1.5),(10,0,.65),(12,0,.65),(14,-2,1.5)],
 [(0,0,.65),(1,0,.65),(3,0,1.1),(5,1,.65),(6,0,.65),(8,0,1.5),(11,6,.65),(12,5,1.1),(14,1,1.5)],
 [(0,0,1.5),(3,0,.65),(4,0,.65),(6,1,1.1),(8,0,1.5),(11,3,.65),(12,1,1.1),(14,0,1.5)],
]
MENU_RIFFS=[
 [(0,0,1.5),(3,0,.8),(4,0,1.5),(7,1,.8),(8,0,1.5),(11,0,.8),(12,5,1.3),(15,1,.7)],
 [(0,0,1.5),(2,0,.8),(4,0,1.5),(7,3,.8),(8,0,1.5),(10,0,.8),(12,1,1.5),(15,0,.7)],
 [(0,0,1.5),(3,7,.8),(4,0,1.5),(6,0,.8),(8,1,1.5),(11,0,.8),(12,-2,1.5),(15,1,.7)],
 [(0,0,1.5),(3,0,.8),(4,0,1.5),(7,1,.8),(8,0,1.5),(10,0,.8),(12,0,1.5),(15,-2,.7)],
]
# A low, angular alarm motif. It states a phrase, leaves a rest, then answers;
# octave doubles appear only in the climax. No continuous tinkling arpeggio.
MOTIF_A=[[(0,12,.6),(1.5,13,.4),(2.5,12,.8)],[],[(.5,19,.5),(1.5,18,.4),(2.5,17,.9)],[],
         [(0,12,.6),(1.5,13,.4),(2.5,15,.8)],[(1,13,.5),(2,12,.9)],[(0,10,.7),(1.5,13,.4),(2.5,12,.8)],[]]
MOTIF_B=[[(0,19,1.1),(2,18,.6),(3,17,.6)],[(1,15,.6),(2.5,13,.8)],
         [(0,12,.7),(1.5,13,.4),(2.5,12,.8)],[],[(0,24,1),(1.5,25,.4),(2.5,24,.8)],
         [(0,22,.7),(1.5,19,.7),(3,18,.6)],[(0,17,.7),(1,15,.5),(2.5,13,.5)],[(0,12,1.1),(3,1,.5)]]

def hz(note): return 440*2**((note-69)/12)
def seed_for(label): return SEED+int.from_bytes(hashlib.sha256(str(label).encode()).digest()[:4],'little')
def db(value): return float(20*np.log10(max(float(value),1e-12)))

def filtered(x,rate,low=0,high=20000,peaks=()):
    """Zero-phase spectral shaping, with padded tails for per-voice processing."""
    n=1 << (len(x)-1).bit_length()
    f=np.fft.rfftfreq(n,1/rate)
    response=np.ones_like(f)
    if low: response*=1/np.sqrt(1+(low/np.maximum(f,.01))**6)
    if high: response*=1/np.sqrt(1+(f/high)**8)
    for center,width,gain in peaks:
        response*=10**((gain*np.exp(-.5*((f-center)/width)**2))/20)
    return np.fft.irfft(np.fft.rfft(x,n)*response,n)[:len(x)].astype(np.float32)

def envelope(t,duration,attack=.002,release=.035,decay=0):
    a=np.minimum(t/attack,1)
    r=np.minimum(np.maximum((duration+release-t)/release,0),1)**2
    return a*r*(.42+.58*np.exp(-t*decay) if decay else 1)

def normalized(x,peak=.95):
    return (x*(peak/max(float(np.max(np.abs(x))),1e-9))).astype(np.float32)

class Score:
    def __init__(self,name):
        self.name=name;self.spec=TRACKS[name];self.beat=60/self.spec['bpm']
        self.samples=round(BARS*4*self.beat*RATE)
        self.buses={n:np.zeros((self.samples,2),np.float32) for n in ['drums','bass','riff','signal','space','room']}
        self.events=[];self.cache={};self.kicks=[];self.harmonic_plan=[]

    def put(self,x,beat,volume,pan,bus):
        p=(pan+1)*np.pi/4
        sound=np.asarray(x,np.float32)[:,None]*np.array([np.cos(p),np.sin(p)],np.float32)*volume
        start=round(beat*self.beat*RATE)%self.samples
        for offset in range(0,len(sound),self.samples):
            part=sound[offset:offset+self.samples];n=min(len(part),self.samples-start)
            self.buses[bus][start:start+n]+=part[:n]
            if n<len(part):self.buses[bus][:len(part)-n]+=part[n:]

    def tone(self,kind,note,beats,variant):
        key=(kind,note,beats,variant)
        if key in self.cache:return self.cache[key]
        rng=np.random.default_rng(seed_for(key));duration=beats*self.beat
        rate=RATE*2 if kind in ['chug','reese','alarm','brass'] else RATE
        release={'chug':.026,'reese':.04,'alarm':.075,'brass':.11,'space':.9,'riser':.1}[kind]
        t=np.arange(math.ceil((duration+release)*rate),dtype=np.float64)/rate
        f=hz(note);phase=2*np.pi*f*t
        if kind=='chug':
            # Two detuned string excitations, pick scrape, two-stage preamp and
            # cabinet response.2xoversampling controls waveshaper aliasing.
            x=np.zeros_like(t)
            bend=.035*np.exp(-t*100)
            for detune in [1-.0009*(variant+1),1+.0012*(variant+1)]:
                ph=2*np.pi*f*detune*(t+.00035*(1-np.exp(-t*100)))
                for h in range(1,min(25,int(14000/f))):
                    x+=np.sin(ph*h+.09*variant*h)*np.exp(-t*h*1.8)/(h**.82)
            x+=filtered(rng.standard_normal(len(t)),rate,1400,7800)*np.exp(-t*170)*.2
            x=np.tanh(x*2.7)
            x=filtered(x,rate,105,8500,[(350,180,-3),(1000,650,5),(2400,1100,5)])
            x=np.tanh(x*1.65)
            x=filtered(x,rate,110,6200,[(3100,750,-3)])
            x*=envelope(t,duration,.0012,release,16)
        elif kind=='reese':
            x=np.sin(phase)*.85
            for ratio in [.997,1.003]:
                ph=phase*ratio+.13*np.sin(2*np.pi*.8*t)
                for h in range(2,12):x+=np.sin(ph*h+.11*variant)*(.5/h)
            x=np.tanh(x*2.3)
            x=filtered(x,rate,31,2400,[(100,80,2),(730,320,3)])
            x*=envelope(t,duration,.002,release,5)
        elif kind=='alarm':
            # Metallic phase modulation with a formant body, rather than a
            # soft pure-tone lead. The motif sits below the old score's bells.
            mod=(1.4+1.9*np.exp(-t*17))*np.sin(phase*2.003)
            x=np.sin(phase+mod)+.35*np.sin(phase*.501+.6*np.sin(phase*1.499))
            x=np.tanh(x*1.8)
            x=filtered(x,rate,180,5200,[(950,400,4),(2400,700,2)])
            x*=envelope(t,duration,.003,release,4)
        elif kind=='brass':
            x=np.zeros_like(t)
            for ratio in [.996,1.,1.004]:
                for h in range(1,min(18,int(12000/f))):x+=np.sin(phase*ratio*h+.06*variant*h)/h
            x=np.tanh(x*1.65)
            x=filtered(x,rate,130,3800,[(1000,600,3)])
            x*=envelope(t,duration,.022,release,5)
        elif kind=='riser':
            u=np.minimum(t/duration,1)
            noise=rng.standard_normal(len(t))
            low=filtered(noise,rate,500,2000)
            high=filtered(noise,rate,2500,8500)
            x=(low*(1-u)+high*u)*u*u
            x+=.12*np.sin(2*np.pi*(f*t+f*t*t/(2*duration)))*u*u
            x*=envelope(t,duration,.08,release)
        else:
            # An uneasy octave/fifth pedal with inharmonic machinery, no
            # sustained major/minor chord wash or melodic pluck layer.
            noise=filtered(rng.standard_normal(len(t)),rate,250,3300)
            x=.3*np.sin(phase)+.16*np.sin(phase*1.5+.4*np.sin(t*1.7))
            x+=.1*np.sin(phase*2.014)+.1*noise
            x*=envelope(t,duration,.65,release)*(.8+.2*np.sin(t*5.1)**2)
        if rate!=RATE:
            x=filtered(x,rate,0,15000)[::2]
        x=normalized(x)
        # Explicit endpoint gates prevent a synthesized voice's FFT wrap from
        # making a click; this is not a fade at the complete track boundary.
        ramp=min(round(.001*RATE),len(x)//2)
        x[:ramp]*=np.linspace(0,1,ramp);x[-ramp:]*=np.linspace(1,0,ramp)
        self.cache[key]=x
        return x

    def note(self,kind,note,beat,duration,volume,pan=0,variant=0):
        bus={'chug':'riff','reese':'bass','alarm':'signal','brass':'signal','space':'space','riser':'space'}[kind]
        x=self.tone(kind,note,duration,variant)
        self.put(x,beat,volume,pan,bus)
        if kind in ['alarm','brass']:self.put(x,beat,volume*.09,pan,'room')
        self.events.append({'voice':kind,'beat':round(beat,5),'midi':note,'duration':duration,'velocity':round(volume,4),'pan':pan,'variation':variant})

    def drum(self,kind,beat,volume=1,pan=0,variant=0):
        key=(kind,variant)
        if key not in self.cache:
            duration={'kick':.36,'snare':.28,'hat':.065,'open':.21,'ride':.33,'crash':1.05,
                      'tom-low':.43,'tom-mid':.33,'tom-high':.25,'impact':1.2,'metal':.13}[kind]
            t=np.arange(round(duration*RATE))/RATE
            rng=np.random.default_rng(seed_for(key));noise=rng.standard_normal(len(t))
            if kind=='kick':
                phase=2*np.pi*(49*t+2.1*(1-np.exp(-t*75))+.28*(1-np.exp(-t*260)))
                sub=np.sin(phase)*np.exp(-t*14)
                punch=np.sin(2*np.pi*103*t)*np.exp(-t*65)*.28
                click=filtered(noise,RATE,1700,7800)*np.exp(-t*620)*.11
                x=np.tanh((sub+punch)*1.8)*.8+click
            elif kind=='snare':
                body=(np.sin(2*np.pi*(174*t+.2*(1-np.exp(-t*60))))+.45*np.sin(2*np.pi*322*t))*np.exp(-t*25)
                crack=filtered(noise,RATE,700,10500)*np.exp(-t*29)
                wire=filtered(np.roll(noise,117),RATE,2200,12500)*np.exp(-t*17)
                x=np.tanh(body*1.1)*.38+crack*.70+wire*.20
                x+=filtered(noise,RATE,1500,7000)*np.exp(-t*400)*.30
            elif kind.startswith('tom'):
                f={'tom-low':57,'tom-mid':84,'tom-high':123}[kind]
                ph=2*np.pi*(f*t+.7*(1-np.exp(-t*30)))
                x=(np.sin(ph)+.24*np.sin(ph*1.59))*np.exp(-t*9)
                x=np.tanh(x*1.45)+filtered(noise,RATE,650,4500)*np.exp(-t*100)*.08
            elif kind=='impact':
                x=np.sin(2*np.pi*(34*t+1.4*(1-np.exp(-t*14))))*np.exp(-t*5)
                x+=filtered(noise,RATE,120,4800)*np.exp(-t*5)*.22
                x=np.tanh(x*1.4)
            else:
                metal=np.zeros_like(t)
                for f in [3173,4231,5897,7193,9281]:metal+=np.sin(2*np.pi*f*t+1.3*np.sin(2*np.pi*f*1.414*t))*.10
                noise=filtered(noise,RATE,4500 if kind!='metal' else 1300,15000)
                decay={'hat':95,'open':23,'ride':18,'crash':5,'metal':38}[kind]
                x=(noise*.7+metal*.3)*np.exp(-t*decay)
            x*=np.minimum(t/.0007,1)*np.minimum((duration-t)/.006,1)
            self.cache[key]=normalized(x)
        self.put(self.cache[key],beat,volume,pan,'drums')
        if kind in ['snare','tom-low','tom-mid','tom-high','impact']:
            self.put(self.cache[key],beat,volume*.095,pan,'room')
        if kind=='kick':self.kicks.append((beat,volume))
        self.events.append({'voice':kind,'beat':round(beat,5),'midi':None,'duration':None,'velocity':round(volume,4),'pan':pan,'variation':variant})

    def arrange(self):
        battle=self.name=='battle';root=self.spec['root']
        for bar in range(BARS):
            section=bar//8;phrase=bar%8;b=bar*4;energy=self.spec['energy'][section]
            tactical=section==4;build=section==5
            pedal_shift=1 if section==2 and phrase in [2,3,6] else -2 if section in [3,6] and phrase==6 else 0
            self.harmonic_plan.append({'bar':bar+1,'pedal_midi':root+pedal_shift,'mode':'Phrygian','riff':phrase%4})
            if phrase%2==0:self.note('space',root+12,b,7.8,.047 if tactical else .029,(-1 if phrase%4 else 1)*.6,phrase%4)
            riff=(BATTLE_RIFFS if battle else MENU_RIFFS)[(phrase+(1 if section==2 else 0))%4]
            if tactical:riff=[(0,0,2.5),(6,1,1.2),(8,0,2.5),(14,0,1.2)] if phrase<4 else riff[::2]
            if build and phrase<4:riff=riff[::2]
            # Very short coordinated breaks emphasize the next impact, rather
            # than leaving96seconds of mechanically identical energy.
            cutoff=12 if phrase==7 and section in [1,3,6] else 16
            for j,(step,interval,gate) in enumerate(riff):
                if step>=cutoff:continue
                beat=b+step/4;duration=gate/4;note=root+interval+pedal_shift
                self.note('reese',note-12,beat,duration,.38*energy,0,(bar+j)%3)
                self.note('chug',note,beat,duration,.48*energy,-.63,(bar+j)%4)
                self.note('chug',note+7,beat+.013,duration*.96,.32*energy,.63,(bar+j+2)%4)
                if section in [3,6] and j%3==0:self.note('chug',note+12,beat+.007,duration,.10,.22,(j+1)%4)
            motif=MOTIF_B if section in [2,3,6] else MOTIF_A
            if not tactical and not (build and phrase<4):
                for off,interval,duration in motif[phrase]:
                    self.note('alarm',root+interval,b+off,duration,.26*energy,-.10,phrase%3)
                    if section in [3,6]:self.note('brass',root+interval,b+off+.012,duration,.13*energy,.32,phrase%2)
            if phrase in [0,4] and section!=4:
                for interval,pan in [(0,-.3),(7,.3)]:self.note('brass',root+12+interval,b,.65,.13*energy,pan,section%3)
            if tactical:
                kicks=[0,2.75] if phrase<4 else [0,1.5,2.75]
                snares=[2]
            elif battle:
                kicks=[[0,.75,1.75,2.5,3.5],[0,.5,1.5,2.25,2.75,3.75],[0,1.5,2,2.75,3.5],[0,.75,1.5,2.5,3.25]][phrase%4]
                snares=[1,3]
            else:
                kicks=[[0,1.5,2,3.5],[0,.75,2,2.75],[0,1.5,2.5,3.5],[0,.5,2,3.25]][phrase%4]
                snares=[1,3]
            for step in kicks:
                if step<cutoff/4:self.drum('kick',b+step,.78 if battle else .76,variant=bar%2)
            for step in snares:
                if step<cutoff/4:self.drum('snare',b+step,.72*max(energy,.72),variant=phrase%3)
            if not tactical and phrase%2==1:
                for step in [2.75,3.75]:
                    if step<cutoff/4:self.drum('snare',b+step,.12*energy,.08,variant=1)
            for step in range(8 if not tactical else 4):
                at=step*.5 if not tactical else step
                if at>=cutoff/4:continue
                self.drum('hat',b+at,(.11 if step%2 else .16)*energy,(-1 if step%2 else 1)*.28,variant=step%3)
                if battle and section in [1,3,6,7] and step in [1,5]:self.drum('hat',b+at+.25,.09,-.32,variant=2)
            if not tactical:
                self.drum('open',b+1.5,.17*energy,.36)
                if cutoff==16:self.drum('ride' if section in [3,6] else 'open',b+3.5,.14*energy,.4)
            # Marching low toms and metallic answer hits give the fleet track
            # launch-preparation urgency even outside its full drum sections.
            if not battle or tactical:
                for step,kind in [(0,'tom-low'),(.75,'tom-mid'),(2.5,'tom-low'),(3.25,'tom-high')]:
                    self.drum(kind,b+step,.23*energy,(-.22 if step<2 else .22))
            if phrase in [0,4]:self.drum('metal',b+2.75,.12*energy,-.5)
            if phrase==0:
                self.drum('impact',b,.42 if not tactical else .24)
                self.drum('crash',b,.22*energy,-.35)
            if phrase==7:
                fill_start=2 if section in [1,3,6] else 3
                for j,step in enumerate(np.arange(fill_start,4,.25)):
                    kind=['snare','snare','tom-high','tom-mid','snare','tom-mid','tom-low','tom-low'][j%8]
                    self.drum(kind,b+float(step),(.27+j*.027)*max(.75,energy),(j%5-2)*.13,variant=j%2)
                self.note('riser',root+12,b+2,1.96,.105,-.4,section%3)
            if build and phrase>=4:
                for j,step in enumerate(np.arange(0,4,.5 if phrase<7 else .25)):
                    self.drum('snare',b+float(step),.10+.025*phrase+.007*j,(-1 if j%2 else 1)*.1,variant=j%3)
            if tactical and phrase==3:
                self.note('alarm',root+13,b+2,1.3,.14,.15)
        return self

    def finish(self):
        # Sample-accurate kick ducking of tonal buses retains drum transients.
        duck=np.zeros(self.samples,np.float32)
        t=np.arange(round(.17*RATE))/RATE
        for beat,velocity in self.kicks:
            env=np.exp(-t/0.055)*velocity
            start=round(beat*self.beat*RATE)%self.samples;n=min(len(env),self.samples-start)
            duck[start:start+n]=np.maximum(duck[start:start+n],env[:n])
            if n<len(env):duck[:len(env)-n]=np.maximum(duck[:len(env)-n],env[n:])
        for bus,amount in [('bass',.46),('riff',.38),('signal',.24),('space',.38)]:self.buses[bus]*=(1-duck*amount)[:,None]
        room=self.buses['room'];room_mix=np.zeros_like(room)
        for seconds,gain in [(.023,.38),(.041,.30),(.071,.21),(.107,.13),(.151,.07)]:
            delay=round(seconds*RATE)
            room_mix[:,0]+=np.roll(room[:,1],delay)*gain
            room_mix[:,1]+=np.roll(room[:,0],delay+337)*gain
        # Tiny offbeat signal reflection adds depth without a wash that masks
        # native weapon effects. Every reflection wraps at the exact loop edge.
        signal=self.buses['signal']
        for beats,gain in [(.75,.105),(1.5,.04)]:
            delay=round(beats*self.beat*RATE)
            room_mix[:,0]+=np.roll(signal[:,1],delay)*gain
            room_mix[:,1]+=np.roll(signal[:,0],delay+211)*gain
        self.bus_levels={name:round(db(np.sqrt(np.mean(bus**2))),3) for name,bus in self.buses.items() if name!='room'}
        mix=room_mix
        for name in ['drums','bass','riff','signal','space']:mix+=self.buses[name]
        mix-=mix.mean(axis=0)
        mix=np.tanh(mix*.86)/.86
        # Global gain only: battle targets about-14LUFS, menu about-15.5LUFS.
        # The peak cap preserves at least1.8dB PCM headroom for encoding, without
        # flattening the transients or adding whole-track gain pumping.
        peak=np.max(np.abs(mix));rms=np.sqrt(np.mean(mix**2))
        target_rms=-14.2 if self.name=='battle' else -15.5
        mix*=min(10**(-1.8/20)/peak,10**(target_rms/20)/rms)
        return mix.astype(np.float32)

def render(name,output,keep_wav):
    score=Score(name).arrange();pcm=score.finish();raw=np.round(pcm*32767).astype('<i2').tobytes()
    output.mkdir(parents=True,exist_ok=True)
    with tempfile.TemporaryDirectory(prefix='uqm-martial-score-') as temp:
        wav=Path(temp)/f'{name}.wav'
        with wave.open(str(wav),'wb') as stream:
            stream.setnchannels(2);stream.setsampwidth(2);stream.setframerate(RATE);stream.writeframes(raw)
        encoders=subprocess.check_output(['ffmpeg','-hide_banner','-encoders'],text=True,stderr=subprocess.DEVNULL)
        encoder='libvorbis' if 'libvorbis' in encoders else 'vorbis'
        target=output/f'{name}.ogg'
        subprocess.run(['ffmpeg','-hide_banner','-loglevel','error','-y','-i',str(wav),'-map_metadata','-1','-c:a',encoder,'-strict','experimental','-q:a','6','-fflags','+bitexact','-flags:a','+bitexact','-metadata',f'title={score.spec["title"]}','-metadata','artist=Impulse original score','-metadata',f'comment=Original martial space score {VERSION}; no third-party samples',str(target)],check=True)
        if keep_wav:shutil.copyfile(wav,output/f'{name}.wav')
    section_metrics=[]
    for i,title in enumerate(score.spec['sections']):
        part=pcm[round(i*score.samples/8):round((i+1)*score.samples/8)]
        section_metrics.append({'first_bar':i*8+1,'bars':8,'name':title,'rms_dbfs':round(db(np.sqrt(np.mean(part**2))),3)})
    summary={'file':f'{name}.ogg','title':score.spec['title'],'version':VERSION,'bpm':score.spec['bpm'],'key':score.spec['key'],'bars':BARS,'beats_per_bar':4,'sample_rate':RATE,'channels':2,'samples':score.samples,'seconds':score.samples/RATE,'encoder':encoder,'sections':section_metrics,'harmonic_plan':score.harmonic_plan,'event_count':len(score.events),'instrument_event_counts':{kind:sum(e['voice']==kind for e in score.events) for kind in sorted({e['voice'] for e in score.events})},'bus_rms_dbfs_before_master':score.bus_levels,'peak_dbfs':round(db(np.max(np.abs(pcm))),3),'rms_dbfs':round(db(np.sqrt(np.mean(pcm**2))),3),'seam_step':float(np.max(np.abs(pcm[-1]-pcm[0]))),'pcm_s16le_sha256':hashlib.sha256(raw).hexdigest(),'sha256':hashlib.sha256(target.read_bytes()).hexdigest(),'bytes':target.stat().st_size,'resource':'music.meleemenu' if name=='fleet-menu' else 'music.battle','resource_value':f'MUSICRES:addons/remastered/music/{name}.ogg'}
    (output/f'{name}.score.json').write_text(json.dumps({**summary,'events':score.events},indent=2)+'\n')
    print(json.dumps({k:summary[k] for k in ['file','title','seconds','event_count','peak_dbfs','rms_dbfs','seam_step','bytes']}),flush=True)
    return summary

def main():
    ap=argparse.ArgumentParser(description=__doc__);ap.add_argument('--track',choices=[*TRACKS,'all'],default='all');ap.add_argument('--output',type=Path,default=ROOT);ap.add_argument('--keep-wav',action='store_true');args=ap.parse_args()
    tracks=[render(n,args.output,args.keep_wav) for n in (TRACKS if args.track=='all' else [args.track])]
    if args.track=='all':
        manifest={'schema_version':2,'collection':'Impulse original Melee score','version':VERSION,'provenance':'Original authored Phrygian riffs, alarm motifs and martial drum arrangements; deterministic oversampled synthesis and modeled percussion. No upstream music, sampled audio, soundfont or third-party melody.','render_command':'python3 hd/remastered/music/render.py --keep-wav','dependencies':['Python3','NumPy','ffmpeg with Vorbis encoder'],'loop_method':'64 whole bars; note releases, short room reflections and signal echoes wrap modulo exact PCM sample count. No track-boundary fade or inserted silence.','seed':SEED,'source_sha256':hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),'render_toolchain':{'python':platform.python_version(),'numpy':np.__version__,'ffmpeg':subprocess.check_output(['ffmpeg','-version'],text=True).splitlines()[0]},'tracks':tracks}
        (args.output/'manifest.json').write_text(json.dumps(manifest,indent=2)+'\n')
if __name__=='__main__':main()
