#!/usr/bin/env python3
"""Write the third-party notices of the site (web/editions/THIRD-PARTY-NOTICES.txt).

The file names the third-party code that is compiled into, or sent with, the
published builds: the UQM engines (Original and HD, C and Rust), Melee
Remastered (the HD builds again) and Cosmo Clash. Build tools are not listed.
Each part has its version, the builds that use it, its copyright lines, its
license (SPDX) and its license text. The script reads them from the pinned
sources: the Emscripten SDK and its port cache, the UQM and UQM-MegaMod
sources, the Rust toolchains and the cargo registry, and Cosmo Clash's
node_modules. It stops with an error when a source, a version or a text is
missing or not as expected, so that a dependency change gets a review here.

Three texts are not in those sources. FETCHED keeps them with the URL, the tag
or commit and the SHA-256 of the copy that was fetched once from GitHub.

    python3 scripts/make-third-party-notices.py --date 2026-10-23
    python3 scripts/make-third-party-notices.py --date 2026-10-23 --check
"""
import argparse
import dataclasses
import datetime
import hashlib
import html
import json
import os
from pathlib import Path
import re
import subprocess
import textwrap

ROOT = Path(__file__).resolve().parents[1]
WIDTH = 76

BUILDS = (
    ('/original/', 'The Ur-Quan Masters, C engine'),
    ('/original-rust/', 'The Ur-Quan Masters, Rust engine (port of the C engine)'),
    ('/hd/', 'UQM HD (UQM-MegaMod 0.8.5 source), C engine'),
    ('/hd-rust/', 'UQM HD, Rust engine (port of the HD C engine)'),
    ('/melee/', 'Melee Remastered: the /hd/ build with the Melee page'),
    ('/melee-rust/', 'Melee Remastered: the /hd-rust/ build with the Melee page'),
    ('/cosmoclash/', 'Cosmo Clash'),
)
UQM = ('/original/', '/original-rust/', '/hd/', '/hd-rust/', '/melee/', '/melee-rust/')
HD = ('/hd/', '/hd-rust/', '/melee/', '/melee-rust/')
UQM_RUST = ('/original-rust/', '/hd-rust/', '/melee-rust/')
COSMO = ('/cosmoclash/',)
EVERY = UQM + COSMO

# Emscripten ports that the UQM builds use (scripts/build-web.sh, scripts/build-hd-web.sh).
# USE_VORBIS brings ogg and libpng brings zlib (the deps of their ports).
PORT_FLAGS = {'USE_SDL', 'USE_LIBPNG', 'USE_ZLIB', 'USE_VORBIS', 'USE_OGG'}
# Registry crates in the linked code (proc-macro and build-only crates are not linked).
# The script finds them in the Cargo.lock files and stops when the set changes.
UQM_RUST_CRATES = {'c2rust-bitfields'}
COSMO_CRATES = {'itoa', 'memchr', 'ryu', 'serde', 'serde_json'}
COSMO_NPM = {'three'}
CRATE_ROLES = {
    'itoa': 'Integer to text, for serde_json in cosmoclash.wasm.',
    'memchr': 'Byte search, for serde_json in cosmoclash.wasm.',
    'ryu': 'Floating-point number to text, for serde_json in cosmoclash.wasm.',
    'serde': 'Serialization framework in cosmoclash.wasm.',
    'serde_json': 'JSON reading and writing in cosmoclash.wasm.',
}

# The Rust 1.85.1 library for wasm32-unknown-unknown (Cosmo Clash) has these crates
# from crates.io, and their sources are not in the local registry. The LICENSE.txt of
# compiler-builtins at tag compiler_builtins-v0.1.140, fetched once from
#   https://raw.githubusercontent.com/rust-lang/compiler-builtins/compiler_builtins-v0.1.140/LICENSE.txt
# is the same file as the local 0.1.143 LICENSE.txt (same SHA-256).
COMPILER_BUILTINS_LICENSE_SHA256 = {
    '0.1.140': '789f172b2ebcfdd6ce9f0ef16763615c07d04fd1e0d352f7ce39c927d308de64',
    '0.1.143': '789f172b2ebcfdd6ce9f0ef16763615c07d04fd1e0d352f7ce39c927d308de64',
}
# dlmalloc-rs: its Cargo.toml at tag 0.2.7 (fetched with LICENSE-MIT, see FETCHED) gives
# version 0.2.7 and license "MIT/Apache-2.0".
DLMALLOC_RS_VERSION = '0.2.7'
# Rust 1.85.1 keeps library/core/src/unicode/unicode_data.rs under Unicode-3.0
# ("1991-2024 Unicode, Inc."), as the nightly's COPYRIGHT-library.html says; checked in
#   https://raw.githubusercontent.com/rust-lang/rust/1.85.1/REUSE.toml
#   (SHA-256 74ee216756e5cf3944a37f35934eac4656e661d846fc47205b099eb6d524dfda).
# The tables of that file are in cosmoclash.wasm (grapheme_extend), not in the UQM Rust
# engines: checked with the bytes of the tables in the built wasm files (2026-10-23).
# Vite's source says that its polyfill is adapted from es-module-shims, MIT License,
# "Copyright (C) 2018-2021 Guy Bedford" (comment in modulePreloadPolyfill.ts):
#   https://raw.githubusercontent.com/vitejs/vite/v7.3.7/packages/vite/src/node/plugins/modulePreloadPolyfill.ts
#   (SHA-256 5b5e7909c0ca30f239ce1c3f35d21706d6e2508a3c1e1f4a5a364b7932be4836)
VITE_POLYFILL_VERSION = '7.3.7'
# The crate c2rust-bitfields 0.22.1 has no license file. Its .cargo_vcs_info.json
# names the C2Rust commit that crates.io version 0.22.1 was published from.
C2RUST_BITFIELDS_COMMIT = 'e9ccd715597f7d63e8635629f4450e040a230118'


@dataclasses.dataclass
class Fetched:
    url: str
    sha256: str
    text: str


