#!/usr/bin/env python3
"""Check decoded shipment audio, its authored score and objective mix limits."""
from __future__ import annotations
import argparse
import hashlib
import json
from pathlib import Path
import re
import subprocess
import tempfile
import numpy as np
ROOT=Path(__file__).resolve().parent
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
def db(x):return float(20*np.log10(max(float(x),1e-12)))

def inspect(folder):
    manifest=json.loads((folder/'manifest.json').read_text())
    assert manifest['version']=='martial-space-2'
    source=folder/'render.py' if (folder/'render.py').exists() else ROOT/'render.py'
    assert sha(source)==manifest['source_sha256'],'Renderer differs from manifest'
    report={'passed':False,'version':manifest['version'],'subjective_audition':False,
        'listening_limit':'No model audio input was available. This report establishes measured signal properties, not musical taste or in-game masking.','tracks':[]}
    for track in manifest['tracks']:
        path=folder/track['file'];assert sha(path)==track['sha256'],f'{path.name}: changed artifact'
        probe=json.loads(subprocess.check_output(['ffprobe','-v','error','-show_streams','-of','json',str(path)]))['streams'][0]
        assert probe['codec_name']=='vorbis' and probe['channels']==2 and int(probe['sample_rate'])==44100
        assert 90<=float(probe['duration'])<=120
        with tempfile.TemporaryDirectory(prefix='uqm-audio-check-') as temp:
            raw=Path(temp)/'decoded.f32'
            subprocess.run(['ffmpeg','-hide_banner','-loglevel','error','-i',str(path),'-f','f32le','-acodec','pcm_f32le',str(raw)],check=True)
            audio=np.fromfile(raw,dtype='<f4').reshape(-1,2)
        assert len(audio)==track['samples'],f'{path.name}: granule count/decoder duration changed'
        assert np.isfinite(audio).all()
        peak=float(np.max(np.abs(audio)));assert peak<.8913,f'{path.name}: less than1dB sample headroom'
        dc=float(np.max(np.abs(audio.mean(axis=0))));assert dc<.002
        win=4410;windows=audio[:len(audio)//win*win].reshape(-1,win,2)
        rms=np.sqrt(np.mean(windows**2,axis=(1,2)));assert rms.min()>.002,f'{path.name}: unintended silent gap'
        seam=float(np.max(np.abs(audio[0]-audio[-1])))
        ordinary=float(np.quantile(np.abs(np.diff(audio,axis=0)),.999))
        assert seam<max(.025,ordinary),f'{path.name}: boundary discontinuity'
        # Compare first/last50ms energy as well as the single wrap sample, to
        # catch inserted encoder padding that a favorable endpoint could hide.
        assert np.sqrt(np.mean(audio[:2205]**2))>.002 and np.sqrt(np.mean(audio[-2205:]**2))>.002
        mid=(audio[:,0]+audio[:,1])*.5;side=(audio[:,0]-audio[:,1])*.5
        width=float(np.sqrt(np.mean(side**2))/np.sqrt(np.mean(mid**2)));assert .03<width<1
        score=json.loads(path.with_suffix('.score.json').read_text());events=score['events'];kinds={e['voice'] for e in events}
        assert {'chug','reese','alarm','brass','space','riser','kick','snare','tom-low','tom-mid','tom-high','hat'}<=kinds
        assert not {'pluck','pad','lead'}&kinds,'Legacy soft arrangement retained'
        assert len(score['harmonic_plan'])==64 and len(events)>2500
        assert score['bpm']==(168 if path.stem=='battle' else 150)
        # Distinct rhythm signatures measure score variation without asserting
        # that variation alone establishes musical interest.
        signatures=[]
        for bar in range(64):
            signatures.append(tuple((e['voice'],round(e['beat']-bar*4,3),e['midi']) for e in events if bar*4<=e['beat']<(bar+1)*4 and e['voice'] in ['chug','reese','kick','snare']))
        unique_patterns=len(set(signatures));assert unique_patterns>=16
        sections=[float(np.sqrt(np.mean(audio[round(i*len(audio)/8):round((i+1)*len(audio)/8)]**2))) for i in range(8)]
        section_range=db(max(sections)/min(sections));assert section_range>3
        # Whole-track spectral power in broad bands (not a judgment of timbre).
        size=8192;frames=mid[:len(mid)//size*size].reshape(-1,size)
        spectrum=np.mean(np.abs(np.fft.rfft(frames*np.hanning(size),axis=1))**2,axis=0)
        f=np.fft.rfftfreq(size,1/44100);total=float(spectrum.sum())
        bands={f'{low}-{high}Hz':round(float(spectrum[(f>=low)&(f<high)].sum()/total),5) for low,high in [(0,30),(30,80),(80,250),(250,2000),(2000,8000),(8000,18000),(18000,22050)]}
        assert bands['0-30Hz']<.08 and bands['18000-22050Hz']<.015,'Excess infrasonic or near-Nyquist energy'
        meter=subprocess.run(['ffmpeg','-hide_banner','-nostats','-i',str(path),'-af','ebur128=peak=true','-f','null','-'],capture_output=True,text=True,check=True).stderr.split('Summary:')[-1]
        integrated=float(re.search(r'I:\s+(-?[\d.]+) LUFS',meter).group(1))
        lra=float(re.search(r'LRA:\s+([\d.]+) LU',meter).group(1))
        truepeak=float(re.search(r'Peak:\s+(-?[\d.]+) dBFS',meter).group(1))
        assert truepeak<=-1.2 and -22<integrated<=-13.5
        report['tracks'].append({'file':path.name,'sha256':sha(path),'decoded_samples':len(audio),'seconds':len(audio)/44100,'tempo_bpm':score['bpm'],'sample_peak_dbfs':round(db(peak),3),'true_peak_dbtp':truepeak,'integrated_lufs':integrated,'loudness_range_lu':lra,'average_rms_dbfs':round(db(np.sqrt(np.mean(audio**2))),3),'crest_db':round(db(peak/np.sqrt(np.mean(audio**2))),3),'dc_offset':dc,'quietest_100ms_dbfs':round(db(rms.min()),3),'loop_seam_step':seam,'normal_99_9_percent_step':ordinary,'stereo_side_to_mid_ratio':round(width,3),'section_rms_dbfs':[round(db(x),3) for x in sections],'section_range_db':round(section_range,3),'unique_bar_rhythm_signatures':unique_patterns,'event_count':len(events),'spectral_power_fractions':bands})
    report['passed']=True
    return report

def main():
    ap=argparse.ArgumentParser(description=__doc__);ap.add_argument('--input',type=Path,default=ROOT);ap.add_argument('--report',type=Path);args=ap.parse_args()
    result=inspect(args.input);text=json.dumps(result,indent=2)+'\n'
    if args.report:args.report.write_text(text)
    print(text)
if __name__=='__main__':main()
