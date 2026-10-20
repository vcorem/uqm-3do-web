"""Focused regression checks for generated WebAssembly compatibility tools."""
import hashlib
import json
from pathlib import Path
import subprocess
import runpy
import sys
import tempfile
import unittest

HELPER = Path(__file__).resolve().with_name('asm-const-fix.py')
SOURCE = '''pub unsafe fn invoke() {
    let code = {
        #[link_section = "em_asm"]
        static mut x: [::core::ffi::c_char; 9] = unsafe {
            ::core::mem::transmute(*b"hello();\\0")
        };
        &raw const x as *const ::core::ffi::c_char
    };
}
'''


class AsmConstants(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory()
        self.addCleanup(self.temporary.cleanup)
        self.root = Path(self.temporary.name)
        (self.root / 'src').mkdir()
        (self.root / 'src/one.rs').write_text(SOURCE)

    def run_helper(self):
        return subprocess.run([sys.executable, str(HELPER), str(self.root)], capture_output=True, text=True)

    def snapshot(self):
        return {str(p.relative_to(self.root)):p.read_bytes() for p in self.root.rglob('*') if p.is_file()}

    def test_exact_bytes_and_repeatability(self):
        self.assertEqual(self.run_helper().returncode, 0)
        report = json.loads((self.root / 'asm-const-report.json').read_text())
        self.assertEqual(report[0]['sha256'], hashlib.sha256(b'hello();\0').hexdigest())
        first = self.snapshot()
        self.assertEqual(self.run_helper().returncode, 0)
        self.assertEqual(first, self.snapshot())

    def test_mixed_state_is_rejected_without_mutation(self):
        self.assertEqual(self.run_helper().returncode, 0)
        (self.root / 'src/two.rs').write_text(SOURCE)
        before = self.snapshot()
        result = self.run_helper()
        self.assertNotEqual(result.returncode, 0)
        self.assertIn('Mixed', result.stderr)
        self.assertEqual(before, self.snapshot())

    def test_corrupted_adapter_is_rejected(self):
        self.assertEqual(self.run_helper().returncode, 0)
        path = self.root / 'uqm_asm_consts.c'
        path.write_text(path.read_text().replace('104, 101', '105, 101'))
        result = self.run_helper()
        self.assertNotEqual(result.returncode, 0)
        self.assertIn('checksum', result.stderr)

    def test_bad_length_fails_before_writing_any_source(self):
        path = self.root / 'src/one.rs'
        path.write_text(SOURCE.replace('c_char; 9', 'c_char; 8'))
        before = self.snapshot()
        self.assertNotEqual(self.run_helper().returncode, 0)
        self.assertEqual(before, self.snapshot())


class SolarSortCorrection(unittest.TestCase):
    def test_repeatable_and_keeps_invalid_indices_checked(self):
        tools = Path(__file__).resolve().parent
        transform = runpy.run_path(str(tools / 'solar-sort-fix.py'))['transform']
        for edition in ['original', 'hd']:
            root = tools.parents[1] / 'rust' / ('uqm-engine-' + edition)
            text = (root / 'src/uqm/planets/solarsys.rs').read_text()
            self.assertEqual(transform(text, edition), text)
            self.assertIn('assert!(index >= 0, "invalid solar display index")', text)
            self.assertIn('PlanetDesc[index as usize]', text)
            self.assertNotIn('PlanetDesc[real_i as usize]', text)

    def test_mixed_correction_state_is_rejected(self):
        tools = Path(__file__).resolve().parent
        transform = runpy.run_path(str(tools / 'solar-sort-fix.py'))['transform']
        with self.assertRaisesRegex(ValueError, 'mixed'):
            transform('unsafe fn solarDisplayBody() {} PlanetDesc[real_i as usize]', 'original')


class LinkPolicy(unittest.TestCase):
    def setUp(self):
        tools = Path(__file__).resolve().parent
        self.validate = runpy.run_path(str(tools / 'link-policy.py'))['validate']
        self.root = tools.parents[1]
        self.source = self.root / 'sc2'
        self.generated = self.root / 'rust/uqm-engine-original'
        self.library = self.root / '.tools/libuqm_engine_original.a'
        self.adapters = [self.generated / 'uqm_rust_entry.c', self.generated / 'uqm_asm_consts.c',
                         self.root / 'rust/uqm-png/png-guard.c']

    def check(self, adapters=None, flags=None):
        return self.validate(self.root, 'original', self.source, self.generated, self.library,
                             self.adapters if adapters is None else adapters,
                             ['-lpng-mt', 'wasm/compat/emscripten-proxying.o'] if flags is None else flags)

    def test_only_explicit_platform_adapters_are_allowed(self):
        result = self.check()
        self.assertEqual(result['legacy_c_game_objects'], [])
        self.assertTrue(result['emscripten_compatibility_object'])

    def test_game_source_cannot_be_disguised_as_adapter(self):
        with self.assertRaisesRegex(ValueError, 'forbidden'):
            self.check(adapters=self.adapters + [self.source / 'src/uqm/comm.c'])

    def test_object_archive_and_response_file_inputs_are_rejected(self):
        for value in ['obj/debug-wasm/src/uqm/comm.c.o', '/tmp/game.a',
                      '-Wl,/tmp/game.bc', '@/tmp/response', '-Wl,@/tmp/response',
                      '-llegacy_game', '-L/tmp/game', '/tmp/game.so']:
            with self.subTest(value=value), self.assertRaises(ValueError):
                self.check(flags=[value])


class ReproductionPins(unittest.TestCase):
    def setUp(self):
        self.tools = Path(__file__).resolve().parent
        self.checks = runpy.run_path(str(self.tools / 'verify-provenance.py'))

    def test_actual_patch_and_pipeline_hashes(self):
        provenance = self.checks['verify'](self.tools)
        self.assertIn('scripts/rust-engine/c2rust-wasm.patch', provenance['reproduction_sha256'])
        self.assertIn('scripts/rust-engine/package.py', provenance['reproduction_sha256'])

    def test_versions_reject_compatible_but_unpinned_toolchains(self):
        pins = {'emscripten_version': '3.1.50', 'llvm_version': '18.1.8'}
        emcc = 'emcc (Emscripten gcc/clang-like replacement + linker emulating GNU ld) 3.1.50 (revision)'
        clang = 'Homebrew clang version 18.1.8\nTarget: arm64-apple-darwin'
        self.checks['verify_versions'](emcc, clang, pins)
        with self.assertRaisesRegex(ValueError, 'Emscripten'):
            self.checks['verify_versions'](emcc.replace('3.1.50', '3.1.51'), clang, pins)
        with self.assertRaisesRegex(ValueError, 'Clang'):
            self.checks['verify_versions'](emcc, clang.replace('18.1.8', '19.1.0'), pins)

    def test_changed_patch_or_nonportable_path_is_rejected(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            tools = root / 'scripts/rust-engine'
            tools.mkdir(parents=True)
            patch = tools / 'patch'
            patch.write_bytes(b'reviewed patch')
            provenance = {'translate_const_macros': 'none', 'translate_fn_macros': 'none',
                          'reproduction_sha256': {'scripts/rust-engine/patch': hashlib.sha256(patch.read_bytes()).hexdigest()}}
            file = tools / 'provenance.json'
            file.write_text(json.dumps(provenance))
            self.checks['verify'](tools)
            patch.write_bytes(b'changed patch')
            with self.assertRaisesRegex(ValueError, 'changed'):
                self.checks['verify'](tools)
            for path in ['/tmp/patch', '../patch', '.tools/patch']:
                provenance['reproduction_sha256'] = {path: '0' * 64}
                file.write_text(json.dumps(provenance))
                with self.subTest(path=path), self.assertRaisesRegex(ValueError, 'Nonportable'):
                    self.checks['verify'](tools)


class NoticeWhitespace(unittest.TestCase):
    def test_notice_cleanup_does_not_touch_multiline_string_bytes(self):
        helper = runpy.run_path(str(Path(__file__).resolve().with_name('package.py')))
        prefix = '/* License  \n * preserved text\t\n */\n\n'
        code = 'static TEXT: &str = "line with spaces  \nnext line\t\n";\n'
        normalized = helper['normalize_leading_comment_whitespace'](prefix + code)
        self.assertEqual(normalized, '/* License\n * preserved text\n */\n\n' + code)
        self.assertEqual(helper['normalize_leading_comment_whitespace'](code), code)


if __name__ == '__main__':
    unittest.main()
