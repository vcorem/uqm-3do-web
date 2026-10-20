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
shutil.copy2(source / 'wasm/compat/toolchain-fixes.json', output / 'toolchain-fixes.json')
# Page help (download progress, keys panel, How to start) is shared by every engine build.
shutil.copy2(root / 'web/game-help/game-help.js', output / 'game-help.js')
# The page starts the engine through this loader, which keeps the game data in the browser.
shutil.copy2(root / 'web/engine-loader/engine-loader.js', output / 'engine-loader.js')
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
(output / 'link-inputs.json').write_text(json.dumps(policy, indent=2) + '\n')
print('Linked Rust engine:', output / 'index.html')
