#!/usr/bin/env python3
"""Render the original Afterlight score. No samples or external melodies.

Each cue has an authored motif, harmonic sequence, meter and instrumentation.
The four-section arrangements exchange the central 0/2/7/3/10 motif between
characters. Event tails wrap around the loop; exported MIDI contains the score.
Requires numpy and ffmpeg. WAV intermediates go to ignored test-results/.
"""
from pathlib import Path
import argparse, hashlib, json, math, struct, subprocess, wave
import numpy as np

ROOT = Path(__file__).resolve().parents[1]
SR = 32000
# Motif tuples: chromatic offset, duration in quarter notes; None is a rest.
SCORES = [
 dict(id='afterlight', title='What the Light Leaves Behind', bpm=86, meter=4, root=50,
      chords=[0,8,3,10,0,5,8,7], lead='bell', bass='round', pad='air',
      motif=[(12,1),(14,1),(19,2),(15,1.5),(22,.5),(19,1),(None,1)],
      answer=[(22,1),(19,1),(17,1),(15,1),(14,2),(12,1),(None,1)], drums='soft'),
 dict(id='drift', title='A Map Is a Promise', bpm=100, meter=4, root=45,
      chords=[0,5,3,10,0,8,5,7], lead='pluck', bass='round', pad='space',
      motif=[(12,.5),(19,.5),(22,1),(26,2),(24,1),(19,1),(17,1),(None,1)],
      answer=[(24,1.5),(22,.5),(19,1),(17,1),(15,1),(14,1),(12,2)], drums='pulse'),
 dict(id='hearth', title='Mara Keeps the Kettle On', bpm=78, meter=3, root=53,
      chords=[0,5,10,7,0,3,5,0], lead='keys', bass='pluck', pad='air',
      motif=[(12,1),(16,.5),(19,.5),(21,1),(19,1),(14,1),(12,1)],
      answer=[(16,1),(17,1),(21,1),(19,1.5),(16,.5),(12,1)], drums='none'),
 dict(id='mandate', title='A Perfectly Ordered Rescue', bpm=92, meter=4, root=48,
      chords=[0,8,5,7,0,8,3,7], lead='keys', bass='round', pad='organ',
      motif=[(12,1),(19,1),(15,1),(14,1),(12,1),(19,1),(22,1),(19,1)],
      answer=[(24,.5),(22,.5),(19,1),(15,1),(14,1),(12,2),(11,1),(None,1)], drums='clock'),
 dict(id='counterpoint', title='We Are Not Finished', bpm=88, meter=5, root=50,
      chords=[0,5,10,3,8,0,5,7], lead='metal', bass='round', pad='space',
      motif=[(12,1),(19,1),(14,1),(22,1),(15,1),(24,1),(19,1),(17,1),(14,1),(None,1)],
      answer=[(19,1.5),(22,1.5),(26,2),(24,1),(19,1),(17,1),(15,1),(14,1),(12,1)], drums='crystal'),
 dict(id='firstvoice', title='A Name Chosen This Morning', bpm=72, meter=4, root=57,
      chords=[0,5,8,3,0,10,5,0], lead='glass', bass='round', pad='air',
      motif=[(12,1.5),(14,.5),(19,2),(15,1),(22,1),(None,2)],
      answer=[(24,2),(22,1),(19,1),(17,1.5),(14,.5),(12,2)], drums='none'),
 dict(id='smallcourage', title='Form 17: Returning for One Passenger', bpm=112, meter=3.5, root=46,
      chords=[0,5,7,0,8,5,7,0], lead='reed', bass='pluck', pad='organ',
      motif=[(12,.5),(12,.5),(19,.5),(15,.5),(14,.5),(12,.5),(None,.5),
             (10,.5),(12,.5),(14,.5),(19,1),(15,.5),(None,.5)],
      answer=[(24,.5),(22,.5),(19,.5),(17,.5),(15,1),(14,.5),(12,1),(7,.5),(12,1),(None,.5)], drums='comic'),
 dict(id='witness', title='Names, Not a Pardon', bpm=66, meter=4, root=38,
      chords=[0,5,1,7,0,8,5,0], lead='lowstring', bass='round', pad='air',
      motif=[(19,2),(14,1),(15,1),(12,2),(10,1),(None,1)],
      answer=[(24,1),(22,1),(19,2),(17,1),(15,1),(14,2)], drums='distant'),
 dict(id='ceramic', title='The Shape of a Kept Promise', bpm=104, meter=3, root=55,
      chords=[0,5,10,3,0,8,5,7], lead='marimba', bass='pluck', pad='space',
      motif=[(12,.5),(19,.5),(24,1),(22,.5),(19,.5),(17,.5),(14,.5),(12,1),(None,1)],
      answer=[(19,.5),(22,.5),(26,1),(24,1),(22,.5),(19,.5),(17,1),(14,1)], drums='clay'),
 dict(id='secondrain', title='The Roots Refuse the Map', bpm=94, meter=5, root=52,
      chords=[0,5,3,10,0,8,5,0], lead='flute', bass='pluck', pad='air',
      motif=[(12,1),(14,.5),(15,.5),(19,1),(17,2),(15,1),(14,1),(12,2),(None,1)],
      answer=[(19,1.5),(22,.5),(24,2),(22,1),(19,1),(17,1),(15,1),(14,1),(12,1)], drums='wood'),
 dict(id='convoy', title='Wings Over the Refugee Line', bpm=108, meter=4, root=45,
      chords=[0,3,5,7,8,3,5,0], lead='horn', bass='round', pad='organ',
      motif=[(12,1.5),(19,.5),(22,1),(24,1),(19,1),(17,1),(15,2)],
      answer=[(24,1),(26,1),(27,2),(26,1),(22,1),(19,2)], drums='march'),
 dict(id='battle', title='Hold the Corridor', bpm=132, meter=4, root=40,
      chords=[0,0,8,7,0,3,5,7], lead='pulse', bass='pulse', pad='space',
      motif=[(12,.5),(12,.5),(19,.5),(14,.5),(15,1),(22,1),(19,.5),(17,.5),(15,.5),(14,.5),(12,1),(None,1)],
      answer=[(24,.5),(19,.5),(22,.5),(26,.5),(27,1),(26,1),(22,.5),(19,.5),(17,1),(14,1),(12,1)], drums='drive'),
 dict(id='fracture', title='The Missing Voice in the Choir', bpm=76, meter=5, root=47,
      chords=[0,1,5,8,0,6,3,7], lead='glass', bass='round', pad='space',
      motif=[(12,1),(13,1),(19,2),(15,1),(22,1),(18,2),(14,1),(None,1)],
      answer=[(24,2),(23,1),(19,1),(18,1),(15,2),(14,1),(12,2)], drums='distant'),
 dict(id='accord', title='No One Voice Is the Sky', bpm=96, meter=4, root=50,
      chords=[0,5,8,3,10,5,8,0], lead='horn', bass='round', pad='air',
      motif=[(12,1),(14,1),(19,1.5),(15,.5),(22,1),(24,1),(19,2)],
      answer=[(24,1),(26,1),(29,2),(27,1),(26,1),(24,2)], drums='march'),
 dict(id='homecoming', title='There Is Room at the Table', bpm=74, meter=3, root=53,
      chords=[0,5,10,3,8,5,7,0], lead='keys', bass='pluck', pad='air',
      motif=[(12,1),(14,.5),(19,.5),(16,1),(22,1),(24,1),(19,1)],
      answer=[(24,1),(22,1),(19,1),(17,1),(16,1),(12,1)], drums='none'),
]

