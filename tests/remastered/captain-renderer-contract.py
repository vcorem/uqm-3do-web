#!/usr/bin/env python3
"""Check both native lookup tables against actual archive resource paths."""
import json
import re
from pathlib import Path

root = Path(__file__).resolve().parents[2]
inventory = json.loads((root / "hd/remastered/art/captains/native-inventory.json").read_text())
ships = sorted(inventory["ships"], key=lambda ship: ship["melee_index"])
assert [ship["melee_index"] for ship in ships] == list(range(25))
expected = [ship["ani_path"].split("/")[2] for ship in ships]
rust = (root / "rust/uqm-engine-hd/uqm_melee_presentation.rs").read_text()
rust_table = re.search(r"const CAPTAIN_RACES:.*?= \[(.*?)\];", rust, re.S).group(1)
c_patch = (root / "hd/patches/0017-captain-presentation.patch").read_text()
c_table = re.search(r"static const char \*captainRaces\[25\] = \{(.*?)\};", c_patch, re.S).group(1)
assert re.findall(r'b"([a-z]+)"', rust_table) == expected, "Rust native race/path mapping differs from archive"
assert re.findall(r'"([a-z]+)"', c_table) == expected, "C native race/path mapping differs from archive"
print("Captain renderer resource contract: all25 Melee indexes match the original archive paths in C and Rust.")
