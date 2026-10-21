#!/usr/bin/env python3
"""Decode the committed score files and check their actual media properties."""
import json, subprocess
from pathlib import Path
root=Path(__file__).resolve().parents[1]
manifest=json.loads((root/'assets/music/manifest.json').read_text())
for track in manifest['tracks']:
    file=root/track['file']
    probe=json.loads(subprocess.check_output(['ffprobe','-v','error','-show_streams','-show_format','-of','json',str(file)]))
    stream=probe['streams'][0]
    assert stream['codec_name']=='vorbis' and stream['channels']==2,track['id']
    assert abs(float(probe['format']['duration'])-track['duration'])<.1,track['id']
    subprocess.run(['ffmpeg','-v','error','-i',str(file),'-f','null','-'],check=True)
    print(track['id'],stream['codec_name'],stream['channels'],probe['format']['duration'])
print('All original score files decode successfully.')
