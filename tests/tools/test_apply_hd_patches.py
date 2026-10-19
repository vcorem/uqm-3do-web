#!/usr/bin/env python3
"""HD patch staging regressions; --upstream also checks the real patch series."""

import argparse
import difflib
import importlib.util
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import unittest


ROOT = Path(__file__).resolve().parents[2]
SPEC = importlib.util.spec_from_file_location("apply_hd_patches", ROOT / "scripts/apply-hd-patches.py")
PATCHER = importlib.util.module_from_spec(SPEC)
sys.modules[SPEC.name] = PATCHER
SPEC.loader.exec_module(PATCHER)
UPSTREAM = None


def make_patch(name, before, after):
    return "".join(difflib.unified_diff(
        before.splitlines(keepends=True) if before is not None else [],
        after.splitlines(keepends=True) if after is not None else [],
        fromfile="a/" + name if before is not None else "/dev/null",
        tofile="b/" + name if after is not None else "/dev/null"))


def snapshot(directory):
    return {path.relative_to(directory): (path.read_bytes(), path.stat().st_mode)
            for path in directory.rglob("*") if path.is_file()}


class PatchTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory(prefix="uqm-patch-test-")
        self.addCleanup(self.temporary.cleanup)
        self.root = Path(self.temporary.name)
        self.source = self.root / "source"
        self.patches = self.root / "patches"
        self.source.mkdir()
        self.patches.mkdir()
        (self.source / "settings").write_text("before\n")
        (self.source / "settings").chmod(0o755)
        (self.source / "untouched").write_text("local data\n")
        (self.patches / "0001.patch").write_text(
            make_patch("settings", "before\n", "middle\n")
            + make_patch("added/header", None, "first header\n"))
        (self.patches / "0002.patch").write_text(
            make_patch("settings", "middle\n", "after\n")
            + make_patch("added/header", "first header\n", "final header\n"))

    def apply(self):
        return PATCHER.apply_patches(self.source, self.patches)

    def test_fresh_and_repeated_application_preserve_mode_and_unrelated_files(self):
        self.assertEqual(len(self.apply()), 2)
        self.assertEqual((self.source / "settings").read_text(), "after\n")
        self.assertEqual((self.source / "settings").stat().st_mode & 0o777, 0o755)
        self.assertEqual((self.source / "added/header").read_text(), "final header\n")
        before = snapshot(self.source)
        timestamps = {path: (self.source / path).stat().st_mtime_ns for path in before}
        self.assertEqual(self.apply(), [])
        self.assertEqual(snapshot(self.source), before)
        self.assertEqual({path: (self.source / path).stat().st_mtime_ns for path in before}, timestamps)
        self.assertEqual((self.source / "untouched").read_text(), "local data\n")

    def test_upgrades_previous_series(self):
        result = PATCHER.git_apply(self.source, self.patches / "0001.patch")
        self.assertEqual(result.returncode, 0, result.stderr)
        self.apply()
        self.assertEqual((self.source / "settings").read_text(), "after\n")

    def test_late_failure_does_not_publish_earlier_successful_changes(self):
        (self.patches / "0003.patch").write_text(make_patch("settings", "unexpected\n", "broken\n"))
        before = snapshot(self.source)
        with self.assertRaisesRegex(PATCHER.PatchError, "Cannot apply 0003.patch"):
            self.apply()
        self.assertEqual(snapshot(self.source), before)
        self.assertFalse((self.source / "added").exists())

    def test_conflicting_local_edit_is_preserved(self):
        (self.source / "settings").write_text("user edited this\n")
        before = snapshot(self.source)
        with self.assertRaises(PATCHER.PatchError):
            self.apply()
        self.assertEqual(snapshot(self.source), before)

    def test_symlink_parent_is_rejected_without_touching_target(self):
        outside = self.root / "outside"
        outside.mkdir()
        (outside / "header").write_text("private\n")
        (self.source / "added").symlink_to(outside, target_is_directory=True)
        before = (outside / "header").read_bytes()
        with self.assertRaisesRegex(PATCHER.PatchError, "symbolic link"):
            self.apply()
        self.assertEqual((outside / "header").read_bytes(), before)