FETCHED = {
    'c2rust': Fetched(
        'https://raw.githubusercontent.com/immunant/c2rust/' + C2RUST_BITFIELDS_COMMIT + '/LICENSE',
        '2bd73a6df34e41c531e4088a97e90a489b163926fb214ac90e9c172d124bc2f8',
        'Copyright 2017-2019 Galois, Inc., Immunant, Inc.\n'
        '\n'
        'Redistribution and use in source and binary forms, with or without modification, are '
        'permitted provided that the following conditions are met:\n'
        '\n'
        '1. Redistributions of source code must retain the above copyright notice, this list of '
        'conditions and the following disclaimer.\n'
        '\n'
        '2. Redistributions in binary form must reproduce the above copyright notice, this list '
        'of conditions and the following disclaimer in the documentation and/or other materials '
        'provided with the distribution.\n'
        '\n'
        '3. Neither the name of the copyright holder nor the names of its contributors may be '
        'used to endorse or promote products derived from this software without specific prior '
        'written permission.\n'
        '\n'
        'THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND ANY '
        'EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES '
        'OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT '
        'SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, '
        'INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED '
        'TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR '
        'BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN '
        'CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY '
        'WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH '
        'DAMAGE.\n'
        '\n'
        '\n'
        'Portions of C2Rust are derived from packages written by third parties. The following '
        'third party packages are included, and carry their own copyright notices and license '
        'terms:\n'
        '\n'
        '* c2rust-ast-printer. Dual-licensed under Apache 2.0 and MIT, see '
        'c2rust-ast-printer/COPYRIGHT for details.\n'),
    'dlmalloc-rs': Fetched(
        'https://raw.githubusercontent.com/alexcrichton/dlmalloc-rs/' + DLMALLOC_RS_VERSION + '/LICENSE-MIT',
        '378f5840b258e2779c39418f3f2d7b2ba96f1c7917dd6be0713f88305dbda397',
        'Copyright (c) 2014 Alex Crichton\n'
        '\n'
        'Permission is hereby granted, free of charge, to any\n'
        'person obtaining a copy of this software and associated\n'
        'documentation files (the "Software"), to deal in the\n'
        'Software without restriction, including without\n'
        'limitation the rights to use, copy, modify, merge,\n'
        'publish, distribute, sublicense, and/or sell copies of\n'
        'the Software, and to permit persons to whom the Software\n'
        'is furnished to do so, subject to the following\n'
        'conditions:\n'
        '\n'
        'The above copyright notice and this permission notice\n'
        'shall be included in all copies or substantial portions\n'
        'of the Software.\n'
        '\n'
        'THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF\n'
        'ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED\n'
        'TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A\n'
        'PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT\n'
        'SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY\n'
        'CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION\n'
        'OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR\n'
        'IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER\n'
        'DEALINGS IN THE SOFTWARE.\n'),
    # The LICENSE at tag 1.0.0 has the copyright line that the Vite source quotes.
    'es-module-shims': Fetched(
        'https://raw.githubusercontent.com/guybedford/es-module-shims/1.0.0/LICENSE',
        '13dc18f97f767475f7ad48fdd12a2e45be30fb309c604730f406f625cf7d8f7f',
        'MIT License\n'
        '-----------\n'
        '\n'
        'Copyright (C) 2018-2021 Guy Bedford\n'
        '\n'
        'Permission is hereby granted, free of charge, to any person obtaining a copy of this '
        'software and associated documentation files (the "Software"), to deal in the Software '
        'without restriction, including without limitation the rights to use, copy, modify, '
        'merge, publish, distribute, sublicense, and/or sell copies of the Software, and to '
        'permit persons to whom the Software is furnished to do so, subject to the following '
        'conditions:\n'
        '\n'
        'The above copyright notice and this permission notice shall be included in all copies '
        'or substantial portions of the Software.\n'
        '\n'
        'THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, '
        'INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A '
        'PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT '
        'HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF '
        'CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE '
        'OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.\n'),
}


class NoticeError(Exception):
    """A required source, version or license text is missing or not as expected."""


@dataclasses.dataclass
class Shared:
    """A license text that more than one part uses: printed once, at the end."""
    title: str
    origin: str
    text: str


@dataclasses.dataclass
class Part:
    name: str
    version: str
    builds: tuple
    source: str
    copyright: list
    license: str
    texts: list  # (heading, text) or (heading, key of a Shared text)
    notes: list = dataclasses.field(default_factory=list)
    check_copyright: bool = True  # each copyright line is in the part's own texts


def fail(message):
    raise NoticeError(message)


def read(path):
    """The text of a required file, with a clear error when it is missing."""
    if not path.is_file():
        fail('Missing file: ' + str(path))
    try:
        return path.read_text(encoding='utf-8')
    except UnicodeDecodeError:
        fail('Not UTF-8 text: ' + str(path))


def tidy(text):
    """Unix line ends, tabs as spaces, no trailing spaces and no blank edge lines."""
    lines = [line.rstrip() for line in text.replace('\r\n', '\n').expandtabs(8).split('\n')]
    while lines and not lines[0]:
        lines.pop(0)
    while lines and not lines[-1]:
        lines.pop()
    return '\n'.join(lines)


def flat(text):
    """The words of a text, for comparisons that ignore line breaks."""
    return ' '.join(text.split())


def para(text, **options):
    return textwrap.fill(text, WIDTH, break_long_words=False, break_on_hyphens=False, **options)


def find(pattern, text, where, flags=re.M):
    match = re.search(pattern, text, flags)
    if not match:
        fail(f'{where}: no match for {pattern!r}')
    return match


def section(text, start, end, where):
    """The text from the first start up to, not including, the next end."""
    i = text.find(start)
    j = text.find(end, i + len(start)) if i >= 0 else -1
    if i < 0 or j < 0:
        fail(f'{where}: expected {start!r} and then {end!r}')
    return text[i:j]


def line_start(text, index):
    return text.rfind('\n', 0, index) + 1


def uncomment(text):
    """A C or Rust comment without its comment marks; '*' lines become blank lines."""
    lines = []
    for line in text.split('\n'):
        if re.fullmatch(r'\s*(/\*+|\*+/|/?\*{3,}/?)\s*', line):
            continue
        line = re.sub(r'^\s*(?://+|/\*+|\*+(?!/))\s?', '', line)
        lines.append(re.sub(r'\s*\*+/\s*$', '', line))
    return tidy(textwrap.dedent('\n'.join(lines)))


def contains(path, *fragments):
    """Each fragment is in the file (line breaks and comment marks at line starts do not count)."""
    text = flat(re.sub(r'(?m)^\s*(?:\*+|//+)(?!/)', ' ', read(path)))
    for fragment in fragments:
        if flat(fragment) not in text:
            fail(f'{path}: expected the text {fragment!r}')


def same(text, shared, where):
    if flat(text) != flat(shared.text):
        fail(f'{where}: the text is not the same as the {shared.title} text')


def apache_terms(text, where):
    """The terms of the Apache License 2.0 in text, without the appendix."""
    start = line_start(text, find(r'Apache License\s+Version 2\.0, January 2004', text, where).start())
    end = text.find('END OF TERMS AND CONDITIONS', start)
    if end < 0:
        fail(f'{where}: no end of the Apache License terms')
    return tidy(textwrap.dedent(text[start:end + len('END OF TERMS AND CONDITIONS')]))


def cargo_lock(path):
    """{name: [(version, source, dependency names)]} of a Cargo.lock."""
    packages = {}
    for block in read(path).split('[[package]]')[1:]:
        name = find(r'^name = "([^"]+)"', block, path).group(1)
        version = find(r'^version = "([^"]+)"', block, path).group(1)
        source = re.search(r'^source = "([^"]+)"', block, re.M)
        deps = re.search(r'^dependencies = \[(.*?)\]', block, re.M | re.S)
        packages.setdefault(name, []).append((version, source and source.group(1), [
            item.split()[0] for item in re.findall(r'"([^"]+)"', deps.group(1))] if deps else []))
    return packages


def manifest(path):
    """(normal dependencies, build dependencies, proc-macro) of a Cargo.toml."""
    normal, build, proc_macro, table = set(), set(), False, None
    for line in read(path).split('\n'):
        line = line.split('#', 1)[0].strip()
        header = re.fullmatch(r'(\[\[?)([^\[\]]+)\]\]?', line)
        if header and header.group(1) == '[[':
            table = '[[' + header.group(2).strip()  # [[test]], [[bench]]: not dependency tables
            continue
        if header:
            table = header.group(2).strip()
            kind = re.fullmatch(r'(?:target\.(?:"[^"]*"|\'[^\']*\'|[^.]+)\.)?'
                                r'(dependencies|build-dependencies|dev-dependencies)(?:\.(.+))?', table)
            if kind and kind.group(2):
                {'dependencies': normal, 'build-dependencies': build}.get(kind.group(1), set()).add(
                    kind.group(2).strip('"\''))
            table = kind.group(1) if kind and not kind.group(2) else table
            continue
        if '=' not in line:
            continue
        key, value = (part.strip() for part in line.split('=', 1))
        if table == 'lib' and key == 'proc-macro' and value == 'true':
            proc_macro = True
        elif table in ('dependencies', 'build-dependencies'):
            (normal if table == 'dependencies' else build).add(key.strip('"\''))
    return normal, build, proc_macro


