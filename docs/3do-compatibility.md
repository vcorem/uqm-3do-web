# 3DO compatibility target

The reference is **Star Control II for the 3DO**. This project ports the existing
UQM engine to WebAssembly; it does not replace the simulation, ship code or
original artwork. A successful browser build is not evidence of complete 3DO
fidelity. Original-hardware or suitable reference-capture comparisons remain
required before making that claim.

## Bundled content

The upstream checkout already includes expanded UQM 0.8.0 base content under
`sc2/content/base`, original 3DO music under `sc2/content/addons/3domusic`, and
3DO dialogue recordings under `sc2/content/addons/3dovoice`. These are tracked
assets, so the first build does not need separate content downloads. The
WebAssembly build currently preloads the entire `sc2/content` directory.

Official package alternatives are listed on the [UQM download page](https://sc2.sourceforge.net/downloads.php):
base content is approximately 11.5 MB, music 19.0 MB, and voice 115.2 MB. Base
packages belong in `content/packages`; music and voice packages belong in
`content/addons`. Avoid bundling archives alongside the same expanded assets.

Code and content have separate licenses; preserve the attribution and terms in
[`sc2/COPYING`](../sc2/COPYING).

## Initial settings

Use this supported command-line baseline for the browser version:

```text
--menu=3do --font=3do --cscan=3do --scroll=3do --shield=3do --meleezoom=3do --subtitles --intro=pc
```

This selects graphical menus, 3DO fonts and scan icons, smooth dialogue seeking,
pulsing shields and smooth combat zoom. Dialogue subtitles are on by default. The added `--subtitles`
startup option explicitly enables them even if an older saved preference has
them disabled. This differs from the upstream
[Mimic 3DO preset](../sc2/build/win32_install/uqm-3do.cfg). Music and speech should be enabled and the later Precursors remix packs
disabled. Current engine defaults already select those audio settings.

`--intro=pc` is the fallback for a missing or incomplete original video
installation, not a 3DO-equivalent presentation. `--intro=3do` is supported when those
assets are available: the launcher detects a complete 134-file set at startup
and selects this option automatically. See [installation and source status](3do-videos.md).
The older manual mentions `--music`/`-m`, but the current
[argument parser](../sc2/src/uqm.c) does not support that option; use the existing
music settings instead.

## Known gaps and differences

- **Original videos are included in the repository content.** `content/addons/3dovideo`
  includes the intro, ending and all 25 ship presentations. Their source and
  package attribution are recorded in [the media provenance file](3do-video-source.json).
  The [mapping](../sc2/content/addons/3dovideo/3dovideo.rmp) describes the expected
  paths. Current code includes ship-video playback support in
  [`fmv.c`](../sc2/src/uqm/fmv.c) and
  [`supermelee/buildpick.c`](../sc2/src/uqm/supermelee/buildpick.c); playback validation is recorded in [the test record](validation.md). The
  videos have no known license; see [Licenses](licenses.md).
- **UQM includes visual departures.** Its own
  [graphics comparison](../sc2/doc/devel/gfxversions) documents a larger Sa-Matra
  captain portrait and colored interplanetary suns, whereas 3DO suns were yellow.
- **Other historical differences need confirmation.** The project wiki reports
  removal of the 3DO television borders and a bug fix allowing Slylandro probes
  inside star systems. Its [version comparison](https://wiki.uqm.stack.nl/Version_Comparison)
  was last edited in 2011, so treat it as an audit checklist, not current
  implementation evidence. Its statement that ship videos are unsupported is
  already contradicted by the source in this checkout.

## Online Melee extension

Online rooms are an optional addition to the original game. The port reuses
UQM's ship simulation, random-seed exchange, input-delay protocol and combat
checksums; it replaces native sockets with the browser relay transport. The
3DO presentation settings still apply. Successful network checksums establish
agreement between the two UQM instances, not independent parity with 3DO
hardware. See [online validation](validation.md#online-melee-private-alpha--2026-10-18).

## Required parity validation

1. Record reference hardware/emulator, game revision, control mappings, audio
   settings and screen framing; compare them with a pinned browser build.
2. Compare every ship's movement, turning, weapons, special ability, energy,
   collisions, gravity effects, wraparound, AI and combat timing. Preserve the
   existing simulation while fixing browser platform behavior.
3. Compare menu and battle screenshots, zoom transitions, fonts, scan icons,
   shields and campaign screens; track inherited UQM differences separately.
4. Check 3DO music selection and looping, sound effects, speech seeking, mixing
   and synchronization, including browser audio startup and suspend/resume.
5. Exercise keyboard and controller input, two local players, campaign progress,
   saving and reloads across supported browsers. Verify frame pacing and input
   behavior under load, including after a tab returns from the background.

Track each check as untested, passed or differing with reproducible evidence.
A build that launches or finishes one Melee battle has only passed a smoke test.
