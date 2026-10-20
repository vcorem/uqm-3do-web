#!/usr/bin/env python3
"""Fail closed when a whole-engine link includes legacy C game inputs."""
from pathlib import Path

LUA_RUNTIME = frozenset('lapi.c lauxlib.c lbaselib.c lbitlib.c lcode.c lctype.c ldebug.c ldo.c ldump.c lfunc.c lgc.c llex.c lmathlib.c lmem.c lobject.c lopcodes.c lparser.c lstate.c lstring.c lstrlib.c ltable.c ltablib.c ltm.c lundump.c lvm.c lzio.c'.split())
BINARY_SUFFIXES = {'.o', '.obj', '.a', '.bc', '.c', '.cc', '.cpp', '.rs', '.wasm', '.s', '.so', '.dylib', '.dll', '.lib'}
NATIVE_LIBRARIES = {'-lpng-mt', '-lpng', '-lm', '-lopenal', '-lvorbis', '-logg', '-lz', '-lpthread', '-lidbfs.js'}


def validate(root, edition, source, generated, library, adapters, link_flags):
    root, source, generated, library = (Path(value).resolve() for value in (root, source, generated, library))
    if edition not in {'original', 'hd'}:
        raise ValueError('Unknown engine edition')
    if library.name != f'libuqm_engine_{edition}.a':
        raise ValueError('Expected the complete Rust engine static library')
    allowed = {generated / 'uqm_rust_entry.c', generated / 'uqm_asm_consts.c',
               root / 'rust/uqm-png/png-guard.c'}
    if edition == 'hd':
        allowed.add(root / 'rust/uqm-lua/lua-guard.c')
        allowed.update(source / 'src/libs/lua' / name for name in LUA_RUNTIME)
    resolved = [Path(path).resolve() for path in adapters]
    if len(set(resolved)) != len(resolved):
        raise ValueError('Duplicate C adapter in whole-engine link')
    disallowed = set(resolved) - allowed
    if disallowed:
        raise ValueError('C game/unknown source forbidden in Rust engine link: ' + ', '.join(map(str, sorted(disallowed))))
    required = {generated / 'uqm_rust_entry.c', generated / 'uqm_asm_consts.c', root / 'rust/uqm-png/png-guard.c'}
    if not required.issubset(resolved):
        raise ValueError('Missing required explicit runtime adapters')
    compatibility_object = (source / 'wasm/compat/emscripten-proxying.o').resolve()
    seen_compatibility = False
    for flag in link_flags:
        if flag.startswith('-l') and flag not in NATIVE_LIBRARIES:
            raise ValueError('Unreviewed native library in engine link: ' + flag)
        if flag.startswith('-L'):
            raise ValueError('Custom native library search paths are forbidden in the engine link')
        # Reject response files, which could conceal arbitrary source/object inputs.
        if flag.startswith('@') or ',@' in flag:
            raise ValueError('Response files are forbidden in the engine link')
        candidates = flag.split(',') if flag.startswith('-Wl,') else [flag]
        for candidate in candidates:
            if candidate.startswith('-l') and candidate not in NATIVE_LIBRARIES:
                raise ValueError('Unreviewed native library in engine link: ' + candidate)
            if candidate.startswith('-L'):
                raise ValueError('Custom native library search paths are forbidden in the engine link')
            value = candidate.split('=', 1)[-1]
            if Path(value).suffix.lower() in BINARY_SUFFIXES:
                resolved_flag = (source / value).resolve()
                if resolved_flag != compatibility_object:
                    raise ValueError('C game/unknown object forbidden in Rust engine link: ' + candidate)
                seen_compatibility = True
    def label(path):
        for base, prefix in [(root, ''), (generated, 'generated/'), (source, 'source/')]:
            if path.is_relative_to(base):
                return prefix + path.relative_to(base).as_posix()
        raise ValueError('Adapter is outside all reviewed roots')
    return {'edition': edition, 'engine_archive': library.name,
            'c_adapters': [label(path) for path in resolved],
            'emscripten_compatibility_object': seen_compatibility,
            'legacy_c_game_objects': []}
