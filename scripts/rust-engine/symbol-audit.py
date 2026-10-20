"""Compare every C build object's public defined symbols to the Rust archive."""
import argparse
import json
from pathlib import Path
import subprocess
import sys

root = Path(__file__).resolve().parents[2]
parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('database', type=Path)
parser.add_argument('archive', type=Path)
parser.add_argument('report', type=Path)
parser.add_argument('--allow-missing', action='append', default=[])
args = parser.parse_args()
database, archive, report = (p.resolve() for p in (args.database, args.archive, args.report))
commands = json.loads(database.read_text())
nm = root / '.tools/emsdk/upstream/bin/llvm-nm'

def symbols(path):
    output = subprocess.run([str(nm), '--format=posix', '--defined-only', '--extern-only', str(path)],
                            text=True, capture_output=True, check=True)
    found = {}
    for line in output.stdout.splitlines():
        parts = line.split()
        if len(parts) >= 2 and len(parts[1]) == 1:
            found[parts[0]] = parts[1]
    return found

expected = {}
for command in commands:
    source = Path(command['file'])
    base = Path(command['directory'])
    relative = source.relative_to(base)
    obj = Path(str(base / 'obj/debug-wasm' / relative) + '.o')
    if not obj.exists():
        raise SystemExit('Missing oracle object: ' + str(obj))
    for name, kind in symbols(obj).items():
        expected[name] = {'source': str(relative), 'kind': kind}
actual = symbols(archive)
missing = {name: value for name, value in expected.items() if name not in actual}
result = {'source_units': len(commands), 'expected_public_symbols': len(expected),
          'actual_rust_public_symbols': len(actual), 'missing': missing}
report.write_text(json.dumps(result, indent=2) + '\n')
print(json.dumps(result, indent=2))

unexpected = set(missing) - set(args.allow_missing)
if unexpected:
    raise SystemExit('Unexpected missing public symbols: ' + ', '.join(sorted(unexpected)))
