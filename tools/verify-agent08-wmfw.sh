#!/usr/bin/env bash
# SPDX-License-Identifier: GPL-3.0-only
# Rebuild only the INDEPENDENT metadata, then explicitly provide opaque HALO
# DSP program/data as private external source inputs. Never commit firmware.
set -euo pipefail
if [ "$#" -ne 1 ]; then
    echo "Usage: $0 /private/cirrus/cs35l41-dsp1-spk-prot.wmfw" >&2
    exit 2
fi
ref="$1"
repo="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo"
if [ ! -f "$ref" ]; then
    echo "WMFW reference missing: $ref" >&2
    exit 1
fi
ref="$(realpath "$ref")"
work="$(mktemp -d "${TMPDIR:-/tmp}/agent08-wmfw-audit.XXXXXXXX")"
cleanup() { rm -rf -- "$work"; }
trap cleanup EXIT

cargo test --workspace --offline --locked
python3 -m unittest discover -s tests -p 'test_*.py'
cargo build --workspace --release --offline --locked
cargo run --offline --locked -q -p firmware-tools -- verify-wmfw-layout "$ref"
cargo run --offline --locked -q -p firmware-tools -- wmfw-coverage
cargo run --offline --locked -q -p firmware-tools -- wmfw-memory-map
cargo run --offline --locked -q -p firmware-tools -- wmfw-cs35l41-bus-map
cargo run --offline --locked -q -p firmware-tools -- wmfw-control-ranges
cargo run --offline --locked -q -p firmware-tools -- inspect-wmfw-program-patterns "$ref"

# Explicit original DSP payloads are temporarily isolated outside Git.
cargo run --offline --locked -q -p firmware-tools -- extract-wmfw-external-payloads "$ref" "$work/original-dsp-payloads"
cargo run --offline --locked -q -p firmware-tools -- assemble-wmfw-with-external-payloads "$work/original-dsp-payloads" "$work/repacked.wmfw"
cmp "$ref" "$work/repacked.wmfw"
sha256sum "$work/repacked.wmfw"
git diff --check
printf '\nAgent08 exact reference REPACK verified. Independent 3984 metadata bytes; 30252 externally supplied proprietary DSP bytes.\n'
printf 'HALO source reconstruction is not complete, regardless of byte-identical repack.\n'
