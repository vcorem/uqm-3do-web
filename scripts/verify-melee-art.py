#!/usr/bin/env python3
"""Check coverage, geometry, hashes and immutable collision source assets."""
import hashlib
import io
import json
from pathlib import Path
import zipfile
from PIL import Image

ROOT = Path(__file__).resolve().parents[1]
RENDER = ROOT / 'hd/remastered/render'
manifest = json.loads((RENDER / 'manifest.json').read_text())
archive_path = ROOT / '.tools/hd-assets/mm-0.8.5-hd-content.uqm'
digest = lambda data: hashlib.sha256(data).hexdigest()
assert digest(archive_path.read_bytes()) == manifest['archive_sha256'], 'Pinned original archive changed'
assert manifest['ship_forms'] == 28
seen = set()
coverage = {}
with zipfile.ZipFile(archive_path) as archive:
    for entry in manifest['frames']:
        name = entry['path']
        assert name not in seen, f'Duplicate output: {name}'
        seen.add(name)
        raw = (RENDER / name).read_bytes()
        assert digest(raw) == entry['sha256'], f'Out of date art manifest: {name}'
        original = archive.read(entry['source'])
        assert digest(original) == entry['source_sha256'], f'Collision source changed: {name}'
        image = Image.open(io.BytesIO(raw)).convert('RGBA')
        baseline = Image.open(io.BytesIO(original))
        assert list(image.size) == entry['size'] == list(baseline.size), f'Geometry differs: {name}'
        extrema = image.getchannel('A').getextrema()
        assert extrema[1] > 0, f'Empty art: {name}'
        if min(image.size) >= 8:
            assert extrema[0] == 0, f'Missing transparent background: {name}'
        assert raw != original, f'Unchanged placeholder passed as new artwork: {name}'
        if name.startswith('ships/') and not entry.get('ui'):
            stem = name.rsplit('-', 1)[0]
            coverage[stem] = coverage.get(stem, 0) + 1
assert len(coverage) == 28 * 3
assert all(count == 16 for count in coverage.values()), coverage
assert Image.open(RENDER / 'background.png').size == (1280, 960)
print(f"PASS: {len(coverage)//3} forms, all 16 facings at three zooms, {len(seen)} authored render frames")
print('PASS: every original collision PNG and pinned content archive remains byte-identical')
