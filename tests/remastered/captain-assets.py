#!/usr/bin/env python3
"""Verify every captain frame against the pinned native resource archive.

This validates real authored output, not synthetic fixtures. It deliberately
parses the original ANI files independently of the captain compiler.
"""
from __future__ import annotations

import argparse
import hashlib
import io
import json
from pathlib import Path
import zipfile

from PIL import Image, ImageChops

ROOT = Path(__file__).resolve().parents[2]


def sha(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def changed_pixels(first, second):
    return sum(max(pixel) > 8 for pixel in ImageChops.difference(first, second).convert("RGB").getdata())


def check_compositions(ship, originals, replacements):
    """Replay incremental native stamping, including transition and erase rows.

    Starting every pose on a fresh background hides persistent seams. Original
    PNGs provide a control for any pre-existing native edge/alpha imperfections.
    """
    def stamp(canvas, images, rows):
        for index in rows:
            frame = ship["frames"][index]
            canvas.alpha_composite(images[frame["filename"]], tuple(frame["position_in_portrait"]))

    def neutral(images):
        image = images[ship["frames"][0]["filename"]].copy()
        stamp(image, images, [2, 3, 6, 9, 12])
        return image

    phases = {
        "left": ([2, 3, 4, 2, 3, 4, 5], [2, 3, 4, 2, 3], [2, 3, 4, 5]),
        "right": ([4, 3, 2, 4, 3, 2, 1], [4, 3, 2, 4, 3], [4, 3, 2, 1]),
        "thrust": ([7, 8], [7, 6], [8]),
        "weapon": ([10, 11], [10, 9], [11]),
        "combined": ([2, 3, 4, 7, 10, 2, 3, 4, 5, 8, 11], [2, 3, 4, 7, 10, 2, 3, 6, 9], [2, 3, 4, 5, 8, 11]),
    }
    if ship["race"] != "shofixti":
        phases["special"] = ([13, 14], [13, 12], [14])
    native_neutral, rendered_neutral = neutral(originals), neutral(replacements)
    results = []
    for name, (held_rows, released_rows, refreshed_rows) in phases.items():
        native, rendered = native_neutral.copy(), rendered_neutral.copy()
        stamp(native, originals, held_rows)
        stamp(rendered, replacements, held_rows)
        native_motion = changed_pixels(native_neutral, native)
        rendered_motion = changed_pixels(rendered_neutral, rendered)
        # A style toggle repaints the background and the current nonzero
        # offsets. Neutral patches must not erase overlapping active layers.
        native_refresh = originals[ship["frames"][0]["filename"]].copy()
        rendered_refresh = replacements[ship["frames"][0]["filename"]].copy()
        stamp(native_refresh, originals, refreshed_rows)
        stamp(rendered_refresh, replacements, refreshed_rows)
        native_refresh_delta = changed_pixels(native, native_refresh)
        rendered_refresh_delta = changed_pixels(rendered, rendered_refresh)
        stamp(native, originals, released_rows)
        stamp(rendered, replacements, released_rows)
        native_residue = changed_pixels(native_neutral, native)
        rendered_residue = changed_pixels(rendered_neutral, rendered)
        results.append({"phase": name, "native_motion_pixels": native_motion,
            "rendered_motion_pixels": rendered_motion, "native_residual_pixels": native_residue,
            "rendered_residual_pixels": rendered_residue,
            "native_refresh_delta_pixels": native_refresh_delta,
            "rendered_refresh_delta_pixels": rendered_refresh_delta})
        if native_motion >= 4:
            assert rendered_motion >= 4, f"{ship['race']} {name}: authored reaction is visually unchanged"
        assert rendered_residue <= native_residue + 2, (
            f"{ship['race']} {name}: incremental release leaves {rendered_residue} changed pixels "
            f"(native control {native_residue}); inspect neutral/erasure patches")
        assert rendered_refresh_delta <= native_refresh_delta + 2, (
            f"{ship['race']} {name}: style refresh changed {rendered_refresh_delta} active pose pixels "
            f"(native control {native_refresh_delta})")
    if ship["race"] == "shofixti":
        # shofixti_preprocess keeps the lever latched when SPECIAL is released.
        # The third press reaches offset5/row17 and detonates before offset6;
        # PostProcessStatus then draws row19 (row18 is an unused PNG alias).
        native, rendered = native_neutral.copy(), rendered_neutral.copy()
        for name, rows, final_row in (
            ("glory-first-stage", [13, 14, 14], 14),
            ("glory-second-stage", [15, 16, 16], 16),
            ("glory-death-start", [17, 19], 19),
        ):
            previous_native, previous_rendered = native.copy(), rendered.copy()
            stamp(native, originals, rows)
            stamp(rendered, replacements, rows)
            native_motion = changed_pixels(previous_native, native)
            rendered_motion = changed_pixels(previous_rendered, rendered)
            native_fresh, rendered_fresh = native_neutral.copy(), rendered_neutral.copy()
            stamp(native_fresh, originals, [final_row])
            stamp(rendered_fresh, replacements, [final_row])
            native_residue = changed_pixels(native_fresh, native)
            rendered_residue = changed_pixels(rendered_fresh, rendered)
            results.append({"phase": name, "native_rows": rows,
                "native_motion_pixels": native_motion, "rendered_motion_pixels": rendered_motion,
                "native_residual_pixels": native_residue, "rendered_residual_pixels": rendered_residue})
            if native_motion >= 4:
                assert rendered_motion >= 4, f"Shofixti {name}: latched native stage is visually unchanged"
            assert rendered_residue <= native_residue + 2, (
                f"Shofixti {name}: cumulative lever leaves {rendered_residue} pixels from a previous stage "
                f"(native control {native_residue})")
    return results


def validate(inventory: Path, render: Path, archive_path: Path, content: Path | None):
    data = json.loads(inventory.read_text())
    assert sha(archive_path.read_bytes()) == data["source_archive"]["sha256"], "Native archive hash changed"
    assert sorted(s["melee_index"] for s in data["ships"]) == list(range(25)), "Must cover all 25 Melee races"
    records, seen, compositions = [], set(), []
    total_rows = 0
    with zipfile.ZipFile(archive_path) as archive:
        for ship in data["ships"]:
            original_images, replacement_images = {}, {}
            ani = archive.read(ship["ani_path"])
            assert sha(ani) == ship["ani_sha256"], ship["ani_path"]
            rows = [line.split() for line in ani.decode().splitlines()
                    if line.strip() and not line.lstrip().startswith("#")]
            count = 20 if ship["race"] == "shofixti" else 15
            assert len(rows) == ship["frame_count"] == len(ship["frames"]) == count, ship["race"]
            assert [f["index"] for f in ship["frames"]] == list(range(count)), ship["race"]
            assert len({row[0] for row in rows}) == ship["unique_png_count"], ship["race"]
            total_rows += count
            if content:
                # Most content packages remain zipped. If an extracted ANI is
                # present, it must still be the original descriptor byte-for-byte.
                live = content / "addons" / ship["ani_path"]
                if live.exists():
                    assert live.read_bytes() == ani, f"Original ANI changed: {live}"
            for row, frame in zip(rows, ship["frames"]):
                source = str(Path(ship["ani_path"]).parent / row[0])
                context = f"{ship['race']} row {frame['index']} ({row[0]})"
                assert frame["source_path"] == source and frame["filename"] == row[0], context
                assert frame["hotspot"] == [int(row[3]), int(row[4])], context
                assert frame["position_in_portrait"] == [-int(row[3]), -int(row[4])], context
                assert frame["transparent_color"] == int(row[1]), context
                assert frame["colormap_index"] == int(row[2]), context
                assert frame["aliases"] == [i for i, alias in enumerate(rows) if alias[0] == row[0]], context
                source_bytes = archive.read(source)
                assert sha(source_bytes) == frame["source_sha256"], context
                native = Image.open(io.BytesIO(source_bytes)).convert("RGBA")
                assert list(native.size) == frame["size"], context
                relative = Path(source).relative_to("mm-hd")
                destination = render / relative
                assert destination.is_file(), f"Missing replacement: {destination}"
                replacement_bytes = destination.read_bytes()
                assert replacement_bytes != source_bytes, f"Original PNG copied unchanged: {destination}"
                replacement = Image.open(io.BytesIO(replacement_bytes)).convert("RGBA")
                assert replacement.size == native.size, f"Changed native dimensions: {destination}"
                original_alpha = native.getchannel("A")
                new_alpha = replacement.getchannel("A")
                assert new_alpha.getbbox(), f"Empty replacement: {destination}"
                assert original_alpha.tobytes() == new_alpha.tobytes(), f"Native overlay alpha changed: {destination}"
                assert replacement.tobytes() != native.tobytes(), f"Original pixels reused: {destination}"
                original_images[frame["filename"]] = native
                replacement_images[frame["filename"]] = replacement
                if content:
                    packaged = content / "remastered" / "render" / relative
                    assert packaged.read_bytes() == replacement_bytes, f"Unstaged captain: {packaged}"
                if source not in seen:
                    seen.add(source)
                    records.append({"race": ship["race"], "path": relative.as_posix(),
                        "size": frame["size"], "aliases": frame["aliases"],
                        "sha256": sha(replacement_bytes), "source_sha256": sha(source_bytes)})
            compositions.append({"race": ship["race"],
                "phases": check_compositions(ship, original_images, replacement_images)})
    assert total_rows == 380 and len(seen) == 253, "Native captain coverage is incomplete"
    return {"passed": True, "races": 25, "ani_rows": total_rows, "unique_pngs": len(seen),
        "archive_sha256": data["source_archive"]["sha256"],
        "validated_content": str(content) if content else None, "frames": records,
        "incremental_compositions": compositions}


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--inventory", type=Path, default=ROOT / "hd/remastered/art/captains/native-inventory.json")
    parser.add_argument("--render", type=Path, default=ROOT / "hd/remastered/render")
    parser.add_argument("--archive", type=Path, default=ROOT / ".tools/hd-assets/mm-0.8.5-hd-content.uqm")
    parser.add_argument("--content", type=Path, help="Also validate staged native content")
    parser.add_argument("--output", type=Path, default=ROOT / "test-results/captain-assets/results.json")
    args = parser.parse_args()
    result = validate(args.inventory, args.render, args.archive, args.content)
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(result, indent=2) + "\n")
    print(f"Captain assets passed: {result['races']} races, {result['ani_rows']} ANI rows, {result['unique_pngs']} PNGs")
