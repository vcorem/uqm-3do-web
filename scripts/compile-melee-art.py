#!/usr/bin/env python3
"""Compile authored sprite atlases into presentation-only native HD frames.

This is asset packing/resampling, not an art generator. The pinned game archive
is read only. Its frames, collision masks, ANI data, and hotspots never change.
Requires Pillow for development; players need no Python dependencies.
"""
from __future__ import annotations

import argparse
import hashlib
import io
import json
from pathlib import Path
import zipfile

from PIL import Image

ROOT = Path(__file__).resolve().parents[1]
ART = ROOT / "hd/remastered/art"
RENDER = ROOT / "hd/remastered/render"


def sha(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def artwork_cell(atlas: Image.Image, slot: int, columns: int,
                 source_rect: list[int] | None = None) -> Image.Image:
    """Select an authored cell, optionally bounded by an explicit packing rect.

    source_rect is [left, top, right, bottom] in cell-relative pixels, with
    exclusive right/bottom bounds. It precedes the existing alpha tight crop.
    """
    x, y = slot % columns, slot // columns
    w, h = atlas.width // columns, atlas.height // columns
    cell = atlas.crop((x * w, y * h, (x + 1) * w, (y + 1) * h)).convert("RGBA")
    if source_rect is not None:
        if len(source_rect) != 4 or any(type(value) is not int for value in source_rect):
            raise ValueError(f"Invalid source_rect for artwork cell {slot}: {source_rect}")
        left, top, right, bottom = source_rect
        if not (0 <= left < right <= w and 0 <= top < bottom <= h):
            raise ValueError(f"source_rect outside artwork cell {slot}: {source_rect}")
        cell = cell.crop((left, top, right, bottom))
    alpha = cell.getchannel("A")
    if alpha.getextrema()[0] == 255:
        raise ValueError("Artwork must have real alpha transparency, not a baked background")
    bounds = alpha.point(lambda p: 255 if p >= 24 else 0).getbbox()
    if bounds is None:
        raise ValueError(f"Empty artwork cell {slot}")
    return cell.crop(bounds)


def frame_rows(archive: zipfile.ZipFile, name: str):
    for line in archive.read(name).decode("utf-8").splitlines():
        fields = line.split()
        if fields and not fields[0].startswith("#"):
            yield fields


def compile_art(archive_path: Path):
    records = []
    manifest = json.loads((ART / "ships-manifest.json").read_text())
    RENDER.mkdir(parents=True, exist_ok=True)
    with zipfile.ZipFile(archive_path) as archive:
        for group in manifest:
            sheet_path = ART / f"ships-{group['group']}.png"
            sheet = Image.open(sheet_path).convert("RGBA")
            for entry in group["entries"]:
                race, stem = entry["race"], entry["stem"]
                north = artwork_cell(sheet, entry["slot"], 2, entry.get("source_rect"))
                thumbnail = north.copy()
                thumbnail.thumbnail((512, 512), Image.Resampling.LANCZOS)
                thumb_path = ART / "ships" / f"{race}-{stem}.png"
                thumb_path.parent.mkdir(parents=True, exist_ok=True)
                thumbnail.save(thumb_path, optimize=True)
                for size in ("big", "med", "sml"):
                    ani = f"mm-hd/ships/{race}/{stem}-{size}.ani"
                    rows = list(frame_rows(archive, ani))
                    # Orz turret's extra rows are launch/weapon animation, not
                    # hull facings. Keep those on their existing HD resources.
                    if race == "orz" and stem == "turret":
                        rows = rows[:16]
                    if len(rows) != 16:
                        raise ValueError(f"Expected 16 facings: {ani}: {len(rows)}")
                    for index, fields in enumerate(rows):
                        source_name = f"mm-hd/ships/{race}/{fields[0]}"
                        source_bytes = archive.read(source_name)
                        original = Image.open(io.BytesIO(source_bytes)).convert("RGBA")
                        bounds = original.getchannel("A").getbbox()
                        if bounds is None:
                            raise ValueError(f"Empty original frame {source_name}")
                        rotated = north.rotate(-index * 22.5, resample=Image.Resampling.BICUBIC, expand=True)
                        tight = rotated.getchannel("A").point(lambda p: 255 if p >= 24 else 0).getbbox()
                        rotated = rotated.crop(tight)
                        # Individual original extents include deliberate zoom rounding.
                        # Never derive the medium/small collision geometry by scaling big.
                        fitted = rotated.resize((bounds[2] - bounds[0], bounds[3] - bounds[1]), Image.Resampling.LANCZOS)
                        visible = Image.new("RGBA", original.size)
                        visible.alpha_composite(fitted, bounds[:2])
                        relative = f"ships/{race}/{fields[0]}"
                        destination = RENDER / relative
                        destination.parent.mkdir(parents=True, exist_ok=True)
                        visible.save(destination, optimize=True)
                        records.append({"path": relative, "source": source_name,
                            "source_sha256": sha(source_bytes), "size": list(original.size),
                            "hotspot": [int(fields[3]), int(fields[4])],
                            "sha256": sha(destination.read_bytes())})
                # HUD and fleet portraits use their own (often asymmetric)
                # frames. Keep that canvas and hotspot, fit the same authored
                # hull inside it, and let the UI-only render scope select it.
                # Alternate-form ANIs can alias the primary form's icon PNGs.
                # One primary hull owns those files; never overwrite it with
                # the transformed hull while compiling a second ANI.
                for kind in (() if stem in {"ywing", "blazer", "turret"} else ("icons", "meleeicons")):
                    ani = f"mm-hd/ships/{race}/{stem}-{kind}.ani"
                    if ani not in archive.namelist():
                        continue
                    for index, fields in enumerate(frame_rows(archive, ani)):
                        if index > 1 or (kind == "icons" and index == 0):
                            continue
                        source_name = f"mm-hd/ships/{race}/{fields[0]}"
                        source_bytes = archive.read(source_name)
                        original = Image.open(io.BytesIO(source_bytes)).convert("RGBA")
                        bounds = original.getchannel("A").getbbox()
                        fitted = north.copy()
                        fitted.thumbnail((bounds[2]-bounds[0], bounds[3]-bounds[1]), Image.Resampling.LANCZOS)
                        portrait = Image.new("RGBA", original.size)
                        portrait.alpha_composite(fitted, (bounds[0]+(bounds[2]-bounds[0]-fitted.width)//2,
                            bounds[1]+(bounds[3]-bounds[1]-fitted.height)//2))
                        relative = f"ships/{race}/{fields[0]}"
                        destination = RENDER / relative
                        destination.parent.mkdir(parents=True, exist_ok=True)
                        portrait.save(destination, optimize=True)
                        records.append({"path":relative,"source":source_name,"ui":True,
                            "source_sha256":sha(source_bytes),"size":list(original.size),
                            "hotspot":[int(fields[3]),int(fields[4])],"sha256":sha(destination.read_bytes())})
        background = Image.open(ART / "nebula.png").convert("RGB")
        background.resize((1280, 960), Image.Resampling.LANCZOS).save(RENDER / "background.png", optimize=True)
        explosion_path = ART / "explosion.png"
        if explosion_path.exists():
            explosion = Image.open(explosion_path).convert("RGBA")
            for stem, count in (("boom", 9), ("blast", 8)):
                for size in ("big", "med", "sml"):
                    ani = f"mm-hd/battle/{stem}-{size}.ani"
                    rows = list(frame_rows(archive, ani))
                    if len(rows) != count:
                        raise ValueError(f"Unexpected explosion animation: {ani}")
                    for index, fields in enumerate(rows):
                        source_name = f"mm-hd/battle/{fields[0]}"
                        source_bytes = archive.read(source_name)
                        original = Image.open(io.BytesIO(source_bytes)).convert("RGBA")
                        # Keep each cell's size/position: later frames should fade, not
                        # be enlarged again by a fresh tight crop.
                        cell_index = round(index * 8 / (count - 1))
                        cw, ch = explosion.width // 3, explosion.height // 3
                        x, y = cell_index % 3, cell_index // 3
                        frame = explosion.crop((x*cw, y*ch, (x+1)*cw, (y+1)*ch))
                        frame = frame.resize(original.size, Image.Resampling.LANCZOS)
                        relative = f"battle/{stem}-{size}-{index:03}.png"
                        dest = RENDER / relative; dest.parent.mkdir(parents=True, exist_ok=True)
                        frame.save(dest, optimize=True)
                        records.append({"path":relative,"source":source_name,
                            "source_sha256":sha(source_bytes),"size":list(original.size),
                            "hotspot":[int(fields[3]),int(fields[4])],"sha256":sha(dest.read_bytes())})
    data = {"schema":1,"purpose":"render-only; original collision images remain authoritative",
        "archive_sha256":sha(archive_path.read_bytes()),"ship_forms":sum(len(g['entries']) for g in manifest),
        "frames":records}
    (RENDER / "manifest.json").write_text(json.dumps(data, indent=2) + "\n")
    print(f"Compiled {data['ship_forms']} ship forms, {len(records)} presentation frames and background")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--archive", type=Path, default=ROOT / ".tools/hd-assets/mm-0.8.5-hd-content.uqm")
    compile_art(parser.parse_args().archive)
