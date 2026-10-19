#!/usr/bin/env python3
"""Package completed C2Rust output without changing the frozen experiment."""
import argparse
import hashlib
import json
from pathlib import Path
import re
import shutil
import subprocess


LEADING_COMMENTS = re.compile(r'\s*(?:(?://[^\n]*(?:\n|$)|/\*[\s\S]*?\*/)\s*)*')


def normalize_leading_comment_whitespace(text):
    """Strip trailing spaces only in the copied notice, never Rust literals."""
    match = LEADING_COMMENTS.match(text)
    return re.sub(r'[ \t]+(?=\r?$)', '', match[0], flags=re.M) + text[match.end():]


def leading_comments(text):
    match = LEADING_COMMENTS.match(text)
    return normalize_leading_comment_whitespace(match[0]).strip()


def package(generated, source, output, edition):
    generated, source, output = (Path(p).resolve() for p in (generated, source, output))
    if output.exists() and any(output.iterdir()):
        raise ValueError('Packaging output must be empty; use a fresh staging directory.')
    output.mkdir(parents=True, exist_ok=True)
    mapping = []
    for path in sorted(generated.rglob('*.rs')):
        relative = path.relative_to(generated)
        if 'target' in relative.parts or relative == Path('build.rs'):
            continue
        text = path.read_text()
        original = source / relative.with_suffix('.c')
        if relative.parts[0] == 'src' and original.exists():
            raw = original.read_bytes()
            notices = leading_comments(raw.decode('utf-8', errors='replace'))
            text = notices + '\n\n// Rust translation of ' + relative.with_suffix('.c').as_posix() + '.\n' + text
            mapping.append({'rust': relative.as_posix(), 'c': relative.with_suffix('.c').as_posix(),
                            'source_sha256': hashlib.sha256(raw).hexdigest()})
        else:
            text = '// SPDX-License-Identifier: GPL-2.0-or-later\n// Rust engine integration support.\n' + text
        # C __FILE__ diagnostics remain useful without embedding a developer's home.
        text = text.replace(str(source) + '/', '')
        if '/Users/' in text or '/opt/homebrew/' in text:
            raise ValueError(f'unexpected machine-specific path in {relative}')
        destination = output / relative
        destination.parent.mkdir(parents=True, exist_ok=True)
        destination.write_text(text)

    name = 'uqm-engine-' + edition
    manifest = (generated / 'Cargo.toml').read_text()
    old_name = re.search(r'\[package\][\s\S]*?^name = "([^"]+)"', manifest, re.M)[1]
    manifest = manifest.replace(f'name = "{old_name}"', f'name = "{name}"', 1)
    manifest = manifest.replace(f'name = "{old_name}"', f'name = "uqm_engine_{edition}"')
    manifest = manifest.replace('version = "0.0.0"', 'version = "0.1.0"')
    manifest = manifest.replace('authors = ["C2Rust"]', 'authors = ["The Ur-Quan Masters contributors"]\nlicense = "GPL-2.0-only"')
    dependency = 'uqm-core = { path = "../uqm-core"' + (', features = ["hd"]' if edition == 'hd' else '') + ' }'
    manifest = manifest.replace('[dependencies]', '[dependencies]\n' + dependency)
    manifest += '\n[profile.dev]\npanic = "abort"\n\n'
    manifest = manifest.replace('[profile.release]', '[profile.release]\npanic = "abort"')
    (output / 'Cargo.toml').write_text(manifest.rstrip() + '\n')
    lock = (generated / 'Cargo.lock').read_text().replace(f'name = "{old_name}"\nversion = "0.0.0"', f'name = "{name}"\nversion = "0.1.0"')
    marker = f'[[package]]\nname = "{name}"\nversion = "0.1.0"'
    begin = lock.index(marker)
    end = lock.find('\n[[package]]', begin + len(marker))
    if end < 0:
        end = len(lock)
    block = lock[begin:end]
    if 'dependencies = [' in block:
        block = block.replace('dependencies = [', 'dependencies = [\n "uqm-core",', 1)
    else:
        block = block.rstrip() + '\ndependencies = [\n "uqm-core",\n]\n'
    lock = lock[:begin] + block + lock[end:]
    lock += '\n[[package]]\nname = "uqm-core"\nversion = "0.1.0"\n'
    (output / 'Cargo.lock').write_text(lock)
    (output / 'rust-toolchain.toml').write_text('[toolchain]\nchannel = "nightly-2025-02-01"\ncomponents = ["rust-src"]\n')
    (output / 'SOURCE-MAP.json').write_text(json.dumps(mapping, indent=2) + '\n')
    for filename in ['uqm_rust_entry.c', 'init-fix-report.json', 'initialization-report.json', 'normalization-report.json', 'uqm_asm_consts.c', 'asm-const-report.json']:
        if (generated / filename).exists():
            shutil.copyfile(generated / filename, output / filename)
    license_source = source / ('COPYING' if edition == 'original' else 'LICENSE')
    shutil.copyfile(license_source, output / 'COPYING')
    (output / 'notices').mkdir(exist_ok=True)
    shutil.copyfile(Path(__file__).resolve().parent / 'licenses/LGPL-2.0.txt', output / 'notices/LGPL-2.0.txt')
    for relative in ['AUTHORS', 'src/libs/uio/COPYING', 'src/libs/mikmod/AUTHORS', 'src/libs/mikmod/README', 'src/libs/md5/README']:
        if (source / relative).exists():
            destination = output / 'notices' / relative
            destination.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(source / relative, destination)

    lib = output / 'lib.rs'
    text = lib.read_text()
    migrated = ['random', 'random2', 'sqrt', 'trans', 'velocity', 'displist', 'boxint', 'clipline', 'bbox']
    if edition == 'original':
        migrated.append('crc')
    for module in migrated:
        declaration = f'pub mod {module};'
        if text.count(declaration) != 1:
            raise ValueError(f'expected one module declaration for {module}')
        text = text.replace(declaration, f'// {module}: supplied by the independently tested uqm-core crate.')
    text = re.sub(r'\n#\[panic_handler\]\nfn panic\(_:\s*&core::panic::PanicInfo\) -> ! \{ core::arch::wasm32::unreachable\(\) \}\n?', '\n', text)
    if '#[panic_handler]' in text:
        raise ValueError('unrecognized duplicate panic handler')
    position = text.index('extern crate alloc;')
    guard = '#[cfg(not(target_os = "emscripten"))]\ncompile_error!("Build this engine for wasm32-unknown-emscripten using scripts/rust-engine/build.sh.");\nextern crate uqm_core;\n'
    text = text[:position] + guard + text[position:]
    if edition == 'hd':
        text += '\nmod hd_rng_config;\n'
        random = (generated / 'src/libs/math/random.rs').read_text()
        start = random.index('#[no_mangle]\npub unsafe extern "C" fn printBits')
        end = random.index('{', start) + 1
        depth = 1
        while depth:
            depth += (random[end] == '{') - (random[end] == '}')
            end += 1
        print_bits = random[start:end]
        helper = '''// SPDX-License-Identifier: GPL-2.0-or-later
// SeedA from src/libs/math/random.h and printBits from random.c.
// Access follows the same engine synchronization as the original macros.
type size_t = core::ffi::c_ulong;
extern "C" {
    fn printf(format: *const core::ffi::c_char, ...) -> core::ffi::c_int;
    fn puts(text: *const core::ffi::c_char) -> core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn uqm_hd_seed_a() -> u32 {
    let seed = crate::src::uqm::globdata::GlobData.SIS_state.Seed;
    if crate::src::options::optSeedType == 1 && (3..=2_147_483_644).contains(&seed) {
        seed as u32
    } else {
        16_807
    }
}
'''
        notice = leading_comments((source / 'src/libs/math/random.c').read_text())
        (output / 'hd_rng_config.rs').write_text(notice + '\n\n' + helper + print_bits + '\n')
    lib.write_text(text)
    solar_helper = Path(__file__).resolve().parent / 'solar-sort-fix.py'
    subprocess.run(['python3', str(solar_helper), str(output), '--edition', edition], check=True)
    asm_helper = Path(__file__).resolve().parent / 'asm-const-fix.py'
    subprocess.run(['python3', str(asm_helper), str(output)], check=True)
    # PNG error handling must return through C libpng guards before Rust resumes.
    # The shared implementation is manually ported and oracle-tested; never
    # re-enable the generated setjmp/longjmp body during regeneration.
    png = output / 'src/libs/graphics/sdl/png2sdl.rs'
    png_notice = leading_comments((source / 'src/libs/graphics/sdl/png2sdl.c').read_text())
    png.write_text(png_notice + '\n\n// Shared Rust PNG implementation; C contains only guarded library calls.\n'
                   '#[path = "../../../../../uqm-png/png.rs"]\n'
                   'mod implementation;\npub use implementation::*;\n')
    lib.write_text(lib.read_text().replace('// External libpng adapter.',
                   '// Shared Rust PNG orchestration with guarded C library calls.')
                   .replace('// pub mod png2sdl;', 'pub mod png2sdl;'))
    for item in mapping:
        if item['c'] == 'src/libs/graphics/sdl/png2sdl.c':
            item['implementation'] = '../uqm-png/png.rs'
        elif Path(item['rust']).stem in migrated:
            item['implementation'] = '../uqm-core/Cargo.toml'
    (output / 'SOURCE-MAP.json').write_text(json.dumps(mapping, indent=2) + '\n')
    if edition == 'hd':
        lua_helper = Path(__file__).resolve().parent / 'lua-fix.py'
        subprocess.run(['python3', str(lua_helper), str(output), '--apply'], check=True)
    # lib.rs ends with module declarations, never a multiline string literal.
    lib.write_text(lib.read_text().rstrip() + '\n')
    print(f'Packaged {len(mapping)} mapped source units into {output}')


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--generated', required=True)
    parser.add_argument('--source', required=True)
    parser.add_argument('--output', required=True)
    parser.add_argument('--edition', choices=['original', 'hd'], required=True)
    args = parser.parse_args()
    package(args.generated, args.source, args.output, args.edition)
