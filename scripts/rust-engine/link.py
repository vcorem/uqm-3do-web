"""Link the whole Rust engine against the pinned browser runtime.

Only the entry point and explicit codec/platform adapters are compiled from C.
The legacy game's C object tree is never an input to this link.
"""
from pathlib import Path
import argparse
import json
import re
import runpy
import shlex
import shutil
import subprocess
import tempfile

root = Path(__file__).resolve().parents[2]
parser = argparse.ArgumentParser()
parser.add_argument('edition', choices=['original', 'hd'])
parser.add_argument('--library', type=Path, required=True)
parser.add_argument('--generated', type=Path, required=True)
parser.add_argument('--output', type=Path, required=True)
parser.add_argument('--source', type=Path, required=True)
parser.add_argument('--campaign-combat', action='store_true')
parser.add_argument('--campaign-test-driver', action='store_true')
args = parser.parse_args()
source = args.source.resolve()
generated = args.generated.resolve()
library = args.library.resolve()
output = args.output.resolve()
output.mkdir(parents=True, exist_ok=True)
build = (source / 'build.vars').read_text()
def flags(name):
    return shlex.split(re.search(r"^" + name + r"='(.*)'$", build, re.M)[1])
compile_flags = flags('uqm_CFLAGS') + ['-Isrc']
if "uqm_HAVE_REGEX='0'" in build:
    compile_flags += ['-Isrc/regex']
link_flags = [value for value in flags('uqm_LDFLAGS')
              if not value.endswith('/libuqm_core.a')]
campaign_content = None
if args.campaign_combat:
    if args.edition != 'original':
        raise SystemExit('Afterlight combat currently uses the Original Rust engine.')
    for option, replacement in [('--shell-file', root / 'campaign/combat-shell/index.html'),
                                ('--pre-js', root / 'campaign/combat-shell/pre.js')]:
        link_flags[link_flags.index(option) + 1] = str(replacement)
    # Combat requires the base ship/UI assets, not the original campaign's
    # enormous voice and FMV add-ons. Keep the source media tree untouched.
    campaign_content = tempfile.TemporaryDirectory(prefix='uqm-afterlight-content-')
    combat_content = Path(campaign_content.name)
    shutil.copytree(source / 'content/base', combat_content / 'base')
    (combat_content / 'addons').mkdir()
    for name in ('uqm.rmp', 'uqm.key', 'menu.key', 'version'):
        shutil.copy2(source / 'content' / name, combat_content / name)
    link_flags[link_flags.index('--preload-file=content')] = '--preload-file=' + str(combat_content) + '@/content'
    exports = ['_main'] + ['_uqm_campaign_' + suffix for suffix in
        ('enable', 'begin', 'phase', 'outcome', 'crew', 'remaining', 'frames', 'retreat')]
    if args.campaign_test_driver:
        exports.append('_uqm_campaign_test_ai')
    link_flags.append('-sEXPORTED_FUNCTIONS=' + json.dumps(exports, separators=(',', ':')))
elif args.campaign_test_driver:
    raise SystemExit('The test driver requires the isolated campaign combat build.')
emcc = shutil.which('emcc')
if not emcc:
    raise SystemExit('Emscripten must be on PATH.')
adapters = [generated / 'uqm_rust_entry.c', root / 'rust/uqm-png/png-guard.c']
if (generated / 'uqm_asm_consts.c').is_file():
    adapters.append(generated / 'uqm_asm_consts.c')
if args.edition == 'hd':
    # Bundled Lua is an explicit third-party runtime dependency, like SDL and
    # libpng, and retains its original protected longjmp error machinery.
    lua = source / 'src/libs/lua'
    compile_flags += ['-I' + str(lua)]
    adapters.append(root / 'rust/uqm-lua/lua-guard.c')
    lua_files = shlex.split(re.search(r'uqm_CFILES="([^"]+)"', (lua / 'Makeinfo').read_text())[1])
    adapters += [lua / name for name in lua_files]
policy = runpy.run_path(str(root / 'scripts/rust-engine/link-policy.py'))['validate'](
    root, args.edition, source, generated, library, adapters, link_flags)
temporary = tempfile.TemporaryDirectory(prefix='uqm-rust-link-')
adapter_output = Path(temporary.name)
objects = []
for index, adapter in enumerate(adapters):
    target = adapter_output / ('adapter-' + str(index) + '.o')
    subprocess.run([str(emcc), '-c', *compile_flags, str(adapter), '-o', str(target)], cwd=source, check=True)
    objects.append(target)
subprocess.run([str(emcc), *map(str, objects), str(library), *link_flags,
                '-o', str(output / 'uqm-debug.html')], cwd=source, check=True)
shutil.copy2(output / 'uqm-debug.html', output / 'index.html')
if args.campaign_combat:
    shutil.copy2(root / 'campaign/combat-shell/host.js', output / 'host.js')
shutil.copy2(source / 'wasm/compat/toolchain-fixes.json', output / 'toolchain-fixes.json')
if args.edition == 'original':
    shutil.copy2(source / 'COPYING', output / 'COPYING.txt')
    shutil.copy2(source / 'wasm/netplay-client.js', output / 'netplay-client.js')
else:
    shutil.copy2(source / 'content/notices/upstream/licenses/COPYING-UQM', output / 'COPYING.txt')
    shutil.copy2(source / 'content/content-licenses.txt', output / 'content-licenses.txt')
    shutil.copy2(root / 'hd/content-lock.json', output / 'content-lock.json')
    shutil.copy2(root / 'sc2/wasm/netplay-client.js', output / 'netplay-client.js')
(output / 'engine-language.json').write_text(json.dumps({
    'core': 'rust', 'edition': args.edition,
    'game_engine': 'rust', 'implementation': 'translated unsafe Rust plus verified Rust core',
    'c_adapters': ['main entry', 'Emscripten JavaScript string data', 'Emscripten proxy compatibility', 'libpng setjmp boundary'] +
                  (['Lua protected operations and callback trampolines'] if args.edition == 'hd' else []),
    'native_dependencies': ['SDL2', 'OpenAL', 'libpng', 'zlib', 'Ogg/Vorbis'] +
                           (['Lua'] if args.edition == 'hd' else []),
    'browser_bindings': 'JavaScript/Emscripten',
    'rust_toolchain': (root / '.rust-wasm-toolchain').read_text().strip(),
    'emscripten': (root / '.emscripten-version').read_text().strip(),
}, indent=2) + '\n')
temporary.cleanup()
if campaign_content:
    campaign_content.cleanup()
(output / 'link-inputs.json').write_text(json.dumps(policy, indent=2) + '\n')
print('Linked Rust engine:', output / 'index.html')