def instrument(note, seconds, kind, velocity, rng):
    t = np.arange(max(64, int(seconds*SR)), dtype=np.float64) / SR
    f = 440 * 2**((note-69)/12)
    phase = 2*np.pi*f*t
    if kind in ('bell','glass','metal'):
        ratio = {'bell':2.01,'glass':2.76,'metal':3.51}[kind]
        signal = (np.sin(phase + 2.1*np.exp(-t*4)*np.sin(phase*ratio)) * np.exp(-t*2.2)
                  + .19*np.sin(phase*4.01)*np.exp(-t*5))
    elif kind in ('pluck','marimba','keys'):
        decay = {'pluck':4.4,'marimba':6.0,'keys':2.2}[kind]
        signal = sum((1/n**1.6)*np.sin(phase*n)*np.exp(-t*(decay+n*.3)) for n in range(1,7))
        signal += rng.normal(0,.02,len(t))*np.exp(-t*70)
    elif kind in ('reed','horn','pulse','lowstring'):
        harmonics = [1,3,5,7] if kind=='reed' else [1,2,3,4,5]
        signal = sum(np.sin(phase*n + .012*np.sin(t*28))/n**(1.3 if kind=='pulse' else 1.8) for n in harmonics)
        signal *= np.exp(-t*(1.2 if kind=='lowstring' else .45))
    elif kind=='flute':
        signal = np.sin(phase+.025*np.sin(t*30))+.12*np.sin(phase*2)
        signal += rng.normal(0,.018,len(t))
    elif kind in ('air','space','organ'):
        signal = sum(np.sin(phase*(1+d)) for d in [-.0018,0,.0018])/3
        signal += .15*np.sin(phase*2)+.06*np.sin(phase*3)
        if kind=='space': signal *= .8+.2*np.sin(t*1.7)
    else:
        signal = np.sin(phase)+.22*np.sin(phase*2)
    attack = .28 if kind in ('air','space','organ') else (.05 if kind in ('flute','horn') else .006)
    release = min(.35 if kind in ('air','space','organ') else .13, seconds*.3)
    envelope = np.minimum(t/attack,1) * np.minimum(np.maximum(seconds-t,0)/release,1)
    return (signal*envelope*velocity).astype(np.float32)

