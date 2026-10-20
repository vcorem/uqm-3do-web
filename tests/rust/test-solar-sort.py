#!/usr/bin/env python3
"""Compare the production Rust solar sort with the extracted upstream C body.

The one oracle-only edit omits the final unused NextIndex assignment: upstream
reads an uninitialized array element there, and at 16 planets reads past it.
All ordering/comparison/link logic remains the actual per-edition C source.
"""
import argparse
import os
from pathlib import Path
import subprocess
import shutil

ROOT = Path(__file__).resolve().parents[2]


def function(text, signature):
    start = text.index(signature)
    begin = text.index('{', start)
    depth = 1
    end = begin + 1
    while depth:
        depth += (text[end] == '{') - (text[end] == '}')
        end += 1
    return text[start:end]


C_PREFIX = '''#include <stdint.h>
#include <string.h>
typedef uint16_t COUNT;
typedef int16_t SIZE;
#define MAX_PLANETS 16
struct Point { int16_t x, y; };
struct Stamp { struct Point origin; };
typedef struct { struct Stamp image; uint8_t NumPlanets; COUNT NextIndex; } PLANET_DESC;
typedef struct { PLANET_DESC SunDesc[1], PlanetDesc[16]; SIZE FirstPlanetIndex, LastPlanetIndex; } SOLARSYS_STATE;
static SOLARSYS_STATE state;
static SOLARSYS_STATE *pSolarSysState = &state;
'''
C_SUFFIX = '''
void oracle_sort(uint8_t count, const int16_t *ys, int16_t *output) {
    memset(&state, 0, sizeof state);
    state.SunDesc[0].NumPlanets = count;
    state.SunDesc[0].image.origin.y = ys[0];
    for (unsigned i = 0; i < count; ++i) state.PlanetDesc[i].image.origin.y = ys[i+1];
    sortPlanetPositions();
    output[0] = state.FirstPlanetIndex;
    output[1] = state.LastPlanetIndex;
    output[2] = state.SunDesc[0].NextIndex;
    for (unsigned i = 0; i < count; ++i) output[i+3] = state.PlanetDesc[i].NextIndex;
}
'''
RUST_PREFIX = '''#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused_assignments, unused_mut, dead_code)]
type COUNT = u16;
type SIZE = i16;
#[derive(Clone, Copy, Default)] struct POINT { x: i16, y: i16 }
#[derive(Clone, Copy, Default)] struct STAMP { origin: POINT }
#[derive(Clone, Copy, Default)] struct PLANET_DESC { image: STAMP, NumPlanets: u8, NextIndex: COUNT }
#[derive(Default)] struct SOLARSYS_STATE {
    SunDesc: [PLANET_DESC;1], PlanetDesc: [PLANET_DESC;16],
    FirstPlanetIndex: SIZE, LastPlanetIndex: SIZE,
}
static mut pSolarSysState: *mut SOLARSYS_STATE = core::ptr::null_mut();
extern "C" {
    fn memset(pointer: *mut core::ffi::c_void, value: core::ffi::c_int, count: usize) -> *mut core::ffi::c_void;
    fn oracle_sort(count: u8, ys: *const i16, output: *mut i16);
}
'''
RUST_SUFFIX = '''
fn main() {
    let mut seed = 0x01a23456_u32;
    let mut comparisons = 0_u64;
    let mut cases = 0_u64;
    for count in 0..=16_usize {
        for case in 0..2008 {
            let mut ys = [0_i16;17];
            for (i,y) in ys.iter_mut().enumerate() {
                seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
                *y = match case {
                    0 => 0, 1 => i as i16, 2 => -(i as i16),
                    3 => i16::MIN, 4 => i16::MAX,
                    5 => if i % 2 == 0 { i16::MIN } else { i16::MAX },
                    6 => if i == 0 { 0 } else { 1 },
                    7 => if i == 0 { 0 } else { -1 },
                    _ if case % 2 == 0 => (seed % 7) as i16 - 3,
                    _ => (seed >> 16) as i16,
                };
            }
            let mut state = SOLARSYS_STATE::default();
            state.SunDesc[0].NumPlanets = count as u8;
            state.SunDesc[0].image.origin.y = ys[0];
            for i in 0..count { state.PlanetDesc[i].image.origin.y = ys[i+1]; }
            let mut expected = [0_i16;19];
            unsafe {
                pSolarSysState = &mut state;
                sortPlanetPositions();
                oracle_sort(count as u8, ys.as_ptr(), expected.as_mut_ptr());
            }
            assert_eq!(state.FirstPlanetIndex, expected[0], "first: count={count} case={case}");
            assert_eq!(state.LastPlanetIndex, expected[1], "last: count={count} case={case}");
            comparisons += 2;
            let mut index = state.FirstPlanetIndex;
            let mut visited = [false;17];
            for step in 0..=count {
                assert!((-1..count as i16).contains(&index));
                assert!(!visited[(index+1) as usize], "display list cycle");
                visited[(index+1) as usize] = true;
                let next = if index == -1 { state.SunDesc[0].NextIndex } else { state.PlanetDesc[index as usize].NextIndex };
                if step == count {
                    assert_eq!(index, state.LastPlanetIndex);
                    assert_eq!(next, 0, "unused terminal must be deterministic");
                } else {
                    assert_eq!(next as i16, expected[(index+3) as usize], "link: count={count} case={case} index={index}");
                    index = next as i16;
                }
                comparisons += 1;
            }
            cases += 1;
        }
    }
    println!("solar sort: {cases} cases, {comparisons} comparisons passed");
}
'''


