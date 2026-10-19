#!/usr/bin/env python3
"""Compare actual generated HD packet/transport modules with their locked C sources.

The selected production Rust files are imported unmodified. The reference C
functions are renamed only at the compiler boundary and linked into the same
Wasm program. Tests run both directly and on a pthread through the real browser
main-thread Emscripten proxy bridge. No Lua script runs in this isolated test;
the pending-error callback returns zero. Full Lua behavior has its own oracle.
"""
import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import shutil
import subprocess

ROOT = Path(__file__).resolve().parents[2]
HERE = Path(__file__).resolve().parent


def run(command, *, cwd=ROOT, env=None, log=None, timeout=600):
    if log:
        with log.open("w") as stream:
            result = subprocess.run(command, cwd=cwd, env=env, stdout=stream, stderr=subprocess.STDOUT, timeout=timeout)
        if result.returncode:
            print(log.read_text()[-14000:])
            raise SystemExit(f"Command failed ({result.returncode}); see {log}")
        return ""
    result = subprocess.run(command, cwd=cwd, env=env, capture_output=True, text=True, timeout=timeout)
    if result.returncode:
        print(result.stdout[-10000:])
        print(result.stderr[-10000:])
        raise SystemExit(f"Command failed ({result.returncode}): {command[0]}")
    return result.stdout


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--source", type=Path,
                        default=Path(os.environ.get("HD_SOURCE_DIR", ROOT / ".tools/hd-source")))
    parser.add_argument("--all", action="store_true", help="Run both debug and release Rust/C profiles")
    args = parser.parse_args()
    source = args.source.resolve()
    engine = ROOT / "rust/uqm-engine-hd"
    work = ROOT / ".tools/rust-hd-netplay-tests"
    crate = work / "crate"
    crate.mkdir(parents=True, exist_ok=True)
    lock = json.loads((ROOT / "scripts/rust-engine/hd-inputs.json").read_text())
    for relative, expected in lock["sha256"].items():
        if hashlib.sha256((source / relative).read_bytes()).hexdigest() != expected:
            raise SystemExit("HD C reference differs from translation lock: " + relative)
    units = ["netrcv", "netsend", "packethandlers"]
    production = {name: engine / f"src/uqm/supermelee/netplay/{name}.rs" for name in units}
    crate.joinpath("Cargo.toml").write_text('''[package]
name = "uqm-hd-netplay-oracle"
version = "0.1.0"
edition = "2021"
[lib]
path = "lib.rs"
crate-type = ["staticlib"]
[dependencies]
c2rust-bitfields = { version = "=0.22.1", features = ["no_std"] }
[workspace]
[profile.dev]
panic = "abort"
[profile.release]
panic = "abort"
''')
    guard = ROOT / "rust/uqm-lua/guard.rs"
    prefix = '''#![no_std]
#![feature(extern_types, c_variadic)]
#![allow(dead_code, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
'''
    prefix += '#[macro_use]\n#[path = ' + json.dumps(str(guard)) + ']\nmod uqm_lua_guard;\n'
    for name, path in production.items():
        prefix += '#[path = ' + json.dumps(str(path)) + ']\nmod ' + name + ';\n'
    prefix += '''
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! { core::arch::wasm32::unreachable() }
#[no_mangle]
pub extern "C" fn rust_netplay_layout(which: u32) -> usize {
    use core::mem::{offset_of, size_of};
    use packethandlers as p;
    match which {
        0 => size_of::<p::NetConnection>(),
        1 => size_of::<p::NetStateFlags>(),
        2 => size_of::<p::NetStatistics>(),
        3 => offset_of!(p::NetConnection, stateFlags),
        4 => offset_of!(p::NetConnection, statistics),
        5 => offset_of!(p::NetConnection, readBuf),
        6 => offset_of!(p::NetConnection, readEnd),
        7 => size_of::<p::Packet_Fleet>(),
        8 => size_of::<p::Packet_TeamName>(),
        9 => size_of::<p::BattleStateData>(),
        10 => offset_of!(p::BattleStateData, meleeState),
        11 => size_of::<netrcv::NetConnection>(),
        12 => size_of::<netsend::NetConnection>(),
        _ => usize::MAX,
    }
}
'''
    crate.joinpath("lib.rs").write_text(prefix)
    emcc = shutil.which("emcc")
    cargo = shutil.which("cargo")
    node = shutil.which("node")
    if not all([emcc, cargo, node]):
        raise SystemExit("Run via test-hd-netplay.sh with the pinned Rust/Emscripten tools installed")
    toolchain = (ROOT / ".rust-wasm-toolchain").read_text().strip()
    run([cargo, "+" + toolchain, "generate-lockfile", "--offline", "--manifest-path", str(crate / "Cargo.toml")])
    target = work / "target"
    env = dict(os.environ, CARGO_TARGET_DIR=str(target),
               RUSTFLAGS="-C target-feature=+atomics,+bulk-memory,+mutable-globals -C panic=abort")
    run(["python3", str(ROOT / "scripts/prepare-emscripten-proxy.py"), str(work / "compat")])
    compile_flags = []
    index = 0
    while index < len(lock["cflags"]):
        flag = lock["cflags"][index]
        if flag == "-s" and lock["cflags"][index + 1] == "ASYNCIFY":
            index += 2
            continue
        if flag not in {"-g", "-O0", "-I.", "-Isrc", "-pthread"}:
            compile_flags.append(flag)
        index += 1
    compile_flags += ["-I" + str(source), "-I" + str(source / "src")]
    profiles = ["debug", "release"] if args.all else ["release"]
    results = []
    for profile in profiles:
        build = [cargo, "+" + toolchain, "build", "--locked", "-Zbuild-std=core,alloc,compiler_builtins",
                 "--manifest-path", str(crate / "Cargo.toml"), "--target", "wasm32-unknown-emscripten"]
        if profile == "release":
            build.append("--release")
        run(build, env=env, log=work / f"rust-{profile}.log")
        library = target / "wasm32-unknown-emscripten" / profile / "libuqm_hd_netplay_oracle.a"
        optimization = "-O0" if profile == "debug" else "-O2"
        for mode in ["direct", "pthread"]:
            # Every input has shared-memory atomics, including the direct-main
            # test; the second mode also exercises actual cross-thread proxying.
            mode_flags = ["-pthread", "-sASSERTIONS=1", "-sSTACK_OVERFLOW_CHECK=2"]
            if mode == "pthread":
                mode_flags += ["-sPROXY_TO_PTHREAD=1", "-sPTHREAD_POOL_SIZE=1"]
            directory = work / f"{profile}-{mode}"
            directory.mkdir(exist_ok=True)
            objects = []
            for name in units:
                c_file = source / f"src/uqm/supermelee/netplay/{name}.c"
                exported = re.findall(r"^(?:int|void)\n((?:PacketHandler_\w+)|webDataReady|sendPacket)\(", c_file.read_text(), re.M)
                expected = 1 if name != "packethandlers" else 18
                if len(exported) != expected:
                    raise SystemExit(f"Unexpected public functions in {name}: {exported}")
                destination = directory / (name + ".o")
                renames = ["-D" + value + "=c_ref_" + value for value in exported]
                run([emcc, *compile_flags, *mode_flags, optimization, *renames, "-c", str(c_file), "-o", str(destination)],
                    log=directory / (name + ".log"))
                objects.append(str(destination))
            script = directory / "oracle.js"
            run([emcc, *compile_flags, *mode_flags, optimization, str(HERE / "hd-netplay.c"),
                 *objects, str(engine / "uqm_asm_consts.c"), str(library),
                 str(work / "compat/emscripten-proxying.o"),
                 "-sSINGLE_FILE=1", "-sENVIRONMENT=node", "-sEXIT_RUNTIME=1", "-sSTACK_SIZE=1048576",
                 "-sDEFAULT_PTHREAD_STACK_SIZE=1048576", "-o", str(script)], log=directory / "link.log")
            output = run([node, str(script)], timeout=180)
            directory.joinpath("output.log").write_text(output)
            result = json.loads(next(line for line in output.splitlines() if line.startswith('{"cases"')))
            result.update(profile=profile, mode=mode)
            assert result["passed"] and result["cases"] > 12000
            results.append(result)
            print(json.dumps(result), flush=True)
    report = {"edition": "hd", "source_units": units,
              "rust_sha256": {name: hashlib.sha256(path.read_bytes()).hexdigest() for name, path in production.items()},
              "c_sha256": {name: lock["sha256"][f"src/uqm/supermelee/netplay/{name}.c"] for name in units},
              "scenarios": ["13 independent C/Rust ABI layout checks", "fleet and team-name lengths/sides/reset/ready states",
                            "ship positions/types", "fragmentation/coalescing", "malformed packets", "callback disconnect",
                            "full receive buffer", "closed connection", "invalid JS byte queue", "send/backpressure/throw"],
              "results": results}
    work.joinpath("results.json").write_text(json.dumps(report, indent=2) + "\n")
    print("Report:", work / "results.json")


if __name__ == "__main__":
    main()