class RealSeriesTests(unittest.TestCase):
    def setUp(self):
        if UPSTREAM is None:
            self.skipTest("pass --upstream /path/to/pinned/hd-source to check real patches")
        self.temporary = tempfile.TemporaryDirectory(prefix="uqm-real-patch-test-")
        self.addCleanup(self.temporary.cleanup)
        self.root = Path(self.temporary.name)
        self.patches = ROOT / "hd/patches"
        self.source = self.root / "source"
        self.source.mkdir()
        self.patch_list = sorted(self.patches.glob("*.patch"))
        paths = PATCHER.patch_paths(self.patch_list, self.source)
        revision = subprocess.check_output(["git", "-C", str(UPSTREAM), "rev-parse", "HEAD"], text=True).strip()
        self.assertEqual(revision, "1eece12ff9a89d029a5d1f65ead7c75f9ab78acb")
        for path in paths:
            result = subprocess.run(["git", "-C", str(UPSTREAM), "show", f"HEAD:{path}"], capture_output=True)
            if result.returncode == 0:
                target = self.source / path
                target.parent.mkdir(parents=True, exist_ok=True)
                target.write_bytes(result.stdout)
        (self.source / "local-save").write_bytes(b"keep me")

    def apply(self):
        return PATCHER.apply_patches(self.source, self.patches)

    def direct(self, patch):
        result = PATCHER.git_apply(self.source, patch)
        self.assertEqual(result.returncode, 0, result.stderr.decode(errors="replace"))

    def test_real_fresh_then_full_series_rerun(self):
        self.assertGreater(len(self.apply()), 0)
        before = snapshot(self.source)
        self.assertEqual(self.apply(), [])
        self.assertEqual(snapshot(self.source), before)
        self.assertIn(b"CHOICE_netplay_VALUE='web'", (self.source / "wasm/config.state.expected").read_bytes())
        self.assertTrue((self.source / "src/uqm/supermelee/netplay/webtransport.h").is_file())

    def test_real_legacy_crc_before_netplay_cache_upgrades_identically_to_fresh(self):
        fresh = self.root / "fresh"
        shutil.copytree(self.source, fresh)
        PATCHER.apply_patches(fresh, self.patches)
        expected = snapshot(fresh)
        for patch in self.patch_list:
            if patch.name < "0006":
                self.direct(patch)
        # This is the prior 0003 patch: same CRC replacement, before the
        # webtransport.h addition to the Makeinfo context in the current 0007.
        legacy = self.root / "legacy-crc.patch"
        legacy.write_text((self.patches / "0007-rust-netplay-crc.patch").read_text()
                          .replace(" webtransport.h", ""))
        self.direct(legacy)
        self.apply()
        upgraded = snapshot(self.source)
        self.assertEqual(upgraded, expected)
        self.assertEqual(self.apply(), [])
        self.assertEqual(snapshot(self.source), upgraded)
        self.assertEqual((self.source / "local-save").read_bytes(), b"keep me")

    def test_real_late_failure_preserves_all_source_files(self):
        self.apply()
        patches = self.root / "broken-patches"
        shutil.copytree(self.patches, patches)
        (patches / "9999-fail.patch").write_text(make_patch("Makeproject", "not upstream\n", "broken\n"))
        before = snapshot(self.source)
        with self.assertRaisesRegex(PATCHER.PatchError, "Cannot apply 9999-fail.patch"):
            PATCHER.apply_patches(self.source, patches)
        self.assertEqual(snapshot(self.source), before)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--upstream", type=Path)
    args, remaining = parser.parse_known_args()
    UPSTREAM = args.upstream.resolve() if args.upstream else None
    unittest.main(argv=[sys.argv[0], *remaining])
