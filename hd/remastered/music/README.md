# Original Melee combat score

This second arrangement replaces the earlier light melodic soundtrack with an
industrial space-combat score. Both pieces use low, syncopated distorted riffs,
Phrygian semitone tension, a metallic alarm motif, punchy layered drums and
short, deliberate breaks before impact returns. There are no soft pluck
arpeggios or open major-chord beds.

| File | Title | Tempo | Length |
| --- | --- | --- | --- |
| `fleet-menu.ogg` | Fleet Mobilization | 150 BPM | 102.400 seconds |
| `battle.ogg` | Hostile Vector | 168 BPM | 91.429 seconds |

The fleet piece uses marching low toms and a repeated launch ostinato. Battle
uses denser syncopated kicks, ghost snares, double-track chugs, a low distorted
bass, angular call-and-response phrases and a full-assault return after its
quieter gravity-trap and target-acquisition sections. Each piece has eight
arranged eight-bar sections, with fills and small rhythmic breaks inside them.

The timbres are synthesized for this project: two-stage saturated string
excitations with cabinet filtering, detuned bass harmonics, metallic phase
modulation, pitched drum bodies, filtered noise attacks and short stereo room
reflections. The nonlinear pitched voices render at twice the delivery sample
rate before filtering/downsampling. Kick-triggered attenuation makes space in
the tonal layers without compressing away the drum attacks.

No upstream soundtrack, external recording, sampled instrument, soundfont or
third-party melody is used. `*.score.json` records every note and drum event,
including articulation, pan and variation. `manifest.json` records the source,
PCM and Ogg hashes, arrangement sections, and measured levels.

## Rebuild

Dependencies: Python 3, NumPy, and ffmpeg with a Vorbis encoder.

```sh
python3 hd/remastered/music/render.py --keep-wav
python3 hd/remastered/music/verify.py --report hd/remastered/music/validation.json
```

`--output PATH` makes an isolated rebuild; `--track fleet-menu` or `--track
battle` renders one piece. The complete collection manifest is written when
both are rendered. `--keep-wav` retains lossless working files; native delivery
uses only the Ogg files. The renderer prefers libvorbis when available and
otherwise uses ffmpeg's native Vorbis encoder. Exact encoded bytes depend on
the recorded toolchain; signed 16-bit PCM hashes permit a separate synthesis
comparison. Noise variation is seeded deterministically per instrument.

## Native integration and looping

Stage the Ogg files under `content/addons/remastered/music/` and load the
presentation addon with these RMP entries:

```text
music.meleemenu = MUSICRES:addons/remastered/music/fleet-menu.ogg
music.battle = MUSICRES:addons/remastered/music/battle.ogg
```

The native player controls looping, volume, pause, battle and victory
transitions. There is no second browser music player. Every note tail and
reflection wraps modulo the exact 64-bar sample count. No track-boundary fade
or silent padding conceals a seam. The selected tempi produce exact loop
lengths that also survive the native Vorbis encoder: 4,515,840 samples for the
fleet menu and 4,032,000 for battle, at 44,100 Hz stereo.

The final master uses global gain with a peak ceiling, rather than a loudness
limiter that flattens each hit. Battle targets roughly -14 LUFS and the menu
roughly -15.5 LUFS. Decoded true-peak headroom and actual measured levels are in
`validation.json`; game music volume still controls the balance with effects.

## Validation and audition

The validator decodes the actual shipped files and checks exact sample count,
finite samples, DC, sample and reconstructed true peaks, loop continuity,
unintended silent gaps, stereo width, section dynamics, score variation and
broad spectral balance. Its score checks establish that the new riff/drum
arrangement replaced the previous soft voices; they cannot establish musical
quality. Lossless reconstruction checks can compare the recorded PCM hashes.

No model audio-input tool was available, so the agent did not subjectively
listen to these tracks. Short MP3 build-and-return excerpts are prepared under
ignored `.tools/music-auditions/` for human audition. Those previews have small
edge fades for standalone playback; the shipped Ogg loops do not. Native
in-game playback and subjective effect masking are separate checks, not
claims made by the signal validator.