def linked_crates(lock_path, root, local, crate_dir):
    """{name: version} of the registry crates compiled into the library of root.

    Proc-macro crates run in the compiler only, so they and what only they use are
    left out, and so are build dependencies. local maps path crates to manifests."""
    packages, found, todo, seen = cargo_lock(lock_path), {}, [root], set()
    while todo:
        name = todo.pop()
        if name in seen:
            continue
        seen.add(name)
        if len(packages.get(name, ())) != 1:
            fail(f'{lock_path}: expected one package {name}')
        version, source, deps = packages[name][0]
        if source is None:
            if name not in local:
                fail(f'{lock_path}: unknown path crate {name}')
            normal, build, proc_macro = manifest(local[name])
        else:
            normal, build, proc_macro = manifest(crate_dir(name, version) / 'Cargo.toml')
            if not proc_macro:
                found[name] = version
        if proc_macro:
            continue
        for dep in deps:
            if dep in normal:
                todo.append(dep)
            elif dep not in build:
                fail(f'{lock_path}: {name} uses {dep}, which its manifest does not name')
    return found


def crate_info(folder):
    """(license, repository, authors without e-mail) from a registry crate's Cargo.toml."""
    text = read(folder / 'Cargo.toml')
    authors = find(r'^authors = \[(.*?)\]', text, folder, re.M | re.S).group(1)
    return (find(r'^license = "([^"]+)"', text, folder).group(1),
            find(r'^repository = "([^"]+)"', text, folder).group(1),
            re.findall(r'"([^"<]+?)\s*(?:<[^>]*>)?"', authors))


def rlib_version(lib_dir, crate):
    rlibs = sorted(lib_dir.glob(f'lib{crate}-*.rlib'))
    if len(rlibs) != 1:
        fail(f'Expected one lib{crate} rlib in {lib_dir}')
    versions = set(re.findall(rb'%s-(\d+\.\d+\.\d+)' % crate.encode(), rlibs[0].read_bytes()))
    if len(versions) != 1:
        fail(f'{rlibs[0]}: expected one {crate} version, found {sorted(versions)}')
    return versions.pop().decode()


def fetched(key):
    item = FETCHED[key]
    if hashlib.sha256(item.text.encode()).hexdigest() != item.sha256:
        fail(f'The kept copy of {item.url} was changed')
    return tidy(item.text)


class Sources:
    """The pinned source trees, with clear errors for missing parts."""

    def __init__(self, args):
        self.emsdk = args.emsdk.resolve()
        self.emscripten = self.emsdk / 'upstream/emscripten'
        self.ports = self.emscripten / 'cache/ports'
        self.hd = args.hd_source.resolve()
        self.rustup = args.rustup_home.resolve()
        self.cargo = args.cargo_home.resolve()
        self.node = args.node_modules.resolve()
        for label, path in (('--emsdk', self.emscripten), ('--hd-source', self.hd / 'src'),
                            ('--rustup-home', self.rustup / 'toolchains'),
                            ('--cargo-home', self.cargo / 'registry/src'), ('--node-modules', self.node)):
            if not path.is_dir():
                fail(f'Missing {label} directory: {path}')
        self.nightly = read(ROOT / '.rust-wasm-toolchain').strip()
        build_rust = ROOT / 'cosmoclash/scripts/build-rust.sh'
        self.stable = find(r'cargo \+(\S+) build .*--target wasm32-unknown-unknown', read(build_rust),
                           build_rust).group(1)

    def crate(self, name, version):
        for index in sorted((self.cargo / 'registry/src').glob('*')):
            if (index / f'{name}-{version}').is_dir():
                return index / f'{name}-{version}'
        fail(f'Missing crate source {name}-{version} in {self.cargo / "registry/src"}')

    def toolchain(self, channel):
        found = sorted((self.rustup / 'toolchains').glob(channel + '-*'))
        if not found:
            fail(f'Missing Rust toolchain {channel} in {self.rustup / "toolchains"}')
        return found[0]

    def port_tag(self, name, key='TAG'):
        return find(rf"^{key} = '([^']+)'", read(self.emscripten / f'tools/ports/{name}.py'),
                    f'Emscripten port {name}').group(1)

    def nightly_compiler_builtins(self):
        lock = cargo_lock(self.toolchain(self.nightly) / 'lib/rustlib/src/rust/library/Cargo.lock')
        if len(lock.get('compiler_builtins', ())) != 1:
            fail(f'Expected one compiler_builtins in the library Cargo.lock of {self.nightly}')
        return lock['compiler_builtins'][0][0]


def shared_texts(src):
    docs = src.toolchain(src.stable) / 'share/doc/rust'
    rt = src.emscripten / 'system/lib/compiler-rt/LICENSE.TXT'
    lgpl = ROOT / 'rust/uqm-engine-original/notices/LGPL-2.0.txt'
    lgpl_text = tidy(read(lgpl))
    for marker in ('GNU LIBRARY GENERAL PUBLIC LICENSE', 'Version 2, June 1991', 'END OF TERMS AND CONDITIONS',
                   "That's all there is to it!"):
        if marker not in lgpl_text:
            fail(f'{lgpl}: not the complete text of the LGPL 2.0')
    return {
        'apache': Shared('Apache License 2.0', f'LICENSE-APACHE of the Rust project (Rust {src.stable} toolchain)',
                         apache_terms(read(docs / 'LICENSE-APACHE'), docs / 'LICENSE-APACHE')),
        'llvm-exception': Shared('LLVM exceptions to the Apache License 2.0',
                                 'LICENSE.TXT of compiler-rt (Emscripten SDK)',
                                 tidy(section(read(rt), '---- LLVM Exceptions', '=====', rt))),
        'lgpl-2.0': Shared('GNU Library General Public License, version 2',
                           'rust/uqm-engine-original/notices/LGPL-2.0.txt in the source repository', lgpl_text),
        'mit-rust': Shared('MIT License (the text of the Rust project, without a copyright line)',
                           f'LICENSE-MIT of the Rust project (Rust {src.stable} toolchain)',
                           tidy(read(docs / 'LICENSE-MIT'))),
    }


def sun_block(path):
    lines = read(path).split('\n')
    marks = [i for i, line in enumerate(lines) if '=' * 20 in line][:2]
    if len(marks) < 2:
        fail(f'{path}: no Sun Microsystems notice')
    return uncomment('\n'.join(lines[marks[0]:marks[1] + 1]))


