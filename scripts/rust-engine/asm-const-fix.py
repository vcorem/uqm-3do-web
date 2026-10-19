#!/usr/bin/env python3
"""Preserve Emscripten JS constants as data, not Rust Wasm custom sections.

Only string storage is moved to the platform adapter. Rust still implements
each game function and calls the same Emscripten API with unchanged arguments.
The helper validates literal lengths and fails on unexpected generated syntax.
"""
import argparse
import ast
import hashlib
import json
from pathlib import Path
import re

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('crate', type=Path)
args = parser.parse_args()
root = args.crate.resolve()
marker = '#[link_section = "em_asm"]'
pattern = re.compile(r'#\[link_section = "em_asm"\]\s*'
                     r'static mut x: \[::core::ffi::c_char; (\d+)\] = unsafe \{'
                     r'(.*?)\n\s*\};\s*&raw const x as \*const ::core::ffi::c_char', re.S)
literal = re.compile(r'\*b("(?:\\.|[^"\\])*")', re.S)
changes, records, definitions = [], [], []
existing_references = []
for path in sorted((root / 'src').rglob('*.rs')):
    text = path.read_text()
    if re.search(r'\bstatic uqm_em_asm_\d+\s*:', text):
        existing_references.append(path)
    count = text.count(marker)
    if not count:
        continue
    matches = list(pattern.finditer(text))
    if len(matches) != count:
        raise SystemExit('Unexpected EM_ASM declaration in ' + str(path))
    def replace(match):
        values = literal.findall(match[2])
        if len(values) != 1:
            raise ValueError('Expected one EM_ASM byte literal: ' + str(path))
        payload = ast.literal_eval('b' + values[0])
        length = int(match[1])
        if len(payload) != length or payload[-1:] != b'\0':
            raise ValueError('EM_ASM length or NUL mismatch: ' + str(path))
        symbol = 'uqm_em_asm_' + str(len(records))
        records.append({'source': path.relative_to(root).as_posix(),
                        'symbol': symbol, 'bytes': length,
                        'sha256': hashlib.sha256(payload).hexdigest()})
        definitions.append('__attribute__((section("em_asm"), used, aligned(1)))\n'
                           + 'const char ' + symbol + '[' + str(length) + '] = {\n  '
                           + ', '.join(str(value) for value in payload) + '\n};\n')
        return ('extern "C" { static ' + symbol + ': [::core::ffi::c_char; '
                + str(length) + ']; }\n            &raw const ' + symbol
                + ' as *const ::core::ffi::c_char')
    changes.append((path, pattern.sub(replace, text)))

adapter = root / 'uqm_asm_consts.c'
report = root / 'asm-const-report.json'
if changes and existing_references:
    raise SystemExit('Mixed original and extracted EM_ASM declarations; regenerate into a fresh staging directory')
if not changes:
    if not adapter.is_file() or not report.is_file():
        raise SystemExit('No generated EM_ASM declarations and no existing adapter')
    existing = json.loads(report.read_text())
    storage = re.findall(r'const char (uqm_em_asm_\d+)\[(\d+)\] = \{\s*([\d,\s]+)\s*\};', adapter.read_text())
    if len(storage) != len(existing) or len({name for name, _, _ in storage}) != len(storage):
        raise SystemExit('Existing EM_ASM adapter has missing or duplicated definitions')
    payloads = {}
    for name, length, values in storage:
        try:
            payload = bytes(int(value.strip()) for value in values.split(',') if value.strip())
        except ValueError:
            raise SystemExit('Existing EM_ASM adapter has invalid bytes')
        if len(payload) != int(length) or payload[-1:] != b'\0':
            raise SystemExit('Existing EM_ASM adapter length or NUL mismatch')
        payloads[name] = payload
    for item in existing:
        payload = payloads.get(item['symbol'])
        if payload is None or len(payload) != item['bytes'] or hashlib.sha256(payload).hexdigest() != item['sha256']:
            raise SystemExit('Existing EM_ASM adapter checksum does not match report')
        source = (root / item['source']).resolve()
        if not source.is_relative_to(root):
            raise SystemExit('Existing EM_ASM report contains a path outside the crate')
        declaration = 'static ' + item['symbol'] + ': [::core::ffi::c_char; ' + str(item['bytes']) + ']'
        if declaration not in source.read_text():
            raise SystemExit('Existing EM_ASM adapter no longer matches source')
    print('EM_ASM data adapter already applied:', root)
else:
    # Validate the entire input before modifying any file.
    for path, content in changes:
        path.write_text(content)
    adapter.write_text('/* Generated platform data only: original JS bytes unchanged.\n'
                       ' * Rust link_section emits CUSTOM sections on Wasm; emcc\n'
                       ' * requires DATA segments for its ASM_CONSTS extraction. */\n'
                       + '\n'.join(definitions))
    report.write_text(json.dumps(records, indent=2) + '\n')
    print('Extracted', len(records), 'EM_ASM constants into', adapter)
