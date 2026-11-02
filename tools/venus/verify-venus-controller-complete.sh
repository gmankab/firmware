#!/usr/bin/env bash
# SPDX-License-Identifier: GPL-3.0-only
# Deterministically rebuild and compare Agent 20-04's entire VMA 0x40000..0x68000.
# Reference firmware is strictly read for verification and is never packaged.
set -euo pipefail
if [ "$#" -lt 1 ] || [ "$#" -gt 2 ] || [ ! -f "$1" ]; then
    echo 'usage: tools/venus/verify-venus-controller-complete.sh /private/venus.mbn [generated-fragment.bin]' >&2
    exit 2
fi
REF=$(realpath "$1")
OUTPUT=${2:-}
ROOT=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
cd "$ROOT"
export CARGO_BUILD_JOBS=${CARGO_BUILD_JOBS:-1}
# This large historical branch compiles in a 4 GiB environment at opt-level 0.
# Callers can deliberately override the profile for their own machines.
export CARGO_PROFILE_RELEASE_OPT_LEVEL=${CARGO_PROFILE_RELEASE_OPT_LEVEL:-0}
export CARGO_PROFILE_RELEASE_DEBUG=${CARGO_PROFILE_RELEASE_DEBUG:-0}
export CARGO_PROFILE_RELEASE_CODEGEN_UNITS=${CARGO_PROFILE_RELEASE_CODEGEN_UNITS:-256}
TEMP=$(mktemp -d)
trap 'rm -rf "$TEMP"' EXIT
cargo run --offline --locked --release --quiet -p firmware-tools --example venus_controller_prefix -- emit "$TEMP/prefix.bin"
cargo run --offline --locked --release --quiet -p firmware-tools --example venus_controller_middle -- emit "$TEMP/middle"
cargo run --offline --locked --release --quiet -p firmware-tools --example venus_controller_continue -- emit "$TEMP/tail.bin"
if [ -n "$OUTPUT" ]; then
    python3 tools/venus/venus-controller_full_range.py --prefix "$TEMP/prefix.bin" --middle-prefix "$TEMP/middle" --tail "$TEMP/tail.bin" --reference "$REF" --output "$OUTPUT"
else
    python3 tools/venus/venus-controller_full_range.py --prefix "$TEMP/prefix.bin" --middle-prefix "$TEMP/middle" --tail "$TEMP/tail.bin" --reference "$REF"
fi