def emscripten_parts(src, shared):
    pinned = read(ROOT / '.emscripten-version').strip()
    sdk = read(src.emscripten / 'emscripten-version.txt').strip().strip('"')
    if sdk != pinned:
        fail(f'Emscripten {sdk} in {src.emscripten}, but .emscripten-version pins {pinned}')
    if 'var MALLOC = "dlmalloc";' not in read(src.emscripten / 'src/settings.js'):
        fail('Emscripten no longer uses dlmalloc by default; review the notices')
    revision = read(src.emscripten / 'emscripten-revision.txt').strip()
    musl = src.emscripten / 'system/lib/libc/musl'
    rt = src.emscripten / 'system/lib/compiler-rt'
    rt_version = find(r'based on release (\d+\.\d+\.\d+)', read(rt / 'readme.txt'), rt / 'readme.txt').group(1)
    rt_text = read(rt / 'LICENSE.TXT')
    same(apache_terms(rt_text, rt / 'LICENSE.TXT'), shared['apache'], rt / 'LICENSE.TXT')
    rt_head = rt_text[:line_start(rt_text, find(r'Apache License\s+Version 2\.0', rt_text, rt).start())]
    third = rt_text.index('Software from third parties included')
    rt_tail = rt_text[line_start(rt_text, line_start(rt_text, third) - 1):]
    if not rt_tail.startswith('====='):
        fail(f'{rt / "LICENSE.TXT"}: unexpected layout after the LLVM exceptions')
    malloc_c = read(src.emscripten / 'system/lib/dlmalloc.c')
    dedication = find(r'This is a version \(aka dlmalloc\).*?dl@cs\.oswego\.edu', malloc_c, 'dlmalloc.c', re.S)
    libm = src.crate('compiler_builtins', src.nightly_compiler_builtins()) / 'libm/src/math'
    sun = []
    for c_file, rust_file, office in (('sin.c', 'sin.rs', 'SunPro'),
                                      ('__rem_pio2_large.c', 'rem_pio2_large.rs', 'SunSoft')):
        notice = sun_block(musl / 'src/math' / c_file)
        if flat(sun_block(libm / rust_file)) != flat(notice) or 'Developed at ' + office not in notice:
            fail(f'{musl / "src/math" / c_file} and {libm / rust_file}: the Sun notices differ')
        sun.append((f'Notice in {c_file} (musl) and {rust_file} (Rust libm)', notice))
    return [
        Part('Emscripten', pinned, UQM, f'https://github.com/emscripten-core/emscripten, {pinned} '
             f'(revision {revision})', ['Copyright (c) 2010-2014 Emscripten authors, see AUTHORS file.'],
             'MIT OR NCSA', [('LICENSE', tidy(read(src.emscripten / 'LICENSE')))],
             ['The compiler toolchain of the UQM builds. Its JavaScript runtime (uqm-debug.js and '
              'uqm-debug.worker.js) and its system libraries (C library glue, threads, OpenAL audio, '
              'WebGL, the IndexedDB file system) are part of each UQM build. The builds link '
              'Emscripten\'s proxying.c with a backported upstream fix '
              '(scripts/prepare-emscripten-proxy.py).']),
        Part('musl libc', read(musl / 'VERSION').strip(), UQM, f'Emscripten {pinned}, system/lib/libc/musl',
             ['Copyright © 2005-2020 Rich Felker, et al.'], 'MIT', [('COPYRIGHT', tidy(read(musl / 'COPYRIGHT')))],
             ['The C standard library in Emscripten.']),
        Part('Math code from FreeBSD msun and Sun fdlibm', 'the copies in musl and in the Rust libm', EVERY,
             'musl src/math (UQM builds) and the libm module of compiler_builtins (Cosmo Clash)',
             ['Copyright (C) 1993 by Sun Microsystems, Inc. All rights reserved.'], 'SunPro', sun,
             ['Many math functions of musl and of the Rust libm come from FreeBSD msun and Sun fdlibm. '
              'Their source files have these notices (the "Developed at" line says "SunPro" in some '
              'files and "SunSoft" in others).']),
        Part('LLVM compiler-rt', rt_version, UQM, f'Emscripten {pinned}, system/lib/compiler-rt '
             f'(from LLVM {rt_version})', ['Copyright (c) 2009-2019 by the contributors listed in CREDITS.TXT',
                                           'Copyright (c) 2009-2015 by the contributors listed in CREDITS.TXT'],
             'Apache-2.0 WITH LLVM-exception (older parts: NCSA OR MIT)',
             [('LICENSE.TXT, start', tidy(rt_head)), ('Apache License 2.0', 'apache'),
              ('LLVM exceptions', 'llvm-exception'), ('LICENSE.TXT, end', tidy(rt_tail))],
             ['Runtime functions of the compiler, for example 128-bit arithmetic.']),
        Part('dlmalloc', find(r'\* Version (\d+\.\d+\.\d+)', malloc_c, 'dlmalloc.c').group(1), UQM,
             f'Emscripten {pinned}, system/lib/dlmalloc.c', ['Written by Doug Lea, released to the public domain'],
             'CC0-1.0', [('dlmalloc.c', tidy(textwrap.dedent(' ' + dedication.group(0))))],
             ['The memory allocator (malloc) of the UQM builds.'], check_copyright=False),
    ]


