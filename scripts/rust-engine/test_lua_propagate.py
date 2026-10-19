#!/usr/bin/env python3
"""Check Lua call propagation, typed returns and fail-closed source rewriting."""
import argparse
import json
from pathlib import Path
import subprocess
import tempfile

APIS = '''luaL_checkinteger luaL_checklstring luaL_checkoption luaL_checktype luaL_error luaL_loadstring luaL_newstate luaL_setfuncs lua_absindex lua_callk lua_close lua_createtable lua_getfield lua_getglobal lua_gettable lua_gettop lua_load lua_pcallk lua_pushboolean lua_pushcclosure lua_pushinteger lua_pushlightuserdata lua_pushnil lua_pushstring lua_pushvalue lua_rawgeti lua_rawseti lua_replace lua_setfield lua_settable lua_settop lua_toboolean lua_tointegerx lua_tolstring lua_tonumberx lua_topointer lua_type lua_typename'''.split()


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('binary', type=Path)
    binary = parser.parse_args().binary.resolve()
    with tempfile.TemporaryDirectory(prefix='uqm-lua-propagate-') as temporary:
        base = Path(temporary)
        source = base / 'src/uqm/lua/test.rs'
        source.parent.mkdir(parents=True)
        declarations = 'extern "C" {\n' + ''.join(f'fn {name}();\n' for name in APIS) + '}\n'
        original = declarations + '''
#[repr(transparent)] struct Boolean(u32);
#[repr(C)] struct Pair { flag: Boolean, pointer: *const u8 }
unsafe fn leaf() -> u32 { let label = "é"; lua_gettop(); 42 }
unsafe fn parent() -> u32 { leaf() + 3 }
unsafe fn callback(f: unsafe extern "C" fn()) -> bool { f(); true }
unsafe fn composite() -> Pair { leaf(); Pair { flag: Boolean(9), pointer: core::ptr::null() } }
fn pure() -> u32 { 9 }
fn vectors() -> Vec<u8> { Vec::new() }
'''
        source.write_text(original)
        (base / 'lib.rs').write_text('pub mod src {}\n')
        subprocess.run([str(binary), str(base), '--apply'], check=True)
        actual = source.read_text()
        assert actual.count('#[link_name = "uqm_lua_guard_') == len(APIS)
        assert 'uqm_lua_checked!(lua_gettop(), 0)' in actual
        assert 'uqm_lua_checked!(leaf(), 0) + 3' in actual
        assert 'uqm_lua_checked!(f(), false)' in actual
        assert 'Pair { flag: Boolean(0), pointer: ::core::ptr::null() }' in actual
        assert 'fn pure() -> u32 { 9 }' in actual
        assert 'fn vectors() -> Vec<u8> { Vec::new() }' in actual
        assert actual.count('uqm_lua_checked!') == 4
        assert '#[path = "../uqm-lua/guard.rs"]' in (base / 'lib.rs').read_text()
        result = subprocess.run([str(binary), str(base), '--apply'], capture_output=True)
        assert result.returncode != 0
        assert source.read_text() == actual
        subprocess.run([str(binary), str(base), '--audit'], check=True)
        # A modified count is not evidence: recompute the actual source graph.
        source.write_text(actual + '\nunsafe fn newcomer() -> u32 { uqm_lua_checked!(lua_gettop(), 0); 7 }\nunsafe fn outsider() -> u32 { self::newcomer() }\n')
        report = base / 'lua-propagation-report.json'
        data = json.loads(report.read_text())
        data['checked_calls'] += 1
        report.write_text(json.dumps(data))
        result = subprocess.run([str(binary), str(base), '--audit'], capture_output=True, text=True)
        assert result.returncode != 0 and 'unchecked propagating call' in result.stderr, result.stderr

        def rejects(body, expected):
            (base / 'lua-propagation-report.json').unlink(missing_ok=True)
            (base / 'lib.rs').write_text('pub mod src {}\n')
            text = declarations + body
            source.write_text(text)
            result = subprocess.run([str(binary), str(base), '--apply'], capture_output=True, text=True)
            assert result.returncode != 0, result.stdout
            assert expected in result.stderr, result.stderr
            assert source.read_text() == text, 'failed validation must not modify source'
            assert (base / 'lib.rs').read_text() == 'pub mod src {}\n'

        rejects("unsafe fn bad() -> &'static u8 { lua_gettop(); &0 }", 'unsupported/nonzero/reference')
        rejects('enum Choice { A } unsafe fn bad() -> Choice { lua_gettop(); Choice::A }', 'unresolved return type')
        rejects('extern "C" { fn lua_new_unsupported_api(); }', 'unknown Lua API')
        rejects('unsafe fn bad() -> u32 { let run = || { lua_gettop(); 1 }; run() }', 'unsupported closure scope')
    print('Lua propagation source tests passed: calls, indirect callbacks, explicit aggregates, Unicode spans, duplicate and unsafe-type rejection.')


if __name__ == '__main__':
    main()
