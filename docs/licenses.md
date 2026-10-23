# Licenses

This repository combines several works with different licenses. The table
below lists them by folder. [`COPYING`](../COPYING) at the root is the text of
the GNU General Public License, version 2.

The game pages of the site link these notices: `COPYING.txt` (UQM code and
content licenses), `content-licenses.txt` (HD content), `melee-credits.txt`
(Melee Remastered art and music) and `THIRD-PARTY-NOTICES.txt` (libraries in
the WebAssembly builds). [`web/editions/THIRD-PARTY-NOTICES.txt`](../web/editions/THIRD-PARTY-NOTICES.txt)
is made by `scripts/make-third-party-notices.py` from the pinned sources; run it
again (`--check` compares) when a library or toolchain version changes.

## Code

| Part | Where | License |
| --- | --- | --- |
| The Ur-Quan Masters 0.8.0 engine (C), based on [intgr/uqm-wasm](https://github.com/intgr/uqm-wasm) | `sc2/src` | GPL-2.0-or-later ([`sc2/COPYING`](../sc2/COPYING)). Some bundled files are GPL-2.0-only (`src/libs/uio`, `list`, `strings`, `cdp`, `md5`), LGPL-2.0-or-later (`src/libs/mikmod`) or LGPL (`rotozoom`). |
| UQM HD MegaMod 0.8.5 engine | not in this repository: the build downloads [JHGuitarFreak/UQM-MegaMod](https://github.com/JHGuitarFreak/UQM-MegaMod) at the pinned commit `1eece12`; the changes are in `hd/patches` | GPL-2.0 (MegaMod `LICENSE`), with the same mixed files as UQM |
| Rust engines (C2Rust translations of the two engines) | `rust/uqm-engine-original`, `rust/uqm-engine-hd` | the licenses of the C files they come from |
| Rust crates of this project | `rust/uqm-core`, `rust/uqm-png`, `rust/uqm-lua`, `rust/uqm-relay` | GPL-2.0-or-later |
| This project's own code: the site picker, page shells, online lobby, data loader, Cloudflare Worker, Node relay, build scripts and tests | `web/`, `sc2/wasm/`, `hd/wasm/`, `hd/patches/`, `server/`, `deploy/`, `scripts/`, `tests/` | GPL-2.0-or-later |
| [Cosmo Clash](../cosmoclash/README.md) by Ori Hanegby, Guy Corem and Orr Kowarsky | `cosmoclash/` | ISC ([`cosmoclash/LICENSE`](../cosmoclash/LICENSE)) |

Because of the GPL-2.0-only files, each compiled engine (C or Rust, Original or
HD) as a whole is distributed under **GPL-2.0-only**; the Rust engine crates say
so in their `Cargo.toml`.

## Content

| Part | Where | License |
| --- | --- | --- |
| UQM content: graphics, sounds, music, voices and dialogue, with the 3DO music and voice packs | `sc2/content` | © Toys for Bob, Inc. or their creators. CC BY-NC-SA 2.5; it may also be copied freely as part of a distribution of The Ur-Quan Masters ([`sc2/COPYING`](../sc2/COPYING)). |
| 3DO videos: intro, ending and 25 ship presentations | `sc2/content/addons/3dovideo` | © 1992 Crystal Dynamics. No license is known. They come from the 3DO Video Pack that UQM MegaMod distributes ([provenance](3do-videos.md)). |
| UQM HD MegaMod content | not in this repository: downloaded at build time ([`hd/content-lock.json`](../hd/content-lock.json)) | CC BY-NC-SA 2.5 for the UQM base content; the HD art has the licenses that its `ATTRIBUTION.txt` lists for each file (some entries say "Custom" or "Unknown"). Every HD page ships these texts as `content-licenses.txt`. |
| Melee Remastered ship, crew and arena art | `hd/remastered/art`, `hd/remastered/render` | AI-assisted redrawings of the MegaMod HD ship and crew art. As derivative works of CC BY-NC-SA 2.5 content they are distributed under CC BY-NC-SA 2.5, with credit to Toys for Bob and the UQM MegaMod artists ([credits](../hd/remastered/credits.md)). |
| Melee Remastered music: *Fleet Mobilization* and *Hostile Vector* | `hd/remastered/music` | Original scores made for this project. CC BY-NC-SA 2.5. |
| Cosmo Clash models, textures, sounds and music | `cosmoclash/public` | © Ori Hanegby, Guy Corem and Orr Kowarsky, from their original game. ISC, the same license as the code ([`cosmoclash/LICENSE`](../cosmoclash/LICENSE)). |
| UQM documentation | `sc2/doc` | CC BY 2.0 ([`sc2/COPYING`](../sc2/COPYING)) |
| This project's documentation | `README.md`, `docs/` | GPL-2.0-or-later, as the code |

## Changes to the upstream code

This project changed files of the UQM engine in `sc2/src` (from intgr/uqm-wasm
commit `daadbb540`) and the UQM MegaMod sources (`hd/patches`) from 18 October
2026. The Git history and the patch files show each change and its date.

## Source of the site builds

The site [urquanmasters.org](https://urquanmasters.org) is built from this
repository by [`.github/workflows/deploy.yml`](../.github/workflows/deploy.yml)
from the branch `deploy/cloudflare`. The HD engine source is UQM MegaMod at the
pinned commit plus `hd/patches`; [`scripts/build-hd-web.sh`](../scripts/build-hd-web.sh)
fetches and patches it. All tool versions, lock files and content hashes are
pinned in the repository.

This project is not affiliated with or endorsed by Toys for Bob or the owner of
the Star Control trademark. Star Control II was made by Toys for Bob.