def port_parts(src):
    flags = set()
    for path in ('scripts/build-web.sh', 'scripts/build-hd-web.sh', 'sc2/build/unix/config_proginfo_host',
                 'hd/patches/0001-webassembly-platform.patch', 'scripts/rust-engine/original-inputs.json',
                 'scripts/rust-engine/hd-inputs.json'):
        flags |= set(re.findall(r'-s(?:\s+|",\s*")?(USE_[A-Z0-9_]+)', read(ROOT / path)))
    if not flags or flags - PORT_FLAGS:
        fail('The UQM builds use the Emscripten ports ' + ', '.join(sorted(flags - PORT_FLAGS or flags))
             + '; review the notices')
    for path in ('scripts/build-web.sh', 'scripts/build-hd-web.sh'):
        contains(ROOT / path, '-s USE_ZLIB=1 -s USE_LIBPNG=1 -s USE_VORBIS=1 -s USE_SDL=2')
    for name, dep in (('vorbis', 'ogg'), ('libpng', 'zlib')):
        contains(src.emscripten / f'tools/ports/{name}.py', f"deps = ['{dep}']")
    sdl_tag = src.port_tag('sdl2')
    sdl = src.ports / 'sdl2' / ('SDL-' + sdl_tag)
    header = read(sdl / 'include/SDL_version.h')
    sdl_version = '.'.join(find(rf'#define SDL_{key}\s+(\d+)', header, sdl).group(1)
                           for key in ('MAJOR_VERSION', 'MINOR_VERSION', 'PATCHLEVEL'))
    if sdl_tag != 'release-' + sdl_version:
        fail(f'The SDL port tag {sdl_tag} does not match SDL_version.h ({sdl_version})')
    port_list = read(src.emscripten / 'tools/ports/sdl2.py')
    for unit in ('video/yuv2rgb/yuv_rgb.c', 'render/software/SDL_rotate.c', 'joystick/controller_type.c'):
        if unit not in port_list:
            fail(f'The SDL port no longer builds {unit}; review the notices')
    # With these, SDL calls the C library: its own copies of qsort, strtok_r and malloc
    # (other copyright holders) are not compiled.
    contains(sdl / 'include/SDL_config_emscripten.h', '#define HAVE_QSORT 1', '#define HAVE_STRTOK_R 1',
             '#define HAVE_MALLOC 1')
    sdl_source = f'https://github.com/libsdl-org/SDL, tag {sdl_tag} (Emscripten port sdl2)'
    rotate = section(read(sdl / 'src/render/software/SDL_rotate.c'), 'Copyright (C) 2001-2011  Andreas Schiffler',
                     '*/', 'SDL_rotate.c')
    controller = read(sdl / 'src/joystick/controller_type.c')
    png_tag = src.port_tag('libpng')
    png = src.ports / 'libpng' / ('libpng-' + png_tag)
    if find(r'#define PNG_LIBPNG_VER_STRING "([^"]+)"', read(png / 'png.h'), png).group(1) != png_tag:
        fail(f'{png}: png.h does not match the port tag {png_tag}')
    zlib_version = src.port_tag('zlib', 'VERSION')
    zlib = src.ports / 'zlib' / ('zlib-' + zlib_version)
    if find(r'#define ZLIB_VERSION "([^"]+)"', read(zlib / 'zlib.h'), zlib).group(1) != zlib_version:
        fail(f'{zlib}: zlib.h does not match the port version {zlib_version}')
    xiph = {}
    for name, project in (('ogg', 'libogg'), ('vorbis', 'libvorbis')):
        tag = src.port_tag(name)
        folder = src.ports / name / (name.capitalize() + '-' + tag)
        xiph[name] = (folder, tag, find(rf'AC_INIT\(\[{project}\],\[([^\]]+)\]', read(folder / 'configure.ac'),
                                        folder).group(1))
    return [
        Part('SDL (Simple DirectMedia Layer)', sdl_version, UQM, sdl_source,
             ['Copyright (C) 1997-2022 Sam Lantinga <slouken@libsdl.org>'], 'Zlib',
             [('LICENSE.txt', tidy(read(sdl / 'LICENSE.txt')))], ['Window, input, audio output and 2D drawing.']),
        Part('yuv2rgb (part of SDL)', sdl_version, UQM, sdl_source + ', src/video/yuv2rgb',
             ['Copyright (c) 2016, Adrien Descamps'], 'BSD-3-Clause',
             [('src/video/yuv2rgb/LICENSE', tidy(read(sdl / 'src/video/yuv2rgb/LICENSE')))],
             ['YUV to RGB conversion in SDL.']),
        Part('SDL_rotate (part of SDL, from SDL_gfx)', sdl_version, UQM,
             sdl_source + ', src/render/software/SDL_rotate.c', ['Copyright (C) 2001-2011  Andreas Schiffler'],
             'Zlib', [('Notice in SDL_rotate.c', tidy(rotate))],
             ['Surface rotation in SDL. SDL_rotate.c says: "Shamelessly stolen from SDL_gfx by Andreas '
              'Schiffler. Original copyright follows:"']),
        Part('Game controller types (part of SDL)', sdl_version, UQM, sdl_source + ', src/joystick/controller_type.c',
             ['Copyright (C) Valve Corporation'], 'Zlib',
             [('Notice in controller_type.c', uncomment(controller[:controller.index('*/') + 2]))],
             ['The list of game controller types in SDL.']),
        Part('libpng', png_tag, UQM, f'Emscripten port libpng (libpng-{png_tag}.tar.gz)',
             ['Copyright (c) 1995-2022 The PNG Reference Library Authors.', 'Copyright (c) 2018-2022 Cosmin Truta.',
              'Copyright (c) 2000-2002, 2004, 2006-2018 Glenn Randers-Pehrson.',
              'Copyright (c) 1996-1997 Andreas Dilger.', 'Copyright (c) 1995-1996 Guy Eric Schalnat, Group 42, Inc.'],
             'libpng-2.0', [('LICENSE', tidy(read(png / 'LICENSE')))], ['PNG image decoding.']),
        Part('zlib', zlib_version, UQM,
             f'https://github.com/madler/zlib, tag v{zlib_version} (Emscripten port zlib)',
             ['(C) 1995-2022 Jean-loup Gailly and Mark Adler'], 'Zlib', [('LICENSE', tidy(read(zlib / 'LICENSE')))],
             ['Data decompression, for PNG images and zip packages.']),
        Part('libogg', xiph['ogg'][2], UQM,
             f'https://github.com/emscripten-ports/ogg, tag {xiph["ogg"][1]} (Emscripten port ogg)',
             ['Copyright (c) 2002, Xiph.org Foundation'], 'BSD-3-Clause',
             [('COPYING', tidy(read(xiph['ogg'][0] / 'COPYING')))], ['The Ogg container of the music and speech.']),
        Part('libvorbis', xiph['vorbis'][2], UQM,
             f'https://github.com/emscripten-ports/vorbis, tag {xiph["vorbis"][1]} (Emscripten port vorbis)',
             ['Copyright (c) 2002-2015 Xiph.org Foundation'], 'BSD-3-Clause',
             [('COPYING', tidy(read(xiph['vorbis'][0] / 'COPYING')))], ['Ogg Vorbis audio decoding.']),
    ]


def uqm_library_parts(src, hd_revision):
    mikmod = ROOT / 'sc2/src/libs/mikmod'
    readme = read(mikmod / 'README')
    if read(src.hd / 'src/libs/mikmod/README') != readme:
        fail('The MikMod copies of the Original and HD sources differ; review the notices')
    version = find(r'libmikmod library\s+version (\d+(?:\.\d+)+)', readme, mikmod / 'README').group(1)
    for edition in ('original', 'hd'):
        contains(ROOT / f'scripts/rust-engine/{edition}-inputs.json', '-DUSE_INTERNAL_MIKMOD')
    contains(mikmod / 'mikmod.h', '(c) 1998-2014 Miodrag Vallat and others', 'GNU Library General Public License',
             'either version 2 of the License, or (at your option) any later version')
    contains(mikmod / 'mwav.c', '(c) 2004, Raphael Assenat')
    contains(mikmod / 'mikmod_ctype.h', 'RPM is Copyright (c) 1998 by Red Hat Software, Inc.',
             'GNU Library General Public License')
    authors = section(read(mikmod / 'AUTHORS'), 'libmikmod main authors', 'Previous Unix maintainers',
                      mikmod / 'AUTHORS')
    ulaw = '\t' + section(read(mikmod / 'mdulaw.c'), 'The contents of this file are derived', '*/', 'mdulaw.c')
    ulaw = tidy(textwrap.dedent(re.sub(r'(?m)^(\s*)\*\* ?', r'\1', ulaw)))
    rotozoom = ROOT / 'sc2/src/libs/graphics/sdl/rotozoom.c'
    note = uncomment(section(read(rotozoom), '/*', '*/', rotozoom))
    hd_rotozoom = src.hd / 'src/libs/graphics/sdl/rotozoom.c'
    if uncomment(section(read(hd_rotozoom), '/*', '*/', hd_rotozoom)) != note:
        fail('The rotozoom.c notes of the Original and HD sources differ')
    lua_h = src.hd / 'src/libs/lua/lua.h'
    lua = read(lua_h)
    lua_version = '.'.join(find(rf'#define LUA_VERSION_{key}\s+"(\d+)"', lua, lua_h).group(1)
                           for key in ('MAJOR', 'MINOR', 'RELEASE'))
    lua_notice = uncomment(find(r'^/\*{20,}\n\* Copyright \(C\) [^\n]*Lua\.org.*?^\*{20,}/', lua, lua_h,
                                re.M | re.S).group(0))
    contains(ROOT / 'scripts/rust-engine/hd-inputs.json', '-DUSE_INTERNAL_LUA')
    megamod = f'UQM-MegaMod commit {hd_revision[:7]}'
    source_note = ('The source code of this library, as built, is in the source repository (Original) and in '
                   f'UQM-MegaMod (https://github.com/JHGuitarFreak/UQM-MegaMod, commit {hd_revision[:7]}) with '
                   'the patches in hd/patches (HD). The Rust engines are made from the same C source.')
    return [
        Part('MikMod sound library (libmikmod, changed for UQM)', version, UQM,
             f'sc2/src/libs/mikmod (Original) and src/libs/mikmod of {megamod} (HD); '
             'upstream http://sourceforge.net/projects/mikmod',
             ['(c) 1998-2014 Miodrag Vallat and others - see the AUTHORS file', '(c) 2004, Raphael Assenat (mwav.c)',
              'RPM is Copyright (c) 1998 by Red Hat Software, Inc. (mikmod_ctype.h)',
              'Copyright (C) 1989 by Jef Poskanzer (mdulaw.c)'],
             'LGPL-2.0-or-later; mdulaw.c also has the notice of Jef Poskanzer below',
             [('Main authors, from AUTHORS', tidy(authors)), ('Notice in mdulaw.c', ulaw),
              ('GNU Library General Public License 2.0', 'lgpl-2.0')],
             ['Playback of the module (MOD) music. ' + source_note], check_copyright=False),
        Part('rotozoom (from SDL_gfx 2.0, changed for UQM)', 'none given (code from SDL_gfx 2.0)', UQM,
             f'sc2/src/libs/graphics/sdl/rotozoom.c (Original) and the same file of {megamod} (HD); '
             'upstream http://www.ferzkopp.net/Software/SDL_gfx-2.0/', ['(c) A. Schiffler'],
             'LGPL (the file names no version; under the LGPL\'s own terms, LGPL-2.0-or-later)',
             [('Note in rotozoom.c', note), ('GNU Library General Public License 2.0', 'lgpl-2.0')],
             ['Rotation and zoom of images. ' + source_note]),
        Part('Lua', lua_version, HD, f'src/libs/lua of {megamod}; upstream https://www.lua.org',
             [find(r'Copyright \(C\) [\d-]+ Lua\.org, PUC-Rio\.', lua_notice, lua_h).group(0)], 'MIT',
             [('Notice at the end of lua.h', lua_notice)],
             ['The script language of UQM-MegaMod, in the HD and Melee builds.']),
    ]


