#!/usr/bin/env bash
# Make a downloadable game with the exact corresponding repository source.
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$root"
if [[ -n "$(git status --porcelain)" ]]; then
  echo 'Commit the source first. A release must identify an exact clean revision.' >&2
  exit 1
fi
revision="$(git rev-parse HEAD)"
short_revision="$(git rev-parse --short=12 HEAD)"
bash scripts/build-afterlight.sh
if [[ -n "$(git status --porcelain)" ]]; then
  echo 'The build changed tracked source. Review and commit it before packaging.' >&2
  exit 1
fi
mkdir -p .tools/releases
stage="$(mktemp -d "$root/.tools/afterlight-release.XXXXXX")"
trap 'rm -rf "$stage"' EXIT
cp -R .tools/afterlight-dist/. "$stage/"
mkdir -p "$stage/source"
source_name="afterlight-$short_revision-source.tar.gz"
git archive --format=tar.gz --prefix="uqm-3do-web-$short_revision/" --output="$stage/source/$source_name" "$revision"
python3 - "$stage" "$revision" "$source_name" <<'PY'
from pathlib import Path
import hashlib,json,sys
stage=Path(sys.argv[1]);revision=sys.argv[2];source=sys.argv[3]
(stage/'SOURCE.html').write_text(f'''<!doctype html><html lang="en"><meta charset="utf-8"><title>Afterlight corresponding source</title><body><h1>The Afterlight Accord — source</h1><p>This download includes the exact repository source, Rust engine, native hooks, build scripts, campaign content and editable score sources used for revision <code>{revision}</code>.</p><p><a href="source/{source}">Download the matching source archive</a></p><p>Toolchain versions and reproducible dependency setup are documented in the archive's README.md and campaign/README.md. Existing third-party notices remain applicable.</p><p><a href="https://github.com/vcorem/uqm-3do-web/tree/{revision}">Repository revision</a> · <a href="CREDITS.html">Credits</a> · <a href="./">Play</a></p></body></html>''')
(stage/'START-HERE.txt').write_text('THE AFTERLIGHT ACCORD\n\nServe this directory with the UQM Rust server described in the source archive.\nUse HTTPS outside localhost and these response headers:\nCross-Origin-Opener-Policy: same-origin\nCross-Origin-Embedder-Policy: require-corp\n\nDo not open index.html with file://. Open SOURCE.html for the matching source.\nThe campaign is single-player; no remote account or AI service is required.\n')
files={str(p.relative_to(stage)):hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted(stage.rglob('*')) if p.is_file() and p.name!='build-manifest.json'}
(stage/'build-manifest.json').write_text(json.dumps({'campaign':'afterlight-accord','revision':revision,'sourceArchive':'source/'+source,'files':files},indent=2)+'\n')
PY
archive="$root/.tools/releases/afterlight-$short_revision-web.zip"
(cd "$stage" && zip -q -r "$archive" .)
cp "$stage/source/$source_name" "$root/.tools/releases/$source_name"
python3 - "$archive" <<'PY'
from pathlib import Path
import hashlib,sys
p=Path(sys.argv[1]);(p.with_suffix(p.suffix+'.sha256')).write_text(hashlib.sha256(p.read_bytes()).hexdigest()+'  '+p.name+'\n')
print(f'Downloadable game and matching source: {p}')
PY
