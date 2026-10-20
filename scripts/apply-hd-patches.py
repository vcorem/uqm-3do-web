#!/usr/bin/env python3
"""Apply the HD patch series without disturbing source files on patch failure.

Later patches can change the context of earlier ones. Check applicability on a
private copy, first removing recognizable patches in reverse order and then
applying the current series. This also upgrades the legacy cache whose CRC
patch preceded web netplay: the new netplay patch restores the CRC patch's
current context, allowing its already-applied change to be recognized.
"""

from __future__ import annotations

import argparse
from dataclasses import dataclass
import os
from pathlib import Path, PurePosixPath
import stat
import subprocess
import tempfile


class PatchError(RuntimeError):
    pass


@dataclass(frozen=True)
class FileState:
    data: bytes | None
    mode: int = 0


def git_apply(directory, patch, *options):
    # The staging directory has no repository. An inherited Git override must
    # not redirect apply into the user's real index or worktree.
    env = {key: value for key, value in os.environ.items()
           if key not in {"GIT_DIR", "GIT_WORK_TREE", "GIT_INDEX_FILE"}}
    return subprocess.run(
        ["git", "-C", str(directory), "apply", *options, str(patch)],
        capture_output=True, env=env, check=False)


def patch_paths(patches, staging_directory):
    paths = set()
    for patch in patches:
        # This series changes ordinary files only. Reject rename/copy patches
        # rather than accidentally omitting their old path from the transaction.
        if any(line.startswith((b"rename from ", b"rename to ", b"copy from ", b"copy to "))
               for line in patch.read_bytes().splitlines()):
            raise PatchError(f"Rename/copy patches are unsupported: {patch.name}")
        result = git_apply(staging_directory, patch, "--numstat", "-z")
        if result.returncode:
            raise PatchError(f"Cannot read {patch.name}: {result.stderr.decode(errors='replace').strip()}")
        for record in result.stdout.split(b"\0"):
            if not record:
                continue
            fields = record.split(b"\t", 2)
            if len(fields) != 3:
                raise PatchError(f"Unsupported path record in {patch.name}")
            name = fields[2].decode("utf-8")
            path = PurePosixPath(name)
            if (not name or path.is_absolute() or ".." in path.parts
                    or ".git" in path.parts or "\\" in name or str(path) != name):
                raise PatchError(f"Unsafe patch path in {patch.name}: {name!r}")
            paths.add(path)
    if not paths:
        raise PatchError("The patch series contains no files")
    return sorted(paths)


def read_state(root, relative):
    path = root / relative
    for component in (path, *path.parents):
        if component == root:
            break
        if component.is_symlink():
            raise PatchError(f"Refusing a symbolic link in patch path: {path}")
    if not path.exists():
        return FileState(None)
    info = path.stat()
    if not stat.S_ISREG(info.st_mode):
        raise PatchError(f"Patch target is not a regular file: {path}")
    return FileState(path.read_bytes(), stat.S_IMODE(info.st_mode))


def write_state(path, state):
    if state.data is None:
        path.unlink(missing_ok=True)
        return
    path.parent.mkdir(parents=True, exist_ok=True)
    temporary = None
    try:
        with tempfile.NamedTemporaryFile(prefix=".uqm-patch-", dir=path.parent, delete=False) as stream:
            temporary = Path(stream.name)
            stream.write(state.data)
        temporary.chmod(state.mode)
        os.replace(temporary, path)
    finally:
        if temporary is not None:
            temporary.unlink(missing_ok=True)


def apply_patches(source, patch_directory):
    source = Path(source).resolve()
    patch_directory = Path(patch_directory).resolve()
    if not source.is_dir():
        raise PatchError(f"HD source directory does not exist: {source}")
    patches = sorted(patch_directory.glob("*.patch"))
    if not patches:
        raise PatchError(f"No patches found in {patch_directory}")
    with tempfile.TemporaryDirectory(prefix="uqm-hd-patches-") as temporary:
        staged = Path(temporary)
        paths = patch_paths(patches, staged)
        original = {path: read_state(source, path) for path in paths}
        for path, state in original.items():
            write_state(staged / path, state)
        for patch in reversed(patches):
            if git_apply(staged, patch, "--reverse", "--check").returncode == 0:
                result = git_apply(staged, patch, "--reverse")
                if result.returncode:
                    raise PatchError(f"Cannot reverse {patch.name}: {result.stderr.decode(errors='replace').strip()}")
        for patch in patches:
            if git_apply(staged, patch, "--reverse", "--check").returncode == 0:
                continue
            result = git_apply(staged, patch, "--check")
            if result.returncode:
                raise PatchError(f"Cannot apply {patch.name}; source files were not changed:\n"
                                 f"{result.stderr.decode(errors='replace').strip()}")
            result = git_apply(staged, patch)
            if result.returncode:
                raise PatchError(f"Cannot apply {patch.name}: {result.stderr.decode(errors='replace').strip()}")
        updated = {path: read_state(staged, path) for path in paths}
        # Avoid clobbering edits made while the patch checks were running.
        for path, state in original.items():
            if read_state(source, path) != state:
                raise PatchError(f"HD source changed during patch validation: {path}; no files were published")
        changed = [path for path in paths if original[path] != updated[path]]
        published = []
        try:
            for path in changed:
                write_state(source / path, updated[path])
                published.append(path)
        except OSError:
            for path in reversed(published):
                write_state(source / path, original[path])
            raise
    return changed


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("source", type=Path)
    parser.add_argument("--patch-dir", type=Path,
                        default=Path(__file__).resolve().parent.parent / "hd" / "patches")
    args = parser.parse_args()
    try:
        changed = apply_patches(args.source, args.patch_dir)
    except (PatchError, OSError, UnicodeError) as error:
        parser.exit(1, f"HD patch update failed: {error}\n")
    print(f"HD patch series verified ({len(changed)} source files updated).")


if __name__ == "__main__":
    main()