def rust_parts(src, shared):
    docs = src.toolchain(src.nightly) / 'share/doc/rust'
    library = flat(html.unescape(re.sub(r'<[^>]+>', ' ', read(docs / 'COPYRIGHT-library.html'))))
    rust_developers = 'The Rust Project Developers (see https://thanks.rust-lang.org)'
    for fragment in ('File/Directory: . License: Apache-2.0 OR MIT Copyright: ' + rust_developers,
                     'File/Directory: library/core/src/unicode/unicode_data.rs License: Unicode-3.0 '
                     'Copyright: 1991-2024 Unicode, Inc'):
        if fragment not in library:
            fail(f'{docs / "COPYRIGHT-library.html"}: expected {fragment!r}')
    if flat(shared['mit-rust'].text) not in flat(read(docs / 'licences/MIT.txt')):
        fail(f'{docs / "licences/MIT.txt"}: not the MIT text of Rust {src.stable}')
    same(apache_terms(read(docs / 'licences/Apache-2.0.txt'), docs), shared['apache'], docs / 'licences/Apache-2.0.txt')
    same(read(docs / 'licences/LLVM-exception.txt'), shared['llvm-exception'], docs / 'licences/LLVM-exception.txt')
    unicode = tidy(read(docs / 'licences/Unicode-3.0.txt'))
    wasm_lib = src.toolchain(src.stable) / 'lib/rustlib/wasm32-unknown-unknown/lib'
    cb_cosmo, cb_uqm = rlib_version(wasm_lib, 'compiler_builtins'), src.nightly_compiler_builtins()
    cb_dir = src.crate('compiler_builtins', cb_uqm)
    cb_text = read(cb_dir / 'LICENSE.txt')
    for version in (cb_cosmo, cb_uqm):
        if COMPILER_BUILTINS_LICENSE_SHA256.get(version) != hashlib.sha256(cb_text.encode()).hexdigest():
            fail(f'compiler_builtins {version}: its LICENSE.txt is not checked to be {cb_dir / "LICENSE.txt"}')
    same(apache_terms(cb_text, cb_dir), shared['apache'], cb_dir / 'LICENSE.txt')
    same(section(cb_text, '---- LLVM Exceptions', '\n---', cb_dir), shared['llvm-exception'], cb_dir / 'LICENSE.txt')
    cb_license, cb_repository, cb_authors = crate_info(cb_dir)
    dlmalloc_version = rlib_version(wasm_lib, 'dlmalloc')
    if dlmalloc_version != DLMALLOC_RS_VERSION:
        fail(f'Rust {src.stable} has dlmalloc {dlmalloc_version}; fetch and check its license again')
    bitfields = set()
    for engine in ('original', 'hd'):
        lock = ROOT / f'rust/uqm-engine-{engine}/Cargo.lock'
        crates = linked_crates(lock, f'uqm-engine-{engine}', {f'uqm-engine-{engine}': lock.with_name('Cargo.toml'),
                                                              'uqm-core': ROOT / 'rust/uqm-core/Cargo.toml'}, src.crate)
        if set(crates) != UQM_RUST_CRATES:
            fail(f'{lock} links {sorted(crates)}; review the notices')
        bitfields.add(crates['c2rust-bitfields'])
    if len(bitfields) != 1:
        fail(f'The Rust engines use different c2rust-bitfields versions: {sorted(bitfields)}')
    bitfields = bitfields.pop()
    bf_dir = src.crate('c2rust-bitfields', bitfields)
    commit = json.loads(read(bf_dir / '.cargo_vcs_info.json')).get('git', {}).get('sha1')
    if commit != C2RUST_BITFIELDS_COMMIT or list(bf_dir.glob('LICENSE*')):
        fail(f'{bf_dir}: not the crate whose license text this script keeps')
    bf_license = crate_info(bf_dir)[0]
    return [
        Part('Rust standard library', f'{src.nightly} (UQM Rust engines) and {src.stable} (Cosmo Clash)',
             UQM_RUST + COSMO, 'https://github.com/rust-lang/rust', [rust_developers], 'MIT OR Apache-2.0',
             [('MIT License', 'mit-rust'), ('Apache License 2.0', 'apache')],
             [f'core and alloc in the UQM Rust engines, built from source with Rust {src.nightly} '
              '(-Zbuild-std=core,alloc,compiler_builtins); core, alloc and std in Cosmo Clash, from the '
              f'library of Rust {src.stable} for wasm32-unknown-unknown.'], check_copyright=False),
        Part('Unicode data tables (part of the Rust core library)', f'Rust {src.stable}', COSMO,
             'https://github.com/rust-lang/rust, library/core/src/unicode/unicode_data.rs',
             ['Copyright © 1991-2024 Unicode, Inc.'], 'Unicode-3.0', [('Unicode License v3', unicode)],
             ['Character tables of the Rust core library in cosmoclash.wasm.']),
        Part('compiler_builtins', f'{cb_uqm} (UQM Rust engines) and {cb_cosmo} (Cosmo Clash)', UQM_RUST + COSMO,
             cb_repository + ' (crates.io)',
             ['Copyright (c) 2009-2016 by the contributors listed in CREDITS.TXT (code from compiler-rt)',
              'Authors (Cargo.toml): ' + ', '.join(cb_authors)], cb_license,
             [('LICENSE.txt, start', tidy(section(cb_text, 'compiler-builtins as a whole', '-----', cb_dir))),
              ('LICENSE.txt, MIT License', tidy(section(cb_text, 'MIT License', '-----', cb_dir))),
              ('Apache License 2.0', 'apache'), ('LLVM exceptions', 'llvm-exception'),
              ('LICENSE.txt, end', tidy(cb_text[cb_text.index('Portions of this software are derived'):]))],
             ['Runtime functions of the Rust compiler. In Cosmo Clash it also gives the memory copy functions '
              'and the math functions (its libm module; see the FreeBSD and Sun notices above). Versions '
              f'{cb_uqm} and {cb_cosmo} have the same LICENSE.txt.'], check_copyright=False),
        Part('c2rust-bitfields', bitfields, UQM_RUST,
             f'https://github.com/immunant/c2rust, commit {C2RUST_BITFIELDS_COMMIT} (crates.io)',
             ['Copyright 2017-2019 Galois, Inc., Immunant, Inc.'], bf_license, [('LICENSE of C2Rust', fetched('c2rust'))],
             ['Bit fields of C structures in the Rust code that C2Rust made from the UQM C source. The crate '
              'has no license file: this is the LICENSE of the C2Rust repository at the commit that crates.io '
              f'version {bitfields} was published from.']),
        Part('dlmalloc (Rust crate)', dlmalloc_version, COSMO,
             f'https://github.com/alexcrichton/dlmalloc-rs, tag {dlmalloc_version}',
             ['Copyright (c) 2014 Alex Crichton'], 'MIT OR Apache-2.0', [('LICENSE-MIT', fetched('dlmalloc-rs'))],
             [f'The memory allocator of the Rust {src.stable} standard library for wasm32-unknown-unknown, a '
              'Rust port of the dlmalloc allocator. The MIT terms are given here.']),
    ]


