#!/usr/bin/env bash
# SPDX-License-Identifier: GPL-3.0-only
# Verifies bare-metal Hexagon code generation; DOES NOT produce Nabu firmware.
set -euo pipefail
ROOT=$(cd "$(dirname "$0")/.." && pwd)
VENDOR=${RUST_STD_VENDOR:?Set RUST_STD_VENDOR to the extracted rustc-1.99.1-src/library/vendor directory}
if [[ ! -f "$VENDOR/rustc-literal-escaper-0.0.8/Cargo.toml" ]]; then
    echo "Missing vendored Rust std dependencies in: $VENDOR" >&2
    exit 1
fi
if [[ ! -f "$(rustc --print sysroot)/lib/rustlib/src/rust/library/core/Cargo.toml" ]]; then
    echo "Install the matching rust-src distribution into the active Rust sysroot" >&2
    exit 1
fi
# Needed to compile the Rust 'core' library with a stable 1.99.1 toolchain.
export RUSTC_BOOTSTRAP=1
cargo -Z build-std=core --offline \
    --config "source.crates-io.replace-with='vendored-sources'" \
    --config "source.vendored-sources.directory='$VENDOR'" \
    build --release --locked --target hexagon-unknown-none-elf \
    --manifest-path "$ROOT/probes/hexagon/Cargo.toml"
A="$ROOT/probes/hexagon/target/hexagon-unknown-none-elf/release/libnabu_hexagon_compiler_probe.a"
test -s "$A"
readelf -h "$A" | grep -q 'QUALCOMM DSP6 Processor'
echo "Verified Hexagon machine code in $A; this is not a firmware rebuild"
