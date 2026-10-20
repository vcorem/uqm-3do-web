#!/usr/bin/env python3
"""Build a target-correct Clang database from the recorded source/config lock."""
import argparse
import hashlib
import json
from pathlib import Path
import shlex
import subprocess
import runpy


def generate(edition, source, output, emcc, clang):
    tools = Path(__file__).resolve().parent
    source, output = Path(source).resolve(), Path(output).resolve()
    emcc, clang = Path(emcc).resolve(), Path(clang).resolve()
    checks = runpy.run_path(str(tools / 'verify-provenance.py'))
    provenance = checks['verify'](tools)
    emcc_version = subprocess.run([str(emcc), '--version'], text=True, capture_output=True, check=True)
    clang_version = subprocess.run([str(clang), '--version'], text=True, capture_output=True, check=True)
    checks['verify_versions'](emcc_version.stdout + emcc_version.stderr,
                              clang_version.stdout + clang_version.stderr, provenance)
    lock = json.loads((tools / f'{edition}-inputs.json').read_text())
    for relative, expected in lock['sha256'].items():
        path = source / relative
        if not path.is_file() or hashlib.sha256(path.read_bytes()).hexdigest() != expected:
            raise ValueError(f'input differs from the translated revision: {relative}')
    units = [source / relative for relative in lock['source_units']]
    result = subprocess.run([str(emcc), '-###', '-c', str(units[0]), *lock['cflags']],
                            cwd=source, text=True, capture_output=True, check=True)
    lines = (result.stdout + result.stderr).strip().splitlines()
    driver = shlex.split(lines[-1])
    if '-target' not in driver or 'wasm32-unknown-emscripten' not in driver:
        raise ValueError('emcc did not produce the expected Wasm compiler invocation')
    normalized = [str(Path(clang).resolve())]
    i = 1
    while i < len(driver):
        value = driver[i]
        if value == '-mllvm':
            i += 2  # Backend optimization/SjLj switches do not affect the C AST.
            continue
        if value == str(units[0]) or value == '-###' or value.startswith('-g') or value.startswith('-O'):
            i += 1
            continue
        normalized.append(value)
        i += 1
    normalized += ['-O0', '-Wno-builtin-macro-redefined',
                   '-D__DATE__="' + lock['build_date'] + '"',
                   '-D__TIME__="' + lock['build_time'] + '"']
    commands = []
    for unit in units:
        arguments = normalized + [str(unit)]
        if edition == 'hd' and unit.name == 'starmap.c':
            arguments += ['-include', str(tools / 'hd-starmap-prelude.h')]
        commands.append({'directory': str(source), 'file': str(unit), 'arguments': arguments})
    output.mkdir(parents=True, exist_ok=True)
    (output / 'compile_commands.json').write_text(json.dumps(commands, indent=2) + '\n')
    return output / 'compile_commands.json'


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--edition', choices=['original', 'hd'], required=True)
    parser.add_argument('--source', required=True)
    parser.add_argument('--output', required=True)
    parser.add_argument('--emcc', required=True)
    parser.add_argument('--clang', required=True)
    args = parser.parse_args()
    print(generate(args.edition, args.source, args.output, args.emcc, args.clang))