def cosmo_parts(src, shared):
    lock = ROOT / 'cosmoclash/rust/Cargo.lock'
    crates = linked_crates(lock, 'cosmoclash-core', {'cosmoclash-core': ROOT / 'cosmoclash/rust/Cargo.toml'},
                           src.crate)
    if set(crates) != COSMO_CRATES:
        fail(f'{lock} links {sorted(crates)}; review the notices')
    parts = []
    for name in sorted(crates):
        folder = src.crate(name, crates[name])
        license_id, repository, authors = crate_info(folder)
        copyright, notes, check = ['Authors (Cargo.toml): ' + ', '.join(authors)], [CRATE_ROLES[name]], False
        if license_id in ('MIT OR Apache-2.0', 'Apache-2.0 OR BSL-1.0'):
            same(apache_terms(read(folder / 'LICENSE-APACHE'), folder), shared['apache'], folder / 'LICENSE-APACHE')
        if license_id == 'MIT OR Apache-2.0':
            same(read(folder / 'LICENSE-MIT'), shared['mit-rust'], folder / 'LICENSE-MIT')
            texts = [('MIT License', 'mit-rust'), ('Apache License 2.0', 'apache')]
        elif license_id == 'Apache-2.0 OR BSL-1.0':
            texts = [('Apache License 2.0', 'apache'), ('LICENSE-BOOST', tidy(read(folder / 'LICENSE-BOOST')))]
        elif license_id == 'Unlicense OR MIT':
            mit = tidy(read(folder / 'LICENSE-MIT'))
            copyright, check = [find(r'^Copyright .*$', mit, folder).group(0)], True
            texts, notes = [('LICENSE-MIT', mit)], [notes[0] + ' The MIT terms are given here.']
        else:
            fail(f'{folder}: no rule for the license {license_id}')
        parts.append(Part(name, crates[name], COSMO, repository + ' (crates.io)', copyright, license_id, texts,
                          notes, check_copyright=check))
    package = json.loads(read(ROOT / 'cosmoclash/package.json'))
    if set(package.get('dependencies', {})) != COSMO_NPM:
        fail('cosmoclash/package.json has other runtime dependencies; review the notices')
    npm = json.loads(read(ROOT / 'cosmoclash/package-lock.json')).get('packages', {})
    versions = {}
    for name in ('three', 'vite'):
        locked = npm.get('node_modules/' + name, {})
        installed = json.loads(read(src.node / name / 'package.json'))
        if installed.get('version') != locked.get('version') or locked.get('license') != 'MIT':
            fail(f'{src.node / name}: version {installed.get("version")}, but the lock has {locked.get("version")}')
        versions[name] = locked['version']
    if versions['vite'] != VITE_POLYFILL_VERSION:
        fail(f'Vite {versions["vite"]}: check the source of its module preload polyfill again')
    if 'modulePreload' in read(ROOT / 'cosmoclash/vite.config.ts'):
        fail('cosmoclash/vite.config.ts changes the module preload settings; review the notices')
    three = tidy(read(src.node / 'three/LICENSE'))
    vite_core = tidy(section(read(src.node / 'vite/LICENSE.md'), 'MIT License', '# Licenses of bundled dependencies',
                             src.node / 'vite/LICENSE.md'))
    return parts + [
        Part('three.js', versions['three'], COSMO, 'https://github.com/mrdoob/three.js (npm package three)',
             [find(r'^Copyright .*$', three, 'three/LICENSE').group(0)], 'MIT', [('LICENSE', three)],
             ['3D drawing, in the Cosmo Clash page script.']),
        Part('Vite module preload polyfill', versions['vite'], COSMO,
             'https://github.com/vitejs/vite (npm package vite), packages/vite/src/node/plugins/modulePreloadPolyfill.ts',
             [find(r'^Copyright .*$', vite_core, 'vite/LICENSE.md').group(0), 'Copyright (C) 2018-2021 Guy Bedford'],
             'MIT', [('Vite core license, from LICENSE.md', vite_core),
                     ('LICENSE of es-module-shims, tag 1.0.0', fetched('es-module-shims'))],
             ['A small script that Vite puts at the start of the Cosmo Clash page script. The Vite source says '
              'that it is adapted from es-module-shims (https://github.com/guybedford/es-module-shims).']),
    ]


def gpl_notes(src):
    """Notes on code from other projects that is part of the GPL program."""
    for tree in (ROOT / 'sc2/src', src.hd / 'src'):
        contains(tree / 'libs/uio/io.c', 'Copyright (C) 2003  Serge van den Boom',
                 'later versions of the GNU General Public License do not apply to this program')
        contains(tree / 'libs/graphics/sdl/hq2x.c', 'Copyright (C) 2003 MaxSt ( maxst@hiend3d.com ) - Original version')
        contains(tree / 'libs/graphics/sdl/png2sdl.c', 'adapted from IMG_png.c in the SDL2_image library',
                 '(c) 1997-2019 Sam Lantinga', 'Philippe Lavoie')
        contains(tree / 'endian_uqm.h', 'taken from SDL-1.2.5 sources', 'Original copyright (C) Sam Lantinga')
    return [
        'uio (src/libs/uio): the file system library of UQM, Copyright (C) 2003 Serge van den Boom. Its files '
        'say: "later versions of the GNU General Public License do not apply to this program" (GPL-2.0-only).',
        'hq2x scaler (src/libs/graphics/sdl/hq2x.c): adapted from hq2x (www.hiend3d.com/hq2x.html); the original '
        'version is Copyright (C) 2003 MaxSt (maxst@hiend3d.com).',
        'PNG loading (src/libs/graphics/sdl/png2sdl.c): adapted from IMG_png.c of SDL2_image, (c) 1997-2019 '
        'Sam Lantinga (zlib license); the code is credited to Philippe Lavoie.',
        'Byte order macros (src/endian_uqm.h): taken from the SDL-1.2.5 sources (original copyright (C) Sam '
        'Lantinga) and changed.',
    ]


