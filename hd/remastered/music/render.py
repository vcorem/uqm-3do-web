#!/usr/bin/env python3
"""Render two original 64-bar electronic scores without samples or soundfonts.

Python 3 + NumPy + ffmpeg (Vorbis encoder) are the only dependencies. Events, note
names, section arrangement and synthesis are intentionally reviewable here.
All note tails, echoes and room reflections wrap into the loop start. No fade
is used to conceal a discontinuity. The first and last bars share a downbeat.
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

RATE = 44100
BARS = 64
ROOT = Path(__file__).resolve().parent
SEED = 0x554D5152
NOTE_OFFSETS = {'C': 0, 'C#': 1, 'D': 2, 'Eb': 3, 'E': 4, 'F': 5, 'F#': 6,
                'G': 7, 'Ab': 8, 'A': 9, 'Bb': 10, 'B': 11}

def midi(name):
    return NOTE_OFFSETS[name[:-1]] + (int(name[-1]) + 1) * 12

# Chords spell the voice leading, rather than generating random pitches.
CHORDS = {
    'Em': ['E3', 'G3', 'B3', 'E4'], 'C': ['C3', 'G3', 'C4', 'E4'],
    'G': ['G2', 'B3', 'D4', 'G4'], 'D': ['D3', 'A3', 'D4', 'F#4'],
    'Am': ['A2', 'E3', 'A3', 'C4'], 'B': ['B2', 'F#3', 'B3', 'Eb4'],
    'Dm': ['D3', 'A3', 'D4', 'F4'], 'Bb': ['Bb2', 'F3', 'Bb3', 'D4'],
    'F': ['F3', 'A3', 'C4', 'F4'], 'Cm': ['C3', 'G3', 'C4', 'Eb4'],
    'Gm': ['G2', 'D3', 'G3', 'Bb3'], 'A': ['A2', 'E3', 'A3', 'C#4'],
}

# Melodies are eight-bar phrases. Tuple = beat within bar, note, duration in beats.
# Long notes and rests leave space around the syncopated answer figures.
MENU_A = [
 [(0,'B4',.75),(1,'E5',.75),(2,'G5',1.5),(3.5,'F#5',.5)],
 [(0,'E5',1.5),(2,'D5',.5),(2.75,'B4',1)],
 [(0,'D5',.75),(1,'G5',.75),(2,'B5',1),(3.25,'A5',.5)],
 [(0,'F#5',1.5),(2,'E5',.5),(2.75,'D5',1)],
 [(0,'E5',.75),(1,'G5',.75),(2,'E5',.5),(2.75,'D5',.5),(3.5,'C5',.5)],
 [(0,'B4',.75),(1,'D5',.75),(2,'G5',1.5)],
 [(0,'F#5',.75),(1,'A5',.75),(2,'F#5',.5),(2.75,'E5',1)],
 [(0,'Eb5',1),(1.5,'F#5',.5),(2.5,'B4',1.25)],
]
MENU_B = [
 [(0,'G5',1.5),(2,'E5',.5),(3,'D5',.75)],
 [(0,'C5',.75),(1,'E5',.75),(2,'A5',1.5)],
 [(0,'B5',.75),(1,'A5',.75),(2,'G5',1),(3.25,'D5',.5)],
 [(0,'F#5',2),(2.5,'A5',.5),(3.25,'F#5',.5)],
 [(0,'G5',.75),(1,'E5',.75),(2,'D5',.5),(3,'C5',.75)],
 [(0,'E5',1.5),(2,'C5',.75),(3,'A4',.75)],
 [(0,'F#5',1),(1.5,'E5',.5),(2.5,'Eb5',.5),(3.25,'F#5',.5)],
 [(0,'B4',2.5),(3,'B5',.5),(3.5,'A5',.5)],
]
BATTLE_A = [
 [(0,'D5',.5),(.75,'D5',.25),(1.5,'F5',.5),(2.25,'A5',.75),(3.25,'G5',.5)],
 [(0,'F5',1),(1.5,'D5',.5),(2.5,'Bb4',.5),(3.25,'D5',.5)],
 [(0,'C5',.5),(.75,'F5',.5),(1.5,'A5',.75),(2.5,'C6',.5),(3.25,'A5',.5)],
 [(0,'G5',1.25),(1.5,'E5',.5),(2.25,'D5',.5),(3,'C5',.75)],
 [(0,'D5',.5),(.75,'F5',.5),(1.5,'A5',.5),(2.25,'D6',.75),(3.25,'C6',.5)],
 [(0,'Bb5',1),(1.5,'A5',.5),(2.25,'F5',.5),(3,'D5',.75)],
 [(0,'G5',.5),(.75,'Bb5',.5),(1.5,'A5',.5),(2.25,'G5',.5),(3,'F5',.75)],
 [(0,'E5',.75),(1,'C#5',.5),(1.75,'E5',.5),(2.5,'A5',1)],
]
BATTLE_B = [
 [(0,'D6',1.5),(2,'C6',.5),(2.75,'A5',.75)],
 [(0,'Bb5',.75),(1,'A5',.5),(1.75,'F5',.5),(2.5,'D5',1)],
 [(0,'A5',1),(1.5,'C6',.5),(2.25,'A5',.5),(3,'G5',.75)],
 [(0,'G5',.75),(1,'E5',.5),(1.75,'G5',.5),(2.5,'C6',1)],
 [(0,'Bb5',.75),(1,'A5',.5),(1.75,'G5',.5),(2.5,'D5',1)],
 [(0,'F5',.5),(.75,'Bb5',.5),(1.5,'D6',1),(3,'C6',.75)],
 [(0,'C#6',.75),(1,'A5',.5),(1.75,'E5',.5),(2.5,'G5',1)],
 [(0,'E5',1),(1.5,'C#5',.5),(2.5,'A4',1)],
]
TRACKS = {
 'fleet-menu': {
  'title': 'Constellation Assembly', 'bpm': 144, 'key': 'E minor',
  'chords': ['Em','C','G','D','C','G','D','B'] * 2 +
            ['C','Am','G','D','C','Am','B','B'] +
            ['Em','C','G','D','C','G','D','B'] +
            ['C','Am','G','D','C','Am','B','B'] +
            ['Em','C','G','D','C','G','D','B'] * 3,
  'sections': ['Assembly','Main theme','Open orbit','Main theme + octave',
               'Signal break','Ignition','Full fleet','Return vector'],
  'energy': [.70,.88,.78,1.,.38,.72,1.,.87],
  'melodies': [MENU_A,MENU_A,MENU_B,MENU_A,MENU_B,MENU_A,MENU_B,MENU_A],
 },
 'battle': {
  'title': 'Redshift Intercept', 'bpm': 160, 'key': 'D minor',
  'chords': ['Dm','Bb','F','C','Dm','Bb','Gm','A'] * 8,
  'sections': ['Intercept','Engage','Counterattack','Afterburner',
               'Gravity well','Weapons armed','All engines','Pursuit'],
  'energy': [.90,1.,.92,1.,.55,.82,1.,1.],
  'melodies': [BATTLE_A,BATTLE_A,BATTLE_B,BATTLE_A,BATTLE_B,BATTLE_A,BATTLE_B,BATTLE_A],
 }
}

class Score:
    def __init__(self, name):
        self.name=name; self.spec=TRACKS[name]; self.beat=60/self.spec['bpm']
        self.samples=round(BARS*4*self.beat*RATE)
        self.mix=np.zeros((self.samples,2),np.float32)
        self.room=np.zeros_like(self.mix); self.echo=np.zeros_like(self.mix)
        self.rng=np.random.default_rng(SEED + (1 if name=='battle' else 0))
        self.events=[]; self.cache={}; self.kicks=[]

    def put(self, sound, beat, volume, pan=0., bus='mix'):
        x=np.asarray(sound,np.float32)*volume
        p=(pan+1)*math.pi/4
        stereo=x[:,None]*np.array([math.cos(p),math.sin(p)],np.float32)
        dest=getattr(self,bus); start=round(beat*self.beat*RATE)%self.samples
        n=min(len(x),self.samples-start); dest[start:start+n]+=stereo[:n]
        if n<len(x): dest[:len(x)-n]+=stereo[n:]

    def tone(self, kind, note, duration, velocity=1.):
        dur=duration*self.beat
        key=(kind,note,round(dur,5))
        if key in self.cache: return self.cache[key]
        release={'bass':.065,'pluck':.28,'lead':.20,'guitar':.12,'pad':.50}[kind]
        n=math.ceil((dur+release)*RATE); t=np.arange(n,dtype=np.float32)/RATE
        freq=440*2**((note-69)/12)
        gate=np.minimum(t/max(.004, min(.028,dur*.12)),1)
        gate*=np.minimum(np.maximum((dur+release-t)/release,0),1)
        gate[-1]=0
        if kind=='bass':
            phase=2*np.pi*freq*t
            # Solid fundamental, moderate odd-harmonic bite, click-free release.
            out=np.sin(phase)*.65
            for h in range(2,9): out+=np.sin(phase*h)*(.35/h)*np.exp(-t*h*2)
            out=np.tanh(out*1.8)*(.68+.32*np.exp(-t*13))*gate
        elif kind=='pad':
            out=np.zeros(n,np.float32)
            for h in [1,2,3,4,6]:
                for detune in [-.0018,.0018]:
                    out+=np.sin(2*np.pi*freq*h*(1+detune)*t)/(h*h*2)
            gate=np.minimum(t/.22,1)*np.minimum(np.maximum((dur+release-t)/release,0),1)
            out=out*gate*.62
        elif kind=='pluck':
            out=np.zeros(n,np.float32)
            phase=2*np.pi*freq*t
            for h in range(1,min(10,int(RATE/2/freq))):
                out+=np.sin(phase*h+.12*h)*np.exp(-t*(4+h*2.5))/h
            out*=gate*.60
        elif kind=='guitar':
            out=np.zeros(n,np.float32)
            # A damped power-chord string voice, mildly saturated, no samples.
            for ratio in [1.,1.0017,1.4983]:
                for h in range(1,min(15,int(15000/(freq*ratio)))):
                    out+=np.sin(2*np.pi*freq*ratio*h*t+.06*h)*np.exp(-t*(3+h*.7))/(h*3)
            out=np.tanh(out*2.3)*gate*.60
        else:
            vibrato=.003*np.sin(2*np.pi*5.1*t)*np.minimum(t/.22,1)
            phase=2*np.pi*freq*np.cumsum(1+vibrato)/RATE
            out=np.zeros(n,np.float32)
            for h in range(1,min(12,int(16000/freq))):
                out+=np.sin(phase*h)*(.68**(h-1))/h
            out+=.18*np.sin(phase*1.0025)
            out=np.tanh(out*1.22)*gate*(.75+.25*np.exp(-t*8))*.65
        self.cache[key]=out.astype(np.float32)
        return self.cache[key]

    def note(self,kind,note,beat,duration,volume,pan=0,echo=.0):
        note=midi(note) if isinstance(note,str) else note
        sound=self.tone(kind,note,duration)
        self.put(sound,beat,volume,pan)
        if kind in ['lead','pad','guitar']: self.put(sound,beat,volume*.16,pan,bus='room')
        if echo: self.put(sound,beat,volume*echo,pan,bus='echo')
        self.events.append([kind,round(beat,3),int(note),round(duration,3),round(volume,3)])

    def drum(self,kind,beat,volume=1.,pan=0):
        if kind not in self.cache:
            dur={'kick':.43,'snare':.24,'clap':.19,'hat':.07,'open':.29,'crash':1.6,
                 'tom':.28,'rim':.055}[kind]
            t=np.arange(round(RATE*dur),dtype=np.float32)/RATE
            noise=self.rng.standard_normal(len(t)).astype(np.float32)
            hp=np.concatenate(([0],np.diff(noise)))
            if kind=='kick':
                freq=46+105*np.exp(-t*48)
                x=np.sin(2*np.pi*np.cumsum(freq)/RATE)*np.exp(-t*11)
                x+=hp*.04*np.exp(-t*500)
                x=np.tanh(x*1.25)*.90
            elif kind=='snare':
                body=(np.sin(2*np.pi*183*t)+.48*np.sin(2*np.pi*331*t))*np.exp(-t*31)
                x=(body*.40+hp*.21*np.exp(-t*22))*(1-np.exp(-t*1800))
            elif kind=='clap':
                env=sum(np.exp(-np.maximum(t-a,0)*90)*(t>=a) for a in [0,.012,.024])
                x=hp*.14*env+hp*.045*np.exp(-t*17)
            elif kind in ['hat','open']:
                env=np.exp(-t*(70 if kind=='hat' else 15))
                metal=sum(np.sin(2*np.pi*f*t) for f in [4831,5937,7193,8291])*.026
                x=(hp*.11+metal)*env
            elif kind=='crash':
                env=np.exp(-t*3)*(1-np.exp(-t*1400))
                metal=sum(np.sin(2*np.pi*f*t) for f in [3181,4093,5309,6917,8353])*.020
                x=(hp*.08+metal)*env
            elif kind=='rim':
                x=(np.sin(2*np.pi*1421*t)+np.sin(2*np.pi*1931*t))*.20*np.exp(-t*100)
            else:
                freq=90+65*np.exp(-t*25)
                x=(np.sin(2*np.pi*np.cumsum(freq)/RATE)+noise*.025)*np.exp(-t*14)*.65
            x*=np.minimum(t/.0005,1)*np.minimum((dur-t)/.004,1)
            self.cache[kind]=x.astype(np.float32)
        self.put(self.cache[kind],beat,volume,pan)
        if kind in ['snare','clap','crash','tom']: self.put(self.cache[kind],beat,volume*.12,pan,bus='room')
        if kind=='kick': self.kicks.append(beat)
        self.events.append([kind,round(beat,3),None,None,round(volume,3)])

    def arrange(self):
        battle=self.name=='battle'
        for bar in range(BARS):
            section=bar//8; phrase=bar%8; beat=bar*4; energy=self.spec['energy'][section]
            chord=[midi(n) for n in CHORDS[self.spec['chords'][bar]]]
            root=chord[0]
            while root>42: root-=12
            # Chord-bed is deliberately sparse and low behind the lead.
            for i,n in enumerate(chord[1:]):
                self.note('pad',n,beat,3.9,.075*energy,[-.65,.15,.65][i])
            # Melodic eighth-note bass with octave and fifth pickups.
            bass_pattern=[(0,0,.42),( .75,0,.22),(1.5,12,.36),(2,0,.4),(2.75,7,.2),(3.5,0,.36)]
            if section==4: bass_pattern=[(0,0,1.4),(2,12,.7),(3.5,7,.35)]
            for b,offset,d in bass_pattern:
                self.note('bass',root+offset,beat+b,d,.33*energy)
            # Stereo picked power chords supply space-rock weight in the battle.
            if battle and section!=4 or not battle and section in [3,6]:
                for b in [0,.75,1.5,2.5,3.25]:
                    self.note('guitar',root+12,beat+b,.34,.12*energy,-.58)
                    self.note('guitar',root+12,beat+b+.012,.32,.11*energy,.58)
            # A sixteenth arp changes register and articulation with the phrase.
            steps=range(16) if section!=4 else range(0,16,2)
            order=[0,2,1,3,2,1,3,2,0,2,3,1,2,3,1,2]
            for k in steps:
                n=chord[order[(k+phrase*2)%16]]+12
                if k in [3,7,11,15]: n+=12
                self.note('pluck',n,beat+k*.25,.15,.055*energy*(1 if k%2 else 1.3),
                          -.42 if k%2 else .42,echo=.11)
            melody=self.spec['melodies'][section][phrase]
            # Break: first half becomes a high, bell-like question; second answers.
            for b,n,d in melody:
                kind='pluck' if section==4 else 'lead'
                self.note(kind,n,beat+b,d,.15*energy if battle else .14*energy,echo=.22)
                if section in [3,6] and b in [0,2]:
                    self.note('lead',midi(n)-12,beat+b,d,.045*energy,-.18,echo=.1)
            # Rhythmic roles: a dance kick foundation and rock backbeat; breaks
            # retain pulse rather than disappearing into a drone.
            kicks=[0,1.5,2,2.75,3.5] if battle else [0,1,2,3]
            if section==4: kicks=[0,2]
            for k in kicks: self.drum('kick',beat+k,.58 if battle else .54)
            for k in [1,3]:
                self.drum('snare',beat+k,.67*energy)
                self.drum('clap',beat+k+.006,.28*energy,.12)
            for k in range(8):
                self.drum('hat',beat+k*.5,.36*energy if k%2 else .25*energy,
                          -.30 if k%2 else .30)
            if section!=4:
                for k in [.75,2.75]: self.drum('open',beat+k,.30*energy,.35)
                for k in [1.75,3.75]: self.drum('hat',beat+k,.18*energy,-.35)
            else:
                for k in [.75,2.75]: self.drum('rim',beat+k,.3,-.25)
            if phrase==0: self.drum('crash',beat,.43*energy,-.3)
            if phrase==7:
                # Local sixteenth fill creates a clean pickup into each section.
                for j,k in enumerate([2.5,2.75,3,3.25,3.5,3.75]):
                    self.drum('tom' if j>2 else 'snare',beat+k,.23+j*.035,(j-2.5)*.16)
                self.drum('open',beat+3.5,.3,.3)
        return self

    def finish(self):
        # All reflections wrap. A tail at 255.9 beats is heard at beat zero on
        # subsequent loops, making the PCM itself periodic and repeatable.
        for beats,gain in [(.75,.65),(1.5,.33),(2.25,.15)]:
            n=round(beats*self.beat*RATE)
            self.mix[:,0]+=np.roll(self.echo[:,1],n)*gain
            self.mix[:,1]+=np.roll(self.echo[:,0],n+round(.009*RATE))*gain
        for seconds,gain in [(.037,.45),(.061,.32),(.089,.24),(.131,.19),(.179,.14),(.233,.10),(.307,.065)]:
            n=round(seconds*RATE)
            self.mix[:,0]+=np.roll(self.room[:,1],n)*gain
            self.mix[:,1]+=np.roll(self.room[:,0],n+round(.011*RATE))*gain
        # Remove DC, gentle soft clip catches coincident transients, then retain
        # 1.4 dB peak headroom for perceptual encoding. No audio normalization tool
        # or random plugin settings are required to reproduce the mix.
        self.mix-=self.mix.mean(axis=0)
        self.mix=np.tanh(self.mix*1.10)
        self.mix*=10**(-1.4/20)/np.max(np.abs(self.mix))
        return self.mix


def render(name,output,keep_wav=False):
    score=Score(name).arrange(); pcm=score.finish()
    raw=np.round(pcm*32767).astype('<i2').tobytes()
    output.mkdir(parents=True,exist_ok=True)
    with tempfile.TemporaryDirectory(prefix='uqm-score-') as temp:
        wav=Path(temp)/f'{name}.wav'
        with wave.open(str(wav),'wb') as stream:
            stream.setnchannels(2); stream.setsampwidth(2); stream.setframerate(RATE)
            stream.writeframes(raw)
        target=output/f'{name}.ogg'
        encoders=subprocess.run(['ffmpeg','-hide_banner','-encoders'],capture_output=True,text=True,check=True).stdout
        encoder='libvorbis' if 'libvorbis' in encoders else 'vorbis'
        subprocess.run(['ffmpeg','-hide_banner','-loglevel','error','-y','-i',str(wav),
                        '-map_metadata','-1','-c:a',encoder,'-strict','experimental','-q:a','6','-fflags','+bitexact',
                        '-flags:a','+bitexact','-metadata',f'title={score.spec["title"]}',
                        '-metadata','artist=Impulse original score',
                        '-metadata','comment=Original score and synthesis; no third-party samples',
                        str(target)],check=True)
        if keep_wav: shutil.copyfile(wav,output/f'{name}.wav')
    summary={
        'file':f'{name}.ogg','title':score.spec['title'],'bpm':score.spec['bpm'],'encoder':encoder,
        'key':score.spec['key'],'bars':BARS,'beats_per_bar':4,'sample_rate':RATE,
        'channels':2,'samples':score.samples,'seconds':score.samples/RATE,
        'sections':[{'first_bar':i*8+1,'bars':8,'name':s} for i,s in enumerate(score.spec['sections'])],
        'chords':score.spec['chords'],'event_count':len(score.events),
        'peak_dbfs':round(float(20*np.log10(np.max(np.abs(pcm)))),3),
        'rms_dbfs':round(float(20*np.log10(np.sqrt(np.mean(pcm**2)))),3),
        'seam_step':float(np.max(np.abs(pcm[-1]-pcm[0]))),
        'pcm_s16le_sha256':hashlib.sha256(raw).hexdigest(),
        'sha256':hashlib.sha256(target.read_bytes()).hexdigest(),'bytes':target.stat().st_size,
        'resource':'music.meleemenu' if name=='fleet-menu' else 'music.battle',
        'resource_value':f'MUSICRES:addons/remastered/music/{name}.ogg',
    }
    (output/f'{name}.score.json').write_text(json.dumps({**summary,'events':score.events},indent=2)+'\n')
    print(json.dumps({k:summary[k] for k in ['file','seconds','event_count','peak_dbfs','rms_dbfs','seam_step','bytes']}),flush=True)
    return summary


def main():
    ap=argparse.ArgumentParser(description=__doc__)
    ap.add_argument('--track',choices=list(TRACKS)+['all'],default='all')
    ap.add_argument('--output',type=Path,default=ROOT)
    ap.add_argument('--keep-wav',action='store_true')
    args=ap.parse_args()
    names=list(TRACKS) if args.track=='all' else [args.track]
    tracks=[render(n,args.output,args.keep_wav) for n in names]
    if args.track=='all':
        manifest={'schema_version':1,'collection':'Impulse original Melee score',
          'provenance':'New original compositions and deterministic additive/subtractive-style synthesis. No sampled audio, soundfont, upstream music or third-party melody is used.',
          'render_command':'python3 hd/remastered/music/render.py',
          'dependencies':['Python 3','NumPy','ffmpeg with a Vorbis encoder (libvorbis preferred; native vorbis supported)'],
          'loop_method':'64 whole bars; note releases, stereo echoes and room reflections wrap at exact PCM loop length. Playback is gapless through native looping music resources.',
          'seed':SEED,
          'render_toolchain':{'python':platform.python_version(),'numpy':np.__version__,
             'ffmpeg':subprocess.check_output(['ffmpeg','-version'],text=True).splitlines()[0]},
          'tracks':tracks}
        (args.output/'manifest.json').write_text(json.dumps(manifest,indent=2)+'\n')

if __name__=='__main__': main()