def vlq(value):
    out = [value & 127]
    while value >> 7:
        value >>= 7; out.insert(0,(value & 127)|128)
    return bytes(out)

def write_midi(path, notes, bpm, meter, lead):
    numerator,denominator=(7,3) if meter==3.5 else (int(meter),2)
    programs={'bell':10,'glass':98,'metal':14,'pluck':24,'keys':0,'marimba':12,
              'reed':68,'horn':60,'lowstring':42,'flute':73,'pulse':80}
    events=[(0, b'\xff\x51\x03'+int(60000000/bpm).to_bytes(3,'big')),
            (0,bytes([0xff,0x58,4,numerator,denominator,24,8])),
            (0,bytes([0xc0,programs.get(lead,0)])),(0,bytes([0xc1,33])),
            (0,bytes([0xc2,89])),(0,bytes([0xc3,10]))]
    for beat,note,length,velocity,channel in notes:
        events.extend([(round(beat*480),bytes([0x90|channel,note,min(127,round(velocity*250))])),
                       (round((beat+length)*480),bytes([0x80|channel,note,0]))])
    events.sort(key=lambda e:(e[0],e[1][0]>>4 != 8))
    data=b''; prev=0
    for tick,event in events: data+=vlq(tick-prev)+event; prev=tick
    data+=b'\x00\xff\x2f\x00'
    path.write_bytes(b'MThd'+struct.pack('>IHHH',6,0,1,480)+b'MTrk'+struct.pack('>I',len(data))+data)