def field(label, value):
    return para(value, initial_indent=f'{label:<12}', subsequent_indent=' ' * 12)


def notices(src, date):
    try:
        revision = subprocess.run(['git', '-C', str(src.hd), 'rev-parse', 'HEAD'], capture_output=True,
                                  text=True).stdout.strip()
    except FileNotFoundError:
        fail('Git is required to read the commit of --hd-source')
    pinned = find(r"^revision='([0-9a-f]{40})'", read(ROOT / 'scripts/build-hd-web.sh'), 'build-hd-web.sh').group(1)
    if revision != pinned:
        fail(f'{src.hd} is at {revision or "no Git commit"}, but scripts/build-hd-web.sh pins {pinned}')
    shared = shared_texts(src)
    parts = (emscripten_parts(src, shared) + port_parts(src) + uqm_library_parts(src, revision)
             + rust_parts(src, shared) + cosmo_parts(src, shared))
    seen = {flat(item.text): key for key, item in shared.items()}
    letters, users = {}, {}
    for number, part in enumerate(parts, 1):
        own = ' '.join(flat(text) for _, text in part.texts if text not in shared)
        for line in part.copyright if part.check_copyright else ():
            if flat(line) not in own:
                fail(f'{part.name}: the copyright line {line!r} is not in its license text')
        for _, text in part.texts:
            if text in shared:
                letters.setdefault(text, chr(ord('A') + len(letters)))
                users.setdefault(text, []).append(str(number))
            elif flat(text) in seen:
                fail(f'Part {number} ({part.name}) repeats the license text of {seen[flat(text)]}; share it')
            else:
                seen[flat(text)] = f'part {number}'
    rule, thin = '=' * WIDTH, '-' * WIDTH
    out = ['Third-party notices for urquanmasters.org', rule, '',
           para('This file lists the third-party software in the games on urquanmasters.org: the code that is '
                'compiled into the game builds or that the site sends with them. For each part it gives the '
                'version, the builds that use it, the copyright lines, the license (SPDX identifier) and the '
                'license text. Tools that only make the builds (compilers, bundlers, build scripts) are not in '
                'this list.'), '',
           para('The Ur-Quan Masters game code is free software under the GNU General Public License (GPL). See '
                'COPYING.txt on each UQM game page and the source repository:'),
           '    https://github.com/vcorem/uqm-3do-web', '',
           para('The game content (graphics, music, sound, speech and text) has its own licenses: see '
                'COPYING.txt, content-licenses.txt and melee-credits.txt on the game pages. Cosmo Clash is based '
                'on https://github.com/ohanegby/cosmoclash-web and keeps its models, textures and sounds (see '
                'cosmoclash/ in the source repository).'), '',
           field('Date:', date), field('Generator:', 'scripts/make-third-party-notices.py in the source repository'),
           '', '', 'Builds', thin]
    out += [f'{folder:<16} {label}' for folder, label in BUILDS]
    out += ['', '', 'Contents', thin]
    out += [para(f'{number:>2}. {part.name}' + (' ' + part.version if re.fullmatch(r'[\w.-]+', part.version) else ''),
                 subsequent_indent=' ' * 4) for number, part in enumerate(parts, 1)]
    out += ['    Code from other projects in the UQM program', '    License texts']
    out += [f'      {letter}. {shared[key].title}' for key, letter in letters.items()]
    for number, part in enumerate(parts, 1):
        out += ['', '', rule, para(f'{number}. {part.name}'), rule]
        out += [field('Version:', part.version), field('Used in:', ' '.join(part.builds)),
                field('Source:', part.source)]
        out += [field('Copyright:' if i == 0 else '', line) for i, line in enumerate(part.copyright)]
        out += [field('License:', part.license)]
        out += [line for note in part.notes for line in ('', para(note))]
        for heading, text in part.texts:
            if text in shared:
                out += ['', para(f'{heading}: see license text {letters[text]} at the end of this file.')]
            else:
                out += ['', f'--- {heading} ---', '', text]
    out += ['', '', rule, 'Code from other projects in the UQM program', rule, '',
            para('These files are part of the UQM program and have its GPL notices. Their license text is the GPL '
                 'in COPYING.txt on each UQM game page.')]
    out += [line for note in gpl_notes(src) for line in ('', para(note, initial_indent='* ', subsequent_indent='  '))]
    out += ['', '', rule, 'License texts', rule]
    for key, letter in letters.items():
        item = shared[key]
        out += ['', '', thin, f'{letter}. {item.title}', thin, field('From:', item.origin),
                field('Used by:', 'parts ' + ', '.join(users[key])), '', item.text]
    return '\n'.join(out) + '\n', len(parts), len(letters)


def iso_date(value):
    try:
        return datetime.date.fromisoformat(value).isoformat()
    except ValueError:
        raise argparse.ArgumentTypeError('expected a date as YYYY-MM-DD')


def main():
    tools = ROOT / '.tools'
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument('--date', required=True, type=iso_date, help='date for the header, YYYY-MM-DD')
    parser.add_argument('--output', type=Path, default=ROOT / 'web/editions/THIRD-PARTY-NOTICES.txt',
                        help='file to write (default: web/editions/THIRD-PARTY-NOTICES.txt)')
    parser.add_argument('--check', action='store_true', help='compare with --output and write nothing')
    # The defaults are where the build scripts keep these inputs (README.md, Build locally).
    parser.add_argument('--emsdk', type=Path, default=tools / 'emsdk',
                        help='Emscripten SDK (default: .tools/emsdk)')
    parser.add_argument('--hd-source', type=Path, default=tools / 'hd-source',
                        help='UQM-MegaMod checkout of scripts/build-hd-web.sh (default: .tools/hd-source)')
    parser.add_argument('--rustup-home', type=Path, default=Path(os.environ.get('RUSTUP_HOME', Path.home() / '.rustup')),
                        help='Rust toolchains (default: $RUSTUP_HOME, else ~/.rustup)')
    parser.add_argument('--cargo-home', type=Path, default=Path(os.environ.get('CARGO_HOME', Path.home() / '.cargo')),
                        help='cargo registry (default: $CARGO_HOME, else ~/.cargo)')
    parser.add_argument('--node-modules', type=Path, default=ROOT / 'cosmoclash/node_modules',
                        help='Cosmo Clash node_modules (default: cosmoclash/node_modules, after npm ci --prefix cosmoclash)')
    args = parser.parse_args()
    try:
        text, count, shared = notices(Sources(args), args.date)
    except NoticeError as error:
        parser.exit(1, 'make-third-party-notices.py: ' + str(error) + '\n')
    if args.check:
        current = args.output.read_text(encoding='utf-8') if args.output.is_file() else None
        if current != text:
            parser.exit(1, f'{args.output} is not up to date; run the script without --check\n')
        print(f'{args.output} is up to date ({count} parts)')
        return
    args.output.write_text(text, encoding='utf-8')
    print(f'Wrote {args.output}: {count} parts, {shared} shared license texts')


if __name__ == '__main__':
    main()
