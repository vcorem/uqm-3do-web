#!/usr/bin/env python3
"""Create a clean, manifest-checked portable campaign, replacing only owned builds."""
import hashlib, json, os, shutil, tempfile
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2]
SOURCE=ROOT/'campaign'
DEST=Path(os.environ.get('AFTERLIGHT_OUTPUT',str(ROOT/'.tools/afterlight-dist'))).resolve()
COMBAT=Path(os.environ.get('AFTERLIGHT_COMBAT_DIST',str(ROOT/'.tools/afterlight-combat'))).resolve()
for protected in (ROOT,SOURCE,COMBAT):
    if DEST==protected or DEST in protected.parents or protected in DEST.parents and protected!=ROOT:
        raise SystemExit('Build output must be separate from source and combat input.')
if not (COMBAT/'index.html').is_file():
    raise SystemExit('Build the native combat package first: bash scripts/build-afterlight-combat.sh')
if DEST.exists() and any(DEST.iterdir()):
    try:
        marker=json.loads((DEST/'build-manifest.json').read_text())
        assert marker['campaign']=='afterlight-accord'
    except (OSError,ValueError,KeyError,AssertionError):
        raise SystemExit('Refusing to replace a nonempty directory without an Afterlight build manifest.')
DEST.parent.mkdir(parents=True,exist_ok=True)
stage=Path(tempfile.mkdtemp(prefix='.afterlight-stage-',dir=DEST.parent))
backup=None
try:
    for name in ('index.html','style.css','CREDITS.html'):
        shutil.copy2(SOURCE/name,stage/name)
    for name in ('src','content','assets'):
        shutil.copytree(SOURCE/name,stage/name)
    shutil.copytree(COMBAT,stage/'combat')
    files={str(path.relative_to(stage)):hashlib.sha256(path.read_bytes()).hexdigest()
           for path in sorted(stage.rglob('*')) if path.is_file()}
    (stage/'build-manifest.json').write_text(json.dumps({'campaign':'afterlight-accord','files':files},indent=2)+'\n')
    if DEST.exists():
        backup=Path(tempfile.mkdtemp(prefix='.afterlight-old-',dir=DEST.parent));backup.rmdir()
        DEST.rename(backup)
    try:
        stage.rename(DEST)
    except OSError:
        if backup:backup.rename(DEST);backup=None
        raise
finally:
    if stage.exists():shutil.rmtree(stage)
    if backup and backup.exists():shutil.rmtree(backup)
print(f'Afterlight packaged: {DEST} ({len(files)} files)')
