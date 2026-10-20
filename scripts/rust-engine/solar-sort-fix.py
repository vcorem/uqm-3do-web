#!/usr/bin/env python3
"""Represent the C solar display list's -1 sun index explicitly in Rust."""
import argparse
from pathlib import Path
import re

HELPER = '''// The C display list aliases PlanetDesc[-1] to the preceding SunDesc.
// Select the actual field instead of relying on out-of-array pointer arithmetic.
unsafe fn solarDisplayBody(index: SIZE) -> *mut PLANET_DESC {
    if index == -1 {
        &raw mut (*pSolarSysState).SunDesc[0usize]
    } else {
        assert!(index >= 0, "invalid solar display index");
        &raw mut (*pSolarSysState).PlanetDesc[index as usize]
    }
}
'''


def transform(text, edition):
    if 'unsafe fn solarDisplayBody(' in text:
        if 'PlanetDesc[real_i as usize]' in text or '.offset(sort_array[i as usize] as isize)' in text:
            raise ValueError('mixed solar-sort correction state')
        return text
    declaration = 'unsafe extern "C" fn sortPlanetPositions() {'
    if text.count(declaration) != 1:
        raise ValueError('expected one sortPlanetPositions function')
    text = text.replace(declaration, HELPER + declaration)
    for variable in ['real_i', 'real_j']:
        old = f'(*pSolarSysState).PlanetDesc[{variable} as usize].image.origin.y'
        expected = 2 if edition == 'hd' else 1
        if text.count(old) != expected:
            raise ValueError(f'unexpected solar comparison count: {variable}')
        text = text.replace(old, f'(*solarDisplayBody({variable})).image.origin.y')
    pattern = r'\(&raw mut \(\*pSolarSysState\)\.PlanetDesc\s+as \*mut PLANET_DESC\)\s*\.offset\(sort_array\[i as usize\] as isize\)'
    text, count = re.subn(pattern, 'solarDisplayBody(sort_array[i as usize])', text)
    if count != 1:
        raise ValueError('expected one solar next-link target')
    old = '''(*planet).NextIndex = sort_array[(i as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int) as usize] as COUNT;'''
    new = '''// The last link is never traversed. C reads an uninitialized slot
        // here (out of bounds at 16 planets); use a deterministic terminator.
        (*planet).NextIndex = if i < (*pSolarSysState).SunDesc[0usize].NumPlanets as COUNT {
            sort_array[i as usize + 1] as COUNT
        } else {
            0
        };'''
    if text.count(old) != 1:
        raise ValueError('expected one terminal next-link assignment')
    text = text.replace(old, new)
    pattern = r'\(&raw mut \(\*pSolarSysState\)\.PlanetDesc as \*mut PLANET_DESC\)\s*\.offset\(index as isize\)'
    text, count = re.subn(pattern, 'solarDisplayBody(index)', text)
    if count != 1:
        raise ValueError('expected one solar display-list traversal')
    return text


def apply(root, edition):
    path = Path(root) / 'src/uqm/planets/solarsys.rs'
    text = path.read_text()
    corrected = transform(text, edition)
    if corrected != text:
        path.write_text(corrected)


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('root')
    parser.add_argument('--edition', choices=['original', 'hd'], required=True)
    args = parser.parse_args()
    apply(args.root, args.edition)
