#!/usr/bin/env python3
"""Verify pinned public C exports are supplied by the complete Rust engine."""
import argparse
import json
from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parents[2]
TOOLS = Path(__file__).resolve().parent


def check(edition, archive, nm=None, ar=None):
    archive = Path(archive).resolve()
    nm = Path(nm or ROOT / '.tools/emsdk/upstream/bin/llvm-nm')
    ar = Path(ar or ROOT / '.tools/emsdk/upstream/bin/llvm-ar')
    snapshot = json.loads((TOOLS / f'{edition}-exports.json').read_text())
    inputs = json.loads((TOOLS / f'{edition}-inputs.json').read_text())
    records = snapshot['source_units']
    if [item['source'] for item in records] != inputs['source_units']:
        raise ValueError('Export snapshot does not cover the locked translation source set')
    for item in records:
        if item['source_sha256'] != inputs['sha256'][item['source']]:
            raise ValueError('Export snapshot source hash differs from translation lock: ' + item['source'])
    if snapshot['target'] != 'wasm32-unknown-emscripten' or snapshot['emscripten'] != (ROOT / '.emscripten-version').read_text().strip():
        raise ValueError('Export snapshot target/toolchain differs from this build')
    members = subprocess.run([str(ar), 't', str(archive)], check=True, text=True, capture_output=True).stdout.splitlines()
    bad_members = [name for name in members if not name.endswith('.rcgu.o')]
    if not members or bad_members:
        raise ValueError('Engine archive contains unexpected non-Rust object members: ' + ', '.join(bad_members))
    output = subprocess.run([str(nm), '--format=posix', '--defined-only', '--extern-only', str(archive)],
                            check=True, text=True, capture_output=True).stdout
    actual = {parts[0] for line in output.splitlines() if len(parts := line.split()) >= 2 and len(parts[1]) == 1}
    expected = {name: item['source'] for item in records for name in item['symbols']}
    missing = {name: source for name, source in expected.items() if name not in actual}
    allowed = set(snapshot['allowed_missing'])
    if allowed != {'__main_argc_argv', 'crcTable'}:
        raise ValueError('Export snapshot contains an unreviewed missing-symbol exception')
    unexpected = {name: source for name, source in missing.items() if name not in allowed}
    result = {'edition': edition, 'source_units': len(records), 'expected_public_symbols': len(expected),
              'rust_archive_members': len(members), 'missing': missing, 'unexpected_missing': unexpected}
    if unexpected:
        raise ValueError('Missing game exports: ' + json.dumps(unexpected, sort_keys=True))
    return result


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('edition', choices=['original', 'hd'])
    parser.add_argument('archive')
    parser.add_argument('--report', type=Path)
    parser.add_argument('--nm')
    parser.add_argument('--ar')
    args = parser.parse_args()
    result = check(args.edition, args.archive, args.nm, args.ar)
    if args.report:
        args.report.parent.mkdir(parents=True, exist_ok=True)
        args.report.write_text(json.dumps(result, indent=2) + '\n')
    print(json.dumps(result, indent=2))
