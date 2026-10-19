#!/usr/bin/env python3
"""Capture a reviewed export snapshot from a complete pinned C reference build.

This does not build C and never updates the checked-in snapshots implicitly.
Use --output to write a staging snapshot, then review the difference.
"""
import argparse
import hashlib
import json
from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parents[2]
TOOLS = Path(__file__).resolve().parent
parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('edition', choices=['original', 'hd'])
parser.add_argument('--source', type=Path, required=True)
parser.add_argument('--output', type=Path, required=True)
args = parser.parse_args()
source = args.source.resolve()
inputs = json.loads((TOOLS / f'{args.edition}-inputs.json').read_text())
for relative, expected in inputs['sha256'].items():
    if hashlib.sha256((source / relative).read_bytes()).hexdigest() != expected:
        raise SystemExit('C reference input differs from the translation lock: ' + relative)
nm = ROOT / '.tools/emsdk/upstream/bin/llvm-nm'
records = []
for relative in inputs['source_units']:
    obj = source / ('obj/debug-wasm/' + relative + '.o')
    if not obj.is_file():
        raise SystemExit('Missing C reference object; build the complete C edition first: ' + str(obj))
    output = subprocess.run([str(nm), '--format=posix', '--defined-only', '--extern-only', str(obj)],
                            check=True, text=True, capture_output=True).stdout
    symbols = {parts[0]: parts[1] for line in output.splitlines()
               if len(parts := line.split()) >= 2 and len(parts[1]) == 1}
    records.append({'source': relative, 'source_sha256': inputs['sha256'][relative], 'symbols': symbols})
result = {'edition': args.edition, 'target': 'wasm32-unknown-emscripten',
          'emscripten': (ROOT / '.emscripten-version').read_text().strip(),
          'source_units': records, 'allowed_missing': {
              '__main_argc_argv': 'Provided by the explicit C platform entry adapter.'}}
result['allowed_missing']['crcTable'] = 'Private implementation data replaced by uqm-core; no game caller references it.'
args.output.parent.mkdir(parents=True, exist_ok=True)
args.output.write_text(json.dumps(result, indent=2) + '\n')
print(f'Captured {len(records)} source units into {args.output}')
