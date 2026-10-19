#!/usr/bin/env python3
"""Fetch pinned official HD content and stage its original package layout.

Binary assets remain in .tools; hd/content-lock.json records provenance and
mixed license notices. A pinned upstream ANI fix is staged as a loose overlay;
the downloaded archives and classic content tree are never modified.
Use --offline to require existing verified cache/local copies.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import os
from pathlib import Path, PurePosixPath
import shutil
import tempfile
import urllib.request
import zipfile

ROOT = Path(__file__).resolve().parents[1]
LOCK = ROOT / "hd" / "content-lock.json"


def relative_path(value: str) -> Path:
    """Reject paths that could escape the managed cache/runtime directories."""
    path = PurePosixPath(value)
    if not value or path.is_absolute() or ".." in path.parts or "\\" in value:
        raise ValueError(f"Unsafe manifest path: {value!r}")
    return Path(*path.parts)


def verify_file(path: Path, record: dict) -> None:
    if path.stat().st_size != record["bytes"]:
        raise ValueError(f"Size mismatch for {path}; expected {record['bytes']} bytes")
    sha256, md5 = hashlib.sha256(), hashlib.md5()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            sha256.update(chunk)
            md5.update(chunk)
    if sha256.hexdigest() != record["sha256"]:
        raise ValueError(f"SHA-256 mismatch for {path}; refusing unverified content")
    if record.get("upstream_installer_md5") not in (None, md5.hexdigest()):
        raise ValueError(f"Upstream installer checksum mismatch for {path}")


def atomic_copy(source: Path, destination: Path) -> None:
    destination.parent.mkdir(parents=True, exist_ok=True)
    fd, temporary = tempfile.mkstemp(prefix=destination.name + ".", dir=destination.parent)
    os.close(fd)
    try:
        shutil.copyfile(source, temporary)
        os.replace(temporary, destination)
    finally:
        Path(temporary).unlink(missing_ok=True)


def fetch(record: dict, cache: Path, offline: bool) -> Path:
    destination = cache / relative_path(record.get("cache_path", record["name"]))
    if destination.exists():
        verify_file(destination, record)
        return destination
    # Reuse only byte-identical local upstream files or the already-downloaded
    # original movie archive. Nothing in the classic runtime is changed.
    for source_name in record.get("local_sources", []):
        source = ROOT / relative_path(source_name)
        if not source.is_file():
            continue
        try:
            verify_file(source, record)
        except ValueError:
            continue
        atomic_copy(source, destination)
        return destination
    if offline:
        raise FileNotFoundError(f"Offline cache is missing {destination}")
    destination.parent.mkdir(parents=True, exist_ok=True)
    fd, temporary = tempfile.mkstemp(prefix=destination.name + ".download-", dir=destination.parent)
    try:
        request = urllib.request.Request(record["url"], headers={"User-Agent": "uqm-3do-web-content/1"})
        print(f"Downloading {record['name']} ({record['bytes']:,} bytes)", flush=True)
        with os.fdopen(fd, "wb") as stream:
            with urllib.request.urlopen(request, timeout=60) as response:
                received = 0
                while chunk := response.read(1024 * 1024):
                    received += len(chunk)
                    if received > record["bytes"]:
                        raise ValueError(f"Download exceeds locked size: {record['name']}")
                    stream.write(chunk)
                final_url = response.url
        verify_file(Path(temporary), record)
        os.replace(temporary, destination)
        destination.with_name(destination.name + ".download.json").write_text(
            json.dumps({"source": record["url"], "effective_url": final_url}, indent=2) + "\n",
            encoding="utf-8",
        )
    finally:
        Path(temporary).unlink(missing_ok=True)
    return destination


def verify_archive(path: Path, record: dict) -> None:
    with zipfile.ZipFile(path) as archive:
        names = archive.namelist()
        if len(names) != len(set(names)):
            raise ValueError(f"Duplicate archive paths in {path}")
        for info in archive.infolist():
            relative_path(info.filename)
            if (info.external_attr >> 16) & 0o170000 == 0o120000:
                raise ValueError(f"Unexpected archive symlink: {info.filename}")
        for name in record["required_entries"] + record["preserve_metadata"]:
            if name not in names:
                raise ValueError(f"Missing required member {name!r} in {path}")
        failed = archive.testzip()
        if failed:
            raise ValueError(f"ZIP CRC failure in {path}: {failed}")


def verify_overlay(source: Path, record: dict, packages: dict[str, Path]) -> None:
    """Limit the upstream compatibility overlay to its documented text change."""
    overlay = record.get("overlay")
    if not overlay:
        return
    with zipfile.ZipFile(packages[overlay["package"]]) as archive:
        original = archive.read(overlay["member"])
    if hashlib.sha256(original).hexdigest() != overlay["original_sha256"]:
        raise ValueError(f"Overlay base differs from pinned archive: {record['name']}")
    suffix = b"\n" + overlay["remove_final_line"].encode("utf-8")
    if not original.endswith(suffix) or source.read_bytes() != original[:-len(suffix)]:
        raise ValueError(f"Unexpected change in upstream overlay: {record['name']}")


def stage(source: Path, output: Path, record: dict) -> None:
    destination = output / relative_path(record["install_path"])
    try:
        verify_file(destination, record)
    except (FileNotFoundError, ValueError):
        atomic_copy(source, destination)
    if "preserve_metadata" in record:
        with zipfile.ZipFile(source) as archive:
            for member in record["preserve_metadata"]:
                target = output / "notices" / record["name"] / relative_path(member)
                target.parent.mkdir(parents=True, exist_ok=True)
                target.write_bytes(archive.read(member))



def write_notices(manifest: dict, output: Path) -> None:
    """Produce a distributable text index without replacing original notices."""
    sections = [
        "THE UR-QUAN MASTERS HD MEGAMOD CONTENT: PROVENANCE AND NOTICES",
        "Upstream release: " + manifest["upstream"]["release"],
        "Source commit: " + manifest["upstream"]["source_commit"],
        "Content repository commit: " + manifest["upstream"]["content_commit"],
        manifest["upstream"]["content_license"],
        manifest["scope"],
        "The texts below are an index/aggregation. Original notice bytes are "
        "preserved under notices/. This project does not override upstream or third-party licenses. "
        "See the explicit custom/Unknown/None? entries in ATTRIBUTION.txt.",
    ]
    for record in manifest["packages"]:
        sections.extend([
            "=" * 72,
            record["name"],
            "Official source: " + record["source_page"],
            "SHA-256: " + record["sha256"],
            record["license_record"],
        ])
        for member in record["preserve_metadata"]:
            relative = Path("notices") / record["name"] / relative_path(member)
            sections.extend([str(relative), (output / relative).read_text(encoding="utf-8", errors="replace")])
    for record in manifest["source_files"]:
        if record.get("overlay"):
            sections.extend([
                "=" * 72, "UPSTREAM CONTENT FIX: " + record["install_path"],
                "Pinned source: " + record["url"], "SHA-256: " + record["sha256"],
                "Upstream change: " + record["overlay"]["upstream_commit"],
                record["overlay"]["reason"],
            ])
        if record["install_path"].startswith("notices/"):
            sections.extend([
                "=" * 72, record["install_path"], "Pinned source: " + record["url"],
                (output / record["install_path"]).read_text(encoding="utf-8", errors="replace"),
            ])
    (output / "content-licenses.txt").write_text("\n\n".join(sections) + "\n", encoding="utf-8")


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--cache", type=Path, default=ROOT / ".tools" / "hd-assets")
    parser.add_argument("--output", type=Path, default=ROOT / ".tools" / "hd-content")
    parser.add_argument("--offline", action="store_true", help="Use verified cached/local files only")
    args = parser.parse_args()
    manifest = json.loads(LOCK.read_text(encoding="utf-8"))
    if manifest["schema_version"] != 1:
        raise ValueError("Unsupported HD content lock schema")
    cache, output = args.cache.resolve(), args.output.resolve()
    expected_packages = {record["install_path"] for record in manifest["packages"]}
    # Mixing unrelated MegaMod content versions is unsupported. Refuse stale
    # archives without deleting anything the caller may want to preserve.
    for folder in ("packages", "addons"):
        for existing in (output / folder).glob("*.uqm"):
            if existing.relative_to(output).as_posix() not in expected_packages:
                raise ValueError(f"Unexpected runtime package {existing}; use a clean --output directory")
    verified = []
    for record in manifest["packages"]:
        source = fetch(record, cache, args.offline)
        verify_archive(source, record)
        verified.append((source, record))
        print(f"Verified {record['name']}: SHA-256, installer checksum and ZIP CRC", flush=True)
    packages = {record["name"]: source for source, record in verified}
    for record in manifest["source_files"]:
        source = fetch(record, cache, args.offline)
        verify_overlay(source, record, packages)
        verified.append((source, record))
    # Verify every input before updating the managed runtime paths.
    for source, record in verified:
        stage(source, output, record)
    output.mkdir(parents=True, exist_ok=True)
    atomic_copy(LOCK, output / "content-lock.json")
    write_notices(manifest, output)
    print(f"HD content ready: {output}")
    print(f"Original package bytes: {sum(record['bytes'] for record in manifest['packages']):,}")
    print("Package attribution and upstream notices preserved under notices/.")


if __name__ == "__main__":
    try:
        main()
    except (OSError, ValueError, zipfile.BadZipFile) as error:
        raise SystemExit(f"HD content preparation failed: {error}") from error
