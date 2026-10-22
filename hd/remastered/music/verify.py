#!/usr/bin/env python3
"""Validate the shipped Vorbis files after decoding, including the loop seam."""
from __future__ import annotations
import argparse
import hashlib
import json
from pathlib import Path
import subprocess
import tempfile
import numpy as np

ROOT=Path(__file__).resolve().parent

def inspect(folder):
    manifest=json.loads((folder/'manifest.json').read_text())
    report={'passed':False,'tracks':[]}
    for track in manifest['tracks']:
        path=folder/track['file']
        assert hashlib.sha256(path.read_bytes()).hexdigest()==track['sha256'], f'{path.name}: changed artifact'
        probe=json.loads(subprocess.check_output(['ffprobe','-v','error','-show_streams','-of','json',str(path)]))['streams'][0]
        assert probe['codec_name']=='vorbis', f'{path.name}: the engine requires Vorbis, not Opus'
        assert probe['channels']==2 and int(probe['sample_rate'])==44100
        assert 90<=float(probe['duration'])<=120
        with tempfile.TemporaryDirectory(prefix='uqm-audio-verify-') as temp:
            raw=Path(temp)/'decoded.f32'
            subprocess.run(['ffmpeg','-hide_banner','-loglevel','error','-i',str(path),
                            '-f','f32le','-acodec','pcm_f32le',str(raw)],check=True)
            audio=np.fromfile(raw,dtype='<f4').reshape(-1,2)
        # Ogg granule positions encode the exact loop length. An encoder delay or
        # discarded final packet is audible in repeated playback and must fail.
        assert len(audio)==track['samples'], f'{path.name}: decoded sample count changed {len(audio)} != {track["samples"]}'
        assert np.isfinite(audio).all()
        peak=float(np.max(np.abs(audio)))
        assert peak<1., f'{path.name}: encoded overshoot clips'
        dc=float(np.max(np.abs(audio.mean(axis=0))))
        assert dc<.002, f'{path.name}: DC offset'
        win=4410
        windows=audio[:len(audio)//win*win].reshape(-1,win,2)
        rms=np.sqrt(np.mean(windows**2,axis=(1,2)))
        assert float(rms.min())>.008, f'{path.name}: silence or near-silence in loop'
        # Compare the wrap step with ordinary waveform increments. This catches
        # a discontinuity rather than rejecting valid high-frequency percussion.
        seam=float(np.max(np.abs(audio[0]-audio[-1])))
        derivative=np.abs(np.diff(audio,axis=0))
        reference=float(np.quantile(derivative,.999))
        assert seam<max(.025,reference), f'{path.name}: audible boundary discontinuity'
        assert float(np.max(np.abs(audio[-4410:])))>.02 and float(np.max(np.abs(audio[:4410])))>.02
        mid=(audio[:,0]+audio[:,1])*.5
        side=(audio[:,0]-audio[:,1])*.5
        width=float(np.sqrt(np.mean(side**2))/np.sqrt(np.mean(mid**2)))
        assert .03<width<.9, f'{path.name}: invalid stereo field'
        score=json.loads((folder/path.with_suffix('.score.json').name).read_text())
        kinds={x[0] for x in score['events']}
        assert {'lead','bass','pad','pluck','kick','snare','hat'}<=kinds
        assert len(score['chords'])==64 and score['event_count']>2000
        # A real changing arrangement should vary measured energy across sections.
        bar_samples=len(audio)/64
        section_rms=[float(np.sqrt(np.mean(audio[round(i*8*bar_samples):round((i+1)*8*bar_samples)]**2))) for i in range(8)]
        assert max(section_rms)/min(section_rms)>1.15, f'{path.name}: missing section dynamics'
        report['tracks'].append({'file':path.name,'decoded_samples':len(audio),
           'seconds':len(audio)/44100,'peak_dbfs':round(20*np.log10(peak),3),
           'dc_offset':dc,'quietest_100ms_dbfs':round(float(20*np.log10(rms.min())),3),
           'loop_seam_step':seam,'normal_99_9_percent_step':reference,
           'stereo_side_to_mid_ratio':round(width,3),'section_rms':section_rms})
    report['passed']=True
    return report

def main():
    ap=argparse.ArgumentParser(description=__doc__)
    ap.add_argument('--input',type=Path,default=ROOT)
    ap.add_argument('--report',type=Path)
    args=ap.parse_args(); report=inspect(args.input)
    text=json.dumps(report,indent=2)+'\n'
    if args.report: args.report.write_text(text)
    print(text)

if __name__=='__main__': main()
