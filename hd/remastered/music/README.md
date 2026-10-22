# Original Melee score

Two newly composed, seamlessly looping electronic tracks for the Remastered
presentation. The fleet theme uses a clear melodic refrain and open chord
voicings; battle adds a faster drum pattern, picked power chords, and an urgent
minor-key answer phrase. Each has eight arranged sections, including a quieter
break, a rebuild, and a return to the opening pulse.

| File | Title | Tempo | Length |
| --- | --- | --- | --- |
| `fleet-menu.ogg` | Constellation Assembly | 144 BPM | 106.667 seconds |
| `battle.ogg` | Redshift Intercept | 160 BPM | 96 seconds |

The compositions, event scores, synthesizer voices, percussion, and room effects
are original to this project. No upstream soundtrack, sampled instrument,
soundfont, external recording, or third-party melody is used. `manifest.json`
records hashes and `*.score.json` records every note and drum event. This is a
new soundtrack, not a remix of the original game's music.

## Rebuild

Install Python 3, NumPy, and ffmpeg with a Vorbis encoder, then from the repository
root run:

```sh
python3 hd/remastered/music/render.py
python3 hd/remastered/music/verify.py
```

`--output PATH` permits an isolated rebuild. `--track fleet-menu` or `--track
battle` renders one track; the complete collection manifest is written when
both are rendered. `--keep-wav` retains lossless working files for listening or
further mastering; these files are not required for shipping.

The renderer prefers `libvorbis` if available and otherwise uses ffmpeg's native
Vorbis encoder. The synthesizer is deterministic; encoded bytes depend on the
ffmpeg encoder and version. To reproduce the shipped byte hashes exactly, use
the encoder recorded in the manifest and the recorded toolchain versions.
The uncompressed signed 16-bit PCM hash is also recorded for independent
comparisons. The synth uses only authored note sequences and a fixed noise seed.

## Engine integration

Stage the Ogg files below `content/addons/remastered/music/`, put these resource
entries in the presentation addon's RMP, and load `--addon=remastered`:

```text
music.meleemenu = MUSICRES:addons/remastered/music/fleet-menu.ogg
music.battle = MUSICRES:addons/remastered/music/battle.ogg
```

Native playback retains the game's existing volume, pause, battle, and victory
transition behavior. Do not start a second browser music player over it. Notes,
releases, echoes, and reflections wrap within exactly 64 bars. There is no
start/end fade or inserted silence. Vorbis granule positions preserve the exact
PCM sample length.

## Verification and listening

`verify.py` checks the actual decoded shipment files: Vorbis format, exact sample
length, finite samples, clipping, DC offset, silent gaps, stereo field, section
energy variation, and loop continuity against normal waveform changes.
`validation.json` records the checked output. These checks cannot establish
whether a person likes the arrangement. This environment did not expose a
model audio input; the tracks have not been subjectively listened to by the
agent. Audition both at normal game volume and verify the final native loop and
victory transition in the browser before a public release.
