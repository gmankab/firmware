#!/usr/bin/env bash
# SPDX-License-Identifier: GPL-3.0-only
set -euo pipefail
# Usage: tools/verify.sh /path/to/untouched/extracted/usr/lib/firmware
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
ORIGINAL="${1:?pass path to directory containing the cirrus, novatek and qcom trees}"
cd "$ROOT"
cargo test --offline --locked
cargo build --release --offline --locked
mkdir -p out
./target/release/nabu-firmware-research rebuild-modemuw out/modemuw.jsn
REF="$ORIGINAL/qcom/sm8150/Xiaomi/nabu/modemuw.jsn"
./target/release/nabu-firmware-research verify-modemuw "$REF"
cmp -- "$REF" out/modemuw.jsn
(
    cd "$ORIGINAL"
    sha256sum -c "$ROOT/reference/firmware.sha256"
)
sha256sum "$REF" out/modemuw.jsn
printf '%s\n' 'Verification passed: one text configuration rebuilt byte-for-byte; proprietary executables NOT rebuilt.'