def run(edition):
    source = ROOT / ('sc2' if edition == 'original' else '.tools/hd-source')
    c = (source / 'src/uqm/planets/solarsys.c').read_text()
    c_sort = function(c, 'sortPlanetPositions (void)')
    terminal = 'planet->NextIndex = sort_array[i + 1];'
    if c_sort.count(terminal) != 1:
        raise ValueError('upstream terminal assignment changed; review oracle boundary')
    c_sort = c_sort.replace(terminal, 'if (i < pSolarSysState->SunDesc[0].NumPlanets) ' + terminal)
    rust = (ROOT / f'rust/uqm-engine-{edition}/src/uqm/planets/solarsys.rs').read_text()
    helper = function(rust, 'unsafe fn solarDisplayBody(')
    sorter = function(rust, 'unsafe extern "C" fn sortPlanetPositions(')
    out = ROOT / f'.tools/rust-solar-sort-oracle/{edition}'
    out.mkdir(parents=True, exist_ok=True)
    (out / 'oracle.c').write_text(C_PREFIX + 'static void\n' + c_sort + C_SUFFIX)
    (out / 'main.rs').write_text(RUST_PREFIX + helper + '\n' + sorter + RUST_SUFFIX)
    env = os.environ.copy()
    isolated_rustc = ROOT / '.tools/cargo/bin/rustc'
    if isolated_rustc.is_file():
        rustc = str(isolated_rustc)
        env.setdefault('RUSTUP_HOME', str(ROOT / '.tools/rustup'))
        env.setdefault('CARGO_HOME', str(ROOT / '.tools/cargo'))
    else:
        rustc = shutil.which('rustc')
        if not rustc:
            raise SystemExit('Rust 1.85.1 is required; install the pinned toolchain first.')
    for optimization in ['0', '2']:
        subprocess.run([os.environ.get('CC','cc'), '-std=c11', '-O'+optimization,
                        '-c', str(out / 'oracle.c'), '-o', str(out / 'oracle.o')], check=True)
        executable = out / ('test-O' + optimization)
        subprocess.run([str(rustc), '+1.85.1', '--edition=2021', '-Copt-level='+optimization,
                        '-Clink-arg='+str(out/'oracle.o'), str(out/'main.rs'), '-o', str(executable)],
                       env=env, check=True)
        print(f'{edition} native O{optimization}:', flush=True)
        subprocess.run([str(executable)], check=True)


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--edition', choices=['original','hd','all'], default='all')
    args = parser.parse_args()
    for edition in (['original','hd'] if args.edition == 'all' else [args.edition]):
        run(edition)
