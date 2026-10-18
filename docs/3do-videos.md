# Original 3DO movies

The engine already includes the original DUK video and audio decoders, plus
AIFF audio support. No transcoding or replacement player is required.
The browser launcher now detects a complete movie set after content loading
and before engine startup. It selects `--intro=3do` only when every required
file is present and nonempty. Otherwise it keeps the PC intro and ending;
an incomplete installation is listed under Diagnostics.

## Source status

The repository content includes the original intro, ending and 25 ship
presentations from the 3DO Videos package listed on the
[MegaMod download page](https://uqm-mods.sourceforge.net/Download), downloaded
on 2026-10-18 at the user's request for local play. The archive is retained in
`.tools/media/uqm-0.8.0-3dovideo.uqm`, with tracked source details in [3do-video-source.json](3do-video-source.json).
The package metadata attributes these video resources to Crystal Dynamics.

- Download size: 135,563,859 bytes.
- SHA-256: `9c190d1fae862da741e8da9c0a7cba34060a9e175a87623391ba2296e417ecce`.
- Installed: 134 required media files, 151,047,342 bytes.
- Archive `victory/` and `ships/` paths are mapped to the existing engine's
  `ending/` and `spins/` paths. The original engine resource map is preserved.
- Extra MegaMod logo/drumall presentations are retained in the download archive
  but are not used by this UQM build.

The 134 installed media files are included in Git. The download cache and
generated builds remain ignored. The package contains no separate license
file; its original attribution is retained in the provenance record.

## Media layout

The existing engine resource map references these lowercase filenames under
`sc2/content/addons/3dovideo/`:

- `intro/intro.duk`, `.frm`, `.hdr`, `.tbl`
- `ending/victory.duk`, `.frm`, `.hdr`, `.tbl`
- `spins/ship00` through `spins/ship24`, each with `.duk`, `.frm`, `.hdr`, `.tbl`, `.aif`
- `spins/spin.aif`

There are 134 media files. Rebuild with `./scripts/build-web.sh`, restart the
preview server and reload the page. The launcher reports when all videos are
enabled. Select New Game for the intro; press Alt in a ship selection menu for
a ship video. In Super Melee, press Esc to edit the fleet, Enter to open
the ship picker, then Alt / Option to play its video. Enter or Esc dismisses playback. The ending is used at victory.

The build preloads these media into `dist/` together with the campaign content.

## Validation limits

Automated tests check the complete resource list, absent/partial installations,
and empty files/directories. Detection establishes file presence only, not
codec validity or authenticity. The local browser loads the full pack and plays the original opening movie and the Androsynth ship movie.
Audible synchronization and complete ending playback remain unverified; see
[the validation record](validation.md).