def compose(score, output, masters):
    rng=np.random.default_rng(int.from_bytes(hashlib.sha256(score['id'].encode()).digest()[:4],'big'))
    beat=60/score['bpm']; meter=score['meter']; bars=32; seconds=beat*meter*bars
    count=round(seconds*SR); mix=np.zeros((count,2),dtype=np.float32); midi=[]
    def add(at,note,length,kind,level,pan=0,channel=0):
        samples=instrument(note,length*beat,kind,level,rng)
        start=round(at*beat*SR)%count
        stereo=samples[:,None]*np.array([math.sqrt((1-pan)/2),math.sqrt((1+pan)/2)],dtype=np.float32)
        # Include the preceding loop's sustained tails for seamless playback.
        offset=0
        while offset<len(samples):
            take=min(len(samples)-offset,count-start)
            mix[start:start+take]+=stereo[offset:offset+take]; offset+=take; start=0
        midi.append((at,note,length,level,channel))
    for bar in range(bars):
        section=bar//8; chord=score['chords'][bar%8]; root=score['root']+chord
        at=bar*meter; major=score['id'] in ('hearth','homecoming') and chord in (0,5,10)
        third=4 if major else 3
        # Orchestration grows in A/B/A', then leaves a bridge to the loop opening.
        pad_level=[.035,.045,.05,.032][section]
        for j,interval in enumerate([0,7,12,12+third]):
            add(at,root+interval,meter+.6,score['pad'],pad_level,(-.55+j*.35),2)
        basspattern=[(0,0,1.5),(meter/2,7,max(.5,meter/2-.2))]
        if score['drums']=='drive': basspattern=[(i*.5,0 if i%4<2 else 7,.42) for i in range(int(meter*2))]
        for pos,interval,length in basspattern:
            add(at+pos,root-12+interval,length,score['bass'],.13,-.08,1)
        if section in (1,2) or (section==0 and bar>=4):
            pattern=score['answer'] if section==1 else score['motif']
            # A melody phrase spans two bars; each melody itself is authored.
            if bar%2==0:
                pos=0
                for note,length in pattern:
                    if note is not None:
                        add(at+pos,score['root']+note,length*.94,score['lead'],.17,.18,0)
                    pos+=length
        if section==3 and bar%4==0:
            for pos,note in [(0,12),(1,14),(2,19)]:
                add(at+pos,score['root']+note,1.8,score['lead'],.10,-.15,0)
        if section==2 and score['id'] in ('afterlight','accord','counterpoint','convoy'):
            for p,n in [(0,7),(1.5,10),(3,3)]:
                if p<meter: add(at+p,root+n,1.2,'bell',.055,-.6,3)
        if score['drums']!='none':
            active=section in (1,2) or bar%2==0
            if active:
                positions=np.arange(0,meter,.5 if score['drums'] in ('drive','pulse','clock') else 1)
                for p in positions:
                    n=int(.12*SR); t=np.arange(n)/SR
                    hit=(rng.normal(0,1,n)*np.exp(-t*48)).astype(np.float32)
                    if p%2==0:
                        hit += (2*np.sin(2*np.pi*(45*t+1.8*(1-np.exp(-t*32))))*np.exp(-t*30)).astype(np.float32)
                    # Every hit starts at zero; the loop must not begin with a
                    # random noise discontinuity that produces an audible click.
                    hit *= np.minimum(t/.003,1)
                    strength=.026 if score['drums'] in ('soft','distant','crystal') else .052
                    index=round((at+p)*beat*SR); take=min(n,count-index)
                    mix[index:index+take]+=hit[:take,None]*strength
    # A restrained stereo room, periodic so the music remains loopable.
    dry=mix.copy()
    for delay,gain in [(.083,.12),(.157,.11),(.263,.085),(.397,.055),(.563,.035)]:
        mix+=np.roll(dry,round(delay*SR),axis=0)[:,::-1]*gain
    mix-=mix.mean(axis=0)
    peak=float(np.max(np.abs(mix))); mix*=.79/max(peak,1e-8)
    assert np.isfinite(mix).all()
    pcm=(mix*32767).astype('<i2')
    wav=masters/(score['id']+'.wav')
    with wave.open(str(wav),'wb') as file:
        file.setnchannels(2);file.setsampwidth(2);file.setframerate(SR);file.writeframes(pcm.tobytes())
    ogg=output/(score['id']+'.ogg')
    subprocess.run(['ffmpeg','-y','-v','error','-i',str(wav),'-c:a','vorbis','-strict','-2','-b:a','160k',
                    '-metadata','title='+score['title'],'-metadata','artist=The Afterlight Accord original score',str(ogg)],check=True)
    write_midi(output/(score['id']+'.mid'),midi,score['bpm'],meter,score['lead'])
    return dict(id=score['id'],title=score['title'],file='assets/music/'+score['id']+'.ogg',
                duration=round(seconds,3),bpm=score['bpm'],quarterNotesPerBar=meter,bars=bars,
                peak=float(np.max(np.abs(mix))),rms=float(np.sqrt(np.mean(mix**2))),
                loopBoundaryJump=float(np.max(np.abs(mix[-1]-mix[0]))),
                sha256=hashlib.sha256(ogg.read_bytes()).hexdigest())

if __name__=='__main__':
    parser=argparse.ArgumentParser();parser.add_argument('--only',nargs='*');args=parser.parse_args()
    output=ROOT/'assets/music';masters=ROOT/'test-results/music-masters'
    output.mkdir(parents=True,exist_ok=True);masters.mkdir(parents=True,exist_ok=True)
    tracks=[]
    for score in SCORES:
        if args.only and score['id'] not in args.only:continue
        info=compose(score,output,masters);tracks.append(info);print(json.dumps(info),flush=True)
    if not args.only:
        (output/'manifest.json').write_text(json.dumps(dict(composer='Original score composed for The Afterlight Accord',
          source='campaign/tools/compose.py',sampleRate=SR,channels=2,tracks=tracks),indent=2)+'\n')
        (output/'scores.json').write_text(json.dumps(SCORES,indent=2)+'\n')
