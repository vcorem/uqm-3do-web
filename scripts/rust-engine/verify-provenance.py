#!/usr/bin/env python3
"""Check the reviewed translator patches and transformation pipeline pins."""
import argparse
import hashlib
import json
from pathlib import Path
import re


def verify(directory):
    directory = Path(directory).resolve()
    root = directory.parents[1]
    provenance = json.loads((directory / 'provenance.json').read_text())
    files = provenance.get('reproduction_sha256', {})
    if not files:
        raise ValueError('Missing reproduction file hashes in provenance.json')
    for relative, expected in files.items():
        path = Path(relative)
        if path.is_absolute() or '..' in path.parts or '.tools' in path.parts:
            raise ValueError(f'Nonportable reproduction path: {relative}')
        resolved = (root / path).resolve()
        if root not in resolved.parents:
            raise ValueError(f'Reproduction path escapes repository: {relative}')
        if not re.fullmatch(r'[0-9a-f]{64}', expected):
            raise ValueError(f'Invalid reproduction hash: {relative}')
        if not resolved.is_file() or hashlib.sha256(resolved.read_bytes()).hexdigest() != expected:
            raise ValueError(f'Reproduction input changed: {relative}; review and update its provenance hash')
    if provenance.get('translate_const_macros') != 'none' or provenance.get('translate_fn_macros') != 'none':
        raise ValueError('Both macro reconstruction modes must remain disabled')
    return provenance


def verify_versions(emcc_version, clang_version, provenance):
    emcc = re.search(r'^emcc .*?\) (\d+\.\d+\.\d+)\b', emcc_version, re.M)
    clang = re.search(r'\bclang version (\d+\.\d+\.\d+)\b', clang_version)
    if not emcc or emcc[1] != provenance['emscripten_version']:
        raise ValueError('Emscripten version differs from the recorded translation toolchain')
    if not clang or clang[1] != provenance['llvm_version']:
        raise ValueError('Clang version differs from the recorded translation toolchain')


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--directory', type=Path, default=Path(__file__).resolve().parent)
    args = parser.parse_args()
    result = verify(args.directory)
    print(f'Verified {len(result["reproduction_sha256"])} reproduction inputs.')
